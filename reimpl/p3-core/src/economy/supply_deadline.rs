//! Vencimiento y distribución del encargo urbano `0x004E6744`.
use crate::{ai::reputation::Merchant, economy::prices::SMALL_LOT_WARES, scheduler::TaskRecord, town::Town};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Notice {
    pub recipient: usize,
    pub record: [u8; 16],
}
fn word(t: &TaskRecord, at: usize) -> u16 {
    u16::from_le_bytes([t.u8_at(at), t.u8_at(at + 1)])
}
fn set_word(t: &mut TaskRecord, at: usize, value: u16) {
    let b = value.to_le_bytes();
    t.set_u8_at(at, b[0]);
    t.set_u8_at(at + 1, b[1]);
}

/// El resultado se publica solo después de validar mercancía y comerciante.
pub fn advance(
    task: &mut TaskRecord,
    head: u16,
    towns: &mut [Town],
    merchants: &mut [Merchant],
) -> Result<(bool, Option<Notice>), String> {
    let city = usize::from(task.u8_at(8));
    let Some(original) = towns.get(city) else {
        return Ok((false, None));
    };
    let total = word(task, 0x12);
    if total == 0 {
        return Ok((false, None));
    }
    let ware = usize::from(task.u8_at(9));
    let small = *SMALL_LOT_WARES
        .get(ware)
        .ok_or("mercancía del encargo fuera de rango")?;
    let mut town = original.clone();
    let mut staged = *task;
    for at in [0x830, 0x832] {
        if town.u16(at) == head {
            town.set_u16(at, u16::MAX)
        }
    }
    let owner = usize::from(staged.u8_at(0x16));
    let mut updated = None;
    let mut notice = None;
    if let Some(original) = merchants.get(owner) {
        let mut person = original.clone();
        if person.bytes.len() < 0x4cc {
            return Err("comerciante de encargo truncado".into());
        }
        let remaining = word(&staged, 0x10);
        let mode = staged.u8_at(0x17);
        let reputation = person.u16(0x22) as i16;
        let human = person.u16(8) == 0;
        if remaining > 0 {
            if mode != 0 {
                if human {
                    let mut record = [0u8; 16];
                    record[4] = 0x5b;
                    record[5] = city as u8;
                    record[8..10].copy_from_slice(&total.to_le_bytes());
                    record[10..12].copy_from_slice(&(ware as u16).to_le_bytes());
                    record[12] = staged.u8_at(0xb);
                    record[13] = staged.u8_at(0xa);
                    record[14] = person.bytes[0x1d];
                    record[15] = person.bytes[0x1c];
                    notice = Some(Notice {
                        recipient: owner,
                        record,
                    })
                }
                if reputation > -30000 {
                    person.set_u16(0x22, reputation.wrapping_sub(0x42) as u16)
                }
            } else if reputation < 30000 {
                let fraction = (i32::from(total) - i32::from(remaining)) * 61 / i32::from(total);
                person.set_u16(0x22, (i32::from(reputation) + fraction) as u16)
            }
            set_word(&mut staged, 0x10, 0);
        } else {
            if reputation < 30000 {
                person.set_u16(0x22, reputation.wrapping_add(0x3d) as u16)
            }
            if mode != 0 {
                let price = word(&staged, 0x14);
                let payment = u32::from(price) * u32::from(total);
                person.set_i32(0, (person.i32(0) as u32).wrapping_add(payment) as i32);
                person.set_i32(0x4c8, (person.i32(0x4c8) as u32).wrapping_add(payment) as i32);
                if human {
                    let mut record = [0u8; 16];
                    record[4] = 0x5a;
                    record[5] = city as u8;
                    record[8..10].copy_from_slice(&price.to_le_bytes());
                    record[10..12].copy_from_slice(&(ware as u16).to_le_bytes());
                    record[12..14].copy_from_slice(&total.to_le_bytes());
                    record[14] = person.bytes[0x1d];
                    record[15] = person.bytes[0x1c];
                    notice = Some(Notice {
                        recipient: owner,
                        record,
                    })
                }
            }
        }
        staged.set_u8_at(0x16, 0xff);
        updated = Some(person);
    }
    let at = 4 + ware * 4;
    town.set_u32(at, town.u32(at).wrapping_add(if small { 200 } else { 2000 }));
    staged.due = staged.due.wrapping_add(0x100);
    set_word(&mut staged, 0x12, total - 1);
    *task = staged;
    towns[city] = town;
    if let Some(person) = updated {
        merchants[owner] = person
    }
    Ok((total > 1, notice))
}
