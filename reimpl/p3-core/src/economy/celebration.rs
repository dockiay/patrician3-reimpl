//! Preparación de la fiesta decenal `0x005316E0`.
use crate::{ai::reputation::Merchant, pump::Operation, rng::CrtRand, scheduler::Scheduler, town::Town};

/// Son campos transitorios del mundo: el serializador original no los escribe.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CelebrationState {
    pub next: u32,
    pub town: u8,
}

/// La preparación no altera las entradas ni el CRT cuando falta una dependencia.
pub fn prepare(
    state: CelebrationState,
    tick: u32,
    towns: &[Town],
    offices: &[u8],
    merchants: &[Merchant],
    player: u32,
    scheduler: &Scheduler,
    rng: Option<CrtRand>,
) -> Result<(CelebrationState, Vec<Operation>, Option<CrtRand>), &'static str> {
    if offices.len() % 0x44c != 0 {
        return Err("oficinas truncadas");
    }
    let count = offices.len() / 0x44c;
    if count > u16::MAX as usize {
        return Err("demasiadas oficinas");
    }
    let mut result = state;
    let mut operations = Vec::new();
    let mut random = rng;
    for city in 0..towns.len() {
        if city != usize::from(result.town) || tick <= result.next {
            continue;
        }
        let mut at = scheduler.head;
        let mut seen = vec![false; usize::from(scheduler.capacity)];
        let mut scheduled = false;
        while at < scheduler.capacity {
            let i = usize::from(at);
            if seen[i] {
                return Err("lista de tareas cíclica");
            }
            seen[i] = true;
            let task = scheduler.records.get(i).ok_or("planificador truncado")?;
            if task.kind == 7 && task.u32_at(0xc) == city as u32 && task.u32_at(0x10) == 2 {
                scheduled = true;
                break;
            }
            at = task.next;
        }
        if scheduled {
            result.next = tick.wrapping_add((u32::from(result.town) + 13) << 8);
            return Ok((result, operations, random));
        }
        let local = merchants.get(player as usize).ok_or("jugador local ausente")?;
        if local.bytes.len() < 10 {
            return Err("comerciante truncado");
        }
        let mut candidates = Vec::new();
        let mut index = usize::from(towns[city].u16(0x784));
        let mut office_seen = vec![false; count];
        while index < count {
            if office_seen[index] {
                return Err("lista de oficinas cíclica");
            }
            office_seen[index] = true;
            let office = &offices[index * 0x44c..(index + 1) * 0x44c];
            let owner = usize::from(u16::from_le_bytes([office[0x2c4], office[0x2c5]]));
            if let Some(m) = merchants.get(owner) {
                if m.bytes.len() < 10 {
                    return Err("comerciante truncado");
                }
                if m.u16(8) & 4 == 0 && owner != player as usize {
                    candidates.push(owner);
                    if candidates.len() > merchants.len() {
                        return Err("lista de oficinas desborda el original");
                    }
                }
            }
            index = usize::from(u16::from_le_bytes([office[0x2ca], office[0x2cb]]));
        }
        let mut chosen = player as u8;
        let mut skip = false;
        if local.u16(8) == 0 {
            let generator = random.as_mut().ok_or("estado CRT ausente")?;
            let sample = generator.rand() as usize;
            let mut pick = sample * candidates.len() / 32768;
            if candidates.is_empty() {
                skip = true
            } else if candidates.len() == 1 && merchants[candidates[0]].u16(8) == 0 {
                skip = true
            } else {
                if candidates.iter().all(|&i| merchants[i].u16(8) == 0) {
                    return Err("selección de fiesta entraría en bucle en el original");
                }
                let mut attempts = 0usize;
                loop {
                    chosen = candidates[pick] as u8;
                    if merchants[usize::from(chosen)].u16(8) != 0 {
                        break;
                    }
                    attempts += 1;
                    if attempts > 1_000_000 {
                        return Err("selección de fiesta no termina");
                    }
                    pick = (generator.rand() as usize) * candidates.len() / 32768;
                }
            }
        }
        if !skip {
            let population = towns[city].u32(0x2d4) as i32;
            let due = tick.wrapping_add(((population % 11 + 5) as u32).wrapping_shl(9));
            operations.push(Operation([0x61, u32::from(chosen), due, city as u32, 0]));
        }
        let generator = random.as_mut().ok_or("estado CRT ausente")?;
        result.town = ((generator.rand() as u32 * 20) / 32768) as u8;
        result.next = tick.wrapping_add((u32::from(result.town) + 13) << 8);
        if skip {
            return Ok((result, operations, random));
        }
    }
    Ok((result, operations, random))
}

/// `0x0053DBF0`: crea la tarea 7 y descuenta el coste, con comparación sin signo.
pub fn apply(op: Operation, merchants: &mut [Merchant], scheduler: &mut Scheduler) -> Result<(), &'static str> {
    if op.0[0] != 0x61 {
        return Err("operación de fiesta incorrecta");
    }
    let Some(merchant) = merchants.get_mut(op.0[1] as usize) else {
        return Ok(());
    };
    if merchant.bytes.len() < 0x4c0 {
        return Err("comerciante truncado");
    }
    let gold = merchant.i32(0) as u32;
    if gold < op.0[4] {
        return Ok(());
    }
    if scheduler.records.len() != usize::from(scheduler.capacity) {
        return Err("planificador truncado");
    }
    let mut at = scheduler.head;
    let mut seen = vec![false; usize::from(scheduler.capacity)];
    while at < scheduler.capacity {
        let i = usize::from(at);
        if seen[i] {
            return Err("lista de tareas cíclica");
        }
        seen[i] = true;
        at = scheduler.records[i].next;
    }
    if scheduler.free_head >= scheduler.capacity && scheduler.capacity > u16::MAX - 128 {
        return Err("capacidad de tareas agotada");
    }
    let mut staged = scheduler.clone();
    let id = staged.add_task(op.0[2]);
    let task = &mut staged.records[usize::from(id)];
    task.kind = 7;
    task.set_u32_at(8, op.0[1]);
    task.set_u32_at(0xc, op.0[3]);
    task.set_u32_at(0x10, 2);
    merchant.set_i32(0, gold.wrapping_sub(op.0[4]) as i32);
    merchant.set_i32(0x4bc, (merchant.i32(0x4bc) as u32).wrapping_sub(op.0[4]) as i32);
    *scheduler = staged;
    Ok(())
}

/// Rama sin oficina y rama de IA de `0x004E23A4`. Las fiestas humanas
/// necesitan consumo de mercancías, reputación y cartas, aún no conectados.
pub fn find_office(
    task: &crate::scheduler::TaskRecord,
    towns: &[Town],
    offices: &[u8],
    merchants: &[Merchant],
) -> Result<Option<usize>, &'static str> {
    if offices.len() % 0x44c != 0 {
        return Err("oficinas truncadas");
    }
    let owner = task.u32_at(8);
    let city = task.u32_at(0xc);
    let owner16 = usize::from(owner as u16);
    let city16 = usize::from(city as u16);
    if owner16 >= merchants.len() || city16 >= towns.len() {
        return Ok(None);
    }
    let count = offices.len() / 0x44c;
    let mut at = usize::from(towns[city16].u16(0x784));
    let mut seen = vec![false; count];
    let mut found = false;
    while at < count {
        if seen[at] {
            return Err("lista de oficinas cíclica");
        }
        seen[at] = true;
        let o = &offices[at * 0x44c..(at + 1) * 0x44c];
        if u16::from_le_bytes([o[0x2c4], o[0x2c5]]) == owner as u16 {
            found = true;
            break;
        }
        at = usize::from(u16::from_le_bytes([o[0x2ca], o[0x2cb]]));
    }
    Ok(if found { Some(at) } else { None })
}

pub fn finish_ai_task(
    task: &crate::scheduler::TaskRecord,
    towns: &mut [Town],
    offices: &[u8],
    merchants: &[Merchant],
    tick: u32,
) -> Result<(), &'static str> {
    if find_office(task, towns, offices, merchants)?.is_none() {
        return Ok(());
    }
    let owner = task.u32_at(8);
    let city = task.u32_at(0xc);
    let merchant = merchants
        .get(owner as usize)
        .ok_or("comerciante de fiesta fuera de rango")?;
    if merchant.bytes.len() < 10 {
        return Err("comerciante truncado");
    }
    if merchant.u16(8) == 0 {
        return Err("fiesta humana pendiente");
    }
    let town = towns.get_mut(city as usize).ok_or("ciudad de fiesta fuera de rango")?;
    town.set_u32(0x2d0, tick);
    Ok(())
}

/// Datos que la fiesta humana entrega a la carta 0x24 y a la presentación.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HumanCelebration {
    pub requested: u16,
    pub guests: u16,
    pub quality: u16,
    pub notify: bool,
}

/// Parte económica de `0x004E23A4`, una vez localizada la oficina humana.
pub fn finish_human(
    task: &crate::scheduler::TaskRecord,
    town: &mut Town,
    office: &mut crate::economy::trade::OfficeStock,
    merchant: &mut Merchant,
    tick: u32,
) -> Result<HumanCelebration, &'static str> {
    const PORTIONS: [i32; 8] = [3, 2, 2, 2, 0, 1, 0, 2];
    let city = usize::try_from(task.u32_at(0xc)).map_err(|_| "ciudad inválida")?;
    if city >= 40 || merchant.bytes.len() < 0x468 {
        return Err("reputación de fiesta ausente");
    }
    let rep = merchant.f32(0x2fc + city * 4);
    let value = if rep > 32758.0 {
        32767i16
    } else {
        crate::numeric::ftol_i32(rep as f64) as i16
    };
    let percentage = (i32::from(value) + 39).clamp(0, 99);
    let pop = town.i32(0x2d4);
    let initial = pop.wrapping_mul(percentage) / 100;
    let mut covered = 0i32;
    for i in 0..8 {
        if PORTIONS[i] > 0 && PORTIONS[i].wrapping_mul(initial) <= office.stock[i] {
            covered += 1
        }
    }
    let score = (covered.wrapping_mul(percentage) / 6).clamp(2, 99);
    let guests = (pop.wrapping_mul(score) / 100).max(5);
    let count = guests as u16;
    let mut next = office.clone();
    for i in 0..8 {
        let qty = PORTIONS[i] * i32::from(count);
        if qty <= 0 {
            continue;
        }
        let desired = qty * if town.u32(0x2c8) & 0x1000 != 0 { 4 } else { 2 };
        let lot = if crate::economy::prices::SMALL_LOT_WARES[i] {
            200
        } else {
            2000
        };
        let required = ((desired + lot - 1) / lot) * lot;
        next.stock[i] = if required <= next.stock[i] {
            next.stock[i].wrapping_sub(required)
        } else {
            0
        };
    }
    let mut person = merchant.clone();
    person.add_social_reputation_celebration(city as u32, (covered / 2) as u32);
    if covered > 1 {
        town.set_u32(0x2d0, tick)
    }
    *office = next;
    *merchant = person;
    Ok(HumanCelebration {
        requested: task.u32_at(0x10) as u16,
        guests: count,
        quality: (covered / 2) as u16,
        notify: covered >= 2 && task.u32_at(0x10) != 0,
    })
}
