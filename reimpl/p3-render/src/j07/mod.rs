//! J07: funciones restantes del área «mapa y gráficos» portadas sobre la memoria del original.
//!
//! Cada porte trabaja sobre [`Mem`] con las direcciones y disposiciones del proceso original y delega en [`Host`] las llamadas a funciones
//! que quedan fuera del porte (asignador, capturas de la simulación, dibujo SGL, `_rand`...). `oracle/x86_j07.py` ejecuta las funciones
//! originales con esas mismas llamadas interceptadas y `tests/x86_j07.rs` exige iguales EAX, escrituras y secuencia de llamadas.

pub use crate::scene_math::Mem;

pub mod entity;
pub mod misc;
pub mod mover;
pub mod sheet;
pub mod sprite;
pub mod stats;
pub mod town;
pub mod walker;

/// Valor que devuelve un porte, con el ancho que fija el original (en `AL`/`AX` el resto de EAX no está definido).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ret {
    None,
    U8(u8),
    U16(u16),
    U32(u32),
}

impl Ret {
    pub fn matches(self, eax: u32) -> bool {
        match self {
            Ret::None => true,
            Ret::U8(v) => eax as u8 == v,
            Ret::U16(v) => eax as u16 == v,
            Ret::U32(v) => eax == v,
        }
    }
}

/// Llamada virtual: `call [[this] + slot]` con `ecx = this`.
pub fn vcall(m: &mut dyn Mem, h: &mut dyn Host, this: u32, slot: u32, args: &[u32]) -> u32 {
    let vt = m.r32(this);
    let f = m.r32(vt.wrapping_add(slot));
    h.call(m, f, this, args)
}

/// Ejecuta el porte de la función original `f` (ECX `ecx`, argumentos de pila `args`, pila de entrada `esp`). `None` si no hay porte.
pub fn run(f: u32, m: &mut dyn Mem, h: &mut dyn Host, ecx: u32, a: &[u32], esp: u32) -> Option<Ret> {
    stats::run(f, m, h, ecx, a, esp)
        .or_else(|| sprite::run(f, m, h, ecx, a, esp))
        .or_else(|| sheet::run(f, m, h, ecx, a, esp))
        .or_else(|| town::run(f, m, h, ecx, a, esp))
        .or_else(|| entity::run(f, m, h, ecx, a, esp))
        .or_else(|| mover::run(f, m, h, ecx, a, esp))
        .or_else(|| misc::run(f, m, h, ecx, a, esp))
        .or_else(|| crate::j08::run(f, m, h, ecx, a, esp))
}

/// `operator new` (`0x64F7B9`, cdecl, 1 argumento).
pub const NEW: u32 = 0x0064_F7B9;
/// `operator delete` (`0x64F7E2`, cdecl, 1 argumento).
pub const FREE: u32 = 0x0064_F7E2;

/// Llamadas a funciones que no forman parte del porte: el anfitrión las ejecuta (en la prueba, repite lo que hizo el original).
pub trait Host {
    /// Llama a `target` con `ecx` y argumentos de pila `args`; devuelve EAX.
    fn call(&mut self, m: &mut dyn Mem, target: u32, ecx: u32, args: &[u32]) -> u32;
    /// Palabras de pila que retira el `ret N` de `target` (0 si es cdecl o no se sabe): las llamadas indirectas elevadas las suman a ESP.
    fn callee_pop(&self, _target: u32) -> u32 {
        0
    }
    /// Como [`Host::call`], pero con el `esp` real del llamante elevado (la pila viva está por encima): el anfitrión puede colocar el marco del
    /// llamado justo debajo de los locales del llamante. Por defecto ignora `esp`.
    fn call_at(&mut self, _esp: u32, m: &mut dyn Mem, target: u32, ecx: u32, args: &[u32]) -> u32 {
        self.call(m, target, ecx, args)
    }
    /// `_ftol` (0x639C28) interceptado: `Some(eax)` sustituye la truncación en línea de ST0 (las pruebas con trazas que
    /// grabaron `_ftol` como llamada interceptada, sin tocar la pila x87). Por defecto `None`: truncación real.
    fn ftol_hook(&mut self, _m: &mut dyn Mem, _ecx: u32) -> Option<u32> {
        None
    }
    /// Llamada que devuelve un `float` en ST0 (dibujo, geometría). Por defecto, el EAX reinterpretado.
    fn call_f(&mut self, m: &mut dyn Mem, target: u32, ecx: u32, args: &[u32]) -> f64 {
        f64::from(f32::from_bits(self.call(m, target, ecx, args)))
    }
}

/// ECX de una llamada cdecl/stdcall: el original deja ahí un valor sin significado, que la prueba no compara.
pub const CDECL: u32 = u32::MAX;

/// Llamada a una función sin `this` (cdecl o stdcall).
pub fn cdecl(m: &mut dyn Mem, h: &mut dyn Host, target: u32, args: &[u32]) -> u32 {
    h.call(m, target, CDECL, args)
}

pub fn new(m: &mut dyn Mem, h: &mut dyn Host, size: u32) -> u32 {
    h.call(m, NEW, CDECL, &[size])
}

pub fn free(m: &mut dyn Mem, h: &mut dyn Host, p: u32) {
    h.call(m, FREE, CDECL, &[p]);
}

/// `memcpy` del CRT (`0x63A790`): copia hacia delante.
pub fn memcpy(m: &mut dyn Mem, dst: u32, src: u32, n: u32) {
    for i in 0..n {
        let b = m.r8(src.wrapping_add(i));
        m.w8(dst.wrapping_add(i), b);
    }
}

/// `memset` del CRT (`0x639F20`).
pub fn memset(m: &mut dyn Mem, dst: u32, value: u8, n: u32) {
    for i in 0..n {
        m.w8(dst.wrapping_add(i), value);
    }
}

pub fn read_vec(m: &mut dyn Mem, a: u32, n: u32) -> Vec<u8> {
    (0..n).map(|i| m.r8(a.wrapping_add(i))).collect()
}

pub fn write_slice(m: &mut dyn Mem, a: u32, data: &[u8]) {
    for (i, b) in data.iter().enumerate() {
        m.w8(a.wrapping_add(i as u32), *b);
    }
}

pub fn rf32(m: &mut dyn Mem, a: u32) -> f32 {
    f32::from_bits(m.r32(a))
}

pub fn wf32(m: &mut dyn Mem, a: u32, v: f32) {
    m.w32(a, v.to_bits());
}

/// `[[p] + off]` (32 bits).
pub fn at32(m: &mut dyn Mem, p: u32, off: u32) -> u32 {
    let b = m.r32(p);
    m.r32(b.wrapping_add(off))
}

/// `[[p] + off]` (16 bits).
pub fn at16(m: &mut dyn Mem, p: u32, off: u32) -> u16 {
    let b = m.r32(p);
    m.r16(b.wrapping_add(off))
}

/// `[[p] + off]` (8 bits).
pub fn at8(m: &mut dyn Mem, p: u32, off: u32) -> u8 {
    let b = m.r32(p);
    m.r8(b.wrapping_add(off))
}
