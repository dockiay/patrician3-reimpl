//! Selección y cierre de la tarea urbana anual 0x1F (0x004E0334).
//! La evaluación y ejecución del acontecimiento queda explícitamente separada.
use crate::{
    pump::Operation,
    rng::CrtRand,
    scheduler::TaskRecord,
    town::{off, Town},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Outcome {
    BeforeSeason {
        task: TaskRecord,
    },
    YearClosed {
        task: TaskRecord,
        operation: Option<Operation>,
        rng: Option<CrtRand>,
    },
    Candidate {
        task: TaskRecord,
        slot: usize,
        town: u8,
    },
}

/// No modifica el registro ni el RNG de entrada. El llamante debe rechazar
/// `Candidate` hasta completar su evento, y publicar la operación junto al reloj.
pub fn prepare(
    task: &TaskRecord,
    towns: &[Town],
    day: u16,
    operations_remote: bool,
    rng: Option<CrtRand>,
) -> Result<Outcome, String> {
    if task.kind != 0x1f {
        return Err("tarea de sucesos urbanos inválida".into());
    }
    let mut out = *task;
    if day < 59 {
        out.due = 0x94;
        return Ok(Outcome::BeforeSeason { task: out });
    }
    let mut slot = 0usize;
    let mut empty = 0usize;
    while slot < 8 {
        let city = usize::from(out.payload[slot]);
        if city < towns.len() {
            if towns[city].u32(off::TOWN_FLAGS) & 0x40 == 0 {
                break;
            }
            out.payload[slot] = 0xff;
        } else if out.payload[8 + slot] == 0 {
            empty += 1;
        }
        slot += 1;
    }
    if day >= 304 || slot >= 8 || empty > 1 {
        let mut r = rng;
        let operation = if operations_remote {
            None
        } else {
            let random = r.as_mut().ok_or("estado CRT de sucesos urbanos ausente")?;
            Some(Operation([
                0x7e,
                random.rand() as u32,
                random.rand() as u32,
                random.rand() as u32,
                random.rand() as u32,
            ]))
        };
        return Ok(Outcome::YearClosed {
            task: out,
            operation,
            rng: r,
        });
    }
    Ok(Outcome::Candidate {
        town: out.payload[slot],
        task: out,
        slot,
    })
}

/// Resuelve la rama sin acontecimiento cuando todas sus condiciones pueden
/// acreditarse sin consultar las listas de oficinas o negocios. En cualquier
/// otra entrada deja al llamante la rama de evento todavía no portada.
pub fn finish_without_event(
    task: &TaskRecord,
    slot: usize,
    town: &Town,
    active_facilities: u16,
    bands: [Option<u16>; 5],
) -> Result<TaskRecord, String> {
    if task.kind != 0x1f || slot >= 8 {
        return Err("candidato urbano inválido".into());
    }
    let satisfaction = |at: usize| town.u16(at) as i16;
    let low = (0..3).all(|i| satisfaction(off::SATISFACTION + i * 2) < -20)
        && satisfaction(off::SATISFACTION_TARGET) < -20
        && satisfaction(off::SATISFACTION_TARGET + 2) < -22
        && satisfaction(off::SATISFACTION_TARGET + 4) < -24;
    let high = satisfaction(off::SATISFACTION) > 10
        && satisfaction(off::SATISFACTION + 2) > 20
        && satisfaction(off::SATISFACTION + 4) > 20
        && bands.into_iter().flatten().any(|strength| strength > 500);
    let facility_candidate = active_facilities != 0 && (4..21).any(|i| town.u16(off::FACILITY + i * 16 + 8) > 950);
    let producer_candidate = [1usize, 5, 7, 9, 12, 14, 18]
        .into_iter()
        .any(|ware| town.i32(off::TRADE_FLOW_AVERAGE + ware * 4) > 0);
    if low || high || facility_candidate || producer_candidate {
        return Err("evaluación de acontecimiento urbano pendiente".into());
    }
    let mut out = *task;
    let seed = out.payload[8 + slot];
    out.payload[slot] = 0xff;
    out.due = out
        .due
        .wrapping_add((u32::from(seed & 7) + 20) * 0x100)
        .wrapping_add(u32::from(seed));
    Ok(out)
}
