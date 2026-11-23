//! Guardado y carga del comerciante de la IA (0x004F4010 / 0x004F4100 y sus cadenas). Referencia: x86 original.
//!
//! La estructura es de 0x650 bytes. Los tramos fijos están en [`merchant_layout`](super::merchant_layout) (deducidos de
//! ejecuciones del original) y las diez listas de palabras (`+0x624`, recuentos `+0x60C & 0x7FFF`) se tratan aquí.

use super::merchant_layout::{LOAD_LAYOUTS, SAVE_LAYOUT, SAVE_TABLE_LEN};

pub const MERCHANT_SIZE: usize = 0x650;
const COUNTS: usize = 0x60C;
const POINTERS: usize = 0x624;

/// Asignador y copia de las listas de palabras (`0x64F7B9`, `0x64F7E2`).
pub trait LoadHost {
    fn free(&mut self, ptr: u32);
    fn alloc(&mut self, size: u32) -> u32;
    fn write(&mut self, ptr: u32, data: &[u8]);
}

fn count(m: &[u8], i: usize) -> usize {
    (u16::from_le_bytes([m[COUNTS + 2 * i], m[COUNTS + 2 * i + 1]]) & 0x7FFF) as usize
}

/// `0x004F4010`: serializa la estructura (tramos fijos) y las listas con recuento no nulo. `lists[i]` aporta `2*n` bytes.
pub fn save_merchant(m: &[u8], lists: &[&[u8]; 10]) -> Vec<u8> {
    let mut out = vec![0u8; SAVE_TABLE_LEN];
    for &(q, s, l) in SAVE_LAYOUT {
        let (q, s, l) = (q as usize, s as usize, l as usize);
        out[q..q + l].copy_from_slice(&m[s..s + l]);
    }
    for (i, list) in lists.iter().enumerate() {
        let n = count(m, i);
        if n != 0 {
            out.extend_from_slice(&list[..2 * n]);
        }
    }
    out
}

/// `0x004F4100`: carga desde `stream` (con relleno de ceros tras el final) según la `version` del guardado y devuelve los
/// bytes consumidos. Verificada para versiones 0..=0xA0 (la comparación original es con signo; no se modelan versiones mayores).
pub fn load_merchant(m: &mut [u8], version: u32, stream: &[u8], host: &mut dyn LoadHost) -> usize {
    let layout = LOAD_LAYOUTS
        .iter()
        .rev()
        .find(|l| l.min_version <= version)
        .expect("versión sin tabla");
    for &(p, d, l) in layout.copies {
        let (p, d, l) = (p as usize, d as usize, l as usize);
        m[d..d + l].copy_from_slice(&stream[p..p + l]);
    }
    for &(d, bytes) in layout.consts {
        m[d as usize..d as usize + bytes.len()].copy_from_slice(bytes);
    }
    for &(p, d) in layout.bools {
        m[d as usize] = (stream[p as usize] != 0) as u8;
    }
    let mut pos = layout.table_len;
    for i in 0..10 {
        let slot = POINTERS + 4 * i;
        let old = u32::from_le_bytes(m[slot..slot + 4].try_into().unwrap());
        if old != 0 {
            host.free(old);
        }
        m[slot..slot + 4].fill(0);
        let n = count(m, i);
        if n != 0 {
            let size = 2 * n;
            let ptr = host.alloc(size as u32);
            m[slot..slot + 4].copy_from_slice(&ptr.to_le_bytes());
            host.write(ptr, &stream[pos..pos + size]);
            pos += if version < 0x7B { 4 } else { size };
        }
    }
    pos
}
