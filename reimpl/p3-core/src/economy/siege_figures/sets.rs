//! Conjuntos de sprites de las clases del asalto en sus listas globales: alta de un conjunto nuevo
//! (`entity_sprite_set_list_append_new`, `0x005629D0`, con el constructor `0x00558FC0`) y tablas por conjunto que
//! rellena la cabecera extra del `.lcr` (`0x00577710`, `0x005672B0`, `0x00562280`, `0x00572960`, `0x00579B40`,
//! `0x00564660`; los proyectiles copian un registro de 12 bytes con `0x005773C0`).
//!
//! El cargador de cada clase (método `+0x10`: `0x00577750`, `0x00567380`, ...) da de alta el conjunto y lo carga
//! con `0x00558770`; la carga del `.lcr` en memoria la hace quien llama (`p3_render::sprite_set::load_set`).
use super::FigureHost;
use crate::flatmem::Mem;

/// Clase de la fábrica, lista `{_, cuántos, conjuntos}` y tablas de un byte por conjunto (en el orden de la cabecera).
pub const CLASS_SETS: &[(u8, u32, &[u32])] = &[
    (0x08, 0x006E_2E54, &[0x006E_2E60]),
    (0x09, 0x006E_1748, &[0x006E_1754, 0x006E_1758, 0x006E_175C, 0x006E_1760]),
    (0x0A, 0x006E_16DC, &[0x006E_16E8, 0x006E_16EC, 0x006E_16F0, 0x006E_16F4]),
    (0x0B, 0x006E_2DCC, &[0x006E_2DD8, 0x006E_2DDC, 0x006E_2DE0, 0x006E_2DE4]),
    (0x12, 0x006E_2E68, &[0x006E_2E74]),
    (0x15, 0x006E_1730, &[0x006E_173C, 0x006E_1740]),
    (0x16, 0x006E_2E10, &[]),
    (0x17, 0x006E_2E40, &[]),
    (0x20, 0x006E_2DA8, &[]),
    (0x23, 0x006E_1850, &[]),
    (0x24, 0x006E_2D88, &[]),
];

/// Registros de 12 bytes de los proyectiles (`{clase, altura base, variación}` por conjunto).
pub const PROJECTILE_RECORDS: u32 = 0x006E_2E4C;

fn class_entry(class: u32) -> Option<&'static (u8, u32, &'static [u32])> {
    CLASS_SETS.iter().find(|c| u32::from(c.0) == class)
}

/// Lista de conjuntos de la clase.
pub fn class_list(class: u32) -> Option<u32> {
    class_entry(class).map(|c| c.1)
}

/// `0x005629D0(ecx = lista)`: un conjunto más (reserva de `0x40` bytes con el constructor `0x00558FC0`). Devuelve el
/// conjunto, ya apuntado en la última posición.
pub fn append_set(m: &mut dyn Mem, h: &mut dyn FigureHost, list: u32) -> u32 {
    let n = m.r32(list + 4) + 1;
    m.w32(list + 4, n);
    let old = m.r32(list + 8);
    let arr = h.realloc(m, old, n * 4);
    m.w32(list + 8, arr);
    let set = h.alloc(m, 0x40);
    if set != 0 {
        for i in 0..0x40 {
            m.w8(set + i, 0);
        }
        m.w32(set, 0x0067_4B5C);
    }
    m.w32(arr + 4 * (n - 1), set);
    set
}

/// Valor de la tabla `table` de la cabecera extra para el último conjunto de la clase (amplía la tabla con `realloc`).
pub fn push_table(m: &mut dyn Mem, h: &mut dyn FigureHost, class: u32, table: usize, value: u8) {
    let Some(&(_, list, tables)) = class_entry(class) else { return };
    let Some(&at) = tables.get(table) else { return };
    let n = m.r32(list + 4);
    let old = m.r32(at);
    let p = h.realloc(m, old, n);
    m.w32(at, p);
    if n != 0 {
        m.w8(p + n - 1, value);
    }
}

/// Registro de 12 bytes del último proyectil (`0x00577400` amplía la tabla y `0x005773C0` copia el registro).
pub fn push_projectile_record(m: &mut dyn Mem, h: &mut dyn FigureHost, record: [u8; 12]) {
    let n = m.r32(0x006E_2E44);
    if n == 0 {
        return;
    }
    let old = m.r32(PROJECTILE_RECORDS);
    let p = h.realloc(m, old, n * 12);
    m.w32(PROJECTILE_RECORDS, p);
    for (i, &b) in record.iter().enumerate() {
        m.w8(p + (n - 1) * 12 + i as u32, b);
    }
}
