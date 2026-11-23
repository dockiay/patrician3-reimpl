//! Estadísticas de comerciantes y ciudades tal como las guarda el original en memoria (`MerchantStatsHistory`, `TownStatsNode`,
//! `GameStatistics`, `0x54C4C0`–`0x54E250`).
//!
//! Disposición (medida en el ejecutable):
//! - Registro de comerciante: `0x6C` bytes (ver [`p3_core::economy::merchant_statistics`]). En los anillos cada registro ocupa `0x70`:
//!   el registro y en `+0x6C` el puntero al registro anterior.
//! - `MerchantStatsHistory` (`0x2310` bytes): `+0` comerciante (u16), `+2` día actual (u16, 0–31), `+4` semana actual (u8, 0–23), `+5` mes
//!   actual (u8, 0–23), `+6` años (u16), `+8` anillo diario de 32, `+0xE08` semanal de 24, `+0x1888` mensual de 24, `+0x2308` lista de años
//!   (nodos de `0x70` enlazados por `+0x6C`, el más reciente primero) y `+0x230C` siguiente histórico.
//! - `TownStatsNode` (`0xC` bytes): `+0` número de ciudades, `+4` datos (`0x204` bytes por ciudad), `+8` siguiente nodo.
//! - `GameStatistics`: `+0` contador de días (12 bits), `+2` semanas, `+4` meses, `+6`, `+8` años, `+0xA` históricos (u16); `+0xC` diario,
//!   `+0x10` semanal, `+0x14` mensual, `+0x18` anual (cadenas de nodos), `+0x1C` lista de históricos y `+0x20..+0x23` banderas de cambio.

use super::{free, memcpy, memset, new, read_vec, write_slice, Host, Mem, Ret};
use p3_core::economy::merchant_statistics as ms;

/// `merchant_stats_record_capture` (`0x54C520`, thiscall, 1 argumento): muestra del comerciante. Fuera de este porte.
pub const RECORD_CAPTURE: u32 = 0x0054_C520;
/// `town_stats_node_capture_snapshot` (`0x54D1D0`, thiscall): muestra de todas las ciudades. Fuera de este porte.
pub const TOWN_CAPTURE: u32 = 0x0054_D1D0;
/// `game_statistics_clear` (`0x54DDF0`, thiscall).
pub const STATS_CLEAR: u32 = 0x0054_DDF0;
/// `get_merchant` (`0x5303C0`, thiscall sobre `0x6DE4A0`, 1 argumento).
pub const GET_MERCHANT: u32 = 0x0053_03C0;

const REC: u32 = 0x6C;
/// Mundo: byte del día del mes (`0x6DE4A0`), del mes (`0x6DE4A1`), día del año (`0x6DE4A4`), comerciantes (`0x6DE4AA`), reloj (`0x6DE4B4`).
const WORLD: u32 = 0x006D_E4A0;

/// Días del mes que promedia el original: tabla de `.rdata` `0x672D92` (el mes 0 lee la entrada 12).
pub fn month_days(month: u8) -> u32 {
    const T: [u32; 13] = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    if month == 0 {
        T[12]
    } else {
        T[usize::from(month)]
    }
}

fn rec(m: &mut dyn Mem, a: u32) -> [u8; 0x6C] {
    read_vec(m, a, REC).try_into().unwrap()
}

/// `0x54C4C0`.
pub fn record_clear(m: &mut dyn Mem, rec: u32) {
    memset(m, rec, 0, REC);
}

/// `0x54C4D0`.
pub fn record_get_size() -> u32 {
    REC
}

/// `0x54C4E0`: copia el registro en `dst` y devuelve el final.
pub fn record_write(m: &mut dyn Mem, rec: u32, dst: u32) -> u32 {
    memcpy(m, dst, rec, REC);
    dst.wrapping_add(REC)
}

/// `0x54C500`: lee el registro de `src` (el segundo argumento no se usa) y devuelve el final.
pub fn record_read(m: &mut dyn Mem, rec: u32, src: u32) -> u32 {
    memcpy(m, rec, src, REC);
    src.wrapping_add(REC)
}

/// `0x54C810`.
pub fn record_accumulate(m: &mut dyn Mem, dst: u32, src: u32) {
    let mut d = rec(m, dst);
    let s = rec(m, src);
    ms::accumulate(&mut d, &s);
    write_slice(m, dst, &d);
}

/// `0x54C940`: el original solo usa los 16 bits bajos del recuento (y divide por cero si son 0).
pub fn record_average(m: &mut dyn Mem, dst: u32, count: u32) {
    let mut d = rec(m, dst);
    ms::average(&mut d, count as u16).expect("división por cero del original");
    // Solo escribe los campos que promedia: 0x08–0x0F, 0x14–0x47 quedan como sumas.
    for p in [0u32, 4, 0x10, 0x48, 0x4C, 0x50, 0x5C, 0x60, 0x64, 0x68] {
        m.w32(dst + p, u32::from_le_bytes(d[p as usize..p as usize + 4].try_into().unwrap()));
    }
    for p in [0x54u32, 0x56, 0x58, 0x5A] {
        m.w16(dst + p, u16::from_le_bytes([d[p as usize], d[p as usize + 1]]));
    }
}

/// `0x54CA20`: construye un histórico con la muestra actual del comerciante `owner` en todos los anillos.
pub fn history_ctor(m: &mut dyn Mem, h: &mut dyn Host, this: u32, owner: u32) -> u32 {
    let base = this.wrapping_add(8);
    m.w16(this, owner as u16);
    m.w16(this + 2, 0);
    m.w8(this + 4, 0);
    m.w8(this + 5, 0);
    m.w16(this + 6, 0);
    m.w32(this + 0x2308, 0);
    m.w32(this + 0x230C, 0);
    record_clear(m, base);
    h.call(m, RECORD_CAPTURE, base, &[owner & 0xFFFF]);
    for (start, count) in [(8u32, 32u32), (0xE08, 24), (0x1888, 24)] {
        let first = this + start;
        if start == 8 {
            m.w32(this + 0x74, this + 8 + 31 * 0x70);
        } else {
            record_clear(m, first);
            record_accumulate(m, first, base);
            m.w32(first + 0x6C, first + (count - 1) * 0x70);
        }
        for i in 1..count {
            let node = first + i * 0x70;
            record_clear(m, node);
            record_accumulate(m, node, base);
            m.w32(node, 0);
            m.w32(node + 0x6C, node - 0x70);
        }
    }
    this
}

/// `0x54CB40`: libera la lista de años y, recursivamente, los históricos siguientes.
pub fn history_free_chain(m: &mut dyn Mem, h: &mut dyn Host, this: u32) {
    loop {
        let year = m.r32(this + 0x2308);
        if year == 0 {
            break;
        }
        let next = m.r32(year + 0x6C);
        m.w32(this + 0x2308, next);
        free(m, h, year);
    }
    let next = m.r32(this + 0x230C);
    if next != 0 {
        history_free_chain(m, h, next);
        free(m, h, next);
    }
}

/// `0x54CB90`: cuenta los años en `+6` y devuelve el tamaño serializado.
pub fn history_get_save_size(m: &mut dyn Mem, this: u32) -> u32 {
    m.w16(this + 6, 0);
    let mut year = m.r32(this + 0x2308);
    while year != 0 {
        let c = m.r16(this + 6).wrapping_add(1);
        m.w16(this + 6, c);
        year = m.r32(year + 0x6C);
    }
    (u32::from(m.r16(this + 6)) + 0x50) * REC + 8
}

/// `0x54CBD0`: cabecera, los 80 registros de los anillos y los años. Si la lista es más corta que `+6`, el original repite el
/// primer registro diario.
pub fn history_serialize(m: &mut dyn Mem, this: u32, dst: u32) -> u32 {
    memcpy(m, dst, this, 2);
    memcpy(m, dst + 2, this + 2, 2);
    memcpy(m, dst + 4, this + 4, 1);
    memcpy(m, dst + 5, this + 5, 1);
    memcpy(m, dst + 6, this + 6, 2);
    let mut out = dst + 8;
    for (start, count) in [(8u32, 32u32), (0xE08, 24), (0x1888, 24)] {
        for i in 0..count {
            out = record_write(m, this + start + i * 0x70, out);
        }
    }
    let years = i32::from(m.r16(this + 6));
    let mut year = m.r32(this + 0x2308);
    for _ in 0..years.max(0) {
        if year != 0 {
            out = record_write(m, year, out);
            year = m.r32(year + 0x6C);
        } else {
            out = record_write(m, this + 8, out);
        }
    }
    out
}

/// `0x54CCC0`: inverso de [`history_serialize`]; sustituye la lista de años. `extra` es el segundo argumento, que el original pasa sin usar.
pub fn history_deserialize(m: &mut dyn Mem, h: &mut dyn Host, this: u32, src: u32, _extra: u32) -> u32 {
    memcpy(m, this, src, 2);
    memcpy(m, this + 2, src + 2, 2);
    memcpy(m, this + 4, src + 4, 1);
    memcpy(m, this + 5, src + 5, 1);
    memcpy(m, this + 6, src + 6, 2);
    let mut src = src + 8;
    for (start, count) in [(8u32, 32u32), (0xE08, 24), (0x1888, 24)] {
        for i in 0..count {
            src = record_read(m, this + start + i * 0x70, src);
        }
    }
    let years = u32::from(m.r16(this + 6));
    let slot = this + 0x2308;
    loop {
        let year = m.r32(slot);
        if year == 0 {
            break;
        }
        let next = m.r32(year + 0x6C);
        m.w32(slot, next);
        free(m, h, year);
    }
    let mut slot = slot;
    for _ in 0..years {
        let node = new(m, h, 0x70);
        m.w32(slot, node);
        if node != 0 {
            src = record_read(m, node, src);
            slot = node + 0x6C;
            m.w32(slot, 0);
        } else {
            src += REC;
        }
    }
    m.w32(this + 0x230C, 0);
    src
}

/// `0x54CE20`: avanza un día; con `week`, `month` y `year` (bytes) promedia los anillos semanal y mensual y antepone un año.
pub fn history_advance(m: &mut dyn Mem, h: &mut dyn Host, this: u32, week: u32, month: u32, year: u32) {
    let day = u32::from(m.r8(this + 2).wrapping_add(1)) & 0x1F;
    m.w16(this + 2, day as u16);
    let owner = u32::from(m.r16(this));
    h.call(m, RECORD_CAPTURE, this + 8 + day * 0x70, &[owner]);
    if week as u8 != 0 {
        let mut w = m.r8(this + 4).wrapping_add(1);
        if w > 0x17 {
            w = 0;
        }
        m.w8(this + 4, w);
        let dst = this + 0xE08 + u32::from(w) * 0x70;
        let mut src = this + 8 + u32::from(m.r16(this + 2)) * 0x70;
        record_clear(m, dst);
        for _ in 0..7 {
            record_accumulate(m, dst, src);
            src = m.r32(src + 0x6C);
        }
        record_average(m, dst, 7);
    }
    if month as u8 != 0 {
        let mut mo = m.r8(this + 5).wrapping_add(1);
        if mo > 0x17 {
            mo = 0;
        }
        m.w8(this + 5, mo);
        let dst = this + 0x1888 + u32::from(mo) * 0x70;
        record_clear(m, dst);
        let mut src = this + 8 + u32::from(m.r16(this + 2)) * 0x70;
        let days = month_days(m.r8(WORLD + 1));
        for _ in 0..days {
            record_accumulate(m, dst, src);
            src = m.r32(src + 0x6C);
        }
        record_average(m, dst, days);
    }
    if year as u8 != 0 {
        let node = new(m, h, 0x70);
        if node != 0 {
            let c = m.r16(this + 6).wrapping_add(1);
            m.w16(this + 6, c);
            record_clear(m, node);
            let mut src = this + 0x1888 + u32::from(m.r8(this + 5)) * 0x70;
            for _ in 0..12 {
                record_accumulate(m, node, src);
                src = m.r32(src + 0x6C);
            }
            record_average(m, node, 12);
            let head = m.r32(this + 0x2308);
            m.w32(node + 0x6C, head);
            m.w32(this + 0x2308, node);
        }
    }
}

/// `0x54D010`.
pub fn node_ctor(m: &mut dyn Mem, this: u32) -> u32 {
    m.w32(this + 8, 0);
    m.w32(this + 4, 0);
    m.w32(this, 0);
    this
}

/// `0x54D020`: libera los nodos siguientes y los datos (sin limpiar los campos).
pub fn node_free_chain(m: &mut dyn Mem, h: &mut dyn Host, this: u32) {
    let next = m.r32(this + 8);
    if next != 0 {
        node_free_chain(m, h, next);
        free(m, h, next);
    }
    let data = m.r32(this + 4);
    if data != 0 {
        free(m, h, data);
    }
}

/// `0x54D050`.
pub fn node_get_save_size(m: &mut dyn Mem, this: u32) -> u32 {
    let mut size = m.r32(this).wrapping_mul(0x204).wrapping_add(5);
    let next = m.r32(this + 8);
    if next != 0 {
        size = size.wrapping_add(node_get_save_size(m, next));
    }
    size
}

/// `0x54D080`: recuento y datos de cada nodo, con un byte 1 entre nodos y un 0 al final.
pub fn node_serialize(m: &mut dyn Mem, this: u32, dst: u32) -> u32 {
    let mut node = this;
    let mut out = dst;
    loop {
        let n = m.r32(node).wrapping_mul(0x204);
        memcpy(m, out, node, 4);
        let data = m.r32(node + 4);
        memcpy(m, out + 4, data, n);
        out = out.wrapping_add(4).wrapping_add(n);
        node = m.r32(node + 8);
        if node == 0 {
            break;
        }
        m.w8(out, 1);
        out += 1;
    }
    m.w8(out, 0);
    out + 1
}

/// `0x54D710` (`scalar deleting destructor`).
pub fn node_deleting_dtor(m: &mut dyn Mem, h: &mut dyn Host, this: u32, flags: u32) -> u32 {
    let next = m.r32(this + 8);
    if next != 0 {
        node_deleting_dtor(m, h, next, 1);
    }
    let data = m.r32(this + 4);
    if data != 0 {
        free(m, h, data);
    }
    if flags & 1 != 0 {
        free(m, h, this);
    }
    this
}

/// Destrucción del nodo siguiente que repiten `0x54D100`, `0x54D660` y `0x54D6C0`.
fn delete_node(m: &mut dyn Mem, h: &mut dyn Host, node: u32) {
    let next = m.r32(node + 8);
    if next != 0 {
        node_deleting_dtor(m, h, next, 1);
    }
    let data = m.r32(node + 4);
    if data != 0 {
        free(m, h, data);
    }
    free(m, h, node);
}

/// `0x54D100`: inverso de [`node_serialize`]; crea los nodos siguientes que hagan falta y destruye los que sobren.
pub fn node_deserialize(m: &mut dyn Mem, h: &mut dyn Host, this: u32, src: u32) -> u32 {
    let mut node = this;
    let mut src = src;
    loop {
        memcpy(m, node, src, 4);
        src += 4;
        let old = m.r32(node + 4);
        if old != 0 {
            free(m, h, old);
        }
        let count = m.r32(node);
        if count != 0 {
            let size = count.wrapping_mul(0x204);
            let data = new(m, h, size);
            m.w32(node + 4, data);
            if data != 0 {
                memcpy(m, data, src, size);
            } else {
                m.w32(node, 0);
            }
            src = src.wrapping_add(size);
        } else {
            m.w32(node + 4, 0);
        }
        let more = m.r8(src);
        src += 1;
        if more == 0 {
            break;
        }
        let next = new(m, h, 0xC);
        if next != 0 {
            node_ctor(m, next);
        }
        m.w32(node + 8, next);
        node = next;
    }
    let stale = m.r32(node + 8);
    if stale != 0 {
        delete_node(m, h, stale);
        m.w32(node + 8, 0);
    }
    src
}

/// `0x54D390`: promedia en `this` los `limit` primeros nodos de la cadena `src` (recuento de ciudades del primero si `this` está vacío).
pub fn node_average_chain(m: &mut dyn Mem, h: &mut dyn Host, this: u32, src: u32, limit: u32) {
    if src == 0 || limit == 0 {
        return;
    }
    if m.r32(this + 4) == 0 {
        let size = m.r32(src).wrapping_mul(0x204);
        let data = new(m, h, size);
        m.w32(this + 4, data);
        if data == 0 {
            m.w32(this, 0);
            return;
        }
        memset(m, data, 0, size);
        let c = m.r32(src);
        m.w32(this, c);
    }
    let mut count = 0u32;
    let mut src = src;
    let mut limit = limit;
    loop {
        if limit == 0 {
            break;
        }
        limit -= 1;
        let towns = m.r32(this).min(m.r32(src));
        if (towns as i32) > 0 {
            for k in (0..towns).rev() {
                let s = m.r32(src + 4) + k * 0x204;
                let d = m.r32(this + 4) + k * 0x204;
                for j in 0..0x18u32 {
                    for block in [0u32, 0x60, 0xC0, 0x120, 0x180] {
                        let p = block + 4 * j;
                        let v = m.r32(d + p).wrapping_add(m.r32(s + p));
                        m.w32(d + p, v);
                    }
                }
                let v = m.r32(d + 0x200) | m.r32(s + 0x200);
                m.w32(d + 0x200, v);
                for j in 0..4 {
                    let p = 0x1E0 + 4 * j;
                    let v = m.r32(d + p).wrapping_add(m.r32(s + p));
                    m.w32(d + p, v);
                }
                for j in 0..3 {
                    for p in [0x1F0 + 2 * j, 0x1F6 + 2 * j] {
                        let v = m.r16(d + p).wrapping_add(m.r16(s + p));
                        m.w16(d + p, v);
                    }
                }
                let v = m.r32(d + 0x1FC).wrapping_add(m.r32(s + 0x1FC));
                m.w32(d + 0x1FC, v);
            }
        }
        src = m.r32(src + 8);
        count += 1;
        if src == 0 {
            break;
        }
    }
    let n = count as i32;
    for i in 0..m.r32(this) {
        let d = m.r32(this + 4) + i * 0x204;
        for j in 0..0x78 {
            let v = (m.r32(d + 4 * j) as i32) / n;
            m.w32(d + 4 * j, v as u32);
        }
        let flags = m.r32(d + 0x200) & 0xFBFF_FFF9;
        m.w32(d + 0x200, flags);
        for j in 0..4 {
            let v = m.r32(d + 0x1E0 + 4 * j) / count;
            m.w32(d + 0x1E0 + 4 * j, v);
        }
        for j in 0..3 {
            for p in [0x1F0 + 2 * j, 0x1F6 + 2 * j] {
                let v = i32::from(m.r16(d + p) as i16) / n;
                m.w16(d + p, v as u16);
            }
        }
    }
}

/// `0x54D5F0`: copia los datos de `src` (no el siguiente).
pub fn node_copy_data_from(m: &mut dyn Mem, h: &mut dyn Host, this: u32, src: u32) {
    let old = m.r32(this + 4);
    if old != 0 {
        free(m, h, old);
        m.w32(this + 4, 0);
    }
    if m.r32(src + 4) != 0 {
        let count = m.r32(src);
        let data = new(m, h, count.wrapping_mul(0x204));
        m.w32(this + 4, data);
        if data != 0 {
            let count = m.r32(src);
            m.w32(this, count);
            let from = m.r32(src + 4);
            memcpy(m, data, from, count.wrapping_mul(0x204));
            return;
        }
    }
    m.w32(this + 4, 0);
}

/// `0x54D660`: conserva el nodo y los `keep` siguientes y destruye el resto.
pub fn node_truncate_chain(m: &mut dyn Mem, h: &mut dyn Host, this: u32, keep: u32) {
    let mut node = this;
    let mut keep = keep;
    if m.r32(node + 8) == 0 {
        return;
    }
    loop {
        if keep == 0 {
            break;
        }
        node = m.r32(node + 8);
        keep -= 1;
        if m.r32(node + 8) == 0 {
            return;
        }
    }
    let next = m.r32(node + 8);
    if next != 0 {
        delete_node(m, h, next);
    }
    m.w32(node + 8, 0);
}

/// `0x54D6C0`: destruye el siguiente actual y enlaza `next`.
pub fn node_replace_next(m: &mut dyn Mem, h: &mut dyn Host, this: u32, next: u32) {
    let old = m.r32(this + 8);
    if old != 0 {
        delete_node(m, h, old);
    }
    m.w32(this + 8, next);
}

/// `0x54D7F0`: tamaño del `.pst`; cuenta los históricos en `+0xA` (suma un byte de más tras la cadena mensual, como el original).
pub fn game_get_save_size(m: &mut dyn Mem, this: u32) -> u32 {
    let mut size = 0x24u32;
    let c = m.r32(this + 0xC);
    if c != 0 {
        size = node_get_save_size(m, c).wrapping_add(0x24);
    }
    size = size.wrapping_add(1);
    let c = m.r32(this + 0x10);
    if c != 0 {
        size = size.wrapping_add(node_get_save_size(m, c));
    }
    size = size.wrapping_add(1);
    let c = m.r32(this + 0x14);
    if c != 0 {
        size = size.wrapping_add(node_get_save_size(m, c)).wrapping_add(1);
    }
    size = size.wrapping_add(1);
    let c = m.r32(this + 0x18);
    if c != 0 {
        size = size.wrapping_add(node_get_save_size(m, c));
    }
    size = size.wrapping_add(1);
    m.w16(this + 0xA, 0);
    let mut item = m.r32(this + 0x1C);
    while item != 0 {
        let c = m.r16(this + 0xA).wrapping_add(1);
        m.w16(this + 0xA, c);
        size = size.wrapping_add(history_get_save_size(m, item));
        item = m.r32(item + 0x230C);
    }
    size
}

/// `0x54DA60`: lee el `.pst` (marca `0xAFFE000B`). `esp` es la pila de entrada del original (apunta a la dirección de vuelta): el
/// original construye un histórico temporal en `esp - 0x231C` (que solo recibe la muestra y se libera) y pasa a cada histórico leído el
/// valor `0xB` como segundo argumento. Si la marca no coincide, limpia las estadísticas (`0x54DDF0`) y devuelve `src` sin avanzar.
pub fn game_deserialize(m: &mut dyn Mem, h: &mut dyn Host, this: u32, src: u32, esp: u32) -> u32 {
    let temp = esp.wrapping_sub(0x231C);
    history_ctor(m, h, temp, 0);
    let magic = m.r32(src);
    let mut src = src + 4;
    if magic.wrapping_add(0x5002_0000) != 0xB {
        h.call(m, STATS_CLEAR, this, &[]);
        history_free_chain(m, h, temp);
        return src - 4;
    }
    for off in [0u32, 2, 4, 6, 8, 0xA] {
        memcpy(m, this + off, src, 2);
        src += 2;
    }
    for (i, off) in [0xCu32, 0x10, 0x14, 0x18].into_iter().enumerate() {
        if i > 0 {
            src += 4;
        }
        let old = m.r32(this + off);
        if old != 0 {
            node_free_chain(m, h, old);
            free(m, h, old);
            m.w32(this + off, 0);
        }
        let present = m.r8(src);
        src += 1;
        if present == 1 {
            let node = new(m, h, 0xC);
            if node != 0 {
                node_ctor(m, node);
            }
            m.w32(this + off, node);
            src = node_deserialize(m, h, node, src);
        }
    }
    src += 4;
    let old = m.r32(this + 0x1C);
    if old != 0 {
        history_free_chain(m, h, old);
        free(m, h, old);
        m.w32(this + 0x1C, 0);
    }
    let mut slot = this + 0x1C;
    let mut i = 0i32;
    while i < i32::from(m.r16(this + 0xA)) {
        let item = new(m, h, 0x2310);
        let item = if item != 0 { history_ctor(m, h, item, 0) } else { 0 };
        m.w32(slot, item);
        if item != 0 {
            src = history_deserialize(m, h, item, src, 0xB);
            slot = item + 0x230C;
        } else {
            // El original lee entonces sobre `esp - 0x2324` (8 bytes antes del temporal); con el asignador real no ocurre.
            src = history_deserialize(m, h, temp.wrapping_sub(8), src, 0xB);
        }
        i += 1;
    }
    src += 4;
    history_free_chain(m, h, temp);
    src
}

/// `0x54DE90`: muestra diaria de ciudades y comerciantes; promedios semanal (día 6 de la semana), mensual (día 1) y anual (día 1 del año).
pub fn game_daily_update(m: &mut dyn Mem, h: &mut dyn Host, this: u32) {
    let weekday = (m.r32(WORLD + 0x14) >> 8) % 7;
    let mut year_flag = 0u8;
    let node = new(m, h, 0xC);
    if node != 0 {
        node_ctor(m, node);
        h.call(m, TOWN_CAPTURE, node, &[]);
        let head = m.r32(this + 0xC);
        node_replace_next(m, h, node, head);
        m.w32(this + 0xC, node);
        node_truncate_chain(m, h, node, 0x1F);
    }
    let month_flag;
    if m.r8(WORLD) == 1 {
        month_flag = 1u8;
        let node = new(m, h, 0xC);
        if node != 0 {
            node_ctor(m, node);
            let days = month_days(m.r8(WORLD + 1));
            let daily = m.r32(this + 0xC);
            node_average_chain(m, h, node, daily, days);
            let head = m.r32(this + 0x14);
            node_replace_next(m, h, node, head);
            m.w32(this + 0x14, node);
            node_truncate_chain(m, h, node, 0x18);
            if m.r16(WORLD + 4) == 0 {
                year_flag = 1;
                let year = new(m, h, 0xC);
                if year != 0 {
                    node_ctor(m, year);
                    let monthly = m.r32(this + 0x14);
                    node_average_chain(m, h, year, monthly, 12);
                    let head = m.r32(this + 0x18);
                    node_replace_next(m, h, year, head);
                    let c = m.r16(this + 8).wrapping_add(1);
                    m.w16(this + 8, c);
                    m.w32(this + 0x18, year);
                }
            }
        }
        let c = m.r16(this + 4);
        if c < 0x18 {
            m.w16(this + 4, c + 1);
        }
    } else {
        month_flag = 0;
    }
    if weekday == 6 {
        let node = new(m, h, 0xC);
        if node != 0 {
            node_ctor(m, node);
            let daily = m.r32(this + 0xC);
            node_average_chain(m, h, node, daily, 7);
            let head = m.r32(this + 0x10);
            node_replace_next(m, h, node, head);
            m.w32(this + 0x10, node);
            node_truncate_chain(m, h, node, 0x18);
        }
        let c = m.r16(this + 2);
        if c < 0x18 {
            m.w16(this + 2, c + 1);
        }
    }
    if m.r32(this + 0x1C) == 0 {
        let mut owner = i32::from(m.r16(WORLD + 0xA));
        while owner > 0 {
            owner -= 1;
            let merchant = h.call(m, GET_MERCHANT, WORLD, &[owner as u32]);
            if m.r8(merchant + 8) & 4 != 0 {
                continue;
            }
            let mut item = m.r32(this + 0x1C);
            let mut found = false;
            while item != 0 {
                if i32::from(m.r16(item)) == owner {
                    found = true;
                    break;
                }
                item = m.r32(item + 0x230C);
            }
            if found {
                continue;
            }
            let item = new(m, h, 0x2310);
            if item != 0 {
                history_ctor(m, h, item, owner as u32);
                let head = m.r32(this + 0x1C);
                m.w32(item + 0x230C, head);
                m.w32(this + 0x1C, item);
            }
        }
    }
    let mut item = m.r32(this + 0x1C);
    while item != 0 {
        history_advance(m, h, item, u32::from(weekday == 0), u32::from(month_flag), u32::from(year_flag));
        item = m.r32(item + 0x230C);
    }
    let v = m.r8(this + 0x21) | u8::from(weekday == 0);
    m.w8(this + 0x21, v);
    m.w8(this + 0x20, 1);
    let v = m.r8(this + 0x22) | month_flag;
    m.w8(this + 0x22, v);
    let v = m.r8(this + 0x23) | year_flag;
    m.w8(this + 0x23, v);
    let day = m.r16(this).wrapping_add(1) & 0xFFF;
    m.w16(this, day);
}

/// `0x54E170`: añade el histórico del comerciante si no existe.
pub fn game_add_merchant_history(m: &mut dyn Mem, h: &mut dyn Host, this: u32, owner: u32) {
    let mut item = m.r32(this + 0x1C);
    while item != 0 {
        if m.r16(item) == owner as u16 {
            return;
        }
        item = m.r32(item + 0x230C);
    }
    let item = new(m, h, 0x2310);
    if item != 0 {
        let item = history_ctor(m, h, item, owner);
        if item != 0 {
            let head = m.r32(this + 0x1C);
            m.w32(item + 0x230C, head);
            m.w32(this + 0x1C, item);
        }
    }
}

/// `0x54E1F0`: quita y libera el histórico del comerciante (solo si es menor que el número de comerciantes del mundo).
pub fn game_remove_merchant_history(m: &mut dyn Mem, h: &mut dyn Host, this: u32, owner: u32) {
    let owner = owner as u16;
    if owner >= m.r16(WORLD + 0xA) {
        return;
    }
    let mut slot = this + 0x1C;
    loop {
        let item = m.r32(slot);
        if item == 0 {
            return;
        }
        if m.r16(item) == owner {
            let next = m.r32(item + 0x230C);
            m.w32(slot, next);
            m.w32(item + 0x230C, 0);
            history_free_chain(m, h, item);
            free(m, h, item);
            return;
        }
        slot = item + 0x230C;
    }
}

/// `0x54E250`: cuenta los nodos anuales en `+8` y lo devuelve.
pub fn game_count_yearly_nodes(m: &mut dyn Mem, this: u32) -> u32 {
    m.w16(this + 8, 0);
    let mut node = m.r32(this + 0x18);
    while node != 0 {
        let c = m.r16(this + 8).wrapping_add(1);
        m.w16(this + 8, c);
        node = m.r32(node + 8);
    }
    u32::from(m.r16(this + 8))
}

/// Tabla de portes de este módulo (direcciones del original).
pub fn run(f: u32, m: &mut dyn Mem, h: &mut dyn Host, ecx: u32, a: &[u32], esp: u32) -> Option<Ret> {
    match f {
        0x54C4C0 => record_clear(m, ecx),
        0x54C4D0 => return Some(Ret::U32(record_get_size())),
        0x54C4E0 => return Some(Ret::U32(record_write(m, ecx, a[0]))),
        0x54C500 => return Some(Ret::U32(record_read(m, ecx, a[0]))),
        0x54C810 => record_accumulate(m, ecx, a[0]),
        0x54C940 => record_average(m, ecx, a[0]),
        0x54CA20 => return Some(Ret::U32(history_ctor(m, h, ecx, a[0]))),
        0x54CB40 => history_free_chain(m, h, ecx),
        0x54CB90 => return Some(Ret::U32(history_get_save_size(m, ecx))),
        0x54CBD0 => return Some(Ret::U32(history_serialize(m, ecx, a[0]))),
        0x54CCC0 => return Some(Ret::U32(history_deserialize(m, h, ecx, a[0], a[1]))),
        0x54CE20 => history_advance(m, h, ecx, a[0], a[1], a[2]),
        0x54D010 => return Some(Ret::U32(node_ctor(m, ecx))),
        0x54D020 => node_free_chain(m, h, ecx),
        0x54D050 => return Some(Ret::U32(node_get_save_size(m, ecx))),
        0x54D080 => return Some(Ret::U32(node_serialize(m, ecx, a[0]))),
        0x54D100 => return Some(Ret::U32(node_deserialize(m, h, ecx, a[0]))),
        0x54D390 => node_average_chain(m, h, ecx, a[0], a[1]),
        0x54D5F0 => node_copy_data_from(m, h, ecx, a[0]),
        0x54D660 => node_truncate_chain(m, h, ecx, a[0]),
        0x54D6C0 => node_replace_next(m, h, ecx, a[0]),
        0x54D710 => return Some(Ret::U32(node_deleting_dtor(m, h, ecx, a[0]))),
        0x54D7F0 => return Some(Ret::U32(game_get_save_size(m, ecx))),
        0x54DA60 => return Some(Ret::U32(game_deserialize(m, h, ecx, a[0], esp))),
        0x54DE90 => game_daily_update(m, h, ecx),
        0x54E170 => game_add_merchant_history(m, h, ecx, a[0]),
        0x54E1F0 => game_remove_merchant_history(m, h, ecx, a[0]),
        0x54E250 => return Some(Ret::U16(game_count_yearly_nodes(m, ecx) as u16)),
        _ => return None,
    }
    Some(Ret::None)
}

