//! Lista doble de barcos ordenada por la coordenada x (`ships::remove_from_x_order`, `insert_in_x_order`,
//! `resort_in_x_order`).
//!
//! Cada barco guarda en `+0xA` el anterior y en `+0xC` el siguiente (índices `u16`, fin 0xFFFF o cualquier
//! valor `>= count`) y en `+0x1E` su x (i16). `count` es `ships+0xF4` y `sea_head` es `ships+0xEA`, la
//! cabeza de la lista de barcos en el mar, que sirve de punto de partida de las inserciones.

use super::{r16, w16};
use crate::shipyard::SHIP_SIZE;

pub struct XOrder<'a> {
    pub ships: &'a mut [u8],
    pub count: u16,
    pub sea_head: u16,
}

/// Reordena un barco usando directamente las listas contiguas del tick marítimo.
pub fn resort_from_tick(world: &mut super::tick::TickWorld, ship: u16) {
    let remove = &mut *world.remove;
    let mut order = XOrder {
        ships: remove.ships,
        count: r16(remove.manager, 0xF4),
        sea_head: r16(remove.manager, 0xEA),
    };
    order.resort(ship);
}

pub fn remove_from_tick(world: &mut super::tick::TickWorld, ship: u16) {
    let remove = &mut *world.remove;
    let mut order = XOrder {
        ships: remove.ships,
        count: r16(remove.manager, 0xF4),
        sea_head: r16(remove.manager, 0xEA),
    };
    order.remove(ship);
}

pub fn insert_from_tick(world: &mut super::tick::TickWorld, ship: u16, after: u16) {
    let remove = &mut *world.remove;
    let mut order = XOrder {
        ships: remove.ships,
        count: r16(remove.manager, 0xF4),
        sea_head: r16(remove.manager, 0xEA),
    };
    order.insert(ship, u32::from(after));
}

const PREV: usize = 0xA;
const NEXT: usize = 0xC;
const X: usize = 0x1E;

impl XOrder<'_> {
    fn get(&self, ship: u16, o: usize) -> u16 {
        r16(self.ships, ship as usize * SHIP_SIZE + o)
    }

    fn set(&mut self, ship: u16, o: usize, v: u16) {
        w16(self.ships, ship as usize * SHIP_SIZE + o, v);
    }

    fn x(&self, ship: u16) -> i16 {
        self.get(ship, X) as i16
    }

    /// `ships::remove_from_x_order` (0x0050D040).
    pub fn remove(&mut self, ship: u16) {
        let prev = self.get(ship, PREV);
        if prev < self.count && self.get(prev, NEXT) == ship {
            let v = self.get(ship, NEXT);
            self.set(prev, NEXT, v);
        }
        if self.get(ship, NEXT) < self.count {
            let next = self.get(ship, NEXT);
            if self.get(next, PREV) == ship {
                let v = self.get(ship, PREV);
                self.set(next, PREV, v);
            }
        }
        self.set(ship, PREV, 0xFFFF);
        self.set(ship, NEXT, 0xFFFF);
    }

    /// `ships::insert_in_x_order` (0x0050CE50): coloca `ship` detrás de `after` (o de la cabeza de la lista
    /// del mar si `after` queda fuera) y lo reordena. Sin cabeza válida no hace nada.
    pub fn insert(&mut self, ship: u16, after: u32) {
        let mut after = after;
        if self.count as u32 <= after {
            if self.count <= self.sea_head {
                return;
            }
            after = self.sea_head as u32;
        }
        let prev = self.get(ship, PREV);
        let target = after as u16;
        if prev < self.count {
            let next = self.get(ship, NEXT);
            let linked = (self.count <= next || self.get(next, PREV) == ship) && self.get(prev, NEXT) == ship;
            if linked {
                self.resort(ship);
                return;
            }
            if self.get(prev, NEXT) == ship {
                self.set(prev, NEXT, 0xFFFF);
            }
            self.set(ship, PREV, 0xFFFF);
            let next = self.get(ship, NEXT);
            if next < self.count {
                if self.get(next, PREV) == ship {
                    self.set(next, PREV, 0xFFFF);
                }
                self.set(ship, NEXT, 0xFFFF);
            }
        }
        self.set(ship, PREV, after as u16);
        let tail = self.get(target, NEXT);
        self.set(ship, NEXT, tail);
        self.set(target, NEXT, ship);
        let next = self.get(ship, NEXT);
        if next < self.count {
            self.set(next, PREV, ship);
        }
        self.resort(ship);
    }

    /// `ships::resort_in_x_order` (0x0050D0C0): desplaza `ship` hacia atrás o hacia delante hasta que su x
    /// quede entre las de sus vecinos. Un enlace roto lo corta con 0xFFFF.
    pub fn resort(&mut self, ship: u16) {
        let x = self.x(ship);
        let mut moved = false;
        let mut cur = self.get(ship, PREV);
        if cur < self.count {
            loop {
                if self.get(cur, NEXT) == ship {
                    if self.x(cur) < x {
                        break;
                    }
                    moved = true;
                    let next = self.get(ship, NEXT);
                    self.set(cur, NEXT, next);
                    if next < self.count {
                        let v = self.get(ship, PREV);
                        self.set(next, PREV, v);
                    }
                    let v = self.get(ship, PREV);
                    self.set(ship, NEXT, v);
                    let back = self.get(self.get(ship, PREV), PREV);
                    self.set(ship, PREV, back);
                    let now_next = self.get(ship, NEXT);
                    self.set(now_next, PREV, ship);
                    let back = self.get(ship, PREV);
                    if back < self.count {
                        self.set(back, NEXT, ship);
                    }
                } else {
                    self.set(ship, PREV, 0xFFFF);
                }
                cur = self.get(ship, PREV);
                if cur >= self.count {
                    break;
                }
            }
            if moved {
                return;
            }
        }
        cur = self.get(ship, NEXT);
        if cur < self.count {
            while self.get(cur, PREV) == ship {
                if x <= self.x(cur) {
                    return;
                }
                let back = self.get(ship, PREV);
                self.set(cur, PREV, back);
                if back < self.count {
                    let v = self.get(ship, NEXT);
                    self.set(back, NEXT, v);
                }
                let v = self.get(ship, NEXT);
                self.set(ship, PREV, v);
                cur = self.get(self.get(ship, NEXT), NEXT);
                self.set(ship, NEXT, cur);
                if cur < self.count {
                    self.set(cur, PREV, ship);
                }
                let back = self.get(ship, PREV);
                self.set(back, NEXT, ship);
                cur = self.get(ship, NEXT);
                if self.count <= cur {
                    return;
                }
            }
            self.set(ship, NEXT, 0xFFFF);
        }
    }
}
