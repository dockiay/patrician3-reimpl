//! Cobro del suelo y cierres de la contabilidad de ciudades y oficinas (C04).
//!
//! Los vectores de `Accounts` usan los mismos índices que las tablas de `EconomyWorld`.
//! La reputación del alcalde sigue siendo un puerto explícito: el estado reducido de comerciante
//! en `EconomyWorld` todavía no incluye las tablas de reputación de `ai::reputation`.
use super::militia::book_transaction;
use super::world::EconomyWorld;
use crate::town::off;

/// Campos adicionales del comerciante que escribe `office_charge_land_tax`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MerchantAccounts {
    pub expenses_4a4: i32,
    pub expenses_4a8: i32,
    pub houses_4d4: i32,
}

/// Campos de la oficina no incluidos todavía en `economy::world::Office`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct OfficeAccounts {
    pub owner: u16,
    pub town: u8,
    pub business_buildings: u16,
    pub houses: u16,
    /// `+0` y `+2`, contadores de almacén actual y previo.
    pub storage: u16,
    pub previous_storage: u16,
    /// Pares actuales y anteriores en +418/41C, +420/424, +428/42C,
    /// +430/434, +438/43C (impuesto del suelo) y +440/444.
    pub current: [i32; 6],
    pub previous: [i32; 6],
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Accounts {
    pub offices: Vec<OfficeAccounts>,
    pub merchants: Vec<MerchantAccounts>,
}

/// Cierra las once cuentas mensuales. La reputación se delega al sistema de comerciantes.
pub fn close_town_month<F: FnMut(u16, u8, u32)>(world: &mut EconomyWorld, town: usize, reputation: &mut F) {
    let t = &mut world.towns[town];
    let mayor = t.u8(off::MAYOR_ID) as u16;
    if mayor < world.merchant_count {
        let amount = t
            .i32(off::TRANSACTIONS + 0x50)
            .wrapping_sub(t.i32(off::TRANSACTIONS + 0x38))
            .wrapping_sub(t.i32(off::TRANSACTIONS + 0x0c))
            .wrapping_add(t.i32(off::TRANSACTIONS + 0x24));
        if amount > 0 {
            reputation(mayor, t.index(), (amount as u32) >> 1);
        }
    }
    for index in 0..11 {
        let at = off::TRANSACTIONS + index * 4;
        let value = t.i32(at);
        t.set_i32(at + 0x2c, value);
        t.set_i32(at, 0);
    }
}

/// `office_charge_land_tax` (0x00500820): acumula gastos; no retira oro directamente.
pub fn charge_land_tax(world: &EconomyWorld, accounts: &mut Accounts, office: usize) {
    let o = &mut accounts.offices[office];
    let tax = world.towns[o.town as usize].u16(0x2c6) as i32;
    let total = (o.business_buildings as i32 + o.houses as i32 + 1).wrapping_mul(tax);
    let merchant = &mut accounts.merchants[o.owner as usize];
    merchant.expenses_4a4 = merchant.expenses_4a4.wrapping_add(total);
    merchant.expenses_4a8 = merchant.expenses_4a8.wrapping_add(total);
    merchant.houses_4d4 = merchant.houses_4d4.wrapping_add((o.houses as i32).wrapping_mul(tax));
    o.current[4] = o.current[4].wrapping_add(total);
}

/// `office_close_weekly_period` (0x005006F0): copia contadores y transfiere la mitad del suelo si es positiva.
pub fn close_office_week(world: &mut EconomyWorld, accounts: &mut Accounts, office: usize) {
    let o = &mut accounts.offices[office];
    o.previous = o.current;
    o.previous_storage = o.storage;
    let paid = o.current[4] / 2;
    if paid > 0 {
        book_transaction(&mut world.towns[o.town as usize], world.tick, paid, 0);
    }
    o.current = [0; 6];
    o.storage = 0;
}

/// `charge_daily_taxes_and_close_week` (0x005315C0), llamado después de actualizar el calendario.
/// `day_of_month` es el byte de `GameWorld+0`; `week` procede de `calendar::is_week_start`.
pub fn charge_daily<F: FnMut(u16, u8, u32)>(
    world: &mut EconomyWorld,
    accounts: &mut Accounts,
    day_of_month: u8,
    week: bool,
    mut reputation: F,
) {
    if day_of_month == 1 {
        for town in 0..world.towns.len() {
            close_town_month(world, town, &mut reputation);
        }
    }
    for town in 0..world.towns.len() {
        let mut office = world.towns[town].u16(off::FIRST_OFFICE);
        while office < world.office_count {
            charge_land_tax(world, accounts, office as usize);
            if week {
                close_office_week(world, accounts, office as usize);
            }
            office = world.offices[office as usize].next;
        }
        if week {
            let t = &mut world.towns[town];
            let weighted = (t.pop(0) as u32)
                .wrapping_mul(3)
                .wrapping_add((t.pop(1) as u32).wrapping_mul(2))
                .wrapping_add(t.pop(2) as u32);
            let amount = weighted
                .wrapping_mul(t.u8(off::TAX_RATE_AND_STAMP) as u32)
                .wrapping_mul(10)
                / 100;
            book_transaction(t, world.tick, amount as i32, 1);
        }
    }
}

/// Conecta la contabilidad fiscal con el comerciante de C06, incluida su reputación mensual.
/// Los registros de `merchants` y `accounts` siguen el mismo orden que `world.merchants`.
pub fn charge_daily_with_reputation(
    world: &mut EconomyWorld,
    accounts: &mut Accounts,
    merchants: &mut [crate::ai::reputation::Merchant],
    day_of_month: u8,
    week: bool,
) {
    assert_eq!(accounts.offices.len(), world.offices.len());
    assert_eq!(accounts.merchants.len(), merchants.len());
    for (account, merchant) in accounts.merchants.iter_mut().zip(merchants.iter()) {
        account.expenses_4a4 = merchant.i32(0x4a4);
        account.expenses_4a8 = merchant.i32(0x4a8);
        account.houses_4d4 = merchant.i32(0x4d4);
    }
    charge_daily(world, accounts, day_of_month, week, |owner, town, amount| {
        merchants[owner as usize].subtract_social_reputation_town_coffers(town as u32, amount);
    });
    for (account, merchant) in accounts.merchants.iter().zip(merchants.iter_mut()) {
        merchant.set_i32(0x4a4, account.expenses_4a4);
        merchant.set_i32(0x4a8, account.expenses_4a8);
        merchant.set_i32(0x4d4, account.houses_4d4);
    }
}
