//! Recálculo del intervalo de construcción IA (parte de la tarea 0x2C).
use crate::{ai::reputation::Merchant, scheduler::TaskRecord, town::Town};
#[derive(Clone, Copy, Debug)]
pub struct Office {
    pub next: u16,
    pub first_business: u16,
}
#[derive(Clone, Copy, Debug)]
pub struct Business {
    pub next: u16,
    pub employees: u16,
}

/// La decisión vuelve sin efectos si el barrido de comerciantes 20..N no encuentra oro > 40000.
/// Con menos de 20 comerciantes el original entra en la decisión completa.
pub fn decision_is_empty(merchants: &[Merchant]) -> Result<bool, String> {
    if merchants.len() < 20 {
        return Ok(false);
    }
    for merchant in &merchants[20..] {
        if merchant.bytes.len() < 4 {
            return Err("comerciante de IA truncado".into());
        }
        if merchant.i32(0) > 40000 {
            return Ok(false);
        }
    }
    Ok(true)
}

pub fn recompute(
    task: &mut TaskRecord,
    towns: &[Town],
    merchants: &[Merchant],
    offices: &[Office],
    businesses: &[Business],
    scale: u8,
) -> Result<(), String> {
    let mut largest = 0u32;
    for merchant in merchants {
        if merchant.bytes.len() < 0x478 {
            return Err("comerciante de IA truncado".into());
        }
        if merchant.u16(8) != 0 {
            continue;
        }
        let mut size = merchant.i32(0x474) as u32;
        let mut at = usize::from(merchant.u16(0xc));
        let mut visited = vec![false; offices.len()];
        while at < offices.len() {
            if visited[at] {
                return Err("cadena de oficinas cíclica".into());
            }
            visited[at] = true;
            let office = offices[at];
            let mut bi = usize::from(office.first_business);
            let mut seen = vec![false; businesses.len()];
            while bi < businesses.len() {
                if seen[bi] {
                    return Err("cadena de negocios cíclica".into());
                }
                seen[bi] = true;
                let b = businesses[bi];
                size = size.wrapping_add(u32::from(b.employees));
                bi = usize::from(b.next)
            }
            at = usize::from(office.next)
        }
        largest = largest.max(size)
    }
    let population = towns.iter().fold(1u32, |sum, t| sum.wrapping_add(t.u32(0x2d4)));
    let previous = task.u32_at(8).wrapping_add(1);
    if population == 0 || previous == 0 {
        return Err("divisor de construcción IA nulo".into());
    }
    let interval = 0x6978000u32 / population;
    let ratio = (largest.wrapping_shl(10) / previous).wrapping_add(u32::from(scale) * 102);
    let scaled = interval.wrapping_shl(10);
    let next = if ratio <= 0x400 {
        interval
    } else if ratio <= 0x600 {
        scaled / ratio
    } else {
        (((u64::from(scaled) * 0xaaaaaaab) >> 32) as u32) >> 10
    };
    task.set_u32_at(8, largest);
    task.set_u32_at(0xc, next);
    Ok(())
}
