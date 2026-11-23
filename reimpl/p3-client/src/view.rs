//! Vista de prueba sobre una partida (`.pat`): la ciudad que dibuja `Session::render_fx_rgba` y la carta náutica de
//! `Session::navmap_frame_rgba`, desplazables con el ratón y el teclado, dibujadas en el búfer trasero del dispositivo SGL y
//! presentadas con `SwapBuffer` en la ventana nativa. Es el primer resultado del cliente nativo mientras la escena
//! (`0x6E51AC`) y el gestor de ventanas del original se conectan al enlazador.
//!
//! Controles: ratón en el borde o flechas para desplazar, arrastrar con el botón izquierdo o derecho, rueda vertical; Tab
//! alterna ciudad y carta; Alt+Intro pantalla completa; Esc sale.
use crate::device::{self, NativeHost, NativeScreen};
use crate::window::GameWindow;
use p3_mvp::navmap::NavRequest;
use p3_mvp::ui_rt::win32::{self, InputState, Msg};
use p3_mvp::Session;
use p3_render::sgl_screen::SglScreen;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    City,
    Chart,
}

/// Imagen entera de la ciudad y posición de la vista.
struct City {
    w: usize,
    h: usize,
    rgba: Vec<u8>,
    scroll: (i32, i32),
}

pub struct View {
    pub session: Session,
    pub mode: Mode,
    city: Option<City>,
    /// Último arrastre: posición del ratón al pulsar.
    drag: Option<(i32, i32)>,
    pub log: Vec<String>,
    /// Teclas de la carta pulsadas desde el fotograma anterior (izquierda, arriba, derecha, abajo).
    chart_keys: [bool; 4],
    chart_center: Option<(i32, i32)>,
    /// El ratón ya se movió dentro de la ventana (antes, el cursor (0, 0) no desplaza por el borde).
    mouse_seen: bool,
    /// Resolución de la vista: 1024×768 como el cliente original, o la moderna (se ve más ciudad y más carta, sin ampliar).
    pub size: (u32, u32),
    /// Dónde quedó la carta dentro de la pantalla en el último fotograma (centrada si es menor que la pantalla).
    chart_offset: (i32, i32),
}

/// Píxeles de desplazamiento por fotograma con el ratón en el borde o las flechas.
const SCROLL_STEP: i32 = 16;
/// Ancho del borde que desplaza.
const EDGE: i32 = 6;

impl View {
    pub fn new(session: Session, mode: Mode) -> Self {
        View { session, mode, city: None, drag: None, log: Vec::new(), chart_keys: [false; 4], chart_center: None, mouse_seen: false, size: (1024, 768), chart_offset: (0, 0) }
    }

    fn city(&mut self, sw: usize, sh: usize) -> Result<&mut City, String> {
        if self.city.is_none() {
            let t = std::time::Instant::now();
            let (v, (w, h, rgba)) = self.session.render_fx_rgba(0, &Default::default())?;
            let focus = (v["focus"][0].as_i64().unwrap_or(0) as i32, v["focus"][1].as_i64().unwrap_or(0) as i32);
            let scroll = (focus.0 - sw as i32 / 2, focus.1 - sh as i32 / 2);
            self.log.push(format!("ciudad {w}x{h} en {:.2} s, foco {focus:?}", t.elapsed().as_secs_f64()));
            self.city = Some(City { w, h, rgba, scroll });
        }
        Ok(self.city.as_mut().unwrap())
    }

    /// Mensajes del fotograma: arrastre, rueda y teclas de la vista. Devuelve `false` con Esc.
    fn handle(&mut self, queue: &mut VecDeque<Msg>, input: &InputState) -> bool {
        let mut pan = (0, 0);
        for m in queue.drain(..) {
            match m.message {
                win32::WM_KEYDOWN => match m.wparam as u8 {
                    win32::VK_ESCAPE => return false,
                    win32::VK_TAB => {
                        self.mode = if self.mode == Mode::City { Mode::Chart } else { Mode::City };
                        self.drag = None;
                    }
                    win32::VK_LEFT => self.chart_keys[0] = true,
                    win32::VK_UP => self.chart_keys[1] = true,
                    win32::VK_RIGHT => self.chart_keys[2] = true,
                    win32::VK_DOWN => self.chart_keys[3] = true,
                    _ => {}
                },
                win32::WM_LBUTTONDOWN | win32::WM_RBUTTONDOWN => self.drag = Some(input_pt(m.lparam)),
                win32::WM_LBUTTONUP | win32::WM_RBUTTONUP => {
                    if input.mouse_keys() & (win32::MK_LBUTTON | win32::MK_RBUTTON) == 0 {
                        self.drag = None;
                    }
                }
                win32::WM_LBUTTONDBLCLK => self.chart_center = Some(input_pt(m.lparam)),
                win32::WM_MOUSEMOVE => {
                    self.mouse_seen = true;
                    if let Some(d) = self.drag {
                        let p = input_pt(m.lparam);
                        pan.0 += d.0 - p.0;
                        pan.1 += d.1 - p.1;
                        self.drag = Some(p);
                    }
                }
                win32::WM_MOUSEWHEEL => {
                    let delta = (m.wparam >> 16) as u16 as i16 as i32;
                    pan.1 -= delta / 120 * 48;
                }
                _ => {}
            }
        }
        if self.mode == Mode::City {
            if let Some(c) = self.city.as_mut() {
                let held = |vk: u8| input.down[usize::from(vk)];
                let (cx, cy) = input.cursor;
                let (sw, sh) = (self.size.0 as i32, self.size.1 as i32);
                let edge = self.mouse_seen && self.drag.is_none();
                if self.drag.is_none() {
                    if (edge && cx < EDGE) || held(win32::VK_LEFT) {
                        pan.0 -= SCROLL_STEP;
                    }
                    if (edge && cx >= sw - EDGE) || held(win32::VK_RIGHT) {
                        pan.0 += SCROLL_STEP;
                    }
                    if (edge && cy < EDGE) || held(win32::VK_UP) {
                        pan.1 -= SCROLL_STEP;
                    }
                    if (edge && cy >= sh - EDGE) || held(win32::VK_DOWN) {
                        pan.1 += SCROLL_STEP;
                    }
                }
                c.scroll.0 += pan.0;
                c.scroll.1 += pan.1;
            }
        }
        true
    }

    /// Dibuja el fotograma en `back` (RGBA `sw × sh`).
    fn draw(&mut self, back: &mut [u8], sw: usize, sh: usize, input: &InputState) -> Result<(), String> {
        match self.mode {
            Mode::City => {
                let c = self.city(sw, sh)?;
                c.scroll.0 = c.scroll.0.clamp(0, (c.w as i32 - sw as i32).max(0));
                c.scroll.1 = c.scroll.1.clamp(0, (c.h as i32 - sh as i32).max(0));
                back.fill(0);
                let (x0, y0) = (c.scroll.0 as usize, c.scroll.1 as usize);
                let cols = sw.min(c.w.saturating_sub(x0));
                for y in 0..sh.min(c.h.saturating_sub(y0)) {
                    let src = &c.rgba[((y0 + y) * c.w + x0) * 4..((y0 + y) * c.w + x0 + cols) * 4];
                    back[y * sw * 4..(y * sw + cols) * 4].copy_from_slice(src);
                }
            }
            Mode::Chart => {
                let req = NavRequest {
                    width: sw as i32,
                    height: sh as i32,
                    clock_ms: input.time_ms.unwrap_or(0),
                    mouse: self.mouse_seen.then_some((input.cursor.0 - self.chart_offset.0, input.cursor.1 - self.chart_offset.1)),
                    keys: std::mem::take(&mut self.chart_keys),
                    center: self.chart_center.take().map(|p| (p.0 - self.chart_offset.0, p.1 - self.chart_offset.1)),
                    ticks: 0,
                    selection: Vec::new(),
                };
                let (v, (w, h, rgba)) = self.session.navmap_frame_rgba(&req, None)?;
                if let Some(e) = v["errors"].as_array().filter(|e| !e.is_empty()) {
                    let line = format!("carta: {e:?}");
                    if self.log.last() != Some(&line) {
                        self.log.push(line);
                    }
                }
                back.fill(0);
                // Con una resolución mayor que la carta entera (unos 2400×1430 con su marco) sobra pantalla: la carta va centrada.
                let (w, h) = (w.min(sw), h.min(sh));
                let black = |x: usize, y: usize| rgba[(y * w.max(1) + x) * 4..][..3] == [0, 0, 0];
                let (mut cw, mut ch) = (w, h);
                while cw > 1 && (0..h).step_by(8).all(|y| black(cw - 1, y)) {
                    cw -= 1;
                }
                while ch > 1 && (0..cw).step_by(8).all(|x| black(x, ch - 1)) {
                    ch -= 1;
                }
                let (ox, oy) = ((sw - cw) / 2, (sh - ch) / 2);
                self.chart_offset = (ox as i32, oy as i32);
                for y in 0..ch {
                    back[((y + oy) * sw + ox) * 4..][..cw * 4].copy_from_slice(&rgba[y * w * 4..][..cw * 4]);
                }
            }
        }
        Ok(())
    }

    /// Bucle de la vista: cada fotograma dibuja en el trasero, `SwapBuffer` y presenta en la ventana. `frames` = 0 sin límite.
    /// Con `capture` guarda el último fotograma como PNG al acabar.
    pub fn run(&mut self, window: &mut GameWindow, frames: u64, script: &mut Script, capture: Option<&std::path::Path>) -> Result<(), String> {
        let mut screen: SglScreen<NativeScreen> = device::init_screen(self.size.0 as i32, self.size.1 as i32, !window.is_fullscreen())?;
        let mut host = NativeHost::default();
        let mut queue = VecDeque::new();
        let mut input = InputState::default();
        let mut slow = (0u64, 0f64);
        loop {
            if !window.poll(&mut queue, &mut input) {
                break;
            }
            script.apply(window.frames, &mut queue, &mut input);
            if !self.handle(&mut queue, &input) {
                break;
            }
            let t = std::time::Instant::now();
            let dev = device::active(&mut screen).ok_or("sin pantalla activa")?;
            let (sw, sh) = dev.size();
            let mut back = std::mem::take(&mut dev.back);
            let r = self.draw(&mut back, sw, sh, &input);
            device::active(&mut screen).unwrap().back = back;
            r?;
            screen.swap_buffer(&mut host, 1).map_err(|_| "SwapBuffer: fallo de la DLL")?;
            let dev = device::active(&mut screen).unwrap();
            dev.fresh = false;
            let front = std::mem::take(&mut dev.front);
            window.show_rgba(sw, sh, &front)?;
            device::active(&mut screen).unwrap().front = front;
            let dt = t.elapsed().as_secs_f64();
            if dt > 1.0 / 60.0 {
                slow = (slow.0 + 1, slow.1.max(dt));
            }
            window.wait_frame();
            if window.frames % 120 == 0 {
                self.log.push(format!("fotograma {}: {:.1} fps, modo {:?}", window.frames, window.measured_fps, self.mode));
            }
            if frames != 0 && window.frames >= frames {
                break;
            }
        }
        if slow.0 > 0 {
            self.log.push(format!("{} fotogramas de más de 16,7 ms (máximo {:.1} ms)", slow.0, slow.1 * 1000.0));
        }
        if let Some(path) = capture {
            let dev = device::active(&mut screen).unwrap();
            let (w, h) = dev.size();
            p3_render::write_png(path, w, h, &dev.front).map_err(|e| format!("captura: {e}"))?;
            self.log.push(format!("captura en {}", path.display()));
        }
        Ok(())
    }
}

fn input_pt(lparam: u32) -> (i32, i32) {
    ((lparam & 0xFFFF) as u16 as i16 as i32, (lparam >> 16) as u16 as i16 as i32)
}

/// Entrada guionizada para pruebas sin persona (p. ej. `--guion "10:mover 1020 400;60:tab;90:tecla 27"`): en el fotograma
/// indicado se añaden los mensajes como si vinieran de la ventana.
#[derive(Debug, Default, Clone)]
pub struct Script(pub Vec<(u64, String)>);

impl Script {
    pub fn parse(s: &str) -> Result<Self, String> {
        let mut out = Vec::new();
        for part in s.split(';').map(str::trim).filter(|p| !p.is_empty()) {
            let (f, cmd) = part.split_once(':').ok_or_else(|| format!("guion: falta «fotograma:» en {part}"))?;
            out.push((f.trim().parse().map_err(|_| format!("guion: fotograma inválido en {part}"))?, cmd.trim().to_string()));
        }
        Ok(Script(out))
    }

    pub fn apply(&mut self, frame: u64, queue: &mut VecDeque<Msg>, input: &mut InputState) {
        for (_, cmd) in self.0.iter().filter(|(f, _)| *f == frame) {
            let w: Vec<&str> = cmd.split_whitespace().collect();
            let num = |i: usize| w.get(i).and_then(|v| v.parse::<i32>().ok()).unwrap_or(0);
            match w.first().copied() {
                Some("mover") => {
                    input.cursor = (num(1), num(2));
                    win32::post(queue, input, win32::WM_MOUSEMOVE, input.mouse_keys(), win32::mouse_lparam(num(1), num(2)));
                }
                Some(b @ ("pulsar" | "soltar")) => {
                    let down = b == "pulsar";
                    input.set_key(win32::VK_LBUTTON, down);
                    let msg = if down { win32::WM_LBUTTONDOWN } else { win32::WM_LBUTTONUP };
                    let (x, y) = input.cursor;
                    win32::post(queue, input, msg, input.mouse_keys(), win32::mouse_lparam(x, y));
                }
                Some("tab") => win32::post(queue, input, win32::WM_KEYDOWN, u32::from(win32::VK_TAB), 0),
                Some("rueda") => {
                    let (x, y) = input.cursor;
                    win32::post(queue, input, win32::WM_MOUSEWHEEL, input.mouse_keys() | ((num(1) as i16 as u16 as u32) << 16), win32::mouse_lparam(x, y));
                }
                Some("tecla") => {
                    let vk = num(1) as u8;
                    let was = input.set_key(vk, true);
                    win32::post(queue, input, win32::WM_KEYDOWN, u32::from(vk), win32::key_lparam(win32::scan_code(vk), false, was, false));
                    input.set_key(vk, false);
                }
                Some("mantener") => {
                    input.set_key(num(1) as u8, true);
                }
                Some("liberar") => {
                    input.set_key(num(1) as u8, false);
                }
                _ => eprintln!("guion: orden desconocida {cmd}"),
            }
        }
    }
}
