//! Marco de ventanas (`Window`, `ScreenWindow`, `ScreenList`, `ParchmentBase`, `ControlValue`): accesores, destructores escalares,
//! envolturas y llamadas virtuales pequeñas.
//!
//! Disposición de `Window` (medida en el ejecutable): `+0` vtable, `+4` modo, `+8` modo anterior, `+0xC` bandera de sucio, `+0x10`
//! contador, `+0x14` posición (3 enteros), `+0x20` desplazamiento (3), `+0x2C` tamaño (3), `+0x38` registro de valor, `+0x3C` índice de
//! usuario, `+0x41` habilitada, `+0x42`/`+0x43`/`+0x48`/`+0x4A`/`+0x4B` banderas, `+0x44` banderas de estado, `+0x4C` rectángulo sucio,
//! `+0x5C` contenido, `+0x70` sonidos por modo, `+0x80` cuatro identificadores de ayuda, `+0x90` byte.

use super::{Host, Mem, Ret};
use crate::j07::{cdecl, free, vcall};

const CSTRING_DTOR: u32 = 0x0064_F253;
const CSTRING_CTOR: u32 = 0x0064_F2C1;

/// Copia tres enteros de `src` a `dst`.
fn copy3(m: &mut dyn Mem, dst: u32, src: u32) {
    for k in 0..3 {
        let v = m.r32(src + 4 * k);
        m.w32(dst + 4 * k, v);
    }
}

fn copy4(m: &mut dyn Mem, dst: u32, src: u32) {
    for k in 0..4 {
        let v = m.r32(src + 4 * k);
        m.w32(dst + 4 * k, v);
    }
}

/// `jmp [iat]`/`call [iat]` stdcall: llama a la función cuyo puntero está en `iat`.
fn import(m: &mut dyn Mem, h: &mut dyn Host, iat: u32, args: &[u32]) -> u32 {
    let f = m.r32(iat);
    cdecl(m, h, f, args)
}

/// Destructor escalar: llama al destructor `dtor` y libera el objeto si el bit 0 del argumento está activo.
fn scalar_dtor(m: &mut dyn Mem, h: &mut dyn Host, this: u32, flags: u32, dtor: u32) -> Ret {
    h.call(m, dtor, this, &[]);
    if flags & 1 != 0 {
        free(m, h, this);
    }
    Ret::U32(this)
}

/// Nombre de clase: construye en `out` una cadena a partir del literal guardado en `global`.
fn class_name(m: &mut dyn Mem, h: &mut dyn Host, out: u32, global: u32) -> Ret {
    let s = m.r32(global);
    h.call(m, CSTRING_CTOR, out, &[s]);
    Ret::U32(out)
}

/// Llamada virtual con una estructura de tres enteros local (`esp - 0xC`) que la ranura rellena; devuelve la palabra `word`.
fn get_triple(m: &mut dyn Mem, h: &mut dyn Host, this: u32, esp: u32, slot: u32, word: u32) -> Ret {
    let buf = esp.wrapping_sub(0xC);
    vcall(m, h, this, slot, &[buf]);
    Ret::U32(m.r32(buf + 4 * word))
}

pub fn run(f: u32, m: &mut dyn Mem, h: &mut dyn Host, ecx: u32, a: &[u32], esp: u32) -> Option<Ret> {
    let r = match f {
        // Accesores de dword.
        0x4023D0 => Ret::U32(m.r32(ecx + 4)),
        0x402450 => Ret::U32(m.r32(ecx + 8)),
        0x402640 => Ret::U32(m.r32(ecx + 0x20)),
        0x402650 => Ret::U32(m.r32(ecx + 0x24)),
        0x4B1860 => Ret::U32(m.r32(ecx + 0x3C)),
        0x4B18B0 => Ret::U32(m.r32(ecx + 0x38)),
        0x4B1AC0 => Ret::U32(m.r32(ecx + 0x44)),
        0x4B1CB0 => Ret::U32(m.r32(ecx + 0x60)),
        // Accesores de byte.
        0x402460 => Ret::U8(m.r8(ecx + 0x41)),
        0x402680 => Ret::U8(m.r8(ecx + 0x4A)),
        0x4026E0 => Ret::U8(m.r8(ecx + 0x42)),
        0x4026A0 => Ret::U8(m.r8(ecx + 0x90)),
        0x4B1BF0 => Ret::U8(m.r8(ecx + 0x4B)),
        0x4B2200 => Ret::U8(m.r8(ecx + 0x48)),
        0x4B2220 => Ret::U8(m.r8(ecx + 0xC)),
        0x4B23A0 => Ret::U8(m.r8(ecx + 0x43)),
        0x4B8510 => Ret::U8(m.r8(ecx + 0x120)),
        // Copias de tres enteros hacia el argumento.
        0x402470 | 0x402530 | 0x402620 => {
            let src = match f {
                0x402470 => 0x2C,
                0x402530 => 0x14,
                _ => 0x20,
            };
            copy3(m, a[0], ecx + src);
            Ret::U32(a[0])
        }
        0x402490 => {
            let (w, hh) = (m.r32(ecx + 0x2C), m.r32(ecx + 0x30));
            m.w32(a[0] + 4, 0);
            m.w32(a[0], 0);
            m.w32(a[0] + 0xC, hh);
            m.w32(a[0] + 8, w);
            Ret::U32(a[0])
        }
        0x4024B0 => {
            m.w32(ecx + 0x2C, a[0]);
            m.w32(ecx + 0x34, a[2]);
            m.w32(ecx + 0x30, a[1]);
            Ret::U8(1)
        }
        0x4024D0 | 0x402500 => {
            let buf = esp.wrapping_sub(0xC);
            vcall(m, h, ecx, 0x40, &[buf]);
            m.w32(buf + if f == 0x4024D0 { 0 } else { 4 }, a[0]);
            Ret::U32(vcall(m, h, ecx, 0x54, &[buf]))
        }
        0x402550 => {
            m.w32(ecx + 0x14, a[0]);
            m.w32(ecx + 0x18, a[1]);
            Ret::U8(1)
        }
        0x402570 => get_triple(m, h, ecx, esp, 0x60, 0),
        0x402590 => get_triple(m, h, ecx, esp, 0x60, 1),
        0x4025B0 => get_triple(m, h, ecx, esp, 0x60, 2),
        0x4BDAC0 => get_triple(m, h, ecx, esp, 0x40, 0),
        0x4BDAE0 => get_triple(m, h, ecx, esp, 0x40, 1),
        0x4025D0 => {
            copy3(m, ecx + 0x20, a[0]);
            Ret::U8(1)
        }
        0x4C9270 => {
            copy3(m, ecx + 0x14, a[0]);
            Ret::U8(1)
        }
        0x4025F0 => {
            let buf = esp.wrapping_sub(0xC);
            m.w32(buf + 4, a[1]);
            m.w32(buf, a[0]);
            m.w32(buf + 8, 0);
            Ret::U32(vcall(m, h, ecx, 0x7C, &[buf]))
        }
        0x402660 => {
            let (x, y) = (m.r32(ecx + 0x20), m.r32(ecx + 0x24));
            m.w32(a[0], x);
            m.w32(a[1], y);
            Ret::U32(y)
        }
        0x402690 | 0x4B1C00 => {
            let off = if f == 0x402690 { 0x4A } else { 0x4B };
            let old = m.r8(ecx + off);
            m.w8(ecx + off, a[0] as u8);
            Ret::U8(old)
        }
        0x4026B0 => {
            m.w8(ecx + 0x90, a[0] as u8);
            Ret::U8(a[0] as u8)
        }
        0x4B2210 => {
            m.w8(ecx + 0xC, a[0] as u8);
            Ret::U8(a[0] as u8)
        }
        0x4026C0 => {
            m.w8(ecx + 0x42, 1);
            Ret::U8(1)
        }
        0x4026D0 => {
            m.w8(ecx + 0x42, 0);
            Ret::U8(1)
        }
        0x4B2410 => {
            m.w8(ecx + 0xC, 0);
            Ret::None
        }
        0x4B2230 => {
            m.w32(ecx + 0x10, 0);
            Ret::None
        }
        0x4B7850 => {
            m.w8(ecx + 0x13C, 1);
            Ret::None
        }
        // Nombres de clase.
        0x4026F0 => class_name(m, h, a[0], 0x6BBAB8),
        0x40B160 => class_name(m, h, a[0], 0x69861C),
        0x40BC60 => class_name(m, h, a[0], 0x695AFC),
        0x4418F0 => class_name(m, h, a[0], 0x697A64),
        0x4B9790 => class_name(m, h, a[0], 0x6BBBD0),
        // Destructores escalares.
        0x441920 => scalar_dtor(m, h, ecx, a[0], 0x440A20),
        0x462440 => scalar_dtor(m, h, ecx, a[0], 0x462180),
        0x4B0810 => scalar_dtor(m, h, ecx, a[0], 0x4B0440),
        0x4B2260 => scalar_dtor(m, h, ecx, a[0], 0x4B1640),
        0x4B8640 => scalar_dtor(m, h, ecx, a[0], 0x4B4A60),
        0x4B97C0 => scalar_dtor(m, h, ecx, a[0], 0x4B87E0),
        // Funciones vacías.
        0x440E40 | 0x4B1850 | 0x4B1C10 | 0x4B5A50 | 0x4B6B70 => Ret::None,
        // Órdenes de texto y configuración sin efecto: destruyen la cadena que reciben por valor.
        0x40BC90 | 0x440B00 | 0x4B57A0 | 0x440B10 => {
            h.call(m, CSTRING_DTOR, esp.wrapping_add(4), &[]);
            Ret::U8(if f == 0x440B10 { 1 } else { 0 })
        }
        0x408FB0 | 0x408FC0 => {
            let w = m.r32(ecx + 0x1C);
            Ret::U32(import(m, h, 0x66A4DC, &[w, if f == 0x408FB0 { 0 } else { 1 }]))
        }
        0x441180 => Ret::U32(h.call(m, 0x440F70, crate::j07::CDECL, &[ecx, a[0]])),
        0x441710 => {
            let rect = esp.wrapping_sub(0x10);
            let (x, y) = (m.r32(ecx + 0x14), m.r32(ecx + 0x18));
            let right = m.r32(ecx + 0x2C).wrapping_add(x);
            let bottom = m.r32(ecx + 0x30).wrapping_add(y);
            m.w32(rect, x);
            m.w32(rect + 4, y);
            m.w32(rect + 8, right);
            m.w32(rect + 0xC, bottom);
            Ret::U32(h.call(m, 0x4B9650, 0x6DA5F0, &[rect]))
        }
        0x45C0D0 => {
            let v = m.r32(ecx + 0x10);
            let limit = m.r32(0x670F6C);
            if (v as i32) >= (limit as i32) {
                Ret::U32(v)
            } else {
                Ret::U32(h.call(m, 0x4B1980, ecx, a))
            }
        }
        0x45C9D0 => {
            let args = [m.r32(a[0]), m.r32(a[0] + 4), m.r32(a[0] + 8)];
            Ret::U32(vcall(m, h, ecx, 0x4C, &args))
        }
        0x45C9F0 => {
            let vtbl = m.r32(ecx);
            let args = [m.r32(a[0] + 8), m.r32(a[0] + 0xC), 0];
            let t = m.r32(vtbl + 0x4C);
            Ret::U32(h.call(m, t, ecx, &args))
        }
        0x462140 | 0x462160 => {
            let s = if f == 0x462140 { m.r32(0x69861C) } else { a[0] };
            h.call(m, 0x4B15F0, ecx, &[s]);
            m.w32(ecx, 0x66E240);
            Ret::U32(ecx)
        }
        0x462180 => {
            m.w32(ecx, 0x66E240);
            Ret::U32(h.call(m, 0x4B1640, ecx, &[]))
        }
        0x462190 | 0x4621C0 => {
            let obj = esp.wrapping_sub(0xC);
            let (lit, target) = if f == 0x462190 { (0x698620, 0x4B3EA0) } else { (0x698638, 0x4B41F0) };
            h.call(m, CSTRING_CTOR, obj, &[lit]);
            let s = m.r32(obj);
            Ret::U32(h.call(m, target, 0x6DA820, &[s, 0x3E80]))
        }
        0x4B0440 => {
            m.w32(ecx, 0x67078C);
            Ret::U32(h.call(m, CSTRING_DTOR, ecx + 8, &[]))
        }
        0x4B04C0 => {
            let b = m.r8(a[0]);
            m.w8(ecx + 0x10, b);
            Ret::U32(ecx)
        }
        0x4B0500 => {
            h.call(m, 0x64F340, ecx + 8, &[a[0] + 8]);
            let v = m.r32(a[0] + 0xC);
            m.w32(ecx + 0xC, v);
            let b = m.r8(a[0] + 0x10);
            m.w8(ecx + 0x10, b);
            for off in [0x14, 0x18, 0x1C] {
                let v = m.r32(a[0] + off);
                m.w32(ecx + off, v);
            }
            Ret::U32(ecx)
        }
        0x4B0540 => {
            h.call(m, 0x64F390, ecx + 8, &[0x6D95B9]);
            m.w32(ecx + 0xC, 0);
            m.w8(ecx + 0x10, 0);
            for off in [0x14, 0x18, 0x1C] {
                m.w32(ecx + off, 0);
            }
            Ret::U32(0)
        }
        0x4B1640 => {
            m.w32(ecx, 0x6708D0);
            Ret::U32(h.call(m, 0x4B2BE0, ecx, &[]))
        }
        0x4B1870 => {
            if (a[0] as i32) < -1 {
                Ret::U8(0)
            } else {
                m.w32(ecx + 0x3C, a[0]);
                Ret::U8(1)
            }
        }
        0x4B1890 => {
            if a[0] == 0 {
                Ret::U8(0)
            } else {
                m.w32(ecx + 0x38, a[0]);
                Ret::U8(1)
            }
        }
        0x4B18C0 => Ret::U32(vcall(m, h, ecx, 0x9C, &[a[0], 0, 0, 0])),
        0x4B1A80 => {
            let v = m.r32(ecx + 0x44) & a[0];
            m.w32(ecx + 0x44, v);
            let r = vcall(m, h, ecx, 0xAC, &[]);
            vcall(m, h, ecx, 0x2C, &[if r as u8 != 0 { 3 } else { 1 }]);
            Ret::U8(1)
        }
        0x4B1AD0 => {
            let content = m.r32(ecx + 0x5C);
            copy4(m, ecx + 0x4C, a[0]);
            if content == 0 {
                Ret::U32(a[0])
            } else {
                Ret::U32(vcall(m, h, content, 0xD8, &[a[0]]))
            }
        }
        0x4B1B10 => {
            let rect = esp.wrapping_sub(0x10);
            m.w32(rect + 4, 0);
            m.w32(rect, 0);
            let bottom = vcall(m, h, ecx, 0x48, &[]);
            m.w32(rect + 0xC, bottom);
            let right = vcall(m, h, ecx, 0x44, &[]);
            m.w32(rect + 8, right);
            Ret::U32(vcall(m, h, ecx, 0xD8, &[rect]))
        }
        0x4B1BA0 => Ret::U32(vcall(m, h, ecx, 0xE0, &[0])),
        0x4B1BB0 => {
            let r = import(m, h, 0x66A4D0, &[ecx + 0x4C]);
            Ret::U32((r != 0) as u32)
        }
        0x4B1BD0 => {
            copy4(m, a[0], ecx + 0x4C);
            Ret::U32(a[0])
        }
        0x4B1EA0 => {
            let mode = m.r32(ecx + 4);
            let id = m.r32(ecx.wrapping_add(mode.wrapping_mul(4)).wrapping_add(0x70));
            if id != u32::MAX {
                let snd = m.r32(0x6DCCDC);
                if snd != 0 {
                    h.call(m, 0x4C45A0, snd, &[id]);
                }
            }
            Ret::None
        }
        0x4B1F50 => {
            let mode = m.r32(ecx + 4);
            vcall(m, h, ecx, 0xFC, &[a[0], mode]);
            Ret::U32(a[0])
        }
        0x4B2010 => {
            let i = a[0] as i32;
            if i < 0 {
                for k in 0..4 {
                    m.w32(ecx + 0x80 + 4 * k, a[1]);
                }
                Ret::U8(1)
            } else if i >= 4 {
                Ret::U8(0)
            } else {
                m.w32(ecx + 0x80 + 4 * a[0], a[1]);
                Ret::U8(1)
            }
        }
        0x4B2050 => {
            let mode = m.r32(ecx + 4);
            Ret::U32(vcall(m, h, ecx, 0x108, &[mode]))
        }
        0x4B2060 => {
            let i = a[0] as i32;
            if (0..4).contains(&i) {
                Ret::U32(m.r32(ecx + 0x80 + 4 * a[0]))
            } else {
                Ret::U32(u32::MAX)
            }
        }
        0x4B2280 => {
            let buf = esp.wrapping_sub(0xC);
            m.w32(buf + 8, 0);
            let (x, y) = (m.r32(a[0]), m.r32(a[0] + 4));
            m.w32(buf + 4, y);
            m.w32(buf, x);
            Ret::U32(vcall(m, h, ecx, 0x24, &[buf]))
        }
        0x4B4C70 => {
            let c = m.r32(ecx + 0xC0);
            Ret::U32(h.call(m, 0x4B4CE0, ecx, &[c, a[0], a[1]]))
        }
        0x4B4E10 => {
            let c = m.r32(ecx + 0xC0);
            Ret::U32(h.call(m, 0x4B4CE0, ecx, &[c, a[0], esp.wrapping_add(8)]))
        }
        0x4B4E30 => {
            let buf = esp.wrapping_sub(0xC);
            vcall(m, h, a[0], 0x84, &[buf]);
            let c = m.r32(ecx + 0xC0);
            Ret::U32(h.call(m, 0x4B4CE0, ecx, &[c, a[0], buf]))
        }
        0x4B50F0 => {
            let list = ecx + 0x144;
            let r = h.call(m, 0x64E7A6, list, &[a[0], 0]);
            if r != 0 {
                Ret::U32(r)
            } else {
                Ret::U32(h.call(m, 0x64E6FC, list, &[a[0]]))
            }
        }
        0x4B5A60 => {
            let buf = esp.wrapping_sub(0xC);
            for k in 0..3 {
                m.w32(buf + 4 * k, 0);
            }
            Ret::U32(vcall(m, h, ecx, 0x138, &[a[0], buf]))
        }
        0x4B6C70 => Ret::U32(0xFFFF_FFFE),
        0x4B6CA0 | 0x4B6CC0 => {
            let i = m.r32(ecx + if f == 0x4B6CA0 { 0xC8 } else { 0xC4 });
            if i == u32::MAX {
                Ret::U32(0)
            } else {
                let tab = m.r32(ecx + 0x94);
                Ret::U32(m.r32(i.wrapping_shl(4).wrapping_add(tab)))
            }
        }
        0x4B6E40 => {
            cdecl(m, h, 0x4BBA50, &[0x4416_0000, 0x4416_0000, 0x3F80_0000]);
            cdecl(m, h, 0x4BB260, &[2, 0, 0x3F80_0000]);
            cdecl(m, h, 0x4BB200, &[]);
            cdecl(m, h, 0x4BB3C0, &[0]);
            Ret::U32(cdecl(m, h, 0x4BB440, &[]))
        }
        0x4B7090 => {
            vcall(m, h, ecx, 0x118, &[]);
            let r = vcall(m, h, ecx, 0x120, &[]);
            vcall(m, h, ecx, 0x128, &[]);
            Ret::U32(r)
        }
        0x4B70C0 => loop {
            let r = h.call(m, 0x4B8A40, 0x6DA5F0, &[]);
            if r != u32::MAX {
                break Ret::U32(r);
            }
        },
        0x4B77F0 => {
            let (x, y) = (m.r32(ecx + 0xA0), m.r32(ecx + 0xA4));
            m.w32(a[0], x);
            m.w32(a[0] + 4, y);
            Ret::U32(a[0])
        }
        0x4B7C80 => Ret::U32(h.call(m, 0x4B7370, ecx, &[u32::MAX, 0])),
        0x4B8520 => {
            let on = a[0] as u8;
            if on == 0 {
                h.call(m, 0x4B8380, ecx, &[0]);
            }
            m.w8(ecx + 0x122, on);
            Ret::None
        }
        0x4B92C0 => {
            if m.r32(ecx + 0x10) == 0 {
                return Some(Ret::None);
            }
            let list = m.r32(ecx + 0xC);
            let top = m.r32(list + 8);
            if top == 0 {
                return Some(Ret::None);
            }
            Ret::U32(h.call(m, 0x4B7850, top, &[]))
        }
        0x4B9430 => {
            if a[0] != 0 {
                copy3(m, a[0], ecx + 0x208);
            }
            Ret::U32(a[0])
        }
        0x4B9540 => {
            let i = m.r32(ecx + 0x214);
            if (i as i32) < 0 {
                Ret::U32(0)
            } else {
                Ret::U32(m.r32(ecx.wrapping_add(i.wrapping_mul(4)).wrapping_add(0x1E8)))
            }
        }
        0x4B9730 => {
            if (m.r32(ecx + 0x10) as i32) <= 0 {
                Ret::U32(0)
            } else {
                let list = m.r32(ecx + 0xC);
                Ret::U32(m.r32(list + 8))
            }
        }
        _ => return None,
    };
    Some(r)
}
