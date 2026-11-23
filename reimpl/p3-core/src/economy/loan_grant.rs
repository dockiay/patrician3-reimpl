//! Concesión de un préstamo (`op_grant_loan`, `0x00539DC0`): encola la tarea 1 de vencimiento, anula el contador de la ciudad, abona al prestatario y
//! suma la reputación social del prestamista (`Merchant::grant_loan_to_town_with_reputation`, `0x004F8FE0`).
use crate::{ai::reputation::Merchant, scheduler::Scheduler, town::Town};

/// Los 0x14 bytes de la operación (`+4` u32, `+8..+0xB`, `+0xC` importe, `+0xE` semanas/plazo, `+0xF`, `+0x10` prestamista, `+0x11` prestatario, `+0x12` ciudad,
/// `+0x13` banderas). Devuelve la operación con `+0x13` posiblemente recortado (el original lo escribe).
pub fn grant(op: &mut [u8; 0x14], towns: &mut [Town], merchants: &mut [Merchant], s: &mut Scheduler, tick: u32) {
    let u16at = |b: &[u8], i: usize| u16::from(b[i]) | u16::from(b[i + 1]) << 8;
    let amount = u16at(op, 0xc);
    let (lender, borrower, town) = (usize::from(op[0x10]), usize::from(op[0x11]), usize::from(op[0x12]));
    if town >= towns.len() {
        return;
    }
    // `cmp cx, [0x6DE4AA]` con `cx` de 16 bits: el índice 8 bits cuenta como comerciante si es menor que el recuento.
    if lender < merchants.len() && merchants[lender].i32(0) < i32::from(amount) {
        return;
    }
    let due = tick.wrapping_add(u32::from(op[0xe]) * 0x700);
    s.dispatching = 0;
    let id = s.add_task(due);
    s.dispatching = 1;
    let r = &mut s.records[usize::from(id)];
    r.kind = 1;
    r.set_u32_at(8, u32::from_le_bytes([op[4], op[5], op[6], op[7]]));
    r.set_u32_at(0xc, tick);
    r.set_u8_at(0xc, op[0xf]);
    r.set_u8_at(0x10, op[0x10]);
    r.set_u8_at(0x11, op[0x11]);
    r.set_u8_at(0x12, op[8]);
    r.set_u8_at(0x13, op[0xe]);
    r.set_u8_at(0x14, op[0x12]);
    r.set_u8_at(0x15, op[9]);
    r.set_u8_at(0x16, op[0xa]);
    r.set_u8_at(0x17, op[0xb]);
    let class = usize::from(op[0x13] >> 4);
    if class < 6 {
        towns[town].set_u16(0x9ca + class * 6, 0);
    }
    if borrower < merchants.len() {
        let m = &mut merchants[borrower];
        let v = m.i32(0).wrapping_add(i32::from(amount));
        m.set_i32(0, v);
    }
    if op[0x13] & 0xf > 4 {
        op[0x13] = (op[0x13] & 0xf4) | 4;
    }
    if lender < merchants.len() {
        merchants[lender].grant_loan_to_town_with_reputation(
            u32::from(op[0x12]),
            u32::from(op[0x13] & 0xf),
            i32::from(amount),
        );
    }
}
