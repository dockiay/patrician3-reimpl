//! `TownObjectType29` (0x580B00–0x580DA0): el anillo que pulsa alrededor de una ciudad en el mapa y la bandera que se le engancha.
//! Disposición: `+0x4` subobjeto (con vtable), `+0x20` bandera, `+0x24` instante del siguiente paso, `+0x28` brillo del anillo, `+0x29`
//! sentido (`0` sube, `1` baja, `0xFF` apagado), `+0x2C/+0x30` posición, `+0x34` clase.

use super::{cdecl, vcall, Host, Mem, Ret};

const TICK: u32 = 0x006D_CCF8;
const SCENE: u32 = 0x006E_51AC;
const BITMAP: u32 = 0x006E_1924;
const BITMAP_TARGET: u32 = 0x006E_192C;
const INVALIDATE: u32 = 0x004B_9650;
const DIRTY: u32 = 0x006D_A5F0;
const STEP: u32 = 0x0067_58DB;
const PEAK: u32 = 0x0067_58DC;
const DELAY: u32 = 0x0067_58DD;
const SET_COLOR: u32 = 0x004B_B870;
const SET_TARGET: u32 = 0x004B_B9C0;
const BLIT: u32 = 0x004B_B330;
const CLASS_TABLE: u32 = 0x006E_2F98;
const CLASS_INDEX: u32 = 0x006E_2FB2;
const FLAG_HEAD: u32 = 0x006E_31E0;
const FLAG_OFFSETS: u32 = 0x006E_2F30;
const FLAG_BYTE: u32 = 0x0067_7929;
const FLAG_FACTORY: u32 = 0x0059_0610;

fn rect_of(m: &mut dyn Mem, this: u32) -> (u32, u32, u32, u32) {
    let scene = m.r32(SCENE);
    let left = m.r32(this + 0x2C).wrapping_sub(m.r32(scene + 0xC650));
    let top = m.r32(this + 0x30).wrapping_sub(m.r32(scene + 0xC654));
    let bm = m.r32(BITMAP);
    (left, top, u32::from(m.r16(bm + 4)).wrapping_add(left), u32::from(m.r16(bm + 6)).wrapping_add(top))
}

fn invalidate(m: &mut dyn Mem, h: &mut dyn Host, at: u32, r: (u32, u32, u32, u32)) {
    m.w32(at, r.0);
    m.w32(at + 4, r.1);
    m.w32(at + 8, r.2);
    m.w32(at + 0xC, r.3);
    h.call(m, INVALIDATE, DIRTY, &[at]);
}

/// `0x580B00`: avanza el brillo del anillo.
fn update_ring_pulse(m: &mut dyn Mem, h: &mut dyn Host, this: u32, esp: u32) {
    let dir = m.r8(this + 0x29);
    if dir == 0xFF || m.r32(TICK) < m.r32(this + 0x24) {
        return;
    }
    if dir == 0 {
        let v = m.r8(this + 0x28).wrapping_add(m.r8(STEP));
        m.w8(this + 0x28, v);
        if v >= m.r8(PEAK) {
            m.w8(this + 0x29, 1);
        }
    } else {
        let v = m.r8(this + 0x28).wrapping_sub(m.r8(STEP));
        m.w8(this + 0x28, v);
        if v <= m.r8(STEP) {
            m.w8(this + 0x29, 0xFF);
        }
    }
    let r = rect_of(m, this);
    invalidate(m, h, esp.wrapping_sub(0x10), r);
    let t = u32::from(m.r8(DELAY)).wrapping_add(m.r32(TICK));
    m.w32(this + 0x24, t);
}

/// `0x580BD0`: arranca el pulso en `(x, y)` si el subobjeto lo permite.
fn start_ring_pulse(m: &mut dyn Mem, h: &mut dyn Host, this: u32, a: &[u32], esp: u32) {
    if vcall(m, h, this + 4, 0x64, &[]) != 0 {
        return;
    }
    m.w8(this + 0x29, 0);
    m.w8(this + 0x28, 0xFF);
    let t = u32::from(m.r8(DELAY)).wrapping_add(m.r32(TICK));
    m.w32(this + 0x24, t);
    let bm = m.r32(BITMAP);
    let w = u32::from(m.r16(bm + 4));
    m.w32(this + 0x2C, a[0].wrapping_sub(w >> 1));
    let off = u32::from(m.r8(this + 0x34)).wrapping_mul(60);
    let dl = u32::from(m.r8(CLASS_INDEX.wrapping_add(off)));
    let table = m.r32(CLASS_TABLE.wrapping_add(off));
    let cx = u32::from(m.r16(table.wrapping_add(dl.wrapping_mul(8)).wrapping_sub(2)));
    let bm = m.r32(BITMAP);
    let hh = u32::from(m.r16(bm + 6));
    m.w32(this + 0x30, a[1].wrapping_sub(hh).wrapping_sub(cx));
    let r = rect_of(m, this);
    invalidate(m, h, esp.wrapping_sub(0x10), r);
}

/// `0x580D20`: dibuja el anillo.
fn draw_ring_pulse(m: &mut dyn Mem, h: &mut dyn Host, this: u32) {
    if m.r8(this + 0x29) == 0xFF {
        return;
    }
    let scene = m.r32(SCENE);
    let (ox, oy) = (m.r32(scene + 0xC650), m.r32(scene + 0xC654));
    let target = m.r32(BITMAP_TARGET);
    cdecl(m, h, SET_TARGET, &[target]);
    let color = (u32::from(m.r8(this + 0x28)) << 24).wrapping_add(0x00FF_FFFF);
    cdecl(m, h, SET_COLOR, &[color]);
    let bm = m.r32(BITMAP);
    let (w, hh) = (u32::from(m.r16(bm + 4)), u32::from(m.r16(bm + 6)));
    let y = m.r32(this + 0x30).wrapping_sub(oy);
    let x = m.r32(this + 0x2C).wrapping_sub(ox);
    let (b0, b1) = (u32::from(m.r16(bm)), u32::from(m.r16(bm + 2)));
    cdecl(m, h, BLIT, &[b0, b1, x, y, w, hh]);
}

/// `0x580D10`: avisa a la bandera (slot 2 de su vtable).
fn draw_flag(m: &mut dyn Mem, h: &mut dyn Host, this: u32) {
    let flag = m.r32(this + 0x20);
    if flag != 0 {
        vcall(m, h, flag, 8, &[0]);
    }
}

/// `0x580DA0`: engancha una bandera nueva al final de la lista y la crea en pantalla.
fn attach_flag(m: &mut dyn Mem, h: &mut dyn Host, a: &[u32]) -> u32 {
    let (obj, px, py) = (a[0], a[1], a[2]);
    let head = m.r32(FLAG_HEAD);
    if head != 0 {
        let mut p = head;
        loop {
            let next = m.r32(p + 0x1C);
            if next == 0 {
                break;
            }
            p = next;
        }
        m.w32(p + 0x1C, obj);
    } else {
        m.w32(FLAG_HEAD, obj);
    }
    let sub = obj.wrapping_add(4);
    let scene = m.r32(SCENE);
    let sx_ = m.r32(scene + 0x4D0);
    let e = vcall(m, h, sub, 0x48, &[]);
    let bx = px.wrapping_sub((e.wrapping_sub(2) as i32).wrapping_mul(sx_ as i32) as u32);
    let scene = m.r32(SCENE);
    let sy_ = m.r32(scene + 0x4D4);
    let e = vcall(m, h, sub, 0x48, &[]);
    let mut di = py.wrapping_sub((e.wrapping_sub(2) as i32).wrapping_mul(sy_ as i32) as u32);
    let r = vcall(m, h, sub, 0x74, &[]);
    let t = m.r32(r + 0x10);
    di = di.wrapping_sub(u32::from(m.r16(t + 6)));
    let r = vcall(m, h, sub, 0x74, &[]);
    let t = m.r32(r + 0x10);
    di = di.wrapping_sub(u32::from(m.r16(t + 2)));
    let idx8 = u32::from(m.r8(obj + 0x34)) << 3;
    let (sx0, sy0) = (m.r32(FLAG_OFFSETS.wrapping_add(idx8)), m.r32(FLAG_OFFSETS.wrapping_add(idx8).wrapping_add(4)));
    let eax = (idx8 & 0xFFFF_FF00) | u32::from(m.r8(FLAG_BYTE));
    di = di.wrapping_add(sy0);
    let bx = bx.wrapping_add(sx0);
    let scene = m.r32(SCENE);
    let res = h.call(m, FLAG_FACTORY, scene, &[bx << 16, di << 16, 0x1E, obj, eax, 0, 0]);
    m.w32(obj + 0x20, res);
    res
}

pub fn run(f: u32, m: &mut dyn Mem, h: &mut dyn Host, ecx: u32, a: &[u32], esp: u32) -> Option<Ret> {
    Some(match f {
        0x0058_0B00 => {
            update_ring_pulse(m, h, ecx, esp);
            Ret::None
        }
        0x0058_0BD0 => {
            start_ring_pulse(m, h, ecx, a, esp);
            Ret::None
        }
        0x0058_0D10 => {
            draw_flag(m, h, ecx);
            Ret::None
        }
        0x0058_0D20 => {
            draw_ring_pulse(m, h, ecx);
            Ret::None
        }
        0x0058_0DA0 => Ret::U32(attach_flag(m, h, a)),
        _ => return None,
    })
}
