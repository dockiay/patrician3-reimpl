//! `0x0052F030`: cuántas unidades de la mercancía `c` conviene mover de la ciudad `a` a la ciudad `b` y el factor de precio resultante.
//!
//! Las ciudades guardan, por mercancía, cuatro umbrales (`+0x4F0 + 16·c`, `+0x4F4`, `+0x4F8`, `+0x4FC`) entre los que el precio
//! baja de 1,4 a 1,0 (hasta `t1`) y de 1,0 a 0,6 (desde `t1`). Las operaciones en coma flotante se hacen en doble precisión
//! (palabra de control 0x27F) y se redondean a `f32` cada vez que el original guarda un `dword`.

use super::r32;
use super::remove::TOWN_SIZE;

/// Constante `f32` del EXE ampliada a `f64` sin pérdida.
const fn f32_const(bits: u32) -> f64 {
    // Decodificación manual: `f32::from_bits` no es `const` en todas las versiones.
    let sign = if bits >> 31 != 0 { -1.0 } else { 1.0 };
    let exp = ((bits >> 23) & 0xFF) as i32;
    let man = (bits & 0x7F_FFFF) as f64;
    let mut scale = 1.0f64;
    let mut e = exp - 127;
    while e > 0 {
        scale *= 2.0;
        e -= 1;
    }
    while e < 0 {
        scale *= 0.5;
        e += 1;
    }
    sign * (1.0 + man / 8_388_608.0) * scale
}

const K1: f64 = f32_const(0x3FB3_3333); // 0x673A08 = 1,4
const K2: f64 = f32_const(0x3F80_0000); // 0x673A0C = 1,0
const K3: f64 = f32_const(0x3F80_0000); // 0x6739F0 = 1,0
const K4: f64 = f32_const(0x3F4C_CCCD); // 0x6739F4 = 0,8
const K5: f64 = f32_const(0x3F19_999A); // 0x6739F8 = 0,6
const K6: f64 = f32_const(0x3FC0_0000); // 0x6739EC = 1,5

/// Mercancías de lote pequeño en los tratos de la IA (`0x672C14`): umbral 1.000 en lugar de 6.000.
pub const TRADE_FLAG: [u8; 24] = [0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1];
/// Peso de cada mercancía en la puntuación de ciudad (`0x673684`).
pub const TRADE_WEIGHT: [u8; 24] = [
    16, 9, 9, 9, 16, 9, 9, 9, 9, 9, 9, 11, 11, 9, 11, 8, 8, 9, 9, 8, 0, 0, 0, 0,
];

/// Resultado: unidades y factor de precio (`*out` del original).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Trade {
    pub units: i32,
    pub price: f32,
}

/// Error de división por cero del último `idiv`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DivideByZero;

fn f(v: i32) -> f64 {
    f64::from(v as f32)
}

fn st(v: f64) -> f64 {
    f64::from(v as f32)
}

/// `towns` son los bytes de las ciudades (stride `0x9F8`); `base` es el float de `this+0x64`; `p` es el último argumento de pila previo al puntero.
pub fn trade_amount(towns: &[u8], base: f32, a: usize, b: usize, c: usize, p: i32) -> Result<Trade, DivideByZero> {
    let (ta, tb) = (a * TOWN_SIZE, b * TOWN_SIZE);
    let base = f64::from(base);
    let mut out: f32 = 0.0;
    let stock_a = r32(towns, ta + 4 + 4 * c);
    let t = ta + 0x4F0 + 16 * c;
    let (a0, x1, a2, a3) = (r32(towns, t), r32(towns, t + 4), r32(towns, t + 8), r32(towns, t + 12));
    let mid = a0.wrapping_add(x1) >> 1;
    let zero = Trade { units: 0, price: out };
    if mid >= stock_a {
        return Ok(zero);
    }
    let shortfall = r32(towns, tb + 0x310 + 4 * c)
        .wrapping_sub(r32(towns, tb + 0xC4 + 4 * c))
        .wrapping_add(r32(towns, tb + 0x64 + 4 * c));
    let mut eb = r32(towns, tb + 0x670 + 4 * c)
        .wrapping_sub(shortfall.wrapping_mul(p))
        .wrapping_add(r32(towns, tb + 4 + 4 * c));
    if eb < 0 {
        eb = 0;
    }
    let tbase = tb + 0x4F0 + 16 * c;
    let (b0, b1) = (r32(towns, tbase), r32(towns, tbase + 4));
    if b1 < eb {
        return Ok(zero);
    }
    let d = b1.wrapping_sub(b0) >> 3;
    let lim = b0.wrapping_add(d).wrapping_add(d.wrapping_mul(2));
    if eb < b0 {
        out = (base - (base - K1) * f(eb) / f(b0)) as f32;
    } else {
        let g = st(K1 - K2);
        let g = st(f(eb.wrapping_sub(b0)) * g);
        let g = st(g / f(b1.wrapping_sub(b0)));
        out = (K1 - g) as f32;
    }
    let mut sa = stock_a;
    let over = sa.wrapping_sub(x1);
    let mut total: i32;
    if over > 0 {
        if sa < a2 {
            let g = st(K3 - K4);
            let g = st(f(a2.wrapping_sub(sa)) * g);
            let g = st(g / f(a2.wrapping_sub(x1)));
            out = (f64::from(out) - (K4 + g)) as f32;
        } else if sa < a3 {
            let g = st(K4 - K5);
            let g = st(f(a3.wrapping_sub(sa)) * g);
            let g = st(g / f(a3.wrapping_sub(a2)));
            out = (f64::from(out) - (K5 + g)) as f32;
        } else {
            out = (f64::from(out) - K5) as f32;
        }
        let room = b1.wrapping_sub(eb);
        if room < over {
            return Ok(Trade {
                units: room,
                price: out,
            });
        }
        eb = eb.wrapping_add(over);
        total = over;
        sa = sa.wrapping_sub(over);
    } else {
        let g = st(f(x1.wrapping_sub(sa)) + st(K6 - K3));
        let g = st(g / f(x1.wrapping_sub(a0)));
        let value = (f64::from(out) - (K3 + g)) as f32;
        out = value;
        if !(0.0f64 < f64::from(value)) {
            return Ok(Trade { units: 0, price: out });
        }
        total = 0;
    }
    let reach = lim.wrapping_sub(eb);
    if reach >= 0 {
        let free = sa.wrapping_sub(mid);
        if reach >= free {
            return Ok(Trade {
                units: total.wrapping_add(free),
                price: out,
            });
        }
        sa = sa.wrapping_sub(reach);
        total = total.wrapping_add(reach);
        eb = eb.wrapping_add(reach);
    }
    let head = b1.wrapping_sub(lim);
    let num = eb
        .wrapping_sub(lim)
        .wrapping_sub(mid)
        .wrapping_add(sa)
        .wrapping_mul(head);
    let den = head.wrapping_sub(mid).wrapping_add(x1);
    if den == 0 {
        return Err(DivideByZero);
    }
    let q = num.wrapping_div(den).wrapping_add(lim);
    if q < eb {
        return Ok(Trade {
            units: total,
            price: out,
        });
    }
    Ok(Trade {
        units: q.wrapping_sub(eb).wrapping_add(total),
        price: out,
    })
}

/// Mercado de una mercancía en una ciudad, como lo leen `buy_price` y `sell_price`.
pub fn market_of(towns: &[u8], town: usize, ware: usize) -> crate::economy::prices::Market {
    let o = town * TOWN_SIZE;
    crate::economy::prices::Market {
        trading: r32(towns, o + 0x76C) & 0x100 != 0,
        stock: r32(towns, o + 4 + 4 * ware),
        thresholds: std::array::from_fn(|i| r32(towns, o + 0x4F0 + 16 * ware + 4 * i)),
        marked_ware: towns[o + 0x6F0],
    }
}
