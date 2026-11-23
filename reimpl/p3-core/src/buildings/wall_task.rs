//! Gestor de la muralla de una ciudad fundada (`0x004ECDF4`, tarea 0x39).
use super::Site;
use crate::{
    scheduler::TaskRecord,
    town::{off, Town},
};

/// Resultado del manejador. La colocación se ejecuta en la frontera del llamante.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WallTaskResult {
    pub keep: bool,
    pub place: Option<u32>,
}

/// Comprueba las listas que utiliza el original antes de acceder al almacén.
fn chain(site: &Site, town: &Town, head: u16) -> Result<Vec<usize>, &'static str> {
    let count = usize::from(town.u16(off::BUILDING_RECORD_COUNT));
    if site.records.len() < count * 8 {
        return Err("registros de muralla truncados");
    }
    let mut seen = vec![false; count];
    let mut result = Vec::new();
    let mut i = usize::from(head);
    while i < count {
        if seen[i] {
            return Err("ciclo en lista de muralla");
        }
        seen[i] = true;
        result.push(i);
        i = usize::from(site.next(i));
    }
    Ok(result)
}

/// No modifica la tarea si la imagen de ciudad es inválida.
pub fn tick(task: &mut TaskRecord, towns: &[Town], sites: &[Site], tick: u32) -> Result<WallTaskResult, &'static str> {
    let done = WallTaskResult {
        keep: false,
        place: None,
    };
    let wait = WallTaskResult {
        keep: true,
        place: None,
    };
    let city = usize::from(task.u8_at(8));
    let Some(town) = towns.get(city) else { return Ok(done) };
    if town.u8(off::TOWN_FLAGS) & 0xe0 != 0 {
        return Ok(done);
    }
    let site = sites.get(city).ok_or("plano de muralla ausente")?;
    let wall = chain(site, town, town.u16(off::WALL_HEAD))?;
    if !wall.is_empty() {
        task.due = task.due.wrapping_add((wall.len() as u32).wrapping_mul(0x100));
        return Ok(wait);
    }
    // wall_expansion_level también recorre la lista de defensa en algunos estados.
    chain(site, town, town.u16(off::DEFENSE_HEAD))?;
    let level = site.wall_expansion_level(town);
    if level == 0xff {
        task.due = task.due.wrapping_add(0x1f00);
        return Ok(wait);
    }
    if level >= 2 {
        return Ok(done);
    }
    let construction = chain(site, town, town.u16(off::CONSTRUCTION_HEAD))?;
    let progress = construction
        .into_iter()
        .filter(|&i| site.rec(i, 3) == 0x36)
        .fold(0u32, |sum, i| sum.wrapping_add(u32::from(site.rec(i, 6))));
    if progress != 0 {
        task.due = task.due.wrapping_add(progress.wrapping_mul(4)) & !0xff;
        if task.due <= tick {
            task.due = task.due.wrapping_add(0x100);
        }
        return Ok(wait);
    }
    for point in [1, 2] {
        if Site::is_plan_point_valid(town, point) {
            task.due = task.due.wrapping_add(0x1f00);
            return Ok(WallTaskResult {
                keep: true,
                place: Some(point),
            });
        }
    }
    Ok(done)
}
