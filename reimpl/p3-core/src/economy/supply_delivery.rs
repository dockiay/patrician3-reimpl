//! Entrega de mercancías para el encargo urbano (operación 0x51).
use crate::{
    ai::{reputation::Merchant, settlement::RoadDeliveryShip},
    pump::Operation,
    scheduler::Scheduler,
    town::Town,
};
const BARRELS: [bool; 20] = [
    false, false, false, true, true, true, true, true, true, true, true, false, true, true, false, true, false, false,
    true, false,
];

/// Opera sobre copias para rechazar cadenas inválidas sin efectos parciales.
pub fn deliver(
    op: &mut Operation,
    scheduler: &mut Scheduler,
    towns: &mut [Town],
    merchant: &mut Merchant,
    office: Option<&mut [i32; 24]>,
    ships: &mut [RoadDeliveryShip],
    head: u16,
) -> Result<Vec<usize>, String> {
    let index = op.0[3] as usize;
    if op.0[0] != 0x51 {
        return Err("operación de entrega inválida".into());
    }
    let Some(original) = scheduler
        .records
        .get(index)
        .filter(|_| index < usize::from(scheduler.capacity))
    else {
        return Ok(Vec::new());
    };
    let city = usize::from(original.u8_at(8));
    let good = usize::from(original.u8_at(9));
    if original.kind != 0x11 || city >= towns.len() || good >= 20 || u32::from(original.u8_at(0x16)) != op.0[1] {
        return Ok(Vec::new());
    }
    if merchant.bytes.len() < 0x4a0 {
        return Err("comerciante de encargo truncado".into());
    }
    let mut task = *original;
    let mut quantity = op.0[4].min(u32::from(u16::from_le_bytes([task.u8_at(0x10), task.u8_at(0x11)])));
    let mut person = merchant.clone();
    let mut staged = ships.to_vec();
    let mut stock = office.as_deref().copied();
    let mut changed = Vec::new();
    let mut at = usize::from(head);
    let mut seen = vec![false; ships.len()];
    let unit = if BARRELS[good] { 200i32 } else { 2000 };
    while quantity > 0 && at < staged.len() {
        if seen[at] {
            return Err("cadena de barcos cíclica".into());
        }
        seen[at] = true;
        let next = usize::from(staged[at].next);
        if usize::from(staged[at].town) == city && staged[at].stock[good] > 0 {
            let amount = quantity.min((staged[at].stock[good] / unit) as u32);
            staged[at].stock[good] -= (amount as i32) * unit;
            quantity -= amount;
            let remaining = u16::from_le_bytes([task.u8_at(0x10), task.u8_at(0x11)]).wrapping_sub(amount as u16);
            task.set_u8_at(0x10, remaining as u8);
            task.set_u8_at(0x11, (remaining >> 8) as u8);
            if task.u8_at(0x17) == 0 {
                let price = u16::from_le_bytes([task.u8_at(0x14), task.u8_at(0x15)]);
                let pay = amount.wrapping_mul(u32::from(price));
                person.set_i32(0, (person.i32(0) as u32).wrapping_add(pay) as i32);
                person.set_i32(0x49c, (person.i32(0x49c) as u32).wrapping_add(pay) as i32)
            }
            changed.push(at);
        }
        at = next;
    }
    if quantity > 0 {
        if let Some(ref mut stock) = stock {
            if stock[good] < 0 {
                return Err("existencias negativas en oficina".into());
            }
            let amount = quantity.min((stock[good] / unit) as u32);
            stock[good] -= (amount as i32) * unit;
            let remaining = u16::from_le_bytes([task.u8_at(0x10), task.u8_at(0x11)]).wrapping_sub(amount as u16);
            task.set_u8_at(0x10, remaining as u8);
            task.set_u8_at(0x11, (remaining >> 8) as u8);
            if task.u8_at(0x17) == 0 {
                let price = u16::from_le_bytes([task.u8_at(0x14), task.u8_at(0x15)]);
                let pay = amount.wrapping_mul(u32::from(price));
                person.set_i32(0, (person.i32(0) as u32).wrapping_add(pay) as i32);
                person.set_i32(0x49c, (person.i32(0x49c) as u32).wrapping_add(pay) as i32)
            }
            quantity = 0;
        } else {
            op.0[4] = quantity;
            scheduler.records[index] = task;
            *merchant = person;
            ships.clone_from_slice(&staged);
            return Ok(changed);
        }
    }
    if task.u8_at(0x10) == 0 && task.u8_at(0x11) == 0 {
        for offset in [0x830, 0x832] {
            if usize::from(towns[city].u16(offset)) == index {
                towns[city].set_u16(offset, u16::MAX)
            }
        }
    }
    op.0[4] = quantity;
    scheduler.records[index] = task;
    *merchant = person;
    ships.clone_from_slice(&staged);
    if let (Some(out), Some(stock)) = (office, stock) {
        *out = stock
    }
    Ok(changed)
}
