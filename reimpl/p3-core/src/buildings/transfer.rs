//! `town_transfer_building_to_merchant` (0x005256F0, C05): traspasa un edificio a otro propietario.
//!
//! Cobra y paga a los dos comerciantes, desenlaza el registro de la lista de su antiguo dueño (la pública de
//! la ciudad o la de su oficina), lo enlaza en la de la oficina del nuevo y avisa al mapa visual. Los
//! comerciantes, las oficinas y el mapa visual pasan por `TransferHost`.

use super::tables::RECORD_BYTE_6;
use super::Site;
use crate::town::{off, Town};

/// Campos del comerciante que toca el traspaso.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TransferMerchant {
    /// `+0`: dinero.
    pub gold: i32,
    /// `+0x4B8`: cobrado por ventas.
    pub sold: i32,
    /// `+0x4BC`: pagado por compras.
    pub bought: i32,
}

/// Lo que pide el traspaso fuera de la ciudad.
pub trait TransferHost {
    /// `get_merchant` (0x005303C0).
    fn merchant(&mut self, owner: u32) -> &mut TransferMerchant;
    /// `find_or_create_office` (0x005308A0): (comerciante, propietario de la ciudad, crear); la oficina o `None`.
    fn find_office(&mut self, merchant: u32, town_owner: u32, create: u32) -> Option<u32>;
    /// `Office+0x2CE` (edificios) o `Office+0x2D0` (el resto) de la oficina.
    fn office_head(&mut self, office: u32, second: bool) -> u16;
    fn set_office_head(&mut self, office: u32, second: bool, v: u16);
    /// `office_recount_buildings_and_facilities` (0x004FF2D0).
    fn recount_office(&mut self, office: u32);
    /// `town_view_draw_building_chain` (0x00521310): (primer registro, 0).
    fn draw_chain(&mut self, head: u32, flag: u32);
    /// `class22_set_building_progress_at_tile` (0x005891C0).
    fn set_progress(&mut self, row: u32, col: u32, done: u32, total: u32, flag: u32);
}

/// Entorno del traspaso.
pub struct TransferCtx<'a> {
    /// Número de comerciantes (`0x006DE4AA`).
    pub merchant_count: u16,
    /// Propietario (`+0xC324`) del objeto `0x006E51AC`, que el original lee sin comprobar que exista.
    pub visual_owner: u32,
    pub host: &'a mut dyn TransferHost,
}

/// Cabeza de lista de la que se desenlaza el registro.
#[derive(Clone, Copy)]
enum Slot {
    Town,
    Office(u32, bool),
}

/// Tipo de edificio desde el que los registros van a la segunda lista de la oficina.
const SECOND_LIST_FROM: u8 = 0x15;

impl Site {
    /// `rec` es un registro existente; `to` el nuevo propietario (solo cuenta su byte bajo); `paid_by_new` se
    /// resta al dinero del nuevo dueño (si es comerciante) y `paid_to_old` se suma al del antiguo. Si el
    /// propietario antiguo es un comerciante sin oficina en la ciudad se paga y se cobra, pero el registro
    /// no cambia de dueño. Si el edificio era de un propietario sin oficina propia (público) y es de tipo
    /// bajo, el original lee una oficina nula al avisar al mapa visual: aquí eso es un `panic`.
    pub fn transfer_building_to_merchant(
        &mut self,
        t: &mut Town,
        cx: &mut TransferCtx,
        rec: u32,
        to: u8,
        paid_by_new: i32,
        paid_to_old: i32,
    ) {
        let count = t.u16(off::BUILDING_RECORD_COUNT);
        assert!(rec < count as u32, "registro fuera del almacén");
        let i = rec as usize;
        let old = self.rec(i, 2);
        if (old as u16) < cx.merchant_count {
            let m = cx.host.merchant(old as u32);
            m.gold = m.gold.wrapping_add(paid_to_old);
            m.sold = m.sold.wrapping_add(paid_to_old);
        }
        if (to as u16) < cx.merchant_count {
            let m = cx.host.merchant(to as u32);
            m.gold = m.gold.wrapping_sub(paid_by_new);
            m.bought = m.bought.wrapping_add(paid_by_new);
        }
        if old == to {
            return;
        }
        let town_owner = t.u8(0x2C0) as u32;
        let second = self.rec(i, 3) >= SECOND_LIST_FROM;
        let (old_office, slot) = if (old as u16) < cx.merchant_count {
            match cx.host.find_office(old as u32, town_owner, 0) {
                Some(o) => (Some(o), Slot::Office(o, second)),
                None => return,
            }
        } else {
            (None, Slot::Town)
        };
        // Desenlaza el registro de la lista del antiguo dueño.
        let read = |t: &Town, host: &mut dyn TransferHost| match slot {
            Slot::Town => t.u16(off::PUBLIC_BUILDING_HEAD),
            Slot::Office(o, second) => host.office_head(o, second),
        };
        let head = read(t, cx.host);
        if head as u32 == rec {
            let next = self.next(i);
            match slot {
                Slot::Town => t.set_u16(off::PUBLIC_BUILDING_HEAD, next),
                Slot::Office(o, second) => cx.host.set_office_head(o, second, next),
            }
        } else if head < count {
            let mut cur = head as usize;
            loop {
                let next = self.next(cur);
                if next as u32 == rec {
                    let after = self.next(i);
                    self.set_next(cur, after);
                    break;
                }
                if next >= count {
                    break;
                }
                cur = next as usize;
            }
        }
        self.set_rec(i, 2, to);
        let new_office = if (to as u16) < cx.merchant_count {
            let o = cx
                .host
                .find_office(to as u32, town_owner, 1)
                .expect("el original lee una oficina nula");
            let list_head = cx.host.office_head(o, second);
            self.set_next(i, list_head);
            cx.host.set_office_head(o, second, rec as u16);
            Some(o)
        } else {
            None
        };
        if let Some(o) = old_office {
            cx.host.recount_office(o);
        }
        if let Some(o) = new_office {
            cx.host.recount_office(o);
        }
        if cx.visual_owner != town_owner {
            return;
        }
        if !second {
            let o = old_office.expect("el original lee una oficina nula");
            let head = cx.host.office_head(o, false);
            cx.host.draw_chain(head as u32, 0);
        }
        cx.host.draw_chain(rec, 0);
        let spent = self.rec(i, 6) as u32;
        if spent == 0 {
            return;
        }
        let ty = self.rec(i, 3) as usize;
        let (row, col) = (self.rec(i, 0) as u32, self.rec(i, 1) as u32);
        let list = t.u16(0x76A);
        let mut at = list;
        let mut in_list = false;
        while at < count {
            if at as u32 == rec {
                in_list = true;
                break;
            }
            at = self.next(at as usize);
        }
        if in_list {
            cx.host.set_progress(row, col, spent, 0x32, 1);
        } else {
            let total = RECORD_BYTE_6[ty] as u32;
            cx.host.set_progress(row, col, total.wrapping_sub(spent), total, 0);
        }
    }
}
