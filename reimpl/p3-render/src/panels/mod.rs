//! Barras y paneles de la interfaz: tablas, barras de desplazamiento, listas y menús laterales portados sobre la memoria del original.
//!
//! Mismo modelo que [`crate::j07`]: cada porte lee y escribe la memoria con las direcciones y disposiciones del proceso original y delega en
//! [`Host`] las llamadas que quedan fuera (asignador, dibujo SGL, funciones de ventana). `oracle/x86_panels.py` ejecuta las funciones
//! originales con esas llamadas interceptadas y `tests/x86_panels.rs` exige iguales EAX, escrituras y secuencia de llamadas.

pub use crate::j07::{Host, Mem, Ret};

pub mod gamescreen;
pub mod listbox;
pub mod scrollbar;
pub mod shiplist;
pub mod stubs;
pub mod sidemenu;
pub mod table;

/// Ejecuta el porte de la función original `f`. `None` si no hay porte.
pub fn run(f: u32, m: &mut dyn Mem, h: &mut dyn Host, ecx: u32, a: &[u32], esp: u32) -> Option<Ret> {
    table::run(f, m, h, ecx, a, esp)
        .or_else(|| scrollbar::run(f, m, h, ecx, a, esp))
        .or_else(|| listbox::run(f, m, h, ecx, a, esp))
        .or_else(|| sidemenu::run(f, m, h, ecx, a, esp))
        .or_else(|| gamescreen::run(f, m, h, ecx, a, esp))
        .or_else(|| shiplist::run(f, m, h, ecx, a, esp))
        .or_else(|| stubs::run(f, m, h, ecx, a, esp))
}

/// Llama a `f`: si está portado se ejecuta el porte (`esp` es la pila de entrada del llamado, con la dirección de retorno en `esp`);
/// si no, el anfitrión ejecuta la función original.
pub fn call(m: &mut dyn Mem, h: &mut dyn Host, f: u32, ecx: u32, args: &[u32], esp: u32) -> u32 {
    match run(f, m, h, ecx, args, esp) {
        Some(Ret::U8(v)) => u32::from(v),
        Some(Ret::U16(v)) => u32::from(v),
        Some(Ret::U32(v)) => v,
        Some(Ret::None) => 0,
        None => h.call(m, f, ecx, args),
    }
}

/// `_ftol` (`0x639C28`): trunca hacia cero; fuera de rango o no finito devuelve el entero indefinido.
pub fn ftol(v: f32) -> u32 {
    if v.is_finite() && v > -2147483649.0 && v < 2147483648.0 {
        v as i32 as u32
    } else {
        0x8000_0000
    }
}
