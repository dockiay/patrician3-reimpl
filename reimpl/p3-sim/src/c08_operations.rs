//! C08: operaciones de eventos, política y progresión en el despachador de la simulación.
//!
//! Cada operación se aplica sobre copias de las vistas vivas y solo se publica si termina sin
//! error; así un fallo deja la operación en la cola sin cambios parciales. Las dos copias del
//! comerciante (`rep_merchants` y `economy.merchants`) y la cuenta (`world.merchants`) se mantienen
//! sincronizadas en cada publicación. Contrastes con el despachador original en
//! `tests/c08_*_live_x86.rs`.
use crate::{RepMerchant, Sim};
use p3_core::pump::Operation;

/// Operaciones C08 que este módulo despacha.
pub const OPERATIONS: [u32; 50] = [0x5d, 0x5e, 0x39, 0x56, 0x5c, 0x15, 0x16, 0x17, 0x18, 0x4d, 0x50, 0x1d, 0x51, 0x0c, 0x04, 0x06, 0x28, 0x29, 0x2a, 0x31, 0x32, 0x3f, 0x42, 0x43, 0x45, 0x49, 0x4a, 0x4c, 0x4f, 0x52, 0x55, 0x6e, 0x6f, 0x70, 0x71, 0x65, 0x74, 0x76, 0x53, 0x54, 0x82, 0x47, 0x5b, 0x2f, 0x4e, 0x05, 0x10, 0x08, 0x11, 0x1b];

impl Sim {
    /// `Some(resultado)` si `op` es una operación C08 despachada aquí; `None` si no lo es.
    pub(crate) fn apply_c08_operation(&mut self, op: Operation) -> Option<Result<(), String>> {
        let code = op.0[0];
        if !OPERATIONS.contains(&code) {
            return None;
        }
        Some(match code {
            0x28 => self.c08_flat(op, |m, host| p3_core::economy::ops_misc::put_building_up_for_auction(m, host, crate::c08_flat::OP)),
            0x65 => self.c06_bankruptcy(op),
            0x29 => self.c08_grant_loan(op),
            0x2a => self.c08_flat(op, |m, _| p3_core::economy::council_small::op_repay_loan(m, crate::c08_flat::OP)),
            0x4a => self.c08_flat(op, |m, host| p3_core::economy::ops_misc::decide_hanse_day_proposal(m, host, crate::c08_flat::OP)),
            0x76 => self.c08_script_offer(op),
            0x52 => self.c08_tavern_order(op),
            0x4c => self.c08_region_news(op),
            0x4f => self.c08_flat(op, |m, host| p3_core::economy::c08_burglary::commission_burglary(m, host, crate::c08_flat::OP)),
            0x74 => self.c08_flat(op, |m, calls| {
                if let Err(e) = p3_core::economy::c08_script_letters::answer_script_letter(m, crate::c08_flat::OP) {
                    calls.error.get_or_insert(e.to_string());
                }
            }),
            0x6e | 0x6f | 0x70 | 0x71 | 0x55 => self.c08_letter_operation(op),
            0x31 => self.c08_church_donation(op),
            0x32 => self.c08_town_donation(op),
            0x42 => self.c08_bribe(op, true),
            0x43 => self.c08_bribe(op, false),
            0x3f => self.c08_cast_ballot(op),
            0x45 => self.c08_candidacy(op),
            0x0c => self.shipyard_start_repair(op.0[1], op.0[2]),
            0x4d | 0x50 | 0x51 | 0x1d | 0x08 | 0x1b => self.c08_supply_contract(op),
            0x39 => self.c08_flat(op, |m, host| p3_core::economy::ops_office::bathe_set_merchant_field_4(m, host, crate::c08_flat::OP)),
            0x5e => self.c08_administrator_operation(op),
            0x5d => self.c08_found_office(op),
            0x56 => self.c08_flat(op, |m, host| p3_core::economy::ops_office::op_buy_sell_ship_armament_lot(m, host, crate::c08_flat::OP)),
            0x5c => self.c08_flat(op, |m, host| p3_core::economy::ops_office::op_trade_ship_armament_with_office(m, host, crate::c08_flat::OP)),
            0x15 | 0x16 | 0x17 | 0x18 => self.c08_convoy_operation(op),
            0x2f => self.c08_cancel_ship_order(op),
            0x4e => self.c08_hire_pirate_captain(op),
            0x47 => self.c08_treasury_transfer(op),
            0x5b => self.c08_flat(op, |m, host| p3_core::economy::ops_office::set_office_autotrade_ware_setting(m, host, crate::c08_flat::OP)),
            0x49 => self.c08_council_vote(op),
            0x04 | 0x05 | 0x06 | 0x10 | 0x11 | 0x53 | 0x54 => self.c08_tavern_ship_operation(op),
            0x82 => self.c08_mission_pirate_ship(op),
            _ => unreachable!(),
        })
    }

    pub(crate) fn c08_merchant_views_synced(&self) -> Result<usize, String> {
        let count = usize::from(self.economy.world.merchant_count);
        if self.rep_merchants.len() < count || self.economy.merchants.len() < count || self.economy.world.merchants.len() < count {
            return Err("vistas de comerciantes truncadas".into());
        }
        Ok(count)
    }

    /// Publica comerciantes modificados: copia a la vista de economía los bytes que cambiaron y
    /// actualiza dinero e ingresos de la cuenta.
    pub(crate) fn c08_publish_merchants(&mut self, merchants: Vec<RepMerchant>) {
        for (i, merchant) in merchants.into_iter().enumerate() {
            if merchant.bytes == self.rep_merchants[i].bytes {
                continue;
            }
            let economic = &mut self.economy.merchants[i].bytes;
            for (j, (&new, &old)) in merchant.bytes.iter().zip(self.rep_merchants[i].bytes.iter()).enumerate() {
                if new != old && j < economic.len() {
                    economic[j] = new;
                }
            }
            self.economy.world.merchants[i].gold = merchant.i32(0);
            self.economy.world.merchants[i].income = merchant.i32(0x4c8);
            self.rep_merchants[i] = merchant;
        }
    }

    /// Publica el planificador y amplía las variables de misión si creció.
    fn c08_publish_scheduler(&mut self, scheduler: p3_core::scheduler::Scheduler) {
        self.host.mission_variables.resize(usize::from(scheduler.capacity), None);
        self.host.council_scheduler = scheduler;
    }

    /// Ejecuta un puerto plano de `p3_core` sobre el estado vivo y lo publica si termina sin error.
    fn c08_flat(&mut self, op: Operation, port: impl FnOnce(&mut dyn p3_core::flatmem::Mem, &mut crate::c08_flat::Calls)) -> Result<(), String> {
        self.c08_merchant_views_synced()?;
        let mut flat = self.flat_seed(op.0)?;
        crate::c08_flat::guarded(|| port(&mut flat.mem, &mut flat.calls))?;
        self.flat_publish(flat)
    }

    /// 0x52: órdenes de la taberna (`0x0053C2C0`). Las órdenes 3, 8 y 10 (tipos 2, 7 y 9) usan la tabla de
    /// `0x006DD7A0`, que el puente no siembra: se detienen sin cambios.
    fn c08_tavern_order(&mut self, op: Operation) -> Result<(), String> {
        use crate::c08_flat::{guarded, SharedMem, TavernTasks};
        self.c08_merchant_views_synced()?;
        let kind = op.0[4].wrapping_sub(1);
        if (op.0[3] as usize) < self.economy.world.towns.len() && matches!(kind, 2 | 7 | 9) {
            return Err(format!("orden de taberna {} con barcos de la tabla 0x6DD7A0 no conectada", op.0[4]));
        }
        let mut flat = self.flat_seed(op.0)?;
        let shared = SharedMem(std::rc::Rc::new(std::cell::RefCell::new(std::mem::take(&mut flat.mem))));
        let result = {
            let mut host = TavernTasks { calls: &mut flat.calls, mem: shared.clone() };
            let mut mem = shared.clone();
            guarded(|| p3_core::economy::tavern_interaction::run(&mut mem, &mut host, crate::c08_flat::OP))
        };
        flat.mem = std::rc::Rc::try_unwrap(shared.0).map_err(|_| "memoria del puente aún compartida")?.into_inner();
        result?;
        self.flat_publish(flat)
    }

    /// 0x6E–0x71: leer, borrar, preferencias de cartas y reparación/purga del buzón.
    fn c08_letter_operation(&mut self, op: Operation) -> Result<(), String> {
        use p3_state::letter_ops;
        let count = self.c08_merchant_views_synced()?;
        let mut pool = self.host.message_pool.clone();
        let mut merchants = self.rep_merchants[..count].to_vec();
        let merchant_count = self.economy.world.merchant_count;
        let mut towns = None;
        match op.0[0] {
            0x55 => {
                let date = self.idle.clock.calendar;
                letter_ops::answer_with_payment(&mut pool, &mut merchants, merchant_count, op.0[1], op.0[2], op.0[3] as i32, (op.0[4] >> 5) as u16, (date.day, date.month, date.year))?;
                // Caso 0x55 del despachador: libera la taberna de la ciudad `+0x10 & 0x1F` (`Town+0x82C`).
                let city = (op.0[4] & 0x1f) as usize;
                if city < self.economy.world.towns.len() {
                    let mut all = self.economy.world.towns.clone();
                    all[city].set_u8(0x82c, 0xff);
                    towns = Some(all);
                }
            }
            0x6e => {
                letter_ops::mark_read(&mut pool, &merchants, merchant_count, op.0[1] as i32, op.0[2])?;
            }
            0x6f => {
                letter_ops::delete(&mut pool, &mut merchants, merchant_count, op.0[1] as i32, op.0[2])?;
            }
            0x70 => {
                // `+4` palabra: comerciante; `+6..+0xE`: preferencias `+0x38..+0x3F`; `+0xE`: byte `+0x26`.
                let bytes = op.0.iter().flat_map(|w| w.to_le_bytes()).collect::<Vec<_>>();
                let merchant = usize::from(u16::from_le_bytes([bytes[4], bytes[5]]));
                if merchant < count {
                    merchants[merchant].bytes[0x38..0x40].copy_from_slice(&bytes[6..0xe]);
                    merchants[merchant].bytes[0x26] = bytes[0xe];
                }
            }
            _ => {
                if op.0[2] != 0 {
                    letter_ops::repair_chains(&mut pool, &mut merchants, merchant_count)?;
                }
                let merchant = op.0[1] as i32;
                if merchant < 0 {
                    return Err("purga del buzón con comerciante negativo".into());
                }
                if (merchant as usize) < count {
                    p3_state::messages::purge_expired(&mut pool, &mut merchants[merchant as usize], self.idle.clock.tick)?;
                }
            }
        }
        self.host.message_pool = pool;
        self.c08_publish_merchants(merchants);
        if let Some(towns) = towns {
            self.economy.world.towns = towns;
        }
        Ok(())
    }

    /// 0x76: aceptar la oferta de un guion en la taberna (`0x0053EDF0`): retira la carta 0x71 y asigna el comerciante
    /// a las variables de la tarea 0x1B.
    fn c08_script_offer(&mut self, op: Operation) -> Result<(), String> {
        self.c08_merchant_views_synced()?;
        let mut flat = self.flat_seed(op.0)?;
        let mut error = None;
        let mut released = Vec::new();
        crate::c08_flat::guarded(|| {
            let mut host = crate::c08_flat::OffersHost { mem_error: &mut error, released: Vec::new(), history: None };
            p3_core::economy::tavern_offers::accept_script_offer(&mut flat.mem, &mut host, crate::c08_flat::OP);
            released = host.released;
        })?;
        for entry in released {
            crate::c08_flat::reset_letter(&mut flat.mem, entry);
        }
        if let Some(e) = error {
            return Err(e);
        }
        self.flat_publish(flat)
    }

    /// 0x4C: comprar las noticias de una región en la taberna (`0x0053B4D0`): cobra, marca como conocidas las
    /// ciudades de la región en el historial (`0x00535400`) y libera la taberna de la ciudad `+8`.
    fn c08_region_news(&mut self, op: Operation) -> Result<(), String> {
        self.c08_merchant_views_synced()?;
        let n = self.economy.world.towns.len();
        let world = self.host.founded_world.as_ref().ok_or("matriz de ciudades ausente")?;
        let distances = world.matrix.get(..n * n).ok_or("matriz de ciudades truncada")?.to_vec();
        let history = self.host.town_history.clone().ok_or("historial de ciudades ausente")?;
        let mut flat = self.flat_seed(op.0)?;
        let mut error = None;
        let mut result = None;
        crate::c08_flat::guarded(|| {
            let mut host = crate::c08_flat::OffersHost { mem_error: &mut error, released: Vec::new(), history: Some((history, &distances, n)) };
            p3_core::economy::tavern_offers::buy_region_news(&mut flat.mem, &mut host, crate::c08_flat::OP);
            result = host.history.map(|(h, _, _)| h);
        })?;
        if let Some(e) = error {
            return Err(e);
        }
        let old = self.host.town_history.clone();
        self.flat_publish(flat)?;
        if let (Some(next), Some(old)) = (result, old) {
            if next != old {
                self.host.town_history_versions.insert(crate::save_version(&old));
            }
            self.host.town_history = Some(next);
        }
        Ok(())
    }

    /// 0x29: concesión de un préstamo (`0x00539DC0`).
    fn c08_grant_loan(&mut self, op: Operation) -> Result<(), String> {
        let count = self.c08_merchant_views_synced()?;
        let mut bytes: [u8; 0x14] = op.0.iter().flat_map(|w| w.to_le_bytes()).collect::<Vec<_>>().try_into().unwrap();
        let mut towns = self.economy.world.towns.clone();
        let mut merchants = self.rep_merchants[..count].to_vec();
        let mut scheduler = self.host.council_scheduler.clone();
        if scheduler.free_head >= scheduler.capacity && scheduler.capacity > u16::MAX - 128 {
            return Err("crecimiento del planificador de préstamos fuera de rango".into());
        }
        p3_core::economy::loan_grant::grant(&mut bytes, &mut towns, &mut merchants, &mut scheduler, self.idle.clock.tick);
        self.economy.world.towns = towns;
        self.c08_publish_merchants(merchants);
        self.c08_publish_scheduler(scheduler);
        Ok(())
    }

    /// 0x47: ingresar en las arcas de la ciudad o retirar de ellas (`0x0053AF30`).
    fn c08_treasury_transfer(&mut self, op: Operation) -> Result<(), String> {
        let count = self.c08_merchant_views_synced()?;
        let mut towns = self.economy.world.towns.clone();
        let mut merchants = self.rep_merchants[..count].to_vec();
        p3_core::economy::church::treasury_transfer(op.0, &mut towns, &mut merchants, count as u16, self.idle.clock.tick);
        self.economy.world.towns = towns;
        self.c08_publish_merchants(merchants);
        Ok(())
    }

    /// Entorno de reputación del original: ciudades vivas y `GameWorld+0x8C`.
    fn c08_rep_env<'a>(towns: &'a [p3_core::town::Town], factor: u8) -> p3_core::ai::reputation::RepEnv<'a> {
        p3_core::ai::reputation::RepEnv { town_count: towns.len() as u32, towns, renown: [0; 3], byte_52c: factor }
    }

    /// 0x31: donación a la caja de obras de la iglesia (`0x004FE420` sobre `Town+0x794`).
    fn c08_church_donation(&mut self, op: Operation) -> Result<(), String> {
        use p3_core::buildings::donation::{DonationEnv, DonationHost};
        struct Host<'a> {
            merchant: &'a mut RepMerchant,
            towns: &'a [p3_core::town::Town],
            factor: u8,
        }
        impl DonationHost for Host<'_> {
            fn get_merchant(&mut self, _merchant: u32) {}
            fn add_church_donation_reputation(&mut self, town: u8, amount: i32) {
                let env = Sim::c08_rep_env(self.towns, self.factor);
                self.merchant.add_social_reputation_church_donation(&env, u32::from(town), amount);
            }
        }
        let count = self.c08_merchant_views_synced()?;
        let city = op.0[2] as usize;
        if city >= self.economy.world.towns.len() {
            return Ok(());
        }
        let owner = op.0[1] as usize;
        if owner >= count {
            return Err("comerciante donante ausente".into());
        }
        let mut towns = self.economy.world.towns.clone();
        let mut merchants = self.rep_merchants[..count].to_vec();
        let mut gold = merchants[owner].i32(0);
        let mut donations = merchants[owner].i32(0x4bc);
        let mut merchant = merchants[owner].clone();
        let mut town = towns[city].clone();
        {
            let mut host = Host { merchant: &mut merchant, towns: &self.economy.world.towns, factor: self.host.church_cost_factor };
            let mut env = DonationEnv { setting: self.host.church_cost_factor, merchant_gold: &mut gold, merchant_donations: &mut donations };
            p3_core::buildings::Site::donate_to_building_fund(&mut town, op.0[1], op.0[2] as u8, op.0[3] as i32, &mut env, &mut host);
        }
        merchant.set_i32(0, gold);
        merchant.set_i32(0x4bc, donations);
        merchants[owner] = merchant;
        towns[city] = town;
        self.economy.world.towns = towns;
        self.c08_publish_merchants(merchants);
        Ok(())
    }

    /// 0x32: donación a la caja municipal (`0x004FE2D0`).
    fn c08_town_donation(&mut self, op: Operation) -> Result<(), String> {
        let count = self.c08_merchant_views_synced()?;
        let city = op.0[2] as usize;
        if city >= self.economy.world.towns.len() {
            return Ok(());
        }
        let owner = op.0[1] as usize;
        if owner >= count {
            return Err("comerciante donante ausente".into());
        }
        let mut merchants = self.rep_merchants[..count].to_vec();
        let mut town = self.economy.world.towns[city].clone();
        let env = Self::c08_rep_env(&self.economy.world.towns, self.host.church_cost_factor);
        p3_core::economy::c08_reputation::donate_to_town_fund(&mut town, &mut merchants[owner], &env, op.0[2], op.0[3] as i32, self.host.church_cost_factor);
        self.economy.world.towns[city] = town;
        self.c08_publish_merchants(merchants);
        Ok(())
    }

    /// `town_calculate_expected_bribe` (0x00529E20) para el comerciante `merchant` y el escaño `seat` de la ciudad `city`: según su reputación y un
    /// sorteo `rand() % 11` del CRT de la simulación (solo se sortea si los índices valen, como el original).
    pub fn bath_expected_bribe(&mut self, city: usize, merchant: u32, seat: u32) -> Result<u32, String> {
        if city >= self.economy.world.towns.len() {
            return Err("ciudad inexistente".into());
        }
        let valid = merchant < u32::from(self.economy.world.merchant_count) && seat < 4;
        let draw = if valid { Some(self.host.investigation_rng.as_mut().ok_or("estado CRT ausente")?.rand()) } else { None };
        self.c08_merchant_views_synced()?;
        let mut flat = self.flat_seed([0; 5])?;
        flat.calls.rand_queue.extend(draw);
        // tabla de saltos del ejecutable (`0x00529EF4`) que el puerto lee para elegir el tramo de reputación
        let jump: Vec<u8> = [0x529e6eu32, 0x529e75, 0x529e7c, 0x529e83, 0x529e8a, 0x529e91, 0x529e98, 0x529e9f].iter().flat_map(|v| v.to_le_bytes()).collect();
        flat.mem.seed(0x529ef4, &jump);
        let town = crate::c08_flat::TOWNS + crate::c08_flat::TOWN_SIZE * city as u32;
        let mut out = 0;
        {
            let (calls, m) = (&mut flat.calls, &mut flat.mem);
            crate::c08_flat::guarded(|| out = p3_core::economy::small_fns::town_calculate_expected_bribe(m, calls, town, &[merchant, seat]))?;
        }
        if let Some(e) = flat.calls.error.take() {
            return Err(e);
        }
        Ok(out)
    }

    /// 0x42/0x43: soborno en los baños, con éxito o fracaso.
    fn c08_bribe(&mut self, op: Operation, success: bool) -> Result<(), String> {
        let count = self.c08_merchant_views_synced()?;
        let mut towns = self.economy.world.towns.clone();
        let mut merchants = self.rep_merchants[..count].to_vec();
        if success {
            p3_core::economy::c08_reputation::bribe_success(&mut towns, &mut merchants, op.0[1], op.0[2], op.0[3] as i32, op.0[4])?;
        } else {
            p3_core::economy::c08_reputation::bribe_failure(&mut towns, &mut merchants, op.0[1], op.0[2], op.0[4], self.economy.world.merchant_count);
        }
        self.economy.world.towns = towns;
        self.c08_publish_merchants(merchants);
        Ok(())
    }

    /// 0x3F: voto de un concejal en la elección del Ältermann o en la Dieta de la Hansa del día.
    fn c08_cast_ballot(&mut self, op: Operation) -> Result<(), String> {
        let count = self.c08_merchant_views_synced()?;
        let mut scheduler = self.host.council_scheduler.clone();
        p3_core::economy::c08_votes::cast_ballot(
            &mut scheduler,
            &self.economy.world.towns,
            &self.rep_merchants[..count],
            self.idle.clock.tick,
            op.0[1],
            op.0[2],
        );
        self.c08_publish_scheduler(scheduler);
        Ok(())
    }

    /// 0x45: declaración o retirada de candidatura a la alcaldía.
    fn c08_candidacy(&mut self, op: Operation) -> Result<(), String> {
        let count = self.c08_merchant_views_synced()?;
        let mut merchants = self.rep_merchants[..count].to_vec();
        p3_core::economy::c08_votes::set_candidacy(&mut merchants, op.0[1], op.0[2]);
        self.c08_publish_merchants(merchants);
        Ok(())
    }

    /// 0x49: voto en una reunión del concejo (`0x0053B0D0`).
    fn c08_council_vote(&mut self, op: Operation) -> Result<(), String> {
        let count = self.c08_merchant_views_synced()?;
        // `+8` se lee como palabra para buscar la reunión y como doble palabra para el rango y la ciudad.
        let city = op.0[2];
        if city > 0xff {
            return Err("ciudad de voto del concejo fuera de dominio".into());
        }
        let merchant = op.0[1] as usize;
        if merchant >= count {
            return Ok(());
        }
        let Some(town) = self.economy.world.towns.get(city as usize) else {
            return Err("ciudad de voto del concejo ausente".into());
        };
        let mut scheduler = self.host.council_scheduler.clone();
        p3_core::economy::council::cast_vote(&mut scheduler, town, &self.rep_merchants[..count], self.idle.clock.tick, merchant, city as u8, op.0[3]);
        self.c08_publish_scheduler(scheduler);
        Ok(())
    }
}
