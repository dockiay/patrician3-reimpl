//! Máquina de asedio de la clase `0x15` (vtabla `0x006752E0`, Ghidra `siege_unit_update`): recorre una ruta de
//! aproximación como el ariete ([`super::ram`]), pero se detiene donde le cabe la huella (`0x00565F50`) cerca de
//! los defensores o del destino, apunta al objetivo más cercano (edificio atacado, defensores de las tres listas del
//! registro de la ciudad, cabecillas o el destino del asalto; `0x005663D0`) y dispara un proyectil (clase `0x17`)
//! en el fotograma 0x11 de la animación de tiro (`0x00566E30`). Marca sus casillas con el bit 8. Con daño echa
//! humo y arde; al destruirse se derrumba (animaciones `0x91` y `0xB1`).
//!
//! Campos: `+0x14` rumbo dibujado, `+0x15` fotograma, `+0x20` estado de animación (`+0x28` código), `+0x2C`
//! destruida, `+0x2E` daño, `+0x30` contador de fotogramas, `+0x31` conjunto (`0x006E1734`), `+0x34/+0x38`
//! velocidad, `+0x3C` fase (0 ida, 1 hacia la posición de tiro, 2 en posición, 3 vuelta, 4 regreso a la ruta),
//! `+0x3D` punto de la ruta, `+0x3E` rumbo hacia el punto, `+0x40/+0x42` punto, `+0x44` ruta, `+0x45` ciudad,
//! `+0x46/+0x47` casilla delantera de la huella, `+0x48/+0x4A` punto donde dejó la ruta, `+0x4C/+0x4E` tiempo
//! del paso y periodo, `+0x50/+0x54` casilla marcada, `+0x58` objetivo, `+0x5C` tipo de objetivo (`0x80 +` lista,
//! `0x84` punto), `+0x60` empuje, `+0x64` tiros pendientes de la ráfaga, `+0x66/+0x68` píxel del objetivo, `+0x6A`
//! empujando, `+0x6C/+0x70` humo, `+0x74` fuego.
use super::sprite::{anim_select, anim_set_default, clip, erase, frame_rects, place, set_by};
use super::walker::{clear_mark, distance_to_target, mark_at_least, set_mark};
use super::{s16, siege_active, siege_of, vcall, view, FigureHost, CLOCK, DIRS, TOWNS};
use crate::economy::siege_battle::building_record;
use crate::flatmem::Mem;
use crate::ships::wind::table_sine;

/// Conjuntos de la clase (`{cuántos, conjuntos}`) y tablas por conjunto: fotograma de tiro y tiros por ráfaga.
pub const SETS: u32 = 0x006E_1734;
pub const FIRE_FRAMES: u32 = 0x006E_173C;
pub const BURSTS: u32 = 0x006E_1740;
const MAX: i32 = 0x7FFF_FFFF;

/// `0x00566FE0` (método `+0x4C`): conjunto de la máquina.
pub fn unit_set(m: &mut dyn Mem, e: u32) -> u32 {
    set_by(m, e + 0x31, SETS)
}

fn scene_of(m: &mut dyn Mem, e: u32) -> u32 {
    let city = m.r8(e + 0x45);
    let rec = siege_of(m, city);
    m.r32(rec + 8)
}

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

/// `0x00565E10(ecx = máquina, x, y)`: rumbo diagonal hacia el punto `+0x40/+0x42` desde `(x, y)` y velocidad.
pub fn heading_velocity(m: &mut dyn Mem, e: u32, a: u16, b: u16) {
    let set = unit_set(m, e);
    let s = m.r32(set + 0xC);
    let f = ((u64::from(s.wrapping_mul(31856)) * 0xC561_06EB) >> 45) as u32;
    let g = ((u64::from(s.wrapping_mul(2896)) * 0xD1B7_1759) >> 41) as u32;
    let left = m.r16(e + 0x40) < a;
    let (hd, vy) = match (left, m.r16(e + 0x42) < b) {
        (true, true) => (DIRS[7], f.wrapping_neg()),
        (true, false) => (DIRS[5], f),
        (false, true) => (DIRS[1], f.wrapping_neg()),
        (false, false) => (DIRS[3], f),
    };
    m.w8(e + 0x3E, hd);
    m.w32(e + 0x38, vy);
    m.w32(e + 0x34, if left { g.wrapping_neg() } else { g });
}

/// `0x00565D70(ecx = máquina, x, y)`: punto `+0x3D` de la ruta (medio de los dos carriles) y rumbo hacia él.
pub fn set_anchor(m: &mut dyn Mem, e: u32, a: u16, b: u16) {
    let k = u32::from(m.r8(e + 0x44));
    let i = m.r8(e + 0x3D);
    let j = lane_len8(m, 0xC678, k).wrapping_sub(i).wrapping_sub(1);
    let (ax, ay) = lane_point(m, 0xC674, k, u32::from(i));
    let (bx, by) = lane_point(m, 0xC678, k, u32::from(j));
    m.w16(e + 0x40, ((ax.wrapping_add(bx) as i32) >> 1) as u16);
    m.w16(e + 0x42, (by.wrapping_add(ay).wrapping_add(0x16) as i32 >> 1) as u16);
    heading_velocity(m, e, a, b);
}

/// `0x005661D0(columna, fila)`: casilla de suelo `0x28` sin pieza alta salvo los muros `0x30/0x31`.
fn tile_ok(m: &mut dyn Mem, h: &mut dyn FigureHost, col: u32, row: u32) -> bool {
    let v = view(m);
    let w = m.r32(v + 0xC314);
    if col >= w || row >= m.r32(v + 0xC318) {
        return false;
    }
    let idx = w.wrapping_mul(row).wrapping_add(col);
    let objs = m.r32(v + 0x98);
    let ua = m.r32(v + 0xC30C).wrapping_add(idx * 4);
    let up = m.r16(ua);
    if up != 0 {
        let o = m.r32(objs.wrapping_add(4 * u32::from(up)));
        let k = h.tile_kind(m, o);
        if !(0x30..=0x31).contains(&k) {
            return false;
        }
    }
    let ga = m.r32(v + 0xC308).wrapping_add(idx * 4);
    let g = m.r16(ga);
    let o = m.r32(objs.wrapping_add(4 * u32::from(g)));
    h.tile_kind(m, o) == 0x28
}

/// `0x00565F50(ecx = máquina, columna, fila, rumbo)`: ¿cabe la huella (3×4 o 4×3 casillas sin marcas y con
/// suelo de paso) a dos pasos en el rumbo? Guarda la casilla delantera en `+0x46/+0x47`.
pub fn footprint_free(m: &mut dyn Mem, h: &mut dyn FigureHost, e: u32, col: u32, row: u32, dir: u8) -> bool {
    let (mut c, mut r) = (col, row);
    h.step(m, &mut c, &mut r, dir);
    h.step(m, &mut c, &mut r, dir);
    m.w8(e + 0x46, c as u8);
    m.w8(e + 0x47, r as u8);
    match dir {
        0x20 => {
            h.step(m, &mut c, &mut r, DIRS[3]);
        }
        0x60 | 0xA0 => {
            h.step(m, &mut c, &mut r, DIRS[4]);
        }
        0xE0 => {
            h.step(m, &mut c, &mut r, DIRS[5]);
        }
        _ => {}
    }
    let (inner, outer) = if dir & 0x40 != 0 { (3u8, 4u8) } else { (4, 3) };
    let s = scene_of(m, e);
    for _ in 0..outer {
        let (mut cc, mut rr) = (c, r);
        for _ in 0..inner {
            if mark_at_least(m, s, cc, rr, 1) || !tile_ok(m, h, cc, rr) {
                return false;
            }
            h.step(m, &mut cc, &mut rr, DIRS[7]);
        }
        h.step(m, &mut c, &mut r, DIRS[1]);
    }
    true
}

/// Distancia al cuadrado con el eje vertical a `34/22` (`(34·dy/22)² + dx²`, aritmética de 32 bits).
fn d2(dx: i32, dy: i32) -> (i32, i32) {
    let q = dy.wrapping_mul(34) / 22;
    (q.wrapping_mul(q).wrapping_add(dx.wrapping_mul(dx)), q)
}

/// Ajuste hacia el borde del edificio o del destino: 17 px en el eje dominante horizontal, 22 abajo en el vertical.
fn adjust(dx: i32, dy: i32) -> (i32, i32) {
    if dx.wrapping_abs() > dy.wrapping_abs() {
        if dx < 0 {
            (dx + 0x11, dy - 0x11)
        } else if dx > 0 {
            (dx - 0x11, dy - 0x11)
        } else {
            (dx, dy - 0x11)
        }
    } else if dy > 0xB {
        (dx, dy - 0x16)
    } else {
        (dx, dy)
    }
}

/// Edificio atacado por el asalto: `escena+0x250 + 12 · escena+0x6C`, campo `+8`.
fn target_building(m: &mut dyn Mem, h: &mut dyn FigureHost, e: u32, s: u32) -> (i32, i32) {
    let i = u32::from(m.r8(s + 0x6C));
    let ia = m.r32(s + 0x250).wrapping_add(i * 12 + 8);
    let id = m.r32(ia);
    let city = u32::from(m.r8(e + 0x45));
    let town = m.r32(TOWNS).wrapping_add(city * 0x9F8);
    let b = building_record(m, town, id);
    let (bc, br) = (u32::from(m.r8(b)), u32::from(m.r8(b.wrapping_add(1))));
    h.cell_to_pixel(m, bc, br)
}

fn fighter_xy(m: &mut dyn Mem, f: u32) -> (i32, i32) {
    let p = if f != 0 { f + 8 } else { 0 };
    (s16(m, p + 6), s16(m, p + 0xA))
}

fn isqrt(v: i32) -> u32 {
    if v < 0 {
        return 0x8000_0000;
    }
    f64::from(v).sqrt() as i32 as u32
}

/// `0x005669D0(ecx = máquina, x, y)`: distancia (eje vertical a `34/22`) del píxel al objetivo más cercano: el
/// edificio atacado, si no los defensores de las tres listas o, si no hay, los cabecillas. `0x7FFFFFFF` si el
/// píxel o las casillas de detrás tienen pieza alta.
pub fn nearest_distance(m: &mut dyn Mem, h: &mut dyn FigureHost, e: u32, x: u32, y: u32) -> u32 {
    let s = scene_of(m, e);
    let (px, py) = ((x & 0xFFFF) as i32, (y & 0xFFFF) as i32);
    let (c, r) = h.pixel_to_cell(m, px, py);
    let v = view(m);
    let w = m.r32(v + 0xC314);
    let ht = m.r32(v + 0xC318);
    let up = m.r32(v + 0xC30C);
    let free = |m: &mut dyn Mem, c: u32, r: u32| c < w && r < ht && m.r16(up.wrapping_add(w.wrapping_mul(r).wrapping_add(c).wrapping_mul(4))) == 0;
    if !free(m, c, r) || !free(m, c, r.wrapping_sub(2)) {
        return MAX as u32;
    }
    let c1 = if r & 1 == 0 { c.wrapping_sub(1) } else { c };
    if !free(m, c1, r.wrapping_sub(1)) || !free(m, c1.wrapping_add(1), r.wrapping_sub(1)) {
        return MAX as u32;
    }
    let mut best = MAX;
    if m.r16(s + 0x242) > 0 {
        let (qx, qy) = target_building(m, h, e, s);
        let (dx, dy) = adjust(qx.wrapping_sub(px), qy.wrapping_sub(py));
        let (d, _) = d2(dx, dy);
        if d < MAX {
            return isqrt(d);
        }
    }
    let city = m.r8(e + 0x45);
    let rec = siege_of(m, city);
    for (k, list) in [0x410u32, 0x80C, 0xC08].into_iter().enumerate() {
        let n = m.r16(rec + 0x1006 + 2 * k as u32);
        let mut i = 0u8;
        while u16::from(i) < n {
            let f = m.r32(rec + list + 4 * u32::from(i));
            let (fx, fy) = fighter_xy(m, f);
            let (d, _) = d2(fx.wrapping_sub(px), fy.wrapping_sub(py));
            if d < best {
                best = d;
            }
            i = i.wrapping_add(1);
        }
    }
    if best == MAX {
        let n = m.r16(rec + 0x1004);
        let mut i = 0u8;
        while u16::from(i) < n {
            let f = m.r32(rec + 0x14 + 4 * u32::from(i));
            let (fx, fy) = fighter_xy(m, f);
            let (d, _) = d2(fx.wrapping_sub(px), fy.wrapping_sub(py));
            if d < best {
                best = d;
            }
            i = i.wrapping_add(1);
        }
    }
    isqrt(best)
}

/// Rumbo de la rosa de ocho hacia `(dx, dy)` (umbral `tan 22,5° ≈ 414/1000`).
fn aim_heading(dx: i32, dy: i32) -> u8 {
    let (ax, ay) = (dx.wrapping_abs(), dy.wrapping_abs());
    if ax.wrapping_mul(414) > ay.wrapping_mul(1000) {
        if dx > 0 { DIRS[2] } else { DIRS[6] }
    } else if ay.wrapping_mul(414) > ax.wrapping_mul(1000) {
        if dy > 0 { DIRS[4] } else { DIRS[0] }
    } else if dy > 0 {
        if dx > 0 { DIRS[3] } else { DIRS[5] }
    } else if dx > 0 {
        DIRS[1]
    } else {
        DIRS[7]
    }
}

/// `0x005663D0(ecx = máquina, conjunto)`: elige el objetivo más cercano (edificio atacado; defensores de pie de
/// las tres listas; con `escena+0x78` los cabecillas; si no, el destino del asalto), guarda su píxel en
/// `+0x66/+0x68`, el objetivo en `+0x58` y su tipo en `+0x5C`, gira hacia él y empieza la animación de tiro
/// (`0xF`). Sin objetivo vuelve a andar hacia su punto (animación 2). Devuelve el fotograma inicial.
pub fn acquire_target(m: &mut dyn Mem, h: &mut dyn FigureHost, e: u32, set: u32) -> u8 {
    let s = scene_of(m, e);
    let (ex, ey) = (s16(m, e + 0xE), s16(m, e + 0x12));
    let mut best = MAX;
    let mut target = 0u32;
    let mut kind = 0u8;
    let mut aim: Option<(i32, i32)> = None;
    if m.r16(s + 0x242) > 0 {
        let (qx, qy) = target_building(m, h, e, s);
        let (dx, dy) = adjust(qx.wrapping_sub(ex), qy.wrapping_sub(ey));
        let (d, q) = d2(dx, dy);
        if d < MAX {
            m.w16(e + 0x66, qx as u16);
            m.w16(e + 0x68, qy as u16);
            best = d;
            target = e;
            kind = 4;
            aim = Some((dx, q));
        }
    }
    if aim.is_none() {
        let city = m.r8(e + 0x45);
        let rec = siege_of(m, city);
        for (k, list) in [0x410u32, 0x80C, 0xC08].into_iter().enumerate() {
            let n = m.r16(rec + 0x1006 + 2 * k as u32);
            let mut i = 0u8;
            while u16::from(i) < n {
                let f = m.r32(rec + list + 4 * u32::from(i));
                if m.r8(f + 0x58) == 0 {
                    let fx = m.r16(f + 0xE);
                    let fy = s16(m, f + 0x12);
                    let (d, _) = d2(i32::from(fx as i16).wrapping_sub(ex), fy.wrapping_sub(ey));
                    if d < best {
                        best = d;
                        m.w16(e + 0x66, fx);
                        target = f;
                        kind = k as u8 + 1;
                        let y = m.r16(f + 0x12);
                        m.w16(e + 0x68, y);
                    }
                }
                i = i.wrapping_add(1);
            }
        }
        if best == MAX && m.r8(s + 0x78) != 0 {
            let n = m.r16(rec + 0x1004);
            let mut i = 0u8;
            while u16::from(i) < n {
                let f = m.r32(rec + 0x14 + 4 * u32::from(i));
                if m.r8(f + 0x58) == 0 {
                    let fx = m.r16(f + 0xE);
                    let fy = s16(m, f + 0x12);
                    let (d, _) = d2(i32::from(fx as i16).wrapping_sub(ex), fy.wrapping_sub(ey));
                    if d < best {
                        best = d;
                        m.w16(e + 0x66, fx);
                        target = f;
                        kind = 0;
                        let y = m.r16(f + 0x12);
                        m.w16(e + 0x68, y);
                    }
                }
                i = i.wrapping_add(1);
            }
        }
        if best == MAX {
            if m.r8(s + 0x78) == 0 {
                let (tc, tr) = (u32::from(m.r16(s + 8)), u32::from(m.r16(s + 0xA)));
                let (qx, qy) = h.cell_to_pixel(m, tc, tr);
                let (dx, dy) = adjust(qx.wrapping_sub(ex), qy.wrapping_sub(ey));
                let (d, q) = d2(dx, dy);
                if d < MAX {
                    best = d;
                    target = e;
                    kind = 4;
                    m.w16(e + 0x66, qx as u16);
                    m.w16(e + 0x68, qy as u16);
                    aim = Some((dx, q));
                }
            }
            if best == MAX {
                // El original vuelve a pedir el píxel del destino; sin objetivo el rumbo no se usa.
                let (tc, tr) = (u32::from(m.r16(s + 8)), u32::from(m.r16(s + 0xA)));
                h.cell_to_pixel(m, tc, tr);
            }
        } else {
            let (fx, fy) = fighter_xy(m, target);
            let (_, q) = d2(0, fy.wrapping_sub(ey));
            aim = Some((fx.wrapping_sub(ex), q));
        }
    }
    if best == MAX {
        let (a, b) = (m.r16(e + 0xE), m.r16(e + 0x12));
        heading_velocity(m, e, a, b);
        return anim_select(m, e + 0x20, set, 2);
    }
    let (dx, dy) = aim.unwrap_or((0, 0));
    m.w32(e + 0x58, target);
    m.w8(e + 0x5C, kind.wrapping_add(0x80));
    m.w8(e + 0x3E, aim_heading(dx, dy));
    anim_select(m, e + 0x20, set, 0xF)
}

/// `0x00566E30(ecx = máquina, fotograma)` (método `+0x50`): ráfagas (el fotograma de tiro se repite `0x006E1740`
/// veces, con sonido al acabar) y en el fotograma 0x11 lanza el proyectil (clase `0x17`) a la casilla del objetivo.
pub fn on_frame(m: &mut dyn Mem, h: &mut dyn FigureHost, e: u32, frame: u8) -> u8 {
    let k = u32::from(m.r8(e + 0x31));
    let fa = m.r32(FIRE_FRAMES).wrapping_add(k);
    let fire = m.r8(fa);
    if u32::from(frame) == u32::from(fire) + 1 {
        let left = m.r8(e + 0x64);
        if left == 0 {
            let ba = m.r32(BURSTS).wrapping_add(k);
            let n = m.r8(ba);
            m.w8(e + 0x64, n);
            return frame.wrapping_sub(1);
        }
        m.w8(e + 0x64, left - 1);
        if left - 1 != 0 {
            return frame.wrapping_sub(1);
        }
        if h.sound_list_size(m, 0xEE66) != 0 {
            let n = h.sound_list_size(m, 0xEE66) as i32;
            let r = (h.rand(m) as i32).checked_rem(n).unwrap_or(0);
            let (x, y) = (s16(m, e + 0xE), s16(m, e + 0x12));
            h.play_sound(m, x, y, r.wrapping_sub(0x119A) as u32);
        }
    }
    if frame == 0x11 {
        let (tx, ty) = (u32::from(m.r16(e + 0x66)) as i32, u32::from(m.r16(e + 0x68)) as i32);
        let (c, r) = h.pixel_to_cell(m, tx, ty);
        let mut v = ((c << 8).wrapping_add(r)) << 16;
        match m.r8(e + 0x5C).wrapping_sub(0x80) {
            0 => v = v.wrapping_add(0x6004),
            1..=3 => v = v.wrapping_add(0x603E),
            4 => {
                let s = scene_of(m, e);
                v = v.wrapping_add(if m.r16(s + 0x242) == 0 { 0x6004 } else { 0x603E });
            }
            _ => {}
        }
        let (x, y) = (m.r32(e + 0xC), m.r32(e + 0x10));
        let p = h.spawn_entity(m, [x, y, 0x17, v, 3, 0, 0]);
        let city = m.r8(e + 0x45);
        if p != 0 {
            m.w8(p + 0x2C, city);
        }
    }
    frame
}

/// `0x00566260(ecx = estado de animación, conjunto)` (método `+8`): al acabar el tiro (`0xF`) sigue disparando si
/// el asalto no avanza y está cerca del destino (o en posición), o vuelve a andar; al acabar la marcha en la ida,
/// cerca del destino, empieza a tirar.
pub fn unit_anim_finished(m: &mut dyn Mem, h: &mut dyn FigureHost, anim: u32, set: u32) {
    let e = anim - 0x20;
    let city = m.r8(e + 0x45);
    let rec = siege_of(m, city);
    let s = m.r32(rec + 8);
    if m.r8(anim + 8) == 0xF {
        if siege_active(m, rec) && m.r16(s + 0x3A) == 0 && m.r16(s + 0x3C) == 0 {
            let (ex, ey) = (s16(m, e + 0xE), s16(m, e + 0x12));
        let (c, r) = h.pixel_to_cell(m, ex, ey);
            if m.r8(e + 0x3C) != 2 && distance_to_target(m, s, c, r) as u16 > 0xF {
                anim_set_default(m, anim, 2);
                let (a, b) = (m.r16(e + 0xE), m.r16(e + 0x12));
                heading_velocity(m, e, a, b);
            } else {
                anim_set_default(m, anim, 0xF);
                acquire_target(m, h, e, set);
            }
        } else {
            anim_set_default(m, anim, 2);
            let st = m.r8(e + 0x3C);
            if st < 3 {
                let st = st + 3;
                m.w8(e + 0x3C, st);
                let (a, b) = (m.r16(e + 0xE), m.r16(e + 0x12));
                if st == 3 {
                    let i = m.r8(e + 0x3D);
                    m.w8(e + 0x3D, i.wrapping_sub(1));
                    set_anchor(m, e, a, b);
                } else {
                    if st == 5 {
                        m.w8(e + 0x3C, 4);
                    }
                    let (x, y) = (m.r16(e + 0x48), m.r16(e + 0x4A));
                    m.w16(e + 0x40, x);
                    m.w16(e + 0x42, y);
                    heading_velocity(m, e, a, b);
                }
                m.w16(e + 0x4C, 0);
            }
        }
    }
    if m.r8(anim + 8) == 2 && m.r8(e + 0x3C) == 0 {
        let (ex, ey) = (s16(m, e + 0xE), s16(m, e + 0x12));
        let (c, r) = h.pixel_to_cell(m, ex, ey);
        if (distance_to_target(m, s, c, r) as u16) < 0xF {
            anim_set_default(m, anim, 0xF);
            acquire_target(m, h, e, set);
        }
    }
}

fn mark_four(m: &mut dyn Mem, h: &mut dyn FigureHost, s: u32, mut c: u32, mut r: u32) {
    set_mark(m, s, c, r, 8);
    for d in [DIRS[7], DIRS[2], DIRS[7]] {
        h.step(m, &mut c, &mut r, d);
        set_mark(m, s, c, r, 8);
    }
}

fn clear_stored(m: &mut dyn Mem, h: &mut dyn FigureHost, e: u32, s: u32) {
    let (c, r) = (m.r32(e + 0x50), m.r32(e + 0x54));
    clear_mark(m, s, c, r, 8);
    for d in [DIRS[7], DIRS[2], DIRS[7]] {
        let (mut c, mut r) = (m.r32(e + 0x50), m.r32(e + 0x54));
        h.step(m, &mut c, &mut r, d);
        m.w32(e + 0x50, c);
        m.w32(e + 0x54, r);
        clear_mark(m, s, c, r, 8);
    }
}

/// `0x00564810(ecx = máquina, x, y, rumbo, enlace, variante)` (método `+0x40`): como el ariete, con la ciudad en
/// `variante & 0xFF`, la ruta en `variante >> 8` y alta con repintado.
pub fn unit_init(m: &mut dyn Mem, h: &mut dyn FigureHost, e: u32, a: [u32; 5]) {
    let [_, _, _, id, variant] = a;
    let n = m.r32(SETS);
    let r = h.rand(m);
    let pick = r.checked_rem(n).unwrap_or(0);
    m.w8(e + 0x45, variant as u8);
    m.w8(e + 0x44, (variant >> 8) as u8);
    m.w8(e + 0x31, pick as u8);
    let set = unit_set(m, e);
    let f = anim_select(m, e + 0x20, set, 2);
    m.w8(e + 0x15, f);
    m.w32(e + 0x24, 0);
    super::spawn::links_alloc(m, h, e, set);
    m.w16(e + 0x16, id as u16);
    let k = u32::from(m.r8(e + 0x44));
    let j = u32::from(lane_len8(m, 0xC678, k).wrapping_sub(1));
    let (ax, ay) = lane_point(m, 0xC674, k, 0);
    let (bx, by) = lane_point(m, 0xC678, k, j);
    m.w32(e + 0xC, ax.wrapping_add(bx) << 15);
    m.w8(e + 0x3C, 0);
    m.w8(e + 0x3D, 1);
    m.w32(e + 0x10, by.wrapping_add(ay).wrapping_add(0x16) << 15);
    let (x, y) = (m.r16(e + 0xE), m.r16(e + 0x12));
    set_anchor(m, e, x, y);
    let hd = m.r8(e + 0x3E);
    m.w8(e + 0x14, hd);
    let (ra, rb) = frame_rects(m, set, e + 8);
    clip(m, h, &ra);
    if let Some(b) = rb {
        clip(m, h, &b);
    }
    place(m, h, set, e + 8, None);
    let (px, py) = (s16(m, e + 0xE), s16(m, e + 0x12));
    let (c, r) = h.pixel_to_cell(m, px, py);
    m.w32(e + 0x50, c);
    m.w32(e + 0x54, r);
    let s = scene_of(m, e);
    mark_four(m, h, s, c, r);
    let period = 0x3_AB20u32.checked_div(m.r32(set + 0xC)).unwrap_or(0);
    m.w16(e + 0x4C, 0);
    m.w16(e + 0x4E, period as u16);
    m.w32(e + 0x60, 0);
    m.w8(e + 0x6A, 0);
    m.w16(e + 0x2E, 0xFFFF);
    m.w32(e + 0x6C, 0xFFFF_FFFF);
    m.w32(e + 0x70, 0);
    m.w32(e + 0x74, 0);
}

/// `0x00564A80` (método `+0x44`): borra la máquina de la vista.
pub fn unit_remove(m: &mut dyn Mem, h: &mut dyn FigureHost, e: u32) {
    let set = unit_set(m, e);
    erase(m, h, false, set, e + 8);
}

fn release_fire(m: &mut dyn Mem, h: &mut dyn FigureHost, e: u32) {
    let fx = m.r32(e + 0x74);
    if fx != 0 {
        h.release_entity(m, fx);
        m.w32(e + 0x74, 0);
    }
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

fn absorb_push(m: &mut dyn Mem, e: u32, rem: &mut u32) {
    let a = m.r16(e + 0x4C);
    let c = m.r16(e + 0x4E);
    let d = m.r32(e + 0x60);
    if a >= c || d == 0 {
        return;
    }
    let (au, cu) = (u32::from(a), u32::from(c));
    if au.wrapping_add(d) > cu {
        if a == 0 {
            let half = cu.wrapping_sub(*rem) >> 1;
            *rem = rem.wrapping_add(half);
            m.w16(e + 0x4C, a.wrapping_add(*rem as u16));
            m.w32(e + 0x60, d.wrapping_sub(half));
        } else {
            m.w32(e + 0x60, d.wrapping_sub(cu).wrapping_add(au));
            *rem = rem.wrapping_add(cu.wrapping_sub(au));
            m.w16(e + 0x4C, c);
        }
    } else {
        *rem = rem.wrapping_add(d);
        let d16 = m.r16(e + 0x60);
        let a = m.r16(e + 0x4C);
        m.w16(e + 0x4C, a.wrapping_add(d16));
        m.w32(e + 0x60, 0);
    }
}

/// `0x00564AC0` (método `+0x14`): paso por fotograma de la máquina. Devuelve 0 cuando desaparece.
pub fn unit_update(m: &mut dyn Mem, h: &mut dyn FigureHost, e: u32) -> u8 {
    let set = unit_set(m, e);
    let s = scene_of(m, e);
    if s == 0 {
        return 0;
    }
    let mut x = m.r32(e + 0xC);
    let mut y = m.r32(e + 0x10);
    let mut head = m.r8(e + 0x14);
    let mut rem = { let v = view(m); m.r32(v + 0x414) }.min(500);
    let old30 = m.r8(e + 0x30);
    let mut frame = if m.r8(e + 0x3C) == 2 && m.r8(e + 0x28) == 2 { m.r8(e + 0x15) } else { vcall(m, h, e + 0x20, 4, &[set, e + 8, e + 0x30]) as u8 };
    if frame != m.r8(e + 0x15) {
        frame = vcall(m, h, e, 0x50, &[u32::from(frame)]) as u8;
    }
    if m.r8(e + 0x2C) != 0 {
        let code = m.r8(e + 0x28);
        let f = match code {
            0x91 => frame,
            0xB1 => {
                let (ex, ey, hd) = (m.r32(e + 0xC), m.r32(e + 0x10), m.r8(e + 0x14));
                let fx = h.spawn_entity(m, [ex, ey, 0x12, 0xC1, u32::from(hd), 0, 0]);
                h.set_fade(m, fx, 2000, 0xFFFF_FF80);
                clear_stored(m, h, e, s);
                return 0;
            }
            _ => {
                for _ in 0..5 {
                    let r = h.rand(m);
                    let (ex, ey) = (m.r32(e + 0xC), m.r32(e + 0x10));
                    h.spawn_entity(m, [ex, ey, 0x13, 0, r, 0, 0]);
                }
                sound(m, h, e, 0xEA7E, 0x1582);
                release_fire(m, h, e);
                let (ex, ey) = (m.r32(e + 0xC), m.r32(e + 0x10));
                h.spawn_entity(m, [ex, ey, 0x12, 0xB_0023, u32::from(DIRS[1]), 0, 0]);
                anim_select(m, e + 0x20, set, 0x91)
            }
        };
        if f != m.r8(e + 0x15) {
            let rect = erase(m, h, true, set, e + 8);
            m.w8(e + 0x15, f);
            place(m, h, set, e + 8, rect);
        }
        return 1;
    }
    let mut n = 1u8;
    if m.r32(e + 0x60) != 0 {
        n = 0xFF;
        m.w8(e + 0x6A, 1);
    }
    let mut i = 0u8;
    loop {
        if i != 0 {
            rem = 0;
        }
        let (sx, sy) = (x, y);
        let (px, py) = (x >> 16, y >> 16);
        let (c0, r0) = h.pixel_to_cell(m, px as i32, py as i32);
        if m.r8(e + 0x3C) < 3 && (m.r16(s + 0x3A) != 0 || m.r16(s + 0x3C) != 0) && m.r32(e + 0x60) == 0 {
            anim_set_default(m, e + 0x20, 2);
            let st = m.r8(e + 0x3C).wrapping_add(3);
            m.w8(e + 0x3C, st);
            if st == 3 {
                let k = m.r8(e + 0x3D);
                m.w8(e + 0x3D, k.wrapping_sub(1));
                set_anchor(m, e, px as u16, py as u16);
            } else {
                if st == 5 {
                    m.w8(e + 0x3C, 4);
                }
                let (ax, ay) = (m.r16(e + 0x48), m.r16(e + 0x4A));
                m.w16(e + 0x40, ax);
                m.w16(e + 0x42, ay);
                heading_velocity(m, e, px as u16, py as u16);
            }
            let a = m.r16(e + 0x4C);
            if a > 0 {
                let c = m.r16(e + 0x4E);
                m.w16(e + 0x4C, c.wrapping_sub(a));
            }
        }
        let tx = u32::from(m.r16(e + 0x40)) as i32;
        let vx = m.r32(e + 0x34) as i32;
        let ty = u32::from(m.r16(e + 0x42)) as i32;
        let vy = m.r32(e + 0x38) as i32;
        let (pxi, pyi) = (px as i32, py as i32);
        let reached = (pxi <= tx && vx < 0) || (pxi >= tx && vx > 0) || (pyi <= ty && vy < 0) || (pyi >= ty && vy > 0);
        if reached {
            match m.r8(e + 0x3C) {
                0 => {
                    let k = u32::from(m.r8(e + 0x44));
                    let len = { let v = view(m); let t = m.r32(v + 0xC674); m.r32(t.wrapping_add(k * 8)) }.wrapping_sub(1);
                    let p = m.r8(e + 0x3D);
                    if u32::from(p) != len {
                        m.w8(e + 0x3D, p.wrapping_add(1));
                    }
                }
                st @ (1 | 2) => {
                    if st == 1 {
                        m.w32(e + 0x60, 0);
                        m.w8(e + 0x3C, 2);
                    }
                    if m.r16(s + 0x3A) == 0 && m.r16(s + 0x3C) == 0 && m.r8(e + 0x28) != 0xF {
                        frame = acquire_target(m, h, e, set);
                    }
                }
                st @ (3 | 4) => {
                    if st == 4 {
                        m.w8(e + 0x3C, 3);
                    }
                    let p = m.r8(e + 0x3D);
                    if p == 0 {
                        clear_stored(m, h, e, s);
                        let left = m.r16(s + 0x22E);
                        if left != 0 {
                            m.w16(s + 0x22E, left - 1);
                        }
                        release_fire(m, h, e);
                        return 0;
                    }
                    m.w8(e + 0x3D, p - 1);
                }
                _ => {}
            }
            let a = m.r16(e + 0x4C);
            m.w16(e + 0x4C, a.wrapping_add(rem as u16));
            x = u32::from(m.r16(e + 0x40)) << 16;
            y = u32::from(m.r16(e + 0x42)) << 16;
            let c = m.r16(e + 0x4E);
            absorb_push(m, e, &mut rem);
            let a = m.r16(e + 0x4C);
            if a >= c {
                rem = rem.wrapping_add(u32::from(c)).wrapping_sub(u32::from(a));
                m.w16(e + 0x4C, 0);
            }
            if m.r8(e + 0x3C) != 2 {
                set_anchor(m, e, (x >> 16) as u16, (y >> 16) as u16);
            }
        } else {
            let a = m.r16(e + 0x4C);
            m.w16(e + 0x4C, a.wrapping_add(rem as u16));
            let c = m.r16(e + 0x4E);
            absorb_push(m, e, &mut rem);
            let a = m.r16(e + 0x4C);
            if a >= c {
                rem = rem.wrapping_add(u32::from(c)).wrapping_sub(u32::from(a));
                m.w16(e + 0x4C, 0);
                if m.r8(e + 0x3C) == 0 {
                    let near = nearest_distance(m, h, e, px, py) <= 0x1A4 || (distance_to_target(m, s, c0, r0) as u16) < 0x14;
                    if near {
                        let hd = m.r8(e + 0x3E);
                        if footprint_free(m, h, e, c0, r0, hd.wrapping_add(0x40)) || footprint_free(m, h, e, c0, r0, hd.wrapping_sub(0x40)) {
                            let nx = m.r32(e + 0x34).wrapping_mul(rem).wrapping_add(x);
                            let ny = m.r32(e + 0x38).wrapping_mul(rem).wrapping_add(y);
                            m.w16(e + 0x4A, (ny >> 16) as u16);
                            m.w16(e + 0x48, (nx >> 16) as u16);
                            m.w8(e + 0x3C, 1);
                            let (cc, cr) = h.pixel_to_cell(m, (nx >> 16) as i32, (ny >> 16) as i32);
                            let (qx, qy) = h.cell_to_pixel(m, cc, cr);
                            m.w16(e + 0x48, qx as u16);
                            m.w16(e + 0x4A, qy as u16);
                            let (fc, fr) = (u32::from(m.r8(e + 0x46)), u32::from(m.r8(e + 0x47)));
                            let (qx, qy) = h.cell_to_pixel(m, fc, fr);
                            m.w16(e + 0x40, qx as u16);
                            m.w16(e + 0x42, qy as u16);
                            let (ax, ay) = (m.r16(e + 0x48), m.r16(e + 0x4A));
                            y = u32::from(ay) << 16;
                            x = u32::from(ax) << 16;
                            rem = 0;
                            heading_velocity(m, e, ax, ay);
                        }
                    }
                }
            }
            if m.r8(e + 0x3C) != 2 && m.r8(e + 0x28) == 2 {
                x = x.wrapping_add(m.r32(e + 0x34).wrapping_mul(rem));
                y = y.wrapping_add(m.r32(e + 0x38).wrapping_mul(rem));
            }
        }
        let (mut nc, mut nr) = h.pixel_to_cell(m, (x >> 16) as i32, (y >> 16) as i32);
        let (oc, or) = (m.r32(e + 0x50), m.r32(e + 0x54));
        if (nc != oc || nr != or) && m.r8(e + 0x28) == 2 {
            let mut accept = true;
            if m.r8(e + 0x3C) == 0 && (distance_to_target(m, s, oc, or) as u16) < 0xF {
                accept = false;
            }
            clear_stored(m, h, e, s);
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
            let (tx, ty) = (u32::from(m.r16(e + 0x40)) as i32, u32::from(m.r16(e + 0x42)) as i32);
            let (tc, tr) = h.pixel_to_cell(m, tx, ty);
            let mut forced = false;
            if nc != tc || nr != tr {
                let extra = match m.r8(e + 0x3E) {
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
            } else if m.r8(e + 0x3C) == 4 {
                forced = true;
            }
            if forced || accept {
                m.w32(e + 0x50, nc);
                m.w32(e + 0x54, nr);
            } else {
                m.w32(e + 0x50, oc);
                m.w32(e + 0x54, or);
                nc = oc;
                nr = or;
                let a = m.r16(e + 0x4C);
                let mut t = a.wrapping_sub(rem as u16);
                if rem >= u32::from(a) {
                    t = t.wrapping_add(m.r16(e + 0x4E));
                }
                x = sx;
                m.w16(e + 0x4C, t);
                y = sy;
                let d = nearest_distance(m, h, e, px, py);
                frame = if m.r16(s + 0x3A) == 0 && m.r16(s + 0x3C) == 0 && d <= 0x348 && d > 0 && m.r32(e + 0x60) == 0 {
                    acquire_target(m, h, e, set)
                } else {
                    m.r8(e + 0x15)
                };
                let t = m.r32(s + 0x574);
                if t == 0 || m.r32(t + 0x30) == 0 {
                    let lim = u32::from(m.r16(e + 0x4E)).wrapping_mul(u32::from(n));
                    if m.r32(e + 0x60) > lim {
                        m.w32(e + 0x60, lim);
                    }
                }
            }
            mark_four(m, h, s, nc, nr);
        }
        let d60 = m.r32(e + 0x60);
        if d60 == 0 && m.r8(e + 0x6A) == 0 {
            let to = m.r8(e + 0x3E);
            if (old30 != m.r8(e + 0x30) || m.r8(e + 0x28) == 0xF) && to != head {
                head = if to.wrapping_sub(head) < 0x80 { head.wrapping_add(0x20) } else { head.wrapping_sub(0x20) };
            }
        } else {
            head = m.r8(e + 0x3E);
            if n == 1 {
                m.w8(e + 0x6A, 0);
            }
        }
        let k = if d60 == 0 { n.wrapping_sub(1) } else { i };
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
        let next = m.r32(e + 0x6C);
        if m.r32(CLOCK) > next {
            m.w32(e + 0x70, next);
            let ry = h.rand(m) as i32;
            let fy = (((ry % 0xCF0) - 0x678) << 8) as u32;
            let fy = fy.wrapping_add(m.r32(e + 0x10));
            let rx = h.rand(m) as i32;
            let fx = (((rx % 0x1400) - 0xA00) << 8) as u32;
            let fx = fx.wrapping_add(m.r32(e + 0xC));
            let smoke = h.spawn_entity(m, [fx, fy, 0x12, 0x16_0024, u32::from(DIRS[1]), 0, 0]);
            let t = (((dmg * 1200) as i32 >> 16) as u32).wrapping_add(m.r32(CLOCK)).wrapping_add(0xC8);
            m.w32(e + 0x6C, t);
            let alpha = (((dmg * 345) as i32 >> 16) as u8).wrapping_add(0x19);
            let (direction, strength) = h.wind(m);
            let dir = direction as u8;
            let speed = (strength & 0xFF) as i32;
            let cos = table_sine(dir.wrapping_add(0x40));
            let sin = table_sine(dir);
            if smoke != 0 {
                m.w8(smoke + 0x30, alpha);
                m.w32(smoke + 0x40, (speed.wrapping_mul(sin).wrapping_add(500) / 1000) as u32);
                m.w32(smoke + 0x44, ((speed.wrapping_mul(cos)).wrapping_neg().wrapping_add(500) / 1000) as u32);
            }
        } else {
            let t = (((dmg * 1200) as i32 >> 16) as u32).wrapping_add(m.r32(CLOCK)).wrapping_add(0xC8);
            if t < next {
                m.w32(e + 0x6C, t);
            }
        }
    }
    if dmg * 3 < 0x1_0000 {
        let w = 3u8.wrapping_sub(((dmg * 9) >> 16) as u8);
        let fx = m.r32(e + 0x74);
        if fx != 0 {
            let fset = super::effect::effect_set(m, fx);
            if u32::from(w) == m.r32(fset + 4) {
                let (ex, ey) = (s16(m, e + 0xE), s16(m, e + 0x12));
                vcall(m, h, fx, 0x48, &[ex as u32, ey.wrapping_sub(0x16) as u32, 0]);
            } else {
                h.release_entity(m, fx);
                m.w32(e + 0x74, 0);
            }
        }
        if m.r32(e + 0x74) == 0 {
            let ex = m.r32(e + 0xC);
            let fx = h.spawn_entity(m, [ex, ex, 0x12, (u32::from(w) << 8).wrapping_add(0x16_00B0), u32::from(DIRS[1]), 0, 0]);
            m.w32(e + 0x74, fx);
            return 1;
        }
    } else {
        release_fire(m, h, e);
    }
    1
}
