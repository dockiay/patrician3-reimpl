//! Precios de compra y venta: `get_buy_price` (0x0052E430) y `get_sell_price` (0x0052E1D0).
//!
//! El precio recorre los cinco intervalos que delimitan los umbrales de la ciudad. En cada uno el
//! factor varía linealmente y se evalúa en el punto medio del tramo comprado o vendido; por encima
//! del último umbral el factor es fijo. El resultado se multiplica por el precio base y se trunca con
//! `__ftol`.
//!
//! El original calcula en la FPU x87 con la precisión de 53 bits que deja Windows (palabra de control
//! 0x27F; el juego no la cambia) y guarda cada resultado intermedio como `f32`. Aquí cada operación se
//! hace en `f64` y se redondea a `f32` donde el original hace `fstp dword`. Los contrastes ejecutan
//! con 0x27F y cubren casos normales y extremos, no todas las entradas ni otros entornos de FPU.
//! El índice de mercancía debe estar en 0..24; las lecturas fuera de tabla del original no se modelan.

use crate::numeric::ftol_i32;

/// Mercancías con precio (20 normales y 4 armas).
pub const WARE_COUNT: usize = 24;

/// Precio base por unidad (0x00673A18).
pub const BASE_PRICE: [f32; WARE_COUNT] = [
    0.055000003,
    0.47855002,
    0.22005,
    0.174,
    0.1425,
    0.55,
    1.4,
    1.1,
    1.034,
    3.3825,
    0.4125,
    0.027500002,
    1.278,
    1.12,
    0.44000003,
    0.278,
    0.44000003,
    0.22000001,
    0.85499996,
    0.039900005,
    10.0,
    8.0,
    12.0,
    14.0,
];

/// Factores de compra en cada umbral y por encima del último (0x006739E8).
pub const BUY_FACTORS: [f32; 5] = [4.0, 1.5, 1.0, 0.8, 0.6];

/// Factores de venta en cada umbral; el último vale también por encima de `t3` (0x00673A08).
pub const SELL_FACTORS: [f32; 4] = [1.4, 1.0, 0.7, 0.5];

/// Mercancías que se venden desde 200 unidades en lugar de 2.000 (0x00672C14).
pub const SMALL_LOT_WARES: [bool; WARE_COUNT] = [
    false, false, false, true, true, true, true, true, true, true, true, false, true, true, false, true, false, false,
    true, false, true, true, true, true,
];

/// Recargo de compra sobre la mercancía marcada en `Town+0x6F0` (0x00673A7C).
pub const MARKED_WARE_SURCHARGE: f32 = 1.15;

/// Lo que los precios leen de una ciudad.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Market {
    /// Bit 0x100 de `Town+0x76C`: sin él la ciudad no comercia y ambos precios valen 0.
    pub trading: bool,
    /// Existencias de la mercancía (`Town+4+4·w`).
    pub stock: i32,
    /// Umbrales de precio de la mercancía (`Town+0x4F0+16·w`).
    pub thresholds: [i32; 4],
    /// Byte `Town+0x6F0`: la mercancía con recargo de compra.
    pub marked_ware: u8,
}

fn f32_of(x: i32) -> f64 {
    x as f32 as f64
}

/// Factor en el punto medio de `[low, high]` y suma del tramo, como el bucle común del original.
fn accumulate(acc: f32, low: i32, high: i32, slope: f32, start: f32) -> f32 {
    let mid = f32_of(high.wrapping_add(low));
    let factor = (mid * slope as f64 * 0.5 + start as f64) as f32;
    (f32_of(high.wrapping_sub(low)) * factor as f64 + acc as f64) as f32
}

/// Primer intervalo cuyo umbral supera `low`, o `None` si `low` está por encima de `t3`.
fn first_interval(low: i32, t: &[i32; 4]) -> Option<usize> {
    let mut i = 0;
    if low >= t[0] {
        loop {
            if i >= 3 {
                return None;
            }
            i += 1;
            if low < t[i] {
                break;
            }
        }
    }
    Some(i)
}

/// Precio que paga quien compra `amount` unidades de `ware` (`get_buy_price`).
///
/// Comprar más de lo que hay cobra el exceso a 4 veces el precio base. El recargo de la mercancía
/// marcada no se aplica cuando las existencias que quedan superan `t3`, igual que en el original.
pub fn buy_price(ware: usize, market: &Market, amount: i32) -> i32 {
    if !market.trading {
        return 0;
    }
    let stock = market.stock;
    if stock < if SMALL_LOT_WARES[ware] { 200 } else { 2000 } {
        return 0;
    }
    let t = &market.thresholds;
    let base = BASE_PRICE[ware] as f64;
    let remaining = stock.wrapping_sub(amount);
    let (mut low, excess) = if remaining < 0 {
        (0, (f32_of(remaining.wrapping_neg()) * BUY_FACTORS[0] as f64) as f32)
    } else {
        (remaining, 0.0)
    };
    let Some(mut i) = first_interval(low, t) else {
        return ftol_i32(f32_of(stock.wrapping_sub(low)) * base * BUY_FACTORS[4] as f64);
    };
    let mut acc = 0.0f32;
    loop {
        let (high, last) = if i < 4 && stock > t[i] {
            (t[i], false)
        } else {
            (stock, true)
        };
        let (slope, start) = match i {
            0 => (
                ((BUY_FACTORS[1] as f64 - BUY_FACTORS[0] as f64) / f32_of(t[0])) as f32,
                BUY_FACTORS[0],
            ),
            1..=3 => {
                let slope =
                    ((BUY_FACTORS[i + 1] as f64 - BUY_FACTORS[i] as f64) / f32_of(t[i].wrapping_sub(t[i - 1]))) as f32;
                (slope, (BUY_FACTORS[i] as f64 - f32_of(t[i - 1]) * slope as f64) as f32)
            }
            _ => (0.0, BUY_FACTORS[4]),
        };
        acc = accumulate(acc, low, high, slope, start);
        low = high;
        i += 1;
        if last {
            break;
        }
    }
    let mut price = (excess as f64 + acc as f64) * base;
    if market.marked_ware as usize == ware {
        price *= MARKED_WARE_SURCHARGE as f64;
    }
    ftol_i32(price)
}

/// Precio que recibe quien vende `amount` unidades de `ware` (`get_sell_price`).
///
/// `difficulty` es el factor de venta con existencias nulas (`this+0x64`, 2,2/2,0/1,8 según la
/// dificultad comercial); el primer intervalo va de él a 1,4.
pub fn sell_price(ware: usize, market: &Market, amount: i32, difficulty: f32) -> i32 {
    if !market.trading || amount < 1 {
        return 0;
    }
    let t = &market.thresholds;
    let base = BASE_PRICE[ware] as f64;
    let end = market.stock.wrapping_add(amount);
    let mut low = market.stock.max(0);
    let Some(mut i) = first_interval(low, t) else {
        return ftol_i32(f32_of(end.wrapping_sub(low)) * base * SELL_FACTORS[3] as f64);
    };
    let mut acc = 0.0f32;
    loop {
        let (high, last) = if i < 4 && end > t[i] {
            (t[i], false)
        } else {
            (end, true)
        };
        let (slope, start) = match i {
            0 => (
                ((SELL_FACTORS[0] as f64 - difficulty as f64) / f32_of(t[0])) as f32,
                difficulty,
            ),
            1..=3 => {
                let slope = ((SELL_FACTORS[i - 1] as f64 - SELL_FACTORS[i] as f64)
                    / f32_of(t[i - 1].wrapping_sub(t[i]))) as f32;
                (
                    slope,
                    (SELL_FACTORS[i - 1] as f64 - f32_of(t[i - 1]) * slope as f64) as f32,
                )
            }
            _ => (0.0, SELL_FACTORS[3]),
        };
        acc = accumulate(acc, low, high, slope, start);
        low = high;
        i += 1;
        if last {
            break;
        }
    }
    ftol_i32(acc as f64 * base)
}

/// Límite de existencias para vender al precio indicado (0x0052EA80).
pub fn sell_stock_limit(ware: usize, market: &Market, price: i32, difficulty: f32) -> i32 {
    if !market.trading {
        return if price > 0 { 0 } else { i32::MAX };
    }
    let lot = if SMALL_LOT_WARES[ware] { 200.0 } else { 2000.0 };
    let ratio =
        (((price as f32 as f64) - (f32::from_bits(0x3f666666) as f64)) / ((BASE_PRICE[ware] as f64) * lot)) as f32;
    let factors = SELL_FACTORS;
    if ratio <= factors[3] {
        return i32::MAX;
    }
    let t = market.thresholds;
    for segment in (1..=3).rev() {
        if ratio < factors[segment - 1] {
            let width = t[segment].wrapping_sub(t[segment - 1]) as f32 as f64;
            let value = ((factors[segment - 1] as f64 - ratio as f64) * width)
                / (factors[segment - 1] as f64 - factors[segment] as f64);
            return t[segment - 1].wrapping_add(ftol_i32(value));
        }
    }
    if ratio < difficulty {
        let value =
            ((difficulty as f64 - ratio as f64) * (t[0] as f32 as f64)) / (difficulty as f64 - factors[0] as f64);
        return ftol_i32(value);
    }
    0
}

/// Límite de existencias para comprar al precio indicado (0x0052EC50).
pub fn buy_stock_limit(ware: usize, market: &Market, price: i32, discount: u32) -> i32 {
    if !market.trading {
        return market.stock;
    }
    let lot = if SMALL_LOT_WARES[ware] { 200.0 } else { 2000.0 };
    let mut ratio =
        (((price as f32 as f64) + (f32::from_bits(0x3f666666) as f64)) / ((BASE_PRICE[ware] as f64) * lot)) as f32;
    if discount > 0 {
        let difference = (2560.0f64 - (discount as u8 as f64)) as f32;
        ratio = ((2560.0f64 / (difference as f64)) * (ratio as f64)) as f32;
    }
    if market.marked_ware as usize == ware {
        ratio = ((ratio as f64) * (MARKED_WARE_SURCHARGE as f64)) as f32;
    }
    let factors = BUY_FACTORS;
    if ratio <= factors[4] {
        return i32::MAX;
    }
    let t = market.thresholds;
    for segment in (1..=3).rev() {
        if ratio < factors[segment] {
            let width = t[segment].wrapping_sub(t[segment - 1]) as f32 as f64;
            let value = ((factors[segment] as f64 - ratio as f64) * width)
                / (factors[segment] as f64 - factors[segment + 1] as f64);
            return t[segment - 1].wrapping_add(ftol_i32(value));
        }
    }
    if ratio < factors[0] {
        let value =
            ((factors[0] as f64 - ratio as f64) * (t[0] as f32 as f64)) / (factors[0] as f64 - factors[1] as f64);
        return ftol_i32(value);
    }
    0
}
