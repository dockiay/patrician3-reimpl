//! Fases 0 y 1 de la tarea de acontecimiento urbano 0x1E (0x004DFD44).
use crate::{
    ai::reputation::Merchant,
    scheduler::TaskRecord,
    town::{off, Town},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Prepared {
    pub task: TaskRecord,
    pub town: Town,
    pub keep: bool,
}

/// Prepara el resultado sobre copias. Las fases piratas y saqueos aún se
/// rechazan para que el llamante no publique efectos parciales.
pub fn prepare(task: &TaskRecord, town: &Town, tick: u32) -> Result<Prepared, String> {
    if task.kind != 0x1e || !matches!(task.u8_at(0x10), 0 | 1) {
        return Err("fase de acontecimiento urbano no conectada".into());
    }
    let mut task = *task;
    let mut town = town.clone();
    let flags = town.u32(off::TOWN_FLAGS);
    if flags & 0x600 != 0 {
        task.set_u8_at(0x14, task.u8_at(0x14).wrapping_sub(1));
    }
    let sum = (0..3)
        .map(|i| i32::from(town.u16(off::SATISFACTION + i * 2) as i16))
        .sum::<i32>();
    let keep = tick < task.u32_at(0x0c) || (tick < task.u32_at(8) && sum > -30 && (task.u8_at(0x14) as i8) > 0);
    if keep {
        task.due = task.due.wrapping_add(0x300);
        return Ok(Prepared { task, town, keep: true });
    }
    if task.u8_at(0x10) == 0 {
        town.set_u8(0x6f0, 0xff);
    } else {
        town.set_u16(0x2c6, u16::from_le_bytes([task.u8_at(0x12), task.u8_at(0x13)]));
        town.set_u32(off::TOWN_FLAGS, flags & !0x0200_0000);
    }
    if flags & 0x600 != 0 && (0..3).all(|i| (town.u16(off::SATISFACTION_TARGET + i * 2) as i16) > -10) {
        task.set_u8_at(0x10, 4);
        task.due = task.due.wrapping_add(5);
        return Ok(Prepared { task, town, keep: true });
    }
    Ok(Prepared {
        task,
        town,
        keep: false,
    })
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RaidNotice {
    pub owner: u16,
    pub hometown: u8,
    pub city: u8,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PreparedRaid {
    pub task: TaskRecord,
    pub town: Town,
    pub offices: Vec<Vec<u8>>,
    pub notices: Vec<RaidNotice>,
    pub keep: bool,
}

/// Tipo 4: mueve existencias de oficinas humanas; la entrega real de las
/// cartas debe publicarse junto con el resultado preparado.
pub fn prepare_raid(
    task: &TaskRecord,
    town: &Town,
    offices: &[Vec<u8>],
    merchants: &[Merchant],
    tick: u32,
) -> Result<PreparedRaid, String> {
    if task.kind != 0x1e || task.u8_at(0x10) != 4 {
        return Err("saqueo urbano inválido".into());
    }
    let mut task = *task;
    let mut town = town.clone();
    let mut offices = offices.to_vec();
    let mut notices = Vec::new();
    let flags = town.u32(off::TOWN_FLAGS);
    if flags & 0x600 != 0 {
        task.set_u8_at(0x14, task.u8_at(0x14).wrapping_sub(1));
    }
    let sum = (0..3)
        .map(|i| i32::from(town.u16(off::SATISFACTION + i * 2) as i16))
        .sum::<i32>();
    let mut keep = tick < task.u32_at(0x0c) || (tick < task.u32_at(8) && sum > -30 && (task.u8_at(0x14) as i8) > 0);
    task.set_u32_at(0x0c, 0);
    let mut index = usize::from(town.u16(off::FIRST_OFFICE));
    let mut seen = vec![false; offices.len()];
    while index < offices.len() {
        if seen[index] {
            return Err("lista de oficinas urbanas cíclica".into());
        }
        seen[index] = true;
        let office = &mut offices[index];
        if office.len() != 0x44c {
            return Err("oficina urbana truncada".into());
        }
        let owner = u16::from_le_bytes([office[0x2c4], office[0x2c5]]);
        let next = u16::from_le_bytes([office[0x2ca], office[0x2cb]]);
        let merchant = merchants
            .get(usize::from(owner))
            .ok_or("comerciante de saqueo ausente")?;
        if merchant.bytes.len() != 0x650 {
            return Err("comerciante de saqueo truncado".into());
        }
        if merchant.u16(8) == 0 {
            let mut last = 0i32;
            for ware in 0..24 {
                let pos = 4 + ware * 4;
                let v = i32::from_le_bytes(office[pos..pos + 4].try_into().unwrap());
                last = v;
                town.set_i32(pos, town.i32(pos).wrapping_add(v));
                office[pos..pos + 4].fill(0);
            }
            for slot in 0..6 {
                let pos = 0x124 + slot * 4;
                let v = i32::from_le_bytes(office[pos..pos + 4].try_into().unwrap());
                town.set_i32(pos, town.i32(pos).wrapping_add(v));
                office[pos..pos + 4].fill(0);
            }
            if last >= 200 {
                notices.push(RaidNotice {
                    owner,
                    hometown: merchant.bytes[0x19],
                    city: task.u8_at(0x11),
                });
                let left = task.u8_at(0x15);
                task.set_u8_at(0x15, left.wrapping_sub(1));
                if left < 2 {
                    keep = false;
                }
                let limit = tick.wrapping_add(0x700);
                if limit < task.u32_at(8) {
                    task.set_u32_at(8, limit);
                }
            }
        }
        index = usize::from(next);
    }
    Ok(PreparedRaid {
        task,
        town,
        offices,
        notices,
        keep,
    })
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PirateNotice {
    pub owner: u8,
    pub kind: u8,
    pub hometown: u8,
    pub payload_a: u32,
    pub payload_b: u32,
}
#[derive(Clone, Debug)]
pub struct PreparedPirates {
    pub task: TaskRecord,
    pub merchants: Vec<Merchant>,
    pub notices: Vec<PirateNotice>,
    pub spawn: Option<(u16, u8)>,
    pub keep: bool,
}

/// Fases 2 y 3: identifica la presa y prepara las cartas y la recompensa.
/// El anfitrión debe ejecutar `spawn` y entregar las cartas antes de publicar.
pub fn prepare_pirates(
    task: &TaskRecord,
    town: &Town,
    merchants: &[Merchant],
    merchant_count: u16,
    ships: &[u8],
    radius: u16,
    hideout: (i32, i32),
    tick: u32,
) -> Result<PreparedPirates, String> {
    if task.kind != 0x1e
        || !matches!(task.u8_at(0x10), 2 | 3)
        || usize::from(merchant_count) > merchants.len()
        || ships.len() % 0x180 != 0
        || ships.len() / 0x180 > u16::MAX as usize
    {
        return Err("contexto de piratas urbanos inválido".into());
    }
    let mut task = *task;
    let mut people = merchants.to_vec();
    let mut notices = Vec::new();
    let mut spawn = None;
    let flags = town.u32(off::TOWN_FLAGS);
    if flags & 0x600 != 0 {
        task.set_u8_at(0x14, task.u8_at(0x14).wrapping_sub(1));
    }
    let sum = (0..3)
        .map(|i| i32::from(town.u16(off::SATISFACTION + i * 2) as i16))
        .sum::<i32>();
    let keep = tick < task.u32_at(0xc) || (tick < task.u32_at(8) && sum > -30 && (task.u8_at(0x14) as i8) > 0);
    let near = |person: &Merchant| -> Result<Option<u16>, String> {
        if person.bytes.len() != 0x650 {
            return Err("comerciante pirata truncado".into());
        }
        let count = ships.len() / 0x180;
        let mut id = usize::from(person.u16(0xe));
        let mut seen = vec![false; count];
        while id < count {
            if seen[id] {
                return Err("lista de barcos de comerciante cíclica".into());
            }
            seen[id] = true;
            let at = id * 0x180;
            let s = &ships[at..at + 0x180];
            let state = u16::from_le_bytes([s[0x134], s[0x135]]);
            let x = i32::from(i16::from_le_bytes([s[0x1e], s[0x1f]]));
            let y = i32::from(i16::from_le_bytes([s[0x22], s[0x23]]));
            let dx = hideout.0.wrapping_sub(x);
            let dy = hideout.1.wrapping_sub(y);
            let distance = dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy));
            if state == 0xf && distance < i32::from(radius) * 64 {
                return Ok(Some(id as u16));
            }
            id = usize::from(u16::from_le_bytes([s[4], s[5]]));
        }
        Ok(None)
    };
    if task.u8_at(0x10) == 2 {
        if !keep {
            return Ok(PreparedPirates {
                task,
                merchants: people,
                notices,
                spawn,
                keep: false,
            });
        }
        task.due = task.due.wrapping_add(0x80);
        for owner in (0..usize::from(merchant_count)).rev() {
            if people[owner].bytes.len() != 0x650 {
                return Err("comerciante pirata truncado".into());
            }
            if people[owner].u16(8) != 0 {
                continue;
            }
            if let Some(ship) = near(&people[owner])? {
                let at = usize::from(ship) * 0x180;
                task.set_u8_at(0x10, 3);
                task.set_u8_at(0x16, ship as u8);
                task.set_u8_at(0x12, ships[at + 0x15e]);
                task.set_u8_at(0x13, ships[at + 0x15f]);
                task.due = task.due.wrapping_add(0x200);
                let count = task.u8_at(0x15);
                task.set_u8_at(0x15, count.wrapping_sub(1));
                spawn = Some((ship, 1));
                return Ok(PreparedPirates {
                    task,
                    merchants: people,
                    notices,
                    spawn,
                    keep: count > 1,
                });
            }
        }
        return Ok(PreparedPirates {
            task,
            merchants: people,
            notices,
            spawn,
            keep: true,
        });
    }
    task.due = task.due.wrapping_add(0x129);
    let owner = task.u8_at(0x16);
    if u16::from(owner) >= merchant_count {
        return Ok(PreparedPirates {
            task,
            merchants: people,
            notices,
            spawn,
            keep: false,
        });
    }
    let person = &mut people[usize::from(owner)];
    if person.bytes.len() != 0x650 {
        return Err("comerciante pirata truncado".into());
    }
    let name = u16::from_le_bytes([task.u8_at(0x12), task.u8_at(0x13)]);
    let city = u32::from(task.u8_at(0x11));
    let names = u32::from(person.bytes[0x1c]) | (u32::from(person.bytes[0x1d]) << 16);
    if name != u16::MAX {
        notices.push(PirateNotice {
            owner,
            kind: 0x41,
            hometown: person.bytes[0x19],
            payload_a: city | (u32::from(name) << 16),
            payload_b: names,
        });
        task.set_u8_at(0x12, 255);
        task.set_u8_at(0x13, 255);
        task.set_u8_at(0x17, person.bytes[0x1b]);
    }
    if u32::from(person.bytes[0x1b]) < u32::from(task.u8_at(0x17)) + 4 {
        if let Some(ship) = near(person)? {
            spawn = Some((ship, 0))
        }
        return Ok(PreparedPirates {
            task,
            merchants: people,
            notices,
            spawn,
            keep,
        });
    }
    notices.push(PirateNotice {
        owner,
        kind: 0x42,
        hometown: person.bytes[0x19],
        payload_a: city | (4000 << 16),
        payload_b: names,
    });
    person.set_i32(0, person.i32(0).wrapping_add(4000));
    person.set_i32(0x4c8, person.i32(0x4c8).wrapping_add(4000));
    Ok(PreparedPirates {
        task,
        merchants: people,
        notices,
        spawn,
        keep: false,
    })
}
