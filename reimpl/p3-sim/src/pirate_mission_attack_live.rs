//! Preparación de la misión pirata desde las vistas vivas de la sesión.
use crate::{
    convoy_absorption::AbsorptionState,
    pirate_mission_attack_stage::{self, Stage},
    Sim,
};
use p3_core::{
    scheduler::TaskRecord,
    ships::{remove::RemoveWorld, tick::TickWorld},
};

impl Sim {
    /// Prepara desde la misión viva la apertura con dos barcos de IA. No
    /// publica ninguna de las vistas ni sustituye el despachador de sesión.
    /// El consumidor debe confirmar en conjunto la tarea, flota y escena.
    pub fn prepare_live_populated_pirate_scene<F>(
        &self,index:u16,expected:TaskRecord,
        resources:crate::pirate_mission_transaction::Resources<'_>,stack_initial:Option<u16>,
        pixel_to_cell:F,
    )->Result<Option<crate::pirate_mission_transaction::PopulatedPrepared>,String>
    where F:FnMut(i16,i16)->(u32,u32) {
        if resources.tick!=self.idle.clock.tick || resources.player!=self.economy.world.local_merchant
            || resources.fallback_town!=self.economy.world.alderman_town {
            return Err("recursos de escena de otra sesión o tick".into());
        }
        let locations=self.host.settlement_locations.as_ref().ok_or("posiciones urbanas ausentes")?;
        let loc=locations.get(usize::from(expected.u8_at(0x11))).ok_or("destino pirata fuera de rango")?;
        if resources.position!=(loc.x as u16,loc.y as u16) {return Err("posición de escena desincronizada".into())}
        let stage=self.prepare_live_pirate_attack(index,expected,false,stack_initial)?;
        stage.map(|stage|crate::pirate_mission_transaction::prepare_populated(stage,&self.rep_merchants,
            self.economy.world.merchant_count,self.economy.world.towns.len() as u32,resources,pixel_to_cell)).transpose()
    }

    /// Encadena las vistas de sesión y la escena cuando el gestor aportado
    /// acredita que no hay batalla previa. No publica datos ni efectos.
    pub(crate) fn prepare_live_pirate_scene(
        &self,
        index: u16,
        expected: TaskRecord,
        resources: crate::pirate_mission_transaction::Resources<'_>,
        stack_initial: Option<u16>,
    ) -> Result<Option<crate::pirate_mission_transaction::Prepared>, String> {
        if resources.tick != self.idle.clock.tick
            || resources.player != self.economy.world.local_merchant
            || resources.fallback_town != self.economy.world.alderman_town
        {
            return Err("recursos de escena de otra sesión o tick".into());
        }
        let locations = self
            .host
            .settlement_locations
            .as_ref()
            .ok_or("posiciones urbanas ausentes")?;
        let location = locations
            .get(usize::from(expected.u8_at(0x11)))
            .ok_or("destino pirata fuera de rango")?;
        if resources.position != (location.x as u16, location.y as u16) {
            return Err("posición de escena desincronizada".into());
        }
        let stage = self.prepare_live_pirate_attack(index, expected, false, stack_initial)?;
        stage
            .map(|stage| {
                crate::pirate_mission_transaction::prepare(
                    stage,
                    &self.rep_merchants,
                    self.economy.world.merchant_count,
                    resources,
                )
            })
            .transpose()
    }

    /// Solo prepara; la escena y todos sus efectos deben completarse antes de publicar.
    pub(crate) fn prepare_live_pirate_attack(
        &self,
        index: u16,
        expected: TaskRecord,
        existing_scene: bool,
        stack_initial: Option<u16>,
    ) -> Result<Option<Stage>, String> {
        let scheduler = &self.host.council_scheduler;
        if scheduler.head != index || scheduler.records.get(usize::from(index)) != Some(&expected) {
            return Err("tarea pirata cambió durante el despacho".into());
        }
        let owner = usize::from(expected.u8_at(0xd));
        if owner >= usize::from(self.economy.world.merchant_count) {
            return Ok(None);
        }
        let merchant = self.rep_merchants.get(owner).ok_or("comerciante pirata ausente")?;
        let nav = self.host.navigation.as_ref().ok_or("navegación pirata ausente")?;
        let factory = self.host.expedition_factory.as_ref().ok_or("fábrica naval ausente")?;
        let snap = &nav.snapshot;
        if snap.manager.len() < 0x110
            || snap.ships.len() % 0x180 != 0
            || snap.convoys.len() % 0x3c != 0
            || snap.captains.len() % 16 != 0
        {
            return Err("flota pirata truncada".into());
        }
        let r16 = |a: usize| u16::from_le_bytes([snap.manager[a], snap.manager[a + 1]]);
        let ships = snap.ships.len() / 0x180;
        let convoys = snap.convoys.len() / 0x3c;
        if usize::from(r16(0xf4)) != ships
            || usize::from(r16(0xf6)) != convoys
            || usize::from(r16(0xf2)) != snap.captains.len() / 16
            || factory.ships.len() != ships
            || factory.convoys.len() != convoys
            || self.host.expedition_ship_images.len() != ships
        {
            return Err("vistas de flota pirata incompletas".into());
        }
        // La selección usa enlaces, posiciones, estados y convoy de la misma
        // vista que la operación transaccional. Rechazar divergencias en las
        // cadenas consultadas; no interpretar punteros de imágenes obsoletas.
        let check = |mut at: usize| -> Result<(), String> {
            let mut seen = vec![false; ships];
            while at < ships {
                if seen[at] {
                    return Err("barcos de comerciante cíclicos".into());
                }
                seen[at] = true;
                let live = self.host.expedition_ship_images[at]
                    .as_ref()
                    .unwrap_or(&factory.ships[at]);
                let view = &snap.ships[at * 0x180..(at + 1) * 0x180];
                if live.len() < 0x136
                    || [(4, 10), (0x1e, 0x24), (0x134, 0x136)]
                        .iter()
                        .any(|&(a, b)| live[a..b] != view[a..b])
                {
                    return Err("vistas de barco pirata desincronizadas".into());
                }
                at = usize::from(u16::from_le_bytes([view[4], view[5]]));
            }
            Ok(())
        };
        check(usize::from(merchant.u16(0xe)))?;
        let tracked = usize::from(u16::from_le_bytes([expected.u8_at(0x14), expected.u8_at(0x15)]));
        if tracked < ships {
            let live = self.host.expedition_ship_images[tracked]
                .as_ref()
                .unwrap_or(&factory.ships[tracked]);
            let view = &snap.ships[tracked * 0x180..(tracked + 1) * 0x180];
            if live.len() < 0x24 || live[0x1e..0x24] != view[0x1e..0x24] {
                return Err("posición de barco seguido desincronizada".into());
            }
        }
        let live = self.host.pirate_band_images.as_ref().ok_or("bandas piratas ausentes")?;
        if live.len() != 5 {
            return Err("bandas piratas incompletas".into());
        }
        let mut bands = Vec::with_capacity(5);
        for (i, band) in live.iter().enumerate() {
            bands.push(if band.is_some() {
                Some(self.host.reconcile_pirate_band_image(i)?)
            } else {
                None
            })
        }
        let targets = self.host.pirate_targets.as_ref().ok_or("destinos piratas ausentes")?;
        let missions = self
            .host
            .mission_pirates
            .as_ref()
            .ok_or("vistas de misión pirata ausentes")?;
        for i in 0..5 {
            let target = bands[i].as_ref().map(|band| band.header[6]);
            if targets[i] != target || missions[i].as_ref().map(|mission| mission.target) != target {
                return Err("vistas de banda pirata desincronizadas".into());
            }
        }
        let locations = self
            .host
            .settlement_locations
            .as_ref()
            .ok_or("posiciones urbanas ausentes")?;
        let directions = self
            .host
            .settlement_sea_directions
            .as_ref()
            .ok_or("rumbos marítimos ausentes")?;
        let direction = *directions
            .get(usize::from(expected.u8_at(0x11)))
            .ok_or("destino pirata fuera de rango")?;
        let pool = self
            .host
            .route_stop_runtime
            .as_ref()
            .ok_or("depósito de rutas ausente")?;
        let pool_limit = p3_save::tail::RouteStopPool::parse(pool.bytes())
            .map_err(|e| format!("depósito de rutas: {e:?}"))?
            .count;
        let state = AbsorptionState {
            bands,
            day: self.economy.world.day_of_year,
            network_client: self.economy.world.network_client,
            operations: vec![],
        };
        let mut manager = snap.manager.clone();
        let mut ship_data = snap.ships.clone();
        let mut captain_data = snap.captains.clone();
        let mut convoy_data = snap.convoys.clone();
        let mut merchant_data = self
            .rep_merchants
            .iter()
            .flat_map(|v| v.bytes.iter().copied())
            .collect::<Vec<_>>();
        let mut town_data = self
            .economy
            .world
            .towns
            .iter()
            .flat_map(|v| v.0.iter().copied())
            .collect::<Vec<_>>();
        let mut remove = RemoveWorld {
            manager: &mut manager,
            ships: &mut ship_data,
            captains: &mut captain_data,
            convoys: &mut convoy_data,
            merchants: &mut merchant_data,
            towns: &mut town_data,
            pool_limit,
            merchant_count: self.economy.world.merchant_count,
            town_count: self.economy.world.towns.len() as u32,
            ui_a: false,
            ui_b: false,
        };
        let world = TickWorld {
            remove: &mut remove,
            time: self.idle.clock.tick,
            player: self.economy.world.local_merchant,
        };
        let date = self.idle.clock.calendar;
        let population = self.economy.world.towns.get(usize::from(date.day_of_year & 15))
            .map(|town| town.u32(0x2e0));
        let (first_names, last_names) = self.host.pirate_captain_name_counts().unwrap_or((0, 0));
        let create = self.host.expedition_resources.as_ref().and_then(|resources| {
            Some(crate::pirate_group_native_host::CreateInputs {
                town0: self.economy.world.towns.first()?.clone(),
                port0: resources.ports.first().copied().flatten()?,
                ports: locations.iter().map(|p| (p.x as u16, p.y as u16)).collect(),
                weapon_experience: self.economy.world.towns
                    .get(usize::from(self.economy.world.alderman_town))?.u32(0x810),
                ship_names: resources.ship_names.clone(),
            })
        });
        let factory_host = crate::pirate_group_native_host::NativeHost {
            captain: p3_core::economy::pirate_group_callbacks::CaptainResources {
                tick: self.idle.clock.tick, day_of_year: date.day_of_year, day: date.day,
                population: population.unwrap_or(0), first_names, last_names,
            },
            captain_population_available: population.is_some(), player: world.player, pool_limit,
            cleared_routes: vec![], remove_events: vec![], removed_plans: vec![],
            merchant_owners: vec![], create, operations: vec![],
        };
        let staged = pirate_mission_attack_stage::prepare_with_factory(
            &expected,
            targets,
            locations,
            direction,
            merchant.u16(0xe),
            &world,
            &state,
            self.economy.world.alderman_town,
            existing_scene,
            stack_initial,
            self.host.settlement_plans.as_ref().zip(self.host.town_pirate_piece_bytes),
            Some(&factory_host),
        )?;
        if let Some(Stage::Attack { attack, captains, .. }) = &staged {
            if self.host.expedition_convoy_images.len() != convoys {
                return Err("vistas de convoy pirata incompletas".into());
            }
            for (at, value) in [(0xe0, factory.free_captain), (0xe2, factory.free_ship),
                (0xe4, factory.free_convoy), (0xec, factory.convoy_head),
                (0xee, factory.name_cursor), (0xf0, factory.name_step)] {
                if attack.manager[at..at + 2] != snap.manager[at..at + 2] && value != r16(at) {
                    return Err("cabeza naval pirata desincronizada".into());
                }
            }
            if captains.len() < snap.captains.len() || captains.len() % 16 != 0 {
                return Err("capitanes preparados incompletos".into());
            }
            for i in 0..snap.captains.len() / 16 {
                let range = i * 16..(i + 1) * 16;
                if captains[range.clone()] != snap.captains[range.clone()]
                    && factory.captains.get(i).map(|c| c.as_slice()) != Some(&snap.captains[range]) {
                    return Err("capitán pirata desincronizado".into());
                }
            }
            for i in 0..ships {
                let range = i * 0x180..(i + 1) * 0x180;
                if attack.ships[range.clone()] != snap.ships[range.clone()] {
                    let live = self.host.expedition_ship_images[i]
                        .as_ref()
                        .unwrap_or(&factory.ships[i]);
                    if live.len()!=0x180{return Err(format!("barco vivo {i} truncado"))}
                    let p=i*0x180;
                    let old_state = u16::from_le_bytes([snap.ships[p+0x134], snap.ships[p+0x135]]);
                    let new_state = u16::from_le_bytes([attack.ships[p+0x134], attack.ships[p+0x135]]);
                    if old_state >= 0x17 && new_state < 0x17 {
                        if [6usize, 7, 0x134, 0x135, 0x137].into_iter()
                            .any(|off| live[off] != snap.ships[p+off]) {
                            return Err("ranura naval libre desincronizada".into());
                        }
                        continue;
                    }
                    // +0x24 es un puntero de ruta: navegación lo representa
                    // con un token o cero y conserva la ruta por separado.
                    if let Some(off)=live.iter().zip(&snap.ships[p..p+0x180]).enumerate().find_map(|(off,(a,b))|(!(0x24..0x28).contains(&off)&&a!=b).then_some(off)) {
                        return Err(format!("barco modificado desincronizado: {i} +0x{off:X} (vivo {:02x}, navegación {:02x})",live[off],snap.ships[p+off]));
                    }
                }
            }
            for i in 0..convoys {
                let range = i * 0x3c..(i + 1) * 0x3c;
                if attack.convoys[range.clone()] != snap.convoys[range.clone()]
                    && (self.host.expedition_convoy_images[i].as_slice() != &snap.convoys[range.clone()]
                        || factory.convoys[i].as_slice() != &snap.convoys[range])
                {
                    return Err("convoy modificado desincronizado".into());
                }
            }
        }
        Ok(staged)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::navigation::LiveVoyage;
    use p3_core::ai::settlement::SettlementLocation;
    use p3_save::{model::SaveModel, ships::Ships};
    #[test]
    fn live_wait_uses_save_views_and_rejects_stale_positions() {
        let Ok(root) = std::env::var("P3_SAVES") else { return };
        let data = std::fs::read(format!("{root}/visibility/save/Ein/AUTO.pat")).unwrap();
        let model = SaveModel::parse(&data).unwrap();
        let snapshot = Ships::parse(&model.ships.write())
            .unwrap()
            .navigation_snapshot()
            .unwrap();
        let mut sim = Sim::from_save_bytes(&data).unwrap();
        let (band, target) = sim
            .host
            .pirate_targets
            .as_ref()
            .unwrap()
            .iter()
            .enumerate()
            .find_map(|(i, x)| x.filter(|v| *v < 40).map(|v| (i, v)))
            .expect("se requiere banda con destino");
        let (owner, ship) = sim
            .rep_merchants
            .iter()
            .enumerate()
            .find_map(|(i, m)| {
                let ship = usize::from(m.u16(0xe));
                (ship < snapshot.ships.len() / 0x180).then_some((i, ship))
            })
            .expect("se requiere barco comercial");
        let image = &snapshot.ships[ship * 0x180..(ship + 1) * 0x180];
        let x = i32::from(i16::from_le_bytes([image[0x1e], image[0x1f]]));
        let y = i32::from(i16::from_le_bytes([image[0x22], image[0x23]]));
        let mut locations = [SettlementLocation { x: 0, y: 0, kind: 0 }; 40];
        locations[target as usize].x = x + 1;
        locations[target as usize].y = y;
        let ports = vec![(100, 500); sim.economy.world.towns.len()];
        sim.host.navigation = Some(LiveVoyage::idle(snapshot, ports, owner as u32));
        sim.economy.world.local_merchant = owner as u32;
        sim.host.settlement_locations = Some(locations);
        sim.host.settlement_sea_directions = Some([0; 40]);
        let tick = sim.idle.clock.tick;
        let mut scheduler = sim.host.council_scheduler.clone();
        let mut id = scheduler.head;
        while id < scheduler.capacity {
            scheduler.records[id as usize].due = tick + 0x100000;
            id = scheduler.records[id as usize].next
        }
        let id = scheduler.add_task(tick);
        let t = &mut scheduler.records[id as usize];
        t.kind = 0x32;
        t.payload = [0; 16];
        t.set_u32_at(8, tick + 0x10000);
        t.set_u8_at(0xc, 3);
        t.set_u8_at(0xd, owner as u8);
        t.set_u8_at(0xf, 1);
        t.set_u8_at(0x10, band as u8);
        t.set_u8_at(0x11, target);
        t.set_u8_at(0x12, 1);
        t.set_u8_at(0x14, 255);
        t.set_u8_at(0x15, 255);
        sim.host.council_scheduler = scheduler;
        let task = sim.host.council_scheduler.records[id as usize];
        assert!(matches!(
            sim.prepare_live_pirate_attack(id, task, false, None).unwrap(),
            Some(Stage::Wait(_))
        ));
        let resources = || crate::pirate_mission_transaction::Resources {
            previous_manager: &[],
            heap_image: &[],
            address: 0,
            tick,
            position: ((x + 1) as u16, y as u16),
            portrait: (0, 0),
            projection: p3_core::ships::town_attack_mask::TownProjection {
                width: 34,
                height: 16,
                x_offset: 17,
                y_offset: 8,
            },
            collision_id: 0,
            fallback_town: sim.economy.world.alderman_town,
            report_flag: 0,
            first_name: &[],
            last_name: &[],
            format: &[],
            own_separator: &[],
            enemy_separator: &[],
            player: sim.economy.world.local_merchant,
            dialog_active: 0,
            simple_text: &[],
            reward_format: &[],
        };
        assert!(matches!(
            sim.prepare_live_pirate_scene(id, task, resources(), None).unwrap(),
            Some(crate::pirate_mission_transaction::Prepared::Wait(_))
        ));
        assert!(matches!(sim.prepare_live_populated_pirate_scene(id,task,resources(),None,|_,_|(0,0)).unwrap(),
            Some(crate::pirate_mission_transaction::PopulatedPrepared::Wait(_))));
        let mut bad = resources();
        bad.position.0 = bad.position.0.wrapping_add(1);
        assert!(sim
            .prepare_live_pirate_scene(id, task, bad, None)
            .err()
            .unwrap()
            .contains("posición"));
        let before = sim.host.expedition_ship_images[ship].clone();
        let image = sim.host.expedition_ship_images[ship].as_mut().expect("imagen viva");
        image[0x1e] ^= 1;
        assert!(sim
            .prepare_live_pirate_attack(id, task, false, None)
            .err()
            .unwrap()
            .contains("desincronizadas"));
        sim.host.expedition_ship_images[ship] = before;
        sim.host.settlement_sea_directions = None;
        assert!(sim
            .prepare_live_pirate_attack(id, task, false, None)
            .err()
            .unwrap()
            .contains("rumbos"));
        // Un convoy sintético en la misma imagen de AUTO alcanza el ataque;
        // la preparación tampoco debe modificar las vistas de sesión.
        sim.host.settlement_sea_directions = Some([0; 40]);
        sim.host.settlement_locations.as_mut().unwrap()[target as usize].x = x;
        let snap = &mut sim.host.navigation.as_mut().unwrap().snapshot;
        assert!(snap.convoys.len() >= 0x3c);
        snap.manager[0xea..0xec].fill(255);
        snap.manager[0xec..0xee].copy_from_slice(&0u16.to_le_bytes());
        let base = ship * 0x180;
        let image = &mut snap.ships[base..base + 0x180];
        image[4..8].fill(255);
        image[8..10].copy_from_slice(&0u16.to_le_bytes());
        image[10..14].fill(255);
        image[0x134..0x136].copy_from_slice(&15u16.to_le_bytes());
        let image = image.to_vec();
        sim.host.expedition_ship_images[ship] = Some(image.clone());
        sim.host.expedition_factory.as_mut().unwrap().ships[ship] = image;
        let convoy = &mut snap.convoys[..0x3c];
        convoy[8..10].fill(255);
        convoy[0xa..0xc].copy_from_slice(&(ship as u16).to_le_bytes());
        convoy[0x10..0x12].copy_from_slice(&(ship as u16).to_le_bytes());
        let convoy = convoy.to_vec();
        sim.host.expedition_convoy_images[0] = convoy.clone();
        let factory = sim.host.expedition_factory.as_mut().unwrap();
        factory.convoys[0] = convoy;
        factory.convoy_head = 0;
        let before = sim.host.navigation.as_ref().unwrap().snapshot.clone();
        let stage = sim.prepare_live_pirate_attack(id, task, true, None).unwrap().unwrap();
        assert!(matches!(stage, Stage::Attack { .. }));
        assert_eq!(sim.host.navigation.as_ref().unwrap().snapshot, before);
        sim.host.expedition_convoy_images[0][0x12] ^= 1;
        assert!(sim
            .prepare_live_pirate_attack(id, task, true, None)
            .err()
            .unwrap()
            .contains("convoy modificado"));
        assert_eq!(sim.host.navigation.as_ref().unwrap().snapshot, before);
        sim.host.expedition_convoy_images[0][0x12] ^= 1;
        assert!(sim
            .apply_live_pirate_scene(id, task)
            .unwrap_err()
            .contains("recursos explícitos"));
        assert_eq!(sim.host.navigation.as_ref().unwrap().snapshot, before);
        assert!(sim.host.navigation.as_ref().unwrap().snapshot.convoys.len() >= 2 * 0x3c);
        {
            let nav = &mut sim.host.navigation.as_mut().unwrap().snapshot;
            nav.ships[base] = owner as u8;
            nav.convoys[0] = owner as u8;
            nav.convoys[0x3c + 0xa..0x3c + 0xc].fill(255);
            sim.host.expedition_ship_images[ship].as_mut().unwrap()[0] = owner as u8;
            sim.host.expedition_factory.as_mut().unwrap().ships[ship][0] = owner as u8;
            for i in 0..2 {
                let v = nav.convoys[i * 0x3c..(i + 1) * 0x3c].to_vec();
                sim.host.expedition_convoy_images[i] = v.clone();
                sim.host.expedition_factory.as_mut().unwrap().convoys[i] = v;
            }
        }
        for p in [&mut sim.rep_merchants[owner], &mut sim.economy.merchants[owner]] {
            p.set_u16(8, 1);
        }
        let band_image = sim.host.pirate_band_images.as_mut().unwrap()[band].as_mut().unwrap();
        band_image.header[..2].fill(0);
        band_image.header[4..6].fill(255);
        band_image.header[8..10].copy_from_slice(&1u16.to_le_bytes());
        band_image.header[10..12].fill(255);
        band_image.header[14..16].copy_from_slice(&1u16.to_le_bytes());
        band_image.pieces.clear();
        let mut plan = vec![0; 0x64 + 64];
        plan[..4].copy_from_slice(&8u32.to_le_bytes());
        plan[4..8].copy_from_slice(&8u32.to_le_bytes());
        plan[0x64..].fill(6);
        band_image.plan = Some(plan);
        let mission = sim.host.mission_pirates.as_mut().unwrap()[band].as_mut().unwrap();
        mission.reserve_strength = 1;
        mission.first_ship = 0xffff;
        let config = crate::pirate_mission_publish::Config {
            band: band as u8,
            heap_image: vec![0; 0x2f24],
            address: 0x54000000,
            projection: p3_core::ships::town_attack_mask::TownProjection {
                width: 34,
                height: 21,
                x_offset: 17,
                y_offset: 11,
            },
            portrait: (45, 12),
            first_name: b"First".to_vec(),
            last_name: b"Last".to_vec(),
            format: b"VICT %s %s %d".to_vec(),
            own_separator: b"Own".to_vec(),
            enemy_separator: b"Enemy".to_vec(),
            report_flag: 0,
            dialog_active: 0,
            simple_text: vec![],
            reward_format: vec![],
            stack_initial: None,
            verify_live_portrait: false,
        };
        sim.host.pirate_scene_config = Some(config.clone());
        sim.host.pirate_scene_config.as_mut().unwrap().heap_image.clear();
        let before = sim.host.navigation.as_ref().unwrap().snapshot.clone();
        let pool = sim.host.message_pool.clone();
        let sched = sim.host.council_scheduler.clone();
        assert!(sim.apply_live_pirate_scene(id, task).unwrap_err().contains("asignador"));
        assert_eq!(sim.host.navigation.as_ref().unwrap().snapshot, before);
        assert_eq!(sim.host.message_pool, pool);
        assert_eq!(sim.host.council_scheduler.records, sched.records);
        // El barco suelto con el pool de convoyes lleno debe rechazarse
        // antes de modificar ninguna vista ni consumir su configuración.
        let count = u16::from_le_bytes(before.manager[0xf6..0xf8].try_into().unwrap());
        let original_free = u16::from_le_bytes(before.manager[0xe4..0xe6].try_into().unwrap());
        {
            let nav = &mut sim.host.navigation.as_mut().unwrap().snapshot;
            nav.manager[0xe4..0xe6].copy_from_slice(&count.to_le_bytes());
            nav.manager[0xea..0xec].copy_from_slice(&(ship as u16).to_le_bytes());
            nav.ships[base + 8..base + 10].fill(255);
        }
        sim.host.expedition_ship_images[ship].as_mut().unwrap()[8..10].fill(255);
        {
            let f = sim.host.expedition_factory.as_mut().unwrap();
            f.ships[ship][8..10].fill(255);
            f.free_convoy = count;
        }
        let mut single_config = config.clone();
        single_config.stack_initial = Some(0xffff);
        sim.host.pirate_scene_config = Some(single_config);
        let full_before = sim.host.navigation.as_ref().unwrap().snapshot.clone();
        assert!(sim
            .apply_live_pirate_scene(id, task)
            .unwrap_err()
            .contains("ampliación del pool"));
        assert_eq!(sim.host.navigation.as_ref().unwrap().snapshot, full_before);
        assert_eq!(sim.host.message_pool, pool);
        assert_eq!(sim.host.council_scheduler.records, sched.records);
        assert!(sim.host.pirate_scene_config.is_some());
        {
            let nav = &mut sim.host.navigation.as_mut().unwrap().snapshot;
            nav.manager[0xe4..0xe6].copy_from_slice(&original_free.to_le_bytes());
            nav.manager[0xea..0xec].fill(255);
            nav.ships[base + 8..base + 10].copy_from_slice(&0u16.to_le_bytes());
        }
        sim.host.expedition_ship_images[ship].as_mut().unwrap()[8..10].copy_from_slice(&0u16.to_le_bytes());
        {
            let f = sim.host.expedition_factory.as_mut().unwrap();
            f.ships[ship][8..10].copy_from_slice(&0u16.to_le_bytes());
            f.free_convoy = original_free;
        }
        sim.host.pirate_scene_config = Some(config);
        // El reloj deja la tarea sin consumir antes del puerto.
        let end=sim.advance_with_merchant_daily(tick+1,false).unwrap();
        assert_eq!(end,p3_core::tick::AdvanceEnd::InterruptedBeforeTask);
        assert!(sim.host.pending_pirate_boundary.is_some());
        assert!(sim.host.town_attack_active.is_none());
        assert!(sim.persist_all(&mut model.clone()).is_err());
        assert!(sim.resume_pirate_boundary().unwrap_err().contains("capturar"));
        assert!(sim.host.pending_pirate_attack.is_some());
        sim.host.investigation_rng=Some(p3_core::rng::CrtRand{state:0x12345678});
        let queued=p3_core::pump::Operation([0xb5,0,0,0,0]);
        sim.host.council_operations.push(queued);
        assert!(sim.stage_pirate_boundary_baseline(&model).unwrap_err().contains("operaciones pendientes"));
        assert_eq!(sim.host.council_operations,vec![queued]);
        assert!(sim.host.pirate_boundary_model.is_none());
        sim.host.council_operations.pop();
        sim.stage_pirate_boundary_baseline(&model).unwrap();
        assert!(sim.host.pirate_boundary_model.is_some());
        assert_eq!(sim.host.pending_pirate_attack.as_ref().map(|p|p.0),Some(id));
        assert_eq!(sim.resume_pirate_boundary().unwrap(),p3_core::tick::AdvanceEnd::InterruptedBeforeBattle);
        assert!(sim.host.pending_pirate_attack.is_none());
        assert!(sim.host.town_attack_active.is_some());
        assert!(sim.host.town_attack_collision.is_some());
        assert!(!sim.host.town_attack_effects.is_empty());
        assert!(sim.host.pirate_scene_config.is_none());
        assert!(sim.require_no_active_town_attack_save().is_err());
        assert!(sim.advance_with_merchant_daily(tick + 1, false).is_err());
        assert!(sim.advance_pumped_until(tick + 1, 8, false).is_err());
        assert!(!sim.take_town_attack_effects().is_empty());
        // La reanudación real detuvo el tick antes del gestor de batalla.
        assert!(sim.host.clock_before_battle);
        assert!(sim.host.town_attack_tick_pending);
        assert!(sim.advance_town_attack_frame(1, false, false, false).is_err());
        sim.prepare_town_attack_manager().unwrap();
        assert!(sim.complete_town_attack_tick().is_err());
        assert!(!sim.take_town_attack_manager_effects().is_empty());
        sim.complete_town_attack_tick().unwrap();
        assert!(!sim.host.clock_before_battle);
        assert!(!sim.host.town_attack_tick_pending);
        sim.host.navigation.as_mut().unwrap().snapshot.manager[0xea..0xec]
            .copy_from_slice(&(ship as u16).to_le_bytes());
        let inconsistent = sim.host.navigation.as_ref().unwrap().snapshot.clone();
        assert!(sim
            .advance_town_attack_frame(1, false, false, false)
            .unwrap_err()
            .contains("líder de asalto ya enlazado"));
        assert_eq!(sim.host.navigation.as_ref().unwrap().snapshot, inconsistent);
        sim.host.navigation.as_mut().unwrap().snapshot.manager[0xea..0xec].fill(255);
        sim.host.investigation_rng = Some(p3_core::rng::CrtRand { state: 0x12345678 });
        sim.economy.world.network_client = 0;
        sim.host.settlement_inland=Some([0;40]);
        sim.host.town_pirate_piece_bytes=Some((0x5a,0xa5));
        sim.host.expedition_resources=Some(crate::HanseFleetResources{ship_names:vec![b"Alpha".to_vec(),b"Brig".to_vec(),b"Crown".to_vec()],first_names:75,last_names:80,ports:vec![Some((100,500));sim.economy.world.towns.len()],hotseat:false,player:0});
        let before_bad_delta = sim.host.town_attack_active.as_ref().unwrap().manager.clone();
        let bad_frame=p3_core::pump::Operation([0x94,0,0,0,0]);
        sim.host.council_operations.push(bad_frame);
        assert!(!sim.dispatch_pending_council_operations().is_empty());
        assert_eq!(sim.host.council_operations,vec![bad_frame]);
        sim.host.town_attack_active.as_mut().unwrap().object[0x2f1a]=1;
        assert!(sim.dispatch_town_attack_frame_operation(false, false, false, 0).is_err());
        assert_eq!(sim.host.council_operations,vec![bad_frame]);
        assert_eq!(sim.host.town_attack_active.as_ref().unwrap().manager, before_bad_delta);
        sim.host.town_attack_active.as_mut().unwrap().object[0x2f1a]=0;
        sim.host.council_operations[0].0[1]=2;
        sim.dispatch_town_attack_frame_operation(false, false, false, 0).unwrap();
        assert!(sim.host.town_attack_active.is_none());
        assert!(sim.host.town_attack_collision.is_none());
        assert!(!sim.host.town_attack_frame_effects.is_empty());
        assert!(sim.require_no_active_town_attack_save().is_err());
        assert!(sim.complete_town_attack_tick().is_err());
        assert!(!sim.take_town_attack_frame_effects().is_empty());
        assert!(sim.require_no_active_town_attack_save().unwrap_err().contains("conciliar con S05"));
        assert_eq!(sim.host.council_operations.first().map(|op|op.0[0]),Some(0xb4));
        assert!(sim.stage_completed_pirate_attack().is_err());
        let errors=sim.dispatch_pending_council_operations();
        assert!(errors.is_empty(),"{errors:?}");
        // Representa un cambio independiente del tick posterior a la captura.
        // Debe sobrevivir a la conciliación y a la recarga de S05.
        let sieges=&mut sim.host.founded_world.as_mut().unwrap().sieges;
        sieges[0][10]^=1;
        let expected_siege=sieges[0].clone();
        let saved=sim.stage_completed_pirate_attack().unwrap();
        let restored=Sim::from_save_bytes(&saved.write()).unwrap();
        assert_eq!(saved.world.sieges[0],expected_siege);
        assert_eq!(restored.host.founded_world.as_ref().unwrap().sieges[0],expected_siege);
        assert_eq!(restored.host.message_pool,sim.host.message_pool);
        assert_eq!(restored.host.pirate_band_images,sim.host.pirate_band_images);
        assert_eq!(restored.host.council_scheduler.head,sim.host.council_scheduler.head);
        assert_eq!(restored.host.council_scheduler.capacity,sim.host.council_scheduler.capacity);
        let mut wrong=saved.clone();wrong.world.header[0]^=1;
        assert!(sim.confirm_completed_pirate_attack(&wrong).is_err());
        assert!(sim.require_no_active_town_attack_save().is_err());
        sim.confirm_completed_pirate_attack(&saved).unwrap();
        assert!(sim.require_no_active_town_attack_save().is_ok());
        let mut second=saved.clone();
        sim.persist_all(&mut second).unwrap();
        Sim::from_save_bytes(&second.write()).unwrap();
    }
}

#[cfg(test)]
mod navigation_projection_regression {
    use super::*;
    #[test]
    fn freed_ship_projects_no_owner_even_if_runtime_byte_is_preserved() {
        let Ok(root) = std::env::var("P3_SAVES") else { return };
        let data = std::fs::read(format!("{root}/visibility/save/Ein/AUTO.pat")).unwrap();
        let model = p3_save::model::SaveModel::parse(&data).unwrap();
        let snapshot = p3_save::ships::Ships::parse(&model.ships.write())
            .unwrap()
            .navigation_snapshot()
            .unwrap();
        let mut sim = Sim::from_save_bytes(&data).unwrap();
        let ship = snapshot
            .ships
            .chunks_exact(0x180)
            .position(|s| u16::from_le_bytes([s[0x134], s[0x135]]) < 0x17)
            .unwrap();
        let ports = vec![(100, 500); sim.economy.world.towns.len()];
        sim.host.navigation = Some(crate::navigation::LiveVoyage::idle(snapshot, ports, 0));
        let before = sim.host.navigation.as_ref().unwrap().snapshot.clone();
        let image = &mut sim.host.navigation.as_mut().unwrap().snapshot.ships[ship * 0x180..(ship + 1) * 0x180];
        image[0] = 7;
        image[0x134..0x136].copy_from_slice(&0xffu16.to_le_bytes());
        sim.host
            .publish_navigation_delta(&before, &mut sim.economy.world, &mut sim.economy.cargo_ships);
        assert_eq!(sim.host.navigation.as_ref().unwrap().snapshot.ships[ship * 0x180], 7);
        assert_eq!(sim.economy.world.ships[ship].owner, 255);
        assert_eq!(sim.economy.world.ships[ship].state, 0xff);
    }
}
