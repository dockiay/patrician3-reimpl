//! Tick de peste `0x004E9094`, sin entrega de avisos ni interfaz.
use crate::{scheduler::TaskRecord, town::Town};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Outcome {
    pub keep: bool,
    pub started: bool,
    pub city: u8,
}

/// Prepara los efectos económicos y deja sin modificar las entradas si una
/// cadena es inválida o si hay que resolver la muerte de un administrador.
pub fn step(
    task: &mut TaskRecord,
    towns: &mut [Town],
    offices: &[u8],
    businesses: &mut [[u8; 20]],
    captains: u16,
    tick: u32,
) -> Result<Outcome, &'static str> {
    let mut images = offices.to_vec();
    step_inner(task, towns, &mut images, businesses, captains, None, tick)
}

/// Variante con lista libre de capitanes y cambios de administrador de oficina.
pub fn step_with_captains(
    task: &mut TaskRecord,
    towns: &mut [Town],
    offices: &mut [u8],
    businesses: &mut [[u8; 20]],
    captains: &mut crate::ships::captains::CaptainPool,
    tick: u32,
) -> Result<Outcome, &'static str> {
    step_inner(
        task,
        towns,
        offices,
        businesses,
        captains.capacity,
        Some(captains),
        tick,
    )
}

fn step_inner(
    task: &mut TaskRecord,
    towns: &mut [Town],
    offices: &mut [u8],
    businesses: &mut [[u8; 20]],
    captain_count: u16,
    captains: Option<&mut crate::ships::captains::CaptainPool>,
    tick: u32,
) -> Result<Outcome, &'static str> {
    if offices.len() % 0x44c != 0 {
        return Err("oficinas truncadas");
    }
    let city = task.u8_at(0x16);
    if usize::from(city) >= towns.len() {
        return Ok(Outcome {
            keep: false,
            started: false,
            city,
        });
    }
    let mut record = *task;
    let mut town = towns[usize::from(city)].clone();
    let mut pool = businesses.to_vec();
    let mut images = offices.to_vec();
    let mut people = captains.as_deref().cloned();
    if people
        .as_ref()
        .is_some_and(|p| p.entries.len() != usize::from(p.capacity) * 16)
    {
        return Err("pool de capitanes truncado");
    };
    let old = record.due;
    let byte = record.u8_at(8 + (old as usize & 3));
    record.due = old.wrapping_add(u32::from(byte) + 1);
    if (record.due & 255) < u32::from(byte) {
        record.set_u32_at(0x10, ((town.i32(0x2d4) / 200) as u16 as u32) << 16)
    }
    let progress = u16::from_le_bytes([record.u8_at(0x10), record.u8_at(0x11)]);
    let strength = u16::from_le_bytes([record.u8_at(0x12), record.u8_at(0x13)]);
    let mut deaths = ((i32::from(strength) >> 2) + 4) & !3;
    if i32::from(progress) + deaths >= i32::from(strength) {
        deaths = i32::from(strength) - i32::from(progress)
    }
    let mut done = progress;
    if town.i32(0x2e4) < 4 {
        town.set_i32(0x2e4, town.i32(0x2e4).wrapping_add(4));
        deaths = deaths.wrapping_add(4);
        if done >= 4 {
            done -= 4
        } else {
            let extra = strength.wrapping_add(4);
            record.set_u8_at(0x12, extra as u8);
            record.set_u8_at(0x13, (extra >> 8) as u8)
        }
    }
    let office_count = images.len() / 0x44c;
    let mut office = usize::from(town.u16(0x784));
    let mut seen = vec![false; office_count];
    while office < office_count && deaths > 3 {
        if seen[office] {
            return Err("lista de oficinas cíclica");
        }
        seen[office] = true;
        let image = &images[office * 0x44c..(office + 1) * 0x44c];
        let mut facility = usize::from(u16::from_le_bytes([image[0x2cc], image[0x2cd]]));
        let next = usize::from(u16::from_le_bytes([image[0x2ca], image[0x2cb]]));
        let mut visited = vec![false; pool.len()];
        while facility < pool.len() && deaths > 3 {
            if visited[facility] {
                return Err("lista de negocios cíclica");
            }
            visited[facility] = true;
            let f = &mut pool[facility];
            let link = u16::from_le_bytes([f[8], f[9]]);
            let employees = u16::from_le_bytes([f[4], f[5]]);
            if u32::from(link & 3) == tick & 3 && employees > 1 && town.i32(0x2e0) > 4 {
                f[4..6].copy_from_slice(&(employees - 1).to_le_bytes());
                deaths -= 4;
                done = done.wrapping_add(4);
                town.set_i32(0x2e0, town.i32(0x2e0).wrapping_sub(4))
            }
            facility = usize::from(link)
        }
        let administrator = u16::from_le_bytes([image[0x2f2], image[0x2f3]]);
        if (tick ^ (next as u32)) & 0x7f == 0 && administrator < captain_count {
            let people = people.as_mut().ok_or("baja de administrador por peste pendiente")?;
            let mut cursor = people.free_head;
            let mut visited = vec![false; usize::from(people.capacity)];
            while cursor < people.capacity {
                let i = usize::from(cursor);
                if visited[i] {
                    return Err("lista de capitanes cíclica");
                }
                visited[i] = true;
                cursor = u16::from_le_bytes([people.entries[i * 16], people.entries[i * 16 + 1]])
            }
            let mut slot = administrator;
            people.free(&mut slot);
            let p = office * 0x44c;
            images[p + 0x2f2..p + 0x2f4].copy_from_slice(&slot.to_le_bytes());
            images[p + 0x2d6] &= 0xfc;
        }
        office = next;
    }
    let mut facility = (tick & 3) as usize + 1;
    while facility < 21 && deaths > 3 {
        let p = 0x844 + facility * 16;
        let employees = town.u16(p);
        if employees > 1 {
            if town.i32(0x2e0) <= 4 {
                break;
            }
            town.set_u16(p, employees - 1);
            deaths -= 4;
            done = done.wrapping_add(4);
            town.set_i32(0x2e0, town.i32(0x2e0).wrapping_sub(4))
        }
        facility += 1 + usize::from(record.u8_at(8 + (tick as usize & 3)) & 1)
    }
    if deaths > 0 {
        let beggars = town.i32(0x2e4);
        if deaths < beggars {
            town.set_i32(0x2e4, beggars.wrapping_sub(deaths));
            done = done.wrapping_add(deaths as u16)
        } else {
            done = done.wrapping_add((beggars as u16).wrapping_sub(1));
            town.set_i32(0x2e4, 1)
        }
    }
    record.set_u8_at(0x10, done as u8);
    record.set_u8_at(0x11, (done >> 8) as u8);
    let mut population = 0i32;
    for p in [0x2d8, 0x2dc, 0x2e0, 0x2e4] {
        population = population.wrapping_add(town.i32(p))
    }
    town.set_i32(0x2d4, population);
    let keep = record.due <= record.u32_at(0xc);
    let started = keep && town.u32(0x2c8) & 8 == 0;
    if keep {
        if started {
            town.set_u32(0x2c8, town.u32(0x2c8) | 8)
        }
    } else {
        town.set_u32(0x2c8, town.u32(0x2c8) & !8)
    }
    *task = record;
    towns[usize::from(city)] = town;
    businesses.copy_from_slice(&pool);
    offices.copy_from_slice(&images);
    if let (Some(dst), Some(src)) = (captains, people) {
        *dst = src
    }
    Ok(Outcome { keep, started, city })
}
