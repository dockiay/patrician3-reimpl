//! IMA WAV 0x11, cuatro bits, mono/estéreo, bloques completos → PCM16 intercalado.
//! Los dos redondeos observados en referencias independientes son opciones
//! explícitas. decode_miles_defined reproduce el PCM definido contrastado con
//! mss32.dll en x86 aislado, sin copiar finales de reserva sin escribir.
use super::wave::{WaveError, WaveInfo};

const STEPS: [i32; 89] = [
    7, 8, 9, 10, 11, 12, 13, 14, 16, 17, 19, 21, 23, 25, 28, 31, 34, 37, 41, 45, 50, 55, 60, 66, 73, 80, 88, 97, 107,
    118, 130, 143, 157, 173, 190, 209, 230, 253, 279, 307, 337, 371, 408, 449, 494, 544, 598, 658, 724, 796, 876, 963,
    1060, 1166, 1282, 1411, 1552, 1707, 1878, 2066, 2272, 2499, 2749, 3024, 3327, 3660, 4026, 4428, 4871, 5358, 5894,
    6484, 7132, 7845, 8630, 9493, 10442, 11487, 12635, 13899, 15289, 16818, 18500, 20350, 22385, 24623, 27086, 29794,
    32767,
];
const INDEX: [i32; 8] = [-1, -1, -1, -1, 2, 4, 6, 8];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImaRounding {
    Product,
    ShiftSum,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FactPolicy {
    Available,
    Clamp,
    Strict,
}
#[derive(Debug, Clone, Copy)]
pub struct DecodeOptions {
    pub rounding: ImaRounding,
    pub fact: FactPolicy,
    pub max_frames: usize,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeError {
    Wave(WaveError),
    UnsupportedFormat,
    BlockAlign,
    SamplesPerBlock,
    IncompleteBlock,
    Header { block: usize, channel: usize },
    FactBeyondAvailable { declared: u32, available: usize },
    FrameLimit,
    Capacity,
    NoFrames,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pcm16 {
    pub channels: u16,
    pub sample_rate: u32,
    pub samples: Vec<i16>,
    pub available_frames: usize,
    pub declared_frames: Option<u32>,
}
impl Pcm16 {
    pub fn frames(&self) -> usize {
        self.samples.len() / self.channels as usize
    }
}

/// Perfil de la DLL de referencia: ShiftSum y fact. Conserva declared_frames,
/// pero no inventa bytes cuando fact supera los bloques disponibles. No reproduce
/// el ABI AILSOUNDINFO ni la cabecera WAV de AIL_decompress_ADPCM.
pub fn decode_miles_defined(wav: &[u8], max_frames: usize) -> Result<Pcm16, DecodeError> {
    let pcm = decode(
        wav,
        DecodeOptions {
            rounding: ImaRounding::ShiftSum,
            fact: FactPolicy::Clamp,
            max_frames,
        },
    )?;
    if pcm.frames() == 0 {
        return Err(DecodeError::NoFrames);
    }
    Ok(pcm)
}

fn expand(predictor: &mut i32, index: &mut usize, code: u8, rounding: ImaRounding) -> i16 {
    let step = STEPS[*index];
    let magnitude = (code & 7) as i32;
    let delta = match rounding {
        ImaRounding::Product => ((magnitude * 2 + 1) * step) >> 3,
        ImaRounding::ShiftSum => {
            (step >> 3)
                + if code & 4 != 0 { step } else { 0 }
                + if code & 2 != 0 { step >> 1 } else { 0 }
                + if code & 1 != 0 { step >> 2 } else { 0 }
        }
    };
    let next = *predictor + if code & 8 != 0 { -delta } else { delta };
    *predictor = next.clamp(i16::MIN as i32, i16::MAX as i32);
    *index = (*index as i32 + INDEX[magnitude as usize]).clamp(0, 88) as usize;
    *predictor as i16
}

pub fn decode(wav: &[u8], options: DecodeOptions) -> Result<Pcm16, DecodeError> {
    let info = WaveInfo::parse(wav).map_err(DecodeError::Wave)?;
    if info.format_tag != 0x11 || info.bits_per_sample != 4 || !matches!(info.channels, 1 | 2) {
        return Err(DecodeError::UnsupportedFormat);
    }
    let channels = info.channels as usize;
    let align = info.block_align as usize;
    let header = 4 * channels;
    let group = 4 * channels;
    if align < header || !(align - header).is_multiple_of(group) {
        return Err(DecodeError::BlockAlign);
    }
    let per_block = 1 + (align - header) * 2 / channels;
    if info.extra.len() < 2
        || u16::from_le_bytes(wav[info.extra.start..info.extra.start + 2].try_into().unwrap()) as usize != per_block
    {
        return Err(DecodeError::SamplesPerBlock);
    }
    let data = &wav[info.data];
    if !data.len().is_multiple_of(align) {
        return Err(DecodeError::IncompleteBlock);
    }
    let available = (data.len() / align)
        .checked_mul(per_block)
        .ok_or(DecodeError::FrameLimit)?;
    let frames = match (info.fact_frames, options.fact) {
        (Some(n), FactPolicy::Strict) if n as usize > available => {
            return Err(DecodeError::FactBeyondAvailable { declared: n, available })
        }
        (Some(n), FactPolicy::Strict | FactPolicy::Clamp) => (n as usize).min(available),
        _ => available,
    };
    if frames > options.max_frames {
        return Err(DecodeError::FrameLimit);
    }
    let count = frames.checked_mul(channels).ok_or(DecodeError::FrameLimit)?;
    // Validar también las cabeceras fuera de la longitud recortada por fact.
    for (n, block) in data.chunks_exact(align).enumerate() {
        for ch in 0..channels {
            if block[4 * ch + 2] > 88 || block[4 * ch + 3] != 0 {
                return Err(DecodeError::Header { block: n, channel: ch });
            }
        }
    }
    let mut samples = Vec::new();
    samples.try_reserve_exact(count).map_err(|_| DecodeError::Capacity)?;
    samples.resize(count, 0);
    let mut base = 0;
    for block in data.chunks_exact(align) {
        if base >= frames {
            break;
        }
        let mut predictors = [0; 2];
        let mut indices = [0; 2];
        for ch in 0..channels {
            predictors[ch] = i16::from_le_bytes(block[4 * ch..4 * ch + 2].try_into().unwrap()) as i32;
            indices[ch] = block[4 * ch + 2] as usize;
            samples[base * channels + ch] = predictors[ch] as i16;
        }
        for (n, packet) in block[header..].chunks_exact(group).enumerate() {
            for ch in 0..channels {
                for byte in 0..4 {
                    let value = packet[ch * 4 + byte];
                    for half in 0..2 {
                        let frame = base + 1 + 8 * n + 2 * byte + half;
                        if frame >= frames {
                            break;
                        }
                        let code = (value >> (4 * half)) & 15;
                        samples[frame * channels + ch] =
                            expand(&mut predictors[ch], &mut indices[ch], code, options.rounding);
                    }
                }
            }
        }
        base += per_block;
    }
    Ok(Pcm16 {
        channels: info.channels,
        sample_rate: info.sample_rate,
        samples,
        available_frames: available,
        declared_frames: info.fact_frames,
    })
}
