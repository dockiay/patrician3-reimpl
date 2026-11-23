//! Mensajes y entrada de Windows para el enlazador: la ventana nativa (`reimpl/p3-client`, binario `p3-juego`) recoge ratón y teclado
//! y los deja aquí con el formato de Win32; el código portado los lee con las mismas importaciones que el original
//! (`PeekMessageA`, `GetMessageA`, `TranslateMessage`, `DispatchMessageA`, `GetCursorPos`, `GetAsyncKeyState`, `GetKeyState`,
//! `timeGetTime`). El bucle sigue dentro del código emulado (`screen_list_run_frame` `0x4B8A40` → `present_frame` `0x4B8FF0`
//! → `sgl_SwapBuffer`); en `sgl_SwapBuffer` se llama a [`Present`], que presenta la superficie y recoge la entrada nueva.
//! Acuerdo entre hilos: `reimpl/UI_PANTALLA_COMPLETA.md`.
use std::collections::VecDeque;

pub const WM_NULL: u32 = 0x0000;
pub const WM_DESTROY: u32 = 0x0002;
pub const WM_ACTIVATE: u32 = 0x0006;
pub const WM_SETFOCUS: u32 = 0x0007;
pub const WM_KILLFOCUS: u32 = 0x0008;
pub const WM_PAINT: u32 = 0x000F;
pub const WM_CLOSE: u32 = 0x0010;
pub const WM_QUIT: u32 = 0x0012;
pub const WM_ACTIVATEAPP: u32 = 0x001C;
pub const WM_SETCURSOR: u32 = 0x0020;
pub const WM_KEYDOWN: u32 = 0x0100;
pub const WM_KEYUP: u32 = 0x0101;
pub const WM_CHAR: u32 = 0x0102;
pub const WM_SYSKEYDOWN: u32 = 0x0104;
pub const WM_SYSKEYUP: u32 = 0x0105;
pub const WM_SYSCHAR: u32 = 0x0106;
pub const WM_MOUSEMOVE: u32 = 0x0200;
pub const WM_LBUTTONDOWN: u32 = 0x0201;
pub const WM_LBUTTONUP: u32 = 0x0202;
pub const WM_LBUTTONDBLCLK: u32 = 0x0203;
pub const WM_RBUTTONDOWN: u32 = 0x0204;
pub const WM_RBUTTONUP: u32 = 0x0205;
pub const WM_RBUTTONDBLCLK: u32 = 0x0206;
pub const WM_MBUTTONDOWN: u32 = 0x0207;
pub const WM_MBUTTONUP: u32 = 0x0208;
pub const WM_MOUSEWHEEL: u32 = 0x020A;

/// Bits de `wParam` de los mensajes de ratón.
pub const MK_LBUTTON: u32 = 0x0001;
pub const MK_RBUTTON: u32 = 0x0002;
pub const MK_SHIFT: u32 = 0x0004;
pub const MK_CONTROL: u32 = 0x0008;
pub const MK_MBUTTON: u32 = 0x0010;

pub const VK_LBUTTON: u8 = 0x01;
pub const VK_RBUTTON: u8 = 0x02;
pub const VK_MBUTTON: u8 = 0x04;
pub const VK_BACK: u8 = 0x08;
pub const VK_TAB: u8 = 0x09;
pub const VK_RETURN: u8 = 0x0D;
pub const VK_SHIFT: u8 = 0x10;
pub const VK_CONTROL: u8 = 0x11;
pub const VK_MENU: u8 = 0x12;
pub const VK_PAUSE: u8 = 0x13;
pub const VK_CAPITAL: u8 = 0x14;
pub const VK_ESCAPE: u8 = 0x1B;
pub const VK_SPACE: u8 = 0x20;
pub const VK_PRIOR: u8 = 0x21;
pub const VK_NEXT: u8 = 0x22;
pub const VK_END: u8 = 0x23;
pub const VK_HOME: u8 = 0x24;
pub const VK_LEFT: u8 = 0x25;
pub const VK_UP: u8 = 0x26;
pub const VK_RIGHT: u8 = 0x27;
pub const VK_DOWN: u8 = 0x28;
pub const VK_SNAPSHOT: u8 = 0x2C;
pub const VK_INSERT: u8 = 0x2D;
pub const VK_DELETE: u8 = 0x2E;
/// `'0'..'9'` y `'A'..'Z'` son su propio código ASCII.
pub const VK_NUMPAD0: u8 = 0x60;
pub const VK_MULTIPLY: u8 = 0x6A;
pub const VK_ADD: u8 = 0x6B;
pub const VK_SUBTRACT: u8 = 0x6D;
pub const VK_DECIMAL: u8 = 0x6E;
pub const VK_DIVIDE: u8 = 0x6F;
pub const VK_F1: u8 = 0x70;
pub const VK_NUMLOCK: u8 = 0x90;
pub const VK_SCROLL: u8 = 0x91;
pub const VK_LSHIFT: u8 = 0xA0;
pub const VK_RSHIFT: u8 = 0xA1;
pub const VK_LCONTROL: u8 = 0xA2;
pub const VK_RCONTROL: u8 = 0xA3;
pub const VK_LMENU: u8 = 0xA4;
pub const VK_RMENU: u8 = 0xA5;
pub const VK_OEM_PLUS: u8 = 0xBB;
pub const VK_OEM_COMMA: u8 = 0xBC;
pub const VK_OEM_MINUS: u8 = 0xBD;
pub const VK_OEM_PERIOD: u8 = 0xBE;

/// `MSG` de Win32 (28 bytes: `hwnd`, `message`, `wParam`, `lParam`, `time`, `pt.x`, `pt.y`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Msg {
    pub hwnd: u32,
    pub message: u32,
    pub wparam: u32,
    pub lparam: u32,
    pub time: u32,
    pub pt: (i32, i32),
}

impl Msg {
    pub const SIZE: u32 = 28;

    pub fn words(&self) -> [u32; 7] {
        [self.hwnd, self.message, self.wparam, self.lparam, self.time, self.pt.0 as u32, self.pt.1 as u32]
    }

    pub fn from_words(w: [u32; 7]) -> Self {
        Msg { hwnd: w[0], message: w[1], wparam: w[2], lparam: w[3], time: w[4], pt: (w[5] as i32, w[6] as i32) }
    }
}

/// `lParam` de un mensaje de ratón: `x` en la palabra baja y `y` en la alta, con signo.
pub fn mouse_lparam(x: i32, y: i32) -> u32 {
    (x as u16 as u32) | ((y as u16 as u32) << 16)
}

/// `lParam` de `WM_KEYDOWN`/`WM_KEYUP`: repetición 1, código de exploración, bit 24 extendida, bit 30 estado previo, bit 31 transición.
pub fn key_lparam(scan: u8, extended: bool, was_down: bool, up: bool) -> u32 {
    1 | (u32::from(scan) << 16) | (u32::from(extended) << 24) | (u32::from(was_down || up) << 30) | (u32::from(up) << 31)
}

/// Estado de ratón y teclado que leen `GetCursorPos`, `GetKeyState` y `GetAsyncKeyState`. Coordenadas del cliente (la ventana
/// del juego ocupa toda la pantalla lógica, así que coinciden con las de pantalla que devolvería Windows).
#[derive(Debug, Clone)]
pub struct InputState {
    pub cursor: (i32, i32),
    /// Tecla pulsada ahora (por código virtual; incluye `VK_LBUTTON`/`VK_RBUTTON`/`VK_MBUTTON`).
    pub down: [bool; 256],
    /// Bit bajo de `GetKeyState`: alterna con cada pulsación (bloq. mayús., bloq. núm.…).
    pub toggled: [bool; 256],
    /// Bit bajo de `GetAsyncKeyState`: pulsada desde la última consulta.
    pub pressed_since: [bool; 256],
    /// Milisegundos de `timeGetTime` (los pone la ventana nativa con su reloj; sin ventana, `P3_TIME`).
    pub time_ms: Option<u32>,
}

impl Default for InputState {
    fn default() -> Self {
        InputState { cursor: (0, 0), down: [false; 256], toggled: [false; 256], pressed_since: [false; 256], time_ms: None }
    }
}

impl InputState {
    /// Anota una pulsación o una suelta (teclas y botones) y devuelve si la tecla ya estaba pulsada.
    pub fn set_key(&mut self, vk: u8, down: bool) -> bool {
        let i = usize::from(vk);
        let was = self.down[i];
        if down && !was {
            self.toggled[i] = !self.toggled[i];
            self.pressed_since[i] = true;
        }
        self.down[i] = down;
        // Las variantes izquierda/derecha también cuentan como la genérica, como en Windows.
        let generic = match vk {
            VK_LSHIFT | VK_RSHIFT => Some(VK_SHIFT),
            VK_LCONTROL | VK_RCONTROL => Some(VK_CONTROL),
            VK_LMENU | VK_RMENU => Some(VK_MENU),
            _ => None,
        };
        if let Some(g) = generic {
            let (l, r) = match g {
                VK_SHIFT => (VK_LSHIFT, VK_RSHIFT),
                VK_CONTROL => (VK_LCONTROL, VK_RCONTROL),
                _ => (VK_LMENU, VK_RMENU),
            };
            let any = self.down[usize::from(l)] || self.down[usize::from(r)];
            self.set_key(g, any);
        }
        was
    }

    /// `GetKeyState(vk)`: bit 15 = pulsada, bit 0 = alternada.
    pub fn key_state(&self, vk: u32) -> u32 {
        let i = (vk & 0xFF) as usize;
        let hi: u16 = if self.down[i] { 0x8000 } else { 0 };
        let v = (hi | u16::from(self.toggled[i])) as i16;
        v as i32 as u32
    }

    /// `GetAsyncKeyState(vk)`: bit 15 = pulsada, bit 0 = pulsada desde la última consulta (que lo borra).
    pub fn async_key_state(&mut self, vk: u32) -> u32 {
        let i = (vk & 0xFF) as usize;
        let hi: u16 = if self.down[i] { 0x8000 } else { 0 };
        let lo = u16::from(std::mem::take(&mut self.pressed_since[i]));
        ((hi | lo) as i16) as i32 as u32
    }

    /// Bits `MK_*` del `wParam` de un mensaje de ratón con el estado actual.
    pub fn mouse_keys(&self) -> u32 {
        let b = |vk: u8, bit: u32| if self.down[usize::from(vk)] { bit } else { 0 };
        b(VK_LBUTTON, MK_LBUTTON) | b(VK_RBUTTON, MK_RBUTTON) | b(VK_MBUTTON, MK_MBUTTON) | b(VK_SHIFT, MK_SHIFT) | b(VK_CONTROL, MK_CONTROL)
    }
}

/// Gancho de presentación (`sgl_SwapBuffer`): recibe la superficie del fotograma, la cola de mensajes y el estado de entrada; la
/// ventana nativa muestra la superficie, añade los mensajes nuevos y actualiza la entrada. `false` = cerrar el juego.
pub type Present = Box<dyn FnMut(&super::Surface, &mut VecDeque<Msg>, &mut InputState) -> bool>;

/// `HWND` de la ventana principal en los mensajes que encola la ventana nativa.
pub const MAIN_HWND: u32 = 1;

/// `PostMessage` a la ventana principal con la hora y el cursor actuales.
pub fn post(queue: &mut VecDeque<Msg>, input: &InputState, message: u32, wparam: u32, lparam: u32) {
    let time = input.time_ms.unwrap_or(0);
    queue.push_back(Msg { hwnd: MAIN_HWND, message, wparam, lparam, time, pt: input.cursor });
}

/// `PeekMessageA(msg, hwnd, min, max, remove)` sin filtro de ventana (el juego pasa 0). Devuelve el mensaje si hay.
pub fn peek(queue: &mut VecDeque<Msg>, min: u32, max: u32, remove: bool) -> Option<Msg> {
    let fits = |m: &Msg| (min == 0 && max == 0) || (m.message >= min && m.message <= max);
    let at = queue.iter().position(fits)?;
    if remove {
        queue.remove(at)
    } else {
        queue.get(at).copied()
    }
}

/// `ToAscii` con la distribución estadounidense (la del juego en inglés): letras, números, espacio y teclas de control. `None` si la
/// tecla no da carácter.
pub fn to_ascii(vk: u8, shift: bool, caps: bool) -> Option<u8> {
    const SHIFTED: &[u8; 10] = b")!@#$%^&*(";
    Some(match vk {
        b'A'..=b'Z' => if shift != caps { vk } else { vk + 32 },
        b'0'..=b'9' => if shift { SHIFTED[usize::from(vk - b'0')] } else { vk },
        0x60..=0x69 => b'0' + (vk - 0x60),
        VK_SPACE => b' ',
        VK_RETURN => b'\r',
        VK_BACK => 8,
        VK_TAB => b'\t',
        VK_ESCAPE => 0x1B,
        VK_MULTIPLY => b'*',
        VK_ADD => b'+',
        VK_SUBTRACT => b'-',
        VK_DECIMAL => b'.',
        VK_DIVIDE => b'/',
        VK_OEM_PLUS => if shift { b'+' } else { b'=' },
        VK_OEM_MINUS => if shift { b'_' } else { b'-' },
        VK_OEM_COMMA => if shift { b'<' } else { b',' },
        VK_OEM_PERIOD => if shift { b'>' } else { b'.' },
        _ => return None,
    })
}

/// Teclas que llevan el bit «extendida» en el `lParam`.
pub fn is_extended(vk: u8) -> bool {
    matches!(vk, VK_PRIOR | VK_NEXT | VK_END | VK_HOME | VK_LEFT | VK_UP | VK_RIGHT | VK_DOWN | VK_INSERT | VK_DELETE | VK_DIVIDE | VK_RCONTROL | VK_RMENU)
        || vk == VK_NUMLOCK
}

/// Código de exploración (conjunto 1) de las teclas más comunes; 0 si no se conoce. El juego no lo lee, pero lo lleva `lParam`.
pub fn scan_code(vk: u8) -> u8 {
    const LETTERS: &[u8; 26] = b"\x1e\x30\x2e\x20\x12\x21\x22\x23\x17\x24\x25\x26\x32\x31\x18\x19\x10\x13\x1f\x14\x16\x2f\x11\x2d\x15\x2c";
    match vk {
        b'A'..=b'Z' => LETTERS[usize::from(vk - b'A')],
        b'1'..=b'9' => 0x02 + (vk - b'1'),
        b'0' => 0x0B,
        VK_ESCAPE => 0x01,
        VK_BACK => 0x0E,
        VK_TAB => 0x0F,
        VK_RETURN => 0x1C,
        VK_CONTROL | VK_LCONTROL | VK_RCONTROL => 0x1D,
        VK_SHIFT | VK_LSHIFT => 0x2A,
        VK_RSHIFT => 0x36,
        VK_MENU | VK_LMENU | VK_RMENU => 0x38,
        VK_SPACE => 0x39,
        VK_F1..=0x79 => 0x3B + (vk - VK_F1),
        VK_HOME => 0x47,
        VK_UP => 0x48,
        VK_PRIOR => 0x49,
        VK_LEFT => 0x4B,
        VK_RIGHT => 0x4D,
        VK_END => 0x4F,
        VK_DOWN => 0x50,
        VK_NEXT => 0x51,
        VK_INSERT => 0x52,
        VK_DELETE => 0x53,
        VK_ADD => 0x4E,
        VK_SUBTRACT => 0x4A,
        VK_OEM_PLUS => 0x1B,
        VK_OEM_MINUS => 0x35,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_states_follow_windows() {
        let mut s = InputState::default();
        assert_eq!(s.key_state(u32::from(VK_CAPITAL)), 0);
        s.set_key(VK_CAPITAL, true);
        assert_eq!(s.key_state(u32::from(VK_CAPITAL)), 0xFFFF_8001);
        s.set_key(VK_CAPITAL, false);
        assert_eq!(s.key_state(u32::from(VK_CAPITAL)), 1);
        s.set_key(VK_LSHIFT, true);
        assert_eq!(s.key_state(u32::from(VK_SHIFT)) & 0x8000, 0x8000);
        assert_eq!(s.async_key_state(u32::from(VK_SHIFT)), 0xFFFF_8001);
        assert_eq!(s.async_key_state(u32::from(VK_SHIFT)), 0xFFFF_8000);
        s.set_key(VK_LSHIFT, false);
        assert_eq!(s.key_state(u32::from(VK_SHIFT)) & 0x8000, 0);
        s.set_key(VK_LBUTTON, true);
        assert_eq!(s.mouse_keys(), MK_LBUTTON);
    }

    #[test]
    fn queue_peek_and_lparams() {
        let mut q = VecDeque::new();
        let input = InputState { cursor: (10, 20), ..Default::default() };
        post(&mut q, &input, WM_MOUSEMOVE, 0, mouse_lparam(10, 20));
        post(&mut q, &input, WM_KEYDOWN, u32::from(VK_ESCAPE), key_lparam(1, false, false, false));
        assert_eq!(mouse_lparam(-1, 2), 0x0002_FFFF);
        assert_eq!(key_lparam(1, false, true, true), 0xC001_0001);
        assert_eq!(peek(&mut q, WM_KEYDOWN, WM_KEYDOWN, false).unwrap().message, WM_KEYDOWN);
        assert_eq!(q.len(), 2);
        let m = peek(&mut q, 0, 0, true).unwrap();
        assert_eq!((m.message, m.pt, m.hwnd), (WM_MOUSEMOVE, (10, 20), 1));
        assert_eq!(Msg::from_words(m.words()), m);
        assert_eq!(q.len(), 1);
    }
}
