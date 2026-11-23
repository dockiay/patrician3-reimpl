//! Contenedor de partidas `.pat` (S04): `operations_save_game` (`0x005472F0`) y `operations_load_game` (`0x005476E0`).
//!
//! El fichero es un bloque comprimido (`0x0054C410`/`0x0054C480`) cuyo contenido es: marcador `0x0085AFFE`, ajustes de partida
//! (`0x4C` bytes), tres dwords de velocidad y, tras un separador `0xAFFE`, siete bloques (`game_world`, `ships`, `message_pool`,
//! `route_stop_pool`, `scheduled_tasks`, `sea_battles`, `name_bank`), cada uno cerrado por otro separador. Orden y límites:
//! `research/systems/borde-motor/save-container`. Los bloques se conservan **como bytes opacos**: solo la cabecera tiene campos con nombre.

pub mod layout;
pub mod model;
pub mod pirate_bands;
pub mod ships;
pub mod siege_scene;
pub mod statistics;
pub mod tail;
pub mod world;
pub mod world_rest;

use p3_core::route::compress::compress;
use p3_core::route::rou::decompress;
use std::fmt;

pub const MARKER: u32 = 0x0085_AFFE;
pub const SEPARATOR: [u8; 4] = [0xFE, 0xAF, 0, 0];
/// Marcador (4) + ajustes (`0x4C`) + tres dwords de velocidad (12).
pub const HEADER_LEN: usize = 4 + SETTINGS_LEN + 12;
pub const SETTINGS_LEN: usize = 0x4C;
/// Bloques tras la cabecera, en el orden del fichero.
pub const BLOCK_NAMES: [&str; 7] = [
    "game_world",
    "ships",
    "message_pool",
    "route_stop_pool",
    "scheduled_tasks",
    "sea_battles",
    "name_bank",
];

#[derive(Debug, PartialEq, Eq)]
pub enum SaveError {
    /// El bloque comprimido no se descomprime.
    Corrupt,
    BadMarker,
    /// Falta el separador tras la cabecera o el último no cierra el fichero.
    BadSeparators,
    /// Cabecera más corta de lo esperado.
    Truncated,
}

impl fmt::Display for SaveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            SaveError::Corrupt => "bloque comprimido inválido",
            SaveError::BadMarker => "marcador distinto de 0x0085AFFE",
            SaveError::BadSeparators => "separadores 0xAFFE fuera de lugar",
            SaveError::Truncated => "fichero truncado",
        })
    }
}

/// Partida descompuesta; `header` conserva el marcador, los ajustes y la velocidad tal cual.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Save {
    header: [u8; HEADER_LEN],
    blocks: [Vec<u8>; 7],
}

impl Save {
    /// Lee un `.pat` completo (bloque comprimido).
    pub fn parse(file: &[u8]) -> Result<Save, SaveError> {
        let (data, _) = decompress(file).ok_or(SaveError::Corrupt)?;
        Save::parse_unpacked(&data)
    }

    /// Lee el contenido ya descomprimido.
    ///
    /// `game_world` puede contener `0xAFFE` por azar; los siete separadores reales son los últimos, y el último cierra el fichero.
    pub fn parse_unpacked(data: &[u8]) -> Result<Save, SaveError> {
        let head: [u8; HEADER_LEN] = data
            .get(..HEADER_LEN)
            .ok_or(SaveError::Truncated)?
            .try_into()
            .unwrap();
        if u32::from_le_bytes(head[..4].try_into().unwrap()) != MARKER {
            return Err(SaveError::BadMarker);
        }
        if data.get(HEADER_LEN..HEADER_LEN + 4) != Some(&SEPARATOR[..]) || data.len() < HEADER_LEN + 8 {
            return Err(SaveError::BadSeparators);
        }
        let found: Vec<usize> = (HEADER_LEN + 4..data.len() - 3)
            .filter(|&i| data[i..i + 4] == SEPARATOR)
            .collect();
        if found.len() < 7 || *found.last().unwrap() != data.len() - 4 {
            return Err(SaveError::BadSeparators);
        }
        let seps = Self::choose_separators(data, &found)?;
        let mut blocks: [Vec<u8>; 7] = Default::default();
        let mut start = HEADER_LEN + 4;
        for (block, &sep) in blocks.iter_mut().zip(seps.iter()) {
            if sep < start {
                return Err(SaveError::BadSeparators);
            }
            *block = data[start..sep].to_vec();
            start = sep + 4;
        }
        Ok(Save { header: head, blocks })
    }

    /// Elige los siete separadores reales entre los candidatos. Cualquier bloque puede contener `0xAFFE0000` por azar
    /// (un campo de barco con ese valor lo hizo en ConvoyHecho, día 120). Se prefieren los últimos candidatos, pero el
    /// bloque de barcos debe empezar por su versión `0x85` y analizarse completo; así un falso separador dentro de
    /// `game_world` o de `ships` no desplaza los límites.
    fn choose_separators(data: &[u8], found: &[usize]) -> Result<Vec<usize>, SaveError> {
        let last = *found.last().unwrap();
        let inner = &found[..found.len() - 1];
        // Combinaciones de 6 separadores internos, de las más tardías a las más tempranas.
        let n = inner.len();
        if n <= 24 {
            let mut combos: Vec<Vec<usize>> = Vec::new();
            fn gen(start: usize, n: usize, left: usize, cur: &mut Vec<usize>, out: &mut Vec<Vec<usize>>) {
                if left == 0 {
                    out.push(cur.clone());
                    return;
                }
                for i in start..=n - left {
                    cur.push(i);
                    gen(i + 1, n, left - 1, cur, out);
                    cur.pop();
                }
            }
            gen(0, n, 6, &mut Vec::new(), &mut combos);
            for idx in combos.iter().rev() {
                let mut seps: Vec<usize> = idx.iter().map(|&i| inner[i]).collect();
                seps.push(last);
                if seps[0] + 4 <= seps[1] && crate::ships::Ships::parse(&data[seps[0] + 4..seps[1]]).is_ok() {
                    return Ok(seps);
                }
            }
        }
        // Sin combinación válida: comportamiento anterior (los siete últimos).
        Ok(found[found.len() - 7..].to_vec())
    }

    /// Partida montada desde sus partes (cabecera del contenedor y los siete bloques en el orden de [`BLOCK_NAMES`]).
    pub fn from_parts(header: [u8; HEADER_LEN], blocks: [Vec<u8>; 7]) -> Save {
        Save { header, blocks }
    }

    /// Cabecera del contenedor (marcador, ajustes y velocidad), tal cual.
    pub fn header(&self) -> &[u8; HEADER_LEN] {
        &self.header
    }

    /// Contenido sin comprimir, idéntico al leído.
    pub fn to_unpacked(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(HEADER_LEN + 8 * 4 + self.blocks.iter().map(Vec::len).sum::<usize>());
        out.extend_from_slice(&self.header);
        out.extend_from_slice(&SEPARATOR);
        for b in &self.blocks {
            out.extend_from_slice(b);
            out.extend_from_slice(&SEPARATOR);
        }
        out
    }

    /// Fichero `.pat` con el compresor original (`0x0054C410`, portado en `p3-core`).
    pub fn to_file(&self) -> Vec<u8> {
        compress(&self.to_unpacked())
    }

    pub fn block(&self, index: usize) -> &[u8] {
        &self.blocks[index]
    }

    pub fn block_named(&self, name: &str) -> Option<&[u8]> {
        BLOCK_NAMES.iter().position(|n| *n == name).map(|i| &self.blocks[i][..])
    }

    /// Ajustes de partida (`g_class37+0x610`, `0x4C` bytes).
    pub fn settings(&self) -> &[u8] {
        &self.header[4..4 + SETTINGS_LEN]
    }

    /// Primer dword de los ajustes (`0x541` en los guardados del corpus).
    pub fn settings_version(&self) -> u32 {
        u32::from_le_bytes(self.settings()[..4].try_into().unwrap())
    }

    /// Fecha de los ajustes: día `+0x1A`, mes `+0x1B`, año `u16` en `+0x1C`.
    pub fn date(&self) -> (u8, u8, u16) {
        let s = self.settings();
        (s[0x1A], s[0x1B], u16::from_le_bytes([s[0x1C], s[0x1D]]))
    }

    /// Nombre guardado en `+0x27` (cadena terminada en cero, hasta el final de los ajustes).
    pub fn name(&self) -> String {
        let tail = &self.settings()[0x27..];
        let end = tail.iter().position(|&b| b == 0).unwrap_or(tail.len());
        tail[..end].iter().map(|&b| b as char).collect()
    }

    /// Dwords de velocidad `Operations+0x8D4/+0x8D8/+0x8DC`.
    pub fn speed(&self) -> [u32; 3] {
        let at = 4 + SETTINGS_LEN;
        std::array::from_fn(|i| u32::from_le_bytes(self.header[at + 4 * i..at + 4 * i + 4].try_into().unwrap()))
    }
}
pub mod office;
pub mod office_factory;
pub mod captain;
