//! Transacción de la fase final de `st_expedition_return` (004E6A54).
//! La implementación concreta del comercio vive en un anfitrión explícito.
use crate::{
    expedition_finish_stage,
    expedition_return_letter::{self, ExpeditionLetter, ExpeditionLetterTexts, Incident},
    expedition_visit::{self, CenterHost, VisitInput, VisitState},
    navigation::LiveVoyage,
};
use p3_core::{economy::expedition_damage, route::finder::RoutePlanner, town::Town};

/// El anfitrión debe transportar las modificaciones de comercio a la flota y
/// los dos textos construidos durante las visitas. Ninguna salida se publica
/// hasta que la transacción completa termine.
pub trait ReturnHost: CenterHost {
    fn navigation_after_visits(&self, before: &LiveVoyage) -> Result<LiveVoyage, String>;
    fn sales_text(&self) -> &[u8];
    fn contracts_text(&self) -> &[u8];
    fn no_contract_suffix(&self) -> &[u8] {
        &[]
    }
}

/// Banco vivo de nombres. Los índices proceden del capitán posterior al cierre.
pub trait CaptainNames {
    fn names(&self, given: u8, family: u8) -> Result<(&[u8], &[u8]), String>;
}
pub struct LetterContext<'a> {
    pub texts: &'a ExpeditionLetterTexts,
    pub ship: &'a [u8],
    pub town: &'a [u8],
    pub captain_names: Option<&'a dyn CaptainNames>,
}
pub struct Inputs<'a, H> {
    pub nav: &'a LiveVoyage,
    pub towns: &'a [Town],
    pub state: &'a VisitState,
    pub host: &'a H,
    pub visit: VisitInput<'a>,
    pub planner: Option<&'a RoutePlanner>,
    pub tick: u32,
    pub due: u32,
    pub start: u32,
    pub convoy: u16,
    pub difficulty: u8,
    pub renown_delta: i16,
    pub pool_limit: u16,
    /// Datos de nombres anteriores al cierre; solo necesarios para carta humana.
    pub letter: Option<LetterContext<'a>>,
}
pub struct Staged<H> {
    pub state: VisitState,
    pub host: H,
    pub finish: expedition_finish_stage::Staged,
    pub damage: expedition_damage::DamageOutcome,
    pub letter_delivered: bool,
}

/// 4E78xx consulta el líder en el convoy *después* de 502B20, incluso
/// si este ha cambiado de líder o ha liberado el registro del convoy.
pub fn captain_after_finish<'a>(
    nav: &LiveVoyage,
    convoy: u16,
    names: Option<&'a dyn CaptainNames>,
) -> Result<Option<(&'a [u8], &'a [u8])>, String> {
    let snap = &nav.snapshot;
    let manager = &snap.manager;
    if manager.len() < 0xf6 {
        return Err("gestor de capitanes truncado".into());
    }
    let ships = usize::from(u16::from_le_bytes([manager[0xf4], manager[0xf5]]));
    let captains = usize::from(u16::from_le_bytes([manager[0xf2], manager[0xf3]]));
    let at = usize::from(convoy) * 0x3c;
    let cv = snap
        .convoys
        .get(at..at + 0x3c)
        .ok_or("convoy de expedición ausente después del cierre")?;
    let leader = usize::from(u16::from_le_bytes([cv[0x10], cv[0x11]]));
    if leader >= ships {
        return Ok(None);
    }
    let ship = snap
        .ships
        .get(leader * 0x180..leader * 0x180 + 0x180)
        .ok_or("barco líder truncado")?;
    let captain = usize::from(u16::from_le_bytes([ship[0x42], ship[0x43]]));
    if captain >= captains {
        return Ok(None);
    }
    let cap = snap
        .captains
        .get(captain * 16..captain * 16 + 16)
        .ok_or("capitán truncado")?;
    let bank = names.ok_or("banco vivo de nombres de capitán ausente")?;
    Ok(Some(bank.names(cap[2], cap[3])?))
}

/// La transacción no modifica entradas incluso si falla un callback o falta
/// algún recurso; el llamante debe publicar estado, flota y tarea juntos.
pub fn prepare<H: ReturnHost>(i: Inputs<'_, H>) -> Result<Staged<H>, String> {
    let offset = usize::from(i.convoy) * 0x3c;
    let convoy = i
        .nav
        .snapshot
        .convoys
        .get(offset..offset + 0x3c)
        .ok_or("convoy de expedición inexistente")?;
    let initial_money = u32::from_le_bytes(convoy[0x20..0x24].try_into().unwrap());
    let word = |p| u16::from_le_bytes([convoy[p], convoy[p + 1]]);
    let points = std::array::from_fn(|n| (word(0x24 + n * 4), word(0x26 + n * 4)));
    if i.visit.owner != convoy[0] || i.visit.waypoints != points || i.visit.mode != word(0x38) || i.visit.tick != i.tick
    {
        return Err("contexto de expedición incoherente con el convoy o reloj".into());
    }
    let mut visits = expedition_visit::prepare(i.state, i.host, i.visit)?;
    let mut nav = visits.host.navigation_after_visits(i.nav)?;
    let convoy = nav
        .snapshot
        .convoys
        .get(offset..offset + 0x3c)
        .ok_or("convoy ausente tras visitas")?;
    let (ships, convoy, damage) =
        expedition_damage::prepare(&nav.snapshot.ships, convoy, i.due, i.start, i.tick, i.difficulty)?;
    let returned_money = u32::from_le_bytes(convoy[0x20..0x24].try_into().unwrap());
    let count = ships.len() / 0x180;
    let mut member = usize::from(u16::from_le_bytes([convoy[0xa], convoy[0xb]]));
    let mut alive = false;
    while member < count {
        let at = member * 0x180;
        alive |= i32::from_le_bytes(ships[at + 0x18..at + 0x1c].try_into().unwrap()) > 0;
        member = usize::from(u16::from_le_bytes([ships[at + 6], ships[at + 7]]));
    }
    nav.snapshot.ships = ships;
    nav.snapshot.convoys[offset..offset + 0x3c].copy_from_slice(&convoy);
    let mut finish = expedition_finish_stage::prepare(expedition_finish_stage::Inputs {
        nav: &nav,
        towns: i.towns,
        merchants: &visits.state.merchants,
        message_pool: &visits.state.message_pool,
        planner: i.planner,
        calendar: i.visit.calendar,
        tick: i.tick,
        start: i.start,
        convoy: i.convoy,
        renown_delta: i.renown_delta,
        pool_limit: i.pool_limit,
    })?;
    let owner = usize::from(i.visit.owner);
    let mut delivered = false;
    if alive
        && owner < usize::from(i.visit.merchant_count)
        && owner < finish.merchants.len()
        && finish.merchants[owner].bytes.get(8..10) == Some(&[0, 0][..])
    {
        let context = i.letter.ok_or("textos y nombres de carta de expedición ausentes")?;
        let merchant = &mut finish.merchants[owner];
        let hometown = *merchant.bytes.get(0x19).ok_or("comerciante truncado")?;
        let incident = if damage.storm {
            Incident::Storm
        } else if damage.attack {
            Incident::Attack { stolen: damage.stolen }
        } else {
            Incident::None
        };
        let captain = if initial_money != 0 || returned_money != 0 {
            captain_after_finish(&finish.nav, i.convoy, context.captain_names)?
        } else {
            None
        };
        let mut contracts = visits.host.contracts_text().to_vec();
        if !visits.has_contract {
            contracts.extend_from_slice(visits.host.no_contract_suffix());
        }
        let letter = ExpeditionLetter {
            ship: context.ship,
            town: context.town,
            discovered: &visits.discovered,
            unmatched: visits.unmatched,
            sales: visits.host.sales_text(),
            contracts: &contracts,
            incident,
            captain,
            initial_money,
            returned_money,
        };
        delivered = expedition_return_letter::deliver(
            context.texts,
            &letter,
            &mut finish.message_pool,
            merchant,
            (i.visit.calendar.day, i.visit.calendar.month, i.visit.calendar.year),
            hometown,
        )?;
    }
    visits.state.merchants = finish.merchants.clone();
    visits.state.message_pool = finish.message_pool.clone();
    Ok(Staged {
        state: visits.state,
        host: visits.host,
        finish,
        damage,
        letter_delivered: delivered,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use p3_save::ships::{NavigationLists, NavigationSnapshot};
    struct Names;
    impl CaptainNames for Names {
        fn names(&self, given: u8, family: u8) -> Result<(&[u8], &[u8]), String> {
            match (given, family) {
                (0x12, 0x34) => Ok((b"old", b"captain")),
                (0x56, 0x78) => Ok((b"new", b"captain")),
                _ => Err("nombre no conocido".into()),
            }
        }
    }
    #[test]
    fn captain_name_uses_leader_after_finish() {
        let mut manager = vec![0; 0x110];
        manager[0xf2..0xf4].copy_from_slice(&2u16.to_le_bytes());
        manager[0xf4..0xf6].copy_from_slice(&2u16.to_le_bytes());
        let mut ships = vec![0; 2 * 0x180];
        ships[0x42..0x44].copy_from_slice(&0u16.to_le_bytes());
        ships[0x180 + 0x42..0x180 + 0x44].copy_from_slice(&1u16.to_le_bytes());
        let mut captains = vec![0; 32];
        captains[2..4].copy_from_slice(&[0x12, 0x34]);
        captains[18..20].copy_from_slice(&[0x56, 0x78]);
        let mut convoys = vec![0; 0x3c];
        let snap = NavigationSnapshot {
            manager,
            ships,
            captains,
            convoys: convoys.clone(),
            routes: vec![],
            lists: NavigationLists {
                docked_head: 0,
                sea_head: 0,
                docked: vec![],
                sea: vec![],
            },
        };
        let before = LiveVoyage::idle(snap, vec![], 0);
        let mut after = before.clone();
        convoys[0x10..0x12].copy_from_slice(&1u16.to_le_bytes());
        after.snapshot.convoys = convoys;
        assert_eq!(
            captain_after_finish(&before, 0, Some(&Names)).unwrap(),
            Some((&b"old"[..], &b"captain"[..]))
        );
        assert_eq!(
            captain_after_finish(&after, 0, Some(&Names)).unwrap(),
            Some((&b"new"[..], &b"captain"[..]))
        );
        assert!(captain_after_finish(&after, 0, None).is_err());
        after.snapshot.ships[0x180 + 0x42..0x180 + 0x44].fill(0xff);
        assert_eq!(captain_after_finish(&after, 0, None).unwrap(), None);
    }
}
