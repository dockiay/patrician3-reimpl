//! Comercio entre barcos y ciudades; el llamante valida los índices de la flota.
use super::prices::{buy_price, sell_price, Market, SMALL_LOT_WARES};
use super::trade::{fit_budget, TradeSide};
use crate::ai::reputation::{Merchant, RepEnv};
use crate::numeric::ftol_i32;
use crate::town::Town;

const CREW_BASE: [u16; 4] = [10, 16, 30, 24];
const GUN_WEIGHT: [i32; 6] = [1000, 1000, 2000, 2000, 2000, 1000];
const GUN_VALUE: [i32; 6] = [9, 10, 22, 24, 30, 18];

#[derive(Clone, Debug, PartialEq)]
pub struct ShipCargo {
    pub owner: u8,
    pub town: u8,
    pub kind: u8,
    pub crew: u16,
    pub marines: u8,
    pub cutlasses: u16,
    pub capacity: i32,
    pub stock: [i32; 24],
    pub average_price: [f32; 24],
    pub guns: [u8; 24],
    pub cargo_value: i32,
    pub cargo_weight: i32,
    pub equipment_weight: i32,
    pub gun_power: i32,
    pub dirty: u16,
}
impl Default for ShipCargo {
    fn default() -> Self {
        Self {
            owner: 0,
            town: 0,
            kind: 0,
            crew: 0,
            marines: 0,
            cutlasses: 0,
            capacity: 0,
            stock: [0; 24],
            average_price: [0.0; 24],
            guns: [0xff; 24],
            cargo_value: 0,
            cargo_weight: 0,
            equipment_weight: 0,
            gun_power: 0,
            dirty: 0,
        }
    }
}
impl ShipCargo {
    /// 0x005182B0: ajusta lotes, calcula pesos/valor y devuelve capacidad libre sin redondear.
    pub fn recompute_load(&mut self) -> i32 {
        let mut weight = 0i32;
        let mut value = 0i32;
        for ware in (0..24).rev() {
            let lot = if ware >= 20 {
                10
            } else if SMALL_LOT_WARES[ware] {
                200
            } else {
                2000
            };
            let stock = self.stock[ware];
            let rounded = stock.wrapping_sub(stock % lot);
            self.stock[ware] = rounded;
            weight = weight.wrapping_add(rounded);
            value = value.wrapping_add(ftol_i32((rounded as f32 as f64) * (self.average_price[ware] as f64)));
        }
        weight = weight.wrapping_add((self.marines as i32).wrapping_mul(2000));
        let extra = self.crew.saturating_sub(CREW_BASE[(self.kind & 3) as usize]);
        let mut equipment = (extra as i32).wrapping_mul(400);
        let mut power = 0i32;
        for &gun in &self.guns {
            if gun < 6 {
                equipment = equipment.wrapping_add(GUN_WEIGHT[gun as usize]);
                power = power.wrapping_add(GUN_VALUE[gun as usize]);
            }
        }
        equipment = equipment.wrapping_add((self.cutlasses as i32).wrapping_mul(10));
        self.cargo_value = value;
        self.equipment_weight = equipment;
        self.gun_power = power;
        self.cargo_weight = (weight.wrapping_add(199) / 200).wrapping_mul(200);
        self.capacity.wrapping_sub(equipment).wrapping_sub(weight)
    }
    /// 0x00517FE0: añade una cantidad positiva y recalcula el precio medio por lote.
    pub fn add_with_cost(&mut self, ware: usize, amount: i32, cost: i32) {
        if amount <= 0 {
            return;
        }
        self.dirty |= 4;
        let old = self.stock[ware];
        let total = old.wrapping_add(amount);
        let lot = if !SMALL_LOT_WARES[ware] {
            2000
        } else if ware < 20 {
            200
        } else {
            10
        };
        let rounded = (total / lot).wrapping_mul(lot);
        self.stock[ware] = rounded;
        let sum = (old as f64) * (self.average_price[ware] as f64) + (cost as f64);
        self.average_price[ware] = (sum / (rounded as f64)) as f32;
    }
}

fn market(t: &Town, w: usize) -> Market {
    Market {
        trading: t.more_flags() & 0x100 != 0,
        stock: t.stock(w),
        thresholds: std::array::from_fn(|i| t.threshold(w, i)),
        marked_ware: t.u8(0x6f0),
    }
}

/// Operación de barco-ciudad con reputación. `town` debe estar en 0..24 y el propietario existir.
/// La propagación del bit 4 al convoy se comunica mediante `convoy_dirty`; `None` indica sin convoy.
pub fn trade_ship(
    towns: &mut [Town],
    ship: &mut ShipCargo,
    merchant: &mut Merchant,
    town: usize,
    ware: usize,
    amount: &mut i32,
    side: TradeSide,
    difficulty: f32,
    mut convoy_dirty: Option<&mut u32>,
) {
    if town >= towns.len() || town >= 32 || ware >= 24 {
        return;
    }
    if (merchant.i32(0x608) as u32) & (1u32 << town) != 0 {
        return;
    }
    if side == TradeSide::Buy && ship.town as usize != town {
        return;
    }
    let before = if side == TradeSide::Sell {
        Some(towns[town].clone())
    } else {
        None
    };
    let price;
    match side {
        TradeSide::Sell => {
            if ship.stock[ware] < *amount {
                *amount = ship.stock[ware];
                ship.stock[ware] = 0;
            } else {
                ship.stock[ware] = ship.stock[ware].wrapping_sub(*amount);
            }
            ship.recompute_load();
            ship.dirty |= 4;
            if let Some(flag) = convoy_dirty.as_mut() {
                **flag |= 4;
            }
            price = sell_price(ware, &market(&towns[town], ware), *amount, difficulty);
            merchant.set_i32(0, merchant.i32(0).wrapping_add(price));
            // La reputación se calcula antes de añadir la mercancía a la ciudad.
            let mut view = towns.to_vec();
            view[town] = before.unwrap();
            let env = RepEnv {
                town_count: view.len() as u32,
                towns: &view,
                renown: [0; 3],
                byte_52c: 0,
            };
            merchant.on_goods_sold_to_town(&env, town as u32, ware as u32, price, *amount, 0);
            towns[town].set_stock(ware, towns[town].stock(ware).wrapping_add(*amount));
        }
        TradeSide::Buy => {
            let free = ship.recompute_load();
            let available = free.min(towns[town].stock(ware));
            if available < *amount {
                if available < 0 {
                    return;
                }
                let lot = if SMALL_LOT_WARES[ware] { 200 } else { 2000 };
                *amount = (available / lot).wrapping_mul(lot);
            }
            let mut cost = buy_price(ware, &market(&towns[town], ware), *amount);
            if merchant.i32(0) < cost {
                fit_budget(ware, &market(&towns[town], ware), merchant.i32(0), amount, &mut cost);
            }
            if *amount <= 0 {
                return;
            }
            price = cost;
            merchant.set_i32(0, merchant.i32(0).wrapping_sub(price));
            ship.add_with_cost(ware, *amount, price);
            if let Some(flag) = convoy_dirty.as_mut() {
                **flag |= 4;
            }
            ship.recompute_load();
            towns[town].set_stock(ware, towns[town].stock(ware).wrapping_sub(*amount));
            let env = RepEnv {
                town_count: towns.len() as u32,
                towns,
                renown: [0; 3],
                byte_52c: 0,
            };
            merchant.on_goods_bought_from_town(&env, town as u32, ware as u32, price, *amount, 0);
        }
    }
}

/// Estado del convoy necesario para 0x00539300.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ConvoyTrade {
    pub first_ship: u16,
    pub town: u16,
    pub dirty: u32,
}

/// Compra o venta agregada de convoy. `next` reproduce la cadena `ship+6`; los índices y el dueño
/// deben estar validados por el anfitrión. La cantidad del registro de operación no se modifica.
pub fn trade_convoy(
    towns: &mut [Town],
    ships: &mut [ShipCargo],
    next: &[u16],
    convoy: &mut ConvoyTrade,
    merchant: &mut Merchant,
    owner: u16,
    town: usize,
    ware: usize,
    amount: i32,
    side: TradeSide,
    difficulty: f32,
) {
    if ware >= 24 || town >= towns.len() || convoy.town as usize != town {
        return;
    }
    let mut current = convoy.first_ship;
    if side == TradeSide::Sell {
        let mut remaining = amount;
        let mut sold = 0i32;
        while (current as usize) < ships.len() && remaining > 0 {
            let i = current as usize;
            if ships[i].owner as u16 == owner {
                let have = ships[i].stock[ware];
                if have > remaining {
                    ships[i].stock[ware] = have.wrapping_sub(remaining);
                    sold = sold.wrapping_add(remaining);
                    remaining = 0;
                } else {
                    remaining = remaining.wrapping_sub(have);
                    sold = sold.wrapping_add(have);
                    ships[i].stock[ware] = 0;
                }
                ships[i].recompute_load();
                ships[i].dirty |= 4;
            }
            current = next[i];
        }
        let price = sell_price(ware, &market(&towns[town], ware), sold, difficulty);
        let env = RepEnv {
            town_count: towns.len() as u32,
            towns,
            renown: [0; 3],
            byte_52c: 0,
        };
        merchant.on_goods_sold_to_town(&env, town as u32, ware as u32, price, sold, 0);
        merchant.set_i32(0, merchant.i32(0).wrapping_add(price));
        towns[town].set_stock(ware, towns[town].stock(ware).wrapping_add(sold));
        convoy.dirty |= 4;
        return;
    }
    if merchant.i32(0) <= 0 {
        return;
    }
    let mut free_total = 0i32;
    while (current as usize) < ships.len() {
        let i = current as usize;
        if ships[i].owner as u16 == owner {
            let free = ships[i].recompute_load();
            if free > 0 {
                free_total = free_total.wrapping_add(free);
            }
        }
        current = next[i];
    }
    let mut remaining = amount.min(free_total).min(towns[town].stock(ware));
    current = convoy.first_ship;
    let mut follow = true;
    while (current as usize) < ships.len() && follow {
        let i = current as usize;
        if ships[i].owner as u16 == owner {
            let free = ships[i].recompute_load();
            let mut quantity = free;
            if remaining < quantity {
                quantity = remaining;
                follow = false;
            }
            if quantity > 0 {
                let mut price = buy_price(ware, &market(&towns[town], ware), quantity);
                if price > merchant.i32(0) {
                    follow = false;
                    loop {
                        let ratio = merchant
                            .i32(0)
                            .wrapping_mul(quantity)
                            .checked_div(price)
                            .expect("división por cero o desbordamiento x86");
                        let lot = if SMALL_LOT_WARES[ware] { 200 } else { 2000 };
                        quantity = (ratio / lot).wrapping_mul(lot);
                        price = buy_price(ware, &market(&towns[town], ware), quantity);
                        if price <= merchant.i32(0) {
                            break;
                        }
                    }
                }
                let lot = if SMALL_LOT_WARES[ware] { 200 } else { 2000 };
                quantity = (quantity / lot).wrapping_mul(lot);
                if quantity > 0 && price >= 0 {
                    let env = RepEnv {
                        town_count: towns.len() as u32,
                        towns,
                        renown: [0; 3],
                        byte_52c: 0,
                    };
                    merchant.on_goods_bought_from_town(&env, town as u32, ware as u32, price, quantity, 0);
                    merchant.set_i32(0, merchant.i32(0).wrapping_sub(price));
                    towns[town].set_stock(ware, towns[town].stock(ware).wrapping_sub(quantity));
                    ships[i].add_with_cost(ware, quantity, price);
                    remaining = remaining.wrapping_sub(quantity);
                    ships[i].recompute_load();
                    convoy.dirty |= 4;
                }
            }
        }
        current = next[i];
    }
}
