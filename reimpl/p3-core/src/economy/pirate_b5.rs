//! Validación y argumentos de la operación 0xB5 (execute_operation 0x535760).
use crate::pump::Operation;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PirateLaunchRequest {
    pub band: u8,
    pub limit: u16,
    pub loadout_initial: u16,
}
/// `present` corresponde a los cinco punteros de grupo del gestor naval.
/// `None` también cubre ranuras fuera del rango o grupos inexistentes.
pub fn decode(op: Operation, present: [bool; 5]) -> Option<PirateLaunchRequest> {
    if op.0[0] != 0xb5 {
        return None;
    }
    let band = usize::try_from(op.0[1]).ok()?;
    if band >= 5 || !present[band] {
        return None;
    }
    Some(PirateLaunchRequest {
        band: band as u8,
        limit: op.0[3] as u16,
        loadout_initial: op.0[2] as u16,
    })
}
