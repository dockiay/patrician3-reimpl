//! Restos del área: envolturas de la biblioteca SGL, cambio de resolución, rechazo de conexiones en red y marcadores de ámbito.
//!
//! Las envolturas `sgl_*` (`0x4BB140`–`0x4BBB20`) y las 15 envolturas `arc_*` del contenedor (`0x6389A0`–`0x6389F4`) son `jmp [importación]`: el porte llama a la
//! entrada de la tabla de importaciones con los mismos argumentos y ECX.

use super::{cdecl, free, Host, Mem, Ret};

/// Envolturas `jmp [iat]` del área: (función, entrada de la tabla de importaciones).
pub const THUNKS: [(u32, u32); 29] = [
    (0x4BB140, 0x6DAA8C),
    (0x4BB330, 0x6DAA28),
    (0x4BB340, 0x6DAA24),
    (0x4BB430, 0x6DA9F4),
    (0x4BB490, 0x6DA9DC),
    (0x4BB4A0, 0x6DA9D8),
    (0x4BB640, 0x6DA974),
    (0x4BB780, 0x6DA928),
    (0x4BB870, 0x6DA8EC),
    (0x4BB8F0, 0x6DA8E0),
    (0x4BB9B0, 0x6DA8AC),
    (0x4BB9C0, 0x6DA8A8),
    (0x4BBA10, 0x6DA898),
    (0x4BBB20, 0x6DA858),
    (0x6389A0, 0x66A02C),
    (0x6389A6, 0x66A028),
    (0x6389AC, 0x66A050),
    (0x6389B2, 0x66A020),
    (0x6389B8, 0x66A01C),
    (0x6389BE, 0x66A018),
    (0x6389C4, 0x66A024),
    (0x6389CA, 0x66A034),
    (0x6389D0, 0x66A038),
    (0x6389D6, 0x66A03C),
    (0x6389DC, 0x66A040),
    (0x6389E2, 0x66A044),
    (0x6389E8, 0x66A048),
    (0x6389EE, 0x66A04C),
    (0x6389F4, 0x66A030),
];

const SGL_INIT_SCREEN: u32 = 0x004B_B590;
const SGL_SET_SCREEN_DIMENSIONS: u32 = 0x004B_B9D0;
const SGL_SWITCH_SCREEN_MODE: u32 = 0x004B_BAC0;
const SGL_SET_ACTIVE_CLIPPER: u32 = 0x004B_B780;
const SGL_RESET_CLIPPER: u32 = 0x004B_B740;
const GET_WINDOW_LONG: u32 = 0x0066_A510;
const SET_WINDOW_LONG: u32 = 0x0066_A514;
const SET_WINDOW_POS: u32 = 0x0066_A518;
const GET_SYSTEM_METRICS: u32 = 0x0066_A51C;
const CLIENT_TO_SCREEN: u32 = 0x0066_A520;
const SET_CURSOR_POS: u32 = 0x0066_A524;
const OPERATION_RECORD_INIT: u32 = 0x0053_5740;
const NET_RECEIVE: u32 = 0x0066_A31C;
const NET_SEND: u32 = 0x0066_A2F0;
const NET_DROP: u32 = 0x0066_A2FC;
const NET_LOCAL_ADDRESS: u32 = 0x0066_A330;
const SCREEN_READY: u32 = 0x006D_CCC0;
const ONE_F32: u32 = 0x3F80_0000;
const THOUSAND_F32: u32 = 0x447A_0000;

fn import(m: &mut dyn Mem, h: &mut dyn Host, iat: u32, args: &[u32]) -> u32 {
    let f = m.r32(iat);
    cdecl(m, h, f, args)
}

/// `0x4BCCC0` (cdecl: ventana, configuración `{+4 bits, +8 ancho, +0xC alto, +0x20 pantalla completa}`): abre la pantalla o
/// cambia de modo, ajusta la ventana, los recortes y el cursor al nuevo tamaño; devuelve 1, o el resultado del cambio de modo.
fn set_ddraw_resolution(m: &mut dyn Mem, h: &mut dyn Host, a: &[u32], esp: u32) -> u32 {
    let (hwnd, cfg) = (a[0], a[1]);
    let mut ret = 1;
    let (wp, hp) = (cfg + 8, cfg + 0xC);
    if m.r32(SCREEN_READY) == 0 {
        // El original salta sobre el ajuste de ventana de este camino (`jmp` incondicional tras `test`).
        let (full, bits) = (m.r32(cfg + 0x20), m.r32(cfg + 4));
        let r = cdecl(m, h, SGL_INIT_SCREEN, &[hwnd, full, bits, wp]);
        m.w32(SCREEN_READY, r);
        if r == 0 {
            return 0;
        }
        let (w, hh) = (m.r32(wp), m.r32(hp));
        cdecl(m, h, SGL_SET_SCREEN_DIMENSIONS, &[0, 0, w, hh, ONE_F32, THOUSAND_F32]);
    } else {
        let r = cdecl(m, h, SGL_SWITCH_SCREEN_MODE, &[wp]);
        if r == 0 {
            return 0;
        }
        if m.r32(cfg + 0x20) != 0 {
            let st = import(m, h, GET_WINDOW_LONG, &[hwnd, (-16i32) as u32]);
            import(m, h, SET_WINDOW_LONG, &[hwnd, (-16i32) as u32, st & 0xFF3B_FFFF]);
            let (w, hh) = (m.r32(wp), m.r32(hp));
            import(m, h, SET_WINDOW_POS, &[hwnd, u32::MAX, 0, 0, w, hh, 8]);
        } else {
            let gsm = m.r32(GET_SYSTEM_METRICS);
            let fx = cdecl(m, h, gsm, &[7]).wrapping_mul(2);
            let fy = cdecl(m, h, gsm, &[8]);
            m.w32(esp + 8, fy);
            let cap = cdecl(m, h, gsm, &[4]);
            let fy = m.r32(esp + 8);
            let extra = cap.wrapping_add(fy.wrapping_mul(2));
            let (w, hh) = (m.r32(wp), m.r32(hp));
            import(m, h, SET_WINDOW_POS, &[hwnd, u32::MAX, 0, 0, w.wrapping_add(fx), hh.wrapping_add(extra), 4]);
        }
        let (w, hh) = (m.r32(wp), m.r32(hp));
        cdecl(m, h, SGL_SET_SCREEN_DIMENSIONS, &[0, 0, w, hh, ONE_F32, THOUSAND_F32]);
        let mut i = 0u32;
        while (i as i32) < m.r32(0x0067_0F6C) as i32 {
            let c = m.r32(0x006D_CD20 + i * 4);
            cdecl(m, h, SGL_SET_ACTIVE_CLIPPER, &[c]);
            cdecl(m, h, SGL_RESET_CLIPPER, &[]);
            i += 1;
        }
        m.w32(0x006D_CD04, 0);
        let c = m.r32(0x006D_CD20);
        cdecl(m, h, SGL_SET_ACTIVE_CLIPPER, &[c]);
        ret = r;
    }
    let w = m.r32(wp);
    m.w32(0x006D_AB4C, w);
    let hh = m.r32(hp);
    m.w32(0x006D_AB44, hh);
    let v = m.r32(wp);
    m.w32(0x006D_CCE0, v);
    let v = m.r32(hp);
    m.w32(0x006D_CCE4, v);
    m.w32(0x006B_ED68, 0);
    m.w32(0x006B_ED64, 0);
    m.w32(0x006B_ED70, hh);
    m.w32(0x006B_ED6C, w);
    // Cursor dentro de la nueva pantalla.
    let mut moved = false;
    let mut x = m.r32(0x006D_AB50);
    if (x as i32) < 0 {
        x = 0;
        moved = true;
        m.w32(0x006D_AB50, x);
    } else if x as i32 >= w as i32 {
        x = w.wrapping_sub(1);
        moved = true;
        m.w32(0x006D_AB50, x);
    }
    let mut y = m.r32(0x006D_AB48);
    if (y as i32) < 0 {
        y = 0;
        moved = true;
        m.w32(0x006D_AB48, 0);
    }
    if y as i32 >= hh as i32 {
        y = hh.wrapping_sub(1);
        m.w32(0x006D_AB48, y);
    } else if !moved {
        return ret;
    }
    let pt = esp - 8;
    m.w32(pt, x);
    m.w32(pt + 4, y);
    let wnd = m.r32(0x006D_CBB4);
    import(m, h, CLIENT_TO_SCREEN, &[wnd, pt]);
    let (x, y) = (m.r32(pt), m.r32(pt + 4));
    import(m, h, SET_CURSOR_POS, &[x, y]);
    ret
}

/// `0x54C280` (this): el anfitrión, lleno, contesta a la conexión pendiente con un rechazo (o la da por leída si ya lo envió).
fn host_reject_connection_when_full(m: &mut dyn Mem, h: &mut dyn Host, this: u32, esp: u32) -> u32 {
    let s0 = esp - 0x430;
    for k in 0..0x34 {
        h.call(m, OPERATION_RECORD_INIT, s0 + 0x20 + k * 0x14, &[]);
    }
    let peer = m.r32(this + 0x44);
    let sock = m.r32(this + 0x3C);
    if m.r32(peer + 0x18) == 2 {
        let r = import(m, h, NET_RECEIVE, &[sock, s0 + 0x10, 0x420, peer, s0 + 0xC]);
        if r != 1 {
            return r;
        }
        let p = m.r32(this + 0x44);
        m.w32(p + 0x18, 3);
        return r;
    }
    m.w16(s0 + 0x1E, 2);
    for (off, v) in [(0x18, 1u32), (0x20, 0xD0), (0x24, 1), (0x28, 1), (0x2C, 0), (0x30, 0), (0x34, 0xC7), (0x38, 1), (0x3C, 0xFFFF),
        (0x40, 0), (0x44, 0)]
    {
        m.w32(s0 + off, v);
    }
    m.w16(s0 + 0x1C, 1);
    import(m, h, NET_SEND, &[sock, s0 + 0x10, 0x38, peer, s0 + 0xC]);
    let (p, s) = (m.r32(this + 0x44), m.r32(this + 0x3C));
    import(m, h, NET_DROP, &[s, p]);
    let p = m.r32(this + 0x44);
    m.w32(p + 0x18, 1);
    let p = m.r32(this + 0x44);
    m.w32(p + 0x10, 0);
    let p = m.r32(this + 0x44);
    m.w32(p + 0x14, 0);
    let p = m.r32(this + 0x44);
    m.w32(p + 0x1C, 0);
    m.w32(p + 0x20, 4);
    m.r32(this + 0x44)
}

pub fn run(f: u32, m: &mut dyn Mem, h: &mut dyn Host, ecx: u32, a: &[u32], esp: u32) -> Option<Ret> {
    if let Some(&(_, iat)) = THUNKS.iter().find(|(t, _)| *t == f) {
        let target = m.r32(iat);
        return Some(Ret::U32(h.call(m, target, ecx, a)));
    }
    let r = match f {
        0x4B3DD0 => {
            let n = m.r32(ecx + 0x10) as i32;
            let tab = m.r32(ecx + 8);
            let found = (0..n.max(0) as u32).find(|&i| m.r32(tab.wrapping_add(i * 4)) == a[0]);
            Ret::U32(match found {
                Some(i) => {
                    let t = m.r32(ecx + 4);
                    m.r32(t.wrapping_add(i * 4))
                }
                None => 0,
            })
        }
        0x4BCCC0 => Ret::U32(set_ddraw_resolution(m, h, a, esp)),
        0x54C280 => Ret::U32(host_reject_connection_when_full(m, h, ecx, esp)),
        0x54C390 => Ret::U32(if m.r32(ecx) & 0xC != 0 {
            let s = m.r32(ecx + 0x3C);
            import(m, h, NET_LOCAL_ADDRESS, &[s])
        } else {
            0
        }),
        0x54C3B0 => {
            let v = m.r32(ecx + 0x38);
            m.w32(ecx, 0);
            if v == 0 {
                m.w32(ecx + 0x2C, u32::MAX);
            }
            Ret::U32(v)
        }
        0x54C3F0 => {
            m.w32(ecx, 0x0067_3D00);
            Ret::U32(ecx)
        }
        0x54C400 => {
            m.w32(ecx, 0x0067_3D00);
            Ret::None
        }
        0x54C4A0 => {
            m.w32(ecx, 0x0067_3D00);
            if a[0] & 1 != 0 {
                free(m, h, ecx);
            }
            Ret::U32(ecx)
        }
        0x553490 => return super::sheet::run(0x553470, m, h, ecx.wrapping_sub(4), a, esp),
        _ => return None,
    };
    Some(r)
}
