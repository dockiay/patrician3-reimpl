//! Contenedor `ASCARON_ARCHIVE V0.9` (`.cpr`), tal como lo carga `Archiver.dll` (`1001642C`, índice `100031E0`).
//!
//! Disposición observada en los 86 bloques de `p2arch0_eng.cpr` (6.485 entradas) y reproducida byte a byte por [`build`]:
//!
//! * `0x00`: `ASCARON_ARCHIVE V0.9` y ceros hasta `0x20`.
//! * Desde `0x20`, bloques consecutivos. Cada bloque empieza con cuatro `u32`: tamaño del índice (siempre 4096), bytes de índice
//!   usados (cabecera de 16 bytes incluida), número de entradas y tamaño de los datos.
//! * Después, por entrada, `{u32 desplazamiento absoluto, u32 tamaño, u32 tipo, nombre Latin-1 con NUL}` y ceros hasta el tamaño del
//!   índice. Un bloque admite entradas mientras quepan en él; la siguiente abre otro bloque.
//! * Los datos del bloque siguen al índice. Cada archivo empieza en un múltiplo de 32 bytes contado desde el inicio de los datos
//!   y se rellena con ceros hasta el siguiente múltiplo; `tamaño de datos` incluye ese relleno final.
//! * Tipo 1: almacenado (las 6.485 entradas inglesas). El tipo 2 (comprimido) lo admite el original pero no aparece en los datos
//!   instalados; aquí solo se conserva el campo, sin descomprimir.

use std::fmt;

pub const MAGIC: &[u8] = b"ASCARON_ARCHIVE V0.9";
const FIRST_CHUNK: usize = 0x20;
/// Tamaño del índice de cada bloque.
pub const INDEX_SIZE: usize = 4096;
const CHUNK_HEADER: usize = 16;
const ENTRY_FIXED: usize = 12;
/// Alineación de cada archivo dentro de los datos.
pub const ALIGN: usize = 32;
/// Tipo de entrada almacenada.
pub const TYPE_STORED: u32 = 1;

#[derive(Debug, PartialEq, Eq)]
pub enum CprError {
    /// No empieza por la cabecera (o falta el NUL que la cierra).
    BadMagic,
    /// Un bloque o su índice se sale del archivo, o el índice no cierra bien.
    Truncated { chunk: usize },
    /// Un nombre no cabe en el índice de un bloque (o un archivo supera los 4 GiB).
    TooLarge,
}

impl fmt::Display for CprError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CprError::BadMagic => write!(f, "cabecera ASCARON_ARCHIVE V0.9 ausente"),
            CprError::Truncated { chunk } => write!(f, "bloque {chunk} truncado o mal formado"),
            CprError::TooLarge => write!(f, "entrada demasiado grande para el formato"),
        }
    }
}

impl std::error::Error for CprError {}

/// Entrada del índice.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    /// Nombre en Latin-1, tal como está en el archivo.
    pub name: Vec<u8>,
    pub kind: u32,
    pub offset: u32,
    pub size: u32,
    /// Bloque en que aparece.
    pub chunk: usize,
    /// `false` si el cargador del original la descarta: desplazamiento `0xFFFFFFFF` o `desplazamiento + tamaño`
    /// (suma de 32 bits sin control de desbordamiento) mayor que la longitud del archivo.
    pub kept: bool,
}

#[derive(Debug)]
pub struct Archive<'a> {
    bytes: &'a [u8],
    /// Con CPR por tramos (web): los bytes salen de aquí y `bytes` está vacío.
    streamed: Option<&'a StreamedArchive>,
    pub entries: Vec<Entry>,
}

/// Pliegue de nombres del original (`10004BB0`, `10004C80`): `A-Z` y las mayúsculas Latin-1 `C4`, `D6`, `DC` a minúsculas y `/` a `\`.
pub fn fold_name(name: &[u8]) -> Vec<u8> {
    name.iter().map(|&b| fold_byte(b)).collect()
}

fn fold_byte(b: u8) -> u8 {
    match b {
        b'A'..=b'Z' => b + 32,
        0xC4 | 0xD6 | 0xDC => b + 32,
        b'/' => b'\\',
        _ => b,
    }
}

fn u32_at(b: &[u8], o: usize) -> Option<u32> {
    Some(u32::from_le_bytes(b.get(o..o + 4)?.try_into().ok()?))
}

/// Recorre los bloques del índice. `get(offset, len)` da esos bytes del archivo (o `None` si se salen); `total` es su longitud. Solo
/// pide las cabeceras de bloque y su índice (4 KiB), nunca los datos: sirve igual para un `&[u8]` que para un lector por tramos.
fn scan<B: AsRef<[u8]>>(total: usize, get: &dyn Fn(usize, usize) -> Option<B>) -> Result<Vec<Entry>, CprError> {
    let head = get(0, FIRST_CHUNK.min(total));
    let head = head.as_ref().map(AsRef::as_ref).unwrap_or(&[]);
    if total < FIRST_CHUNK || !head.starts_with(MAGIC) || head[MAGIC.len()] != 0 {
        return Err(CprError::BadMagic);
    }
    let mut entries = Vec::new();
    let mut pos = FIRST_CHUNK;
    let mut chunk = 0;
    while pos < total {
        let bad = CprError::Truncated { chunk };
        let hdr = get(pos, CHUNK_HEADER).ok_or(CprError::Truncated { chunk })?;
        let hdr = hdr.as_ref();
        let index_size = u32_at(hdr, 0).ok_or(CprError::Truncated { chunk })? as usize;
        let files = u32_at(hdr, 8).ok_or(CprError::Truncated { chunk })? as usize;
        let data_size = u32_at(hdr, 12).ok_or(CprError::Truncated { chunk })? as usize;
        let index_end = pos.checked_add(index_size).filter(|&e| e <= total && index_size >= CHUNK_HEADER).ok_or(CprError::Truncated { chunk })?;
        let next = index_end.checked_add(data_size).filter(|&e| e <= total).ok_or(CprError::Truncated { chunk })?;
        let index = get(pos, index_size).ok_or(CprError::Truncated { chunk })?;
        let index = index.as_ref();
        let mut p = CHUNK_HEADER;
        for _ in 0..files {
            let (Some(offset), Some(size), Some(kind)) = (u32_at(index, p), u32_at(index, p + 4), u32_at(index, p + 8)) else { return Err(bad) };
            let name_start = p + ENTRY_FIXED;
            let len = index.get(name_start..).and_then(|s| s.iter().position(|&b| b == 0)).ok_or(CprError::Truncated { chunk })?;
            let kept = offset != u32::MAX && (offset.wrapping_add(size) as usize) <= total;
            entries.push(Entry { name: index[name_start..name_start + len].to_vec(), kind, offset, size, chunk, kept });
            p = name_start + len + 1;
        }
        pos = next;
        chunk += 1;
    }
    Ok(entries)
}

/// Búsqueda como la del original: nombres plegados y gana la última entrada conservada con ese nombre.
fn find_entry<'e>(entries: &'e [Entry], name: &[u8]) -> Option<&'e Entry> {
    let key = fold_name(name);
    // Compara plegando byte a byte, sin reservar una copia plegada de cada nombre del índice en cada búsqueda.
    entries.iter().rev().find(|e| e.kept && e.name.len() == key.len() && e.name.iter().zip(&key).all(|(&n, &k)| fold_byte(n) == k))
}

impl<'a> Archive<'a> {
    /// Recorre los bloques. No decodifica datos.
    pub fn parse(bytes: &'a [u8]) -> Result<Self, CprError> {
        let entries = scan(bytes.len(), &|o, l| bytes.get(o..o.checked_add(l)?))?;
        Ok(Archive { bytes, streamed: None, entries })
    }

    /// Vista de un [`StreamedArchive`] con la misma API: `read` devuelve los mismos bytes, que se leen del origen la primera vez.
    pub fn from_streamed(streamed: &'a StreamedArchive) -> Self {
        Archive { bytes: &[], streamed: Some(streamed), entries: streamed.entries.clone() }
    }

    /// Bytes de una entrada almacenada (tipo 1). `None` si se descartó, no es del tipo 1 o se sale del archivo.
    pub fn read(&self, entry: &Entry) -> Option<&'a [u8]> {
        if !entry.kept || entry.kind != TYPE_STORED {
            return None;
        }
        if let Some(s) = self.streamed {
            return s.read(entry);
        }
        self.bytes.get(entry.offset as usize..entry.offset as usize + entry.size as usize)
    }

    /// Como [`Archive::read`], para quien usa los bytes una vez y los suelta (descodificar una imagen): con el CPR por tramos no
    /// los deja guardados para siempre (WEB-6, memoria; al entrar en una ciudad se piden miles de texturas). Con el CPR entero en
    /// memoria es lo mismo que `read`.
    pub fn read_transient(&self, entry: &Entry) -> Option<std::borrow::Cow<'a, [u8]>> {
        if !entry.kept || entry.kind != TYPE_STORED {
            return None;
        }
        if let Some(s) = self.streamed {
            return s.read_transient(entry);
        }
        self.read(entry).map(std::borrow::Cow::Borrowed)
    }

    /// Búsqueda como la del original: nombres plegados y gana la última entrada conservada con ese nombre.
    pub fn find(&self, name: &[u8]) -> Option<&Entry> {
        find_entry(&self.entries, name)
    }
}

fn align_up(v: usize) -> usize {
    v.div_ceil(ALIGN) * ALIGN
}

/// Construye un archivo con las entradas almacenadas dadas, en ese orden: llena cada bloque mientras quepa la entrada siguiente.
pub fn build(files: &[(&[u8], &[u8])]) -> Result<Vec<u8>, CprError> {
    let mut out = vec![0u8; FIRST_CHUNK];
    out[..MAGIC.len()].copy_from_slice(MAGIC);
    let mut i = 0;
    while i < files.len() {
        let mut used = CHUNK_HEADER;
        let mut j = i;
        while j < files.len() {
            let need = ENTRY_FIXED + files[j].0.len() + 1;
            if used + need > INDEX_SIZE {
                break;
            }
            used += need;
            j += 1;
        }
        if j == i {
            return Err(CprError::TooLarge);
        }
        let base = out.len() + INDEX_SIZE;
        let mut data_size = 0usize;
        let mut index = Vec::with_capacity(INDEX_SIZE);
        index.extend_from_slice(&(INDEX_SIZE as u32).to_le_bytes());
        index.extend_from_slice(&(used as u32).to_le_bytes());
        index.extend_from_slice(&((j - i) as u32).to_le_bytes());
        let mut sizes = Vec::new();
        for (name, data) in &files[i..j] {
            let offset = u32::try_from(base + data_size).map_err(|_| CprError::TooLarge)?;
            let size = u32::try_from(data.len()).map_err(|_| CprError::TooLarge)?;
            sizes.push((offset, size));
            data_size += align_up(data.len());
            let _ = name;
        }
        index.extend_from_slice(&(u32::try_from(data_size).map_err(|_| CprError::TooLarge)?).to_le_bytes());
        for ((name, _), (offset, size)) in files[i..j].iter().zip(&sizes) {
            index.extend_from_slice(&offset.to_le_bytes());
            index.extend_from_slice(&size.to_le_bytes());
            index.extend_from_slice(&TYPE_STORED.to_le_bytes());
            index.extend_from_slice(name);
            index.push(0);
        }
        index.resize(INDEX_SIZE, 0);
        out.extend_from_slice(&index);
        for (_, data) in &files[i..j] {
            out.extend_from_slice(data);
            out.resize(out.len() + align_up(data.len()) - data.len(), 0);
        }
        i = j;
    }
    Ok(out)
}

/// Bytes de un recurso grande (el `.cpr` inglés pesa 248 MB): un `Vec<u8>` o un archivo proyectado en memoria. Clonar solo cuenta
/// una referencia, de modo que reabrir sesiones o compartir el CPR entre hilos no copia 248 MB.
#[derive(Clone)]
pub struct Bytes(std::sync::Arc<BytesInner>);

enum BytesInner {
    Heap(Vec<u8>),
    Mapped(memmap2::Mmap),
}

impl Bytes {
    /// Proyecta el archivo en memoria (solo se leen las páginas que se tocan); si el sistema no puede, lo lee entero.
    pub fn map_file(path: impl AsRef<std::path::Path>) -> std::io::Result<Bytes> {
        let path = path.as_ref();
        let file = std::fs::File::open(path)?;
        // SAFETY: el recurso se trata como de solo lectura; si otro proceso lo trunca mientras está abierto el acceso falla igual
        // que fallaría con cualquier lectura de un archivo que cambia, y el contenido instalado no se modifica durante el juego.
        match unsafe { memmap2::Mmap::map(&file) } {
            Ok(m) => Ok(Bytes(std::sync::Arc::new(BytesInner::Mapped(m)))),
            Err(_) => std::fs::read(path).map(Bytes::from),
        }
    }
}

impl From<Vec<u8>> for Bytes {
    fn from(v: Vec<u8>) -> Self {
        Bytes(std::sync::Arc::new(BytesInner::Heap(v)))
    }
}

impl std::ops::Deref for Bytes {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        match &*self.0 {
            BytesInner::Heap(v) => v,
            BytesInner::Mapped(m) => m,
        }
    }
}

impl AsRef<[u8]> for Bytes {
    fn as_ref(&self) -> &[u8] {
        self
    }
}

/// Lectura por tramos de un recurso grande: lo que necesita [`StreamedArchive`] del almacenamiento (archivo de escritorio, OPFS en la
/// web, un `Vec<u8>` en las pruebas). No exige tener el CPR entero en memoria.
pub trait RangeRead: Send + Sync {
    /// Longitud total en bytes.
    fn len(&self) -> u64;
    /// Rellena `buf` con los bytes desde `offset`; error si el tramo se sale del recurso.
    fn read_at(&self, offset: u64, buf: &mut [u8]) -> std::io::Result<()>;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl RangeRead for [u8] {
    fn len(&self) -> u64 {
        <[u8]>::len(self) as u64
    }
    fn read_at(&self, offset: u64, buf: &mut [u8]) -> std::io::Result<()> {
        let src = usize::try_from(offset).ok().and_then(|o| self.get(o..o.checked_add(buf.len())?));
        buf.copy_from_slice(src.ok_or(std::io::ErrorKind::UnexpectedEof)?);
        Ok(())
    }
}

impl RangeRead for Vec<u8> {
    fn len(&self) -> u64 {
        self.as_slice().len() as u64
    }
    fn read_at(&self, offset: u64, buf: &mut [u8]) -> std::io::Result<()> {
        self.as_slice().read_at(offset, buf)
    }
}

/// Archivo del sistema leído con lecturas posicionales (sin proyectarlo en memoria).
pub struct FileRange {
    file: std::fs::File,
    len: u64,
}

impl FileRange {
    pub fn open(path: impl AsRef<std::path::Path>) -> std::io::Result<Self> {
        let file = std::fs::File::open(path)?;
        let len = file.metadata()?.len();
        Ok(Self { file, len })
    }
}

impl RangeRead for FileRange {
    fn len(&self) -> u64 {
        self.len
    }
    fn read_at(&self, offset: u64, buf: &mut [u8]) -> std::io::Result<()> {
        #[cfg(unix)]
        {
            std::os::unix::fs::FileExt::read_exact_at(&self.file, buf, offset)
        }
        #[cfg(windows)]
        {
            use std::os::windows::fs::FileExt;
            let mut done = 0;
            while done < buf.len() {
                match self.file.seek_read(&mut buf[done..], offset + done as u64)? {
                    0 => return Err(std::io::ErrorKind::UnexpectedEof.into()),
                    n => done += n,
                }
            }
            Ok(())
        }
        // `wasm32-unknown-unknown` no tiene archivos: en web el lector lo pone el anfitrión (`RangeRead` propio).
        #[cfg(not(any(unix, windows)))]
        {
            let _ = (offset, buf);
            Err(std::io::ErrorKind::Unsupported.into())
        }
    }
}

/// Como [`Archive`] pero sin los bytes del CPR en memoria: lee el índice (4 KiB por bloque) al abrir y cada archivo la primera vez
/// que se pide, y lo recuerda (el CPR no cambia en marcha). Misma búsqueda y mismas entradas que `Archive`; `read` devuelve los
/// mismos bytes.
pub struct StreamedArchive {
    source: std::sync::Arc<dyn RangeRead>,
    pub entries: Vec<Entry>,
    cache: Vec<std::sync::OnceLock<Option<Box<[u8]>>>>,
    /// (desplazamiento, tamaño) → posición de la primera entrada con ese tramo (dos entradas con el mismo tramo dan los mismos bytes).
    spans: std::collections::HashMap<(u32, u32), usize>,
}

impl std::fmt::Debug for StreamedArchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StreamedArchive").field("entries", &self.entries.len()).finish()
    }
}

impl StreamedArchive {
    pub fn open(source: std::sync::Arc<dyn RangeRead>) -> Result<Self, CprError> {
        let total = usize::try_from(source.len()).map_err(|_| CprError::TooLarge)?;
        let entries = scan(total, &|o, l| {
            let mut v = vec![0u8; l];
            source.read_at(o as u64, &mut v).ok().map(|()| v)
        })?;
        let cache = entries.iter().map(|_| std::sync::OnceLock::new()).collect();
        let mut spans = std::collections::HashMap::new();
        for (i, e) in entries.iter().enumerate() {
            if e.kept && e.kind == TYPE_STORED {
                spans.entry((e.offset, e.size)).or_insert(i);
            }
        }
        Ok(Self { source, entries, cache, spans })
    }

    /// Entrada por su posición en [`StreamedArchive::entries`] (la que devuelve [`StreamedArchive::find_index`]).
    pub fn find_index(&self, name: &[u8]) -> Option<usize> {
        let key = fold_name(name);
        self.entries.iter().rposition(|e| e.kept && e.name.len() == key.len() && e.name.iter().zip(&key).all(|(&n, &k)| fold_byte(n) == k))
    }

    pub fn find(&self, name: &[u8]) -> Option<&Entry> {
        find_entry(&self.entries, name)
    }

    /// Bytes de la entrada `index` (tipo 1): la primera vez los lee del origen; `None` en los casos en que [`Archive::read`] da `None`
    /// o si la lectura falla.
    pub fn read_index(&self, index: usize) -> Option<&[u8]> {
        let entry = self.entries.get(index)?;
        self.cache[index]
            .get_or_init(|| {
                if !entry.kept || entry.kind != TYPE_STORED {
                    return None;
                }
                let mut buf = vec![0u8; entry.size as usize];
                let end = u64::from(entry.offset) + u64::from(entry.size);
                if end > self.source.len() {
                    return None;
                }
                self.source.read_at(u64::from(entry.offset), &mut buf).ok()?;
                Some(buf.into_boxed_slice())
            })
            .as_deref()
    }

    /// Como [`Archive::read`].
    pub fn read(&self, entry: &Entry) -> Option<&[u8]> {
        // Las entradas se identifican por posición; una copia de `Entry` de otro archivo no se reconoce.
        let index = *self.spans.get(&(entry.offset, entry.size))?;
        if !entry.kept || entry.kind != TYPE_STORED {
            return None;
        }
        self.read_index(index)
    }

    /// Como [`StreamedArchive::read`] pero sin recordar los bytes: si ya estaban leídos los presta; si no, los lee del origen y
    /// los entrega en propiedad.
    pub fn read_transient(&self, entry: &Entry) -> Option<std::borrow::Cow<'_, [u8]>> {
        let index = *self.spans.get(&(entry.offset, entry.size))?;
        let entry = self.entries.get(index)?;
        if !entry.kept || entry.kind != TYPE_STORED {
            return None;
        }
        if let Some(cached) = self.cache[index].get() {
            return cached.as_deref().map(std::borrow::Cow::Borrowed);
        }
        if u64::from(entry.offset) + u64::from(entry.size) > self.source.len() {
            return None;
        }
        let mut buf = vec![0u8; entry.size as usize];
        self.source.read_at(u64::from(entry.offset), &mut buf).ok()?;
        Some(std::borrow::Cow::Owned(buf))
    }

    /// Bytes de archivos que la caché retiene ahora (diagnóstico de memoria).
    pub fn cached_bytes(&self) -> usize {
        self.cache.iter().filter_map(|c| c.get()).flatten().map(|b| b.len()).sum()
    }
}

/// Origen de los bytes de una entrada de [`build_virtual`]: un bloque en memoria o un tramo de otro recurso (otro CPR).
#[derive(Clone)]
pub enum Piece {
    Mem(std::sync::Arc<[u8]>),
    Range(std::sync::Arc<dyn RangeRead>, u64, u32),
}

impl Piece {
    fn len(&self) -> usize {
        match self {
            Piece::Mem(b) => b.len(),
            Piece::Range(_, _, n) => *n as usize,
        }
    }
}

/// CPR que no existe en disco: el índice está en memoria y los datos de cada entrada se leen de donde estén (otros CPR, bloques en
/// memoria). Mismo formato byte a byte que [`build`] con esos datos; se abre con [`StreamedArchive::open`]. Sirve para combinar dos
/// CPR (idioma, web) sin escribir 250 MB.
pub struct VirtualCpr {
    /// (inicio virtual, longitud, origen). Huecos = ceros.
    segments: Vec<(u64, u64, Seg)>,
    len: u64,
}

enum Seg {
    Mem(std::sync::Arc<[u8]>, usize),
    Range(std::sync::Arc<dyn RangeRead>, u64),
}

/// Como [`build`] pero sin copiar los datos: devuelve un [`VirtualCpr`].
pub fn build_virtual(files: &[(Vec<u8>, Piece)]) -> Result<VirtualCpr, CprError> {
    let mut segments: Vec<(u64, u64, Seg)> = Vec::new();
    let mut head = vec![0u8; FIRST_CHUNK];
    head[..MAGIC.len()].copy_from_slice(MAGIC);
    let mut pos = FIRST_CHUNK;
    segments.push((0, FIRST_CHUNK as u64, Seg::Mem(head.into(), 0)));
    let mut i = 0;
    while i < files.len() {
        let mut used = CHUNK_HEADER;
        let mut j = i;
        while j < files.len() {
            let need = ENTRY_FIXED + files[j].0.len() + 1;
            if used + need > INDEX_SIZE {
                break;
            }
            used += need;
            j += 1;
        }
        if j == i {
            return Err(CprError::TooLarge);
        }
        let base = pos + INDEX_SIZE;
        let mut data_size = 0usize;
        let mut index = Vec::with_capacity(INDEX_SIZE);
        index.extend_from_slice(&(INDEX_SIZE as u32).to_le_bytes());
        index.extend_from_slice(&(used as u32).to_le_bytes());
        index.extend_from_slice(&((j - i) as u32).to_le_bytes());
        let mut places = Vec::new();
        for (_, piece) in &files[i..j] {
            let offset = u32::try_from(base + data_size).map_err(|_| CprError::TooLarge)?;
            let size = u32::try_from(piece.len()).map_err(|_| CprError::TooLarge)?;
            places.push((offset, size));
            data_size += align_up(piece.len());
        }
        index.extend_from_slice(&(u32::try_from(data_size).map_err(|_| CprError::TooLarge)?).to_le_bytes());
        for ((name, _), (offset, size)) in files[i..j].iter().zip(&places) {
            index.extend_from_slice(&offset.to_le_bytes());
            index.extend_from_slice(&size.to_le_bytes());
            index.extend_from_slice(&TYPE_STORED.to_le_bytes());
            index.extend_from_slice(name);
            index.push(0);
        }
        index.resize(INDEX_SIZE, 0);
        segments.push((pos as u64, INDEX_SIZE as u64, Seg::Mem(index.into(), 0)));
        for ((_, piece), (offset, size)) in files[i..j].iter().zip(&places) {
            if *size == 0 {
                continue;
            }
            let seg = match piece {
                Piece::Mem(b) => Seg::Mem(b.clone(), 0),
                Piece::Range(src, at, _) => Seg::Range(src.clone(), *at),
            };
            segments.push((u64::from(*offset), u64::from(*size), seg));
        }
        pos = base + data_size;
        i = j;
    }
    Ok(VirtualCpr { segments, len: pos as u64 })
}

impl RangeRead for VirtualCpr {
    fn len(&self) -> u64 {
        self.len
    }
    fn read_at(&self, offset: u64, buf: &mut [u8]) -> std::io::Result<()> {
        let end = offset.checked_add(buf.len() as u64).filter(|&e| e <= self.len).ok_or(std::io::ErrorKind::UnexpectedEof)?;
        buf.fill(0);
        // Primer segmento que termina después de `offset`.
        let mut k = self.segments.partition_point(|(s, l, _)| s + l <= offset);
        while let Some((start, len, seg)) = self.segments.get(k) {
            if *start >= end {
                break;
            }
            let from = offset.max(*start);
            let to = end.min(start + len);
            let dst = &mut buf[(from - offset) as usize..(to - offset) as usize];
            let inner = from - start;
            match seg {
                Seg::Mem(b, at) => dst.copy_from_slice(&b[at + inner as usize..at + (inner + (to - from)) as usize]),
                Seg::Range(src, at) => src.read_at(at + inner, dst)?,
            }
            k += 1;
        }
        Ok(())
    }
}

#[cfg(test)]
mod virtual_tests {
    use super::*;

    #[test]
    fn virtual_cpr_matches_build_byte_for_byte() {
        let other: std::sync::Arc<dyn RangeRead> = std::sync::Arc::new(b"xxHELLO-FROM-OTHERyy".to_vec());
        let big = vec![7u8; 5000];
        let names: Vec<Vec<u8>> = (0..400).map(|i| format!("dir\\file_{i:04}.bin").into_bytes()).collect();
        let mut real: Vec<(&[u8], &[u8])> = Vec::new();
        let mut virt: Vec<(Vec<u8>, Piece)> = Vec::new();
        for (i, n) in names.iter().enumerate() {
            let (data, piece): (&[u8], Piece) = match i % 3 {
                0 => (&b"HELLO-FROM-OTHER"[..], Piece::Range(other.clone(), 2, 16)),
                1 => (&big[..i], Piece::Mem(big[..i].to_vec().into())),
                _ => (&b""[..], Piece::Mem(Vec::new().into())),
            };
            real.push((n, data));
            virt.push((n.clone(), piece));
        }
        let want = build(&real).unwrap();
        let v = build_virtual(&virt).unwrap();
        assert_eq!(v.len(), want.len() as u64);
        let mut got = vec![0u8; want.len()];
        v.read_at(0, &mut got).unwrap();
        assert_eq!(got, want);
        let mut piece = vec![0u8; 777];
        v.read_at(4000, &mut piece).unwrap();
        assert_eq!(piece, want[4000..4777]);
        let st = StreamedArchive::open(std::sync::Arc::new(v)).unwrap();
        let e = st.find(b"dir/file_0003.bin").unwrap().clone();
        assert_eq!(st.read(&e), Some(&b"HELLO-FROM-OTHER"[..]));
    }
}
