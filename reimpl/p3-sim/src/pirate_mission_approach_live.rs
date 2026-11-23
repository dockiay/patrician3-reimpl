//! Publicación conjunta de la aproximación 0x32 y sus reposiciones piratas.
use crate::{
    convoy_absorption::AbsorptionState,
    pirate_mission_approach_stage::{self, Inputs},
    Sim,
};
use p3_core::{economy::ship_trade::ShipCargo, scheduler::TaskRecord};

impl Sim {
    pub(crate) fn apply_pirate_mission_approach(&mut self, index: u16, expected: TaskRecord) -> Result<(), String> {
        if self.host.council_scheduler.head != index
            || self.host.council_scheduler.records.get(usize::from(index)) != Some(&expected)
        {
            return Err("tarea de misión cambió durante el despacho".into());
        }
        let mut scheduler = self.host.council_scheduler.clone();
        let live = self.host.pirate_band_images.as_ref().ok_or("bandas navales ausentes")?;
        if live.len() != 5 {
            return Err("bandas navales incompletas".into());
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
            let expected_target = bands[i].as_ref().map(|band| band.header[6]);
            if targets[i] != expected_target || missions[i].as_ref().map(|mission| mission.target) != expected_target {
                return Err(format!("vistas de banda {i} desincronizadas"));
            }
        }
        let state = AbsorptionState {
            bands,
            day: self.economy.world.day_of_year,
            network_client: self.economy.world.network_client,
            operations: Vec::new(),
        };
        let nav = self.host.navigation.as_ref().ok_or("navegación de misión ausente")?;
        let pool = self
            .host
            .route_stop_runtime
            .as_ref()
            .ok_or("depósito vivo de paradas ausente")?;
        let locations = self
            .host
            .settlement_locations
            .as_ref()
            .ok_or("faltan posiciones de StadtDaten.ini")?;
        let factory = self.host.expedition_factory.as_ref().ok_or("fábrica naval ausente")?;
        let old_ships = nav.snapshot.ships.len() / 0x180;
        let old_caps = nav.snapshot.captains.len() / 16;
        if nav.snapshot.ships.len() % 0x180 != 0
            || nav.snapshot.captains.len() % 16 != 0
            || factory.ships.len() != old_ships
            || factory.captains.len() != old_caps
            || nav.snapshot.manager.len() < 0x110
            || nav.snapshot.routes.len() != old_ships
            || nav.baseline.ships.len() != nav.snapshot.ships.len()
            || nav.baseline.routes.len() != old_ships
            || self.economy.world.ship_count as usize != old_ships
            || self.economy.world.ships.len() != old_ships
            || self.economy.cargo_ships.len() != old_ships
            || self.host.expedition_original_ship_images.len() != old_ships
            || self.host.debt_ship_metadata.len() != old_ships
        {
            return Err("vistas para reposición pirata incompletas".into());
        }
        let manager = &nav.snapshot.manager;
        let word = |at: usize| u16::from_le_bytes([manager[at], manager[at + 1]]);
        if usize::from(word(0xf4)) != old_ships
            || usize::from(word(0xf2)) != old_caps
            || nav.snapshot.convoys.len() % 0x3c != 0
            || usize::from(word(0xf6)) != nav.snapshot.convoys.len() / 0x3c
            || factory.convoys.len() != nav.snapshot.convoys.len() / 0x3c
        {
            return Err("contadores navales desincronizados".into());
        }
        // La reserva se cuenta por +6. Cada vista debe señalar la misma lista.
        for band in state
            .bands
            .iter()
            .flatten()
            .filter(|band| band.header[6] == expected.u8_at(0x11) && band.plan.is_none())
        {
            let mut at = usize::from(u16::from_le_bytes([band.header[10], band.header[11]]));
            let mut seen = vec![false; old_ships];
            while at < old_ships {
                if seen[at] {
                    return Err("reserva pirata cíclica".into());
                }
                seen[at] = true;
                let navship = &nav.snapshot.ships[at * 0x180..(at + 1) * 0x180];
                let source = factory.ships.get(at).ok_or("barco de reserva ausente")?;
                if source.len() < 8 || source[6..8] != navship[6..8] {
                    return Err("vistas de reserva pirata desincronizadas".into());
                }
                if let Some(Some(live)) = self.host.expedition_ship_images.get(at) {
                    if live.len() < 8 || live[6..8] != navship[6..8] {
                        return Err("vistas de reserva pirata desincronizadas".into());
                    }
                }
                at = usize::from(u16::from_le_bytes([navship[6], navship[7]]));
            }
        }
        // La detección en el scheduler usa las imágenes de expedición. La
        // transacción lee navegación; debe ser la misma cadena y posición.
        if self.host.expedition_ship_images.len() != old_ships {
            return Err("imágenes de expedición incompletas".into());
        }
        let owner = usize::from(expected.u8_at(0xd));
        let merchant = self.rep_merchants.get(owner).ok_or("comerciante de misión ausente")?;
        let mut at = usize::from(merchant.u16(0xe));
        let mut seen = vec![false; old_ships];
        while at < old_ships {
            if seen[at] {
                return Err("barcos de comerciante cíclicos".into());
            }
            seen[at] = true;
            let live = self.host.expedition_ship_images[at]
                .as_ref()
                .unwrap_or(&factory.ships[at]);
            let image = &nav.snapshot.ships[at * 0x180..(at + 1) * 0x180];
            if live.len() < 0x24
                || live[4..6] != image[4..6]
                || live[0x1e..0x20] != image[0x1e..0x20]
                || live[0x22..0x24] != image[0x22..0x24]
            {
                return Err("vistas de posición pirata desincronizadas".into());
            }
            at = usize::from(u16::from_le_bytes([image[4], image[5]]));
        }
        let date = self.idle.clock.calendar;
        let input = Inputs {
            tick: self.idle.clock.tick,
            day: date.day,
            day_of_year: date.day_of_year,
            player: self.economy.world.local_merchant,
            alderman_town: self.economy.world.alderman_town,
            merchant_count: self.economy.world.merchant_count,
            world_lock: self.economy.world.world_8a,
            network: self.economy.world.network_client,
            difficulty: self.host.pirate_difficulty,
            thresholds: self.host.pirate_group_thresholds,
            names: self.host.pirate_captain_name_counts(),
            resources: self.host.expedition_resources.as_ref(),
            locations,
        };
        let mut staged = pirate_mission_approach_stage::prepare(
            &expected,
            nav,
            &state,
            &self.economy.world.towns,
            &self.rep_merchants,
            pool,
            &self.host.merchant_ship_groups,
            input,
        )?
        .ok_or("aproximación pirata dejó de ser válida")?;
        let count = staged.nav.snapshot.ships.len() / 0x180;
        if count < old_ships
            || staged.nav.snapshot.ships.len() % 0x180 != 0
            || staged.nav.snapshot.captains.len() < nav.snapshot.captains.len()
            || staged.nav.snapshot.captains.len() % 16 != 0
            || staged.nav.snapshot.routes.len() != count
            || staged.merchants.len() != self.rep_merchants.len()
        {
            return Err("resultado de reposición incompatible".into());
        }
        // La asignación solo lee estas cabezas y cursores cuando se crean
        // barcos; una banda ya preparada no depende de ellos.
        let allocated = staged.nav.snapshot.manager[0xe0..0xe4] != manager[0xe0..0xe4]
            || staged.nav.snapshot.manager[0xee..0xf2] != manager[0xee..0xf2];
        if allocated
            && [
                (0xe0, factory.free_captain),
                (0xe2, factory.free_ship),
                (0xee, factory.name_cursor),
                (0xf0, factory.name_step),
            ]
            .into_iter()
            .any(|(at, value)| word(at) != value)
        {
            return Err("vistas del asignador pirata desincronizadas".into());
        }
        // Comprobar las ranuras libres que la preparación realmente consumió.
        // Las demás pueden conservar bytes no serializados distintos.
        for i in 0..old_ships {
            let old = &nav.snapshot.ships[i * 0x180..(i + 1) * 0x180];
            let new = &staged.nav.snapshot.ships[i * 0x180..(i + 1) * 0x180];
            let old_state = u16::from_le_bytes([old[0x134], old[0x135]]);
            let new_state = u16::from_le_bytes([new[0x134], new[0x135]]);
            if old_state >= 0x17 && new_state < 0x17 {
                let source = &factory.ships[i];
                if source.len() < 0x138
                    || source[6..8] != old[6..8]
                    || source[0x134..0x136] != old[0x134..0x136]
                    || source[0x137] != old[0x137]
                {
                    return Err("ranura naval libre desincronizada".into());
                }
            }
        }
        for i in 0..old_caps {
            let old = &nav.snapshot.captains[i * 16..(i + 1) * 16];
            let new = &staged.nav.snapshot.captains[i * 16..(i + 1) * 16];
            if old != new && old != factory.captains[i] {
                return Err("ranura de capitán desincronizada".into());
            }
        }
        let mut before = nav.snapshot.clone();
        if count > old_ships || staged.nav.snapshot.captains.len() > before.captains.len() {
            for i in old_ships..count {
                let image = &staged.nav.snapshot.ships[i * 0x180..(i + 1) * 0x180];
                staged.nav.baseline.ships.extend_from_slice(image);
                staged.nav.baseline.routes.push(staged.nav.snapshot.routes[i].clone());
                let mut old = vec![0; 0x180];
                old[0x134..0x136].copy_from_slice(&0xffu16.to_le_bytes());
                before.ships.extend_from_slice(&old);
            }
            staged.nav.baseline.manager[0xf4..0xf6].copy_from_slice(&(count as u16).to_le_bytes());
        }
        scheduler.records[usize::from(index)] = staged.task;
        scheduler.reschedule_first();
        scheduler.dispatching = 0;
        // Desde aquí todo está preparado y validado; las publicaciones no fallan.
        self.economy.world.ship_count = count as u16;
        self.economy
            .world
            .ships
            .resize_with(count, || p3_core::economy::world::Ship { owner: 255, state: 255 });
        self.economy.cargo_ships.resize_with(count, ShipCargo::default);
        self.host.expedition_ship_images.resize_with(count, || None);
        self.host.expedition_original_ship_images.resize_with(count, || None);
        self.host.debt_ship_metadata.resize(count, (u16::MAX, 255, 0, u16::MAX));
        let factory = self.host.expedition_factory.as_mut().expect("fábrica validada");
        for image in staged.nav.snapshot.ships.chunks_exact(0x180).skip(factory.ships.len()) {
            factory.ships.push(image.to_vec())
        }
        for image in staged
            .nav
            .snapshot
            .captains
            .chunks_exact(16)
            .skip(factory.captains.len())
        {
            factory.captains.push(image.try_into().expect("capitán completo"))
        }
        if let Some(missions) = self.host.mission_pirates.as_mut() {
            for (mission, band) in missions.iter_mut().zip(&staged.bands.bands) {
                match (mission, band) {
                    (slot, None) => *slot = None,
                    (Some(m), Some(b)) => {
                        m.first_ship = u16::from_le_bytes([b.header[4], b.header[5]]);
                        m.target = b.header[6];
                        m.status = b.header[12];
                        m.reserve_strength = u16::from_le_bytes([b.header[14], b.header[15]]);
                    }
                    (None, Some(_)) => {}
                }
            }
        }
        if let Some(targets) = self.host.pirate_targets.as_mut() {
            for (dst, band) in targets.iter_mut().zip(&staged.bands.bands) {
                *dst = band.as_ref().map(|b| b.header[6])
            }
        }
        if let Some(p) = self.host.hanse_pirates.as_mut() {
            for (dst, band) in p.heads.iter_mut().zip(&staged.bands.bands) {
                *dst = band.as_ref().map(|b| u16::from_le_bytes([b.header[4], b.header[5]]))
            }
            p.next = staged
                .nav
                .snapshot
                .ships
                .chunks_exact(0x180)
                .map(|s| u16::from_le_bytes([s[4], s[5]]))
                .collect();
            p.ship_captains = staged
                .nav
                .snapshot
                .ships
                .chunks_exact(0x180)
                .map(|s| u16::from_le_bytes([s[0x42], s[0x43]]))
                .collect();
            p.captains = staged
                .nav
                .snapshot
                .captains
                .chunks_exact(16)
                .map(|c| c.try_into().expect("capitán completo"))
                .collect();
        }
        self.host.pirate_band_images = Some(staged.bands.bands);
        self.host.council_operations.extend(staged.bands.operations);
        self.host.route_stop_runtime = Some(staged.pool);
        self.host.merchant_ship_groups = staged.groups;
        for id in staged.changed_groups {
            if !self.host.merchant_ship_groups_changed.contains(&id) {
                self.host.merchant_ship_groups_changed.push(id)
            }
        }
        self.economy.world.towns = staged.towns;
        self.economy.merchants.clone_from(&staged.merchants);
        self.rep_merchants = staged.merchants;
        self.host.navigation = Some(staged.nav);
        self.host.council_scheduler = scheduler;
        self.host
            .publish_navigation_delta(&before, &mut self.economy.world, &mut self.economy.cargo_ships);
        Ok(())
    }
}
