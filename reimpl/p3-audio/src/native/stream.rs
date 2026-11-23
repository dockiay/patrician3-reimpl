//! Streams lógicos con PCM predecodificado y límites explícitos; sin salida/hilo.
use super::ima::{decode_miles_defined, Pcm16};
use super::wave::WaveInfo;
use std::io::Cursor;
use std::sync::Arc;
use symphonia::core::codecs::audio::{well_known::CODEC_ID_MP3, AudioDecoderOptions};
use symphonia::core::formats::{probe::Hint, FormatOptions, TrackType};
use symphonia::core::io::MediaSourceStream;

/// Selección explícita del decoder para streams MP3.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Mp3Backend {
    /// Ruta predecodificada existente.
    #[default]
    Predecoded,
    /// Ruta progresiva Rust con fuente en memoria y sample de dos búferes.
    NativeProgressive,
    /// E/S en hilo productor y consumo no bloqueante; fuente de recursos acotada.
    AsyncNative,
    /// Productor asíncrono con programación y reinicio del servicio HSTREAM.
    AsyncNativeHstream,
}

#[derive(Debug, Clone, Copy)]
pub struct StreamLimits {
    pub max_encoded_bytes: usize,
    pub max_frames: usize,
    pub max_streams: usize,
}
impl Default for StreamLimits {
    fn default() -> Self {
        Self {
            max_encoded_bytes: 16 * 1024 * 1024,
            max_frames: 44100 * 600,
            max_streams: 8,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StreamError {
    Limit,
    Format,
    Decode,
    Capacity,
    Empty,
}

pub fn decode_stream(bytes: Arc<[u8]>, name: &str, limits: StreamLimits) -> Result<Pcm16, StreamError> {
    if bytes.len() > limits.max_encoded_bytes {
        return Err(StreamError::Limit);
    }
    if bytes.starts_with(b"RIFF") {
        let w = WaveInfo::parse(&bytes).map_err(|_| StreamError::Format)?;
        if w.format_tag == 0x11 {
            return decode_miles_defined(&bytes, limits.max_frames).map_err(|_| StreamError::Decode);
        }
        if w.format_tag != 1
            || w.sample_rate == 0
            || !matches!(w.channels, 1 | 2)
            || !matches!(w.bits_per_sample, 8 | 16)
        {
            return Err(StreamError::Format);
        }
        let width = w.bits_per_sample as usize / 8;
        if w.block_align as usize != width * w.channels as usize
            || !w.data.len().is_multiple_of(width * w.channels as usize)
        {
            return Err(StreamError::Format);
        }
        let n = w.data.len() / width;
        if n / w.channels as usize > limits.max_frames {
            return Err(StreamError::Limit);
        }
        let mut samples = Vec::new();
        samples.try_reserve_exact(n).map_err(|_| StreamError::Capacity)?;
        let raw = &bytes[w.data];
        if width == 1 {
            samples.extend(raw.iter().map(|v| ((*v as i16) - 128) << 8));
        } else {
            samples.extend(raw.as_chunks::<2>().0.iter().map(|v| i16::from_le_bytes(*v)));
        }
        let frames = samples.len() / w.channels as usize;
        if frames == 0 {
            return Err(StreamError::Empty);
        }
        return Ok(Pcm16 {
            channels: w.channels,
            sample_rate: w.sample_rate,
            samples,
            available_frames: frames,
            declared_frames: None,
        });
    }
    if !name.to_ascii_lowercase().ends_with(".mp3") {
        return Err(StreamError::Format);
    }
    let mss = MediaSourceStream::new(Box::new(Cursor::new(bytes)), Default::default());
    let mut hint = Hint::new();
    hint.with_extension("mp3");
    let mut format = symphonia::default::get_probe()
        .probe(&hint, mss, FormatOptions::default(), Default::default())
        .map_err(|_| StreamError::Format)?;
    let track = format.default_track(TrackType::Audio).ok_or(StreamError::Format)?;
    let params = track
        .codec_params
        .as_ref()
        .and_then(|p| p.audio())
        .ok_or(StreamError::Format)?;
    if params.codec != CODEC_ID_MP3 {
        return Err(StreamError::Format);
    }
    let rate = params.sample_rate.ok_or(StreamError::Format)?;
    let channels = params.channels.as_ref().ok_or(StreamError::Format)?.count();
    if rate == 0 || !matches!(channels, 1 | 2) {
        return Err(StreamError::Format);
    }
    let mut decoder = symphonia::default::get_codecs()
        .make_audio_decoder(params, &AudioDecoderOptions::default())
        .map_err(|_| StreamError::Decode)?;
    let id = track.id;
    let max_samples = limits.max_frames.checked_mul(channels).ok_or(StreamError::Limit)?;
    let mut samples: Vec<i16> = Vec::new();
    while let Some(packet) = format.next_packet().map_err(|_| StreamError::Decode)? {
        if packet.track_id != id {
            continue;
        }
        let audio = decoder.decode(&packet).map_err(|_| StreamError::Decode)?;
        if audio.spec().rate() != rate || audio.spec().channels().count() != channels {
            return Err(StreamError::Format);
        }
        let n = audio.samples_interleaved();
        let length = samples
            .len()
            .checked_add(n)
            .filter(|v| *v <= max_samples)
            .ok_or(StreamError::Limit)?;
        if samples.capacity() < length {
            let target = length.max(samples.capacity().saturating_mul(2)).min(max_samples);
            samples
                .try_reserve_exact(target - samples.len())
                .map_err(|_| StreamError::Capacity)?;
        }
        let at = samples.len();
        samples.resize(length, 0);
        audio.copy_to_slice_interleaved(&mut samples[at..]);
    }
    let frames = samples.len() / channels;
    if frames == 0 {
        return Err(StreamError::Empty);
    }
    Ok(Pcm16 {
        channels: channels as u16,
        sample_rate: rate,
        samples,
        available_frames: frames,
        declared_frames: None,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamSnapshot {
    pub sample: u32,
    pub state: super::mixer::SampleState,
    pub cursor: usize,
    pub frames: usize,
    pub loop_count: u32,
    pub loops_left: u32,
    pub started: bool,
    pub total_ms: u32,
    pub current_ms: u32,
}
pub(super) struct StreamVoice {
    pub sample: u32,
    pub started: bool,
}
