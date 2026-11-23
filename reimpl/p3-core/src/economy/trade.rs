//! Compraventa de existencias entre una oficina y la ciudad (opcodes 0x58 y 0x59).
//!
//! El contexto valida y localiza la oficina antes de llamar; el seguimiento de reputación y
//! estadísticas del comerciante se entrega al puerto `record_trade`.
use super::prices::{buy_price, sell_price, Market, SMALL_LOT_WARES};
use crate::town::Town;

#[derive(Clone, Debug, PartialEq)]
pub struct OfficeStock {
    pub owner: u16,
    pub town: u8,
    pub stock: [i32; 24],
    pub average_price: [f32; 24],
    /// Consumo de negocios de la oficina (0x64).
    pub business_consumption: [i32; 24],
    /// Objetivo de almacén para la IA (primeras 20 mercancías, 0x354).
    pub ai_target: [i32; 24],
    /// Precio configurado para el administrador: positivo vende; negativo compra.
    pub administrator_limit: [i32; 24],
    /// Experiencia del administrador si existe.
    pub administrator_experience: Option<u8>,
    /// Banderas de automatización de oficina (+0x2D6).
    pub automation_flags: u16,
}
impl Default for OfficeStock {
    fn default() -> Self {
        Self {
            owner: 0,
            town: 0,
            stock: [0; 24],
            average_price: [0.0; 24],
            business_consumption: [0; 24],
            ai_target: [0; 24],
            administrator_limit: [0; 24],
            administrator_experience: None,
            automation_flags: 0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TradeSide {
    Buy,
    Sell,
}

fn market(town: &Town, ware: usize) -> Market {
    Market {
        trading: town.more_flags() & 0x100 != 0,
        stock: town.stock(ware),
        thresholds: std::array::from_fn(|i| town.threshold(ware, i)),
        marked_ware: town.u8(0x6f0),
    }
}

/// `fit_buy_amount_to_budget`: la búsqueda ascendente exige precio estrictamente menor al presupuesto.
pub fn fit_budget(ware: usize, market: &Market, budget: i32, amount: &mut i32, price: &mut i32) {
    if budget <= 0 {
        *price = 0;
        *amount = 0;
        return;
    }
    let maximum = *amount;
    let lot = if SMALL_LOT_WARES[ware] { 200 } else { 2000 };
    let mut candidate = *amount;
    loop {
        // El original usa helpers de multiplicación y división de 64 bits con signo.
        let ratio = (candidate as i64).wrapping_mul(budget as i64) / (*price as i64);
        candidate = (ratio as i32 / lot).wrapping_mul(lot);
        if candidate == 0 {
            *price = 0;
            *amount = 0;
            return;
        }
        let calculated = buy_price(ware, market, candidate);
        if calculated <= budget {
            *price = calculated;
            let mut accepted = candidate;
            candidate = candidate.wrapping_add(lot);
            while candidate <= maximum {
                let p = buy_price(ware, market, candidate);
                if p >= budget {
                    break;
                }
                *price = p;
                accepted = candidate;
                candidate = candidate.wrapping_add(lot);
            }
            *amount = accepted;
            return;
        }
    }
}

/// El precio de adquisición de oficina guarda cada intermedio `f64`, después redondea a `f32`.
pub fn add_stock_with_cost(office: &mut OfficeStock, ware: usize, amount: i32, cost: i32) {
    let old = office.stock[ware];
    let new = old.wrapping_add(amount);
    office.stock[ware] = new;
    let value = (old as f64 * office.average_price[ware] as f64) + cost as f64;
    if new > 0 {
        office.average_price[ware] = (value / new as f64) as f32;
    }
}

/// Venta de oficina a ciudad. La función modifica `quantity` igual que el registro de operación.
pub fn sell_office<F: FnMut(TradeSide, u16, u8, usize, i32, i32)>(
    town: &mut Town,
    office: &mut OfficeStock,
    merchant_gold: Option<&mut i32>,
    ware: usize,
    quantity: &mut i32,
    difficulty: f32,
    mut record_trade: F,
) {
    if ware >= 24 || *quantity <= 0 {
        return;
    }
    if office.stock[ware] < *quantity {
        *quantity = office.stock[ware];
    }
    if let Some(gold) = merchant_gold {
        let price = sell_price(ware, &market(town, ware), *quantity, difficulty);
        *gold = gold.wrapping_add(price);
        record_trade(TradeSide::Sell, office.owner, office.town, ware, price, *quantity);
    }
    office.stock[ware] = office.stock[ware].wrapping_sub(*quantity);
    town.set_stock(ware, town.stock(ware).wrapping_add(*quantity));
}

/// Compra de ciudad a oficina. El llamante suministra el oro del dueño y la ciudad validada.
pub fn buy_office<F: FnMut(TradeSide, u16, u8, usize, i32, i32)>(
    town: &mut Town,
    office: &mut OfficeStock,
    merchant_gold: &mut i32,
    ware: usize,
    quantity: &mut i32,
    mut record_trade: F,
) {
    if ware >= 24 || *quantity <= 0 {
        return;
    }
    let available = town.stock(ware);
    if available < *quantity {
        let lot = if SMALL_LOT_WARES[ware] { 200 } else { 2000 };
        *quantity = (available / lot).wrapping_mul(lot);
    }
    let mut price = buy_price(ware, &market(town, ware), *quantity);
    if *merchant_gold < price {
        fit_budget(ware, &market(town, ware), *merchant_gold, quantity, &mut price);
        if *quantity <= 0 {
            return;
        }
    }
    add_stock_with_cost(office, ware, *quantity, price);
    town.set_stock(ware, town.stock(ware).wrapping_sub(*quantity));
    *merchant_gold = merchant_gold.wrapping_sub(price);
    record_trade(TradeSide::Buy, office.owner, office.town, ware, price, *quantity);
}

/// Conecta una operación con el registro de ventas/compras y reputación del comerciante de C06.
/// El comerciante debe ser el dueño de la oficina y el índice de ciudad debe existir.
pub fn office_trade_with_reputation(
    towns: &mut [Town],
    office: &mut OfficeStock,
    merchant: &mut crate::ai::reputation::Merchant,
    side: TradeSide,
    ware: usize,
    quantity: &mut i32,
    difficulty: f32,
) {
    use crate::ai::reputation::RepEnv;
    let index = office.town as usize;
    // La venta registra reputación antes de aumentar el stock de la ciudad; la compra, después de restarlo.
    let before = if side == TradeSide::Sell {
        Some(towns[index].clone())
    } else {
        None
    };
    let mut gold = merchant.i32(0);
    let mut event = None;
    let mut capture = |side, owner, town, ware, price, qty| event = Some((side, owner, town, ware, price, qty));
    match side {
        TradeSide::Buy => buy_office(&mut towns[index], office, &mut gold, ware, quantity, &mut capture),
        TradeSide::Sell => sell_office(
            &mut towns[index],
            office,
            Some(&mut gold),
            ware,
            quantity,
            difficulty,
            &mut capture,
        ),
    }
    merchant.set_i32(0, gold);
    if let Some((side, _, town, ware, price, qty)) = event {
        let mut environment_towns = towns.to_vec();
        if let Some(before) = before {
            environment_towns[index] = before;
        }
        let env = RepEnv {
            town_count: environment_towns.len() as u32,
            towns: &environment_towns,
            renown: [0; 3],
            byte_52c: 0,
        };
        match side {
            TradeSide::Buy => merchant.on_goods_bought_from_town(&env, town as u32, ware as u32, price, qty, 0),
            TradeSide::Sell => merchant.on_goods_sold_to_town(&env, town as u32, ware as u32, price, qty, 0),
        }
    }
}

/// Compra automática de insumos de negocios (`Office::buy_business_inputs`, 0x004FFC20).
/// El original factura la cantidad pedida antes de limitarla a las existencias de la ciudad.
pub fn buy_business_inputs(
    towns: &mut [Town],
    office: &mut OfficeStock,
    account: &mut super::taxes::OfficeAccounts,
    merchant: &mut crate::ai::reputation::Merchant,
) {
    use crate::ai::reputation::RepEnv;
    let city = office.town as usize;
    for ware in (0..24).rev() {
        let have = office.stock[ware];
        let need = office.business_consumption[ware];
        if need <= have {
            continue;
        }
        let requested = need.wrapping_sub(have);
        let price = buy_price(ware, &market(&towns[city], ware), requested);
        let ceiling =
            (requested as f32 as f64) * (super::prices::BASE_PRICE[ware] as f64) * (f32::from_bits(0x3fa66666) as f64);
        if ceiling < (price as f32 as f64) || merchant.i32(0) < price {
            continue;
        }
        account.current[1] = account.current[1].wrapping_add(price);
        merchant.set_i32(0x4b4, merchant.i32(0x4b4).wrapping_add(price));
        merchant.set_i32(0, merchant.i32(0).wrapping_sub(price));
        let env = RepEnv {
            town_count: towns.len() as u32,
            towns,
            renown: [0; 3],
            byte_52c: 0,
        };
        merchant.on_goods_bought_from_town(&env, city as u32, ware as u32, price, requested, 1);
        let stock = towns[city].stock(ware);
        let taken = if requested > stock {
            towns[city].set_stock(ware, 0);
            stock
        } else {
            towns[city].set_stock(ware, stock.wrapping_sub(requested));
            requested
        };
        office.stock[ware] = have.wrapping_add(taken);
    }
}

/// Comercio automático de oficina de IA (`0x004FFA30`); el original solo recorre 20 bienes.
pub fn ai_office_trade(
    towns: &mut [Town],
    office: &mut OfficeStock,
    account: &mut super::taxes::OfficeAccounts,
    merchant: &mut crate::ai::reputation::Merchant,
    difficulty: f32,
) {
    use crate::ai::reputation::RepEnv;
    let city = office.town as usize;
    for ware in 0..20 {
        let have = office.stock[ware];
        let target = office.ai_target[ware];
        let stock = towns[city].stock(ware);
        if target > have {
            let need = target.wrapping_sub(have);
            let available = stock.wrapping_sub(towns[city].threshold(ware, 0));
            let quantity = if need > available {
                if available <= 0 {
                    continue;
                }
                available
            } else {
                need
            };
            let price = buy_price(ware, &market(&towns[city], ware), quantity);
            account.current[1] = account.current[1].wrapping_add(price);
            merchant.set_i32(0x4b4, merchant.i32(0x4b4).wrapping_add(price));
            merchant.set_i32(0, merchant.i32(0).wrapping_sub(price));
            let env = RepEnv {
                town_count: towns.len() as u32,
                towns,
                renown: [0; 3],
                byte_52c: 0,
            };
            merchant.on_goods_bought_from_town(&env, city as u32, ware as u32, price, quantity, 1);
            office.stock[ware] = have.wrapping_add(quantity);
            towns[city].set_stock(ware, stock.wrapping_sub(quantity));
        } else {
            let excess = have.wrapping_sub(target);
            let t1 = towns[city].threshold(ware, 1);
            let market_amount = if t1 > stock {
                t1.wrapping_sub(stock).min(excess)
            } else {
                0
            };
            let market_price = if t1 > stock {
                sell_price(ware, &market(&towns[city], ware), market_amount, difficulty)
            } else {
                0
            };
            let remainder = excess.wrapping_sub(market_amount);
            let base = crate::numeric::ftol_i32((remainder as f32 as f64) * (super::prices::BASE_PRICE[ware] as f64));
            let price = market_price.wrapping_add(base);
            let room = towns[city].threshold(ware, 3).wrapping_sub(stock);
            if room > 0 {
                towns[city].set_stock(ware, stock.wrapping_add(room.min(excess)));
            }
            office.stock[ware] = have.wrapping_sub(excess);
            merchant.set_i32(0, merchant.i32(0).wrapping_add(price));
            merchant.set_i32(0x4b0, merchant.i32(0x4b0).wrapping_add(price));
            let env = RepEnv {
                town_count: towns.len() as u32,
                towns,
                renown: [0; 3],
                byte_52c: 0,
            };
            merchant.on_goods_sold_to_town(&env, city as u32, ware as u32, price, excess, 1);
        }
    }
}

/// Rama comercial de `Office::daily_tick` en 0x00500181–0x005001D5.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OfficeCommercePhase {
    pub flags: u16,
    pub ai: bool,
    pub inputs: bool,
    pub administrator: bool,
}

pub fn office_commerce_phase(flags: u16, human_owner: bool) -> OfficeCommercePhase {
    if flags & 0x10 != 0 {
        if human_owner {
            OfficeCommercePhase {
                flags: flags & !0x10,
                ai: false,
                inputs: false,
                administrator: false,
            }
        } else {
            OfficeCommercePhase {
                flags,
                ai: true,
                inputs: false,
                administrator: false,
            }
        }
    } else {
        OfficeCommercePhase {
            flags,
            ai: false,
            inputs: flags & 2 != 0,
            administrator: flags & 1 != 0,
        }
    }
}

/// Comercio del administrador (`0x004FF780`), con límites de precio por mercancía.
pub fn administrator_trade(
    towns: &mut [Town],
    office: &mut OfficeStock,
    account: &mut super::taxes::OfficeAccounts,
    merchant: &mut crate::ai::reputation::Merchant,
    difficulty: f32,
) {
    use super::prices::{buy_stock_limit, sell_stock_limit};
    use crate::ai::reputation::RepEnv;
    let city = office.town as usize;
    let factor = office
        .administrator_experience
        .map(|x| ((50u8.wrapping_sub(x / 43)).wrapping_mul(2)) as i32)
        .unwrap_or(100);
    for ware in (0..24).rev() {
        let limit = office.administrator_limit[ware];
        let have = office.stock[ware];
        let target = office.ai_target[ware];
        if limit > 0 {
            let excess = have.wrapping_sub(target);
            if excess <= 0 {
                continue;
            }
            let maximum = sell_stock_limit(ware, &market(&towns[city], ware), limit, difficulty);
            let room = maximum.wrapping_sub(towns[city].stock(ware));
            if room <= 0 {
                continue;
            }
            let quantity = room.min(excess);
            let price = sell_price(ware, &market(&towns[city], ware), quantity, difficulty);
            towns[city].set_stock(ware, towns[city].stock(ware).wrapping_add(quantity));
            office.stock[ware] = have.wrapping_sub(quantity);
            merchant.set_i32(0, merchant.i32(0).wrapping_add(price));
            account.current[1] = account.current[1].wrapping_sub(price);
            let env = RepEnv {
                town_count: towns.len() as u32,
                towns,
                renown: [0; 3],
                byte_52c: 0,
            };
            merchant.on_goods_sold_to_town(&env, city as u32, ware as u32, price, quantity, 1);
        } else if limit < 0 {
            let need = target.wrapping_sub(have);
            if need <= 0 {
                continue;
            }
            let threshold = buy_stock_limit(ware, &market(&towns[city], ware), limit.wrapping_neg(), 0);
            let stock = towns[city].stock(ware);
            let available = stock.wrapping_sub(threshold);
            if available <= 0 {
                continue;
            }
            let quantity = available.min(need).min(stock);
            let price = buy_price(ware, &market(&towns[city], ware), quantity);
            let discounted = if price >= 0x1000000 {
                (price / 100).wrapping_mul(factor)
            } else {
                price.wrapping_mul(factor) / 100
            };
            if merchant.i32(0) < discounted {
                continue;
            }
            towns[city].set_stock(ware, stock.wrapping_sub(quantity));
            // El original divide sin comprobar el denominador; los estados ensayados tienen total positivo.
            add_stock_with_cost(office, ware, quantity, discounted);
            account.current[1] = account.current[1].wrapping_add(discounted);
            merchant.set_i32(0, merchant.i32(0).wrapping_sub(discounted));
            let env = RepEnv {
                town_count: towns.len() as u32,
                towns,
                renown: [0; 3],
                byte_52c: 0,
            };
            merchant.on_goods_bought_from_town(&env, city as u32, ware as u32, discounted, quantity, 1);
        }
    }
}
