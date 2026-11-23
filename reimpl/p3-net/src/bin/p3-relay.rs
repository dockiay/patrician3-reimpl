//! Repetidor WebSocket del multijugador web (WEB-7). Reenvía mensajes entre las máquinas de
//! una sala; no ejecuta el juego ni lee las partidas.
//! Uso: `p3-relay [DIRECCIÓN:PUERTO]` (por defecto `127.0.0.1:8787`).
fn main() {
    let addr = std::env::args().nth(1).unwrap_or_else(|| "127.0.0.1:8787".into());
    let listener = std::net::TcpListener::bind(&addr).unwrap_or_else(|e| {
        eprintln!("no se puede escuchar en {addr}: {e}");
        std::process::exit(1);
    });
    let server = p3_net::ws::Server::spawn(listener).unwrap();
    println!("repetidor p3 escuchando en ws://{}/", server.addr);
    loop {
        std::thread::sleep(std::time::Duration::from_secs(3600));
    }
}
