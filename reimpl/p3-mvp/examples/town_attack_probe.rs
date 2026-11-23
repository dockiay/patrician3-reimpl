//! Diagnóstico del ataque de barcos propios a una ciudad en el cliente (hilo «Probar combates navales contra ciudades», 22/11/2026).
//! Carga la primera partida de `P3_TEST_GAME_DIR/save/ein` por el menú real (pensado para `LABATTACK.pat`: el barco 109 del
//! humano llega a Edimburgo con la marca de asalto `ship+0x15C`), deja correr el reloj y sigue el combate de ataque a ciudad
//! (`TownAttackBattle`, tabla de combates `0x6E55D0`, ranura vista `0x6E59CC`).
//! Uso: `P3_EXE=.. P3_CPR=.. P3_TEST_GAME_DIR=dir town_attack_probe`. Opcionales: `P3_FRAMES` (600), `P3_SHIP` (109),
//! `P3_SHOTS=carpeta` y `P3_SHOT_EVERY=N` (capturas PNG), `P3_CLICK=F:x:y[;F:x:y…]` (clics izquierdos), `P3_RCLICK` (derechos, igual), `P3_FAST=F` (tiempo
//! rápido desde F), `P3_KEY=F:vk` (tecla).
use p3_mvp::ui_rt::{menus, PageMem, Runtime};

fn main() {
    let exe: &'static [u8] = Box::leak(std::fs::read(std::env::var_os("P3_EXE").unwrap()).unwrap().into_boxed_slice());
    let cpr: &'static [u8] = Box::leak(std::fs::read(std::env::var_os("P3_CPR").unwrap()).unwrap().into_boxed_slice());
    let archive = Box::leak(Box::new(p3_assets::cpr::Archive::parse(cpr).unwrap()));
    let mut mem = PageMem::default();
    let (w, hgt) = std::env::var("P3_SIZE").ok().and_then(|v| v.split_once('x').map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))).unwrap_or((1024, 768));
    let mut rt = Runtime::new(&mut mem, exe, archive, w, hgt).unwrap();
    rt.menus.game_dir = Some(std::env::var_os("P3_TEST_GAME_DIR").unwrap().into());
    rt.call_limit = 400_000_000;
    assert!(menus::boot(&mut rt, &mut mem));
    let frames: u32 = std::env::var("P3_FRAMES").ok().and_then(|v| v.parse().ok()).unwrap_or(600);
    let ship: u32 = std::env::var("P3_SHIP").ok().and_then(|v| v.parse().ok()).unwrap_or(109);
    let shots = std::env::var("P3_SHOTS").ok();
    let every: u32 = std::env::var("P3_SHOT_EVERY").ok().and_then(|v| v.parse().ok()).unwrap_or(50);
    let fast: Option<u32> = std::env::var("P3_FAST").ok().and_then(|v| v.parse().ok());
    let clicks: Vec<(u32, i32, i32)> = std::env::var("P3_CLICK").unwrap_or_default().split(';').filter(|s| !s.is_empty())
        .map(|s| { let v: Vec<i32> = s.split(':').map(|x| x.parse().unwrap()).collect(); (v[0] as u32, v[1], v[2]) }).collect();
    let rclicks: Vec<(u32, i32, i32)> = std::env::var("P3_RCLICK").unwrap_or_default().split(';').filter(|s| !s.is_empty())
        .map(|s| { let v: Vec<i32> = s.split(':').map(|x| x.parse().unwrap()).collect(); (v[0] as u32, v[1], v[2]) }).collect();
    let keys: Vec<(u32, u32)> = std::env::var("P3_KEY").unwrap_or_default().split(';').filter(|s| !s.is_empty())
        .map(|s| { let v: Vec<u32> = s.split(':').map(|x| x.parse().unwrap()).collect(); (v[0], v[1]) }).collect();
    let mut frame = 0u32;
    rt.on_present = Some(Box::new(move |_, queue, input| {
        frame += 1; input.time_ms = Some(frame * 40);
        use p3_mvp::ui_rt::win32;
        for &(f, x, y) in &clicks {
            if frame == f { input.cursor = (x as _, y as _); win32::post(queue, input, win32::WM_MOUSEMOVE, 0, win32::mouse_lparam(x as _, y as _)); }
            if frame == f + 1 || frame == f + 2 {
                let down = frame == f + 1;
                input.set_key(win32::VK_LBUTTON, down);
                win32::post(queue, input, if down { win32::WM_LBUTTONDOWN } else { win32::WM_LBUTTONUP }, input.mouse_keys(), win32::mouse_lparam(x as _, y as _));
                if down { println!("f{frame}: clic en ({x},{y})"); }
            }
        }
        for &(f, x, y) in &rclicks {
            if frame == f { input.cursor = (x as _, y as _); win32::post(queue, input, win32::WM_MOUSEMOVE, 0, win32::mouse_lparam(x as _, y as _)); }
            if frame == f + 1 || frame == f + 2 {
                let down = frame == f + 1;
                input.set_key(win32::VK_RBUTTON, down);
                win32::post(queue, input, if down { win32::WM_RBUTTONDOWN } else { win32::WM_RBUTTONUP }, input.mouse_keys(), win32::mouse_lparam(x as _, y as _));
                if down { println!("f{frame}: clic derecho en ({x},{y})"); }
            }
        }
        for &(f, vk) in &keys {
            if frame == f || frame == f + 1 {
                let down = frame == f;
                input.set_key(vk as _, down);
                win32::post(queue, input, if down { win32::WM_KEYDOWN } else { win32::WM_KEYUP }, vk, 1);
            }
        }
        true
    }));
    let rtp: *mut Runtime<'static> = &mut rt;
    let memp: *mut PageMem = &mut mem;
    let mut last = String::new();
    rt.menus.frame_script = Some(Box::new(move |m, frame| {
        if frame == 3 { m.w32(menus::MAIN_MENU + 0x284, 4); }
        if frame == 8 { let load = m.r32(0x006C_BB7C); m.w32(load + 0x11c4, 0); }
        if fast.is_some_and(|f| frame >= f) { m.w32(0x6DFC1C, 1); }
        let ships = m.r32(0x6DD7A4);
        let s = ships + ship * 0x180;
        let st = m.r32(s + 0x134) & 0xffff;
        let mut battles = Vec::new();
        for k in 0..8u32 {
            let b = m.r32(0x6E55D0 + 4 * k);
            if b != 0 { battles.push(format!("{k}:{b:x}/vt{:x}", m.r32(b))); }
        }
        let view = m.r8(0x6E59CC);
        let line = format!("barco {ship} st {st:x} pos ({},{}) 15c={:x} | combates [{}] vista {view:x} | t={:x} modo={:x}",
            m.r32(s + 0x1e) as i16, m.r32(s + 0x22) as i16, m.r8(s + 0x15c), battles.join(" "), m.r32(0x6de4b4), m.r32(0x6DFC1C));
        // Barcos locales del combate 0 (`+4`: tabla de punteros, `LocalMapShip` posición `+8/+0xC`) y proyectiles (`+0x668`).
        let b0 = m.r32(0x6E55D0);
        let mut locals = String::new();
        if b0 != 0 && m.r32(b0) == 0x67AAA0 {
            let tab = m.r32(b0 + 4);
            for k in 0..8u32 {
                let p = if tab != 0 { m.r32(tab + 4 * k) } else { 0 };
                if p != 0 { locals += &format!(" L{k}({:x},{:x})", m.r32(p + 8), m.r32(p + 12)); }
            }
            let pt = m.r32(b0 + 0x668);
            let n = (0..64u32).filter(|k| pt != 0 && m.r32(pt + 4 * k) != 0).count();
            locals += &format!(" proy {n} | 5c={:x} 15c={:x} 658={:x} 2f1a={:x}", m.r8(b0 + 0x5c), m.r32(b0 + 0x15c) & 0xffff, m.r32(b0 + 0x658) & 0xffff, m.r8(b0 + 0x2f1a));
        }
        let line = format!("{line} |{locals}");
        let key = format!("{st:x} {} {view:x}", battles.join(" "));
        if key != last || frame % 100 == 0 { println!("f{frame}: {line}"); last = key; }
        if let Some(dir) = &shots { if frame % every == 0 {
            let rt = unsafe { &mut *rtp };
            let _ = std::fs::create_dir_all(dir);
            rt.surface.write_png(std::path::Path::new(&format!("{dir}/f{frame:05}.png"))).unwrap();
        } }
        // `P3_PEEK=F:dir[,dir…]` (hex): en F escribe las palabras de esas direcciones (`*dir+desp` lee primero el puntero).
        if let Some(v) = std::env::var("P3_PEEK").ok() {
            let (f, list) = v.split_once(':').unwrap();
            if frame == f.parse::<u32>().unwrap() {
                for a in list.split(',') {
                    let at = if let Some(r) = a.strip_prefix('*') {
                        let (p, d) = r.split_once('+').unwrap_or((r, "0"));
                        m.r32(u32::from_str_radix(p, 16).unwrap()).wrapping_add(u32::from_str_radix(d, 16).unwrap())
                    } else { u32::from_str_radix(a, 16).unwrap() };
                    println!("f{frame}: [{a}] = {at:#x}: {:#x}", m.r32(at));
                }
            }
        }
        // `P3_DUMP=F:función:args:carpeta` (hex salvo F; args separados por comas, `-` sin argumentos): en F vuelca la memoria, llama a la función con ecx = combate 0 y vuelca
        // otra vez, para `oracle/x86_ui_dump_call.py` (la misma llamada en el x86 original sobre `before.bin`).
        if let Some(v) = std::env::var("P3_DUMP").ok() {
            let v: Vec<&str> = v.splitn(4, ':').collect();
            if frame == v[0].parse::<u32>().unwrap() && b0 != 0 {
                let (rt, mem) = unsafe { (&mut *rtp, &mut *memp) };
                let func = u32::from_str_radix(v[1], 16).unwrap();
                let args: Vec<u32> = v[2].split(',').filter(|x| *x != "-" && !x.is_empty()).map(|x| u32::from_str_radix(x, 16).unwrap()).collect();
                let dir = std::path::Path::new(v[3]);
                std::fs::create_dir_all(dir).unwrap();
                let dump = |mem: &PageMem, name: &str| {
                    let mut out = Vec::new();
                    for (a, p) in mem.mapped_pages() { out.extend_from_slice(&a.to_le_bytes()); out.extend_from_slice(p); }
                    std::fs::write(dir.join(name), out).unwrap();
                };
                // `P3_DUMP_SKIP=K`: K llamadas antes del volcado (para aislar el paso en que el cliente y el x86 se separan).
                let skip: u32 = std::env::var("P3_DUMP_SKIP").ok().and_then(|v| v.parse().ok()).unwrap_or(0);
                for _ in 0..skip { rt.call_limit = rt.calls + 200_000_000; rt.call(mem, func, b0, &args); }
                dump(mem, "before.bin");
                let seed = p3_mvp::ui_rt::rand_state();
                let (lim, ab) = (rt.call_limit, rt.aborted);
                rt.call_limit = rt.calls + 200_000_000;
                rt.missing.clear();
                // `P3_DUMP_REPEAT=N`: la misma llamada N veces seguidas (varios pasos del combate).
                let times: u32 = std::env::var("P3_DUMP_REPEAT").ok().and_then(|v| v.parse().ok()).unwrap_or(1);
                let mut r = 0;
                // Se para cuando el combate desaparece de la tabla (cierre y destructor): llamar sobre el objeto liberado no vale.
                rt.alloc_log = Some(Vec::new());
                let mut done = 0;
                for _ in 0..times { r = rt.call(mem, func, b0, &args); done += 1; if mem.u32_at(0x6E55D0) != b0 { break; } }
                let times = done;
                let allocs = rt.alloc_log.take().unwrap_or_default();
                println!("f{frame}: volcado {func:#x}({args:x?}) ecx {b0:#x} -> {r:#x}; abortada {} faltan {:x?} args mal {:x?}", rt.aborted, rt.missing, rt.arg_mismatch);
                rt.call_limit = lim; rt.aborted = ab;
                dump(mem, "after.bin");
                let call = format!("{{\"func\": {func}, \"ecx\": {b0}, \"args\": {args:?}, \"ret\": {r}, \"times\": {times}, \"allocs\": {allocs:?}, \"rand\": [{seed}, {}]}}", p3_mvp::ui_rt::rand_state());
                std::fs::write(dir.join("call.json"), call).unwrap();
            }
        }
        frame < frames
    }));
    menus::run(&mut rt, &mut mem);
    println!("frames={} aborted={} missing={:x?} args mal={:x?} sgl={:?}", rt.menus.frames, rt.aborted, rt.missing_from, rt.arg_mismatch, rt.sgl_missing);
    if let Some(dir) = std::env::var("P3_SHOTS").ok() { rt.surface.write_png(std::path::Path::new(&format!("{dir}/final.png"))).unwrap(); }
}
