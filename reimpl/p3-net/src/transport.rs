//! Transportes para el lockstep: red simulada en memoria (latencia por enlace, cortes y
//! reloj virtual, reproducible) y TCP con el marco de 8 bytes de `SchnellTransfer.dll`.

use crate::wire::{check_frame, frame, split_frames};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

/// Identificador de máquina: 0 es el anfitrión.
pub type PeerId = usize;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LinkError {
    /// El enlace está cortado (desconexión o máquina caída).
    Disconnected,
    Io(String),
}

/// Red simulada. Los mensajes de un enlace llegan en orden (como TCP) tras su latencia.
#[derive(Clone, Debug, Default)]
pub struct MemoryNet {
    pub now_ms: u64,
    latency: BTreeMap<(PeerId, PeerId), u64>,
    queues: BTreeMap<(PeerId, PeerId), VecDeque<(u64, Vec<u8>)>>,
    down: BTreeSet<PeerId>,
    /// Mensajes y bytes entregados (para informes).
    pub delivered: u64,
    pub bytes: u64,
}

impl MemoryNet {
    pub fn new() -> MemoryNet {
        MemoryNet::default()
    }
    /// Latencia en un sentido entre dos máquinas (por defecto 0).
    pub fn set_latency(&mut self, a: PeerId, b: PeerId, ms: u64) {
        self.latency.insert((a, b), ms);
        self.latency.insert((b, a), ms);
    }
    /// Corta una máquina: se pierden sus mensajes en vuelo y los que envíe o le envíen.
    pub fn disconnect(&mut self, peer: PeerId) {
        self.down.insert(peer);
        self.queues.retain(|(from, to), _| *from != peer && *to != peer);
    }
    pub fn reconnect(&mut self, peer: PeerId) {
        self.down.remove(&peer);
    }
    pub fn is_down(&self, peer: PeerId) -> bool {
        self.down.contains(&peer)
    }
    pub fn send(&mut self, from: PeerId, to: PeerId, message: Vec<u8>) -> Result<(), LinkError> {
        if self.down.contains(&from) || self.down.contains(&to) {
            return Err(LinkError::Disconnected);
        }
        let at = self.now_ms + self.latency.get(&(from, to)).copied().unwrap_or(0);
        self.queues.entry((from, to)).or_default().push_back((at, message));
        Ok(())
    }
    /// Mensajes ya llegados a `to`, por orden de remitente y de envío.
    pub fn receive(&mut self, to: PeerId) -> Vec<(PeerId, Vec<u8>)> {
        let mut out = Vec::new();
        if self.down.contains(&to) {
            return out;
        }
        for ((from, dest), queue) in self.queues.iter_mut() {
            if *dest != to {
                continue;
            }
            while queue.front().is_some_and(|(at, _)| *at <= self.now_ms) {
                let (_, m) = queue.pop_front().unwrap();
                self.delivered += 1;
                self.bytes += m.len() as u64;
                out.push((*from, m));
            }
        }
        out
    }
    pub fn advance(&mut self, ms: u64) {
        self.now_ms += ms;
    }
    /// Mensajes en vuelo.
    pub fn in_flight(&self) -> usize {
        self.queues.values().map(VecDeque::len).sum()
    }
}

/// Enlace TCP con el marco de la DLL (firma de 3 bytes, `0xFF`, tamaño `u32`).
pub struct TcpLink {
    stream: std::net::TcpStream,
    signature: [u8; 3],
    buffer: Vec<u8>,
}

impl TcpLink {
    pub fn new(stream: std::net::TcpStream, signature: [u8; 3]) -> std::io::Result<TcpLink> {
        stream.set_nodelay(true)?;
        stream.set_nonblocking(true)?;
        Ok(TcpLink { stream, signature, buffer: Vec::new() })
    }
    /// Envía `payload` con 8 bytes de marco delante.
    pub fn send(&mut self, payload: &[u8]) -> Result<(), LinkError> {
        use std::io::Write;
        let mut message = vec![0u8; 8];
        message.extend_from_slice(payload);
        frame(self.signature, &mut message).map_err(|c| LinkError::Io(format!("marco {c:#x}")))?;
        self.stream.set_nonblocking(false).map_err(|e| LinkError::Io(e.to_string()))?;
        let r = self.stream.write_all(&message).map_err(|e| match e.kind() {
            std::io::ErrorKind::BrokenPipe | std::io::ErrorKind::ConnectionReset | std::io::ErrorKind::ConnectionAborted => LinkError::Disconnected,
            _ => LinkError::Io(e.to_string()),
        });
        self.stream.set_nonblocking(true).map_err(|e| LinkError::Io(e.to_string()))?;
        r
    }
    /// Mensajes completos recibidos (sin el marco).
    pub fn receive(&mut self) -> Result<Vec<Vec<u8>>, LinkError> {
        use std::io::Read;
        let mut chunk = [0u8; 0x4000];
        loop {
            match self.stream.read(&mut chunk) {
                Ok(0) => {
                    if self.buffer.is_empty() {
                        return Err(LinkError::Disconnected);
                    }
                    break;
                }
                Ok(n) => self.buffer.extend_from_slice(&chunk[..n]),
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(e) => return Err(LinkError::Io(e.to_string())),
            }
        }
        let frames = split_frames(&mut self.buffer).map_err(|e| LinkError::Io(e.into()))?;
        let mut out = Vec::with_capacity(frames.len());
        for f in frames {
            if !check_frame(self.signature, &f) {
                return Err(LinkError::Io("firma de marco distinta".into()));
            }
            out.push(f[8..].to_vec());
        }
        Ok(out)
    }
}
