//! Reloj monótono para los perfiles opcionales (`P3_PROF`, perfil por función).
//!
//! En `wasm32-unknown-unknown` `std::time::Instant::now()` aborta; allí se usa un reloj nulo (los perfiles salen en cero).
//! El comportamiento del juego no depende de este reloj: el tiempo del juego sale de `InputState.time_ms`.
#[cfg(not(target_arch = "wasm32"))]
pub use std::time::Instant;

#[cfg(target_arch = "wasm32")]
#[derive(Clone, Copy, Debug)]
pub struct Instant;

#[cfg(target_arch = "wasm32")]
impl Instant {
    pub fn now() -> Self {
        Instant
    }
    pub fn elapsed(&self) -> std::time::Duration {
        std::time::Duration::ZERO
    }
}
