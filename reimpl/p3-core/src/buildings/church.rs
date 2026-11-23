//! Mejora de iglesia (C05): `town_start_church_upgrade_construction` (0x00525390) saca de la lista pública el
//! primer edificio de tipo `0x1F`, lo vuelve obra sin propietario y lo encola en la lista de construcción.

use super::tables::RECORD_BYTE_6;
use super::Site;
use crate::town::{off, Town};

const CHURCH: u8 = 0x1F;
/// Byte global `0x00672FDF` (clase del objeto del mapa visual).
const MAP_CLASS: i8 = 4;

/// Avisos al mapa visual (class22).
pub trait ChurchHost {
    /// `class22_add_map_object` (0x00590680): (fila, columna, clase, `Town+0x7A3`, 0xFF).
    fn add_map_object(&mut self, row: u32, col: u32, class: u32, packed: u32, owner: u32);
    /// `class22_set_building_progress_at_tile` (0x005891C0): (fila, columna, 0, byte 6, 0).
    fn set_progress(&mut self, row: u32, col: u32, done: u32, total: u32, flag: u32);
}

impl Site {
    /// `town_start_church_upgrade_construction`. Si no hay iglesia en la lista pública no hace nada. La obra
    /// se cuelga de la lista de construcción detrás de la tercera pieza seguida de tipo `>= 0x30`, o al final.
    /// `visual_owner` es el propietario (`+0xC324`) del objeto visual, si existe.
    pub fn start_church_upgrade_construction(
        &mut self,
        t: &mut Town,
        visual_owner: Option<u32>,
        host: &mut dyn ChurchHost,
    ) {
        let count = Self::record_count(t) as u16;
        let mut cur = t.u16(off::PUBLIC_BUILDING_HEAD);
        if cur >= count {
            return;
        }
        let mut prev: Option<usize> = None;
        let found = loop {
            let i = cur as usize;
            if self.rec(i, 3) == CHURCH {
                break i;
            }
            prev = Some(i);
            cur = self.next(i);
            if cur >= count {
                return;
            }
        };
        let after = self.next(found);
        match prev {
            None => t.set_u16(off::PUBLIC_BUILDING_HEAD, after),
            Some(p) => self.set_next(p, after),
        }
        self.set_rec(found, 6, RECORD_BYTE_6[CHURCH as usize]);
        self.set_rec(found, 2, 0xFF);
        let id = found as u16;
        let mut run = 0u16;
        let mut link: Option<usize> = None;
        let mut cur = t.u16(off::CONSTRUCTION_HEAD);
        let mut inserted = false;
        while cur < count {
            let i = cur as usize;
            if self.rec(i, 3) < 0x30 {
                run = 0;
            } else {
                run += 1;
                if run >= 3 {
                    let next = self.next(i);
                    self.set_next(found, next);
                    self.set_next(i, id);
                    inserted = true;
                    break;
                }
            }
            link = Some(i);
            cur = self.next(i);
        }
        if !inserted {
            match link {
                None => t.set_u16(off::CONSTRUCTION_HEAD, id),
                Some(l) => self.set_next(l, id),
            }
            self.set_next(found, 0xFFFF);
            if let Some(owner) = visual_owner {
                if owner == t.u8(0x2C0) as u32 {
                    let (row, col) = (self.rec(found, 0) as u32, self.rec(found, 1) as u32);
                    host.add_map_object(row, col, MAP_CLASS as i32 as u32, t.u8(0x7A3) as u32, 0xFF);
                    host.set_progress(row, col, 0, self.rec(found, 6) as u32, 0);
                }
            }
        }
    }
}
