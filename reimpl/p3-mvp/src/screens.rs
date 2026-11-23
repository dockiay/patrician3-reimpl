//! Pantallas del juego sobre la sesión: lectura de lo que muestra cada ventana y acciones que encolan la misma operación que el
//! original (`enqueue_operation`, `0x0054AA70`) y la despachan con el despachador de la simulación.
//!
//! Aquí no se dibuja: cada pantalla devuelve un JSON con los datos que calcula el original al abrir y actualizar la ventana
//! (con las funciones ya contrastadas con el x86) y cada acción construye el paquete que arma el controlador del botón.
//! Prestamista: `MoneyLenderDialog` (`0x005B2800`..`0x005B6AA0`).
use crate::Session;
use p3_core::{economy::{loan_applicants, loan_offers, loan_terms}, pump::Operation, rng::CrtRand};
use serde_json::{json, Value};

/// Operaciones que las ventanas encolan y que el despachador de la simulación aplica con paridad (`c08_operations`).
const SCREEN_OPERATIONS: [u32; 42] = [0x5d, 0x5e, 0x37, 0x56, 0x39, 0x42, 0x43, 0x15, 0x16, 0x17, 0x18, 0x1b, 0x11, 0x08, 0x6e, 0x6f, 0x70, 0x55, 0x05, 0x10, 0x4e, 0x2f, 0x1d, 0x4d, 0x50, 0x51, 0x53, 0x0c, 0x04, 0x06, 0x4c, 0x4f, 0x5b, 0x29, 0x2a, 0x31, 0x32, 0x3f, 0x45, 0x47, 0x49, 0x4a];

/// Semanas de un préstamo propio y total a devolver, de la tarea 1 del planificador (`op_grant_loan`, `0x00539DC0`).
fn loan_task(i: usize, r: &p3_core::scheduler::TaskRecord) -> Value {
    json!({"task": i, "lender": r.u8_at(0x10), "borrower": r.u8_at(0x11), "town": r.u8_at(0x14), "repay": r.u32_at(8),
           "rate": r.u8_at(0x12), "weeks": r.u8_at(0x13), "due": r.due})
}

impl Session {
    pub(crate) fn ensure_no_battle(&self) -> Result<(), String> {
        if self.paused_battle.is_some() || self.sim.host.clock_before_battle || self.sim.host.encounter_battle.is_some() || self.sim.host.town_attack_active.is_some() {
            return Err("batalla activa en pausa".into());
        }
        Ok(())
    }

    /// Encola `op` y la despacha; si el despachador la rechaza se retira de la cola (el original la habría descartado igual).
    #[doc(hidden)]
    pub fn run_operation(&mut self, op: Operation) -> Result<(), String> {
        self.ensure_no_battle()?;
        // El despachador manda a `dispatch_offer` cualquier código que no reconoce; solo se admiten las operaciones que las pantallas arman.
        if !SCREEN_OPERATIONS.contains(&op.0[0]) {
            return Err(format!("operación {:#x} sin pantalla conectada", op.0[0]));
        }
        let before = self.sim.host.council_operations.len();
        self.sim.host.council_operations.push(op);
        let errors = self.sim.dispatch_pending_council_operations();
        if !errors.is_empty() {
            self.sim.host.council_operations.truncate(before);
            return Err(errors.join("; "));
        }
        Ok(())
    }

    fn lender_offers(&self) -> (i32, Vec<[u8; 0x10]>) {
        let town = &self.sim.economy.world.towns[self.city];
        let mut recs = [0u8; 0x30];
        let mut changed = false;
        let n = loan_offers::build(town, &self.sim.rep_merchants, self.owner as u8, self.sim.economy.world.towns.len() as u32, &mut recs, &mut changed);
        let list = (0..n.max(0) as usize).map(|i| <[u8; 0x10]>::try_from(&recs[i * 0x10..i * 0x10 + 0x10]).unwrap()).collect();
        (n, list)
    }

    /// Solicitantes de la ciudad (pestaña «conceder préstamo»): los plazos usan el `rand` del anfitrión; aquí se calculan con una copia
    /// para no consumirlo al solo mirar (la acción recalcula con el generador real).
    fn lender_applicants(&self, rng: &mut CrtRand) -> Vec<[u8; 0x10]> {
        let town = &self.sim.economy.world.towns[self.city];
        let mut recs = [0u8; 0x30];
        let (n, _) = loan_applicants::build(town, &mut recs, self.owner as u8, rng);
        (0..n as usize).map(|i| <[u8; 0x10]>::try_from(&recs[i * 0x10..i * 0x10 + 0x10]).unwrap()).collect()
    }

    fn offer_json(rec: &[u8; 0x10]) -> Value {
        json!({"repay": u32::from_le_bytes([rec[0], rec[1], rec[2], rec[3]]), "rate": rec[4], "amount": u16::from_le_bytes([rec[8], rec[9]]),
               "weeks": rec[0xa], "interest": rec[0xf] & 0xf, "slot": rec[0xf] >> 4, "applicant": [rec[5], rec[6], rec[7]]})
    }

    /// Ventana del prestamista: ofertas para tomar un préstamo (página 0), préstamos propios por devolver (1), solicitantes (2) y préstamos concedidos (5).
    /// `status` de las ofertas: `-1` comerciante inhabilitado, `-2` sin capacidad de crédito (menos de 2000) y, con tres préstamos ya, no se ofrece más.
    pub fn lender_screen(&self) -> Result<Value, String> {
        let (status, offers) = self.lender_offers();
        let tasks = &self.sim.host.council_scheduler.records;
        let owner = self.owner as u8;
        let own: Vec<Value> = tasks.iter().enumerate().filter(|(_, r)| r.kind == 1 && r.u8_at(0x11) == owner).map(|(i, r)| loan_task(i, r)).collect();
        let granted: Vec<Value> = tasks.iter().enumerate().filter(|(_, r)| r.kind == 1 && r.u8_at(0x10) == owner).map(|(i, r)| loan_task(i, r)).collect();
        let mut rng = self.sim.host.investigation_rng.unwrap_or(CrtRand { state: self.sim.idle.clock.tick });
        let applicants: Vec<Value> = self.lender_applicants(&mut rng).iter().map(Self::offer_json).collect();
        Ok(json!({"city": self.city, "town_name": self.town_name(self.city), "gold": self.sim.economy.merchants[self.owner].i32(0),
                  "offers_status": status, "offers": offers.iter().map(Self::offer_json).collect::<Vec<_>>(), "can_take": own.len() < 3,
                  "own_loans": own, "granted_loans": granted, "applicants": applicants}))
    }

    /// Botón «tomar el préstamo» de la oferta `index` (`0x005B5A00`): operación `0x29` con el registro de 16 bytes de la oferta.
    pub fn lender_take_loan(&mut self, index: usize) -> Result<Value, String> {
        let owner = self.owner as u8;
        let own = self.sim.host.council_scheduler.records.iter().filter(|r| r.kind == 1 && r.u8_at(0x11) == owner).count();
        if own > 2 {
            return Err("ya tienes tres préstamos".into());
        }
        let (_, offers) = self.lender_offers();
        let rec = offers.get(index).ok_or("oferta inexistente")?;
        let amount = i64::from(u16::from_le_bytes([rec[8], rec[9]]));
        let before = self.sim.economy.merchants[self.owner].i32(0);
        self.run_operation(grant_operation(rec))?;
        Ok(json!({"amount": amount, "gold_before": before, "screen": self.lender_screen()?, "state": self.state()}))
    }

    /// Botón «devolver» del préstamo de la tarea `task` (`0x005B5B90`): operación `0x2A` si el efectivo alcanza (`+8` del registro: total a devolver).
    pub fn lender_repay(&mut self, task: usize) -> Result<Value, String> {
        let r = self.sim.host.council_scheduler.records.get(task).filter(|r| r.kind == 1 && r.u8_at(0x11) == self.owner as u8).ok_or("préstamo inexistente")?;
        let repay = r.u32_at(8);
        if repay > self.sim.economy.merchants[self.owner].i32(0) as u32 {
            return Err("no hay dinero para saldar el préstamo".into());
        }
        self.run_operation(Operation([0x2a, self.owner as u32, task as u32, 0, 0]))?;
        Ok(json!({"repaid": repay, "screen": self.lender_screen()?, "state": self.state()}))
    }

    /// Conceder un préstamo al solicitante `index` con interés `interest` (0..=4; `0x005B6000`): reajusta el nibble bajo de `+0xF`, recalcula el total
    /// (`0x0051D270`) y encola `0x29` como prestamista el propio jugador.
    pub fn lender_grant(&mut self, index: usize, interest: u8) -> Result<Value, String> {
        if interest > 4 {
            return Err("interés fuera de 0..=4".into());
        }
        let mut shared = self.sim.host.investigation_rng.take();
        let local = shared.is_none();
        let mut rng = shared.take().unwrap_or(CrtRand { state: self.sim.idle.clock.tick });
        let mut applicants = self.lender_applicants(&mut rng);
        let result = (|| {
            let rec = applicants.get_mut(index).ok_or("solicitante inexistente")?;
            rec[0xf] = (rec[0xf] & 0xf0) | interest;
            loan_terms::compute(rec, &mut rng);
            let amount = i32::from(u16::from_le_bytes([rec[8], rec[9]]));
            if amount > self.sim.economy.merchants[self.owner].i32(0) {
                return Err("efectivo insuficiente".to_string());
            }
            Ok(*rec)
        })();
        if !local {
            self.sim.host.investigation_rng = Some(rng);
        }
        let rec = result?;
        self.run_operation(grant_operation(&rec))?;
        Ok(json!({"screen": self.lender_screen()?, "state": self.state()}))
    }
}

impl Session {
    /// Ventana de la iglesia (`ChurchContentWindow`): fondo de donaciones y su nivel (`0x004FE360`), caja de obras, aforo y etapa de la
    /// ampliación (`0x004FEA97`). La página «alimentar a los pobres» envía la operación `0x30`, aún sin portar.
    pub fn church_screen(&self) -> Result<Value, String> {
        let town = &self.sim.economy.world.towns[self.city];
        let site = self.sim.host.sites.get(self.city).ok_or("plano de la ciudad ausente")?;
        let setting = self.sim.host.church_cost_factor;
        let gold = self.sim.economy.merchants[self.owner].i32(0);
        Ok(json!({"city": self.city, "town_name": self.town_name(self.city), "gold": gold,
                  "level": town.u8(0x794 + 0xf), "fund": town.i32(0x794), "tier": p3_core::economy::church::donation_tier(town, setting),
                  "collected": town.i32(0x794 + 4), "capacity": town.u32(0x794 + 8),
                  "stage": p3_core::economy::church::extension_stage(town, site, setting),
                  "max_donation": gold.clamp(0, 999_999), "feed_poor_available": true, "feed": self.church_feed_screen()}))
    }

    /// Donar `amount` a la redecoración (operación `0x32`, `0x005CB280`; el importe se limita al oro y a 999 999).
    pub fn church_redecorate(&mut self, amount: i32) -> Result<Value, String> {
        let amount = self.church_amount(amount)?;
        self.run_operation(Operation([0x32, self.owner as u32, self.city as u32, amount as u32, 0]))?;
        Ok(json!({"screen": self.church_screen()?, "state": self.state()}))
    }

    /// Donar `amount` a la ampliación (operación `0x31`, `0x005CB4E0`); solo con etapa menor que 3.
    pub fn church_extend(&mut self, amount: i32) -> Result<Value, String> {
        let amount = self.church_amount(amount)?;
        let stage = self.church_screen()?["stage"].as_i64().unwrap_or(5);
        if stage >= 3 {
            return Err("la iglesia no recibe donaciones de ampliación en esta etapa".into());
        }
        self.run_operation(Operation([0x31, self.owner as u32, self.city as u32, amount as u32, 0]))?;
        Ok(json!({"screen": self.church_screen()?, "state": self.state()}))
    }

    fn church_amount(&self, amount: i32) -> Result<i32, String> {
        let gold = self.sim.economy.merchants[self.owner].i32(0);
        let amount = amount.clamp(0, 999_999).min(gold);
        if amount <= 0 {
            return Err("no hay nada que donar".into());
        }
        Ok(amount)
    }
}

impl Session {
    /// Ayuntamiento (`TownHallWindow`): alcalde, arcas (`Town+0x700`, partidas del mes anterior en `+0x704`), candidatura del jugador
    /// (`Merchant+0x118`) y votación del día (tarea 0x20/0x21, `0x004D9C70`). Donar o tomar de las arcas: operación `0x47` (`0x0053AF30`).
    pub fn town_hall_screen(&self) -> Result<Value, String> {
        let town = &self.sim.economy.world.towns[self.city];
        let me = &self.sim.rep_merchants[self.owner];
        let vote = p3_core::economy::c08_votes::find_todays_vote_task(&self.sim.host.council_scheduler, self.sim.idle.clock.tick)
            .and_then(|i| self.sim.host.council_scheduler.records.get(usize::from(i)).map(|r| json!({"task": i, "kind": r.kind, "town": r.u8_at(0xe)})));
        Ok(json!({"city": self.city, "town_name": self.town_name(self.city), "gold": self.sim.economy.merchants[self.owner].i32(0),
                  "mayor": town.u8(0x6f1), "mayor_name": self.office_holder_label(town.u8(0x6f1)), "is_mayor": usize::from(town.u8(0x6f1)) == self.owner, "town_money": town.i32(0x700), "transactions": (0..8).map(|k| town.i32(0x704 + 4 * k)).collect::<Vec<_>>(),
                  "candidate": me.i32(0x118) != 0, "vote": vote}))
    }

    /// Declarar o retirar la candidatura (operación `0x45`, `0x005E7C80`).
    pub fn town_hall_candidacy(&mut self, candidate: bool) -> Result<Value, String> {
        self.run_operation(Operation([0x45, self.owner as u32, u32::from(candidate), 0, 0]))?;
        Ok(json!({"screen": self.town_hall_screen()?, "state": self.state()}))
    }

    /// Ingresar (`deposit`) o retirar oro de las arcas de la ciudad (operación `0x47`).
    pub fn town_hall_treasury(&mut self, deposit: bool, amount: u32) -> Result<Value, String> {
        if amount == 0 {
            return Err("importe nulo".into());
        }
        // La página de arcas del original (`0x005E65xx`) solo se activa si el alcalde (`Town+0x6F1`) es el jugador local.
        if usize::from(self.sim.economy.world.towns[self.city].u8(0x6f1)) != self.owner {
            return Err("solo el alcalde dispone de las arcas de la ciudad".into());
        }
        if deposit && i64::from(amount) > i64::from(self.sim.economy.merchants[self.owner].i32(0)) {
            return Err("no tienes tanto oro".into());
        }
        self.run_operation(Operation([0x47, self.owner as u32, self.city as u32, u32::from(deposit), amount]))?;
        Ok(json!({"screen": self.town_hall_screen()?, "state": self.state()}))
    }

    /// Voto de la sesión del día (`0x005E8280`): `0x3F` en la elección del Ältermann o la Dieta (opciones 0..2) o `0x49` en el concejo de la ciudad.
    pub fn town_hall_vote(&mut self, council: bool, choice: u32) -> Result<Value, String> {
        if choice > 2 {
            return Err("opción de voto fuera de 0..=2".into());
        }
        let op = if council { Operation([0x49, self.owner as u32, self.city as u32, choice, 0]) } else { Operation([0x3f, self.owner as u32, choice, 0, 0]) };
        self.run_operation(op)?;
        Ok(json!({"screen": self.town_hall_screen()?, "state": self.state()}))
    }
}

impl Session {
    /// Taberna (`TavernSideMenu`): páginas de conversación disponibles hoy para el jugador en la ciudad (`0x005A6E60`).
    pub fn tavern_screen(&self) -> Result<Value, String> {
        let pages = self.sim.tavern_pages(self.city)?;
        let list = p3_sim::TAVERN_PAGES.iter().zip(pages).enumerate().map(|(i, (name, open))| json!({"page": i, "name": name, "available": open})).collect::<Vec<_>>();
        let hireable = self.sim.rep_merchants[self.owner].get_hireable_sailors(self.city as u32, &self.sim.economy.world.towns) & 0xff;
        let ships = self.sim.tavern_ships(self.city, self.owner).into_iter().map(|(id, crew, room, captain)| json!({"ship": id, "crew": crew, "room": room, "captain": captain})).collect::<Vec<_>>();
        Ok(json!({"city": self.city, "town_name": self.town_name(self.city), "gold": self.sim.economy.merchants[self.owner].i32(0), "pages": list, "hireable_sailors": hireable, "ships": ships, "news": self.tavern_news(),
                  "rivals": self.sim.tavern_rivals(self.city).into_iter().map(|(who, price)| json!({"merchant": who, "price": price})).collect::<Vec<_>>(),
                  "traveller": self.tavern_traveller(), "contracts": self.tavern_contracts(), "weapons": self.tavern_weapons(), "pirate": self.tavern_pirate(), "trader": self.tavern_trader_offer().map(|(rec, _)| json!({"ware": rec[0xe], "lots": u16::from_le_bytes([rec[8], rec[9]]), "price": i32::from_le_bytes(rec[4..8].try_into().unwrap()), "seller": rec[0xc]})),
                  "captains": self.sim.tavern_captains(self.city, self.owner).into_iter().map(|(i, kind, who)| json!({"captain": i, "kind": kind, "owner": who})).collect::<Vec<_>>()}))
    }

    /// Página del capitán pirata (`0x4E`): el capitán que ofrece la taberna y los barcos del jugador que puede llevarse (atracados o en puerto, sin capitán y con más
    /// del 20 % de casco). `crew` es el aviso de marineros extra que fija la ventana al abrirse (10..=20).
    pub fn tavern_pirate(&self) -> Option<Value> {
        let captain = self.sim.tavern_pirate_captain_id(self.city).ok().flatten()?;
        let nav = self.sim.host.navigation.as_ref()?;
        let ships = self
            .sim
            .tavern_ships(self.city, self.owner)
            .into_iter()
            .filter(|(id, _, _, cap)| {
                let img = &nav.snapshot.ships[id * 0x180..(id + 1) * 0x180];
                let state = u16::from_le_bytes([img[0x134], img[0x135]]);
                let (max, cur) = (i32::from_le_bytes(img[0x14..0x18].try_into().unwrap()), i32::from_le_bytes(img[0x18..0x1c].try_into().unwrap()));
                state <= 1 && u32::from(*cap) >= self.sim.captain_count().unwrap_or(0) as u32 && max > 0 && i64::from(cur) * 100 / i64::from(max) > 20
            })
            .map(|(id, ..)| id)
            .collect::<Vec<_>>();
        Some(json!({"captain": captain, "ships": ships}))
    }

    /// Contratar al capitán pirata para el barco `ship` (operación `0x4E`, `0x0053B850`): el barco pasa a la banda pirata más cercana con `crew` marineros más.
    pub fn tavern_hire_pirate(&mut self, ship: u32, crew: u32) -> Result<Value, String> {
        if !(10..=20).contains(&crew) {
            return Err("marineros extra fuera de 10..=20".into());
        }
        let page = self.tavern_pirate().ok_or("la taberna no ofrece capitán pirata")?;
        if !page["ships"].as_array().is_some_and(|s| s.iter().any(|v| v.as_u64() == Some(u64::from(ship)))) {
            return Err("ese barco no puede ir con el capitán pirata (atracado, sin capitán y con casco sano)".into());
        }
        let captain = page["captain"].as_u64().unwrap() as u32;
        // La ruta del barco entregado la calcula el planificador del mapa.
        self.ensure_planner()?;
        if self.sim.host.navigation_planner.is_none() {
            self.sim.host.navigation_planner = self.planner.clone();
        }
        self.run_operation(Operation([0x4e, captain, ship, crew, 0xff]))?;
        Ok(json!({"screen": self.tavern_screen()?, "state": self.state()}))
    }

    /// Noticias de la taberna (página 4, `0x00526B40` tipo `0x4C`): región del informante, ciudad que se olvida y precio. `None` si la página no está
    /// abierta o este día toca una carta de oferta (tipo `0x55`, que no se vende por esta orden).
    pub fn tavern_news(&self) -> Option<Value> {
        // `0x00673731 + 4 * informante`: región de cada informante.
        const REGION: [u8; 20] = [3, 1, 2, 3, 4, 0, 2, 3, 4, 0, 1, 3, 4, 0, 1, 2, 4, 0, 1, 2];
        let town = self.sim.economy.world.towns.get(self.city)?;
        let informant = town.u8(0x82c);
        if informant >= 0x14 || !self.sim.tavern_pages(self.city).ok()?[4] {
            return None;
        }
        if ((self.sim.idle.clock.tick >> 8).wrapping_add(u32::from(town.u8(0x2c0)))) % 3 == 0 {
            return None;
        }
        let region = REGION[usize::from(informant)];
        let me = &self.sim.rep_merchants[self.owner];
        let home = usize::from(me.bytes[0x19]);
        let cost = (u32::from(me.bytes[0x39c + home]) + 1) * 100;
        let locations = self.sim.host.settlement_locations.as_ref()?;
        let from = self.sim.economy.world.towns.iter().position(|t| locations.get(usize::from(t.u8(0x2c1))).is_some_and(|l| l.kind == region))?;
        Some(json!({"region": region, "from": from, "cost": cost}))
    }

    /// Comprar las noticias de la región (operación `0x4C`, `0x0053B4D0`).
    pub fn tavern_buy_news(&mut self) -> Result<Value, String> {
        let news = self.tavern_news().ok_or("el informante no tiene noticias hoy")?;
        let cost = news["cost"].as_i64().unwrap();
        if cost > i64::from(self.sim.economy.merchants[self.owner].i32(0)) {
            return Err("no tienes oro para las noticias".into());
        }
        self.run_operation(Operation([0x4c, self.owner as u32, self.city as u32, cost as u32, news["from"].as_u64().unwrap() as u32]))?;
        Ok(json!({"screen": self.tavern_screen()?, "state": self.state()}))
    }

    /// Encargar un robo al rival `victim` (operación `0x4F`, `0x0053BA40`): cuesta el precio de la lista; los días de espera salen del `rand` del CRT (`op+8`, 3 bits bajos, mínimo 3).
    pub fn tavern_burglary(&mut self, victim: u32) -> Result<Value, String> {
        let city = self.city;
        if !self.sim.tavern_pages(city)?[7] {
            return Err("el ladrón no está disponible en esta taberna".into());
        }
        let Some((_, price)) = self.sim.tavern_rivals(city).into_iter().find(|r| u32::from(r.0) == victim) else {
            return Err("ese rival no tiene oficina aquí o no se le puede robar".into());
        };
        if i64::from(price) > i64::from(self.sim.economy.merchants[self.owner].i32(0)) {
            return Err("no tienes oro para el encargo".into());
        }
        let roll = self.sim.host.investigation_rng.as_mut().map_or(0, |r| r.rand()) as u32;
        // Campos como los arma `0x005D6A08`: precio en +4, palabra aleatoria en +8, ciudad en +0xA, cliente en +0xC, víctima en +0xE.
        self.run_operation(Operation([0x4f, u32::from(price), (roll & 0xffff) | (city as u32) << 16, self.owner as u32 | victim << 16, 0]))?;
        Ok(json!({"screen": self.tavern_screen()?, "state": self.state()}))
    }

    /// Asignar un capitán de la taberna a un barco propio atracado aquí (operación `0x06`, `0x00537EE0`): el capitán anterior pasa a la lista de la ciudad.
    pub fn tavern_hire_captain(&mut self, captain: u32, ship: u32) -> Result<Value, String> {
        if !self.sim.tavern_captains(self.city, self.owner).iter().any(|c| u32::from(c.0) == captain) {
            return Err("ese capitán no está disponible en la taberna".into());
        }
        if !self.sim.tavern_ships(self.city, self.owner).iter().any(|s| s.0 == ship as usize) {
            return Err("ese barco no es tuyo o no está atracado aquí".into());
        }
        self.run_operation(Operation([0x06, captain, ship, self.city as u32, 0]))?;
        Ok(json!({"screen": self.tavern_screen()?, "state": self.state()}))
    }

    /// Oferta del comerciante de mercancías (página 1; `0x00526650`): el registro de 16 bytes que arma la ventana y que `0x4D` consume, y las mercancías
    /// que se pueden comprar con él. `None` si la página no está abierta o no hay excedente que ofrecer.
    pub fn tavern_trader_offer(&self) -> Option<([u8; 16], usize)> {
        if !self.sim.tavern_pages(self.city).ok()?[1] {
            return None;
        }
        let town = self.sim.economy.world.towns.get(self.city)?;
        let (records, head) = match &self.sim.host.navigation {
            Some(nav) => (nav.snapshot.ships.clone(), u32::from(u16::from_le_bytes([nav.snapshot.manager[0xe8], nav.snapshot.manager[0xe9]]))),
            None => (self.sim.host.expedition_factory.as_ref()?.ships.concat(), u32::from(self.sim.host.expedition_docked_head)),
        };
        let count = u32::from(self.sim.economy.world.ship_count);
        let mut rec = p3_core::economy::tavern_offer::create(town, &records, head, count, &self.sim.rep_merchants, self.sim.idle.clock.tick, self.owner as u8, self.sim.commercial_sell_factor, true)?;
        rec[0xf] = self.city as u8;
        Some((rec, 0))
    }

    /// Comprar la oferta del comerciante de la taberna (operación `0x4D`, `0x0053B5E0`): la mercancía va al barco `ship` y a los de su convoy si no cabe,
    /// o a la oficina de la ciudad si `ship` es `None`; se paga el precio total.
    pub fn tavern_buy_ware(&mut self, ship: Option<u32>) -> Result<Value, String> {
        let (mut rec, _) = self.tavern_trader_offer().ok_or("el comerciante no ofrece nada ahora")?;
        let price = i32::from_le_bytes(rec[4..8].try_into().unwrap());
        if self.sim.rep_merchants[self.owner].i32(0) < price {
            return Err("no tienes dinero para esa oferta".into());
        }
        match ship {
            Some(s) => {
                if !self.sim.tavern_ships(self.city, self.owner).iter().any(|x| x.0 == s as usize) {
                    return Err("ese barco no es tuyo o no está atracado aquí".into());
                }
                rec[0xa..0xc].copy_from_slice(&(s as u16).to_le_bytes());
            }
            None => {
                if !self.sim.persisted.iter().any(|o| usize::from(o.stock.owner) == self.owner && usize::from(o.stock.town) == self.city) {
                    return Err("no tienes oficina en esta ciudad".into());
                }
                rec[0xa..0xc].copy_from_slice(&self.sim.economy.world.ship_count.to_le_bytes());
            }
        }
        let w = |i: usize| u32::from_le_bytes([rec[i], rec[i + 1], rec[i + 2], rec[i + 3]]);
        self.run_operation(Operation([0x4d, w(0), w(4), w(8), w(12)]))?;
        Ok(json!({"screen": self.tavern_screen()?, "state": self.state()}))
    }

    /// Existencias de armas (mercancías `0x14..0x17`, en unidades) del destino de la página del armero: un barco del jugador atracado aquí, o su oficina si
    /// `ship` es `None`.
    fn weapons_held(&self, ship: Option<u32>) -> Result<[i32; 4], String> {
        match ship {
            Some(s) => {
                if !self.sim.tavern_ships(self.city, self.owner).iter().any(|x| x.0 == s as usize) {
                    return Err("ese barco no es tuyo o no está atracado aquí".into());
                }
                let c = self.sim.economy.cargo_ships.get(s as usize).ok_or("barco desconocido")?;
                Ok(std::array::from_fn(|k| c.stock[0x14 + k]))
            }
            None => {
                let o = self.sim.persisted.iter().find(|o| usize::from(o.stock.owner) == self.owner && usize::from(o.stock.town) == self.city).ok_or("no tienes oficina en esta ciudad")?;
                Ok(std::array::from_fn(|k| o.stock.stock[0x14 + k]))
            }
        }
    }

    /// Armero (página 6; `0x00526470`): por clase de arma, decenas en existencia y precio por arma al comprar o vender 1, 5 o 50. `None` si la página no está abierta.
    pub fn tavern_weapons(&self) -> Option<Value> {
        if !self.sim.tavern_pages(self.city).ok()?[6] {
            return None;
        }
        let stock = self.sim.weapon_quote(self.city, 0, [0; 4])?.0;
        let quote = |kind| self.sim.weapon_quote(self.city, kind, [0; 4]);
        let (q1, q5, q50) = (quote(0)?, quote(1)?, quote(2)?);
        Some(json!((0..4).map(|k| json!({"class": k, "ware": 0x14 + k, "stock10": stock[k], "buy": [q1.2[k], q5.2[k], q50.2[k]], "sell": [q1.1[k], q5.1[k], q50.1[k]]})).collect::<Vec<_>>()))
    }

    /// Comprar (`buy`) o vender armas al armero (operación `0x50`, `0x0053BC90`): `quantity` 1, 5 o 50 armas, o `0` para todo lo que haya (comprar) o
    /// lo que se tenga (vender, tope 50). El destino es el barco `ship` o la oficina de la ciudad.
    pub fn tavern_weapons_deal(&mut self, class: u32, quantity: u32, buy: bool, ship: Option<u32>) -> Result<Value, String> {
        if class > 3 {
            return Err("clase de arma fuera de 0..=3".into());
        }
        if !self.sim.tavern_pages(self.city)?[6] {
            return Err("el armero no está hoy en la taberna".into());
        }
        let kind = match quantity {
            1 => 0,
            5 => 1,
            50 => 2,
            0 => 3,
            _ => return Err("cantidad de armas: 1, 5, 50 o 0 para todo".into()),
        };
        let held = self.weapons_held(ship)?;
        let held10: [i32; 4] = std::array::from_fn(|k| held[k] / 10);
        let (stock10, sell, buy_price) = self.sim.weapon_quote(self.city, kind, held10).ok_or("ciudad desconocida")?;
        let k = class as usize;
        let (price, count) = if buy {
            (buy_price[k], if kind == 3 { stock10[k] } else { quantity as i32 })
        } else {
            (sell[k], if kind == 3 { held10[k].min(50) } else { quantity as i32 })
        };
        if count <= 0 || price <= 0 {
            return Err("no hay armas que comprar o vender con esa cantidad".into());
        }
        let target = match ship {
            Some(s) => s,
            None => u32::from(self.sim.economy.world.ship_count),
        };
        let packed = self.city as u32 | (self.owner as u32) << 8 | class << 16 | u32::from(buy) << 24;
        self.run_operation(Operation([0x50, price as u32, count as u32, packed, target]))?;
        Ok(json!({"screen": self.tavern_screen()?, "state": self.state()}))
    }

    /// Contratos de suministro de la ciudad que son del comerciante local (página 0; tareas `0x11` en las ranuras `Town+0x830/0x832`): mercancía,
    /// lo que falta, precio por unidad y las unidades que se pueden entregar con lo que tiene en barcos de la ciudad y en su oficina.
    pub fn tavern_contracts(&self) -> Vec<Value> {
        let Some(town) = self.sim.economy.world.towns.get(self.city) else { return Vec::new() };
        let sched = &self.sim.host.council_scheduler;
        let mut out = Vec::new();
        for k in 0..2 {
            let idx = usize::from(town.u16(0x830 + 2 * k));
            let Some(r) = sched.records.get(idx).filter(|_| idx < usize::from(sched.capacity)) else { continue };
            if r.kind != 0x11 || usize::from(r.u8_at(0x16)) != self.owner || r.u8_at(9) >= 0x14 {
                continue;
            }
            let ware = usize::from(r.u8_at(9));
            let unit: i64 = if p3_core::economy::prices::SMALL_LOT_WARES[ware] { 200 } else { 2000 };
            let mut have: i64 = self.sim.economy.offices.iter().find(|o| usize::from(o.owner) == self.owner && usize::from(o.town) == self.city).map_or(0, |o| i64::from(o.stock[ware]).max(0));
            for (id, ..) in self.sim.tavern_ships(self.city, self.owner) {
                have += self.sim.economy.cargo_ships.get(id).map_or(0, |c| i64::from(c.stock[ware]).max(0));
            }
            out.push(json!({"task": idx, "ware": ware, "remaining": (u16::from(r.u8_at(0x10)) | u16::from(r.u8_at(0x11)) << 8), "price": (u16::from(r.u8_at(0x14)) | u16::from(r.u8_at(0x15)) << 8), "paid": r.u8_at(0x17) == 0, "deliverable": (have / unit).min(i64::from((u16::from(r.u8_at(0x10)) | u16::from(r.u8_at(0x11)) << 8)))}));
        }
        out
    }

    /// Entregar `amount` unidades a un contrato de suministro (operación `0x51`, `0x0053C050`): sale de los barcos de la ciudad y luego de la oficina.
    pub fn tavern_deliver(&mut self, task: u32, amount: u32) -> Result<Value, String> {
        let list = self.tavern_contracts();
        let contract = list.iter().find(|c| c["task"] == task).ok_or("ese contrato no es tuyo o no está en esta taberna")?;
        let ware = contract["ware"].as_u64().unwrap() as usize;
        let before = contract["remaining"].as_u64().unwrap();
        if amount == 0 || contract["deliverable"].as_i64().unwrap() == 0 {
            return Err("no tienes mercancía para entregar".into());
        }
        let _ = ware;
        self.run_operation(Operation([0x51, self.owner as u32, 0, task, amount]))?;
        let after = self.tavern_contracts().iter().find(|c| c["task"] == task).map_or(0, |c| c["remaining"].as_u64().unwrap());
        Ok(json!({"delivered": before - after, "screen": self.tavern_screen()?, "state": self.state()}))
    }

    /// Oferta del viajero (página 5; `Town+0x834` destino, `+0x836` pago, `+0x838` fecha límite): `None` si la página no está abierta o ya no queda
    /// margen (la ventana exige más de `0x700` ticks hasta la fecha, `0x005D6E1F`).
    pub fn tavern_traveller(&self) -> Option<Value> {
        let town = self.sim.economy.world.towns.get(self.city)?;
        if !self.sim.tavern_pages(self.city).ok()?[5] || town.u8(0x834) == 0xff {
            return None;
        }
        let (dest, price, due) = (usize::from(town.u8(0x834)), town.u16(0x836), town.u32(0x838));
        let left = due.wrapping_sub(self.sim.idle.clock.tick) as i32;
        Some(json!({"dest": dest, "dest_name": self.town_name(dest), "price": price, "due": due, "late": left <= 0x700}))
    }

    /// Aceptar el pasaje con un barco propio atracado aquí (operación `0x53`, `0x0053C840`): crea la tarea `0x10`, avisa al dueño y reserva la oferta.
    pub fn tavern_passage(&mut self, ship: u32) -> Result<Value, String> {
        let offer = self.tavern_traveller().ok_or("no hay viajero que ofrezca pasaje")?;
        if offer["late"] == true {
            return Err("la fecha límite del viajero es demasiado próxima".into());
        }
        if !self.sim.tavern_ships(self.city, self.owner).iter().any(|s| s.0 == ship as usize) {
            return Err("ese barco no es tuyo o no está atracado aquí".into());
        }
        let (dest, price, due) = (offer["dest"].as_u64().unwrap() as u32, offer["price"].as_u64().unwrap() as u32, offer["due"].as_u64().unwrap() as u32);
        // Cuatro palabras como las arma `0x005D6E36`: barco (+4) y pago (+6); fecha (+8); ciudad (+0xC) y destino (+0xD).
        self.run_operation(Operation([0x53, ship | (price << 16), due, self.city as u32 | (dest << 8), 0]))?;
        if self.tavern_traveller().is_some() {
            return Err("la taberna rechazó el pasaje (destino o tamaño del barco)".into());
        }
        Ok(json!({"screen": self.tavern_screen()?, "state": self.state()}))
    }

    /// Contratar marineros para un barco propio atracado aquí (operación `0x04`, `0x00537C20`): se limita a lo que cabe y a la reserva de la ciudad.
    pub fn tavern_hire_sailors(&mut self, ship: u32, amount: u32) -> Result<Value, String> {
        if amount == 0 {
            return Err("cantidad nula".into());
        }
        if !self.sim.tavern_ships(self.city, self.owner).iter().any(|s| s.0 == ship as usize) {
            return Err("ese barco no es tuyo o no está atracado aquí".into());
        }
        self.run_operation(Operation([0x04, ship, amount, 0, 0]))?;
        Ok(json!({"screen": self.tavern_screen()?, "state": self.state()}))
    }
}

impl Session {
    /// Hansa: propuestas del día hanseático pendientes (tareas `0x30` con `+8 != 0`; `0x0053B1F0`).
    pub fn hanse_screen(&self) -> Result<Value, String> {
        let s = &self.sim.host.council_scheduler;
        let mut at = s.head;
        let mut proposals = Vec::new();
        while at < s.capacity && proposals.len() < s.records.len() {
            let Some(r) = s.records.get(usize::from(at)) else { break };
            if r.kind == 0x30 && r.u32_at(8) != 0 {
                proposals.push(json!({"task": at, "due": r.due, "kind_byte": r.u8_at(0xc), "bytes": [r.u8_at(0x12), r.u8_at(0x13), r.u8_at(0x14), r.u8_at(0x15)]}));
            }
            at = r.next;
        }
        Ok(json!({"city": self.city, "town_name": self.town_name(self.city), "tick": self.sim.idle.clock.tick, "proposals": proposals}))
    }

    /// Decidir una propuesta: `reject` (código 6) o aceptarla con `code` y fecha `due` (>= hoy + 0x700, `0x0053B1F0`).
    pub fn hanse_decide(&mut self, task: u32, code: u32, due: u32) -> Result<Value, String> {
        self.run_operation(Operation([0x4a, self.owner as u32, due, task, code]))?;
        Ok(json!({"screen": self.hanse_screen()?, "state": self.state()}))
    }
}

impl Session {
    /// Oficina del jugador en la ciudad: comercio automático por mercancía (`+0x2F4` cantidad, `+0x354` precio; `+0x2D6` bit 0 = activo).
    pub fn office_screen(&self) -> Result<Value, String> {
        let images = self.sim.host.scheduled_office_images.as_ref().ok_or("oficinas vivas ausentes")?;
        let i = self.sim.persisted.iter().position(|o| usize::from(o.stock.owner) == self.owner && usize::from(o.stock.town) == self.city).ok_or("no tienes oficina en esta ciudad")?;
        let o = images.get(i).ok_or("oficina viva ausente")?;
        let word = |at: usize| i32::from_le_bytes(o[at..at + 4].try_into().unwrap());
        let admin = self.sim.captain_count().is_some_and(|n| usize::from(u16::from_le_bytes([o[0x2f2], o[0x2f3]])) < n);
        let wares = (0..24).map(|k| json!({"ware": k, "amount": word(0x2f4 + 4 * k), "price": word(0x354 + 4 * k)})).collect::<Vec<_>>();
        Ok(json!({"city": self.city, "town_name": self.town_name(self.city), "office": i, "autotrade_on": o[0x2d6] & 3 != 0, "administrator": admin,
                  "administrator_wage": self.sim.wages.get(i).copied().flatten().filter(|_| admin), "administrator_skill": self.sim.persisted[i].stock.administrator_experience.filter(|_| admin), "wares": wares}))
    }

    /// Astillero (`ShipyardWindow`): barcos del jugador en la ciudad con su nivel, estado y presupuesto de subir un nivel
    /// (`0x0052B670`: coste y días de obra en las palabras 12 y 13, y si el dinero alcanza).
    pub fn shipyard_screen(&mut self) -> Result<Value, String> {
        let ships = self.sim.tavern_ships(self.city, self.owner);
        let mut list = Vec::new();
        for (id, ..) in ships {
            let image = match &self.sim.host.navigation {
                Some(nav) => nav.snapshot.ships[id * 0x180..(id + 1) * 0x180].to_vec(),
                None => self.sim.host.expedition_ship_images.get(id).cloned().flatten().ok_or("barco sin imagen")?,
            };
            let state = u16::from_le_bytes([image[0x134], image[0x135]]);
            let docked = self.sim.shipyard_is_docked(id as u32)?;
            let quote = self.sim.shipyard_quote(id as u32, 1)?.filter(|_| docked).map(|(ok, q)| json!({"affordable": ok, "cost": q[12], "days": q[13]}));
            list.push(json!({"ship": id, "level": image[0xf], "state": state, "in_yard": state == 7, "docked": docked, "quote": quote}));
        }
        let names = ["snaikka", "crayer", "cog", "hulk"];
        let mut orders = Vec::new();
        for kind in 0..4u8 {
            let cost = self.sim.shipyard_order_quote(self.city, self.owner, kind).ok().flatten();
            orders.push(json!({"kind": kind, "name": names[usize::from(kind)], "cost": cost}));
        }
        let queue = self.shipyard_queue()?;
        Ok(json!({"city": self.city, "town_name": self.town_name(self.city), "gold": self.sim.economy.merchants[self.owner].i32(0), "ships": list, "orders": orders, "queue": queue,
                  "convoys": self.sim.shipyard_convoys(self.city, self.owner)?.into_iter().map(|(c, state, cost)| json!({"convoy": c, "state": state, "cost": cost})).collect::<Vec<_>>()}))
    }

    /// Barcos del jugador en la cola de obra de la ciudad (`Town+0x81C`, enlace `+6`): clase, progreso y lo que devolvería cancelarlos (`+0x48 · 9 / 10`, solo si la obra no empezó).
    fn shipyard_queue(&self) -> Result<Vec<Value>, String> {
        let nav = self.sim.host.navigation.as_ref().ok_or("navegación ausente")?;
        let ships = &nav.snapshot.ships;
        let count = ships.len() / 0x180;
        let word = |i: usize, at: usize| i32::from_le_bytes(ships[i * 0x180 + at..i * 0x180 + at + 4].try_into().unwrap());
        let mut at = usize::from(self.sim.economy.world.towns[self.city].u16(0x81c));
        let (mut out, mut steps) = (Vec::new(), 0);
        while at < count && steps <= count {
            if usize::from(ships[at * 0x180]) == self.owner {
                let started = word(at, 0x18) > 0;
                out.push(json!({"ship": at, "kind": ships[at * 0x180 + 0xe], "level": ships[at * 0x180 + 0xf], "progress": word(at, 0x18), "target": word(at, 0x14), "refund": if started { None } else { Some(word(at, 0x48).wrapping_mul(9).wrapping_div(10)) }}));
            }
            at = usize::from(u16::from_le_bytes([ships[at * 0x180 + 6], ships[at * 0x180 + 7]]));
            steps += 1;
        }
        Ok(out)
    }

    /// Cancelar un encargo de la cola del astillero (operación `0x2F`, `0x0052B110`): si la obra no empezó devuelve el 90 % del coste; en cualquier caso el dueño recibe la carta.
    pub fn shipyard_cancel_order(&mut self, ship: u32) -> Result<Value, String> {
        if !self.shipyard_queue()?.iter().any(|o| o["ship"].as_u64() == Some(u64::from(ship))) {
            return Err("ese barco no es tuyo o no está en la cola de este astillero".into());
        }
        self.run_operation(Operation([0x2f, self.owner as u32, self.city as u32, ship, 0]))?;
        Ok(json!({"screen": self.shipyard_screen()?, "state": self.state()}))
    }

    /// Encargar un barco nuevo de clase `kind` (0 snaikka, 1 crayer, 2 cog, 3 hulk; operación `0x2C`, `0x0052A360`): usa los materiales de la oficina, de los barcos
    /// del puerto y de la ciudad, cobra el coste y deja el barco en la cola de obra del astillero.
    pub fn shipyard_order(&mut self, kind: u32) -> Result<Value, String> {
        if kind > 3 {
            return Err("clase de barco fuera de 0..=3".into());
        }
        match self.sim.shipyard_order_ship(self.city, self.owner, kind as u8)? {
            Some(_) => Ok(json!({"screen": self.shipyard_screen()?, "state": self.state()})),
            None => Err("el astillero no puede construirlo ahora: faltan materiales, dinero o nivel".into()),
        }
    }

    /// Reparar un convoy entero (operación `0x1D`, `0x00539990`): el convoy `convoy` pasa a estado 4 y se cobra el coste de reparar todos sus barcos.
    pub fn shipyard_repair_convoy(&mut self, convoy: u32) -> Result<Value, String> {
        let (_, _, cost) = self.sim.shipyard_convoys(self.city, self.owner)?.into_iter().find(|c| c.0 == convoy as usize).ok_or("ese convoy no es tuyo, no está en este puerto o ya está en reparación")?;
        if self.sim.economy.merchants[self.owner].i32(0) < cost {
            return Err("no tienes dinero para reparar el convoy".into());
        }
        self.run_operation(Operation([0x1d, convoy, 0, 0, 0]))?;
        Ok(json!({"screen": self.shipyard_screen()?, "state": self.state()}))
    }

    /// Mejorar `levels` niveles el barco `ship` (operación `0x0C`, `start_repair` 0x00508660): reserva materiales y dinero y lo pasa a la cola del astillero.
    pub fn shipyard_upgrade(&mut self, ship: u32, levels: u32) -> Result<Value, String> {
        if levels == 0 {
            return Err("niveles a mejorar: 0".into());
        }
        if !self.sim.shipyard_is_docked(ship)? {
            return Err("el barco no está atracado en el puerto".into());
        }
        let (affordable, _) = self.sim.shipyard_quote(ship, levels)?.ok_or("barco inexistente")?;
        if !affordable {
            return Err("la mejora no es posible o no hay dinero suficiente".into());
        }
        self.run_operation(Operation([0x0c, ship, levels, 0, 0]))?;
        Ok(json!({"screen": self.shipyard_screen()?, "state": self.state()}))
    }

    /// Fijar cantidad y precio de comercio automático de una mercancía (operación `0x5B`, `0x0053D5C0`).
    /// Contratar (o, si ya hay uno, despedir) al administrador de la oficina de esta ciudad (`0x5E`, `0x0053D990`). Como el botón del original,
    /// no consulta el oro ni el jornal.
    pub fn office_administrator(&mut self, hire: bool) -> Result<Value, String> {
        let screen = self.office_screen()?;
        let has = screen["administrator"] == true;
        if hire == has {
            return Err(if has { "la oficina ya tiene administrador" } else { "la oficina no tiene administrador" }.into());
        }
        self.run_operation(Operation([0x5e, self.owner as u32, self.city as u32, u32::from(has), 0]))?;
        Ok(json!({"screen": self.office_screen()?, "state": self.state()}))
    }

    pub fn office_autotrade(&mut self, ware: u32, amount: u32, price: u32) -> Result<Value, String> {
        if ware >= 24 {
            return Err("mercancía fuera de rango".into());
        }
        let office = self.office_screen()?["office"].as_u64().ok_or("oficina sin índice")? as u32;
        self.run_operation(Operation([0x5b, price, amount, office, ware]))?;
        Ok(json!({"screen": self.office_screen()?, "state": self.state()}))
    }
}

/// Existencias y naves sobre las que opera «alimentar a los pobres», copiadas de las vistas vivas de la sesión.
struct FeedView {
    office: Option<[i32; 24]>,
    first: u16,
    ships: Vec<(u16, u8, [i32; 24])>, // (siguiente, ciudad, carga)
    prices: Vec<(u32, u32, u32)>,
    markets: Vec<p3_core::economy::prices::Market>,
    difficulty: f32,
}
impl p3_core::economy::church::FeedHost for FeedView {
    fn office(&mut self) -> Option<&mut [i32; 24]> {
        self.office.as_mut()
    }
    fn first_ship(&self) -> u16 {
        self.first
    }
    fn ship_count(&self) -> u16 {
        self.ships.len() as u16
    }
    fn ship_next(&self, ship: u16) -> u16 {
        self.ships[usize::from(ship)].0
    }
    fn ship_town(&self, ship: u16) -> u8 {
        self.ships[usize::from(ship)].1
    }
    fn ship_cargo(&mut self, ship: u16) -> &mut [i32; 24] {
        &mut self.ships[usize::from(ship)].2
    }
    fn price(&mut self, ware: u32, _town: u32, amount: u32) -> i32 {
        self.prices.push((ware, _town, amount));
        p3_core::economy::prices::sell_price(ware as usize, &self.markets[ware as usize], amount as i32, self.difficulty)
    }
}

impl Session {
    fn feed_view(&self) -> FeedView {
        let town = &self.sim.economy.world.towns[self.city];
        let office = self.sim.persisted.iter().find(|o| usize::from(o.stock.owner) == self.owner && usize::from(o.stock.town) == self.city);
        let ships = self.sim.host.expedition_ship_images.iter().enumerate().map(|(i, image)| {
            let cargo = self.sim.economy.cargo_ships.get(i).map_or([0; 24], |c| c.stock);
            image.as_ref().filter(|v| v.len() >= 0x180).map_or((0xffff, 0xff, [0; 24]), |v| (u16::from_le_bytes([v[4], v[5]]), v[0x39], cargo))
        });
        FeedView {
            office: office.map(|o| {
                let mut stock = [0i32; 24];
                stock.copy_from_slice(&o.stock.stock[..24]);
                stock
            }),
            first: self.sim.rep_merchants[self.owner].u16(0xe),
            ships: ships.collect(),
            prices: vec![],
            markets: (0..24)
                .map(|w| p3_core::economy::prices::Market {
                    trading: town.more_flags() & 0x100 != 0,
                    stock: town.stock(w),
                    thresholds: std::array::from_fn(|i| town.threshold(w, i)),
                    marked_ware: town.u8(0x6f0),
                })
                .collect(),
            difficulty: self.sim.commercial_sell_factor,
        }
    }

    /// Lo que puede entregarse a los pobres (`0x005CAAB0`): por mercancía, lotes de la oficina y de los barcos propios atracados aquí.
    pub fn church_feed_screen(&self) -> Value {
        use p3_core::economy::church::FEED_WARES;
        let view = self.feed_view();
        let lots: Vec<i32> = FEED_WARES
            .iter()
            .map(|&w| {
                let unit = if p3_core::economy::prices::SMALL_LOT_WARES[w] { 200 } else { 2000 };
                let mut have = view.office.map_or(0, |o| o[w]);
                for (i, (_, town, cargo)) in view.ships.iter().enumerate() {
                    let image = self.sim.host.expedition_ship_images[i].as_ref();
                    if usize::from(*town) == self.city && image.is_some_and(|v| v.len() > 0x136 && v[0x136] & 1 == 0) && self.sim.economy.cargo_ships.get(i).is_some_and(|c| usize::from(c.owner) == self.owner) {
                        have += cargo[w];
                    }
                }
                have.max(0) / unit
            })
            .collect();
        json!({"wares": FEED_WARES, "lots": lots})
    }

    /// Entregar `amounts` lotes a los pobres: puntuación `total / (√(población · … / 18) + 8)` (`0x005CAAB0`) y operación `0x30` (`0x004FE557`).
    pub fn church_feed_poor(&mut self, amounts: [u16; 5]) -> Result<Value, String> {
        use p3_core::economy::church::{feed_poor, FeedHost, FEED_WARES};
        self.ensure_no_battle()?;
        let lots = self.church_feed_screen()["lots"].as_array().unwrap().iter().map(|v| v.as_i64().unwrap()).collect::<Vec<_>>();
        if amounts.iter().zip(&lots).any(|(&a, &l)| i64::from(a) > l) {
            return Err("no hay tanta mercancía disponible".into());
        }
        if amounts.iter().all(|&a| a == 0) {
            return Err("no hay nada que entregar".into());
        }
        let mut view = self.feed_view();
        let town = &self.sim.economy.world.towns[self.city];
        let mut total = 0i64;
        for k in 0..5 {
            let w = FEED_WARES[k];
            let unit = if p3_core::economy::prices::SMALL_LOT_WARES[w] { 200 } else { 2000 };
            total += i64::from(view.price(w as u32, self.city as u32, u32::from(amounts[k]) * unit));
        }
        let x = i64::from(town.i16(0x304)) * i64::from(town.i32(0x2d4)) / 18;
        let divisor = (x.max(0) as f64).sqrt() as i64 + 8;
        let score = (total / divisor).clamp(0, 255) as u8;
        view.prices.clear();
        let count = self.sim.economy.world.merchant_count;
        let result = feed_poor(&mut view, count, self.owner as u8, self.city as u8, score, amounts).ok_or("comerciante inexistente")?;
        // Publicar: oficina, carga de los barcos en todas sus vistas, reputación y marca de la ciudad.
        if let (Some(stock), Some(i)) = (view.office, self.sim.persisted.iter().position(|o| usize::from(o.stock.owner) == self.owner && usize::from(o.stock.town) == self.city)) {
            self.sim.persisted[i].stock.stock[..24].copy_from_slice(&stock);
            self.sim.economy.offices[i] = self.sim.persisted[i].stock.clone();
        }
        for (id, (_, _, cargo)) in view.ships.iter().enumerate() {
            let Some(c) = self.sim.economy.cargo_ships.get_mut(id) else { continue };
            if c.stock == *cargo {
                continue;
            }
            c.stock = *cargo;
            let write = |image: &mut [u8]| {
                for w in 0..24 {
                    image[0x54 + 4 * w..0x58 + 4 * w].copy_from_slice(&cargo[w].to_le_bytes());
                }
            };
            if let Some(image) = self.sim.host.expedition_ship_images[id].as_mut() {
                write(image);
            }
            if let Some(nav) = self.sim.host.navigation.as_mut() {
                write(&mut nav.snapshot.ships[id * 0x180..(id + 1) * 0x180]);
            }
            if !self.changed_ships.contains(&id) {
                self.changed_ships.push(id)
            }
        }
        if let Some(value) = result.reputation {
            let env = p3_core::ai::reputation::RepEnv { town_count: self.sim.economy.world.towns.len() as u32, towns: &self.sim.economy.world.towns, renown: [0; 3], byte_52c: self.sim.host.church_cost_factor };
            let mut merchant = self.sim.rep_merchants[self.owner].clone();
            merchant.add_social_reputation_church_donation(&env, self.city as u32, value);
            self.sim.economy.merchants[self.owner] = merchant.clone();
            self.sim.rep_merchants[self.owner] = merchant;
        }
        if result.flag {
            let t = &mut self.sim.economy.world.towns[self.city];
            let v = t.u32(0x2c8) | 0x80_0000;
            t.set_u32(0x2c8, v);
        }
        Ok(json!({"given": result.given, "score": score, "reputation": result.reputation, "marked": result.flag, "screen": self.church_screen()?, "feed": self.church_feed_screen(), "state": self.state()}))
    }
}

/// Operación `0x29` de la ventana: palabra 0 = código, palabras 1..5 = registro de 16 bytes (`0x63A790` copia 0x10 bytes a `op+4`).
fn grant_operation(rec: &[u8; 0x10]) -> Operation {
    let w = |i: usize| u32::from_le_bytes([rec[i], rec[i + 1], rec[i + 2], rec[i + 3]]);
    Operation([0x29, w(0), w(4), w(8), w(12)])
}

fn arg(args: &Value, key: &str) -> Result<usize, String> {
    args[key].as_u64().map(|n| n as usize).ok_or_else(|| format!("falta {key}"))
}

impl Session {
    /// Nombres de las pantallas disponibles (`screen`/`screen_action`).
    pub const SCREENS: [&'static str; 16] = ["lender", "church", "town_hall", "tavern", "hanse", "office", "shipyard", "letters", "market", "fleet", "warehouse", "baths", "weapons", "guild", "found", "statistics"];

    /// Lo que muestra la ventana `name` ahora mismo.
    pub fn screen(&mut self, name: &str) -> Result<Value, String> {
        self.sim.ensure_office_images();
        match name {
            "lender" => self.lender_screen(),
            "church" => self.church_screen(),
            "town_hall" => self.town_hall_screen(),
            "tavern" => self.tavern_screen(),
            "hanse" => self.hanse_screen(),
            "office" => self.office_screen(),
            "shipyard" => self.shipyard_screen(),
            "letters" => self.letters_screen(),
            "market" => self.market_screen(),
            "fleet" => self.fleet_screen(),
            "warehouse" => self.warehouse_screen(),
            "baths" => self.baths_screen(),
            "weapons" => self.weapons_screen(),
            "guild" => self.guild_screen(),
            "found" => self.found_screen(),
            "statistics" => self.statistics_screen(),
            _ => Err(format!("pantalla desconocida: {name}")),
        }
    }

    /// Pulsación de un botón de la ventana `name`; devuelve la ventana actualizada y el estado de la sesión.
    pub fn screen_action(&mut self, name: &str, action: &str, args: &Value) -> Result<Value, String> {
        self.sim.ensure_office_images();
        if let Some(done) = self.screen_trade_action(name, action, args) {
            return done;
        }
        match (name, action) {
            ("lender", "take") => self.lender_take_loan(arg(args, "index")?),
            ("lender", "repay") => self.lender_repay(arg(args, "task")?),
            ("lender", "grant") => self.lender_grant(arg(args, "index")?, arg(args, "interest")? as u8),
            ("church", "redecorate") => self.church_redecorate(arg(args, "amount")? as i32),
            ("church", "feed_poor") => {
                let list = args["amounts"].as_array().filter(|a| a.len() == 5).ok_or("faltan cinco cantidades")?;
                let mut amounts = [0u16; 5];
                for (k, v) in list.iter().enumerate() {
                    amounts[k] = v.as_u64().and_then(|n| u16::try_from(n).ok()).ok_or("cantidad inválida")?;
                }
                self.church_feed_poor(amounts)
            }
            ("church", "extend") => self.church_extend(arg(args, "amount")? as i32),
            ("town_hall", "candidacy") => self.town_hall_candidacy(arg(args, "candidate")? != 0),
            ("town_hall", "treasury") => self.town_hall_treasury(arg(args, "deposit")? != 0, arg(args, "amount")? as u32),
            ("hanse", "decide") => self.hanse_decide(arg(args, "task")? as u32, arg(args, "code")? as u32, arg(args, "due")? as u32),
            ("shipyard", "repair_convoy") => self.shipyard_repair_convoy(arg(args, "convoy")? as u32),
            ("tavern", "hire_pirate") => self.tavern_hire_pirate(arg(args, "ship")? as u32, arg(args, "crew")? as u32),
            ("shipyard", "cancel_order") => self.shipyard_cancel_order(arg(args, "ship")? as u32),
            ("shipyard", "order") => self.shipyard_order(arg(args, "kind")? as u32),
            ("shipyard", "upgrade") => self.shipyard_upgrade(arg(args, "ship")? as u32, arg(args, "levels")? as u32),
            ("office", "hire_administrator") => self.office_administrator(true),
            ("office", "fire_administrator") => self.office_administrator(false),
            ("office", "autotrade") => self.office_autotrade(arg(args, "ware")? as u32, arg(args, "amount")? as u32, arg(args, "price")? as u32),
            ("tavern", "hire_captain") => self.tavern_hire_captain(arg(args, "captain")? as u32, arg(args, "ship")? as u32),
            ("tavern", "weapons") => self.tavern_weapons_deal(arg(args, "class")? as u32, arg(args, "quantity")? as u32, arg(args, "buy")? != 0, args["ship"].as_u64().map(|n| n as u32)),
            ("tavern", "buy_ware") => self.tavern_buy_ware(args["ship"].as_u64().map(|n| n as u32)),
            ("tavern", "deliver") => self.tavern_deliver(arg(args, "task")? as u32, arg(args, "amount")? as u32),
            ("tavern", "passage") => self.tavern_passage(arg(args, "ship")? as u32),
            ("tavern", "burglary") => self.tavern_burglary(arg(args, "victim")? as u32),
            ("tavern", "buy_news") => self.tavern_buy_news(),
            ("tavern", "hire_sailors") => self.tavern_hire_sailors(arg(args, "ship")? as u32, arg(args, "amount")? as u32),
            ("town_hall", "vote") => self.town_hall_vote(arg(args, "council")? != 0, arg(args, "choice")? as u32),
            _ => Err(format!("acción desconocida: {name}/{action}")),
        }
    }
}
