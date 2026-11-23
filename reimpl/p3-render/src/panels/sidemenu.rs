//! Menús laterales de la interfaz (`*SideMenu`): las funciones plantilla de los once menús (ocultar, cerrar, limpiar la selección,
//! soltar botones, estado inicial y destructor con liberación) portadas con sus parámetros por menú.
//!
//! Disposición común: botones `ViperButton` de `0xE8` bytes desde `+0xAC` (vtable: `+0x34` visible, `+0xCC` activar, `+0x118`
//! ocultar/soltar, `+0x124` limpiar), byte de selección en `+sel` (0xFF = ninguna) y un objeto global por menú con vtable
//! (`+0xC8` consulta, `+0x118` cierre).

use super::{call, Host, Mem, Ret};
use crate::j07::{free, vcall};

const DTORS: [(u32, u32); 19] = [
    (0x4620E0, 0x460A70),
    (0x4BABC0, 0x4B98E0),
    (0x431330, 0x430FE0),
    (0x478570, 0x472910),
    (0x47C070, 0x4787D0),
    (0x495840, 0x4874A0),
    (0x496AB0, 0x4958F0),
    (0x5A3170, 0x5A2680),
    (0x5A4090, 0x5A3270),
    (0x5A4ED0, 0x5A41A0),
    (0x5A5CE0, 0x5A4FD0),
    (0x5A7D20, 0x5A5DE0),
    (0x5A9A00, 0x5A8B70),
    (0x5AA680, 0x5A9B20),
    (0x5AB160, 0x5AA730),
    (0x5AC390, 0x5AB270),
    (0x5ACF20, 0x5AC490),
    (0x5AE370, 0x5AD040),
    (0x486B60, 0x47C2C0),
];
const REL: [(u32, u32, u32); 10] = [
    (0x5A2700, 0x6, 0x620),
    (0x5A32F0, 0x7, 0x708),
    (0x5A4230, 0x7, 0x708),
    (0x5A5050, 0x4, 0x450),
    (0x5A5E60, 0xA, 0x9C0),
    (0x5A8BF0, 0x8, 0x7F0),
    (0x5AA7B0, 0x2, 0x280),
    (0x5AB300, 0x8, 0x7F0),
    (0x5AC510, 0x3, 0x368),
    (0x5AD0E0, 0x6, 0x620),
];
const HIDE: [(u32, u32, u32, u32); 8] = [
    (0x5A2DA0, 0x6, 0x6E2, 0x6E5534),
    (0x5A49F0, 0x7, 0x806, 0x6E557C),
    (0x5A5870, 0x4, 0x4D2, 0x6E556C),
    (0x5A9260, 0x8, 0x8F4, 0x6E558C),
    (0x5ABBB0, 0x8, 0x8F0, 0x6E55B8),
    (0x5ACB50, 0x3, 0x3CA, 0x6E5584),
    (0x5ADB20, 0x6, 0x6FA, 0x6E55C0),
    (0x5AA230, 0x1, 0x1B8, 0x6E55A0),
];
const CLOSE: [(u32, u32); 10] = [
    (0x5A2E20, 0x6),
    (0x5A3BA0, 0x7),
    (0x5A4A70, 0x7),
    (0x5A92E0, 0x8),
    (0x5AAF10, 0x2),
    (0x5ABC30, 0x8),
    (0x5ACBD0, 0x3),
    (0x5ADBA0, 0x6),
    (0x5A6660, 0xA),
    (0x5AA2A0, 0x1),
];
const CLEAR: [(u32, u32, u32); 8] = [
    (0x5A5C50, 0x4, 0x4D2),
    (0x5A9750, 0x8, 0x8F4),
    (0x5ABF70, 0x8, 0x8F0),
    (0x5AE070, 0x6, 0x6FA),
    (0x5A3120, 0x6, 0x6E2),
    (0x5A4040, 0x7, 0x7EA),
    (0x5A6DE0, 0xA, 0xB16),
    (0x5AA600, 0x1, 0x1B8),
];
const RESET1: [(u32, u32); 7] = [
    (0x5A2740, 0x6E3),
    (0x5A3330, 0x7EC),
    (0x5A4270, 0x808),
    (0x5A5EA0, 0xB17),
    (0x5A8C30, 0x8F6),
    (0x5A9BE0, 0x1BA),
    (0x5AA7F0, 0x2C2),
];

/// (constructor, destructor, botones, matriz de 0x20 bytes, vtable, miembro extra: 0 ninguno, 0xFFFF los dos `CString` del astillero).
const CTOR: [(u32, u32, u32, u32, u32, u32); 11] = [
    (0x5A2600, 0x5A2680, 0x6, 0x620, 0x677E70, 0x0),
    (0x5A31F0, 0x5A3270, 0x7, 0x708, 0x677FB8, 0x0),
    (0x5A4110, 0x5A41A0, 0x7, 0x708, 0x6780F0, 0x7E8),
    (0x5A4F50, 0x5A4FD0, 0x4, 0x450, 0x678228, 0x0),
    (0x5A5D60, 0x5A5DE0, 0xA, 0x9C0, 0x678360, 0x0),
    (0x5A8AF0, 0x5A8B70, 0x8, 0x7F0, 0x6785E0, 0x0),
    (0x5A9A80, 0x5A9B20, 0x1, 0x198, 0x678718, 0x1BC),
    (0x5AA6B0, 0x5AA730, 0x2, 0x280, 0x678850, 0x0),
    (0x5AB1E0, 0x5AB270, 0x8, 0x7F0, 0x678988, 0x904),
    (0x5AC410, 0x5AC490, 0x3, 0x368, 0x678AC0, 0x0),
    (0x5ACFA0, 0x5AD040, 0x6, 0x620, 0x678BF8, 0xFFFF),
];

const BUTTONS: u32 = 0xAC;
const STRIDE: u32 = 0xE8;

fn row<T: Copy>(table: &[T], f: u32, key: impl Fn(&T) -> u32) -> Option<T> {
    table.iter().copied().find(|r| key(r) == f)
}

pub fn run(f: u32, m: &mut dyn Mem, h: &mut dyn Host, t: u32, a: &[u32], esp: u32) -> Option<Ret> {
    if let Some((_, _, n, arr, vt, extra)) = row(&CTOR, f, |r| r.0) {
        h.call(m, 0x440900, t, &[]);
        h.call(m, 0x639E9C, crate::j07::CDECL, &[t + BUTTONS, STRIDE, n, 0x4C6910, 0x4C6A30]);
        h.call(m, 0x639E9C, crate::j07::CDECL, &[t + arr, 0x20, n, 0x4B03E0, 0x4B0440]);
        if extra == 0xFFFF {
            // astillero: los dos `CString` apuntan a la cadena vacía compartida.
            let empty = m.r32(0x006C_7CCC);
            m.w32(t + 0x6E0, empty);
            let empty = m.r32(0x006C_7CCC);
            m.w32(t + 0x6E4, empty);
        } else if extra != 0 {
            h.call(m, 0x4209E0, t + extra, &[]);
        }
        m.w32(t, vt);
        if f == 0x5A9A80 {
            m.w32(t + 0x1D8, 0);
            m.w8(t + 0x1DC, 0xFF);
        }
        return Some(Ret::U32(t));
    }
    if let Some((_, _, n, arr, vt, extra)) = row(&CTOR, f, |r| r.1) {
        m.w32(t, vt);
        if extra == 0xFFFF {
            h.call(m, 0x64F253, t + 0x6E4, &[]);
            h.call(m, 0x64F253, t + 0x6E0, &[]);
        } else if extra != 0 {
            h.call(m, 0x420A00, t + extra, &[]);
        }
        h.call(m, 0x639DA8, crate::j07::CDECL, &[t + arr, 0x20, n, 0x4B0440]);
        h.call(m, 0x639DA8, crate::j07::CDECL, &[t + BUTTONS, STRIDE, n, 0x4C6A30]);
        h.call(m, 0x440A20, t, &[]);
        return Some(Ret::None);
    }
    if let Some((_, dtor)) = row(&DTORS, f, |r| r.0) {
        // `scalar deleting destructor`: destruye y, con el bit 0 del argumento, libera el bloque.
        call(m, h, dtor, t, &[], esp.wrapping_sub(8));
        if a[0] & 1 != 0 {
            free(m, h, t);
        }
        return Some(Ret::U32(t));
    }
    if let Some((_, n, arr)) = row(&REL, f, |r| r.0) {
        for i in 0..n {
            vcall(m, h, t + BUTTONS + i * STRIDE, 0x118, &[]);
            h.call(m, 0x4B0540, t + arr + i * 0x20, &[]);
        }
        return Some(Ret::None);
    }
    if let Some((_, n, sel, glob)) = row(&HIDE, f, |r| r.0) {
        h.call(m, 0x440ED0, t, &[]);
        for i in 0..n {
            let b = t + BUTTONS + i * STRIDE;
            vcall(m, h, b, 0x34, &[0]);
            vcall(m, h, b, 0xCC, &[0]);
            vcall(m, h, b, 0x124, &[0]);
        }
        let g = m.r32(0x006C_C7E0);
        h.call(m, 0x466B40, g, &[]);
        m.w8(t + sel, 0xFF);
        let o = m.r32(glob);
        let r = vcall(m, h, o, 0xC8, &[]);
        if r as u8 != 0 {
            let o = m.r32(glob);
            return Some(Ret::U32(vcall(m, h, o, 0x118, &[])));
        }
        return Some(Ret::None);
    }
    if f == 0x5AAE90 {
        // Menú del armero: como los demás, sin la llamada `0x466B40`, y deja el byte `+0x2C3` a 1.
        h.call(m, 0x440ED0, t, &[]);
        for i in 0..2 {
            let b = t + BUTTONS + i * STRIDE;
            vcall(m, h, b, 0x34, &[0]);
            vcall(m, h, b, 0xCC, &[0]);
            vcall(m, h, b, 0x124, &[0]);
        }
        m.w8(t + 0x2C0, 0xFF);
        let o = m.r32(0x006E_55A8);
        if vcall(m, h, o, 0xC8, &[]) as u8 != 0 {
            let o = m.r32(0x006E_55A8);
            vcall(m, h, o, 0x118, &[]);
        }
        m.w8(t + 0x2C3, 1);
        return Some(Ret::None);
    }
    if let Some((_, n)) = row(&CLOSE, f, |r| r.0) {
        let o = h.call(m, 0x4B9730, 0x006D_A5F0, &[]);
        for i in 0..n {
            h.call(m, 0x4B4EB0, o, &[t + BUTTONS + i * STRIDE]);
        }
        let p = m.r32(0x006C_BDB8);
        if m.r8(p + 0x1C5) != 0 {
            h.call(m, 0x4436E0, crate::j07::CDECL, &[0, 0]);
        }
        return Some(Ret::U32(h.call(m, 0x4410F0, t, &[])));
    }
    if let Some((_, n, sel)) = row(&CLEAR, f, |r| r.0) {
        for i in 0..n {
            vcall(m, h, t + BUTTONS + i * STRIDE, 0x124, &[0]);
        }
        m.w8(t + sel, 0xFF);
        if f == 0x5A6DE0 {
            m.w16(t + 0xB1C, 0xFFFF);
        }
        return Some(Ret::None);
    }
    if let Some((_, off)) = row(&RESET1, f, |r| r.0) {
        m.w8(t + off, 0);
        return Some(Ret::U8(1));
    }
    let r = match f {
        0x5A5090 => {
            m.w8(t + 0x4D3, 0);
            m.w32(t + 0x4D8, 0);
            Ret::U32(1)
        }
        0x5AB340 => {
            m.w8(t + 0x8F2, 0);
            m.w32(t + 0x8F4, 8);
            Ret::U8(1)
        }
        0x5AC550 | 0x5AD120 => {
            let sel = if f == 0x5AC550 { 0x3CA } else { 0x6FA };
            m.w8(t + sel + 1, 0);
            m.w8(t + sel, 0xFF);
            Ret::U8(1)
        }
        _ => return None,
    };
    Some(r)
}
