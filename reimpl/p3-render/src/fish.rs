//! Paso de los peces (tipo `0x1B`): `0x569560`. El objeto se maneja como memoria plana (`SceneMem`) con los desplazamientos del original.
use crate::ambience::SceneMem;
use crate::clouds::{union_rect, Rect};

pub struct FishCtx {
    /// `escena+0xC31C`, `+0xC320`
    pub size: (u32, u32),
    /// `escena+0x414`
    pub budget: u32,
    /// Entrada de la variante: `+0xC` (velocidad)
    pub speed: u32,
    /// Par `DAT_006E1838 + variante*8`: base y rango del contador
    pub base: u32,
    pub range: u32,
}

pub trait FishEnv {
    fn rand(&mut self) -> u32;
    /// `0x557340(entrada, objeto+8, &objeto+0x38)`: actualiza el contador y devuelve el fotograma.
    fn advance(&mut self, counter: &mut u8) -> u8;
    /// `0x55A2C0(objeto+8, rumbo, 0, 1)`
    fn try_step(&mut self, heading: u8) -> bool;
    /// `0x559370`: rectángulo del pez en su estado actual.
    fn measure(&mut self, o: &SceneMem) -> Rect;
    /// `0x59B5B0`
    fn dirty(&mut self, r: Rect);
}

pub fn step(o: &mut SceneMem, c: &FishCtx, env: &mut dyn FishEnv) -> bool {
    let mut u = c.budget.min(1000);
    let mut delta = (u >> 3) as u8;
    let mut fade = false;
    let r = env.rand();
    if (r as u8 & 7) < (u as u8 & 7) {
        delta = delta.wrapping_add(1);
    }
    if delta != 0 {
        let alpha = o.r8(0x3B);
        let apply = if o.r8(0x3C) == 0 {
            if alpha < delta {
                return false;
            }
            delta = delta.wrapping_neg();
            true
        } else if alpha != 0xFF {
            fade = true;
            if (delta as i32) < 0xFF - alpha as i32 {
                true
            } else {
                o.w8(0x3B, 0xFF);
                false
            }
        } else {
            false
        };
        if apply {
            fade = true;
            o.w8(0x3B, alpha.wrapping_add(delta));
        }
    }
    let (mut nx, mut ny) = (o.r32(0xC), o.r32(0x10));
    let h0 = o.r8(0x14);
    u = c.speed.wrapping_mul(u).wrapping_mul(0x1000) / 0x271;
    let quarter = |i: i32| (i.wrapping_mul(2)).wrapping_div(0x22) as u32;
    match h0 {
        0x00 => ny = ny.wrapping_add(quarter((u as i32).wrapping_mul(-0xb))),
        0x80 => ny = ny.wrapping_add(u.wrapping_mul(0x16) / 0x22),
        0x40 => nx = nx.wrapping_add(u),
        0xC0 => nx = nx.wrapping_sub(u),
        h => {
            let i6 = (u.wrapping_mul(0xb5) as i32) >> 8;
            match h {
                0x20 | 0xE0 => {
                    let i7 = if h == 0x20 { i6 } else { i6.wrapping_neg() };
                    nx = nx.wrapping_add(i7 as u32);
                    ny = ny.wrapping_add(i6.wrapping_mul(-0x16).wrapping_div(0x22) as u32);
                }
                0x60 | 0xA0 => {
                    nx = if h == 0x60 { nx.wrapping_add(i6 as u32) } else { nx.wrapping_sub(i6 as u32) };
                    ny = ny.wrapping_add(quarter(i6.wrapping_mul(0xb)));
                }
                _ => {}
            }
        }
    }
    let old_counter = o.r8(0x38);
    let mut counter = old_counter;
    let frame = env.advance(&mut counter);
    o.w8(0x38, counter);
    let mut heading = h0;
    if old_counter != counter {
        if !env.try_step(heading) {
            let r = env.rand();
            counter = (c.base as u8).wrapping_sub((r % c.range.max(1)) as u8);
            o.w8(0x38, counter);
            let mut b = 0x20u8;
            loop {
                if env.try_step(h0.wrapping_add(b)) || env.try_step(h0.wrapping_sub(b)) {
                    break;
                }
                b = b.wrapping_add(0x20);
                if b >= 0x81 {
                    o.w8(0x3C, 0);
                    break;
                }
            }
            if o.r8(0x3C) != 0 {
                if env.try_step(h0.wrapping_add(b)) {
                    if env.try_step(h0.wrapping_sub(b)) {
                        let r = env.rand() & 1;
                        heading = heading.wrapping_add((r as u8).wrapping_mul(0x40)).wrapping_sub(0x20);
                    } else {
                        heading = heading.wrapping_add(0x20);
                    }
                } else {
                    heading = heading.wrapping_sub(0x20);
                }
            }
        } else if counter as u32 > c.base {
            let r = env.rand();
            counter = (c.base as u8).wrapping_sub((r % c.range.max(1)) as u8);
            o.w8(0x38, counter);
            let left = o.r8(0x3A).wrapping_sub(1);
            o.w8(0x3A, left);
            let mut taken = false;
            if left != 0 {
                if counter & 1 != 0 && env.try_step(h0.wrapping_sub(0x20)) {
                    heading = heading.wrapping_sub(0x20);
                    taken = true;
                } else if env.try_step(h0.wrapping_add(0x20)) {
                    heading = heading.wrapping_add(0x20);
                    taken = true;
                }
            }
            if !taken {
                o.w8(0x3C, 0);
            }
        }
    }
    let (nxs, nys) = (nx >> 16, ny >> 16);
    let (px, py) = (o.r16(0xE) as i16 as i32 as u32, o.r16(0x12) as i16 as i32 as u32);
    if nxs == px && nys == py && frame == o.r8(0x15) && heading == o.r8(0x14) && !fade {
        o.w32(0xC, nx);
        o.w32(0x10, ny);
        return true;
    }
    if nxs <= c.size.0 && nys <= c.size.1 {
        let r1 = env.measure(o);
        o.w32(0xC, nx);
        o.w32(0x10, ny);
        o.w8(0x15, frame);
        o.w8(0x14, heading);
        let r2 = env.measure(o);
        env.dirty(union_rect(r1, r2));
        return true;
    }
    false
}
