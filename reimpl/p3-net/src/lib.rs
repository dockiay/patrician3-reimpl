//! Red de Patrician III (M02/M03).
//!
//! - [`wire`], [`queue`], [`transfer`], [`sync`] y [`session`]: el protocolo del original
//!   (PRE-SPEC-NET), portado con contraste x86 donde se indica en cada módulo (M03).
//! - [`lockstep`]: multijugador entre clientes nuevos con el mismo modelo de autoridad
//!   (el anfitrión ordena, todos ejecutan el mismo flujo) y detección de divergencias por
//!   hash del estado completo (M02).
//! - [`relay`] (WEB-7): transporte por repetidor WebSocket para navegadores; sin E/S propia.
//!   Los enlaces y el servidor nativos están en `ws` (característica `ws`).
//! - [`transport`]: red en memoria con latencia y cortes, y TCP con el marco de la DLL.
pub mod wire;
pub mod queue;
pub mod transfer;
pub mod sync;
pub mod session;
pub mod lockstep;
pub mod transport;
pub mod relay;
#[cfg(all(feature = "ws", not(target_arch = "wasm32")))]
pub mod ws;
