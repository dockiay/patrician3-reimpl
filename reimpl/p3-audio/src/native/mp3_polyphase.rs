//! Síntesis polifásica escalar, con estado explícito y límites conservadores.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolyphaseError {
    InvalidPosition,
    NonFiniteInput,
    OutsideValidatedMagnitude,
}
const INPUT_LIMIT: f32 = f32::from_bits(0x71800000); // 2^100
fn validate<'a>(values: impl Iterator<Item = &'a u32>) -> Result<(), PolyphaseError> {
    for &bits in values {
        let x = f32::from_bits(bits);
        if !x.is_finite() {
            return Err(PolyphaseError::NonFiniteInput);
        }
        if x.abs() > INPUT_LIMIT {
            return Err(PolyphaseError::OutsideValidatedMagnitude);
        }
    }
    Ok(())
}
/// DCT32 original para un instante de las 18 muestras de cada subbanda.
/// Actualiza únicamente la columna `position` de cada banco de 272 floats.
pub fn dct32(
    input: &[u32; 576],
    time: usize,
    a: &mut [u32; 272],
    b: &mut [u32; 272],
    position: usize,
) -> Result<(), PolyphaseError> {
    if time >= 18 || position >= 16 {
        return Err(PolyphaseError::InvalidPosition);
    }
    validate(input.iter().chain(a.iter()).chain(b.iter()))?;
    let src = input.map(f32::from_bits);
    let mut aa = a.map(f32::from_bits);
    let mut bb = b.map(f32::from_bits);
    super::mp3_poly_dct::apply(&src, time, &mut aa, &mut bb, position);
    if aa.iter().chain(bb.iter()).any(|v| !v.is_finite()) {
        return Err(PolyphaseError::OutsideValidatedMagnitude);
    }
    *a = aa.map(f32::to_bits);
    *b = bb.map(f32::to_bits);
    Ok(())
}

/// Aplica la ventana de síntesis y la conversión PCM16 sin modificar el banco.
pub fn window(bank: &[u32; 272], position: usize, phase: bool) -> Result<[i16; 32], PolyphaseError> {
    if position >= 16 {
        return Err(PolyphaseError::InvalidPosition);
    }
    validate(bank.iter())?;
    let samples = bank.map(f32::from_bits);
    Ok(super::mp3_poly_window::apply(&samples, position, phase))
}

/// Dos bancos de 17 × 16 valores, posición circular y fase para un canal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolyphaseState {
    pub banks: [[u32; 272]; 2],
    pub position: u8,
    pub phase: bool,
}
impl PolyphaseState {
    /// Estado explícito: el llamador decide el estado inicial o restaurado.
    pub fn new(position: u8, phase: bool) -> Result<Self, PolyphaseError> {
        if position >= 16 {
            return Err(PolyphaseError::InvalidPosition);
        }
        Ok(Self {
            banks: [[0; 272]; 2],
            position,
            phase,
        })
    }
    /// Procesa una de las 18 posiciones temporales y devuelve 32 muestras PCM.
    /// Los bancos, fase y posición se actualizan solo si todo el paso es válido.
    pub fn step(&mut self, input: &[u32; 576], time: usize) -> Result<[i16; 32], PolyphaseError> {
        if time >= 18 || self.position >= 16 {
            return Err(PolyphaseError::InvalidPosition);
        }
        let mut next = self.clone();
        let pos = usize::from(next.position);
        let (first, second) = next.banks.split_at_mut(1);
        if next.phase {
            dct32(input, time, &mut second[0], &mut first[0], pos)?;
        } else {
            dct32(input, time, &mut first[0], &mut second[0], pos)?;
        }
        let output = window(&next.banks[usize::from(next.phase)], pos, next.phase)?;
        next.position = (next.position.wrapping_sub(1)) & 15;
        next.phase = !next.phase;
        *self = next;
        Ok(output)
    }
}
