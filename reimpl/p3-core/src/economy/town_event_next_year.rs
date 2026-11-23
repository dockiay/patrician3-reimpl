//! Operación `0x7E` (`0x0053F8B0`): programa el plan urbano anual.
use crate::{
    pump::Operation,
    scheduler::Scheduler,
    town::{off, Town},
};

/// Prepara la tarea sobre una copia. Los bytes que no escribe el original
/// conservan el contenido anterior de la ranura libre.
pub fn prepare(scheduler: &Scheduler, towns: &[Town], year: u16, day: u16, op: Operation) -> Result<Scheduler, String> {
    if op.0[0] != 0x7e || scheduler.records.len() != usize::from(scheduler.capacity) || towns.len() > 255 {
        return Err("operación anual urbana inválida".into());
    }
    let mut seen = vec![false; scheduler.records.len()];
    let mut at = scheduler.head;
    while usize::from(at) < seen.len() {
        if seen[usize::from(at)] {
            return Err("planificador anual cíclico".into());
        }
        seen[usize::from(at)] = true;
        at = scheduler.records[usize::from(at)].next
    }
    let cities = towns
        .iter()
        .enumerate()
        .filter_map(|(i, t)| (t.u32(off::TOWN_FLAGS) & 0x40 == 0).then_some(i as u8))
        .collect::<Vec<_>>();
    if cities.len() > 16 {
        return Err("el original escribiría fuera del registro de tarea anual".into());
    }
    if scheduler.free_head > scheduler.capacity {
        return Err("cabeza libre anual fuera de tabla".into());
    }
    if scheduler.free_head >= scheduler.capacity && scheduler.capacity > u16::MAX - 128 {
        return Err("depósito de tareas lleno".into());
    }
    let year = u32::from(year).wrapping_add(u32::from(day >= 59));
    let due = year
        .wrapping_mul(365)
        .wrapping_add(59)
        .wrapping_add(op.0[1] & 15)
        .wrapping_mul(256)
        .wrapping_add(op.0[2] & 255);
    let mut out = scheduler.clone();
    let id = out.add_task(due);
    let task = &mut out.records[usize::from(id)];
    task.kind = 0x1f;
    for (i, city) in cities.into_iter().enumerate() {
        task.payload[i] = city
    }
    for j in 0..4 {
        let bits = op.0[j + 1] as u8;
        if bits & 1 == 0 {
            task.payload.swap(j, j + 4)
        }
        if bits & 2 == 0 {
            task.payload.swap(j * 2, j * 2 + 1)
        }
        if bits & 4 == 0 {
            task.payload.swap(j, 7 - j)
        }
    }
    for j in 0..4 {
        task.payload[8 + j * 2..10 + j * 2].copy_from_slice(&(op.0[j + 1] as u16).to_le_bytes())
    }
    Ok(out)
}
