//! Velocidad y rumbo de un barco (`ship::get_speed`, `ship::set_velocity_towards`).
//!
//! `ship` son los 0x180 bytes del barco. Campos usados: `+1` rumbo (0..15), `+0xE` tipo, `+0x10` capacidad,
//! `+0x14` vida máxima, `+0x18` vida, `+0x1E`/`+0x22` posición (i16), `+0x28`/`+0x2C` velocidad (i32 en
//! punto fijo de 16 bits) y `+0x118` peso de la carga.

use super::{r16, r32, w32, DivideError};
use crate::numeric::ftol_i32;

/// Velocidad base por tipo de barco (`0x0067366C`, índice `tipo & 3`).
pub const SPEED_TABLE: [u16; 4] = [635, 693, 520, 578];

/// `ship::get_speed` (0x005170F0). Divisiones: carga entre capacidad con signo (cociente fuera de rango
/// o capacidad cero fallan en x86) y vida entre vida máxima sin signo (vida máxima cero falla).
pub fn get_speed(ship: &[u8]) -> Result<u32, DivideError> {
    let load = r32(ship, 0x118).wrapping_mul(0x266);
    let capacity = r32(ship, 0x10);
    if capacity == 0 || (load == i32::MIN && capacity == -1) {
        return Err(DivideError);
    }
    let fill = load / capacity;
    let table = SPEED_TABLE[(ship[0xE] & 3) as usize] as u32;
    let by_load = (0x1000u32.wrapping_sub(fill as u32)).wrapping_mul(table) >> 12;
    let max_health = r32(ship, 0x14) as u32;
    if max_health == 0 {
        return Err(DivideError);
    }
    let hull = ((r32(ship, 0x18).wrapping_mul(0x82)) as u32 / max_health).wrapping_add(0xA5);
    if hull < 0xCC {
        return Ok(by_load.wrapping_mul(0xCC) >> 10);
    }
    Ok(hull.min(0x100).wrapping_mul(by_load) >> 10)
}

/// División con redondeo hacia la mitad del paso de 16 bits, como la del original.
fn scaled(delta: i32, distance: i32) -> Result<i32, DivideError> {
    let numerator = if delta < 1 {
        delta.wrapping_mul(0x10000).wrapping_sub(0x7FFF)
    } else {
        delta.wrapping_mul(0x10000).wrapping_add(0x8000)
    };
    let divisor = 1i32.wrapping_sub(distance);
    if divisor == 0 || (numerator == i32::MIN && divisor == -1) {
        return Err(DivideError);
    }
    Ok(numerator / divisor)
}

/// Constante `0x00673678`: la distancia se escala por -128 antes de dividirla entre la velocidad.
pub const DISTANCE_SCALE: f64 = -128.0;

/// `ship::set_velocity_towards` (0x00517410): velocidad (x, y) hacia un punto y rumbo de 0 a 15 en `+1`.
/// `speed` es el primer argumento de pila (divisor con signo de la distancia). Las componentes se dividen
/// entre `1 - ftol(-128 · distancia / speed)`; si eso vale cero, el original falla con #DE. Con `speed` cero
/// x87 produce infinito o NaN y `__ftol` devuelve el entero indefinido (parte baja 0).
pub fn set_velocity_towards(ship: &mut [u8], speed: i32, x: i16, y: i16) -> Result<(), DivideError> {
    let dx = x as i32 - r16(ship, 0x1E) as i16 as i32;
    let dy = y as i32 - r16(ship, 0x22) as i16 as i32;
    let squared = dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx));
    let distance = ftol_i32((squared as f64).sqrt() * DISTANCE_SCALE / speed as f64);
    let vx = if dx == 0 { 0 } else { scaled(dx, distance)? };
    w32(ship, 0x28, vx);
    let vy = if dy == 0 { 0 } else { scaled(dy, distance)? };
    w32(ship, 0x2C, vy);
    let (base, p, q): (u8, i32, i32) = if vx < 1 {
        if vy < 1 {
            (1, vx.wrapping_neg(), vy.wrapping_neg())
        } else {
            (0xD, vy, vx.wrapping_neg())
        }
    } else if vy < 1 {
        (5, vy.wrapping_neg(), vx)
    } else {
        (9, vx, vy)
    };
    ship[1] = base;
    let bump = if q < p {
        let shifted = (q as i64) << 16;
        if (p as i64).wrapping_mul(0x96CC) < shifted {
            Some(2)
        } else if shifted <= (p as i64).wrapping_mul(0x3244) {
            None
        } else {
            Some(1)
        }
    } else {
        let shifted = (p as i64) << 16;
        if (q as i64).wrapping_mul(0x96CC) < shifted {
            Some(2)
        } else if (q as i64).wrapping_mul(0x3244) < shifted {
            Some(3)
        } else {
            Some(4)
        }
    };
    if let Some(add) = bump {
        ship[1] = base.wrapping_add(add);
    }
    ship[1] &= 0xF;
    Ok(())
}
