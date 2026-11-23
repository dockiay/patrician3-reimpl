//! `0x59BC90(x, y, p)`: objeto de efecto tipo `0x22` colocado en el cruce de dos rectas de la silueta de un objeto del mapa.
//! Traducción 1:1 del x86 (aritmética x87 de 53 bits con almacenamientos a `f32`); la escena es memoria plana.
use crate::ambience::SceneMem;

pub trait CrackEnv {
    /// `0x57FF20(x, y, p)`: distinto de cero cancela la operación.
    fn skip(&mut self, x: u32, y: u32, p: u32) -> bool;
    /// Valor de la casilla `indice = c314 * y + x`.
    fn cell(&mut self, idx: u32) -> u16;
    /// `0x58DAC0(&(x, y), &salida)`
    fn pair(&mut self, x: u32, y: u32) -> (u32, u32);
    /// Métodos virtuales `+0x48` y `+0x4C` del objeto del mapa.
    fn obj_w(&mut self, h: u16) -> i32;
    fn obj_h(&mut self, h: u16) -> i32;
    fn make(&mut self, ty: u32) -> u32;
    fn new_link(&mut self) -> u16;
    fn register(&mut self, h: u32, link: u16);
    /// Virtual `+0x40(x, y, 0, enlace, empaquetado)`
    fn spawn(&mut self, h: u32, x: i32, y: i32, a: u32, link: u32, packed: u32);
    /// `byte [objeto+0x29] = v`
    fn set_byte29(&mut self, h: u32, v: u8);
}

/// `__ftol` (`0x639C28`): `fistp` de 64 bits con truncación y devuelve la mitad baja; NaN o fuera de rango dan el «entero indefinido» de 64 bits (baja = 0).
pub fn ftol(v: f64) -> i32 {
    let q = if v > -9.223_372_036_854_775_808e18 && v < 9.223_372_036_854_775_808e18 { v as i64 } else { i64::MIN };
    q as i32
}

pub fn spawn_crack(m: &SceneMem, x: u32, y: u32, p: u32, env: &mut dyn CrackEnv) {
    if env.skip(x, y, p) {
        return;
    }
    let idx = m.r32(0xC314).wrapping_mul(y).wrapping_add(x);
    let h = env.cell(idx);
    let (qx, qy) = env.pair(x, y);
    let c4d0 = m.r32(0x4D0) as i32;
    let c4d4 = m.r32(0x4D4) as i32;
    let qy = (qy as i32).wrapping_add((m.r32(0x4D4) as i32).wrapping_sub(m.r32(0x18) as i32));
    let qx = (qx as i32).wrapping_add(m.r32(0x510) as i32);
    let w = env.obj_w(h);
    let a = qx.wrapping_sub(w.wrapping_mul(c4d0));
    let w2 = env.obj_w(h);
    let b = qy.wrapping_sub(w2.wrapping_mul(c4d4));
    let h1 = env.obj_h(h);
    let c = h1.wrapping_mul(c4d0).wrapping_add(qx);
    let h2 = env.obj_h(h);
    let d = qy.wrapping_sub(h2.wrapping_mul(c4d4));
    let h3 = env.obj_h(h);
    let e = h3.wrapping_mul(c4d0).wrapping_add(a);
    let h4 = env.obj_h(h);
    let f = b.wrapping_sub(h4.wrapping_mul(c4d4)).wrapping_sub(qy);
    let g = e.wrapping_sub(qx);
    let s1 = ((f as f32) / (g as f32)) as f32;
    let s2 = (((d.wrapping_sub(b)) as f32) / ((c.wrapping_sub(a)) as f32)) as f32;
    let (qxf, qyf, af, bf) = (qx as f32, qy as f32, a as f32, b as f32);
    let c1 = (f64::from(qyf) - f64::from(qxf) * f64::from(s1)) as f32;
    let c2 = (f64::from(bf) - f64::from(af) * f64::from(s2)) as f32;
    let xv = ((f64::from(c2) - f64::from(c1)) / (f64::from(s1) - f64::from(s2))) as f32;
    let yv = (f64::from(xv) * f64::from(s1) + f64::from(c1)) as f32;
    let obj = env.make(0x22);
    let link = env.new_link();
    env.register(obj, link);
    let packed = (y & 0xFFFF) << 16 | (x & 0xFFFF);
    let yi = ftol(f64::from(yv)).wrapping_sub(10);
    let xi = ftol(f64::from(xv));
    env.spawn(obj, xi, yi, 0, u32::from(link), packed);
    env.set_byte29(obj, p as u8);
}
