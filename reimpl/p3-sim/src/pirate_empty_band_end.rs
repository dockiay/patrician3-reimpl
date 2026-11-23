//! Transactional 0x514920 for an empty pirate hideout.
use crate::convoy_absorption::AbsorptionState;
use p3_core::{
    economy::pirate_end::{self, BattleEndHost},
    flatmem::{Mem, SparseMem},
    pump::Operation,
    rng::CrtRand,
    scheduler::{Scheduler, TaskRecord},
    ships::{
        convoy,
        remove::RemoveWorld,
        tick::{ConvoyHost, TickWorld},
    },
};
use std::{cell::RefCell, rc::Rc};
const M: u32 = 0x6dd7a0;
const BAND: u32 = 0x56000000;
const PLAN: u32 = 0x57000000;
const CELLS: u32 = 0x57100000;
const PIECES: u32 = 0x57200000;
const SHIPS: u32 = 0x58000000;
const CVS: u32 = 0x59000000;
const CAPS: u32 = 0x5a000000;
const MER: u32 = 0x5b000000;
const TOWNS: u32 = 0x5c000000;
const TASKS: u32 = 0x5d000000;
#[derive(Clone)]
struct Shared(Rc<RefCell<SparseMem>>);
impl Mem for Shared {
    fn r8(&mut self, a: u32) -> u8 {
        self.0.borrow_mut().r8(a)
    }
    fn w8(&mut self, a: u32, v: u8) {
        self.0.borrow_mut().w8(a, v)
    }
}
struct Ui {
    events: Vec<u16>,
}
impl ConvoyHost for Ui {
    fn hook(&mut self, name: &'static str, args: &[u32]) -> u32 {
        assert_eq!(name, "cv_ui_a");
        self.events.push(args[0] as u16);
        0
    }
    fn read(&mut self, _: u32, _: u8) -> u32 {
        panic!("lectura auxiliar no soportada")
    }
}
struct Host {
    mem: Shared,
    ships: usize,
    caps: usize,
    cvs: usize,
    merchants: usize,
    towns: usize,
    merchant_count: u16,
    town_count: u32,
    player: u32,
    ui: bool,
    pool_limit: u16,
    rng: CrtRand,
    operations: Vec<Operation>,
    notifications: Vec<u16>,
}
impl BattleEndHost for Host {
    fn convoy_notify(&mut self, cv: u32) {
        self.notifications.push(cv as u16)
    }
    fn remove_ship(&mut self, _: u32) {
        panic!("barco defensor pendiente")
    }
    fn release_convoy(&mut self, cv: u32) {
        let read = |base, len| self.mem.0.borrow().read(base, len);
        let mut manager = read(M, 0x110);
        let mut ships = read(SHIPS, self.ships);
        let mut caps = read(CAPS, self.caps);
        let mut convoys = read(CVS, self.cvs);
        let mut merchants = read(MER, self.merchants);
        let mut towns = read(TOWNS, self.towns);
        let mut remove = RemoveWorld {
            manager: &mut manager,
            ships: &mut ships,
            captains: &mut caps,
            convoys: &mut convoys,
            merchants: &mut merchants,
            towns: &mut towns,
            pool_limit: self.pool_limit,
            merchant_count: self.merchant_count,
            town_count: self.town_count,
            ui_a: self.ui,
            ui_b: false,
        };
        let mut world = TickWorld {
            remove: &mut remove,
            time: 0,
            player: self.player,
        };
        let mut ui = Ui { events: Vec::new() };
        convoy::release_convoy(&mut world, cv as u16, &mut ui);
        self.notifications.extend(ui.events);
        for (base, data) in [
            (M, manager),
            (SHIPS, ships),
            (CAPS, caps),
            (CVS, convoys),
            (MER, merchants),
            (TOWNS, towns),
        ] {
            for (i, b) in data.into_iter().enumerate() {
                self.mem.w8(base + i as u32, b)
            }
        }
    }
    fn post_event(&mut self, v: [u32; 5]) {
        self.operations.push(Operation(v))
    }
    fn rand(&mut self) -> u32 {
        self.rng.rand() as u32
    }
    fn free(&mut self, p: u32) {
        assert!(p == PLAN || p == CELLS, "liberación de memoria desconocida")
    }
    fn object_at(&mut self, i: u32) -> u32 {
        TASKS + i * 24
    }
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
    pub scheduler: Scheduler,
    pub rng: CrtRand,
    pub operations: Vec<Operation>,
    pub notifications: Vec<u16>,
}
fn word(b: &[u8], i: usize) -> u16 {
    u16::from_le_bytes([b[i], b[i + 1]])
}
fn task_raw(t: &TaskRecord) -> Vec<u8> {
    [
        t.due.to_le_bytes().as_slice(),
        t.next.to_le_bytes().as_slice(),
        t.kind.to_le_bytes().as_slice(),
        t.payload.as_slice(),
    ]
    .concat()
}
/// Ejecuta 514920 con convoy defensor existente y vacío, sin torres operativas.
/// El caller publica operaciones, avisos y todas las imágenes conjuntamente.
pub fn prepare(
    w: &TickWorld,
    bands: &AbsorptionState,
    index: usize,
    scheduler: &Scheduler,
    network_client: bool,
    ui: bool,
    rng: CrtRand,
) -> Result<Prepared, String> {
    if index >= 5 || bands.bands.len() != 5 || w.remove.manager.len() != 0x110 {
        return Err("estado de bandas o gestor inválido".into());
    }
    let band = bands.bands[index].as_ref().ok_or("banda ausente")?;
    band.validate()?;
    if crate::pirate_empty_hideout_scene::has_operational_towers(band)? {
        return Err("torres de guarida pendientes".into());
    }
    let plan = band.plan.as_ref().ok_or("plano ausente")?;
    let ns = usize::from(word(w.remove.manager, 0xf4));
    let nc = usize::from(word(w.remove.manager, 0xf6));
    let cv = usize::from(word(&band.header, 8));
    if w.remove.ships.len() != ns * 0x180
        || w.remove.convoys.len() != nc * 0x3c
        || cv >= nc
        || w.remove.captains.len() != usize::from(word(w.remove.manager, 0xf2)) * 16
        || w.remove.merchants.len() != usize::from(w.remove.merchant_count) * 0x650
        || w.remove.towns.len() != w.remove.town_count as usize * 0x9f8
    {
        return Err("pools de guarida desincronizados".into());
    }
    if usize::from(word(w.remove.convoys, cv * 0x3c + 0xa)) < ns {
        return Err("defensores de guarida pendientes".into());
    }
    for mut cursor in [word(w.remove.manager, 0xec), word(w.remove.manager, 0xe4)] {
        let mut seen = vec![false; nc];
        while usize::from(cursor) < nc {
            let n = usize::from(cursor);
            if seen[n] {
                return Err("lista de convoyes cíclica".into());
            }
            seen[n] = true;
            cursor = word(w.remove.convoys, n * 0x3c + 8)
        }
    }
    if scheduler.records.len() != usize::from(scheduler.capacity) {
        return Err("planificador truncado".into());
    }
    let mut cursor = scheduler.head;
    let mut seen = vec![false; usize::from(scheduler.capacity)];
    while cursor < scheduler.capacity {
        let n = usize::from(cursor);
        if seen[n] {
            return Err("planificador cíclico".into());
        }
        seen[n] = true;
        cursor = scheduler.records[n].next
    }
    let mut memory = SparseMem::default();
    for (base, data) in [
        (M, &*w.remove.manager),
        (SHIPS, &*w.remove.ships),
        (CVS, &*w.remove.convoys),
        (CAPS, &*w.remove.captains),
        (MER, &*w.remove.merchants),
        (TOWNS, &*w.remove.towns),
    ] {
        memory.seed(base, data)
    }
    memory.w32(M, CAPS);
    memory.w32(M + 4, SHIPS);
    memory.w32(M + 8, CVS);
    for (i, other) in bands.bands.iter().enumerate() {
        let ptr = if other.is_some() {
            BAND + i as u32 * 0x10000
        } else {
            0
        };
        memory.w32(M + 12 + 4 * i as u32, ptr)
    }
    let at = BAND + index as u32 * 0x10000;
    memory.seed(at, &[0; 24]);
    memory.seed(at + 8, &band.header);
    if !band.pieces.is_empty() {
        memory.seed(PIECES, &band.pieces);
        memory.w32(at + 4, PIECES);
    }
    memory.w32(at, PLAN);
    memory.seed(PLAN, &vec![0; 0x6c]);
    memory.w32(PLAN + 0x68, CELLS);
    memory.seed(CELLS, &[0]);
    let _ = plan;
    let raw: Vec<u8> = scheduler.records.iter().flat_map(task_raw).collect();
    memory.seed(TASKS, &raw);
    memory.seed(0x6dd744, &scheduler.head.to_le_bytes());
    memory.seed(0x6dd748, &scheduler.capacity.to_le_bytes());
    memory.seed(0x6ce6d0, &u32::from(ui).to_le_bytes());
    memory.seed(0x6df31c, &u32::from(network_client).to_le_bytes());
    memory.seed(0x6de4aa, &w.remove.merchant_count.to_le_bytes());
    memory.seed(0x6de518, &MER.to_le_bytes());
    let shared = Shared(Rc::new(RefCell::new(memory)));
    let mut view = shared.clone();
    let mut host = Host {
        mem: shared.clone(),
        ships: w.remove.ships.len(),
        caps: w.remove.captains.len(),
        cvs: w.remove.convoys.len(),
        merchants: w.remove.merchants.len(),
        towns: w.remove.towns.len(),
        merchant_count: w.remove.merchant_count,
        town_count: w.remove.town_count,
        player: w.player,
        ui,
        pool_limit: w.remove.pool_limit,
        rng,
        operations: Vec::new(),
        notifications: Vec::new(),
    };
    if std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        pirate_end::band_on_battle_end(&mut view, &mut host, at)
    }))
    .is_err()
    {
        return Err("memoria o dependencia de cierre pirata ausente".into());
    }
    let m = shared.0.borrow();
    let mut manager = m.read(M, 0x110);
    manager[..0x20].copy_from_slice(&w.remove.manager[..0x20]);
    let mut after_bands = bands.clone();
    let changed = after_bands.bands[index].as_mut().unwrap();
    changed.header.copy_from_slice(&m.read(at + 8, 16));
    changed.plan = None;
    let mut result_scheduler = scheduler.clone();
    for (i, t) in result_scheduler.records.iter_mut().enumerate() {
        let raw = m.read(TASKS + i as u32 * 24, 24);
        t.due = u32::from_le_bytes(raw[0..4].try_into().unwrap());
        t.next = word(&raw, 4);
        t.kind = word(&raw, 6);
        t.payload.copy_from_slice(&raw[8..24]);
    }
    Ok(Prepared {
        manager,
        ships: m.read(SHIPS, w.remove.ships.len()),
        captains: m.read(CAPS, w.remove.captains.len()),
        convoys: m.read(CVS, w.remove.convoys.len()),
        merchants: m.read(MER, w.remove.merchants.len()),
        towns: m.read(TOWNS, w.remove.towns.len()),
        bands: after_bands,
        scheduler: result_scheduler,
        rng: host.rng,
        operations: host.operations,
        notifications: host.notifications,
    })
}
