//! Estadísticas del `.pst` (S04): objeto `GameStatistics` serializado por `0x0054D870` (tamaño `0x0054D7F0`).
//!
//! Formato: marcador `0xAFFE000B`, seis `u16` (el sexto es el número de elementos), cuatro cadenas de series opcionales (byte de presencia, eslabones
//! de `0x0054D080`, separador `0xAFFEAFFE`), los elementos de `0x0054CBD0` y un último separador. El cálculo de tamaño suma un byte de más que el
//! escritor, de modo que el fichero acaba con un byte de relleno. Cada lector consume el fichero entero o falla.

use std::ops::Range;

pub const MARKER: u32 = 0xAFFE_000B;
pub const SEPARATOR: u32 = 0xAFFE_AFFE;
pub const SERIES_CHAINS: usize = 4;
/// Valores `f32`/`u32` por serie en un eslabón: `129` palabras por cada una de las `n` series.
pub const WORDS_PER_SERIES: usize = 129;
pub const ITEM_HEAD: usize = 8;
/// Nodos fijos de un elemento (`32 + 24 + 24`) y su tamaño serializado.
pub const ITEM_NODES: usize = 80;
pub const NODE_SIZE: usize = 0x6C;
pub const SIZE_SLACK: usize = 1;
const HEADER_LEN: usize = 16;

#[derive(Debug, PartialEq, Eq)]
pub enum StatisticsError {
    Truncated,
    Marker(u32),
    Presence { chain: usize, byte: u8 },
    Continuation { byte: u8 },
    Separator { at: usize },
    Trailing { used: usize, len: usize },
}

fn u16_at(d: &[u8], o: usize) -> Result<u16, StatisticsError> {
    d.get(o..o + 2).map(|s| u16::from_le_bytes([s[0], s[1]])).ok_or(StatisticsError::Truncated)
}

fn u32_at(d: &[u8], o: usize) -> Result<u32, StatisticsError> {
    d.get(o..o + 4).map(|s| u32::from_le_bytes([s[0], s[1], s[2], s[3]])).ok_or(StatisticsError::Truncated)
}

/// Eslabón de una cadena: `n` series de [`WORDS_PER_SERIES`] palabras.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Link {
    pub series: u32,
    pub data: Range<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Item {
    pub range: Range<usize>,
    /// Nodos extra (`u16` en `+6`) además de los [`ITEM_NODES`] fijos.
    pub extra_nodes: u16,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Statistics {
    /// Los seis `u16` tras el marcador; `header[5]` cuenta los elementos.
    pub header: [u16; 6],
    pub chains: [Option<Vec<Link>>; SERIES_CHAINS],
    pub items: Vec<Item>,
    pub slack: u8,
}

fn read_chain(d: &[u8], mut pos: usize) -> Result<(Vec<Link>, usize), StatisticsError> {
    let mut links = Vec::new();
    loop {
        let n = u32_at(d, pos)?;
        let len = (n as usize).checked_mul(WORDS_PER_SERIES * 4).ok_or(StatisticsError::Truncated)?;
        let data = pos + 4..pos + 4 + len;
        let flag = *d.get(data.end).ok_or(StatisticsError::Truncated)?;
        links.push(Link { series: n, data });
        pos = links.last().unwrap().data.end + 1;
        match flag {
            0 => return Ok((links, pos)),
            1 => {}
            byte => return Err(StatisticsError::Continuation { byte }),
        }
    }
}

impl Statistics {
    pub fn parse(d: &[u8]) -> Result<Statistics, StatisticsError> {
        let marker = u32_at(d, 0)?;
        if marker != MARKER {
            return Err(StatisticsError::Marker(marker));
        }
        let mut header = [0u16; 6];
        for (i, h) in header.iter_mut().enumerate() {
            *h = u16_at(d, 4 + 2 * i)?;
        }
        let mut pos = HEADER_LEN;
        let mut chains: [Option<Vec<Link>>; SERIES_CHAINS] = Default::default();
        for (chain, slot) in chains.iter_mut().enumerate() {
            let byte = *d.get(pos).ok_or(StatisticsError::Truncated)?;
            pos += 1;
            match byte {
                0 => {}
                1 => {
                    let (links, end) = read_chain(d, pos)?;
                    *slot = Some(links);
                    pos = end;
                }
                byte => return Err(StatisticsError::Presence { chain, byte }),
            }
            if u32_at(d, pos)? != SEPARATOR {
                return Err(StatisticsError::Separator { at: pos });
            }
            pos += 4;
        }
        let mut items = Vec::with_capacity(header[5] as usize);
        for _ in 0..header[5] {
            let extra_nodes = u16_at(d, pos + 6)?;
            let end = pos + ITEM_HEAD + (ITEM_NODES + extra_nodes as usize) * NODE_SIZE;
            if end > d.len() {
                return Err(StatisticsError::Truncated);
            }
            items.push(Item { range: pos..end, extra_nodes });
            pos = end;
        }
        if u32_at(d, pos)? != SEPARATOR {
            return Err(StatisticsError::Separator { at: pos });
        }
        pos += 4;
        if pos + SIZE_SLACK != d.len() {
            return Err(StatisticsError::Trailing { used: pos, len: d.len() });
        }
        Ok(Statistics { header, chains, items, slack: d[pos] })
    }

    /// Eslabones de la cadena `index` (vacío si falta).
    pub fn links(&self, index: usize) -> &[Link] {
        self.chains[index].as_deref().unwrap_or(&[])
    }
}
