//! Almacén de registros de edificios (`Town+0x75C`) y listas que los recorren.

use super::{Site, NIL};
use crate::buildings::Grid;
use crate::town::{off, Town};

/// Registros añadidos cada vez que el almacén se llena.
const GROWTH: u16 = 0x80;

impl Site {
    pub fn record_count(t: &Town) -> usize {
        t.u16(off::BUILDING_RECORD_COUNT) as usize
    }
    pub fn rec(&self, i: usize, k: usize) -> u8 {
        self.records[i * 8 + k]
    }
    pub fn set_rec(&mut self, i: usize, k: usize, v: u8) {
        self.records[i * 8 + k] = v;
    }
    /// Enlace `+4` del registro.
    pub fn next(&self, i: usize) -> u16 {
        u16::from_le_bytes([self.rec(i, 4), self.rec(i, 5)])
    }
    pub fn set_next(&mut self, i: usize, v: u16) {
        let b = v.to_le_bytes();
        self.set_rec(i, 4, b[0]);
        self.set_rec(i, 5, b[1]);
    }

    /// `town_alloc_building_record` (0x0051F100): saca el primer registro libre. Si no queda ninguno
    /// (la cabeza libre es el recuento) amplía el almacén en 0x80 registros, todos libres y
    /// encadenados.
    pub fn alloc_building_record(&mut self, t: &mut Town) -> u16 {
        let free = t.u16(off::FREE_BUILDING_HEAD);
        let count = t.u16(off::BUILDING_RECORD_COUNT);
        if free == count {
            let count = count.wrapping_add(GROWTH);
            t.set_u16(off::BUILDING_RECORD_COUNT, count);
            self.records.resize(count as usize * 8, 0xFF);
            let mut a = count;
            while a > free {
                a -= 1;
                self.set_next(a as usize, a + 1);
            }
        }
        let i = t.u16(off::FREE_BUILDING_HEAD) as usize;
        let next = self.next(i);
        t.set_u16(off::FREE_BUILDING_HEAD, next);
        self.set_next(i, NIL);
        i as u16
    }

    /// `town_get_building_record` (0x0051F1F0): `None` si el índice no existe.
    pub fn get_building_record(&self, t: &Town, i: u32) -> Option<usize> {
        (i < Self::record_count(t) as u32).then_some(i as usize)
    }

    /// `town_free_building_record_if_unlinked` (0x0051F050): libera el registro `i` solo si ninguna
    /// lista lo enlaza ni es ya la cabeza libre.
    pub fn free_building_record_if_unlinked(&mut self, t: &mut Town, i: u32) {
        let count = Self::record_count(t);
        let free = t.u16(off::FREE_BUILDING_HEAD);
        if i as usize >= count || free as u32 == i {
            return;
        }
        if (0..count).any(|j| self.next(j) as u32 == i) {
            return;
        }
        let i = i as usize;
        for k in 0..4 {
            self.set_rec(i, k, 0xFF);
        }
        self.set_next(i, free);
        self.set_rec(i, 6, 0);
        self.set_rec(i, 7, 0);
        t.set_u16(off::FREE_BUILDING_HEAD, i as u16);
    }

    /// `town_free_building_record_chain` (0x00521240): libera la lista que empieza en `i`, quitando de
    /// la rejilla la huella de cada edificio. `special` es el byte global `0x0067792F`.
    pub fn free_building_record_chain(&mut self, t: &mut Town, i: u32, special: u8) {
        let count = Self::record_count(t);
        if (i as u16) as usize >= count {
            return;
        }
        let mut i = (i as u16) as usize;
        loop {
            let next = self.next(i);
            self.set_next(i, t.u16(off::FREE_BUILDING_HEAD));
            t.set_u16(off::FREE_BUILDING_HEAD, i as u16);
            let (row, col, ty, extra) = (self.rec(i, 0), self.rec(i, 1), self.rec(i, 3), self.rec(i, 7));
            let mut grid = Grid {
                stride: t.i32(off::PLAN_STRIDE),
                rows: t.i32(off::PLAN_ROWS),
                cells: &mut self.cells,
            };
            grid.clear_building_footprint(row as u32, col as u32, ty as u32, extra as u32, special);
            for k in 0..4 {
                self.set_rec(i, k, 0xFF);
            }
            self.set_rec(i, 6, 0);
            self.set_rec(i, 7, 0);
            if next as usize >= count {
                break;
            }
            i = next as usize;
        }
    }

    /// `town_get_construction_head` (0x004F3630).
    pub fn construction_head(t: &Town) -> u16 {
        t.u16(off::CONSTRUCTION_HEAD)
    }

    /// `town_free_building_records` (0x0051E8A0): suelta el almacén y las celdas del plano.
    pub fn free_building_records(&mut self) {
        self.records = Vec::new();
        self.cells = Vec::new();
    }
}
