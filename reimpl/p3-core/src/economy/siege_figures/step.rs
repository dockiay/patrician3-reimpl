//! Paso del caminante por la rejilla de la vista: reglas de casilla (`0x0055AD90`), prueba del paso en un rumbo
//! (`0x0055A2C0`), vector de avance hacia una casilla (`0x0055CF60`), ocupante único de una casilla (`0x0055D700`),
//! rumbo hacia el destino (`0x0055D910`) y giro de un paso (`0x0055A1C0`).
//!
//! Las reglas de movimiento (`caminante + 0x20`) son bytes: `+4` respeta puertas, `+5` va fuera de las calles, `+6`
//! cruza figuras de clase `0x1F` y puertas de suelo `0x18`, `+7` radio de la figura (dos pasadas si no es 0) y `+8`
//! (salida) la casilla de destino cumple las reglas. La base de dibujo (`caminante + 8`) tiene la posición 16.16 en
//! `+4/+8` y el enlace en `+0xE`.
use super::{ld16, ld32, s16, siege_active, siege_of, vcall, view, FigureHost, DIRS, TOWNS};
use crate::flatmem::Mem;

/// Tipo de la pieza de suelo (capa 0, `+0xC308`) de la casilla `idx` por su método `+0x50`.
pub(crate) fn ground_kind(m: &mut dyn Mem, h: &mut dyn FigureHost, idx: u32) -> u32 {
    let v = view(m);
    let id = ld16(m, v + 0xC308, idx.wrapping_mul(4));
    let obj = ld32(m, v + 0x98, u32::from(id) * 4);
    h.tile_kind(m, obj)
}

/// Bit de calle (`0x80`) del plano de la ciudad de la vista en `(col, row)`.
fn street(m: &mut dyn Mem, col: u32, row: u32) -> bool {
    let v = view(m);
    let city = m.r32(v + 0xC324);
    let town = m.r32(TOWNS).wrapping_add(city.wrapping_mul(0x9F8));
    let plan = m.r32(town + 0x80C);
    let stride = m.r32(town + 0x7AC);
    m.r8(plan.wrapping_add(stride.wrapping_mul(col)).wrapping_add(row)) & 0x80 != 0
}

/// `0x00559970(ecx = objeto, &índice)`: siguiente objeto de la lista de la casilla.
pub fn links_next(m: &mut dyn Mem, obj: u32, slot: &mut u8) -> u32 {
    let e = m.r32(obj + 4).wrapping_add(u32::from(*slot) * 4);
    let id = m.r16(e);
    if id == 0 {
        return 0;
    }
    *slot = m.r8(e + 2);
    let v = view(m);
    ld32(m, v + 0x3FC, u32::from(id) * 4)
}

/// Primer objeto de la lista de la casilla `idx` y su índice.
fn links_first(m: &mut dyn Mem, idx: u32) -> (u32, u8) {
    let v = view(m);
    let cell = m.r32(v + 0xC310).wrapping_add(idx.wrapping_mul(4));
    let slot = m.r8(cell + 2);
    let id = m.r16(cell);
    (ld32(m, v + 0x3FC, u32::from(id) * 4), slot)
}

/// `0x0055AD90(ecx = reglas, &casilla, enlace, objetos, figuras)`: ¿la figura puede entrar en la casilla?
pub fn cell_walkable(m: &mut dyn Mem, h: &mut dyn FigureHost, order: u32, col: u32, row: u32, link: u16, objects: bool, figures: bool) -> bool {
    let v = view(m);
    let w = m.r32(v + 0xC314);
    if col >= w || row >= m.r32(v + 0xC318) {
        return true;
    }
    let idx = w.wrapping_mul(row).wrapping_add(col);
    let up = ld16(m, v + 0xC30C, idx.wrapping_mul(4));
    let k0 = ground_kind(m, h, idx);
    if !objects {
        return k0 == 0xE;
    }
    if up != 0 {
        let v = view(m);
        let obj = ld32(m, v + 0x98, u32::from(up) * 4);
        match h.tile_kind(m, obj) {
            0x2F => {
                if m.r8(order + 6) == 0 || m.r8(order + 4) == 0 || k0 != 0x18 {
                    return false;
                }
            }
            0x30 | 0x31 => {}
            _ => return false,
        }
    }
    if figures {
        let (mut e, mut slot) = links_first(m, idx);
        while e != 0 {
            if m.r16(e + 0x16) != link && (m.r8(order + 6) == 0 || vcall(m, h, e, 0x20, &[]) as u8 != 0x1F) {
                return false;
            }
            e = links_next(m, e, &mut slot);
        }
    }
    match k0 {
        0x10 | 0x17 | 0x18 => {
            if m.r8(order + 6) != 0 {
                return true;
            }
            let s = street(m, col, row);
            let b5 = m.r8(order + 5) != 0;
            if s {
                !b5
            } else {
                b5
            }
        }
        0x28 => {
            let s = street(m, col, row);
            let b5 = m.r8(order + 5) != 0;
            if s == b5 {
                return false;
            }
            m.r8(order + 8) == 0
        }
        _ => false,
    }
}

/// `0x00639F78` (`abs`).
fn abs(v: i32) -> i32 {
    v.wrapping_abs()
}

/// Posición de una entidad por su método `+0x2C` (`0x005586B0`: píxel `+0xE/+0x12`).
fn entity_position(m: &mut dyn Mem, h: &mut dyn FigureHost, e: u32) -> (i32, i32) {
    let f = ld32(m, e, 0x2C);
    if f == 0x0055_86B0 {
        (s16(m, e + 0xE), s16(m, e + 0x12))
    } else {
        let p = h.virt(m, f, e, &[]);
        (p as i16 as i32, (p >> 16) as i16 as i32)
    }
}

/// Bloque del asedio de `0x0055A2C0`: una figura de las listas del asalto a menos de 500 px en el rumbo exacto
/// impide el paso.
pub fn siege_blocks(m: &mut dyn Mem, h: &mut dyn FigureHost, base: u32, dir: u8) -> bool {
    let v = view(m);
    let city = m.r32(v + 0xC324);
    let s = ld32(m, super::SIEGES, city.wrapping_mul(0x1024) + 8);
    if s == 0 || m.r16(s + 0x3C) == 0 {
        return false;
    }
    const LISTS: [u32; 4] = [0x7C, 0x11C, 0x174, 0x1BC];
    for (k, &list) in LISTS.iter().enumerate() {
        let mut i = 0u8;
        while u16::from(i) < m.r16(s + 0x18 + 2 * k as u32) {
            let unit = m.r32(s + list + u32::from(i) * 4);
            if unit != 0 {
                let (x, y) = entity_position(m, h, unit);
                let dx = x.wrapping_sub(s16(m, base + 6));
                let dy = y.wrapping_sub(s16(m, base + 0xA));
                let ax = abs(dx) & 0xFFFF;
                let ay = abs(dy) & 0xFFFF;
                if ax + ay < 500 {
                    let cone = ay <= 2 * ax && ax <= 2 * ay;
                    let hit = match dir {
                        0x00 => dy < 0 && ay >= 2 * ax,
                        0x20 => dy < 0 && dx > 0 && cone,
                        0x40 => dx > 0 && ax >= 2 * ay,
                        0x60 => dy > 0 && dx > 0 && cone,
                        0x80 => dy > 0 && ay >= 2 * ax,
                        0xA0 => dy > 0 && dx < 0 && cone,
                        0xC0 => dx < 0 && ax >= 2 * ay,
                        // El original prueba 0xD0 (no 0xE0) para el octante noroeste.
                        0xD0 => dy < 0 && dx < 0 && cone,
                        _ => false,
                    };
                    if hit {
                        return true;
                    }
                }
            }
            i = i.wrapping_add(1);
        }
    }
    false
}

/// `0x0055A2C0(ecx = reglas, base, rumbo, objetos, figuras)`: ¿la figura puede dar un paso en el rumbo `dir`?
/// Escribe `reglas + 8` (la casilla actual cumple las reglas).
pub fn check_step(m: &mut dyn Mem, h: &mut dyn FigureHost, order: u32, base: u32, dir: u8, objects: bool, figures: bool) -> bool {
    if objects && siege_blocks(m, h, base, dir) {
        return false;
    }
    let link = m.r16(base + 0xE);
    let mut lx = m.r32(base + 4) >> 16;
    let mut ly = m.r32(base + 8) >> 16;
    let b7 = u32::from(m.r8(order + 7));
    let s = b7 as i32;
    let mut passes = 1u8;
    if b7 != 0 {
        passes = 2;
        if !(0x10..0xF0).contains(&dir) {
            lx = lx.wrapping_sub(b7);
        } else if dir < 0x30 {
            lx = lx.wrapping_add((s * -0x11 / 0x14) as u32);
            ly = ly.wrapping_add((s * -0xB / 0x14) as u32);
        } else if dir < 0x50 {
            ly = ly.wrapping_add((s * -0xB / 0x11) as u32);
        } else if dir < 0x70 {
            lx = lx.wrapping_add((s * 0x11 / 0x14) as u32);
            ly = ly.wrapping_add((s * -0xB / 0x14) as u32);
        } else if dir < 0x90 {
            lx = lx.wrapping_add(b7);
        } else if dir < 0xB0 {
            lx = lx.wrapping_add((s * 0x11 / 0x14) as u32);
            ly = ly.wrapping_add((s * 0xB / 0x14) as u32);
        } else if dir < 0xD0 {
            ly = ly.wrapping_add((s * 0xB / 0x11) as u32);
        } else {
            lx = lx.wrapping_add((s * -0x11 / 0x14) as u32);
            ly = ly.wrapping_add((s * 0xB / 0x14) as u32);
        }
    }
    loop {
        let (mut col, mut row) = h.pixel_to_cell(m, lx as i32, ly as i32);
        if m.r8(order + 4) != 0 {
            m.w8(order + 8, 1);
            if !cell_walkable(m, h, order, col, row, link, objects, false) {
                m.w8(order + 8, 0);
            }
        } else {
            m.w8(order + 8, 0);
        }
        macro_rules! go {
            ($d:expr) => {{
                h.step(m, &mut col, &mut row, $d);
                if !cell_walkable(m, h, order, col, row, link, objects, figures) {
                    return false;
                }
            }};
        }
        if !(0x10..0xF0).contains(&dir) {
            let (px, _) = h.cell_to_pixel(m, col, row);
            if lx < px as u32 {
                go!(0xE0);
                go!(0x20);
            } else {
                go!(0x20);
                go!(0xE0);
            }
            lx = lx.wrapping_add(b7 * 2);
        } else if dir < 0x30 {
            go!(0x20);
            lx = lx.wrapping_add((s * 0x11 / 10) as u32);
            ly = ly.wrapping_add((s * 0xB / 10) as u32);
        } else if dir < 0x50 {
            let (_, py) = h.cell_to_pixel(m, col, row);
            if ly < py as u32 {
                go!(0x20);
                go!(0x60);
            } else {
                go!(0x60);
                go!(0x20);
            }
            ly = ly.wrapping_add((s * 0x16 / 0x11) as u32);
        } else if dir < 0x70 {
            go!(0x60);
            lx = lx.wrapping_add((s * -0x11 / 10) as u32);
            ly = ly.wrapping_add((s * 0xB / 10) as u32);
        } else if dir < 0x90 {
            let (px, _) = h.cell_to_pixel(m, col, row);
            if lx > px as u32 {
                go!(0x60);
                go!(0xA0);
            } else {
                go!(0xA0);
                go!(0x60);
            }
            lx = lx.wrapping_add((s * -2) as u32);
        } else if dir < 0xB0 {
            go!(0xA0);
            lx = lx.wrapping_add((s * -0x11 / 10) as u32);
            ly = ly.wrapping_add((s * -0xB / 10) as u32);
        } else if dir < 0xD0 {
            let (_, py) = h.cell_to_pixel(m, col, row);
            if ly > py as u32 {
                go!(0xA0);
                go!(0xE0);
            } else {
                go!(0xE0);
                go!(0xA0);
            }
            ly = ly.wrapping_add((s * -0x16 / 0x11) as u32);
        } else {
            go!(0xE0);
            lx = lx.wrapping_add((s * 0x11 / 10) as u32);
            ly = ly.wrapping_add((s * -0xB / 10) as u32);
        }
        passes -= 1;
        if passes == 0 {
            return true;
        }
    }
}

/// `0x0055D700(ecx = caminante, &casilla, enlace)`: la única figura «de paso» de la casilla (clases 0–4 y 7; la 8
/// solo sin asalto activo), o 0 si no hay, hay dos, otra clase la ocupa o la casilla no admite al caminante.
pub fn single_occupant(m: &mut dyn Mem, h: &mut dyn FigureHost, w: u32, col: u32, row: u32, link: u16) -> u32 {
    let v = view(m);
    let idx = m.r32(v + 0xC314).wrapping_mul(row).wrapping_add(col);
    let k0 = ground_kind(m, h, idx);
    match k0 {
        0x10 | 0x17 | 0x18 => {
            if m.r8(w + 0x26) == 0 && !street(m, col, row) {
                return 0;
            }
        }
        0x28 => {
            if m.r8(w + 0x28) != 0 {
                return 0;
            }
        }
        _ => return 0,
    }
    let v = view(m);
    let up = ld16(m, v + 0xC30C, idx.wrapping_mul(4));
    if up != 0 {
        let obj = ld32(m, v + 0x98, u32::from(up) * 4);
        match h.tile_kind(m, obj) {
            0x2F => {
                if m.r8(w + 0x26) == 0 || m.r8(w + 0x24) == 0 || k0 != 0x18 {
                    return 0;
                }
            }
            0x30 | 0x31 => {}
            _ => return 0,
        }
    }
    let (mut e, mut slot) = links_first(m, idx);
    let mut found = 0u32;
    while e != 0 {
        if m.r16(e + 0x16) != link {
            match vcall(m, h, e, 0x20, &[]) & 0xFF {
                0..=4 | 7 => {}
                8 => {
                    let v = view(m);
                    let city = m.r32(v + 0xC324) as u8;
                    let rec = siege_of(m, city);
                    if siege_active(m, rec) {
                        return 0;
                    }
                }
                _ => return 0,
            }
            if found != 0 {
                return 0;
            }
            found = e;
        }
        e = links_next(m, e, &mut slot);
    }
    found
}

/// `0x0055CF60(ecx = caminante, &casilla, conjunto)`: vector de avance hacia el centro de la casilla (rodeando a su
/// ocupante, `+0x54`): pasos `+0x3A` (paso actual `+0x38` a 0) y avance por paso `+0x40/+0x44` en 16.16.
pub fn begin_move(m: &mut dyn Mem, h: &mut dyn FigureHost, w: u32, col: u32, row: u32, set: u32) {
    let (mut tx, mut ty) = h.cell_to_pixel(m, col, row);
    let link = m.r16(w + 0x16);
    if single_occupant(m, h, w, col, row, link) != 0 {
        ty = if (tx as u32) < s16(m, w + 0xE) as u32 { ty.wrapping_sub(3) } else { ty.wrapping_add(3) };
        tx = if (ty as u32) < s16(m, w + 0x12) as u32 { tx.wrapping_add(4) } else { tx.wrapping_sub(4) };
        m.w8(w + 0x54, 1);
    } else {
        m.w8(w + 0x54, 0);
    }
    let dx = tx.wrapping_sub(s16(m, w + 0xE));
    let dy = ty.wrapping_sub(s16(m, w + 0x12));
    m.w32(w + 0x40, dx as u32);
    m.w32(w + 0x44, dy as u32);
    let d2 = dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx));
    let t = f64::from(d2).sqrt() * 10000.0 / f64::from(m.r32(set + 0xC)) + 0.5;
    let steps = if t.is_finite() && t.abs() < 9.2e18 { (t as i64) as i32 } else { 0 };
    m.w16(w + 0x38, 0);
    m.w16(w + 0x3A, steps as u16);
    if steps != 0 {
        m.w32(w + 0x40, (dx.wrapping_shl(16)).wrapping_div(steps) as u32);
        m.w32(w + 0x44, (dy.wrapping_shl(16)).wrapping_div(steps) as u32);
    }
}

/// `0x0055D910(ecx = caminante)`: rumbo hacia el destino `+0x50/+0x52`; escribe el rumbo del eje en `+0x3D`.
pub fn heading_to_target(m: &mut dyn Mem, w: u32) -> u8 {
    let x = s16(m, w + 0xE);
    let y = s16(m, w + 0x12);
    let tx = i32::from(m.r16(w + 0x50));
    let ty = i32::from(m.r16(w + 0x52));
    let pick = |dx: i32, dy: i32, base: u8| base.wrapping_add((((dx <= dy.wrapping_mul(0x11)) as u32).wrapping_sub(1) & 0x80) as u8);
    if tx > x {
        let dx = (tx - x).wrapping_mul(0xB);
        if ty > y {
            m.w8(w + 0x3D, 0x60);
            pick(dx, ty - y, 0xA0)
        } else {
            m.w8(w + 0x3D, 0x20);
            pick(dx, y - ty, 0xE0)
        }
    } else {
        let dx = (x - tx).wrapping_mul(0xB);
        if ty > y {
            m.w8(w + 0x3D, 0xA0);
            pick(dx, ty - y, 0x60)
        } else {
            m.w8(w + 0x3D, 0xE0);
            pick(dx, y - ty, 0x20)
        }
    }
}

/// `0x0055A1C0(&rumbo, objetivo, n)`: gira un paso de `0x100 / n` hacia `objetivo` (0x00 y 0xE0 se tocan).
pub fn turn_toward(m: &mut dyn Mem, at: u32, target: u8, n: u8) {
    let cur = m.r8(at);
    let (cw, ccw) = (target.wrapping_sub(cur), cur.wrapping_sub(target));
    // El paso solo se calcula fuera de los saltos 0xE0 <-> 0x00; con `n == 0` el original divide entre cero
    // (excepción) y aquí no gira.
    let step = || 0x100i32.checked_div(i32::from(n)).map(|s| s as u8);
    let (d0, d3) = (DIRS[0], DIRS[7]);
    if ccw > cw {
        if cur == d3 {
            m.w8(at, d0);
        } else if let Some(k) = step() {
            m.w8(at, cur.wrapping_add(k));
        }
    } else if cur == d0 {
        m.w8(at, d3);
    } else if let Some(k) = step() {
        m.w8(at, cur.wrapping_sub(k));
    }
}
