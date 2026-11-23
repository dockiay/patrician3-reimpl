//! WEB-7: transporte por repetidor (WebSocket) para el lockstep.
//!
//! Un navegador no puede abrir TCP ni escuchar puertos, así que todas las máquinas se
//! conectan por WebSocket a un **repetidor** que solo reenvía mensajes dentro de una sala.
//! El repetidor no ejecuta el juego, no abre las partidas ni mira el contenido: ve
//! `(origen, destino, bytes)`. La topología sigue siendo en estrella como en
//! [`crate::lockstep`]: el anfitrión (máquina 0) habla con todos y cada cliente solo con el
//! anfitrión. Este módulo no usa E/S ni hilos (vale en WASM); el servidor y el cliente
//! WebSocket nativos están en [`crate::ws`] tras la característica `ws`.
//!
//! Cada mensaje WebSocket es binario y lleva un byte de tipo:
//!
//! | Sentido | Byte | Contenido |
//! |---|---|---|
//! | cliente → repetidor | `1` Hello | `host: u8` (1 crea la sala), nombre de sala UTF-8 |
//! | cliente → repetidor | `2` Data | `to: u8`, bytes del [`Message`](crate::lockstep::Message) |
//! | cliente → repetidor | `3` Bye | |
//! | repetidor → cliente | `0x81` Assigned | `peer: u8` (0 anfitrión, 1..7 clientes) |
//! | repetidor → cliente | `0x82` Error | código (`ERR_*`) |
//! | repetidor → cliente | `0x83` Data | `from: u8`, bytes |
//! | repetidor → cliente | `0x84` PeerGone | `peer: u8` |
//!
//! La sala nace con el Hello del anfitrión y muere con él (los clientes reciben `PeerGone(0)`
//! y se les cierra la conexión). Una máquina que cae libera su número; si vuelve ocupa el
//! menor libre. Los números son los [`PeerId`] del lockstep.

use crate::lockstep::{Message, Port, MAX_SLOTS};
use crate::transport::{LinkError, PeerId};
use std::collections::{BTreeMap, VecDeque};

pub const C_HELLO: u8 = 1;
pub const C_DATA: u8 = 2;
pub const C_BYE: u8 = 3;
pub const S_ASSIGNED: u8 = 0x81;
pub const S_ERROR: u8 = 0x82;
pub const S_DATA: u8 = 0x83;
pub const S_GONE: u8 = 0x84;

/// El anfitrión pide crear una sala que ya existe.
pub const ERR_ROOM_EXISTS: u8 = 1;
/// El cliente pide una sala sin anfitrión.
pub const ERR_NO_ROOM: u8 = 2;
/// La sala tiene las 8 máquinas.
pub const ERR_FULL: u8 = 3;
/// Mensaje malformado o antes del Hello.
pub const ERR_PROTOCOL: u8 = 4;
/// Un cliente intentó hablar con otro cliente.
pub const ERR_NOT_HOST: u8 = 5;
/// Nombre de sala vacío o demasiado largo.
pub const ERR_ROOM_NAME: u8 = 6;

pub const MAX_ROOM_NAME: usize = 64;

pub type ConnId = u64;

/// Acción que el repetidor pide a su capa de E/S.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Out {
    Send(ConnId, Vec<u8>),
    Close(ConnId),
}

#[derive(Default)]
struct Room {
    /// `peers[i]` es la conexión de la máquina `i`.
    peers: [Option<ConnId>; MAX_SLOTS],
}

/// Estado del repetidor: salas y a qué sala/máquina pertenece cada conexión.
#[derive(Default)]
pub struct Relay {
    rooms: BTreeMap<String, Room>,
    conns: BTreeMap<ConnId, (String, PeerId)>,
    /// Mensajes y bytes reenviados (para informes).
    pub forwarded: u64,
    pub bytes: u64,
}

fn error(conn: ConnId, code: u8) -> [Out; 2] {
    [Out::Send(conn, vec![S_ERROR, code]), Out::Close(conn)]
}

impl Relay {
    pub fn new() -> Relay {
        Relay::default()
    }
    pub fn rooms(&self) -> usize {
        self.rooms.len()
    }
    pub fn connections(&self) -> usize {
        self.conns.len()
    }
    /// Máquinas conectadas a una sala.
    pub fn peers_in(&self, room: &str) -> Vec<PeerId> {
        self.rooms.get(room).map(|r| (0..MAX_SLOTS).filter(|&i| r.peers[i].is_some()).collect()).unwrap_or_default()
    }

    /// Procesa un mensaje WebSocket de `conn`.
    pub fn handle(&mut self, conn: ConnId, data: &[u8]) -> Vec<Out> {
        let Some((&kind, rest)) = data.split_first() else { return self.protocol_error(conn) };
        match kind {
            C_HELLO => self.hello(conn, rest),
            C_DATA => self.data(conn, rest),
            C_BYE => self.disconnected(conn),
            _ => self.protocol_error(conn),
        }
    }

    fn protocol_error(&mut self, conn: ConnId) -> Vec<Out> {
        let mut out = self.disconnected(conn);
        out.extend(error(conn, ERR_PROTOCOL));
        out
    }

    fn hello(&mut self, conn: ConnId, rest: &[u8]) -> Vec<Out> {
        if self.conns.contains_key(&conn) {
            return self.protocol_error(conn);
        }
        let Some((&host, name)) = rest.split_first() else { return self.protocol_error(conn) };
        let Ok(name) = std::str::from_utf8(name) else { return self.protocol_error(conn) };
        if name.is_empty() || name.len() > MAX_ROOM_NAME {
            return error(conn, ERR_ROOM_NAME).to_vec();
        }
        let peer = if host != 0 {
            if self.rooms.contains_key(name) {
                return error(conn, ERR_ROOM_EXISTS).to_vec();
            }
            self.rooms.insert(name.to_string(), Room::default());
            0
        } else {
            let Some(room) = self.rooms.get(name) else { return error(conn, ERR_NO_ROOM).to_vec() };
            match (1..MAX_SLOTS).find(|&i| room.peers[i].is_none()) {
                Some(i) => i,
                None => return error(conn, ERR_FULL).to_vec(),
            }
        };
        self.rooms.get_mut(name).unwrap().peers[peer] = Some(conn);
        self.conns.insert(conn, (name.to_string(), peer));
        vec![Out::Send(conn, vec![S_ASSIGNED, peer as u8])]
    }

    fn data(&mut self, conn: ConnId, rest: &[u8]) -> Vec<Out> {
        let Some((room, from)) = self.conns.get(&conn).cloned() else { return self.protocol_error(conn) };
        let Some((&to, payload)) = rest.split_first() else { return self.protocol_error(conn) };
        let to = usize::from(to);
        if to >= MAX_SLOTS || to == from {
            return self.protocol_error(conn);
        }
        if from != 0 && to != 0 {
            let mut out = self.disconnected(conn);
            out.extend(error(conn, ERR_NOT_HOST));
            return out;
        }
        // Un destino ausente no es un error: el lockstep ya lo trata como corte.
        let Some(target) = self.rooms.get(&room).and_then(|r| r.peers[to]) else { return Vec::new() };
        self.forwarded += 1;
        self.bytes += payload.len() as u64;
        let mut msg = Vec::with_capacity(payload.len() + 2);
        msg.extend_from_slice(&[S_DATA, from as u8]);
        msg.extend_from_slice(payload);
        vec![Out::Send(target, msg)]
    }

    /// La conexión se cerró (o dijo adiós). Si era el anfitrión, la sala desaparece.
    pub fn disconnected(&mut self, conn: ConnId) -> Vec<Out> {
        let Some((name, peer)) = self.conns.remove(&conn) else { return Vec::new() };
        let mut out = Vec::new();
        if peer == 0 {
            if let Some(room) = self.rooms.remove(&name) {
                for c in room.peers.into_iter().flatten().filter(|&c| c != conn) {
                    self.conns.remove(&c);
                    out.push(Out::Send(c, vec![S_GONE, 0]));
                    out.push(Out::Close(c));
                }
            }
        } else if let Some(room) = self.rooms.get_mut(&name) {
            room.peers[peer] = None;
            if let Some(host) = room.peers[0] {
                out.push(Out::Send(host, vec![S_GONE, peer as u8]));
            }
        }
        out
    }
}

/// Enlace de mensajes completos con el repetidor (un WebSocket binario).
pub trait Duplex {
    fn send(&mut self, message: &[u8]) -> Result<(), LinkError>;
    /// Mensajes ya llegados, sin bloquear. `Err` cuando la conexión se ha cerrado.
    fn receive(&mut self) -> Result<Vec<Vec<u8>>, LinkError>;
}

/// Puerto del lockstep sobre un enlace con el repetidor.
pub struct RelayPort<L: Duplex> {
    link: L,
    /// Número de máquina asignado por el repetidor (`None` hasta recibir `Assigned`).
    pub peer: Option<PeerId>,
    /// Código `ERR_*` recibido, si el repetidor rechazó la conexión.
    pub error: Option<u8>,
    pub closed: bool,
    /// Mensajes de datos que llegaron antes de saber el número propio (no ocurre: el
    /// repetidor envía `Assigned` primero), conservados por si acaso.
    early: VecDeque<(PeerId, Message)>,
    pub sent: u64,
    pub received: u64,
}

impl<L: Duplex> RelayPort<L> {
    /// Abre la sala como anfitrión (`host`) o se une a ella; envía el Hello.
    pub fn connect(mut link: L, room: &str, host: bool) -> Result<RelayPort<L>, LinkError> {
        let mut hello = vec![C_HELLO, u8::from(host)];
        hello.extend_from_slice(room.as_bytes());
        link.send(&hello)?;
        Ok(RelayPort { link, peer: None, error: None, closed: false, early: VecDeque::new(), sent: 0, received: 0 })
    }
    pub fn link(&mut self) -> &mut L {
        &mut self.link
    }
    /// Despedida ordenada.
    pub fn bye(&mut self) {
        let _ = self.link.send(&[C_BYE]);
        self.closed = true;
    }
    fn pump(&mut self) {
        match self.link.receive() {
            Err(_) => self.closed = true,
            Ok(messages) => {
                for m in messages {
                    match m.split_first() {
                        Some((&S_ASSIGNED, [p])) => self.peer = Some(usize::from(*p)),
                        Some((&S_ERROR, [c])) => {
                            self.error = Some(*c);
                            self.closed = true;
                        }
                        Some((&S_DATA, [from, payload @ ..])) => {
                            if let Ok(msg) = Message::decode(payload) {
                                self.received += 1;
                                self.early.push_back((usize::from(*from), msg));
                            }
                        }
                        // PeerGone: el lockstep lo detecta por tiempo de espera, igual que
                        // con TCP; se deja aquí como señal por si el cliente quiere usarla.
                        _ => {}
                    }
                }
            }
        }
    }
}

impl<L: Duplex> Port for RelayPort<L> {
    fn send(&mut self, to: PeerId, message: &Message) -> Result<(), LinkError> {
        if self.closed {
            return Err(LinkError::Disconnected);
        }
        let payload = message.encode();
        let mut m = Vec::with_capacity(payload.len() + 2);
        m.extend_from_slice(&[C_DATA, to as u8]);
        m.extend_from_slice(&payload);
        self.sent += 1;
        self.link.send(&m)
    }
    fn receive(&mut self) -> Vec<(PeerId, Message)> {
        self.pump();
        self.early.drain(..).collect()
    }
}
