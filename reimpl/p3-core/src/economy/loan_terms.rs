//! Condiciones de un préstamo (`loan_compute_interest_and_default`, `0x0051D270`): tipo, total a devolver y posible impago.
//! Registro de 0x10 bytes: `+0` u32 total, `+4` tipo (u8), `+5` u8, `+8` principal (u16), `+0xA` semanas, `+0xB` impago (u8), `+0xD` u8, `+0xF` nivel.
//! Los intermedios del x87 (53 bits) se redondean a f32 solo donde el original hace `fstp dword`.
use crate::numeric::ftol_i32;
use crate::rng::CrtRand;

fn f(x: f64) -> f64 {
    f64::from(x as f32)
}

pub fn compute(rec: &mut [u8], rng: &mut CrtRand) {
    let principal = u32::from(rec[8]) | u32::from(rec[9]) << 8;
    let weeks = u32::from(rec[0xa]);
    let p = f(f64::from(principal));
    let w = f(f64::from(weeks));
    let t = f(w * p);
    // 0x0063AAE4 es la raíz cuadrada; la división se hace en 53 bits.
    let root = f((1.0f64 / t).sqrt());
    let x = f(root * 300.0 + f64::from(1.2f32));
    let level = f(f64::from(u32::from(rec[0xf] & 0xf) + 8));
    let u = f(level * x);
    let rate = (ftol_i32(u) & 0xff) as u8;
    rec[4] = rate;
    let factor = f(f(f64::from(rate)) * f64::from(0.001f32) + 1.0);
    let mut v = p;
    for _ in 0..weeks {
        v = f(v * factor);
    }
    let total: u32 = if v > 65000.0 || v < p || v.is_nan() {
        0xfde8
    } else {
        (ftol_i32(v) as u32) & 0xffff
    };
    rec[0..4].copy_from_slice(&total.to_le_bytes());
    let flag = rec[0xd];
    rec[0xb] = 0;
    if flag == 0xff {
        let base = (u32::from(rec[5]) + 1).wrapping_mul(5000);
        if base < total {
            let c = total - base + 0x4e2;
            let q = ((u64::from(c) * 0xd1b7_1759) >> 32) as u32 >> 10;
            let r1 = (rng.rand() as u32) & 0x3ff;
            if r1 < q * 75 {
                let r2 = (rng.rand() as u32) & 0xff;
                rec[0xb] = (r2 % 0x4e + 0x33) as u8;
            }
        }
    }
}
