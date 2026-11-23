//! Guardado y carga de los datos de edificios de una ciudad (C05, parte C10): tamaño
//! (`0x0051E9F0`), serialización (`0x0051EA30`) y deserialización (`0x0051EC10`), con las de la iglesia
//! (`0x004FE1E0`/`0x004FE240`) y el plano urbano (`0x004F9680`/`0x004F9770`) que llaman.
//!
//! El formato es una secuencia de campos de la ciudad sin relleno. La carga acepta la versión del
//! guardado: las anteriores a la actual omiten campos (que se rellenan con el valor que fija el
//! original) y las más viejas reajustan los tipos de edificio, que se renumeraron al añadir tipos.

use super::Site;
use crate::town::{off, Town};

/// Versión de guardado a partir de la cual el formato es el vigente.
pub const CURRENT_VERSION: u32 = 0x7D;

const CHURCH: usize = 0x794;
const PLAN: usize = 0x7A4;

struct Writer(Vec<u8>);

impl Writer {
    fn town(&mut self, t: &Town, o: usize, n: usize) {
        self.0.extend_from_slice(&t.0[o..o + n]);
    }
}

struct Reader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl Reader<'_> {
    fn take(&mut self, n: usize) -> &[u8] {
        let s = &self.data[self.pos..self.pos + n];
        self.pos += n;
        s
    }
    fn byte(&mut self) -> u8 {
        self.take(1)[0]
    }
    /// Copia `n` bytes del guardado a `Town+o`.
    fn town(&mut self, t: &mut Town, o: usize, n: usize) {
        let src = self.take(n).to_vec();
        t.0[o..o + n].copy_from_slice(&src);
    }
    fn skip(&mut self, n: usize) {
        self.pos += n;
    }
}

impl Site {
    /// `town_get_building_data_serialized_size` (0x0051E9F0).
    pub fn building_data_size(&self, t: &Town) -> usize {
        let cells = t.i32(off::PLAN_STRIDE).wrapping_mul(t.i32(off::PLAN_ROWS)).max(0) as usize;
        0x10 + 0x34 + Self::record_count(t) * 8 + 0x64 + cells
    }

    /// `church_serialize` (0x004FE1E0).
    fn serialize_church(t: &Town, w: &mut Writer) {
        w.town(t, CHURCH, 12);
        w.town(t, CHURCH + 0xC, 4);
    }

    /// `town_plan_serialize` (0x004F9680).
    fn serialize_plan(&self, t: &Town, w: &mut Writer) {
        let cells = t.i32(off::PLAN_STRIDE).wrapping_mul(t.i32(off::PLAN_ROWS));
        w.town(t, off::PLAN_ROWS, 4);
        w.town(t, off::PLAN_STRIDE, 4);
        w.town(t, PLAN + 0xC, 8);
        w.town(t, PLAN + 0x14, 8);
        w.town(t, PLAN + 0x1C, 0x10);
        w.town(t, PLAN + 0x2C, 0x20);
        w.town(t, PLAN + 0x4C, 0x10);
        w.town(t, PLAN + 0x5C, 12);
        if cells > 0 {
            w.0.extend_from_slice(&self.cells[..cells as usize]);
        }
    }

    /// `town_serialize_building_data` (0x0051EA30).
    pub fn serialize_building_data(&self, t: &Town) -> Vec<u8> {
        let mut w = Writer(Vec::with_capacity(self.building_data_size(t)));
        for o in [
            off::BUILDING_RECORD_COUNT,
            off::FREE_BUILDING_HEAD,
            off::CONSTRUCTION_HEAD,
            off::PUBLIC_BUILDING_HEAD,
            off::BURNING_HEAD,
            off::DEFENSE_HEAD,
            off::WALL_HEAD,
        ] {
            w.town(t, o, 2);
        }
        w.town(t, 0x774, 1);
        w.town(t, 0x771, 1);
        w.town(t, 0x770, 1);
        w.town(t, off::MORE_FLAGS, 4);
        w.town(t, off::FIRST_OFFICE, 2);
        for o in [0x773, 0x772, off::WELLS, off::TOWERS, 0x78A] {
            w.town(t, o, 1);
        }
        for o in [
            off::FACILITIES_INSIDE_WALLS,
            off::FACILITIES_TOTAL,
            off::DEFENSE_CAPACITY,
            off::STREETS_BUILT,
            off::STREETS_TOTAL,
        ] {
            w.town(t, o, 2);
        }
        w.town(t, off::WEAPONSMITH_SHORTAGE, 1);
        w.town(t, 0x78B, 1);
        w.town(t, 0x778, 6);
        w.town(t, 0x77E, 6);
        Self::serialize_church(t, &mut w);
        let count = Self::record_count(t);
        w.0.extend_from_slice(&self.records[..count * 8]);
        self.serialize_plan(t, &mut w);
        w.0
    }

    /// `church_deserialize` (0x004FE240).
    fn deserialize_church(t: &mut Town, r: &mut Reader, version: u32) {
        r.town(t, CHURCH, 12);
        if version > 0xB && version < 0x31 {
            r.skip(4);
        }
        r.town(t, CHURCH + 0xC, 4);
    }

    /// `town_plan_deserialize` (0x004F9770).
    fn deserialize_plan(&mut self, t: &mut Town, r: &mut Reader, version: u32) {
        t.set_u32(PLAN, 1);
        r.town(t, off::PLAN_ROWS, 4);
        r.town(t, off::PLAN_STRIDE, 4);
        r.town(t, PLAN + 0xC, 8);
        r.town(t, PLAN + 0x14, 8);
        r.town(t, PLAN + 0x1C, 0x10);
        if version < 0x69 {
            r.town(t, PLAN + 0x64, 2);
            let (a, b) = (t.u8(PLAN + 0x64), t.u8(PLAN + 0x65));
            t.set_u8(PLAN + 0x66, a);
            t.set_u8(PLAN + 0x67, b);
            r.skip(0xC - 2);
        }
        if version < CURRENT_VERSION {
            t.0[PLAN + 0x2C..PLAN + 0x4C].fill(0xFF);
            t.0[PLAN + 0x4C..PLAN + 0x5C].fill(0);
            r.town(t, PLAN + 0x2C, 0x10);
            if version == 0x7C {
                r.town(t, PLAN + 0x4C, 8);
            }
        } else {
            r.town(t, PLAN + 0x2C, 0x20);
            r.town(t, PLAN + 0x4C, 0x10);
        }
        r.town(t, PLAN + 0x5C, 8);
        if version > 0x68 {
            r.town(t, PLAN + 0x64, 4);
        }
        let cells = t.i32(off::PLAN_STRIDE).wrapping_mul(t.i32(off::PLAN_ROWS));
        self.cells = if cells > 0 {
            r.take(cells as usize).to_vec()
        } else {
            Vec::new()
        };
    }

    /// `town_plan_deserialize` (0x004F9770) con los bytes de un archivo de plano.
    pub fn deserialize_plan_data(&mut self, t: &mut Town, data: &[u8], version: u32) {
        let mut r = Reader { data, pos: 0 };
        self.deserialize_plan(t, &mut r, version);
    }

    /// `town_deserialize_building_data` (0x0051EC10). Devuelve los bytes consumidos.
    pub fn deserialize_building_data(&mut self, t: &mut Town, data: &[u8], version: u32) -> usize {
        let mut r = Reader { data, pos: 0 };
        r.town(t, off::BUILDING_RECORD_COUNT, 2);
        for o in [
            off::FREE_BUILDING_HEAD,
            off::CONSTRUCTION_HEAD,
            off::PUBLIC_BUILDING_HEAD,
            off::BURNING_HEAD,
            off::DEFENSE_HEAD,
            off::WALL_HEAD,
        ] {
            r.town(t, o, 2);
        }
        let b = r.byte();
        t.set_u8(0x774, b);
        let b = r.byte();
        t.set_u8(0x771, b);
        let c = if version < 0x7B { b } else { r.byte() };
        t.set_u8(0x770, c);
        if version < 0x68 {
            let v = u16::from_le_bytes(r.take(2).try_into().unwrap());
            t.set_u32(off::MORE_FLAGS, v as u32);
        } else {
            r.town(t, off::MORE_FLAGS, 4);
        }
        r.town(t, off::FIRST_OFFICE, 2);
        let b = r.byte();
        t.set_u8(0x773, b);
        let c = if version < 0x7B { b } else { r.byte() };
        t.set_u8(0x772, c);
        for o in [off::WELLS, off::TOWERS, 0x78A] {
            let b = r.byte();
            t.set_u8(o, b);
        }
        for o in [
            off::FACILITIES_INSIDE_WALLS,
            off::FACILITIES_TOTAL,
            off::DEFENSE_CAPACITY,
            off::STREETS_BUILT,
            off::STREETS_TOTAL,
        ] {
            r.town(t, o, 2);
        }
        if version < 0x7B {
            t.set_u8(0x78B, 0);
            t.set_u8(off::WEAPONSMITH_SHORTAGE, 0);
        } else {
            let b = r.byte();
            t.set_u8(off::WEAPONSMITH_SHORTAGE, b);
            let b = r.byte();
            t.set_u8(0x78B, b);
        }
        r.town(t, 0x778, 6);
        if version < 0x68 {
            let (a, b) = (t.0[0x778..0x77E].to_vec(), 0x77E);
            t.0[b..b + 6].copy_from_slice(&a);
        } else {
            r.town(t, 0x77E, 6);
        }
        Self::deserialize_church(t, &mut r, version);

        let count = Self::record_count(t);
        self.records = if count > 0 {
            r.take(count * 8).to_vec()
        } else {
            Vec::new()
        };
        if count > 0 {
            if version < 0x59 {
                for i in 0..count {
                    let ty = self.rec(i, 3);
                    if ty > 0x29 && ty < 0xFA {
                        self.set_rec(i, 3, ty.wrapping_add(2));
                    }
                }
            }
            if version < 0x67 {
                for i in 0..count {
                    let ty = self.rec(i, 3);
                    if ty > 0x30 && ty < 0xFA {
                        self.set_rec(i, 3, ty.wrapping_add(2));
                    }
                }
            }
            if version < 0x69 {
                for i in 0..count {
                    let ty = self.rec(i, 3);
                    if ty > 0x2D && ty < 0xFA {
                        self.set_rec(i, 3, ty.wrapping_add(1));
                    }
                }
            }
            // Los registros libres pierden su posición.
            let mut i = t.u16(off::FREE_BUILDING_HEAD) as usize;
            while i < count {
                self.set_rec(i, 0, 0xFF);
                self.set_rec(i, 1, 0xFF);
                i = self.next(i) as usize;
            }
        }
        if version < 0x58 {
            r.skip(0xC);
        }
        self.deserialize_plan(t, &mut r, version);

        // Debe haber un edificio de tipo 0x2D en la lista pública si la ciudad tiene la bandera 1.
        let count = Self::record_count(t);
        let mut found = false;
        let mut i = t.u16(off::PUBLIC_BUILDING_HEAD) as usize;
        while i < count {
            if self.rec(i, 3) == 0x2D {
                found = true;
                break;
            }
            i = self.next(i) as usize;
        }
        if !found && t.more_flags() & 1 == 1 {
            let n = self.alloc_building_record(t) as usize;
            let (row, col) = (t.u8(PLAN + 0x5C), t.u8(PLAN + 0x5D));
            self.set_rec(n, 0, row);
            self.set_rec(n, 1, col);
            self.set_rec(n, 2, 0xFF);
            self.set_rec(n, 3, 0x2D);
            self.set_next(n, t.u16(off::PUBLIC_BUILDING_HEAD));
            self.set_rec(n, 6, 0);
            self.set_rec(n, 7, 0);
            t.set_u16(off::PUBLIC_BUILDING_HEAD, n as u16);
        }
        r.pos
    }
}
