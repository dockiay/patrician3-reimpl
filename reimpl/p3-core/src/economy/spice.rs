//! Llegada periódica de especias, `0x004DB330`.
use crate::{ai::settlement::SettlementLocation, pump::Operation, rng::CrtRand, scheduler::TaskRecord, town::Town};

/// Ejecuta la tarea; los errores dejan tarea, ciudades y CRT sin cambios.
pub fn arrival(
    task: &mut TaskRecord,
    towns: &mut [Town],
    town_ids: &[u8],
    locations: Option<&[SettlementLocation; 40]>,
    client: bool,
    mode: u32,
    rng: &mut Option<CrtRand>,
) -> Result<(bool, Option<Operation>), &'static str> {
    if towns.len() > 40 || town_ids.len() < towns.len() {
        return Err("tabla de ciudades inválida");
    }
    let mut record = *task;
    let mut cities = towns.to_vec();
    let mut random = *rng;
    let old_city = usize::from(record.u8_at(0x10));
    if let Some(town) = cities.get_mut(old_city) {
        town.set_u16(0x996, town.u16(0x996).wrapping_add(1));
        town.set_u32(0x1c, town.u32(0x1c).wrapping_add(record.u32_at(0x0c)));
    }
    if client {
        *task = record;
        towns.clone_from_slice(&cities);
        return Ok((false, None));
    }
    let generator = random.as_mut().ok_or("estado CRT ausente")?;
    let loc = locations.ok_or("faltan coordenadas de StadtDaten.ini")?;
    let mut demand = 0u32;
    let mut stocks = 0u32;
    for city in &cities {
        demand = demand.wrapping_add(city.u32(0x328));
        stocks = stocks.wrapping_add(city.u32(0x1c));
    }
    let total = demand.wrapping_mul(40);
    record.set_u32_at(
        0x0c,
        if (total as i32) > (stocks as i32) {
            total.wrapping_sub(stocks)
        } else {
            0
        },
    );
    record.set_u32_at(
        8,
        record
            .due
            .wrapping_add(0xa00)
            .wrapping_add(generator.rand() as u32 & 0x3ff),
    );
    let mut distance = [u32::MAX; 4];
    let mut nearest = [0u8; 4];
    for i in (0..cities.len()).rev() {
        let position = loc
            .get(usize::from(town_ids[i]))
            .ok_or("identificador de ciudad inválido")?;
        let x = position.x as u32;
        let y = 1500u32.wrapping_sub(position.y as u32);
        let square = x.wrapping_mul(x).wrapping_add(y.wrapping_mul(y));
        for j in 0..4 {
            if distance[j] > square {
                for k in (j + 1..4).rev() {
                    distance[k] = distance[k - 1];
                    nearest[k] = nearest[k - 1];
                }
                distance[j] = square;
                nearest[j] = i as u8;
                break;
            }
        }
    }
    let selected = nearest[(generator.rand() as usize) & 3];
    record.set_u8_at(0x10, selected);
    let op = if mode & 0xc != 0 {
        Some(Operation([
            0x83,
            record.u32_at(8),
            record.u32_at(0xc),
            u32::from(selected),
            0,
        ]))
    } else {
        record.due = record.u32_at(8);
        None
    };
    *task = record;
    towns.clone_from_slice(&cities);
    *rng = random;
    Ok((op.is_none(), op))
}

/// `0x005367B5`: crea la siguiente tarea cuando se consume 0x83.
pub fn apply(scheduler: &mut crate::scheduler::Scheduler, op: Operation) -> Result<(), &'static str> {
    if scheduler.free_head >= scheduler.capacity && scheduler.capacity > u16::MAX - 0x80 {
        return Err("planificador lleno");
    }
    let i = scheduler.add_task(op.0[1]);
    let record = &mut scheduler.records[usize::from(i)];
    record.kind = 0;
    record.set_u32_at(8, op.0[1]);
    record.set_u32_at(0xc, op.0[2]);
    record.set_u8_at(0x10, op.0[3] as u8);
    Ok(())
}
