//! Soporte de los portes generados por `oracle/lift_x86.py` (transcripción de funciones x86 del original a Rust).
//!
//! Los portes generados conservan el estado de registros de la función original (EAX..EDI, pila en memoria) y las banderas que usan los saltos
//! condicionales; las llamadas a otras funciones van por [`super::Host`]. Cada porte generado se contrasta con el oráculo igual que los
//! escritos a mano (mismo EAX, mismas escrituras y mismas llamadas en el mismo orden).

use super::Mem;

/// Banderas aritméticas que consultan los saltos (CF, ZF, SF, OF).
#[derive(Clone, Copy, Default)]
pub struct Fl {
    pub cf: bool,
    pub zf: bool,
    pub sf: bool,
    pub of: bool,
    pub pf: bool,
}

fn par(v: u32) -> bool {
    (v as u8).count_ones() % 2 == 0
}

fn mask(bits: u32) -> u32 {
    if bits == 32 {
        u32::MAX
    } else {
        (1u32 << bits) - 1
    }
}

fn sign(v: u32, bits: u32) -> bool {
    (v >> (bits - 1)) & 1 == 1
}

pub fn add(a: u32, b: u32, bits: u32) -> (u32, Fl) {
    adc(a, b, false, bits)
}

pub fn adc(a: u32, b: u32, c: bool, bits: u32) -> (u32, Fl) {
    let mk = mask(bits);
    let (a, b) = (a & mk, b & mk);
    let r = u64::from(a) + u64::from(b) + u64::from(c);
    let res = (r as u32) & mk;
    let of = sign(a, bits) == sign(b, bits) && sign(res, bits) != sign(a, bits);
    (res, Fl { cf: r > u64::from(mk), zf: res == 0, sf: sign(res, bits), of, pf: par(res) })
}

pub fn sub(a: u32, b: u32, bits: u32) -> (u32, Fl) {
    sbb(a, b, false, bits)
}

pub fn sbb(a: u32, b: u32, c: bool, bits: u32) -> (u32, Fl) {
    let mk = mask(bits);
    let (a, b) = (a & mk, b & mk);
    let total = u64::from(b) + u64::from(c);
    let res = (u64::from(a).wrapping_sub(total) as u32) & mk;
    let of = sign(a, bits) != sign(b, bits) && sign(res, bits) != sign(a, bits);
    (res, Fl { cf: u64::from(a) < total, zf: res == 0, sf: sign(res, bits), of, pf: par(res) })
}

/// Banderas de una operación lógica (`and`/`or`/`xor`/`test`).
pub fn logic(res: u32, bits: u32) -> Fl {
    let res = res & mask(bits);
    Fl { cf: false, zf: res == 0, sf: sign(res, bits), of: false, pf: par(res) }
}

pub fn shl(a: u32, n: u32, bits: u32, old: Fl) -> (u32, Fl) {
    let n = n & 31;
    if n == 0 {
        return (a & mask(bits), old);
    }
    let a = a & mask(bits);
    let res = if n >= 32 { 0 } else { (a << n) & mask(bits) };
    let cf = if n <= bits { (u64::from(a) >> (bits - n)) & 1 == 1 } else { false };
    (res, Fl { cf, zf: res == 0, sf: sign(res, bits), of: sign(res, bits) != cf, pf: par(res) })
}

pub fn shr(a: u32, n: u32, bits: u32, old: Fl) -> (u32, Fl) {
    let n = n & 31;
    if n == 0 {
        return (a & mask(bits), old);
    }
    let a = a & mask(bits);
    let res = a >> n;
    (res, Fl { cf: (a >> (n - 1)) & 1 == 1, zf: res == 0, sf: sign(res, bits), of: sign(a, bits), pf: par(res) })
}

pub fn sar(a: u32, n: u32, bits: u32, old: Fl) -> (u32, Fl) {
    let n = n & 31;
    if n == 0 {
        return (a & mask(bits), old);
    }
    let sh = 32 - bits;
    let v = ((a << sh) as i32) >> sh;
    let res = ((v >> n) as u32) & mask(bits);
    (res, Fl { cf: ((v >> (n - 1)) & 1) == 1, zf: res == 0, sf: sign(res, bits), of: false, pf: par(res) })
}

/// `imul` de 32 bits (truncado) con CF = OF = desbordamiento con signo.
pub fn imul(a: u32, b: u32) -> (u32, Fl) {
    let full = i64::from(a as i32) * i64::from(b as i32);
    let res = full as u32;
    let ov = full != i64::from(res as i32);
    (res, Fl { cf: ov, zf: res == 0, sf: sign(res, 32), of: ov, pf: par(res) })
}

pub fn sx8(v: u32) -> u32 {
    v as u8 as i8 as i32 as u32
}

pub fn sx16(v: u32) -> u32 {
    v as u16 as i16 as i32 as u32
}

/// `rep movsd/movsb` hacia delante (DF = 0).
pub fn rep_movs(m: &mut dyn Mem, esi: &mut u32, edi: &mut u32, ecx: &mut u32, size: u32) {
    while *ecx != 0 {
        match size {
            1 => {
                let b = m.r8(*esi);
                m.w8(*edi, b);
            }
            2 => {
                let b = m.r16(*esi);
                m.w16(*edi, b);
            }
            _ => {
                let b = m.r32(*esi);
                m.w32(*edi, b);
            }
        }
        *esi = esi.wrapping_add(size);
        *edi = edi.wrapping_add(size);
        *ecx -= 1;
    }
}

/// `rep stosd/stosb` hacia delante (DF = 0).
pub fn rep_stos(m: &mut dyn Mem, eax: u32, edi: &mut u32, ecx: &mut u32, size: u32) {
    while *ecx != 0 {
        match size {
            1 => m.w8(*edi, eax as u8),
            2 => m.w16(*edi, eax as u16),
            _ => m.w32(*edi, eax),
        }
        *edi = edi.wrapping_add(size);
        *ecx -= 1;
    }
}

/// Pila x87 de los portes generados (aritmética en `f64`; el original trabaja en precisión extendida, el oráculo decide si coincide).
#[derive(Clone, Copy, Default)]
pub struct Fpu {
    s: [f64; 8],
    n: usize,
    c: u16,
}

impl Fpu {
    pub fn push(&mut self, v: f64) {
        self.s.copy_within(0..7, 1);
        self.s[0] = v;
        self.n = (self.n + 1).min(8);
    }
    pub fn pop(&mut self) -> f64 {
        let v = self.s[0];
        self.s.copy_within(1..8, 0);
        self.s[7] = f64::NAN;
        self.n = self.n.saturating_sub(1);
        v
    }
    pub fn st(&self, i: usize) -> f64 {
        self.s[i]
    }
    pub fn set(&mut self, i: usize, v: f64) {
        self.s[i] = v;
    }
    pub fn xch(&mut self, i: usize) {
        self.s.swap(0, i);
    }
    /// `fcom`: fija C3/C2/C0 según `st(0)` frente a `v`.
    pub fn com(&mut self, v: f64) {
        let a = self.s[0];
        self.c = if a.is_nan() || v.is_nan() {
            0x4500
        } else if a > v {
            0
        } else if a < v {
            0x0100
        } else {
            0x4000
        };
    }
    /// Palabra de estado (`fnstsw`): códigos de condición y TOP.
    pub fn sw(&self) -> u32 {
        u32::from(self.c) | ((((8 - self.n) & 7) as u32) << 11)
    }
    pub fn depth(&self) -> usize {
        self.n
    }
}

/// `fistp` con redondeo al par más cercano; indefinido (`i32::MIN`) fuera de rango.
pub fn fist32(v: f64) -> u32 {
    let r = v.round_ties_even();
    if r.is_nan() || r >= 2147483648.0 || r < -2147483648.0 {
        0x8000_0000
    } else {
        r as i32 as u32
    }
}

pub fn fld32(m: &mut dyn Mem, a: u32) -> f64 {
    f64::from(f32::from_bits(m.r32(a)))
}

pub fn fld64(m: &mut dyn Mem, a: u32) -> f64 {
    f64::from_bits(u64::from(m.r32(a)) | (u64::from(m.r32(a.wrapping_add(4))) << 32))
}

pub fn fst32(m: &mut dyn Mem, a: u32, v: f64) {
    m.w32(a, (v as f32).to_bits());
}

pub fn fst64(m: &mut dyn Mem, a: u32, v: f64) {
    let b = v.to_bits();
    m.w32(a, b as u32);
    m.w32(a.wrapping_add(4), (b >> 32) as u32);
}

pub fn fild64(m: &mut dyn Mem, a: u32) -> f64 {
    (u64::from(m.r32(a)) | (u64::from(m.r32(a.wrapping_add(4))) << 32)) as i64 as f64
}

thread_local! {
    static FP_RET: std::cell::Cell<(usize, f64)> = const { std::cell::Cell::new((0, 0.0)) };
}

/// Deja anotado lo que la función generada deja en la pila x87 al volver (profundidad y ST0; las que devuelven `float` lo usan).
pub fn set_fp(depth: usize, st0: f64) {
    FP_RET.with(|c| c.set((depth, st0)));
}

/// Recoge (y borra) la profundidad x87 y ST0 que dejó la última función generada.
pub fn take_fp() -> (usize, f64) {
    FP_RET.with(|c| c.replace((0, 0.0)))
}
