//! M02: partida para varios jugadores entre clientes nuevos sobre la red simulada.
//!
//! `m02 GUARDADO.pat ARCHIVO.cpr [--exe ORIGINAL.exe] [--days N] [--clients N] [--seed S] [--humans A,B] [--relay]`
//!
//! Con `--relay` (característica `ws`, WEB-7) las máquinas hablan por WebSocket a través de un
//! repetidor real en localhost (`p3_net::ws::Server`) en lugar de la red simulada, y además
//! se cotejan los hashes de todas las máquinas entre sí.
//!
//! Un anfitrión y N clientes (`NetGame`) abren el mismo guardado; cada máquina da órdenes
//! de compra/venta aleatorias desde una oficina humana y el anfitrión las secuencia en
//! pasadas de 8 ticks. Latencias distintas por enlace; el último cliente se corta a un
//! tercio de la partida y vuelve a la mitad (instantánea y resincronización); al final se altera el
//! estado de un cliente fuera del protocolo para comprobar que se detecta la divergencia.
//! Imprime un informe JSON y sale con error si algo no cuadra.
use p3_mvp::net::{NetGame, Setup, OP_OFFICE_TRADE};
use p3_mvp::headless::{BattlePacing, DEFAULT_FRAME_MS};
use p3_net::lockstep::{cross_check, Client, Config, Event, Game, Host, MemoryPort, Message, Port};
use p3_net::transport::{LinkError, MemoryNet, PeerId};
use std::sync::Arc;
use std::time::Instant;

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.0 >> 33
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

fn hex(h: &[u8]) -> String {
    h.iter().map(|b| format!("{b:02x}")).collect()
}

/// Oficinas humanas por máquina (todas si no hay suficientes humanos).
fn human_offices(game: &mut NetGame) -> Vec<usize> {
    let previous = game.session.office;
    let n = game.session.sim.persisted.len();
    let out = (0..n).filter(|&i| game.session.select_office(i).is_ok()).collect();
    game.session.select_office(previous).unwrap();
    out
}

struct DeadPort;
impl Port for DeadPort {
    fn send(&mut self, _: PeerId, _: &Message) -> Result<(), LinkError> {
        Err(LinkError::Disconnected)
    }
    fn receive(&mut self) -> Vec<(PeerId, Message)> {
        Vec::new()
    }
}

#[cfg(feature = "ws")]
struct WsNet {
    server: p3_net::ws::Server,
    ports: Vec<Option<p3_net::relay::RelayPort<p3_net::ws::WsLink>>>,
}

#[cfg(feature = "ws")]
impl WsNet {
    const ROOM: &'static str = "m02";
    fn connect(&mut self, me: usize) -> Result<(), String> {
        use p3_net::relay::RelayPort;
        let link = p3_net::ws::WsLink::connect(self.server.addr).map_err(|e| format!("{e:?}"))?;
        let mut port = RelayPort::connect(link, Self::ROOM, me == 0).map_err(|e| format!("{e:?}"))?;
        for _ in 0..5000 {
            let _ = port.receive();
            if port.peer.is_some() || port.error.is_some() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
        if port.peer != Some(me) {
            return Err(format!("el repetidor asignó {:?} a la máquina {me} (error {:?})", port.peer, port.error));
        }
        self.ports[me] = Some(port);
        Ok(())
    }
}

/// Red de la prueba: simulada en memoria o WebSocket con repetidor.
enum Net {
    Mem(MemoryNet),
    #[cfg(feature = "ws")]
    Ws { net: WsNet, now_ms: u64 },
}

impl Net {
    fn now_ms(&self) -> u64 {
        match self {
            Net::Mem(n) => n.now_ms,
            #[cfg(feature = "ws")]
            Net::Ws { now_ms, .. } => *now_ms,
        }
    }
    fn advance(&mut self, ms: u64) {
        match self {
            Net::Mem(n) => n.advance(ms),
            #[cfg(feature = "ws")]
            Net::Ws { now_ms, .. } => {
                *now_ms += ms;
                std::thread::sleep(std::time::Duration::from_millis(1));
            }
        }
    }
    fn disconnect(&mut self, me: usize) {
        match self {
            Net::Mem(n) => n.disconnect(me),
            #[cfg(feature = "ws")]
            Net::Ws { net, .. } => {
                // Caída sin despedida: se cierra el socket y se espera a que el repetidor libere el número.
                if let Some(p) = net.ports[me].take() {
                    drop(p);
                }
                for _ in 0..5000 {
                    if !net.server.peers_in(WsNet::ROOM).contains(&me) {
                        break;
                    }
                    std::thread::sleep(std::time::Duration::from_millis(1));
                }
            }
        }
    }
    fn reconnect(&mut self, me: usize) -> Result<(), String> {
        match self {
            Net::Mem(n) => {
                n.reconnect(me);
                Ok(())
            }
            #[cfg(feature = "ws")]
            Net::Ws { net, .. } => net.connect(me),
        }
    }
    fn with_port<R>(&mut self, me: usize, f: impl FnOnce(&mut dyn Port) -> R) -> R {
        match self {
            Net::Mem(n) => f(&mut MemoryPort { net: n, me }),
            #[cfg(feature = "ws")]
            Net::Ws { net, .. } => match net.ports[me].as_mut() {
                Some(p) => f(p),
                None => f(&mut DeadPort),
            },
        }
    }
    /// `(mensajes, bytes)` entregados.
    fn traffic(&self) -> (u64, u64) {
        match self {
            Net::Mem(n) => (n.delivered, n.bytes),
            #[cfg(feature = "ws")]
            Net::Ws { net, .. } => {
                let (_, _, m, b) = net.server.stats();
                (m, b)
            }
        }
    }
}

fn run() -> Result<serde_json::Value, String> {
    let mut args = Vec::new();
    let (mut exe, mut days, mut clients, mut seed, mut humans, mut relay) = (None, 30u64, 2usize, 0x4D02u64, Vec::new(), false);
    let mut it = std::env::args().skip(1);
    while let Some(a) = it.next() {
        match a.as_str() {
            "--exe" => exe = Some(it.next().ok_or("falta --exe")?),
            "--days" => days = it.next().ok_or("falta --days")?.parse().map_err(|_| "días")?,
            "--clients" => clients = it.next().ok_or("falta --clients")?.parse().map_err(|_| "clientes")?,
            "--seed" => seed = it.next().ok_or("falta --seed")?.parse().map_err(|_| "semilla")?,
            "--relay" => relay = true,
            "--humans" => humans = it.next().ok_or("falta --humans")?.split(',').map(|v| v.parse().map_err(|_| "humanos")).collect::<Result<_, _>>()?,
            _ => args.push(a),
        }
    }
    if args.len() != 2 || !(1..=7).contains(&clients) {
        return Err("uso: m02 GUARDADO.pat ARCHIVO.cpr [--exe EXE] [--days N] [--clients 1..7] [--seed S] [--humans A,B]".into());
    }
    let save = std::fs::read(&args[0]).map_err(|e| format!("partida: {e}"))?;
    let setup = Setup {
        archive: Arc::new(std::fs::read(&args[1]).map_err(|e| format!("recursos: {e}"))?),
        exe: exe.map(|p| std::fs::read(p).map(Arc::new)).transpose().map_err(|e| format!("exe: {e}"))?,
        crt: 123,
        celebration: Some(p3_core::economy::celebration::CelebrationState { next: 0, town: 0xFF }),
        investigation_local: true,
        human_battles_auto: true,
        pacing: BattlePacing::WithClock { frame_ms: DEFAULT_FRAME_MS },
        extra_humans: humans,
    };
    let t0 = Instant::now();
    let config = Config::default();
    let mut host = Host::new(NetGame::open(&save, setup.clone())?, config.clone());
    let open_ms = t0.elapsed().as_millis();
    let t = Instant::now();
    let snap = host.game.snapshot()?;
    let snapshot_ms = t.elapsed().as_millis();
    let mut probe = NetGame::open(&save, setup.clone())?;
    let t = Instant::now();
    probe.restore(&snap)?;
    let restore_ms = t.elapsed().as_millis();
    let restored_equal = probe.state_hash() == host.game.state_hash();
    drop(probe);
    let offices = human_offices(&mut host.game);
    if offices.is_empty() {
        return Err("el guardado no tiene oficinas humanas".into());
    }
    let mut peers: Vec<Client<NetGame>> = (0..clients).map(|_| Ok(Client::new(NetGame::open(&save, setup.clone())?, config.clone(), 0))).collect::<Result<_, String>>()?;
    let mut net = if relay {
        #[cfg(feature = "ws")]
        {
            let server = p3_net::ws::Server::spawn(std::net::TcpListener::bind("127.0.0.1:0").map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
            let mut n = WsNet { server, ports: (0..=clients).map(|_| None).collect() };
            for me in 0..=clients {
                n.connect(me)?;
            }
            Net::Ws { net: n, now_ms: 0 }
        }
        #[cfg(not(feature = "ws"))]
        return Err("--relay exige compilar con --features ws".into());
    } else {
        let mut n = MemoryNet::new();
        for c in 1..=clients {
            n.set_latency(0, c, [30, 120, 60, 90, 45, 75, 100][(c - 1) % 7]);
        }
        Net::Mem(n)
    };
    let mut rng = Rng(seed);
    for (i, c) in peers.iter_mut().enumerate() {
        let now = net.now_ms();
        net.with_port(i + 1, |p| c.join(now, p, format!("cliente{}", i + 1).as_bytes()));
    }
    let passes_per_day = 256 / u64::from(config.ticks_per_pass);
    let target = days * passes_per_day;
    let quit = clients; // máquina que se corta
    let (cut_at, back_at) = (target / 3, target / 2);
    let (mut cut, mut back) = (false, false);
    let mut issued = 0u64;
    let mut corrupt_at = None;
    let step_ms = 10;
    let t_run = Instant::now();
    let mut guard = 0u64;
    while host.pass < target + 20 {
        guard += 1;
        if guard > target * 400 {
            return Err(format!("sin avance: pasada {} de {}", host.pass, target));
        }
        let now = net.now_ms();
        // Órdenes: cada máquina, de media una cada dos pasadas.
        if rng.below(2) == 0 && host.pass < target {
            let who = rng.below(clients as u64 + 1) as usize;
            // Con una oficina humana por máquina, cada una manda en la suya; si no, al azar.
            let office = if offices.len() > clients { offices[who] } else { offices[rng.below(offices.len() as u64) as usize] };
            let record = [OP_OFFICE_TRADE, office as u32, rng.below(2) as u32, rng.below(20) as u32, 1 + rng.below(40) as u32];
            if who == 0 {
                host.command(record);
                issued += 1;
            } else if peers[who - 1].is_running() {
                peers[who - 1].command(record);
                issued += 1;
            }
        }
        if !cut && host.pass >= cut_at {
            net.disconnect(quit);
            cut = true;
        }
        if cut && !back && host.pass >= back_at {
            net.reconnect(quit)?;
            let c = &mut peers[quit - 1];
            net.with_port(quit, |p| c.join(now, p, b"vuelta"));
            back = true;
        }
        if corrupt_at.is_none() && host.pass >= target {
                        // Fuera del protocolo: una unidad más en la oficina del cliente 1.
            let c = &mut peers[0];
            c.game.session.sim.persisted[offices[0]].stock.stock[0] += 1;
            corrupt_at = Some(c.pass);
        }
        net.with_port(0, |p| host.step(now, p));
        for (i, c) in peers.iter_mut().enumerate() {
            net.with_port(i + 1, |p| c.step(now, p));
        }
        net.advance(step_ms);
    }
    let run_ms = t_run.elapsed().as_millis();
    // Comprobaciones.
    let corrupt_at = corrupt_at.unwrap();
    let desyncs: Vec<_> = host.events.iter().filter_map(|e| if let Event::Desync { slot, pass } = e { Some((*slot, *pass)) } else { None }).collect();
    let early_desync = desyncs.iter().any(|&(_, p)| p <= corrupt_at);
    let detected = desyncs.iter().any(|&(s, p)| s == peers[0].slot.unwrap_or(0) && p > corrupt_at);
    let mut compared = 0u64;
    let mut mismatched = 0u64;
    for c in &peers {
        for (&p, h) in &c.hashes {
            if p > corrupt_at && c.slot == peers[0].slot {
                continue;
            }
            if let Some(hh) = host.hash_at(p) {
                compared += 1;
                mismatched += u64::from(hh != *h);
            }
        }
    }
    // Comprobación cruzada: anfitrión y todos los clientes entre sí (el alterado fuera del
    // protocolo solo cuenta hasta su alteración).
    let trimmed: Vec<std::collections::BTreeMap<u64, [u8; 32]>> = peers
        .iter()
        .enumerate()
        .map(|(i, c)| c.hashes.iter().filter(|(&p, _)| i != 0 || p <= corrupt_at).map(|(&p, h)| (p, *h)).collect())
        .collect();
    let (cross_compared, cross_different) = cross_check(std::iter::once(host.hashes()).chain(trimmed.iter()));
    let game_errors: Vec<_> = host.events.iter().filter_map(|e| if let Event::GameError { error, .. } = e { Some(error.clone()) } else { None }).collect();
    let advance_errors = game_errors.iter().filter(|e| e.contains("0xC4") || !e.contains("oficina") && !e.contains("existencias") && !e.contains("batalla")).count();
    let lost = host.events.iter().any(|e| matches!(e, Event::PeerLost { slot, .. } if *slot == peers[quit - 1].slot.unwrap_or(quit)) || matches!(e, Event::PeerLost { .. }));
    let rejoined = peers[quit - 1].is_running() && peers[quit - 1].pass >= back_at;
    let summary = host.game.session.sim.summary();
    let report = serde_json::json!({
        "save": args[0], "days": days, "clients": clients, "seed": seed, "offices": offices,
        "open_ms": open_ms, "snapshot_ms": snapshot_ms, "snapshot_bytes": snap.len(), "restore_ms": restore_ms,
        "restored_hash_equal": restored_equal, "run_ms": run_ms,
        "passes": host.pass, "transport": if relay { "websocket+relay" } else { "memory" }, "virtual_ms": net.now_ms(), "stalls": host.stalls, "records_sent": host.records_sent,
        "issued": issued, "applied": host.game.applied, "rejected": host.game.rejected,
        "messages": net.traffic().0, "bytes": net.traffic().1,
        "cross_compared": cross_compared, "cross_different": cross_different,
        "hashes_compared": compared, "hash_mismatches": mismatched,
        "peer_lost": lost, "rejoined": rejoined, "resyncs": host.events.iter().filter(|e| matches!(e, Event::Resynced { .. })).count(),
        "corrupt_at": corrupt_at, "desyncs": desyncs, "early_desync": early_desync, "desync_detected": detected,
        "game_errors": game_errors.len(), "advance_errors": advance_errors, "first_errors": game_errors.iter().take(5).collect::<Vec<_>>(),
        "final_tick": summary.tick, "final_hash": summary.hash, "final_state_hash": hex(&host.game.state_hash()),
        "client_events": peers.iter().map(|c| format!("{:?}", c.events.iter().filter(|e| !matches!(e, Event::GameError { .. })).collect::<Vec<_>>())).collect::<Vec<_>>(),
    });
    let ok = mismatched == 0 && cross_different == 0 && cross_compared > 0 && compared > 0 && !early_desync && detected && lost && rejoined && advance_errors == 0;
    if !ok {
        return Err(format!("M02 no cuadra: {report}"));
    }
    Ok(report)
}

fn main() {
    match run() {
        Ok(r) => println!("{r}"),
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    }
}
