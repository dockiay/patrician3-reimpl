//! Cliente nativo del juego: ventana de Windows/Linux (winit + softbuffer) a 1024x768, en ventana o a pantalla completa, con el
//! bucle de fotogramas a 60 por segundo, ratón y teclado traducidos a mensajes Win32 y el dispositivo de pantalla de la capa SGL
//! portada. Dos usos:
//! - [`original::attach`]: conecta la ventana al enlazador (`p3_mvp::ui_rt::Runtime`) por el gancho de `sgl_SwapBuffer`; el bucle
//!   sigue siendo el del código original (`screen_list_run_frame` 0x4B8A40). Ver `reimpl/UI_PANTALLA_COMPLETA.md`.
//! - [`view::View`]: vista de prueba de una partida (ciudad y carta náutica desplazables).
pub mod audio;
pub mod device;
pub mod idioma;
pub mod keys;
pub mod original;
pub mod precarga;
pub mod view;
pub mod window;
mod scale;
