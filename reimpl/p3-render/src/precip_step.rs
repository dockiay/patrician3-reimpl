//! Avance de la nieve (`0x576300`, tipo 29), de la lluvia (`0x575B00`, tipo 33) y de la grieta (`0x57FEB0`, tipo 34), el recorrido de grietas (`0x57FFB0`)
//! y el recorte de ventana (`0x59B5B0`). Traducción 1:1 sobre memoria plana (`Mem`, direcciones absolutas). Paridad en `tests/x86_precip_step.rs`.
//! Las funciones de repintado (`0x559370`, `0x559130`, `0x5593C0`, que calculan rectángulos sucios) y los tres virtuales de ventana de la escena
//! (`+0x44`, `+0x48`, `+0x70`) son del entorno: su valor de retorno solo se propaga.
use crate::scene_math::Mem;

const SCENE_PTR: u32 = 0x006E_51AC;
const CLOCK: u32 = 0x006D_CCF8;

pub trait StepEnv {
    /// Virtual `+0x44` de la escena (ancho de ventana).
    fn win_w(&mut self) -> u32;
    /// Virtual `+0x48` (alto de ventana).
    fn win_h(&mut self) -> u32;
    /// Virtual `+0x70` (desplazamiento vertical de la ventana).
    fn win_y(&mut self) -> u32;
    /// `0x559370`, `0x559130` o `0x5593C0` (rectángulo sucio); devuelve el `eax` de la función.
    fn repaint(&mut self, which: u32) -> u32;
    /// `0x4B9650(&rect)`: marca la zona visible.
    fn mark(&mut self, rect: [i32; 4]);
}

fn abs32(v: i32) -> i32 {
    if v < 0 {
        v.wrapping_neg()
    } else {
        v
    }
}

/// `0x576300(objeto)`: devuelve `AL` (0 = el objeto se retira).
pub fn snow_step(m: &mut dyn Mem, obj: u32, env: &mut dyn StepEnv, rand: &mut dyn FnMut() -> u32) -> u8 {
    let idx = u32::from(m.r8(obj + 0x24));
    let set = if idx < m.r32(0x006E_2E30) {
        let a = m.r32(0x006E_2E34);
        m.r32(a.wrapping_add(idx * 4))
    } else {
        0
    };
    let scene = m.r32(SCENE_PTR);
    let budget = m.r32(scene + 0x414);
    let clock = m.r32(CLOCK);
    let tbl = m.r32(0x006E_2E38);
    if m.r8(obj + 0x25) as i8 == -0x80 {
        if m.r32(obj + 0x20) < clock {
            env.repaint(0x0055_9130);
            let thr = m.r32(tbl.wrapping_add(4).wrapping_add(idx * 8));
            let s26 = m.r8(obj + 0x26);
            if thr < u32::from(s26) {
                let sub = m.r8(tbl.wrapping_add(4).wrapping_add(idx * 8));
                m.w8(obj + 0x26, s26.wrapping_sub(sub));
                let interval = m.r32(set.wrapping_add(0x10));
                m.w32(obj + 0x20, interval.wrapping_add(clock));
                return env.repaint(0x0055_93C0) as u8;
            }
            return 0;
        }
        return 1;
    }
    env.repaint(0x0055_9370);
    let (ux, uy) = (m.r16(obj + 0xE), m.r16(obj + 0x12));
    let q = m.r32(set.wrapping_add(0xC)) as i32;
    let s25 = m.r8(obj + 0x25) as i8;
    let s26 = m.r8(obj + 0x26) as i8;
    let speed = |a: i32| -> i32 { ((((abs32(a).wrapping_mul(q)) as u32) >> 6).wrapping_mul(budget).wrapping_mul(0x20) as i32).wrapping_div(0x138) };
    let (a5, a6) = (speed(i32::from(s25)), speed(i32::from(s26)));
    let mut dx = a5.wrapping_mul(0x20);
    let mut dy = a6.wrapping_mul(0x20);
    if s25 < 0 {
        dx = a5.wrapping_mul(-0x20);
    }
    if s26 < 0 {
        dy = a6.wrapping_mul(-0x20);
    }
    let (px, py) = (m.r32(obj + 0xC), m.r32(obj + 0x10));
    m.w32(obj + 0xC, px.wrapping_add(dx as u32));
    m.w32(obj + 0x10, py.wrapping_add(dy as u32));
    let mut changed = false;
    if m.r32(obj + 0x20) < clock {
        let f = m.r8(obj + 0x15).wrapping_add(1);
        m.w8(obj + 0x15, f);
        let frames = m.r32(set.wrapping_add(0x20));
        if m.r32(frames) <= u32::from(f) {
            m.w8(obj + 0x15, 0);
        }
        let interval = m.r32(set.wrapping_add(0x10));
        m.w32(obj + 0x20, interval.wrapping_add(clock));
        changed = true;
        let chance = m.r32(tbl.wrapping_add(idx * 8));
        if chance != 0 {
            let h = env.win_h();
            let r = rand();
            let d = u64::from(h.wrapping_mul(chance)) / 0x22E;
            if u64::from(r) % d == 0 {
                m.w8(obj + 0x25, 0x80);
                m.w8(obj + 0x26, 0xFF);
            }
        }
    }
    // x86 (0x5764D1): el valor guardado se zero-extiende (`and eax, 0xFFFF`) y el actual se sign-extiende (`movsx`),
    // así que con una coordenada de 16 bits ≥ 0x8000 SIEMPRE cuenta como movido, aunque no haya cambiado.
    let moved = u32::from(ux) != (m.r16(obj + 0xE) as i16 as i32 as u32) || u32::from(uy) != (m.r16(obj + 0x12) as i16 as i32 as u32);
    if changed || moved {
        return env.repaint(0x0055_9130) as u8;
    }
    1
}

fn normalized(l: i32, t: i32, r: i32, b: i32) -> [i32; 4] {
    // `SetRect` + `CRect::NormalizeRect` + (`right + 1`, `bottom + 1`)
    let (l2, r2) = if l > r { (r, l) } else { (l, r) };
    let (t2, b2) = if t > b { (b, t) } else { (t, b) };
    [l2, t2, r2.wrapping_add(1), b2.wrapping_add(1)]
}

fn rect_empty(r: [i32; 4]) -> bool {
    r[0] >= r[2] || r[1] >= r[3]
}

/// `UnionRect` de Windows: las vacías no cuentan.
fn union(a: [i32; 4], b: [i32; 4]) -> [i32; 4] {
    match (rect_empty(a), rect_empty(b)) {
        (true, true) => [0; 4],
        (true, false) => b,
        (false, true) => a,
        _ => [a[0].min(b[0]), a[1].min(b[1]), a[2].max(b[2]), a[3].max(b[3])],
    }
}

/// `0x575B00(objeto)`. `clip` es `0x59B5B0(&rect)` (por ejemplo [`clip_window`]). Devuelve `AL`.
pub fn rain_step(m: &mut dyn Mem, obj: u32, env: &mut dyn StepEnv, clip: &mut dyn FnMut(&mut dyn Mem, [i32; 4]) -> bool) -> u8 {
    let scene = m.r32(SCENE_PTR);
    let budget = m.r32(scene + 0x414);
    let (x0, y0) = (i32::from(m.r16(obj + 0xE) as i16), i32::from(m.r16(obj + 0x12) as i16));
    let (dx, dy) = (i32::from(m.r8(obj + 0x21) as i8), i32::from(m.r8(obj + 0x23) as i8));
    let first = normalized(x0, y0, x0.wrapping_add(dx), y0.wrapping_add(dy));
    let (u1, u2) = (m.r16(obj + 0xE), m.r16(obj + 0x12));
    let speed = |a: i32| -> i32 {
        let p = abs32(a).wrapping_mul(12000);
        let adj = p.wrapping_add((p >> 31) & 0x3F);
        ((((adj >> 6) as u32).wrapping_mul(budget).wrapping_mul(0x20)) as i32).wrapping_div(0x138)
    };
    let (a5, a6) = (speed(dx), speed(dy));
    let mut mx = a5.wrapping_mul(0x20);
    let mut my = a6.wrapping_mul(0x20);
    if dx < 0 {
        mx = a5.wrapping_mul(-0x20);
    }
    if dy < 0 {
        my = a6.wrapping_mul(-0x20);
    }
    let (px, py) = (m.r32(obj + 0xC), m.r32(obj + 0x10));
    m.w32(obj + 0xC, px.wrapping_add(mx as u32));
    m.w32(obj + 0x10, py.wrapping_add(my as u32));
    let x_left = i32::from(m.r16(obj + 0xE) as i16);
    // El original compara el `u16` anterior con el valor nuevo extendido con signo (`uint`): una `x` negativa nunca coincide.
    if u32::from(u1) == x_left as u32 && u32::from(u2) == i32::from(m.r16(obj + 0x12) as i16) as u32 {
        return 1;
    }
    let s20 = m.r16(obj + 0x20) as i16;
    m.w16(obj + 0x20, s20.wrapping_add((dx.wrapping_mul(-6)) as i16) as u16);
    let s22 = m.r16(obj + 0x22) as i16;
    m.w16(obj + 0x22, s22.wrapping_add((dy.wrapping_mul(-6)) as i16) as u16);
    let y_now = i32::from(m.r16(obj + 0x12) as i16);
    // `+0x20` (u16) solapa `+0x21` (dx) y `+0x22` solapa `+0x23` (dy): el original los vuelve a leer ya modificados.
    let (dx, dy) = (i32::from(m.r8(obj + 0x21) as i8), i32::from(m.r8(obj + 0x23) as i8));
    let second = normalized(x_left, y_now, dx.wrapping_add(x_left), dy.wrapping_add(y_now));
    let mut u = union(first, second);
    let (c650, c654) = (m.r32(scene + 0xC650) as i32, m.r32(scene + 0xC654) as i32);
    let wy = env.win_y() as i32;
    let (ox, oy) = (c650.wrapping_neg(), wy.wrapping_sub(c654));
    u = [u[0].wrapping_add(ox), u[1].wrapping_add(oy), u[2].wrapping_add(ox), u[3].wrapping_add(oy)];
    let hit = clip(m, u);
    u8::from(hit && 3 < dy)
}

/// `0x59B5B0(escena, &rect)`: recorta con la ventana de la escena; si queda algo llama a `0x4B9650`.
pub fn clip_window(m: &mut dyn Mem, scene: u32, rect: [i32; 4], env: &mut dyn StepEnv) -> bool {
    let (l, t) = (m.r32(scene + 0x14) as i32, m.r32(scene + 0x18) as i32);
    let view = [l, t, l.wrapping_add(m.r32(scene + 0x2C) as i32), t.wrapping_add(m.r32(scene + 0x30) as i32)];
    let inter = [rect[0].max(view[0]), rect[1].max(view[1]), rect[2].min(view[2]), rect[3].min(view[3])];
    if rect_empty(rect) || rect_empty(view) || rect_empty(inter) {
        return false;
    }
    env.mark(inter);
    true
}

/// `0x57FEB0(grieta)`: devuelve falso cuando ya está apagada.
pub fn crack_step(m: &mut dyn Mem, obj: u32, env: &mut dyn StepEnv) -> bool {
    if m.r8(obj + 0x28) != 0 {
        return false;
    }
    let clock = m.r32(CLOCK);
    if clock < m.r32(obj + 0x2C) {
        return true;
    }
    let f = m.r8(obj + 0x15).wrapping_add(1);
    m.w8(obj + 0x15, f);
    let set = m.r32(0x006E_2EF0);
    if u32::from(f) >= m.r32(set) {
        m.w8(obj + 0x15, 0);
    }
    let interval = m.r32(0x006E_2EE0);
    m.w32(obj + 0x2C, interval.wrapping_add(clock));
    env.repaint(0x0055_9130);
    m.w8(obj + 0x28, 1);
    true
}

/// `0x57FFB0`: recorre el vector global `[0x6E2F14, 0x6E2F18)`; las grietas que devuelven falso se retiran con `remove` (que debe quitarlas del vector).
pub fn sweep_cracks(m: &mut dyn Mem, step: &mut dyn FnMut(&mut dyn Mem, u32) -> bool, remove: &mut dyn FnMut(&mut dyn Mem, u32)) {
    let mut at = m.r32(0x006E_2F14);
    while at != m.r32(0x006E_2F18) {
        let obj = m.r32(at);
        if step(m, obj) {
            at = at.wrapping_add(4);
        } else {
            remove(m, obj);
        }
    }
}

/// `0x57FF20(x, y, p)`: busca en el vector global de grietas la primera en `(x, y)` (`+0x20`, `+0x24`).
/// Si la encuentra: cuando su byte `+0x29` difiere de `p` llama al virtual `+0x1C` (`on_change`, que repinta) y lo guarda;
/// pone `+0x28` a cero (la mantiene viva) y devuelve verdadero. Si no, falso.
pub fn renew_crack(m: &mut dyn Mem, x: u32, y: u32, p: u8, on_change: &mut dyn FnMut(&mut dyn Mem, u32)) -> bool {
    let (mut at, end) = (m.r32(0x006E_2F14), m.r32(0x006E_2F18));
    while at != end {
        let o = m.r32(at);
        if m.r32(o + 0x20) == x && m.r32(o + 0x24) == y {
            if m.r8(o + 0x29) != p {
                on_change(m, o);
                m.w8(o + 0x29, p);
            }
            m.w8(o + 0x28, 0);
            return true;
        }
        at = at.wrapping_add(4);
    }
    false
}
