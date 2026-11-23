//! Selección diaria de un pozo de `0x51D870` (día & 7 igual a 6).
use crate::{
    buildings::Site,
    pump::Operation,
    town::{off, Town},
};

pub fn plan(t: &Town, site: &Site, merchant_count: u16) -> Result<Option<Operation>, &'static str> {
    if t.flags() & 0x40 != 0 || i32::from(t.u8(0x789)) * 500 >= t.i32(off::CITIZENS_TOTAL) {
        return Ok(None);
    }
    let rows = t.i32(off::PLAN_ROWS);
    let cols = t.i32(off::PLAN_STRIDE);
    if rows <= 4
        || cols <= 4
        || rows > 255
        || cols > 255
        || site.cells.len() < (rows * cols) as usize
        || i32::from(t.u8(0x800)) >= rows
        || i32::from(t.u8(0x801)) >= cols
    {
        return Err("plano de pozo inválido");
    }
    let (row, col) =
        std::panic::catch_unwind(|| site.pick_new_well_site(t)).map_err(|_| "búsqueda de pozo fuera del plano")?;
    let mayor = t.u8(off::MAYOR_ID);
    let owner = if u16::from(mayor) < merchant_count {
        mayor
    } else {
        t.index()
    };
    Ok(Some(Operation([
        0x22,
        u32::from_le_bytes([owner, t.index(), row, col]),
        0x28,
        0,
        0,
    ])))
}
