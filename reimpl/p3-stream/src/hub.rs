//! Estado compartido entre el juego, el codificador y los clientes WebSocket: cola de entrada, lista de espectadores y reparto
//! de paquetes. Una sola partida; todos los espectadores reciben lo mismo y cualquiera puede mandar entrada.
use p3_mvp::ui_rt::win32::{self, InputState, Msg};
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::mpsc::{sync_channel, Receiver, SyncSender, TrySendError};
use std::sync::{Arc, Mutex};

/// Paquete binario hacia el navegador: `[tipo][...]`. Tipo 1 = vídeo `[1][flags][w u16][h u16][Annex B]` (flags bit 0 = clave);
/// tipo 2 = sonido `[2][PCM i16 LE estéreo]` a la frecuencia del saludo.
pub type Packet = Arc<Vec<u8>>;

pub struct Client {
    pub id: u64,
    pub tx: SyncSender<Packet>,
    /// El cliente perdió paquetes de vídeo: hasta el próximo fotograma clave se le saltan los demás.
    pub need_key: Arc<AtomicBool>,
}

pub struct Hub {
    pub audio_rate: u32,
    pub clients: Mutex<Vec<Client>>,
    pub input: Mutex<VecDeque<Event>>,
    /// Alguien necesita un fotograma clave (espectador nuevo o desincronizado): el juego reenvía la imagen actual aunque no cambie.
    pub want_key: AtomicBool,
    pub viewers: AtomicUsize,
    pub next_id: AtomicU64,
    /// Estadísticas: bytes de vídeo y sonido enviados, fotogramas codificados.
    pub video_bytes: AtomicU64,
    pub audio_bytes: AtomicU64,
    pub video_frames: AtomicU64,
    /// Tiempo acumulado de codificación (µs) y fotogramas medidos.
    pub enc_us: AtomicU64,
    pub enc_n: AtomicU64,
    pub quit: AtomicBool,
    /// Tamaño actual de la imagen del juego (ancho << 16 | alto): la entrada del ratón se traduce a él.
    pub game_size: AtomicU64,
}

impl Hub {
    pub fn new(audio_rate: u32) -> Arc<Hub> {
        Arc::new(Hub {
            audio_rate,
            clients: Mutex::new(Vec::new()),
            input: Mutex::new(VecDeque::new()),
            want_key: AtomicBool::new(true),
            viewers: AtomicUsize::new(0),
            next_id: AtomicU64::new(1),
            video_bytes: AtomicU64::new(0),
            audio_bytes: AtomicU64::new(0),
            video_frames: AtomicU64::new(0),
            enc_us: AtomicU64::new(0),
            enc_n: AtomicU64::new(0),
            quit: AtomicBool::new(false),
            game_size: AtomicU64::new((800 << 16) | 600),
        })
    }

    pub fn add_client(&self) -> (u64, Receiver<Packet>, Arc<AtomicBool>) {
        let (tx, rx) = sync_channel(12);
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let need_key = Arc::new(AtomicBool::new(true));
        self.clients.lock().unwrap().push(Client { id, tx, need_key: need_key.clone() });
        self.viewers.fetch_add(1, Ordering::Relaxed);
        self.want_key.store(true, Ordering::Relaxed);
        (id, rx, need_key)
    }

    pub fn remove_client(&self, id: u64) {
        self.clients.lock().unwrap().retain(|c| c.id != id);
        self.viewers.fetch_sub(1, Ordering::Relaxed);
        // Sin espectadores se sueltan teclas y botones: si no, la tecla quedaría «pulsada» en el juego.
        if self.viewers.load(Ordering::Relaxed) == 0 {
            self.input.lock().unwrap().push_back(Event::Focus(false));
        }
    }

    /// Reparte un fotograma de vídeo. A quien va atrasado se le salta hasta el próximo fotograma clave.
    pub fn send_video(&self, key: bool, w: u32, h: u32, annexb: &[u8]) {
        let mut p = Vec::with_capacity(6 + annexb.len());
        p.extend_from_slice(&[1, u8::from(key)]);
        p.extend_from_slice(&(w as u16).to_le_bytes());
        p.extend_from_slice(&(h as u16).to_le_bytes());
        p.extend_from_slice(annexb);
        let p: Packet = Arc::new(p);
        self.video_bytes.fetch_add(p.len() as u64, Ordering::Relaxed);
        self.video_frames.fetch_add(1, Ordering::Relaxed);
        for c in self.clients.lock().unwrap().iter() {
            if key {
                c.need_key.store(false, Ordering::Relaxed);
            } else if c.need_key.load(Ordering::Relaxed) {
                continue;
            }
            if let Err(TrySendError::Full(_)) = c.tx.try_send(p.clone()) {
                c.need_key.store(true, Ordering::Relaxed);
                self.want_key.store(true, Ordering::Relaxed);
            }
        }
    }

    pub fn send_audio(&self, pcm: &[i16]) {
        if self.viewers.load(Ordering::Relaxed) == 0 {
            return;
        }
        let mut p = Vec::with_capacity(1 + pcm.len() * 2);
        p.push(2);
        for s in pcm {
            p.extend_from_slice(&s.to_le_bytes());
        }
        let p: Packet = Arc::new(p);
        self.audio_bytes.fetch_add(p.len() as u64, Ordering::Relaxed);
        for c in self.clients.lock().unwrap().iter() {
            let _ = c.tx.try_send(p.clone());
        }
    }
}

/// Evento de entrada del navegador. `Move` va normalizado a `0..=65535` sobre la imagen (el modo del juego puede cambiar).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Move { fx: u32, fy: u32 },
    /// 0 izquierdo, 1 central, 2 derecho.
    Button { button: u32, down: bool },
    Wheel(i32),
    Key { vk: u8, down: bool },
    Focus(bool),
}

impl Event {
    /// Mensajes de texto del navegador: `m fx fy`, `b botón 0|1`, `w pasos`, `k vk 0|1`, `f 0|1`.
    pub fn parse(s: &str) -> Option<Event> {
        let mut it = s.split_ascii_whitespace();
        let tag = it.next()?;
        let mut n = || it.next()?.parse::<i64>().ok();
        Some(match tag {
            "m" => Event::Move { fx: n()?.clamp(0, 65535) as u32, fy: n()?.clamp(0, 65535) as u32 },
            "b" => Event::Button { button: n()? as u32, down: n()? != 0 },
            "w" => Event::Wheel(n()?.clamp(-1200, 1200) as i32),
            "k" => Event::Key { vk: n()?.clamp(1, 255) as u8, down: n()? != 0 },
            "f" => Event::Focus(n()? != 0),
            _ => return None,
        })
    }
}

/// Aplica un evento: actualiza `input` y encola los mensajes Win32 (misma traducción que `p3-web/src/input.rs`).
pub fn apply(ev: Event, game: (usize, usize), input: &mut InputState, out: &mut VecDeque<Msg>) {
    let mouse = |input: &InputState, out: &mut VecDeque<Msg>, m: u32| {
        let (x, y) = input.cursor;
        win32::post(out, input, m, input.mouse_keys(), win32::mouse_lparam(x, y));
    };
    match ev {
        Event::Move { fx, fy } => {
            let (gw, gh) = (game.0 as u64, game.1 as u64);
            let x = ((u64::from(fx) * gw) / 65536).min(gw.saturating_sub(1)) as i32;
            let y = ((u64::from(fy) * gh) / 65536).min(gh.saturating_sub(1)) as i32;
            if (x, y) != input.cursor {
                input.cursor = (x, y);
                mouse(input, out, win32::WM_MOUSEMOVE);
            }
        }
        Event::Button { button, down } => {
            let (vk, d, u) = match button {
                0 => (win32::VK_LBUTTON, win32::WM_LBUTTONDOWN, win32::WM_LBUTTONUP),
                1 => (win32::VK_MBUTTON, win32::WM_MBUTTONDOWN, win32::WM_MBUTTONUP),
                2 => (win32::VK_RBUTTON, win32::WM_RBUTTONDOWN, win32::WM_RBUTTONUP),
                _ => return,
            };
            input.set_key(vk, down);
            mouse(input, out, if down { d } else { u });
        }
        Event::Wheel(steps) => {
            if steps != 0 {
                let (x, y) = input.cursor;
                win32::post(out, input, win32::WM_MOUSEWHEEL, input.mouse_keys() | ((steps as i16 as u16 as u32) << 16), win32::mouse_lparam(x, y));
            }
        }
        Event::Key { vk, down } => {
            let alt = input.down[usize::from(win32::VK_MENU)];
            let was = input.set_key(vk, down);
            let generic = match vk {
                win32::VK_LSHIFT | win32::VK_RSHIFT => win32::VK_SHIFT,
                win32::VK_LCONTROL | win32::VK_RCONTROL => win32::VK_CONTROL,
                win32::VK_LMENU | win32::VK_RMENU => win32::VK_MENU,
                v => v,
            };
            let alt_now = input.down[usize::from(win32::VK_MENU)];
            let sys = alt_now || generic == win32::VK_MENU || (alt && !down);
            let message = match (down, sys) {
                (true, false) => win32::WM_KEYDOWN,
                (true, true) => win32::WM_SYSKEYDOWN,
                (false, false) => win32::WM_KEYUP,
                (false, true) => win32::WM_SYSKEYUP,
            };
            let lparam = win32::key_lparam(win32::scan_code(vk), win32::is_extended(vk), was, !down);
            win32::post(out, input, message, u32::from(generic), lparam);
        }
        Event::Focus(on) => {
            if !on {
                for vk in 0..=255u8 {
                    if input.down[usize::from(vk)] {
                        input.set_key(vk, false);
                    }
                }
            }
            win32::post(out, input, win32::WM_ACTIVATEAPP, u32::from(on), 0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_text_events() {
        assert_eq!(Event::parse("m 100 200"), Some(Event::Move { fx: 100, fy: 200 }));
        assert_eq!(Event::parse("b 2 1"), Some(Event::Button { button: 2, down: true }));
        assert_eq!(Event::parse("k 27 0"), Some(Event::Key { vk: 27, down: false }));
        assert_eq!(Event::parse("x 1"), None);
        assert_eq!(Event::parse("m 1"), None);
    }

    #[test]
    fn click_becomes_win32_messages() {
        let (mut input, mut out) = (InputState::default(), VecDeque::new());
        apply(Event::Move { fx: 32768, fy: 32768 }, (800, 600), &mut input, &mut out);
        apply(Event::Button { button: 0, down: true }, (800, 600), &mut input, &mut out);
        apply(Event::Button { button: 0, down: false }, (800, 600), &mut input, &mut out);
        assert_eq!(input.cursor, (400, 300));
        let m: Vec<_> = out.iter().map(|m| m.message).collect();
        assert_eq!(m, vec![win32::WM_MOUSEMOVE, win32::WM_LBUTTONDOWN, win32::WM_LBUTTONUP]);
    }
}
