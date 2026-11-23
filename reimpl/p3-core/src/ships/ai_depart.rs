//! Salida de la IA atracada: `Ship::ai_state2` (0x00519EB0) y la creación de convoy `Fleet::create_convoy`
//! (0x00509B90) con la puesta a punto `Convoy::init` (0x00501680).
//!
//! La limpieza de ruta (0x00516840) y la ampliación de la tabla de convoyes (0x00509AE0) son ganchos del anfitrión:
//! la segunda no cabe en un segmento de bytes de tamaño fijo y el anfitrión debe devolver `true` si la ha hecho.

use super::convoy::refresh_from_ships;
use super::convoy_drop::pool_ok;
use super::remove::{CONVOY_SIZE, SHIP_SIZE};
use super::tick::{Link, TickWorld};
use super::{r16, r32, w16, w32, DivideError};

#[derive(Debug, PartialEq, Eq)]
pub enum DepartError {
    /// El cálculo de velocidad del convoy dividió por cero (fallo del original).
    Divide,
    /// Hacía falta ampliar la tabla de convoyes y el anfitrión no pudo.
    NoGrow,
    /// El original leería memoria ajena a la tabla (índice de convoy libre fuera de ella).
    OutOfRange,
}

impl From<DivideError> for DepartError {
    fn from(_: DivideError) -> Self {
        DepartError::Divide
    }
}

pub trait AiDepartHost {
    /// `ship::clear_route` (0x00516840).
    fn clear_route(&mut self, w: &mut TickWorld, ship: u16);
    /// `Fleet::grow_convoys` (0x00509AE0): amplía la tabla en 0x20 entradas y enlaza las nuevas. `false` si no puede.
    fn grow_convoys(&mut self, _w: &mut TickWorld) -> bool {
        false
    }
}

const GUARD: usize = 0x10000;

/// Entradas que añade `Fleet::grow_convoys` (0x00509AE0).
pub const GROW_STEP: usize = 0x20;

/// `Fleet::grow_convoys` (0x00509AE0) sobre una tabla que ya tiene sitio para `capacidad + 0x20` entradas: sube la
/// capacidad (+0xF6) y enlaza las entradas libres nuevas (siguiente = índice + 1, estado +0x12 = 0xFFFF).
/// Devuelve `false`, sin tocar nada, si la tabla no tiene sitio.
pub fn grow_in_place(w: &mut TickWorld) -> bool {
    let cap = r16(w.remove.manager, 0xF6) as usize;
    let free = r16(w.remove.manager, 0xE4) as usize;
    if free < cap {
        return true;
    }
    let new = (cap + GROW_STEP) & 0xFFFF;
    if new * CONVOY_SIZE > w.remove.convoys.len() {
        return false;
    }
    w16(w.remove.manager, 0xF6, new as u16);
    for i in free..new {
        let b = i * CONVOY_SIZE;
        w16(w.remove.convoys, b + 8, i as u16 + 1);
        w16(w.remove.convoys, b + 0x12, 0xFFFF);
    }
    true
}

fn read(w: &TickWorld, l: Link) -> u16 {
    match l {
        Link::Manager(o) => r16(w.remove.manager, o),
        Link::Ship(o) => r16(w.remove.ships, o),
    }
}

fn write(w: &mut TickWorld, l: Link, v: u16) {
    match l {
        Link::Manager(o) => w16(w.remove.manager, o, v),
        Link::Ship(o) => w16(w.remove.ships, o, v),
    }
}

/// `Convoy::init` (0x00501680): el barco `ship` queda como único miembro y líder; `kind` va a `+0x14`.
pub(crate) fn init_convoy(w: &mut TickWorld, cv: u16, ship: u16, kind: u16) -> Result<(), DepartError> {
    let b = cv as usize * CONVOY_SIZE;
    w16(w.remove.convoys, b + 0xA, ship);
    w16(w.remove.convoys, b + 0x10, ship);
    w16(w.remove.convoys, b + 0x14, kind);
    refresh_from_ships(w, cv)?;
    let c = &mut *w.remove.convoys;
    w32(c, b + 0x20, 0);
    c[b + 0x24..b + 0x34].fill(0xFF);
    c[b + 0x34..b + 0x38].fill(0xFF);
    w16(c, b + 0x38, 0xFFFF);
    w16(c, b + 2, 0xFFFF);
    let f = r32(c, b + 0x18) | 0x4401;
    w32(c, b + 0x18, f);
    Ok(())
}

/// `Fleet::create_convoy` (0x00509B90): saca el barco de la lista de atracados y le crea un convoy propio.
/// `keep_here` (arg. 2): el destino es la propia ciudad y la fase es 0. Devuelve `false` si no se cumplen las condiciones.
pub fn create_convoy(
    w: &mut TickWorld,
    ship: u16,
    keep_here: bool,
    phase: u32,
    dest: u32,
    host: &mut dyn AiDepartHost,
) -> Result<bool, DepartError> {
    let count = r16(w.remove.manager, 0xF4);
    if ship >= count {
        return Ok(false);
    }
    let o = ship as usize * SHIP_SIZE;
    if o + SHIP_SIZE > w.remove.ships.len() || !pool_ok(w, ship) {
        return Ok(false);
    }
    if r16(w.remove.ships, o + 0x134) > 1 || u32::from(w.remove.ships[o + 0x39]) >= w.remove.town_count {
        return Ok(false);
    }
    if r16(w.remove.ships, o + 8) < r16(w.remove.manager, 0xF6) {
        return Ok(false);
    }
    if r16(w.remove.manager, 0xE4) >= r16(w.remove.manager, 0xF6) && !host.grow_convoys(w) {
        return Err(DepartError::NoGrow);
    }
    // busca el enlace que apunta al barco en la lista de atracados (cabeza en +0xE8)
    let mut link = Link::Manager(0xE8);
    let mut cur = read(w, link);
    let mut guard = 0;
    while cur != ship {
        guard += 1;
        if cur >= count || guard > GUARD {
            return Ok(false);
        }
        link = Link::Ship(cur as usize * SHIP_SIZE + 6);
        cur = read(w, link);
    }
    let next = r16(w.remove.ships, o + 6);
    write(w, link, next);
    w16(w.remove.ships, o + 6, 0xFFFF);
    if keep_here {
        w.remove.ships[o + 0x38] = w.remove.ships[o + 0x39];
        w16(w.remove.ships, o + 0x138, 0);
    } else {
        w.remove.ships[o + 0x38] = dest as u8;
        w16(w.remove.ships, o + 0x138, (phase << 8) as u16);
    }
    w.remove.ships[o + 0x3C] |= 0x20;
    w.remove.ships[o + 0x37] = w.remove.ships[o + 0x39];
    w16(w.remove.ships, o + 0x134, 1);
    let cv = r16(w.remove.manager, 0xE4);
    let b = cv as usize * CONVOY_SIZE;
    if b + CONVOY_SIZE > w.remove.convoys.len() {
        return Err(DepartError::OutOfRange);
    }
    w16(w.remove.ships, o + 8, cv);
    let free_next = r16(w.remove.convoys, b + 8);
    w16(w.remove.manager, 0xE4, free_next);
    let head = r16(w.remove.manager, 0xEC);
    w16(w.remove.convoys, b + 8, head);
    w16(w.remove.manager, 0xEC, cv);
    if !keep_here && dest != u32::from(w.remove.ships[o + 0x39]) {
        init_convoy(w, cv, ship, 0)?;
    } else {
        init_convoy(w, cv, ship, 3)?;
    }
    Ok(true)
}

/// `Ship::ai_state2` (0x00519EB0): la IA acaba de decidir zarpar. Une el barco a un convoy en formación hacia el mismo
/// destino o le crea uno. `link` es el enlace de la lista de atracados que contiene al barco; devuelve `true` si el
/// barco ya no está en esa lista.
pub fn ai_depart(w: &mut TickWorld, ship: u16, link: Link, host: &mut dyn AiDepartHost) -> Result<bool, DepartError> {
    let o = ship as usize * SHIP_SIZE;
    // `di`: lo que el enlace contiene al entrar (el propio barco cuando la lista es coherente)
    let di = read(w, link);
    let cap = r16(w.remove.manager, 0xF6);
    let mut cv = r16(w.remove.manager, 0xEC);
    let mut mismatches = 0u32;
    let mut guard = 0;
    while cv < cap {
        guard += 1;
        if guard > GUARD {
            break;
        }
        let b = cv as usize * CONVOY_SIZE;
        let flags = w.remove.convoys[b + 0x14];
        if flags & 2 == 0 && r16(w.remove.convoys, b + 0x12) == 1 {
            if r16(w.remove.convoys, b + 0x1C) == u16::from(w.remove.ships[o + 0x39])
                && w.remove.convoys[b + 0x1F] == w.remove.ships[o + 0x38]
            {
                let next = r16(w.remove.ships, o + 6);
                write(w, link, next);
                let head = r16(w.remove.convoys, b + 0xA);
                w16(w.remove.ships, o + 6, head);
                w16(w.remove.convoys, b + 0xA, di);
                w.remove.ships[o + 0x3C] |= 0x60;
                w16(w.remove.ships, o + 8, cv);
                w16(w.remove.ships, o + 0x134, 1);
                let f = r32(w.remove.convoys, b + 0x18) | 0x4002;
                w32(w.remove.convoys, b + 0x18, f);
                host.clear_route(w, di);
                return Ok(true);
            }
            mismatches += 1;
        }
        cv = r16(w.remove.convoys, b + 8);
    }
    if mismatches > 10 {
        w16(w.remove.ships, o + 0x134, 2);
        return Ok(false);
    }
    w16(w.remove.ships, o + 0x134, 0);
    let dest = u32::from(w.remove.ships[o + 0x38]);
    create_convoy(w, di, false, 5, dest, host)?;
    if read(w, link) != di {
        return Ok(true);
    }
    w16(w.remove.ships, o + 0x134, 2);
    Ok(false)
}
