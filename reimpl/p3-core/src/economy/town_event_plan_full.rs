//! Preparación transaccional del planificador urbano `0x004E0334`.
use crate::{
    ai::reputation::Merchant,
    economy::{
        town_event_prelude::{self, Outcome},
        town_event_selection,
    },
    pump::Operation,
    rng::CrtRand,
    scheduler::Scheduler,
    town::{off, Town},
};

pub struct Inputs<'a> {
    pub scheduler: &'a Scheduler,
    pub towns: &'a [Town],
    pub merchants: &'a [Merchant],
    pub merchant_count: u16,
    pub facilities: &'a [[u8; 20]],
    pub offices: &'a [Vec<u8>],
    pub bands: [Option<u16>; 5],
    pub facility_byte: u8,
    pub tick: u32,
    pub day: u16,
    pub operations_remote: bool,
    pub rng: Option<CrtRand>,
}
pub struct Prepared {
    pub scheduler: Scheduler,
    pub towns: Vec<Town>,
    pub rng: Option<CrtRand>,
    pub operation: Option<Operation>,
    pub keep: bool,
}

pub fn prepare(i: Inputs<'_>) -> Result<Prepared, String> {
    if i.scheduler.records.len() != usize::from(i.scheduler.capacity)
        || usize::from(i.scheduler.head) >= i.scheduler.records.len()
        || usize::from(i.merchant_count) > i.merchants.len()
    {
        return Err("tablas del plan urbano incompatibles".into());
    }
    let mut seen = vec![false; i.scheduler.records.len()];
    let mut link = i.scheduler.head;
    while usize::from(link) < seen.len() {
        if seen[usize::from(link)] {
            return Err("planificador urbano cíclico".into());
        }
        seen[usize::from(link)] = true;
        link = i.scheduler.records[usize::from(link)].next
    }
    let index = usize::from(i.scheduler.head);
    let original = i.scheduler.records[index];
    let mut scheduler = i.scheduler.clone();
    let mut towns = i.towns.to_vec();
    let (mut random, mut operation) = (i.rng, None);
    let outcome = town_event_prelude::prepare(&original, i.towns, i.day, i.operations_remote, i.rng)?;
    let keep = match outcome {
        Outcome::BeforeSeason { task } => {
            scheduler.records[index] = task;
            true
        }
        Outcome::YearClosed {
            task,
            operation: op,
            rng,
        } => {
            scheduler.records[index] = task;
            operation = op;
            random = rng;
            false
        }
        Outcome::Candidate { mut task, slot, town } => {
            let city = usize::from(town);
            let (updated, selection) =
                town_event_selection::classify(i.towns, city, i.facilities, i.offices, i.bands, i.facility_byte)?;
            towns[city] = updated;
            if let Some(event) = selection {
                let seed = task.payload[8 + slot];
                let due = i.tick.wrapping_add(0x100).wrapping_add(u32::from(seed));
                if due <= i.tick {
                    return Err("fecha de acontecimiento desbordada".into());
                }
                if scheduler.free_head > scheduler.capacity {
                    return Err("cabeza libre del plan urbano fuera de tabla".into());
                }
                if scheduler.free_head >= scheduler.capacity && scheduler.capacity > u16::MAX - 128 {
                    return Err("depósito de tareas lleno".into());
                }
                scheduler.dispatching = 0;
                let id = scheduler.add_task(due);
                scheduler.dispatching = 1;
                let next = &mut scheduler.records[usize::from(id)];
                next.kind = 0x1e;
                next.set_u32_at(8, due.wrapping_add(0x4300));
                next.set_u32_at(0xc, due.wrapping_add(0x1500));
                next.set_u8_at(0x10, event.kind);
                next.set_u8_at(0x11, town);
                let detail = if event.kind == 1 {
                    towns[city].u16(0x2c6)
                } else {
                    u16::MAX
                };
                next.set_u8_at(0x12, detail as u8);
                next.set_u8_at(0x13, (detail >> 8) as u8);
                next.set_u8_at(0x14, (seed & 7) + ((seed >> 3) & 3) + 5);
                next.set_u8_at(0x15, (task.payload[15] & 7) + 2);
                next.set_u8_at(0x16, 255);
                next.set_u8_at(0x17, 0);
                if !i.operations_remote {
                    let rng = random.as_mut().ok_or("estado CRT de suceso urbano ausente")?;
                    let start = (rng.rand() as usize) % towns.len();
                    let mut chosen = None;
                    for step in 0..towns.len() {
                        let k = (start + step) % towns.len();
                        let t = &towns[k];
                        if t.u32(off::TOWN_FLAGS) & 0x40 == 0 {
                            continue;
                        }
                        let mayor = usize::from(t.u8(off::MAYOR_ID));
                        if mayor >= usize::from(i.merchant_count) {
                            let slot = (255usize.wrapping_sub(mayor)) & 3;
                            chosen = Some((
                                k,
                                t.u8(0x6d4 + slot),
                                t.u8(0x6d8 + slot),
                                t.u8(0x6e0 + slot),
                                t.u8(0x6e4 + slot),
                            ));
                            break;
                        }
                        let person = i.merchants.get(mayor).ok_or("alcalde de suceso ausente")?;
                        if person.bytes.len() != 0x650 {
                            return Err("alcalde de suceso truncado".into());
                        }
                        if person.u16(8) == 0 {
                            continue;
                        }
                        let home = usize::from(person.bytes[0x19]);
                        let rank = *person
                            .bytes
                            .get(0x39c + home)
                            .ok_or("rango de alcalde fuera de tabla")?;
                        chosen = Some((
                            k,
                            person.bytes[0x1c],
                            person.bytes[0x1d],
                            person.get_merchant_epithet_index(),
                            rank,
                        ));
                        break;
                    }
                    let (notable, first, last, epithet, rank) =
                        chosen.ok_or("original sin alcalde válido: búsqueda circular infinita")?;
                    let mut bytes = [0u8; 20];
                    bytes[..4].copy_from_slice(&0x84u32.to_le_bytes());
                    bytes[8] = town;
                    bytes[9] = event.kind;
                    bytes[10] = event.detail;
                    bytes[11] = event.extra;
                    bytes[12] = (rng.rand() as u8 & 3) + 2;
                    bytes[13] = first;
                    bytes[14] = last;
                    bytes[15] = notable as u8;
                    bytes[16] = epithet;
                    bytes[17] = rank;
                    let due = i.tick.wrapping_add(((rng.rand() as u32 & 7) + 5) * 256);
                    bytes[4..8].copy_from_slice(&due.to_le_bytes());
                    operation = Some(Operation(std::array::from_fn(|k| {
                        u32::from_le_bytes(bytes[4 * k..4 * k + 4].try_into().unwrap())
                    })));
                }
                if slot < 6 {
                    task.payload[slot + 1] = 255;
                    task.due = task.due.wrapping_add(0x1e00)
                }
                if slot < 5 {
                    task.payload[slot + 2] = 255;
                    task.due = task.due.wrapping_add(0x1e00)
                }
                task.payload[slot] = 255;
                task.due = task
                    .due
                    .wrapping_add((u32::from(seed & 7) + 20) * 256)
                    .wrapping_add(u32::from(seed));
                task.payload[8 + slot] = 0;
            } else {
                let seed = task.payload[8 + slot];
                task.payload[slot] = 255;
                task.due = task
                    .due
                    .wrapping_add((u32::from(seed & 7) + 20) * 256)
                    .wrapping_add(u32::from(seed))
            }
            if scheduler.head != i.scheduler.head {
                return Err("tarea nueva alteraría la cabeza del despachador".into());
            }
            // `add_task` puede enlazar la nueva tarea detrás de esta misma
            // cabeza; no sobrescribir ese enlace al publicar el payload.
            task.next = scheduler.records[index].next;
            scheduler.records[index] = task;
            true
        }
    };
    if keep && scheduler.records[index].due <= i.tick {
        return Err("la tarea urbana mantendría su fecha en el pasado".into());
    }
    Ok(Prepared {
        scheduler,
        towns,
        rng: random,
        operation,
        keep,
    })
}
