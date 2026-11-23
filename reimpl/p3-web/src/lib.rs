//! Cliente del juego en el navegador (WEB-3, `coordination/ARQUITECTURA_WEB.md`). Hace lo que `p3-client` en Windows: arranca el
//! menú principal con `ui_rt::menus::{boot, run}` y conecta `Runtime::on_present` / `on_pump` con la página. El código es
//! independiente de la plataforma salvo [`web`] (importaciones `p3.*` del Worker, solo `wasm32`); todo lo demás se prueba en nativo
//! con un [`WebHost`] de mentira.
//!
//! Interfaces con los demás hilos web:
//! - **Archivos (WEB-2, `HostFs`)**: hoy el Worker lee el EXE y el CPR de OPFS y los copia a la memoria de WASM ([`web::p3_start`]).
//!   Cuando exista `HostFs`/lector por tramos del CPR, solo cambia cómo [`run_menu`] recibe `cpr` y `rt.menus.game_dir`.
//! - **Sonido (WEB-5)**: [`attach_audio`] + [`wrap_audio_pump`] en `run_game` (WebAudio de `p3-audio`, anillo hacia un `AudioWorklet`).
//! - **Partidas OPFS, PWA, multijugador**: viven en `www/` (página) o en `HostFs`; este crate no los toca.
pub mod input;
pub mod screen;
pub mod webfs;
#[cfg(target_arch = "wasm32")]
pub mod web;

use p3_mvp::ui_rt::{menus, win32, PageMem, Runtime, Surface};
use p3_render::sgl_screen::SglScreen;
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

/// Lo que el navegador ofrece al juego. En `wasm32` lo implementa [`web::BrowserHost`] con importaciones JS.
pub trait WebHost {
    /// Muestra un fotograma RGBA de `w × h` en el lienzo.
    fn present(&mut self, rgba: &[u8], w: usize, h: usize);
    /// Saca el siguiente evento del anillo de entrada (ver [`input`]).
    fn pop_event(&mut self) -> Option<[u32; 4]>;
    /// Bloquea hasta el siguiente fotograma de la página (`requestAnimationFrame`, con tiempo máximo).
    fn wait_frame(&mut self);
    /// `performance.now()` en milisegundos.
    fn now_ms(&mut self) -> u32;
    /// Mensaje para la consola de la página.
    fn log(&self, _msg: &str) {}
    /// Frecuencia del `AudioContext` de la página (0 = sin sonido).
    fn audio_rate(&self) -> u32 {
        0
    }
    /// Avisa a la página del anillo de audio (`ptr`, `bytes` en la memoria de WASM): crea su copia compartida con el worklet.
    fn audio_attach(&self, _ring_ptr: usize, _bytes: usize) {}
    /// Sincroniza el anillo de WASM con el compartido: trae la posición de lectura del worklet y envía lo mezclado.
    fn audio_sync(&self, _ring_ptr: usize) {}
}

/// Latencia del colchón de audio (ms): más es más robusto ante fotogramas lentos, menos es más reactivo.
pub const AUDIO_LATENCY_MS: u32 = 150;

/// Resolución de los menús del juego.
pub const MENU_SIZE: (usize, usize) = (800, 600);

fn drain<H: WebHost>(host: &mut H, game: (usize, usize), queue: &mut std::collections::VecDeque<win32::Msg>, input: &mut win32::InputState) -> bool {
    input.time_ms = Some(host.now_ms());
    let mut alive = true;
    while let Some(w) = host.pop_event() {
        if let Some(ev) = input::Event::decode(w) {
            alive &= input::apply(ev, game, input, queue);
        }
    }
    input.time_ms = Some(host.now_ms());
    alive
}

/// Resolución moderna pedida por la página (como `--moderna` de `p3-juego`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Modern {
    /// Sin petición: vale la elección guardada en `video-mode.txt` (o 800×600).
    #[default]
    Saved,
    /// La de la pantalla ([`p3_mvp::ui_rt::display::auto_resolution`]).
    Auto,
    Size(u32, u32),
}

/// Ajustes del arranque.
#[derive(Clone, Debug)]
pub struct GameOptions {
    /// Carpeta del juego tal como la ve el enlazador (la raíz de `HostFs`): ahí están `save/ein` y `video-mode.txt`.
    pub game_dir: std::path::PathBuf,
    pub modern: Modern,
    /// Tamaño de la pantalla del jugador en píxeles físicos (para `Modern::Auto` y la lista de modos del juego).
    pub monitor: (u32, u32),
    /// Destino de los avisos del enlazador (funciones ausentes, importaciones sin sustituto, abortos): la consola de la página.
    pub log: Option<fn(&str)>,
    /// Poca memoria (móviles, WEB-6): `Runtime::set_low_memory`, que devuelve las páginas de los bloques grandes que el juego libera.
    pub low_memory: bool,
    /// Idioma (como `p3-juego`): paquete de cadenas del EXE y selector de Opciones. Lo prepara [`select_language`].
    pub language: Option<WebLanguage>,
}

/// Idioma elegido en la web.
#[derive(Clone, Debug, Default)]
pub struct WebLanguage {
    pub code: String,
    /// (código, nombre) de los idiomas con CPR en la carpeta del juego, el inglés primero.
    pub options: Vec<(String, String)>,
    pub active: usize,
    /// `save/idioma.txt`: escribible en la web (las partidas viven bajo `save/`).
    pub choice_file: std::path::PathBuf,
    /// Texto de `idioma/<código>.pack` (traducción de las cadenas del EXE), si está.
    pub pack: Option<String>,
}

/// Archivo con la elección de idioma, relativo a la carpeta del juego.
pub const LANGUAGE_CHOICE: &str = "save/idioma.txt";

/// Elige el CPR con que arrancar. `cprs`: (código, lector) de cada `p2arch0_<código>.cpr` de la carpeta del juego. Devuelve el lector
/// del CPR que debe abrir el juego (el inglés, o uno combinado sin copiar nada: [`p3_assets::cpr::build_virtual`] con las entradas de
/// [`p3_mvp::lang_cpr::plan`]) y el idioma para [`GameOptions::language`]. La elección sale de `wanted` o de `save/idioma.txt`.
pub fn select_language(
    fs: &dyn p3_mvp::hostfs::HostFs,
    root: &std::path::Path,
    mut cprs: Vec<(String, std::sync::Arc<dyn p3_assets::cpr::RangeRead>)>,
    wanted: Option<&str>,
) -> Result<(std::sync::Arc<dyn p3_assets::cpr::RangeRead>, Option<WebLanguage>), String> {
    use p3_assets::cpr::{build_virtual, Archive, Piece, StreamedArchive};
    use p3_mvp::lang_cpr::{self, Source, BASE};
    cprs.sort_by(|a, b| (a.0 != BASE, &a.0).cmp(&(b.0 != BASE, &b.0)));
    cprs.dedup_by(|a, b| a.0 == b.0);
    let base = cprs.first().ok_or("falta el archivo .cpr de recursos")?.clone();
    if cprs.len() < 2 || base.0 != BASE {
        return Ok((base.1, None));
    }
    let wanted = wanted.map(str::to_lowercase).or_else(|| fs.read_to_string(&root.join(LANGUAGE_CHOICE)).ok().map(|t| t.trim().to_lowercase())).unwrap_or_else(|| BASE.into());
    let mut active = cprs.iter().position(|c| c.0 == wanted).unwrap_or(0);
    let mut chosen = base.1.clone();
    if active != 0 {
        let lang = &cprs[active];
        let combined = (|| -> Result<_, String> {
            let b = StreamedArchive::open(base.1.clone()).map_err(|e| format!("CPR {BASE}: {e:?}"))?;
            let l = StreamedArchive::open(lang.1.clone()).map_err(|e| format!("CPR {}: {e:?}", lang.0))?;
            let (ba, la) = (Archive::from_streamed(&b), Archive::from_streamed(&l));
            let files: Vec<(Vec<u8>, Piece)> = lang_cpr::plan(&ba, &la, &lang.0)
                .into_iter()
                .map(|(name, src)| {
                    let piece = match src {
                        Source::Base(e) => Piece::Range(base.1.clone(), u64::from(e.offset), e.size),
                        Source::Lang(e) => Piece::Range(lang.1.clone(), u64::from(e.offset), e.size),
                        Source::Owned(v) => Piece::Mem(v.into()),
                    };
                    (name, piece)
                })
                .collect();
            build_virtual(&files).map_err(|e| format!("CPR combinado: {e:?}"))
        })();
        match combined {
            Ok(v) => chosen = std::sync::Arc::new(v),
            Err(e) => {
                eprintln!("idioma {}: {e}; se usa el inglés", lang.0);
                active = 0;
            }
        }
    }
    let code = cprs[active].0.clone();
    let pack = (code != BASE)
        .then(|| [format!("idioma/{code}.pack"), format!("{code}.pack")].iter().find_map(|f| fs.read_to_string(&root.join(f)).ok()))
        .flatten();
    let options = cprs.iter().map(|c| (c.0.clone(), lang_cpr::display_name(&c.0))).collect();
    Ok((chosen, Some(WebLanguage { code, options, active, choice_file: root.join(LANGUAGE_CHOICE), pack })))
}

impl Default for GameOptions {
    fn default() -> Self {
        GameOptions { game_dir: "/juego".into(), modern: Modern::Saved, monitor: (1920, 1080), log: None, low_memory: false, language: None }
    }
}

/// Arranca el juego con el EXE y el CPR del jugador (menú principal, nueva partida, cargar, ciudad y carta náutica: todo el bucle
/// del original, `ui_rt::menus::{boot, run}`) y no vuelve hasta que se cierre. Devuelve el resultado de `game_main_window_run`
/// (o `None` si `game_main_window_initialize` falló). Como `menu()` de `p3-juego`.
pub fn run_game<H: WebHost + Clone + Send + Sync + 'static>(host: H, exe: &[u8], archive: &p3_assets::cpr::Archive<'_>, opts: &GameOptions) -> Result<Option<u32>, String> {
    let mut mem = PageMem::default();
    let (w, h) = MENU_SIZE;
    let mut rt = Runtime::new(&mut mem, exe, archive, w, h)?;
    rt.surface = Surface::new(w, h, [0, 0, 0, 0xFF]);
    // Empezar una partida o cargarla hace ~2 millones de llamadas sin presentar: el tope por fotograma de las pruebas cortaba el juego.
    rt.call_limit = 1_000_000_000;
    rt.menus.game_dir = Some(opts.game_dir.clone());
    // La música suelta (`Musik\...`) la abre Miles buscándola bajo la carpeta del juego.
    rt.miles.dirs = vec![opts.game_dir.clone()];
    rt.set_low_memory(opts.low_memory);
    let explicit = match opts.modern {
        Modern::Saved => None,
        Modern::Auto => Some(None),
        Modern::Size(w, h) => Some(Some((w, h))),
    };
    rt.display.modern = match opts.modern {
        Modern::Saved => None,
        Modern::Auto => {
            let (w, h, _) = p3_mvp::ui_rt::display::auto_resolution(opts.monitor.0, opts.monitor.1);
            Some((w, h))
        }
        Modern::Size(w, h) => Some((w, h)),
    };
    rt.enable_video_options(Some(opts.game_dir.join("save").join("video-mode.txt")), opts.monitor, explicit);
    if let Some(log) = opts.log {
        rt.diag = Some(Box::new(move |s| log(&s)));
    }
    if let Some(lang) = &opts.language {
        if let Some(text) = &lang.pack {
            let (n, skipped, tables) = rt.load_language_pack(&mut mem, text);
            host.log(&format!("idioma {}: {n} cadenas traducidas ({skipped} descartadas), {tables} tablas", lang.code));
        } else if lang.code != p3_mvp::lang_cpr::BASE {
            host.log(&format!("idioma {}: no hay idioma/{}.pack; menús y mensajes del EXE siguen en inglés", lang.code, lang.code));
        }
        rt.language.enable(lang.options.clone(), lang.active, Some(lang.choice_file.clone()));
    }
    let audio = attach_audio(&mut rt, &host);
    attach(&mut rt, host)?;
    if let Some(audio) = audio {
        wrap_audio_pump(&mut rt, audio);
    }
    let booted = menus::boot(&mut rt, &mut mem);
    if !booted {
        return Ok(None);
    }
    rt.restore_video_choice(&mut mem);
    Ok(Some(menus::run(&mut rt, &mut mem)))
}

/// Sonido: mezclador de `p3-audio` con salida a un anillo que la página vacía en un `AudioWorklet`. Devuelve el mezclador y la
/// función de sincronización, o `None` sin audio (la página no pasó frecuencia).
pub type AudioPump = Rc<dyn Fn()>;

pub fn attach_audio<H: WebHost + Clone + Send + Sync + 'static>(rt: &mut Runtime<'_>, host: &H) -> Option<AudioPump> {
    let rate = host.audio_rate();
    if rate == 0 {
        return None;
    }
    let web = p3_audio::web::WebAudio::new(rate, AUDIO_LATENCY_MS);
    let ptr = web.ring_ptr();
    host.audio_attach(ptr, web.ring_bytes());
    rt.miles.mixer = Some(web.mixer());
    // Con el sondeo de órdenes AIL (cargas largas) el anillo se alimenta aunque no haya fotogramas.
    let (h, w) = (host.clone(), web.clone());
    web.mixer().set_service(Some(std::sync::Arc::new(move || {
        h.audio_sync(ptr);
        w.pump();
        h.audio_sync(ptr);
    })));
    let h = host.clone();
    Some(Rc::new(move || {
        h.audio_sync(ptr);
        web.pump();
        h.audio_sync(ptr);
    }))
}

/// Antepone el bombeo de audio a `on_present` y `on_pump` (antes de que el Worker espere al siguiente fotograma).
pub fn wrap_audio_pump(rt: &mut Runtime<'_>, pump: AudioPump) {
    if let Some(mut inner) = rt.on_present.take() {
        let p = pump.clone();
        rt.on_present = Some(Box::new(move |s, q, i| {
            p();
            inner(s, q, i)
        }));
    }
    if let Some(mut inner) = rt.on_pump.take() {
        rt.on_pump = Some(Box::new(move |q, i| {
            pump();
            inner(q, i)
        }));
    }
}

/// Gancho de presentación (`sgl_SwapBuffer`) y de carga (`on_pump`), como `p3_client::original::attach`.
pub fn attach<H: WebHost + 'static>(rt: &mut Runtime<'_>, host: H) -> Result<(), String> {
    rt.zoom.enabled = true;
    let (w, h) = (rt.surface.width as i32, rt.surface.height as i32);
    let mut screen: SglScreen<screen::NativeScreen> = screen::init_screen(w, h, true)?;
    let mut sgl_host = screen::NativeHost::default();
    let host = Rc::new(RefCell::new(host));
    rt.input.time_ms = Some(host.borrow_mut().now_ms());
    // Tamaño del último fotograma presentado: la posición del ratón llega normalizada y se convierte con él, también durante las
    // cargas largas (`on_pump`), donde ya puede ser el de la resolución moderna.
    let size = Rc::new(Cell::new((w as usize, h as usize)));
    let pump = host.clone();
    let pump_size = size.clone();
    let mut last_pump = 0u32;
    rt.on_pump = Some(Box::new(move |queue, input| {
        let mut host = pump.borrow_mut();
        let now = host.now_ms();
        if now.wrapping_sub(last_pump) < 50 {
            return true;
        }
        last_pump = now;
        drain(&mut *host, pump_size.get(), queue, input)
    }));
    rt.on_present = Some(Box::new(move |surface, queue, input| {
        let mut host = host.borrow_mut();
        let (w, h) = (surface.width as i32, surface.height as i32);
        if screen::active(&mut screen).map(|d| d.size()) != Some((surface.width, surface.height)) && screen.switch_screen_mode(w, h, 16) != Ok(1) {
            return false;
        }
        // WEB-6: la página recibe directamente la superficie del enlazador. Los dos búferes del dispositivo (un fotograma entero
        // cada uno) no se rellenan ni se conservan: eran dos copias de pantalla por fotograma y, a 1920×1080, 16 MiB fijos.
        if let Some(dev) = screen::active(&mut screen) {
            if !dev.back.is_empty() {
                (dev.back, dev.front) = (Vec::new(), Vec::new());
            }
        }
        if screen.swap_buffer(&mut sgl_host, 1).is_err() {
            return false;
        }
        let dev = screen::active(&mut screen).unwrap();
        let (sw, sh) = dev.size();
        dev.fresh = false;
        size.set((sw, sh));
        host.present(&surface.rgba, sw, sh);
        host.wait_frame();
        drain(&mut *host, (sw, sh), queue, input)
    }));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    struct Fake {
        events: VecDeque<[u32; 4]>,
        t: u32,
    }
    impl WebHost for Fake {
        fn present(&mut self, _: &[u8], _: usize, _: usize) {}
        fn pop_event(&mut self) -> Option<[u32; 4]> {
            self.events.pop_front()
        }
        fn wait_frame(&mut self) {}
        fn now_ms(&mut self) -> u32 {
            self.t += 7;
            self.t
        }
    }

    #[test]
    fn drain_translates_events_and_sets_the_clock() {
        let mut host = Fake { events: VecDeque::from([[input::EV_MOVE, 0, 0, 0], [input::EV_MOVE, 65535, 65535, 0], [input::EV_QUIT, 0, 0, 0]]), t: 0 };
        let (mut q, mut input) = (VecDeque::new(), win32::InputState::default());
        assert!(!drain(&mut host, (800, 600), &mut q, &mut input));
        assert_eq!(input.cursor, (799, 599));
        assert_eq!(q.len(), 1);
        assert!(input.time_ms.unwrap() > 0);
    }

    /// Con los CPR reales: `P3_IDIOMA_DIR` (carpeta con `p2arch0_eng.cpr` y `p2arch0_spa.cpr`). Si además `P3_IDIOMA_DERIVED` apunta
    /// al CPR que escribió `p3-juego` (`idioma/p2arch0_spa.cpr`), el combinado de la web debe ser idéntico byte a byte.
    #[test]
    fn real_spanish_virtual_cpr() {
        use p3_assets::cpr::{Archive, FileRange, RangeRead, StreamedArchive};
        use std::sync::Arc;
        let Some(dir) = std::env::var_os("P3_IDIOMA_DIR").map(std::path::PathBuf::from) else { return };
        let src = |n: &str| -> Arc<dyn RangeRead> { Arc::new(FileRange::open(dir.join(n)).unwrap()) };
        let cprs = vec![("spa".to_string(), src("p2arch0_spa.cpr")), ("eng".to_string(), src("p2arch0_eng.cpr"))];
        let fs = p3_mvp::hostfs::current();
        let (cpr, lang) = select_language(&*fs, &dir, cprs, Some("spa")).unwrap();
        let lang = lang.unwrap();
        assert_eq!(lang.code, "spa");
        assert_eq!(lang.options.iter().map(|o| o.0.as_str()).collect::<Vec<_>>(), ["eng", "spa"]);
        let st = StreamedArchive::open(cpr.clone()).unwrap();
        let a = Archive::from_streamed(&st);
        let credits = a.find(b"scripts/credits_eng.ini").and_then(|e| a.read(e)).unwrap();
        assert!(!credits.is_empty());
        assert!(a.find(b"fonts/elgreco24.aim").is_some());
        if let Some(derived) = std::env::var_os("P3_IDIOMA_DERIVED") {
            let want = FileRange::open(derived).unwrap();
            assert_eq!(cpr.len(), want.len(), "tamaño");
            let (mut x, mut y) = (vec![0u8; 8 << 20], vec![0u8; 8 << 20]);
            let mut at = 0u64;
            while at < want.len() {
                let n = (want.len() - at).min(x.len() as u64) as usize;
                cpr.read_at(at, &mut x[..n]).unwrap();
                want.read_at(at, &mut y[..n]).unwrap();
                assert!(x[..n] == y[..n], "difiere en el tramo que empieza en {at}");
                at += n as u64;
            }
        }
    }
}
