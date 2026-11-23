//! Eventos de capitanes de la actualización decenal `0x004DCEA0`.
use crate::{ai::reputation::Merchant, pump::Operation, scheduler::TaskRecord};

pub struct Context<'a> {
    pub tick: u32,
    pub day_of_year: u16,
    pub network_client: bool,
    pub queued: u16,
    pub merchants: &'a [Merchant],
    pub ships: &'a [u8],
    pub offices: &'a [u8],
}
/// Fronteras del original; la mejora directa puede modificar `captains`.
pub trait Host {
    fn random(&mut self) -> Result<u32, String>;
    fn schedule_retirement(&mut self, due: u32, ship: u16, captain: u16) -> Result<(), String>;
    fn direct_skill(&mut self, op: Operation, captains: &mut [u8]) -> Result<(), String>;
    fn enqueue(&mut self, op: Operation) -> Result<(), String>;
}
fn u16_at(b: &[u8], o: usize) -> u16 {
    u16::from_le_bytes([b[o], b[o + 1]])
}
fn u32_at(b: &[u8], o: usize) -> u32 {
    u32::from_le_bytes(b[o..o + 4].try_into().unwrap())
}

/// Recorre las listas en el orden original. El anfitrión debe ofrecer efectos reales
/// para las operaciones 0x12, 0x13, 0x67 y la tarea 0x27 antes de conectar el reloj.
pub fn update<H: Host>(
    task: &mut TaskRecord,
    captains: &mut [u8],
    ctx: &Context<'_>,
    host: &mut H,
) -> Result<(), String> {
    if ctx.ships.len() % 0x180 != 0 || captains.len() % 16 != 0 || ctx.offices.len() % 0x44c != 0 {
        return Err("tablas de capitanes truncadas".into());
    }
    let ships = ctx.ships.len() / 0x180;
    let caps = captains.len() / 16;
    let offices = ctx.offices.len() / 0x44c;
    if ships > u16::MAX as usize
        || caps > u16::MAX as usize
        || offices > u16::MAX as usize
        || ctx.merchants.len() > u16::MAX as usize
    {
        return Err("tablas de capitanes demasiado grandes".into());
    }
    if ctx.merchants.iter().any(|m| m.bytes.len() < 0x10) {
        return Err("comerciante truncado".into());
    }
    let phase = if ctx.day_of_year < 10 {
        0
    } else {
        task.u32_at(0x10).wrapping_add(1)
    };
    task.set_u32_at(0x10, phase);
    if phase > 31 {
        return Ok(());
    }
    let cycle = (phase & 7) as u8;
    let threshold = [250u8, 200, 250, 150][usize::from(cycle & 3)];
    let mut budget = 52i32 - i32::from(ctx.queued);
    let mut op = Operation([u32::MAX, 0, 0, 0, 0]);
    for owner in (0..ctx.merchants.len()).rev() {
        let merchant = &ctx.merchants[owner];
        let human = merchant.u16(8) == 0;
        let mut ship = merchant.u16(0xe);
        let mut seen = vec![false; ships];
        while usize::from(ship) < ships {
            let si = usize::from(ship);
            if seen[si] {
                return Err("lista de barcos cíclica".into());
            }
            seen[si] = true;
            let image = &ctx.ships[si * 0x180..(si + 1) * 0x180];
            let captain = u16_at(image, 0x42);
            let ci = usize::from(captain);
            if ci < caps && u16_at(image, 0x134) != 0x11 {
                let at = ci * 16;
                let hired = u32_at(captains, at + 4);
                if hired < ctx.tick.wrapping_sub(0x474a00) && captains[at + 0xe] == 0 {
                    if !human {
                        host.schedule_retirement(ctx.tick.wrapping_add(0x20), ship, captain)?;
                        captains[at + 0xe] = 1;
                    } else if !ctx.network_client {
                        let age = ctx.tick.wrapping_sub(hired) >> 13;
                        let roll = host.random()? & 0x3ff;
                        if roll.wrapping_mul(age) & 0xfffffc00 > 0x93000 {
                            op.0[0] = 0x13;
                            op.0[1] = u32::from(ship);
                            op.0[2] = u32::from(captain);
                            let can = budget > 0;
                            budget -= 1;
                            if can {
                                host.enqueue(op)?;
                            }
                        }
                    }
                }
                if (captain as u8 & 7) == cycle {
                    op.0[0] = 0x12;
                    op.0[1] = u32::from(ship);
                    if !human {
                        op.0[2] = 8;
                        op.0[3] = 8;
                        host.direct_skill(op, captains)?;
                    } else if !ctx.network_client
                        && [captains[at + 9], captains[at + 10], captains[at + 11]]
                            .iter()
                            .any(|&skill| skill < threshold)
                    {
                        op.0[2] = 0;
                        op.0[3] = 0;
                        let choice = host.random()? & 0x3ff;
                        let amount = host.random()? % 0x33;
                        let skill = if choice <= 0x155 {
                            op.0[2] = amount;
                            captains[at + 9]
                        } else if choice < 0x2aa {
                            op.0[3] = amount;
                            captains[at + 10]
                        } else {
                            op.0[3] = amount;
                            captains[at + 11]
                        };
                        if skill < threshold {
                            let can = budget > 0;
                            budget -= 1;
                            if can {
                                host.enqueue(op)?;
                            }
                        }
                    }
                }
            }
            ship = u16_at(image, 4);
        }
        if human && !ctx.network_client {
            let mut office = merchant.u16(0xc);
            let mut seen = vec![false; offices];
            while usize::from(office) < offices {
                let oi = usize::from(office);
                if seen[oi] {
                    return Err("lista de oficinas cíclica".into());
                }
                seen[oi] = true;
                let image = &ctx.offices[oi * 0x44c..(oi + 1) * 0x44c];
                let cap = u16_at(image, 0x2f2);
                let ci = usize::from(cap);
                if ci < caps && (cap as u8 & 7) == cycle && host.random()? & 0x3ff < 0x1b3 && budget > 0 {
                    let old = captains[ci * 16 + 0xa];
                    let new = old.wrapping_add(0x2b);
                    op.0[4] = u32::from(new);
                    if new > old {
                        op.0 = [
                            0x67,
                            u32::from(cap),
                            owner as u32,
                            u32::from(image[0x2c6]),
                            u32::from(new),
                        ];
                        host.enqueue(op)?;
                        budget -= 1;
                    }
                }
                office = u16_at(image, 0x2c8);
            }
        }
    }
    Ok(())
}
