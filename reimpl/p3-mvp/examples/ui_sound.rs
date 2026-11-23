//! Sonido del juego original en el enlazador, sin tarjeta: arranca el menú con el mezclador de `p3_audio::live` (Miles
//! sustituida, `ui_rt/miles.rs`), opcionalmente entra en la ciudad («city», como `ui_benchmark`), mezcla 40 ms por fotograma
//! y guarda la salida en un WAV. Imprime las llamadas `AIL_*`, lo que falte y el nivel de la mezcla. Recursos comerciales locales.
//!
//! ```text
//! P3_EXE=Patrician3.exe P3_CPR=p2arch0_eng.cpr [P3_GAME_DIR=carpeta con Musik] ui_sound [menu|city] [FOTOGRAMAS] [salida.wav]
//! ```
use p3_mvp::ui_rt::{menus, miles, win32, PageMem, Runtime};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

fn write_wav(path: &Path, rate: u32, pcm: &[f32]) {
    let data: Vec<u8> = pcm
        .iter()
        .flat_map(|x| ((x.clamp(-1.0, 1.0) * 32767.0) as i16).to_le_bytes())
        .collect();
    let mut w = Vec::new();
    w.extend_from_slice(b"RIFF");
    w.extend_from_slice(&(36 + data.len() as u32).to_le_bytes());
    w.extend_from_slice(b"WAVEfmt ");
    w.extend_from_slice(&16u32.to_le_bytes());
    w.extend_from_slice(&1u16.to_le_bytes());
    w.extend_from_slice(&2u16.to_le_bytes());
    w.extend_from_slice(&rate.to_le_bytes());
    w.extend_from_slice(&(rate * 4).to_le_bytes());
    w.extend_from_slice(&4u16.to_le_bytes());
    w.extend_from_slice(&16u16.to_le_bytes());
    w.extend_from_slice(b"data");
    w.extend_from_slice(&(data.len() as u32).to_le_bytes());
    w.extend_from_slice(&data);
    std::fs::write(path, w).unwrap();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let scenario = args.get(1).map(String::as_str).unwrap_or("menu");
    let frames: u32 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(200);
    let out = args.get(3).map(PathBuf::from);
    let exe_path = PathBuf::from(std::env::var_os("P3_EXE").expect("P3_EXE required"));
    let cpr_path = PathBuf::from(std::env::var_os("P3_CPR").expect("P3_CPR required"));
    let exe = std::fs::read(&exe_path).unwrap();
    let cpr = std::fs::read(&cpr_path).unwrap();
    let archive = p3_assets::cpr::Archive::parse(&cpr).unwrap();
    let mut mem = PageMem::default();
    let mut rt = Runtime::new(&mut mem, &exe, &archive, 800, 600).unwrap();
    rt.call_limit = 1_000_000_000;
    let mixer = p3_audio::live::Mixer::new(44100);
    let game_dir = std::env::var_os("P3_GAME_DIR").map(PathBuf::from);
    let mut roots: Vec<&Path> = game_dir.iter().map(PathBuf::as_path).collect();
    roots.extend([cpr_path.as_path(), exe_path.as_path()]);
    rt.miles.dirs = miles::default_dirs(&roots);
    rt.miles.mixer = Some(mixer.clone());
    assert!(menus::boot(&mut rt, &mut mem));
    let city = scenario == "city";
    let pcm = Arc::new(Mutex::new(Vec::<f32>::new()));
    let (sink, m2) = (pcm.clone(), mixer.clone());
    let levels = Arc::new(Mutex::new(Vec::<(u32, f32, usize)>::new()));
    let lv = levels.clone();
    let mut frame = 0u32;
    rt.on_present = Some(Box::new(move |_surface, queue, input| {
        frame += 1;
        input.time_ms = Some(frame * 40);
        let mut block = vec![0.0f32; 2 * 1764];
        m2.render(&mut block);
        let rms = (block.iter().map(|x| x * x).sum::<f32>() / block.len() as f32).sqrt();
        lv.lock().unwrap().push((frame, rms, m2.playing()));
        sink.lock().unwrap().extend_from_slice(&block);
        if city {
            let (x, y) = (685, 561);
            if frame == 10 {
                input.cursor = (x, y);
                win32::post(queue, input, win32::WM_MOUSEMOVE, 0, win32::mouse_lparam(x, y));
            }
            if frame == 11 || frame == 12 {
                let down = frame == 11;
                input.set_key(win32::VK_LBUTTON, down);
                win32::post(
                    queue,
                    input,
                    if down {
                        win32::WM_LBUTTONDOWN
                    } else {
                        win32::WM_LBUTTONUP
                    },
                    input.mouse_keys(),
                    win32::mouse_lparam(x, y),
                );
            }
        }
        // Da tiempo a los hilos de descodificación (en el juego real los fotogramas ya duran eso).
        std::thread::sleep(std::time::Duration::from_millis(5));
        true
    }));
    rt.menus.frame_script = Some(Box::new(move |m, f| {
        if city && f == 3 {
            m.w32(menus::MAIN_MENU + 0x284, 2);
        }
        f < frames
    }));
    menus::run(&mut rt, &mut mem);
    for f in [0x0058_A4F0u32, 0x0058_9010, 0x0042_CD90, 0x0042_C770, 0x0044_F680] {
        eprintln!(
            "  {f:#x}: ejecutada {:?} · ausente {:?}",
            rt.ported.get(&f),
            rt.missing_from
                .iter()
                .filter(|((g, _), _)| *g == f)
                .map(|((_, from), n)| format!("{from:x}:{n}"))
                .collect::<Vec<_>>()
        );
    }
    let gs = mem.u32_at(0x006C_BB40);
    if gs != 0 {
        let f: Vec<String> = (0x287Cu32..0x28CC)
            .step_by(4)
            .map(|o| format!("{o:x}={:x}", mem.u32_at(gs + o)))
            .collect();
        eprintln!("GameScreen {gs:#x}: {}", f.join(" "));
        let sm = mem.u32_at(0x006C_BDB8);
        let f: Vec<String> = [
            0u32, 4, 8, 0xc, 0x10, 0x14, 0x68, 0x6c, 0x120, 0x1b8, 0x1bc, 0x1c4, 0x1c8, 0x1cc, 0x1d0, 0x1d4, 0x1d8,
            0x1dc, 0x1e0,
        ]
        .iter()
        .map(|o| format!("{o:x}={:x}", mem.u32_at(sm + o)))
        .collect();
        eprintln!("SoundManager {sm:#x}: {}", f.join(" "));
    }
    let opts = mem.u32_at(0x006C_C3E8);
    eprintln!(
        "opciones [0x6CC3E8]={opts:#x} +0x21={} · reloj [0x6DCCF0]={} · escena [0x6E51AC]={:#x}",
        if opts != 0 { mem.u8_at(opts + 0x21) } else { 0 },
        mem.u32_at(0x006D_CCF0),
        mem.u32_at(0x006E_51AC)
    );
    let levels = levels.lock().unwrap();
    let audible: Vec<u32> = levels.iter().filter(|l| l.1 > 1e-3).map(|l| l.0).collect();
    eprintln!("llamadas AIL: {:?}", rt.miles.calls);
    let sound: Vec<_> = rt
        .ported
        .iter()
        .filter(|(f, _)| {
            (0x0044_1900..0x0044_4E00).contains(*f)
                || (0x004C_4100..0x004C_4800).contains(*f)
                || (0x0042_C000..0x0042_D000).contains(*f)
        })
        .map(|(f, n)| format!("{f:x}:{n}"))
        .collect();
    eprintln!("gestor de sonido ejecutado: {sound:?}");
    let missing: Vec<_> = rt
        .missing_from
        .iter()
        .filter(|((f, _), _)| (0x0044_1900..0x0044_4E00).contains(f) || (0x0042_C000..0x0042_D000).contains(f))
        .map(|((f, from), n)| format!("{f:x}<-{from:x}:{n}"))
        .collect();
    eprintln!("gestor de sonido ausente: {missing:?}");
    eprintln!(
        "importaciones ausentes: {:?}",
        rt.import_missing
            .iter()
            .filter(|n| n.contains("AIL"))
            .collect::<Vec<_>>()
    );
    eprintln!(
        "fotogramas {} · con sonido {} (primero {:?}, último {:?}) · voces sonando al final {} · pico {:.3}",
        levels.len(),
        audible.len(),
        audible.first(),
        audible.last(),
        levels.last().map_or(0, |l| l.2),
        mixer.stats().1
    );
    for l in levels.iter().step_by(25) {
        eprintln!("  fotograma {:4}: rms {:.4}, voces {}", l.0, l.1, l.2);
    }
    if let Some(out) = out {
        write_wav(&out, 44100, &pcm.lock().unwrap());
    }
}
