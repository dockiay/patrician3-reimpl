//! Elección periódica de oficina para la IA (tarea 0x2A).
use crate::{ai::reputation::Merchant, scheduler::TaskRecord, town::Town};
#[derive(Clone, Copy, Debug)]
pub struct Office {
    pub owner: u16,
    pub next_merchant: u16,
    pub next_town: u16,
    pub first_business: u16,
}
#[derive(Clone, Copy, Debug)]
pub struct Business {
    pub next: u16,
    pub employees: u16,
    pub maximum: u16,
}

/// Devuelve la pareja elegida y actualiza el plazo; no crea la oficina.
pub fn choose(
    task: &mut TaskRecord,
    towns: &[Town],
    merchants: &[Merchant],
    offices: &[Office],
    businesses: &[Business],
) -> Result<Option<(u16, u16)>, String> {
    let mut best_money = 40000u32;
    let mut owner = None;
    for i in (0..merchants.len()).rev() {
        let merchant = &merchants[i];
        if merchant.bytes.len() < 14 {
            return Err("comerciante de expansión truncado".into());
        }
        let money = merchant.i32(0) as u32;
        let kind = merchant.u16(8);
        if money < best_money || kind == 0 || kind & 4 != 0 {
            continue;
        }
        let mut n = 0u32;
        let mut at = usize::from(merchant.u16(0xc));
        let mut seen = vec![false; offices.len()];
        while at < offices.len() {
            if seen[at] {
                return Err("cadena de oficinas del comerciante cíclica".into());
            }
            seen[at] = true;
            n += 1;
            at = usize::from(offices[at].next_merchant)
        }
        if n > 5 {
            continue;
        }
        let score = if n == 0 { money } else { money / n };
        if score > best_money {
            best_money = score;
            owner = Some(i)
        }
    }
    let Some(owner) = owner else {
        task.due = task.due.wrapping_add(0x1500);
        return Ok(None);
    };
    let mut best = 0u32;
    let mut selected = None;
    for (city, town) in towns.iter().enumerate() {
        if town.u32(0x76c) & 0x100 == 0 {
            continue;
        }
        let mut current = 0u32;
        let mut maximum = 0u32;
        let mut count = 0u32;
        let mut at = usize::from(town.u16(0x784));
        let mut seen = vec![false; offices.len()];
        while at < offices.len() {
            if seen[at] {
                return Err("cadena de oficinas urbanas cíclica".into());
            }
            seen[at] = true;
            let office = offices[at];
            let merchant = merchants
                .get(usize::from(office.owner))
                .ok_or("comerciante de oficina ausente")?;
            if merchant.bytes.len() < 10 {
                return Err("comerciante de oficina truncado".into());
            }
            if merchant.u16(8) != 0 {
                let mut bi = usize::from(office.first_business);
                let mut visited = vec![false; businesses.len()];
                while bi < businesses.len() {
                    if visited[bi] {
                        return Err("negocios cíclicos".into());
                    }
                    visited[bi] = true;
                    let b = businesses[bi];
                    current = current.wrapping_add(u32::from(b.employees));
                    maximum = maximum.wrapping_add(u32::from(b.maximum));
                    bi = usize::from(b.next)
                }
            }
            count += 1;
            at = usize::from(office.next_town)
        }
        if maximum != 0 && current / 7 < maximum / 10 {
            continue;
        }
        let mut score = 0u32;
        for k in 3..21 {
            if town.u16(0x848 + 16 * k) > 1000 {
                score += 1000
            }
        }
        if count != 0 {
            score /= count
        }
        if score > best {
            best = score;
            selected = Some(city)
        }
    }
    let Some(city) = selected else {
        task.due = task.due.wrapping_add(0x2400);
        return Ok(None);
    };
    let owner = u16::try_from(owner).map_err(|_| "comerciante de expansión fuera de rango")?;
    let city = u16::try_from(city).map_err(|_| "ciudad de expansión fuera de rango")?;
    task.due = task
        .due
        .wrapping_add((u32::from(owner) + u32::from(city) + 120).wrapping_mul(72));
    Ok(Some((owner, city)))
}
