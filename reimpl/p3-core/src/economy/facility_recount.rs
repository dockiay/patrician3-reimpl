//! Inicialización y ajuste de negocios usados por el recuento de oficina.
use crate::town::Town;
const PER_BUILDING: [u8; 21] = [
    50, 40, 30, 15, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 15, 15, 30,
];
/// `facility_set_building_count` (0x004D36E0). Requiere el tipo y la ciudad ya asignados.
pub fn set_building_count(facility: &mut [u8; 20], town: &Town, count: i32) -> Result<(), &'static str> {
    let kind = usize::from(facility[6]);
    if kind >= PER_BUILDING.len() {
        return Err("tipo de negocio fuera de tabla");
    }
    let base = i32::from(town.u16(0x848 + 16 * kind));
    let value = if count >= 9 {
        base + base / 10
    } else if count >= 6 {
        base * 106 / 100
    } else if count >= 3 {
        base * 103 / 100
    } else {
        base
    };
    facility[..4].copy_from_slice(&value.to_le_bytes());
    facility[12..14].copy_from_slice(&(i32::from(PER_BUILDING[kind]).wrapping_mul(count) as u16).to_le_bytes());
    Ok(())
}
/// `facility_init` (0x004D3490); los bytes no escritos se preservan.
pub fn init(facility: &mut [u8; 20], town: &Town, city: u8, kind: u8, count: i32) -> Result<(), &'static str> {
    if usize::from(kind) >= PER_BUILDING.len() {
        return Err("tipo de negocio fuera de tabla");
    }
    facility[..6].fill(0);
    facility[6] = kind;
    facility[7] = city;
    facility[8..10].copy_from_slice(&0xffffu16.to_le_bytes());
    facility[10..16].fill(0);
    facility[16..20].copy_from_slice(&1f32.to_le_bytes());
    set_building_count(facility, town, count)
}
