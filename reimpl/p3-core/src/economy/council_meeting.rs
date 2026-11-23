//! Reunión programada del concejo (`0x004E9A94`).
use crate::{ai::reputation::Merchant, scheduler::Scheduler, town::Town};

/// Fronteras de presentación, cartas y plano; el orden sigue el original.
pub trait MeetingHost {
    fn notify_voter(&mut self, merchant: u16, town: u8);
    fn free_space_percent(&mut self, town: &Town) -> u32;
    fn point_valid(&mut self, town: &Town, point: u32) -> bool;
    fn place_wall(&mut self, town: &mut Town, point: u32);
    fn decision_letter(&mut self, merchant: u8, town: u8, payload_a: u32, payload_b: u32);
}

/// Ejecuta el registro de cabeza; devuelve 1 para reprogramarlo o 0 para retirarlo.
pub fn run(
    scheduler: &mut Scheduler,
    towns: &mut [Town],
    merchants: &[Merchant],
    tick: u32,
    host: &mut dyn MeetingHost,
) -> u32 {
    let index = scheduler.head as usize;
    if index >= scheduler.records.len() {
        return 0;
    }
    let mut r = scheduler.records[index];
    let city = r.u8_at(0xc) as usize;
    if city >= towns.len() {
        return 0;
    }
    let town = &mut towns[city];
    let low = tick & 255;
    let due;
    if low == 0 {
        for o in [0x10, 0x11, 0x12, 0xe, 0x13] {
            r.set_u8_at(o, 0)
        }
        r.set_u8_at(0xf, 22);
        for j in 0..4 {
            if usize::from(town.u8(0x6dc + j)) < merchants.len() {
                r.set_u8_at(0xf, r.u8_at(0xf).wrapping_sub(1));
            }
        }
        let mut human = 0u8;
        for (i, m) in merchants.iter().enumerate() {
            if m.bytes.len() < 0x39c + city + 1 {
                continue;
            }
            if u16::from_le_bytes([m.bytes[8], m.bytes[9]]) == 0 {
                let rank = m.bytes[0x39c + city];
                if rank > 3 {
                    host.notify_voter(i as u16, city as u8);
                    let loss = if rank == 4 {
                        1
                    } else if rank == 5 {
                        2
                    } else {
                        3
                    };
                    r.set_u8_at(0xf, r.u8_at(0xf).wrapping_sub(loss));
                    r.set_u8_at(0x13, r.u8_at(0x13) | ((1u32.wrapping_shl(u32::from(human) & 31)) as u8));
                }
                human = human.wrapping_add(1);
            }
        }
        if (r.u8_at(0xf) as i8) < 4 {
            r.set_u8_at(0xf, 4)
        }
        let votes = i32::from(r.u8_at(0xf) as i8);
        let yes = match r.u8_at(0xd) {
            0 => {
                let mut amount = r.u32_at(8);
                if tick < (town.u32(0x6f8) & 0xffff_ff00).wrapping_add(0x5400) {
                    amount = amount.wrapping_add(town.u32(0x6f4));
                }
                let f = (town.i32(0x2d4) as f64).sqrt() * 100.0;
                let baseline = if !f.is_finite() || f >= 2147483648.0 || f < -2147483648.0 {
                    i32::MIN
                } else {
                    f.trunc() as i32
                };
                let pct = (90i32.wrapping_sub((amount.wrapping_sub(baseline as u32) as i32) / 1500)).clamp(10, 90);
                votes * pct / 100
            }
            1 => {
                if r.u8_at(0x15) != 0 {
                    votes * 9 / 10
                } else if town.u16(0x766) < town.u16(0x776) {
                    votes >> 4
                } else if town.u16(0x78c) < town.u16(0x78e) {
                    votes * 4 / 5
                } else {
                    let f = host.free_space_percent(town);
                    ((100u32.wrapping_sub(f)).wrapping_mul(votes as u32) / 100) as i32
                }
            }
            2 => {
                let mut p = town.i32(0x2d4);
                if town.flags() & 0x10 != 0 {
                    p = p.wrapping_mul(2)
                }
                let v = p.wrapping_sub(i32::from(town.u16(0x9a0)) * 203).wrapping_add(512);
                if v < 100 {
                    votes / 10
                } else if v > 920 {
                    votes * 9 / 10
                } else {
                    votes.wrapping_mul(v) >> 10
                }
            }
            3 => {
                let new = i32::from(r.u8_at(0x14));
                let old = i32::from(town.u8(0x6f8));
                if new < old {
                    votes * 9 / 10
                } else {
                    votes
                        .wrapping_mul(old - new + 50)
                        .wrapping_mul(25 - new)
                        .wrapping_mul(10)
                        / 10000
                }
            }
            _ => {
                scheduler.records[index] = r;
                return 0;
            }
        };
        r.set_u8_at(0xe, yes as u8);
        r.set_u8_at(0xf, (votes as u8).wrapping_sub(yes as u8));
        due = r.due.wrapping_add(4);
    } else {
        let no = r.u8_at(0xf) as i8;
        let yes = r.u8_at(0xe) as i8;
        if no == 0 {
            if yes > 0 {
                r.set_u8_at(0xe, (yes as u8).wrapping_sub(1));
                r.set_u8_at(0x10, r.u8_at(0x10).wrapping_add(1));
            }
        } else if yes == 0
            || i32::from(no) * 3 + i32::from(r.u8_at(0x11)) >= i32::from(yes) * 3 + i32::from(r.u8_at(0x10))
        {
            r.set_u8_at(0xf, (no as u8).wrapping_sub(1));
            r.set_u8_at(0x11, r.u8_at(0x11).wrapping_add(1));
        } else {
            r.set_u8_at(0xe, (yes as u8).wrapping_sub(1));
            r.set_u8_at(0x10, r.u8_at(0x10).wrapping_add(1));
        }
        due = r.due.wrapping_add(if low < 0xdc { 4 } else { 1 });
    }
    r.due = due;
    scheduler.records[index] = r;
    if low < (due & 255) {
        return 1;
    }
    let mayor = town.u8(0x6f1);
    let proposer = r.u8_at(0x16);
    let a = if r.u8_at(0xd) == 3 {
        u32::from(town.u8(0x6f8)) * 256 + u32::from(r.u8_at(0x14))
    } else {
        r.u32_at(8)
    };
    let b = (u32::from(r.u8_at(0xd)) << 16) | (u32::from(r.u8_at(0x10)) << 8) | u32::from(r.u8_at(0x11));
    if (mayor as usize) < merchants.len() {
        host.decision_letter(mayor, city as u8, a, b)
    }
    if (proposer as usize) < merchants.len() && proposer != mayor {
        host.decision_letter(proposer, city as u8, a, b)
    }
    if r.u8_at(0x11) < r.u8_at(0x10) {
        match r.u8_at(0xd) {
            0 => {
                scheduler.dispatching = 0;
                let next = scheduler.add_task(r.due.wrapping_add(0x100));
                scheduler.dispatching = 1;
                scheduler.records[next as usize].kind = 0x31;
                // El original vuelve a leer la cabeza después de insertar.
                let source = scheduler.records[scheduler.head as usize];
                let task = &mut scheduler.records[next as usize];
                task.set_u32_at(8, source.u32_at(8));
                task.set_u8_at(0xc, source.u8_at(0xc));
            }
            1 => {
                if host.point_valid(town, 1) {
                    host.place_wall(town, 1)
                } else if host.point_valid(town, 2) {
                    host.place_wall(town, 2)
                }
            }
            2 => {
                let n = town.u16(0x9a0);
                if n < 0x8000 {
                    town.set_u16(0x9a0, n.wrapping_add(10))
                }
            }
            3 => {
                let n = r.u8_at(0x14);
                town.set_u8(0x6f8, if n > 20 { 10 } else { n })
            }
            _ => {}
        }
    }
    0
}
