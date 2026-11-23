//! Daño de arma al casco (`LocalMapShip::apply_weapon_damage`, 0x0061F96F).
//!
//! Trabaja sobre los bytes del barco mundial (0x180), el local (0x170) y el combate. Los efectos que dependen de otros
//! sistemas (aviso de hundimiento, baja del índice espacial) se devuelven como eventos para el anfitrión. El recálculo
//! del equipo (0x005182B0) se aplica aquí con `cargo::recompute_load`.

use super::cargo::recompute_load;
use super::wind::next_seed;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DamageEvent {
    /// Aviso de hundimiento (capitán del bando contrario válido); lleva `world+0x42`.
    SinkNotice(u16),
    /// Baja de un registro del índice espacial.
    RemoveSpatial(u16),
    RecalculateEquipment,
}

/// Construye la noticia global 0x2A emitida al hundir un barco pirata.
/// Un capitán inválido o adversario 0xFF no generan noticia.
pub fn pirate_sink_notice(
    captain: u16,
    opponent: u16,
    captains: &[u8],
    merchants: &[u8],
    alderman_town: u8,
) -> Result<Option<[u8; 16]>, &'static str> {
    if captains.len() % 16 != 0 || merchants.len() % 0x650 != 0 {
        return Err("tablas de noticia truncadas");
    }
    if opponent == 0xff || usize::from(captain) >= captains.len() / 16 {
        return Ok(None);
    }
    let mut record = [0u8; 16];
    record[1] = 0x80;
    record[2..4].fill(0xff);
    record[4] = 0x2a;
    let cap = &captains[usize::from(captain) * 16..usize::from(captain) * 16 + 16];
    record[13] = cap[2];
    record[14] = cap[3];
    if usize::from(opponent) < merchants.len() / 0x650 {
        let m = &merchants[usize::from(opponent) * 0x650..(usize::from(opponent) + 1) * 0x650];
        let home = m[0x19];
        record[5] = home;
        record[8] = m[0x1a];
        record[9] = crate::ai::reputation::Merchant { bytes: m.to_vec() }.get_merchant_epithet_index();
        record[10] = m[0x39c + usize::from(home)];
        record[11] = m[0x1c];
        record[12] = m[0x1d];
    } else {
        record[5] = alderman_town;
        record[11] = 0xff;
        record[12] = 0xff;
    }
    Ok(Some(record))
}

fn r16(b: &[u8], o: usize) -> u16 {
    u16::from_le_bytes([b[o], b[o + 1]])
}
fn w16(b: &mut [u8], o: usize, v: u16) {
    b[o..o + 2].copy_from_slice(&v.to_le_bytes());
}
fn r32(b: &[u8], o: usize) -> u32 {
    u32::from_le_bytes(b[o..o + 4].try_into().unwrap())
}
fn w32(b: &mut [u8], o: usize, v: u32) {
    b[o..o + 4].copy_from_slice(&v.to_le_bytes());
}

/// Siguiente valor del generador del combate (`+0xADC`).
pub fn battle_rng(battle: &mut [u8]) -> u32 {
    let value = next_seed(r32(battle, 0xADC));
    w32(battle, 0xADC, value);
    value
}

/// `amount` con signo de 32 bits, `casualties` con signo de 16, `mask` sin signo de 8 (0 = sin banda).
/// Devuelve 0 (sin efecto de casco), 1 (arma destruida) o 2 (hundido). `Err` si el original divide por cero.
pub fn apply_weapon_damage(
    world: &mut [u8],
    local: &mut [u8],
    battle: &mut [u8],
    spatial: (u16, u16),
    amount: i32,
    casualties: i16,
    mask: u8,
    events: &mut Vec<DamageEvent>,
) -> Result<u8, &'static str> {
    let mut casualties = casualties;
    let mut result = 0u8;
    if battle[0x5C] != 0 {
        return Ok(result);
    }
    if casualties < 0 {
        if amount <= 0 {
            casualties = 0;
        } else {
            let raw = battle_rng(battle);
            let numerator = ((raw & 1024) + 3072)
                .wrapping_mul(amount as u32)
                .wrapping_mul(u32::from(r16(world, 0x40)) + 1);
            let denominator = (r32(world, 0x14) as i32 >> 4) as u32;
            if denominator == 0 {
                return Err("división por cero: vida máxima >> 4");
            }
            let total = (numerator / denominator).wrapping_add(u32::from(r16(local, 0x13A)));
            w16(local, 0x13A, total as u16);
            casualties = ((total as i32) >> 16) as i16;
        }
    }
    if local[0x12A] & 6 == 0 && amount > 0 {
        let flags = r16(world, 0x3C) | 0x100;
        w16(world, 0x3C, flags);
        if amount < r32(world, 0x18) as i32 {
            w32(world, 0x18, r32(world, 0x18).wrapping_sub(amount as u32));
        } else {
            let other = 0x35A + 2 * (1isize - isize::from(local[0x13D]));
            if world[0] == 255 {
                let at = usize::try_from(other).map_err(|_| "bando fuera de la tabla de participantes")?;
                if r16(battle, at) != 255 {
                    events.push(DamageEvent::SinkNotice(r16(world, 0x42)));
                }
            }
            w32(world, 0x18, 0);
            local[0x12A] = 4;
            battle[0xAC4] = 1;
            let id = r16(local, 0x138);
            w16(battle, 0xAC6, id);
            events.push(DamageEvent::RemoveSpatial(spatial.0));
            events.push(DamageEvent::RemoveSpatial(spatial.1));
            result = 2;
        }
        if mask != 0 && battle_rng(battle) % 28000 < amount as u32 {
            let side = if mask == 3 {
                (battle_rng(battle) & 1) as u8 + 1
            } else {
                3u8.wrapping_sub(mask)
            };
            let start = usize::from(side.wrapping_shl(1));
            let mut indices = Vec::new();
            let mut pair = start;
            while pair < 24 {
                for index in [pair, pair + 1] {
                    if world[0x13C + index] < 6 {
                        indices.push(index);
                    }
                }
                pair += 4;
            }
            if !indices.is_empty() {
                let chosen = indices[(battle_rng(battle) % indices.len() as u32) as usize];
                if result == 0 {
                    result = 1;
                }
                world[0x13C + chosen] = 255;
                if chosen % 4 == start % 4 && world[0x13D + chosen] == 6 {
                    world[0x13D + chosen] = 255;
                }
                let flags = r16(world, 0x3C) | 16;
                w16(world, 0x3C, flags);
                events.push(DamageEvent::RecalculateEquipment);
                recompute_load(world);
            }
        }
    }
    if casualties > 0 {
        let flags = r16(world, 0x3C) | 2;
        w16(world, 0x3C, flags);
        let crew = r16(world, 0x40);
        if i32::from(casualties) > i32::from(crew) {
            w16(world, 0x42, 0xFFFF);
            w16(world, 0x40, 0);
        } else {
            w16(world, 0x40, crew - casualties as u16);
        }
    }
    Ok(result)
}
