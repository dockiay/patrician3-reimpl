//! Reproductor MP3 con productor de E/S asíncrono y consumo no bloqueante.
//! La mezcla y la decodificación siguen en el hilo llamante; no es un driver físico.
use super::async_hstream::{AsyncHstream, AsyncServiceError, ServicePoll};
use super::async_input::{AsyncInput, InputError, InputState};
use super::files::ResourceStream;
use super::hstream_buffers::{BufferService, SampleSink, ServiceError};
use super::mp3_decoder::{DecodeError, ProgressiveMp3, ProgressiveStatus};
use super::progressive_playback::{probe_mp3_format, PlaybackError};
use super::sample_input::{SampleInput, SampleInputError};
use super::timing::{MilesTiming, TimingError};
use std::io::{Read, Seek, SeekFrom};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AsyncPlaybackError {
    Input(InputError),
    Service(ServiceError),
    Decoder(DecodeError),
    Sample(SampleInputError),
    Timing(TimingError),
    Probe(PlaybackError),
    Limit,
    InvalidFormat,
}
impl From<InputError> for AsyncPlaybackError {
    fn from(e: InputError) -> Self {
        Self::Input(e)
    }
}
impl From<DecodeError> for AsyncPlaybackError {
    fn from(e: DecodeError) -> Self {
        Self::Decoder(e)
    }
}
impl From<SampleInputError> for AsyncPlaybackError {
    fn from(e: SampleInputError) -> Self {
        Self::Sample(e)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AsyncPlaybackStatus {
    Playing,
    Paused,
    Underrun,
    End,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AsyncPlaybackRead {
    pub frames: usize,
    pub status: AsyncPlaybackStatus,
}
/// La emulación original solo cubre primeras tramas cortas aceptadas por el guard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UnderrunPolicy {
    #[default]
    WaitForInput,
    OriginalFirstFrameGuarded,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct UnderrunStats {
    pub calls: u64,
    pub silent_frames: u64,
    pub callback_starvations: u64,
    pub lock_contentions: u64,
    pub original_short_frames: u64,
}

pub struct AsyncPlayback {
    pub source: AsyncInput,
    pub hstream: Option<AsyncHstream>,
    hstream_serviced: bool,
    pub decoder: ProgressiveMp3,
    pub ring: SampleInput,
    pub timing: MilesTiming,
    pub sample_rate: u32,
    pub channels: u16,
    pub length: u32,
    pub rendered_frames: usize,
    pub max_frames: usize,
    pub stats: UnderrunStats,
    pub underrun_policy: UnderrunPolicy,
    pub status: AsyncPlaybackStatus,
    pending_slot: Option<usize>,
    eof_marker: bool,
    loops: u32,
    loops_left: u32,
    started: bool,
    input_scratch: Vec<u8>,
    pcm_scratch: Vec<u8>,
    deferred: Option<AsyncPlaybackError>,
    source_error: Option<InputError>,
}
impl AsyncPlayback {
    /// La lectura de cabecera ocurre al abrir, antes del callback de audio.
    /// Las etiquetas mayores que el prefijo de sondeo son un límite explícito.
    pub fn open(
        mut reader: ResourceStream,
        chunk_bytes: usize,
        queue_chunks: usize,
        decoder_limit: usize,
    ) -> Result<Self, AsyncPlaybackError> {
        let length = u32::try_from(reader.len()).map_err(|_| AsyncPlaybackError::Limit)?;
        if length == 0 || length > i32::MAX as u32 {
            return Err(AsyncPlaybackError::Limit);
        }
        let mut header = vec![0; (length as usize).min(4096)];
        reader
            .read_exact(&mut header)
            .map_err(|e| AsyncPlaybackError::Input(InputError::Io(e.kind())))?;
        let (rate, channels, timing) = probe_mp3_format(&header).map_err(AsyncPlaybackError::Probe)?;
        reader
            .seek(SeekFrom::Start(0))
            .map_err(|e| AsyncPlaybackError::Input(InputError::Io(e.kind())))?;
        let source = AsyncInput::new(Box::new(reader), u64::from(length), chunk_bytes, queue_chunks)?;
        Self::from_source(source, length, rate, channels, timing, chunk_bytes, decoder_limit)
    }
    /// El lector ya abierto puede ser un puerto de E/S utilizado en pruebas.
    pub fn from_source(
        source: AsyncInput,
        length: u32,
        rate: u32,
        channels: u16,
        timing: MilesTiming,
        chunk_bytes: usize,
        decoder_limit: usize,
    ) -> Result<Self, AsyncPlaybackError> {
        if length == 0
            || length > i32::MAX as u32
            || rate == 0
            || !matches!(channels, 1 | 2)
            || chunk_bytes == 0
            || chunk_bytes > decoder_limit
            || timing.format_flags != if channels == 2 { 3 } else { 1 }
            || !matches!(timing.codec_byte_rate, Some(1..))
        {
            return Err(AsyncPlaybackError::InvalidFormat);
        }
        Ok(Self {
            source,
            hstream: None,
            hstream_serviced: false,
            decoder: ProgressiveMp3::new(decoder_limit)?,
            ring: SampleInput::default(),
            timing,
            sample_rate: rate,
            channels,
            length,
            rendered_frames: 0,
            max_frames: usize::MAX,
            stats: UnderrunStats::default(),
            underrun_policy: UnderrunPolicy::default(),
            status: AsyncPlaybackStatus::Paused,
            pending_slot: None,
            eof_marker: false,
            loops: 1,
            loops_left: 1,
            started: false,
            input_scratch: vec![0; chunk_bytes],
            pcm_scratch: Vec::new(),
            deferred: None,
            source_error: None,
        })
    }
    /// Opta por la programación HSTREAM de tres búferes antes de arrancar.
    pub fn enable_hstream(&mut self, capacity: usize, chunk: usize) -> Result<(), AsyncPlaybackError> {
        if self.started || self.hstream.is_some() {
            return Err(AsyncPlaybackError::InvalidFormat);
        }
        let mut buffers = BufferService::new(self.length, capacity, chunk).map_err(AsyncPlaybackError::Service)?;
        buffers.loop_end = i32::MAX as u32;
        buffers.codec_format = true;
        buffers.loops = self.loops;
        buffers.budget_default = u32::try_from(capacity.checked_mul(3).ok_or(AsyncPlaybackError::Limit)?)
            .map_err(|_| AsyncPlaybackError::Limit)?;
        buffers.budget = buffers.budget_default;
        self.hstream = Some(AsyncHstream::new(buffers).map_err(|e| match e {
            AsyncServiceError::Service(e) => AsyncPlaybackError::Service(e),
            AsyncServiceError::Input(e) => AsyncPlaybackError::Input(e),
        })?);
        Ok(())
    }
    pub fn set_loop_count(&mut self, n: u32) {
        self.loops = n;
        self.loops_left = n;
        if let Some(hstream) = &mut self.hstream {
            hstream.buffers.loops = n;
        }
    }
    pub fn loops_left(&self) -> u32 {
        self.hstream.as_ref().map_or(self.loops_left, |h| h.buffers.loops)
    }
    pub fn start(&mut self) -> Result<(), AsyncPlaybackError> {
        if self.started {
            self.seek_ms(0)?;
        }
        self.started = true;
        self.loops_left = self.loops;
        if let Some(h) = &mut self.hstream {
            h.buffers.loops = self.loops;
        }
        self.status = AsyncPlaybackStatus::Playing;
        Ok(())
    }
    pub fn pause(&mut self, paused: bool) -> Result<(), AsyncPlaybackError> {
        if !paused && !self.started {
            return self.start();
        }
        if self.status == AsyncPlaybackStatus::End {
            return Ok(());
        }
        self.status = if paused {
            AsyncPlaybackStatus::Paused
        } else {
            AsyncPlaybackStatus::Playing
        };
        Ok(())
    }
    pub fn seek_ms(&mut self, ms: u32) -> Result<u32, AsyncPlaybackError> {
        if let Some(e) = self.deferred.clone() {
            return Err(e);
        }
        let requested = self.timing.seek_bytes(ms).map_err(AsyncPlaybackError::Timing)?;
        let at = self
            .timing
            .align_with_provider(requested, self.length, true)
            .map_err(AsyncPlaybackError::Timing)?;
        self.decoder.reset_input(at == 0)?;
        self.source.seek(u64::from(at))?;
        self.ring = SampleInput::default();
        self.pending_slot = None;
        self.eof_marker = false;
        self.source_error = None;
        self.hstream_serviced = false;
        if let Some(h) = &mut self.hstream {
            h.cancel();
            let b = &mut h.buffers;
            b.file_position = at;
            b.logical_position = at;
            b.end_state = 0;
            b.loop_end = i32::MAX as u32;
            b.pad_count = u32::MAX;
            b.started = false;
            b.order = [0, 1, 2];
            b.budget = b.budget_default;
            for buffer in &mut b.buffers {
                buffer.used = 0;
            }
        }
        let actual = self.timing.milliseconds(at).map_err(AsyncPlaybackError::Timing)?;
        self.rendered_frames = (u64::from(actual) * u64::from(self.sample_rate) / 1000) as usize;
        Ok(at)
    }
    fn pump(&mut self) -> Result<bool, AsyncPlaybackError> {
        if self.hstream.is_some() {
            return self.pump_hstream();
        }
        let mut starved = false;
        for _ in 0..2 {
            if self.eof_marker || self.source_error.is_some() {
                break;
            }
            if self.pending_slot.is_none() {
                self.pending_slot = self.ring.buffer_ready()?;
            }
            let Some(slot) = self.pending_slot else { break };
            let r = match self.source.try_read(&mut self.input_scratch) {
                Ok(r) => r,
                Err(e) => {
                    self.source_error = Some(e);
                    break;
                }
            };
            if r.state == InputState::Contended {
                self.stats.lock_contentions = self.stats.lock_contentions.saturating_add(1);
                break;
            }
            if r.bytes > 0 {
                self.ring.load(slot, &self.input_scratch[..r.bytes])?;
                self.pending_slot = None;
            } else if r.state == InputState::End {
                self.ring.load(slot, &[])?;
                self.pending_slot = None;
                self.eof_marker = true;
                break;
            } else {
                starved = r.state == InputState::Starved;
                break;
            }
        }
        Ok(starved)
    }
    fn pump_hstream(&mut self) -> Result<bool, AsyncPlaybackError> {
        struct Sink<'a> {
            ring: &'a mut SampleInput,
            error: Option<SampleInputError>,
            eof: bool,
        }
        impl SampleSink for Sink<'_> {
            fn buffer_ready(&mut self) -> Option<u32> {
                match self.ring.buffer_ready() {
                    Ok(slot) => slot.map(|s| s as u32),
                    Err(e) => {
                        self.error = Some(e);
                        None
                    }
                }
            }
            fn load_buffer(&mut self, slot: u32, data: &[u8]) {
                if data.is_empty() {
                    self.eof = true;
                }
                if let Err(e) = self.ring.load(slot as usize, data) {
                    self.error = Some(e);
                }
            }
        }
        let h = self.hstream.as_mut().ok_or(AsyncPlaybackError::InvalidFormat)?;
        let b = &self.ring.buffers;
        let info = [
            b[0].position as u32,
            b[0].data.len() as u32,
            b[1].position as u32,
            b[1].data.len() as u32,
        ];
        if h.buffers.end_state == 0 || h.in_progress() {
            let previous = h.lock_contentions;
            let result = h.poll(&self.source, &mut self.decoder, info, true);
            self.stats.lock_contentions = self
                .stats
                .lock_contentions
                .saturating_add(h.lock_contentions - previous);
            match result {
                Ok(ServicePoll::Pending) => return Ok(true),
                Ok(ServicePoll::Complete(_)) => {}
                Err(AsyncServiceError::Input(e)) => {
                    self.source_error = Some(e);
                    return Ok(false);
                }
                Err(AsyncServiceError::Service(e)) => return Err(AsyncPlaybackError::Service(e)),
            }
        }
        let mut sink = Sink {
            ring: &mut self.ring,
            error: None,
            eof: false,
        };
        for _ in 0..2 {
            if h.buffers
                .dispatch(&mut sink, 1)
                .map_err(AsyncPlaybackError::Service)?
                .is_none()
            {
                break;
            }
        }
        if let Some(e) = sink.error {
            return Err(e.into());
        }
        if sink.eof {
            self.eof_marker = true;
        }
        h.buffers.provider_open = true;
        Ok(false)
    }
    /// Traslada únicamente lo ya disponible; no espera al productor.
    pub fn service(&mut self) -> Result<(), AsyncPlaybackError> {
        if self.status != AsyncPlaybackStatus::Paused
            && self.status != AsyncPlaybackStatus::End
            && (self.hstream.is_none() || !self.hstream_serviced)
        {
            let pending = self.pump()?;
            if self.hstream.is_some() && !pending {
                self.hstream_serviced = true;
            }
        }
        Ok(())
    }
    /// Indica si el servicio previo está preparado para el siguiente callback.
    pub fn service_ready(&self) -> bool {
        self.hstream_serviced
    }
    /// No espera al productor. La política predeterminada rellena la falta con
    /// silencio; la política experimental puede consumir una primera trama corta.
    pub fn read(&mut self, out: &mut [i16]) -> Result<AsyncPlaybackRead, AsyncPlaybackError> {
        let channels = self.channels as usize;
        if !out.len().is_multiple_of(channels) {
            return Err(AsyncPlaybackError::InvalidFormat);
        }
        out.fill(0);
        if self.status == AsyncPlaybackStatus::Paused || self.status == AsyncPlaybackStatus::End {
            return Ok(AsyncPlaybackRead {
                frames: 0,
                status: self.status,
            });
        }
        if let Some(e) = self.deferred.clone() {
            return Err(e);
        }
        let requested = out.len() / channels;
        if requested > self.max_frames.saturating_sub(self.rendered_frames) {
            return Err(AsyncPlaybackError::Limit);
        }
        let bytes = out.len().checked_mul(2).ok_or(AsyncPlaybackError::Limit)?;
        if self.pcm_scratch.len() < bytes {
            self.pcm_scratch
                .try_reserve(bytes - self.pcm_scratch.len())
                .map_err(|_| AsyncPlaybackError::Limit)?;
            self.pcm_scratch.resize(bytes, 0);
        }
        let mut written = 0usize;
        let mut underrun = false;
        let mut attempts = 0usize;
        // HSTREAM se ejecuta antes del callback, aunque quede PCM pendiente.
        let mut pre_pump = if self.hstream.is_some() {
            if std::mem::take(&mut self.hstream_serviced) {
                Some(false)
            } else {
                Some(self.pump()?)
            }
        } else {
            None
        };
        while written < bytes {
            attempts += 1;
            if attempts > 100_000 {
                return Err(AsyncPlaybackError::Limit);
            }
            let result = match self.decoder.read(&mut self.pcm_scratch[written..bytes]) {
                Ok(r) => r,
                Err(e) if written > 0 => {
                    self.deferred = Some(e.into());
                    break;
                }
                Err(e) => return Err(e.into()),
            };
            written += result.written;
            match result.status {
                ProgressiveStatus::Filled => continue,
                ProgressiveStatus::ErrorPending => break,
                ProgressiveStatus::End => {
                    self.status = AsyncPlaybackStatus::End;
                    break;
                }
                ProgressiveStatus::NeedInput => {}
            }
            let starved = if self.hstream.is_some() {
                pre_pump.take().unwrap_or(true)
            } else {
                match self.pump() {
                    Ok(value) => value,
                    Err(e) if written > 0 => {
                        self.deferred = Some(e);
                        break;
                    }
                    Err(e) => return Err(e),
                }
            };
            let need = if self.hstream.is_some() {
                1
            } else {
                self.input_scratch.len()
            }
            .min(self.decoder.remaining_capacity());
            if need == 0 {
                return Err(AsyncPlaybackError::Limit);
            }
            let before = self.ring.queued_end_callbacks;
            let got = self.ring.fetch(&mut self.input_scratch[..need], None)?;
            if self.ring.starved || self.ring.queued_end_callbacks != before {
                self.stats.callback_starvations = self.stats.callback_starvations.saturating_add(1);
            }
            if got > 0 {
                self.decoder.push(&self.input_scratch[..got])?;
                continue;
            }
            let empty = self.ring.buffers.iter().all(|b| b.position == b.data.len());
            if empty {
                if let Some(error) = self.source_error {
                    if written > 0 {
                        self.deferred = Some(error.into());
                        break;
                    }
                    return Err(error.into());
                }
            }
            if self.eof_marker && empty {
                if self.hstream.is_some() || self.loops_left == 1 {
                    self.decoder.finish();
                    continue;
                }
                if self.source.try_seek(0)?.is_none() {
                    self.stats.lock_contentions = self.stats.lock_contentions.saturating_add(1);
                    underrun = true;
                    break;
                }
                if self.loops_left > 1 {
                    self.loops_left -= 1;
                }
                self.decoder.discontinuity()?;
                self.ring = SampleInput::default();
                self.pending_slot = None;
                self.eof_marker = false;
                continue;
            }
            underrun = true;
            if starved
                && self.underrun_policy == UnderrunPolicy::OriginalFirstFrameGuarded
                && self.decoder.try_original_short_first_frame()
            {
                self.stats.original_short_frames = self.stats.original_short_frames.saturating_add(1);
                continue;
            }
            break;
        }
        if !written.is_multiple_of(2 * channels) {
            return Err(AsyncPlaybackError::InvalidFormat);
        }
        for (dst, src) in out.iter_mut().zip(self.pcm_scratch[..written].as_chunks::<2>().0) {
            *dst = i16::from_le_bytes(*src);
        }
        let frames = written / (2 * channels);
        self.rendered_frames = self
            .rendered_frames
            .checked_add(frames)
            .ok_or(AsyncPlaybackError::Limit)?;
        if underrun {
            self.stats.calls = self.stats.calls.saturating_add(1);
            self.stats.silent_frames = self.stats.silent_frames.saturating_add((requested - frames) as u64);
            return Ok(AsyncPlaybackRead {
                frames,
                status: AsyncPlaybackStatus::Underrun,
            });
        }
        Ok(AsyncPlaybackRead {
            frames,
            status: self.status,
        })
    }
}
