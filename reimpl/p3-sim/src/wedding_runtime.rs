//! Transacción de la boda: cartas, fábrica y vistas del reloj se preparan juntos.
use crate::{u16_at, u32_at, wedding_delivery, Calendar, CouncilExecution, RepMerchant, Ship, ShipCargo};
use p3_core::{
    scheduler::{Scheduler, TaskRecord},
    ships::factory::HanseFactory,
};

pub(crate) struct Prepared {
    task: TaskRecord,
    factory: HanseFactory,
    docked: u16,
    merchants: Vec<RepMerchant>,
    pool: Vec<u8>,
    navigation: Option<p3_save::ships::NavigationSnapshot>,
    effects: wedding_delivery::WeddingEffects,
}

impl CouncilExecution<'_> {
    /// La consulta 5308A0 recorre la cadena urbana; tener una oficina en otra
    /// ciudad no basta para abrir la ventana de boda.
    fn wedding_local_office(&self, city: u8) -> Result<bool, String> {
        if self.world.local_merchant >= u32::from(self.world.merchant_count) {
            return Ok(false);
        }
        let Some(town) = self.world.towns.get(usize::from(city)) else {
            return Ok(false);
        };
        let mut cursor = usize::from(town.u16(0x784));
        let mut seen = vec![false; self.world.offices.len()];
        while cursor < seen.len() {
            if seen[cursor] {
                return Err("cadena de oficinas de boda cíclica".into());
            }
            seen[cursor] = true;
            let office = self
                .stocks
                .get(cursor)
                .ok_or("propietario de oficina de boda ausente")?;
            if u32::from(office.owner) == self.world.local_merchant {
                return Ok(true);
            }
            cursor = usize::from(self.world.offices[cursor].next);
        }
        Ok(false)
    }

    pub(crate) fn wedding_preview(&self, s: &Scheduler) -> Result<Option<Prepared>, String> {
        let mut task = *s.records.get(usize::from(s.head)).ok_or("tarea de boda ausente")?;
        let count = usize::from(self.world.merchant_count);
        let people = self.merchants.get(..count).ok_or("comerciantes de boda incompletos")?;
        if p3_core::economy::matchmaker::wedding_is_inactive(&task, self.tick, people)? {
            return Ok(None);
        }
        let owner = usize::from(task.u8_at(0x15));
        let home = people[owner].bytes[0x19];
        let config = self
            .expedition_resources
            .as_ref()
            .ok_or("recursos navales de boda ausentes")?;
        let city = usize::from(task.u8_at(0x12));
        config
            .ports
            .get(city)
            .copied()
            .flatten()
            .ok_or("puerto de boda ausente")?;
        let ports = config.ports.iter().map(|p| p.unwrap_or((0, 0))).collect::<Vec<_>>();
        let mut factory = self.expedition_factory.clone().ok_or("fábrica naval de boda ausente")?;
        if factory.ships.len() != self.expedition_images.len()
            || factory.ships.len() != self.fleet.len()
            || factory.ships.len() != usize::from(self.world.ship_count)
        {
            return Err("vistas navales de boda incompletas".into());
        }
        let mut docked = *self.expedition_docked;
        if let Some(nav) = self.navigation.as_ref() {
            if nav.snapshot.manager.len() < 0xf6
                || nav.snapshot.ships.len() != factory.ships.len() * 0x180
                || nav.snapshot.routes.len() != factory.ships.len()
            {
                return Err("navegación de boda incompleta".into());
            }
            for (dst, src) in factory.ships.iter_mut().zip(nav.snapshot.ships.chunks_exact(0x180)) {
                dst.copy_from_slice(src)
            }
            factory.free_ship = u16_at(&nav.snapshot.manager, 0xe2);
            factory.name_cursor = u16_at(&nav.snapshot.manager, 0xee);
            factory.name_step = u16_at(&nav.snapshot.manager, 0xf0);
            docked = u16_at(&nav.snapshot.manager, 0xe8);
        } else {
            for (dst, src) in factory.ships.iter_mut().zip(self.expedition_images.iter()) {
                if let Some(src) = src {
                    if src.len() != 0x180 {
                        return Err("barco de boda truncado".into());
                    }
                    dst.copy_from_slice(src)
                }
            }
        }
        let mut merchants = self.merchants.to_vec();
        let resources = wedding_delivery::WeddingResources {
            tick: self.tick,
            operations_mode: self.operations_mode,
            ship_names: &config.ship_names,
            ports: &ports,
            local_player: self.world.local_merchant,
            local_controls_home: if self.world.local_merchant == owner as u32 {
                false
            } else {
                self.wedding_local_office(home)?
            },
        };
        let effects = wedding_delivery::deliver(
            &mut task,
            &mut factory,
            &mut docked,
            &mut merchants[..count],
            &self.world.towns,
            &resources,
        )?
        .ok_or("boda activa no entregada")?;
        let date = Calendar::for_tick(self.tick);
        let mut pool = self.pool.clone();
        for (recipient, letter) in &effects.letters {
            if *recipient == -2 {
                p3_state::messages::add_global_inline_letter(&mut pool, date.day, date.month, date.year, letter)?
            } else {
                let person = merchants
                    .get_mut(*recipient as usize)
                    .ok_or("destinatario de boda ausente")?;
                p3_state::messages::add_wedding_gifts(&mut pool, person, date.day, date.month, date.year, letter)?;
            }
        }
        let navigation = if let Some(nav) = self.navigation.as_ref() {
            let mut snapshot = nav.snapshot.clone();
            snapshot.ships = factory.ships.iter().flatten().copied().collect();
            snapshot.routes.resize(factory.ships.len(), None);
            for &id in &effects.delivered {
                snapshot.routes[usize::from(id)] = Some(p3_save::ships::SavedRoute::None)
            }
            for (at, value) in [
                (0xe2, factory.free_ship),
                (0xe8, docked),
                (0xee, factory.name_cursor),
                (0xf4, factory.ships.len() as u16),
            ] {
                snapshot.manager[at..at + 2].copy_from_slice(&value.to_le_bytes())
            }
            snapshot.lists.docked_head = docked;
            snapshot.lists.docked.clear();
            let mut cursor = usize::from(docked);
            let mut seen = vec![false; factory.ships.len()];
            while cursor < seen.len() {
                if seen[cursor] {
                    return Err("lista de atracados de boda cíclica".into());
                }
                seen[cursor] = true;
                snapshot.lists.docked.push(cursor as u16);
                cursor = usize::from(u16_at(&factory.ships[cursor], 6))
            }
            Some(snapshot)
        } else {
            None
        };
        Ok(Some(Prepared {
            task,
            factory,
            docked,
            merchants,
            pool,
            navigation,
            effects,
        }))
    }

    pub(crate) fn apply_wedding(&mut self, s: &mut Scheduler, p: Prepared) {
        let count = p.factory.ships.len();
        self.fleet.resize_with(count, || ShipCargo {
            owner: 0xff,
            ..Default::default()
        });
        self.world.ships.resize(
            count,
            Ship {
                owner: 0xff,
                state: 0xff,
            },
        );
        self.ship_metadata.resize(count, (u16::MAX, 0xff, 0, u16::MAX));
        self.expedition_images.resize(count, None);
        self.expedition_original_images.resize(count, None);
        self.world.ship_count = count as u16;
        for &id in &p.effects.delivered {
            let i = usize::from(id);
            let image = &p.factory.ships[i];
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
            self.world.ships[i] = Ship {
                owner: image[0],
                state: u16_at(image, 0x134),
            };
            self.ship_metadata[i] = (
                u16_at(image, 4),
                u16_at(image, 0x134),
                u16_at(image, 0x15e),
                u16_at(image, 8),
            );
            self.expedition_images[i] = Some(image.clone());
            self.expedition_original_images[i] = Some(image.clone());
            if !self.expedition_changed_ships.contains(&i) {
                self.expedition_changed_ships.push(i)
            }
        }
        if let Some(pirates) = self.pirates.as_mut() {
            pirates.next = self.ship_metadata.iter().map(|m| m.0).collect();
            pirates.ship_captains = self
                .expedition_images
                .iter()
                .map(|s| s.as_ref().map_or(u16::MAX, |v| u16_at(v, 0x42)))
                .collect();
        }
        if let Some(snapshot) = p.navigation {
            let nav=self.navigation.as_mut().expect("validado");
            let prior=nav.baseline.ships.len()/0x180;
            // S02 publica las altas de la fábrica antes de los deltas de viaje.
            for &id in &p.effects.delivered {
                let i=usize::from(id);if i<prior {
                    nav.baseline.ships[i*0x180..(i+1)*0x180].copy_from_slice(&p.factory.ships[i]);
                    nav.baseline.routes[i]=Some(p3_save::ships::SavedRoute::None);
                }
            }
            for image in p.factory.ships.iter().skip(prior) {
                nav.baseline.ships.extend_from_slice(image);
                nav.baseline.routes.push(if u16_at(image,0x134)<0x17 {Some(p3_save::ships::SavedRoute::None)}else{None});
            }
            nav.baseline.manager[0xf4..0xf6].copy_from_slice(&(count as u16).to_le_bytes());
            nav.snapshot = snapshot
        }
        self.wedding_notifications.push(p.effects.notice);
        if p.effects.show_window {
            self.wedding_windows
                .push((p.effects.notice[0] as u16, p.effects.notice[3] as u8))
        }
        self.merchants.clone_from_slice(&p.merchants);
        *self.pool = p.pool;
        *self.expedition_docked = p.docked;
        *self.expedition_factory = Some(p.factory);
        s.records[usize::from(s.head)] = p.task;
    }
}
