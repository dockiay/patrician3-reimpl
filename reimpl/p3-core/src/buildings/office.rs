//! Fundación de una oficina comercial (C05): `op_found_trading_office` (0x0051B890) valora la obra de tipo 0 del
//! comerciante, descuenta de las existencias de la ciudad lo que falta, retira de sus barcos lo que aportan,
//! le carga el coste y crea la oficina.

use super::evaluate::{BuildRequest, EvalEnv, EvalHost, EvalShip};
use super::Site;
use crate::town::Town;

/// Mercancías de la obra (ladrillo, madera, hierro y la del extra) y cuánto vale cada unidad que falta en
/// existencias de la ciudad; el hierro cuenta por 200 y las demás por 2000.
const WARES: [u32; 4] = [19, 11, 12, 17];
const UNIT: [i32; 4] = [2000, 2000, 200, 2000];

/// Lo que pide fuera de la ciudad: la valoración y la retirada de mercancías de los barcos.
pub trait FoundHost: EvalHost {
    /// `merchant_take_wares_from_ships_in_town` (0x004F5450): (mercancía, propietario, cantidad pedida).
    fn take_wares_from_ships(&mut self, ware: u32, owner: u32, quantity: i32);
}

/// Entorno global del que lee la valoración y comerciante al que se carga el coste.
pub struct FoundEnv<'a> {
    /// Número de comerciantes (`0x006DE4AA`).
    pub merchant_count: u16,
    /// Número de barcos (`0x006DD894`).
    pub ship_count: u16,
    /// Primer barco del comerciante (`Merchant+0xE`).
    pub ship_head: u16,
    /// `+0x6F1` (alcalde) de la ciudad de la tabla de ciudades con este mismo índice.
    pub mayor: u8,
    pub ships: &'a [EvalShip],
    /// Dinero (`+0`) e ingresos (`+0x4C0`) del comerciante.
    pub merchant_gold: &'a mut i32,
    pub merchant_income: &'a mut i32,
}

impl Site {
    /// `op_found_trading_office` para el comerciante `merchant` (el argumento sin recortar).
    pub fn found_trading_office(t: &mut Town, merchant: u32, env: &mut FoundEnv, host: &mut dyn FoundHost) {
        if merchant as u16 >= env.merchant_count {
            return;
        }
        host.get_merchant(merchant & 0xFFFF);
        let owner = t.u8(0x2C0) as u32;
        if host.find_office(merchant, owner, 0).is_some() {
            return;
        }
        let mut req = BuildRequest {
            ty: 0,
            merchant: merchant as u16,
            ..Default::default()
        };
        let ok = {
            let mut eval = EvalEnv {
                merchant_count: env.merchant_count,
                ship_count: env.ship_count,
                ship_head: env.ship_head,
                merchant_gold: *env.merchant_gold,
                mayor: env.mayor,
                ships: env.ships,
                host: &mut *host,
            };
            Site::evaluate_build_request(t, &mut req, &mut eval)
        };
        if !ok {
            return;
        }
        for k in 0..4 {
            let slot = 4 + 4 * WARES[k] as usize;
            let missing = req.short[k];
            t.set_u32(slot, t.u32(slot).wrapping_add(missing.wrapping_mul(-UNIT[k]) as u32));
            let quantity = req.need[k].wrapping_sub(missing).wrapping_mul(UNIT[k]);
            if req.have[k] > 0 {
                host.take_wares_from_ships(WARES[k], owner, quantity);
            }
            if t.i32(slot) < 0 {
                t.set_u32(slot, 0);
            }
        }
        *env.merchant_gold = env.merchant_gold.wrapping_sub(req.total);
        *env.merchant_income = env.merchant_income.wrapping_add(req.total);
        host.find_office(merchant, owner, 1);
    }
}
