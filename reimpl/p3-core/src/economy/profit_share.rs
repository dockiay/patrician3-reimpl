//! Participaciones y reparto de beneficios, tarea 0x18 (0x004E8804).
use crate::{ai::reputation::Merchant, scheduler::TaskRecord};
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Notice {
    pub recipient: u8,
    pub record: [u8; 16],
}
#[derive(Clone, Debug)]
pub struct Prepared {
    pub task: TaskRecord,
    pub merchants: Vec<Merchant>,
    pub notices: Vec<Notice>,
    pub debt: Option<TaskRecord>,
    pub keep: bool,
}
fn word(b: &mut [u8], at: usize, v: u16) {
    b[at..at + 2].copy_from_slice(&v.to_le_bytes())
}
fn dword(b: &mut [u8], at: usize, v: u32) {
    b[at..at + 4].copy_from_slice(&v.to_le_bytes())
}
/// La reserva de una deuda y la entrega de cartas se preparan fuera del núcleo.
pub fn prepare(
    task: TaskRecord,
    merchants: &[Merchant],
    tick: u32,
    year: u16,
    head: u16,
    town_stock: u32,
) -> Result<Prepared, String> {
    if task.kind != 0x18 {
        return Err("tipo de participación inválido".into());
    }
    if merchants.len() > u16::MAX as usize || merchants.iter().any(|m| m.bytes.len() < 0x650) {
        return Err("comerciantes de participación truncados".into());
    }
    let mut out = Prepared {
        task,
        merchants: merchants.to_vec(),
        notices: Vec::new(),
        debt: None,
        keep: false,
    };
    let owner = usize::from(task.u8_at(0x10));
    let investor = usize::from(task.u8_at(0x11));
    let shares = task.u8_at(0x12);
    if owner >= merchants.len() || shares == 0 {
        return Ok(out);
    }
    let boundary = [0u16, 90, 181, 273, 365]
        .iter()
        .copied()
        .map(|day| (u32::from(year).wrapping_mul(365).wrapping_add(u32::from(day))).wrapping_shl(8))
        .find(|&d| d >= task.due)
        .or_else(|| {
            // Al final del rango u32 la última fecha trimestral se desborda. El
            // original continúa leyendo la tabla de seis en seis; en el EXE inglés
            // verificado, la primera palabra posterior que satisface este intervalo
            // está en la entrada 430 y vale 350. Fuera de él se rechaza la entrada.
            (year == 45964 && task.due > 0xffffad00 && task.due <= 0xfffffa00).then_some(0xfffffa00)
        })
        .ok_or("fecha de participación fuera de la tabla contrastada")?;
    if boundary == task.due {
        let old = task.u32_at(0xc) as i32;
        let net = out.merchants[owner].i32(0x46c);
        let gain = net.wrapping_sub(old);
        out.task.set_u32_at(0xc, net as u32);
        let mut payment = 0i32;
        if gain > 0 {
            payment = (i32::from(shares).wrapping_mul(gain)) / 400;
            let m = &mut out.merchants[owner];
            m.set_i32(0x4bc, m.i32(0x4bc).wrapping_add(payment));
            let cash = m.i32(0);
            if cash >= payment {
                m.set_i32(0, cash.wrapping_sub(payment));
                let mut msg = [0u8; 16];
                msg[4] = 0x62;
                msg[5] = m.bytes[0x19];
                let name = if investor < merchants.len() {
                    u16::from(merchants[investor].bytes[0x1c]) * 256 + u16::from(merchants[investor].bytes[0x1d])
                } else {
                    u16::MAX
                };
                word(&mut msg, 8, name);
                word(&mut msg, 10, (gain >> 16) as u16);
                word(&mut msg, 12, gain as u16);
                word(&mut msg, 14, u16::from(shares));
                out.notices.push(Notice {
                    recipient: owner as u8,
                    record: msg,
                });
            } else {
                let balance = if cash > 0 {
                    m.set_i32(0, 0);
                    payment.wrapping_sub(cash)
                } else {
                    payment
                };
                let mut debt = TaskRecord {
                    due: tick.wrapping_add(0x1400),
                    kind: 1,
                    ..Default::default()
                };
                debt.set_u32_at(8, balance as u32);
                debt.set_u32_at(0xc, tick & 0xffff_ff00);
                debt.set_u8_at(0x10, 0xff);
                debt.set_u8_at(0x11, owner as u8);
                debt.set_u8_at(0x14, m.bytes[0x19]);
                if m.u16(8) == 0 {
                    let mut msg = [0u8; 16];
                    msg[4] = 0x28;
                    msg[5] = m.bytes[0x19];
                    word(&mut msg, 8, payment as u16);
                    word(&mut msg, 10, balance as u16);
                    dword(&mut msg, 12, debt.due);
                    out.notices.push(Notice {
                        recipient: owner as u8,
                        record: msg,
                    });
                }
                if m.bytes[0x1e] < 0xfe {
                    m.bytes[0x1e] = m.bytes[0x1e].wrapping_add(2)
                }
                out.debt = Some(debt);
            }
        }
        if investor < merchants.len() {
            let town = out.merchants[owner].bytes[0x19];
            let m = &mut out.merchants[investor];
            m.set_i32(0, m.i32(0).wrapping_add(payment));
            m.set_i32(0x4c8, m.i32(0x4c8).wrapping_add(payment));
            if m.u16(8) == 0 {
                let mut msg = [0u8; 16];
                msg[4] = 0x27;
                msg[5] = town;
                word(&mut msg, 8, owner as u16);
                word(&mut msg, 10, u16::from(shares));
                dword(&mut msg, 12, gain as u32);
                out.notices.push(Notice {
                    recipient: investor as u8,
                    record: msg,
                });
            }
        }
        out.task.due = task.due.wrapping_add(u32::from(head as u8).wrapping_add(0x100));
        out.keep = true;
        return Ok(out);
    }
    if investor == 255 {
        let threshold = task.u32_at(8).wrapping_add(
            ((u32::from(task.next as u8) & 3) + (u32::from(head as u8) & 15) + (task.u32_at(8) & 3) + 0x15) << 8,
        );
        if threshold < task.due {
            let net = out.merchants[owner].i32(0x46c);
            if net <= 100 {
                out.task.set_u8_at(0x13, shares)
            } else {
                let value = (net >> 4).wrapping_mul(((town_stock & 15) + 1) as i32) / 100000;
                let portion = (i32::from(shares) / 5).wrapping_sub(value).wrapping_mul(5);
                out.task.set_u8_at(
                    0x13,
                    if portion < 5 {
                        5
                    } else if portion > i32::from(shares) {
                        shares
                    } else {
                        portion as u8
                    },
                );
            }
        }
        out.task.due = task.due.wrapping_add(0x700 + (task.u32_at(8) & 31));
    } else {
        let net = out.merchants[owner].i32(0x46c);
        let chosen = if net <= 100 {
            0
        } else {
            let product = ((task.u32_at(0xc) >> 3) as i32).wrapping_mul(i32::from(shares) / 5);
            let q = product / net;
            let rem = product % net;
            let rem_with_owner = (rem as u32 & 0xffff_ff00) | u32::from(task.u8_at(0x10));
            let factor = rem_with_owner.wrapping_add(tick) & 3;
            let value = q.wrapping_mul(factor as i32);
            if value <= 0 {
                0
            } else {
                value.wrapping_mul(5).min(i32::from(shares)).min(25) as u8
            }
        };
        out.task.set_u8_at(0x13, chosen);
        out.task.due = task.due.wrapping_add(((task.u32_at(8) & 7) + 3) << 8);
    }
    if out.task.due > boundary {
        out.task.due = boundary
    }
    out.keep = true;
    Ok(out)
}
