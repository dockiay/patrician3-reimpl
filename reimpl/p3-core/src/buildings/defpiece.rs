//! Arranque de una pieza de defensa (C05): `op_start_town_defense_piece` (0x00522C70).
//!
//! Toma la primera pieza de la lista de muralla (`Town+0x766`) que pide el modo (`mode`: 0 tramos de muralla,
//! 1 torres, otro valor cualquiera), la valora, la pasa a la cola de obras, paga los materiales y marca el plano.
//! Si con ella la lista de muralla queda vacía, rehace el resto de la defensa: descarta las piezas de menor
//! prioridad, rellena el interior, recuenta, avisa a las oficinas y devuelve su parte de lo invertido. Las
//! consultas a otros sistemas (comerciantes, oficinas, mapa visual, mensajes, tareas) pasan por `StartPieceHost`.

use super::evaluate::BuildRequest;
use super::tables::BASE_COST;
use super::Site;
use crate::numeric::ftol_i32;
use crate::town::{off, Town};

/// Clase del objeto visual de cada tipo (`0x00672FC0`).
pub const MAP_KIND: [i8; 64] = [
    44, 27, 11, 8, 93, 87, 88, 91, 89, 83, 86, 100, 90, 94, 96, 85, 92, 99, 98, 97, 84, 81, 81, 81, 80, 80, 80, 42, 42,
    42, 102, 4, 3, 11, 7, 0, 2, 5, 9, 51, 82, 101, 103, 104, 105, 55, 1, 95, 41, 41, 41, 41, 47, 47, 46, 47, 47, 0, 0,
    0, -28, 0, -44, 1,
];

/// Mercancías que paga la obra, en el orden de `BuildRequest::need`.
const WARES: [u32; 4] = [0x13, 0x0B, 0x0C, 0x11];
/// Unidades por tonel de cada una: 2000, 2000, 200 y 2000.
const UNITS: [i32; 4] = [2000, 2000, 200, 2000];
/// Campos de la ciudad con las existencias de cada una.
const STOCK: [usize; 4] = [0x50, 0x30, 0x34, 0x48];

/// Los campos de una oficina que recorre esta función: siguiente (`+0x2CA`), listas de edificios (`+0x2CE` y
/// `+0x2D0`) y comerciante (`+0x2C4`).
#[derive(Clone, Copy, Debug, Default)]
pub struct OfficeInfo {
    pub next: u16,
    pub head: u16,
    pub head2: u16,
    pub merchant: u16,
}

/// Campos del comerciante que lee o escribe (todos los comerciantes comparten registro en las pruebas).
#[derive(Clone, Copy, Debug, Default)]
pub struct MerchantInfo {
    /// `+8`: palabra que vale 0 si el comerciante es del jugador.
    pub word8: u16,
    /// `+0x1C` y `+0x1D`.
    pub b1c: u8,
    pub b1d: u8,
}

/// Carta que se envía al comerciante: los campos `+4`, `+5`, `+8`, `+0xA` y `+0xC`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Letter {
    pub kind: u8,
    pub owner: u8,
    pub w8: u16,
    pub wa: u16,
    pub wc: u16,
}

/// Entorno global del que lee la función.
pub struct StartEnv<'a> {
    /// Número de comerciantes (`0x006DE4AA`) y de oficinas (`0x006DE4A8`).
    pub merchant_count: u16,
    pub office_count: u16,
    pub offices: &'a [OfficeInfo],
    /// Propietario del objeto visual (`0x006E51AC`+`0xC324`), si hay objeto.
    pub visual_owner: Option<u32>,
    /// Reloj del mundo (`0x006DE4B4`).
    pub tick: u32,
    /// Byte global `0x0067792F`.
    pub special: u8,
    /// Factores `f32` de precio de las cuatro mercancías (`0x00673A64`, `A44`, `A48` y `A5C`).
    pub factors: [f32; 4],
}

/// Lo que pide `op_start_town_defense_piece` fuera de la ciudad.
pub trait StartPieceHost {
    /// `find_or_create_office` (0x005308A0).
    fn find_office(&mut self, merchant: u32, town_owner: u32, create: u32) -> Option<u32>;
    /// `get_merchant` (0x005303C0): el registro (único) del comerciante.
    fn get_merchant(&mut self, merchant: u32) -> MerchantInfo;
    /// `town_evaluate_build_request` (0x0051F3A0) con tipo y comerciante: la petición, o `None` si devuelve 0.
    fn evaluate(&mut self, ty: u16, merchant: u16) -> Option<BuildRequest>;
    /// `office_take_ware_from_stock_and_ships` (0x00500A70): lo que queda sin conseguir.
    fn take_ware(&mut self, office: u32, ware: u32, quantity: i32) -> i32;
    /// Suma `amount` al dinero (`+0`) y a `+0x49C` del comerciante.
    fn merchant_credit(&mut self, amount: i32);
    /// Suma `amount` a `+0x4BC` del comerciante.
    fn merchant_add_4bc(&mut self, amount: i32);
    /// `book_transaction` (0x00529F90): (importe, tipo).
    fn book_transaction(&mut self, amount: i32, kind: u32);
    /// `class22_add_map_object` (0x00590680).
    fn add_map_object(&mut self, row: u32, col: u32, class: u32, packed: u32, owner: u32);
    /// `class22_set_building_progress_at_tile` (0x005891C0): (fila, columna, 0, byte 6, 0).
    fn set_progress(&mut self, row: u32, col: u32, done: u32, total: u32, flag: u32);
    /// `class22_remove_building_with_effects` (0x00599040).
    fn remove_building(&mut self, row: u32, col: u32);
    /// `add_message` (0x004D6530): (comerciante, carta).
    fn add_message(&mut self, merchant: u32, letter: &Letter);
    /// `scheduled_tasks_add_task` (0x004D8CF0): hora y los 0x20 bytes de la tarea que rellena la función.
    fn add_task(&mut self, time: u32, task: &[u8; 0x20]);
}

impl Site {
    /// Vuelve a poner en la cola de borrado (`buf`) la esquina de una pieza que se quita.
    fn drop_defense_record(&mut self, t: &mut Town, i: usize, link: Option<usize>, head_off: usize) {
        let next = self.next(i);
        match link {
            None => t.set_u16(head_off, next),
            Some(l) => self.set_next(l, next),
        }
        let free = t.u16(off::FREE_BUILDING_HEAD);
        self.set_next(i, free);
        t.set_u16(off::FREE_BUILDING_HEAD, i as u16);
    }

    /// `op_start_town_defense_piece`. `merchant` es el primer argumento y `mode` el segundo.
    pub fn start_town_defense_piece(
        &mut self,
        t: &mut Town,
        env: &StartEnv,
        merchant: u32,
        mode: u32,
        host: &mut dyn StartPieceHost,
    ) {
        if t.flags() & 0x10 != 0 {
            return;
        }
        let owner = t.u8(0x2C0) as u32;
        let count = t.u16(off::BUILDING_RECORD_COUNT);
        let office = if merchant < env.merchant_count as u32 {
            match host.find_office(merchant, owner, 0) {
                None => return,
                Some(o) => {
                    host.get_merchant(merchant);
                    Some(o)
                }
            }
        } else {
            None
        };
        // Primera pieza de la lista de muralla que pide el modo, y la anterior a ella.
        let mut prev: Option<usize> = None;
        let rec = loop {
            let cur = match prev {
                None => t.u16(off::WALL_HEAD),
                Some(l) => self.next(l),
            };
            if cur >= count {
                return;
            }
            let i = cur as usize;
            if (self.rec(i, 3) == 0x36) as u32 == mode {
                prev = Some(i);
                continue;
            }
            break i;
        };
        let Some(req) = host.evaluate(self.rec(rec, 3) as u16, merchant as u16) else {
            return;
        };
        let after = self.next(rec);
        let old = match prev {
            None => {
                let o = t.u16(off::WALL_HEAD);
                t.set_u16(off::WALL_HEAD, after);
                o
            }
            Some(p) => {
                let o = self.next(p);
                self.set_next(p, after);
                o
            }
        };
        self.set_next(rec, 0xFFFF);
        self.enqueue_construction_record(t, old);
        if let Some(office) = office {
            let mut spent = 0i32;
            for k in 0..4 {
                let asked = req.need[k].wrapping_mul(UNITS[k]);
                if asked > 0 {
                    let left = host.take_ware(office, WARES[k], asked);
                    let taken = asked.wrapping_sub(left);
                    spent = spent.wrapping_add(ftol_i32((taken as f32 as f64) * env.factors[k] as f64));
                }
            }
            host.merchant_credit(spent);
        }
        host.book_transaction(req.total, 6);
        for k in 0..4 {
            let left = t
                .i32(STOCK[k])
                .wrapping_add(req.short[k].wrapping_mul(UNITS[k]).wrapping_neg());
            t.set_i32(STOCK[k], if left < 0 { 0 } else { left });
        }
        let (row, col, ty) = (self.rec(rec, 0), self.rec(rec, 1), self.rec(rec, 3));
        let packed;
        if mode == 0 {
            packed = self.rec(rec, 7) as u32;
        } else {
            let mut g = self.grid(t);
            if ty == 0x38 {
                g.stamp_defense_cells(row as i32 - 1, col as i32 - 2);
                g.stamp_defense_cells(row as i32 - 2, col as i32 - 4);
            } else {
                g.stamp_defense_cells(row as i32 + 1, col as i32 - 2);
                g.stamp_defense_cells(row as i32 + 2, col as i32 - 4);
            }
            self.mark_defense_piece_fit_flags(t, row as i32, col as i32, ty as u32);
            packed = (self.rec(rec, 7) as u32) << 16 | 0xFF;
        }
        self.grid(t).stamp_defense_cells(row as i32, col as i32);
        let visual = env.visual_owner == Some(owner);
        if visual {
            host.add_map_object(
                row as u32,
                col as u32,
                MAP_KIND[ty as usize] as i32 as u32,
                packed,
                self.rec(rec, 2) as u32,
            );
            host.set_progress(row as u32, col as u32, 0, self.rec(rec, 6) as u32, 0);
        }
        // La lista de muralla vacía: se rehace el resto de la defensa.
        if t.u16(off::WALL_HEAD) < t.u16(off::BUILDING_RECORD_COUNT) {
            return;
        }
        let own = self.rec(rec, 2) as u32;
        t.set_u16(0x786, 0);
        let mut removed: Vec<(u8, u8)> = Vec::new();
        // Lista de defensa (`+0x768`): se quitan las piezas de otro propietario con prioridad menor.
        let mut link: Option<usize> = None;
        loop {
            let cur = match link {
                None => t.u16(off::DEFENSE_HEAD),
                Some(l) => self.next(l),
            };
            if cur >= t.u16(off::BUILDING_RECORD_COUNT) {
                break;
            }
            let i = cur as usize;
            let rty = self.rec(i, 3);
            if rty >= 0x32 {
                if own < self.rec(i, 2) as u32 {
                    let (r, c) = (self.rec(i, 0), self.rec(i, 1));
                    let extra = self.rec(i, 7) as u32;
                    self.grid(t)
                        .clear_building_footprint(r as u32, c as u32, rty as u32, extra, env.special);
                    removed.push((r, c));
                    if visual {
                        let (r, c) = (r as i32, c as i32);
                        host.remove_building(r as u32, c as u32);
                        match rty {
                            0x36 => {}
                            0x37 | 0x34 => {
                                host.remove_building((r + 1) as u32, (c - 2) as u32);
                                host.remove_building((r + 2) as u32, (c - 4) as u32);
                            }
                            0x38 | 0x35 => {
                                host.remove_building((r - 1) as u32, (c - 2) as u32);
                                host.remove_building((r - 2) as u32, (c - 4) as u32);
                            }
                            _ => {}
                        }
                    }
                    self.drop_defense_record(t, i, link, off::DEFENSE_HEAD);
                    continue;
                }
                let add = if rty == 0x36 || rty < 0x34 { 1 } else { 3 };
                t.set_u16(0x786, t.u16(0x786).wrapping_add(add));
            }
            link = Some(i);
        }
        if t.u16(0x786) == 0 {
            t.set_u16(0x786, 1);
        }
        // Cola de obras (`+0x762`): igual, y los registros quitados quedan con tipo 0xFF.
        let mut link: Option<usize> = None;
        loop {
            let cur = match link {
                None => t.u16(off::CONSTRUCTION_HEAD),
                Some(l) => self.next(l),
            };
            if cur >= t.u16(off::BUILDING_RECORD_COUNT) {
                break;
            }
            let i = cur as usize;
            let rty = self.rec(i, 3);
            if own < self.rec(i, 2) as u32 && rty >= 0x32 {
                let (r, c) = (self.rec(i, 0), self.rec(i, 1));
                let extra = self.rec(i, 7) as u32;
                self.grid(t)
                    .clear_building_footprint(r as u32, c as u32, rty as u32, extra, env.special);
                if visual {
                    host.remove_building(r as u32, c as u32);
                }
                self.drop_defense_record(t, i, link, off::CONSTRUCTION_HEAD);
                self.set_rec(i, 3, 0xFF);
                continue;
            }
            link = Some(i);
        }
        // Piezas que quedan en la cola de obras.
        let mut cur = t.u16(off::CONSTRUCTION_HEAD);
        while cur < t.u16(off::BUILDING_RECORD_COUNT) {
            let rty = self.rec(cur as usize, 3);
            if rty >= 0x32 {
                let add = if rty <= 0x36 { 1 } else { 3 };
                t.set_u16(0x786, t.u16(0x786).wrapping_add(add));
            }
            cur = self.next(cur as usize);
        }
        t.set_u32(0x76C, t.u32(0x76C) | 0x10);
        self.flood_fill_inside_walls(t);
        t.set_u8(0x788, 0);
        t.set_u16(0x78C, 1);
        t.set_u16(0x78E, 1);
        t.set_u16(0x786, 0);
        let head = t.u16(off::DEFENSE_HEAD) as u32;
        self.count_defense_buildings(t, head);
        let mut o = t.u16(0x784);
        while o < env.office_count {
            let info = env.offices[o as usize];
            self.count_defense_buildings(t, info.head as u32);
            o = info.next;
        }
        if t.u16(0x786) == 0 {
            t.set_u16(0x786, 1);
        }
        // Carta al alcalde.
        let mayor = t.u8(0x6F1);
        let mut letter = Letter {
            kind: 0x2F,
            owner: t.u8(0x2C0),
            ..Default::default()
        };
        if (mayor as u16) < env.merchant_count {
            let m = host.get_merchant(mayor as u32);
            letter.wa = (m.b1c as u16) << 8;
            letter.wa = letter.wa.wrapping_add(m.b1d as u16);
        } else {
            let idx = (!mayor) as usize & 3;
            letter.wa = (t.u8(0x6D4 + idx) as u16) << 8;
            letter.wa = letter.wa.wrapping_add(t.u8(0x6D8 + idx) as u16);
        }
        // Torres en obra sin la pieza que las completa: se descartan (lista de obras, tipos 0x34 y 0x35).
        let mut link: Option<usize> = None;
        loop {
            let cur = match link {
                None => t.u16(off::CONSTRUCTION_HEAD),
                Some(l) => self.next(l),
            };
            let n = t.u16(off::BUILDING_RECORD_COUNT);
            if cur >= n {
                break;
            }
            let i = cur as usize;
            let rty = self.rec(i, 3);
            if rty == 0x34 || rty == 0x35 {
                let mut found = false;
                let mut d = t.u16(off::DEFENSE_HEAD);
                while d < n {
                    let r2 = d as usize;
                    let next2 = self.next(r2);
                    if self.rec(r2, 3) >= 0x37
                        && (self.rec(r2, 3) as i32 - rty as i32) == 3
                        && self.rec(r2, 0) == self.rec(i, 0)
                        && self.rec(r2, 1) == self.rec(i, 1)
                    {
                        found = true;
                        d = 0xFFFF;
                    } else {
                        d = next2;
                    }
                }
                if !found {
                    self.drop_defense_record(t, i, link, off::CONSTRUCTION_HEAD);
                    for k in 0..4 {
                        self.set_rec(i, k, 0xFF);
                    }
                    self.set_rec(i, 6, 0);
                    self.set_rec(i, 7, 0);
                    continue;
                }
            }
            link = Some(i);
        }
        // Devolución a los comerciantes cuando el alcalde no es uno de ellos.
        if mayor as u16 >= env.merchant_count {
            let mut o = t.u16(0x784);
            while o < env.office_count {
                let info = env.offices[o as usize];
                let n = t.u16(off::BUILDING_RECORD_COUNT);
                let mut sum = 0i32;
                for head in [info.head, info.head2] {
                    let mut cur = head;
                    while cur < n {
                        sum = sum.wrapping_add(BASE_COST[self.rec(cur as usize, 3) as usize] as i32);
                        cur = self.next(cur as usize);
                    }
                }
                let limit = (t.u16(0x786) as i32).wrapping_mul(500);
                let refund = if sum > limit { limit } else { sum };
                let m = host.get_merchant(info.merchant as u32);
                if m.word8 == 0 && refund > 0 {
                    letter.w8 = (m.b1c as u16) << 8;
                    letter.wc = refund as u16;
                    letter.w8 = letter.w8.wrapping_add(m.b1d as u16);
                    host.add_message(info.merchant as u32, &letter);
                }
                let amount = refund.wrapping_mul(100);
                if amount > 0 {
                    let mut task = [0u8; 0x20];
                    task[6..8].copy_from_slice(&1u16.to_le_bytes());
                    task[8..12].copy_from_slice(&amount.to_le_bytes());
                    task[0x10] = 0xFF;
                    task[0x11] = info.merchant as u8;
                    task[0x14] = t.u8(0x2C0);
                    task[0xC..0x10].copy_from_slice(&env.tick.to_le_bytes());
                    task[0xC] = 0;
                    host.add_task(env.tick.wrapping_add(0x1400), &task);
                    host.merchant_add_4bc(amount);
                    host.book_transaction(amount, 2);
                }
                o = info.next;
            }
        }
        // Vuelve a calcular los encajes alrededor de lo quitado y los pares de parcelas.
        let mut n = removed.len() * 2;
        while n > 1 {
            n -= 2;
            let (r, c) = removed[n / 2];
            self.grid(t).restore_fit_flags_around(r as i32, c as i32, 0);
        }
        let mut row = t.i32(off::PLAN_ROWS) - 0xC;
        while row > 0xC {
            self.grid(t).update_lot_pair_flags(row);
            row -= 0x12;
        }
    }
}
