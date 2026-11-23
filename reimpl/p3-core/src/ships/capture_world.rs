//! `op_capture_ship` 0x00540520: propietarios comerciales y, con anfitrión, NPC (piratas).
//! La actualización mundial ocurre de forma inmediata durante el abordaje.

const SHIP: usize = 0x180;
const MERCHANT: usize = 0x650;
const CONVOY: usize = 0x3c;
const CREW: [u8; 4] = [5, 8, 10, 12];
fn r16(b: &[u8], p: usize) -> u16 {
    u16::from_le_bytes([b[p], b[p + 1]])
}
fn w16(b: &mut [u8], p: usize, v: u16) {
    b[p..p + 2].copy_from_slice(&v.to_le_bytes())
}
fn r32(b: &[u8], p: usize) -> u32 {
    u32::from_le_bytes(b[p..p + 4].try_into().unwrap())
}
fn w32(b: &mut [u8], p: usize, v: u32) {
    b[p..p + 4].copy_from_slice(&v.to_le_bytes())
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WorldCaptureEvent {
    UiShipState(u16, u16),
    MapShipState(u16, u16),
}
/// Tablas coherentes del mundo; en esta etapa se rechazan capitanes,
/// misiones; el líder requiere la variante con gestor de navegación. Los errores no mutan el mundo.
pub struct WorldCapture<'a> {
    pub ships: &'a mut Vec<u8>,
    pub merchants: &'a mut Vec<u8>,
    pub convoys: &'a mut Vec<u8>,
    pub convoy_count: u16,
    pub town_count: u32,
    pub prestige: i16,
}

pub fn capture_commercial(
    world: &mut WorldCapture<'_>,
    captor: u32,
    target: u32,
) -> Result<Vec<WorldCaptureEvent>, &'static str> {
    capture_commercial_with_route(world, captor, target, None).map(|(events, _)| events)
}
/// `route_child` is the first word of an owned route allocation. The caller
/// supplies it instead of dereferencing a foreign x86 pointer. Returned
/// addresses identify the allocations the original frees, in order.
pub fn capture_commercial_with_route(
    world: &mut WorldCapture<'_>,
    captor: u32,
    target: u32,
    route_child: Option<u32>,
) -> Result<(Vec<WorldCaptureEvent>, Vec<u32>), &'static str> {
    capture_inner(world, captor, target, route_child, None, None)
}
/// Also executes `ships::free_captain` on the original captain pool. All
/// supplied tables and the pool remain unchanged on a rejected capture.
pub fn capture_commercial_with_captains(
    world: &mut WorldCapture<'_>,
    captor: u32,
    target: u32,
    route_child: Option<u32>,
    pool: &mut super::captains::CaptainPool,
) -> Result<(Vec<WorldCaptureEvent>, Vec<u32>), &'static str> {
    let mut staged = pool.clone();
    let result = capture_inner(world, captor, target, route_child, Some(&mut staged), None)?;
    *pool = staged;
    Ok(result)
}
/// Variant with a live navigation manager, including leader replacement.
pub fn capture_commercial_with_leaders(
    world: &mut WorldCapture<'_>,
    captor: u32,
    target: u32,
    route_child: Option<u32>,
    pool: &mut super::captains::CaptainPool,
    manager: &mut Vec<u8>,
    pool_limit: u16,
) -> Result<(Vec<WorldCaptureEvent>, Vec<u32>), &'static str> {
    let mut staged_pool = pool.clone();
    let mut staged_manager = manager.clone();
    let result = capture_inner(
        world,
        captor,
        target,
        route_child,
        Some(&mut staged_pool),
        Some((&mut staged_manager, pool_limit)),
    )?;
    *pool = staged_pool;
    *manager = staged_manager;
    Ok(result)
}
/// Imágenes del mundo en la llamada original a 0x00515F70, dentro de la operación.
pub struct NpcCaptureWorld<'a> {
    pub ships: &'a mut Vec<u8>,
    pub merchants: &'a mut Vec<u8>,
    pub convoys: &'a mut Vec<u8>,
    pub captains: &'a mut super::captains::CaptainPool,
    pub manager: &'a mut Vec<u8>,
}

/// Efectos de `op_capture_ship` 0x00540520 cuando interviene un propietario NPC
/// (byte de dueño `>= 0x006DE4AA`). Las cartas y la tarea quedan en el anfitrión.
pub trait NpcCaptureHost {
    /// Cinco punteros `0x006DD7AC[i]` no nulos.
    fn band_objects(&self) -> [bool; 5];
    /// `remove_ship_from_pirate_groups` 0x005066F0 (0x00515F00 en cada banda) sobre la imagen actual.
    fn remove_from_bands(&mut self, ships: &mut [u8], target: u16) -> Result<(), String>;
    /// 0x004D8CF0 en `tick+0x300`, tipo 5: `+8` patrocinador, `+9/+0xB` su ciudad, `+0xA` captor,
    /// `+0xC` instante, `+0x10=4`, `+0x11=0`, `+0x16` palabra `+0x15E` del barco.
    fn sponsor_task(&mut self, sponsor: u8, home: u8, captor: u8, word: u16) -> Result<(), String>;
    /// 0x00515F70 sobre la banda `index`, con el barco ya enlazado en `captor+4`.
    fn register_capture(&mut self, index: u8, world: NpcCaptureWorld<'_>) -> Result<(), String>;
    /// 0x004D6530 con un destinatario comerciante; los 16 primeros bytes de la carta.
    fn letter(&mut self, recipient: u8, record: [u8; 16]);
    /// `scheduled_tasks_alderman_mission_report_captured_ship` 0x004EC864(objetivo, dueño captor),
    /// llamada con el objetivo aún en estado 0x12; `ships`/`captains` son las imágenes en ese punto.
    fn alderman_report(&mut self, _target: u16, _captor: u8, _ships: &[u8], _captains: Option<&super::captains::CaptainPool>) -> Result<(), String> {
        Err("informe del regidor pendiente".into())
    }
    /// Recursos de `Captain::init` 0x004FDF50 para un capitán nuevo del cambio de líder.
    fn captain_resources(&self) -> Option<crate::economy::pirate_group_callbacks::CaptainResources> {
        None
    }
    /// `convoy_task_is_active_pirate_band_for` 0x00503B00.
    fn hanse_expedition(&mut self, _convoy: u16) -> bool {
        false
    }
    /// Tarea 0x34 del convoy de la caza (`scheduled_tasks_get_task` 0x4D8DB0 con `convoy+2`).
    fn hunt_task(&mut self, _convoy: u16) -> Option<&mut crate::scheduler::TaskRecord> {
        None
    }
    /// Nodo de bajas (`operator new(8)` + 0x4F3980 + 0x4F3AD0) antepuesto a `tarea+8` de la caza.
    fn hunt_loss(&mut self, _convoy: u16, _loss: super::return_convoy::HuntLoss) -> Result<(), String> {
        Err("pérdida de expedición hanseática pendiente".into())
    }
}

/// Byte 0x00672855: clase del nodo de un barco de la caza capturado.
pub const HUNT_CAPTURED_CLASS: u8 = 1;
/// Byte 0x00672857: clase del nodo de un barco capturado por la caza.
pub const HUNT_CAPTOR_CLASS: u8 = 3;
/// Byte 0x0067285A: `tarea+0xE` cuando la caza captura al capitán que busca.
pub const HUNT_FOUND_MARK: u8 = 1;

/// Captura completa: comerciantes y NPC. Sin anfitrión NPC la variante NPC se rechaza
/// igual que antes. Todas las tablas permanecen intactas ante un error.
#[allow(clippy::too_many_arguments)]
pub fn capture_world_with_npc(
    world: &mut WorldCapture<'_>,
    captor: u32,
    target: u32,
    route_child: Option<u32>,
    pool: &mut super::captains::CaptainPool,
    manager: &mut Vec<u8>,
    pool_limit: u16,
    npc: &mut dyn NpcCaptureHost,
) -> Result<(Vec<WorldCaptureEvent>, Vec<u32>), &'static str> {
    let mut staged_pool = pool.clone();
    let mut staged_manager = manager.clone();
    let result = capture_inner_npc(
        world,
        captor,
        target,
        route_child,
        Some(&mut staged_pool),
        Some((&mut staged_manager, pool_limit)),
        Some(npc),
    )?;
    *pool = staged_pool;
    *manager = staged_manager;
    Ok(result)
}

fn capture_inner(
    world: &mut WorldCapture<'_>,
    captor: u32,
    target: u32,
    route_child: Option<u32>,
    pool: Option<&mut super::captains::CaptainPool>,
    manager: Option<(&mut Vec<u8>, u16)>,
) -> Result<(Vec<WorldCaptureEvent>, Vec<u32>), &'static str> {
    capture_inner_npc(world, captor, target, route_child, pool, manager, None)
}

/// `clear_route` 0x00516840 tal como lo observa la captura: libera la ruta propia.
fn clear_route(ships: &mut [u8], bo: usize, route_child: Option<u32>, freed: &mut Vec<u32>) -> Result<(), &'static str> {
    let route = r32(ships, bo + 0x24);
    if route != 0 {
        if ships[bo + 0x36] != 0 {
            let child = route_child.ok_or("descriptor de ruta propia ausente")?;
            if child != 0 {
                freed.push(child)
            }
            freed.push(route);
            ships[bo + 0x36] = 0
        }
        w32(ships, bo + 0x24, 0)
    }
    w16(ships, bo + 0x34, 0);
    w16(ships, bo + 2, 0);
    Ok(())
}

/// Suma con saturación de `merchant+0x16` (0x5409xx / 0x540Cxx).
fn add_prestige(merchants: &mut [u8], at: usize, delta: i16) {
    let current = r16(merchants, at + 0x16) as i16;
    w16(merchants, at + 0x16, current.saturating_add(delta) as u16);
}

/// Valor de botín del patrón pirata (0x540C53..0x540CC5): casco más carga, 65 % con tope 60000.
fn pirate_reward(ship: &[u8]) -> Result<i32, &'static str> {
    let value = crate::ai::asset_values::ship_current_value(ship[0xe], r32(ship, 0x14) as i32, r32(ship, 0x18) as i32)?;
    let mut sum = value as i32;
    for (ware, price) in crate::economy::prices::BASE_PRICE.iter().enumerate() {
        let quantity = r32(ship, 0x54 + 4 * ware) as i32;
        sum = sum.wrapping_add(crate::numeric::ftol_i32(f64::from(quantity as f32) * f64::from(*price)));
    }
    Ok((sum.wrapping_mul(65) / 100).min(60000))
}

fn capture_inner_npc(
    world: &mut WorldCapture<'_>,
    captor: u32,
    target: u32,
    route_child: Option<u32>,
    mut pool: Option<&mut super::captains::CaptainPool>,
    mut manager: Option<(&mut Vec<u8>, u16)>,
    mut npc: Option<&mut dyn NpcCaptureHost>,
) -> Result<(Vec<WorldCaptureEvent>, Vec<u32>), &'static str> {
    if world.ships.len() % SHIP != 0
        || world.merchants.len() % MERCHANT != 0
        || world.convoys.len() % CONVOY != 0
        || usize::from(world.convoy_count) > world.convoys.len() / CONVOY
    {
        return Err("tablas de captura truncadas");
    }
    let count = world.ships.len() / SHIP;
    let (a, b) = (captor as usize, target as usize);
    if a >= count || b >= count {
        return Ok((Vec::new(), Vec::new()));
    }
    if a == b {
        return Err("captura del propio barco no contrastada");
    }
    let mut ships = world.ships.clone();
    let mut merchants = world.merchants.clone();
    let mut convoys = world.convoys.clone();
    let mut convoy_count = world.convoy_count;
    let mut events = Vec::new();
    let mut freed = Vec::new();
    let (ao, bo) = (a * SHIP, b * SHIP);
    let (own_a, own_b) = (ships[ao], ships[bo]);
    let (kind_a, kind_b) = (usize::from(ships[ao + 0xe]), usize::from(ships[bo + 0xe]));
    if kind_a >= CREW.len() || kind_b >= CREW.len() {
        return Err("tipo de casco desconocido");
    }
    if own_a != own_b {
        let mc = merchants.len() / MERCHANT;
        let (npc_a, npc_b) = (usize::from(own_a) >= mc, usize::from(own_b) >= mc);
        if (npc_a || npc_b) && npc.is_none() {
            return Err("captura NPC pendiente");
        }
        if r16(&ships, bo + 0x42) != u16::MAX && pool.is_none() {
            return Err("pool de capitanes ausente");
        }
        let capacity = pool.as_ref().map_or(0, |p| p.capacity);
        if let Some(p) = pool.as_ref() {
            if p.entries.len() != usize::from(p.capacity) * 16 {
                return Err("pool de capitanes truncado");
            }
            if r16(&ships, bo + 0x42) < p.capacity {
                let mut node = p.free_head;
                let mut seen = vec![false; usize::from(p.capacity)];
                while node < p.capacity {
                    if node == r16(&ships, bo + 0x42) {
                        break;
                    }
                    let i = usize::from(node);
                    if seen[i] {
                        return Err("lista libre de capitanes cíclica");
                    }
                    seen[i] = true;
                    node = r16(&p.entries, i * 16);
                }
            }
        }
        if r16(&ships, bo + 0xa) != u16::MAX || r16(&ships, bo + 0xc) != u16::MAX {
            return Err("captura con índice mundial pendiente");
        }
        if world.town_count > 40 {
            return Err("ciudades de reputación excesivas");
        }
        let sponsor = ships[bo + 0x15d];
        if usize::from(sponsor) < mc {
            if npc_b && !npc_a && own_a != sponsor {
                let home = merchants[usize::from(sponsor) * MERCHANT + 0x19];
                let word = r16(&ships, bo + 0x15e);
                npc.as_deref_mut()
                    .unwrap()
                    .sponsor_task(sponsor, home, own_a, word)
                    .map_err(|_| "tarea de patrocinio pirata fallida")?;
            }
            ships[bo + 0x15d] = 255
        }
        if r16(&ships, bo + 0x134) == 0x12 {
            let host = npc.as_deref_mut().ok_or("captura con misión pendiente")?;
            host.alderman_report(target as u16, own_a, &ships, pool.as_deref())
                .map_err(|_| "informe del regidor fallido")?;
        }
        let state = r16(&ships, bo + 0x134);
        events.push(WorldCaptureEvent::UiShipState(target as u16, state));
        events.push(WorldCaptureEvent::MapShipState(target as u16, state));
        let flags = r16(&ships, bo + 0x3c) | 0x400;
        w16(&mut ships, bo + 0x3c, flags);
        if !npc_b {
            let old = usize::from(own_b) * MERCHANT;
            clear_route(&mut ships, bo, route_child, &mut freed)?;
            let mut node = usize::from(r16(&merchants, old + 0xe));
            let mut previous = None;
            let mut seen = vec![false; count];
            while node != b {
                if node >= count {
                    w32(&mut ships, bo + 0x18, 0);
                    *world.ships = ships;
                    return Ok((events, freed));
                }
                if seen[node] {
                    return Err("flota comercial cíclica");
                }
                seen[node] = true;
                previous = Some(node);
                node = usize::from(r16(&ships, node * SHIP + 4));
            }
            let next = r16(&ships, bo + 4);
            if let Some(p) = previous {
                w16(&mut ships, p * SHIP + 4, next)
            } else {
                w16(&mut merchants, old + 0xe, next)
            }
            let v = r32(&merchants, old + 0x47c).saturating_sub(1);
            w32(&mut merchants, old + 0x47c, v);
            let v = r16(&merchants, old + 0x478).saturating_sub(1);
            w16(&mut merchants, old + 0x478, v);
            merchants[old + 0x1b] = merchants[old + 0x1b].saturating_sub(1);
        } else {
            npc.as_deref_mut()
                .unwrap()
                .remove_from_bands(&mut ships, target as u16)
                .map_err(|_| "desenlace de bandas piratas fallido")?;
        }
        let old_cv = r16(&ships, bo + 8);
        if old_cv < convoy_count {
            if npc.as_deref_mut().is_some_and(|h| h.hanse_expedition(old_cv)) {
                let host = npc.as_deref_mut().unwrap();
                // 0x540836: el barco era de la caza; nodo {dueño perdido, 0x672855, +0x15E}.
                host.hunt_task(old_cv).ok_or("tarea de expedición hanseática ausente")?;
                let loss = super::return_convoy::HuntLoss { mark: own_b, class: HUNT_CAPTURED_CLASS, text: r16(&ships, bo + 0x15e) };
                host.hunt_loss(old_cv, loss).map_err(|_| "pérdida de expedición hanseática pendiente")?;
            }
            let co = usize::from(old_cv) * CONVOY;
            let leader = usize::from(r16(&convoys, co + 0x10)) == b;
            let mut node = usize::from(r16(&convoys, co + 0xa));
            let mut previous = None;
            let mut seen = vec![false; count];
            while node < count {
                if seen[node] {
                    return Err("convoy cíclico");
                }
                seen[node] = true;
                let next = r16(&ships, node * SHIP + 6);
                if node == b {
                    if let Some(p) = previous {
                        w16(&mut ships, p * SHIP + 6, next)
                    } else {
                        w16(&mut convoys, co + 0xa, next)
                    }
                    node = usize::from(next)
                } else {
                    previous = Some(node);
                    node = usize::from(next)
                }
            }
            if leader {
                let (mgr, limit) = manager.as_mut().ok_or("gestor de líder ausente")?;
                let pool = pool.as_deref_mut().ok_or("pool de capitanes ausente")?;
                if mgr.len() < 0xf8
                    || usize::from(r16(mgr, 0xf4)) != count
                    || r16(mgr, 0xf6) != convoy_count
                    || r16(mgr, 0xf2) != pool.capacity
                {
                    return Err("gestor de líder incoherente");
                }
                let mut towns = Vec::new();
                let mut hooks = super::convoy::LeaderHooks::new(npc.as_deref().and_then(|h| h.captain_resources()), false);
                let mut remove = super::remove::RemoveWorld {
                    manager: mgr,
                    ships: &mut ships,
                    captains: &mut pool.entries,
                    convoys: &mut convoys,
                    merchants: &mut merchants,
                    towns: &mut towns,
                    pool_limit: *limit,
                    merchant_count: mc as u16,
                    town_count: world.town_count,
                    ui_a: false,
                    ui_b: false,
                };
                let mut tick = super::tick::TickWorld {
                    remove: &mut remove,
                    time: 0,
                    player: u32::MAX,
                };
                super::convoy::pick_main(&mut tick, old_cv, &mut hooks);
                super::convoy::refresh_from_ships(&mut tick, old_cv).map_err(|_| "recuento de convoy inválido")?;
                if hooks.unsupported {
                    return Err("efecto de líder pendiente");
                }
                pool.free_head = r16(mgr, 0xe0);
            }
        }
        if !npc_a {
            let new = usize::from(own_a) * MERCHANT;
            let head = r16(&merchants, new + 0xe);
            w16(&mut ships, bo + 4, head);
            w16(&mut merchants, new + 0xe, target as u16);
            let n = r16(&merchants, new + 0x478).wrapping_add(1);
            w16(&mut merchants, new + 0x478, n);
            let n = r32(&merchants, new + 0x47c).wrapping_add(1);
            w32(&mut merchants, new + 0x47c, n);
            if !npc_b {
                add_prestige(&mut merchants, new, world.prestige);
            } else {
                // `merchant_reward_pirate_ship_defeated` 0x004F8D90 con el i16 0x00672DDE.
                let mut merchant = crate::ai::reputation::Merchant { bytes: merchants[new..new + MERCHANT].to_vec() };
                let env = crate::ai::reputation::RepEnv {
                    town_count: world.town_count,
                    towns: &[],
                    renown: [0, 0, -512],
                    byte_52c: 0,
                };
                merchant.reward_pirate_ship_defeated(&env, u32::from(r16(&ships, bo + 0x42) < capacity));
                merchants[new..new + MERCHANT].copy_from_slice(&merchant.bytes);
            }
            if merchants[new + 0x1b] < 250 {
                merchants[new + 0x1b] += 1
            }
            w16(&mut ships, bo + 0x134, 0x14);
            // [esp+0x13]/[esp+0x12] de 0x540520: bytes +2/+3 del último capitán con carta 0x54.
            let mut look = (0xffu8, 0xffu8);
            if let Some(p) = pool.as_deref_mut() {
                let mut slot = r16(&ships, bo + 0x42);
                while slot < p.capacity {
                    if npc_b {
                        let at = usize::from(slot) * 16;
                        let mut record = [0u8; 16];
                        record[1] = 0x80;
                        record[2..6].fill(0xff);
                        record[4] = 0x54;
                        record[5] = 0;
                        look = (p.entries[at + 2], p.entries[at + 3]);
                        let word = (u16::from(look.0) << 8).wrapping_add(u16::from(look.1));
                        record[8..10].copy_from_slice(&word.to_le_bytes());
                        record[10..12].copy_from_slice(&slot.to_le_bytes());
                        record[12..14].copy_from_slice(&(target as u16).to_le_bytes());
                        npc.as_deref_mut().unwrap().letter(own_a, record);
                    }
                    p.free(&mut slot);
                }
                w16(&mut ships, bo + 0x42, slot);
                if let Some((mgr, _)) = manager.as_mut() {
                    if mgr.len() < 0xe2 {
                        return Err("gestor de capitanes truncado");
                    }
                    w16(mgr, 0xe0, p.free_head);
                }
            }
            let x = r32(&ships, ao + 0x1c);
            let y = r32(&ships, ao + 0x20);
            w32(&mut ships, bo + 0x1c, x);
            w32(&mut ships, bo + 0x20, y);
            clear_route(&mut ships, bo, route_child, &mut freed)?;
            let new_cv = r16(&ships, ao + 8);
            let city = if new_cv < convoy_count {
                if npc.as_deref_mut().is_some_and(|h| h.hanse_expedition(new_cv)) {
                let host = npc.as_deref_mut().unwrap();
                    // 0x540B00: la caza captura; nodo {captor, 0x672857, +0x15E} y, si el capitán
                    // es el buscado (`tarea+0xC/+0xD`), `tarea+0xE` = 0x67285A.
                    let task = host.hunt_task(new_cv).ok_or("tarea de expedición hanseática ausente")?;
                    let found = task.u8_at(0xd) == look.1 && task.u8_at(0xc) == look.0;
                    let mut edited = *task;
                    if found {
                        edited.set_u8_at(0xe, HUNT_FOUND_MARK);
                    }
                    let loss = super::return_convoy::HuntLoss { mark: own_a, class: HUNT_CAPTOR_CLASS, text: r16(&ships, bo + 0x15e) };
                    host.hunt_loss(new_cv, loss).map_err(|_| "pérdida de expedición hanseática pendiente")?;
                    *host.hunt_task(new_cv).ok_or("tarea de expedición hanseática ausente")? = edited;
                }
                let co = usize::from(new_cv) * CONVOY;
                let c = convoys[co + 0x1f];
                if u32::from(c) >= world.town_count && convoys[co + 0x14] & 16 != 0 {
                    merchants[new + 0x19]
                } else {
                    c
                }
            } else {
                merchants[new + 0x19]
            };
            ships[bo + 0x38] = city;
        } else {
            let host = npc.as_deref_mut().unwrap();
            let cv = r16(&ships, ao + 8);
            if cv >= convoy_count {
                return Err("captor pirata sin convoy");
            }
            let leader = usize::from(r16(&convoys, usize::from(cv) * CONVOY + 0x10));
            if leader >= count {
                return Err("convoy pirata sin líder");
            }
            let lo = leader * SHIP;
            let patron = ships[lo + 0x15d];
            let captain = r16(&ships, lo + 0x42);
            if usize::from(patron) < mc && captain < capacity {
                let p = pool.as_deref().unwrap();
                let person = &p.entries[usize::from(captain) * 16..][..16];
                let reward = pirate_reward(&ships[bo..bo + SHIP])?;
                let at = usize::from(patron) * MERCHANT;
                let gold = r32(&merchants, at) as i32;
                w32(&mut merchants, at, gold.wrapping_add(reward) as u32);
                let income = r32(&merchants, at + 0x4b8) as i32;
                w32(&mut merchants, at + 0x4b8, income.wrapping_add(reward) as u32);
                add_prestige(&mut merchants, at, 65);
                if reward > 0 {
                    let mut record = [0u8; 16];
                    record[1] = 0x80;
                    record[2..6].fill(0xff);
                    record[4] = 0x30;
                    record[5] = merchants[at + 0x19];
                    record[8] = person[2];
                    record[9] = person[3];
                    if !npc_b {
                        let o = usize::from(own_b) * MERCHANT;
                        record[10] = merchants[o + 0x1c];
                        record[11] = merchants[o + 0x1d];
                    } else {
                        record[10] = (captor & 0x3f) as u8;
                        record[11] = (target & 0x3f) as u8;
                    }
                    record[12..14].copy_from_slice(&(reward as u16).to_le_bytes());
                    let word = (r16(&ships, lo + 0x15e) << 2).wrapping_add(u16::from(ships[bo + 0xe] & 3));
                    record[14..16].copy_from_slice(&word.to_le_bytes());
                    host.letter(patron, record);
                }
            }
            let link = r16(&ships, ao + 4);
            w16(&mut ships, bo + 4, link);
            w16(&mut ships, ao + 4, target as u16);
            let p = pool.as_deref_mut().ok_or("pool de capitanes ausente")?;
            let mut slot = r16(&ships, bo + 0x42);
            while slot < p.capacity {
                p.free(&mut slot);
            }
            w16(&mut ships, bo + 0x42, slot);
            let (mgr, _) = manager.as_mut().ok_or("gestor de capitanes ausente")?;
            if mgr.len() < 0xf8 {
                return Err("gestor de capitanes truncado");
            }
            w16(mgr, 0xe0, p.free_head);
            let band = super::convoy_plan::first_object(&host.band_objects().map(|v| v.then_some(0)), ships[ao + 0x158]);
            if band < 5 && host.band_objects()[usize::from(band)] {
                let before = convoys.len();
                host.register_capture(
                    band,
                    NpcCaptureWorld { ships: &mut ships, merchants: &mut merchants, convoys: &mut convoys, captains: p, manager: mgr },
                )
                .map_err(|_| "efecto de banda pirata fallido")?;
                if ships.len() % SHIP != 0 || ships.len() < count * SHIP || convoys.len() % CONVOY != 0 || convoys.len() < before {
                    return Err("resultado de banda inválido");
                }
                convoy_count = convoy_count.wrapping_add(((convoys.len() - before) / CONVOY) as u16);
            }
            w16(&mut ships, bo + 0x134, 0x12);
        }
        ships[bo] = own_a;
        ships[bo + 0x15c] = ships[ao + 0x15c];
        let flags = r16(&ships, bo + 0x3c) | 0x1420;
        w16(&mut ships, bo + 0x3c, flags);
        let new_cv = r16(&ships, ao + 8);
        if new_cv < convoy_count {
            let co = usize::from(new_cv) * CONVOY;
            w16(&mut ships, bo + 8, new_cv);
            let head = r16(&convoys, co + 0xa);
            w16(&mut ships, bo + 6, head);
            w16(&mut convoys, co + 0xa, target as u16)
        }
        ships[bo + 0x136] = 0;
    }
    let total = r16(&ships, ao + 0x40).wrapping_add(r16(&ships, bo + 0x40)).max(2);
    let ca = u32::from(CREW[kind_a]);
    let cb = u32::from(CREW[kind_b]);
    let crew = (u32::from(total) * cb / (ca + cb)).max(1).min(cb) as u16;
    w16(&mut ships, bo + 0x40, crew);
    w16(&mut ships, ao + 0x40, total - crew);
    if own_a != own_b {
        w16(&mut ships, bo + 0x3a, u16::MAX);
    }
    *world.ships = ships;
    *world.merchants = merchants;
    *world.convoys = convoys;
    Ok((events, freed))
}
