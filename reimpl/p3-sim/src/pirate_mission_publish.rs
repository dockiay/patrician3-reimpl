//! Entrada explícita y publicación conjunta de una escena pirata viva.
use crate::{
    pirate_empty_hideout_messages,
    pirate_mission_transaction::{Prepared, Resources},
    Sim,
};
use p3_core::{
    scheduler::TaskRecord,
    ships::{spatial::CollisionIndex, town_attack_mask::TownProjection},
};

#[derive(Clone)]
pub struct Config {
    pub band: u8,
    pub heap_image: Vec<u8>,
    pub address: u32,
    pub projection: TownProjection,
    pub portrait: (u8, u8),
    pub first_name: Vec<u8>,
    pub last_name: Vec<u8>,
    pub format: Vec<u8>,
    pub own_separator: Vec<u8>,
    pub enemy_separator: Vec<u8>,
    pub report_flag: u8,
    pub dialog_active: u8,
    pub simple_text: Vec<u8>,
    pub reward_format: Vec<u8>,
    pub stack_initial: Option<u16>,
    /// Las pruebas aisladas pueden aportar retratos sintéticos explícitos.
    /// En sesión se recalcula desde la banda preparada al despachar.
    pub verify_live_portrait: bool,
}
impl Sim {
    /// Publica la variante poblada acotada a dos barcos de IA sin torres.
    /// El frame y el guardado permanecen protegidos hasta conectar sus efectos.
    pub fn publish_live_populated_pirate_scene<F>(&mut self,index:u16,expected:TaskRecord,
        resources:Resources<'_>,stack_initial:Option<u16>,pixel_to_cell:F)->Result<(),String>
    where F:FnMut(i16,i16)->(u32,u32) {
        if self.host.town_attack_active.is_some() || self.host.town_attack_collision.is_some()
            || !self.host.town_attack_effects.is_empty() {
            return Err("escena o efectos de asalto pendientes".into());
        }
        let previous=if let Some(b)=&self.idle.battle_manager {
            if b.error.is_some() || b.manager.len()<0x405 || b.manager[..0x3fc].iter().any(|&v|v!=0) {
                return Err("gestor con batalla pendiente".into());
            }
            p3_save::tail::serialize_basic_sea_battles(&[],&crate::battle_manager::groups_to_save(&b.groups))
                .map_err(|e|format!("gestor naval: {e:?}"))?
        } else {self.host.town_attack_origin.clone()};
        if resources.previous_manager!=previous {return Err("gestor de asalto desincronizado".into())}
        let dialog=resources.dialog_active;
        let prepared=self.prepare_live_populated_pirate_scene(index,expected,resources,stack_initial,pixel_to_cell)?
            .ok_or("misión pirata ya no aplicable")?;
        let crate::pirate_mission_transaction::PopulatedPrepared::Attack(prepared)=prepared else {
            return Err("el ataque pirata aún no alcanzó su destino".into());
        };
        let collision=crate::battle_spatial::into_live(&prepared.scene.collisions).map_err(str::to_owned)?;
        let people=self.rep_merchants.clone();
        if people.len()!=self.economy.merchants.len()
            || people.len()!=usize::from(self.economy.world.merchant_count)
            || self.economy.world.merchants.len()<people.len() {
            return Err("vistas de comerciantes incompletas".into());
        }
        let raw=people.iter().flat_map(|m|m.bytes.iter().copied()).collect::<Vec<_>>();
        let scene=crate::pirate_empty_hideout_scene::Prepared{
            object:prepared.scene.state.object.clone(),merchants:raw,start_effects:vec![],letters:vec![],
            human_finish_effects:vec![],dialog_active:None,convoy_ship_owners:vec![prepared.attack.owner],attacker_count:1,
        };
        let published=pirate_empty_hideout_messages::Published {scene:scene.clone(),
            pool:self.host.message_pool.clone(),effects:vec![]};
        let mut manager=prepared.manager;manager.populated=Some(prepared.scene);
        let tx=crate::pirate_mission_transaction::AttackTransaction{attack:prepared.attack,bands:prepared.bands,
            captains:prepared.captains,manager,scene};
        self.commit_prepared_pirate_scene(index,expected,expected.u8_at(0x10),dialog,tx,published,collision,people)
    }

    /// Prepara recursos de la banda a partir de los bancos originales ya
    /// cargados. El heap y los estados de UI son entradas explícitas.
    pub fn configure_pirate_scene_from_live(
        &mut self,
        band: usize,
        heap_image: Vec<u8>,
        address: u32,
        dialog_active: u8,
        report_flag: u8,
        stack_initial: Option<u16>,
    ) -> Result<(), String> {
        if band >= 5 || heap_image.len() != 0x2f24 || address == 0 || address.checked_add(0x2f24).is_none() {
            return Err("banda, heap o dirección de escena inválidos".into());
        }
        if self.host.town_attack_active.is_some() || self.host.pirate_scene_config.is_some() {
            return Err("escena o configuración de asalto ya existente".into());
        }
        let image = self.host.reconcile_pirate_band_image(band)?;
        let nav = self.host.navigation.as_ref().ok_or("flota de asalto ausente")?;
        let locations = self
            .host
            .settlement_locations
            .as_ref()
            .ok_or("posiciones urbanas ausentes")?;
        crate::pirate_hideout_resources::position(&image, locations)?;
        let count = self.host.foundation_names.ok_or("contador de nombres ausente")?.1;
        let (_, given, family) =
            crate::pirate_hideout_resources::portrait(&image, &nav.snapshot.ships, &nav.snapshot.captains, count)?;
        let given_bank = self.host.first_names.as_ref().ok_or("banco de nombres ausente")?;
        let family_bank = self.host.family_names.as_ref().ok_or("banco de apellidos ausente")?;
        let (first, last) = crate::pirate_hideout_resources::names(given, family, given_bank, family_bank)?;
        let texts = self
            .host
            .pirate_scene_texts
            .as_ref()
            .ok_or("textos de asalto ausentes")?;
        let texts = crate::pirate_hideout_resources::texts(|slot| texts.get(slot))?;
        let projection = self.host.pirate_scene_projection.ok_or("geometría de asalto ausente")?;
        self.host.pirate_scene_config = Some(Config {
            band: band as u8,
            heap_image,
            address,
            projection,
            portrait: (given, family),
            first_name: first.to_vec(),
            last_name: last.to_vec(),
            format: texts.victory,
            own_separator: texts.own_separator,
            enemy_separator: texts.enemy_separator,
            report_flag,
            dialog_active,
            simple_text: texts.human,
            reward_format: texts.reward,
            stack_initial,
            verify_live_portrait: true,
        });
        Ok(())
    }

    /// Prepara y publica una sola escena después de validar todas las vistas.
    /// Los recursos de heap, nombres, textos y diálogo son evidencia del host.
    /// El consumidor debe procesar efectos y ejecutar el frame antes de avanzar.
    pub(crate) fn apply_live_pirate_scene(&mut self, index: u16, expected: TaskRecord) -> Result<(), String> {
        if self.host.town_attack_active.is_some()
            || self.host.town_attack_collision.is_some()
            || !self.host.town_attack_effects.is_empty()
        {
            return Err("escena o efectos de asalto pendientes".into());
        }
        let stack_initial = self
            .host
            .pirate_scene_config
            .as_ref()
            .and_then(|config| config.stack_initial);
        if let Some(crate::pirate_mission_attack_stage::Stage::Wait(task)) =
            self.prepare_live_pirate_attack(index, expected, true, stack_initial)?
        {
            let mut scheduler = self.host.council_scheduler.clone();
            scheduler.records[usize::from(index)] = task;
            scheduler.reschedule_first();
            scheduler.dispatching = 0;
            self.host.council_scheduler = scheduler;
            return Ok(());
        }
        let config = self
            .host
            .pirate_scene_config
            .as_ref()
            .ok_or("recursos explícitos de asalto ausentes")?.clone();
        if config.band != expected.u8_at(0x10) {
            return Err("recursos de otra banda pirata".into());
        }
        let (portrait, first_name, last_name, report_flag) = if config.verify_live_portrait {
            // 0x515AE0 se ejecuta después de 0x514530, que puede mover
            // barcos de la reserva. No usar el retrato preparado al configurar.
            let stage = self.prepare_live_pirate_attack(index, expected, false, config.stack_initial)?
                .ok_or("misión pirata ya no aplicable")?;
            let crate::pirate_mission_attack_stage::Stage::Attack { attack, bands, captains } = stage else {
                return Err("ataque pirata aún no alcanzó su destino".into());
            };
            let band = bands.bands.get(usize::from(config.band)).and_then(Option::as_ref)
                .ok_or("banda pirata ausente")?;
            let count = self.host.foundation_names.ok_or("contador de nombres ausente")?.1;
            let (_, given, family) = crate::pirate_hideout_resources::portrait(
                band, &attack.ships, &captains, count,
            )?;
            let given_bank = self.host.first_names.as_ref().ok_or("banco de nombres ausente")?;
            let family_bank = self.host.family_names.as_ref().ok_or("banco de apellidos ausente")?;
            let (first, last) = crate::pirate_hideout_resources::names(given, family, given_bank, family_bank)?;
            let flag=self.rep_merchants.get(usize::from(attack.owner)).ok_or("comerciante del informe ausente")?.get_merchant_epithet_index();
            ((given, family), first.to_vec(), last.to_vec(), flag)
        } else {
            (config.portrait, config.first_name.clone(), config.last_name.clone(), config.report_flag)
        };
        let locations = self
            .host
            .settlement_locations
            .as_ref()
            .ok_or("posiciones de asalto ausentes")?;
        let loc = locations
            .get(usize::from(expected.u8_at(0x11)))
            .ok_or("destino de asalto inválido")?;
        let previous = if let Some(b) = &self.idle.battle_manager {
            if b.error.is_some() || b.manager.len() < 0x405 || b.manager[..0x3fc].iter().any(|&v| v != 0) {
                return Err("gestor con batalla pendiente".into());
            }
            p3_save::tail::serialize_basic_sea_battles(&[], &crate::battle_manager::groups_to_save(&b.groups))
                .map_err(|e| format!("gestor naval: {e:?}"))?
        } else {
            self.host.town_attack_origin.clone()
        };
        let mut collision = CollisionIndex::new(32).map_err(str::to_owned)?;
        let args = p3_core::ships::town_attack_mask::LAND_COLLISION_ARGS;
        let collision_id = collision
            .add(
                args[0] as u8,
                args[1] as u8,
                args[2] as i16,
                args[3] as i16,
                args[4] as i16,
                args[5] as i16,
            )
            .ok_or("índice de colisión lleno")?;
        let r = Resources {
            previous_manager: &previous,
            heap_image: &config.heap_image,
            address: config.address,
            tick: self.idle.clock.tick,
            position: (loc.x as u16, loc.y as u16),
            portrait,
            projection: config.projection,
            collision_id,
            fallback_town: self.economy.world.alderman_town,
            report_flag,
            first_name: &first_name,
            last_name: &last_name,
            format: &config.format,
            own_separator: &config.own_separator,
            enemy_separator: &config.enemy_separator,
            player: self.economy.world.local_merchant,
            dialog_active: config.dialog_active,
            simple_text: &config.simple_text,
            reward_format: &config.reward_format,
        };
        let Prepared::Attack(tx) = self
            .prepare_live_pirate_scene(index, expected, r, config.stack_initial)?
            .ok_or("misión pirata ya no aplicable")?
        else {
            return Err("el ataque pirata dejó de estar en destino".into());
        };
        let date = self.idle.clock.calendar;
        let published = pirate_empty_hideout_messages::prepare(
            tx.scene.clone(),
            &self.host.message_pool,
            self.economy.world.merchant_count,
            (date.day, date.month, date.year),
            &p3_state::messages::RETENTION,
        )?;
        let mut people = Vec::new();
        for chunk in published.scene.merchants.chunks_exact(0x650) {
            people.push(p3_core::ai::reputation::Merchant { bytes: chunk.to_vec() })
        }
        if people.len() != self.rep_merchants.len()
            || people.len() != self.economy.merchants.len()
            || self.economy.world.merchants.len() < usize::from(self.economy.world.merchant_count)
        {
            return Err("vistas de comerciantes incompletas".into());
        }
        self.commit_prepared_pirate_scene(index,expected,config.band,config.dialog_active,tx,published,collision,people)
    }

    /// Publica las vistas ya validadas; no ejecuta el frame de escena.
    fn commit_prepared_pirate_scene(&mut self,index:u16,expected:TaskRecord,band:u8,dialog_active:u8,
        tx:crate::pirate_mission_transaction::AttackTransaction,
        published:pirate_empty_hideout_messages::Published,collision:CollisionIndex,
        people:Vec<p3_core::ai::reputation::Merchant>)->Result<(),String> {
        let nav = self.host.navigation.as_ref().ok_or("flota ausente")?;
        let mut before = nav.snapshot.clone();
        let mut updated = nav.clone();
        let old_ships = before.ships.len() / 0x180;
        let old_convoys = before.convoys.len() / 0x3c;
        let old_caps = before.captains.len() / 16;
        let ships = tx.attack.ships.len() / 0x180;
        let convoys = tx.attack.convoys.len() / 0x3c;
        let caps = tx.captains.len() / 16;
        if tx.attack.ships.len() % 0x180 != 0 || ships < old_ships
            || tx.attack.convoys.len() % 0x3c != 0 || convoys < old_convoys
            || tx.captains.len() % 16 != 0 || caps < old_caps
            || updated.baseline.ships.len() != before.ships.len()
            || updated.baseline.convoys.len() != before.convoys.len()
            || updated.baseline.routes.len() != old_ships || before.routes.len() != old_ships
            || self.host.expedition_factory.as_ref().is_none_or(|f|
                f.ships.len() != old_ships || f.convoys.len() != old_convoys || f.captains.len() != old_caps)
            || self.host.expedition_convoy_images.len() != old_convoys
        { return Err("pools de asalto incompatibles con las vistas vivas".into()); }
        updated.snapshot.manager = tx.attack.manager.clone();
        updated.snapshot.ships = tx.attack.ships.clone();
        updated.snapshot.convoys = tx.attack.convoys.clone();
        updated.snapshot.captains = tx.captains.clone();
        for i in 0..ships {
            let image = &updated.snapshot.ships[i*0x180..(i+1)*0x180];
            let active = u16::from_le_bytes([image[0x134], image[0x135]]) < 0x17;
            if i >= old_ships {
                let route = active.then_some(p3_save::ships::SavedRoute::None);
                updated.snapshot.routes.push(route.clone());
                updated.baseline.ships.extend_from_slice(image);
                updated.baseline.routes.push(route);
                let mut empty = vec![0; 0x180];
                empty[0x134..0x136].fill(255);
                before.ships.extend_from_slice(&empty);
                before.routes.push(None);
            } else if active && u16::from_le_bytes([before.ships[i*0x180+0x134], before.ships[i*0x180+0x135]]) >= 0x17 {
                updated.snapshot.routes[i] = Some(p3_save::ships::SavedRoute::None);
            }
        }
        updated.baseline.convoys.extend_from_slice(&updated.snapshot.convoys[old_convoys*0x3c..]);
        updated.baseline.captains.extend_from_slice(&updated.snapshot.captains[old_caps*16..]);
        updated.baseline.manager[0xf2..0xf8].copy_from_slice(&updated.snapshot.manager[0xf2..0xf8]);
        before.convoys.resize(convoys * 0x3c, 0);
        crate::navigation::refresh_lists(&mut updated.snapshot)?;
        let mut scheduler = self.host.council_scheduler.clone();
        if scheduler.head != index || scheduler.records.get(usize::from(index)) != Some(&expected) {
            return Err("tarea cambió antes de publicar".into());
        }
        scheduler.records[usize::from(index)] = tx.attack.task_after_scene(true);
        scheduler.reschedule_first();
        scheduler.dispatching = 0;
        let mut manager = tx.manager;
        manager.replace_object(&published.scene.object)?;
        manager.manager[0x3fd] = published.scene.dialog_active.unwrap_or(dialog_active);
        // Desde aquí solo hay asignaciones a vistas ya validadas.
        self.economy.world.ship_count = ships as u16;
        self.economy.world.ships.resize_with(ships, || p3_core::economy::world::Ship { owner: 255, state: 255 });
        self.economy.cargo_ships.resize_with(ships, p3_core::economy::ship_trade::ShipCargo::default);
        self.host.expedition_ship_images.resize_with(ships, || None);
        self.host.expedition_original_ship_images.resize_with(ships, || None);
        self.host.debt_ship_metadata.resize(ships, (u16::MAX, 255, 0, u16::MAX));
        self.host.debt_convoy_flags.resize(convoys, 0);
        let factory = self.host.expedition_factory.as_mut().expect("factoría validada");
        for image in updated.snapshot.ships.chunks_exact(0x180).skip(old_ships) { factory.ships.push(image.to_vec()); }
        for image in updated.snapshot.convoys.chunks_exact(0x3c).skip(old_convoys) {
            factory.convoys.push(image.to_vec());
            self.host.expedition_convoy_images.push(image.to_vec());
        }
        for image in updated.snapshot.captains.chunks_exact(16).skip(old_caps) {
            factory.captains.push(image.try_into().expect("capitán completo"));
        }
        self.economy.merchants.clone_from(&people);
        self.rep_merchants = people;
        for i in 0..usize::from(self.economy.world.merchant_count) {
            self.economy.world.merchants[i].gold = self.rep_merchants[i].i32(0);
            self.economy.world.merchants[i].income = self.rep_merchants[i].i32(0x4c8)
        }
        if let Some(missions) = self.host.mission_pirates.as_mut() {
            for (mission, band) in missions.iter_mut().zip(&tx.bands.bands) {
                if let (Some(m), Some(b)) = (mission, band) {
                    m.first_ship = u16::from_le_bytes([b.header[4], b.header[5]]);
                    m.target = b.header[6];
                    m.status = b.header[12];
                    m.reserve_strength = u16::from_le_bytes([b.header[14], b.header[15]])
                }
            }
        }
        if let Some(pirates) = self.host.hanse_pirates.as_mut() {
            for (head, band) in pirates.heads.iter_mut().zip(&tx.bands.bands) {
                *head = band.as_ref().map(|b| u16::from_le_bytes([b.header[4], b.header[5]]));
            }
            pirates.next = updated.snapshot.ships.chunks_exact(0x180)
                .map(|s| u16::from_le_bytes([s[4], s[5]])).collect();
            pirates.ship_captains = updated.snapshot.ships.chunks_exact(0x180)
                .map(|s| u16::from_le_bytes([s[0x42], s[0x43]])).collect();
            pirates.captains = updated.snapshot.captains.chunks_exact(16)
                .map(|c| c.try_into().expect("capitán completo")).collect();
        }
        self.host.pirate_band_images = Some(tx.bands.bands);
        self.host.navigation = Some(updated);
        self.host.council_scheduler = scheduler;
        self.host.message_pool = published.pool;
        self.host.town_attack_effects = published.effects;
        self.host.town_attack_collision = Some(collision);
        self.host.town_attack_active = Some(manager);
        self.host.town_attack_save_unreconciled = true;
        self.host.town_attack_band = Some(band);
        self.host.pirate_scene_config = None;
        self.host
            .publish_navigation_delta(&before, &mut self.economy.world, &mut self.economy.cargo_ships);
        Ok(())
    }
}
