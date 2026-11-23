//! Viento del combate naval (`SeaBattle::update_wind`, 0x006113C9) y deriva visual.
//!
//! El objeto del combate se maneja como bytes (+0x662 id, +0x670 dirección, +0x671 intensidad, +0xAC8 modo directo,
//! +0xADC semilla). Cada llamada consume exactamente una transición de la semilla aunque ningún byte cambie.

/// Cuarto de la tabla trigonométrica del original (0x0067ABB0): `round(65536·sin(k·2π/256))`, k = 0..=64.
pub const SINE_QUARTER: [i32; 65] = [
    0, 1608, 3216, 4821, 6424, 8022, 9616, 11204, 12785, 14359, 15924, 17479, 19024, 20557, 22078, 23586, 25080, 26558,
    28020, 29466, 30893, 32303, 33692, 35062, 36410, 37736, 39040, 40320, 41576, 42806, 44011, 45190, 46341, 47464,
    48559, 49624, 50660, 51665, 52639, 53581, 54491, 55368, 56212, 57022, 57798, 58538, 59244, 59914, 60547, 61145,
    61705, 62228, 62714, 63162, 63572, 63944, 64277, 64571, 64827, 65043, 65220, 65358, 65457, 65516, 65536,
];

const SEED: usize = 0xADC;
const DIRECTION: usize = 0x670;
const STRENGTH: usize = 0x671;
const DIRECT: usize = 0xAC8;
const SLOT: usize = 0x662;

/// Siguiente semilla (misma fórmula que `rng::sea_battle_next`).
pub fn next_seed(seed: u32) -> u32 {
    0xA205_B063u32
        .wrapping_sub(seed.wrapping_mul(0x44BF_19B3))
        .rotate_right(1)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WindOutcome {
    pub changed: bool,
    /// Se ejecuta la rama gráfica: recalcular la velocidad de deriva de las primeras `drift_slots` entidades.
    pub refresh_drift: bool,
    pub drift_slots: u16,
}

/// Seno de la tabla (ángulo de 256 pasos).
pub fn table_sine(angle: u8) -> i32 {
    let half = angle >= 128;
    let local = if half { angle - 128 } else { angle };
    let index = if local <= 64 { local } else { 128 - local } as usize;
    let value = SINE_QUARTER[index];
    if half {
        value.wrapping_neg()
    } else {
        value
    }
}

/// Velocidad visual de deriva (+0x2C, +0x30 de cada entidad); no es la velocidad de un barco.
pub fn drift_velocity(direction: u8, strength: u8) -> (i32, i32) {
    let strength = i32::from(strength);
    let dx = strength.wrapping_mul(table_sine(direction)).wrapping_mul(54) >> 16;
    let dy = (strength
        .wrapping_mul(table_sine(direction.wrapping_add(64)))
        .wrapping_mul(-594)
        / 17)
        >> 16;
    (dx, dy)
}

/// Una llamada de `update_wind`. `displayed_slot` es el byte global 0x006E59CC; `land_attack` el resultado ya resuelto
/// del método virtual 10 (ataque a ciudad = 56 entidades de deriva, combate normal = 224).
pub fn update_wind(object: &mut [u8], displayed_slot: u8, land_attack: bool) -> WindOutcome {
    let old_seed = u32::from_le_bytes(object[SEED..SEED + 4].try_into().unwrap());
    let seed = next_seed(old_seed);
    object[SEED..SEED + 4].copy_from_slice(&seed.to_le_bytes());
    let (old_dir, old_str) = (object[DIRECTION], object[STRENGTH]);
    let (mut direction, mut strength) = (old_dir, old_str);
    let direct = object[DIRECT] != 0;
    let mask: u32 = if direct { 0x0F } else { 0xFF };
    match seed & mask {
        0 => direction = direction.wrapping_add(8),
        1 => direction = direction.wrapping_sub(8),
        _ => {}
    }
    if (seed >> 8) & mask == 0 {
        let target = ((((seed >> 16) & 0xFF) % 10 + 2 + u32::from(strength)) >> 1) as u8;
        if target < strength {
            strength = strength.wrapping_sub(1);
        } else if target > strength {
            strength = strength.wrapping_add(1);
        }
    }
    object[DIRECTION] = direction;
    object[STRENGTH] = strength;
    let changed = direction != old_dir || strength != old_str;
    let slot = u16::from_le_bytes([object[SLOT], object[SLOT + 1]]);
    let refresh = changed && !direct && slot == u16::from(displayed_slot);
    WindOutcome {
        changed,
        refresh_drift: refresh,
        drift_slots: if refresh {
            if land_attack {
                56
            } else {
                224
            }
        } else {
            0
        },
    }
}
