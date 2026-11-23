//! Paso del objeto «efecto» (tipo `0x12`, polvo, humo, fuego y similares): `0x57A8C0`.
//! El objeto se maneja como memoria plana (`SceneMem`); la tabla de conjuntos (`DAT_006E2E6C/70/74`), el puerto y los demás externos pasan por [`EffectStepEnv`].
use crate::ambience::SceneMem;
use crate::clouds::{union_rect, Rect};

pub struct EffectCtx {
    /// `escena+0x414`
    pub budget: u32,
    /// `DAT_006DCCF8`
    pub time: u32,
    /// Byte de tipo `DAT_006E2E74[objeto+0x2C]`
    pub kind: u8,
    /// `registro_ciudad(plan)+0x2C8 & 0x4000000 == 0`
    pub no_flag: bool,
    /// `escena+0xC594 == 0`
    pub c594_zero: bool,
    /// `DAT_006DF270 != 0` y el plan es una ciudad existente.
    pub harbor_ok: bool,
    /// Valores `*(registro+100)` de las tres listas (`+0x410`, `+0x80C`, `+0xC08`) y de las cuentas que las recorren.
    pub lists: [Vec<u32>; 3],
    /// Identificador del objeto que se busca en las listas.
    pub id: u32,
}

pub trait EffectStepEnv {
    /// `0x557340(conjunto, objeto+8, &salida)`: devuelve el fotograma.
    fn advance(&mut self) -> u8;
    /// `0x613070(&a, &b)`: devuelve `b`.
    fn f613070(&mut self) -> u32;
    fn f561f60(&mut self);
    /// `0x57AF60(conjunto, bandera)`
    fn toggle(&mut self, on: bool);
    /// `0x5595F0(conjunto, objeto+8, &r)`
    fn occupy_remove(&mut self, o: &SceneMem) -> Rect;
    /// `0x559790(conjunto, objeto+8, &r)`
    fn occupy_add(&mut self, r: Rect);
    /// `0x559370(&r, conjunto, objeto+8)`
    fn measure(&mut self, o: &SceneMem) -> Rect;
    /// `0x59B5B0(&r)`
    fn dirty(&mut self, r: Rect);
}

fn px(o: &SceneMem) -> i32 {
    o.r16(0xE) as i16 as i32
}
fn py(o: &SceneMem) -> i32 {
    o.r16(0x12) as i16 as i32
}

/// Devuelve `false` si el objeto debe retirarse.
pub fn step(o: &mut SceneMem, c: &EffectCtx, env: &mut dyn EffectStepEnv) -> bool {
    let kind = c.kind;
    let budget = c.budget;
    if budget > 2000 && kind < 0x80 {
        return false;
    }
    if o.r32(0x34) == 0 {
        if kind == 199 && c.no_flag {
            o.w32(0x3C, 0);
            o.w32(0x34, 0xFFFF_FF80);
        }
    } else {
        let w = o.r32(0x3C);
        if budget < w {
            o.w32(0x3C, w - budget);
        } else {
            o.w32(0x3C, 0);
            let v = (budget.wrapping_sub(w) as i32).wrapping_mul(o.r32(0x34) as i32).wrapping_add(o.r16(0x38) as i16 as i32);
            let q = v / 16000;
            let inc = q.wrapping_mul(0x10);
            o.w16(0x38, (v as i16).wrapping_add((q as i16).wrapping_mul(-16000)) as u16);
            if inc != 0 {
                let s = o.r8(0x30) as i32 + inc;
                if s < 1 {
                    return false;
                }
                if s < 0xFF {
                    o.w8(0x30, o.r8(0x30).wrapping_add(inc as u8));
                } else {
                    o.w8(0x30, 0xFF);
                    o.w32(0x34, 0);
                }
            }
        }
    }
    let frame = env.advance();
    if kind == 0x50 && frame != o.r8(0x15) {
        let left = (o.r32(0x24).wrapping_sub(c.time)) as i32;
        let b = env.f613070() & 0xFF;
        o.w32(0x24, ((left.wrapping_mul(9)) as u32) / (b + 8) + c.time);
    }
    let x = (o.r32(0x40).wrapping_mul(budget)).wrapping_add(o.r32(0xC));
    let y = (o.r32(0x44).wrapping_mul(budget)).wrapping_add(o.r32(0x10));
    let old = o.r8(0x15);
    if frame == old && o.r8(0x30) == o.r8(0x31) && (x >> 16) as i16 as i32 == px(o) && (y >> 16) as i16 as i32 == py(o) {
        o.w32(0x10, y);
        o.w32(0xC, x);
        o.w8(0x31, o.r8(0x30));
        return true;
    }
    if frame < old && (kind as i8) >= 0 && o.r8(0x28) == 2 {
        return false;
    }
    let mut ty = kind;
    if frame == 0x14 && old != 0x14 && (0x28..=0x2A).contains(&kind) && c.harbor_ok {
        let li = (kind - 0x28) as usize;
        if c.lists[li].iter().any(|&v| v == c.id) {
            env.f561f60();
        }
        ty = kind;
    }
    if c.c594_zero && ty > 0x3F && ty < 0x44 {
        o.w8(0x15, frame);
        o.w32(0xC, x);
        o.w32(0x10, y);
        return true;
    }
    env.toggle(true);
    let r1 = if ty & 0x40 == 0 { env.occupy_remove(o) } else { env.measure(o) };
    env.toggle(false);
    o.w32(0xC, x);
    o.w8(0x15, frame);
    o.w32(0x10, y);
    env.toggle(true);
    if ty & 0x40 == 0 {
        env.occupy_add(r1);
    } else {
        let r2 = env.measure(o);
        env.dirty(union_rect(r1, r2));
    }
    env.toggle(false);
    o.w8(0x31, o.r8(0x30));
    true
}
