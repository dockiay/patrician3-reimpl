//! Sesión del anfitrión original: recogida de paquetes de los clientes con el apretón de
//! manos del estado 2 (versión y contraseña) y el orden del flujo (PRE-SPEC-NET §3–§4).
//!
//! Portado de `host_receive_from_clients` (0x0054B670) con contraste x86 en
//! `oracle/x86_net_host.py`: ranuras 1..7, estado de conexión en `conn+0x18`, contraseña
//! (`CString` de `OPS+0x40`) en registros `0xCE` de 14 bytes con su desplazamiento en
//! `u16 +4`, versión `0x85` en `0xC3`, rechazo reescrito a `0xD0 {ranura, razón}` +
//! `0xC7 {ranura, 0xFFFF, 1}`; los paquetes de clientes van a `Q_out` por orden de ranura
//! y el paquete de cabeza del anfitrión al final.

use crate::queue::OpQueue;
use crate::transfer::{STATE_SEND_GAME, STATE_SEND_SETTINGS, STATE_STABLE};
use crate::wire::{Packet, PROTOCOL_VERSION};

pub const OP_LEAVE: u32 = 0xC7;
pub const OP_VERSION: u32 = 0xC3;
pub const OP_PASSWORD: u32 = 0xCE;
pub const OP_PASSWORD_DONE: u32 = 0xCF;
pub const OP_REJECT: u32 = 0xD0;
pub const OP_START: u32 = 0xC6;
pub const OP_CONNECTION_LOST: u32 = 0xD3;
pub const OP_ADVANCE_TIME: u32 = 0xC4;
/// Bytes de contraseña por registro `0xCE`.
pub const PASSWORD_CHUNK: usize = 14;
/// Errores del apretón de manos (`errors.tsv`).
pub const ERR_VERSION: u32 = 0x7530;
pub const ERR_PASSWORD: u32 = 0x4E20;
pub const STATE_CLOSED: u32 = 0;
pub const STATE_FREE: u32 = 1;
pub const STATE_HANDSHAKE: u32 = 2;
/// Razones de `0xD0`.
pub const REJECT_VERSION: u32 = 0;
pub const REJECT_FULL: u32 = 1;
pub const REJECT_PASSWORD: u32 = 2;

/// Lo que el anfitrión necesita del objeto de operaciones.
#[derive(Clone, Debug)]
pub struct HostSession {
    /// `OPS+0x40`.
    pub password: Vec<u8>,
    /// `OPS+0x14 != 0`: hay partida cargada que transferir.
    pub has_game_snapshot: bool,
    /// `OPS+0xC != 0`: hay mapa personalizado que enviar (registros `0xCB`, `0x0054BF80`)
    /// en lugar de partida.
    pub custom_map: bool,
    /// `conn[i]+0x18` para i = 0..7 (la 0 es la conexión de rechazo).
    pub states: [u32; 8],
    /// `OPS+0x48`: paquete de cabeza del anfitrión.
    pub head: Packet,
    /// `OPS+0x8B8`.
    pub q_out: OpQueue,
}

/// Resultado de la comprobación del estado 2 (valor final de `ecx`).
pub fn handshake_code(packet: &mut Packet, password: &[u8]) -> u32 {
    let count = i32::from(packet.count);
    if password.is_empty() {
        let mut code = 0xE;
        for i in 0..count.min(52) as usize {
            let r = &mut packet.records[i];
            if r[0] == OP_PASSWORD {
                r[0] = OP_PASSWORD_DONE;
            } else if r[0] == OP_VERSION && r[1] != PROTOCOL_VERSION {
                code = ERR_VERSION;
            }
        }
        return code;
    }
    let len = password.len() as i64;
    let mut code: u32 = 0;
    for i in 0..count.min(52) as usize {
        let r = &mut packet.records[i];
        if r[0] == OP_PASSWORD {
            let offset = r[1] & 0xFFFF;
            if offset != code {
                code = ERR_PASSWORD;
            } else {
                let mut bytes = [0u8; 16];
                for (k, w) in r[1..5].iter().enumerate() {
                    bytes[4 * k..4 * k + 4].copy_from_slice(&w.to_le_bytes());
                }
                for &b in &bytes[2..2 + PASSWORD_CHUNK] {
                    let ok = if i64::from(code as i32) < len { password[code as usize] == b } else { b == 0 };
                    code = if ok { code.wrapping_add(1) } else { ERR_PASSWORD };
                }
            }
            r[0] = OP_PASSWORD_DONE;
        } else if r[0] == OP_VERSION && r[1] != PROTOCOL_VERSION {
            code = ERR_VERSION;
        }
    }
    code
}

/// Código esperado: `14 · (longitud / 14 + 1)` (aritmética con signo del original).
pub fn expected_code(password_len: usize) -> u32 {
    let len = password_len as i32;
    (14 * (len / 14 + 1)) as u32
}

/// Divide la contraseña en registros `0xCE` como el cliente (14 bytes por registro,
/// desplazamiento en `u16 +4`, relleno con ceros y siempre un registro de cierre).
pub fn password_records(password: &[u8]) -> Vec<crate::wire::Record> {
    let mut out = Vec::new();
    let mut offset = 0usize;
    loop {
        let mut bytes = [0u8; 16];
        bytes[..2].copy_from_slice(&(offset as u16).to_le_bytes());
        let end = (offset + PASSWORD_CHUNK).min(password.len());
        if offset < password.len() {
            bytes[2..2 + end - offset].copy_from_slice(&password[offset..end]);
        }
        let mut r = [OP_PASSWORD, 0, 0, 0, 0];
        for k in 0..4 {
            r[1 + k] = u32::from_le_bytes(bytes[4 * k..4 * k + 4].try_into().unwrap());
        }
        out.push(r);
        offset += PASSWORD_CHUNK;
        if offset > password.len() {
            return out;
        }
    }
}

impl HostSession {
    pub fn new(password: &[u8]) -> HostSession {
        let mut states = [STATE_FREE; 8];
        states[0] = STATE_CLOSED;
        HostSession { password: password.to_vec(), has_game_snapshot: true, custom_map: false, states, head: Packet::new(), q_out: OpQueue::new() }
    }

    /// Una pasada de `host_receive_from_clients`. `receive(ranura)` devuelve el paquete
    /// recibido (si `ReceiveMessage` devolvió 1) y `send_custom_map(ranura)` el código de
    /// `0x0054BF80` (envío del mapa personalizado). Devuelve las ranuras cuyo paquete se recogió.
    pub fn receive_from_clients(
        &mut self,
        mut receive: impl FnMut(usize) -> Option<Vec<u8>>,
        mut send_custom_map: impl FnMut(usize) -> u32,
    ) -> Vec<usize> {
        let mut collected = Vec::new();
        // Un único búfer de pila para las siete ranuras: un mensaje corto deja los bytes
        // de la ranura anterior (y las reescrituras de rechazo o 0xCF) tras su final.
        let mut packet = Packet::new();
        for slot in 1..8 {
            if self.states[slot] <= STATE_FREE {
                continue;
            }
            let Some(bytes) = receive(slot) else { continue };
            let mut buffer = packet.to_full();
            let n = bytes.len().min(buffer.len());
            buffer[..n].copy_from_slice(&bytes[..n]);
            packet = Packet::from_wire(&buffer).expect("búfer de 0x420 bytes");
            if self.states[slot] == STATE_HANDSHAKE {
                let code = handshake_code(&mut packet, &self.password);
                if code != expected_code(self.password.len()) {
                    let reason = if code != ERR_VERSION { REJECT_PASSWORD } else { REJECT_VERSION };
                    packet.records[0][0] = OP_REJECT;
                    packet.records[0][1] = slot as u32;
                    packet.records[0][2] = reason;
                    packet.records[1][0] = OP_LEAVE;
                    packet.records[1][1] = slot as u32;
                    packet.records[1][2] = 0xFFFF;
                    packet.records[1][3] = 1;
                    packet.count = 2;
                    self.states[slot] = STATE_STABLE;
                } else if self.has_game_snapshot {
                    self.states[slot] = STATE_SEND_GAME;
                } else if self.custom_map {
                    match send_custom_map(slot) {
                        1 => self.states[slot] = STATE_SEND_SETTINGS,
                        0x320 | 0x3E7 => self.states[slot] = STATE_FREE,
                        _ => {}
                    }
                } else {
                    self.states[slot] = STATE_FREE;
                }
            }
            self.q_out.push_packet(&mut packet);
            collected.push(slot);
        }
        let mut head = std::mem::take(&mut self.head);
        self.q_out.push_packet(&mut head);
        // 0x00544040: la cabeza recibe los registros del paquete encadenado.
        head.promote_chain();
        self.head = head;
        collected
    }
}
