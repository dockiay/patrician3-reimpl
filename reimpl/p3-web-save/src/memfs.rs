//! Árbol de archivos en memoria, sin distinguir mayúsculas (como NTFS), compartido entre manejadores.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct File {
    /// Nombre y ruta tal como se crearon (la capitalización se conserva al listar).
    pub path: PathBuf,
    pub data: Arc<Vec<u8>>,
    pub modified: u64,
}

#[derive(Default, Debug)]
pub struct Tree {
    pub files: BTreeMap<String, File>,
    pub dirs: BTreeMap<String, PathBuf>,
}

/// Clave de comparación: componentes en minúsculas unidos por `/`, sin `.` ni vacíos.
pub fn key_of(path: &Path) -> String {
    path.to_string_lossy()
        .replace('\\', "/")
        .split('/')
        .filter(|p| !p.is_empty() && *p != ".")
        .map(|p| p.to_ascii_lowercase())
        .collect::<Vec<_>>()
        .join("/")
}

pub fn parent_key(key: &str) -> &str {
    key.rsplit_once('/').map_or("", |(p, _)| p)
}

#[derive(Default, Clone)]
pub struct MemTree(pub Arc<Mutex<Tree>>);

impl MemTree {
    pub fn lock(&self) -> std::sync::MutexGuard<'_, Tree> {
        self.0.lock().unwrap_or_else(|e| e.into_inner())
    }
}
