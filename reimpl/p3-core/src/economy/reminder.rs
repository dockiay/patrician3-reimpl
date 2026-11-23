//! Recordatorio diario de comerciante `0x004E7BB4`.
use crate::{ai::reputation::Merchant, rng::CrtRand, scheduler::TaskRecord};

/// Si el registro apunta justo después del vector, el original accede fuera
/// del límite; esa situación se rechaza y no se simula como válida.
pub fn advance(
    task: &mut TaskRecord,
    merchants: &[Merchant],
    count: u16,
    rng: &mut Option<CrtRand>,
) -> Result<(bool, Option<(u16, u8)>), String> {
    let owner = u16::from_le_bytes([task.u8_at(0xc), task.u8_at(0xd)]);
    if owner > count {
        return Ok((false, None));
    }
    if owner == count {
        return Err("comerciante de recordatorio fuera del vector".into());
    }
    let person = merchants
        .get(usize::from(owner))
        .ok_or("comerciante de recordatorio ausente")?;
    if person.bytes.len() < 0x1a {
        return Err("comerciante truncado".into());
    }
    if person.u16(8) != 0 {
        return Ok((false, None));
    }
    let mut staged = *task;
    let mut random = *rng;
    let last = staged.u32_at(8);
    let mut notice = None;
    if staged.due <= last.wrapping_add(0x100) {
        let generator = random.as_mut().ok_or("estado CRT ausente")?;
        staged.due = last.wrapping_add(0x100).wrapping_add((generator.rand() as u32) & 15)
    } else {
        notice = Some((owner, person.bytes[0x19]));
        staged.set_u32_at(8, staged.due);
        staged.due = staged.due.wrapping_add(0x100);
        if staged.due & 255 < 0xf0 {
            let generator = random.as_mut().ok_or("estado CRT ausente")?;
            staged.due = staged.due.wrapping_add((generator.rand() as u32) & 15)
        }
    }
    *task = staged;
    *rng = random;
    Ok((true, notice))
}
