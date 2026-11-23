//! Segunda tanda de pantallas del cliente: cartas (buzón), mercado (compra y venta en la plaza) y flota (barcos atracados y convoyes).
//!
//! Igual que `screens.rs`: aquí no se dibuja; cada pantalla devuelve el JSON que la ventana original calcula y cada botón encola la
//! operación que arma su controlador (`enqueue_operation`, `0x0054AA70`) o llama al camino de comercio ya contrastado con el x86
//! (`office_trade_with_reputation`, `trade_ship`).
//!
//! - Cartas (`0x6E` leída, `0x6F` borrar, `0x70` preferencias, `0x55` contestar con pago): `p3_state::letter_ops`, `c08_letter_operation`.
//! - Mercado: precios de compra/venta de `p3_core::economy::prices` (paridad x86 en `x86_c04_*`); compra y venta de la oficina y de un
//!   barco atracado.
//! - Flota: marineros (`0x05` despedir, `0x10` pasar de un barco a otro; `ops_ship`, paridad en `x86_ops_ship`).
use crate::Session;
use p3_core::economy::{
    prices::{buy_price, sell_price, Market, SMALL_LOT_WARES},
    trade::TradeSide,
};
use p3_core::pump::Operation;
use p3_save::tail::{LetterBody, MessagePool, LETTER_HEAD};
use serde_json::{json, Value};

const LETTER_TYPES: &str = include_str!("../../../research/systems/cartas/letter_types.tsv");

/// Nombre de máquina del tipo de carta (`letter_types.tsv`), p. ej. `letter_shipyard_ship_completed`.
fn letter_type_name(kind: u8) -> Option<&'static str> {
    let key = format!("{kind:#04x}");
    LETTER_TYPES.lines().skip(1).find_map(|l| {
        let mut f = l.split('\t');
        (f.next()?.to_ascii_lowercase() == key).then(|| f.next()).flatten()
    })
}

/// Descripción de investigación del tipo (columnas `meaning`, `payload_a`, `payload_b`): no es el texto del juego.
fn letter_type_notes(kind: u8) -> Option<[&'static str; 3]> {
    let key = format!("{kind:#04x}");
    LETTER_TYPES.lines().skip(1).find_map(|l| {
        let f: Vec<&str> = l.split('\t').collect();
        (f.len() > 5 && f[0].to_ascii_lowercase() == key).then(|| [f[3], f[4], f[5]])
    })
}

/// Entorno de nombres vivos para el compositor de texto de cartas.
struct LetterEnv<'a> {
    session: &'a Session,
    row: Vec<u8>,
}
impl p3_sim::letter_text::LetterContext for LetterEnv<'_> {
    fn town_name(&self, id: usize) -> Option<Vec<u8>> {
        self.session.sim.town_names(id).map(|(name, _)| name.to_vec())
    }
    fn ship_name(&self, id: usize) -> Option<Vec<u8>> {
        let nav = self.session.sim.host.navigation.as_ref()?;
        let record = nav.snapshot.ships.get(id * 0x180 + 0x160..id * 0x180 + 0x180)?;
        Some(record.iter().copied().take_while(|&c| c != 0).collect())
    }
    fn first_name(&self, id: u32) -> Option<Vec<u8>> {
        self.session.sim.letter_first_name(id)
    }
    fn family_name(&self, id: u32) -> Option<Vec<u8>> {
        self.session.sim.letter_family_name(id)
    }
    /// `0x512B20`: banco base de nombres de barco; fuera de él, tabla fija del EXE (el rango de nombres propios no está modelado).
    fn bank_c_name(&self, id: u32) -> Option<Vec<u8>> {
        self.session.sim.letter_ship_bank_name(id).or_else(|| self.session.letter_texts.as_ref()?.bank_c_fallback(id))
    }
    fn town_text3(&self, id: usize) -> Option<Vec<u8>> {
        self.session.sim.town_text3(id)
    }
    /// El objeto de imagen de mapa de la ventana de cartas no existe en el cliente: 0x46/0x6E conservan la descripción de investigación.
    fn map_image_handle(&self) -> Option<u32> {
        None
    }
    /// `0x636620` (texto de mercancías) trabaja sobre memoria del x86 y no está accesible desde el cliente: 0x65 conserva la descripción de investigación.
    fn goods_text(&self, _args: [u8; 5]) -> Option<Vec<u8>> {
        None
    }
    fn merchant(&self, idx: u32) -> Option<p3_sim::letter_text::PlayerInfo> {
        let b = &self.session.sim.rep_merchants.get(idx as usize)?.bytes;
        Some(p3_sim::letter_text::PlayerInfo { index: idx, flags: *b.get(0x1a)?, first: *b.get(0x1c)?, family: *b.get(0x1d)? })
    }
    fn player(&self) -> Option<p3_sim::letter_text::PlayerInfo> {
        let index = self.session.sim.economy.world.local_merchant as u32;
        let b = &self.session.sim.rep_merchants.get(index as usize)?.bytes;
        Some(p3_sim::letter_text::PlayerInfo { index, flags: *b.get(0x1a)?, first: *b.get(0x1c)?, family: *b.get(0x1d)? })
    }
    fn captain_name_ids(&self, idx: usize) -> Option<(u8, u8)> {
        self.session.sim.captain_name_ids(idx)
    }
    fn town_count(&self) -> usize {
        self.session.sim.economy.world.towns.len()
    }
    fn ship_count(&self) -> usize {
        self.session.sim.host.navigation.as_ref().map_or(0, |n| n.snapshot.ships.len() / 0x180)
    }
    /// El cuerpo libre apunta a memoria del x86 (no existe en el cliente): estas cartas conservan la descripción de investigación.
    fn memory(&self, _addr: u32, _len: u32) -> Option<Vec<u8>> {
        None
    }
    /// Globales del x86 no disponibles en el cliente: se leen como cero (las ramas que dependen de ellas no añaden efectos).
    fn global(&self, _addr: u32) -> u32 {
        0
    }
    /// Includes the built-in epithet tables populated by CRT initializer 0x46BD80.
    fn runtime_text(&self, slot: u32) -> Option<Vec<u8>> {
        self.session.letter_texts.as_ref()?.initialized_text(slot)
    }
    /// La lista de deudas del x86 no existe en el cliente.
    fn debt_record(&self, _idx: u32) -> Option<[u8; 24]> {
        None
    }
    fn selected_row_word(&self) -> u16 {
        0
    }
    fn offer_text(&self, _key: [u8; 8]) -> Option<Vec<u8>> {
        None
    }
    /// Libro mayor del x86 no disponible en el cliente: lista vacía.
    fn ledger_node(&self, _idx: u32) -> Option<[u8; 16]> {
        None
    }
    fn ledger_count(&self) -> u32 {
        0
    }
    fn player_u16(&self, _off: u32) -> u16 {
        0
    }
    fn row_text(&self) -> Vec<u8> {
        self.row.clone()
    }
}

fn arg(args: &Value, key: &str) -> Result<usize, String> {
    args[key].as_u64().map(|n| n as usize).ok_or_else(|| format!("falta {key}"))
}

impl Session {
    // ---------------------------------------------------------------------------------------------------------------- cartas

    /// Buzón del jugador: la cadena de cartas del comerciante (cabeza en `Merchant+0xA`, enlace en `+6` de cada carta). Cada carta lleva
    /// fecha (`+0` día, `+1` mes con el bit 7 de «sin leer», `+2` año), tipo (`+4`), las palabras de datos (`+8`, `+0xA`, `+0xC`) y el
    /// nombre del tipo según `research/systems/cartas/letter_types.tsv`.
    pub fn letters_screen(&self) -> Result<Value, String> {
        let pool = &self.sim.host.message_pool;
        let parsed = MessagePool::parse(pool).map_err(|e| format!("buzón: {e:?}"))?;
        let merchant = self.sim.rep_merchants.get(self.owner).ok_or("comerciante ausente")?;
        let mut at = merchant.u16(0xa);
        let mut letters = Vec::new();
        let mut budget = parsed.slots;
        while at < parsed.slots && budget > 0 {
            budget -= 1;
            let bytes = parsed.letter(usize::from(at));
            let kind = bytes[4];
            let word = |o: usize| u16::from_le_bytes([bytes[o], bytes[o + 1]]);
            // Las cartas con bloque (elecciones 0x47, guionizadas 0x3C..0x40, ofertas 0x71...) llevan el texto redactado tras la cabecera.
            let text = match parsed.letters[usize::from(at)].body {
                LetterBody::Inline => None,
                LetterBody::InlineBlob => bytes.get(LETTER_HEAD + 8..),
                LetterBody::SizedBlob => bytes.get(LETTER_HEAD + 16..),
            }
            .map(|b| b.iter().take_while(|&&c| c != 0).map(|&c| c as char).collect::<String>())
            .filter(|t| !t.is_empty());
            // Datos tipados de los tipos cuyo significado está acreditado contra el emisor y el lector (`letter_types.tsv`, confianza alta):
            // 0x00 astillero (w1 = tipo de casco), 0x01 reparación (w0 = casco o 4 convoy, w1 = % de estado, u32 = factura), 0x02 rechazo
            // (w1 = casco, u32 = precio impagado; 0 si el barco ya está sano). El texto redactado del juego sigue sin portarse.
            let hull = |n: u16| ["snaikka", "crayer", "cog", "hulk", "convoy"].get(usize::from(n)).copied();
            let money = i32::from_le_bytes([bytes[0xc], bytes[0xd], bytes[0xe], bytes[0xf]]);
            let detail = match kind {
                0x00 => Some(json!({"hull": hull(word(0xa))})),
                0x01 => Some(json!({"hull": hull(word(8)), "condition_percent": word(0xa), "bill": money})),
                0x02 => Some(json!({"hull": hull(word(0xa)), "unpaid_price": money, "already_repaired": money <= 0})),
                _ => None,
            };
            let composed = self.letter_texts.as_ref().and_then(|bank| {
                let mut copy: [u8; 16] = bytes[..16].try_into().ok()?;
                let env = LetterEnv { session: self, row: p3_sim::letter_text::row_date_text(&copy).to_vec() };
                bank.compose(&mut copy, &env).ok().flatten()
            });
            // Latín-1 como el juego; los códigos de formato (`0x00462520`) se interpretan en `markup` (WIN-04).
            let layout = composed.as_ref().map(|c| crate::markup::layout(&c.text, 1, 0));
            let composed_text = layout.as_deref().map(crate::markup::plain);
            let layout_json = layout.as_deref().map(crate::markup::to_json);
            let composed_raw: Option<String> = composed.as_ref().map(|c| c.text.iter().map(|&b| char::from(b)).collect());
            // WIN-05: mes con base 0 en el nibble bajo, como la fila original (`row_date`, `0x47C691`) y el cuerpo.
            let (day, month, year) = p3_sim::letter_text::row_date(&bytes);
            letters.push(json!({
                "composed": composed_text,
                "layout": layout_json,
                "composed_raw": composed_raw,
                "city": bytes[5], "city_name": self.town_name(usize::from(bytes[5])), "detail": detail,
                "letter": at, "kind": kind, "name": letter_type_name(kind), "notes": letter_type_notes(kind),
                "day": day, "month": month, "year": year, "unread": bytes[1] & 0x80 != 0,
                "row_date": bytes.get(..16).and_then(|b| b.try_into().ok()).map(|b: [u8; 16]| p3_sim::letter_text::row_date_text(&b).iter().map(|&c| char::from(c)).collect::<String>()),
                "words": [word(8), word(0xa)], "long": u32::from_le_bytes([bytes[0xc], bytes[0xd], bytes[0xe], bytes[0xf]]),
                "answerable": matches!(kind, 0x45 | 0x46), "answered": kind == 0x46, "text": text,
            }));
            at = word(6);
        }
        let b = &merchant.bytes;
        Ok(json!({
            "merchant": self.owner, "gold": merchant.i32(0), "letters": letters, "unread": letters.iter().filter(|l| l["unread"] == true).count(),
            "preferences": b[0x38..0x40].to_vec(), "preference_flag": b[0x26],
        }))
    }

    fn letters_after(&self) -> Result<Value, String> {
        Ok(json!({"screen": self.letters_screen()?, "state": self.state()}))
    }

    /// Marcar una carta como leída (`0x6E`).
    pub fn letter_read(&mut self, letter: u32) -> Result<Value, String> {
        self.run_operation(Operation([0x6e, self.owner as u32, letter, 0, 0]))?;
        self.letters_after()
    }

    /// Borrar una carta (`0x6F`).
    pub fn letter_delete(&mut self, letter: u32) -> Result<Value, String> {
        self.run_operation(Operation([0x6f, self.owner as u32, letter, 0, 0]))?;
        self.letters_after()
    }

    /// Preferencias de avisos (`0x70`): ocho bytes `+0x38..+0x3F` y el byte `+0x26`.
    pub fn letter_settings(&mut self, preferences: [u8; 8], flag: u8) -> Result<Value, String> {
        let mut bytes = [0u8; 16];
        bytes[0] = 0x70;
        bytes[4..6].copy_from_slice(&(self.owner as u16).to_le_bytes());
        bytes[6..14].copy_from_slice(&preferences);
        bytes[14] = flag;
        let w = |i: usize| u32::from_le_bytes(bytes[i * 4..i * 4 + 4].try_into().unwrap());
        self.run_operation(Operation([w(0), w(1), w(2), w(3), 0]))?;
        self.letters_after()
    }

    /// Contestar una carta con un pago (`0x55`): `city` es la taberna que se libera y `bits` los indicadores de respuesta.
    pub fn letter_answer(&mut self, letter: u32, payment: i32, city: u32, bits: u32) -> Result<Value, String> {
        if payment < 0 || self.sim.economy.merchants[self.owner].i32(0) < payment {
            return Err("saldo insuficiente".into());
        }
        self.run_operation(Operation([0x55, self.owner as u32, letter, payment as u32, (bits << 5) | (city & 0x1f)]))?;
        self.letters_after()
    }

    // --------------------------------------------------------------------------------------------------------------- mercado

    fn market_of(&self, ware: usize) -> Market {
        let t = &self.sim.economy.world.towns[self.city];
        Market { trading: t.more_flags() & 0x100 != 0, stock: t.stock(ware), thresholds: std::array::from_fn(|i| t.threshold(ware, i)), marked_ware: t.u8(0x6f0) }
    }

    /// Plaza del mercado: por mercancía, existencias de la ciudad, precio de compra y de venta de un lote, existencias y precio medio de la
    /// oficina y de cada barco atracado propio. Los lotes son 200 (pequeños) o 2000 unidades (`SMALL_LOT_WARES`).
    pub fn market_screen(&self) -> Result<Value, String> {
        let factor = self.sim.commercial_sell_factor;
        let office = &self.sim.persisted.get(self.office).ok_or("oficina ausente")?.stock;
        let ships: Vec<_> = self
            .dockside_ships()
            .into_iter()
            .map(|(id, c)| json!({"ship": id, "capacity": c.capacity, "load": c.cargo_weight, "cargo": c.stock[..20].to_vec()}))
            .collect();
        let wares: Vec<_> = (0..20)
            .map(|w| {
                let m = self.market_of(w);
                let lot = if SMALL_LOT_WARES[w] { 200 } else { 2000 };
                json!({"ware": w, "lot": lot, "stock": m.stock, "buy_price": buy_price(w, &m, lot), "sell_price": sell_price(w, &m, lot, factor), "office": office.stock[w]})
            })
            .collect();
        let convoys: Vec<_> = self
            .dockside_convoys()
            .into_iter()
            .map(|(c, _, chain)| {
                let free: i32 = chain.iter().map(|&i| self.sim.economy.cargo_ships[i].capacity - self.sim.economy.cargo_ships[i].equipment_weight - self.sim.economy.cargo_ships[i].cargo_weight).sum();
                let cargo: Vec<i32> = (0..20).map(|w| chain.iter().map(|&i| self.sim.economy.cargo_ships[i].stock[w]).sum()).collect();
                json!({"convoy": c, "ships": chain, "free": free, "cargo": cargo})
            })
            .collect();
        Ok(json!({"city": self.city, "town_name": self.town_name(self.city), "gold": self.sim.economy.merchants[self.owner].i32(0), "trading": self.market_of(0).trading,
                  "wares": wares, "ships": ships, "convoys": convoys}))
    }

    /// Barcos propios atracados en esta ciudad con su carga.
    fn dockside_ships(&self) -> Vec<(usize, &p3_core::economy::ship_trade::ShipCargo)> {
        self.sim
            .economy
            .cargo_ships
            .iter()
            .enumerate()
            .filter(|(i, c)| {
                usize::from(c.owner) == self.owner
                    && usize::from(c.town) == self.city
                    && self.sim.host.expedition_ship_images.get(*i).and_then(Option::as_ref).is_some_and(|im| im.len() >= 0x136 && u16::from_le_bytes([im[0x134], im[0x135]]) < 2)
            })
            .collect()
    }

    /// Convoyes propios atracados aquí: índice, primer barco, barcos y carga total (suma de sus bodegas).
    fn dockside_convoys(&self) -> Vec<(usize, u16, Vec<usize>)> {
        let images = &self.sim.host.expedition_convoy_images;
        let ships = &self.sim.host.expedition_ship_images;
        let mut out = Vec::new();
        for (c, image) in images.iter().enumerate() {
            if image.len() < 0x1c || usize::from(image[0]) != self.owner || usize::from(u16::from_le_bytes([image[0x1c], image[0x1d]])) != self.city {
                continue;
            }
            let first = u16::from_le_bytes([image[0xa], image[0xb]]);
            let (mut chain, mut at) = (Vec::new(), first);
            while let Some(Some(ship)) = ships.get(usize::from(at)) {
                if chain.len() > ships.len() || ship.len() < 0x136 {
                    chain.clear();
                    break;
                }
                chain.push(usize::from(at));
                at = u16::from_le_bytes([ship[6], ship[7]]);
            }
            if !chain.is_empty() && chain.iter().all(|&i| u16::from_le_bytes([ships[i].as_ref().unwrap()[0x134], ships[i].as_ref().unwrap()[0x135]]) < 2) {
                out.push((c, first, chain));
            }
        }
        out
    }

    /// Compra o venta agregada de un convoy atracado con el mercado (`0x19` vende, `0x1A` compra; `0x00539300`): recorre los barcos del convoy.
    pub fn trade_convoy(&mut self, convoy: usize, side: TradeSide, ware: usize, quantity: i32) -> Result<Value, String> {
        self.ensure_no_battle()?;
        if ware >= 24 || quantity <= 0 || quantity > 1_000_000 {
            return Err("mercancía o cantidad fuera de rango".into());
        }
        let (_, first, chain) = self.dockside_convoys().into_iter().find(|(c, ..)| *c == convoy).ok_or("el convoy debe ser propio y estar atracado en esta ciudad")?;
        let flags = *self.sim.host.debt_convoy_flags.get(convoy).ok_or("convoy sin banderas")?;
        let count = self.sim.economy.cargo_ships.len();
        let next: Vec<u16> = (0..count).map(|i| self.sim.host.expedition_ship_images.get(i).and_then(Option::as_ref).filter(|im| im.len() >= 8).map_or(0xffff, |im| u16::from_le_bytes([im[6], im[7]]))).collect();
        let before = self.sim.economy.cargo_ships.clone();
        let gold = self.sim.economy.merchants[self.owner].i32(0);
        let mut state = p3_core::economy::ship_trade::ConvoyTrade { first_ship: first, town: self.city as u16, dirty: flags };
        self.sim.economy.trade_convoy(&mut state, &next, self.owner, self.city, ware, quantity, side, self.sim.commercial_sell_factor);
        let changed: Vec<usize> = chain.iter().copied().filter(|&i| self.sim.economy.cargo_ships[i] != before[i]).collect();
        for &id in &changed {
            if !self.changed_ships.contains(&id) {
                self.changed_ships.push(id)
            }
            let cargo = self.sim.economy.cargo_ships[id].clone();
            let write = |image: &mut [u8]| {
                image[0x3c..0x3e].copy_from_slice(&cargo.dirty.to_le_bytes());
                for w in 0..24 {
                    image[0x54 + 4 * w..0x58 + 4 * w].copy_from_slice(&cargo.stock[w].to_le_bytes());
                    image[0xb4 + 4 * w..0xb8 + 4 * w].copy_from_slice(&cargo.average_price[w].to_bits().to_le_bytes());
                }
                for (at, value) in [(0x114, cargo.cargo_value), (0x118, cargo.cargo_weight), (0x11c, cargo.equipment_weight), (0x120, cargo.gun_power)] {
                    image[at..at + 4].copy_from_slice(&value.to_le_bytes());
                }
            };
            if let Some(image) = self.sim.host.expedition_ship_images[id].as_mut() {
                write(image);
            }
            if let Some(nav) = self.sim.host.navigation.as_mut() {
                write(&mut nav.snapshot.ships[id * 0x180..(id + 1) * 0x180]);
            }
        }
        if state.dirty != flags {
            self.sim.host.debt_convoy_flags[convoy] = state.dirty;
            if let Some(image) = self.sim.host.expedition_convoy_images.get_mut(convoy) {
                image[0x18..0x1c].copy_from_slice(&state.dirty.to_le_bytes());
            }
            if let Some(nav) = self.sim.host.navigation.as_mut() {
                nav.snapshot.convoys[convoy * 0x3c + 0x18..convoy * 0x3c + 0x1c].copy_from_slice(&state.dirty.to_le_bytes());
            }
            if !self.sim.host.debt_changed_convoys.contains(&convoy) {
                self.sim.host.debt_changed_convoys.push(convoy)
            }
        }
        self.sim.rep_merchants[self.owner] = self.sim.economy.merchants[self.owner].clone();
        let moved: i64 = chain.iter().map(|&i| i64::from(self.sim.economy.cargo_ships[i].stock[ware]) - i64::from(before[i].stock[ware])).sum();
        Ok(json!({"traded": moved.abs(), "gold_change": i64::from(self.sim.economy.merchants[self.owner].i32(0)) - i64::from(gold), "screen": self.market_screen()?, "state": self.state()}))
    }

    /// Comprar o vender en la plaza: con `ship` el barco comercia directamente con la ciudad (`0x01`/`0x02`), sin él la oficina (`0x58`/`0x59`).
    pub fn market_trade(&mut self, buy: bool, ware: usize, quantity: i32, ship: Option<usize>) -> Result<Value, String> {
        let side = if buy { TradeSide::Buy } else { TradeSide::Sell };
        let done = match ship {
            Some(id) => self.trade_ship(id, side, ware, quantity)?,
            None => self.trade(side, ware, quantity)?,
        };
        Ok(json!({"traded": done["quantity"], "gold_change": done["gold_change"], "screen": self.market_screen()?}))
    }

    // ----------------------------------------------------------------------------------------------------------------- flota

    /// Flota del jugador: por barco, clase y nivel, estado (`+0x134`), ciudad y convoy, tripulación, faltante de marineros, capitán, moral, carga
    /// y sables; y los marineros disponibles de la ciudad en el barco atracado.
    pub fn fleet_screen(&self) -> Result<Value, String> {
        let count = usize::from(self.sim.economy.world.ship_count);
        let mut ships = Vec::new();
        for id in 0..count {
            let Some(image) = self.sim.host.expedition_ship_images.get(id).and_then(Option::as_ref) else { continue };
            if image.len() < 0x180 || usize::from(image[0]) != self.owner {
                continue;
            }
            let (word, state) = (|o: usize| u16::from_le_bytes([image[o], image[o + 1]]), u16::from_le_bytes([image[0x134], image[0x135]]));
            let mut copy = image.clone();
            let needed = p3_core::ships::ai_dock::needed_crew_image(&mut copy);
            let cargo = self.sim.economy.cargo_ships.get(id);
            ships.push(json!({
                "ship": id, "kind": image[0xe], "level": image[0xf], "state": state, "town": image[0x39], "docked_here": state < 2 && usize::from(image[0x39]) == self.city,
                "convoy": word(8), "crew": word(0x40), "needed_crew": needed, "captain": word(0x42), "morale": word(0x3e), "cutlasses": word(0x154),
                "capacity": cargo.map(|c| c.capacity), "load": cargo.map(|c| c.cargo_weight), "cargo": cargo.map(|c| c.stock[..20].to_vec()),
            }));
        }
        let convoys: Vec<_> = self
            .sim
            .shipyard_convoys(self.city, self.owner)
            .unwrap_or_default()
            .into_iter()
            .map(|(convoy, state, repair_cost)| json!({"convoy": convoy, "state": state, "repair_cost": repair_cost}))
            .collect();
        let hireable = self.sim.rep_merchants.get(self.owner).map(|m| m.get_hireable_sailors(self.city as u32, &self.sim.economy.world.towns));
        Ok(json!({"city": self.city, "town_name": self.town_name(self.city), "gold": self.sim.economy.merchants[self.owner].i32(0), "ships": ships, "convoys": convoys, "formations": self.fleet_formations(), "hireable_sailors": hireable}))
    }

    /// Convoyes del jugador en la flota viva: líder, estado, ciudad de destino y miembros (cadena `ship+6` desde `convoy+0xA`).
    fn fleet_formations(&self) -> Vec<Value> {
        let Some(nav) = self.sim.host.navigation.as_ref() else { return Vec::new() };
        let snap = &nav.snapshot;
        let (count, ncv) = (snap.ships.len() / 0x180, snap.convoys.len() / 0x3c);
        let word = |b: &[u8], o: usize| u16::from_le_bytes([b[o], b[o + 1]]);
        let mut out = Vec::new();
        let mut cv = word(&snap.manager, 0xec);
        let mut guard = 0;
        while usize::from(cv) < ncv && guard < 0x1000 {
            guard += 1;
            let c = &snap.convoys[usize::from(cv) * 0x3c..(usize::from(cv) + 1) * 0x3c];
            if usize::from(c[0]) == self.owner {
                let mut members = Vec::new();
                let mut ship = word(c, 0xa);
                while usize::from(ship) < count && members.len() < 64 {
                    members.push(ship);
                    ship = word(&snap.ships, usize::from(ship) * 0x180 + 6);
                }
                out.push(json!({"convoy": cv, "leader": word(c, 0x10), "state": word(c, 0x12), "town": word(c, 0x1c), "destination": c[0x1f], "members": members}));
            }
            cv = word(c, 8);
        }
        out
    }

    /// Un barco atracado sale como convoy nuevo (`0x15`, `0x00509B90`). Sin destino (o con el de su ciudad) forma el convoy aquí.
    pub fn fleet_convoy_depart(&mut self, ship: u32, destination: Option<u32>) -> Result<Value, String> {
        self.require_own_docked(ship)?;
        let here = self.city as u32;
        let op = match destination {
            Some(d) if d != here => Operation([0x15, ship, 0, 0, d]),
            _ => Operation([0x15, ship, 1, 0, here]),
        };
        self.run_operation(op)?;
        self.fleet_after()
    }

    /// Unir un barco atracado a un convoy que ya está formado aquí; `member` es cualquier barco del convoy (`0x18`, `0x00509D10`).
    pub fn fleet_convoy_join(&mut self, member: u32, ship: u32) -> Result<Value, String> {
        self.require_own_docked(ship)?;
        self.run_operation(Operation([0x18, member, ship, 0, 0]))?;
        self.fleet_after()
    }

    /// Sacar un barco de su convoy; si es el líder se disuelve el convoy (`0x17`, `0x00539270`).
    pub fn fleet_convoy_remove(&mut self, ship: u32) -> Result<Value, String> {
        self.require_own_docked(ship)?;
        self.run_operation(Operation([0x17, ship, 0, 0, 0]))?;
        self.fleet_after()
    }

    /// Disolver un convoy propio (`0x16`, `0x0050A580`).
    pub fn fleet_convoy_disband(&mut self, convoy: u32) -> Result<Value, String> {
        let mine = self.fleet_formations().iter().any(|f| f["convoy"].as_u64() == Some(u64::from(convoy)));
        if !mine {
            return Err("convoy ajeno o inexistente".into());
        }
        self.run_operation(Operation([0x16, convoy, 0, 0, 0]))?;
        self.fleet_after()
    }

    fn fleet_after(&mut self) -> Result<Value, String> {
        Ok(json!({"screen": self.fleet_screen()?, "state": self.state()}))
    }

    /// Despedir marineros (`0x05`, `0x00537DD0`): vuelven a la reserva de la ciudad y baja la moral.
    pub fn fleet_dismiss_sailors(&mut self, ship: u32, amount: u32) -> Result<Value, String> {
        self.require_own_docked(ship)?;
        self.run_operation(Operation([0x05, ship, amount, 0, 0]))?;
        self.fleet_after()
    }

    /// Pasar marineros de un barco a otro, ambos atracados aquí (`0x10`, `0x00538930`).
    pub fn fleet_transfer_sailors(&mut self, from: u32, to: u32, amount: u32) -> Result<Value, String> {
        if from == to {
            return Err("origen y destino iguales".into());
        }
        self.require_own_docked(from)?;
        self.require_own_docked(to)?;
        self.run_operation(Operation([0x10, from, to, amount, 0]))?;
        self.fleet_after()
    }

    /// Pasar sables de un barco a otro (`0x11`, `0x005389C0`): el destino no acepta más de un décimo de su capacidad de carga.
    pub fn fleet_transfer_cutlasses(&mut self, from: u32, to: u32, amount: u32) -> Result<Value, String> {
        if from == to {
            return Err("origen y destino iguales".into());
        }
        self.require_own_docked(from)?;
        self.require_own_docked(to)?;
        self.run_operation(Operation([0x11, from, to, amount, 0]))?;
        self.fleet_after()
    }

    fn require_own_docked(&self, ship: u32) -> Result<(), String> {
        if self.dockside_ships().iter().any(|(i, _)| *i == ship as usize) {
            Ok(())
        } else {
            Err("el barco debe ser propio y estar atracado en esta ciudad".into())
        }
    }

    // -------------------------------------------------------------------------------------------------------------- almacén

    /// Almacén de la oficina: existencias y precio medio por mercancía (24, con las armas) y, por barco atracado, su carga y hueco libre.
    pub fn warehouse_screen(&self) -> Result<Value, String> {
        let office = &self.sim.persisted.get(self.office).ok_or("oficina ausente")?.stock;
        let wares: Vec<_> = (0..24).map(|w| json!({"ware": w, "amount": office.stock[w], "average_price": office.average_price[w]})).collect();
        let ships: Vec<_> = self
            .dockside_ships()
            .into_iter()
            .map(|(id, c)| json!({"ship": id, "capacity": c.capacity, "load": c.cargo_weight, "cargo": c.stock.to_vec(), "average_price": c.average_price.to_vec()}))
            .collect();
        let convoys: Vec<_> = self
            .dockside_convoys()
            .into_iter()
            .map(|(c, _, chain)| {
                let cargo: Vec<i32> = (0..24).map(|w| chain.iter().map(|&i| self.sim.economy.cargo_ships[i].stock[w]).sum()).collect();
                json!({"convoy": c, "ships": chain, "cargo": cargo})
            })
            .collect();
        Ok(json!({"city": self.city, "town_name": self.town_name(self.city), "gold": self.sim.economy.merchants[self.owner].i32(0), "wares": wares, "ships": ships, "convoys": convoys}))
    }

    /// Mover mercancía entre un convoy atracado y la oficina (`0x1B`, `0x005396B0`): `to_convoy` carga en sus barcos (lotes de 200/2000, hasta el hueco libre).
    pub fn warehouse_convoy_transfer(&mut self, convoy: u32, ware: u32, amount: u32, to_convoy: bool) -> Result<Value, String> {
        if ware >= 24 || amount == 0 || amount > 1_000_000 {
            return Err("mercancía o cantidad fuera de rango".into());
        }
        if !self.dockside_convoys().iter().any(|(c, ..)| *c == convoy as usize) {
            return Err("el convoy debe ser propio y estar atracado en esta ciudad".into());
        }
        self.run_operation(Operation([0x1b, amount, convoy | (ware << 16), self.owner as u32 | (u32::from(to_convoy) << 16), 0]))?;
        Ok(json!({"screen": self.warehouse_screen()?, "state": self.state()}))
    }

    /// Mover mercancía entre un barco atracado y la oficina (`0x08`, `0x00538100`): `to_ship` pasa de la oficina al barco (limitado por el hueco y las existencias).
    pub fn warehouse_transfer(&mut self, ship: u32, ware: u32, amount: u32, to_ship: bool) -> Result<Value, String> {
        if ware >= 24 || amount == 0 || amount > 1_000_000 {
            return Err("mercancía o cantidad fuera de rango".into());
        }
        self.require_own_docked(ship)?;
        self.run_operation(Operation([0x08, amount, ship | (ware << 16), u32::from(to_ship) << 16, 0]))?;
        Ok(json!({"screen": self.warehouse_screen()?, "state": self.state()}))
    }

    /// Acciones de las pantallas de esta tanda; `None` si `name`/`action` no son suyas.
    pub(crate) fn screen_trade_action(&mut self, name: &str, action: &str, args: &Value) -> Option<Result<Value, String>> {
        if let Some(r) = self.screen_statistics_action(name, action, args) {
            return Some(r);
        }
        if let Some(r) = self.screen_found_action(name, action, args) {
            return Some(r);
        }
        if let Some(r) = self.screen_guild_action(name, action) {
            return Some(r);
        }
        if let Some(r) = self.screen_weapons_action(name, action, args) {
            return Some(r);
        }
        if let Some(r) = self.screen_baths_action(name, action, args) {
            return Some(r);
        }
        Some(match (name, action) {
            ("letters", "read") => arg(args, "letter").and_then(|l| self.letter_read(l as u32)),
            ("letters", "delete") => arg(args, "letter").and_then(|l| self.letter_delete(l as u32)),
            ("letters", "answer") => (|| self.letter_answer(arg(args, "letter")? as u32, arg(args, "payment")? as i32, arg(args, "city")? as u32, arg(args, "bits")? as u32))(),
            ("letters", "settings") => (|| {
                let list = args["preferences"].as_array().filter(|a| a.len() == 8).ok_or("faltan ocho preferencias")?;
                let mut p = [0u8; 8];
                for (k, v) in list.iter().enumerate() {
                    p[k] = v.as_u64().and_then(|n| u8::try_from(n).ok()).ok_or("preferencia inválida")?;
                }
                self.letter_settings(p, arg(args, "flag")? as u8)
            })(),
            ("market", "trade") => (|| {
                let quantity = args["quantity"].as_i64().and_then(|n| i32::try_from(n).ok()).ok_or("falta quantity")?;
                if let Some(convoy) = args["convoy"].as_u64() {
                    let side = if arg(args, "buy")? != 0 { TradeSide::Buy } else { TradeSide::Sell };
                    return self.trade_convoy(convoy as usize, side, arg(args, "ware")?, quantity);
                }
                self.market_trade(arg(args, "buy")? != 0, arg(args, "ware")?, quantity, args["ship"].as_u64().map(|n| n as usize))
            })(),
            ("warehouse", "convoy_transfer") => (|| self.warehouse_convoy_transfer(arg(args, "convoy")? as u32, arg(args, "ware")? as u32, arg(args, "amount")? as u32, arg(args, "to_convoy")? != 0))(),
            ("warehouse", "transfer") => (|| self.warehouse_transfer(arg(args, "ship")? as u32, arg(args, "ware")? as u32, arg(args, "amount")? as u32, arg(args, "to_ship")? != 0))(),
            ("fleet", "dismiss_sailors") => (|| self.fleet_dismiss_sailors(arg(args, "ship")? as u32, arg(args, "amount")? as u32))(),
            ("fleet", "transfer_cutlasses") => (|| self.fleet_transfer_cutlasses(arg(args, "from")? as u32, arg(args, "to")? as u32, arg(args, "amount")? as u32))(),
            ("fleet", "convoy_depart") => (|| self.fleet_convoy_depart(arg(args, "ship")? as u32, args.get("destination").and_then(Value::as_u64).map(|d| d as u32)))(),
            ("fleet", "convoy_join") => (|| self.fleet_convoy_join(arg(args, "member")? as u32, arg(args, "ship")? as u32))(),
            ("fleet", "convoy_remove") => (|| self.fleet_convoy_remove(arg(args, "ship")? as u32))(),
            ("fleet", "convoy_disband") => (|| self.fleet_convoy_disband(arg(args, "convoy")? as u32))(),
            ("fleet", "transfer_sailors") => (|| self.fleet_transfer_sailors(arg(args, "from")? as u32, arg(args, "to")? as u32, arg(args, "amount")? as u32))(),
            _ => return None,
        })
    }
}
