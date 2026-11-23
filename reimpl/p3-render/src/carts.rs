//! Paso de los carros que siguen una ruta de puntos (tipo 31): `0x570120` con sus ayudantes `0x570800`, `0x5709C0` y `0x570AB0`.
//!
//! Observado y probado en emulador (`oracle/x86_cart_step.py`, 4.000 casos con las cuatro funciones x86 reales). Solo `0x5595F0`/`0x559790` (sacar y volver
//! a meter en la rejilla de ocupación) se sustituyen por [`CartEnv`]. La ruta son dos listas de puntos `a` y `b` (las dos orillas del carril); el punto medio
//! de la posición `i` es `(a[i] + b[n-1-i]) / 2`.

#[derive(Clone, Debug, Default)]
pub struct Cart {
    pub x: u32,
    pub y: u32,
    /// `+0xE`, `+0x12`
    pub px: i16,
    pub py: i16,
    pub heading: u8,
    pub frame: u8,
    /// `+0x30`, `+0x34`: posición candidata.
    pub x30: u32,
    pub y30: u32,
    /// `+0x38`: punto actual de la ruta.
    pub idx: u8,
    /// `+0x39`: punto anterior.
    pub prev: u8,
    /// `+0x3A`: sentido (0 = decreciente).
    pub dir: u8,
    /// `+0x3B`: número de puntos.
    pub n: u8,
    /// `+0x40`, `+0x44`, `+0x48`: duración, inicio y progreso.
    pub dur: u32,
    pub start: u32,
    pub prog: u32,
    /// `+0x4C`
    pub frac: u8,
    /// `+0x50`, `+0x54`: velocidad por unidad de progreso (16.16).
    pub vx: i32,
    pub vy: i32,
    /// `+0x58`: rumbo base del tramo.
    pub h58: u8,
    /// `+0x5A`: terminado.
    pub done: bool,
    /// `+0x5B`, `+0x5C`, `+0x5D`: registros de fotograma (giro, giro, recto).
    pub s5b: u8,
    pub s5c: u8,
    pub s5d: u8,
}

/// Elemento de la lista de carros (`DAT_006E6280`, siguiente en `+0x60`).
#[derive(Clone, Debug)]
pub struct CartLink {
    pub dir: u8,
    /// `+0x64`
    pub tag: i32,
    pub done: bool,
}

pub struct Route<'a> {
    pub a: &'a [(i32, i32)],
    pub b: &'a [(i32, i32)],
}

pub struct CartCtx<'a> {
    /// `escena+0xC674` y `+0xC678` presentes.
    pub routes_present: bool,
    pub route: Route<'a>,
    /// `DAT_006DCCF8`, `DAT_006DFC1C`, `DAT_006DFBC4`
    pub time: u32,
    pub fc1c: u32,
    pub bc4: u32,
    /// `+0xC650`, `+0xC654`, `+0x2C`, `+0x30`
    pub view: (u32, u32, u32, u32),
    /// `juego+0x2C`: registros de 12 bytes, mínimo y máximo de fotograma.
    pub recs: &'a [(u8, u8)],
}

pub trait CartEnv {
    /// `0x5595F0` con la entrada de fotograma y `py` desplazadas en `(-0x21; +0x21, +0x21)`.
    fn remove(&mut self);
    /// `0x559790` ídem, con `py` ya actualizado: recibe `py` nueva menos `py` anterior menos `0x21`.
    fn add(&mut self, dpy: i32);
}

fn mid(r: &Route, n: u8, i: u8) -> (u32, u32) {
    let (a, b) = (r.a[usize::from(i)], r.b[usize::from(n - i - 1)]);
    ((b.0.wrapping_add(a.0) as u32) >> 1, (b.1.wrapping_add(a.1) as u32) >> 1)
}

fn mid_s(r: &Route, n: u8, i: u8) -> (u32, u32) {
    let (a, b) = (r.a[usize::from(i)], r.b[usize::from(n - i - 1)]);
    ((b.0.wrapping_add(a.0) >> 1) as u32, (b.1.wrapping_add(a.1) >> 1) as u32)
}

/// `0x570800`: velocidad y rumbo hacia el punto actual.
pub fn retarget(c: &mut Cart, r: &Route) {
    let (mx, my) = mid(r, c.n, c.idx);
    let (px, py) = (i32::from(c.px) as u32, i32::from(c.py) as u32);
    if mx <= px {
        c.vx = -0x10000;
        if py < my {
            c.h58 = 0xA0;
            c.vy = 0xA5A6;
        } else {
            c.h58 = 0xE0;
            c.vy = 0xFFFF_5A5Au32 as i32;
        }
    } else {
        c.vx = 0x10000;
        if py < my {
            c.h58 = 0x60;
            c.vy = 0xA5A6;
        } else {
            c.h58 = 0x20;
            c.vy = 0xFFFF_5A5Au32 as i32;
        }
    }
}

fn pick(lt_x: bool, lt_y: bool) -> u8 {
    if lt_x {
        if lt_y {
            0x60
        } else {
            0x20
        }
    } else if lt_y {
        0xA0
    } else {
        0xE0
    }
}

/// `0x5709C0`: rumbo hacia el punto siguiente en el sentido de la marcha.
pub fn heading_next(c: &Cart, r: &Route) -> u8 {
    let i = c.idx;
    let nx = if c.dir == 0 {
        if i == 0 {
            return c.h58;
        }
        i - 1
    } else {
        if i == c.n.wrapping_sub(1) {
            return c.h58;
        }
        i + 1
    };
    let (mx, my) = mid(r, c.n, i);
    let (nmx, nmy) = mid(r, c.n, nx);
    pick(mx < nmx, my < nmy)
}

/// `0x570AB0`: rumbo del tramo anterior.
pub fn heading_prev(c: &Cart, r: &Route) -> u8 {
    let i = c.prev;
    let nx = if c.dir == 0 {
        if i == c.n.wrapping_sub(1) {
            return c.h58;
        }
        i + 1
    } else {
        if i == 0 {
            return c.h58;
        }
        i - 1
    };
    let (mx, my) = mid(r, c.n, i);
    let (nmx, nmy) = mid(r, c.n, nx);
    pick(nmx < mx, nmy < my)
}

fn dist_x(r: &Route, n: u8, i: u8, x30: u32) -> u32 {
    let (a, b) = (r.a[usize::from(i)], r.b[usize::from(n - i - 1)]);
    let m = (b.0.wrapping_add(a.0) as u32) >> 1;
    let d = m.wrapping_sub((x30 >> 16) & 0xFFFF) as i32;
    d.wrapping_abs() as u32
}

/// Fotograma de un tramo recto con el desfase `fr` (`0x11 - dist`).
fn tail(q: i32, lo: u8) -> u8 {
    (q as u8).wrapping_sub((q >> 31) as u8).wrapping_add(lo)
}

/// `0x570120`: avanza el carro hasta el progreso del reloj; `true` si sigue activo.
pub fn step(c: &mut Cart, links: &mut [CartLink], ctx: &CartCtx, env: &mut dyn CartEnv) -> bool {
    if c.done || !ctx.routes_present {
        return false;
    }
    let dur = c.dur;
    let elapsed = ctx.time.wrapping_sub(c.start);
    let mut prog = if ctx.fc1c == 0 {
        if ctx.bc4 < elapsed {
            dur / 0xff
        } else {
            dur.wrapping_mul(elapsed) / ctx.bc4.wrapping_mul(0xff)
        }
    } else if elapsed < 0xd30 {
        dur.wrapping_mul(elapsed) / 0xd21d1
    } else {
        dur / 0xff
    };
    prog = (u32::from(c.frac).wrapping_mul(dur) / 0xff).wrapping_add(prog);
    let (v650, v654, s2c, s30) = ctx.view;
    if dur <= prog {
        let px = i32::from(c.px) as u32;
        let py = i32::from(c.py) as u32;
        if px.wrapping_add(100) < v650 || py.wrapping_add(100) < v654 || s2c.wrapping_add(100).wrapping_add(v650) < px || s30.wrapping_add(100).wrapping_add(v654) < py {
            return false;
        }
        prog = dur;
    }
    let last = c.prog;
    if prog <= last {
        return true;
    }
    c.prog = prog;
    let mut delta = prog.wrapping_sub(last) as i32;
    c.x30 = c.x;
    c.y30 = c.y;
    let mut turn: i8 = 0;
    let r = &ctx.route;
    while delta != 0 {
        let vx = c.vx;
        c.x30 = c.x30.wrapping_add(vx.wrapping_mul(delta) as u32);
        c.y30 = c.y30.wrapping_add(c.vy.wrapping_mul(delta) as u32);
        let i = c.idx;
        let (mxs, mys) = mid_s(r, c.n, i);
        let w = (c.x30 >> 16) & 0xFFFF;
        if (vx < 1 || w < mxs) && (-1 < vx || mxs < w) {
            break;
        }
        if c.idx == 0 {
            return false;
        }
        if i == c.n.wrapping_sub(1) {
            c.x30 = mxs << 16;
            c.y30 = mys << 16;
            break;
        }
        delta = (w.wrapping_sub(mxs) as i32).wrapping_abs();
        c.x30 = mxs << 16;
        c.y30 = mys << 16;
        c.prev = c.idx;
        c.idx = if c.dir == 0 { c.idx.wrapping_sub(1) } else { c.idx.wrapping_add(1) };
        retarget(c, r);
        if heading_next(c, r) == c.h58 {
            loop {
                let b = c.idx;
                if b == 0 || c.n.wrapping_sub(1) == b {
                    break;
                }
                c.idx = if c.dir == 0 { b.wrapping_sub(1) } else { b.wrapping_add(1) };
                if heading_next(c, r) != c.h58 {
                    break;
                }
            }
        }
        if c.dir != 0 && c.idx == c.n.wrapping_sub(1) {
            for l in links.iter_mut() {
                if l.dir != 0 && l.tag == -1 {
                    l.done = true;
                }
            }
        }
    }
    let rec = |k: u8| ctx.recs[usize::from(k)];
    let d = dist_x(r, c.n, c.idx, c.x30);
    let mut fr: u8;
    let tail_straight = |c: &Cart, u9: u32| -> u8 {
        let (lo, hi) = rec(c.s5d);
        let iv = (i32::from(hi) - i32::from(lo) + 1).wrapping_mul((((u9 & 0xFFFF) + 0x11) % 0x22) as i32);
        tail(iv / 0x22 + (iv >> 31), lo)
    };
    let second = |c: &Cart, turn: &mut i8| -> u8 {
        let u9 = dist_x(r, c.n, c.prev, c.x30);
        if (u9 & 0xFFFF) < 0x12 {
            let cc = heading_prev(c, r).wrapping_sub(c.h58) as i8;
            if cc != 0 {
                let (lo, hi) = if cc < 0 { rec(c.s5c) } else { rec(c.s5b) };
                let half = (((i32::from(hi) - i32::from(lo)) >> 1) as u8).wrapping_add(lo).wrapping_add(1);
                let iv = (i32::from(hi) - i32::from(half) + 1).wrapping_mul((u9 & 0xFF) as i32);
                let f = ((iv / 0x12) as u8).wrapping_add((iv >> 31) as u8).wrapping_sub(((i64::from(iv) * 0x38e3_8e39) >> 63) as u8).wrapping_add(half);
                *turn = cc;
                return f;
            }
        }
        tail_straight(c, u9)
    };
    if (d & 0xFFFF) < 0x12 {
        let cc = heading_next(c, r);
        if cc == c.h58 {
            fr = second(c, &mut turn);
        } else {
            let bv = (0x11u8).wrapping_sub(d as u8);
            let (lo, hi) = if (cc.wrapping_sub(c.h58) as i8) < 0 { rec(c.s5b) } else { rec(c.s5c) };
            let t = i32::from(((((i32::from(hi) - i32::from(lo)) >> 1) as u8).wrapping_add(lo))) - i32::from(lo);
            let iv = (t + 1).wrapping_mul(i32::from(bv));
            fr = tail(iv / 0x12 + (iv >> 31), lo);
        }
    } else {
        fr = second(c, &mut turn);
    }
    let _ = &mut fr;
    let py_old = i32::from(c.py);
    env.remove();
    c.frame = fr;
    c.x = c.x30;
    c.y = c.y30;
    // `+0xE` y `+0x12` son la parte alta de `x` e `y`.
    c.px = (c.x >> 16) as i16;
    c.py = (c.y >> 16) as i16;
    c.heading = c.h58.wrapping_add(turn as u8);
    env.add(i32::from(c.py) - py_old - 0x21);
    true
}
