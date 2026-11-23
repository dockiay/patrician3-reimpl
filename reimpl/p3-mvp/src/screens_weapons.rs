//! Armería (`WeaponsSmithDialog`, ventana de contenido `0x005ED5BD`/`0x005ED623`): comprar o vender a la ciudad lotes de armamento de barco.
//!
//! Siete filas: los tipos 0 a 5 son lotes del almacén de la ciudad (`Town+0x124`, 1000 o 2000 unidades por lote según la tabla
//! `0x00672CB4`) y el tipo 6 son los alfanjes sueltos (`Town+0x2BC`). El precio de la fila es `town_get_cycle_bonus` (`0x005285B0`)
//! al comprar y su mitad (`>> 1`) al vender; la venta lleva `0x100` en el campo de ciudad. El original solo permite la ventana con oficina
//! propia en la ciudad y envía la orden `0x56` sin mirar existencias ni dinero; esta pantalla lo comprueba antes para dar el motivo.
//! La orden `0x5C` (armamento entre la oficina y la ciudad) está portada y conectada al despachador, pero no se ha localizado en el EXE
//! el botón que la envía, así que no tiene acción aquí.
use crate::Session;
use p3_core::flatmem::SparseMem;
use p3_core::pump::Operation;
use serde_json::{json, Value};

const TOWN: u32 = 0x1000_0000;
const WEIGHT: [i32; 6] = [1000, 1000, 2000, 2000, 2000, 1000];

impl Session {
    /// Precio de la fila `kind` (`0x005285B0`) con el estado de la ciudad actual.
    fn weapon_lot_price(&self, kind: u32) -> u32 {
        let town = &self.sim.economy.world.towns[self.city];
        let mut m = SparseMem::default();
        m.seed(TOWN, &town.0[..]);
        m.seed(0x6de4a4, &self.sim.idle.clock.calendar.day_of_year.to_le_bytes());
        m.seed(0x673a78, &[100]);
        p3_core::economy::small_fns::town_get_cycle_bonus(&mut m, TOWN, &[kind])
    }

    /// Piezas de la fila `kind` en el almacén (`bytes` de ciudad u oficina): `existencias / peso` o, en el tipo 6, el contador suelto.
    fn weapon_pieces(bytes: &[u8], kind: usize) -> i32 {
        let word = |o: usize| i32::from_le_bytes(bytes[o..o + 4].try_into().unwrap());
        if kind >= 6 {
            word(0x2bc)
        } else {
            word(0x124 + 4 * kind) / WEIGHT[kind]
        }
    }

    fn weapon_office(&self) -> Result<&Vec<u8>, String> {
        self.sim.host.scheduled_office_images.as_ref().and_then(|v| v.get(self.office)).ok_or_else(|| "no tienes oficina en esta ciudad".to_string())
    }

    pub fn weapons_screen(&self) -> Result<Value, String> {
        let town = &self.sim.economy.world.towns[self.city];
        let office = self.weapon_office()?;
        let rows: Vec<Value> = (0..7)
            .map(|k| {
                let price = self.weapon_lot_price(k as u32);
                json!({"kind": k, "lot": WEIGHT.get(k).copied().unwrap_or(1), "town": Self::weapon_pieces(&town.0[..], k), "office": Self::weapon_pieces(office, k),
                       "buy_price": price, "sell_price": price >> 1})
            })
            .collect();
        Ok(json!({"city": self.city, "town_name": self.town_name(self.city), "gold": self.sim.economy.merchants[self.owner].i32(0), "rows": rows}))
    }

    /// Compra (`buy`) o vende un lote del tipo `kind` a la ciudad (orden `0x56`).
    pub fn weapons_deal(&mut self, kind: u32, buy: bool) -> Result<Value, String> {
        if kind > 6 {
            return Err("tipo de armamento fuera de 0..=6".into());
        }
        let price = self.weapon_lot_price(kind);
        let gold = self.sim.economy.merchants[self.owner].i32(0);
        if buy {
            if Self::weapon_pieces(&self.sim.economy.world.towns[self.city].0[..], kind as usize) < 1 {
                return Err("la ciudad no tiene existencias de ese tipo".into());
            }
            if i64::from(gold) < i64::from(price) {
                return Err("no tienes oro suficiente".into());
            }
        } else if Self::weapon_pieces(self.weapon_office()?, kind as usize) < 1 {
            return Err("tu oficina no tiene ese tipo de armamento".into());
        }
        let (op_price, town_field) = if buy { (price, self.city as u32) } else { (price >> 1, self.city as u32 | 0x100) };
        self.run_operation(Operation([0x56, self.owner as u32, kind, op_price, town_field]))?;
        Ok(json!({"screen": self.weapons_screen()?, "state": self.state()}))
    }

    pub(crate) fn screen_weapons_action(&mut self, name: &str, action: &str, args: &Value) -> Option<Result<Value, String>> {
        if name != "weapons" {
            return None;
        }
        let num = |k: &str| args[k].as_u64().ok_or_else(|| format!("falta {k}"));
        Some(match action {
            "buy" => (|| self.weapons_deal(num("kind")? as u32, true))(),
            "sell" => (|| self.weapons_deal(num("kind")? as u32, false))(),
            _ => Err(format!("acción desconocida: {action}")),
        })
    }
}
