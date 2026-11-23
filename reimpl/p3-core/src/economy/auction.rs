//! Fases de espera y apertura de pujas de `st_auction` (0x004E2CD4).
use crate::{scheduler::TaskRecord, town::Town};

fn put_word(task: &mut TaskRecord, offset: usize, value: u16) {
    task.set_u8_at(offset, value as u8);
    task.set_u8_at(offset + 1, (value >> 8) as u8);
}

/// Tasación inicial de casco en la subasta (`0x004DA860`, EXE inglés).
pub fn ship_opening_price(image: &[u8]) -> Result<i32, &'static str> {
    if image.len() < 0x1c {
        return Err("barco de subasta truncado");
    }
    let kind = image[0xe];
    let level = i32::from(image[0xf]);
    let capacity = i32::from_le_bytes(image[0x10..0x14].try_into().unwrap());
    let max = i32::from_le_bytes(image[0x14..0x18].try_into().unwrap());
    let current = i32::from_le_bytes(image[0x18..0x1c].try_into().unwrap());
    let (step, add, first, second, values) = match kind {
        0 => (3, 2700, 19, 23, [11414, 12074, 12784]),
        1 => (5, 4500, 31, 34, [24450, 25010, 26290]),
        2 => (8, 7200, 48, 52, [22296, 22346, 23446]),
        _ => (10, 9000, 59, 65, [34442, 34679, 36644]),
    };
    let capacity_class = (capacity / 2000).wrapping_add(level * step);
    let base = values[if capacity_class < first {
        0
    } else if capacity_class < second {
        1
    } else {
        2
    }];
    let damage = max.wrapping_sub(current);
    let penalty = (i64::from(damage) * 300 / 2800) as i32;
    let subtotal = (base as i32).wrapping_add(level * add).wrapping_sub(penalty);
    let rounded = crate::numeric::ftol_i32(f64::from(subtotal as f32) * 0.7 + 0.5);
    Ok((rounded / 100) * 100)
}

/// Escala el valor base de un edificio como `0x004DAF10`.
pub fn building_sale_price(base: i32, inside_wall: bool, siege_active: bool) -> i32 {
    let multiplier = if inside_wall { 6 } else { 4 };
    let mut value = base.wrapping_mul(multiplier).wrapping_add(5) / 10;
    if !inside_wall && siege_active {
        value >>= 1;
    }
    value - value % 100
}

/// `0x00633AE0/0x00629C30`: si la escena de asedio continúa activa.
pub fn siege_active(scene: Option<&[u8]>) -> Result<bool, &'static str> {
    let Some(scene) = scene else { return Ok(false) };
    if scene.len() < 0x220 {
        return Err("escena de asedio truncada");
    }
    let word = |at| u16::from_le_bytes([scene[at], scene[at + 1]]);
    let limit = word(0x3a);
    Ok(if limit != 0 {
        word(0x6a) < limit
    } else {
        word(0x21e) != 0
    })
}

/// Fase 8: libera la reserva si aún pertenece a esta tarea.
pub fn finish(task: &TaskRecord, towns: &mut [Town], head: u16) -> Result<u8, &'static str> {
    let city = task.u8_at(0x15);
    let town = towns.get_mut(usize::from(city)).ok_or("ciudad de subasta ausente")?;
    if town.u16(0x9a2) == head {
        town.set_u16(0x9a2, u16::MAX)
    }
    Ok(city)
}

/// Resultado para la presentación de una subasta.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AuctionResult {
    pub buyer: u8,
    pub object: u16,
    pub price: u16,
    pub city: u8,
    pub building: bool,
}

/// Lote inmobiliario sin comprador y posible destinatario de la carta 0x7E.
pub fn unsold_building(
    task: &mut TaskRecord,
    towns: &[Town],
    sites: &[crate::buildings::Site],
    merchant_count: u16,
) -> Result<(AuctionResult, Option<u8>), &'static str> {
    if task.kind != 0x36 || task.u8_at(0x14) != 7 || towns.is_empty() {
        return Err("adjudicación pública inválida");
    }
    let mut next = *task;
    let city = usize::from(next.u8_at(0x15)) % towns.len();
    let town = &towns[city];
    let site = sites.get(city).ok_or("plano de subasta ausente")?;
    let object = u16::from_le_bytes([next.u8_at(8), next.u8_at(9)]) & 0x7fff;
    let at = usize::from(object) * 8;
    if object >= town.u16(0x776) || site.records.len() < at + 8 {
        return Err("lote inmobiliario ausente");
    }
    let buyer = next.u8_at(0x17);
    if u16::from(buyer) < merchant_count {
        return Err("adjudicación inmobiliaria pendiente");
    }
    let price = u16::from_le_bytes([next.u8_at(0xe), next.u8_at(0xf)]);
    next.set_u8_at(0x15, u8::try_from(city).map_err(|_| "ciudad fuera de rango")?);
    put_word(&mut next, 8, u16::MAX);
    skip_missing_lot(&mut next, towns, 0)?;
    *task = next;
    let owner = site.records[at + 2];
    Ok((
        AuctionResult {
            buyer,
            object,
            price,
            city: city as u8,
            building: true,
        },
        (u16::from(owner) < merchant_count).then_some(owner),
    ))
}

/// Venta de un inmueble a un comerciante; devuelve también el dueño anterior.
pub fn bought_building(
    task: &mut TaskRecord,
    towns: &[Town],
    sites: &[crate::buildings::Site],
    merchant_count: u16,
) -> Result<(AuctionResult, u8), &'static str> {
    if task.kind != 0x36 || task.u8_at(0x14) != 7 || towns.is_empty() {
        return Err("adjudicación inmobiliaria inválida");
    }
    let mut next = *task;
    let city = usize::from(next.u8_at(0x15)) % towns.len();
    let town = &towns[city];
    let site = sites.get(city).ok_or("plano de subasta ausente")?;
    let object = u16::from_le_bytes([next.u8_at(8), next.u8_at(9)]) & 0x7fff;
    let at = usize::from(object) * 8;
    if object >= town.u16(0x776) || site.records.len() < at + 8 {
        return Err("lote inmobiliario ausente");
    }
    let buyer = next.u8_at(0x17);
    if u16::from(buyer) >= merchant_count {
        return Err("comprador inmobiliario inválido");
    }
    let owner = site.records[at + 2];
    let price = u16::from_le_bytes([next.u8_at(0xe), next.u8_at(0xf)]);
    next.set_u8_at(0x15, u8::try_from(city).map_err(|_| "ciudad fuera de rango")?);
    put_word(&mut next, 8, u16::MAX);
    skip_missing_lot(&mut next, towns, 0)?;
    *task = next;
    Ok((
        AuctionResult {
            buyer,
            object,
            price,
            city: city as u8,
            building: true,
        },
        owner,
    ))
}

/// Venta de un inmueble al mismo comerciante que ya lo posee.
pub fn self_bought_building(
    task: &mut TaskRecord,
    towns: &[Town],
    sites: &[crate::buildings::Site],
    merchant_count: u16,
) -> Result<AuctionResult, &'static str> {
    let mut staged = *task;
    let (result, owner) = bought_building(&mut staged, towns, sites, merchant_count)?;
    if owner != result.buyer {
        return Err("traspaso inmobiliario pendiente");
    }
    *task = staged;
    Ok(result)
}

/// Fase 7 sin lote actual válido: selecciona el siguiente o prepara el cierre.
pub fn skip_missing_lot(task: &mut TaskRecord, towns: &[Town], ship_count: u16) -> Result<(), &'static str> {
    if task.u8_at(0x14) != 7 || !matches!(task.kind, 0x0a | 0x36) {
        return Err("fase de adjudicación inválida");
    }
    let mut next = *task;
    let is_ship = next.kind == 0x0a;
    let count = if is_ship {
        ship_count
    } else {
        if towns.is_empty() {
            return Err("subasta sin ciudades");
        }
        let city = usize::from(next.u8_at(0x15)) % towns.len();
        next.set_u8_at(0x15, u8::try_from(city).map_err(|_| "ciudad fuera de rango")?);
        towns[city].u16(0x776)
    };
    let word = |t: &TaskRecord, p| u16::from_le_bytes([t.u8_at(p), t.u8_at(p + 1)]);
    let valid = |v: u16| if is_ship { v < count } else { v & 0x7fff < count };
    if valid(word(&next, 8)) {
        return Err("adjudicación de lote existente no conectada");
    }
    if valid(word(&next, 10)) {
        let v = word(&next, 10);
        put_word(&mut next, 10, u16::MAX);
        put_word(&mut next, 8, v)
    } else if valid(word(&next, 12)) {
        let v = word(&next, 12);
        put_word(&mut next, 12, u16::MAX);
        put_word(&mut next, 8, v)
    }
    if valid(word(&next, 8)) {
        next.set_u8_at(0x14, 3);
        next.due = next.due.wrapping_add(25)
    } else {
        next.set_u8_at(0x14, 8);
        next.due = (next.due & !0xff) | 0xff
    }
    *task = next;
    Ok(())
}

/// Fase 2: encuentra el siguiente lote, reserva la ciudad y anuncia a humanos.
/// El llamante entrega los avisos de presentación que devuelve esta función.
pub fn announce(
    task: &mut TaskRecord,
    towns: &mut [Town],
    ship_count: u16,
    scheduler_capacity: u16,
    head: u16,
    controls: &[u16],
) -> Result<(bool, Vec<u16>), String> {
    if !matches!(task.kind, 0x0a | 0x36) || task.u8_at(0x14) != 2 {
        return Err("fase de anuncio inválida".into());
    }
    let mut staged = *task;
    let mut cities = towns.to_vec();
    let city = usize::from(staged.u8_at(0x15));
    let town = cities.get_mut(city).ok_or("ciudad de subasta ausente")?;
    let is_ship = staged.kind == 0x0a;
    let count = if is_ship { ship_count } else { town.u16(0x776) };
    let valid = |v: u16| if is_ship { v < count } else { v & 0x7fff < count };
    let word = |t: &TaskRecord, p| u16::from_le_bytes([t.u8_at(p), t.u8_at(p + 1)]);
    if !valid(word(&staged, 8)) {
        if valid(word(&staged, 10)) {
            let v = word(&staged, 10);
            put_word(&mut staged, 10, u16::MAX);
            put_word(&mut staged, 8, v)
        } else if valid(word(&staged, 12)) {
            let v = word(&staged, 12);
            put_word(&mut staged, 12, u16::MAX);
            put_word(&mut staged, 8, v)
        }
        if !valid(word(&staged, 8)) {
            *task = staged;
            return Ok((false, Vec::new()));
        }
    }
    let mut notices = Vec::new();
    if town.u16(0x9a2) >= scheduler_capacity {
        town.set_u16(0x9a2, head);
        let mut bit = 1u8;
        for (i, &control) in controls.iter().enumerate() {
            if control == 0 {
                if staged.u8_at(0x16) & bit != 0 {
                    notices.push(u16::try_from(i).map_err(|_| "demasiados comerciantes")?)
                }
                bit = bit.wrapping_shl(1)
            }
        }
        staged.set_u8_at(0x14, 3);
    }
    staged.due = staged.due.wrapping_add(0x100) & !0xff;
    *task = staged;
    towns.clone_from_slice(&cities);
    Ok((true, notices))
}

/// Fase 1: espera el intervalo del anuncio y redondea al inicio del día.
pub fn wait_for_announcement(task: &mut TaskRecord) {
    let delay = u16::from_le_bytes([task.u8_at(0xe), task.u8_at(0xf)]);
    put_word(task, 0xe, 0);
    task.set_u8_at(0x14, 2);
    task.due = task.due.wrapping_add(u32::from(delay)) & !0xff;
}

/// Fase 3: el valor se obtiene del cálculo naval o inmobiliario del original.
pub fn open_bidding(task: &mut TaskRecord, value: u16) {
    put_word(task, 0x12, 0);
    task.due = task.due.wrapping_add(1);
    task.set_u8_at(0x17, task.u8_at(0x15));
    task.set_u8_at(0x14, 4);
    put_word(task, 0xe, value);
    let target = u32::from(value) / 25;
    let mut increment = (u32::from(value) / 5000 + 1) * 100;
    let mut phase = 0;
    while increment < target {
        match phase {
            0 => {
                increment *= 2;
                if (increment as u16) < 20_000 {
                    phase = 1;
                }
            }
            1 => {
                increment = increment * 2 + ((increment >> 1) & 0x7fff);
                phase = 2;
            }
            _ => {
                increment *= 2;
                phase = 0;
            }
        }
    }
    put_word(task, 0x10, increment as u16);
}

/// Límite de puja de un comerciante, calculado por el sistema correspondiente.
pub trait BidLimit {
    fn limit(&mut self, merchant: u16, ship: bool, object: u16, city: u8) -> Result<i32, String>;
}

/// Fases 4 a 6: recorre los posibles pujadores y avanza la ronda.
/// Conserva el registro si falta una dependencia del cálculo de puja.
pub fn run_bids(task: &mut TaskRecord, merchant_count: u16, host: &mut impl BidLimit) -> Result<bool, String> {
    let phase = task.u8_at(0x14);
    if !matches!(phase, 4..=6) || merchant_count == 0 {
        return Err("ronda de subasta inválida".into());
    }
    let mut staged = *task;
    let word = |t: &TaskRecord, p| u16::from_le_bytes([t.u8_at(p), t.u8_at(p + 1)]);
    let price = u32::from(word(&staged, 0xe)) + u32::from(word(&staged, 0x10));
    let counter = word(&staged, 0x12);
    let mut merchant = (staged.due % u32::from(merchant_count)) as u16;
    staged.due = staged.due.wrapping_add(1);
    let loops = (i32::from(counter >> 1) - 64 + i32::from(phase)).max(0) as u32 + 1;
    let ship = staged.kind == 0x0a;
    let object = if ship {
        word(&staged, 8)
    } else {
        word(&staged, 8) & 0x7fff
    };
    let city = staged.u8_at(0x15);
    let mut changed = false;
    for _ in 0..loops {
        if merchant != u16::from(staged.u8_at(0x17)) && (price as i32) <= host.limit(merchant, ship, object, city)? {
            put_word(&mut staged, 0xe, price as u16);
            staged.set_u8_at(0x17, merchant as u8);
            put_word(&mut staged, 0x12, 0);
            staged.set_u8_at(0x14, 4);
            changed = true;
            break;
        }
        merchant += 1;
        if merchant >= merchant_count {
            merchant = 0;
        }
    }
    if !changed {
        let next = counter.wrapping_add(1);
        put_word(&mut staged, 0x12, next);
        if next > 16 {
            put_word(&mut staged, 0x12, 0);
            staged.set_u8_at(0x14, phase + 1);
        }
    }
    *task = staged;
    Ok(changed)
}

/// Tarea 0x2D: actualiza una puja automatica vinculada a una subasta.
/// Devuelve si el despachador debe conservar y reprogramar la tarea.
pub fn standing_bid(s: &mut crate::scheduler::Scheduler, tick: u32, merchant_count: u16) -> bool {
    let head = usize::from(s.head);
    let Some(task) = s.records.get(head).copied() else {
        return false;
    };
    let word = |r: &TaskRecord, off| u16::from_le_bytes([r.u8_at(off), r.u8_at(off + 1)]);
    let target = usize::from(word(&task, 0xc));
    if target >= usize::from(s.capacity) {
        return false;
    }
    let Some(mut auction) = s.records.get(target).copied() else {
        return false;
    };
    if !matches!(auction.kind, 0x0a | 0x36)
        || auction.u8_at(0x15) != task.u8_at(0x11)
        || auction.due < tick
        || auction.u8_at(0x14) > 7
        || u16::from(task.u8_at(0x10)) >= merchant_count
    {
        return false;
    }
    if auction.u8_at(0x14) <= 3 {
        s.records[head].due = auction.due.wrapping_add(1);
        return true;
    }
    if word(&auction, 8) != word(&task, 0xe) {
        s.records[head].due = task.due.wrapping_add(3);
        return true;
    }
    let amount = u32::from(word(&auction, 0xe)) + u32::from(word(&auction, 0x10));
    if (amount as i32) > (task.u32_at(8) as i32) {
        return false;
    }
    if auction.u8_at(0x17) != task.u8_at(0x10) && auction.u8_at(0x14) != 7 {
        put_word(&mut auction, 0xe, amount as u16);
        auction.set_u8_at(0x17, task.u8_at(0x10));
        auction.set_u8_at(0x14, 4);
        put_word(&mut auction, 0x12, 0);
        s.records[target] = auction;
    }
    s.records[head].due = task.due.wrapping_add(1);
    true
}
