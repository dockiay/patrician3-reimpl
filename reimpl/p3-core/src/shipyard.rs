//! Astillero (C05): cola de reparaciones y cola de obras de barcos de una ciudad.
//!
//! El gestor de barcos del original (`ECX` de estas funciones) guarda el array de barcos de 0x180 bytes
//! en `+4`, el de entradas de obra de 60 bytes en `+8`, las listas de barcos atracados en `+0xE8` y de
//! entradas libres en `+0xEC`, y sus recuentos en `+0xF4` y `+0xF6`. Los barcos y las entradas se
//! enlazan por índices `u16` (fin 0xFFFF). Aquí van como bytes con accesores por desplazamiento.

/// Tamaño de un barco.
pub const SHIP_SIZE: usize = 0x180;
/// Tamaño de una entrada de obra.
pub const YARD_SIZE: usize = 60;

/// Campos de ciudad que tocan el astillero.
pub mod town_off {
    /// `i32`: trabajo acumulado del astillero.
    pub const WORK: usize = 0x814;
    /// `u16`: primer barco de la cola de reparaciones.
    pub const REPAIR_HEAD: usize = 0x81E;
    /// `u16`: primera entrada de la cola de obras.
    pub const YARD_HEAD: usize = 0x820;
}

/// El gestor de barcos con sus dos arrays.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ShipManager {
    pub ships: Vec<u8>,
    pub entries: Vec<u8>,
    /// `+0xE8`: primer barco atracado.
    pub docked_head: u16,
    /// `+0xEC`: primera entrada libre.
    pub free_entry_head: u16,
    /// `+0xF4`.
    pub ship_count: u16,
    /// `+0xF6`.
    pub entry_count: u16,
}

/// Lo que el astillero pide al resto del juego, en el orden de las llamadas del original.
pub trait ShipyardHost {
    /// `get_merchant(owner)`: el valor `u16` en `+8` del comerciante.
    fn merchant_flag(&mut self, owner: u8) -> u16;
    /// `do_notification` (0x00548CA0): propietario, 0, 0, ciudad, tipo y barco.
    fn notify(&mut self, owner: u16, town: u32, kind: u8, ship: u32);
    /// `harbor_scene_add_berthed_ships` (0x00613920).
    fn berth(&mut self, ship: u32);
}

/// Entorno de una llamada.
pub struct ShipyardCtx<'a> {
    /// Número de comerciantes (`u16` en 0x006DE4AA).
    pub merchant_count: u16,
    pub host: &'a mut dyn ShipyardHost,
}

impl ShipManager {
    fn ship(&self, i: u16) -> usize {
        i as usize * SHIP_SIZE
    }
    fn entry(&self, i: u16) -> usize {
        i as usize * YARD_SIZE
    }
    pub fn s16(&self, i: u16, o: usize) -> u16 {
        let b = self.ship(i) + o;
        u16::from_le_bytes([self.ships[b], self.ships[b + 1]])
    }
    pub fn set_s16(&mut self, i: u16, o: usize, v: u16) {
        let b = self.ship(i) + o;
        self.ships[b..b + 2].copy_from_slice(&v.to_le_bytes());
    }
    pub fn s32(&self, i: u16, o: usize) -> i32 {
        let b = self.ship(i) + o;
        i32::from_le_bytes(self.ships[b..b + 4].try_into().unwrap())
    }
    pub fn set_s32(&mut self, i: u16, o: usize, v: i32) {
        let b = self.ship(i) + o;
        self.ships[b..b + 4].copy_from_slice(&v.to_le_bytes());
    }
    pub fn s8(&self, i: u16, o: usize) -> u8 {
        self.ships[self.ship(i) + o]
    }
    pub fn set_s8(&mut self, i: u16, o: usize, v: u8) {
        let b = self.ship(i) + o;
        self.ships[b] = v;
    }
    pub fn e16(&self, i: u16, o: usize) -> u16 {
        let b = self.entry(i) + o;
        u16::from_le_bytes([self.entries[b], self.entries[b + 1]])
    }
    pub fn set_e16(&mut self, i: u16, o: usize, v: u16) {
        let b = self.entry(i) + o;
        self.entries[b..b + 2].copy_from_slice(&v.to_le_bytes());
    }
    pub fn e8(&self, i: u16, o: usize) -> u8 {
        self.entries[self.entry(i) + o]
    }

    /// `unlink_docked_ship` (0x00508FC0): saca el barco `ship` de la lista de atracados.
    pub fn unlink_docked_ship(&mut self, ship: u16) {
        // `slot` identifica el enlace anterior: None es la cabeza (+0xE8), Some(i) el campo +6 del barco i.
        let mut slot: Option<u16> = None;
        let mut cur = self.docked_head;
        while cur != ship {
            if cur >= self.ship_count {
                return;
            }
            slot = Some(cur);
            cur = self.s16(cur, 6);
        }
        let next = self.s16(ship, 6);
        match slot {
            None => self.docked_head = next,
            Some(p) => self.set_s16(p, 6, next),
        }
        self.set_s16(ship, 6, 0xFFFF);
    }

    /// `advance_repairs` (0x00508AA0): avanza `budget` de trabajo en la cola de reparaciones y en la de
    /// obras de la ciudad `town_index`; `town` son sus bytes (solo `+0x814`, `+0x81E` y `+0x820`).
    pub fn advance_repairs(
        &mut self,
        town: &mut crate::town::Town,
        town_index: u32,
        budget: i32,
        cx: &mut ShipyardCtx,
    ) {
        use town_off::*;
        // ---- cola de reparaciones: `slot` es la cabeza de la ciudad (None) o el +6 de un barco.
        let mut slot: Option<u16> = None;
        let read = |m: &Self, t: &crate::town::Town, s: Option<u16>| match s {
            None => t.u16(REPAIR_HEAD),
            Some(p) => m.s16(p, 6),
        };
        let mut head = read(self, town, slot);
        while head < self.ship_count {
            let i = head;
            let progress = self.s32(i, 0x18).wrapping_add(budget);
            let level = self.s16(i, 0x3E) as i16;
            self.set_s8(i, 0x3D, self.s8(i, 0x3D) | 1);
            let flags = self.s16(i, 0x3C);
            self.set_s32(i, 0x18, progress);
            if level < 0x700 {
                self.set_s16(i, 0x3E, (level as i32 + 0x100) as u16);
                self.set_s16(i, 0x3C, flags | 0x200);
            } else {
                self.set_s16(i, 0x3E, 0x700);
            }
            let target = self.s32(i, 0x14);
            let finished = progress >= target;
            let relink;
            if finished {
                town.add_i32(WORK, target.wrapping_sub(progress).wrapping_add(budget));
                self.set_s32(i, 0x18, target);
                relink = true;
                let next = self.s16(i, 6);
                self.write_link(town, slot, next);
                let owner = self.s8(i, 0);
                if (owner as u16) < cx.merchant_count && cx.host.merchant_flag(owner) == 0 {
                    let kind = if self.s16(i, 0x134) != 6 { 0x0A } else { 0x07 };
                    if self.s8(i, 0x136) & 3 == 0 {
                        cx.host.notify(owner as u16, town_index, kind, i as u32);
                    }
                }
                self.set_s16(i, 0x134, 5);
                self.set_s16(i, 0x138, 0);
                self.set_s16(i, 6, self.docked_head);
                if self.s8(i, 0x136) == 2 {
                    self.set_s8(i, 0x136, 1);
                }
            } else if self.s8(i, 0x136) == 1 {
                town.add_i32(WORK, budget);
                relink = true;
                let next = self.s16(i, 6);
                self.write_link(town, slot, next);
                self.set_s16(i, 0x134, 5);
                self.set_s16(i, 0x138, 0);
                self.set_s16(i, 6, self.docked_head);
            } else {
                town.add_i32(WORK, budget);
                relink = false;
                slot = Some(i);
            }
            if relink {
                self.docked_head = i;
                cx.host.berth(i as u32);
            }
            head = read(self, town, slot);
        }
        // ---- cola de obras
        let mut yslot: Option<u16> = None;
        let yread = |m: &Self, t: &crate::town::Town, s: Option<u16>| match s {
            None => t.u16(YARD_HEAD),
            Some(p) => m.e16(p, 8),
        };
        let mut cur = yread(self, town, yslot);
        while cur < self.entry_count {
            let e = cur;
            self.set_e16(e, 0x12, 6);
            let mut unfinished = false;
            let mut s = self.e16(e, 0xA);
            while s < self.ship_count {
                let progress = self.s32(s, 0x18).wrapping_add(budget);
                self.set_s8(s, 0x3D, self.s8(s, 0x3D) | 1);
                let next = self.s16(s, 6);
                let target = self.s32(s, 0x14);
                self.set_s32(s, 0x18, progress);
                self.set_s16(s, 0x134, 6);
                if progress >= target {
                    town.add_i32(WORK, target.wrapping_sub(progress).wrapping_add(budget));
                    self.set_s32(s, 0x18, target);
                } else {
                    town.add_i32(WORK, budget);
                    unfinished = true;
                }
                s = next;
            }
            let lead = self.e16(e, 0x10);
            if self.s8(lead, 0x136) == 1 || !unfinished {
                let next = self.e16(e, 8);
                let old = match yslot {
                    None => {
                        let o = town.u16(YARD_HEAD);
                        town.set_u16(YARD_HEAD, next);
                        o
                    }
                    Some(p) => {
                        let o = self.e16(p, 8);
                        self.set_e16(p, 8, next);
                        o
                    }
                };
                self.set_e16(e, 8, self.free_entry_head);
                self.free_entry_head = old;
                self.set_e16(e, 0x12, 0);
                if self.s8(lead, 0x136) == 2 {
                    self.set_s8(lead, 0x136, 1);
                }
                let mut s = self.e16(e, 0xA);
                while s < self.ship_count {
                    let target = self.s32(s, 0x14);
                    let next = self.s16(s, 6);
                    self.set_s8(s, 0x3D, self.s8(s, 0x3D) | 1);
                    self.set_s32(s, 0x18, target);
                    self.set_s16(s, 0x134, 0);
                    s = next;
                }
                cx.host.berth(lead as u32);
                let owner = self.e8(e, 0);
                if (owner as u16) < cx.merchant_count
                    && cx.host.merchant_flag(owner) == 0
                    && self.s8(self.e16(e, 0x10), 0x136) != 1
                {
                    cx.host.notify(owner as u16, town_index, 7, self.e16(e, 0x10) as u32);
                }
            } else {
                yslot = Some(e);
            }
            cur = yread(self, town, yslot);
        }
    }

    fn write_link(&mut self, town: &mut crate::town::Town, slot: Option<u16>, v: u16) {
        match slot {
            None => town.set_u16(town_off::REPAIR_HEAD, v),
            Some(p) => self.set_s16(p, 6, v),
        }
    }
}

/// Comerciante tal como lo toca el astillero.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Merchant {
    /// `+0`: dinero.
    pub gold: i32,
    /// `+0x4C4`: gastos acumulados.
    pub spent: i32,
    /// `+0xE`: primer barco de su lista (enlace `+4` de cada barco).
    pub first_ship: u16,
}

/// Presupuesto de una mejora (`town_quote_ship_upgrade`, 0x0052B670): 17 palabras de las que
/// `start_repair` lee tres cantidades de materiales (`+0xC`, `+0x10`, `+0x14`), tres existencias a
/// descontar (`+0x18`, `+0x1C`, `+0x20`) y el coste (`+0x30`).
pub type Quote = [i32; 17];

/// Lo que `start_repair` pide al resto del juego.
pub trait RepairHost {
    /// La ciudad `index` (`None` si no existe).
    fn town(&mut self, index: u8) -> Option<&mut crate::town::Town>;
    /// `town_quote_ship_upgrade`: `None` si la mejora no es posible.
    fn quote(&mut self, town: u8, ship: u32, request: u32) -> Option<Quote>;
    /// `find_or_create_office(owner, town, 0)`: existencias `+0x30`, `+0x40`, `+0x34` de la oficina.
    fn office_stock(&mut self, owner: u8, town: u8) -> Option<&mut [i32; 3]>;
    /// `get_merchant(owner)`.
    fn merchant(&mut self, owner: u8) -> &mut Merchant;
    /// `ship_upgrade_levels` (0x0051A750).
    fn upgrade_levels(&mut self, ship: u16, request: u32);
    /// Efecto de `ship_upgrade_levels` sobre la imagen del barco (el original lo hace dentro de la llamada); sin él la prueba x86 no cambia nada.
    fn upgrade_image(&mut self, _ship: &mut [u8], _request: u32) {}
    /// `merchant_add_social_reputation_ship_hull_work` (0x004F9120).
    fn hull_work_reputation(&mut self, town: u8, remaining: i32);
}

/// Cociente por multiplicación mágica con signo (`imul`, `sar`, y +1 si es negativo).
fn magic_div(x: i32, magic: i64, shift: u32) -> i32 {
    let q = ((x as i64 * magic) >> 32) as i32 >> shift;
    q.wrapping_add(((q as u32) >> 31) as i32)
}

impl ShipManager {
    /// `start_repair` (0x00508660): reserva materiales y dinero, mueve el barco de atracados a la cola de
    /// reparaciones del astillero y lo pone en estado 7.
    pub fn start_repair(&mut self, ship: u32, request: u32, host: &mut dyn RepairHost) {
        if ship >= self.ship_count as u32 || request == 0 {
            return;
        }
        let i = ship as u16;
        let town_index = self.s8(i, 0x39);
        if host.town(town_index).is_none() || self.s16(i, 0x134) > 1 {
            return;
        }
        let Some(q) = host.quote(town_index, ship, request) else {
            return;
        };
        let mut q = q;
        // sacar de atracados
        let mut slot: Option<u16> = None;
        let mut cur = self.docked_head;
        while cur as u32 != ship {
            if cur >= self.ship_count {
                return;
            }
            slot = Some(cur);
            cur = self.s16(cur, 6);
        }
        let next = self.s16(i, 6);
        match slot {
            None => self.docked_head = next,
            Some(p) => self.set_s16(p, 6, next),
        }
        let owner = self.s8(i, 0);
        {
            let t = host.town(town_index).expect("ciudad del barco");
            let head = t.u16(town_off::REPAIR_HEAD);
            self.set_s16(i, 6, head);
            t.set_u16(town_off::REPAIR_HEAD, i);
        }
        self.set_s8(i, 0x3C, self.s8(i, 0x3C) | 0x20);
        self.set_s16(i, 0x134, 7);
        let cost = q[12];
        let first_ship = {
            let m = host.merchant(owner);
            m.gold = m.gold.wrapping_sub(cost);
            m.spent = m.spent.wrapping_add(cost);
            m.first_ship
        };
        // existencias de la ciudad (nunca negativas)
        {
            let t = host.town(town_index).expect("ciudad del barco");
            for (offset, quote_at, factor) in [(0x30usize, 6usize, 2000i32), (0x40, 7, 200), (0x34, 8, 200)] {
                let v = t
                    .i32(offset)
                    .wrapping_add(q[quote_at].wrapping_mul(factor).wrapping_neg());
                t.set_i32(offset, v.max(0));
            }
        }
        // la oficina cubre primero lo que puede
        if q[3] != 0 || q[4] != 0 || q[5] != 0 {
            if let Some(stock) = host.office_stock(owner, town_index) {
                Self::take_stock(stock, &mut q[3..6]);
            }
        }
        // los demás barcos del comerciante en la misma ciudad
        let mut s = first_ship;
        while s < self.ship_count {
            let state = self.s16(s, 0x134);
            if self.s8(s, 0x39) == town_index && (state < 4 || state == 7) && self.s8(s, 0x136) & 1 == 0 {
                let mut stock = [self.s32(s, 0x80), self.s32(s, 0x90), self.s32(s, 0x84)];
                Self::take_stock(&mut stock, &mut q[3..6]);
                self.set_s32(s, 0x80, stock[0]);
                self.set_s32(s, 0x90, stock[1]);
                self.set_s32(s, 0x84, stock[2]);
            }
            s = self.s16(s, 4);
        }
        host.upgrade_levels(i, request);
        {
            let at = self.ship(i);
            host.upgrade_image(&mut self.ships[at..at + SHIP_SIZE], request);
        }
        let remaining = self.s32(i, 0x14).wrapping_sub(self.s32(i, 0x18));
        host.hull_work_reputation(town_index, remaining);
    }

    /// Descuenta de las existencias `stock` (en unidades de 2000, 200 y 200) lo que haga falta de `need`.
    fn take_stock(stock: &mut [i32], need: &mut [i32]) {
        for (k, (magic, shift, unit)) in [
            (0x10624DD3i64, 7u32, 2000i32),
            (0x51EB851F, 6, 200),
            (0x51EB851F, 6, 200),
        ]
        .into_iter()
        .enumerate()
        {
            let avail = magic_div(stock[k], magic, shift);
            if avail >= need[k] {
                stock[k] = stock[k].wrapping_sub(need[k].wrapping_mul(unit));
                need[k] = 0;
            } else {
                stock[k] = stock[k].wrapping_sub(avail.wrapping_mul(unit));
                need[k] = need[k].wrapping_sub(avail);
            }
        }
    }
}

// ---- presupuesto de mejoras y reparaciones (tablas del ejecutable inglés) ----

/// Cuatro filas de seis palabras en 0x0066E030: trabajo base, tres cantidades de materiales y dos más.
const UPGRADE_TABLE: [[i32; 6]; 4] = [
    [3, 2, 1, 1, 6, 2700],
    [5, 3, 2, 2, 10, 4500],
    [8, 4, 3, 3, 16, 7200],
    [10, 5, 4, 4, 20, 9000],
];
/// Bytes en 0x00672C14: 1 si la mercancía se mide en unidades de 200 (24 mercancías).
pub(crate) const WARE_SMALL_UNIT: [u8; 24] = [0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1];
/// Bytes en 0x00672CF0 (pares indexados por clase, nivel y cantidad).
const PAIR_TABLE: [u8; 64] = [
    0, 0, 4, 0, 4, 2, 0, 0, 4, 0, 2, 4, 0, 4, 0, 6, 2, 8, 0, 6, 2, 8, 0, 12, 0, 0, 5, 0, 29, 0, 0, 0, 5, 0, 61, 0, 240,
    0, 252, 0, 253, 3, 252, 0, 253, 3, 255, 15, 48, 117, 16, 39, 232, 3, 0, 0, 19, 19, 8, 15, 15, 14, 14, 16,
];
/// Mercancías del presupuesto: las tres que se descuentan en `start_repair`.
const QUOTE_WARES: [usize; 3] = [11, 15, 12];
/// Umbrales de `Town+0x818` (0x00672F34, 0x0066FD54, 0x00673848, 0x00671FC8).
const FACTOR_STEPS: [f32; 4] = [8.0, 3.0, 13.0, 20.0];

impl ShipManager {
    /// Factor de coste de reparación según `Town+0x818` (la escalera de comparaciones x87).
    fn repair_factor(t: &crate::town::Town) -> i64 {
        let f = t.f32(0x818);
        let [c1, c2, c3, c4] = FACTOR_STEPS;
        if !(f < c1) {
            if !(f < c3) {
                if !(f > c4) {
                    270
                } else {
                    285
                }
            } else {
                300
            }
        } else if !(f < c2) {
            315
        } else {
            330
        }
    }

    /// `town_calc_ship_repair_cost` (0x0052A9E0).
    pub fn calc_ship_repair_cost(&self, t: &crate::town::Town, ship: u16) -> i32 {
        let target = self.s32(ship, 0x14);
        let diff = target.wrapping_sub(self.s32(ship, 0x18)) as i64;
        let floor = magic_div(target, 0x51EB851F, 5) as i64;
        let work = diff.max(floor);
        let p1 = (self.s32(ship, 0x10) as i64).wrapping_mul(work);
        let p2 = p1.wrapping_mul(Self::repair_factor(t)) << 4;
        let den = target.wrapping_mul(2000) as i64;
        let q = p2.checked_div(den).expect("división por cero del original") >> 4;
        (q as i32).wrapping_add(1)
    }

    /// `town_estimate_ship_repair_days` (0x0052AAD0). `day` es el byte de 0x006DE4B4.
    pub fn estimate_ship_repair_days(&self, t: &crate::town::Town, day: u8, ship: u16, flag: u32) -> i32 {
        let speed = |t: &crate::town::Town| -> i32 {
            let v = t.u16(0x854) as i32;
            if v < 6 {
                600
            } else {
                100 * v
            }
        };
        let days = |s: u16, speed: i32| -> i32 {
            self.s32(s, 0x14)
                .wrapping_sub(self.s32(s, 0x18))
                .wrapping_add(speed)
                .wrapping_sub(1)
                .checked_div(speed)
                .unwrap()
        };
        let early = (day as i32) < t.u8(0x2C0) as i32 * 8 + 7;
        let entry = self.s16(ship, 8);
        if entry >= self.entry_count {
            let d = days(ship, speed(t));
            return if flag == 0 && early { d - 1 } else { d };
        }
        let mut best = 0i32;
        let mut s = self.e16(entry, 0xA);
        while s < self.ship_count {
            let d = days(s, speed(t));
            if d > best {
                best = d;
            }
            s = self.s16(s, 6);
        }
        if flag == 0 && early {
            best - 1
        } else {
            best
        }
    }
}

/// Lo que el presupuesto pide al resto del juego.
pub trait QuoteHost {
    /// `find_or_create_office(owner, town, 0)`: existencias de las mercancías 11, 15 y 12.
    fn office_stock(&mut self, owner: u16, town: u16) -> Option<[i32; 3]>;
    /// `get_merchant(owner)`.
    fn merchant(&mut self, owner: u8) -> Merchant;
    /// `get_buy_price(ware, town, amount)` (0x0052E430).
    fn buy_price(&mut self, ware: u32, town: u32, amount: i32) -> i32;
}

/// Entorno del presupuesto.
pub struct QuoteCtx<'a> {
    /// `u16` en 0x006DE4AA.
    pub merchant_count: u16,
    /// Byte de 0x006DE4B4.
    pub day: u8,
    pub host: &'a mut dyn QuoteHost,
}

impl ShipManager {
    /// `town_quote_ship_upgrade` (0x0052B670): presupuesto de mejorar `count` niveles el barco `ship`.
    /// Devuelve si es posible y la estructura de 0x44 bytes (`None`: el original no la toca).
    pub fn quote_ship_upgrade(
        &self,
        t: &crate::town::Town,
        ship: u32,
        count: u32,
        cx: &mut QuoteCtx,
    ) -> (bool, Option<Quote>) {
        let i = (ship & 0xFFFF) as u16;
        if i >= self.ship_count {
            return (false, None);
        }
        let owner = self.s8(i, 0);
        if (owner as u16) >= cx.merchant_count {
            return (false, None);
        }
        let level = self.s8(i, 0xF) as u32;
        let mut n = count & 0xFFFF;
        if level + n > 2 {
            if level >= 2 {
                return (false, Some([0; 17]));
            }
            n = 2 - level;
        }
        let mut ok = n != 0;
        let tier = (self.s8(i, 0xE) & 3) as usize;
        let town = t.u8(0x2C0);
        let mut have = cx.host.office_stock(owner as u16, town as u16).unwrap_or([0; 3]);
        let first = cx.host.merchant(owner).first_ship;
        let mut s = first;
        while s < self.ship_count {
            if self.s8(s, 0x39) == town && self.s16(s, 0x134) < 4 && self.s8(s, 0x136) & 1 == 0 {
                for k in 0..3 {
                    have[k] = have[k].wrapping_add(self.s32(s, 0x54 + 4 * QUOTE_WARES[k]));
                }
            }
            s = self.s16(s, 4);
        }
        let mut q: Quote = [0; 17];
        for k in 0..3 {
            let ware = QUOTE_WARES[k];
            let unit = if WARE_SMALL_UNIT[ware] != 0 { 200 } else { 2000 };
            let avail = have[k] / unit;
            q[k] = avail;
            q[3 + k] = avail;
            let need = UPGRADE_TABLE[tier][1 + k].wrapping_mul(n as i32);
            if avail >= need {
                q[3 + k] = need;
                q[9 + k] = 0;
                q[6 + k] = 0;
            } else {
                let missing = need - avail;
                q[6 + k] = missing;
                q[9 + k] = cx.host.buy_price(ware as u32, town as u32, missing.wrapping_mul(unit));
                let units = t.stock(ware) / unit;
                if missing > units {
                    ok = false;
                    q[6 + k] = units;
                }
            }
        }
        let work = UPGRADE_TABLE[tier][0].wrapping_mul(n as i32);
        q[15] = work;
        q[14] = magic_div(self.s32(i, 0x10), 0x10624DD3, 7).wrapping_sub(work);
        let speed = (t.u16(0x854) as i32).max(6);
        q[13] = speed.wrapping_add(work.wrapping_mul(28)).wrapping_sub(1) / speed;
        q[12] = work
            .wrapping_mul(405)
            .wrapping_add(q[9])
            .wrapping_add(q[10])
            .wrapping_add(q[11]);
        let at = 2 * (self.s8(i, 0xF) as usize + 3 * self.s8(i, 0xE) as usize + n as usize);
        q[16] = PAIR_TABLE[at + 1] as i32 | (PAIR_TABLE[at] as i32) << 8;
        let entry = self.s16(i, 8);
        let cost = if entry >= self.entry_count {
            self.calc_ship_repair_cost(t, i)
        } else {
            let mut sum = 0i32;
            let mut s = self.e16(entry, 0xA);
            while s < self.ship_count {
                sum = sum.wrapping_add(self.calc_ship_repair_cost(t, s));
                s = self.s16(s, 6);
            }
            sum
        };
        q[12] = q[12].wrapping_add(cost);
        q[13] = q[13].wrapping_add(self.estimate_ship_repair_days(t, cx.day, i, 1));
        let gold = cx.host.merchant(owner).gold;
        (!(gold < q[12]) && ok, Some(q))
    }
}

// ---- botadura de barcos nuevos (cola de obra en Town+0x81C) ----

/// Contadores del comerciante que toca la botadura.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LaunchCounters {
    /// `+0x478`: barcos botados.
    pub launched: u16,
    /// `+8`: distinto de cero si el comerciante no recibe avisos.
    pub flag: u16,
}

/// Lo que la botadura pide al resto del juego.
pub trait LaunchHost: ShipyardHost {
    /// `add_message(owner, letter)` (0x004D6530): la carta tiene `+4` a 0, `+5` la ciudad, `+8` el barco y
    /// `+0xA` la clase del barco.
    fn add_message(&mut self, owner: u8, town: u8, ship: u16, class: u16);
    /// `get_merchant(owner)`.
    fn launch_counters(&mut self, owner: u8) -> &mut LaunchCounters;
    /// `do_notification` con el tipo 6 y el barco.
    fn launch_notify(&mut self, owner: u16, town: u32, ship: u32);
    /// `event_window_open_or_defer(0)` (0x00469380), cuando el barco es del jugador.
    fn event_open(&mut self);
    /// `event_window_show_event` (0x00469AF0) con dos textos (`fill_p3_string` de 0x6DD8A8+2k y +1); `kind`
    /// es 0xA..0xD según la clase del barco.
    fn event_show(&mut self, kind: u8, town: u32, ship: u32);
}

impl ShipManager {
    /// `advance_construction` (0x005083B0): suma `budget` de trabajo al primer barco de la cola de obra de la
    /// ciudad y, si lo termina, lo bota (estado 5) y lo pasa a la lista de atracados. `day` es el dword de
    /// 0x006DE4B4, `player` el de 0x006DFC14 y `merchant_count` el `u16` de 0x006DE4AA.
    pub fn advance_ship_construction(
        &mut self,
        t: &mut crate::town::Town,
        town_index: u32,
        budget: i32,
        day: u32,
        player: u32,
        merchant_count: u16,
        host: &mut dyn LaunchHost,
    ) {
        let head = t.u16(0x81C);
        if head >= self.ship_count {
            return;
        }
        let i = head;
        let progress = self.s32(i, 0x18).wrapping_add(budget);
        self.set_s8(i, 0x3D, self.s8(i, 0x3D) | 1);
        self.set_s32(i, 0x18, progress);
        t.add_i32(town_off::WORK, budget);
        let target = self.s32(i, 0x14);
        if target > progress {
            return;
        }
        self.set_s32(i, 0x4C, day.min(0x7FFF_FFFF) as i32);
        self.set_s8(i, 0x3C, self.s8(i, 0x3C) | 0x20);
        self.set_s32(i, 0x18, target);
        self.set_s16(i, 0x134, 5);
        self.set_s16(i, 0x138, 0);
        let owner = self.s8(i, 0);
        host.add_message(owner, town_index as u8, head, self.s8(i, 0xE) as u16);
        t.set_u16(0x81C, self.s16(i, 6));
        self.set_s16(i, 6, self.docked_head);
        self.docked_head = head;
        if (owner as u16) < merchant_count {
            let counters = host.launch_counters(owner);
            counters.launched = counters.launched.wrapping_add(1);
            if counters.flag == 0 {
                host.launch_notify(owner as u16, town_index, head as u32);
                if player == owner as u32 {
                    host.event_open();
                    let class = self.s8(i, 0xE);
                    if class <= 3 {
                        host.event_show(0xA + class, town_index, head as u32);
                    }
                }
            }
        }
        host.berth(head as u32);
    }
}
