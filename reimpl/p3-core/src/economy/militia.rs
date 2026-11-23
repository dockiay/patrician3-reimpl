//! Milicia de la ciudad (instalación 0): reclutamiento semanal, sueldos, despidos por impago y
//! ajuste automático de unidades cuando el alcalde no es un jugador humano. También `book_transaction`,
//! con la que la ciudad apunta ingresos y gastos.

use super::world::{EconomyWorld, TownHost};
use crate::numeric::idiv;
use crate::town::{off, Town};

/// Sueldo diario de cada tipo de unidad (`0x00672DB0`).
pub const MILITIA_WAGES: [u8; 4] = [33, 26, 40, 46];

/// Empleados de la instalación de la milicia (`Town+0x844`): 5 por unidad.
const MILITIA_EMPLOYEES: usize = off::FACILITY + 4;

fn units(t: &Town, i: usize) -> u8 {
    t.u8(off::MILITIA_UNITS + i)
}
fn recruits(t: &Town, i: usize) -> u8 {
    t.u8(off::MILITIA_RECRUITS + i)
}

/// Primer tipo con más reclutas o unidades (`cmp; jbe`: el primero gana los empates).
fn most(t: &Town, field: usize) -> usize {
    (1..4).fold(0, |m, k| if t.u8(field + k) > t.u8(field + m) { k } else { m })
}

/// `Town::book_transaction` (0x00529F90): suma `amount` a la partida `kind` (0–10); las partidas 0–4
/// son ingresos y las 5–10 gastos. La 2 (impuesto extraordinario) acumula también lo cobrado en los
/// últimos 0x5400 ticks y guarda la fecha en los bits altos de `+0x6F8`.
pub fn book_transaction(t: &mut Town, tick: u32, amount: i32, kind: u32) {
    if kind >= 11 {
        return;
    }
    t.add_i32(off::TRANSACTIONS + 4 * kind as usize, amount);
    if kind >= 5 {
        t.add_i32(off::MONEY, amount.wrapping_neg());
        return;
    }
    t.add_i32(off::MONEY, amount);
    if kind == 2 {
        let stamp = t.u32(off::TAX_RATE_AND_STAMP);
        if tick > stamp.wrapping_add(0x5400) {
            t.set_i32(off::RECENT_EXTRA_TAXES, amount);
        } else {
            t.add_i32(off::RECENT_EXTRA_TAXES, amount);
        }
        t.set_u32(off::TAX_RATE_AND_STAMP, (tick & !0xFF) | (stamp & 0xFF));
    }
}

/// ¿Es humano el alcalde? (`get_merchant(mayor)->kind == 0` con `mayor < merchant_count`).
fn human_mayor(world: &EconomyWorld, t: &Town) -> Option<u8> {
    let mayor = t.u8(off::MAYOR_ID);
    if (mayor as u16) < world.merchant_count && world.merchants[mayor as usize].kind == 0 {
        Some(mayor)
    } else {
        None
    }
}

/// `dismiss_militia_unit` (0x0051D410): quita un recluta (o, si no hay, una unidad) del tipo `i`;
/// sus 20 personas pasan a mendigos sacándolas de pobres, acomodados y ricos (sin dejar ninguno a 0).
pub fn dismiss_militia_unit(t: &mut Town, i: u32) {
    if i >= 4 || t.u16(MILITIA_EMPLOYEES) < 5 {
        return;
    }
    let i = i as usize;
    if units(t, i) == 0 && recruits(t, i) == 0 {
        return;
    }
    if recruits(t, i) > 0 {
        t.set_u8(off::MILITIA_RECRUITS + i, recruits(t, i) - 1);
    } else {
        t.set_u8(off::MILITIA_UNITS + i, units(t, i) - 1);
    }
    t.set_pop(3, t.pop(3).wrapping_add(20));
    let mut need = 20i32;
    for g in (0..3).rev() {
        let p = t.pop(g);
        if p > need {
            t.set_pop(g, p - need);
            break;
        }
        t.set_pop(g, 1);
        need = need.wrapping_add(1 - p);
    }
    t.add_u16(MILITIA_EMPLOYEES, 0xFFFB);
}

/// `recruit_militia_unit` (0x0051D4C0): recluta una unidad del tipo `i` con 50 armas del almacén
/// de la ciudad (o de la oficina del alcalde humano) y 20 mendigos, que pasan a pobres.
pub fn recruit_militia_unit(world: &mut EconomyWorld, town: usize, i: u32, host: &mut dyn TownHost) {
    if i >= 4 {
        return;
    }
    let i = i as usize;
    let t = &world.towns[town];
    let wall = host.wall_expansion_level(t) as u32;
    if (units(t, i) as i32 + recruits(t, i) as i32) >= (10i32.wrapping_shl(wall)) {
        return;
    }
    if t.flags() & 0xE0 != 0 {
        let total: i32 = (0..4).map(|k| units(t, k) as i32 + recruits(t, k) as i32).sum();
        if total >= t.u16(off::MILITIA_LIMIT) as i32 {
            return;
        }
    }
    if t.pop(3) < 20 {
        return;
    }
    let office = match human_mayor(world, t) {
        Some(mayor) => {
            let index = t.index();
            match host.find_or_create_office(world, mayor, index) {
                Some(o) => Some(o),
                None => return,
            }
        }
        None => None,
    };
    let weapons = match office {
        Some(o) => world.offices[o].weapons[i],
        None => world.towns[town].stock(20 + i),
    };
    if weapons < 50 {
        return;
    }
    match office {
        Some(o) => world.offices[o].weapons[i] = weapons - 50,
        None => world.towns[town].set_stock(20 + i, weapons - 50),
    }
    let t = &mut world.towns[town];
    t.add_u16(MILITIA_EMPLOYEES, 5);
    t.set_pop(3, t.pop(3).wrapping_sub(20));
    t.set_pop(2, t.pop(2).wrapping_add(20));
    t.set_u8(off::MILITIA_RECRUITS + i, recruits(t, i).wrapping_add(1));
}

/// `militia_daily_tick` (0x00510C50) sobre la instalación 0 de la ciudad `town`.
pub fn militia_daily_tick(world: &mut EconomyWorld, town: usize, host: &mut dyn TownHost) {
    let tick = world.tick;
    let t = &mut world.towns[town];
    // Cada 7 días un recluta de cada tipo pasa a unidad.
    let weekly = (tick >> 8) % 7 == 0;
    let mut total = 0u32;
    for i in 0..4 {
        if weekly && recruits(t, i) > 0 {
            t.set_u8(off::MILITIA_RECRUITS + i, recruits(t, i) - 1);
            t.set_u8(off::MILITIA_UNITS + i, units(t, i).wrapping_add(1));
        }
        total += recruits(t, i) as u32 + units(t, i) as u32;
    }
    // La plantilla sigue a las unidades; la diferencia sale de mendigos y pobres.
    let staff = total * 5;
    let employees = t.u16(MILITIA_EMPLOYEES) as u32;
    if employees != staff {
        let people = staff.wrapping_sub(employees).wrapping_mul(4) as i32;
        t.set_u16(MILITIA_EMPLOYEES, staff as u16);
        let beggars = t.pop(3).wrapping_sub(people);
        t.set_pop(3, beggars);
        if beggars < 0 {
            t.set_pop(3, 0);
            t.add_i32(off::CITIZENS_TOTAL, beggars.wrapping_neg());
        }
        let poor = t.pop(2).wrapping_add(people);
        t.set_pop(2, poor);
        if poor < 0 {
            let poor = poor
                .wrapping_add(t.pop(0))
                .wrapping_sub(1)
                .wrapping_add(t.pop(1).wrapping_sub(1));
            t.set_pop(0, 1);
            t.set_pop(1, 1);
            t.set_pop(2, poor);
        }
    }
    let wages: i32 = (0..4)
        .map(|i| (units(t, i) as i32 + recruits(t, i) as i32) * MILITIA_WAGES[i] as i32)
        .sum();
    if wages > t.i32(off::MONEY) {
        let unpaid = t.u8(off::MILITIA_UNPAID).wrapping_add(2);
        t.set_u8(off::MILITIA_UNPAID, unpaid);
        if unpaid >= 10 {
            t.set_u8(off::MILITIA_UNPAID, 0);
            if t.flags() & 0x10 == 0 {
                let r = most(t, off::MILITIA_RECRUITS);
                if recruits(t, r) > 0 {
                    dismiss_militia_unit(t, r as u32);
                } else {
                    let u = most(t, off::MILITIA_UNITS);
                    if units(t, u) > 0 {
                        dismiss_militia_unit(t, u as u32);
                    }
                }
            }
        }
    }
    book_transaction(t, tick, wages, 5);
    if human_mayor(world, &world.towns[town]).is_some() {
        return;
    }
    let t = &world.towns[town];
    if t.flags() & 0xF10 != 0 {
        for i in 0..4 {
            recruit_militia_unit(world, town, i, host);
        }
        return;
    }
    let want = idiv(t.i32(off::CITIZENS_TOTAL), 80);
    let have = units(t, 0) as i32 + recruits(t, 0) as i32;
    if want > have {
        recruit_militia_unit(world, town, 0, host);
    } else if want < have && want > 0 {
        dismiss_militia_unit(&mut world.towns[town], 0);
    }
    let t = &world.towns[town];
    let cap = t.u16(off::DEFENSE_CAPACITY) as i32 / 5;
    let ranged: i32 = (1..4).map(|i| units(t, i) as i32 + recruits(t, i) as i32).sum();
    if cap > ranged {
        let kind = (world.day_of_year as u32 & 1) + 1;
        recruit_militia_unit(world, town, kind, host);
    } else if cap < ranged {
        let kind = if recruits(t, 1) as i32 + units(t, 1) as i32 > 0 {
            1
        } else if recruits(t, 2) as i32 + units(t, 2) as i32 > 0 {
            2
        } else {
            3
        };
        dismiss_militia_unit(&mut world.towns[town], kind);
    }
}
