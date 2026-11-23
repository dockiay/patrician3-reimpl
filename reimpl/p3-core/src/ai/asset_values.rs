//! Valor de barcos y coste de tripulación del día de comerciante C10.

/// Coste íntegro de cada uno de los cuatro cascos comerciales, calculado por
/// `calc_ship_current_value` (0x52A7D0) con las tablas del ejecutable inglés.
const HULL_BASE_VALUE: [i64; 4] = [13208, 26925, 24298, 37700];

/// Valor actual del casco; rechaza los tipos y divisores no acreditados.
pub fn ship_current_value(kind: u8, maximum: i32, current: i32) -> Result<u32, &'static str> {
    let base = *HULL_BASE_VALUE
        .get(usize::from(kind))
        .ok_or("tipo de casco no admitido")?;
    if maximum <= 0 || current < 0 {
        return Err("estado del casco no admitido");
    }
    Ok((base.wrapping_mul(i64::from(current)) / i64::from(maximum)) as u32)
}

/// `ship_get_daily_crew_cost` (0x517F70); salarios por índice de capitán.
pub fn ship_daily_crew_cost(sailors: u16, captain: u16, captain_wages: &[u16]) -> u32 {
    let base = u32::from(sailors) * 2;
    match captain_wages.get(usize::from(captain)) {
        Some(wage) => base + u32::from(*wage),
        None => base + if sailors != 0 { 2 } else { 0 },
    }
}

/// `scheduled_tasks_sum_merchant_debts` (0x4D9080): devuelve deuda neta y
/// cantidad adeudada al comerciante. Rechaza una lista truncada o cíclica.
pub fn sum_merchant_debts(scheduler: &crate::scheduler::Scheduler, merchant: u32) -> Result<(i32, i32), &'static str> {
    let mut net = 0i32;
    let mut owed = 0i32;
    let mut at = scheduler.head;
    let mut seen = vec![false; usize::from(scheduler.capacity)];
    while at < scheduler.capacity {
        let i = usize::from(at);
        let record = scheduler.records.get(i).ok_or("planificador truncado")?;
        if seen[i] {
            return Err("lista de tareas cíclica");
        }
        seen[i] = true;
        if record.kind == 1 {
            let amount = record.u32_at(8) as i32;
            if u32::from(record.u8_at(0x11)) == merchant {
                owed = owed.wrapping_add(amount);
                net = net.wrapping_sub(amount)
            } else if u32::from(record.u8_at(0x10)) == merchant {
                net = net.wrapping_add(amount)
            }
        }
        at = record.next;
    }
    Ok((net, owed))
}

/// Valor de una oficina (0x500330) con sus dos listas y existencias reales.
/// Si una lista contiene índices imposibles el original termina al llegar a
/// ellos; una lista cíclica o un registro truncado se rechazan explícitamente.
pub fn office_asset_value(
    town: &crate::town::Town,
    site: &crate::buildings::Site,
    town_index: u8,
    hometown: u32,
    store_town_flag: i32,
    business_head: u16,
    house_head: u16,
    stock: &[i32; 24],
) -> Result<i32, &'static str> {
    use crate::town::off;
    let count = usize::from(town.u16(off::BUILDING_RECORD_COUNT));
    if site.records.len() < count * 8 {
        return Err("edificios truncados");
    }
    let mut value = if hometown == u32::from(town_index) { 0i32 } else { 23201 };
    for head in [business_head, house_head] {
        let mut id = usize::from(head);
        let mut seen = vec![false; count];
        while id != usize::from(u16::MAX) {
            if id >= count {
                break;
            }
            if seen[id] {
                return Err("lista de edificios cíclica");
            }
            seen[id] = true;
            let p = id * 8;
            if site.records[p + 3] >= 64 {
                return Err("tipo de edificio inválido");
            }
            value = value.wrapping_add(site.building_base_value(town, id as u32, store_town_flag));
            id = usize::from(u16::from_le_bytes([site.records[p + 4], site.records[p + 5]]));
        }
    }
    for (i, &amount) in stock.iter().enumerate() {
        if amount > 0 {
            let price = f32::from_bits(crate::ai::daily::GOOD_VALUES[i]);
            let part = crate::numeric::ftol_i32(crate::ai::reputation::mul((amount as f32) as f64, price as f64));
            value = value.wrapping_add(part)
        }
    }
    Ok(value)
}

/// Información viva de una oficina necesaria para su valoración diaria.
pub struct DailyOfficeAssets<'a> {
    pub town: u8,
    pub store_town_flag: i32,
    pub business_head: u16,
    pub house_head: u16,
    pub stock: &'a [i32; 24],
    pub site: &'a crate::buildings::Site,
}
/// Dependencias originales de la actualización diaria del comerciante.
pub struct MerchantDailyAssets<'a> {
    pub scheduler: &'a crate::scheduler::Scheduler,
    pub ship_images: &'a [Vec<u8>],
    pub captain_wages: &'a [u16],
    pub offices: &'a [DailyOfficeAssets<'a>],
}
/// Efectos que el anfitrión debe publicar junto al comerciante actualizado.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MerchantDailyEffects {
    pub messages: Vec<(i32, [u8; 16])>,
    pub prepare_alderman_missions: bool,
}

/// Actualiza al comerciante con las dependencias de valoración originales.
/// Si falta una consulta, rechaza la operación antes de modificar el
/// comerciante. La preparación de misiones se devuelve como efecto pendiente.
pub fn update_merchant_daily_with_assets(
    merchant: &mut crate::ai::reputation::Merchant,
    env: &crate::ai::daily::DailyEnv<'_>,
    assets: &MerchantDailyAssets<'_>,
) -> Result<MerchantDailyEffects, String> {
    update_merchant_daily_with_query(merchant, env, assets, None)
}

/// Variante que consulta la cadena real de cartas antes de emitir avisos de rango.
pub fn update_merchant_daily_with_query(
    merchant: &mut crate::ai::reputation::Merchant,
    env: &crate::ai::daily::DailyEnv<'_>,
    assets: &MerchantDailyAssets<'_>,
    query: Option<&dyn Fn(u32, u32) -> Result<bool, String>>,
) -> Result<MerchantDailyEffects, String> {
    use crate::ai::daily::{DailyEnv, DailyHost};
    use crate::ai::misc::{MessageHost, RankHost};
    struct Host<'a> {
        debts: (i32, i32),
        ships: &'a [(i32, i32)],
        offices: &'a [i32],
        messages: Vec<(i32, [u8; 16])>,
        unknown: bool,
        prepare: bool,
        query: Option<&'a dyn Fn(u32, u32) -> Result<bool, String>>,
    }
    impl MessageHost for Host<'_> {
        fn post_message(&mut self, k: i32, r: &[u8; 16]) {
            self.messages.push((k, *r))
        }
    }
    impl RankHost for Host<'_> {
        fn init_world(&mut self) {
            self.prepare = true
        }
        fn message_pending(&mut self, head: u32, value: u32) -> bool {
            match self.query.map(|q| q(head, value)) {
                Some(Ok(found)) => found,
                _ => {
                    self.unknown = true;
                    false
                }
            }
        }
    }
    impl DailyHost for Host<'_> {
        fn debts(&mut self, _: u32) -> (i32, i32) {
            self.debts
        }
        fn ship_value(&mut self, i: usize) -> i32 {
            self.ships[i].0
        }
        fn ship_crew_cost(&mut self, i: usize) -> i32 {
            self.ships[i].1
        }
        fn office_value(&mut self, i: usize, _: u32) -> i32 {
            self.offices[i]
        }
    }
    let _: &DailyEnv<'_> = env;
    if merchant.bytes.len() < 0x650 {
        return Err("comerciante truncado".into());
    }
    let towns = env.rank.town_count as usize;
    let home = usize::from(merchant.bytes[0x19]);
    let ships = usize::from(env.ship_count);
    let offices = usize::from(env.rank.office_count);
    let facilities = usize::from(env.facility_count);
    if towns == 0
        || towns > 40
        || env.rank.towns.len() < towns
        || home >= env.rank.towns.len()
        || env.town_ship_refs.len() < towns
        || env.ships.len() < ships
        || assets.ship_images.len() < ships
        || env.offices.len() < offices
        || assets.offices.len() < offices
        || env.rank.offices.len() < offices
        || env.facilities.len() < facilities
    {
        return Err("entorno diario truncado".into());
    }
    if merchant.u16(8) == 0
        && u16::from(env.rank.byte_52d) >= env.rank.word_4aa
        && usize::from(env.rank.byte_52e) >= env.rank.towns.len()
    {
        return Err("ciudad de elección ausente".into());
    }
    let debts = sum_merchant_debts(assets.scheduler, env.index & 0xffff).map_err(str::to_string)?;
    let mut ship_values = vec![(0i32, 0i32); ships];
    let mut seen = vec![false; ships];
    let mut at = usize::from(merchant.u16(0xe));
    while at < ships {
        if seen[at] {
            return Err("lista de barcos cíclica".into());
        }
        seen[at] = true;
        let image = assets.ship_images[at].as_slice();
        if image.len() < 0x180 {
            return Err("barco truncado".into());
        }
        let ship = &env.ships[at];
        if ship.state != 0x11 && ship.state != 0xe {
            let kind = image[0xe];
            let maximum = i32::from_le_bytes(image[0x14..0x18].try_into().unwrap());
            let current = i32::from_le_bytes(image[0x18..0x1c].try_into().unwrap());
            let sailors = u16::from_le_bytes(image[0x40..0x42].try_into().unwrap());
            let captain = u16::from_le_bytes(image[0x42..0x44].try_into().unwrap());
            ship_values[at] = (
                ship_current_value(kind, maximum, current).map_err(str::to_string)? as i32,
                ship_daily_crew_cost(sailors, captain, assets.captain_wages) as i32,
            );
        }
        at = usize::from(ship.next)
    }
    let mut office_values = vec![0i32; offices];
    let mut seen = vec![false; offices];
    let mut at = usize::from(merchant.u16(0xc));
    while at < offices {
        if seen[at] {
            return Err("lista de oficinas cíclica".into());
        }
        seen[at] = true;
        let office = &env.offices[at];
        if usize::from(office.rep_index) >= 40 || office.bytes.iter().any(|&v| v >= 16) {
            return Err("datos de oficina no admitidos".into());
        }
        let item = &assets.offices[at];
        let town = env
            .rank
            .towns
            .get(usize::from(item.town))
            .ok_or("ciudad de oficina ausente")?;
        office_values[at] = office_asset_value(
            town,
            item.site,
            item.town,
            home as u32,
            item.store_town_flag,
            item.business_head,
            item.house_head,
            item.stock,
        )
        .map_err(str::to_string)?;
        let mut fac = usize::from(office.first_facility);
        let mut fs = vec![false; facilities];
        while fac < facilities {
            if fs[fac] {
                return Err("lista de negocios cíclica".into());
            }
            fs[fac] = true;
            fac = usize::from(env.facilities[fac].next)
        }
        at = usize::from(office.next)
    }
    let mut next = merchant.clone();
    let mut host = Host {
        debts,
        ships: &ship_values,
        offices: &office_values,
        messages: Vec::new(),
        unknown: false,
        prepare: false,
        query,
    };
    next.update_reputation_and_value(env, &mut host);
    if host.unknown {
        return Err("consulta de mensajes no disponible".into());
    }
    *merchant = next;
    Ok(MerchantDailyEffects {
        messages: host.messages,
        prepare_alderman_missions: host.prepare,
    })
}
