//! Edificio de tipo `0x2F` según el número de empleados (C05): `town_plan_find_type_2f_marker` (0x004FA740)
//! y `town_try_add_type_2f_building_by_employees` (0x00524920).

use super::tables::RECORD_BYTE_6;
use super::Site;
use crate::town::{off, Town};

/// Tipo de edificio que busca y crea esta función.
const TYPE_2F: u8 = 0x2F;
/// Celda que marca el sitio del edificio en el plano (`0x0F`).
const MARKER_CELL: u8 = 0x0F;
/// Celda de calle de obra (`0x0B`) que deja el edificio en su huella.
const SITE_CELL: u8 = 0x0B;
/// Bytes de `0x006729DE`, `0x00672A7B` y `0x00672AB7` (tabla por tipo, entrada `0x2F`).
const EMPLOYEE_STEP: u32 = 30;
const REQUIRED_GOLD_UNITS: i32 = 10;
const REQUIRED_STOCK_UNITS: i32 = 10;
/// Globales `0x0067792D` y `0x0067792B`: parámetro del marcador según la celda de abajo a la izquierda.
const KIND_IF_ROAD: u8 = 0xA0;
const KIND_OTHER: u8 = 0x60;

/// Avisos a otros sistemas que hace `town_try_add_type_2f_building_by_employees`.
pub trait Type2fHost {
    /// `class22_add_map_object` (0x00590680): (fila, columna, 0x5F, parámetro, 0xFF).
    fn add_map_object(&mut self, row: u32, col: u32, param: u32, packed: u32, owner: u32);
    /// `class22_set_building_progress_at_tile` (0x005891C0): (fila, columna, 0, 100, 0).
    fn set_progress(&mut self, row: u32, col: u32, done: u32, total: u32, flag: u32);
}

/// Lo que `find_type_2f_marker` deja en sus tres salidas: fila, columna y parámetro.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Marker {
    pub row: u8,
    pub col: u8,
    pub kind: u8,
}

impl Site {
    /// `town_plan_find_type_2f_marker`: recorre la rejilla de atrás adelante y se queda con la última celda
    /// `0x0F` que encuentra, es decir, la de menor índice. Sin ninguna, devuelve `(0xFF, 0xFF, 0)`. El
    /// original lee la celda de la fila siguiente (dos columnas atrás) sin comprobar el final de la rejilla.
    pub fn find_type_2f_marker(&self, t: &Town) -> Marker {
        let (rows, stride) = (t.i32(off::PLAN_ROWS), t.i32(off::PLAN_STRIDE));
        let (center_row, center_col) = (t.u8(0x800) as i32, t.u8(0x801) as i32);
        let mut out = Marker {
            row: 0xFF,
            col: 0xFF,
            kind: 0,
        };
        let mut at = rows.wrapping_mul(stride);
        if at > 0 {
            loop {
                at -= 1;
                if self.cells[at as usize] == MARKER_CELL {
                    let (q, rem) = (at / stride, at % stride);
                    let (dr, dc) = (q - center_row, rem - center_col);
                    if dc.wrapping_mul(dc).wrapping_add(dr.wrapping_mul(dr)) < i32::MAX {
                        out.row = q as u8;
                        out.col = rem as u8;
                        out.kind = if self.cells[(at + stride - 2) as usize] == 0x0D {
                            KIND_IF_ROAD
                        } else {
                            KIND_OTHER
                        };
                    }
                }
                if at <= 0 {
                    break;
                }
            }
        }
        out
    }

    /// `town_try_add_type_2f_building_by_employees`: si la ciudad aún no marcó que no cabe más
    /// (`Town+0x2C8` bit 24), calcula cuántos edificios `0x2F` hay en la lista pública, comprueba que el
    /// número ponderado queda bajo `limit` y que hay oro y existencias, y coloca uno nuevo en el marcador.
    /// `visual_owner` es el propietario (`+0xC324`) del objeto `0x006E51AC`, si existe.
    pub fn try_add_type_2f_building_by_employees(
        &mut self,
        t: &mut Town,
        limit: u32,
        visual_owner: Option<u32>,
        host: &mut dyn Type2fHost,
    ) {
        if t.flags() & 0x100_0000 != 0 {
            return;
        }
        let count = t.u16(off::BUILDING_RECORD_COUNT) as u32;
        let mut cur = t.u16(off::PUBLIC_BUILDING_HEAD) as u32;
        let mut weighted = 0u32;
        if cur < count {
            let mut found = 0u32;
            while cur < count {
                if self.rec(cur as usize, 3) == TYPE_2F {
                    found += 1;
                }
                cur = self.next(cur as usize) as u32;
            }
            if found != 0 {
                weighted = found.wrapping_mul(((found + 1) >> 1).wrapping_mul(EMPLOYEE_STEP));
            }
        }
        if weighted >= limit {
            return;
        }
        if t.i32(0x30) < REQUIRED_GOLD_UNITS * 2000 || t.i32(0x34) < REQUIRED_STOCK_UNITS * 200 {
            return;
        }
        let marker = self.find_type_2f_marker(t);
        if (marker.row as i32) >= t.i32(off::PLAN_ROWS) {
            t.set_flags(t.flags() | 0x100_0000);
            return;
        }
        let i = self.alloc_building_record(t);
        let (r, c, k) = (marker.row, marker.col, marker.kind);
        for (b, v) in [r, c, 0xFF, TYPE_2F].into_iter().enumerate() {
            self.set_rec(i as usize, b, v);
        }
        self.set_next(i as usize, 0xFFFF);
        self.set_rec(i as usize, 6, RECORD_BYTE_6[TYPE_2F as usize]);
        self.set_rec(i as usize, 7, k);
        self.enqueue_construction_record(t, i);
        let stride = t.i32(off::PLAN_STRIDE) as usize;
        let base = stride * r as usize + c as usize;
        for back in 0..3 {
            self.cells[base - back] = SITE_CELL;
        }
        let other_row = if c & 1 != 0 { r as usize + 1 } else { r as usize - 1 };
        self.cells[stride * other_row + c as usize - 1] = SITE_CELL;
        if visual_owner == Some(t.u8(0x2C0) as u32) {
            host.add_map_object(r as u32, c as u32, 0x5F, k as u32, 0xFF);
            host.set_progress(r as u32, c as u32, 0, 100, 0);
        }
    }
}
