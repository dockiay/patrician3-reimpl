//! Población de la ciudad en el tick diario: mínimo de pobres, consumo de los ciudadanos y hambruna,
//! media del comercio y copia de existencias.

use super::world::{EconomyWorld, TownHost};
use crate::numeric::{ftol_i32, idiv};
use crate::town::{off, Town, FACILITIES, WARES};

/// Consumo diario por mercancía y grupo (ricos, acomodados, pobres, mendigos), en centésimas por
/// habitante (`0x00672860`, `i16 [24][4]`).
pub const CONSUMPTION: [[i16; 4]; WARES] = [
    [90, 120, 150, 120],
    [110, 87, 12, 5],
    [40, 80, 100, 110],
    [65, 130, 65, 75],
    [1, 1, 1, 1],
    [50, 25, 5, 2],
    [4, 2, 2, 0],
    [150, 38, 0, 0],
    [50, 35, 15, 1],
    [60, 30, 0, 0],
    [50, 35, 10, 0],
    [80, 80, 40, 20],
    [100, 75, 25, 0],
    [44, 35, 5, 0],
    [10, 40, 20, 5],
    [0, 0, 0, 0],
    [0, 0, 0, 0],
    [5, 3, 2, 3],
    [30, 18, 12, 1],
    [1, 1, 0, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0],
];

/// Bandera de hambruna en `town_flags`.
pub const FAMINE: u32 = 0x1000;

/// `ensure_min_poor_population` (0x0051CD80): los pobres no bajan de los trabajadores de la ciudad
/// (4 por empleado u ocioso de sus instalaciones, menos los de los negocios de comerciantes) menos
/// ricos y acomodados.
pub fn ensure_min_poor(world: &EconomyWorld, t: &mut Town) {
    let mut workers = 0i32;
    for f in 0..FACILITIES {
        workers = workers.wrapping_add((t.employees(f) as i32 + t.idle(f) as i32) * 4);
    }
    let mut o = t.u16(off::FIRST_OFFICE);
    while o < world.office_count {
        let office = &world.offices[o as usize];
        let mut m = office.first_facility;
        while m < world.merchant_facility_count {
            let facility = &world.merchant_facilities[m as usize];
            workers = workers.wrapping_sub(facility.employees as i32 * 4);
            m = facility.next;
        }
        o = office.next;
    }
    let floor = workers.wrapping_sub(t.pop(1).wrapping_add(t.pop(0)));
    if t.pop(2) < floor {
        t.set_pop(2, floor);
    }
}

/// `update_trade_flow_average` (0x00527CD0): media móvil (13/14) de lo que salió de cada mercancía
/// desde el último tick (`stock_at_last_tick − stock`).
pub fn update_trade_flow_average(t: &mut Town) {
    for w in 0..WARES {
        let at = off::TRADE_FLOW_AVERAGE + 4 * w;
        let flow = t
            .i32(at)
            .wrapping_mul(13)
            .wrapping_sub(t.stock(w))
            .wrapping_add(t.i32(off::STOCK_AT_LAST_TICK + 4 * w));
        t.set_i32(at, idiv(flow, 14));
    }
}

/// `snapshot_stock` (0x00527D20): copia las existencias en `stock_at_last_tick`.
pub fn snapshot_stock(t: &mut Town) {
    for w in 0..WARES {
        t.set_i32(off::STOCK_AT_LAST_TICK + 4 * w, t.stock(w));
    }
}

/// `do_population_consumption` (0x00527D40): pobres, acomodados y ricos (en ese orden) consumen de las
/// existencias. En invierno se gasta un 20 % más de madera y pieles; sin grano suficiente, un 30 % más
/// de carne, pescado y paño. Sin grano el contador de hambre sube (más aún sin carne ni pescado) y
/// al pasar de 80 empieza la hambruna, que se avisa por la frontera.
pub fn do_population_consumption(world: &EconomyWorld, t: &mut Town, host: &mut dyn TownHost) {
    for g in (0..3).rev() {
        let pop = t.pop(g);
        if pop <= 0 {
            continue;
        }
        let mut need = [0i32; WARES];
        for (w, n) in need.iter_mut().enumerate() {
            *n = (CONSUMPTION[w][g] as i32).wrapping_mul(pop);
        }
        if t.flags() & 2 != 0 {
            need[11] = need[11].wrapping_add(need[11] / 5);
            need[9] = need[9].wrapping_add(need[9] / 5);
        }
        if t.stock(0) < need[0] / 100 {
            for w in [1, 2, 8] {
                need[w] = need[w].wrapping_mul(13) / 10;
            }
        }
        for (w, &n) in need.iter().enumerate() {
            if n <= 0 {
                continue;
            }
            let mut eaten = n / 100;
            t.add_i32(off::CITIZEN_CONSUMPTION + 4 * w, eaten);
            let stock = t.stock(w);
            if stock < eaten {
                eaten = stock;
                t.set_stock(w, 0);
            } else {
                t.set_stock(w, stock - eaten);
            }
            t.add_i32(off::STOCK_AT_LAST_TICK + 4 * w, eaten.wrapping_neg());
        }
    }
    let counter = t.u8(off::FAMINE_COUNTER);
    if t.stock(0) > 0 {
        let counter = if counter > 7 { counter - 8 } else { 0 };
        t.set_u8(off::FAMINE_COUNTER, counter);
        if t.flags() & FAMINE != 0 && counter < 10 {
            t.set_flags(t.flags() & !FAMINE);
        }
        return;
    }
    let mut counter = counter.wrapping_add(1);
    if t.stock(1) < t.threshold(1, 1) {
        counter = counter.wrapping_add(1);
    }
    if t.stock(2) < t.threshold(2, 0) {
        counter = counter.wrapping_add(1);
    }
    t.set_u8(off::FAMINE_COUNTER, counter);
    if counter <= 0x50 {
        return;
    }
    t.set_u8(off::FAMINE_COUNTER, 0x50);
    if t.flags() & FAMINE != 0 {
        return;
    }
    t.set_flags(t.flags() | FAMINE);
    let town = t.index();
    host.famine_message_with_world(world, town);
    for m in (0..world.merchant_count).rev() {
        let merchant = &world.merchants[m as usize];
        if merchant.home_town == town && merchant.kind == 0 {
            host.famine_notify(m, town, m as u32 == world.local_merchant);
        }
    }
}

/// Satisfacción base por dificultad y grupo (`0x006736A0`, `i8 [4][4]`, la fila 3 se solapa con los
/// pasos) y pasos de subida y bajada (`0x006736AC`, `u8 [4][2]`): los 20 bytes tal cual.
pub const SATISFACTION_TABLES: [u8; 20] = [
    249, 244, 236, 0, 243, 238, 229, 0, 236, 231, 224, 0, 3, 1, 2, 1, 1, 2, 1, 1,
];
/// Peso de cada mercancía en la satisfacción de ricos, acomodados y pobres (`0x00672938`).
pub const SATISFACTION_WEIGHTS: [[u8; WARES]; 3] = [
    [2, 5, 2, 2, 2, 3, 3, 5, 5, 3, 3, 3, 2, 2, 2, 0, 0, 0, 3, 0, 0, 0, 0, 0],
    [4, 4, 6, 6, 2, 2, 0, 2, 4, 2, 4, 4, 2, 2, 6, 0, 0, 0, 2, 0, 0, 0, 0, 0],
    [8, 4, 6, 6, 4, 0, 0, 0, 0, 0, 4, 6, 0, 4, 4, 0, 0, 0, 4, 0, 0, 0, 0, 0],
];

/// `min(q, 4)` con el `cmp q, 4; jbe` sin signo del original.
fn up_to_4(q: u32) -> i32 {
    q.min(4) as i32
}

/// Bonificación común de la satisfacción: sucesos, pozos, capillas y hospitales, calles, fiestas,
/// milicia, edificios dentro de la muralla, torres, defensa, impuestos e iglesia (16 bits bajos).
fn satisfaction_bonus(world: &EconomyWorld, t: &mut Town, host: &mut dyn TownHost) -> u16 {
    let total = t.i32(off::CITIZENS_TOTAL);
    if total < 1 {
        return 40;
    }
    let total = total as u32;
    let flags = t.flags();
    let mut b = 0i32;
    for (bit, malus) in [(0x10, 10), (0x800, 8), (0x8, 10), (0x200, 6), (0x400, 4), (0x1000, 10)] {
        if flags & bit != 0 {
            b -= malus;
        }
    }
    b += up_to_4(t.u8(off::WELLS) as u32 * 2000 / total);
    b += up_to_4((t.u8(off::HOSPITALS) as u32 + t.u8(off::CHAPELS) as u32) * 20000 / total);
    let streets = t.u16(off::STREETS_TOTAL) as u32;
    if streets > 0 {
        b += up_to_4(t.u16(off::STREETS_BUILT) as u32 * 4 / streets);
    }
    let weeks = world.tick.wrapping_sub(t.u32(off::CELEBRATION)) / 1792;
    if weeks < 4 {
        b += 4 - weeks as i32;
    }
    b += up_to_4(t.u8(off::MILITIA_UNITS) as u32 * 2000 / total);
    b += up_to_4(
        (idiv(
            t.u16(off::FACILITIES_INSIDE_WALLS) as i32 * 4,
            t.u16(off::FACILITIES_TOTAL) as i32,
        )) as u32,
    );
    let mut towers = t.u8(off::TOWERS) as u32;
    if t.u16(off::OUTRIGGER_ID) < world.ship_count {
        towers += 3;
    }
    let q = towers / 3;
    b += if q > 3 { 4 } else { q as i32 };
    if t.u16(off::DEFENSE_CAPACITY) == 0 {
        host.recount_defense_buildings(t);
    }
    let ranged: u32 = (1..4).map(|i| t.u8(off::MILITIA_UNITS + i) as u32).sum();
    let q = ranged * 50 / t.u16(off::DEFENSE_CAPACITY) as u32;
    b += if q > 3 { 4 } else { q as i32 };
    let rate = t.u8(off::TAX_RATE_AND_STAMP) as u32;
    let church = t.u8(off::CHURCH_LEVEL) as i32;
    (b + church - (rate * rate / 20) as i32 + 9) as u16
}

/// `update_citizen_satisfaction` (0x0051C830): con la bandera `0x100` de `more_flags`, calcula la
/// satisfacción objetivo de ricos, acomodados y pobres (base por dificultad, abastecimiento de lo que
/// consumen frente al umbral 0, y bonificación común) y acerca a ella la actual, entre −40 y 80. Sin
/// la bandera, todo queda en 40.
pub fn update_citizen_satisfaction(world: &EconomyWorld, t: &mut Town, host: &mut dyn TownHost) {
    let d = match world.difficulty {
        Some(byte) => (byte & 3) as usize,
        None => 1,
    };
    let (up, down) = match world.difficulty {
        Some(_) => (
            SATISFACTION_TABLES[12 + 2 * d] as u16,
            SATISFACTION_TABLES[13 + 2 * d] as u16,
        ),
        None => (2, 1),
    };
    if t.more_flags() & 0x100 == 0 {
        for g in 0..4 {
            t.set_u16(off::SATISFACTION + 2 * g, 40);
            t.set_u16(off::SATISFACTION_TARGET + 2 * g, 40);
        }
        return;
    }
    let bonus = satisfaction_bonus(world, t, host);
    for g in 0..3 {
        let mut target = SATISFACTION_TABLES[4 * d + g] as i8 as i16 as u16;
        for w in 0..WARES {
            if t.i32(off::CITIZEN_CONSUMPTION + 4 * w) <= 0 {
                continue;
            }
            let stock = t.stock(w);
            let limit = t.threshold(w, 0);
            let weight = SATISFACTION_WEIGHTS[g][w] as i32;
            let delta = if stock < limit {
                -idiv(weight.wrapping_mul(limit - stock), limit)
            } else if stock < limit.wrapping_mul(2) {
                idiv(weight.wrapping_mul(stock - limit), limit)
            } else {
                weight
            };
            target = target.wrapping_add(delta as u16);
        }
        let target = target.wrapping_add(bonus) as i16;
        t.set_u16(off::SATISFACTION_TARGET + 2 * g, target as u16);
        let current = t.i16(off::SATISFACTION + 2 * g);
        let twice = target as i32 * 2;
        let next = if twice > current as i32 {
            (current as u16).wrapping_add(up).cast_signed().min(0x50)
        } else if twice < current as i32 {
            (current as u16).wrapping_sub(down).cast_signed().max(-40)
        } else {
            current
        };
        t.set_u16(off::SATISFACTION + 2 * g, next as u16);
    }
}

/// `town_employ_beggars_in_vacancies` (0x0051CBE0): si hay viviendas de pobres libres, coloca
/// mendigos (de 4 en 4) en los huecos de mano de obra, del último al primero, repartidos con un paso
/// que depende del día. Los colocados pasan de mendigos a pobres.
pub fn employ_beggars(world: &mut EconomyWorld, town: usize) {
    let t = &world.towns[town];
    let beggars = t.pop(3);
    if beggars <= 4 {
        return;
    }
    let mut people = (beggars - 1) as u32;
    let poor = t.pop(2);
    let housing = t.u16(off::HOUSING + 4) as i32;
    if housing <= poor.wrapping_add(3) {
        return;
    }
    let mut room = housing.wrapping_sub(poor) as u32;
    people = people.min(room);
    if t.more_flags() & 0x200 == 0 {
        room = room.wrapping_sub(40);
        if room < 4 {
            return;
        }
        people = people.min(room);
    }
    let mut workers = people >> 2;
    let ls = &world.labour;
    let vacancies: u32 = if ls.count as i32 > 0 {
        (0..ls.count as usize).map(|i| ls.amounts[i] as u32).sum()
    } else {
        0
    };
    let step = if vacancies <= workers { 1 } else { vacancies / workers };
    let mut acc = (world.day_of_year as u32) & step;
    let mut hired = 0u32;
    let mut i = ls.count;
    let index = t.index();
    while i > 0 {
        i -= 1;
        let amount = world.labour.amounts[i as usize] as u32;
        acc = acc.wrapping_add(amount);
        if acc < step {
            continue;
        }
        let n = (acc / step).min(workers).min(amount);
        let ty = world.labour.types[i as usize] as i16;
        if ty < 0 {
            let m = (-1 - ty as i32) as u32;
            if m >= world.merchant_facility_count as u32 || world.merchant_facilities[m as usize].town != index {
                continue;
            }
            let f = &mut world.merchant_facilities[m as usize];
            f.employees = f.employees.wrapping_add(n as u16);
        } else if ty < 21 {
            world.towns[town].add_u16(Town::fac(ty as usize, crate::town::fac::EMPLOYEES), n as u16);
        } else {
            continue;
        }
        if n != 0 {
            workers -= n;
            acc = acc.wrapping_sub(n.wrapping_mul(step));
            hired += n;
        }
    }
    let t = &mut world.towns[town];
    let people = hired.wrapping_mul(4) as i32;
    t.set_pop(3, t.pop(3).wrapping_sub(people));
    t.set_pop(2, t.pop(2).wrapping_add(people));
}

/// Suma de los cuatro grupos en `citizens_total`.
fn recount_citizens(t: &mut Town) {
    let total = (0..4).fold(0i32, |s, g| s.wrapping_add(t.pop(g)));
    t.set_i32(off::CITIZENS_TOTAL, total);
}

/// `update_population_levels` (0x0051C650): los pobres suben a acomodados y ricos (o bajan) según su
/// satisfacción y las viviendas de cada grupo, y si están descontentos o sin vivienda emigran
/// llevándose trabajadores.
pub fn update_population_levels(world: &mut EconomyWorld, town: usize) {
    let t = &mut world.towns[town];
    let poor = t.pop(2);
    let rich = (t.i16(off::SATISFACTION) as i32 + 40).wrapping_mul(poor);
    let rich = if t.more_flags() & 0x400 != 0 {
        idiv(rich, 213)
    } else {
        idiv(rich, 320)
    };
    let middle = idiv((t.i16(off::SATISFACTION + 2) as i32 + 40).wrapping_mul(poor), 160);
    let caps = [rich.max(1), middle.max(1)];
    for (g, &cap) in caps.iter().enumerate() {
        let housing = t.u16(off::HOUSING + 2 * g) as i32;
        let cap = if housing < cap { housing.max(10) } else { cap };
        let pop = t.pop(g);
        if cap >= pop {
            let n = if pop != 0 {
                idiv(cap.wrapping_mul(2), pop) + 1
            } else {
                1
            };
            if t.pop(2) > n {
                t.set_pop(2, t.pop(2) - n);
                t.set_pop(g, pop.wrapping_add(n));
            }
        } else {
            let mut n = idiv(pop.wrapping_mul(2), cap) + 1;
            if n >= pop {
                n = pop - 1;
            }
            t.set_pop(2, t.pop(2).wrapping_add(n));
            t.set_pop(g, pop - n);
        }
    }
    let poor = t.pop(2);
    let mut leaving = if t.i16(off::SATISFACTION + 4) < 0 {
        idiv(poor, 100)
    } else {
        0
    };
    let homeless = poor.wrapping_sub(t.u16(off::HOUSING + 4) as i32);
    if homeless > 0 {
        leaving = leaving.max((homeless >> 2) + 1);
    }
    if leaving >= poor {
        leaving = poor - 1;
    }
    if leaving >= 4 {
        let workers = remove_workers_for_emigration(world, town, leaving >> 2);
        let t = &mut world.towns[town];
        t.set_pop(2, t.pop(2).wrapping_sub(workers.wrapping_mul(4)));
    }
    recount_citizens(&mut world.towns[town]);
}

/// Plantilla a partir de la cual una instalación tiene empleados de sobra (60 en la granja de grano).
fn spare_limit(f: usize) -> u16 {
    if f == 9 {
        60
    } else {
        30
    }
}

/// `remove_workers_for_emigration` (0x0051C360): quita `n` trabajadores para los que emigran: primero
/// ociosos, luego los que sobran en las instalaciones y por último de los negocios de comerciantes,
/// repartidos según lo que sobra. Devuelve cuántos quitó.
///
/// Al contar los negocios de comerciantes el original toma como siguiente oficina el índice con el
/// que acabó la lista de negocios, no `office.next`; aquí se reproduce.
pub fn remove_workers_for_emigration(world: &mut EconomyWorld, town: usize, n: i32) -> i32 {
    let t = &mut world.towns[town];
    let mut n = n;
    let mut removed = 0i32;
    let mut spare = 0i32;
    for f in 1..FACILITIES {
        let idle = t.idle(f) as i32;
        if idle > 0 {
            if idle >= n {
                t.set_idle(f, (idle as u16).wrapping_sub(n as u16));
                return n.wrapping_add(removed);
            }
            n -= idle;
            removed += idle;
            t.set_idle(f, 0);
        }
        let employees = t.employees(f);
        if employees > spare_limit(f) {
            spare += (employees - spare_limit(f)) as i32;
        }
    }
    let mut o = t.u16(off::FIRST_OFFICE);
    while o < world.office_count && n >= 0 {
        let mut m = world.offices[o as usize].first_facility;
        while m < world.merchant_facility_count {
            let facility = &world.merchant_facilities[m as usize];
            spare = spare.wrapping_add(facility.employees as i32);
            m = facility.next;
        }
        o = m;
    }
    let ratio = idiv(spare, n).max(2);
    let t = &mut world.towns[town];
    let mut acc = 0i32;
    for f in 1..FACILITIES {
        let employees = t.employees(f);
        if employees <= spare_limit(f) {
            continue;
        }
        acc += (employees - spare_limit(f)) as i32;
        if acc < ratio {
            continue;
        }
        let mut k = idiv(acc, ratio).min(n);
        if k < employees as i32 {
            t.set_employees(f, employees.wrapping_sub(k as u16));
        } else {
            k = employees as i32 - 1;
            t.set_employees(f, 1);
        }
        n -= k;
        acc -= k.wrapping_mul(ratio);
        removed += k;
    }
    if n <= 0 {
        return removed;
    }
    let mut o = world.towns[town].u16(off::FIRST_OFFICE);
    while o < world.office_count && n > 0 {
        let mut m = world.offices[o as usize].first_facility;
        while m < world.merchant_facility_count {
            let facility = &mut world.merchant_facilities[m as usize];
            let next = facility.next;
            let employees = facility.employees as i32;
            if employees > 0 {
                acc += employees;
                if acc >= ratio {
                    let mut k = idiv(acc, ratio).min(n);
                    if k <= employees {
                        facility.employees = (employees - k) as u16;
                    } else {
                        k = employees;
                        facility.employees = 0;
                    }
                    acc -= k.wrapping_mul(ratio);
                    n -= k;
                    removed += k;
                }
            }
            m = next;
        }
        o = world.offices[o as usize].next;
    }
    removed
}

/// `update_beggar_population` (0x0051C0E0): los mendigos tienden a `8 + √(habitantes · satisfacción
/// / 2) / 3`, con pasos limitados por √habitantes; un suceso (`0x800000` sin `0x8`) trae más. La
/// población total no pasa del tope de la partida (`more_flags`) y el cambio del día queda en el
/// historial de 8 días.
pub fn update_beggar_population(t: &mut Town, tick: u32) {
    let before = t.pop(3);
    let mut target = 8;
    let mood = t.i16(off::SATISFACTION + 6);
    if mood > 0 {
        let x = t.i32(off::CITIZENS_TOTAL).wrapping_mul(mood as i32) >> 1;
        target = idiv(ftol_i32((x as f64).sqrt()), 3) + 8;
    }
    let mut step = ftol_i32((t.i32(off::CITIZENS_TOTAL) as f64).sqrt());
    if target > before {
        if t.flags() & 8 == 0 {
            step = idiv(step.wrapping_add(4), 5);
            let mut more = (target - before).min(step);
            if t.more_flags() & 0x1000 != 0 {
                more = idiv(more.wrapping_mul(13).wrapping_add(9), 10);
            }
            if t.u16(off::FACILITY + 4) == 0 && more.wrapping_add(before) < 24 {
                more = 24 - before;
            }
            t.set_pop(3, t.pop(3).wrapping_add(more));
        }
    } else {
        step = idiv(step.wrapping_add(9), 10);
        let fewer = before - target;
        t.set_pop(3, t.pop(3).wrapping_sub(fewer.min(step)));
    }
    let flags = t.flags();
    if flags & 0x80_0008 == 0x80_0000 {
        t.set_flags(flags & !0x80_0000);
        let mut more = idiv(step, 6) + (target >> 1);
        let beggars = t.pop(3);
        if beggars > 50 {
            let total = t.i32(off::CITIZENS_TOTAL);
            if beggars.wrapping_add(more).wrapping_mul(4) > total {
                more = (idiv(total, 4) - beggars).max(0);
            }
        }
        t.set_pop(3, beggars.wrapping_add(more));
    }
    recount_citizens(t);
    let more_flags = t.more_flags();
    let limit: i32 = if more_flags & 0x200 != 0 {
        10_000_000
    } else if more_flags & 2 == 0 {
        400
    } else {
        let mut l = if more_flags & 0x10 != 0 { 1000 } else { 500 };
        if more_flags & 0x20 != 0 {
            l += 250;
        }
        if more_flags & 0x80 != 0 {
            l += 250;
        }
        l
    };
    let total = t.i32(off::CITIZENS_TOTAL);
    let mut excess = limit.wrapping_sub(total);
    if excess < 0 {
        let beggars = t.pop(3);
        if excess.wrapping_neg() >= beggars {
            excess = 1 - beggars;
        }
        t.set_pop(3, beggars.wrapping_add(excess));
        t.set_i32(off::CITIZENS_TOTAL, total.wrapping_add(excess));
    }
    let day = super::production::history_day(tick);
    t.set_u16(
        off::BEGGAR_HISTORY + 2 * day,
        (t.pop(3) as u16).wrapping_sub(before as u16),
    );
}
