//! Cola de `game_world` (S04): lo que sigue a los comerciantes (`World::rest`), de `0x0052FAFC` a `0x0052FB5C`.
//!
//! Orden: historial de información de las ciudades (`0x535160`: `0xA00` bytes con 40 × 16 banderas de `u32`, ocho ranuras de un byte y 40 bytes por ranura distinta de
//! `0xFF`), quince centros comerciales (`0x52BB30`: 30 bytes y 16 por elemento, con el número de elementos en el `u16` de `+0x1C`), un byte
//! 0/1 que dice si hay lista de ofertas, la lista de ofertas (solo si el byte es 1) y por último `Town`-mundo `+0x8F`.
//! `0x533d10` escribe ofertas de 19 bytes; el lector `0x533da0` acepta además
//! una extensión cuyo tamaño figura en los dos bytes previos al marcador.

use std::ops::Range;

pub const HISTORY_TABLE: usize = 0xA00;
pub const HISTORY_SLOTS: usize = 8;
pub const HISTORY_SLOT_LEN: usize = 40;
pub const TRADE_CENTERS: usize = 15;
pub const TRADE_CENTER_LEN: usize = 30;
pub const TRADE_ITEM_LEN: usize = 16;
pub const OFFER_LEN: usize = 19;

#[derive(Debug, PartialEq, Eq)]
pub enum RestError {
    Truncated,
    Trailing { used: usize, len: usize },
    Unsupported(&'static str),
}

#[derive(Debug)]
pub struct WorldRest<'a> {
    data: &'a [u8],
    /// Tabla de `0xA00` bytes del historial de información.
    pub history: Range<usize>,
    /// Ocho ranuras: `0xFF` = libre; si no, 40 bytes siguen a las ocho (en orden de ranura).
    pub slots: [u8; HISTORY_SLOTS],
    pub slot_data: Vec<Range<usize>>,
    /// Quince centros: cada rango incluye sus elementos de 16 bytes.
    pub centers: Vec<Range<usize>>,
    /// Rango completo: byte de presencia y registros serializados.
    pub offers: Range<usize>,
    offer_records: Vec<Range<usize>>,
    /// Último byte: campo `+0x8F` del mundo (sin nombre semántico probado).
    pub field_8f: u8,
}

fn u16_at(d: &[u8], o: usize) -> Result<u16, RestError> {
    d.get(o..o + 2).map(|s| u16::from_le_bytes([s[0], s[1]])).ok_or(RestError::Truncated)
}

impl<'a> WorldRest<'a> {
    pub fn parse(data: &'a [u8]) -> Result<Self, RestError> {
        let history = 0..HISTORY_TABLE;
        let slots: [u8; HISTORY_SLOTS] =
            data.get(HISTORY_TABLE..HISTORY_TABLE + HISTORY_SLOTS).ok_or(RestError::Truncated)?.try_into().unwrap();
        let mut pos = HISTORY_TABLE + HISTORY_SLOTS;
        let mut slot_data = Vec::new();
        for &s in &slots {
            if s != 0xFF {
                slot_data.push(pos..pos + HISTORY_SLOT_LEN);
                pos += HISTORY_SLOT_LEN;
            }
        }
        let mut centers = Vec::with_capacity(TRADE_CENTERS);
        for _ in 0..TRADE_CENTERS {
            let items = usize::from(u16_at(data, pos + 0x1C)?);
            let end = pos + TRADE_CENTER_LEN + TRADE_ITEM_LEN * items;
            centers.push(pos..end);
            pos = end;
        }
        let offer_start = pos;
        let mut present = *data.get(pos).ok_or(RestError::Truncated)?;
        pos += 1;
        let mut offer_records = Vec::new();
        while present != 0 {
            let extension = usize::from(u16_at(data, pos.checked_add(16).ok_or(RestError::Truncated)?)?);
            let end = pos.checked_add(OFFER_LEN).and_then(|v| v.checked_add(extension)).ok_or(RestError::Truncated)?;
            data.get(pos..end).ok_or(RestError::Truncated)?;
            offer_records.push(pos..pos + 16);
            present = data[end - 1];
            pos = end;
        }
        let offers = offer_start..pos;
        let field_8f = *data.get(pos).ok_or(RestError::Truncated)?;
        pos += 1;
        if pos != data.len() {
            return Err(if pos > data.len() { RestError::Truncated } else { RestError::Trailing { used: pos, len: data.len() } });
        }
        Ok(WorldRest { data, history, slots, slot_data, centers, offers, offer_records, field_8f })
    }

    pub fn history(&self) -> &'a [u8] {
        &self.data[self.history.clone()]
    }

    pub fn center(&self, i: usize) -> &'a [u8] {
        &self.data[self.centers[i].clone()]
    }

    /// Cada imagen contiene los 16 bytes del nodo vivo a partir de +4;
    /// los dos bytes reservados del flujo no forman parte del nodo.
    pub fn offer_records(&self) -> Vec<[u8; 16]> {
        self.offer_records.iter()
            .map(|range| self.data[range.clone()].try_into().unwrap())
            .collect()
    }

    pub fn slot(&self, i: usize) -> &'a [u8] {
        &self.data[self.slot_data[i].clone()]
    }
}

/// Sustituye la lista sólo si el estado guardado coincide con la base o el
/// resultado anterior. Los demás campos de la cola se conservan intactos.
pub fn replace_offers(data: &mut Vec<u8>, before: &[[u8; 16]], after: &[[u8; 16]]) -> Result<(), String> {
    let parsed = WorldRest::parse(data).map_err(|e| format!("cola de mundo: {e:?}"))?;
    let current = parsed.offer_records();
    if current != before && current != after {
        return Err("ofertas comerciales modificadas por otro sistema".into());
    }
    if current == after {
        return Ok(());
    }
    let range = parsed.offers.clone();
    let mut output = Vec::with_capacity(1 + OFFER_LEN * after.len());
    output.push(u8::from(!after.is_empty()));
    for (index, offer) in after.iter().enumerate() {
        output.extend_from_slice(offer);
        output.extend_from_slice(&[0, 0]);
        output.push(u8::from(index + 1 < after.len()));
    }
    data.splice(range, output);
    Ok(())
}
