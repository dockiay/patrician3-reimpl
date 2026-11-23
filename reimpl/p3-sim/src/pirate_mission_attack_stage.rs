//! Preparación conjunta del atacante y la guarida antes de abrir la escena.
use crate::{convoy_absorption::AbsorptionState, pirate_group_runtime};
use p3_core::{
    ai::settlement::SettlementLocation,
    scheduler::TaskRecord,
    ships::{
        convoy_nudge, pirate_engage,
        pirate_mission_attack::{self, Attack, Prepared},
        remove::RemoveWorld,
        tick::TickWorld,
    },
};

pub enum Stage {
    Wait(TaskRecord),
    Attack { attack: Attack, bands: AbsorptionState, captains: Vec<u8> },
}

/// No publica cambios. El consumidor debe completar la escena y publicar sus
/// efectos, flota, banda y tarea en una sola transacción.
pub fn prepare(
    task: &TaskRecord,
    targets: &[Option<u8>; 5],
    locations: &[SettlementLocation; 40],
    direction: u8,
    head: u16,
    world: &TickWorld<'_, '_>,
    bands: &AbsorptionState,
    alderman_town: u8,
    existing_scene: bool,
) -> Result<Option<Stage>, String> {
    prepare_with_stack(
        task,
        targets,
        locations,
        direction,
        head,
        world,
        bands,
        alderman_town,
        existing_scene,
        None,
    )
}

/// La palabra de pila es explícita porque el original puede leerla sin
/// inicializar al crear convoy para el primer barco suelto.
pub fn prepare_with_stack(
    task: &TaskRecord,
    targets: &[Option<u8>; 5],
    locations: &[SettlementLocation; 40],
    direction: u8,
    head: u16,
    world: &TickWorld<'_, '_>,
    bands: &AbsorptionState,
    alderman_town: u8,
    existing_scene: bool,
    stack_initial: Option<u16>,
) -> Result<Option<Stage>, String> {
    prepare_with_stack_and_plans(task, targets, locations, direction, head, world, bands,
        alderman_town, existing_scene, stack_initial, None)
}

/// Recursos del plano solo se consultan cuando realmente empieza un asalto.
pub fn prepare_with_stack_and_plans(
    task: &TaskRecord,
    targets: &[Option<u8>; 5],
    locations: &[SettlementLocation; 40],
    direction: u8,
    head: u16,
    world: &TickWorld<'_, '_>,
    bands: &AbsorptionState,
    alderman_town: u8,
    existing_scene: bool,
    stack_initial: Option<u16>,
    plans: Option<(&crate::town_plans::TownPlanBank, (u8, u8))>,
) -> Result<Option<Stage>, String> {
    prepare_with_factory(task, targets, locations, direction, head, world, bands,
        alderman_town, existing_scene, stack_initial, plans, None)
}

/// La fábrica solo se consulta si la guarida necesita crear un barco nuevo.
pub fn prepare_with_factory(
    task: &TaskRecord,
    targets: &[Option<u8>; 5],
    locations: &[SettlementLocation; 40],
    direction: u8,
    head: u16,
    world: &TickWorld<'_, '_>,
    bands: &AbsorptionState,
    alderman_town: u8,
    existing_scene: bool,
    stack_initial: Option<u16>,
    plans: Option<(&crate::town_plans::TownPlanBank, (u8, u8))>,
    factory: Option<&crate::pirate_group_native_host::NativeHost>,
) -> Result<Option<Stage>, String> {
    let prepared = pirate_mission_attack::prepare_with_single(
        task,
        targets,
        locations,
        direction,
        head,
        world.remove.manager,
        world.remove.ships,
        world.remove.convoys,
        stack_initial,
    )
    .map_err(str::to_owned)?;
    let Some(prepared) = prepared else { return Ok(None) };
    let mut attack = match prepared {
        Prepared::Wait(task) => return Ok(Some(Stage::Wait(task))),
        Prepared::Attack(attack) => attack,
        Prepared::SingleShip {
            mut task,
            ship,
            output_initial,
        } => {
            let mut single = pirate_engage::prepare_pirate_mission_single_ship_convoy(world, ship, output_initial)?;
            if single.captains != world.remove.captains
                || single.merchants != world.remove.merchants
                || single.towns != world.remove.towns
            {
                return Err("la creación de convoy alteró vistas no conectadas".into());
            }
            if single.ui_ships.len() > 1 {
                return Err("avisos de barco suelto inesperados".into());
            }
            convoy_nudge::nudge(
                &single.manager,
                &mut single.ships,
                &single.convoys,
                u32::from(single.convoy),
                direction,
            )
            .map_err(str::to_owned)?;
            let at = usize::from(single.convoy) * 0x3c;
            let word = |i| u16::from_le_bytes([single.convoys[at + i], single.convoys[at + i + 1]]);
            let head = word(0xa);
            let leader = word(0x10);
            let owner = single.convoys[at];
            task.due = task.due.wrapping_add(0x40);
            Attack {
                ship,
                convoy: single.convoy,
                head,
                leader,
                owner,
                task,
                manager: single.manager,
                ships: single.ships,
                convoys: single.convoys,
                refresh_ship: single.ui_ships.first().copied(),
            }
        }
    };
    let mut staged_bands = bands.clone();
    let mut staged_captains = world.remove.captains.to_vec();
    if !existing_scene {
        let band_index = usize::from(task.u8_at(0x10));
        let band = staged_bands.bands.get_mut(band_index).and_then(Option::as_mut)
            .ok_or("cuadrilla ausente")?;
        if band.plan.is_none() {
            let (bank, bytes) = plans.ok_or("recursos del plano pirata ausentes")?;
            *band = pirate_group_runtime::prepare_missing_hideout_plan(band, bank, world.time, bytes)?;
        }
        let mut caps = world.remove.captains.to_vec();
        let mut merchants = world.remove.merchants.to_vec();
        let mut towns = world.remove.towns.to_vec();
        let mut remove = RemoveWorld {
            manager: &mut attack.manager,
            ships: &mut attack.ships,
            captains: &mut caps,
            convoys: &mut attack.convoys,
            merchants: &mut merchants,
            towns: &mut towns,
            pool_limit: world.remove.pool_limit,
            merchant_count: world.remove.merchant_count,
            town_count: world.remove.town_count,
            ui_a: world.remove.ui_a,
            ui_b: world.remove.ui_b,
        };
        let staged_world = TickWorld {
            remove: &mut remove,
            time: world.time,
            player: world.player,
        };
        let band = staged_bands.bands[band_index].as_ref().ok_or("cuadrilla ausente")?;
        let word = |v: &[u8], at: usize| u16::from_le_bytes([v[at], v[at + 1]]);
        let needs_ship = word(&band.header, 8) >= word(staged_world.remove.manager, 0xf6)
            && word(&band.header, 10) >= word(staged_world.remove.manager, 0xf4);
        let (manager, ships, convoys, captains, state) = if needs_ship {
            let factory = factory.ok_or("recursos de creación de reserva ausentes")?;
            let out = pirate_group_runtime::prepare_hideout_new_reserve(
                &staged_world, &staged_bands, band_index, alderman_town, factory, locations)?;
            (out.manager, out.ships, out.convoys, out.captains, out.state)
        } else {
            let out = pirate_group_runtime::prepare_hideout_with_ports(
                &staged_world, &staged_bands, band_index, alderman_town, &(), Some(locations))?;
            (out.manager, out.ships, out.convoys, out.captains, out.state)
        };
        attack.manager = manager;
        attack.ships = ships;
        attack.convoys = convoys;
        staged_captains = captains;
        staged_bands = state;
    }
    Ok(Some(Stage::Attack {
        attack,
        bands: staged_bands,
        captains: staged_captains,
    }))
}
