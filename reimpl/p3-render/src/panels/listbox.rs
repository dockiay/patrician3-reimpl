//! `ViperListBox` (`0x4CAEA0`–`0x4CE820`): lista con columnas, celdas-objeto y barra de desplazamiento SGL embebida en `+0xB4`.
//!
//! Disposición: `+0x98` filas (puntero a punteros de fila; cada fila es un puntero a celdas), `+0x9C` número de filas, `+0xA0` cabecera,
//! `+0xA4` columnas, `+0xAC` anchos de columna, `+0xB4` barra (subobjeto `SGL2DScrollbar`), `+0x208` fila seleccionada,
//! `+0x228` barra visible, `+0x230` primera fila visible, `+0x23C/+0x240` posición de la barra.

use super::{call, Host, Mem, Ret};
use crate::j07::vcall;

pub fn run(f: u32, m: &mut dyn Mem, h: &mut dyn Host, t: u32, a: &[u32], esp: u32) -> Option<Ret> {
    let r = match f {
        0x4CC190 => {
            let pt = a[0];
            let buf = esp.wrapping_sub(0xC);
            let (x, y) = (m.r32(pt), m.r32(pt + 4));
            m.w32(buf + 8, 0);
            m.w32(buf + 4, y);
            m.w32(buf, x);
            Ret::U32(vcall(m, h, t + 0xB4, 0x11C, &[buf]))
        }
        0x4CD920 => {
            let out = a[0];
            m.w32(out + 8, 0);
            m.w32(out + 4, 0);
            m.w32(out, 0);
            let head = m.r32(t + 0xA0);
            if head == 0 {
                return Some(Ret::None);
            }
            let c = m.r32(head);
            if c == 0 {
                return Some(Ret::None);
            }
            vcall(m, h, c, 0x40, &[out]);
            m.w32(out, 0);
            let widths = m.r32(t + 0xAC);
            let mut i = 0i32;
            while i < m.r32(t + 0xA4) as i32 {
                let w = m.r32(widths.wrapping_add(i as u32 * 4));
                let v = m.r32(out).wrapping_add(w);
                m.w32(out, v);
                i += 1;
            }
            let rows = m.r32(t + 0x98);
            if rows == 0 {
                return Some(Ret::None);
            }
            let o = m.r32(rows);
            if o == 0 {
                return Some(Ret::None);
            }
            let c2 = m.r32(o);
            if c2 == 0 {
                return Some(Ret::None);
            }
            let v = vcall(m, h, c2, 0x48, &[]);
            m.w32(out + 4, v);
            Ret::None
        }
        0x4CD9B0 => {
            let out = a[0];
            call(m, h, 0x4CD920, t, &[out], esp.wrapping_sub(0x10));
            if m.r8(t + 0x228) != 0 {
                let w = vcall(m, h, t + 0xB4, 0x44, &[]);
                let v = m.r32(out).wrapping_sub(w);
                m.w32(out, v);
            }
            Ret::None
        }
        0x4CD9E0 => Ret::U32(m.r32(t + 0x208)),
        0x4CE460 => {
            let page = call(m, h, 0x4BA040, t + 0xB4, &[], esp.wrapping_sub(8));
            let first = m.r32(t + 0x230);
            let rows = m.r32(t + 0x9C) as i32;
            let end = page.wrapping_add(first) as i32;
            Ret::U32(if end > rows { rows.wrapping_sub(1) } else { end.wrapping_sub(1) } as u32)
        }
        0x4CE020 => {
            let height = vcall(m, h, t, 0x48, &[]) as i32;
            let mut v = a[0] as i32;
            if v > height || v < 0 {
                v = 0;
            }
            let w = vcall(m, h, t, 0x44, &[]);
            let bar = t + 0xB4;
            let bw = vcall(m, h, bar, 0x44, &[]);
            let pos = t + 0x23C;
            m.w32(pos, w.wrapping_sub(bw).wrapping_sub(1));
            m.w32(t + 0x240, if v > 0 { v.wrapping_sub(2) as u32 } else { 0 });
            Ret::U32(vcall(m, h, bar, 0x68, &[pos]))
        }
        0x4CE100 => {
            let row = a[0] as i32;
            let rows = m.r32(t + 0x9C) as i32;
            if row >= rows {
                return Some(Ret::U8(0));
            }
            let (start, end) = if row == -1 { (0, rows) } else { (row, row + 1) };
            let color = a[1];
            let mut r = start;
            while r < end {
                let cols = m.r32(t + 0xA4) as i32;
                let mut c = 0i32;
                while c < cols {
                    let table = m.r32(t + 0x98);
                    let rp = m.r32(table.wrapping_add(r as u32 * 4));
                    if rp != 0 {
                        let cell = m.r32(rp.wrapping_add(c as u32 * 4));
                        if cell != 0 {
                            h.call(m, 0x4D2D80, cell, &[color]);
                            let table = m.r32(t + 0x98);
                            let rp = m.r32(table.wrapping_add(r as u32 * 4));
                            let cell = m.r32(rp.wrapping_add(c as u32 * 4));
                            vcall(m, h, cell, 0xD4, &[]);
                        }
                    }
                    c += 1;
                    if c >= m.r32(t + 0xA4) as i32 {
                        break;
                    }
                }
                r += 1;
            }
            Ret::U8(1)
        }
        0x4CD5C0 => {
            let mut ok = 1u8;
            let mut col = 0i32;
            while (m.r32(t + 0xA4) as i32) > 0 && ok != 0 {
                let mut row = 0i32;
                while (m.r32(t + 0x9C) as i32) > 0 && ok != 0 {
                    ok = call(m, h, 0x4CD640, t, &[row as u32, col as u32, a[0], a[1]], esp.wrapping_sub(36)) as u8;
                    row += 1;
                    if row >= m.r32(t + 0x9C) as i32 {
                        break;
                    }
                }
                col += 1;
                if col >= m.r32(t + 0xA4) as i32 {
                    break;
                }
            }
            vcall(m, h, t, 0xD4, &[]);
            Ret::U8(ok)
        }
        _ => return None,
    };
    Some(r)
}
