//! Publicación de los barcos que crea un guion de misión (`0x67` → `0x0050E000`) sobre la imagen plana.
//!
//! El guion trabaja sobre la tabla naval sembrada desde la flota de navegación; al terminar, la tabla entera y
//! los cuatro campos del gestor que mueve la fábrica (`+0xE2` libre, `+0xE8` atracados, `+0xEE` nombre,
//! `+0xF4` cuenta) se publican en todas las vistas navales a la vez, como la boda (`wedding_runtime`): fábrica,
//! carga, metadatos, imágenes de expedición, piratas y navegación (base y estado).
use crate::c10_script_flat::CreatedShips;
use crate::{u16_at, u32_at, CouncilExecution, Ship, ShipCargo};

impl CouncilExecution<'_> {
    /// Comprueba que las vistas navales permiten publicar las altas (sin modificar nada).
    pub(crate) fn check_created_ships(&self, c: &CreatedShips) -> Result<(), String> {
        let factory = self.expedition_factory.as_ref().ok_or("fábrica naval ausente para los barcos del guion")?;
        let nav = self.navigation.as_ref().ok_or("navegación ausente para los barcos del guion")?;
        let before = factory.ships.len();
        if before != self.expedition_images.len() || before != self.fleet.len() || before != usize::from(self.world.ship_count) {
            return Err("vistas navales del guion incompletas".into());
        }
        if nav.snapshot.ships.len() != before * 0x180 || nav.snapshot.routes.len() != before || nav.snapshot.manager.len() < 0xf6 {
            return Err("navegación del guion incompleta".into());
        }
        if c.ships.len() % 0x180 != 0 || c.ships.len() < nav.snapshot.ships.len() || c.manager.len() < 0xf6 {
            return Err("tabla naval del guion truncada".into());
        }
        if c.ids.iter().any(|&id| usize::from(id) * 0x180 >= c.ships.len()) {
            return Err("barco del guion fuera de la tabla".into());
        }
        let count = c.ships.len() / 0x180;
        let mut cursor = usize::from(u16_at(&c.manager, 0xe8));
        let mut seen = vec![false; count];
        while cursor < count {
            if seen[cursor] {
                return Err("lista de atracados del guion cíclica".into());
            }
            seen[cursor] = true;
            cursor = usize::from(u16_at(&c.ships[cursor * 0x180..], 6));
        }
        Ok(())
    }

    pub(crate) fn apply_created_ships(&mut self, c: CreatedShips) {
        let count = c.ships.len() / 0x180;
        self.fleet.resize_with(count, || ShipCargo { owner: 0xff, ..Default::default() });
        self.world.ships.resize(count, Ship { owner: 0xff, state: 0xff });
        self.ship_metadata.resize(count, (u16::MAX, 0xff, 0, u16::MAX));
        self.expedition_images.resize(count, None);
        self.expedition_original_images.resize(count, None);
        self.world.ship_count = count as u16;
        let images: Vec<Vec<u8>> = c.ships.chunks_exact(0x180).map(<[u8]>::to_vec).collect();
        for &id in &c.ids {
            let i = usize::from(id);
            let image = &images[i];
            let mut cargo = ShipCargo {
                owner: image[0],
                town: image[0x39],
                kind: image[0xe],
                crew: u16_at(image, 0x40),
                marines: image[0x156],
                cutlasses: u16_at(image, 0x154),
                capacity: u32_at(image, 0x10) as i32,
                dirty: u16_at(image, 0x3c),
                cargo_value: u32_at(image, 0x114) as i32,
                cargo_weight: u32_at(image, 0x118) as i32,
                equipment_weight: u32_at(image, 0x11c) as i32,
                gun_power: u32_at(image, 0x120) as i32,
                ..Default::default()
            };
            for w in 0..24 {
                cargo.stock[w] = u32_at(image, 0x54 + 4 * w) as i32;
                cargo.average_price[w] = f32::from_bits(u32_at(image, 0xb4 + 4 * w));
                cargo.guns[w] = image[0x13c + w]
            }
            self.fleet[i] = cargo;
            self.world.ships[i] = Ship { owner: image[0], state: u16_at(image, 0x134) };
            self.ship_metadata[i] = (u16_at(image, 4), u16_at(image, 0x134), u16_at(image, 0x15e), u16_at(image, 8));
            self.expedition_images[i] = Some(image.clone());
            self.expedition_original_images[i] = Some(image.clone());
            if !self.expedition_changed_ships.contains(&i) {
                self.expedition_changed_ships.push(i)
            }
        }
        if let Some(pirates) = self.pirates.as_mut() {
            pirates.next = self.ship_metadata.iter().map(|m| m.0).collect();
            pirates.ship_captains = self.expedition_images.iter().map(|s| s.as_ref().map_or(u16::MAX, |v| u16_at(v, 0x42))).collect();
        }
        let docked = u16_at(&c.manager, 0xe8);
        let nav = self.navigation.as_mut().expect("validado");
        let prior = nav.baseline.ships.len() / 0x180;
        // Como en S02, las altas de la fábrica entran en la base antes de los deltas de viaje.
        for &id in &c.ids {
            let i = usize::from(id);
            if i < prior {
                nav.baseline.ships[i * 0x180..(i + 1) * 0x180].copy_from_slice(&images[i]);
                nav.baseline.routes[i] = Some(p3_save::ships::SavedRoute::None);
            }
        }
        for image in images.iter().skip(prior) {
            nav.baseline.ships.extend_from_slice(image);
            nav.baseline.routes.push(if u16_at(image, 0x134) < 0x17 { Some(p3_save::ships::SavedRoute::None) } else { None });
        }
        nav.baseline.manager[0xf4..0xf6].copy_from_slice(&(count as u16).to_le_bytes());
        let snapshot = &mut nav.snapshot;
        snapshot.ships = c.ships.clone();
        snapshot.routes.resize(count, None);
        for &id in &c.ids {
            snapshot.routes[usize::from(id)] = Some(p3_save::ships::SavedRoute::None)
        }
        for at in [0xe2, 0xe8, 0xee, 0xf4] {
            snapshot.manager[at..at + 2].copy_from_slice(&c.manager[at..at + 2]);
        }
        snapshot.lists.docked_head = docked;
        snapshot.lists.docked.clear();
        let mut cursor = usize::from(docked);
        let mut seen = vec![false; count];
        while cursor < count && !seen[cursor] {
            seen[cursor] = true;
            snapshot.lists.docked.push(cursor as u16);
            cursor = usize::from(u16_at(&images[cursor], 6))
        }
        *self.expedition_docked = docked;
        let factory = self.expedition_factory.as_mut().expect("validado");
        factory.ships = images;
        factory.free_ship = u16_at(&c.manager, 0xe2);
        factory.name_cursor = u16_at(&c.manager, 0xee);
    }
}
