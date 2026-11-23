//! Reproducible UI runtime benchmark. Reads commercial resources locally only.
use p3_mvp::ui_rt::{menus, win32, PageMem, Runtime};
use std::{cell::RefCell, rc::Rc, time::Instant};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let scenario = args.get(1).map(String::as_str).unwrap_or("menu");
    let frames: u32 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(120);
    let exe = std::fs::read(std::env::var_os("P3_EXE").expect("P3_EXE required")).unwrap();
    let cpr = std::fs::read(std::env::var_os("P3_CPR").expect("P3_CPR required")).unwrap();
    let archive = p3_assets::cpr::Archive::parse(&cpr).unwrap();
    let mut mem = PageMem::default();
    let mut rt = Runtime::new(&mut mem, &exe, &archive, 800, 600).unwrap();
    rt.call_limit = 1_000_000_000;
    if let Some(pack) = std::env::var_os("P3_BENCH_PACK").and_then(|p| std::fs::read_to_string(p).ok()) {
        // Paquete de cadenas del EXE (`idioma/<código>.pack`), como `p3-juego --idioma`.
        let (n, skipped, tables) = rt.load_language_pack(&mut mem, &pack);
        eprintln!("idioma: {n} cadenas ({skipped} descartadas), {tables} tablas");
    }
    // `P3_GAME_DIR`: carpeta del juego donde buscar `Videos\*.sks` (vídeos de las ventanas de suceso, `ui_rt/vto.rs`).
    if let Some(d) = std::env::var_os("P3_GAME_DIR") {
        rt.vto.dirs = vec![d.into()];
    }
    assert!(menus::boot(&mut rt, &mut mem));
    let modern = args.get(3).map(|s| {
        let (w,h)=s.split_once('x').expect("resolution WIDTHxHEIGHT");
        (w.parse::<u32>().unwrap(),h.parse::<u32>().unwrap())
    });
    rt.display.modern = modern;
    let records = Rc::new(RefCell::new(Vec::new()));
    let output = records.clone();
    let mut last = Instant::now();
    let mut frame = 0u32;
    let city = scenario == "city" || scenario == "zoom" || scenario == "pan" || scenario == "panv" || scenario == "wheel" || scenario == "edge";
    let pan = scenario == "pan";
    let panv = scenario == "panv";
    let wheel = scenario == "wheel";
    let edge = scenario == "edge";
    let zout: u32 = std::env::var("P3_BENCH_ZOOMOUT").ok().and_then(|v| v.parse().ok()).unwrap_or(0);
    rt.zoom.enabled = wheel || std::env::var_os("P3_BENCH_WHEELSEQ").is_some() || zout > 0 || std::env::var_os("P3_BENCH_ZOOM").is_some();
    rt.on_present = Some(Box::new(move |surface, queue, input| {
        frame += 1;
        let now = Instant::now();
        // Hash every pixel: before/after timings must retain identical output.
        let hash = surface.rgba.iter().fold(0xcbf29ce484222325u64, |h, b| (h ^ u64::from(*b)).wrapping_mul(0x100000001b3));
        output.borrow_mut().push(serde_json::json!({"frame":frame,"width":surface.width,"height":surface.height,"ms":now.duration_since(last).as_secs_f64()*1000.0,"pixels":format!("{hash:016x}")}));
        if let Some(dir) = std::env::var_os("P3_BENCH_DUMP") {
            if frame >= std::env::var("P3_BENCH_FROM").ok().and_then(|v| v.parse().ok()).unwrap_or(40) && frame % std::env::var("P3_BENCH_EVERY").ok().and_then(|v| v.parse().ok()).unwrap_or(1) == 0 && frame <= std::env::var("P3_BENCH_LAST").ok().and_then(|v| v.parse().ok()).unwrap_or(134) {
                let _ = std::fs::write(std::path::Path::new(&dir).join(format!("f{frame}.rgba")), &surface.rgba);
            }
        }
        input.time_ms = Some(frame * std::env::var("P3_BENCH_MS").ok().and_then(|v| v.parse().ok()).unwrap_or(40));
        if city {
            let (x,y) = (685,561);
            if frame == 10 { input.cursor=(x,y); win32::post(queue,input,win32::WM_MOUSEMOVE,0,win32::mouse_lparam(x,y)); }
            if frame == 11 || frame == 12 {
                let down=frame==11;
                input.set_key(win32::VK_LBUTTON, down);
                win32::post(queue,input,if down {win32::WM_LBUTTONDOWN} else {win32::WM_LBUTTONUP},input.mouse_keys(),win32::mouse_lparam(x,y));
            }
        }
        if pan && (frame == 40 || frame == 70) {
            let down=frame==40;
            input.set_key(win32::VK_RIGHT,down);
            win32::post(queue,input,if down {win32::WM_KEYDOWN} else {win32::WM_KEYUP},u32::from(win32::VK_RIGHT),0);
        }
        if zout > 0 && frame >= 15 && frame < 15 + 3 * zout && (frame - 15) % 3 == 0 {
            let (x, y) = (400, 300);
            input.cursor = (x, y);
            win32::post(queue, input, win32::WM_MOUSEWHEEL, input.mouse_keys() | ((-120i32 as i16 as u16 as u32) << 16), win32::mouse_lparam(x, y));
        }
        if wheel && frame >= 40 && frame < 200 && (frame - 40) % 4 == 0 {
            // Rueda rapida: 8 pasos alejando y 8 acercando, repetido.
            let n = ((frame - 40) / 4) % 16;
            let d: i32 = if n < 8 { -120 } else { 120 };
            let (x, y) = (400, 300);
            input.cursor = (x, y);
            win32::post(queue, input, win32::WM_MOUSEWHEEL, input.mouse_keys() | ((d as i16 as u16 as u32) << 16), win32::mouse_lparam(x, y));
        }
        if let Ok(spec) = std::env::var("P3_BENCH_WHEELSEQ") {
            // "F:delta:x:y;...": un giro de rueda (delta en 1/120) en el fotograma F con el cursor en (x, y).
            for c in spec.split(';').filter(|c| !c.is_empty()) {
                let v: Vec<i32> = c.split(':').filter_map(|n| n.parse().ok()).collect();
                if v.len() == 4 && frame as i32 == v[0] {
                    input.cursor = (v[2] as _, v[3] as _);
                    win32::post(queue, input, win32::WM_MOUSEMOVE, 0, win32::mouse_lparam(v[2] as _, v[3] as _));
                    win32::post(queue, input, win32::WM_MOUSEWHEEL, input.mouse_keys() | ((v[1] as i16 as u16 as u32) << 16), win32::mouse_lparam(v[2] as _, v[3] as _));
                }
            }
        }
        if wheel {
            for (f, vk, down) in [(205, win32::VK_DOWN, true), (240, win32::VK_DOWN, false), (241, win32::VK_RIGHT, true), (275, win32::VK_RIGHT, false)] {
                if frame == f {
                    input.set_key(vk, down);
                    win32::post(queue,input,if down {win32::WM_KEYDOWN} else {win32::WM_KEYUP},u32::from(vk),0);
                }
            }
        }
        if let Ok(spec) = std::env::var("P3_BENCH_CLICKS") {
            // "F:x:y;..." clic izquierdo en el fotograma F (mueve, pulsa en F+1, suelta en F+2).
            for c in spec.split(';').filter(|c| !c.is_empty()) {
                let v: Vec<i32> = c.split(':').filter_map(|n| n.parse().ok()).collect();
                // "F:x:y:r" usa el botón derecho.
                let right = c.ends_with(":r");
                if v.len() == 3 && frame as i32 >= v[0] && frame as i32 <= v[0] + 2 {
                    let (x, y) = (v[1], v[2]);
                    if frame as i32 == v[0] { input.cursor = (x as _, y as _); win32::post(queue, input, win32::WM_MOUSEMOVE, 0, win32::mouse_lparam(x as _, y as _)); }
                    else {
                        let down = frame as i32 == v[0] + 1;
                        let (vk, dn, up) = if right { (win32::VK_RBUTTON, win32::WM_RBUTTONDOWN, win32::WM_RBUTTONUP) } else { (win32::VK_LBUTTON, win32::WM_LBUTTONDOWN, win32::WM_LBUTTONUP) };
                        input.set_key(vk, down);
                        win32::post(queue, input, if down { dn } else { up }, input.mouse_keys(), win32::mouse_lparam(x as _, y as _));
                    }
                }
            }
        }
        if let Ok(spec) = std::env::var("P3_BENCH_KEYS") {
            // "F:VK[:N];..." pulsa la tecla virtual VK (decimal) en el fotograma F y la suelta en F+N (1 por omisión) (p. ej. 40:123 = F12, avance rápido).
            for c in spec.split(';').filter(|c| !c.is_empty()) {
                let v: Vec<u32> = c.split(':').filter_map(|n| n.parse().ok()).collect();
                // "F:VK:N" la mantiene N fotogramas.
                let hold = v.get(2).copied().unwrap_or(1).max(1);
                if v.len() >= 2 && (frame == v[0] || frame == v[0] + hold) {
                    let down = frame == v[0];
                    input.set_key(v[1] as _, down);
                    win32::post(queue, input, if down { win32::WM_KEYDOWN } else { win32::WM_KEYUP }, v[1], 0);
                }
            }
        }
        if let Ok(spec) = std::env::var("P3_BENCH_DRAG") {
            // "F:x0:y0:x1:y1": pulsa en F, arrastra linealmente 30 fotogramas y suelta; boton `P3_BENCH_DRAG_BTN=r` usa el derecho.
            let v: Vec<i32> = spec.split(':').filter_map(|n| n.parse().ok()).collect();
            if v.len() == 5 {
                let f = frame as i32;
                let right = std::env::var("P3_BENCH_DRAG_BTN").is_ok_and(|b| b == "r");
                let (vk, dn, up) = if right { (win32::VK_RBUTTON, win32::WM_RBUTTONDOWN, win32::WM_RBUTTONUP) } else { (win32::VK_LBUTTON, win32::WM_LBUTTONDOWN, win32::WM_LBUTTONUP) };
                if f >= v[0] && f <= v[0] + 31 {
                    let t = (f - v[0]).clamp(0, 30);
                    let (x, y) = (v[1] + (v[3] - v[1]) * t / 30, v[2] + (v[4] - v[2]) * t / 30);
                    input.cursor = (x as _, y as _);
                    if f == v[0] { win32::post(queue, input, win32::WM_MOUSEMOVE, 0, win32::mouse_lparam(x as _, y as _)); }
                    if f == v[0] + 1 { input.set_key(vk, true); win32::post(queue, input, dn, input.mouse_keys(), win32::mouse_lparam(x as _, y as _)); }
                    if f > v[0] + 1 && f < v[0] + 31 { win32::post(queue, input, win32::WM_MOUSEMOVE, input.mouse_keys(), win32::mouse_lparam(x as _, y as _)); }
                    if f == v[0] + 31 { input.set_key(vk, false); win32::post(queue, input, up, input.mouse_keys(), win32::mouse_lparam(x as _, y as _)); }
                }
            }
        }
        if std::env::var_os("P3_BENCH_WIGGLE").is_some() && frame >= 40 {
            // Cursor recorriendo la escena en elipse (sin tocar los bordes), un paso por fotograma.
            let t = (frame - 40) as f64 * 0.12;
            let (w, h) = (surface.width as f64, surface.height as f64);
            let (x, y) = ((w * 0.4 + w * 0.3 * t.sin()) as i32, (h * 0.5 + h * 0.3 * (t * 0.7).cos()) as i32);
            input.cursor = (x as _, y as _);
            // `P3_BENCH_WIGGLE=N`: N movimientos por fotograma (un raton real manda varios entre dos presentaciones).
            let n: i32 = std::env::var("P3_BENCH_WIGGLE").ok().and_then(|v| v.parse().ok()).unwrap_or(1).max(1);
            for k in 1..n {
                let (xk, yk) = (x + k * 7, y - k * 5);
                input.cursor = (xk as _, yk as _);
                win32::post(queue, input, win32::WM_MOUSEMOVE, 0, win32::mouse_lparam(xk as _, yk as _));
            }
            win32::post(queue, input, win32::WM_MOUSEMOVE, 0, win32::mouse_lparam(x as _, y as _));
        }
        if edge {
            // Raton contra el borde inferior (40..160) y despues el superior (160..280).
            let (w, h) = (surface.width as i32, surface.height as i32);
            let pts: Vec<(i32, i32)> = std::env::var("P3_BENCH_PT").ok().map(|v| v.split(';').filter_map(|p| { let (a, b) = p.split_once(',')?; Some((a.parse::<i32>().ok()?, b.parse::<i32>().ok()?)) }).collect()).unwrap_or_else(|| vec![(500, 1000), (500, 0), (500, 500)]);
            let k = if frame >= 40 && (frame - 40) % 120 == 0 { Some(((frame - 40) / 120) as usize) } else { None };
            let target = k.and_then(|k| pts.get(k)).map(|&(fx, fy)| (((w - 1) * fx / 1000).min(w - 1), ((h - 1) * fy / 1000).min(h - 1)));
            // Zona de escena: la barra lateral (284 px) queda fuera.
            let target = target.map(|(x, y)| (x.min(w - 290), y));
            if let Some((x, y)) = target {
                input.cursor = (x as u32 as _, y as u32 as _);
                win32::post(queue, input, win32::WM_MOUSEMOVE, 0, win32::mouse_lparam(x as _, y as _));
            }
        }
        if panv {
            // Desplazamiento vertical rapido: abajo 40..100, arriba 100..160 (flechas).
            for (f, vk, down) in [(40, win32::VK_DOWN, true), (300, win32::VK_DOWN, false), (301, win32::VK_UP, true), (560, win32::VK_UP, false)] {
                if frame == f {
                    input.set_key(vk, down);
                    win32::post(queue,input,if down {win32::WM_KEYDOWN} else {win32::WM_KEYUP},u32::from(vk),0);
                }
            }
        }
        last = Instant::now();
        true
    }));
    if let Ok(spec) = std::env::var("P3_BENCH_CALL") {
        // "F:función:ecx[:arg...];..." (hex salvo F): llama al original al presentar el fotograma F (diagnóstico).
        let h = |x: &str| u32::from_str_radix(x.trim_start_matches("0x"), 16).unwrap_or(0);
        for c in spec.split(';').filter(|c| !c.is_empty()) {
            let v: Vec<&str> = c.split(':').collect();
            if v.len() >= 3 {
                rt.menus.frame_calls.push((v[0].parse().unwrap_or(0), h(v[1]), h(v[2]), v[3..].iter().map(|a| h(a)).collect()));
            }
        }
    }
    rt.menus.frame_script = Some(Box::new(move |m,f| {
        if let Ok(spec) = std::env::var("P3_BENCH_MEMDUMP") {
            // "F:addr:len:archivo;..." (hex) guarda memoria del invitado en el fotograma F; addr con * lee primero el puntero.
            for c in spec.split(';').filter(|c| !c.is_empty()) {
                let v: Vec<&str> = c.splitn(4, ':').collect();
                if v.len() == 4 && v[0].parse::<u32>().ok() == Some(f) {
                    let h = |x: &str| u32::from_str_radix(x.trim_start_matches('*').trim_start_matches("0x"), 16).unwrap_or(0);
                    let mut a = h(v[1]);
                    if v[1].starts_with('*') { a = m.r32(a); }
                    let bytes: Vec<u8> = (0..h(v[2])).map(|i| m.r8(a + i)).collect();
                    let _ = std::fs::write(v[3], &bytes);
                    eprintln!("volcado f{f} {a:#x}+{:#x} -> {}", bytes.len(), v[3]);
                }
            }
        }
        if city && f==3 { m.w32(menus::MAIN_MENU+0x284,2); }
        if let Ok(spec) = std::env::var("P3_BENCH_POKE") {
            // "F:addr:valor;..." escribe un dword en el fotograma F (diagnóstico, p. ej. 0x6DFBC4 = ms por tick).
            for c in spec.split(';') {
                let v: Vec<&str> = c.split(':').collect();
                if v.len() == 3 && v[0].parse::<u32>().ok() == Some(f) {
                    let h = |x: &str| u32::from_str_radix(x.trim_start_matches("0x"), 16).unwrap_or(0);
                    m.w32(h(v[1]), h(v[2]));
                }
            }
        }
        if let Ok(spec) = std::env::var("P3_BENCH_PEEK") {
            // "F:addr" imprime el puntero en addr y las 80 primeras ranuras de su vtable (diagnóstico).
            if let Some((ff, a)) = spec.split_once(':') {
                if ff.parse::<u32>().ok() == Some(f) {
                    let a = u32::from_str_radix(a.trim_start_matches("0x"), 16).unwrap_or(0);
                    let o = m.r32(a);
                    let vt = m.r32(o);
                    let slots: Vec<String> = (0..80u32).map(|i| format!("{:x}:{:x}", i * 4, m.r32(vt + 4 * i))).collect();
                    eprintln!("peek {a:#x} -> {o:#x} vt {vt:#x} {}", slots.join(" "));
                }
            }
        }
        if std::env::var_os("P3_BENCH_PROBE").is_some() && (f == 30 || f == 60) {
            let scene = m.r32(0x006E_51AC);
            if scene != 0 {
                let b = m.r32(scene + 0x530);
                eprintln!("sonda f{f}: escena geom {}x{} pos {},{}; suelo vt {:#x} +0xc={} +0x10={} +0x14={} +0x18={}", m.r32(scene + 0x2C), m.r32(scene + 0x30), m.r32(scene + 0x14), m.r32(scene + 0x18), m.r32(b), m.r32(b + 0xc), m.r32(b + 0x10), m.r32(b + 0x14), m.r32(b + 0x18));
            }
        }
        f < frames
    }));
    let started=Instant::now();
    menus::run(&mut rt,&mut mem);
    let mut counts: Vec<_> = rt.ported.iter().map(|(&f,&n)| (n,format!("{f:08x}"))).collect();
    counts.sort_by(|a,b| b.cmp(a));
    let mut profile: Vec<_> = rt.profile.iter().flat_map(|p| p.iter()).map(|(&f,&(calls,own,total))| serde_json::json!({"address":format!("{f:08x}"),"calls":calls,"self_ms":own as f64/1e6,"total_ms":total as f64/1e6})).collect();
    profile.sort_by(|a,b| b["self_ms"].as_f64().unwrap().total_cmp(&a["self_ms"].as_f64().unwrap()));
    println!("{}",serde_json::json!({"scenario":scenario,"modern":modern,"seconds":started.elapsed().as_secs_f64(),"frames":records.borrow().as_slice(),"counts":counts,"profile":profile,"missing":format!("{:x?}",rt.missing_from),"towns":mem.u32_at(0x6de4b0),"memory":format!("{:016x}",mem.content_hash()),"memory_no_stack":format!("{:016x}",mem.content_hash_outside(p3_mvp::ui_rt::STACK_TOP-0x0100_0000..p3_mvp::ui_rt::STACK_TOP+0x1000)),"calls":rt.calls}));
}
