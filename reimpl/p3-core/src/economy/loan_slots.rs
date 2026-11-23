//! Solicitante de préstamo, `0x004E8CA4`.
use crate::rng::CrtRand;

/// Genera los seis bytes de una operación 0x7B. Los bancos no pueden estar vacíos.
pub fn applicant(population: [u32; 2], slot: u32, names: (u16, u16), rng: &mut CrtRand) -> Option<[u8; 6]> {
    if names.0 == 0 || names.1 == 0 {
        return None;
    }
    let first = (rng.rand() as u32 % u32::from(names.0)) as u8;
    let last = (rng.rand() as u32 % u32::from(names.1)) as u8;
    let base = ((population[1] as i32 >> 1) as u16)
        .wrapping_add(population[0] as u16)
        .min(2000);
    let (amount, weeks) = match slot {
        0 => (
            base.wrapping_add((rng.rand() as u16 & 0x3ff).wrapping_add(2000)),
            (rng.rand() as u8 & 3) + 6,
        ),
        1 => (
            base.wrapping_shl(1)
                .wrapping_add((rng.rand() as u16 & 0x7ff).wrapping_add(4000)),
            (rng.rand() as u8 & 7) + 12,
        ),
        2 => (
            base.wrapping_shl(2)
                .wrapping_add((rng.rand() as u16 & 0xfff).wrapping_add(8000)),
            (rng.rand() as u8 & 15) + 24,
        ),
        3..=5 => {
            let a = base
                .wrapping_add(rng.rand() as u16 & 0x1000)
                .wrapping_add((rng.rand() as u16 & 0x3fff).wrapping_add(2000));
            (a, (rng.rand() as u8 & 31) + 6)
        }
        _ => (0, 0),
    };
    let level = (i32::from(amount) / 5000 + rng.rand() % 5 - 2).clamp(0, 7) as u8;
    let [lo, hi] = amount.to_le_bytes();
    Some([level, first, last, weeks, lo, hi])
}

/// `0x004E8E24`: actualiza el vencimiento y encola solicitudes 0x7B.
/// No aplica las operaciones: el original las procesa después en el bombeo.
pub fn refresh(
    task: &mut crate::scheduler::TaskRecord,
    towns: &[crate::town::Town],
    merchant_count: u16,
    network_client: bool,
    world_8a: u8,
    queued: u16,
    names: Option<(u16, u16)>,
    rng: &mut Option<CrtRand>,
) -> Result<Vec<crate::pump::Operation>, &'static str> {
    use crate::pump::Operation;
    if towns.len() > 40 {
        return Err("demasiadas ciudades");
    }
    let mut staged = *task;
    let mut random = *rng;
    let occupied: usize = towns
        .iter()
        .map(|town| (0..6).filter(|&i| town.u16(0x9ca + 6 * i) != 0).count())
        .sum();
    let mut budget = 52i32 - i32::from(queued);
    let (extra, limit) = if occupied > 90 {
        (0x100, 12)
    } else if occupied > 60 {
        (0xc0, 8)
    } else {
        (0x90, 120 - occupied as i32)
    };
    staged.due = staged.due.wrapping_add(extra + occupied as u32);
    budget = budget.min(limit);
    let mut result = Vec::new();
    if usize::from(merchant_count) > towns.len() {
        if network_client {
            staged.set_u8_at(8, 0);
        } else if world_8a == 0 && budget > 0 {
            if towns.is_empty() {
                return Err("sin ciudades para solicitudes");
            }
            let (first, last) = names.ok_or("banco de nombres ausente")?;
            if first == 0 || last == 0 {
                return Err("banco de nombres vacío");
            }
            let generator = random.as_mut().ok_or("estado CRT ausente")?;
            let mut idle = 0usize;
            while budget > 0 {
                let mut city = staged.u8_at(8).wrapping_add(1);
                if usize::from(city) >= towns.len() {
                    city = 0;
                }
                staged.set_u8_at(8, city);
                let town = &towns[usize::from(city)];
                let mut slots = Vec::with_capacity(2);
                if occupied <= 60 {
                    if let Some(slot) = (0..3).find(|&i| town.u16(0x9ca + 6 * i) == 0) {
                        slots.push(slot);
                    }
                    if let Some(slot) = (3..6).find(|&i| town.u16(0x9ca + 6 * i) == 0) {
                        slots.push(slot);
                    }
                } else {
                    slots.push(((generator.rand() % 0x600) as usize & 0x700) >> 8);
                }
                if slots.is_empty() {
                    idle += 1;
                    if idle >= towns.len() {
                        return Err("ninguna ranura disponible");
                    }
                } else {
                    idle = 0;
                }
                for slot in slots {
                    let bytes = applicant(
                        [town.u32(0x2d8), town.u32(0x2dc)],
                        slot as u32,
                        (first, last),
                        generator,
                    )
                    .expect("nombres validados");
                    let target = u32::from(city) | (slot as u32) << 8;
                    result.push(Operation([
                        0x7b,
                        u32::from_le_bytes(bytes[..4].try_into().unwrap()),
                        u32::from(u16::from_le_bytes([bytes[4], bytes[5]])),
                        0,
                        target,
                    ]));
                    budget -= 1;
                }
            }
        }
    }
    *task = staged;
    *rng = random;
    Ok(result)
}

/// `0x005365E3`: copia los seis bytes de la operación a la ranura, si existe.
pub fn apply(towns: &mut [crate::town::Town], operation: crate::pump::Operation) {
    let city = (operation.0[4] & 0xff) as usize;
    let slot = ((operation.0[4] >> 8) & 7) as usize;
    if let Some(town) = towns.get_mut(city) {
        if slot < 6 {
            let offset = 0x9c6 + 6 * slot;
            town.0[offset..offset + 4].copy_from_slice(&operation.0[1].to_le_bytes());
            town.0[offset + 4..offset + 6].copy_from_slice(&(operation.0[2] as u16).to_le_bytes());
        }
    }
}
