//! `town_advance_construction` (0x0051FF30): gasta un presupuesto de obra (C05, parte C8).
//!
//! Recorre la lista de construcción (`Town+0x762`). A cada registro le quita progreso (hasta 5 por
//! registro; el resto del presupuesto pasa al siguiente) y, cuando llega a cero, lo saca de la lista y lo
//! pasa a la que le toca según su tipo: públicos, defensa, cabeza de oficina del comerciante o libres. Si
//! el propietario es un comerciante, además se le avisa con un mensaje. Las consultas a otros sistemas
//! (oficina, mapa visual, mensajes y recuentos) pasan por `AdvanceHost`.

use super::tables::RECORD_BYTE_6;
use super::Site;
use crate::town::{off, Town};

/// Consultas y avisos a otros sistemas.
pub trait AdvanceHost {
    /// `find_or_create_office` (0x005308A0): ¿existe la oficina del propietario en esta ciudad?
    fn find_office(&mut self, owner: u32, town: u32, flag: u32) -> bool;
    /// Cabeza de edificios de la oficina `which` (0: `Office+0x2CE`, 1: `+0x2D0`): devuelve la anterior
    /// y guarda `new`.
    fn swap_office_head(&mut self, which: usize, new: u16) -> u16;
    /// `office_recount_buildings_and_facilities` (0x004FF2D0) de la oficina devuelta.
    fn recount_office(&mut self);
    fn recount_office_with_site(&mut self, _site: &mut super::Site, _town: &mut Town) {
        self.recount_office();
    }
    /// `town_recount_buildings` (0x00520BD0).
    fn recount_buildings(&mut self);
    /// Variante con acceso al estado urbano, para persistir el recuento en el mismo tick.
    fn recount_buildings_with_site(&mut self, _site: &mut super::Site, _town: &mut Town) {
        self.recount_buildings();
    }
    /// `class22_set_building_progress_at_tile` (0x005891C0).
    fn set_progress(&mut self, row: u32, col: u32, done: u32, total: u32, flag: u32);
    /// `add_message` (0x004D6530) con un aviso de tipo 3 (obra terminada).
    fn add_message(&mut self, owner: u32, kind: u8, town: u8, ty: u16, prev: u16);
    fn live_business_count(&self) -> Option<usize> {
        None
    }
    fn live_business(&self, _index: u16) -> Option<[u8; 20]> {
        None
    }
    fn take_relocated_business(&mut self, _index: u16) -> Option<[u8; 20]> {
        None
    }
    fn put_live_business(&mut self, _index: u16, _record: [u8; 20]) -> Result<(), String> {
        Err("sin gestor de negocios vivos".into())
    }
    fn sync_active_office(&mut self, _image: &[u8]) {}
    fn apply_active_office(&mut self, _image: &mut [u8]) -> Result<(), String> {
        Ok(())
    }
}

/// Entorno de la obra.
pub struct AdvanceCtx<'a> {
    /// Número de comerciantes (`0x006DE4AA`).
    pub merchant_count: u16,
    /// Propietario (`+0xC324`) del objeto `0x006E51AC`, si existe.
    pub visual_owner: Option<u32>,
    pub host: &'a mut dyn AdvanceHost,
}

/// Dónde está el enlace que se está recorriendo: la cabeza de construcción o el `+4` de un registro.
#[derive(Clone, Copy)]
enum Cursor {
    Head,
    Rec(usize),
}

const FREE: usize = off::FREE_BUILDING_HEAD;
const PUBLIC: usize = off::PUBLIC_BUILDING_HEAD;
const DEFENSE: usize = off::DEFENSE_HEAD;

impl Site {
    fn cursor_get(&self, t: &Town, c: Cursor) -> u16 {
        match c {
            Cursor::Head => t.u16(off::CONSTRUCTION_HEAD),
            Cursor::Rec(i) => self.next(i),
        }
    }
    fn cursor_set(&mut self, t: &mut Town, c: Cursor, v: u16) {
        match c {
            Cursor::Head => t.set_u16(off::CONSTRUCTION_HEAD, v),
            Cursor::Rec(i) => self.set_next(i, v),
        }
    }

    /// Pone el registro `i` al frente de la lista cuya cabeza está en `Town+head`.
    fn push_front(&mut self, t: &mut Town, head: usize, i: usize) {
        self.set_next(i, t.u16(head));
        t.set_u16(head, i as u16);
    }

    /// Vacía el registro `i` y lo devuelve a la lista libre.
    fn release(&mut self, t: &mut Town, i: usize) {
        for k in 0..4 {
            self.set_rec(i, k, 0xFF);
        }
        self.set_next(i, t.u16(FREE));
        self.set_rec(i, 6, 0xFF);
        self.set_rec(i, 7, 0xFF);
        t.set_u16(FREE, i as u16);
    }

    /// Busca en la lista de defensa la pieza de muralla pendiente (tipo 0x37/0x38) de la misma casilla y
    /// la convierte en 0x34/0x35.
    fn finish_wall_piece(&mut self, t: &Town, row: u8, col: u8) {
        let count = Self::record_count(t);
        let mut i = t.u16(DEFENSE) as usize;
        while i < count {
            let mut next = self.next(i) as usize;
            if self.rec(i, 0) == row && self.rec(i, 1) == col {
                let ty = self.rec(i, 3);
                if ty == 0x37 || ty == 0x38 {
                    self.set_rec(i, 6, 0);
                    self.set_rec(i, 3, if ty == 0x37 { 0x34 } else { 0x35 });
                    next = 0xFFFF;
                }
            }
            i = next;
        }
    }

    /// Gasta `budget` de obra en la lista de construcción de la ciudad.
    pub fn advance_construction(&mut self, t: &mut Town, cx: &mut AdvanceCtx, mut budget: u32) {
        let dynamic = t.flags() >> 4 & 1 != 0;
        let skip_walls = t.flags() >> 11 & 1 != 0;
        let town = t.u8(off::TOWN_INDEX);
        let visible = cx.visual_owner == Some(town as u32);
        let mut cursor = Cursor::Head;
        while budget > 0 {
            let idx = self.cursor_get(t, cursor) as usize;
            if idx >= Self::record_count(t) {
                break;
            }
            let (row, col) = (self.rec(idx, 0), self.rec(idx, 1));
            if dynamic {
                let stride = t.u32(off::PLAN_STRIDE);
                let at = stride.wrapping_mul(row as u32).wrapping_add(col as u32) as usize;
                if self.cells[at] < 0x80 {
                    cursor = Cursor::Rec(idx);
                    continue;
                }
            }
            if skip_walls && self.rec(idx, 3) == 0x30 {
                cursor = Cursor::Rec(idx);
                continue;
            }
            let progress = self.rec(idx, 6);
            if progress > 5 || progress as u32 > budget {
                // Solo avanza una parte.
                let progress = if budget < 5 {
                    let low = budget as u8;
                    budget = 0;
                    progress.wrapping_sub(low)
                } else {
                    budget -= 5;
                    progress.wrapping_sub(5)
                };
                self.set_rec(idx, 6, progress);
                cursor = Cursor::Rec(idx);
                if visible && self.rec(idx, 3) != 0x28 {
                    let total = RECORD_BYTE_6[self.rec(idx, 3) as usize] as u32;
                    cx.host
                        .set_progress(row as u32, col as u32, total.wrapping_sub(progress as u32), total, 0);
                }
                continue;
            }
            // Termina el registro.
            if visible && self.rec(idx, 3) != 0x28 {
                cx.host.set_progress(row as u32, col as u32, 100, 100, 0);
            }
            let next = self.next(idx);
            budget -= progress as u32;
            self.set_rec(idx, 6, 0);
            self.cursor_set(t, cursor, next);
            let owner = self.rec(idx, 2);
            let ty = self.rec(idx, 3);
            if (owner as u16) < cx.merchant_count {
                let has_office = cx.host.find_office(owner as u32, town as u32, 0);
                match ty {
                    1 | 46 => {
                        t.set_u32(off::MORE_FLAGS, t.more_flags() | 0x80);
                        t.set_employees(1, 1);
                        t.add_i32(0x2E0, 4);
                        t.add_i32(off::CITIZENS_TOTAL, 4);
                        self.release(t, idx);
                    }
                    3 | 31..=38 | 45 | 47 => {
                        self.set_rec(idx, 2, 0xFF);
                        self.push_front(t, PUBLIC, idx);
                    }
                    4..=20 | 30 | 21..=29 => {
                        assert!(has_office, "el original lee la oficina sin comprobar que exista");
                        let which = (ty >= 21 && ty != 30) as usize;
                        let old = cx.host.swap_office_head(which, idx as u16);
                        self.set_next(idx, old);
                        cx.host.recount_office_with_site(self, t);
                    }
                    39..=44 => self.push_front(t, PUBLIC, idx),
                    48..=51 => {
                        self.set_rec(idx, 2, 0xFF);
                        self.push_front(t, DEFENSE, idx);
                    }
                    52 | 53 => {
                        self.finish_wall_piece(t, row, col);
                        self.release(t, idx);
                    }
                    54 | 55 | 56 => {
                        t.add_u16(off::DEFENSE_CAPACITY, if ty == 54 { 1 } else { 3 });
                        self.set_rec(idx, 2, 0xFF);
                        self.push_front(t, DEFENSE, idx);
                    }
                    _ => self.release(t, idx),
                }
                cx.host
                    .add_message(self.rec(idx, 2) as u32, 3, town, self.rec(idx, 3) as u16, idx as u16);
            } else {
                match ty {
                    1 => {
                        t.set_u32(off::MORE_FLAGS, t.more_flags() | 0x80);
                        self.release(t, idx);
                    }
                    3 | 31..=45 | 47 => self.push_front(t, PUBLIC, idx),
                    21..=29 => {
                        self.push_front(t, PUBLIC, idx);
                        cx.host.recount_buildings_with_site(self, t);
                    }
                    46 => {
                        t.set_u32(off::MORE_FLAGS, t.more_flags() | 0x20000);
                        self.release(t, idx);
                    }
                    48..=51 | 54..=56 => self.push_front(t, DEFENSE, idx),
                    52 | 53 => {
                        self.finish_wall_piece(t, row, col);
                        self.release(t, idx);
                    }
                    0 | 2 | 4..=20 | 30 => self.release(t, idx),
                    _ => {}
                }
            }
        }
    }
}

impl Site {
    /// `town_finish_all_pending_construction` (0x005205D0): termina de golpe toda la lista de
    /// construcción, como `advance_construction` con presupuesto infinito pero sin avisos ni mapa
    /// visual, y sin los contadores de los tipos 1/46 ni la capacidad de defensa. Las piezas de
    /// muralla sin comerciante (52 y 53) y los tipos mayores de 0x38 sin comerciante se descartan sin
    /// enlazarlos a ninguna lista, como en el original.
    pub fn finish_all_pending(&mut self, t: &mut Town, cx: &mut AdvanceCtx) {
        let town = t.u8(off::TOWN_INDEX) as u32;
        loop {
            let idx = t.u16(off::CONSTRUCTION_HEAD) as usize;
            if idx >= Self::record_count(t) {
                break;
            }
            let next = self.next(idx);
            t.set_u16(off::CONSTRUCTION_HEAD, next);
            let owner = self.rec(idx, 2);
            self.set_next(idx, 0xFFFF);
            self.set_rec(idx, 6, 0);
            let ty = self.rec(idx, 3);
            if (owner as u16) < cx.merchant_count {
                let has_office = cx.host.find_office(owner as u32, town, 0);
                match ty {
                    1 | 4..=30 => {
                        if ty == 1 {
                            t.set_u32(off::MORE_FLAGS, t.more_flags() | 0x80);
                        }
                        assert!(has_office, "el original lee la oficina sin comprobar que exista");
                        let which = (ty >= 21 && ty != 30) as usize;
                        let old = cx.host.swap_office_head(which, idx as u16);
                        self.set_next(idx, old);
                        cx.host.recount_office();
                    }
                    3 | 31..=38 | 45 | 47 => {
                        self.set_rec(idx, 2, 0xFF);
                        self.push_front(t, PUBLIC, idx);
                    }
                    39..=44 => self.push_front(t, PUBLIC, idx),
                    48..=51 | 54..=56 => {
                        self.set_rec(idx, 2, 0xFF);
                        self.push_front(t, DEFENSE, idx);
                    }
                    _ => self.release(t, idx),
                }
            } else {
                match ty {
                    0 | 2 | 4..=20 | 30 => self.release(t, idx),
                    1 => {
                        t.set_u32(off::MORE_FLAGS, t.more_flags() | 0x80);
                        self.release(t, idx);
                    }
                    3 | 21..=29 | 31..=45 | 47 => self.push_front(t, PUBLIC, idx),
                    46 => {
                        t.set_u32(off::MORE_FLAGS, t.more_flags() | 0x20000);
                        self.release(t, idx);
                    }
                    48..=51 | 54..=56 => self.push_front(t, DEFENSE, idx),
                    _ => {}
                }
            }
        }
    }
}
