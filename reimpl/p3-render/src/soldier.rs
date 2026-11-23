//! Inicio de una figura de la escena (`0x55E620`) y la cadena que la acompaña, sobre la memoria plana de la escena (`Mem`).
//!
//! Funciones del original: `0x55E620` (inicio de soldado/figura), `0x633AE0`/`0x629C30` (¿el registro del lado está activo?), `0x561080` (¿la casilla
//! vecina es de tipo 0x29/0x2E/0x2F?), `0x5614E0` (propaga la prioridad `+0x7A` por la lista del lado), `0x55E430` (alta de la figura en la rejilla
//! con ajuste temporal del fotograma), `0x559790`/`0x559810`/`0x559260` (rectángulos y celdas del fotograma), `0x5934E0`/`0x559A50` (lista
//! por profundidad de cada casilla), `0x59B5B0` (recorte con la ventana), `0x559CF0` (tabla de enlaces `+4`) y `0x557730` (selección de
//! animación sobre el conjunto en memoria). Lo que depende de objetos virtuales, aleatorios o de pantalla pasa por [`SoldierEnv`].
//!
//! Las funciones de Windows (`UnionRect`, `IntersectRect`, `CopyRect`, `IsRectEmpty`) se modelan con su semántica documentada; la prueba del x86
//! sustituye esos imports por la misma semántica, así que solo verifica el flujo, no las propias API.

use crate::scene_math::{mark_mask, pixel_to_cell, step_dir, Mem};

pub const SCENE_PTR: u32 = 0x6E51AC;
pub const SIDE_BASE: u32 = 0x6DF270;
pub const TOWNS: u32 = 0x6DE508;
pub const CLOCK: u32 = 0x6DCCF8;
pub const DIR_BYTES: [u32; 4] = [0x677929, 0x67792B, 0x67792D, 0x67792F];

pub type Rect = [i32; 4];

pub trait SoldierEnv: Mem {
    /// `0x639C03` (`rand`).
    fn rand(&mut self) -> u32;
    /// `0x64F7B9` (`operator new`); la memoria devuelta se pone a cero a continuación (`_memset`).
    fn alloc(&mut self, size: u32) -> u32;
    /// `vtable[+0x68]` de la figura (clase de la figura; `4` activa el ajuste de las celdas).
    fn virt_68(&mut self, obj: u32) -> u8;
    /// `vtable[+0x50]` del objeto de la rejilla.
    fn virt_50(&mut self, obj: u32) -> u32;
    /// `vtable[+0x30]` del objeto del registro (profundidad).
    fn virt_30(&mut self, obj: u32) -> u32;
    /// Byte `+3` de la entrada que devuelve `0x51F1F0(ecx = ciudad, id)`.
    fn building_byte3(&mut self, town: u32, id: u32) -> u8;
    /// Rectángulo que pasa `0x4B9650` a las capas de dibujo.
    fn dirty(&mut self, r: Rect);
}

fn s16(m: &mut dyn Mem, a: u32) -> i32 {
    i32::from(m.r16(a) as i16)
}

fn is_empty(r: &Rect) -> bool {
    r[2] <= r[0] || r[3] <= r[1]
}

/// `UnionRect(dst, a, b)` de Windows.
pub fn union_rect(a: &Rect, b: &Rect) -> Rect {
    match (is_empty(a), is_empty(b)) {
        (true, true) => [0; 4],
        (true, false) => *b,
        (false, true) => *a,
        _ => [a[0].min(b[0]), a[1].min(b[1]), a[2].max(b[2]), a[3].max(b[3])],
    }
}

/// `IntersectRect` de Windows (vacío a ceros si no se cortan).
pub fn intersect_rect(a: &Rect, b: &Rect) -> Rect {
    let r = [a[0].max(b[0]), a[1].max(b[1]), a[2].min(b[2]), a[3].min(b[3])];
    if is_empty(a) || is_empty(b) || is_empty(&r) {
        [0; 4]
    } else {
        r
    }
}

/// `0x557730(ecx = estado de animación, conjunto, código)` sobre el conjunto en memoria. Devuelve el fotograma inicial.
pub fn select(m: &mut dyn Mem, anim: u32, set: u32, code: u32) -> u32 {
    let n = m.r32(set + 0x28);
    let states = m.r32(set + 0x2C);
    let find = |m: &mut dyn Mem| -> Option<u32> {
        let mut i = 0u8;
        while u32::from(i) < n {
            if m.r32(states + 8 + u32::from(i) * 12) == (code & 0x1F) {
                return Some(u32::from(i));
            }
            i = i.wrapping_add(1);
        }
        None
    };
    let Some(i) = find(m) else { return 0xFF };
    m.w8(anim + 8, code as u8);
    let t = m.r32(CLOCK).wrapping_add(m.r32(set + 0x10));
    if t < m.r32(anim + 4) {
        m.w32(anim + 4, t);
    }
    let first = |m: &mut dyn Mem| m.r8(states + i * 12);
    let last = |m: &mut dyn Mem| m.r8(states + 4 + i * 12);
    match code & 0xE0 {
        0 | 0xC0 => {
            m.w8(anim + 9, code as u8);
            u32::from(first(m))
        }
        0x20 | 0xA0 => {
            m.w8(anim + 9, code as u8);
            u32::from(last(m))
        }
        0x40 | 0x80 => {
            m.w8(anim + 9, (code as u8).wrapping_add(0x20));
            u32::from(first(m))
        }
        _ => {
            m.w8(anim + 9, (code as u8).wrapping_sub(0x20));
            u32::from(last(m))
        }
    }
}

fn dir_index(m: &mut dyn Mem, set: u32, heading: u8) -> u32 {
    let n = m.r32(set + 0x14);
    (u32::from(heading).wrapping_mul(n).wrapping_add(0x80) >> 8) & n.wrapping_sub(1)
}

/// `0x559260(rect, mascara, conjunto, base)`: rectángulos de pantalla del fotograma de la figura (`base` = figura + 8). Devuelve la máscara y si existe.
pub fn frame_rects(m: &mut dyn Mem, scene: u32, set: u32, o: u32) -> (Rect, Option<Rect>) {
    let x0 = s16(m, o + 6) - m.r32(scene + 0xC650) as i32;
    let y0 = s16(m, o + 10) - m.r32(scene + 0xC654) as i32 + m.r32(scene + 0x18) as i32;
    let hd = m.r8(o + 0xC);
    let d = dir_index(m, set, hd);
    let f = u32::from(m.r8(o + 0xD)) * 12;
    let one = |m: &mut dyn Mem, array: u32| -> Rect {
        let e = m.r32(array + d * 4).wrapping_add(f);
        let l = s16(m, e + 8) + x0;
        let t = s16(m, e + 10) + y0;
        [l, t, i32::from(m.r16(e + 4)) + l, i32::from(m.r16(e + 6)) + t]
    };
    let arr = m.r32(set + 0x18);
    let a = one(m, arr);
    let mask = m.r32(set + 0x1C);
    (a, if mask == 0 { None } else { Some(one(m, mask)) })
}

/// `0x559A50(celda, enlace, índice)`: inserta el enlace en la lista de la casilla ordenada por la profundidad de cada figura.
pub fn insert_by_depth(env: &mut dyn SoldierEnv, scene: u32, cell: u32, link: u16, idx: u8) {
    let reg = env.r32(scene + 0x3FC);
    let obj = |env: &mut dyn SoldierEnv, id: u32| env.r32(reg.wrapping_add(id * 4));
    let p3 = obj(env, u32::from(link));
    let first = u32::from(env.r16(cell));
    let mut p4 = obj(env, first);
    let d3 = env.virt_30(p3);
    let d4 = env.virt_30(p4);
    let tab3 = env.r32(p3 + 4);
    if d3 < d4 {
        let (w, b) = (env.r16(cell), env.r8(cell + 2));
        env.w16(u32::from(idx) * 4 + tab3, w);
        env.w8(u32::from(idx) * 4 + 2 + tab3, b);
        env.w16(cell, link);
        env.w8(cell + 2, idx);
        return;
    }
    let mut cur = env.r8(cell + 2);
    loop {
        let e = u32::from(cur) * 4 + env.r32(p4 + 4);
        let next = env.r16(e);
        if next == 0 {
            env.w16(e, link);
            env.w8(e + 2, idx);
            return;
        }
        let after = env.r16(e + 2) as u8;
        let holder = p4;
        p4 = obj(env, u32::from(next));
        let d3 = env.virt_30(p3);
        let d4 = env.virt_30(p4);
        if d4 > d3 {
            let e = u32::from(cur) * 4 + env.r32(holder + 4);
            let (w, b) = (env.r16(e), env.r8(e + 2));
            env.w16(u32::from(idx) * 4 + tab3, w);
            env.w8(u32::from(idx) * 4 + 2 + tab3, b);
            env.w16(e, link);
            env.w8(e + 2, idx);
            return;
        }
        cur = after;
    }
}

/// `0x5934E0(ecx = escena, &x, &y, &enlace, &índice)`.
pub fn mark_cell(env: &mut dyn SoldierEnv, scene: u32, x: u32, y: u32, link: u16, idx: u8) {
    let grid = env.r32(scene + 0xC310);
    let i = env.r32(scene + 0xC314).wrapping_mul(y).wrapping_add(x);
    let cell = grid.wrapping_add(i.wrapping_mul(4));
    if env.r16(cell) != 0 {
        insert_by_depth(env, scene, cell, link, idx);
    } else {
        env.w16(cell, link);
        env.w8(cell + 2, idx);
    }
}

/// `0x559810(conjunto, base)`: da de alta en la rejilla cada celda del fotograma (de la última a la primera).
pub fn mark_cells(env: &mut dyn SoldierEnv, scene: u32, set: u32, o: u32) {
    let frame = u32::from(env.r8(o + 0xD));
    let hd = env.r8(o + 0xC);
    let d = dir_index(env, set, hd);
    let arr = env.r32(set + 0x24);
    let cells = env.r32(arr.wrapping_add(d * 4));
    let y = s16(env, o + 10);
    let x = s16(env, o + 6);
    let k = env.r32(cells.wrapping_add(frame * 12)) as i32 - 1;
    let link = env.r16(o + 0xE);
    let mut j = k;
    while j >= 0 {
        let e = env.r32(cells.wrapping_add(frame * 12 + 8)).wrapping_add(j as u32 * 12);
        let px = s16(env, e + 8) + x + i32::from(env.r16(e + 4) >> 1);
        let py = i32::from(env.r16(e + 6)) + y + s16(env, e + 10);
        let (cx, cy, _) = pixel_to_cell(env, scene, px, py);
        mark_cell(env, scene, cx, cy, link, j as u8);
        j -= 1;
    }
}

/// `0x59B5B0(ecx = escena, rect)`: recorta con la ventana y avisa a `0x4B9650` (que decide si hay que dibujar y recorre las capas: externo). Devuelve si quedó algo.
pub fn clip_and_invalidate(env: &mut dyn SoldierEnv, scene: u32, r: &Rect) -> bool {
    let left = env.r32(scene + 0x14) as i32;
    let top = env.r32(scene + 0x18) as i32;
    let win = [left, top, (env.r32(scene + 0x2C) as i32).wrapping_add(left), (env.r32(scene + 0x30) as i32).wrapping_add(top)];
    let c = intersect_rect(r, &win);
    if is_empty(&c) {
        return false;
    }
    env.dirty(c);
    true
}

/// `0x559790(conjunto, base, rect_extra)`.
pub fn paint(env: &mut dyn SoldierEnv, scene: u32, set: u32, o: u32, extra: Option<Rect>) {
    mark_cells(env, scene, set, o);
    let (a, mask) = frame_rects(env, scene, set, o);
    let mut u = match mask {
        Some(b) => union_rect(&a, &b),
        None => a,
    };
    if let Some(e) = extra {
        u = union_rect(&u, &e);
    }
    clip_and_invalidate(env, scene, &u);
}

/// `0x55E430(ecx = figura, conjunto, rect)`: ajusta el fotograma de la figura, la da de alta con `0x559790` y deshace el ajuste.
pub fn place(env: &mut dyn SoldierEnv, scene: u32, obj: u32, set: u32, extra: Option<Rect>) {
    let hd = env.r8(obj + 0x14);
    let dir = dir_index(env, set, hd);
    if env.r8(obj + 0x15) == 0xFF {
        env.w8(obj + 0x15, 0);
    }
    let frames = env.r32(set + 0x18);
    let mask = env.r32(set + 0x1C);
    let fr = u32::from(env.r8(obj + 0x15)) * 12;
    let row = env.r32(frames.wrapping_add(dir * 4));
    let (dx, dy) = (env.r16(obj + 0x82), env.r16(obj + 0x84));
    let flag = env.r8(obj + 0x59) != 0;
    let adj = |env: &mut dyn SoldierEnv, a: u32, v: u16, sub: bool| {
        let w = env.r16(a);
        env.w16(a, if sub { w.wrapping_sub(v) } else { w.wrapping_add(v) });
    };
    let mut four = false;
    if !flag {
        four = env.virt_68(obj) == 4;
        if four {
            adj(env, row + 10 + fr, 0xB, true);
            adj(env, obj + 0x12, 0xB, false);
        } else {
            adj(env, row + 8 + fr, dx, false);
            adj(env, row + 10 + fr, dy, false);
            if mask != 0 {
                let mrow = env.r32(mask.wrapping_add(dir * 4));
                adj(env, mrow + 8 + fr, dx, false);
                adj(env, mrow + 10 + fr, dy, false);
            }
        }
    } else {
        adj(env, row + 10 + fr, 0x46, true);
        adj(env, obj + 0x12, 0xD, false);
    }
    paint(env, scene, set, obj + 8, extra);
    let fr = u32::from(env.r8(obj + 0x15)) * 12;
    if flag {
        adj(env, row + 10 + fr, 0x46, false);
        adj(env, obj + 0x12, 0xD, true);
        return;
    }
    if env.virt_68(obj) == 4 {
        adj(env, row + 10 + fr, 0xB, false);
        adj(env, obj + 0x12, 0xB, true);
        let _ = four;
        return;
    }
    adj(env, row + 8 + fr, dx, true);
    adj(env, row + 10 + fr, dy, true);
    if mask != 0 {
        let mrow = env.r32(mask.wrapping_add(dir * 4));
        adj(env, mrow + 8 + fr, dx, true);
        adj(env, mrow + 10 + fr, dy, true);
    }
}

/// `0x629C30(ecx = registro)`.
pub fn side_active(m: &mut dyn Mem, rec: u32) -> bool {
    let w = m.r16(rec + 0x3A);
    if w != 0 {
        m.r16(rec + 0x6A) < w
    } else {
        m.r16(rec + 0x21E) != 0
    }
}

/// `0x633AE0(ecx = contenedor del lado)`.
pub fn side_container_active(m: &mut dyn Mem, container: u32) -> bool {
    let rec = m.r32(container + 8);
    rec != 0 && side_active(m, rec)
}

/// `0x561080(x, y, dir)`: ¿la casilla vecina en esa dirección contiene un objeto de tipo 0x29, 0x2E o 0x2F?
pub fn neighbour_is_gate(env: &mut dyn SoldierEnv, scene: u32, x: u32, y: u32, dir: u8) -> bool {
    let (mut a, mut b) = (x, y);
    step_dir(env, scene, &mut a, &mut b, dir);
    if dir == env.r8(DIR_BYTES[0]) || dir == env.r8(DIR_BYTES[3]) {
        step_dir(env, scene, &mut a, &mut b, dir);
    }
    if a < env.r32(scene + 0xC314) && b < env.r32(scene + 0xC318) {
        let i = env.r32(scene + 0xC314).wrapping_mul(b).wrapping_add(a);
        let g = env.r32(scene + 0xC30C);
        let id = env.r16(g.wrapping_add(i.wrapping_mul(4)));
        if id != 0 {
            let t98 = env.r32(scene + 0x98);
            let obj = env.r32(t98.wrapping_add(u32::from(id) * 4));
            let t = env.virt_50(obj);
            return t == 0x29 || (t > 0x2D && t < 0x30);
        }
    }
    false
}

/// `0x5614E0(ecx = figura, valor)`: propaga la prioridad `+0x7A` por la lista de la figura.
pub fn propagate_priority(m: &mut dyn Mem, mut obj: u32, mut v: u8) {
    if m.r8(obj + 0x7A) == v {
        return;
    }
    loop {
        m.w8(obj + 0x7A, v);
        obj = m.r32(obj + 0x5C);
        if obj == 0 || v < m.r8(obj + 0x7A) {
            return;
        }
        v = v.wrapping_add(1);
        if m.r8(obj + 0x7A) == v {
            return;
        }
    }
}

/// Argumentos de `0x55E620` (tras `ecx = figura`), en el orden de la pila.
#[derive(Clone, Copy, Debug)]
pub struct InitArgs {
    pub x: u32,
    pub y: u32,
    pub heading: u8,
    pub set: u32,
    pub link: u16,
    pub side: u8,
    pub bits: u32,
    pub flag: u8,
    pub kind: u8,
    pub param: u8,
}

/// `0x55E620`. Devuelve el cociente de la última división (`EAX`).
pub fn init_figure(env: &mut dyn SoldierEnv, obj: u32, a: &InitArgs) -> u32 {
    let scene = env.r32(SCENE_PTR);
    env.w8(obj + 0x14, a.heading);
    env.w8(obj + 0x59, a.flag);
    env.w32(obj + 0xC, a.x);
    env.w16(obj + 0x16, a.link);
    env.w32(obj + 0x10, a.y);
    env.w16(obj + 0x38, 0);
    env.w16(obj + 0x3A, 0);
    env.w8(obj + 0x6A, a.side);
    let f = select(env, obj + 0x2C, a.set, 2);
    env.w8(obj + 0x15, f as u8);
    env.w8(obj + 0x7B, a.kind);
    env.w32(obj + 0x30, 0);
    let (px, py) = (s16(env, obj + 0xE), s16(env, obj + 0x12));
    let (cx, cy, _) = pixel_to_cell(env, scene, px, py);
    let sides = env.r32(SIDE_BASE);
    let container = sides.wrapping_add(u32::from(a.side) * 0x1024);
    let rec = env.r32(container + 8);
    if rec != 0 && env.r32(rec + 0x70) != 0 {
        mark_mask(env, rec, scene, cx as i16, cy as i16, a.bits as u8);
    }
    env.w16(obj + 0x6C, cx as u16);
    env.w16(obj + 0x6E, cy as u16);
    if env.r8(obj + 0x59) != 0 {
        let sides = env.r32(SIDE_BASE);
        let container = sides.wrapping_add(u32::from(env.r8(obj + 0x6A)) * 0x1024);
        if !side_container_active(env, container) {
            env.w8(obj + 0x5B, 0);
            env.w8(obj + 0x5A, 0);
            let r = env.rand();
            env.w8(obj + 0x14, ((r & 3) as u8).wrapping_mul(0x40).wrapping_add(0x20));
            let mut n = 0u8;
            loop {
                let cur = env.r8(obj + 0x14);
                if neighbour_is_gate(env, scene, cx, cy, cur) {
                    break;
                }
                let d = env.r8(obj + 0x14).wrapping_add(0x40);
                n += 1;
                env.w8(obj + 0x14, d);
                if n == 4 {
                    break;
                }
            }
        } else {
            let rec = env.r32(container + 8);
            let idx = u32::from(env.r8(rec + 0x6D));
            let id = env.r32(rec + 0x244 + idx * 4);
            let side = u32::from(env.r8(obj + 0x6A));
            let town = env.r32(TOWNS).wrapping_add(side.wrapping_mul(319).wrapping_mul(8));
            let b3 = env.building_byte3(town, id);
            let north = cy > u32::from(env.r16(rec + 0xA));
            env.w8(obj + 0x5B, north as u8);
            let (hi, lo) = if north { (DIR_BYTES[1], DIR_BYTES[2]) } else { (DIR_BYTES[3], DIR_BYTES[0]) };
            let d = env.r8(if b3 == 0x38 { hi } else { lo });
            env.w8(obj + 0x14, d);
            env.w8(obj + 0x5A, 0);
        }
        let side = u32::from(env.r8(obj + 0x6A));
        let off = (side * 0x409 + u32::from(env.r8(obj + 0x5B))) * 4 + 0xC;
        let base = env.r32(SIDE_BASE);
        let head = env.r32(base.wrapping_add(off));
        env.w32(obj + 0x5C, head);
        let base = env.r32(SIDE_BASE);
        env.w32(base.wrapping_add(off), obj);
        let h = env.r8(obj + 0x14);
        env.w8(obj + 0x79, h);
        if env.r8(obj + 0x7A) != 2 {
            env.w8(obj + 0x7A, 2);
            let next = env.r32(obj + 0x5C);
            if next != 0 && env.r8(next + 0x7A) < 3 {
                propagate_priority(env, next, 3);
            }
        }
    }
    let h = env.r8(obj + 0x14);
    env.w8(obj + 0x3C, h);
    // 0x55A290(ecx = figura+0x20, 0, 1, 0, 0)
    env.w8(obj + 0x20 + 7, 0);
    env.w8(obj + 0x20 + 4, 1);
    env.w8(obj + 0x20 + 5, 0);
    env.w8(obj + 0x20 + 6, 0);
    // 0x559CF0(ecx = figura, conjunto): tabla de enlaces por celda
    let size = env.r32(a.set + 0x3C).wrapping_mul(4);
    let table = env.alloc(size);
    for i in 0..size {
        env.w8(table.wrapping_add(i), 0);
    }
    env.w32(obj + 4, table);
    place(env, scene, obj, a.set, None);
    let r = env.rand() as i32;
    env.w16(obj + 0x82, (r % 7 - 3) as u16);
    let r = env.rand() as i32;
    env.w16(obj + 0x84, (r % 5 - 2) as u16);
    let r = env.rand() as i32;
    env.w32(obj + 0x64, 0);
    env.w8(obj + 0x86, ((r % 0x19) as u8).wrapping_add(0x74));
    env.w8(obj + 0x60, a.param);
    (r / 0x19) as u32
}
