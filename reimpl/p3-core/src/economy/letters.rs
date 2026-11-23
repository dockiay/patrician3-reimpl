//! Cartas y capitanes de taberna (C08), sobre la imagen de memoria del original: tomar una carta `D` (`0x004D7340`), estado del capitán
//! (`0x004D7770`), liberar y tomar cartas de la ciudad (`0x00526AA0`, `0x00526B00`), carta de oferta disponible (`0x005A7350`) y asignar un
//! barco a una ranura de tarea (`0x0053CA30`). `Q` es el depósito de mensajes (`0x006DD730`): `+0` tabla de 16 bytes, `+4` cabeza libre, `+6`
//! cuantos registros.
use crate::economy::sched_queries::{merchant_addr, G_MERCHANTS, G_PLAYER};
use crate::flatmem::Mem;

pub const QUEUE: u32 = 0x006d_d730;
const LETTER_TABLE_PTR: u32 = 0x006d_d7a0;
const LETTER_TABLE_LEN: u32 = 0x006d_d892;
const SHIP_TABLE_PTR: u32 = 0x006d_d7a4;
const SHIP_TABLE_LEN: u32 = 0x006d_d894;
const SCHED: u32 = 0x006d_d73c;
const G_TOWNS: u32 = 0x006d_e4b0;
const TOWN_TABLE: u32 = 0x006d_e508;

fn mc(m: &mut dyn Mem) -> u32 {
    u32::from(m.r16(G_MERCHANTS))
}

/// `0x004D7340`: toma de la lista del comerciante la carta `D` de esa ciudad y tarea. Devuelve `(u16 de +0xC o 0xFFFF, limpiezas 0x51AAB0)`.
pub fn take_tavern_letter(m: &mut dyn Mem, q: u32, merchant: u32, town: u8, task: u16) -> (u16, u32) {
    if merchant >= mc(m) {
        return (0xffff, 0);
    }
    let mut budget = i32::from(m.r16(q + 6));
    let mer = merchant_addr(m, merchant);
    let mut link = mer + 10;
    let mut idx = u32::from(m.r16(link));
    while idx < u32::from(m.r16(q + 6)) && budget > 0 {
        let rec = m.r32(q).wrapping_add(idx * 0x10);
        if m.r8(rec + 4) == b'D' && m.r8(rec + 5) == town && m.r16(rec + 8) == task {
            let taken = m.r16(link);
            let next = m.r16(rec + 6);
            m.w16(link, next);
            let free = m.r16(q + 4);
            m.w16(rec + 6, free);
            m.w16(q + 4, taken);
            return (m.r16(rec + 0xc), 1);
        }
        link = rec + 6;
        idx = u32::from(m.r16(rec + 6));
        budget -= 1;
    }
    (0xffff, 0)
}

/// `captain_message_status` (`0x004D7770`): 0 = sin carta o capitán ocupado en un barco, 1 = libre, 2 = ya hay una carta `T` del comerciante.
pub fn captain_message_status(m: &mut dyn Mem, q: u32, merchant: u32, reference: u32) -> u32 {
    if reference == 0xffff_ffff {
        return 0;
    }
    let id = reference & 0xffff;
    let hi = (reference >> 16) as u16;
    if id < u32::from(m.r16(LETTER_TABLE_LEN)) {
        let rec = m.r32(LETTER_TABLE_PTR).wrapping_add(id * 0x10);
        if m.r8(rec + 3) == (reference >> 16) as u8 && m.r8(rec + 2) == (reference >> 24) as u8 {
            if merchant < mc(m) {
                let mer = merchant_addr(m, merchant);
                let mut idx = u32::from(m.r16(mer + 10));
                let mut budget = u32::from(m.r16(q + 6));
                while idx < u32::from(m.r16(q + 6)) && (budget as i32) > 0 {
                    let r = m.r32(q).wrapping_add(idx * 0x10);
                    if m.r8(r + 4) == b'T' && u32::from(m.r16(r + 10)) == id && m.r16(r + 8) == hi {
                        return 2;
                    }
                    idx = u32::from(m.r16(r + 6));
                    budget -= 1;
                }
            }
            let ships = u32::from(m.r16(SHIP_TABLE_LEN));
            if ships != 0 {
                let table = m.r32(SHIP_TABLE_PTR);
                for i in 0..ships {
                    let ship = table.wrapping_add(i * 0x180);
                    if u32::from(m.r16(ship + 0x42)) == id && m.r16(ship + 0x134) == 0x12 {
                        return 0;
                    }
                }
            }
        }
    }
    1
}

/// `0x00526AA0`: desenlaza la carta `id` de la lista libre de la ciudad (`town+0x82E`) y la marca vacía.
pub fn free_town_letter(m: &mut dyn Mem, town: u32, id: u32) -> bool {
    let len = u32::from(m.r16(LETTER_TABLE_LEN));
    let mut link = town + 0x82e;
    let mut cur = u32::from(m.r16(link));
    loop {
        if len <= cur {
            return false;
        }
        let rec = m.r32(LETTER_TABLE_PTR).wrapping_add(u32::from(m.r16(link)) * 0x10);
        if u32::from(m.r16(link)) == id {
            let next = m.r16(rec);
            m.w16(link, next);
            m.w16(rec, 0xffff);
            return true;
        }
        cur = u32::from(m.r16(rec));
        link = rec;
    }
}

/// `town_take_captain_from_list_head` (`0x00526B00`): mueve la carta cuyo índice hay en `*p` a la cabeza de la lista de la ciudad.
pub fn take_captain_from_list_head(m: &mut dyn Mem, town: u32, p: u32) {
    let idx = m.r16(p);
    if u32::from(idx) < u32::from(m.r16(LETTER_TABLE_LEN)) {
        let rec = m.r32(LETTER_TABLE_PTR).wrapping_add(u32::from(idx) * 0x10);
        let next = m.r16(rec);
        m.w16(p, next);
        let head = m.r16(town + 0x82e);
        m.w16(rec, head);
        m.w16(town + 0x82e, idx);
    }
}

/// `tavern_offer_letter_is_available_to_player` (`0x005A7350`): la carta `index` de la cola sigue disponible para el jugador local.
pub fn tavern_offer_letter_is_available(m: &mut dyn Mem, index: u32) -> bool {
    let index = index & 0xffff;
    let rec = if m.r16(QUEUE + 6) < index as u16 { 0 } else { m.r32(QUEUE).wrapping_add(index << 4) };
    let payload = m.r32(rec.wrapping_add(8));
    let id = u32::from(m.r16(payload.wrapping_add(8)));
    let task = if u32::from(m.r16(SCHED + 0xc)) < id { 0 } else { m.r32(SCHED).wrapping_add(id.wrapping_mul(0x18)) };
    if m.r16(task.wrapping_add(6)) != 0x1b {
        return false;
    }
    let slot = u32::from(m.r8(payload.wrapping_add(0xc)));
    let array = m.r32(task + 0xc);
    let value = m.r32(array.wrapping_add(slot * 4));
    !(value != m.r32(G_PLAYER) && value < mc(m))
}

/// `op_tavern_assign_ship_to_task_slot` (`0x0053CA30`): devuelve las limpiezas (`0x0051AAB0`) pedidas por la carta tomada.
pub fn op_tavern_assign_ship_to_task_slot(m: &mut dyn Mem, op: u32) -> u32 {
    let town = m.r8(op + 0xc);
    let towns = m.r32(G_TOWNS);
    if u32::from(town) >= towns {
        return 0;
    }
    let ship = u32::from(m.r16(op + 4));
    if ship < u32::from(m.r16(SHIP_TABLE_LEN)) && u32::from(m.r8(op + 0xd)) < towns {
        let table = m.r32(SHIP_TABLE_PTR);
        let owner = u32::from(m.r8(table.wrapping_add(ship * 0x180)));
        let id = m.r16(op + 0xe);
        let (slot, cleanups) = take_tavern_letter(m, QUEUE, owner, town, id);
        let task = if m.r16(SCHED + 0xc) < id { 0 } else { m.r32(SCHED).wrapping_add(u32::from(id) * 0x18) };
        if task != 0 && slot < m.r16(task + 0x12) {
            let array = m.r32(task + 0xc);
            let s = u32::from(m.r16(op + 4));
            m.w32(array.wrapping_add(u32::from(slot) * 4), s);
        }
        return cleanups;
    }
    if m.r32(op) == 0x53 {
        let base = m.r32(TOWN_TABLE);
        m.w8(base.wrapping_add(0x83f).wrapping_add(u32::from(town) * 0x9f8), 0xff);
    }
    0
}
