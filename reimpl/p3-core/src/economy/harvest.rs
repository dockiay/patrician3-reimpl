//! Ciclo programado de cosecha `0x004E2824`.
use crate::{scheduler::TaskRecord, town::Town};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HarvestNotice {
    pub recipient: i32,
    pub record: [u8; 16],
}

/// Prepara la fase sin alterar las entradas si encuentra datos incompletos.
pub fn advance(
    task: &mut TaskRecord,
    towns: &mut [Town],
    offices: &[Vec<u8>],
    day_of_year: u16,
) -> Result<(bool, Vec<HarvestNotice>), String> {
    let city = usize::from(u16::from_le_bytes([task.u8_at(0x10), task.u8_at(0x11)]));
    let Some(original) = towns.get(city) else {
        return Ok((false, Vec::new()));
    };
    let mut town = original.clone();
    let mut staged = *task;
    let phase = staged.u32_at(8);
    let mut notices = Vec::new();
    let mut notice = [0u8; 16];
    notice[5] = city as u8;
    notice[8..10].copy_from_slice(&u16::from(staged.u8_at(0x14)).to_le_bytes());
    match phase {
        0 => {
            let flags = town.u32(0x2c8);
            town.set_u32(
                0x2c8,
                if staged.u8_at(0x14) == 0 {
                    flags & !0x4000 | 0x2000
                } else {
                    flags & !0x2000 | 0x4000
                },
            );
            staged.set_u32_at(8, 1);
            staged.due = staged.due.wrapping_add(0x700)
        }
        1 | 4 => {
            notice[4] = 8;
            notice[10..12].copy_from_slice(&u16::from(phase == 1).to_le_bytes());
            let mut index = usize::from(town.u16(0x784));
            let mut seen = vec![false; offices.len()];
            while index < offices.len() {
                if seen[index] {
                    return Err("cadena de oficinas de cosecha cíclica".into());
                }
                seen[index] = true;
                let office = &offices[index];
                if office.len() < 0x2cc {
                    return Err("oficina de cosecha truncada".into());
                }
                let positive = [0xd8, 0xe0, 0x108, 0xc4]
                    .into_iter()
                    .any(|at| i32::from_le_bytes(office[at..at + 4].try_into().unwrap()) > 0);
                if positive {
                    notices.push(HarvestNotice {
                        recipient: i32::from(u16::from_le_bytes([office[0x2c4], office[0x2c5]])),
                        record: notice,
                    })
                }
                index = usize::from(u16::from_le_bytes([office[0x2ca], office[0x2cb]]))
            }
            staged.set_u32_at(8, phase + 1);
            staged.due = staged.due.wrapping_add(0x700)
        }
        2 => {
            notice[4] = 9;
            notice[10..12].copy_from_slice(&1u16.to_le_bytes());
            notices.push(HarvestNotice {
                recipient: -2,
                record: notice,
            });
            let day = u32::from(day_of_year) << 8;
            let mut end = staged.u32_at(0xc).wrapping_add(day);
            if end > 0x8e00 && end < 0x10900 {
                let n = (0x108ff - end) / 0x3c7b + 1;
                end = end.wrapping_add(n * 0x3c7b)
            }
            staged.due = staged.due.wrapping_add(end.wrapping_sub(day));
            staged.set_u32_at(8, 3)
        }
        3 => {
            town.set_u32(0x2c8, town.u32(0x2c8) & !0x6000);
            staged.set_u32_at(8, 4);
            staged.due = staged.due.wrapping_add(0x700)
        }
        5 => {
            town.set_u32(0x2c8, town.u32(0x2c8) & !0x6000);
            notice[4] = 9;
            notices.push(HarvestNotice {
                recipient: -2,
                record: notice,
            });
            let status = u16::from_le_bytes([staged.u8_at(0x12), staged.u8_at(0x13)]) as i16;
            if status == 0 {
                *task = staged;
                towns[city] = town;
                return Ok((false, notices));
            }
            if status < 0 {
                staged.set_u8_at(0x14, u8::from(staged.u8_at(0x14) == 0))
            }
            staged.set_u8_at(0x12, 0);
            staged.set_u8_at(0x13, 0);
            staged.set_u32_at(8, 0);
            staged.due = staged
                .due
                .wrapping_add((0x1f5u32.wrapping_sub(u32::from(day_of_year))) << 8)
        }
        _ => {
            town.set_u32(0x2c8, town.u32(0x2c8) & !0x6000);
            *task = staged;
            towns[city] = town;
            return Ok((false, notices));
        }
    }
    *task = staged;
    towns[city] = town;
    Ok((true, notices))
}
