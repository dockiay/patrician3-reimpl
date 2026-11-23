//! Etapa espectral estéreo de Mp3dec antes de la síntesis híbrida.
use super::mp3_band::{self, BandParams, ReorderError};
// El original almacena 0.707106781, no el valor exacto de 1/sqrt(2).
const MILES_MS_GAIN: f64 = f64::from_bits(0x3fe6a09e6665983e);
#[derive(Debug, Clone, Copy)]
pub struct StereoParams {
    pub channels: [BandParams; 2],
    pub joint: bool,
    pub mid_side: bool,
    pub intensity: bool,
    /// Campo que consulta el original para elegir el exponente LSF.
    pub scale_compress: u16,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StereoError {
    InvalidParameters,
    NonFiniteInput,
    InvalidIntensityPosition,
    Reorder(ReorderError),
}

/// Reproduce la selección por banda de 0x26F04270 antes de aplicar mezclas.
/// 7 indica que la posición conserva los dos canales o usa mid/side.
pub fn intensity_positions(p: StereoParams, right: &[u32; 576], scales: &[u32; 62]) -> Result<[u8; 576], StereoError> {
    let b = p.channels[0];
    if b.version > 1
        || b.sample_rate_index > 2
        || p.channels[1].version != b.version
        || p.channels[1].sample_rate_index != b.sample_rate_index
        || p.scale_compress > 511
    {
        return Err(StereoError::InvalidParameters);
    }
    if right.iter().any(|&x| !f32::from_bits(x).is_finite()) {
        return Err(StereoError::NonFiniteInput);
    }
    let mut mask = [7u8; 576];
    if !p.joint || !p.intensity {
        return Ok(mask);
    }
    let t = &mp3_band::BANDS[usize::from(b.version) * 3 + usize::from(b.sample_rate_index)];
    let short = b.window_switch && b.block_type == 2;
    let position = |i: usize| -> Result<u8, StereoError> {
        let v = scales[i];
        let max = if b.version == 1 { 7 } else { 31 };
        if v > max {
            return Err(StereoError::InvalidIntensityPosition);
        }
        Ok(v as u8)
    };
    if !short {
        let last = (0..576).rev().find(|&i| f32::from_bits(right[i]) != 0.0).unwrap_or(0);
        let start = (0..23)
            .find(|&i| usize::from(t[i]) > last)
            .ok_or(StereoError::InvalidParameters)?;
        for band in start..21 {
            let pos = position(band)?;
            for slot in &mut mask[usize::from(t[band])..usize::from(t[band + 1])] {
                *slot = pos;
            }
        }
        if start <= 21 {
            let value = mask[usize::from(t[20])];
            for slot in &mut mask[usize::from(t[21])..] {
                *slot = value;
            }
        }
    } else {
        let mut highest = 0usize;
        for window in 0..3 {
            let first = if b.mixed { 3 } else { 0 };
            let mut found = if b.mixed { 2isize } else { -1isize };
            for band in (first..13).rev() {
                let width = usize::from(t[24 + band] - t[23 + band]);
                let start = usize::from(t[23 + band]) * 3 + window * width;
                if right[start..start + width].iter().any(|&v| f32::from_bits(v) != 0.0) {
                    found = band as isize;
                    break;
                }
            }
            let begin = (found + 1) as usize;
            highest = highest.max(begin);
            for band in begin..12 {
                let width = usize::from(t[24 + band] - t[23 + band]);
                let start = usize::from(t[23 + band]) * 3 + window * width;
                let pos = position(23 + window * 13 + band)?;
                for slot in &mut mask[start..start + width] {
                    *slot = pos;
                }
            }
            // El original copia el último factor a las posiciones calculadas
            // desde la banda 11, no desde la banda 12.
            let before = usize::from(t[35] - t[34]);
            let after = usize::from(t[36] - t[35]);
            let base = usize::from(t[34]) * 3;
            let source = base + window * before;
            let dest = base + window * after;
            let value = mask[source];
            for slot in &mut mask[dest..dest + after] {
                *slot = value;
            }
        }
        if b.mixed && highest <= 3 {
            let last = (0..54).rev().find(|&i| f32::from_bits(right[i]) != 0.0);
            let start = match last {
                Some(last) => (0..23)
                    .find(|&i| usize::from(t[i]) > last)
                    .ok_or(StereoError::InvalidParameters)?,
                None => 0,
            };
            for band in start..8 {
                let pos = position(band)?;
                for slot in &mut mask[usize::from(t[band])..usize::from(t[band + 1])] {
                    *slot = pos;
                }
            }
        }
    }
    Ok(mask)
}
/// Los espectros entran en el orden Huffman; se mezclan y reordenan en la rama
/// de dos canales. Entradas no finitas quedan fuera del perfil contrastado.
pub fn transform(
    p: StereoParams,
    left: &[u32; 576],
    right: &[u32; 576],
    scales_right: &[u32; 62],
) -> Result<[[u32; 576]; 2], StereoError> {
    if left.iter().any(|&x| !f32::from_bits(x).is_finite()) {
        return Err(StereoError::NonFiniteInput);
    }
    let mask = intensity_positions(p, right, scales_right)?;
    let version = p.channels[0].version;
    let mut raw = [[0u32; 576]; 2];
    for i in 0..576 {
        let l = f32::from_bits(left[i]);
        let r = f32::from_bits(right[i]);
        let pos = mask[i];
        if p.joint && p.intensity && pos != 7 {
            let (fl, fr) = if version == 1 {
                let t = ((f64::from(pos) * 0.2617993877991492_f64).tan() as f32) as f64;
                (t / (t + 1.0), 1.0 / (t + 1.0))
            } else {
                let base = if p.scale_compress & 1 != 0 {
                    f32::from_bits(0x3f3504f3)
                } else {
                    f32::from_bits(0x3f5744fd)
                };
                if pos == 0 {
                    (1.0, 1.0)
                } else if pos & 1 != 0 {
                    ((f64::from(base)).powi(i32::from(pos + 1) / 2) as f32 as f64, 1.0)
                } else {
                    (1.0, (f64::from(base)).powi(i32::from(pos) / 2) as f32 as f64)
                }
            };
            raw[0][i] = ((f64::from(l) * fl) as f32).to_bits();
            raw[1][i] = ((f64::from(l) * fr) as f32).to_bits();
        } else if p.joint && p.mid_side {
            raw[0][i] = (((f64::from(l) + f64::from(r)) * MILES_MS_GAIN) as f32).to_bits();
            raw[1][i] = (((f64::from(l) - f64::from(r)) * MILES_MS_GAIN) as f32).to_bits();
        } else {
            raw[0][i] = left[i];
            raw[1][i] = right[i];
        }
    }
    let mut result = [[0u32; 576]; 2];
    for ch in 0..2 {
        mp3_band::reorder_stereo_channel(p.channels[ch], &raw[ch], &mut result[ch]).map_err(StereoError::Reorder)?;
    }
    Ok(result)
}
