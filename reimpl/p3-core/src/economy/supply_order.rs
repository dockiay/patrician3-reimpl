//! Aceptación de un pedido urbano `0x0053A620`.
use super::supply_deadline::Notice;
use crate::{calendar::MONTH_START, pump::Operation, scheduler::Scheduler, town::Town};

fn original_date(tick: u32) -> (u8, u8, u16) {
    let days = (tick as i32) >> 8;
    let year = (days / 365) as u16;
    let rem = days.wrapping_sub(i32::from(year) * 365);
    if rem < 0 {
        return (rem.wrapping_sub(24).wrapping_add(1) as u8, 0, year);
    }
    let mut month = 0usize;
    while month < 12 && i32::from(MONTH_START[month]) <= rem {
        month += 1
    }
    let day = (rem - i32::from(MONTH_START[month - 1]) + 1) as u8;
    (day, month as u8, year)
}

/// El tablón son los siete registros de doce bytes de GameWorld+0x90.
pub fn accept(
    op: Operation,
    tick: u32,
    current_year: u16,
    towns: &mut [Town],
    scheduler: &mut Scheduler,
    board: &mut [u8; 84],
) -> Result<Option<Notice>, String> {
    if op.0[0] != 0x3d {
        return Err("operación de encargo inválida".into());
    }
    let mut payload = [0u8; 16];
    for (dst, word) in payload.chunks_exact_mut(4).zip(&op.0[1..]) {
        dst.copy_from_slice(&word.to_le_bytes())
    }
    let city = usize::from(payload[0]);
    let Some(original) = towns.get(city) else {
        return Ok(None);
    };
    if payload[1] >= 24 || op.0[2] <= tick {
        return Ok(None);
    }
    let mut town = original.clone();
    let slot = if town.u16(0x830) == u16::MAX {
        0x830
    } else if town.u16(0x832) == u16::MAX {
        0x832
    } else {
        return Ok(None);
    };
    let mut next = scheduler.clone();
    if next.records.len() != usize::from(next.capacity) {
        return Err("planificador truncado".into());
    }
    let mut at = next.head;
    let mut seen = vec![false; usize::from(next.capacity)];
    while at < next.capacity {
        let i = usize::from(at);
        if seen[i] {
            return Err("planificador cíclico".into());
        }
        seen[i] = true;
        at = next.records[i].next
    }
    if next.free_head > next.capacity || next.free_head == next.capacity && next.capacity > u16::MAX - 128 {
        return Err("capacidad del planificador inválida".into());
    }
    let id = next.free_head;
    town.set_u16(slot, id);
    let allocated = next.add_task(op.0[2]);
    if allocated != id {
        return Err("asignación de tarea inconsistente".into());
    }
    let task = &mut next.records[usize::from(id)];
    task.kind = 0x11;
    task.payload = payload;
    let (day, month, year) = original_date(op.0[2]);
    let months = u32::from(year)
        .wrapping_mul(12)
        .wrapping_sub(u32::from(current_year.wrapping_mul(12)))
        .wrapping_add(u32::from(month))
        .wrapping_sub(1);
    let packed = ((months << 5) | u32::from(day & 31)) as u16;
    let mut record = [0u8; 16];
    record[4] = 0x1a;
    record[5] = payload[0];
    record[8] = payload[3];
    record[9] = payload[2];
    record[10..12].copy_from_slice(&u16::from(payload[1]).to_le_bytes());
    record[12..14].copy_from_slice(&payload[10..12]);
    record[14..16].copy_from_slice(&packed.to_le_bytes());
    let mut staged = *board;
    for row in staged.chunks_exact_mut(12) {
        if row[0] == payload[0] {
            row[0] = 255;
            row[1] = 255;
            row[3] = 0
        }
    }
    *board = staged;
    towns[city] = town;
    *scheduler = next;
    Ok(Some(Notice {
        recipient: usize::from(payload[14]),
        record,
    }))
}
