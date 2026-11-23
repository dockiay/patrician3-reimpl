//! Avance de incendio de edificio público (`st_building_burn_progress`, 0x004E1E54).
use super::{tables::RECORD_BYTE_6, Site};
use crate::{
    scheduler::{Scheduler, TaskRecord, FREE},
    town::{off, Town},
};

/// `0x004DB100`: retira el edificio de la primera subasta coincidente y libera la tarea si queda vacía.
pub fn remove_from_auction(
    scheduler: &mut Scheduler,
    towns: &[Town],
    city: u8,
    target: u16,
) -> Result<(), &'static str> {
    let Some(town) = towns.get(usize::from(city)) else {
        return Ok(());
    };
    let count = town.u16(off::BUILDING_RECORD_COUNT);
    if target >= count {
        return Ok(());
    }
    if scheduler.records.len() != usize::from(scheduler.capacity) {
        return Err("planificador truncado");
    }
    let mut staged = scheduler.clone();
    let mut cursor = staged.head;
    let mut previous: Option<usize> = None;
    let mut seen = vec![false; usize::from(staged.capacity)];
    while cursor < staged.capacity {
        let i = usize::from(cursor);
        if seen[i] {
            return Err("lista de tareas cíclica");
        }
        seen[i] = true;
        let entry = staged.records[i];
        if entry.kind == 0x36 && entry.u8_at(0x15) == city {
            let mut words = std::array::from_fn::<_, 3, _>(|j| {
                u16::from_le_bytes([entry.payload[2 * j], entry.payload[2 * j + 1]])
            });
            let mut found = false;
            for j in 0..3 {
                if found {
                    words[j - 1] = words[j]
                } else if words[j] & 0x7fff == target {
                    found = true
                }
            }
            if found {
                words[2] = u16::MAX;
                for (j, v) in words.into_iter().enumerate() {
                    staged.records[i].payload[2 * j..2 * j + 2].copy_from_slice(&v.to_le_bytes());
                }
                if words[0] & 0x7fff >= count {
                    if let Some(prev) = previous {
                        staged.records[prev].next = entry.next
                    } else {
                        staged.head = entry.next
                    }
                    staged.records[i].kind = FREE;
                    staged.records[i].next = staged.free_head;
                    staged.free_head = cursor;
                }
                *scheduler = staged;
                return Ok(());
            }
        }
        previous = Some(i);
        cursor = entry.next;
    }
    Ok(())
}

/// `0x004E1B84`: derrumbe de pieza defensiva de una ciudad, sin vista ni guarida pirata.
pub fn tick_ruin(task: &mut TaskRecord, town: &mut Town, site: &mut Site, special: u8) -> Result<bool, &'static str> {
    if task.kind != 0x25 || town.index() != task.u8_at(0x10) {
        return Err("tarea o ciudad incompatibles");
    }
    let count = usize::from(town.u16(off::BUILDING_RECORD_COUNT));
    if site.records.len() < count * 8 {
        return Err("registros truncados");
    }
    let (mut r, mut t, mut s) = (*task, town.clone(), site.clone());
    if r.u8_at(0x11) != 0 {
        r.set_u8_at(0x11, 0);
        r.payload[2..4].copy_from_slice(&100i16.to_le_bytes());
        r.payload[..2].fill(0xff);
        let x = u16::from_le_bytes([r.payload[4], r.payload[5]]);
        let y = u16::from_le_bytes([r.payload[6], r.payload[7]]);
        let mut cursor = t.u16(off::DEFENSE_HEAD);
        let mut previous: Option<usize> = None;
        let mut seen = vec![false; count];
        while usize::from(cursor) < count {
            let i = usize::from(cursor);
            if seen[i] {
                return Err("lista defensiva cíclica");
            }
            seen[i] = true;
            if u16::from(s.rec(i, 0)) == x && u16::from(s.rec(i, 1)) == y {
                break;
            }
            previous = Some(i);
            cursor = s.next(i);
        }
        if usize::from(cursor) >= count {
            *task = r;
            return Ok(false);
        }
        let i = usize::from(cursor);
        r.payload[..2].copy_from_slice(&cursor.to_le_bytes());
        let successor = s.next(i);
        if let Some(prev) = previous {
            s.set_next(prev, successor)
        } else {
            t.set_u16(off::DEFENSE_HEAD, successor)
        }
        s.set_next(i, t.u16(off::BURNING_HEAD));
        t.set_u16(off::BURNING_HEAD, cursor);
        let work = s.rec(i, 6);
        if work != 0 {
            let base = *RECORD_BYTE_6.get(usize::from(s.rec(i, 3))).ok_or("tipo inválido")?;
            if base == 0 {
                return Err("división original por cero");
            };
            let value = 100i16 + ((i32::from(work) * -50) / i32::from(base)) as i16;
            r.payload[2..4].copy_from_slice(&value.to_le_bytes());
        }
    }
    let target = usize::from(u16::from_le_bytes([r.payload[0], r.payload[1]]));
    if target >= count {
        return Err("edificio de derrumbe ausente");
    }
    let progress = i16::from_le_bytes([r.payload[2], r.payload[3]]);
    if progress > 0 {
        let value = progress - if progress < 51 { 1 } else { 51 };
        r.payload[2..4].copy_from_slice(&value.to_le_bytes());
        s.set_rec(target, 6, value as u8);
        r.due = r.due.wrapping_add(5);
        *task = r;
        *town = t;
        *site = s;
        return Ok(true);
    }
    let mut cursor = t.u16(off::BURNING_HEAD);
    let mut previous: Option<usize> = None;
    let mut seen = vec![false; count];
    while usize::from(cursor) < count && usize::from(cursor) != target {
        let i = usize::from(cursor);
        if seen[i] {
            return Err("lista de incendios cíclica");
        }
        seen[i] = true;
        previous = Some(i);
        cursor = s.next(i);
    }
    if usize::from(cursor) >= count {
        *task = r;
        *town = t;
        *site = s;
        return Ok(false);
    }
    let successor = s.next(target);
    if let Some(prev) = previous {
        s.set_next(prev, successor)
    } else {
        t.set_u16(off::BURNING_HEAD, successor)
    }
    let ty = s.rec(target, 3);
    if matches!(ty, 0x32 | 0x33) {
        s.set_rec(target, 3, 0x36);
        s.set_rec(target, 6, 0);
        s.set_next(target, t.u16(off::DEFENSE_HEAD));
        t.set_u16(off::DEFENSE_HEAD, target as u16);
    } else {
        let rows = t.i32(off::PLAN_ROWS);
        let stride = t.i32(off::PLAN_STRIDE);
        let row = i32::from(s.rec(target, 0));
        let col = i32::from(s.rec(target, 1));
        if rows <= 32
            || stride <= 32
            || row < 16
            || col < 16
            || row >= rows - 16
            || col >= stride - 16
            || (rows as usize)
                .checked_mul(stride as usize)
                .is_none_or(|n| n > s.cells.len())
            || usize::from(ty) >= 60
        {
            return Err("huella de defensa fuera del plano admitido");
        }
        s.set_next(target, u16::MAX);
        s.free_building_record_chain(&mut t, target as u32, special);
    }
    *task = r;
    *town = t;
    *site = s;
    Ok(false)
}

/// Ruta sin vista y sin subasta aplicable. Rechaza edificios privados y planos no acreditados.
/// Actúa sobre copias: un error no cambia la tarea, la ciudad ni el plano.
pub fn tick_public(
    task: &mut TaskRecord,
    town: &mut Town,
    site: &mut Site,
    merchants: u16,
    special: u8,
) -> Result<bool, &'static str> {
    tick(task, town, site, merchants, special, None)
}

/// La oficina, si corresponde a un dueño privado, contiene ambas cabezas en +0x2CE/+0x2D0.
pub fn tick(
    task: &mut TaskRecord,
    town: &mut Town,
    site: &mut Site,
    merchants: u16,
    special: u8,
    mut office: Option<&mut [u8]>,
) -> Result<bool, &'static str> {
    if task.kind != 0x26 || town.index() != task.u8_at(0x10) {
        return Err("tarea o ciudad incompatibles");
    }
    let count = usize::from(town.u16(off::BUILDING_RECORD_COUNT));
    if site.records.len() < count * 8 {
        return Err("registros truncados");
    }
    let target = usize::from(u16::from_le_bytes([task.u8_at(8), task.u8_at(9)]));
    if target >= count {
        return Err("edificio ausente");
    }
    let mut t = town.clone();
    let mut s = site.clone();
    let mut r = *task;
    let mut office_data = office.as_ref().map(|o| o.to_vec());
    macro_rules! finish {
        ($result:expr) => {{
            *task = r;
            *town = t;
            *site = s;
            if let (Some(dst), Some(src)) = (office.as_deref_mut(), office_data.as_deref()) {
                dst.copy_from_slice(src);
            }
            return Ok($result);
        }};
    }
    let ty = s.rec(target, 3);
    if r.u8_at(0x11) != 0 {
        r.set_u8_at(0x11, 0);
        r.payload[2..4].copy_from_slice(&100i16.to_le_bytes());
        if matches!(ty, 0x28 | 0x32 | 0x33) {
            *task = r;
            return Ok(false);
        }
        let owner = s.rec(target, 2);
        r.payload[4..6].copy_from_slice(&u16::from(s.rec(target, 0)).to_le_bytes());
        r.payload[6..8].copy_from_slice(&u16::from(s.rec(target, 1)).to_le_bytes());
        let private = u16::from(owner) < merchants && ty != 0x29;
        let heads = if private {
            let office = office_data.as_ref().ok_or("oficina privada ausente")?;
            if office.len() < 0x2d2 {
                return Err("oficina privada truncada");
            }
            vec![
                u16::from_le_bytes([office[0x2ce], office[0x2cf]]),
                u16::from_le_bytes([office[0x2d0], office[0x2d1]]),
            ]
        } else {
            vec![t.u16(off::PUBLIC_BUILDING_HEAD)]
        };
        let mut found = None;
        for (head, mut index) in heads.into_iter().enumerate() {
            let mut previous = None;
            let mut seen = vec![false; count];
            while usize::from(index) < count && usize::from(index) != target {
                if seen[usize::from(index)] {
                    return Err("lista de edificios cíclica");
                }
                seen[usize::from(index)] = true;
                previous = Some(usize::from(index));
                index = s.next(usize::from(index));
            }
            if usize::from(index) == target {
                found = Some((head, previous));
                break;
            }
        }
        let Some((head, previous)) = found else {
            if private {
                finish!(false)
            } else {
                return Err("edificio ausente de lista pública");
            }
        };
        let successor = s.next(target);
        if let Some(previous) = previous {
            s.set_next(previous, successor);
        } else if private {
            let at = 0x2ce + 2 * head;
            office_data.as_mut().unwrap()[at..at + 2].copy_from_slice(&successor.to_le_bytes());
        } else {
            t.set_u16(off::PUBLIC_BUILDING_HEAD, successor);
        }
        s.set_next(target, t.u16(off::BURNING_HEAD));
        t.set_u16(off::BURNING_HEAD, target as u16);
        let work = s.rec(target, 6);
        if work != 0 {
            let base = *RECORD_BYTE_6
                .get(usize::from(ty))
                .ok_or("tipo de edificio no admitido")?;
            if base == 0 {
                return Err("división original por cero");
            }
            let progress = 100i16 + ((i32::from(work) * -50) / i32::from(base)) as i16;
            r.payload[2..4].copy_from_slice(&progress.to_le_bytes());
        }
    }
    if t.u32(off::TOWN_FLAGS) & 0x8000 != 0 {
        finish!(false);
    }
    let progress = i16::from_le_bytes([r.payload[2], r.payload[3]]);
    if progress > 0 {
        let next = progress - if progress < 51 { 1 } else { 51 };
        r.payload[2..4].copy_from_slice(&next.to_le_bytes());
        s.set_rec(target, 6, next as u8);
        r.due = r.due.wrapping_add(5);
        finish!(true);
    }
    let mut index = t.u16(off::BURNING_HEAD);
    let mut previous = None;
    let mut seen = vec![false; count];
    while usize::from(index) < count && usize::from(index) != target {
        if seen[usize::from(index)] {
            return Err("lista de incendios cíclica");
        }
        seen[usize::from(index)] = true;
        previous = Some(usize::from(index));
        index = s.next(usize::from(index));
    }
    if usize::from(index) != target {
        finish!(false);
    }
    let rows = t.i32(off::PLAN_ROWS);
    let stride = t.i32(off::PLAN_STRIDE);
    let row = i32::from(s.rec(target, 0));
    let col = i32::from(s.rec(target, 1));
    if rows <= 32
        || stride <= 32
        || row < 16
        || col < 16
        || row >= rows - 16
        || col >= stride - 16
        || (rows as usize)
            .checked_mul(stride as usize)
            .is_none_or(|n| n > s.cells.len())
        || usize::from(ty) >= 60
    {
        return Err("huella de edificio fuera del plano admitido");
    }
    let successor = s.next(target);
    if let Some(previous) = previous {
        s.set_next(previous, successor);
    } else {
        t.set_u16(off::BURNING_HEAD, successor);
    }
    s.set_next(target, u16::MAX);
    s.free_building_record_chain(&mut t, target as u32, special);
    finish!(false)
}
