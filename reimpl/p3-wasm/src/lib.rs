//! Punto de entrada mínimo de WebAssembly (WEB-0/WEB-1).
//!
//! Dos bancos sin pantalla que corren igual en nativo y en WASM y devuelven JSON con hashes FNV-1a/SHA-256:
//! - [`ui_bench`]: el escenario `menu`/`city` de `p3-mvp/examples/ui_benchmark.rs` (ui_rt, 800x600).
//! - [`sim_run`]: un tramo de `p3-sim` como `c10_snapshots` (hash SHA-256 de la carga útil de cada día).
//!
//! Los bytes del EXE, del CPR y de la partida los pone el anfitrión en memoria; aquí no se lee ningún archivo.
use p3_mvp::ui_rt::{menus, win32, PageMem, Runtime};
use sha2::{Digest, Sha256};
use std::{cell::RefCell, rc::Rc};

/// Marca de tiempo por fotograma para medir (no entra en el JSON ni en los hashes). En WASM la toma el anfitrión
/// (importación `p3.mark`); en nativo se imprime en stderr si existe `P3_MARK`.
#[cfg(target_arch = "wasm32")]
fn mark(frame: u32) {
    #[link(wasm_import_module = "p3")]
    extern "C" {
        #[link_name = "mark"]
        fn p3_mark(frame: u32);
    }
    unsafe { p3_mark(frame) }
}
#[cfg(not(target_arch = "wasm32"))]
fn mark(frame: u32) {
    use std::sync::OnceLock;
    static T0: OnceLock<Option<std::time::Instant>> = OnceLock::new();
    if let Some(t0) = T0.get_or_init(|| std::env::var_os("P3_MARK").map(|_| std::time::Instant::now())) {
        eprintln!("mark {frame} {:.1}", t0.elapsed().as_secs_f64() * 1000.0);
    }
}

const FNV_BASIS: u64 = 0xcbf29ce484222325;
fn fnv(bytes: &[u8]) -> u64 {
    bytes.iter().fold(FNV_BASIS, |h, b| (h ^ u64::from(*b)).wrapping_mul(0x100000001b3))
}

/// Banco de interfaz: `scenario` = `menu` o `city`; devuelve JSON con el hash FNV-1a de cada fotograma y de la memoria.
pub fn ui_bench(exe: &[u8], cpr: &[u8], scenario: &str, frames: u32) -> Result<String, String> {
    let archive = p3_assets::cpr::Archive::parse(cpr).map_err(|e| format!("{e:?}"))?;
    ui_bench_archive(exe, &archive, scenario, frames)
}

/// Como [`ui_bench`] con el CPR ya abierto (entero en memoria o por tramos, como en la web).
pub fn ui_bench_archive(exe: &[u8], archive: &p3_assets::cpr::Archive<'_>, scenario: &str, frames: u32) -> Result<String, String> {
    mark(0);
    let mut mem = PageMem::default();
    let mut rt = Runtime::new(&mut mem, exe, archive, 800, 600).map_err(|e| format!("{e:?}"))?;
    rt.call_limit = 1_000_000_000;
    if !menus::boot(&mut rt, &mut mem) {
        return Err("menus::boot falló".into());
    }
    let city = scenario == "city";
    let records = Rc::new(RefCell::new(Vec::new()));
    let output = records.clone();
    let mut frame = 0u32;
    rt.on_present = Some(Box::new(move |surface, queue, input| {
        frame += 1;
        mark(frame);
        output.borrow_mut().push(serde_json::json!({"frame":frame,"width":surface.width,"height":surface.height,"pixels":format!("{:016x}",fnv(&surface.rgba))}));
        input.time_ms = Some(frame * 40);
        if city {
            let (x, y) = (685, 561);
            if frame == 10 {
                input.cursor = (x, y);
                win32::post(queue, input, win32::WM_MOUSEMOVE, 0, win32::mouse_lparam(x, y));
            }
            if frame == 11 || frame == 12 {
                let down = frame == 11;
                input.set_key(win32::VK_LBUTTON, down);
                win32::post(queue, input, if down { win32::WM_LBUTTONDOWN } else { win32::WM_LBUTTONUP }, input.mouse_keys(), win32::mouse_lparam(x, y));
            }
        }
        true
    }));
    rt.menus.frame_script = Some(Box::new(move |m, f| {
        if city && f == 3 {
            m.w32(menus::MAIN_MENU + 0x284, 2);
        }
        f < frames
    }));
    menus::run(&mut rt, &mut mem);
    #[cfg(not(target_arch = "wasm32"))]
    if std::env::var_os("P3_MARK").is_some() {
        eprintln!("mem redecodes {}", rt.assets.redecodes);
        for (k, v) in rt.memory_report(&mem) {
            eprintln!("mem {k} {:.1} MiB", v as f64 / 1048576.0);
        }
    }
    let stack = p3_mvp::ui_rt::STACK_TOP;
    Ok(serde_json::json!({"scenario":scenario,"frames":records.borrow().as_slice(),"missing":format!("{:x?}",rt.missing_from),"towns":mem.u32_at(0x6de4b0),
        "memory":format!("{:016x}",mem.content_hash()),"memory_no_stack":format!("{:016x}",mem.content_hash_outside(stack-0x0100_0000..stack+0x1000)),"calls":rt.calls}).to_string())
}

/// Tramo de simulación: abre la partida, avanza `days` días con la política de `c10_snapshots --human-battles-auto
/// --battle-pacing clock` y devuelve una fila por día con SHA-256 y FNV-1a de la carga útil guardada.
pub fn sim_run(save: &[u8], cpr: &[u8], exe: &[u8], days: u32) -> Result<String, String> {
    let mut session = p3_mvp::Session::open(save, cpr.to_vec())?;
    session.sim.configure_crt_state(123);
    session.sim.host.celebration_state = Some(p3_core::economy::celebration::CelebrationState { next: 0, town: 255 });
    session.sim.host.investigation_local_mode = true;
    session.sim.host.siege_headless_march = true;
    session.sim.host.headless_human_battles = true;
    session.configure_battle_texts(exe)?;
    session.sim.configure_election_texts(exe)?;
    session.sim.configure_hunt_texts(exe)?;
    session.enable_full_fleet()?;
    let pacing = p3_mvp::headless::BattlePacing::WithClock { frame_ms: 40 };
    let start = session.sim.idle.clock.tick;
    let mut rows = Vec::new();
    for day in 0..=days {
        if day > 0 {
            let target = start + day * 256;
            let remaining = target - session.sim.idle.clock.tick;
            p3_mvp::headless::advance_ticks_with(&mut session, remaining, pacing)?;
        }
        let saved = session.save()?;
        let payload = p3_save::Save::parse(&saved).map_err(|e| e.to_string())?.to_unpacked();
        rows.push(serde_json::json!({"day":day,"tick":session.sim.idle.clock.tick,"bytes":payload.len(),
            "sha256":format!("{:x}",Sha256::digest(&payload)),"fnv1a":format!("{:016x}",fnv(&payload))}));
    }
    Ok(serde_json::json!({"start_tick":start,"days":rows}).to_string())
}

// --- Exportaciones C para el anfitrión (Node o navegador) -------------------------------------------------------

/// Reserva `len` bytes en la memoria lineal para que el anfitrión copie un archivo.
#[no_mangle]
pub extern "C" fn p3_alloc(len: usize) -> *mut u8 {
    let mut v = Vec::<u8>::with_capacity(len.max(1));
    let ptr = v.as_mut_ptr();
    std::mem::forget(v);
    ptr
}

/// Libera un bloque de `p3_alloc`.
///
/// # Safety
/// `ptr`/`len` deben venir de `p3_alloc(len)`.
#[no_mangle]
pub unsafe extern "C" fn p3_free(ptr: *mut u8, len: usize) {
    drop(Vec::from_raw_parts(ptr, 0, len.max(1)));
}

thread_local! { static OUT: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) }; }

thread_local! { static PANIC: RefCell<String> = const { RefCell::new(String::new()) }; }

/// Mensaje (y lugar) del último pánico: en wasm32 un pánico aborta, pero el anfitrión puede leerlo tras el trap.
fn install_panic_hook() {
    std::panic::set_hook(Box::new(|info| PANIC.with(|p| *p.borrow_mut() = info.to_string())));
}

/// Escribe el último pánico en el búfer de resultado y devuelve su longitud (0 si no hubo).
#[no_mangle]
pub extern "C" fn p3_last_panic() -> usize {
    PANIC.with(|p| OUT.with(|o| {
        *o.borrow_mut() = p.borrow().clone().into_bytes();
        o.borrow().len()
    }))
}

fn finish(r: Result<String, String>) -> usize {
    let text = match r {
        Ok(s) => s,
        Err(e) => serde_json::json!({"error": e}).to_string(),
    };
    OUT.with(|o| {
        *o.borrow_mut() = text.into_bytes();
        o.borrow().len()
    })
}

/// Puntero al último resultado JSON (su longitud la devolvió la función que lo produjo).
#[no_mangle]
pub extern "C" fn p3_result_ptr() -> *const u8 {
    OUT.with(|o| o.borrow().as_ptr())
}

unsafe fn slice<'a>(ptr: *const u8, len: usize) -> &'a [u8] {
    std::slice::from_raw_parts(ptr, len)
}

/// Autocomprobación de las rutas vectoriales dentro del `.wasm` (hoy, las filas de `blit`): casos comprobados, o 0 si alguno
/// difiere de la ruta escalar (el detalle queda en el búfer de resultado).
#[no_mangle]
pub extern "C" fn p3_selftest() -> u32 {
    match p3_mvp::ui_rt::blit_self_check() {
        Ok(n) => n,
        Err(e) => {
            finish(Err(e));
            0
        }
    }
}

/// Banco de interfaz. `scenario`: 0 = menu, 1 = city. Devuelve la longitud del JSON.
///
/// # Safety
/// Los punteros vienen de `p3_alloc` con las longitudes dadas.
#[no_mangle]
pub unsafe extern "C" fn p3_ui_bench(exe: *const u8, exe_len: usize, cpr: *const u8, cpr_len: usize, scenario: u32, frames: u32) -> usize {
    install_panic_hook();
    finish(ui_bench(slice(exe, exe_len), slice(cpr, cpr_len), if scenario == 1 { "city" } else { "menu" }, frames))
}

/// Tramo de simulación. Devuelve la longitud del JSON.
///
/// # Safety
/// Los punteros vienen de `p3_alloc` con las longitudes dadas.
#[no_mangle]
pub unsafe extern "C" fn p3_sim_run(save: *const u8, save_len: usize, cpr: *const u8, cpr_len: usize, exe: *const u8, exe_len: usize, days: u32) -> usize {
    install_panic_hook();
    finish(sim_run(slice(save, save_len), slice(cpr, cpr_len), slice(exe, exe_len), days))
}
