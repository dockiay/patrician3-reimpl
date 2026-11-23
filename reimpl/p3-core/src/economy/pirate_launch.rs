//! `op_launch_pirate_fleet` (0x00514D40): saca de la lista de la flota el primer barco listo de un comerciante-pirata,
//! avisa por carta y, si no hay nadie que lo reciba, reúne los barcos sueltos en un convoy nuevo.
//! Traducción 1:1 sobre la memoria del original ([`Mem`]); las llamadas a otros subsistemas pasan por [`LaunchHost`].
use crate::flatmem::Mem;

const SHIPS: u32 = 0x6DD7A4;
const SHIP_COUNT: u32 = 0x6DD894;
const TRADERS: u32 = 0x6DD7A0;
const TRADER_COUNT: u32 = 0x6DD892;
const CONVOYS: u32 = 0x6DD7A8;
const MERCHANT_COUNT: u32 = 0x6DE4AA;
const MERCHANTS: u32 = 0x6DE4A0 + 0x78;
const TICK: u32 = 0x6DE4B4;
const KINDS: u32 = 0x6DDBB0;

pub trait LaunchHost {
    /// `0x005098B0(0x6DD7A0, &barco.trader)`.
    fn trader_cleanup(&mut self, ship_trader_field: u32);
    /// `0x0046C340(rng, tick)`.
    fn rng_seed(&mut self, tick: u32);
    /// `0x0046C380(rng, n)`: valor aleatorio en `0..n` (byte bajo).
    fn rng_next(&mut self, n: u32) -> u8;
    /// `0x004F9430(comerciante)`.
    fn merchant_byte(&mut self, merchant: u32) -> u8;
    /// `0x004D6530(0x6DD730, -2, carta)`.
    fn send_letter(&mut self, record: &[u8; 16]);
    /// `0x004D8CF0(0x6DD73C, vencimiento)`: dirección de la tarea nueva.
    fn new_task(&mut self, m: &mut dyn Mem, due: u32) -> u32;
    /// `0x00516840(barco, arg)`; `arg` solo se compara en 16 bits.
    fn ship_detach(&mut self, ship: u32, arg: u16);
    /// `0x0050D040(0x6DD7A0, índice)`.
    fn ship_release(&mut self, index: u16);
    /// `0x00453DF0(objeto global 0x6CBDD8)`.
    fn notify(&mut self, object: u32);
    /// `0x00507FB0(0x6DD7A0, índice)`.
    fn ship_finish(&mut self, index: u16);
    /// `0x005062F0(0x6DD7A0, &salida, índice)`: crea el convoy y devuelve su índice.
    fn create_convoy(&mut self, m: &mut dyn Mem, ship: u16) -> u16;
    /// `0x00515750(índice, &temporal)` (cdecl).
    fn ship_setup(&mut self, index: u16);
    /// `0x004D4DD0(0x6DD728, ranura, 1)`.
    fn free_slot(&mut self, slot: u16);
    /// `0x00501BE0(convoy)`.
    fn convoy_clear(&mut self, convoy: u32);
    /// `0x0050AAE0(0x6DD7A0, convoy)`.
    fn convoy_done(&mut self, index: u32);
    /// `0x005182B0(barco)`.
    fn ship_update(&mut self, ship: u32);
}

fn ship_addr(m: &mut dyn Mem, idx: u32) -> u32 {
    m.r32(SHIPS).wrapping_add((idx & 0xffff).wrapping_mul(0x180))
}

/// `this`: objeto con la lista de barcos en `+0x12` (índices enlazados por `barco+6`, fin 0xFFFF) y el tipo en `+0xE`.
/// `limit` (primer argumento): máximo de barcos a reunir en el convoy; el segundo argumento no se usa.
pub fn launch_pirate_fleet(m: &mut dyn Mem, host: &mut dyn LaunchHost, this: u32, limit: u32) {
    let head = this + 0x12;
    let mut link = head;
    let mut saved: u16 = 0xffff;
    let mut valid: u32 = 0;
    let s = m.r16(SHIP_COUNT);
    let mut cur = m.r16(link);
    if cur < s {
        loop {
            let ship = ship_addr(m, u32::from(cur));
            let tfield = ship + 0x42;
            let tidx = m.r16(tfield);
            'next: {
                if tidx >= m.r16(TRADER_COUNT) {
                    break 'next;
                }
                let trader = m.r32(TRADERS).wrapping_add(u32::from(tidx) << 4);
                if m.r8(trader + 8) <= 0x20 {
                    host.trader_cleanup(tfield);
                    break 'next;
                }
                if (m.r32(ship + 0x18) as i32) < (m.r32(ship + 0x14) as i32) {
                    break 'next;
                }
                // Desenlaza el barco de la lista.
                let nxt = m.r16(ship + 6);
                saved = m.r16(link);
                m.w16(link, nxt);
                m.w16(ship + 6, 0xffff);
                let mi = u16::from(m.r8(ship + 0x15d));
                valid = u32::from(mi < m.r16(MERCHANT_COUNT));
                if valid == 0 {
                    break 'next;
                }
                let tick = m.r32(TICK);
                if m.r32(ship + 0x4c) >= tick {
                    break 'next;
                }
                // Hay barco listo: elige un comerciante con barcos en cola y avisa.
                host.rng_seed(tick);
                let count = u32::from(m.r16(MERCHANT_COUNT));
                let mut nz = 0u8;
                if count != 0 {
                    let mut i = 0u8;
                    loop {
                        let mer = m.r32(MERCHANTS).wrapping_add(u32::from(i).wrapping_mul(0x650));
                        if m.r16(mer + 8) != 0 {
                            nz = nz.wrapping_add(1);
                        }
                        i = i.wrapping_add(1);
                        if u32::from(i) >= count {
                            break;
                        }
                    }
                }
                let mut k = host.rng_next(u32::from(nz));
                let mut i = 0u8;
                loop {
                    let mer = m.r32(MERCHANTS).wrapping_add(u32::from(i).wrapping_mul(0x650));
                    if m.r16(mer + 8) == 0 {
                        k = k.wrapping_add(1);
                    }
                    i = i.wrapping_add(1);
                    if i > k {
                        break;
                    }
                }
                let mer = m.r32(MERCHANTS).wrapping_add(u32::from(k).wrapping_mul(0x650));
                let trader = m.r32(TRADERS).wrapping_add(u32::from(m.r16(ship + 0x42)) << 4);
                let mut rec = [0u8; 16];
                rec[1] = 0x80;
                rec[2] = 0xff;
                rec[3] = 0xff;
                rec[4] = 0x2a;
                rec[5] = m.r8(mer + 0x19);
                rec[8] = m.r8(mer + 0x1a);
                rec[9] = host.merchant_byte(mer);
                let h = u32::from(m.r8(mer + 0x19));
                rec[10] = m.r8(mer + h + 0x39c);
                rec[11] = m.r8(mer + 0x1c);
                rec[12] = m.r8(mer + 0x1d);
                rec[13] = m.r8(trader + 2);
                rec[14] = m.r8(trader + 3);
                rec[15] = 0;
                host.send_letter(&rec);
                if host.rng_next(10) == 0 {
                    let task = host.new_task(m, tick.wrapping_add(0x300));
                    m.w16(task + 6, 5);
                    let sm = m.r8(ship + 0x15d);
                    m.w8(task + 8, sm);
                    let merc2 = m.r32(MERCHANTS).wrapping_add(u32::from(sm).wrapping_mul(0x650));
                    let al = m.r8(merc2 + 0x19);
                    m.w8(task + 9, al);
                    m.w8(task + 0xa, k);
                    m.w8(task + 0xb, al);
                    let t = m.r32(TICK);
                    m.w8(task + 0x10, 4);
                    m.w32(task + 0xc, t);
                    m.w8(task + 0x11, 0);
                    let w = m.r16(ship + 0x15e);
                    m.w16(task + 0x16, w);
                }
                host.ship_detach(ship, saved);
                host.ship_release(saved);
                let obj = m.r32(0x6CBDD8);
                host.notify(obj);
                let f = m.r8(ship + 0x3c) | 0x20;
                m.w8(ship + 0x3c, f);
                m.w16(ship + 0x134, 0x11);
                m.w32(ship + 0x18, 0);
                m.w16(ship + 0x50, 0xffff);
                m.w16(ship + 0x52, 0xffff);
                host.ship_finish(saved);
                return;
            }
            // 0x51506c: siguiente barco.
            let nxt = m.r16(ship + 6);
            link = ship + 6;
            cur = nxt;
            if cur >= m.r16(SHIP_COUNT) {
                break;
            }
        }
    }
    if saved >= m.r16(SHIP_COUNT) {
        return;
    }
    // Hubo un barco desenlazado pero sin aviso: crea un convoy y reúne los barcos sueltos.
    let c = host.create_convoy(m, saved);
    let convoy = m.r32(CONVOYS).wrapping_add(u32::from(c).wrapping_mul(0x3c));
    let mut cnt: u16 = 1;
    if valid == 0 {
        let mut link = head;
        let mut cur = m.r16(link);
        let s = m.r16(SHIP_COUNT);
        if cur < s {
            let lim = limit as u16;
            if cnt < lim {
                loop {
                    let ship = ship_addr(m, u32::from(cur));
                    let ready = (m.r32(ship + 0x18) as i32) >= (m.r32(ship + 0x14) as i32);
                    if ready && m.r16(ship + 0x42) >= m.r16(TRADER_COUNT) {
                        let nxt = m.r16(ship + 6);
                        m.w16(link, nxt);
                        let first = m.r16(convoy + 0xa);
                        m.w16(ship + 6, first);
                        cnt = cnt.wrapping_add(1);
                        m.w16(convoy + 0xa, cur);
                    } else {
                        link = ship + 6;
                    }
                    cur = m.r16(link);
                    if cur >= m.r16(SHIP_COUNT) {
                        break;
                    }
                    if cnt >= lim {
                        break;
                    }
                }
            }
        }
    }
    let mut bp = m.r16(convoy + 0xa);
    if bp < m.r16(SHIP_COUNT) {
        loop {
            let ship = ship_addr(m, u32::from(bp));
            let kind = u32::from(m.r8(this + 0xe));
            let v1e = m.r16(KINDS + 0x34 * kind);
            m.w16(ship + 0x1e, v1e);
            let v22 = m.r16(KINDS + 4 + 0x34 * kind);
            m.w16(ship + 0x1c, 0);
            m.w16(ship + 0x22, v22);
            m.w16(ship + 0x20, 0);
            m.w16(ship + 0x3e, 0x700);
            m.w16(ship + 0x134, 0x12);
            m.w8(ship + 0x15c, 0);
            host.ship_setup(bp);
            let slot = m.r16(ship + 0x132);
            m.w8(ship, 0xff);
            m.w8(ship + 1, 0);
            m.w16(ship + 8, c);
            m.w32(ship + 0x124, 0);
            m.w32(ship + 0x128, 0);
            m.w32(ship + 0x12c, 0);
            m.w16(ship + 0x130, 0);
            m.w8(ship + 0x136, 0);
            m.w8(ship + 0x13b, 0);
            m.w8(ship + 0x13a, 0);
            if slot <= m.r16(0x6DD72A) {
                host.free_slot(slot);
                m.w16(ship + 0x132, 0xffff);
            }
            let s = m.r16(SHIP_COUNT);
            if m.r16(ship + 0xa) < s || m.r16(ship + 0xc) < s {
                host.ship_release(bp);
            }
            m.w32(ship + 0x28, 0);
            m.w32(ship + 0x2c, 0);
            m.w32(ship + 0x30, 0);
            m.w8(ship + 0x37, 0);
            m.w8(ship + 0x38, 0xff);
            m.w8(ship + 0x39, 0xff);
            m.w16(ship + 0x3a, 0xffff);
            host.ship_detach(ship, bp);
            m.w16(ship + 0x138, 0);
            let t = m.r32(TICK);
            m.w32(ship + 0x44, t);
            m.w32(ship + 0x48, 0x7fff_ffff);
            m.w16(ship + 0x50, 0xffff);
            m.w16(ship + 0x52, 0xffff);
            host.ship_update(ship);
            bp = m.r16(ship + 6);
            if bp >= m.r16(SHIP_COUNT) {
                break;
            }
        }
    }
    m.w8(convoy, 0xff);
    m.w16(convoy + 0x12, 0x12);
    m.w16(convoy + 0x14, 2);
    m.w16(convoy + 0x16, 0);
    m.w32(convoy + 0x18, 0xffff);
    m.w16(convoy + 0x1c, 0xff);
    m.w8(convoy + 0x1e, 0);
    m.w8(convoy + 0x1f, 0xff);
    host.convoy_clear(convoy);
    host.convoy_done(u32::from(c));
}
