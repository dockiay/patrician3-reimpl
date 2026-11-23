//! Exporta estados inicial/primer tick/primer día con el perfil del oráculo real.
//! Usa el protocolo headless: pausa mundo, resuelve batalla IA y reanuda.
//! Se detiene ante un efecto no conectado o una intervención del jugador.
use p3_mvp::Session;
use sha2::{Digest, Sha256};
use std::{env, fs, path::PathBuf};

fn run() -> Result<(), String> {
    let mut human_battles_auto = false;
    let mut siege_tails = None;
    let mut pacing = p3_mvp::headless::BattlePacing::Paused;
    let mut crt = 123u32;
    let mut ai_residues: Option<String> = None;
    let mut probe: Option<(u32, u32, u32)> = None;
    let mut ai_work_residue: Option<u32> = None;
    let mut args = Vec::new();
    let mut input = env::args().skip(1);
    while let Some(arg) = input.next() {
        match arg.as_str() {
            // Informe del oráculo x86 (`siege_stack_tails`): bytes de pila que el original emulado
            // deja en 0x635220 en cada elección mensual de demandas de asedio, en orden.
            "--siege-stack-tails" => siege_tails = Some(input.next().ok_or("falta el informe tras --siege-stack-tails")?),
            // Política sin pantalla de C07_HEADLESS_HUMAN_BATTLES.md; el oráculo x86 la aplica igual
            // (`x86_c10_real_save_clock.py --human-battles-auto`, 0x006056A9).
            "--human-battles-auto" => human_battles_auto = true,
            // Ritmo de las batallas automáticas: `paused` (mundo detenido) o `clock` (mundo y batalla
            // intercalados con fotogramas de 40 ms, `headless::run_with_clock`). El oráculo x86 lo
            // reproduce con `x86_c10_real_save_clock.py --battle-pacing clock`.
            "--battle-pacing" => {
                pacing = match input.next().ok_or("falta clock o paused tras --battle-pacing")?.as_str() {
                    "clock" => p3_mvp::headless::BattlePacing::WithClock { frame_ms: 40 },
                    "paused" => p3_mvp::headless::BattlePacing::Paused,
                    other => return Err(format!("--battle-pacing desconocido: {other}")),
                }
            }
            // Estado del generador CRT tras la carga (por defecto 123, como el oráculo x86).
            // Ranura de pila sin inicializar de 0x004F5A20 (informe x86 `ai_work_slots`).
            "--ai-work-residue" => ai_work_residue = Some(input.next().ok_or("falta valor tras --ai-work-residue")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())?),
            // Muestras extra DESDE:HASTA:PASO (ticks desde el inicio): sirven para acotar un día distinto.
            // Un corte fuerza el final de una batalla viva: usarlo en los dos lados igual.
            "--probe-ticks" => {
                let v = input.next().ok_or("falta DESDE:HASTA:PASO")?;
                let n: Vec<u32> = v.split(':').map(|x| x.parse().map_err(|_| "DESDE:HASTA:PASO inválido")).collect::<Result<_, _>>()?;
                probe = Some((n[0], n[1], n[2]));
            }
            // Informe x86 con `ai_work_slots`: ranura de 0x004F5A20 por tick (residuo medido).
            "--ai-work-residues" => ai_residues = Some(input.next().ok_or("falta el informe tras --ai-work-residues")?),
            "--crt" => crt = input.next().ok_or("falta valor tras --crt")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())?,
            _ => args.push(arg),
        }
    }
    if !(4..=5).contains(&args.len()) {
        return Err("uso: c10_snapshots SAVE.pat ARCHIVE.cpr EXE OUT_DIR [DAYS] [--human-battles-auto] [--siege-stack-tails INFORME_X86.json]".into());
    }
    let days = args
        .get(4)
        .map(|s| s.parse::<u32>().map_err(|e| e.to_string()))
        .transpose()?
        .unwrap_or(1);
    if days > 1100 {
        return Err("DAYS debe estar entre 0 y 1100".into());
    }
    let raw = fs::read(&args[0]).map_err(|e| e.to_string())?;
    let archive = fs::read(&args[1]).map_err(|e| e.to_string())?;
    let exe = fs::read(&args[2]).map_err(|e| e.to_string())?;
    let out = PathBuf::from(&args[3]);
    if out.exists() && fs::read_dir(&out).map_err(|e| e.to_string())?.next().is_some() {
        return Err("el directorio de muestras debe estar vacío".into());
    }
    fs::create_dir_all(&out).map_err(|e| e.to_string())?;
    let hash = |bytes: &[u8]| format!("{:x}", Sha256::digest(bytes));
    let mut manifest = serde_json::json!({"complete":false,"requested_days":days,
        "input_sha256":hash(&raw),"archive_sha256":hash(&archive),"original_exe_sha256":hash(&exe),
        "test_executable_sha256":hash(&fs::read(env::current_exe().map_err(|e|e.to_string())?).map_err(|e|e.to_string())?),
        "compiled_sources":{"exporter":hash(include_bytes!("c10_snapshots.rs")),"headless":hash(include_bytes!("../src/headless.rs")),"session":hash(include_bytes!("../src/lib.rs")),"sim":hash(include_bytes!("../../p3-sim/src/lib.rs")),"navigation":hash(include_bytes!("../../p3-sim/src/navigation.rs"))},
        "configuration":{"crt":crt,"celebration":[0,255],"operations_mode":0,"host_slot":0,"hotseat":0,"token":0,"max_ticks":8,"freeze_world_during_ai_battle":matches!(pacing, p3_mvp::headless::BattlePacing::Paused),"battle_step":16,"original_pst":false,"human_battles_auto":human_battles_auto,"battle_pacing":format!("{pacing:?}")},"snapshots":[]});
    fs::write(out.join("manifest.json"), serde_json::to_vec_pretty(&manifest).unwrap()).map_err(|e| e.to_string())?;
    let mut session = Session::open(&raw, archive)?;
    session.sim.configure_crt_state(crt);
    session.sim.host.celebration_state = Some(p3_core::economy::celebration::CelebrationState { next: 0, town: 255 });
    session.sim.host.ai_work_residue = ai_work_residue;
    if let Some(path) = &ai_residues {
        let report: serde_json::Value =
            serde_json::from_slice(&fs::read(path).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
        for row in report["ai_work_slots"].as_array().ok_or("informe sin ai_work_slots")? {
            let tick = row["tick"].as_u64().ok_or("tick de ai_work_slots")? as u32;
            let low = row["slot_lo"].as_u64().ok_or("slot_lo de ai_work_slots")? as u32;
            session.sim.host.ai_work_residue_by_tick.push((tick, low));
        }
    }
    session.sim.host.investigation_local_mode = true;
    session.sim.host.siege_headless_march = true;
    session.sim.host.headless_human_battles = human_battles_auto;
    if let Some(path) = &siege_tails {
        let report: serde_json::Value =
            serde_json::from_slice(&fs::read(path).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
        for row in report["siege_stack_tails"].as_array().ok_or("informe sin siege_stack_tails")? {
            let tick = row["tick"].as_u64().ok_or("tick de pila de asedio")? as u32;
            let bytes: Vec<u8> = row["bytes"]
                .as_array()
                .ok_or("bytes de pila de asedio")?
                .iter()
                .map(|v| v.as_u64().map(|v| v as u8).ok_or("byte de pila de asedio"))
                .collect::<Result<_, _>>()?;
            session.sim.host.siege_stack_tails.push_back((tick, bytes.try_into().map_err(|_| "tres bytes de pila")?));
        }
        for row in report["siege_starts"].as_array().map(|v| v.as_slice()).unwrap_or(&[]) {
            let field = |name: &str| row[name].as_u64().ok_or(format!("inicio de asedio sin {name}"));
            session.sim.host.siege_start_clocks.push((field("tick")? as u32, field("city")? as u8, field("clock")? as u32));
        }
        manifest["configuration"]["siege_stack_tails"] = session.sim.host.siege_stack_tails.len().into();
    }
    session.configure_battle_texts(&exe)?;
    session.sim.configure_election_texts(&exe)?;
    session.sim.configure_hunt_texts(&exe)?;
    session.enable_full_fleet()?;
    let start = session.sim.idle.clock.tick;
    manifest["configuration"]["player"] = session.owner.into();
    manifest["start_tick"] = start.into();
    let mut snapshots = vec![(0, "day-00".to_string())];
    if days > 0 {
        snapshots.push((1, "tick-000001".to_string()));
    }
    snapshots.extend((1..=days).map(|d| (d * 256, format!("day-{d:02}"))));
    if let Some((from, to, step)) = probe {
        snapshots.extend((from..=to).step_by(step as usize).map(|o| (o, format!("probe-{o:07}"))));
        snapshots.sort_by_key(|(o, _)| *o);
        snapshots.dedup_by_key(|(o, _)| *o);
    }
    for (offset, label) in snapshots {
        if offset > 0 {
            let target = start.checked_add(offset).ok_or("objetivo fuera de rango")?;
            let remaining = target
                .checked_sub(session.sim.idle.clock.tick)
                .ok_or("muestra anterior al reloj")?;
            let result = p3_mvp::headless::advance_ticks_with(&mut session, remaining, pacing)?;
            eprintln!("{}", serde_json::json!({"label":label,"advance":result}));
        }
        let saved = session.save()?;
        let payload = p3_save::Save::parse(&saved).map_err(|e| e.to_string())?.to_unpacked();
        fs::write(out.join(format!("{label}.payload.bin")), &payload).map_err(|e| e.to_string())?;
        let sum = session.sim.summary();
        let row = serde_json::json!({"label":label,"tick":session.sim.idle.clock.tick,
            "owner":session.owner,"office":session.office,"city":session.city,
            "rng":session.sim.host.investigation_rng.map(|v|v.state),"gold":sum.merchants_gold,
            "town_money":sum.town_money,"population":sum.population,"bytes":payload.len(),
            "summary":{"merchants_gold":sum.merchants_gold,"town_money":sum.town_money,"population":sum.population,"stock":sum.stock,"town_count":session.sim.economy.world.towns.len(),"merchant_count":session.sim.economy.world.merchant_count},
            "sha256":format!("{:x}",Sha256::digest(&payload))});
        println!("{row}");
        manifest["snapshots"].as_array_mut().unwrap().push(row);
        fs::write(out.join("manifest.json"), serde_json::to_vec_pretty(&manifest).unwrap())
            .map_err(|e| e.to_string())?;
    }
    manifest["complete"] = true.into();
    fs::write(out.join("manifest.json"), serde_json::to_vec_pretty(&manifest).unwrap()).map_err(|e| e.to_string())?;
    Ok(())
}
fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1)
    }
}
