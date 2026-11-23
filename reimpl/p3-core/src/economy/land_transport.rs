//! 0x004E38B4: fases del transporte terrestre sobre copias transaccionales.
use crate::{ai::reputation::Merchant, economy::trade::OfficeStock, scheduler::TaskRecord, town::Town};
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Event {
    Letter {
        owner: u8,
        kind: u8,
        town: u8,
        origin: u8,
        destination: u8,
    },
    Notify {
        owner: u8,
        town: u8,
        origin: u8,
        destination: u8,
    },
}
#[derive(Clone)]
pub struct Prepared {
    pub task: TaskRecord,
    pub offices: Vec<OfficeStock>,
    pub merchants: Vec<Merchant>,
    pub events: Vec<Event>,
    pub keep: bool,
}
pub struct Env<'a> {
    pub towns: &'a [Town],
    pub offices: &'a [OfficeStock],
    pub merchants: &'a [Merchant],
    pub town_indices: &'a [u8],
    pub routes: &'a [[u8; 4]; 7],
    pub units: &'a [u8; 24],
    pub scene_active: bool,
}
fn word(t: &TaskRecord, p: usize) -> u16 {
    u16::from_le_bytes([t.u8_at(p), t.u8_at(p + 1)])
}
fn set_word(t: &mut TaskRecord, p: usize, n: u16) {
    let b = n.to_le_bytes();
    t.set_u8_at(p, b[0]);
    t.set_u8_at(p + 1, b[1]);
}
fn lot(unit: u8) -> i32 {
    if unit == 0 {
        2000
    } else if unit == 1 {
        200
    } else {
        10
    }
}
/// `offices` debe seguir el orden de búsqueda original de las listas urbanas.
/// La presentación de escena activa todavía no está conectada.
pub fn prepare(before: &TaskRecord, env: &Env<'_>) -> Result<Prepared, String> {
    if env.scene_active {
        return Err("presentación de transporte terrestre no conectada".into());
    }
    if env.merchants.iter().any(|m| m.bytes.len() != 0x650) {
        return Err("comerciante terrestre truncado".into());
    }
    let mut out = Prepared {
        task: *before,
        offices: env.offices.to_vec(),
        merchants: env.merchants.to_vec(),
        events: vec![],
        keep: true,
    };
    let t = &mut out.task;
    let origin = t.u8_at(8);
    let dest = t.u8_at(9);
    let owner = t.u8_at(10);
    let flags = t.u8_at(11);
    let phase = t.u8_at(0x15);
    let ware = usize::from(t.u8_at(0x14));
    let quantity = word(t, 0x12);
    let find = |city: u8| {
        if usize::from(owner) >= env.merchants.len() || usize::from(city) >= env.towns.len() {
            return None;
        }
        out.offices
            .iter()
            .position(|o| o.town == city && o.owner == u16::from(owner))
    };
    let blocked = |city: u8| {
        env.towns
            .get(usize::from(city))
            .map(|v| v.u8(0x2c8) & 0x10 != 0)
            .ok_or_else(|| "ciudad de transporte ausente".to_string())
    };
    let letter = |events: &mut Vec<Event>, kind, town, from, to| {
        events.push(Event::Letter {
            owner,
            kind,
            town,
            origin: from,
            destination: to,
        })
    };
    match phase {
        0 => {
            let Some(i) = find(origin) else {
                out.keep = false;
                return Ok(out);
            };
            if usize::from(dest) >= env.towns.len() {
                out.keep = false;
                return Ok(out);
            }
            if ware < 24 {
                let stock = out.offices[i].stock[ware];
                let amount = i32::from(quantity) * lot(env.units[ware]);
                if stock >= amount {
                    out.offices[i].stock[ware] = stock.wrapping_sub(amount)
                } else {
                    let factor = lot(env.units[ware]);
                    let divisor = stock / factor;
                    out.offices[i].stock[ware] = stock.wrapping_sub(divisor.wrapping_mul(factor));
                    set_word(t, 0x12, (divisor / 2000) as u16)
                }
                t.set_u32_at(0xc, out.offices[i].average_price[ware].to_bits());
            }
            let merchant = out
                .merchants
                .get_mut(usize::from(owner))
                .ok_or("dueño terrestre ausente")?;
            let fee = i32::from(word(t, 0x10));
            merchant.set_i32(0, merchant.i32(0).wrapping_sub(fee));
            merchant.set_i32(0x4bc, merchant.i32(0x4bc).wrapping_add(fee));
            t.set_u8_at(0x15, 1);
            t.set_u8_at(0x16, 0);
            t.due = t.due.wrapping_add(1);
        }
        1 => {
            if blocked(origin)? {
                if flags & 1 != 0 {
                    letter(&mut out.events, 0x67, origin, origin, dest)
                }
                t.set_u8_at(0x15, 6);
                t.set_u8_at(9, origin)
            } else {
                let old = t.u8_at(0x16);
                t.set_u8_at(0x16, old.wrapping_add(1));
                if old == 0xff {
                    t.set_u8_at(0x15, 2)
                }
            }
            t.due = t.due.wrapping_add(1);
        }
        2 | 4 => {
            let a = *env
                .town_indices
                .get(usize::from(origin))
                .ok_or("índice urbano terrestre ausente")?;
            let b = *env
                .town_indices
                .get(usize::from(dest))
                .ok_or("índice urbano terrestre ausente")?;
            let mut delay = 16u32;
            for r in env.routes {
                if (r[0] == a && r[1] == b) || (r[0] == b && r[1] == a) {
                    delay = 16 * u32::from(r[3]);
                    break;
                }
            }
            t.set_u8_at(0x16, 0xff);
            t.set_u8_at(0x15, phase + 1);
            t.due = t.due.wrapping_add(delay);
        }
        3 | 5 => {
            if !blocked(dest)? {
                if flags > 1 {
                    let reduction = (i32::from(flags >> 1) * i32::from(quantity)) / 100;
                    set_word(t, 0x12, quantity.wrapping_sub(reduction as u16));
                    if flags & 1 != 0 {
                        letter(&mut out.events, 0x68, dest, origin, dest)
                    }
                }
                t.set_u8_at(0x15, 6);
                t.set_u8_at(0x16, 0xff);
                t.due = t.due.wrapping_add(1)
            } else if phase == 3 {
                if flags & 1 != 0 {
                    letter(&mut out.events, 0x67, dest, origin, dest)
                }
                t.set_u8_at(8, dest);
                t.set_u8_at(9, origin);
                t.set_u8_at(0x15, 4);
                t.due = t.due.wrapping_add(0x41)
            } else {
                if flags & 1 != 0 {
                    letter(&mut out.events, 0x69, dest, 0, 0)
                }
                out.keep = false
            }
        }
        6 => {
            let old = t.u8_at(0x16);
            t.set_u8_at(0x16, old.wrapping_sub(1));
            t.due = t.due.wrapping_add(if old == 0 {
                t.set_u8_at(0x15, 7);
                4
            } else {
                1
            })
        }
        7 => {
            if let Some(i) = find(dest) {
                if ware < 24 {
                    let amount = i32::from(quantity) * lot(env.units[ware]);
                    let old = out.offices[i].stock[ware];
                    let new = old.wrapping_add(amount);
                    if new > 0 {
                        let prev = f64::from(old) * f64::from(out.offices[i].average_price[ware]);
                        let incoming = f64::from(amount as f32) * f64::from(t.f32_at(0xc));
                        out.offices[i].stock[ware] = new;
                        out.offices[i].average_price[ware] = ((prev + incoming) / f64::from(new)) as f32
                    } else {
                        out.offices[i].stock[ware] = 0;
                        out.offices[i].average_price[ware] = 0.0
                    }
                }
                if origin != dest {
                    if flags & 1 == 0 {
                        out.keep = false;
                        return Ok(out);
                    }
                    letter(&mut out.events, 0x66, dest, origin, dest)
                }
            }
            if flags & 1 != 0 {
                out.events.push(Event::Notify {
                    owner,
                    town: dest,
                    origin,
                    destination: dest,
                })
            }
            out.keep = false;
        }
        _ => out.keep = false,
    }
    Ok(out)
}
