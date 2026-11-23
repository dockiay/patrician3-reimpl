//! Alta transaccional del barco pirata de un suceso urbano (0x5063C0).
//! La creación de la primera banda necesita sus recursos de mapa y piezas.
use p3_core::{
    economy::{
        pirate_b5_memory::{Effect, Effects, NativeBasics},
        pirate_group_callbacks::{self, CaptainResources},
        pirate_group_tick,
        pirate_loadout::{self, LoadoutHost},
        pirate_small,
    },
    flatmem::{Mem, SparseMem},
    ships::factory,
    town::{Town, TOWN_SIZE},
};
use p3_save::pirate_bands::PirateBand;
use std::{cell::RefCell, rc::Rc};
const M: u32 = 0x6dd7a0;
const BAND: u32 = 0x56000000;
const SHIPS: u32 = 0x58000000;
const CV: u32 = 0x59000000;
const CAPS: u32 = 0x5a000000;
const MER: u32 = 0x5b000000;
const TOWNS: u32 = 0x5c000000;
const SETTINGS: u32 = 0x5e000000;
const SCRATCH: u32 = 0x5f000000;
const PIECES: u32 = 0x5d000000;
// Rectángulos con signo de 0x00673564 en el ejecutable inglés contrastado.
const PIRATE_ZONE_BOXES: [u8; 24] = [
    0xba, 0x01, 0xf1, 0x05, 0x92, 0x02, 0x9b, 0x06,
    0xd9, 0x08, 0x93, 0x00, 0x1c, 0x09, 0xc5, 0x01,
    0xd9, 0x05, 0xef, 0x04, 0x2a, 0x06, 0xa4, 0x05,
];
#[derive(Clone)]
struct Shared(Rc<RefCell<SparseMem>>);
impl Mem for Shared {
    fn r8(&mut self, p: u32) -> u8 {
        self.0.borrow_mut().r8(p)
    }
    fn w8(&mut self, p: u32, v: u8) {
        self.0.borrow_mut().w8(p, v)
    }
    fn w8_seed(&mut self, p: u32) {
        self.0.borrow_mut().w8_seed(p)
    }
}
struct Loadout {
    m: Shared,
    basic: NativeBasics,
    error: Option<String>,
}
impl Loadout {
    fn call(&mut self, e: Effect) -> u32 {
        match self.basic.call(&mut self.m, e) {
            Ok(v) => v,
            Err(x) => {
                self.error = Some(x);
                std::panic::panic_any(Abort)
            }
        }
    }
}
#[derive(Debug)]
struct Abort;
impl LoadoutHost for Loadout {
    fn ship_517770(&mut self, p: u32, v: u32) {
        self.call(Effect::LoadoutMark(p, v));
    }
    fn ship_5181d0(&mut self, p: u32) {
        self.call(Effect::LoadoutSlots(p));
    }
    fn ship_5182b0(&mut self, p: u32) -> u32 {
        self.call(Effect::LoadoutRecount(p))
    }
    fn town_52bb00(&mut self, p: u32) -> u32 {
        self.call(Effect::LoadoutTownLevel(p))
    }
    fn ship_51a4e0(&mut self, p: u32, a: u32, b: u32) {
        self.call(Effect::LoadoutFit(p, a, b));
    }
    fn ship_5185c0(&mut self, p: u32) -> u32 {
        self.call(Effect::LoadoutCrew(p))
    }
}
#[derive(Clone,Copy)]
pub struct Resources<'a> {
    pub tick: u32,
    pub difficulty: u8,
    pub alderman: u8,
    pub port0: (u16, u16),
    pub names: &'a [Vec<u8>],
    pub captain: CaptainResources,
    pub player: u32,
    pub new_band: Option<NewBandResources<'a>>,
}
#[derive(Clone,Copy)]
pub struct NewBandResources<'a> {
    pub difficulty: u8,
    pub town_order: &'a [u8],
    pub occupied: &'a [u8; 40],
    pub locations: &'a [p3_core::ai::settlement::SettlementLocation; 40],
    pub piece_bytes: (u8, u8),
}
#[derive(Clone,Copy)]
pub struct Inputs<'a> {
    pub manager: &'a [u8],
    pub ships: &'a [u8],
    pub captains: &'a [u8],
    pub convoys: &'a [u8],
    pub merchants: &'a [u8],
    pub towns: &'a [u8],
    pub merchant_count: u16,
    pub bands: &'a [Option<PirateBand>],
    pub target: u16,
    pub mode: u8,
    pub resources: Resources<'a>,
}
pub struct Prepared {
    pub manager: Vec<u8>,
    pub ships: Vec<u8>,
    pub captains: Vec<u8>,
    pub convoys: Vec<u8>,
    pub bands: Vec<Option<PirateBand>>,
    pub ship: u16,
}
fn read(m: &mut dyn Mem, p: u32, n: usize) -> Vec<u8> {
    (0..n).map(|i| m.r8(p + i as u32)).collect()
}
fn write(m: &mut dyn Mem, p: u32, b: &[u8]) {
    for (i, &v) in b.iter().enumerate() {
        m.w8(p + i as u32, v)
    }
}
/// Usa los constructores existentes y reproduce la inserción en una banda viva.
/// No modifica los argumentos, incluso si falta memoria o una dependencia.
pub fn prepare(i: Inputs<'_>) -> Result<Prepared, String> {
    if i.manager.len() < 0x110
        || i.bands.len() != 5
        || i.ships.len() % 0x180 != 0
        || i.captains.len() % 16 != 0
        || i.convoys.len() % 0x3c != 0
        || i.towns.len() % TOWN_SIZE != 0
        || i.towns.is_empty()
        || usize::from(i.merchant_count) * 0x650 > i.merchants.len()
        || i.resources.names.is_empty()
        || usize::from(i.resources.alderman) >= i.towns.len() / TOWN_SIZE
        || i.resources.captain.tick != i.resources.tick
    {
        return Err("estado de alta pirata incompleto".into());
    }
    let word = |at: usize| usize::from(u16::from_le_bytes([i.manager[at], i.manager[at + 1]]));
    if word(0xf4) * 0x180 != i.ships.len()
        || word(0xf2) * 16 != i.captains.len()
        || word(0xf6) * 0x3c != i.convoys.len()
    {
        return Err("capacidades de alta pirata incompatibles".into());
    }
    let existing = i.bands.iter().position(Option::is_some);
    let band = existing.unwrap_or(4);
    let original = i.bands[band].as_ref();
    if let Some(original)=original { original.validate().map_err(str::to_owned)?; }
    else if i.resources.new_band.is_none() { return Err("recursos de banda pirata nueva ausentes".into()); }
    let mut memory = SparseMem::default();
    for (p, b) in [
        (M, i.manager),
        (SHIPS, i.ships),
        (CAPS, i.captains),
        (CV, i.convoys),
        (MER, i.merchants),
        (TOWNS, i.towns),
    ] {
        memory.seed(p, b)
    }
    memory.seed(BAND, &[0; 24]);
    memory.seed(0x673564, &PIRATE_ZONE_BOXES);
    if let Some(original)=original { memory.seed(BAND + 8, &original.header); }
    memory.w32(M, CAPS);
    memory.w32(M + 4, SHIPS);
    memory.w32(M + 8, CV);
    for n in 0..5 {
        memory.w32(M + 12 + 4 * n as u32, if existing.is_some() && n == band { BAND } else { 0 })
    }
    memory.seed(0x6de518, &MER.to_le_bytes());
    memory.seed(0x6de508, &TOWNS.to_le_bytes());
    memory.seed(0x6de4aa, &i.merchant_count.to_le_bytes());
    memory.seed(0x6de4b0, &((i.towns.len() / TOWN_SIZE) as u32).to_le_bytes());
    memory.seed(0x6de4b4, &i.resources.tick.to_le_bytes());
    memory.seed(0x6de52e, &[i.resources.alderman]);
    memory.seed(0x6cc3e8, &SETTINGS.to_le_bytes());
    memory.seed(SETTINGS + 0x12, &[i.resources.difficulty]);
    memory.seed(SCRATCH, &u32::MAX.to_le_bytes());
    let shared = Shared(Rc::new(RefCell::new(memory)));
    let mut m = shared.clone();
    let mut host = Loadout {
        m: shared.clone(),
        basic: NativeBasics {
            player: i.resources.player,
            ..Default::default()
        },
        error: None,
    };
    let work = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> Result<u16, String> {
        let count = m.r16(M + 0xf4);
        let (x, y) = if i.target < count {
            let p = SHIPS + u32::from(i.target) * 0x180;
            (m.r16(p + 0x1e), m.r16(p + 0x22))
        } else {
            (0x3d2, 0x317)
        };
        let mode = i.mode % 3;
        let class = match mode {
            0 => 0,
            1 => 1,
            _ if i.target < count && pirate_small::point_in_pirate_zone_box(&mut m, x, y) => 1,
            _ => 3,
        };
        let (addr, id) = pirate_group_callbacks::allocate_reserve_ship(&mut m, i.resources.tick, i.resources.port0)?;
        let mut ship = read(&mut m, addr, 0x180);
        let town0 = Town::from_bytes(&read(&mut m, TOWNS, TOWN_SIZE));
        factory::init_ship_stats(&town0, &mut ship, class).ok_or("estadísticas de alta pirata inválidas")?;
        let health = ship[0x14..0x18].to_vec();
        ship[0x18..0x1c].copy_from_slice(&health);
        crate::absorb_ship_refresh::refresh(&mut ship, if mode < 2 { 1 } else { 2 }).map_err(str::to_owned)?;
        let mut cursor = m.r16(M + 0xee);
        let name = factory::next_ship_name_index(&mut cursor, m.r16(M + 0xf0), i.resources.names.len())
            .ok_or("banco de nombres piratas inválido")?;
        m.w16(M + 0xee, cursor);
        ship[0x15e..0x160].copy_from_slice(&name.to_le_bytes());
        let name = &i.resources.names[usize::from(name)];
        let n = name.iter().position(|&v| v == 0).unwrap_or(name.len()).min(31);
        ship[0x160..0x160 + n].copy_from_slice(&name[..n]);
        ship[0x160 + n] = 0;
        ship[0x3c] |= 1;
        let crew = pirate_group_tick::target_crew(&ship).ok_or("tripulación pirata inválida")?;
        ship[0x40..0x42].copy_from_slice(&crew.to_le_bytes());
        let captain = pirate_group_callbacks::allocate_pirate_captain(&mut m, i.resources.captain)?;
        ship[0x42..0x44].copy_from_slice(&captain.to_le_bytes());
        m.w8(CAPS + u32::from(captain) * 16 + 11, (mode + 2) * 0x33);
        write(&mut m, addr, &ship);
        pirate_loadout::ship_reset_loadout_for_pirate(&mut m, &mut host, u32::from(id), SCRATCH);
        host.basic.call(&mut m, Effect::ShipUpdate(addr))?;
        if existing.is_none() {
            let cfg=i.resources.new_band.as_ref().ok_or("recursos de banda pirata nueva ausentes")?;
            // 5063C0 conserva `ship*` mientras 513D80 crea reservas. Si
            // éstas realojan el pool, el EXE escribe luego en el bloque viejo.
            // El modelo rechaza esa situación hasta representarla con toda
            // su procedencia; no debe inventar una actualización del nuevo.
            let count=usize::from(m.r16(M+0xf4));let mut free=m.r16(M+0xe2);
            let mut seen=vec![false;count];let mut available=0usize;
            while usize::from(free)<count {
                if seen[usize::from(free)]{return Err("lista libre naval cíclica".into())}
                seen[usize::from(free)]=true;available+=1;free=m.r16(SHIPS+u32::from(free)*0x180+6);
            }
            if available<usize::from(cfg.difficulty&3)+1{return Err("crecimiento naval durante banda nueva: puntero original obsoleto".into())}
            let resources=crate::town_pirate_new_band::Resources{tick:i.resources.tick,difficulty:cfg.difficulty,
                town_count:i.towns.len()/TOWN_SIZE,town_order:cfg.town_order,occupied:cfg.occupied,
                locations:cfg.locations,piece_bytes:cfg.piece_bytes,town0:&town0,port0:i.resources.port0,
                names:i.resources.names,captain:i.resources.captain};
            crate::town_pirate_new_band::setup(&mut m,BAND,PIECES,&resources)?;
        }
        let mut at = m.r16(BAND + 12);
        let mut seen = vec![false; usize::from(m.r16(M + 0xf4))];
        while usize::from(at) < seen.len() {
            let n = usize::from(at);
            if seen[n] {
                return Err("cadena de banda pirata cíclica".into());
            }
            seen[n] = true;
            at = m.r16(SHIPS + u32::from(at) * 0x180 + 4)
        }
        pirate_small::band_append_ship(&mut m, BAND, u32::from(id));
        pirate_small::ship_set_pirate_band_index(&mut m, addr, band as u32);
        let flags = m.r8(addr + 0x3d) | 0x20;
        m.w8(addr + 0x3d, flags);
        m.w16(addr + 0x1e, x);
        m.w16(addr + 0x134, 0x12);
        m.w16(addr + 0x1c, 0x7fff);
        m.w16(addr + 0x20, 0x7fff);
        m.w16(addr + 0x22, y);
        m.w16(addr + 0x50, i.target);
        m.w16(addr + 0x52, i.target);
        // La inserción original puede no terminar si el orden en x está
        // cíclico. Comprobar ambos sentidos antes de ejecutar ese algoritmo.
        let count=usize::from(m.r16(M+0xf4));let head=m.r16(M+0xea);
        for link in [0xau32,0xcu32] {
            let mut seen=vec![false;count];let mut current=head;
            while usize::from(current)<count {
                if seen[usize::from(current)]{return Err("orden naval en x cíclico".into())}
                seen[usize::from(current)]=true;current=m.r16(SHIPS+u32::from(current)*0x180+link);
            }
        }
        let cv = host.basic.call(&mut m, Effect::CreateConvoy(id))? as u16;
        host.basic.call(&mut m, Effect::ConvoyDone(u32::from(cv)))?;
        let c = CV + u32::from(cv) * 0x3c;
        m.w16(c + 0x16, 0);
        let flags = m.r8(c + 0x14) | 0x40;
        m.w8(c + 0x14, flags);
        m.w16(addr + 0x138, 0);
        let flags = m.r8(addr + 0x136) | 0xc0;
        m.w8(addr + 0x136, flags);
        Ok(id)
    }));
    if let Some(err) = host.error {
        return Err(err);
    }
    let id = match work {
        Ok(r) => r?,
        Err(_) => return Err("memoria insuficiente al crear pirata".into()),
    };
    let mut m = shared.clone();
    let ship_count = usize::from(m.r16(M + 0xf4));
    let captain_count = usize::from(m.r16(M + 0xf2));
    let convoy_count = usize::from(m.r16(M + 0xf6));
    let ships = read(&mut m, SHIPS, ship_count * 0x180);
    let captains = read(&mut m, CAPS, captain_count * 16);
    let convoys = read(&mut m, CV, convoy_count * 0x3c);
    let mut manager = read(&mut m, M, i.manager.len());
    manager[..0x20].copy_from_slice(&i.manager[..0x20]);
    let mut bands = i.bands.to_vec();
    if let Some(original)=bands[band].as_mut(){
        original.header.copy_from_slice(&read(&mut m,BAND+8,16));
    }else{
        let header:[u8;16]=read(&mut m,BAND+8,16).try_into().unwrap();
        let capacity=usize::from(u16::from_le_bytes([header[0],header[1]]));
        bands[band]=Some(PirateBand{header,plan:None,pieces:read(&mut m,PIECES,capacity*8)});
    }
    Ok(Prepared {
        manager,
        ships,
        captains,
        convoys,
        bands,
        ship: id,
    })
}
