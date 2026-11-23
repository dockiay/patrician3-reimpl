//! Cola de obras de la IA (C06): `merchant_ai_queue_building_work` (0x004F5A20).

use super::housing::HousingRecord;

const BUILD_WORK: u32 = 0x28;
/// Palabras de la tabla estática `0x00672A18` para los índices -7..=78 de grupo (`(tipo - 0x15) / 3`).
const GROUP_BASE: i32 = -7;
pub const GROUP_TABLE: [u16; 86] = [
    2093, 2344, 11308, 8230, 771, 6151, 0, 80, 140, 280, 0, 20, 50, 80, 35, 100, 140, 70, 200, 280, 10, 20, 40, 274,
    141, 54, 0, 10292, 40, 12348, 48, 40, 0, 7700, 5121, 1290, 5125, 522, 532, 532, 5130, 2580, 257, 5122, 5140, 2570,
    522, 514, 12820, 5170, 5140, 5140, 532, 7681, 10290, 30, 2580, 5140, 5140, 1285, 513, 2, 0, 5140, 5121, 1290, 5125,
    517, 532, 532, 5130, 2580, 257, 5122, 5140, 2570, 522, 514, 12820, 5170, 5140, 5140, 532, 7681, 10290, 30,
];

/// Oficina del comerciante.
#[derive(Clone, Debug)]
pub struct WorkOffice {
    /// `+0x2C6`.
    pub town: u8,
    /// `+0x2C8`.
    pub next: u16,
    /// `+0x2CC`: primera entrada de la tabla de tareas de oficina.
    pub task_head: u16,
    /// `+0x2CE`: primera lista de edificios.
    pub list1: u16,
    /// Palabras desde `+0x2D0`: la primera es la segunda lista y `words[k + 7]` el valor del grupo `k` (`+0x2DE + 2k`).
    pub words: [u16; 86],
}

/// Entrada de la tabla de tareas de oficina (20 bytes, `0x005303B0`).
#[derive(Clone, Copy, Debug)]
pub struct WorkTask {
    /// `+4`.
    pub a: u16,
    /// `+6`: tipo de edificio.
    pub kind: u8,
    /// `+8`: siguiente.
    pub next: u16,
    /// `+0xC`.
    pub b: u16,
}

#[derive(Clone, Debug, Default)]
pub struct WorkTown {
    pub records: Vec<HousingRecord>,
    /// `+0x7AC`: ancho de la rejilla de celdas.
    pub stride: u32,
    /// Celdas propias de la ciudad (`+0x80C`, un puntero por ciudad en el original). Vacío: se usa `WorkEnv::cells`.
    pub cells: Vec<u8>,
}

pub struct WorkEnv<'a> {
    pub towns: &'a [WorkTown],
    /// Celdas (`+0x80C`, compartidas); el bit 7 marca una celda ocupada.
    pub cells: &'a [u8],
    pub tasks: &'a [WorkTask],
    pub offices: &'a [WorkOffice],
    /// `Merchant+0xC`.
    pub office_head: u16,
    /// `Merchant+0x468`: bit por ciudad con gremio.
    pub guild_mask: u32,
    /// Ranura de pila sin inicializar que lee la primera comparación (`best_a`); el original la deja con el residuo
    /// de la llamada anterior.
    pub residue: u32,
}

pub trait WorkHost {
    /// `0x004DAFE0`: `true` si el registro no tiene obra pendiente (salida -1).
    fn is_unclaimed(&mut self, town: u8, record: u16) -> bool;
    /// `0x004DAF10`: coste de construir (comparado sin signo).
    fn building_cost(&mut self, town: u8, record: u16, cell_marked: bool) -> u32;
    fn enqueue_operation(&mut self, record: &[u32; 5]);
}

fn word(office: &WorkOffice, group: i32) -> u16 {
    office.words[(group - GROUP_BASE) as usize]
}

fn group_of(kind: u8) -> i32 {
    // división con signo de `(tipo - 0x15) / 3` por la constante mágica del original
    (kind as i32 - 0x15) / 3
}

/// `merchant_ai_queue_building_work`. Devuelve `true` si encola una obra.
pub fn queue_building_work(task: u32, env: &WorkEnv, host: &mut dyn WorkHost) -> bool {
    let (mut best_cost, mut best_a, mut best_b) = (0u32, env.residue, 0u32);
    let (mut best_town, mut best_record) = (0xFFu8, 0u16);
    // `esi` conserva su valor entre candidatos: arranca con la tarea y lo pisan las listas.
    let mut esi = task;
    let empty = WorkTown::default();
    let mut office = env.offices.get(env.office_head as usize);
    while let Some(o) = office {
        if env.guild_mask & (1u32 << (o.town & 31)) != 0 {
            let town = env.towns.get(o.town as usize).unwrap_or(&empty);
            let cells = if town.cells.is_empty() { env.cells } else { town.cells.as_slice() };
            let bit = |rec: &HousingRecord| {
                cells
                    .get((town.stride.wrapping_mul(rec.x as u32).wrapping_add(rec.y as u32)) as usize)
                    .map_or(false, |c| c & 0x80 != 0)
            };
            let count = town.records.len() as u32;
            // primera lista: ratio con las tareas de la oficina
            let mut index = o.list1 as u32;
            while index < count {
                let rec = &town.records[index as usize];
                if host.is_unclaimed(o.town, index as u16) {
                    let cost = host.building_cost(o.town, index as u16, bit(rec));
                    let mut ebp = 0u32;
                    let mut t = o.task_head as u32;
                    while t < env.tasks.len() as u32 {
                        let entry = &env.tasks[t as usize];
                        if entry.kind == rec.kind {
                            esi = entry.a as u32;
                            ebp = entry.b as u32;
                            break;
                        }
                        t = entry.next as u32;
                    }
                    if esi.wrapping_mul(best_b) < best_a.wrapping_mul(ebp)
                        || (esi.wrapping_mul(best_b) == best_a.wrapping_mul(ebp) && cost < best_cost)
                    {
                        (best_record, best_a, best_b, best_cost, best_town) = (index as u16, esi, ebp, cost, o.town);
                    }
                }
                index = rec.next as u32;
            }
            // segunda lista: ratio con la tabla de grupos
            let mut index = o.words[0] as u32;
            while index < count {
                let rec = &town.records[index as usize];
                if host.is_unclaimed(o.town, index as u16) {
                    let cost = host.building_cost(o.town, index as u16, bit(rec));
                    let group = group_of(rec.kind);
                    esi = word(o, group) as u32;
                    let table = GROUP_TABLE[(group - GROUP_BASE) as usize] as u32;
                    if esi.wrapping_mul(best_b) < best_a.wrapping_mul(table)
                        || (esi.wrapping_mul(best_b) == best_a.wrapping_mul(table) && cost < best_cost)
                    {
                        (best_record, best_a, best_b, best_cost, best_town) = (index as u16, esi, table, cost, o.town);
                    }
                }
                index = rec.next as u32;
            }
        }
        office = env.offices.get(o.next as usize);
    }
    if best_cost == 0 {
        return false;
    }
    let town = env.towns.get(best_town as usize).unwrap_or(&empty);
    let rec = town.records.get(best_record as usize).copied().unwrap_or_default();
    host.enqueue_operation(&[BUILD_WORK, best_town as u32, task & 0xFF, rec.x as u32, rec.y as u32]);
    true
}
