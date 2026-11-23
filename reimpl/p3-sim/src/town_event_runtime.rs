//! Preparación transaccional de 0x1E y sus cartas S05.
use p3_core::{
    ai::reputation::Merchant, calendar::Calendar, economy::town_event_tick, scheduler::TaskRecord, town::Town,
};
use p3_state::messages::{self, Payload};

#[derive(Clone, Copy)]
pub struct Inputs<'a> {
    pub task: TaskRecord,
    pub town: &'a Town,
    pub offices: &'a [Vec<u8>],
    pub merchants: &'a [Merchant],
    pub merchant_count: u16,
    pub ships: &'a [u8],
    pub radius: u16,
    pub hideout: Option<(i32, i32)>,
    pub tick: u32,
    pub pool: &'a [u8],
}
pub struct Prepared {
    pub task: TaskRecord,
    pub town: Town,
    pub offices: Vec<Vec<u8>>,
    pub merchants: Vec<Merchant>,
    pub pool: Vec<u8>,
    pub keep: bool,
    pub spawn: Option<(u16, u8)>,
}

/// Las altas de barco pirata se rechazan hasta conectar el host de 0x5063C0.
/// Los avisos, oficinas y comerciantes se preparan en copias con la tarea.
pub fn prepare(i: Inputs<'_>) -> Result<Prepared, String> {
    prepare_inner(i, false)
}

/// Devuelve el alta pendiente solo a un llamante que vaya a prepararla y
/// confirmar ambos resultados en la misma transacción.
pub fn prepare_deferred(i: Inputs<'_>) -> Result<Prepared, String> {
    prepare_inner(i, true)
}
fn prepare_inner(i: Inputs<'_>, allow_spawn: bool) -> Result<Prepared, String> {
    let mut out = Prepared {
        task: i.task,
        town: i.town.clone(),
        offices: i.offices.to_vec(),
        merchants: i.merchants.to_vec(),
        pool: i.pool.to_vec(),
        keep: false,
        spawn: None,
    };
    let day = Calendar::for_tick(i.tick);
    let retention = [u32::MAX, 0x2800, 0x1400, 0xa00, 0x500, 0];
    match i.task.u8_at(0x10) {
        0 | 1 => {
            let result = town_event_tick::prepare(&i.task, i.town, i.tick)?;
            out.task = result.task;
            out.town = result.town;
            out.keep = result.keep;
        }
        4 => {
            let result = town_event_tick::prepare_raid(&i.task, i.town, i.offices, i.merchants, i.tick)?;
            out.task = result.task;
            out.town = result.town;
            out.offices = result.offices;
            out.keep = result.keep;
            for n in result.notices {
                let person = out
                    .merchants
                    .get_mut(usize::from(n.owner))
                    .ok_or("destinatario de saqueo ausente")?;
                let mut record = [0u8; 16];
                record[4] = 0x43;
                record[5] = n.hometown;
                record[8..10].copy_from_slice(&u16::from(n.city).to_le_bytes());
                messages::post_to_merchant(
                    &mut out.pool,
                    person,
                    day.day,
                    day.month,
                    day.year,
                    &record,
                    Payload::None,
                    &retention,
                )?;
            }
        }
        2 | 3 => {
            let hideout = i.hideout.ok_or("escondite pirata ausente")?;
            let result = town_event_tick::prepare_pirates(
                &i.task,
                i.town,
                i.merchants,
                i.merchant_count,
                i.ships,
                i.radius,
                hideout,
                i.tick,
            )?;
            if result.spawn.is_some() && !allow_spawn {
                return Err("alta de pirata urbano no conectada".into());
            }
            out.spawn = result.spawn;
            out.task = result.task;
            out.merchants = result.merchants;
            out.keep = result.keep;
            for n in result.notices {
                let person = out
                    .merchants
                    .get_mut(usize::from(n.owner))
                    .ok_or("destinatario de piratas ausente")?;
                let mut record = [0u8; 16];
                record[4] = n.kind;
                record[5] = n.hometown;
                record[8..12].copy_from_slice(&n.payload_a.to_le_bytes());
                record[12..16].copy_from_slice(&n.payload_b.to_le_bytes());
                messages::post_to_merchant(
                    &mut out.pool,
                    person,
                    day.day,
                    day.month,
                    day.year,
                    &record,
                    Payload::None,
                    &retention,
                )?;
            }
        }
        _ => return Err("tipo de acontecimiento urbano no conectado".into()),
    }
    Ok(out)
}
