//! Avance sin interfaz de C10 sobre la sesión y los componentes navales existentes.
use crate::Session;
use serde_json::{json, Value};
use std::collections::BTreeMap;

fn calls_since(now: &BTreeMap<&'static str, u64>, before: &BTreeMap<&'static str, u64>) -> BTreeMap<&'static str, u64> {
    now.iter()
        .filter_map(|(&name, &count)| {
            let delta = count.saturating_sub(*before.get(name).unwrap_or(&0));
            (delta > 0).then_some((name, delta))
        })
        .collect()
}

/// Avanza un número de días, resolviendo encuentros que no requieren diálogo ni
/// operaciones del usuario. Devuelve las notificaciones y efectos observados.
/// Si una rama necesita un efecto o una decisión aún no conectados, devuelve el error y no
/// afirma haber completado el intervalo; la sesión conserva el punto alcanzado.
pub fn advance_days(session: &mut Session, days: u32) -> Result<Value, String> {
    let ticks = days.checked_mul(256).ok_or("objetivo de reloj fuera de rango")?;
    let mut result = advance_ticks(session, ticks)?;
    result["days"] = days.into();
    Ok(result)
}

/// Cómo avanza una batalla automática respecto al reloj mundial.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BattlePacing {
    /// Mundo detenido mientras la batalla avanza de 16 en 16 subticks hasta
    /// terminar (comportamiento anterior; útil para contrastes que no
    /// ejecutan el reloj durante la batalla).
    Paused,
    /// Como el original: mientras todas las batallas son automáticas el reloj
    /// sigue (`0x00612D20` → C8) y cada fotograma de `frame_ms` milisegundos
    /// despacha la operación 0x94 pendiente y programa la siguiente con la
    /// cadencia de `0x00612DB0` (`BattleTimePump`, modo de tiempo 0). El reloj
    /// de pared no se guarda en la partida: la duración del fotograma es una
    /// elección del anfitrión, igual que lo es la del equipo que ejecuta el juego.
    WithClock { frame_ms: u32 },
}

/// Fotograma por defecto: una unidad de 40 ms de la cadencia original.
pub const DEFAULT_FRAME_MS: u32 = 40;

/// Mismo protocolo de encuentros que `advance_days`, con puntos de muestreo
/// exactos dentro del día para el contraste encadenado con x86.
pub fn advance_ticks(session: &mut Session, ticks: u32) -> Result<Value, String> {
    advance_ticks_with(session, ticks, BattlePacing::Paused)
}

/// `advance_days` con la política de batallas indicada.
pub fn advance_days_with(session: &mut Session, days: u32, pacing: BattlePacing) -> Result<Value, String> {
    let ticks = days.checked_mul(256).ok_or("objetivo de reloj fuera de rango")?;
    let mut result = advance_ticks_with(session, ticks, pacing)?;
    result["days"] = days.into();
    Ok(result)
}

fn collect_step(step: &Value, effects: &mut Vec<Value>) -> bool {
    if step["capture_events"].as_array().is_some_and(|events| !events.is_empty()) {
        effects.push(step["capture_events"].clone());
    }
    step["battle_destroyed"] == true
}

/// Termina con el mundo detenido la batalla activada, de 16 en 16 subticks.
fn finish_paused(session: &mut Session, tick: u32, effects: &mut Vec<Value>) -> Result<u32, String> {
    for steps in 1..=65536 {
        let step = session.advance_paused_battle_lean(16)?;
        if collect_step(&step, effects) {
            return Ok(steps);
        }
    }
    Err(format!("batalla en tick {tick} no terminó"))
}

/// Comprueba que el encuentro devuelto por el reloj no necesita al usuario.
fn admissible_encounter(session: &Session, result: &Value, start: u32) -> Result<Value, String> {
    let encounter = result
        .get("encounter")
        .filter(|v| v.is_object())
        .ok_or("avance detenido sin encuentro identificable")?;
    if encounter["open_for_player_requested"] == true {
        return Err(format!(
            "efecto open_for_player pendiente en tick {} ({} ticks desde el inicio): {encounter}",
            session.sim.idle.clock.tick,
            session.sim.idle.clock.tick - start
        ));
    }
    if !encounter["dialog"].is_null() || !encounter["operations"].as_array().is_some_and(|ops| ops.is_empty()) {
        return Err(format!("encuentro requiere intervención: {encounter}"));
    }
    Ok(encounter.clone())
}

/// Resultado de una batalla que avanza junto al reloj.
enum ClockBattleEnd {
    /// Destruida por el original; el reloj sigue en el mismo tick.
    Destroyed,
    /// El intervalo terminó con la batalla viva: se completó sin reloj.
    IntervalEnd,
    /// Otro encuentro apareció durante la batalla: la viva se completó sin
    /// reloj y el nuevo encuentro (`Value`) queda pendiente de integrar.
    Overlap(Value),
}

fn run_with_clock(
    session: &mut Session,
    target: u32,
    frame_ms: u32,
    start: u32,
    effects: &mut Vec<Value>,
    world_ticks: &mut u32,
    battle_ticks: &mut u32,
) -> Result<ClockBattleEnd, String> {
    let (ms, div, hotseat) = session.battle_time_settings();
    if frame_ms == 0 || hotseat == 0 && ms == 0 {
        return Err("división por cero en reloj de batalla".into());
    }
    let mut now = 0u32;
    let mut pump = p3_core::pump::BattleTimePump { enabled: true, base: now };
    let mut pending: Option<u16> = None;
    let mut elapsed_in_tick = 0u32;
    let tick_ms = ms.max(1);
    for _ in 0..(1u32 << 24) {
        // Fotograma: primero la 0x94 encolada en el bombeo anterior.
        if let Some(t) = pending.take() {
            *battle_ticks += u32::from(t);
            if collect_step(&session.advance_paused_battle_lean(t)?, effects) {
                return Ok(ClockBattleEnd::Destroyed);
            }
            pump.enabled = true;
        }
        pending = pump
            .schedule(now, 0, hotseat, ms, div)
            .map_err(str::to_string)?
            .map(|op| op.0[1] as u16);
        now = now.wrapping_add(frame_ms);
        elapsed_in_tick += frame_ms;
        while elapsed_in_tick >= tick_ms {
            elapsed_in_tick -= tick_ms;
            let interrupted = session.sim.idle.clock.tick >= target;
            let result = if interrupted { None } else { Some(session.advance_tick_beside_auto_battle()?) };
            let overlap = match &result {
                Some(r) if r["reached"] != true && !r["encounter"].is_null() => Some(admissible_encounter(session, r, start)?),
                _ => None,
            };
            if result.is_some() && overlap.is_none() {
                *world_ticks += 1;
                continue;
            }
            // Fin del intervalo u otro encuentro: la batalla viva se completa sin reloj.
            let parked = if overlap.is_some() { session.take_pending_encounter() } else { None };
            if let Some(t) = pending.take() {
                *battle_ticks += u32::from(t);
                if collect_step(&session.advance_paused_battle_lean(t)?, effects) {
                    if let Some(start) = parked { session.restore_pending_encounter(start)? }
                    return Ok(overlap.map_or(ClockBattleEnd::IntervalEnd, ClockBattleEnd::Overlap));
                }
            }
            let tick = session.sim.idle.clock.tick;
            *battle_ticks += 16 * finish_paused(session, tick, effects)?;
            if let Some(start) = parked { session.restore_pending_encounter(start)? }
            return Ok(overlap.map_or(ClockBattleEnd::IntervalEnd, ClockBattleEnd::Overlap));
        }
    }
    Err("batalla automática sin terminar tras el límite de fotogramas".into())
}

/// Avanza `ticks` resolviendo los encuentros que no requieren al usuario con
/// la política `pacing`. Con `WithClock` el informe añade, por encuentro, los
/// ticks mundiales transcurridos durante la batalla y cómo terminó.
pub fn advance_ticks_with(session: &mut Session, ticks: u32, pacing: BattlePacing) -> Result<Value, String> {
    let start = session.sim.idle.clock.tick;
    let idle_before = session.sim.idle.calls.clone();
    let host_before = session.sim.host.calls.clone();
    let target = start.checked_add(ticks).ok_or("objetivo de reloj fuera de rango")?;
    let mut encounters = Vec::new();
    let mut pending: Option<Value> = None;
    while session.sim.idle.clock.tick < target || pending.is_some() {
        let encounter = match pending.take() {
            Some(encounter) => encounter,
            None => {
                let result = session.advance_until_encounter_lean(target, Vec::new())?;
                if result["reached"] == true {
                    break;
                }
                admissible_encounter(session, &result, start)?
            }
        };
        let tick = session.sim.idle.clock.tick;
        session.complete_encounter_tick()?;
        session.activate_encounter_battle()?;
        let mut effects = Vec::new();
        match pacing {
            BattlePacing::WithClock { frame_ms } if session.auto_battle_live() => {
                let (mut world_ticks, mut battle_ticks) = (0, 0);
                let end = run_with_clock(session, target, frame_ms, start, &mut effects, &mut world_ticks, &mut battle_ticks)?;
                let ended = match end {
                    ClockBattleEnd::Destroyed => "destruida",
                    ClockBattleEnd::IntervalEnd => "fin_de_intervalo_sin_reloj",
                    ClockBattleEnd::Overlap(next) => {
                        pending = Some(next);
                        "otro_encuentro_sin_reloj"
                    }
                };
                encounters.push(json!({"tick": tick, "notifications": encounter["notifications"], "capture_events": effects,
                    "world_ticks_during_battle": world_ticks, "battle_subticks": battle_ticks, "ended": ended}));
            }
            _ => {
                finish_paused(session, tick, &mut effects)?;
                encounters.push(json!({"tick": tick, "notifications": encounter["notifications"], "capture_events": effects}));
            }
        }
    }
    if session.sim.idle.clock.tick != target {
        return Err("el reloj no alcanzó el objetivo".into());
    }
    Ok(
        json!({"start_tick": start, "end_tick": target, "ticks": ticks, "encounters": encounters,
        "callbacks_auxiliares": calls_since(&session.sim.idle.calls, &idle_before),
        "callbacks_urbanos_sin_implementar": calls_since(&session.sim.host.calls, &host_before)}),
    )
}
