//! 004E5664: resolution of a commissioned tavern burglary.
use crate::{ai::reputation::Merchant, rng::LcgRandom, scheduler::Scheduler, town::Town};
const UNITS: [u8; 16] = [0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1];
#[derive(Clone, Debug)]
pub struct Notice {
    pub recipient: u8,
    pub record: [u8; 16],
}
pub struct Prepared {
    pub scheduler: Scheduler,
    pub towns: Vec<Town>,
    pub offices: Vec<Vec<u8>>,
    pub merchants: Vec<Merchant>,
    pub notices: Vec<Notice>,
}
fn word(b: &[u8], i: usize) -> u16 {
    u16::from_le_bytes([b[i], b[i + 1]])
}
fn set16(b: &mut [u8], i: usize, v: u16) {
    b[i..i + 2].copy_from_slice(&v.to_le_bytes())
}
fn set32(b: &mut [u8], i: usize, v: i32) {
    b[i..i + 4].copy_from_slice(&v.to_le_bytes())
}
pub fn prepare(
    q: &Scheduler,
    towns: &[Town],
    offices: &[Vec<u8>],
    merchants: &[Merchant],
    merchant_count: u16,
    tick: u32,
) -> Result<Prepared, String> {
    let mut out = Prepared {
        scheduler: q.clone(),
        towns: towns.to_vec(),
        offices: offices.to_vec(),
        merchants: merchants.to_vec(),
        notices: vec![],
    };
    let head = usize::from(q.head);
    let task = *q.records.get(head).ok_or("tarea de robo ausente")?;
    if task.kind != 0xf {
        return Err("tipo de robo incorrecto".into());
    }
    if usize::from(merchant_count) > merchants.len()
        || merchants
            .iter()
            .take(usize::from(merchant_count))
            .any(|m| m.bytes.len() != 0x650)
        || offices.iter().any(|o| o.len() != 0x44c)
    {
        return Err("vistas de robo truncadas".into());
    }
    let city = usize::from(task.u8_at(0xb));
    let victim = usize::from(task.u8_at(0xc));
    let mut sponsor = usize::from(task.u8_at(0xa));
    if city >= towns.len() || victim >= usize::from(merchant_count) || victim == sponsor {
        return Ok(out);
    }
    out.towns[city].set_u8(0x83c, 0);
    let mut pos = usize::from(towns[city].u16(0x784));
    let mut seen = vec![false; offices.len()];
    let mut found = None;
    while pos < offices.len() {
        if seen[pos] {
            return Err("lista de oficinas de robo ciclica".into());
        }
        seen[pos] = true;
        let o = &offices[pos];
        if usize::from(word(o, 0x2c4)) == victim {
            found = Some(pos);
            break;
        }
        pos = usize::from(word(o, 0x2ca));
    }
    let strength = u32::from(u16::from_le_bytes([task.u8_at(8), task.u8_at(9)]));
    let score = if let Some(i) = found {
        let o = &offices[i];
        let mut n = (((i32::from(word(o, 0x2ec)) - 50) / 200 + 1) as i16 as i32) as u32;
        if merchants[victim].u16(8) != 0 && n < 2 {
            n = 2
        }
        let div = n.wrapping_add(u32::from(o[0x2c7]));
        if div == 0 {
            return Err("division por cero en robo original".into());
        }
        (strength & 0x3ff).wrapping_mul(n.wrapping_sub(1)) / div
    } else {
        0x19a
    };
    let mut letter = [0u8; 16];
    letter[1] = 0x80;
    letter[2..4].fill(255);
    letter[5] = city as u8;
    letter[6..8].fill(255);
    if sponsor < usize::from(merchant_count) && score < 0x67 {
        let v = &merchants[victim];
        let p = &merchants[sponsor];
        let vh = usize::from(v.bytes[0x19]);
        let sh = usize::from(p.bytes[0x19]);
        let old_dispatch = out.scheduler.dispatching;
        out.scheduler.dispatching = 0;
        let id = out
            .scheduler
            .add_task(task.due.wrapping_add(score.wrapping_add(0x20).wrapping_mul(0x10)));
        out.scheduler.dispatching = old_dispatch;
        let r = &mut out.scheduler.records[usize::from(id)];
        r.kind = 5;
        r.set_u8_at(8, sponsor as u8);
        r.set_u8_at(9, city as u8);
        r.set_u8_at(10, victim as u8);
        r.set_u8_at(11, p.bytes[0x19]);
        r.set_u32_at(12, task.due);
        r.set_u8_at(16, 3);
        r.set_u8_at(17, 0);
        let a = (strength as u16)
            .wrapping_mul(256)
            .wrapping_add(u16::from(task.u8_at(9)));
        r.set_u8_at(18, a as u8);
        r.set_u8_at(19, (a >> 8) as u8);
        let b = (strength as u16).wrapping_mul(task.due as u16);
        r.set_u8_at(20, b as u8);
        r.set_u8_at(21, (b >> 8) as u8);
        r.set_u8_at(22, 255);
        r.set_u8_at(23, 255);
        letter[4] = 0x74;
        set16(&mut letter, 8, u16::from_be_bytes([v.bytes[0x1c], v.bytes[0x1d]]));
        set16(
            &mut letter,
            10,
            u16::from(v.bytes[0x39c + vh]) | if offices[found.unwrap()][0x2c7] != 0 { 0x100 } else { 0 },
        );
        set16(&mut letter, 12, u16::from_be_bytes([p.bytes[0x1c], p.bytes[0x1d]]));
        set16(
            &mut letter,
            14,
            u16::from(p.bytes[0x39c + sh]) | (u16::from(p.get_merchant_epithet_index()) << 8),
        );
        out.notices.push(Notice {
            recipient: victim as u8,
            record: letter,
        });
        return Ok(out);
    }
    if sponsor >= usize::from(merchant_count) {
        sponsor = 0;
        out.scheduler.records[head].set_u8_at(0xa, 0)
    }
    if score > 0x19a {
        let v = &merchants[victim];
        set16(&mut letter, 8, u16::from_be_bytes([v.bytes[0x1c], v.bytes[0x1d]]));
        let money = v.i32(0);
        let mut stolen = 0i32;
        if money >= 101 {
            let percentage = ((score & 7) + 3).min(8);
            stolen = (money.wrapping_mul(percentage as i32) / 100).min(50000);
            out.merchants[victim].set_i32(0, money.wrapping_sub(stolen));
            let spent = out.merchants[victim].i32(0x4bc);
            out.merchants[victim].set_i32(0x4bc, spent.wrapping_add(stolen));
            let gain = stolen >> 1;
            let m = &mut out.merchants[sponsor];
            m.set_i32(0, m.i32(0).wrapping_add(gain));
            m.set_i32(0x4b8, m.i32(0x4b8).wrapping_add(gain))
        }
        set16(&mut letter, 14, stolen as u16);
        let o = &mut out.offices[found.ok_or("oficina de robo ausente")?];
        let mut random = LcgRandom::seeded(tick).next_u32();
        for i in 0..16 {
            let at = 4 + 4 * i;
            let stock = i32::from_le_bytes(o[at..at + 4].try_into().unwrap());
            if stock >= 200 {
                let mut amount = ((random & 31) as i32 + 15).wrapping_mul(stock / 100);
                amount = if UNITS[i] == 0 {
                    (amount / 2000) * 10
                } else {
                    amount / 200
                };
                amount = amount.min(1000);
                if amount > 0 {
                    set32(o, at, stock.wrapping_sub(amount.wrapping_mul(200)));
                    let bits = word(&letter, 10) | (1u16 << i);
                    let sum = word(&letter, 12).wrapping_add(amount as u16);
                    set16(&mut letter, 10, bits);
                    set16(&mut letter, 12, sum)
                }
                random >>= 2
            }
        }
        letter[4] = 0x20;
        out.notices.push(Notice {
            recipient: victim as u8,
            record: letter,
        });
        letter[4] = 0x15;
        set16(&mut letter, 14, (stolen as u16) >> 1);
        out.notices.push(Notice {
            recipient: sponsor as u8,
            record: letter,
        });
        return Ok(out);
    }
    letter[4] = 0x16;
    set16(&mut letter, 8, u16::from(merchants[victim].bytes[0x1c]));
    set16(&mut letter, 10, u16::from(merchants[victim].bytes[0x1d]));
    out.notices.push(Notice {
        recipient: sponsor as u8,
        record: letter,
    });
    Ok(out)
}
