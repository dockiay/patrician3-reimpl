//! Transactional IA ship orders: opcode 0x2C / Town::handle_build_ship (0x52A360).
//! Inputs are live images; the caller publishes the returned state to its
//! economic and naval views together. No operation is enqueued here.
use p3_core::{
    ai::reputation::Merchant,
    economy::{
        prices::{buy_price, Market},
        trade::OfficeStock,
    },
    numeric::ftol_i32,
    ships::factory,
    town::Town,
};
const WARES: [usize; 5] = [11, 8, 12, 17, 15];
const UNITS: [i32; 5] = [2000, 200, 200, 2000, 200];
const REQUIRED: [[[i32; 5]; 3]; 4] = [
    [[7, 3, 3, 3, 20], [9, 3, 3, 3, 20], [11, 3, 3, 3, 20]],
    [[12, 5, 5, 5, 30], [14, 5, 5, 5, 30], [16, 5, 5, 5, 30]],
    [[18, 3, 4, 4, 40], [20, 3, 4, 4, 40], [22, 3, 4, 4, 40]],
    [[30, 10, 10, 8, 50], [33, 10, 10, 8, 50], [36, 10, 10, 8, 50]],
];
const BASE_COST: [[i32; 3]; 4] = [
    [7650, 8200, 8800],
    [18270, 18720, 19890],
    [16560, 16500, 17490],
    [22968, 23040, 24840],
];
/// Nombres de clase del ejecutable (`0x006C0CB4` apunta a ellos: «snaikka», «crayer», «cog», «hulk»).
const HUMAN_NAMES: [&[u8]; 4] = [b"snaikka", b"crayer", b"cog", b"hulk"];
const POINTS: [[u8; 4]; 4] = [[15, 19, 23, 25], [28, 31, 34, 35], [45, 48, 52, 55], [55, 59, 65, 70]];

#[derive(Clone)]
pub struct BuildState {
    pub town: Town,
    pub merchant: Merchant,
    pub office: Option<OfficeStock>,
    pub ships: Vec<Vec<u8>>,
    pub free_ship: u16,
    pub name_cursor: u16,
    pub name_step: u16,
}
pub struct BuildResources<'a> {
    pub tick: u32,
    pub port: (u16, u16),
    pub names: &'a [Vec<u8>],
}
pub struct PreparedBuild {
    pub state: BuildState,
    pub ship: u16,
    pub cost: i32,
}
fn word(b: &[u8], at: usize) -> u16 {
    u16::from_le_bytes(b[at..at + 2].try_into().unwrap())
}
fn integer(b: &[u8], at: usize) -> i32 {
    i32::from_le_bytes(b[at..at + 4].try_into().unwrap())
}
fn w16(b: &mut [u8], at: usize, v: u16) {
    b[at..at + 2].copy_from_slice(&v.to_le_bytes())
}
fn w32(b: &mut [u8], at: usize, v: i32) {
    b[at..at + 4].copy_from_slice(&v.to_le_bytes())
}

/// The implemented dispatch is for AI merchants; human names/UI are a separate
/// path and are rejected before mutation. A rejected game order returns None.
pub fn prepare(
    state: &BuildState,
    owner: u8,
    kind: u8,
    allow_credit: bool,
    r: &BuildResources<'_>,
) -> Result<Option<PreparedBuild>, String> {
    if state.merchant.bytes.len() < 0x650 || state.ships.iter().any(|s| s.len() != 0x180) {
        return Err("imágenes de pedido naval incompletas".into());
    }
    // `Merchant+8 == 0` es el comerciante humano: no puede endeudarse y su barco lleva el nombre de la clase.
    let human = state.merchant.u16(8) == 0;
    if kind >= 4 {
        return Ok(None);
    }
    let k = usize::from(kind);
    let level = state.town.u8(0x824 + k) as i8;
    if level < 0 || state.town.more_flags() & 0x20000 == 0 || state.town.flags() & 1 != 0 && kind & 2 != 0 {
        return Ok(None);
    }
    if level > 3 {
        return Err("nivel de astillero fuera de tabla".into());
    }
    if state.town.u16(0x854) == 0 {
        return Err("divisor de astillero nulo".into());
    }
    let city = state.town.u8(0x2c0);
    let mut at = usize::from(state.merchant.u16(0xe));
    let mut chain = Vec::new();
    let mut seen = vec![false; state.ships.len()];
    while at < state.ships.len() {
        if seen[at] {
            return Err("cadena de barcos de pedido cíclica".into());
        }
        seen[at] = true;
        chain.push(at);
        at = usize::from(word(&state.ships[at], 4));
    }
    if let Some(office) = &state.office {
        if office.owner != u16::from(owner) || office.town != city {
            return Err("oficina de pedido incompatible".into());
        }
    }
    let mut available = state
        .office
        .as_ref()
        .map_or([0; 5], |o| std::array::from_fn(|i| o.stock[WARES[i]]));
    for &id in &chain {
        let s = &state.ships[id];
        if s[0x39] == city && word(s, 0x134) < 4 && s[0x136] & 1 == 0 {
            for i in 0..5 {
                available[i] = available[i].wrapping_add(integer(s, 0x54 + 4 * WARES[i]));
            }
        }
    }
    let requirements = REQUIRED[k][usize::from((level as u8).min(2))];
    let mut owned = [0i32; 5];
    let mut town_take = [0i32; 5];
    let mut cost = BASE_COST[k][usize::from((level as u8).min(2))];
    let points =
        (i32::from(POINTS[k][level as usize]) + i32::from(state.town.u8(0x828 + k))).min(i32::from(POINTS[k][3]));
    let supplement = if points > 60 {
        300
    } else if points > 50 {
        330
    } else if points > 40 {
        360
    } else if points > 30 {
        390
    } else if points > 20 {
        420
    } else {
        450
    };
    cost = cost
        .wrapping_add(supplement * 2)
        .wrapping_add(ftol_i32(f64::from(f32::from_bits(state.town.u32(0x818)))));
    for i in 0..5 {
        owned[i] = available[i] / UNITS[i];
        if owned[i] >= requirements[i] {
            owned[i] = requirements[i]
        } else {
            town_take[i] = requirements[i].wrapping_sub(owned[i]);
            if state.town.stock(WARES[i]) / UNITS[i] < town_take[i] {
                return Ok(None);
            }
            let ware = WARES[i];
            let market = Market {
                trading: state.town.more_flags() & 0x100 != 0,
                stock: state.town.stock(ware),
                thresholds: std::array::from_fn(|j| state.town.threshold(ware, j)),
                marked_ware: state.town.u8(0x6f0),
            };
            cost = cost.wrapping_add(buy_price(ware, &market, town_take[i].wrapping_mul(UNITS[i])));
        }
    }
    if cost > state.merchant.i32(0) && (!allow_credit || human) {
        return Ok(None);
    }
    let mut next = state.clone();
    let mut remaining = owned;
    if let Some(office) = next.office.as_mut() {
        for i in 0..5 {
            let ware = WARES[i];
            let take = (office.stock[ware] / UNITS[i]).min(remaining[i]);
            remaining[i] = remaining[i].wrapping_sub(take);
            office.stock[ware] = office.stock[ware].wrapping_sub(take.wrapping_mul(UNITS[i]));
        }
    }
    for &id in &chain {
        let s = &mut next.ships[id];
        if s[0x39] == city && word(s, 0x134) < 4 {
            let mut recalc = false;
            for i in 0..5 {
                let offset = 0x54 + 4 * WARES[i];
                let stock = integer(s, offset);
                if stock > 0 {
                    let take = (stock / UNITS[i]).min(remaining[i]);
                    remaining[i] = remaining[i].wrapping_sub(take);
                    w32(s, offset, stock.wrapping_sub(take.wrapping_mul(UNITS[i])));
                    recalc = true;
                }
            }
            if recalc {
                p3_core::ships::cargo::recompute_load(s);
            }
        }
    }
    for i in 0..5 {
        let ware = WARES[i];
        next.town.set_stock(
            ware,
            next.town.stock(ware).wrapping_sub(town_take[i].wrapping_mul(UNITS[i])),
        );
    }
    for ware in 0..24 {
        if next.town.stock(ware) < 0 {
            next.town.set_stock(ware, 0)
        }
    }
    let count = next.ships.len();
    let mut queue = usize::from(next.town.u16(0x81c));
    let mut last = None;
    let mut queued = vec![false; count];
    while queue < count {
        if queued[queue] {
            return Err("cola de obra naval cíclica".into());
        }
        queued[queue] = true;
        last = Some(queue);
        queue = usize::from(word(&next.ships[queue], 6));
    }
    if usize::from(next.free_ship) >= count {
        if count + 64 > u16::MAX as usize {
            return Err("pool naval fuera de rango".into());
        }
        next.free_ship = count as u16;
        for i in count..count + 64 {
            let mut image = vec![0; 0x180];
            w16(&mut image, 4, u16::MAX);
            w16(&mut image, 6, (i + 1) as u16);
            w16(&mut image, 8, u16::MAX);
            w16(&mut image, 0x134, 255);
            next.ships.push(image);
        }
    }
    let id = next.free_ship;
    let p = usize::from(id);
    if queued.get(p) == Some(&true) || chain.contains(&p) {
        return Err("ranura libre naval está enlazada".into());
    }
    next.free_ship = word(&next.ships[p], 6);
    factory::reset_ship(&mut next.ships[p], r.tick).ok_or("barco de pedido truncado")?;
    next.ships[p][0x137] = 0;
    w16(&mut next.ships[p], 0x134, 0xe);
    if let Some(last) = last {
        w16(&mut next.ships[last], 6, id)
    } else {
        next.town.set_u16(0x81c, id)
    }
    factory::init_ship_at_town(&mut next.ships[p], city, r.port.0, r.port.1, r.tick)
        .ok_or("barco de pedido truncado")?;
    next.ships[p][0x13b] = city;
    factory::init_ship_stats(&next.town, &mut next.ships[p], kind).ok_or("estadísticas navales fuera de tabla")?;
    factory::recompute_weapon_slots(&mut next.ships[p]).ok_or("ranuras navales inválidas")?;
    p3_core::ships::cargo::recompute_load(&mut next.ships[p]);
    next.ships[p][0] = owner;
    w16(&mut next.ships[p], 4, next.merchant.u16(0xe));
    next.merchant.set_u16(0xe, id);
    next.ships[p][0x13a] = next.merchant.bytes[0x19];
    if human {
        // `strcpy(ship+0x160, 0x6C0CB4[clase])` y `+0x15E = 0x7FFF - (clase & 3)`; no consume el generador de nombres.
        let class = usize::from(next.ships[p][0xe] & 3);
        let name = HUMAN_NAMES[usize::from(kind & 3)];
        w16(&mut next.ships[p], 0x15e, 0x7fff - class as u16);
        next.ships[p][0x160..0x160 + name.len()].copy_from_slice(name);
        next.ships[p][0x160 + name.len()] = 0;
    } else {
    let name_index = factory::next_ship_name_index(&mut next.name_cursor, next.name_step, r.names.len())
        .ok_or("generador de nombres naval sin salida")?;
    let name = &r.names[usize::from(name_index)];
    let length = name.iter().position(|&b| b == 0).unwrap_or(name.len());
    if length > 31 {
        return Err("nombre de barco excede el campo".into());
    }
    w16(&mut next.ships[p], 0x15e, next.name_cursor);
    next.ships[p][0x160..0x160 + length].copy_from_slice(&name[..length]);
    next.ships[p][0x160 + length] = 0;
    }
    next.ships[p][0x3c] |= 1;
    w32(&mut next.ships[p], 0x18, 0);
    let gold = next.merchant.i32(0).wrapping_sub(cost);
    next.merchant.set_i32(0, gold);
    let total = next.merchant.i32(0x4c4).wrapping_add(cost);
    next.merchant.set_i32(0x4c4, total);
    w32(&mut next.ships[p], 0x48, cost);
    w32(&mut next.ships[p], 0x4c, r.tick as i32);
    let hull = integer(&next.ships[p], 0x14) as u32;
    next.merchant
        .add_social_reputation_ship_hull_work(u32::from(city), hull);
    Ok(Some(PreparedBuild {
        state: next,
        ship: id,
        cost,
    }))
}
