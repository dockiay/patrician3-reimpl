//! Reproductor MP3 progresivo fuera de línea sobre HSTREAM y sample de dos buffers.
//! La fuente se conserva en memoria y no hay hilo ni dispositivo físico.
use super::hstream::{PauseControl, SeekResult};
use super::hstream_buffers::{
    BufferService, ProgressiveFileIo, ProgressiveSample, SampleSink, ServiceError, StreamController,
};
use super::mp3_decoder::{DecodeError, ProgressiveStatus};
use super::mp3_frame::{audio_start, parse_final_frame, FrameError};
use super::sample_input::{SampleInput, SampleInputError};
use super::timing::{MilesTiming, TimingError};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlaybackError {
    Decoder(DecodeError),
    Frame(FrameError),
    Timing(TimingError),
    Service(ServiceError),
    Sample(SampleInputError),
    InvalidConfiguration,
    Limit,
    Stalled,
}
impl From<DecodeError> for PlaybackError {
    fn from(e: DecodeError) -> Self {
        Self::Decoder(e)
    }
}
impl From<ServiceError> for PlaybackError {
    fn from(e: ServiceError) -> Self {
        Self::Service(e)
    }
}
impl From<SampleInputError> for PlaybackError {
    fn from(e: SampleInputError) -> Self {
        Self::Sample(e)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaybackStatus {
    Playing,
    Paused,
    End,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlaybackRead {
    pub frames: usize,
    pub status: PlaybackStatus,
}

struct RingSink<'a> {
    ring: &'a mut SampleInput,
    error: Option<SampleInputError>,
    status: u32,
}
impl SampleSink for RingSink<'_> {
    fn buffer_ready(&mut self) -> Option<u32> {
        match self.ring.buffer_ready() {
            Ok(n) => n.map(|v| v as u32),
            Err(e) => {
                self.error = Some(e);
                None
            }
        }
    }
    fn load_buffer(&mut self, slot: u32, data: &[u8]) {
        if let Err(e) = self.ring.load(slot as usize, data) {
            self.error = Some(e);
        }
    }
}
impl super::hstream_buffers::PauseSample for RingSink<'_> {
    fn status(&mut self) -> u32 {
        self.status
    }
    fn end_sample(&mut self) {
        self.status = 8;
    }
    fn resume_sample(&mut self) {
        self.status = 4;
    }
    fn start_stream(&mut self) {
        self.status = 4;
    }
}

pub struct ProgressivePlayback {
    input: Arc<[u8]>,
    file_pos: usize,
    pub controller: StreamController,
    pub sample: ProgressiveSample,
    pub ring: SampleInput,
    pub timing: MilesTiming,
    pub sample_rate: u32,
    pub channels: u16,
    pub status: PlaybackStatus,
    pub rendered_frames: usize,
    pub max_frames: usize,
    started: bool,
    deferred_error: Option<PlaybackError>,
}
/// Metadatos de la primera trama; la tasa de bytes es una estimación si hay VBR.
pub fn probe_mp3_format(input: &[u8]) -> Result<(u32, u16, MilesTiming), PlaybackError> {
    let start = audio_start(input).map_err(PlaybackError::Frame)?;
    let frame = parse_final_frame(input, start).map_err(PlaybackError::Frame)?;
    let word = u32::from_be_bytes(input[start..start + 4].try_into().unwrap());
    let index = ((word >> 12) & 15) as usize;
    let br = if frame.version == 1 {
        [0, 32, 40, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320, 0][index]
    } else {
        [0, 8, 16, 24, 32, 40, 48, 56, 64, 80, 96, 112, 128, 144, 160, 0][index]
    };
    let timing = MilesTiming {
        format_flags: if frame.channels == 2 { 3 } else { 1 },
        sample_rate: frame.sample_rate,
        codec_byte_rate: Some(br * 1000 / 8),
        block_size: frame.length as u32,
    };
    Ok((frame.sample_rate, frame.channels as u16, timing))
}
impl ProgressivePlayback {
    pub fn new(input: Arc<[u8]>, buffer_size: usize, input_limit: usize) -> Result<Self, PlaybackError> {
        let (rate, channels, timing) = probe_mp3_format(&input)?;
        let mut buffers = BufferService::new(
            u32::try_from(input.len()).map_err(|_| PlaybackError::Limit)?,
            buffer_size,
            buffer_size,
        )
        .map_err(PlaybackError::Service)?;
        buffers.loop_end = i32::MAX as u32;
        buffers.codec_format = true;
        buffers.provider_open = true;
        buffers.budget_default =
            u32::try_from(buffer_size.checked_mul(3).ok_or(PlaybackError::Limit)?).map_err(|_| PlaybackError::Limit)?;
        buffers.budget = buffers.budget_default;
        Ok(Self {
            input,
            file_pos: 0,
            controller: StreamController {
                buffers,
                pause: PauseControl {
                    control: 0,
                    started: false,
                },
            },
            sample: ProgressiveSample::new(input_limit, buffer_size)?,
            ring: SampleInput::default(),
            timing,
            sample_rate: rate,
            channels,
            status: PlaybackStatus::Paused,
            rendered_frames: 0,
            max_frames: usize::MAX,
            started: false,
            deferred_error: None,
        })
    }
    pub fn set_loop_count(&mut self, n: u32) {
        self.controller.buffers.loops = n;
    }
    pub fn start(&mut self) -> Result<(), PlaybackError> {
        use super::hstream::{StartAction, StartControl};
        let mut control = StartControl {
            direct: false,
            started: self.controller.buffers.started,
            pending: 0,
            control: self.controller.pause.control,
        };
        for action in control.start() {
            match action {
                StartAction::SeekZero => {
                    self.seek_ms(0)?;
                }
                StartAction::Service => {
                    let mut sink = RingSink {
                        ring: &mut self.ring,
                        error: None,
                        status: 4,
                    };
                    if self
                        .controller
                        .buffers
                        .dispatch(&mut sink, self.controller.pause.control)?
                        .is_some()
                    {
                        self.controller
                            .buffers
                            .dispatch(&mut sink, self.controller.pause.control)?;
                    }
                    if let Some(e) = sink.error {
                        return Err(e.into());
                    }
                }
                StartAction::EndSample => {}
                StartAction::StartSample => return Err(PlaybackError::InvalidConfiguration),
            }
        }
        self.started = true;
        self.status = PlaybackStatus::Playing;
        self.controller.pause.control = control.control;
        Ok(())
    }
    pub fn pause(&mut self, paused: bool) -> Result<(), PlaybackError> {
        if !paused && !self.started {
            return self.start();
        }
        if !paused && self.controller.pause.control & 7 == 0 {
            return self.start();
        }
        if self.status == PlaybackStatus::End {
            return Ok(());
        }
        let status = if self.status == PlaybackStatus::Playing { 4 } else { 8 };
        let mut sink = RingSink {
            ring: &mut self.ring,
            error: None,
            status,
        };
        self.controller.set_paused(&mut sink, paused)?;
        if let Some(e) = sink.error {
            return Err(e.into());
        }
        self.status = if paused {
            PlaybackStatus::Paused
        } else {
            PlaybackStatus::Playing
        };
        Ok(())
    }
    pub fn seek_ms(&mut self, ms: u32) -> Result<SeekResult, PlaybackError> {
        if let Some(error) = self.deferred_error.clone() {
            return Err(error);
        }
        let v = self
            .controller
            .seek_progressive_ms(&mut self.sample, &mut self.file_pos, self.timing, ms)
            .map_err(|e| match e {
                super::hstream_buffers::ProgressiveSeekError::Timing(x) => PlaybackError::Timing(x),
                super::hstream_buffers::ProgressiveSeekError::Provider(x) => PlaybackError::Decoder(x),
                _ => PlaybackError::InvalidConfiguration,
            })?;
        self.ring = SampleInput::default();
        let actual_ms = self
            .timing
            .milliseconds(v.aligned_byte)
            .map_err(PlaybackError::Timing)?;
        self.rendered_frames = ((actual_ms as u64) * u64::from(self.sample_rate) / 1000) as usize;
        Ok(v)
    }
    fn pump(&mut self) -> Result<(), PlaybackError> {
        let b = &self.ring.buffers;
        let info = [
            b[0].position as u32,
            b[0].data.len() as u32,
            b[1].position as u32,
            b[1].data.len() as u32,
        ];
        if self.controller.buffers.end_state == 0 {
            let mut io = ProgressiveFileIo {
                input: &self.input,
                position: self.file_pos,
                decoder: &mut self.sample.decoder,
            };
            self.controller.buffers.service(&mut io, info, true)?;
            self.file_pos = io.position;
        }
        let mut sink = RingSink {
            ring: &mut self.ring,
            error: None,
            status: 4,
        };
        for _ in 0..2 {
            if self
                .controller
                .buffers
                .dispatch(&mut sink, self.controller.pause.control)?
                .is_none()
            {
                break;
            }
        }
        if let Some(e) = sink.error {
            return Err(e.into());
        }
        Ok(())
    }
    /// Alimenta los búferes disponibles sin consumir PCM.
    pub fn service(&mut self) -> Result<(), PlaybackError> {
        if self.status == PlaybackStatus::Playing {
            self.pump()?;
        }
        Ok(())
    }
    /// Entrega PCM16 intercalado. Falta temporal de entrada nunca se convierte en EOF.
    pub fn read(&mut self, out: &mut [i16]) -> Result<PlaybackRead, PlaybackError> {
        let channels = self.channels as usize;
        if !out.len().is_multiple_of(channels) {
            return Err(PlaybackError::InvalidConfiguration);
        }
        if self.status != PlaybackStatus::Playing {
            out.fill(0);
            return Ok(PlaybackRead {
                frames: 0,
                status: self.status,
            });
        }
        if let Some(error) = self.deferred_error.clone() {
            return Err(error);
        }
        let requested_frames = out.len() / channels;
        if requested_frames > self.max_frames.saturating_sub(self.rendered_frames) {
            return Err(PlaybackError::Limit);
        }
        let mut bytes = vec![0u8; out.len().checked_mul(2).ok_or(PlaybackError::Limit)?];
        let mut written = 0usize;
        let mut attempts = 0usize;
        loop {
            if written == bytes.len() {
                break;
            }
            attempts += 1;
            if attempts > 100_000 {
                return Err(PlaybackError::Stalled);
            }
            let result = match self.sample.read(&mut bytes[written..]) {
                Ok(result) => result,
                Err(error) if written > 0 => {
                    self.deferred_error = Some(error.into());
                    break;
                }
                Err(error) => return Err(error.into()),
            };
            written += result.written;
            match result.status {
                ProgressiveStatus::Filled => continue,
                ProgressiveStatus::End => {
                    self.status = PlaybackStatus::End;
                    break;
                }
                // Entregar el PCM ya producido; el siguiente read devuelve el error guardado.
                ProgressiveStatus::ErrorPending => break,
                ProgressiveStatus::NeedInput => {}
            }
            if let Err(error) = self.pump() {
                if written > 0 {
                    self.deferred_error = Some(error);
                    break;
                }
                return Err(error);
            }
            let active = self.ring.active;
            let next = 1 - active;
            let current = &self.ring.buffers[active];
            let other = &self.ring.buffers[next];
            let mut available = current.data.len().saturating_sub(current.position);
            if !other.done && other.position == 0 {
                available = available.saturating_add(other.data.len());
            }
            if available > 0 {
                let n = available.min(512).min(self.sample.decoder.remaining_capacity());
                if n == 0 {
                    return Err(PlaybackError::Limit);
                }
                let mut chunk = vec![0; n];
                let got = self.ring.fetch(&mut chunk, None)?;
                self.sample.decoder.push(&chunk[..got])?;
            } else if self.controller.buffers.end_state == 2 {
                self.sample.decoder.finish();
            } else {
                return Err(PlaybackError::Stalled);
            }
        }
        if !written.is_multiple_of(2 * channels) {
            return Err(PlaybackError::InvalidConfiguration);
        }
        for (dst, src) in out.iter_mut().zip(bytes[..written].as_chunks::<2>().0) {
            *dst = i16::from_le_bytes([src[0], src[1]]);
        }
        out[written / 2..].fill(0);
        let frames = written / (2 * channels);
        self.rendered_frames = self.rendered_frames.checked_add(frames).ok_or(PlaybackError::Limit)?;
        Ok(PlaybackRead {
            frames,
            status: self.status,
        })
    }
}
