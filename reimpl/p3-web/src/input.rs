//! Eventos del navegador → mensajes Win32 del juego. El hilo principal escribe eventos de 4 palabras en un anillo
//! `SharedArrayBuffer` y el Worker los recoge con `p3_input_pop`; aquí se traducen igual que `p3-client/src/window.rs`.
use p3_mvp::ui_rt::win32::{self, InputState, Msg};
use std::collections::VecDeque;

pub const EV_MOVE: u32 = 1;
pub const EV_BUTTON: u32 = 2;
pub const EV_WHEEL: u32 = 3;
pub const EV_KEY: u32 = 4;
pub const EV_FOCUS: u32 = 5;
pub const EV_QUIT: u32 = 6;

/// Evento ya decodificado. `Move` lleva la posición normalizada a `0..=65535` sobre el lienzo: el modo del juego puede cambiar
/// (resoluciones modernas) y la página no lo sabe.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Move { fx: u32, fy: u32 },
    /// `button`: 0 izquierdo, 1 central, 2 derecho (como `MouseEvent.button`).
    Button { button: u32, down: bool },
    /// Pasos de rueda de 120 por muesca, con signo.
    Wheel(i32),
    /// Código virtual de Windows ya traducido por la página.
    Key { vk: u8, down: bool },
    Focus(bool),
    Quit,
}

impl Event {
    pub fn decode(w: [u32; 4]) -> Option<Event> {
        Some(match w[0] {
            EV_MOVE => Event::Move { fx: w[1], fy: w[2] },
            EV_BUTTON => Event::Button { button: w[1], down: w[2] != 0 },
            EV_WHEEL => Event::Wheel(w[1] as i32),
            EV_KEY => Event::Key { vk: w[1] as u8, down: w[2] != 0 },
            EV_FOCUS => Event::Focus(w[1] != 0),
            EV_QUIT => Event::Quit,
            _ => return None,
        })
    }
}

fn post(out: &mut VecDeque<Msg>, input: &InputState, message: u32, wparam: u32, lparam: u32) {
    win32::post(out, input, message, wparam, lparam);
}

/// Aplica un evento: actualiza `input` y encola los mensajes. `false` si hay que cerrar el juego.
pub fn apply(ev: Event, game: (usize, usize), input: &mut InputState, out: &mut VecDeque<Msg>) -> bool {
    let mouse = |input: &InputState, out: &mut VecDeque<Msg>, m: u32| {
        let (x, y) = input.cursor;
        post(out, input, m, input.mouse_keys(), win32::mouse_lparam(x, y));
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
                _ => return true,
            };
            input.set_key(vk, down);
            mouse(input, out, if down { d } else { u });
        }
        Event::Wheel(steps) => {
            if steps != 0 {
                let (x, y) = input.cursor;
                post(out, input, win32::WM_MOUSEWHEEL, input.mouse_keys() | ((steps as i16 as u16 as u32) << 16), win32::mouse_lparam(x, y));
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
            post(out, input, message, u32::from(generic), lparam);
        }
        Event::Focus(on) => {
            if !on {
                for vk in 0..=255u8 {
                    if input.down[usize::from(vk)] {
                        input.set_key(vk, false);
                    }
                }
            }
            post(out, input, win32::WM_ACTIVATEAPP, u32::from(on), 0);
        }
        Event::Quit => return false,
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_and_click_become_win32_messages() {
        let (mut input, mut out) = (InputState::default(), VecDeque::new());
        assert!(apply(Event::decode([EV_MOVE, 32768, 32768, 0]).unwrap(), (800, 600), &mut input, &mut out));
        assert_eq!(input.cursor, (400, 300));
        apply(Event::Button { button: 0, down: true }, (800, 600), &mut input, &mut out);
        apply(Event::Button { button: 0, down: false }, (800, 600), &mut input, &mut out);
        let m: Vec<_> = out.iter().map(|m| (m.message, m.lparam)).collect();
        let lp = win32::mouse_lparam(400, 300);
        assert_eq!(m, vec![(win32::WM_MOUSEMOVE, lp), (win32::WM_LBUTTONDOWN, lp), (win32::WM_LBUTTONUP, lp)]);
        assert_eq!(out[1].wparam, win32::MK_LBUTTON);
    }

    #[test]
    fn edge_positions_stay_inside_the_game() {
        let (mut input, mut out) = (InputState::default(), VecDeque::new());
        apply(Event::Move { fx: 65535, fy: 65535 }, (800, 600), &mut input, &mut out);
        assert_eq!(input.cursor, (799, 599));
    }

    #[test]
    fn alt_keys_are_system_messages_and_quit_closes() {
        let (mut input, mut out) = (InputState::default(), VecDeque::new());
        apply(Event::Key { vk: win32::VK_LMENU, down: true }, (800, 600), &mut input, &mut out);
        apply(Event::Key { vk: b'A', down: true }, (800, 600), &mut input, &mut out);
        assert_eq!(out[0].message, win32::WM_SYSKEYDOWN);
        assert_eq!(out[1].message, win32::WM_SYSKEYDOWN);
        assert!(!apply(Event::Quit, (800, 600), &mut input, &mut out));
        assert_eq!(Event::decode([99, 0, 0, 0]), None);
    }
}
