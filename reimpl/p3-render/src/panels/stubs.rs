//! Ranuras triviales de las barras de desplazamiento y ventanas de lista, nombres de clase, `draw_all` y el recorte interior de `ShipListWindow`.

use super::{Host, Mem, Ret};
use crate::j07::{vcall, CDECL};

pub fn run(f: u32, m: &mut dyn Mem, h: &mut dyn Host, t: u32, a: &[u32], esp: u32) -> Option<Ret> {
    let r = match f {
        0x461C60 | 0x476780 => Ret::None,
        0x460D90 | 0x4B9A70 | 0x4B9A80 | 0x472C50 => {
            h.call(m, 0x64F253, esp.wrapping_add(4), &[]);
            Ret::U8(0)
        }
        0x4620B0 | 0x4BAB90 | 0x478540 => {
            let g = match f {
                0x4620B0 => 0x0069_85B4,
                0x4BAB90 => 0x006B_BC00,
                _ => 0x006B_9870,
            };
            let v = m.r32(g);
            m.w32(esp.wrapping_sub(4), 0);
            h.call(m, 0x64F2C1, a[0], &[v]);
            Ret::U32(a[0])
        }
        0x4749C0 => {
            let flag = m.r8(t + 0x94) != 0;
            let edi = if flag { m.r32(0x006C_BAEC) } else { 0 };
            let (ebx, ebp, esi) = (m.r32(t + 0x18), m.r32(t + 0x14), m.r32(0x006C_BB08));
            let (x2, y2) = (m.r32(t + 0x2C), m.r32(t + 0x30));
            let a3 = x2.wrapping_sub(esi).wrapping_sub(edi).wrapping_add(ebp);
            let a4 = y2.wrapping_sub(esi).wrapping_add(ebx);
            let a2 = ((esi as i32).wrapping_sub((esi as i32) >> 31) >> 1) as u32;
            let a2 = a2.wrapping_add(ebx);
            let a1 = ebp.wrapping_add(esi).wrapping_add(edi);
            h.call(m, 0x4BBAA0, t, &[a1, a2, a3, a4]);
            let _ = CDECL;
            Ret::None
        }
        0x461FE0 => {
            for off in [0u32, 0x94, 0x17C, 0x264] {
                vcall(m, h, t + off, 0x9C, &[a[0], a[1], a[2], a[3]]);
            }
            Ret::None
        }
        _ => return None,
    };
    Some(r)
}
