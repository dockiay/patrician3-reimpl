//! M02: la sesión del cliente nuevo como simulación replicada del lockstep de `p3-net`.
//!
//! Cada máquina abre la misma partida con la misma configuración ([`Setup`]) y ejecuta
//! solo lo que llega en las pasadas del anfitrión: el avance `0xC4 {desde, hasta}` va a
//! [`crate::headless::advance_ticks_with`] y las órdenes de jugador (opcodes propios a
//! partir de `0x1001`) a los métodos de [`Session`] desde la oficina indicada.
//!
//! **Instantánea.** `Session::save` (formato S05 del original) más lo que la partida no
//! guarda y el simulador necesita para continuar igual: estado CRT, fiestas y marca de
//! noticias de batalla. El resto del estado no serializado (planificador de rutas, matriz
//! de la IA naval, notificaciones pendientes) se reconstruye al abrir; por eso el
//! lockstep restaura la instantánea en todas las máquinas a la vez (`resync_on_join`) y
//! no supone que la sesión restaurada sea idéntica a la viva.
//!
//! **Hash.** [`NetGame::state_hash`] cubre reloj, ciudades y comerciantes
//! (`Sim::summary`), imágenes de barcos, capitanes y convoyes, gestor naval, existencias
//! de oficinas y la cola del planificador. [`normalized_hash`] es el hash del guardado
//! completo (más caro), usado al unirse y en C09.

use crate::headless::{self, BattlePacing};
use crate::Session;
use p3_core::economy::celebration::CelebrationState;
use p3_core::economy::trade::TradeSide;
use p3_net::lockstep::{Game, OP_ADVANCE_TIME, OP_PEER_JOINED, OP_PEER_LEFT, OP_PEER_LOST};
use p3_net::wire::Record;
use sha2::{Digest, Sha256};
use std::sync::Arc;

/// Comercio de oficina: `[op, oficina, lado (0 compra, 1 venta), mercancía, cantidad]`.
pub const OP_OFFICE_TRADE: u32 = 0x1001;
/// Comercio de barco atracado: `[op, oficina, barco, lado << 8 | mercancía, cantidad]`.
pub const OP_SHIP_TRADE: u32 = 0x1002;
/// Construir: `[op, oficina, tipo, fila, columna]` en la ciudad de la oficina.
pub const OP_BUILD: u32 = 0x1003;
/// Zarpar: `[op, oficina, barco, destino, 0]`.
pub const OP_SAIL: u32 = 0x1004;

/// Configuración común de todas las máquinas (la misma que el recorrido de C10).
#[derive(Clone)]
pub struct Setup {
    pub archive: Arc<Vec<u8>>,
    pub exe: Option<Arc<Vec<u8>>>,
    pub crt: u32,
    pub celebration: Option<CelebrationState>,
    pub investigation_local: bool,
    pub human_battles_auto: bool,
    pub pacing: BattlePacing,
    /// Comerciantes de IA que pasan a humanos al abrir (partida para varios jugadores).
    pub extra_humans: Vec<usize>,
}

/// Lo que S05 no guarda y la sesión necesita para continuar igual.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Sidecar {
    crt: Option<u32>,
    celebration: Option<(u32, u8)>,
    battle_news_stamp: u32,
    office: u32,
}

impl Sidecar {
    fn encode(&self) -> [u8; 24] {
        let mut out = [0u8; 24];
        out[0] = u8::from(self.crt.is_some());
        out[1] = u8::from(self.celebration.is_some());
        out[2..6].copy_from_slice(&self.crt.unwrap_or(0).to_le_bytes());
        let (next, town) = self.celebration.unwrap_or((0, 0));
        out[6..10].copy_from_slice(&next.to_le_bytes());
        out[10] = town;
        out[12..16].copy_from_slice(&self.battle_news_stamp.to_le_bytes());
        out[16..20].copy_from_slice(&self.office.to_le_bytes());
        out
    }
    fn decode(b: &[u8]) -> Result<Sidecar, String> {
        if b.len() != 24 {
            return Err("instantánea: bloque adicional inválido".into());
        }
        let w = |at: usize| u32::from_le_bytes(b[at..at + 4].try_into().unwrap());
        Ok(Sidecar {
            crt: (b[0] != 0).then(|| w(2)),
            celebration: (b[1] != 0).then(|| (w(6), b[10])),
            battle_news_stamp: w(12),
            office: w(16),
        })
    }
}

/// Abre una partida con la configuración común.
pub fn open_session(save: &[u8], setup: &Setup) -> Result<Session, String> {
    let save = if setup.extra_humans.is_empty() { save.to_vec() } else { make_humans(save, &setup.extra_humans)? };
    let mut session = Session::open(&save, (*setup.archive).clone())?;
    session.sim.configure_crt_state(setup.crt);
    session.sim.host.celebration_state = setup.celebration;
    session.sim.host.investigation_local_mode = setup.investigation_local;
    session.sim.host.headless_human_battles = setup.human_battles_auto;
    if let Some(exe) = &setup.exe {
        session.configure_battle_texts(exe)?;
        session.sim.configure_election_texts(exe)?;
        session.sim.configure_hunt_texts(exe)?;
    }
    session.sim.host.siege_headless_march = true;
    session.enable_full_fleet()?;
    Ok(session)
}

/// Convierte comerciantes de IA en humanos (`+8 = 0`) en el modelo del guardado.
pub fn make_humans(save: &[u8], merchants: &[usize]) -> Result<Vec<u8>, String> {
    let mut model = p3_save::model::SaveModel::parse(save).map_err(|e| format!("modelo: {e:?}"))?;
    for &m in merchants {
        let rec = model.world.merchants.get_mut(m).ok_or("comerciante inexistente")?;
        if rec.len() < 10 {
            return Err(format!("comerciante {m}: registro corto"));
        }
        // `u16 +8 == 0` es humano (`is_human` de ships::tick).
        rec[8..10].copy_from_slice(&0u16.to_le_bytes());
    }
    Ok(model.write())
}

/// Hash del guardado completo de la sesión (estado normalizado en formato del original).
pub fn normalized_hash(session: &Session) -> Result<[u8; 32], String> {
    Ok(Sha256::digest(session.save()?).into())
}


/// Hash del estado vivo que compara el lockstep (más barato que [`normalized_hash`]).
pub fn session_hash(session: &Session) -> [u8; 32] {
    let sim = &session.sim;
    let mut h = Sha256::new();
    let s = sim.summary();
    h.update(s.tick.to_le_bytes());
    h.update(s.hash.as_bytes());
    if let Some(nav) = &sim.host.navigation {
        h.update(&nav.snapshot.manager);
        h.update(&nav.snapshot.ships);
        h.update(&nav.snapshot.captains);
        h.update(&nav.snapshot.convoys);
    }
    for o in &sim.persisted {
        for v in o.stock.stock {
            h.update(v.to_le_bytes());
        }
    }
    for c in &sim.economy.cargo_ships {
        for v in c.stock {
            h.update(v.to_le_bytes());
        }
    }
    h.update(format!("{:?}", sim.host.council_scheduler).as_bytes());
    h.update(sim.host.investigation_rng.map_or(0, |r| r.state).to_le_bytes());
    h.finalize().into()
}


pub struct NetGame {
    pub session: Session,
    pub setup: Setup,
    /// Errores de órdenes rechazadas por la sesión (parte del resultado replicado).
    pub rejected: u64,
    pub applied: u64,
}

impl NetGame {
    pub fn open(save: &[u8], setup: Setup) -> Result<NetGame, String> {
        Ok(NetGame { session: open_session(save, &setup)?, setup, rejected: 0, applied: 0 })
    }

    fn with_office<T>(&mut self, office: u32, f: impl FnOnce(&mut Session) -> Result<T, String>) -> Result<T, String> {
        let previous = self.session.office;
        self.session.select_office(office as usize)?;
        let result = f(&mut self.session);
        // La oficina local vuelve a la de partida en todas las máquinas.
        self.session.select_office(previous)?;
        result
    }
}

fn side(v: u32) -> Result<TradeSide, String> {
    match v {
        0 => Ok(TradeSide::Buy),
        1 => Ok(TradeSide::Sell),
        _ => Err("lado de comercio inválido".into()),
    }
}

impl Game for NetGame {
    fn tick(&self) -> u32 {
        self.session.sim.idle.clock.tick
    }

    fn apply(&mut self, r: &Record) -> Result<(), String> {
        self.applied += 1;
        let result = match r[0] {
            OP_ADVANCE_TIME => {
                let ticks = r[2].checked_sub(r[1]).ok_or("0xC4 hacia atrás")?;
                if r[1] != self.tick() {
                    return Err(format!("0xC4 desde {:#x} con el reloj en {:#x}", r[1], self.tick()));
                }
                headless::advance_ticks_with(&mut self.session, ticks, self.setup.pacing).map(|_| ())
            }
            OP_OFFICE_TRADE => self.with_office(r[1], |s| s.trade(side(r[2])?, r[3] as usize, r[4] as i32).map(|_| ())),
            OP_SHIP_TRADE => self.with_office(r[1], |s| s.trade_ship(r[2] as usize, side(r[3] >> 8)?, (r[3] & 0xFF) as usize, r[4] as i32).map(|_| ())),
            OP_BUILD => self.with_office(r[1], |s| s.build(r[2] as u8, r[3] as u8, r[4] as u8).map(|_| ())),
            OP_SAIL => self.with_office(r[1], |s| s.sail(r[2] as usize, r[3] as usize).map(|_| ())),
            OP_PEER_JOINED | OP_PEER_LEFT | OP_PEER_LOST => Ok(()),
            other => Err(format!("orden {other:#x} no admitida por el cliente nuevo")),
        };
        if result.is_err() && r[0] != OP_ADVANCE_TIME {
            self.rejected += 1;
        }
        result
    }

    fn state_hash(&self) -> [u8; 32] {
        session_hash(&self.session)
    }

    fn snapshot(&self) -> Result<Vec<u8>, String> {
        let save = self.session.save()?;
        let sidecar = Sidecar {
            crt: self.session.sim.host.investigation_rng.map(|r| r.state),
            celebration: self.session.sim.host.celebration_state.map(|c| (c.next, c.town)),
            battle_news_stamp: self.session.sim.host.battle_news_stamp,
            office: self.session.office as u32,
        };
        let mut out = sidecar.encode().to_vec();
        out.extend_from_slice(&save);
        Ok(out)
    }

    fn restore(&mut self, snapshot: &[u8]) -> Result<(), String> {
        if snapshot.len() < 24 {
            return Err("instantánea truncada".into());
        }
        let sidecar = Sidecar::decode(&snapshot[..24])?;
        let mut setup = self.setup.clone();
        // Los humanos añadidos ya están en el guardado de la instantánea.
        setup.extra_humans.clear();
        let mut session = open_session(&snapshot[24..], &setup)?;
        if let Some(state) = sidecar.crt {
            session.sim.configure_crt_state(state);
        }
        session.sim.host.celebration_state = sidecar.celebration.map(|(next, town)| CelebrationState { next, town });
        session.sim.host.battle_news_stamp = sidecar.battle_news_stamp;
        session.select_office(sidecar.office as usize)?;
        self.session = session;
        Ok(())
    }
}
