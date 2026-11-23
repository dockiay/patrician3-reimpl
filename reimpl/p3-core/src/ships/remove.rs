//! Baja de un barco (`ships::remove_ship` 0x00507FB0): lo saca de la cola de capitanes, de la lista por x, de la
//! lista del comerciante, de las listas de puerto/mar/ciudad y del convoy, y lo devuelve a la lista de libres.
//!
//! Campos del barco: `+0` dueño, `+4` siguiente del comerciante, `+6` enlace de lista, `+8` convoy,
//! `+0xA`/`+0xC` vecinos por x, `+0x42` capitán, `+0x132` ranura del pool, `+0x134` estado, `+0x137` tipo de libres.
//! Gestor: `+0` capitanes (16 bytes), `+8` convoyes (0x3C), `+0xC` cinco punteros, `+0xE0` capitán libre,
//! `+0xE2`/`+0xE6` libres, `+0xE8` lista A, `+0xEA` lista del mar, `+0xF2` capacidad de capitanes, `+0xF4` barcos,
//! `+0xF6` convoyes. Ciudad (0x9F8 bytes): listas en `+0x81C`/`+0x81E`. Comerciante (0x650): cabeza en `+0xE`.

use super::{r16, r32, w16};

pub const SHIP_SIZE: usize = 0x180;
pub const TOWN_SIZE: usize = 0x9F8;
pub const MERCHANT_SIZE: usize = 0x650;
pub const CONVOY_SIZE: usize = 0x3C;

/// Llamadas que el original hace fuera de las listas. Las dos que modifican otros datos (`ClearRoute` y
/// `OrderRemove`) no cambian nada que lea el resto de la función: quien llame las aplica después.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RemoveEvent {
    /// 0x00494990 (interfaz A, si hay) con el barco y 0x17.
    UiA(u32),
    /// 0x004778A0 (interfaz B, si hay) con el barco y 0x17.
    UiB(u32),
    /// `clear_route` (0x00516840).
    ClearRoute(u32),
    /// `remove_from_x_order` (0x0050D040).
    OrderRemove(u32),
    /// Liberar la ranura del pool (0x004D4DD0, segundo argumento 0).
    PoolRelease(u16),
    /// Aviso al comerciante con su índice (0x004F5900).
    MerchantChanged(u32),
    /// Cinco punteros del gestor distintos de cero (0x00515F00): puntero y barco.
    ConvoyRelease(u32, u32),
    /// Convoy con sus miembros cambiados (0x00501BE0).
    ConvoyRefresh(u16),
}

pub struct RemoveWorld<'a> {
    pub manager: &'a mut [u8],
    pub ships: &'a mut [u8],
    pub captains: &'a mut [u8],
    pub convoys: &'a mut [u8],
    pub merchants: &'a mut [u8],
    pub towns: &'a mut [u8],
    /// Global 0x6DD72A (tamaño del pool de ranuras).
    pub pool_limit: u16,
    /// Global 0x6DE4AA.
    pub merchant_count: u16,
    /// Global 0x6DE4B0.
    pub town_count: u32,
    pub ui_a: bool,
    pub ui_b: bool,
}

fn ship_at(i: u32) -> usize {
    i as usize * SHIP_SIZE
}

impl RemoveWorld<'_> {
    /// Recorre la lista cuyo enlace de cabeza está en `head` (`Head::Manager`/`Town`/`Convoy`) y, si encuentra
    /// `index`, le pone el siguiente del barco.
    fn unlink(&mut self, head: Head, index: u32, count: u16) -> bool {
        let mut link = head;
        loop {
            let value = self.read(link);
            if value >= count {
                return false;
            }
            if value as u32 == index {
                let next = r16(self.ships, ship_at(index) + 6);
                self.write(link, next);
                return true;
            }
            link = Head::Ship(ship_at(value as u32) + 6);
        }
    }

    fn read(&self, h: Head) -> u16 {
        match h {
            Head::Manager(o) => r16(self.manager, o),
            Head::Town(o) => r16(self.towns, o),
            Head::Convoy(o) => r16(self.convoys, o),
            Head::Ship(o) => r16(self.ships, o),
        }
    }

    fn write(&mut self, h: Head, v: u16) {
        match h {
            Head::Manager(o) => w16(self.manager, o, v),
            Head::Town(o) => w16(self.towns, o, v),
            Head::Convoy(o) => w16(self.convoys, o, v),
            Head::Ship(o) => w16(self.ships, o, v),
        }
    }
}

#[derive(Clone, Copy)]
enum Head {
    Manager(usize),
    Town(usize),
    Convoy(usize),
    Ship(usize),
}

/// `ships::remove_ship` (0x00507FB0). Devuelve las llamadas externas en el orden del original.
pub fn remove_ship(w: &mut RemoveWorld, index: u32) -> Vec<RemoveEvent> {
    use RemoveEvent::*;
    let mut ev = Vec::new();
    let count = r16(w.manager, 0xF4);
    if index >= count as u32 {
        return ev;
    }
    if w.ui_a {
        ev.push(UiA(index));
    }
    if w.ui_b {
        ev.push(UiB(index));
    }
    let s = ship_at(index);
    ev.push(ClearRoute(index));

    // Capitán: si no está ya en la lista de libres, se le devuelve.
    let capacity = r16(w.manager, 0xF2);
    let captain = r16(w.ships, s + 0x42);
    if captain < capacity {
        let mut free = r16(w.manager, 0xE0);
        let mut found = false;
        while free < capacity {
            if free == captain {
                found = true;
                break;
            }
            free = r16(w.captains, free as usize * 16);
        }
        if !found {
            let old = r16(w.captains, captain as usize * 16);
            w16(w.ships, s + 0x42, old);
            let head = r16(w.manager, 0xE0);
            w16(w.captains, captain as usize * 16, head);
            w16(w.manager, 0xE0, captain);
        }
    }
    w16(w.ships, s + 0x42, 0xFFFF);

    if r16(w.ships, s + 0xA) < count || r16(w.ships, s + 0xC) < count {
        ev.push(OrderRemove(index));
    } else {
        w16(w.ships, s + 0xA, 0xFFFF);
        w16(w.ships, s + 0xC, 0xFFFF);
    }
    let slot = r16(w.ships, s + 0x132);
    if slot < w.pool_limit {
        ev.push(PoolRelease(slot));
        w16(w.ships, s + 0x132, 0xFFFF);
    }

    // Ya está en la lista de libres: no se repite.
    let mut free = r16(w.manager, 0xE2);
    while free < count {
        if free as u32 == index {
            return ev;
        }
        free = r16(w.ships, ship_at(free as u32) + 6);
    }

    w16(w.ships, s + 0x134, 0xFF);
    let owner = w.ships[s] as u16;
    let mut first_town = 0usize;
    if owner < w.merchant_count {
        let m = owner as usize * MERCHANT_SIZE;
        let mut link = m + 0xE;
        let mut value = r16(w.merchants, link);
        let mut at_merchant = true;
        while value < count {
            if value as u32 == index {
                let next = r16(w.ships, s + 4);
                if at_merchant {
                    w16(w.merchants, link, next);
                } else {
                    w16(w.ships, link, next);
                }
            } else {
                link = ship_at(value as u32) + 4;
                at_merchant = false;
            }
            value = if at_merchant {
                r16(w.merchants, link)
            } else {
                r16(w.ships, link)
            };
        }
        w.ships[s] = 0xFF;
        ev.push(MerchantChanged(owner as u32));
    } else {
        // El contador de ciudades de más abajo arranca en 5 en esta rama (quedó del bucle de punteros).
        first_town = 5;
        for k in 0..5 {
            let p = r32(w.manager, 0xC + 4 * k) as u32;
            if p != 0 {
                ev.push(ConvoyRelease(p, index));
            }
        }
        w.ships[s] = 0xFF;
    }
    w16(w.ships, s + 4, 0xFFFF);

    let found = w.unlink(Head::Manager(0xE8), index, count) || w.unlink(Head::Manager(0xEA), index, count);
    if !found {
        let mut t = first_town;
        loop {
            if t as u32 >= w.town_count {
                break;
            }
            let base = t * TOWN_SIZE;
            let a = w.unlink(Head::Town(base + 0x81C), index, count);
            let b = w.unlink(Head::Town(base + 0x81E), index, count);
            t += 1;
            if a || b {
                break;
            }
        }
    }

    let convoy = r16(w.ships, s + 8);
    if convoy < count {
        let head = convoy as usize * CONVOY_SIZE + 0xA;
        if r16(w.convoys, head) < count && w.unlink(Head::Convoy(head), index, count) && convoy < r16(w.manager, 0xF6) {
            ev.push(ConvoyRefresh(convoy));
        }
    }

    // A la lista de libres que corresponde.
    let slot = if w.ships[s + 0x137] != 0 { 0xE6 } else { 0xE2 };
    let head = r16(w.manager, slot);
    w16(w.ships, s + 6, head);
    w16(w.manager, slot, index as u16);
    ev
}
