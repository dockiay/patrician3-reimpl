//! Selección de trabajos sobre la muralla en `0x51D870`, día del año & 7 == 0.
use crate::{
    buildings::Site,
    pump::Operation,
    town::{off, Town},
};

pub trait WallPlannerHost {
    type Error;
    fn office(&mut self, index: u16) -> Result<(u16, u16), Self::Error>;
    fn merchant_kind(&mut self, index: u16) -> Result<u16, Self::Error>;
    fn invalid_wall_record(&mut self) -> Self::Error;
    fn invalid_office_chain(&mut self) -> Self::Error;
}

pub fn plan<H: WallPlannerHost>(
    town: &Town,
    site: &Site,
    merchant_count: u16,
    office_count: u16,
    host: &mut H,
) -> Result<Vec<Operation>, H::Error> {
    let head = town.u16(off::WALL_HEAD);
    if head >= town.u16(off::BUILDING_RECORD_COUNT) {
        return Ok(vec![]);
    }
    let Some(&kind) = site.records.get(usize::from(head) * 8 + 3) else {
        return Err(host.invalid_wall_record());
    };
    let mayor = u16::from(town.u8(off::MAYOR_ID));
    let mut owner = if mayor < merchant_count { u32::from(mayor) } else { 255 };
    if mayor >= merchant_count {
        let mut office = town.u16(off::FIRST_OFFICE);
        let mut remaining = usize::from(office_count);
        while office < office_count {
            if remaining == 0 {
                return Err(host.invalid_office_chain());
            }
            remaining -= 1;
            let (candidate, next) = host.office(office)?;
            if host.merchant_kind(candidate)? & 4 != 0 {
                owner = u32::from(candidate);
                break;
            }
            office = next;
        }
    }
    let mode = u32::from(kind != 0x36);
    Ok(vec![
        Operation([0x24, owner, u32::from(town.index()), mode, 0]);
        if mode == 0 { 12 } else { 1 }
    ])
}
