//! Fundar una oficina comercial (`0x5D`, `Town::op_found_trading_office`, `0x0051B890`): ciudades donde el jugador aún no tiene oficina, con el
//! coste que calcula la propia función (obra de tipo 0: materiales de la ciudad, de sus barcos atracados y dinero). No se ha localizado en
//! el EXE el botón que envía la orden, así que la ventana es la lista de ciudades con su presupuesto.
use crate::Session;
use p3_core::pump::Operation;
use serde_json::{json, Value};

impl Session {
    fn has_office_in(&self, city: usize) -> bool {
        self.sim.persisted.iter().any(|o| usize::from(o.stock.owner) == self.owner && usize::from(o.stock.town) == city)
    }

    pub fn found_screen(&self) -> Result<Value, String> {
        let cities: Vec<Value> = (0..self.sim.economy.world.towns.len())
            .map(|c| {
                let has = self.has_office_in(c);
                let cost = if has { None } else { self.sim.found_office_quote(self.owner as u32, c).ok().flatten() };
                json!({"city": c, "name": self.town_name(c), "has_office": has, "cost": cost, "can_found": cost.is_some()})
            })
            .collect();
        Ok(json!({"gold": self.sim.economy.merchants[self.owner].i32(0), "cities": cities}))
    }

    pub fn found_office(&mut self, city: usize) -> Result<Value, String> {
        if city >= self.sim.economy.world.towns.len() {
            return Err("ciudad inexistente".into());
        }
        if self.has_office_in(city) {
            return Err("ya tienes oficina en esa ciudad".into());
        }
        if self.sim.found_office_quote(self.owner as u32, city)?.is_none() {
            return Err("no se puede fundar ahora: faltan materiales o dinero".into());
        }
        self.run_operation(Operation([0x5d, self.owner as u32, city as u32, 0, 0]))?;
        let index = self.sim.persisted.iter().position(|o| usize::from(o.stock.owner) == self.owner && usize::from(o.stock.town) == city).ok_or("la oficina no se ha creado")?;
        Ok(json!({"office": index, "screen": self.found_screen()?, "state": self.state()}))
    }

    pub(crate) fn screen_found_action(&mut self, name: &str, action: &str, args: &Value) -> Option<Result<Value, String>> {
        if name != "found" {
            return None;
        }
        Some(match action {
            "found" => args["city"].as_u64().ok_or_else(|| "falta city".to_string()).and_then(|c| self.found_office(c as usize)),
            _ => Err(format!("acción desconocida: {action}")),
        })
    }
}
