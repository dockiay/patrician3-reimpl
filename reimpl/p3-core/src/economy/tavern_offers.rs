//! Órdenes de la taberna que cierran ofertas: `op_accept_tavern_script_offer` (0x0053EDF0) y `op_buy_tavern_region_news` (0x0053B4D0).
//! Traducción 1:1 sobre la memoria del original ([`Mem`]); lo que el original delega en otras funciones pasa por el anfitrión.
use super::tavern_interaction::{get_task, merchant, LETTERS, MERCHANT_COUNT, SCHED, TOWNS, TOWN_COUNT, TOWN_STRIDE};
use crate::flatmem::Mem;

const REGION_KEY_TABLE: u32 = 0x6DDBC0;
const REGION_KEY_STRIDE: u32 = 52;

pub trait TavernOffersHost {
    /// `0x0051AAB0`: destructor de la entrada de carta (sin escribir su enlace).
    fn release_letter(&mut self, entry: u32);
    /// `0x00535400(0x6DE588, ciudad, comerciante)`: olvida las noticias de la ciudad para ese comerciante.
    fn forget_news(&mut self, town: u32, merchant: u32);
}

/// `0x004D7870(cabeza, a, b)`: desengancha la primera carta tipo 0x71 cuyo objeto lleva `a` (u16 en +8) y `b` (byte en +0xA).
fn unlink_letter(m: &mut dyn Mem, host: &mut dyn TavernOffersHost, head: u32, a: u32, b: u32) -> bool {
    let count = m.r16(LETTERS + 6);
    let mut link = head;
    let mut ax = m.r16(link);
    if ax >= count {
        return false;
    }
    let table = m.r32(LETTERS);
    loop {
        let e = table.wrapping_add(u32::from(ax) << 4);
        if m.r8(e + 4) == 0x71 {
            let obj = m.r32(e + 8);
            if u32::from(m.r16(obj + 8)) == a && u32::from(m.r8(obj + 0xA)) == b {
                let idx = m.r16(link);
                let offset = u32::from(idx) << 4;
                let next = m.r16(table.wrapping_add(offset) + 6);
                m.w16(link, next);
                host.release_letter(m.r32(LETTERS).wrapping_add(offset));
                let free = m.r16(LETTERS + 4);
                let table = m.r32(LETTERS);
                m.w16(table.wrapping_add(offset) + 6, free);
                m.w16(LETTERS + 4, idx);
                return true;
            }
        }
        link = e + 6;
        ax = m.r16(e + 6);
        if ax >= count {
            return false;
        }
    }
}

/// `op_accept_tavern_script_offer`: `op+4` tarea, `+8` ranura (alto/bajo), `+0xC`/`+0x10` valores.
pub fn accept_script_offer(m: &mut dyn Mem, host: &mut dyn TavernOffersHost, op: u32) {
    let mc = u32::from(m.r16(MERCHANT_COUNT));
    let mut found = false;
    for i in 0..mc {
        if i == m.r32(op + 0x10) {
            let boss = merchant(m, i);
            let (a, b) = (m.r32(op + 4), m.r32(op + 8) & 0xff);
            found |= unlink_letter(m, host, boss + 0xA, a, b);
        }
    }
    let idx = m.r32(op + 4);
    if u32::from(m.r16(SCHED + 0xC)) <= idx || !found {
        return;
    }
    let task = get_task(m, idx);
    if m.r16(task + 6) != 0x1B {
        return;
    }
    let high = m.r32(op + 8) >> 8;
    if u32::from(m.r16(task + 0x12)) > high {
        let cell = m.r32(task + 0xC).wrapping_add(high << 2);
        let cur = m.r32(cell);
        if cur >= u32::from(m.r16(MERCHANT_COUNT)) {
            return;
        }
        let v = m.r32(op + 0x10);
        m.w32(cell, v);
    }
    let low = m.r32(op + 8) & 0xff;
    if u32::from(m.r16(task + 0x12)) <= low {
        return;
    }
    let cell = m.r32(task + 0xC).wrapping_add(low << 2);
    let v = m.r32(op + 0xC);
    m.w32(cell, v);
}

/// `op_buy_tavern_region_news`: cobra al comerciante `op+4` (`+0xC`), olvida noticias de las ciudades de la misma región que `op+0x10` y libera `op+8`.
pub fn buy_region_news(m: &mut dyn Mem, host: &mut dyn TavernOffersHost, op: u32) {
    let mc = u32::from(m.r16(MERCHANT_COUNT));
    let who = m.r32(op + 4);
    let mut towns = m.r32(TOWN_COUNT);
    if who < mc {
        let boss = merchant(m, who);
        let cost = m.r32(op + 0xC);
        let gold = m.r32(boss).wrapping_sub(cost);
        let spent = m.r32(boss + 0x4BC);
        m.w32(boss, gold);
        m.w32(boss + 0x4BC, spent.wrapping_add(cost));
        let from = m.r32(op + 0x10);
        towns = m.r32(TOWN_COUNT);
        if from < towns {
            let region_key = |m: &mut dyn Mem, t: u32| {
                let base = m.r32(TOWNS).wrapping_add(t.wrapping_mul(TOWN_STRIDE));
                let region = u32::from(m.r8(base + 0x2C1));
                m.r8(REGION_KEY_TABLE.wrapping_add(region.wrapping_mul(REGION_KEY_STRIDE)))
            };
            let key = region_key(m, from);
            if towns > 0 {
                let mut i = 0u32;
                loop {
                    if region_key(m, i) == key {
                        host.forget_news(i, who);
                        towns = m.r32(TOWN_COUNT);
                    }
                    i = (i + 1) & 0xff;
                    if i >= towns {
                        break;
                    }
                }
            }
        }
    }
    let slot = m.r32(op + 8);
    if slot < towns {
        let t = m.r32(TOWNS).wrapping_add(slot.wrapping_mul(TOWN_STRIDE));
        m.w8(t + 0x82C, 0xff);
    }
}
