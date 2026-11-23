//! Métodos comunes de las figuras de la batalla en la ciudad: visibles y con casillas libres (`+0x60`,
//! `0x00561140`, con `0x00561280` para las figuras anchas), paso posible a una casilla (`0x00561330`), búsqueda de
//! objetivo (`+0x58`, `0x00561520`) y reserva del objetivo (`0x00561F60`).
//!
//! La búsqueda mide con `dx² + (dy·34/22)²` (el eje vertical de la vista isométrica pesa más) y, para defensores
//! (`+0x7B & 1 == 0`) y atacantes, recorre en orden: la puerta del edificio asaltado (`escena+0x6F`), las tres listas
//! de figuras del bando contrario, la lista de vigías (`+0x78`), el edificio de reunión y, por último, la casilla
//! objetivo (defensores) o el cabecilla de la cola (atacantes). Devuelve 0 (nada), 1 (objetivo lejano), 3 (muy
//! lejos con asalto en marcha) o 4 (cerca, sin pausa); deja el rumbo hacia el objetivo en `+0x3C`, el objetivo en
//! `+0x74`, su clase en `+0x78` (más `0x80` en los defensores) y su píxel en `+0x7C/+0x7E`.
use super::walker::distance_to_target;
use super::{ld16, ld32, s16, siege_of, view, FigureHost, DIRS, SIEGES, TOWNS};
use crate::economy::siege_battle::building_record;
use crate::flatmem::Mem;

/// Tipo de la pieza de la capa `layer` (`+0xC308` suelo, `+0xC30C` alta) en la casilla `idx`, o `None` si está vacía
/// (solo la capa alta).
fn layer_kind(m: &mut dyn Mem, h: &mut dyn FigureHost, layer: u32, idx: u32) -> Option<u32> {
    let v = view(m);
    let id = ld16(m, v + layer, idx.wrapping_mul(4));
    if layer == 0xC30C && id == 0 {
        return None;
    }
    let obj = ld32(m, v + 0x98, u32::from(id) * 4);
    Some(h.tile_kind(m, obj))
}

/// Índice de la casilla `(col, fila)` de la rejilla o `None` fuera de ella.
fn cell_index(m: &mut dyn Mem, col: u32, row: u32) -> Option<u32> {
    let v = view(m);
    let w = m.r32(v + 0xC314);
    if col >= w || row >= m.r32(v + 0xC318) {
        return None;
    }
    Some(w.wrapping_mul(row).wrapping_add(col))
}

fn ground_gate(g: u32) -> bool {
    matches!(g, 0x10 | 0x17 | 0x18)
}

/// `0x00561280(columna, fila)` (`ecx` sin usar): casilla pisable por una figura ancha. Fuera de la rejilla o con
/// una pieza alta `0x29`, no; con una `0x2F` (puerta), solo si el suelo no es `0x10/0x17/0x18`.
pub fn wide_tile_free(m: &mut dyn Mem, h: &mut dyn FigureHost, col: u32, row: u32) -> bool {
    let Some(idx) = cell_index(m, col, row) else { return false };
    let g = layer_kind(m, h, 0xC308, idx).unwrap_or(0);
    match layer_kind(m, h, 0xC30C, idx) {
        None => true,
        Some(0x29) => false,
        Some(0x2F) => !ground_gate(g),
        Some(_) => true,
    }
}

/// `0x00561140(ecx = caminante, comprobar ventana)` (método `+0x60`): con `ventana`, la figura debe estar cerca de la
/// pantalla (`50` px de margen en x, `100` en y); las anchas (`+0x59`) además necesitan libres las dos casillas
/// bajo ellas según su orientación (`+0x79`).
pub fn visible_and_cells_free(m: &mut dyn Mem, h: &mut dyn FigureHost, w: u32, window: u8) -> bool {
    let v = view(m);
    let px = s16(m, w + 0xE);
    let py = s16(m, w + 0x12);
    if window != 0 {
        let sx = m.r32(v + 0xC650);
        if (px.wrapping_add(0x32) as u32) < sx {
            return false;
        }
        let sy = m.r32(v + 0xC654);
        if (py.wrapping_add(0x64) as u32) < sy {
            return false;
        }
        if px as u32 > sx.wrapping_add(m.r32(v + 0x2C)).wrapping_add(0x32) {
            return false;
        }
        if py as u32 > m.r32(v + 0x30).wrapping_add(sy).wrapping_add(0x64) {
            return false;
        }
    }
    if m.r8(w + 0x59) == 0 {
        return true;
    }
    let y = py.wrapping_sub(7);
    let d = m.r8(w + 0x79);
    let (x, dir) = if d == DIRS[1] || d == DIRS[5] { (px.wrapping_add(9), DIRS[5]) } else { (px.wrapping_sub(9), DIRS[3]) };
    let (mut c, mut r) = h.pixel_to_cell(m, x, y);
    if !wide_tile_free(m, h, c, r) {
        return false;
    }
    h.step(m, &mut c, &mut r, dir);
    wide_tile_free(m, h, c, r)
}

/// `0x00561330(ecx = caminante, columna, fila, rumbo)`: ¿puede dar el paso? La casilla siguiente debe estar en la
/// rejilla, sin pieza alta salvo puertas (`0x2F` sobre suelo de puerta, `0x30`, `0x31`) y con suelo pisable
/// (`0x10`, `0x17`, `0x28`); el suelo de puerta (`0x18`) solo si la figura está dentro del alcance `+0x80` de la
/// casilla objetivo (con salida del asalto `escena+0x78` y atacante, o acercándose desde más lejos).
pub fn can_step_to_tile(m: &mut dyn Mem, h: &mut dyn FigureHost, w: u32, col: u32, row: u32, dir: u8) -> bool {
    let city = m.r8(w + 0x6A);
    let rec = siege_of(m, city);
    let s = m.r32(rec + 8);
    let (mut c, mut r) = (col, row);
    h.step(m, &mut c, &mut r, dir);
    let Some(idx) = cell_index(m, c, r) else { return false };
    let g = layer_kind(m, h, 0xC308, idx).unwrap_or(0);
    if let Some(k) = layer_kind(m, h, 0xC30C, idx) {
        match k {
            0x2F => return ground_gate(g),
            0x30 | 0x31 => {}
            _ => return false,
        }
    }
    match g {
        0x10 | 0x17 | 0x28 => true,
        0x18 => {
            let reach = m.r16(w + 0x80);
            let attacker = m.r8(w + 0x7B) & 1 != 0;
            let exit = m.r8(s + 0x78) != 0;
            let within = distance_to_target(m, s, c, r) as u16 <= reach;
            let mut leaving = false;
            if within {
                if exit {
                    leaving = true;
                } else if m.r16(s + 0x3A) == 0 && !attacker {
                    return false;
                }
            }
            if !leaving && !exit {
                return true;
            }
            // `0x0056146A`.
            if attacker {
                return true;
            }
            if (distance_to_target(m, s, col, row) & 0xFFFF) as i32 >= i32::from(reach) + 10 {
                return true;
            }
            distance_to_target(m, s, col, row) as u16 <= reach
        }
        _ => false,
    }
}

/// `dx² + (dy·34/22)²` con la aritmética de 32 bits del original y el término vertical escalado.
fn weighted(dx: i32, dy: i32) -> (i32, i32) {
    let q = dy.wrapping_mul(34) / 22;
    (q.wrapping_mul(q).wrapping_add(dx.wrapping_mul(dx)), q)
}

/// Vector hasta un píxel fijo (edificio, casilla o cabecilla) acortado para quedarse delante: 17 px en diagonal si
/// domina el eje x; 22 px por arriba si domina el y y está más de 11 px por debajo.
fn approach(dx: i32, dy: i32) -> (i32, i32) {
    let (mut dx, mut dy) = (dx, dy);
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
    (dx, dy)
}

struct Best {
    d: i32,
    who: u32,
    kind: u8,
    dx: i32,
    q: i32,
}

/// Figuras de una lista: la más cercana sin pausa (`+0x58 == 0`).
fn scan(m: &mut dyn Mem, w: u32, best: &mut Best, list: u32, count: u16, kind: u8) {
    if count == 0 {
        return;
    }
    let mut i: u8 = 0;
    loop {
        let e = m.r32(list + 4 * u32::from(i));
        if m.r8(e + 0x58) == 0 {
            let ex = m.r16(e + 0xE);
            let (dx, dy) = (i32::from(ex as i16).wrapping_sub(s16(m, w + 0xE)), s16(m, e + 0x12).wrapping_sub(s16(m, w + 0x12)));
            let (d, _) = weighted(dx, dy);
            if d < best.d {
                best.d = d;
                m.w16(w + 0x7C, ex);
                best.who = e;
                best.kind = kind;
                let ey = m.r16(e + 0x12);
                m.w16(w + 0x7E, ey);
            }
        }
        i = i.wrapping_add(1);
        if u16::from(i) >= count {
            break;
        }
    }
}

/// Objetivo en un píxel fijo acortado ([`approach`]): `None` si la distancia desborda.
fn fixed(m: &mut dyn Mem, w: u32, best: &mut Best, tx: i32, ty: i32, who: u32) -> bool {
    let (dx, dy) = approach(tx.wrapping_sub(s16(m, w + 0xE)), ty.wrapping_sub(s16(m, w + 0x12)));
    let (d, q) = weighted(dx, dy);
    if d == i32::MAX {
        return false;
    }
    best.d = d;
    best.who = who;
    best.dx = dx;
    best.q = q;
    let x = m.r16(w + 0xE).wrapping_add(dx as u16);
    m.w16(w + 0x7C, x);
    let y = ((q.wrapping_mul(22)) / 34) as u16;
    let y = y.wrapping_add(m.r16(w + 0x12));
    m.w16(w + 0x7E, y);
    best.kind = 4;
    true
}

/// `0x00561520(ecx = caminante, cualquiera)` (método `+0x58`): busca objetivo (ver el módulo). `cualquiera` acepta
/// objetivos a más de `0x12D9E` y usa la puerta del edificio asaltado.
pub fn find_target(m: &mut dyn Mem, h: &mut dyn FigureHost, w: u32, any: u8) -> u8 {
    let city = m.r8(w + 0x6A);
    let rec = siege_of(m, city);
    let s = m.r32(rec + 8);
    if s == 0 || m.r16(s + 0x3A) != 0 {
        return 0;
    }
    let attacker = m.r8(w + 0x7B) & 1 != 0;
    if m.r8(s + 0x79) != 0 && attacker {
        return 0;
    }
    let town = m.r32(TOWNS).wrapping_add(u32::from(city) * 0x9F8);
    let mut best = Best { d: i32::MAX, who: 0, kind: 0, dx: 0, q: 0 };
    let found = 'search: {
        if attacker {
            for l in 1..=3u32 {
                let count = m.r16(s + 0x1A + 2 * (l - 1));
                let list = s + [0x11C, 0x174, 0x1BC][l as usize - 1];
                scan(m, w, &mut best, list, count, l as u8);
            }
            if best.d != i32::MAX {
                break 'search true;
            }
            let count = m.r16(s + 0x18);
            scan(m, w, &mut best, s + 0x7C, count, 0);
            if best.d != i32::MAX {
                break 'search true;
            }
            let i = m.r8(s + 0x578);
            if u16::from(i) >= m.r16(s + 0x20) {
                return 0;
            }
            let e = m.r32(s + 0x1F8 + 4 * u32::from(i));
            if m.r8(e + 0x2C) != 0 {
                return 0;
            }
            let (ex, ey) = (s16(m, e + 0xE), s16(m, e + 0x12));
            if !fixed(m, w, &mut best, ex, ey, e) {
                return 0;
            }
            break 'search true;
        }
        if m.r16(s + 0x3C) != 0 {
            return 0;
        }
        let gate = m.r8(s + 0x6F);
        if gate != 0xFF && any != 0 {
            let id = m.r32(s + u32::from(gate) * 8 + 0x258);
            let r = building_record(m, town, id);
            let (col, row) = (u32::from(m.r8(r)), u32::from(m.r8(r + 1)));
            let (tx, ty) = h.cell_to_pixel(m, col, row);
            let ty = ty.wrapping_sub(0x16);
            let dx = tx.wrapping_sub(s16(m, w + 0xE));
            let (d, q) = weighted(dx, ty.wrapping_sub(s16(m, w + 0x12)));
            if d == i32::MAX {
                return 0;
            }
            best = Best { d, who: w, kind: 4, dx, q };
            m.w16(w + 0x7C, tx as u16);
            m.w16(w + 0x7E, ty as u16);
            break 'search true;
        }
        let base = m.r32(SIEGES).wrapping_add(u32::from(city) * 0x1024);
        for l in 1..=3u32 {
            let count = m.r16(base + 0x1006 + 2 * (l - 1));
            let list = base + [0x410, 0x80C, 0xC08][l as usize - 1];
            scan(m, w, &mut best, list, count, l as u8);
        }
        if best.d != i32::MAX {
            break 'search true;
        }
        if m.r8(s + 0x78) != 0 {
            let count = m.r16(base + 0x1004);
            scan(m, w, &mut best, base + 0x14, count, 0);
            if best.d != i32::MAX {
                break 'search true;
            }
        }
        if m.r16(s + 0x242) != 0 {
            let idx = u32::from(m.r8(s + 0x6C));
            let id = ld32(m, s + 0x250, idx * 12 + 8);
            let r = building_record(m, town, id);
            let (col, row) = (u32::from(m.r8(r)), u32::from(m.r8(r + 1)));
            let (tx, ty) = h.cell_to_pixel(m, col, row);
            if fixed(m, w, &mut best, tx, ty, w) {
                break 'search true;
            }
        }
        if m.r8(s + 0x78) != 0 {
            return 0;
        }
        let (col, row) = (u32::from(m.r16(s + 8)), u32::from(m.r16(s + 0xA)));
        let (tx, ty) = h.cell_to_pixel(m, col, row);
        if !fixed(m, w, &mut best, tx, ty, w) {
            return 0;
        }
        true
    };
    debug_assert!(found);
    if best.d > 0x12D9E && any == 0 {
        return 0;
    }
    let (dx, q) = if best.kind == 4 && !attacker {
        (best.dx, best.q)
    } else {
        let e = best.who;
        let dx = s16(m, e + 0xE).wrapping_sub(s16(m, w + 0xE));
        let (_, q) = weighted(0, s16(m, e + 0x12).wrapping_sub(s16(m, w + 0x12)));
        (dx, q)
    };
    let dir = heading_of(dx, q);
    if best.d > 0x1DE84 && m.r16(s + 0x42) != 0 {
        m.w8(w + 0x3C, dir);
        return 3;
    }
    m.w8(w + 0x3C, dir);
    m.w32(w + 0x74, best.who);
    let kind = if attacker { best.kind } else { best.kind.wrapping_add(0x80) };
    m.w8(w + 0x78, kind);
    if best.d < 0x16412 && m.r8(w + 0x59) == 0 {
        4
    } else {
        1
    }
}

/// Rumbo de ocho direcciones hacia `(dx, q)` (q ya escalado): eje dominante si supera al otro en `1000/414`.
pub(crate) fn heading_of(dx: i32, q: i32) -> u8 {
    let (ax, aq) = (dx.wrapping_abs(), q.wrapping_abs());
    if ax.wrapping_mul(414) > aq.wrapping_mul(1000) {
        return if dx > 0 { DIRS[2] } else { DIRS[6] };
    }
    if aq.wrapping_mul(414) > ax.wrapping_mul(1000) {
        return if q > 0 { DIRS[4] } else { DIRS[0] };
    }
    match (q > 0, dx > 0) {
        (true, true) => DIRS[3],
        (true, false) => DIRS[5],
        (false, true) => DIRS[1],
        (false, false) => DIRS[7],
    }
}

/// `0x00561F60(ecx = caminante)`: reserva el objetivo `+0x74` (bandera `+0x58`) si aún caben combates en la
/// escena (`+0x21E + +0x22 < +0x18`).
pub fn mark_target_engaged(m: &mut dyn Mem, w: u32) {
    let city = m.r8(w + 0x6A);
    let rec = siege_of(m, city);
    let s = m.r32(rec + 8);
    if s == 0 {
        return;
    }
    let e = m.r32(w + 0x74);
    if e == 0 || m.r32(e + 0x8C) == 0 || m.r8(e + 0x58) != 0 {
        return;
    }
    if i32::from(m.r16(s + 0x21E)) + i32::from(m.r16(s + 0x22)) < i32::from(m.r16(s + 0x18)) {
        m.w8(e + 0x58, 1);
        let n = m.r16(s + 0x22).wrapping_add(1);
        m.w16(s + 0x22, n);
    }
}
