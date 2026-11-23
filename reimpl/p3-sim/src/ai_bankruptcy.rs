//! C06/C10: quiebra de un comerciante IA (operación 0x65, `merchant_bankruptcy_reset`, `0x004F5D80`).
//!
//! El reinicio del comerciante es el puerto `ai::bankrupt`; aquí se conectan sus efectos externos sobre el estado
//! vivo: misiones de préstamo (tipo 1) del planificador, alcaldía de su ciudad (`Town+0x6F1`), purga de su buzón
//! (`0x004D6780`) y su historial de estadísticas (`0x0054E1F0` lo quita y `0x0054E170` lo vuelve a crear con una
//! muestra nueva). Las cuentas de nombres son las de `Namen_eng.ini` y `0x006DD888` es la palabra `+0xE8` del gestor
//! de barcos (cabeza de los atracados).
use crate::{RepMerchant, Sim};
use p3_core::ai::bankrupt::{BankruptcyEnv, BankruptcyHost, Mission};
use p3_core::ai::misc::NameHost;

#[derive(Default)]
struct Events {
    purge: bool,
    history_removed: bool,
    history_added: bool,
}

impl NameHost for Events {
    fn name_a(&mut self, index: u32) -> u32 {
        index
    }
    fn name_b(&mut self, index: u32) -> u32 {
        index
    }
}

impl BankruptcyHost for Events {
    fn on_event(&mut self, tag: &'static str, _arg: u32) {
        match tag {
            "m1" => self.history_removed = true,
            "purge" => self.purge = true,
            "m2" => self.history_added = true,
            _ => {}
        }
    }
}

impl Sim {
    /// Operación 0x65 con el comerciante `+4`.
    pub(crate) fn c06_bankruptcy(&mut self, op: p3_core::pump::Operation) -> Result<(), String> {
        let count = self.c08_merchant_views_synced()?;
        let index = op.0[1] as usize;
        if index >= count {
            return Err("quiebra de un comerciante fuera de la tabla".into());
        }
        if self.rep_merchants[index].u16(8) == 0 {
            return Ok(());
        }
        let resources = self.host.expedition_resources.as_ref().ok_or("nombres de comerciante (Namen_eng.ini) no configurados")?;
        let (first, last) = (resources.first_names, resources.last_names);
        // `0x006DD888` = cabeza de la lista de barcos atracados (`ShipManager+0xE8`): la del viaje vivo si está
        // reconstruido y, si no, la del guardado (que mantienen las expediciones).
        let dword_888 = u32::from(match &self.host.navigation {
            Some(nav) => {
                let manager = &nav.snapshot.manager;
                if manager.len() < 0xea {
                    return Err("gestor de barcos truncado".into());
                }
                u16::from_le_bytes([manager[0xe8], manager[0xe9]])
            }
            None => self.host.expedition_docked_head,
        });
        let mut scheduler = self.host.council_scheduler.clone();
        let mut missions = scheduler
            .records
            .iter()
            .map(|r| Mission { next: r.next, kind: r.kind, b10: r.u8_at(0x10), b11: r.u8_at(0x11), bc: r.u8_at(0xc) })
            .collect::<Vec<_>>();
        if missions.is_empty() {
            missions.push(Mission::default());
        }
        let mut town_flags = self.economy.world.towns.iter().map(|t| t.u8(0x6f1)).collect::<Vec<_>>();
        let last_names = self.rep_merchants[..count].iter().map(|m| m.bytes[0x1d]).collect::<Vec<_>>();
        let date = self.idle.clock.calendar;
        let env = BankruptcyEnv {
            index: index as u32,
            this_addr: 0,
            date: self.idle.clock.tick,
            word_4a4: u32::from(date.day_of_year),
            name_count: u32::from(first),
            name2_count: u32::from(last),
            dword_888,
            merchant_count: count as u16,
            year: date.year,
            cursor: u32::from(scheduler.head),
            mission_count: scheduler.capacity,
            last_names: &last_names,
        };
        if usize::from(self.rep_merchants[index].bytes[0x19]) >= town_flags.len() {
            return Err("ciudad del comerciante en quiebra fuera de la tabla".into());
        }
        if env.name_count & 0xffff == 0 {
            return Err("cuenta de nombres nula".into());
        }
        let mut merchants = self.rep_merchants[..count].to_vec();
        let mut events = Events::default();
        let mut merchant = merchants[index].clone();
        // Ciudad y misiones ya comprobadas; el único fallo restante es la división por cero del original.
        merchant.try_bankruptcy_reset(&env, &mut missions, &mut town_flags, &mut events)?;
        merchants[index] = merchant;
        for (record, mission) in scheduler.records.iter_mut().zip(&missions) {
            record.set_u8_at(0x10, mission.b10);
            record.set_u8_at(0x11, mission.b11);
            record.set_u8_at(0xc, mission.bc);
        }
        let mut pool = self.host.message_pool.clone();
        if events.purge {
            p3_state::messages::purge_expired(&mut pool, &mut merchants[index], self.idle.clock.tick)?;
        }
        let mut statistics = self.host.statistics.clone();
        if let Some(stats) = statistics.as_mut() {
            if events.history_removed {
                if let Some(at) = stats.items.iter().position(|item| item.len() >= 2 && usize::from(u16::from_le_bytes([item[0], item[1]])) == index) {
                    stats.items.remove(at);
                }
            }
            if events.history_added && !stats.items.iter().any(|item| item.len() >= 2 && usize::from(u16::from_le_bytes([item[0], item[1]])) == index) {
                let sample = self.c06_history_sample(&mut merchants[index])?;
                stats.items.insert(0, p3_core::economy::merchant_statistics::new_history(index as u16, &sample));
            }
        }
        let mut towns = self.economy.world.towns.clone();
        for (town, flag) in towns.iter_mut().zip(&town_flags) {
            town.set_u8(0x6f1, *flag);
        }
        self.economy.world.towns = towns;
        self.host.message_pool = pool;
        self.host.statistics = statistics;
        self.c08_publish_merchants(merchants);
        self.host.council_scheduler = scheduler;
        Ok(())
    }

    /// Muestra inicial del historial (`merchant_stats_record_capture`, `0x0054C520`) con las mismas entradas que el
    /// paso diario de estadísticas.
    fn c06_history_sample(&self, merchant: &mut RepMerchant) -> Result<[u8; 0x6c], String> {
        let n = self.economy.world.towns.len();
        let pool = self.host.office_pool.as_ref().ok_or("pool de oficinas ausente")?;
        if pool.records.len() != self.persisted.len() || pool.records.len() != self.economy.world.offices.len() {
            return Err("oficinas de estadísticas incompatibles".into());
        }
        let mut offices = Vec::with_capacity(pool.records.len());
        for (i, persisted) in self.persisted.iter().enumerate() {
            let record = &pool.records[i];
            if record.len() != 0x44c {
                return Err("oficina truncada".into());
            }
            let mut image = p3_save::office::decode_office(&persisted.to_save_v85());
            image[0x2c4..0x2ca].copy_from_slice(&record[0x2c4..0x2ca]);
            image[0x2ca..0x2cc].copy_from_slice(&self.economy.world.offices[i].next.to_le_bytes());
            offices.push(image.to_vec());
        }
        let captains = if let Some(nav) = &self.host.navigation { nav.snapshot.captains.len() / 16 } else { self.host.expedition_factory.as_ref().ok_or("pool de capitanes ausente")?.captains.len() };
        let captain_count = u16::try_from(captains).map_err(|_| "demasiados capitanes")?;
        let home = usize::from(merchant.bytes[0x19]);
        let local = if home >= n {
            0
        } else {
            let value = f32::from_bits(u32::from_le_bytes(merchant.bytes[0x2fc + 4 * home..0x300 + 4 * home].try_into().unwrap()));
            if value > 32758.0 { 0x7fff } else { p3_core::numeric::ftol_i32(value as f64) as u16 }
        };
        let total = merchant.calc_total_reputation_score(n as i32, &self.economy.world.towns) as i16;
        p3_core::economy::merchant_statistics::capture(&mut merchant.bytes, &offices, &self.host.employment_businesses, captain_count, local as i16, total).map_err(str::to_string)
    }
}
