//! Órdenes de jugador en la batalla naval, sobre la imagen de memoria del original: poner objetivo (`0x00541050`), nivel de velas (`0x005414E0`),
//! modo de orden (`0x00541380`), devolver a la IA (`0x005415E0`) y retirada (`0x00541700`). Todas recorren las unidades de un bando: la tabla
//! `0x006E55D0[op+4]` apunta a la batalla, cuyo `+4` es la lista de unidades; las 64 unidades de cada bando se eligen con dos máscaras de 32 bits
//! (`op+0xC`: primera mitad; `op+8`: segunda).
use crate::flatmem::Mem;

const BATTLES: u32 = 0x006e_55d0;

/// Recorre las unidades seleccionadas del bando `side` y llama a `f(unidad)`; la unidad puede ser 0 (casilla vacía con su bit activo).
fn each_unit(m: &mut dyn Mem, op: u32, side: u8, mut f: impl FnMut(&mut dyn Mem, u32)) {
    let battle_index = u32::from(m.r8(op + 4));
    if m.r32(BATTLES + battle_index * 4) == 0 {
        return;
    }
    let side = u32::from(side);
    for (mask_at, start, end) in [(op + 0xc, side << 6, (side << 6) + 0x20), (op + 8, (side << 6) + 0x20, (side + 1) << 6)] {
        if m.r32(mask_at) == 0 {
            continue;
        }
        let mut index = start as u16;
        while u32::from(index) < end {
            if m.r32(mask_at) & (1u32 << (u32::from(index) & 0x1f)) != 0 {
                let slot = u32::from(m.r8(op + 4));
                let battle = m.r32(BATTLES + slot * 4);
                let list = m.r32(battle + 4);
                let unit = m.r32(list.wrapping_add(u32::from(index) * 4));
                f(m, unit);
            }
            index = index.wrapping_add(1);
        }
    }
}

/// `0x00541050`: las unidades seleccionadas del bando `op+5` reciben el objetivo `op+6` (si no están abordando, bit 3 de `+0x12A`) y pierden el bit 0.
/// Con una casilla vacía el original escribe en `0x12A`.
pub fn set_target(m: &mut dyn Mem, op: u32) {
    let side = m.r8(op + 5);
    each_unit(m, op, side, |m, unit| {
        if unit != 0 && m.r8(unit + 0x12a) & 8 == 0 {
            let target = m.r16(op + 6);
            m.w16(unit + 0x128, target);
        }
        let flags = m.r8(unit.wrapping_add(0x12a));
        m.w8(unit.wrapping_add(0x12a), flags & 0xfe);
    });
}

/// `0x005414E0`: nivel de velas `op+5` (`+0x148`) de las unidades seleccionadas del bando `op+6`.
pub fn set_sail_level(m: &mut dyn Mem, op: u32) {
    let side = m.r8(op + 6);
    each_unit(m, op, side, |m, unit| {
        if unit != 0 {
            let level = m.r8(op + 5);
            m.w8(unit + 0x148, level);
        }
    });
}

/// `0x00541380`: modo de orden `op+5` (`+0x13E`; si no es 0 quita el bit 0 de `+0x12A`) y bit 7 de `+0x12A` según `op+6`, bando `op+7`.
pub fn set_unit_order_mode(m: &mut dyn Mem, op: u32) {
    let side = m.r8(op + 7);
    each_unit(m, op, side, |m, unit| {
        if unit == 0 {
            return;
        }
        let mode = m.r8(op + 5);
        if mode != 0 {
            let flags = m.r8(unit + 0x12a);
            m.w8(unit + 0x12a, flags & 0xfe);
        }
        m.w8(unit + 0x13e, mode);
        let flags = m.r8(unit + 0x12a);
        let flags = if m.r8(op + 6) != 0 { flags | 0x80 } else { flags & 0x7f };
        m.w8(unit + 0x12a, flags);
    });
}

/// `0x005415E0`: devuelve a la IA las unidades seleccionadas del bando `op+5`: modo 0 y banderas `(flags & 0x5E) | 0x20`.
pub fn return_to_ai(m: &mut dyn Mem, op: u32) {
    let side = m.r8(op + 5);
    each_unit(m, op, side, |m, unit| {
        if unit == 0 {
            return;
        }
        let flags = m.r8(unit + 0x12a);
        m.w8(unit + 0x13e, 0);
        m.w8(unit + 0x12a, (flags & 0x5e) | 0x20);
    });
}

/// `0x00541700`: retirada: con `op+5` ≠ 0 el bit 0 de `+0x12A` se activa, los bits 1–6 se borran y el modo `+0x13E` se pone a 0; con 0 solo se borra el bit 0. Bando `op+6`.
pub fn set_retreat(m: &mut dyn Mem, op: u32) {
    let side = m.r8(op + 6);
    each_unit(m, op, side, |m, unit| {
        if unit == 0 {
            return;
        }
        let mut flags = m.r8(unit + 0x12a);
        if m.r8(op + 5) != 0 {
            flags &= 0x7e;
            m.w8(unit + 0x13e, 0);
            flags |= 1;
        } else {
            flags &= 0xfe;
        }
        m.w8(unit + 0x12a, flags);
    });
}
