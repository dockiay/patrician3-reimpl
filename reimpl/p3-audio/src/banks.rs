//! Listas y catálogo de muestras. Los servicios INI y de archivo son explícitos.
use std::collections::BTreeMap;

use crate::{samples::SampleCatalog, SoundManager};

pub trait BankResources {
    fn integer(&mut self, file: &str, section: &str, key: &str, default: i32) -> i32;
    fn text(&mut self, file: &str, section: &str, key: &str, capacity: u32) -> Option<String>;
    fn archive_new(&mut self);
    fn archive_open(&mut self, name: &str, mode: u32) -> bool;
    fn archive_length(&mut self) -> u32;
    fn archive_close(&mut self);
    fn archive_drop(&mut self);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BankError {
    NegativeCount(i32),
    InconsistentState,
    Capacity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SoundList {
    pub id: i32,
    pub count: u32,
}

/// Las tablas conservan orden y duplicados, igual que el cargador original.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SoundBank {
    pub ids: Vec<u32>,
    pub names: Vec<String>,
    pub gains: Vec<u16>,
    pub sizes: Vec<u32>,
    pub lists: Vec<SoundList>,
}

impl SoundBank {
    /// Nombres prestados para el catálogo usado por play_sound y los volúmenes.
    pub fn name_refs(&self) -> Vec<&str> {
        self.names.iter().map(String::as_str).collect()
    }

    pub fn catalog<'a>(&'a self, names: &'a [&'a str]) -> SampleCatalog<'a> {
        SampleCatalog {
            ids: &self.ids,
            gains: &self.gains,
            names,
            sizes: &self.sizes,
        }
    }
}

/// atoi del CRT en locale C: espacios ASCII, signo, prefijo decimal y vuelta i32.
/// No exige que toda la cadena sea numérica ni satura al desbordar.
pub fn decimal_prefix(value: &str) -> i32 {
    let mut bytes = value
        .bytes()
        .skip_while(|b| matches!(b, b' ' | b'\t' | b'\n' | b'\r' | 11 | 12))
        .peekable();
    let negative = bytes.peek() == Some(&b'-');
    if negative || bytes.peek() == Some(&b'+') {
        bytes.next();
    }
    let mut result = 0u32;
    for byte in bytes {
        if !byte.is_ascii_digit() {
            break;
        }
        result = result.wrapping_mul(10).wrapping_add((byte - b'0') as u32);
    }
    if negative {
        result.wrapping_neg() as i32
    } else {
        result as i32
    }
}

impl SoundManager {
    /// 0x00443690. Decide la primera lista con ese número, incluso si se repite.
    pub fn get_sound_list_size(&self, bank: &SoundBank, id: i32) -> Result<u32, BankError> {
        if self.enabled() {
            for i in 0..(self.word(0x138) as i32).max(0) as usize {
                let list = bank.lists.get(i).ok_or(BankError::InconsistentState)?;
                if list.id == id {
                    return Ok(list.count);
                }
            }
        }
        Ok(0)
    }

    /// 0x004439B0 en el dominio de asignaciones correctas y Count no negativo.
    /// true indica que se anexó una lista; no representa un retorno del x86 (void).
    pub fn load_sound_list(
        &mut self,
        bank: &mut SoundBank,
        resources: &mut impl BankResources,
        ini: &str,
        id: i32,
    ) -> Result<bool, BankError> {
        if !self.enabled() {
            return Ok(false);
        }
        let section = format!("SoundList{id}");
        let count = resources.integer(ini, &section, "Count", -1);
        let Some(file) = resources.text(ini, &section, "Files", 4096) else {
            return Ok(false);
        };
        let Some(mut tokens) = resources.text(ini, &section, "IDs", 4096) else {
            return Ok(false);
        };
        let count_usize = usize::try_from(count).map_err(|_| BankError::NegativeCount(count))?;
        let old = bank.ids.len();
        if bank.names.len() != old
            || bank.gains.len() != old
            || bank.sizes.len() != old
            || self.word(0x134) as usize != old
            || self.word(0x138) as usize != bank.lists.len()
        {
            return Err(BankError::InconsistentState);
        }
        let total = old
            .checked_add(count_usize)
            .filter(|&n| n <= i32::MAX as usize)
            .ok_or(BankError::Capacity)?;
        bank.ids.try_reserve(count_usize).map_err(|_| BankError::Capacity)?;
        bank.names.try_reserve(count_usize).map_err(|_| BankError::Capacity)?;
        bank.gains.try_reserve(count_usize).map_err(|_| BankError::Capacity)?;
        bank.sizes.try_reserve(count_usize).map_err(|_| BankError::Capacity)?;
        bank.lists.try_reserve(1).map_err(|_| BankError::Capacity)?;
        let mut ids = Vec::new();
        ids.try_reserve_exact(count_usize).map_err(|_| BankError::Capacity)?;
        // El registro de lista se publica antes de leer las muestras, también con Count=0.
        bank.lists.push(SoundList {
            id,
            count: count as u32,
        });
        self.put(0x138, bank.lists.len() as u32);
        for _ in 0..count_usize {
            // CString::Find(" ",1), Left(index), Delete(0,index+1). Sin separador
            // Delete(0,0) no cambia nada: el último token se repite.
            if let Some(index) = tokens
                .bytes()
                .enumerate()
                .skip(1)
                .find_map(|(i, b)| (b == b' ').then_some(i))
            {
                ids.push(decimal_prefix(&tokens[..index]) as u32);
                tokens.drain(..index + 1);
            } else {
                ids.push(decimal_prefix(&tokens) as u32);
            }
        }
        let mut names = Vec::new();
        let mut gains = Vec::new();
        for sample_id in &ids {
            let sample_section = format!("SAMPLE{}", *sample_id as i32);
            names.push(
                resources
                    .text(&file, &sample_section, "SampleFile", 4096)
                    .unwrap_or_default(),
            );
            gains.push(resources.integer(&file, &sample_section, "Loudness", 100) as u16);
        }
        let mut sizes = Vec::new();
        resources.archive_new();
        for name in &names {
            let size = if resources.archive_open(name, 0x8000) {
                resources.archive_length()
            } else {
                0
            };
            sizes.push(size);
            // El original llama close incluso si open devuelve cero.
            resources.archive_close();
        }
        bank.ids.extend(ids);
        bank.names.extend(names);
        bank.gains.extend(gains);
        bank.sizes.extend(sizes);
        self.put(0x134, total as u32);
        resources.archive_drop();
        Ok(true)
    }
}

/// Adaptador de datos: no hace E/S. Recibe documentos INI decodificados y un
/// índice de longitudes. No pretende sustituir toda la clase INI original.
#[derive(Default)]
pub struct IndexedResources {
    documents: BTreeMap<String, BTreeMap<(String, String), String>>,
    lengths: BTreeMap<String, u32>,
    current: Option<u32>,
}

fn path_key(path: &str) -> String {
    path.replace('\\', "/").trim_start_matches("./").to_ascii_lowercase()
}

impl IndexedResources {
    /// Formato simple de los bancos: secciones, clave=valor y comentarios ';'.
    /// Espacios exteriores se ignoran; gana la última clave repetida.
    pub fn add_ini(&mut self, path: &str, content: &str) {
        let mut section = String::new();
        let mut values = BTreeMap::new();
        for line in content.lines().map(str::trim) {
            if line.starts_with(';') || line.is_empty() {
                continue;
            }
            if let Some(s) = line.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
                section = s.to_ascii_lowercase();
            } else if let Some((key, value)) = line.split_once('=') {
                values.insert(
                    (section.clone(), key.trim().to_ascii_lowercase()),
                    value.trim().to_owned(),
                );
            }
        }
        self.documents.insert(path_key(path), values);
    }

    pub fn add_length(&mut self, path: &str, length: u32) {
        self.lengths.insert(path_key(path), length);
    }

    fn value(&self, file: &str, section: &str, key: &str) -> Option<&str> {
        self.documents
            .get(&path_key(file))?
            .get(&(section.to_ascii_lowercase(), key.to_ascii_lowercase()))
            .map(String::as_str)
    }
}

impl BankResources for IndexedResources {
    fn integer(&mut self, file: &str, section: &str, key: &str, default: i32) -> i32 {
        self.value(file, section, key).map(decimal_prefix).unwrap_or(default)
    }
    fn text(&mut self, file: &str, section: &str, key: &str, capacity: u32) -> Option<String> {
        let value = self.value(file, section, key)?;
        if value.is_empty() || capacity == 0 {
            return None;
        }
        // La entrada pública es UTF-8; no cortar dentro de un carácter.
        let mut end = value.len().min(capacity as usize - 1);
        while !value.is_char_boundary(end) {
            end -= 1;
        }
        (end != 0).then(|| value[..end].to_owned())
    }
    fn archive_new(&mut self) {
        self.current = None;
    }
    fn archive_open(&mut self, name: &str, _mode: u32) -> bool {
        self.current = self.lengths.get(&path_key(name)).copied();
        self.current.is_some()
    }
    fn archive_length(&mut self) -> u32 {
        self.current.unwrap_or(0)
    }
    fn archive_close(&mut self) {
        self.current = None;
    }
    fn archive_drop(&mut self) {
        self.current = None;
    }
}
