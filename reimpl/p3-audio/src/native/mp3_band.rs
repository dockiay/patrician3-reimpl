//! Ganancias de banda y espectro antes de estéreo de Mp3dec escalar.
use super::mp3_requantize::{self, RequantizeError};

pub(super) const BANDS: [[u16; 37]; 6] = [
    [
        0, 6, 12, 18, 24, 30, 36, 44, 54, 66, 80, 96, 116, 140, 168, 200, 238, 284, 336, 396, 464, 522, 576, 0, 4, 8,
        12, 18, 24, 32, 42, 56, 74, 100, 132, 174, 192,
    ],
    [
        0, 6, 12, 18, 24, 30, 36, 44, 54, 66, 80, 96, 114, 136, 162, 194, 232, 278, 330, 394, 464, 540, 576, 0, 4, 8,
        12, 18, 26, 36, 48, 62, 80, 104, 136, 180, 192,
    ],
    [
        0, 6, 12, 18, 24, 30, 36, 44, 54, 66, 80, 96, 116, 140, 168, 200, 238, 284, 336, 396, 464, 522, 576, 0, 4, 8,
        12, 18, 26, 36, 48, 62, 80, 104, 134, 174, 192,
    ],
    [
        0, 4, 8, 12, 16, 20, 24, 30, 36, 44, 52, 62, 74, 90, 110, 134, 162, 196, 238, 288, 342, 418, 576, 0, 4, 8, 12,
        16, 22, 30, 40, 52, 66, 84, 106, 136, 192,
    ],
    [
        0, 4, 8, 12, 16, 20, 24, 30, 36, 42, 50, 60, 72, 88, 106, 128, 156, 190, 230, 276, 330, 384, 576, 0, 4, 8, 12,
        16, 22, 28, 38, 50, 64, 80, 100, 126, 192,
    ],
    [
        0, 4, 8, 12, 16, 20, 24, 30, 36, 44, 54, 66, 82, 102, 126, 156, 194, 240, 296, 364, 448, 550, 576, 0, 4, 8, 12,
        16, 22, 30, 42, 58, 78, 104, 138, 180, 192,
    ],
];
const PRETAB: [u32; 22] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 3, 3, 3, 2, 0];
#[derive(Debug, Clone, Copy)]
pub struct BandParams {
    pub version: u8,
    pub sample_rate_index: u8,
    pub global_gain: u8,
    pub preflag: bool,
    pub scale_mode: bool,
    pub subblock_gain: [u8; 3],
    pub window_switch: bool,
    pub block_type: u8,
    pub mixed: bool,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BandError {
    InvalidParameters,
    UnsupportedMixedMpeg2,
    ScaleOutOfRange,
    Requantize(RequantizeError),
}

/// Los 576 valores conservan el orden que el original usa antes de estéreo.
/// El reordenado de ventanas cortas es una etapa posterior distinta.
pub fn spectrum(
    p: BandParams,
    scales: &[u32; 62],
    magnitude: &[u16; 576],
    sign: &[u8; 576],
    used: usize,
) -> Result<[u32; 576], BandError> {
    if p.version > 1
        || p.sample_rate_index > 2
        || p.block_type > 3
        || used > 576
        || p.subblock_gain.iter().any(|&x| x > 7)
    {
        return Err(BandError::InvalidParameters);
    }
    let short = p.window_switch && p.block_type == 2;
    if short && p.mixed && p.version == 0 {
        return Err(BandError::UnsupportedMixedMpeg2);
    }
    let table = &BANDS[usize::from(p.version) * 3 + usize::from(p.sample_rate_index)];
    let multiplier = 1 + u32::from(p.scale_mode);
    // El original prepara las 22 ganancias largas y las 39 cortas incluso
    // aunque used sea cero. No leer fuera de la tabla por campos no usados.
    for (i, &scale) in scales.iter().take(22).enumerate() {
        let index = scale
            .checked_add(if p.preflag { PRETAB[i] } else { 0 })
            .and_then(|v| v.checked_mul(multiplier))
            .ok_or(BandError::ScaleOutOfRange)?;
        if index > 255 {
            return Err(BandError::ScaleOutOfRange);
        }
    }
    for &scale in &scales[23..62] {
        if scale.checked_mul(multiplier).is_none_or(|index| index > 255) {
            return Err(BandError::ScaleOutOfRange);
        }
    }
    let base = f64::from_bits(mp3_requantize::base_gain_bits(p.global_gain));
    let mut out = [0; 576];
    for i in 0..used {
        let (index, sub) = if short && (!p.mixed || i >= 36) {
            let b = (0..13)
                .find(|&b| i < usize::from(table[24 + b]) * 3)
                .ok_or(BandError::InvalidParameters)?;
            let start = usize::from(table[23 + b]) * 3;
            let width = usize::from(table[24 + b] - table[23 + b]);
            let w = (i - start) / width;
            if w >= 3 {
                return Err(BandError::InvalidParameters);
            }
            (
                scales[23 + w * 13 + b]
                    .checked_mul(multiplier)
                    .ok_or(BandError::ScaleOutOfRange)?,
                Some(p.subblock_gain[w]),
            )
        } else {
            let b = (0..22)
                .find(|&b| i < usize::from(table[b + 1]))
                .ok_or(BandError::InvalidParameters)?;
            (
                (scales[b]
                    .checked_add(if p.preflag { PRETAB[b] } else { 0 })
                    .and_then(|v| v.checked_mul(multiplier)))
                .ok_or(BandError::ScaleOutOfRange)?,
                None,
            )
        };
        let index = u8::try_from(index).map_err(|_| BandError::ScaleOutOfRange)?;
        let mut gain = base * f64::from(f32::from_bits(mp3_requantize::scale_factor_bits(index)));
        if let Some(sub) = sub {
            gain *= f64::from(f32::from_bits(mp3_requantize::subblock_factor_bits(sub)));
        }
        out[i] = mp3_requantize::apply_magnitude(magnitude[i], (gain as f32).to_bits(), sign[i] != 0)
            .map_err(BandError::Requantize)?;
    }
    Ok(out)
}

/// Resultado del reordenado monocanal: el original puede escribir fuera del
/// bloque de 576 valores para bloques mixtos. El llamador debe proporcionar
/// espacio explícito y conservar los valores que la rutina no escribe.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReorderFootprint {
    pub writes: usize,
    pub last_destination: usize,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReorderError {
    InvalidParameters,
    DestinationTooShort,
}

/// Emula el tramo de copia y reordenado de 0x26F04270 con un canal, sin estéreo.
/// En mixtos conserva el desplazamiento y el último elemento omitido que se
/// observan en x86, pero exige un búfer capaz de contener las 36 escrituras
/// adicionales; nunca escribe fuera del slice suministrado.
pub fn reorder_mono(p: BandParams, input: &[u32; 576], output: &mut [u32]) -> Result<ReorderFootprint, ReorderError> {
    if p.version > 1 || p.sample_rate_index > 2 || p.block_type > 3 {
        return Err(ReorderError::InvalidParameters);
    }
    let short = p.window_switch && p.block_type == 2;
    let mixed = short && p.mixed;
    let required = if mixed { 612 } else { 576 };
    if output.len() < required {
        return Err(ReorderError::DestinationTooShort);
    }
    if !short {
        output[..576].copy_from_slice(input);
        return Ok(ReorderFootprint {
            writes: 576,
            last_destination: 575,
        });
    }
    if mixed {
        output[..36].copy_from_slice(&input[..36]);
        for (i, &value) in input.iter().enumerate().take(575).skip(36) {
            output[36 + short_destination(p, i + 1)] = value;
        }
        Ok(ReorderFootprint {
            writes: 575,
            last_destination: 611,
        })
    } else {
        for (i, &value) in input.iter().enumerate() {
            output[short_destination(p, i)] = value;
        }
        Ok(ReorderFootprint {
            writes: 576,
            last_destination: 575,
        })
    }
}

fn short_destination(p: BandParams, index: usize) -> usize {
    let t = &BANDS[usize::from(p.version) * 3 + usize::from(p.sample_rate_index)];
    let band = (0..13)
        .find(|&b| index < usize::from(t[24 + b]) * 3)
        .expect("576 indices covered by band table");
    let start = usize::from(t[23 + band]) * 3;
    let width = usize::from(t[24 + band] - t[23 + band]);
    let window = (index - start) / width;
    let freq = usize::from(t[23 + band]) + (index - start) % width;
    (freq / 6) * 18 + window * 6 + freq % 6
}
/// Reordena un canal de la rama de dos canales, una vez calculado su espectro.
/// Esta función no implementa la mezcla mid/side ni el estéreo de intensidad.
pub fn reorder_stereo_channel(
    p: BandParams,
    input: &[u32; 576],
    output: &mut [u32],
) -> Result<ReorderFootprint, ReorderError> {
    if p.version > 1 || p.sample_rate_index > 2 || p.block_type > 3 {
        return Err(ReorderError::InvalidParameters);
    }
    if output.len() < 576 {
        return Err(ReorderError::DestinationTooShort);
    }
    let short = p.window_switch && p.block_type == 2;
    if !short {
        output[..576].copy_from_slice(input);
    } else {
        for (i, &value) in input.iter().enumerate() {
            let dest = if p.mixed && i < 36 { i } else { short_destination(p, i) };
            output[dest] = value;
        }
    }
    Ok(ReorderFootprint {
        writes: 576,
        last_destination: 575,
    })
}
