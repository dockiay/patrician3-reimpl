//! Operación 0x36 (`0x00535760` → `0x004DA510`): un comerciante pone un barco propio en subasta.
//!
//! El barco debe estar en puerto, libre de convoy, con estado < 4 y su dueño debe tener la
//! ciudad en la máscara `+0x468`. Si hay en esa ciudad una subasta naval (tarea 0x0A) en la
//! fase 1 que vence antes del final del tick siguiente, o en la fase 2 dentro de 0x32 ticks,
//! con un lote libre (`+8/+0xA/+0xC` >= número de barcos), el barco ocupa ese lote. Si no, se
//! crea una tarea 0x0A en la fase 1 que vence en el tick siguiente; el byte de desfase
//! `+0xF` evita los huecos de 6 bytes que ocupan las demás subastas (0x0A y 0x36) en una tabla
//! de 0x190 casillas de 256 ticks. En ambos casos el barco pasa a estado 0x0C con `+0x3C|=0x20`.
use crate::scheduler::Scheduler;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Registered {
    /// Lote `slot` (0..3) de una subasta existente.
    Joined { task: u16, slot: u8 },
    /// Tarea nueva.
    Created { task: u16 },
}

fn r16(b: &[u8], at: usize) -> u16 {
    u16::from_le_bytes([b[at], b[at + 1]])
}

/// Vista de lo que lee `0x004DA510`.
pub struct RegisterWorld<'a> {
    /// `gestor+0xF6`
    pub convoy_count: u16,
    /// `GameWorld+0x10`
    pub town_count: u32,
    /// Máscara `+0x468` de cada comerciante; su longitud es `GameWorld+0xA`.
    pub town_masks: &'a [u32],
    /// `0x006DE4B4`
    pub tick: u32,
    /// Byte `+0xD` del objeto de opciones `0x006CC3E8`: distinto de 0 salta la búsqueda en la fase 1.
    pub options_d: u8,
}

/// `0x004DA510`. Sin efecto (`None`) si el barco no cumple las condiciones.
pub fn register_ship_for_auction(s: &mut Scheduler, ships: &mut [Vec<u8>], w: &RegisterWorld<'_>, ship: u32) -> Option<Registered> {
    let count = ships.len();
    if ship as usize >= (count & 0xffff) {
        return None;
    }
    let id = ship as usize;
    let img = &ships[id];
    if r16(img, 8) < w.convoy_count {
        return None;
    }
    let town = img[0x39];
    if u32::from(town) >= w.town_count || r16(img, 0x134) >= 4 {
        return None;
    }
    let owner = usize::from(img[0]);
    if owner >= w.town_masks.len() || w.town_masks[owner] & (1u32 << (town & 31)) == 0 {
        return None;
    }
    let cap = s.capacity;
    let free_lot = |s: &Scheduler, at: u16| (0..3).find(|&k| r16(&s.records[usize::from(at)].payload, 2 * k) as usize >= count);
    let mut at = s.head;
    let limit = (w.tick | 0xff).wrapping_add(0x301);
    if w.options_d == 0 {
        while at < cap {
            let r = &s.records[usize::from(at)];
            if r.due >= limit {
                break;
            }
            if r.kind == 0x0a && r.u8_at(0x15) == town && r.u8_at(0x14) == 1 {
                if let Some(k) = free_lot(s, at) {
                    return Some(join(s, ships, id, at, k));
                }
            }
            at = s.records[usize::from(at)].next;
        }
    } else {
        while at < cap && s.records[usize::from(at)].due < limit {
            at = s.records[usize::from(at)].next;
        }
    }
    let limit = w.tick.wrapping_add(0x3200);
    while at < cap {
        let r = &s.records[usize::from(at)];
        if r.due > limit {
            break;
        }
        if r.kind == 0x0a && r.u8_at(0x15) == town && r.u8_at(0x14) == 2 {
            if let Some(k) = free_lot(s, at) {
                return Some(join(s, ships, id, at, k));
            }
        }
        at = s.records[usize::from(at)].next;
    }
    // Tabla de casillas ocupadas por las demás subastas.
    let mut free = [1u8; 0x190];
    let mut at = s.head;
    while at < cap {
        let r = &s.records[usize::from(at)];
        if matches!(r.kind, 0x0a | 0x36) {
            let t = match r.u8_at(0x14) {
                0 => r.due.wrapping_add(0x2000),
                1 => r.due.wrapping_add(0x200).wrapping_add(u32::from(r16(&r.payload, 6))),
                2 => r.due.wrapping_add(0x200),
                _ => r.due,
            };
            let slot = (t.wrapping_sub(w.tick) >> 8) as usize;
            if slot < 0x17c {
                free[slot..slot + 6].fill(0);
            }
        }
        at = r.next;
    }
    let task = s.add_task(w.tick.wrapping_add(1));
    let r = &mut s.records[usize::from(task)];
    r.kind = 0x0a;
    r.payload = [0; 16];
    r.payload[0..2].copy_from_slice(&(ship as u16).to_le_bytes());
    r.payload[2..6].fill(0xff);
    r.set_u8_at(0x14, 1);
    r.set_u8_at(0x15, town);
    r.set_u8_at(0x17, 0xff);
    let mut offset = ((w.tick & 3) + (ship & 3) + 0xe) as u16;
    loop {
        let base = usize::from(offset);
        if base + 5 >= 0x190 {
            break;
        }
        match (0..6).rev().find(|&k| free[base + k] == 0) {
            Some(k) => offset = offset.wrapping_add(k as u16 + 1),
            None => break,
        }
    }
    r.payload[6..8].copy_from_slice(&(offset << 8).to_le_bytes());
    mark(&mut ships[id]);
    Some(Registered::Created { task })
}

fn mark(img: &mut [u8]) {
    img[0x3c] |= 0x20;
    img[0x134..0x136].copy_from_slice(&0xcu16.to_le_bytes());
}

fn join(s: &mut Scheduler, ships: &mut [Vec<u8>], id: usize, at: u16, k: usize) -> Registered {
    mark(&mut ships[id]);
    s.records[usize::from(at)].payload[2 * k..2 * k + 2].copy_from_slice(&(id as u16).to_le_bytes());
    Registered::Joined { task: at, slot: k as u8 }
}
