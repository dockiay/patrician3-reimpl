//! Fuentes de recursos propias: CPR V0.9 tipo 1, archivos sueltos y datos en memoria.
//! Lectura acotada; no ejecuta Archiver.dll ni reproduce sus punteros/mapeos.
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourceError {
    InvalidName,
    NotFound,
    Io(std::io::ErrorKind),
    TooLarge,
    Capacity,
    BadArchive,
    UnsupportedEntry(u32),
    UnknownToken(u32),
    WrongKind(u32),
    BufferBounds,
    BufferShared,
    TokenLimit,
}
impl From<std::io::Error> for ResourceError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e.kind())
    }
}

/// El plegado acreditado de Archiver es Latin-1: A-Z, Ä/Ö/Ü y separadores.
/// El adaptador admite solo rutas relativas, sin ascensos ni NUL; omite './'.
pub fn resource_key(name: &str) -> Result<String, ResourceError> {
    let name = name.replace('\\', "/");
    if name.starts_with('/') || name.contains(['\0', ':']) {
        return Err(ResourceError::InvalidName);
    }
    let mut parts = Vec::new();
    for part in name.split('/') {
        if part.is_empty() || part == "." {
            continue;
        }
        if part == ".." {
            return Err(ResourceError::InvalidName);
        }
        let mut folded = String::new();
        for ch in part.chars() {
            folded.push(match ch {
                'A'..='Z' => ch.to_ascii_lowercase(),
                'Ä' => 'ä',
                'Ö' => 'ö',
                'Ü' => 'ü',
                '\u{1}'..='\u{ff}' => ch,
                _ => return Err(ResourceError::InvalidName),
            });
        }
        parts.push(folded);
    }
    if parts.is_empty() {
        return Err(ResourceError::InvalidName);
    }
    Ok(parts.join("/"))
}

#[derive(Clone)]
enum Asset {
    Memory(Arc<[u8]>),
    Disk { path: PathBuf, offset: u64, length: u32 },
}

/// Lector acotado a una entrada, incluido el offset de un archivo CPR.
pub struct ResourceStream {
    inner: ResourceStreamInner,
    length: u64,
    position: u64,
}
enum ResourceStreamInner {
    Memory(Arc<[u8]>),
    Disk { file: File, base: u64 },
}
impl ResourceStream {
    pub fn len(&self) -> u64 {
        self.length
    }
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
}
impl Read for ResourceStream {
    fn read(&mut self, output: &mut [u8]) -> std::io::Result<usize> {
        let n = output.len().min((self.length - self.position) as usize);
        let got = match &mut self.inner {
            ResourceStreamInner::Memory(data) => {
                let start = self.position as usize;
                output[..n].copy_from_slice(&data[start..start + n]);
                n
            }
            ResourceStreamInner::Disk { file, .. } => file.read(&mut output[..n])?,
        };
        self.position += got as u64;
        Ok(got)
    }
}
impl Seek for ResourceStream {
    fn seek(&mut self, from: SeekFrom) -> std::io::Result<u64> {
        let position = match from {
            SeekFrom::Start(n) => i128::from(n),
            SeekFrom::Current(n) => i128::from(self.position) + i128::from(n),
            SeekFrom::End(n) => i128::from(self.length) + i128::from(n),
        };
        if position < 0 || position > i128::from(self.length) {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidInput));
        }
        let position = position as u64;
        if let ResourceStreamInner::Disk { file, base } = &mut self.inner {
            file.seek(SeekFrom::Start(*base + position))?;
        }
        self.position = position;
        Ok(position)
    }
}
impl Asset {
    fn open(&self, limit: u32) -> Result<ResourceStream, ResourceError> {
        match self {
            Self::Memory(data) => {
                if data.len() > limit as usize {
                    return Err(ResourceError::TooLarge);
                }
                Ok(ResourceStream {
                    inner: ResourceStreamInner::Memory(data.clone()),
                    length: data.len() as u64,
                    position: 0,
                })
            }
            Self::Disk { path, offset, length } => {
                if *length > limit {
                    return Err(ResourceError::TooLarge);
                }
                let mut file = File::open(path)?;
                file.seek(SeekFrom::Start(*offset))?;
                Ok(ResourceStream {
                    inner: ResourceStreamInner::Disk { file, base: *offset },
                    length: u64::from(*length),
                    position: 0,
                })
            }
        }
    }
    fn read(&self, limit: u32) -> Result<Arc<[u8]>, ResourceError> {
        match self {
            Self::Memory(data) => {
                if data.len() > limit as usize {
                    return Err(ResourceError::TooLarge);
                }
                Ok(data.clone())
            }
            Self::Disk { path, offset, length } => {
                if *length > limit {
                    return Err(ResourceError::TooLarge);
                }
                let mut data = Vec::new();
                data.try_reserve_exact(*length as usize)
                    .map_err(|_| ResourceError::Capacity)?;
                data.resize(*length as usize, 0);
                let mut file = File::open(path)?;
                file.seek(SeekFrom::Start(*offset))?;
                file.read_exact(&mut data)?;
                Ok(data.into())
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Search {
    LooseOnly,
    ArchiveOnly,
    LooseFirst,
    ArchiveFirst,
}

pub struct ResourceFiles {
    archive: BTreeMap<String, Asset>,
    loose: BTreeMap<String, Asset>,
    pub max_file_bytes: u32,
}
impl Default for ResourceFiles {
    fn default() -> Self {
        Self {
            archive: BTreeMap::new(),
            loose: BTreeMap::new(),
            max_file_bytes: 16 * 1024 * 1024,
        }
    }
}
impl ResourceFiles {
    pub fn insert_bytes(&mut self, name: &str, data: Arc<[u8]>, loose: bool) -> Result<(), ResourceError> {
        if data.len() > self.max_file_bytes as usize {
            return Err(ResourceError::TooLarge);
        }
        let key = resource_key(name)?;
        if loose { &mut self.loose } else { &mut self.archive }.insert(key, Asset::Memory(data));
        Ok(())
    }
    /// Registro explícito, sin explorar ni modificar la instalación. La ruta del
    /// disco es elegida por el cliente; las consultas posteriores usan el índice.
    pub fn mount_loose_file(&mut self, name: &str, path: impl AsRef<Path>) -> Result<(), ResourceError> {
        let key = resource_key(name)?;
        let length = File::open(path.as_ref())?.metadata()?.len();
        if length > self.max_file_bytes as u64 {
            return Err(ResourceError::TooLarge);
        }
        self.loose.insert(
            key,
            Asset::Disk {
                path: path.as_ref().to_owned(),
                offset: 0,
                length: length as u32,
            },
        );
        Ok(())
    }
    pub fn read(&self, name: &str, search: Search) -> Result<Arc<[u8]>, ResourceError> {
        let key = resource_key(name)?;
        let a = self.archive.get(&key);
        let l = self.loose.get(&key);
        let read = |asset: Option<&Asset>| asset.ok_or(ResourceError::NotFound)?.read(self.max_file_bytes);
        match search {
            Search::LooseOnly => read(l),
            Search::ArchiveOnly => read(a),
            Search::LooseFirst => read(l).or_else(|first| if a.is_some() { read(a) } else { Err(first) }),
            Search::ArchiveFirst => read(a).or_else(|first| if l.is_some() { read(l) } else { Err(first) }),
        }
    }
    /// Abre una entrada sin cargarla completa; el trabajador posee su lector.
    pub fn open_stream(&self, name: &str, search: Search) -> Result<ResourceStream, ResourceError> {
        let key = resource_key(name)?;
        let a = self.archive.get(&key);
        let l = self.loose.get(&key);
        let open = |asset: Option<&Asset>| asset.ok_or(ResourceError::NotFound)?.open(self.max_file_bytes);
        match search {
            Search::LooseOnly => open(l),
            Search::ArchiveOnly => open(a),
            Search::LooseFirst => open(l).or_else(|first| if a.is_some() { open(a) } else { Err(first) }),
            Search::ArchiveFirst => open(a).or_else(|first| if l.is_some() { open(l) } else { Err(first) }),
        }
    }
    pub fn archive_names(&self) -> impl Iterator<Item = &str> {
        self.archive.keys().map(String::as_str)
    }

    /// Subconjunto normal de CPR: cabecera de 32 bytes, bloques índice/datos,
    /// entradas tipo 1 con offset absoluto. Publicación atómica del índice.
    /// El archivo debe mantenerse inmutable mientras esté montado.
    pub fn append_cpr(&mut self, path: impl AsRef<Path>) -> Result<usize, ResourceError> {
        const MAX_INDEX: u32 = 16 * 1024 * 1024;
        const MAX_ENTRIES: usize = 100_000;
        let path = path.as_ref();
        let mut file = File::open(path)?;
        let length = file.metadata()?.len();
        let mut header = [0; 32];
        file.read_exact(&mut header)?;
        if !header.starts_with(b"ASCARON_ARCHIVE V0.9\0") {
            return Err(ResourceError::BadArchive);
        }
        let mut pos = 32u64;
        let mut entries = Vec::new();
        while pos < length {
            if length - pos < 16 {
                return Err(ResourceError::BadArchive);
            }
            file.seek(SeekFrom::Start(pos))?;
            let mut block = [0; 16];
            file.read_exact(&mut block)?;
            let word = |i| u32::from_le_bytes(block[i..i + 4].try_into().unwrap());
            let (index, used, count, data) = (word(0), word(4), word(8), word(12));
            if index < 16
                || used < 16
                || used > index
                || used > MAX_INDEX
                || count > (used - 16) / 13
                || entries.len().saturating_add(count as usize) > MAX_ENTRIES
            {
                return Err(ResourceError::BadArchive);
            }
            let next = pos
                .checked_add(index as u64)
                .and_then(|n| n.checked_add(data as u64))
                .ok_or(ResourceError::BadArchive)?;
            if next > length {
                return Err(ResourceError::BadArchive);
            }
            let mut raw = Vec::new();
            // Ambos tamaños de índice incluyen su cabecera de 16 bytes.
            raw.try_reserve_exact((used - 16) as usize)
                .map_err(|_| ResourceError::Capacity)?;
            raw.resize((used - 16) as usize, 0);
            file.read_exact(&mut raw)?;
            let mut at = 0usize;
            for _ in 0..count {
                if raw.len().saturating_sub(at) < 13 {
                    return Err(ResourceError::BadArchive);
                }
                let word = |i| u32::from_le_bytes(raw[at + i..at + i + 4].try_into().unwrap());
                let (offset, size, kind) = (word(0), word(4), word(8));
                if kind != 1 {
                    return Err(ResourceError::UnsupportedEntry(kind));
                }
                if offset as u64 + size as u64 > length {
                    return Err(ResourceError::BadArchive);
                }
                let end = raw[at + 12..]
                    .iter()
                    .position(|&b| b == 0)
                    .map(|i| at + 12 + i)
                    .ok_or(ResourceError::BadArchive)?;
                let name: String = raw[at + 12..end].iter().map(|b| char::from(*b)).collect();
                let key = resource_key(&name)?;
                entries.push((
                    key,
                    Asset::Disk {
                        path: path.to_owned(),
                        offset: offset as u64,
                        length: size,
                    },
                ));
                at = end + 1;
            }
            pos = next;
        }
        let count = entries.len();
        for (key, value) in entries {
            self.archive.insert(key, value);
        }
        Ok(count)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ResourceLimits {
    pub max_buffer_bytes: usize,
    pub max_owned_bytes: usize,
    pub max_resources: usize,
}
impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_buffer_bytes: 16 * 1024 * 1024,
            max_owned_bytes: 64 * 1024 * 1024,
            max_resources: 4096,
        }
    }
}
enum Resource {
    File { bytes: Arc<[u8]>, cursor: usize },
    Buffer { bytes: Arc<[u8]>, filled: usize },
}
#[derive(Debug, PartialEq, Eq)]
pub struct ResourceStats {
    pub files: usize,
    pub buffers: usize,
    pub owned_buffer_bytes: usize,
}
pub struct ResourceArena {
    entries: BTreeMap<u32, Resource>,
    next: Option<u32>,
    limits: ResourceLimits,
    owned: usize,
}
impl ResourceArena {
    pub fn new(limits: ResourceLimits) -> Self {
        Self {
            entries: BTreeMap::new(),
            next: Some(1),
            limits,
            owned: 0,
        }
    }
    fn token(&mut self) -> Result<u32, ResourceError> {
        if self.entries.len() >= self.limits.max_resources {
            return Err(ResourceError::TokenLimit);
        }
        let token = self.next.ok_or(ResourceError::TokenLimit)?;
        self.next = token.checked_add(1);
        Ok(token)
    }
    pub fn open(&mut self, bytes: Arc<[u8]>) -> Result<u32, ResourceError> {
        if bytes.len() > u32::MAX as usize {
            return Err(ResourceError::TooLarge);
        }
        let token = self.token()?;
        self.entries.insert(token, Resource::File { bytes, cursor: 0 });
        Ok(token)
    }
    pub fn allocate(&mut self, size: u32) -> Result<u32, ResourceError> {
        let size = size as usize;
        if size > self.limits.max_buffer_bytes || size > self.limits.max_owned_bytes.saturating_sub(self.owned) {
            return Err(ResourceError::TooLarge);
        }
        let mut bytes = Vec::new();
        bytes.try_reserve_exact(size).map_err(|_| ResourceError::Capacity)?;
        bytes.resize(size, 0);
        self.insert_buffer(bytes.into(), 0)
    }
    pub fn from_bytes(&mut self, bytes: Arc<[u8]>) -> Result<u32, ResourceError> {
        let filled = bytes.len();
        if filled > self.limits.max_buffer_bytes || filled > self.limits.max_owned_bytes.saturating_sub(self.owned) {
            return Err(ResourceError::TooLarge);
        }
        self.insert_buffer(bytes, filled)
    }
    fn insert_buffer(&mut self, bytes: Arc<[u8]>, filled: usize) -> Result<u32, ResourceError> {
        let token = self.token()?;
        self.owned += bytes.len();
        self.entries.insert(token, Resource::Buffer { bytes, filled });
        Ok(token)
    }
    fn get(&self, token: u32) -> Result<&Resource, ResourceError> {
        self.entries.get(&token).ok_or(ResourceError::UnknownToken(token))
    }
    pub fn length(&self, token: u32) -> Result<u32, ResourceError> {
        match self.get(token)? {
            Resource::File { bytes, .. } => u32::try_from(bytes.len()).map_err(|_| ResourceError::TooLarge),
            _ => Err(ResourceError::WrongKind(token)),
        }
    }
    pub fn read(&mut self, file: u32, buffer: u32, requested: u32) -> Result<u32, ResourceError> {
        let (source, cursor) = match self.get(file)? {
            Resource::File { bytes, cursor } => (bytes.clone(), *cursor),
            _ => return Err(ResourceError::WrongKind(file)),
        };
        match self
            .entries
            .get_mut(&buffer)
            .ok_or(ResourceError::UnknownToken(buffer))?
        {
            Resource::Buffer { bytes, filled } => {
                if requested as usize > bytes.len() {
                    return Err(ResourceError::BufferBounds);
                }
                let n = (requested as usize).min(source.len() - cursor);
                if n != 0 {
                    let destination = Arc::get_mut(bytes).ok_or(ResourceError::BufferShared)?;
                    destination[..n].copy_from_slice(&source[cursor..cursor + n]);
                    *filled = (*filled).max(n);
                }
                if let Some(Resource::File { cursor, .. }) = self.entries.get_mut(&file) {
                    *cursor += n;
                }
                Ok(n as u32)
            }
            _ => Err(ResourceError::WrongKind(buffer)),
        }
    }
    pub fn buffer(&self, token: u32) -> Result<&[u8], ResourceError> {
        match self.get(token)? {
            Resource::Buffer { bytes, .. } => Ok(bytes),
            _ => Err(ResourceError::WrongKind(token)),
        }
    }
    pub fn lease(&self, token: u32) -> Result<(Arc<[u8]>, usize), ResourceError> {
        match self.get(token)? {
            Resource::Buffer { bytes, filled } => Ok((bytes.clone(), *filled)),
            _ => Err(ResourceError::WrongKind(token)),
        }
    }
    pub fn close(&mut self, token: u32) -> Result<(), ResourceError> {
        if token == 0 {
            return Ok(());
        }
        if !matches!(self.get(token)?, Resource::File { .. }) {
            return Err(ResourceError::WrongKind(token));
        }
        self.entries.remove(&token);
        Ok(())
    }
    pub fn free(&mut self, token: u32) -> Result<(), ResourceError> {
        if token == 0 {
            return Ok(());
        }
        let len = match self.get(token)? {
            Resource::Buffer { bytes, .. } => bytes.len(),
            _ => return Err(ResourceError::WrongKind(token)),
        };
        self.entries.remove(&token);
        self.owned -= len;
        Ok(())
    }
    pub fn stats(&self) -> ResourceStats {
        let files = self
            .entries
            .values()
            .filter(|v| matches!(v, Resource::File { .. }))
            .count();
        ResourceStats {
            files,
            buffers: self.entries.len() - files,
            owned_buffer_bytes: self.owned,
        }
    }
}
