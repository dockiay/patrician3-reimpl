//! Puente con JavaScript (`js/p3-saves-worker.js`) en `wasm32-unknown-unknown`, sin `wasm-bindgen`: importaciones y
//! exportaciones `extern "C"` con punteros a la memoria lineal.
//!
//! Importa (las provee el Worker): `p3_save_commit(ruta_ptr, ruta_len, datos_ptr, datos_len, tipo)` con tipo 1 = escribir,
//! 2 = borrar, y `p3_save_now_secs() -> f64`. Exporta: `p3_save_alloc`, `p3_save_free`, `p3_save_install`, `p3_save_preload`,
//! `p3_save_forget` y `p3_save_inspect`.

use crate::{CommitSink, SaveFs};
use p3_mvp::hostfs::HostFs;
use std::sync::{Arc, OnceLock};

#[link(wasm_import_module = "env")]
extern "C" {
    fn p3_save_commit(path: *const u8, path_len: usize, data: *const u8, data_len: usize, kind: u32);
    fn p3_save_now_secs() -> f64;
}

struct JsSink;

impl CommitSink for JsSink {
    fn commit(&self, rel: &str, data: Option<&[u8]>) {
        let d = data.unwrap_or(&[]);
        // SAFETY: los punteros son de trozos vivos durante la llamada; JS copia lo que necesita antes de volver.
        unsafe { p3_save_commit(rel.as_ptr(), rel.len(), d.as_ptr(), d.len(), if data.is_some() { 1 } else { 2 }) }
    }
}

fn now() -> u64 {
    // SAFETY: importación sin efectos.
    unsafe { p3_save_now_secs() as u64 }
}

static FS: OnceLock<Arc<SaveFs>> = OnceLock::new();

fn text<'a>(ptr: *const u8, len: usize) -> &'a str {
    // SAFETY: JS escribe `len` bytes UTF-8 en un trozo de `p3_save_alloc`.
    std::str::from_utf8(unsafe { std::slice::from_raw_parts(ptr, len) }).unwrap_or("")
}

#[cfg_attr(not(feature = "embedded"), no_mangle)]
pub extern "C" fn p3_save_alloc(len: usize) -> *mut u8 {
    let mut v = Vec::<u8>::with_capacity(len.max(1));
    let p = v.as_mut_ptr();
    std::mem::forget(v);
    p
}

#[cfg_attr(not(feature = "embedded"), no_mangle)]
pub extern "C" fn p3_save_free(ptr: *mut u8, cap: usize) {
    // SAFETY: `ptr` y `cap` vienen de `p3_save_alloc(cap)`.
    drop(unsafe { Vec::from_raw_parts(ptr, 0, cap.max(1)) });
}

/// Instala `SaveFs` como sistema de archivos actual para la carpeta del juego `root`. Hay que llamarla **antes** de arrancar el
/// enlazador y después de instalar el sistema base de solo lectura.
#[cfg_attr(not(feature = "embedded"), no_mangle)]
pub extern "C" fn p3_save_install(root_ptr: *const u8, root_len: usize) {
    FS.get_or_init(|| SaveFs::install(text(root_ptr, root_len), Arc::new(JsSink), now));
}

/// Copia a memoria un archivo que JS acaba de leer de OPFS o de importar. Con `validate != 0` rechaza (devuelve 0) lo que el
/// cargador no entienda; los `.pat` se validan, el resto se acepta.
#[cfg_attr(not(feature = "embedded"), no_mangle)]
pub extern "C" fn p3_save_preload(path_ptr: *const u8, path_len: usize, data_ptr: *const u8, data_len: usize, modified: f64, validate: u32) -> u32 {
    let Some(fs) = FS.get() else { return 0 };
    let path = text(path_ptr, path_len);
    // SAFETY: igual que `text`.
    let data = unsafe { std::slice::from_raw_parts(data_ptr, data_len) }.to_vec();
    if validate != 0 && path.to_ascii_lowercase().ends_with(".pat") && crate::inspect_pat(&data).is_err() {
        return 0;
    }
    fs.preload(path, data, modified as u64);
    1
}

#[cfg_attr(not(feature = "embedded"), no_mangle)]
pub extern "C" fn p3_save_forget(path_ptr: *const u8, path_len: usize) {
    if let Some(fs) = FS.get() {
        fs.forget(text(path_ptr, path_len));
    }
}

/// 1 si `bytes` es una partida `.pat` válida, 0 si no (para validar importaciones antes de guardarlas).
#[cfg_attr(not(feature = "embedded"), no_mangle)]
pub extern "C" fn p3_save_inspect(data_ptr: *const u8, data_len: usize) -> u32 {
    // SAFETY: igual que `text`.
    u32::from(crate::inspect_pat(unsafe { std::slice::from_raw_parts(data_ptr, data_len) }).is_ok())
}

/// Gancho de pruebas y herramientas: escribe `data` en `path` a través de `HostFs` (lo mismo que hace el guardado del juego).
#[cfg_attr(not(feature = "embedded"), no_mangle)]
pub extern "C" fn p3_save_hostfs_write(path_ptr: *const u8, path_len: usize, data_ptr: *const u8, data_len: usize) -> u32 {
    // SAFETY: igual que `text`.
    let data = unsafe { std::slice::from_raw_parts(data_ptr, data_len) };
    u32::from(p3_mvp::hostfs::current().write(std::path::Path::new(text(path_ptr, path_len)), data).is_ok())
}

/// Gancho de pruebas: tamaño de `path` visto por `HostFs`, o -1 si no existe.
#[cfg_attr(not(feature = "embedded"), no_mangle)]
pub extern "C" fn p3_save_hostfs_len(path_ptr: *const u8, path_len: usize) -> f64 {
    p3_mvp::hostfs::current().metadata(std::path::Path::new(text(path_ptr, path_len))).map_or(-1.0, |m| m.len as f64)
}
