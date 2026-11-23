//! Transacción del despachador B4 (0x542E40), sobre recursos explícitos.
use crate::{
    convoy_absorption::AbsorptionState,
    pirate_group_create::{self, CreateResources},
    pirate_group_remove::{self, DissolveDelta},
};
use p3_core::{
    ai::settlement::SettlementLocation,
    economy::{
        pirate_band::{self, BandHost},
        pirate_group_callbacks::CaptainResources,
        pirate_ops::{self, OpsHost},
    },
    flatmem::{Mem, SparseMem},
    pump::Operation,
    ships::{
        convoy_plan::{self, PlanEnv},
        remove::RemoveEvent,
        tick::TickWorld,
    },
    town::Town,
};
use std::{cell::RefCell, rc::Rc};
const M: u32 = 0x6dd7a0;
const OP: u32 = 0x52000000;
const BAND: u32 = 0x56000000;
const PIECES: u32 = 0x57000000;
const PLAN: u32 = 0x5d000000;
const SHIPS: u32 = 0x58000000;
const CVS: u32 = 0x59000000;
const CAPS: u32 = 0x5a000000;
const MER: u32 = 0x5b000000;
const TOWNS: u32 = 0x5c000000;
#[derive(Clone)]
struct Shared(Rc<RefCell<SparseMem>>);
impl Mem for Shared {
    fn r8(&mut self, a: u32) -> u8 {
        self.0.borrow_mut().r8(a)
    }
    fn w8(&mut self, a: u32, v: u8) {
        self.0.borrow_mut().w8(a, v)
    }
    fn w8_seed(&mut self, a: u32) {
        self.0.borrow_mut().w8_seed(a)
    }
}
pub struct Resources<'a> {
    pub locations: &'a [SettlementLocation; 40],
    pub town_order: &'a [u8],
    pub occupied: &'a [u8; 40],
    pub piece_bytes: (u8, u8),
    pub town0: &'a Town,
    pub alderman_town: u8,
    pub weapon_experience: u32,
    pub port0: (u16, u16),
    pub ship_names: &'a [Vec<u8>],
    pub captain: CaptainResources,
    pub new_band_token: u32,
}
#[derive(Debug, Clone)]
pub struct Prepared {
    pub manager: Vec<u8>,
    pub ships: Vec<u8>,
    pub captains: Vec<u8>,
    pub convoys: Vec<u8>,
    pub merchants: Vec<u8>,
    pub towns: Vec<u8>,
    pub bands: AbsorptionState,
    pub removed_routes: Vec<u16>,
    pub remove_events: Vec<RemoveEvent>,
    pub merchant_owners: Vec<(u32, Vec<u8>)>,
    pub removed_plan: bool,
    pub consumed: bool,
}
struct SetupHost<'a> {
    resources: &'a Resources<'a>,
    piece: u32,
    port: u8,
    allocations: usize,
    error: Option<String>,
}
impl BandHost for SetupHost<'_> {
    fn alloc(&mut self, m: &mut dyn Mem, size: u32) -> u32 {
        if self.allocations != 0 || size > 0x10000 {
            self.error = Some("crecimiento de piezas no soportado".into());
            std::panic::panic_any(())
        }
        self.allocations += 1;
        for i in 0..size {
            m.w8_seed(self.piece + i)
        }
        self.piece
    }
    fn free(&mut self, _: u32) {}
    fn free_port(&mut self, _: u32) -> u8 {
        self.port
    }
    fn create_ship(
        &mut self,
        m: &mut dyn Mem,
        band: u32,
        kind: u32,
        first: u32,
        power: u32,
    ) -> u32 {
        let loc = self.resources.locations[usize::from(self.port)];
        let r = CreateResources {
            town0: self.resources.town0,
            port0: self.resources.port0,
            band_port: (loc.x as u16, loc.y as u16),
            weapon_experience: self.resources.weapon_experience,
            ship_names: self.resources.ship_names,
            captain: self.resources.captain,
        };
        match pirate_group_create::create(m, band, kind, first, power, &r) {
            Ok(v) => v,
            Err(e) => {
                self.error = Some(e);
                std::panic::panic_any(())
            }
        }
    }
}
struct Host<'a> {
    m: Shared,
    resources: &'a Resources<'a>,
    port: u8,
    piece: u32,
    band: u32,
    player: u32,
    pool_limit: u16,
    error: Option<String>,
    removed_routes: Vec<u16>,
    events: Vec<RemoveEvent>,
    merchant_owners: Vec<(u32, Vec<u8>)>,
    removed_plan: bool,
}
impl OpsHost for Host<'_> {
    fn alloc(&mut self, m: &mut dyn Mem, size: u32) -> u32 {
        if size != 24 {
            self.error = Some("tamaño de banda inválido".into());
            std::panic::panic_any(())
        }
        for i in 0..size {
            m.w8_seed(self.band + i)
        }
        self.band
    }
    fn dissolve(&mut self, band: u32) {
        let mut m = self.m.clone();
        match pirate_group_remove::dissolve(&mut m, band, self.player, self.pool_limit) {
            Ok(DissolveDelta {
                events,
                cleared_routes,
                removed_plan,
                merchant_owners,
            }) => {
                self.events.extend(events);
                self.removed_routes.extend(cleared_routes);
                self.merchant_owners.extend(merchant_owners);
                self.removed_plan |= removed_plan
            }
            Err(e) => {
                self.error = Some(e);
                std::panic::panic_any(())
            }
        }
    }
    fn setup(&mut self, band: u32, a: u32, b: u32, c: u32) {
        let mut m = self.m.clone();
        let mut h = SetupHost {
            resources: self.resources,
            piece: self.piece,
            port: self.port,
            allocations: 0,
            error: None,
        };
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            pirate_band::setup(
                &mut m,
                &mut h,
                band,
                a,
                b,
                c,
                &|id| self.resources.locations[usize::from(id)].kind,
                self.resources.piece_bytes,
            )
        }));
        if let Some(e) = h.error {
            self.error = Some(e);
            std::panic::panic_any(())
        }
        if result.is_err() {
            self.error = Some("preparación de banda inválida".into());
            std::panic::panic_any(())
        }
    }
}
fn r16(b: &[u8], i: usize) -> u16 {
    u16::from_le_bytes([b[i], b[i + 1]])
}
/// El caller retira B4 de la cola solo tras publicar todas las imágenes y
/// efectos devueltos. Requiere recursos y token de banda lógicos explícitos.
pub fn prepare(
    w: &TickWorld,
    bands: &AbsorptionState,
    op: Operation,
    r: &Resources<'_>,
) -> Result<Prepared, String> {
    if op.0[0] != 0xb4 {
        return Err("operación no es B4".into());
    }
    if bands.bands.len() != 5 || w.remove.manager.len() != 0x110 {
        return Err("gestor pirata inválido".into());
    }
    let slot = op.0[1] as usize;
    let mut base = Prepared {
        manager: w.remove.manager.to_vec(),
        ships: w.remove.ships.to_vec(),
        captains: w.remove.captains.to_vec(),
        convoys: w.remove.convoys.to_vec(),
        merchants: w.remove.merchants.to_vec(),
        towns: w.remove.towns.to_vec(),
        bands: bands.clone(),
        removed_routes: vec![],
        remove_events: vec![],
        merchant_owners: vec![],
        removed_plan: false,
        consumed: true,
    };
    if slot >= 5 {
        return Ok(base);
    }
    let ns = usize::from(r16(w.remove.manager, 0xf4));
    let nc = usize::from(r16(w.remove.manager, 0xf6));
    let ncap = usize::from(r16(w.remove.manager, 0xf2));
    if w.remove.ships.len() != ns * 0x180
        || w.remove.convoys.len() != nc * 0x3c
        || w.remove.captains.len() != ncap * 16
        || w.remove.merchants.len() != usize::from(w.remove.merchant_count) * 0x650
        || w.remove.towns.len() != w.remove.town_count as usize * 0x9f8
    {
        return Err("pools B4 desincronizados".into());
    }
    if w.remove.ships.len() > 0x1000000 || w.remove.captains.len() > 0x1000000 {
        return Err("pools B4 exceden regiones aisladas".into());
    }
    if w.remove.town_count > 40
        || r.town_order.len() < w.remove.town_count as usize
        || r.ship_names.is_empty()
        || r.captain.tick != w.time
    {
        return Err("recursos de B4 ausentes".into());
    }
    let population_town = usize::from(r.captain.day_of_year & 15);
    let weapon_town = usize::from(r.alderman_town);
    if population_town >= w.remove.town_count as usize
        || weapon_town >= w.remove.town_count as usize
        || r.weapon_experience != u32::from_le_bytes(w.remove.towns[weapon_town * 0x9f8 + 0x810..weapon_town * 0x9f8 + 0x814].try_into().unwrap())
        || r.town0.0.as_slice() != &w.remove.towns[..0x9f8]
        || r.captain.population != u32::from_le_bytes(w.remove.towns[population_town * 0x9f8 + 0x2e0..population_town * 0x9f8 + 0x2e4].try_into().unwrap())
        || r.port0 != (r.locations[0].x as u16, r.locations[0].y as u16)
    {
        return Err("recursos B4 no concuerdan con vistas vivas".into());
    }
    let selected_token = u32::from_le_bytes(w.remove.manager[12 + 4 * slot..16 + 4 * slot].try_into().unwrap());
    if (selected_token != 0) != bands.bands[slot].is_some() {
        return Err("identidad de banda B4 desincronizada".into());
    }
    for band in bands.bands.iter().flatten() {
        band.validate()?;
        if band.plan.as_ref().is_some_and(|p| p.len() > 0x100000) || band.pieces.len() > 0x100000 {
            return Err("banda excesiva".into());
        }
    }
    if bands.bands[slot].is_none() {
        if r.new_band_token == 0
            || (0..5).any(|i| {
                u32::from_le_bytes(w.remove.manager[12 + 4 * i..16 + 4 * i].try_into().unwrap())
                    == r.new_band_token
            })
        {
            return Err("identidad nueva de banda inválida".into());
        }
    }
    let env = PlanEnv {
        town_count: w.remove.town_count,
        slot_flag: r.occupied.to_vec(),
        order: r.town_order.to_vec(),
        objects: std::array::from_fn(|i| {
            if i == slot {
                None
            } else {
                bands.bands[i].as_ref().map(|b| b.header[6])
            }
        }),
        ..Default::default()
    };
    let port = convoy_plan::free_port(&env, op.0[3]);
    let mut m = SparseMem::default();
    for (base, data) in [
        (M, &*w.remove.manager),
        (SHIPS, &*w.remove.ships),
        (CAPS, &*w.remove.captains),
        (CVS, &*w.remove.convoys),
        (MER, &*w.remove.merchants),
        (TOWNS, &*w.remove.towns),
    ] {
        m.seed(base, data)
    }
    m.w32(M, CAPS);
    m.w32(M + 4, SHIPS);
    m.w32(M + 8, CVS);
    for (i, old) in bands.bands.iter().enumerate() {
        let b = BAND + i as u32 * 0x10000;
        let p = PIECES + i as u32 * 0x100000;
        let q = PLAN + i as u32 * 0x100000;
        if let Some(old) = old {
            m.seed(b, &[0; 24]);
            m.seed(b + 8, &old.header);
            if !old.pieces.is_empty() {
                m.seed(p, &old.pieces);
                m.w32(b + 4, p)
            }
            if let Some(plan) = &old.plan {
                m.seed(q, plan);
                m.w32(b, q)
            }
            m.w32(M + 12 + 4 * i as u32, b)
        } else {
            m.w32(M + 12 + 4 * i as u32, 0)
        }
    }
    m.seed(0x6de4aa, &w.remove.merchant_count.to_le_bytes());
    m.seed(0x6de4b0, &w.remove.town_count.to_le_bytes());
    m.seed(0x6de508, &TOWNS.to_le_bytes());
    m.seed(0x6de518, &MER.to_le_bytes());
    let mut bytes = Vec::new();
    for v in op.0 {
        bytes.extend_from_slice(&v.to_le_bytes())
    }
    m.seed(OP, &bytes);
    let shared = Shared(Rc::new(RefCell::new(m)));
    let mut memory = shared.clone();
    let mut host = Host {
        m: shared.clone(),
        resources: r,
        port,
        piece: PIECES + slot as u32 * 0x100000,
        band: BAND + slot as u32 * 0x10000,
        player: w.player,
        pool_limit: w.remove.pool_limit,
        error: None,
        removed_routes: vec![],
        events: vec![],
        merchant_owners: vec![],
        removed_plan: false,
    };
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        pirate_ops::op_create_pirate_band(&mut memory, &mut host, OP)
    }));
    if let Some(e) = host.error {
        return Err(e);
    }
    if result.is_err() {
        return Err("memoria o dependencias de B4 incompletas".into());
    }
    let mut m = shared.0.borrow_mut();
    let ns = usize::from(m.r16(M + 0xf4));
    let caps = usize::from(m.r16(M + 0xf2));
    if ns * 0x180 > 0x1000000 || caps * 16 > 0x1000000 {
        return Err("crecimiento B4 excede regiones aisladas".into());
    }
    base.manager = m.read(M, 0x110);
    base.manager[..0x20].copy_from_slice(&w.remove.manager[..0x20]);
    if bands.bands[slot].is_none() {
        base.manager[12 + 4 * slot..16 + 4 * slot].copy_from_slice(&r.new_band_token.to_le_bytes())
    }
    base.ships = m.read(SHIPS, ns * 0x180);
    base.captains = m.read(CAPS, caps * 16);
    base.convoys = m.read(CVS, w.remove.convoys.len());
    base.merchants = m.read(MER, w.remove.merchants.len());
    base.towns = m.read(TOWNS, w.remove.towns.len());
    let b = BAND + slot as u32 * 0x10000;
    let mut header = [0u8; 16];
    header.copy_from_slice(&m.read(b + 8, 16));
    let count = usize::from(u16::from_le_bytes([header[0], header[1]]));
    let piece = m.r32(b + 4);
    if count > 0 && piece == 0 {
        return Err("piezas B4 ausentes".into());
    }
    let new_band = p3_save::pirate_bands::PirateBand {
        header,
        plan: None,
        pieces: if count == 0 {
            vec![]
        } else {
            m.read(piece, count * 8)
        },
    };
    new_band.validate()?;
    base.bands.bands[slot] = Some(new_band);
    for i in 0..5 {
        if i == slot {
            continue;
        }
        if let Some(old) = &bands.bands[i] {
            let addr = BAND + i as u32 * 0x10000;
            if m.r32(M + 12 + 4 * i as u32) != addr {
                return Err("identidad de banda vecina modificada".into());
            }
            let mut updated = old.clone();
            updated.header.copy_from_slice(&m.read(addr + 8, 16));
            let count = usize::from(u16::from_le_bytes([updated.header[0], updated.header[1]]));
            if count != old.pieces.len() / 8 {
                return Err("tamaño de banda vecina modificado".into());
            }
            if count != 0 {
                let piece = PIECES + i as u32 * 0x100000;
                if m.r32(addr + 4) != piece {
                    return Err("piezas vecinas reubicadas".into());
                }
                updated.pieces = m.read(piece, count * 8)
            }
            let plan = m.r32(addr);
            if (plan != 0) != old.plan.is_some() {
                return Err("plano de banda vecina modificado".into());
            }
            updated.validate()?;
            base.bands.bands[i] = Some(updated);
        } else if m.r32(M + 12 + 4 * i as u32) != 0 {
            return Err("alta inesperada de banda".into());
        }
    }
    base.removed_routes = host.removed_routes;
    base.remove_events = host.events;
    base.merchant_owners = host.merchant_owners;
    base.removed_plan = host.removed_plan;
    Ok(base)
}
