//! `ship::ai_docked_update` (0x00518B80): decisión de un barco de IA atracado, con `reachable_ports` (0x00518D60) y
//! `rest_or_crew` (0x00518E00). Los ayudantes de comercio y tripulación (`0x519280`, `0x51A070`, `0x519FC0`, `0x519920`,
//! `0x5182B0`, `0x518FF0`, `0x519170`, `0x5193F0`, `0x5196B0`, `0x517770`) son ganchos del anfitrión.

use super::ai_arm::arm_and_crew;
use super::ai_cargo::{sell_for_goal, sell_surplus, unload_and_sell};
use super::ai_trade::{market_of, trade_amount, TRADE_FLAG, TRADE_WEIGHT};
use super::cargo::recompute_load;
use super::convoy_drop::dock_ok;
use super::convoy_plan::CREW_MIN;
use super::remove::{CONVOY_SIZE, MERCHANT_SIZE, SHIP_SIZE, TOWN_SIZE};
use super::tick::{ConvoyError, ConvoyHost, TickWorld};
use super::{r16, r32, w16, w32};
use crate::ai::reputation::{Merchant, RepEnv};
use crate::economy::prices::buy_price;
use crate::town::Town;

/// Ganchos de los ayudantes. Todo `ConvoyHost` los admite; el valor devuelto de `0x5182B0` es la capacidad libre.
pub trait AiDockHost {
    fn dock_hook(&mut self, name: &'static str, args: &[u32]) -> u32;
    /// `this+0x64` del objeto de comercio 0x6DE3D8 (factor de venta con existencias nulas), como `f32`.
    fn difficulty(&mut self) -> f32 {
        f32::from_bits(self.dock_hook("trade_difficulty", &[]))
    }
    /// Reserva de oficinas (`[0x006DE514]`, registros de 0x44C bytes). Sin ella no hay oficinas y la búsqueda `0x005308A0` no halla ninguna.
    fn offices(&mut self) -> Option<&mut [u8]> {
        None
    }
    /// `0x00531B70`: byte de la matriz de distancias del mundo (0xFF si falta o está fuera de rango).
    fn distance(&mut self, from: u32, to: u32) -> u8 {
        (self.dock_hook("cv_map", &[from, to]) & 0xFF) as u8
    }
}

impl<T: ConvoyHost + ?Sized> AiDockHost for T {
    fn dock_hook(&mut self, name: &'static str, args: &[u32]) -> u32 {
        self.hook(name, args)
    }
}

/// Datos del mundo que `ai_docked_update` necesita y que no viven en `TickWorld`: la matriz de distancias entre ciudades (0x531B70),
/// el factor de dificultad (`this+0x64` del objeto 0x6DE3D8) y la reserva de oficinas (`[0x6DE514]`).
#[derive(Clone, Debug, Default)]
pub struct AiDockData {
    /// Matriz `size × size` de bytes; vacía si el mundo no la tiene (toda distancia vale 0xFF).
    pub distances: Vec<u8>,
    pub size: u32,
    pub difficulty: f32,
    /// Dificultad pirata de la configuración original (+0x13).
    pub pirate_difficulty: u8,
    /// Tablas del planificador de rutas piratas, cuando están reconstruidas.
    pub convoy_plan: Option<super::convoy_plan::PlanEnv>,
    /// Registros de oficina de 0x44C bytes.
    pub offices: Vec<u8>,
}

impl AiDockHost for AiDockData {
    fn dock_hook(&mut self, _name: &'static str, _args: &[u32]) -> u32 {
        0
    }
    fn distance(&mut self, from: u32, to: u32) -> u8 {
        if self.distances.is_empty() || from >= self.size || to >= self.size {
            return 0xFF;
        }
        self.distances
            .get((from * self.size + to) as usize)
            .copied()
            .unwrap_or(0xFF)
    }
    fn difficulty(&mut self) -> f32 {
        self.difficulty
    }
    fn offices(&mut self) -> Option<&mut [u8]> {
        if self.offices.is_empty() {
            None
        } else {
            Some(&mut self.offices)
        }
    }
}

/// `0x00518D60`: máscara de ciudades (bit = índice) con el bit `0x100` de `+0x76C` y sin las banderas de `+0x2C8`
/// (`0x4000E00`, más `0x4000E01` si el casco tiene el bit 2 de `+0xE`), sin la ciudad actual del barco.
pub fn reachable_ports(w: &TickWorld, ship: u16) -> u32 {
    let o = ship as usize * SHIP_SIZE;
    let blocked: u32 = if w.remove.ships[o + 0xE] & 2 == 0 {
        0x0400_0E00
    } else {
        0x0400_0E01
    };
    let mut mask = 0u32;
    let mut bit = 1u32;
    for t in 0..w.remove.town_count as usize {
        let base = t * TOWN_SIZE;
        if r32(w.remove.towns, base + 0x2C8) as u32 & blocked == 0 && r32(w.remove.towns, base + 0x76C) & 0x100 != 0 {
            mask |= bit;
        }
        bit = bit.wrapping_add(bit);
    }
    let here = w.remove.ships[o + 0x39] & 31;
    mask & !(1u32 << here)
}

/// `0x00517770`: marca el barco (`+0x3C`, palabra) y, si va en un convoy válido, su bandera `+0x18`.
pub fn mark_changed(w: &mut TickWorld, ship: u16, bits: u16) {
    let o = ship as usize * SHIP_SIZE;
    let v = r16(w.remove.ships, o + 0x3C) | bits;
    w16(w.remove.ships, o + 0x3C, v);
    let convoy = r16(w.remove.ships, o + 8);
    if convoy < r16(w.remove.manager, 0xF6) {
        let c = convoy as usize * CONVOY_SIZE + 0x18;
        let f = r32(w.remove.convoys, c) | i32::from(bits);
        w32(w.remove.convoys, c, f);
    }
}

/// Tripulación por casco y capacidad: primer dato de `0x0066E030 + 0x18·clase` (marineros por nivel).
const LEVEL_CREW: [i32; 4] = [3, 5, 8, 10];
/// `0x00673664`.
const SECOND_CREW: [u8; 4] = [0x0A, 0x10, 0x1E, 0x18];

fn div_trunc(a: i32, mul: i64, shift: u32) -> i32 {
    let q = ((a as i64 * mul) >> 32) as i32 >> shift;
    q.wrapping_add(((q as u32) >> 31) as i32)
}

/// `0x005184F0`: marineros que faltan al barco según su carga libre y su nivel (recalcula la carga con `0x5182B0`).
pub fn needed_crew(w: &mut TickWorld, ship: u16) -> i32 {
    let o = ship as usize * SHIP_SIZE;
    needed_crew_image(&mut w.remove.ships[o..o + SHIP_SIZE])
}

/// Cuerpo de `0x005184F0` sobre la imagen de 0x180 bytes del barco.
pub fn needed_crew_image(ship: &mut [u8]) -> i32 {
    let o = 0usize;
    let free = recompute_load(&mut ship[o..o + SHIP_SIZE]);
    let class = (ship[o + 0xE] & 3) as usize;
    let by_level = i32::from(ship[o + 0xF]).wrapping_mul(LEVEL_CREW[class]);
    let crew = i32::from(r16(ship, o + 0x40));
    let cap = r32(ship, o + 0x10);
    let q = ((cap as i64 * 0x1062_4DD3) >> 32) as i32 >> 7;
    let mut target = q.wrapping_add(by_level).wrapping_add(((q as u32) >> 31) as i32);
    if target < 0x14 {
        target = 0x14;
    }
    let missing = target.wrapping_sub(crew);
    if free < 0 {
        return 0;
    }
    let mut extra = div_trunc(free, 0x51EB_851F, 7);
    let second = i32::from(SECOND_CREW[class]);
    if r16(ship, o + 0x40) < second as u16 {
        extra = extra.wrapping_add(second - crew);
    }
    if extra <= missing {
        extra
    } else {
        missing
    }
}

/// `0x00519FC0`: completa la tripulación del barco con la reserva de la ciudad de destino y la lleva al mínimo de su casco.
pub fn fill_crew(w: &mut TickWorld, ship: u16) {
    let o = ship as usize * SHIP_SIZE;
    if u32::from(w.remove.ships[o + 0x38]) > w.remove.town_count {
        return;
    }
    let min = CREW_MIN[(w.remove.ships[o + 0xE] & 3) as usize] as u16;
    if w.remove.ships[o + 0x136] & 0x10 == 0 {
        w.remove.ships[o + 0x3C] |= 2;
        w16(w.remove.ships, o + 0x40, min);
        return;
    }
    let mut need = needed_crew(w, ship);
    if need != 0 {
        let town = w.remove.ships[o + 0x38] as usize * TOWN_SIZE;
        let pool = r32(w.remove.towns, town + 0x2E4);
        if need >= pool {
            need = pool.wrapping_sub(1);
        }
        w32(w.remove.towns, town + 0x2E4, pool.wrapping_sub(need));
        let a = r32(w.remove.towns, town + 0x2D4).wrapping_sub(need);
        w32(w.remove.towns, town + 0x2D4, a);
        let crew = r16(w.remove.ships, o + 0x40).wrapping_add(need as u16);
        w16(w.remove.ships, o + 0x40, crew);
    }
    if r16(w.remove.ships, o + 0x40) < min {
        w16(w.remove.ships, o + 0x40, min);
    }
    w.remove.ships[o + 0x3C] |= 2;
}

/// `0x0067366C`: divisor de la velocidad por byte de casco (`ship+0xE`).
const TRIP_DIV: [u16; 256] = [
    635, 693, 520, 578, 0, 0, 0, 0, 0, 49248, 0, 19200, 2320, 2313, 2320, 2313, 2313, 2825, 2315, 2059, 2312, 2057, 0,
    0, 0, 19200, 62713, 236, 61171, 229, 59372, 224, 259, 258, 513, 257, 65535, 65535, 65534, 65535, 59392, 18301, 0,
    16752, 32768, 17723, 0, 17302, 0, 19200, 0, 19200, 0, 0, 256, 513, 771, 1028, 1541, 1798, 2056, 2313, 2826, 3083,
    3340, 3597, 3855, 4112, 4369, 4626, 4883, 5140, 5397, 5654, 5911, 6168, 6425, 6681, 6938, 6939, 7196, 7453, 7709,
    7710, 7967, 8223, 32, 0, 34576, 17, 19, 0, 11, 0, 12, 0, 17, 0, 772, 2572, 256, 2827, 512, 3090, 768, 3328, 1024,
    3591, 1, 3860, 513, 4109, 769, 4356, 1025, 4631, 2, 4876, 258, 5150, 770, 5377, 1026, 5634, 3, 5907, 259, 6156,
    515, 6429, 1027, 6681, 4, 6917, 260, 7195, 516, 7455, 37044, 64728, 16464, 28768, 12348, 21576, 350, 280, 420, 490,
    250, 200, 300, 350, 14406, 25172, 34944, 15624, 0, 16880, 0, 17402, 0, 19200, 0, 19200, 16699, 20550, 51320, 0, 0,
    0, 256, 513, 771, 1028, 1541, 1798, 2056, 2313, 2826, 3083, 3340, 3597, 3855, 4112, 4369, 4626, 4883, 5140, 5397,
    5654, 5911, 6168, 6425, 6681, 6938, 6939, 7196, 7453, 7709, 7710, 7967, 8223, 32, 0, 0, 19200, 450, 450, 420, 390,
    360, 330, 300, 0, 0, 100, 300, 1050, 0, 100, 600, 900, 0, 200, 400, 800, 300, 500, 600, 1200, 4879, 6423, 7964,
    8994, 12333, 14132, 15159, 17985, 0, 16720, 0, 0, 0, 19200, 0, 0, 248, 22, 236, 29, 217, 40, 216, 51, 217, 31,
];

/// `0x00516AB0`: tiempo de viaje del barco desde su ciudad actual a `dest`, `(v + 255) >> 8` y mínimo 1. El divisor sale de una tabla indexada por el byte
/// completo de casco: si es 0, el original provoca una división por cero.
pub fn trip_time(w: &TickWorld, host: &mut dyn AiDockHost, ship: u16, dest: u32) -> Result<i32, ConvoyError> {
    let o = ship as usize * SHIP_SIZE;
    let kind = w.remove.ships[o + 0xE];
    let d = i32::from(host.distance(u32::from(w.remove.ships[o + 0x39]), dest));
    let mut v = d << 4;
    if kind & 3 != 0 {
        let div = i32::from(TRIP_DIV[kind as usize]);
        if div == 0 {
            return Err(ConvoyError::Divide);
        }
        v = (v << 7).wrapping_sub(v).wrapping_mul(5) / div;
    }
    Ok(if v > 0 { v.wrapping_add(0xFF) >> 8 } else { 1 })
}

/// `0x00519170`: entre las ciudades de `mask`, elige como destino (`+0x38`) la de mayor reserva `(+0x2D4 + ship+0x18) & 0xFF` cuyo
/// rodeo `d(aquí, b) + d(b, siguiente)` no pase de `d(aquí, siguiente)` más una quinta parte.
pub fn choose_destination(w: &mut TickWorld, host: &mut dyn AiDockHost, ship: u16, mask: u32) {
    let o = ship as usize * SHIP_SIZE;
    let (here, next) = (w.remove.ships[o + 0x39], w.remove.ships[o + 0x13B]);
    if here == next {
        return;
    }
    let direct = u32::from(host.distance(u32::from(here), u32::from(next)));
    let limit = direct + direct / 5;
    let mut best = 0i32;
    let mut bit = 1u32;
    for b in 0..w.remove.town_count {
        if bit & mask != 0 {
            let leg1 = u32::from(host.distance(u32::from(here), b));
            let leg2 = u32::from(host.distance(b, u32::from(next)));
            if leg1 + leg2 < limit {
                let score = (r32(w.remove.towns, b as usize * TOWN_SIZE + 0x2D4)
                    .wrapping_add(r32(w.remove.ships, o + 0x18))
                    & 0xFF) as i32;
                if best <= score {
                    best = score;
                    w.remove.ships[o + 0x38] = b as u8;
                }
            }
        }
        bit = bit.wrapping_add(bit);
    }
}

/// `0x00518FF0`: entre las ciudades de `mask` (salvo la meta `+0x13A`) elige como siguiente destino (`+0x13B` y `+0x38`) la de mayor
/// puntuación: suma, por mercancía con trato válido, `peso · factor de precio`. Sin candidata, vuelve a la ciudad actual (`+0x39`).
pub fn best_trade_town(w: &mut TickWorld, host: &mut dyn AiDockHost, ship: u16, mask: u32) -> Result<(), ConvoyError> {
    let o = ship as usize * SHIP_SIZE;
    let goal = w.remove.ships[o + 0x13A];
    let count = w.remove.town_count;
    let mut best = 0f32;
    let mut best_town = 0xFFFFu32;
    let mut bit = 1u32;
    for t in 0..count {
        if t != u32::from(goal) && bit & mask == bit {
            let trip = trip_time(w, host, ship, t)?;
            let base = host.difficulty();
            let mut score = 0f32;
            for c in 0..24usize {
                let threshold = if TRADE_FLAG[c] != 0 { 1000 } else { 6000 };
                let (units, price) = {
                    let direct = trade_amount(w.remove.towns, base, usize::from(goal), t as usize, c, trip)
                        .map_err(|_| ConvoyError::Divide)?;
                    if direct.units > 0 {
                        (direct.units, direct.price)
                    } else {
                        let back = trade_amount(w.remove.towns, base, t as usize, usize::from(goal), c, trip)
                            .map_err(|_| ConvoyError::Divide)?;
                        (back.units, back.price)
                    }
                };
                if units < threshold {
                    continue;
                }
                let term = f32::from(TRADE_WEIGHT[c]) * price;
                score += term;
            }
            if score > best {
                best = score;
                best_town = t;
            }
        }
        bit = bit.wrapping_add(bit);
    }
    let pick = if best_town < count {
        best_town as u8
    } else {
        w.remove.ships[o + 0x39]
    };
    w.remove.ships[o + 0x13B] = pick;
    w.remove.ships[o + 0x38] = pick;
    Ok(())
}

/// `ship_ai_trade_for_next_destination` (0x00519B50): guarda el destino (`+0x38`) en `+0x13B`; si es la meta
/// (`+0x13A`) descarga hacia casa (`0x005193F0`) y si no hacia el socio (`0x005196B0`); después compra la carga para
/// ese destino (`0x00519920`).
pub fn trade_for_next_destination(w: &mut TickWorld, host: &mut dyn AiDockHost, ship: u16) -> Result<(), ConvoyError> {
    let o = ship as usize * SHIP_SIZE;
    let dest = w.remove.ships[o + 0x38];
    w.remove.ships[o + 0x13B] = dest;
    if dest == w.remove.ships[o + 0x13A] {
        sell_for_goal(w, host, ship);
    } else {
        sell_surplus(w, host, ship);
    }
    buy_cargo_for(w, host, ship, u32::from(w.remove.ships[o + 0x13B]))
}

/// `0x00519920`: el barco de IA compra en su ciudad la carga que mejor se vende en `dest`: mientras quede capacidad libre, elige la
/// mercancía de mayor `peso · factor de precio`, la limita a lo que cabe y a las existencias, la redondea a lotes de 200 o 2.000,
/// la carga, paga (el comerciante recibe −5/6 del precio) y anota la reputación. Termina con la bandera `+0x3C |= 4`.
pub fn buy_cargo_for(w: &mut TickWorld, host: &mut dyn AiDockHost, ship: u16, dest: u32) -> Result<(), ConvoyError> {
    let o = ship as usize * SHIP_SIZE;
    let dest = dest & 0xFFFF;
    let here = usize::from(w.remove.ships[o + 0x39]);
    let m = w.remove.ships[o] as usize * MERCHANT_SIZE;
    let trip = trip_time(w, host, ship, dest)?;
    let base = host.difficulty();
    let mut units = [0i32; 24];
    let mut score = [0f32; 24];
    for c in 0..24usize {
        let r = trade_amount(w.remove.towns, base, here, dest as usize, c, trip).map_err(|_| ConvoyError::Divide)?;
        if r.units > 0 {
            let threshold = if TRADE_FLAG[c] != 0 { 1000 } else { 6000 };
            if r.units < threshold {
                units[c] = 0;
                score[c] = 0.0;
            } else {
                units[c] = r.units;
                score[c] = f32::from(TRADE_WEIGHT[c]) * r.price;
            }
        }
    }
    let mut free = recompute_load(&mut w.remove.ships[o..o + SHIP_SIZE]);
    while free > 0 {
        let mut best = 0usize;
        for c in 1..24 {
            if score[c] > score[best] {
                best = c;
            }
        }
        let c = best;
        if units[c] <= 0 {
            break;
        }
        if units[c] > free {
            units[c] = free;
        }
        let stock = r32(w.remove.towns, here * TOWN_SIZE + 4 + 4 * c);
        if stock < units[c] {
            units[c] = stock;
        }
        let lot = if TRADE_FLAG[c] != 0 { 200 } else { 2000 };
        units[c] = units[c] / lot * lot;
        if units[c] <= 0 {
            score[c] = 0.0;
            continue;
        }
        let a = units[c];
        let cargo = o + 0x54 + 4 * c;
        w32(w.remove.ships, cargo, r32(w.remove.ships, cargo).wrapping_add(a));
        free = free.wrapping_sub(a);
        let slot = here * TOWN_SIZE + 4 + 4 * c;
        w32(w.remove.towns, slot, r32(w.remove.towns, slot).wrapping_sub(a));
        let cost = buy_price(c, &market_of(w.remove.towns, here, c), a);
        let v = cost.wrapping_neg().wrapping_shl(2).wrapping_sub(cost);
        let mut pay = ((i64::from(v) * 0x2AAA_AAAB) >> 32) as i32;
        pay = pay.wrapping_add(((pay as u32) >> 31) as i32);
        let cash = r32(w.remove.merchants, m).wrapping_add(pay);
        w32(w.remove.merchants, m, cash);
        w32(
            w.remove.ships,
            o + 0x124,
            r32(w.remove.ships, o + 0x124).wrapping_sub(cost),
        );
        let mut towns = vec![Town::default(); here + 1];
        towns[here]
            .0
            .copy_from_slice(&w.remove.towns[here * TOWN_SIZE..(here + 1) * TOWN_SIZE]);
        let env = RepEnv {
            town_count: w.remove.town_count,
            towns: &towns,
            renown: [0; 3],
            byte_52c: 0,
        };
        let mut merchant = Merchant {
            bytes: w.remove.merchants[m..m + MERCHANT_SIZE].to_vec(),
        };
        merchant.on_goods_bought_from_town(&env, here as u32, c as u32, cost, a, 1);
        w.remove.merchants[m..m + MERCHANT_SIZE].copy_from_slice(&merchant.bytes);
        units[c] = 0;
        score[c] = 0.0;
    }
    w.remove.ships[o + 0x3C] |= 4;
    Ok(())
}

/// `0x00518E00`: el barco no puede zarpar; descansa (estado 4 o 1) o completa tripulación con el puesto de la ciudad.
fn rest_or_crew(w: &mut TickWorld, ship: u16) {
    let o = ship as usize * SHIP_SIZE;
    if (r32(w.remove.ships, o + 0x14) as i32 >> 2) > r32(w.remove.ships, o + 0x18) as i32 {
        w16(w.remove.ships, o + 0x134, 4);
        w16(w.remove.ships, o + 0x138, 0);
        return;
    }
    if (w.remove.ships[o + 0x3F] as i8) < 1 {
        w16(w.remove.ships, o + 0x134, 1);
        let m = w.remove.ships[o] as usize * MERCHANT_SIZE;
        let v = w.remove.merchants[m + 0x547] as u16 * 3;
        w16(w.remove.ships, o + 0x138, v);
        return;
    }
    let crew_min = CREW_MIN[(w.remove.ships[o + 0xE] & 3) as usize] as u16;
    let crew = r16(w.remove.ships, o + 0x40);
    let town = w.remove.ships[o + 0x39] as usize * TOWN_SIZE;
    if crew < crew_min {
        let need = (crew_min - crew) as i32;
        let pool = r32(w.remove.towns, town + 0x2E4) as i32;
        let take = if pool > need { need } else { pool.wrapping_sub(1) };
        let a = (r32(w.remove.towns, town + 0x2D4) as i32).wrapping_sub(take);
        w32(w.remove.towns, town + 0x2D4, a);
        let b = (r32(w.remove.towns, town + 0x2E4) as i32).wrapping_sub(take);
        w32(w.remove.towns, town + 0x2E4, b);
        w16(w.remove.ships, o + 0x40, crew.wrapping_add(take as u16));
        mark_changed(w, ship, 2);
        let convoy = r16(w.remove.ships, o + 8);
        if convoy < r16(w.remove.manager, 0xF6) {
            let c = convoy as usize * CONVOY_SIZE + 0x18;
            let f = r32(w.remove.convoys, c) | 2;
            w32(w.remove.convoys, c, f);
        }
        return;
    }
    if w.remove.ships[o + 0x136] & 0x10 == 0x10 || crew <= crew_min {
        return;
    }
    let extra = crew - crew_min;
    w16(w.remove.ships, o + 0x40, crew - extra);
    mark_changed(w, ship, 2);
    let a = r32(w.remove.towns, town + 0x2D4).wrapping_add(i32::from(extra));
    w32(w.remove.towns, town + 0x2D4, a);
    let b = r32(w.remove.towns, town + 0x2E4).wrapping_add(i32::from(extra));
    w32(w.remove.towns, town + 0x2E4, b);
}

pub fn ai_docked_update(w: &mut TickWorld, ship: u16, host: &mut dyn AiDockHost) {
    let o = ship as usize * SHIP_SIZE;
    let m = w.remove.ships[o] as usize * MERCHANT_SIZE;
    let flags = w.remove.ships[o + 0x136];
    if flags & 0x20 != 0 {
        w16(w.remove.ships, o + 0x134, 1);
        w16(w.remove.ships, o + 0x138, 0x80);
        w16(w.remove.ships, o + 0x40, 0);
        return;
    }
    if flags & 0x10 != 0 {
        unload_and_sell(w, host, ship);
        arm_and_crew(w, ship, u32::from(w.remove.ships[o + 0x39]));
    }
    fill_crew(w, ship);
    let here = w.remove.ships[o + 0x39];
    if w.remove.ships[o + 0x37] == here && w.remove.ships[o + 0x38] != here {
        if dock_ok(w, ship) {
            let next = w.remove.ships[o + 0x13B];
            if buy_cargo_for(w, host, ship, u32::from(next)).is_err() {
                return;
            }
            let free = recompute_load(&mut w.remove.ships[o..o + SHIP_SIZE]);
            if free <= (r32(w.remove.ships, o + 0x10) as i32 >> 2) {
                w16(w.remove.ships, o + 0x134, 2);
                w.remove.ships[o + 0x38] = w.remove.ships[o + 0x13B];
            } else {
                if buy_cargo_for(w, host, ship, u32::from(w.remove.ships[o + 0x38])).is_err() {
                    return;
                }
                w16(w.remove.ships, o + 0x134, 2);
            }
        } else {
            rest_or_crew(w, ship);
        }
        return;
    }
    let mask = reachable_ports(w, ship);
    let (here, next, goal) = (
        w.remove.ships[o + 0x39],
        w.remove.ships[o + 0x13B],
        w.remove.ships[o + 0x13A],
    );
    let mut route_changed = false;
    let mut skip_store = false;
    if here == next {
        route_changed = true;
        if here == goal {
            if best_trade_town(w, host, ship, mask).is_err() {
                return;
            }
            skip_store = true;
        } else {
            w.remove.ships[o + 0x13B] = goal;
        }
        if !skip_store {
            w.remove.ships[o + 0x38] = goal;
        }
    } else {
        w.remove.ships[o + 0x38] = next;
    }
    if w.remove.ships[o + 0x39] != w.remove.ships[o + 0x13A] {
        choose_destination(w, host, ship, mask);
    }
    if route_changed {
        unload_and_sell(w, host, ship);
    } else if w.remove.ships[o + 0x13B] == w.remove.ships[o + 0x13A] {
        sell_for_goal(w, host, ship);
    } else {
        sell_surplus(w, host, ship);
    }
    let here = w.remove.ships[o + 0x39];
    let prev = w.remove.ships[o + 0x37];
    let merchant_547 = i32::from(w.remove.merchants[m + 0x547]);
    if prev != here {
        w.remove.ships[o + 0x37] = here;
        let limit = (0x200 - merchant_547).wrapping_mul(r32(w.remove.ships, o + 0x14) as i32) >> 9;
        if limit > r32(w.remove.ships, o + 0x18) as i32 {
            w16(w.remove.ships, o + 0x134, 4);
            w16(w.remove.ships, o + 0x138, 0);
            w.remove.ships[o + 0x38] = here;
            w.remove.ships[o + 0x37] = here;
        }
    }
    if r16(w.remove.ships, o + 0x134) != 4 {
        w16(w.remove.ships, o + 0x134, 1);
        w16(w.remove.ships, o + 0x138, (merchant_547 + 0x40) as u16);
    }
}

/// `Ship::redirect_to_nearest_port` (0x0051A2C0): elige el puerto más cercano a `except` (distancia del mapa 0x531B70,
/// primero ganador) que no esté bloqueado para este barco y devuelve los argumentos con que el original llama a
/// `ship::plan_route` (0x005172A0): `(destino, a)`. Las ciudades con las banderas `0x04000E00` (más `1` para barcos de clase
/// < 2 o de convoyes de clase >= 2) y la propia `except` quedan fuera; sin candidata devuelve el puerto 0.
pub fn nearest_port_plan(w: &TickWorld, host: &mut dyn AiDockHost, ship: u16, a: u32, except: u32) -> (u32, u32) {
    let o = ship as usize * SHIP_SIZE;
    let except = except & 0xFF;
    let mut mask = 0x0400_0E00u32;
    if w.remove.ships[o + 0xE] < 2 {
        mask |= 1;
    } else {
        let cv = r16(w.remove.ships, o + 8);
        if cv < r16(w.remove.manager, 0xF6) {
            let b = cv as usize * super::remove::CONVOY_SIZE;
            if w.remove.convoys.get(b + 1).is_some_and(|&v| v >= 2) {
                mask |= 1;
            }
        }
    }
    let (mut best, mut pick) = (0x0098_9680u32, 0u32);
    for t in 0..w.remove.town_count {
        if t == except || r32(w.remove.towns, t as usize * TOWN_SIZE + 0x2C8) as u32 & mask != 0 {
            continue;
        }
        let d = u32::from(host.distance(t, except));
        if d < best {
            best = d;
            pick = t;
        }
    }
    (pick, a)
}
