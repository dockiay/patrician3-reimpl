//! Barcos de la cuadrilla pirata: reposición (0x005144A0), retrato del líder (0x00515AE0), ruina de una pieza (0x00515C00)
//! y alta de barco (0x00516170). Traducción 1:1 sobre la memoria del original ([`Mem`]).
use crate::flatmem::Mem;

const SHIPS: u32 = 0x6DD7A4;
const SHIP_COUNT: u32 = 0x6DD894;
const TRADERS: u32 = 0x6DD7A0;
const TRADER_COUNT: u32 = 0x6DD892;
const KINDS: u32 = 0x6DDBB0;

pub trait ShipsHost {
    /// `0x00516170(banda, a0, a1, a2)`.
    fn create_ship(&mut self, band: u32, a0: u32, a1: u32, a2: u32) -> u32;
    /// `0x00599040(a, b)` (stdcall).
    fn piece_gone(&mut self, a: u32, b: u32);
    /// `0x005891C0(a, b, restante, 0x32, 1)` (stdcall).
    fn piece_ruin_effect(&mut self, a: u32, b: u32, remaining: u32);
    /// `0x00507D60(0x6DD7A0, &índice, 0, 3)`: reserva un barco; devuelve (dirección, índice).
    fn alloc_ship(&mut self, m: &mut dyn Mem) -> (u32, u16);
    /// `0x0052A8D0(ciudades, barco, a0)`.
    fn ship_place(&mut self, ship: u32, a0: u32);
    /// `0x005181D0(barco)`.
    fn ship_init(&mut self, ship: u32);
    /// `0x00509A50(0x6DD7A0, barco)`.
    fn ship_register(&mut self, ship: u32);
    /// `0x0051A750(barco, 1)`.
    fn ship_flag(&mut self, ship: u32);
    /// `0x0052BB00(ciudad)`.
    fn town_value(&mut self, town: u32) -> u32;
    /// `0x0051A4E0(barco, a2 & 0xFFFF, valor)` (stdcall).
    fn ship_assign(&mut self, ship: u32, a2: u32, value: u32);
    /// `0x005184F0(barco)`: devuelve un valor de 16 bits.
    fn ship_capacity(&mut self, ship: u32) -> u16;
    /// `0x005182B0(barco)`.
    fn ship_update(&mut self, ship: u32);
    /// `0x005097C0(0x6DD7A0, 1)`: devuelve un valor de 16 bits.
    fn trader_new(&mut self) -> u16;
}

/// `pirate_band_replenish_reserve_ships` (0x005144A0).
pub fn band_replenish_reserve_ships(m: &mut dyn Mem, host: &mut dyn ShipsHost, band: u32, arg: u32) {
    if (arg as u16) != u16::from(m.r8(band + 0xe)) || m.r32(band) != 0 {
        return;
    }
    let obj = m.r32(0x6CC3E8);
    let kind = u32::from(m.r8(obj + 0x13));
    let mut left = i32::from(m.r8(0x673658 + kind));
    let s = u32::from(m.r16(SHIP_COUNT));
    let mut cur = u32::from(m.r16(band + 0x12));
    if cur < s {
        loop {
            let ship = m.r32(SHIPS).wrapping_add(cur * 0x180);
            left -= 1;
            cur = u32::from(m.r16(ship + 6));
            if cur >= s {
                break;
            }
        }
    }
    // EAX conserva la parte alta de lo que devolvió la llamada anterior (el original no la limpia).
    let mut hi = 0u32;
    while left > 0 {
        let n = u32::from(m.r8(band + 0x14));
        let a0 = hi | u32::from(m.r8(0x6DE4A4).wrapping_add(left as u8) & 3);
        let a2 = (hi | n).wrapping_mul(40).wrapping_add(0x64);
        hi = host.create_ship(band, a0, 1, a2) & 0xffff_0000;
        left -= 1;
    }
}

/// `pirate_band_get_leader_portrait` (0x00515AE0): aspecto (+2, +3) del primer barco con comerciante pirata, o `('-', [0x6DDB74])`.
pub fn band_leader_portrait(m: &mut dyn Mem, band: u32) -> (bool, u8, u8) {
    let s = u32::from(m.r16(SHIP_COUNT));
    let mut cur = u32::from(m.r16(band + 0xc));
    if cur < s {
        loop {
            let ship = m.r32(SHIPS).wrapping_add(cur * 0x180);
            let t = m.r16(ship + 0x42);
            if t < m.r16(TRADER_COUNT) {
                let trader = m.r32(TRADERS).wrapping_add(u32::from(t) << 4);
                if m.r8(trader + 8) > 0x20 {
                    return (true, m.r8(trader + 2), m.r8(trader + 3));
                }
            }
            cur = u32::from(m.r16(ship + 4));
            if cur >= u32::from(m.r16(SHIP_COUNT)) {
                break;
            }
        }
    }
    (false, 0x2d, m.r8(0x6DDB74))
}

/// `pirate_band_ruin_piece_at` (0x00515C00): envejece la primera pieza de tipo 0x30/0x31 en (`x`,`y`); la quita al llegar a 0xFA.
pub fn band_ruin_piece_at(m: &mut dyn Mem, host: &mut dyn ShipsHost, band: u32, a0: u32, x: u32, y: u32) -> bool {
    let table = m.r32(band + 4);
    let count = m.r16(band + 8);
    let mut link = table + 4;
    let mut cur = m.r16(link);
    if cur >= count {
        return false;
    }
    loop {
        let piece = table.wrapping_add(u32::from(cur) * 8);
        if u32::from(m.r8(piece)) == x && u32::from(m.r8(piece + 1)) == y && matches!(m.r8(piece + 3), 0x30 | 0x31) {
            break;
        }
        link = piece + 4;
        cur = m.r16(link);
        if cur >= count {
            return false;
        }
    }
    let idx = u32::from(m.r16(link));
    let age_addr = table.wrapping_add(idx * 8 + 6);
    let age = m.r8(age_addr);
    m.w8(age_addr, if age > 0xc8 { age.wrapping_add(1) } else { 0xc9 });
    let piece = m.r32(band + 4).wrapping_add(u32::from(m.r16(link)) * 8);
    let age = m.r8(piece + 6);
    let global = m.r32(0x6E51AC);
    let same = global != 0 && m.r32(global + 0xc324) == a0;
    if age >= 0xfa {
        let next = m.r16(piece + 4);
        m.w16(link, next);
        if same {
            host.piece_gone(x, y);
        }
        return false;
    }
    if same {
        host.piece_ruin_effect(x, y, 0xfa - u32::from(age));
    }
    true
}

/// `pirate_band_create_ship` (0x00516170).
pub fn band_create_ship(m: &mut dyn Mem, host: &mut dyn ShipsHost, band: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    let (ship, idx) = host.alloc_ship(m);
    let first = m.r16(band + 0xc);
    m.w16(ship + 4, first);
    let head = m.r16(band + 0x12);
    m.w16(band + 0xc, idx);
    m.w16(ship + 6, head);
    m.w16(band + 0x12, idx);
    host.ship_place(ship, a0);
    host.ship_init(ship);
    host.ship_register(ship);
    host.ship_flag(ship);
    let towns = m.r32(0x6DE508);
    let town = towns.wrapping_add(u32::from(m.r8(0x6DE52E)).wrapping_mul(0x9f8));
    let v = host.town_value(town);
    host.ship_assign(ship, a2 & 0xffff, v);
    let cap = host.ship_capacity(ship);
    m.w16(ship + 0x40, cap);
    m.w16(ship + 0x154, cap);
    host.ship_update(ship);
    let v = m.r32(ship + 0x14);
    m.w8(ship, 0xff);
    m.w8(ship + 0x13a, 0);
    m.w32(ship + 0x18, v);
    let kind = u32::from(m.r8(band + 0xe));
    let w = m.r16(KINDS + 0x34 * kind);
    m.w16(ship + 0x1e, w);
    let w = m.r16(KINDS + 4 + 0x34 * kind);
    m.w16(ship + 0x22, w);
    if a1 as u8 != 0 {
        let t = host.trader_new();
        m.w16(ship + 0x42, t);
    }
    ship
}
