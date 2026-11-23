//! `sea_battle_get_pirate_captain_name` (0x00610B29) y `get_pirate_captain_name_for_ship` (0x00612EE0): nombre del capitán
//! pirata de un barco como `CString` devuelto por puntero. Traducción 1:1 sobre la memoria del original ([`Mem`]).
use crate::flatmem::Mem;

/// Cadena local (dirección sintética).
pub const LOCAL: u32 = 0x5e10_0000;

pub trait CaptainHost {
    /// `CString::Format` (0x0064DF80): `(destino, formato, argumentos…)`.
    fn format(&mut self, m: &mut dyn Mem, dst: u32, fmt: u32, args: &[u32]);
    /// Constructor desde literal (0x0064F390).
    fn from_lit(&mut self, m: &mut dyn Mem, dst: u32, lit: u32);
    /// Constructor de copia (0x0064EFC8): `dst = copia de src`.
    fn copy(&mut self, m: &mut dyn Mem, dst: u32, src: u32);
    /// Destructor (0x0064F253).
    fn dtor(&mut self, cstr: u32);
    /// `0x00512AE0` (ECX = 0x6DDAA0).
    fn text_a(&mut self, v: u32) -> u32;
    /// `0x00512A60` (ECX = 0x6DDAA0).
    fn text_b(&mut self, v: u32) -> u32;
    /// `0x006087F0` sobre un objeto de capitán: ¿el barco `ship` es suyo?
    fn captain_owns(&mut self, captain: u32, ship: u32) -> u32;
}

/// 0x00610B29: `this` = objeto de combate (`+0x1E90`/`+0x1E91` = códigos de nombre); `ret` = destino de la cadena.
pub fn sea_battle_get_pirate_captain_name(m: &mut dyn Mem, h: &mut dyn CaptainHost, this: u32, ret: u32) -> u32 {
    let b0 = m.r8(this + 0x1e90);
    if b0 == 0xff && m.r8(this + 0x1e91) == 0xff {
        let lit = m.r32(0x6A1780);
        h.from_lit(m, LOCAL, lit);
    } else {
        let a = u32::from(m.r8(this + 0x1e91));
        let a = h.text_a(a);
        let b = u32::from(m.r8(this + 0x1e90));
        let b = h.text_b(b);
        let x = m.r32(0x69C98C);
        let f = 0x6C7708;
        h.format(m, LOCAL, f, &[x, b, a]);
    }
    h.copy(m, ret, LOCAL);
    h.dtor(LOCAL);
    ret
}

/// 0x00612EE0: `this` = lista de capitanes (`+0x404` cuenta, punteros desde `+0`), `ship` = índice de barco.
pub fn get_pirate_captain_name_for_ship(
    m: &mut dyn Mem,
    h: &mut dyn CaptainHost,
    this: u32,
    ret: u32,
    ship: u32,
) -> u32 {
    let ships = m.r32(0x6DD7A4);
    let mut sp = ship.wrapping_mul(0x180).wrapping_add(ships);
    let count = m.r8(this + 0x404);
    let mut i = 0u32;
    while i < u32::from(count) {
        let c = m.r32(this + i * 4);
        if c != 0 && h.captain_owns(c, ship) != 0 {
            sea_battle_get_pirate_captain_name(m, h, c, ret);
            h.dtor(LOCAL);
            return ret;
        }
        i += 1;
        if i as u8 >= m.r8(this + 0x404) {
            break;
        }
    }
    let conv = m.r16(sp + 8);
    if conv < m.r16(0x6DD896) {
        let e = m.r32(0x6DD7A8).wrapping_add(u32::from(conv) * 0x3c);
        let s = u32::from(m.r16(e + 0x10));
        sp = s.wrapping_mul(0x180).wrapping_add(m.r32(0x6DD7A4));
    }
    let tr = m.r16(sp + 0x42);
    if tr < m.r16(0x6DD892) {
        let t = m.r32(0x6DD7A0).wrapping_add(u32::from(tr) << 4);
        let a = u32::from(m.r8(t + 3));
        let a = h.text_a(a);
        let b = u32::from(m.r8(t + 2));
        let b = h.text_b(b);
        let x = m.r32(0x69C98C);
        let f = 0x6C77B8;
        h.format(m, LOCAL, f, &[x, b, a]);
    } else {
        let lit = m.r32(0x6A1780);
        h.from_lit(m, LOCAL, lit);
    }
    h.copy(m, ret, LOCAL);
    h.dtor(LOCAL);
    ret
}
