//! Actualización diaria de la iglesia de la ciudad (`0x004FE877`).
use crate::town::Town;

/// Actualiza donativos, entregas y nivel. Devuelve `true` si se debe encolar
/// la obra de ampliación con `Site::start_church_upgrade_construction`.
pub fn daily_update(town: &mut Town, factor: u8) -> bool {
    const BASE: [i32; 3] = [20_000, 40_000, 60_000];
    const STEP: [i32; 3] = [10_000, 15_000, 20_000];
    const GOODS: [usize; 3] = [11, 12, 19];
    const REQUIRED: [[u8; 3]; 3] = [[20, 20, 30], [20, 20, 30], [50, 50, 60]];
    const UNIT: [i32; 3] = [2000, 200, 2000];
    let base = 0x794;
    town.set_u32(base + 8, town.u32(base + 8).wrapping_add(1));
    let donations = town.i32(base);
    town.set_i32(base, if donations >= 100 { donations - 100 } else { 0 });
    let level = town.u8(base + 15) as usize;
    if level >= 3 || town.i32(base + 4) < BASE[level] + STEP[level] * i32::from(factor) {
        return false;
    }
    let mut complete = true;
    for i in 0..3 {
        let current = town.u8(base + 12 + i);
        let needed = REQUIRED[i][level];
        if current >= needed {
            continue;
        }
        let ware = GOODS[i];
        let available = town.stock(ware).wrapping_sub(town.threshold(ware, 0) >> 1);
        if available <= 0 {
            complete = false;
            continue;
        }
        let units = (available / UNIT[i]).min(i32::from(needed - current));
        if units < i32::from(needed - current) {
            complete = false;
        }
        town.set_u8(base + 12 + i, current.wrapping_add(units as u8));
        town.set_stock(ware, town.stock(ware).wrapping_sub(units * UNIT[i]));
    }
    if complete {
        town.set_i32(base + 4, 0);
        town.set_u32(base + 8, 0);
        for i in 0..3 {
            town.set_u8(base + 12 + i, 0);
        }
        town.set_u8(base + 15, (level + 1) as u8);
    }
    complete
}

/// `church_get_donation_tier` (`0x004FE360`, `this = Town+0x794`): nivel de donaciones recientes de la ciudad,
/// `(fondo + mitad de la unidad) / unidad` con unidad `2000 · (ajuste + 1)` y tope 5. `setting` es el byte `0x006DE52C`.
pub fn donation_tier(town: &Town, setting: u8) -> i32 {
    let unit = 2000 * (i32::from(setting) + 1);
    (town.i32(0x794).wrapping_add(unit >> 1) / unit).min(5)
}

/// `church_get_extension_stage` (`0x004FEA97`): 5 con la iglesia al máximo (nivel ≥ 3), 4 sin aforo (`+8 ≤ 100`) o con la obra ya en la
/// lista de pendientes (registro de tipo 0x1F), 3 con la caja llena y, si no, `3 · recaudado / límite` (0..2).
pub fn extension_stage(town: &Town, site: &crate::buildings::Site, setting: u8) -> i32 {
    const BONUS: [i32; 3] = [20_000, 40_000, 60_000];
    let level = town.u8(0x794 + 0xf);
    if level >= 3 {
        return 5;
    }
    if town.u32(0x794 + 8) <= 0x64 {
        return 4;
    }
    let s = i32::from(setting);
    let step = [10_000, 15_000, 20_000][usize::from(level)];
    let limit = s.wrapping_mul(step).wrapping_add(BONUS[usize::from(level)]);
    let collected = town.i32(0x794 + 4);
    if collected >= limit {
        return 3;
    }
    let count = town.u16(crate::town::off::BUILDING_RECORD_COUNT);
    let mut at = town.u16(crate::town::off::CONSTRUCTION_HEAD);
    while at < count {
        if site.rec(usize::from(at), 3) == 0x1f {
            return 4;
        }
        at = site.next(usize::from(at));
    }
    collected.wrapping_mul(3).wrapping_div(limit)
}

/// Mercancías de la ayuda a los pobres (tabla `0x006734CC`): cereales, carne, pescado, cerveza y vino... en el orden del original.
pub const FEED_WARES: [usize; 5] = [0, 3, 2, 1, 7];

/// Entorno de [`feed_poor`] (`handle_feeding_the_poor`, `0x004FE557`).
pub trait FeedHost {
    /// Existencias de la oficina del comerciante en la ciudad (`find_or_create_office`, `0x005308A0`, sin crear): `Office+4`.
    fn office(&mut self) -> Option<&mut [i32; 24]>;
    /// Primera nave del comerciante (`Merchant+0xE`) y recuento de naves (`0x006DD894`, 16 bits).
    fn first_ship(&self) -> u16;
    fn ship_count(&self) -> u16;
    /// Enlace `+4`, última ciudad `+0x39` y carga `+0x54` de la nave.
    fn ship_next(&self, ship: u16) -> u16;
    fn ship_town(&self, ship: u16) -> u8;
    fn ship_cargo(&mut self, ship: u16) -> &mut [i32; 24];
    /// `get_sell_price` (`0x0052E1D0`): `(mercancía, ciudad, cantidad)`.
    fn price(&mut self, ware: u32, town: u32, amount: u32) -> i32;
}

/// Resultado: lo entregado por mercancía, el valor total que se anota como reputación (`None` con puntuación < 10) y si la ciudad queda marcada.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct FeedResult {
    pub given: [i32; 5],
    pub reputation: Option<i32>,
    pub flag: bool,
}

/// Toma de cada existencia lo pedido (`amounts` en unidades de 200 o 2000), primero de la oficina y después de las naves del comerciante
/// atracadas en la ciudad. `None` si el comerciante no existe. Con puntuación ≥ 10 valora lo entregado a precio de venta y suma esa reputación;
/// con ≥ 50 se marca la ciudad (`Town+0x2C8 |= 0x800000`).
pub fn feed_poor(host: &mut dyn FeedHost, merchant_count: u16, merchant: u8, town: u8, score: u8, amounts: [u16; 5]) -> Option<FeedResult> {
    if u16::from(merchant) >= merchant_count {
        return None;
    }
    let unit = |ware: usize| if super::prices::SMALL_LOT_WARES[ware] { 200 } else { 2000 };
    let mut need = [0i32; 5];
    for k in 0..5 {
        need[k] = i32::from(amounts[k]) * unit(FEED_WARES[k]);
    }
    let mut given = [0i32; 5];
    // Toma de una lista de existencias; devuelve el indicador final del original (1 solo si todo quedó cubierto sin recorte parcial).
    let take = |stock: &mut [i32; 24], need: &mut [i32; 5], given: &mut [i32; 5]| -> bool {
        let mut done = true;
        for k in 0..5 {
            let w = FEED_WARES[k];
            let have = stock[w];
            if have <= 0 {
                stock[w] = 0;
                done = done && need[k] == 0;
            } else if have > need[k] {
                stock[w] = have - need[k];
                given[k] += need[k];
                need[k] = 0;
            } else {
                let u = unit(w);
                let part = have / u * u;
                need[k] -= part;
                given[k] += part;
                stock[w] = have - part;
                done = false;
            }
        }
        done
    };
    let mut done = match host.office() {
        Some(stock) => take(stock, &mut need, &mut given),
        None => false,
    };
    if !done {
        let count = u32::from(host.ship_count());
        let mut at = host.first_ship();
        while u32::from(at) < count {
            let next = host.ship_next(at);
            if host.ship_town(at) == town {
                done = take(host.ship_cargo(at), &mut need, &mut given);
                if done {
                    break;
                }
            }
            at = next;
        }
    }
    let mut result = FeedResult { given, reputation: None, flag: false };
    if score >= 10 {
        let mut total = 0i32;
        for k in 0..5 {
            if given[k] > 0 {
                total = total.wrapping_add(host.price(FEED_WARES[k] as u32, u32::from(town), given[k] as u32));
            }
        }
        result.reputation = Some(total);
        result.flag = score >= 0x32;
    }
    Some(result)
}

/// `op_town_treasury_transfer` (`0x0053AF30`, operación `0x47`): `op = [0x47, comerciante, ciudad, sentido, importe]`. Con sentido distinto de 0 el
/// comerciante ingresa en las arcas (partida 3, suma `Merchant+0x4BC`); con 0 retira, limitado a `Town.money` (partida 9, suma `+0x4B8`).
/// Los índices fuera de rango no hacen nada. `tick` es el reloj del mundo (`0x006DE4B4`).
pub fn treasury_transfer(op: [u32; 5], towns: &mut [Town], merchants: &mut [crate::ai::reputation::Merchant], merchant_count: u16, tick: u32) {
    let (who, town) = (op[1] as usize, op[2] as usize);
    if op[1] >= u32::from(merchant_count) || town >= towns.len() || who >= merchants.len() {
        return;
    }
    let mut amount = op[4];
    let m = &mut merchants[who];
    if op[3] != 0 {
        super::militia::book_transaction(&mut towns[town], tick, amount as i32, 3);
        m.set_i32(0, m.i32(0).wrapping_sub(amount as i32));
        m.set_i32(0x4bc, m.i32(0x4bc).wrapping_add(amount as i32));
    } else {
        let money = towns[town].u32(crate::town::off::MONEY);
        if money < amount {
            amount = if (money as i32) <= 0 { 0 } else { money };
        }
        super::militia::book_transaction(&mut towns[town], tick, amount as i32, 9);
        m.set_i32(0, m.i32(0).wrapping_add(amount as i32));
        m.set_i32(0x4b8, m.i32(0x4b8).wrapping_add(amount as i32));
    }
}
