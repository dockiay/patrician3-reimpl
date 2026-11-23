//! Operación B5 de la cola viva: preparar todos los efectos antes de publicarlos.
use crate::{pirate_b5_context, pirate_b5_effects::PirateB5Effects, pirate_b5_stage, Sim};
use p3_core::{economy::pirate_b5_memory::NativeBasics, pump::Operation, ships::{remove::RemoveWorld, tick::TickWorld}};
use p3_save::ships::SavedRoute;

impl Sim {
    pub(crate) fn apply_pirate_b5_operation(&mut self, op: Operation) -> Result<(), String> {
        let nav = self.host.navigation.as_ref().ok_or("flota B5 ausente")?;
        let mut before = nav.snapshot.clone();
        let mut staged = nav.clone();
        let bands = self.host.pirate_band_images.as_ref().ok_or("bandas B5 ausentes")?
            .iter().enumerate().map(|(i, band)| {
                if band.is_some() { self.host.reconcile_pirate_band_image(i).map(Some) } else { Ok(None) }
            }).collect::<Result<Vec<_>, String>>()?;
        let locations = self.host.settlement_locations.as_ref().ok_or("posiciones B5 ausentes")?;
        let pool = self.host.route_stop_runtime.as_ref().ok_or("depósito B5 ausente")?;
        let extra = pirate_b5_context::extra_memory(locations, self.host.battle_difficulty,
            self.economy.world.alderman_town, pool.bytes())?;
        let mut native = NativeBasics::default();
        native.player = self.economy.world.local_merchant;
        let effects = PirateB5Effects::new(native, self.host.message_pool.clone(),
            self.host.council_scheduler.clone(), pool.clone(), self.host.merchant_ship_groups.clone(),
            self.idle.clock.calendar);
        let mut towns = self.economy.world.towns.iter().flat_map(|t| t.0.iter().copied()).collect::<Vec<_>>();
        let mut merchants = self.rep_merchants.iter().flat_map(|m| m.bytes.iter().copied()).collect::<Vec<_>>();
        let snapshot = &mut staged.snapshot;
        let mut remove = RemoveWorld { manager: &mut snapshot.manager, ships: &mut snapshot.ships,
            captains: &mut snapshot.captains, convoys: &mut snapshot.convoys,
            merchants: &mut merchants, towns: &mut towns, pool_limit: 0,
            merchant_count: self.economy.world.merchant_count,
            town_count: self.economy.world.towns.len() as u32, ui_a: false, ui_b: false };
        let world = TickWorld { remove: &mut remove, time: self.idle.clock.tick, player: self.economy.world.local_merchant };
        let prepared = pirate_b5_stage::prepare(&world, &bands, &extra, op, &effects)?;
        if prepared.ships.len() != before.ships.len() || prepared.captains.len() != before.captains.len()
            || prepared.convoys.len() < before.convoys.len() || prepared.convoys.len() % 0x3c != 0 {
            return Err("dimensiones de pools B5 incompatibles".into());
        }
        if prepared.merchants.len() != self.rep_merchants.len() * 0x650
            || prepared.towns.len() != self.economy.world.towns.len() * 0x9f8
            || prepared.host.merchant_groups.len() != self.host.merchant_ship_groups.len() {
            return Err("vistas B5 incompletas".into());
        }
        staged.snapshot.manager = prepared.manager;
        staged.snapshot.ships = prepared.ships;
        staged.snapshot.captains = prepared.captains;
        staged.snapshot.convoys = prepared.convoys;
        for &id in &prepared.host.native.cleared_routes {
            let route = staged.snapshot.routes.get_mut(usize::from(id)).ok_or("ruta B5 fuera de flota")?;
            let at = usize::from(id) * 0x180;
            let state = u16::from_le_bytes(staged.snapshot.ships[at + 0x134..at + 0x136].try_into().unwrap());
            *route = if state >= 0x17 { None } else { Some(SavedRoute::None) };
            staged.forget_captured_route(id);
        }
        crate::navigation::refresh_lists(&mut staged.snapshot)?;
        let changed_groups = prepared.host.merchant_groups.iter().zip(&self.host.merchant_ship_groups)
            .enumerate().filter_map(|(i, (new, old))| (new != old).then_some(i)).collect::<Vec<_>>();
        let count = usize::from(self.economy.world.merchant_count);
        if self.economy.merchants.len() < count || self.economy.world.merchants.len() < count {
            return Err("cuentas B5 incompletas".into());
        }
        let old_convoys = before.convoys.len() / 0x3c;
        let new_convoys = staged.snapshot.convoys.len() / 0x3c;
        if new_convoys > old_convoys {
            let factory = self.host.expedition_factory.as_ref().ok_or("fábrica B5 ausente")?;
            if staged.baseline.convoys.len() != before.convoys.len()
                || factory.convoys.len() != old_convoys
                || self.host.expedition_convoy_images.len() != old_convoys
                || self.host.debt_convoy_flags.len() != old_convoys
                || u16::try_from(new_convoys).is_err()
                || u16::from_le_bytes(staged.snapshot.manager[0xf6..0xf8].try_into().unwrap()) as usize != new_convoys {
                return Err("vistas del crecimiento de convoy B5 incompatibles".into());
            }
        }
        // No quedan operaciones falibles: publicar todas las vistas juntas.
        if new_convoys > old_convoys {
            let factory = self.host.expedition_factory.as_mut().expect("fábrica comprobada");
            for image in staged.snapshot.convoys.chunks_exact(0x3c).skip(old_convoys) {
                staged.baseline.convoys.extend_from_slice(image);
                before.convoys.extend_from_slice(image);
                factory.convoys.push(image.to_vec());
                self.host.expedition_convoy_images.push(image.to_vec());
                self.host.debt_convoy_flags.push(u32::from_le_bytes(image[0x18..0x1c].try_into().unwrap()));
            }
            staged.baseline.manager[0xf6..0xf8].copy_from_slice(&(new_convoys as u16).to_le_bytes());
        }
        for (town, bytes) in self.economy.world.towns.iter_mut().zip(prepared.towns.chunks_exact(0x9f8)) { town.0.copy_from_slice(bytes); }
        for (i, bytes) in prepared.merchants.chunks_exact(0x650).enumerate() {
            self.rep_merchants[i].bytes.copy_from_slice(bytes);
            if i < count {
                self.economy.merchants[i] = self.rep_merchants[i].clone();
                self.economy.world.merchants[i].gold = self.rep_merchants[i].i32(0);
                self.economy.world.merchants[i].income = self.rep_merchants[i].i32(0x4c8);
            }
        }
        for i in changed_groups { if !self.host.merchant_ship_groups_changed.contains(&i) { self.host.merchant_ship_groups_changed.push(i); } }
        self.host.merchant_ship_groups = prepared.host.merchant_groups;
        self.host.message_pool = prepared.host.message_pool;
        self.host.mission_variables.resize(usize::from(prepared.host.scheduler.capacity), None);
        self.host.council_scheduler = prepared.host.scheduler;
        self.host.route_stop_runtime = Some(prepared.host.route_stops);
        if let Some(missions) = self.host.mission_pirates.as_mut() {
            for (mission, band) in missions.iter_mut().zip(&prepared.bands) {
                match (mission, band) {
                    (slot, None) => *slot = None,
                    (Some(m), Some(b)) => { m.first_ship = u16::from_le_bytes([b.header[4], b.header[5]]); m.target = b.header[6]; m.status = b.header[12]; m.reserve_strength = u16::from_le_bytes([b.header[14], b.header[15]]); },
                    (None, Some(_)) => {},
                }
            }
        }
        if let Some(targets) = self.host.pirate_targets.as_mut() {
            for (target, band) in targets.iter_mut().zip(&prepared.bands) { *target = band.as_ref().map(|b| b.header[6]); }
        }
        if let Some(pirates) = self.host.hanse_pirates.as_mut() {
            for (head, band) in pirates.heads.iter_mut().zip(&prepared.bands) { *head = band.as_ref().map(|b| u16::from_le_bytes([b.header[4], b.header[5]])); }
            pirates.next = staged.snapshot.ships.chunks_exact(0x180).map(|s| u16::from_le_bytes([s[4], s[5]])).collect();
            pirates.ship_captains = staged.snapshot.ships.chunks_exact(0x180).map(|s| u16::from_le_bytes([s[0x42], s[0x43]])).collect();
            pirates.captains = staged.snapshot.captains.chunks_exact(16).map(|c| c.try_into().expect("capitán completo")).collect();
        }
        self.host.pirate_band_images = Some(prepared.bands);
        for _ in prepared.host.notifications { staged.events.push("pirata:actualizar_interfaz".into()); }
        self.host.navigation = Some(staged);
        self.host.publish_navigation_delta(&before, &mut self.economy.world, &mut self.economy.cargo_ships);
        Ok(())
    }
}
