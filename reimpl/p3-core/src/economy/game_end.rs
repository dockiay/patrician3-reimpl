//! Scheduled victory/defeat conditions at 004E4A44.
use crate::{
    ai::reputation::Merchant,
    scheduler::{Scheduler, TaskRecord},
    town::Town,
};
const DURATION: [u32; 14] = [
    0x16d00, 0x2da00, 0x44700, 0x5b400, 0x72100, 0x88e00, 0x9fb00, 0xb6800, 0xcd500, 0xe4200, 0x156300, 0x1c8400,
    0x2ac600, 0x474a00,
];
const MONTH: [u32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Event {
    /// Original objective, retained even when the window argument is zero.
    pub objective: u16,
    pub kind: u16,
    pub winner: u32,
    pub goal: u16,
    pub notification_home: u8,
    pub timed_out: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Prepared {
    pub task: TaskRecord,
    pub keep: bool,
    pub event: Option<Event>,
}
pub struct Context<'a> {
    pub start: u32,
    pub tick: u32,
    pub month: u8,
    pub game_mode: u8,
    pub player: u32,
    pub merchant_count: u16,
    pub merchants: &'a [Merchant],
    pub towns: &'a [Town],
    pub alderman: u8,
    pub top_ranked: Option<u16>,
}
pub fn prepare(q: &Scheduler, c: &Context<'_>) -> Result<Prepared, String> {
    let mut t = *q.records.get(usize::from(q.head)).ok_or("tarea de fin ausente")?;
    if t.kind != 0xe {
        return Err("tipo de fin incorrecto".into());
    }
    let goal = u16::from_le_bytes([t.u8_at(8), t.u8_at(9)]);
    let duration = u16::from_le_bytes([t.u8_at(10), t.u8_at(11)]);
    let limit = match duration {
        0..=13 => Some(DURATION[usize::from(duration)]),
        16 => Some(70000),
        17 => Some(100000),
        18 => Some(200000),
        _ => None,
    };
    let unlimited = limit.is_none();
    let end = c.start.wrapping_add(limit.unwrap_or(0));
    let mut out = Prepared {
        task: t,
        keep: false,
        event: None,
    };
    if !(1..=6).contains(&goal) {
        return Ok(out);
    }
    if usize::from(c.merchant_count) > c.merchants.len()
        || c.merchants
            .iter()
            .take(usize::from(c.merchant_count))
            .any(|m| m.bytes.len() != 0x650)
    {
        return Err("comerciantes de fin incompletos".into());
    }
    if unlimited && !matches!(goal, 2 | 3) {
        return Err("duracion indefinida usa memoria sin inicializar en el original".into());
    }
    let mut winner = None;
    let mut timeout = false;
    let mut notification_home = None;
    if goal <= 3 && (t.due < end || unlimited) {
        for id in 20..usize::from(c.merchant_count) {
            let m = &c.merchants[id];
            if c.game_mode != 3 && m.u16(8) != 0 {
                continue;
            }
            let ok = match goal {
                1 => m.bytes[0x39c + usize::from(m.bytes[0x19])] == 4,
                2 => c.towns.iter().any(|t| usize::from(t.u8(0x6f1)) == id),
                3 => usize::from(c.alderman) == id,
                _ => false,
            };
            if ok {
                winner = Some(id as u32);
                notification_home = Some(m.bytes[0x19]);
                break;
            }
        }
        if winner.is_none() {
            match goal {
                1 => {
                    let days = *MONTH.get(usize::from(c.month)).ok_or("mes de fin fuera de tabla")?;
                    let next = q
                        .records
                        .get(usize::from(c.merchant_count))
                        .ok_or("reprogramacion original lee fuera del pool")?
                        .due;
                    t.due = next.wrapping_add(days << 8)
                }
                2 => {
                    let mut at = q.head;
                    let mut seen = vec![false; usize::from(q.capacity)];
                    while at < q.capacity {
                        let i = usize::from(at);
                        if seen[i] {
                            return Err("lista ciclica al buscar eleccion".into());
                        }
                        seen[i] = true;
                        let r = q.records.get(i).ok_or("lista de elecciones truncada")?;
                        if r.kind == 2 {
                            t.due = r.due.wrapping_add(1);
                            break;
                        }
                        at = r.next
                    }
                }
                3 => {
                    let r = q
                        .records
                        .iter()
                        .take(usize::from(q.capacity))
                        .find(|r| r.kind == 0x20)
                        .ok_or("el original desreferencia una eleccion de liga ausente")?;
                    t.due = r.due.wrapping_add(1)
                }
                _ => unreachable!(),
            };
            if !unlimited && end < t.due {
                t.due = end
            }
            out.task = t;
            out.keep = true;
            return Ok(out);
        }
    } else if goal <= 3 {
        timeout = true;
        winner = Some(u32::from(c.merchant_count));
    } else if goal == 4 || goal == 5 {
        if t.due < end {
            out.task.due = end;
            out.keep = true;
            return Ok(out);
        }
        if goal == 4 {
            winner = Some(u32::from(c.top_ranked.ok_or("clasificacion final no disponible")?))
        } else {
            let mut best = i32::MIN;
            let mut selected = 0;
            for id in 20..usize::from(c.merchant_count) {
                let m = &c.merchants[id];
                notification_home = Some(m.bytes[0x19]);
                let value = m.i32(0x46c);
                if (c.game_mode == 3 || m.u16(8) == 0) && best < value {
                    best = value;
                    selected = id as u32
                }
            }
            if notification_home.is_none() {
                return Err("el original usa comerciante no inicializado al calcular fortuna".into());
            }
            winner = Some(selected)
        }
    } else {
        let total = c
            .towns
            .iter()
            .fold(0u32, |sum, t| sum.wrapping_add(t.i32(0x2d4) as u32));
        if total < limit.unwrap() {
            out.task.due = c.tick.wrapping_add(0x100) & !255;
            out.keep = true;
            return Ok(out);
        }
        winner = Some(u32::from(c.top_ranked.ok_or("clasificacion final no disponible")?))
    }
    let winner = winner.expect("objetivo validado");
    let home = if let Some(home) = notification_home {
        home
    } else {
        c.merchants
            .get(c.player as usize)
            .ok_or("jugador de fin ausente")?
            .bytes[0x19]
    };
    let success = !timeout && winner == c.player;
    out.event = Some(Event {
        objective: goal,
        kind: if success { 15 } else { 16 },
        winner,
        goal: if success { 0 } else { goal },
        notification_home: home,
        timed_out: timeout,
    });
    Ok(out)
}
