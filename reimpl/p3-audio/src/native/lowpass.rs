//! Dos polos de Lowpass.flt, perfil x87 CW027F (53 bits, nearest-even).
//! Entrada/salida en acumulador Q11; coeficientes e historia se guardan como f32.
pub const LOWPASS_PROVIDER: u32 = 0x4c504631;
pub const CUTOFF_KEY: u32 = 0;
pub const NAME: &str = "LowPass Filter";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LowpassError {
    UnsupportedRate(u32),
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LowpassSnapshot {
    pub rate: u32,
    pub cutoff_bits: u32,
    pub history_bits: [[u32; 4]; 2],
    pub coefficient_bits: [u32; 5],
}
pub struct Lowpass {
    rate: u32,
    cutoff: f32,
    coefficients: [f32; 5],
    history: [[f32; 4]; 2],
}
impl Lowpass {
    pub fn new(rate: u32) -> Result<Self, LowpassError> {
        if !(8000..=192000).contains(&rate) {
            return Err(LowpassError::UnsupportedRate(rate));
        }
        let argument = (314.15927_f32 as f64) / (rate as f32 as f64);
        Ok(Self {
            rate,
            cutoff: 100.0,
            coefficients: coefficients(argument),
            history: [[0.0; 4]; 2],
        })
    }
    /// La preferencia devuelve el patrón anterior y conserva las historias.
    pub fn set_cutoff_bits(&mut self, bits: u32) -> u32 {
        let old = self.cutoff.to_bits();
        let requested = f32::from_bits(bits);
        let upper = ((self.rate as f32 as f64) * 0.5 - 1.0) as f32;
        self.cutoff = if requested > upper {
            upper
        } else if requested < 20.0 || requested.is_nan() {
            20.0
        } else {
            requested
        };
        let argument = (self.cutoff as f64) * (std::f32::consts::PI as f64) / (self.rate as f32 as f64);
        self.coefficients = coefficients(argument);
        old
    }
    pub fn snapshot(&self) -> LowpassSnapshot {
        LowpassSnapshot {
            rate: self.rate,
            cutoff_bits: self.cutoff.to_bits(),
            history_bits: self.history.map(|h| h.map(f32::to_bits)),
            coefficient_bits: self.coefficients.map(f32::to_bits),
        }
    }
    /// Ambos canales usan sus propios cuatro valores. Se conserva el orden de
    /// sumas distinto de las dos ramas del kernel original, sin FMA.
    pub fn process_stereo(&mut self, input: [i32; 2]) -> [i32; 2] {
        let c = self.coefficients.map(|v| v as f64);
        std::array::from_fn(|channel| {
            let h = self.history[channel].map(|v| v as f64);
            let x = input[channel] as f64;
            let sum = if channel == 0 {
                (c[2] * h[1] + c[0] * x) + c[1] * h[0]
            } else {
                (c[2] * h[1] + c[1] * h[0]) + c[0] * x
            };
            let y = ((sum - c[3] * h[2]) - c[4] * h[3]) as f32;
            self.history[channel] = [
                input[channel] as f32,
                self.history[channel][0],
                ((y as f64) + (1e-10_f32 as f64)) as f32,
                self.history[channel][2],
            ];
            // FISTP qword y uso del dword bajo. Indefinido x87 -> 0x8000... -> 0.
            let value = (y as f64).round_ties_even();
            if !value.is_finite() || !(-9223372036854775808.0..9223372036854775808.0).contains(&value) {
                0
            } else {
                value as i64 as i32
            }
        })
    }
}
fn coefficients(argument: f64) -> [f32; 5] {
    let c = 1.0 / argument.tan();
    let a = 1.0 / ((std::f64::consts::SQRT_2 + c) * c + 1.0);
    [
        a as f32,
        (a + a) as f32,
        a as f32,
        ((1.0 - c * c) * a * 2.0) as f32,
        ((1.0 - std::f64::consts::SQRT_2 * c + c * c) * a) as f32,
    ]
}
