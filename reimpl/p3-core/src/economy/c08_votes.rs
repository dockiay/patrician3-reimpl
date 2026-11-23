//! C08: votos de comerciantes en elecciones y dietas (operaciones 0x3F y 0x45).
//!
//! - `scheduled_tasks_find_todays_vote_task` (`0x004D9C70`): primera tarea 0x20/0x21 que vence hoy.
//! - `op_cast_ballot_pending_vote` (`0x0053AA10`): voto de un concejal en la elección del Ältermann
//!   (tarea 0x20) o en la votación de la Dieta (tarea 0x21).
//! - Caso 0x45 del despachador (`0x00535FA4`): `merchant_set_mayor_candidacy_flag` (`0x004F9390`).
//!
//! Contrastados con el despachador original sobre estado vivo en `p3-sim/tests/c08_votes_live_x86.rs`.
use crate::{ai::reputation::Merchant, scheduler::Scheduler, town::Town};

/// `0x004D9C70`: índice de la primera tarea 0x20/0x21 cuyo vencimiento cae en el día de `tick`
/// (`due - (tick & !0xFF) <= 0xFF`); `None` si la lista se acaba o llega a una tarea posterior.
pub fn find_todays_vote_task(s: &Scheduler, tick: u32) -> Option<u16> {
    let day = tick & 0xffff_ff00;
    let mut at = s.head;
    let mut steps = 0usize;
    while at < s.capacity {
        let r = s.records.get(usize::from(at))?;
        if r.due.wrapping_sub(day) > 0xff {
            return None;
        }
        if (0x20..=0x21).contains(&r.kind) {
            return Some(at);
        }
        at = r.next;
        steps += 1;
        if steps > s.records.len() {
            return None;
        }
    }
    None
}

/// Peso del voto según el rango en la ciudad natal: 1 hasta concejal, 2 para el alcalde (rango 6) y 3 por encima.
fn ballot_weight(rank: u8) -> u8 {
    if rank < 6 {
        1
    } else if rank == 6 {
        2
    } else {
        3
    }
}

/// Máscara de 8 bits del votante humano `n` tal como la calculan `shl r8, cl` y `test r8, r8` (cuenta módulo 32).
fn voter_mask(n: u32) -> u8 {
    let n = n & 31;
    if n < 8 {
        1 << n
    } else {
        0
    }
}

/// `op_cast_ballot_pending_vote` (`0x0053AA10`). `voter` es `+4` y `choice` es `+8` de la operación.
/// Devuelve `true` si modificó la tarea.
pub fn cast_ballot(s: &mut Scheduler, towns: &[Town], merchants: &[Merchant], tick: u32, voter: u32, choice: u32) -> bool {
    if voter as usize >= merchants.len() {
        return false;
    }
    let m = &merchants[voter as usize];
    let rank = m.bytes[0x39c + usize::from(m.bytes[0x19])];
    if rank < 4 {
        return false;
    }
    let Some(index) = find_todays_vote_task(s, tick) else { return false };
    let humans = merchants[..voter as usize].iter().filter(|m| m.u16(8) == 0).count() as u32;
    let seats = towns
        .iter()
        .map(|t| (0..4).filter(|&i| u32::from(t.u8(0x6dc + i)) == voter).count() as i32)
        .sum::<i32>();
    let weight = ballot_weight(rank);
    let mask = voter_mask(humans);
    let task = &mut s.records[usize::from(index)];
    if task.kind == 0x20 {
        if task.u8_at(0x14) & mask == 0 {
            return false;
        }
        task.set_u8_at(0x14, task.u8_at(0x14) & !mask);
        if task.u8_at(0x16) < 1 {
            task.set_u8_at(0x16, 1);
        }
        let mut share = i32::from(task.u8_at(0x15)).wrapping_mul(seats) / i32::from(task.u8_at(0x16));
        if choice > 3 {
            return true;
        }
        let c = choice as usize;
        let count = i32::from(task.u8_at(0xc + c));
        let votes = i32::from(task.u8_at(0x10 + c));
        if count + votes + share > 0xff {
            share = 0xff - count - votes;
            if share < 0 {
                share = 0;
            }
        }
        task.set_u8_at(0xc + c, (count as u8).wrapping_add(weight));
        task.set_u8_at(0x10 + c, (votes as u8).wrapping_add(share as u8));
    } else {
        if task.u8_at(0x16) & mask == 0 {
            return false;
        }
        task.set_u8_at(0x16, task.u8_at(0x16) & !mask);
        if choice > 1 {
            task.set_u8_at(0x14, task.u8_at(0x14).wrapping_add(weight));
            let current = task.u8_at(0x15);
            let room = 0xfa - i32::from(current);
            task.set_u8_at(0x15, if room > seats { current.wrapping_add(seats as u8) } else { 0xfa });
        } else {
            let c = choice as usize;
            task.set_u8_at(0x12 + c, task.u8_at(0x12 + c).wrapping_add(weight));
            task.set_u8_at(0x10 + c, task.u8_at(0x10 + c).wrapping_add(seats as u8));
        }
    }
    true
}

/// Caso 0x45 del despachador: el comerciante `+4` (si existe) declara o retira su candidatura a la alcaldía con `+8`.
pub fn set_candidacy(merchants: &mut [Merchant], merchant: u32, value: u32) -> bool {
    let Some(m) = merchants.get_mut(merchant as usize) else { return false };
    m.set_mayor_candidacy_flag(value);
    true
}
