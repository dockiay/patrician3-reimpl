//! Registros y órdenes de defensa de la ciudad (C05): piezas y tramos de muralla, y las órdenes que mejoran
//! torres y lanzadores de brea. Cubre `town_add_defense_piece_from_trace_entry` (0x005240A0),
//! `town_add_wall_segment_record` (0x00524140), `town_queue_op_upgrade_first_tower_site` (0x00524CD0),
//! `town_queue_op_tower_on_wall_segment_at` (0x00524D60), `town_queue_op_for_first_pitch_shoot` (0x00524E20)
//! y `town_queue_next_defense_upgrade_op` (0x00524F00). Las llamadas a otros sistemas pasan por `DefenseHost`.

use super::query::is_plan_point_valid;
use super::tables::RECORD_BYTE_6;
use super::Site;
use crate::town::{off, Town};

/// Tipos de la lista de defensa (`Town+0x768`).
const FIRST_TOWER: u8 = 0x30;
const PITCH_SHOOT: u8 = 0x32;
const WALL_SEGMENT: u8 = 0x36;
const TOWER_LEFT: u8 = 0x37;
const TOWER_RIGHT: u8 = 0x38;

/// Lo que piden estas funciones fuera de la ciudad.
pub trait DefenseHost {
    /// `class22_add_map_object` (0x00590680): (fila, columna, clase, parámetro, 0xFF).
    fn add_map_object(&mut self, row: u32, col: u32, class: u32, packed: u32, owner: u32);
    /// `enqueue_operation` (0x0054AA70): los cinco dwords del registro de operación.
    fn enqueue_operation(&mut self, record: &[u32; 5]);
    /// `get_merchant` (0x005303C0): palabra `+8` y byte `+0x19` del comerciante.
    fn merchant_fields(&mut self, merchant: u32) -> (u16, u8);
}

/// Registro de operación `0x22` que arma `operation_record_init` más los bytes `+4..+9`.
fn operation(bytes: [u8; 6]) -> [u32; 5] {
    [
        0x22,
        u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
        u32::from_le_bytes([bytes[4], bytes[5], 0, 0]),
        0,
        0,
    ]
}

impl Site {
    /// Alta de un registro en la cabeza de la lista de defensa con los campos comunes de los dos altas.
    fn push_defense_record(&mut self, t: &mut Town, fields: [u8; 4], counter: u16) -> usize {
        let i = self.alloc_building_record(t) as usize;
        for (k, v) in fields.into_iter().enumerate() {
            self.set_rec(i, k, v);
        }
        let head = t.u16(off::DEFENSE_HEAD);
        self.set_next(i, head);
        t.set_u16(off::DEFENSE_HEAD, i as u16);
        self.set_rec(i, 6, 0);
        t.set_u16(0x786, t.u16(0x786).wrapping_add(counter));
        i
    }

    /// `town_add_defense_piece_from_trace_entry`: `entry` son fila, columna, tipo y el byte del registro `+7`.
    pub fn add_defense_piece_from_trace_entry(
        &mut self,
        t: &mut Town,
        entry: [u8; 4],
        visual_owner: Option<u32>,
        host: &mut dyn DefenseHost,
    ) {
        let i = self.push_defense_record(t, [entry[0], entry[1], 0xFF, entry[2]], 3);
        self.set_rec(i, 7, entry[3]);
        if visual_owner == Some(t.u8(0x2C0) as u32) {
            host.add_map_object(
                entry[0] as u32,
                entry[1] as u32,
                0x2F,
                (entry[3] as u32) << 16 | 0xFFFF,
                0xFF,
            );
        }
    }

    /// `town_add_wall_segment_record`: tramo de muralla (tipo `0x36`) en (`row`, `col`) con el byte `extra`.
    pub fn add_wall_segment_record(
        &mut self,
        t: &mut Town,
        row: u8,
        col: u8,
        extra: u8,
        visual_owner: Option<u32>,
        host: &mut dyn DefenseHost,
    ) {
        let i = self.push_defense_record(t, [row, col, 0xFF, WALL_SEGMENT], 1);
        self.set_rec(i, 7, extra);
        if visual_owner == Some(t.u8(0x2C0) as u32) {
            host.add_map_object(row as u32, col as u32, 0x2E, extra as u32, 0xFF);
        }
    }

    /// Recorre la lista de defensa desde su cabeza y llama a `f` con cada registro hasta que devuelva `true`.
    fn find_defense(&self, t: &Town, mut f: impl FnMut(&Site, usize) -> bool) -> Option<usize> {
        let count = t.u16(off::BUILDING_RECORD_COUNT);
        let mut cur = t.u16(off::DEFENSE_HEAD);
        while cur < count {
            if f(self, cur as usize) {
                return Some(cur as usize);
            }
            cur = self.next(cur as usize);
        }
        None
    }

    /// `town_queue_op_upgrade_first_tower_site`: encola la mejora del primer registro de tipo `0x30`.
    pub fn queue_op_upgrade_first_tower_site(&self, t: &Town, host: &mut dyn DefenseHost) {
        if let Some(i) = self.find_defense(t, |s, i| s.rec(i, 3) == FIRST_TOWER) {
            host.enqueue_operation(&operation([
                t.u8(0x6F1),
                t.u8(0x2C0),
                self.rec(i, 0),
                self.rec(i, 1),
                0x33,
                0,
            ]));
        }
    }

    /// `town_queue_op_tower_on_wall_segment_at`: encola la mejora del tramo de muralla en (`row`, `col`).
    /// Las coordenadas se comparan como enteros de 32 bits con los bytes del registro.
    pub fn queue_op_tower_on_wall_segment_at(&self, t: &Town, row: i32, col: i32, host: &mut dyn DefenseHost) -> bool {
        let Some(i) = self.find_defense(t, |s, i| s.rec(i, 0) as i32 == row && s.rec(i, 1) as i32 == col) else {
            return false;
        };
        if self.rec(i, 3) != WALL_SEGMENT {
            return false;
        }
        host.enqueue_operation(&operation([
            t.u8(0x6F1),
            t.u8(0x2C0),
            self.rec(i, 0),
            self.rec(i, 1),
            0x32,
            0,
        ]));
        true
    }

    /// `town_queue_op_for_first_pitch_shoot`: busca el primer comerciante sin dueño de ciudad y encola la
    /// mejora del primer lanzador (`0x34` o `0x35`).
    pub fn queue_op_for_first_pitch_shoot(&self, t: &Town, merchant_count: u16, host: &mut dyn DefenseHost) {
        let owner = t.u8(0x2C0);
        let mut chosen = 0xFFu8;
        let mut id = 0u32;
        while id < merchant_count as u32 {
            let (word, byte) = host.merchant_fields(id);
            if word == 0 || byte != owner {
                chosen = id as u8;
            }
            id += 1;
            if chosen != 0xFF {
                break;
            }
        }
        if chosen == 0xFF {
            return;
        }
        if let Some(i) = self.find_defense(t, |s, i| matches!(s.rec(i, 3), 0x34 | 0x35)) {
            host.enqueue_operation(&operation([
                chosen,
                owner,
                self.rec(i, 0),
                self.rec(i, 1),
                self.rec(i, 3).wrapping_sub(3),
                self.rec(i, 7),
            ]));
        }
    }

    /// `town_queue_next_defense_upgrade_op`: busca, por orden de propietario, una torre (`0x37`/`0x38`) junto a
    /// un tramo de muralla que se pueda mejorar y, si no, el primer lanzador `0x32`.
    pub fn queue_next_defense_upgrade_op(&self, t: &Town, host: &mut dyn DefenseHost) {
        let count = t.u16(off::BUILDING_RECORD_COUNT);
        let mut limit: u32 = if is_plan_point_valid(t, 1) {
            0xFF
        } else {
            0xFD + is_plan_point_valid(t, 2) as u32
        };
        let towers = |s: &Site, i: usize, shift: [(i32, i32); 2], host: &mut dyn DefenseHost| -> bool {
            let (row, col) = (s.rec(i, 0) as i32, s.rec(i, 1) as i32);
            let (dr, dc) = match s.rec(i, 3) {
                TOWER_LEFT => shift[0],
                TOWER_RIGHT => shift[1],
                _ => return false,
            };
            s.queue_op_tower_on_wall_segment_at(t, row + dr, col + dc, host)
        };
        // Primera pasada: el límite baja al propietario de cada registro que lo respeta.
        let mut cur = t.u16(off::DEFENSE_HEAD);
        while cur < count {
            let i = cur as usize;
            let owner = self.rec(i, 2) as u32;
            if owner <= limit {
                limit = owner;
                if towers(self, i, [(3, -6), (-3, -6)], host) {
                    return;
                }
            }
            cur = self.next(i);
        }
        // Segunda pasada con el límite resultante, hacia el otro lado de la torre.
        let mut cur = t.u16(off::DEFENSE_HEAD);
        while cur < count {
            let i = cur as usize;
            if self.rec(i, 2) as u32 <= limit && towers(self, i, [(-1, 2), (1, 2)], host) {
                return;
            }
            cur = self.next(i);
        }
        let mut cur = t.u16(off::DEFENSE_HEAD);
        while cur < count {
            let i = cur as usize;
            if self.rec(i, 2) as u32 <= limit && self.rec(i, 3) == PITCH_SHOOT {
                host.enqueue_operation(&operation([
                    t.u8(0x6F1),
                    t.u8(0x2C0),
                    self.rec(i, 0),
                    self.rec(i, 1),
                    0x33,
                    0,
                ]));
                return;
            }
            cur = self.next(i);
        }
    }
}

/// Marca la huella de una pieza de defensa en la máscara: el punto, el de la fila anterior, su vecino según la
/// paridad de la columna y el de dos filas atrás. El original indexa con `col * Town+0x7A8 + fila`.
fn mark_defense_footprint(mask: &mut [u8], span: i64, x: i64, y: i64) {
    let at = y * span + x;
    mask[at as usize] = 1;
    let up = at - span;
    mask[up as usize] = 1;
    mask[(up + if y & 1 != 0 { 1 } else { -1 }) as usize] = 1;
    mask[(up - span) as usize] = 1;
}

impl Site {
    /// `town_build_defense_upgrade_site_mask` (0x005241E0): máscara de `Town+0x7A8 * Town+0x7AC` bytes con
    /// los puntos del plano donde cabe la mejora `mode`. `0x32` (lanzador de brea) marca los tramos de
    /// muralla que casan con una torre y pone a cero `Town+0x7A4`; `0x30`, `0x31` y `0x33` marcan los registros
    /// de tipo `mode - 1`; `0x34` y `0x35` marcan seis pares de celdas junto a cada torre `0x37`/`0x38` sin
    /// progreso. Otros valores dan una máscara vacía. El original guarda los pares en la pila (24 como
    /// máximo) y escribe sin comprobar el borde de la máscara; aquí lo segundo es un `panic`.
    pub fn build_defense_upgrade_site_mask(&self, t: &mut Town, mode: u32) -> Vec<u8> {
        let span = t.i32(0x7A8) as i64;
        let mut mask = vec![0u8; (span * t.i32(0x7AC) as i64) as usize];
        let count = t.u16(off::BUILDING_RECORD_COUNT);
        let head = t.u16(off::DEFENSE_HEAD);
        let pos = |s: &Site, i: usize| (s.rec(i, 0) as i64, s.rec(i, 1) as i64);
        if mode == 0x32 {
            let mut pairs: Vec<(i64, i64)> = Vec::new();
            let mut cur = head;
            while cur < count {
                let i = cur as usize;
                let (x, y) = pos(self, i);
                let first_second = match self.rec(i, 3) {
                    0x38 | 0x35 => Some((x + 1, x - 3)),
                    0x37 | 0x34 => Some((x - 1, x + 3)),
                    _ => None,
                };
                if let Some((a, b)) = first_second {
                    pairs.push((a, y + 2));
                    pairs.push((b, y - 6));
                }
                cur = self.next(i);
            }
            let mut cur = head;
            while cur < count {
                let i = cur as usize;
                if self.rec(i, 3) == WALL_SEGMENT {
                    let (x, y) = pos(self, i);
                    if let Some(k) = pairs.iter().position(|&p| p == (x, y)) {
                        mark_defense_footprint(&mut mask, span, x, y);
                        pairs.swap_remove(k);
                    }
                }
                cur = self.next(i);
            }
            t.set_u32(0x7A4, 0);
        } else if mode <= 0x33 {
            let want = mode.wrapping_sub(1);
            let mut cur = head;
            while cur < count {
                let i = cur as usize;
                if self.rec(i, 3) as u32 == want {
                    let (x, y) = pos(self, i);
                    mark_defense_footprint(&mut mask, span, x, y);
                }
                cur = self.next(i);
            }
        } else if mode == 0x34 || mode == 0x35 {
            let mut cur = head;
            while cur < count {
                let i = cur as usize;
                if self.rec(i, 6) == 0 {
                    let (x, y) = pos(self, i);
                    match self.rec(i, 3) {
                        TOWER_LEFT => {
                            let mut p = y * span + x - 1;
                            for step in 0..6 {
                                p += 1;
                                mask[p as usize] = 1;
                                p += ((y + step) & 1) - span - 1;
                                mask[p as usize] = 1;
                            }
                        }
                        TOWER_RIGHT => {
                            let mut p = y * span + x + 1;
                            for step in 0..6 {
                                p -= 1;
                                mask[p as usize] = 1;
                                p += ((y + step) & 1) - span;
                                mask[p as usize] = 1;
                            }
                        }
                        _ => {}
                    }
                }
                cur = self.next(i);
            }
        }
        mask
    }
}

/// Constantes del EXE que usa `town_complete_pitch_shoot`: un byte por mercancía (`0x00672AF8`,
/// `0x00672A80`, `0x00672ABC`, `0x00672B34`), el del precio base (`0x00672BAC`) y cinco `f32`
/// (`0x00673A64`, `0x00673A44`, `0x00673A48`, `0x0066BC6C`, `0x00673A5C`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PitchShootTables {
    /// Cantidades (por 2000, 2000, 200 y 2000) de las mercancías 0x13, 0x0B, 0x0C y 0x11.
    pub units: [u8; 4],
    pub price_base: u8,
    pub factors: [f32; 5],
}

impl Default for PitchShootTables {
    fn default() -> Self {
        PitchShootTables {
            units: [25, 5, 5, 0],
            price_base: 20,
            factors: [
                f32::from_bits(0x3D23_6E30),
                f32::from_bits(0x3CE1_47AF),
                f32::from_bits(0x3FA3_9581),
                0.0,
                f32::from_bits(0x3E61_47AF),
            ],
        }
    }
}

/// Lo que pide `town_complete_pitch_shoot` fuera de la ciudad.
pub trait PitchShootHost {
    /// `find_or_create_office` (0x005308A0): (comerciante, propietario de la ciudad, crear).
    fn find_office(&mut self, merchant: u32, town_owner: u32, create: u32) -> Option<u32>;
    /// `office_take_ware_from_stock_and_ships` (0x00500A70): pide `quantity` de la mercancía y devuelve lo que
    /// queda sin conseguir (el valor nuevo de la variable de salida).
    fn take_ware(&mut self, office: u32, ware: u32, quantity: i32) -> i32;
    /// `get_buy_price` (0x0052E430): (mercancía, propietario, cantidad).
    fn buy_price(&mut self, ware: u32, owner: u32, quantity: i32) -> i32;
    /// La ciudad ya ha descontado los materiales cuando se consulta el precio.
    fn buy_price_at(&mut self, _town: &Town, ware: u32, owner: u32, quantity: i32) -> i32 {
        self.buy_price(ware, owner, quantity)
    }
    /// `book_transaction` (0x00529F90): (importe, 6).
    fn book_transaction(&mut self, cost: i32, kind: u32);
    /// `class22_add_map_object` (0x00590680).
    fn add_map_object(&mut self, row: u32, col: u32, class: u32, packed: u32, owner: u32);
}

/// Mercancías que consume el lanzador de brea.
const WARE_A: u32 = 0x13;
const WARE_B: u32 = 0x0B;
const WARE_C: u32 = 0x0C;
const WARE_D: u32 = 0x11;

impl Site {
    /// `town_complete_pitch_shoot` (0x00523D70): termina el lanzador de brea (registro sin progreso en
    /// (`row`, `col`)): toma los materiales de la oficina del comerciante `merchant` (si existe) y el resto de
    /// las existencias de la ciudad, apunta el coste, baja el tipo del registro en 3 y marca el plano
    /// (`Town+0x7A4 = 1`). La mercancía `0x0C` pide su cantidad con la variable de la `0x0B`, como el original.
    #[allow(clippy::too_many_arguments)]
    pub fn complete_pitch_shoot(
        &mut self,
        t: &mut Town,
        row: u8,
        col: u8,
        merchant: u8,
        merchant_count: u16,
        visual_owner: Option<u32>,
        k: &PitchShootTables,
        host: &mut dyn PitchShootHost,
    ) {
        use crate::numeric::ftol_i32;
        let count = t.u16(off::BUILDING_RECORD_COUNT);
        let mut cur = t.u16(off::DEFENSE_HEAD);
        if cur >= count {
            return;
        }
        let found = loop {
            let i = cur as usize;
            if self.rec(i, 6) == 0 && self.rec(i, 0) == row && self.rec(i, 1) == col {
                break i;
            }
            cur = self.next(i);
            if cur >= count {
                return;
            }
        };
        let owner = t.u8(0x2C0) as u32;
        let (mut a, mut b, c, mut d) = (
            k.units[0] as i32 * 2000,
            k.units[1] as i32 * 2000,
            k.units[2] as i32 * 200,
            k.units[3] as i32 * 2000,
        );
        let mut cost = 0i32;
        let scaled = |taken: i32, factor: f32| ftol_i32((taken as f32 as f64) * factor as f64);
        if (merchant as u16) < merchant_count {
            if let Some(office) = host.find_office(merchant as u32, owner, 0) {
                if a > 0 {
                    let asked = a;
                    a = host.take_ware(office, WARE_A, a);
                    cost = scaled(asked.wrapping_sub(a), k.factors[0]);
                }
                if b > 0 {
                    let asked = b;
                    b = host.take_ware(office, WARE_B, b);
                    cost = cost.wrapping_add(scaled(asked.wrapping_sub(b), k.factors[1]));
                }
                if c > 0 {
                    b = host.take_ware(office, WARE_C, b);
                    cost = cost.wrapping_add(ftol_i32(k.factors[2] as f64 * k.factors[3] as f64));
                }
                if d > 0 {
                    let asked = d;
                    d = host.take_ware(office, WARE_D, d);
                    cost = cost.wrapping_add(scaled(asked.wrapping_sub(d), k.factors[4]));
                }
            }
        }
        for (field, spent) in [(0x50, a), (0x30, b), (0x34, c), (0x48, d)] {
            let left = t.i32(field).wrapping_sub(spent);
            t.set_i32(field, if left < 0 { 0 } else { left });
        }
        let base = (k.price_base as i32) * 100;
        cost = cost.wrapping_add(host.buy_price_at(t, WARE_A, owner, a).wrapping_add(base));
        for (ware, qty) in [(WARE_B, b), (WARE_C, c), (WARE_D, d)] {
            cost = cost.wrapping_add(host.buy_price_at(t, ware, owner, qty));
        }
        host.book_transaction(cost, 6);
        let i = found;
        self.set_rec(i, 3, self.rec(i, 3).wrapping_sub(3));
        if visual_owner == Some(owner) {
            host.add_map_object(
                row as u32,
                col as u32,
                0x2F,
                (self.rec(i, 7) as u32) << 16 | 0x1FF,
                0xFF,
            );
        }
        t.set_u32(0x7A4, 1);
    }
}

/// Lo que pide `town_build_wall_tower_stage` además de lo del lanzador de brea.
pub trait TowerStageHost: PitchShootHost {
    /// `get_merchant` (0x005303C0): suma `amount` al dinero (`+0`) y al campo `+0x49C` del comerciante.
    fn merchant_credit(&mut self, merchant: u32, amount: i32);
    /// `class22_set_building_progress_at_tile` (0x005891C0): (fila, columna, hecho, total, 0).
    fn set_progress(&mut self, row: u32, col: u32, done: u32, total: u32, flag: u32);
}

impl Site {
    /// `town_build_wall_tower_stage` (0x00523830): da un paso a una torre o lanzador de muralla en (`row`,
    /// `col`). Sobre una celda libre (`& 0x3F == 0x1C`) crea el registro de obra `0x30` y ocupa el bloque de
    /// 2×2. Si no, busca en la lista de defensa un registro `0x36`, `0x30` o `0x32` en ese punto y lo avanza
    /// (`0x36`→`0x32`, que pasa a la cola de obras salvo que la ciudad tenga el bit `0x10`; `0x30`→`0x31`;
    /// `0x32`→`0x33`). Después toma los materiales de la oficina de `merchant` y de las existencias de la
    /// ciudad, apunta el coste y avisa al mapa visual. `factors` son los `f32` del EXE de la función del lanzador
    /// de brea (se usan el 0, 1, 2 y 4).
    #[allow(clippy::too_many_arguments)]
    pub fn build_wall_tower_stage(
        &mut self,
        t: &mut Town,
        row: u8,
        col: u8,
        merchant: u8,
        merchant_count: u16,
        visual_owner: Option<u32>,
        factors: &[f32; 5],
        host: &mut dyn TowerStageHost,
    ) {
        use super::tables::{BASE_COST, BRICK, EXTRA_COST, IRON, TIMBER};
        use crate::numeric::ftol_i32;
        let stride = t.i32(off::PLAN_STRIDE) as usize;
        let i = if self.cells[stride * row as usize + col as usize] & 0x3F == 0x1C {
            let i = self.alloc_building_record(t) as usize;
            self.set_rec(i, 3, 0x30);
            self.set_next(i, 0xFFFF);
            self.enqueue_construction_record(t, i as u16);
            self.mark_2x2_occupied(t, row as i32, col as i32);
            i
        } else {
            let count = t.u16(off::BUILDING_RECORD_COUNT);
            let mut prev: Option<usize> = None;
            let mut cur = t.u16(off::DEFENSE_HEAD);
            let i = loop {
                if cur >= count {
                    return;
                }
                let i = cur as usize;
                if self.rec(i, 0) == row && self.rec(i, 1) == col && matches!(self.rec(i, 3), 0x36 | 0x30 | 0x32) {
                    break i;
                }
                prev = Some(i);
                cur = self.next(i);
            };
            let after = self.next(i);
            match prev {
                None => t.set_u16(off::DEFENSE_HEAD, after),
                Some(p) => self.set_next(p, after),
            }
            self.set_next(i, 0xFFFF);
            let relink_at_head = |site: &mut Site, t: &mut Town| {
                let head = t.u16(off::DEFENSE_HEAD);
                site.set_next(i, head);
                t.set_u16(off::DEFENSE_HEAD, i as u16);
            };
            match self.rec(i, 3) {
                0x36 => {
                    if t.flags() & 0x10 != 0 {
                        relink_at_head(self, t);
                        return;
                    }
                    self.set_rec(i, 3, 0x32);
                    let mut link: Option<usize> = None;
                    let mut at = t.u16(off::CONSTRUCTION_HEAD);
                    while at < count {
                        link = Some(at as usize);
                        at = self.next(at as usize);
                    }
                    match link {
                        None => t.set_u16(off::CONSTRUCTION_HEAD, i as u16),
                        Some(l) => self.set_next(l, i as u16),
                    }
                }
                0x30 => {
                    self.set_rec(i, 3, 0x31);
                    relink_at_head(self, t);
                }
                0x32 => {
                    relink_at_head(self, t);
                    if t.flags() & 0x10 != 0 {
                        return;
                    }
                    self.set_rec(i, 3, 0x33);
                }
                _ => return,
            }
            t.set_u32(0x7A4, 1);
            i
        };
        let owner = t.u8(0x2C0) as u32;
        let ty = self.rec(i, 3) as usize;
        let (mut a, mut b, mut c, mut d) = (
            BRICK[ty] as i32 * 2000,
            TIMBER[ty] as i32 * 2000,
            IRON[ty] as i32 * 200,
            EXTRA_COST[ty] as i32 * 2000,
        );
        let mut cost = 0i32;
        let scaled = |taken: i32, factor: f32| ftol_i32((taken as f32 as f64) * factor as f64);
        if (merchant as u16) < merchant_count {
            if let Some(office) = host.find_office(merchant as u32, owner, 0) {
                for (ware, left, factor) in [
                    (WARE_A, &mut a, factors[0]),
                    (WARE_B, &mut b, factors[1]),
                    (WARE_C, &mut c, factors[2]),
                    (WARE_D, &mut d, factors[4]),
                ] {
                    if *left > 0 {
                        let asked = *left;
                        *left = host.take_ware(office, ware, asked);
                        cost = cost.wrapping_add(scaled(asked.wrapping_sub(*left), factor));
                    }
                }
                host.merchant_credit(merchant as u32, cost);
            }
        }
        for (field, spent) in [(0x50, a), (0x30, b), (0x34, c), (0x48, d)] {
            let left = t.i32(field).wrapping_sub(spent);
            t.set_i32(field, if left < 0 { 0 } else { left });
        }
        cost = cost.wrapping_add(
            host.buy_price_at(t, WARE_A, owner, a)
                .wrapping_add(BASE_COST[ty] as i32 * 100),
        );
        for (ware, qty) in [(WARE_B, b), (WARE_C, c), (WARE_D, d)] {
            cost = cost.wrapping_add(host.buy_price_at(t, ware, owner, qty));
        }
        host.book_transaction(cost, 6);
        self.set_rec(i, 0, row);
        self.set_rec(i, 1, col);
        self.set_rec(i, 6, RECORD_BYTE_6[ty]);
        if ty == 0x30 {
            self.set_rec(i, 2, 0xFF);
            self.set_rec(i, 7, 0);
        }
        t.set_u8(0x788, t.u8(0x788).wrapping_add(1));
        if visual_owner == Some(owner) {
            if matches!(ty, 0x30 | 0x32) {
                host.add_map_object(row as u32, col as u32, 0x29, 0, 0xFF);
                let total = RECORD_BYTE_6[ty] as u32;
                host.set_progress(
                    row as u32,
                    col as u32,
                    total.wrapping_sub(self.rec(i, 6) as u32),
                    total,
                    0,
                );
            } else {
                host.add_map_object(row as u32, col as u32, 0x29, 1, 0xFF);
            }
        }
    }
}
