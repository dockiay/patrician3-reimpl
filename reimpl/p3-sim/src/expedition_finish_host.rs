//! Dependencias nativas transaccionales de `ships::expedition_finish` 502B20.
//!
//! `prepare` del núcleo clona este host: cartas, rutas y planificador solo se
//! publican con la memoria devuelta. Ninguna dependencia desconocida se omite.
use p3_core::{
    ai::reputation::Merchant,
    calendar::Calendar,
    flatmem::Mem,
    route::finder::{RouteOut, RoutePlanner},
    ships::{
        captains::CaptainPool,
        convoy,
        expedition_finish::{Effect, FinishHost},
        order,
        remove::RemoveWorld,
        tick::{ConvoyHost, TickWorld},
    },
};
use p3_save::ships::SavedRoute;
const M: u32 = 0x6dd7a0;
fn read(m: &mut dyn Mem, p: u32, n: usize) -> Vec<u8> {
    (0..n).map(|i| m.r8(p + i as u32)).collect()
}
fn write(m: &mut dyn Mem, p: u32, v: &[u8]) {
    for (i, &b) in v.iter().enumerate() {
        m.w8(p + i as u32, b)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Notification {
    Lost { owner: u8, town: u8, count: u32 },
    Returned { owner: u8, ship: u16 },
}
#[derive(Clone)]
pub struct NativeFinishHost {
    pub message_pool: Vec<u8>,
    pub calendar: Calendar,
    pub player: u32,
    pub pool_limit: u16,
    pub ports: Vec<(i16, i16)>,
    pub planner: Option<RoutePlanner>,
    pub routes: Vec<Option<SavedRoute>>,
    pub route_points: Vec<Option<Vec<(i16, i16)>>>,
    pub notifications: Vec<Notification>,
}
#[derive(Default)]
struct RejectHooks {
    events: Vec<String>,
}
impl ConvoyHost for RejectHooks {
    fn hook(&mut self, name: &'static str, _: &[u32]) -> u32 {
        self.events.push(name.into());
        0
    }
    fn read(&mut self, _: u32, _: u8) -> u32 {
        self.events.push("lectura de memoria no conectada".into());
        0
    }
}

/// Construye un mundo temporal para los portes existentes, y solo copia la
/// salida si el callback termina. `m` ya pertenece a la transacción exterior.
fn world<F>(m: &mut dyn Mem, player: u32, limit: u16, f: F) -> Result<(), String>
where
    F: FnOnce(&mut TickWorld) -> Result<(), String>,
{
    let mut manager = read(m, M, 0x110);
    let sb = m.r32(M + 4);
    let cb = m.r32(M + 8);
    let pb = m.r32(M);
    let mb = m.r32(0x6de518);
    let tb = m.r32(0x6de508);
    let sc = m.r16(M + 0xf4);
    let cc = m.r16(M + 0xf6);
    let pc = m.r16(M + 0xf2);
    let mc = m.r16(0x6de4aa);
    let tc = m.r32(0x6de4b0);
    if tc > 40 {
        return Err("ciudades de expedición inválidas".into());
    }
    let mut ships = read(m, sb, usize::from(sc) * 0x180);
    let mut convoys = read(m, cb, usize::from(cc) * 0x3c);
    let mut captains = read(m, pb, usize::from(pc) * 16);
    let mut merchants = read(m, mb, usize::from(mc) * 0x650);
    let mut towns = read(m, tb, tc as usize * 0x9f8);
    let tick = m.r32(0x6de4b4);
    let mut remove = RemoveWorld {
        manager: &mut manager,
        ships: &mut ships,
        captains: &mut captains,
        convoys: &mut convoys,
        merchants: &mut merchants,
        towns: &mut towns,
        pool_limit: limit,
        merchant_count: mc,
        town_count: tc,
        ui_a: false,
        ui_b: false,
    };
    let mut w = TickWorld {
        remove: &mut remove,
        time: tick,
        player,
    };
    f(&mut w)?;
    write(m, M, &manager);
    write(m, sb, &ships);
    write(m, cb, &convoys);
    write(m, pb, &captains);
    write(m, mb, &merchants);
    write(m, tb, &towns);
    Ok(())
}
fn cv_index(m: &mut dyn Mem, p: u32) -> Result<u16, String> {
    let base = m.r32(M + 8);
    let offset = p.checked_sub(base).ok_or("puntero de convoy inválido")?;
    if offset % 0x3c != 0 || offset / 0x3c >= u32::from(m.r16(M + 0xf6)) {
        return Err("convoy fuera del pool".into());
    }
    Ok((offset / 0x3c) as u16)
}
fn check_chain(
    m: &mut dyn Mem,
    mut current: u16,
    base: u32,
    count: u16,
    stride: u32,
    offset: u32,
) -> Result<(), String> {
    let mut seen = vec![false; usize::from(count)];
    while current < count {
        let i = usize::from(current);
        if seen[i] {
            return Err("lista libre o activa cíclica".into());
        }
        seen[i] = true;
        current = m.r16(base + u32::from(current) * stride + offset)
    }
    Ok(())
}
impl FinishHost for NativeFinishHost {
    fn call(&mut self, m: &mut dyn Mem, e: Effect) -> Result<(), String> {
        match e {
            Effect::LostLetter { owner, record } => {
                let n = m.r16(0x6de4aa);
                if u16::from(owner) >= n {
                    return Err("destinatario de expedición ausente".into());
                }
                let p = m.r32(0x6de518) + u32::from(owner) * 0x650;
                let mut mer = Merchant {
                    bytes: read(m, p, 0x650),
                };
                let c = self.calendar;
                p3_state::messages::add_expedition_lost(
                    &mut self.message_pool,
                    &mut mer,
                    c.day,
                    c.month,
                    c.year,
                    &record,
                )?;
                write(m, p, &mer.bytes);
                Ok(())
            }
            Effect::LostWindow { owner, town, count } => {
                self.notifications.push(Notification::Lost { owner, town, count });
                Ok(())
            }
            Effect::ReturnWindow { owner, ship } => {
                self.notifications.push(Notification::Returned { owner, ship });
                Ok(())
            }
            Effect::FreeCaptain { index } => {
                let count = m.r16(M + 0xf2);
                if index >= count {
                    return Err("capitán fuera del pool".into());
                }
                let base = m.r32(M);
                let free = m.r16(M + 0xe0);
                check_chain(m, free, base, count, 16, 0)?;
                let mut pool = CaptainPool {
                    entries: read(m, base, usize::from(count) * 16),
                    capacity: count,
                    free_head: free,
                };
                let mut slot = index;
                pool.free(&mut slot);
                write(m, base, &pool.entries);
                m.w16(M + 0xe0, pool.free_head);
                Ok(())
            }
            Effect::ReleaseConvoy { index } => {
                let count = m.r16(M + 0xf6);
                if index >= count {
                    return Err("baja de convoy fuera del pool".into());
                }
                let base = m.r32(M + 8);
                let cv = base + u32::from(index) * 0x3c;
                if m.r16(cv + 0xa) < m.r16(M + 0xf4) {
                    return Err("baja de convoy con miembros".into());
                }
                let active = m.r16(M + 0xec);
                let free = m.r16(M + 0xe4);
                check_chain(m, active, base, count, 0x3c, 8)?;
                check_chain(m, free, base, count, 0x3c, 8)?;
                world(m, self.player, self.pool_limit, |w| {
                    let mut h = RejectHooks::default();
                    convoy::release_convoy(w, index, &mut h);
                    if !h.events.is_empty() {
                        return Err(format!("efectos de baja no conectados: {:?}", h.events));
                    }
                    Ok(())
                })
            }
            Effect::PickLeader { convoy: ptr } => {
                let index = cv_index(m, ptr)?;
                if m.r16(ptr + 0x10) < m.r16(M + 0xf4) {
                    return Err("elección con líder anterior inesperado".into());
                }
                world(m, self.player, self.pool_limit, |w| {
                    let mut h = RejectHooks::default();
                    convoy::pick_main(w, index, &mut h);
                    if !h.events.is_empty() {
                        return Err(format!("efectos de elección no conectados: {:?}", h.events));
                    }
                    Ok(())
                })
            }
            Effect::RefreshConvoy { convoy: ptr } => {
                let index = cv_index(m, ptr)?;
                world(m, self.player, self.pool_limit, |w| {
                    convoy::refresh_from_ships(w, index).map_err(|e| format!("recuento de expedición: {e:?}"))
                })
            }
            Effect::InsertOrder { ship } => world(m, self.player, self.pool_limit, |w| {
                if ship >= u16::from_le_bytes([w.remove.manager[0xf4], w.remove.manager[0xf5]]) {
                    return Err("barco de orden inexistente".into());
                }
                order::insert_from_tick(w, ship, 0xffff);
                Ok(())
            }),
            Effect::PlanRoute { ship, destination } => {
                let count = m.r16(M + 0xf4);
                if ship >= count {
                    return Err("barco de ruta inexistente".into());
                }
                let pos = m.r32(M + 4) + u32::from(ship) * 0x180;
                let mut image = read(m, pos, 0x180);
                if image[0x24..0x28] != [0; 4] {
                    return Err("ruta anterior de expedición no liberada".into());
                }
                let start = (
                    u16::from_le_bytes([image[0x1e], image[0x1f]]) as i16,
                    u16::from_le_bytes([image[0x22], image[0x23]]) as i16,
                );
                let end = *self
                    .ports
                    .get(usize::from(destination))
                    .ok_or("puerto de expedición ausente")?;
                let planner = self.planner.as_mut().ok_or("planificador de expedición ausente")?;
                let found = planner.find_route(&[start, end]);
                let (points, saved) = match found.route {
                    RouteOut::Built(p) => {
                        if found.status == 0 {
                            return Err("ruta propia de expedición sin propiedad acreditada".into());
                        }
                        let saved = SavedRoute::Owned(p.clone());
                        (p, saved)
                    }
                    RouteOut::Shared(id) => {
                        if found.status != 0 {
                            return Err("ruta compartida con propiedad incompatible".into());
                        }
                        let points = planner.routes.get(id).cloned().ok_or("ruta compartida ausente")?;
                        let origin = image[0x37];
                        if self.ports.get(usize::from(origin)) != Some(&start) {
                            return Err("origen compartido de expedición no acreditado".into());
                        }
                        (points, SavedRoute::Shared { origin, destination })
                    }
                    RouteOut::Unset => return Err("ruta de expedición no disponible".into()),
                };
                let tick = m.r32(0x6de4b4);
                let token = 0x10000 + u32::from(ship) * 16;
                if p3_core::ships::route::apply_route_plan(
                    &mut image,
                    Some((destination, end.0, end.1)),
                    end,
                    &points,
                    found.status != 0,
                    token,
                    tick,
                )
                .is_none()
                {
                    return Err("ruta de regreso inválida".into());
                }
                let slot = self
                    .routes
                    .get_mut(usize::from(ship))
                    .ok_or("tabla de rutas incompleta")?;
                let cache = self
                    .route_points
                    .get_mut(usize::from(ship))
                    .ok_or("caché de rutas incompleta")?;
                *slot = Some(saved);
                *cache = Some(points);
                write(m, pos, &image);
                Ok(())
            }
        }
    }
}
