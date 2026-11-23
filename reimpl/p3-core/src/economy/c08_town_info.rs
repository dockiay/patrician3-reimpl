//! C08: historial de información de ciudades (`0x006DE588`) en su forma serializada S04.
//!
//! - `town_info_history_mark_town_known` (`0x00535400`): en cada ranura del comerciante pone la edad de la ciudad a 0
//!   y relaja las demás.
//! - `town_info_history_relax_ages` (`0x00535630`): desde cada ciudad conocida (edad 0) la edad de las demás baja a
//!   `distancia/16 + 1`; las edades mayores que 0xF se limitan a 0xF.
//!
//! Formato: tabla de 0xA00 bytes, ocho ranuras (comerciante o 0xFF) y 40 edades por ranura ocupada.

/// Edades de una ranura: `relax_ages`. `distances` es la matriz cuadrada `n × n` (`get_town_distance`, `0x00531B70`).
pub fn relax_ages(ages: &mut [u8], distances: &[u8], n: usize) {
    for b in 0..n {
        if ages[b] == 0 {
            for s in 0..n {
                let v = (distances[b * n + s] >> 4).wrapping_add(1);
                if v < ages[s] {
                    ages[s] = v;
                }
            }
        } else if ages[b] > 0xf {
            ages[b] = 0xf;
        }
    }
}

/// `mark_town_known(town, merchant)` sobre el historial serializado.
pub fn mark_town_known(history: &mut [u8], town: u32, merchant: u32, distances: &[u8], n: usize) -> Result<(), &'static str> {
    if town as usize >= n {
        return Ok(());
    }
    if history.len() < 0xa08 || distances.len() != n * n || n > 40 {
        return Err("historial de ciudades incompatible");
    }
    let slots: [u8; 8] = history[0xa00..0xa08].try_into().unwrap();
    let mut cursor = 0xa08;
    for slot in slots {
        if slot == 0xff {
            continue;
        }
        let ages = history.get_mut(cursor..cursor + 40).ok_or("ranura de historial truncada")?;
        if u32::from(slot) == merchant {
            ages[town as usize] = 0;
            relax_ages(ages, distances, n);
        }
        cursor += 40;
    }
    Ok(())
}

/// `town_info_history_get_known_flags` (`0x00535300`, ECX = `0x006DE588`) sobre el historial serializado: banderas de
/// noticias de la ciudad en la posición `town` tal como las conoce `player` (`DAT_006DFC14`).
///
/// La tabla `+0x000` guarda 16 días de banderas (`u32`) por ciudad, en anillo por `(tick >> 8) & 0xF`. Si el jugador ocupa
/// una ranura, se lee el día de hace «edad» días (`(día + 16 − edad) & 0xF`); si no, el más antiguo (`(día + 1) & 0xF`). El
/// resultado se enmascara con `town_mask` (`Town+0x2C8` de la tabla `0x006DE508`) más los bits `0x7E0000`. En memoria cada
/// ranura `+0xA08 + 4·k` apunta a sus edades; en la forma serializada las ranuras libres (`0xFF`) no las tienen.
pub fn known_flags(history: &[u8], town_mask: u32, tick: u32, player: u32, town: u32) -> Result<u32, &'static str> {
    if history.len() < 0xa08 {
        return Err("historial de ciudades truncado");
    }
    let day = (tick >> 8) & 0xf;
    let mut ages_at = None;
    let mut cursor = 0xa08;
    for k in 0..8 {
        let slot = history[0xa00 + k];
        if u32::from(slot) == player {
            if slot != 0xff {
                ages_at = Some(cursor);
            }
            break;
        }
        if slot != 0xff {
            cursor += 40;
        }
    }
    let idx = match ages_at {
        Some(at) => {
            let age = *history.get(at + town as usize).ok_or("edades del historial truncadas")?;
            (day + 16).wrapping_sub(u32::from(age)) & 0xf
        }
        None => (day + 17) & 0xf,
    };
    let o = (town.wrapping_mul(16).wrapping_add(idx) as usize).checked_mul(4).ok_or("ciudad fuera del historial")?;
    let v = history.get(o..o + 4).ok_or("ciudad fuera del historial")?;
    Ok(u32::from_le_bytes(v.try_into().unwrap()) & (town_mask | 0x7e_0000))
}
