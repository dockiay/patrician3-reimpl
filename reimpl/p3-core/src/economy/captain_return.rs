//! Fin de reserva de un capitán de taberna, tarea 0x29.
use crate::{scheduler::TaskRecord, ships::captains::CaptainPool, town::Town};

/// Devuelve si la tarea debe permanecer. La lista libre y las cadenas inválidas se rechazan.
pub fn advance(task: &mut TaskRecord, towns: &mut [Town], pool: &mut CaptainPool) -> Result<bool, &'static str> {
    if pool.entries.len() != usize::from(pool.capacity) * 16 {
        return Err("pool de capitanes truncado");
    }
    let captain = u16::from_le_bytes([task.u8_at(12), task.u8_at(13)]);
    let city = u16::from_le_bytes([task.u8_at(8), task.u8_at(9)]);
    if captain >= pool.capacity || usize::from(city) >= towns.len() {
        return Ok(false);
    }
    let town = &mut towns[usize::from(city)];
    let mut at = town.u16(0x82e);
    let mut prev = None;
    let mut seen = vec![false; usize::from(pool.capacity)];
    while at < pool.capacity && at != captain {
        let i = usize::from(at);
        if seen[i] {
            return Err("lista de capitanes de taberna cíclica");
        }
        seen[i] = true;
        prev = Some(i);
        at = u16::from_le_bytes(pool.entries[i * 16..i * 16 + 2].try_into().unwrap());
    }
    if at != captain {
        return Ok(false);
    }
    if seen[usize::from(at)] {
        return Err("lista de capitanes de taberna cíclica");
    }
    let next = u16::from_le_bytes(
        pool.entries[usize::from(at) * 16..usize::from(at) * 16 + 2]
            .try_into()
            .unwrap(),
    );
    if task.u32_at(16) == 0 {
        let mut cursor = pool.free_head;
        let mut free_seen = vec![false; usize::from(pool.capacity)];
        while cursor < pool.capacity {
            let i = usize::from(cursor);
            if free_seen[i] {
                return Err("lista libre de capitanes cíclica");
            }
            free_seen[i] = true;
            cursor = u16::from_le_bytes(pool.entries[i * 16..i * 16 + 2].try_into().unwrap());
        }
    }
    if let Some(prev) = prev {
        pool.entries[prev * 16..prev * 16 + 2].copy_from_slice(&next.to_le_bytes());
    } else {
        town.set_u16(0x82e, next);
    }
    if task.u32_at(16) == 0 {
        let mut slot = captain;
        pool.free(&mut slot);
        Ok(false)
    } else {
        let head = town.u16(0x82e);
        let entry = &mut pool.entries[usize::from(captain) * 16..usize::from(captain + 1) * 16];
        entry[0..2].copy_from_slice(&head.to_le_bytes());
        entry[15] = 0xff;
        town.set_u16(0x82e, captain);
        task.due = task.due.wrapping_add(0xe00);
        Ok(true)
    }
}
