//! Un paso de un barco por su ruta (`ship::advance_along_route`, 0x00516CC0).
//!
//! Campos del barco: `+0` dueño, `+2` marca de ruta (u16), `+0x18` vida, `+0x1C`/`+0x20` posición en punto
//! fijo (la parte entera, i16, está en `+0x1E`/`+0x22`), `+0x24` ruta (identificador opaco), `+0x28`/`+0x2C`
//! velocidad, `+0x30` ritmo, `+0x34` punto actual de la ruta, `+0x36` la ruta es propia, `+0x38`/`+0x39`
//! ciudad de origen y de llegada, `+0x3C`/`+0x3D` marcas, `+0x134` estado, `+0x15C` marca de ruta de comercio.

use super::motion::{get_speed, set_velocity_towards};
use super::{r16, r32, w16, w32, DivideError};

/// Estado global que lee el paso.
pub struct AdvanceWorld<'a> {
    /// `0x006DE4B4`.
    pub time: u32,
    /// `0x006DE4AA`.
    pub merchant_count: u16,
    /// `0x006DE4B0`.
    pub town_count: u32,
    /// `0x006DFC14`.
    pub player_merchant: u32,
    /// `0x006DF31C`: distinto de cero, no se generan salidas de puerto.
    pub paused: u32,
    /// `0x006DE4B8`.
    pub port_index: &'a [u8],
    /// Posiciones de puerto (x en +0 e y en +4 de registros de 52 bytes en `0x006DDBB0`); el original las copia
    /// como `u32`, aquí con la parte alta a cero.
    pub positions: &'a [(i16, i16)],
    /// Registros de comerciante (+8 control, +0x19 ciudad natal).
    pub merchants: &'a [&'a [u8]],
}

/// Lo que el paso pide fuera del barco.
pub trait AdvanceHost {
    /// Puntos de la ruta `route` (`**route` apunta a ellos y `route+4` es su número).
    fn route_points(&mut self, route: u32) -> Vec<(i16, i16)>;
    /// `**route`: puntero a los puntos, para liberarlo.
    fn points_pointer(&mut self, route: u32) -> u32;
    /// `operator delete` (0x0064F7E2).
    fn free(&mut self, pointer: u32);
    /// `find_town_at_point` (0x00446D50) sobre el punto actual.
    fn find_town_at_point(&mut self, point: (i16, i16)) -> i32;
    /// Salida de un barco de comerciante humano desde puerto (0x0054AA70): `[0, índice, ciudad, x, y]`.
    fn spawn_departure(&mut self, args: [u32; 5]);
    /// Aviso de interfaz (0x0044F940) con el índice del barco.
    fn ui_route_done(&mut self, ship: u16);
    /// `do_notification` (0x00548CA0): `[dueño, 0, x, y, 8, argumento]`. Las partes altas de dueño, x e y son
    /// restos de registros del original y no se reproducen.
    fn notify(&mut self, args: [u32; 6]);
}

fn merchant_human(world: &AdvanceWorld, owner: u8) -> bool {
    r16(&world.merchants[owner as usize], 8) == 0
}

/// `ship::advance_along_route` (0x00516CC0). `arg` es el argumento de pila (índice del barco; el original solo
/// usa sus 16 bits bajos salvo en el aviso de `do_notification`, que lo pasa entero). Devuelve el `al` del
/// original: 0 si el barco ha dejado de navegar (vida agotada o llegada), 1 en otro caso.
pub fn advance_along_route(
    ship: &mut [u8],
    arg: u32,
    world: &AdvanceWorld,
    host: &mut dyn AdvanceHost,
) -> Result<u8, DivideError> {
    let owner = ship[0];
    let route = r32(ship, 0x24) as u32;
    let old_x = r16(ship, 0x1E);
    let old_y = r16(ship, 0x22);
    if (owner as u16) < world.merchant_count && world.time & 3 == 0 {
        w32(ship, 0x18, r32(ship, 0x18).wrapping_sub(1));
    }
    if r32(ship, 0x18) <= 0 {
        return Ok(0);
    }
    if route == 0 {
        if (owner as u16) < world.merchant_count && !merchant_human(world, owner) {
            ship[0x15C] = 0;
            let index = arg & 0xFFFF;
            if (index ^ world.time) & 0x3F == 0 && world.paused == 0 {
                let mut town = ship[0x38] as u32;
                if world.town_count <= town {
                    town = world.merchants[owner as usize][0x19] as u32;
                }
                let (x, y) = world.positions[world.port_index[town as usize] as usize];
                host.spawn_departure([0, index, town, x as u16 as u32, y as u16 as u32]);
                return Ok(1);
            }
        }
        return Ok(1);
    }
    w32(ship, 0x1C, r32(ship, 0x1C).wrapping_add(r32(ship, 0x28)));
    w32(ship, 0x20, r32(ship, 0x20).wrapping_add(r32(ship, 0x2C)));
    let (x, y) = (r16(ship, 0x1E) as i16, r16(ship, 0x22) as i16);
    if old_x as i32 != x as i32 || old_y as i32 != y as i32 {
        ship[0x3D] |= 0x20;
    }
    let index = r16(ship, 0x34) as usize;
    let points = host.route_points(route);
    let count = points.len() as i32;
    if index as i32 >= count {
        return Ok(1);
    }
    let mark = r16(ship, 2);
    if mark != 0 && (mark as i32) < count {
        let p = points[mark as usize];
        let (dx, dy) = (p.0 as i32 - x as i32, p.1 as i32 - y as i32);
        if dx > 0x48 || dx < -0x48 || dy > 0x48 || dy < -0x48 {
            w16(ship, 2, 0);
        }
    }
    if points[index] != (x, y) {
        return Ok(1);
    }
    let next = r16(ship, 0x34).wrapping_add(1);
    w16(ship, 0x34, next);
    if (next as i32) < count {
        w16(ship, 0x1C, 0x7FFF);
        w16(ship, 0x20, 0x7FFF);
        let speed = get_speed(ship)?;
        let target = points[next as usize];
        set_velocity_towards(ship, speed as i32, target.0, target.1)?;
        if (r32(ship, 0x30) as u32 ^ speed) & 0xFFFF_FFF0 != 0 {
            ship[0x3C] |= 0x80;
        }
        w32(ship, 0x30, ((speed << 6) as i32 / 0x73).wrapping_add(4));
        return Ok(1);
    }
    ship[0x3C] |= 0x80;
    for o in [0x28, 0x2C, 0x30] {
        w32(ship, o, 0);
    }
    let town = host.find_town_at_point((r16(ship, 0x1E) as i16, r16(ship, 0x22) as i16));
    if (0..0x28).contains(&town) {
        ship[0x3C] |= 0x28;
        if ship[0x15C] == 0 {
            let (px, py) = world.positions[world.port_index[town as usize] as usize];
            w16(ship, 0x1C, 0);
            w16(ship, 0x1E, px as u16);
            w16(ship, 0x20, 0);
            w16(ship, 0x22, py as u16);
            ship[0x39] = town as u8;
            return Ok(0);
        }
        if points.len() > 1 {
            w16(ship, 0x134, 0x14);
            ship[0x39] = town as u8;
            return Ok(0);
        }
        ship[0x39] = 0xFF;
        ship[0x38] = 0xFF;
        return Ok(1);
    }
    if ship[0x36] != 0 {
        let pointer = host.points_pointer(route);
        if pointer != 0 {
            host.free(pointer);
        }
        host.free(route);
        ship[0x36] = 0;
    }
    w32(ship, 0x24, 0);
    w32(ship, 0x48, 0);
    if owner as u32 == world.player_merchant {
        host.ui_route_done(arg as u16);
    }
    ship[0x3C] |= 8;
    if (owner as u16) < world.merchant_count && merchant_human(world, owner) {
        host.notify([owner as u32, 0, r16(ship, 0x1E) as u32, r16(ship, 0x22) as u32, 8, arg]);
    }
    Ok(1)
}
