//! Rama de edificios cívicos del planificador diario `0x0051D870` (día & 7 == 4).
use crate::buildings::tables::{BRICK, IRON, TIMBER};
use crate::pump::Operation;

/// Fronteras que el original llama en orden; el aleatorio debe ser el CRT compartido.
pub trait CivicHost {
    type Error;
    fn point_zero_valid(&mut self) -> Result<bool, Self::Error>;
    fn place_building(&mut self, kind: u8) -> Result<u32, Self::Error>;
    fn random(&mut self) -> Result<u32, Self::Error>;
    fn enqueue(&mut self, operation: Operation) -> Result<(), Self::Error>;
}

pub fn plan<H: CivicHost>(flags: u32, town: u8, host: &mut H) -> Result<(), H::Error> {
    if flags & 0x200 != 0 {
        return Ok(());
    }
    let kind = if flags & 1 == 0 {
        0x23
    } else if flags & 2 == 0 {
        0x20
    } else if flags & 4 == 0 {
        3
    } else if flags & 8 == 0 {
        0x21
    } else if flags & 0x10 == 0 {
        if host.point_zero_valid()? {
            return host.enqueue(Operation([0x23, 0, u32::from(town), 0, 0]));
        }
        let scale = ((host.random()? & 31) + 0x25) * 200 / 32;
        return host.enqueue(Operation([
            0x27,
            u32::from(town),
            u32::from(BRICK[0x37]) * scale * 10,
            u32::from(TIMBER[0x37]) * scale * 10,
            u32::from(IRON[0x37]) * scale,
        ]));
    } else if flags & 0x20 == 0 {
        0x24
    } else if flags & 0x40 == 0 {
        1
    } else {
        0x1f
    };
    if host.place_building(kind)? != 0 {
        return Ok(());
    }
    let scale = ((host.random()? & 15) + 0x18) * 200 / 32;
    let kind = usize::from(kind);
    host.enqueue(Operation([
        0x27,
        u32::from(town),
        u32::from(BRICK[kind]) * scale * 10,
        u32::from(TIMBER[kind]) * scale * 10,
        u32::from(IRON[kind]) * scale,
    ]))
}
