//! Static port of all sixteen investigation crimes in 0x004E5B84.
//! CRT state is supplied by the caller. Letters, fines, goods, reputation
//! and the temporary trading prohibition are calculated before publication.
use p3_core::economy::hanse_expedition::InvestigationVerdict;
use p3_core::{ai::reputation::Merchant, calendar::Calendar, scheduler::TaskRecord, town::Town};
pub fn supports(crime: u8) -> bool {
    crime <= 15
}
pub fn office_city(crime: u8, court: u8) -> Option<usize> {
    (!matches!(crime, 5..=7 | 15)).then_some(usize::from(court))
}
fn victim(r: &TaskRecord, merchants: &[Merchant], fallback: bool) -> Option<[u8; 2]> {
    let id = usize::from(r.u8_at(10));
    let id = if fallback && id >= merchants.len() {
        usize::from(r.u8_at(9))
    } else {
        id
    };
    let m = merchants.get(id)?;
    Some([*m.bytes.get(0x1c)?, *m.bytes.get(0x1d)?])
}
pub fn begin(
    r: &mut TaskRecord,
    town_count: usize,
    merchants: &[Merchant],
    random: [u16; 2],
) -> Option<Option<(usize, u8, [u8; 8])>> {
    let crime = r.u8_at(16);
    if r.kind != 5 || !supports(crime) || r.u8_at(17) != 0 {
        return None;
    }
    let owner = usize::from(r.u8_at(8));
    let court = r.u8_at(11);
    if usize::from(r.u8_at(9)) >= town_count || usize::from(court) >= town_count {
        return None;
    }
    let m = merchants.get(owner)?;
    if m.bytes.len() < 0x1e {
        return None;
    }
    let other = if crime == 3 {
        Some(victim(r, merchants, true)?)
    } else {
        None
    };
    let message = if m.u16(8) == 0 {
        let mut p = [0; 8];
        if matches!(crime, 5..=8) {
            p[0] = m.bytes[0x1c];
            p[2] = m.bytes[0x1d];
            p[4] = crime
        } else {
            p[0] = m.bytes[0x1c];
            p[1] = m.bytes[0x1d];
            p[2] = crime;
            p[3] = if matches!(crime, 2 | 9..=12) { court } else { r.u8_at(9) };
            if let Some(names) = other {
                p[4..6].copy_from_slice(&names)
            } else if matches!(crime, 2 | 9..=15) {
                p[4] = r.u8_at(22);
                p[5] = r.u8_at(23)
            }
        }
        Some((owner, court, p))
    } else {
        None
    };
    r.set_u8_at(17, 1);
    for (i, n) in random.into_iter().enumerate() {
        r.set_u8_at(18 + 2 * i, n as u8);
        r.set_u8_at(19 + 2 * i, (n >> 8) as u8)
    }
    r.due = r.due.wrapping_add(((r.due & 7) + 8) * 256) | 0x80;
    Some(message)
}
pub fn publish_indictment(
    pool: &mut Vec<u8>,
    m: &mut Merchant,
    date: Calendar,
    town: u8,
    payload: [u8; 8],
    enabled: bool,
    crime: u8,
) -> Result<bool, String> {
    if !enabled {
        return Ok(false);
    }
    let mut r = [0; 16];
    r[4] = if matches!(crime, 5..=8) { 10 } else { 25 };
    r[5] = town;
    r[8..].copy_from_slice(&payload);
    p3_state::messages::post_to_merchant(
        pool,
        m,
        date.day,
        date.month,
        date.year,
        &r,
        p3_state::messages::Payload::None,
        &p3_state::messages::RETENTION,
    )
}
pub fn verdict(
    r: &mut TaskRecord,
    towns: &mut [Town],
    merchants: &mut [Merchant],
    tick: u32,
    mut stock: Option<&mut [i32; 24]>,
) -> Option<InvestigationVerdict> {
    let crime = r.u8_at(16);
    if r.kind != 5 || !supports(crime) || r.u8_at(17) == 0 {
        return None;
    }
    let owner = usize::from(r.u8_at(8));
    let target = usize::from(r.u8_at(9));
    let court = usize::from(r.u8_at(11));
    let m = merchants.get(owner)?;
    if m.bytes.len() < 0x650 || court >= towns.len() || target >= towns.len() {
        return None;
    }
    let home = usize::from(m.bytes[0x19]);
    if home >= towns.len() {
        return None;
    }
    let flags = towns[home].flags();
    let special = flags & 0x20000 != 0;
    let ware = ((flags >> 18) & 31) as usize;
    if special && (ware >= 24 || towns.len() > 255) {
        return None;
    }
    let mut cleared = Vec::new();
    for city in [court, target] {
        if cleared.len() >= 2 {
            break;
        }
        for k in 0..4 {
            if towns[city].u8(0x6dc + k) == owner as u8 && !cleared.contains(&(city, k)) {
                cleared.push((city, k))
            }
        }
    }
    let roll = u16::from_le_bytes([r.u8_at(18), r.u8_at(19)]);
    let acquit = cleared.len() > 1 || (cleared.len() == 1 && roll <= 20000);
    let human = m.u16(8) == 0;
    let other = if crime == 3 && human {
        Some(victim(r, merchants, !acquit)?)
    } else {
        None
    };
    let m = &mut merchants[owner];
    let mut changed = Vec::new();
    for (city, k) in cleared {
        towns[city].set_u8(0x6dc + k, 255);
        if !changed.contains(&city) {
            changed.push(city)
        }
    }
    if acquit {
        let letter = human.then(|| {
            let mut p = [0; 8];
            p[0] = m.bytes[0x1d];
            p[1] = m.bytes[0x1c];
            if let Some(names) = other {
                p[2] = names[1];
                p[3] = names[0]
            }
            p[4] = target as u8;
            p[6] = if crime < 9 { crime } else { 2 };
            (owner, court as u8, 0x4a, p)
        });
        return Some(InvestigationVerdict {
            keep: false,
            letter,
            notify: false,
            towns: changed,
        });
    }
    let random = u16::from_le_bytes([r.u8_at(20), r.u8_at(21)]);
    let factor = match crime {
        0 | 9 | 10 | 13 => random % 11 + 10,
        1 | 8 | 11 => random % 21 + 30,
        2 | 5..=7 => random % 7 + 2,
        3 | 4 | 12 | 14 => random % 21 + 60,
        15 => (random & 7) + 10,
        _ => unreachable!(),
    };
    if !matches!(crime, 5..=7) {
        m.subtract_social_reputation_event(court as u32, u32::from(crime))
    }
    if matches!(crime, 0 | 3 | 13..=15) {
        m.subtract_social_reputation_event(target as u32, u32::from(crime))
    } else if crime == 8 {
        m.subtract_social_reputation_event(home as u32, u32::from(crime))
    }
    if crime == 15 {
        r.set_u8_at(17, 3)
    }
    let value = i64::from(m.i32(0x46c).max(5000));
    let raw = ((value * i64::from(factor) / 500000 + 1) as i32).wrapping_mul(500);
    let fine = raw.min(32750000) as u32;
    let mut amount = fine;
    let mut sanction_code = factor as u8;
    if r.u8_at(17) == 1 && special {
        sanction_code = ware as u8;
        let unit = if p3_core::economy::prices::SMALL_LOT_WARES[ware] {
            200
        } else {
            2000
        };
        let raw = p3_core::numeric::ftol_i32(
            f64::from(fine as i32 as f32) / f64::from(p3_core::economy::prices::BASE_PRICE[ware]),
        ) as u32;
        let quantity = raw - raw % unit;
        if let Some(stock) = stock.as_deref_mut() {
            if quantity <= stock[ware] as u32 {
                stock[ware] = stock[ware].wrapping_sub(quantity as i32);
                r.set_u8_at(17, 2);
                amount = quantity / unit
            }
        }
    }
    let phase = r.u8_at(17);
    let letter = human.then(|| {
        let mut p = [0; 8];
        p[0] = phase.wrapping_sub(1);
        match phase {
            1 => p[2..4].copy_from_slice(&((amount as i32 / 500) as u16).to_le_bytes()),
            2 => {
                p[2] = sanction_code;
                p[3] = amount as u8
            }
            3 => {
                p[2] = target as u8;
                p[3] = factor as u8
            }
            _ => {}
        }
        p[4] = crime;
        p[5] = m.bytes[0x1c];
        p[6] = m.bytes[0x1d];
        if matches!(crime, 0 | 1) {
            p[7] = target as u8
        } else if let Some(names) = other {
            p[1] = names[0];
            p[7] = names[1]
        }
        (owner, court as u8, 0x4b, p)
    });
    let keep = match phase {
        1 => {
            r.due = r.due.wrapping_add(0x1400);
            r.kind = 1;
            r.set_u32_at(8, fine & 0x7fffffff);
            r.set_u32_at(12, tick & 0xffffff00);
            r.set_u8_at(16, 255);
            r.set_u8_at(17, owner as u8);
            r.set_u8_at(18, 0);
            r.set_u8_at(19, 0);
            r.set_u8_at(20, court as u8);
            r.set_u8_at(21, 0);
            r.set_u8_at(22, 0);
            r.set_u8_at(23, 0);
            true
        }
        3 => {
            m.set_i32(0x608, m.i32(0x608) | (1i32.wrapping_shl(target as u32)));
            r.kind = 0x37;
            r.set_u32_at(8, target as u32);
            r.set_u32_at(12, owner as u32);
            r.due = r.due.wrapping_add(u32::from(factor) * 7 * 256);
            true
        }
        _ => false,
    };
    Some(InvestigationVerdict {
        keep,
        letter,
        notify: human,
        towns: changed,
    })
}
