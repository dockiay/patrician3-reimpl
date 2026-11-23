//! Tick diario de una instalación de la ciudad (`Facility::daily_tick`, 0x005101D0): produce o
//! presta su servicio, calcula la plantilla que quiere y contrata o despide hasta alcanzarla. Lo que
//! no logra contratar queda en la lista de huecos de mano de obra del mundo.

use super::employment::{builders_daily, dismiss_workers, hire_workers, record_labour_shortage, shipyard_consume};
use super::employment::{militia_consume, target_employees, MAX_EMPLOYEES};
use super::militia::militia_daily_tick;
use super::production as p;
use super::world::{EconomyWorld, TownHost};
use crate::numeric::ftol_i32;
use crate::town::{off, Town};

/// Mercancía de cada productor (tipos 4–20); los tipos 0–3 no usan este envoltorio.
const PRODUCER_WARE: [usize; 21] = [0, 0, 0, 0, 9, 2, 3, 12, 5, 0, 1, 11, 8, 4, 16, 14, 7, 18, 19, 15, 17];

/// Llama a la función `produce_*` del tipo con los argumentos que le pasa el original.
fn produce(t: &mut Town, f: usize, ty: usize, tick: u32) {
    let cap = MAX_EMPLOYEES[ty] as u32;
    let idle = t.idle(f) as u32;
    match ty {
        4 => p::hunting_lodge(t, f, tick, cap as i32),
        5 => {
            let cap = if t.u32(off::FIELD_2CC) != 0 { 0x48 } else { 0x41 };
            p::fishermans_hut(t, f, tick, cap, true)
        }
        6 => p::brewery(t, f, tick, idle, cap),
        7 => p::workshop(t, f, tick, cap as i32),
        8 => p::apiary(t, f, tick, cap as i32),
        9 => p::farm_grain(t, f, tick, cap as i32),
        10 => p::farm_cattle(t, f, tick, cap as i32),
        11 => p::sawmill(t, f, tick, cap as i32),
        12 => p::weaving_mill(t, f, tick, cap as i32),
        13 => p::saltworks(t, f, tick, idle, cap),
        14 => p::iron_smelter(t, f, tick, cap as i32),
        15 => p::farm_sheep(t, f, tick, cap as i32),
        16 => p::vineyard(t, f, tick, cap),
        17 => p::pottery(t, f, tick, cap),
        18 => p::brickworks(t, f, tick, cap as i32),
        19 => p::pitchmaker(t, f, tick, idle, cap),
        20 => p::farm_hemp(t, f, tick, cap as i32),
        _ => unreachable!(),
    }
}

/// `Facility::daily_tick` (0x005101D0) de la instalación `f` de la ciudad `town`.
///
/// Sin empleados solo trabajan la milicia (tipo 0) y el astillero con la bandera `0x80` de
/// `more_flags`. Si la mercancía ya estaba por encima del umbral 3, lo producido no se guarda (la
/// existencia vuelve a la de antes, aunque los insumos sí se gastan).
pub fn daily_tick(world: &mut EconomyWorld, town: usize, f: usize, host: &mut dyn TownHost) {
    let tick = world.tick;
    let t = &world.towns[town];
    let ty = t.facility_type(f) as usize;
    if t.employees(f) == 0 && (ty >= 2 || (ty == 1 && t.more_flags() & 0x80 == 0)) {
        return;
    }
    let target: u32 = match ty {
        0 => {
            militia_daily_tick(world, town, host);
            let t = &mut world.towns[town];
            let staff = t.employees(f) as u32;
            militia_consume(t, f);
            staff
        }
        1 => {
            let t = &mut world.towns[town];
            shipyard_consume(t, f, host);
            let wanted = ftol_i32(t.f32(off::SHIPYARD_SOMETHING) as f64)
                .wrapping_mul(3)
                .wrapping_add(10) as u32;
            wanted.min(t.employees(f) as u32 + 3).min(0x28)
        }
        2 => (builders_daily(&mut world.towns[town], f, host) as u32).max(5),
        3 => {
            let t = &mut world.towns[town];
            let product = p::weaponsmith_choose_product(t, f);
            p::weaponsmith(t, f, tick, product);
            MAX_EMPLOYEES[3] as u32
        }
        4..=20 => {
            let t = &mut world.towns[town];
            let w = PRODUCER_WARE[ty];
            let before = t.stock(w);
            let last = off::STOCK_AT_LAST_TICK + 4 * w;
            t.add_i32(last, before.wrapping_neg());
            produce(t, f, ty, tick);
            if before > t.threshold(w, 3) {
                t.set_stock(w, before);
            }
            t.add_i32(last, t.stock(w));
            target_employees(t, f)
        }
        _ => 0,
    };
    let t = &mut world.towns[town];
    let employees = t.employees(f) as u32;
    if employees > target {
        dismiss_workers(t, f, employees - target);
        return;
    }
    if employees == target {
        return;
    }
    let mut need = target - employees;
    let idle = t.idle(f) as u32;
    if idle > 0 {
        if need < idle {
            t.set_idle(f, (idle - need) as u16);
            t.set_employees(f, (employees + need) as u16);
            return;
        }
        need -= idle;
        t.set_employees(f, (employees + idle) as u16);
        t.set_idle(f, 0);
    }
    // Las granjas de grano (tipo 9) tienen hasta 60 empleados y las demás 30; el original invierte
    // los casos en las granjas y contrata toda la necesidad cuando no cabe.
    let employees = t.employees(f) as u32;
    let limit = if ty == 9 { 60 } else { 30 };
    if employees < limit {
        let mut room = limit - employees;
        let hire_all = if ty == 9 { room < need } else { room >= need };
        if hire_all {
            hire_workers(t, f, &mut need);
        } else {
            need = need.wrapping_sub(room);
            hire_workers(t, f, &mut room);
            need = need.wrapping_add(room);
        }
    }
    record_labour_shortage(&mut world.labour, ty as u16, need as i16);
}
