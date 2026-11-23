//! Transacción de aproximación y reposición real de una misión pirata 0x32.
use crate::{
    convoy_absorption::AbsorptionState,
    navigation::LiveVoyage,
    pirate_group_native_host::{CreateInputs, NativeHost},
    pirate_group_runtime::{self, GroupEnv},
    route_stop_runtime::RouteStopRuntime,
    HanseFleetResources,
};
use p3_core::{
    ai::{
        reputation::Merchant,
        settlement::{prepare_pirate_mission_approach, SettlementLocation},
    },
    economy::pirate_group_callbacks::CaptainResources,
    scheduler::TaskRecord,
    ships::{remove::RemoveWorld, tick::TickWorld},
    town::Town,
};
use p3_save::world::MerchantShipGroups;

pub struct Inputs<'a> {
    pub tick: u32,
    pub day: u8,
    pub day_of_year: u16,
    pub player: u32,
    pub alderman_town: u8,
    pub merchant_count: u16,
    pub world_lock: u8,
    pub network: u32,
    pub difficulty: u8,
    pub thresholds: Option<[u8; 256]>,
    pub names: Option<(u16, u16)>,
    pub resources: Option<&'a HanseFleetResources>,
    pub locations: &'a [SettlementLocation; 40],
}
pub struct Staged {
    pub task: TaskRecord,
    pub nav: LiveVoyage,
    pub bands: AbsorptionState,
    pub towns: Vec<Town>,
    pub merchants: Vec<Merchant>,
    pub pool: RouteStopRuntime,
    pub groups: Vec<MerchantShipGroups>,
    pub changed_groups: Vec<usize>,
}
/// Ejecuta cada pasada completa de reposición en copias, en el mismo orden que
/// los barcos del comerciante. El llamador confirma todos los campos juntos.
pub fn prepare(
    task: &TaskRecord,
    nav: &LiveVoyage,
    bands: &AbsorptionState,
    towns: &[Town],
    merchants: &[Merchant],
    pool: &RouteStopRuntime,
    groups: &[MerchantShipGroups],
    input: Inputs<'_>,
) -> Result<Option<Staged>, String> {
    if bands.bands.len() != 5 || nav.snapshot.ships.len() % 0x180 != 0 {
        return Err("bandas o flota incompletas".into());
    }
    let targets = std::array::from_fn(|i| bands.bands[i].as_ref().map(|b| b.header[6]));
    let owner = usize::from(task.u8_at(0xd));
    if owner >= usize::from(input.merchant_count) {
        return Ok(None);
    }
    let merchant = merchants.get(owner).ok_or("comerciante de misión ausente")?;
    let ships = nav
        .snapshot
        .ships
        .chunks_exact(0x180)
        .map(|s| s.to_vec())
        .collect::<Vec<_>>();
    let Some(approach) =
        prepare_pirate_mission_approach(task, merchant, &targets, input.locations, &ships, merchant.u16(0xe))
            .map_err(str::to_owned)?
    else {
        return Ok(None);
    };
    let band_count = targets.iter().filter(|b| b.is_some()).count();
    if band_count == 0 || approach.replenishments.len() % band_count != 0 {
        return Err("secuencia de reposición incompleta".into());
    }
    let target = u32::from(task.u8_at(0x11));
    let expected = targets
        .iter()
        .enumerate()
        .filter_map(|(i, v)| v.map(|_| (i, target)))
        .collect::<Vec<_>>();
    for calls in approach.replenishments.chunks_exact(band_count) {
        if calls != expected {
            return Err("orden de reposición incompatible".into());
        }
    }
    // El original sale antes de leer la tabla si todas las bandas del
    // destino ya tienen plano. Si alguna no lo tiene, la tabla es necesaria.
    let reads_threshold = bands
        .bands
        .iter()
        .flatten()
        .any(|band| u32::from(band.header[6]) == target && band.plan.is_none());
    let thresholds = match input.thresholds {
        Some(value) => value,
        None if !reads_threshold => [0; 256],
        None => return Err("tabla original de cuadrillas ausente".into()),
    };
    let mut out = Staged {
        task: approach.task,
        nav: nav.clone(),
        bands: bands.clone(),
        towns: towns.to_vec(),
        merchants: merchants.to_vec(),
        pool: pool.clone(),
        groups: groups.to_vec(),
        changed_groups: Vec::new(),
    };
    out.merchants[owner] = approach.merchant;
    for _ in approach.replenishments.chunks_exact(band_count) {
        let town0 = out.towns.first().cloned();
        let weapons = out
            .towns
            .get(usize::from(input.alderman_town))
            .map(|town| town.u32(0x810));
        let create = match (town0, weapons, input.resources) {
            (Some(town0), Some(weapon_experience), Some(r)) => {
                r.ports.first().copied().flatten().map(|port0| CreateInputs {
                    weapon_experience,
                    town0,
                    port0,
                    ports: input.locations.iter().map(|p| (p.x as u16, p.y as u16)).collect(),
                    ship_names: r.ship_names.clone(),
                })
            }
            _ => None,
        };
        let (first_names, last_names) = input.names.unwrap_or((0, 0));
        let population = out.towns.get(usize::from(input.day_of_year & 15)).map(|t| t.u32(0x2e0));
        let captain = CaptainResources {
            tick: input.tick,
            day_of_year: input.day_of_year,
            day: input.day,
            population: population.unwrap_or(0),
            first_names,
            last_names,
        };
        let parsed =
            p3_save::tail::RouteStopPool::parse(out.pool.bytes()).map_err(|e| format!("depósito de rutas: {e:?}"))?;
        let host = NativeHost {
            captain_population_available: population.is_some(),
            captain,
            create,
            player: input.player,
            pool_limit: parsed.count,
            cleared_routes: vec![],
            remove_events: vec![],
            removed_plans: vec![],
            merchant_owners: vec![],
            operations: vec![],
        };
        let env = GroupEnv {
            calendar_byte: input.day_of_year as u8,
            suppress_events: input.world_lock,
            operation_host: input.network,
            level_thresholds: thresholds,
            extra_memory: vec![],
        };
        let mut tb = out.towns.iter().flat_map(|t| t.0.iter().copied()).collect::<Vec<_>>();
        let mut mb = out
            .merchants
            .iter()
            .flat_map(|m| m.bytes.iter().copied())
            .collect::<Vec<_>>();
        let snapshot = &mut out.nav.snapshot;
        let mut remove = RemoveWorld {
            manager: &mut snapshot.manager,
            ships: &mut snapshot.ships,
            captains: &mut snapshot.captains,
            convoys: &mut snapshot.convoys,
            towns: &mut tb,
            merchants: &mut mb,
            pool_limit: parsed.count,
            merchant_count: input.merchant_count,
            town_count: out.towns.len() as u32,
            ui_a: false,
            ui_b: false,
        };
        let world = TickWorld {
            remove: &mut remove,
            time: input.tick,
            player: input.player,
        };
        let prepared =
            pirate_group_runtime::prepare_replenish(&world, &out.bands, target as u16, input.difficulty, &env, &host)?;
        let effects =
            crate::pirate_group_effects::prepare(&prepared, out.pool.bytes(), &out.nav.snapshot.routes, &out.groups)?;
        let old_pool = out.pool.bytes().to_vec();
        out.pool.replace(&old_pool, effects.route_stop_pool)?;
        out.nav.snapshot.manager = prepared.manager;
        out.nav.snapshot.ships = prepared.ships;
        out.nav.snapshot.captains = prepared.captains;
        out.nav.snapshot.convoys = prepared.convoys;
        out.nav.snapshot.routes = effects.routes;
        crate::navigation::refresh_lists(&mut out.nav.snapshot)?;
        for (t, src) in out.towns.iter_mut().zip(prepared.towns.chunks_exact(0x9f8)) {
            t.0.copy_from_slice(src)
        }
        for (m, src) in out.merchants.iter_mut().zip(effects.merchants.chunks_exact(0x650)) {
            m.bytes.copy_from_slice(src)
        }
        out.groups = effects.merchant_groups;
        for id in effects.changed_groups {
            if !out.changed_groups.contains(&id) {
                out.changed_groups.push(id)
            }
        }
        out.bands = prepared.state;
        out.bands.operations.extend(effects.operations);
    }
    Ok(Some(out))
}
