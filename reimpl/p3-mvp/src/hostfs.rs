//! Acceso a archivos del anfitrión (WEB-2 de `coordination/ARQUITECTURA_WEB.md`).
//!
//! Todo lo que el enlazador lee o escribe fuera del CPR (partidas `save/ein`, música suelta, `video.cfg`, texturas sueltas, el
//! borrado de partidas) pasa por el rasgo [`HostFs`]. En escritorio la implementación es [`StdFs`] (`std::fs`, sin cambio de
//! comportamiento); en la web se instalará otra sobre OPFS con [`install`]. Las rutas son las de siempre (`PathBuf` bajo la
//! carpeta del juego); la implementación decide cómo interpretarlas.

use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

/// Entrada de un directorio.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DirEntry {
    /// Nombre sin ruta, tal como está almacenado.
    pub name: String,
    pub path: PathBuf,
    pub is_file: bool,
    pub is_dir: bool,
}

/// Lo que el enlazador pregunta de un archivo.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Meta {
    pub len: u64,
    pub is_file: bool,
    pub is_dir: bool,
    /// Última modificación en segundos desde 1970 (`None` si el anfitrión no la da). No se usa `SystemTime` porque `now()` aborta en
    /// `wasm32-unknown-unknown`.
    pub modified_secs: Option<u64>,
}

/// Modo de apertura (`CFile::Open` de MFC): lectura, escritura, crear y truncar.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct OpenMode {
    pub read: bool,
    pub write: bool,
    pub create: bool,
    pub truncate: bool,
}

/// Archivo abierto para leer y escribir secuencialmente.
pub trait HostFile: Read + Write + Seek + Send {}
impl<T: Read + Write + Seek + Send> HostFile for T {}

/// Sistema de archivos del anfitrión. Todos los métodos son bloqueantes, como las llamadas que sustituyen.
pub trait HostFs: Send + Sync {
    fn read(&self, path: &Path) -> io::Result<Vec<u8>>;
    /// Lee hasta `buf.len()` bytes desde `offset`; devuelve cuántos leyó (0 al final).
    fn read_at(&self, path: &Path, offset: u64, buf: &mut [u8]) -> io::Result<usize>;
    fn write(&self, path: &Path, data: &[u8]) -> io::Result<()>;
    fn rename(&self, from: &Path, to: &Path) -> io::Result<()>;
    fn remove_file(&self, path: &Path) -> io::Result<()>;
    fn create_dir_all(&self, path: &Path) -> io::Result<()>;
    fn metadata(&self, path: &Path) -> io::Result<Meta>;
    /// Entradas de un directorio, sin orden garantizado.
    fn list(&self, dir: &Path) -> io::Result<Vec<DirEntry>>;
    fn open(&self, path: &Path, mode: OpenMode) -> io::Result<Box<dyn HostFile>>;

    fn read_to_string(&self, path: &Path) -> io::Result<String> {
        String::from_utf8(self.read(path)?).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    fn exists(&self, path: &Path) -> bool {
        self.metadata(path).is_ok()
    }

    fn is_file(&self, path: &Path) -> bool {
        self.metadata(path).is_ok_and(|m| m.is_file)
    }

    /// Entrada de `dir` cuyo nombre es `name` sin distinguir mayúsculas ASCII (como NTFS; las instalaciones copiadas a otros
    /// sistemas cambian la capitalización de `save/ein`).
    fn find_ci(&self, dir: &Path, name: &str) -> Option<DirEntry> {
        self.list(dir).ok()?.into_iter().find(|e| e.name.eq_ignore_ascii_case(name))
    }
}

/// `std::fs` (escritorio).
#[derive(Clone, Copy, Debug, Default)]
pub struct StdFs;

fn meta_of(m: &std::fs::Metadata) -> Meta {
    Meta {
        len: m.len(),
        is_file: m.is_file(),
        is_dir: m.is_dir(),
        modified_secs: m.modified().ok().and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok()).map(|d| d.as_secs()),
    }
}

impl HostFs for StdFs {
    fn read(&self, path: &Path) -> io::Result<Vec<u8>> {
        std::fs::read(path)
    }

    fn read_at(&self, path: &Path, offset: u64, buf: &mut [u8]) -> io::Result<usize> {
        let mut f = std::fs::File::open(path)?;
        f.seek(SeekFrom::Start(offset))?;
        let mut done = 0;
        while done < buf.len() {
            match f.read(&mut buf[done..])? {
                0 => break,
                n => done += n,
            }
        }
        Ok(done)
    }

    fn write(&self, path: &Path, data: &[u8]) -> io::Result<()> {
        std::fs::write(path, data)
    }

    fn rename(&self, from: &Path, to: &Path) -> io::Result<()> {
        std::fs::rename(from, to)
    }

    fn remove_file(&self, path: &Path) -> io::Result<()> {
        std::fs::remove_file(path)
    }

    fn create_dir_all(&self, path: &Path) -> io::Result<()> {
        std::fs::create_dir_all(path)
    }

    fn metadata(&self, path: &Path) -> io::Result<Meta> {
        std::fs::metadata(path).map(|m| meta_of(&m))
    }

    fn list(&self, dir: &Path) -> io::Result<Vec<DirEntry>> {
        Ok(std::fs::read_dir(dir)?
            .flatten()
            .map(|e| {
                let path = e.path();
                // `Path::is_file` sigue enlaces simbólicos, como el código que esto sustituye.
                DirEntry { name: e.file_name().to_string_lossy().into_owned(), is_file: path.is_file(), is_dir: path.is_dir(), path }
            })
            .collect())
    }

    fn open(&self, path: &Path, mode: OpenMode) -> io::Result<Box<dyn HostFile>> {
        let mut o = std::fs::OpenOptions::new();
        o.read(mode.read).write(mode.write).create(mode.create).truncate(mode.truncate);
        Ok(Box::new(o.open(path)?))
    }
}

static CURRENT: RwLock<Option<Arc<dyn HostFs>>> = RwLock::new(None);

/// Sistema de archivos en uso ([`StdFs`] mientras nadie instale otro).
pub fn current() -> Arc<dyn HostFs> {
    CURRENT.read().ok().and_then(|g| g.clone()).unwrap_or_else(|| Arc::new(StdFs))
}

/// Instala el sistema de archivos del anfitrión (la web instala el de OPFS antes de arrancar el enlazador).
pub fn install(fs: Arc<dyn HostFs>) {
    if let Ok(mut g) = CURRENT.write() {
        *g = Some(fs);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("p3-hostfs-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    #[test]
    fn std_fs_round_trip() {
        let root = scratch("rt");
        let fs = StdFs;
        let p = root.join("Save/Ein");
        fs.create_dir_all(&p).unwrap();
        fs.write(&p.join("AUTO.pat"), b"0123456789").unwrap();
        assert_eq!(fs.read(&p.join("AUTO.pat")).unwrap(), b"0123456789");
        let mut b = [0u8; 4];
        assert_eq!(fs.read_at(&p.join("AUTO.pat"), 8, &mut b).unwrap(), 2);
        assert_eq!(&b[..2], b"89");
        assert_eq!(fs.find_ci(&root, "save").unwrap().name, "Save");
        assert!(fs.find_ci(&root, "nada").is_none());
        let m = fs.metadata(&p.join("AUTO.pat")).unwrap();
        assert!((m.len, m.is_file, m.modified_secs.is_some()) == (10, true, true));
        fs.rename(&p.join("AUTO.pat"), &p.join("B.pat")).unwrap();
        assert!(!fs.exists(&p.join("AUTO.pat")) && fs.is_file(&p.join("B.pat")));
        let mut f = fs.open(&p.join("C.pat"), OpenMode { read: true, write: true, create: true, truncate: true }).unwrap();
        f.write_all(b"xyz").unwrap();
        drop(f);
        assert_eq!(fs.read_to_string(&p.join("C.pat")).unwrap(), "xyz");
        fs.remove_file(&p.join("C.pat")).unwrap();
        assert!(fs.remove_file(&p.join("C.pat")).is_err());
        let names: Vec<_> = { let mut v: Vec<_> = fs.list(&p).unwrap().into_iter().map(|e| e.name).collect(); v.sort(); v };
        assert_eq!(names, ["B.pat"]);
        let _ = std::fs::remove_dir_all(root);
    }
}
