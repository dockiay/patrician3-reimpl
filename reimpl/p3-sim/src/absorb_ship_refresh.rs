//! `ship_upgrade_levels` (0x0051A750), used immediately during convoy absorption.
//! The original reads only the low byte of `request`. Levels outside 0..=2
//! would index beyond the documented weapon-slot table and are rejected.
//! Weapon slots and cargo use the existing ports of 0x5181D0 and 0x5182B0.

/// Apply upgrades to a live ship image, including slots and cargo. Validation
/// precedes mutation. No host callback, UI effect or allocation is omitted.
pub fn refresh(ship: &mut [u8], request: u32) -> Result<(), &'static str> {
    if ship.len() < 0x180 {
        return Err("barco de mejora truncado");
    }
    let requested = request as u8;
    if requested == 0 {
        return Ok(());
    }
    let level = ship[0xf];
    if level > 2 {
        return Err("nivel de mejora fuera de la tabla verificada");
    }
    let increment = (2 - level).min(requested);
    if increment == 0 {
        return Ok(());
    }
    let coefficient = [3u32, 5, 8, 10][usize::from(ship[0xe] & 3)];
    let points = u32::from(increment) * coefficient;
    let capacity = u32::from_le_bytes(ship[0x10..0x14].try_into().unwrap());
    let health = u32::from_le_bytes(ship[0x14..0x18].try_into().unwrap());
    let flags = u16::from_le_bytes(ship[0x3c..0x3e].try_into().unwrap());
    ship[0xf] = level + increment;
    ship[0x10..0x14].copy_from_slice(&capacity.wrapping_sub(points * 2000).to_le_bytes());
    ship[0x14..0x18].copy_from_slice(&health.wrapping_add(points * 2800).to_le_bytes());
    ship[0x3c..0x3e].copy_from_slice(&(flags | 0x104).to_le_bytes());
    p3_core::ships::factory::recompute_weapon_slots(ship).expect("longitud y nivel comprobados");
    p3_core::ships::cargo::recompute_load(ship);
    Ok(())
}
