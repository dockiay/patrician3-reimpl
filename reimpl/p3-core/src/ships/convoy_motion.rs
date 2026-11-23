//! `Convoy::set_motion` (0x0050A240): escribe velocidad (`+0x30`, segundo argumento), instante de llegada (`+0x48`, primer argumento) y rumbo (`+1`) en cada barco de la lista del convoy.
//! Fixture: `oracle/x86_convoy_motion.py`.

use super::r16;
use super::remove::{CONVOY_SIZE, SHIP_SIZE};

/// `manager+4` y `manager+8` son los punteros a barcos y convoyes del original; aquí llegan como cortes. Sale sin hacer
/// nada si el convoy o su primer barco no existen (`>=` el recuento); la lista no se protege de ciclos en el original y
/// aquí devuelve `Err` tras más pasos que barcos.
pub fn set_convoy_motion(
    manager: &[u8],
    ships: &mut [u8],
    convoys: &[u8],
    time: u32,
    speed: u32,
    convoy: u32,
    heading: u8,
) -> Result<(), &'static str> {
    if convoy >= u32::from(r16(manager, 0xF6)) {
        return Ok(());
    }
    let ship_count = u32::from(r16(manager, 0xF4));
    let mut ship = u32::from(r16(convoys, convoy as usize * CONVOY_SIZE + 0xA));
    if ship >= ship_count {
        return Ok(());
    }
    let mut steps = 0u32;
    loop {
        let base = ship as usize * SHIP_SIZE;
        let next = r16(ships, base + 6);
        ships[base + 0x30..base + 0x34].copy_from_slice(&speed.to_le_bytes());
        ships[base + 0x48..base + 0x4C].copy_from_slice(&time.to_le_bytes());
        ships[base + 1] = heading;
        ship = u32::from(next);
        if ship >= ship_count {
            return Ok(());
        }
        steps += 1;
        if steps > ship_count {
            return Err("lista de barcos cíclica: el original no termina");
        }
    }
}
