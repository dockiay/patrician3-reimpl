//! WEB-4: partidas en el navegador (`coordination/ARQUITECTURA_WEB.md`, «Partidas `.pat`»).
//!
//! El enlazador guarda y carga partidas por el rasgo [`p3_mvp::hostfs::HostFs`] bajo `<carpeta del juego>/save/...`
//! (`.pat`, `.pst`, `AUTO.pat`). En el Worker de la web no se puede esperar a OPFS (sus API de apertura son asíncronas y el
//! Worker pasa casi todo el tiempo bloqueado dentro del juego), así que [`SaveFs`] mantiene **toda la carpeta `save/` en
//! memoria**: lecturas y escrituras son síncronas, y cada cambio terminado (cerrar un archivo, escribir, renombrar, borrar) se
//! entrega a un [`CommitSink`] que lo manda al hilo principal, el cual lo guarda en OPFS (`js/p3-saves-main.js`). Al arrancar, el
//! hilo principal lee `save/` de OPFS y la copia a la memoria con [`SaveFs::preload`]. El resto de rutas (EXE, CPR, música...) va
//! al sistema de archivos que hubiera instalado antes (OPFS de solo lectura en la web, `StdFs` en las pruebas).

pub mod memfs;
pub mod savefs;
#[cfg(target_arch = "wasm32")]
pub mod wasm;

pub use savefs::{CommitSink, SaveFs};

/// Resumen de una partida `.pat` para la lista de la página.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SaveInfo {
    pub name: String,
    pub day: u8,
    pub month: u8,
    pub year: u16,
}

/// Comprueba que `bytes` es una partida `.pat` que el cargador entiende (la misma validación que `p3-save`).
pub fn inspect_pat(bytes: &[u8]) -> Result<SaveInfo, String> {
    let s = p3_save::Save::parse(bytes).map_err(|e| format!("{e:?}"))?;
    let (day, month, year) = s.date();
    Ok(SaveInfo { name: s.name(), day, month, year })
}
