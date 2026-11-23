//! Etapas espectrales de la síntesis híbrida de Mp3dec escalar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HybridError {
    InvalidBlock,
    NonFiniteInput,
    OutsideValidatedMagnitude,
}
// Perfil conservador: deja margen a las operaciones intermedias almacenadas como f32.
const IMDCT_INPUT_LIMIT: f32 = f32::from_bits(0x71800000); // 2^100
fn validate_imdct_input<'a>(values: impl Iterator<Item = &'a u32>) -> Result<(), HybridError> {
    for &bits in values {
        let value = f32::from_bits(bits);
        if !value.is_finite() {
            return Err(HybridError::NonFiniteInput);
        }
        if value.abs() > IMDCT_INPUT_LIMIT {
            return Err(HybridError::OutsideValidatedMagnitude);
        }
    }
    Ok(())
}
/// Aplica las ocho mariposas a cada una de las 31 fronteras de subbanda.
/// El original omite toda esta etapa cuando hay ventana corta, incluso en mixtos.
pub fn antialias(window_switch: bool, block_type: u8, samples: &mut [u32; 576]) -> Result<(), HybridError> {
    if block_type > 3 {
        return Err(HybridError::InvalidBlock);
    }
    if samples.iter().any(|&v| !f32::from_bits(v).is_finite()) {
        return Err(HybridError::NonFiniteInput);
    }
    if window_switch && block_type == 2 {
        return Ok(());
    }
    const CS: [u32; 8] = [
        0x3f5b84a8, 0x3f61b9d8, 0x3f731add, 0x3f7bba81, 0x3f7eda41, 0x3f7fc8fd, 0x3f7ff965, 0x3f7fff8d,
    ];
    const CA: [u32; 8] = [
        0xbf03b5fe, 0xbef186da, 0xbea07302, 0xbe3a4774, 0xbdc1b01d, 0xbd27cb87, 0xbc68a11d, 0xbb727b46,
    ];
    for band in 1..32 {
        for i in 0..8 {
            let a = band * 18 - 1 - i;
            let b = band * 18 + i;
            let x = f64::from(f32::from_bits(samples[a]));
            let y = f64::from(f32::from_bits(samples[b]));
            let cs = f64::from(f32::from_bits(CS[i]));
            let ca = f64::from(f32::from_bits(CA[i]));
            samples[a] = ((x * cs - y * ca) as f32).to_bits();
            samples[b] = ((y * cs + x * ca) as f32).to_bits();
        }
    }
    Ok(())
}

/// IMDCT larga por subbanda y solapamiento. Tipos 0, 1 y 3; el 2 es corto.
/// El original modifica los coeficientes de entrada y el estado de solapamiento.
pub fn imdct_long(
    block_type: u8,
    subband: usize,
    input: &mut [u32; 18],
    overlap: &mut [u32; 18],
) -> Result<[u32; 18], HybridError> {
    if block_type > 3 || block_type == 2 || subband >= 32 {
        return Err(HybridError::InvalidBlock);
    }
    validate_imdct_input(input.iter().chain(overlap.iter()))?;
    let mut src = input.map(f32::from_bits);
    let mut old = overlap.map(f32::from_bits);
    let window = super::mp3_imdct_long::WINDOWS[block_type as usize].map(f32::from_bits);
    let mut out = [0.0f32; 18];
    super::mp3_imdct_long::apply(&mut src, subband & 1 != 0, &mut out, &mut old, &window);
    *input = src.map(f32::to_bits);
    *overlap = old.map(f32::to_bits);
    Ok(out.map(f32::to_bits))
}

/// IMDCT de tres ventanas cortas de una subbanda, con solapamiento explícito.
pub fn imdct_short(subband: usize, input: &mut [u32; 18], overlap: &mut [u32; 18]) -> Result<[u32; 18], HybridError> {
    if subband >= 32 {
        return Err(HybridError::InvalidBlock);
    }
    validate_imdct_input(input.iter().chain(overlap.iter()))?;
    let mut src = input.map(f32::from_bits);
    let mut old = overlap.map(f32::from_bits);
    let mut out = [0.0f32; 18];
    super::mp3_imdct_short::apply(&mut src, subband & 1 != 0, &mut out, &mut old);
    *input = src.map(f32::to_bits);
    *overlap = old.map(f32::to_bits);
    Ok(out.map(f32::to_bits))
}

/// Información lateral usada por el selector de subbandas del original.
#[derive(Debug, Clone, Copy)]
pub struct HybridChannel {
    pub window_switch: bool,
    pub block_type: u8,
    pub mixed: bool,
}
/// Número calculado por el llamante x86; siempre procesa al menos dos subbandas.
pub fn active_subbands(channels: [HybridChannel; 2], used: [usize; 2]) -> Result<usize, HybridError> {
    if channels.iter().any(|c| c.block_type > 3) || used.iter().any(|&v| v > 576) {
        return Err(HybridError::InvalidBlock);
    }
    if channels.iter().any(|c| c.window_switch && c.block_type == 2) {
        return Ok(32);
    }
    Ok(used
        .into_iter()
        .map(|v| if v == 0 { 1 } else { v.div_ceil(18) })
        .max()
        .unwrap())
}
/// Antialias, IMDCT larga/corta y estado de solapamiento para un canal.
/// Los dos canales comparten el número de subbandas calculado por el llamante.
/// Los buffers se actualizan solo si toda la operación está dentro del perfil.
pub fn hybrid_channel(
    channel: HybridChannel,
    active: usize,
    input: &mut [u32; 576],
    overlap: &mut [u32; 576],
) -> Result<[u32; 576], HybridError> {
    if channel.block_type > 3
        || active > 32
        || active == 0
        || (channel.window_switch && channel.block_type == 2 && active != 32)
    {
        return Err(HybridError::InvalidBlock);
    }
    validate_imdct_input(input.iter().chain(overlap.iter()))?;
    let mut samples = *input;
    let mut history = *overlap;
    let mut output = [0u32; 576];
    antialias(channel.window_switch, channel.block_type, &mut samples)?;
    let short = channel.window_switch && channel.block_type == 2;
    for band in 0..32 {
        let start = band * 18;
        let stop = start + 18;
        if band >= active.max(2) {
            output[start..stop].copy_from_slice(&history[start..stop]);
            history[start..stop].fill(0);
            continue;
        }
        let x: &mut [u32; 18] = (&mut samples[start..stop]).try_into().unwrap();
        let old: &mut [u32; 18] = (&mut history[start..stop]).try_into().unwrap();
        let y = if short && (!channel.mixed || band >= 2) {
            imdct_short(band, x, old)?
        } else {
            imdct_long(
                if short || !channel.window_switch {
                    0
                } else {
                    channel.block_type
                },
                band,
                x,
                old,
            )?
        };
        output[start..stop].copy_from_slice(&y);
    }
    *input = samples;
    *overlap = history;
    Ok(output)
}
