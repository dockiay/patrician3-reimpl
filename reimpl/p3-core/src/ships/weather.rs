//! Fenómenos meteorológicos marítimos (0x005053C0).
use super::remove::{CONVOY_SIZE, SHIP_SIZE};
use super::{r16, r32, w16, w32};

/// Efecto de invalidación de la vista devuelto al anfitrión.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WeatherEffects {
    pub redraw: bool,
}

/// Procesa fenómenos de tipos 0, 1 y 2 sobre barcos y convoyes, incluida la
/// caducidad. Rechaza capitanes cercanos con ruta: su replanificación aún no
/// está conectada. Un capitán sin ruta no requiere esa replanificación. Rechaza
/// cualquier caso fuera de alcance sin alterar las imágenes recibidas.
pub fn tick_restricted_weather(
    manager: &mut [u8],
    ships: &mut [u8],
    convoys: &mut [u8],
) -> Result<WeatherEffects, &'static str> {
    tick_weather_with_captains(manager, ships, convoys, &mut |_, ship, _| {
        if r32(ship, 0x24) != 0 {
            Err("ruta de capitán ante fenómeno meteorológico no admitida")
        } else {
            Ok(())
        }
    })
}
/// Igual que `tick_restricted_weather`, delegando la replanificación de los
/// capitanes al anfitrión. El callback debe usar recursos transaccionales:
/// los cambios externos que haga no se revierten si una rama posterior falla.
pub fn tick_weather_with_captains<F>(
    manager: &mut [u8],
    ships: &mut [u8],
    convoys: &mut [u8],
    replan: &mut F,
) -> Result<WeatherEffects, &'static str>
where
    F: FnMut(u16, &mut [u8], (i16, i16)) -> Result<(), &'static str>,
{
    if manager.len() < 0xf8 {
        return Err("gestor naval truncado");
    }
    let n = usize::from(r16(manager, 0xf4));
    let nc = usize::from(r16(manager, 0xf6));
    if ships.len() < n * SHIP_SIZE || convoys.len() < nc * CONVOY_SIZE {
        return Err("flota truncada");
    }
    let mut m = manager.to_vec();
    let mut s = ships.to_vec();
    let mut c = convoys.to_vec();
    let mut redraw = false;
    for i in 0..16 {
        let at = 0x20 + i * 12;
        let id = usize::from(r16(&m, at));
        if id >= n {
            continue;
        }
        let ship = id * SHIP_SIZE;
        let x = i32::from(r16(&s, ship + 0x1e) as i16);
        let y = i32::from(r16(&s, ship + 0x22) as i16);
        w16(&mut s, ship + 0x134, 0x15);
        for link in [0xausize, 0xc] {
            let mut cursor = r16(&s, ship + link);
            let mut seen = vec![false; n];
            while usize::from(cursor) < n {
                let j = usize::from(cursor);
                if seen[j] || j == id {
                    return Err("lista espacial cíclica");
                }
                seen[j] = true;
                let q = j * SHIP_SIZE;
                let nx = i32::from(r16(&s, q + 0x1e) as i16);
                if (link == 0xa && nx < x - 36) || (link == 0xc && nx > x + 36) {
                    break;
                }
                let ny = i32::from(r16(&s, q + 0x22) as i16);
                if (y - 36..=y + 36).contains(&ny) {
                    if r16(&s, q + 0x42) < r16(&m, 0xf2) {
                        replan(cursor, &mut s[q..q + SHIP_SIZE], (x as i16, y as i16))?
                    }
                    let dx = x - nx;
                    let dy = y - ny;
                    if dx * dx + dy * dy < 0x310 {
                        let kind = r16(&m, at + 8);
                        if kind == 0 || kind == 2 {
                            let shift = if kind == 0 { 6 } else { 5 };
                            let cv = usize::from(r16(&s, q + 8));
                            if cv < nc {
                                let co = cv * CONVOY_SIZE;
                                let flags = r32(&c, co + 0x18) | 0x100;
                                w32(&mut c, co + 0x18, flags);
                                let mut member = r16(&c, co + 0xa);
                                let mut chain = vec![false; n];
                                while usize::from(member) < n {
                                    let j = usize::from(member);
                                    if chain[j] {
                                        return Err("convoy meteorológico cíclico");
                                    }
                                    chain[j] = true;
                                    let p = j * SHIP_SIZE;
                                    let flags = r16(&s, p + 0x3c) | 0x100;
                                    w16(&mut s, p + 0x3c, flags);
                                    let health = r32(&s, p + 0x18).wrapping_sub(r32(&s, p + 0x14) >> shift).max(1);
                                    w32(&mut s, p + 0x18, health);
                                    member = r16(&s, p + 6)
                                }
                            } else {
                                let flags = r16(&s, q + 0x3c) | 0x100;
                                w16(&mut s, q + 0x3c, flags);
                                let health = r32(&s, q + 0x18).wrapping_sub(r32(&s, q + 0x14) >> shift);
                                w32(&mut s, q + 0x18, health)
                            }
                        }
                        if kind == 0 || kind == 1 {
                            let mut px = r32(&s, q + 0x1c) as u32;
                            let mut py = r32(&s, q + 0x20) as u32;
                            let vx = r32(&s, q + 0x28) as u32;
                            let vy = r32(&s, q + 0x2c) as u32;
                            if vx != 0 || vy != 0 {
                                loop {
                                    let nx = px.wrapping_sub(vx);
                                    let ny = py.wrapping_sub(vy);
                                    if ((r32(&s, q + 0x1c) as u32 ^ nx) & 0xffff0000) != 0
                                        || ((r32(&s, q + 0x20) as u32 ^ ny) & 0xffff0000) != 0
                                    {
                                        break;
                                    }
                                    px = nx;
                                    py = ny;
                                    w32(&mut s, q + 0x1c, px as i32);
                                    w32(&mut s, q + 0x20, py as i32);
                                }
                            }
                            let cv = usize::from(r16(&s, q + 8));
                            if cv < nc {
                                let mut member = r16(&c, cv * CONVOY_SIZE + 0xa);
                                let mut chain = vec![false; n];
                                while usize::from(member) < n {
                                    let j = usize::from(member);
                                    if chain[j] {
                                        return Err("convoy meteorológico cíclico");
                                    }
                                    chain[j] = true;
                                    let p = j * SHIP_SIZE;
                                    w32(&mut s, p + 0x1c, px as i32);
                                    w32(&mut s, p + 0x20, py as i32);
                                    member = r16(&s, p + 6)
                                }
                            }
                        }
                    }
                }
                cursor = r16(&s, q + link);
            }
        }
        let timer = r16(&m, at + 10);
        if timer != 0 {
            if timer < 5 {
                w16(&mut m, at + 10, 0);
                redraw = true
            } else {
                w16(&mut m, at + 10, timer - 4)
            }
        }
        let life = r16(&m, at + 2);
        if life < 8 {
            let head = r16(&m, 0xe2);
            super::order::XOrder {
                ships: &mut s,
                count: n as u16,
                sea_head: r16(&m, 0xea),
            }
            .remove(id as u16);
            w16(&mut s, ship + 0x134, 0xff);
            w16(&mut s, ship + 6, head);
            w16(&mut m, 0xe2, id as u16);
            w16(&mut m, at, u16::MAX);
            if r16(&m, at + 10) != 0 {
                redraw = true
            }
        } else {
            w16(&mut m, at + 2, life - 8)
        }
    }
    manager.copy_from_slice(&m);
    ships.copy_from_slice(&s);
    convoys.copy_from_slice(&c);
    Ok(WeatherEffects { redraw })
}

/// Efecto de una inserción de desvío ante un fenómeno meteorológico.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HazardRouteEffect {
    pub ship: u16,
    pub notify_player: bool,
    pub exit_index: usize,
    pub start_index: usize,
}

/// `ship_replan_route_around_hazard` (0x00505290), con la inserción nativa del
/// anillo 0x00447580. La dirección del descriptor permanece como identificador
/// lógico del anfitrión; este debe publicar la ruta y ejecutar el aviso visual.
pub fn replan_around_hazard(
    ship: &mut [u8],
    route: &mut Vec<(i16, i16)>,
    hazard: (i16, i16),
    ship_id: u16,
    player: u32,
    grid: &crate::route::planner::NavGrid,
) -> Result<Option<HazardRouteEffect>, &'static str> {
    if ship.len() < SHIP_SIZE {
        return Err("barco truncado");
    }
    if r32(ship, 0x24) == 0 {
        return Ok(None);
    }
    let current = usize::from(r16(ship, 0x34));
    if current >= route.len() {
        return Ok(None);
    }
    let mut exit = current;
    while exit < route.len() {
        let dx = i32::from(hazard.0) - i32::from(route[exit].0);
        let dy = i32::from(hazard.1) - i32::from(route[exit].1);
        if dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy)) > 0x510 {
            break;
        }
        exit += 1
    }
    if exit == route.len() {
        return Ok(None);
    }
    let mark = usize::from(r16(ship, 2));
    if let Some(&(x, y)) = route.get(mark) {
        let dx = i32::from(x) - i32::from(hazard.0);
        let dy = i32::from(y) - i32::from(hazard.1);
        if (-36..=36).contains(&dx) && (-36..=36).contains(&dy) {
            return Ok(None);
        }
    }
    let w = usize::try_from(grid.width).map_err(|_| "rejilla inválida")?;
    let h = usize::try_from(grid.height).map_err(|_| "rejilla inválida")?;
    if w == 0 || h == 0 || w.checked_mul(h).is_none_or(|n| n > grid.cells.len()) {
        return Err("rejilla truncada");
    }
    let origin = (r16(ship, 0x1e) as i16, r16(ship, 0x22) as i16);
    let next = crate::route::finder::insert_port_ring(grid, hazard, route, origin, exit, current);
    ship[0x36] = 1;
    w32(ship, 0x28, 0);
    w32(ship, 0x2c, 0);
    w16(ship, 2, current as u16);
    *route = next;
    Ok(Some(HazardRouteEffect {
        ship: ship_id,
        notify_player: u32::from(ship[0]) == player,
        exit_index: exit,
        start_index: current,
    }))
}
