//! Absorber un convoy en un objeto del puerto (`0x00514B80`, `this` = objeto): pasa cada barco de la lista del convoy
//! (`convoy+0xA`, enlace `ship+6`) a la lista del objeto (`obj+0x12`), vacía las 24 cantidades de carga del barco
//! (`ship+0x54`, escaladas por la tabla de floats 0x673A18 y sumadas), libera el convoy y suma `suma/1000` a `obj+0x16`.
//! Los avisos a la interfaz y a los demás objetos son ganchos del anfitrión. Fixture: `oracle/x86_convoy_absorb.py`.

use super::remove::{CONVOY_SIZE, SHIP_SIZE};
use super::{r16, r32, w16, w32};
use crate::numeric::ftol_i32;

/// Tabla de floats de 0x00673A18 (24 entradas, bits exactos).
pub const CARGO_SCALE_BITS: [u32; 24] = [
    0x3D6147AF, 0x3EF50482, 0x3E6154CA, 0x3E322D0E, 0x3E11EB85, 0x3F0CCCCD, 0x3FB33333, 0x3F8CCCCD, 0x3F845A1D,
    0x40587AE1, 0x3ED33333, 0x3CE147AF, 0x3FA39581, 0x3F8F5C29, 0x3EE147AF, 0x3E8E5604, 0x3EE147AF, 0x3E6147AF,
    0x3F5AE147, 0x3D236E30, 0x41200000, 0x41000000, 0x41400000, 0x41600000,
];

/// Llamadas que el original hace fuera de las listas.
pub trait AbsorbHost {
    /// 0x00494A80 sobre el puntero de interfaz `[0x6CE6D0]` (si no es nulo), con el argumento recibido.
    fn ui(&mut self, ui: u32, argument: u32);
    /// 0x0051A750 con `1` sobre un barco de tipo menor que 2.
    fn ship_refresh(&mut self, ship: u16);
    /// El anfitrión vivo aplica la mejora antes de continuar con ese barco.
    fn ship_refresh_with_image(&mut self, ship: u16, image: &mut [u8]) {
        let _ = image;
        self.ship_refresh(ship);
    }
    /// 0x0054AA70 con el bloque de cinco dwords `{0x12, barco del convoy, 0x32, 0x32, 0}`.
    fn event(&mut self, block: [u32; 5]);
    /// `release_convoy` (0x0050A3D0) del gestor.
    fn release(&mut self, convoy: u16);
    /// 0x00515DA0 sobre el objeto, al final.
    fn object_changed(&mut self);
}

/// Globales y tablas que lee 0x514B80.
pub struct AbsorbWorld<'a> {
    pub ships: &'a mut [u8],
    pub convoys: &'a mut [u8],
    /// Word 0x6DD894.
    pub ship_count: u16,
    /// Word 0x6DD892.
    pub captain_count: u16,
    /// Dword 0x6CE6D0.
    pub ui: u32,
    /// Dword 0x6DF31C.
    pub suppress_events: u32,
}

pub fn absorb_convoy(
    object: &mut [u8],
    w: &mut AbsorbWorld,
    argument: u32,
    host: &mut dyn AbsorbHost,
) -> Result<(), &'static str> {
    let convoy = (argument & 0xFFFF) as usize;
    let rec = convoy * CONVOY_SIZE;
    if w.ui != 0 {
        host.ui(w.ui, argument);
    }
    let mut sum = 0i32;
    let mut index = r16(w.convoys, rec + 0xA);
    let mut steps = 0u32;
    if index < w.ship_count {
        loop {
            let b = index as usize * SHIP_SIZE;
            let next = r16(w.ships, b + 6);
            w16(w.ships, b + 8, 0xFFFF);
            w16(w.convoys, rec + 0xA, next);
            let head = r16(object, 0x12);
            w16(w.ships, b + 6, head);
            w16(object, 0x12, index);
            w32(w.ships, b + 0x1C, 0x7FFF_FFFF);
            w32(w.ships, b + 0x20, 0x7FFF_FFFF);
            for (k, bits) in CARGO_SCALE_BITS.iter().enumerate() {
                let quantity = r32(w.ships, b + 0x54 + 4 * k);
                let scaled = f64::from(quantity as f32) * f64::from(f32::from_bits(*bits));
                sum = sum.wrapping_add(ftol_i32(scaled));
                w32(w.ships, b + 0x54 + 4 * k, 0);
            }
            if r32(w.ships, b + 0x14) >> 1 > r32(w.ships, b + 0x18) {
                w.ships[b + 0x15D] = 0xFF;
            }
            if w.ships[b + 0xF] < 2 {
                host.ship_refresh_with_image(index, &mut w.ships[b..b + SHIP_SIZE]);
            }
            if r16(w.ships, b + 0x42) < w.captain_count && w.suppress_events == 0 {
                let reference = r16(w.convoys, rec + 0x10);
                if reference < w.ship_count && r16(w.ships, reference as usize * SHIP_SIZE + 0x42) < w.captain_count {
                    host.event([0x12, u32::from(reference), 0x32, 0x32, 0]);
                }
            }
            index = r16(w.convoys, rec + 0xA);
            if index >= w.ship_count {
                break;
            }
            steps += 1;
            if steps > 4096 {
                return Err("lista de barcos cíclica: el original no termina");
            }
        }
    }
    host.release(convoy as u16);
    let thousand = ((i64::from(sum) * 0x1062_4DD3) >> 32) as i32 >> 6;
    let adjusted = thousand.wrapping_add(((thousand as u32) >> 31) as i32);
    let value = r16(object, 0x16).wrapping_add(adjusted as u16);
    w16(object, 0x16, value);
    host.object_changed();
    Ok(())
}
