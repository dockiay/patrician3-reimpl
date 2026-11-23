//! J08: interfaz del juego (marco de ventanas, controles, barras de desplazamiento, listas y fuentes) portada sobre la memoria del
//! original. Usa el mismo modelo que [`crate::j07`]: cada porte trabaja sobre [`Mem`] con las direcciones y disposiciones del proceso
//! original y delega en [`Host`] las llamadas a funciones que quedan fuera del porte (cadenas, asignador, métodos virtuales, SGL).
//! `oracle/x86_j08.py` ejecuta las funciones originales con esas llamadas interceptadas y `tests/x86_j08.rs` exige iguales EAX,
//! escrituras y secuencia de llamadas.

pub use crate::j07::{Host, Mem, Ret};

pub mod lift;
pub mod lift_ctl1;
pub mod lift_ctl2;
pub mod lift_ctl3;
pub mod lift_pan1;
pub mod lift_wk6;
pub mod lift_wk4;
pub mod lift_wk3;
pub mod lift_wk2;
pub mod lift_wk1;
pub mod lift_pan7;
pub mod lift_pan6;
pub mod lift_pan5;
pub mod lift_pan4;
pub mod lift_pan3;
pub mod lift_pan2;
pub mod lift_win2;
pub mod lift_misc2;
pub mod lift_rest1;
pub mod lift_rest2;
pub mod lift_rest3;
pub mod lift_rest4;
pub mod lift_sim1;
pub mod lift_sim2;
pub mod lift_sim3;
pub mod lift_sim4;
pub mod lift_sim5;
pub mod lift_sim6;
pub mod lift_sim7;
pub mod lift_sim8;
pub mod lift_sim9;
pub mod lift_sim10;
pub mod lift_sim11;
pub mod lift_sim12;
pub mod lift_sim13;
pub mod win1;

/// Ejecuta el porte de la función original `f` (ECX `ecx`, argumentos de pila `a`, pila de entrada `esp`). `None` si no hay porte.
pub fn run(f: u32, m: &mut dyn Mem, h: &mut dyn Host, ecx: u32, a: &[u32], esp: u32) -> Option<Ret> {
    // sim12/sim13 primero: sus portes de 0x605420, 0x534A40, 0x51AC60 y 0x54AAE0 (con las firmas de llamada corregidas) ocultan los de sim3/sim4/sim5
    lift_sim13::run(f, m, h, ecx, a, esp)
        .or_else(|| lift_sim12::run(f, m, h, ecx, a, esp))
        .or_else(|| win1::run(f, m, h, ecx, a, esp))
        .or_else(|| lift_ctl1::run(f, m, h, ecx, a, esp))
        .or_else(|| lift_ctl2::run(f, m, h, ecx, a, esp))
        .or_else(|| lift_ctl3::run(f, m, h, ecx, a, esp))
        .or_else(|| lift_win2::run(f, m, h, ecx, a, esp))
        .or_else(|| lift_pan1::run(f, m, h, ecx, a, esp))
        .or_else(|| lift_wk6::run(f, m, h, ecx, a, esp))
        .or_else(|| lift_wk4::run(f, m, h, ecx, a, esp))
        .or_else(|| lift_wk3::run(f, m, h, ecx, a, esp))
        .or_else(|| lift_wk2::run(f, m, h, ecx, a, esp))
        .or_else(|| lift_wk1::run(f, m, h, ecx, a, esp))
        .or_else(|| lift_pan7::run(f, m, h, ecx, a, esp))
        .or_else(|| lift_pan6::run(f, m, h, ecx, a, esp))
        .or_else(|| lift_pan5::run(f, m, h, ecx, a, esp))
        .or_else(|| lift_pan4::run(f, m, h, ecx, a, esp))
        .or_else(|| lift_pan3::run(f, m, h, ecx, a, esp))
        .or_else(|| lift_pan2::run(f, m, h, ecx, a, esp))
        .or_else(|| lift_misc2::run(f, m, h, ecx, a, esp))
        .or_else(|| lift_rest1::run(f, m, h, ecx, a, esp))
        .or_else(|| lift_rest2::run(f, m, h, ecx, a, esp))
        .or_else(|| lift_rest3::run(f, m, h, ecx, a, esp))
        .or_else(|| lift_rest4::run(f, m, h, ecx, a, esp))
        .or_else(|| lift_sim1::run(f, m, h, ecx, a, esp))
        .or_else(|| lift_sim2::run(f, m, h, ecx, a, esp))
        .or_else(|| lift_sim3::run(f, m, h, ecx, a, esp))
        .or_else(|| lift_sim4::run(f, m, h, ecx, a, esp))
        .or_else(|| lift_sim5::run(f, m, h, ecx, a, esp))
        .or_else(|| lift_sim6::run(f, m, h, ecx, a, esp))
        .or_else(|| lift_sim7::run(f, m, h, ecx, a, esp))
        .or_else(|| lift_sim8::run(f, m, h, ecx, a, esp))
        .or_else(|| lift_sim9::run(f, m, h, ecx, a, esp))
        .or_else(|| lift_sim10::run(f, m, h, ecx, a, esp))
        .or_else(|| lift_sim11::run(f, m, h, ecx, a, esp))
}
