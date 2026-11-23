//! Exploración RIFF/WAVE acotada. Identifica datos y formato, no decodifica audio.
use std::ops::Range;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaveError {
    Header,
    Bounds,
    DuplicateChunk,
    MissingFormat,
    MissingData,
    Format,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaveInfo {
    pub format_tag: u16,
    pub channels: u16,
    pub sample_rate: u32,
    pub average_bytes_per_second: u32,
    pub block_align: u16,
    pub bits_per_sample: u16,
    pub extra: Range<usize>,
    pub data: Range<usize>,
    pub fact_frames: Option<u32>,
}
impl WaveInfo {
    pub fn parse(bytes: &[u8]) -> Result<Self, WaveError> {
        if bytes.len() < 12 || &bytes[..4] != b"RIFF" || &bytes[8..12] != b"WAVE" {
            return Err(WaveError::Header);
        }
        let u32_at = |i| u32::from_le_bytes(bytes[i..i + 4].try_into().unwrap());
        let end = (u32_at(4) as u64) + 8;
        if end != bytes.len() as u64 {
            return Err(WaveError::Bounds);
        }
        let mut at = 12;
        let mut format = None;
        let mut data = None;
        let mut fact = None;
        while at < bytes.len() {
            if bytes.len() - at < 8 {
                return Err(WaveError::Bounds);
            }
            let size = u32_at(at + 4) as usize;
            let start = at + 8;
            let finish = start.checked_add(size).ok_or(WaveError::Bounds)?;
            let padded = finish.checked_add(size & 1).ok_or(WaveError::Bounds)?;
            if padded > bytes.len() {
                return Err(WaveError::Bounds);
            }
            match &bytes[at..at + 4] {
                b"fmt " => {
                    if format.replace(start..finish).is_some() {
                        return Err(WaveError::DuplicateChunk);
                    }
                }
                b"data" => {
                    if data.replace(start..finish).is_some() {
                        return Err(WaveError::DuplicateChunk);
                    }
                }
                b"fact" => {
                    if size < 4 {
                        return Err(WaveError::Format);
                    }
                    if fact.replace(u32_at(start)).is_some() {
                        return Err(WaveError::DuplicateChunk);
                    }
                }
                _ => {}
            }
            at = padded;
        }
        let fmt = format.ok_or(WaveError::MissingFormat)?;
        let data = data.ok_or(WaveError::MissingData)?;
        if fmt.len() < 16 {
            return Err(WaveError::Format);
        }
        let u16_at = |i| u16::from_le_bytes(bytes[i..i + 2].try_into().unwrap());
        let p = fmt.start;
        let tag = u16_at(p);
        let extra = if fmt.len() == 16 && tag == 1 {
            p + 16..p + 16
        } else {
            if fmt.len() < 18 {
                return Err(WaveError::Format);
            }
            let n = u16_at(p + 16) as usize;
            if n > fmt.len() - 18 {
                return Err(WaveError::Format);
            }
            p + 18..p + 18 + n
        };
        let result = Self {
            format_tag: tag,
            channels: u16_at(p + 2),
            sample_rate: u32_at(p + 4),
            average_bytes_per_second: u32_at(p + 8),
            block_align: u16_at(p + 12),
            bits_per_sample: u16_at(p + 14),
            extra,
            data,
            fact_frames: fact,
        };
        if result.channels == 0 || result.sample_rate == 0 || result.block_align == 0 {
            return Err(WaveError::Format);
        }
        Ok(result)
    }
}
