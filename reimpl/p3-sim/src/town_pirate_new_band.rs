//! Construcción de la primera banda requerida por `0x5063C0`.
//!
//! Debe invocarse sobre memoria de trabajo transaccional: la banda, el pool de
//! piezas y los barcos de reserva solo se publican después de completar el alta.
use crate::pirate_group_create::{self, CreateResources};
use p3_core::{
    ai::settlement::SettlementLocation,
    economy::{pirate_band::{self, BandHost}, pirate_group_callbacks::CaptainResources},
    flatmem::Mem,
    ships::convoy_plan::{self, PlanEnv},
    town::Town,
};

pub struct Resources<'a> {
    pub tick: u32,
    pub difficulty: u8,
    pub town_count: usize,
    pub town_order: &'a [u8],
    pub occupied: &'a [u8; 40],
    pub locations: &'a [SettlementLocation; 40],
    pub piece_bytes: (u8, u8),
    pub town0: &'a Town,
    pub port0: (u16, u16),
    pub names: &'a [Vec<u8>],
    pub captain: CaptainResources,
}

struct Host<'a> {
    r: &'a Resources<'a>,
    piece_base: u32,
    port: u8,
    allocated: bool,
    error: Option<String>,
}
#[derive(Debug)]
struct Abort;
impl BandHost for Host<'_> {
    fn alloc(&mut self, m: &mut dyn Mem, size: u32) -> u32 {
        if self.allocated || size > 0x10000 {
            self.error = Some("crecimiento de piezas de banda no admitido".into());
            std::panic::panic_any(Abort)
        }
        self.allocated = true;
        for i in 0..size { m.w8_seed(self.piece_base + i); }
        self.piece_base
    }
    fn free(&mut self, _addr: u32) {}
    fn free_port(&mut self, _town: u32) -> u8 { self.port }
    fn create_ship(&mut self, m: &mut dyn Mem, band: u32, kind: u32, first: u32, power: u32) -> u32 {
        let loc = self.r.locations[usize::from(self.port)];
        let resources = CreateResources { town0:self.r.town0, port0:self.r.port0,
            band_port:(loc.x as u16, loc.y as u16), weapon_experience:self.r.town0.u32(0x810),
            ship_names:self.r.names, captain:self.r.captain };
        match pirate_group_create::create(m, band, kind, first, power, &resources) {
            Ok(ship) => ship,
            Err(e) => { self.error = Some(e); std::panic::panic_any(Abort) }
        }
    }
}

/// Inicializa la ranura 4 como `0x513720 → 0x513D80`, incluidos los barcos
/// de reserva. El puntero `band` y el área de piezas deben estar desocupados.
pub fn setup(m: &mut dyn Mem, band: u32, piece_base: u32, r: &Resources<'_>) -> Result<(), String> {
    if band == 0 || piece_base == 0 || r.town_count > 40 || r.town_order.len() < r.town_count
        || r.names.is_empty() || r.captain.tick != r.tick {
        return Err("recursos de banda nueva incompletos".into());
    }
    let env = PlanEnv { town_count:r.town_count as u32, slot_flag:r.occupied.to_vec(),
        order:r.town_order.to_vec(), objects:[None; 5], ..Default::default() };
    let port = convoy_plan::free_port(&env, r.tick % 40);
    for i in 0..24 { m.w8_seed(band + i); }
    for offset in [0xc, 0xe, 0x10, 0x12] { m.w16(band + offset, u16::MAX); }
    m.w8(band + 0x14, 0xff);
    m.w32(0x6dd7a0 + 12 + 4 * 4, band);
    let mut host = Host { r, piece_base, port, allocated:false, error:None };
    let seed = r.town0.u32(4).wrapping_add(r.tick);
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        pirate_band::setup(m, &mut host, band, r.tick % 40, u32::from(r.difficulty), seed,
            &|id| r.locations[usize::from(id)].kind, r.piece_bytes);
    }));
    if let Some(e) = host.error { return Err(e); }
    result.map_err(|_| "error al construir banda pirata".to_string())
}
