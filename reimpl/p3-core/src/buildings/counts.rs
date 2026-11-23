//! Recuentos de calles, defensa y muralla de una ciudad.

use super::Site;
use crate::town::{off, Town};

/// Enlaces de una oficina hacia la lista de edificios de su comerciante (`Office+0x2CA` y `+0x2CE`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct OfficeLists {
    /// `+0x2CA`: siguiente oficina de la ciudad.
    pub next: u16,
    /// `+0x2CE`: primer edificio de la oficina.
    pub building_head: u16,
}

/// Desplazamientos de contadores que escribe el recuento de defensa.
const TOWERS: usize = 0x788;
const INSIDE_WALLS: usize = 0x78C;
const TOTAL: usize = 0x78E;
const DEFENSE: usize = 0x786;

impl Site {
    /// `town_plan_count_street_cells` (0x004FA6F0): cuenta las celdas con el bit `0x80`. Devuelve
    /// (celdas `0x80..=0x83`, celdas `0x80` o `0x81`).
    pub fn count_street_cells(&self, t: &Town) -> (u16, u16) {
        let total = (t.i32(off::PLAN_STRIDE) as i64 * t.i32(off::PLAN_ROWS) as i64) as usize;
        let (mut paved, mut straight) = (0u16, 0u16);
        for &c in self.cells[..total].iter().rev() {
            if c >= 0x80 {
                if c < 0x84 {
                    paved = paved.wrapping_add(1);
                }
                if c == 0x81 || c == 0x80 {
                    straight = straight.wrapping_add(1);
                }
            }
        }
        (paved, straight)
    }

    /// `count_defense_buildings` (0x00524B70): recorre la lista desde `start` sumando instalaciones
    /// (tipos de oficio, con las que ya están dentro de la muralla), torres y capacidad defensiva.
    pub fn count_defense_buildings(&self, t: &mut Town, start: u32) {
        let count = Self::record_count(t) as u32;
        if start >= count {
            return;
        }
        let stride = t.i32(off::PLAN_STRIDE) as i64;
        let mut i = start;
        loop {
            let r = i as usize;
            let ty = self.rec(r, 3);
            if ty != 5 {
                if ty < 0x15 {
                    let cell = self.cells[(stride * self.rec(r, 0) as i64 + self.rec(r, 1) as i64) as usize];
                    if cell >= 0x80 {
                        t.add_u16(INSIDE_WALLS, 1);
                    }
                    t.add_u16(TOTAL, 1);
                } else if ty >= 0x30 {
                    match ty {
                        0x30 | 0x31 => t.set_u8(TOWERS, t.u8(TOWERS).wrapping_add(1)),
                        0x32 | 0x33 => {
                            t.set_u8(TOWERS, t.u8(TOWERS).wrapping_add(1));
                            t.add_u16(DEFENSE, 1);
                        }
                        0x36 => t.add_u16(DEFENSE, 1),
                        _ => t.add_u16(DEFENSE, 3),
                    }
                }
            }
            i = self.next(r) as u32;
            if i >= count {
                break;
            }
        }
    }

    /// `Town::recount_defense_buildings` (0x00524C30): vuelve a contar torres, instalaciones y
    /// capacidad defensiva (mínimo 1) con la lista de la ciudad y las de sus oficinas.
    pub fn recount_defense_buildings(&self, t: &mut Town, offices: &[OfficeLists], office_count: u16) {
        t.set_u8(TOWERS, 0);
        t.set_u16(INSIDE_WALLS, 1);
        t.set_u16(TOTAL, 1);
        t.set_u16(DEFENSE, 0);
        self.count_defense_buildings(t, t.u16(off::DEFENSE_HEAD) as u32);
        let mut o = t.u16(off::FIRST_OFFICE);
        while o < office_count {
            let office = offices[o as usize];
            self.count_defense_buildings(t, office.building_head as u32);
            o = office.next;
        }
        if t.u16(DEFENSE) == 0 {
            t.set_u16(DEFENSE, 1);
        }
    }

    /// `town_plan_is_plan_point_valid` (0x004FCFE0): ¿cabe el punto `n` (0–3) de arranque de muralla
    /// dentro de la rejilla?
    pub fn is_plan_point_valid(t: &Town, n: u32) -> bool {
        if n > 3 {
            return false;
        }
        let n = n as usize;
        (t.u8(0x7B0 + 2 * n) as i32) < t.i32(off::PLAN_ROWS) && (t.u8(0x7B1 + 2 * n) as i32) < t.i32(off::PLAN_STRIDE)
    }

    /// `town_get_wall_expansion_level` (0x005252F0). Con muralla en construcción o terminada mira la
    /// lista de defensa; sin ella, cuántos de los puntos de arranque 2, 1 y 0 caben (el último
    /// devuelve 0xFF si el punto 0 es válido, como el original).
    pub fn wall_expansion_level(&self, t: &Town) -> u8 {
        let count = Self::record_count(t);
        let head = t.u16(off::WALL_HEAD) as usize;
        if head < count {
            if self.rec(head, 2) >= 0xFE {
                return 0;
            }
            let mut i = t.u16(off::DEFENSE_HEAD) as usize;
            while i < count {
                if self.rec(i, 2) == 0xFE {
                    return 1;
                }
                i = self.next(i) as usize;
            }
            return 0;
        }
        if !Self::is_plan_point_valid(t, 2) {
            return 2;
        }
        if !Self::is_plan_point_valid(t, 1) {
            return 1;
        }
        if Self::is_plan_point_valid(t, 0) {
            0xFF
        } else {
            0
        }
    }
}
