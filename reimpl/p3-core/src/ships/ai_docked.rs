//! Rama de parada de la IA atracada (0x00518B80).
use super::w16;
/// La bandera `ship+0x136 & 0x20` fuerza espera y desembarca tripulación.
/// Devuelve false para las demás ramas, que requieren comercio e IA de destino.
pub fn stop_flagged_ship(ship: &mut [u8]) -> Option<bool> {
    if ship.len() != 0x180 {
        return None;
    }
    if ship[0x136] & 0x20 == 0 {
        return Some(false);
    }
    w16(ship, 0x134, 1);
    w16(ship, 0x138, 0x80);
    w16(ship, 0x40, 0);
    Some(true)
}
