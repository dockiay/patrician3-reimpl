//! Partículas del pueblo: niebla (`0x573350`, tipo `0x1C`), nieve (`0x576300`, tipo `0x1D`) y lluvia (`0x575B00`, tipo `0x21`).
//! El objeto se maneja como memoria plana con los desplazamientos del original (los campos de lluvia se solapan: `+0x20` es un `i16` cuyo byte alto es `+0x21`).
use crate::ambience::SceneMem;

pub type Rect = [i32; 4];

fn px(o: &SceneMem) -> i32 {
    o.r16(0xE) as i16 as i32
}
fn py(o: &SceneMem) -> i32 {
    o.r16(0x12) as i16 as i32
}

pub trait ParticleEnv {
    fn budget(&self) -> u32;
    fn time(&self) -> u32;
    /// `0x559370(&rect, entrada, objeto+8)`: rectángulo en la posición actual (`px`, `py`).
    fn measure(&mut self, px: i16, py: i16) -> Rect;
    /// `0x559130(rect|0, entrada, objeto+8)`; devuelve el valor de retorno.
    fn dirty(&mut self, r: Option<Rect>) -> u32;
    fn rand(&mut self) -> u32;
}

/// Entrada de la tabla de niebla (`DAT_006E2DF4`): `+0xC` y el `i16` de `(**(entrada+0x24)+8)+4`.
pub struct FogEnt {
    pub f0c: i32,
    pub short: i16,
}

/// `0x573350`: paso de la niebla; siempre devuelve 1. `c31c` es `escena+0xC31C`.
pub fn fog_step(o: &mut SceneMem, ent: &FogEnt, c31c: i32, env: &mut dyn ParticleEnv) -> u32 {
    let b = env.budget();
    let r = env.measure(px(o) as i16, py(o) as i16);
    let add = ((ent.f0c as u32) << 16).wrapping_mul(b) / 10000;
    let old_hw = o.r16(0xE) as i32;
    let x = o.r32(0xC).wrapping_add(add);
    o.w32(0xC, x);
    if old_hw != px(o) {
        env.dirty(Some(r));
        if px(o) > c31c {
            o.w16(0xC, 0);
            o.w16(0xE, ent.short.wrapping_neg() as u16);
            env.dirty(None);
        }
    }
    1
}

/// Entrada de la tabla de nieve/lluvia: `+0xC`, `+0x10` y el recuento de fotogramas `*(*(entrada+0x20))`.
pub struct SnowEnt {
    pub f0c: i32,
    pub f10: u32,
    pub frames: u32,
    /// Par `DAT_006E2E38[idx]`: `(a, b)`.
    pub pair: (u32, u32),
}

pub trait SnowEnv: ParticleEnv {
    /// Método virtual `+0x1C` del objeto.
    fn virt1c(&mut self);
    /// Método virtual `+0x18`; devuelve `eax`.
    fn virt18(&mut self) -> u32;
    /// Método virtual `+0x48` de la escena.
    fn scene48(&mut self) -> i32;
}

fn step_len(v: i8, f0c: i32, budget: u32) -> i32 {
    let a = (v as i32).abs();
    let t = ((a.wrapping_mul(f0c)) as u32 >> 6).wrapping_mul(budget);
    ((t << 5) as i32).wrapping_div(0x138) << 5
}

/// `0x576300`: paso de la nieve (devuelve `eax`).
pub fn snow_step(o: &mut SceneMem, ent: &SnowEnt, env: &mut dyn SnowEnv) -> u32 {
    let b = env.budget();
    if o.r8(0x25) == 0x80 {
        let t = env.time();
        if o.r32(0x20) < t {
            env.virt1c();
            let eb = ent.pair.1;
            let vy = o.r8(0x26);
            if (vy as u32) > eb {
                o.w8(0x26, vy.wrapping_sub(eb as u8));
                o.w32(0x20, ent.f10.wrapping_add(t));
                return env.virt18();
            }
            return 0;
        }
        return 1;
    }
    let r = env.measure(px(o) as i16, py(o) as i16);
    let (hx0, hy0) = (o.r16(0xE), o.r16(0x12));
    let vx = o.r8(0x25) as i8;
    let vy = o.r8(0x26) as i8;
    let mut dx = step_len(vx, ent.f0c, b);
    let mut dy = step_len(vy, ent.f0c, b);
    if vx < 0 {
        dx = dx.wrapping_neg();
    }
    if vy < 0 {
        dy = dy.wrapping_neg();
    }
    o.w32(0xC, o.r32(0xC).wrapping_add(dx as u32));
    o.w32(0x10, o.r32(0x10).wrapping_add(dy as u32));
    let mut bl = false;
    let t = env.time();
    if o.r32(0x20) < t {
        let f = o.r8(0x15).wrapping_add(1);
        o.w8(0x15, f);
        if f as u32 >= ent.frames {
            o.w8(0x15, 0);
        }
        o.w32(0x20, ent.f10.wrapping_add(t));
        bl = true;
        if ent.pair.0 != 0 {
            let n = env.scene48();
            let m = (n as u32).wrapping_mul(ent.pair.0);
            let d = ((m as u64 * 0x3ab9_5901) >> 32) as u32 >> 7;
            let rr = env.rand();
            if rr % d == 0 {
                o.w8(0x25, 0x80);
                o.w8(0x26, 0xFF);
            }
        }
    }
    let same = hx0 as i32 == px(o) && hy0 as i32 == py(o);
    if !same || bl {
        return env.dirty(Some(r));
    }
    1
}

pub trait RainEnv: ParticleEnv {
    /// `escena.vtable+0x70()`
    fn scene70(&mut self) -> i32;
    /// `escena+0xC650`, `+0xC654`
    fn view_origin(&self) -> (i32, i32);
    /// `0x59B5B0(&rect)`: devuelve `al`.
    fn dirty_rect(&mut self, r: Rect) -> bool;
}

fn normalized(l: i32, t: i32, r: i32, b: i32) -> Rect {
    let (l, r) = if l > r { (r, l) } else { (l, r) };
    let (t, b) = if t > b { (b, t) } else { (t, b) };
    [l, t, r + 1, b + 1]
}

fn rain_len(v: i8, budget: u32) -> i32 {
    let a = (v as i32).abs().wrapping_mul(12000);
    let a = (a + ((a >> 31) & 0x3F)) >> 6;
    let t = a.wrapping_mul(budget as i32).wrapping_shl(5);
    t.wrapping_div(0x138) << 5
}

/// `0x575B00`: paso de la lluvia (devuelve `al`).
pub fn rain_step(o: &mut SceneMem, env: &mut dyn RainEnv) -> u32 {
    let b = env.budget();
    let (hx, hy) = (px(o), py(o));
    let vx = o.r8(0x21) as i8;
    let vy = o.r8(0x23) as i8;
    let mut r1 = normalized(hx, hy, hx + vx as i32, hy + vy as i32);
    let (bp, cy) = (o.r16(0xE), o.r16(0x12));
    let mut dx = rain_len(vx, b);
    let mut dy = rain_len(vy, b);
    if vx < 0 {
        dx = dx.wrapping_neg();
    }
    if vy < 0 {
        dy = dy.wrapping_neg();
    }
    o.w32(0xC, o.r32(0xC).wrapping_add(dx as u32));
    o.w32(0x10, o.r32(0x10).wrapping_add(dy as u32));
    if bp as i32 == px(o) && cy as i32 == py(o) {
        return 1;
    }
    o.w16(0x20, o.r16(0x20).wrapping_add((-(vx as i32 * 3) * 2) as u16));
    o.w16(0x22, o.r16(0x22).wrapping_add((-(vy as i32 * 3) * 2) as u16));
    let (nx, ny) = (px(o), py(o));
    let r2 = normalized(nx, ny, nx + o.r8(0x21) as i8 as i32, ny + o.r8(0x23) as i8 as i32);
    // UnionRect
    let empty = |r: &Rect| r[2] <= r[0] || r[3] <= r[1];
    r1 = if empty(&r1) && empty(&r2) {
        [0, 0, 0, 0]
    } else if empty(&r1) {
        r2
    } else if empty(&r2) {
        r1
    } else {
        [r1[0].min(r2[0]), r1[1].min(r2[1]), r1[2].max(r2[2]), r1[3].max(r2[3])]
    };
    let (ox, oy) = env.view_origin();
    let v70 = env.scene70();
    let dxo = ox.wrapping_neg();
    let dyo = v70.wrapping_sub(oy);
    let r1 = [r1[0].wrapping_add(dxo), r1[1].wrapping_add(dyo), r1[2].wrapping_add(dxo), r1[3].wrapping_add(dyo)];
    if env.dirty_rect(r1) && (o.r8(0x23) as i8) > 3 {
        1
    } else {
        0
    }
}
