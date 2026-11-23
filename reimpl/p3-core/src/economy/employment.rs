//! Empleo de las instalaciones de la ciudad: plantilla objetivo, contratación, despidos y regreso de
//! los trabajadores a la población, más el consumo de la milicia, el astillero y los constructores.
//!
//! Como en `production`, el original lee la ciudad de la tabla global que indica el byte `+7` de la
//! instalación; aquí es la ciudad que se recibe.

use super::world::{LabourShortage, TownHost};
use crate::numeric::{ftol_i32, idiv};
use crate::town::{fac, off, Town};

/// Plantilla máxima de cada tipo de instalación (`0x006735E8`).
pub const MAX_EMPLOYEES: [u8; 21] = [
    250, 40, 25, 5, 72, 65, 68, 78, 60, 68, 60, 48, 60, 54, 70, 50, 93, 58, 27, 26, 62,
];
/// Mercancía de cada tipo de instalación (`0x00672C2C`).
pub const FACILITY_WARE: [u8; 21] = [6, 6, 6, 20, 9, 2, 3, 12, 5, 0, 1, 11, 8, 4, 16, 14, 7, 18, 19, 15, 17];
/// Fracción de los ociosos que se despide cada día (`0x00673600`).
pub const DISMISSAL_FACTOR: f32 = 0.02;

/// `record_labour_shortage` (0x004D4A40): apunta una instalación a la que le faltan trabajadores
/// (como mucho 12 por entrada). El tipo se escribe aunque la cantidad no sea positiva.
pub fn record_labour_shortage(ls: &mut LabourShortage, facility_type: u16, amount: i16) {
    let n = ls.count as usize;
    if ls.count >= 63 {
        return;
    }
    ls.types[n] = facility_type;
    if amount > 0 {
        ls.amounts[n] = amount.min(12) as u16;
        ls.count += 1;
    }
}

/// `target_employees` (0x00510120): plantilla que quiere la instalación según las existencias de su
/// mercancía frente a los umbrales de precio.
pub fn target_employees(t: &Town, f: usize) -> u32 {
    let ty = t.facility_type(f) as usize;
    let ware = FACILITY_WARE[ty] as usize;
    let mut max = MAX_EMPLOYEES[ty] as u32;
    if ty == 5 && t.u32(off::FIELD_2CC) > 10 {
        max = 0x48;
    }
    let stock = t.stock(ware);
    let employees = t.employees(f) as u32;
    if t.threshold(ware, 3) < stock {
        return 1;
    }
    let target = if t.threshold(ware, 2) < stock {
        employees + 1
    } else if t.threshold(ware, 1) < stock {
        employees + 4
    } else if t.threshold(ware, 0) < stock {
        employees + 8
    } else {
        employees + 16
    };
    target.min(max)
}

/// `hire_workers` (0x005108E0): contrata de los mendigos (si los pobres están contentos) y de los
/// ociosos de las instalaciones 1–20 hasta cubrir `wanted`, que se va descontando.
pub fn hire_workers(t: &mut Town, f: usize, wanted: &mut u32) {
    if t.i16(off::SATISFACTION + 4) > 0 {
        let mut hired = (t.pop(3) >> 2) as u32;
        if *wanted < hired {
            hired = *wanted;
            *wanted = 0;
        } else {
            *wanted -= hired;
        }
        t.set_employees(f, t.employees(f).wrapping_add(hired as u16));
        let people = hired.wrapping_mul(4) as i32;
        t.set_pop(3, t.pop(3).wrapping_sub(people));
        t.set_pop(2, t.pop(2).wrapping_add(people));
    }
    let mut i = 1;
    while *wanted > 0 && i < 21 {
        let idle = t.idle(i);
        if idle > 0 {
            if idle as u32 >= *wanted {
                t.set_idle(i, idle.wrapping_sub(*wanted as u16));
                t.set_employees(f, t.employees(f).wrapping_add(*wanted as u16));
                *wanted = 0;
            } else {
                *wanted -= idle as u32;
                t.set_employees(f, t.employees(f).wrapping_add(idle));
                t.set_idle(i, 0);
            }
        }
        i += 1;
    }
}

/// `dismiss_workers` (0x005109B0): pasa `n` empleados a ociosos y devuelve a la población la parte
/// del 2 % diario de los ociosos que le toca, sin dejar menos de 30 entre empleados y ociosos. Las
/// instalaciones de tipo 0–3 devuelven los `n` directamente.
pub fn dismiss_workers(t: &mut Town, f: usize, n: u32) {
    let employees = t.employees(f);
    let mut n = n;
    if (employees as u32) < n {
        n = (employees as u32).wrapping_sub(1);
    }
    if t.facility_type(f) > 3 {
        let idle_at = Town::fac(f, fac::IDLE);
        let frac_at = Town::fac(f, fac::DISMISSAL_FRACTION);
        t.add_u16(idle_at, n as u16);
        let employees = employees.wrapping_sub(n as u16);
        t.set_employees(f, employees);
        let idle = t.u16(idle_at);
        let frac = ((idle as i32 as f32) as f64 * DISMISSAL_FACTOR as f64 + t.f32(frac_at) as f64) as f32;
        let k = ftol_i32(frac as f64) as u32;
        t.set_f32(frac_at, (frac as f64 - (k as f64 as f32) as f64) as f32);
        let staff = idle as u32 + employees as u32;
        n = if staff.wrapping_add(k) <= 30 {
            k
        } else if staff >= 30 {
            return;
        } else {
            30 - staff
        };
        t.set_employees(f, employees.wrapping_add(n as u16));
        t.set_u16(idle_at, t.u16(idle_at).wrapping_sub(n as u16));
        let frac = t.f32(frac_at);
        t.set_f32(frac_at, (frac as f64 - (n as f64 as f32) as f64) as f32);
    }
    if n != 0 {
        release_workers(t, f, n, false);
    }
}

/// `release_workers` (0x0050E380): quita `n` empleados (sin `exact`, deja al menos uno) y reparte
/// sus `4n` personas como mendigos, sacándolas de ricos, acomodados y pobres según su peso y su
/// descontento; lo que sobra vuelve como empleados o sale de uno en uno de los grupos.
pub fn release_workers(t: &mut Town, f: usize, n: u32, exact: bool) {
    let employees = t.employees(f);
    let mut n = n;
    if exact {
        n = n.min(employees as u32);
    } else if n >= employees as u32 {
        n = (employees as u32).wrapping_sub(1);
    }
    if n < 1 {
        return;
    }
    t.set_employees(f, employees.wrapping_sub(n as u16));
    let people = n.wrapping_mul(4) as i32;
    let mut weight = [0i32; 3];
    let mut sum = 0i32;
    for (g, w) in weight.iter_mut().enumerate() {
        let sat = t.i16(off::SATISFACTION + 2 * g) as i32;
        let base = if sat < 0 { 10 - sat } else { 10 };
        *w = t.pop(g).wrapping_sub(1).wrapping_mul(base);
        sum = sum.wrapping_add(*w);
    }
    while sum > 0x1000 {
        sum >>= 4;
        for w in weight.iter_mut() {
            *w >>= 4;
        }
    }
    let mut left = people;
    for (g, w) in weight.iter().enumerate() {
        let pop = t.pop(g);
        let mut a = idiv(w.wrapping_mul(people), sum);
        if a >= pop {
            a = pop.wrapping_sub(1);
        }
        if a > left {
            a = left;
        }
        t.set_pop(g, pop.wrapping_sub(a));
        left = left.wrapping_sub(a);
        t.set_pop(3, t.pop(3).wrapping_add(a));
    }
    if left > 3 {
        t.add_u16(Town::fac(f, fac::EMPLOYEES), (left >> 2) as u16);
        left &= 3;
    }
    let mut g = 2;
    while left > 0 {
        let pop = t.pop(g);
        if pop > 1 {
            t.set_pop(g, pop - 1);
            t.set_pop(3, t.pop(3).wrapping_add(1));
            left -= 1;
        }
        g = if g == 0 { 2 } else { g - 1 };
    }
}

/// `militia_consume` (0x0050E500): la milicia bebe cerveza, salvo con la bandera `0x10` (asedio).
pub fn militia_consume(t: &mut Town, f: usize) {
    if t.flags() & 0x10 != 0 {
        return;
    }
    let beer = idiv(
        (t.employees(f) as i32).wrapping_mul(t.efficiency(f)).wrapping_mul(60),
        100,
    ) >> 10;
    t.add_i32(off::BUSINESS_CONSUMPTION + 4 * 3, beer);
    let stock = t.stock(3);
    t.set_stock(3, if stock < beer { 0 } else { stock - beer });
}

/// `shipyard_consume` (0x0050E570): el astillero gasta brea (el doble con la bandera `0x10`) y
/// avanza obras y reparaciones con 100 por empleado.
pub fn shipyard_consume(t: &mut Town, f: usize, host: &mut dyn TownHost) {
    let employees = t.employees(f) as i32;
    let mut pitch = (employees * 71 >> 5) + 0x43;
    if t.flags() & 0x10 != 0 {
        pitch += pitch;
    }
    t.add_i32(off::BUSINESS_CONSUMPTION + 4 * 15, pitch);
    let stock = t.stock(15);
    t.set_stock(15, if stock > pitch { stock - pitch } else { 0 });
    let work = employees as u32 * 100;
    // El identificador de la instalación es el que recibe el original. La
    // ciudad viva debe corresponder a él antes de conectar sus colas.
    let town = t.u8(Town::fac(f, fac::TOWN));
    if town == t.index() {
        host.shipyard_advance_with_town(t, work)
    } else {
        host.shipyard_construction(town, work);
        host.shipyard_repairs(town, work)
    }
}

/// `builders_daily` (0x0050E610): los constructores avanzan las obras; devuelve la plantilla que
/// quieren (`5·(habitantes/5000 + 5)`, menos si la satisfacción ponderada es negativa, mínimo 5).
pub fn builders_daily(t: &mut Town, f: usize, host: &mut dyn TownHost) -> i32 {
    let target = (idiv(t.i32(off::CITIZENS_TOTAL), 5000) + 5).wrapping_mul(5);
    let workers = t.employees(f);
    host.advance_construction(t, workers);
    let sat = |g: usize| t.i16(off::SATISFACTION + 2 * g) as i32;
    let mood = sat(2) * 5 + sat(1) * 2 + sat(0);
    if mood >= 0 {
        return target;
    }
    let target = target + idiv(mood + 7, 8);
    target.max(5)
}
