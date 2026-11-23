//! Propuesta diaria del concejo (`0x0051E070`).
use crate::{pump::Operation, scheduler::Scheduler, town::Town};

/// `town_propose_wall_enlargement` (0x0051E270).
pub fn propose_wall_enlargement(town: &Town, tick: u32, scheduler: &Scheduler, world_8a: u8) -> Option<Operation> {
    if world_8a != 0 || town.u16(0x766) < town.u16(0x776) || !crate::buildings::Site::is_plan_point_valid(town, 2) {
        return None;
    }
    if scheduler.queue().into_iter().any(|i| {
        let r = &scheduler.records[usize::from(i)];
        r.kind == 0x2e && r.u8_at(0xc) == town.index() && r.u8_at(0xd) == 1
    }) {
        return None;
    }
    let date = scheduler.next_free_day_for_town(tick.wrapping_add(0x700), town.index());
    Some(Operation([
        0x48,
        date,
        0,
        u32::from(town.index()) | 0x100 | 0x01000000,
        255,
    ]))
}

/// `0x0053B0D0`/`0x004DA060`: voto humano y votos de concejales sobornados.
pub fn cast_vote(
    scheduler: &mut Scheduler,
    town: &Town,
    merchants: &[crate::ai::reputation::Merchant],
    tick: u32,
    merchant: usize,
    city: u8,
    choice: u32,
) -> bool {
    let Some(m) = merchants.get(merchant) else { return false };
    if m.bytes.len() <= 0x39c + city as usize || m.u16(8) != 0 {
        return false;
    }
    let mut bit = 1u32;
    for prior in &merchants[..merchant] {
        if prior.u16(8) == 0 {
            bit = bit.wrapping_mul(2)
        }
    }
    let mut at = scheduler.head;
    let mut steps = 0;
    while at < scheduler.capacity && (at as usize) < scheduler.records.len() && steps < scheduler.records.len() {
        let r = scheduler.records[at as usize];
        steps += 1;
        if r.due > (tick | 255) {
            return false;
        }
        if r.kind == 0x2e && r.u8_at(0xc) == city {
            let mask = bit as u8;
            if mask & r.u8_at(0x13) == 0 {
                return false;
            }
            let rank = m.bytes[0x39c + city as usize];
            let weight = if rank < 5 {
                1
            } else if rank < 7 {
                2
            } else {
                3
            };
            let record = &mut scheduler.records[at as usize];
            let target = if choice == 0 {
                0x10
            } else if choice == 1 {
                0x11
            } else {
                0x12
            };
            record.set_u8_at(target, record.u8_at(target).wrapping_add(weight));
            record.set_u8_at(0x13, record.u8_at(0x13).wrapping_sub(mask));
            for i in 0..4 {
                if usize::from(town.u8(0x6dc + i)) == merchant {
                    let target = if choice == 0 {
                        0xe
                    } else if choice == 1 {
                        0xf
                    } else {
                        0x12
                    };
                    record.set_u8_at(target, record.u8_at(target).wrapping_add(1));
                }
            }
            return true;
        }
        at = r.next;
    }
    false
}

/// Devuelve la operación 0x48; el anfitrión debe encolarla y despacharla.
pub fn propose(town: &Town, tick: u32, scheduler: &Scheduler) -> Option<Operation> {
    if tick < town.u32(0x6fc) || scheduler.council_meeting_mask(town.index(), tick) != 0 {
        return None;
    }
    let mut score = 0u32;
    let mut kind = 255u8;
    let mut arg = 0u8;
    let pop = town.i32(0x2d4);
    if pop > 0 {
        let troops = u32::from(town.u8(0x999)) + u32::from(town.u8(0x99a)) + u32::from(town.u8(0x99b));
        let scaled = u32::from(town.u16(0x9a0))
            .wrapping_mul(2000)
            .wrapping_sub(troops.wrapping_mul(2000));
        let ratio = scaled / (pop as u32);
        if ratio < 4 {
            score = 4 - ratio;
            kind = 2;
        }
    }
    if town.u16(0x766) >= town.u16(0x776) {
        let total = i32::from(town.u16(0x78e));
        let inside = i32::from(town.u16(0x78c));
        if total - inside > 15 + i32::from(town.index() % 5) {
            let n = (inside * 4) / total;
            if n < 4 && (4 - n) as u32 > score {
                score = (4 - n) as u32;
                kind = 1;
            }
        }
    }
    let tax = u32::from(town.u8(0x6f8));
    let scaled = tax * tax / 20;
    if scaled > 5 && scaled - 5 > score {
        kind = 3;
        arg = 10;
    }
    if kind == 255 {
        return None;
    }
    let date = scheduler.next_free_day_for_town(tick.wrapping_add(0xe00), town.index());
    Some(Operation([
        0x48,
        date,
        0,
        u32::from(town.index()) | (u32::from(kind) << 8) | (u32::from(arg) << 16),
        255,
    ]))
}

/// `0x0053B010`: despacha una operación de oferta para la ciudad indicada.
/// El llamante valida el índice de ciudad y conserva la operación modificada.
pub fn dispatch_offer(town: &mut Town, tick: u32, scheduler: &mut Scheduler, op: &mut Operation) {
    let packed = op.0[3];
    let city = packed as u8;
    let kind = (packed >> 8) as u8;
    let value = (packed >> 16) as u8;
    if town.flags() & 0xe0 == 0 {
        if kind == 0 {
            let i = scheduler.add_task(tick.wrapping_add(1));
            let r = &mut scheduler.records[i as usize];
            r.kind = 0x31;
            r.set_u32_at(8, op.0[2]);
            r.set_u8_at(0xc, city);
        } else if kind == 3 {
            town.set_u8(0x6f8, if value > 20 { 10 } else { value });
        }
        return;
    }
    op.0[1] = scheduler.next_free_day_for_town(op.0[1], city);
    let i = scheduler.add_task(op.0[1]);
    let r = &mut scheduler.records[i as usize];
    r.kind = 0x2e;
    r.set_u32_at(8, op.0[2]);
    r.set_u8_at(0xc, city);
    r.set_u8_at(0xd, kind);
    for offset in 0xe..=0x13 {
        r.set_u8_at(offset, 0);
    }
    r.set_u8_at(0x14, value);
    r.set_u8_at(0x15, (packed >> 24) as u8);
    r.set_u8_at(0x16, op.0[4] as u8);
}
