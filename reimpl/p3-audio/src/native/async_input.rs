//! Entrada comprimida asíncrona y acotada: productor de E/S y consumidor no bloqueante.
//! `try_read` nunca espera al productor; EOF, underrun, contención y fallo son distintos.
use std::collections::VecDeque;
use std::io::{ErrorKind, Read, Seek, SeekFrom};
use std::sync::{Arc, Condvar, Mutex, TryLockError};
use std::thread::{self, JoinHandle};

pub trait ReadSeek: Read + Seek + Send {}
impl<T: Read + Seek + Send> ReadSeek for T {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputState {
    Ready,
    Starved,
    Contended,
    End,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InputRead {
    pub bytes: usize,
    pub state: InputState,
    pub epoch: u64,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputError {
    Configuration,
    InvalidPosition,
    Closed,
    Poisoned,
    Io(ErrorKind),
    Spawn,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InputSnapshot {
    pub epoch: u64,
    pub queued_chunks: usize,
    pub queued_bytes: usize,
    pub producer_position: u64,
    pub eof: bool,
    pub paused: bool,
    pub error: Option<ErrorKind>,
}
struct Chunk {
    data: Vec<u8>,
    at: usize,
}
struct State {
    queue: VecDeque<Chunk>,
    epoch: u64,
    seek: Option<(u64, u64)>,
    producer_position: u64,
    eof: bool,
    error: Option<ErrorKind>,
    paused: bool,
    closed: bool,
}
struct Shared {
    state: Mutex<State>,
    wake: Condvar,
}

/// La fuente puede ser un archivo o región CPR, siempre con longitud conocida.
/// El hilo hace como máximo `queue_chunks` bloques de `chunk_bytes` pendientes.
pub struct AsyncInput {
    shared: Arc<Shared>,
    worker: Option<JoinHandle<()>>,
    length: u64,
}
impl AsyncInput {
    pub fn new(
        reader: Box<dyn ReadSeek>,
        length: u64,
        chunk_bytes: usize,
        queue_chunks: usize,
    ) -> Result<Self, InputError> {
        if length == 0
            || chunk_bytes == 0
            || queue_chunks == 0
            || chunk_bytes > 16 * 1024 * 1024
            || queue_chunks > 1024
            || chunk_bytes
                .checked_mul(queue_chunks)
                .is_none_or(|v| v > 64 * 1024 * 1024)
        {
            return Err(InputError::Configuration);
        }
        let shared = Arc::new(Shared {
            state: Mutex::new(State {
                queue: VecDeque::new(),
                epoch: 0,
                seek: Some((0, 0)),
                producer_position: 0,
                eof: false,
                error: None,
                paused: false,
                closed: false,
            }),
            wake: Condvar::new(),
        });
        let thread_shared = shared.clone();
        let worker = thread::Builder::new()
            .name("p3-audio-input".into())
            .spawn(move || producer(reader, length, chunk_bytes, queue_chunks, thread_shared))
            .map_err(|_| InputError::Spawn)?;
        Ok(Self {
            shared,
            worker: Some(worker),
            length,
        })
    }
    /// Devuelve inmediatamente lo disponible. El tramo no escrito no se modifica.
    /// `Starved` no es EOF; `Contended` indica que el cerrojo estaba ocupado.
    pub fn try_read(&self, out: &mut [u8]) -> Result<InputRead, InputError> {
        let mut s = match self.shared.state.try_lock() {
            Ok(s) => s,
            Err(TryLockError::WouldBlock) => {
                return Ok(InputRead {
                    bytes: 0,
                    state: InputState::Contended,
                    epoch: 0,
                })
            }
            Err(TryLockError::Poisoned(_)) => return Err(InputError::Poisoned),
        };
        if s.closed {
            return Err(InputError::Closed);
        }
        if let Some(e) = s.error.filter(|_| s.queue.is_empty()) {
            return Err(InputError::Io(e));
        }
        let epoch = s.epoch;
        let mut n = 0;
        while n < out.len() {
            let Some(front) = s.queue.front_mut() else { break };
            let count = (out.len() - n).min(front.data.len() - front.at);
            out[n..n + count].copy_from_slice(&front.data[front.at..front.at + count]);
            n += count;
            front.at += count;
            if front.at == front.data.len() {
                s.queue.pop_front();
            }
        }
        let state = if n == out.len() {
            InputState::Ready
        } else if s.eof && s.queue.is_empty() {
            InputState::End
        } else {
            InputState::Starved
        };
        if n > 0 {
            self.shared.wake.notify_one();
        }
        Ok(InputRead { bytes: n, state, epoch })
    }
    /// Invalida los bloques anteriores inmediatamente; los que terminen de leer
    /// después se descartan por generación antes de publicar.
    pub fn seek(&self, position: u64) -> Result<u64, InputError> {
        if position > self.length {
            return Err(InputError::InvalidPosition);
        }
        let mut s = self.shared.state.lock().map_err(|_| InputError::Poisoned)?;
        let epoch = Self::set_seek(&mut s, position)?;
        self.shared.wake.notify_one();
        Ok(epoch)
    }
    /// Igual que seek, pero no espera al cerrojo: None indica contención.
    pub fn try_seek(&self, position: u64) -> Result<Option<u64>, InputError> {
        if position > self.length {
            return Err(InputError::InvalidPosition);
        }
        let mut s = match self.shared.state.try_lock() {
            Ok(s) => s,
            Err(TryLockError::WouldBlock) => return Ok(None),
            Err(TryLockError::Poisoned(_)) => return Err(InputError::Poisoned),
        };
        let epoch = Self::set_seek(&mut s, position)?;
        self.shared.wake.notify_one();
        Ok(Some(epoch))
    }
    fn set_seek(s: &mut State, position: u64) -> Result<u64, InputError> {
        if s.closed {
            return Err(InputError::Closed);
        }
        s.epoch = s.epoch.checked_add(1).ok_or(InputError::Configuration)?;
        let epoch = s.epoch;
        s.queue.clear();
        s.seek = Some((epoch, position));
        s.eof = false;
        s.error = None;
        Ok(epoch)
    }
    pub fn pause_producer(&self, paused: bool) -> Result<(), InputError> {
        let mut s = self.shared.state.lock().map_err(|_| InputError::Poisoned)?;
        if s.closed {
            return Err(InputError::Closed);
        }
        s.paused = paused;
        self.shared.wake.notify_one();
        Ok(())
    }
    pub fn snapshot(&self) -> Result<InputSnapshot, InputError> {
        let s = self.shared.state.lock().map_err(|_| InputError::Poisoned)?;
        Ok(InputSnapshot {
            epoch: s.epoch,
            queued_chunks: s.queue.len(),
            queued_bytes: s.queue.iter().map(|c| c.data.len() - c.at).sum(),
            producer_position: s.producer_position,
            eof: s.eof,
            paused: s.paused,
            error: s.error,
        })
    }
    /// Señala el cierre sin esperar una E/S bloqueada. El hilo se recoge si ya salió.
    pub fn close(&mut self) {
        if let Ok(mut s) = self.shared.state.lock() {
            s.closed = true;
            s.queue.clear();
            self.shared.wake.notify_all();
        }
        if let Some(h) = self.worker.take() {
            if h.is_finished() {
                let _ = h.join();
            }
        }
    }
}
impl Drop for AsyncInput {
    fn drop(&mut self) {
        self.close();
    }
}
fn producer(mut reader: Box<dyn ReadSeek>, length: u64, chunk_bytes: usize, queue_chunks: usize, shared: Arc<Shared>) {
    loop {
        let (epoch, pos, seek) = {
            let Ok(mut s) = shared.state.lock() else { return };
            loop {
                if s.closed {
                    return;
                }
                if let Some((epoch, pos)) = s.seek.take() {
                    break (epoch, pos, true);
                }
                if !s.paused && !s.eof && s.error.is_none() && s.queue.len() < queue_chunks {
                    break (s.epoch, s.producer_position, false);
                }
                s = match shared.wake.wait(s) {
                    Ok(s) => s,
                    Err(_) => return,
                };
            }
        };
        if seek {
            let result = reader.seek(SeekFrom::Start(pos));
            let Ok(mut s) = shared.state.lock() else { return };
            if s.closed {
                return;
            }
            if s.epoch != epoch {
                continue;
            }
            match result {
                Ok(actual) if actual == pos => {
                    s.producer_position = pos;
                    s.eof = pos == length;
                }
                Ok(_) => s.error = Some(ErrorKind::InvalidData),
                Err(e) => s.error = Some(e.kind()),
            }
            shared.wake.notify_all();
            continue;
        }
        let size = (length - pos).min(chunk_bytes as u64) as usize;
        let mut data = vec![0; size];
        let result = reader.read(&mut data);
        let Ok(mut s) = shared.state.lock() else { return };
        if s.closed {
            return;
        }
        if s.epoch != epoch {
            continue;
        }
        match result {
            Ok(0) => s.error = Some(ErrorKind::UnexpectedEof),
            Ok(n) if n <= size => {
                data.truncate(n);
                s.producer_position += n as u64;
                s.queue.push_back(Chunk { data, at: 0 });
                if s.producer_position == length {
                    s.eof = true;
                }
            }
            Ok(_) => s.error = Some(ErrorKind::InvalidData),
            Err(e) if e.kind() == ErrorKind::Interrupted => {}
            Err(e) => s.error = Some(e.kind()),
        }
        shared.wake.notify_all();
    }
}
