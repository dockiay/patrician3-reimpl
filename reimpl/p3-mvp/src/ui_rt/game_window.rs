//! Ventana principal de Windows del juego (`GameWindow`, vtable `0x00671150`; el objeto global `GameMainWindow` `0x006CC3F0` deriva de
//! ella y comparte su mapa de mensajes) y su camino hasta la interfaz: `DispatchMessageA` busca el mensaje en el mapa de mensajes de
//! MFC (`AFX_MSGMAP` `0x00671148` → entradas `0x00670F98`, devuelto por la ranura 10 `0x004C09B0`), desempaqueta `wParam`/`lParam`
//! según la firma de MFC y llama al manejador. Los manejadores (`0x004C0B50..0x004C0FA0`), los reenvíos a la ventana superior de la
//! lista de pantallas (`0x004B92E0..0x004B9410`) y el recorte del cursor `0x004BD5E0` están portados aquí a mano (comprobados contra el
//! x86 con `oracle/x86_game_window.py` y `p3-mvp/tests/ui_game_window.rs`).
//!
//! Lo que guardan los manejadores es lo que lee el resto del juego:
//! - Teclado: `0x006DCBC0 + tecla` bit 0 (pulsada alguna vez, lo consume quien la lee) y bit 1 (pulsada ahora); `0x006DD598` = hay tecla
//!   normal pulsada. Ratón: bytes `0x006DCBC1` (izquierdo) y `0x006DCBC2` (derecho) con los mismos bits.
//! - Cursor del juego en `0x006DAB50` (x) y `0x006DAB48` (y), escalado de la ventana (`0x006DAB4C`×`0x006DAB44`) a la resolución del
//!   juego (`0x006DCCE0`×`0x006DCCE4`) y recortado al rectángulo `0x006BED64..0x006BED70`; punto de pulsación izquierda en
//!   `0x006BED74/78` (-1 tras soltar o con el derecho).
use p3_render::scene_math::Mem;

/// Objeto de la ventana principal (`GameMainWindow`).
pub const MAIN_WINDOW: u32 = 0x006C_C3F0;
/// Lista de pantallas (`ScreenList`).
pub const SCREEN_LIST: u32 = 0x006D_A5F0;
/// Mapa de mensajes de `GameWindow`: `{base, entradas}`.
pub const MESSAGE_MAP: u32 = 0x0067_1148;

pub const CURSOR_X: u32 = 0x006D_AB50;
pub const CURSOR_Y: u32 = 0x006D_AB48;
pub const KEY_FLAGS: u32 = 0x006D_CBC0;

/// Firmas de MFC (`AfxSig_*`) que usa este mapa.
const SIG_VV: u32 = 0x0C;
const SIG_VWWW: u32 = 0x10;
const SIG_VWII: u32 = 0x11;
const SIG_BWWW: u32 = 0x03;
const SIG_VWWB: u32 = 0x1C;
const SIG_WP: u32 = 0x22;
const SIG_VW_WND: u32 = 0x2C;
const SIG_BWSP: u32 = 0x2E;
const SIG_VWP: u32 = 0x31;

/// Entrada del mapa de mensajes para `message` (sin código de notificación ni identificador): (firma, manejador).
pub fn lookup(m: &mut dyn Mem, map: u32, message: u32) -> Option<(u32, u32)> {
    let mut e = m.r32(map + 4);
    for _ in 0..256 {
        let (msg, sig, pfn) = (m.r32(e), m.r32(e + 16), m.r32(e + 20));
        if sig == 0 && pfn == 0 {
            return None;
        }
        if msg == message && m.r32(e + 4) == 0 && m.r32(e + 8) == 0 {
            return Some((sig, pfn));
        }
        e += 24;
    }
    None
}

/// Argumentos del manejador según la firma de MFC (`CWnd::OnWndMsg`). `wnd` es el `CWnd*` de las firmas que lo reciben (la propia
/// ventana si el `HWND` es el suyo; si no, 0).
pub fn handler_args(sig: u32, wparam: u32, lparam: u32, wnd: u32) -> Option<Vec<u32>> {
    let lo = |v: u32| v & 0xFFFF;
    let hi = |v: u32| v >> 16;
    let slo = |v: u32| (v as u16 as i16) as i32 as u32;
    let shi = |v: u32| ((v >> 16) as u16 as i16) as i32 as u32;
    Some(match sig {
        SIG_VV => vec![],
        SIG_VWWW | SIG_VWII => vec![wparam, lo(lparam), hi(lparam)],
        SIG_BWWW => vec![wnd, lo(lparam), hi(lparam)],
        SIG_VWWB => vec![lo(wparam), wnd, hi(wparam)],
        SIG_WP => vec![slo(lparam), shi(lparam)],
        SIG_VW_WND => vec![wnd],
        SIG_BWSP => vec![lo(wparam), shi(wparam), slo(lparam), shi(lparam)],
        SIG_VWP => vec![wparam, slo(lparam), shi(lparam)],
        _ => return None,
    })
}

impl super::Runtime<'_> {
    /// `DispatchMessageA(msg)`: el mensaje va a la ventana principal por su mapa de mensajes. Sin entrada, `DefWindowProc` (0).
    pub(super) fn dispatch_window_message(&mut self, m: &mut dyn Mem, msg_ptr: u32) -> u32 {
        let (hwnd, message, wparam, lparam) = (m.r32(msg_ptr), m.r32(msg_ptr + 4), m.r32(msg_ptr + 8), m.r32(msg_ptr + 12));
        let obj = MAIN_WINDOW;
        // El mapa sale de la ranura 10 (`GetMessageMap`) de la vtable del objeto; sin objeto construido, nada que despachar.
        let vt = m.r32(obj);
        if vt == 0 {
            return 0;
        }
        let get_map = m.r32(vt + 40);
        let map = if get_map == 0x004C_09B0 { MESSAGE_MAP } else { return 0 };
        let Some((sig, pfn)) = lookup(m, map, message) else { return 0 };
        let wnd = if hwnd == m.r32(obj + 0x1C) { obj } else { 0 };
        let wnd_arg = if sig == SIG_VW_WND { if lparam == m.r32(obj + 0x1C) && lparam != 0 { obj } else { 0 } } else { wnd };
        let Some(args) = handler_args(sig, wparam, lparam, wnd_arg) else { return 0 };
        self.dispatch(m, pfn, obj, &args)
    }

    /// Manejadores de `GameWindow`, reenvíos de la lista de pantallas y recorte del cursor. `None` = no es de aquí.
    pub(super) fn window_native(&mut self, m: &mut dyn Mem, f: u32, ecx: u32, esp: u32) -> Option<u32> {
        if f == 0x004C_0FA0 {
            let delta = m.r32(esp + 8) as i16 as i32;
            let point = (m.r32(esp + 12) as i32, m.r32(esp + 16) as i32);
            if self.zoom_wheel(m, delta, point) { return Some(0); }
        }
        let size = (self.surface.width as i32, self.surface.height as i32);
        native(&mut RtHost(self), size, m, f, ecx, esp)
    }
}

/// Llamadas que hacen los manejadores (en el enlazador, `Runtime::dispatch`; en la prueba de paridad, un registro).
pub trait WindowHost {
    fn call(&mut self, m: &mut dyn Mem, f: u32, ecx: u32, args: &[u32]) -> u32;
}

struct RtHost<'r, 'a>(&'r mut super::Runtime<'a>);
impl WindowHost for RtHost<'_, '_> {
    fn call(&mut self, m: &mut dyn Mem, f: u32, ecx: u32, args: &[u32]) -> u32 {
        self.0.dispatch(m, f, ecx, args)
    }
}

/// Porte de los manejadores. `client` = tamaño del área cliente (`GetClientRect`). `None` = `f` no es de aquí.
pub fn native(host: &mut dyn WindowHost, client: (i32, i32), m: &mut dyn Mem, f: u32, ecx: u32, esp: u32) -> Option<u32> {
    {
        let arg = |m: &mut dyn Mem, i: u32| m.r32(esp + 4 + 4 * i);
        let or8 = |m: &mut dyn Mem, a: u32, v: u8| {
            let b = m.r8(a);
            m.w8(a, b | v)
        };
        let and8 = |m: &mut dyn Mem, a: u32, v: u8| {
            let b = m.r8(a);
            m.w8(a, b & v)
        };
        Some(match f {
            // `CWnd::Default` (`0x006504F2`): `DefWindowProc` del último mensaje; nada que hacer para el juego. `WM_NCHITTEST` sería
            // `HTCLIENT`, pero su manejador no usa el resultado más que para devolverlo.
            0x0065_04F2 => 0,
            // `CWnd::FromHandle` (`0x0065058B`): solo se usa para descartar el resultado de `SetCapture`.
            0x0065_058B => 0,
            // `WM_SYSKEYUP` (`0x004C0B50`): baja el bit «pulsada» y avisa a la pantalla superior como una pulsación (`0x004B92E0`).
            0x004C_0B50 => {
                let (key, rep, flags) = (arg(m, 0), arg(m, 1), arg(m, 2));
                and8(m, KEY_FLAGS.wrapping_add(key), 0xFD);
                host.call(m, 0x0065_04F2, ecx, &[]);
                host.call(m, 0x004B_92E0, SCREEN_LIST, &[key, rep, flags])
            }
            // `WM_SYSKEYDOWN` (`0x004C0B80`).
            0x004C_0B80 => {
                let key = arg(m, 0);
                or8(m, KEY_FLAGS.wrapping_add(key), 3);
                host.call(m, 0x0065_04F2, ecx, &[])
            }
            // `WM_KEYUP` (`0x004C0BA0`).
            0x004C_0BA0 => {
                let (key, rep, flags) = (arg(m, 0), arg(m, 1), arg(m, 2));
                and8(m, KEY_FLAGS.wrapping_add(key), 0xFD);
                host.call(m, 0x0065_04F2, ecx, &[]);
                let r = host.call(m, 0x004B_9300, SCREEN_LIST, &[key, rep, flags]);
                m.w8(0x006D_D598, 0);
                r
            }
            // `WM_KEYDOWN` (`0x004C0BE0`).
            0x004C_0BE0 => {
                let (key, rep, flags) = (arg(m, 0), arg(m, 1), arg(m, 2));
                or8(m, KEY_FLAGS.wrapping_add(key), 3);
                host.call(m, 0x0065_04F2, ecx, &[]);
                m.w8(0x006D_D598, 1);
                host.call(m, 0x004B_92E0, SCREEN_LIST, &[key, rep, flags])
            }
            // Botones (`0x004C0C20` izquierdo abajo, `0x004C0C60` arriba, `0x004C0CA0`/`0x004C0CE0` derecho): usan el cursor ya guardado
            // por `WM_MOUSEMOVE`, no el punto del mensaje.
            0x004C_0C20 | 0x004C_0C60 | 0x004C_0CA0 | 0x004C_0CE0 => {
                let flags = arg(m, 0);
                let (byte, down, fwd) = match f {
                    0x004C_0C20 => (0x006D_CBC1, true, 0x004B_9320),
                    0x004C_0C60 => (0x006D_CBC1, false, 0x004B_9350),
                    0x004C_0CA0 => (0x006D_CBC2, true, 0x004B_9380),
                    _ => (0x006D_CBC2, false, 0x004B_93B0),
                };
                if down {
                    or8(m, byte, 3);
                } else {
                    and8(m, byte, 0xFD);
                }
                host.call(m, 0x0065_04F2, ecx, &[]);
                let (x, y) = (m.r32(CURSOR_X), m.r32(CURSOR_Y));
                if f == 0x004C_0C20 {
                    m.w32(0x006B_ED74, x);
                    m.w32(0x006B_ED78, y);
                } else {
                    m.w32(0x006B_ED78, u32::MAX);
                    m.w32(0x006B_ED74, u32::MAX);
                }
                host.call(m, fwd, SCREEN_LIST, &[flags, x, y])
            }
            // `WM_CLOSE` (`0x004C0D20`, no cierra: se sale desde el menú) y `WM_SIZE` (`0x004C0E00`).
            0x004C_0D20 | 0x004C_0E00 => 0,
            // `WM_MOUSEMOVE` (`0x004C0D30`): escala el punto a la resolución del juego, lo recorta (`0x004BD5E0`), captura el ratón la
            // primera vez que entra en la ventana a pantalla completa, oculta el cursor del sistema si hacía falta y avisa a la pantalla
            // superior (`0x004B93E0`).
            0x004C_0D30 => {
                let (flags, px, py) = (arg(m, 0), arg(m, 1) as i32, arg(m, 2) as i32);
                let (gw, gh) = (m.r32(0x006D_CCE0) as i32, m.r32(0x006D_CCE4) as i32);
                if gw != 0 && gh != 0 {
                    let (ww, wh) = (m.r32(0x006D_AB4C) as i32, m.r32(0x006D_AB44) as i32);
                    // `idiv` por cero sería una excepción en el original; con el tamaño sin fijar no se escala.
                    if ww != 0 && wh != 0 {
                        m.w32(CURSOR_X, gw.wrapping_mul(px).wrapping_div(ww) as u32);
                        m.w32(CURSOR_Y, gh.wrapping_mul(py).wrapping_div(wh) as u32);
                    }
                }
                host.call(m, 0x004B_D5E0, 0, &[]);
                let (x, y) = (m.r32(CURSOR_X) as i32, m.r32(CURSOR_Y) as i32);
                let (cw, ch) = client;
                if m.r8(ecx + 0x3D) == 0 && (0..cw).contains(&x) && (0..ch).contains(&y) {
                    // `SetCapture` solo a pantalla completa (`0x006DCB90`); aquí la captura la lleva la ventana nativa.
                    m.w8(ecx + 0x3D, 1);
                }
                if m.r32(0x006D_CBA4) != 0 {
                    m.w32(0x006D_CBA4, 0);
                }
                host.call(m, 0x004B_93E0, SCREEN_LIST, &[flags, x as u32, y as u32]);
                host.call(m, 0x0065_04F2, ecx, &[])
            }
            // `WM_SETCURSOR` (`0x004C0E80`) ya está portado en `p3_core::ui_input` y `WM_PAINT` (`0x004C0F20`) en J08: van por sus tablas.
            // `WM_NCHITTEST` (`0x004C0EC0`).
            0x004C_0EC0 => host.call(m, 0x0065_04F2, ecx, &[]),
            // `WM_CAPTURECHANGED` (`0x004C0ED0`): si la captura pasa a otra ventana, se olvida.
            0x004C_0ED0 => {
                if arg(m, 0) != ecx {
                    m.w8(ecx + 0x3D, 0);
                }
                0
            }
            // `WM_ACTIVATE` (`0x004C0EE0`): al activarse, repintar entera la pantalla superior (ranura `0xD4`).
            0x004C_0EE0 => {
                if arg(m, 0) != 0 {
                    // El original pide dos veces la ventana superior (`0x004B9730`): una para comprobarla y otra para usarla.
                    if host.call(m, 0x004B_9730, SCREEN_LIST, &[]) != 0 {
                        let top = host.call(m, 0x004B_9730, SCREEN_LIST, &[]);
                        let vt = m.r32(top);
                        let slot = m.r32(vt + 0xD4);
                        host.call(m, slot, top, &[]);
                    }
                }
                host.call(m, 0x0065_04F2, ecx, &[])
            }
            // `WM_MOUSEWHEEL` (`0x004C0FA0`): rueda a la pantalla superior (`0x004B9410`: botones e incremento con signo).
            0x004C_0FA0 => {
                let (flags, delta) = (arg(m, 0), arg(m, 1));
                host.call(m, 0x004B_9410, SCREEN_LIST, &[flags, delta]);
                host.call(m, 0x0065_04F2, ecx, &[])
            }
            // Reenvíos a la ventana superior de la lista (`+0x10` cuenta, `+0xC` cabeza, nodo `+8` ventana).
            0x004B_92E0 | 0x004B_9300 | 0x004B_9320 | 0x004B_9350 | 0x004B_9380 | 0x004B_93B0 | 0x004B_93E0 | 0x004B_9410 => {
                if m.r32(ecx + 0x10) == 0 {
                    return Some(0);
                }
                let head = m.r32(ecx + 0xC);
                let top = m.r32(head + 8);
                let n = if f == 0x004B_9410 { 2 } else { 3 };
                let args: Vec<u32> = (0..n).map(|i| arg(m, i)).collect();
                let target = match f {
                    0x004B_9300 => 0x004B_6C10,
                    _ => {
                        let off = match f {
                            0x004B_92E0 => 0x1C,
                            0x004B_9320 => 0x13C,
                            0x004B_9350 => 0x140,
                            0x004B_9380 => 0x144,
                            0x004B_93B0 => 0x148,
                            0x004B_93E0 => 0x14C,
                            _ => 0x150,
                        };
                        let vt = m.r32(top);
                        m.r32(vt + off)
                    }
                };
                host.call(m, target, top, &args)
            }
            // `0x004BD5E0`: recorta el cursor del juego al rectángulo `0x006BED64` (izquierda, arriba, derecha, abajo; la derecha y la
            // abajo excluidas). El original tiene detrás un `ClientToScreen` + `SetCursorPos` que nunca se ejecuta (salto incondicional).
            0x004B_D5E0 => {
                let (l, t, r, b) = (m.r32(0x006B_ED64) as i32, m.r32(0x006B_ED68) as i32, m.r32(0x006B_ED6C) as i32, m.r32(0x006B_ED70) as i32);
                let mut moved = false;
                let x = m.r32(CURSOR_X) as i32;
                if x < l {
                    m.w32(CURSOR_X, l as u32);
                    moved = true;
                } else if x >= r {
                    m.w32(CURSOR_X, (r - 1) as u32);
                    moved = true;
                }
                // En y las dos comprobaciones van seguidas (en x la primera salta la segunda): con un rectángulo vacío la y acaba en
                // `abajo - 1`. La de abajo no marca el resultado.
                let mut y = m.r32(CURSOR_Y) as i32;
                if y < t {
                    y = t;
                    m.w32(CURSOR_Y, y as u32);
                    moved = true;
                }
                if y >= b {
                    m.w32(CURSOR_Y, (b - 1) as u32);
                }
                u32::from(moved)
            }
            _ => return None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mfc_signatures_unpack_like_onwndmsg() {
        assert_eq!(handler_args(SIG_VWP, 1, 0xFFFE_0010, 0), Some(vec![1, 0x10, 0xFFFF_FFFE]));
        assert_eq!(handler_args(SIG_VWWW, 0x41, 0xC01E_0001, 0), Some(vec![0x41, 1, 0xC01E]));
        assert_eq!(handler_args(SIG_BWSP, 0xFF88_0001, 0x0002_0003, 0), Some(vec![1, (-120i32) as u32, 3, 2]));
        assert_eq!(handler_args(SIG_VWWB, 0x0001_0002, 0, 7), Some(vec![2, 7, 1]));
        assert_eq!(handler_args(0x99, 0, 0, 0), None);
    }
}
