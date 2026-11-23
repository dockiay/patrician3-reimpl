//! Bloque del consejo en el guardado: `town_serialize_council_block` (0x005287A0) y `town_deserialize_council_block` (0x00528870).
//! Trabajan sobre los bytes de `Town` (`+0x6D4..+0x75C`) y un búfer; devuelven el desplazamiento final.

fn copy(dst: &mut [u8], d: usize, src: &[u8], s: usize, n: usize) {
    dst[d..d + n].copy_from_slice(&src[s..s + n]);
}

/// Escribe el bloque (0x88 bytes) en `buf`.
pub fn serialize(town: &[u8], buf: &mut [u8]) -> usize {
    copy(buf, 0, town, 0x6d4, 0x14);
    copy(buf, 0x14, town, 0x6e8, 8);
    for k in 0..4 {
        buf[0x1c + k] = town[0x6f0 + k];
    }
    copy(buf, 0x20, town, 0x6f4, 4);
    copy(buf, 0x24, town, 0x6f8, 4);
    copy(buf, 0x28, town, 0x6fc, 4);
    copy(buf, 0x2c, town, 0x700, 4);
    copy(buf, 0x30, town, 0x704, 0x2c);
    copy(buf, 0x5c, town, 0x730, 0x2c);
    0x88
}

/// Lee el bloque de un guardado de `version`; los campos que no existen en versiones antiguas quedan a cero.
pub fn deserialize(town: &mut [u8], buf: &[u8], version: u32) -> usize {
    copy(town, 0x6d4, buf, 0, 0x14);
    copy(town, 0x6e8, buf, 0x14, 8);
    for k in 0..4 {
        town[0x6f0 + k] = buf[0x1c + k];
    }
    let mut s = 0x20;
    if version < 0x77 {
        town[0x6f4..0x6f8].fill(0);
    } else {
        copy(town, 0x6f4, buf, s, 4);
        s += 4;
    }
    copy(town, 0x6f8, buf, s, 4);
    s += 4;
    if version < 0x76 {
        town[0x6fc..0x700].fill(0);
    } else {
        copy(town, 0x6fc, buf, s, 4);
        s += 4;
    }
    copy(town, 0x700, buf, s, 4);
    s += 4;
    if version < 0x6a {
        copy(town, 0x704, buf, s, 0x28);
        s += 0x28;
        town[0x72c..0x730].fill(0);
        copy(town, 0x730, buf, s, 0x28);
        s += 0x28;
        town[0x758..0x75c].fill(0);
    } else {
        copy(town, 0x704, buf, s, 0x2c);
        s += 0x2c;
        copy(town, 0x730, buf, s, 0x2c);
        s += 0x2c;
    }
    s
}

/// `get_alderman_name_index_pair` (0x005333C0): índices de nombre (`+0x1C`/`+0x1D` del comerciante o `+0x6D4`/`+0x6D8` de la ciudad).
/// `world` es el objeto `0x006DE4A0`; `towns` la tabla `0x006DE508`. Devuelve `(primero, segundo)`.
pub fn alderman_name_pair(m: &mut dyn crate::flatmem::Mem, world: u32, towns: u32) -> (u8, u8) {
    let idx = u32::from(m.r8(world + 0x8d));
    let count = u32::from(m.r16(world + 0xa));
    if idx < count {
        let merchant = m.r32(world + 0x78).wrapping_add(idx.wrapping_mul(0x650));
        return (m.r8(merchant + 0x1c), m.r8(merchant + 0x1d));
    }
    let town = towns.wrapping_add(u32::from(m.r8(world + 0x8e)).wrapping_mul(0x9f8));
    let mayor = u32::from(m.r8(town + 0x6f1));
    // Los dos bits bajos de `eax` proceden del byte `+0x8D`, cargado justo antes.
    let k = if mayor < count { 0 } else { !idx & 3 };
    (m.r8(town + 0x6d4 + k), m.r8(town + 0x6d8 + k))
}
