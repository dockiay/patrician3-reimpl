//! Integración transaccional del frame y destructor de una guarida vacía.
use crate::{
    convoy_absorption::AbsorptionState,
    pirate_empty_scene_publish::{self, CollisionIndex, FrameInput, LiveState, Scene},
    Sim,
};
use std::collections::BTreeMap;

fn require_live_defense_piece(scene:&[u8],piece:usize)->Result<(),String>{
    if piece>=4 {return Err("pieza de defensa ausente".into())}
    let at=0x1ec4+4*piece;
    let health=scene.get(at..at+4).ok_or("pieza de defensa truncada")?;
    if health==[0;4] {return Err("pieza de defensa sin vida".into())}
    Ok(())
}

impl Sim {
    /// Frame de dos barcos de IA antes de contacto. Confirma escena, flota,
    /// índice y RNG juntos; rechaza efectos aún no conectados a la sesión.
    fn advance_live_populated_town_attack_frame(&mut self,delta:u32,ui_active:bool,
        convoy_ui:bool,gui_present:bool)->Result<(),String> {
        if ui_active || convoy_ui || gui_present {return Err("interfaz de asalto poblado no conectada".into())}
        let active=self.host.town_attack_active.as_ref().ok_or("asalto activo ausente")?;
        let band_id=usize::from(self.host.town_attack_band.ok_or("banda de asalto ausente")?);
        let band=self.host.pirate_band_images.as_ref().and_then(|v|v.get(band_id)).and_then(Option::as_ref).ok_or("banda de asalto ausente")?;
        let mut battle=active.populated.clone().ok_or("escena poblada ausente")?;
        if active.object.len()!=0x2f24 || active.object!=battle.state.object {return Err("vistas de escena poblada desincronizadas".into())}
        let total=u32::from(battle.state.object[0x130f])+u32::from(delta as u16);
        // El original no actualiza las torres hasta que se completa un tick.
        let _=crate::pirate_empty_hideout_scene::has_operational_towers(band)?;
        if total>=16 && battle.state.object[0x1ec4..0x1ed4].iter().any(|&v|v!=0){return Err("frame con torres operativas pendiente".into())}
        if total>=16 && active.populated_ticks>=2 {return Err("tercer tick de asalto poblado no contrastado".into())}
        let collision=self.host.town_attack_collision.as_ref().ok_or("índice de asalto ausente")?;
        if crate::battle_spatial::into_saved(collision).map_err(str::to_owned)?!=battle.collisions {
            return Err("índice de escena poblada desincronizado".into());
        }
        let nav=self.host.navigation.as_ref().ok_or("flota de asalto ausente")?;
        let before=nav.snapshot.clone();let mut updated=nav.clone();
        if before.manager.len()<0xf8 || before.ships.len()%0x180!=0 || before.convoys.len()%0x3c!=0
            || before.captains.len()%16!=0 || before.routes.len()!=before.ships.len()/0x180 {
            return Err("flota de asalto truncada".into());
        }
        let mut worlds=before.ships.clone();let mut rng=self.host.investigation_rng;
        let merchant_data=self.rep_merchants.iter().flat_map(|m|m.bytes.iter().copied()).collect::<Vec<_>>();
        let merchants=self.rep_merchants.iter().map(|m|m.u16(8)).collect::<Vec<_>>();
        let hotseat=self.host.expedition_resources.as_ref().ok_or("recursos navales ausentes")?.hotseat;
        let env=crate::battle_runtime::Environment {
            player:self.economy.world.local_merchant,hotseat,merchants:&merchants,merchant_data:&merchant_data,
            routes:&before.routes,alderman_town:self.economy.world.alderman_town,
            town_count:self.economy.world.towns.len() as u32,merchant_towns:&[],report_enabled:&[],report_texts:None,
            fleets:&before.convoys,captains:&before.captains,manager:&before.manager,
            captain_free_head:u16::from_le_bytes([before.manager[0xe0],before.manager[0xe1]]),
            difficulty:self.host.battle_difficulty,
        };
        let outcome=crate::pirate_active_frame::advance_populated_ai_frame(&mut battle,&mut worlds,env,delta,&mut rng)?;
        let ran_tick=outcome.is_some();
        if let Some(out)=outcome {
            if out.destroyed || out.manager_data.is_some() || !out.global_letters.is_empty() || !out.news.is_empty()
                || out.merchant_data.is_some() || out.fleet_data.is_some() || !out.capture_events.is_empty()
                || !out.cleared_routes.is_empty() || out.captain_pool.is_some()
                || !out.loss_notices.is_empty() || !out.reports.is_empty() || !out.alderman_reports.is_empty() {
                return Err("efectos de asalto pendientes de publicar".into());
            }
        }
        let collision=crate::battle_spatial::into_live(&battle.collisions).map_err(str::to_owned)?;
        let mut next=active.clone();next.replace_object(&battle.state.object)?;next.populated=Some(battle);
        if ran_tick {next.populated_ticks+=1;}
        updated.snapshot.ships=worlds;
        self.host.town_attack_active=Some(next);self.host.town_attack_collision=Some(collision);
        self.host.navigation=Some(updated);self.host.investigation_rng=rng;
        self.host.publish_navigation_delta(&before,&mut self.economy.world,&mut self.economy.cargo_ships);
        Ok(())
    }

    /// Entrega al anfitrión efectos ya preparados; debe procesarlos en orden.
    pub fn take_town_attack_effects(&mut self) -> Vec<crate::pirate_empty_hideout_messages::Effect> {
        std::mem::take(&mut self.host.town_attack_effects)
    }
    pub fn take_town_attack_frame_effects(&mut self) -> Vec<crate::pirate_empty_scene_publish::Effect> {
        if !self.host.town_attack_manager_effects.is_empty() {
            return Vec::new();
        }
        std::mem::take(&mut self.host.town_attack_frame_effects)
    }
    pub fn take_town_attack_manager_effects(&mut self) -> Vec<crate::battle_manager::ManagerEffect> {
        std::mem::take(&mut self.host.town_attack_manager_effects)
    }
    /// Actualiza el gestor una sola vez; sus efectos preceden a los asedios.
    pub fn prepare_town_attack_manager(&mut self) -> Result<(), String> {
        if !self.host.clock_before_battle
            || !self.host.town_attack_tick_pending
            || self.host.town_attack_manager_processed
        {
            return Err("tick de asalto no preparado o ya completado".into());
        }
        if !self.host.town_attack_effects.is_empty()
            || !self.host.town_attack_frame_effects.is_empty()
            || !self.host.town_attack_manager_effects.is_empty()
        {
            return Err("efectos de asalto pendientes de entregar".into());
        }
        let scene = self.host.town_attack_active.as_ref().ok_or("asalto activo ausente")?;
        if scene.manager.len() < 0xc38 || scene.manager[0x42c] != 0 {
            return Err("vista del gestor de asalto pendiente".into());
        }
        if (0..255usize).any(|i|i!=usize::from(scene.slot)&&scene.manager[i*4..i*4+4]!=[0;4]){
            return Err("0x94 con otras escenas activas no está conectado".into());
        }
        let mut frame_manager = scene.manager.clone();
        let mut frame_groups = scene.groups.clone();
        let manager_effects = crate::battle_manager::tick(
            &mut frame_manager,
            &mut frame_groups,
            true,
            crate::battle_manager::ViewState { busy: false, current_scene_matches: false },
        ).map_err(str::to_owned)?;
        let scene = self.host.town_attack_active.as_mut().ok_or("asalto activo ausente")?;
        scene.manager = frame_manager;
        scene.groups = frame_groups;
        self.host.town_attack_manager_effects = manager_effects;
        self.host.town_attack_manager_processed = true;
        Ok(())
    }

    /// Completa los asedios y el tope del tick antes del frame 0x94.
    pub fn complete_town_attack_tick(&mut self) -> Result<(), String> {
        use p3_core::tick::TickSystems;
        if !self.host.clock_before_battle || !self.host.town_attack_tick_pending
            || !self.host.town_attack_manager_processed {
            return Err("gestor de asalto sin preparar".into());
        }
        if !self.host.town_attack_effects.is_empty()
            || !self.host.town_attack_frame_effects.is_empty()
            || !self.host.town_attack_manager_effects.is_empty() {
            return Err("efectos de asalto pendientes de entregar".into());
        }
        let target = self.host.town_attack_clock_target.ok_or("objetivo del reloj ausente")?;
        let mut clock = crate::merchant_daily::DailyClock {
            sim: self,
            world_ready: false,
            error: None,
        };
        let mut town = 0;
        while town < clock.clock().town_count {
            clock.siege_scene(town);
            if clock.error.is_some() {
                break;
            }
            town += 1;
        }
        if let Some(error) = clock.error {
            clock.sim.host.clock_partial_error = Some(error.clone());
            return Err(error);
        }
        if clock.clock().tick >= target {
            for merchant in 0..u32::from(clock.clock().merchant_count) {
                if clock.merchant_gold(merchant) > p3_core::tick::GOLD_CAP {
                    clock.set_merchant_gold(merchant, p3_core::tick::GOLD_CAP)
                }
            }
        }
        clock.sim.host.clock_before_battle = false;
        clock.sim.host.town_attack_tick_pending = false;
        clock.sim.host.town_attack_clock_target = None;
        Ok(())
    }

    /// Avanza la escena terminada con los resultados explícitos de consultas UI.
    /// El reloj mundial se mantiene detenido hasta completar esta transición.
    pub fn advance_town_attack_frame(
        &mut self,
        delta: u32,
        ui_active: bool,
        convoy_ui: bool,
        gui_present: bool,
    ) -> Result<(), String> {
        if !self.host.town_attack_manager_processed || self.host.town_attack_tick_pending {
            return Err("frame de asalto sin gestor preparado".into());
        }
        if !self.host.town_attack_effects.is_empty()
            || !self.host.town_attack_frame_effects.is_empty()
            || !self.host.town_attack_manager_effects.is_empty()
        {
            return Err("efectos de asalto pendientes de entregar".into());
        }
        let scene = self.host.town_attack_active.as_ref().ok_or("asalto activo ausente")?;
        if scene.manager.len() < 0xc38 || scene.manager[0x42c] != 0 {
            return Err("vista del gestor de asalto pendiente".into());
        }
        if scene.populated.is_some() {
            return self.advance_live_populated_town_attack_frame(delta,ui_active,convoy_ui,gui_present);
        }
        let frame_manager = scene.manager.clone();
        if (0..255usize).any(|i|i!=usize::from(scene.slot)&&frame_manager[i*4..i*4+4]!=[0;4]){
            return Err("0x94 con otras escenas activas no está conectado".into());
        }
        let frame_groups = scene.groups.clone();
        let band = usize::from(self.host.town_attack_band.ok_or("banda de asalto ausente")?);
        let collision = self
            .host
            .town_attack_collision
            .as_ref()
            .ok_or("índice de asalto ausente")?;
        if self.rep_merchants.len() != usize::from(self.economy.world.merchant_count) {
            return Err("capacidad y recuento de comerciantes de asalto distintos".into());
        }
        let nav = self.host.navigation.as_ref().ok_or("flota de asalto ausente")?;
        // El retorno inserta el líder en el orden X. Si ya es el ancla o
        // está enlazado en esa lista, la inserción crearía un ciclo; rechazar
        // la imagen antes de entrar en el puerto que recorre los enlaces.
        if scene.object.len() < 0x660 {
            return Err("escena de asalto truncada".into());
        }
        let image = &nav.snapshot;
        if image.manager.len() < 0xf8 || image.ships.len() % 0x180 != 0 || image.convoys.len() % 0x3c != 0 {
            return Err("flota de asalto truncada".into());
        }
        let count = image.ships.len() / 0x180;
        let cv = u16::from_le_bytes([scene.object[0x65e], scene.object[0x65f]]) as usize;
        if cv >= image.convoys.len() / 0x3c {
            return Err("convoy de asalto ausente".into());
        }
        let at = cv * 0x3c;
        let leader = u16::from_le_bytes([image.convoys[at + 0x10], image.convoys[at + 0x11]]) as usize;
        let mut current = u16::from_le_bytes([image.manager[0xea], image.manager[0xeb]]) as usize;
        let mut seen = vec![false; count];
        while current < count {
            if seen[current] {
                return Err("orden marítimo cíclico".into());
            }
            seen[current] = true;
            if current == leader {
                return Err("líder de asalto ya enlazado en orden marítimo".into());
            }
            let at = current * 0x180;
            current = u16::from_le_bytes([image.ships[at + 0xc], image.ships[at + 0xd]]) as usize;
        }
        let factory = self.host.expedition_factory.as_ref().ok_or("factoría naval ausente")?;
        let rng = self.host.investigation_rng.ok_or("estado CRT de asalto ausente")?;
        let live = self
            .host
            .pirate_band_images
            .as_ref()
            .ok_or("bandas de asalto ausentes")?;
        if live.len() != 5 {
            return Err("bandas de asalto incompletas".into());
        }
        let mut bands = Vec::with_capacity(5);
        for (i, b) in live.iter().enumerate() {
            bands.push(if b.is_some() {
                Some(self.host.reconcile_pirate_band_image(i)?)
            } else {
                None
            })
        }
        let targets = *self.host.pirate_targets.as_ref().ok_or("destinos piratas ausentes")?;
        let heads = std::array::from_fn(|i| {
            bands[i]
                .as_ref()
                .map(|b| u16::from_le_bytes([b.header[4], b.header[5]]))
        });
        let state = AbsorptionState {
            bands,
            day: self.economy.world.day_of_year,
            network_client: self.economy.world.network_client,
            operations: vec![],
        };
        let mut collisions = BTreeMap::new();
        collisions.insert(
            scene.slot,
            CollisionIndex {
                address: scene.address,
                pool: collision.clone(),
            },
        );
        let before = LiveState {
            nav: nav.snapshot.clone(),
            factory: factory.clone(),
            merchants: self.rep_merchants.clone(),
            towns: self.economy.world.towns.clone(),
            bands: state,
            scheduler: self.host.council_scheduler.clone(),
            operations: self.host.council_operations.clone(),
            rng,
            scene: Some(Scene {
                manager: frame_manager.clone(),
                object: scene.object.clone(),
                address: scene.address,
                saved_block: scene.original_block.clone(),
            }),
            scene_manager: frame_manager.clone(),
            collision_indices: collisions,
            pending_effects: vec![],
            projected_ships: self.economy.world.ships.clone(),
            changed_ships: vec![],
            pirate_targets: targets,
            pirate_heads: heads,
        };
        let pool = self
            .host
            .route_stop_runtime
            .as_ref()
            .ok_or("depósito de rutas ausente")?;
        let pool_limit = p3_save::tail::RouteStopPool::parse(pool.bytes())
            .map_err(|e| format!("depósito de rutas: {e:?}"))?
            .count;
        let out = pirate_empty_scene_publish::prepare_frame(
            &before,
            FrameInput {
                band,
                delta,
                ui_active,
                network_client: self.economy.world.network_client != 0,
                convoy_ui,
                gui_present,
                pool_limit,
                player: self.economy.world.local_merchant,
                time: self.idle.clock.tick,
            },
        )?;
        if out.live.merchants.len() != self.rep_merchants.len()
            || out.live.merchants.len() != self.economy.merchants.len()
            || out.live.towns.len() != self.economy.world.towns.len()
            || out.live.projected_ships.len() != self.economy.world.ships.len()
            || self.economy.world.merchants.len() < usize::from(self.economy.world.merchant_count)
            || out.live.merchants.len() < usize::from(self.economy.world.merchant_count)
        {
            return Err("resultado de asalto incompatible".into());
        }
        let mut updated = nav.clone();
        updated.snapshot = out.live.nav.clone();
        let mut next_scene = scene.clone();
        next_scene.groups = frame_groups.clone();
        let next_collision = if let Some(current) = out.live.scene.as_ref() {
            next_scene.manager = current.manager.clone();
            next_scene.object = current.object.clone();
            Some(
                out.live
                    .collision_indices
                    .get(&next_scene.slot)
                    .ok_or("colisión de asalto desaparecida")?
                    .pool
                    .clone(),
            )
        } else {
            None
        };
        let idle_manager = if out.live.scene.is_none() {
            let frame = p3_save::tail::CurrentSeaBattles::parse(&scene.original_block)
                .map_err(|e| format!("gestor anterior: {e:?}"))?;
            let initial = crate::battle_manager::groups_from_save(&frame).map_err(str::to_owned)?;
            Some(crate::battle_manager::Runtime {
                manager: out.live.scene_manager.clone(),
                groups: frame_groups,
                initial,
                original_block: scene.original_block.clone(),
                error: None,
                rewrite: true,
            })
        } else {
            None
        };
        if self.host.pirate_boundary_model.is_some() && self.host.pirate_active_live.is_none(){
            self.host.pirate_active_live=Some(before.clone());
        }
        // Las vistas que cambian ya fueron reconciliadas por prepare_frame.
        self.host.navigation = Some(updated);
        self.host.expedition_factory = Some(out.live.factory);
        self.host.council_scheduler = out.live.scheduler;
        self.host.council_operations = out.live.operations;
        self.host.investigation_rng = Some(out.live.rng);
        self.host.pirate_targets = Some(out.live.pirate_targets);
        self.economy.world.towns = out.live.towns;
        self.economy.world.ships = out.live.projected_ships;
        self.rep_merchants = out.live.merchants;
        self.economy.merchants.clone_from(&self.rep_merchants);
        for i in 0..usize::from(self.economy.world.merchant_count) {
            self.economy.world.merchants[i].gold = self.rep_merchants[i].i32(0);
            self.economy.world.merchants[i].income = self.rep_merchants[i].i32(0x4c8)
        }
        if let Some(missions) = self.host.mission_pirates.as_mut() {
            for (slot, band) in missions.iter_mut().zip(&out.live.bands.bands) {
                match (slot, band) {
                    (dst, None) => *dst = None,
                    (Some(m), Some(b)) => {
                        m.first_ship = u16::from_le_bytes([b.header[4], b.header[5]]);
                        m.target = b.header[6];
                        m.status = b.header[12];
                        m.reserve_strength = u16::from_le_bytes([b.header[14], b.header[15]])
                    }
                    (None, Some(_)) => {}
                }
            }
        }
        self.host.pirate_band_images = Some(out.live.bands.bands);
        if let Some(p) = self.host.hanse_pirates.as_mut() {
            p.heads = out.live.pirate_heads;
            p.next = out
                .live
                .nav
                .ships
                .chunks_exact(0x180)
                .map(|s| u16::from_le_bytes([s[4], s[5]]))
                .collect();
            p.ship_captains = out
                .live
                .nav
                .ships
                .chunks_exact(0x180)
                .map(|s| u16::from_le_bytes([s[0x42], s[0x43]]))
                .collect();
            p.captains = out
                .live
                .nav
                .captains
                .chunks_exact(16)
                .map(|c| c.try_into().expect("capitán completo"))
                .collect();
        }
        // Varios frames 0x94 pueden pertenecer al mismo tick ya completado.
        // El gestor deberá prepararse de nuevo cuando se abra otro tick.
        self.host.town_attack_manager_processed = out.live.scene.is_some();
        self.host.town_attack_frame_effects = out.live.pending_effects;
        if let Some(collision) = next_collision {
            self.host.town_attack_active = Some(next_scene);
            self.host.town_attack_collision = Some(collision)
        } else {
            self.host.town_attack_active = None;
            self.host.town_attack_collision = None;
            self.host.town_attack_band = None;
            self.idle.battle_manager = idle_manager;
        }
        self.host
            .publish_navigation_delta(&before.nav, &mut self.economy.world, &mut self.economy.cargo_ships);
        Ok(())
    }

    /// Despacha la operación de frame únicamente si está en cabeza. Si la
    /// escena no admite el resultado, devuelve la operación a su posición.
    pub fn dispatch_town_attack_frame_operation(&mut self,ui_active:bool,convoy_ui:bool,gui_present:bool,time_mode:u32)->Result<(),String>{
        let op=*self.host.council_operations.first().ok_or("operación de frame ausente")?;
        if op.0[0]!=p3_core::pump::OP_ADVANCE_BATTLES{return Err("la primera operación no es 0x94".into())}
        self.host.council_operations.remove(0);
        if let Err(error)=self.advance_town_attack_frame(op.0[1],ui_active,convoy_ui,gui_present){
            self.host.council_operations.insert(0,op);
            return Err(error)
        }
        // 0x536C37 marca el gestor tras procesar 0x94, tanto si la escena
        // continúa como si el destructor la retiró de su ranura.
        if let Some(scene)=self.host.town_attack_active.as_mut(){scene.manager[0x3fe]=1;}
        else if let Some(manager)=self.idle.battle_manager.as_mut(){manager.manager[0x3fe]=1;}
        let direct=self.host.town_attack_active.as_ref().map(|scene|scene.object[0xac8]!=0);
        if let Some(request)=p3_core::pump::battle_time_mode_request(time_mode,self.economy.world.network_client!=0,false,&[direct]){
            self.host.council_operations.push(request);
        }
        Ok(())
    }

    /// El anfitrión aplica el modo de tiempo devuelto; no se trata de una
    /// operación del concejo ni debe consumirse silenciosamente.
    pub fn take_town_attack_time_request(&mut self)->Result<u32,String>{
        let op=*self.host.council_operations.first().ok_or("petición de modo ausente")?;
        if op.0[0]!=p3_core::pump::OP_SET_TIME_MODE || op.0[1]!=u32::MAX
            || op.0[2]!=u32::MAX || op.0[4]!=u32::MAX || !matches!(op.0[3],0|2)
        {return Err("primera operación no es petición de modo de batalla".into())}
        self.host.council_operations.remove(0);
        Ok(op.0[3])
    }
}

impl Sim {
    /// Publica un contacto de costa explícito en un asalto pirata cargado y
    /// con geometría reconstruida. El frame y S05 no se confirman aquí.
    /// El anfitrión debe aportar tablas y proyección de la escena original.
    pub fn apply_pirate_coast_contact<F,G>(&mut self,local:usize,weights:&[u8;45],
        shapes:&[[(i16,i16);5];4],sine:&[i32;65],angles:&[u8;65],sound_possible:bool,
        pixel_to_cell:F,cell_to_pixel:G)->Result<bool,String>
    where F:FnMut(i32,i32)->Result<(u16,u16),&'static str>,G:FnMut(u16,u16)->(u32,u32){
        let active=self.host.town_attack_active.as_ref().ok_or("asalto activo ausente")?;
        let mut battle=active.populated.clone().ok_or("escena poblada ausente")?;
        if active.object!=battle.state.object || !self.host.town_attack_save_unreconciled || active.object.get(0x2f1a)!=Some(&0){return Err("asalto no reconstruido o desincronizado".into())}
        let band_id=usize::from(self.host.town_attack_band.ok_or("banda de asalto ausente")?);
        if band_id>=5 || active.object.get(0x1e98).copied()!=Some(254-band_id as u8){return Err("destino de banda desincronizado".into())}
        let plan=self.host.pirate_band_images.as_ref().and_then(|v|v.get(band_id)).and_then(Option::as_ref).and_then(|b|b.plan.as_deref()).ok_or("plano de banda ausente")?;
        let index=self.host.town_attack_collision.as_ref().ok_or("índice de asalto ausente")?;
        if crate::battle_spatial::into_saved(index).map_err(str::to_owned)?!=battle.collisions{return Err("índice de asalto desincronizado".into())}
        let ship=battle.state.local_ships.get_mut(local).ok_or("barco local ausente")?;
        if ship.len()<0x20{return Err("barco local truncado".into())}
        let world_id=u32::from_le_bytes(ship[4..8].try_into().unwrap()) as usize;
        let snapshot=&self.host.navigation.as_ref().ok_or("flota ausente")?.snapshot;
        let kind=*snapshot.ships.get(world_id.checked_mul(0x180).and_then(|v|v.checked_add(0xe)).ok_or("barco mundial inválido")?).ok_or("barco mundial ausente")?;
        let shape=shapes.get(usize::from(kind)).ok_or("tipo de casco no admitido")?;
        let changed=crate::pirate_active_frame::apply_town_coast_contact(&battle.state.object,plan,self.economy.world.towns.len() as u32,ship,weights,shape,sine,angles,sound_possible,pixel_to_cell,cell_to_pixel).map_err(str::to_owned)?;
        let mut next=active.clone();next.populated=Some(battle);self.host.town_attack_active=Some(next);Ok(changed)
    }

    /// Aplica explícitamente la rama de recarga de una pieza viva. No ejecuta
    /// selección de blanco ni habilita el frame de torres operativas.
    pub fn apply_pirate_tower_cooldown(&mut self,piece:usize)->Result<(),String>{
        let active=self.host.town_attack_active.as_ref().ok_or("asalto activo ausente")?;
        let mut battle=active.populated.clone().ok_or("escena poblada ausente")?;
        if active.object!=battle.state.object || !self.host.town_attack_save_unreconciled{return Err("vistas de asalto desincronizadas".into())}
        require_live_defense_piece(&battle.state.object,piece)?;
        let band_id=usize::from(self.host.town_attack_band.ok_or("banda de asalto ausente")?);
        if band_id>=5 || active.object.get(0x1e98).copied()!=Some(254-band_id as u8){return Err("destino de banda desincronizado".into())}
        let live=self.host.town_attack_collision.as_ref().ok_or("índice de asalto ausente")?;
        if crate::battle_spatial::into_saved(live).map_err(str::to_owned)?!=battle.collisions{return Err("índice de asalto desincronizado".into())}
        crate::pirate_active_frame::town_defense_piece_wait(&mut battle.state.object,piece,&battle.state.local_ships,&battle.state.next).map_err(str::to_owned)?;
        let mut next=active.clone();next.replace_object(&battle.state.object)?;next.populated=Some(battle);self.host.town_attack_active=Some(next);Ok(())
    }

    /// Daño directo de un barco a una pieza pirata en pausa diagnóstica.
    /// Publica escena, índice y planificador juntos; no ejecuta el barco.
    pub fn apply_pirate_ship_tower_damage(&mut self,piece:usize,damage:i32)->Result<bool,String>{
        let active=self.host.town_attack_active.as_ref().ok_or("asalto activo ausente")?;
        if active.manager.len()<0xc38 || active.manager[0x42c]!=0{return Err("vista de asalto pendiente".into())}
        if self.host.expedition_resources.as_ref().ok_or("recursos navales ausentes")?.hotseat{return Err("daño a torre en hotseat pendiente".into())}
        let mut battle=active.populated.clone().ok_or("escena poblada ausente")?;
        if active.object!=battle.state.object || !self.host.town_attack_save_unreconciled{return Err("vistas de asalto desincronizadas".into())}
        require_live_defense_piece(&battle.state.object,piece)?;
        if damage<0{return Err("daño negativo no contrastado".into())}
        let band_id=usize::from(self.host.town_attack_band.ok_or("banda de asalto ausente")?);
        if band_id>=5 || active.object.get(0x1e98).copied()!=Some(254-band_id as u8){return Err("destino de banda desincronizado".into())}
        let band=self.host.pirate_band_images.as_ref().and_then(|v|v.get(band_id)).and_then(Option::as_ref).ok_or("imagen de banda ausente")?;
        let index=self.host.town_attack_collision.as_ref().ok_or("índice de asalto ausente")?;
        if crate::battle_spatial::into_saved(index).map_err(str::to_owned)?!=battle.collisions{return Err("índice de asalto desincronizado".into())}
        let at=0x1ec4+4*piece;let health=i32::from_le_bytes(battle.state.object[at..at+4].try_into().unwrap());
        if health<=0{return Err("vida de pieza fuera del alcance contrastado".into())}
        let destroyed=health<=damage;
        let (scene,collision,scheduler)=if destroyed {
            let staged=crate::pirate_active_frame::stage_pirate_tower_destruction(&battle.state.object,piece,damage,
                self.economy.world.towns.len() as u32,band,index,&self.host.council_scheduler,self.idle.clock.tick)?;
            (staged.scene,staged.collision,staged.scheduler)
        } else {
            let mut scene=battle.state.object.clone();
            crate::pirate_active_frame::town_pirate_tower_nonlethal(&mut scene,piece,damage,self.economy.world.towns.len() as u32).map_err(str::to_owned)?;
            (scene,index.clone(),self.host.council_scheduler.clone())
        };
        let mut next=active.clone();next.replace_object(&scene)?;battle.state.object=scene;
        battle.collisions=crate::battle_spatial::into_saved(&collision).map_err(str::to_owned)?;next.populated=Some(battle);
        self.host.town_attack_active=Some(next);self.host.town_attack_collision=Some(collision);self.host.council_scheduler=scheduler;
        Ok(destroyed)
    }

    /// Ejecuta armas instantáneas de asedio solo contra torres, en pausa.
    pub fn fire_pirate_siege_ship_at_towers(&mut self,id:usize)->Result<usize,String>{
        let active=self.host.town_attack_active.as_ref().ok_or("asalto activo ausente")?;
        if active.manager.len()<0xc38 || active.manager[0x42c]!=0{return Err("vista de asalto pendiente".into())}
        if self.host.expedition_resources.as_ref().ok_or("recursos navales ausentes")?.hotseat{return Err("disparo de asedio en hotseat pendiente".into())}
        let mut battle=active.populated.clone().ok_or("escena poblada ausente")?;
        if active.object!=battle.state.object || !self.host.town_attack_save_unreconciled{return Err("vistas de asalto desincronizadas".into())}
        let band_id=usize::from(self.host.town_attack_band.ok_or("banda de asalto ausente")?);
        if band_id>=5 || active.object.get(0x1e98).copied()!=Some(254-band_id as u8){return Err("destino de banda desincronizado".into())}
        let band=self.host.pirate_band_images.as_ref().and_then(|v|v.get(band_id)).and_then(Option::as_ref).ok_or("imagen de banda ausente")?;
        let index=self.host.town_attack_collision.as_ref().ok_or("índice de asalto ausente")?;
        if crate::battle_spatial::into_saved(index).map_err(str::to_owned)?!=battle.collisions{return Err("índice de asalto desincronizado".into())}
        let nav=self.host.navigation.as_ref().ok_or("flota de asalto ausente")?;
        let merchants=self.rep_merchants.iter().map(|m|m.u16(8)).collect::<Vec<_>>();
        let staged=crate::pirate_active_frame::stage_siege_tower_shots(&battle.state.object,&battle.state.local_ships,
            &nav.snapshot.ships,&battle.state.next,id,&nav.snapshot.captains,&merchants,self.host.battle_difficulty,band,index,
            &self.host.council_scheduler,self.economy.world.towns.len() as u32,self.idle.clock.tick,false)?;
        let mut next=active.clone();next.replace_object(&staged.scene)?;battle.state.object=staged.scene;
        battle.state.local_ships[id]=staged.ship;battle.collisions=crate::battle_spatial::into_saved(&staged.collision).map_err(str::to_owned)?;
        next.populated=Some(battle);self.host.town_attack_active=Some(next);self.host.town_attack_collision=Some(staged.collision);
        self.host.council_scheduler=staged.scheduler;Ok(staged.shots)
    }

    /// Paso explícito de un barco urbano con su caché de navegación y armas.
    /// El llamante debe situarse en el punto del frame anterior al barco.
    pub fn step_pirate_siege_ship(&mut self,id:usize)->Result<usize,String>{
        let active=self.host.town_attack_active.as_ref().ok_or("asalto activo ausente")?;
        if active.manager.len()<0xc38 || active.manager[0x42c]!=0{return Err("vista de asalto pendiente".into())}
        if self.host.expedition_resources.as_ref().ok_or("recursos navales ausentes")?.hotseat{return Err("paso de asedio en hotseat pendiente".into())}
        let mut battle=active.populated.clone().ok_or("escena poblada ausente")?;
        if active.object!=battle.state.object || !self.host.town_attack_save_unreconciled{return Err("vistas de asalto desincronizadas".into())}
        let band_id=usize::from(self.host.town_attack_band.ok_or("banda de asalto ausente")?);
        if band_id>=5 || active.object.get(0x1e98).copied()!=Some(254-band_id as u8){return Err("destino de banda desincronizado".into())}
        let band=self.host.pirate_band_images.as_ref().and_then(|v|v.get(band_id)).and_then(Option::as_ref).ok_or("imagen de banda ausente")?;
        let index=self.host.town_attack_collision.as_ref().ok_or("índice de asalto ausente")?;
        if crate::battle_spatial::into_saved(index).map_err(str::to_owned)?!=battle.collisions{return Err("índice de asalto desincronizado".into())}
        let mut cache=active.town_navigation.clone().ok_or("caché de asedio ausente")?;
        let images=battle.state.local_ships.iter().enumerate().map(|(i,s)|cache.image(i,s).map_err(str::to_owned)).collect::<Result<Vec<_>,_>>()?;
        let nav=self.host.navigation.as_ref().ok_or("flota de asalto ausente")?;let before=nav.snapshot.clone();let mut updated=nav.clone();
        let merchants=self.rep_merchants.iter().map(|m|m.u16(8)).collect::<Vec<_>>();
        let staged=crate::pirate_active_frame::stage_siege_ship_step(&battle.state.object,&images,&before.ships,
            &battle.state.next,&battle.spatial_0,&battle.spatial_1,id,&before.captains,&merchants,self.host.battle_difficulty,
            band,index,&self.host.council_scheduler,self.economy.world.towns.len() as u32,self.idle.clock.tick)?;
        for(i,image)in staged.ships.iter().enumerate(){cache.replace_image(i,image).map_err(str::to_owned)?;battle.state.local_ships[i]=image[..0x164].to_vec();}
        let mut next=active.clone();next.replace_object(&staged.scene)?;battle.state.object=staged.scene;
        battle.collisions=crate::battle_spatial::into_saved(&staged.collision).map_err(str::to_owned)?;next.populated=Some(battle);next.town_navigation=Some(cache);
        updated.snapshot.ships=staged.worlds;self.host.town_attack_active=Some(next);self.host.town_attack_collision=Some(staged.collision);
        self.host.council_scheduler=staged.scheduler;self.host.navigation=Some(updated);
        self.host.publish_navigation_delta(&before,&mut self.economy.world,&mut self.economy.cargo_ships);Ok(staged.shots)
    }

    /// Ordena y despacha parejas urbanas explícitamente sobre una copia.
    pub fn dispatch_pirate_siege_pairs<F,G>(&mut self,weights:&[u8;45],shapes:&[[(i16,i16);5];4],
        sine:&[i32;65],angles:&[u8;65],sound_possible:bool,pixel_to_cell:F,cell_to_pixel:G)->Result<(usize,usize),String>
    where F:FnMut(i32,i32)->Result<(u16,u16),&'static str>,G:Fn(u16,u16)->(u32,u32){
        let active=self.host.town_attack_active.as_ref().ok_or("asalto activo ausente")?;
        if active.manager.len()<0xc38 || active.manager[0x42c]!=0{return Err("vista de asalto pendiente".into())}
        if self.host.expedition_resources.as_ref().ok_or("recursos navales ausentes")?.hotseat{return Err("parejas en hotseat pendientes".into())}
        let mut battle=active.populated.clone().ok_or("escena poblada ausente")?;
        if active.object!=battle.state.object || !self.host.town_attack_save_unreconciled{return Err("vistas de asalto desincronizadas".into())}
        let band_id=usize::from(self.host.town_attack_band.ok_or("banda de asalto ausente")?);
        if band_id>=5 || active.object.get(0x1e98).copied()!=Some(254-band_id as u8){return Err("destino de banda desincronizado".into())}
        let plan=self.host.pirate_band_images.as_ref().and_then(|v|v.get(band_id)).and_then(Option::as_ref).and_then(|b|b.plan.as_deref()).ok_or("plano de banda ausente")?;
        let index=self.host.town_attack_collision.as_ref().ok_or("índice de asalto ausente")?;
        if crate::battle_spatial::into_saved(index).map_err(str::to_owned)?!=battle.collisions{return Err("índice de asalto desincronizado".into())}
        let mut cache=active.town_navigation.clone().ok_or("caché de asedio ausente")?;
        let images=battle.state.local_ships.iter().enumerate().map(|(i,s)|cache.image(i,s).map_err(str::to_owned)).collect::<Result<Vec<_>,_>>()?;
        let nav=self.host.navigation.as_ref().ok_or("flota de asalto ausente")?;let before=nav.snapshot.clone();let mut updated=nav.clone();
        let staged=crate::pirate_active_frame::stage_siege_pairs(&battle.state.object,&images,&before.ships,&before.captains,index,plan,
            self.economy.world.towns.len() as u32,weights,shapes,sine,angles,sound_possible,pixel_to_cell,cell_to_pixel)?;
        for(i,image)in staged.ships.iter().enumerate(){cache.replace_image(i,image).map_err(str::to_owned)?;battle.state.local_ships[i]=image[..0x164].to_vec();}
        let mut next=active.clone();next.replace_object(&staged.scene)?;battle.state.object=staged.scene;
        battle.collisions=crate::battle_spatial::into_saved(&staged.collision).map_err(str::to_owned)?;next.populated=Some(battle);next.town_navigation=Some(cache);
        updated.snapshot.ships=staged.worlds;self.host.town_attack_active=Some(next);self.host.town_attack_collision=Some(staged.collision);
        self.host.navigation=Some(updated);self.host.publish_navigation_delta(&before,&mut self.economy.world,&mut self.economy.cargo_ships);
        Ok((staged.candidates,staged.coast_contacts))
    }

    /// Frame urbano acotado en pausa, con recursos de costa del anfitrión.
    pub fn advance_pirate_siege_frame<F,G>(&mut self,delta:u32,power:u8,divisor:u8,weights:&[u8;45],shapes:&[[(i16,i16);5];4],
        sine:&[i32;65],angles:&[u8;65],sound_possible:bool,pixel_to_cell:F,cell_to_pixel:G)->Result<(usize,usize,usize),String>
    where F:FnMut(i32,i32)->Result<(u16,u16),&'static str>,G:Fn(u16,u16)->(u32,u32){
        let active=self.host.town_attack_active.as_ref().ok_or("asalto activo ausente")?;
        if active.manager.len()<0xc38 || active.manager[0x42c]!=0{return Err("vista de asalto pendiente".into())}
        if self.host.expedition_resources.as_ref().ok_or("recursos navales ausentes")?.hotseat{return Err("frame de asedio en hotseat pendiente".into())}
        let mut battle=active.populated.clone().ok_or("escena poblada ausente")?;
        if active.object!=battle.state.object || !self.host.town_attack_save_unreconciled{return Err("vistas de asalto desincronizadas".into())}
        let band_id=usize::from(self.host.town_attack_band.ok_or("banda de asalto ausente")?);
        if band_id>=5 || active.object.get(0x1e98).copied()!=Some(254-band_id as u8){return Err("destino de banda desincronizado".into())}
        let band=self.host.pirate_band_images.as_ref().and_then(|v|v.get(band_id)).and_then(Option::as_ref).ok_or("banda ausente")?;
        let plan=band.plan.as_deref().ok_or("plano de banda ausente")?;
        let index=self.host.town_attack_collision.as_ref().ok_or("índice de asalto ausente")?;
        if crate::battle_spatial::into_saved(index).map_err(str::to_owned)?!=battle.collisions{return Err("índice de asalto desincronizado".into())}
        let mut cache=active.town_navigation.clone().ok_or("caché de asedio ausente")?;
        let images=battle.state.local_ships.iter().enumerate().map(|(i,s)|cache.image(i,s).map_err(str::to_owned)).collect::<Result<Vec<_>,_>>()?;
        let nav=self.host.navigation.as_ref().ok_or("flota de asalto ausente")?;let before=nav.snapshot.clone();let mut updated=nav.clone();
        let merchants=self.rep_merchants.iter().map(|m|m.u16(8)).collect::<Vec<_>>();
        let staged=crate::pirate_active_frame::stage_siege_frame(&battle.state.object,&images,&before.ships,&battle.state.next,
            &battle.spatial_0,&battle.spatial_1,&before.captains,&merchants,self.host.battle_difficulty,band,index,
            &self.host.council_scheduler,self.economy.world.towns.len()as u32,self.idle.clock.tick,delta,power,divisor,plan,
            weights,shapes,sine,angles,sound_possible,pixel_to_cell,cell_to_pixel)?;
        for(i,image)in staged.ships.iter().enumerate(){cache.replace_image(i,image).map_err(str::to_owned)?;battle.state.local_ships[i]=image[..0x164].to_vec();}
        let mut next=active.clone();next.replace_object(&staged.scene)?;battle.state.object=staged.scene;
        battle.collisions=crate::battle_spatial::into_saved(&staged.collision).map_err(str::to_owned)?;next.populated=Some(battle);next.town_navigation=Some(cache);
        next.populated_ticks=next.populated_ticks.checked_add(1).ok_or("contador de asedio excedido")?;
        updated.snapshot.ships=staged.worlds;self.host.town_attack_active=Some(next);self.host.town_attack_collision=Some(staged.collision);
        self.host.council_scheduler=staged.scheduler;self.host.navigation=Some(updated);
        self.host.publish_navigation_delta(&before,&mut self.economy.world,&mut self.economy.cargo_ships);
        Ok((staged.shots,staged.candidates,staged.coast_contacts))
    }

    /// Publica el disparo instantáneo sin defensores y sin eventos externos.
    /// La dificultad se obtiene de la sesión y los participantes de comerciantes. El frame continúa protegido; S05 exige reconciliación.
    pub fn apply_pirate_tower_instant(&mut self,piece:usize,power:u8)->Result<bool,String>{
        self.apply_pirate_tower_instant_inner(piece,power,None)
    }
    /// Como el anterior, con defensores y tabla angular original verificada.
    pub fn apply_pirate_tower_instant_with_defenders(&mut self,piece:usize,power:u8,angles:&[u8;65])->Result<bool,String>{
        self.apply_pirate_tower_instant_inner(piece,power,Some(angles))
    }
    fn apply_pirate_tower_instant_inner(&mut self,piece:usize,power:u8,angles:Option<&[u8;65]>)->Result<bool,String>{
        let active=self.host.town_attack_active.as_ref().ok_or("asalto activo ausente")?;
        let mut battle=active.populated.clone().ok_or("escena poblada ausente")?;
        if active.object!=battle.state.object || !self.host.town_attack_save_unreconciled{return Err("vistas de asalto desincronizadas".into())}
        require_live_defense_piece(&battle.state.object,piece)?;
        let band=usize::from(self.host.town_attack_band.ok_or("banda de asalto ausente")?);
        if band>=5 || active.object.get(0x1e98).copied()!=Some(254-band as u8){return Err("destino de banda desincronizado".into())}
        let collision=self.host.town_attack_collision.as_ref().ok_or("índice de asalto ausente")?;
        if crate::battle_spatial::into_saved(collision).map_err(str::to_owned)?!=battle.collisions{return Err("índice de asalto desincronizado".into())}
        let nav=self.host.navigation.as_ref().ok_or("flota de asalto ausente")?;let before=nav.snapshot.clone();let mut after=nav.clone();
        let mut human=[false;2];
        for (side,is_human) in human.iter_mut().enumerate(){
            let at=0x35a+2*side;let bytes=battle.state.object.get(at..at+2).ok_or("participantes de torre truncados")?;
            let id=u16::from_le_bytes([bytes[0],bytes[1]]);
            if id!=0xff {*is_human=self.rep_merchants.get(usize::from(id)).ok_or("comerciante de torre ausente")?.u16(8)==0;}
        }
        let prepared=crate::pirate_active_frame::stage_tower_damage_with_notices(&battle.state.object,piece,&battle.state.local_ships,&before.ships,&battle.state.next,&battle.spatial_0,&battle.spatial_1,power,human,self.host.battle_difficulty,angles,collision);
        let Some((scene,ships,worlds,index,notices))=prepared.map_err(str::to_owned)? else{return Ok(false)};
        let merchants=self.rep_merchants.iter().flat_map(|m|m.bytes.iter().copied()).collect::<Vec<_>>();let mut pool=self.host.message_pool.clone();let mut news=self.host.town_attack_global_news.clone();let date=self.idle.clock.calendar;
        for (captain,opponent) in notices {if let Some(letter)=p3_core::ships::weapon_damage::pirate_sink_notice(captain,opponent,&before.captains,&merchants,self.economy.world.alderman_town).map_err(str::to_owned)?{
            p3_state::messages::add_global_inline_letter(&mut pool,date.day,date.month,date.year,&letter)?;news.push((date,letter));
        }}
        let mut next=active.clone();next.replace_object(&scene)?;battle.state.object=scene;battle.state.local_ships=ships;battle.collisions=crate::battle_spatial::into_saved(&index).map_err(str::to_owned)?;next.populated=Some(battle);after.snapshot.ships=worlds;
        self.host.town_attack_active=Some(next);self.host.town_attack_collision=Some(index);self.host.navigation=Some(after);self.host.message_pool=pool;self.host.town_attack_global_news=news;self.host.publish_navigation_delta(&before,&mut self.economy.world,&mut self.economy.cargo_ships);Ok(true)
    }

    /// Crea explícitamente el proyectil de torre en escena diagnóstica pausada.
    /// Aún no avanza su trayectoria ni habilita el frame automático.
    pub fn spawn_pirate_tower_projectile(&mut self,piece:usize,power:u8,angles:&[u8;65])->Result<bool,String>{
        let active=self.host.town_attack_active.as_ref().ok_or("asalto activo ausente")?;
        if active.manager.len()<0xc38 || active.manager[0x42c]!=0{return Err("vista de asalto pendiente".into())}
        if self.host.expedition_resources.as_ref().ok_or("recursos navales ausentes")?.hotseat{return Err("proyectil en hotseat pendiente".into())}
        let mut battle=active.populated.clone().ok_or("escena poblada ausente")?;
        if active.object!=battle.state.object || !self.host.town_attack_save_unreconciled{return Err("vistas de asalto desincronizadas".into())}
        require_live_defense_piece(&battle.state.object,piece)?;
        let band=usize::from(self.host.town_attack_band.ok_or("banda de asalto ausente")?);if band>=5 || active.object.get(0x1e98).copied()!=Some(254-band as u8){return Err("destino de banda desincronizado".into())}
        let index=self.host.town_attack_collision.as_ref().ok_or("índice de asalto ausente")?;if crate::battle_spatial::into_saved(index).map_err(str::to_owned)?!=battle.collisions{return Err("índice de asalto desincronizado".into())}
        let mut human=[false;2];for (side,value) in human.iter_mut().enumerate(){let at=0x35a+2*side;let bytes=battle.state.object.get(at..at+2).ok_or("participantes de torre truncados")?;let id=u16::from_le_bytes([bytes[0],bytes[1]]);if id!=0xff{*value=self.rep_merchants.get(usize::from(id)).ok_or("comerciante de torre ausente")?.u16(8)==0;}}
        let mut table=p3_core::ships::projectile::ProjectileTable::new(battle.state.projectiles.len());if table.bitmap.len()!=battle.state.bitmap.len(){return Err("bitmap de proyectiles inválido".into())}table.bitmap.clone_from(&battle.state.bitmap);
        for id in 0..table.slots.len(){if table.bitmap[id/32]&(1<<(id%32))!=0{table.slots[id]=Some(battle.state.projectiles[id].as_slice().try_into().map_err(|_|"proyectil truncado")?)}}
        let rng=self.host.investigation_rng.ok_or("RNG CRT de asalto ausente")?;
        let Some((scene,table,index,rng))=crate::pirate_active_frame::stage_tower_projectile_spawn(&battle.state.object,piece,&battle.state.local_ships,&battle.state.next,power,human,self.host.battle_difficulty,Some(angles),active.address,&table,index,rng).map_err(str::to_owned)? else{return Ok(false)};
        let mut next=active.clone();next.replace_object(&scene)?;battle.state.object=scene;battle.state.bitmap=table.bitmap;battle.state.projectiles=table.slots.into_iter().map(|p|p.map(|v|v.to_vec()).unwrap_or_else(||vec![0;0x38])).collect();battle.collisions=crate::battle_spatial::into_saved(&index).map_err(str::to_owned)?;next.populated=Some(battle);
        self.host.town_attack_active=Some(next);self.host.town_attack_collision=Some(index);self.host.investigation_rng=Some(rng);Ok(true)
    }

    /// Avanza explícitamente un proyectil de torre sin despachar colisiones.
    pub fn advance_pirate_tower_projectile(&mut self,id:usize)->Result<bool,String>{
        let active=self.host.town_attack_active.as_ref().ok_or("asalto activo ausente")?;if active.manager.len()<0xc38 || active.manager[0x42c]!=0{return Err("vista de asalto pendiente".into())}
        if self.host.expedition_resources.as_ref().ok_or("recursos navales ausentes")?.hotseat{return Err("proyectil en hotseat pendiente".into())}
        let mut battle=active.populated.clone().ok_or("escena poblada ausente")?;if active.object!=battle.state.object || !self.host.town_attack_save_unreconciled{return Err("vistas de asalto desincronizadas".into())}
        let index=self.host.town_attack_collision.as_ref().ok_or("índice de asalto ausente")?;if crate::battle_spatial::into_saved(index).map_err(str::to_owned)?!=battle.collisions{return Err("índice de asalto desincronizado".into())}
        let mut table=p3_core::ships::projectile::ProjectileTable::new(battle.state.projectiles.len());if table.bitmap.len()!=battle.state.bitmap.len(){return Err("bitmap de proyectiles inválido".into())}table.bitmap.clone_from(&battle.state.bitmap);
        for i in 0..table.slots.len(){if table.bitmap[i/32]&(1<<(i%32))!=0{table.slots[i]=Some(battle.state.projectiles[i].as_slice().try_into().map_err(|_|"proyectil truncado")?)}}
        let (scene,table,index,ended)=crate::pirate_active_frame::stage_tower_projectile_advance(&battle.state.object,id,&table,index).map_err(str::to_owned)?;let mut next=active.clone();next.replace_object(&scene)?;battle.state.object=scene;battle.state.bitmap=table.bitmap;battle.state.projectiles=table.slots.into_iter().map(|p|p.map(|v|v.to_vec()).unwrap_or_else(||vec![0;0x38])).collect();battle.collisions=crate::battle_spatial::into_saved(&index).map_err(str::to_owned)?;next.populated=Some(battle);self.host.town_attack_active=Some(next);self.host.town_attack_collision=Some(index);Ok(ended)
    }

    /// Despacha una pareja barco–proyectil de torre en pausa diagnóstica.
    pub fn apply_pirate_tower_projectile_ship_contact(&mut self,ship:u16,projectile:u16)->Result<bool,String>{
        let active=self.host.town_attack_active.as_ref().ok_or("asalto activo ausente")?;if active.manager.len()<0xc38 || active.manager[0x42c]!=0{return Err("vista de asalto pendiente".into())}if self.host.expedition_resources.as_ref().ok_or("recursos navales ausentes")?.hotseat{return Err("impacto en hotseat pendiente".into())}
        let mut battle=active.populated.clone().ok_or("escena poblada ausente")?;if active.object!=battle.state.object || !self.host.town_attack_save_unreconciled{return Err("vistas de asalto desincronizadas".into())}let index=self.host.town_attack_collision.as_ref().ok_or("índice de asalto ausente")?;if crate::battle_spatial::into_saved(index).map_err(str::to_owned)?!=battle.collisions{return Err("índice de asalto desincronizado".into())}
        let nav=self.host.navigation.as_ref().ok_or("flota de asalto ausente")?;let before=nav.snapshot.clone();let mut after=nav.clone();let mut table=p3_core::ships::projectile::ProjectileTable::new(battle.state.projectiles.len());if table.bitmap.len()!=battle.state.bitmap.len(){return Err("bitmap de proyectiles inválido".into())}table.bitmap.clone_from(&battle.state.bitmap);for i in 0..table.slots.len(){if table.bitmap[i/32]&(1<<(i%32))!=0{table.slots[i]=Some(battle.state.projectiles[i].as_slice().try_into().map_err(|_|"proyectil truncado")?)}}
        let Some(hit)=crate::pirate_active_frame::stage_tower_projectile_ship_impact(&battle.state.object,&battle.state.local_ships,&before.ships,&battle.spatial_0,&battle.spatial_1,&table,index,ship,projectile).map_err(str::to_owned)? else{return Ok(false)};
        let merchants=self.rep_merchants.iter().flat_map(|m|m.bytes.iter().copied()).collect::<Vec<_>>();let mut pool=self.host.message_pool.clone();let mut news=self.host.town_attack_global_news.clone();let date=self.idle.clock.calendar;for(captain,opponent)in hit.notices{if let Some(letter)=p3_core::ships::weapon_damage::pirate_sink_notice(captain,opponent,&before.captains,&merchants,self.economy.world.alderman_town).map_err(str::to_owned)?{p3_state::messages::add_global_inline_letter(&mut pool,date.day,date.month,date.year,&letter)?;news.push((date,letter));}}
        let mut next=active.clone();next.replace_object(&hit.scene)?;battle.state.object=hit.scene;battle.state.local_ships=hit.ships;battle.state.bitmap=hit.projectiles.bitmap;battle.state.projectiles=hit.projectiles.slots.into_iter().map(|p|p.map(|v|v.to_vec()).unwrap_or_else(||vec![0;0x38])).collect();battle.collisions=crate::battle_spatial::into_saved(&hit.spatial).map_err(str::to_owned)?;next.populated=Some(battle);after.snapshot.ships=hit.worlds;
        self.host.town_attack_active=Some(next);self.host.town_attack_collision=Some(hit.spatial);self.host.navigation=Some(after);self.host.message_pool=pool;self.host.town_attack_global_news=news;self.host.publish_navigation_delta(&before,&mut self.economy.world,&mut self.economy.cargo_ships);Ok(true)
    }

    /// Publica atómicamente el caso sin vista ni hotseat del contacto entre
    /// proyectil sin tirador y torre pirata. El llamante lo invoca únicamente
    /// al despachar ese par; no avanza el frame ni confirma S05.
    pub fn apply_pirate_tower_impact(&mut self,tower_slot:u16,projectile_slot:u16,
        viewed:bool,hotseat:bool)->Result<bool,String>{
        let active=self.host.town_attack_active.as_ref().ok_or("asalto activo ausente")?;
        let mut battle=active.populated.clone().ok_or("escena poblada ausente")?;
        if active.object!=battle.state.object || !self.host.town_attack_save_unreconciled {
            return Err("vistas de asalto desincronizadas".into());
        }
        let band_id=usize::from(self.host.town_attack_band.ok_or("banda de asalto ausente")?);
        if band_id>=5 || active.object.get(0x1e98).copied()!=Some(254-band_id as u8){return Err("destino de banda desincronizado".into())}
        let band=self.host.pirate_band_images.as_ref().and_then(|v|v.get(band_id)).and_then(Option::as_ref)
            .ok_or("imagen de banda ausente")?;
        let live=self.host.town_attack_collision.as_ref().ok_or("índice de asalto ausente")?;
        if crate::battle_spatial::into_saved(live).map_err(str::to_owned)?!=battle.collisions {
            return Err("índice de asalto desincronizado".into());
        }
        let count=battle.state.projectiles.len();let mut table=p3_core::ships::projectile::ProjectileTable::new(count);
        if table.bitmap.len()!=battle.state.bitmap.len(){return Err("bitmap de proyectiles incompatible".into())}
        table.bitmap.clone_from(&battle.state.bitmap);
        for i in 0..count {if table.bitmap[i/32]&(1u32<<(i%32))!=0 {
            table.slots[i]=Some(battle.state.projectiles[i].as_slice().try_into().map_err(|_|"proyectil truncado")?);
        }}
        let staged=crate::pirate_active_frame::stage_pirate_tower_impact(&battle.state.object,active.address,
            self.economy.world.towns.len() as u32,band,live,&self.host.council_scheduler,&table,
            tower_slot,projectile_slot,self.idle.clock.tick,viewed,hotseat)?;
        let mut next=active.clone();next.replace_object(&staged.scene)?;
        battle.state.object=staged.scene;battle.collisions=crate::battle_spatial::into_saved(&staged.collision).map_err(str::to_owned)?;
        battle.state.bitmap=staged.projectiles.bitmap;
        battle.state.projectiles=staged.projectiles.slots.into_iter().map(|p|p.map(|v|v.to_vec()).unwrap_or_else(||vec![0;0x38])).collect();
        next.populated=Some(battle);
        self.host.town_attack_active=Some(next);self.host.town_attack_collision=Some(staged.collision);
        self.host.council_scheduler=staged.scheduler;Ok(staged.hit)
    }
}
