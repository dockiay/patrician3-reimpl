//! 0x509AE0: amplía 32 slots en memoria plana sintética.
//!
//! La dirección lógica se mantiene para que los puentes publiquen las vistas
//! relocalizadas. Los bytes recién asignados usan la política `w8_seed` del
//! anfitrión; el EXE no inicializa más que los dos campos de lista.
use crate::flatmem::Mem;

const BASE: u32 = 0x6dd7a0;
const SIZE: u32 = 0x3c;

/// Amplía si la lista libre está agotada. El gestor y la región original deben
/// estar presentes. Rechaza cabezas corruptas y desbordamiento antes de mutar.
pub fn grow(m: &mut dyn Mem) -> Result<(), String> {
    let cap = m.r16(BASE + 0xf6);
    let head = m.r16(BASE + 0xe4);
    if head < cap {
        return Ok(());
    }
    if head != cap {
        return Err("cabeza libre de convoy fuera del pool".into());
    }
    let next = cap.checked_add(32).ok_or("desbordamiento de pool de convoyes")?;
    let base = m.r32(BASE + 8);
    if base == 0 {
        return Err("pool de convoyes sin dirección sintética".into());
    }
    let end = base
        .checked_add(u32::from(next) * SIZE)
        .ok_or("dirección de convoy desborda")?;
    let start = base + u32::from(cap) * SIZE;
    for p in start..end {
        m.w8_seed(p);
    }
    m.w16(BASE + 0xf6, next);
    for i in head..next {
        let at = base + u32::from(i) * SIZE;
        m.w16(at + 8, i + 1);
        m.w16(at + 0x12, 0xffff);
    }
    Ok(())
}
