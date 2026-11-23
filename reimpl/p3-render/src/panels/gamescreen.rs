//! `GameScreen` (`0x422FC0`–`0x42DB50`): barra superior de la pantalla de juego (dinero, texto, contadores y rótulos).
//!
//! Disposición usada aquí: `+0x1230` etiqueta del dinero, `+0x1238` etiqueta de texto, `+0x123C` texto del dinero, `+0x1260/+0x1264`
//! desplazamientos del texto, `+0x2608` índice de la fuente (los objetos de fuente están en `0x6DCD28 + 168 * índice`),
//! `+0x488` y `+0x7B4` contadores de las dos líneas de aviso (sucesos y cartas), `+0x7D0`/`+0x7E8` sus temporizadores, `+0x494` las líneas de
//! carta (`0xA0` bytes cada una, con vtable; `+0xCC` activar).

use super::{Host, Mem, Ret};
use crate::j07::vcall;

const FONTS: u32 = 0x006D_CD28;
const DIRTY: u32 = 0x006D_A5F0;

fn font(idx: u32) -> u32 {
    FONTS.wrapping_add(idx.wrapping_mul(168))
}

pub fn run(f: u32, m: &mut dyn Mem, h: &mut dyn Host, t: u32, a: &[u32], esp: u32) -> Option<Ret> {
    let r = match f {
        0x428C70 => {
            let sp = esp.wrapping_sub(0x2C);
            let obj = m.r32(t + 0x1230);
            let w = vcall(m, h, obj, 0x88, &[]);
            let y = vcall(m, h, obj, 0x6C, &[]);
            let x = w.wrapping_add(y).wrapping_sub(2);
            let wd = vcall(m, h, obj, 0x44, &[]);
            m.w32(sp + 0x1C, x.wrapping_add(wd));
            let obj = m.r32(t + 0x1230);
            let ht = vcall(m, h, obj, 0x70, &[]);
            let off = m.r32(t + 0x1264);
            m.w32(sp + 0x20, ht.wrapping_add(off));
            let e = font(m.r32(t + 0x2608));
            vcall(m, h, e, 0x40, &[sp + 0x10]);
            let c = m.r32(sp + 0x20);
            let ry = m.r32(sp + 0x14);
            m.w32(sp + 0x28, ry.wrapping_add(c));
            let e = font(m.r32(t + 0x2608));
            let r = h.call(m, 0x4C1B20, e, &[t + 0x123C]);
            let x = m.r32(sp + 0x1C);
            m.w32(sp + 0x24, r.wrapping_add(x));
            h.call(m, 0x4B9650, DIRTY, &[sp + 0x1C]);
            Ret::None
        }
        0x428D40 => {
            let sp = esp.wrapping_sub(0x2C);
            let obj = m.r32(t + 0x1238);
            vcall(m, h, obj, 0x84, &[sp + 4]);
            let obj = m.r32(t + 0x1238);
            let hgt = vcall(m, h, obj, 0x6C, &[]);
            let x = m.r32(sp + 4);
            m.w32(sp + 0x24, hgt.wrapping_add(x).wrapping_add(2));
            let e = font(m.r32(t + 0x2608));
            let arg = m.r32(t + 0x1260);
            let w = h.call(m, 0x4C1B20, e, &[arg]);
            let top = m.r32(sp + 0x24).wrapping_sub(w);
            let off = m.r32(t + 0x1264);
            let ry = m.r32(sp + 8);
            m.w32(sp + 0x1C, top);
            m.w32(sp + 0x20, off.wrapping_add(ry));
            let e = font(m.r32(t + 0x2608));
            vcall(m, h, e, 0x40, &[sp + 0x10]);
            let y1 = m.r32(sp + 0x20);
            let y0 = m.r32(sp + 0x14);
            m.w32(sp + 0x28, y0.wrapping_add(y1));
            h.call(m, 0x4B9650, DIRTY, &[sp + 0x1C]);
            Ret::None
        }
        0x42BA60 => {
            let d = a[0];
            let mut i = 0u32;
            if m.r8(t + 0x488) != 0 {
                loop {
                    let v = m.r32(t + 0x7D0 + 4 * i).wrapping_add(d);
                    m.w32(t + 0x7D0 + 4 * i, v);
                    i += 1;
                    if (i as i32) >= i32::from(m.r8(t + 0x488)) {
                        break;
                    }
                }
            }
            i = 0;
            if m.r8(t + 0x7B4) != 0 {
                loop {
                    let v = m.r32(t + 0x7E8 + 4 * i).wrapping_add(d);
                    m.w32(t + 0x7E8 + 4 * i, v);
                    i += 1;
                    if (i as i32) >= i32::from(m.r8(t + 0x7B4)) {
                        break;
                    }
                }
            }
            Ret::None
        }
        0x42C020 => {
            let o = m.r32(0x006E_51AC);
            vcall(m, h, o, 0x44, &[]);
            let n = m.r8(t + 0x7B4);
            if n == 0 {
                return Some(Ret::None);
            }
            let n = n - 1;
            m.w8(t + 0x7B4, n);
            let line = t.wrapping_add(u32::from(n) * 160).wrapping_add(0x494);
            vcall(m, h, line, 0xCC, &[0]);
            let n = u32::from(m.r8(t + 0x7B4));
            m.w32(t + n * 4 + 0x7E8, 0);
            Ret::None
        }
        0x42B920 | 0x42BE40 => {
            let letter = f == 0x42BE40;
            let (cnt_o, tim_o, lines_o, r1, r2) = if letter { (0x7B4, 0x7E8, 0x494, 0x7B8, 0x7BC) } else { (0x488, 0x7D0, 0x168, 0x48C, 0x490) };
            let mut w0 = 0;
            if letter {
                let o = m.r32(0x006E_51AC);
                w0 = vcall(m, h, o, 0x44, &[]);
            }
            if m.r8(t + cnt_o) == 0 || m.r32(t + tim_o) >= m.r32(0x006D_CCF0) {
                return Some(Ret::None);
            }
            let mut esi = t + lines_o;
            let v = vcall(m, h, esi, 0x6C, &[]);
            m.w32(t + r1, v);
            let hh = vcall(m, h, esi, 0x6C, &[]);
            let w = vcall(m, h, esi, 0x44, &[]);
            m.w32(t + r2, hh.wrapping_add(w));
            let cnt = m.r8(t + cnt_o).wrapping_sub(1);
            m.w8(t + cnt_o, cnt);
            if cnt != 0 {
                let mut i = 0i32;
                loop {
                    vcall(m, h, esi, 0xD4, &[]);
                    let a = m.r32(esi + 0x13C);
                    let fa = font(m.r32(esi + 0x98));
                    let vt = m.r32(esi);
                    let wd = h.call(m, 0x4C1B20, fa, &[a]);
                    let tgt = m.r32(vt + 0x58);
                    h.call(m, tgt, esi, &[wd.wrapping_add(0x10)]);
                    if letter {
                        let r = vcall(m, h, esi, 0x70, &[0x64]);
                        let d = vcall(m, h, esi, 0x44, &[r]);
                        vcall(m, h, esi, 0x64, &[w0.wrapping_sub(d)]);
                    }
                    let a = m.r32(esi + 0x13C);
                    h.call(m, 0x64F390, esi + 0x9C, &[a]);
                    vcall(m, h, esi, 0xD4, &[]);
                    i += 1;
                    let c = i32::from(m.r8(t + cnt_o));
                    esi = esi.wrapping_add(0xA0);
                    if i >= c {
                        break;
                    }
                }
            }
            let n = u32::from(m.r8(t + cnt_o)) * 4;
            h.call(m, 0x63A790, crate::j07::CDECL, &[t + tim_o, t + tim_o + 4, n]);
            let c = u32::from(m.r8(t + cnt_o));
            m.w32(t + c * 4 + tim_o, 0);
            let c = u32::from(m.r8(t + cnt_o));
            vcall(m, h, t.wrapping_add(c * 160).wrapping_add(lines_o), 0xCC, &[0]);
            Ret::None
        }
        0x42D6D0 => {
            let (lu, lb) = (esp.wrapping_sub(4), esp.wrapping_sub(5));
            m.w32(lu, 0);
            m.w8(lb, 0);
            let obj = m.r32(0x006C_E6D0);
            let esi = h.call(m, 0x4885A0, obj, &[lu, lb, 0]);
            if esi != 0 {
                let edx = m.r32(0x006D_FC14);
                let e5 = h.call(m, 0x5303C0, 0x006D_E4A0, &[edx]);
                let a0 = a[0];
                let flag = u32::from(m.r8(0x006D_F2F0) & 0xC != 0 && (a0 as i32) < 10);
                let bl = m.r8(lb);
                let set = u32::from(m.r8(0x0066_F354) & bl != 0);
                let up = edx & 0xFFFF_FF00;
                let l = m.r32(lu);
                h.call(m, 0x4F5500, e5, &[a0, esi, l, set | up, flag | up]);
            }
            Ret::None
        }
        0x423720 => {
            m.w32(t + 0x124C, 0);
            m.w32(t + 0x164, 0);
            m.w32(t + 0x7C0, 0);
            let buf = t + 0x123C;
            h.call(m, 0x639F20, crate::j07::CDECL, &[buf, 0, 0x10]);
            let v = m.r32(t + 0x7C0);
            h.call(m, 0x63AB8A, crate::j07::CDECL, &[buf, 0x10, 0x0069_64AC, v]);
            for off in [0x260C, 0x2610, 0x2614, 0x2618] {
                m.w32(t + off, u32::MAX);
            }
            m.w32(t + 0x7C8, 0);
            m.w32(t + 0x7C4, 0);
            h.call(m, 0x64F390, t + 0x2630, &[0x0069_64B0]);
            h.call(m, 0x64F390, t + 0x262C, &[0x0069_64B2]);
            m.w32(t + 0x800, u32::MAX);
            m.w16(t + 0x804, 0xFFFF);
            m.w16(t + 0x806, 0xFFFF);
            m.w32(t + 0x1258, 0);
            Ret::None
        }
        _ => return None,
    };
    Some(r)
}
