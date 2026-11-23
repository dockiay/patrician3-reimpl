//! Puente atómico de B4 a flota viva, factoría, rutas, grupos y cola.
use crate::{
    convoy_absorption::AbsorptionState,
    navigation::LiveVoyage,
    pirate_b4_stage::{self, Resources},
    pirate_group_effects,
    pirate_group_native_host::NativeHost,
    pirate_group_runtime,
    route_stop_runtime::RouteStopRuntime,
};
use p3_core::{
    ai::reputation::Merchant,
    pump::Operation,
    ships::{factory::HanseFactory, remove::RemoveWorld, tick::TickWorld},
    town::Town,
};
use p3_save::world::MerchantShipGroups;

#[derive(Clone)]
pub struct Live {
    pub nav: LiveVoyage,
    pub factory: HanseFactory,
    pub bands: AbsorptionState,
    pub towns: Vec<Town>,
    pub merchants: Vec<Merchant>,
    pub pool: RouteStopRuntime,
    pub groups: Vec<MerchantShipGroups>,
    pub operations: Vec<Operation>,
    pub changed_groups: Vec<usize>,
}

fn word(b: &[u8], at: usize) -> u16 { u16::from_le_bytes([b[at], b[at + 1]]) }
fn merge(old: &[u8], new: &[u8], target: &mut [u8], label: &str) -> Result<(), String> {
    if old.len() != new.len() || new.len() != target.len() { return Err(format!("{label} truncado")); }
    for i in 0..old.len() {
        if old[i] != new[i] {
            if target[i] != old[i] && target[i] != new[i] { return Err(format!("{label} desincronizado +{i:X}")); }
            target[i] = new[i];
        }
    }
    Ok(())
}

/// Solo consume la operación inicial tras preparar todas las vistas y efectos.
/// El caller publica el resultado completo junto a las proyecciones de sesión.
pub fn prepare(before: &Live, expected: Operation, resources: &Resources<'_>, tick: u32, player: u32) -> Result<Live, String> {
    if expected.0[0] != 0xb4 || before.operations.first() != Some(&expected) { return Err("cola B4 cambió".into()); }
    let snap = &before.nav.snapshot;
    if snap.manager.len() != 0x110 || snap.ships.len() % 0x180 != 0 || snap.captains.len() % 16 != 0 || snap.convoys.len() % 0x3c != 0 { return Err("flota B4 truncada".into()); }
    let ns = snap.ships.len() / 0x180;
    let nc = snap.captains.len() / 16;
    let nv = snap.convoys.len() / 0x3c;
    if before.factory.ships.len() != ns || before.factory.captains.len() != nc || before.factory.convoys.len() != nv || snap.routes.len() != ns || before.nav.baseline.ships.len() != snap.ships.len() || before.nav.baseline.routes.len() != ns || before.groups.len() != before.merchants.len() || before.merchants.len() > u16::MAX as usize || before.towns.len() > 40 { return Err("vistas B4 incompletas".into()); }
    for (at, value) in [(0xe0,before.factory.free_captain),(0xe2,before.factory.free_ship),(0xe4,before.factory.free_convoy),(0xec,before.factory.convoy_head),(0xee,before.factory.name_cursor),(0xf0,before.factory.name_step)] {
        if word(&snap.manager, at) != value { return Err("cabezas de factoría B4 desincronizadas".into()); }
    }
    let parsed = p3_save::tail::RouteStopPool::parse(before.pool.bytes()).map_err(|e| format!("depósito de rutas B4: {e:?}"))?;
    let mut manager = snap.manager.clone();
    let mut ships = snap.ships.clone();
    let mut caps = snap.captains.clone();
    let mut convoys = snap.convoys.clone();
    let mut towns = before.towns.iter().flat_map(|t| t.0.iter().copied()).collect::<Vec<_>>();
    let mut merchants = before.merchants.iter().flat_map(|m| m.bytes.iter().copied()).collect::<Vec<_>>();
    if merchants.len() != before.merchants.len() * 0x650 { return Err("comerciantes B4 truncados".into()); }
    let mut remove = RemoveWorld { manager: &mut manager, ships: &mut ships, captains: &mut caps, convoys: &mut convoys, merchants: &mut merchants, towns: &mut towns, pool_limit: parsed.count, merchant_count: before.merchants.len() as u16, town_count: before.towns.len() as u32, ui_a: false, ui_b: false };
    let w = TickWorld { remove: &mut remove, time: tick, player };
    let result = pirate_b4_stage::prepare(&w, &before.bands, expected, resources)?;
    let slot = expected.0[1] as usize;
    let host = NativeHost { captain_population_available: true, captain: resources.captain, create: None, player, pool_limit: parsed.count, cleared_routes: result.removed_routes.clone(), remove_events: result.remove_events.clone(), removed_plans: if result.removed_plan { vec![slot] } else { vec![] }, merchant_owners: result.merchant_owners.clone(), operations: vec![] };
    let group_result = pirate_group_runtime::Prepared { manager: result.manager, ships: result.ships, captains: result.captains, convoys: result.convoys, merchants: result.merchants, towns: result.towns, state: result.bands, host };
    let effects = pirate_group_effects::prepare(&group_result, before.pool.bytes(), &snap.routes, &before.groups)?;
    let mut out = before.clone();
    let new_ns = group_result.ships.len() / 0x180;
    let new_nc = group_result.captains.len() / 16;
    if new_ns < ns || new_nc < nc || group_result.convoys.len() != snap.convoys.len() || effects.routes.len() != new_ns { return Err("resultado B4 incompatible".into()); }
    for i in 0..ns {
        let old = &snap.ships[i * 0x180..(i + 1) * 0x180];
        let new = &group_result.ships[i * 0x180..(i + 1) * 0x180];
        let target = &mut out.factory.ships[i];
        if word(old, 0x134) >= 0x17 && word(new, 0x134) < 0x17 {
            // Una ranura libre S02 solo conserva estado, enlace y tipo.
            if target.len() != 0x180 || target[6..8] != old[6..8] || target[0x134..0x136] != old[0x134..0x136] || target[0x137] != old[0x137] {
                return Err("ranura naval libre B4 desincronizada".into());
            }
            target.copy_from_slice(new);
        } else {
            merge(old, new, target, "barco de factoría B4")?;
        }
    }
    for i in 0..nc { merge(&snap.captains[i*16..(i+1)*16], &group_result.captains[i*16..(i+1)*16], &mut out.factory.captains[i], "capitán de factoría B4")?; }
    for i in 0..nv { merge(&snap.convoys[i*0x3c..(i+1)*0x3c], &group_result.convoys[i*0x3c..(i+1)*0x3c], &mut out.factory.convoys[i], "convoy de factoría B4")?; }
    for image in group_result.ships.chunks_exact(0x180).skip(ns) { out.factory.ships.push(image.to_vec()); out.nav.baseline.ships.extend_from_slice(image); }
    for image in group_result.captains.chunks_exact(16).skip(nc) { out.factory.captains.push(image.try_into().unwrap()); }
    for route in effects.routes.iter().skip(ns) { out.nav.baseline.routes.push(route.clone()); }
    if new_ns != ns { out.nav.baseline.manager[0xf4..0xf6].copy_from_slice(&(new_ns as u16).to_le_bytes()); }
    let m = &group_result.manager;
    out.factory.free_captain = word(m, 0xe0); out.factory.free_ship = word(m, 0xe2); out.factory.free_convoy = word(m, 0xe4); out.factory.convoy_head = word(m, 0xec); out.factory.name_cursor = word(m, 0xee); out.factory.name_step = word(m, 0xf0);
    out.nav.snapshot.manager = group_result.manager;
    out.nav.snapshot.ships = group_result.ships;
    out.nav.snapshot.captains = group_result.captains;
    out.nav.snapshot.convoys = group_result.convoys;
    out.nav.snapshot.routes = effects.routes;
    for id in group_result.host.cleared_routes { out.nav.forget_captured_route(id); }
    crate::navigation::refresh_lists(&mut out.nav.snapshot)?;
    for (town, data) in out.towns.iter_mut().zip(group_result.towns.chunks_exact(0x9f8)) { town.0.copy_from_slice(data); }
    for (merchant, data) in out.merchants.iter_mut().zip(effects.merchants.chunks_exact(0x650)) { merchant.bytes.copy_from_slice(data); }
    out.pool.replace(before.pool.bytes(), effects.route_stop_pool)?;
    out.groups = effects.merchant_groups;
    for id in effects.changed_groups { if !out.changed_groups.contains(&id) { out.changed_groups.push(id); } }
    out.bands = group_result.state;
    out.operations.remove(0);
    Ok(out)
}
