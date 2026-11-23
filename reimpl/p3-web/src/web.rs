//! Pegamento con el Worker (`www/worker.js`). Sin `wasm-bindgen`: importaciones C del módulo `p3` y exportaciones C, de modo que
//! basta `cargo build --target wasm32-unknown-unknown` y un cargador JS pequeño.
use crate::webfs::{FileSource, WebFs};
use crate::{GameOptions, Modern, WebHost};
use p3_assets::cpr::{Archive, RangeRead, StreamedArchive};
use p3_mvp::hostfs::HostFs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};

#[link(wasm_import_module = "p3")]
extern "C" {
    /// Muestra `w × h` píxeles RGBA que están en `ptr` de la memoria de WASM.
    fn p3_present(ptr: *const u8, w: u32, h: u32);
    /// Escribe el siguiente evento (4 × u32) en `ptr`; devuelve 0 si el anillo está vacío.
    fn p3_input_pop(ptr: *mut u32) -> u32;
    /// `Atomics.wait` hasta el siguiente fotograma de la página (o 40 ms).
    fn p3_frame_wait();
    fn p3_now_ms() -> f64;
    /// Mensaje de texto (UTF-8) para la consola de la página; `level` 0 info, 1 error.
    fn p3_log(level: u32, ptr: *const u8, len: u32);
    /// Avisa de que el juego terminó (`code`) o falló.
    fn p3_finished(code: i32);
    /// El anillo de audio de WASM está en `ptr` (`bytes`): la página crea su gemelo en un SharedArrayBuffer para el worklet.
    fn p3_audio_attach(ptr: *const u8, bytes: u32);
    /// Sincroniza ambos anillos (lectura del worklet -> WASM, lo mezclado -> SharedArrayBuffer).
    fn p3_audio_sync(ptr: *const u8);
}

fn log(level: u32, s: &str) {
    unsafe { p3_log(level, s.as_ptr(), s.len() as u32) }
}

/// `audio_rate`: frecuencia del `AudioContext` (0 sin sonido).
#[derive(Clone, Copy)]
pub struct BrowserHost {
    pub audio_rate: u32,
}

impl WebHost for BrowserHost {
    fn present(&mut self, rgba: &[u8], w: usize, h: usize) {
        unsafe { p3_present(rgba.as_ptr(), w as u32, h as u32) }
    }
    fn pop_event(&mut self) -> Option<[u32; 4]> {
        let mut ev = [0u32; 4];
        (unsafe { p3_input_pop(ev.as_mut_ptr()) } != 0).then_some(ev)
    }
    fn wait_frame(&mut self) {
        unsafe { p3_frame_wait() }
    }
    fn now_ms(&mut self) -> u32 {
        unsafe { p3_now_ms() as u64 as u32 }
    }
    fn log(&self, msg: &str) {
        log(0, msg)
    }
    fn audio_rate(&self) -> u32 {
        self.audio_rate
    }
    fn audio_attach(&self, ring_ptr: usize, bytes: usize) {
        unsafe { p3_audio_attach(ring_ptr as *const u8, bytes as u32) }
    }
    fn audio_sync(&self, ring_ptr: usize) {
        unsafe { p3_audio_sync(ring_ptr as *const u8) }
    }
}

/// Reserva `len` bytes en la memoria de WASM para que el Worker escriba en ellos un texto (rutas) o un archivo pequeño.
#[no_mangle]
pub extern "C" fn p3_alloc(len: usize) -> *mut u8 {
    let mut v = Vec::<u8>::with_capacity(len.max(1));
    let p = v.as_mut_ptr();
    std::mem::forget(v);
    p
}

/// Libera un bloque de `p3_alloc`.
#[no_mangle]
pub extern "C" fn p3_free(ptr: *mut u8, cap: usize) {
    // SAFETY: `ptr` y `cap` vienen de `p3_alloc(cap)`.
    drop(unsafe { Vec::from_raw_parts(ptr, 0, cap.max(1)) });
}

fn text<'a>(ptr: *const u8, len: usize) -> &'a str {
    // SAFETY: el Worker escribió `len` bytes UTF-8 en un bloque de `p3_alloc`.
    std::str::from_utf8(unsafe { std::slice::from_raw_parts(ptr, len) }).unwrap_or("")
}

#[link(wasm_import_module = "p3")]
extern "C" {
    /// Lee de un archivo abierto del Worker (`FileSystemSyncAccessHandle.read`): hasta `len` bytes desde `offset` a `ptr`;
    /// devuelve cuántos leyó, 0 al final o -1 si falla.
    fn p3_fs_read(id: u32, offset: f64, ptr: *mut u8, len: u32) -> i32;
}

struct JsSource;

impl FileSource for JsSource {
    fn read_at(&self, id: u32, offset: u64, buf: &mut [u8]) -> std::io::Result<usize> {
        // SAFETY: `buf` es memoria de WASM viva durante la llamada; JS escribe como mucho `len` bytes.
        let n = unsafe { p3_fs_read(id, offset as f64, buf.as_mut_ptr(), buf.len().min(i32::MAX as usize) as u32) };
        usize::try_from(n).map_err(|_| std::io::Error::other("lectura de OPFS fallida"))
    }
}

static FS: OnceLock<Arc<WebFs>> = OnceLock::new();

/// Crea el sistema de archivos de solo lectura sobre los archivos que el Worker tiene abiertos y lo instala. `root` es la carpeta
/// del juego que verá el enlazador (`/juego`). Después, una llamada a `p3_fs_add` por archivo y, si se quieren partidas,
/// `p3_save_install` (p3-web-save) antes de `p3_start`.
#[no_mangle]
pub extern "C" fn p3_fs_begin(root_ptr: *const u8, root_len: usize) {
    let fs = FS.get_or_init(|| WebFs::new(text(root_ptr, root_len), Arc::new(JsSource)));
    p3_mvp::hostfs::install(fs.clone());
}

/// Registra el archivo `id` (abierto por el Worker) con su ruta relativa a la carpeta del juego y su tamaño.
#[no_mangle]
pub extern "C" fn p3_fs_add(id: u32, path_ptr: *const u8, path_len: usize, size: f64) {
    if let Some(fs) = FS.get() {
        fs.add(id, text(path_ptr, path_len), size as u64);
    }
}

fn find(fs: &dyn HostFs, dir: &Path, pred: impl Fn(&str) -> bool) -> Option<PathBuf> {
    fs.list(dir).ok()?.into_iter().find(|e| e.is_file && pred(&e.name.to_ascii_lowercase())).map(|e| e.path)
}

fn run(modern: Modern, monitor: (u32, u32), audio_rate: u32) -> Result<Option<u32>, String> {
    let web = FS.get().ok_or("p3_fs_begin no se llamó")?;
    let fs = p3_mvp::hostfs::current();
    let root = web.root().to_path_buf();
    let exe_path = find(&*fs, &root, |n| n == "patrician3.exe").ok_or("falta Patrician3.exe en la carpeta del juego")?;
    let exe = fs.read(&exe_path).map_err(|e| format!("Patrician3.exe: {e}"))?;
    // CPR de cada idioma (`p2arch0_<código>.cpr`); si no hay ninguno con ese nombre, el primer `.cpr`.
    let mut cprs: Vec<(String, Arc<dyn RangeRead>)> = Vec::new();
    for e in fs.list(&root).map_err(|e| format!("carpeta del juego: {e}"))? {
        if let (true, Some(code)) = (e.is_file, p3_mvp::lang_cpr::cpr_code(&e.name)) {
            if let Some(src) = web.source(&e.path) {
                cprs.push((code, Arc::new(src)));
            }
        }
    }
    if cprs.is_empty() {
        let cpr_path = find(&*fs, &root, |n| n.ends_with(".cpr")).ok_or("falta el archivo .cpr de recursos")?;
        cprs.push((p3_mvp::lang_cpr::BASE.into(), Arc::new(web.source(&cpr_path).ok_or("CPR no registrado")?)));
    }
    let (source, language) = crate::select_language(&*fs, &root, cprs, None)?;
    if let Some(l) = &language {
        log(0, &format!("idioma: {} (disponibles: {:?})", l.code, l.options.iter().map(|o| o.0.as_str()).collect::<Vec<_>>()));
    }
    // El CPR no se carga: se lee su índice y cada archivo la primera vez que el juego lo pide.
    let streamed = StreamedArchive::open(source).map_err(|e| format!("CPR: {e:?}"))?;
    let archive = Archive::from_streamed(&streamed);
    let low_memory = LOW_MEMORY.load(std::sync::atomic::Ordering::Relaxed);
    crate::run_game(BrowserHost { audio_rate }, &exe, &archive, &GameOptions { game_dir: root, modern, monitor, log: Some(|s| log(0, s)), low_memory, language })
}

static LOW_MEMORY: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Modo de poca memoria (WEB-6): llamar antes de `p3_start`. El Worker lo enciende en móviles.
#[no_mangle]
pub extern "C" fn p3_set_low_memory(on: u32) {
    LOW_MEMORY.store(on != 0, std::sync::atomic::Ordering::Relaxed);
}

/// Arranca el juego. `audio_rate`: frecuencia del `AudioContext` (0 = sin sonido). `modern`: 0 = el modo guardado o 800×600, 1 = automático según `monitor`, 2 = `w`×`h`. No vuelve hasta cerrar
/// el juego.
#[no_mangle]
pub extern "C" fn p3_start(modern: u32, w: u32, h: u32, monitor_w: u32, monitor_h: u32, audio_rate: u32) {
    std::panic::set_hook(Box::new(|info| log(1, &format!("panic: {info}"))));
    let modern = match modern {
        1 => Modern::Auto,
        2 => Modern::Size(w, h),
        _ => Modern::Saved,
    };
    let code = match run(modern, (monitor_w.max(800), monitor_h.max(600)), audio_rate) {
        Ok(Some(c)) => c as i32,
        Ok(None) => {
            log(1, "game_main_window_initialize falló");
            -1
        }
        Err(e) => {
            log(1, &e);
            -2
        }
    };
    unsafe { p3_finished(code) }
}

// Partidas (WEB-4, `p3-web-save`): un cdylib no exporta los `#[no_mangle]` de sus dependencias, así que se reenvían aquí. El JS está
// en `p3-web-save/js/p3-saves-worker.js` y el módulo `env` de las importaciones lo provee `WorkerSaves.imports()`.
use p3_web_save::wasm as sv;

#[no_mangle]
pub extern "C" fn p3_save_alloc(len: usize) -> *mut u8 {
    sv::p3_save_alloc(len)
}

#[no_mangle]
pub extern "C" fn p3_save_free(ptr: *mut u8, cap: usize) {
    sv::p3_save_free(ptr, cap)
}

#[no_mangle]
pub extern "C" fn p3_save_install(root_ptr: *const u8, root_len: usize) {
    sv::p3_save_install(root_ptr, root_len)
}

#[no_mangle]
pub extern "C" fn p3_save_preload(path_ptr: *const u8, path_len: usize, data_ptr: *const u8, data_len: usize, modified: f64, validate: u32) -> u32 {
    sv::p3_save_preload(path_ptr, path_len, data_ptr, data_len, modified, validate)
}

#[no_mangle]
pub extern "C" fn p3_save_forget(path_ptr: *const u8, path_len: usize) {
    sv::p3_save_forget(path_ptr, path_len)
}

#[no_mangle]
pub extern "C" fn p3_save_inspect(data_ptr: *const u8, data_len: usize) -> u32 {
    sv::p3_save_inspect(data_ptr, data_len)
}

/// Gancho de pruebas: escribe por `HostFs` como lo hace el guardado del juego.
#[no_mangle]
pub extern "C" fn p3_save_hostfs_write(path_ptr: *const u8, path_len: usize, data_ptr: *const u8, data_len: usize) -> u32 {
    sv::p3_save_hostfs_write(path_ptr, path_len, data_ptr, data_len)
}

/// Gancho de pruebas: tamaño visto por `HostFs`, o -1.
#[no_mangle]
pub extern "C" fn p3_save_hostfs_len(path_ptr: *const u8, path_len: usize) -> f64 {
    sv::p3_save_hostfs_len(path_ptr, path_len)
}
