//! Servidor HTTP + WebSocket mínimo (sin runtime asíncrono). Todo exige la contraseña:
//! - la página (`GET /`) con autenticación Basic (usuario cualquiera, la contraseña de `P3_PASSWORD`);
//! - el WebSocket (`/ws?t=TOKEN`) con un token aleatorio de esta ejecución que solo da la página ya autenticada.
//! Detrás del túnel Cloudflare se ve como un sitio https normal. No se sirve ningún archivo del juego: solo la página y los flujos.
use crate::hub::{Event, Hub, Packet};
use crate::Opts;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::Ordering;
use std::sync::mpsc::RecvTimeoutError;
use std::sync::Arc;
use std::time::Duration;
use tungstenite::handshake::server::{ErrorResponse, Request, Response};
use tungstenite::http::StatusCode;
use tungstenite::{Error as WsError, Message};

const PAGE: &str = include_str!("../www/index.html");

struct Auth {
    password: String,
    token: String,
}

fn ct_eq(a: &[u8], b: &[u8]) -> bool {
    a.len() == b.len() && a.iter().zip(b).fold(0u8, |d, (x, y)| d | (x ^ y)) == 0
}

fn b64_decode(s: &str) -> Vec<u8> {
    let val = |c: u8| match c {
        b'A'..=b'Z' => Some(c - b'A'),
        b'a'..=b'z' => Some(c - b'a' + 26),
        b'0'..=b'9' => Some(c - b'0' + 52),
        b'+' => Some(62),
        b'/' => Some(63),
        _ => None,
    };
    let (mut out, mut acc, mut bits) = (Vec::new(), 0u32, 0);
    for c in s.bytes().filter_map(val) {
        acc = (acc << 6) | u32::from(c);
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((acc >> bits) as u8);
        }
    }
    out
}

pub fn start(o: &Opts, hub: Arc<Hub>) -> Result<(), String> {
    let listener = TcpListener::bind((o.bind.as_str(), o.port)).map_err(|e| format!("{}:{}: {e}", o.bind, o.port))?;
    let mut tok = [0u8; 18];
    getrandom::fill(&mut tok).map_err(|e| format!("aleatorio: {e}"))?;
    let token: String = tok.iter().map(|b| format!("{b:02x}")).collect();
    let auth = Arc::new(Auth { password: o.password.clone(), token });
    eprintln!("servidor en http://{}:{}/ (contraseña obligatoria)", o.bind, o.port);
    let hello = format!("{{\"rate\":{}}}", o.audio_rate);
    std::thread::Builder::new()
        .name("aceptar".into())
        .spawn(move || {
            for s in listener.incoming().flatten() {
                let (auth, hub, hello) = (auth.clone(), hub.clone(), hello.clone());
                std::thread::spawn(move || {
                    if let Err(e) = connection(s, &auth, hub, &hello) {
                        if !e.contains("Connection reset") && !e.contains("Broken pipe") {
                            eprintln!("conexión: {e}");
                        }
                    }
                });
            }
        })
        .unwrap();
    Ok(())
}

fn header<'a>(head: &'a str, name: &str) -> Option<&'a str> {
    head.lines().skip(1).find_map(|l| {
        let (k, v) = l.split_once(':')?;
        k.trim().eq_ignore_ascii_case(name).then(|| v.trim())
    })
}

fn password_ok(head: &str, auth: &Auth) -> bool {
    let Some(h) = header(head, "authorization") else { return false };
    let Some(b) = h.strip_prefix("Basic ").or_else(|| h.strip_prefix("basic ")) else { return false };
    let dec = b64_decode(b);
    let pass = dec.splitn(2, |&c| c == b':').nth(1).unwrap_or(&[]);
    ct_eq(pass, auth.password.as_bytes())
}

fn respond(s: &mut TcpStream, status: &str, extra: &str, ctype: &str, body: &[u8]) {
    let head = format!(
        "HTTP/1.1 {status}\r\nContent-Type: {ctype}\r\nContent-Length: {}\r\nCache-Control: no-store\r\nX-Content-Type-Options: nosniff\r\nReferrer-Policy: no-referrer\r\nConnection: close\r\n{extra}\r\n",
        body.len()
    );
    let _ = s.write_all(head.as_bytes());
    let _ = s.write_all(body);
}

fn connection(mut s: TcpStream, auth: &Auth, hub: Arc<Hub>, hello: &str) -> Result<(), String> {
    s.set_read_timeout(Some(Duration::from_secs(10))).ok();
    s.set_nodelay(true).ok();
    // Se mira la cabecera sin consumirla (peek): si es un WebSocket la lee tungstenite entera.
    let mut buf = [0u8; 4096];
    let mut n = 0;
    for _ in 0..50 {
        n = s.peek(&mut buf).map_err(|e| e.to_string())?;
        if buf[..n].windows(4).any(|w| w == b"\r\n\r\n") || n == buf.len() {
            break;
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    let head = String::from_utf8_lossy(&buf[..n]).into_owned();
    let path = head.lines().next().and_then(|l| l.split(' ').nth(1)).unwrap_or("/").to_string();
    let is_ws = header(&head, "upgrade").is_some_and(|v| v.eq_ignore_ascii_case("websocket"));
    if is_ws {
        let token_ok = path.split_once("?t=").is_some_and(|(p, t)| p == "/ws" && ct_eq(t.split('&').next().unwrap_or("").as_bytes(), auth.token.as_bytes()));
        if !token_ok {
            std::thread::sleep(Duration::from_millis(500));
        }
        let cb = |_: &Request, resp: Response| -> Result<Response, ErrorResponse> {
            if token_ok {
                Ok(resp)
            } else {
                let mut e = ErrorResponse::new(Some("no autorizado".into()));
                *e.status_mut() = StatusCode::UNAUTHORIZED;
                Err(e)
            }
        };
        let ws = tungstenite::accept_hdr(s, cb).map_err(|e| e.to_string())?;
        return websocket(ws, hub, hello);
    }
    // HTTP normal: se consume la cabecera.
    let mut consumed = 0;
    while consumed < n {
        let k = s.read(&mut buf[..n - consumed]).map_err(|e| e.to_string())?;
        if k == 0 {
            break;
        }
        consumed += k;
    }
    if !password_ok(&head, auth) {
        std::thread::sleep(Duration::from_millis(500));
        respond(&mut s, "401 Unauthorized", "WWW-Authenticate: Basic realm=\"Patrician III\", charset=\"UTF-8\"\r\n", "text/plain", b"contrasena requerida");
        return Ok(());
    }
    let route = path.split('?').next().unwrap_or("/");
    match route {
        "/" | "/index.html" => {
            let body = PAGE.replace("__TOKEN__", &auth.token);
            respond(&mut s, "200 OK", "", "text/html; charset=utf-8", body.as_bytes());
        }
        "/favicon.ico" => respond(&mut s, "204 No Content", "", "image/x-icon", b""),
        _ => respond(&mut s, "404 Not Found", "", "text/plain", b"no encontrado"),
    }
    Ok(())
}

fn websocket(mut ws: tungstenite::WebSocket<TcpStream>, hub: Arc<Hub>, hello: &str) -> Result<(), String> {
    let stream = ws.get_ref();
    stream.set_read_timeout(None).ok();
    stream.set_nonblocking(true).ok();
    let (id, rx, need_key) = hub.add_client();
    eprintln!("espectador {id} conectado ({} en total)", hub.viewers.load(Ordering::Relaxed));
    let result = (|| -> Result<(), String> {
        ws.send(Message::text(hello)).or_else(ignore_block).map_err(|e| e.to_string())?;
        loop {
            // Saliente: paquetes del reparto (vídeo/sonido).
            match rx.recv_timeout(Duration::from_millis(3)) {
                Ok(p) => {
                    send_packet(&mut ws, &p)?;
                    while let Ok(p) = rx.try_recv() {
                        send_packet(&mut ws, &p)?;
                    }
                }
                Err(RecvTimeoutError::Timeout) => {}
                Err(RecvTimeoutError::Disconnected) => return Ok(()),
            }
            flush(&mut ws)?;
            // Entrante: ratón, teclado, petición de fotograma clave, ping.
            loop {
                match ws.read() {
                    Ok(Message::Text(t)) => {
                        let t = t.as_str();
                        if let Some(rest) = t.strip_prefix("p ") {
                            ws.send(Message::text(format!("p {rest}"))).or_else(ignore_block).map_err(|e| e.to_string())?;
                        } else if t == "r" {
                            need_key.store(true, Ordering::Relaxed);
                            hub.want_key.store(true, Ordering::Relaxed);
                        } else if let Some(ev) = Event::parse(t) {
                            let mut q = hub.input.lock().unwrap();
                            if q.len() < 4096 {
                                q.push_back(ev);
                            }
                        }
                    }
                    Ok(Message::Close(_)) => return Ok(()),
                    Ok(_) => {}
                    Err(WsError::Io(e)) if e.kind() == std::io::ErrorKind::WouldBlock => break,
                    Err(WsError::ConnectionClosed | WsError::AlreadyClosed) => return Ok(()),
                    Err(e) => return Err(e.to_string()),
                }
            }
        }
    })();
    hub.remove_client(id);
    eprintln!("espectador {id} desconectado");
    result
}

fn ignore_block(e: WsError) -> Result<(), WsError> {
    match e {
        WsError::Io(ref io) if io.kind() == std::io::ErrorKind::WouldBlock => Ok(()),
        e => Err(e),
    }
}

fn send_packet(ws: &mut tungstenite::WebSocket<TcpStream>, p: &Packet) -> Result<(), String> {
    ws.write(Message::binary(p.as_ref().clone())).or_else(ignore_block).map_err(|e| e.to_string())
}

fn flush(ws: &mut tungstenite::WebSocket<TcpStream>) -> Result<(), String> {
    ws.flush().or_else(ignore_block).map_err(|e| e.to_string())
}
