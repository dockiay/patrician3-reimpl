//! Minimapa y vistas de navegación de la interfaz: `TownMiniMap` (0x59C200–0x59D8xx), `SeaMapWindow` (0x46CE70–0x46F6E0) y el pulso de anillo
//! de los objetos de tipo 29.
//!
//! Misma forma que `p3_render::j07`: cada porte trabaja sobre [`Mem`] con las direcciones y la disposición del proceso original y delega en
//! [`Host`] las llamadas que quedan fuera (SGL, clase base de la ventana, asignador). `oracle/x86_minimap.py` ejecuta las funciones del
//! `Patrician3.exe` inglés con esas llamadas interceptadas y `tests/x86_minimap.rs` exige iguales EAX, escrituras y llamadas.

pub use crate::j07::{cdecl, free, memcpy, memset, new, vcall, Host, Mem, Ret, CDECL};

pub mod ring;
pub mod sea;
pub mod town;

/// Ejecuta el porte de la función original `f` (ECX `ecx`, argumentos de pila `a`, ESP de entrada `esp`). `None` si no hay porte.
pub fn run(f: u32, m: &mut dyn Mem, h: &mut dyn Host, ecx: u32, a: &[u32], esp: u32) -> Option<Ret> {
    town::run(f, m, h, ecx, a, esp).or_else(|| sea::run(f, m, h, ecx, a, esp))
        .or_else(|| ring::run(f, m, h, ecx, a, esp))
}
