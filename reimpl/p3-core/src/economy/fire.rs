//! Propagación sin vista de la tarea `0x004E9564`.
use crate::{
    buildings::{burn::remove_from_auction, Site},
    scheduler::Scheduler,
    town::{off, Town},
};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Outcome {
    pub keep: bool,
    pub started: bool,
    pub city: u8,
    pub warehouse: Option<WarehouseLoss>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WarehouseLoss {
    pub office: usize,
    pub owner: u16,
    pub town: u8,
    pub payload: [u8; 8],
}
fn word(image: &[u8], p: usize) -> u16 {
    u16::from_le_bytes([image[p], image[p + 1]])
}
fn task_word(t: &crate::scheduler::TaskRecord, p: usize) -> u16 {
    u16::from_le_bytes([t.u8_at(p), t.u8_at(p + 1)])
}
fn put_word(t: &mut crate::scheduler::TaskRecord, p: usize, v: u16) {
    t.set_u8_at(p, v as u8);
    t.set_u8_at(p + 1, (v >> 8) as u8)
}

/// Prepara el tick en copias. Sin pool de negocios se rechazan edificios privados.
pub fn step(
    scheduler: &mut Scheduler,
    towns: &mut [Town],
    sites: &mut [Site],
    offices: &[u8],
    merchants: u16,
    special: u8,
) -> Result<Outcome, &'static str> {
    let mut images = offices.to_vec();
    step_inner(scheduler, towns, sites, &mut images, None, merchants, special, None)
}
/// Variante con recuento de oficinas y pool de negocios; los almacenes aún se rechazan.
pub fn step_with_offices(
    scheduler: &mut Scheduler,
    towns: &mut [Town],
    sites: &mut [Site],
    offices: &mut [u8],
    pool: &mut crate::economy::facility_pool::FacilityPool,
    merchants: u16,
    special: u8,
) -> Result<Outcome, &'static str> {
    step_inner(scheduler, towns, sites, offices, Some(pool), merchants, special, None)
}
/// Variante que también calcula la pérdida de almacén y devuelve la carta pendiente.
pub fn step_with_warehouses(
    scheduler: &mut Scheduler,
    towns: &mut [Town],
    sites: &mut [Site],
    offices: &mut [u8],
    pool: &mut crate::economy::facility_pool::FacilityPool,
    merchants: u16,
    special: u8,
    tick: u32,
    names: &[[u8; 2]],
) -> Result<Outcome, &'static str> {
    step_inner(
        scheduler,
        towns,
        sites,
        offices,
        Some(pool),
        merchants,
        special,
        Some((tick, names)),
    )
}
fn step_inner(
    scheduler: &mut Scheduler,
    towns: &mut [Town],
    sites: &mut [Site],
    offices: &mut [u8],
    pool: Option<&mut crate::economy::facility_pool::FacilityPool>,
    merchants: u16,
    special: u8,
    warehouse: Option<(u32, &[[u8; 2]])>,
) -> Result<Outcome, &'static str> {
    if offices.len() % 0x44c != 0 {
        return Err("oficinas truncadas");
    };
    let index = usize::from(scheduler.head);
    let mut task = *scheduler.records.get(index).ok_or("tarea de incendio ausente")?;
    if task.kind != 0x1d {
        return Err("tarea de incendio inválida");
    };
    let city = task.u8_at(0x12);
    let mut town = towns
        .get(usize::from(city))
        .ok_or("ciudad de incendio ausente")?
        .clone();
    let mut site = sites.get(usize::from(city)).ok_or("plano de incendio ausente")?.clone();
    let count = usize::from(town.u16(off::BUILDING_RECORD_COUNT));
    if site.records.len() < count * 8 {
        return Err("registros de incendio truncados");
    };
    let mut next = scheduler.clone();
    let mut images = offices.to_vec();
    let mut facilities = pool.as_deref().cloned();
    let mut loss = None;
    if task.due & 0x38 == 0 && task_word(&task, 0xe) > task_word(&task, 0xc) {
        let mut heads = vec![(town.u16(off::PUBLIC_BUILDING_HEAD), false)];
        let mut office = usize::from(town.u16(0x784));
        let offices_count = images.len() / 0x44c;
        let mut seen = vec![false; offices_count];
        while office < offices_count {
            if seen[office] {
                return Err("lista de oficinas cíclica");
            };
            seen[office] = true;
            let o = &images[office * 0x44c..(office + 1) * 0x44c];
            heads.push((word(o, 0x2d0), true));
            heads.push((word(o, 0x2ce), false));
            office = usize::from(word(o, 0x2ca));
        }
        let (mut best, mut distance) = (None, 100i32);
        for (head, house) in heads {
            let mut at = usize::from(head);
            let mut visited = vec![false; count];
            while at < count {
                if visited[at] {
                    return Err("lista de edificios cíclica");
                };
                visited[at] = true;
                let kind = site.rec(at, 3);
                if kind >= 0x3c {
                    return Err("tipo de edificio de incendio no acreditado");
                };
                let x = i32::from(site.rec(at, 0)) - i32::from(task.u8_at(0x10));
                let y = i32::from(site.rec(at, 1)) - i32::from(task.u8_at(0x11));
                let d = x * x + y * y;
                if ((4..=0x1e).contains(&kind) || matches!(kind, 0x29 | 0x2c)) && d < distance {
                    best = Some((at, house));
                    distance = d
                }
                at = usize::from(site.next(at));
            }
        }
        if let Some((target, house)) = best {
            let owner = u16::from(site.rec(target, 2));
            let private = owner < merchants;
            let mut selected = None;
            if private {
                if facilities.is_none() {
                    return Err("propagación a edificio privado pendiente");
                };
                let mut at = usize::from(town.u16(0x784));
                let mut visited = vec![false; offices_count];
                while at < offices_count {
                    if visited[at] {
                        return Err("lista de oficinas cíclica");
                    };
                    visited[at] = true;
                    let o = &images[at * 0x44c..(at + 1) * 0x44c];
                    if word(o, 0x2c4) == owner {
                        selected = Some(at);
                        break;
                    }
                    at = usize::from(word(o, 0x2ca));
                }
                if let Some(i) = selected {
                    if site.rec(target, 3) == 0x1e {
                        let (tick, names) = warehouse.ok_or("incendio de almacén pendiente")?;
                        let names = *names.get(usize::from(owner)).ok_or("nombre de propietario ausente")?;
                        let p = i * 0x44c;
                        let image = &mut images[p..p + 0x44c];
                        if let Some(payload) = crate::economy::fire_warehouse::lose(image, tick, names)? {
                            loss = Some(WarehouseLoss {
                                office: i,
                                owner,
                                town: image[0x2c6],
                                payload,
                            })
                        }
                    }
                }
            }
            let head = if let Some(i) = selected {
                word(&images[i * 0x44c..(i + 1) * 0x44c], if house { 0x2d0 } else { 0x2ce })
            } else if private {
                u16::MAX
            } else {
                town.u16(off::PUBLIC_BUILDING_HEAD)
            };
            let mut at = head;
            let mut previous = None;
            let mut visited = vec![false; count];
            while usize::from(at) < count && usize::from(at) != target {
                let i = usize::from(at);
                if visited[i] {
                    return Err("lista de edificios cíclica");
                };
                visited[i] = true;
                previous = Some(i);
                at = site.next(i)
            }
            if usize::from(at) == target {
                task.set_u8_at(0x10, site.rec(target, 0));
                task.set_u8_at(0x11, site.rec(target, 1));
                let progress = task_word(&task, 0xc).wrapping_add(1);
                put_word(&mut task, 0xc, progress);
                let successor = site.next(target);
                if let Some(p) = previous {
                    site.set_next(p, successor)
                } else if let Some(i) = selected {
                    let p = i * 0x44c + if house { 0x2d0 } else { 0x2ce };
                    images[p..p + 2].copy_from_slice(&successor.to_le_bytes())
                } else {
                    town.set_u16(off::PUBLIC_BUILDING_HEAD, successor)
                }
                site.set_next(target, town.u16(off::BURNING_HEAD));
                town.set_u16(off::BURNING_HEAD, target as u16);
                site.set_rec(target, 6, 0x32);
                remove_from_auction(&mut next, towns, city, target as u16)?;
                task.next = next.records[index].next;
            } else {
                let progress = task_word(&task, 0xc);
                put_word(&mut task, 0xe, progress)
            }
            if let Some(i) = selected {
                let p = i * 0x44c;
                let mut relocated = None;
                let mut native = crate::economy::office_recount::NativeFacilities {
                    pool: facilities.as_mut().expect("validado"),
                    relocated: Some(&mut relocated),
                };
                crate::economy::office_recount::recount(
                    (&mut images[p..p + 0x44c]).try_into().expect("oficina validada"),
                    &mut town,
                    &mut site,
                    &mut native,
                )
                .map_err(|_| "recuento de oficina de incendio inválido")?;
            }
        } else {
            let progress = task_word(&task, 0xc);
            put_word(&mut task, 0xe, progress)
        }
    }
    let mut at = town.u16(off::BURNING_HEAD);
    let mut previous = None;
    let mut visited = vec![false; count];
    while usize::from(at) < count {
        let i = usize::from(at);
        if visited[i] {
            return Err("lista de incendios cíclica");
        };
        visited[i] = true;
        let successor = site.next(i);
        let kind = site.rec(i, 3);
        if !matches!(kind, 0x32 | 0x33) {
            let progress = site.rec(i, 6);
            if progress == 0 {
                let rows = town.i32(off::PLAN_ROWS);
                let stride = town.i32(off::PLAN_STRIDE);
                let row = i32::from(site.rec(i, 0));
                let col = i32::from(site.rec(i, 1));
                if rows <= 32
                    || stride <= 32
                    || row < 16
                    || col < 16
                    || row >= rows - 16
                    || col >= stride - 16
                    || (rows as usize)
                        .checked_mul(stride as usize)
                        .is_none_or(|n| n > site.cells.len())
                    || usize::from(kind) >= 60
                {
                    return Err("huella de incendio fuera del plano admitido");
                };
                if let Some(p) = previous {
                    site.set_next(p, successor)
                } else {
                    town.set_u16(off::BURNING_HEAD, successor)
                }
                site.set_next(i, u16::MAX);
                site.free_building_record_chain(&mut town, i as u32, special);
                at = successor;
                continue;
            } else {
                site.set_rec(i, 6, if progress > 50 { progress - 51 } else { progress - 1 })
            }
        }
        previous = Some(i);
        at = successor;
    }
    task.due = task.due.wrapping_add(8);
    let keep = task_word(&task, 0xe) > task_word(&task, 0xc) || town.u16(off::BURNING_HEAD) != u16::MAX;
    let started = keep && town.u32(0x2c8) & 0x8000 == 0;
    if keep {
        if started {
            town.set_u32(0x2c8, town.u32(0x2c8) | 0x8000)
        }
    } else {
        town.set_u32(0x2c8, town.u32(0x2c8) & !0x8000)
    }
    next.records[index] = task;
    *scheduler = next;
    towns[usize::from(city)] = town;
    sites[usize::from(city)] = site;
    offices.copy_from_slice(&images);
    if let (Some(dst), Some(src)) = (pool, facilities) {
        *dst = src
    }
    Ok(Outcome {
        keep,
        started,
        city,
        warehouse: loss,
    })
}
