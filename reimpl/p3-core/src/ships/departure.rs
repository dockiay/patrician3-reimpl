//! Inicio acotado de la orden naval (0x00537670).
use super::{r16, r32, w16, w32};

fn refund_expedition(
    ships: &[u8],
    convoys: &mut [u8],
    merchants: &mut [u8],
    ship: usize,
    convoy: usize,
) -> Result<(), String> {
    if r16(convoys, convoy + 0x38) < 3 {
        let owner = usize::from(ships[ship]);
        let at = owner
            .checked_mul(super::remove::MERCHANT_SIZE)
            .ok_or("comerciante inválido")?;
        if at + 4 > merchants.len() {
            return Err("comerciante para reembolso ausente".into());
        }
        let gold = r32(merchants, at).wrapping_add(r32(convoys, convoy + 0x20));
        w32(merchants, at, gold);
        w32(convoys, convoy + 0x20, 0);
        w16(convoys, convoy + 0x38, u16::MAX)
    }
    Ok(())
}

/// 0x00519BA0: aptitud para zarpar, sin cambios en el barco.
pub fn ready_to_sail(ship: &[u8], towns: &[crate::town::Town], captain_count: u16) -> Option<bool> {
    if ship.len() != 0x180 {
        return None;
    }
    const CREW: [u16; 4] = [5, 8, 10, 12];
    let state = r16(ship, 0x134);
    let city = usize::from(ship[0x39]);
    Some(
        r16(ship, 0x40) >= CREW[usize::from(ship[0xe] & 3)]
            && (ship[0x3f] as i8) > 0
            && r32(ship, 0x118) <= r32(ship, 0x10)
            && (r16(ship, 0x42) >= captain_count || r32(ship, 0x14) <= r32(ship, 0x18).wrapping_mul(5))
            && state != 7
            && state != 9
            && !(0xc..=0xe).contains(&state)
            && state < 0x11
            && (city >= towns.len() || towns[city].flags() & 0x04000200 == 0),
    )
}

/// Orden a una ciudad distinta para un barco en estado 0/1 sin convoy.
/// `ready` es la respuesta del original 0x00519BA0; el llamador aporta
/// `plan_route` y ha validado propietario, índices y recursos. Devuelve si
/// se solicitó una ruta. No abarca giros, convoyes ni la salida del puerto.
pub fn start_docked<F: FnMut(&mut [u8], u8)>(
    ship: &mut [u8],
    dest: u8,
    town_count: u32,
    convoy_count: u16,
    ready: bool,
    mut plan: F,
) -> Option<bool> {
    if ship.len() != 0x180 || r16(ship, 0x134) > 1 || r16(ship, 8) < convoy_count || u32::from(dest) >= town_count {
        return None;
    }
    w16(ship, 0x3a, u16::MAX);
    ship[0x136] &= 0xfc;
    let flags = r16(ship, 0x3c) | 0x800;
    w16(ship, 0x3c, flags);
    if !ready || ship[0x39] == dest {
        return Some(false);
    }
    ship[0x3c] |= 0x20;
    ship[0x37] = ship[0x39];
    w16(ship, 0x138, 0);
    w16(ship, 0x134, 2);
    plan(ship, dest);
    if u32::from(ship[0x38]) < town_count {
        ship[0x13b] = ship[0x38]
    }
    w32(ship, 0x30, 1);
    Some(true)
}

/// Separación de un barco no líder de un convoy en puerto y preparación de la
/// orden. Las cuentas necesarias para reembolsos deben estar presentes.
pub fn start_convoy_member(
    w: &mut super::tick::TickWorld,
    ship: u16,
    dest: u8,
    points: &[(i16, i16)],
    port: (i16, i16),
    token: u32,
) -> Result<(), String> {
    use super::{
        convoy::{refresh_from_ships, remove_from_convoy},
        remove::{RemoveWorld, CONVOY_SIZE, SHIP_SIZE, TOWN_SIZE},
        tick::{ConvoyHost, TickWorld},
    };
    struct H {
        called: bool,
    }
    impl ConvoyHost for H {
        fn hook(&mut self, _: &'static str, _: &[u32]) -> u32 {
            self.called = true;
            0
        }
        fn read(&mut self, _: u32, _: u8) -> u32 {
            self.called = true;
            0
        }
    }
    let mut manager = w.remove.manager.to_vec();
    let mut ships = w.remove.ships.to_vec();
    let mut captains = w.remove.captains.to_vec();
    let mut convoys = w.remove.convoys.to_vec();
    let mut merchants = w.remove.merchants.to_vec();
    let mut towns = w.remove.towns.to_vec();
    let at = usize::from(ship) * SHIP_SIZE;
    if at + SHIP_SIZE > ships.len()
        || usize::from(dest) >= w.remove.town_count as usize
        || token == 0
        || points.is_empty()
    {
        return Err("orden naval inválida".into());
    }
    let count = r16(&manager, 0xf6);
    let cv = r16(&ships, at + 8);
    if cv >= count {
        return Err("barco sin convoy".into());
    }
    let c = usize::from(cv) * CONVOY_SIZE;
    if c + CONVOY_SIZE > convoys.len()
        || r16(&convoys, c + 0x10) == ship
        || !matches!(r16(&convoys, c + 0x12), 0 | 1 | 4 | 5)
    {
        return Err("estado de convoy no admitido".into());
    }
    let mut cur = r16(&convoys, c + 0xa);
    let mut found = false;
    for _ in 0..usize::from(r16(&manager, 0xf4)) {
        if cur >= r16(&manager, 0xf4) {
            break;
        }
        if cur == ship {
            found = true;
            break;
        }
        let pos = usize::from(cur) * SHIP_SIZE;
        if pos + SHIP_SIZE > ships.len() {
            return Err("cadena de convoy inválida".into());
        }
        cur = r16(&ships, pos + 6)
    }
    if !found {
        return Err("barco ausente de su convoy".into());
    }
    let town_images = towns
        .chunks_exact(TOWN_SIZE)
        .map(crate::town::Town::from_bytes)
        .collect::<Vec<_>>();
    let tick = w.time;
    let player = w.player;
    w16(&mut ships, at + 0x3a, u16::MAX);
    ships[at + 0x136] &= 0xfc;
    let flags = r16(&ships, at + 0x3c) | 0x800;
    w16(&mut ships, at + 0x3c, flags);
    let convoy_flags = r32(&convoys, c + 0x18) | 0x800;
    w32(&mut convoys, c + 0x18, convoy_flags);
    refund_expedition(&ships, &mut convoys, &mut merchants, at, c)?;
    {
        let mut remove = RemoveWorld {
            manager: &mut manager,
            ships: &mut ships,
            captains: &mut captains,
            convoys: &mut convoys,
            merchants: &mut merchants,
            towns: &mut towns,
            pool_limit: w.remove.pool_limit,
            merchant_count: w.remove.merchant_count,
            town_count: w.remove.town_count,
            ui_a: w.remove.ui_a,
            ui_b: w.remove.ui_b,
        };
        let mut staged = TickWorld {
            remove: &mut remove,
            time: tick,
            player,
        };
        let mut host = H { called: false };
        remove_from_convoy(&mut staged, ship, &mut host, 0);
        refresh_from_ships(&mut staged, cv).map_err(|_| "recuento de convoy inválido")?;
        if host.called {
            return Err("efecto de convoy no admitido".into());
        }
        let cap = r16(staged.remove.manager, 0xf2);
        let ships = &mut *staged.remove.ships;
        let image = &mut ships[at..at + SHIP_SIZE];
        if r16(image, 0x42) >= cap {
            image[0x3d] |= 0x10;
            image[0x15c] = 0
        }
        let ready = ready_to_sail(image, &town_images, cap).ok_or("barco inválido")?;
        if !ready {
            return Err("barco no preparado".into());
        }
        let mut planned = false;
        let done = start_docked(image, dest, w.remove.town_count, count, true, |image, d| {
            planned = super::route::apply_route_plan(image, Some((d, port.0, port.1)), port, points, true, token, tick)
                .is_some()
        })
        .ok_or("estado naval no admitido")?;
        if !done || !planned {
            return Err("ruta no preparada".into());
        }
    }
    w.remove.manager.copy_from_slice(&manager);
    w.remove.ships.copy_from_slice(&ships);
    w.remove.captains.copy_from_slice(&captains);
    w.remove.convoys.copy_from_slice(&convoys);
    w.remove.merchants.copy_from_slice(&merchants);
    w.remove.towns.copy_from_slice(&towns);
    Ok(())
}

/// Orden del líder de un convoy atracado, con reembolso de expedición.
/// Actualiza de forma transaccional el convoy y todos sus miembros.
pub fn start_convoy_leader(
    w: &mut super::tick::TickWorld,
    ship: u16,
    dest: u8,
    points: &[(i16, i16)],
    port: (i16, i16),
    token: u32,
) -> Result<(), String> {
    use super::{
        convoy::refresh_from_ships,
        remove::{RemoveWorld, CONVOY_SIZE, SHIP_SIZE, TOWN_SIZE},
        tick::TickWorld,
    };
    let mut manager = w.remove.manager.to_vec();
    let mut ships = w.remove.ships.to_vec();
    let mut captains = w.remove.captains.to_vec();
    let mut convoys = w.remove.convoys.to_vec();
    let mut merchants = w.remove.merchants.to_vec();
    let mut towns = w.remove.towns.to_vec();
    let at = usize::from(ship) * SHIP_SIZE;
    if at + SHIP_SIZE > ships.len() || u32::from(dest) >= w.remove.town_count || token == 0 || points.is_empty() {
        return Err("orden naval inválida".into());
    }
    let cap = r16(&manager, 0xf2);
    let cv = r16(&ships, at + 8);
    if cv >= r16(&manager, 0xf6) {
        return Err("barco sin convoy".into());
    }
    let c = usize::from(cv) * CONVOY_SIZE;
    if c + CONVOY_SIZE > convoys.len() || r16(&convoys, c + 0x10) != ship || r16(&ships, at + 0x134) > 1 {
        return Err("estado de convoy no admitido".into());
    }
    let mut cur = r16(&convoys, c + 0xa);
    let count = r16(&manager, 0xf4);
    let mut found = false;
    for _ in 0..usize::from(count) {
        if cur >= count {
            break;
        }
        if cur == ship {
            found = true;
            break;
        }
        let pos = usize::from(cur) * SHIP_SIZE;
        if pos + SHIP_SIZE > ships.len() {
            return Err("cadena de convoy inválida".into());
        }
        cur = r16(&ships, pos + 6)
    }
    if !found {
        return Err("líder ausente del convoy".into());
    }
    let town_images = towns
        .chunks_exact(TOWN_SIZE)
        .map(crate::town::Town::from_bytes)
        .collect::<Vec<_>>();
    let image = &mut ships[at..at + SHIP_SIZE];
    if image[0x39] == dest || !ready_to_sail(image, &town_images, cap).ok_or("barco inválido")? {
        return Err("líder no preparado o ya en destino".into());
    }
    w16(image, 0x3a, u16::MAX);
    image[0x136] &= 0xfc;
    let f = r16(image, 0x3c) | 0x820;
    w16(image, 0x3c, f);
    let f = r32(&convoys, c + 0x18) | 0x800;
    w32(&mut convoys, c + 0x18, f);
    w16(&mut convoys, c + 0x16, 0);
    image[0x37] = image[0x39];
    w16(image, 0x138, 0);
    w16(image, 0x134, 2);
    refund_expedition(&ships, &mut convoys, &mut merchants, at, c)?;
    let image = &mut ships[at..at + SHIP_SIZE];
    super::route::apply_route_plan(image, Some((dest, port.0, port.1)), port, points, true, token, w.time)
        .ok_or("ruta no preparada")?;
    if u32::from(image[0x38]) < w.remove.town_count {
        image[0x13b] = image[0x38]
    }
    w32(image, 0x30, 1);
    w16(&mut convoys, c + 0x38, u16::MAX);
    convoys[c + 0x1f] = dest;
    {
        let mut remove = RemoveWorld {
            manager: &mut manager,
            ships: &mut ships,
            captains: &mut captains,
            convoys: &mut convoys,
            merchants: &mut merchants,
            towns: &mut towns,
            pool_limit: w.remove.pool_limit,
            merchant_count: w.remove.merchant_count,
            town_count: w.remove.town_count,
            ui_a: w.remove.ui_a,
            ui_b: w.remove.ui_b,
        };
        let mut staged = TickWorld {
            remove: &mut remove,
            time: w.time,
            player: w.player,
        };
        refresh_from_ships(&mut staged, cv).map_err(|_| "recuento de convoy inválido")?;
    }
    w.remove.manager.copy_from_slice(&manager);
    w.remove.ships.copy_from_slice(&ships);
    w.remove.captains.copy_from_slice(&captains);
    w.remove.convoys.copy_from_slice(&convoys);
    w.remove.merchants.copy_from_slice(&merchants);
    w.remove.towns.copy_from_slice(&towns);
    Ok(())
}

/// Cambia el destino del líder de un convoy navegando (operación 0x00).
/// La ruta ya calculada y las cuentas de comerciantes son del anfitrión.
pub fn reroute_convoy_at_sea(
    w: &mut super::tick::TickWorld,
    ship: u16,
    dest: u8,
    points: &[(i16, i16)],
    port: (i16, i16),
    token: u32,
) -> Result<(), String> {
    reroute_convoy_destination(w, ship, Some(dest), points, port, token)
}

/// Mismo 0x00537670 para un punto externo, incluida la salida de expedición
/// (118,1600): no convierte el punto en ciudad ni modifica el último puerto.
/// El anfitrión calcula y conserva los puntos y el descriptor de la ruta.
pub fn reroute_convoy_to_point(
    w: &mut super::tick::TickWorld,
    ship: u16,
    points: &[(i16, i16)],
    point: (i16, i16),
    token: u32,
) -> Result<(), String> {
    reroute_convoy_destination(w, ship, None, points, point, token)
}
fn reroute_convoy_destination(
    w: &mut super::tick::TickWorld,
    ship: u16,
    dest: Option<u8>,
    points: &[(i16, i16)],
    port: (i16, i16),
    token: u32,
) -> Result<(), String> {
    use super::{
        convoy::refresh_from_ships,
        remove::{RemoveWorld, CONVOY_SIZE, SHIP_SIZE},
        tick::TickWorld,
    };
    let mut manager = w.remove.manager.to_vec();
    let mut ships = w.remove.ships.to_vec();
    let mut captains = w.remove.captains.to_vec();
    let mut convoys = w.remove.convoys.to_vec();
    let mut merchants = w.remove.merchants.to_vec();
    let mut towns = w.remove.towns.to_vec();
    let at = usize::from(ship) * SHIP_SIZE;
    if at + SHIP_SIZE > ships.len()
        || dest.is_some_and(|d| u32::from(d) >= w.remove.town_count)
        || token == 0
        || points.is_empty()
    {
        return Err("orden naval inválida".into());
    }
    let id = r16(&ships, at + 8);
    let c = usize::from(id) * CONVOY_SIZE;
    if id >= r16(&manager, 0xf6)
        || c + CONVOY_SIZE > convoys.len()
        || r16(&convoys, c + 0x10) != ship
        || !matches!(r16(&ships, at + 0x134), 0xf | 0x10)
        || !matches!(r16(&convoys, c + 0x12), 0xf | 0x10)
    {
        return Err("el convoy no está navegando con ese líder".into());
    }
    w16(&mut ships, at + 0x3a, u16::MAX);
    ships[at + 0x136] &= 0xfc;
    let flags = r16(&ships, at + 0x3c) | 0x800;
    w16(&mut ships, at + 0x3c, flags);
    let flags = r32(&convoys, c + 0x18) | 0x800;
    w32(&mut convoys, c + 0x18, flags);
    w16(&mut convoys, c + 0x16, 0);
    refund_expedition(&ships, &mut convoys, &mut merchants, at, c)?;
    let image = &mut ships[at..at + SHIP_SIZE];
    super::route::apply_route_plan(
        image,
        dest.map(|d| (d, port.0, port.1)),
        port,
        points,
        true,
        token,
        w.time,
    )
    .ok_or("ruta inválida")?;
    if u32::from(image[0x38]) < w.remove.town_count {
        image[0x13b] = image[0x38]
    }
    w32(image, 0x30, 1);
    w16(&mut convoys, c + 0x38, u16::MAX);
    convoys[c + 0x1f] = dest.unwrap_or(0xff);
    {
        let mut remove = RemoveWorld {
            manager: &mut manager,
            ships: &mut ships,
            captains: &mut captains,
            convoys: &mut convoys,
            merchants: &mut merchants,
            towns: &mut towns,
            pool_limit: w.remove.pool_limit,
            merchant_count: w.remove.merchant_count,
            town_count: w.remove.town_count,
            ui_a: w.remove.ui_a,
            ui_b: w.remove.ui_b,
        };
        let mut staged = TickWorld {
            remove: &mut remove,
            time: w.time,
            player: w.player,
        };
        refresh_from_ships(&mut staged, id).map_err(|_| "recuento de convoy inválido")?;
    }
    w.remove.manager.copy_from_slice(&manager);
    w.remove.ships.copy_from_slice(&ships);
    w.remove.captains.copy_from_slice(&captains);
    w.remove.convoys.copy_from_slice(&convoys);
    w.remove.merchants.copy_from_slice(&merchants);
    w.remove.towns.copy_from_slice(&towns);
    Ok(())
}

/// Separa un miembro de un convoy navegando y le asigna un destino.
pub fn reroute_convoy_member_at_sea(
    w: &mut super::tick::TickWorld,
    ship: u16,
    dest: u8,
    points: &[(i16, i16)],
    port: (i16, i16),
    token: u32,
) -> Result<(), String> {
    use super::{
        convoy::{refresh_from_ships, remove_from_convoy},
        remove::{RemoveWorld, CONVOY_SIZE, SHIP_SIZE},
        tick::{ConvoyHost, TickWorld},
    };
    struct H {
        insert: Option<(u16, u16)>,
        other: bool,
    }
    impl ConvoyHost for H {
        fn hook(&mut self, name: &'static str, a: &[u32]) -> u32 {
            if name == "insert" {
                self.insert = Some((a[0] as u16, a[1] as u16))
            } else {
                self.other = true
            }
            0
        }
        fn read(&mut self, _: u32, _: u8) -> u32 {
            self.other = true;
            0
        }
    }
    let mut manager = w.remove.manager.to_vec();
    let mut ships = w.remove.ships.to_vec();
    let mut captains = w.remove.captains.to_vec();
    let mut convoys = w.remove.convoys.to_vec();
    let mut merchants = w.remove.merchants.to_vec();
    let mut towns = w.remove.towns.to_vec();
    let at = usize::from(ship) * SHIP_SIZE;
    if at + SHIP_SIZE > ships.len() || u32::from(dest) >= w.remove.town_count || token == 0 || points.is_empty() {
        return Err("orden naval inválida".into());
    }
    let id = r16(&ships, at + 8);
    let c = usize::from(id) * CONVOY_SIZE;
    if id >= r16(&manager, 0xf6)
        || c + CONVOY_SIZE > convoys.len()
        || r16(&convoys, c + 0x10) == ship
        || r16(&ships, at + 0x134) != 0xf
        || r16(&convoys, c + 0x12) != 0xf
    {
        return Err("miembro no navegando".into());
    }
    let mut cur = r16(&convoys, c + 0xa);
    let count = r16(&manager, 0xf4);
    let mut found = false;
    for _ in 0..usize::from(count) {
        if cur >= count {
            break;
        }
        if cur == ship {
            found = true;
            break;
        }
        let pos = usize::from(cur) * SHIP_SIZE;
        if pos + SHIP_SIZE > ships.len() {
            return Err("cadena inválida".into());
        }
        cur = r16(&ships, pos + 6)
    }
    if !found {
        return Err("miembro ausente".into());
    }
    w16(&mut ships, at + 0x3a, u16::MAX);
    ships[at + 0x136] &= 0xfc;
    let flags = r16(&ships, at + 0x3c) | 0x800;
    w16(&mut ships, at + 0x3c, flags);
    let flags = r32(&convoys, c + 0x18) | 0x800;
    w32(&mut convoys, c + 0x18, flags);
    refund_expedition(&ships, &mut convoys, &mut merchants, at, c)?;
    {
        let mut remove = RemoveWorld {
            manager: &mut manager,
            ships: &mut ships,
            captains: &mut captains,
            convoys: &mut convoys,
            merchants: &mut merchants,
            towns: &mut towns,
            pool_limit: w.remove.pool_limit,
            merchant_count: w.remove.merchant_count,
            town_count: w.remove.town_count,
            ui_a: w.remove.ui_a,
            ui_b: w.remove.ui_b,
        };
        let mut staged = TickWorld {
            remove: &mut remove,
            time: w.time,
            player: w.player,
        };
        let mut host = H {
            insert: None,
            other: false,
        };
        remove_from_convoy(&mut staged, ship, &mut host, 0);
        if host.other {
            return Err("efecto de convoy no conectado".into());
        }
        if let Some((id, after)) = host.insert {
            super::order::insert_from_tick(&mut staged, id, after)
        }
        refresh_from_ships(&mut staged, id).map_err(|_| "recuento inválido")?;
        let cap = r16(staged.remove.manager, 0xf2);
        let image = &mut staged.remove.ships[at..at + SHIP_SIZE];
        if r16(image, 0x42) >= cap {
            image[0x3d] |= 0x10;
            image[0x15c] = 0
        }
        super::route::apply_route_plan(image, Some((dest, port.0, port.1)), port, points, true, token, w.time)
            .ok_or("ruta inválida")?;
        image[0x13b] = image[0x38];
        w32(image, 0x30, 1);
    }
    w.remove.manager.copy_from_slice(&manager);
    w.remove.ships.copy_from_slice(&ships);
    w.remove.captains.copy_from_slice(&captains);
    w.remove.convoys.copy_from_slice(&convoys);
    w.remove.merchants.copy_from_slice(&merchants);
    w.remove.towns.copy_from_slice(&towns);
    Ok(())
}

/// Invierte la maniobra de puerto del líder. Devuelve true si se planificó
/// una nueva ruta (atraque interrumpido), false si se vuelve al puerto de salida.
pub fn reverse_convoy_phase(
    w: &mut super::tick::TickWorld,
    ship: u16,
    dest: u8,
    points: &[(i16, i16)],
    port: (i16, i16),
    token: u32,
) -> Result<bool, String> {
    use super::{
        convoy::refresh_from_ships,
        remove::{RemoveWorld, CONVOY_SIZE, SHIP_SIZE},
        route::{clear_route, RouteHost},
        tick::TickWorld,
    };
    struct H;
    impl RouteHost for H {
        fn points_of(&mut self, _: u32) -> u32 {
            1
        }
        fn free(&mut self, _: u32) {}
        fn ui_route_cleared(&mut self, _: u16) {}
    }
    let mut manager = w.remove.manager.to_vec();
    let mut ships = w.remove.ships.to_vec();
    let mut captains = w.remove.captains.to_vec();
    let mut convoys = w.remove.convoys.to_vec();
    let mut merchants = w.remove.merchants.to_vec();
    let mut towns = w.remove.towns.to_vec();
    let at = usize::from(ship) * SHIP_SIZE;
    if at + SHIP_SIZE > ships.len() || u32::from(dest) >= w.remove.town_count {
        return Err("orden inválida".into());
    }
    let id = r16(&ships, at + 8);
    let c = usize::from(id) * CONVOY_SIZE;
    if id >= r16(&manager, 0xf6) || c + CONVOY_SIZE > convoys.len() || r16(&convoys, c + 0x10) != ship {
        return Err("líder inválido".into());
    }
    let state = r16(&ships, at + 0x134);
    if !matches!(state, 2 | 3) || r16(&convoys, c + 0x12) != state {
        return Err("fase incompatible".into());
    }
    let origin = ships[at + 0x37];
    let town_images = towns
        .chunks_exact(super::remove::TOWN_SIZE)
        .map(crate::town::Town::from_bytes)
        .collect::<Vec<_>>();
    if !ready_to_sail(&ships[at..at + SHIP_SIZE], &town_images, r16(&manager, 0xf2)).unwrap_or(false) {
        return Err("líder no preparado".into());
    }
    if ((state == 3 && dest != ships[at + 0x38]) || (state == 2 && dest != origin)) && (token == 0 || points.is_empty())
    {
        return Err("ruta o reembolso no admitido".into());
    }
    w16(&mut ships, at + 0x3a, u16::MAX);
    ships[at + 0x136] &= 0xfc;
    let f = r16(&ships, at + 0x3c) | 0x800;
    w16(&mut ships, at + 0x3c, f);
    let f = r32(&convoys, c + 0x18) | 0x800;
    w32(&mut convoys, c + 0x18, f);
    w16(&mut convoys, c + 0x16, 0);
    if state == 3 && dest == ships[at + 0x38] {
        w.remove.manager.copy_from_slice(&manager);
        w.remove.ships.copy_from_slice(&ships);
        w.remove.captains.copy_from_slice(&captains);
        w.remove.convoys.copy_from_slice(&convoys);
        w.remove.merchants.copy_from_slice(&merchants);
        w.remove.towns.copy_from_slice(&towns);
        return Ok(false);
    }
    if state == 2 && dest == origin {
        let phase = 0x40u16.wrapping_sub(r16(&ships, at + 0x138));
        w16(&mut convoys, c + 0x12, 3);
        w16(&mut convoys, c + 0x16, phase);
        convoys[c + 0x1f] = origin;
        let count = r16(&manager, 0xf4);
        let mut current = r16(&convoys, c + 0xa);
        let mut seen = vec![false; usize::from(count)];
        while current < count {
            let i = usize::from(current);
            if seen[i] {
                return Err("ciclo de convoy".into());
            }
            seen[i] = true;
            let o = i * SHIP_SIZE;
            if o + SHIP_SIZE > ships.len() {
                return Err("miembro inexistente".into());
            }
            let phase = 0x40u16.wrapping_sub(r16(&ships, o + 0x138));
            ships[o + 0x3c] |= 0x20;
            w16(&mut ships, o + 0x134, 3);
            w16(&mut ships, o + 0x138, phase);
            ships[o + 0x38] = ships[o + 0x37];
            w32(
                &mut ships,
                o + 0x48,
                w.time.wrapping_sub(u32::from(phase)).wrapping_add(0x40) as i32,
            );
            clear_route(&mut ships[o..o + SHIP_SIZE], ship, w.player, false, &mut H);
            current = r16(&ships, o + 6)
        }
    } else {
        if state == 3 {
            let phase = 0x40u16.wrapping_sub(r16(&ships, at + 0x138));
            w16(&mut ships, at + 0x134, 2);
            ships[at + 0x3c] |= 0x28;
            w16(&mut ships, at + 0x138, phase);
            ships[at + 0x37] = ships[at + 0x39];
            let flags = r32(&convoys, c + 0x18) | 0x28;
            w32(&mut convoys, c + 0x18, flags);
            let town = ships[at + 0x39];
            convoys[c + 0x1e] = town;
            w16(&mut convoys, c + 0x12, 2);
            w16(&mut convoys, c + 0x16, phase);
        }
        refund_expedition(&ships, &mut convoys, &mut merchants, at, c)?;
        let image = &mut ships[at..at + SHIP_SIZE];
        super::route::apply_route_plan(image, Some((dest, port.0, port.1)), port, points, true, token, w.time)
            .ok_or("ruta inválida")?;
        image[0x13b] = image[0x38];
        w32(image, 0x30, 1);
        w16(&mut convoys, c + 0x38, u16::MAX);
        convoys[c + 0x1f] = dest;
        let mut remove = RemoveWorld {
            manager: &mut manager,
            ships: &mut ships,
            captains: &mut captains,
            convoys: &mut convoys,
            merchants: &mut merchants,
            towns: &mut towns,
            pool_limit: w.remove.pool_limit,
            merchant_count: w.remove.merchant_count,
            town_count: w.remove.town_count,
            ui_a: w.remove.ui_a,
            ui_b: w.remove.ui_b,
        };
        let mut staged = TickWorld {
            remove: &mut remove,
            time: w.time,
            player: w.player,
        };
        refresh_from_ships(&mut staged, id).map_err(|_| "recuento inválido")?;
    }
    w.remove.manager.copy_from_slice(&manager);
    w.remove.ships.copy_from_slice(&ships);
    w.remove.captains.copy_from_slice(&captains);
    w.remove.convoys.copy_from_slice(&convoys);
    w.remove.merchants.copy_from_slice(&merchants);
    w.remove.towns.copy_from_slice(&towns);
    Ok(state == 3 || dest != origin)
}

/// Separa un miembro durante salida/atraque y aplica su nueva orden.
pub fn order_member_during_maneuver(
    w: &mut super::tick::TickWorld,
    ship: u16,
    dest: u8,
    points: &[(i16, i16)],
    port: (i16, i16),
    token: u32,
) -> Result<bool, String> {
    use super::{
        convoy::{refresh_from_ships, remove_from_convoy},
        remove::{RemoveWorld, CONVOY_SIZE, SHIP_SIZE, TOWN_SIZE},
        tick::{ConvoyHost, TickWorld},
    };
    struct H {
        called: bool,
    }
    impl ConvoyHost for H {
        fn hook(&mut self, _: &'static str, _: &[u32]) -> u32 {
            self.called = true;
            0
        }
        fn read(&mut self, _: u32, _: u8) -> u32 {
            self.called = true;
            0
        }
    }
    let mut manager = w.remove.manager.to_vec();
    let mut ships = w.remove.ships.to_vec();
    let mut captains = w.remove.captains.to_vec();
    let mut convoys = w.remove.convoys.to_vec();
    let mut merchants = w.remove.merchants.to_vec();
    let mut towns = w.remove.towns.to_vec();
    let at = usize::from(ship) * SHIP_SIZE;
    if at + SHIP_SIZE > ships.len() || u32::from(dest) >= w.remove.town_count || token == 0 {
        return Err("orden inválida".into());
    }
    let id = r16(&ships, at + 8);
    let c = usize::from(id) * CONVOY_SIZE;
    if id >= r16(&manager, 0xf6)
        || c + CONVOY_SIZE > convoys.len()
        || r16(&convoys, c + 0x10) == ship
        || !matches!(r16(&convoys, c + 0x12), 2 | 3)
    {
        return Err("miembro fuera de maniobra".into());
    }
    let count = r16(&manager, 0xf4);
    let mut cur = r16(&convoys, c + 0xa);
    let mut found = false;
    for _ in 0..usize::from(count) {
        if cur >= count {
            break;
        }
        if cur == ship {
            found = true;
            break;
        }
        let p = usize::from(cur) * SHIP_SIZE;
        if p + SHIP_SIZE > ships.len() {
            return Err("cadena inválida".into());
        }
        cur = r16(&ships, p + 6)
    }
    if !found {
        return Err("miembro ausente".into());
    }
    let town_images = towns
        .chunks_exact(TOWN_SIZE)
        .map(crate::town::Town::from_bytes)
        .collect::<Vec<_>>();
    w16(&mut ships, at + 0x3a, u16::MAX);
    ships[at + 0x136] &= 0xfc;
    let f = r16(&ships, at + 0x3c) | 0x800;
    w16(&mut ships, at + 0x3c, f);
    let f = r32(&convoys, c + 0x18) | 0x800;
    w32(&mut convoys, c + 0x18, f);
    {
        let mut remove = RemoveWorld {
            manager: &mut manager,
            ships: &mut ships,
            captains: &mut captains,
            convoys: &mut convoys,
            merchants: &mut merchants,
            towns: &mut towns,
            pool_limit: w.remove.pool_limit,
            merchant_count: w.remove.merchant_count,
            town_count: w.remove.town_count,
            ui_a: w.remove.ui_a,
            ui_b: w.remove.ui_b,
        };
        let mut stage = TickWorld {
            remove: &mut remove,
            time: w.time,
            player: w.player,
        };
        let mut host = H { called: false };
        remove_from_convoy(&mut stage, ship, &mut host, 0);
        if host.called {
            return Err("efecto no conectado".into());
        }
        refresh_from_ships(&mut stage, id).map_err(|_| "recuento inválido")?;
    }
    let cap = r16(&manager, 0xf2);
    if r16(&ships, at + 0x42) >= cap {
        ships[at + 0x3d] |= 0x10;
        ships[at + 0x15c] = 0
    }
    if !ready_to_sail(&ships[at..at + SHIP_SIZE], &town_images, cap).unwrap_or(false) {
        return Err("miembro no preparado".into());
    }
    let routed = dest != ships[at + 0x38];
    if routed {
        if points.is_empty() {
            return Err("ruta ausente".into());
        }
        w16(&mut ships, at + 0x134, 2);
        ships[at + 0x3c] |= 0x28;
        let phase = 0x40u16.wrapping_sub(r16(&ships, at + 0x138));
        w16(&mut ships, at + 0x138, phase);
        ships[at + 0x37] = ships[at + 0x39];
        refund_expedition(&ships, &mut convoys, &mut merchants, at, c)?;
        let image = &mut ships[at..at + SHIP_SIZE];
        super::route::apply_route_plan(image, Some((dest, port.0, port.1)), port, points, true, token, w.time)
            .ok_or("ruta inválida")?;
        image[0x13b] = image[0x38];
        w32(image, 0x30, 1)
    }
    w.remove.manager.copy_from_slice(&manager);
    w.remove.ships.copy_from_slice(&ships);
    w.remove.captains.copy_from_slice(&captains);
    w.remove.convoys.copy_from_slice(&convoys);
    w.remove.merchants.copy_from_slice(&merchants);
    w.remove.towns.copy_from_slice(&towns);
    Ok(routed)
}
