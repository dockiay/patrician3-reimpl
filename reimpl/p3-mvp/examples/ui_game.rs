//! Arranca el flujo original del juego en el enlazador `ui_rt` (diagnóstico del hilo «menú principal y barra lateral»).
//! Uso: `ui_game Patrician3.exe p2arch0_eng.cpr salida.png PASO...`; pasos: `ctor`, `init`, `run`, `menu`, o `DIR[:args]` con ecx=0x6CC3F0.
fn hex(s: &str) -> u32 {
    u32::from_str_radix(s.trim_start_matches("0x"), 16).expect("hex")
}
fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let exe = std::fs::read(&args[0]).map_err(|e| e.to_string())?;
    let cpr = std::fs::read(&args[1]).map_err(|e| e.to_string())?;
    let archive = p3_assets::cpr::Archive::parse(&cpr).map_err(|e| format!("{e:?}"))?;
    let mut mem = p3_mvp::ui_rt::PageMem::default();
    let mut rt = p3_mvp::ui_rt::Runtime::new(&mut mem, &exe, &archive, 800, 600)?;
    rt.surface = p3_mvp::ui_rt::Surface::new(800, 600, [0x30, 0x30, 0x30, 0xFF]);
    if let Some(n) = std::env::var("P3_CALLS").ok().and_then(|v| v.parse().ok()) {
        rt.call_limit = n;
    }
    // `P3_ZOOM=1`: activa el zoom de la ciudad como el cliente nativo (`p3-client`).
    rt.zoom.enabled = std::env::var_os("P3_ZOOM").is_some();
    const APP: u32 = 0x006C_C3F0;
    // `P3_MODERN=ANCHOxALTO`: partida a resolución moderna (`RESOLUCIONES_MODERNAS.md`).
    if let Ok(v) = std::env::var("P3_MODERN") {
        let (w, h) = v.split_once('x').expect("ANCHOxALTO");
        rt.display.modern = Some((w.parse().expect("ancho"), h.parse().expect("alto")));
    }
    // `P3_GAME_DIR`: carpeta del juego con `save/ein/*.pat` (partidas de la pantalla de carga).
    rt.menus.game_dir = std::env::var_os("P3_GAME_DIR").map(Into::into);
    // `P3_SCRIPT="F:DIR=VALOR;F:*BASE+OFF=VALOR;F:quit;F:stop"`: en el fotograma F escribe una palabra (hex), pide salir (`0x6DA846` = 1,
    // como `WM_QUIT`) o corta la ejecución.
    if let Ok(spec) = std::env::var("P3_SCRIPT") {
        let steps: Vec<(u32, String)> = spec.split(';').filter(|s| !s.is_empty()).map(|s| {
            let (f, a) = s.split_once(':').expect("F:acción");
            (f.parse().expect("fotograma"), a.to_string())
        }).collect();
        rt.menus.frame_script = Some(Box::new(move |m, frame| {
            let mut keep = true;
            for (f, a) in &steps {
                if *f != frame {
                    continue;
                }
                eprintln!("guion: fotograma {frame}: {a}");
                if a == "stop" {
                    keep = false;
                } else if a == "quit" {
                    m.w8(0x006D_A846, 1);
                } else if let Some((addr, v)) = a.split_once('=') {
                    // `*BASE+OFF`: palabra en `[BASE] + OFF` (p. ej. `*6cbb7c+11c4` en la pantalla de carga).
                    let at = match addr.strip_prefix('*').and_then(|r| r.split_once('+')) {
                        Some((base, off)) => m.r32(hex(base)).wrapping_add(hex(off)),
                        None => hex(addr),
                    };
                    m.w32(at, hex(v));
                }
            }
            keep
        }));
    }
    // `P3_INPUT="F:click=X,Y;F:rclick=X,Y;F:move=X,Y;F:key=VK;F:shot=ruta.png"`: entrada en el fotograma F (contado por `sgl_SwapBuffer`).
    // Un clic mueve el ratón en F y pulsa/suelta en F+1/F+2, como la ventana nativa (`p3-client`); `shot` guarda la pantalla.
    if let Ok(spec) = std::env::var("P3_INPUT") {
        use p3_mvp::ui_rt::win32;
        let mut events: Vec<(u32, u32, u32, u32)> = Vec::new(); // (fotograma, tipo, a, b); tipo 0 mover, 1/2 izq. abajo/arriba, 3/4 der., 5/6 tecla
        let mut shots: Vec<(u32, String)> = Vec::new();
        for s in spec.split(';').filter(|s| !s.is_empty()) {
            let (f, a) = s.split_once(':').expect("F:acción");
            let f: u32 = f.parse().expect("fotograma");
            let (k, v) = a.split_once('=').expect("acción=valor");
            let xy = || {
                let (x, y) = v.split_once(',').expect("X,Y");
                (x.parse::<u32>().expect("x"), y.parse::<u32>().expect("y"))
            };
            match k {
                "move" => events.push((f, 0, xy().0, xy().1)),
                "click" => events.extend([(f, 0, xy().0, xy().1), (f + 1, 1, 0, 0), (f + 2, 2, 0, 0)]),
                "rclick" => events.extend([(f, 0, xy().0, xy().1), (f + 1, 3, 0, 0), (f + 2, 4, 0, 0)]),
                "key" => events.extend([(f, 5, hex(v), 0), (f + 1, 6, hex(v), 0)]),
                "shot" => shots.push((f, v.to_string())),
                _ => panic!("acción desconocida {k}"),
            }
        }
        let mut frame = 0u32;
        // Reloj de `timeGetTime`: `P3_FRAME_MS` milisegundos por fotograma (40 por defecto), como la ventana nativa con su reloj
        // real; sin él las animaciones por tiempo (pergamino del menú lateral, `window_base44_update` 0x4415D0) no avanzan.
        let frame_ms: u32 = std::env::var("P3_FRAME_MS").ok().and_then(|v| v.parse().ok()).unwrap_or(40);
        rt.on_present = Some(Box::new(move |surface, queue, input| {
            frame += 1;
            input.time_ms = Some(frame * frame_ms);
            for &(f, kind, a, b) in events.iter().filter(|e| e.0 == frame) {
                let _ = f;
                let mouse = |queue: &mut std::collections::VecDeque<win32::Msg>, input: &win32::InputState, msg: u32| {
                    let (x, y) = input.cursor;
                    win32::post(queue, input, msg, input.mouse_keys(), win32::mouse_lparam(x, y));
                };
                match kind {
                    0 => {
                        input.cursor = (a as i32, b as i32);
                        mouse(queue, input, win32::WM_MOUSEMOVE);
                    }
                    1 | 2 => {
                        input.set_key(win32::VK_LBUTTON, kind == 1);
                        mouse(queue, input, if kind == 1 { win32::WM_LBUTTONDOWN } else { win32::WM_LBUTTONUP });
                    }
                    3 | 4 => {
                        input.set_key(win32::VK_RBUTTON, kind == 3);
                        mouse(queue, input, if kind == 3 { win32::WM_RBUTTONDOWN } else { win32::WM_RBUTTONUP });
                    }
                    _ => {
                        input.set_key(a as u8, kind == 5);
                        win32::post(queue, input, if kind == 5 { win32::WM_KEYDOWN } else { win32::WM_KEYUP }, a, 1);
                    }
                }
                eprintln!("entrada: fotograma {frame}: tipo {kind} {a},{b}");
            }
            for (_, path) in shots.iter().filter(|s| s.0 == frame) {
                let _ = surface.write_png(std::path::Path::new(path));
                eprintln!("entrada: fotograma {frame}: captura {path}");
            }
            true
        }));
    }
    for step in &args[3..] {
        let before = rt.missing_from.clone();
        let calls0 = rt.calls;
        let r = match step.as_str() {
            "ctor" => rt.call(&mut mem, 0x0046_3180, APP, &[]),
            "init" => rt.call(&mut mem, 0x0046_3240, APP, &[]),
            "run" => p3_mvp::ui_rt::menus::run(&mut rt, &mut mem),
            "boot" => p3_mvp::ui_rt::menus::boot(&mut rt, &mut mem) as u32,
            "scene" => rt.create_scene(&mut mem),
            // `dump:DIR,N`: palabras de memoria
            s if s.starts_with("dump:") => {
                let (a, n) = s[5..].split_once(',').unwrap();
                let a = hex(a);
                let w: Vec<String> = (0..n.parse::<u32>().unwrap()).map(|i| format!("{:x}", mem.u32_at(a + 4 * i))).collect();
                println!("{a:#x}: {}", w.join(" "));
                0
            }
            // `routes`: tabla ciudad→ciudad del planificador `[0x6CBDC8]` (+0x10 tabla, descriptores `búfer, n`), una línea por par.
            "routes" => {
                let planner = mem.u32_at(0x006C_BDC8);
                let towns = mem.u32_at(0x006D_E4B0);
                let table = mem.u32_at(planner + 0x10);
                println!("planificador {planner:#x} ciudades {towns} tabla {table:#x}");
                for k in 0..towns * towns {
                    let desc = mem.u32_at(table + 4 * k);
                    if desc == 0 {
                        continue;
                    }
                    let (buf, n) = (mem.u32_at(desc), mem.u32_at(desc + 4));
                    let pts: Vec<String> = (0..n.min(64))
                        .map(|q| {
                            let v = mem.u32_at(buf + 4 * q);
                            format!("{},{}", v as u16 as i16, (v >> 16) as u16 as i16)
                        })
                        .collect();
                    println!("ruta {}->{} n={n}: {}", k / towns, k % towns, pts.join(" "));
                }
                0
            }
            // `routes_native`: la misma tabla con el porte nativo (`p3_core::route`, paridad x86) a partir del CPR y de las casillas
            // de ciudad en memoria; cuenta los pares distintos y compara rejilla, nodos y matriz cargados en memoria.
            "routes_native" => {
                use p3_core::route::{finder::RoutePlanner, planner::{load_nav_data, NavFiles, TownSlot}};
                struct Files(Vec<Vec<u8>>);
                impl NavFiles for Files {
                    fn open(&mut self, i: usize) -> Option<Vec<u8>> { self.0.get(i).cloned() }
                }
                let read = |n: &str| archive.read(archive.find(n.as_bytes()).unwrap()).unwrap().to_vec();
                let files = vec![read("navdata/nav_matrix.dat"), read("navdata/nav_vec.dat"), read("navdata/matrix_int.dat")];
                let nav = load_nav_data(&mut Files(files), mem.u32_at(0x0066_E0D0) as i32).unwrap();
                let planner = mem.u32_at(0x006C_BDC8);
                // Rejilla, nodos y matriz en memoria frente al CPR.
                let grid = mem.u32_at(planner + 0x14);
                let (w, h) = (mem.u32_at(planner + 0x18) as u16 as i16, (mem.u32_at(planner + 0x18) >> 16) as u16 as i16);
                let cnt = mem.u32_at(planner + 0x1C) as u16 as i16;
                println!("memoria: rejilla {w}x{h} nodos {cnt}; CPR {}x{} nodos {} escala {}", nav.grid.width, nav.grid.height, nav.node_count, nav.grid.scale);
                let bad_cells = (0..nav.grid.cells.len() as u32).filter(|&i| (mem.u32_at(grid + i) as u8) != nav.grid.cells[i as usize]).count();
                let nodes_at = mem.u32_at(planner + 0x0C);
                let bad_nodes = (0..nav.nodes.len() as u32).filter(|&k| { let v = mem.u32_at(nodes_at + 4 * k); (v as u16 as i16, (v >> 16) as u16 as i16) != nav.nodes[k as usize] }).count();
                let rows_at = mem.u32_at(planner + 0x04);
                let n = nav.nodes.len() as u32;
                let mut bad_matrix = 0;
                for a in 0..n {
                    let row = mem.u32_at(rows_at + 4 * a);
                    for b in 0..n {
                        let (cost, next) = (mem.u32_at(row + 8 * b) as i32, mem.u32_at(row + 8 * b + 4) as u16 as i16);
                        if (cost, next) != nav.matrix[a as usize][b as usize] { bad_matrix += 1; }
                    }
                }
                println!("celdas distintas {bad_cells}, nodos distintos {bad_nodes}, ranuras de matriz distintas {bad_matrix}");
                let towns = mem.u32_at(0x006D_E4B0);
                let slots: Vec<TownSlot> = (0..40u32).map(|k| { let s = 0x006D_DB90 + 52 * k; TownSlot { x: mem.u32_at(s) as i32, y: mem.u32_at(s + 4) as i32, anchor_x: mem.u32_at(s + 0x20) as i32, anchor_y: mem.u32_at(s + 0x24) as i32 } }).collect();
                let order: Vec<u8> = (0..towns).map(|k| mem.u32_at(0x006D_E4B8 + k) as u8).collect();
                let mut native = RoutePlanner::new(nav, slots, order, towns, vec![], vec![]);
                println!("tabla nativa: {}", native.build_town_route_table(200));
                let table = mem.u32_at(planner + 0x10);
                let mut diff = 0;
                for k in 0..towns * towns {
                    let desc = mem.u32_at(table + 4 * k);
                    let got: Vec<(i16, i16)> = if desc == 0 { vec![] } else {
                        let (buf, c) = (mem.u32_at(desc), mem.u32_at(desc + 4));
                        (0..c).map(|q| { let v = mem.u32_at(buf + 4 * q); (v as u16 as i16, (v >> 16) as u16 as i16) }).collect()
                    };
                    let want = native.town_table[k as usize].map(|i| native.routes[i].clone()).unwrap_or_default();
                    // El primer punto lo reescribe cada consulta: se compara el resto.
                    if got.get(1..) != want.get(1..) {
                        diff += 1;
                        if diff <= 3 { println!("par {}->{}: memoria {got:?}\n  nativo {want:?}", k / towns, k % towns); }
                    }
                }
                println!("pares distintos: {diff} de {}", towns * towns);
                0
            }
            // `route_diff:N`: N pares de puntos de mar al azar; compara `is_line_clear` (0x4445A0) y `build_route_from_points`
            // (0x445D50) del enlazador con el porte nativo (paridad x86) sobre los datos cargados.
            s if s.starts_with("route_diff:") => {
                use p3_core::route::{finder::{RoutePlanner, RouteOut}, planner::{load_nav_data, NavFiles}};
                struct Files(Vec<Vec<u8>>);
                impl NavFiles for Files {
                    fn open(&mut self, i: usize) -> Option<Vec<u8>> { self.0.get(i).cloned() }
                }
                let read = |n: &str| archive.read(archive.find(n.as_bytes()).unwrap()).unwrap().to_vec();
                let files = vec![read("navdata/nav_matrix.dat"), read("navdata/nav_vec.dat"), read("navdata/matrix_int.dat")];
                let nav = load_nav_data(&mut Files(files), mem.u32_at(0x0066_E0D0) as i32).unwrap();
                let mut native = RoutePlanner::new(nav, vec![], vec![], 0, vec![], vec![]);
                let planner = mem.u32_at(0x006C_BDC8);
                let grid = mem.u32_at(planner + 0x14);
                let (w, h) = (native.nav.grid.width as u32, native.nav.grid.height as u32);
                let n: u32 = s[11..].parse().unwrap();
                rt.calls = 0;
                rt.call_limit = 1 << 40;
                rt.aborted = false;
                let scratch = rt.cstring(&mut mem, &[0x41u8; 0x100]);
                let mut seed = 0x1234_5678u32;
                let mut rnd = |m: u32| { seed = seed.wrapping_mul(1_103_515_245).wrapping_add(12345); (seed >> 8) % m };
                let (mut line_bad, mut route_bad, mut tried) = (0, 0, 0);
                while tried < n {
                    let a = (rnd(w) as i16, rnd(h) as i16);
                    let b = (rnd(w) as i16, rnd(h) as i16);
                    let sea = |p: (i16, i16)| native.nav.grid.cells[(p.1 as u32 * w + p.0 as u32) as usize] == 0;
                    if !sea(a) || !sea(b) { continue; }
                    tried += 1;
                    let pack = |p: (i16, i16)| p.0 as u16 as u32 | (p.1 as u16 as u32) << 16;
                    mem.set_u32(scratch, pack(a));
                    mem.set_u32(scratch + 4, pack(b));
                    mem.set_u32(scratch + 0x40, w);
                    let lifted = rt.call(&mut mem, 0x0044_45A0, 0, &[grid, scratch + 0x40, scratch, scratch + 4]) as u8 != 0;
                    let mut ctr = 0;
                    let nat = native.nav.grid.is_line_clear(b, a, &mut ctr);
                    if lifted != nat { line_bad += 1; if line_bad <= 5 { println!("línea {a:?}->{b:?}: enlazador {lifted} nativo {nat}"); } }
                    // Ruta entre los puntos del mundo de esas celdas.
                    let world = |p: (i16, i16)| (((p.0 as i32 * 0xdd0 + 500) / 1000 + 0x44) as i16, ((p.1 as i32 * 0xdd0 + 500) / 1000 + 0x44) as i16);
                    let (wa, wb) = (world(a), world(b));
                    if wa == wb { continue; }
                    mem.set_u32(scratch + 0x10, pack(wa));
                    mem.set_u32(scratch + 0x14, pack(wb));
                    mem.set_u32(scratch + 0x20, scratch + 0x10);
                    mem.set_u32(scratch + 0x24, 2);
                    mem.set_u32(scratch + 0x30, 0xDEAD_BEEF);
                    rt.call(&mut mem, 0x0044_5D50, planner, &[scratch + 0x20, scratch + 0x30]);
                    let out = mem.u32_at(scratch + 0x30);
                    let got: Option<Vec<(i16, i16)>> = (out != 0xDEAD_BEEF).then(|| {
                        let (buf, c) = (mem.u32_at(out), mem.u32_at(out + 4));
                        (0..c).map(|q| { let v = mem.u32_at(buf + 4 * q); (v as u16 as i16, (v >> 16) as u16 as i16) }).collect()
                    });
                    let want = match native.build_route_from_points(&[wa, wb]).route { RouteOut::Built(p) => Some(p), _ => None };
                    if got != want { route_bad += 1; if route_bad <= 5 { println!("ruta {wa:?}->{wb:?}:\n  enlazador {got:?}\n  nativo    {want:?}"); } }
                }
                println!("pares {tried}: líneas distintas {line_bad}, rutas distintas {route_bad}");
                0
            }
            // `frame`: un fotograma de la lista de pantallas (`screen_list_run_frame`)
            "frame" => rt.call(&mut mem, 0x004B_8A40, 0x006D_A5F0, &[]),
            // `menu`: abre el menú principal como `game_main_window_run` (init con options.ini, reset del juego y del menú)
            "menu" => {
                let ini = rt.cstring(&mut mem, b"./scripts/options.ini");
                rt.call(&mut mem, 0x0042_ECC0, 0x006C_C434, &[ini, 0]);
                rt.call(&mut mem, 0x0054_A310, 0x006D_F2F0, &[]);
                rt.call(&mut mem, 0x0042_F1F0, 0x006C_C434, &[])
            }
            s => {
                let (f, a) = s.split_once(':').unwrap_or((s, ""));
                let a: Vec<u32> = a.split(',').filter(|x| !x.is_empty()).map(hex).collect();
                rt.call(&mut mem, hex(f), APP, &a)
            }
        };
        let new: Vec<_> = rt.missing_from.iter().filter(|(k, v)| before.get(k) != Some(v)).map(|(k, v)| format!("{:x}<-{:x}:{}", k.0, k.1, v)).collect();
        println!("{step} -> {r:#x}  llamadas {} abortado {}  faltan nuevas: {}", rt.calls - calls0, rt.aborted, new.join(" "));
    }
    rt.surface.write_png(std::path::Path::new(&args[2]))?;
    // `P3_ARGLOG=fichero`: llamadas virtuales con argumentos mal deducidos (entrada de `tools/ui_rt_fix.py`).
    if let Some(path) = std::env::var_os("P3_ARGLOG") {
        use std::io::Write;
        let mut f = std::fs::OpenOptions::new().create(true).append(true).open(path).map_err(|e| e.to_string())?;
        for ((caller, ret), (real, deduced)) in &rt.arg_mismatch {
            writeln!(f, "{caller:#x} {ret:#x} {real} {deduced}").map_err(|e| e.to_string())?;
        }
    }
    println!("SGL sin sustituto: {:?}", rt.sgl_missing);
    println!("importaciones sin sustituto: {:?}", rt.import_missing);
    Ok(())
}
