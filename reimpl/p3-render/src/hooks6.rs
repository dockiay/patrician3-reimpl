//! Seis ayudantes de la escena que seguían como ganchos: `0x58CFB0`, `0x5986A0`, `0x563090`, `0x575560`, `0x565D70` y `0x57AF60`.
//! Traducción 1:1 sobre memoria plana (`Mem`, direcciones absolutas); paridad en `tests/x86_hooks6.rs` frente al x86 original.
use crate::scene_math::Mem;

const SCENE_PTR: u32 = 0x006E_51AC;
const DIR_29: u32 = 0x0067_7929;
const DIR_2B: u32 = 0x0067_792B;
const DIR_2D: u32 = 0x0067_792D;
const DIR_2F: u32 = 0x0067_792F;

pub trait Hooks6 {
    /// `0x59EDF0(ecx = escena + 0x5A0, x, y, 0xFF)`: marca un punto en la máscara.
    fn f59edf0(&mut self, ecx: u32, x: i32, y: i32, v: u32);
    /// `0x565E10(ecx = objeto, a, b)`: último paso de `0x565D70`.
    fn f565e10(&mut self, obj: u32, a: u32, b: u32);
}

/// Fórmula pura de `0x58CFB0` (casilla `(x, y)` a píxel) con los cuatro campos de proyección `(+0x510, +0x514, +0x4D0, +0x4D4)`.
pub fn cell_to_pixel_alt_pure(x: i32, y: u32, c510: i32, c514: i32, c4d0: i32, c4d4: i32) -> (u32, u32) {
    let half = (y >> 1) as i32;
    if y & 1 != 0 {
        return (x.wrapping_add(1).wrapping_mul(c510) as u32, c514.wrapping_add(1).wrapping_mul(half).wrapping_add(c514) as u32);
    }
    (c510.wrapping_mul(x).wrapping_add(c4d0) as u32, c514.wrapping_add(1).wrapping_mul(half).wrapping_sub(c4d4).wrapping_add(c514) as u32)
}

/// `0x58CFB0(escena, &(x, y), &sx, &sy)`: casilla a píxel (variante sin el desplazamiento de origen de `0x58DAC0`).
pub fn cell_to_pixel_alt(m: &mut dyn Mem, scene: u32, at: u32, out_x: u32, out_y: u32) {
    let (x, y) = (m.r32(at) as i32, m.r32(at + 4));
    let (c510, c514, c4d0, c4d4) = (m.r32(scene + 0x510) as i32, m.r32(scene + 0x514) as i32, m.r32(scene + 0x4D0) as i32, m.r32(scene + 0x4D4) as i32);
    let (px, py) = cell_to_pixel_alt_pure(x, y, c510, c514, c4d0, c4d4);
    m.w32(out_x, px);
    m.w32(out_y, py);
}

/// `0x5986A0(escena, x, y)`.
pub fn mark_pixel(m: &mut dyn Mem, scene: u32, x: u32, y: u32, env: &mut dyn Hooks6) {
    if m.r8(scene + 0x524) == 0 {
        return;
    }
    let dx = ((x & 0xFFFF) as i32).wrapping_sub(m.r32(scene + 0xC650) as i32);
    let top = m.r32(scene + 0x18) as i32;
    let dy = (((y & 0xFFFF) as i32).wrapping_sub(m.r32(scene + 0xC654) as i32)).wrapping_add(top);
    if dx >= 0 && dx <= m.r32(scene + 0x2C) as i32 && top <= dy && dy < m.r32(0x006D_CCE4) as i32 {
        env.f59edf0(scene + 0x5A0, dx, dy, 0xFF);
    }
}

/// Factores de `0x7C70/0x2981` y `0xB50/0x271` sobre el campo `+0xC` de una fila (aritmética de 32 bits sin signo).
fn ratios(q: u32) -> (u32, u32) {
    (q.wrapping_mul(0x7C70) / 0x2981, q.wrapping_mul(0xB50) / 0x271)
}

/// Cuerpo común de `0x563090` y `0x575560`: dirección y velocidades según el signo de `(x, y)` frente al objetivo.
/// Devuelve `(byte de dirección, velocidad A, velocidad B)` ya con signo.
fn heading(m: &mut dyn Mem, x_lt: bool, y_lt: bool, q: u32) -> (u8, u32, u32) {
    let (a, b) = ratios(q);
    if x_lt {
        if y_lt {
            (m.r8(DIR_2F), a.wrapping_neg(), b.wrapping_neg())
        } else {
            (m.r8(DIR_2D), a, b.wrapping_neg())
        }
    } else if y_lt {
        (m.r8(DIR_29), a.wrapping_neg(), b)
    } else {
        (m.r8(DIR_2B), a, b)
    }
}

fn row_field(m: &mut dyn Mem, slot_count: u32, slot_ptr: u32, idx: u8) -> u32 {
    if u32::from(idx) < slot_count {
        let base = m.r32(slot_ptr);
        let p = m.r32(base.wrapping_add(u32::from(idx) * 4));
        m.r32(p.wrapping_add(0xC))
    } else {
        m.r32(0xC)
    }
}

/// `0x563090(objeto)`.
pub fn aim_at_row(m: &mut dyn Mem, obj: u32) {
    let scene = m.r32(SCENE_PTR);
    let count = m.r32(0x006E_1700);
    let idx = m.r8(obj + 0x2D);
    let q = row_field(m, count, 0x006E_1704, idx);
    let i2 = u32::from(m.r8(obj + 0x39)) * 8;
    let tbl = {
        let t = m.r32(scene + 0xC67C);
        m.r32(t + 4)
    };
    let x = m.r16(tbl.wrapping_add(i2));
    m.w16(obj + 0x3C, x);
    let y = m.r16(tbl.wrapping_add(4).wrapping_add(i2));
    m.w16(obj + 0x3E, y);
    let tx = i32::from(m.r16(obj + 0xE) as i16);
    let ty = i32::from(m.r16(obj + 0x12) as i16);
    let (d, a, b) = heading(m, i32::from(x) < tx, i32::from(y) < ty, q);
    m.w8(obj + 0x3A, d);
    m.w32(obj + 0x34, a);
    m.w32(obj + 0x30, b);
}

/// Punto medio entre dos filas de las tablas `escena+0xC674/0xC678`: `((P1[i] + P2[j]) >> 1, (P2[j+4] + 0x16 + P1[i+4]) >> 1)`.
fn midpoint(m: &mut dyn Mem, scene: u32, row: u32, i: u32, j: u32) -> (u16, u16) {
    let t1 = m.r32(scene + 0xC674);
    let t2 = m.r32(scene + 0xC678);
    let p1 = m.r32(t1.wrapping_add(4).wrapping_add(row));
    let p2 = m.r32(t2.wrapping_add(4).wrapping_add(row));
    let (a, b) = (m.r32(p1.wrapping_add(i)) as i32, m.r32(p2.wrapping_add(j)) as i32);
    let (c, d) = (m.r32(p2.wrapping_add(j).wrapping_add(4)) as i32, m.r32(p1.wrapping_add(i).wrapping_add(4)) as i32);
    ((a.wrapping_add(b) >> 1) as u16, (c.wrapping_add(0x16).wrapping_add(d) >> 1) as u16)
}

/// `0x575560(objeto, a, b)`.
pub fn aim_between_rows(m: &mut dyn Mem, obj: u32, a: u16, b: u16) {
    let scene = m.r32(SCENE_PTR);
    let count = m.r32(0x006E_2E14);
    let idx = m.r8(obj + 0x35);
    let q = row_field(m, count, 0x006E_2E18, idx);
    let row = u32::from(m.r8(obj + 0x48)) * 8;
    let first = m.r8(obj + 0x41);
    let t2 = m.r32(scene + 0xC678).wrapping_add(row);
    let count_rows = m.r8(t2) as i8 as i32;
    let local = (count_rows.wrapping_sub(i32::from(first)).wrapping_sub(1)) as u8;
    let (x, y) = midpoint(m, scene, row, u32::from(first) * 8, u32::from(local) * 8);
    m.w16(obj + 0x44, x);
    m.w16(obj + 0x46, y);
    let (d, s, t) = heading(m, x < a, y < b, q);
    m.w8(obj + 0x42, d);
    m.w32(obj + 0x3C, s);
    m.w32(obj + 0x38, t);
}

/// `0x565D70(objeto, a, b)`.
pub fn aim_between_rows_b(m: &mut dyn Mem, obj: u32, a: u32, b: u32, env: &mut dyn Hooks6) {
    let scene = m.r32(SCENE_PTR);
    let row = u32::from(m.r8(obj + 0x44)) * 8;
    let first = m.r8(obj + 0x3D);
    let t2 = m.r32(scene + 0xC678).wrapping_add(row);
    let count_rows = m.r8(t2) as i8 as i32;
    let local = (count_rows.wrapping_sub(i32::from(first)).wrapping_sub(1)) as u8;
    let (x, y) = midpoint(m, scene, row, u32::from(first) * 8, u32::from(local) * 8);
    m.w16(obj + 0x40, x);
    m.w16(obj + 0x42, y);
    env.f565e10(obj, a, b);
}

/// Expresión del compilador para `n / 5` con signo: `(n/5) + (n>>31) - ((n*0x66666667)>>63)`, cada término truncado a `i16`.
fn div5(n: i32) -> i16 {
    ((n / 5) as i16).wrapping_add((n >> 31) as i16).wrapping_sub(((i64::from(n) * 0x6666_6667) >> 63) as i16)
}

fn add16(m: &mut dyn Mem, a: u32, v: i16) {
    let o = m.r16(a) as i16;
    m.w16(a, o.wrapping_add(v) as u16);
}

/// `0x57AF60(objeto, juego de sprites, sentido)`: desplaza el marco actual según la clase del objeto.
pub fn shift_frame(m: &mut dyn Mem, obj: u32, set: u32, forward: u8) {
    let step = i32::from(forward != 0) * 2 - 1;
    let n = m.r32(set + 0x14);
    let dir = ((u32::from(m.r8(obj + 0x14)).wrapping_mul(n).wrapping_add(0x80)) >> 8) & n.wrapping_sub(1);
    let ki = u32::from(m.r8(obj + 0x2C));
    let kb = m.r32(0x006E_2E74);
    let kind = m.r8(ki.wrapping_add(kb));
    let s = step as i16;
    let w = m.r16(obj + 0x2E) as i16;
    let tab = m.r32(set + 0x18);
    let f = m.r32(tab.wrapping_add(dir.wrapping_mul(4)));
    let fi = u32::from(m.r8(obj + 0x15));
    let frame = f.wrapping_add(fi * 12);
    if w != 0 {
        if m.r32(set + 0x1C) != 0 || matches!(kind, 0x20 | 0x24 | 0x21 | 0x22 | 0x25) {
            add16(m, frame + 6, w.wrapping_mul(s));
            let n5 = i32::from(w).wrapping_mul(step);
            add16(m, frame + 4, div5(n5));
        }
        add16(m, frame + 10, w.wrapping_mul(s).wrapping_neg());
    }
    if kind == 0x24 {
        add16(m, frame + 6, s.wrapping_mul(0x30));
        let n5 = i32::from(m.r8(obj + 0x15)).wrapping_mul(step).wrapping_mul(2);
        add16(m, frame + 4, div5(n5));
        add16(m, frame + 10, s.wrapping_mul(-0x30));
        return;
    }
    if matches!(kind, 0x30 | 0x11 | 0xB0) {
        add16(m, frame + 10, s.wrapping_mul(-0x30));
    } else if kind == 0x81 {
        add16(m, frame + 10, s.wrapping_mul(-0xB));
    }
}
