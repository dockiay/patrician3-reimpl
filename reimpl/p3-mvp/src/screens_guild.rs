//! Gremio de la ciudad (botón de ingreso de `hanse_content_window_update`, `0x005BFEF8`/`0x005C1243`).
//!
//! La cuota es `town_get_guild_entry_fee` (`0x0051B7E0`, `GuildMerchant::guild_entry_fee`, con oráculo) sobre la ciudad elegida; el botón se
//! habilita si el oro del jugador la alcanza y envía la orden `0x37` `[comerciante, ciudad, cuota]`, que el despachador resuelve con
//! `Merchant::buy_town_map_knowledge` (`0x004F8560`): cobra la cuota, marca el bit de la ciudad en `Merchant+0x468`, publica la carta global
//! `0x0C` y funde el mapa del comerciante en el global. La orden no hace nada si la ciudad ya está comprada o el byte `0x006DE52A` no es cero.
use crate::Session;
use p3_core::ai::guild::GuildMerchant;
use p3_core::pump::Operation;
use serde_json::{json, Value};

impl Session {
    fn guild_fee(&self, city: usize) -> Result<i32, String> {
        let me = self.sim.rep_merchants.get(self.owner).ok_or("comerciante ausente")?;
        let towns = &self.sim.economy.world.towns;
        let town = towns.get(city).ok_or("ciudad ausente")?;
        let person = GuildMerchant {
            gold: me.i32(0),
            flags: me.bytes[8],
            office_head: me.u16(0xc),
            home: me.bytes[0x19],
            guild_mask: u32::from_le_bytes(me.bytes[0x468..0x46c].try_into().unwrap()),
            income: me.i32(0x46c),
            reputation: (0..40).map(|i| f32::from_bits(u32::from_le_bytes(me.bytes[0x2fc + 4 * i..0x300 + 4 * i].try_into().unwrap()))).collect(),
        };
        Ok(person.guild_entry_fee(towns.len() as u32, town.u8(0x2c0)))
    }

    pub fn guild_screen(&self) -> Result<Value, String> {
        let me = self.sim.rep_merchants.get(self.owner).ok_or("comerciante ausente")?;
        let fee = self.guild_fee(self.city)?;
        let gold = me.i32(0);
        let member = u32::from_le_bytes(me.bytes[0x468..0x46c].try_into().unwrap()) & (1 << (self.city & 31)) != 0;
        let locked = self.sim.economy.world.world_8a != 0;
        Ok(json!({"city": self.city, "town_name": self.town_name(self.city), "gold": gold, "fee": fee, "member": member, "locked": locked,
                  "can_join": !member && !locked && gold >= fee}))
    }

    pub fn guild_join(&mut self) -> Result<Value, String> {
        let screen = self.guild_screen()?;
        if screen["member"] == true {
            return Err("ya perteneces al gremio de esta ciudad".into());
        }
        if screen["locked"] == true {
            return Err("el ingreso en gremios no está permitido en esta partida".into());
        }
        if screen["can_join"] != true {
            return Err("no tienes oro suficiente para la cuota".into());
        }
        let fee = screen["fee"].as_i64().unwrap_or(0) as u32;
        self.run_operation(Operation([0x37, self.owner as u32, self.city as u32, fee, 0]))?;
        Ok(json!({"screen": self.guild_screen()?, "state": self.state()}))
    }

    pub(crate) fn screen_guild_action(&mut self, name: &str, action: &str) -> Option<Result<Value, String>> {
        if name != "guild" {
            return None;
        }
        Some(match action {
            "join" => self.guild_join(),
            _ => Err(format!("acción desconocida: {action}")),
        })
    }
}
