//! Une el suceso urbano, entrega de cartas y alta pirata en copias.
use crate::{town_event_runtime, town_pirate_spawn};
use p3_save::pirate_bands::PirateBand;

pub struct NavalInputs<'a> {
    pub manager: &'a [u8],
    pub captains: &'a [u8],
    pub convoys: &'a [u8],
    pub towns: &'a [u8],
    pub bands: &'a [Option<PirateBand>],
    pub resources: town_pirate_spawn::Resources<'a>,
}
pub struct Prepared {
    pub event: town_event_runtime::Prepared,
    pub fleet: Option<town_pirate_spawn::Prepared>,
}

/// No publica el evento si falla la creación o la banda no está disponible.
pub fn prepare(event: town_event_runtime::Inputs<'_>, naval: NavalInputs<'_>) -> Result<Prepared, String> {
    let staged = town_event_runtime::prepare_deferred(event)?;
    let Some((target, mode)) = staged.spawn else {
        return Ok(Prepared {
            event: staged,
            fleet: None,
        });
    };
    let city = usize::from(event.task.u8_at(0x11));
    let mut towns = naval.towns.to_vec();
    let dst = towns
        .get_mut(city * 0x9f8..(city + 1) * 0x9f8)
        .ok_or("ciudad naval truncada")?;
    if dst != event.town.0.as_slice() {
        return Err("ciudad naval desincronizada".into());
    }
    dst.copy_from_slice(staged.town.0.as_slice());
    let merchants = staged
        .merchants
        .iter()
        .flat_map(|m| m.bytes.iter().copied())
        .collect::<Vec<_>>();
    let fleet = town_pirate_spawn::prepare(town_pirate_spawn::Inputs {
        manager: naval.manager,
        ships: event.ships,
        captains: naval.captains,
        convoys: naval.convoys,
        merchants: &merchants,
        towns: &towns,
        merchant_count: event.merchant_count,
        bands: naval.bands,
        target,
        mode,
        resources: naval.resources,
    })?;
    Ok(Prepared {
        event: staged,
        fleet: Some(fleet),
    })
}
