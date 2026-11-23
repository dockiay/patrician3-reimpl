//! Operaciones de habilidad y jubilación de capitanes usadas por C10.
use crate::{ai::reputation::Merchant, pump::Operation};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SkillNotice {
    pub merchant: u8,
    pub town: u8,
    pub look_a: u16,
    pub look_b: u16,
    pub ship_name: u16,
    pub wage: u16,
}
fn r16(b: &[u8], o: usize) -> u16 {
    u16::from_le_bytes([b[o], b[o + 1]])
}

/// `0x00538A80`, operación 0x12. Los avisos se entregan aparte mediante el depósito.
pub fn captain_skill_gain(
    op: Operation,
    ships: &[u8],
    captains: &mut [u8],
    merchants: &[Merchant],
    merchant_count: u16,
    town_count: usize,
) -> Result<Option<SkillNotice>, &'static str> {
    if ships.len() % 0x180 != 0 || captains.len() % 16 != 0 || usize::from(merchant_count) > merchants.len() {
        return Err("tablas de capitán truncadas");
    }
    let ship = op.0[1] as usize;
    if ship >= ships.len() / 0x180 {
        return Ok(None);
    }
    let image = &ships[ship * 0x180..(ship + 1) * 0x180];
    let id = r16(image, 0x42);
    let ci = usize::from(id);
    if ci >= captains.len() / 16 {
        return Ok(None);
    }
    let owner = usize::from(image[0]);
    if owner < usize::from(merchant_count) && merchants[owner].bytes.len() < 0x1a {
        return Err("comerciante truncado");
    }
    let person = &mut captains[ci * 16..(ci + 1) * 16];
    let old_wage = r16(person, 0xc);
    let max = [250u8, 200, 250, 150];
    for (offset, inc, limit) in [
        (9, op.0[2] as u8, max[usize::from(id & 3)]),
        (10, op.0[3] as u8, max[usize::from((id >> 2) & 3)]),
        (11, op.0[3] as u8, max[usize::from((id >> 4) & 3)]),
    ] {
        let old = person[offset];
        let sum = old.wrapping_add(inc);
        person[offset] = if sum > limit || sum < old { limit } else { sum };
    }
    let wage =
        (u16::from(person[9]) + u16::from(person[10]) + u16::from(person[11])) / 50 + u16::from(person[8] % 11) + 10;
    person[0xc..0xe].copy_from_slice(&wage.to_le_bytes());
    if wage <= old_wage || owner >= usize::from(merchant_count) || merchants[owner].u16(8) != 0 {
        return Ok(None);
    }
    let town = if usize::from(image[0x39]) < town_count {
        image[0x39]
    } else {
        merchants[owner].bytes[0x19]
    };
    Ok(Some(SkillNotice {
        merchant: owner as u8,
        town,
        look_a: u16::from(person[2]),
        look_b: u16::from(person[3]),
        ship_name: r16(image, 0x15e),
        wage,
    }))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RetirementNotice {
    pub merchant: u8,
    pub town: u8,
    pub look_a: u16,
    pub look_b: u16,
    pub ship_name: u16,
    pub merchant_name: u16,
}

/// `0x00538C40`, operación 0x13: programa la jubilación y marca al capitán.
pub fn captain_retirement(
    op: Operation,
    ships: &[u8],
    captains: &mut [u8],
    merchants: &[Merchant],
    merchant_count: u16,
    town_count: usize,
    tick: u32,
    scheduler: &mut crate::scheduler::Scheduler,
) -> Result<Option<RetirementNotice>, &'static str> {
    if ships.len() % 0x180 != 0 || captains.len() % 16 != 0 || usize::from(merchant_count) > merchants.len() {
        return Err("tablas de capitán truncadas");
    }
    let ship = op.0[1] as usize;
    let cap = op.0[2] as usize;
    if ship >= ships.len() / 0x180 || cap >= captains.len() / 16 {
        return Ok(None);
    }
    let image = &ships[ship * 0x180..(ship + 1) * 0x180];
    let owner = usize::from(image[0]);
    if owner < usize::from(merchant_count) && merchants[owner].bytes.len() < 0x1e {
        return Err("comerciante truncado");
    }
    if scheduler.free_head >= scheduler.capacity && scheduler.capacity > u16::MAX - 0x80 {
        return Err("planificador lleno");
    }
    let id = scheduler.add_task(tick.wrapping_add(1));
    let task = &mut scheduler.records[usize::from(id)];
    task.kind = 0x27;
    task.set_u32_at(8, op.0[1]);
    task.set_u32_at(0xc, op.0[2]);
    let person = &mut captains[cap * 16..(cap + 1) * 16];
    person[0xe] = 1;
    if owner >= usize::from(merchant_count) || merchants[owner].u16(8) != 0 {
        return Ok(None);
    }
    let m = &merchants[owner];
    let town = if usize::from(image[0x39]) < town_count {
        image[0x39]
    } else {
        m.bytes[0x19]
    };
    Ok(Some(RetirementNotice {
        merchant: owner as u8,
        town,
        look_a: u16::from(person[2]),
        look_b: u16::from(person[3]),
        ship_name: r16(image, 0x15e),
        merchant_name: u16::from(m.bytes[0x1c]) << 8 | u16::from(m.bytes[0x1d]),
    }))
}

/// Disolución de convoy de C07, necesaria para jubilar a un líder en puerto.
pub trait RetirementHost {
    fn disband_convoy(
        &mut self,
        convoy: u16,
        ships: &mut [u8],
        convoys: &mut [u8],
        towns: &mut [crate::town::Town],
        pool: &mut crate::ships::captains::CaptainPool,
    ) -> Result<(), String>;
}

/// `0x004DDC00`: espera una maniobra segura y libera al capitán mediante C07.
/// El original puede quedar en un bucle al buscar un capitán reasignado; se rechaza esa imagen.
pub fn retirement_tick<H: RetirementHost>(
    task: &mut crate::scheduler::TaskRecord,
    ships: &mut [u8],
    convoys: &mut [u8],
    towns: &mut [crate::town::Town],
    pool: &mut crate::ships::captains::CaptainPool,
    merchants: &[Merchant],
    host: &mut H,
) -> Result<bool, String> {
    let mut staged_task = *task;
    let mut staged_ships = ships.to_vec();
    let mut staged_convoys = convoys.to_vec();
    let mut staged_towns = towns.to_vec();
    let mut staged_pool = pool.clone();
    let keep = retirement_tick_inner(
        &mut staged_task,
        &mut staged_ships,
        &mut staged_convoys,
        &mut staged_towns,
        &mut staged_pool,
        merchants,
        host,
    )?;
    *task = staged_task;
    ships.copy_from_slice(&staged_ships);
    convoys.copy_from_slice(&staged_convoys);
    towns.clone_from_slice(&staged_towns);
    *pool = staged_pool;
    Ok(keep)
}
fn retirement_tick_inner<H: RetirementHost>(
    task: &mut crate::scheduler::TaskRecord,
    ships: &mut [u8],
    convoys: &mut [u8],
    towns: &mut [crate::town::Town],
    pool: &mut crate::ships::captains::CaptainPool,
    merchants: &[Merchant],
    host: &mut H,
) -> Result<bool, String> {
    if ships.len() % 0x180 != 0
        || convoys.len() % 0x3c != 0
        || pool.entries.len() != usize::from(pool.capacity) * 16
        || merchants.iter().any(|m| m.bytes.len() < 0x10)
    {
        return Err("tablas de jubilación truncadas".into());
    }
    let n = ships.len() / 0x180;
    let nc = convoys.len() / 0x3c;
    let target = task.u32_at(0xc);
    if target >= u32::from(pool.capacity) {
        return Ok(false);
    }
    let mut chosen = task.u32_at(8) as usize;
    if chosen >= n || u32::from(r16(ships, chosen * 0x180 + 0x42)) != target {
        let mut found = None;
        for m in merchants.iter().rev() {
            let id = usize::from(m.u16(0xe));
            if id >= n {
                continue;
            }
            if u32::from(r16(ships, id * 0x180 + 0x42)) != target {
                return Err("búsqueda de capitán entraría en bucle en el original".into());
            }
            found = Some(id);
            break;
        }
        let Some(id) = found else { return Ok(false) };
        chosen = id;
        task.set_u32_at(8, id as u32);
    }
    let at = chosen * 0x180;
    let state = r16(ships, at + 0x134);
    if state > 0x14 {
        return Ok(false);
    }
    let action = [0u8, 0, 1, 2, 3, 3, 3, 4, 5, 1, 6, 6, 4, 4, 4, 1, 1, 6, 6, 1, 1][usize::from(state)];
    match action {
        1 => {
            task.due = task.due.wrapping_add(0x40);
            return Ok(true);
        }
        2 => {
            task.due = task
                .due
                .wrapping_add(0x41u32.wrapping_sub(u32::from(r16(ships, at + 0x138))));
            return Ok(true);
        }
        6 => return Ok(false),
        3 => {
            let convoy = usize::from(r16(ships, at + 8));
            if convoy < nc && usize::from(r16(convoys, convoy * 0x3c + 0x10)) == chosen {
                task.due = task.due.wrapping_add(0x20);
                return Ok(true);
            }
        }
        0 => {
            let convoy = usize::from(r16(ships, at + 8));
            if convoy < nc && usize::from(r16(convoys, convoy * 0x3c + 0x10)) == chosen {
                host.disband_convoy(convoy as u16, ships, convoys, towns, pool)?;
            }
        }
        _ => {}
    }
    let mut cursor = pool.free_head;
    let mut seen = vec![false; usize::from(pool.capacity)];
    while cursor < pool.capacity {
        let i = usize::from(cursor);
        if seen[i] {
            return Err("lista libre de capitanes cíclica".into());
        }
        seen[i] = true;
        cursor = r16(&pool.entries, i * 16);
    }
    if action == 5 && usize::from(ships[at + 0x39]) >= towns.len() {
        return Err("ciudad de jubilación ausente".into());
    }
    let mut slot = r16(ships, at + 0x42);
    pool.free(&mut slot);
    ships[at + 0x42..at + 0x44].copy_from_slice(&slot.to_le_bytes());
    if action == 5 {
        ships[at + 0x134..at + 0x136].fill(0);
        let town = &mut towns[usize::from(ships[at + 0x39])];
        town.set_u16(0x994, u16::MAX);
        town.set_u32(0x990, 0);
    }
    Ok(false)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdministratorNotice {
    pub merchant: u16,
    pub town: u8,
    pub combined_wage: u16,
    pub look_a: u16,
    pub look_b: u16,
    pub office: usize,
}

/// `0x0053DDF0`, operación 0x67: mejora un administrador y prepara su aviso.
pub fn administrator_skill_gain(
    op: Operation,
    towns: &[crate::town::Town],
    offices: &[Vec<u8>],
    captains: &mut [u8],
    merchant_count: u16,
) -> Result<Option<AdministratorNotice>, &'static str> {
    if captains.len() % 16 != 0 || offices.iter().any(|o| o.len() < 0x2f4) {
        return Err("tablas de administrador truncadas");
    }
    let cap = op.0[1] as usize;
    let owner = op.0[2] as usize;
    let town = op.0[3] as usize;
    if cap >= captains.len() / 16 || owner >= usize::from(merchant_count) || town >= towns.len() {
        return Ok(None);
    }
    let mut office = usize::from(towns[town].u16(0x784));
    let mut seen = vec![false; offices.len()];
    let found = loop {
        if office >= offices.len() {
            break None;
        }
        if seen[office] {
            return Err("lista de oficinas cíclica");
        }
        seen[office] = true;
        if usize::from(r16(&offices[office], 0x2c4)) == owner {
            break Some(office);
        }
        office = usize::from(r16(&offices[office], 0x2ca));
    };
    let Some(index) = found else { return Ok(None) };
    let office = &offices[index];
    if usize::from(r16(office, 0x2f2)) != cap {
        return Ok(None);
    }
    let person = &mut captains[cap * 16..(cap + 1) * 16];
    let skill = op.0[4] as u8;
    person[0xa] = skill;
    let wage = (u16::from(skill) / 43) * 20 + 10;
    person[0xc..0xe].copy_from_slice(&wage.to_le_bytes());
    Ok(Some(AdministratorNotice {
        merchant: owner as u16,
        town: town as u8,
        combined_wage: wage.wrapping_add(r16(office, 0x2d2)),
        look_a: u16::from(person[2]),
        look_b: u16::from(person[3]),
        office: index,
    }))
}
