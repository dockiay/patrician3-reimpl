//! Aritmética HSTREAM aislada. No estima bytes consumidos desde PCM de un codec.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MilesTiming {
    pub format_flags: u32,
    pub sample_rate: u32,
    pub codec_byte_rate: Option<u32>,
    pub block_size: u32,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimingError {
    Format,
    Block,
    Range,
}
impl MilesTiming {
    /// Subdominio seguro: enteros positivos con signo, sin wrap del SDK.
    pub fn byte_rate(self) -> Result<u32, TimingError> {
        if self.format_flags > 7 {
            return Err(TimingError::Format);
        }
        if self.block_size > i32::MAX as u32
            || ((self.format_flags & 4 != 0 || self.codec_byte_rate.is_none()) && self.sample_rate > i32::MAX as u32)
        {
            return Err(TimingError::Range);
        }
        let value = if self.format_flags & 4 != 0 {
            let channels = if self.format_flags & 2 != 0 { 2 } else { 1 };
            let header = 4 * channels;
            if self.block_size < header {
                return Err(TimingError::Block);
            }
            let frames = ((self.block_size - header) as u64) * 2 / (channels as u64) + 1;
            (self.sample_rate as u64) * (self.block_size as u64) / frames
        } else if let Some(rate) = self.codec_byte_rate {
            rate as u64
        } else {
            let stride = (1 + (self.format_flags & 1)) * (1 + ((self.format_flags >> 1) & 1));
            (self.sample_rate as u64) * (stride as u64)
        };
        if value > i32::MAX as u64 {
            Err(TimingError::Range)
        } else {
            Ok(value as u32)
        }
    }
    pub fn milliseconds(self, bytes: u32) -> Result<u32, TimingError> {
        if bytes > i32::MAX as u32 {
            return Err(TimingError::Range);
        }
        let rate = self.byte_rate()?;
        if rate == 0 {
            return Ok(1);
        }
        let ms = ((bytes as f64) * 1000.0 / (rate as f64)).round_ties_even();
        if ms > u32::MAX as f64 {
            Err(TimingError::Range)
        } else {
            Ok(ms as u32)
        }
    }
    pub fn seek_bytes(self, ms: u32) -> Result<u32, TimingError> {
        if ms > i32::MAX as u32 {
            return Err(TimingError::Range);
        }
        let bytes = ((self.byte_rate()? as f64) * (ms as f64) * 0.001).round_ties_even();
        if bytes > i32::MAX as f64 {
            Err(TimingError::Range)
        } else {
            Ok(bytes as u32)
        }
    }
    pub fn align(self, bytes: u32, total: u32) -> Result<u32, TimingError> {
        self.align_with_provider(bytes, total, false)
    }
    /// Con proveedor abierto Miles alinea por el formato PCM, incluso con códec.
    /// `provider_open` representa el puntero HSTREAM+4 observado en x86.
    pub fn align_with_provider(self, bytes: u32, total: u32, provider_open: bool) -> Result<u32, TimingError> {
        self.byte_rate()?;
        if bytes > i32::MAX as u32 || total > i32::MAX as u32 {
            return Err(TimingError::Range);
        }
        let at = if self.codec_byte_rate.is_some() && !provider_open {
            if self.block_size == 0 {
                return Err(TimingError::Block);
            }
            let q = bytes / self.block_size;
            let next = u64::from(bytes % self.block_size > self.block_size / 2);
            (q as u64 + next) * (self.block_size as u64)
        } else {
            let stride = (1 + (self.format_flags & 1)) * (1 + ((self.format_flags >> 1) & 1));
            (bytes & !(stride - 1)) as u64
        };
        if at > i32::MAX as u64 {
            return Err(TimingError::Range);
        }
        Ok(at.min(total as u64) as u32)
    }
}

/// Reloj del PCM16 predecodificado; no es la posición de entrada de MP3/IMA.
pub(crate) fn pcm_milliseconds(frames: usize, rate: u32) -> u32 {
    ((frames as f64) * 1000.0 / (rate as f64))
        .round_ties_even()
        .min(u32::MAX as f64) as u32
}
pub(crate) fn pcm_seek_frame(ms: u32, rate: u32, channels: u16, frames: usize) -> usize {
    let stride = channels as usize * 2;
    let bytes = ((rate as f64) * (stride as f64) * (ms as f64) * 0.001).round_ties_even();
    bytes.min((frames as f64) * (stride as f64)) as usize / stride
}
