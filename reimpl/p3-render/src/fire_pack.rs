//! Paquete de fragmentos del incendio (`0x585220` constructor, `0x585250` destructor, `0x5853E0` reinicio, `0x585430` lector de la lista de piezas,
//! `0x5852A0` lector del paquete). Traducción 1:1 sobre memoria plana con paridad en `tests/x86_fire_pack.rs` (`oracle/x86_fire_pack.py`).
//!
//! Disposición: `+0` vtable `0x677824`, `+4` tabla de rectángulos `{x, y, w, h}` (u16 ×4) con `+0x14` entradas, `+8` lista de `+0x16` palabras (u32),
//! `+0xC` superficie, `+0x10`/`+0x12`/`+0x14`/`+0x16` campos de 16 bits. La superficie la crea `0x54F290` (caché de imágenes), aquí un gancho.
use crate::scene_math::Mem;

pub const VTABLE: u32 = 0x0067_7824;

pub trait PackEnv {
    /// `0x64F7B9` (`operator new`) con la memoria sin inicializar; el llamador pone a cero cuando el original lo hace.
    fn alloc(&mut self, size: u32) -> u32;
    /// `0x64F7E2` (`free`).
    fn free(&mut self, addr: u32);
    /// `0x54F290(datos, &pack+0xC, a, 0, b)`.
    fn surface(&mut self, data: u32, slot: u32, a: u32, b: u32);
}

/// `0x585220`: constructor.
pub fn ctor(m: &mut dyn Mem, p: u32) {
    m.w32(p, VTABLE);
    m.w32(p + 4, 0);
    m.w32(p + 8, 0);
    m.w32(p + 0xC, 0);
    m.w16(p + 0x10, 0);
    m.w16(p + 0x12, 0);
    m.w16(p + 0x14, 0);
    m.w16(p + 0x16, 0);
}

/// `0x5853E0`: libera y deja a cero.
pub fn reset(m: &mut dyn Mem, p: u32, env: &mut dyn PackEnv) {
    for o in [4, 8] {
        let v = m.r32(p + o);
        if v != 0 {
            env.free(v);
        }
        m.w32(p + o, 0);
    }
    m.w16(p + 0x16, 0);
    m.w16(p + 0x10, 0);
    m.w16(p + 0x12, 0);
    m.w16(p + 0x14, 0);
    m.w32(p + 0xC, 0);
}

/// `0x585250`: destructor (vtable propia y reinicio inline).
pub fn dtor(m: &mut dyn Mem, p: u32, env: &mut dyn PackEnv) {
    m.w32(p, VTABLE);
    reset(m, p, env);
}

/// `0x585430`: lee `n` (u16 en el cursor, avanza 4) y copia `n` palabras a una lista nueva en `+8`.
fn read_list(m: &mut dyn Mem, p: u32, cur: u32, env: &mut dyn PackEnv) {
    let c = m.r32(cur);
    let n = m.r16(c);
    m.w16(p + 0x16, n);
    m.w32(cur, c.wrapping_add(4));
    if n != 0 {
        let d = env.alloc(u32::from(n) << 2);
        m.w32(p + 8, d);
        let src = m.r32(cur);
        for i in 0..u32::from(n) << 2 {
            let b = m.r8(src.wrapping_add(i));
            m.w8(d.wrapping_add(i), b);
        }
        m.w32(cur, src.wrapping_add(u32::from(n) << 2));
    }
}

/// `0x5852A0(cursor, a, b)`: devuelve el puntero a los datos de imagen o 0.
pub fn load(m: &mut dyn Mem, p: u32, cur: u32, a: u32, b: u32, env: &mut dyn PackEnv) -> u32 {
    reset(m, p, env);
    read_list(m, p, cur, env);
    let mut q = m.r32(cur);
    for (o, k) in [(0x10u32, 0u32), (0x12, 4), (0x14, 8)] {
        let v = m.r16(q.wrapping_add(k));
        m.w16(p + o, v);
    }
    q = q.wrapping_add(12);
    m.w32(cur, q);
    let n = u32::from(m.r16(p + 0x14));
    let t = env.alloc(n << 3);
    for i in 0..(n << 3) {
        m.w8(t.wrapping_add(i), 0);
    }
    m.w32(p + 4, t);
    for i in 0..n {
        let mut c = m.r32(cur);
        for k in 0..4u32 {
            let v = m.r16(c);
            m.w16(t.wrapping_add(i * 8 + 2 * k), v);
            c = c.wrapping_add(4);
        }
        m.w32(cur, c);
    }
    let c = m.r32(cur);
    let len = m.r32(c);
    let data = c.wrapping_add(4);
    m.w32(cur, data);
    if len == 0 {
        m.w32(p + 0xC, a);
        return 0;
    }
    let after = data.wrapping_add(len);
    let tag = m.r32(after);
    m.w32(cur, after.wrapping_add(4));
    env.surface(data, p + 0xC, tag, b);
    data
}
