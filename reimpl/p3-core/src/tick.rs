//! Avance del tiempo: `advance_time(objetivo, ficha)` (0x00530E80).
//!
//! Única forma de que avance el tick durante la partida. Cada tick ejecuta una fase según
//! `(tick & 0xFF) & 7` (tabla 0x005310A4) y después, en orden fijo, barcos, planificador, escena del
//! puerto, batallas navales y escena de cada asedio.

use crate::calendar::{is_week_start, Calendar};

/// Tope del oro de un comerciante al terminar un avance.
pub const GOLD_CAP: i32 = 999_999_999;
/// Centros comerciales que se actualizan (el decimosexto registro no).
pub const TRADE_CENTERS_UPDATED: u32 = 15;

/// Campos de reloj del mundo (`GameWorld`, 0x006DE4A0).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WorldClock {
    /// `+0..+5`.
    pub calendar: Calendar,
    /// `+0xA`: número de comerciantes.
    pub merchant_count: u16,
    /// `+0x10`: número de ciudades.
    pub town_count: u32,
    /// `+0x14`: tick (0x006DE4B4).
    pub tick: u32,
    /// `+0x8B`: cuenta atrás diaria de expediciones.
    pub expedition_countdown: u8,
}

/// Sistemas que cuelgan del reloj. Cada método es una llamada del original.
pub trait TickSystems {
    fn clock(&mut self) -> &mut WorldClock;
    /// `OPS+0x92C` (0x006DFC1C): ficha de tiempo.
    fn time_token(&self) -> u32;
    /// El anfitrión puede rechazar el siguiente tick antes de modificar estado.
    fn preflight_tick(&mut self) -> bool {
        true
    }
    /// `charge_daily_taxes_and_close_week(semana)` (0x005315C0).
    fn close_day(&mut self, week_start: bool);
    /// `game_statistics_daily_update` (0x0054DE90).
    fn statistics_daily(&mut self);
    /// `town_info_history_daily_update(tick)` (0x00535450).
    fn town_history_daily(&mut self, tick: u32);
    /// `update_merchant_reputation_and_value(i)` (0x004F7BB0).
    fn merchant_daily(&mut self, merchant: u32);
    /// Solo para un fallo atómico de la fase de comerciante: no debe haber
    /// publicado cambios. Permite reintentar el mismo tick con otra entrada.
    fn merchant_phase_failed(&self) -> bool {
        false
    }
    /// `handle_town_tick(día del año)` (0x0051BA10) sobre la ciudad.
    fn town_daily(&mut self, town: u32, day_of_year: u16);
    /// `trade_center_daily_update` (0x0052D7D0).
    fn trade_center_daily(&mut self, center: u32);
    /// `siege_record_daily_update` (0x006349F0).
    fn siege_daily(&mut self, town: u32);
    /// Barcos (0x00506720 sobre 0x006DD7A0).
    fn ships_tick(&mut self, tick: u32);
    /// `handle_scheduled_tasks_tick(tick)` (0x004D85C0), ver [`crate::scheduler::Scheduler::dispatch`].
    fn scheduled_tasks_tick(&mut self, tick: u32);
    /// `harbor_scene_sync_ships_with_game_state` (0x00613DA0).
    fn harbor_scene_sync(&mut self);
    /// `handle_class11_tick(1)` (0x006127C0): batallas navales.
    fn sea_battles_tick(&mut self);
    /// `siege_record_update_scene` (0x00633A90).
    fn siege_scene(&mut self, town: u32);
    /// Oro del comerciante (`merchant+0`, registros de 0x650 bytes).
    fn merchant_gold(&self, merchant: u32) -> i32;
    fn set_merchant_gold(&mut self, merchant: u32, gold: i32);
}

/// Cómo terminó un avance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdvanceEnd {
    /// Se alcanzó el objetivo (o ya se había alcanzado) y se aplicó el tope de oro.
    Reached,
    /// La ficha no coincidía con `OPS+0x92C`: sale sin tope de oro.
    TokenMismatch,
    /// El anfitrión detuvo el avance antes de la fase naval de batalla.
    /// El tick queda incompleto y debe reanudarse en esa fase.
    InterruptedBeforeBattle,
    /// El planificador dejó una tarea pendiente antes de sincronizar el puerto.
    InterruptedBeforeTask,
    /// La fase diaria del comerciante se rechazó sin cambios y no se avanzó el tick.
    MerchantPhaseFailed,
    /// El anfitrión detuvo el reloj antes del tick por una dependencia no resuelta.
    PreflightFailed,
}

/// `advance_time(target, token)`.
pub fn advance_time<S: TickSystems + ?Sized>(sys: &mut S, target: u32, token: u32) -> AdvanceEnd {
    advance_time_controlled(sys, target, token, |_| false)
}

/// Avanza con una interrupción explícita del anfitrión antes del tick de batalla.
/// Al interrumpirse no se ejecutan ni esa fase ni las escenas de asedio ni el
/// tope de oro: corresponde al llamante reanudar el resto del tick.
pub fn advance_time_controlled<S: TickSystems + ?Sized, F: FnMut(&mut S) -> bool>(
    sys: &mut S,
    target: u32,
    token: u32,
    mut interrupt: F,
) -> AdvanceEnd {
    match drive_with_task_boundary(sys, target, token, &mut |_| false, &mut interrupt) {
        TaskBoundaryEnd::Complete(end) => end,
        TaskBoundaryEnd::Pending(_) => unreachable!("el avance ordinario no habilita cortes de tareas"),
    }
}

/// Continuación privada de un tick cuyo planificador quedó suspendido.
/// El marcador pertenece al mismo sistema y solo debe consumirse una vez.
/// `scheduled_tick` conserva el argumento leído antes del primer despacho,
/// incluso si una tarea modifica el reloj durante la suspensión.
#[derive(Debug, PartialEq, Eq)]
pub struct TaskBoundary {
    scheduled_tick: u32,
    target: u32,
    token: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TaskBoundaryEnd {
    /// El avance terminó o se detuvo en uno de los límites habituales.
    Complete(AdvanceEnd),
    /// El planificador quedó pendiente, antes de la escena de puerto.
    Pending(TaskBoundary),
}

/// Ejecuta el reloj y permite suspenderlo justo después de la primera pasada
/// del planificador, sin ejecutar puerto, batallas, asedios ni tope de oro.
/// `pending` debe identificar una tarea interrumpida que el anfitrión debe
/// resolver; no convierte el estado parcial en una partida guardable.
pub fn advance_time_with_task_boundary<S, P, F>(
    sys: &mut S,
    target: u32,
    token: u32,
    mut pending: P,
    mut interrupt: F,
) -> TaskBoundaryEnd
where
    S: TickSystems + ?Sized,
    P: FnMut(&mut S) -> bool,
    F: FnMut(&mut S) -> bool,
{
    drive_with_task_boundary(sys, target, token, &mut pending, &mut interrupt)
}

fn drive_with_task_boundary<S, P, F>(
    sys: &mut S,
    target: u32,
    token: u32,
    pending: &mut P,
    interrupt: &mut F,
) -> TaskBoundaryEnd
where
    S: TickSystems + ?Sized,
    P: FnMut(&mut S) -> bool,
    F: FnMut(&mut S) -> bool,
{
    while sys.clock().tick < target {
        if token != 0 && sys.time_token() != token {
            return TaskBoundaryEnd::Complete(AdvanceEnd::TokenMismatch);
        }
        if !sys.preflight_tick() {
            return TaskBoundaryEnd::Complete(AdvanceEnd::PreflightFailed);
        }
        let clock = sys.clock();
        clock.tick = clock.tick.wrapping_add(1);
        let tick = clock.tick;
        run_phase(sys, tick & 0xFF);
        if sys.merchant_phase_failed() {
            sys.clock().tick = tick.wrapping_sub(1);
            return TaskBoundaryEnd::Complete(AdvanceEnd::MerchantPhaseFailed);
        }
        // El original vuelve a leer el tick tras cada llamada: un subsistema
        // puede modificar el reloj antes de barcos o del planificador.
        let ship_tick = sys.clock().tick;
        sys.ships_tick(ship_tick);
        let task_tick = sys.clock().tick;
        sys.scheduled_tasks_tick(task_tick);
        if pending(sys) {
            return TaskBoundaryEnd::Pending(TaskBoundary {
                scheduled_tick: task_tick,
                target,
                token,
            });
        }
        sys.harbor_scene_sync();
        if interrupt(sys) {
            return TaskBoundaryEnd::Complete(AdvanceEnd::InterruptedBeforeBattle);
        }
        sys.sea_battles_tick();
        // El número de ciudades se vuelve a leer en cada vuelta.
        let mut town = 0;
        while town < sys.clock().town_count {
            sys.siege_scene(town);
            town += 1;
        }
    }
    let mut merchant = 0;
    while merchant < sys.clock().merchant_count as u32 {
        if sys.merchant_gold(merchant) > GOLD_CAP {
            sys.set_merchant_gold(merchant, GOLD_CAP);
        }
        merchant += 1;
    }
    TaskBoundaryEnd::Complete(AdvanceEnd::Reached)
}

/// Reanuda el mismo tick después de que el anfitrión resuelva la tarea que
/// produjo el marcador. Si aún está pendiente no ejecuta nada. El planificador
/// debe conservar su cabeza y no volver a ejecutar las tareas ya completadas;
/// se despacha de nuevo con el tick original para terminar las vencidas. Si
/// otra tarea necesita resolución, se devuelve un nuevo marcador antes de
/// avanzar al puerto. Ninguna publicación S05 se autoriza mediante esta API.
pub fn resume_after_task_boundary<S, P, F>(
    sys: &mut S,
    boundary: TaskBoundary,
    mut pending: P,
    mut interrupt: F,
) -> TaskBoundaryEnd
where
    S: TickSystems + ?Sized,
    P: FnMut(&mut S) -> bool,
    F: FnMut(&mut S) -> bool,
{
    if pending(sys) {
        return TaskBoundaryEnd::Pending(boundary);
    }
    sys.scheduled_tasks_tick(boundary.scheduled_tick);
    if pending(sys) {
        return TaskBoundaryEnd::Pending(boundary);
    }
    sys.harbor_scene_sync();
    if interrupt(sys) {
        return TaskBoundaryEnd::Complete(AdvanceEnd::InterruptedBeforeBattle);
    }
    sys.sea_battles_tick();
    let mut town = 0;
    while town < sys.clock().town_count {
        sys.siege_scene(town);
        town += 1;
    }
    drive_with_task_boundary(sys, boundary.target, boundary.token, &mut pending, &mut interrupt)
}

/// Completa una interrupción `InterruptedBeforeBattle` y continúa hasta el
/// objetivo. El llamante debe usarla una sola vez para el tick pendiente.
pub fn resume_before_battle<S: TickSystems + ?Sized, F: FnMut(&mut S) -> bool>(
    sys: &mut S,
    target: u32,
    token: u32,
    interrupt: F,
) -> AdvanceEnd {
    sys.sea_battles_tick();
    let mut town = 0;
    while town < sys.clock().town_count {
        sys.siege_scene(town);
        town += 1;
    }
    advance_time_controlled(sys, target, token, interrupt)
}

fn run_phase<S: TickSystems + ?Sized>(sys: &mut S, t: u32) {
    let clock = *sys.clock();
    match t & 7 {
        0 if t == 0 => {
            let clock = sys.clock();
            let tick = clock.tick;
            clock.calendar.update(tick);
            sys.close_day(is_week_start(tick));
            sys.statistics_daily();
            let tick = sys.clock().tick;
            sys.town_history_daily(tick);
            let clock = sys.clock();
            if clock.expedition_countdown > 0 {
                clock.expedition_countdown -= 1;
            }
        }
        1 => {
            let merchant = t >> 3;
            if merchant < clock.merchant_count as u32 {
                sys.merchant_daily(merchant);
            }
        }
        3 => {
            let town = (t + 0xFF) >> 3;
            if town < clock.town_count {
                sys.town_daily(town, clock.calendar.day_of_year);
            }
        }
        4 => {
            let center = t >> 3;
            if center < TRADE_CENTERS_UPDATED {
                sys.trade_center_daily(center);
            }
        }
        6 => {
            let merchant = (t + 0x100) >> 3;
            if merchant < clock.merchant_count as u32 {
                sys.merchant_daily(merchant);
            }
        }
        7 => {
            let town = t >> 3;
            if town < clock.town_count {
                sys.town_daily(town, clock.calendar.day_of_year);
                sys.siege_daily(town);
            }
        }
        _ => {}
    }
}
