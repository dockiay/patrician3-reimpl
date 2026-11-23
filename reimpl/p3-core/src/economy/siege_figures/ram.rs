//! Ariete del asalto (clase `0x16`, vtabla `0x006760C0`): recorre una de las rutas de aproximación de la vista
//! (`vista+0xC674`/`+0xC678`, punto medio de los dos carriles), se para delante de la puerta (pieza `0x2F`) y,
//! si el asalto sigue, da la vuelta; marca en la escena del asedio las cuatro casillas que ocupa (bit `0x10`).
//! Con daño (`+0x2E`) echa humo y arde, y al destruirse (`+0x2C`) deja polvo, fuego y astillas (clase `0x13`).
//!
//! Campos: `+0x14` rumbo dibujado, `+0x15` fotograma, `+0x16` enlace, `+0x20` estado de animación (`+0x28`
//! código), `+0x2C` destruido, `+0x2E` daño, `+0x30` empuje pendiente, `+0x34` contador de fotogramas, `+0x35`
//! conjunto (`0x006E2E14`), `+0x38/+0x3C` velocidad 16.16 por ms, `+0x40` de vuelta, `+0x41` punto de la ruta,
//! `+0x42` rumbo hacia el punto, `+0x44/+0x46` punto (píxel), `+0x48` ruta, `+0x49` ciudad, `+0x4A/+0x4C` tiempo
//! acumulado y periodo del paso, `+0x50/+0x54` casilla marcada, `+0x58` empujando, `+0x5C/+0x60` siguiente humo
//! y anterior, `+0x64` efecto de fuego.
use super::sprite::{anim_select, anim_set_default, clip, erase, frame_rects, place, register_cells, set_by};
use super::walker::{clear_mark, distance_to_target, mark_at_least, set_mark};
use super::{s16, siege_of, vcall, view, FigureHost, CLOCK, DIRS};
use crate::flatmem::Mem;
use crate::ships::wind::table_sine;

/// Conjuntos del ariete (`{cuántos, conjuntos}`).
pub const SETS: u32 = 0x006E_2E14;

/// `0x005758E0` (método `+0x4C`): conjunto del ariete.
pub fn ram_set(m: &mut dyn Mem, e: u32) -> u32 {
    set_by(m, e + 0x35, SETS)
}

fn scene_of(m: &mut dyn Mem, e: u32) -> u32 {
    let city = m.r8(e + 0x49);
    let rec = siege_of(m, city);
    m.r32(rec + 8)
}

/// Punto `i` del carril `table` (`vista+0xC674` o `+0xC678`) de la ruta `k`: `(x, y)`.
fn lane_point(m: &mut dyn Mem, table: u32, k: u32, i: u32) -> (u32, u32) {
    let v = view(m);
    let t = m.r32(v + table);
    let pts = m.r32(t.wrapping_add(k * 8 + 4));
    let p = pts.wrapping_add(i * 8);
    (m.r32(p), m.r32(p + 4))
}

fn lane_len8(m: &mut dyn Mem, table: u32, k: u32) -> u8 {
    let v = view(m);
    let t = m.r32(v + table);
    m.r8(t.wrapping_add(k * 8))
}

/// `0x00575560(ecx = ariete, x, y)`: siguiente punto de la ruta (punto medio del punto `+0x41` del primer carril y
/// del simétrico del segundo, 22 px más abajo), rumbo diagonal hacia él desde `(x, y)` y velocidad.
pub fn calc_dir(m: &mut dyn Mem, e: u32, a: u16, b: u16) {
    let set = ram_set(m, e);
    let k = u32::from(m.r8(e + 0x48));
    let i = m.r8(e + 0x41);
    let j = lane_len8(m, 0xC678, k).wrapping_sub(i).wrapping_sub(1);
    let (ax, ay) = lane_point(m, 0xC674, k, u32::from(i));
    let (bx, by) = lane_point(m, 0xC678, k, u32::from(j));
    m.w16(e + 0x44, ((ax.wrapping_add(bx) as i32) >> 1) as u16);
    let y = (by.wrapping_add(ay).wrapping_add(0x16) as i32 >> 1) as u16;
    m.w16(e + 0x46, y);
    let s = m.r32(set + 0xC);
    let f = ((u64::from(s.wrapping_mul(31856)) * 0xC561_06EB) >> 45) as u32;
    let g = ((u64::from(s.wrapping_mul(2896)) * 0xD1B7_1759) >> 41) as u32;
    let left = m.r16(e + 0x44) < a;
    let (hd, vy) = match (left, y < b) {
        (true, true) => (DIRS[7], f.wrapping_neg()),
        (true, false) => (DIRS[5], f),
        (false, true) => (DIRS[1], f.wrapping_neg()),
        (false, false) => (DIRS[3], f),
    };
    m.w8(e + 0x42, hd);
    m.w32(e + 0x3C, vy);
    m.w32(e + 0x38, if left { g.wrapping_neg() } else { g });
}

/// `0x00575730(ecx = ariete, columna, fila, rumbo)`: ¿a dos pasos hay una pieza de puerta (`0x2F`) a menos de 10
/// casillas del destino del asalto?
pub fn gate_near(m: &mut dyn Mem, h: &mut dyn FigureHost, e: u32, col: u32, row: u32, dir: u8) -> bool {
    let (mut c, mut r) = (col, row);
    h.step(m, &mut c, &mut r, dir);
    h.step(m, &mut c, &mut r, dir);
    let v = view(m);
    let w = m.r32(v + 0xC314);
    if c >= w || r >= m.r32(v + 0xC318) {
        return false;
    }
    let up = m.r32(v + 0xC30C);
    let id = m.r16(up.wrapping_add(w.wrapping_mul(r).wrapping_add(c).wrapping_mul(4)));
    if id == 0 {
        return false;
    }
    let objs = m.r32(v + 0x98);
    let o = m.r32(objs.wrapping_add(4 * u32::from(id)));
    if h.tile_kind(m, o) != 0x2F {
        return false;
    }
    let s = scene_of(m, e);
    (distance_to_target(m, s, c, r) as u16) < 10
}

/// Marca (o borra) el bit `0x10` en la casilla y en las tres que ocupa el ariete (pasos `0xE0`, `0x40`, `0xE0`).
fn mark_four(m: &mut dyn Mem, h: &mut dyn FigureHost, s: u32, mut c: u32, mut r: u32, on: bool) {
    let f = if on { set_mark } else { clear_mark };
    f(m, s, c, r, 0x10);
    for d in [DIRS[7], DIRS[2], DIRS[7]] {
        h.step(m, &mut c, &mut r, d);
        f(m, s, c, r, 0x10);
    }
}

/// Como [`mark_four`] para borrar, con los pasos sobre la casilla guardada (`+0x50/+0x54`), como el original.
fn clear_stored(m: &mut dyn Mem, h: &mut dyn FigureHost, e: u32, s: u32) {
    let (c, r) = (m.r32(e + 0x50), m.r32(e + 0x54));
    clear_mark(m, s, c, r, 0x10);
    for d in [DIRS[7], DIRS[2], DIRS[7]] {
        let (mut c, mut r) = (m.r32(e + 0x50), m.r32(e + 0x54));
        h.step(m, &mut c, &mut r, d);
        m.w32(e + 0x50, c);
        m.w32(e + 0x54, r);
        clear_mark(m, s, c, r, 0x10);
    }
}

/// `0x005740F0(ecx = ariete, x, y, rumbo, enlace, variante)` (método `+0x40`): conjunto al azar, ruta `variante >>
/// 8` y ciudad `variante & 0xFF`; empieza en el primer punto, marca sus casillas y calcula el periodo del paso.
pub fn ram_init(m: &mut dyn Mem, h: &mut dyn FigureHost, e: u32, a: [u32; 5]) {
    let [_, _, _, id, variant] = a;
    let n = m.r32(SETS);
    let r = h.rand(m);
    let pick = r.checked_rem(n).unwrap_or(0);
    m.w8(e + 0x49, variant as u8);
    m.w8(e + 0x48, (variant >> 8) as u8);
    m.w8(e + 0x35, pick as u8);
    let set = ram_set(m, e);
    let f = anim_select(m, e + 0x20, set, 2);
    m.w8(e + 0x15, f);
    m.w32(e + 0x24, 0);
    super::spawn::links_alloc(m, h, e, set);
    m.w16(e + 0x16, id as u16);
    let k = u32::from(m.r8(e + 0x48));
    let j = u32::from(lane_len8(m, 0xC678, k).wrapping_sub(1));
    let (ax, ay) = lane_point(m, 0xC674, k, 0);
    let (bx, by) = lane_point(m, 0xC678, k, j);
    m.w32(e + 0xC, ax.wrapping_add(bx) << 15);
    m.w8(e + 0x40, 0);
    m.w8(e + 0x41, 1);
    m.w32(e + 0x10, by.wrapping_add(ay).wrapping_add(0x16) << 15);
    let (x, y) = (m.r16(e + 0xE), m.r16(e + 0x12));
    calc_dir(m, e, x, y);
    let hd = m.r8(e + 0x42);
    m.w8(e + 0x14, hd);
    let (ra, rb) = frame_rects(m, set, e + 8);
    clip(m, h, &ra);
    if let Some(b) = rb {
        clip(m, h, &b);
    }
    register_cells(m, h, set, e + 8);
    let (px, py) = (s16(m, e + 0xE), s16(m, e + 0x12));
    let (c, r) = h.pixel_to_cell(m, px, py);
    m.w32(e + 0x50, c);
    m.w32(e + 0x54, r);
    let s = scene_of(m, e);
    mark_four(m, h, s, c, r, true);
    let period = 0x3_AB20u32.checked_div(m.r32(set + 0xC)).unwrap_or(0);
    m.w16(e + 0x4A, 0);
    m.w32(e + 0x30, 0);
    m.w16(e + 0x4C, period as u16);
    m.w8(e + 0x58, 0);
    m.w16(e + 0x2E, 0xFFFF);
    m.w32(e + 0x5C, 0xFFFF_FFFF);
    m.w32(e + 0x60, 0);
    m.w32(e + 0x64, 0);
}

/// `0x00574360` (método `+0x44`): borra el ariete de la vista (sin quitar sus marcas).
pub fn ram_remove(m: &mut dyn Mem, h: &mut dyn FigureHost, e: u32) {
    let set = ram_set(m, e);
    erase(m, h, false, set, e + 8);
}

/// `0x00575810(ecx = estado de animación, conjunto)` (método `+8` del estado): al acabar la animación de parada
/// (`0xF`) con el asalto en marcha vuelve a andar (2) y emprende la vuelta.
pub fn ram_anim_finished(m: &mut dyn Mem, anim: u32) {
    if m.r8(anim + 8) != 0xF {
        return;
    }
    let e = anim - 0x20;
    let s = scene_of(m, e);
    if m.r16(s + 0x3A) == 0 && m.r8(s + 0x78) == 0 {
        return;
    }
    anim_set_default(m, anim, 2);
    if m.r8(e + 0x40) == 0 {
        m.w8(e + 0x40, 1);
        let i = m.r8(e + 0x41);
        m.w8(e + 0x41, i.wrapping_sub(1));
        let (x, y) = (m.r16(e + 0xE), m.r16(e + 0x12));
        calc_dir(m, e, x, y);
    }
    m.w16(e + 0x4A, 0);
}

fn sound(m: &mut dyn Mem, h: &mut dyn FigureHost, e: u32, list: u32, offset: i32) {
    if h.sound_list_size(m, list) == 0 {
        return;
    }
    let n = h.sound_list_size(m, list) as i32;
    let r = (h.rand(m) as i32).checked_rem(n).unwrap_or(0);
    let (x, y) = (s16(m, e + 0xE), s16(m, e + 0x12));
    h.play_sound(m, x, y, r.wrapping_sub(offset) as u32);
}

fn release_fire(m: &mut dyn Mem, h: &mut dyn FigureHost, e: u32) {
    let fx = m.r32(e + 0x64);
    if fx != 0 {
        h.release_entity(m, fx);
        m.w32(e + 0x64, 0);
    }
}

/// Reparto del empuje pendiente `+0x30` en el tiempo del paso (`+0x4A` hasta `+0x4C`).
fn absorb_push(m: &mut dyn Mem, e: u32, rem: &mut u32, head: Option<&mut u8>) {
    let a = m.r16(e + 0x4A);
    let c = m.r16(e + 0x4C);
    let d = m.r32(e + 0x30);
    if a >= c || d == 0 {
        return;
    }
    let (au, cu) = (u32::from(a), u32::from(c));
    if au.wrapping_add(d) > cu {
        if a == 0 {
            let half = cu.wrapping_sub(*rem) >> 1;
            *rem = rem.wrapping_add(half);
            m.w16(e + 0x4A, a.wrapping_add(*rem as u16));
            m.w32(e + 0x30, d.wrapping_sub(half));
        } else {
            m.w32(e + 0x30, d.wrapping_sub(cu).wrapping_add(au));
            *rem = rem.wrapping_add(cu.wrapping_sub(au));
            m.w16(e + 0x4A, c);
        }
    } else {
        if let Some(hd) = head {
            *hd = m.r8(e + 0x42);
        }
        *rem = rem.wrapping_add(d);
        let d16 = m.r16(e + 0x30);
        m.w32(e + 0x30, 0);
        let a = m.r16(e + 0x4A);
        m.w16(e + 0x4A, a.wrapping_add(d16));
    }
}

/// Resta `rem` del tiempo del paso, dando la vuelta al periodo si no llega.
fn rewind_step(m: &mut dyn Mem, e: u32, rem: u32) {
    let a = m.r16(e + 0x4A);
    if rem > u32::from(a) {
        let c = m.r16(e + 0x4C);
        m.w16(e + 0x4A, c.wrapping_sub(rem as u16).wrapping_add(a));
    } else {
        m.w16(e + 0x4A, a.wrapping_sub(rem as u16));
    }
}

/// `0x005743A0` (método `+0x14`): paso por fotograma del ariete. Devuelve 0 cuando desaparece (destruido o al
/// final de la vuelta).
pub fn ram_update(m: &mut dyn Mem, h: &mut dyn FigureHost, e: u32) -> u8 {
    let set = ram_set(m, e);
    let s = scene_of(m, e);
    if s == 0 {
        return 0;
    }
    let mut x = m.r32(e + 0xC);
    let mut y = m.r32(e + 0x10);
    let mut head = m.r8(e + 0x14);
    let mut rem = { let v = view(m); m.r32(v + 0x414) }.min(1000);
    let old34 = m.r8(e + 0x34);
    let mut frame = vcall(m, h, e + 0x20, 4, &[set, e + 8, e + 0x34]) as u8;
    if m.r8(e + 0x2C) != 0 {
        release_fire(m, h, e);
        let (ex, ey) = (m.r32(e + 0xC), m.r32(e + 0x10));
        h.spawn_entity(m, [ex, ey, 0x12, 0xB_0023, u32::from(DIRS[1]), 0, 0]);
        let (ex, ey) = (m.r32(e + 0xC), m.r32(e + 0x10));
        let fx = h.spawn_entity(m, [ex, ey, 0x12, 0xC2, u32::from(DIRS[1]), 0, 0]);
        h.set_fade(m, fx, 2000, 0xFFFF_FF80);
        for _ in 0..5 {
            let r = h.rand(m);
            let (ex, ey) = (m.r32(e + 0xC), m.r32(e + 0x10));
            h.spawn_entity(m, [ex, ey, 0x13, 0, r, 0, 0]);
        }
        sound(m, h, e, 0xEA7E, 0x1582);
        clear_stored(m, h, e, s);
        return 0;
    }
    if frame == 6 && m.r8(e + 0x15) != 6 {
        sound(m, h, e, 0xEE7A, 0x1186);
        let hd = m.r8(e + 0x14);
        let ex = m.r32(e + 0xC);
        let fx = if hd < 0x80 { ex.wrapping_add(0x1A_0000) } else { ex.wrapping_sub(0x1A_0000) };
        let ey = m.r32(e + 0x10);
        let fy = if hd == 0x60 || hd == 0xA0 { ey.wrapping_add(0x5_0000) } else { ey.wrapping_sub(0x1B_0000) };
        h.spawn_entity(m, [fx, fy, 0x12, 0x8_0020, u32::from(DIRS[1]), 0, 0]);
    }
    let mut n = 1u8;
    if m.r32(e + 0x30) != 0 {
        n = 0xFF;
        m.w8(e + 0x58, 1);
    }
    let mut i = 0u8;
    loop {
        if i != 0 {
            rem = 0;
        }
        let (sx, sy) = (x, y);
        if m.r8(e + 0x28) == 2 {
            let (px, py) = (x >> 16, y >> 16);
            if m.r8(e + 0x40) == 0 && (m.r16(s + 0x3A) != 0 || m.r8(s + 0x78) != 0) && m.r32(e + 0x30) == 0 {
                m.w8(e + 0x40, 1);
                let c = m.r8(e + 0x41);
                m.w8(e + 0x41, c.wrapping_sub(1));
                calc_dir(m, e, px as u16, py as u16);
                let a = m.r16(e + 0x4A);
                if a > 0 {
                    let c = m.r16(e + 0x4C);
                    m.w16(e + 0x4A, c.wrapping_sub(a));
                }
            }
            let tx = u32::from(m.r16(e + 0x44)) as i32;
            let vx = m.r32(e + 0x38) as i32;
            let ty = u32::from(m.r16(e + 0x46)) as i32;
            let vy = m.r32(e + 0x3C) as i32;
            let (pxi, pyi) = (px as i32, py as i32);
            let reached = (pxi <= tx && vx < 0) || (pxi >= tx && vx > 0) || (pyi <= ty && vy < 0) || (pyi >= ty && vy > 0);
            if reached {
                if m.r8(e + 0x40) != 0 {
                    let c = m.r8(e + 0x41);
                    if c == 0 {
                        clear_stored(m, h, e, s);
                        release_fire(m, h, e);
                        return 0;
                    }
                    m.w8(e + 0x41, c - 1);
                } else {
                    let k = u32::from(m.r8(e + 0x48));
                    let len = { let v = view(m); let t = m.r32(v + 0xC674); m.r32(t.wrapping_add(k * 8)) }.wrapping_sub(1);
                    if u32::from(m.r8(e + 0x41)) != len {
                        let c = m.r8(e + 0x41).wrapping_add(1);
                        m.w8(e + 0x41, c);
                        if u32::from(c) == len {
                            m.w32(e + 0x30, 0);
                            rem = 0;
                        }
                    }
                }
                x = (tx as u32) << 16;
                y = u32::from(m.r16(e + 0x46)) << 16;
                let (a, b) = (m.r16(e + 0x44), m.r16(e + 0x46));
                calc_dir(m, e, a, b);
                let a = m.r16(e + 0x4A);
                m.w16(e + 0x4A, a.wrapping_add(rem as u16));
                let c = m.r16(e + 0x4C);
                absorb_push(m, e, &mut rem, Some(&mut head));
                let a = m.r16(e + 0x4A);
                if a >= c {
                    rem = rem.wrapping_add(u32::from(c)).wrapping_sub(u32::from(a));
                    m.w16(e + 0x4A, 0);
                }
            } else {
                let a = m.r16(e + 0x4A);
                m.w16(e + 0x4A, a.wrapping_add(rem as u16));
                let c = m.r16(e + 0x4C);
                absorb_push(m, e, &mut rem, None);
                let a = m.r16(e + 0x4A);
                let (vx, vy) = (m.r32(e + 0x38), m.r32(e + 0x3C));
                if a >= c {
                    rem = rem.wrapping_add(u32::from(c)).wrapping_sub(u32::from(a));
                    let nx = vx.wrapping_mul(rem).wrapping_add(x);
                    let ny = vy.wrapping_mul(rem).wrapping_add(y);
                    m.w16(e + 0x4A, 0);
                    let (cc, cr) = h.pixel_to_cell(m, (nx >> 16) as i32, (ny >> 16) as i32);
                    let (qx, qy) = h.cell_to_pixel(m, cc, cr);
                    x = (qx as u32) << 16;
                    y = (qy as u32) << 16;
                } else {
                    x = x.wrapping_add(vx.wrapping_mul(rem));
                    y = y.wrapping_add(vy.wrapping_mul(rem));
                }
            }
        }
        let (px, py) = (x >> 16, y >> 16);
        let (mut nc, mut nr) = h.pixel_to_cell(m, px as i32, py as i32);
        let (oc, or) = (m.r32(e + 0x50), m.r32(e + 0x54));
        if nc != oc || nr != or {
            let mut accept = true;
            clear_stored(m, h, e, s);
            let (a, b) = (m.r16(s + 0x36), m.r16(s + 0x38));
            let d0 = distance_to_target(m, s, u32::from(a), u32::from(b)) as u16;
            let thr = (u32::from(m.r16(s + 0x42)).wrapping_mul(u32::from(d0))).checked_div(u32::from(m.r16(s + 0x66))).unwrap_or(0);
            let (c, r) = (m.r32(e + 0x50), m.r32(e + 0x54));
            if u32::from(distance_to_target(m, s, c, r) as u16) < thr {
                accept = false;
            }
            if m.r8(e + 0x40) == 0 {
                let hd = m.r8(e + 0x42);
                if gate_near(m, h, e, oc, or, hd) {
                    head = m.r8(e + 0x42);
                    if m.r16(s + 0x3A) == 0 && m.r8(s + 0x78) == 0 {
                        frame = anim_select(m, e + 0x20, set, 0xF);
                    } else {
                        m.w8(e + 0x40, 1);
                        let c = m.r8(e + 0x41);
                        m.w8(e + 0x41, c.wrapping_sub(1));
                        calc_dir(m, e, px as u16, py as u16);
                        rewind_step(m, e, rem);
                        let a = m.r16(e + 0x4A);
                        if a > 0 {
                            let c = m.r16(e + 0x4C);
                            m.w16(e + 0x4A, c.wrapping_sub(a));
                        }
                        rem = 0;
                    }
                    accept = false;
                    m.w32(e + 0x30, 0);
                }
            }
            let (mut c, mut r) = (nc, nr);
            if mark_at_least(m, s, c, r, 1) {
                accept = false;
            }
            for d in [DIRS[7], DIRS[2], DIRS[7]] {
                h.step(m, &mut c, &mut r, d);
                if mark_at_least(m, s, c, r, 1) {
                    accept = false;
                }
            }
            let (tx, ty) = (u32::from(m.r16(e + 0x44)) as i32, u32::from(m.r16(e + 0x46)) as i32);
            let (tc, tr) = h.pixel_to_cell(m, tx, ty);
            if nc != tc || nr != tr {
                let extra = match m.r8(e + 0x42) {
                    0x20 => Some((false, [DIRS[2], DIRS[7]])),
                    0x60 => Some((true, [DIRS[2], DIRS[5]])),
                    0xA0 => Some((true, [DIRS[6], DIRS[3]])),
                    0xE0 => Some((false, [DIRS[6], DIRS[1]])),
                    _ => None,
                };
                if let Some((reset, ds)) = extra {
                    if reset {
                        c = nc;
                        r = nr;
                    }
                    for d in ds {
                        h.step(m, &mut c, &mut r, d);
                        if mark_at_least(m, s, c, r, 1) {
                            accept = false;
                        }
                    }
                }
            }
            if accept {
                m.w32(e + 0x50, nc);
                m.w32(e + 0x54, nr);
            } else {
                m.w32(e + 0x50, oc);
                m.w32(e + 0x54, or);
                nc = oc;
                nr = or;
                rewind_step(m, e, rem);
                x = sx;
                y = sy;
                m.w32(e + 0x30, 0);
                if frame != 0 {
                    frame = m.r8(e + 0x15);
                }
            }
            mark_four(m, h, s, nc, nr, true);
        }
        let d30 = m.r32(e + 0x30);
        if d30 == 0 && m.r8(e + 0x58) == 0 {
            let to = m.r8(e + 0x42);
            if old34 != m.r8(e + 0x34) && to != head && m.r8(e + 0x28) == 2 {
                head = if to.wrapping_sub(head) < 0x80 { head.wrapping_add(0x20) } else { head.wrapping_sub(0x20) };
            }
        } else {
            head = m.r8(e + 0x42);
            if n == 1 {
                m.w8(e + 0x58, 0);
            }
        }
        let k = if d30 == 0 { n.wrapping_sub(1) } else { i };
        i = k.wrapping_add(1);
        if i >= n {
            break;
        }
    }
    let changed = (x >> 16) as i32 != s16(m, e + 0xE) || (y >> 16) as i32 != s16(m, e + 0x12) || frame != m.r8(e + 0x15) || head != m.r8(e + 0x14);
    let rect = if changed { erase(m, h, true, set, e + 8) } else { None };
    m.w32(e + 0xC, x);
    m.w32(e + 0x10, y);
    if changed {
        m.w8(e + 0x15, frame);
        m.w8(e + 0x14, head);
        place(m, h, set, e + 8, rect);
    }
    if m.r8(e + 0x2C) != 0 {
        return 1;
    }
    let dmg = u32::from(m.r16(e + 0x2E));
    if dmg * 3 < 0x2_0000 {
        let next = m.r32(e + 0x5C);
        if m.r32(CLOCK) > next {
            m.w32(e + 0x60, next);
            let ry = h.rand(m) as i32;
            let fy = ((ry % 0xCF0 - 0x678) << 8) as u32;
            let fy = fy.wrapping_add(m.r32(e + 0x10));
            let rx = h.rand(m) as i32;
            let fx = ((rx % 0x1400 - 0xA00) << 8) as u32;
            let fx = fx.wrapping_add(m.r32(e + 0xC));
            let smoke = h.spawn_entity(m, [fx, fy, 0x12, 0x16_0024, u32::from(DIRS[1]), 0, 0]);
            let t = (((dmg * 1200) as i32 >> 16) as u32).wrapping_add(m.r32(CLOCK)).wrapping_add(0xC8);
            m.w32(e + 0x5C, t);
            let alpha = (((dmg * 345) as i32 >> 16) as u8).wrapping_add(0x19);
            let (direction, strength) = h.wind(m);
            let dir = direction as u8;
            let speed = (strength & 0xFF) as i32;
            let cos = table_sine(dir.wrapping_add(0x40));
            let sin = table_sine(dir);
            // Sin entidad el original escribiría en la página cero; aquí no se escribe.
            if smoke != 0 {
                m.w8(smoke + 0x30, alpha);
                m.w32(smoke + 0x40, (speed.wrapping_mul(sin).wrapping_add(500) / 1000) as u32);
                m.w32(smoke + 0x44, ((speed.wrapping_mul(cos)).wrapping_neg().wrapping_add(500) / 1000) as u32);
            }
        } else {
            let t = (((dmg * 1200) as i32 >> 16) as u32).wrapping_add(m.r32(CLOCK)).wrapping_add(0xC8);
            if t < next {
                m.w32(e + 0x5C, t);
            }
        }
    }
    if dmg * 3 < 0x1_0000 {
        let w = 3u8.wrapping_sub(((dmg * 9) >> 16) as u8);
        let fx = m.r32(e + 0x64);
        if fx != 0 {
            let fset = super::effect::effect_set(m, fx);
            if u32::from(w) == m.r32(fset + 4) {
                let (ex, ey) = (s16(m, e + 0xE), s16(m, e + 0x12));
                vcall(m, h, fx, 0x48, &[ex as u32, ey.wrapping_sub(0x16) as u32, 0]);
            } else {
                h.release_entity(m, fx);
                m.w32(e + 0x64, 0);
            }
        }
        if m.r32(e + 0x64) == 0 {
            let ex = m.r32(e + 0xC);
            let fx = h.spawn_entity(m, [ex, ex, 0x12, (u32::from(w) << 8).wrapping_add(0x16_00B0), u32::from(DIRS[1]), 0, 0]);
            m.w32(e + 0x64, fx);
            return 1;
        }
    } else {
        release_fire(m, h, e);
    }
    1
}
