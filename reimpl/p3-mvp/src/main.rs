use p3_core::economy::trade::TradeSide;
use p3_mvp::Session;
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};
use std::path::Path;

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1)
    }
}
fn run() -> Result<(), String> {
    let args = std::env::args_os().skip(1).collect::<Vec<_>>();
    if args.len() != 3 {
        return Err("uso: p3-mvp PARTIDA.pat RECURSOS.cpr IMAGEN.png".into());
    }
    let save = std::fs::read(&args[0]).map_err(|e| format!("partida: {e}"))?;
    let archive = p3_assets::cpr::Bytes::map_file(&args[1]).map_err(|e| format!("recursos: {e}"))?;
    let mut session = if let Some(raw) = std::env::var_os("P3_CRT_STATE") {
        let value = raw.to_str().ok_or("P3_CRT_STATE inválido")?;
        let parsed = if let Some(hex) = value.strip_prefix("0x").or_else(|| value.strip_prefix("0X")) {
            u32::from_str_radix(hex, 16)
        } else {
            value.parse::<u32>()
        }
        .map_err(|_| "P3_CRT_STATE inválido")?;
        Session::open_with_paused_battle_and_crt(&save, archive, parsed)?
    } else {
        // Como el original al cargar una partida (`srand(timeGetTime())` en 0x00547F44): semilla del reloj del anfitrión.
        let ms = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_millis() as u32);
        Session::open_with_paused_battle_and_crt(&save, archive, ms)?
    };
    if let Some(path) = std::env::var_os("P3_ORIGINAL_EXE") {
        let bytes = std::fs::read(&path).map_err(|e| format!("ejecutable original: {e}"))?;
        session.configure_battle_texts(&bytes)?;
    } else if let Some(path) = Path::new(&args[1])
        .parent()
        .map(|p| p.join("Patrician3.exe"))
        .filter(|p| p.is_file())
    {
        if let Ok(bytes) = std::fs::read(path) {
            let _ = session.configure_battle_texts(&bytes);
        }
    }
    // Volumen de efectos del jugador: `P2.CFG` [OPTIONS] LSSOUND junto al juego (0x45CFCF; 80 si falta o no está en 0..=255).
    let cfg = Path::new(&args[1]).parent().map(|p| p.join("P2.cfg")).and_then(|p| std::fs::read(p).ok());
    session.set_effects_volume(p3_mvp::audio::effects_volume(cfg.as_deref()));
    let stdin = io::stdin();
    let mut stdout = io::BufWriter::new(io::stdout().lock());
    for line in stdin.lock().lines() {
        let line = line.map_err(|e| e.to_string())?;
        let response = match serde_json::from_str::<Value>(&line) {
            Ok(v) => handle(&mut session, &v, Path::new(&args[2])),
            Err(e) => Err(format!("petición inválida: {e}")),
        };
        let out = match response {
            Ok(v) => json!({"ok":true,"data":v}),
            Err(e) => json!({"ok":false,"error":e}),
        };
        serde_json::to_writer(&mut stdout, &out).map_err(|e| e.to_string())?;
        writeln!(stdout).map_err(|e| e.to_string())?;
        stdout.flush().map_err(|e| e.to_string())?;
    }
    Ok(())
}
fn build_args(v: &Value) -> Result<(u8, u8, u8), String> {
    let g = |k: &str| {
        v[k].as_u64()
            .and_then(|n| u8::try_from(n).ok())
            .ok_or(format!("falta {k}"))
    };
    Ok((g("type")?, g("row")?, g("col")?))
}
fn handle(s: &mut Session, v: &Value, image: &Path) -> Result<Value, String> {
    match v["command"].as_str().unwrap_or("") {
        "paused_battle_policy" => {
            let merchant = v["merchant"]
                .as_u64()
                .and_then(|n| u16::try_from(n).ok())
                .ok_or("participante inválido")?;
            let policy = v["policy"]
                .as_u64()
                .and_then(|n| u8::try_from(n).ok())
                .ok_or("política inválida")?;
            s.set_paused_battle_policy(merchant, policy)
        }
        "activate_encounter_battle" => s.activate_encounter_battle(),
        "pump_battle_time" => {
            let now = v["now_ms"]
                .as_u64()
                .and_then(|n| u32::try_from(n).ok())
                .ok_or("reloj inválido")?;
            let mode = v["mode"]
                .as_u64()
                .and_then(|n| u32::try_from(n).ok())
                .ok_or("modo de tiempo inválido")?;
            s.pump_battle_time(now, mode)
        }
        "drain_battle_time" => s.drain_battle_time(),
        "advance_battle" => {
            let ticks = v["ticks"]
                .as_u64()
                .and_then(|n| u16::try_from(n).ok())
                .ok_or("ticks de batalla inválidos")?;
            s.advance_paused_battle(ticks)
        }
        "weather" => {
            let n = |k: &str| v[k].as_i64().and_then(|n| i32::try_from(n).ok());
            let rect = v["rect"]
                .as_array()
                .filter(|a| a.len() == 4)
                .map(|a| {
                    a.iter()
                        .map(|x| x.as_i64().and_then(|n| i32::try_from(n).ok()))
                        .collect::<Option<Vec<_>>>()
                })
                .flatten()
                .ok_or("rectángulo inválido")?;
            s.weather(
                v["snow"].as_bool().unwrap_or(false),
                n("capacity").ok_or("capacidad inválida")?,
                n("count").ok_or("cuenta inválida")?,
                [rect[0], rect[1], rect[2], rect[3]],
                v["seed"].as_u64().and_then(|n| u32::try_from(n).ok()).unwrap_or(1),
            )
        }
        "scene_init" => s.scene_init(),
        "state" => Ok(s.state()),
        "enable_full_fleet" => s.enable_full_fleet(),
        "render" => {
            let fx = p3_mvp::Effects {
                weather: match v["weather"].as_str() {
                    Some("rain") => Some((false, v["precip_count"].as_i64().unwrap_or(400) as i32)),
                    Some("snow") => Some((true, v["precip_count"].as_i64().unwrap_or(400) as i32)),
                    _ => None,
                },
                cracks: v["cracks"]
                    .as_array()
                    .map(|a| {
                        a.iter()
                            .filter_map(|c| Some((c[0].as_u64()? as u32, c[1].as_u64()? as u32, c[2].as_u64()? as u32)))
                            .collect()
                    })
                    .unwrap_or_default(),
                seed: v["seed"].as_u64().unwrap_or(1) as u32,
                conductor: v["conductor_cracks"].as_bool().unwrap_or(false),
            };
            s.render_fx(
                image,
                v["clock_ms"].as_u64().and_then(|n| u32::try_from(n).ok()).unwrap_or(0),
                &fx,
            )
        }
        "render_world" => s.render_world(&image.with_file_name("mundo.png")),
        "navmap" => {
            let int = |k: &str, d: i64| v[k].as_i64().unwrap_or(d) as i32;
            let pair = |a: &serde_json::Value| Some((a[0].as_i64()? as i32, a[1].as_i64()? as i32));
            let keys = v["keys"].as_array().map(|a| a.iter().filter_map(|k| k.as_str()).collect::<Vec<_>>()).unwrap_or_default();
            let req = p3_mvp::navmap::NavRequest {
                width: int("width", 800),
                height: int("height", 600),
                clock_ms: v["clock_ms"].as_u64().and_then(|n| u32::try_from(n).ok()).unwrap_or(0),
                mouse: pair(&v["mouse"]),
                keys: ["left", "up", "right", "down"].map(|k| keys.contains(&k)),
                center: pair(&v["center"]),
                ticks: v["ticks"].as_u64().and_then(|n| u32::try_from(n).ok()).unwrap_or(0),
                selection: v["selection"].as_array().map(|a| a.iter().filter_map(|x| x.as_u64()).map(|x| x as u16).collect()).unwrap_or_default(),
            };
            s.navmap_frame(&req, &image.with_file_name("carta.ppm"))
        }
        "advance" => Ok(s.advance()),
        "advance_until_encounter" => {
            let target = v["target"]
                .as_u64()
                .and_then(|n| u32::try_from(n).ok())
                .ok_or("tick objetivo inválido")?;
            let text = v["dialog_bytes"]
                .as_array()
                .ok_or("faltan bytes del diálogo")?
                .iter()
                .map(|b| {
                    b.as_u64()
                        .and_then(|n| u8::try_from(n).ok())
                        .ok_or("byte de diálogo inválido")
                })
                .collect::<Result<Vec<_>, _>>()?;
            s.advance_until_encounter(target, text)
        }
        "complete_encounter_tick" => s.complete_encounter_tick(),
        "set_speed" => {
            let level = v["level"].as_u64().and_then(|n| usize::try_from(n).ok()).ok_or("nivel de velocidad inválido")?;
            s.set_game_speed(level)
        }
        "dispatch_encounter_policy" => s.dispatch_encounter_policy(),
        "town_sounds" => {
            let dir = image.parent().unwrap_or(Path::new("."));
            let pair = |a: &serde_json::Value| Some((a[0].as_i64()? as i32, a[1].as_i64()? as i32));
            let req = p3_mvp::SoundRequest {
                sites: v["sites"].as_array().map(|a| a.iter().filter_map(|t| Some((t[0].as_i64()? as i32, t[1].as_i64()? as i32, t[2].as_u64()? as u16))).collect()).unwrap_or_default(),
                collapses: v["collapses"].as_array().map(|a| a.iter().filter_map(pair).collect()).unwrap_or_default(),
                map_kinds: v["map_kinds"].as_array().map(|a| a.iter().filter_map(|k| k.as_u64().map(|k| k as u32)).collect()).unwrap_or_default(),
                map_events: v["map_events"]
                    .as_array()
                    .map(|a| {
                        a.iter()
                            .map(|e| {
                                let f = |k: &str| e[k].as_i64().unwrap_or(0);
                                (f("class") as u32, f("id") as u32, f("x") as i32, f("y") as i32, f("extra") as u32)
                            })
                            .collect()
                    })
                    .unwrap_or_default(),
                map_view: v["map_view"].as_array().filter(|a| a.len() == 4).map(|a| {
                    let f = |i: usize| a[i].as_i64().unwrap_or(0) as i32;
                    [f(0), f(1), f(2), f(3)]
                }),
            };
            s.town_sounds(v["clock_ms"].as_u64().and_then(|n| u32::try_from(n).ok()).unwrap_or(0), dir, &req)
        }
        "ambient" => {
            s.ambient(&image.with_file_name("ambiente.wav"))?;
            Ok(json!({}))
        }
        "office" => {
            let i = v["index"]
                .as_u64()
                .and_then(|n| usize::try_from(n).ok())
                .ok_or("oficina inválida")?;
            s.select_office(i)
        }
        "route_preview" => {
            let i = v["destination"]
                .as_u64()
                .and_then(|n| usize::try_from(n).ok())
                .ok_or("destino inválido")?;
            let result = s.preview_route(i)?;
            s.render_navigation(&image.with_file_name("navegacion.png"))?;
            Ok(result)
        }
        "trade" => {
            let side = match v["side"].as_str() {
                Some("buy") => TradeSide::Buy,
                Some("sell") => TradeSide::Sell,
                _ => return Err("operación desconocida".into()),
            };
            let ware = v["ware"]
                .as_u64()
                .and_then(|v| usize::try_from(v).ok())
                .ok_or("mercancía inválida")?;
            let quantity = v["quantity"]
                .as_i64()
                .and_then(|v| i32::try_from(v).ok())
                .ok_or("cantidad inválida")?;
            s.trade(side, ware, quantity)
        }
        "ship_trade" => {
            let side = match v["side"].as_str() {
                Some("buy") => TradeSide::Buy,
                Some("sell") => TradeSide::Sell,
                _ => return Err("operación desconocida".into()),
            };
            let ware = v["ware"]
                .as_u64()
                .and_then(|v| usize::try_from(v).ok())
                .ok_or("mercancía inválida")?;
            let quantity = v["quantity"]
                .as_i64()
                .and_then(|v| i32::try_from(v).ok())
                .ok_or("cantidad inválida")?;
            let id = v["ship"]
                .as_u64()
                .and_then(|v| usize::try_from(v).ok())
                .ok_or("barco inválido")?;
            s.trade_ship(id, side, ware, quantity)
        }
        "sail" => {
            let id = v["ship"]
                .as_u64()
                .and_then(|v| usize::try_from(v).ok())
                .ok_or("barco inválido")?;
            let dest = v["destination"]
                .as_u64()
                .and_then(|v| usize::try_from(v).ok())
                .ok_or("destino inválido")?;
            s.sail(id, dest)
        }
        "land_transport_quote" | "land_transport_send" => {
            let mut loads = [0u8; 20];
            if let Some(a) = v["loads"].as_array() {
                for (i, x) in a.iter().take(20).enumerate() {
                    loads[i] = x.as_u64().and_then(|n| u8::try_from(n).ok()).ok_or("cargas inválidas")?;
                }
            }
            let escort = v["escort"].as_u64().and_then(|n| u8::try_from(n).ok()).unwrap_or(0);
            if v["command"] == "land_transport_send" {
                s.send_land_transport(&loads, escort)
            } else {
                s.land_transport_quote(&loads, escort)
            }
        }
        "land_transports" => Ok(s.land_transports()),
        "ui_text" => {
            // Texto en latin-1 dentro de una cadena JSON (cada carácter es un byte).
            let text: Vec<u8> = v["text"].as_str().ok_or("falta text")?.chars().map(|c| u8::try_from(u32::from(c)).unwrap_or(b'?')).collect();
            let width = v["width"].as_u64().unwrap_or(420) as u32;
            let color = v["color"].as_u64().unwrap_or(0xFF20_1810) as u32;
            let font = v["font"].as_u64().unwrap_or(0) as u8;
            // Sin EXE o con fallo del enlazador, el cliente sigue con su maqueta propia: no es un error de la sesión.
            Ok(s.ui_text(&text, width, color, font).unwrap_or_else(|e| json!({"error": e})))
        }
        "screen" => s.screen(v["name"].as_str().ok_or("falta name")?),
        "screen_action" => s.screen_action(v["name"].as_str().ok_or("falta name")?, v["action"].as_str().ok_or("falta action")?, &v["args"]),
        "build_check" => {
            let (ty, row, col) = build_args(v)?;
            s.build_check(ty, row, col)
        }
        "build" => {
            let (ty, row, col) = build_args(v)?;
            s.build(ty, row, col)
        }
        "save" => {
            let path = v["path"]
                .as_str()
                .filter(|s| !s.is_empty())
                .ok_or("ruta de salida ausente")?;
            let data = s.save()?;
            let mut f = std::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(path)
                .map_err(|e| format!("no se ha creado la partida: {e}"))?;
            f.write_all(&data).map_err(|e| format!("escritura incompleta: {e}"))?;
            f.sync_all()
                .map_err(|e| format!("no se ha confirmado la escritura: {e}"))?;
            Ok(json!({"bytes":data.len()}))
        }
        _ => Err("comando desconocido".into()),
    }
}
