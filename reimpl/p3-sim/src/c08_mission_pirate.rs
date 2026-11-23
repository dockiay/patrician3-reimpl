//! C08: operación 0x82 `op_spawn_mission_pirate_ship` (0x0053FB00): crea un barco pirata de misión con
//! `spawn_pirate_ship_near_ship` (0x005063C0), ajusta su capitán (`+2/+3`), la ciudad de patrulla
//! (0x0051A820, `+0x159`) y `+0x136 = 0x80`, y anota el barco en la variable de la tarea de guion 0x1B.
//!
//! El alta usa el mismo camino transaccional que los sucesos urbanos (`town_pirate_spawn::prepare` y
//! `town_event_live::prepare`); los efectos de 0x0053FB00 se aplican sobre la flota preparada antes de
//! integrarla, así navegación, fábrica y retratos reciben las mismas imágenes. La variable de guion pasa
//! por el puente plano de C08. Contraste: `tests/c08_mission_pirate_live_x86.rs`.
use crate::c08_flat::{guarded, SharedMem, OP};
use crate::{town_event_live, town_pirate_spawn, u16_at, u32_at, Ship, Sim};
use p3_core::flatmem::{Mem, SparseMem};
use p3_core::pump::Operation;

const SHIPS: u32 = 0x6000_0000;
const CAPTAINS: u32 = 0x6200_0000;
const MANAGER: u32 = 0x006d_d7a0;

struct Host {
    mem: SharedMem,
    ship: u16,
    expected: (u16, u32),
    /// Gestor, barcos y capitanes tras el alta: se escriben en la memoria al llamar a 0x005063C0.
    after: (Vec<u8>, Vec<u8>, Vec<u8>),
    error: Option<String>,
}

impl p3_core::economy::pirate_ops::MissionShipHost for Host {
    fn spawn(&mut self, arg0: u32, arg1: u32) -> u32 {
        // 0x005063C0 solo lee la palabra baja del barco de referencia y el modo.
        if (arg0 as u16, arg1) != self.expected {
            self.error.get_or_insert_with(|| "alta pirata de misión con argumentos distintos de los preparados".into());
        }
        let mut m = self.mem.clone();
        seed_fleet(&mut m, &self.after.0, &self.after.1, &self.after.2);
        u32::from(self.ship)
    }
    fn ship_adjust(&mut self, ship: u32, value: u32) {
        let mut m = self.mem.clone();
        p3_core::economy::pirate_small::ship_set_pirate_patrol_town(&mut m, ship, value);
    }
    fn object_at(&mut self, index: u32) -> u32 {
        // 0x004D8DB0 sobre 0x006DD73C.
        let mut m = self.mem.clone();
        if u32::from(m.r16(0x006d_d748)) < index {
            0
        } else {
            m.r32(0x006d_d73c).wrapping_add(index * 0x18)
        }
    }
}

/// Gestor en 0x006DD7A0 con sus punteros a las tablas de barcos y capitanes del puente.
fn seed_fleet(m: &mut dyn Mem, manager: &[u8], ships: &[u8], captains: &[u8]) {
    for (k, &b) in manager.iter().enumerate() {
        m.w8(MANAGER + k as u32, b);
    }
    m.w32(MANAGER, CAPTAINS);
    m.w32(MANAGER + 4, SHIPS);
    for (k, &b) in ships.iter().enumerate() {
        m.w8(SHIPS + k as u32, b);
    }
    for (k, &b) in captains.iter().enumerate() {
        m.w8(CAPTAINS + k as u32, b);
    }
}

impl Sim {
    /// Bandas vivas de suceso conciliadas con su vista de misión (como `event_bands_now` del despacho de tareas).
    pub(crate) fn c08_event_bands(&self) -> Result<Vec<Option<p3_save::pirate_bands::PirateBand>>, String> {
        let current = self.host.pirate_band_images.as_ref().ok_or("bandas de suceso ausentes")?;
        if current.len() != 5 {
            return Err("bandas de suceso incompletas".into());
        }
        let saved = self.host.pirate_band_images_saved.borrow().clone();
        current
            .iter()
            .enumerate()
            .map(|(i, image)| match image {
                None => Ok(None),
                Some(image) => {
                    let mission = self.host.mission_pirates.as_ref().and_then(|v| v[i].as_ref()).ok_or("vista de misión de banda ausente")?;
                    let last = saved.as_ref().and_then(|v| v.get(i)).and_then(Option::as_ref);
                    let base = self.host.pirate_band_images_initial.as_ref().and_then(|v| v.get(i)).and_then(Option::as_ref).or(last).unwrap_or(image);
                    crate::reconcile_event_band(base, image, mission, last, i).map(Some)
                }
            })
            .collect()
    }

    /// 0x82: barco pirata de misión.
    pub(crate) fn c08_mission_pirate_ship(&mut self, op: Operation) -> Result<(), String> {
        let bytes = op.0.iter().flat_map(|w| w.to_le_bytes()).collect::<Vec<_>>();
        let target = u16::from_le_bytes([bytes[4], bytes[5]]);
        let mode = bytes[0xb];
        let nav = self.host.navigation.as_ref().ok_or("flota de misión pirata no reconstruida")?;
        let count = usize::from(self.economy.world.ship_count);
        if nav.snapshot.ships.len() != count * 0x180 || nav.snapshot.manager.len() < 0xf8 {
            return Err("flota de misión pirata truncada".into());
        }
        // Barco de referencia en la tabla que no está en los estados 0xF/0x10: el original no hace nada.
        if usize::from(target) < count && !matches!(u16_at(&nav.snapshot.ships, usize::from(target) * 0x180 + 0x134), 0xf | 0x10) {
            return Ok(());
        }
        self.c08_merchant_views_synced()?;
        let factory = self.host.expedition_factory.as_ref().ok_or("fábrica de misión pirata ausente")?;
        let portraits = self.host.hanse_pirates.as_ref().ok_or("retratos de misión pirata ausentes")?;
        let resources = self.host.expedition_resources.as_ref().ok_or("recursos navales de misión pirata ausentes")?;
        let bands = self.c08_event_bands()?;
        if self.host.mission_pirates.is_none() || self.host.pirate_targets.is_none() {
            return Err("vistas de bandas de misión ausentes".into());
        }
        let ncv = nav.snapshot.convoys.len() / 0x3c;
        if self.economy.world.ships.len() != count
            || self.economy.cargo_ships.len() != count
            || self.host.debt_ship_metadata.len() != count
            || self.host.expedition_ship_images.len() != count
            || self.host.expedition_original_ship_images.len() != count
            || self.host.expedition_convoy_images.len() != ncv
            || self.host.debt_convoy_flags.len() != ncv
        {
            return Err("vistas navales de misión pirata incompletas".into());
        }
        let world = &self.economy.world;
        let tick = self.idle.clock.tick;
        let population = world.towns.get(usize::from(world.day_of_year & 15)).ok_or("población de capitán pirata ausente")?.u32(0x2e0);
        let port0 = resources.ports.first().copied().flatten().ok_or("puerto inicial de misión pirata ausente")?;
        let town_bytes = world.towns.iter().flat_map(|t| t.0.iter().copied()).collect::<Vec<_>>();
        let mut people = self.economy.merchants.to_vec();
        for (person, account) in people.iter_mut().zip(world.merchants.iter()).take(usize::from(world.merchant_count)) {
            person.set_i32(0, account.gold);
            person.set_i32(0x4c8, account.income);
        }
        let people_bytes = people.iter().flat_map(|p| p.bytes.iter().copied()).collect::<Vec<_>>();
        let new_band = if bands.iter().all(Option::is_none) {
            Some(town_pirate_spawn::NewBandResources {
                difficulty: self.host.pirate_difficulty,
                town_order: &self.host.expedition_town_ids,
                occupied: self.host.settlement_inland.as_ref().ok_or("ocupación de puertos de banda ausente")?,
                locations: self.host.settlement_locations.as_ref().ok_or("ubicaciones de banda ausentes")?,
                piece_bytes: self.host.town_pirate_piece_bytes.ok_or("bytes del ejecutable para banda ausentes")?,
            })
        } else {
            None
        };
        let mut fleet = town_pirate_spawn::prepare(town_pirate_spawn::Inputs {
            manager: &nav.snapshot.manager,
            ships: &nav.snapshot.ships,
            captains: &nav.snapshot.captains,
            convoys: &nav.snapshot.convoys,
            merchants: &people_bytes,
            towns: &town_bytes,
            merchant_count: world.merchant_count,
            bands: &bands,
            target,
            mode,
            resources: town_pirate_spawn::Resources {
                tick,
                difficulty: self.host.battle_difficulty,
                alderman: world.alderman_town,
                port0,
                names: &resources.ship_names,
                captain: p3_core::economy::pirate_group_callbacks::CaptainResources {
                    tick,
                    day_of_year: world.day_of_year,
                    day: world.day_of_month,
                    population,
                    first_names: resources.first_names,
                    last_names: resources.last_names,
                },
                player: u32::from(resources.player),
                new_band,
            },
        })?;
        // Resto de 0x0053FB00 sobre la flota preparada y el puente plano (variable de guion).
        // 0x0053FB00 mira el barco de referencia antes del alta: se siembra la flota previa y el alta escribe la nueva.
        let mut flat = self.flat_seed(op.0)?;
        flat.mem.seed(MANAGER, &vec![0; fleet.manager.len()]);
        flat.mem.seed(SHIPS, &vec![0; fleet.ships.len()]);
        flat.mem.seed(CAPTAINS, &vec![0; fleet.captains.len()]);
        seed_fleet(&mut flat.mem, &nav.snapshot.manager, &nav.snapshot.ships, &nav.snapshot.captains);
        let shared = SharedMem(std::rc::Rc::new(std::cell::RefCell::new(std::mem::take(&mut flat.mem))));
        let after = (fleet.manager.clone(), fleet.ships.clone(), fleet.captains.clone());
        let mut host = Host { mem: shared.clone(), ship: fleet.ship, expected: (target, u32::from(mode)), after, error: None };
        let result = {
            let mut mem = shared.clone();
            guarded(|| p3_core::economy::pirate_ops::op_spawn_mission_pirate_ship(&mut mem, &mut host, OP))
        };
        let error = host.error.take();
        drop(host);
        let mut mem: SparseMem = std::rc::Rc::try_unwrap(shared.0).map_err(|_| "memoria del puente aún compartida")?.into_inner();
        result?;
        if let Some(e) = error {
            return Err(e);
        }
        fleet.ships = mem.read(SHIPS, fleet.ships.len());
        fleet.captains = mem.read(CAPTAINS, fleet.captains.len());
        if let Some(e) = flat.calls.error.take() {
            return Err(e);
        }
        flat.mem = mem;
        let staged = town_event_live::prepare(nav, factory, portraits, &fleet)?;
        self.flat_publish(flat)?;
        self.c08_publish_pirate_fleet(staged, fleet.bands);
        Ok(())
    }

    /// Publicación del alta (misma que la del suceso urbano 0x1E en el despacho de tareas).
    fn c08_publish_pirate_fleet(&mut self, staged: town_event_live::Prepared, bands: Vec<Option<p3_save::pirate_bands::PirateBand>>) {
        let before = self.host.navigation.as_ref().expect("validado").snapshot.ships.len() / 0x180;
        let after = &staged.navigation.snapshot;
        let count = after.ships.len() / 0x180;
        self.economy.world.ships.resize_with(count, Ship::default);
        self.economy.cargo_ships.resize_with(count, Default::default);
        self.host.debt_ship_metadata.resize(count, (0xffff, 0xff, 0xffff, 0xffff));
        self.host.expedition_ship_images.resize_with(count, || None);
        self.host.expedition_original_ship_images.resize_with(count, || None);
        let old = &self.host.navigation.as_ref().expect("validado").snapshot.ships;
        let activated = (0..count).filter(|&i| u16_at(&after.ships, i * 0x180 + 0x134) < 0x17 && (i >= before || u16_at(old, i * 0x180 + 0x134) >= 0x17)).collect::<Vec<_>>();
        for i in (before..count).chain(activated) {
            let image = &after.ships[i * 0x180..(i + 1) * 0x180];
            self.economy.world.ships[i].owner = image[0];
            self.economy.world.ships[i].state = u16_at(image, 0x134);
            self.host.debt_ship_metadata[i] = (u16_at(image, 4), u16_at(image, 0x134), u16_at(image, 0x15e), u16_at(image, 8));
            if u16_at(image, 0x134) < 0x17 {
                self.host.expedition_ship_images[i] = Some(image.to_vec())
            }
            let cargo = &mut self.economy.cargo_ships[i];
            cargo.owner = image[0];
            cargo.town = image[0x39];
            cargo.kind = image[0xe];
            cargo.capacity = u32_at(image, 0x10) as i32;
            cargo.dirty = u16_at(image, 0x3c);
            cargo.crew = u16_at(image, 0x40);
            cargo.cutlasses = u16_at(image, 0x154);
            cargo.marines = image[0x156];
            for k in 0..24 {
                cargo.stock[k] = u32_at(image, 0x54 + 4 * k) as i32;
                cargo.average_price[k] = f32::from_bits(u32_at(image, 0xb4 + 4 * k));
                cargo.guns[k] = image[0x13c + k]
            }
            cargo.cargo_value = u32_at(image, 0x114) as i32;
            cargo.cargo_weight = u32_at(image, 0x118) as i32;
            cargo.equipment_weight = u32_at(image, 0x11c) as i32;
            cargo.gun_power = u32_at(image, 0x120) as i32;
        }
        self.economy.world.ship_count = count as u16;
        let old_cv = self.host.expedition_convoy_images.len();
        self.host.expedition_convoy_images.resize_with(after.convoys.len() / 0x3c, || vec![0; 0x3c]);
        self.host.debt_convoy_flags.resize(after.convoys.len() / 0x3c, 0);
        for i in old_cv..self.host.expedition_convoy_images.len() {
            let image = &after.convoys[i * 0x3c..(i + 1) * 0x3c];
            self.host.expedition_convoy_images[i].copy_from_slice(image);
            self.host.debt_convoy_flags[i] = u32_at(image, 0x18)
        }
        for (i, band) in bands.iter().enumerate() {
            if let Some(band) = band {
                self.host.mission_pirates.as_mut().expect("validado")[i] = Some(p3_core::ai::settlement::MissionPirateBand { first_ship: u16_at(&band.header, 4), target: band.header[6], status: band.header[12], reserve_strength: u16_at(&band.header, 14) });
                self.host.pirate_targets.as_mut().expect("validado")[i] = Some(band.header[6]);
            }
        }
        self.host.pirate_band_images = Some(bands);
        self.host.hanse_pirates = Some(staged.portraits);
        self.host.expedition_factory = Some(staged.factory);
        self.host.navigation = Some(staged.navigation);
    }
}
