//! Atomic live reconciliation and S05 staging for an empty hideout destruction.
use crate::{
    convoy_absorption::AbsorptionState,
    navigation,
    pirate_empty_scene_destroy::{self, ConvoyReturn, FinishedFrame},
};
use p3_core::{
    ai::reputation::Merchant,
    economy::world::Ship,
    pump::Operation,
    rng::CrtRand,
    scheduler::Scheduler,
    ships::{
        factory::HanseFactory, remove::RemoveWorld,
        spatial::CollisionIndex as NativeCollisionIndex, tick::TickWorld,
        town_attack_frame::FinishedAttackEffect,
    },
    town::Town,
};
use p3_save::{model::SaveModel, pirate_bands, ships::NavigationSnapshot};
use std::collections::BTreeMap;
#[derive(Clone)]
pub struct Scene {
    pub manager: Vec<u8>,
    pub object: Vec<u8>,
    pub address: u32,
    pub saved_block: Vec<u8>,
}
#[derive(Clone)]
pub struct CollisionIndex {
    pub address: u32,
    pub pool: NativeCollisionIndex,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Effect {
    OpenDialog(u32),
    SyncTime,
    RefreshShip(u16),
    BandNotify(u16),
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Action {
    Effect(Effect),
    Queue(Operation),
    ReleaseCollision(u8),
}
#[derive(Clone)]
pub struct LiveState {
    pub nav: NavigationSnapshot,
    pub factory: HanseFactory,
    pub merchants: Vec<Merchant>,
    pub towns: Vec<Town>,
    pub bands: AbsorptionState,
    pub scheduler: Scheduler,
    pub operations: Vec<Operation>,
    pub rng: CrtRand,
    pub scene: Option<Scene>,
    pub scene_manager: Vec<u8>,
    pub collision_indices: BTreeMap<u8, CollisionIndex>,
    pub pending_effects: Vec<Effect>,
    pub projected_ships: Vec<Ship>,
    pub changed_ships: Vec<u16>,
    pub pirate_targets: [Option<u8>; 5],
    pub pirate_heads: [Option<u16>; 5],
}
pub struct FrameInput {
    pub band: usize,
    pub delta: u32,
    pub ui_active: bool,
    pub network_client: bool,
    pub convoy_ui: bool,
    pub gui_present: bool,
    pub pool_limit: u16,
    pub player: u32,
    pub time: u32,
}
pub struct Staged {
    pub live: LiveState,
    pub actions: Vec<Action>,
}

// Solo el rectángulo de terreno, sin otros objetos que puedan colisionar.
fn single_static_terrain(pool: &NativeCollisionIndex, scene: &[u8]) -> bool {
    let i = usize::from(r16(scene, 0x1ed4));
    if pool.capacity == 0 || i >= pool.capacity || pool.span != 1 || i != 0
        || pool.free_next.len() != pool.capacity || pool.types.get(i) != Some(&7)
        || pool.references.get(i) != Some(&0)
        || pool.x.coordinates.get(i*2..i*2+2) != Some(&[0, 0x1100][..])
        || pool.y.coordinates.get(i*2..i*2+2) != Some(&[0, 0x0f02][..]) { return false; }
    for axis in [&pool.x, &pool.y] {
        if axis.head != 0 || axis.next.get(..2) != Some(&[1, u16::MAX][..])
            || axis.previous.get(..2) != Some(&[u16::MAX, 0][..]) { return false; }
    }
    let mut free = pool.free_head;
    let mut seen = vec![false; pool.capacity];
    while usize::from(free) < pool.capacity {
        let at = usize::from(free);
        if at == i || seen[at] { return false; }
        seen[at] = true;
        free = pool.free_next[at];
    }
    free == u16::MAX && seen.iter().filter(|&&v| v).count() + 1 == pool.capacity
}
fn r16(v: &[u8], i: usize) -> u16 {
    u16::from_le_bytes([v[i], v[i + 1]])
}
fn merge(before: &[u8], after: &[u8], other: &mut [u8], what: &str) -> Result<(), String> {
    if before.len() != after.len() || after.len() != other.len() {
        return Err(format!("{what} truncado"));
    }
    for i in 0..before.len() {
        if before[i] != after[i] {
            if other[i] != before[i] && other[i] != after[i] {
                return Err(format!("{what} desincronizado +{i:X}"));
            }
            other[i] = after[i]
        }
    }
    Ok(())
}
/// Consume el índice de colisión de la ranura solo en la copia preparada.
/// La consulta UI ya viene resuelta en `ui_active`; los demás efectos quedan
/// pendientes y no se ejecutan ni descartan silenciosamente.
pub fn prepare_frame(before: &LiveState, v: FrameInput) -> Result<Staged, String> {
    let scene = before.scene.as_ref().ok_or("escena ausente")?;
    if before.scene_manager != scene.manager {
        return Err("gestores de escena desincronizados".into());
    }
    if scene.object.len() != 0x2f24 {
        return Err("escena truncada".into());
    }
    let slot = r16(&scene.object, 0x662);
    if slot >= 255 {
        return Err("ranura inválida".into());
    }
    let collision = before
        .collision_indices
        .get(&(slot as u8))
        .ok_or("índice de colisión ausente")?;
    if collision.address != scene.address {
        return Err("índice de otra escena".into());
    }
    if before.bands.bands.len() != 5
        || before.nav.manager.len() != 0x110
        || before.nav.ships.len() % 0x180 != 0
        || before.nav.captains.len() % 16 != 0
        || before.nav.convoys.len() % 0x3c != 0
        || before.merchants.len() > 255
    {
        return Err("vistas vivas incompletas".into());
    }
    let ns = before.nav.ships.len() / 0x180;
    let nc = before.nav.convoys.len() / 0x3c;
    if ns > usize::from(u16::MAX)
        || nc > usize::from(u16::MAX)
        || usize::from(r16(&before.nav.manager, 0xf4)) != ns
        || usize::from(r16(&before.nav.manager, 0xf6)) != nc
        || usize::from(r16(&before.nav.manager, 0xf2)) != before.nav.captains.len() / 16
        || before.factory.ships.iter().any(|s| s.len() != 0x180)
        || before.factory.convoys.iter().any(|c| c.len() != 0x3c)
        || before.factory.ships.len() != ns
        || before.factory.convoys.len() != nc
        || before.factory.captains.len() != before.nav.captains.len() / 16
        || before.projected_ships.len() != ns
        || before.nav.routes.len() != ns
    {
        return Err("factoría o proyecciones incompletas".into());
    }
    for i in 0..ns {
        let image = &before.nav.ships[i * 0x180..(i + 1) * 0x180];
        let state = r16(image, 0x134);
        let owner = if state < 0x17 { image[0] } else { 255 };
        if before.projected_ships[i].state != state || before.projected_ships[i].owner != owner {
            return Err(format!("proyección de barco {i} desincronizada"));
        }
    }
    for (at, value) in [
        (0xe0, before.factory.free_captain),
        (0xe2, before.factory.free_ship),
        (0xe4, before.factory.free_convoy),
        (0xec, before.factory.convoy_head),
        (0xee, before.factory.name_cursor),
        (0xf0, before.factory.name_step),
    ] {
        if r16(&before.nav.manager, at) != value {
            return Err("cabezas de factoría desincronizadas".into());
        }
    }
    for i in 0..5 {
        let band = before.bands.bands[i].as_ref();
        if before.pirate_targets[i] != band.map(|b| b.header[6])
            || before.pirate_heads[i] != band.map(|b| r16(&b.header, 4))
        {
            return Err("vistas de banda desincronizadas".into());
        }
    }
    let mut manager = before.nav.manager.clone();
    let mut ships = before.nav.ships.clone();
    let mut captains = before.nav.captains.clone();
    let mut convoys = before.nav.convoys.clone();
    let mut merchants = before
        .merchants
        .iter()
        .flat_map(|m| m.bytes.iter().copied())
        .collect::<Vec<_>>();
    let mut towns = before
        .towns
        .iter()
        .flat_map(|t| t.0.iter().copied())
        .collect::<Vec<_>>();
    if before.merchants.iter().any(|m| m.bytes.len() != 0x650) {
        return Err("comerciantes truncados".into());
    }
    let mut remove = RemoveWorld {
        manager: &mut manager,
        ships: &mut ships,
        captains: &mut captains,
        convoys: &mut convoys,
        merchants: &mut merchants,
        towns: &mut towns,
        pool_limit: v.pool_limit,
        merchant_count: before.merchants.len() as u16,
        town_count: before.towns.len() as u32,
        ui_a: v.convoy_ui,
        ui_b: false,
    };
    let w = TickWorld {
        remove: &mut remove,
        time: v.time,
        player: v.player,
    };
    let static_terrain = single_static_terrain(&collision.pool, &scene.object);
    let frame = pirate_empty_scene_destroy::prepare_finished_frame(
        &w,
        &before.bands,
        v.band,
        &before.scheduler,
        &scene.manager,
        &scene.object,
        scene.address,
        v.delta,
        v.ui_active,
        v.network_client,
        v.convoy_ui,
        v.gui_present,
        v.pool_limit,
        before.rng,
        static_terrain,
    )?;
    let mut out = before.clone();
    let mut actions = Vec::new();
    let mut frame_effects = |effects: Vec<FinishedAttackEffect>| -> Result<(), String> {
        for e in effects {
            match e {
                FinishedAttackEffect::QueryDialog => {}
                FinishedAttackEffect::OpenDialog { address } => {
                    let e = Effect::OpenDialog(address);
                    actions.push(Action::Effect(e.clone()));
                    out.pending_effects.push(e)
                }
                FinishedAttackEffect::Destroy { .. } => return Err("destructor duplicado".into()),
            }
        }
        Ok(())
    };
    match frame {
        FinishedFrame::Active {
            scene,
            scene_manager,
            effects,
        } => {
            frame_effects(effects)?;
            let s = out.scene.as_mut().unwrap();
            s.object = scene;
            s.manager = scene_manager.clone();
            out.scene_manager = scene_manager;
            let pool = &mut out.collision_indices.get_mut(&(slot as u8)).ok_or("índice espacial ausente")?.pool;
            if static_terrain && v.delta as u16 != 0 {
                let pairs = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| pool.update(|_, _, _| {})))
                    .map_err(|_| "índice espacial de terreno corrupto")?;
                if !pairs.is_empty() { return Err("colisiones adicionales en escena vacía".into()); }
                s.object[0x34..0x36].copy_from_slice(&pool.x.head.to_le_bytes());
                s.object[0x44..0x46].copy_from_slice(&pool.y.head.to_le_bytes());
            } else if !static_terrain && (pool.x.head != u16::MAX || pool.y.head != u16::MAX) {
                return Err("continuación de asalto con índice espacial ocupado no conectada".into());
            }
            return Ok(Staged { live: out, actions });
        }
        FinishedFrame::Destroyed { result, effects } => {
            frame_effects(effects)?;
            if result.sync_time {
                let e = Effect::SyncTime;
                actions.push(Action::Effect(e.clone()));
                out.pending_effects.push(e)
            }
            if let Some(ret) = &result.return_effects {
                match ret {
                    ConvoyReturn::Preserved(x) => {
                        if x.recount_incoming {
                            return Err("recuento urbano pendiente".into());
                        }
                        if let Some(id) = x.refresh_ship {
                            let e = Effect::RefreshShip(id);
                            actions.push(Action::Effect(e.clone()));
                            out.pending_effects.push(e)
                        }
                    }
                    ConvoyReturn::Dissolved(x) => {
                        if x.recount_incoming
                            || !x.removed_routes.is_empty()
                            || !x.npc_losses.is_empty()
                            || !x.sunk_ships.is_empty()
                            || !x.pirate_notices.is_empty()
                        {
                            return Err("efectos de baja o ruta pendientes".into());
                        }
                        for &id in &x.refresh_ships {
                            let e = Effect::RefreshShip(id);
                            actions.push(Action::Effect(e.clone()));
                            out.pending_effects.push(e)
                        }
                    }
                }
            }
            out.scene_manager = result.scene_manager.clone();
            let world = result.world;
            for &id in &world.notifications {
                let e = Effect::BandNotify(id);
                actions.push(Action::Effect(e.clone()));
                out.pending_effects.push(e)
            }
            for op in &world.operations {
                actions.push(Action::Queue(*op));
                out.operations.push(*op)
            }
            let before_nav = &before.nav;
            out.nav.manager = world.manager;
            out.nav.ships = world.ships;
            out.nav.captains = world.captains;
            out.nav.convoys = world.convoys;
            navigation::refresh_lists(&mut out.nav)?;
            for i in 0..ns {
                let at = i * 0x180;
                let old = &before_nav.ships[at..at + 0x180];
                let new = &out.nav.ships[at..at + 0x180];
                merge(old, new, &mut out.factory.ships[i], "barco de factoría")?;
                if old != new {
                    let state = r16(new, 0x134);
                    out.projected_ships[i] = Ship {
                        owner: if state < 0x17 { new[0] } else { 255 },
                        state,
                    };
                    if !out.changed_ships.contains(&(i as u16)) {
                        out.changed_ships.push(i as u16)
                    }
                }
            }
            for i in 0..nc {
                let at = i * 0x3c;
                merge(
                    &before_nav.convoys[at..at + 0x3c],
                    &out.nav.convoys[at..at + 0x3c],
                    &mut out.factory.convoys[i],
                    "convoy de factoría",
                )?
            }
            for i in 0..out.factory.captains.len() {
                let at = i * 16;
                merge(
                    &before_nav.captains[at..at + 16],
                    &out.nav.captains[at..at + 16],
                    &mut out.factory.captains[i],
                    "capitán de factoría",
                )?
            }
            for (at, field) in [
                (0xe0, &mut out.factory.free_captain),
                (0xe2, &mut out.factory.free_ship),
                (0xe4, &mut out.factory.free_convoy),
                (0xec, &mut out.factory.convoy_head),
                (0xee, &mut out.factory.name_cursor),
                (0xf0, &mut out.factory.name_step),
            ] {
                let old = r16(&before_nav.manager, at);
                let new = r16(&out.nav.manager, at);
                if old != new {
                    if *field != old && *field != new {
                        return Err("cabeza de factoría desincronizada".into());
                    }
                    *field = new
                }
            }
            for (i, m) in out.merchants.iter_mut().enumerate() {
                m.bytes
                    .copy_from_slice(&world.merchants[i * 0x650..(i + 1) * 0x650])
            }
            for (i, t) in out.towns.iter_mut().enumerate() {
                t.0.copy_from_slice(&world.towns[i * 0x9f8..(i + 1) * 0x9f8])
            }
            out.bands = world.bands;
            out.scheduler = world.scheduler;
            out.rng = world.rng;
            for i in 0..5 {
                let band = out.bands.bands[i].as_ref();
                out.pirate_targets[i] = band.map(|b| b.header[6]);
                out.pirate_heads[i] = band.map(|b| r16(&b.header, 4));
            }
            if !result.release_collision_index {
                return Err("liberación espacial ausente".into());
            }
            let mut collision = out
                .collision_indices
                .remove(&(slot as u8))
                .ok_or("índice espacial ausente")?;
            let land = r16(&scene.object, 0x1ed4);
            let i = usize::from(land);
            if collision.pool.free_next.len() < collision.pool.capacity
                || i >= collision.pool.capacity
                || i >= usize::from(collision.pool.span)
                || collision.pool.types.get(i) != Some(&7)
                || collision.pool.references.get(i) != Some(&0)
                || collision.pool.x.coordinates.get(i * 2..i * 2 + 2) != Some(&[0, 0x1100][..])
                || collision.pool.y.coordinates.get(i * 2..i * 2 + 2) != Some(&[0, 0x0f02][..])
            {
                return Err("colisión de terreno desincronizada".into());
            }
            let mut cursor = collision.pool.free_head;
            let mut visited = vec![false; collision.pool.capacity];
            while usize::from(cursor) < collision.pool.capacity {
                let j = usize::from(cursor);
                if visited[j] {
                    return Err("lista espacial cíclica".into());
                }
                if j == i {
                    return Err("colisión de terreno ya libre".into());
                }
                visited[j] = true;
                cursor = collision.pool.free_next[j];
            }
            if visited.iter().filter(|&&v| v).count() + 1 != collision.pool.capacity {
                return Err("objetos espaciales adicionales no representados".into());
            }
            if std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                collision.pool.remove(land)
            }))
            .is_err()
            {
                return Err("índice espacial corrupto".into());
            }
            if collision.pool.free_head != land {
                return Err("baja espacial incompleta".into());
            }
            actions.push(Action::ReleaseCollision(slot as u8));
            out.scene = None;
        }
    }
    Ok(Staged { live: out, actions })
}

fn scheduler_table(s: &Scheduler) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(&s.head.to_le_bytes());
    out.extend_from_slice(&s.free_head.to_le_bytes());
    out.extend_from_slice(&s.capacity.to_le_bytes());
    for t in &s.records {
        out.extend_from_slice(&t.due.to_le_bytes());
        out.extend_from_slice(&t.next.to_le_bytes());
        out.extend_from_slice(&t.kind.to_le_bytes());
        out.extend_from_slice(&t.payload)
    }
    out
}
fn validate_saved_navigation(model: &SaveModel, live: &LiveState) -> Result<(), String> {
    use p3_save::layout::{CONVOY, SHIP_CORE, SHIP_REST, SHIP_STATE};
    if live.nav.manager.len() < 0xf2 {
        return Err("gestor naval guardado truncado".into());
    }
    let raw = model.ships.write();
    let saved = p3_save::ships::Ships::parse(&raw)
        .map_err(|e| format!("flota guardada: {e:?}"))?
        .navigation_snapshot()
        .map_err(str::to_owned)?;
    if live.nav.ships.len() != saved.ships.len()
        || live.nav.convoys.len() != saved.convoys.len()
        || live.nav.captains.len() != saved.captains.len()
        || live.nav.routes != saved.routes
    {
        return Err("capacidad o rutas navales guardadas desincronizadas".into());
    }
    for at in [0xe0usize, 0xe2, 0xe4, 0xe6, 0xe8, 0xea, 0xec, 0xee, 0xf0] {
        if r16(&live.nav.manager, at) != r16(&saved.manager, at) {
            return Err(format!("cabecera naval guardada desincronizada +{at:X}"));
        }
    }
    for (i, (a, b)) in live
        .nav
        .ships
        .chunks_exact(0x180)
        .zip(saved.ships.chunks_exact(0x180))
        .enumerate()
    {
        let state = r16(a, 0x134);
        if state >= 0x17 {
            for j in [6usize, 7, 0x134, 0x135, 0x137] {
                if a[j] != b[j] {
                    return Err(format!("barco libre {i} desincronizado"));
                }
            }
        } else {
            for seg in SHIP_CORE
                .iter()
                .chain(SHIP_REST.iter())
                .chain(std::iter::once(&SHIP_STATE))
            {
                let off = usize::from(seg.obj);
                for j in off..off + usize::from(seg.len) {
                    if !(0x24..0x28).contains(&j) && a[j] != b[j] {
                        return Err(format!("barco {i} guardado desincronizado +{j:X}"));
                    }
                }
            }
        }
    }
    for (i, (a, b)) in live
        .nav
        .convoys
        .chunks_exact(0x3c)
        .zip(saved.convoys.chunks_exact(0x3c))
        .enumerate()
    {
        for seg in &CONVOY {
            let off = usize::from(seg.obj);
            let end = off + usize::from(seg.len);
            if a[off..end] != b[off..end] {
                return Err(format!("convoy {i} guardado desincronizado +{off:X}"));
            }
        }
    }
    if live.nav.captains != saved.captains {
        return Err("capitanes guardados desincronizados".into());
    }
    Ok(())
}
/// Construye S05 sobre una copia, nunca consume B4 ni efectos sin atender.
/// `before` representa las imágenes contra las que se cargó `model`.
pub fn prepare_save(
    model: &SaveModel,
    before: &LiveState,
    after: &LiveState,
) -> Result<SaveModel, String> {
    prepare_save_inner(model,before,after,false)
}

/// Concilia flota y tarea conservando una escena aún activa. El bloque de
/// batalla debe escribirse y validarse por el consumidor antes de guardar.
pub fn prepare_active_save(model:&SaveModel,before:&LiveState,after:&LiveState)->Result<SaveModel,String>{
    prepare_save_inner(model,before,after,true)
}

fn prepare_save_inner(model:&SaveModel,before:&LiveState,after:&LiveState,active:bool)->Result<SaveModel,String> {
    if !active && after.scene.is_some() {
        return Err("escena todavía activa".into());
    }
    if !after.operations.is_empty() {
        return Err("operaciones pendientes de despacho".into());
    }
    if !after.pending_effects.is_empty() {
        return Err("efectos de presentación pendientes".into());
    }
    validate_saved_navigation(model, before)?;
    let old_scene = before.scene.as_ref().ok_or("base de escena ausente")?;
    let slot = r16(&old_scene.object, 0x662);
    if slot>=255 || after.scene_manager.len()<0x405 {return Err("gestor de escena inválido".into())}
    let pointer=&after.scene_manager[usize::from(slot)*4..usize::from(slot)*4+4];
    if active {
        let now=after.scene.as_ref().ok_or("escena activa ausente")?;
        let collision=after.collision_indices.get(&(slot as u8)).ok_or("índice activo ausente")?;
        if now.address!=old_scene.address || collision.address!=now.address
            || pointer!=now.address.to_le_bytes() || now.saved_block!=old_scene.saved_block {
            return Err("escena activa de otro gestor".into());
        }
    } else if after.collision_indices.contains_key(&(slot as u8)) || pointer!=[0;4] {
        return Err("escena o índice aún presentes".into());
    }
    if model.sea_battles != old_scene.saved_block {
        return Err("gestor de escenas modificado por otro sistema".into());
    }
    // Una base capturada tras cargar puede contener el mismo asalto urbano;
    // solo se admite si su formato singular reconstruye y vuelve a escribir.
    let loaded_attack=active && crate::pirate_active_frame::parse_single_active_attack(
        &model.sea_battles,old_scene.address,slot as u8).is_ok();
    if !loaded_attack {
        let parsed = p3_save::tail::CurrentSeaBattles::parse(&model.sea_battles)
            .map_err(|e| format!("gestor de batallas: {e:?}"))?;
        if !parsed.objects.is_empty() {return Err("base S05 contiene batallas activas".into());}
    }
    let n = usize::from(before.scheduler.capacity);
    let len = 6usize
        .checked_add(n.checked_mul(24).ok_or("tareas excesivas")?)
        .ok_or("tareas excesivas")?;
    if before.scheduler.records.len() != n
        || model.scheduled_tasks.get(..len) != Some(scheduler_table(&before.scheduler).as_slice())
    {
        return Err("planificador guardado desincronizado".into());
    }
    if pirate_bands::read(&model.ships.tail).map_err(str::to_owned)? != before.bands.bands {
        return Err("bandas guardadas desincronizadas".into());
    }
    if before.merchants.len() != after.merchants.len()
        || model.world.merchants.len() != before.merchants.len()
    {
        return Err("comerciantes guardados incompletos".into());
    }
    if before.towns.len() != after.towns.len()
        || before
            .towns
            .iter()
            .zip(&after.towns)
            .any(|(a, b)| a.0 != b.0)
    {
        return Err("ciudades modificadas sin serializador".into());
    }
    let mut staged = model.clone();
    staged.ships.apply_hanse_factory_navigation(
        &before.factory,
        &after.factory,
        &before.nav.routes,
        &after.nav.routes,
        before.towns.len(),
    )?;
    staged
        .ships
        .apply_navigation_heads(&before.nav.manager, &after.nav.manager)?;
    for i in 0..5 {
        pirate_bands::replace_optional(
            &mut staged.ships.tail,
            i,
            before.bands.bands[i].as_ref(),
            after.bands.bands[i].as_ref(),
        )?
    }
    staged.scheduled_tasks =
        p3_save::tail::update_scheduled_task_table(&staged.scheduled_tasks, &after.scheduler)?;
    for i in 0..before.merchants.len() {
        let old = &before.merchants[i].bytes;
        let new = &after.merchants[i].bytes;
        let mut persisted =
            p3_state::office::PersistedMerchant::from_save_v85(&staged.world.merchants[i])
                .ok_or("comerciante guardado truncado")?;
        for seg in &p3_save::layout::MERCHANT {
            let start = usize::from(seg.obj);
            let end = start + usize::from(seg.len);
            if old.get(start..end) != persisted.merchant.bytes.get(start..end) {
                return Err(format!(
                    "comerciante {i} guardado desincronizado +{start:X}"
                ));
            }
        }
        merge(
            old,
            new,
            &mut persisted.merchant.bytes,
            "comerciante guardado",
        )?;
        let record = persisted.to_save_v85();
        let check = p3_state::office::PersistedMerchant::from_save_v85(&record)
            .ok_or("comerciante no recargable")?;
        for j in 0..old.len() {
            if old[j] != new[j] && check.merchant.bytes[j] != new[j] {
                return Err(format!("comerciante {i}+{j:X} no serializado"));
            }
        }
        staged.world.merchants[i] = record;
    }
    Ok(staged)
}
