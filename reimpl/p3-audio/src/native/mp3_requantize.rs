//! Tabla de magnitudes y producto del perfil Miles definido. No es un decoder MP3.
//! El original solo reserva8192 entradas; nunca se copia su lectura fuera de tabla.
use std::sync::LazyLock;
pub const MILES_MAGNITUDES: usize = 8192;
static MAGNITUDES: LazyLock<Vec<f32>> = LazyLock::new(|| {
    (0..MILES_MAGNITUDES)
        .map(|q| (q as f64).powf(4.0_f64 / 3.0) as f32)
        .collect()
});

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequantizeError {
    OutsideOriginalTable(u16),
    NonFiniteGain(u32),
}
pub fn magnitude_bits(index: u16) -> Result<u32, RequantizeError> {
    MAGNITUDES
        .get(index as usize)
        .map(|v| v.to_bits())
        .ok_or(RequantizeError::OutsideOriginalTable(index))
}
pub fn apply_magnitude(index: u16, gain_bits: u32, negative: bool) -> Result<u32, RequantizeError> {
    let magnitude = f32::from_bits(magnitude_bits(index)?);
    let gain = f32::from_bits(gain_bits);
    if !gain.is_finite() {
        return Err(RequantizeError::NonFiniteGain(gain_bits));
    }
    let value = (magnitude as f64) * (gain as f64);
    Ok((if negative { -value } else { value } as f32).to_bits())
}

/// `global_gain` ya extraído de la información lateral. No analiza sus bits.
pub fn base_gain_bits(global_gain: u8) -> u64 {
    2.0_f64.powf((global_gain as f64 - 210.0) * 0.25).to_bits()
}
pub fn scale_factor_bits(index: u8) -> u32 {
    (2.0_f64.powf(-(index as f64) * 0.5) as f32).to_bits()
}
pub fn subblock_factor_bits(index: u8) -> u32 {
    (2.0_f64.powf(-(index as f64) * 2.0) as f32).to_bits()
}
/// Almacena la escala de banda como f32 antes de aplicar magnitudes.
/// `scale_index` es el índice ya formado por el lector de escalas del original.
pub fn band_gain_bits(global_gain: u8, scale_index: u8) -> u32 {
    let base = f64::from_bits(base_gain_bits(global_gain));
    let scale = f32::from_bits(scale_factor_bits(scale_index)) as f64;
    ((base * scale) as f32).to_bits()
}
