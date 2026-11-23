//! Selecciona oficinas siguiendo las cadenas urbanas y prepara 0x0C.
use p3_core::{
    ai::reputation::Merchant,
    calendar::Calendar,
    economy::{land_transport::Env, trade::OfficeStock, world::Office},
    scheduler::TaskRecord,
    town::Town,
};
const ROUTES: [[u8; 4]; 7] = [
    [3, 4, 0x50, 0x20],
    [9, 10, 0x82, 0x34],
    [11, 13, 0x1e, 0xc],
    [12, 14, 0x28, 0x10],
    [28, 31, 0x78, 0x30],
    [36, 37, 0x3c, 0x18],
    [38, 39, 0x5a, 0x24],
];
const UNITS: [u8; 24] = [0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1];
pub struct Inputs<'a> {
    pub task: &'a TaskRecord,
    pub towns: &'a [Town],
    pub offices: &'a [Office],
    pub stocks: &'a [OfficeStock],
    pub merchants: &'a [Merchant],
    pub town_indices: &'a [u8],
    pub message_pool: &'a [u8],
    pub calendar: Calendar,
    pub scene_active: bool,
}
pub struct Prepared {
    pub task: TaskRecord,
    pub stocks: Vec<OfficeStock>,
    pub merchants: Vec<Merchant>,
    pub message_pool: Vec<u8>,
    pub notifications: Vec<[u32; 6]>,
    pub keep: bool,
}
/// No publica mutaciones si una lista está dañada o falla una carta.
pub fn prepare(i: Inputs<'_>) -> Result<Prepared, String> {
    if i.offices.len() != i.stocks.len() {
        return Err("pool de oficinas terrestres desincronizado".into());
    }
    let mut ordered = Vec::new();
    let mut ids = Vec::new();
    let mut seen = vec![false; i.offices.len()];
    let mut cities = [usize::from(i.task.u8_at(8)), usize::from(i.task.u8_at(9))];
    cities.sort_unstable();
    for (n, &city) in cities.iter().enumerate() {
        if n == 1 && city == cities[0] {
            continue;
        }
        let Some(town) = i.towns.get(city) else { continue };
        let mut cursor = usize::from(town.u16(0x784));
        while cursor < i.offices.len() {
            if seen[cursor] {
                return Err("cadena de oficinas terrestres cíclica o compartida".into());
            }
            seen[cursor] = true;
            let stock = &i.stocks[cursor];
            if usize::from(stock.town) != city {
                return Err("oficina terrestre pertenece a otra ciudad".into());
            }
            ordered.push(stock.clone());
            ids.push(cursor);
            cursor = usize::from(i.offices[cursor].next)
        }
    }
    let env = Env {
        towns: i.towns,
        offices: &ordered,
        merchants: i.merchants,
        town_indices: i.town_indices,
        routes: &ROUTES,
        units: &UNITS,
        scene_active: i.scene_active,
    };
    let prepared = p3_state::land_transport::prepare(i.task, &env, i.message_pool, i.calendar)?;
    let mut stocks = i.stocks.to_vec();
    for (index, stock) in ids.into_iter().zip(prepared.core.offices) {
        stocks[index] = stock
    }
    Ok(Prepared {
        task: prepared.core.task,
        stocks,
        merchants: prepared.core.merchants,
        message_pool: prepared.message_pool,
        notifications: prepared.notifications,
        keep: prepared.core.keep,
    })
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::Sim;
    use p3_core::economy::world::TownHost;
    use std::io::Write;
    fn hex(b: &[u8]) -> String {
        b.iter().map(|v| format!("{v:02x}")).collect()
    }
    fn unhex(s: &str) -> Vec<u8> {
        s.as_bytes()
            .chunks_exact(2)
            .map(|v| u8::from_str_radix(std::str::from_utf8(v).unwrap(), 16).unwrap())
            .collect()
    }
    #[test]
    fn scheduler_office_lookup_letters_and_save_match_x86() {
        let (Ok(root), Ok(exe), Ok(py)) = (
            std::env::var("P3_SAVES"),
            std::env::var("P3_EXE"),
            std::env::var("P3_ORACLE_PY"),
        ) else {
            eprintln!("SKIPPED: P3_SAVES, P3_EXE y P3_ORACLE_PY requeridos");
            return;
        };
        let raw = std::fs::read(format!("{root}/visibility/save/Ein/AUTO.pat")).unwrap();
        for phase in [0u8, 7] {
            let mut model = p3_save::model::SaveModel::parse(&raw).unwrap();
            let probe = Sim::from_save_bytes(&raw).unwrap();
            let tick = probe.idle.clock.tick;
            let mut candidates = Vec::new();
            for (city, town) in probe.economy.world.towns.iter().enumerate() {
                let head = usize::from(town.u16(0x784));
                let mut cursor = head;
                let mut seen = vec![false; probe.economy.world.offices.len()];
                while cursor < probe.economy.world.offices.len() {
                    if seen[cursor] {
                        break;
                    }
                    seen[cursor] = true;
                    let stock = &probe.economy.offices[cursor];
                    if usize::from(stock.town) == city
                        && stock.owner < 256
                        && probe.rep_merchants.get(usize::from(stock.owner)).is_some()
                    {
                        candidates.push((usize::from(cursor != head), cursor))
                    }
                    cursor = usize::from(probe.economy.world.offices[cursor].next)
                }
            }
            let selected = candidates
                .into_iter()
                .max_by_key(|v| v.0)
                .expect("oficina en cadena urbana");
            assert_eq!(selected.0, 1, "se requiere una oficina interior de la lista");
            let index = selected.1;
            let stock = &probe.economy.offices[index];
            let owner = stock.owner as u8;
            let city = stock.town;
            let other = ((usize::from(city) + 1) % probe.economy.world.towns.len()) as u8;
            for (merchant_id, record) in model.world.merchants.iter_mut().enumerate() {
                let mut m = p3_state::office::PersistedMerchant::from_save_v85(record).unwrap();
                m.merchant.set_u16(10, u16::MAX);
                if merchant_id == usize::from(owner) {
                    m.merchant.set_u16(8, 0);
                    m.merchant.bytes[0x3a] = 0;
                }
                *record = m.to_save_v85()
            }
            let mut image = p3_save::office::decode_office(model.world.offices[index].as_slice().try_into().unwrap());
            image[4..8].copy_from_slice(&20000i32.to_le_bytes());
            image[0x3b8..0x3bc].copy_from_slice(&1.25f32.to_bits().to_le_bytes());
            model.world.offices[index] = p3_save::office::encode_office(&image).to_vec();
            let mut task = TaskRecord {
                due: tick,
                next: u16::MAX,
                kind: 12,
                payload: [0; 16],
            };
            task.set_u8_at(8, if phase == 0 { city } else { other });
            task.set_u8_at(9, if phase == 0 { other } else { city });
            task.set_u8_at(10, owner);
            task.set_u8_at(11, 1);
            task.set_u32_at(0xc, 2.5f32.to_bits());
            task.set_u8_at(0x10, 170);
            task.set_u8_at(0x12, 5);
            task.set_u8_at(0x14, 0);
            task.set_u8_at(0x15, phase);
            let rec = [
                task.due.to_le_bytes().as_slice(),
                task.next.to_le_bytes().as_slice(),
                task.kind.to_le_bytes().as_slice(),
                task.payload.as_slice(),
            ]
            .concat();
            model.scheduled_tasks = [
                0u16.to_le_bytes().as_slice(),
                u16::MAX.to_le_bytes().as_slice(),
                1u16.to_le_bytes().as_slice(),
                rec.as_slice(),
            ]
            .concat();
            let mut pool = vec![0u8; 12 + 8 * 16];
            pool[..4].copy_from_slice(&0xaffe0005u32.to_le_bytes());
            pool[4..6].copy_from_slice(&8u16.to_le_bytes());
            pool[8..10].fill(255);
            for i in 0..8 {
                pool[12 + i * 16 + 4] = 255;
                pool[12 + i * 16 + 6..12 + i * 16 + 8].copy_from_slice(&((i + 1) as u16).to_le_bytes())
            }
            model.message_pool = pool;
            if phase == 7 {
                let mut bad = Sim::from_save_bytes(&model.write()).unwrap();
                bad.host.message_pool = vec![1, 2, 3];
                let previous_task = bad.host.council_scheduler.records.clone();
                let previous_stocks = bad.economy.offices.iter().map(|v| v.stock).collect::<Vec<_>>();
                let previous_merchant = bad.rep_merchants[usize::from(owner)].bytes.clone();
                {
                    let mut bridge = p3_state::office::OfficeTownHost {
                        offices: &mut bad.persisted,
                        merchants: &mut bad.rep_merchants,
                        businesses: &mut bad.businesses,
                        captain_wages: &mut bad.wages,
                        difficulty: bad.commercial_sell_factor,
                        other: &mut bad.host,
                        completed: Vec::new(),
                        errors: Vec::new(),
                    };
                    bridge.dispatch_scheduled_tasks_fleet(
                        &mut bad.economy.world,
                        &mut bad.economy.merchants,
                        &[],
                        &mut bad.economy.offices,
                        &mut bad.economy.cargo_ships,
                        2.0,
                        tick,
                    );
                }
                assert!(!bad.host.council_errors.is_empty());
                assert_eq!(bad.host.council_scheduler.records, previous_task);
                assert_eq!(
                    bad.economy.offices.iter().map(|v| v.stock).collect::<Vec<_>>(),
                    previous_stocks
                );
                assert_eq!(bad.rep_merchants[usize::from(owner)].bytes, previous_merchant);
                assert!(bad.host.land_transport_notifications.is_empty());
            }
            let mut sim = Sim::from_save_bytes(&model.write()).unwrap();
            let other_owner=(usize::from(owner)+1)%usize::from(sim.economy.world.merchant_count);
            let other_gold=sim.rep_merchants[other_owner].i32(0).wrapping_add(1777);
            let other_income=sim.rep_merchants[other_owner].i32(0x4c8).wrapping_add(1333);
            sim.economy.world.merchants[other_owner].gold=other_gold;
            sim.economy.world.merchants[other_owner].income=other_income;
            sim.economy.world.merchants[usize::from(owner)].gold=sim.rep_merchants[usize::from(owner)].i32(0).wrapping_add(777);
            let raw_offices = sim.current_office_pool().unwrap().records.concat();
            let towns = sim
                .economy
                .world
                .towns
                .iter()
                .flat_map(|t| t.0.iter().copied())
                .collect::<Vec<_>>();
            let merchants = sim.rep_merchants.iter().zip(&sim.economy.world.merchants).flat_map(|(m,account)|{let mut image=m.bytes.clone();image[..4].copy_from_slice(&account.gold.to_le_bytes());image[0x4c8..0x4cc].copy_from_slice(&account.income.to_le_bytes());image}).collect::<Vec<_>>();
            let d = p3_core::calendar::Calendar::for_tick(tick);
            let input = serde_json::json!({"binary":exe,"tasks":hex(&model.scheduled_tasks),"towns":hex(&towns),"merchants":hex(&merchants),"offices":hex(&raw_offices),"town_count":sim.economy.world.towns.len(),"merchant_count":sim.economy.world.merchant_count,"office_count":sim.economy.world.offices.len(),"pool":hex(&model.message_pool),"town_ids":sim.host.expedition_town_ids,"tick":tick,"date":{"day":d.day,"month":d.month,"year":d.year,"doy":d.day_of_year},"owner":owner});
            let script =
                std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../oracle/x86_c10_land_transport_live.py");
            let mut child = std::process::Command::new(&py)
                .arg(script)
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .spawn()
                .unwrap();
            child
                .stdin
                .take()
                .unwrap()
                .write_all(input.to_string().as_bytes())
                .unwrap();
            let out = child.wait_with_output().unwrap();
            assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
            let expected: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
            {
                let mut bridge = p3_state::office::OfficeTownHost {
                    offices: &mut sim.persisted,
                    merchants: &mut sim.rep_merchants,
                    businesses: &mut sim.businesses,
                    captain_wages: &mut sim.wages,
                    difficulty: sim.commercial_sell_factor,
                    other: &mut sim.host,
                    completed: Vec::new(),
                    errors: Vec::new(),
                };
                bridge.dispatch_scheduled_tasks_fleet(
                    &mut sim.economy.world,
                    &mut sim.economy.merchants,
                    &[],
                    &mut sim.economy.offices,
                    &mut sim.economy.cargo_ships,
                    2.0,
                    tick,
                );
                assert!(bridge.errors.is_empty(), "{:?}", bridge.errors)
            }
            assert!(sim.host.council_errors.is_empty(), "{:?}", sim.host.council_errors);
            assert_eq!(sim.economy.world.merchants[other_owner].gold,other_gold);assert_eq!(sim.economy.world.merchants[other_owner].income,other_income);assert_eq!(sim.rep_merchants[other_owner].i32(0),other_gold);assert_eq!(sim.rep_merchants[other_owner].i32(0x4c8),other_income);
            assert_eq!(
                serde_json::json!(sim.host.land_transport_notifications),
                expected["events"],
                "notificaciones fase {phase}"
            );
            sim.persist_all(&mut model).unwrap();
            assert_eq!(
                model.scheduled_tasks,
                unhex(expected["tasks"].as_str().unwrap()),
                "tasks phase {phase}"
            );
            assert_eq!(
                model.message_pool,
                unhex(expected["pool"].as_str().unwrap()),
                "pool phase {phase}"
            );
            assert_eq!(
                model.world.merchants[usize::from(owner)],
                unhex(expected["merchant"].as_str().unwrap()),
                "merchant phase {phase}"
            );
            assert_eq!(
                serde_json::json!(model.world.offices.iter().map(|v| hex(v)).collect::<Vec<_>>()),
                expected["offices"],
                "offices phase {phase}"
            );
            let saved = model.write();
            assert_eq!(model.write(), saved);
            let before_reload = model.clone();
            let loaded = Sim::from_save_bytes(&saved).unwrap();
            assert_eq!(
                loaded.economy.offices[index].stock[0],
                sim.economy.offices[index].stock[0]
            );
            loaded.persist_all(&mut model).unwrap();
            assert_eq!(model.scheduled_tasks, before_reload.scheduled_tasks);
            assert_eq!(model.message_pool, before_reload.message_pool);
            assert_eq!(model.world.offices[index], before_reload.world.offices[index]);
            assert_eq!(
                model.world.merchants[usize::from(owner)],
                before_reload.world.merchants[usize::from(owner)]
            );
        }
    }
}
