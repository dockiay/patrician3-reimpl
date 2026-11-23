//! Descarga y venta de la carga del barco de IA atracado: `0x00519280`, `0x005193F0` y `0x005196B0`, con la búsqueda de oficina
//! `0x005308A0` (solo la rama de consulta; la que crea la oficina no la usa la IA, que pasa `0` como tercer argumento).
//!
//! Una oficina es un registro de 0x44C bytes de la reserva `[0x006DE514]`: existencias por mercancía en `+4+4·i`, objetivo en
//! `+0x354+4·i`, comerciante en `+0x2C4` (u16) y siguiente de la ciudad en `+0x2CA` (u16). Cada ciudad guarda en `+0x784` la primera.

use super::ai_dock::AiDockHost;
use super::ai_trade::{market_of, TRADE_FLAG};
use super::remove::{MERCHANT_SIZE, SHIP_SIZE, TOWN_SIZE};
use super::tick::TickWorld;
use super::{r16, r32, w32};
use crate::ai::reputation::{Merchant, RepEnv};
use crate::economy::prices::sell_price;
use crate::town::Town;

pub const OFFICE_SIZE: usize = 0x44C;

/// `0x005308A0` con `crear = 0`: desplazamiento de la oficina de `merchant` en `town`, si existe.
pub fn find_office(w: &TickWorld, host: &mut dyn AiDockHost, merchant: u32, town: u32) -> Option<usize> {
    let (merchant, town) = (merchant & 0xFFFF, town & 0xFFFF);
    if town >= w.remove.town_count || merchant >= u32::from(w.remove.merchant_count) {
        return None;
    }
    let pool = host.offices()?;
    let capacity = pool.len() / OFFICE_SIZE;
    let mut at = usize::from(r16(w.remove.towns, town as usize * TOWN_SIZE + 0x784));
    let mut steps = 0;
    while at < capacity {
        let node = at * OFFICE_SIZE;
        if u32::from(r16(pool, node + 0x2C4)) == merchant {
            return Some(node);
        }
        at = usize::from(r16(pool, node + 0x2CA));
        steps += 1;
        if steps > capacity + 1 {
            return None;
        }
    }
    None
}

fn stock(host: &mut dyn AiDockHost, office: usize, i: usize) -> i32 {
    r32(host.offices().unwrap(), office + 4 + 4 * i)
}

fn target(host: &mut dyn AiDockHost, office: usize, i: usize) -> i32 {
    r32(host.offices().unwrap(), office + 0x354 + 4 * i)
}

fn add_stock(host: &mut dyn AiDockHost, office: usize, i: usize, v: i32) {
    let pool = host.offices().unwrap();
    let old = r32(pool, office + 4 + 4 * i);
    w32(pool, office + 4 + 4 * i, old.wrapping_add(v));
}

/// Tamaño de lote al descargar en una oficina: 10 desde la mercancía 0x14; si no, 200 o 2.000 según la mercancía.
fn office_lot(i: usize) -> i32 {
    if i >= 0x14 {
        10
    } else if TRADE_FLAG[i] != 0 {
        200
    } else {
        2000
    }
}

/// Redondeo a lote de venta: 200 o 2.000 por mercancía (`x / lote · lote`, división truncada).
fn sale_lot(i: usize) -> i32 {
    if TRADE_FLAG[i] != 0 {
        200
    } else {
        2000
    }
}

/// Vende `amount` unidades de `ware` en la ciudad actual del barco: cobra `sell_price`, paga al comerciante 6/10 del precio, anota la
/// reputación y suma las existencias de la ciudad. El llamador descuenta la carga.
fn sell_units(w: &mut TickWorld, host: &mut dyn AiDockHost, ship: u16, ware: usize, amount: i32) {
    let o = ship as usize * SHIP_SIZE;
    let here = usize::from(w.remove.ships[o + 0x39]);
    let m = w.remove.ships[o] as usize * MERCHANT_SIZE;
    let price = sell_price(ware, &market_of(w.remove.towns, here, ware), amount, host.difficulty());
    w32(
        w.remove.ships,
        o + 0x124,
        r32(w.remove.ships, o + 0x124).wrapping_add(price),
    );
    let v = price.wrapping_mul(6);
    let mut pay = ((i64::from(v) * 0x6666_6667) >> 32) as i32 >> 1;
    pay = pay.wrapping_add(((pay as u32) >> 31) as i32);
    w32(w.remove.merchants, m, r32(w.remove.merchants, m).wrapping_add(pay));
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
    merchant.on_goods_sold_to_town(&env, here as u32, ware as u32, price, amount, 1);
    w.remove.merchants[m..m + MERCHANT_SIZE].copy_from_slice(&merchant.bytes);
    let slot = here * TOWN_SIZE + 4 + 4 * ware;
    w32(w.remove.towns, slot, r32(w.remove.towns, slot).wrapping_add(amount));
}

fn cargo_at(w: &TickWorld, o: usize, i: usize) -> i32 {
    r32(w.remove.ships, o + 0x54 + 4 * i)
}

fn set_cargo(w: &mut TickWorld, o: usize, i: usize, v: i32) {
    w32(w.remove.ships, o + 0x54 + 4 * i, v);
}

/// `0x00519280`: descarga en la oficina de la meta lo que le falta (por lotes) y vende todo lo demás en la ciudad actual.
pub fn unload_and_sell(w: &mut TickWorld, host: &mut dyn AiDockHost, ship: u16) {
    let o = ship as usize * SHIP_SIZE;
    let (merchant, goal) = (u32::from(w.remove.ships[o]), u32::from(w.remove.ships[o + 0x13A]));
    if let Some(office) = find_office(w, host, merchant, goal) {
        for i in 0..24 {
            let have = cargo_at(w, o, i);
            if have > 0 {
                let mut want = target(host, office, i).wrapping_sub(stock(host, office, i));
                let lot = office_lot(i);
                want = want.wrapping_sub(want.wrapping_rem(lot));
                if want > 0 {
                    let moved = want.min(have);
                    set_cargo(w, o, i, have - moved);
                    add_stock(host, office, i, moved);
                }
            } else {
                set_cargo(w, o, i, 0);
            }
        }
    }
    for i in 0..24 {
        let have = cargo_at(w, o, i);
        if have > 0 {
            sell_units(w, host, ship, i, have);
        }
        set_cargo(w, o, i, 0);
    }
    w.remove.ships[o + 0x3C] |= 4;
}

/// `0x005196B0`: reserva para el siguiente puerto lo que su oficina pide y descarga en la oficina de la ciudad actual lo que
/// pide; el resto, en lotes, se vende aquí.
pub fn sell_surplus(w: &mut TickWorld, host: &mut dyn AiDockHost, ship: u16) {
    let o = ship as usize * SHIP_SIZE;
    let merchant = u32::from(w.remove.ships[o]);
    let mut spare = [0i32; 24];
    for (i, s) in spare.iter_mut().enumerate() {
        *s = cargo_at(w, o, i);
    }
    let next = u32::from(w.remove.ships[o + 0x13B]);
    let reserve = |spare: &mut [i32; 24], host: &mut dyn AiDockHost, office: Option<usize>| {
        if let Some(office) = office {
            for i in 0..24 {
                let want = target(host, office, i).wrapping_sub(stock(host, office, i));
                if want > 0 {
                    spare[i] = spare[i].wrapping_sub(want);
                }
            }
        }
    };
    let office = find_office(w, host, merchant, next);
    reserve(&mut spare, host, office);
    let here = u32::from(w.remove.ships[o + 0x39]);
    if let Some(office) = find_office(w, host, merchant, here) {
        for i in 0..24 {
            let left = spare[i];
            if left > 0 {
                let want = target(host, office, i).wrapping_sub(stock(host, office, i));
                if want > 0 {
                    let mut moved = want.wrapping_div(sale_lot(i)).wrapping_mul(sale_lot(i));
                    if moved > left {
                        moved = left;
                        spare[i] = 0;
                    } else {
                        spare[i] = left - moved;
                    }
                    set_cargo(w, o, i, cargo_at(w, o, i).wrapping_sub(moved));
                    add_stock(host, office, i, moved);
                }
            }
        }
    }
    if w.remove.ships[o + 0x38] != w.remove.ships[o + 0x13B] {
        let office = find_office(w, host, merchant, u32::from(w.remove.ships[o + 0x13B]));
        reserve(&mut spare, host, office);
    }
    for i in 0..24 {
        if spare[i] > 0 {
            let lot = sale_lot(i);
            spare[i] = spare[i].wrapping_div(lot).wrapping_mul(lot);
            let amount = spare[i];
            sell_units(w, host, ship, i, amount);
            set_cargo(w, o, i, cargo_at(w, o, i).wrapping_sub(amount));
        }
    }
    w.remove.ships[o + 0x3C] |= 4;
}

/// `0x005193F0`: el barco ya está en la ciudad de su meta o la tiene como siguiente. Por mercancía calcula cuánto sobra
/// (carga menos lo que la meta no puede absorber, menos lo que piden las oficinas de la meta y del puerto intermedio), descarga
/// en la oficina de la ciudad actual lo que le falta y vende en lotes de 200 lo que quepa hasta el umbral de la ciudad.
pub fn sell_for_goal(w: &mut TickWorld, host: &mut dyn AiDockHost, ship: u16) {
    let o = ship as usize * SHIP_SIZE;
    let merchant = u32::from(w.remove.ships[o]);
    let (goal, here, next) = (
        u32::from(w.remove.ships[o + 0x13A]),
        u32::from(w.remove.ships[o + 0x39]),
        w.remove.ships[o + 0x38],
    );
    let office_goal = find_office(w, host, merchant, goal);
    let office_here = find_office(w, host, merchant, here);
    let office_next = if u32::from(next) != goal {
        find_office(w, host, merchant, u32::from(next))
    } else {
        None
    };
    let (tg, th) = (goal as usize * TOWN_SIZE, here as usize * TOWN_SIZE);
    for i in 0..24usize {
        let cargo = cargo_at(w, o, i);
        let cap_goal = r32(w.remove.towns, tg + 0x4F4 + 16 * i);
        let stock_goal = r32(w.remove.towns, tg + 4 + 4 * i);
        let mut v = cargo;
        if cap_goal > stock_goal {
            v = v.wrapping_add(stock_goal.wrapping_sub(cap_goal));
        }
        if let Some(office) = office_goal {
            let (want, have) = (target(host, office, i), stock(host, office, i));
            if want > have {
                v = v.wrapping_add(have.wrapping_sub(want));
            }
        }
        if let Some(office) = office_here {
            if v > 0 {
                let mut want = target(host, office, i).wrapping_sub(stock(host, office, i));
                let lot = office_lot(i);
                want = want.wrapping_sub(want.wrapping_rem(lot));
                if want > 0 {
                    if want > v {
                        want = v;
                    }
                    set_cargo(w, o, i, cargo.wrapping_sub(want));
                    v = v.wrapping_sub(want);
                    add_stock(host, office, i, want);
                }
            }
        }
        if let Some(office) = office_next {
            let (want, have) = (target(host, office, i), stock(host, office, i));
            if want > have {
                v = v.wrapping_add(have.wrapping_sub(want));
            }
        }
        let room = r32(w.remove.towns, th + 0x4F4 + 16 * i).wrapping_sub(r32(w.remove.towns, th + 4 + 4 * i));
        if room <= 0 {
            continue;
        }
        if room < v {
            v = room;
        }
        if v <= 0 {
            continue;
        }
        let amount = v.wrapping_div(200).wrapping_mul(200);
        sell_units(w, host, ship, i, amount);
        set_cargo(w, o, i, cargo_at(w, o, i).wrapping_sub(amount));
    }
    w.remove.ships[o + 0x3C] |= 4;
}
