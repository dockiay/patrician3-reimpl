//! Adaptador de la rama cívica al plano y al generador CRT explícito del simulador.
use p3_core::{buildings::Site, economy::civic_planner::{self, CivicHost}, pump::Operation, rng::CrtRand, town::Town};

struct Host<'a> {
    town: &'a Town,
    site: &'a Site,
    merchants: u16,
    rng: &'a mut Option<CrtRand>,
    operations: Vec<Operation>,
}
impl CivicHost for Host<'_> {
    type Error = String;
    fn point_zero_valid(&mut self) -> Result<bool, String> { Ok(Site::is_plan_point_valid(self.town, 0)) }
    fn place_building(&mut self, kind: u8) -> Result<u32, String> {
        let (result, op) = crate::lender::plan_kind(self.town, self.site, self.merchants, kind)?;
        if let Some(op) = op { self.operations.push(op); }
        Ok(result)
    }
    fn random(&mut self) -> Result<u32, String> {
        Ok(self.rng.as_mut().ok_or("estado CRT compartido no suministrado")?.rand() as u32)
    }
    fn enqueue(&mut self, op: Operation) -> Result<(), String> { self.operations.push(op); Ok(()) }
}

/// No modifica la semilla ni publica operaciones si alguna frontera falla.
pub fn plan(town: &Town, site: &Site, merchants: u16, rng: &mut Option<CrtRand>) -> Result<Vec<Operation>, String> {
    let mut staged = *rng;
    let mut host = Host { town, site, merchants, rng: &mut staged, operations: vec![] };
    civic_planner::plan(town.more_flags(), town.index(), &mut host)?;
    let operations = host.operations;
    *rng = staged;
    Ok(operations)
}

/// Ajuste de almacén de la operación 0x27 (`0x00535BBC`).
pub fn supply(town: &mut Town, op: &Operation) -> Result<(), String> {
    if op.0[0] != 0x27 || op.0[1] as u8 != town.index() { return Err("suministro de ciudad incompatible".into()); }
    if (op.0[1] as i32) < 0 {
        town.set_i32(0x48, town.i32(0x48).wrapping_add(i32::from(p3_core::buildings::tables::EXTRA_COST[1]) * 2000));
    }
    for (offset, amount) in [(0x50,op.0[2]),(0x30,op.0[3]),(0x34,op.0[4])] {
        town.set_i32(offset, town.i32(offset).wrapping_add(amount as i32));
    }
    Ok(())
}

/// Despacha 0x23 sobre copias: un plano inválido no deja una muralla parcialmente escrita.
pub fn wall(town: &mut Town, site: &mut Site, merchants: u16, op: &Operation) -> Result<(), String> {
    if op.0[0]!=0x23 || op.0[2]!=u32::from(town.index()) || op.0[1]>3 { return Err("operación de muralla incompatible".into()); }
    crate::lender::validate(town,site,merchants)?;
    let mut staged_town=town.clone();let mut staged_site=site.clone();
    let result=std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| staged_site.place_town_defense_pieces(&mut staged_town,op.0[1],&p3_core::buildings::walls::DEFAULT_KINDS)));
    if result.is_err(){return Err("trazado de muralla fuera del plano o del pool".into())}
    *town=staged_town;*site=staged_site;Ok(())
}
