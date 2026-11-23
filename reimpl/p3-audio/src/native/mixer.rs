//! Canales y mezcla PCM16 estéreo fuera de línea. Sin hilo, reloj o dispositivo.
//! Ruta escalar con ganancia Q11, Lowpass opcional y sin remuestreo.
use super::ima::{DecodeError, Pcm16};
use super::lowpass::{Lowpass, LowpassError, LowpassSnapshot};
use super::stream::{StreamLimits, StreamSnapshot, StreamVoice};
use super::{EffectDevice, WaveLease};
use crate::Backend;
use std::collections::BTreeMap;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SampleState {
    Done = 2,
    Playing = 4,
    Stopped = 8,
}
#[derive(Debug, Clone, Copy)]
pub enum MixMode {
    /// Selector seco de 0x21111D90, PCM16 a estéreo16 y a igual frecuencia.
    DryDispatch,
    /// Ruta con escala forzada, para reproducir el corpus histórico de kernels.
    ScaledKernel,
}
#[derive(Debug, Clone, Copy)]
pub struct MixerConfig {
    pub mode: MixMode,
    pub output_rate: u32,
    pub max_voices: usize,
    pub max_render_frames: usize,
    pub max_clip_frames: usize,
    pub max_pcm_samples: usize,
}
impl Default for MixerConfig {
    fn default() -> Self {
        Self {
            mode: MixMode::DryDispatch,
            output_rate: 44100,
            max_voices: 32,
            max_render_frames: 65536,
            max_clip_frames: 2_000_000,
            max_pcm_samples: 128 * 1024 * 1024,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MixerError {
    Config,
    UnknownDriver(u32),
    UnknownSample(u32),
    VoiceLimit,
    TokenLimit,
    RenderLimit,
    ClipLimit,
    PcmBudget,
    InvalidPcm,
    NoData,
    RateMismatch { source: u32, output: u32 },
    UnsupportedBlock(u32),
    UnsupportedStream(u32),
    Decode(DecodeError),
    Capacity,
    UnknownStream(u32),
    StreamLimit,
    Lowpass(LowpassError),
    LowpassNotInstalled(u32),
    Progressive(super::progressive_playback::PlaybackError),
    Async(super::async_playback::AsyncPlaybackError),
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VoiceSnapshot {
    pub state: SampleState,
    pub cursor: usize,
    pub frames: usize,
    pub channels: u16,
    pub volume: i32,
    pub pan: i32,
    pub gains: [i32; 2],
    pub lowpass: Option<LowpassSnapshot>,
}
#[derive(Debug, PartialEq, Eq)]
pub struct FinishedSample {
    pub handle: u32,
    /// Número de fotogramas consumidos en este bloque hasta la terminación.
    pub frame_offset: usize,
}
#[derive(Debug, PartialEq, Eq)]
pub struct FinishedStream {
    pub handle: u32,
    pub frame_offset: usize,
}
#[derive(Debug, PartialEq, Eq)]
pub struct RenderedPcm {
    pub samples: Vec<i16>,
    /// Terminaciones naturales de este render, no callbacks de Miles.
    pub finished: Vec<FinishedSample>,
    pub finished_streams: Vec<FinishedStream>,
}
struct Voice {
    pcm: Option<Arc<Pcm16>>,
    progressive: Option<super::progressive_playback::ProgressivePlayback>,
    asynchronous: Option<super::async_playback::AsyncPlayback>,
    state: SampleState,
    cursor: usize,
    volume: i32,
    pan: i32,
    loop_count: u32,
    loops_left: u32,
    lowpass: Option<Lowpass>,
}
impl Voice {
    fn new() -> Self {
        Self {
            pcm: None,
            progressive: None,
            asynchronous: None,
            state: SampleState::Done,
            cursor: 0,
            volume: 127,
            pan: 64,
            loop_count: 1,
            loops_left: 1,
            lowpass: None,
        }
    }
    fn sample_count(&self) -> usize {
        self.pcm.as_ref().map_or(0, |p| p.samples.len())
    }
}

/// Coeficientes de 0x21113A90, para master ya normalizado por la API nativa.
/// La tabla de pan observada es 2*p hasta 62; 128 desde 63, sin valor 126.
pub fn sample_gains(volume: i32, pan: i32, master: i32) -> [i32; 2] {
    let effective = master.clamp(0, 127) * volume.clamp(0, 127) / 127;
    let pan = pan.clamp(0, 127);
    let table = |p| if p < 63 { 2 * p } else { 128 };
    let scale = |p| {
        let v = table(p) * effective / 8;
        if v >= 0x7f0 {
            0x7ff
        } else {
            v
        }
    };
    [scale(127 - pan), scale(pan)]
}

/// Dominio seco PCM16 mono/estéreo a estéreo16, sin inversión ni remuestreo.
pub fn mix_operation(channels: u16, gains: [i32; 2], mode: MixMode) -> u32 {
    3 | if channels == 2 { 8 } else { 0 }
        | if matches!(mode, MixMode::ScaledKernel) || gains != [2047; 2] {
            16
        } else {
            0
        }
}

pub struct OfflineDevice {
    config: MixerConfig,
    voices: BTreeMap<u32, Voice>,
    next: Option<u32>,
    owned_samples: usize,
    master: i32,
    errors: Vec<MixerError>,
    delete_requests: usize,
    slot_clear_requests: usize,
    streams: BTreeMap<u32, StreamVoice>,
    next_stream: Option<u32>,
}
impl OfflineDevice {
    pub const DRIVER: u32 = 1;
    pub fn new(config: MixerConfig) -> Result<Self, MixerError> {
        // 32 voces con Q11 quedan dentro del acumulador i32 en este alcance seco.
        if config.output_rate == 0 || config.max_voices == 0 || config.max_voices > 32 {
            return Err(MixerError::Config);
        }
        Ok(Self {
            config,
            voices: BTreeMap::new(),
            next: Some(1),
            owned_samples: 0,
            master: 127,
            errors: vec![],
            delete_requests: 0,
            slot_clear_requests: 0,
            streams: BTreeMap::new(),
            next_stream: Some(0x80000000),
        })
    }
    pub fn take_errors(&mut self) -> Vec<MixerError> {
        std::mem::take(&mut self.errors)
    }
    pub fn take_delete_requests(&mut self) -> usize {
        std::mem::take(&mut self.delete_requests)
    }
    pub fn take_slot_clear_requests(&mut self) -> usize {
        std::mem::take(&mut self.slot_clear_requests)
    }
    pub fn owned_samples(&self) -> usize {
        self.owned_samples
    }
    pub fn voice_count(&self) -> usize {
        self.voices.len()
    }
    pub fn set_master_volume(&mut self, volume: i32) {
        self.master = volume.clamp(0, 127);
    }
    pub fn master_volume(&self) -> i32 {
        self.master
    }
    pub fn snapshot(&self, handle: u32) -> Result<VoiceSnapshot, MixerError> {
        let v = self.voices.get(&handle).ok_or(MixerError::UnknownSample(handle))?;
        let (frames, channels) = if let Some(p) = &v.asynchronous {
            let ms = p
                .timing
                .milliseconds(p.length)
                .map_err(|e| MixerError::Async(super::async_playback::AsyncPlaybackError::Timing(e)))?;
            ((u64::from(ms) * u64::from(p.sample_rate) / 1000) as usize, p.channels)
        } else if let Some(p) = &v.progressive {
            let ms = p
                .timing
                .milliseconds(p.controller.buffers.total)
                .map_err(|e| MixerError::Progressive(super::progressive_playback::PlaybackError::Timing(e)))?;
            ((u64::from(ms) * u64::from(p.sample_rate) / 1000) as usize, p.channels)
        } else {
            (
                v.pcm.as_ref().map_or(0, |p| p.frames()),
                v.pcm.as_ref().map_or(0, |p| p.channels),
            )
        };
        Ok(VoiceSnapshot {
            state: v.state,
            cursor: v.cursor,
            frames,
            channels,
            volume: v.volume,
            pan: v.pan,
            gains: sample_gains(v.volume, v.pan, self.master),
            lowpass: v.lowpass.as_ref().map(Lowpass::snapshot),
        })
    }
    pub fn allocate(&mut self, driver: u32) -> Result<u32, MixerError> {
        if driver != Self::DRIVER {
            return Err(MixerError::UnknownDriver(driver));
        }
        if self.voices.len() == self.config.max_voices {
            return Err(MixerError::VoiceLimit);
        }
        let h = self.next.ok_or(MixerError::TokenLimit)?;
        self.next = h.checked_add(1);
        self.voices.insert(h, Voice::new());
        Ok(h)
    }
    fn voice_mut(&mut self, h: u32) -> Result<&mut Voice, MixerError> {
        self.voices.get_mut(&h).ok_or(MixerError::UnknownSample(h))
    }
    pub fn initialize(&mut self, h: u32) -> Result<(), MixerError> {
        let v = self.voice_mut(h)?;
        let count = v.sample_count();
        *v = Voice::new();
        self.owned_samples -= count;
        Ok(())
    }
    pub fn release(&mut self, h: u32) -> Result<(), MixerError> {
        let v = self.voices.remove(&h).ok_or(MixerError::UnknownSample(h))?;
        self.owned_samples -= v.sample_count();
        self.streams.retain(|_, s| s.sample != h);
        Ok(())
    }
    pub fn attach_pcm(&mut self, h: u32, pcm: Arc<Pcm16>) -> Result<(), MixerError> {
        self.attach_limited(h, pcm, self.config.max_clip_frames)
    }
    fn attach_limited(&mut self, h: u32, pcm: Arc<Pcm16>, max_frames: usize) -> Result<(), MixerError> {
        let old = self.voices.get(&h).ok_or(MixerError::UnknownSample(h))?.sample_count();
        if !matches!(pcm.channels, 1 | 2) || !pcm.samples.len().is_multiple_of(pcm.channels as usize) {
            return Err(MixerError::InvalidPcm);
        }
        if pcm.samples.is_empty() {
            return Err(MixerError::NoData);
        }
        if pcm.sample_rate != self.config.output_rate {
            return Err(MixerError::RateMismatch {
                source: pcm.sample_rate,
                output: self.config.output_rate,
            });
        }
        if pcm.frames() > max_frames {
            return Err(MixerError::ClipLimit);
        }
        if pcm.samples.len() > self.config.max_pcm_samples.saturating_sub(self.owned_samples - old) {
            return Err(MixerError::PcmBudget);
        }
        let len = pcm.samples.len();
        let v = self.voice_mut(h)?;
        v.progressive = None;
        v.asynchronous = None;
        v.pcm = Some(pcm);
        v.cursor = 0;
        v.state = SampleState::Done;
        self.owned_samples = self.owned_samples - old + len;
        Ok(())
    }
    pub fn attach(&mut self, h: u32, wave: WaveLease, block: u32) -> Result<(), MixerError> {
        self.voices.get(&h).ok_or(MixerError::UnknownSample(h))?;
        if block > 1 {
            return Err(MixerError::UnsupportedBlock(block));
        }
        let pcm = wave
            .decode_miles_ima(self.config.max_clip_frames)
            .map_err(MixerError::Decode)?;
        self.attach_pcm(h, Arc::new(pcm))
    }
    pub fn volume(&self, h: u32) -> Result<i32, MixerError> {
        Ok(self.snapshot(h)?.volume)
    }
    pub fn set_volume(&mut self, h: u32, volume: i32) -> Result<(), MixerError> {
        self.voice_mut(h)?.volume = volume.clamp(0, 127);
        Ok(())
    }
    pub fn set_pan(&mut self, h: u32, pan: i32) -> Result<(), MixerError> {
        self.voice_mut(h)?.pan = pan.clamp(0, 127);
        Ok(())
    }
    pub fn set_lowpass(&mut self, h: u32, enabled: bool) -> Result<(), MixerError> {
        let rate = self.config.output_rate;
        let v = self.voice_mut(h)?;
        if enabled {
            // AIL_set_sample_processor cierra/reabre incluso el mismo proveedor.
            v.lowpass = Some(Lowpass::new(rate).map_err(MixerError::Lowpass)?);
        } else {
            v.lowpass = None;
        }
        Ok(())
    }
    pub fn set_lowpass_cutoff(&mut self, h: u32, bits: u32) -> Result<u32, MixerError> {
        self.voice_mut(h)?
            .lowpass
            .as_mut()
            .ok_or(MixerError::LowpassNotInstalled(h))
            .map(|f| f.set_cutoff_bits(bits))
    }
    pub fn start(&mut self, h: u32) -> Result<(), MixerError> {
        let v = self.voice_mut(h)?;
        if let Some(p) = &mut v.asynchronous {
            p.set_loop_count(v.loop_count);
            p.start().map_err(MixerError::Async)?;
            v.cursor = 0;
            v.state = SampleState::Playing;
            return Ok(());
        }
        if let Some(p) = &mut v.progressive {
            p.set_loop_count(v.loop_count);
            p.start().map_err(MixerError::Progressive)?;
            v.cursor = 0;
            v.state = SampleState::Playing;
            return Ok(());
        }
        if v.pcm.is_none() {
            return Err(MixerError::NoData);
        }
        v.cursor = 0;
        v.state = SampleState::Playing;
        v.loops_left = v.loop_count;
        Ok(())
    }
    pub fn stop(&mut self, h: u32) -> Result<(), MixerError> {
        let v = self.voice_mut(h)?;
        if v.state == SampleState::Playing {
            if let Some(p) = &mut v.asynchronous {
                p.pause(true).map_err(MixerError::Async)?;
            }
            if let Some(p) = &mut v.progressive {
                p.pause(true).map_err(MixerError::Progressive)?;
            }
            v.state = SampleState::Stopped;
        }
        Ok(())
    }
    pub fn end(&mut self, h: u32) -> Result<(), MixerError> {
        let v = self.voice_mut(h)?;
        if let Some(p) = &mut v.asynchronous {
            p.pause(true).map_err(MixerError::Async)?;
        }
        if let Some(p) = &mut v.progressive {
            p.pause(true).map_err(MixerError::Progressive)?;
        }
        v.state = SampleState::Done;
        Ok(())
    }
    pub fn resume(&mut self, h: u32) -> Result<bool, MixerError> {
        let v = self.voice_mut(h)?;
        if let Some(p) = &mut v.asynchronous {
            if v.state == SampleState::Stopped {
                p.pause(false).map_err(MixerError::Async)?;
                v.state = SampleState::Playing;
                return Ok(true);
            }
            return Ok(false);
        }
        if let Some(p) = &mut v.progressive {
            if v.state == SampleState::Stopped {
                p.pause(false).map_err(MixerError::Progressive)?;
                v.state = SampleState::Playing;
                return Ok(true);
            }
            return Ok(false);
        }
        if v.state != SampleState::Playing && v.pcm.as_ref().is_some_and(|p| v.cursor < p.frames()) {
            v.state = SampleState::Playing;
            Ok(true)
        } else {
            Ok(false)
        }
    }
    fn result<T: Default>(&mut self, r: Result<T, MixerError>) -> T {
        match r {
            Ok(v) => v,
            Err(e) => {
                self.errors.push(e);
                T::default()
            }
        }
    }
    pub fn open_stream_pcm(&mut self, driver: u32, pcm: Arc<Pcm16>, limits: StreamLimits) -> Result<u32, MixerError> {
        if self.streams.len() >= limits.max_streams {
            return Err(MixerError::StreamLimit);
        }
        let stream = self.next_stream.ok_or(MixerError::TokenLimit)?;
        let sample = self.allocate(driver)?;
        if let Err(e) = self.attach_limited(sample, pcm, limits.max_frames) {
            self.release(sample)?;
            return Err(e);
        }
        self.next_stream = stream.checked_add(1);
        self.streams.insert(stream, StreamVoice { sample, started: false });
        Ok(stream)
    }
    /// Abre el decoder MP3 progresivo en el dispositivo lógico sin salida física.
    pub fn open_stream_mp3_progressive(
        &mut self,
        driver: u32,
        input: Arc<[u8]>,
        limits: StreamLimits,
    ) -> Result<u32, MixerError> {
        if self.streams.len() >= limits.max_streams {
            return Err(MixerError::StreamLimit);
        }
        if input.len() > limits.max_encoded_bytes {
            return Err(MixerError::ClipLimit);
        }
        let mut playback = super::progressive_playback::ProgressivePlayback::new(input, 4096, 65536)
            .map_err(MixerError::Progressive)?;
        if playback.sample_rate != self.config.output_rate {
            return Err(MixerError::RateMismatch {
                source: playback.sample_rate,
                output: self.config.output_rate,
            });
        }
        playback.max_frames = limits.max_frames;
        let stream = self.next_stream.ok_or(MixerError::TokenLimit)?;
        let sample = self.allocate(driver)?;
        self.voice_mut(sample)?.progressive = Some(playback);
        self.next_stream = stream.checked_add(1);
        self.streams.insert(stream, StreamVoice { sample, started: false });
        Ok(stream)
    }
    /// Abre un stream MP3 con lectura de recursos en un hilo productor acotado.
    pub fn open_stream_mp3_async(
        &mut self,
        driver: u32,
        source: super::files::ResourceStream,
        limits: StreamLimits,
    ) -> Result<u32, MixerError> {
        self.open_stream_mp3_async_impl(driver, source, limits, false)
    }
    pub fn open_stream_mp3_async_hstream(
        &mut self,
        driver: u32,
        source: super::files::ResourceStream,
        limits: StreamLimits,
    ) -> Result<u32, MixerError> {
        self.open_stream_mp3_async_impl(driver, source, limits, true)
    }
    fn open_stream_mp3_async_impl(
        &mut self,
        driver: u32,
        source: super::files::ResourceStream,
        limits: StreamLimits,
        hstream: bool,
    ) -> Result<u32, MixerError> {
        if self.streams.len() >= limits.max_streams {
            return Err(MixerError::StreamLimit);
        }
        if source.len() > limits.max_encoded_bytes as u64 {
            return Err(MixerError::ClipLimit);
        }
        if driver != Self::DRIVER {
            return Err(MixerError::UnknownDriver(driver));
        }
        if self.voices.len() == self.config.max_voices {
            return Err(MixerError::VoiceLimit);
        }
        let mut player =
            super::async_playback::AsyncPlayback::open(source, 4096, 3, 65536).map_err(MixerError::Async)?;
        if hstream {
            player.enable_hstream(4096, 4096).map_err(MixerError::Async)?;
        }
        if player.sample_rate != self.config.output_rate {
            return Err(MixerError::RateMismatch {
                source: player.sample_rate,
                output: self.config.output_rate,
            });
        }
        player.max_frames = limits.max_frames;
        let stream = self.next_stream.ok_or(MixerError::TokenLimit)?;
        let sample = self.allocate(driver)?;
        self.voice_mut(sample)?.asynchronous = Some(player);
        self.next_stream = stream.checked_add(1);
        self.streams.insert(stream, StreamVoice { sample, started: false });
        Ok(stream)
    }
    fn stream_sample_handle(&self, h: u32) -> Result<u32, MixerError> {
        Ok(self.streams.get(&h).ok_or(MixerError::UnknownStream(h))?.sample)
    }
    pub fn stream_snapshot(&self, h: u32) -> Result<StreamSnapshot, MixerError> {
        let s = self.streams.get(&h).ok_or(MixerError::UnknownStream(h))?;
        let v = self.voices.get(&s.sample).ok_or(MixerError::UnknownSample(s.sample))?;
        if let Some(p) = &v.asynchronous {
            let total_ms = p
                .timing
                .milliseconds(p.length)
                .map_err(|e| MixerError::Async(super::async_playback::AsyncPlaybackError::Timing(e)))?;
            let frames = (u64::from(total_ms) * u64::from(p.sample_rate) / 1000) as usize;
            return Ok(StreamSnapshot {
                sample: s.sample,
                state: v.state,
                cursor: v.cursor,
                frames,
                loop_count: v.loop_count,
                loops_left: p.loops_left(),
                started: s.started,
                total_ms,
                current_ms: super::timing::pcm_milliseconds(v.cursor, p.sample_rate),
            });
        }
        if let Some(p) = &v.progressive {
            let total_ms = p
                .timing
                .milliseconds(p.controller.buffers.total)
                .map_err(|e| MixerError::Progressive(super::progressive_playback::PlaybackError::Timing(e)))?;
            let frames = (u64::from(total_ms) * u64::from(p.sample_rate) / 1000) as usize;
            return Ok(StreamSnapshot {
                sample: s.sample,
                state: v.state,
                cursor: v.cursor,
                frames,
                loop_count: v.loop_count,
                loops_left: p.controller.buffers.loops,
                started: s.started,
                total_ms,
                current_ms: super::timing::pcm_milliseconds(v.cursor, p.sample_rate),
            });
        }
        let p = v.pcm.as_ref().ok_or(MixerError::NoData)?;
        let ms = |n: usize| super::timing::pcm_milliseconds(n, p.sample_rate);
        Ok(StreamSnapshot {
            sample: s.sample,
            state: v.state,
            cursor: v.cursor,
            frames: p.frames(),
            loop_count: v.loop_count,
            loops_left: v.loops_left,
            started: s.started,
            total_ms: ms(p.frames()),
            current_ms: ms(v.cursor),
        })
    }
    pub fn seek_stream_ms(&mut self, h: u32, ms: u32) -> Result<(), MixerError> {
        let sample = self.stream_sample_handle(h)?;
        let v = self.voice_mut(sample)?;
        if let Some(p) = &mut v.asynchronous {
            p.seek_ms(ms).map_err(MixerError::Async)?;
            v.cursor = p.rendered_frames;
            return Ok(());
        }
        if let Some(p) = &mut v.progressive {
            p.seek_ms(ms).map_err(MixerError::Progressive)?;
            v.cursor = p.rendered_frames;
            return Ok(());
        }
        let p = v.pcm.as_ref().ok_or(MixerError::NoData)?;
        v.cursor = super::timing::pcm_seek_frame(ms, p.sample_rate, p.channels, p.frames());
        if v.cursor == p.frames() && v.state == SampleState::Playing {
            v.state = SampleState::Done;
        }
        Ok(())
    }
    pub fn start_stream_handle(&mut self, h: u32) -> Result<(), MixerError> {
        let s = self.streams.get(&h).ok_or(MixerError::UnknownStream(h))?;
        let restarted = s.started;
        let sample = s.sample;
        let v = self.voice_mut(sample)?;
        if let Some(p) = &mut v.asynchronous {
            p.set_loop_count(v.loop_count);
            p.start().map_err(MixerError::Async)?;
            v.cursor = p.rendered_frames;
            v.state = SampleState::Playing;
            self.streams.get_mut(&h).expect("stream comprobado").started = true;
            return Ok(());
        }
        if let Some(p) = &mut v.progressive {
            p.set_loop_count(v.loop_count);
            p.start().map_err(MixerError::Progressive)?;
            v.cursor = p.rendered_frames;
            v.state = SampleState::Playing;
            self.streams.get_mut(&h).expect("stream comprobado").started = true;
            return Ok(());
        }
        let frames = v.pcm.as_ref().ok_or(MixerError::NoData)?.frames();
        if restarted {
            v.cursor = 0;
        }
        v.loops_left = v.loop_count;
        v.state = if v.cursor < frames {
            SampleState::Playing
        } else {
            SampleState::Done
        };
        self.streams.get_mut(&h).expect("stream comprobado").started = true;
        Ok(())
    }
    pub fn pause_stream_handle(&mut self, h: u32, pause: bool) -> Result<(), MixerError> {
        let s = self.streams.get(&h).ok_or(MixerError::UnknownStream(h))?;
        let (sample, started) = (s.sample, s.started);
        if pause {
            self.stop(sample)
        } else if !started {
            self.start_stream_handle(h)
        } else {
            self.resume(sample).map(|_| ())
        }
    }
    pub fn stream_loops(&mut self, h: u32, count: u32) -> Result<(), MixerError> {
        let sample = self.stream_sample_handle(h)?;
        let v = self.voice_mut(sample)?;
        if let Some(p) = &mut v.asynchronous {
            p.set_loop_count(count);
        }
        if let Some(p) = &mut v.progressive {
            p.set_loop_count(count);
        }
        v.loop_count = count;
        v.loops_left = count;
        Ok(())
    }
    pub fn close_stream_handle(&mut self, h: u32) -> Result<(), MixerError> {
        let sample = self.stream_sample_handle(h)?;
        self.release(sample)
    }
    pub fn service_stream_handle(&mut self, h: u32) -> Result<(), MixerError> {
        let sample = self.stream_sample_handle(h)?;
        if let Some(player) = &mut self.voice_mut(sample)?.asynchronous {
            player.service().map_err(MixerError::Async)?;
        }
        if let Some(player) = &mut self.voice_mut(sample)?.progressive {
            player.service().map_err(MixerError::Progressive)?;
        }
        Ok(())
    }
    /// Selecciona la política del puerto asíncrono; la emulación original es limitada.
    pub fn set_async_underrun_policy(
        &mut self,
        h: u32,
        policy: super::async_playback::UnderrunPolicy,
    ) -> Result<(), MixerError> {
        let sample = self.stream_sample_handle(h)?;
        let player = self
            .voice_mut(sample)?
            .asynchronous
            .as_mut()
            .ok_or(MixerError::UnsupportedStream(h))?;
        player.underrun_policy = policy;
        Ok(())
    }
    /// Indica que HSTREAM terminó la precarga para el siguiente callback.
    pub fn async_stream_service_ready(&self, h: u32) -> Result<bool, MixerError> {
        let sample = self.stream_sample_handle(h)?;
        self.voices
            .get(&sample)
            .ok_or(MixerError::UnknownSample(sample))?
            .asynchronous
            .as_ref()
            .map(|p| p.service_ready())
            .ok_or(MixerError::UnsupportedStream(h))
    }
    pub fn async_stream_underruns(&self, h: u32) -> Result<super::async_playback::UnderrunStats, MixerError> {
        let sample = self.stream_sample_handle(h)?;
        self.voices
            .get(&sample)
            .ok_or(MixerError::UnknownSample(sample))?
            .asynchronous
            .as_ref()
            .map(|p| p.stats)
            .ok_or(MixerError::UnsupportedStream(h))
    }
    /// Estéreo PCM16 a la frecuencia del dispositivo lógico. Solo esta llamada
    /// avanza cursores. No consume tiempo real ni solicita salida al sistema.
    pub fn render(&mut self, frames: usize) -> Result<RenderedPcm, MixerError> {
        if frames > self.config.max_render_frames {
            return Err(MixerError::RenderLimit);
        }
        let count = frames.checked_mul(2).ok_or(MixerError::RenderLimit)?;
        let mut build = Vec::new();
        build.try_reserve_exact(count).map_err(|_| MixerError::Capacity)?;
        build.resize(count, 0i32);
        let mut samples = Vec::new();
        samples.try_reserve_exact(count).map_err(|_| MixerError::Capacity)?;
        samples.resize(count, 0i16);
        let mut finished = Vec::new();
        finished
            .try_reserve_exact(self.voices.len())
            .map_err(|_| MixerError::Capacity)?;
        let mut finished_streams = Vec::new();
        finished_streams
            .try_reserve_exact(self.streams.len())
            .map_err(|_| MixerError::Capacity)?;
        if frames == 0 {
            return Ok(RenderedPcm {
                samples,
                finished,
                finished_streams,
            });
        }
        for (&h, v) in &mut self.voices {
            if v.state != SampleState::Playing {
                continue;
            }
            if let Some(player) = &mut v.asynchronous {
                let channels = player.channels as usize;
                let n = frames.checked_mul(channels).ok_or(MixerError::RenderLimit)?;
                let mut pcm = Vec::new();
                pcm.try_reserve_exact(n).map_err(|_| MixerError::Capacity)?;
                pcm.resize(n, 0i16);
                let result = player.read(&mut pcm).map_err(MixerError::Async)?;
                let mut gains = sample_gains(v.volume, v.pan, self.master);
                if mix_operation(player.channels, gains, self.config.mode) & 16 == 0 {
                    gains = [2048; 2];
                }
                for i in 0..result.frames {
                    let at = i * channels;
                    let l = pcm[at] as i32;
                    let r = if channels == 1 { l } else { pcm[at + 1] as i32 };
                    let input = [l * gains[0], r * gains[1]];
                    let filtered = if let Some(filter) = &mut v.lowpass {
                        filter.process_stereo(input)
                    } else {
                        input
                    };
                    build[2 * i] = build[2 * i].wrapping_add(filtered[0]);
                    build[2 * i + 1] = build[2 * i + 1].wrapping_add(filtered[1]);
                }
                v.cursor = v.cursor.checked_add(result.frames).ok_or(MixerError::RenderLimit)?;
                if result.status == super::async_playback::AsyncPlaybackStatus::End {
                    v.state = SampleState::Done;
                    finished.push(FinishedSample {
                        handle: h,
                        frame_offset: result.frames,
                    });
                }
                continue;
            }
            if let Some(player) = &mut v.progressive {
                let channels = player.channels as usize;
                let n = frames.checked_mul(channels).ok_or(MixerError::RenderLimit)?;
                let mut pcm = Vec::new();
                pcm.try_reserve_exact(n).map_err(|_| MixerError::Capacity)?;
                pcm.resize(n, 0i16);
                let result = player.read(&mut pcm).map_err(MixerError::Progressive)?;
                let mut gains = sample_gains(v.volume, v.pan, self.master);
                if mix_operation(player.channels, gains, self.config.mode) & 16 == 0 {
                    gains = [2048; 2];
                }
                for i in 0..result.frames {
                    let at = i * channels;
                    let l = pcm[at] as i32;
                    let r = if channels == 1 { l } else { pcm[at + 1] as i32 };
                    let input = [l * gains[0], r * gains[1]];
                    let filtered = if let Some(filter) = &mut v.lowpass {
                        filter.process_stereo(input)
                    } else {
                        input
                    };
                    build[2 * i] = build[2 * i].wrapping_add(filtered[0]);
                    build[2 * i + 1] = build[2 * i + 1].wrapping_add(filtered[1]);
                }
                v.cursor = v.cursor.checked_add(result.frames).ok_or(MixerError::RenderLimit)?;
                if result.status == super::progressive_playback::PlaybackStatus::End {
                    v.state = SampleState::Done;
                    finished.push(FinishedSample {
                        handle: h,
                        frame_offset: result.frames,
                    });
                }
                continue;
            }
            let p = v.pcm.as_ref().expect("Playing requiere datos adjuntos");
            let mut gains = sample_gains(v.volume, v.pan, self.master);
            if mix_operation(p.channels, gains, self.config.mode) & 16 == 0 {
                // El kernel sin escala desplaza la entrada 11 bits: unidad exacta.
                gains = [2048; 2];
            }
            let mut offset = 0;
            while offset < frames && v.state == SampleState::Playing {
                let n = (frames - offset).min(p.frames() - v.cursor);
                for i in 0..n {
                    let at = (v.cursor + i) * p.channels as usize;
                    let l = p.samples[at] as i32;
                    let r = if p.channels == 1 { l } else { p.samples[at + 1] as i32 };
                    let out = 2 * (offset + i);
                    let input = [l * gains[0], r * gains[1]];
                    let filtered = if let Some(filter) = &mut v.lowpass {
                        filter.process_stereo(input)
                    } else {
                        input
                    };
                    build[out] = build[out].wrapping_add(filtered[0]);
                    build[out + 1] = build[out + 1].wrapping_add(filtered[1]);
                }
                v.cursor += n;
                offset += n;
                if v.cursor == p.frames() {
                    if v.loops_left != 1 {
                        if v.loops_left > 1 {
                            v.loops_left -= 1;
                        }
                        v.cursor = 0;
                        continue;
                    }
                    v.state = SampleState::Done;
                    finished.push(FinishedSample {
                        handle: h,
                        frame_offset: offset,
                    });
                }
            }
        }
        for (out, wide) in samples.iter_mut().zip(build) {
            *out = (wide >> 11).clamp(i16::MIN as i32, i16::MAX as i32) as i16;
        }
        finished.sort_by_key(|f| (f.frame_offset, f.handle));
        for f in &finished {
            if let Some((&h, _)) = self.streams.iter().find(|(_, s)| s.sample == f.handle) {
                finished_streams.push(FinishedStream {
                    handle: h,
                    frame_offset: f.frame_offset,
                });
            }
        }
        Ok(RenderedPcm {
            samples,
            finished,
            finished_streams,
        })
    }
}
impl Backend for OfflineDevice {
    fn sample_status(&mut self, h: u32) -> u32 {
        if h == 0 {
            return 0;
        }
        match self.voices.get(&h) {
            Some(v) => v.state as u32,
            None => {
                self.errors.push(MixerError::UnknownSample(h));
                1
            }
        }
    }
    fn sample_pan(&mut self, h: u32) -> i32 {
        if h == 0 {
            return 0;
        }
        let r = self.snapshot(h).map(|v| v.pan);
        self.result(r)
    }
    fn set_sample_pan(&mut self, h: u32, pan: u16) {
        if h != 0 {
            let r = self.set_pan(h, pan as i32);
            self.result(r);
        }
    }
    fn end_sample(&mut self, h: u32) {
        if h != 0 {
            let r = self.end(h);
            self.result(r);
        }
    }
    fn stop_sample(&mut self, h: u32) {
        if h != 0 {
            let r = self.stop(h);
            self.result(r);
        }
    }
    fn stream_status(&mut self, h: u32) -> u32 {
        if h == 0 {
            return u32::MAX;
        }
        let r = self.stream_snapshot(h).map(|s| s.state as u32);
        match r {
            Ok(v) => v,
            Err(e) => {
                self.errors.push(e);
                u32::MAX
            }
        }
    }
    fn stream_volume(&mut self, h: u32) -> i32 {
        if h == 0 {
            return -1;
        }
        let r = self.stream_sample_handle(h).and_then(|s| self.volume(s));
        self.result(r)
    }
    fn set_stream_volume(&mut self, h: u32, volume: i32) {
        if h != 0 {
            let r = self.stream_sample_handle(h).and_then(|s| self.set_volume(s, volume));
            self.result(r);
        }
    }
    fn pause_stream(&mut self, h: u32, pause: u32) {
        if h != 0 {
            let r = self.pause_stream_handle(h, pause != 0);
            self.result(r);
        }
    }
    fn close_stream(&mut self, h: u32) {
        if h != 0 {
            let r = self.close_stream_handle(h);
            self.result(r);
        }
    }
    fn clear_sound_slot_tables(&mut self) {
        self.slot_clear_requests += 1;
    }
}
impl EffectDevice for OfflineDevice {
    fn allocate_sample(&mut self, d: u32) -> u32 {
        let r = self.allocate(d);
        self.result(r)
    }
    fn init_sample(&mut self, h: u32) {
        if h != 0 {
            let r = self.initialize(h);
            self.result(r);
        }
    }
    fn release_sample(&mut self, h: u32) {
        if h != 0 {
            let r = self.release(h);
            self.result(r);
        }
    }
    fn set_sample_volume(&mut self, h: u32, volume: i32) {
        if h != 0 {
            let r = self.set_volume(h, volume);
            self.result(r);
        }
    }
    fn attach_wave(&mut self, h: u32, wave: WaveLease, block: u32) -> i32 {
        let r = self.attach(h, wave, block).map(|_| 1);
        self.result(r)
    }
    fn start_sample(&mut self, h: u32) {
        if h != 0 {
            let r = self.start(h);
            self.result(r);
        }
    }
    fn delete_effect_list_object(&mut self) {
        self.delete_requests += 1;
    }
}
