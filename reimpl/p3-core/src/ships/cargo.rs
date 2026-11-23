//! Carga de un barco sobre sus 0x180 bytes: puente con `ShipCargo` (C04) para `ship::recompute_load`.
//!
//! Campos: `+0xE` tipo, `+0x10` capacidad, `+0x40` tripulación (u16), `+0x54` existencias (24 × i32),
//! `+0xB4` precio medio (24 × f32), `+0x114` valor, `+0x118` peso, `+0x11C` peso del equipo, `+0x120`
//! potencia de cañones, `+0x13C` cañones (24 × u8), `+0x154` espadas (u16) y `+0x156` soldados.

use super::{r16, r32, w32};
use crate::economy::ship_trade::ShipCargo;

/// `ship::recompute_load` (0x005182B0) sobre los bytes del barco; devuelve la capacidad libre sin redondear.
pub fn recompute_load(ship: &mut [u8]) -> i32 {
    let mut cargo = ShipCargo {
        kind: ship[0xE],
        capacity: r32(ship, 0x10),
        crew: r16(ship, 0x40),
        marines: ship[0x156],
        cutlasses: r16(ship, 0x154),
        ..ShipCargo::default()
    };
    for ware in 0..24 {
        cargo.stock[ware] = r32(ship, 0x54 + 4 * ware);
        cargo.average_price[ware] = f32::from_bits(r32(ship, 0xB4 + 4 * ware) as u32);
        cargo.guns[ware] = ship[0x13C + ware];
    }
    let free = cargo.recompute_load();
    for ware in 0..24 {
        w32(ship, 0x54 + 4 * ware, cargo.stock[ware]);
    }
    w32(ship, 0x114, cargo.cargo_value);
    w32(ship, 0x118, cargo.cargo_weight);
    w32(ship, 0x11C, cargo.equipment_weight);
    w32(ship, 0x120, cargo.gun_power);
    free
}
