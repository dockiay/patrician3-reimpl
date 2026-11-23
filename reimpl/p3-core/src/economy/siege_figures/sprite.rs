//! Animación, fotogramas y listas por casilla de las entidades de la vista.
//!
//! El conjunto de sprites (`set`) guarda: `+0x10` intervalo entre fotogramas (ms), `+0x14` direcciones, `+0x18` y
//! `+0x1C` fotogramas y máscaras por dirección (entradas de 12 bytes: `+4` ancho, `+6` alto, `+8` y `+0xA` desfase),
//! `+0x20` fotogramas por dirección, `+0x24` celdas por fotograma y dirección (`{n, _, lista}` de 12 bytes) y
//! `+0x28`/`+0x2C` la tabla de estados `{primero, último, id}`. La base de dibujo de la entidad (`entidad + 8`) tiene
//! el píxel en `+6/+0xA` (16.16, parte alta), el enlace en `+0xE`, el rumbo en `+0xC` y el fotograma en `+0xD`.
use super::{ld16, ld32, s16, vcall, view, FigureHost, Rect, CLOCK};
use crate::flatmem::Mem;

/// Conjunto de sprites de una tabla global `{cuántos, punteros}` por el índice del byte `at`.
pub(crate) fn set_by(m: &mut dyn Mem, at: u32, table: u32) -> u32 {
    let i = u32::from(m.r8(at));
    if i < m.r32(table) {
        let p = m.r32(table + 4);
        m.r32(p.wrapping_add(i * 4))
    } else {
        0
    }
}

/// Dirección que se dibuja con el rumbo `heading`.
pub fn dir_index(m: &mut dyn Mem, set: u32, heading: u8) -> u32 {
    let n = m.r32(set + 0x14);
    (u32::from(heading).wrapping_mul(n).wrapping_add(0x80) >> 8) & n.wrapping_sub(1)
}

/// `0x005576B0(ecx = estado, código)`: fija la animación siguiente.
pub fn anim_set_default(m: &mut dyn Mem, anim: u32, code: u8) {
    m.w8(anim + 9, code);
}

/// `0x005576C0(ecx = estado, conjunto, código)`: índice de la animación en la tabla de estados (`0xFF` si no está).
pub fn anim_find(m: &mut dyn Mem, set: u32, code: u32) -> u8 {
    let n = m.r32(set + 0x28);
    let states = m.r32(set + 0x2C);
    let mut i = 0u8;
    while u32::from(i) < n {
        if m.r32(states.wrapping_add(u32::from(i) * 12 + 8)) == code & 0x1F {
            return i;
        }
        i = i.wrapping_add(1);
    }
    0xFF
}

/// `0x00557730(ecx = estado, conjunto, código)`: empieza la animación `código`; devuelve su fotograma inicial.
pub fn anim_select(m: &mut dyn Mem, anim: u32, set: u32, code: u8) -> u8 {
    let i = anim_find(m, set, u32::from(code));
    if i == 0xFF {
        return 0xFF;
    }
    let states = m.r32(set + 0x2C);
    let e = states.wrapping_add(u32::from(i) * 12);
    m.w8(anim + 8, code);
    let t = m.r32(CLOCK).wrapping_add(m.r32(set + 0x10));
    if t < m.r32(anim + 4) {
        m.w32(anim + 4, t);
    }
    match code & 0xE0 {
        0 | 0xC0 => {
            m.w8(anim + 9, code);
            m.r8(e)
        }
        0x20 | 0xA0 => {
            m.w8(anim + 9, code);
            m.r8(e + 4)
        }
        0x40 | 0x80 => {
            m.w8(anim + 9, code.wrapping_add(0x20));
            m.r8(e)
        }
        _ => {
            m.w8(anim + 9, code.wrapping_sub(0x20));
            m.r8(e + 4)
        }
    }
}

/// `0x00557340(ecx = estado, conjunto, base, contador)`: fotograma que toca con el reloj. `base` es la base de
/// dibujo (rumbo `+0xC`, fotograma `+0xD`); `contador`, si no es 0, baja en cada cambio. Al acabar la animación
/// llama al método `+8` del estado y empieza la siguiente (o repite la actual si no existe).
pub fn anim_advance(m: &mut dyn Mem, h: &mut dyn FigureHost, anim: u32, set: u32, base: u32, counter: u32) -> u8 {
    let heading = m.r8(base + 0xC);
    let mut f = m.r8(base + 0xD);
    let dir = dir_index(m, set, heading);
    if m.r32(CLOCK) < m.r32(anim + 4) {
        return f;
    }
    if counter != 0 {
        let c = m.r8(counter);
        m.w8(counter, c.wrapping_sub(1));
    }
    let state = m.r8(anim + 8);
    let idx = anim_find(m, set, u32::from(state));
    let e = m.r32(set + 0x2C).wrapping_add(u32::from(idx) * 12);
    let count = ld32(m, set + 0x20, dir * 4);
    let finished = match state & 0xE0 {
        0 | 0x40 | 0x80 => {
            f = f.wrapping_add(1);
            !(u32::from(f) < count && f < m.r8(e + 4).wrapping_add(1))
        }
        0x20 | 0x60 | 0xE0 => {
            f = f.wrapping_sub(1);
            !(u32::from(f) < count && f > m.r8(e).wrapping_sub(1))
        }
        _ => true,
    };
    if finished {
        vcall(m, h, anim, 8, &[set]);
        let next = m.r8(anim + 9);
        let r = anim_select(m, anim, set, next);
        f = if r != 0xFF {
            r
        } else {
            let cur = m.r8(anim + 8);
            anim_select(m, anim, set, cur)
        };
    }
    let count = ld32(m, set + 0x20, dir * 4);
    if u32::from(f) >= count {
        f = 0;
    }
    let interval = m.r32(set + 0x10);
    let clock = m.r32(CLOCK);
    let next = if m.r8(anim + 8) == 6 { (interval >> 1).wrapping_add(clock) } else { interval.wrapping_add(clock) };
    m.w32(anim + 4, next);
    f
}

/// `0x005591B0(conjunto, base)`: fila de profundidad del fotograma (la celda que ancla el dibujo).
pub fn set_depth(m: &mut dyn Mem, set: u32, base: u32) -> u32 {
    let heading = m.r8(base + 0xC);
    let d = dir_index(m, set, heading);
    let frame = u32::from(m.r8(base + 0xD));
    let cells = ld32(m, set + 0x24, d * 4);
    let e = cells.wrapping_add(frame * 12);
    let k = m.r32(e + 4);
    let list = m.r32(e + 8);
    let c = list.wrapping_add(k.wrapping_mul(12));
    let y = s16(m, base + 0xA);
    (u32::from(m.r16(c + 6)) as i32 + s16(m, c + 0xA) + y) as u32
}

/// `0x005586D0(ecx = entidad)`: profundidad de dibujo (`+0x30`).
pub fn entity_depth(m: &mut dyn Mem, h: &mut dyn FigureHost, e: u32) -> u32 {
    let set = vcall(m, h, e, 0x4C, &[]);
    let base = if e != 0 { e + 8 } else { 0 };
    set_depth(m, set, base)
}

fn is_empty(r: &Rect) -> bool {
    r[2] <= r[0] || r[3] <= r[1]
}

/// `UnionRect` de Windows.
pub fn union_rect(a: &Rect, b: &Rect) -> Rect {
    match (is_empty(a), is_empty(b)) {
        (true, true) => [0; 4],
        (true, false) => *b,
        (false, true) => *a,
        _ => [a[0].min(b[0]), a[1].min(b[1]), a[2].max(b[2]), a[3].max(b[3])],
    }
}

/// `IntersectRect` de Windows.
pub fn intersect_rect(a: &Rect, b: &Rect) -> Rect {
    let r = [a[0].max(b[0]), a[1].max(b[1]), a[2].min(b[2]), a[3].min(b[3])];
    if is_empty(a) || is_empty(b) || is_empty(&r) {
        [0; 4]
    } else {
        r
    }
}

/// `0x00559260(rect, máscara, conjunto, base)`: rectángulos de pantalla del fotograma y de su máscara (si la hay).
pub fn frame_rects(m: &mut dyn Mem, set: u32, base: u32) -> (Rect, Option<Rect>) {
    let v = view(m);
    let x0 = s16(m, base + 6).wrapping_sub(m.r32(v + 0xC650) as i32);
    let y0 = s16(m, base + 0xA).wrapping_sub(m.r32(v + 0xC654) as i32).wrapping_add(m.r32(v + 0x18) as i32);
    let heading = m.r8(base + 0xC);
    let d = dir_index(m, set, heading);
    let f = u32::from(m.r8(base + 0xD)) * 12;
    let one = |m: &mut dyn Mem, array: u32| -> Rect {
        let e = m.r32(array.wrapping_add(d * 4)).wrapping_add(f);
        let l = s16(m, e + 8).wrapping_add(x0);
        let t = s16(m, e + 0xA).wrapping_add(y0);
        [l, t, i32::from(m.r16(e + 4)).wrapping_add(l), i32::from(m.r16(e + 6)).wrapping_add(t)]
    };
    let frames = m.r32(set + 0x18);
    let a = one(m, frames);
    let mask = m.r32(set + 0x1C);
    (a, if mask == 0 { None } else { Some(one(m, mask)) })
}

pub(crate) fn frame_union(m: &mut dyn Mem, set: u32, base: u32) -> Rect {
    match frame_rects(m, set, base) {
        (a, Some(b)) => union_rect(&a, &b),
        (a, None) => a,
    }
}

/// `0x0059B5B0(ecx = vista, rect)`: recorta con la ventana de la vista y avisa al recortador de pantalla.
pub fn clip(m: &mut dyn Mem, h: &mut dyn FigureHost, r: &Rect) -> bool {
    let v = view(m);
    let left = m.r32(v + 0x14) as i32;
    let top = m.r32(v + 0x18) as i32;
    let win = [left, top, (m.r32(v + 0x2C) as i32).wrapping_add(left), (m.r32(v + 0x30) as i32).wrapping_add(top)];
    let c = intersect_rect(r, &win);
    if is_empty(&c) {
        return false;
    }
    h.dirty(m, c);
    true
}

/// `0x00559130(rect, conjunto, base)`: repinta el fotograma (unido a `rect` si lo hay).
pub fn invalidate(m: &mut dyn Mem, h: &mut dyn FigureHost, extra: Option<Rect>, set: u32, base: u32) {
    let mut u = frame_union(m, set, base);
    if let Some(e) = extra {
        u = union_rect(&u, &e);
    }
    clip(m, h, &u);
}

/// `0x005934E0(ecx = vista, &x, &y, &enlace, &índice)`: alta del enlace en la lista de la casilla.
pub fn add_to_cell(m: &mut dyn Mem, h: &mut dyn FigureHost, x: u32, y: u32, link: u16, idx: u8) {
    let v = view(m);
    let i = m.r32(v + 0xC314).wrapping_mul(y).wrapping_add(x);
    let cell = m.r32(v + 0xC310).wrapping_add(i.wrapping_mul(4));
    if m.r16(cell) != 0 {
        links_insert(m, h, cell, link, idx);
    } else {
        m.w16(cell, link);
        m.w8(cell + 2, idx);
    }
}

/// `0x00593540(ecx = vista, &x, &y, &enlace, &índice)`: baja del enlace de la lista de la casilla.
pub fn remove_from_cell(m: &mut dyn Mem, x: u32, y: u32, link: u16, idx: u8) -> bool {
    let v = view(m);
    let i = m.r32(v + 0xC314).wrapping_mul(y).wrapping_add(x);
    let cell = m.r32(v + 0xC310).wrapping_add(i.wrapping_mul(4));
    links_remove(m, cell, link, idx)
}

fn registry(m: &mut dyn Mem, id: u32) -> u32 {
    let v = view(m);
    let t = m.r32(v + 0x3FC);
    m.r32(t.wrapping_add(id * 4))
}

/// `0x00559A50(celda, enlace, índice)`: inserta el enlace en la lista de la casilla ordenada por profundidad.
pub fn links_insert(m: &mut dyn Mem, h: &mut dyn FigureHost, cell: u32, link: u16, idx: u8) {
    let p3 = registry(m, u32::from(link));
    let first = u32::from(m.r16(cell));
    let mut p4 = registry(m, first);
    let d3 = vcall(m, h, p3, 0x30, &[]);
    let d4 = vcall(m, h, p4, 0x30, &[]);
    if d3 < d4 {
        let tab3 = m.r32(p3 + 4);
        let (w, b) = (m.r16(cell), m.r8(cell + 2));
        m.w16(u32::from(idx) * 4 + tab3, w);
        let tab3 = m.r32(p3 + 4);
        m.w8(u32::from(idx) * 4 + 2 + tab3, b);
        m.w16(cell, link);
        m.w8(cell + 2, idx);
        return;
    }
    let mut cur = m.r8(cell + 2);
    loop {
        let e = u32::from(cur) * 4 + m.r32(p4 + 4);
        let next = m.r16(e);
        if next == 0 {
            let e = u32::from(cur) * 4 + m.r32(p4 + 4);
            m.w16(e, link);
            let e = u32::from(cur) * 4 + m.r32(p4 + 4);
            m.w8(e + 2, idx);
            return;
        }
        let after = m.r8(e + 2);
        let holder = p4;
        p4 = registry(m, u32::from(next));
        let d3 = vcall(m, h, p3, 0x30, &[]);
        let d4 = vcall(m, h, p4, 0x30, &[]);
        if d3 < d4 {
            let tab3 = m.r32(p3 + 4);
            let e = u32::from(cur) * 4 + m.r32(holder + 4);
            let w = m.r16(e);
            m.w16(u32::from(idx) * 4 + tab3, w);
            let e = u32::from(cur) * 4 + m.r32(holder + 4);
            let b = m.r8(e + 2);
            let tab3 = m.r32(p3 + 4);
            m.w8(u32::from(idx) * 4 + 2 + tab3, b);
            let e = u32::from(cur) * 4 + m.r32(holder + 4);
            m.w16(e, link);
            let e = u32::from(cur) * 4 + m.r32(holder + 4);
            m.w8(e + 2, idx);
            return;
        }
        cur = after;
    }
}

/// `0x00559BB0(celda, enlace, índice)`: quita el enlace de la lista de la casilla. Devuelve si estaba.
pub fn links_remove(m: &mut dyn Mem, cell: u32, link: u16, idx: u8) -> bool {
    let first = m.r16(cell);
    let mut obj = registry(m, u32::from(first));
    let mut slot = m.r8(cell + 2);
    if ld16(m, obj + 4, u32::from(slot) * 4) == 0 {
        // Un único elemento: la casilla queda vacía.
        m.w16(cell, 0);
        m.w8(cell + 2, 0);
        return false;
    }
    let mut prev = 0u32;
    let mut prev_slot = 0u8;
    loop {
        if m.r16(obj + 0x16) == link && slot == idx {
            let at = u32::from(slot) * 4;
            if prev != 0 {
                let src = m.r32(obj + 4).wrapping_add(at);
                let (w, b) = (m.r16(src), m.r8(src + 2));
                let dst = m.r32(prev + 4).wrapping_add(u32::from(prev_slot) * 4);
                m.w16(dst, w);
                let dst = m.r32(prev + 4).wrapping_add(u32::from(prev_slot) * 4);
                m.w8(dst + 2, b);
            } else {
                let src = m.r32(obj + 4).wrapping_add(at);
                let w = m.r16(src);
                m.w16(cell, w);
                let src = m.r32(obj + 4).wrapping_add(at);
                let b = m.r8(src + 2);
                m.w8(cell + 2, b);
            }
            let t = m.r32(obj + 4).wrapping_add(at);
            m.w8(t + 2, 0);
            let t = m.r32(obj + 4).wrapping_add(at);
            m.w16(t, 0);
            return true;
        }
        prev_slot = slot;
        prev = obj;
        let e = m.r32(obj + 4).wrapping_add(u32::from(slot) * 4);
        let next = m.r16(e);
        if next == 0 {
            return true;
        }
        slot = m.r8(e + 2);
        obj = registry(m, u32::from(next));
        if obj == 0 {
            return true;
        }
    }
}

/// Recorre las celdas del fotograma (de la última a la primera) con la casilla de cada una.
fn each_cell(m: &mut dyn Mem, h: &mut dyn FigureHost, set: u32, base: u32, f: &mut dyn FnMut(&mut dyn Mem, &mut dyn FigureHost, u32, u32, u8)) {
    let heading = m.r8(base + 0xC);
    let d = dir_index(m, set, heading);
    let frame = u32::from(m.r8(base + 0xD));
    let cells = ld32(m, set + 0x24, d * 4);
    let e = cells.wrapping_add(frame * 12);
    let x = s16(m, base + 6);
    let y = s16(m, base + 0xA);
    let mut j = m.r32(e) as i32 - 1;
    while j >= 0 {
        let c = m.r32(e + 8).wrapping_add(j as u32 * 12);
        let px = s16(m, c + 8).wrapping_add(x.wrapping_add(i32::from(m.r16(c + 4) >> 1)));
        let py = i32::from(m.r16(c + 6)).wrapping_add(y.wrapping_add(s16(m, c + 0xA)));
        let (cx, cy) = h.pixel_to_cell(m, px, py);
        f(m, h, cx, cy, j as u8);
        j -= 1;
    }
}

/// `0x00559810(conjunto, base)`: alta de cada celda del fotograma en su casilla.
pub fn register_cells(m: &mut dyn Mem, h: &mut dyn FigureHost, set: u32, base: u32) {
    let link = m.r16(base + 0xE);
    each_cell(m, h, set, base, &mut |m, h, x, y, j| add_to_cell(m, h, x, y, link, j));
}

/// `0x005596C0(conjunto, base)`: baja de cada celda del fotograma.
pub fn unregister_cells(m: &mut dyn Mem, h: &mut dyn FigureHost, set: u32, base: u32) {
    let link = m.r16(base + 0xE);
    each_cell(m, h, set, base, &mut |m, _, x, y, j| {
        remove_from_cell(m, x, y, link, j);
    });
}

/// `0x005595F0(rect, conjunto, base)`: borra el fotograma de la vista. Con `rect` devuelve su rectángulo (no repinta).
pub fn erase(m: &mut dyn Mem, h: &mut dyn FigureHost, keep: bool, set: u32, base: u32) -> Option<Rect> {
    let r = frame_union(m, set, base);
    let out = if keep {
        Some(r)
    } else {
        clip(m, h, &r);
        None
    };
    unregister_cells(m, h, set, base);
    out
}

/// `0x00559790(conjunto, base, rect)`: alta del fotograma en las casillas y repintado (unido a `rect`).
pub fn place(m: &mut dyn Mem, h: &mut dyn FigureHost, set: u32, base: u32, extra: Option<Rect>) {
    register_cells(m, h, set, base);
    let mut u = frame_union(m, set, base);
    if let Some(e) = extra {
        u = union_rect(&u, &e);
    }
    clip(m, h, &u);
}

/// Ajuste temporal del fotograma del caminante alrededor de `0x005595F0`/`0x00559790` (`sign` +1 antes, -1 después).
fn walker_adjust(m: &mut dyn Mem, e: u32, set: u32, mode: u8, sign: i32) {
    let heading = m.r8(e + 0x14);
    let d = dir_index(m, set, heading);
    let fr = u32::from(m.r8(e + 0x15)) * 12;
    let add = |m: &mut dyn Mem, a: u32, v: i32| {
        let w = m.r16(a);
        m.w16(a, (i32::from(w) + v * sign) as u16);
    };
    let row = ld32(m, set + 0x18, d * 4).wrapping_add(fr);
    match mode {
        0 => {
            add(m, row + 0xA, -0x46);
            add(m, e + 0x12, 0xD);
        }
        1 => {
            add(m, row + 0xA, -0xB);
            add(m, e + 0x12, 0xB);
        }
        _ => {
            let (dx, dy) = (i32::from(m.r16(e + 0x82)), i32::from(m.r16(e + 0x84)));
            add(m, row + 8, dx);
            let row = ld32(m, set + 0x18, d * 4).wrapping_add(fr);
            add(m, row + 0xA, dy);
            let mask = m.r32(set + 0x1C);
            if mask != 0 {
                let mrow = m.r32(mask.wrapping_add(d * 4)).wrapping_add(fr);
                add(m, mrow + 8, dx);
                let mask = m.r32(set + 0x1C);
                let mrow = m.r32(mask.wrapping_add(d * 4)).wrapping_add(fr);
                add(m, mrow + 0xA, dy);
            }
        }
    }
}

fn walker_mode(m: &mut dyn Mem, h: &mut dyn FigureHost, e: u32) -> u8 {
    if m.r8(e + 0x59) != 0 {
        0
    } else if vcall(m, h, e, 0x68, &[]) as u8 == 4 {
        1
    } else {
        2
    }
}

/// `0x0055E250(ecx = caminante, conjunto, rect)`: borra al caminante de la vista con el desfase de su clase.
pub fn walker_remove(m: &mut dyn Mem, h: &mut dyn FigureHost, e: u32, set: u32, keep: bool) -> Option<Rect> {
    let mode = walker_mode(m, h, e);
    walker_adjust(m, e, set, mode, 1);
    let r = erase(m, h, keep, set, e + 8);
    let mode = walker_mode(m, h, e);
    walker_adjust(m, e, set, mode, -1);
    r
}

/// `0x0055E430(ecx = caminante, conjunto, rect)`: da de alta al caminante en la vista con el desfase de su clase.
pub fn walker_insert(m: &mut dyn Mem, h: &mut dyn FigureHost, e: u32, set: u32, extra: Option<Rect>) {
    if m.r8(e + 0x15) == 0xFF {
        m.w8(e + 0x15, 0);
    }
    let mode = walker_mode(m, h, e);
    walker_adjust(m, e, set, mode, 1);
    place(m, h, set, e + 8, extra);
    let mode = walker_mode(m, h, e);
    walker_adjust(m, e, set, mode, -1);
}
