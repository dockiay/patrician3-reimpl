//! Decisiones de vivienda de `0x51D870` (días & 7: 1, 3 y 5).
use crate::{
    buildings::Site,
    town::{off, Town},
};

pub trait HousingPlannerHost {
    type Error;
    fn place(&mut self, kind: u8) -> Result<u32, Self::Error>;
    fn expand_walls(&mut self) -> Result<(), Self::Error>;
    fn invalid_construction_chain(&mut self) -> Self::Error;
}
fn high(a: u32, b: u32) -> u32 {
    ((u64::from(a) * u64::from(b)) >> 32) as u32
}

pub fn plan<H: HousingPlannerHost>(town: &Town, site: &Site, day: u16, host: &mut H) -> Result<(), H::Error> {
    let (population, capacity, kind) = match day & 7 {
        1 => (0x2d8, 0x2f8, 0x15),
        3 => (0x2dc, 0x2fa, 0x18),
        5 => (0x2e0, 0x2fc, 0x1b),
        _ => return Ok(()),
    };
    let pop = town.u32(population);
    let hanse = town.flags() & 0x40 != 0;
    let x = pop.wrapping_mul(100);
    let required = match (day & 7, hanse) {
        (1, true) => pop,
        (1, false) => {
            let h = high(x, 0x51d07eaf);
            ((x.wrapping_sub(h) >> 1).wrapping_add(h)) >> 6
        }
        (3, true) => high(x, 0xa57eb503) >> 6,
        (3, false) => high(x, 0xaaaaaaab) >> 6,
        (5, true) => high(x, 0x5397829d) >> 5,
        (5, false) => {
            let h = high(x, 0x58ed2309);
            ((x.wrapping_sub(h) >> 1).wrapping_add(h)) >> 6
        }
        _ => unreachable!(),
    };
    if required <= u32::from(town.u16(capacity)) {
        return Ok(());
    }
    let count = town.u16(off::BUILDING_RECORD_COUNT);
    let mut current = town.u16(off::CONSTRUCTION_HEAD);
    let mut seen = vec![false; usize::from(count)];
    while current < count {
        let i = usize::from(current);
        let p = i * 8;
        if seen[i] || p + 8 > site.records.len() {
            return Err(host.invalid_construction_chain());
        }
        seen[i] = true;
        if site.records[p + 3] == kind {
            return Ok(());
        }
        current = u16::from_le_bytes([site.records[p + 4], site.records[p + 5]]);
    }
    if host.place(kind)? == 0x37 {
        host.expand_walls()?
    }
    Ok(())
}
