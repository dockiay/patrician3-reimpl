//! Reputación y prestigio del comerciante (C06): métodos de `Merchant` de 0x004F8690 a 0x004F9550.
//!
//! La reputación por ciudad son bloques de 12 bytes desde `+0x11C`: edificios (`+0`), social (`+4`, desde `+0x120`)
//! y comercio (`+8`, desde `+0x124`), todos f32. El x87 trabaja a 53 bits y redondea a f32 en cada `fstp`; aquí cada
//! operación entre `fld` y `fstp` se hace en f64 y se redondea una vez al guardar. Una operación inválida
//! (0/0, inf·0) devuelve el NaN «indefinido» del x87 (`0xFFC00000` en f32).

use crate::town::Town;

const BUILDINGS: usize = 0x11C;
const SOCIAL: usize = 0x120;
const TRADE: usize = 0x124;
const BASE_FACTOR: usize = 0x464;
const BLOCK: usize = 12;
/// Ciudades que recorren los eventos de prestigio (bloques fijos de `+0x11C`/`+0x120`).
const FIXED_TOWNS: usize = 40;

fn c(bits: u32) -> f64 {
    f32::from_bits(bits) as f64
}
const ONE: u32 = 0x3F80_0000;
const TWO: u32 = 0x4000_0000;
const THREE: u32 = 0x4040_0000;
const FOUR: u32 = 0x4080_0000;
const SIX: u32 = 0x40C0_0000;
const EIGHT: u32 = 0x4100_0000;
const TEN: u32 = 0x4120_0000;
const HALF: u32 = 0x3F00_0000;
const ONE_HALF: u32 = 0x3FC0_0000;
const TENTH: u32 = 0x3DCC_CCCD;
const HUNDREDTH: u32 = 0x3C23_D70A;
const K_CHURCH: u32 = 966_609_234;
const K_BRIBE: u32 = 953_267_991;
const K_LOAN: u32 = 928_102_167;
const K_PIRATE_DELIVERY: u32 = 3_192_704_205;
const K_GOODS: u32 = 944_879_383;
const K_HULL: u32 = 1_028_443_341;
const K_HULL_DIV: u32 = 1_160_708_096;
/// Constantes de `0x00672E04`: factor base según la dificultad módulo 3.
const BASE_FACTORS: [u32; 3] = [1_067_869_798, 1_065_353_216, 1_060_320_051];
/// Filas de 12 bytes de `0x00672E10` (una por mercancía, tres pesos).
// Las cuatro filas adicionales son lecturas de datos adyacentes del original, no pesos acreditados.
const ROWS: [[u32; 3]; 24] = [
    [0x3FAAAAAB, 0x402AAAAB, 0x40AAAAAB],
    [0x40800000, 0x402AAAAB, 0x402AAAAB],
    [0x3FAAAAAB, 0x40800000, 0x40800000],
    [0x3FAAAAAB, 0x40800000, 0x40800000],
    [0x3FAAAAAB, 0x3FAAAAAB, 0x3FAAAAAB],
    [0x40000000, 0x3FAAAAAB, 0x3F2AAAAB],
    [0x40000000, 0x3FAAAAAB, 0x3F2AAAAB],
    [0x40800000, 0x3FAAAAAB, 0x00000000],
    [0x40800000, 0x402AAAAB, 0x3FAAAAAB],
    [0x40000000, 0x3FAAAAAB, 0x00000000],
    [0x40000000, 0x402AAAAB, 0x402AAAAB],
    [0x402AAAAB, 0x402AAAAB, 0x40800000],
    [0x3FAAAAAB, 0x3FAAAAAB, 0x3FAAAAAB],
    [0x3FAAAAAB, 0x3FAAAAAB, 0x402AAAAB],
    [0x3FAAAAAB, 0x40800000, 0x402AAAAB],
    [0x3F2AAAAB, 0x3EAAAAAB, 0x3EAAAAAB],
    [0x3FAAAAAB, 0x3FAAAAAB, 0x3FAAAAAB],
    [0x3FAAAAAB, 0x3FAAAAAB, 0x3FAAAAAB],
    [0x40000000, 0x3FAAAAAB, 0x402AAAAB],
    [0x3FAAAAAB, 0x3FAAAAAB, 0x3FAAAAAB],
    [0x42700000, 0x42200000, 0x41400000],
    [0x40E00000, 0x42180000, 0x3727C5AC],
    [0x47C35000, 0x00000000, 0xBC6A7EFA],
    [0x3F689374, 0x3FC00000, 0x399D4952],
];

/// NaN «indefinido» del x87 para las operaciones inválidas sobre operandos que no son NaN.
pub(crate) fn x87(result: f64, a: f64, b: f64) -> f64 {
    if result.is_nan() && !a.is_nan() && !b.is_nan() {
        f64::from_bits(0xFFF8_0000_0000_0000)
    } else {
        result
    }
}
pub(crate) fn add(a: f64, b: f64) -> f64 {
    x87(a + b, a, b)
}
pub(crate) fn sub(a: f64, b: f64) -> f64 {
    x87(a - b, a, b)
}
pub(crate) fn mul(a: f64, b: f64) -> f64 {
    x87(a * b, a, b)
}
pub(crate) fn div(a: f64, b: f64) -> f64 {
    x87(a / b, a, b)
}

/// Bytes del comerciante (`0x650`).
#[derive(Clone, Debug)]
pub struct Merchant {
    pub bytes: Vec<u8>,
}

/// Lo que leen de fuera estos métodos.
pub struct RepEnv<'a> {
    /// `0x006DE4B0`.
    pub town_count: u32,
    pub towns: &'a [Town],
    /// i16 de `0x00672DD2`, `0x00672DD4` y `0x00672DDE`: prestigio de los eventos grande, pequeño y de pirata.
    pub renown: [i16; 3],
    /// Byte `0x006DE52C`.
    pub byte_52c: u8,
}

/// Barco de la flota para `count_active_ships_div5`.
#[derive(Clone, Copy, Debug)]
pub struct RepShip {
    pub next: u16,
    pub kind: u8,
    /// `+0x18`.
    pub hull: i32,
    /// `+0x134`.
    pub state: u16,
}

pub trait RepHost {
    /// Predicado de barco `0x00519C40`.
    fn ship_predicate(&mut self, ship: usize) -> bool;
}

impl Merchant {
    pub fn u16(&self, o: usize) -> u16 {
        u16::from_le_bytes([self.bytes[o], self.bytes[o + 1]])
    }
    pub fn set_u16(&mut self, o: usize, v: u16) {
        self.bytes[o..o + 2].copy_from_slice(&v.to_le_bytes());
    }
    pub fn i32(&self, o: usize) -> i32 {
        i32::from_le_bytes(self.bytes[o..o + 4].try_into().unwrap())
    }
    pub fn set_i32(&mut self, o: usize, v: i32) {
        self.bytes[o..o + 4].copy_from_slice(&v.to_le_bytes());
    }
    pub fn f32(&self, o: usize) -> f32 {
        f32::from_bits(self.i32(o) as u32)
    }
    pub fn set_f32(&mut self, o: usize, v: f32) {
        self.set_i32(o, v.to_bits() as i32);
    }
    fn base(&self) -> f64 {
        self.f32(BASE_FACTOR) as f64
    }
    /// `fld [base]; fadd/fsub ...; fstp` sobre un campo de reputación.
    fn rep_add(&mut self, o: usize, delta: f64) {
        let v = add(delta, self.f32(o) as f64);
        self.set_f32(o, v as f32);
    }
    fn rep_sub(&mut self, o: usize, delta: f64) {
        let v = sub(self.f32(o) as f64, delta);
        self.set_f32(o, v as f32);
    }
    fn block(base: usize, town: u32) -> usize {
        base.wrapping_add(BLOCK.wrapping_mul(town as usize))
    }

    fn saturate_add(current: i32, add: i32) -> i32 {
        if current > 0 {
            if i32::MAX.wrapping_sub(current) < add {
                i32::MAX
            } else {
                current.wrapping_add(add)
            }
        } else {
            add
        }
    }

    /// Parte común de venta/compra: suma al acumulado y reparte reputación de comercio. `half` es la compra (el tope
    /// se reduce a la mitad y solo las tres primeras mercancías duplican con la marca 0x1000).
    fn trade_reputation(
        &mut self,
        env: &RepEnv,
        town: u32,
        ware: u32,
        limit: i32,
        flag: u32,
        amount: i32,
        bought: bool,
    ) {
        let (on, off) = if bought { (0x4B4, 0x4A0) } else { (0x4B0, 0x49C) };
        let slot = if flag & 0xFF != 0 { on } else { off };
        let total = Self::saturate_add(self.i32(slot), amount);
        self.set_i32(slot, total);
        if !bought && ware >= 0x14 {
            return;
        }
        let t = &env.towns[town as usize];
        let stock = t.i32(4 + 4 * ware as usize);
        let mut cap = t.i32(0x4F4 + 16 * ware as usize);
        if bought {
            cap >>= 1;
        }
        let room = cap.wrapping_sub(stock);
        if room <= 0 {
            return;
        }
        let chosen = if limit >= room { room } else { limit };
        let mut x = chosen as f32;
        let flags = t.u32(0x2C8);
        x = div(x as f64, cap as f32 as f64) as f32;
        x = div(x as f64, t.i32(0x2D4) as f32 as f64) as f32;
        x = mul(x as f64, self.base()) as f32;
        if flags & 0x1000 != 0 && (if bought { ware <= 2 } else { ware == 0 }) {
            x = add(x as f64, x as f64) as f32;
        }
        if flags & 0x8A18 != 0 {
            x = mul(x as f64, c(ONE_HALF)) as f32;
        }
        let row = ROWS[ware as usize];
        let w2 = mul(t.i32(0x2E0) as f32 as f64, c(row[2]));
        let w1 = mul(t.i32(0x2DC) as f32 as f64, c(row[1]));
        let w0 = mul(t.i32(0x2D8) as f32 as f64, c(row[0]));
        let weight = add(add(w2, w1), w0) as f32;
        let o = Self::block(TRADE, town);
        let product = mul(weight as f64, x as f64);
        let v = if bought {
            sub(self.f32(o) as f64, product)
        } else {
            add(product, self.f32(o) as f64)
        };
        self.set_f32(o, v as f32);
    }

    /// `merchant_on_goods_sold_to_town` (0x004F8690).
    pub fn on_goods_sold_to_town(&mut self, env: &RepEnv, town: u32, ware: u32, amount: i32, limit: i32, flag: u32) {
        self.trade_reputation(env, town, ware, limit, flag, amount, false);
    }

    /// `merchant_on_goods_bought_from_town` (0x004F8840).
    pub fn on_goods_bought_from_town(
        &mut self,
        env: &RepEnv,
        town: u32,
        ware: u32,
        amount: i32,
        limit: i32,
        flag: u32,
    ) {
        self.trade_reputation(env, town, ware, limit, flag, amount, true);
    }

    /// `merchant_add_buildings_reputation_well` (0x004F89E0).
    pub fn add_buildings_reputation_well(&mut self, env: &RepEnv, town: u32) {
        let t = town & 0xFF;
        let town_data = &env.towns[t as usize];
        let citizens = town_data.i32(0x2D4);
        if citizens >= 1 {
            let q = (town_data.u8(0x789) as i32 * 2500) / citizens;
            if q < 5 {
                let o = Self::block(BUILDINGS, t);
                let v = add(mul(self.base(), c(TENTH)), self.f32(o) as f64);
                self.set_f32(o, v as f32);
            }
        }
    }

    /// `merchant_add_buildings_reputation_hospital_chapel` (0x004F8A50).
    pub fn add_buildings_reputation_hospital_chapel(&mut self, env: &RepEnv, town: u32, flag: u32) {
        let t = town & 0xFF;
        let town_data = &env.towns[t as usize];
        let citizens = town_data.i32(0x2D4);
        if citizens < 1 {
            return;
        }
        let o = Self::block(BUILDINGS, t);
        if flag & 0xFF != 0 {
            let sum = town_data.u8(0x773) as i32 + town_data.u8(0x771) as i32;
            if sum * 5000 >= citizens {
                return;
            }
            self.rep_add(o, self.base());
        } else {
            let v = add(mul(self.base(), c(HUNDREDTH)), self.f32(o) as f64);
            self.set_f32(o, v as f32);
        }
    }

    /// `merchant_add_social_reputation_church_donation` (0x004F8AF0).
    pub fn add_social_reputation_church_donation(&mut self, env: &RepEnv, town: u32, amount: i32) {
        let x = amount as f32;
        let scaled = mul(x as f64, c(K_CHURCH));
        let divisor = (env.byte_52c as i32 + 1) as f32;
        let q = div(scaled, divisor as f64) as f32;
        let r = mul(q as f64, self.base()) as f32;
        let o = Self::block(SOCIAL, town & 0xFF);
        let v = add(r as f64, self.f32(o) as f64);
        self.set_f32(o, v as f32);
    }

    /// Ajuste con saturación del prestigio sin signo (`+0x14`, máximo `0xC800`).
    fn bump_renown(&mut self, delta: i16) {
        let cur = self.u16(0x14);
        let d = delta as i32;
        if d > 0 {
            let value = if cur as i32 <= 0xC800 - d {
                cur.wrapping_add(d as u16)
            } else {
                0xC800
            };
            self.set_u16(0x14, value);
        } else {
            let value = if cur as i32 > -d { cur.wrapping_add(d as u16) } else { 0 };
            self.set_u16(0x14, value);
        }
    }

    /// `merchant_add_renown_and_social_all_towns_large` (0x004F8B50).
    pub fn add_renown_and_social_all_towns_large(&mut self, env: &RepEnv) {
        self.bump_renown(env.renown[0]);
        for k in 0..FIXED_TOWNS {
            let o = Self::block(SOCIAL, k as u32);
            let v = add(self.f32(o) as f64, self.base());
            self.set_f32(o, v as f32);
        }
    }

    /// `merchant_add_renown_and_social_all_towns_small` (0x004F8BD0).
    pub fn add_renown_and_social_all_towns_small(&mut self, env: &RepEnv) {
        let half = mul(self.base(), c(HALF)) as f32;
        for k in 0..FIXED_TOWNS {
            let o = Self::block(BUILDINGS, k as u32);
            let v = add(half as f64, self.f32(o) as f64);
            self.set_f32(o, v as f32);
        }
        self.bump_renown(env.renown[1]);
    }

    /// `merchant_add_buildings_reputation_civic_building` (0x004F8C70).
    pub fn add_buildings_reputation_civic_building(&mut self, town: u32) {
        let o = Self::block(BUILDINGS, town & 0xFF);
        self.rep_add(o, self.base());
    }

    /// `merchant_apply_severe_reputation_penalty` (0x004F8CA0).
    pub fn apply_severe_reputation_penalty(&mut self, env: &RepEnv, town: u32) {
        if town < env.town_count {
            self.rep_sub(Self::block(SOCIAL, town), c(THREE));
        } else {
            for t in 0..env.town_count as usize {
                if env.towns[t].u8(0x2C8) & 0x40 != 0 {
                    self.rep_sub(Self::block(SOCIAL, t as u32), c(THREE));
                }
            }
        }
        let counter = self.bytes[0x1E];
        let v = self.u16(0x22).wrapping_add(0xFFC4);
        self.set_u16(0x22, v);
        if counter != 0 {
            self.bytes[0x1E] = counter - 1;
        }
    }

    /// `merchant_add_social_reputation_flagged_towns_small` (0x004F8D20).
    pub fn add_social_reputation_flagged_towns_small(&mut self, env: &RepEnv) {
        if (env.town_count as i32) <= 0 {
            return;
        }
        for t in (0..env.town_count as usize).rev() {
            if env.towns[t].u8(0x2C8) & 0x40 != 0 {
                self.rep_add(Self::block(SOCIAL, t as u32), c(TENTH));
            }
        }
    }

    /// `merchant_subtract_social_reputation_demolition` (0x004F8D70).
    pub fn subtract_social_reputation_demolition(&mut self, town: u32) {
        self.rep_sub(Self::block(SOCIAL, town & 0xFF), c(ONE));
    }

    /// `merchant_reward_pirate_ship_defeated` (0x004F8D90).
    pub fn reward_pirate_ship_defeated(&mut self, env: &RepEnv, flag: u32) {
        let delta = env.renown[2] as i32;
        let cur = self.u16(0x16) as i16 as i32;
        let value = if delta > 0 {
            if cur < 0x7FFF - delta {
                (cur + delta) as u16
            } else {
                0x7FFF
            }
        } else if cur + 0x8000 > -delta {
            (cur + delta) as u16
        } else {
            0x8000
        };
        self.set_u16(0x16, value);
        if flag & 0xFF != 0 {
            for t in 0..env.town_count {
                let o = Self::block(SOCIAL, t);
                let v = add(self.f32(o) as f64, self.base());
                self.set_f32(o, v as f32);
            }
        }
    }

    /// `merchant_add_social_reputation_bath_house_bribe_success` (0x004F8E30).
    pub fn add_social_reputation_bath_house_bribe_success(&mut self, town: u32, amount: i32) {
        let mut x = mul(amount as f32 as f64, c(K_BRIBE)) as f32;
        if x > 3.0 {
            x = 3.0;
        }
        let x = mul(x as f64, self.base()) as f32;
        let o = Self::block(SOCIAL, town);
        let v = add(x as f64, self.f32(o) as f64);
        self.set_f32(o, v as f32);
    }

    /// `merchant_subtract_social_reputation_bath_house_bribe_failure` (0x004F8E90).
    pub fn subtract_social_reputation_bath_house_bribe_failure(&mut self, town: u32) {
        self.rep_sub(Self::block(SOCIAL, town), c(TWO));
    }

    /// `merchant_add_social_reputation_celebration` (0x004F8EB0).
    pub fn add_social_reputation_celebration(&mut self, town: u32, guests: u32) {
        let delta = if guests != 0 {
            let x = guests as f32;
            mul(mul(x as f64, self.base()), c(HALF)) as f32
        } else {
            -1.0f32
        };
        let o = Self::block(SOCIAL, town);
        let v = add(delta as f64, self.f32(o) as f64);
        self.set_f32(o, v as f32);
    }

    /// Función con tabla de saltos de `0x004F8F10`: resta a la reputación social según el tipo de suceso.
    pub fn subtract_social_reputation_event(&mut self, town: u32, kind: u32) {
        if kind > 0xF {
            return;
        }
        const SELECT: [u8; 16] = [0, 1, 2, 3, 3, 5, 5, 5, 5, 0, 0, 1, 3, 0, 3, 4];
        let amount = match SELECT[kind as usize] {
            0 => TWO,
            1 => FOUR,
            2 => ONE,
            3 => SIX,
            4 => EIGHT,
            _ => return,
        };
        self.rep_sub(Self::block(SOCIAL, town), c(amount));
    }

    /// `merchant_grant_loan_to_town_with_reputation` (0x004F8FE0).
    pub fn grant_loan_to_town_with_reputation(&mut self, town: u32, level: u32, amount: i32) {
        let level_f = (4u32.wrapping_sub(level)) as f32;
        let gold = self.i32(0).wrapping_sub(amount);
        self.set_i32(0, gold);
        let a = amount as f32;
        let p = mul(mul(level_f as f64, a as f64), c(K_LOAN)) as f32;
        let q = mul(p as f64, self.base()) as f32;
        let o = Self::block(SOCIAL, town);
        let v = add(q as f64, self.f32(o) as f64);
        self.set_f32(o, v as f32);
    }

    /// `merchant_apply_ship_delivery_outcome` (0x004F9050).
    pub fn apply_ship_delivery_outcome(&mut self, env: &RepEnv, n: u32, success: u32) {
        let x = n as f32;
        let (factor, points) = if success != 0 {
            (TENTH, n.wrapping_mul(20))
        } else {
            (K_PIRATE_DELIVERY, n.wrapping_mul(15).wrapping_neg().wrapping_mul(2))
        };
        let p = mul(x as f64, c(factor)) as f32;
        let y = mul(p as f64, self.base()) as f32;
        let score = self.u16(0x22).wrapping_add(points as u16);
        self.set_u16(0x22, score);
        for t in 0..env.town_count {
            let o = Self::block(SOCIAL, t);
            let v = add(y as f64, self.f32(o) as f64);
            self.set_f32(o, v as f32);
        }
        let sum = (self.bytes[0x20] as i8 as i32).wrapping_add(n as i32);
        self.bytes[0x20] = if sum > 0x7F {
            0x7F
        } else if sum < -0x7F {
            0x81
        } else {
            sum as u8
        };
    }

    /// `merchant_add_social_reputation_ship_hull_work` (0x004F9120).
    pub fn add_social_reputation_ship_hull_work(&mut self, town: u32, hull: u32) {
        let x = hull as f32;
        let r = div(mul(mul(x as f64, self.base()), c(K_HULL)), c(K_HULL_DIV)) as f32;
        let o = Self::block(SOCIAL, town);
        let v = add(r as f64, self.f32(o) as f64);
        self.set_f32(o, v as f32);
    }

    /// `merchant_subtract_buildings_reputation_flagged_towns` (0x004F9170).
    pub fn subtract_buildings_reputation_flagged_towns(&mut self, env: &RepEnv) {
        if (env.town_count as i32) <= 0 {
            return;
        }
        for t in 0..env.town_count as usize {
            if env.towns[t].u8(0x2C8) & 0xC0 != 0 {
                self.rep_sub(Self::block(BUILDINGS, t as u32), c(ONE));
            }
        }
    }

    /// `merchant_subtract_social_reputation_flagged_towns` (0x004F91C0).
    pub fn subtract_social_reputation_flagged_towns(&mut self, env: &RepEnv) {
        if env.town_count == 0 {
            return;
        }
        let mut t = 0u8;
        loop {
            if env.towns[t as usize].u8(0x2C8) & 0xC0 != 0 {
                self.rep_sub(Self::block(SOCIAL, t as u32), c(ONE_HALF));
            }
            t = t.wrapping_add(1);
            if (t as u32) >= env.town_count {
                break;
            }
        }
    }

    /// `merchant_subtract_social_reputation_town_coffers` (0x004F9220).
    pub fn subtract_social_reputation_town_coffers(&mut self, town: u32, amount: u32) {
        let o = Self::block(SOCIAL, town);
        if amount >= 1_000_000 {
            self.rep_sub(o, c(TEN));
        } else {
            let x = amount as f32;
            let v = sub(self.f32(o) as f64, mul(x as f64, c(K_BRIBE)));
            self.set_f32(o, v as f32);
        }
    }

    /// `merchant_add_social_reputation_goods_value` (0x004F9280).
    pub fn add_social_reputation_goods_value(&mut self, value: i32, town: u32) {
        let o = Self::block(SOCIAL, town & 0xFF);
        let v = add(mul(value as f32 as f64, c(K_GOODS)), self.f32(o) as f64);
        self.set_f32(o, v as f32);
    }

    /// `merchant_count_active_ships_div5` (0x004F92B0): barcos útiles del comerciante en la ciudad, entre cinco.
    pub fn count_active_ships_div5(
        &self,
        env: &RepEnv,
        ships: &[RepShip],
        ship_count: u16,
        town: u32,
        flag: u32,
        host: &mut dyn RepHost,
    ) -> u32 {
        let t = (town & 0xFF) as usize;
        let mask = ((env.towns[t].u8(0x2C8) as i8 as i32) & 1) * 2;
        let mut count = 0u32;
        let mut index = self.u16(0xE);
        while index < ship_count {
            let ship = &ships[index as usize];
            if ship.hull > 0
                && (ship.state < 4 || ship.state == 0xF || ship.state == 0x10)
                && (ship.kind as i32 & mask) == 0
            {
                if flag & 0xFF == 0 || host.ship_predicate(index as usize) {
                    count += 1;
                }
            }
            index = ship.next;
        }
        (count & 0xFFFF) / 5
    }

    /// `merchant_get_mayor_candidacy_flag` (0x004F9360).
    pub fn get_mayor_candidacy_flag(&mut self) -> u32 {
        let control = self.u16(8);
        if control == 0 {
            return self.i32(0x118) as u32;
        }
        let value = if control & 4 != 0 { 0 } else { 1 };
        self.set_i32(0x118, value);
        value as u32
    }

    /// `merchant_set_mayor_candidacy_flag` (0x004F9390).
    pub fn set_mayor_candidacy_flag(&mut self, value: u32) {
        let v = if self.u16(8) == 0 { value } else { 1 };
        self.set_i32(0x118, v as i32);
    }

    /// `get_merchant_epithet_index` (0x004F9430).
    pub fn get_merchant_epithet_index(&self) -> u8 {
        let income = self.i32(0x46C);
        let mut renown = self.bytes[0x15] as i32;
        if income >= 0x9C40 {
            let high = ((income as i64 * 0x68DB_8BAD_i64) >> 32) as i32 >> 14;
            renown += high.wrapping_add(((high as u32) >> 31) as i32);
        }
        let by_renown = renown as f64 / 200.0;
        let guild = self.bytes[0x1B] as i32;
        let by_guild = guild as f64 / 120.0;
        if by_guild > by_renown {
            match guild {
                ..=4 => 0,
                5..=9 => 8,
                10..=19 => 9,
                20..=34 => 10,
                35..=64 => 11,
                65..=89 => 12,
                90..=119 => 13,
                _ => 14,
            }
        } else {
            match renown {
                ..=9 => 0,
                10..=19 => 1,
                20..=29 => 2,
                30..=49 => 3,
                50..=99 => 4,
                100..=149 => 5,
                150..=199 => 6,
                _ => 7,
            }
        }
    }

    /// `merchant_set_base_reputation_factor_from_difficulty` (0x004F9550).
    pub fn set_base_reputation_factor_from_difficulty(&mut self, difficulty: u32) {
        if self.u16(8) == 0 {
            self.set_i32(BASE_FACTOR, BASE_FACTORS[(difficulty % 3) as usize] as i32);
        }
    }
}
