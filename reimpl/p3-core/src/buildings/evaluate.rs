//! `town_evaluate_build_request` (0x0051F3A0): valora una petición de obra (C05, parte C7).
//!
//! Rellena el registro de petición con los materiales que pide el tipo, los que ya hay en los barcos del
//! comerciante que están en puerto y en su oficina, lo que falta, el coste de comprarlo, el plazo y el
//! motivo por el que no se puede construir. Las consultas a otros sistemas (comerciante, oficina, precio)
//! pasan por `EvalHost`; la lista de barcos y los datos del comerciante llegan en `EvalEnv`.

use super::tables::{BASE_COST, BRICK, EXTRA_COST, IRON, RECORD_BYTE_6, TIMBER};
use super::Site;
use crate::numeric::ftol_i32;
use crate::town::{off, Town};

/// Mercancías que consume una obra: ladrillo, madera, hierro y la del extra (tabla `0x00673720`).
const WARES: [u32; 4] = [19, 11, 12, 17];
/// Unidades por tonel de cada una (`0x00672C14`): 2000 u. o 200 u. según la mercancía.
const DIVISOR: [i32; 4] = [2000, 2000, 200, 2000];
/// `f32` almacenados en `0x00673A18` para esas mercancías (precio de referencia por unidad).
const FACTOR: [f32; 4] = [
    f32::from_bits(0x3D23_6E30),
    f32::from_bits(0x3CE1_47AF),
    f32::from_bits(0x3FA3_9581),
    f32::from_bits(0x3E61_47AF),
];

/// Registro de petición (0x44 bytes en el original).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BuildRequest {
    /// +0x00: cantidad necesaria de cada material.
    pub need: [i32; 4],
    /// +0x10: cantidad disponible (barcos y oficina).
    pub have: [i32; 4],
    /// +0x20: lo que falta.
    pub short: [i32; 4],
    /// +0x30: coste de lo que se compra.
    pub cost: i32,
    /// +0x34: coste total (obra más materiales).
    pub total: i32,
    /// +0x38: plazo.
    pub time: i32,
    /// +0x3C: tipo de edificio.
    pub ty: u16,
    /// +0x3E: comerciante.
    pub merchant: u16,
    /// +0x40: ¿puede pagarlo?
    pub affordable: u8,
    /// +0x41: ¿hay materiales en la ciudad para lo que falta?
    pub materials: u8,
    /// +0x42: ¿tiene oficina (o es el alcalde)?
    pub office: u8,
    /// +0x43: motivo de bloqueo (0 ninguno).
    pub block: u8,
}

impl BuildRequest {
    /// Los 0x44 bytes del registro.
    pub fn to_bytes(&self) -> [u8; 0x44] {
        let mut b = [0u8; 0x44];
        for k in 0..4 {
            b[4 * k..4 * k + 4].copy_from_slice(&self.need[k].to_le_bytes());
            b[0x10 + 4 * k..0x14 + 4 * k].copy_from_slice(&self.have[k].to_le_bytes());
            b[0x20 + 4 * k..0x24 + 4 * k].copy_from_slice(&self.short[k].to_le_bytes());
        }
        b[0x30..0x34].copy_from_slice(&self.cost.to_le_bytes());
        b[0x34..0x38].copy_from_slice(&self.total.to_le_bytes());
        b[0x38..0x3C].copy_from_slice(&self.time.to_le_bytes());
        b[0x3C..0x3E].copy_from_slice(&self.ty.to_le_bytes());
        b[0x3E..0x40].copy_from_slice(&self.merchant.to_le_bytes());
        b[0x40] = self.affordable;
        b[0x41] = self.materials;
        b[0x42] = self.office;
        b[0x43] = self.block;
        b
    }
}

/// Un barco de la lista global (`0x006DD7A4`, 0x180 bytes cada uno).
#[derive(Clone, Copy, Debug)]
pub struct EvalShip {
    /// +0x39: ciudad en la que está.
    pub town: u8,
    /// +0x04: siguiente barco del comerciante.
    pub next: u16,
    /// +0x134: estado (0–3 en puerto, salvo 2).
    pub state: u16,
    /// Mercancías 19, 11, 12 y 17 (`+0x54 + 4·mercancía`).
    pub wares: [i32; 4],
}

/// Consultas a otros sistemas.
pub trait EvalHost {
    /// `get_merchant` (0x005303C0).
    fn get_merchant(&mut self, merchant: u32);
    /// `find_or_create_office` (0x005308A0): existencias de las cuatro mercancías de la oficina, si hay.
    fn find_office(&mut self, merchant: u32, town: u32, flag: u32) -> Option<[i32; 4]>;
    /// `get_buy_price` (0x0052E430).
    fn buy_price(&mut self, ware: u32, owner: u32, qty: u32) -> i32;
}

/// Entorno del que lee la valoración.
pub struct EvalEnv<'a> {
    /// Número de comerciantes (`0x006DE4AA`).
    pub merchant_count: u16,
    /// Número de barcos (`0x006DD894`).
    pub ship_count: u16,
    /// Primer barco del comerciante (`Merchant+0xE`).
    pub ship_head: u16,
    /// Dinero del comerciante (`Merchant+0`).
    pub merchant_gold: i32,
    /// `+0x6F1` (alcalde) de la ciudad de la tabla de ciudades con este mismo índice.
    pub mayor: u8,
    pub ships: &'a [EvalShip],
    pub host: &'a mut dyn EvalHost,
}

fn needs_flag(t: &Town, req: &mut BuildRequest, mask: u32) {
    if t.more_flags() & mask != 0 {
        req.block = 1;
    }
}

impl Site {
    /// Valora la petición `req` (con `ty` y `merchant` ya puestos). Devuelve si se puede construir. Con
    /// un comerciante o tipo fuera de rango devuelve `false` sin tocar el registro.
    pub fn evaluate_build_request(t: &Town, req: &mut BuildRequest, env: &mut EvalEnv) -> bool {
        if req.merchant >= env.merchant_count || req.ty >= 0x39 {
            return false;
        }
        let ty = req.ty as usize;
        let town = t.u8(off::TOWN_INDEX) as u32;
        let extra_flag = t.u32(off::FIELD_2CC);
        req.block = 0;
        req.cost = 0;
        req.need = [
            BRICK[ty] as i32,
            TIMBER[ty] as i32,
            IRON[ty] as i32,
            EXTRA_COST[ty] as i32,
        ];
        if ty == 5 && extra_flag == 0 {
            req.need[3] >>= 1;
        }
        req.have = [0; 4];

        // Barcos del comerciante en puerto.
        env.host.get_merchant(req.merchant as u32);
        let mut ship = env.ship_head;
        while ship < env.ship_count {
            let s = &env.ships[ship as usize];
            if s.town == t.u8(off::TOWN_INDEX) && s.state < 4 && s.state != 2 {
                for k in 0..4 {
                    req.have[k] = req.have[k].wrapping_add(s.wares[k] / DIVISOR[k]);
                }
            }
            ship = s.next;
        }

        // Oficina (o, sin ella, ser el alcalde o el tipo 0).
        let office = env.host.find_office(req.merchant as u32, town, 0);
        let has_office = office.is_some();
        match office {
            Some(stock) => {
                req.office = 1;
                for k in 0..4 {
                    req.have[k] = req.have[k].wrapping_add(stock[k] / DIVISOR[k]);
                }
            }
            None => req.office = (req.ty == 0 || env.mayor as u16 == req.merchant) as u8,
        }

        for k in 0..4 {
            req.short[k] = if req.need[k] > req.have[k] {
                req.need[k] - req.have[k]
            } else {
                0
            };
        }

        // Lo que cuesta lo que ya hay (las obras del plan de ciudad, tipos 0x30 en adelante).
        if req.ty >= 0x30 {
            for k in 0..4 {
                let h = req.have[k];
                if h > 0 {
                    let units = if req.need[k] < h { req.need[k] } else { h };
                    let qty = DIVISOR[k].wrapping_mul(units);
                    req.cost = req.cost.wrapping_add(ftol_i32((qty as f32) as f64 * FACTOR[k] as f64));
                }
            }
        }

        // Lo que falta se compra en la ciudad.
        req.materials = 1;
        for k in 0..4 {
            if req.short[k] > 0 {
                let qty = req.short[k].wrapping_mul(DIVISOR[k]);
                let price = env.host.buy_price(WARES[k], town, qty as u32);
                req.cost = req.cost.wrapping_add(price);
                let stock = t.stock(WARES[k] as usize);
                if qty > stock {
                    req.materials = 0;
                    req.short[k] = stock / DIVISOR[k];
                }
            }
        }

        let base = BASE_COST[ty] as i32;
        let work = if ty == 5 && extra_flag == 0 {
            base * 60
        } else {
            base * 100
        };
        req.total = work.wrapping_add(req.cost);
        if req.ty >= 0x30 {
            req.affordable = 1;
            if t.i32(off::MONEY) < req.total {
                req.block = 4;
            }
        } else {
            env.host.get_merchant(req.merchant as u32);
            req.affordable = (env.merchant_gold >= req.total) as u8;
        }
        req.time = (RECORD_BYTE_6[ty] as i32 + 4) / 5;

        if ty <= 0x2C {
            Self::block_reason(t, req, has_office);
        }
        req.affordable != 0 && req.materials != 0 && req.office != 0 && req.block == 0
    }

    /// El `switch` por tipo de la valoración: deja en `req.block` el motivo de bloqueo.
    fn block_reason(t: &Town, req: &mut BuildRequest, has_office: bool) {
        let citizens = t.i32(off::CITIZENS_TOTAL);
        let valid = |n| Self::is_plan_point_valid(t, n);
        let wall_room = || t.u16(off::WALL_HEAD) < t.u16(off::BUILDING_RECORD_COUNT);
        match req.ty {
            0 => {
                if has_office {
                    req.block = 1;
                }
            }
            1 => needs_flag(t, req, 0x40),
            2 => {
                if t.u16(off::STREETS_BUILT) >= t.u16(off::STREETS_TOTAL) {
                    req.block = 5;
                }
            }
            3 => needs_flag(t, req, 0x4),
            31 => needs_flag(t, req, 0x200),
            32 => needs_flag(t, req, 0x2),
            33 => needs_flag(t, req, 0x8),
            34 => needs_flag(t, req, 0x2000),
            35 => needs_flag(t, req, 0x1),
            36 => needs_flag(t, req, 0x20),
            37 => needs_flag(t, req, 0x800),
            38 => needs_flag(t, req, 0x4000),
            42 => needs_flag(t, req, 0x400),
            43 => needs_flag(t, req, 0x1000),
            40 => {
                if 500 * t.u8(off::WELLS) as i32 > citizens {
                    req.block = 6;
                }
            }
            41 | 44 => {
                // Hospital (41) o capilla (44): ¿cabe el punto de arranque, hay sitio en la muralla y
                // lo permite la población?
                if valid(1) || (valid(2) && wall_room()) || {
                    if req.ty == 41 {
                        citizens / 10000 < t.u8(0x772) as i32
                    } else {
                        citizens.wrapping_add(5000) / 10000 <= t.u8(0x770) as i32
                    }
                } {
                    req.block = 3;
                }
                if req.block == 0 || req.block == 3 {
                    if valid(1) || (valid(2) && wall_room()) {
                        req.block = 2;
                    }
                }
            }
            _ => {}
        }
    }
}
