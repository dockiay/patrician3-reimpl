//! Ventana nativa del juego (winit + softbuffer) llevada «desde dentro»: quien tiene el bucle (el código original emulado en
//! `screen_list_run_frame`, o una vista de prueba) llama a [`GameWindow::present`] una vez por fotograma; ese paso muestra la
//! imagen, recoge los eventos del sistema con `pump_app_events` (sin invertir el bucle) y los deja como mensajes Win32 en la cola
//! que lee `PeekMessageA` (`p3_mvp::ui_rt::win32`). Luego espera al siguiente fotograma (60 por segundo por defecto).
//!
//! Pantalla completa: ventana sin bordes del tamaño del monitor con la imagen del juego escalada sin deformar y bandas negras
//! (Alt+Intro alterna). `exclusive` pide además el modo de vídeo del juego, como hacía DirectDraw (`set_ddraw_resolution` 0x4BCCC0).
//!
//! Ratón confinado: a pantalla completa y con el foco, el cursor no sale de la imagen del juego (`set_cursor_grab` y, si hay
//! bandas negras, se devuelve al borde de la imagen). Con dos monitores el original lo conseguía con DirectDraw exclusivo; sin
//! eso el cursor pasaba a la otra pantalla y no se podía desplazar la cámara contra el borde. Al perder el foco (Alt+Tab) o al
//! pasar a ventana se suelta.
use crate::keys;
use p3_mvp::ui_rt::win32::{self, InputState, Msg};
use std::collections::VecDeque;
use std::num::NonZeroU32;
use std::rc::Rc;
use std::time::{Duration, Instant};
use winit::application::ApplicationHandler;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::platform::pump_events::{EventLoopExtPumpEvents, PumpStatus};
use winit::window::{CursorGrabMode, Fullscreen, Window, WindowId};

/// Opciones de la ventana.
#[derive(Debug, Clone)]
pub struct WindowConfig {
    pub title: String,
    /// Resolución del juego (la de la superficie que se presenta).
    pub width: u32,
    pub height: u32,
    pub fullscreen: bool,
    /// Pantalla completa con cambio de modo de vídeo (si el monitor ofrece la resolución del juego).
    pub exclusive: bool,
    /// Fotogramas por segundo; 0 = sin esperar.
    pub fps: u32,
    /// Mostrar el cursor del sistema (el juego original lo oculta y dibuja el suyo en `screen_list_run_frame`).
    pub system_cursor: bool,
    /// Confinar el ratón a la imagen del juego a pantalla completa mientras la ventana tenga el foco.
    pub confine_cursor: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        WindowConfig { title: "Patrician III".into(), width: 1024, height: 768, fullscreen: true, exclusive: false, fps: 60, system_cursor: false, confine_cursor: true }
    }
}

struct Gfx {
    window: Rc<Window>,
    surface: softbuffer::Surface<Rc<Window>, Rc<Window>>,
    _context: softbuffer::Context<Rc<Window>>,
}

/// Estado que ve `ApplicationHandler` mientras se vacían los eventos.
struct App {
    cfg: WindowConfig,
    gfx: Option<Gfx>,
    error: Option<String>,
    /// Eventos traducidos desde la última presentación.
    out: VecDeque<Msg>,
    input: InputState,
    close: bool,
    /// Rectángulo de la imagen del juego dentro de la ventana (x, y, ancho, alto en píxeles físicos).
    view: (i32, i32, i32, i32),
    fullscreen: bool,
    resized: bool,
    /// Tamaño de la última imagen del juego mostrada: el ratón se traduce a esas coordenadas (cambia con el modo de vídeo).
    game: (u32, u32),
    /// La ventana tiene el foco del teclado.
    focused: bool,
    /// El cursor está confinado ahora mismo.
    grabbed: bool,
    /// El cursor del sistema está oculto con `set_cursor_visible(false)`: entonces no se confina (winit lo fijaría al centro).
    cursor_hidden: bool,
}

/// Se confina el ratón solo a pantalla completa y con el foco: en ventana el jugador necesita salir de ella.
pub(crate) fn wants_grab(confine: bool, fullscreen: bool, focused: bool) -> bool {
    confine && fullscreen && focused
}

/// Punto de la ventana dentro del rectángulo de la imagen `(x, y, ancho, alto)`; `None` si ya lo está (o no hay imagen).
pub(crate) fn clamp_to_view(view: (i32, i32, i32, i32), x: f64, y: f64) -> Option<(f64, f64)> {
    let (vx, vy, vw, vh) = view;
    if vw <= 0 || vh <= 0 {
        return None;
    }
    let cx = x.clamp(f64::from(vx), f64::from(vx + vw - 1));
    let cy = y.clamp(f64::from(vy), f64::from(vy + vh - 1));
    (cx != x || cy != y).then_some((cx, cy))
}

impl App {
    fn attributes(&self) -> winit::window::WindowAttributes {
        let mut a = Window::default_attributes()
            .with_title(self.cfg.title.clone())
            .with_inner_size(PhysicalSize::new(self.cfg.width, self.cfg.height))
            .with_resizable(true);
        if self.fullscreen {
            a = a.with_fullscreen(Some(Fullscreen::Borderless(None)));
        }
        a
    }

    /// Aplica o suelta el confinamiento del ratón según pantalla completa y foco. `force` lo vuelve a pedir aunque no cambie
    /// (tras un cambio de tamaño el rectángulo de recorte del sistema queda viejo).
    fn update_grab(&mut self, force: bool) {
        let want = wants_grab(self.cfg.confine_cursor && !self.cursor_hidden, self.fullscreen, self.focused);
        if want == self.grabbed && !(want && force) {
            return;
        }
        let Some(g) = &self.gfx else { return };
        if want {
            match g.window.set_cursor_grab(CursorGrabMode::Confined) {
                Ok(()) => self.grabbed = true,
                Err(e) => {
                    // Sin confinamiento del sistema (p. ej. X11 sin gestor de ventanas) basta devolver el cursor a la imagen.
                    if !self.grabbed {
                        eprintln!("ratón: no se pudo confinar ({e}); se devuelve a la imagen al salir");
                    }
                    self.grabbed = true;
                }
            }
        } else {
            let _ = g.window.set_cursor_grab(CursorGrabMode::None);
            self.grabbed = false;
        }
    }

    fn set_fullscreen(&mut self, on: bool) {
        self.fullscreen = on;
        self.update_grab(false);
        let Some(g) = &self.gfx else { return };
        if !on {
            g.window.set_fullscreen(None);
            let _ = g.window.request_inner_size(PhysicalSize::new(self.cfg.width, self.cfg.height));
            return;
        }
        let exclusive = if self.cfg.exclusive {
            g.window.current_monitor().and_then(|m| {
                m.video_modes()
                    .filter(|v| v.size() == PhysicalSize::new(self.cfg.width, self.cfg.height))
                    .max_by_key(|v| (v.bit_depth(), v.refresh_rate_millihertz()))
            })
        } else {
            None
        };
        match exclusive {
            Some(mode) => g.window.set_fullscreen(Some(Fullscreen::Exclusive(mode))),
            None => g.window.set_fullscreen(Some(Fullscreen::Borderless(None))),
        }
    }

    /// Píxel de la ventana → coordenadas del juego (recortadas a la pantalla del juego).
    fn to_game(&self, x: f64, y: f64) -> (i32, i32) {
        let (vx, vy, vw, vh) = self.view;
        if vw <= 0 || vh <= 0 {
            return (0, 0);
        }
        let (gw, gh) = self.game;
        let gx = ((x - f64::from(vx)) * f64::from(gw) / f64::from(vw)).floor() as i32;
        let gy = ((y - f64::from(vy)) * f64::from(gh) / f64::from(vh)).floor() as i32;
        (gx.clamp(0, gw as i32 - 1), gy.clamp(0, gh as i32 - 1))
    }

    fn post(&mut self, message: u32, wparam: u32, lparam: u32) {
        win32::post(&mut self.out, &self.input, message, wparam, lparam);
    }

    fn mouse_message(&mut self, message: u32) {
        let (x, y) = self.input.cursor;
        let keys = self.input.mouse_keys();
        self.post(message, keys, win32::mouse_lparam(x, y));
    }

    fn release_all(&mut self) {
        for vk in 0..=255u8 {
            if self.input.down[usize::from(vk)] {
                self.input.set_key(vk, false);
            }
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, el: &ActiveEventLoop) {
        if self.gfx.is_some() {
            return;
        }
        let made = (|| -> Result<Gfx, String> {
            let window = Rc::new(el.create_window(self.attributes()).map_err(|e| format!("ventana: {e}"))?);
            let context = softbuffer::Context::new(window.clone()).map_err(|e| format!("softbuffer: {e}"))?;
            let surface = softbuffer::Surface::new(&context, window.clone()).map_err(|e| format!("softbuffer: {e}"))?;
            // Cursor del juego: el del sistema se sustituye por uno transparente en vez de ocultarlo. winit en Windows confina
            // un cursor oculto al píxel central de la ventana (pensado para ratón relativo), así que con `set_cursor_visible(false)`
            // y `set_cursor_grab(Confined)` el ratón quedaba inmóvil a pantalla completa (13/11/2026). Con un cursor visible pero
            // transparente se confina al área cliente. Si no se puede crear, se oculta y no se confina.
            self.cursor_hidden = false;
            if !self.cfg.system_cursor {
                match winit::window::CustomCursor::from_rgba(vec![0u8; 16 * 16 * 4], 16, 16, 0, 0) {
                    Ok(source) => window.set_cursor(el.create_custom_cursor(source)),
                    Err(e) => {
                        eprintln!("ratón: sin cursor transparente ({e}); se oculta y no se confina");
                        window.set_cursor_visible(false);
                        self.cursor_hidden = true;
                    }
                }
            }
            Ok(Gfx { window, surface, _context: context })
        })();
        match made {
            Ok(g) => {
                self.gfx = Some(g);
                self.resized = true;
                if self.fullscreen && self.cfg.exclusive {
                    self.set_fullscreen(true);
                }
                self.post(win32::WM_ACTIVATEAPP, 1, 0);
            }
            Err(e) => {
                self.error = Some(e);
                self.close = true;
            }
        }
    }

    fn window_event(&mut self, _el: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => self.close = true,
            WindowEvent::Resized(_) | WindowEvent::ScaleFactorChanged { .. } => {
                self.resized = true;
                self.update_grab(true);
            }
            WindowEvent::RedrawRequested => self.post(win32::WM_PAINT, 0, 0),
            WindowEvent::Focused(on) => {
                if !on {
                    self.release_all();
                }
                self.focused = on;
                self.update_grab(false);
                self.post(win32::WM_ACTIVATEAPP, u32::from(on), 0);
            }
            WindowEvent::CursorMoved { mut position, .. } => {
                // Confinado y con bandas negras: el cursor vuelve al borde de la imagen (así la cámara se desplaza contra él y
                // no queda una franja muerta que recorrer de vuelta).
                if self.grabbed {
                    if let (Some((x, y)), Some(g)) = (clamp_to_view(self.view, position.x, position.y), &self.gfx) {
                        let _ = g.window.set_cursor_position(PhysicalPosition::new(x, y));
                        position = PhysicalPosition::new(x, y);
                    }
                }
                let p = self.to_game(position.x, position.y);
                if p != self.input.cursor {
                    self.input.cursor = p;
                    self.mouse_message(win32::WM_MOUSEMOVE);
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let down = state == ElementState::Pressed;
                let (vk, msg_down, msg_up) = match button {
                    MouseButton::Left => (win32::VK_LBUTTON, win32::WM_LBUTTONDOWN, win32::WM_LBUTTONUP),
                    MouseButton::Right => (win32::VK_RBUTTON, win32::WM_RBUTTONDOWN, win32::WM_RBUTTONUP),
                    MouseButton::Middle => (win32::VK_MBUTTON, win32::WM_MBUTTONDOWN, win32::WM_MBUTTONUP),
                    _ => return,
                };
                self.input.set_key(vk, down);
                self.mouse_message(if down { msg_down } else { msg_up });
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let steps = match delta {
                    MouseScrollDelta::LineDelta(_, y) => (y * 120.0) as i32,
                    MouseScrollDelta::PixelDelta(p) => (p.y * 2.0) as i32,
                };
                if steps != 0 {
                    let keys = self.input.mouse_keys();
                    let (x, y) = self.input.cursor;
                    self.post(win32::WM_MOUSEWHEEL, keys | ((steps as i16 as u16 as u32) << 16), win32::mouse_lparam(x, y));
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                let PhysicalKey::Code(code) = event.physical_key else { return };
                let down = event.state == ElementState::Pressed;
                let alt = self.input.down[usize::from(win32::VK_MENU)];
                // Alt+Intro alterna pantalla completa y ventana; no llega al juego.
                if down && alt && code == KeyCode::Enter {
                    let on = !self.fullscreen;
                    self.set_fullscreen(on);
                    return;
                }
                let Some(vk) = keys::vk(code) else { return };
                let was = self.input.set_key(vk, down);
                let generic = match vk {
                    win32::VK_LSHIFT | win32::VK_RSHIFT => win32::VK_SHIFT,
                    win32::VK_LCONTROL | win32::VK_RCONTROL => win32::VK_CONTROL,
                    win32::VK_LMENU | win32::VK_RMENU => win32::VK_MENU,
                    v => v,
                };
                let alt_now = self.input.down[usize::from(win32::VK_MENU)];
                // Como Windows: con Alt (o la propia Alt) los mensajes son WM_SYSKEY*.
                let sys = alt_now || generic == win32::VK_MENU || (alt && !down);
                let message = match (down, sys) {
                    (true, false) => win32::WM_KEYDOWN,
                    (true, true) => win32::WM_SYSKEYDOWN,
                    (false, false) => win32::WM_KEYUP,
                    (false, true) => win32::WM_SYSKEYUP,
                };
                let lparam = win32::key_lparam(win32::scan_code(vk), win32::is_extended(vk), was, !down);
                self.post(message, u32::from(generic), lparam);
                // Sin `WM_CHAR`: el juego no llama a `TranslateMessage`; el texto lo saca de `ToAscii` con el estado del teclado.
            }
            _ => {}
        }
    }
}

/// Ventana del juego. Ver el módulo.
pub struct GameWindow {
    event_loop: EventLoop<()>,
    app: App,
    start: Instant,
    next_frame: Instant,
    /// Fotogramas presentados.
    pub frames: u64,
    /// Fotogramas por segundo medidos en el último segundo.
    pub measured_fps: f64,
    fps_mark: (Instant, u64),
    scaler: crate::scale::Scaler,
    stats: FrameStats,
}

/// Medición de fotogramas para el registro: cada 30 s (o en cuanto un fotograma pasa de 150 ms) una línea con el ritmo, el peor
/// fotograma y cuánto se va en el juego, en mostrar la imagen y en recoger la entrada. Sirve para saber, sin capturas, si un
/// «congelado» es del juego o de la ventana.
struct FrameStats {
    since: Instant,
    last: Instant,
    frames: u32,
    worst_ms: f64,
    game_ms: f64,
    show_ms: f64,
    poll_ms: f64,
    messages: usize,
    slow: u32,
}

impl FrameStats {
    fn new(now: Instant) -> Self {
        FrameStats { since: now, last: now, frames: 0, worst_ms: 0.0, game_ms: 0.0, show_ms: 0.0, poll_ms: 0.0, messages: 0, slow: 0 }
    }
}

impl GameWindow {
    pub fn open(cfg: WindowConfig) -> Result<Self, String> {
        let event_loop = EventLoop::new().map_err(|e| format!("bucle de eventos: {e}"))?;
        let fullscreen = cfg.fullscreen;
        let game = (cfg.width.max(1), cfg.height.max(1));
        let app = App {
            cfg,
            gfx: None,
            error: None,
            out: VecDeque::new(),
            input: InputState::default(),
            close: false,
            view: (0, 0, 0, 0),
            fullscreen,
            resized: true,
            game,
            focused: false,
            grabbed: false,
            cursor_hidden: false,
        };
        let now = Instant::now();
        let mut w = GameWindow { event_loop, app, start: now, next_frame: now, frames: 0, measured_fps: 0.0, fps_mark: (now, 0), scaler: Default::default(), stats: FrameStats::new(now) };
        // Primera pasada: crea la ventana (`resumed`).
        w.pump();
        if let Some(e) = w.app.error.take() {
            return Err(e);
        }
        if w.app.gfx.is_none() {
            return Err("no se pudo crear la ventana".into());
        }
        Ok(w)
    }

    /// Milisegundos desde que se abrió la ventana (el `timeGetTime` del juego).
    pub fn now_ms(&self) -> u32 {
        self.start.elapsed().as_millis() as u32
    }

    /// Tamaño en píxeles del monitor donde está la ventana (para elegir la resolución moderna).
    pub fn monitor_size(&self) -> Option<(u32, u32)> {
        let g = self.app.gfx.as_ref()?;
        let m = g.window.current_monitor().or_else(|| g.window.primary_monitor()).or_else(|| g.window.available_monitors().next())?;
        let s = m.size();
        (s.width > 0 && s.height > 0).then_some((s.width, s.height))
    }

    /// Tamaño de la ventana cuando no está a pantalla completa (y al que vuelve Alt+Intro).
    pub fn set_windowed_size(&mut self, width: u32, height: u32) {
        self.app.cfg.width = width;
        self.app.cfg.height = height;
        if !self.app.fullscreen {
            if let Some(g) = &self.app.gfx {
                let _ = g.window.request_inner_size(PhysicalSize::new(width, height));
            }
            self.pump();
        }
    }

    pub fn is_fullscreen(&self) -> bool {
        self.app.fullscreen
    }

    pub fn set_fullscreen(&mut self, on: bool) {
        self.app.set_fullscreen(on);
    }

    fn pump(&mut self) -> bool {
        let status = self.event_loop.pump_app_events(Some(Duration::ZERO), &mut self.app);
        !matches!(status, PumpStatus::Exit(_)) && !self.app.close
    }

    /// Recoge eventos del sistema: deja en `queue` los mensajes Win32 nuevos y actualiza `input` (cursor, teclas, reloj).
    /// `false` si se cerró la ventana.
    pub fn poll(&mut self, queue: &mut VecDeque<Msg>, input: &mut InputState) -> bool {
        // La entrada propia del juego (p. ej. `SetCursorPos` de `clamp_cursor_to_screen`) manda sobre la de la ventana.
        self.app.input.cursor = input.cursor;
        let alive = self.pump();
        queue.extend(self.app.out.drain(..));
        let t = self.now_ms();
        *input = self.app.input.clone();
        input.time_ms = Some(t);
        self.app.input.time_ms = Some(t);
        alive
    }

    /// Muestra una imagen RGBA del tamaño del juego (escalada al tamaño de la ventana sin deformar).
    pub fn show_rgba(&mut self, width: usize, height: usize, rgba: &[u8]) -> Result<(), String> {
        self.app.game = (width.max(1) as u32, height.max(1) as u32);
        let Some(g) = self.app.gfx.as_mut() else { return Ok(()) };
        let size = g.window.inner_size();
        let (ww, wh) = (size.width.max(1), size.height.max(1));
        if self.app.resized {
            g.surface.resize(NonZeroU32::new(ww).unwrap(), NonZeroU32::new(wh).unwrap()).map_err(|e| format!("softbuffer: {e}"))?;
            self.app.resized = false;
        }
        // Rectángulo que conserva la proporción del juego.
        let (gw, gh) = (width.max(1) as u64, height.max(1) as u64);
        let (vw, vh) = if u64::from(ww) * gh <= u64::from(wh) * gw {
            (u64::from(ww), u64::from(ww) * gh / gw)
        } else {
            (u64::from(wh) * gw / gh, u64::from(wh))
        };
        let (vx, vy) = ((u64::from(ww) - vw) / 2, (u64::from(wh) - vh) / 2);
        self.app.view = (vx as i32, vy as i32, vw as i32, vh as i32);
        let mut buf = g.surface.buffer_mut().map_err(|e| format!("softbuffer: {e}"))?;
        if buf.len() < (ww * wh) as usize {
            return Ok(());
        }
        // Escalado «bilineal nítido»: el vecino más cercano con factor no entero duplica filas y columnas de forma
        // desigual (letras con grosor irregular); el bilineal puro emborrona. Aquí cada píxel de origen ocupa una
        // zona plana y solo se mezcla en la costura (ancho ≈ 1 píxel de pantalla), así que con factor entero es
        // idéntico al vecino más cercano y con factor fraccionario el texto queda nítido y regular.
        self.scaler.draw(rgba, width, height, &mut buf, ww as usize, (vx as usize, vy as usize, vw as usize, vh as usize));
        buf.present().map_err(|e| format!("softbuffer: {e}"))?;
        Ok(())
    }

    /// Espera hasta el siguiente fotograma (ritmo fijo; si se va con retraso, no acumula).
    pub fn wait_frame(&mut self) {
        self.frames += 1;
        let fps = self.app.cfg.fps;
        let now = Instant::now();
        if now.duration_since(self.fps_mark.0) >= Duration::from_secs(1) {
            self.measured_fps = (self.frames - self.fps_mark.1) as f64 / now.duration_since(self.fps_mark.0).as_secs_f64();
            self.fps_mark = (now, self.frames);
        }
        if fps == 0 {
            return;
        }
        let period = Duration::from_nanos(1_000_000_000 / u64::from(fps));
        self.next_frame += period;
        if self.next_frame < now {
            self.next_frame = now;
            return;
        }
        let left = self.next_frame - now;
        if left > Duration::from_millis(2) {
            std::thread::sleep(left - Duration::from_millis(1));
        }
        while Instant::now() < self.next_frame {
            std::hint::spin_loop();
        }
    }

    /// Un fotograma completo: mostrar, recoger entrada y esperar. Es lo que hace el gancho de `sgl_SwapBuffer`.
    pub fn present(&mut self, width: usize, height: usize, rgba: &[u8], queue: &mut VecDeque<Msg>, input: &mut InputState) -> bool {
        let t0 = Instant::now();
        let game = t0.duration_since(self.stats.last).as_secs_f64() * 1000.0;
        let tramo = p3_mvp::ui_rt::stall::tag("mostrar");
        if let Err(e) = self.show_rgba(width, height, rgba) {
            eprintln!("{e}");
        }
        drop(tramo);
        let t1 = Instant::now();
        let tramo = p3_mvp::ui_rt::stall::tag("esperar");
        self.wait_frame();
        drop(tramo);
        let t2 = Instant::now();
        let before = queue.len();
        let tramo = p3_mvp::ui_rt::stall::tag("entrada");
        let alive = self.poll(queue, input);
        drop(tramo);
        let t3 = Instant::now();
        let ms = |a: Instant, b: Instant| b.duration_since(a).as_secs_f64() * 1000.0;
        let st = &mut self.stats;
        let total = game + ms(t0, t1) + ms(t2, t3);
        st.frames += 1;
        st.game_ms += game;
        st.show_ms += ms(t0, t1);
        st.poll_ms += ms(t2, t3);
        st.messages += queue.len().saturating_sub(before);
        st.worst_ms = st.worst_ms.max(total);
        if total > 150.0 && st.slow < 20 {
            st.slow += 1;
            eprintln!("fotograma lento: {total:.0} ms (juego {game:.0}, mostrar {:.0}, entrada {:.0}), {width}x{height}", ms(t0, t1), ms(t2, t3));
        }
        if t3.duration_since(st.since) >= Duration::from_secs(30) {
            let n = f64::from(st.frames.max(1));
            eprintln!(
                "ritmo: {:.1} fps, peor {:.0} ms; por fotograma: juego {:.1} ms, mostrar {:.1} ms, entrada {:.1} ms, {:.1} mensajes; {width}x{height}",
                n / t3.duration_since(st.since).as_secs_f64(), st.worst_ms, st.game_ms / n, st.show_ms / n, st.poll_ms / n, st.messages as f64 / n
            );
            *st = FrameStats { slow: st.slow, ..FrameStats::new(t3) };
        }
        self.stats.last = t3;
        alive
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cursor_is_confined_only_in_fullscreen_with_focus() {
        assert!(wants_grab(true, true, true));
        assert!(!wants_grab(true, true, false), "Alt+Tab o clic en la otra pantalla: se suelta");
        assert!(!wants_grab(true, false, true), "en ventana el ratón queda libre");
        assert!(!wants_grab(false, true, true), "--raton-libre");
    }

    #[test]
    fn cursor_returns_to_the_game_image() {
        // 2560x1440 centrada en un monitor de 3440x1440: bandas de 440 px.
        let view = (440, 0, 2560, 1440);
        assert_eq!(clamp_to_view(view, 1000.0, 700.0), None);
        assert_eq!(clamp_to_view(view, 440.0, 0.0), None);
        assert_eq!(clamp_to_view(view, 2999.0, 1439.0), None);
        assert_eq!(clamp_to_view(view, 100.0, 700.0), Some((440.0, 700.0)));
        assert_eq!(clamp_to_view(view, 3439.0, 1500.0), Some((2999.0, 1439.0)));
        assert_eq!(clamp_to_view((0, 0, 0, 0), 5.0, 5.0), None);
    }
}
