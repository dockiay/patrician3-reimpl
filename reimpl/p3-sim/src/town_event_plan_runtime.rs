//! Despacho conservador de la planificación urbana 0x1F.
//!
//! Las ramas sin candidato, sin acontecimiento y de cierre anual se apoyan en
//! el preludio contrastado con x86. Una selección que necesita generar el
//! evento sigue rechazándose hasta conectar también la operación original 84.
use p3_core::{
    economy::town_event_prelude::{self, Outcome},
    pump::Operation,
    rng::CrtRand,
    scheduler::TaskRecord,
    town::Town,
};

pub struct Inputs<'a> {
    pub task: TaskRecord,
    pub towns: &'a [Town],
    pub day: u16,
    pub operations_remote: bool,
    pub rng: Option<CrtRand>,
    pub active_facilities: u16,
    /// `None` indica que las imágenes de banda no están disponibles.
    pub bands: Option<[Option<u16>; 5]>,
}
#[derive(Debug)]
pub struct Prepared {
    pub task: TaskRecord,
    pub keep: bool,
    pub operation: Option<Operation>,
    pub rng: Option<CrtRand>,
}

/// Calcula todo en copias; si hay una rama aún no portada, no consume la
/// tarea, RNG ni cola de operaciones de entrada.
pub fn prepare(i: Inputs<'_>) -> Result<Prepared, String> {
    match town_event_prelude::prepare(&i.task, i.towns, i.day, i.operations_remote, i.rng)? {
        Outcome::BeforeSeason { task } => Ok(Prepared { task, keep: true, operation: None, rng: i.rng }),
        Outcome::YearClosed { task, operation, rng } => Ok(Prepared { task, keep: false, operation, rng }),
        Outcome::Candidate { task, slot, town } => {
            let city = i.towns.get(usize::from(town)).ok_or("ciudad de candidato urbano ausente")?;
            let bands = i.bands.ok_or("bandas de candidato urbano ausentes")?;
            let task = town_event_prelude::finish_without_event(&task, slot, city, i.active_facilities, bands)
                .map_err(|_| "selección de acontecimiento urbano y emisión 0x84 pendientes".to_string())?;
            Ok(Prepared { task, keep: true, operation: None, rng: i.rng })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use p3_core::town::off;

    #[test]
    fn planning_is_staged_and_blocks_unported_event() {
        let mut task = TaskRecord { kind: 0x1f, due: 100, ..Default::default() };
        task.payload.fill(255);
        task.payload[0] = 0;
        task.payload[8] = 3;
        let town = Town::default();
        let before = task;
        let run = |day, towns: &[Town]| prepare(Inputs { task, towns, day, operations_remote: false,
            rng: Some(CrtRand { state: 123 }), active_facilities: 0, bands: Some([None; 5]) });
        let early = run(10, std::slice::from_ref(&town)).unwrap();
        assert!(early.keep);
        assert_eq!(early.task.due, 0x94);
        let ordinary = run(100, std::slice::from_ref(&town)).unwrap();
        assert!(ordinary.keep);
        assert_eq!(ordinary.task.payload[0], 255);
        let end = run(304, std::slice::from_ref(&town)).unwrap();
        assert!(!end.keep);
        assert_eq!(end.operation.unwrap().0[0], 0x7e);
        assert_ne!(end.rng.unwrap().state, 123);
        let mut unhappy = town;
        for i in 0..3 {
            unhappy.set_u16(off::SATISFACTION + i * 2, (-30i16) as u16);
            unhappy.set_u16(off::SATISFACTION_TARGET + i * 2, (-30i16) as u16);
        }
        assert!(run(100, &[unhappy]).unwrap_err().contains("0x84"));
        assert_eq!(task, before);
    }
}
