//! Aparición de las figuras del asalto: método virtual `+0x40(x, y, rumbo, enlace, variante)` de cada clase y el
//! inicio común del caminante `town_walker_init` (`0x0055E620`).
//!
//! La variante lleva la ciudad (bits 0..6), una bandera (bit 7: bando, o «en la cola de la puerta» del caminante
//! común `+0x59`), un bit 8 (lancero ya en su puesto) y la clase del conjunto de sprites (bits 16..23). Cada clase
//! elige su conjunto entre los de su tabla `{cuántos, conjuntos, clases}` con muestreo de depósito.
use super::classes::pikeman_acquire;
use super::sprite::{anim_select, walker_insert, walker_remove};
use super::walker::set_mark;
use super::{s16, siege_active, siege_of, FigureHost, DIRS, TOWNS};
use crate::economy::siege_battle::{building_record, propagate_queue_rank, step_hits_gate};
use crate::flatmem::Mem;

/// Tabla de conjuntos de una clase: cuántos, puntero a los conjuntos y puntero a la clase de cada uno.
#[derive(Clone, Copy)]
pub struct SetTable {
    pub count: u32,
    pub sets: u32,
    pub kinds: u32,
}

/// Arqueros (clase 9), ballesteros (`0xA`), arcabuceros (`0xB`), infantería (`0x23`), jinetes (`0x24`), lanceros
/// (`0x20`) y luchadores de la cola (8).
pub const ARCHERS: SetTable = SetTable { count: 0x006E_174C, sets: 0x006E_1750, kinds: 0x006E_1754 };
pub const CROSSBOWS: SetTable = SetTable { count: 0x006E_16E0, sets: 0x006E_16E4, kinds: 0x006E_16E8 };
pub const GUNNERS: SetTable = SetTable { count: 0x006E_2DD0, sets: 0x006E_2DD4, kinds: 0x006E_2DD8 };
pub const INFANTRY: SetTable = SetTable { count: 0x006E_1854, sets: 0x006E_1858, kinds: 0x006E_185C };
pub const RIDERS: SetTable = SetTable { count: 0x006E_2D8C, sets: 0x006E_2D90, kinds: 0x006E_2D94 };
pub const PIKEMEN: SetTable = SetTable { count: 0x006E_2DAC, sets: 0x006E_2DB0, kinds: 0x006E_2DB4 };
pub const FIGHTERS: SetTable = SetTable { count: 0x006E_2E58, sets: 0x006E_2E5C, kinds: 0x006E_2E60 };

fn set_at(m: &mut dyn Mem, t: SetTable, i: u32) -> u32 {
    if i < m.r32(t.count) {
        let p = m.r32(t.sets);
        m.r32(p.wrapping_add(i * 4))
    } else {
        0
    }
}

fn kind_at(m: &mut dyn Mem, t: SetTable, i: u32) -> u8 {
    let p = m.r32(t.kinds);
    m.r8(p.wrapping_add(i))
}

/// Muestreo de depósito sobre la tabla: escribe en `out` el índice elegido entre los que cumplen `fit`.
fn sample(m: &mut dyn Mem, h: &mut dyn FigureHost, t: SetTable, out: u32, fit: &dyn Fn(&mut dyn Mem, u32) -> bool) {
    let mut seen = 0u8;
    let mut b = 0u8;
    while u32::from(b) < m.r32(t.count) {
        if fit(m, u32::from(b)) {
            if seen == 0 || (h.rand(m) as i32).wrapping_rem(i32::from(seen) + 1) == 0 {
                m.w8(out, b);
            }
            seen = seen.wrapping_add(1);
        }
        b = b.wrapping_add(1);
    }
}

/// Argumentos del inicio común `0x0055E620`.
pub struct WalkerInit {
    pub x: u32,
    pub y: u32,
    pub heading: u8,
    pub set: u32,
    pub link: u16,
    pub city: u8,
    pub mark: u8,
    pub queued: u8,
    pub side: u8,
    pub tag: u8,
}

/// `town_walker_init` (`0x0055E620(ecx = figura, x, y, rumbo, conjunto, enlace, ciudad, marca, cola, bando, etiqueta)`):
/// posición, enlace, animación 2, casilla (`+0x6C/+0x6E`, marcada en la escena con `marca`); en la cola de la
/// puerta (`+0x59`) se orienta hacia la puerta y entra en la lista del contenedor (`+0xC`/`+0x10` según el lado
/// `+0x5B`, el de fuera si la fila pasa de `escena+0xA`), y sin asedio busca un rumbo con pieza de puerta. Después
/// reglas de movimiento, tabla de enlaces, alta en la vista y desvíos al azar (`+0x82`, `+0x84`, `+0x86`).
pub fn walker_init(m: &mut dyn Mem, h: &mut dyn FigureHost, e: u32, a: &WalkerInit) {
    m.w8(e + 0x14, a.heading);
    m.w8(e + 0x59, a.queued);
    m.w32(e + 0xC, a.x);
    m.w16(e + 0x16, a.link);
    m.w32(e + 0x10, a.y);
    m.w16(e + 0x38, 0);
    m.w16(e + 0x3A, 0);
    m.w8(e + 0x6A, a.city);
    let f = anim_select(m, e + 0x2C, a.set, 2);
    m.w8(e + 0x15, f);
    m.w8(e + 0x7B, a.side);
    m.w32(e + 0x30, 0);
    let (px, py) = (s16(m, e + 0xE), s16(m, e + 0x12));
    let (col, row) = h.pixel_to_cell(m, px, py);
    let rec = siege_of(m, a.city);
    let s = m.r32(rec + 8);
    if s != 0 && m.r32(s + 0x70) != 0 {
        set_mark(m, s, col, row, a.mark);
    }
    m.w16(e + 0x6C, col as u16);
    m.w16(e + 0x6E, row as u16);
    if m.r8(e + 0x59) != 0 {
        if siege_active(m, rec) {
            let s = m.r32(rec + 8);
            let town = m.r32(TOWNS).wrapping_add(u32::from(a.city) * 0x9F8);
            let k = u32::from(m.r8(s + 0x6D));
            let gate = m.r32(s + 0x244 + 4 * k);
            let b = building_record(m, town, gate);
            let wall = m.r8(b + 3) == 0x38;
            if row <= u32::from(m.r16(s + 0xA)) {
                m.w8(e + 0x5B, 0);
                m.w8(e + 0x14, if wall { DIRS[7] } else { DIRS[1] });
            } else {
                m.w8(e + 0x5B, 1);
                m.w8(e + 0x14, if wall { DIRS[3] } else { DIRS[5] });
            }
            m.w8(e + 0x5A, 0);
        } else {
            m.w8(e + 0x5B, 0);
            m.w8(e + 0x5A, 0);
            let r = h.rand(m) as u8;
            let mut d = ((r & 3) << 6).wrapping_add(0x20);
            m.w8(e + 0x14, d);
            let mut tries = 0u8;
            while !step_hits_gate(m, h, col, row, d) {
                d = d.wrapping_add(0x40);
                tries += 1;
                m.w8(e + 0x14, d);
                if tries == 4 {
                    break;
                }
            }
        }
        let slot = rec + 0xC + 4 * u32::from(m.r8(e + 0x5B));
        let head = m.r32(slot);
        m.w32(e + 0x5C, head);
        m.w32(slot, e);
        let d = m.r8(e + 0x14);
        m.w8(e + 0x79, d);
        if m.r8(e + 0x7A) != 2 {
            m.w8(e + 0x7A, 2);
            let next = m.r32(e + 0x5C);
            if next != 0 && m.r8(next + 0x7A) <= 2 {
                propagate_queue_rank(m, next, 3);
            }
        }
    }
    let d = m.r8(e + 0x14);
    m.w8(e + 0x3C, d);
    // Reglas de movimiento `0x0055A290(ecx = +0x20, 0, 1, 0, 0)`.
    m.w8(e + 0x27, 0);
    m.w8(e + 0x24, 1);
    m.w8(e + 0x25, 0);
    m.w8(e + 0x26, 0);
    links_alloc(m, h, e, a.set);
    walker_insert(m, h, e, a.set, None);
    let r = h.rand(m) as i32;
    m.w16(e + 0x82, (r % 7 - 3) as u16);
    let r = h.rand(m) as i32;
    m.w16(e + 0x84, (r % 5 - 2) as u16);
    let r = h.rand(m) as i32;
    m.w32(e + 0x64, 0);
    m.w8(e + 0x86, ((r % 0x19) as u8).wrapping_add(0x74));
    m.w8(e + 0x60, a.tag);
}

/// `entity_cell_links_alloc_for_sprite_set` (`0x00559CF0(ecx = entidad, conjunto)`): tabla de enlaces por celda.
pub fn links_alloc(m: &mut dyn Mem, h: &mut dyn FigureHost, e: u32, set: u32) {
    let size = m.r32(set + 0x3C).wrapping_mul(4);
    let table = h.alloc(m, size);
    for i in 0..size {
        m.w8(table.wrapping_add(i), 0);
    }
    m.w32(e + 4, table);
}

fn common(x: u32, y: u32, heading: u32, set: u32, link: u32, v: u32, mark: u8, queued: u8, side: u8, tag: u8) -> WalkerInit {
    WalkerInit { x, y, heading: heading as u8, set, link: link as u16, city: (v & 0x7F) as u8, mark, queued, side, tag }
}

/// `+0x40` de arqueros, ballesteros y arcabuceros (`0x005673E0`, `0x005623B0`, `0x00572A90`): conjunto de la clase
/// `variante >> 16` en `+0x88`, cola según el bit 7.
pub fn shooter_init(m: &mut dyn Mem, h: &mut dyn FigureHost, t: SetTable, e: u32, a: [u32; 5]) {
    let [x, y, heading, link, v] = a;
    m.w8(e + 0x88, 0);
    let want = (v >> 16) as u16;
    sample(m, h, t, e + 0x88, &|m, b| u16::from(kind_at(m, t, b)) == want);
    let i = u32::from(m.r8(e + 0x88));
    let set = set_at(m, t, i);
    let kind = kind_at(m, t, i);
    walker_init(m, h, e, &common(x, y, heading, set, link, v, 1, ((v >> 7) & 1) as u8, kind, 0));
}

/// `+0x40` de la infantería y los jinetes (`0x0056A700`, `0x0056F0B0`): cualquier conjunto de la tabla.
pub fn troop_init(m: &mut dyn Mem, h: &mut dyn FigureHost, t: SetTable, e: u32, a: [u32; 5]) {
    let [x, y, heading, link, v] = a;
    m.w8(e + 0x88, 0);
    sample(m, h, t, e + 0x88, &|_, _| true);
    let i = u32::from(m.r8(e + 0x88));
    let set = set_at(m, t, i);
    walker_init(m, h, e, &common(x, y, heading, set, link, v, 1, ((v >> 7) & 1) as u8, 0, 0));
}

/// `siege_fighter_init` (`0x00570F70`): lancero con conjunto al azar (`+0x91`), bando del bit 7 (`+0x90`), sin
/// desvíos; con el bit 8 ya está en su puesto: se orienta a `escena+0x5D2`, espera (`0x18`) y busca pareja.
pub fn pikeman_init(m: &mut dyn Mem, h: &mut dyn FigureHost, e: u32, a: [u32; 5]) {
    let [x, y, heading, link, v] = a;
    let n = m.r32(PIKEMEN.count);
    let r = h.rand(m);
    let pick = r.checked_rem(n).unwrap_or(0) as u8;
    m.w32(e + 0x88, 0);
    m.w8(e + 0x90, ((v >> 7) & 1) as u8);
    m.w32(e + 0x8C, 0);
    m.w8(e + 0x91, pick);
    let set = set_at(m, PIKEMEN, u32::from(pick));
    walker_init(m, h, e, &common(x, y, heading, set, link, v, 4, 0, 0, 1));
    m.w16(e + 0x82, 0);
    m.w16(e + 0x84, 0);
    if v & 0x100 != 0 {
        let rect = walker_remove(m, h, e, set, true);
        let city = m.r8(e + 0x6A);
        let rec = siege_of(m, city);
        let s = m.r32(rec + 8);
        let d = m.r8(s + 0x5D2);
        m.w8(e + 0x3C, d);
        m.w8(e + 0x14, d);
        let f = anim_select(m, e + 0x2C, set, 0x18);
        m.w8(e + 0x15, f);
        pikeman_acquire(m, e);
        walker_insert(m, h, e, set, rect);
    }
}

/// `siege_fighter_walker_init` (`0x005777B0`): luchador de la cola; con clase `> 2` es de la formación (`+0x88`,
/// puesto `+0x89 = clase - 3`, rumbo del puesto `escena+0x74`, dado la vuelta si hay salida `+0x3A`) y usa los
/// conjuntos de clase 0. Sin conjunto de su clase vuelve a buscar entre los de clase 0.
pub fn fighter_init(m: &mut dyn Mem, h: &mut dyn FigureHost, e: u32, a: [u32; 5]) {
    let [x, y, heading, link, v] = a;
    let t = FIGHTERS;
    let mut kind = (v >> 16) as u8;
    m.w8(e + 0x90, 0xFF);
    if kind > 2 {
        m.w8(e + 0x88, 1);
        m.w8(e + 0x89, kind - 3);
        kind = 0;
    } else {
        m.w8(e + 0x88, 0);
    }
    for _ in 0..2 {
        let want = kind;
        sample(m, h, t, e + 0x90, &|m, b| kind_at(m, t, b) == want);
        if m.r8(e + 0x90) != 0xFF {
            break;
        }
        kind = 0;
    }
    let i = u32::from(m.r8(e + 0x90));
    let set = set_at(m, t, i);
    let side = kind_at(m, t, i);
    walker_init(m, h, e, &common(x, y, heading, set, link, v, 2, ((v >> 7) & 1) as u8, side, 0));
    if m.r8(e + 0x88) != 0 {
        let city = m.r8(e + 0x6A);
        let rec = siege_of(m, city);
        let s = m.r32(rec + 8);
        let post = u32::from(m.r8(e + 0x89)) >> 1;
        let d = m.r8(s + 0x74 + post);
        m.w8(e + 0x3C, d);
        m.w32(e + 0x8C, 0);
        if m.r16(s + 0x3A) != 0 {
            m.w8(e + 0x3C, d.wrapping_add(0x80));
        }
    } else {
        m.w32(e + 0x8C, 0);
    }
}
