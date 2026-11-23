//! Reparto objeto ↔ registro de guardado (versión 0x85). Generado por `scripts/gen_layout_tables.py` desde `save-fields/*.tsv`: no editar.

/// Un tramo copiado: `len` bytes del objeto en `obj` van a `pos` dentro del registro del fichero.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Seg {
    pub obj: u16,
    pub len: u16,
    pub pos: u16,
}

/// Comerciante: `merchant_data` (0x004F4010/0x004F4100) → `merchant_buffer` → `merchant_record` y `map_discovery`.
pub const MERCHANT_OBJECT_LEN: usize = 0x650;
/// Bytes fijos del registro en el fichero.
pub const MERCHANT_RECORD_LEN: usize = 1552;
pub const MERCHANT: [Seg; 87] = [
    Seg { obj: 0x0, len: 4, pos: 0x0 },
    Seg { obj: 0x4, len: 4, pos: 0x4 },
    Seg { obj: 0x8, len: 2, pos: 0x8 },
    Seg { obj: 0xA, len: 2, pos: 0xA },
    Seg { obj: 0xC, len: 2, pos: 0xC },
    Seg { obj: 0xE, len: 2, pos: 0xE },
    Seg { obj: 0x10, len: 4, pos: 0x10 },
    Seg { obj: 0x14, len: 2, pos: 0x14 },
    Seg { obj: 0x16, len: 2, pos: 0x16 },
    Seg { obj: 0x18, len: 1, pos: 0x18 },
    Seg { obj: 0x19, len: 1, pos: 0x19 },
    Seg { obj: 0x1A, len: 1, pos: 0x1A },
    Seg { obj: 0x1B, len: 1, pos: 0x1B },
    Seg { obj: 0x1C, len: 1, pos: 0x1C },
    Seg { obj: 0x1D, len: 1, pos: 0x1D },
    Seg { obj: 0x1E, len: 1, pos: 0x1E },
    Seg { obj: 0x1F, len: 1, pos: 0x1F },
    Seg { obj: 0x20, len: 1, pos: 0x20 },
    Seg { obj: 0x21, len: 1, pos: 0x21 },
    Seg { obj: 0x23, len: 1, pos: 0x22 },
    Seg { obj: 0x22, len: 1, pos: 0x23 },
    Seg { obj: 0x24, len: 1, pos: 0x24 },
    Seg { obj: 0x25, len: 1, pos: 0x25 },
    Seg { obj: 0x26, len: 1, pos: 0x26 },
    Seg { obj: 0x27, len: 1, pos: 0x27 },
    Seg { obj: 0x28, len: 4, pos: 0x28 },
    Seg { obj: 0x2C, len: 4, pos: 0x2C },
    Seg { obj: 0x30, len: 1, pos: 0x30 },
    Seg { obj: 0x31, len: 1, pos: 0x31 },
    Seg { obj: 0x32, len: 1, pos: 0x32 },
    Seg { obj: 0x37, len: 1, pos: 0x33 },
    Seg { obj: 0x33, len: 1, pos: 0x34 },
    Seg { obj: 0x36, len: 1, pos: 0x35 },
    Seg { obj: 0x34, len: 2, pos: 0x36 },
    Seg { obj: 0x38, len: 8, pos: 0x38 },
    Seg { obj: 0xF0, len: 40, pos: 0x40 },
    Seg { obj: 0x40, len: 160, pos: 0x68 },
    Seg { obj: 0xE0, len: 1, pos: 0x108 },
    Seg { obj: 0xE1, len: 1, pos: 0x109 },
    Seg { obj: 0xE2, len: 1, pos: 0x10A },
    Seg { obj: 0xE3, len: 1, pos: 0x10B },
    Seg { obj: 0x118, len: 4, pos: 0x10C },
    Seg { obj: 0x11C, len: 480, pos: 0x110 },
    Seg { obj: 0x2FC, len: 160, pos: 0x2F0 },
    Seg { obj: 0x3C4, len: 160, pos: 0x390 },
    Seg { obj: 0x464, len: 4, pos: 0x430 },
    Seg { obj: 0x39C, len: 40, pos: 0x434 },
    Seg { obj: 0x468, len: 4, pos: 0x45C },
    Seg { obj: 0x46C, len: 4, pos: 0x460 },
    Seg { obj: 0x470, len: 4, pos: 0x464 },
    Seg { obj: 0x474, len: 4, pos: 0x468 },
    Seg { obj: 0x478, len: 2, pos: 0x46C },
    Seg { obj: 0x47A, len: 2, pos: 0x46E },
    Seg { obj: 0x480, len: 4, pos: 0x470 },
    Seg { obj: 0x484, len: 4, pos: 0x474 },
    Seg { obj: 0x488, len: 4, pos: 0x478 },
    Seg { obj: 0x48C, len: 4, pos: 0x47C },
    Seg { obj: 0x490, len: 4, pos: 0x480 },
    Seg { obj: 0x494, len: 4, pos: 0x484 },
    Seg { obj: 0x498, len: 4, pos: 0x488 },
    Seg { obj: 0x49C, len: 4, pos: 0x48C },
    Seg { obj: 0x4A0, len: 4, pos: 0x490 },
    Seg { obj: 0x4A4, len: 4, pos: 0x494 },
    Seg { obj: 0x4A8, len: 4, pos: 0x498 },
    Seg { obj: 0x4AC, len: 4, pos: 0x49C },
    Seg { obj: 0x4B0, len: 4, pos: 0x4A0 },
    Seg { obj: 0x4B4, len: 4, pos: 0x4A4 },
    Seg { obj: 0x4B8, len: 4, pos: 0x4A8 },
    Seg { obj: 0x4BC, len: 4, pos: 0x4AC },
    Seg { obj: 0x4C0, len: 4, pos: 0x4B0 },
    Seg { obj: 0x4C4, len: 4, pos: 0x4B4 },
    Seg { obj: 0x4C8, len: 4, pos: 0x4B8 },
    Seg { obj: 0x4CC, len: 4, pos: 0x4BC },
    Seg { obj: 0x4D0, len: 4, pos: 0x4C0 },
    Seg { obj: 0x4D4, len: 4, pos: 0x4C4 },
    Seg { obj: 0x538, len: 4, pos: 0x4C8 },
    Seg { obj: 0x4D8, len: 48, pos: 0x4CC },
    Seg { obj: 0x508, len: 48, pos: 0x4FC },
    Seg { obj: 0x53C, len: 4, pos: 0x52C },
    Seg { obj: 0x540, len: 4, pos: 0x530 },
    Seg { obj: 0x544, len: 2, pos: 0x534 },
    Seg { obj: 0x546, len: 1, pos: 0x536 },
    Seg { obj: 0x547, len: 1, pos: 0x537 },
    Seg { obj: 0x608, len: 4, pos: 0x538 },
    Seg { obj: 0x548, len: 96, pos: 0x53C },
    Seg { obj: 0x5A8, len: 96, pos: 0x59C },
    Seg { obj: 0x60C, len: 20, pos: 0x5FC },
];

/// Barco: núcleo (`ship_core_serialize`, 0x005162D0), relativo al inicio del núcleo (que va tras el estado de 2 bytes).
pub const SHIP_CORE_OBJECT_LEN: usize = 0x180;
/// Bytes fijos del registro en el fichero.
pub const SHIP_CORE_RECORD_LEN: usize = 84;
pub const SHIP_CORE: [Seg; 35] = [
    Seg { obj: 0x4, len: 2, pos: 0x0 },
    Seg { obj: 0x0, len: 1, pos: 0x2 },
    Seg { obj: 0xE, len: 1, pos: 0x3 },
    Seg { obj: 0xF, len: 1, pos: 0x4 },
    Seg { obj: 0x1, len: 1, pos: 0x5 },
    Seg { obj: 0x3, len: 1, pos: 0x6 },
    Seg { obj: 0x2, len: 1, pos: 0x7 },
    Seg { obj: 0x10, len: 4, pos: 0x8 },
    Seg { obj: 0x14, len: 4, pos: 0xC },
    Seg { obj: 0x6, len: 2, pos: 0x10 },
    Seg { obj: 0x8, len: 2, pos: 0x12 },
    Seg { obj: 0xA, len: 2, pos: 0x14 },
    Seg { obj: 0xC, len: 2, pos: 0x16 },
    Seg { obj: 0x18, len: 4, pos: 0x18 },
    Seg { obj: 0x1C, len: 4, pos: 0x1C },
    Seg { obj: 0x20, len: 4, pos: 0x20 },
    Seg { obj: 0x24, len: 4, pos: 0x24 },
    Seg { obj: 0x28, len: 4, pos: 0x28 },
    Seg { obj: 0x2C, len: 4, pos: 0x2C },
    Seg { obj: 0x30, len: 4, pos: 0x30 },
    Seg { obj: 0x34, len: 2, pos: 0x34 },
    Seg { obj: 0x36, len: 1, pos: 0x36 },
    Seg { obj: 0x37, len: 1, pos: 0x37 },
    Seg { obj: 0x38, len: 1, pos: 0x38 },
    Seg { obj: 0x39, len: 1, pos: 0x39 },
    Seg { obj: 0x3B, len: 1, pos: 0x3A },
    Seg { obj: 0x3A, len: 1, pos: 0x3B },
    Seg { obj: 0x3C, len: 2, pos: 0x3C },
    Seg { obj: 0x3E, len: 2, pos: 0x3E },
    Seg { obj: 0x40, len: 2, pos: 0x40 },
    Seg { obj: 0x42, len: 2, pos: 0x42 },
    Seg { obj: 0x44, len: 4, pos: 0x44 },
    Seg { obj: 0x48, len: 4, pos: 0x48 },
    Seg { obj: 0x4C, len: 4, pos: 0x4C },
    Seg { obj: 0x50, len: 4, pos: 0x50 },
];

/// Barco: resto del registro (`ship_serialize`, 0x005177D0), relativo al inicio de la sección que sigue al núcleo y su ruta.
pub const SHIP_REST_OBJECT_LEN: usize = 0x180;
/// Bytes fijos del registro en el fichero.
pub const SHIP_REST_RECORD_LEN: usize = 286;
pub const SHIP_REST: [Seg; 25] = [
    Seg { obj: 0x54, len: 96, pos: 0x0 },
    Seg { obj: 0xB4, len: 96, pos: 0x60 },
    Seg { obj: 0x114, len: 4, pos: 0xC0 },
    Seg { obj: 0x124, len: 4, pos: 0xC4 },
    Seg { obj: 0x128, len: 4, pos: 0xC8 },
    Seg { obj: 0x12C, len: 4, pos: 0xCC },
    Seg { obj: 0x130, len: 2, pos: 0xD0 },
    Seg { obj: 0x132, len: 2, pos: 0xD2 },
    Seg { obj: 0x136, len: 1, pos: 0xD4 },
    Seg { obj: 0x137, len: 1, pos: 0xD5 },
    Seg { obj: 0x13A, len: 1, pos: 0xD6 },
    Seg { obj: 0x13B, len: 1, pos: 0xD7 },
    Seg { obj: 0x138, len: 2, pos: 0xD8 },
    Seg { obj: 0x13C, len: 24, pos: 0xDA },
    Seg { obj: 0x154, len: 2, pos: 0xF2 },
    Seg { obj: 0x156, len: 1, pos: 0xF4 },
    Seg { obj: 0x157, len: 1, pos: 0xF5 },
    Seg { obj: 0x158, len: 1, pos: 0xF6 },
    Seg { obj: 0x159, len: 1, pos: 0xF7 },
    Seg { obj: 0x15A, len: 2, pos: 0xF8 },
    Seg { obj: 0x15C, len: 1, pos: 0xFA },
    Seg { obj: 0x15D, len: 1, pos: 0xFB },
    Seg { obj: 0x15F, len: 1, pos: 0xFC },
    Seg { obj: 0x15E, len: 1, pos: 0xFD },
    Seg { obj: 0x160, len: 32, pos: 0xFE },
];

/// Convoy (`convoy_serialize`, 0x005013D0).
pub const CONVOY_OBJECT_LEN: usize = 0x3C;
/// Bytes fijos del registro en el fichero.
pub const CONVOY_RECORD_LEN: usize = 58;
pub const CONVOY: [Seg; 19] = [
    Seg { obj: 0x0, len: 1, pos: 0x0 },
    Seg { obj: 0x1, len: 1, pos: 0x1 },
    Seg { obj: 0x2, len: 2, pos: 0x2 },
    Seg { obj: 0x4, len: 4, pos: 0x4 },
    Seg { obj: 0x8, len: 2, pos: 0x8 },
    Seg { obj: 0xA, len: 2, pos: 0xA },
    Seg { obj: 0xC, len: 4, pos: 0xC },
    Seg { obj: 0x10, len: 2, pos: 0x10 },
    Seg { obj: 0x12, len: 2, pos: 0x12 },
    Seg { obj: 0x14, len: 2, pos: 0x14 },
    Seg { obj: 0x16, len: 2, pos: 0x16 },
    Seg { obj: 0x18, len: 4, pos: 0x18 },
    Seg { obj: 0x1C, len: 2, pos: 0x1C },
    Seg { obj: 0x1E, len: 1, pos: 0x1E },
    Seg { obj: 0x1F, len: 1, pos: 0x1F },
    Seg { obj: 0x20, len: 4, pos: 0x20 },
    Seg { obj: 0x24, len: 16, pos: 0x24 },
    Seg { obj: 0x34, len: 4, pos: 0x34 },
    Seg { obj: 0x38, len: 2, pos: 0x38 },
];

/// Oficina completa (`office_serialize`, 0x004FED50) con su almacén de 0x2C0 bytes.
pub const OFFICE_OBJECT_LEN: usize = 0x44C;
/// Bytes fijos del registro en el fichero.
pub const OFFICE_RECORD_LEN: usize = 1100;
pub const OFFICE: [Seg; 39] = [
    Seg { obj: 0x2C4, len: 2, pos: 0x0 },
    Seg { obj: 0x0, len: 4, pos: 0x2 },
    Seg { obj: 0x4, len: 96, pos: 0x6 },
    Seg { obj: 0x64, len: 96, pos: 0x66 },
    Seg { obj: 0xC4, len: 96, pos: 0xC6 },
    Seg { obj: 0x124, len: 24, pos: 0x126 },
    Seg { obj: 0x13C, len: 384, pos: 0x13E },
    Seg { obj: 0x2BC, len: 4, pos: 0x2BE },
    Seg { obj: 0x2C0, len: 4, pos: 0x2C2 },
    Seg { obj: 0x2C8, len: 2, pos: 0x2C6 },
    Seg { obj: 0x2C6, len: 1, pos: 0x2C8 },
    Seg { obj: 0x2C7, len: 1, pos: 0x2C9 },
    Seg { obj: 0x2CA, len: 2, pos: 0x2CA },
    Seg { obj: 0x2CC, len: 2, pos: 0x2CC },
    Seg { obj: 0x2D2, len: 2, pos: 0x2CE },
    Seg { obj: 0x2D4, len: 2, pos: 0x2D0 },
    Seg { obj: 0x2CE, len: 2, pos: 0x2D2 },
    Seg { obj: 0x2DE, len: 6, pos: 0x2D4 },
    Seg { obj: 0x2D8, len: 6, pos: 0x2DA },
    Seg { obj: 0x2E4, len: 4, pos: 0x2E0 },
    Seg { obj: 0x2D0, len: 2, pos: 0x2E4 },
    Seg { obj: 0x2D6, len: 2, pos: 0x2E6 },
    Seg { obj: 0x2EC, len: 2, pos: 0x2E8 },
    Seg { obj: 0x2EE, len: 2, pos: 0x2EA },
    Seg { obj: 0x2F0, len: 2, pos: 0x2EC },
    Seg { obj: 0x2F2, len: 2, pos: 0x2EE },
    Seg { obj: 0x2F4, len: 96, pos: 0x2F0 },
    Seg { obj: 0x354, len: 96, pos: 0x350 },
    Seg { obj: 0x3B8, len: 96, pos: 0x3B0 },
    Seg { obj: 0x3B4, len: 4, pos: 0x410 },
    Seg { obj: 0x418, len: 8, pos: 0x414 },
    Seg { obj: 0x420, len: 8, pos: 0x41C },
    Seg { obj: 0x428, len: 8, pos: 0x424 },
    Seg { obj: 0x430, len: 8, pos: 0x42C },
    Seg { obj: 0x438, len: 8, pos: 0x434 },
    Seg { obj: 0x440, len: 8, pos: 0x43C },
    Seg { obj: 0x448, len: 2, pos: 0x444 },
    Seg { obj: 0x44A, len: 2, pos: 0x446 },
    Seg { obj: 0x2E8, len: 4, pos: 0x448 },
];

/// Estado del barco (`+0x134`), primer `u16` del registro de un barco en uso o libre.
pub const SHIP_STATE: Seg = Seg { obj: 0x134, len: 2, pos: 0 };
/// Un barco con estado `>= 0x17` está libre y ocupa 5 bytes: estado, siguiente libre (`+0x6`) y selector (`+0x137`).
pub const SHIP_FREE_FROM: u16 = 0x17;
/// Bytes del núcleo sin ruta y del resto, tras el estado de 2 bytes.
pub const SHIP_CORE_LEN: usize = 0x54;

/// Imagen del objeto (con ceros en lo que el guardado no contiene) a partir de un registro.
pub fn image(segs: &[Seg], object_len: usize, record: &[u8]) -> Vec<u8> {
    let mut out = vec![0u8; object_len];
    for s in segs {
        let (o, n, p) = (usize::from(s.obj), usize::from(s.len), usize::from(s.pos));
        out[o..o + n].copy_from_slice(&record[p..p + n]);
    }
    out
}
