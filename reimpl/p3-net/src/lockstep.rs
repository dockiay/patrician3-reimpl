//! M02: multijugador entre clientes nuevos.
//!
//! **Autoridad y orden.** Como en el original, el anfitrión es el único secuenciador: cada
//! máquina le envía sus órdenes (registros de 20 bytes) y el anfitrión difunde una
//! *pasada* con las de los clientes por orden de ranura, las suyas y al final el avance
//! de tiempo `0xC4 {tick, tick+n}`. Todas las máquinas, incluido el anfitrión, ejecutan
//! solo lo que llega en las pasadas y en el mismo orden; nadie ejecuta una orden al
//! encolarla. La simulación debe ser determinista: el azar va en el estado (CRT del
//! simulador) o viaja como registro, nunca se sortea fuera de una pasada.
//!
//! **Estado inicial.** Al unirse, el anfitrión toma una instantánea en el límite de una
//! pasada ([`Game::snapshot`]) y la envía con su hash; el cliente la restaura y
//! comprueba el hash antes de ejecutar pasadas. Con `resync_on_join` el anfitrión y
//! todos los clientes restauran la misma instantánea, de modo que la igualdad no
//! depende de que la instantánea capture todo el estado vivo. A diferencia del original
//! (sin unión tardía tras `0xC6`), se admite unirse o volver a mitad de partida.
//!
//! **Divergencias.** Cada `hash_every` pasadas cada cliente envía el hash de su estado
//! completo tras ejecutarlas ([`Game::state_hash`]); el anfitrión lo compara con el suyo
//! de la misma pasada e informa [`Event::Desync`] con la primera pasada distinta. Es más
//! estricto que la suma `0xD4` del original (cada diez días, solo existencias, flota y
//! carga): ver [`crate::sync`] para responder a un anfitrión original.
//!
//! **Latencia y cortes.** Una ventana de `window` pasadas limita cuánto puede adelantarse
//! el anfitrión al cliente más lento (el original no espera: el cliente lento acumula
//! retraso). Una máquina sin mensajes durante `timeout_ms` se da por perdida; el
//! anfitrión publica `OP_PEER_LOST {ranura}` en la siguiente pasada (equivalente a
//! `0xD3`), de modo que todas las máquinas aplican la baja en el mismo punto, y sigue sin
//! ella. Un cliente que pierde al anfitrión se detiene ([`Event::HostLost`]): no hay
//! migración de anfitrión. `Leave` produce `OP_PEER_LEFT` (equivalente a `0xC7`).

use crate::transport::{LinkError, MemoryNet, PeerId};
use crate::wire::Record;
use std::collections::BTreeMap;

pub use crate::session::OP_ADVANCE_TIME;
/// Baja ordenada de una ranura (como `0xC7`).
pub const OP_PEER_LEFT: u32 = 0x1C7;
/// Ranura perdida por tiempo de espera (como `0xD3`).
pub const OP_PEER_LOST: u32 = 0x1D3;
/// Alta de una ranura en la partida.
pub const OP_PEER_JOINED: u32 = 0x1C6;
/// Versión del protocolo nuevo.
pub const LOCKSTEP_VERSION: u32 = 1;
pub const MAX_SLOTS: usize = 8;

/// Simulación replicada.
pub trait Game {
    /// Tick del mundo (origen del siguiente `0xC4`).
    fn tick(&self) -> u32;
    /// Ejecuta un registro de una pasada. Un error se informa pero no detiene el flujo:
    /// debe ser igual en todas las máquinas (es parte de la simulación).
    fn apply(&mut self, record: &Record) -> Result<(), String>;
    fn state_hash(&self) -> [u8; 32];
    fn snapshot(&self) -> Result<Vec<u8>, String>;
    fn restore(&mut self, snapshot: &[u8]) -> Result<(), String>;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Message {
    Join { version: u32, name: Vec<u8> },
    Reject { reason: u32 },
    Welcome { slot: u32, pass: u64, hash: [u8; 32], snapshot: Vec<u8> },
    Resync { pass: u64, hash: [u8; 32], snapshot: Vec<u8> },
    Commands { acked: u64, records: Vec<Record> },
    Pass { index: u64, records: Vec<Record> },
    Hash { pass: u64, hash: [u8; 32] },
    Leave,
}

pub const REJECT_FULL: u32 = 1;
/// No se pudo tomar la instantánea ahora (p. ej. batalla en pausa): el cliente reintenta.
pub const REJECT_BUSY: u32 = 3;
/// Espera del cliente antes de reintentar tras `REJECT_BUSY`.
pub const RETRY_MS: u64 = 500;
pub const REJECT_VERSION: u32 = 0;

fn put_records(out: &mut Vec<u8>, records: &[Record]) {
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        for w in r {
            out.extend_from_slice(&w.to_le_bytes());
        }
    }
}

struct Reader<'a>(&'a [u8]);
impl Reader<'_> {
    fn take(&mut self, n: usize) -> Result<&[u8], &'static str> {
        if self.0.len() < n {
            return Err("mensaje truncado");
        }
        let (a, b) = self.0.split_at(n);
        self.0 = b;
        Ok(a)
    }
    fn u32(&mut self) -> Result<u32, &'static str> {
        Ok(u32::from_le_bytes(self.take(4)?.try_into().unwrap()))
    }
    fn u64(&mut self) -> Result<u64, &'static str> {
        Ok(u64::from_le_bytes(self.take(8)?.try_into().unwrap()))
    }
    fn hash(&mut self) -> Result<[u8; 32], &'static str> {
        Ok(self.take(32)?.try_into().unwrap())
    }
    fn bytes(&mut self) -> Result<Vec<u8>, &'static str> {
        let n = self.u32()? as usize;
        Ok(self.take(n)?.to_vec())
    }
    fn records(&mut self) -> Result<Vec<Record>, &'static str> {
        let n = self.u32()? as usize;
        if n > 1 << 20 {
            return Err("demasiados registros");
        }
        (0..n).map(|_| Ok([self.u32()?, self.u32()?, self.u32()?, self.u32()?, self.u32()?])).collect()
    }
}

impl Message {
    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::new();
        match self {
            Message::Join { version, name } => {
                out.push(1);
                out.extend_from_slice(&version.to_le_bytes());
                out.extend_from_slice(&(name.len() as u32).to_le_bytes());
                out.extend_from_slice(name);
            }
            Message::Reject { reason } => {
                out.push(2);
                out.extend_from_slice(&reason.to_le_bytes());
            }
            Message::Welcome { slot, pass, hash, snapshot } => {
                out.push(3);
                out.extend_from_slice(&slot.to_le_bytes());
                out.extend_from_slice(&pass.to_le_bytes());
                out.extend_from_slice(hash);
                out.extend_from_slice(&(snapshot.len() as u32).to_le_bytes());
                out.extend_from_slice(snapshot);
            }
            Message::Resync { pass, hash, snapshot } => {
                out.push(4);
                out.extend_from_slice(&pass.to_le_bytes());
                out.extend_from_slice(hash);
                out.extend_from_slice(&(snapshot.len() as u32).to_le_bytes());
                out.extend_from_slice(snapshot);
            }
            Message::Commands { acked, records } => {
                out.push(5);
                out.extend_from_slice(&acked.to_le_bytes());
                put_records(&mut out, records);
            }
            Message::Pass { index, records } => {
                out.push(6);
                out.extend_from_slice(&index.to_le_bytes());
                put_records(&mut out, records);
            }
            Message::Hash { pass, hash } => {
                out.push(7);
                out.extend_from_slice(&pass.to_le_bytes());
                out.extend_from_slice(hash);
            }
            Message::Leave => out.push(8),
        }
        out
    }

    pub fn decode(bytes: &[u8]) -> Result<Message, &'static str> {
        let (&kind, rest) = bytes.split_first().ok_or("mensaje vacío")?;
        let mut r = Reader(rest);
        let m = match kind {
            1 => Message::Join { version: r.u32()?, name: r.bytes()? },
            2 => Message::Reject { reason: r.u32()? },
            3 => Message::Welcome { slot: r.u32()?, pass: r.u64()?, hash: r.hash()?, snapshot: r.bytes()? },
            4 => Message::Resync { pass: r.u64()?, hash: r.hash()?, snapshot: r.bytes()? },
            5 => Message::Commands { acked: r.u64()?, records: r.records()? },
            6 => Message::Pass { index: r.u64()?, records: r.records()? },
            7 => Message::Hash { pass: r.u64()?, hash: r.hash()? },
            8 => Message::Leave,
            _ => return Err("tipo de mensaje desconocido"),
        };
        if !r.0.is_empty() {
            return Err("bytes sobrantes en el mensaje");
        }
        Ok(m)
    }
}

/// Canal de una máquina.
pub trait Port {
    fn send(&mut self, to: PeerId, message: &Message) -> Result<(), LinkError>;
    fn receive(&mut self) -> Vec<(PeerId, Message)>;
}

/// Puerto sobre la red simulada.
pub struct MemoryPort<'a> {
    pub net: &'a mut MemoryNet,
    pub me: PeerId,
}

impl Port for MemoryPort<'_> {
    fn send(&mut self, to: PeerId, message: &Message) -> Result<(), LinkError> {
        self.net.send(self.me, to, message.encode())
    }
    fn receive(&mut self) -> Vec<(PeerId, Message)> {
        self.net.receive(self.me).into_iter().filter_map(|(from, m)| Message::decode(&m).ok().map(|m| (from, m))).collect()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Event {
    Joined { slot: usize, pass: u64 },
    Rejected { reason: u32 },
    Left { slot: usize, pass: u64 },
    PeerLost { slot: usize, pass: u64 },
    HostLost { pass: u64 },
    /// Hash distinto en `pass` entre el anfitrión y la ranura.
    Desync { slot: usize, pass: u64 },
    /// La instantánea recibida no reproduce el hash anunciado.
    SnapshotMismatch { pass: u64 },
    /// Error de la simulación al ejecutar un registro (igual en todas las máquinas).
    GameError { pass: u64, record: Record, error: String },
    Resynced { pass: u64 },
}

#[derive(Clone, Debug)]
pub struct Config {
    /// Ticks por pasada (el original admite hasta 8 en modo normal).
    pub ticks_per_pass: u32,
    /// Milisegundos de reloj entre pasadas.
    pub pass_ms: u64,
    /// Pasadas que el anfitrión puede adelantarse al último acuse de un cliente.
    pub window: u64,
    /// Cada cuántas pasadas se compara el hash completo.
    pub hash_every: u64,
    pub timeout_ms: u64,
    /// Todos restauran la instantánea al unirse alguien.
    pub resync_on_join: bool,
    /// Pasadas de hash conservadas por el anfitrión.
    pub history: usize,
}

impl Default for Config {
    fn default() -> Self {
        Config { ticks_per_pass: 8, pass_ms: 40, window: 8, hash_every: 1, timeout_ms: 3000, resync_on_join: true, history: 4096 }
    }
}

#[derive(Clone, Debug)]
struct Peer {
    id: PeerId,
    acked: u64,
    last_seen: u64,
    pending: Vec<Record>,
}

/// Anfitrión: secuenciador y referencia de hashes.
pub struct Host<G: Game> {
    pub game: G,
    pub config: Config,
    /// Pasadas ejecutadas.
    pub pass: u64,
    slots: [Option<Peer>; MAX_SLOTS],
    own: Vec<Record>,
    notices: Vec<Record>,
    hashes: BTreeMap<u64, [u8; 32]>,
    /// Última pasada en que todos restauraron una instantánea: los hashes de clientes
    /// hasta ella se calcularon antes de restaurar y no se comparan.
    resync_floor: Option<u64>,
    next_pass_at: u64,
    pub events: Vec<Event>,
    /// Registros difundidos (todas las pasadas), para informes.
    pub records_sent: u64,
    /// Pasadas aplazadas por la ventana.
    pub stalls: u64,
}

impl<G: Game> Host<G> {
    pub fn new(game: G, config: Config) -> Host<G> {
        Host { game, config, pass: 0, slots: Default::default(), own: Vec::new(), notices: Vec::new(), hashes: BTreeMap::new(), resync_floor: None, next_pass_at: 0, events: Vec::new(), records_sent: 0, stalls: 0 }
    }

    /// Orden del jugador del anfitrión (ranura 0).
    pub fn command(&mut self, record: Record) {
        self.own.push(record);
    }

    pub fn live_slots(&self) -> Vec<usize> {
        (1..MAX_SLOTS).filter(|&s| self.slots[s].is_some()).collect()
    }

    /// Hashes del anfitrión por pasada (para la comprobación cruzada).
    pub fn hashes(&self) -> &BTreeMap<u64, [u8; 32]> {
        &self.hashes
    }
    pub fn hash_at(&self, pass: u64) -> Option<[u8; 32]> {
        self.hashes.get(&pass).copied()
    }

    fn slot_of(&self, peer: PeerId) -> Option<usize> {
        (1..MAX_SLOTS).find(|&s| self.slots[s].as_ref().is_some_and(|p| p.id == peer))
    }

    fn remember_hash(&mut self) {
        if self.pass % self.config.hash_every == 0 {
            self.hashes.insert(self.pass, self.game.state_hash());
            while self.hashes.len() > self.config.history {
                let first = *self.hashes.keys().next().unwrap();
                self.hashes.remove(&first);
            }
        }
    }

    fn drop_slot(&mut self, slot: usize, lost: bool) {
        if self.slots[slot].take().is_some() {
            if lost {
                self.notices.push([OP_PEER_LOST, slot as u32, 0, 0, 0]);
                self.events.push(Event::PeerLost { slot, pass: self.pass });
            } else {
                self.notices.push([OP_PEER_LEFT, slot as u32, 0, 0, 0]);
                self.events.push(Event::Left { slot, pass: self.pass });
            }
        }
    }

    /// Una iteración: mensajes, tiempos de espera y, si toca, una pasada.
    pub fn step(&mut self, now: u64, port: &mut dyn Port) {
        for (from, message) in port.receive() {
            match message {
                Message::Join { version, .. } => self.handle_join(now, from, version, port),
                Message::Commands { acked, records } => {
                    if let Some(s) = self.slot_of(from) {
                        let p = self.slots[s].as_mut().unwrap();
                        p.acked = p.acked.max(acked);
                        p.last_seen = now;
                        p.pending.extend(records);
                    }
                }
                Message::Hash { pass, hash } => {
                    if let Some(s) = self.slot_of(from) {
                        let p = self.slots[s].as_mut().unwrap();
                        p.acked = p.acked.max(pass);
                        p.last_seen = now;
                        let stale = self.resync_floor.is_some_and(|f| pass <= f);
                        if !stale && self.hashes.get(&pass).is_some_and(|h| *h != hash) {
                            self.events.push(Event::Desync { slot: s, pass });
                        }
                    }
                }
                Message::Leave => {
                    if let Some(s) = self.slot_of(from) {
                        self.drop_slot(s, false);
                    }
                }
                _ => {}
            }
        }
        for s in 1..MAX_SLOTS {
            if self.slots[s].as_ref().is_some_and(|p| now.saturating_sub(p.last_seen) > self.config.timeout_ms) {
                self.drop_slot(s, true);
            }
        }
        if now < self.next_pass_at {
            return;
        }
        let behind = self.slots.iter().flatten().any(|p| p.acked + self.config.window < self.pass);
        if behind {
            self.stalls += 1;
            return;
        }
        self.next_pass_at = now + self.config.pass_ms;
        let mut records = Vec::new();
        for s in 1..MAX_SLOTS {
            if let Some(p) = self.slots[s].as_mut() {
                records.append(&mut p.pending);
            }
        }
        records.append(&mut self.own);
        records.append(&mut self.notices);
        let tick = self.game.tick();
        records.push([OP_ADVANCE_TIME, tick, tick.wrapping_add(self.config.ticks_per_pass), 0, 0]);
        let index = self.pass + 1;
        let message = Message::Pass { index, records: records.clone() };
        let mut lost = Vec::new();
        for s in 1..MAX_SLOTS {
            if let Some(p) = &self.slots[s] {
                if port.send(p.id, &message).is_err() {
                    lost.push(s);
                }
            }
        }
        for record in &records {
            if let Err(error) = self.game.apply(record) {
                self.events.push(Event::GameError { pass: index, record: *record, error });
            }
        }
        self.records_sent += records.len() as u64;
        self.pass = index;
        self.remember_hash();
        for s in lost {
            self.drop_slot(s, true);
        }
    }

    fn handle_join(&mut self, now: u64, from: PeerId, version: u32, port: &mut dyn Port) {
        if version != LOCKSTEP_VERSION {
            let _ = port.send(from, &Message::Reject { reason: REJECT_VERSION });
            return;
        }
        if let Some(s) = self.slot_of(from) {
            // Vuelve a unirse la misma máquina: se trata como baja y alta.
            self.drop_slot(s, false);
        }
        let Some(slot) = (1..MAX_SLOTS).find(|&s| self.slots[s].is_none()) else {
            let _ = port.send(from, &Message::Reject { reason: REJECT_FULL });
            return;
        };
        let snapshot = match self.game.snapshot() {
            Ok(s) => s,
            Err(error) => {
                self.events.push(Event::GameError { pass: self.pass, record: [OP_PEER_JOINED, slot as u32, 0, 0, 0], error });
                let _ = port.send(from, &Message::Reject { reason: REJECT_BUSY });
                return;
            }
        };
        if self.config.resync_on_join {
            if let Err(error) = self.game.restore(&snapshot) {
                self.events.push(Event::GameError { pass: self.pass, record: [OP_PEER_JOINED, slot as u32, 0, 0, 0], error });
                return;
            }
            let hash = self.game.state_hash();
            self.hashes.insert(self.pass, hash);
            self.resync_floor = Some(self.pass);
            let resync = Message::Resync { pass: self.pass, hash, snapshot: snapshot.clone() };
            for s in 1..MAX_SLOTS {
                if let Some(p) = &self.slots[s] {
                    let _ = port.send(p.id, &resync);
                }
            }
            self.events.push(Event::Resynced { pass: self.pass });
        }
        let hash = self.game.state_hash();
        self.hashes.insert(self.pass, hash);
        if port.send(from, &Message::Welcome { slot: slot as u32, pass: self.pass, hash, snapshot }).is_ok() {
            self.slots[slot] = Some(Peer { id: from, acked: self.pass, last_seen: now, pending: Vec::new() });
            self.notices.push([OP_PEER_JOINED, slot as u32, 0, 0, 0]);
            self.events.push(Event::Joined { slot, pass: self.pass });
        }
    }
}

/// Cliente nuevo.
pub struct Client<G: Game> {
    pub game: G,
    pub config: Config,
    pub host: PeerId,
    pub slot: Option<usize>,
    /// Última pasada ejecutada.
    pub pass: u64,
    outbox: Vec<Record>,
    last_host: u64,
    last_sent: u64,
    joined: bool,
    stopped: bool,
    /// Reintento de unión pendiente (`REJECT_BUSY`) y nombre usado.
    retry_at: Option<u64>,
    name: Vec<u8>,
    pub events: Vec<Event>,
    /// Hashes propios por pasada (para informes).
    pub hashes: BTreeMap<u64, [u8; 32]>,
}

impl<G: Game> Client<G> {
    pub fn new(game: G, config: Config, host: PeerId) -> Client<G> {
        Client { game, config, host, slot: None, pass: 0, outbox: Vec::new(), last_host: 0, last_sent: 0, joined: false, stopped: false, retry_at: None, name: Vec::new(), events: Vec::new(), hashes: BTreeMap::new() }
    }

    pub fn join(&mut self, now: u64, port: &mut dyn Port, name: &[u8]) {
        self.last_host = now;
        self.stopped = false;
        self.joined = false;
        self.retry_at = None;
        self.name = name.to_vec();
        let _ = port.send(self.host, &Message::Join { version: LOCKSTEP_VERSION, name: name.to_vec() });
    }

    pub fn leave(&mut self, port: &mut dyn Port) {
        let _ = port.send(self.host, &Message::Leave);
        self.stopped = true;
        self.joined = false;
    }

    pub fn command(&mut self, record: Record) {
        self.outbox.push(record);
    }

    pub fn is_running(&self) -> bool {
        self.joined && !self.stopped
    }

    fn adopt(&mut self, pass: u64, hash: [u8; 32], snapshot: &[u8]) -> bool {
        match self.game.restore(snapshot) {
            Ok(()) if self.game.state_hash() == hash => {
                self.pass = pass;
                self.hashes.insert(pass, hash);
                true
            }
            Ok(()) => {
                self.events.push(Event::SnapshotMismatch { pass });
                false
            }
            Err(error) => {
                self.events.push(Event::GameError { pass, record: [OP_PEER_JOINED, 0, 0, 0, 0], error });
                false
            }
        }
    }

    pub fn step(&mut self, now: u64, port: &mut dyn Port) {
        if self.stopped {
            return;
        }
        let mut progressed = false;
        for (from, message) in port.receive() {
            if from != self.host {
                continue;
            }
            self.last_host = now;
            match message {
                Message::Welcome { slot, pass, hash, snapshot } => {
                    if self.adopt(pass, hash, &snapshot) {
                        self.slot = Some(slot as usize);
                        self.joined = true;
                        self.events.push(Event::Joined { slot: slot as usize, pass });
                    } else {
                        self.stopped = true;
                    }
                }
                Message::Resync { pass, hash, snapshot } => {
                    if self.adopt(pass, hash, &snapshot) {
                        self.events.push(Event::Resynced { pass });
                    } else {
                        self.stopped = true;
                    }
                }
                Message::Reject { reason } => {
                    self.events.push(Event::Rejected { reason });
                    if reason == REJECT_BUSY && !self.joined {
                        self.retry_at = Some(now + RETRY_MS);
                    } else {
                        self.stopped = true;
                    }
                }
                Message::Pass { index, records } if self.joined => {
                    if index != self.pass + 1 {
                        // TCP y la red simulada entregan en orden: un salto es un fallo.
                        self.events.push(Event::Desync { slot: self.slot.unwrap_or(0), pass: index });
                        self.stopped = true;
                        return;
                    }
                    for record in &records {
                        if let Err(error) = self.game.apply(record) {
                            self.events.push(Event::GameError { pass: index, record: *record, error });
                        }
                    }
                    self.pass = index;
                    progressed = true;
                    if index % self.config.hash_every == 0 {
                        let hash = self.game.state_hash();
                        self.hashes.insert(index, hash);
                        let _ = port.send(self.host, &Message::Hash { pass: index, hash });
                    }
                }
                _ => {}
            }
        }
        if !self.joined {
            if let Some(at) = self.retry_at.filter(|&at| now >= at) {
                let _ = at;
                let name = std::mem::take(&mut self.name);
                self.join(now, port, &name);
                return;
            }
            if now.saturating_sub(self.last_host) > self.config.timeout_ms {
                self.events.push(Event::HostLost { pass: self.pass });
                self.stopped = true;
            }
            return;
        }
        if !self.outbox.is_empty() || progressed || now.saturating_sub(self.last_sent) >= self.config.timeout_ms / 4 {
            self.last_sent = now;
            let records = std::mem::take(&mut self.outbox);
            if port.send(self.host, &Message::Commands { acked: self.pass, records }).is_err() {
                self.events.push(Event::HostLost { pass: self.pass });
                self.stopped = true;
                return;
            }
        }
        if now.saturating_sub(self.last_host) > self.config.timeout_ms {
            self.events.push(Event::HostLost { pass: self.pass });
            self.stopped = true;
        }
    }
}

/// Comprobación cruzada de hashes entre máquinas (WEB-7): `(comparaciones, distintos)`.
/// Cada serie es la de una máquina (pasada → hash completo). Por cada pasada presente en
/// al menos dos series se compara cada máquina con la primera que la tiene, de modo que
/// también se cotejan clientes entre sí y no solo contra el anfitrión.
pub fn cross_check<'a>(series: impl IntoIterator<Item = &'a BTreeMap<u64, [u8; 32]>>) -> (usize, usize) {
    let mut first: BTreeMap<u64, [u8; 32]> = BTreeMap::new();
    let (mut compared, mut different) = (0, 0);
    for s in series {
        for (&pass, hash) in s {
            match first.get(&pass) {
                None => {
                    first.insert(pass, *hash);
                }
                Some(h) => {
                    compared += 1;
                    different += usize::from(h != hash);
                }
            }
        }
    }
    (compared, different)
}
