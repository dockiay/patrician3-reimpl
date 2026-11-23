//! C06/C10: financiación de los comerciantes IA dentro de su turno (tarea 0x04).
//!
//! `raise_cash_or_go_bankrupt` (`0x004F4E50`) encola un préstamo de la ciudad (0x29) o la quiebra (0x65) y puede
//! poner en obra la venta de un edificio con `merchant_ai_queue_building_work` (`0x004F5A20`, operación 0x28). Las
//! consultas al planificador (`0x004DAFE0`, `0x004DB220`) y el valor de venta (`0x004DAF10`) se resuelven sobre el
//! estado vivo; el azar es el CRT compartido de la sesión.
//!
//! `0x004F5A20` lee una ranura de pila sin inicializar en su primera comparación. Se evalúa con varios valores de esa
//! ranura y solo se acepta el resultado si todos coinciden; si no, el turno se detiene con un error explícito.
use crate::{CouncilExecution, RepMerchant};
use p3_core::ai::{cash, housing::HousingRecord, work};
use p3_core::pump::Operation;
use p3_core::rng::CrtRand;
use p3_core::scheduler::Scheduler;

/// Valores de prueba de la ranura sin inicializar de `0x004F5A20`.
const RESIDUES: [u32; 6] = [0, 1, 2, 7, 0x8000_0000, 0xffff_ffff];

fn u16_at(b: &[u8], at: usize) -> u16 {
    u16::from_le_bytes([b[at], b[at + 1]])
}

/// Tareas 0x36 (subasta de edificios) en el orden de la lista: (ciudad, tres lotes).
fn building_auctions(s: &Scheduler) -> Result<Vec<(u8, [u16; 3])>, String> {
    let mut out = Vec::new();
    let mut at = s.head;
    let mut steps = 0usize;
    while at < s.capacity {
        let r = s.records.get(usize::from(at)).ok_or("planificador truncado")?;
        if r.kind == 0x36 {
            let lots = std::array::from_fn(|k| u16::from_le_bytes([r.u8_at(8 + 2 * k), r.u8_at(9 + 2 * k)]));
            out.push((r.u8_at(0x15), lots));
        }
        at = r.next;
        steps += 1;
        if steps > s.records.len() {
            return Err("planificador cíclico".into());
        }
    }
    Ok(out)
}

struct WorkBridge<'b, 'a> {
    exec: &'b CouncilExecution<'a>,
    auctions: &'b [(u8, [u16; 3])],
    operations: Vec<[u32; 5]>,
    error: Option<String>,
}

impl work::WorkHost for WorkBridge<'_, '_> {
    /// `0x004DAFE0`: sin subasta de ese edificio en la ciudad.
    fn is_unclaimed(&mut self, town: u8, record: u16) -> bool {
        !self.auctions.iter().any(|(city, lots)| *city == town && lots.iter().any(|&l| l & 0x7fff == record))
    }
    /// `town_building_sale_value` (`0x004DAF10`).
    fn building_cost(&mut self, town: u8, record: u16, cell_marked: bool) -> u32 {
        let i = usize::from(town);
        let (Some(t), Some(site)) = (self.exec.world.towns.get(i), self.exec.sites.get(i)) else {
            return u32::MAX;
        };
        if record >= t.u16(0x776) {
            return u32::MAX;
        }
        let flag = match self.exec.world.towns.get(usize::from(t.u8(0x2c0))) {
            Some(home) => home.i32(0x2cc),
            None => {
                self.error.get_or_insert_with(|| "ciudad de referencia del valor de venta ausente".into());
                return 0;
            }
        };
        let siege = if cell_marked {
            false
        } else {
            match self.exec.auction_siege_active(i) {
                Ok(v) => v,
                Err(e) => {
                    self.error.get_or_insert(e);
                    false
                }
            }
        };
        let base = site.building_base_value(t, u32::from(record), flag);
        p3_core::economy::auction::building_sale_price(base, cell_marked, siege) as u32
    }
    fn enqueue_operation(&mut self, record: &[u32; 5]) {
        self.operations.push(*record);
    }
}

struct CashBridge<'b, 'a> {
    exec: &'b CouncilExecution<'a>,
    env: work::WorkEnv<'b>,
    auctions: &'b [(u8, [u16; 3])],
    rng: CrtRand,
    used_rng: bool,
    operations: Vec<[u32; 5]>,
    error: Option<String>,
}

impl cash::CashHost for CashBridge<'_, '_> {
    fn queue_building_work(&mut self, task: u32) -> bool {
        let mut outcome: Option<(bool, Vec<[u32; 5]>)> = None;
        let fixed = self.exec.ai_work_residue.map(|r| [r]);
        let residues: &[u32] = match &fixed {
            Some(one) => one,
            None => &RESIDUES,
        };
        for &residue in residues {
            let env = work::WorkEnv { towns: self.env.towns, cells: self.env.cells, tasks: self.env.tasks, offices: self.env.offices, office_head: self.env.office_head, guild_mask: self.env.guild_mask, residue };
            let mut bridge = WorkBridge { exec: self.exec, auctions: self.auctions, operations: Vec::new(), error: None };
            let queued = work::queue_building_work(task, &env, &mut bridge);
            if let Some(e) = bridge.error {
                self.error.get_or_insert(e);
                return false;
            }
            match &outcome {
                None => outcome = Some((queued, bridge.operations)),
                Some((q, ops)) if *q == queued && *ops == bridge.operations => {}
                Some(_) => {
                    self.error.get_or_insert_with(|| "obra de IA dependiente de una ranura de pila sin inicializar (0x004F5A20)".into());
                    return false;
                }
            }
        }
        let (queued, ops) = outcome.unwrap_or((false, Vec::new()));
        self.operations.extend(ops);
        queued
    }
    /// `scheduled_tasks_merchant_has_building_in_auction` (`0x004DB220`): algún lote de una subasta es del comerciante.
    fn has_building_in_auction(&mut self, task: u8) -> bool {
        self.auctions.iter().any(|(city, lots)| {
            let i = usize::from(*city);
            let (Some(t), Some(site)) = (self.exec.world.towns.get(i), self.exec.sites.get(i)) else { return false };
            lots.iter().any(|&lot| {
                let id = usize::from(lot & 0x7fff);
                id < usize::from(t.u16(0x776)) && site.records.get(id * 8 + 2) == Some(&task)
            })
        })
    }
    fn rand(&mut self) -> i32 {
        self.used_rng = true;
        self.rng.rand()
    }
    fn enqueue_operation(&mut self, record: &[u32; 5]) {
        self.operations.push(*record);
    }
}

impl CouncilExecution<'_> {
    /// Financiación de la IA (`0x004F4E50`) con las oficinas ya actualizadas por el turno (`images`). Devuelve las
    /// operaciones en el orden original y el estado CRT si se consumió.
    pub(crate) fn ai_raise_cash(&self, s: &Scheduler, id: u16, merchant: &RepMerchant, images: &[Vec<u8>]) -> Result<(Vec<Operation>, Option<CrtRand>), String> {
        let towns = self.world.towns.len();
        if towns > 40 || self.sites.len() < towns || images.iter().any(|o| o.len() < 0x37c) || merchant.bytes.len() < 0x650 {
            return Err("datos de financiación IA incompletos".into());
        }
        if self.employment_businesses.len() != self.world.merchant_facilities.len() {
            return Err("negocios de financiación IA incompletos".into());
        }
        let work_towns = self
            .world
            .towns
            .iter()
            .zip(self.sites.iter())
            .map(|(t, site)| {
                let count = usize::from(t.u16(0x776));
                if site.records.len() < count * 8 {
                    return Err("registros de edificios truncados".to_string());
                }
                let records = site.records[..count * 8].chunks_exact(8).map(|r| HousingRecord { x: r[0], y: r[1], kind: r[3], next: u16_at(r, 4) }).collect();
                Ok(work::WorkTown { records, stride: t.u32(0x7ac), cells: site.cells.clone() })
            })
            .collect::<Result<Vec<_>, _>>()?;
        let tasks = self
            .world
            .merchant_facilities
            .iter()
            .zip(self.employment_businesses.iter())
            .map(|(f, image)| work::WorkTask { a: f.employees, kind: image[6], next: f.next, b: u16_at(image, 12) })
            .collect::<Vec<_>>();
        let offices = images
            .iter()
            .map(|o| work::WorkOffice { town: o[0x2c6], next: u16_at(o, 0x2c8), task_head: u16_at(o, 0x2cc), list1: u16_at(o, 0x2ce), words: std::array::from_fn(|k| u16_at(o, 0x2d0 + 2 * k)) })
            .collect::<Vec<_>>();
        let auctions = building_auctions(s)?;
        let rng = self.investigation_rng.clone().ok_or("estado CRT de financiación IA ausente")?;
        let env = work::WorkEnv {
            towns: &work_towns,
            cells: &[],
            tasks: &tasks,
            offices: &offices,
            office_head: merchant.u16(0xc),
            guild_mask: merchant.i32(0x468) as u32,
            residue: 0,
        };
        let mut bridge = CashBridge { exec: self, env, auctions: &auctions, rng, used_rng: false, operations: Vec::new(), error: None };
        let mut gold = merchant.i32(0);
        let cash_env = cash::CashEnv { flag_2a: self.world.world_8a != 0, flag_31c: self.world.network_client != 0 };
        cash::raise_cash_or_go_bankrupt(&mut gold, merchant.bytes[0x19], u32::from(id), &cash_env, &mut bridge);
        if let Some(e) = bridge.error {
            return Err(e);
        }
        let rng = bridge.used_rng.then_some(bridge.rng);
        Ok((bridge.operations.into_iter().map(Operation).collect(), rng))
    }
}
