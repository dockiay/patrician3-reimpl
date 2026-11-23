//! Flota global de barcos (`0x006DD7A4`, 0x180 bytes cada uno) y las funciones de la IA que recorren la lista de
//! barcos de un comerciante (`Merchant+0xE` apunta al primero; cada barco enlaza al siguiente en `+4`).

pub const SHIP_SIZE: usize = 0x180;
const NEXT: usize = 4;
const KIND: usize = 0xE;
const LEVEL: usize = 0xF;
const MARKS: usize = 0x136;
const ARMING_MARK: u8 = 0x10;

/// Los bytes de todos los barcos; su recuento es `0x006DD894`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Fleet {
    pub bytes: Vec<u8>,
}

impl Fleet {
    pub fn count(&self) -> usize {
        self.bytes.len() / SHIP_SIZE
    }
    pub fn u8(&self, ship: usize, o: usize) -> u8 {
        self.bytes[ship * SHIP_SIZE + o]
    }
    pub fn set_u8(&mut self, ship: usize, o: usize, v: u8) {
        self.bytes[ship * SHIP_SIZE + o] = v;
    }
    pub fn next(&self, ship: usize) -> usize {
        let b = ship * SHIP_SIZE + NEXT;
        u16::from_le_bytes([self.bytes[b], self.bytes[b + 1]]) as usize
    }

    /// `merchant_ai_mark_ship_for_arming` (0x004F5080): si menos de un cuarto de los barcos del comerciante
    /// (recorriendo desde `head`) lleva la marca de armar, marca el primero con tipo de casco 1..3 (`+0xE & 3 != 0`)
    /// y sin la marca, y sube su nivel (`+0xF`) hasta 2.
    pub fn mark_ship_for_arming(&mut self, head: u16) {
        let count = self.count();
        let (mut total, mut marked) = (0i32, 0i32);
        let mut i = head as usize;
        while i < count {
            total += 1;
            marked += (self.u8(i, MARKS) & ARMING_MARK != 0) as i32;
            i = self.next(i);
        }
        if total / 4 <= marked {
            return;
        }
        let mut i = head as usize;
        while i < count {
            let next = self.next(i);
            if self.u8(i, KIND) & 3 != 0 && self.u8(i, MARKS) & ARMING_MARK == 0 {
                let marks = self.u8(i, MARKS);
                self.set_u8(i, MARKS, marks | ARMING_MARK);
                let level = self.u8(i, LEVEL);
                if level < 2 {
                    self.set_u8(i, LEVEL, level + 1);
                }
                return;
            }
            i = next;
        }
    }
}

const HOME_TOWN: usize = 0x13A;
const UNASSIGNED: u8 = 0xFF;

/// Oficina del comerciante tal como la ve el reparto de ciudades base: su ciudad (`+0x2C6`) y la siguiente de
/// su lista (`+0x2C8`).
#[derive(Clone, Copy, Debug)]
pub struct OfficeLink {
    pub town: u8,
    pub next: u16,
}

/// Lo que lee `merchant_ai_assign_ship_home_towns` fuera de la flota.
pub struct HomeEnv<'a> {
    /// Recuento de ciudades (`0x006DE4B0`).
    pub town_count: u32,
    /// Dword `+0x2C8` de cada ciudad de la tabla (al menos 40 entradas: la ciudad natal del comerciante puede
    /// quedar por encima del recuento).
    pub town_flags: &'a [u32],
    /// Oficinas del mundo (`get_office`): una consulta fuera de la lista no devuelve oficina.
    pub offices: &'a [OfficeLink],
    /// `Merchant+0xC`, primera oficina del comerciante.
    pub office_head: u16,
    /// `Merchant+0xE`, primer barco.
    pub ship_head: u16,
    /// `Merchant+0x19`, ciudad natal.
    pub home: u8,
}

impl Fleet {
    /// `merchant_ai_assign_ship_home_towns` (0x004F5130). Cupos por ciudad: 2 por oficina del comerciante y 4 para
    /// su ciudad natal. Los barcos con tipo `>= 2` conservan su ciudad mientras queden cupos y la ciudad no lleve
    /// la marca 1; el resto se reasignan. Los de tipo `< 2` pasan después por la misma criba sin la marca.
    pub fn assign_ship_home_towns(&mut self, env: &HomeEnv) {
        let flag = |town: usize| env.town_flags[town] & 1 != 0;
        let count = self.count();
        let towns = env.town_count as usize;
        let mut quota = [0i16; 64];
        let mut office = env.offices.get(env.office_head as usize).copied();
        while let Some(o) = office {
            quota[o.town as usize] = 2;
            office = env.offices.get(o.next as usize).copied();
        }
        quota[env.home as usize] = 4;

        // Primera criba.
        let mut unassigned = false;
        let mut i = env.ship_head as usize;
        while i < count {
            let next = self.next(i);
            let kind = self.u8(i, 0xE) & 7;
            let home = self.u8(i, HOME_TOWN) as usize;
            if kind < 2 {
                if home >= towns {
                    self.set_u8(i, HOME_TOWN, 0);
                }
            } else if home < towns && quota[home] > 0 && !flag(home) {
                quota[home] -= 1;
            } else {
                self.set_u8(i, HOME_TOWN, UNASSIGNED);
                unassigned = true;
            }
            i = next;
        }
        if unassigned {
            let mut i = env.ship_head as usize;
            while i < count {
                let next = self.next(i);
                if self.u8(i, HOME_TOWN) == UNASSIGNED {
                    self.set_u8(i, HOME_TOWN, if flag(env.home as usize) { 0 } else { env.home });
                    for town in (0..towns).rev() {
                        let current = self.u8(i, HOME_TOWN) as usize;
                        if quota[town] > quota[current] && !flag(current) {
                            self.set_u8(i, HOME_TOWN, town as u8);
                        }
                    }
                    let home = self.u8(i, HOME_TOWN) as usize;
                    quota[home] = quota[home].wrapping_sub(1);
                }
                i = next;
            }
        }

        // Segunda criba, para los barcos de tipo < 2.
        let mut unassigned = false;
        let mut i = env.ship_head as usize;
        while i < count {
            let next = self.next(i);
            if self.u8(i, 0xE) & 7 < 2 {
                let home = self.u8(i, HOME_TOWN) as usize;
                if home < towns && quota[home] > 0 {
                    quota[home] -= 1;
                } else {
                    self.set_u8(i, HOME_TOWN, UNASSIGNED);
                    unassigned = true;
                }
            }
            i = next;
        }
        if unassigned {
            let mut i = env.ship_head as usize;
            while i < count {
                let next = self.next(i);
                if self.u8(i, HOME_TOWN) == UNASSIGNED {
                    self.set_u8(i, HOME_TOWN, env.home);
                    for town in (0..towns).rev() {
                        let current = self.u8(i, HOME_TOWN) as usize;
                        if quota[town] > quota[current] {
                            self.set_u8(i, HOME_TOWN, town as u8);
                        }
                    }
                    let home = self.u8(i, HOME_TOWN) as usize;
                    quota[home] = quota[home].wrapping_sub(1);
                }
                i = next;
            }
        }
    }
}
