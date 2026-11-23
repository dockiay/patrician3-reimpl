//! Mercado de la ciudad en el tick diario: umbrales de precio, mercancía más escasa, consumo de lo que
//! sobra, tope del tesoro con alcalde no comerciante y suministro diario de las entradas de la ciudad.

use super::employment::FACILITY_WARE;
use super::population::SATISFACTION_WEIGHTS;
use super::prices::SMALL_LOT_WARES;
use super::world::{EconomyWorld, TickEntry, TownHost};
use crate::numeric::idiv;
use crate::town::{off, Town};

/// Máscaras de mercancías con umbral 0 doble según la situación (`0x00672920`): asedio, bloqueo o
/// peste (`0x4000A10`), hambruna (`0x1000`), sitio (`0x8`) y bandera `0x8000`.
pub const THRESHOLD_MASKS: [u32; 4] = [0x8FF, 0x7F, 0xFF, 0x8FF];

fn add_threshold(t: &mut Town, w: usize, j: usize, d: i32) {
    t.set_threshold(w, j, t.threshold(w, j).wrapping_add(d));
}

/// `update_town_price_thresholds` (0x00528070). Sin `active` (bit `0x100` de `more_flags`) los
/// umbrales de las 19 primeras mercancías quedan en 1, 2, 3 y 4. Con él se calculan a partir del
/// consumo de ciudadanos y empresas, la producción, el mes, el asedio y los habitantes, y al final se
/// fuerzan crecientes y por encima de 1000 (lotes pequeños) o 10000 por paso.
pub fn update_town_price_thresholds(world: &EconomyWorld, t: &mut Town, active: bool, host: &mut dyn TownHost) {
    if !active {
        for w in 0..19 {
            for j in 0..4 {
                t.set_threshold(w, j, j as i32 + 1);
            }
        }
        return;
    }
    let flags = t.flags();
    let (mut mask, step, scale) = if flags & 0x0400_0A10 != 0 {
        (THRESHOLD_MASKS[0], 28, 2)
    } else if flags & 0x8000 != 0 {
        (THRESHOLD_MASKS[3], 14, 1)
    } else if flags & 0x1000 != 0 {
        (THRESHOLD_MASKS[2], 14, 1)
    } else if flags & 8 != 0 {
        (THRESHOLD_MASKS[1], 14, 1)
    } else {
        (0, 14, 1)
    };
    for w in 0..19 {
        let base = if mask & 1 != 0 { 14 } else { 7 };
        t.set_threshold(w, 0, base);
        t.set_threshold(w, 1, base + step);
        mask >>= 1;
    }
    for w in 19..24 {
        t.set_threshold(w, 0, 0);
        t.set_threshold(w, 1, 0);
    }
    add_threshold(t, 0, 1, step);
    for w in 0..20 {
        let demand = t
            .i32(off::CITIZEN_CONSUMPTION + 4 * w)
            .wrapping_add(t.i32(off::BUSINESS_CONSUMPTION + 4 * w))
            .wrapping_add(1);
        for j in 0..2 {
            t.set_threshold(w, j, t.threshold(w, j).wrapping_mul(demand));
        }
    }
    if world.sieges && flags & 0x10 != 0 {
        let pitch = host.siege_pitch_demand_for_town(t);
        add_threshold(t, 15, 0, pitch.wrapping_mul(7));
        add_threshold(t, 15, 1, pitch.wrapping_mul(14));
    }
    // Final del año: más grano y vino de reserva.
    let (num, den) = match world.month {
        8 => (3, 20),
        9 => (3, 10),
        10 => (1, 5),
        11 => (1, 10),
        _ => (0, 1),
    };
    if den > 1 {
        for w in [0, 7] {
            let x = t.threshold(w, 1);
            t.set_threshold(w, 1, x.wrapping_add(idiv(x.wrapping_mul(num), den)));
        }
    }
    add_threshold(t, 15, 0, 1800);
    add_threshold(t, 19, 0, 80000 * scale);
    add_threshold(t, 11, 0, 42000);
    add_threshold(t, 11, 1, 84000);
    add_threshold(t, 19, 1, 160000 * scale);
    add_threshold(t, 15, 1, 3600);
    add_threshold(t, 17, 0, 5000 * scale);
    add_threshold(t, 17, 1, 10000 * scale);
    add_threshold(t, 12, 0, 2000 * scale);
    add_threshold(t, 12, 1, 4000 * scale);
    let mut people = idiv(t.i32(off::CITIZENS_TOTAL).wrapping_mul(10), 200);
    if flags & 0x810 != 0 {
        people = people.wrapping_add(people);
    }
    for w in 20..24 {
        let used = t.i32(off::CITIZEN_CONSUMPTION + 4 * w).wrapping_mul(7).max(10);
        let t0 = used.wrapping_add(people);
        t.set_threshold(w, 0, t0);
        let t1 = t0.wrapping_add(used.wrapping_mul(2)).wrapping_add(people).max(50);
        t.set_threshold(w, 1, t1);
        let made = t.i32(off::DAILY_PRODUCTION + 4 * w).wrapping_add(1);
        let t2 = t1.wrapping_add(made.wrapping_mul(10));
        t.set_threshold(w, 2, t2);
        t.set_threshold(w, 3, t2.wrapping_add(used).wrapping_add(people));
    }
    for w in 0..20 {
        let t2 = t
            .threshold(w, 1)
            .wrapping_add(t.i32(off::PRODUCTION_VALUES + 4 * w).wrapping_mul(10));
        t.set_threshold(w, 2, t2);
        t.set_threshold(w, 3, t.threshold(w, 0).wrapping_add(t2));
    }
    if t.i32(off::DAILY_PRODUCTION + 4 * 15) > 0 {
        add_threshold(t, 15, 3, 3600);
    }
    if t.i32(off::DAILY_PRODUCTION + 4 * 19) > 0 {
        add_threshold(t, 19, 3, 160000);
    }
    for (j, d) in [400, 800, 800, 1600].into_iter().enumerate() {
        add_threshold(t, 8, j, d);
    }
    if t.u16(0x968) > 300 {
        let value = t.i32(off::PRODUCTION_VALUES + 4 * 19);
        let t2 = value.wrapping_add(t.threshold(19, 1));
        t.set_threshold(19, 2, t2);
        t.set_threshold(19, 3, t2.wrapping_add(value));
    } else {
        let t1 = t.threshold(19, 1);
        if t.threshold(19, 2) > t1.wrapping_mul(2) {
            t.set_threshold(19, 2, t1.wrapping_mul(2));
            t.set_threshold(19, 3, t1.wrapping_mul(3));
        }
    }
    for w in 0..20 {
        let d = if SMALL_LOT_WARES[w] { 1000 } else { 10000 };
        let mut floor = d;
        if t.threshold(w, 0) < floor {
            t.set_threshold(w, 0, floor);
        }
        floor += d;
        for j in 1..4 {
            if t.threshold(w, j) < floor {
                t.set_threshold(w, j, floor);
            }
            let prev = t.threshold(w, j - 1);
            if t.threshold(w, j) <= prev {
                t.set_threshold(w, j, prev.wrapping_add(d));
            }
            floor += d;
        }
    }
}

/// `town_compute_scarcest_ware_flag_bits` (0x0051BDB0): bits 17–22 de `town_flags` con la mercancía
/// (0–19) más escasa bajo 200 o 2000 unidades, o `0x7C0000` si no hay ninguna. Sin existencias la
/// escasez se mide como `−1 − pesos de satisfacción`; con existencias, `existencias / umbral 0`.
pub fn scarcest_ware_flag_bits(t: &Town) -> u32 {
    let mut best = 400.0f32;
    let mut ware = 0x1F;
    for w in 0..20 {
        let stock = t.stock(w);
        let limit = if SMALL_LOT_WARES[w] { 200 } else { 2000 };
        if stock >= limit {
            continue;
        }
        let ratio = if stock == 0 {
            let mut r = -1.0f64;
            for g in (0..3).rev() {
                r -= SATISFACTION_WEIGHTS[g][w] as f64;
            }
            r as f32
        } else {
            ((stock as f32) as f64 / (t.threshold(w, 0) as f32) as f64) as f32
        };
        if ratio < best {
            best = ratio;
            ware = w as u32;
        }
    }
    if ware >= 20 {
        0x7C_0000
    } else {
        (ware << 18) | 0x2_0000
    }
}

/// `town_excess_consumption` (0x00528630): con las banderas `0xE0`, lo que sobra por encima del
/// umbral 1 de las mercancías que la ciudad no produce se gasta en proporción a su consumo (entre el
/// umbral 3).
pub fn excess_consumption(t: &mut Town) {
    if t.flags() & 0xE0 == 0 {
        return;
    }
    for w in 0..20 {
        if t.i32(off::DAILY_PRODUCTION + 4 * w) != 0 {
            continue;
        }
        let stock = t.stock(w);
        let t1 = t.threshold(w, 1);
        if stock <= t1 {
            continue;
        }
        let demand = t
            .i32(off::BUSINESS_CONSUMPTION + 4 * w)
            .wrapping_add(t.i32(off::CITIZEN_CONSUMPTION + 4 * w))
            << 4;
        let q = (stock.wrapping_sub(t1) as i64 * demand as i64) / 10;
        let used = (q / t.threshold(w, 3) as i64) as i32;
        let left = if stock > used { stock - used } else { 0 };
        t.set_stock(w, left.max(t1));
    }
}

/// `town_clamp_treasury_when_npc_mayor` (0x0052A170): sin alcalde comerciante, el tesoro se mantiene
/// entre un mínimo (habitantes/16 + día + 16·ciudad) y un máximo que baja con el día del año.
pub fn clamp_treasury_when_npc_mayor(world: &EconomyWorld, t: &mut Town) {
    if (t.u8(off::MAYOR_ID) as u16) < world.merchant_count {
        return;
    }
    let day = world.day_of_year as i32;
    let index = t.index() as i32;
    let high = ((20000 - index) * 5 - day) * 10 - (t.i32(off::CITIZEN_CONSUMPTION + 4 * 3) & 0x3FF);
    let low = index * 16 + day + (t.i32(off::CITIZENS_TOTAL) >> 4);
    let money = t.i32(off::MONEY);
    if high < money {
        t.set_i32(off::MONEY, high);
    } else if low > money {
        t.set_i32(off::MONEY, low);
    }
}

/// `town_tick_entry_daily_supply` (0x0050FE40): la entrada añade a su ciudad una parte del consumo
/// mundial de la mercancía de su instalación; el factor (1024 = 100 %) sube de 10 en 10 hasta `max`
/// mientras la mercancía escasea y baja de 2 en 2 (sin bajar de 4) cuando abunda.
pub fn tick_entry_daily_supply(world: &mut EconomyWorld, entry: &mut TickEntry) {
    let ware = FACILITY_WARE[entry.facility_type as usize] as usize;
    let mut sum = 0i64;
    for town in &world.towns {
        let used = town
            .i32(off::BUSINESS_CONSUMPTION + 4 * ware)
            .wrapping_add(town.i32(off::CITIZEN_CONSUMPTION + 4 * ware));
        sum = sum.wrapping_add(used as i64);
    }
    let t = &mut world.towns[entry.town as usize];
    let stock = t.stock(ware);
    let (t0, t1, t2) = (t.threshold(ware, 0), t.threshold(ware, 1), t.threshold(ware, 2));
    let mid = idiv(t1.wrapping_sub(t0).wrapping_mul(3), 5).wrapping_add(t0);
    let wanted = if stock >= t2 {
        0
    } else if stock <= mid {
        0x400
    } else {
        idiv(mid.wrapping_sub(stock) << 10, t2.wrapping_sub(mid)) + 0x400
    };
    let factor = if wanted < entry.factor as i32 {
        entry.factor = entry.factor.wrapping_sub(2).max(4);
        wanted.max(4)
    } else {
        entry.factor = entry.factor.wrapping_add(10).min(entry.max);
        entry.factor as i32
    };
    let sum = sum as i32;
    entry.sum = sum & 0x7FFF_FFFF;
    entry.amount = sum.wrapping_mul(factor) >> 10;
    t.set_stock(ware, stock.wrapping_add(entry.amount));
}
