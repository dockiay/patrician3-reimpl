//! Andanada de una banda (`LocalMapShip::fire_volley`, 0x0061E8EF): corrige la dirección dentro de una ventana, limita el
//! alcance con el viento, recorre las armas de la banda y crea un proyectil por arma (0x006214CB).
//! Fixture: `oracle/x86_battle_volley.py`.

use super::projectile::{create_shot, heading, ProjectileTable, ShotArgs, ShotContext, Spatial};

/// Cuarto de seno 0x0067AF88: `round(65536 * sin(k * 2π / 256))`, k = 0..=64.
pub const SINE_QUARTER: [i32; 65] = [
    0, 1608, 3216, 4821, 6424, 8022, 9616, 11204, 12785, 14359, 15924, 17479, 19024, 20557, 22078, 23586, 25080, 26558,
    28020, 29466, 30893, 32303, 33692, 35062, 36410, 37736, 39040, 40320, 41576, 42806, 44011, 45190, 46341, 47464,
    48559, 49624, 50660, 51665, 52639, 53581, 54491, 55368, 56212, 57022, 57798, 58538, 59244, 59914, 60547, 61145,
    61705, 62228, 62714, 63162, 63572, 63944, 64277, 64571, 64827, 65043, 65220, 65358, 65457, 65516, 65536,
];

/// Seno de punto fijo (±65536) para un ángulo de 0..255, como las lecturas con simetría del original.
pub fn sine(angle: u8) -> i32 {
    let a = angle as usize;
    if a < 0x80 {
        if a <= 0x40 {
            SINE_QUARTER[a]
        } else {
            SINE_QUARTER[0x80 - a]
        }
    } else {
        let e = a - 0x80;
        -(if e <= 0x40 {
            SINE_QUARTER[e]
        } else {
            SINE_QUARTER[0x80 - e]
        })
    }
}

/// Artillería: ataque A (0x00672CC0), B (0x00672CD0) y alcance R (0x00672CE4) por código de arma 0..5.
const WEAPON_A: [i32; 6] = [32, 32, 77, 77, 96, 58];
const WEAPON_B: [i32; 6] = [60, 80, 60, 80, 90, 90];
const WEAPON_R: [i32; 6] = [480, 160, 480, 160, 120, 120];

/// Datos del barco que dispara (0x170 bytes locales, posición y armas del barco mundial `+0x13C`, 24 bytes).
pub struct Volley<'a> {
    pub local: &'a mut [u8],
    pub weapons: [u8; 24],
    pub ship: (i16, i16),
}

fn idiv(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 || (a == i32::MIN && b == -1) {
        return Err("excepción de división (#DE)");
    }
    Ok(a / b)
}

/// 0x0061E8EF. `side` es 0 ó 1 (la banda); el destino es el punto al que apunta. Escribe la recarga `0x96` en
/// `local[0x130 + side]` sólo si hay armas válidas en la banda.
pub fn fire_volley(
    v: &mut Volley,
    wind: u8,
    side: u8,
    target: (i16, i16),
    shoot: &mut dyn FnMut(&ShotArgs) -> Result<(), &'static str>,
) -> Result<(), &'static str> {
    let heading_ship = v.local[0x11];
    let dx = i32::from(target.0) - i32::from(v.ship.0);
    let dy = i32::from(target.1) - i32::from(v.ship.1);
    let d2 = dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy));
    if d2 < 0 {
        return Err("raíz de un cuadrado desbordado (ruta matherr de la CRT no modelada)");
    }
    let length = f64::from(d2).sqrt() as i32;
    let angle = heading(dx, dy);
    let relative = if side == 0 {
        angle.wrapping_sub(heading_ship).wrapping_add(0x40)
    } else {
        angle.wrapping_sub(heading_ship).wrapping_sub(0x40)
    };
    let (mut dx, mut dy) = (dx, dy);
    if !(relative <= 8 || relative >= 0xF8) {
        let delta: u8 = if relative < 0x80 { 0x08 } else { 0xF8 };
        let direction = heading_ship.wrapping_add(delta);
        let sign = if side == 0 { -1 } else { 1 };
        dx = (sign * sine(direction.wrapping_add(0x40)).wrapping_mul(length)) >> 16;
        dy = (sign * sine(direction).wrapping_mul(length)) >> 16;
    }
    // alcance máximo según el viento (battle+0x670) y el rumbo del barco
    let r = wind.wrapping_sub(heading_ship);
    let b = if r < 0x80 {
        i32::from(r) - 0x40
    } else {
        0xC0 - i32::from(r)
    };
    let max_distance = b.wrapping_mul(4320) / 3200 + 480;
    let mut length = length;
    if length > max_distance {
        dx = idiv(max_distance.wrapping_mul(dx), length)?;
        dy = idiv(max_distance.wrapping_mul(dy), length)?;
        length = max_distance;
    }
    let aim = (v.ship.0.wrapping_add(dx as i16), v.ship.1.wrapping_add(dy as i16));
    let first = usize::from(side) * 2;
    if first >= 24 {
        return Ok(());
    }
    let positions = (0x17 - first) / 4 + 1;
    let mut count = 0u8;
    for k in 0..positions {
        let at = first + 4 * k;
        count = count
            .wrapping_add(u8::from(v.weapons[at] < 6))
            .wrapping_add(u8::from(v.weapons[at + 1] < 6));
    }
    if count == 0 {
        return Ok(());
    }
    let mut index: u8 = if side != 0 { count.wrapping_sub(1) } else { 0 };
    for k in 0..positions {
        for slot in [0usize, 1] {
            let weapon = v.weapons[first + 4 * k + slot];
            if weapon < 6 {
                let w = usize::from(weapon);
                let base = WEAPON_B[w] * WEAPON_A[w];
                let divisor = (WEAPON_R[w] * 4 - WEAPON_R[w]).wrapping_neg().wrapping_mul(2);
                let raw = base + idiv(base.wrapping_mul(length), divisor)?;
                let raw = if raw < 0 { 0 } else { raw };
                shoot(&ShotArgs {
                    target: aim,
                    raw_damage: raw,
                    index,
                    count,
                    unused: u16::from(weapon >= 4),
                })?;
                index = if side != 0 {
                    index.wrapping_sub(1)
                } else {
                    index.wrapping_add(1)
                };
            }
        }
    }
    v.local[0x130 + usize::from(side)] = 0x96;
    Ok(())
}

/// Andanada completa: cada arma crea su proyectil con `create_shot`. El viento es `battle+0x670`.
pub fn fire_volley_real<S: Spatial, F: FnMut() -> u32>(
    v: &mut Volley,
    shot: &ShotContext,
    table: &mut ProjectileTable,
    battle: &mut [u8],
    spatial: &mut S,
    rand: &mut F,
    side: u8,
    target: (i16, i16),
) -> Result<(), &'static str> {
    let wind = battle[0x670];
    let battle_cell = std::cell::RefCell::new(battle);
    let table_cell = std::cell::RefCell::new(table);
    let spatial_cell = std::cell::RefCell::new(spatial);
    let rand_cell = std::cell::RefCell::new(rand);
    fire_volley(v, wind, side, target, &mut |args| {
        create_shot(
            &mut table_cell.borrow_mut(),
            &mut battle_cell.borrow_mut(),
            &mut **spatial_cell.borrow_mut(),
            &mut **rand_cell.borrow_mut(),
            shot,
            args,
        )
        .map(|_| ())
    })
}
