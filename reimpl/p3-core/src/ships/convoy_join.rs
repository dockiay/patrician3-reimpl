//! `op_join_ship_to_convoy` (0x00509D10): une un barco atracado a un convoy ya formado en su ciudad.
//!
//! Argumentos del original: un barco que ya es miembro del convoy (`member`) y el barco que se une (`ship`).
//! Los ganchos del anfitrión son las llamadas que el original hace fuera de las listas: limpieza de ruta (0x516840),
//! aviso de interfaz (0x44F940, solo si el barco es del jugador), admisión hanseática (0x5039F0) y la tarea del convoy
//! (0x4D8DB0 + el comerciante 0x5303C0/0x4F8D20). La baja de la lista por x (0x50D040) y el refresco del convoy
//! (0x501BE0) se ejecutan sobre el mundo.

use super::convoy::refresh_from_ships;
use super::convoy_plan::CREW_MIN;
use super::order::remove_from_tick;
use super::remove::{CONVOY_SIZE, SHIP_SIZE};
use super::tick::TickWorld;
use super::{r16, r32, w16, w32, DivideError};

pub trait JoinHost {
    /// `ship::clear_route` (0x00516840).
    fn clear_route(&mut self, w: &mut TickWorld, ship: u16);
    /// Aviso de interfaz (0x0044F940) cuando el barco pertenece al jugador.
    fn ui_ship_joined(&mut self, ship: u16);
    /// `0x005039F0`: admisión del barco en un convoy en estado 9 (flota hanseática).
    fn hanse_join_allowed(&mut self, w: &mut TickWorld, convoy: u16, ship: u16) -> bool;
    /// `scheduled_tasks_get_task` (0x004D8DB0): los 16 bytes del registro de la tarea (`None` si no existe).
    fn task_record(&mut self, task: u16) -> Option<&mut [u8]>;
    /// `get_merchant` + `0x004F8D20` del dueño del barco, tras fijar la ranura de tarea.
    fn merchant_joined(&mut self, owner: u8);
}

#[derive(Debug, PartialEq, Eq)]
pub enum JoinError {
    Divide,
    NoTask,
}

impl From<DivideError> for JoinError {
    fn from(_: DivideError) -> Self {
        JoinError::Divide
    }
}

/// Devuelve `Ok(true)` si el barco se unió (aunque el original termine antes de la parte de tareas).
pub fn join_convoy(w: &mut TickWorld, member: u32, ship: u32, host: &mut dyn JoinHost) -> Result<bool, JoinError> {
    let count = u32::from(r16(w.remove.manager, 0xF4));
    let cap = r16(w.remove.manager, 0xF6);
    if member >= count || ship >= count {
        return Ok(false);
    }
    let m = member as usize * SHIP_SIZE;
    let s = ship as usize * SHIP_SIZE;
    if s + SHIP_SIZE > w.remove.ships.len() || m + SHIP_SIZE > w.remove.ships.len() {
        return Ok(false);
    }
    let cv = r16(w.remove.ships, m + 8);
    if cv >= cap {
        return Ok(false);
    }
    let b = cv as usize * CONVOY_SIZE;
    if b + CONVOY_SIZE > w.remove.convoys.len() {
        return Ok(false);
    }
    let flags = w.remove.convoys[b + 0x14];
    let sh = &w.remove.ships[s..s + SHIP_SIZE];
    if flags & 2 == 2 {
        if w.remove.convoys[b] != sh[0] {
            return Ok(false);
        }
    } else {
        if r16(sh, 0x40) < u16::from(CREW_MIN[(sh[0xE] & 3) as usize]) {
            return Ok(false);
        }
        if (r32(sh, 0x18).wrapping_mul(5) as i32) < r32(sh, 0x14) as i32 {
            return Ok(false);
        }
    }
    if r16(sh, 8) < cap || flags & 0x10 == 0x10 || (flags & 4 == 4 && sh[0xE] >= 2) {
        return Ok(false);
    }
    if u16::from(sh[0x39]) != r16(w.remove.convoys, b + 0x1C) {
        return Ok(false);
    }
    if sh[0xE] & 2 != 0 {
        let town = u32::from(w.remove.convoys[b + 0x1F]);
        if town < w.remove.town_count {
            let at = town as usize * 0x9F8 + 0x2C8;
            if w.remove.towns.get(at).map_or(false, |v| v & 1 != 0) {
                return Ok(false);
            }
        }
    }
    let state = r16(sh, 0x134);
    if state != 0 && (state != 1 || r16(sh, 0x138) > r16(w.remove.convoys, b + 0x16)) {
        return Ok(false);
    }
    if r16(w.remove.convoys, b + 0x12) == 9 && !host.hanse_join_allowed(w, cv, ship as u16) {
        return Ok(false);
    }
    // Busca el enlace de la lista de atracados (cabeza en +0xE8) que apunta al barco.
    enum Link {
        Manager,
        Ship(usize),
    }
    let mut link = Link::Manager;
    let mut cur = r16(w.remove.manager, 0xE8);
    let mut guard = 0;
    while u32::from(cur) != ship {
        guard += 1;
        if cur >= count as u16 || guard > 0x10000 {
            return Ok(false);
        }
        link = Link::Ship(cur as usize * SHIP_SIZE + 6);
        cur = r16(w.remove.ships, cur as usize * SHIP_SIZE + 6);
    }
    let next = r16(w.remove.ships, s + 6);
    match link {
        Link::Manager => w16(w.remove.manager, 0xE8, next),
        Link::Ship(at) => w16(w.remove.ships, at, next),
    }
    let head = r16(w.remove.convoys, b + 0xA);
    w16(w.remove.ships, s + 6, head);
    w16(w.remove.convoys, b + 0xA, ship as u16);
    let member_cv = r16(w.remove.ships, m + 8);
    w16(w.remove.ships, s + 8, member_cv);
    w.remove.ships[s + 0x3C] |= 0x60;
    w.remove.ships[s + 0x136] &= 0xFC;
    host.clear_route(w, ship as u16);
    if u32::from(w.remove.ships[s]) == w.player {
        host.ui_ship_joined(ship as u16);
    }
    remove_from_tick(w, ship as u16);
    refresh_from_ships(w, cv)?;
    let state = r16(w.remove.convoys, b + 0x12);
    w32(w.remove.convoys, b + 0x18, -1);
    if state != 9 && state != 0xA {
        return Ok(true);
    }
    let task = r16(w.remove.convoys, b + 2);
    let owner = w.remove.ships[s];
    w16(w.remove.ships, s + 0x138, 0);
    // el original llama primero a get_task; si falla, lee memoria ajena
    let ok = host.task_record(task).is_some();
    host.merchant_joined(owner);
    let rec = host.task_record(task).ok_or(JoinError::NoTask)?;
    debug_assert!(ok);
    for i in 0..8 {
        if rec[8 + i] == owner {
            rec[8 + i] = 0xFF;
            break;
        }
    }
    Ok(true)
}
