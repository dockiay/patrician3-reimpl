//! Despacho de turnos de comerciante (`st_merchant_ai_turn`, 0x004DFC94).
//!
//! Las acciones del turno de IA y el factor de reputación previo permanecen a cargo del anfitrión.
use super::turn::{run_ai_turn, TurnHost, TurnMerchant, TurnTask};
use crate::scheduler::TaskRecord;

pub trait ScheduledTurnHost {
    fn update_base_rep_factor(&mut self, merchant: u16);
    fn run_ai_turn(&mut self, merchant: u16, task_index: u16, day: u16, task: &mut TaskRecord) -> u32;
}

/// Resultado preparado antes de aplicar las acciones a la sesión.
pub struct TurnPreparation {
    pub task: TaskRecord,
    pub delay: u32,
    pub stock: bool,
    pub grant: bool,
    pub housing: bool,
    pub guild: bool,
    pub arm: bool,
    pub home: bool,
    /// `raise_cash_or_go_bankrupt` (0x004F4E50) fuera de red y sin el modo de subvención: el anfitrión lo ejecuta en
    /// su posición (tras existencias y flota, antes de vivienda y gremio).
    pub cash: bool,
}

/// Prepara un turno C06 y devuelve las decisiones que necesita ejecutar.
pub fn prepare_with_stock(
    task: TaskRecord,
    merchant: &TurnMerchant,
    network: u32,
    special_mode: bool,
    no_offices: bool,
) -> Result<TurnPreparation, &'static str> {
    struct Probe {
        stock: bool,
        other: Option<&'static str>,
        grant: bool,
        cash: bool,
        housing: bool,
        guild: bool,
        arm: bool,
        home: bool,
        special: bool,
        network: bool,
        no_offices: bool,
    }
    impl Probe {
        fn pending(&mut self, action: &'static str) {
            if self.other.is_none() {
                self.other = Some(action);
            }
        }
    }
    impl TurnHost for Probe {
        fn update_stock_targets(&mut self) {
            self.stock = true;
        }
        fn mark_ship_for_arming(&mut self, _: u16) {
            self.arm = true;
        }
        fn assign_ship_home_towns(&mut self) {
            self.home = true;
        }
        fn raise_cash(&mut self, _: u8) {
            if self.special {
                self.grant = true
            } else if !self.network {
                self.cash = true
            }
        }
        fn choose_housing(&mut self, _: u16) {
            if !self.special && !self.no_offices {
                self.housing = true
            }
        }
        fn try_join_guild(&mut self, _: u16) {
            if !self.special && !self.no_offices {
                self.guild = true
            }
        }
    }
    let mut counters = TurnTask {
        id: u16::from_le_bytes([task.u8_at(8), task.u8_at(9)]),
        stock_counter: task.u8_at(0x10),
        fleet_counter: task.u8_at(0x11),
        housing_counter: task.u8_at(0x0e),
    };
    let mut probe = Probe {
        stock: false,
        other: None,
        grant: false,
        cash: false,
        housing: false,
        guild: false,
        arm: false,
        home: false,
        special: special_mode,
        network: network != 0,
        no_offices,
    };
    let delay = run_ai_turn(merchant, &mut counters, network, &mut probe);
    if let Some(pending) = probe.other {
        return Err(pending);
    }
    let mut prepared = task;
    prepared.set_u8_at(0x10, counters.stock_counter);
    prepared.set_u8_at(0x11, counters.fleet_counter);
    prepared.set_u8_at(0x0e, counters.housing_counter);
    Ok(TurnPreparation {
        task: prepared,
        delay,
        stock: probe.stock,
        grant: probe.grant,
        housing: probe.housing,
        guild: probe.guild,
        arm: probe.arm,
        home: probe.home,
        cash: probe.cash,
    })
}

/// El comerciante inexistente elimina la tarea. Las fechas se calculan con la aritmética x86.
pub fn dispatch(
    task: &mut TaskRecord,
    task_index: u16,
    merchant_count: u16,
    flags: Option<u16>,
    tick: u32,
    day: u16,
    host: &mut impl ScheduledTurnHost,
) -> Result<bool, &'static str> {
    let merchant = u16::from_le_bytes([task.u8_at(8), task.u8_at(9)]);
    if merchant >= merchant_count {
        return Ok(false);
    }
    let flags = flags.ok_or("comerciante ausente")?;
    let delay = if flags == 0 {
        0x1f00
    } else {
        if flags as u8 & 8 != 0 {
            host.update_base_rep_factor(merchant);
        }
        host.run_ai_turn(merchant, task_index, day, task)
    };
    task.due = task.due.wrapping_add(delay);
    if task.due <= tick {
        task.due = tick.wrapping_add(1);
    }
    Ok(true)
}
