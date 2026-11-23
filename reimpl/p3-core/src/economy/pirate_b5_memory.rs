//! Puente de memoria transaccional para la operación B5/514D40.
//! Los callbacks deben ejecutar los subsistemas reales; un error aborta el conjunto.
use super::{
    pirate_b5,
    pirate_launch::{self, LaunchHost},
};
use crate::{
    flatmem::{Mem, SparseMem},
    pump::Operation,
};
use std::{cell::RefCell, rc::Rc};
const BAND_POINTERS: u32 = 0x6dd7ac;
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Effect {
    TraderCleanup(u32),
    RngSeed(u32),
    RngNext(u32),
    MerchantByte(u32),
    SendLetter([u8; 16]),
    NewTask(u32),
    ShipDetach(u32, u16),
    ShipRelease(u16),
    Notify(u32),
    ShipFinish(u16),
    CreateConvoy(u16),
    ShipSetup(u16, u32),
    FreeSlot(u16),
    ConvoyClear(u32),
    ConvoyDone(u32),
    ShipUpdate(u32),
    LoadoutMark(u32, u32),
    LoadoutSlots(u32),
    LoadoutRecount(u32),
    LoadoutTownLevel(u32),
    LoadoutFit(u32, u32, u32),
    LoadoutCrew(u32),
}
/// Implementaciones clonables con efectos reales sobre `memory`; devuelven
/// Err si no pueden cumplir un callback sin publicar cambios parciales.
pub trait Effects: Clone {
    fn call(&mut self, memory: &mut dyn Mem, effect: Effect) -> Result<u32, String>;
    fn native_loadout(&self) -> bool {
        false
    }
    /// Materializa dependencias temporales en la misma copia antes de extraerla.
    /// Un error descarta tanto la memoria como el anfitrión preparados.
    fn finalize(&mut self, _memory: &mut dyn Mem) -> Result<(), String> {
        Ok(())
    }
}
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
struct Adapter<E> {
    memory: Shared,
    host: E,
    error: Option<String>,
    setup_value: u32,
}
impl<E: Effects> Adapter<E> {
    fn invoke(&mut self, e: Effect) -> u32 {
        match self.host.call(&mut self.memory, e) {
            Ok(v) => v,
            Err(err) => {
                self.error = Some(err);
                std::panic::panic_any(Abort)
            }
        }
    }
}
#[derive(Debug)]
struct Abort;
struct LoadoutAdapter<'a, E> {
    memory: Shared,
    host: &'a mut E,
    error: &'a mut Option<String>,
}
impl<E: Effects> LoadoutAdapter<'_, E> {
    fn invoke(&mut self, e: Effect) -> u32 {
        match self.host.call(&mut self.memory, e) {
            Ok(v) => v,
            Err(error) => {
                *self.error = Some(error);
                std::panic::panic_any(Abort)
            }
        }
    }
}
impl<E: Effects> crate::economy::pirate_loadout::LoadoutHost for LoadoutAdapter<'_, E> {
    fn ship_517770(&mut self, p: u32, a: u32) {
        self.invoke(Effect::LoadoutMark(p, a));
    }
    fn ship_5181d0(&mut self, p: u32) {
        self.invoke(Effect::LoadoutSlots(p));
    }
    fn ship_5182b0(&mut self, p: u32) -> u32 {
        self.invoke(Effect::LoadoutRecount(p))
    }
    fn town_52bb00(&mut self, p: u32) -> u32 {
        self.invoke(Effect::LoadoutTownLevel(p))
    }
    fn ship_51a4e0(&mut self, p: u32, a: u32, b: u32) {
        self.invoke(Effect::LoadoutFit(p, a, b));
    }
    fn ship_5185c0(&mut self, p: u32) -> u32 {
        self.invoke(Effect::LoadoutCrew(p))
    }
}
impl<E: Effects> LaunchHost for Adapter<E> {
    fn trader_cleanup(&mut self, p: u32) {
        self.invoke(Effect::TraderCleanup(p));
    }
    fn rng_seed(&mut self, t: u32) {
        self.invoke(Effect::RngSeed(t));
    }
    fn rng_next(&mut self, n: u32) -> u8 {
        self.invoke(Effect::RngNext(n)) as u8
    }
    fn merchant_byte(&mut self, p: u32) -> u8 {
        self.invoke(Effect::MerchantByte(p)) as u8
    }
    fn send_letter(&mut self, r: &[u8; 16]) {
        self.invoke(Effect::SendLetter(*r));
    }
    fn new_task(&mut self, _: &mut dyn Mem, d: u32) -> u32 {
        self.invoke(Effect::NewTask(d))
    }
    fn ship_detach(&mut self, p: u32, n: u16) {
        self.invoke(Effect::ShipDetach(p, n));
    }
    fn ship_release(&mut self, n: u16) {
        self.invoke(Effect::ShipRelease(n));
    }
    fn notify(&mut self, p: u32) {
        self.invoke(Effect::Notify(p));
    }
    fn ship_finish(&mut self, n: u16) {
        self.invoke(Effect::ShipFinish(n));
    }
    fn create_convoy(&mut self, _: &mut dyn Mem, n: u16) -> u16 {
        self.invoke(Effect::CreateConvoy(n)) as u16
    }
    fn ship_setup(&mut self, n: u16) {
        if !self.host.native_loadout() {
            self.setup_value = self.invoke(Effect::ShipSetup(n, self.setup_value));
            return;
        }
        const SCRATCH: u32 = 0xfffe0000;
        if self.memory.0.borrow().contains_any(SCRATCH, 4) {
            self.error = Some("temporal de equipamiento B5 solapado".into());
            std::panic::panic_any(Abort)
        }
        let mut memory = self.memory.clone();
        for i in 0..4 {
            memory.w8_seed(SCRATCH + i)
        }
        memory.w32(SCRATCH, self.setup_value);
        let mut nested = LoadoutAdapter {
            memory: self.memory.clone(),
            host: &mut self.host,
            error: &mut self.error,
        };
        crate::economy::pirate_loadout::ship_reset_loadout_for_pirate(&mut memory, &mut nested, u32::from(n), SCRATCH);
        self.setup_value = memory.r32(SCRATCH);
        for i in 0..4 {
            memory.0.borrow_mut().unseed(SCRATCH + i);
        }
    }
    fn free_slot(&mut self, n: u16) {
        self.invoke(Effect::FreeSlot(n));
    }
    fn convoy_clear(&mut self, p: u32) {
        self.invoke(Effect::ConvoyClear(p));
    }
    fn convoy_done(&mut self, n: u32) {
        self.invoke(Effect::ConvoyDone(n));
    }
    fn ship_update(&mut self, p: u32) {
        self.invoke(Effect::ShipUpdate(p));
    }
}
/// Ejecuta gate y función completa en una copia. El llamante confirma ambas
/// imágenes juntas sólo si esta función retorna Ok; no consume operaciones.
pub fn prepare<E: Effects>(op: Operation, before: &SparseMem, host: &E) -> Result<(SparseMem, E), String> {
    let shared = Shared(Rc::new(RefCell::new(before.clone())));
    let mut memory = shared.clone();
    let mut adapter = Adapter {
        memory: shared.clone(),
        host: host.clone(),
        error: None,
        setup_value: 0,
    };
    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let present = std::array::from_fn(|i| memory.r32(BAND_POINTERS + 4 * i as u32) != 0);
        if let Some(request) = pirate_b5::decode(op, present) {
            let band = memory.r32(BAND_POINTERS + 4 * u32::from(request.band));
            adapter.setup_value = u32::from(request.loadout_initial);
            pirate_launch::launch_pirate_fleet(&mut memory, &mut adapter, band, u32::from(request.limit));
            if let Err(error) = adapter.host.finalize(&mut memory) {
                adapter.error = Some(error);
                std::panic::panic_any(Abort)
            }
        }
    }));
    if let Some(error) = adapter.error {
        return Err(error);
    }
    if let Err(panic) = outcome {
        let detail = panic
            .downcast_ref::<String>()
            .map(String::as_str)
            .or_else(|| panic.downcast_ref::<&str>().copied());
        return Err(match detail {
            Some(s) => format!("memoria B5 incompleta o estado original inválido: {s}"),
            None => "memoria B5 incompleta o estado original inválido".into(),
        });
    }
    let result = shared.0.borrow().clone();
    Ok((result, adapter.host))
}

/// Implementa dependencias acreditadas del original sobre la misma memoria;
/// los demás callbacks devuelven error explícito hasta conectarse al anfitrión.
#[derive(Clone, Default)]
pub struct NativeBasics {
    pub rng: crate::rng::LcgRandom,
    pub player: u32,
    pub ui_present: bool,
    pub cleared_routes: Vec<u16>,
    pub ui_invalidations: Vec<u16>,
}
impl Effects for NativeBasics {
    fn native_loadout(&self) -> bool {
        true
    }
    fn call(&mut self, m: &mut dyn Mem, e: Effect) -> Result<u32, String> {
        fn read(m: &mut dyn Mem, p: u32, n: usize) -> Vec<u8> {
            (0..n).map(|i| m.r8(p + i as u32)).collect()
        }
        fn write(m: &mut dyn Mem, p: u32, b: &[u8]) {
            for (i, &v) in b.iter().enumerate() {
                m.w8(p + i as u32, v)
            }
        }
        match e {
            Effect::LoadoutMark(ptr, bits) => {
                let flags = m.r16(ptr + 0x3c) | bits as u16;
                m.w16(ptr + 0x3c, flags);
                let cid = m.r16(ptr + 8);
                let count = m.r16(0x6dd896);
                if cid < count {
                    let cv = m.r32(0x6dd7a8) + u32::from(cid) * 0x3c;
                    let f = m.r32(cv + 0x18) | bits;
                    m.w32(cv + 0x18, f)
                }
                Ok(0)
            }
            Effect::LoadoutSlots(ptr) => {
                let mut ship = read(m, ptr, 0x180);
                crate::ships::factory::recompute_weapon_slots(&mut ship).ok_or("ranuras B5 inválidas")?;
                write(m, ptr, &ship);
                Ok(0)
            }
            Effect::LoadoutRecount(ptr) => {
                let mut ship = read(m, ptr, 0x180);
                let free = crate::ships::cargo::recompute_load(&mut ship);
                write(m, ptr, &ship);
                Ok(free as u32)
            }
            Effect::LoadoutTownLevel(ptr) => {
                let t = crate::town::Town::from_bytes(&read(m, ptr, crate::town::TOWN_SIZE));
                Ok(u32::from(crate::economy::production::shipyard_level(&t)))
            }
            Effect::LoadoutFit(ptr, target, level) => {
                let mut ship = read(m, ptr, 0x180);
                crate::economy::pirate_group_callbacks::arm_ship(&mut ship, target, level)?;
                write(m, ptr, &ship);
                Ok(0)
            }
            Effect::LoadoutCrew(ptr) => {
                let ship = read(m, ptr, 0x180);
                Ok(u32::from(
                    crate::economy::pirate_group_tick::target_crew(&ship).ok_or("tripulación B5 inválida")?,
                ))
            }
            Effect::RngSeed(seed) => {
                self.rng.set_seed(seed);
                Ok(0)
            }
            Effect::RngNext(n) => {
                if n == 0 {
                    return Err("divisor aleatorio B5 nulo".into());
                }
                Ok(self.rng.next_below(n))
            }
            Effect::MerchantByte(ptr) => {
                let merchant = crate::ai::reputation::Merchant {
                    bytes: read(m, ptr, 0x650),
                };
                Ok(u32::from(merchant.get_merchant_epithet_index()))
            }
            Effect::ShipUpdate(ptr) => {
                let mut ship = read(m, ptr, 0x180);
                crate::ships::cargo::recompute_load(&mut ship);
                write(m, ptr, &ship);
                Ok(0)
            }
            Effect::ShipDetach(ptr, index) => {
                struct Routes {
                    ui: bool,
                }
                impl crate::ships::route::RouteHost for Routes {
                    fn points_of(&mut self, _: u32) -> u32 {
                        0
                    }
                    fn free(&mut self, _: u32) {}
                    fn ui_route_cleared(&mut self, _: u16) {
                        self.ui = true
                    }
                }
                let mut ship = read(m, ptr, 0x180);
                let had_route = ship[0x24..0x28] != [0; 4];
                let mut effects = Routes { ui: false };
                crate::ships::route::clear_route(&mut ship, index, self.player, self.ui_present, &mut effects);
                write(m, ptr, &ship);
                if had_route {
                    self.cleared_routes.push(index)
                }
                if effects.ui {
                    self.ui_invalidations.push(index)
                }
                Ok(0)
            }
            Effect::ShipRelease(index) => {
                let base = m.r32(0x6dd7a4);
                let count = m.r16(0x6dd894);
                if index >= count {
                    return Err("barco fuera del orden B5".into());
                }
                let mut ships = read(m, base, usize::from(count) * 0x180);
                let sea_head = m.r16(0x6dd88a);
                crate::ships::order::XOrder {
                    ships: &mut ships,
                    count,
                    sea_head,
                }
                .remove(index);
                write(m, base, &ships);
                Ok(0)
            }
            Effect::CreateConvoy(ship) => {
                super::b5_convoy_growth::grow(m)?;
                let index = m.r16(0x6dd884);
                let cap = m.r16(0x6dd896);
                let count = m.r16(0x6dd894);
                if index >= cap {
                    return Err("crecimiento de convoy B5 pendiente".into());
                }
                if ship >= count {
                    return Err("barco de convoy B5 inexistente".into());
                }
                // 514D7A inicializa [esp+0xc] a FFFF antes de push edi; la
                // dirección después del push es [esp+0x10], que recibe 5062F0.
                let scratch = 0xffff;
                let cvbase = m.r32(0x6dd7a8);
                let shipbase = m.r32(0x6dd7a4);
                let at = cvbase + u32::from(index) * 0x3c;
                let next = m.r16(at + 8);
                m.w16(0x6dd884, next);
                m.w16(at + 8, scratch);
                let src = shipbase + u32::from(ship) * 0x180;
                let owner = m.r8(src);
                let kind = m.r8(src + 0xe);
                let town = m.r8(src + 0x39);
                let origin = m.r8(src + 0x37);
                let dest = m.r8(src + 0x38);
                let capacity = m.r32(src + 0x10);
                let flags = m.r32(at + 0x18);
                m.w16(at + 0xa, ship);
                m.w8(at, owner);
                m.w8(at + 1, kind);
                m.w16(at + 0x1c, u16::from(town));
                m.w8(at + 0x1e, origin);
                m.w8(at + 0x1f, dest);
                m.w32(at + 4, capacity);
                m.w32(at + 0x18, flags | 0x400);
                m.w16(src + 8, index);
                let mut manager = read(m, 0x6dd7a0, 0x110);
                let mut ships = read(m, shipbase, usize::from(count) * 0x180);
                let mut convoys = read(m, cvbase, usize::from(cap) * 0x3c);
                let (mut captains, mut merchants, mut towns) = (Vec::new(), Vec::new(), Vec::new());
                let mut remove = crate::ships::remove::RemoveWorld {
                    manager: &mut manager,
                    ships: &mut ships,
                    captains: &mut captains,
                    convoys: &mut convoys,
                    merchants: &mut merchants,
                    towns: &mut towns,
                    pool_limit: 0,
                    merchant_count: 0,
                    town_count: 0,
                    ui_a: false,
                    ui_b: false,
                };
                let mut world = crate::ships::tick::TickWorld {
                    remove: &mut remove,
                    time: 0,
                    player: self.player,
                };
                crate::ships::ai_depart::init_convoy(&mut world, index, ship, 2)
                    .map_err(|e| format!("inicio de convoy B5: {e:?}"))?;
                let state = u16::from_le_bytes([
                    ships[usize::from(ship) * 0x180 + 0x134],
                    ships[usize::from(ship) * 0x180 + 0x135],
                ]);
                let at = usize::from(index) * 0x3c;
                convoys[at + 0x16..at + 0x18].copy_from_slice(&0x100u16.to_le_bytes());
                convoys[at + 0x12..at + 0x14].copy_from_slice(&state.to_le_bytes());
                write(m, shipbase, &ships);
                write(m, cvbase, &convoys);
                Ok(u32::from(index))
            }
            Effect::ConvoyDone(raw) => {
                let index = raw as u16;
                let cvcount = m.r16(0x6dd896);
                if index >= cvcount {
                    return Err("convoy B5 inexistente".into());
                }
                let cvbase = m.r32(0x6dd7a8);
                let shipbase = m.r32(0x6dd7a4);
                let count = m.r16(0x6dd894);
                let at = cvbase + u32::from(index) * 0x3c;
                let leader = m.r16(at + 0x10);
                if leader >= count {
                    return Err("elección de líder B5 pendiente".into());
                }
                let sea_head = m.r16(0x6dd88a);
                if leader == sea_head {
                    return Err("líder B5 coincide con cabeza marítima".into());
                }
                let head = m.r16(0x6dd88c);
                m.w16(at + 8, head);
                m.w16(0x6dd88c, index);
                let mut ships = read(m, shipbase, usize::from(count) * 0x180);
                crate::ships::order::XOrder {
                    ships: &mut ships,
                    count,
                    sea_head,
                }
                .insert(leader, 0xffff);
                write(m, shipbase, &ships);
                Ok(0)
            }
            Effect::ConvoyClear(ptr) => {
                let cvbase = m.r32(0x6dd7a8);
                let offset = ptr.checked_sub(cvbase).ok_or("puntero de convoy B5 inválido")?;
                if offset % 0x3c != 0 {
                    return Err("convoy B5 desalineado".into());
                }
                let index = offset / 0x3c;
                let cvcount = m.r16(0x6dd896);
                if index >= u32::from(cvcount) {
                    return Err("convoy B5 fuera de rango".into());
                }
                let shipbase = m.r32(0x6dd7a4);
                let shipcount = m.r16(0x6dd894);
                let mut manager = read(m, 0x6dd7a0, 0x110);
                let mut ships = read(m, shipbase, usize::from(shipcount) * 0x180);
                let mut convoys = read(m, cvbase, usize::from(cvcount) * 0x3c);
                let (mut captains, mut merchants, mut towns) = (Vec::new(), Vec::new(), Vec::new());
                let mut remove = crate::ships::remove::RemoveWorld {
                    manager: &mut manager,
                    ships: &mut ships,
                    captains: &mut captains,
                    convoys: &mut convoys,
                    merchants: &mut merchants,
                    towns: &mut towns,
                    pool_limit: 0,
                    merchant_count: 0,
                    town_count: 0,
                    ui_a: false,
                    ui_b: false,
                };
                let mut world = crate::ships::tick::TickWorld {
                    remove: &mut remove,
                    time: 0,
                    player: self.player,
                };
                crate::ships::convoy::refresh_from_ships(&mut world, index as u16)
                    .map_err(|_| "recuento inválido de convoy B5")?;
                write(m, shipbase, &ships);
                write(m, cvbase, &convoys);
                Ok(0)
            }
            Effect::TraderCleanup(field) => {
                let base = m.r32(0x6dd7a0);
                let capacity = m.r16(0x6dd892);
                let free_head = m.r16(0x6dd880);
                let mut pool = crate::ships::captains::CaptainPool {
                    entries: read(m, base, usize::from(capacity) * 16),
                    capacity,
                    free_head,
                };
                let mut slot = m.r16(field);
                pool.free(&mut slot);
                write(m, base, &pool.entries);
                m.w16(0x6dd880, pool.free_head);
                m.w16(field, slot);
                Ok(0)
            }
            other => Err(format!("callback B5 aún no conectado: {other:?}")),
        }
    }
}
