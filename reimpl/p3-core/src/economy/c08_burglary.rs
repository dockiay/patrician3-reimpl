//! C08: encargo de un robo en la taberna (`op_commission_burglary`, `0x0053BA40`, operación 0x4F), sobre la memoria
//! del original ([`Mem`]). `+4` precio, `+8` palabra (días en los 3 bits bajos), `+0xA` ciudad, `+0xC` cliente,
//! `+0xE` víctima y `+0x10` byte de aviso. Las llamadas al original pasan por [`OpsHost`]:
//! `find_or_create_office` (`0x005308A0`, sin crear) y `scheduled_tasks_add_task` (`0x004D8CF0`).
use super::ops_misc::OpsHost;
use crate::flatmem::Mem;

const WORLD: u32 = 0x006d_e4a0;
const SCHED: u32 = 0x006d_d73c;
const TOWN_COUNT: u32 = 0x006d_e4b0;
const TOWNS: u32 = 0x006d_e508;
const MERCHANT_COUNT: u32 = 0x006d_e4aa;
const TICK: u32 = 0x006d_e4b4;
/// Palabras `0x00672DDA` y `0x00672DD8` del ejecutable inglés: ajustes de `+0x16` del cliente.
pub const PRICE_ADJUST: i16 = 256;
pub const TASK_ADJUST: i16 = 256;

fn merchant(m: &mut dyn Mem, i: u32) -> u32 {
    m.r32(WORLD + 0x78).wrapping_add(i.wrapping_mul(0x650))
}

/// Suma con saturación de `+0x16` tal como la escribe el original (límites `0x7FFF` y `0x8000`).
fn adjust(m: &mut dyn Mem, at: u32, delta: i16) {
    let cur = m.r16(at) as i16;
    let value = if delta > 0 {
        if i32::from(cur) < 0x7fff - i32::from(delta) { cur.wrapping_add(delta) as u16 } else { 0x7fff }
    } else if i32::from(cur) + 0x8000 > -i32::from(delta) {
        cur.wrapping_add(delta) as u16
    } else {
        0x8000
    };
    m.w16(at, value);
}

pub fn commission_burglary(m: &mut dyn Mem, host: &mut dyn OpsHost, op: u32) {
    let city = u32::from(m.r16(op + 0xa));
    if city >= m.r32(TOWN_COUNT) {
        return;
    }
    let town = m.r32(TOWNS).wrapping_add(city * 0x9f8);
    let client = m.r16(op + 0xc);
    let victim = m.r16(op + 0xe);
    let count = m.r16(MERCHANT_COUNT);
    let reset_unless_client = |m: &mut dyn Mem| {
        if u16::from(m.r8(town + 0x83c)) != client {
            m.w8(town + 0x83c, 0xff);
        }
    };
    if client >= count || victim >= count {
        reset_unless_client(m);
        return;
    }
    let target = merchant(m, u32::from(victim));
    if m.r8(target + 8) & 4 == 4 {
        reset_unless_client(m);
        return;
    }
    let slot = m.r8(town + 0x83c);
    if u16::from(slot) != client && slot != 0xff {
        return;
    }
    let buyer = merchant(m, u32::from(client));
    let office = host.call(m, 0x5308a0, WORLD, &[u32::from(victim), city, 0]);
    if office == 0 {
        m.w8(town + 0x83c, 0xff);
        return;
    }
    let price = m.r32(op + 4);
    let gold = m.r32(buyer).wrapping_sub(price);
    m.w32(buyer, gold);
    let spent = m.r32(buyer + 0x4bc).wrapping_add(price);
    m.w32(buyer + 0x4bc, spent);
    m.w8(town + 0x83c, 1);
    adjust(m, buyer + 0x16, PRICE_ADJUST);
    let days = u32::from(m.r8(op + 8) & 7).max(3);
    let due = ((days << 8).wrapping_add(m.r32(TICK))) | 0xfa;
    let task = host.call(m, 0x4d8cf0, SCHED, &[due]);
    m.w16(task + 6, 0xf);
    let (c, t, v) = (m.r8(op + 0xc), m.r8(op + 0xa), m.r8(op + 0xe));
    m.w8(task + 0xa, c);
    m.w8(task + 0xb, t);
    m.w8(task + 0xc, v);
    let w = m.r16(op + 8);
    m.w16(task + 8, w);
    adjust(m, buyer + 0x16, TASK_ADJUST);
    if m.r8(op + 0x10) == 0 {
        return;
    }
    let due = m.r32(TICK).wrapping_add(0x200) | 0x80;
    let notice = host.call(m, 0x4d8cf0, SCHED, &[due]);
    m.w16(notice + 6, 5);
    m.w8(notice + 8, c);
    m.w8(notice + 9, t);
    let home = m.r8(buyer + 0x19);
    m.w8(notice + 0xb, home);
    m.w8(notice + 0xa, v);
    let now = m.r32(TICK);
    m.w32(notice + 0xc, now);
    m.w8(notice + 0x10, 0);
    m.w8(notice + 0x11, 0);
}
