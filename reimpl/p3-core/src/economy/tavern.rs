//! Actualización diaria de la taberna (`0x00525F30`).
use super::world::EconomyWorld;
use crate::town::{off, Town};

/// El original recorre las oficinas sin comprobar ciclos; aquí se limita el
/// recorrido para evitar quedar bloqueado con un guardado mal formado.
pub fn daily_update(world: &EconomyWorld, town: &mut Town) {
    let unpaid = town.u8(0x835);
    let beer = town.stock(3);
    if beer > 0 && unpaid > 0 {
        town.set_u8(0x835, unpaid - 1);
    }
    if beer <= 0 && unpaid < 9 {
        town.set_u8(0x835, unpaid + 1);
    }

    let mut n = 0usize;
    let mut index = town.u16(off::FIRST_OFFICE);
    let cap = usize::from(world.office_count).min(world.offices.len());
    while usize::from(index) < cap && n < cap {
        n += 1;
        index = world.offices[usize::from(index)].next;
    }
    if n < 3 {
        town.set_u8(0x83c, 0);
    } else if matches!(town.u8(0x83c), 0 | 255) {
        // x87 guarda las conversiones y resultados intermedios como f32.
        let poor = (town.i32(0x2e4) as f32) as f64;
        let militia = f64::from(town.u8(0x998));
        let population = (town.i32(0x2d4) as f32) as f64;
        let value = (poor * f64::from(0.01f32) + 1.0 - militia * 500.0 / population) as f32;
        let active = if value >= 30.0 {
            true
        } else {
            let rate = (f64::from(value) * f64::from(f32::from_bits(0x3d088880))) as f32;
            let phase = (f64::from(world.day_of_month) * f64::from(rate)) as f32;
            let whole = if phase.is_finite() && phase >= i32::MIN as f32 && phase < 2147483648.0 {
                phase.trunc() as i32
            } else {
                i32::MIN
            };
            f64::from(phase) - f64::from(whole as f32) < f64::from(rate)
        };
        town.set_u8(0x83c, if active { 255 } else { 0 });
    }
    if u16::from(town.u8(0x83f)) >= world.merchant_count && town.u32(0x838) < world.tick.wrapping_add(0x900) {
        town.set_u8(0x83f, 0);
    }
    if matches!(town.u8(0x83e), 0 | 255) {
        let random = u32::from(town.index()).wrapping_add(u32::from(world.day_of_month)) ^ town.u32(0x2e0);
        town.set_u8(
            0x83e,
            if random & 3 == 0 && town.more_flags() & 0x100 != 0 {
                255
            } else {
                0
            },
        );
    }
}
