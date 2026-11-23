//! `spawn_pirate_ship_near_ship` (0x005063C0): método del gestor de barcos (0x6DD7A0) que crea un barco pirata cerca de otro.
//! Traducción 1:1 sobre la memoria del original ([`Mem`]); los subsistemas pasan por [`SpawnHost`].
use super::pirate_small;
use crate::flatmem::Mem;

pub trait SpawnHost {
    /// `0x00507D60(gestor, &índice, 0, 3)`: reserva un barco; devuelve (dirección, índice).
    fn alloc_ship(&mut self, m: &mut dyn Mem) -> (u32, u16);
    /// `0x0052A8D0(ciudades, barco, clase)`; devuelve EAX (el original usa su parte alta después).
    fn ship_place(&mut self, ship: u32, class: u32) -> u32;
    /// `0x0051A750(barco, valor)`.
    fn ship_flag(&mut self, ship: u32, value: u32);
    /// `0x00512B20(0x6DDAA0, índice)`: dirección de la cadena del nombre (terminada en 0).
    fn name_ptr(&mut self, index: u16) -> u32;
    /// `0x005185C0(barco)`: valor de 16 bits.
    fn ship_capacity(&mut self, ship: u32) -> u16;
    /// `0x005097C0(gestor, 1)`: comerciante nuevo (16 bits).
    fn trader_new(&mut self) -> u16;
    /// `0x00515750(índice, &temporal)` (cdecl).
    fn ship_setup(&mut self, index: u16);
    /// `0x005182B0(barco)`.
    fn ship_update(&mut self, ship: u32);
    /// `operator new` (0x0064F7B9).
    fn alloc(&mut self, m: &mut dyn Mem, size: u32) -> u32;
    /// `0x00513D80(cuadrilla, a, b, c)`.
    fn band_setup(&mut self, band: u32, a: u32, b: u32, c: u32);
    /// `0x005062F0(gestor, &salida(+0xEC), índice)`: crea el convoy y escribe su índice en `salida`.
    fn create_convoy(&mut self, m: &mut dyn Mem, out: u32, ship: u16);
    /// `0x0050CE50(gestor, índice, +0xEA)`.
    fn convoy_finish(&mut self, index: u16, value: u16);
}

/// `arg0`: barco de referencia; `arg1`: modo (se usa `arg1 % 3`). Devuelve el índice del barco nuevo.
pub fn spawn_pirate_ship_near_ship(m: &mut dyn Mem, host: &mut dyn SpawnHost, this: u32, arg0: u32, arg1: u32) -> u16 {
    let count = m.r16(this + 0xf4);
    let (x, y) = if (arg0 as u16) < count {
        let r = m.r32(this + 4).wrapping_add((arg0 & 0xffff).wrapping_mul(0x180));
        (m.r16(r + 0x1e), m.r16(r + 0x22))
    } else {
        (0x3d2u16, 0x317u16)
    };
    let mode = (arg1 & 0xffff) % 3;
    let class: u32 = if mode == 0 {
        0
    } else if mode == 1 {
        1
    } else if (arg0 as u16) >= count {
        3
    } else if pirate_small::point_in_pirate_zone_box(m, x, y) {
        1
    } else {
        3
    };
    let (ship, idx) = host.alloc_ship(m);
    let place_eax = host.ship_place(ship, class);
    let v = m.r32(ship + 0x14);
    m.w32(ship + 0x18, v);
    // SBB AL,AL + ADD EAX,2 sobre el EAX que dejó la llamada anterior.
    let flag = (place_eax & 0xffff_ff00).wrapping_add(if mode < 2 { 0x101 } else { 2 });
    host.ship_flag(ship, flag);
    // Número de nombre: avanza una semilla de 16 bits hasta caer por debajo del límite.
    let limit = m.r16(0x6DDB78);
    let step = m.r16(0x6DD890);
    let mut v = m.r16(0x6DD88E);
    loop {
        v = ((u32::from(v.wrapping_add(step))) % 0x133) as u16;
        if v < limit {
            break;
        }
    }
    m.w16(0x6DD88E, v);
    m.w16(ship + 0x15e, v);
    let name = host.name_ptr(m.r16(0x6DD88E));
    let mut dst = ship + 0x160;
    let mut src = name;
    let mut c = m.r8(src);
    let mut n = 0u32;
    while c != 0 {
        n += 1;
        if n >= 0x20 {
            break;
        }
        m.w8(dst, c);
        c = m.r8(src + 1);
        dst += 1;
        src += 1;
    }
    m.w8(dst, 0);
    let f = m.r8(ship + 0x3c) | 1;
    m.w8(ship + 0x3c, f);
    let cap = host.ship_capacity(ship);
    m.w16(ship + 0x40, cap);
    let t = host.trader_new();
    m.w16(ship + 0x42, t);
    let prod = i16::from((mode as u8 + 2) as i8) * 0x33;
    let traders = m.r32(this);
    m.w8(traders.wrapping_add(u32::from(t) << 4) + 0xb, prod as u8);
    host.ship_setup(idx);
    host.ship_update(ship);
    let mut i = 0u32;
    while i < 5 {
        let group = m.r32(this + 0xc + 4 * i);
        if group != 0 {
            pirate_small::band_append_ship(m, group, u32::from(idx));
            pirate_small::ship_set_pirate_band_index(m, ship, i);
            i = 5;
        } else if i == 4 {
            let block = host.alloc(m, 0x18);
            let g = if block != 0 {
                pirate_small::band_init(m, block);
                block
            } else {
                0
            };
            m.w32(this + 0x1c, g);
            let towns = m.r32(0x6DE508);
            let obj = m.r32(0x6CC3E8);
            let tick = m.r32(0x6DE4B4);
            let a = m.r32(towns + 4).wrapping_add(tick);
            let b = u32::from(m.r8(obj + 0x13));
            host.band_setup(g, tick % 0x28, b, a);
            i = 3;
        }
        i += 1;
    }
    let f = m.r8(ship + 0x3d) | 0x20;
    m.w8(ship + 0x3d, f);
    m.w16(ship + 0x1e, x);
    m.w16(ship + 0x134, 0x12);
    m.w16(ship + 0x1c, 0x7fff);
    m.w16(ship + 0x20, 0x7fff);
    m.w16(ship + 0x22, y);
    m.w16(ship + 0x50, arg0 as u16);
    m.w16(ship + 0x52, arg0 as u16);
    host.create_convoy(m, this + 0xec, idx);
    let cidx = u32::from(m.r16(this + 0xec));
    let convoys = m.r32(this + 8);
    m.w16(convoys.wrapping_add(cidx * 0x3c) + 0x16, 0);
    let cidx = u32::from(m.r16(this + 0xec));
    let convoys = m.r32(this + 8);
    let flags = convoys.wrapping_add(cidx * 0x3c) + 0x14;
    let f = m.r8(flags) | 0x40;
    m.w8(flags, f);
    m.w16(ship + 0x138, 0);
    let v = m.r16(this + 0xea);
    host.convoy_finish(idx, v);
    let f = m.r8(ship + 0x136) | 0xc0;
    m.w8(ship + 0x136, f);
    idx
}
