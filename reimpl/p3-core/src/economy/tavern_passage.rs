//! `op_accept_tavern_passage_request` (0x0053C840) y su ayudante de fecha `0x005311D0`.
use super::tavern_interaction::{TOWNS, TOWN_COUNT, TOWN_STRIDE};
use crate::calendar::MONTH_START;
use crate::flatmem::Mem;

const SHIPS: u32 = 0x6DD7A4;
const SHIP_COUNT: u32 = 0x6DD894;
const SHIP_STRIDE: u32 = 0x180;
const TICK: u32 = 0x6DE4B4;
const SCHED: u32 = 0x6DD73C;

pub trait PassageHost {
    /// `scheduler_add_task` (0x004D8CF0): dirección del registro nuevo.
    fn add_task(&mut self, due: u32) -> u32;
    /// `0x004D6530(0x6DD730, propietario, carta)`. Los bytes 3, 6 y 7 de la carta salen sin inicializar del original.
    fn send_letter(&mut self, owner: u8, record: &[u8; 16]);
    /// `0x00517770(barco, bits)`: marca de cambios del barco.
    fn mark_changed(&mut self, ship: u32, bits: u32);
}

/// `0x005311D0`: (día del mes, mes desde 0, año) de un tick; copia literal de la aritmética del original.
pub fn date_from_tick(tick: i32) -> (u8, u8, u16) {
    let day_count = tick >> 8;
    let high = ((i64::from(day_count) * i64::from(0xB38C_F9B1u32 as i32)) >> 32) as i32;
    let mut year = high.wrapping_add(day_count) >> 8;
    year = year.wrapping_add(((year as u32) >> 31) as i32);
    let year16 = year as u16;
    let y = i32::from(year16);
    let day_of_year = day_count.wrapping_sub(y.wrapping_mul(365));
    let mut month = 0u8;
    if i32::from(MONTH_START[0]) <= day_of_year {
        loop {
            month = month.wrapping_add(1);
            if i32::from(MONTH_START.get(usize::from(month)).copied().unwrap_or(0)) > day_of_year {
                break;
            }
        }
    }
    let low = if month == 0 {
        0x18
    } else {
        MONTH_START[usize::from(month) - 1] as u8
    };
    let day = (day_of_year as u8).wrapping_sub(low).wrapping_add(1);
    (day, month, year16)
}

pub fn accept_request(m: &mut dyn Mem, host: &mut dyn PassageHost, op: u32) {
    let towns = m.r32(TOWN_COUNT);
    let town_of = m.r8(op + 0xC);
    let ship_index = m.r16(op + 4);
    let dest = u32::from(m.r8(op + 0xD));
    let table = m.r32(TOWNS);
    let reject = ship_index >= m.r16(SHIP_COUNT) || dest >= towns;
    if u32::from(town_of) >= towns {
        return;
    }
    if reject {
        if m.r32(op) == 0x53 {
            m.w8(table.wrapping_add(u32::from(town_of) * TOWN_STRIDE) + 0x83F, 0xff);
        }
        return;
    }
    let ships = m.r32(SHIPS);
    let ship = ships.wrapping_add(u32::from(ship_index) * SHIP_STRIDE);
    if m.r32(table.wrapping_add(dest * TOWN_STRIDE) + 0x2C8) & 1 == 1 && m.r8(ship + 0xE) >= 2 {
        return;
    }
    let tick = m.r32(TICK);
    let task = host.add_task(tick.wrapping_add(4));
    m.w16(task + 6, 0x10);
    let v = m.r16(op + 4);
    m.w16(task + 8, v);
    m.w8(task + 0xA, town_of);
    m.w8(task + 0xB, dest as u8);
    let v = m.r16(op + 6);
    m.w16(task + 0xC, v);
    let v = m.r32(op + 8);
    m.w32(task + 0x10, v);
    let base = m.r32(TOWNS);
    m.w8(base.wrapping_add(u32::from(town_of) * TOWN_STRIDE) + 0x83F, 0);
    let when = m.r32(task + 0x10) as i32;
    let (day, month, _) = date_from_tick(when);
    let mut record = [0u8; 16];
    record[1] = 0x80;
    record[2] = 0xff;
    record[3] = 0xff;
    record[4] = 0x17;
    record[5] = town_of;
    record[8..10].copy_from_slice(&(dest as u16).to_le_bytes());
    record[0xA..0xC].copy_from_slice(&((u16::from(month) << 5).wrapping_add(u16::from(day))).to_le_bytes());
    let (w6, w4) = (m.r16(op + 6), m.r16(op + 4));
    record[0xC..0xE].copy_from_slice(&w6.to_le_bytes());
    record[0xE..0x10].copy_from_slice(&w4.to_le_bytes());
    let ship_now = m.r32(SHIPS).wrapping_add(u32::from(m.r16(op + 4)) * SHIP_STRIDE);
    let owner = m.r8(ship_now);
    host.send_letter(owner, &record);
    let ship = m.r32(SHIPS).wrapping_add(u32::from(m.r16(op + 4)) * SHIP_STRIDE);
    let mut counter = m.r8(ship + 0x157).wrapping_add(1);
    m.w8(ship + 0x157, counter);
    if counter >= 0x7F {
        counter = 0;
        m.w8(ship + 0x157, counter);
    }
    let _ = SCHED;
    host.mark_changed(ship, 4);
}
