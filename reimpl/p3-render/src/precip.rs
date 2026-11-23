//! Reparto de partículas de lluvia/nieve en un rectángulo: `0x58AD30(rect, cuenta)` y el envoltorio genérico de aparición `0x590610`.
//! Traducción 1:1 del x86; la escena es memoria plana ([`SceneMem`]) y los externos pasan por [`PrecipEnv`].
use crate::ambience::SceneMem;

/// Estructura devuelta por el método virtual `+0x28` del objeto (`+4`/`+6` sin signo, `+8`/`+0xA` con signo).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SpriteInfo {
    pub w4: u16,
    pub w6: u16,
    pub s8: i16,
    pub sa: i16,
}

pub trait PrecipEnv {
    fn rand(&mut self) -> i32;
    /// Método virtual `+0x20`: tipo del objeto.
    fn obj_type(&mut self, h: u32) -> u8;
    /// Destructor virtual `+0` con el argumento 1.
    fn destroy(&mut self, h: u32);
    /// Fábrica `0x557D10(tipo)`.
    fn make(&mut self, ty: u32) -> u32;
    /// `0x568A70(1)`
    fn new_link(&mut self) -> u16;
    /// `0x568B20(objeto, enlace)`
    fn register(&mut self, h: u32, link: u16);
    /// Virtual `+0x40(x, y, a, enlace, v)`
    fn spawn(&mut self, h: u32, x: u32, y: u32, a: u32, link: u32, v: u32);
    /// Virtual `+0x28(&byte)`
    fn info(&mut self, h: u32) -> SpriteInfo;
    /// Virtual `+0x48(x, y, 0)`
    fn place(&mut self, h: u32, x: u32, y: u32);
    /// Virtual `+0x1C()`
    fn refresh(&mut self, h: u32);
}

/// `0x590610`: crea el objeto si hace falta, le da un enlace nuevo si `link == 0` y lo coloca con el virtual `+0x40(x, y, c, enlace, v)`.
#[allow(clippy::too_many_arguments)]
pub fn spawn_object(x: u32, y: u32, ty: u32, v: u32, c: u32, link: u32, obj: u32, env: &mut dyn PrecipEnv) -> u32 {
    let h = if obj == 0 { env.make(ty) } else { obj };
    let link = if link as u16 == 0 { u32::from(env.new_link()) } else { link };
    env.register(h, link as u16);
    env.spawn(h, x, y, c, link, v);
    h
}

/// `0x58AD30(rect, cuenta)`: `limit` es `[0x6DE4B0]` y `snow` el bit 2 del registro de ciudad (`+0x9C4`) de la ciudad `scene+0xC324 & 0x7F`.
/// `rect` es `[izq, arriba, der, abajo]`.
pub fn fill_rect(m: &mut SceneMem, rect: [i32; 4], mut count: i32, limit: u32, snow: bool, env: &mut dyn PrecipEnv) {
    if m.r32(0xC324) >= limit {
        return;
    }
    let ty: u8 = if snow { 0x1D } else { 0x21 };
    let mut i = m.r32(0xC7BC) as i32;
    let end = m.r32(0xC7CC) as i32;
    if i < end {
        loop {
            if count == 0 {
                break;
            }
            let r = env.rand();
            let (l, t, rr, b) = (rect[0], rect[1], rect[2], rect[3]);
            let x0 = (r.wrapping_rem(rr.wrapping_sub(l))).wrapping_add(l);
            let mut y = (r.wrapping_rem(b.wrapping_sub(t))).wrapping_add(t);
            let (bh, dl) = if ty == 0x1D {
                (0x3Cu8.wrapping_sub((r % 0x14) as u8), ((r % 0x3C) as u8).wrapping_sub(0x1E))
            } else {
                (0x3Cu8.wrapping_sub((r % 0x14) as u8), ((r % 10) as u8).wrapping_sub(0xF))
            };
            let v = ((u32::from(bh) << 8 | u32::from(dl)) & 0xFFFF) << 16;
            let slot = 0xC6F0 + 4 * m.r32(0xC7BC) as usize;
            let cur = m.r32(slot);
            if env.obj_type(cur) != ty {
                if cur != 0 {
                    env.destroy(cur);
                }
                let n = env.make(u32::from(ty));
                m.w32(slot, n);
            }
            let mut h = m.r32(slot);
            if h == 0 {
                h = env.make(u32::from(ty));
            }
            let link = env.new_link();
            env.register(h, link);
            let mut x = x0;
            env.spawn(h, x as u32, y as u32, 0, u32::from(link), v);
            let info = env.info(h);
            let (w4, w6) = (u32::from(info.w4), u32::from(info.w6));
            let (s8, sa) = (info.s8 as i32 as u32, info.sa as i32 as u32);
            let s650 = m.r32(0xC650);
            let s654 = m.r32(0xC654);
            let s2c = m.r32(0x2C);
            let s30 = m.r32(0x30);
            let (l, t, rr, b) = (l as u32, t as u32, rr as u32, b as u32);
            let mut xu = x as u32;
            let mut yu = y as u32;
            if l == s650 {
                if rr == s2c.wrapping_add(s650) {
                    if t == s654 {
                        if sa.wrapping_add(w6).wrapping_add(yu) > s654.wrapping_sub(t).wrapping_add(b) {
                            yu = s654.wrapping_sub(w6).wrapping_sub(sa).wrapping_sub(t).wrapping_add(b);
                        }
                    } else if sa.wrapping_add(yu) < s30.wrapping_sub(b).wrapping_add(s654).wrapping_add(t) {
                        yu = s30.wrapping_sub(sa).wrapping_sub(b).wrapping_add(s654).wrapping_add(t);
                    }
                } else if w4.wrapping_add(s8).wrapping_add(xu) > s650.wrapping_sub(l).wrapping_add(rr) {
                    xu = s650.wrapping_sub(w4).wrapping_sub(s8).wrapping_sub(l).wrapping_add(rr);
                }
            } else if s8.wrapping_add(xu) < s2c.wrapping_sub(rr).wrapping_add(s650).wrapping_add(l) {
                xu = s2c.wrapping_sub(s8).wrapping_sub(rr).wrapping_add(s650).wrapping_add(l);
            }
            x = xu as i32;
            y = yu as i32;
            env.place(h, x as u32, y as u32);
            env.refresh(h);
            count -= 1;
            i += 1;
            m.w32(0xC7BC, m.r32(0xC7BC).wrapping_add(1));
            if i >= end {
                break;
            }
        }
    }
    let n = m.r32(0xC7BC) as i32;
    if n > m.r32(0xC7C0) as i32 {
        m.w32(0xC7C0, n as u32);
    }
}

/// `0x58B090(cuenta, reinicio)`: fija cuántos objetos de precipitación hay (máximo 50), rellena con objetos nuevos las ranuras vacías y, con `reset`, retira los sobrantes.
/// `snow` es el bit 2 del registro de la ciudad (nieve `0x1D` en lugar de lluvia `0x21`). La rama que avisa al registro (ranura por debajo de la actual `0xC7BC`) no se reproduce.
pub fn set_capacity(m: &mut SceneMem, count: i32, reset: bool, snow: bool, env: &mut dyn PrecipEnv) {
    m.w32(0xC7CC, count as u32);
    if 0x32 < count {
        m.w32(0xC7CC, 0x32);
    }
    if reset {
        m.w32(0xC7C0, m.r32(0xC7CC));
    }
    let n = m.r32(0xC7CC) as i32;
    if n == 0 {
        m.w32(0xC7C4, 0);
    } else {
        let ty: u32 = if snow { 0x1D } else { 0x21 };
        let total: i64 = if ty == 0x1D { 9000 } else { 0x9C4 };
        m.w32(0xC7C4, (total / i64::from(n)) as u32);
        for i in 0..n as usize {
            if m.r32(0xC6F0 + 4 * i) == 0 {
                let h = env.make(ty);
                m.w32(0xC6F0 + 4 * i, h);
            }
        }
    }
    let n = m.r32(0xC7CC) as i32;
    if reset && n < 0x32 {
        for idx in n..0x32 {
            let slot = 0xC6F0 + 4 * idx as usize;
            let p = m.r32(slot);
            if p != 0 {
                assert!(idx >= m.r32(0xC7BC) as i32, "ranura anterior a la actual: aviso al registro sin reproducir");
                env.destroy(p);
                m.w32(slot, 0);
            }
        }
    }
}
