//! Paso de las ratas del pueblo (tipo 26): `0x573900`.
//!
//! Solo se mueven con la bandera 8 del registro de la ciudad o dentro de la vista, y solo sobre casillas con el byte de rejilla negativo. Observado y probado
//! en emulador (`oracle/x86_rat_step.py`); `0x55A2C0`, `0x557340`, `0x557730`, `0x58D030` y la rejilla se sustituyen por [`RatEnv`].

#[derive(Clone, Debug, Default)]
pub struct RatState {
    pub x: u32,
    pub y: u32,
    pub px: i16,
    pub py: i16,
    pub heading: u8,
    pub frame: u8,
    /// `+0x28`
    pub state: u8,
    /// `+0x38`
    pub counter: u8,
}

pub struct RatCtx {
    /// `escena+0xC324 < DAT_006DE4B0`: la ciudad existe.
    pub town_valid: bool,
    /// Bit 8 de `registro+0x2C8`: siempre activa.
    pub always: bool,
    /// `+0xC650`, `+0xC654`, `+0x2C`, `+0x30`
    pub view: (u32, u32, u32, u32),
    /// `+0xC31C`, `+0xC320`
    pub size: (u32, u32),
    /// `escena+0x414`
    pub budget: u32,
    /// `juego+0xC`
    pub speed: u32,
    /// Tabla por variante (`DAT_006E2E08 + variante*8`): base y rango del contador.
    pub base: u32,
    pub range: u32,
}

pub trait RatEnv {
    /// `0x58D030` y lectura de la rejilla de la ciudad: byte de la casilla bajo la rata.
    fn cell(&mut self, px: i16, py: i16) -> i8;
    fn try_step(&mut self, heading: u8) -> bool;
    fn advance(&mut self) -> (u8, u8);
    fn select(&mut self) -> u8;
    fn rand(&mut self) -> u32;
    fn remove(&mut self);
    fn add(&mut self);
}

/// `0x573900`; los rumbos han de ser múltiplos de `0x20`.
pub fn step(p: &mut RatState, c: &RatCtx, env: &mut dyn RatEnv) -> bool {
    if !c.town_valid {
        return false;
    }
    if !c.always {
        let (v650, v654, s2c, s30) = c.view;
        let (pxu, pyu) = (i32::from(p.px) as u32, i32::from(p.py) as u32);
        if pxu < v650 || s2c.wrapping_add(v650) < pxu || pyu < v654 || s30.wrapping_add(v654) < pyu {
            return false;
        }
    }
    if env.cell(p.px, p.py) >= 0 {
        return false;
    }
    let (mut nx, mut ny) = (p.x, p.y);
    let mut u = c.budget.min(1000);
    u = c.speed.wrapping_mul(u).wrapping_mul(0x1000) / 0x271;
    let quarter = |i: i32| (i.wrapping_mul(2)).wrapping_div(0x22) as u32;
    match p.heading {
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
    let old_counter = p.counter;
    let (mut frame, counter) = env.advance();
    p.counter = counter;
    let mut heading = p.heading;
    let mut blocked = false;
    if old_counter != p.counter {
        if !env.try_step(heading) {
            if p.state != 2 && p.state != 0x0B {
                frame = env.select();
            }
            let r = env.rand();
            p.counter = (c.base as u8).wrapping_sub((r % c.range.max(1)) as u8);
            let mut b = 0x20u8;
            loop {
                if env.try_step(p.heading.wrapping_add(b)) || env.try_step(p.heading.wrapping_sub(b)) {
                    break;
                }
                b = b.wrapping_add(0x20);
                if b >= 0x81 {
                    blocked = true;
                    break;
                }
            }
            if !env.try_step(p.heading.wrapping_add(b)) {
                heading = heading.wrapping_sub(0x20);
            } else if env.try_step(p.heading.wrapping_sub(b)) {
                let r = env.rand() & 1;
                heading = heading.wrapping_add((r as u8).wrapping_mul(0x40)).wrapping_sub(0x20);
            } else {
                heading = heading.wrapping_add(0x20);
            }
        } else if u32::from(p.counter) > c.base {
            let r = env.rand();
            p.counter = (c.base as u8).wrapping_sub((r % c.range.max(1)) as u8);
            if p.counter & 1 != 0 && env.try_step(p.heading.wrapping_sub(0x20)) {
                heading = heading.wrapping_sub(0x20);
            } else if env.try_step(p.heading.wrapping_add(0x20)) {
                heading = heading.wrapping_add(0x20);
            }
        }
    }
    let (nxs, nys) = (nx >> 16, ny >> 16);
    if nxs == i32::from(p.px) as u32 && nys == i32::from(p.py) as u32 && frame == p.frame && heading == p.heading {
        p.x = nx;
        p.y = ny;
        p.px = (nx >> 16) as i16;
        p.py = (ny >> 16) as i16;
        return true;
    }
    if nxs <= c.size.0 && nys <= c.size.1 && !blocked {
        env.remove();
        p.x = nx;
        p.y = ny;
        p.px = (nx >> 16) as i16;
        p.py = (ny >> 16) as i16;
        p.frame = frame;
        p.heading = heading;
        env.add();
        return true;
    }
    false
}
