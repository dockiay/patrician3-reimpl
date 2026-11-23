//! Clasificación del acontecimiento urbano en `0x004E0334`.
use crate::town::{off, Town};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Selection {
    pub kind: u8,
    pub detail: u8,
    pub extra: u8,
}

/// La ciudad, los depósitos y las listas se consultan sin mutar entradas.
/// Los comerciantes/instalaciones y oficinas son sus imágenes originales.
/// Rechaza ciclos y accesos fuera de tabla que el original no comprueba.
pub fn classify(
    towns: &[Town],
    city: usize,
    facilities: &[[u8; 20]],
    offices: &[Vec<u8>],
    bands: [Option<u16>; 5],
    facility_byte: u8,
) -> Result<(Town, Option<Selection>), String> {
    let town = towns.get(city).ok_or("ciudad de suceso inexistente")?;
    if offices.iter().any(|v| v.len() != 0x44c) {
        return Err("oficina de suceso truncada".into());
    }
    let mut out = town.clone();
    let sat = |p: usize| town.u16(p) as i16;
    let low = (0..3).all(|j| sat(off::SATISFACTION + 2 * j) < -20)
        && sat(off::SATISFACTION_TARGET) < -20
        && sat(off::SATISFACTION_TARGET + 2) < -22
        && sat(off::SATISFACTION_TARGET + 4) < -24;
    let mut selected = if low {
        Some(Selection {
            kind: 4,
            detail: 0,
            extra: 0,
        })
    } else {
        None
    };
    if !low && sat(off::SATISFACTION) > 10 && sat(off::SATISFACTION + 2) > 20 && sat(off::SATISFACTION + 4) > 20 {
        if let Some((i, _)) = bands.iter().enumerate().find(|(_, v)| v.is_some_and(|x| x > 500)) {
            selected = Some(Selection {
                kind: 2,
                detail: i as u8,
                extra: 255,
            });
        }
    }
    for kind in 4..21usize {
        if town.u16(off::FACILITY + kind * 16 + 8) <= 950 {
            continue;
        }
        let (mut total, mut local) = (0i32, 0i32);
        for f in facilities.iter().rev() {
            if usize::from(f[6]) == kind {
                let workers = i32::from(u16::from_le_bytes([f[4], f[5]]));
                total = total.wrapping_add(workers);
                if f[7] == town.u8(off::TOWN_INDEX) {
                    local = local.wrapping_add(workers)
                }
            }
        }
        if local > 150 && local.wrapping_mul(20) > total {
            out.set_u8(0x6f0, facility_byte);
            selected = Some(Selection {
                kind: 1,
                detail: facility_byte,
                extra: 0,
            });
            break;
        }
    }
    for (slot, ware) in [1usize, 5, 7, 9, 12, 14, 18].into_iter().enumerate() {
        if town.i32(off::TRADE_FLOW_AVERAGE + 4 * ware) <= 0 {
            continue;
        }
        // El original usa el índice de mercancía para la ciudad dentro del
        // bucle, que repite `town_count` veces en vez de iterar las ciudades.
        let other = towns.get(slot).ok_or("índice de ciudad del original fuera de tabla")?;
        let (mut production, mut consumption) = (
            other.i32(off::DAILY_PRODUCTION + 4 * ware),
            other
                .i32(off::BUSINESS_CONSUMPTION + 4 * ware)
                .wrapping_add(other.i32(off::CITIZEN_CONSUMPTION + 4 * ware)),
        );
        let mut at = usize::from(other.u16(off::FIRST_OFFICE));
        let mut seen = vec![false; offices.len()];
        while at < offices.len() {
            if seen[at] {
                return Err("cadena de oficinas urbana cíclica".into());
            }
            seen[at] = true;
            let office = &offices[at];
            production = production.wrapping_add(i32::from_le_bytes(
                office[0xc4 + 4 * ware..0xc8 + 4 * ware].try_into().unwrap(),
            ));
            consumption = consumption.wrapping_add(i32::from_le_bytes(
                office[0x64 + 4 * ware..0x68 + 4 * ware].try_into().unwrap(),
            ));
            at = usize::from(u16::from_le_bytes([office[0x2ca], office[0x2cb]]));
        }
        let count = towns.len() as i32;
        production = production.wrapping_mul(count);
        consumption = consumption.wrapping_mul(count);
        if town.i32(off::DAILY_PRODUCTION + 4 * ware).wrapping_mul(20) > production
            && town.i32(off::TRADE_FLOW_AVERAGE + 4 * ware).wrapping_mul(20) > consumption
        {
            out.set_u8(0x6f0, ware as u8);
            selected = Some(Selection {
                kind: 0,
                detail: ware as u8,
                extra: 0,
            });
            break;
        }
    }
    Ok((out, selected))
}
