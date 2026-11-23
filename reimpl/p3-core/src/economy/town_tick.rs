//! Tick diario de una ciudad (`handle_town_tick`, 0x0051BA10): ordena todos los pasos de C04 y llama
//! por la frontera (`TownHost`) a oficinas, construcción, iglesia, taberna y consejo.

use super::facility::daily_tick;
use super::market::{clamp_treasury_when_npc_mayor, excess_consumption, scarcest_ware_flag_bits};
use super::market::{tick_entry_daily_supply, update_town_price_thresholds};
use super::militia::book_transaction;
use super::population::update_trade_flow_average;
use super::population::{do_population_consumption, employ_beggars, ensure_min_poor, snapshot_stock};
use super::population::{update_beggar_population, update_citizen_satisfaction, update_population_levels};
use super::world::{EconomyWorld, TownHost};
use crate::town::{off, Town, FACILITIES, WARES};

/// Saca la ciudad del mundo mientras se llama a pasos que leen el resto del mundo.
fn with_town<R>(world: &mut EconomyWorld, town: usize, f: impl FnOnce(&mut EconomyWorld, &mut Town) -> R) -> R {
    let mut t = std::mem::take(&mut world.towns[town]);
    let r = f(world, &mut t);
    world.towns[town] = t;
    r
}

/// Barco de apoyo (`outrigger`): cada 7 días cobra su precio a la ciudad o se retira si no hay dinero.
fn pay_outrigger(world: &mut EconomyWorld, town: usize) {
    let t = &world.towns[town];
    let ship = t.u16(off::OUTRIGGER_ID);
    if ship >= world.ship_count || (world.tick >> 8) % 7 != 0 {
        return;
    }
    let value = t.u32(off::OUTRIGGER_VALUE);
    if value > t.u32(off::MONEY) {
        world.ships[ship as usize].state = 3;
        world.towns[town].set_u16(off::OUTRIGGER_ID, 0xFFFF);
        return;
    }
    let owner = world.ships[ship as usize].owner as u16;
    if owner >= world.merchant_count {
        return;
    }
    let merchant = &mut world.merchants[owner as usize];
    merchant.gold = merchant.gold.wrapping_add(value as i32);
    merchant.income = merchant.income.wrapping_add(value as i32);
    let tick = world.tick;
    book_transaction(&mut world.towns[town], tick, value as i32, 7);
}

/// Llama al tick diario de cada oficina de la ciudad y pasa la última al principio de la lista.
fn office_daily_ticks(world: &mut EconomyWorld, town: usize, arg: u32, host: &mut dyn TownHost) {
    // `None` es el enlace de la ciudad (`first_office`); `Some(o)` el `next` de la oficina `o`.
    let mut link: Option<usize> = None;
    let read = |world: &EconomyWorld, link: Option<usize>| match link {
        None => world.towns[town].u16(off::FIRST_OFFICE),
        Some(o) => world.offices[o].next,
    };
    let mut o = read(world, link);
    if o >= world.office_count {
        return;
    }
    loop {
        host.office_daily_tick(world, o, arg);
        let next = world.offices[o as usize].next;
        if next < world.office_count {
            link = Some(o as usize);
            o = next;
            continue;
        }
        let first = world.towns[town].u16(off::FIRST_OFFICE);
        let last = read(world, link);
        if last != first {
            world.offices[o as usize].next = first;
            world.towns[town].set_u16(off::FIRST_OFFICE, last);
            match link {
                None => world.towns[town].set_u16(off::FIRST_OFFICE, 0xFFFF),
                Some(p) => world.offices[p].next = 0xFFFF,
            }
            host.office_links_changed(world);
        }
        return;
    }
}

/// `handle_town_tick` (0x0051BA10): tick diario de la ciudad `town`. `arg` es el argumento que el
/// original pasa al tick de cada oficina.
pub fn handle_town_tick(world: &mut EconomyWorld, town: usize, arg: u32, host: &mut dyn TownHost) {
    let business = handle_town_tick_until_facilities(world, town, host);
    for f in 0..FACILITIES {
        host.before_facility(world, town, f);
        daily_tick(world, town, f, host);
        host.after_facility(world, town, f);
    }
    handle_town_tick_rest(world, town, arg, business, host);
}

/// Primera mitad del tick, hasta justo antes de las instalaciones: invierno, pago del barco de apoyo,
/// puesta a cero de los contadores del día y consumo de los ciudadanos. Devuelve los empleados de la
/// instalación 14 (el original los lee al empezar y los suma a los de las oficinas).
fn handle_town_tick_until_facilities(world: &mut EconomyWorld, town: usize, host: &mut dyn TownHost) -> i32 {
    let business = world.towns[town].employees(14) as i32;
    let winter = !(2..=10).contains(&world.month);
    let t = &mut world.towns[town];
    t.set_flags(if winter { t.flags() | 2 } else { t.flags() & !2 });
    pay_outrigger(world, town);
    let t = &mut world.towns[town];
    for w in 0..WARES {
        t.set_i32(off::BUSINESS_CONSUMPTION + 4 * w, 0);
        t.set_i32(off::DAILY_PRODUCTION + 4 * w, 0);
        t.set_i32(off::CITIZEN_CONSUMPTION + 4 * w, 0);
        if t.stock(w) < 0 {
            t.set_stock(w, 0);
        }
    }
    with_town(world, town, |world, t| {
        ensure_min_poor(world, t);
        for g in 0..4 {
            t.set_i32(off::CITIZENS_OLD + 4 * g, t.pop(g));
        }
        update_trade_flow_average(t);
        do_population_consumption(world, t, host);
        for w in 0..WARES {
            t.set_i32(off::PRODUCTION_VALUES + 4 * w, 0);
        }
    });
    world.labour.count = 0;
    business
}

/// Segunda mitad del tick, desde la copia de existencias hasta el consejo y el planificador.
fn handle_town_tick_rest(world: &mut EconomyWorld, town: usize, arg: u32, business: i32, host: &mut dyn TownHost) {
    let mut business = business;
    snapshot_stock(&mut world.towns[town]);
    let mut o = world.towns[town].u16(off::FIRST_OFFICE);
    while o < world.office_count {
        business = business.wrapping_add(host.office_update_business_employees(world, o) as i32);
        o = world.offices[o as usize].next;
    }
    if business > 0 {
        host.try_add_type_2f_building(&mut world.towns[town], business as u32);
    }
    if world.towns[town].index() & 0xF == (world.day_of_year & 0xF) as u8 {
        host.count_street_cells(&mut world.towns[town]);
    }
    with_town(world, town, |world, t| update_citizen_satisfaction(world, t, host));
    employ_beggars(world, town);
    office_daily_ticks(world, town, arg, host);
    let flags = world.towns[town].flags();
    if flags & 0x10 == 0 {
        let mut entries = std::mem::take(&mut world.tick_entries[town]);
        for entry in entries.iter_mut() {
            tick_entry_daily_supply(world, entry);
        }
        world.tick_entries[town] = entries;
    }
    update_population_levels(world, town);
    host.recount_buildings(&mut world.towns[town]);
    let tick = world.tick;
    let t = &mut world.towns[town];
    if t.flags() & 0x10 == 0 {
        update_beggar_population(t, tick);
    }
    let active = (t.more_flags() >> 8) & 1 != 0;
    with_town(world, town, |world, t| {
        update_town_price_thresholds(world, t, active, host)
    });
    let t = &mut world.towns[town];
    let flags = t.flags() & 0xFF81_FFFF;
    t.set_flags(flags | scarcest_ware_flag_bits(t));
    let index = t.index();
    host.church_daily_update(t, index);
    with_town(world, town, |world, t| host.tavern_daily_with_world(world, t));
    let t = &mut world.towns[town];
    if t.flags() & 0x10 == 0 && t.more_flags() > 0x200 {
        excess_consumption(t);
    }
    let mayor = t.u8(off::MAYOR_ID) as u16;
    if mayor < world.merchant_count {
        if world.merchants[mayor as usize].kind == 0 {
            return;
        }
    } else {
        with_town(world, town, |world, t| clamp_treasury_when_npc_mayor(world, t));
    }
    if world.world_8a == 0 && world.network_client == 0 {
        with_town(world, town, |world, t| host.council_offer_with_world(world, t));
        with_town(world, town, |world, t| host.construction_planner_with_world(world, t));
    }
}
