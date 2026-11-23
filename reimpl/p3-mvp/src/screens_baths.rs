//! Baños (`BathHouseDialog` / `BathHouseBribeDialog`): sobornar a un concejal para ocupar un escaño del concejo.
//!
//! Orden del original (`0x005B1D80`, `0x005B2480`): al entrar en la conversación se envía la operación `0x39` (el comerciante
//! apunta la hora de la visita en `Merchant+4`). Al confirmar, el precio esperado es `town_calculate_expected_bribe` (0x00529E20,
//! depende de la reputación y de un `rand() % 11`): si la oferta lo alcanza se envía `0x42` (se paga y el escaño pasa al
//! comerciante); si no, `0x43` (pierde reputación y el escaño queda en 1 si era suyo o no válido) y ese escaño no admite otro
//! intento hasta pasadas 256 unidades de reloj. La marca de «pagó de más» (oferta >= 1,5 × esperado) la usa la ventana para la respuesta.
use crate::Session;
use p3_core::pump::Operation;
use serde_json::{json, Value};

const SEATS: usize = 0x6dc;

impl Session {
    fn bath_reset_if_stale(&mut self) {
        let tick = self.sim.idle.clock.tick;
        if tick.wrapping_sub(self.bath_visit.0) > 0x100 {
            self.bath_visit.1 = [false; 4];
        }
    }

    pub fn baths_screen(&mut self) -> Result<Value, String> {
        self.bath_reset_if_stale();
        let town = self.sim.economy.world.towns.get(self.city).ok_or("ciudad ausente")?;
        let me = self.owner as u8;
        let seats: Vec<Value> = (0..4)
            .map(|i| {
                let holder = town.u8(SEATS + i);
                json!({"seat": i, "holder": holder, "holder_name": self.office_holder_label(holder), "mine": holder == me, "failed_today": self.bath_visit.1[i], "can_bribe": holder != me && !self.bath_visit.1[i]})
            })
            .collect();
        let gold = self.sim.economy.merchants[self.owner].i32(0);
        Ok(json!({"city": self.city, "town_name": self.town_name(self.city), "gold": gold, "max_offer": gold.clamp(0, 999_999), "seats": seats}))
    }

    /// Entra en los baños: la operación `0x39` fija `Merchant+4` a la hora actual.
    pub fn baths_enter(&mut self) -> Result<Value, String> {
        let tick = self.sim.idle.clock.tick;
        self.run_operation(Operation([0x39, self.owner as u32, tick, 0, 0]))?;
        self.bath_reset_if_stale();
        self.bath_visit.0 = tick;
        Ok(json!({"screen": self.baths_screen()?, "state": self.state()}))
    }

    /// Ofrece `amount` al concejal del escaño `seat`.
    pub fn baths_bribe(&mut self, seat: u32, amount: u32) -> Result<Value, String> {
        self.bath_reset_if_stale();
        if seat >= 4 {
            return Err("escaño inexistente".into());
        }
        let current = self.sim.economy.world.towns.get(self.city).ok_or("ciudad ausente")?.u8(SEATS + seat as usize);
        if current == self.owner as u8 || self.bath_visit.1[seat as usize] {
            return Err("este escaño no admite soborno ahora".into());
        }
        let gold = self.sim.economy.merchants[self.owner].i32(0).clamp(0, 999_999) as u32;
        let offer = amount.min(gold);
        let expected = self.sim.bath_expected_bribe(self.city, self.owner as u32, seat)?;
        let (me, city) = (self.owner as u32, self.city as u32);
        let outcome;
        let mut overpaid = false;
        if expected <= offer {
            self.run_operation(Operation([0x42, seat, me, offer, city]))?;
            overpaid = f64::from(offer) >= (f64::from(expected) * 1.5).trunc();
            outcome = "success";
        } else {
            self.run_operation(Operation([0x43, seat, me, 0, city]))?;
            self.bath_visit.1[seat as usize] = true;
            outcome = "failure";
        }
        Ok(json!({"outcome": outcome, "overpaid": overpaid, "offer": offer, "screen": self.baths_screen()?, "state": self.state()}))
    }

    pub(crate) fn screen_baths_action(&mut self, name: &str, action: &str, args: &Value) -> Option<Result<Value, String>> {
        if name != "baths" {
            return None;
        }
        let num = |k: &str| args[k].as_u64().ok_or_else(|| format!("falta {k}"));
        Some(match action {
            "enter" => self.baths_enter(),
            "bribe" => (|| self.baths_bribe(num("seat")? as u32, num("amount")? as u32))(),
            _ => Err(format!("acción desconocida: {action}")),
        })
    }
}
