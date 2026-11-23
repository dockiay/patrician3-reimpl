//! `op_remove_building` (0x0053CF50): derribo de un edificio por orden (C05, parte C12).
//!
//! Busca el registro del edificio por fila y columna en la lista de casas de la oficina, la de edificios de
//! la oficina y la de construcción de la ciudad; lo desenlaza, quita su huella de la rejilla, avisa al mapa
//! visual y, si es un oficio (tipo < 0x15), a la reputación del comerciante, y libera el registro. Un
//! edificio con subasta en marcha no se derriba.

use super::grid::Grid;
use super::Site;
use crate::town::{off, Town};

/// Orden de derribo (`+4` ciudad, `+8` comerciante, `+0xC` fila, `+0x10` columna).
#[derive(Clone, Copy, Debug)]
pub struct RemoveOp {
    pub town: u32,
    pub merchant: u32,
    pub row: u32,
    pub col: u32,
}

/// Consultas y avisos a otros sistemas.
pub trait RemoveHost {
    /// `find_or_create_office` (0x005308A0): ¿existe la oficina del comerciante en la ciudad?
    fn find_office(&mut self, merchant: u32, town: u32, flag: u32) -> bool;
    /// Cabeza de la oficina: 0 = `+0x2D0` (casas), 1 = `+0x2CE` (edificios).
    fn office_head(&mut self, which: usize) -> u16;
    fn set_office_head(&mut self, which: usize, v: u16);
    /// `office_recount_buildings_and_facilities` (0x004FF2D0).
    fn recount_office(&mut self);
    /// `scheduled_tasks_get_building_auction_info` (0x004DAFE0): ¿tiene el edificio una subasta?
    fn auction_in_progress(&mut self, town: u32, record: u32) -> bool;
    /// `class22_remove_building_with_effects` (0x00599040).
    fn remove_visual(&mut self, row: u32, col: u32);
    /// `get_merchant` (0x005303C0).
    fn get_merchant(&mut self, merchant: u32);
    /// `merchant_subtract_social_reputation_demolition` (0x004F8D70).
    fn subtract_reputation(&mut self, town: u32);
}

pub struct RemoveCtx<'a> {
    /// Número de ciudades (`0x006DE4B0`).
    pub town_count: u32,
    /// Número de comerciantes (`0x006DE4AA`).
    pub merchant_count: u16,
    /// Propietario (`+0xC324`) del objeto `0x006E51AC`, si existe.
    pub visual_owner: Option<u32>,
    /// Byte global `0x0067792F`.
    pub special: u8,
    pub host: &'a mut dyn RemoveHost,
}

#[derive(Clone, Copy)]
enum Link {
    OfficeHead(usize),
    Construction,
    Rec(usize),
}

impl Site {
    fn link_get(&self, t: &Town, host: &mut dyn RemoveHost, l: Link) -> u16 {
        match l {
            Link::OfficeHead(w) => host.office_head(w),
            Link::Construction => t.u16(off::CONSTRUCTION_HEAD),
            Link::Rec(i) => self.next(i),
        }
    }

    fn link_set(&mut self, t: &mut Town, host: &mut dyn RemoveHost, l: Link, v: u16) {
        match l {
            Link::OfficeHead(w) => host.set_office_head(w, v),
            Link::Construction => t.set_u16(off::CONSTRUCTION_HEAD, v),
            Link::Rec(i) => self.set_next(i, v),
        }
    }

    /// `op_remove_building` sobre la ciudad `t` (la de la tabla de ciudades con el índice de la orden).
    pub fn remove_building(&mut self, t: &mut Town, op: &RemoveOp, cx: &mut RemoveCtx) {
        if op.town >= cx.town_count || op.merchant >= cx.merchant_count as u32 {
            return;
        }
        if !cx.host.find_office(op.merchant & 0xFFFF, op.town & 0xFFFF, 0) {
            return;
        }
        if t.flags() & 0x10 != 0 {
            let at = t
                .i32(off::PLAN_STRIDE)
                .wrapping_mul(op.row as i32)
                .wrapping_add(op.col as i32);
            if (self.cells[at as usize] as i8) >= 0 {
                return;
            }
        }
        let count = Self::record_count(t);
        let mut link = Link::OfficeHead(0);
        let mut phase = 0;
        let (idx, rec) = loop {
            let idx = self.link_get(t, cx.host, link) as usize;
            if idx >= count {
                match phase {
                    0 => link = Link::OfficeHead(1),
                    1 => link = Link::Construction,
                    _ => {
                        cx.host.recount_office();
                        return;
                    }
                }
                phase += 1;
                continue;
            }
            if self.rec(idx, 0) as u32 == op.row && self.rec(idx, 1) as u32 == op.col {
                break (idx, idx);
            }
            link = Link::Rec(idx);
        };
        let _ = rec;
        if cx.host.auction_in_progress(op.town & 0xFF, idx as u32) {
            return;
        }
        let next = self.next(idx);
        self.link_set(t, cx.host, link, next);
        self.set_next(idx, 0xFFFF);
        if cx.visual_owner == Some(op.town) {
            cx.host.remove_visual(op.row, op.col);
        }
        let (ty, extra) = (self.rec(idx, 3), self.rec(idx, 7));
        let mut grid = Grid {
            stride: t.i32(off::PLAN_STRIDE),
            rows: t.i32(off::PLAN_ROWS),
            cells: &mut self.cells,
        };
        grid.clear_building_footprint(op.row, op.col, ty as u32, extra as u32, cx.special);
        cx.host.recount_office();
        if ty < 0x15 {
            cx.host.get_merchant(op.merchant);
            cx.host.subtract_reputation(op.town & 0xFF);
        }
        self.free_building_record_if_unlinked(t, idx as u32);
    }
}
