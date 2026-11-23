//! Planificación periódica de pedidos de astillero de la IA (tarea 0x2B).
use crate::{ai::reputation::Merchant, pump::Operation, scheduler::TaskRecord, town::Town};
pub trait ShipBuildCost {
    fn possible(&mut self, city: u8, kind: u8, merchant: u16) -> Result<bool, String>;
}

/// Cuenta barcos y, para el anfitrión, consulta el coste antes de emitir `0x2C`.
pub fn plan<H: ShipBuildCost>(
    task: &mut TaskRecord,
    towns: &[Town],
    merchants: &[Merchant],
    ship_next: &[u16],
    network_client: bool,
    cost: &mut H,
) -> Result<Option<Operation>, String> {
    let population = towns.iter().fold(0u32, |sum, t| sum.wrapping_add(t.u32(0x2d4)));
    let mut total = 0u32;
    let mut selected = None;
    let mut fewest = 1000u32;
    for i in (0..merchants.len()).rev() {
        let m = &merchants[i];
        if m.bytes.len() < 0x47a {
            return Err("comerciante de astillero truncado".into());
        }
        let control = m.u16(8);
        let count = if control == 0 || control & 4 != 0 {
            u32::from(m.u16(0x478))
        } else {
            let mut at = usize::from(m.u16(0xe));
            let mut seen = vec![false; ship_next.len()];
            let mut n = 0u32;
            while at < ship_next.len() {
                if seen[at] {
                    return Err("cadena naval cíclica".into());
                }
                seen[at] = true;
                n += 1;
                at = usize::from(ship_next[at])
            }
            if n < fewest {
                fewest = n;
                selected = Some(i)
            }
            n
        };
        total = total.wrapping_add(count)
    }
    let Some(owner) = selected.filter(|_| total.wrapping_mul(400) < population) else {
        task.due = task.due.wrapping_add(0x1500);
        return Ok(None);
    };
    if network_client {
        task.due = task.due.wrapping_add(0x2400);
        return Ok(None);
    }
    let owner = u16::try_from(owner).map_err(|_| "comerciante de astillero fuera de rango")?;
    let m = &merchants[usize::from(owner)];
    let home = m.bytes[0x19];
    let town = towns
        .get(usize::from(home))
        .ok_or("ciudad natal de astillero ausente")?;
    let mut kind = if m.i32(0) < 40000 || town.u32(0x2c8) & 1 != 0 {
        1u8
    } else {
        3
    };
    if (town.u8(0x824 + usize::from(kind)) as i8) < 0 {
        kind -= 1
    }
    let mut city = home;
    loop {
        if cost.possible(city, kind, owner)? {
            task.due = task.due.wrapping_add(0x2400);
            return Ok(Some(Operation([
                0x2c,
                u32::from(kind),
                u32::from(owner),
                u32::from(city),
                1,
            ])));
        }
        city = city.wrapping_add(1);
        if usize::from(city) >= towns.len() {
            city = 0
        }
        if city == home {
            break;
        }
    }
    task.due = task.due.wrapping_add(0x2400);
    Ok(None)
}
