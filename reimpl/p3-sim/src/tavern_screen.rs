//! Taberna (`TavernSideMenu::refresh_available_pages`, `0x005A6E60`): qué páginas de conversación están disponibles para el jugador local.
//! Se evalúan con las funciones portadas sobre la imagen plana del puente C08 más la tabla de capitanes viva.
use crate::c08_flat::{MERCHANTS, MERCHANT_SIZE, TOWNS, TOWN_SIZE};
use crate::Sim;
use p3_core::economy::{council_small as cs, letters, tavern_interaction::find_letter};
use p3_core::flatmem::Mem;

const CAPTAINS: u32 = 0x6000_0000;

/// Nombres de las diez páginas, en el orden de las banderas `+0xB0C..+0xB15` del menú.
pub const PAGES: [&str; 10] =
    ["supply_contract", "trader", "sailors", "captain", "news", "traveller", "weapons_dealer", "thief", "pirate_captain", "letters"];

impl Sim {
    /// Precios del armero de `city` (`0x00526470`): por clase de arma (0..4), decenas en existencia, precio de venta y de compra por arma para la
    /// cantidad que elige `kind` (0: 1, 1: 5, 2: 50, 3: todo). `held` son las decenas que el jugador puede vender (`+0x1C00` de la ventana).
    pub fn weapon_quote(&self, city: usize, kind: u32, held: [i32; 4]) -> Option<([i32; 4], [i32; 4], [i32; 4])> {
        let town = self.economy.world.towns.get(city)?;
        let mut m = p3_core::flatmem::SparseMem::default();
        m.seed(0x1000_0000, &town.0[..]);
        m.seed(0x1100_0000, &[0u8; 0x80]);
        for (k, h) in held.iter().enumerate() {
            m.w32(0x1100_0060 + 4 * k as u32, *h as u32);
        }
        p3_core::economy::ops_misc::weapon_prices(&mut m, 0x1000_0000, kind, 0x1100_0000, 0x1100_0010, 0x1100_0020, 0x1100_0060);
        let mut read = |at: u32| -> [i32; 4] { std::array::from_fn(|i| m.r32(at + 4 * i as u32) as i32) };
        Some((read(0x1100_0000), read(0x1100_0010), read(0x1100_0020)))
    }

    /// Rivales a los que se puede encargar un robo en `city` (`0x00531470`): propietarios de las oficinas de la ciudad que no están en quiebra
    /// (`Merchant+8` bit 2) ni son el jugador local, con el precio de la página del ladrón (`0x005CCC6B..`: `500 * (+0x46C / 10000 + 1)`, tope 0xFDE8, 500 si negativo).
    pub fn tavern_rivals(&self, city: usize) -> Vec<(u16, u16)> {
        let (Some(offices), Some(town)) = (self.host.scheduled_office_images.as_ref(), self.economy.world.towns.get(city)) else { return Vec::new() };
        let mut at = town.u16(0x784);
        let mut out = Vec::new();
        let local = self.economy.world.local_merchant;
        while usize::from(at) < offices.len() && out.len() <= offices.len() {
            let o = &offices[usize::from(at)];
            let who = u16::from_le_bytes([o[0x2c4], o[0x2c5]]);
            if let Some(m) = self.rep_merchants.get(usize::from(who)) {
                if usize::from(who) < usize::from(self.economy.world.merchant_count) && m.bytes[8] & 4 == 0 && u32::from(who) != local {
                    let worth = m.i32(0x46c);
                    let price = if worth < 0 { 500 } else { ((worth / 10000 + 1) * 500).min(0xfde8) as u16 };
                    out.push((who, price));
                }
            }
            at = u16::from_le_bytes([o[0x2ca], o[0x2cb]]);
        }
        out
    }

    /// Capitanes de la lista de la taberna de `city` que `owner` puede contratar (no piratas; libres o ya suyos): `(índice, tipo, propietario)`.
    pub fn tavern_captains(&self, city: usize, owner: usize) -> Vec<(u16, u8, u8)> {
        let table = match &self.host.expedition_factory {
            Some(f) => f.captains.concat(),
            None => return Vec::new(),
        };
        let count = table.len() / 16;
        let mut at = self.economy.world.towns.get(city).map_or(0xffff, |t| t.u16(0x82e));
        let mut out = Vec::new();
        while usize::from(at) < count && out.len() <= count {
            let e = usize::from(at) * 16;
            let (kind, who) = (table[e + 8], table[e + 0xf]);
            if kind <= 0x20 && (usize::from(who) == owner || who == 0xff) {
                out.push((at, kind, who));
            }
            at = u16::from_le_bytes([table[e], table[e + 1]]);
        }
        out
    }

    /// Barcos del comerciante `owner` atracados en `city` con su tripulación y los marineros que aún caben (`0x005184F0`).
    pub fn tavern_ships(&self, city: usize, owner: usize) -> Vec<(usize, u16, i32, u16)> {
        let count = usize::from(self.economy.world.ship_count);
        (0..count)
            .filter_map(|i| {
                let image = match &self.host.navigation {
                    Some(nav) => nav.snapshot.ships.get(i * 0x180..(i + 1) * 0x180)?.to_vec(),
                    None => self.host.expedition_ship_images.get(i)?.clone()?,
                };
                if image.len() < 0x180 || usize::from(image[0]) != owner || usize::from(image[0x39]) != city {
                    return None;
                }
                let crew = u16::from_le_bytes([image[0x40], image[0x41]]);
                let captain = u16::from_le_bytes([image[0x42], image[0x43]]);
                let mut copy = image;
                Some((i, crew, p3_core::ships::ai_dock::needed_crew_image(&mut copy), captain))
            })
            .collect()
    }


    /// Capitanes de la tabla viva (`0x006DD892`).
    pub fn captain_count(&self) -> Option<usize> {
        match &self.host.navigation {
            Some(nav) => Some(nav.snapshot.captains.len() / 16),
            None => self.host.expedition_factory.as_ref().map(|f| f.captains.len()),
        }
    }

    /// Banderas de página de la taberna de `city` para el comerciante local, según `0x005A6E60`.
    pub fn tavern_pages(&self, city: usize) -> Result<[bool; 10], String> {
        if city >= self.economy.world.towns.len() {
            return Err("ciudad inexistente".into());
        }
        let mut flat = self.flat_seed([0; 5])?;
        let captains = match &self.host.navigation {
            Some(nav) => nav.snapshot.captains.clone(),
            None => self.host.expedition_factory.as_ref().ok_or("pool de capitanes ausente")?.captains.concat(),
        };
        let count = (captains.len() / 16) as u16;
        let m = &mut flat.mem;
        m.seed(CAPTAINS, &captains);
        m.seed(0x6dd7a0, &CAPTAINS.to_le_bytes());
        let town = TOWNS + TOWN_SIZE * city as u32;
        let player = self.economy.world.local_merchant;
        let mer = MERCHANTS + MERCHANT_SIZE * player;
        let mut pages = [false; 10];
        if cs::tavern_contract_available(m, town) {
            pages[0] = true;
        } else {
            pages[1] = cs::tavern_ware_offer_available(m, town);
        }
        let sailors = self.rep_merchants.get(player as usize).ok_or("comerciante local ausente")?.get_hireable_sailors(city as u32, &self.economy.world.towns);
        pages[2] = sailors & 0xff != 0;
        pages[3] = cs::tavern_captain(m, town, player as u16) < u32::from(count);
        pages[4] = m.r8(town + 0x82c) < 0x14;
        pages[5] = cs::tavern_ship_task_available(m, town);
        pages[6] = cs::tavern_weapons_dealer(m, town);
        pages[7] = cs::tavern_thief_available(m, town);
        pages[8] = cs::tavern_pirate_captain(m, town) < u32::from(count);
        // Cartas de oferta: la primera de la lista del jugador (`+0xA`) con clave de ciudad que siga disponible.
        let queue = 0x6dd730;
        let total = u32::from(m.r16(queue + 6));
        let start = u32::from(m.r16(mer + 0xa));
        let mut at = find_letter(m, start, city as u32);
        let mut guard = 0;
        while at < total && guard <= total {
            if letters::tavern_offer_letter_is_available(m, at) {
                pages[9] = true;
                break;
            }
            let e = m.r32(queue).wrapping_add(at << 4);
            let next = u32::from(m.r16(e + 6));
            at = find_letter(m, next, city as u32);
            guard += 1;
        }
        Ok(pages)
    }

    /// Capitán pirata que ofrece la taberna de `city` al jugador (`0x005261D0`), si lo hay.
    pub fn tavern_pirate_captain_id(&self, city: usize) -> Result<Option<u16>, String> {
        if city >= self.economy.world.towns.len() {
            return Err("ciudad inexistente".into());
        }
        let mut flat = self.flat_seed([0; 5])?;
        let captains = match &self.host.navigation {
            Some(nav) => nav.snapshot.captains.clone(),
            None => self.host.expedition_factory.as_ref().ok_or("pool de capitanes ausente")?.captains.concat(),
        };
        let count = (captains.len() / 16) as u32;
        let m = &mut flat.mem;
        m.seed(CAPTAINS, &captains);
        m.seed(0x6dd7a0, &CAPTAINS.to_le_bytes());
        let id = cs::tavern_pirate_captain(m, TOWNS + TOWN_SIZE * city as u32);
        Ok((id < count).then_some(id as u16))
    }
}
