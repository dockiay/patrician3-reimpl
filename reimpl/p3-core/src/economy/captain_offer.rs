//! Oferta de capitán 0x28 (`st_captain_offer`, 004DDF40).
use crate::{ai::reputation::Merchant, scheduler::TaskRecord, ships::captains::CaptainPool, town::Town};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Notice {
    pub recipient: u32,
    pub record: [u8; 16],
}
pub trait Host: Clone {
    fn disband(
        &mut self,
        id: u16,
        ships: &mut [u8],
        convoys: &mut [u8],
        towns: &mut [Town],
        pool: &mut CaptainPool,
        merchants: &mut [Merchant],
    ) -> Result<(), String>;
}
#[derive(Clone)]
pub struct Prepared<H> {
    pub task: TaskRecord,
    pub ships: Vec<u8>,
    pub convoys: Vec<u8>,
    pub towns: Vec<Town>,
    pub captains: CaptainPool,
    pub merchants: Vec<Merchant>,
    pub host: H,
    pub notices: Vec<Notice>,
    pub keep: bool,
}
fn r16(b: &[u8], p: usize) -> u16 {
    u16::from_le_bytes([b[p], b[p + 1]])
}
fn w16(b: &mut [u8], p: usize, v: u16) {
    b[p..p + 2].copy_from_slice(&v.to_le_bytes())
}
fn notice(kind: u8, town: u8, given: u8, family: u8, a: u16, b: u16, recipient: u32) -> Notice {
    let mut record = [0u8; 16];
    record[1] = 0x80;
    record[2..4].fill(255);
    record[4] = kind;
    record[5] = town;
    record[6..8].fill(255);
    for (i, v) in [u16::from(given), u16::from(family), a, b].into_iter().enumerate() {
        record[8 + 2 * i..10 + 2 * i].copy_from_slice(&v.to_le_bytes())
    }
    Notice { recipient, record }
}

/// Clona todas las vistas y efectos. Una búsqueda que ciclaría en el x86 se rechaza.
pub fn prepare<H: Host>(
    task: TaskRecord,
    ships: &[u8],
    convoys: &[u8],
    towns: &[Town],
    pool: &CaptainPool,
    merchants: &[Merchant],
    merchant_count: u16,
    host: &H,
) -> Result<Prepared<H>, String> {
    if ships.len() % 0x180 != 0
        || convoys.len() % 0x3c != 0
        || pool.entries.len() != usize::from(pool.capacity) * 16
        || usize::from(merchant_count) > merchants.len()
        || merchants
            .iter()
            .take(usize::from(merchant_count))
            .any(|m| m.bytes.len() != 0x650)
    {
        return Err("tablas de oferta de capitán truncadas".into());
    }
    let mut out = Prepared {
        task,
        ships: ships.to_vec(),
        convoys: convoys.to_vec(),
        towns: towns.to_vec(),
        captains: pool.clone(),
        merchants: merchants.to_vec(),
        host: host.clone(),
        notices: vec![],
        keep: false,
    };
    let offer = out.task.u32_at(0x14);
    let cap = out.task.u32_at(0xc);
    let mut ship = out.task.u32_at(8) as usize;
    let bonus = out.task.u32_at(0x10);
    let count = out.ships.len() / 0x180;
    if offer >= u32::from(merchant_count) || cap >= u32::from(pool.capacity) {
        return Ok(out);
    }
    if ship >= count || u32::from(r16(&out.ships, ship * 0x180 + 0x42)) == cap {
        let found = out.merchants[..usize::from(merchant_count)]
            .iter()
            .rev()
            .map(|m| usize::from(m.u16(0xe)))
            .find(|&i| i < count);
        let Some(id) = found else { return Ok(out) };
        if u32::from(r16(&out.ships, id * 0x180 + 0x42)) != cap {
            return Err("búsqueda de oferta entraría en bucle en el original".into());
        }
        ship = id;
        out.task.set_u32_at(8, id as u32);
    }
    let at = ship * 0x180;
    let ca = cap as usize * 16;
    let dmg = out.ships[at + 0x3f] as i8;
    let cost = ((u32::from(out.captains.entries[ca + 9])
        + u32::from(out.captains.entries[ca + 10])
        + u32::from(out.captains.entries[ca + 11]))
        / 50)
        .wrapping_add(6)
        .wrapping_add((i32::from(dmg) * 2) as u32)
        .wrapping_mul(500)
        .wrapping_sub(((ship as u32 & 2) + (cap & 3)) * 500);
    let current = usize::from(offer as u16);
    let person = &out.merchants[current];
    let town0 = out.ships[at + 0x39];
    let town = if usize::from(town0) < out.towns.len() {
        town0
    } else {
        out.ships[at + 0x13a]
    };
    let given = out.captains.entries[ca + 2];
    let family = out.captains.entries[ca + 3];
    if dmg > 3 || bonus < cost {
        out.notices.push(notice(
            0x4e,
            town,
            given,
            family,
            r16(&out.ships, at + 0x15e),
            u16::from(person.bytes[0x1c]) * 256 + u16::from(person.bytes[0x1d]),
            offer,
        ));
        return Ok(out);
    }
    if (person.i32(0) as u32) < bonus {
        out.notices.push(notice(
            0x4f,
            town,
            given,
            family,
            u16::from(person.bytes[0x1c]),
            u16::from(person.bytes[0x1d]),
            offer,
        ));
        return Ok(out);
    }
    let state = r16(&out.ships, at + 0x134);
    if state > 0x14 {
        return Ok(out);
    }
    let action = [0u8, 0, 1, 2, 3, 3, 3, 4, 5, 1, 6, 6, 4, 4, 4, 1, 1, 6, 6, 1, 1][state as usize];
    match action {
        1 => {
            out.task.due = out.task.due.wrapping_add(0x40);
            out.keep = true;
            return Ok(out);
        }
        2 => {
            out.task.due = out
                .task
                .due
                .wrapping_add(0x41u32.wrapping_sub(u32::from(r16(&out.ships, at + 0x138))));
            out.keep = true;
            return Ok(out);
        }
        6 => return Ok(out),
        0 | 3 => {
            let id = r16(&out.ships, at + 8);
            let j = usize::from(id) * 0x3c;
            if j < out.convoys.len() && usize::from(r16(&out.convoys, j + 0x10)) == ship {
                if action == 3 {
                    out.task.due = out.task.due.wrapping_add(0x20);
                    out.keep = true;
                    return Ok(out);
                }
                out.host.disband(
                    id,
                    &mut out.ships,
                    &mut out.convoys,
                    &mut out.towns,
                    &mut out.captains,
                    &mut out.merchants,
                )?
            }
        }
        _ => {}
    }
    let city = usize::from(out.ships[at + 0x39]);
    if city >= out.towns.len() {
        return Err("ciudad del capitán ausente".into());
    }
    let old_owner = usize::from(out.ships[at]);
    if old_owner >= usize::from(merchant_count) {
        return Err("dueño del barco ausente".into());
    }
    if action == 5 {
        w16(&mut out.ships, at + 0x134, 0);
        out.towns[city].set_u16(0x994, u16::MAX);
        out.towns[city].set_u32(0x990, 0)
    }
    let person = &mut out.merchants[current];
    person.set_i32(0x4bc, person.i32(0x4bc).wrapping_add(bonus as i32));
    person.set_i32(0, person.i32(0).wrapping_sub(bonus as i32));
    out.captains.entries[ca + 15] = offer as u8;
    let slot = r16(&out.ships, at + 0x42);
    if slot < out.captains.capacity {
        let idx = usize::from(slot) * 16;
        let next = r16(&out.captains.entries, idx);
        w16(&mut out.ships, at + 0x42, next);
        let head = out.towns[city].u16(0x82e);
        w16(&mut out.captains.entries, idx, head);
        out.towns[city].set_u16(0x82e, slot)
    }
    let person = &out.merchants[current];
    out.notices.push(notice(
        0x51,
        town0,
        given,
        family,
        u16::from(person.bytes[0x1c]),
        u16::from(person.bytes[0x1d]),
        offer,
    ));
    let owner = &out.merchants[old_owner];
    out.notices.push(notice(
        0x50,
        town0,
        given,
        family,
        r16(&out.ships, at + 0x15e),
        u16::from(owner.bytes[0x1c]) * 256 + u16::from(owner.bytes[0x1d]),
        old_owner as u32,
    ));
    out.task.kind = 0x29;
    out.task.due = out.task.due.wrapping_add(0xe00);
    out.task.set_u32_at(8, city as u32);
    out.task.set_u32_at(0x10, 1);
    out.keep = true;
    Ok(out)
}
