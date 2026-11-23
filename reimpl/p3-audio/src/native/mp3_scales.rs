//! Lector de escalas MPEG-1 de Mp3dec. Conserva los campos que el original no escribe.
#[derive(Debug, Clone, Copy)]
pub struct ScaleParams {
    pub start_bit: usize,
    pub end_bit: usize,
    pub granule: u8,
    pub compress: u16,
    pub window_switch: bool,
    pub block_type: u8,
    pub mixed: bool,
    pub scfsi: [bool; 4],
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScaleError {
    InvalidParameters,
    Truncated,
}
#[derive(Debug, Clone)]
pub struct ScaleResult {
    pub values: [u32; 62],
    pub end_bit: usize,
}

/// El original usa 22 entradas largas y tres ventanas de 13 entradas a partir
/// de la posición 23. No rellena ni reinicia todos los huecos entre granulos.
pub fn read_mpeg1(
    input: &[u8],
    p: ScaleParams,
    prior: [u32; 62],
    previous: &[u32; 62],
) -> Result<ScaleResult, ScaleError> {
    const LEN1: [usize; 16] = [0, 0, 0, 0, 3, 1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4];
    const LEN2: [usize; 16] = [0, 1, 2, 3, 0, 1, 2, 3, 1, 2, 3, 1, 2, 3, 2, 3];
    if p.granule > 1 || p.compress > 15 || p.start_bit > p.end_bit || p.end_bit > input.len().saturating_mul(8) {
        return Err(ScaleError::InvalidParameters);
    }
    let mut at = p.start_bit;
    let mut values = prior;
    let a = LEN1[p.compress as usize];
    let b = LEN2[p.compress as usize];
    let mut read = |width: usize| -> Result<u32, ScaleError> {
        if at.checked_add(width).is_none_or(|end| end > p.end_bit) {
            return Err(ScaleError::Truncated);
        }
        let mut v = 0;
        for _ in 0..width {
            v = (v << 1) | u32::from((input[at / 8] >> (7 - at % 8)) & 1);
            at += 1;
        }
        Ok(v)
    };
    if p.window_switch && p.block_type == 2 {
        if p.mixed {
            for slot in values.iter_mut().take(8) {
                *slot = read(a)?;
            }
        }
        let first = if p.mixed { 3 } else { 0 };
        for band in first..12 {
            let width = if band < 6 { a } else { b };
            for window in 0..3 {
                values[23 + window * 13 + band] = read(width)?;
            }
        }
        for window in 0..3 {
            values[23 + window * 13 + 12] = 0;
        }
    } else {
        let boundaries = [0, 6, 11, 16, 21];
        for group in 0..4 {
            let range = boundaries[group]..boundaries[group + 1];
            if p.granule != 0 && p.scfsi[group] {
                values[range.clone()].copy_from_slice(&previous[range]);
            } else {
                for slot in &mut values[range] {
                    *slot = read(if group < 2 { a } else { b })?;
                }
            }
        }
        values[22] = 0;
    }
    Ok(ScaleResult { values, end_bit: at })
}

/// MPEG-2/2.5: interpreta los cuatro anchos y sus particiones antes de leer.
/// `intensity` corresponde al segundo canal en el modo de intensidad del original.
pub fn read_mpeg2(input: &[u8], p: ScaleParams, prior: [u32; 62], intensity: bool) -> Result<ScaleResult, ScaleError> {
    if p.compress > 511
        || (intensity && p.compress >= 510)
        || p.start_bit > p.end_bit
        || p.end_bit > input.len().saturating_mul(8)
    {
        return Err(ScaleError::InvalidParameters);
    }
    const COUNTS: [[[usize; 4]; 3]; 6] = [
        [[7, 7, 7, 0], [12, 12, 12, 0], [6, 15, 12, 0]],
        [[6, 6, 6, 3], [12, 9, 9, 6], [6, 12, 9, 6]],
        [[8, 8, 5, 0], [15, 12, 9, 0], [6, 18, 9, 0]],
        [[6, 5, 5, 5], [9, 9, 9, 9], [6, 9, 9, 9]],
        [[6, 5, 7, 3], [9, 9, 12, 6], [6, 9, 12, 6]],
        [[11, 10, 0, 0], [18, 18, 0, 0], [15, 18, 0, 0]],
    ];
    let c = usize::from(p.compress);
    let (category, width) = if intensity {
        let n = c / 2;
        if n < 180 {
            (0, [n / 36, (n % 36) / 6, n % 6, 0])
        } else if n < 244 {
            let n = n - 180;
            (1, [n / 16, (n % 16) / 4, n % 4, 0])
        } else {
            let n = n - 244;
            (2, [n / 3, n % 3, 0, 0])
        }
    } else if c < 400 {
        let n = c / 16;
        (3, [n / 5, n % 5, (c % 16) / 4, c % 4])
    } else if c < 500 {
        let n = c - 400;
        (4, [(n / 4) / 5, (n / 4) % 5, n % 4, 0])
    } else {
        let n = c - 500;
        (5, [n / 3, n % 3, 0, 0])
    };
    let short = p.window_switch && p.block_type == 2;
    let kind = if !short {
        0
    } else if p.mixed {
        2
    } else {
        1
    };
    let mut at = p.start_bit;
    let mut values = prior;
    let mut index = 0;
    for group in 0..4 {
        for _ in 0..COUNTS[category][kind][group] {
            let bits = width[group];
            if at.checked_add(bits).is_none_or(|end| end > p.end_bit) {
                return Err(ScaleError::Truncated);
            }
            let mut value = 0;
            for _ in 0..bits {
                value = (value << 1) | u32::from((input[at / 8] >> (7 - at % 8)) & 1);
                at += 1;
            }
            let dest = if !short || (p.mixed && index < 6) {
                index
            } else {
                let i = if p.mixed { index - 6 } else { index };
                let band = if p.mixed { 3 + i / 3 } else { i / 3 };
                23 + (i % 3) * 13 + band
            };
            values[dest] = value;
            index += 1;
        }
    }
    Ok(ScaleResult { values, end_bit: at })
}
