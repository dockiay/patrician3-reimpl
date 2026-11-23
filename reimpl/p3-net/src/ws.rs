//! WEB-7: WebSocket nativo para el repetidor (característica `ws`). Cliente ([`WsLink`], un
//! [`Duplex`]) para pruebas y para clientes de escritorio, y servidor ([`Server`]) que
//! ejecuta un [`Relay`] con un hilo por conexión. En el navegador el cliente es el
//! `WebSocket` de JavaScript (lo aporta `p3-web` implementando [`Duplex`]).
//! No toca [`crate::transport::TcpLink`].

use crate::relay::{ConnId, Duplex, Out, Relay};
use crate::transport::LinkError;
use std::collections::BTreeMap;
use std::io::ErrorKind;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tungstenite::{Message as Ws, WebSocket};

fn io_error(e: tungstenite::Error) -> LinkError {
    match e {
        tungstenite::Error::ConnectionClosed | tungstenite::Error::AlreadyClosed => LinkError::Disconnected,
        tungstenite::Error::Io(e) if matches!(e.kind(), ErrorKind::BrokenPipe | ErrorKind::ConnectionReset | ErrorKind::ConnectionAborted) => LinkError::Disconnected,
        e => LinkError::Io(e.to_string()),
    }
}

fn would_block(e: &tungstenite::Error) -> bool {
    matches!(e, tungstenite::Error::Io(e) if matches!(e.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut))
}

/// Cliente WebSocket: envío bloqueante, recepción sin bloquear.
pub struct WsLink {
    socket: WebSocket<TcpStream>,
}

impl WsLink {
    pub fn connect(addr: SocketAddr) -> Result<WsLink, LinkError> {
        let stream = TcpStream::connect(addr).map_err(|e| LinkError::Io(e.to_string()))?;
        stream.set_nodelay(true).map_err(|e| LinkError::Io(e.to_string()))?;
        let (socket, _) = tungstenite::client(format!("ws://{addr}/"), stream).map_err(|e| LinkError::Io(e.to_string()))?;
        socket.get_ref().set_nonblocking(true).map_err(|e| LinkError::Io(e.to_string()))?;
        Ok(WsLink { socket })
    }
    /// Cierra el socket sin despedida (como una caída de red).
    pub fn drop_connection(self) {
        let _ = self.socket.get_ref().shutdown(std::net::Shutdown::Both);
    }
}

impl Duplex for WsLink {
    fn send(&mut self, message: &[u8]) -> Result<(), LinkError> {
        let blocking = |s: &WsLink, b: bool| s.socket.get_ref().set_nonblocking(!b).map_err(|e| LinkError::Io(e.to_string()));
        blocking(self, true)?;
        let r = self.socket.send(Ws::Binary(message.to_vec().into())).and_then(|_| self.socket.flush()).map_err(io_error);
        blocking(self, false)?;
        r
    }
    fn receive(&mut self) -> Result<Vec<Vec<u8>>, LinkError> {
        let mut out = Vec::new();
        loop {
            match self.socket.read() {
                Ok(Ws::Binary(b)) => out.push(b.to_vec()),
                Ok(Ws::Close(_)) => return if out.is_empty() { Err(LinkError::Disconnected) } else { Ok(out) },
                Ok(_) => {}
                Err(e) if would_block(&e) => return Ok(out),
                Err(e) => return if out.is_empty() { Err(io_error(e)) } else { Ok(out) },
            }
        }
    }
}

enum ToConn {
    Data(Vec<u8>),
    Close,
}

#[derive(Default)]
struct Shared {
    relay: Relay,
    senders: BTreeMap<ConnId, Sender<ToConn>>,
    next: ConnId,
}

impl Shared {
    fn dispatch(&self, outs: Vec<Out>) {
        for o in outs {
            let (id, m) = match o {
                Out::Send(id, d) => (id, ToConn::Data(d)),
                Out::Close(id) => (id, ToConn::Close),
            };
            if let Some(s) = self.senders.get(&id) {
                let _ = s.send(m);
            }
        }
    }
}

/// Servidor del repetidor en un hilo de aceptación; cada conexión tiene su hilo.
pub struct Server {
    pub addr: SocketAddr,
    shared: Arc<Mutex<Shared>>,
}

impl Server {
    pub fn spawn(listener: TcpListener) -> std::io::Result<Server> {
        let addr = listener.local_addr()?;
        let shared = Arc::new(Mutex::new(Shared::default()));
        let accept = shared.clone();
        std::thread::spawn(move || {
            for stream in listener.incoming().flatten() {
                let shared = accept.clone();
                std::thread::spawn(move || serve_connection(stream, shared));
            }
        });
        Ok(Server { addr, shared })
    }
    /// `(salas, conexiones, mensajes reenviados, bytes reenviados)`.
    pub fn stats(&self) -> (usize, usize, u64, u64) {
        let s = self.shared.lock().unwrap();
        (s.relay.rooms(), s.relay.connections(), s.relay.forwarded, s.relay.bytes)
    }
    pub fn peers_in(&self, room: &str) -> Vec<usize> {
        self.shared.lock().unwrap().relay.peers_in(room)
    }
}

fn serve_connection(stream: TcpStream, shared: Arc<Mutex<Shared>>) {
    let _ = stream.set_nodelay(true);
    let Ok(mut socket) = tungstenite::accept(stream) else { return };
    let _ = socket.get_ref().set_read_timeout(Some(Duration::from_millis(2)));
    let (tx, rx): (Sender<ToConn>, Receiver<ToConn>) = channel();
    let id = {
        let mut s = shared.lock().unwrap();
        let id = s.next;
        s.next += 1;
        s.senders.insert(id, tx);
        id
    };
    'conn: loop {
        while let Ok(m) = rx.try_recv() {
            match m {
                ToConn::Data(d) => {
                    if socket.send(Ws::Binary(d.into())).and_then(|_| socket.flush()).is_err() {
                        break 'conn;
                    }
                }
                ToConn::Close => {
                    let _ = socket.close(None);
                    let _ = socket.flush();
                    break 'conn;
                }
            }
        }
        match socket.read() {
            Ok(Ws::Binary(b)) => {
                let mut s = shared.lock().unwrap();
                let outs = s.relay.handle(id, &b);
                s.dispatch(outs);
            }
            Ok(Ws::Close(_)) | Err(tungstenite::Error::ConnectionClosed) | Err(tungstenite::Error::AlreadyClosed) => break,
            Ok(_) => {}
            Err(e) if would_block(&e) => {}
            Err(_) => break,
        }
    }
    let mut s = shared.lock().unwrap();
    let outs = s.relay.disconnected(id);
    s.dispatch(outs);
    s.senders.remove(&id);
}
