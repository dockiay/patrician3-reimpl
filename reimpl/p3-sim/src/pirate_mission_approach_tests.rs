use super::DailyClock;
use crate::{navigation::LiveVoyage, HanseFleetResources, Sim};
use p3_core::{ai::settlement::SettlementLocation, tick::TickSystems};
use p3_save::{
    model::SaveModel,
    pirate_bands::{self, PirateBand},
    ships::Ships,
};

#[test]
fn scheduler_replenishes_band_and_roundtrips_s05() {
    let Ok(root) = std::env::var("P3_SAVES") else { return };
    let bytes = std::fs::read(format!("{root}/visibility/save/Ein/AUTO.pat")).unwrap();
    for force_growth in [false, true] {
        let mut model = SaveModel::parse(&bytes).unwrap();
        let original = pirate_bands::read(&model.ships.tail).unwrap();
        for i in 0..5 {
            let replacement = if i < 2 {
                let mut h = [0u8; 16];
                h[4..6].fill(255);
                h[6] = if i == 0 { 2 } else { 3 };
                h[8..12].fill(255);
                h[12] = 3;
                Some(PirateBand {
                    header: h,
                    plan: None,
                    pieces: vec![],
                })
            } else {
                None
            };
            pirate_bands::replace_optional(&mut model.ships.tail, i, original[i].as_ref(), replacement.as_ref())
                .unwrap();
        }
        if force_growth {
            let source = Sim::from_save_bytes(&model.write()).unwrap();
            let snapshot = Ships::parse(&model.ships.write())
                .unwrap()
                .navigation_snapshot()
                .unwrap();
            let mut before = source.host.expedition_factory.clone().unwrap();
            before.ships = snapshot.ships.chunks_exact(0x180).map(|v| v.to_vec()).collect();
            before.captains = snapshot
                .captains
                .chunks_exact(16)
                .map(|v| v.try_into().unwrap())
                .collect();
            before.convoys = snapshot.convoys.chunks_exact(0x3c).map(|v| v.to_vec()).collect();
            let mut after = before.clone();
            after.free_ship = u16::try_from(before.ships.len()).unwrap();
            after.free_captain = u16::try_from(before.captains.len()).unwrap();
            model
                .ships
                .apply_hanse_factory_navigation(
                    &before,
                    &after,
                    &snapshot.routes,
                    &snapshot.routes,
                    model.world.towns.len(),
                )
                .unwrap();
        }
        let mut sim = Sim::from_save_bytes(&model.write()).unwrap();
        let snapshot = Ships::parse(&model.ships.write())
            .unwrap()
            .navigation_snapshot()
            .unwrap();
        let (owner, ship) = sim
            .rep_merchants
            .iter()
            .enumerate()
            .find_map(|(owner, m)| {
                let id = usize::from(m.u16(0xe));
                let at = id.checked_mul(0x180)?;
                let image = snapshot.ships.get(at..at + 0x180)?;
                let state = u16::from_le_bytes([image[0x134], image[0x135]]);
                (state < 0x17).then_some((owner, id))
            })
            .expect("la partida necesita un barco de comerciante");
        let image = &snapshot.ships[ship * 0x180..(ship + 1) * 0x180];
        let x = i16::from_le_bytes([image[0x1e], image[0x1f]]);
        let y = i16::from_le_bytes([image[0x22], image[0x23]]);
        let mut locations = [SettlementLocation { x: 0, y: 0, kind: 0 }; 40];
        locations[2].x = i32::from(x);
        locations[2].y = i32::from(y);
        let ports = vec![(100i16, 500i16); sim.economy.world.towns.len()];
        sim.host.navigation = Some(LiveVoyage::idle(snapshot, ports.clone(), owner as u32));
        sim.host.settlement_locations = Some(locations);
        sim.host.foundation_names = Some((75, 80));
        sim.host.family_names = Some(vec![vec![]; 21]);
        sim.host.expedition_resources = Some(HanseFleetResources {
            ship_names: vec![b"Alpha".to_vec(), b"Brig".to_vec(), b"Crown".to_vec()],
            first_names: 75,
            last_names: 80,
            ports: ports.iter().map(|&(a, b)| Some((a as u16, b as u16))).collect(),
            hotseat: false,
            player: owner as u16,
        });
        sim.economy.world.local_merchant = owner as u32;
        sim.host.pirate_difficulty = 1;
        let mut limits = [0u8; 256];
        limits[1] = 2;
        sim.host.pirate_group_thresholds = Some(limits);
        let tick = sim.idle.clock.tick;
        let mut scheduler = sim.host.council_scheduler.clone();
        let mut at = scheduler.head;
        while at < scheduler.capacity {
            let r = &mut scheduler.records[at as usize];
            r.due = tick + 0x100000;
            at = r.next
        }
        let id = scheduler.add_task(tick);
        let task = &mut scheduler.records[id as usize];
        task.kind = 0x32;
        task.payload = [0; 16];
        task.set_u32_at(8, tick + 0x10000);
        task.set_u8_at(0xc, 3);
        task.set_u8_at(0xd, owner as u8);
        task.set_u8_at(0xf, 1);
        task.set_u8_at(0x10, 0);
        task.set_u8_at(0x11, 2);
        sim.host.council_scheduler = scheduler;
        let task = sim.host.council_scheduler.records[id as usize];
        let before_nav = sim.host.navigation.as_ref().unwrap().snapshot.clone();
        let before_bands = sim.host.pirate_band_images.clone();
        sim.host.pirate_group_thresholds = None;
        assert!(sim
            .apply_pirate_mission_approach(id, task)
            .unwrap_err()
            .contains("tabla original"));
        assert_eq!(sim.host.navigation.as_ref().unwrap().snapshot, before_nav);
        assert_eq!(sim.host.pirate_band_images, before_bands);
        assert_eq!(sim.host.council_scheduler.records[id as usize], task);
        sim.host.pirate_group_thresholds = Some(limits);
        let old_targets = sim.host.pirate_targets;
        sim.host.pirate_targets.as_mut().unwrap()[1] = Some(9);
        assert!(sim
            .apply_pirate_mission_approach(id, task)
            .unwrap_err()
            .contains("banda"));
        assert_eq!(sim.host.navigation.as_ref().unwrap().snapshot, before_nav);
        sim.host.pirate_targets = old_targets;
        let old_count = before_nav.manager[0xf4..0xf6].to_vec();
        sim.host.navigation.as_mut().unwrap().snapshot.manager[0xf4..0xf6].copy_from_slice(&0u16.to_le_bytes());
        assert!(sim
            .apply_pirate_mission_approach(id, task)
            .unwrap_err()
            .contains("contadores"));
        sim.host.navigation.as_mut().unwrap().snapshot.manager[0xf4..0xf6].copy_from_slice(&old_count);
        let free = sim.host.expedition_factory.as_ref().unwrap().free_ship;
        sim.host.expedition_factory.as_mut().unwrap().free_ship ^= 1;
        assert!(sim
            .apply_pirate_mission_approach(id, task)
            .unwrap_err()
            .contains("asignador"));
        assert_eq!(sim.host.navigation.as_ref().unwrap().snapshot, before_nav);
        sim.host.expedition_factory.as_mut().unwrap().free_ship = free;
        let old_band = sim.host.pirate_band_images.as_ref().unwrap()[0].clone();
        sim.host.pirate_band_images.as_mut().unwrap()[0]
            .as_mut()
            .unwrap()
            .header[10..12]
            .copy_from_slice(&(ship as u16).to_le_bytes());
        let old_next = sim.host.expedition_factory.as_ref().unwrap().ships[ship][6];
        sim.host.expedition_factory.as_mut().unwrap().ships[ship][6] ^= 1;
        assert!(sim
            .apply_pirate_mission_approach(id, task)
            .unwrap_err()
            .contains("reserva pirata"));
        sim.host.expedition_factory.as_mut().unwrap().ships[ship][6] = old_next;
        sim.host.pirate_band_images.as_mut().unwrap()[0] = old_band;
        sim.economy
            .world
            .ships
            .push(p3_core::economy::world::Ship { owner: 255, state: 255 });
        assert!(sim
            .apply_pirate_mission_approach(id, task)
            .unwrap_err()
            .contains("incompletas"));
        sim.economy.world.ships.pop();
        let before_image = sim.host.expedition_ship_images[ship].clone();
        let image = sim.host.expedition_ship_images[ship].as_mut().expect("barco vivo");
        image[0x1e] ^= 1;
        assert!(sim
            .apply_pirate_mission_approach(id, task)
            .unwrap_err()
            .contains("desincronizadas"));
        assert_eq!(sim.host.navigation.as_ref().unwrap().snapshot, before_nav);
        assert_eq!(sim.host.pirate_band_images, before_bands);
        sim.host.expedition_ship_images[ship] = before_image;
        // El original no recorre reservas de una banda con otro destino.
        let unrelated = sim.host.pirate_band_images.as_ref().unwrap()[1].clone();
        sim.host.pirate_band_images.as_mut().unwrap()[1]
            .as_mut()
            .unwrap()
            .header[10..12]
            .copy_from_slice(&(ship as u16).to_le_bytes());
        let unrelated_next = sim.host.expedition_factory.as_ref().unwrap().ships[ship][6];
        sim.host.expedition_factory.as_mut().unwrap().ships[ship][6] ^= 1;
        {
            let mut clock = DailyClock {
                sim: &mut sim,
                world_ready: false,
                error: None,
            };
            clock.scheduled_tasks_tick(tick);
            assert!(clock.error.is_none(), "{:?}", clock.error)
        }
        if force_growth {
            let snapshot = &sim.host.navigation.as_ref().unwrap().snapshot;
            assert_eq!(snapshot.ships.len(), before_nav.ships.len() + 64 * 0x180);
            assert_eq!(snapshot.captains.len(), before_nav.captains.len() + 64 * 16);
        }
        sim.host.expedition_factory.as_mut().unwrap().ships[ship][6] = unrelated_next;
        sim.host.pirate_band_images.as_mut().unwrap()[1] = unrelated;
        assert!(sim.host.pending_pirate_approach.is_none());
        let task = sim.host.council_scheduler.records[id as usize];
        assert_eq!(task.u8_at(0x12), 1);
        assert!(task.due > tick && task.due <= tick + 0x40);
        assert_eq!(sim.rep_merchants[owner].bytes[0xe1], 2);
        let bands = sim.host.pirate_band_images.as_ref().unwrap();
        let band = bands[0].as_ref().unwrap();
        let mut next = u16::from_le_bytes([band.header[10], band.header[11]]);
        let mut reserve = Vec::new();
        let ships = &sim.host.navigation.as_ref().unwrap().snapshot.ships;
        while usize::from(next) < ships.len() / 0x180 {
            assert!(!reserve.contains(&next));
            reserve.push(next);
            let pos = usize::from(next) * 0x180;
            next = u16::from_le_bytes([ships[pos + 6], ships[pos + 7]])
        }
        assert_eq!(reserve.len(), 2);
        let mut nonzero = false;
        for &i in &reserve {
            let pos = usize::from(i) * 0x180;
            let captain = u16::from_le_bytes([ships[pos + 0x42], ships[pos + 0x43]]);
            let at = usize::from(captain) * 16;
            let actual = sim.host.navigation.as_ref().unwrap().snapshot.captains[at + 3];
            let date = sim.idle.clock.calendar;
            let wanted =
                ((u32::from(date.day) + (tick & 255) + (u32::from(captain) >> 1) + u32::from(date.day_of_year)) % 80)
                    as u8;
            assert_eq!(actual, wanted);
            nonzero |= actual != 0;
        }
        assert!(nonzero, "el test debe distinguir D4=80 de D6=1");
        sim.persist_all(&mut model).unwrap();
        let saved = model.write();
        let loaded = Sim::from_save_bytes(&saved).unwrap();
        assert_eq!(loaded.host.council_scheduler.records[id as usize], task);
        assert_eq!(loaded.rep_merchants[owner].bytes[0xe1], 2);
        let loaded_bands = loaded.host.pirate_band_images.as_ref().unwrap();
        assert_eq!(loaded_bands[0].as_ref().unwrap().header, band.header);
        for &i in &reserve {
            assert_eq!(
                loaded.host.expedition_factory.as_ref().unwrap().ships[usize::from(i)][0x134..0x136],
                0x12u16.to_le_bytes()
            )
        }
    }
}

#[test]
fn pirate_captains_use_d4_not_the_loan_name_count() {
    let mut host = crate::NullHost::default();
    host.foundation_names = Some((75, 80));
    host.family_names = Some(vec![vec![]; 21]);
    assert_eq!(host.pirate_captain_name_counts(), Some((75, 80)));
    assert_eq!(host.loan_applicant_name_counts(), Some((75, 1)));
}
