//! `pirate_band_on_battle_end` (0x00514920): desmonta la cuadrilla al acabar el combate (convoy, piezas, comerciantes y misiones).
//! Traducción 1:1 sobre la memoria del original ([`Mem`]).
use crate::flatmem::Mem;

const SHIPS: u32 = 0x6DD7A4;
const SHIP_COUNT: u32 = 0x6DD894;
const CONVOYS: u32 = 0x6DD7A8;
const BANDS: u32 = 0x6DD7AC;

pub trait BattleEndHost {
    /// `0x00494A80` sobre `[0x6CE6D0]`; el argumento conserva en el original la parte alta de una dirección de pila
    /// (se registra enmascarado a 16 bits).
    fn convoy_notify(&mut self, convoy: u32);
    /// `0x00507FB0` (ECX = 0x6DD7A0): retira el barco de su convoy/flota.
    fn remove_ship(&mut self, ship: u32);
    /// `0x0050A3D0` (ECX = 0x6DD7A0): libera el convoy.
    fn release_convoy(&mut self, convoy: u32);
    /// `0x0054AA70` (ECX = 0x6DF2F0): publica el evento `[0xB4, ranura, nivel, rand, rand]`.
    fn post_event(&mut self, fields: [u32; 5]);
    /// `rand` del CRT (0x00639C03).
    fn rand(&mut self) -> u32;
    /// `operator delete` (0x0064F7E2).
    fn free(&mut self, p: u32);
    /// `0x004D8DB0` sobre la tabla `0x6DD73C`.
    fn object_at(&mut self, index: u32) -> u32;
}

pub fn band_on_battle_end(m: &mut dyn Mem, host: &mut dyn BattleEndHost, band: u32) {
    let mut flag = false;
    let convoy = u32::from(m.r16(band + 0x10));
    if convoy as u16 >= m.r16(0x6DD896) {
        // el original salta directamente a la liberación
    } else {
        if m.r32(0x6CE6D0) != 0 {
            host.convoy_notify(convoy & 0xffff);
        }
        let conv = m
            .r32(CONVOYS)
            .wrapping_add(u32::from(m.r16(band + 0x10)).wrapping_mul(0x3c));
        let mut cx = m.r16(conv + 0xa);
        while cx < m.r16(SHIP_COUNT) {
            let ship = m.r32(SHIPS).wrapping_add(u32::from(cx) * 0x180);
            let next = m.r16(ship + 6);
            m.w16(conv + 0xa, next);
            let size = m.r32(ship + 0x18) as i32;
            m.w16(ship + 8, 0xffff);
            if size > 0 {
                let head = m.r16(band + 0x12);
                flag = true;
                m.w16(ship + 6, head);
                m.w16(band + 0x12, cx);
            } else {
                m.w16(ship + 6, 0xffff);
                host.remove_ship(u32::from(cx));
            }
            cx = m.r16(conv + 0xa);
        }
    }
    host.release_convoy(u32::from(m.r16(band + 0x10)));
    let count = m.r16(band + 8);
    m.w16(band + 0x10, 0xffff);
    if count != 0 {
        let mut i = 0u16;
        loop {
            let p = m.r32(band + 4).wrapping_add(u32::from(i) * 8);
            let kind = m.r8(p + 3);
            if (kind == 0x30 || kind == 0x31) && m.r8(p + 6) < 0xc8 {
                flag = true;
            }
            i = m.r16(p + 4);
            if i >= count {
                break;
            }
        }
    }
    if flag {
        return;
    }
    m.w8(band + 0x15, 1);
    if m.r32(0x6DF31C) == 0 {
        let mut slot = 0u32;
        while slot < 5 && m.r32(BANDS + slot * 4) != 0 {
            slot += 1;
        }
        let level = u32::from(m.r8(band + 0x14));
        let r1 = host.rand();
        let r2 = host.rand();
        host.post_event([0xb4, slot, level, r1, r2]);
    }
    let n = m.r16(0x6DE4AA);
    if n != 0 {
        let mut b = 0u16;
        loop {
            let mer = m.r32(0x6DE4A0 + 0x78).wrapping_add(u32::from(b) * 0x650);
            if m.r8(mer + 0xe1) == m.r8(band + 0xe) {
                m.w8(mer + 0xe1, 0xff);
            }
            b = b.wrapping_add(1);
            if b >= m.r16(0x6DE4AA) {
                break;
            }
        }
    }
    let data = m.r32(band);
    if data != 0 {
        let inner = m.r32(data + 0x68);
        if inner != 0 {
            host.free(inner);
            m.w32(data + 0x68, 0);
        }
        host.free(data);
        m.w32(band, 0);
    }
    let mut own = 0xffffu32;
    let mut e = 0u16;
    loop {
        if m.r32(BANDS + u32::from(e) * 4) == band {
            own = u32::from(e);
            e = 5;
        }
        e += 1;
        if e >= 5 {
            break;
        }
    }
    let mut cx = m.r16(0x6DD744);
    if cx >= m.r16(0x6DD748) {
        return;
    }
    loop {
        let obj = host.object_at(u32::from(cx));
        cx = m.r16(obj + 4);
        if m.r16(obj + 6) == 0x32
            && m.r8(obj + 0xc) == 3
            && u16::from(m.r8(obj + 0x10)) == own as u16
            && m.r8(obj + 0x11) == m.r8(band + 0xe)
        {
            m.w8(obj + 0x12, 2);
            cx = 0xffff;
        }
        if cx >= m.r16(0x6DD748) {
            break;
        }
    }
}
