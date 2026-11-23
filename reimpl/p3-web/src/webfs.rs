//! `HostFs` de solo lectura sobre los archivos del jugador que el Worker tiene abiertos en OPFS (EXE, CPR, música...). Lo que
//! se lee sale de OPFS **por tramos** (`FileSource::read_at`, un acceso síncrono del Worker), así que el CPR no entra entero en la
//! memoria de WASM. Las escrituras fuera de `save/` (que atiende `p3-web-save::SaveFs`, encima de este) van a una copia en memoria
//! que no persiste (p. ej. `video-mode.txt`): el navegador no deja abrir archivos nuevos de forma síncrona.
use p3_assets::cpr::RangeRead;
use p3_mvp::hostfs::{DirEntry, HostFile, HostFs, Meta, OpenMode};
use std::collections::BTreeMap;
use std::io::{self, Cursor, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

/// Lo que el anfitrión da de un archivo abierto: lectura posicional (en la web, `FileSystemSyncAccessHandle.read`).
pub trait FileSource: Send + Sync {
    /// Lee hasta `buf.len()` bytes desde `offset`; devuelve cuántos leyó (0 al final).
    fn read_at(&self, id: u32, offset: u64, buf: &mut [u8]) -> io::Result<usize>;
}

type Overlay = Arc<Mutex<BTreeMap<String, (String, Arc<Vec<u8>>)>>>;

#[derive(Clone)]
struct Item {
    id: u32,
    /// Ruta relativa a la carpeta del juego, con `/` y la capitalización original.
    rel: String,
    len: u64,
}

pub struct WebFs {
    root: PathBuf,
    src: Arc<dyn FileSource>,
    files: Mutex<BTreeMap<String, Item>>,
    /// Escrituras fuera de `save/`: no persisten.
    overlay: Overlay,
}

fn key(rel: &str) -> String {
    rel.replace('\\', "/").split('/').filter(|p| !p.is_empty() && *p != ".").map(|p| p.to_ascii_lowercase()).collect::<Vec<_>>().join("/")
}

fn lock<T>(m: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    m.lock().unwrap_or_else(|e| e.into_inner())
}

fn not_found() -> io::Error {
    io::Error::new(io::ErrorKind::NotFound, "no existe")
}

impl WebFs {
    pub fn new(root: impl Into<PathBuf>, src: Arc<dyn FileSource>) -> Arc<Self> {
        Arc::new(WebFs { root: root.into(), src, files: Mutex::default(), overlay: Overlay::default() })
    }

    /// Registra un archivo abierto por el anfitrión.
    pub fn add(&self, id: u32, rel: &str, len: u64) {
        lock(&self.files).insert(key(rel), Item { id, rel: rel.replace('\\', "/"), len });
    }

    fn rel_key(&self, path: &Path) -> String {
        let rest = path.strip_prefix(&self.root).unwrap_or(path);
        key(&rest.to_string_lossy())
    }

    fn item(&self, path: &Path) -> Option<Item> {
        lock(&self.files).get(&self.rel_key(path)).cloned()
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    /// El archivo `path` como lector por tramos (para el CPR).
    pub fn source(&self, path: &Path) -> Option<SourceRange> {
        self.item(path).map(|it| SourceRange { src: self.src.clone(), id: it.id, len: it.len })
    }

    fn abs(&self, rel: &str) -> PathBuf {
        self.root.join(rel)
    }

    fn is_dir(&self, k: &str) -> bool {
        if k.is_empty() {
            return true;
        }
        let pre = format!("{k}/");
        lock(&self.files).keys().any(|f| f.starts_with(&pre)) || lock(&self.overlay).keys().any(|f| f.starts_with(&pre))
    }
}

impl HostFs for WebFs {
    fn read(&self, path: &Path) -> io::Result<Vec<u8>> {
        if let Some((_, d)) = lock(&self.overlay).get(&self.rel_key(path)) {
            return Ok(d.to_vec());
        }
        let it = self.item(path).ok_or_else(not_found)?;
        let mut v = vec![0u8; it.len as usize];
        let mut done = 0;
        while done < v.len() {
            match self.src.read_at(it.id, done as u64, &mut v[done..])? {
                0 => return Err(io::ErrorKind::UnexpectedEof.into()),
                n => done += n,
            }
        }
        Ok(v)
    }

    fn read_at(&self, path: &Path, offset: u64, buf: &mut [u8]) -> io::Result<usize> {
        if let Some((_, d)) = lock(&self.overlay).get(&self.rel_key(path)) {
            let start = (offset as usize).min(d.len());
            let n = buf.len().min(d.len() - start);
            buf[..n].copy_from_slice(&d[start..start + n]);
            return Ok(n);
        }
        let it = self.item(path).ok_or_else(not_found)?;
        if offset >= it.len {
            return Ok(0);
        }
        let want = buf.len().min((it.len - offset) as usize);
        let mut done = 0;
        while done < want {
            match self.src.read_at(it.id, offset + done as u64, &mut buf[done..want])? {
                0 => break,
                n => done += n,
            }
        }
        Ok(done)
    }

    fn write(&self, path: &Path, data: &[u8]) -> io::Result<()> {
        let rel = path.strip_prefix(&self.root).unwrap_or(path).to_string_lossy().replace('\\', "/");
        lock(&self.overlay).insert(key(&rel), (rel, Arc::new(data.to_vec())));
        Ok(())
    }

    fn rename(&self, from: &Path, to: &Path) -> io::Result<()> {
        let data = self.read(from)?;
        self.write(to, &data)?;
        self.remove_file(from)
    }

    fn remove_file(&self, path: &Path) -> io::Result<()> {
        lock(&self.overlay).remove(&self.rel_key(path)).map(|_| ()).ok_or_else(|| io::Error::new(io::ErrorKind::PermissionDenied, "archivo de solo lectura"))
    }

    fn create_dir_all(&self, _path: &Path) -> io::Result<()> {
        Ok(())
    }

    fn metadata(&self, path: &Path) -> io::Result<Meta> {
        let k = self.rel_key(path);
        if let Some((_, d)) = lock(&self.overlay).get(&k) {
            return Ok(Meta { len: d.len() as u64, is_file: true, is_dir: false, modified_secs: None });
        }
        if let Some(it) = lock(&self.files).get(&k) {
            return Ok(Meta { len: it.len, is_file: true, is_dir: false, modified_secs: None });
        }
        if self.is_dir(&k) {
            return Ok(Meta { len: 0, is_file: false, is_dir: true, modified_secs: None });
        }
        Err(not_found())
    }

    fn list(&self, dir: &Path) -> io::Result<Vec<DirEntry>> {
        let k = self.rel_key(dir);
        if !self.is_dir(&k) {
            return Err(not_found());
        }
        let pre = if k.is_empty() { String::new() } else { format!("{k}/") };
        let mut out: BTreeMap<String, DirEntry> = BTreeMap::new();
        let depth = pre.matches('/').count();
        let mut add = |fkey: &str, rel: &str| {
            let Some(rest) = fkey.strip_prefix(&pre) else { return };
            // Nombre con la capitalización original: el componente de `rel` que corresponde.
            let orig: Vec<&str> = rel.split('/').filter(|p| !p.is_empty()).collect();
            let Some(name) = orig.get(depth).map(|s| s.to_string()) else { return };
            let is_file = !rest.contains('/');
            let path = self.abs(&orig[..=depth].join("/"));
            out.entry(rest.split('/').next().unwrap_or(rest).to_string()).or_insert(DirEntry { name, path, is_file, is_dir: !is_file });
        };
        for (fk, it) in lock(&self.files).iter() {
            add(fk, &it.rel);
        }
        for (fk, (rel, _)) in lock(&self.overlay).iter() {
            add(fk, rel);
        }
        Ok(out.into_values().collect())
    }

    fn open(&self, path: &Path, mode: OpenMode) -> io::Result<Box<dyn HostFile>> {
        if mode.write {
            let existing = if mode.truncate { None } else { self.read(path).ok() };
            if existing.is_none() && !mode.create && !mode.truncate {
                return Err(not_found());
            }
            let rel = path.strip_prefix(&self.root).unwrap_or(path).to_string_lossy().replace('\\', "/");
            return Ok(Box::new(Writer { fs: WebFsRef { overlay: self.overlay.clone(), key: key(&rel), rel }, cur: Cursor::new(existing.unwrap_or_default()) }));
        }
        let k = self.rel_key(path);
        if lock(&self.overlay).contains_key(&k) {
            return Ok(Box::new(Cursor::new(self.read(path)?)));
        }
        let it = self.item(path).ok_or_else(not_found)?;
        Ok(Box::new(Reader { src: self.src.clone(), id: it.id, len: it.len, pos: 0 }))
    }
}

struct WebFsRef {
    overlay: Overlay,
    key: String,
    rel: String,
}

struct Writer {
    fs: WebFsRef,
    cur: Cursor<Vec<u8>>,
}

impl Read for Writer {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.cur.read(buf)
    }
}
impl Write for Writer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.cur.write(buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        lock(&self.fs.overlay).insert(self.fs.key.clone(), (self.fs.rel.clone(), Arc::new(self.cur.get_ref().clone())));
        Ok(())
    }
}
impl Seek for Writer {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.cur.seek(pos)
    }
}
impl Drop for Writer {
    fn drop(&mut self) {
        let _ = self.flush();
    }
}

struct Reader {
    src: Arc<dyn FileSource>,
    id: u32,
    len: u64,
    pos: u64,
}

impl Read for Reader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.pos >= self.len {
            return Ok(0);
        }
        let want = buf.len().min((self.len - self.pos) as usize);
        let n = self.src.read_at(self.id, self.pos, &mut buf[..want])?;
        self.pos += n as u64;
        Ok(n)
    }
}
impl Write for Reader {
    fn write(&mut self, _: &[u8]) -> io::Result<usize> {
        Err(io::Error::new(io::ErrorKind::PermissionDenied, "abierto solo para leer"))
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
impl Seek for Reader {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        let np = match pos {
            SeekFrom::Start(p) => p as i128,
            SeekFrom::End(d) => self.len as i128 + d as i128,
            SeekFrom::Current(d) => self.pos as i128 + d as i128,
        };
        if np < 0 {
            return Err(io::ErrorKind::InvalidInput.into());
        }
        self.pos = np as u64;
        Ok(self.pos)
    }
}

/// Un archivo del anfitrión como `RangeRead` para [`p3_assets::cpr::StreamedArchive`].
pub struct SourceRange {
    pub src: Arc<dyn FileSource>,
    pub id: u32,
    pub len: u64,
}

impl RangeRead for SourceRange {
    fn len(&self) -> u64 {
        self.len
    }
    fn read_at(&self, offset: u64, buf: &mut [u8]) -> io::Result<()> {
        let mut done = 0;
        while done < buf.len() {
            match self.src.read_at(self.id, offset + done as u64, &mut buf[done..])? {
                0 => return Err(io::ErrorKind::UnexpectedEof.into()),
                n => done += n,
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Mem(Vec<Vec<u8>>);
    impl FileSource for Mem {
        fn read_at(&self, id: u32, offset: u64, buf: &mut [u8]) -> io::Result<usize> {
            let d = &self.0[id as usize];
            let start = (offset as usize).min(d.len());
            let n = buf.len().min(d.len() - start);
            buf[..n].copy_from_slice(&d[start..start + n]);
            Ok(n)
        }
    }

    fn fs() -> Arc<WebFs> {
        let fs = WebFs::new("/juego", Arc::new(Mem(vec![b"MZexe".to_vec(), (0..=255u8).collect(), b"mp3".to_vec()])));
        fs.add(0, "Patrician3.exe", 5);
        fs.add(1, "p2arch0_eng.cpr", 256);
        fs.add(2, "Musik/Theme1.mp3", 3);
        fs
    }

    #[test]
    fn reads_by_range_ignoring_case_and_lists_folders() {
        let fs = fs();
        let root = Path::new("/juego");
        assert_eq!(fs.read(&root.join("PATRICIAN3.EXE")).unwrap(), b"MZexe");
        let mut b = [0u8; 4];
        assert_eq!(fs.read_at(&root.join("p2arch0_eng.cpr"), 254, &mut b).unwrap(), 2);
        assert_eq!(&b[..2], &[254, 255]);
        assert_eq!(fs.read_at(&root.join("p2arch0_eng.cpr"), 256, &mut b).unwrap(), 0);
        assert!(fs.metadata(&root.join("musik")).unwrap().is_dir);
        let mut names: Vec<_> = fs.list(root).unwrap().into_iter().map(|e| (e.name, e.is_dir)).collect();
        names.sort();
        assert_eq!(names, [("Musik".to_string(), true), ("Patrician3.exe".to_string(), false), ("p2arch0_eng.cpr".to_string(), false)]);
        assert_eq!(fs.list(&root.join("Musik")).unwrap()[0].name, "Theme1.mp3");
        assert!(fs.read(&root.join("nada")).is_err());
        let mut f = fs.open(&root.join("Patrician3.exe"), OpenMode { read: true, ..Default::default() }).unwrap();
        f.seek(SeekFrom::Start(2)).unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();
        assert_eq!(s, "exe");
        assert!(f.write(b"x").is_err());
    }

    #[test]
    fn writes_outside_save_stay_in_memory_and_the_originals_are_read_only() {
        let fs = fs();
        let p = Path::new("/juego/save/video-mode.txt");
        fs.write(p, b"auto").unwrap();
        assert_eq!(fs.read_to_string(p).unwrap(), "auto");
        let mut w = fs.open(p, OpenMode { read: true, write: true, create: true, truncate: true }).unwrap();
        w.write_all(b"1920x1080").unwrap();
        drop(w);
        assert_eq!(fs.read_to_string(p).unwrap(), "1920x1080");
        assert!(fs.remove_file(Path::new("/juego/Patrician3.exe")).is_err());
        fs.remove_file(p).unwrap();
        assert!(!fs.exists(p));
    }

    #[test]
    fn range_reader_feeds_a_streamed_archive() {
        use p3_assets::cpr::{build, Archive, StreamedArchive};
        let bytes = build(&[(&b"a\\b.txt"[..], &b"hola"[..])]).unwrap();
        let fs = WebFs::new("/j", Arc::new(Mem(vec![bytes.clone()])));
        fs.add(0, "x.cpr", bytes.len() as u64);
        let st = StreamedArchive::open(Arc::new(fs.source(Path::new("/j/x.cpr")).unwrap())).unwrap();
        let view = Archive::from_streamed(&st);
        let e = view.find(b"a/b.txt").unwrap();
        assert_eq!(view.read(e).unwrap(), b"hola");
    }
}
