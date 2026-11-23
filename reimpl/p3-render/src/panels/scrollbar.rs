//! Barras de desplazamiento: `P2Scrollbar` (`0x460870`–`0x4620E0`) y `ListScrollbar` (`0x430F90`–`0x431330`).
//!
//! `P2Scrollbar` (0x380+ bytes): `+0x18` desplazamiento vertical del carril, `+0x2C/+0x30` ancho y alto, `+0x4C` destino de invalidación,
//! `+0x94` botón superior, `+0x17C` botón inferior, `+0x264` pulgar (subobjetos con vtable propia; `+0x48` ancho, `+0x60/+0x68` leer y fijar
//! posición, `+0x118` ocultar), `+0x360` visible, `+0x364` extensión total, `+0x368` carril, `+0x36C` posición, `+0x370` ventana visible,
//! `+0x378` última posición leída, `+0x37C` alto de línea, `+0x380` rectángulo (16 bytes).
//! `ListScrollbar` lleva la barra en `+8` y añade `+0x384` líneas visibles, `+0x398` activa, `+0x39C` filas, `+0x3A0` píxeles,
//! `+0x3A4` primera línea, `+0x3A8` última, `+0x3AC` resto, `+0x3B0` delta.

use super::{call, Host, Mem, Ret};
use crate::j07::vcall;

const IAT_INVALIDATE: u32 = 0x0066_A4CC;
const IAT_SET_RECT_EMPTY: u32 = 0x0066_A4D4;
const LIST_VTABLE: u32 = 0x0066_CFC8;

fn zero_state(m: &mut dyn Mem, t: u32) {
    m.w8(t + 0x360, 0);
    for off in [0x36C, 0x368, 0x364, 0x374, 0x370, 0x378, 0x35C] {
        m.w32(t + off, 0);
    }
}

/// Mueve el pulgar a la posición actual y lo invalida (cola común de `set_total_extent`, `set_position` y los desplazamientos).
/// `esp` es la pila de entrada de la función: la estructura local está en `esp - 0x10`.
fn place_thumb(m: &mut dyn Mem, h: &mut dyn Host, t: u32, esp: u32) {
    let thumb = t + 0x264;
    let w = vcall(m, h, thumb, 0x48, &[]);
    let d = m.r32(t + 0x368).wrapping_sub(w) as i32;
    let pos = m.r32(t + 0x36C) as i32;
    let ratio: f32 = if pos != 0 {
        let span = m.r32(t + 0x364).wrapping_sub(m.r32(t + 0x370)) as i32;
        pos as f32 / span as f32
    } else {
        0.0
    };
    let top = vcall(m, h, t + 0x94, 0x48, &[]);
    // x87: el producto se hace en precisión ampliada y `_ftol` trunca sin redondear antes a f32 (los dos factores son f32, producto exacto en f64).
    let off = p3_core::numeric::ftol_i32(f64::from(d as f32) * f64::from(ratio)) as u32;
    let y = top.wrapping_add(off).wrapping_add(m.r32(t + 0x18));
    let buf = esp.wrapping_sub(0x10);
    vcall(m, h, thumb, 0x60, &[buf]);
    m.w32(buf + 4, y);
    vcall(m, h, thumb, 0x68, &[buf]);
    let (cw, ch) = (m.r32(t + 0x2C), m.r32(t + 0x30));
    m.w32(buf, 0);
    m.w32(buf + 4, 0);
    m.w32(buf + 8, cw);
    m.w32(buf + 12, ch);
    let f = m.r32(IAT_INVALIDATE);
    h.call(m, f, crate::j07::CDECL, &[t + 0x4C, t + 0x4C, buf]);
}

fn half_line(h: i32) -> i32 {
    h.wrapping_sub(h >> 31) >> 1
}

pub fn run(f: u32, m: &mut dyn Mem, h: &mut dyn Host, t: u32, a: &[u32], esp: u32) -> Option<Ret> {
    let r = match f {
        0x461450 => {
            let pos = m.r32(t + 0x36C);
            m.w32(a[0], pos);
            let prev = m.r32(t + 0x378);
            m.w32(t + 0x378, pos);
            Ret::U8(u8::from(pos != prev))
        }
        0x461BD0 => {
            h.call(m, 0x4B1650, t, &[]);
            zero_state(m, t);
            vcall(m, h, t + 0x17C, 0x118, &[]);
            vcall(m, h, t + 0x264, 0x118, &[]);
            vcall(m, h, t + 0x94, 0x118, &[]);
            m.w32(t + 0x37C, 0);
            let f = m.r32(IAT_SET_RECT_EMPTY);
            h.call(m, f, crate::j07::CDECL, &[t + 0x380]);
            Ret::None
        }
        0x461C70 => {
            let (cw, ch) = (m.r32(t + 0x2C), m.r32(t + 0x30));
            let buf = esp.wrapping_sub(0x10);
            for (i, v) in [0, 0, cw, ch].into_iter().enumerate() {
                m.w32(buf + 4 * i as u32, v);
            }
            let f = m.r32(IAT_INVALIDATE);
            h.call(m, f, crate::j07::CDECL, &[t + 0x4C, t + 0x4C, buf]);
            Ret::None
        }
        0x461CA0 => {
            let ext = a[0] as i32;
            let view = m.r32(t + 0x370) as i32;
            m.w32(t + 0x364, ext as u32);
            m.w8(t + 0x360, u8::from(ext > view));
            let pos = m.r32(t + 0x36C) as i32;
            if pos.wrapping_add(view) > ext {
                let np = ext.wrapping_sub(view);
                m.w32(t + 0x36C, np as u32);
                if np < 0 {
                    m.w32(t + 0x36C, 0);
                }
            }
            place_thumb(m, h, t, esp);
            Ret::None
        }
        0x461DD0 => {
            let ext = m.r32(t + 0x364) as i32;
            let view = m.r32(t + 0x370) as i32;
            let mut p = a[0] as i32;
            let max = ext.wrapping_sub(view);
            if p > max {
                p = max;
            }
            if p < 0 {
                p = 0;
            }
            if m.r32(t + 0x36C) as i32 != p {
                m.w32(t + 0x36C, p as u32);
                place_thumb(m, h, t, esp);
            }
            Ret::None
        }
        0x461210 => {
            if m.r8(t + 0x360) == 0 {
                return Some(Ret::None);
            }
            let pos = m.r32(t + 0x36C) as i32;
            if pos == 0 {
                return Some(Ret::None);
            }
            let np = pos.wrapping_sub(a[0] as i32);
            m.w32(t + 0x36C, np as u32);
            if np < 0 {
                m.w32(t + 0x36C, 0);
            }
            place_thumb(m, h, t, esp);
            Ret::None
        }
        0x461320 => {
            if m.r8(t + 0x360) == 0 {
                return Some(Ret::None);
            }
            let max = (m.r32(t + 0x364) as i32).wrapping_sub(m.r32(t + 0x370) as i32);
            let pos = m.r32(t + 0x36C) as i32;
            if pos == max {
                return Some(Ret::None);
            }
            let np = pos.wrapping_add(a[0] as i32);
            m.w32(t + 0x36C, if np > max { max } else { np } as u32);
            place_thumb(m, h, t, esp);
            Ret::None
        }
        0x461480 => {
            let o = h.call(m, 0x4B9730, 0x006D_A5F0, &[]);
            for p in [t + 0x94, t + 0x17C, t + 0x264, t] {
                h.call(m, 0x4B4EB0, o, &[p]);
            }
            Ret::None
        }
        0x461FD0 => Ret::U32(vcall(m, h, t + 0x264, 0x44, &[])),
        0x461F80 => {
            let mut busy = call(m, h, 0x4C78B0, t + 0x17C, &[], esp) as u8 != 0;
            busy = busy || call(m, h, 0x4C78B0, t + 0x94, &[], esp) as u8 != 0;
            busy = busy || call(m, h, 0x4C78B0, t + 0x264, &[], esp) as u8 != 0;
            busy = busy || call(m, h, 0x4B1BF0, t + 0x264, &[], esp) as u8 != 0;
            Ret::U8(u8::from(busy || m.r32(t + 4) == 2))
        }
        0x4B2200 => Ret::U8(m.r8(t + 0x48)),
        // ListScrollbar
        0x430F90 => {
            h.call(m, 0x460870, t + 8, &[]);
            m.w32(t, LIST_VTABLE);
            m.w8(t + 0x398, 0);
            for off in [0x39C, 0x3A0, 0x3A4, 0x3A8, 0x3AC, 0x3B0] {
                m.w32(t + off, 0);
            }
            m.w8(t + 4, 1);
            Ret::U32(t)
        }
        0x431060 => {
            m.w8(t + 0x398, 0);
            for off in [0x39C, 0x3A0, 0x3A4, 0x3A8, 0x3AC, 0x3B0] {
                m.w32(t + off, 0);
            }
            Ret::U32(0)
        }
        0x431090 => {
            if m.r32(t + 0x3AC) == 0 {
                return Some(Ret::U8(0));
            }
            if call(m, h, 0x461F80, t + 8, &[], esp.wrapping_sub(0xC)) as u8 != 0 {
                return Some(Ret::U8(0));
            }
            let line = m.r32(t + 0x37C) as i32;
            let rest = m.r32(t + 0x3AC) as i32;
            if rest > half_line(line) {
                let v = m.r32(t + 0x3A4).wrapping_add(1);
                m.w32(t + 0x3A4, v);
            }
            let first = m.r32(t + 0x3A4) as i32;
            let last = m.r32(t + 0x384).wrapping_add(first as u32);
            let px = first.wrapping_mul(line) as u32;
            m.w32(t + 0x3A0, px);
            m.w32(t + 0x3A8, last);
            m.w32(t + 0x3AC, 0);
            call(m, h, 0x461DD0, t + 8, &[px], esp.wrapping_sub(0xC).wrapping_sub(8));
            Ret::U8(1)
        }
        0x431110 => {
            let total = m.r32(t + 0x36C) as i32;
            if total == 0 {
                m.w8(t + 0x398, 0);
                for off in [0x39C, 0x3A0, 0x3A4, 0x3AC, 0x3B0, 0x3A8] {
                    m.w32(t + off, 0);
                }
                return Some(Ret::U32(0));
            }
            let px = m.r32(t + 0x3A0) as i32;
            let line = m.r32(t + 0x37C) as i32;
            let first = px.wrapping_div(line);
            let rest = px.wrapping_sub(first.wrapping_mul(line));
            let rows = total.wrapping_div(line);
            let mut last = m.r32(t + 0x384).wrapping_add(first as u32);
            m.w32(t + 0x3A4, first as u32);
            m.w32(t + 0x3AC, rest as u32);
            m.w32(t + 0x3A8, last);
            m.w32(t + 0x39C, rows as u32);
            if rest != 0 {
                last = last.wrapping_add(1);
                m.w32(t + 0x3A8, last);
            }
            if (last as i32) > rows {
                m.w32(t + 0x3A8, rows as u32);
            }
            Ret::U32(rows as u32)
        }
        0x4311B0 => {
            let bar = t + 8;
            let n = a[0] as i32;
            let shown = call(m, h, 0x4B2200, bar, &[], esp.wrapping_sub(0x10)) as u8;
            if shown == 0 && m.r8(t + 0x398) == 1 {
                m.w8(t + 0x398, 0);
            }
            let al = m.r8(t + 0x398);
            let lines = m.r32(t + 0x384) as i32;
            let mut refresh = true;
            if al == 1 && n <= lines {
                vcall(m, h, bar, 0xCC, &[0]);
                m.w8(t + 0x398, 0);
            } else if al == 0 && n > lines {
                vcall(m, h, bar, 0xCC, &[1]);
                m.w8(t + 0x398, 1);
            } else if n == m.r32(t + 0x39C) as i32 {
                refresh = false;
            }
            if refresh {
                let line = m.r32(t + 0x37C);
                m.w32(t + 0x39C, n as u32);
                // esp del llamado: pushes de ebp, esi, edi y el argumento.
                let ce = esp.wrapping_sub(12).wrapping_sub(8);
                call(m, h, 0x461CA0, bar, &[line.wrapping_mul(n as u32)], ce);
                call(m, h, 0x461450, bar, &[t + 0x3A0], ce);
                call(m, h, 0x431110, t, &[], esp.wrapping_sub(12).wrapping_sub(4));
            }
            Ret::None
        }
        0x431260 => {
            if m.r8(t + 0x398) == 0 {
                return Some(Ret::U8(0));
            }
            let before = m.r32(t + 0x3A0);
            let pos = t + 0x3A0;
            let ce = esp.wrapping_sub(12).wrapping_sub(8);
            let changed = call(m, h, 0x461450, t + 8, &[pos], ce) as u8;
            let now = m.r32(pos);
            if changed != 0 && before != now {
                m.w32(t + 0x3B0, now.wrapping_sub(before));
                call(m, h, 0x431110, t, &[], esp.wrapping_sub(12).wrapping_sub(4));
                return Some(Ret::U8(1));
            }
            if m.r8(t + 4) == 0 || m.r32(t + 0x3AC) == 0 {
                return Some(Ret::U8(0));
            }
            if call(m, h, 0x461F80, t + 8, &[], esp.wrapping_sub(12).wrapping_sub(4)) as u8 != 0 {
                return Some(Ret::U8(0));
            }
            let line = m.r32(t + 0x37C) as i32;
            let rest = m.r32(t + 0x3AC) as i32;
            if rest > half_line(line) {
                let v = m.r32(t + 0x3A4).wrapping_add(1);
                m.w32(t + 0x3A4, v);
            }
            let first = m.r32(t + 0x3A4) as i32;
            let last = m.r32(t + 0x384).wrapping_add(first as u32);
            let px = first.wrapping_mul(line) as u32;
            m.w32(pos, px);
            m.w32(t + 0x3A8, last);
            m.w32(t + 0x3AC, 0);
            call(m, h, 0x461DD0, t + 8, &[px], esp.wrapping_sub(16).wrapping_sub(8));
            let now = m.r32(pos);
            m.w32(t + 0x3B0, now.wrapping_sub(before));
            Ret::U8(1)
        }

        // SGL2DScrollbar (`0x4B9840`–`0x4BACD0`): `+0x94` rango, `+0x98` posición, `+0x9C` página, `+0xA0` tope, `+0xA4` grosor,
        // `+0xF0` paso automático, `+0xF4/+0xF6/+0xF8` frecuencia, activo y cuenta atrás, `+0x148` objeto de aviso.
        0x4B9930 => {
            h.call(m, 0x4B1650, t, &[]);
            for (off, v) in [(0x94, 0), (0x98, 0), (0x9C, 1), (0xA0, 0), (0xA4, 1)] {
                m.w32(t + off, v);
            }
            m.w8(t + 0x12D, 0);
            m.w32(t + 0x130, 0xFF80_8080);
            m.w32(t + 0x134, u32::MAX);
            let mut bp = 0u32;
            let mut first = true;
            for (mask, k) in [(0xF000, 1), (0xF00, 2), (0xF0, 1), (0xF, 2)] {
                let r = h.call(m, 0x4BBDB0, crate::j07::CDECL, &[mask, k, 0]);
                bp = if first { r & 0xFFFF } else { bp | r };
                first = false;
            }
            for off in [0xA8, 0xB4, 0xAC, 0xB8, 0xB0, 0xBC, 0xCC, 0xD0] {
                m.w32(t + off, u32::MAX);
            }
            m.w16(t + 0x138, bp as u16);
            for off in [0xE4, 0xE8, 0xEC] {
                m.w32(t + off, 0);
            }
            m.w32(t + 0xD8, 0x14);
            m.w32(t + 0xDC, 0x14);
            for off in [0xE0, 0x114, 0x118, 0x11C, 0xC0, 0xC4, 0xC8] {
                m.w32(t + off, 0);
            }
            m.w16(t + 0xF4, 0xA);
            m.w8(t + 0xF6, 0);
            m.w16(t + 0xF8, 0x64);
            m.w32(t + 0xD4, 0);
            m.w32(t + 0x148, 0);
            m.w8(t + 0x12C, 0);
            Ret::None
        }
        0x4B9DD0 => {
            let step = m.r32(t + 0xF0);
            if step == 0 || m.r8(t + 0xF6) == 0 {
                return Some(Ret::None);
            }
            let dec = m.r16(0x006D_CCF4);
            let cd = m.r16(t + 0xF8).wrapping_sub(dec);
            m.w16(t + 0xF8, cd);
            if (cd as i16) >= 0 {
                return Some(Ret::None);
            }
            let rate = i32::from(m.r16(t + 0xF4) as i16);
            let q = 1000i32.wrapping_div(rate);
            let old = m.r32(t + 0x98);
            m.w16(t + 0xF8, q as u16);
            let np = old.wrapping_add(step) as i32;
            if np < 0 || np > m.r32(t + 0x94) as i32 {
                return Some(Ret::None);
            }
            m.w32(t + 0x98, np as u32);
            call(m, h, 0x4BA810, t, &[], esp.wrapping_sub(12));
            let obj = m.r32(t + 0x148);
            if obj == 0 || old == m.r32(t + 0x98) {
                return Some(Ret::None);
            }
            let vt = m.r32(obj);
            let f = m.r32(vt);
            Ret::U32(h.call(m, f, obj, &[]))
        }
        0x4B9E60 => {
            let n = a[0] as i32;
            if n < 0 {
                return Some(Ret::U8(0));
            }
            let page = m.r32(t + 0x9C) as i32;
            let eax = n.wrapping_sub(page);
            m.w32(t + 0x94, n as u32);
            let top = if n > eax {
                if eax <= 0 { 0 } else { eax }
            } else if eax > 0 {
                n
            } else {
                0
            };
            m.w32(t + 0xA0, top as u32);
            call(m, h, 0x4BA810, t, &[], esp.wrapping_sub(8));
            call(m, h, 0x4BA810, t, &[], esp.wrapping_sub(8));
            Ret::U8(1)
        }
        0x4B9EC0 => Ret::U32(m.r32(t + 0x98)),
        0x4B9F90 => {
            let p = a[0] as i32;
            if p < 0 || p > m.r32(t + 0x94) as i32 {
                return Some(Ret::U8(0));
            }
            let old = m.r32(t + 0x98);
            m.w32(t + 0x98, p as u32);
            call(m, h, 0x4BA810, t, &[], esp.wrapping_sub(12));
            let obj = m.r32(t + 0x148);
            if obj != 0 && old != m.r32(t + 0x98) {
                let vt = m.r32(obj);
                let f = m.r32(vt);
                h.call(m, f, obj, &[]);
            }
            Ret::U8(1)
        }
        0x4B9FE0 => {
            let max = vcall(m, h, t, 0x44, &[]) as i32;
            if (a[0] as i32) > max {
                return Some(Ret::U8(0));
            }
            m.w32(t + 0xA4, a[0]);
            Ret::U8(1)
        }
        0x4BA020 => {
            if (m.r32(t + 0x9C) as i32) < 0 {
                return Some(Ret::U8(0));
            }
            m.w32(t + 0x9C, a[0]);
            Ret::U8(1)
        }
        0x4BA040 => Ret::U32(m.r32(t + 0x9C)),
        0x4BA6E0 | 0x4BA710 => {
            let off = if f == 0x4BA6E0 { 0x130 } else { 0x134 };
            let old = m.r32(t + off);
            m.w32(t + off, a[0]);
            Ret::U32(old)
        }
        0x4BA810 => {
            let height = vcall(m, h, t, 0x48, &[]);
            let e8 = m.r32(t + 0xE8);
            let dc = m.r32(t + 0xDC);
            let mut len = height.wrapping_add(e8.wrapping_neg().wrapping_add(e8.wrapping_neg()).wrapping_sub(dc)) as i32;
            if len < 0 {
                len = 0;
            }
            let width = vcall(m, h, t, 0x44, &[]) as i32;
            let (e4, e8) = (m.r32(t + 0xE4) as i32, m.r32(t + 0xE8) as i32);
            let cx = half_line(width.wrapping_sub(e4));
            m.w32(t + 0x100, 0);
            m.w32(t + 0xFC, cx as u32);
            m.w32(t + 0x108, cx as u32);
            let height2 = vcall(m, h, t, 0x48, &[]) as i32;
            let y0 = m.r32(t + 0x100) as i32;
            m.w32(t + 0x114, cx as u32);
            m.w32(t + 0x10C, height2.wrapping_sub(e8) as u32);
            m.w32(t + 0x118, y0.wrapping_add(e8) as u32);
            let width2 = vcall(m, h, t, 0x44, &[]) as i32;
            let dw = m.r32(t + 0xD8) as i32;
            let range = m.r32(t + 0x94) as i32;
            m.w32(t + 0x120, half_line(width2.wrapping_sub(dw)) as u32);
            let thumb = if range > 0 {
                let pos = m.r32(t + 0x98) as i32;
                pos.wrapping_mul(len).wrapping_div(range).wrapping_add(e8)
            } else {
                e8
            };
            m.w32(t + 0x124, thumb as u32);
            Ret::U32(vcall(m, h, t, 0xD4, &[]))
        }
        _ => return None,
    };
    Some(r)
}
