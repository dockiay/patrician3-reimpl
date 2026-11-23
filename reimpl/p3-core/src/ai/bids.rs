//! Factor base de reputación y puja máxima de la IA por un barco (C06): `merchant_update_base_rep_factor`
//! (0x004F42B0) y `merchant_get_ship_auction_bid_limit` (0x004F4400).

use super::guild::GuildOffice;

fn lo_hi(scale: u8) -> (f32, f32) {
    // f2 = f32(escala · 0,05); tope inferior = f2 + 0,7 y superior = f2 + 1,2 (f32 tras cada `fstp`)
    let f2 = (scale as f32 as f64 * f32::from_bits(0x3D4C_CCCD) as f64) as f32;
    let hi = (f2 as f64 + f32::from_bits(0x3F99_999A) as f64) as f32;
    let lo = (f2 as f64 + f32::from_bits(0x3F33_3333) as f64) as f32;
    (lo, hi)
}

/// Lo que consulta el factor base.
pub struct BaseFactorEnv<'a> {
    /// Byte `0x006DE52C`: escala de dificultad.
    pub scale: u8,
    /// Dword `0x006DF2F0`: con alguna de las marcas 0xE se mira a todos los humanos; sin ellas, al jugador local.
    pub mode: u32,
    /// `0x006DFC14`: jugador local.
    pub local: usize,
    /// Word `0x006DE4AA`: comerciantes.
    pub merchant_count: u16,
    /// Palabra de control `+8` de cada comerciante (0: humano).
    pub controls: &'a [u16],
}

pub trait ScoreHost {
    /// `0x004F73D0`: puntuación (i16) del comerciante.
    fn merchant_score(&mut self, merchant: usize) -> i16;
}

/// `merchant_update_base_rep_factor`: devuelve el nuevo `Merchant+0x464` (f32).
pub fn update_base_rep_factor(this: usize, env: &BaseFactorEnv, host: &mut dyn ScoreHost) -> f32 {
    let (lo, hi) = lo_hi(env.scale);
    let best: i32 = if env.mode & 0xE == 0 {
        host.merchant_score(env.local) as i32
    } else {
        let mut best = 0i32;
        for index in 0..env.merchant_count as usize {
            if env.controls.get(index).copied().unwrap_or(0) == 0 {
                let score = host.merchant_score(index) as i32;
                if score > best {
                    best = score;
                }
            }
        }
        best
    };
    let own = host.merchant_score(this) as i32;
    if own <= best >> 1 {
        return hi;
    }
    let mut base = best as f32;
    if own > 1 {
        base = (base as f64 / own as f32 as f64) as f32;
    }
    if base > hi {
        hi
    } else if base < lo {
        lo
    } else {
        base
    }
}

/// Comerciante de IA que puja.
#[derive(Clone, Copy, Debug)]
pub struct BidMerchant {
    /// `+8`.
    pub control: u16,
    /// `+0x19`.
    pub home: u8,
    /// `+0xC`: primera oficina.
    pub first_office: u16,
    /// `+0x53C`.
    pub assets: i32,
    /// `+0x470`.
    pub limit: i32,
    /// `+0x546` y `+0x547`.
    pub b546: u8,
    pub b547: u8,
}

pub struct ShipBidEnv<'a> {
    /// `Town+0x2C8` de cada ciudad.
    pub town_flags: &'a [u32],
    /// `Ship+0xE` de cada barco.
    pub ship_kinds: &'a [u8],
    pub offices: &'a [GuildOffice],
    /// Word `0x006DE4A8`.
    pub office_count: u16,
}

pub trait ShipValueHost {
    /// `calc_ship_current_value` (0x0052A7D0) para el barco en la ciudad.
    fn ship_value(&mut self, ship: usize, town: usize) -> i32;
}

/// `merchant_get_ship_auction_bid_limit`.
pub fn ship_auction_bid_limit(
    m: &BidMerchant,
    env: &ShipBidEnv,
    ship: u32,
    town: u32,
    host: &mut dyn ShipValueHost,
) -> i32 {
    if m.control == 0 {
        return 0;
    }
    let ship = (ship & 0xFFFF) as usize;
    let home_flag = env.town_flags[m.home as usize] & 1 != 0;
    if (home_flag || m.control & 6 != 0) && env.ship_kinds[ship] & 2 == 2 {
        return 0;
    }
    if town as u16 != m.home as u16 {
        let mut index = m.first_office as u32;
        loop {
            if index >= env.office_count as u32 {
                return 0;
            }
            let office = env.offices[index as usize];
            index = office.next as u32;
            if office.town as u16 == town as u16 {
                break;
            }
        }
    }
    if m.assets <= 0 {
        return 0;
    }
    let percent = if m.limit <= 0 {
        0x4B0
    } else if m.assets > m.limit {
        let q = ((m.b546 as i32) * 2).wrapping_mul(m.assets) / m.limit;
        (q.wrapping_add(0x320)).min(0x4E2)
    } else {
        let q = (0x1F4 - m.b547 as i32).wrapping_mul(m.assets) / m.limit;
        q.wrapping_add(m.b546 as i32).wrapping_add(0x190)
    };
    host.ship_value(ship, (town & 0xFFFF) as usize).wrapping_mul(percent) >> 10
}

/// Tabla de clases por tipo de edificio (`0x00672C2C`, tipos 0..=0x1D): 1 y 2 piden existencias mínimas de la ciudad.
pub const BUILDING_CLASS: [u8; 30] = [
    6, 6, 6, 20, 9, 2, 3, 12, 5, 0, 1, 11, 8, 4, 16, 14, 7, 18, 19, 15, 17, 0, 0, 0, 255, 255, 255, 12, 12, 4,
];
/// Palabras de `0x00672A18`, `0x00672A1A` y `0x00672A1C`: peso de cada edificio del grupo en la lista de la ciudad.
const GROUP_WEIGHT: [u32; 3] = [0x50, 0x8C, 0x118];

/// Registro de edificio de la ciudad que devuelve `get_building_record` (`0x0051F1F0`).
#[derive(Clone, Copy, Debug, Default)]
pub struct BidBuilding {
    /// `+0` y `+1`: posición en la rejilla.
    pub x: u8,
    pub y: u8,
    /// `+2`: propietario.
    pub owner: u8,
    /// `+3`: tipo.
    pub kind: u8,
    /// `+4`: siguiente en la lista.
    pub next: u16,
    /// `+6`: marca (distinta de cero: no está en venta).
    pub flag: u8,
}

/// Oficina del mundo (`get_office`, `0x00530400`).
#[derive(Clone, Copy, Debug, Default)]
pub struct BidOffice {
    /// `+0x2C4`: comerciante dueño.
    pub merchant: u16,
    /// `+0x2C6`: ciudad.
    pub town: u8,
    /// `+0x2C8`: siguiente oficina del comerciante.
    pub next: u16,
    /// `+0x2CA`: siguiente oficina de la ciudad.
    pub next_in_town: u16,
    /// `+0x2CC`: primera tarea.
    pub task: u16,
}

/// Entrada de la tabla de tareas (`0x005303B0`, 20 bytes): `+4`, `+8` (siguiente) y `+0xC`.
#[derive(Clone, Copy, Debug, Default)]
pub struct BidTask {
    pub a: u16,
    pub next: u16,
    pub b: u16,
}

/// Comerciante que puja por un edificio.
#[derive(Clone, Copy, Debug)]
pub struct BuildingBidMerchant {
    /// `+0`: dinero.
    pub assets: i32,
    /// `+8`.
    pub control: u16,
    /// `+0xC`: primera oficina.
    pub first_office: u16,
    /// `+0x19`.
    pub home: u8,
}

/// Campos de la ciudad que consulta la puja.
pub struct BidTown<'a> {
    /// `+0x7AC`: ancho de la rejilla de celdas.
    pub width: i32,
    /// `+0x80C`: celdas; el bit 7 marca una celda ocupada. Fuera de la tabla valen cero.
    pub cells: &'a [u8],
    /// `+0x2D8`: demanda de los tres grupos.
    pub demand: [i32; 3],
    /// `+0x2F8`: divisor de los tres grupos.
    pub divisor: [u16; 3],
    /// `+0x762`: primer edificio de la lista y `+0x776`: número de registros.
    pub first: u16,
    pub count: u16,
    /// `+0x784`: primera oficina de la ciudad.
    pub first_office: u16,
    /// `+0x2CC`.
    pub reserve: u32,
    /// `+0x848 + 16·tipo`: existencias por tipo (27 tipos dentro de la ciudad).
    pub stock: &'a [u16],
}

pub struct BuildingBidEnv<'a> {
    pub town: BidTown<'a>,
    /// Oficinas del mundo y su número (word `0x006DE4A8`).
    pub offices: &'a [BidOffice],
    pub office_count: u16,
    /// Tareas y su número (word `0x006DE4A6`).
    pub tasks: &'a [BidTask],
    pub task_count: u16,
    /// Palabra `+8` de cada comerciante.
    pub merchants: &'a [u16],
}

pub trait BuildingBidHost {
    /// `get_building_record` (`0x0051F1F0`) de la ciudad de la puja.
    fn building_record(&mut self, index: u16) -> BidBuilding;
    /// `0x004DAF10`: valor de venta del edificio (argumentos completos de 32 bits y la marca de celda ocupada).
    fn sale_value(&mut self, town: u32, building: u32, occupied: u8) -> i32;
}

/// Iteraciones máximas de una lista; el original no las limita y se queda en bucle con listas cíclicas.
const LIST_LIMIT: u32 = 1 << 20;

/// `merchant_get_building_auction_bid_limit` (`0x004F4550`): `town` y `building` son los argumentos de 32 bits tal como llegan.
///
/// Límite: la rama logarítmica (`assets` distinto de cero y menor que diez veces el doble del valor) con `assets` o el doble del
/// valor no positivos entra en el manejador matemático del CRT (`_matherr`), que no se reproduce; aquí `ln` devuelve NaN y la suma queda en cero.
pub fn building_auction_bid_limit(
    m: &BuildingBidMerchant,
    env: &BuildingBidEnv,
    building: u32,
    town: u32,
    merchant: u8,
    host: &mut dyn BuildingBidHost,
) -> u32 {
    use crate::numeric::{ftol_i32, idiv, udiv};
    if m.control == 0 || m.control & 6 != 0 {
        return 0;
    }
    let rec = host.building_record((building & 0xFFFF) as u16);
    if rec.flag != 0 || rec.owner == merchant {
        return 0;
    }
    let town_byte = town as u8;
    if town_byte != m.home {
        let mut index = m.first_office as u32;
        let mut steps = 0;
        loop {
            if index >= env.office_count as u32 {
                return 0;
            }
            let office = env.offices.get(index as usize).copied().unwrap_or_default();
            index = office.next as u32;
            if office.town == town_byte {
                break;
            }
            steps += 1;
            if steps > LIST_LIMIT {
                return 0;
            }
        }
    }
    let t = &env.town;
    let cell = t
        .cells
        .get((t.width.wrapping_mul(rec.x as i32) as usize).wrapping_add(rec.y as usize))
        .copied()
        .unwrap_or(0);
    let value = host.sale_value(town, building, cell >> 7);
    let v2 = value.wrapping_mul(2);
    let assets = m.assets;
    let mut bid = v2;
    if assets >= v2.wrapping_mul(10) {
        bid = idiv(v2.wrapping_mul(4), 3);
    } else if assets != 0 {
        let ratio = assets as f64 / v2 as f64;
        let ln = if ratio > 0.0 { ratio.ln() } else { f64::NAN };
        bid = bid.wrapping_add(ftol_i32(ln * v2 as f64 * 0.33333333 + 0.5));
    }
    let bid = bid as u32;
    let kind = rec.kind as usize;
    let (mut num, mut den): (u32, u32);
    if (0x15..=0x1D).contains(&kind) {
        let group = (kind - 0x15) / 3;
        num = t.demand[group] as u32;
        den = t.divisor[group] as u32;
        let mut index = t.first as u32;
        let mut steps = 0;
        while index < t.count as u32 {
            let r = host.building_record(index as u16);
            if (0x15 + group * 3..=0x17 + group * 3).contains(&(r.kind as usize)) {
                den = den.wrapping_add(GROUP_WEIGHT[group]);
            }
            index = r.next as u32;
            steps += 1;
            if steps > LIST_LIMIT {
                break;
            }
        }
    } else {
        let class = BUILDING_CLASS.get(kind).copied().unwrap_or(0);
        let stock = t.stock.get(kind).copied().unwrap_or(0);
        let need = if class == 1 {
            if stock < 1000 {
                0xFF
            } else {
                0x0D
            }
        } else if class == 2 {
            if t.reserve < 1000 {
                0xFF
            } else {
                0x0A
            }
        } else {
            0xFF
        };
        if (stock < 1000 || class == 0xFF) && need == 0xFF {
            return 0;
        }
        num = 0;
        den = 0;
        let mut office = env
            .offices
            .get(t.first_office as usize)
            .filter(|_| t.first_office < env.office_count)
            .copied();
        let mut outer = 0;
        while let Some(o) = office {
            let owner_control = env.merchants.get(o.merchant as usize).copied().unwrap_or(0);
            if owner_control != 0 {
                let mut task = o.task as u32;
                let mut steps = 0;
                if task < env.task_count as u32 {
                    loop {
                        let e = env.tasks.get(task as usize).copied().unwrap_or_default();
                        num = num.wrapping_add(e.a as u32);
                        den = den.wrapping_add(e.b as u32);
                        task = e.next as u32;
                        steps += 1;
                        if task >= env.task_count as u32 || steps > LIST_LIMIT {
                            break;
                        }
                    }
                }
            }
            outer += 1;
            if outer > LIST_LIMIT {
                break;
            }
            let next = o.next_in_town;
            office = env
                .offices
                .get(next as usize)
                .filter(|_| (next as u32) < env.office_count as u32)
                .copied();
        }
    }
    if den == 0 {
        return bid;
    }
    udiv(num.wrapping_mul(bid), den)
}
