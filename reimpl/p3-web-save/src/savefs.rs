//! [`SaveFs`]: carpeta `save/` en memoria con confirmación hacia fuera; el resto de rutas pasa al sistema base.

use crate::memfs::{key_of, parent_key, File, MemTree};
use p3_mvp::hostfs::{self, DirEntry, HostFile, HostFs, Meta, OpenMode};
use std::io::{self, Cursor, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Destino de los cambios ya terminados. `rel` es la ruta relativa a la carpeta del juego con `/` (`save/ein/AUTO.pat`);
/// `data` es `None` si el archivo se ha borrado. Debe volver rápido y sin bloquear (en la web: `postMessage`).
pub trait CommitSink: Send + Sync {
    fn commit(&self, rel: &str, data: Option<&[u8]>);
}

pub struct SaveFs {
    base: Arc<dyn HostFs>,
    root: PathBuf,
    tree: MemTree,
    sink: Arc<dyn CommitSink>,
    now: fn() -> u64,
}

impl SaveFs {
    /// `root` es la carpeta del juego tal como la ve el enlazador (`game_dir`); `base` atiende lo que no esté bajo `save/`.
    pub fn new(base: Arc<dyn HostFs>, root: impl Into<PathBuf>, sink: Arc<dyn CommitSink>, now: fn() -> u64) -> Arc<Self> {
        Arc::new(SaveFs { base, root: root.into(), tree: MemTree::default(), sink, now })
    }

    /// Lo crea sobre el sistema instalado ahora y lo instala como el actual.
    pub fn install(root: impl Into<PathBuf>, sink: Arc<dyn CommitSink>, now: fn() -> u64) -> Arc<Self> {
        let fs = Self::new(hostfs::current(), root, sink, now);
        hostfs::install(fs.clone());
        fs
    }

    /// Ruta relativa a `root` (con `/`) si cae bajo `save/`.
    fn rel(&self, path: &Path) -> Option<String> {
        let rest = match path.strip_prefix(&self.root) {
            Ok(r) => r,
            Err(_) if path.is_absolute() => return None,
            Err(_) => path,
        };
        let key = key_of(rest);
        (key == "save" || key.starts_with("save/")).then_some(key)
    }

    /// Copia en memoria un archivo leído de OPFS (no avisa al destino). Conserva la capitalización de `rel`.
    pub fn preload(&self, rel: &str, data: Vec<u8>, modified: u64) {
        let path = self.root.join(rel.trim_start_matches("./"));
        let key = key_of(Path::new(rel));
        let mut t = self.tree.lock();
        mkdirs(&mut t, &self.root, parent_key(&key), Path::new(rel));
        t.files.insert(key, File { path, data: Arc::new(data), modified });
    }

    /// Quita de la memoria un archivo borrado desde fuera (no avisa al destino).
    pub fn forget(&self, rel: &str) {
        self.tree.lock().files.remove(&key_of(Path::new(rel)));
    }

    /// Rutas relativas de todos los archivos de partida en memoria (para las pruebas y el panel).
    pub fn files(&self) -> Vec<(String, usize)> {
        self.tree.lock().files.values().map(|f| (f.path.strip_prefix(&self.root).unwrap_or(&f.path).to_string_lossy().replace('\\', "/"), f.data.len())).collect()
    }

    fn commit(&self, key: &str, data: Option<&[u8]>) {
        let rel = {
            let t = self.tree.lock();
            t.files.get(key).map(|f| rel_display(&self.root, &f.path))
        };
        // Un borrado ya no tiene entrada: se usa la clave (la copia en OPFS se busca sin distinguir mayúsculas).
        self.sink.commit(&rel.unwrap_or_else(|| key.to_string()), data);
    }
}

fn rel_display(root: &Path, p: &Path) -> String {
    p.strip_prefix(root).unwrap_or(p).to_string_lossy().replace('\\', "/")
}

/// Registra las carpetas de `key` (sus nombres se toman del original `orig`, que casa componente a componente).
fn mkdirs(t: &mut crate::memfs::Tree, root: &Path, key: &str, orig: &Path) {
    let parts: Vec<_> = orig.to_string_lossy().replace('\\', "/").split('/').filter(|p| !p.is_empty() && *p != ".").map(String::from).collect();
    let mut acc = String::new();
    let mut path = root.to_path_buf();
    for (i, part) in key.split('/').filter(|p| !p.is_empty()).enumerate() {
        if !acc.is_empty() {
            acc.push('/');
        }
        acc.push_str(part);
        path.push(parts.get(i).map_or(part, String::as_str));
        t.dirs.entry(acc.clone()).or_insert_with(|| path.clone());
    }
}

fn not_found() -> io::Error {
    io::Error::new(io::ErrorKind::NotFound, "no existe")
}

impl HostFs for SaveFs {
    fn read(&self, path: &Path) -> io::Result<Vec<u8>> {
        let Some(key) = self.rel(path) else { return self.base.read(path) };
        self.tree.lock().files.get(&key).map(|f| f.data.to_vec()).ok_or_else(not_found)
    }

    fn read_at(&self, path: &Path, offset: u64, buf: &mut [u8]) -> io::Result<usize> {
        let Some(key) = self.rel(path) else { return self.base.read_at(path, offset, buf) };
        let t = self.tree.lock();
        let f = t.files.get(&key).ok_or_else(not_found)?;
        let start = (offset as usize).min(f.data.len());
        let n = buf.len().min(f.data.len() - start);
        buf[..n].copy_from_slice(&f.data[start..start + n]);
        Ok(n)
    }

    fn write(&self, path: &Path, data: &[u8]) -> io::Result<()> {
        let Some(key) = self.rel(path) else { return self.base.write(path, data) };
        self.put(&key, path, data.to_vec())
    }

    fn rename(&self, from: &Path, to: &Path) -> io::Result<()> {
        let (Some(a), Some(b)) = (self.rel(from), self.rel(to)) else { return self.base.rename(from, to) };
        let (data, old) = {
            let mut t = self.tree.lock();
            let f = t.files.remove(&a).ok_or_else(not_found)?;
            (f.data, rel_display(&self.root, &f.path))
        };
        self.sink.commit(&old, None);
        self.put(&b, to, data.to_vec())
    }

    fn remove_file(&self, path: &Path) -> io::Result<()> {
        let Some(key) = self.rel(path) else { return self.base.remove_file(path) };
        let rel = {
            let mut t = self.tree.lock();
            t.files.remove(&key).map(|f| rel_display(&self.root, &f.path)).ok_or_else(not_found)?
        };
        self.sink.commit(&rel, None);
        Ok(())
    }

    fn create_dir_all(&self, path: &Path) -> io::Result<()> {
        let Some(key) = self.rel(path) else { return self.base.create_dir_all(path) };
        let rel = path.strip_prefix(&self.root).unwrap_or(path).to_path_buf();
        mkdirs(&mut self.tree.lock(), &self.root, &key, &rel);
        Ok(())
    }

    fn metadata(&self, path: &Path) -> io::Result<Meta> {
        let Some(key) = self.rel(path) else { return self.base.metadata(path) };
        let t = self.tree.lock();
        if let Some(f) = t.files.get(&key) {
            return Ok(Meta { len: f.data.len() as u64, is_file: true, is_dir: false, modified_secs: Some(f.modified) });
        }
        if t.dirs.contains_key(&key) || key == "save" {
            return Ok(Meta { len: 0, is_file: false, is_dir: true, modified_secs: None });
        }
        Err(not_found())
    }

    fn list(&self, dir: &Path) -> io::Result<Vec<DirEntry>> {
        let Some(key) = self.rel(dir) else {
            // La carpeta del juego mezcla lo que hay en el sistema base con la carpeta `save`, que vive en memoria.
            let mut v = self.base.list(dir)?;
            if key_of(dir.strip_prefix(&self.root).unwrap_or(dir)).is_empty() && dir.strip_prefix(&self.root).is_ok() {
                let t = self.tree.lock();
                if let Some(p) = t.dirs.get("save") {
                    if !v.iter().any(|e| e.name.eq_ignore_ascii_case("save")) {
                        v.push(DirEntry { name: "save".into(), path: p.clone(), is_file: false, is_dir: true });
                    }
                }
            }
            return Ok(v);
        };
        let t = self.tree.lock();
        if !t.dirs.contains_key(&key) && key != "save" {
            return Err(not_found());
        }
        let mut out = Vec::new();
        for (k, f) in &t.files {
            if parent_key(k) == key {
                out.push(DirEntry { name: f.path.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default(), path: f.path.clone(), is_file: true, is_dir: false });
            }
        }
        for (k, p) in &t.dirs {
            if parent_key(k) == key {
                out.push(DirEntry { name: p.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default(), path: p.clone(), is_file: false, is_dir: true });
            }
        }
        Ok(out)
    }

    fn open(&self, path: &Path, mode: OpenMode) -> io::Result<Box<dyn HostFile>> {
        let Some(key) = self.rel(path) else { return self.base.open(path, mode) };
        let existing = self.tree.lock().files.get(&key).map(|f| f.data.to_vec());
        let (buf, dirty) = match existing {
            Some(_) if mode.truncate && mode.write => (Vec::new(), true),
            Some(d) => (d, false),
            None if mode.create && mode.write => (Vec::new(), true),
            None => return Err(not_found()),
        };
        Ok(Box::new(Handle { fs: self.me(), key, path: path.to_path_buf(), cur: Cursor::new(buf), dirty, writable: mode.write }))
    }
}

impl SaveFs {
    fn put(&self, key: &str, path: &Path, data: Vec<u8>) -> io::Result<()> {
        {
            let mut t = self.tree.lock();
            let rel = path.strip_prefix(&self.root).unwrap_or(path);
            mkdirs(&mut t, &self.root, parent_key(key), rel);
            let p = if path.starts_with(&self.root) { path.to_path_buf() } else { self.root.join(rel) };
            t.files.insert(key.to_string(), File { path: p, data: Arc::new(data.clone()), modified: (self.now)() });
        }
        self.commit(key, Some(&data));
        Ok(())
    }

    fn me(&self) -> Inner {
        Inner { tree: self.tree.clone(), sink: self.sink.clone(), root: self.root.clone(), now: self.now }
    }
}

/// Lo que un manejador abierto necesita para confirmar al cerrarse (no retiene el `SaveFs`).
struct Inner {
    tree: MemTree,
    sink: Arc<dyn CommitSink>,
    root: PathBuf,
    now: fn() -> u64,
}

struct Handle {
    fs: Inner,
    key: String,
    path: PathBuf,
    cur: Cursor<Vec<u8>>,
    dirty: bool,
    writable: bool,
}

impl Handle {
    /// Publica el contenido actual en el árbol y avisa al destino.
    fn flush_to_tree(&mut self) {
        if !self.dirty {
            return;
        }
        self.dirty = false;
        let data = self.cur.get_ref().clone();
        let rel = self.path.strip_prefix(&self.fs.root).unwrap_or(&self.path).to_path_buf();
        {
            let mut t = self.fs.tree.lock();
            mkdirs(&mut t, &self.fs.root, parent_key(&self.key), &rel);
            let p = if self.path.starts_with(&self.fs.root) { self.path.clone() } else { self.fs.root.join(&rel) };
            t.files.insert(self.key.clone(), File { path: p, data: Arc::new(data.clone()), modified: (self.fs.now)() });
        }
        self.fs.sink.commit(&rel.to_string_lossy().replace('\\', "/"), Some(&data));
    }
}

impl Read for Handle {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.cur.read(buf)
    }
}

impl Write for Handle {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if !self.writable {
            return Err(io::Error::new(io::ErrorKind::PermissionDenied, "abierto solo para leer"));
        }
        self.dirty = true;
        self.cur.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.flush_to_tree();
        Ok(())
    }
}

impl Seek for Handle {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.cur.seek(pos)
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        self.flush_to_tree();
    }
}
