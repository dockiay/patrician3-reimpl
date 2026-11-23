//! Paso de las nubes del pueblo (tipo 20): `0x580520`.
//!
//! Observado y probado en emulador (`oracle/x86_cloud_step.py`, 4.000 casos con el x86 real y `0x639F78` real). El rectángulo de dibujo (`0x559260`),
//! `UnionRect` y `0x59B5B0` se sustituyen por [`CloudEnv`]; `UnionRect` se reproduce con la semántica de Windows.

#[derive(Clone, Debug, Default)]
pub struct Cloud {
    pub x: u32,
    pub y: u32,
    /// `+0xE`, `+0x12`: parte alta de `x` e `y`.
    pub px: i16,
    pub py: i16,
    /// `+0x24`, `+0x28`: velocidad suavizada.
    pub vx: i32,
    pub vy: i32,
    /// `+0x2C`, `+0x30`: velocidad objetivo.
    pub tx: i32,
    pub ty: i32,
    /// `+0x34`, `+0x36`: ancho y alto del mundo para el envuelto.
    pub wrap_x: u16,
    pub wrap_y: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Rect {
    fn is_empty(&self) -> bool {
        self.left >= self.right || self.top >= self.bottom
    }
    fn shifted(&self, dx: i32, dy: i32) -> Rect {
        Rect { left: self.left + dx, top: self.top + dy, right: self.right + dx, bottom: self.bottom + dy }
    }
}

/// `UnionRect` de Windows.
pub fn union_rect(a: Rect, b: Rect) -> Rect {
    match (a.is_empty(), b.is_empty()) {
        (true, true) => Rect { left: 0, top: 0, right: 0, bottom: 0 },
        (true, false) => b,
        (false, true) => a,
        _ => Rect { left: a.left.min(b.left), top: a.top.min(b.top), right: a.right.max(b.right), bottom: a.bottom.max(b.bottom) },
    }
}

pub trait CloudEnv {
    fn rand(&mut self) -> u32;
    /// `0x559260`: rectángulo de la nube en su posición actual.
    fn measure(&mut self, px: i16, py: i16) -> Rect;
    /// `0x59B5B0`.
    fn dirty(&mut self, r: Rect);
}

/// `0x580520`; siempre devuelve `true`. `shadow` es `ajustes+0x16C >= 2` (capa de sombra).
pub fn step(c: &mut Cloud, budget: u32, shadow: bool, env: &mut dyn CloudEnv) -> bool {
    let u6 = budget.min(10000) as i32;
    let near = (c.vx.wrapping_sub(c.tx)).wrapping_abs() <= 0x20 && (c.vx.wrapping_sub(c.tx)).wrapping_abs() <= 0x16;
    if near {
        let w = 10000 - u6;
        c.vx = c.tx.wrapping_mul(u6).wrapping_add(c.vx.wrapping_mul(w));
        let r = env.rand() as i32;
        c.vx = (r % (u6 << 6)).wrapping_sub(u6.wrapping_mul(0x20)).wrapping_add(c.vx).wrapping_div(10000);
        c.vy = c.ty.wrapping_mul(u6).wrapping_add(c.vy.wrapping_mul(w));
        let r = env.rand() as i32;
        c.vy = (r % u6.wrapping_mul(0x2c)).wrapping_sub(u6.wrapping_mul(0x16)).wrapping_add(c.vy).wrapping_div(10000);
    } else {
        let r = env.rand() & 0x3f;
        c.vx = (r as i32 - 0x20).wrapping_add(c.tx);
        let r = env.rand() as i32;
        c.vy = (r % 0x2c - 0x16).wrapping_add(c.ty);
    }
    let u5 = ((u64::from(budget) << 16) / 10000) as u32;
    let nx = (c.vx as u32).wrapping_mul(u5).wrapping_add(c.x);
    let ny = (c.vy as u32).wrapping_mul(u5).wrapping_add(c.y);
    if nx >> 16 == i32::from(c.px) as u32 && ny >> 16 == i32::from(c.py) as u32 {
        c.x = nx;
        c.y = ny;
        return true;
    }
    let mut plain = true;
    let r1 = env.measure(c.px, c.py);
    let s1 = r1.shifted(0x1e, 0x96);
    c.x = nx;
    c.y = ny;
    c.px = (nx >> 16) as i16;
    c.py = (ny >> 16) as i16;
    let wx = i32::from(c.wrap_x);
    if wx < i32::from(c.px) {
        c.px = (i32::from(c.px) - wx) as i16;
        plain = false;
    } else if c.px < 0 {
        c.px = (wx + i32::from(c.px)) as i16;
        plain = false;
    }
    let wy = i32::from(c.wrap_y);
    if wy < i32::from(c.py) {
        c.py = (i32::from(c.py) - wy) as i16;
        plain = false;
    } else if c.py < 0 {
        c.py = (wy + i32::from(c.py)) as i16;
        plain = false;
    }
    // `+0xE` y `+0x12` son la parte alta de `x` e `y`: el envuelto las modifica.
    c.x = (c.x & 0xFFFF) | (u32::from(c.px as u16) << 16);
    c.y = (c.y & 0xFFFF) | (u32::from(c.py as u16) << 16);
    let r2 = env.measure(c.px, c.py);
    let s2 = r2.shifted(0x1e, 0x96);
    if plain {
        env.dirty(union_rect(r1, r2));
        if shadow {
            env.dirty(union_rect(s1, s2));
        }
    } else {
        env.dirty(r1);
        env.dirty(r2);
        if shadow {
            env.dirty(s1);
            env.dirty(s2);
        }
    }
    true
}
