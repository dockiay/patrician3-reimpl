//! Búsqueda del propietario de un edificio de ciudad en los diálogos de edificio (`mint_dialog_find_owner` 0x5DF3E0,
//! `school_info_dialog_refresh_owner` 0x5EDEC0; oráculo `oracle/x86_dialog_owner.py`, prueba `tests/x86_dialog_owner.rs`).
//!
//! El diálogo guarda la ciudad (`this+0x1A8`, byte) y la posición del edificio (`+0x1AC`, `+0x1B0`, dwords). Se recorren las dos cadenas de
//! obras/edificios de la ciudad (`Town+0x764` y `Town+0x762`, registros de `0x51F1F0`: `+0` x, `+1` y, `+2` propietario, `+3` tipo, `+4`
//! siguiente `u16`; cada cadena termina al llegar a `Town+0x776` o más) hasta el registro del tipo pedido en esa posición; su propietario
//! pasa a `this+0x1A9`.

use crate::economy::ops_misc::OpsHost;
use crate::flatmem::Mem;

const G_TOWNS: u32 = 0x006d_e508;
const G_TOWN_COUNT: u32 = 0x006d_e4aa;
const TOWN_STRIDE: u32 = 0x9f8;

/// Busca el registro de tipo `kind` en la posición del diálogo siguiendo las cadenas `+0x764` y `+0x762` de la ciudad.
fn find_record(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, kind: u8) -> Option<u32> {
    for chain in [0x764u32, 0x762] {
        let town = {
            let t = m.r32(G_TOWNS);
            t.wrapping_add(u32::from(m.r8(this + 0x1a8)).wrapping_mul(TOWN_STRIDE))
        };
        let mut cur = u32::from(m.r16(town + chain));
        while (cur as u16) < m.r16(town + 0x776) {
            let rec = host.call(m, 0x51f1f0, town, &[cur & 0xffff]);
            if m.r8(rec + 3) == kind && u32::from(m.r8(rec)) == m.r32(this + 0x1ac) && u32::from(m.r8(rec + 1)) == m.r32(this + 0x1b0) {
                return Some(rec);
            }
            cur = u32::from(m.r16(rec + 4));
            // el original recalcula la ciudad desde el índice del diálogo en cada vuelta (sin cambios entre vueltas)
        }
    }
    None
}

/// `0x005DF3E0` `mint_dialog_find_owner`: edificio tipo `0x2A` (casa de moneda). El propietario va a `this+0x1A9`; si no es un comerciante
/// válido (`>= recuento de ciudades`, `0x6DE4AA`) se guarda la propia ciudad del diálogo.
pub fn mint_dialog_find_owner(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) {
    if let Some(rec) = find_record(m, host, this, 0x2a) {
        let owner = m.r8(rec + 2);
        m.w8(this + 0x1a9, owner);
        if u32::from(m.r16(G_TOWN_COUNT)) <= u32::from(owner) {
            let town = m.r8(this + 0x1a8);
            m.w8(this + 0x1a9, town);
        }
    }
}

/// `0x005EDEC0` `school_info_dialog_refresh_owner`: edificio tipo `0x2B` (escuela); copia el propietario sin más comprobaciones.
pub fn school_info_dialog_refresh_owner(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) {
    if let Some(rec) = find_record(m, host, this, 0x2b) {
        let owner = m.r8(rec + 2);
        m.w8(this + 0x1a9, owner);
    }
}
