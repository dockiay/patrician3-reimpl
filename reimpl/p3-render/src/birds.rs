//! Paso de las aves del pueblo (tipo 15) del juego original: `0x563EF0`.
//!
//! Observado y probado en emulador (`oracle/x86_bird_step.py`, 4.000 casos con el x86 real, FPU a 0x27F y `__ftol` original). Los ayudantes de dibujo
//! (`0x559260` rectángulos, `UnionRect`, `0x59B5B0` rectángulo sucio), el avance de fotograma `0x557340` y `0x5576B0` se sustituyen por el trait
//! [`BirdEnv`]; los rectángulos no se modelan, solo el orden de las llamadas y los desplazamientos temporales de la entrada de fotograma.

/// Estado del ave: campos del objeto.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BirdState {
    pub x: u32,
    pub y: u32,
    /// `+0xE`, `+0x12`
    pub px: i16,
    pub py: i16,
    /// `+0x14`
    pub heading: u8,
    /// `+0x15`
    pub frame: u8,
    /// `+0x28`
    pub state: u8,
    /// `+0x2C`: altura/tamaño.
    pub h2c: u8,
    /// `+0x2D`: contador de animación.
    pub s2d: u8,
}

/// Registro por variante (`DAT_006E1728 + variante*0x14`).
#[derive(Clone, Copy, Debug)]
pub struct BirdVariant {
    /// `+8`
    pub threshold: u32,
    /// `+0xC`
    pub base: u32,
    /// `+0x10`
    pub range: u32,
}

pub struct BirdCtx<'a> {
    /// `escena+0x414`
    pub budget: u32,
    /// `escena+0xC31C`, `+0xC320`
    pub size: (u32, u32),
    /// float global `0x6E51B0`
    pub scale: f32,
    /// `juego+0xC`
    pub speed: u32,
    /// `juego+0x14`: número de direcciones (potencia de dos).
    pub ndirs: u32,
    /// `juego+0x20`
    pub counts: &'a [u32],
    pub variant: BirdVariant,
}

pub trait BirdEnv {
    /// `0x557340(juego, obj+8, &contador)`: devuelve el fotograma y actualiza el contador.
    fn advance(&mut self, counter: &mut u8) -> u8;
    fn rand(&mut self) -> u32;
    /// `0x5576B0(destino)`.
    fn set_target(&mut self, t: u8);
    /// `0x559260` con la entrada de fotograma desplazada `(+8, +10)`; devuelve si hay segundo rectángulo.
    fn measure(&mut self, d8: i16, d10: i16) -> bool;
    /// `UnionRect` ×2.
    fn union(&mut self);
    /// `0x59B5B0`: marca sucio el primer o el segundo rectángulo.
    fn dirty(&mut self, second: bool);
}

fn ftol(v: f64) -> i32 {
    v as i32
}

/// Desplazamientos temporales de la entrada de fotograma: (`+8`, `+10`).
fn shifts(h2c: u8) -> (i16, i16) {
    (-((h2c / 5) as i16), -(h2c as i16))
}

/// `0x563EF0`: avanza un paso; devuelve `true` si el ave sigue dentro de la escena.
pub fn step(b: &mut BirdState, c: &BirdCtx, env: &mut dyn BirdEnv) -> bool {
    let (mut nx, mut ny) = (b.x, b.y);
    let v = c.budget.wrapping_mul(c.speed).wrapping_mul(0x1000);
    let u = (((u64::from(v)) * 0xd1b7_1759) >> 32) as u32 >> 9;
    let ff = f64::from(f64::from(u as i32) as f32) * f64::from(c.scale);
    let s = f64::from(c.scale);
    let diag = ftol(ff + (1.0 - s) / 2.0);
    let ortho = ftol(ff);
    match b.heading {
        0x00 => ny = ny.wrapping_sub(ortho as u32),
        0x80 => ny = ny.wrapping_add(ortho as u32),
        0x40 => nx = nx.wrapping_add(u),
        0xC0 => nx = nx.wrapping_sub(u),
        0x20 => {
            nx = nx.wrapping_add(diag as u32);
            ny = ny.wrapping_sub(diag as u32);
        }
        0x60 => {
            nx = nx.wrapping_add(diag as u32);
            ny = ny.wrapping_add(diag as u32);
        }
        0xA0 => {
            nx = nx.wrapping_sub(diag as u32);
            ny = ny.wrapping_add(diag as u32);
        }
        0xE0 => {
            nx = nx.wrapping_sub(diag as u32);
            ny = ny.wrapping_sub(diag as u32);
        }
        _ => {}
    }
    let old_s2d = b.s2d;
    let frame = env.advance(&mut b.s2d);
    let mut heading = b.heading;
    if old_s2d != b.s2d && c.variant.base < u32::from(b.s2d) {
        let r = env.rand();
        b.s2d = (c.variant.base as u8).wrapping_sub((r % c.variant.range.max(1)) as u8);
        let st = b.state;
        if st == 0xA1 {
            env.set_target(0xE1);
        } else if st == 0 {
            let dir = (((u32::from(b.heading) * c.ndirs + 0x80) >> 8) & c.ndirs.wrapping_sub(1)) as usize;
            let mut skip = false;
            if c.variant.threshold < c.counts[dir] {
                let r = env.rand();
                if r & 1 != 0 {
                    env.set_target(0x81);
                    skip = true;
                }
            }
            if !skip {
                heading = if b.s2d & 1 == 0 { heading.wrapping_add(0x20) } else { heading.wrapping_sub(0x20) };
            }
        }
    }
    if nx >> 16 == b.px as i32 as u32 && ny >> 16 == b.py as i32 as u32 && frame == b.frame && heading == b.heading {
        b.x = nx;
        b.y = ny;
        b.px = (nx >> 16) as i16;
        b.py = (ny >> 16) as i16;
        return true;
    }
    let (d8, d10) = shifts(b.h2c);
    let two = env.measure(d8, d10);
    b.frame = frame;
    b.x = nx;
    b.y = ny;
    b.px = (nx >> 16) as i16;
    b.py = (ny >> 16) as i16;
    b.heading = heading;
    let h = u32::from(b.h2c);
    if nx >> 16 <= h / 5 + c.size.0 && ny >> 16 <= c.size.1 + h {
        env.measure(d8, d10);
        env.union();
        env.union();
        env.dirty(false);
        if two {
            env.dirty(true);
        }
        true
    } else {
        env.dirty(false);
        if two {
            env.dirty(true);
        }
        false
    }
}
