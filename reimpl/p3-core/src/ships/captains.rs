//! Lista libre de capitanes (`ships::allocate_captain`, `ships::free_captain`).
//!
//! El array de capitanes tiene 16 bytes por entrada; el primer `u16` de una entrada libre enlaza con la
//! siguiente libre (la última apunta a `capacity`). `+0xE0` del gestor es la cabeza de la lista libre y
//! `+0xF2` la capacidad. Crece de 0x40 en 0x40 entradas cuando no quedan libres.

use super::{r16, w16};

pub const ENTRY: usize = 16;
pub const GROWTH: u16 = 0x40;

/// Lo que pide la asignación fuera de la lista: iniciar el registro (`Captain::init`, 0x004FDF50).
pub trait CaptainHost {
    fn init(&mut self, index: u16, random_skill: bool);
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CaptainPool {
    /// `*(gestor)`: el array, vacío mientras no se ha reservado.
    pub entries: Vec<u8>,
    /// `+0xF2`.
    pub capacity: u16,
    /// `+0xE0`.
    pub free_head: u16,
}

impl CaptainPool {
    /// `ships::allocate_captain` (0x005097C0). `flag` es el argumento `char` del original; el registro se
    /// inicia con `random_skill = (flag == 0)`. Con la lista libre agotada (`free_head == capacity`) el array
    /// crece 0x40 entradas, relleno de 0xFF salvo los enlaces nuevos.
    pub fn allocate(&mut self, flag: u8, host: &mut dyn CaptainHost) -> u16 {
        if self.capacity <= self.free_head {
            self.capacity = self.capacity.wrapping_add(GROWTH);
            let mut grown = vec![0xFFu8; self.capacity as usize * ENTRY];
            if !self.entries.is_empty() {
                let n = self.free_head as usize * ENTRY;
                grown[..n].copy_from_slice(&self.entries[..n]);
            }
            for i in self.free_head..self.capacity {
                w16(&mut grown, i as usize * ENTRY, i + 1);
            }
            self.entries = grown;
        }
        let index = self.free_head;
        let at = index as usize * ENTRY;
        self.free_head = r16(&self.entries, at);
        w16(&mut self.entries, at, 0xFFFF);
        host.init(index, flag == 0);
        index
    }

    /// `ships::free_captain` (0x005098B0): devuelve el capitán `*slot` a la lista libre y deja `0xFFFF` en el
    /// campo. Si ya está libre solo borra el campo; fuera de la capacidad no hace nada.
    pub fn free(&mut self, slot: &mut u16) {
        let index = *slot;
        if index >= self.capacity {
            return;
        }
        let mut cursor = self.free_head;
        while cursor < self.capacity {
            if cursor == index {
                *slot = 0xFFFF;
                return;
            }
            cursor = r16(&self.entries, cursor as usize * ENTRY);
        }
        let at = index as usize * ENTRY;
        *slot = r16(&self.entries, at);
        w16(&mut self.entries, at, self.free_head);
        self.free_head = index;
    }
}
