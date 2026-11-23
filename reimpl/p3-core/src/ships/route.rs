//! Ruta de un barco: duración estimada, borrado y posición inicial en una ciudad
//! (`ship::route_duration` 0x005169C0, `ship::clear_route` 0x00516840, `ship::reset_position_to_town` 0x00517170).
//!
//! Campos: `+0` dueño, `+2`/`+3` bytes de ruta, `+0xE` tipo, `+0x10` capacidad, `+0x14`/`+0x18` vida máxima y
//! vida, `+0x1C`/`+0x20` desplazamiento, `+0x1E`/`+0x22` posición (i16), `+0x24` ruta (puntero), `+0x34`,
//! `+0x36` la ruta es propia, `+0x37`..`+0x39` ciudades, `+0x3D` marcas, `+0x118` peso de la carga.

use super::motion::SPEED_TABLE;
use super::{r32, w16, w32, DivideError};

/// `ship::route_duration` (0x005169C0): ticks estimados para recorrer `route` (ninguna ruta: 0). Suma por tramo
/// `16·mayor + 7·menor` de los desplazamientos absolutos, la multiplica por 8 y la divide entre la velocidad
/// (como en `get_speed` pero con la vida acotada a 0xCC..0x100).
pub fn route_duration(ship: &[u8], route: Option<&[(i16, i16)]>) -> Result<u32, DivideError> {
    let Some(points) = route else { return Ok(0) };
    let mut total = 0i32;
    for pair in points.windows(2) {
        let dx = (pair[0].0 as i32 - pair[1].0 as i32).abs();
        let dy = (pair[0].1 as i32 - pair[1].1 as i32).abs();
        total = total.wrapping_add(if dy < dx { dx * 16 + dy * 7 } else { dy * 16 + dx * 7 });
    }
    let max_health = r32(ship, 0x14) as u32;
    let capacity = r32(ship, 0x10);
    if max_health == 0 || capacity == 0 || (r32(ship, 0x118).wrapping_mul(0x266) == i32::MIN && capacity == -1) {
        return Err(DivideError);
    }
    let hull = ((r32(ship, 0x18).wrapping_mul(0x82)) as u32 / max_health)
        .wrapping_add(0xA5)
        .clamp(0xCC, 0x100);
    let fill = r32(ship, 0x118).wrapping_mul(0x266) / capacity;
    let table = SPEED_TABLE[(ship[0xE] & 3) as usize] as u32;
    let by_load = (0x1000u32.wrapping_sub(fill as u32)).wrapping_mul(table) >> 12;
    let speed = hull.wrapping_mul(by_load) >> 10;
    if speed == 0 {
        return Err(DivideError);
    }
    Ok((total.wrapping_mul(8) as u32) / speed)
}

/// Lo que `clear_route` pide fuera del barco.
pub trait RouteHost {
    /// Primer campo del bloque de ruta (`**(ship+0x24)`): el puntero a sus puntos.
    fn points_of(&mut self, route: u32) -> u32;
    /// `operator delete` (0x0064F7E2).
    fn free(&mut self, pointer: u32);
    /// Aviso de interfaz (0x0044F940) con el índice del barco.
    fn ui_route_cleared(&mut self, ship: u16);
}

/// `ship::clear_route` (0x00516840). `player_merchant` y `ui_present` son las globales 0x6DFC14 y 0x6CBDD8
/// (distinta de cero). Solo libera la ruta si el barco la posee (`+0x36`).
pub fn clear_route(ship: &mut [u8], index: u16, player_merchant: u32, ui_present: bool, host: &mut dyn RouteHost) {
    let route = r32(ship, 0x24) as u32;
    if route != 0 {
        if ship[0x36] != 0 {
            let points = host.points_of(route);
            if points != 0 {
                host.free(points);
            }
            host.free(route);
            ship[0x36] = 0;
        }
        w32(ship, 0x24, 0);
        if ship[0] as u32 == player_merchant && ui_present {
            host.ui_route_cleared(index);
        }
    }
    w16(ship, 0x34, 0);
    ship[2] = 0;
    ship[3] = 0;
}

/// `ship::reset_position_to_town` (0x00517170): coloca el barco en el puerto de su ciudad actual, o de la
/// anterior, o de la de origen (si es válida; si no, la 0). `port_index` es 0x006DE4B8 y `positions` las
/// posiciones del puerto (x en +0 e y en +4 de un registro de 52 bytes en 0x006DDBB0).
pub fn reset_position_to_town(ship: &mut [u8], town_count: u32, port_index: &[u8], positions: &[(i16, i16)]) {
    let mut town = ship[0x39] as u32;
    if town_count <= town {
        town = ship[0x37] as u32;
        if town_count <= town {
            town = if (ship[0x38] as u32) < town_count {
                ship[0x38] as u32
            } else {
                0
            };
        }
    }
    let (x, y) = positions[port_index[town as usize] as usize];
    w32(ship, 0x1C, 0);
    w32(ship, 0x20, 0);
    w16(ship, 0x1E, x as u16);
    ship[0x3D] |= 0x20;
    w16(ship, 0x22, y as u16);
}

/// Aplica `ship::plan_route` (0x005172a0) a una ruta calculada por el planificador.
/// `token` identifica el descriptor en el almacén de rutas del anfitrión.
pub fn apply_route_plan(
    ship: &mut [u8],
    destination: Option<(u8, i16, i16)>,
    point: (i16, i16),
    points: &[(i16, i16)],
    owned: bool,
    token: u32,
    tick: u32,
) -> Option<[(i16, i16); 2]> {
    if ship.len() < 0x180 || token == 0 {
        return None;
    }
    let x = i16::from_le_bytes([ship[0x1e], ship[0x1f]]);
    let y = i16::from_le_bytes([ship[0x22], ship[0x23]]);
    let (city, px, py) = destination.unwrap_or((255, point.0, point.1));
    let duration = crate::ships::route::route_duration(ship, Some(points)).ok()?;
    ship[0x38] = city;
    ship[0x1c..0x1e].copy_from_slice(&0x7fffu16.to_le_bytes());
    ship[0x20..0x22].copy_from_slice(&0x7fffu16.to_le_bytes());
    ship[0x34..0x36].fill(0);
    ship[2..4].fill(0);
    ship[0x28..0x30].fill(0);
    ship[0x24..0x28].copy_from_slice(&token.to_le_bytes());
    ship[0x36] = u8::from(owned);
    ship[0x48..0x4c].copy_from_slice(&tick.wrapping_add(duration).to_le_bytes());
    Some([(x, y), (px, py)])
}

/// `ship_start_route_from_port` (0x00516BC0). Si no hay descriptor, el
/// llamador suministra la ruta calculada y su token; el error es atómico.
pub fn start_route_from_port(
    ship: &mut [u8],
    origin: (u16, u16),
    prepared: Option<(&[(i16, i16)], bool, u32)>,
    tick: u32,
) -> Option<()> {
    if ship.len() != 0x180 {
        return None;
    }
    let mut next = ship.to_vec();
    let old_city = next[0x39];
    next[0x28..0x30].fill(0);
    next[0x37] = old_city;
    next[0x1c..0x20].copy_from_slice(&(u32::from(origin.0) << 16).to_le_bytes());
    next[0x20..0x24].copy_from_slice(&(u32::from(origin.1) << 16).to_le_bytes());
    if next[0x24..0x28] == [0; 4] {
        let (points, owned, token) = prepared?;
        if token == 0 {
            return None;
        }
        let duration = route_duration(&next, Some(points)).ok()?;
        next[0x24..0x28].copy_from_slice(&token.to_le_bytes());
        next[0x36] = u8::from(owned);
        next[0x48..0x4c].copy_from_slice(&tick.wrapping_add(duration).to_le_bytes());
    }
    next[0x34..0x36].fill(0);
    next[2..4].fill(0);
    next[1] = if next[1] > 7 { 2 } else { next[1].wrapping_sub(4) & 7 };
    ship.copy_from_slice(&next);
    Some(())
}
