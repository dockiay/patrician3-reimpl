//! Cola de paquetes del objeto de operaciones (`Q_in` `OPS+0x89C`, `Q_out` `OPS+0x8B8`):
//! anillo de trozos de 0x100 registros con tabla de siguientes de un byte.
//!
//! Disposición del original (0x1C bytes): `+0` capacidad de ranuras, `+4` trozo de
//! lectura, `+8` posición de lectura, `+0xC` trozo de escritura, `+0x10` posición de
//! escritura, `+0x14` punteros a trozos, `+0x18` tabla de siguientes.
//! Portado con contraste x86: `operation_queue_init` 0x00543950,
//! `operation_queue_grow` 0x00543D10, `operation_queue_push_packet` 0x00543AE0 y
//! `operation_queue_pop_into_packet` 0x00543C70 (`oracle/x86_net_packets.py`).

use crate::wire::{Packet, Record, EMPTY_RECORD, MAX_RECORDS};

/// Registros por trozo.
pub const CHUNK: usize = 0x100;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OpQueue {
    /// `+0`.
    pub capacity: usize,
    /// `+4`, `+8`.
    pub read_chunk: usize,
    pub read_at: usize,
    /// `+0xC`, `+0x10`.
    pub write_chunk: usize,
    pub write_at: usize,
    /// `+0x14`: trozos reservados (`None` = puntero nulo).
    pub chunks: Vec<Option<Vec<Record>>>,
    /// `+0x18`: siguiente trozo de cada ranura.
    pub next: Vec<u8>,
}

impl Default for OpQueue {
    fn default() -> Self {
        OpQueue::new()
    }
}

impl OpQueue {
    /// `0x00543950`: ocho ranuras, tres trozos en anillo 0→1→2→0.
    pub fn new() -> OpQueue {
        let mut chunks = vec![None; 8];
        let mut next = vec![0u8; 8];
        for (i, slot) in chunks.iter_mut().enumerate().take(3) {
            *slot = Some(vec![EMPTY_RECORD; CHUNK]);
            next[i] = if i < 2 { i as u8 + 1 } else { 0 };
        }
        OpQueue { capacity: 8, read_chunk: 0, read_at: 0, write_chunk: 0, write_at: 0, chunks, next }
    }

    /// `0x00543D10`: dieciséis ranuras más; las nuevas sin trozo y con siguiente 0.
    fn grow(&mut self) {
        self.capacity += 0x10;
        self.chunks.resize(self.capacity, None);
        self.next.resize(self.capacity, 0);
    }

    /// Registros pendientes de leer.
    pub fn len(&self) -> usize {
        let mut total = 0;
        let mut chunk = self.read_chunk;
        let mut at = self.read_at;
        let mut guard = 0;
        while chunk != self.write_chunk {
            total += CHUNK - at;
            chunk = usize::from(self.next[chunk]);
            at = 0;
            guard += 1;
            if guard > self.capacity {
                break;
            }
        }
        total + self.write_at.saturating_sub(at)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// `operation_queue_push_packet` (0x00543AE0): copia los registros válidos del paquete
    /// y deja su recuento a cero. Al llenar un trozo pasa al siguiente del anillo; si es el
    /// de lectura reserva uno nuevo en la primera ranura libre a partir de la 3 (creciendo
    /// la tabla si hace falta) y, si el siguiente es un trozo sobrante (ranura > 2) que no
    /// precede al de lectura, lo libera y salta al que le sigue.
    pub fn push_packet(&mut self, packet: &mut Packet) {
        let total = usize::from(packet.count);
        if total == 0 {
            return;
        }
        let mut done = 0usize;
        loop {
            let room = CHUNK - self.write_at;
            let n = (total - done).min(room);
            let chunk = self.chunks[self.write_chunk].as_mut().expect("trozo de escritura reservado");
            for k in 0..n {
                chunk[self.write_at + k] = packet.records[(done + k).min(MAX_RECORDS - 1)];
            }
            self.write_at += n;
            done += n;
            if self.write_at < CHUNK {
                break;
            }
            let following = usize::from(self.next[self.write_chunk]);
            let target = if following == self.read_chunk {
                let mut slot = 3;
                while self.chunks.get(slot).is_some_and(Option::is_some) {
                    slot += 1;
                    if slot >= self.capacity {
                        self.grow();
                    }
                }
                self.chunks[slot] = Some(vec![EMPTY_RECORD; CHUNK]);
                self.next[slot] = self.next[self.write_chunk];
                self.next[self.write_chunk] = slot as u8;
                slot
            } else if following > 2 {
                let after = usize::from(self.next[following]);
                if after != self.read_chunk {
                    self.next[self.write_chunk] = after as u8;
                    self.chunks[following] = None;
                    after
                } else {
                    following
                }
            } else {
                following
            };
            self.write_chunk = target;
            self.write_at = 0;
        }
        packet.count = 0;
    }

    /// `operation_queue_pop_into_packet` (0x00543C70): llena el paquete hasta 52 registros.
    pub fn pop_into(&mut self, packet: &mut Packet) {
        loop {
            let end = if self.read_chunk == self.write_chunk { self.write_at } else { CHUNK };
            let available = end - self.read_at;
            let room = MAX_RECORDS - usize::from(packet.count).min(MAX_RECORDS);
            let n = available.min(room);
            let chunk = self.chunks[self.read_chunk].as_ref().expect("trozo de lectura reservado");
            for k in 0..n {
                packet.records[usize::from(packet.count) + k] = chunk[self.read_at + k];
            }
            packet.count += n as u16;
            self.read_at += n;
            if self.read_at < CHUNK {
                return;
            }
            self.read_at = 0;
            self.read_chunk = usize::from(self.next[self.read_chunk]);
        }
    }

    /// Ranuras con trozo reservado.
    pub fn allocated(&self) -> usize {
        self.chunks.iter().filter(|c| c.is_some()).count()
    }
}
