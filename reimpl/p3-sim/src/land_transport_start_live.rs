//! Operación 0x3A (`0x0053A2C0`) sobre partidas reales frente al original: `Sim::start_land_transport` contra la operación y el
//! despachador x86 (`oracle/x86_c10_land_transport_live.py` con `op`), y continuación del viaje con el reloj hasta la entrega.
#![cfg(test)]
use crate::Sim;
use p3_core::economy::land_transport_send as lt;
use p3_core::pump::Operation;
use std::io::Write;

fn hex(v: &[u8]) -> String {
    v.iter().map(|b| format!("{b:02x}")).collect()
}
fn unhex(s: &str) -> Vec<u8> {
    s.as_bytes().chunks_exact(2).map(|v| u8::from_str_radix(std::str::from_utf8(v).unwrap(), 16).unwrap()).collect()
}

fn oracle(py: &str, input: &serde_json::Value) -> serde_json::Value {
    let script = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../oracle/x86_c10_land_transport_live.py");
    let mut child = std::process::Command::new(py)
        .arg(script)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(input.to_string().as_bytes()).unwrap();
    let out = child.wait_with_output().unwrap();
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    serde_json::from_slice(&out.stdout).unwrap()
}

/// Oficina del jugador en una ciudad unida por tierra.
fn pick(sim: &Sim) -> Option<(u8, u8, u8, usize)> {
    let ids = &sim.host.expedition_town_ids;
    for (i, o) in sim.economy.offices.iter().enumerate() {
        // Comerciante humano: `+8` a cero (como la selección de oficina de la sesión).
        if !sim.rep_merchants.get(usize::from(o.owner)).is_some_and(|m| m.u16(8) == 0) {
            continue;
        }
        let player = o.owner as u8;
        if let Some(dest) = lt::land_partner(&sim.economy.world.towns, ids, usize::from(o.town)) {
            return Some((player, o.town, dest, i));
        }
    }
    None
}

#[test]
fn start_land_transport_matches_x86_on_real_saves() {
    let (Ok(root), Ok(exe), Ok(py)) = (std::env::var("P3_SAVES"), std::env::var("P3_EXE"), std::env::var("P3_ORACLE_PY")) else {
        eprintln!("SKIPPED: P3_SAVES, P3_EXE y P3_ORACLE_PY requeridos");
        return;
    };
    let (mut checked, mut phase1) = (0, 0);
    for name in ["lab-english/save/Ein/TEST25_ConvoyMar.pat", "fps-cache/save/Ein/AUTO.pat", "visibility/save/Ein/TestAstiller.pat"] {
        let Ok(raw) = std::fs::read(format!("{root}/{name}")) else {
            eprintln!("OMITIDA {name}: no está");
            continue;
        };
        let probe = Sim::from_save_bytes(&raw).unwrap();
        let Some((player, origin, dest, office)) = pick(&probe) else {
            eprintln!("OMITIDA {name}: el jugador no tiene oficina en una ciudad unida por tierra");
            continue;
        };
        let stock = probe.economy.offices[office].stock;
        // Variantes: cargas a granel y en barriles, más cargas de las que hay, pérdidas, sin carro y escoltas distintas.
        let goods: Vec<usize> = (0..20).filter(|&g| stock[g] >= lt::unit(g) as i32).collect();
        let bulk = goods.iter().copied().find(|&g| lt::BARRELS[g] == 0).unwrap_or(0);
        let barrel = goods.iter().copied().find(|&g| lt::BARRELS[g] != 0).unwrap_or(3);
        let variants: Vec<(usize, u16, u16, u8, u16, u8)> = vec![
            (bulk, 1, 50, 1, 0, 0),
            (barrel, 3, 200, 1, 0, 1),
            (bulk, 250, 800, 1, 0, 2),
            (barrel, 2, 0, 0, 1, 0),
            (bulk, 4, 50, 1, 3, 0),
            (19, 0, 0, 1, 0, 0),
        ];
        for (k, &(good, loads, cost, flag, lost, escort)) in variants.iter().enumerate() {
            // Buzón vacío de ocho ranuras (como `land_transport_stage`): el del guardado lleva textos que el serializador x86
            // copia desde punteros que la memoria del oráculo no tiene.
            let mut model = p3_save::model::SaveModel::parse(&raw).unwrap();
            let mut pool = vec![0u8; 12 + 8 * 16];
            pool[..4].copy_from_slice(&0xaffe0005u32.to_le_bytes());
            pool[4..6].copy_from_slice(&8u16.to_le_bytes());
            pool[8..10].fill(255);
            for i in 0..8 {
                pool[12 + i * 16 + 4] = 255;
                pool[12 + i * 16 + 6..12 + i * 16 + 8].copy_from_slice(&((i + 1) as u16).to_le_bytes())
            }
            model.message_pool = pool;
            let raw = model.write();
            let mut sim = Sim::from_save_bytes(&raw).unwrap();
            let tick = sim.idle.clock.tick;
            let op = Operation([
                lt::OP_START_LAND_TRANSPORT,
                u32::from_le_bytes([origin, dest, player, flag]),
                u32::from(loads) | u32::from(cost) << 16,
                u32::from(lost) | (good as u32) << 16 | u32::from(escort) << 24,
                0,
            ]);
            let raw_offices = sim.current_office_pool().unwrap().records.concat();
            let towns = sim.economy.world.towns.iter().flat_map(|t| t.0.iter().copied()).collect::<Vec<_>>();
            let merchants = sim
                .rep_merchants
                .iter()
                .zip(&sim.economy.world.merchants)
                .flat_map(|(m, account)| {
                    let mut image = m.bytes.clone();
                    image[..4].copy_from_slice(&account.gold.to_le_bytes());
                    image[0x4c8..0x4cc].copy_from_slice(&account.income.to_le_bytes());
                    image
                })
                .collect::<Vec<_>>();
            let d = p3_core::calendar::Calendar::for_tick(tick);
            let input = serde_json::json!({"binary":exe,"tasks":hex(&model.scheduled_tasks),"towns":hex(&towns),"merchants":hex(&merchants),"offices":hex(&raw_offices),"town_count":sim.economy.world.towns.len(),"merchant_count":sim.economy.world.merchant_count,"office_count":sim.economy.world.offices.len(),"pool":hex(&model.message_pool),"town_ids":sim.host.expedition_town_ids,"tick":tick,"date":{"day":d.day,"month":d.month,"year":d.year,"doy":d.day_of_year},"owner":player,"op":op.0,"raw_tasks":true});
            let expected = oracle(&py, &input);
            let idx = sim.start_land_transport(op).unwrap_or_else(|e| panic!("{name} variante {k}: {e}"));
            assert_eq!(idx.is_some(), loads != 0, "{name} variante {k}");
            if let Some(t) = idx {
                // La fase 0 corre en el acto: la tarea pasa a la fase 1 (carro saliendo) y vence en el tick siguiente.
                let r = sim.host.council_scheduler.records[usize::from(t)];
                assert_eq!((r.kind, r.u8_at(0x15), r.due), (0x0C, 1, tick + 1), "{name} variante {k}: fase");
                phase1 += 1;
            }
            assert!(sim.host.council_errors.is_empty(), "{name} variante {k}: {:?}", sim.host.council_errors);
            assert_eq!(serde_json::json!(sim.host.land_transport_notifications), expected["events"], "{name} variante {k}: avisos");
            let mut out = model.clone();
            sim.persist_all(&mut out).unwrap();
            let q = &sim.host.council_scheduler;
            let mut raw_tasks = [q.head.to_le_bytes(), q.free_head.to_le_bytes(), q.capacity.to_le_bytes()].concat();
            for r in &q.records {
                raw_tasks.extend_from_slice(&r.due.to_le_bytes());
                raw_tasks.extend_from_slice(&r.next.to_le_bytes());
                raw_tasks.extend_from_slice(&r.kind.to_le_bytes());
                raw_tasks.extend_from_slice(&r.payload);
            }
            assert_eq!(hex(&raw_tasks), expected["tasks"].as_str().unwrap(), "{name} variante {k}: tareas");
            assert_eq!(hex(&out.message_pool), expected["pool"].as_str().unwrap(), "{name} variante {k}: cartas");
            assert_eq!(hex(&out.world.merchants[usize::from(player)]), expected["merchant"].as_str().unwrap(), "{name} variante {k}: comerciante");
            assert_eq!(serde_json::json!(out.world.offices.iter().map(|v| hex(v)).collect::<Vec<_>>()), expected["offices"], "{name} variante {k}: oficinas");
            // Guardar y volver a abrir conserva la tarea.
            let saved = out.write();
            let back = Sim::from_save_bytes(&saved).unwrap();
            assert_eq!(back.host.council_scheduler.records, sim.host.council_scheduler.records, "{name} variante {k}: recarga");
            let _ = unhex;
            checked += 1;
        }
    }
    eprintln!("operaciones 0x3A contrastadas: {checked} ({phase1} en fase 1)");
    assert!(checked > 0, "ninguna partida con oficina del jugador en una ciudad unida por tierra");
}
