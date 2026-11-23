//! Proyectiles del asalto (clase `0x17`: flechas, virotes, balas y piedras): aparición `0x00576990` (`+0x40`),
//! vuelo por fotograma `0x00576E80` (`+0x14`), retirada `0x00576D20` (`+0x44`) y bajas por el impacto
//! `0x0062FB90` (`thiscall` sobre la escena del asalto).
//!
//! El proyectil guarda su tipo en `+0x36` (tablas `0x006E2E44` cuántos, `0x006E2E48` conjuntos y `0x006E2E4C`
//! registros de 12 bytes `{clase, altura base, variación}`), el avance 16.16 en `+0x30` sobre `+0x34` pasos, el
//! origen y el destino en píxeles (`+0x38/+0x3A`, `+0x3C/+0x3E`), la altura de salida y llegada (`+0x40`, `+0x41`),
//! la flecha de la parábola `+0x2E`, la altura dibujada `+0x2D` y la ciudad `+0x2C`. El estado de animación está
//! en `+0x20`. La altura estira el fotograma mientras se borra o se dibuja (`+6` alto, `+4` ancho, `+0xA` desfase).
use super::sprite::{anim_select, dir_index, erase, place, register_cells, clip, frame_rects};
use super::step::ground_kind;
use super::{s16, siege_of, view, FigureHost, DIRS};
use crate::flatmem::Mem;

/// Tablas de los tipos de proyectil.
pub const TYPES: u32 = 0x006E_2E44;

fn r16(m: &mut dyn Mem, a: u32) -> u32 {
    u32::from(m.r16(a))
}

fn inc16(m: &mut dyn Mem, a: u32) {
    let v = m.r16(a).wrapping_add(1);
    m.w16(a, v);
}

/// Conjunto de sprites del tipo `+0x36` (0 si no hay).
pub fn projectile_set(m: &mut dyn Mem, e: u32) -> u32 {
    let t = u32::from(m.r8(e + 0x36));
    if t < m.r32(TYPES) {
        let p = m.r32(TYPES + 4);
        m.r32(p.wrapping_add(t * 4))
    } else {
        0
    }
}

fn type_record(m: &mut dyn Mem, e: u32) -> u32 {
    let t = u32::from(m.r8(e + 0x36));
    m.r32(TYPES + 8).wrapping_add(t * 12)
}

/// Estira (`sign` 1) o devuelve (`-1`) el fotograma actual con la altura `+0x2D`.
fn stretch(m: &mut dyn Mem, e: u32, set: u32, sign: i32) {
    let heading = m.r8(e + 0x14);
    let d = dir_index(m, set, heading);
    let rec = |m: &mut dyn Mem| {
        let frames = m.r32(set + 0x18);
        let f = u32::from(m.r8(e + 0x15)) * 12;
        m.r32(frames.wrapping_add(d * 4)).wrapping_add(f)
    };
    let add = |m: &mut dyn Mem, a: u32, v: i32| {
        let w = m.r16(a);
        m.w16(a, (i32::from(w).wrapping_add(v)) as u16);
    };
    let hgt = i32::from(m.r8(e + 0x2D));
    let r = rec(m);
    add(m, r + 6, hgt * sign);
    let r = rec(m);
    add(m, r + 4, if sign > 0 { hgt / 5 } else { hgt / -5 });
    let r = rec(m);
    add(m, r + 0xA, -hgt * sign);
}

fn abs(v: i32) -> i32 {
    v.wrapping_abs()
}

/// `0x00576990(ecx = proyectil, x, y, clase, enlace, destino)` (`+0x40`): elige entre los tipos de la clase con
/// muestreo de depósito, calcula los pasos (distancia con el eje vertical a `17/11`), la flecha de la parábola y el
/// rumbo de dibujo, y da de alta el fotograma. `destino` lleva columna, fila y alturas de salida y llegada.
pub fn projectile_init(m: &mut dyn Mem, h: &mut dyn FigureHost, e: u32, x: u32, y: u32, class: u32, link: u32, target: u32) {
    m.w8(e + 0x36, 0);
    let want = class & 0xFF;
    let mut seen = 0u8;
    let mut b = 0u8;
    while u32::from(b) < m.r32(TYPES) {
        let recs = m.r32(TYPES + 8);
        if m.r32(recs.wrapping_add(u32::from(b) * 12)) == want {
            let pick = seen == 0 || (h.rand(m) as i32).wrapping_rem(i32::from(seen) + 1) == 0;
            if pick {
                m.w8(e + 0x36, b);
            }
            seen = seen.wrapping_add(1);
        }
        b = b.wrapping_add(1);
    }
    m.w16(e + 0x38, (x >> 16) as u16);
    m.w16(e + 0x3A, (y >> 16) as u16);
    let (px, py) = h.cell_to_pixel(m, target >> 24, (target >> 16) & 0xFF);
    m.w16(e + 0x3C, px as u16);
    m.w16(e + 0x3E, py as u16);
    let dx = (r16(m, e + 0x3C) as i32).wrapping_sub(r16(m, e + 0x38) as i32);
    let dy = (r16(m, e + 0x3E) as i32).wrapping_sub(r16(m, e + 0x3A) as i32).wrapping_mul(34) / 22;
    let d2 = dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx));
    let mut steps = (f64::from(d2).sqrt() as i64) as u16;
    if steps == 0 {
        steps = 1;
    }
    m.w16(e + 0x34, steps);
    m.w32(e + 0x30, 0);
    m.w8(e + 0x40, (target >> 8) as u8);
    m.w8(e + 0x41, target as u8);
    let rec = type_record(m, e);
    let r = h.rand(m);
    let span = m.r32(rec + 8);
    let base = m.r8(rec + 4);
    let arc = (r.checked_rem(span).unwrap_or(0) as u8).wrapping_add(base);
    m.w8(e + 0x2E, arc);
    let root = (f64::from(m.r16(e + 0x34)).sqrt() as i64) as i32;
    let arc = root.wrapping_mul(i32::from(m.r8(e + 0x2E))) >> 5;
    m.w8(e + 0x2E, arc as u8);
    let set = projectile_set(m, e);
    let f = anim_select(m, e + 0x20, set, 0);
    m.w8(e + 0x15, f);
    m.w32(e + 0x24, 0);
    let size = m.r32(set + 0x3C).wrapping_mul(4);
    let table = h.alloc(m, size);
    for i in 0..size {
        m.w8(table.wrapping_add(i), 0);
    }
    m.w32(e + 4, table);
    m.w16(e + 0x16, link as u16);
    m.w32(e + 0xC, x);
    m.w32(e + 0x10, y);
    let heading = if abs(dy) * 5 > abs(dx) * 12 {
        if dy > 0 { DIRS[4] } else { DIRS[0] }
    } else if abs(dx) * 5 > abs(dy) * 12 {
        if dx > 0 { DIRS[2] } else { DIRS[6] }
    } else if dy > 0 {
        if dx > 0 { DIRS[3] } else { DIRS[5] }
    } else if dx > 0 {
        DIRS[1]
    } else {
        DIRS[7]
    };
    m.w8(e + 0x14, heading);
    stretch(m, e, set, 1);
    register_cells(m, h, set, e + 8);
    stretch(m, e, set, -1);
}

/// `0x00576D20(ecx = proyectil)` (`+0x44`): repinta los rectángulos del fotograma estirado y lo borra de la vista.
pub fn projectile_remove(m: &mut dyn Mem, h: &mut dyn FigureHost, e: u32) {
    let set = projectile_set(m, e);
    stretch(m, e, set, 1);
    let (a, b) = frame_rects(m, set, e + 8);
    clip(m, h, &a);
    if let Some(b) = b {
        clip(m, h, &b);
    }
    erase(m, h, false, set, e + 8);
    stretch(m, e, set, -1);
}

fn impact_sound(m: &mut dyn Mem, h: &mut dyn FigureHost, x: i32, y: i32, list: u32, offset: i32) {
    if h.sound_list_size(m, list) == 0 {
        return;
    }
    let n = h.sound_list_size(m, list) as i32;
    let r = (h.rand(m) as i32).wrapping_rem(n);
    h.play_sound(m, x, y, r.wrapping_sub(offset) as u32);
}

/// `0x00576E80(ecx = proyectil)` (`+0x14`): avanza `vista+0x414 · conjunto+0xC · 4096 / 1000` por fotograma,
/// interpola la posición y la altura (parábola de flecha `+0x2E` sobre la recta `+0x40 → +0x41`) y elige el
/// fotograma (subiendo 0, en lo alto 1, bajando 2; nunca vuelve atrás). Al llegar, sobre agua (`0xE`, `0x13`)
/// salpica y en tierra levanta polvo (efecto `0x12`, solo los tipos de clase 3, con sonido), y abate a la figura
/// alcanzada (`0x0062FB90`). Devuelve 1 mientras vuela y 0 al llegar.
pub fn projectile_update(m: &mut dyn Mem, h: &mut dyn FigureHost, e: u32) -> u8 {
    let set = projectile_set(m, e);
    let v = view(m);
    let k = m.r32(v + 0x414).wrapping_mul(m.r32(set + 0xC)) << 12;
    let inc = ((u64::from(k) * 0xD1B7_1759) >> 41) as u32;
    let progress = m.r32(e + 0x30).wrapping_add(inc);
    m.w32(e + 0x30, progress);
    let total = r16(m, e + 0x34);
    if progress > total << 16 {
        m.w32(e + 0x30, total << 16);
    }
    let step = r16(m, e + 0x32) as i32;
    let div = |a: i32| a.checked_div(total as i32).unwrap_or(0);
    let lerp = |m: &mut dyn Mem, from: u32, to: u32| {
        let a = r16(m, e + from) as i32;
        let b = r16(m, e + to) as i32;
        (div(b.wrapping_sub(a).wrapping_mul(step)).wrapping_add(a) as u32) << 16
    };
    let x = lerp(m, 0x38, 0x3C);
    let y = lerp(m, 0x3A, 0x3E);
    let t = m.r32(e + 0x30).checked_div(total << 8).unwrap_or(0);
    let hb = i32::from(m.r8(e + 0x2E));
    let d = if t > 0x80 { t - 0x80 } else { 0x80 - t } as i32;
    let sag = hb.wrapping_mul(d).wrapping_mul(d) >> 14;
    let arc = (hb as u8).wrapping_sub(sag as u8);
    let h40 = m.r8(e + 0x40);
    let lin = div((i32::from(m.r8(e + 0x41)) - i32::from(h40)).wrapping_mul(step)) as u8;
    let hgt = arc.wrapping_add(lin.wrapping_add(h40));
    let anims = m.r32(set + 0x20);
    let frame = if m.r32(anims) > 1 {
        let old = m.r8(e + 0x2D);
        let f = if hgt > old { 0 } else if hgt == old { 1 } else { 2 };
        f.max(m.r8(e + 0x15))
    } else {
        0
    };
    if (x & 0xFFFF) as i32 == s16(m, e + 0xE) && s16(m, e + 0x12) == (y & 0xFFFF) as i32 && frame == m.r8(e + 0x15) && hgt == m.r8(e + 0x2D) {
        return 1;
    }
    if m.r16(e + 0x32) != m.r16(e + 0x34) {
        stretch(m, e, set, 1);
        let rect = erase(m, h, true, set, e + 8);
        stretch(m, e, set, -1);
        m.w32(e + 0xC, x);
        m.w32(e + 0x10, y);
        m.w8(e + 0x15, frame);
        m.w8(e + 0x2D, hgt);
        stretch(m, e, set, 1);
        place(m, h, set, e + 8, rect);
        stretch(m, e, set, -1);
        return 1;
    }
    let (px, py) = ((x >> 16) as i32, (y >> 16) as i32);
    let (col, row) = h.pixel_to_cell(m, px, py);
    if col >= m.r32(v + 0xC314) || row >= m.r32(v + 0xC318) {
        return 0;
    }
    let idx = m.r32(v + 0xC314).wrapping_mul(row).wrapping_add(col);
    let kind = ground_kind(m, h, idx);
    let rec = type_record(m, e);
    if m.r32(rec) == 3 {
        if kind == 0xE || kind == 0x13 {
            impact_sound(m, h, px, py, 0xEA92, 0x156E);
            h.spawn_entity(m, [x, y, 0x12, 0, u32::from(DIRS[1]), 0, 0]);
        } else {
            impact_sound(m, h, px, py, 0xEE98, 0x1168);
            let id = (u32::from(m.r8(e + 0x41)) << 16).wrapping_add(0x20);
            h.spawn_entity(m, [x, y, 0x12, id, u32::from(DIRS[1]), 0, 0]);
        }
    }
    let city = m.r8(e + 0x2C);
    let rec = siege_of(m, city);
    let s = m.r32(rec + 8);
    if s != 0 {
        let (tx, ty) = (r16(m, e + 0x3C), r16(m, e + 0x3E));
        defeat_near(m, s, tx, ty);
    }
    0
}

/// Distancia de la figura `e` al píxel `(x, y)` con el eje vertical a `17/11`.
fn distance(m: &mut dyn Mem, e: u32, x: i32, y: i32) -> i32 {
    let p = if e != 0 { e + 8 } else { 0 };
    let dx = abs(s16(m, p + 6).wrapping_sub(x));
    let dy = abs(s16(m, p + 0xA).wrapping_sub(y));
    (dy.wrapping_mul(34) / 22).wrapping_add(dx)
}

/// `town_siege_defeat_visible_units_near_cell` (`0x0062FB90(ecx = escena, x, y)`): con el asalto en marcha
/// (`+0x77`), abate a la primera figura que quede a menos de 18 px de `(x, y)` (35 los cabecillas) y que aún esté
/// en pie, por este orden: defensores de las cuatro listas del registro (`+0x14..`, ciudad `+0x5D3`), asaltantes de
/// las cuatro listas de la escena y cabecillas (`+0x1F8`, derriba `+0x2C`). Solo mira cada lista si quedan bajas por
/// cubrir (cupo menos bajas `+0x2C/+0x22` y `+0x232/+0x21E`).
pub fn defeat_near(m: &mut dyn Mem, s: u32, x: u32, y: u32) {
    if m.r8(s + 0x77) == 0 {
        return;
    }
    let (x, y) = ((x & 0xFFFF) as i32, (y & 0xFFFF) as i32);
    let city = m.r8(s + 0x5D3);
    let rec = siege_of(m, city);
    for (k, list) in [0x14u32, 0x410, 0x80C, 0xC08].into_iter().enumerate() {
        let k = k as u32;
        let count = r16(m, rec + 0x1004 + 2 * k);
        let done = r16(m, s + 0x232 + 4 * k).wrapping_add(r16(m, s + 0x2C + 2 * k));
        if done as i32 >= count as i32 || count == 0 {
            continue;
        }
        let mut i = 0u32;
        loop {
            let e = m.r32(rec + list + 4 * i);
            if distance(m, e, x, y) < 0x12 && m.r8(e + 0x58) == 0 {
                m.w8(e + 0x58, 1);
                inc16(m, s + 0x2C + 2 * k);
                return;
            }
            i = (i + 1) & 0xFFFF;
            if i >= r16(m, rec + 0x1004 + 2 * k) {
                break;
            }
        }
    }
    for (k, list) in [0x7Cu32, 0x11C, 0x174, 0x1BC, 0x1F8].into_iter().enumerate() {
        let k = k as u32;
        let count = r16(m, s + 0x18 + 2 * k);
        let done = r16(m, s + 0x21E + 4 * k).wrapping_add(r16(m, s + 0x22 + 2 * k));
        if done as i32 >= count as i32 || count == 0 {
            continue;
        }
        let (reach, flag) = if k == 4 { (0x23, 0x2C) } else { (0x12, 0x58) };
        let mut i = 0u32;
        loop {
            let e = m.r32(s + list + 4 * i);
            if distance(m, e, x, y) < reach && m.r8(e + flag) == 0 {
                m.w8(e + flag, 1);
                inc16(m, s + 0x22 + 2 * k);
                return;
            }
            i = (i + 1) & 0xFFFF;
            if i >= r16(m, s + 0x18 + 2 * k) {
                break;
            }
        }
    }
}
