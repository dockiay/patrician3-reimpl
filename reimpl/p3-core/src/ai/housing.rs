//! Elección de vivienda de la IA (C06): `merchant_ai_choose_housing` (0x004F4B40).

use super::guild::GuildOffice;

const BUILD_HOUSING: u32 = 0x22;
/// Puntuación mínima (0x62) que debe superar un grupo para ser elegido.
const MIN_SCORE: i32 = 0x62;
/// Primer tipo de vivienda y recuento de tipos (0x15..0x1D en tres grupos de tres).
const FIRST_TYPE: u8 = 0x15;
const END_TYPE: u8 = 0x1E;
const NO_TOWN: u8 = 0xFF;

/// Registro de edificio de la ciudad (8 bytes): posición, tipo y siguiente.
#[derive(Clone, Copy, Debug, Default)]
pub struct HousingRecord {
    pub x: u8,
    pub y: u8,
    pub kind: u8,
    pub next: u16,
}

/// Lo que lee de cada ciudad.
#[derive(Clone, Debug, Default)]
pub struct HousingTown {
    /// `+0x75C` y `+0x776`: registros de edificios.
    pub records: Vec<HousingRecord>,
    /// `+0x762`: primer registro de la lista de edificios construidos.
    pub head: u16,
    /// `+0x764`: primer registro de la lista pública.
    pub head2: u16,
    /// `+0x774` (con signo): tamaño de la ciudad.
    pub size: i8,
    /// `+0x2D8`: demanda de los tres grupos.
    pub demand: [i32; 3],
    /// `+0x2F8`: divisor de los tres grupos.
    pub divisor: [u16; 3],
    /// `+0x800`/`+0x801`: posición de partida.
    pub start: (u8, u8),
    /// `+0x2C8` bit 4.
    pub flag_10: bool,
}

pub struct HousingEnv<'a> {
    /// `0x006DE4B0`.
    pub town_count: u32,
    /// `0x006DE52A`.
    pub flag_2a: bool,
    /// `0x00677929` (ciudad pequeña) y `0x0067792F` (ciudad con tamaño positivo).
    pub extra_small: u8,
    pub extra_large: u8,
    pub towns: &'a [HousingTown],
    pub offices: &'a [GuildOffice],
    /// `Merchant+0xC`.
    pub office_head: u16,
}

pub trait HousingHost {
    /// `find_or_create_office` (0x005308A0): devuelve `+0x2D0` de la oficina o `None` si no hay.
    fn find_or_create_office(&mut self, task: u32, town: u8) -> Option<u16>;
    /// `town_plan_find_nearest_cell_for_type` (0x004F9C80) sobre el plano de la ciudad.
    fn find_nearest_cell(&mut self, town: u8, x: &mut u8, y: &mut u8, kind: u8, extra: u8, flag: bool) -> bool;
    fn enqueue_operation(&mut self, record: &[u32; 5]);
}

fn walk(town: &HousingTown, start: u16, mut visit: impl FnMut(&HousingRecord) -> bool) {
    let count = town.records.len() as u32;
    let mut index = start as u32;
    while index < count {
        let rec = &town.records[index as usize];
        index = rec.next as u32;
        if visit(rec) {
            index = 0xFFFF;
        }
        if index >= count {
            break;
        }
    }
}

/// `merchant_ai_choose_housing`: encola la operación 0x22 con la ciudad, el tipo y la celda elegidos.
pub fn choose_housing(task: u32, env: &HousingEnv, host: &mut dyn HousingHost) {
    if env.flag_2a {
        return;
    }
    let empty = HousingTown::default();
    let town_of = |t: u8| env.towns.get(t as usize).unwrap_or(&empty);
    let mut best = MIN_SCORE;
    let mut kind = NO_TOWN;
    let mut chosen = NO_TOWN;
    let mut office = env.offices.get(env.office_head as usize).copied();
    while let Some(o) = office {
        let town = town_of(o.town);
        let mut score = [100i32; 3];
        walk(town, town.head, |rec| {
            if rec.kind >= FIRST_TYPE && rec.kind < END_TYPE {
                score[((rec.kind - FIRST_TYPE) / 3) as usize] = 0;
            }
            false
        });
        for i in 0..3 {
            score[i] = score[i].wrapping_mul(town.demand[i]);
            if town.divisor[i] > 10 {
                score[i] /= town.divisor[i] as i32;
            }
            if best < score[i] {
                best = score[i];
                kind = (i as u8 + 7).wrapping_mul(3);
                chosen = o.town;
            }
        }
        office = env.offices.get(o.next as usize).copied();
    }
    if chosen as u32 >= env.town_count {
        return;
    }
    let town = town_of(chosen);
    let office_start = host.find_or_create_office(task, chosen);
    let extra = if town.size > 0 {
        env.extra_large
    } else {
        env.extra_small
    };
    let (mut x, mut y) = (0u8, 0u8);
    if let Some(start) = office_start {
        (x, y) = town.start;
        let near = |from: u16, x: &mut u8, y: &mut u8| {
            walk(town, from, |rec| {
                if rec.kind >= kind && rec.kind as i32 <= kind as i32 + 2 {
                    (*x, *y) = (rec.x, rec.y);
                    true
                } else {
                    false
                }
            });
        };
        near(start, &mut x, &mut y);
        if x == town.start.0 {
            near(town.head2, &mut x, &mut y);
        }
        if !host.find_nearest_cell(chosen, &mut x, &mut y, kind, extra, town.flag_10) {
            return;
        }
    }
    host.enqueue_operation(&[
        BUILD_HOUSING,
        (task & 0xFF) | (chosen as u32) << 8 | (x as u32) << 16 | (y as u32) << 24,
        kind as u32 | (extra as u32) << 8,
        0,
        0,
    ]);
}
