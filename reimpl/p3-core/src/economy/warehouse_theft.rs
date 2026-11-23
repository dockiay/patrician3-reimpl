//! Resolución de `st_warehouse_theft` (0x004EC934).
use crate::{ai::reputation::Merchant, scheduler::TaskRecord, town::Town};
const UNITS: [u8; 24] = [0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1];
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TheftOutcome {
    pub office: usize,
    pub owner: u8,
    pub city: u8,
    pub payload: Option<[u8; 8]>,
}
/// Se trabaja sobre copias; si no existe oficina no se altera ninguna entrada.
pub fn resolve(
    task: &mut TaskRecord,
    towns: &mut [Town],
    offices: &mut [u8],
    merchants: &[Merchant],
) -> Result<Option<TheftOutcome>, &'static str> {
    if task.kind != 0x33 || offices.len() % 0x44c != 0 {
        return Err("tarea o tabla de robo inválida");
    };
    let owner = task.u8_at(0xe);
    let city = task.u8_at(0xf);
    let Some(town) = towns.get(usize::from(city)) else {
        return Ok(None);
    };
    let Some(merchant) = merchants.get(usize::from(owner)) else {
        return Ok(None);
    };
    if merchant.bytes.len() < 0x1e {
        return Err("comerciante de robo truncado");
    };
    let count = offices.len() / 0x44c;
    let mut at = usize::from(town.u16(0x784));
    let mut seen = vec![false; count];
    let mut found = None;
    while at < count {
        if seen[at] {
            return Err("lista de oficinas de robo cíclica");
        };
        seen[at] = true;
        let o = &offices[at * 0x44c..(at + 1) * 0x44c];
        if u16::from_le_bytes([o[0x2c4], o[0x2c5]]) == u16::from(owner) {
            found = Some(at);
            break;
        }
        at = usize::from(u16::from_le_bytes([o[0x2ca], o[0x2cb]]));
    }
    let Some(index) = found else { return Ok(None) };
    let mut image = offices[index * 0x44c..(index + 1) * 0x44c].to_vec();
    let mut next = town.clone();
    let mut record = *task;
    let mut payload = [0xffu8; 8];
    payload[6..8].copy_from_slice(&u16::from_be_bytes([merchant.bytes[0x1c], merchant.bytes[0x1d]]).to_le_bytes());
    let mut reported = 0;
    for i in 0..6 {
        if reported >= 3 {
            break;
        }
        let ware = usize::from(record.u8_at(8 + i));
        if ware >= 24 {
            continue;
        }
        let scale = match UNITS[ware] {
            0 => 2000,
            1 => 200,
            _ => 10,
        };
        let quantity = i32::from(record.u8_at(0x10 + i));
        let wanted = quantity * scale;
        let at = 4 + 4 * ware;
        let stock = i32::from_le_bytes(image[at..at + 4].try_into().unwrap());
        if stock < 0 {
            return Err("existencias negativas en robo");
        };
        let removed = if wanted > stock {
            record.set_u8_at(0x10 + i, (stock / scale) as u8);
            stock
        } else {
            wanted
        };
        image[at..at + 4].copy_from_slice(&(stock - removed).to_le_bytes());
        next.set_i32(at, next.i32(at).wrapping_add(removed));
        let actual = record.u8_at(0x10 + i);
        if actual > 0 {
            let word = ((ware as u16) << 8) | u16::from(actual);
            payload[reported * 2..reported * 2 + 2].copy_from_slice(&word.to_le_bytes());
            reported += 1
        }
    }
    offices[index * 0x44c..(index + 1) * 0x44c].copy_from_slice(&image);
    towns[usize::from(city)] = next;
    *task = record;
    Ok(Some(TheftOutcome {
        office: index,
        owner,
        city,
        payload: (reported > 0).then_some(payload),
    }))
}
