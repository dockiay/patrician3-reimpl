//! Puente transaccional entre 502B20, navegación viva y datos S05.
use crate::{
    expedition_finish_host::{NativeFinishHost, Notification},
    navigation::{self, ExtraVoyage, LiveVoyage},
};
use p3_core::{
    ai::reputation::Merchant,
    calendar::Calendar,
    flatmem::{Mem, SparseMem},
    route::finder::RoutePlanner,
    ships::expedition_finish,
    town::Town,
};

const M: u32 = 0x6dd7a0;
const SHIPS: u32 = 0x57000000;
const CONVOYS: u32 = 0x58000000;
const CAPTAINS: u32 = 0x59000000;
const MERCHANTS: u32 = 0x5a000000;
const TOWNS: u32 = 0x5b000000;

pub struct Inputs<'a> {
    pub nav: &'a LiveVoyage,
    pub towns: &'a [Town],
    pub merchants: &'a [Merchant],
    pub message_pool: &'a [u8],
    pub planner: Option<&'a RoutePlanner>,
    pub calendar: Calendar,
    pub tick: u32,
    pub start: u32,
    pub convoy: u16,
    pub renown_delta: i16,
    pub pool_limit: u16,
}
pub struct Staged {
    pub nav: LiveVoyage,
    pub towns: Vec<Town>,
    pub merchants: Vec<Merchant>,
    pub message_pool: Vec<u8>,
    pub planner: Option<RoutePlanner>,
    pub notifications: Vec<Notification>,
}
fn count(manager: &[u8], offset: usize) -> Result<usize, String> {
    let a = manager.get(offset..offset + 2).ok_or("gestor de expedición truncado")?;
    Ok(usize::from(u16::from_le_bytes([a[0], a[1]])))
}

/// No modifica las entradas; el llamante publica el resultado conjuntamente
/// con el estado de la tarea 0x12. Todas las tablas requeridas deben existir.
pub fn prepare(i: Inputs<'_>) -> Result<Staged, String> {
    let snap = &i.nav.snapshot;
    let sc = count(&snap.manager, 0xf4)?;
    let cc = count(&snap.manager, 0xf6)?;
    let pc = count(&snap.manager, 0xf2)?;
    if snap.manager.len() < 0x110
        || snap.ships.len() != sc * 0x180
        || snap.convoys.len() != cc * 0x3c
        || snap.captains.len() != pc * 16
        || snap.routes.len() != sc
        || i.merchants.len() > u16::MAX as usize
        || i.towns.is_empty()
        || i.towns.len() > 40
        || i.nav.ports.len() != i.towns.len()
    {
        return Err("imágenes o recursos de finalización incompatibles".into());
    }
    let cv = usize::from(i.convoy);
    if cv >= cc {
        return Err("convoy de expedición inexistente".into());
    }
    if i.merchants.iter().any(|m| m.bytes.len() != 0x650) {
        return Err("comerciante truncado".into());
    }
    let mut members = Vec::new();
    let mut seen = vec![false; sc];
    let at = cv * 0x3c;
    let mut id = usize::from(u16::from_le_bytes([snap.convoys[at + 0xa], snap.convoys[at + 0xb]]));
    while id < sc {
        if seen[id] {
            return Err("cadena de expedición cíclica".into());
        }
        seen[id] = true;
        members.push(id);
        let p = id * 0x180 + 6;
        id = usize::from(u16::from_le_bytes([snap.ships[p], snap.ships[p + 1]]));
    }
    let mut m = SparseMem::default();
    m.seed(M, &snap.manager);
    m.seed(SHIPS, &snap.ships);
    m.seed(CONVOYS, &snap.convoys);
    m.seed(CAPTAINS, &snap.captains);
    let merchants = i
        .merchants
        .iter()
        .flat_map(|v| v.bytes.iter().copied())
        .collect::<Vec<_>>();
    let towns = i.towns.iter().flat_map(|v| v.0.iter().copied()).collect::<Vec<_>>();
    m.seed(MERCHANTS, &merchants);
    m.seed(TOWNS, &towns);
    m.seed(0x6de4aa, &(i.merchants.len() as u16).to_le_bytes());
    m.seed(0x6de4b0, &(i.towns.len() as u32).to_le_bytes());
    m.seed(0x6de4b4, &i.tick.to_le_bytes());
    m.seed(0x6de508, &TOWNS.to_le_bytes());
    m.seed(0x6de518, &MERCHANTS.to_le_bytes());
    m.seed(0x672dd0, &i.renown_delta.to_le_bytes());
    m.w32(M, CAPTAINS);
    m.w32(M + 4, SHIPS);
    m.w32(M + 8, CONVOYS);
    let host = NativeFinishHost {
        message_pool: i.message_pool.to_vec(),
        calendar: i.calendar,
        player: i.nav.player,
        pool_limit: i.pool_limit,
        ports: i.nav.ports.clone(),
        planner: i.planner.cloned(),
        routes: snap.routes.clone(),
        route_points: vec![None; sc],
        notifications: vec![],
    };
    let (after, host) = expedition_finish::prepare(&m, &host, CONVOYS + (cv as u32) * 0x3c, i.start)?;
    let mut nav = i.nav.clone();
    let mut manager = after.read(M, snap.manager.len());
    manager[..12].copy_from_slice(&snap.manager[..12]);
    nav.snapshot.manager = manager;
    nav.snapshot.ships = after.read(SHIPS, snap.ships.len());
    nav.snapshot.convoys = after.read(CONVOYS, snap.convoys.len());
    nav.snapshot.captains = after.read(CAPTAINS, snap.captains.len());
    nav.snapshot.routes = host.routes;
    for id in members {
        let p = id * 0x180 + 0x134;
        if nav.snapshot.ships[p..p + 2] == 0x11u16.to_le_bytes() {
            nav.forget_captured_route(id as u16)
        }
    }
    if nav.convoy == Some(i.convoy) {
        nav.convoy = None
    }
    for v in &mut nav.additional {
        if v.convoy == Some(i.convoy) {
            v.convoy = None
        }
    }
    for (id, points) in host.route_points.into_iter().enumerate() {
        if let Some(points) = points {
            let id = id as u16;
            if nav.ship == id {
                nav.points = points;
                nav.convoy = Some(i.convoy)
            } else if let Some(v) = nav.additional.iter_mut().find(|v| v.ship == id) {
                v.points = points;
                v.convoy = Some(i.convoy)
            } else {
                nav.additional.push(ExtraVoyage {
                    ship: id,
                    convoy: Some(i.convoy),
                    points,
                })
            }
        }
    }
    navigation::refresh_lists(&mut nav.snapshot)?;
    let mut out_merchants = i.merchants.to_vec();
    for (v, b) in out_merchants
        .iter_mut()
        .zip(after.read(MERCHANTS, merchants.len()).chunks_exact(0x650))
    {
        v.bytes.copy_from_slice(b)
    }
    let mut out_towns = i.towns.to_vec();
    for (v, b) in out_towns
        .iter_mut()
        .zip(after.read(TOWNS, towns.len()).chunks_exact(0x9f8))
    {
        v.0.copy_from_slice(b)
    }
    Ok(Staged {
        nav,
        towns: out_towns,
        merchants: out_merchants,
        message_pool: host.message_pool,
        planner: host.planner,
        notifications: host.notifications,
    })
}
