//! Experiencia y niveles semanales de los astilleros (tarea 0x06).
use crate::town::Town;

#[derive(Clone, Copy, Debug)]
pub struct YardShip {
    pub next: u16,
    pub capacity: i32,
}
const REQUIREMENTS: [[u32; 4]; 4] = [
    [0, 100, 300, 1050],
    [0, 100, 600, 900],
    [0, 200, 400, 800],
    [300, 500, 600, 1200],
];

/// Devuelve ciudades que anuncian el primer casco grande a los comerciantes.
pub fn update(towns: &mut [Town], ships: &[YardShip]) -> Result<Vec<u8>, String> {
    let mut staged = towns.to_vec();
    let mut notices = Vec::new();
    for (city, town) in staged.iter_mut().enumerate() {
        let mut at = usize::from(town.u16(0x81c));
        let mut seen = vec![false; ships.len()];
        let mut capacity = 0i32;
        while at < ships.len() {
            if seen[at] {
                return Err("cadena de pedidos del astillero cíclica".into());
            }
            seen[at] = true;
            let ship = ships[at];
            capacity = capacity.wrapping_add(ship.capacity);
            at = usize::from(ship.next)
        }
        let pending = town.u32(0x814);
        let exp = town.u32(0x810).wrapping_add(pending);
        town.set_u32(0x810, exp);
        town.set_u32(0x814, 0);
        let amount = (capacity / 2000).wrapping_add((pending / 19600) as i32);
        let old = f32::from_bits(town.u32(0x818));
        let value = ((amount as f32 as f64) / 26.0 + (old as f64) * (0.96153843f32 as f64)) as f32;
        town.set_u32(0x818, value.to_bits());
        for kind in 0..4 {
            let old = town.u8(0x824 + kind) as i8;
            if !(-1..=3).contains(&old) {
                return Err("nivel del astillero fuera de rango".into());
            }
            let next = old + 1;
            if next < 4 && exp > REQUIREMENTS[kind][next as usize] * 2800 {
                if old < 0 && kind == 3 && town.u32(0x2c8) == 0 && town.u32(0x76c) & 0x20000 != 0 {
                    notices.push(u8::try_from(city).map_err(|_| "índice de ciudad fuera de rango")?)
                }
                town.set_u8(0x824 + kind, next as u8)
            }
        }
    }
    towns.clone_from_slice(&staged);
    Ok(notices)
}
