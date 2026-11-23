//! Plazo del pasajero de taberna: tarea 0x10 (`004E63E4`).
use crate::{
    ai::reputation::Merchant,
    scheduler::{Scheduler, TaskRecord},
};
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Notice {
    pub recipient: u8,
    pub record: [u8; 16],
}
pub struct Prepared {
    pub task: TaskRecord,
    pub ships: Vec<u8>,
    pub convoys: Vec<u8>,
    pub merchants: Vec<Merchant>,
    pub notice: Option<Notice>,
    pub keep: bool,
}
fn word(b: &[u8], i: usize) -> u16 {
    u16::from_le_bytes([b[i], b[i + 1]])
}
fn dword(b: &[u8], i: usize) -> u32 {
    u32::from_le_bytes(b[i..i + 4].try_into().unwrap())
}
fn w16(b: &mut [u8], i: usize, v: u16) {
    b[i..i + 2].copy_from_slice(&v.to_le_bytes())
}
fn w32(b: &mut [u8], i: usize, v: u32) {
    b[i..i + 4].copy_from_slice(&v.to_le_bytes())
}
/// Prepara sobre copias, sin publicar la tarea ni las vistas de flota y cuentas.
pub fn prepare(
    q: &Scheduler,
    ships: &[u8],
    convoys: &[u8],
    merchants: &[Merchant],
    merchant_count: u16,
    town_count: u32,
    tick: u32,
) -> Result<Prepared, String> {
    if ships.len() % 0x180 != 0
        || convoys.len() % 0x3c != 0
        || usize::from(merchant_count) > merchants.len()
        || merchants
            .iter()
            .take(usize::from(merchant_count))
            .any(|m| m.bytes.len() != 0x650)
    {
        return Err("vistas de pasajero truncadas".into());
    }
    let task = *q.records.get(usize::from(q.head)).ok_or("tarea de pasajero ausente")?;
    if task.kind != 0x10 {
        return Err("tipo de tarea de pasajero incorrecto".into());
    }
    let mut out = Prepared {
        task,
        ships: ships.to_vec(),
        convoys: convoys.to_vec(),
        merchants: merchants.to_vec(),
        notice: None,
        keep: false,
    };
    let ship = usize::from(u16::from_le_bytes([task.u8_at(8), task.u8_at(9)]));
    if ship >= ships.len() / 0x180 {
        return Ok(out);
    }
    let at = ship * 0x180;
    let dest = task.u8_at(0xb);
    let city = out.ships[at + 0x39];
    let state = word(&out.ships, at + 0x134);
    let deadline = task.u32_at(0x10);
    if u32::from(city) < town_count && city == dest && (state == 2 || state == 3) {
        let counter = u32::from(word(&out.ships, at + 0x138));
        out.task.due = out
            .task
            .due
            .wrapping_add(if counter >= 0x40 { 1 } else { 0x40 - counter });
        out.keep = true;
        return Ok(out);
    }
    if u32::from(city) < town_count && city != dest && tick <= deadline {
        out.task.due = out.task.due.wrapping_add(0x100);
        out.keep = true;
        return Ok(out);
    }
    if u32::from(city) >= town_count && state != 0x11 && state != 0x12 && state <= 0x16 {
        out.task.due = out.task.due.wrapping_add(0x20);
        out.keep = true;
        return Ok(out);
    }
    if u32::from(city) < town_count {
        let owner = out.ships[at];
        let payment = u16::from_le_bytes([task.u8_at(0xc), task.u8_at(0xd)]);
        let (status, paid) = if city != dest {
            (2, 0)
        } else if tick <= deadline {
            (0, payment)
        } else {
            let days = ((tick - deadline) >> 8) + 1;
            if days >= 5 {
                (2, 0)
            } else {
                (1, (u32::from(payment) * (5 - days) / 5) as u16)
            }
        };
        let mut record = [0u8; 16];
        record[1] = 0x80;
        record[2..4].fill(255);
        record[4] = 0x18;
        record[5] = city;
        record[6..8].fill(255);
        w16(&mut record, 8, status);
        w16(&mut record, 10, paid);
        w16(&mut record, 12, u16::from(task.u8_at(0xa)));
        if usize::from(owner) < usize::from(merchant_count) {
            let person = &mut out.merchants[usize::from(owner)];
            if city == dest {
                person.set_i32(0, person.i32(0).wrapping_add(i32::from(paid)));
                person.set_i32(0x4b8, person.i32(0x4b8).wrapping_add(i32::from(paid)))
            }
        }
        // La rama vencida en otra ciudad llama incluso para un dueño inválido;
        // add_message hace posteriormente la validación del destinatario.
        if city != dest || usize::from(owner) < usize::from(merchant_count) {
            out.notice = Some(Notice {
                recipient: owner,
                record,
            });
        }
    }
    let mut next = task.next;
    let mut seen = vec![false; usize::from(q.capacity)];
    let mut other = false;
    while next < q.capacity {
        let i = usize::from(next);
        if seen[i] {
            return Err("lista de tareas de pasajero cíclica".into());
        }
        seen[i] = true;
        let t = q.records.get(i).ok_or("lista de tareas de pasajero truncada")?;
        if t.kind == 0x10 && u16::from_le_bytes([t.u8_at(8), t.u8_at(9)]) == ship as u16 {
            other = true;
            break;
        }
        next = t.next
    }
    if !other {
        let old = out.ships[at + 0x157];
        out.ships[at + 0x157] = if old < 2 { 0 } else { old - 1 };
        let flags = word(&out.ships, at + 0x3c) | 4;
        w16(&mut out.ships, at + 0x3c, flags);
        let cv = usize::from(word(&out.ships, at + 8));
        if cv < out.convoys.len() / 0x3c {
            let p = cv * 0x3c + 0x18;
            let flags = dword(&out.convoys, p) | 4;
            w32(&mut out.convoys, p, flags)
        }
    }
    Ok(out)
}
