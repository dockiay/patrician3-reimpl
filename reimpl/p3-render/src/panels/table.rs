//! `TableLayout` (`0x40A440`–`0x40ACE0`): rejilla de celdas con anchos y separaciones por columna, usada por las listas y tablas de la interfaz.
//!
//! Disposición: `+0` vtable (`0x66AA68`), `+4/+8/+0xC` origen, `+0x10` alto de fila, `+0x14` filas, `+0x18` columnas, `+0x1C` anchos
//! (u32 por columna), `+0x20` separaciones, `+0x24` separación de filas, `+0x28` (-1 sin cabecera), `+0x2C`, `+0x30` alto de cabecera,
//! `+0x34` ancho total, `+0x38` alto total.

use super::{Host, Mem, Ret};
use crate::j07::free;

const VTABLE: u32 = 0x0066_AA68;

fn zero_fields(m: &mut dyn Mem, t: u32) {
    for off in [0x1C, 0x20, 0x14, 0x18, 0x10, 0x24, 0x04, 0x08, 0x0C, 0x2C, 0x30] {
        m.w32(t + off, 0);
    }
    m.w32(t + 0x28, u32::MAX);
}

/// Libera las dos tablas de columnas y deja el objeto vacío (cuerpo común de `0x40A480`, `0x40A500` y `0x40ACE0`).
fn release(m: &mut dyn Mem, h: &mut dyn Host, t: u32) {
    let p = m.r32(t + 0x1C);
    if p != 0 {
        free(m, h, p);
    }
    let p = m.r32(t + 0x20);
    if p != 0 {
        free(m, h, p);
    }
}

/// Posición horizontal tras `col` columnas completas (ancho + separación), partiendo de `x`.
fn advance(m: &mut dyn Mem, t: u32, mut x: u32, col: i32) -> u32 {
    let (w, s) = (m.r32(t + 0x1C), m.r32(t + 0x20));
    for k in 0..col.max(0) as u32 {
        x = x.wrapping_add(m.r32(w.wrapping_add(k * 4)));
        x = x.wrapping_add(m.r32(s.wrapping_add(k * 4)));
    }
    x
}

pub fn run(f: u32, m: &mut dyn Mem, h: &mut dyn Host, t: u32, a: &[u32], _esp: u32) -> Option<Ret> {
    let r = match f {
        0x40A440 => {
            m.w32(t, VTABLE);
            zero_fields(m, t);
            m.w32(t + 0x28, u32::MAX);
            Ret::U32(t)
        }
        0x40A480 => {
            m.w32(t, VTABLE);
            release(m, h, t);
            zero_fields(m, t);
            Ret::None
        }
        0x40A4E0 => {
            let x = m.r32(a[0]);
            m.w32(t + 4, x);
            let y = m.r32(a[0] + 4);
            m.w32(t + 8, y);
            m.w32(t + 0xC, 0);
            Ret::None
        }
        0x40A500 => {
            release(m, h, t);
            zero_fields(m, t);
            Ret::None
        }
        0x40A8F0 => Ret::U32(m.r32(t + 0x14)),
        0x40A900 => {
            let n = a[0];
            m.w32(t + 0x14, n);
            let rows = m.r32(t + 0x24).wrapping_mul(n.wrapping_sub(1));
            let total = rows.wrapping_add(m.r32(t + 0x10).wrapping_mul(n));
            m.w32(t + 0x38, total);
            if m.r32(t + 0x28) != u32::MAX {
                let v = m.r32(t + 0x30).wrapping_add(total);
                m.w32(t + 0x38, v);
            }
            Ret::U32(total)
        }
        0x40A940 | 0x40A960 => {
            let i = a[0] as i32;
            if i >= m.r32(t + 0x18) as i32 {
                Ret::U32(0)
            } else {
                let tab = m.r32(t + if f == 0x40A940 { 0x1C } else { 0x20 });
                Ret::U32(m.r32(tab.wrapping_add(a[0].wrapping_mul(4))))
            }
        }
        0x40A980 => Ret::U32(m.r32(t + 0x18)),
        0x40A990 | 0x40AA40 => {
            let (col, row, out) = (a[0] as i32, a[1] as i32, a[2]);
            if col >= m.r32(t + 0x18) as i32 || row >= m.r32(t + 0x14) as i32 {
                return Some(Ret::U8(0));
            }
            let x0 = m.r32(t + 4);
            if f == 0x40A990 {
                m.w32(out, x0);
                let x = advance(m, t, x0, col);
                m.w32(out, x);
                let mut y = m.r32(t + 8);
                m.w32(out + 4, y);
                y = y.wrapping_add(m.r32(t + 0x10).wrapping_mul(row as u32));
                y = y.wrapping_add(m.r32(t + 0x24).wrapping_mul(row as u32));
                m.w32(out + 4, y);
                if m.r32(t + 0x28) != u32::MAX {
                    y = y.wrapping_add(m.r32(t + 0x30));
                    m.w32(out + 4, y);
                }
                let w = m.r32(t + 0x1C);
                let cw = m.r32(w.wrapping_add(a[0].wrapping_mul(4)));
                m.w32(out + 8, cw.wrapping_add(x));
                let hgt = m.r32(out + 4).wrapping_add(m.r32(t + 0x10));
                m.w32(out + 0xC, hgt);
            } else {
                m.w32(out + 8, 0);
                m.w32(out, x0);
                let x = advance(m, t, x0, col);
                m.w32(out, x);
                let step = m.r32(t + 0x24).wrapping_add(m.r32(t + 0x10)).wrapping_mul(row as u32);
                let mut y = m.r32(t + 8).wrapping_add(step);
                m.w32(out + 4, y);
                if m.r32(t + 0x28) != u32::MAX {
                    y = y.wrapping_add(m.r32(t + 0x30));
                    m.w32(out + 4, y);
                }
            }
            Ret::U8(1)
        }
        0x40AAD0 => {
            let (pt, out_col, out_row) = (a[0], a[1], a[2]);
            m.w32(out_col, u32::MAX);
            m.w32(out_row, u32::MAX);
            let x0 = m.r32(t + 4) as i32;
            let px = m.r32(pt) as i32;
            if px < x0 {
                return Some(Ret::U8(0));
            }
            let y0 = m.r32(t + 8) as i32;
            let py = m.r32(pt + 4) as i32;
            if py < (m.r32(t + 0x30) as i32).wrapping_add(y0) {
                return Some(Ret::U8(0));
            }
            if px > (m.r32(t + 0x34) as i32).wrapping_add(x0) || py > (m.r32(t + 0x38) as i32).wrapping_add(y0) {
                return Some(Ret::U8(0));
            }
            let cols = m.r32(t + 0x18) as i32;
            let (w, s) = (m.r32(t + 0x1C), m.r32(t + 0x20));
            let mut x = x0;
            let mut i = 0i32;
            while i < cols {
                x = x.wrapping_add(m.r32(w.wrapping_add(i as u32 * 4)) as i32);
                if x > px {
                    m.w32(out_col, i as u32);
                    break;
                }
                x = x.wrapping_add(m.r32(s.wrapping_add(i as u32 * 4)) as i32);
                if x > px {
                    break;
                }
                i += 1;
            }
            let mut y = m.r32(t + 8) as i32;
            if m.r32(t + 0x28) != u32::MAX {
                y = y.wrapping_add(m.r32(t + 0x30) as i32);
            }
            let py = m.r32(pt + 4) as i32;
            if y > py {
                return Some(Ret::U8(1));
            }
            let rows = m.r32(t + 0x14) as i32;
            let rh = m.r32(t + 0x10) as i32;
            let mut j = 0;
            while j < rows {
                y = y.wrapping_add(rh);
                if y > py {
                    m.w32(out_row, j as u32);
                    break;
                }
                y = y.wrapping_add(m.r32(t + 0x24) as i32);
                if y > py {
                    break;
                }
                j += 1;
            }
            Ret::U8(1)
        }
        0x40ABC0 => Ret::U32(m.r32(t + 0x34)),
        0x40ABD0 => Ret::U32(m.r32(t + 0x38)),
        0x40ABE0 => Ret::U32(m.r32(t + 0x24)),
        0x40ABF0 => Ret::U32(m.r32(t + 0x10)),
        0x40AC00 => Ret::U32(m.r32(t + 0x30)),
        0x40AC10 => {
            let out = a[0];
            let y = m.r32(t + 8);
            m.w32(out + 4, y);
            let y2 = m.r32(t + 0x38).wrapping_add(y);
            m.w32(out + 0xC, y2);
            let x = m.r32(t + 4);
            m.w32(out, x);
            let x2 = m.r32(t + 0x34).wrapping_add(x);
            m.w32(out + 8, x2);
            Ret::None
        }
        0x40AC40 => {
            let out = a[0];
            for k in 0..3 {
                let v = m.r32(t + 4 + 4 * k);
                m.w32(out + 4 * k, v);
            }
            Ret::None
        }
        0x40AC60 => {
            let (col, out) = (a[0] as i32, a[1]);
            if col >= m.r32(t + 0x18) as i32 {
                return Some(Ret::U8(0));
            }
            for k in 0..3 {
                let v = m.r32(t + 4 + 4 * k);
                m.w32(out + 4 * k, v);
            }
            let x = m.r32(out);
            let x = advance(m, t, x, col);
            m.w32(out, x);
            Ret::U8(1)
        }
        0x40ACE0 => {
            m.w32(t, VTABLE);
            release(m, h, t);
            zero_fields(m, t);
            if a[0] & 1 != 0 {
                free(m, h, t);
            }
            Ret::U32(t)
        }
        _ => return None,
    };
    Some(r)
}
