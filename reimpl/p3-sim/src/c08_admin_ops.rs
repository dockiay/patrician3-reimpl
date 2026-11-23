//! C08: contratar o despedir al administrador de una oficina (`0x5E`, `0x0053D990`, `hire_or_fire_office_administrator`).
//!
//! Contratar da de alta un capitán nuevo de la tabla viva (`0x005097C0` + `Captain::init`, `0x004FDF50`, con los bancos de nombres de
//! `Namen_eng.ini` y la semilla `ciudad[día del año & 15] + 0x2E0` como el resto de altas de capitán), le pone el jornal de
//! `0x004FE160` y lo enlaza en `Office+0x2F2`; despedir lo devuelve a la lista libre y quita los dos bits bajos de `Office+0x2D6`.
//! Se publican la oficina, la tabla de capitanes (fábrica, navegación y piratas), el jornal de la oficina y la experiencia del
//! administrador. Si la lista libre de capitanes está agotada el pool tendría que crecer (0x40 entradas) y se devuelve un error.
use crate::c08_flat::{guarded, CaptainSeed, CAPTAIN_FREE_HEAD};
use crate::Sim;
use p3_core::flatmem::Mem;
use p3_core::pump::Operation;

const CAPTAINS: u32 = 0x6200_0000;
const OFFICE: usize = 0x44c;

impl Sim {
    pub(crate) fn c08_administrator_operation(&mut self, op: Operation) -> Result<(), String> {
        self.c08_merchant_views_synced()?;
        let resources = self.host.expedition_resources.as_ref().ok_or("nombres de capitán (Namen_eng.ini) no configurados")?;
        let (first_names, last_names) = (resources.first_names, resources.last_names);
        let factory_before = self.host.expedition_factory.clone().ok_or("pool de capitanes ausente")?;
        if let Some(nav) = &self.host.navigation {
            if nav.snapshot.captains != factory_before.captains.concat() {
                return Err("vistas de capitanes desincronizadas".into());
            }
        }
        let date = self.idle.clock.calendar;
        let population = self.economy.world.towns.get(usize::from(date.day_of_year & 15)).map(|t| t.u32(0x2e0)).ok_or("ciudad de semilla de capitán ausente")?;
        let table = factory_before.captains.concat();
        let mut flat = self.flat_seed(op.0)?;
        flat.mem.seed(CAPTAINS, &table);
        flat.mem.seed(0x006d_d7a0, &CAPTAINS.to_le_bytes());
        flat.mem.seed(CAPTAIN_FREE_HEAD, &factory_before.free_captain.to_le_bytes());
        flat.calls.captain_seed = Some(CaptainSeed { tick: self.idle.clock.tick, day_of_year: date.day_of_year, day: date.day, population, first_names, last_names });
        {
            let (m, calls) = (&mut flat.mem, &mut flat.calls);
            guarded(|| p3_core::economy::ops_office::hire_or_fire_office_administrator(m, calls, crate::c08_flat::OP))?;
        }
        if let Some(e) = flat.calls.error.take() {
            return Err(e);
        }
        let after_table = flat.mem.read(CAPTAINS, table.len());
        let free = flat.mem.r16(CAPTAIN_FREE_HEAD);
        let images_before = self.host.scheduled_office_images.clone().ok_or("oficinas vivas ausentes")?;
        self.flat_publish(flat)?;
        let images = self.host.scheduled_office_images.as_ref().ok_or("oficinas vivas ausentes")?;
        let changed: Vec<usize> = (0..images.len()).filter(|&i| images_before.get(i) != Some(&images[i])).collect();
        // Capitanes: fábrica, navegación (con su base, porque es un alta) y vista pirata.
        let mut factory_after = factory_before.clone();
        for (c, bytes) in factory_after.captains.iter_mut().zip(after_table.chunks_exact(16)) {
            c.copy_from_slice(bytes);
        }
        factory_after.free_captain = free;
        if factory_after != factory_before {
            if let Some(p) = self.host.hanse_pirates.as_mut() {
                if p.captains.len() != factory_before.captains.len() {
                    return Err("vistas piratas de capitanes truncadas".into());
                }
                for (i, (old, new)) in factory_before.captains.iter().zip(&factory_after.captains).enumerate() {
                    for j in 0..16 {
                        if old[j] != new[j] {
                            p.captains[i][j] = new[j];
                        }
                    }
                }
            }
            if let Some(nav) = self.host.navigation.as_mut() {
                crate::factory_nav::publish_factory(nav, &factory_before, &factory_after)?;
            }
            self.host.expedition_factory = Some(factory_after.clone());
        }
        // Oficina: jornal, experiencia y los dos campos del administrador en la vista persistida.
        for i in changed {
            let image = self.host.scheduled_office_images.as_ref().ok_or("oficinas vivas ausentes")?[i].clone();
            if image.len() < OFFICE {
                return Err("imagen de oficina truncada".into());
            }
            let id = usize::from(u16::from_le_bytes([image[0x2f2], image[0x2f3]]));
            let captain = factory_after.captains.get(id);
            self.wages[i] = captain.map(|c| u16::from_le_bytes([c[0xc], c[0xd]]));
            let experience = captain.map(|c| c[0xa]);
            let mut memory = p3_save::office::decode_office(&self.persisted[i].to_save_v85());
            memory[0x2f2..0x2f4].copy_from_slice(&image[0x2f2..0x2f4]);
            memory[0x2d6] = image[0x2d6];
            let mut persisted = p3_state::office::PersistedOffice::from_save_v85(&p3_save::office::encode_office(&memory));
            persisted.stock.administrator_experience = experience;
            self.economy.offices[i] = persisted.stock.clone();
            self.persisted[i] = persisted;
        }
        Ok(())
    }
}
