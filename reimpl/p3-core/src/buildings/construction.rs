//! Cola de obras de la ciudad: la lista `Town+0x762` de registros en construcción.

use super::tables::{BASE_COST, BRICK, EXTRA_COST, IRON, MAP_OBJECT_PARAM, OWNER_MAP_PARAM, RECORD_BYTE_6, TIMBER};
use super::{Site, NIL};
use crate::numeric::ftol_i32;
use crate::rng::CrtRand;
use crate::town::{off, Town};

/// Tipo de registro de obra que cuenta para ordenar la cola (`0x36`).
const QUEUE_MARK: u8 = 0x36;

impl Site {
    /// `town_enqueue_construction_record` (0x005254C0): enlaza el registro `i` en la lista de obras.
    /// Un edificio de tipo `>= 0x30` va siempre al final; el resto se inserta delante del segundo
    /// registro `0x36` consecutivo, o al final si no lo hay.
    pub fn enqueue_construction_record(&mut self, t: &mut Town, i: u16) {
        let count = t.u16(off::BUILDING_RECORD_COUNT);
        let head = t.u16(off::CONSTRUCTION_HEAD);
        let mut slot: Option<usize> = None; // None = cabeza de la lista, Some(r) = enlace del registro r
        let mut at = head;
        let set = |site: &mut Site, t: &mut Town, slot: Option<usize>, v: u16| match slot {
            None => t.set_u16(off::CONSTRUCTION_HEAD, v),
            Some(r) => site.set_next(r, v),
        };
        if self.rec(i as usize, 3) >= 0x30 {
            while at < count {
                slot = Some(at as usize);
                at = self.next(at as usize);
            }
        } else {
            let mut previous_mark = false;
            while at < count {
                let mark = self.rec(at as usize, 3) == QUEUE_MARK;
                if mark && previous_mark {
                    let current = match slot {
                        None => t.u16(off::CONSTRUCTION_HEAD),
                        Some(r) => self.next(r),
                    };
                    self.set_next(i as usize, current);
                    set(self, t, slot, i);
                    return;
                }
                previous_mark = mark;
                slot = Some(at as usize);
                at = self.next(at as usize);
            }
        }
        set(self, t, slot, i);
        self.set_next(i as usize, NIL);
    }

    /// `town_has_adjacent_same_type_building` (0x005259D0): ¿hay ya, en las listas de edificios de la
    /// oficina del comerciante o en la cola de obras con ese propietario, uno del mismo tipo en alguna
    /// de las 20 posiciones vecinas de (`row`, `col`)? `office_head` es la cabeza de la oficina que
    /// devuelve `find_or_create_office` (o `None`). Las coordenadas son bytes, como en el original.
    pub fn has_adjacent_same_type_building(
        &self,
        t: &Town,
        row: u8,
        col: u8,
        ty: u8,
        merchant: u8,
        office_head: Option<u16>,
    ) -> bool {
        let count = t.u16(off::BUILDING_RECORD_COUNT);
        let cells = adjacent_lot_cells(row, col);
        let near = |r: usize| cells.contains(&(self.rec(r, 0), self.rec(r, 1)));
        if let Some(mut at) = office_head {
            while at < count {
                let r = at as usize;
                if self.rec(r, 3) == ty && near(r) {
                    return true;
                }
                at = self.next(r);
            }
        }
        let mut at = t.u16(off::CONSTRUCTION_HEAD);
        while at < count {
            let r = at as usize;
            if self.rec(r, 3) == ty && self.rec(r, 2) == merchant && near(r) {
                return true;
            }
            at = self.next(r);
        }
        false
    }
}

/// Las 20 posiciones (fila, columna) que `town_has_adjacent_same_type_building` compara: cuatro
/// iniciales y, desde ahí, cada una sale de la cuarta anterior bajando una columna con el ajuste de
/// fila de la rejilla escalonada. Aritmética de bytes.
pub fn adjacent_lot_cells(row: u8, col: u8) -> [(u8, u8); 20] {
    let odd = col & 1;
    let mut p = [(0u8, 0u8); 20];
    p[0] = (row.wrapping_add(1), col.wrapping_add(6));
    p[1] = (odd.wrapping_add(row).wrapping_sub(2), col.wrapping_add(7));
    p[2] = (row.wrapping_sub(3), col.wrapping_sub(2));
    p[3] = (odd.wrapping_add(row).wrapping_add(3), col.wrapping_sub(3));
    for n in 4..20 {
        let (mut r, c) = p[n - 4];
        let c2 = c.wrapping_sub(1);
        if n & 1 != 0 {
            if c2 & 1 == 1 {
                r = r.wrapping_sub(1);
            }
        } else if c & 1 == 1 {
            r = r.wrapping_add(1);
        }
        p[n] = (r, c2);
    }
    p
}

// Factores del ejecutable (`0x00673A5C`, `0x00673A44`, `0x00673A64`, `0x00673A48`): son los `f32` de
// 0,22, 0,0275, 0,0399 y 1,278 tal como están almacenados, no los literales redondeados de Rust.
const COST_SCALE: f32 = f32::from_bits(0x3E61_47AF);
const TIMBER_SCALE: f32 = f32::from_bits(0x3CE1_47AF);
const BRICK_SCALE: f32 = f32::from_bits(0x3D23_6E30);
const IRON_SCALE: f32 = f32::from_bits(0x3FA3_9581);

/// Cantidad de la tabla (`u8`) por 125·16, la escala de los materiales: `n * 2000`.
fn scaled(n: u8) -> i32 {
    n as i32 * 2000
}

/// `(int)(float(n) * k)` como lo hace el original: `fild`/`fstp dword` y un `fmul` x87 de 53 bits.
fn float_part(n: i32, k: f32) -> i32 {
    ftol_i32((n as f32) as f64 * k as f64)
}

impl Site {
    /// `town_get_building_base_value` (0x0051F220): valor base del edificio del registro `i`, suma de
    /// su coste y de sus materiales. `store_town_flag` es `Town+0x2CC` de la ciudad cuyo índice
    /// guarda `Town+0x2C0`: con él a 0, un edificio de tipo 5 usa una fórmula propia.
    pub fn building_base_value(&self, t: &Town, i: u32, store_town_flag: i32) -> i32 {
        if i >= t.u16(off::BUILDING_RECORD_COUNT) as u32 {
            return 0;
        }
        let ty = self.rec(i as usize, 3) as usize;
        let mut cost = if ty == 5 && store_town_flag == 0 {
            let base = scaled(EXTRA_COST[5]) >> 1;
            float_part(base, COST_SCALE) + BASE_COST[5] as i32 * 60
        } else {
            let mut cost = BASE_COST[ty] as i32 * 100;
            if EXTRA_COST[ty] > 0 {
                cost += float_part(scaled(EXTRA_COST[ty]), COST_SCALE);
            }
            cost
        };
        let timber = float_part(scaled(TIMBER[ty]), TIMBER_SCALE);
        cost = cost
            .wrapping_add(float_part(scaled(BRICK[ty]), BRICK_SCALE))
            .wrapping_add(timber);
        float_part(IRON[ty] as i32 * 200, IRON_SCALE).wrapping_add(cost)
    }
}

/// Llamadas de `town_start_building_construction` a sistemas que aún no están reimplementados.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hook {
    /// `find_or_create_office` (0x005308A0): (comerciante, propietario, 0); devuelve la oficina o 0.
    FindOrCreateOffice,
    /// `class22_add_map_object` (0x00590680): (fila, columna, clase, parámetro, 0xFF o auxiliar).
    AddMapObject,
    /// `class22_set_building_progress_at_tile` (0x005891C0): (fila, columna, 0, 100, 0).
    SetBuildingProgress,
    /// `get_merchant` (0x005303C0): (comerciante).
    GetMerchant,
    /// `handle_operation_37_join_guild` (0x004F8560): (comerciante, propietario, 0).
    JoinGuild,
    /// `op_place_town_defense_pieces` (0x00522B20): (0).
    PlaceDefensePieces,
    /// `office_take_ware_from_stock_and_ships` (0x00500A70): (mercancía, cantidad pedida); devuelve la
    /// cantidad que queda en la variable de salida.
    TakeWaresFromOffice,
    /// `merchant_take_wares_from_ships_in_town` (0x004F5450): (mercancía, ciudad, cantidad pedida).
    TakeWaresFromShips,
    /// `get_buy_price` (0x0052E430): (mercancía, ciudad, cantidad).
    GetBuyPrice,
    /// `office_recount_buildings_and_facilities` (0x004FF2D0).
    RecountOffice,
    /// `merchant_add_buildings_reputation_well` (0x004F89E0): (propietario).
    ReputationWell,
    /// `merchant_add_buildings_reputation_civic_building` (0x004F8C70): (propietario).
    ReputationCivic,
    /// `merchant_add_buildings_reputation_hospital_chapel` (0x004F8A50): (propietario, 1).
    ReputationHospital,
    /// `town_build_wall_tower_stage` (0x00523830): (fila, columna, 0xFF).
    WallTowerStage,
    /// `op_start_town_defense_piece` (0x00522C70): (0xFF, tipo != 0x36).
    DefensePiece,
    /// `enqueue_operation` (0x0054AA70): los cinco dwords del registro de operación.
    EnqueueOperation,
}

/// Lo que `start_building_construction` pide fuera de la ciudad.
pub trait BuildHost {
    fn call(&mut self, hook: Hook, args: &[u32]) -> u32;
    /// La reputación usa el contador de pozos después de incrementarlo.
    fn reputation_well(&mut self, _town: &Town, owner: u32) {
        self.call(Hook::ReputationWell, &[owner]);
    }
    fn place_defense_pieces(&mut self, _site: &mut Site, _town: &mut Town, point: u32) {
        self.call(Hook::PlaceDefensePieces, &[point]);
    }
    /// Primer edificio de la oficina que devolvió `FindOrCreateOffice` (`Office+0x2CE`).
    fn office_building_head(&mut self, office: u32) -> u16;
}

/// Entorno del que lee `start_building_construction`.
pub struct BuildCtx<'a> {
    /// Número de comerciantes (`0x006DE4AA`).
    pub merchant_count: u16,
    /// Propietario (`+0xC324`) del objeto `0x006E51AC`, si existe.
    pub visual_owner: Option<u32>,
    /// Byte global `0x0067792F`.
    pub special: u8,
    /// Tabla `0x006DE4B8`: clase de cada propietario.
    pub owner_class: &'a [u8; 256],
    /// Dinero (`+0`) e ingresos (`+0x4C0`) del comerciante que devuelve `GetMerchant`.
    pub merchant_gold: &'a mut i32,
    pub merchant_income: &'a mut i32,
    pub host: &'a mut dyn BuildHost,
}

impl Site {
    /// Marca la celda como obra (`0x8B`) y avisa del cambio del plano.
    fn mark_site_cell(&mut self, t: &mut Town, row: u8, col: u8, value: u8, notify: bool) {
        let at = t.i32(off::PLAN_STRIDE) as usize * row as usize + col as usize;
        self.cells[at] = value;
        if notify {
            t.set_u32(0x7A4, 1);
        }
    }

    /// `town_start_building_construction` (0x00521900): empieza la obra del edificio de tipo `ty` en
    /// (`row`, `col`) para el comerciante `merchant` (>= número de comerciantes: la ciudad). Comprueba
    /// las banderas de ciudad que exige cada tipo, reserva el sitio, crea el registro de obra y cobra
    /// materiales. Devuelve si se empezó. `extra` es el quinto argumento (solo cuenta su byte bajo,
    /// salvo en los objetos de mapa de los tipos 21–29).
    #[allow(clippy::too_many_arguments)]
    pub fn start_building_construction(
        &mut self,
        t: &mut Town,
        cx: &mut BuildCtx,
        merchant: u8,
        row: u8,
        col: u8,
        ty: u8,
        extra: u32,
    ) -> bool {
        if ty >= 0x30 {
            return false;
        }
        let stride = t.i32(off::PLAN_STRIDE) as usize;
        let at = stride * row as usize + col as usize;
        let dynamic = t.flags() & 0x10 == 0x10 && self.cells[at] < 0x80;
        if !self.cell_matches_building_type(t, row as u32, col as u32, ty as u32, extra as u8, cx.special) {
            return false;
        }
        let tyi = ty as usize;
        let brick = BRICK[tyi] as i32 * 2000;
        let timber = TIMBER[tyi] as i32 * 2000;
        let iron = IRON[tyi] as i32 * 200;
        let mut extra_cost = EXTRA_COST[tyi] as i32 * 2000;
        if ty == 5 && t.i32(0x2CC) == 0 {
            extra_cost >>= 1;
        }
        let owner = t.u8(0x2C0);
        let has_merchant = (merchant as u16) < cx.merchant_count;
        let mut office = 0u32;
        if has_merchant {
            office = cx
                .host
                .call(Hook::FindOrCreateOffice, &[merchant as u32, owner as u32, 0]);
        } else if ty != 0x27
            && (t.i32(0x50) < brick || t.i32(0x30) < timber || t.i32(0x34) < iron || t.i32(0x48) < extra_cost)
        {
            return false;
        }
        let visual = cx.visual_owner == Some(owner as u32);
        // EBX y EBP al llegar al final común: parámetros de los objetos de mapa.
        let (mut ebx, mut ebp): (u32, u32) = (0, 0xFF);
        let (mut ty, mut extra) = (ty, extra);
        let mut flags = t.u32(0x76C);
        macro_rules! need {
            ($mask:expr, $want:expr) => {
                if flags & $mask != $want {
                    return false;
                }
            };
        }
        macro_rules! reserve {
            () => {{
                t.set_u32(0x76C, flags);
                self.mark_site_cell(t, row, col, 0x8B, true);
            }};
        }
        macro_rules! place {
            ($ty:expr, $extra:expr) => {
                self.place_building(t, row as i32, col as i32, $ty as u32, $extra as u8, cx.special)
            };
        }
        match ty {
            0 | 2 => return false,
            1 => {
                need!(0x50, 0x10);
                flags |= 0x40;
                reserve!();
            }
            3 => {
                need!(6, 2);
                flags |= 4;
                reserve!();
            }
            4 | 8..=10 | 15 | 16 | 20 | 6 | 7 | 11..=14 | 17..=19 => {
                if dynamic {
                    return false;
                }
                if self.cells[at] >= 0x80 {
                    t.add_u16(0x78C, 1);
                }
                t.add_u16(0x78E, 1);
                if matches!(ty, 4 | 8..=10 | 15 | 16 | 20) {
                    let near_office = cx
                        .host
                        .call(Hook::FindOrCreateOffice, &[merchant as u32, owner as u32, 0]);
                    let head = (near_office != 0).then(|| cx.host.office_building_head(near_office));
                    let near = self.has_adjacent_same_type_building(t, row, col, ty, merchant, head);
                    let byte = if near { t.u8(0x760) & 0x7F } else { 0xFF };
                    extra = (extra & !0xFF) | byte as u32;
                    ebx = byte as i8 as i32 as u32;
                    ebp = merchant as u32;
                }
                if !matches!(ty, 4 | 8..=10 | 15 | 16 | 20) {
                    ebp = merchant as u32;
                }
                place!(ty, extra);
            }
            5 => {
                place!(5, extra);
                ebp = merchant as u32;
            }
            21..=29 => {
                let new_ty = (((ty as i32 - 21) / 3 + 7) * 3) as u8;
                ty = new_ty;
                ebx = (extra & 0xFF) << 16;
                ebp = merchant as u32;
                place!(ty, extra);
            }
            30 => {
                ebx = extra & 0xFF;
                ebp = merchant as u32;
                place!(ty, extra);
            }
            31 => {
                need!(0x27F, 0x7F);
                flags |= 0x200;
                ebx = t.u8(0x7A3) as u32;
                reserve!();
            }
            32 => {
                need!(3, 1);
                flags |= 2;
                reserve!();
            }
            33 => {
                need!(0xC, 4);
                flags |= 8;
                reserve!();
                if Site::is_plan_point_valid(t, 0) {
                    cx.host.place_defense_pieces(self, t, 0);
                }
            }
            34 => {
                need!(0x2010, 0x10);
                flags |= 0x2000;
                reserve!();
                if has_merchant {
                    cx.host.call(Hook::GetMerchant, &[merchant as u32]);
                    cx.host.call(Hook::JoinGuild, &[merchant as u32, owner as u32, 0]);
                }
            }
            35 => {
                need!(1, 0);
                flags |= 1;
                reserve!();
                if cx.visual_owner == Some(owner as u32) && flags != 0 {
                    let rows = t.i32(off::PLAN_ROWS);
                    for k in 0..8 {
                        let (r, c) = (t.u8(0x7C0 + 2 * k), t.u8(0x7C1 + 2 * k));
                        if r as i32 >= rows {
                            break;
                        }
                        cx.host.call(Hook::AddMapObject, &[r as u32, c as u32, 0x16, 0, 0xFF]);
                    }
                }
                let i = self.alloc_building_record(t) as usize;
                let head = t.u16(off::PUBLIC_BUILDING_HEAD);
                self.set_rec(i, 0, t.u8(0x800));
                self.set_rec(i, 1, t.u8(0x801));
                self.set_rec(i, 2, 0xFF);
                self.set_rec(i, 3, 0x2D);
                self.set_next(i, head);
                self.set_rec(i, 6, 0);
                self.set_rec(i, 7, 0);
                t.set_u16(off::PUBLIC_BUILDING_HEAD, i as u16);
                if visual {
                    let param = OWNER_MAP_PARAM[cx.owner_class[owner as usize] as usize];
                    cx.host.call(
                        Hook::AddMapObject,
                        &[self.rec(i, 0) as u32, self.rec(i, 1) as u32, 0xA, param as u32, 0xFF],
                    );
                }
            }
            36 => {
                need!(0x30, 0x10);
                flags |= 0x20;
                reserve!();
            }
            37 => {
                need!(0x810, 0x10);
                flags |= 0x800;
                reserve!();
            }
            38 => {
                need!(0x4010, 0x10);
                flags |= 0x4000;
                t.set_u32(0x76C, flags);
                place!(0x26, extra);
            }
            39 => reserve!(),
            40 => {
                let used = t.u8(0x789);
                if used as i32 * 500 > t.i32(0x2D4) {
                    return false;
                }
                t.set_u8(0x789, used.wrapping_add(1));
                self.mark_site_cell(t, row, col, 0x92, false);
            }
            41 => {
                place!(0x29, extra);
                t.set_u8(0x772, t.u8(0x772).wrapping_add(1));
            }
            42 => {
                need!(0x600, 0x200);
                flags |= 0x400;
                t.set_u32(0x76C, flags);
                place!(0x2A, extra);
            }
            43 => {
                need!(0x1200, 0x200);
                flags |= 0x1000;
                t.set_u32(0x76C, flags);
                place!(0x2B, extra);
            }
            44 => {
                place!(0x29, extra);
                t.set_u8(0x770, t.u8(0x770).wrapping_add(1));
            }
            45 => {
                let class = cx.owner_class[owner as usize];
                extra = (extra & !0xFF) | class as u32;
                ebx = class as u32;
                place!(0x2D, extra);
            }
            46 => {
                need!(0x10080, 0x80);
                flags |= 0x10000;
                reserve!();
            }
            47 => place!(0x2B, extra),
            _ => return false,
        }
        // Final común: registro de obra, objetos de mapa, cobro y contadores.
        let i = self.alloc_building_record(t);
        let r = i as usize;
        self.set_rec(r, 1, col);
        self.set_rec(r, 3, ty);
        self.set_rec(r, 0, row);
        self.set_rec(r, 2, merchant);
        self.set_next(r, NIL);
        self.set_rec(r, 7, extra as u8);
        self.set_rec(r, 6, RECORD_BYTE_6[ty as usize]);
        self.enqueue_construction_record(t, i);
        if visual {
            cx.host.call(
                Hook::AddMapObject,
                &[
                    row as u32,
                    col as u32,
                    MAP_OBJECT_PARAM[ty as usize] as i32 as u32,
                    ebx,
                    ebp,
                ],
            );
            cx.host
                .call(Hook::SetBuildingProgress, &[row as u32, col as u32, 0, 0x64, 0]);
        }
        let (mut a1, mut a2, mut a3) = (brick, timber, iron);
        let is_toggle = |site: &Site, special: u8| site.rec(r, 7) == special;
        let toggle_counter = |t: &mut Town, equal: bool| {
            let n = t.u8(0x774);
            t.set_u8(0x774, if equal { n.wrapping_sub(1) } else { n.wrapping_add(1) });
        };
        if !has_merchant {
            if ty < 0x1E {
                let equal = is_toggle(self, cx.special);
                toggle_counter(t, equal);
            }
        } else {
            cx.host.call(Hook::GetMerchant, &[merchant as u32]);
            if office != 0 {
                a1 = cx.host.call(Hook::TakeWaresFromOffice, &[0x13, a1 as u32]) as i32;
                a2 = cx.host.call(Hook::TakeWaresFromOffice, &[0xB, a2 as u32]) as i32;
                a3 = cx.host.call(Hook::TakeWaresFromOffice, &[0xC, a3 as u32]) as i32;
                cx.host.call(Hook::TakeWaresFromOffice, &[0x11, extra_cost as u32]);
            } else {
                a1 = cx.host.call(Hook::TakeWaresFromShips, &[0x13, owner as u32, a1 as u32]) as i32;
                a2 = cx.host.call(Hook::TakeWaresFromShips, &[0xB, owner as u32, a2 as u32]) as i32;
                a3 = cx.host.call(Hook::TakeWaresFromShips, &[0xC, owner as u32, a3 as u32]) as i32;
                cx.host
                    .call(Hook::TakeWaresFromShips, &[0x11, owner as u32, extra_cost as u32]);
            }
            let base = BASE_COST[ty as usize] as i32;
            let mut cost = if ty == 5 && t.i32(0x2CC) == 0 {
                base * 3
            } else {
                base * 5
            } * 20;
            for (ware, amount) in [(0x13u32, a1), (0xB, a2), (0xC, a3)] {
                if amount > 0 {
                    cost =
                        cost.wrapping_add(cx.host.call(Hook::GetBuyPrice, &[ware, owner as u32, amount as u32]) as i32);
                }
            }
            *cx.merchant_gold = cx.merchant_gold.wrapping_sub(cost);
            *cx.merchant_income = cx.merchant_income.wrapping_add(cost);
            if ty >= 0x15 {
                if ty < 0x1E {
                    let equal = is_toggle(self, cx.special);
                    toggle_counter(t, equal);
                } else if ty == 0x1E {
                    let equal = is_toggle(self, cx.special);
                    let n = t.u8(0x774);
                    t.set_u8(0x774, if equal { n.wrapping_sub(1) } else { n.wrapping_add(1) });
                    if office != 0 {
                        cx.host.call(Hook::RecountOffice, &[]);
                    }
                } else if ty == 0x29 || ty == 0x2C {
                    cx.host.call(Hook::ReputationHospital, &[owner as u32, 1]);
                } else if ty == 0x28 {
                    cx.host.reputation_well(t, owner as u32);
                } else if ty == 0x2A || ty == 0x2B {
                    cx.host.call(Hook::ReputationCivic, &[owner as u32]);
                } else {
                    self.set_rec(r, 2, 0xFF);
                }
            }
        }
        for (o, taken) in [(0x50, a1), (0x30, a2), (0x34, a3)] {
            let left = t.i32(o).wrapping_sub(taken);
            t.set_i32(o, left.max(0));
        }
        true
    }
}

/// Entorno de `place_planned_building`: el de la construcción más lo que lee del mundo.
pub struct PlanCtx<'a> {
    pub build: BuildCtx<'a>,
    pub rng: &'a mut CrtRand,
    /// Dword `0x006DE4A4` (día del año, solo cuentan sus 3 bits bajos).
    pub day: u32,
    /// Byte `0x006DE4A1` (mes, solo cuentan sus 3 bits bajos).
    pub month: u8,
    /// Byte global `0x00677929`.
    pub alt_special: u8,
}

impl Site {
    /// `town_place_planned_building` (0x0051FAB0): elige dónde construir un edificio de tipo `ty` y,
    /// con `now`, lo empieza (`start_building_construction`, torre/muralla o pieza de defensa); sin
    /// `now` encola una operación. Devuelve 0 si no se puede, 0x37 si no hay sitio, 1 si lo
    /// encola o el resultado de `start_building_construction`.
    pub fn place_planned_building(&mut self, t: &mut Town, cx: &mut PlanCtx, ty: u8, now: bool) -> u32 {
        let (mut row, mut col): (u8, u8);
        let jitter = |rng: &mut CrtRand, v: u8| v.wrapping_add(((rng.rand() & 0x1F) as u8).wrapping_sub(0x10));
        let start = (t.u8(0x800), t.u8(0x801));
        match ty {
            1 => (row, col) = (t.u8(0x80A), t.u8(0x80B)),
            3 => (row, col) = start,
            0x15..=0x1D => {
                (row, col) = start;
                let mut edi = cx.day & 7;
                let count = t.u16(off::BUILDING_RECORD_COUNT) as usize;
                let mut at = t.u16(off::PUBLIC_BUILDING_HEAD) as usize;
                while at < count {
                    let rty = self.rec(at, 3);
                    if (0x15..0x1E).contains(&rty) {
                        (row, col) = (self.rec(at, 0), self.rec(at, 1));
                        if rty == ty {
                            if edi == 0 {
                                break;
                            }
                            edi = edi.wrapping_sub(2);
                        } else {
                            edi = edi.wrapping_sub(1);
                        }
                    }
                    at = self.next(at) as usize;
                }
                if !now && row == start.0 {
                    row = jitter(cx.rng, row);
                    col = jitter(cx.rng, col);
                }
            }
            0x1F..=0x25 | 0x2D => (row, col) = start,
            0x26 => (row, col) = (t.u8(0x806), t.u8(0x807)),
            0x2E => (row, col) = (t.u8(0x808), t.u8(0x809)),
            0x30 => {
                (row, col) = start;
                if now {
                    row = row.wrapping_add(3u8.wrapping_sub((cx.day & 7) as u8));
                    col = col.wrapping_add(3u8.wrapping_sub(cx.month & 7));
                } else {
                    row = jitter(cx.rng, row);
                    col = jitter(cx.rng, col);
                }
            }
            _ => return 0,
        }
        let tyi = ty as usize;
        if BRICK[tyi] as i32 * 2000 > t.i32(0x50)
            || TIMBER[tyi] as i32 * 2000 > t.i32(0x30)
            || IRON[tyi] as i32 * 200 > t.i32(0x34)
            || EXTRA_COST[tyi] as i32 * 2000 > t.i32(0x48)
        {
            return 0;
        }
        let mut extra = if (t.u8(0x774) as i8) > 0 {
            cx.build.special
        } else {
            cx.alt_special
        };
        let dynamic = t.flags() & 0x10 == 0x10;
        let special = cx.build.special;
        if !self.find_nearest_cell_for_type(t, &mut row, &mut col, ty as i32, extra, dynamic, special) {
            extra = if extra == cx.alt_special {
                cx.build.special
            } else {
                cx.alt_special
            };
            if !self.find_nearest_cell_for_type(t, &mut row, &mut col, ty as i32, extra, dynamic, special) {
                return 0x37;
            }
        }
        if now {
            return if ty < 0x30 {
                self.start_building_construction(t, &mut cx.build, 0xFF, row, col, ty, extra as u32) as u32
            } else if ty <= 0x33 {
                cx.build
                    .host
                    .call(Hook::WallTowerStage, &[row as u32, col as u32, 0xFF]);
                1
            } else {
                cx.build.host.call(Hook::DefensePiece, &[0xFF, (ty != 0x36) as u32]);
                1
            };
        }
        let record = [
            0x22u32,
            u32::from_le_bytes([0xFF, t.u8(0x2C0), row, col]),
            u32::from_le_bytes([ty, extra, 0, 0]),
            0,
            0,
        ];
        cx.build.host.call(Hook::EnqueueOperation, &record);
        1
    }
}
