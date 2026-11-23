//! Luchador de la cola de la puerta (clase 0, vtabla `0x006762C0`): bucle `+0x14` (`0x00577950`), visibilidad `+0x60`
//! (`0x00579350`), formación de la escalera/ariete (`0x005790B0`), busca y paso hacia el objetivo `+0x58`
//! (`0x00577FB0`), acercamiento a la pareja (`0x00579380`) y nivel de muralla de la ciudad (`0x005252F0`).
//!
//! La tabla de bytes `[0x006E2E60]` por bando (`+0x90`) marca con el bit 0 a los atacantes. La búsqueda recorre las
//! mismas listas que [`super::common::find_target`] y además la huella del edificio asaltado (tablas de tamaño
//! `0x00672F64/74/84` por tipo); cerca (`<= 0x484`) se traba con el objetivo (sonido `0xEE8E` y, contra la puerta, un
//! golpe `0x80020` a 17-34 px en el rumbo), y si no da un paso por las casillas que no son calle (los atacantes en
//! la salida, por las que lo son).
use super::classes::{heading_to, pikeman_acquire, set_of};
use super::common::{can_step_to_tile, heading_of, visible_and_cells_free};
use super::sprite::{anim_select, walker_insert, walker_remove};
use super::step::turn_toward;
use super::update::{seed_cell, walker_update};
use super::walker::{begin_step, clear_mark, mark_at_least, set_mark};
use super::{s16, siege_active, siege_of, vcall, view, FigureHost, DIRS, SIEGES, TOWNS};
use crate::economy::siege_battle::{building_record, flow_distance};
use crate::economy::siege_view::{reset_flow_field_to_cell, update_entry_point_markers};
use crate::flatmem::Mem;

/// Datos del ejecutable desde `0x00672F64`: filas (`+0`) y columnas (`+0x10`) de la huella por clase y clase de
/// huella por tipo de edificio (`+0x20`), con lo que sigue en memoria para clases y tipos fuera de esas tablas
/// (como el original, que no comprueba límites).
const FOOTPRINT: [u8; 0x120] = [
    0x02, 0x03, 0x03, 0x04, 0x04, 0x01, 0x02, 0x04, 0x02, 0x06, 0x06, 0x05, 0x03, 0x01, 0x01, 0x01, 0x03, 0x02, 0x04, 0x03, 0x05, 0x01, 0x02, 0x06,
    0x06, 0x02, 0x04, 0x05, 0x03, 0x01, 0x01, 0x01, 0x03, 0x07, 0x00, 0x00, 0x04, 0x00, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04,
    0x04, 0x04, 0x04, 0x04, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0A, 0x0A, 0x03, 0x02, 0x03, 0x00, 0x00, 0x03, 0x00,
    0x05, 0x03, 0x02, 0x02, 0x03, 0x06, 0x07, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x08, 0x09, 0x08, 0x09, 0x00, 0x00, 0x00, 0x2C, 0x1B, 0x0B, 0x08,
    0x5D, 0x57, 0x58, 0x5B, 0x59, 0x53, 0x56, 0x64, 0x5A, 0x5E, 0x60, 0x55, 0x5C, 0x63, 0x62, 0x61, 0x54, 0x51, 0x51, 0x51, 0x50, 0x50, 0x50, 0x2A,
    0x2A, 0x2A, 0x66, 0x04, 0x03, 0x0B, 0x07, 0x00, 0x02, 0x05, 0x09, 0x33, 0x52, 0x65, 0x67, 0x68, 0x69, 0x37, 0x01, 0x5F, 0x29, 0x29, 0x29, 0x29,
    0x2F, 0x2F, 0x2E, 0x2F, 0x2F, 0x00, 0x00, 0x00, 0xE4, 0x00, 0xD4, 0x01, 0x52, 0x02, 0x18, 0x03, 0x38, 0x04, 0x38, 0x04, 0x00, 0xFC, 0xFF, 0x00,
    0xFC, 0x00, 0x00, 0xFC, 0x01, 0x00, 0xFE, 0xFF, 0x00, 0xFE, 0x00, 0x00, 0xFE, 0x01, 0x00, 0x02, 0xFF, 0x00, 0x02, 0x00, 0x00, 0x02, 0x01, 0x00,
    0x04, 0xFF, 0x00, 0x04, 0x00, 0x00, 0x04, 0x01, 0x00, 0xFF, 0xFB, 0x00, 0xFF, 0xFD, 0x00, 0xFF, 0x03, 0x00, 0xFF, 0x05, 0x00, 0x00, 0xFB, 0x00,
    0x00, 0xFD, 0x00, 0x00, 0x03, 0x00, 0x00, 0x05, 0x00, 0x01, 0xFB, 0x00, 0x01, 0xFD, 0x00, 0x01, 0x03, 0x00, 0x01, 0x05, 0x01, 0xFC, 0xFF, 0x01,
    0xFC, 0x00, 0x01, 0xFE, 0xFF, 0x01, 0xFE, 0x00, 0x01, 0x03, 0xFF, 0x01, 0x03, 0x00, 0x01, 0x05, 0xFF, 0x01, 0x05, 0x00, 0x01, 0x00, 0xFB, 0x01,
    0x00, 0xFD, 0x01, 0x00, 0x02, 0x01, 0x00, 0x04, 0x01, 0x01, 0xFB, 0x01, 0x01, 0xFD, 0x01, 0x01, 0x02, 0x01, 0x01, 0x04, 0x03, 0xFC, 0xFF, 0x03,
];

fn footprint(i: i32) -> i32 {
    FOOTPRINT.get(i as usize).map_or(0, |&b| b as i8 as i32)
}

/// Conjunto de sprites del luchador (`+0x90` en `0x006E2E58`).
fn fighter_set(m: &mut dyn Mem, w: u32) -> u32 {
    set_of(m, w + 0x90, 0x006E_2E58, 0x006E_2E5C)
}

/// Bit de atacante del bando del luchador.
fn attacker_side(m: &mut dyn Mem, w: u32) -> bool {
    let t = m.r32(0x006E_2E60);
    let side = u32::from(m.r8(w + 0x90));
    m.r8(t.wrapping_add(side)) & 1 != 0
}

fn r16(m: &mut dyn Mem, a: u32) -> u32 {
    u32::from(m.r16(a))
}

fn flow(m: &mut dyn Mem, h: &mut dyn FigureHost, s: u32, x: u32, y: u32) -> u32 {
    u32::from(flow_distance(m, h, s, x, y))
}

fn udiv(a: u32, b: u32) -> u32 {
    a.checked_div(b).unwrap_or(0)
}

fn town_of_walker(m: &mut dyn Mem, w: u32) -> u32 {
    let city = u32::from(m.r8(w + 0x6A));
    m.r32(TOWNS).wrapping_add(city * 0x9F8)
}

/// Bit de calle del plano de la ciudad del luchador.
fn street(m: &mut dyn Mem, w: u32, col: u32, row: u32) -> bool {
    let t = town_of_walker(m, w) + 0x7A4;
    let stride = m.r32(t + 8);
    let base = m.r32(t + 0x68);
    m.r8(stride.wrapping_mul(col).wrapping_add(base).wrapping_add(row)) & 0x80 != 0
}

/// `0x00577950(ecx = luchador)` (`+0x14`).
pub fn fighter_update(m: &mut dyn Mem, h: &mut dyn FigureHost, w: u32) -> u8 {
    let set = fighter_set(m, w);
    let city = m.r8(w + 0x6A);
    let rec = siege_of(m, city);
    let s = m.r32(rec + 8);
    if m.r8(w + 0x88) != 0 {
        return formation(m, h, w, set);
    }
    let p = m.r32(w + 0x8C);
    if p != 0 && m.r32(p + 0x8C) == w {
        return 1;
    }
    let idle = |m: &mut dyn Mem, h: &mut dyn FigureHost| walker_update(m, h, w, set, 2, 0, 1, 0, 0);
    if !siege_active(m, rec) {
        return idle(m, h);
    }
    let busy = m.r16(s + 0x3A);
    if busy != 0 && m.r16(s + 0x10) == m.r16(s + 0x3E) && m.r16(s + 0x12) == m.r16(s + 0x40) {
        return idle(m, h);
    }
    if attacker_side(m, w) {
        let a5 = u32::from(m.r8(s + 0x79) != 0);
        return walker_update(m, h, w, set, 2, 0, 1, 1, a5);
    }
    let exit = |m: &mut dyn Mem, h: &mut dyn FigureHost| {
        let (x, y) = (r16(m, s + 0x36), r16(m, s + 0x38));
        flow(m, h, s, x, y)
    };
    if m.r16(s + 0x10) == m.r16(s + 0x14) && m.r16(s + 0x12) == m.r16(s + 0x16) {
        let d = exit(m, h);
        let a = r16(m, s + 0x44);
        let t = udiv(r16(m, s + 0x42).wrapping_sub(a).wrapping_mul(d), r16(m, s + 0x66).wrapping_sub(a));
        return walker_update(m, h, w, set, 2, t, 1, 0, 0);
    }
    let advance = m.r16(s + 0x3C);
    let half = r16(m, s + 0x68) >> 1;
    if u32::from(advance) as i32 > (half + 0x100) as i32 {
        let d = if m.r16(s + 8) == 0xFFFF {
            exit(m, h)
        } else {
            let (x, y) = (r16(m, s + 8), r16(m, s + 0xA));
            flow(m, h, s, x, y)
        };
        let t = udiv(u32::from(advance).wrapping_sub(half).wrapping_sub(0x100).wrapping_mul(d), half);
        return walker_update(m, h, w, set, 2, t, 1, 0, 0);
    }
    if busy != 0 || advance != 0 {
        return idle(m, h);
    }
    let g = m.r8(s + 0x6F);
    if g != 0xFF {
        let d = exit(m, h);
        let gd = r16(m, s + u32::from(g) * 8 + 0x254);
        let mut c = (r16(m, s + 0x44).wrapping_sub(gd) as i32 >> 1) as u32;
        if c > 0x100 {
            c = 0x100;
        }
        let a = r16(m, s + 0x42);
        if a <= gd.wrapping_add(c) {
            return walker_update(m, h, w, set, 2, 0, 1, 1, 0);
        }
        let t = udiv(a.wrapping_sub(gd).wrapping_sub(c).wrapping_mul(d), r16(m, s + 0x66).wrapping_sub(gd).wrapping_sub(c));
        return walker_update(m, h, w, set, 2, t, 1, 1, 0);
    }
    if m.r8(s + 0x78) != 0 {
        return walker_update(m, h, w, set, 2, 0, 1, 1, 0);
    }
    if m.r32(w + 0x8C) != 0 {
        let d = exit(m, h);
        let t = udiv(r16(m, s + 0x42).wrapping_mul(d), r16(m, s + 0x66));
        return walker_update(m, h, w, set, 2, t, 1, 0, 0);
    }
    let d = exit(m, h);
    let e = d - d.min(12);
    let t = udiv(e.wrapping_mul(r16(m, s + 0x42)), r16(m, s + 0x66));
    walker_update(m, h, w, set, 2, t.wrapping_add(12), 12, 0, 0)
}

/// `0x00579350(ecx = luchador, ventana)` (`+0x60`): emparejado, no; si no, [`visible_and_cells_free`].
pub fn fighter_visible(m: &mut dyn Mem, h: &mut dyn FigureHost, w: u32, window: u8) -> bool {
    let p = m.r32(w + 0x8C);
    if p != 0 && m.r32(p + 0x8C) == w {
        return false;
    }
    visible_and_cells_free(m, h, w, window)
}

/// `0x005790B0(ecx = luchador, conjunto)`: marcha en formación (`+0x88`) por la rejilla, con la animación de andar
/// (estado 2), giros de un paso y el paso entre casillas por el vector `+0x40/+0x44`; en el asalto se da la vuelta
/// al llegar al rumbo de su puesto (`escena+0x74`) y se detiene en él (`+0x56/+0x5E`) o en la entrada
/// (`+0x46/+0x4E`, desmarcando la casilla).
pub fn formation(m: &mut dyn Mem, h: &mut dyn FigureHost, w: u32, set: u32) -> u8 {
    let city = m.r8(w + 0x6A);
    let rec = siege_of(m, city);
    let s = m.r32(rec + 8);
    if !siege_active(m, rec) {
        return 0;
    }
    let mut frame = m.r8(w + 0x15);
    let (mut x, mut y) = (m.r32(w + 0xC), m.r32(w + 0x10));
    let slot = u32::from(m.r8(w + 0x89));
    'moved: {
        if m.r16(s + 0x3A) != 0 && m.r8(w + 0x3C) == m.r8(s + 0x74 + (slot >> 1)) {
            let d = m.r8(w + 0x3C).wrapping_add(0x80);
            m.w8(w + 0x3C, d);
            frame = anim_select(m, w + 0x2C, set, 2);
            m.w16(w + 0x38, 0);
            m.w16(w + 0x3A, 0);
            break 'moved;
        }
        if m.r8(w + 0x34) != 2 {
            break 'moved;
        }
        frame = vcall(m, h, w + 0x2C, 4, &[set, w + 8, 0]) as u8;
        if m.r8(w + 0x14) != m.r8(w + 0x3C) {
            if frame == m.r8(w + 0x15) {
                break 'moved;
            }
            let rect = walker_remove(m, h, w, set, true);
            let target = m.r8(w + 0x3C);
            let n = m.r8(set + 0x14);
            turn_toward(m, w + 0x14, target, n);
            walker_insert(m, h, w, set, rect);
            return 1;
        }
        let v = view(m);
        let t = m.r32(v + 0x414).min(0x1F4);
        let step = r16(m, w + 0x38);
        let steps = r16(m, w + 0x3A);
        let k = if step.wrapping_add(t) < steps {
            m.w16(w + 0x38, step.wrapping_add(t) as u16);
            t
        } else {
            let k = steps.wrapping_sub(step);
            let (px, py) = (s16(m, w + 0xE), s16(m, w + 0x12));
            let (c, r) = h.pixel_to_cell(m, px, py);
            let i = 2 * slot;
            if c == r16(m, s + 0x56 + i) && r == r16(m, s + 0x5E + i) && m.r16(s + 0x3A) == 0 {
                frame = anim_select(m, w + 0x2C, set, 5);
            } else if c == r16(m, s + 0x46 + i) && r == r16(m, s + 0x4E + i) && m.r16(s + 0x3A) != 0 {
                clear_mark(m, s, c, r, 2);
                return 0;
            } else {
                let (mut c2, mut r2) = (c, r);
                let d = m.r8(w + 0x3C);
                h.step(m, &mut c2, &mut r2, d);
                begin_step(m, h, w, c2, r2, set);
            }
            k
        };
        x = x.wrapping_add(m.r32(w + 0x40).wrapping_mul(k));
        y = y.wrapping_add(m.r32(w + 0x44).wrapping_mul(k));
    }
    if x >> 16 == s16(m, w + 0xE) as u32 && y >> 16 == s16(m, w + 0x12) as u32 && frame == m.r8(w + 0x15) {
        m.w32(w + 0xC, x);
        m.w32(w + 0x10, y);
        return 1;
    }
    let rect = walker_remove(m, h, w, set, true);
    m.w32(w + 0x10, y);
    m.w8(w + 0x15, frame);
    m.w32(w + 0xC, x);
    walker_insert(m, h, w, set, rect);
    1
}

/// `dx² + (dy·34/22)²` y el término vertical escalado.
fn weighted(dx: i32, dy: i32) -> (i32, i32) {
    let q = dy.wrapping_mul(34) / 22;
    (q.wrapping_mul(q).wrapping_add(dx.wrapping_mul(dx)), q)
}

struct Best {
    d: i32,
    who: u32,
    kind: u8,
    dx: i32,
    q: i32,
}

/// Lista de figuras: la más cercana sin pausa.
fn scan(m: &mut dyn Mem, w: u32, best: &mut Best, list: u32, count: u16, kind: u8) {
    if count == 0 {
        return;
    }
    let mut i: u8 = 0;
    loop {
        let e = m.r32(list + 4 * u32::from(i));
        if m.r8(e + 0x58) == 0 {
            let dx = s16(m, e + 0xE).wrapping_sub(s16(m, w + 0xE));
            let (d, _) = weighted(dx, s16(m, e + 0x12).wrapping_sub(s16(m, w + 0x12)));
            if d < best.d {
                best.d = d;
                best.who = e;
                best.kind = kind;
            }
        }
        i = i.wrapping_add(1);
        if u16::from(i) >= count {
            break;
        }
    }
}

/// Píxel de una figura de cabeza de la cola, acortado 17 px en x o 11 px en y hacia el luchador.
fn leader(m: &mut dyn Mem, w: u32, e: u32) -> (i32, i32, i32, i32) {
    let mut dy = s16(m, e + 0x12).wrapping_sub(s16(m, w + 0x12)).wrapping_sub(0xB);
    let mut dx = s16(m, e + 0xE).wrapping_sub(s16(m, w + 0xE));
    if dx.wrapping_abs() > dy.wrapping_abs() {
        if dx < 0 {
            dx += 0x11;
        } else if dx > 0 {
            dx -= 0x11;
        }
    } else if dy > 0 {
        dy -= 0xB;
    } else {
        dy += 0xB;
    }
    let (d, q) = weighted(dx, dy);
    (d, q, dx, dy)
}

/// Paso del luchador: prueba el rumbo y sus dos vecinos con la casilla de destino libre de marcas (`2`), el paso
/// posible y con distancia en el campo; marca el destino, arranca el paso y desmarca la casilla de la posición.
/// `street`: `Some(b)` exige que el destino sea (o no sea) calle; `near` corta tras el primer intento.
fn fighter_step(m: &mut dyn Mem, h: &mut dyn FigureHost, w: u32, s: u32, dir: u8, need_street: bool) -> u8 {
    let (px, py) = (s16(m, w + 0xE), s16(m, w + 0x12));
    let (c, r) = h.pixel_to_cell(m, px, py);
    let mut dir = dir;
    for k in 0..3 {
        if k == 1 {
            dir = dir.wrapping_add(0x20);
        } else if k == 2 {
            dir = dir.wrapping_add(0xC0);
        }
        let (mut c2, mut r2) = (c, r);
        h.step(m, &mut c2, &mut r2, dir);
        if need_street && !street(m, w, c2, r2) {
            continue;
        }
        if mark_at_least(m, s, c2, r2, 2) || !can_step_to_tile(m, h, w, c, r, dir) || flow(m, h, s, c2, r2) as u16 >= 0x7FFF {
            continue;
        }
        m.w8(w + 0x3C, dir);
        set_mark(m, s, c2, r2, 2);
        let set = fighter_set(m, w);
        begin_step(m, h, w, c2, r2, set);
        let (px, py) = (s16(m, w + 0xE), s16(m, w + 0x12));
        let (c3, r3) = h.pixel_to_cell(m, px, py);
        clear_mark(m, s, c3, r3, 2);
        return 2;
    }
    0
}

/// `0x00577FB0(ecx = luchador, sin uso)` (`+0x58`): busca objetivo y se traba (1) o da un paso hacia él (2); 0 si
/// no hay o queda lejos (más de `0x70E40`, o de `0x1210` con puerta asaltada).
pub fn fighter_target(m: &mut dyn Mem, h: &mut dyn FigureHost, w: u32) -> u8 {
    let city = m.r8(w + 0x6A);
    let rec = siege_of(m, city);
    let s = m.r32(rec + 8);
    if s == 0 || (m.r8(s + 0x78) == 0 && m.r8(s + 0x6F) == 0xFF) {
        return 0;
    }
    let busy = m.r16(s + 0x3A);
    let attacker = attacker_side(m, w);
    if busy != 0 && !attacker {
        return 0;
    }
    let town = town_of_walker(m, w);
    let mut b = Best { d: i32::MAX, who: m.r32(0), kind: 0, dx: 0, q: 0 };
    'found: {
        if attacker {
            let count = m.r16(s + 0x18);
            scan(m, w, &mut b, s + 0x7C, count, 0);
            if b.d != i32::MAX {
                break 'found;
            }
            for l in 1..=3u32 {
                let count = m.r16(s + 0x1A + 2 * (l - 1));
                scan(m, w, &mut b, s + [0x11C, 0x174, 0x1BC][l as usize - 1], count, l as u8);
            }
            if b.d != i32::MAX {
                break 'found;
            }
            let i = m.r8(s + 0x578);
            if u16::from(i) < m.r16(s + 0x20) {
                let e = m.r32(s + 0x1F8 + 4 * u32::from(i));
                if m.r8(e + 0x2C) == 0 {
                    let (d, q, dx, _) = leader(m, w, e);
                    if d != i32::MAX {
                        b = Best { d, who: e, kind: 4, dx, q };
                        break 'found;
                    }
                }
            }
            let e = m.r32(s + 0x574);
            if e == 0 || m.r8(e + 0x2C) != 0 {
                return 0;
            }
            let (d, q, dx, _) = leader(m, w, e);
            if d == i32::MAX {
                return 0;
            }
            b = Best { d, who: e, kind: 5, dx, q };
            break 'found;
        }
        if m.r16(s + 0x3C) != 0 {
            return 0;
        }
        let g = m.r8(s + 0x6F);
        if g != 0xFF {
            // Huella del edificio asaltado: filas y columnas de casillas a medio paso isométrico.
            let id = m.r32(s + u32::from(g) * 8 + 0x258);
            let r = building_record(m, town, id);
            let (col, row) = (u32::from(m.r8(r)), u32::from(m.r8(r + 1)));
            let (bx, by) = h.cell_to_pixel(m, col, row);
            let kind = footprint(0x20 + i32::from(m.r8(r + 3)));
            let rows = footprint(kind);
            if rows <= 0 {
                return 0;
            }
            let cols = footprint(0x10 + kind);
            let (px, py) = (s16(m, w + 0xE), s16(m, w + 0x12));
            for i in 0..rows {
                for j in 0..cols {
                    let dx = (-17 * i).wrapping_sub(px).wrapping_add(bx).wrapping_add(17 * j);
                    let dy = (-11 * i).wrapping_sub(py).wrapping_add(by).wrapping_sub(11 * j);
                    let (d, q) = weighted(dx, dy);
                    if d < b.d {
                        b = Best { d, who: w, kind: 4, dx, q };
                    }
                }
            }
            break 'found;
        }
        let base = m.r32(SIEGES).wrapping_add(u32::from(city) * 0x1024);
        let count = m.r16(base + 0x1004);
        scan(m, w, &mut b, base + 0x14, count, 0);
        if b.d != i32::MAX {
            break 'found;
        }
        for l in 1..=3u32 {
            let count = m.r16(base + 0x1006 + 2 * (l - 1));
            scan(m, w, &mut b, base + [0x410, 0x80C, 0xC08][l as usize - 1], count, l as u8);
        }
        if b.d != i32::MAX {
            break 'found;
        }
        if m.r16(s + 0x242) == 0 {
            return 0;
        }
        let idx = u32::from(m.r8(s + 0x6C));
        let t = m.r32(s + 0x250);
        let id = m.r32(t.wrapping_add(idx * 12 + 8));
        let r = building_record(m, town, id);
        let (col, row) = (u32::from(m.r8(r)), u32::from(m.r8(r + 1)));
        let (tx, ty) = h.cell_to_pixel(m, col, row);
        let (mut dx, mut dy) = (tx.wrapping_sub(s16(m, w + 0xE)), ty.wrapping_sub(s16(m, w + 0x12)));
        if dx.wrapping_abs() > dy.wrapping_abs() {
            if dx < 0 {
                dx += 0x11;
            } else if dx > 0 {
                dx -= 0x11;
            }
            dy = dy.wrapping_sub(0x11);
        } else if dy > 0xB {
            dy -= 0x16;
        }
        let (d, q) = weighted(dx, dy);
        if d == i32::MAX {
            return 0;
        }
        b = Best { d, who: w, kind: 4, dx, q };
    }
    if b.d > 0x70E40 || (m.r8(s + 0x6F) != 0xFF && b.d > 0x1210) {
        return 0;
    }
    let (dx, q) = if b.kind >= 4 && m.r8(w + 0x7B) & 1 == 0 {
        (b.dx, b.q)
    } else {
        let e = b.who;
        let dx = s16(m, e + 0xE).wrapping_sub(s16(m, w + 0xE));
        let q = s16(m, e + 0x12).wrapping_sub(s16(m, w + 0x12)).wrapping_mul(17) / 11;
        (dx, q)
    };
    let dir = heading_of(dx, q);
    if m.r8(s + 0x79) != 0 && attacker {
        // Atacantes en la salida: solo por la calle y de espaldas al objetivo.
        let (px, py) = (s16(m, w + 0xE), s16(m, w + 0x12));
        let (c, r) = h.pixel_to_cell(m, px, py);
        if !street(m, w, c, r) {
            return 0;
        }
        return fighter_step(m, h, w, s, dir.wrapping_add(0x80), true);
    }
    if b.d <= 0x484 {
        m.w32(w + 0x74, b.who);
        let kind = if attacker { b.kind } else { b.kind.wrapping_add(0x80) };
        m.w8(w + 0x78, kind);
        m.w8(w + 0x3C, dir);
        if h.sound_list_size(m, 0xEE8E) != 0 {
            let n = h.sound_list_size(m, 0xEE8E) as i32;
            let r = (h.rand(m) as i32).wrapping_rem(n);
            let (x, y) = (s16(m, w + 0xE), s16(m, w + 0x12));
            h.play_sound(m, x, y, r.wrapping_sub(0x1172) as u32);
        }
        if m.r8(s + 0x6F) != 0xFF {
            let (px, py) = (s16(m, w + 0xE), s16(m, w + 0x12));
            let (ox, oy) = [(0, -22), (17, -11), (34, 0), (17, 11), (0, 22), (-17, 11), (-34, 0), (-17, -11)][usize::from(m.r8(w + 0x3C) >> 5)];
            let (x, y) = (px.wrapping_add(ox), py.wrapping_add(oy));
            h.spawn_entity(m, [(x as u32) << 16, (y as u32) << 16, 0x12, 0x8_0020, u32::from(DIRS[1]), 0, 0]);
        }
        return 1;
    }
    let (px, py) = (s16(m, w + 0xE), s16(m, w + 0x12));
    let (c, r) = h.pixel_to_cell(m, px, py);
    if street(m, w, c, r) {
        return 0;
    }
    fighter_step(m, h, w, s, dir, false)
}

/// `0x00579380(ecx = luchador)`: se acerca a su pareja `+0x8C`. Amortigua el desvío `+0x82/+0x84`; sin cabeza de
/// cola libre en el bando de la pareja o con el asalto en marcha deshace la pareja (0). A 10 px la entrega a otro
/// atacante (`0x00571DA0`) y la deja en espera (animación `0x95`, devuelve 2); si no, reorienta el campo hacia su
/// punto de reunión y da un paso (2), espera cerca (3) o nada (0).
pub fn approach_partner(m: &mut dyn Mem, h: &mut dyn FigureHost, w: u32) -> u8 {
    if m.r8(w + 0x58) != 0 {
        return 0;
    }
    for at in [w + 0x82, w + 0x84] {
        let v = m.r16(at) as i16;
        if v > 0 {
            m.w16(at, (v - 1) as u16);
        } else if v < 0 {
            m.w16(at, (v + 1) as u16);
        }
    }
    let city = m.r8(w + 0x6A);
    let rec = siege_of(m, city);
    let s = m.r32(rec + 8);
    if s == 0 {
        return 0;
    }
    let p = m.r32(w + 0x8C);
    let side = u32::from(m.r8(p + 0x90));
    let head = m.r32(rec.wrapping_add(0x10).wrapping_sub(4 * side));
    if head == 0 || m.r16(s + 0x3A) != 0 || m.r16(s + 0x3C) != 0 || m.r8(s + 0x78) != 0 {
        m.w32(p + 0x88, 0);
        m.w32(w + 0x8C, 0);
        return 0;
    }
    let dx = s16(m, p + 0xE).wrapping_sub(s16(m, w + 0xE));
    let dy = s16(m, p + 0x12).wrapping_sub(s16(m, w + 0x12));
    let d2 = dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx));
    if d2 <= 100 {
        m.w32(p + 0x88, 0);
        pikeman_acquire(m, p);
        m.w32(p + 0x8C, w);
        let set = vcall(m, h, p, 0x4C, &[]);
        let rect = walker_remove(m, h, p, set, true);
        let set = vcall(m, h, p, 0x4C, &[]);
        let f = anim_select(m, p + 0x2C, set, 0x95);
        m.w8(p + 0x15, f);
        let set = vcall(m, h, p, 0x4C, &[]);
        walker_insert(m, h, p, set, rect);
        return 2;
    }
    if d2 > 0x70E40 {
        return 0;
    }
    let (rx, ry) = (r16(m, s + 0x5CA + 2 * side), r16(m, s + 0x5CE + 2 * side));
    if !(r16(m, s + 0x10) == rx && r16(m, s + 0x12) == ry) {
        reset_flow_field_to_cell(m, h, s, rx, ry, 0);
        let (a, b) = (r16(m, s + 0xC), r16(m, s + 0xE));
        seed_cell(m, h, s, a, b, 0);
        update_entry_point_markers(m, h, s, true);
    }
    let mut dir = heading_to(dx, dy);
    let (px, py) = (s16(m, w + 0xE), s16(m, w + 0x12));
    let (c, r) = h.pixel_to_cell(m, px, py);
    for k in 0..3 {
        if k == 1 {
            if d2 <= 0x484 {
                return 3;
            }
            dir = dir.wrapping_add(0x20);
        } else if k == 2 {
            dir = dir.wrapping_add(0xC0);
        }
        let (mut c2, mut r2) = (c, r);
        h.step(m, &mut c2, &mut r2, dir);
        if mark_at_least(m, s, c2, r2, 1) || !can_step_to_tile(m, h, w, c, r, dir) || flow(m, h, s, c2, r2) as u16 >= 0x7FFF {
            continue;
        }
        m.w8(w + 0x3C, dir);
        set_mark(m, s, c2, r2, 2);
        clear_mark(m, s, c, r, 2);
        let set = fighter_set(m, w);
        begin_step(m, h, w, c2, r2, set);
        return 2;
    }
    0
}

/// `town_get_wall_expansion_level` (`0x005252F0`, `ecx = ciudad`): con registros de muralla (`+0x766` dentro de la
/// tabla), 0 si el primero está en obras (`>= 0xFE`) y 1 si alguno de la cadena `+0x768` lo está; sin ellos, por los
/// puntos del plano (`0x004FCFE0`): 2, 1 o `0xFF`/0.
pub fn wall_level(m: &mut dyn Mem, town: u32) -> u8 {
    let first = r16(m, town + 0x766);
    let n = r16(m, town + 0x776);
    if first < n {
        let recs = m.r32(town + 0x75C);
        if m.r8(recs.wrapping_add(first * 8 + 2)) >= 0xFE {
            return 0;
        }
        let mut i = r16(m, town + 0x768);
        while i < n {
            if m.r8(recs.wrapping_add(i * 8 + 2)) == 0xFE {
                return 1;
            }
            i = u32::from(m.r16(recs.wrapping_add(i * 8 + 4)));
        }
        return 0;
    }
    let plan = town + 0x7A4;
    let valid = |m: &mut dyn Mem, k: u32| {
        i32::from(m.r8(plan + 2 * k + 0xC)) < m.r32(plan + 4) as i32 && i32::from(m.r8(plan + 2 * k + 0xD)) < m.r32(plan + 8) as i32
    };
    if !valid(m, 2) {
        return 2;
    }
    if !valid(m, 1) {
        return 1;
    }
    if valid(m, 0) {
        0xFF
    } else {
        0
    }
}
