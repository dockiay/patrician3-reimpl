//! Operaciones de subasta (C08): pujar (`0x0053A010`), recordatorio (`0x0053A0B0`), puja máxima (`0x0053A170`) y programar la subasta de
//! una ciudad (`0x0053A250`), sobre la imagen de memoria. `op` es la dirección de la operación (`+4` valor/comerciante, `+8` tarea/ciudad).
use crate::economy::sched_queries::{find_auction_check_value, merchant_addr, G_MERCHANTS, G_TICK};
use crate::flatmem::Mem;

const SCHED: u32 = 0x006d_d73c;
const G_TOWNS: u32 = 0x006d_e4b0;

/// Alta de una tarea en el planificador (`0x004D8CF0`): el anfitrión devuelve la dirección del registro nuevo con su vencimiento escrito.
pub trait TaskAdder {
    fn add_task(&mut self, m: &mut dyn Mem, due: u32) -> u32;
}

fn mc(m: &mut dyn Mem) -> u32 {
    u32::from(m.r16(G_MERCHANTS))
}
fn count(m: &mut dyn Mem) -> u32 {
    u32::from(m.r16(SCHED + 0xc))
}
/// `0x004D8DB0`: registro `id` (con `id <= capacidad`).
fn task(m: &mut dyn Mem, id: u32) -> u32 {
    if count(m) < id {
        0
    } else {
        m.r32(SCHED).wrapping_add(id.wrapping_mul(0x18))
    }
}
fn is_auction(m: &mut dyn Mem, t: u32) -> bool {
    let k = m.r16(t + 6);
    k == 10 || k == 0x36
}

/// `op_auction_place_bid` (`0x0053A010`): sube la puja si el comerciante tiene el dinero y supera la actual.
pub fn place_bid(m: &mut dyn Mem, op: u32) {
    let id = m.r32(op + 8);
    if id >= count(m) {
        return;
    }
    let t = task(m, id);
    let state = m.r8(t + 0x14);
    let merchant = m.r32(op + 4);
    if !(is_auction(m, t) && state > 3 && state < 7 && merchant < mc(m) && u16::from(m.r8(t + 0x17)) != merchant as u16) {
        return;
    }
    let mer = merchant_addr(m, merchant);
    let bid = m.r32(op + 0xc);
    if bid <= m.r32(mer) && u32::from(m.r16(t + 0xe)) < bid {
        m.w16(t + 0xe, bid as u16);
        let who = m.r8(op + 4);
        m.w8(t + 0x17, who);
        m.w8(t + 0x14, 4);
        m.w16(t + 0x12, 0);
    }
}

/// `op_auction_set_reminder_flag` (`0x0053A0B0`): marca o desmarca el aviso del comerciante (un bit por ranura sin `+8`).
pub fn set_reminder_flag(m: &mut dyn Mem, op: u32) {
    let merchant = m.r32(op + 4);
    if merchant >= mc(m) {
        return;
    }
    let mer = merchant_addr(m, merchant);
    if m.r16(mer + 8) != 0 {
        return;
    }
    let id = m.r32(op + 8);
    if id >= count(m) {
        return;
    }
    let t = task(m, id);
    if !is_auction(m, t) {
        return;
    }
    let mut bit = 1u8;
    for u in 0..mc(m) {
        let other = merchant_addr(m, u);
        if m.r16(other + 8) == 0 {
            if u == merchant {
                let cur = m.r8(t + 0x16);
                let v = if m.r32(op + 0xc) == 0 { cur & !bit } else { cur | bit };
                m.w8(t + 0x16, v);
                return;
            }
            bit = bit.wrapping_mul(2);
        }
    }
}

/// `op_auction_set_max_bid` (`0x0053A170`): registra (o actualiza) la puja máxima automática de la subasta.
pub fn set_max_bid(m: &mut dyn Mem, env: &mut dyn TaskAdder, op: u32) {
    let id = m.r32(op + 8);
    if id >= count(m) {
        return;
    }
    let t = task(m, id);
    let which = u32::from(m.r8(t + 0x15));
    if !is_auction(m, t) {
        return;
    }
    let (who_a, bid_c) = (m.r32(op + 0x10), m.r32(op + 0xc));
    let (found, _) = find_auction_check_value(m, SCHED, who_a, which, bid_c);
    if count(m) <= found & 0xffff {
        let due = m.r32(G_TICK).wrapping_add(1);
        let r = env.add_task(m, due);
        m.w16(r + 6, 0x2d);
        let v = m.r32(op + 4) & 0x7fff_ffff;
        m.w32(r + 8, v);
        let a = m.r16(op + 8);
        m.w16(r + 0xc, a);
        let b = m.r16(op + 0xc);
        m.w16(r + 0xe, b);
        let who = m.r8(op + 0x10);
        m.w8(r + 0x11, which as u8);
        m.w8(r + 0x10, who);
        return;
    }
    let t2 = task(m, found & 0xffff);
    let v = m.r32(op + 4) & 0x7fff_ffff;
    m.w32(t2 + 8, v);
}

/// `op_schedule_town_auction_task` (`0x0053A250`): programa la subasta de la ciudad `op+8 % ciudades` para el día `op+4`.
pub fn schedule_town_auction(m: &mut dyn Mem, env: &mut dyn TaskAdder, op: u32) {
    let due = m.r32(op + 4);
    if m.r32(G_TICK).wrapping_add(0x2800) <= due {
        let r = env.add_task(m, due);
        m.w16(r + 6, 10);
        m.w8(r + 0x14, 0);
        let towns = m.r32(G_TOWNS);
        let town = (m.r32(op + 8) % towns) as u8;
        m.w8(r + 0x15, town);
    }
}
