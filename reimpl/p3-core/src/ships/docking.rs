//! Salida y llegada a puerto de un barco (`ships::put_at_sea`, `ships::set_sail`, `ship::finish_docking`).
//!
//! Los barcos son los 0x180 bytes del original dentro de un array contiguo; las ciudades y los comerciantes
//! se pasan como en el resto del núcleo. Los índices de ciudad y de comerciante se usan sin comprobar el
//! rango salvo donde el original lo hace.

use super::cargo::recompute_load;
use super::order::XOrder;
use super::{r16, r32, w16, w32};
use crate::shipyard::SHIP_SIZE;
use crate::town::Town;

/// Lo que las funciones piden al resto del juego.
pub trait DockHost {
    /// `ship::plan_route` en su forma 0x00516BC0: traza la ruta del barco a su ciudad de destino.
    fn plan_route(&mut self, ship: u16);
    /// Variante para integrar la preparación nativa sobre la imagen prestada.
    /// El comportamiento por defecto conserva los anfitriones de observación.
    fn plan_route_image(&mut self, ship: u16, _image: &mut [u8]) {
        self.plan_route(ship)
    }
    /// Aviso a la interfaz de que el barco del jugador zarpa (0x0044F940), con el índice del barco.
    fn player_ship_sailed(&mut self, ship: u16);
    /// `do_notification` (0x00548CA0), con los seis argumentos de pila tal como se empujan.
    fn notify(&mut self, args: [u32; 6]);
}

/// Estado global que leen estas funciones.
pub struct DockWorld<'a> {
    /// `0x006DE4B4`: hora del juego.
    pub time: u32,
    /// `0x006DE4B0`: número de ciudades.
    pub town_count: u32,
    /// `0x006DE4AA`: número de comerciantes.
    pub merchant_count: u16,
    /// `0x006DFC14`: comerciante del jugador.
    pub player_merchant: u32,
    /// `0x006DD896`: mitad alta del contador de barcos.
    pub ship_threshold: u16,
    pub towns: &'a mut [Town],
    /// Registros de comerciante (0x650 bytes cada uno).
    pub merchants: &'a mut [Vec<u8>],
}

fn ship_mut(ships: &mut [u8], index: u16) -> &mut [u8] {
    let at = index as usize * SHIP_SIZE;
    &mut ships[at..at + SHIP_SIZE]
}

/// `ships::put_at_sea` (0x00505C30): inserta el barco en la lista del mar y lo deja sin ciudad actual.
pub fn put_at_sea(list: &mut XOrder, ship: u16) {
    let head = list.sea_head as u32;
    list.insert(ship, head);
    recompute_load(ship_mut(list.ships, ship));
    let sea_head = list.sea_head;
    let record = ship_mut(list.ships, ship);
    w16(record, 6, sea_head);
    record[0x39] = 0xFF;
    list.sea_head = ship;
}

/// `ships::set_sail` (0x005090E0): el barco zarpa. Estado 0xF, sin ciudad actual, suma su cargamento a las
/// dos tablas de la ciudad de destino y acumula el factor de carga en el barco y en el comerciante. Un
/// dueño inexistente deja el barco en estado 0x12.
pub fn set_sail(list: &mut XOrder, world: &mut DockWorld, host: &mut dyn DockHost, ship: u16) {
    let time = world.time;
    let record = ship_mut(list.ships, ship);
    w32(record, 0x44, time as i32);
    recompute_load(record);
    host.plan_route_image(ship, record);
    let record = ship_mut(list.ships, ship);
    record[0x3C] |= 0x28;
    let from = record[0x38];
    w16(record, 0x134, 0xF);
    w16(record, 0x138, 0);
    record[0x39] = 0xFF;
    let mut sum = 0i32;
    if (from as u32) < world.town_count {
        let town = &mut world.towns[from as usize];
        for ware in 0..24 {
            let stock = r32(record, 0x54 + 4 * ware);
            if stock > 0 {
                let a = town.i32(0x670 + 4 * ware).wrapping_add(stock);
                town.set_u32(0x670 + 4 * ware, a as u32);
                let b = town.i32(0x370 + 4 * ware).wrapping_add(stock);
                town.set_u32(0x370 + 4 * ware, b as u32);
                sum = sum.wrapping_add(stock);
            }
        }
    }
    let after = list.sea_head as u32;
    list.insert(ship, after);
    let record = ship_mut(list.ships, ship);
    let owner = record[0];
    if (owner as u16) < world.merchant_count {
        let ratio = ((sum as f32) as f64 / (r32(record, 0x10) as f32) as f64) as f32;
        let m = &mut world.merchants[owner as usize];
        let total = (ratio as f64 + f32::from_bits(r32(m, 0x540) as u32) as f64) as f32;
        w32(m, 0x540, total.to_bits() as i32);
        let count = r16(m, 0x544).wrapping_add(1);
        w16(m, 0x544, count);
        let own = (ratio as f64 + f32::from_bits(r32(record, 0x12C) as u32) as f64) as f32;
        let own_count = r16(record, 0x130).wrapping_add(1);
        w16(record, 0x130, own_count);
        w32(record, 0x12C, own.to_bits() as i32);
        if owner as u32 == world.player_merchant {
            host.player_ship_sailed(ship);
        }
    } else {
        w16(record, 0x134, 0x12);
    }
}

/// Aplica `set_sail` al mundo contiguo del tick, antes de que este cambie la lista.
/// La preparación de la ruta y los avisos siguen siendo responsabilidad del anfitrión.
pub fn set_sail_from_tick(world: &mut super::tick::TickWorld, host: &mut dyn DockHost, ship: u16, ship_threshold: u16) {
    use super::remove::{MERCHANT_SIZE, TOWN_SIZE};
    let remove = &mut *world.remove;
    let mut towns: Vec<Town> = remove.towns.chunks_exact(TOWN_SIZE).map(Town::from_bytes).collect();
    let mut merchants: Vec<Vec<u8>> = remove
        .merchants
        .chunks_exact(MERCHANT_SIZE)
        .map(<[u8]>::to_vec)
        .collect();
    let mut dock = DockWorld {
        time: world.time,
        town_count: remove.town_count,
        merchant_count: remove.merchant_count,
        player_merchant: world.player,
        ship_threshold,
        towns: &mut towns,
        merchants: &mut merchants,
    };
    let mut order = XOrder {
        ships: remove.ships,
        count: r16(remove.manager, 0xF4),
        sea_head: r16(remove.manager, 0xEA),
    };
    set_sail(&mut order, &mut dock, host, ship);
    for (dst, town) in remove.towns.chunks_exact_mut(TOWN_SIZE).zip(towns) {
        dst.copy_from_slice(town.0.as_slice());
    }
    for (dst, merchant) in remove.merchants.chunks_exact_mut(MERCHANT_SIZE).zip(merchants) {
        dst.copy_from_slice(&merchant);
    }
}

/// `ship::finish_docking` (0x00519C90): remate del atraque. Resta la carga de las tablas de la ciudad de
/// origen (la segunda sin bajar de 0), suma 1 a `Town+0x996` de la de llegada y, para un comerciante humano
/// con barco sin marca, avisa con el tipo 5. Las partes altas de los argumentos de aviso son restos de
/// registros del original: la del dueño es la del índice del barco y la de la ciudad la de su desplazamiento.
pub fn finish_docking(ships: &mut [u8], world: &mut DockWorld, host: &mut dyn DockHost, ship: u16) {
    let time = world.time;
    let record = ship_mut(ships, ship);
    record[0x3C] |= 0x20;
    w16(record, 0x134, 0);
    w32(record, 0x30, 0);
    w32(record, 0x44, time as i32);
    let arrival = &mut world.towns[record[0x39] as usize];
    let visits = arrival.u16(0x996).wrapping_add(1);
    arrival.set_u16(0x996, visits);
    let town = &mut world.towns[record[0x38] as usize];
    for ware in 0..24 {
        let stock = r32(record, 0x54 + 4 * ware);
        let a = town.i32(0x670 + 4 * ware).wrapping_sub(stock);
        let b = town.i32(0x370 + 4 * ware).wrapping_sub(stock);
        town.set_u32(0x370 + 4 * ware, b as u32);
        town.set_u32(0x670 + 4 * ware, a.max(0) as u32);
    }
    let merchant = &world.merchants[record[0] as usize];
    if r16(merchant, 8) == 0 && record[0x136] & 1 == 0 && r16(record, 8) >= world.ship_threshold {
        let offset = ship as u32 * SHIP_SIZE as u32;
        host.notify([
            record[0] as u32 | (ship as u32 >> 16) << 16,
            0,
            0,
            record[0x39] as u32 | (offset >> 16) << 16,
            5,
            ship as u32,
        ]);
    }
    w16(record, 0x138, 0);
}

/// Remata el atraque sobre la imagen contigua del tick, antes de seguir la lista.
pub fn finish_docking_from_tick(
    world: &mut super::tick::TickWorld,
    host: &mut dyn DockHost,
    ship: u16,
    ship_threshold: u16,
) {
    use super::remove::{MERCHANT_SIZE, TOWN_SIZE};
    let remove = &mut *world.remove;
    let mut towns: Vec<Town> = remove.towns.chunks_exact(TOWN_SIZE).map(Town::from_bytes).collect();
    let mut merchants: Vec<Vec<u8>> = remove
        .merchants
        .chunks_exact(MERCHANT_SIZE)
        .map(<[u8]>::to_vec)
        .collect();
    let mut dock = DockWorld {
        time: world.time,
        town_count: remove.town_count,
        merchant_count: remove.merchant_count,
        player_merchant: world.player,
        ship_threshold,
        towns: &mut towns,
        merchants: &mut merchants,
    };
    finish_docking(remove.ships, &mut dock, host, ship);
    for (dst, town) in remove.towns.chunks_exact_mut(TOWN_SIZE).zip(towns) {
        dst.copy_from_slice(town.0.as_slice());
    }
    for (dst, merchant) in remove.merchants.chunks_exact_mut(MERCHANT_SIZE).zip(merchants) {
        dst.copy_from_slice(&merchant);
    }
}
