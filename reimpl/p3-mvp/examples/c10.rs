//! Ejecutable sin interfaz para el recorrido de C10. No modifica la partida de entrada.
use p3_mvp::{headless, Session};
use sha2::{Digest, Sha256};
use std::{
    env, fs,
    fs::OpenOptions,
    io::Write,
    path::{Path, PathBuf},
};

fn sha256(bytes: &[u8]) -> String {
    Sha256::digest(bytes).iter().map(|b| format!("{b:02x}")).collect()
}

/// Create every output before writing; roll back only files created by this call.
/// Existing destinations and input files are never replaced or removed.
fn publish_outputs(outputs: &[(&Path, &[u8])]) -> Result<(), String> {
    let mut created: Vec<(PathBuf, fs::File)> = Vec::new();
    let result: Result<(), String> = (|| {
        for (path, _) in outputs {
            let file = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(path)
                .map_err(|e| format!("salida {}: {e}", path.display()))?;
            created.push((path.to_path_buf(), file));
        }
        for ((path, file), (_, bytes)) in created.iter_mut().zip(outputs) {
            file.write_all(bytes)
                .and_then(|_| file.sync_all())
                .map_err(|e| format!("salida {}: {e}", path.display()))?;
        }
        Ok(())
    })();
    if let Err(mut error) = result {
        for (path, file) in created {
            drop(file);
            if let Err(cleanup) = fs::remove_file(&path) {
                error.push_str(&format!("; no se pudo retirar {}: {cleanup}", path.display()));
            }
        }
        return Err(error);
    }
    Ok(())
}

/// Cotizaciones de referencia por lote, sin ejecutar una operación comercial.
fn market_quotes(session: &Session) -> Vec<serde_json::Value> {
    use p3_core::economy::prices::{buy_price, sell_price, Market, SMALL_LOT_WARES};
    session.sim.economy.world.towns.iter().enumerate().map(|(city,town)|{
        let wares=(0..24).map(|w|{
            let amount=if w>=20{10}else if SMALL_LOT_WARES[w]{200}else{2000};
            let market=Market{trading:town.more_flags()&0x100!=0,stock:town.stock(w),thresholds:std::array::from_fn(|j|town.threshold(w,j)),marked_ware:town.u8(0x6f0)};
            serde_json::json!({"ware":w,"amount":amount,"buy":buy_price(w,&market,amount),"sell":sell_price(w,&market,amount,session.sim.commercial_sell_factor)})
        }).collect::<Vec<_>>();serde_json::json!({"city":city,"wares":wares})
    }).collect()
}

fn run(input: impl Iterator<Item = String>) -> Result<serde_json::Value, String> {
    let mut args = Vec::new();
    let mut pst_in = None;
    let mut pst_out = None;
    let mut celebration = None;
    let mut investigation_local = false;
    let mut exe = None;
    let mut battle_news_stamp = None;
    let mut ai_work_residue = None;
    let mut siege_stack_tail: Option<[u8; 3]> = None;
    let mut human_battles_auto = false;
    let mut battle_pacing = None;
    let mut input = input;
    while let Some(arg) = input.next() {
        match arg.as_str() {
            "--pst" => {
                if pst_in.is_some() {
                    return Err("--pst repetido".into());
                }
                pst_in = Some(input.next().ok_or("falta archivo tras --pst")?);
            }
            "--out-pst" => {
                if pst_out.is_some() {
                    return Err("--out-pst repetido".into());
                }
                pst_out = Some(input.next().ok_or("falta archivo tras --out-pst")?);
            }
            "--celebration-state" => {
                if celebration.is_some() {
                    return Err("--celebration-state repetido".into());
                }
                let value = input.next().ok_or("falta NEXT:TOWN tras --celebration-state")?;
                let (next, town) = value.split_once(':').ok_or("--celebration-state necesita NEXT:TOWN")?;
                celebration = Some(p3_core::economy::celebration::CelebrationState {
                    next: next.parse().map_err(|_| "tick de fiestas inválido")?,
                    town: town.parse().map_err(|_| "ciudad de fiestas inválida")?,
                });
            }
            "--investigation-local" => {
                if investigation_local {
                    return Err("--investigation-local repetido".into());
                }
                investigation_local = true;
            }
            "--battle-news-stamp" => {
                if battle_news_stamp.is_some() {
                    return Err("--battle-news-stamp repetido".into());
                }
                battle_news_stamp = Some(
                    input
                        .next()
                        .ok_or("falta valor tras --battle-news-stamp")?
                        .parse::<u32>()
                        .map_err(|_| "marca de noticias de batalla inválida")?,
                );
            }
            "--ai-work-residue" => {
                if ai_work_residue.is_some() {
                    return Err("--ai-work-residue repetido".into());
                }
                ai_work_residue = Some(
                    input
                        .next()
                        .ok_or("falta valor tras --ai-work-residue")?
                        .parse::<u32>()
                        .map_err(|_| "ranura de obra de IA inválida")?,
                );
            }
            "--human-battles-auto" => {
                if human_battles_auto {
                    return Err("--human-battles-auto repetido".into());
                }
                human_battles_auto = true;
            }
            "--battle-pacing" => {
                if battle_pacing.is_some() {
                    return Err("--battle-pacing repetido".into());
                }
                battle_pacing = Some(match input.next().ok_or("falta clock o paused tras --battle-pacing")?.as_str() {
                    "clock" => headless::BattlePacing::WithClock { frame_ms: headless::DEFAULT_FRAME_MS },
                    "paused" => headless::BattlePacing::Paused,
                    other => return Err(format!("--battle-pacing desconocido: {other} (clock o paused)")),
                });
            }
            "--siege-stack-tail" => {
                if siege_stack_tail.is_some() {
                    return Err("--siege-stack-tail repetido".into());
                }
                let value = input.next().ok_or("falta A,B,C tras --siege-stack-tail")?;
                let parts = value.split(',').map(|v| v.trim().parse::<u8>()).collect::<Result<Vec<_>, _>>().map_err(|_| "bytes de pila de asedio inválidos")?;
                siege_stack_tail = Some(parts.try_into().map_err(|_| "--siege-stack-tail necesita tres bytes")?);
            }
            "--exe" => {
                if exe.is_some() {
                    return Err("--exe repetido".into());
                }
                exe = Some(input.next().ok_or("falta archivo tras --exe")?);
            }
            _ if arg.starts_with("--") => return Err(format!("opción desconocida: {arg}")),
            _ => args.push(arg),
        }
    }
    if !(4..=5).contains(&args.len()) {
        return Err("uso: c10 GUARDADO.pat ARCHIVO.cpr DÍAS ESTADO_CRT_DECIMAL [SALIDA.pat] [--celebration-state NEXT:TOWN] [--investigation-local] [--exe ORIGINAL.exe] [--pst ENTRADA.pst] [--out-pst SALIDA.pst] [--battle-news-stamp VALOR] [--ai-work-residue VALOR] [--siege-stack-tail A,B,C] [--human-battles-auto] [--battle-pacing paused|clock]".into());
    }
    if pst_out.is_some() && args.len() != 5 || pst_in.is_some() && args.len() == 5 && pst_out.is_none() {
        return Err("--out-pst necesita SALIDA.pat; al guardar con --pst también se necesita --out-pst".into());
    }
    let days: u32 = args[2].parse().map_err(|_| "días inválidos")?;
    let crt: u32 = args[3].parse().map_err(|_| "estado CRT inválido")?;
    let raw = fs::read(&args[0]).map_err(|e| format!("partida: {e}"))?;
    let cpr = fs::read(&args[1]).map_err(|e| format!("recursos: {e}"))?;
    let mut session = Session::open(&raw, cpr)?;
    if let Some(path) = &pst_in {
        session
            .sim
            .configure_statistics(&fs::read(path).map_err(|e| format!("entrada PST: {e}"))?)?;
    }
    session.sim.configure_crt_state(crt);
    session.sim.host.celebration_state = celebration;
    session.sim.host.investigation_local_mode = investigation_local;
    // Ranura de pila sin inicializar de 0x004F5A20: sin valor, la obra de venta de la IA solo se acepta si no depende de ella.
    session.sim.host.ai_work_residue = ai_work_residue;
    // Bytes de pila sin inicializar con que el original completa las mercancías pedidas por un asedio (choose_wares).
    session.sim.host.siege_stack_tail = siege_stack_tail;
    // Política sin pantalla (no paridad): un convoy humano atacado por piratas se resuelve en batalla automática,
    // como el original con dos humanos en turnos; sustituye el diálogo y el combate del jugador.
    session.sim.host.headless_human_battles = human_battles_auto;
    // Si no se indica, usa el EXE inglés situado junto al CPR cuando exista.
    let exe_path = exe.map(PathBuf::from).or_else(|| {
        Path::new(&args[1])
            .parent()
            .map(|p| p.join("Patrician3.exe"))
            .filter(|p| p.is_file())
    });
    if let Some(path) = exe_path {
        let bytes = fs::read(path).map_err(|e| format!("ejecutable: {e}"))?;
        session.configure_battle_texts(&bytes)?;
        session.sim.configure_election_texts(&bytes)?;
        session.sim.configure_hunt_texts(&bytes)?;
    }
    session.sim.host.siege_headless_march = true;
    session.enable_full_fleet()?;
    session.sim.host.battle_news_stamp = battle_news_stamp.unwrap_or(0);
    let start = session.sim.summary();
    let start_prices = market_quotes(&session);
    let city_count = session.sim.economy.world.towns.len();
    let ship_count = session.sim.host.navigation.as_ref().map_or(0, |nav| {
        nav.snapshot.routes.iter().filter(|route| route.is_some()).count()
    });
    let report = match headless::advance_days_with(&mut session, days, battle_pacing.unwrap_or(headless::BattlePacing::Paused)) {
        Ok(r) => r,
        Err(e) => {
            // La partida se detuvo: los errores de tareas acumulados siguen siendo útiles para priorizar.
            let mut task_errors = std::collections::BTreeMap::<String, usize>::new();
            for err in &session.sim.host.council_errors {
                *task_errors.entry(err.clone()).or_default() += 1;
            }
            let date = p3_core::calendar::Calendar::for_tick(session.sim.idle.clock.tick);
            eprintln!("{}", serde_json::json!({"stopped": e, "date": [date.day, date.month, date.year], "task_errors": task_errors}));
            return Err(e);
        }
    };
    let end = session.sim.summary();
    let end_prices = market_quotes(&session);
    let saved = session.save()?;
    // No daily capture occurs for zero days. An explicitly requested new PST
    // represents game_statistics_clear (0x54DDF0), never invented past history.
    let captured_stats = session.sim.statistics_bytes();
    let statistics_origin = if pst_in.is_some() {
        "loaded"
    } else if captured_stats.is_some() {
        "new_capture"
    } else if pst_out.is_some() {
        "new_empty"
    } else {
        "absent"
    };
    let saved_stats = captured_stats.or_else(|| {
        pst_out.as_ref().map(|_| {
            let empty = p3_save::model::StatisticsModel {
                header: [0; 6],
                chains: std::array::from_fn(|_| None),
                items: Vec::new(),
                slack: 0,
            };
            p3_core::route::compress::compress(&empty.write())
        })
    });
    if let Some(path) = args.get(4) {
        let mut outputs = vec![(Path::new(path), saved.as_slice())];
        if let Some(stats_path) = &pst_out {
            let bytes = saved_stats.as_deref().ok_or("no hay estadísticas para la salida PST")?;
            outputs.push((Path::new(stats_path), bytes));
        }
        publish_outputs(&outputs)?;
    }
    // Errores de tareas programadas que no detienen la partida (la tarea se conserva), agrupados por texto.
    let mut task_errors = std::collections::BTreeMap::<String, usize>::new();
    for e in &session.sim.host.council_errors {
        *task_errors.entry(e.clone()).or_default() += 1;
    }
    Ok(
        serde_json::json!({"advance": report, "task_errors": task_errors, "start_towns_merchants_hash": start.hash, "end_towns_merchants_hash": end.hash,
        "input_save_sha256": sha256(&raw), "output_save_sha256": sha256(&saved),
        "start_gold": start.merchants_gold, "end_gold": end.merchants_gold,
        "start_population": start.population, "end_population": end.population,
        "start_town_money": start.town_money, "end_town_money": end.town_money,
        "start_market_quotes":start_prices,"end_market_quotes":end_prices,
        "city_count": city_count, "start_ship_count": ship_count,
        "end_ship_count": session.sim.host.navigation.as_ref().map_or(0, |nav| nav.snapshot.routes.iter().filter(|route| route.is_some()).count()),
        "battle_news_stamp": session.sim.host.battle_news_stamp,
        "statistics_origin": statistics_origin, "statistics_input": pst_in,
        "final_crt": session.sim.host.investigation_rng.map(|rng| rng.state),
        "final_celebration": session.sim.host.celebration_state.map(|v|serde_json::json!({"next":v.next,"town":v.town})),
        "investigation_local": investigation_local,
        "saved_bytes": saved.len(), "output": args.get(4), "statistics_sha256": saved_stats.as_deref().map(sha256), "statistics_output": pst_out}),
    )
}
fn main() {
    match run(env::args().skip(1)) {
        Ok(report) => println!("{report}"),
        Err(error) => {
            eprintln!("C10 detenido: {error}");
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    static NEXT: AtomicU64 = AtomicU64::new(0);
    struct Temp(PathBuf);
    impl Temp {
        fn new() -> Self {
            let path = env::temp_dir().join(format!(
                "p3-c10-cli-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir(&path).unwrap();
            Self(path)
        }
        fn path(&self, name: &str) -> PathBuf {
            self.0.join(name)
        }
    }
    impl Drop for Temp {
        fn drop(&mut self) {
            for entry in fs::read_dir(&self.0).into_iter().flatten().flatten() {
                let _ = fs::remove_file(entry.path());
            }
            let _ = fs::remove_dir(&self.0);
        }
    }
    fn inputs() -> Option<(String, String)> {
        Some((
            format!("{}/visibility/save/Ein/AUTO.pat", env::var("P3_SAVES").ok()?),
            env::var("P3_CPR").ok()?,
        ))
    }
    fn args(save: &str, cpr: &str, days: u32, pat: &Path, pst: &Path) -> Vec<String> {
        vec![
            save.into(),
            cpr.into(),
            days.to_string(),
            "123".into(),
            pat.to_string_lossy().into_owned(),
            "--out-pst".into(),
            pst.to_string_lossy().into_owned(),
            "--celebration-state".into(),
            "0:255".into(),
            "--investigation-local".into(),
            "--battle-news-stamp".into(),
            "123456".into(),
        ]
    }

    #[test]
    fn output_pair_creation_errors_leave_preexisting_files_untouched() {
        let temp = Temp::new();
        let pat = temp.path("out.pat");
        let pst = temp.path("out.pst");
        fs::write(&pst, b"existing PST").unwrap();
        assert!(publish_outputs(&[(&pat, b"new PAT"), (&pst, b"new PST")]).is_err());
        assert!(!pat.exists());
        assert_eq!(fs::read(&pst).unwrap(), b"existing PST");
        fs::write(&pat, b"existing PAT").unwrap();
        let new_pst = temp.path("new.pst");
        assert!(publish_outputs(&[(&pat, b"new PAT"), (&new_pst, b"new PST")]).is_err());
        assert!(!new_pst.exists());
        assert_eq!(fs::read(&pat).unwrap(), b"existing PAT");
        let duplicate = temp.path("duplicate");
        assert!(publish_outputs(&[(&duplicate, b"PAT"), (&duplicate, b"PST")]).is_err());
        assert!(!duplicate.exists());
    }

    #[test]
    fn option_errors_are_reported_before_opening_inputs() {
        let a = vec!["missing.pat", "missing.cpr", "0", "123", "--out-pst", "out.pst"];
        assert!(run(a.into_iter().map(str::to_owned))
            .unwrap_err()
            .contains("necesita SALIDA.pat"));
        let a = vec!["missing.pat", "missing.cpr", "0", "123", "out.pat", "--pst", "in.pst"];
        assert!(run(a.into_iter().map(str::to_owned))
            .unwrap_err()
            .contains("también se necesita --out-pst"));
        let a = vec!["--battle-news-stamp", "invalid"];
        assert!(run(a.into_iter().map(str::to_owned))
            .unwrap_err()
            .contains("marca de noticias"));
    }

    #[test]
    fn cli_writes_new_statistics_and_reports_the_transient_battle_clock() {
        let Some((input, cpr)) = inputs() else { return };
        let before = fs::read(&input).unwrap();
        let temp = Temp::new();
        for days in [0, 1] {
            let pat = temp.path(&format!("day{days}.pat"));
            let pst = temp.path(&format!("day{days}.pst"));
            let report = run(args(&input, &cpr, days, &pat, &pst).into_iter()).unwrap();
            assert_eq!(report["battle_news_stamp"], 123456);
            let quotes = report["end_market_quotes"].as_array().unwrap();
            assert_eq!(quotes.len(), report["city_count"].as_u64().unwrap() as usize);
            assert_eq!(quotes[0]["wares"].as_array().unwrap().len(), 24);
            assert_eq!(quotes[0]["wares"][0]["amount"], 2000);
            assert_eq!(quotes[0]["wares"][20]["amount"], 10);
            if days == 0 {
                assert_eq!(report["start_market_quotes"], report["end_market_quotes"])
            }
            assert_eq!(
                report["statistics_origin"],
                if days == 0 { "new_empty" } else { "new_capture" }
            );
            assert_eq!(report["statistics_sha256"], sha256(&fs::read(&pst).unwrap()));
            assert_eq!(report["output_save_sha256"], sha256(&fs::read(&pat).unwrap()));
            let (raw, _) = p3_core::route::rou::decompress(&fs::read(&pst).unwrap()).unwrap();
            let stats = p3_save::model::StatisticsModel::parse(&raw).unwrap();
            assert_eq!(stats.header[0], days as u16);
            if days == 0 {
                // Exact cleared state used by 0x54DDF0 and the daily x86 oracle.
                assert_eq!(stats.header, [0; 6]);
                assert!(stats.chains.iter().all(Option::is_none));
                assert!(stats.items.is_empty());
                assert_eq!(stats.slack, 0);
            } else {
                assert!(!stats.items.is_empty());
                assert_eq!(stats.chains[0].as_ref().unwrap().len(), 1);
            }
            p3_save::model::SaveModel::parse(&fs::read(&pat).unwrap()).unwrap();
        }
        assert_eq!(fs::read(&input).unwrap(), before);
    }

    #[test]
    fn cli_zero_day_output_conflict_rolls_back_the_new_pat() {
        let Some((input, cpr)) = inputs() else { return };
        let temp = Temp::new();
        let pat = temp.path("out.pat");
        let pst = temp.path("out.pst");
        fs::write(&pst, b"do not replace").unwrap();
        let result = run(args(&input, &cpr, 0, &pat, &pst).into_iter());
        assert!(result.unwrap_err().contains("salida"));
        assert!(!pat.exists());
        assert_eq!(fs::read(&pst).unwrap(), b"do not replace");
    }
}
