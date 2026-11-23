//! Donación a la caja de obras de la iglesia (C05): `op_donate_to_building_fund` (0x004FE420). El comerciante
//! entrega dinero (como mucho el que tiene) mientras la caja (`Town+0x798`) no llegue a su límite, que
//! depende del nivel de la iglesia (`Town+0x7A3`) y de un ajuste global.

use super::Site;
use crate::town::{off, Town};

/// Palabras `0x006734B4` para los niveles 0..2.
const LEVEL_BONUS: [i32; 3] = [20000, 40000, 60000];
const FUND_COLLECTED: usize = 0x798;
const FUND_CAPACITY: usize = 0x79C;

/// Consultas fuera de la ciudad.
pub trait DonationHost {
    /// `get_merchant` (0x005303C0).
    fn get_merchant(&mut self, merchant: u32);
    /// `merchant_add_social_reputation_church_donation` (0x004F8AF0): (ciudad, cantidad entregada).
    fn add_church_donation_reputation(&mut self, town: u8, amount: i32);
}

/// Entorno global y comerciante que dona.
pub struct DonationEnv<'a> {
    /// Byte `0x006DE52C` (`GameWorld+0x8C`).
    pub setting: u8,
    /// Dinero (`+0`) y donaciones recientes (`+0x4BC`) del comerciante.
    pub merchant_gold: &'a mut i32,
    pub merchant_donations: &'a mut i32,
}

impl Site {
    /// `op_donate_to_building_fund`: `merchant` es el índice del comerciante; `town_byte` y `amount`, los otros
    /// dos argumentos de la operación.
    pub fn donate_to_building_fund(
        t: &mut Town,
        merchant: u32,
        town_byte: u8,
        mut amount: i32,
        env: &mut DonationEnv,
        host: &mut dyn DonationHost,
    ) {
        host.get_merchant(merchant);
        let level = t.u8(off::CHURCH_LEVEL);
        let multiplier = match level {
            0 => 10000,
            1 => 15000,
            _ => 20000,
        };
        if level >= 3 {
            return;
        }
        let limit = (env.setting as i32)
            .wrapping_mul(multiplier)
            .wrapping_add(LEVEL_BONUS[level as usize]);
        if t.i32(FUND_COLLECTED) >= limit || t.u32(FUND_CAPACITY) <= 100 {
            return;
        }
        if *env.merchant_gold >= amount {
            *env.merchant_gold -= amount;
        } else {
            amount = *env.merchant_gold;
            *env.merchant_gold = 0;
        }
        *env.merchant_donations = env.merchant_donations.wrapping_add(amount);
        let collected = t.i32(FUND_COLLECTED).wrapping_add(amount);
        t.set_u32(FUND_COLLECTED, collected as u32);
        if collected > limit {
            amount = amount.wrapping_sub(collected.wrapping_sub(limit));
            t.set_u32(FUND_COLLECTED, limit as u32);
        }
        host.add_church_donation_reputation(town_byte, amount);
    }
}
