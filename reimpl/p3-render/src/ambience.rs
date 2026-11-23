//! Atmósfera del pueblo: `0x58F4E0` (cambio de clima/niebla/oscuridad y fundido de color por franjas) con sus ayudantes `0x58F3C0` y `0x58F440`.
//! La escena se maneja como memoria plana (desplazamientos del original) para traducir el decompilado sin reinterpretar campos.

pub struct SceneMem(pub Vec<u8>);

impl SceneMem {
    pub fn new() -> Self {
        SceneMem(vec![0; 0xCA00])
    }
    pub fn sized(n: usize) -> Self {
        SceneMem(vec![0; n])
    }
    pub fn r16(&self, o: usize) -> u16 {
        u16::from_le_bytes([self.0[o], self.0[o + 1]])
    }
    pub fn w16(&mut self, o: usize, v: u16) {
        self.0[o..o + 2].copy_from_slice(&v.to_le_bytes());
    }
    pub fn r8(&self, o: usize) -> u8 {
        self.0[o]
    }
    pub fn w8(&mut self, o: usize, v: u8) {
        self.0[o] = v;
    }
    pub fn r32(&self, o: usize) -> u32 {
        u32::from_le_bytes([self.0[o], self.0[o + 1], self.0[o + 2], self.0[o + 3]])
    }
    pub fn w32(&mut self, o: usize, v: u32) {
        self.0[o..o + 4].copy_from_slice(&v.to_le_bytes());
    }
}

impl Default for SceneMem {
    fn default() -> Self {
        Self::new()
    }
}

/// Externos de `0x58F4E0`.
pub trait AmbEnv {
    /// `DAT_006DCCF8`
    fn time(&self) -> u32;
    /// `DAT_006DE4B0`
    fn town_count(&self) -> u32;
    /// `registro_ciudad(plan)+0x2C8`
    fn town_2c8(&self, plan: u32) -> u32;
    /// `registro_ciudad(plan)+0x9C4`
    fn town_9c4(&self, plan: u32) -> u8;
    /// `*(DAT_006CBDF0 + 400)`
    fn detail(&self) -> i32;
    fn rand(&mut self) -> u32;
    /// `0x4B3DD0(clave)` y lectura de `+4` del resultado.
    fn lookup(&mut self, key: u32) -> u32;
    /// `0x58B090(cantidad, inmediato)`
    fn weather(&mut self, count: i32, immediate: u32);
    /// `0x59B410()`
    fn f59b410(&mut self);
    /// `0x42C770(0, 0, 1)`
    fn f42c770(&mut self);
    fn make(&mut self, ty: u32) -> u32;
    fn new_link(&mut self) -> u16;
    fn register(&mut self, h: u32, link: u16);
    fn spawn(&mut self, h: u32, x: u32, y: u32, a: u8, link: u16, b: u32);
    /// `0x4B9650(&rect)`
    fn invalidate(&mut self, l: i32, t: i32, r: i32, b: i32);
    /// Método virtual `+0x34` del objeto en `escena+0x530`.
    fn notify(&mut self);
    /// Método `+4` del subobjeto `escena+0x3F8`.
    fn remove(&mut self, h: u32);
}

/// `0x58F3C0(base, destino, paso, salida)`: devuelve el mayor número de pasos y escribe en `salida` los incrementos (con signo) por byte.
pub fn step_count(m: &mut SceneMem, base: usize, target: usize, step: u32, out: usize) -> i32 {
    m.w32(out, 0);
    let mut max = 0i32;
    for k in 0..4 {
        let d = m.r8(target + k) as i32 - m.r8(base + k) as i32;
        if d != 0 {
            let s = (step & 0xFF) as i32;
            m.w8(out + k, if d < 1 { (-s) as u8 } else { s as u8 });
            let n = (d.abs() - 1 + s) / s;
            if max < n {
                max = n;
            }
        }
    }
    max
}

/// `0x58F440(base, destino, pasos, salida)`: incrementos para llegar en `steps` pasos.
pub fn step_delta(m: &mut SceneMem, base: usize, target: usize, steps: u8, out: usize) {
    m.w32(out, 0);
    if steps != 0 {
        for k in 0..4 {
            let d = m.r8(target + k) as i32 - m.r8(base + k) as i32;
            let s = steps as i32;
            if d < 0 {
                let d2 = d + (1 - s);
                m.w8(out + k, if d2 < -s { (d2 / s) as u8 } else { 0xFF });
            } else if d > 0 {
                let d2 = d - 1 + s;
                m.w8(out + k, if s < d2 { (d2 / s) as u8 } else { 1 });
            }
        }
    }
}

/// Avance de un color de cuatro bytes: `dest[k] += paso[k]` acotado por `limit[k]`.
fn advance4(m: &mut SceneMem, steps: u32, limit: usize, dest: usize) {
    for k in 0..4 {
        let d = (steps >> (8 * k)) as u8 as i8;
        let v = m.r8(dest + k) as i32 + d as i32;
        let lim = m.r8(limit + k) as i32;
        let v = if (d > 0 && lim < v) || (d < 0 && v < lim) { lim } else { v };
        m.w8(dest + k, v as u8);
    }
}

pub fn rect_empty(m: &SceneMem, o: usize) -> bool {
    let (l, t, r, b) = (m.r32(o) as i32, m.r32(o + 4) as i32, m.r32(o + 8) as i32, m.r32(o + 12) as i32);
    r <= l || b <= t
}

fn set_rect(m: &mut SceneMem, o: usize, l: i32, t: i32, r: i32, b: i32) {
    m.w32(o, l as u32);
    m.w32(o + 4, t as u32);
    m.w32(o + 8, r as u32);
    m.w32(o + 12, b as u32);
}

fn view_rect(m: &SceneMem) -> (i32, i32, i32, i32) {
    let l = m.r32(0x14) as i32;
    let t = m.r32(0x18) as i32;
    (l, t, (m.r32(0x2C) as i32).wrapping_add(l), (m.r32(0x30) as i32).wrapping_add(t))
}

fn clear_slots(m: &mut SceneMem, env: &mut dyn AmbEnv) {
    for k in 0..100 {
        let h = m.r32(0xC334 + 4 * k);
        if h != 0 {
            env.remove(h);
            m.w32(0xC334 + 4 * k, 0);
        }
    }
}

/// `0x58F4E0(inmediato)`.
pub fn update(m: &mut SceneMem, immediate: u32, env: &mut dyn AmbEnv) {
    if m.r8(0xC329) == 0 {
        if m.r8(0xC184) != 0 {
            env.f59b410();
            env.weather(0, 1);
        }
        return;
    }
    let plan = m.r32(0xC324);
    let (mut b10, mut b13, mut b12) = (false, false, false);
    if plan < env.town_count() {
        b12 = env.town_2c8(plan) as u8 & 8 == 8;
        let b1 = env.town_9c4(plan);
        b10 = b1 & 1 != 0;
        b13 = b1 & 6 != 0;
    }
    let state: u8 = if b13 {
        2
    } else if b12 {
        1
    } else if b10 {
        3
    } else {
        0
    };
    let mut recompute = false;
    let mut finish = true; // pasa por el fundido (franja 1)
    if state == m.r8(0xC184) {
        if state != 2 || ((m.r8(0xC177) != 0) == (env.town_9c4(plan) & 2 != 0)) {
            finish = false;
        } else {
            recompute = true;
        }
    } else if state != 0 {
        recompute = true;
    } else {
        env.weather(0, 0);
        m.w32(0xC1B0, 0);
        m.w32(0xC174, 0xFFFFFF);
    }
    if recompute {
        if m.r32(0xC330) == 0 {
            let r = env.rand();
            let v = env.lookup(r % m.r32(0xC1BC) + m.r32(0xC1B8));
            m.w32(0xC330, v);
        }
        let (mut b15, mut c16, mut c17, mut l18) = (0u8, 0xFFu8, 0xFFu8, 0xFFu8);
        let mut l35: u8 = 0;
        let mut spread = true;
        match state {
            1 => {
                env.weather(0, immediate & 0xFF);
                m.w32(0xC1B0, 0);
                m.w32(0xC17C, 0xFFFF_FFFF);
                let r = env.rand();
                l35 = 0x46;
                b15 = 0x9Bu8.wrapping_sub((r as i32 % 0x28) as u8);
            }
            2 => {
                env.weather(0x32, immediate & 0xFF);
                m.w32(0xC17C, 0xFF_FFFF);
                m.w32(0xC1B0, 0);
                if m.r8(0xC183) == 0 {
                    m.w32(0xC180, 0xFF_FFFF);
                }
                if env.town_9c4(m.r32(0xC324)) & 2 != 0 {
                    let r = env.rand();
                    b15 = 0x9Bu8.wrapping_sub((r as i32 % 0x28) as u8);
                    let r = env.rand();
                    l35 = 0x96u8.wrapping_sub((r as i32 % 0x28) as u8);
                } else {
                    spread = false;
                }
            }
            _ => {
                env.weather(0, immediate & 0xFF);
                if m.r32(0xC178) == 0 {
                    m.w32(0xC178, 0xFF_FFFF);
                }
                m.w32(0xC17C, 0xFFFF_FFFF);
                let r = env.rand();
                b15 = 0xB9u8.wrapping_sub((r as i32 % 0x28) as u8);
                let r = env.rand();
                m.w32(0xC1B0, 0xFFFF_FFFF);
                l35 = 0xFFu8.wrapping_sub((r as i32 % 0x14) as u8);
                if m.r32(0xC334) == 0 && 2 < env.detail() {
                    let mut slot = 0xC334usize;
                    let mut y = 0u32;
                    let step_y = m.r32(0xC320) / 0x14;
                    let w = m.r32(0xC31C);
                    for _ in 0..0x14 {
                        let mut x = 0u32;
                        for _ in 0..5 {
                            let w2 = m.r32(0xC31C);
                            let r = env.rand();
                            let h = env.make(0x1C);
                            let link = env.new_link();
                            env.register(h, link);
                            env.spawn(h, (r % (w2 / 5)).wrapping_sub(w2 / 10).wrapping_add(x), y, 0x20, link, 0);
                            m.w32(slot, h);
                            slot += 4;
                            x = x.wrapping_add(w / 5);
                        }
                        y = y.wrapping_add(step_y);
                    }
                }
            }
        }
        if spread && b15 != 0 {
            let r = env.rand();
            c16 = l35.wrapping_sub((r as i32 % 10) as u8);
            let r = env.rand();
            c17 = l35.wrapping_sub((r as i32 % 10) as u8);
            let r = env.rand();
            l18 = l35.wrapping_sub((r as i32 % 10) as u8);
        }
        m.w32(0xC174, (b15 as u32) << 24 | (c16 as u32) << 16 | (c17 as u32) << 8 | l18 as u32);
    }
    if finish {
        let n = step_count(m, 0xC178, 0xC174, 2, 0xC18C);
        step_delta(m, 0xC180, 0xC17C, n as u8, 0xC190);
        step_delta(m, 0xC1AC, 0xC1B0, n as u8, 0xC1B4);
        m.w8(0xC184, state);
        env.f42c770();
    }
    // Fundido
    if m.r32(0xC178) == m.r32(0xC174) && rect_empty(m, 0xC194) {
        return;
    }
    let time = env.time();
    if immediate as u8 == 0 {
        if m.r32(0xC5F8) != 0 || m.r32(0xC5FC) != 0 {
            let (vl, vt, vr, vb) = view_rect(m);
            let dx = (m.r32(0xC5F8) as i32).wrapping_neg();
            let dy = (m.r32(0xC5FC) as i32).wrapping_neg();
            let (l, t, r, b) = (m.r32(0xC194) as i32, m.r32(0xC198) as i32, m.r32(0xC19C) as i32, m.r32(0xC1A0) as i32);
            let (l, t, r, b) = (l.wrapping_add(dx), t.wrapping_add(dy), r.wrapping_add(dx), b.wrapping_add(dy));
            // IntersectRect
            let (il, it, ir, ib) = (l.max(vl), t.max(vt), r.min(vr), b.min(vb));
            if il < ir && it < ib && l < r && t < b && vl < vr && vt < vb {
                set_rect(m, 0xC194, il, it, ir, ib);
            } else {
                set_rect(m, 0xC194, 0, 0, 0, 0);
            }
        }
        if !rect_empty(m, 0xC194) {
            let top = m.r32(0xC198) as i32;
            let bot = m.r32(0xC1A0) as i32;
            let span = bot.wrapping_sub(top);
            let rem = (m.r32(0xC188) as i32).wrapping_sub(time as i32);
            let mut u: i32 = if rem < 1 {
                span
            } else {
                ((span.wrapping_mul(time.wrapping_sub(m.r32(0xC1A4)) as i32)) as u32 / rem as u32) as i32
            };
            let band = (m.r32(0x30) as i32) / 10;
            if band < span {
                let q = u / band;
                u -= u % band;
                if u == 0 {
                    let _ = q;
                    return;
                }
            }
            if band < u {
                u = band;
            }
            if span < u {
                u = span;
            }
            let left = m.r32(0xC194) as i32;
            let right = m.r32(0xC19C) as i32;
            env.invalidate(left, top, right, top.wrapping_add(u));
            m.w32(0xC198, (top as u32).wrapping_add(u as u32));
            if m.r32(0xC178) == m.r32(0xC174) && rect_empty(m, 0xC194) {
                let v = m.r32(0xC178);
                m.w32(0xC1A8, v);
                if m.r8(0xC1AF) == 0 && m.r32(0xC334) != 0 {
                    clear_slots(m, env);
                }
            }
            m.w32(0xC1A4, time);
            return;
        }
        if m.r32(0xC188) <= time {
            let s = m.r32(0xC190);
            if s != 0 {
                advance4(m, s, 0xC17C, 0xC180);
            }
            let s = m.r32(0xC1B4);
            if s != 0 {
                advance4(m, s, 0xC1B0, 0xC1AC);
            }
            let v = m.r32(0xC178);
            m.w32(0xC1A8, v);
            let s = m.r32(0xC18C);
            advance4(m, s, 0xC174, 0xC178);
            let (vl, vt, vr, vb) = view_rect(m);
            set_rect(m, 0xC194, vl, vt, vr, vb);
            m.w32(0xC188, time.wrapping_add(200));
        }
    } else {
        let v = m.r32(0xC174);
        m.w32(0xC178, v);
        let v = m.r32(0xC1B0);
        m.w32(0xC1AC, v);
        let v = m.r32(0xC17C);
        m.w32(0xC180, v);
        set_rect(m, 0xC194, 0, 0, 0, 0);
        let (vl, vt, vr, vb) = view_rect(m);
        let v = m.r32(0xC644);
        m.w32(0xC64C, v);
        let v = m.r32(0xC640);
        m.w32(0xC648, v);
        m.w8(0xC5F4, 0);
        env.invalidate(vl, vt, vr, vb);
        env.notify();
        if m.r32(0xC178) == m.r32(0xC174) && rect_empty(m, 0xC194) {
            let v = m.r32(0xC178);
            m.w32(0xC1A8, v);
            if m.r8(0xC1AF) == 0 && m.r32(0xC334) != 0 {
                clear_slots(m, env);
            }
        }
    }
}
