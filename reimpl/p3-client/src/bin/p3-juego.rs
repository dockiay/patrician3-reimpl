//! `p3-juego`: ejecutable nativo del juego.
//!
//! ```text
//! p3-juego                                           menú con p3-juego.ini y partidas junto al EXE
//! p3-juego vista PARTIDA.pat RECURSOS.cpr [opciones]   ciudad y carta náutica de la partida (Tab alterna)
//! p3-juego original Patrician3.exe RECURSOS.cpr [PARTIDA.pat] [--escena] [--abrir TAMAÑO:CTOR[:MÉTODO...]]
//!                                                     bucle del original (screen_list_run_frame) en la ventana
//! p3-juego menu Patrician3.exe RECURSOS.cpr [--juego DIR] [--idioma CÓDIGO]  arranque original (game_main_window_initialize + _run, ui_rt::menus);
//!                                                     DIR: carpeta con save\\ein\\*.pat (por defecto la del .cpr)
//! p3-juego prueba [opciones]                          patrón de prueba y eco de los mensajes de entrada
//! opciones: --sin-sonido (sin música ni efectos)  --ventana  --exclusivo  --raton-libre (no confinar el ratón a pantalla completa)  --carta  --fps N  --fotogramas N  --captura RUTA.png  --guion "F:orden;..."
//!           --moderna [ANCHOxALTO]   resolución moderna (sin tamaño: la del monitor; 4K se reparte en 1920×1080 al doble):
//!                                    se ve más ciudad y más mapa en vez de ampliar la imagen. Ver `reimpl/RESOLUCIONES_MODERNAS.md`.
//! ```
#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

#[path = "p3-juego/launch.rs"]
mod launch;

use p3_client::view::{Mode, Script, View};
use p3_client::window::{GameWindow, WindowConfig};
use p3_mvp::ui_rt::win32;
use std::collections::VecDeque;
use std::path::PathBuf;

struct Opts {
    pos: Vec<String>,
    cfg: WindowConfig,
    chart: bool,
    frames: u64,
    capture: Option<PathBuf>,
    script: Script,
    original: p3_client::original::OriginalOptions,
    /// `--moderna`: resolución moderna pedida (`Some(None)` = la del monitor).
    modern: Option<Option<(u32, u32)>>,
    /// `--juego DIR`: carpeta del juego para `save\ein\*.pat` y demás archivos que abre el original (cargar y guardar).
    game_dir: Option<PathBuf>,
    language: Option<String>,
    /// `--sin-sonido`: no abre la salida de sonido (el gestor del juego queda sin dispositivo).
    no_sound: bool,
    /// `--intro`: deja que el juego lance `binkplay.exe` con los vídeos de introducción, como el original.
    intro: bool,
}

fn parse_size(v: &str) -> Option<(u32, u32)> {
    let (w, h) = v.split_once(['x', 'X'])?;
    let (w, h) = (w.parse().ok()?, h.parse().ok()?);
    (w >= 640 && h >= 480 && w <= 16384 && h <= 16384).then_some((w, h))
}

fn parse() -> Result<Opts, String> {
    let mut o = Opts { pos: Vec::new(), cfg: WindowConfig::default(), chart: false, frames: 0, capture: None, script: Script::default(), original: Default::default(), modern: None, game_dir: None, language: None, no_sound: false, intro: std::env::var_os("P3_INTRO").is_some() };
    let mut args = std::env::args().skip(1).peekable();
    while let Some(a) = args.next() {
        if a == "--moderna" {
            // El tamaño es opcional: `--moderna 2560x1440` o `--moderna` a secas (o con `auto`).
            let size = args.peek().and_then(|v| parse_size(v));
            if size.is_some() || args.peek().is_some_and(|v| v == "auto") {
                args.next();
            }
            o.modern = Some(size);
            continue;
        }
        let mut val = |name: &str| args.next().ok_or_else(|| format!("{name} necesita un valor"));
        match a.as_str() {
            "--ventana" => o.cfg.fullscreen = false,
            "--sin-sonido" => o.no_sound = true,
            "--intro" => o.intro = true,
            "--pantalla-completa" => o.cfg.fullscreen = true,
            "--exclusivo" => o.cfg.exclusive = true,
            "--raton-libre" => o.cfg.confine_cursor = false,
            "--carta" => o.chart = true,
            "--fps" => o.cfg.fps = val("--fps")?.parse().map_err(|_| "--fps inválido")?,
            "--fotogramas" => o.frames = val("--fotogramas")?.parse().map_err(|_| "--fotogramas inválido")?,
            "--captura" => o.capture = Some(PathBuf::from(val("--captura")?)),
            "--guion" => o.script = Script::parse(&val("--guion")?)?,
            "--escena" => o.original.scene = true,
            "--juego" => o.game_dir = Some(PathBuf::from(val("--juego")?)),
            "--idioma" => o.language = Some(val("--idioma")?.to_lowercase()),
            "--resolucion" => {
                let v = val("--resolucion")?;
                let (w, h) = v.split_once('x').ok_or("--resolucion ANCHOxALTO")?;
                o.cfg.width = w.parse().map_err(|_| "--resolucion: ancho inválido")?;
                o.cfg.height = h.parse().map_err(|_| "--resolucion: alto inválido")?;
            }
            "--abrir" => {
                let v = val("--abrir")?;
                let hex = |s: &str| u32::from_str_radix(s.trim_start_matches("0x"), 16).map_err(|_| format!("--abrir: {s} no es hexadecimal"));
                let parts: Vec<&str> = v.split(':').collect();
                if parts.len() < 2 {
                    return Err("--abrir TAMAÑO:CTOR[:MÉTODO...]".into());
                }
                let methods = parts[2..].iter().map(|p| hex(p)).collect::<Result<Vec<_>, _>>()?;
                o.original.windows.push((hex(parts[0])?, hex(parts[1])?, methods));
            }
            s if s.starts_with("--") => return Err(format!("opción desconocida {s}")),
            _ => o.pos.push(a),
        }
    }
    Ok(o)
}

fn main() {
    let direct = std::env::args_os().len() == 1;
    let result = if direct { launch::start() } else { run() };
    if let Err(e) = result {
        eprintln!("p3-juego: {e}");
        if direct {
            launch::show_error(&e);
        }
        std::process::exit(1);
    }
}

/// Resolución moderna efectiva: la pedida o la que sale del monitor ([`p3_mvp::ui_rt::display::auto_resolution`]). En ventana,
/// la ventana toma ese tamaño.
fn modern_resolution(o: &Opts, window: &mut GameWindow) -> Option<(u32, u32)> {
    let (w, h) = match o.modern? {
        Some(size) => size,
        None => {
            let (mw, mh) = window.monitor_size().unwrap_or((1920, 1080));
            let (w, h, k) = p3_mvp::ui_rt::display::auto_resolution(mw, mh);
            eprintln!("monitor {mw}x{mh}: resolución moderna {w}x{h} (ampliada x{k})");
            (w, h)
        }
    };
    window.set_windowed_size(w, h);
    Some((w, h))
}

fn run() -> Result<(), String> {
    let mut o = parse()?;
    // Ejecutable, EXE original y `.cpr` a la caché del sistema en otro hilo (disco duro mecánico: ver `p3_client::precarga`).
    if matches!(o.pos.first().map(String::as_str), Some("menu" | "original")) {
        // Al final, la música suelta (`Musik\...` en la carpeta del juego, la del `.cpr` y la del EXE: las de `sound`).
        let mut folders: Vec<&std::path::Path> = o.game_dir.iter().map(PathBuf::as_path).collect();
        folders.extend(o.pos.get(2).map(std::path::Path::new));
        folders.extend(o.pos.get(1).map(std::path::Path::new));
        const MUSIC: &[&str] = &["mp3", "wav"];
        let later = p3_mvp::ui_rt::miles::default_dirs(&folders).into_iter().map(|d| (d.join("Musik"), MUSIC)).collect();
        p3_client::precarga::warm_with(p3_client::precarga::files(o.pos.get(1).map(String::as_str), o.pos.get(2).map(String::as_str), o.game_dir.as_deref()), later);
    }
    match o.pos.first().map(String::as_str) {
        Some("vista") if o.pos.len() == 3 => {
            let save = std::fs::read(&o.pos[1]).map_err(|e| format!("partida: {e}"))?;
            let archive = p3_assets::cpr::Bytes::map_file(&o.pos[2]).map_err(|e| format!("recursos: {e}"))?;
            let t = std::time::Instant::now();
            let session = p3_mvp::Session::open(&save, archive)?;
            eprintln!("partida abierta en {:.2} s", t.elapsed().as_secs_f64());
            o.cfg.system_cursor = true;
            o.cfg.title = "Patrician III (Rust)".into();
            let mut window = GameWindow::open(o.cfg.clone())?;
            let mut view = View::new(session, if o.chart { Mode::Chart } else { Mode::City });
            if let Some(size) = modern_resolution(&o, &mut window) {
                view.size = size;
            }
            let r = view.run(&mut window, o.frames, &mut o.script, o.capture.as_deref());
            for l in &view.log {
                eprintln!("{l}");
            }
            r
        }
        Some("original") if (3..=4).contains(&o.pos.len()) => original(&mut o),
        Some("menu") if o.pos.len() == 3 => menu(&mut o),
        Some("prueba") => test_pattern(&mut o),
        _ => Err("uso: p3-juego vista PARTIDA.pat RECURSOS.cpr [--ventana] [--exclusivo] [--carta] [--moderna [ANCHOxALTO]] [--fps N] [--fotogramas N] [--captura RUTA.png] [--guion \"F:orden;...\"]\n       p3-juego menu Patrician3.exe RECURSOS.cpr [--moderna [ANCHOxALTO]] [--idioma CÓDIGO] [opciones]\n       p3-juego original Patrician3.exe RECURSOS.cpr [PARTIDA.pat] [--escena] [--abrir TAMAÑO:CTOR[:MÉTODO...]] [opciones]\n       p3-juego prueba [opciones]".into()),
    }
}

fn original(o: &mut Opts) -> Result<(), String> {
    let exe = std::fs::read(&o.pos[1]).map_err(|e| format!("ejecutable original: {e}"))?;
    let cpr = p3_assets::cpr::Bytes::map_file(&o.pos[2]).map_err(|e| format!("recursos: {e}"))?;
    let session = match o.pos.get(3) {
        Some(p) => {
            let save = std::fs::read(p).map_err(|e| format!("partida: {e}"))?;
            Some(p3_mvp::Session::open_with_paused_battle_and_crt(&save, cpr.clone(), 1)?)
        }
        None => None,
    };
    let archive = p3_assets::cpr::Archive::parse(&cpr).map_err(|e| format!("CPR: {e:?}"))?;
    let mut mem = p3_mvp::ui_rt::PageMem::default();
    let (w, h) = (o.cfg.width as usize, o.cfg.height as usize);
    let mut rt = p3_mvp::ui_rt::Runtime::new(&mut mem, &exe, &archive, w, h)?;
    rt.surface = p3_mvp::ui_rt::Surface::new(w, h, [0, 0, 0, 0xFF]);
    live_limits(&mut rt, o);
    let _sound = sound(&mut rt, o);
    o.cfg.title = "Patrician III".into();
    // Hasta que se carguen los cursores del juego (`screen_list_load_cursors` 0x4B8940, arranque del menú) se ve el del sistema.
    o.cfg.system_cursor = true;
    let window = GameWindow::open(o.cfg.clone())?;
    p3_client::original::attach(&mut rt, window)?;
    let r = p3_client::original::run_minimal(&mut rt, &mut mem, &o.original, session.as_ref(), o.frames);
    eprintln!("importaciones sin sustituto: {:?}", rt.import_missing);
    eprintln!("SGL sin sustituto: {:?}", rt.sgl_missing);
    eprintln!("faltan (función, llamante): {:x?}", rt.missing_from);
    let (res, frames) = r?;
    eprintln!("run_frame devolvió {res:#x} tras {frames} fotogramas");
    if let Some(p) = &o.capture {
        rt.surface.write_png(p)?;
    }
    Ok(())
}

/// Ajustes del enlazador para jugar en vivo. El tope de llamadas cuenta por fotograma (se reinicia en cada `sgl_SwapBuffer`), y
/// empezar una partida nueva o cargar una hace ~2 millones de llamadas sin presentar: con el tope de las pruebas (2 M) el
/// enlazador abortaba y la ventana se cerraba al pulsar «Start». La carpeta del juego (`--juego`, si no la del `.cpr`) es donde
/// el original busca y escribe `save\ein\*.pat`.
fn live_limits(rt: &mut p3_mvp::ui_rt::Runtime<'_>, o: &Opts) {
    rt.call_limit = 1_000_000_000;
    // Tirones: qué funciones del original ocupan cada fotograma de más de 150 ms (`p3-juego.log`, «tirón: …»).
    rt.enable_stall_sampler();
    rt.menus.game_dir = o.game_dir.clone().or_else(|| std::path::Path::new(&o.pos[2]).parent().map(PathBuf::from));
    if let Some(d) = &rt.menus.game_dir {
        eprintln!("carpeta del juego (partidas en save\\ein): {}", d.display());
    }
}

/// Sonido del original (`p3_client::audio`): la música suelta (`Musik\...`) se busca en la carpeta del juego, la del `.cpr` y la
/// del EXE. Hay que conservar el valor devuelto mientras dure la partida.
fn sound(rt: &mut p3_mvp::ui_rt::Runtime<'_>, o: &Opts) -> Option<p3_client::audio::AudioOut> {
    let mut folders: Vec<&std::path::Path> = o.game_dir.iter().map(PathBuf::as_path).collect();
    folders.extend([std::path::Path::new(&o.pos[2]), std::path::Path::new(&o.pos[1])]);
    // Los vídeos de las ventanas de suceso (`Videos\*.sks`) se buscan en las mismas carpetas, haya sonido o no.
    rt.vto.dirs = p3_mvp::ui_rt::miles::default_dirs(&folders);
    if o.intro {
        intro(rt);
    }
    p3_client::audio::attach(rt, &folders, o.no_sound)
}

/// Introducción en Bink (`--intro`): `game_main_window_initialize` (`0x00463821`) lanza `binkplay.exe Intro_1.bik Intro_2.bik
/// Intro_3.bik /d408 /b2 /k ...` desde la carpeta del juego y espera a que termine (una tecla la salta). Se lanza el mismo
/// `binkplay.exe` de la instalación; sin él (o fuera de Windows) falla como antes y el juego sigue sin intro.
fn intro(rt: &mut p3_mvp::ui_rt::Runtime<'_>) {
    let Some(dir) = rt.menus.game_dir.clone() else { return };
    rt.on_create_process = Some(Box::new(move |line: &str| {
        let mut parts = line.split_whitespace();
        let Some(exe) = parts.next() else { return false };
        if !exe.eq_ignore_ascii_case("binkplay.exe") {
            return false;
        }
        let status = std::process::Command::new(dir.join(exe)).args(parts).current_dir(&dir).status();
        eprintln!("intro: {line} -> {status:?}");
        status.is_ok()
    }));
}

/// Idioma (`p3_client::idioma`): lee `idioma.txt` de la carpeta del juego (o `--idioma CÓDIGO`) y cambia `o.pos[2]` por el CPR de
/// ese idioma.
fn select_language(o: &mut Opts) -> p3_client::idioma::Setup {
    let base = PathBuf::from(&o.pos[2]);
    let game = o.game_dir.clone().or_else(|| base.parent().map(PathBuf::from)).unwrap_or_default();
    o.game_dir.get_or_insert_with(|| game.clone());
    let setup = p3_client::idioma::prepare(std::path::Path::new(&o.pos[1]), &base, &game, o.language.as_deref(), &[]);
    o.pos[2] = setup.cpr.to_string_lossy().into_owned();
    setup
}

/// Arranque completo del original con la ventana nativa: `ui_rt::menus::boot` (ctor `0x463180` + `game_main_window_initialize`
/// `0x463240`) y `ui_rt::menus::run` (`game_main_window_run` `0x463D20`, menú principal y partida con su bucle modal).
fn menu(o: &mut Opts) -> Result<(), String> {
    let exe = std::fs::read(&o.pos[1]).map_err(|e| format!("ejecutable original: {e}"))?;
    let language = select_language(o);
    let cpr = p3_assets::cpr::Bytes::map_file(&o.pos[2]).map_err(|e| format!("recursos: {e}"))?;
    let archive = p3_assets::cpr::Archive::parse(&cpr).map_err(|e| format!("CPR: {e:?}"))?;
    let mut mem = p3_mvp::ui_rt::PageMem::default();
    let (w, h) = (o.cfg.width as usize, o.cfg.height as usize);
    let mut rt = p3_mvp::ui_rt::Runtime::new(&mut mem, &exe, &archive, w, h)?;
    rt.surface = p3_mvp::ui_rt::Surface::new(w, h, [0, 0, 0, 0xFF]);
    live_limits(&mut rt, o);
    let tramo = p3_mvp::ui_rt::stall::tag("arranque-sonido");
    let _sound = sound(&mut rt, o);
    drop(tramo);
    o.cfg.title = "Patrician III".into();
    let tramo = p3_mvp::ui_rt::stall::tag("arranque-ventana");
    let mut window = GameWindow::open(o.cfg.clone())?;
    drop(tramo);
    // Resolución moderna: los menús siguen a 800×600 y la partida usa la pedida (`ui_rt::display`).
    rt.display.modern = modern_resolution(o, &mut window);
    let monitor = window.monitor_size().unwrap_or((1920, 1080));
    p3_client::idioma::apply(&mut rt, &mut mem, &language);
    let video_file = rt.menus.game_dir.as_ref().map(|d| d.join("video-mode.txt"));
    rt.enable_video_options(video_file, monitor, o.modern);
    if let Some((w, h)) = rt.display.modern { window.set_windowed_size(w, h); }
    p3_client::original::attach_scripted(&mut rt, window, std::mem::take(&mut o.script))?;
    let max = o.frames;
    // `--fotogramas N`: corta la ejecución al llegar a N (como agotar `call_limit`). `P3_CURSOR_TRACE`: cursor del juego por fotograma.
    let trace = std::env::var_os("P3_CURSOR_TRACE").is_some();
    if max != 0 || trace {
        rt.menus.frame_script = Some(Box::new(move |m, frame| {
            if trace {
                eprintln!("fotograma {frame}: cursor ({}, {}) botones {:#x} {:#x}", m.r32(0x006D_AB50) as i32, m.r32(0x006D_AB48) as i32, m.r8(0x006D_CBC1), m.r8(0x006D_CBC2));
            }
            max == 0 || u64::from(frame) < max
        }));
    }
    if let Ok(d) = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
        eprintln!("arranque (unix): {}", d.as_secs());
    }
    let tramo = p3_mvp::ui_rt::stall::tag("arranque-juego");
    let booted = p3_mvp::ui_rt::menus::boot(&mut rt, &mut mem);
    if booted { rt.restore_video_choice(&mut mem); }
    drop(tramo);
    eprintln!("game_main_window_initialize: {}", if booted { "bien" } else { "falló" });
    let r = if booted { Some(p3_mvp::ui_rt::menus::run(&mut rt, &mut mem)) } else { None };
    let secs = |t: std::time::SystemTime| t.duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs());
    eprintln!("fotogramas {} resultado {r:x?} abortado {} montón {:#x} fin (unix) {}", rt.menus.frames, rt.aborted, rt.heap, secs(std::time::SystemTime::now()));
    eprintln!("importaciones sin sustituto: {:?}", rt.import_missing);
    eprintln!("SGL sin sustituto: {:?}", rt.sgl_missing);
    eprintln!("faltan (función, llamante): {:x?}", rt.missing_from);
    if let Some(p) = &o.capture {
        rt.surface.write_png(p)?;
    }
    Ok(())
}

/// Patrón de prueba: rejilla y cursor, con los mensajes recibidos en la salida de error. Comprueba ventana, ritmo y entrada sin
/// datos del juego.
fn test_pattern(o: &mut Opts) -> Result<(), String> {
    o.cfg.title = "p3-juego (prueba)".into();
    let mut window = GameWindow::open(o.cfg.clone())?;
    let (w, h) = (o.cfg.width as usize, o.cfg.height as usize);
    let mut rgba = vec![0u8; w * h * 4];
    let mut queue = VecDeque::new();
    let mut input = win32::InputState::default();
    let mut seen = 0usize;
    loop {
        if !window.poll(&mut queue, &mut input) {
            break;
        }
        o.script.apply(window.frames, &mut queue, &mut input);
        for m in queue.drain(..) {
            seen += 1;
            eprintln!("msg {:#06x} w={:#x} l={:#x} t={} pt={:?}", m.message, m.wparam, m.lparam, m.time, m.pt);
            if m.message == win32::WM_KEYDOWN && m.wparam == u32::from(win32::VK_ESCAPE) {
                return Ok(());
            }
        }
        let t = window.frames as usize;
        for y in 0..h {
            for x in 0..w {
                let p = &mut rgba[(y * w + x) * 4..(y * w + x) * 4 + 4];
                let grid = x % 64 == 0 || y % 64 == 0;
                let c = if grid { [80, 80, 80] } else { [((x + t) % 256) as u8 / 3, (y % 256) as u8 / 3, 40] };
                p.copy_from_slice(&[c[0], c[1], c[2], 255]);
            }
        }
        let (cx, cy) = input.cursor;
        for d in -8..=8i32 {
            for (x, y) in [(cx + d, cy), (cx, cy + d)] {
                if (0..w as i32).contains(&x) && (0..h as i32).contains(&y) {
                    rgba[(y as usize * w + x as usize) * 4..][..3].copy_from_slice(&[255, 255, 255]);
                }
            }
        }
        window.show_rgba(w, h, &rgba)?;
        window.wait_frame();
        if o.frames != 0 && window.frames >= o.frames {
            break;
        }
    }
    eprintln!("{} fotogramas, {:.1} fps, {seen} mensajes", window.frames, window.measured_fps);
    if let Some(p) = &o.capture {
        p3_render::write_png(p, w, h, &rgba).map_err(|e| format!("captura: {e}"))?;
    }
    Ok(())
}
