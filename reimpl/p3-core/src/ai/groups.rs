//! Grupos de barcos del comerciante de la IA: fijar un grupo (0x004F5500) y añadir barcos (0x004F56C0). Referencia: x86 original.

/// Operación de red/orden de 0x0054AA70: cinco dwords.
pub const OP_ADD_SHIPS_GROUP: u32 = 0xBF;

/// Lista de barcos de un grupo: recuento `+0x60C+2*slot` y puntero `+0x624+4*slot` del comerciante.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GroupSlot {
    pub count: u16,
    pub ptr: u32,
}

/// Asignador, memoria de listas de palabras, propietario de barco (byte `+0` de la tabla 0x006DD7A4) y cola de órdenes.
pub trait GroupHost {
    fn alloc(&mut self, size: u32) -> u32;
    fn free(&mut self, ptr: u32);
    fn read_words(&self, ptr: u32, n: usize) -> Vec<u16>;
    fn write_words(&mut self, ptr: u32, word_offset: usize, words: &[u16]);
    fn ship_owner(&self, id: u16) -> u8;
    fn enqueue_operation(&mut self, record: &[u32; 5]);
}

/// Descarta (sustituyendo por el último) los barcos que no son del comerciante; `count` baja a medida que se quitan.
pub(crate) fn filter_own(words: &mut [u16], count: &mut u16, merchant: u8, host: &dyn GroupHost) {
    let mut i = 0usize;
    while i < *count as usize {
        if host.ship_owner(words[i]) != merchant {
            *count -= 1;
            words[i] = words[*count as usize];
        } else {
            i += 1;
        }
    }
}

/// `0x004F5500` (thiscall, `ret 0x14`; solo se leen tres argumentos): sustituye el grupo `slot` por la lista dada
/// (`count` palabras bajas), conserva solo los barcos de `merchant` y libera el puntero si queda vacío.
pub fn set_ship_group(slot: &mut GroupSlot, merchant: u8, list: &[u16], count: u16, host: &mut dyn GroupHost) {
    if slot.ptr != 0 {
        host.free(slot.ptr);
    }
    slot.ptr = 0;
    slot.count = count;
    let ptr = host.alloc(2 * count as u32);
    slot.ptr = ptr;
    let mut words = list[..count as usize].to_vec();
    host.write_words(ptr, 0, &words);
    filter_own(&mut words, &mut slot.count, merchant, host);
    host.write_words(ptr, 0, &words[..slot.count as usize]);
    if slot.count == 0 {
        host.free(slot.ptr);
        slot.ptr = 0;
    }
}

/// `0x004F56C0` (thiscall, `ret 0x10`): filtra en el sitio la lista del llamador, emite si `emit` las órdenes 0xBF (siete
/// barcos por orden, desde el final) y añade los barcos supervivientes al grupo `slot`. `slot_index` es el número de grupo.
pub fn add_ships_to_group(
    slot: &mut GroupSlot,
    slot_index: u8,
    merchant: u8,
    list: &mut [u16],
    count: u16,
    emit: bool,
    host: &mut dyn GroupHost,
) {
    let mut count = count;
    filter_own(list, &mut count, merchant, host);
    if count == 0 {
        return;
    }
    if emit {
        let mut idx = merchant;
        let mut esi = count as i32;
        loop {
            let mut w = [0u16; 7];
            for x in w.iter_mut() {
                esi -= 1;
                *x = if esi < 0 { 0xFFFF } else { list[esi as usize] };
            }
            host.enqueue_operation(&[
                OP_ADD_SHIPS_GROUP,
                idx as u32 | (slot_index as u32) << 8 | (w[0] as u32) << 16,
                w[1] as u32 | (w[2] as u32) << 16,
                w[3] as u32 | (w[4] as u32) << 16,
                w[5] as u32 | (w[6] as u32) << 16,
            ]);
            idx &= 0x7F;
            if esi <= 0 {
                break;
            }
        }
    }
    let old = slot.count as u32;
    let total = old + count as u32;
    let ptr = host.alloc(2 * total);
    let old_words = host.read_words(slot.ptr, old as usize);
    host.write_words(ptr, 0, &old_words);
    host.write_words(ptr, old as usize, &list[..count as usize]);
    if slot.ptr != 0 {
        host.free(slot.ptr);
    }
    slot.ptr = ptr;
    slot.count = total as u16;
}

/// `0x005303E0`: índice de un elemento de 0x650 bytes (división por el número mágico `0x288DF0CB`, `imul` con signo).
pub fn merchant_index(element: u32, table_base: u32) -> i32 {
    let diff = element.wrapping_sub(table_base) as i32;
    let hi = ((diff as i64 * 0x288D_F0CB_i64) >> 32) as i32;
    let q = hi >> 8;
    q + ((q as u32) >> 31) as i32
}
