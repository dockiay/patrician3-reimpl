//! Inicio de `st_expedition_return` (0x004E6A54), junto con sus dos
//! métricas de expedición 0x0052D950 y 0x0052DB50.
//!
//! Este módulo solo cubre la fase inicial (`task+0xA == 0xFFFF`). La fase
//! posterior de regreso, daños y cartas sigue siendo una dependencia aparte.
use crate::scheduler::TaskRecord;

fn distance(a: (u16, u16), b: (u16, u16)) -> u32 {
    let x = i64::from(a.0) - i64::from(b.0);
    let y = i64::from(a.1) - i64::from(b.1);
    (((x * x + y * y) as f64).sqrt() * 29.0) as u32
}

fn zone((x, y): (u16, u16)) -> u8 {
    let (x, y) = (i32::from(x), i32::from(y));
    if x < 0xd8 {
        0
    } else if y <= (0x104 - x) * 5 / 2 {
        1
    } else if y <= x * 15 / 11 - 0x17c {
        3
    } else {
        2
    }
}

/// 52D950. Como el original, en una de las zonas el último tramo suma cero.
pub fn from_start(p: (u16, u16)) -> u32 {
    let mut total = ((569.0_f64).sqrt() * 29.0) as u32;
    let (x, y) = (i32::from(p.0), i32::from(p.1));
    if y <= (0x104 - x) * 5 / 2 {
        let n = distance(p, (0xe2, 0x0d));
        return total.wrapping_add(if p.0 <= 0xd8 { n.wrapping_mul(2) } else { n });
    }
    total = total.wrapping_add(((3349.0_f64).sqrt() * 29.0) as u32);
    if y > x * 15 / 11 - 0x17c {
        return total;
    }
    total
        .wrapping_add(((11917.0_f64).sqrt() * 29.0) as u32)
        .wrapping_add(distance(p, (0x145, 0x40)))
}

/// 52DB50; cada segmento se trunca antes de aplicar su multiplicador.
pub fn between(mut a: (u16, u16), mut b: (u16, u16)) -> u32 {
    let (mut za, mut zb) = (zone(a), zone(b));
    if za > zb {
        std::mem::swap(&mut a, &mut b);
        std::mem::swap(&mut za, &mut zb);
    }
    let mut weight = if za == 0 { 2 } else { u32::from(za) };
    if za == 0 && zb > 1 {
        za = 1;
    }
    let mut total = 0u32;
    for current in za..=zb {
        let next = if current == zb {
            b
        } else {
            match current {
                0 => (0xe2, 0x0d),
                1 => (0xd8, 0x46),
                2 => (0x145, 0x40),
                _ => unreachable!(),
            }
        };
        total = total.wrapping_add(distance(a, next).wrapping_mul(weight));
        a = next;
        weight = 1;
    }
    total
}

/// Prepara únicamente la fase inicial. `Ok(false)` reproduce la retirada de
/// una tarea cuyo convoy ya no existe; las entradas inválidas no se publican.
pub fn start(task: &mut TaskRecord, convoys: &[u8], convoy_count: u16) -> Result<bool, &'static str> {
    if task.kind != 0x12 {
        return Err("tarea de expedición incompatible");
    }
    let id = u16::from_le_bytes([task.u8_at(8), task.u8_at(9)]);
    if id >= convoy_count {
        return Ok(false);
    }
    if task.u8_at(0xa) != 0xff || task.u8_at(0xb) != 0xff {
        return Err("fase de retorno de expedición pendiente");
    }
    let at = usize::from(id) * 0x3c;
    let c = convoys.get(at..at + 0x3c).ok_or("convoy de expedición truncado")?;
    let word = |i| u16::from_le_bytes([c[i], c[i + 1]]);
    let speed = u32::from_le_bytes(c[0xc..0x10].try_into().unwrap());
    if speed == 0 {
        return Err("velocidad de expedición nula");
    }
    let points: Vec<_> = (0..4).map(|i| (word(0x24 + i * 4), word(0x26 + i * 4))).collect();
    if points[0].0 > 0x7fff || points[0].1 > 0x7fff {
        return Err("coordenada de expedición no contrastada");
    }
    let mut last = points[0];
    let mut distance = from_start(last);
    for &point in &points[1..] {
        if point.0 >= 0x7fff {
            break;
        }
        if point.1 > 0x7fff {
            return Err("coordenada de expedición no contrastada");
        }
        distance = distance.wrapping_add(between(last, point));
        last = point;
    }
    distance = distance.wrapping_add(from_start(last));
    let duration = distance.wrapping_shl(7) / speed;
    let mut next = *task;
    next.due = next.due.wrapping_add(duration);
    if word(0x38) == 0 {
        next.due = next.due.wrapping_add(duration >> 1);
    }
    next.set_u8_at(0xa, 4);
    next.set_u8_at(0xb, 0);
    *task = next;
    Ok(true)
}
