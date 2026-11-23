//! Sonda de tirones (19/11/2026): carga una partida en el enlazador como el cliente (`AUTO.pat` de `save\ein`) y corre N
//! fotogramas con el reloj avanzando `P3_FRAME_MS` por fotograma. Escribe cada fotograma de más de `P3_SLOW_MS` (100) con su
//! tiempo, y el muestreador de [`p3_mvp::ui_rt::stall`] añade la línea «tirón: …» con las funciones del original que lo ocupan.
//!
//! `stall_probe EXE CPR PARTIDA.pat FOTOGRAMAS [ANCHOxALTO]`
//!
//! 19/11, Kam/AUTO de Sergio en la nube (main y remaster, 1920x1080): ciudad 14-15 ms, carta 1,3 ms, 6 días de juego, sonido,
//! ratón en movimiento y autoguardado (38 ms, `P3_CFG` con AUTOSAVE=2) sin ningún fotograma de más de 100 ms.
use p3_mvp::ui_rt::{menus, win32, PageMem, Runtime, Surface};
use std::time::Instant;

fn main() {
    let a: Vec<String> = std::env::args().collect();
    if a.len() < 5 {
        eprintln!("uso: stall_probe EXE CPR PARTIDA.pat FOTOGRAMAS [ANCHOxALTO]");
        std::process::exit(2);
    }
    let frames: u32 = a[4].parse().unwrap();
    let (w, h) = a.get(5).and_then(|s| s.split_once('x')).map_or((800, 600), |(w, h)| (w.parse().unwrap(), h.parse().unwrap()));
    let frame_ms: u32 = std::env::var("P3_FRAME_MS").ok().and_then(|v| v.parse().ok()).unwrap_or(20);
    let slow_ms: f64 = std::env::var("P3_SLOW_MS").ok().and_then(|v| v.parse().ok()).unwrap_or(100.0);
    let exe = std::fs::read(&a[1]).unwrap();
    let cpr = std::fs::read(&a[2]).unwrap();
    let archive = p3_assets::cpr::Archive::parse(&cpr).unwrap();
    let dir = std::env::temp_dir().join(format!("p3-stall-{}", std::process::id()));
    let saves = dir.join("save").join("ein");
    std::fs::create_dir_all(&saves).unwrap();
    std::fs::write(saves.join("AUTO.pat"), std::fs::read(&a[3]).unwrap()).unwrap();
    // `P3_CFG`: el `P2.CFG` del jugador (opciones: autoguardado, detalle, tiempo…).
    if let Some(cfg) = std::env::var_os("P3_CFG") {
        std::fs::copy(cfg, dir.join("P2.CFG")).unwrap();
    }
    let mut mem = PageMem::default();
    let mut rt = Runtime::new(&mut mem, &exe, &archive, w, h).unwrap();
    rt.surface = Surface::new(w, h, [0, 0, 0, 0xFF]);
    rt.call_limit = 2_000_000_000;
    rt.menus.game_dir = Some(dir.clone());
    rt.enable_stall_sampler();
    // `P3_AUDIO=1`: mezclador como el del cliente, con un hilo que lo consume a 48 kHz en tramos de 10 ms (sin dispositivo).
    if std::env::var_os("P3_AUDIO").is_some() {
        let mixer = p3_audio::live::Mixer::new(48_000);
        rt.miles.mixer = Some(mixer.clone());
        if let Some(d) = std::env::var_os("P3_GAME_DIR") {
            rt.miles.dirs = vec![d.into()];
        }
        std::thread::spawn(move || {
            let mut buf = vec![0f32; 960];
            loop {
                mixer.render(&mut buf);
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
        });
    }
    assert!(menus::boot(&mut rt, &mut mem));
    if (w, h) != (800, 600) {
        rt.display.modern = Some((w as u32, h as u32));
    }
    // Entrada opcional: `P3_WANDER=1` mueve el ratón por la pantalla en cada fotograma; `P3_KEYS="F:vk,..."` pulsa y suelta la
    // tecla (código virtual decimal) en el fotograma F; `P3_CLICKS="F:x:y,..."` clic izquierdo en (x, y) en el fotograma F.
    let wander = std::env::var_os("P3_WANDER").is_some();
    let keys: Vec<(u32, u8)> = std::env::var("P3_KEYS").unwrap_or_default().split(',').filter_map(|e| {
        let (f, k) = e.split_once(':')?;
        Some((f.trim().parse().ok()?, k.trim().parse().ok()?))
    }).collect();
    let clicks: Vec<(u32, i32, i32)> = std::env::var("P3_CLICKS").unwrap_or_default().split(',').filter_map(|e| {
        let mut p = e.split(':').map(|v| v.trim().parse::<i32>());
        Some((p.next()?.ok()? as u32, p.next()?.ok()?, p.next()?.ok()?))
    }).collect();
    rt.menus.frame_script = Some(Box::new(move |m, frame| {
        if frame == 3 {
            m.w32(0x006C_C6B8, 4);
        }
        if frame == 8 {
            let list = m.r32(0x006C_BB7C).wrapping_add(0x11C4);
            m.w32(list, 0);
        }
        if frame % 1000 == 0 {
            eprintln!("reloj del juego [0x6DCCF8] = {:#x} en el fotograma {frame}", m.r32(0x006D_CCF8));
        }
        frame < frames
    }));
    let mut frame = 0u32;
    let mut last = Instant::now();
    let start = Instant::now();
    let (mut total, mut slow, mut worst, mut slow_sum) = (0f64, 0u32, 0f64, 0f64);
    // `P3_PNG_AT="F,F"` y `P3_PNG_DIR`: captura de la pantalla en esos fotogramas.
    let png_at: Vec<u32> = std::env::var("P3_PNG_AT").unwrap_or_default().split(',').filter_map(|v| v.trim().parse().ok()).collect();
    let png_dir = std::env::var("P3_PNG_DIR").unwrap_or_else(|_| ".".into());
    rt.on_present = Some(Box::new(move |surface, queue, input| {
        frame += 1;
        if png_at.contains(&frame) {
            let _ = surface.write_png(&std::path::Path::new(&png_dir).join(format!("f{frame}.png")));
        }
        if wander && frame > 30 {
            let t = f64::from(frame) * 0.013;
            let (x, y) = (((t.sin() * 0.45 + 0.5) * w as f64) as i32, (((t * 1.7).cos() * 0.45 + 0.5) * h as f64) as i32);
            input.cursor = (x, y);
            win32::post(queue, input, win32::WM_MOUSEMOVE, input.mouse_keys(), win32::mouse_lparam(x, y));
        }
        for &(f, vk) in &keys {
            if f == frame || f + 2 == frame {
                let down = f == frame;
                input.set_key(vk, down);
                win32::post(queue, input, if down { win32::WM_KEYDOWN } else { win32::WM_KEYUP }, u32::from(vk), 0);
            }
        }
        for &(f, x, y) in &clicks {
            if f == frame {
                input.cursor = (x, y);
                win32::post(queue, input, win32::WM_MOUSEMOVE, input.mouse_keys(), win32::mouse_lparam(x, y));
            }
            if f + 1 == frame || f + 2 == frame {
                let down = f + 1 == frame;
                input.set_key(win32::VK_LBUTTON, down);
                win32::post(queue, input, if down { win32::WM_LBUTTONDOWN } else { win32::WM_LBUTTONUP }, input.mouse_keys(), win32::mouse_lparam(x, y));
            }
        }
        let now = Instant::now();
        let ms = now.duration_since(last).as_secs_f64() * 1000.0;
        last = now;
        if frame > 20 {
            total += ms;
            worst = worst.max(ms);
            if ms > slow_ms {
                slow += 1;
                slow_sum += ms;
                eprintln!("lento: fotograma {frame} {ms:.0} ms (reloj del juego {} s)", frame * frame_ms / 1000);
            }
            if frame % 1000 == 0 {
                let n = f64::from(frame - 20);
                eprintln!("resumen {frame}: media {:.1} ms, peor {worst:.0} ms, lentos {slow} ({slow_sum:.0} ms), {:.0} s reales", total / n, start.elapsed().as_secs_f64());
            }
        }
        input.time_ms = Some(frame * frame_ms);
        true
    }));
    menus::run(&mut rt, &mut mem);
    if !rt.miles.calls.is_empty() {
        eprintln!("Miles: {:?}", rt.miles.calls);
    }
    eprintln!("fin: abortado {} montón {:#x}", rt.aborted, rt.heap);
    let _ = std::fs::remove_dir_all(&dir);
}
