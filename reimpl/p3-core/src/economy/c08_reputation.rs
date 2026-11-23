//! C08: operaciones que mueven reputación social con dinero (donaciones y sobornos en los baños).
//!
//! - `op_donate_to_town_fund` (`0x004FE2D0`, operación 0x32): caja de la ciudad en `Town+0x794`.
//! - `handle_operation_bath_house_bribe_success` (`0x0053AC50`, operación 0x42).
//! - `handle_operation_bath_house_bribe_failure` (`0x0053AD10`, operación 0x43).
//!
//! La donación a la caja de obras de la iglesia (0x31) ya está en `buildings::donation`.
//! Contrastes con el despachador original en `p3-sim/tests/c08_reputation_live_x86.rs`.
use crate::{
    ai::reputation::{Merchant, RepEnv},
    town::Town,
};

/// Caja municipal (`Town+0x794`).
const TOWN_FUND: usize = 0x794;
/// Escaños del concejo sobornables (`Town+0x6DC..0x6DF`).
const SEATS: usize = 0x6dc;

/// `op_donate_to_town_fund`: `factor` es `GameWorld+0x8C`. El límite de la caja es `(factor + 1) * 12000`; si la
/// donación lo supera la caja queda en `límite - 1` y la reputación solo cuenta lo que cabía.
pub fn donate_to_town_fund(town: &mut Town, merchant: &mut Merchant, env: &RepEnv, town_arg: u32, mut amount: i32, factor: u8) {
    let gold = merchant.i32(0);
    if gold >= amount {
        merchant.set_i32(0, gold.wrapping_sub(amount));
    } else {
        amount = gold;
        merchant.set_i32(0, 0);
    }
    merchant.set_i32(0x4bc, merchant.i32(0x4bc).wrapping_add(amount));
    let limit = (i32::from(factor) + 1) * 12000;
    let fund = town.i32(TOWN_FUND);
    let total = fund.wrapping_add(amount);
    if total > limit {
        town.set_u32(TOWN_FUND, (limit - 1) as u32);
        merchant.add_social_reputation_church_donation(env, town_arg & 0xff, limit.wrapping_sub(fund));
    } else {
        town.set_u32(TOWN_FUND, total as u32);
        merchant.add_social_reputation_church_donation(env, town_arg & 0xff, amount);
    }
}

/// Operación 0x42: `seat` es `+4`, `merchant` `+8`, `amount` `+0xC`, `city` `+0x10`. El soborno se paga aunque deje
/// el saldo negativo; el original compara cuatro veces el mismo escaño, de modo que se lo asigna al comerciante salvo
/// que ya lo ocupara. Devuelve `Err` si el escaño queda fuera de los cuatro del concejo (el original escribiría en
/// otro campo de la ciudad).
pub fn bribe_success(towns: &mut [Town], merchants: &mut [Merchant], seat: u32, merchant: u32, amount: i32, city: u32) -> Result<bool, &'static str> {
    if city as usize >= towns.len() || merchant as usize >= merchants.len() {
        return Ok(false);
    }
    if seat >= 4 {
        return Err("escaño de soborno fuera del concejo");
    }
    let town = &mut towns[city as usize];
    let at = SEATS + seat as usize;
    let held = u32::from(town.u8(at)) == merchant;
    let m = &mut merchants[merchant as usize];
    m.set_i32(0, m.i32(0).wrapping_sub(amount));
    m.set_i32(0x4bc, m.i32(0x4bc).wrapping_add(amount));
    m.add_social_reputation_bath_house_bribe_success(city, amount);
    if !held {
        town.set_u8(at, merchant as u8);
    }
    Ok(true)
}

/// Operación 0x43: `seat` `+4`, `merchant` `+8`, `city` `+0x10`. Resta reputación y, si el escaño es del
/// comerciante o de nadie válido, lo deja en `1` como el original.
pub fn bribe_failure(towns: &mut [Town], merchants: &mut [Merchant], seat: u32, merchant: u32, city: u32, merchant_count: u16) -> bool {
    if city as usize >= towns.len() || merchant as usize >= merchants.len() {
        return false;
    }
    merchants[merchant as usize].subtract_social_reputation_bath_house_bribe_failure(city);
    if seat < 4 {
        let town = &mut towns[city as usize];
        let at = SEATS + seat as usize;
        let current = town.u8(at);
        if u32::from(current) == merchant || u16::from(current) >= merchant_count {
            town.set_u8(at, 1);
        }
    }
    true
}
