//! Conexión de la ventana nativa con el enlazador de la interfaz original (`p3_mvp::ui_rt`).
use crate::device::{self, NativeHost, NativeScreen};
use crate::window::GameWindow;
use p3_mvp::ui_rt::Runtime;
use p3_render::sgl_screen::SglScreen;

/// Instala en `rt` el gancho de presentación: en cada `sgl_SwapBuffer` del código original la superficie del enlazador pasa por
/// el `SwapBuffer` de la capa SGL portada (dispositivo [`NativeScreen`]), se muestra en `window`, se espera al siguiente
/// fotograma y los eventos nuevos entran en `rt.messages`/`rt.input`. Cerrar la ventana encola `WM_QUIT`.
pub fn attach(rt: &mut Runtime<'_>, window: GameWindow) -> Result<(), String> {
    attach_scripted(rt, window, crate::view::Script::default())
}

/// [`attach`] con entrada guionizada (`--guion`): en cada fotograma se añaden los mensajes del guion tras los de la ventana.
pub fn attach_scripted(rt: &mut Runtime<'_>, window: GameWindow, mut script: crate::view::Script) -> Result<(), String> {
    rt.zoom.enabled = true;
    let (w, h) = (rt.surface.width as i32, rt.surface.height as i32);
    let mut screen: SglScreen<NativeScreen> = device::init_screen(w, h, !window.is_fullscreen())?;
    let mut host = NativeHost::default();
    rt.input.time_ms = Some(window.now_ms());
    let window = std::rc::Rc::new(std::cell::RefCell::new(window));
    let loading_window = window.clone();
    let mut last_pump = std::time::Instant::now();
    rt.on_pump = Some(Box::new(move |queue, input| {
        if last_pump.elapsed() < std::time::Duration::from_millis(50) { return true; }
        last_pump = std::time::Instant::now();
        loading_window.borrow_mut().poll(queue, input)
    }));
    rt.on_present = Some(Box::new(move |surface, queue, input| {
        let mut window = window.borrow_mut();
        // Cambio de modo de vídeo del juego (`set_ddraw_resolution`, o la resolución moderna al entrar en partida): la superficie
        // del enlazador ya tiene el tamaño nuevo y el dispositivo cambia con `SwitchScreenMode`, como hacía la DLL.
        let (w, h) = (surface.width as i32, surface.height as i32);
        let tramo = p3_mvp::ui_rt::stall::tag("cambio-de-modo");
        if device::active(&mut screen).map(|d| d.size()) != Some((surface.width, surface.height)) && screen.switch_screen_mode(w, h, 16) != Ok(1) {
            eprintln!("SwitchScreenMode {w}x{h}: fallo");
            return false;
        }
        drop(tramo);
        let tramo = p3_mvp::ui_rt::stall::tag("swap-sgl");
        if let Some(dev) = device::active(&mut screen) {
            if dev.back.len() == surface.rgba.len() {
                dev.back.copy_from_slice(&surface.rgba);
            }
        }
        if screen.swap_buffer(&mut host, 1).is_err() {
            return false;
        }
        drop(tramo);
        let dev = device::active(&mut screen).unwrap();
        let (sw, sh) = dev.size();
        dev.fresh = false;
        let alive = window.present(sw, sh, &dev.front, queue, input);
        script.apply(window.frames, queue, input);
        if std::env::var_os("P3_MSG_TRACE").is_some() {
            for m in queue.iter() {
                eprintln!("fotograma {} msg {:#06x} w={:#x} l={:#x}", window.frames, m.message, m.wparam, m.lparam);
            }
        }
        alive
    }));
    Ok(())
}

/// Opciones del arranque mínimo de la interfaz original.
#[derive(Debug, Default, Clone)]
pub struct OriginalOptions {
    /// Crear la escena global `[0x6E51AC]` (`Runtime::create_scene`).
    pub scene: bool,
    /// Partida que se siembra en las direcciones del original (`Runtime::seed_session`).
    pub save: Option<Vec<u8>>,
    /// Ventanas que se abren dentro de la pantalla contenedora: (tamaño, constructor, métodos a llamar después).
    pub windows: Vec<(u32, u32, Vec<u32>)>,
}

/// `screen_list_run_frame` (`0x4B8A40`) sobre la lista de pantallas global `0x6DA5F0`.
pub const RUN_FRAME: u32 = 0x004B_8A40;
pub const SCREEN_LIST: u32 = 0x006D_A5F0;
/// Bandera de salida que pone `run_frame` al leer `WM_QUIT`.
pub const QUIT_FLAG: u32 = 0x006D_A846;

/// Arranque mínimo: pantalla contenedora (`screen_window` ctor `0x4B4920`, tamaño `0x4024B0`) apilada en la lista de pantallas
/// (`0x4B90E0`), las ventanas pedidas colgadas de ella (`0x4B4E30`), y el bucle del original: `screen_list_run_frame` hasta que
/// devuelva algo distinto de -1 (como `screen_window_run_modal_loop` `0x4B70C0`) o se cierre la ventana. Devuelve el último
/// resultado de `run_frame` y el número de fotogramas. El arranque completo (menú principal, `game_main_window_run`) es del hilo
/// del menú (`ui_rt/menus.rs`).
pub fn run_minimal(rt: &mut Runtime<'_>, mem: &mut p3_mvp::ui_rt::PageMem, opts: &OriginalOptions, session: Option<&p3_mvp::Session>, max_frames: u64) -> Result<(u32, u64), String> {
    let (w, h) = (rt.surface.width as u32, rt.surface.height as u32);
    if let Some(s) = session {
        rt.seed_session(mem, s)?;
    }
    if opts.scene {
        rt.create_scene(mem);
    } else {
        rt.load_button_styles(mem);
    }
    let host = rt.new_object(mem, 0x400);
    rt.call(mem, 0x004B_4920, host, &[]);
    rt.call(mem, 0x0040_24B0, host, &[w, h]);
    rt.call(mem, 0x004B_90E0, SCREEN_LIST, &[host, 0]);
    for (size, ctor, methods) in &opts.windows {
        let obj = rt.new_object(mem, *size);
        rt.call(mem, *ctor, obj, &[]);
        for &f in methods {
            rt.call(mem, f, obj, &[]);
        }
        rt.call(mem, 0x004B_4E30, host, &[obj]);
    }
    let mut frames = 0u64;
    loop {
        let r = rt.call(mem, RUN_FRAME, SCREEN_LIST, &[]);
        frames += 1;
        if rt.aborted {
            return Err(format!("el enlazador abortó en el fotograma {frames} (llamadas {}, faltan {:x?})", rt.calls, rt.missing_from));
        }
        if r != u32::MAX || rt.on_present.is_none() || (max_frames != 0 && frames >= max_frames) {
            return Ok((r, frames));
        }
        // `WM_QUIT` (ventana cerrada): `run_frame` lo convierte en la bandera `0x6DA846`.
        if mem.u8_at(QUIT_FLAG) != 0 {
            return Ok((r, frames));
        }
    }
}
