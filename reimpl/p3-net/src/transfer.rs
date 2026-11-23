//! Transferencia masiva al unirse (PRE-SPEC-NET §3): la partida `.pat` cargada por el
//! anfitrión, sus estadísticas `.pst` y los 0x4C bytes de ajustes, en paquetes de
//! `0x420` bytes con el bit 15 del recuento.
//!
//! Portado de `net_send_state_transfer_chunk` (0x00544160) y
//! `net_receive_state_transfer_packet` (0x00544300), con contraste x86 en
//! `oracle/x86_net_transfer.py`. Campos de la conexión: `+0x14` contador, `+0x18` estado,
//! `+0x1C` posición, `+0x20` créditos (emisor) o trozos desde el último acuse (receptor).

use crate::wire::{Packet, Record, EMPTY_RECORD, HEADER_LEN, MARK_GAME, MARK_SETTINGS, MARK_STATISTICS, PACKET_LEN};

/// Estados de conexión con transferencia.
pub const STATE_STABLE: u32 = 3;
pub const STATE_SEND_GAME: u32 = 4;
pub const STATE_SEND_STATISTICS: u32 = 5;
pub const STATE_SEND_SETTINGS: u32 = 6;
/// Tamaño de un trozo de datos y de los ajustes.
pub const CHUNK: usize = 0x400;
pub const SETTINGS_LEN: usize = 0x4C;
/// Opcode del acuse.
pub const OP_TRANSFER_ACK: u32 = 0xC5;

/// Resultado de `Network_SendMessage`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SendResult {
    Ok,
    /// `0x320` (socket -1) o `0x3E7` (otro error): la transferencia se abandona (estado 3).
    Fatal(u32),
    /// Cualquier otro código: no se avanza y se reintenta en la siguiente pasada.
    Retry(u32),
}

impl SendResult {
    pub fn from_code(code: u32) -> SendResult {
        match code {
            1 => SendResult::Ok,
            0x320 | 0x3E7 => SendResult::Fatal(code),
            other => SendResult::Retry(other),
        }
    }
}

/// Campos de la conexión del anfitrión que usa el emisor.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SenderConn {
    pub counter: u32,
    pub state: u32,
    pub offset: u32,
    pub credits: u32,
}

/// Datos que envía el anfitrión (`OPS+0x14/+0x18`, tamaños `0x6DF30C/0x6DF310`, ajustes `0x6CC3E8`).
pub struct Snapshot<'a> {
    pub game: &'a [u8],
    pub statistics: &'a [u8],
    pub settings: &'a [u8; SETTINGS_LEN],
}

/// Prepara un trozo del emisor. Devuelve `None` sin créditos (no se envía nada); si no, el
/// paquete de `0x420` bytes. Tras enviarlo hay que llamar a [`sender_after_send`].
pub fn sender_packet(conn: &SenderConn, slot: u16, data: &Snapshot) -> Option<(Vec<u8>, u32)> {
    if conn.credits == 0 {
        return None;
    }
    let mut p = Packet::new();
    p.stamp = conn.counter.wrapping_add(1);
    p.slot = slot;
    let mut bytes;
    let next_offset;
    match conn.state {
        STATE_SEND_GAME | STATE_SEND_STATISTICS => {
            let (mark, source) = if conn.state == STATE_SEND_GAME { (MARK_GAME, data.game) } else { (MARK_STATISTICS, data.statistics) };
            p.count = mark;
            p.records[0][0] = source.len() as u32;
            bytes = p.to_full();
            let offset = conn.offset as usize;
            let total = source.len();
            let take = if offset + CHUNK < total {
                next_offset = (offset + CHUNK) as u32;
                CHUNK
            } else {
                next_offset = 0;
                total.saturating_sub(offset)
            };
            let start = offset.min(total);
            bytes[HEADER_LEN + 4..HEADER_LEN + 4 + take].copy_from_slice(&source[start..start + take]);
        }
        _ => {
            // Cualquier otro estado envía los ajustes (el original no comprueba que sea 6).
            p.count = MARK_SETTINGS;
            bytes = p.to_full();
            bytes[HEADER_LEN..HEADER_LEN + SETTINGS_LEN].copy_from_slice(data.settings);
            next_offset = 0;
        }
    }
    Some((bytes, next_offset))
}

/// Aplica el resultado del envío del trozo preparado con `next_offset`.
/// Los ajustes y el último trozo de estadísticas comparten en el original la cola que
/// escribe `+0x20 = 1` antes de enviar: reponen un crédito aunque el envío falle.
/// Devuelve si la función original devolvería verdadero.
pub fn sender_after_send(conn: &mut SenderConn, next_offset: u32, result: SendResult) -> bool {
    let settings = !matches!(conn.state, STATE_SEND_GAME | STATE_SEND_STATISTICS);
    if settings || (conn.state == STATE_SEND_STATISTICS && next_offset == 0) {
        conn.credits = 1;
    }
    match result {
        SendResult::Fatal(_) => {
            conn.state = STATE_STABLE;
            false
        }
        SendResult::Retry(_) => true,
        SendResult::Ok => {
            conn.credits = conn.credits.wrapping_sub(1);
            conn.counter = conn.counter.wrapping_add(1);
            if next_offset == 0 {
                conn.state = if conn.state == STATE_SEND_GAME { STATE_SEND_STATISTICS } else { STATE_STABLE };
            }
            conn.offset = next_offset;
            true
        }
    }
}

/// `0xC5` en el anfitrión: créditos a 4 (solo rol 0 y ranura < 8).
pub fn on_transfer_ack(conn: &mut SenderConn) {
    conn.credits = 4;
}

/// Receptor del cliente.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Receiver {
    pub state: u32,
    pub offset: u32,
    pub chunks_since_ack: u32,
    /// Búfer de la partida (`0x6DF304`, tamaño `0x6DF30C`).
    pub game: Option<Vec<u8>>,
    /// Búfer de estadísticas (`0x6DF308`, tamaño `0x6DF310`).
    pub statistics: Option<Vec<u8>>,
    pub settings: Option<[u8; SETTINGS_LEN]>,
}

/// Qué ocurrió con un paquete de transferencia recibido.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ReceiveEffects {
    /// Valor devuelto por la función original.
    pub accepted: bool,
    /// `*out = 1` (paquete de datos consumido).
    pub data: bool,
    /// Registros encolados (acuses `0xC5 {rol}`).
    pub enqueued: Vec<Record>,
    /// Partida completa recibida (se carga con `0x00544590`).
    pub game_complete: Option<Vec<u8>>,
    /// Estadísticas completas (se cargan con `0x00544730`).
    pub statistics_complete: Option<Vec<u8>>,
    /// Ajustes recibidos.
    pub settings: bool,
}

impl Receiver {
    /// `net_receive_state_transfer_packet`. `role` es `[0x6DF31C]`, que viaja en el acuse.
    /// `load_game` modela el valor de `0x00544590` (cargar la partida recibida).
    pub fn receive(&mut self, packet: &[u8], role: u32, load_game: impl FnOnce(&[u8]) -> bool) -> ReceiveEffects {
        let mut fx = ReceiveEffects::default();
        if packet.len() < PACKET_LEN {
            return fx;
        }
        let mark = u16::from_le_bytes([packet[0xE], packet[0xF]]);
        let word = |at: usize| u32::from_le_bytes(packet[at..at + 4].try_into().unwrap());
        if self.offset == 0 {
            match mark {
                MARK_GAME => {
                    self.state = STATE_SEND_GAME;
                    self.chunks_since_ack = 0;
                    self.game = Some(vec![0; word(0x10) as usize]);
                }
                MARK_STATISTICS => {
                    self.state = STATE_SEND_STATISTICS;
                    self.statistics = Some(vec![0; word(0x10) as usize]);
                }
                MARK_SETTINGS => {
                    let mut s = [0u8; SETTINGS_LEN];
                    s.copy_from_slice(&packet[HEADER_LEN..HEADER_LEN + SETTINGS_LEN]);
                    self.settings = Some(s);
                    self.state = STATE_STABLE;
                    fx.accepted = true;
                    fx.settings = true;
                    return fx;
                }
                _ => return fx,
            }
        }
        fx.data = true;
        let total = word(0x10);
        let remaining = total.wrapping_sub(self.offset);
        let ack: Record = [OP_TRANSFER_ACK, role, 0, 0, 0];
        let payload = &packet[HEADER_LEN + 4..];
        if remaining > CHUNK as u32 {
            let buffer = if self.state == STATE_SEND_GAME { self.game.as_mut() } else { self.statistics.as_mut() };
            if let Some(buffer) = buffer {
                let at = self.offset as usize;
                if at + CHUNK <= buffer.len() {
                    buffer[at..at + CHUNK].copy_from_slice(&payload[..CHUNK]);
                }
            }
            self.offset += CHUNK as u32;
            self.chunks_since_ack += 1;
            if self.chunks_since_ack >= 4 {
                self.chunks_since_ack = 0;
                fx.enqueued.push(ack);
            }
            fx.accepted = true;
            return fx;
        }
        let at = self.offset as usize;
        let n = remaining as usize;
        match self.state {
            STATE_SEND_GAME => {
                let Some(mut buffer) = self.game.take() else { return fx };
                if at + n <= buffer.len() {
                    buffer[at..at + n].copy_from_slice(&payload[..n]);
                }
                fx.accepted = load_game(&buffer);
                fx.game_complete = Some(buffer);
                self.state = STATE_SEND_STATISTICS;
                self.offset = 0;
                self.chunks_since_ack = 0;
                fx.enqueued.push(ack);
            }
            STATE_SEND_STATISTICS => {
                let Some(mut buffer) = self.statistics.take() else { return fx };
                if at + n <= buffer.len() {
                    buffer[at..at + n].copy_from_slice(&payload[..n]);
                }
                fx.statistics_complete = Some(buffer);
                self.state = STATE_STABLE;
                self.offset = 0;
                fx.accepted = true;
            }
            _ => {}
        }
        fx
    }
}

/// Registro vacío reutilizado por el acuse (`operation_record_init`).
pub const fn empty_record() -> Record {
    EMPTY_RECORD
}
