//! Diagnóstico de piratas en la carta náutica (hilo «Mostrar piratas en la carta náutica», 15/11/2026).
//! Carga la primera partida de `P3_TEST_GAME_DIR/save/ein` por el menú real, manda un barco propio atracado al mar (orden
//! `0x455B50`), abre la carta con un clic y vuelca los avistamientos `ship+0x50/+0x52` (`0x50DC50`/`0x50D2B0`), la lista de la
//! carta (`[0x6CBDD8]+0x2908`, `0x4516B0`) y los piratas (estado 0x12, enlaces del orden X `+0xA/+0xC`, contador `+0x138`).
//! Uso: `P3_EXE=.. P3_CPR=.. P3_TEST_GAME_DIR=dir pirate_probe`. Opcionales: `P3_FRAMES`, `P3_MODERN=1920x1080`,
//! `P3_CLICK=F:x:y` (abrir la carta: 726,285 a 1024x768; 1846,285 a 1920x1080), `P3_POKE_NEAR=1` (en el fotograma 100 pone el
//! barco junto al pirata 96), `P3_PIRATE_AT=x,y`, `P3_TO_PIRATE=1`, `P3_NO_ORDER=1`, `P3_FAST=1`, `P3_EVERY=N`, `P3_ALL=1`,
//! `P3_PNG=salida.png`, `P3_UI_PROFILE=1` (llamadas a las funciones de avistamiento y carta), `P3_TRACK=N` (cada N fotogramas: barco
//! propio, contactos y barcos piratas con distancia), `P3_COMPARE=N` (N estados al azar: 0x50D2B0 elevado frente al puerto con paridad x86).
//! Ataque a un pirata (hilo «Arreglar ataque a piratas en la carta», 18/11/2026): `P3_F8=F` abre la carta con F8, `P3_PLACE=F;b,x,y;…`
//! coloca barcos, `P3_SHIP=b` sigue un barco con `P3_TRACK`, `P3_ATTACK=F:b:objetivo` encola la operación 0x0E como el clic derecho,
//! `P3_RCLICK=F:b:objetivo` suelta el botón derecho sobre el objetivo en la carta (con `P3_NOSEL=1` no fuerza la selección; se
//! selecciona antes con `P3_CLICK=F:x:y`), `P3_SWEEP=F:b` prueba `0x44FB40` en una rejilla.
//! El guion llama al enlazador desde dentro de `present_frame` con punteros crudos: solo para diagnóstico.
use p3_mvp::ui_rt::{menus, PageMem, Runtime};
fn h(m: &PageMem, a: u32) -> u32 { m.u32_at(a) & 0xffff }
fn dump(mem: &PageMem, tag: &str) {
    let ships = mem.u32_at(0x6DD7A4);
    let n = h(mem, 0x6DD894);
    let player = mem.u8_at(0x6DFC14) as u32;
    println!("[{tag}] navmap={:x} minimap={:x} 6DF2F0={:x} player={player}", mem.u32_at(0x6CBDD8), mem.u32_at(0x6CC940), mem.u8_at(0x6DF2F0));
    for i in 0..n {
        let s = ships + i * 0x180;
        let st = h(mem, s + 0x134);
        let own = mem.u8_at(s) as u32;
        if st == 0xff || (std::env::var_os("P3_ALL").is_none() && !(own == player && st == 0xF) && st != 0x12 && h(mem, s + 0x50) == 0xffff) { continue; }
        println!("  ship {i:4} owner {own:3} st {st:2x} xl {:x}/{:x} 138={:x} pos ({},{}) c50 {:4x} c52 {:4x} conv {:x}",
            h(mem, s + 0xA), h(mem, s + 0xC), h(mem, s + 0x138), mem.u32_at(s + 0x1E) as i16, mem.u32_at(s + 0x22) as i16, h(mem, s + 0x50), h(mem, s + 0x52), h(mem, s + 8));
    }
}
fn main() {
    let exe: &'static [u8] = Box::leak(std::fs::read(std::env::var_os("P3_EXE").unwrap()).unwrap().into_boxed_slice());
    let cpr: &'static [u8] = Box::leak(std::fs::read(std::env::var_os("P3_CPR").unwrap()).unwrap().into_boxed_slice());
    let archive = Box::leak(Box::new(p3_assets::cpr::Archive::parse(cpr).unwrap()));
    let mut mem = PageMem::default();
    let mut rt = Runtime::new(&mut mem, exe, archive, 800, 600).unwrap();
    rt.menus.game_dir = Some(std::env::var_os("P3_TEST_GAME_DIR").unwrap().into());
    rt.call_limit = 200_000_000;
    assert!(menus::boot(&mut rt, &mut mem));
    if let Ok(v) = std::env::var("P3_MODERN") { let (w, h) = v.split_once('x').unwrap(); rt.display.modern = Some((w.parse().unwrap(), h.parse().unwrap())); }
    let mut frame = 0;
    let click: Vec<i32> = std::env::var("P3_CLICK").unwrap_or("110:726:285".into()).split(':').map(|v| v.parse().unwrap()).collect();
    // `P3_RCLICK=F:barco:objetivo`: en F selecciona el barco (lista del panel 0x6CE6D0+0xA0/+0xAC, como la orden de abajo) y suelta
    // el botón derecho sobre el objetivo en la carta: pantalla = mapa - desplazamiento (+0x29E4/+0x29E6) + posición de la ventana.
    let rclick: Option<Vec<u32>> = std::env::var("P3_RCLICK").ok().map(|v| v.split(':').map(|x| x.parse().unwrap()).collect());
    let rpos = std::rc::Rc::new(std::cell::Cell::new(None::<(i32, i32)>));
    let rpos2 = rpos.clone();
    rt.on_present = Some(Box::new(move |_, queue, input| {
        frame += 1; input.time_ms = Some(frame * 40);
        use p3_mvp::ui_rt::win32;
        // `P3_F8=F`: pulsa F8 (abre la carta náutica, `0x0042D627`) en los fotogramas F y F+1.
        if let Some(f8) = std::env::var("P3_F8").ok().and_then(|v| v.parse::<u32>().ok()) {
            if frame == f8 || frame == f8 + 1 {
                let down = frame == f8;
                input.set_key(0x77, down);
                win32::post(queue, input, if down { win32::WM_KEYDOWN } else { win32::WM_KEYUP }, 0x77, 1);
            }
        }
        if let Some((x, y)) = rpos2.take() {
            input.cursor = (x as _, y as _);
            win32::post(queue, input, win32::WM_MOUSEMOVE, 0, win32::mouse_lparam(x as _, y as _));
            input.set_key(win32::VK_RBUTTON, true);
            win32::post(queue, input, win32::WM_RBUTTONDOWN, input.mouse_keys(), win32::mouse_lparam(x as _, y as _));
            input.set_key(win32::VK_RBUTTON, false);
            win32::post(queue, input, win32::WM_RBUTTONUP, input.mouse_keys(), win32::mouse_lparam(x as _, y as _));
            println!("clic derecho en ({x},{y})");
        }
        let (f, x, y) = (click[0] as u32, click[1], click[2]);
        if frame == f { input.cursor = (x as _, y as _); win32::post(queue, input, win32::WM_MOUSEMOVE, 0, win32::mouse_lparam(x as _, y as _)); }
        if frame == f + 1 || frame == f + 2 {
            let down = frame == f + 1;
            input.set_key(win32::VK_LBUTTON, down);
            win32::post(queue, input, if down { win32::WM_LBUTTONDOWN } else { win32::WM_LBUTTONUP }, input.mouse_keys(), win32::mouse_lparam(x as _, y as _));
        }
        true
    }));
    let frames: u32 = std::env::var("P3_FRAMES").ok().and_then(|v| v.parse().ok()).unwrap_or(35);
    let rtp: *mut Runtime<'static> = &mut rt;
    let memp: *mut PageMem = &mut mem;
    let target: Vec<i32> = std::env::var("P3_TARGET").unwrap_or("676,859".into()).split(',').map(|v| v.parse().unwrap()).collect();
    let mut sent: Option<u32> = None;
    rt.menus.frame_script = Some(Box::new(move |m, frame| {
        if frame == 3 { m.w32(menus::MAIN_MENU + 0x284, 4); }
        if frame == 8 { let load = m.r32(0x006C_BB7C); m.w32(load + 0x11c4, 0); }
        if frame == 40 && std::env::var_os("P3_NO_ORDER").is_none() {
            // Orden real «navegar a» (0x455B50) para un barco propio atracado, como tests/ui_ship_movement.rs.
            let (rt, mem) = unsafe { (&mut *rtp, &mut *memp) };
            let ships = mem.u32_at(0x6DD7A4);
            let count = h(mem, 0x6DD894);
            let owner = mem.u8_at(0x6DFC14);
            let id = (0..count).find(|id| { let s = ships + id * 0x180; mem.u8_at(s) == owner && h(mem, s + 0x134) == 0 && h(mem, s + 8) == 0xffff }).expect("sin barco propio atracado");
            let panel = mem.u32_at(0x006C_E6D0);
            let list = rt.new_object(mem, 2);
            mem.write(list, &(id as u16).to_le_bytes());
            mem.set_u32(panel + 0xa0, list); mem.set_u32(panel + 0xac, 1);
            let map = rt.new_object(mem, 0x3000);
            let point = rt.new_object(mem, 8);
            let pir = ships + 96 * 0x180;
            let (tx, ty) = if std::env::var_os("P3_TO_PIRATE").is_some() { (mem.u32_at(pir + 0x1e) & 0xffff, mem.u32_at(pir + 0x22) & 0xffff) } else { (target[0] as u32, target[1] as u32) };
            println!("destino ({tx},{ty})");
            mem.set_u32(point, tx); mem.set_u32(point + 4, ty);
            let (lim, ab) = (rt.call_limit, rt.aborted);
            rt.call_limit = rt.calls + 50_000_000;
            rt.call(mem, 0x0045_5B50, map, &[point]);
            println!("tras orden: cola={} st={:x}", h(mem, 0x6df2f0 + 0x56), h(mem, ships + id * 0x180 + 0x134));
            rt.call_limit = lim; rt.aborted = ab;
            println!("orden barco {id} -> {:?} missing={:x?}", target, rt.missing_from);
            m.w32(0x6DFC1C, 1);
            sent = Some(id);
        }
        if let (Some(id), true) = (sent, frame == 100 && std::env::var_os("P3_POKE_NEAR").is_some()) {
            let ships = m.r32(0x6DD7A4);
            let (s, p) = (ships + id * 0x180, ships + 96 * 0x180);
            let (mut px, mut py) = (m.r32(p + 0x1e) & 0xffff, m.r32(p + 0x22) & 0xffff);
            if let Ok(v) = std::env::var("P3_PIRATE_AT") {
                let v: Vec<u32> = v.split(',').map(|x| x.parse().unwrap()).collect();
                (px, py) = (v[0], v[1]);
                let o = m.r32(p + 0x1c) & 0xffff; m.w32(p + 0x1c, o | (px << 16));
                let o = m.r32(p + 0x20) & 0xffff; m.w32(p + 0x20, o | (py << 16));
            }
            let o = m.r32(s + 0x1c) & 0xffff; m.w32(s + 0x1c, o | ((px + 60) << 16));
            let o = m.r32(s + 0x20) & 0xffff; m.w32(s + 0x20, o | ((py + 40) << 16));
            println!("f100: barco movido junto al pirata ({},{})", px + 15, py + 15);
        }
        if sent.is_none() { sent = std::env::var("P3_SHIP").ok().and_then(|v| v.parse().ok()); }
        if let Some(v) = rclick.as_ref() {
            if frame + 10 == v[0] && std::env::var_os("P3_CENTER").is_some() {
                // Centra la carta en el objetivo (desplazamiento +0x29E4/+0x29E6 en coordenadas de mapa).
                let nav = m.r32(0x6CBDD8);
                let t = m.r32(0x6DD7A4) + v[2] * 0x180;
                let (tx, ty) = (m.r32(t + 0x1e) & 0xffff, m.r32(t + 0x22) & 0xffff);
                let o = m.r32(nav + 0x29e4);
                m.w32(nav + 0x29e4, (tx.saturating_sub(370) & 0xffff) | ((ty.saturating_sub(360) & 0xffff) << 16));
                println!("f{frame}: carta centrada; desplazamiento antes {o:x}");
            }
            if frame == v[0] {
                let (rt, mem) = unsafe { (&mut *rtp, &mut *memp) };
                let panel = mem.u32_at(0x006C_E6D0);
                if std::env::var_os("P3_NOSEL").is_none() {
                    let list = rt.new_object(mem, 2);
                    mem.write(list, &(v[1] as u16).to_le_bytes());
                    mem.set_u32(panel + 0xa0, list); mem.set_u32(panel + 0xac, 1);
                }
                println!("seleccion panel: lista {:#x} n {}", mem.u32_at(panel + 0xa0), mem.u32_at(panel + 0xac));
                let nav = mem.u32_at(0x6CBDD8);
                let t = mem.u32_at(0x6DD7A4) + v[2] * 0x180;
                let (tx, ty) = (mem.u32_at(t + 0x1e) as i16 as i32, mem.u32_at(t + 0x22) as i16 as i32);
                let (ox, oy) = (mem.u32_at(nav + 0x29e4) as i16 as i32, mem.u32_at(nav + 0x29e6) as i16 as i32);
                let (wx, wy) = (mem.u32_at(nav + 0x14) as i32, mem.u32_at(nav + 0x18) as i32);
                let dx: i32 = std::env::var("P3_RDX").ok().and_then(|v| v.parse().ok()).unwrap_or(0);
                let dy: i32 = std::env::var("P3_RDY").ok().and_then(|v| v.parse().ok()).unwrap_or(0);
                let p = (tx - ox + wx + dx, ty - oy + wy + dy);
                let sh = mem.u32_at(0x6DD7A4) + v[1] * 0x180;
                println!("f{frame}: objetivo mapa ({tx},{ty}) desplazamiento ({ox},{oy}) ventana ({wx},{wy}) -> pantalla {p:?}; barco ({},{}) contactos {:x}/{:x}", mem.u32_at(sh + 0x1e) as i16, mem.u32_at(sh + 0x22) as i16, h(mem, sh + 0x50), h(mem, sh + 0x52));
                rpos.set(Some(p));
            }
            if frame > v[0] && frame <= v[0] + 8 {
                let s = m.r32(0x6DD7A4) + v[1] * 0x180;
                println!("   f{frame}: +3a={:x} st={:x} cola={}", m.r32(s + 0x3a) & 0xffff, m.r32(s + 0x134) & 0xffff, m.r32(0x6df2f0 + 0x56) & 0xffff);
            }
        }
        // `P3_PLACE=F;barco,x,y;barco,x,y`: en F pone esos barcos en esas coordenadas de mapa (parte entera de +0x1C/+0x20).
        if let Ok(v) = std::env::var("P3_PLACE") {
            let mut it = v.split(';');
            let f: u32 = it.next().unwrap().parse().unwrap();
            if frame == f {
                for e in it {
                    let e: Vec<u32> = e.split(',').map(|x| x.parse().unwrap()).collect();
                    let s = m.r32(0x6DD7A4) + e[0] * 0x180;
                    let o = m.r32(s + 0x1c) & 0xffff; m.w32(s + 0x1c, o | (e[1] << 16));
                    let o = m.r32(s + 0x20) & 0xffff; m.w32(s + 0x20, o | (e[2] << 16));
                }
                println!("f{frame}: barcos colocados {v}");
            }
        }
        // `P3_SWEEP=F:barco`: en F selecciona el barco y prueba `nav_map_handle_point_click(x, y, -3, 0)` en una rejilla; anota
        // los puntos que encolan una operación (la 0x0E del clic derecho sobre un contacto).
        if let Some(v) = std::env::var("P3_SWEEP").ok().map(|v| v.split(':').map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>()) {
            if frame == v[0] {
                let (rt, mem) = unsafe { (&mut *rtp, &mut *memp) };
                let panel = mem.u32_at(0x006C_E6D0);
                let list = rt.new_object(mem, 2);
                mem.write(list, &(v[1] as u16).to_le_bytes());
                mem.set_u32(panel + 0xa0, list); mem.set_u32(panel + 0xac, 1);
                let nav = mem.u32_at(0x6CBDD8);
                let mut hits = Vec::new();
                let (ys, xs) = if std::env::var_os("P3_SWEEP1").is_some() { (270..271, 430..431) } else { (150..400, 300..560) };
                for y in ys.step_by(3) { for x in xs.clone().step_by(3) {
                    let before = h(mem, 0x6df2f0 + 0x56);
                    let r = rt.call(mem, 0x0044_FB40, nav, &[x, y, 0xffff_fffd, 0]);
                    let after = h(mem, 0x6df2f0 + 0x56);
                    if after != before || r != 0 { hits.push((x, y, r, after)); }
                    if after != before { mem.set_u32(0x6df2f0 + 0x56, (mem.u32_at(0x6df2f0 + 0x56) & 0xffff_0000) | before); }
                } }
                for k in [0xacu32, 0xb0, 0xb4, 0xb8] { let o = mem.u32_at(nav + k); println!("nav+{k:x}={o:#x} +4={:#x} +c={:#x} +14={:#x}", if o != 0 { mem.u32_at(o + 4) } else { 0 }, if o != 0 { mem.u32_at(o + 0xc) } else { 0 }, if o != 0 { mem.u32_at(o + 0x14) } else { 0 }); }
                { let t = mem.u32_at(0x6DD7A4) + 0x85 * 0x180; println!("pirata tipo {:x} rumbo {:x} conv {:x} st {:x}", mem.u8_at(t + 0xe), mem.u8_at(t + 1), h(mem, t + 8), h(mem, t + 0x134)); }
                { let cv = mem.u32_at(0x6dd7a8) + 15 * 60; println!("convoy f lider {:x} n conv {:x}", h(mem, cv + 0x10), h(mem, 0x6dd896)); }
                println!("tabla 451078 {:x?} tabla 451088 {:x?}", (0..4).map(|k| mem.u32_at(0x451078 + 4 * k)).collect::<Vec<_>>(), (0..4).map(|k| mem.u32_at(0x451088 + 4 * k)).collect::<Vec<_>>());
                { let b = mem.u32_at(nav + 0xac); let r = mem.u32_at(b + 0xc) + 0xd * 16; println!("rect tipo0 rumbo d: {:?} nav+18={}", (mem.u32_at(r) as i32, mem.u32_at(r+4) as i32, mem.u32_at(r+8) as i32, mem.u32_at(r+12) as i32), mem.u32_at(nav+0x18) as i32); }
                for k in [0x7000013eu32, 0x7000013f, 0x70000140, 0x70000141] { let d = rt.assets.defs.get(&k); println!("tex {k:x}: {:?}", d.map(|d| (d.file.clone(), d.frames.len(), d.flags))); if let Some(d) = d { let f = d.file.clone(); println!("   archivo {} bytes, cabecera {:x?}", rt.assets.file(f.as_bytes()).map_or(0, |b| b.len()), rt.assets.file(f.as_bytes()).map(|b| b[..16.min(b.len())].to_vec())); } }
                println!("6dfc14={:#x} sel lista={:x?}", mem.u32_at(0x6dfc14), (mem.u32_at(panel + 0xa0), mem.u32_at(panel + 0xac)));
                println!("barrido: {} aciertos {:?}", hits.len(), &hits[..hits.len().min(40)]);
            }
        }
        // `P3_ATTACK=F:barco:objetivo`: en el fotograma F encola la operación 0x0E (fijar objetivo) como el clic derecho de la carta
        // (`nav_map_handle_point_click` 0x0045017B: constructor 0x535740, +0 = 0xE, +4 barco, +8 objetivo, cola 0x54AA70 sobre 0x6DF2F0).
        if let Some(v) = std::env::var("P3_ATTACK").ok() {
            let v: Vec<u32> = v.split(':').map(|x| x.parse().unwrap()).collect();
            if frame == v[0] {
                let (rt, mem) = unsafe { (&mut *rtp, &mut *memp) };
                let op = rt.new_object(mem, 0x40);
                rt.call(mem, 0x0053_5740, op, &[]);
                mem.set_u32(op, 0xE); mem.set_u32(op + 4, v[1]); mem.set_u32(op + 8, v[2]);
                rt.call(mem, 0x0054_AA70, 0x006D_F2F0, &[op]);
                let s = mem.u32_at(0x6DD7A4) + v[1] * 0x180;
                println!("f{frame}: op 0x0E {} -> {} encolada; +3a={:x} cola={}", v[1], v[2], h(mem, s + 0x3a), h(mem, 0x6df2f0 + 0x56));
                if std::env::var_os("P3_ATTACK_DIRECT").is_some() {
                    rt.call(mem, 0x0053_8800, op, &[]);
                    println!("   directa 0x538800: +3a={:x}", h(mem, s + 0x3a));
                }
            }
            if frame > v[0] && frame <= v[0] + 6 {
                let s = m.r32(0x6DD7A4) + v[1] * 0x180;
                println!("   f{frame}: +3a={:x} st={:x} cola={}", m.r32(s + 0x3a) & 0xffff, m.r32(s + 0x134) & 0xffff, m.r32(0x6df2f0 + 0x56) & 0xffff);
            }
        }
        if frame >= 40 && std::env::var_os("P3_FAST").is_some() { m.w32(0x6DFC1C, 1); }
        if frame % std::env::var("P3_EVERY").ok().and_then(|v| v.parse().ok()).unwrap_or(250) == 0 {
            let ships = m.r32(0x6DD7A4); let n = m.r32(0x6DD894) & 0xffff;
            for i in 0..n { let s = ships + i * 0x180; let st = m.r32(s + 0x134) & 0xffff; if st == 0x12 || st == 0x16 || (m.r8(s) == 255 && st != 0xff) {
                println!("   f{frame} ship {i} owner {} st {st:x} xl {:x}/{:x} 138={:x} pos ({},{}) conv {:x}", m.r8(s), m.r32(s+0xA)&0xffff, m.r32(s+0xC)&0xffff, m.r32(s+0x138)&0xffff, m.r32(s+0x1e) as i16, m.r32(s+0x22) as i16, m.r32(s+8)&0xffff);
            } }
        }
        if frame % 50 == 0 { println!("f{frame} t={:x} modo={:x} 914={:x}", m.r32(0x6de4b4), m.r32(0x6DFC1C), m.r32(0x6DF2F0 + 0x914)); }
        if let (Some(id), Some(k)) = (sent, std::env::var("P3_TRACK").ok().and_then(|v| v.parse::<u32>().ok())) { if frame % k == 0 {
            let ships = m.r32(0x6DD7A4); let n = m.r32(0x6DD894) & 0xffff;
            let s = ships + id * 0x180;
            let (sx, sy) = (m.r32(s + 0x1e) as i16 as i32, m.r32(s + 0x22) as i16 as i32);
            let mut line = format!("f{frame} t={:x} G st {:x} ({sx},{sy}) 3a {:x} c {:x}/{:x} |", m.r32(0x6de4b4), m.r32(s + 0x134) & 0xffff, m.r32(s + 0x3a) & 0xffff, m.r32(s + 0x50) & 0xffff, m.r32(s + 0x52) & 0xffff);
            for i in 0..n { let p = ships + i * 0x180; let st = m.r32(p + 0x134) & 0xffff; if m.r8(p) == 255 && st != 0xff && st != 0x15 {
                let (px, py) = (m.r32(p + 0x1e) as i16 as i32, m.r32(p + 0x22) as i16 as i32);
                line += &format!(" p{i} st {st:x} ({px},{py}) d={} 138={:x} xl {:x}/{:x} tgt {:x};", (((px - sx).pow(2) + (py - sy).pow(2)) as f64).sqrt() as i32, m.r32(p + 0x138) & 0xffff, m.r32(p + 0xA) & 0xffff, m.r32(p + 0xC) & 0xffff, m.r32(p + 0x50) & 0xffff);
            } }
            println!("{line}");
        } }
        if let Some(id) = sent { if frame % 25 == 0 {
            let mem = unsafe { &*memp };
            let nav = mem.u32_at(0x6CBDD8);
            let n = mem.u32_at(nav + 0x2908);
            let l: Vec<String> = (0..n.min(40)).map(|k| format!("{:x}", h(mem, nav + 0x28A4 + 2 * k))).collect();
            println!("   carta: lista {n} [{}] vista ({},{}) {}x{}", l.join(","), mem.u32_at(nav+0x14) as i32, mem.u32_at(nav+0x18) as i32, mem.u32_at(nav+0x2c) as i32, mem.u32_at(nav+0x30) as i32);
            let s = mem.u32_at(0x6DD7A4) + id * 0x180;
            let p = mem.u32_at(0x6DD7A4) + 96 * 0x180;
            println!("   pirata 96 st {:x} pos ({},{}) c50 {:x}", h(mem, p + 0x134), mem.u32_at(p + 0x1e) as i16, mem.u32_at(p + 0x22) as i16, h(mem, p + 0x50));
            println!("f{frame} día? t={:x} barco {id} st {:x} pos ({},{}) c50 {:x} c52 {:x}", mem.u32_at(0x6de4b4), h(mem, s + 0x134), mem.u32_at(s + 0x1e) as i16, mem.u32_at(s + 0x22) as i16, h(mem, s + 0x50), h(mem, s + 0x52));
        } }
        frame < frames
    }));
    menus::run(&mut rt, &mut mem);
    println!("frames={} missing={:x?}", rt.menus.frames, rt.missing_from);
    if let Some(p) = &rt.profile { for f in [0x50dc50u32, 0x50d2b0, 0x546870, 0x546620, 0x4516b0, 0x44cbf0, 0x506720] { println!("perfil {f:x}: {:?}", p.get(&f)); } }
    dump(&mem, "cargada");
    if let Some(n) = std::env::var("P3_COMPARE").ok().and_then(|v| v.parse::<u32>().ok()) { compare(&mut rt, &mut mem, n); }
    if let Ok(p) = std::env::var("P3_PNG") { rt.surface.write_png(std::path::Path::new(&p)).unwrap(); }
}

struct NoHost;
impl p3_core::ships::spotting::SpotHost for NoHost {
    fn lost(&mut self, _: u16) {}
    fn pulse(&mut self, _: u16, _: u16) {}
    fn sound(&mut self, _: u16, _: i32, _: i32) {}
}

/// `P3_COMPARE=N`: N estados al azar alrededor de un barco propio; compara 0x50D2B0 elevado (el del cliente) con
/// `p3_core::ships::spotting::update_ship_spotted_contacts` (paridad x86 en c10_spotting).
fn compare(rt: &mut Runtime, mem: &mut PageMem, n: u32) {
    let mut seed = 0x1234_5678u32;
    let mut rnd = move |k: u32| { seed = seed.wrapping_mul(1_103_515_245).wrapping_add(12345); (seed >> 8) % k };
    let ships = mem.u32_at(0x6DD7A4);
    let count = h(mem, 0x6DD894);
    let player = mem.u8_at(0x6DFC14);
    let own: Vec<u32> = (0..count).filter(|i| mem.u8_at(ships + i * 0x180) == player).collect();
    let others: Vec<u32> = (0..count).filter(|i| { let st = h(mem, ships + i * 0x180 + 0x134); st == 0xF || st == 0x12 }).filter(|i| mem.u8_at(ships + i * 0x180) != player).collect();
    let (mut ok, mut bad) = (0, 0);
    for case in 0..n {
        let s = own[rnd(own.len() as u32) as usize];
        let sa = ships + s * 0x180;
        // barco propio en el mar en la posición de un barco ajeno enlazado, con rumbo al azar
        let o = others[rnd(others.len() as u32) as usize];
        let oa = ships + o * 0x180;
        let (ox, oy) = (h(mem, oa + 0x1E), h(mem, oa + 0x22));
        mem.write(sa + 0x134, &0xFu16.to_le_bytes());
        mem.write(sa + 0x1E, &((ox + rnd(200)).wrapping_sub(100) as u16).to_le_bytes());
        mem.write(sa + 0x22, &((oy + rnd(200)).wrapping_sub(100) as u16).to_le_bytes());
        mem.write(sa + 1, &[rnd(16) as u8]);
        // algunos ajenos: piratas 0x12 con o sin contador, rumbos al azar
        for _ in 0..6 {
            let t = others[rnd(others.len() as u32) as usize];
            let ta = ships + t * 0x180;
            if rnd(2) == 0 { mem.write(ta + 0x134, &0x12u16.to_le_bytes()); }
            mem.write(ta + 0x138, &(if rnd(2) == 0 { 0 } else { rnd(0x900) as u16 }).to_le_bytes());
            mem.write(ta + 1, &[rnd(16) as u8]);
            mem.write(ta + 0x1E, &((h(mem, sa + 0x1E) + rnd(160)).wrapping_sub(80) as u16).to_le_bytes());
            mem.write(ta + 0x22, &((h(mem, sa + 0x22) + rnd(160)).wrapping_sub(80) as u16).to_le_bytes());
        }
        let prev = [h(mem, sa + 0x50), h(mem, sa + 0x52)];
        if rnd(3) == 0 { let c = others[rnd(others.len() as u32) as usize] as u16; mem.write(sa + 0x50, &c.to_le_bytes()); }
        let mut mgr: Vec<u8> = (0..0x100u32).map(|k| mem.u8_at(0x6DD7A0 + k)).collect();
        let mut tab: Vec<u8> = (0..count * 0x180).map(|k| mem.u8_at(ships + k)).collect();
        p3_core::ships::spotting::update_ship_spotted_contacts(&mut mgr, &mut tab, s as u16, false, &mut NoHost);
        rt.call_limit = rt.calls + 50_000_000; rt.aborted = false;
        rt.call(mem, 0x0050_D2B0, 0x6DD7A0, &[s, 0]);
        let got: Vec<u8> = (0..count * 0x180).map(|k| mem.u8_at(ships + k)).collect();
        let gm: Vec<u8> = (0..0x100u32).map(|k| mem.u8_at(0x6DD7A0 + k)).collect();
        if got == tab && gm == mgr { ok += 1; } else {
            bad += 1;
            if bad <= 5 {
                let diffs: Vec<String> = (0..tab.len()).filter(|&k| tab[k] != got[k]).take(8).map(|k| format!("barco {} +{:x}: puerto {:02x} elevado {:02x}", k / 0x180, k % 0x180, tab[k], got[k])).collect();
                println!("caso {case} barco {s} previo {prev:x?}: {}", diffs.join("; "));
            }
            mem.write(ships, &tab); // seguir desde el resultado del puerto
        }
    }
    println!("comparación 0x50D2B0: {ok} iguales, {bad} distintos; missing={:x?}", rt.missing_from);
}
