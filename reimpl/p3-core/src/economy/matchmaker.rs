//! Oferta periódica del casamentero `0x004E8234`.
use crate::{
    ai::reputation::Merchant,
    calendar::MONTH_START,
    pump::Operation,
    scheduler::{Scheduler, TaskRecord},
    town::Town,
};

/// Carta temporal antes de su inserción en el buzón.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Offer {
    pub recipient: usize,
    pub record: [u8; 16],
}

/// `0x15`: solo las salidas sin boda son independientes de la fábrica naval.
pub fn wedding_is_inactive(task: &TaskRecord, tick: u32, merchants: &[Merchant]) -> Result<bool, String> {
    let owner = usize::from(task.u8_at(0x15));
    let Some(person) = merchants.get(owner) else {
        return Ok(true);
    };
    if person.bytes.len() < 0x2c {
        return Err("comerciante truncado".into());
    }
    Ok((person.i32(0x28) as u32) <= tick)
}

/// `0x004D6C90`: convierte una respuesta a la carta 0x21 en la operación
/// 0x72 y avanza cuatro muestras del CRT de forma transaccional.
pub fn answer_offer(
    letter: &[u8; 16],
    slot: u32,
    tick: u32,
    town_count: u32,
    player: u32,
    rng: &mut crate::rng::CrtRand,
) -> Result<Operation, String> {
    if letter[4] != 0x21 {
        return Err("la carta no es una oferta del casamentero".into());
    }
    if town_count == 0 {
        return Err("sin ciudades para la respuesta".into());
    }
    let mut random = *rng;
    let mut bytes = [0u8; 16];
    bytes[..2].copy_from_slice(&letter[12..14]);
    bytes[2..4].copy_from_slice(&(slot as u16).to_le_bytes());
    for i in 0..3 {
        bytes[4 + i * 2..6 + i * 2].copy_from_slice(&(random.rand() as u16).to_le_bytes())
    }
    bytes[10] = ((random.rand() as u32) % town_count) as u8;
    bytes[11] = player as u8;
    bytes[12] = letter[5];
    bytes[13] = letter[8];
    bytes[14] = letter[10];
    let year = u32::from(u16::from_le_bytes([letter[2], letter[3]]));
    let day = u32::from(u16::from_le_bytes([letter[14], letter[15]]));
    let deadline = year.wrapping_mul(365).wrapping_add(day).wrapping_shl(8) | 0xff;
    bytes[15] = u8::from(tick <= deadline);
    let mut words = [0u32; 5];
    words[0] = 0x72;
    for (i, chunk) in bytes.chunks_exact(4).enumerate() {
        words[i + 1] = u32::from_le_bytes(chunk.try_into().expect("longitud fija"))
    }
    *rng = random;
    Ok(Operation(words))
}

/// La respuesta al dossier `0x22` produce la operación de boda `0x60`.
pub fn answer_dossier(
    letter: &[u8; 16],
    tick: u32,
    town_count: u32,
    player: u32,
    rng: &mut crate::rng::CrtRand,
) -> Result<Operation, String> {
    if letter[4] != 0x22 {
        return Err("la carta no es un dossier del casamentero".into());
    }
    if town_count == 0 {
        return Err("sin ciudades para la respuesta".into());
    }
    let mut random = *rng;
    let due = tick.wrapping_add(((random.rand() as u32 & 7) + 14) << 8);
    let amount = ((random.rand() as u32 & 0x1fff) + 0x13f6) as u16;
    let variation = random.rand() as u8;
    let mut bytes = [0u8; 16];
    bytes[..4].copy_from_slice(&due.to_le_bytes());
    bytes[4..6].copy_from_slice(&amount.to_le_bytes());
    bytes[6] = variation;
    bytes[7] = letter[14];
    bytes[8] = letter[12];
    bytes[9] = letter[13];
    bytes[10] = (u32::from(letter[11]) % town_count) as u8;
    bytes[11] = letter[10];
    bytes[12] = (random.rand() as u8) & 3;
    bytes[13] = player as u8;
    let mut words = [0u32; 5];
    words[0] = 0x60;
    for (i, chunk) in bytes.chunks_exact(4).enumerate() {
        words[i + 1] = u32::from_le_bytes(chunk.try_into().expect("longitud fija"))
    }
    *rng = random;
    Ok(Operation(words))
}

/// Las cartas suministradas son las cabeceras de 16 bytes en orden de ranura.
/// Si falta una dependencia no modifica la tarea ni publica ninguna oferta.
pub fn prepare(
    task: &mut TaskRecord,
    tick: u32,
    year: u16,
    day_of_year: u16,
    towns: &[Town],
    merchants: &[Merchant],
    messages: &[[u8; 16]],
    first_count: u16,
    last_count: u16,
) -> Result<Vec<Offer>, String> {
    let mut staged = *task;
    let mut offers = Vec::new();
    for (index, merchant) in merchants.iter().enumerate() {
        if merchant.bytes.len() < 0x650 {
            return Err("comerciante truncado".into());
        }
        if usize::from(merchant.bytes[0x31]) < towns.len()
            || tick >= merchant.i32(0x2c) as u32
            || tick >= merchant.i32(0x28) as u32
        {
            continue;
        }
        let wealth = merchant.i32(0x46c) / 10000;
        if wealth < 1 {
            continue;
        }
        if towns.len() > 40 {
            return Err("demasiadas ciudades para reputación".into());
        }
        let reputation = merchant.calc_total_reputation_score(towns.len() as i32, towns) as i16 as i32;
        let mut score = wealth.min(30).wrapping_add(reputation.min(30));
        if score < 20 {
            continue;
        }
        let status = merchant.bytes[0x17] as i8;
        if status >= 20 {
            continue;
        }
        if status < 10 {
            score = score.wrapping_add(10)
        } else {
            score = score.wrapping_sub(i32::from(status))
        }
        // La ruta IA salta incondicionalmente en el ejecutable inglés.
        if merchant.u16(8) != 0 {
            continue;
        }
        if towns.len() < 10 {
            return Err("ciudades para umbral de casamentero ausentes".into());
        }
        let threshold = (towns[9].u32(0x304) & 15)
            + (towns[7].u32(0x31c) & 15)
            + (towns[3].u32(0x2e0) & 15)
            + (towns[0].u32(0x14) & 15);
        if score < (threshold as i32) {
            continue;
        }
        let mut at = usize::from(merchant.u16(0x0a));
        let mut seen = vec![false; messages.len()];
        while at < messages.len() {
            if seen[at] {
                return Err("cadena de cartas cíclica".into());
            }
            seen[at] = true;
            let letter = &messages[at];
            if letter[4] == 0x21 || letter[4] == 0x22 {
                let month = usize::from(letter[1] & 15);
                let start = u32::from(*MONTH_START.get(month).ok_or("mes de carta inválido")?);
                let y = u32::from(u16::from_le_bytes([letter[2], letter[3]]));
                let extra = if letter[4] == 0x21 {
                    42
                } else {
                    18 + u32::from(letter[15])
                };
                let deadline = (y
                    .wrapping_mul(365)
                    .wrapping_add(start)
                    .wrapping_add(u32::from(letter[0]))
                    .wrapping_add(extra))
                .wrapping_shl(8);
                if tick <= deadline {
                    staged.due = staged.due.wrapping_add(if letter[4] == 0x21 { 0x6400 } else { 0x5000 });
                    *task = staged;
                    return Ok(offers);
                }
                break;
            }
            at = usize::from(u16::from_le_bytes([letter[6], letter[7]]));
        }
        if first_count == 0 || last_count == 0 {
            return Err("banco de nombres de casamentero vacío".into());
        }
        let counter = staged.u8_at(0xe).wrapping_add(1);
        let counter = if counter > 8 { 1 } else { counter };
        staged.set_u8_at(0xe, counter);
        let seed0 = u16::from_le_bytes([staged.u8_at(8), staged.u8_at(9)]);
        let seed1 = u16::from_le_bytes([staged.u8_at(10), staged.u8_at(11)]);
        let seed2 = u16::from_le_bytes([staged.u8_at(12), staged.u8_at(13)]);
        let n = u32::from(counter);
        let mut record = [0u8; 16];
        record[4] = 0x21;
        record[5] = (u32::from(seed0) * n % towns.len() as u32) as u8;
        record[8..10].copy_from_slice(&((u32::from(seed1) * n % u32::from(first_count)) as u16).to_le_bytes());
        record[10..12].copy_from_slice(&((u32::from(seed2) * n % u32::from(last_count)) as u16).to_le_bytes());
        let price = ((((tick & 7) * u32::from(seed0)) % 6) + 3) * 1000;
        record[12..14].copy_from_slice(&(price as u16).to_le_bytes());
        record[14..16].copy_from_slice(&day_of_year.wrapping_add(14).to_le_bytes());
        offers.push(Offer {
            recipient: index,
            record,
        });
    }
    let year_start = u32::from(year) * 0x16d00;
    staged.due = if staged.due < year_start {
        year_start
    } else {
        staged.due.wrapping_add(0x5a00)
    };
    *task = staged;
    Ok(offers)
}

/// Construye el dossier `0x22` de la tarea `0x17`. El original omite el
/// destinatario inexistente; los divisores ausentes se rechazan sin mutaciones.
pub fn prepare_dossier(
    task: &TaskRecord,
    merchants: &[Merchant],
    town_count: usize,
    first_counts: (u16, u16),
    last_count: u16,
) -> Result<Option<Offer>, String> {
    const FEMALE: [u8; 15] = [5, 11, 13, 8, 9, 6, 7, 14, 0, 4, 10, 12, 1, 2, 3];
    const MALE: [u8; 15] = [26, 28, 17, 22, 22, 25, 27, 16, 23, 23, 15, 18, 20, 21, 21];
    let recipient = usize::from(task.u8_at(0x13));
    let Some(merchant) = merchants.get(recipient) else {
        return Ok(None);
    };
    let sex = *merchant.bytes.get(0x1a).ok_or("comerciante truncado")? != 0;
    let first_count = if sex { first_counts.1 } else { first_counts.0 };
    if town_count == 0 || first_count == 0 || last_count == 0 {
        return Err("divisor de dossier ausente".into());
    }
    let word = |offset| u16::from_le_bytes([task.u8_at(offset), task.u8_at(offset + 1)]);
    let value = u32::from(word(0x0c));
    let quality = value % 3;
    let portrait = (quality * 5 + (value / 3) % 5) as usize;
    let mut record = [0u8; 16];
    record[4] = 0x22;
    record[5] = task.u8_at(0x14);
    record[8] = task.u8_at(0x15);
    record[9] = task.u8_at(0x16);
    record[10] = quality as u8;
    record[11] = (usize::from(task.u8_at(0x12)) % town_count) as u8;
    record[12] = (word(0x0e) % first_count) as u8 | if sex { 0x80 } else { 0 };
    record[13] = (word(0x10) % last_count) as u8;
    record[14] = if sex { FEMALE[portrait] } else { MALE[portrait] };
    record[15] = 14;
    Ok(Some(Offer { recipient, record }))
}

/// Pago `0x72`: devuelve la carta tardía y la ranura de oferta que debe
/// invalidarse. La tarea y el pago se publican juntos tras validar entradas.
pub fn pay_fee(
    op: Operation,
    tick: u32,
    town_count: usize,
    distances: Option<&[u8]>,
    merchants: &mut [Merchant],
    scheduler: &mut Scheduler,
    messages: &[[u8; 16]],
) -> Result<(Option<Offer>, Option<usize>), String> {
    if op.0[0] != 0x72 {
        return Err("operación de casamentero inválida".into());
    }
    let mut payload = [0u8; 16];
    for (dst, word) in payload.chunks_exact_mut(4).zip(&op.0[1..]) {
        dst.copy_from_slice(&word.to_le_bytes())
    }
    let owner = usize::from(payload[11]);
    if owner >= merchants.len() {
        return Ok((None, None));
    }
    let mut person = merchants[owner].clone();
    if person.bytes.len() < 0x4c0 {
        return Err("comerciante truncado".into());
    }
    let fee = u32::from(u16::from_le_bytes([payload[0], payload[1]]));
    person.set_i32(0, (person.i32(0) as u32).wrapping_sub(fee) as i32);
    person.set_i32(0x4bc, (person.i32(0x4bc) as u32).wrapping_add(fee) as i32);
    let mut scheduler_next = scheduler.clone();
    let mut late = None;
    if payload[15] != 0 {
        let a = usize::from(person.bytes[0x19]);
        let b = usize::from(payload[12]);
        let distance = if a >= town_count || b >= town_count {
            255
        } else {
            let matrix = distances.ok_or("matriz de distancias ausente")?;
            *matrix
                .get(
                    a.checked_mul(town_count)
                        .and_then(|v| v.checked_add(b))
                        .ok_or("matriz inválida")?,
                )
                .ok_or("matriz truncada")?
        };
        if scheduler_next.records.len() != usize::from(scheduler_next.capacity) {
            return Err("planificador truncado".into());
        }
        let mut at = scheduler_next.head;
        let mut seen = vec![false; usize::from(scheduler_next.capacity)];
        while at < scheduler_next.capacity {
            let i = usize::from(at);
            if seen[i] {
                return Err("planificador cíclico".into());
            }
            seen[i] = true;
            at = scheduler_next.records[i].next
        }
        if scheduler_next.free_head >= scheduler_next.capacity && scheduler_next.capacity > u16::MAX - 128 {
            return Err("capacidad de tareas agotada".into());
        }
        let due = tick.wrapping_add((u32::from(distance) / 8 + 2).min(8) << 8);
        let id = scheduler_next.add_task(due);
        let task = &mut scheduler_next.records[usize::from(id)];
        task.kind = 0x17;
        task.payload = payload;
    } else {
        let mut record = [0u8; 16];
        record[4] = 0x2b;
        record[5] = payload[12];
        record[8..10].copy_from_slice(&u16::from(payload[13]).to_le_bytes());
        record[10..12].copy_from_slice(&u16::from(payload[14]).to_le_bytes());
        record[12..14].copy_from_slice(&(fee as u16).to_le_bytes());
        late = Some(Offer {
            recipient: owner,
            record,
        });
    }
    let index = usize::from(u16::from_le_bytes([payload[2], payload[3]]));
    let clear = messages.get(index).filter(|m| m[4] == 0x21).map(|_| index);
    merchants[owner] = person;
    *scheduler = scheduler_next;
    Ok((late, clear))
}

/// Alta `0x60`; las modificaciones del buzón se devuelven al llamante.
pub fn schedule_wedding(
    op: Operation,
    tick: u32,
    merchants: &[Merchant],
    scheduler: &mut Scheduler,
    messages: &[[u8; 16]],
) -> Result<(Option<Offer>, Vec<(usize, u8)>), String> {
    if op.0[0] != 0x60 {
        return Err("operación de boda inválida".into());
    }
    if op.0[1] <= tick {
        return Ok((None, Vec::new()));
    }
    let mut payload = [0u8; 16];
    for (dst, word) in payload.chunks_exact_mut(4).zip(&op.0[1..]) {
        dst.copy_from_slice(&word.to_le_bytes())
    }
    payload[0] = 0;
    let due = u32::from_le_bytes(payload[..4].try_into().unwrap());
    let owner = usize::from(payload[13]);
    let mut clear = Vec::new();
    let mut offer = None;
    if let Some(person) = merchants.get(owner) {
        if person.bytes.len() < 0x1a {
            return Err("comerciante truncado".into());
        }
        let mut at = usize::from(person.u16(10));
        let mut seen = vec![false; messages.len()];
        while at < messages.len() {
            if seen[at] {
                return Err("cadena de cartas cíclica".into());
            }
            seen[at] = true;
            let letter = &messages[at];
            match letter[4] {
                0x21 => clear.push((at, 2)),
                0x22 => clear.push((at, 1)),
                _ => {}
            }
            at = usize::from(u16::from_le_bytes([letter[6], letter[7]]));
        }
        // ticks_to_date usa desplazamiento aritmético y división con signo;
        // para ticks con el bit alto activo accede al byte anterior a la tabla.
        let days = (due as i32) >> 8;
        let year = (days / 365) as u16;
        let remainder = days.wrapping_sub(i32::from(year) * 365);
        let (month, day) = if remainder < 0 {
            (255, remainder.wrapping_sub(24) as u8)
        } else {
            let mut m = 0usize;
            while m < 12 && i32::from(MONTH_START[m]) <= remainder {
                m += 1
            }
            if m == 0 || m > 12 {
                return Err("fecha de boda fuera de tabla".into());
            }
            ((m - 1) as u8, (remainder - i32::from(MONTH_START[m - 1]) + 1) as u8)
        };
        let mut record = [0u8; 16];
        record[4] = 0x33;
        record[5] = payload[10];
        record[8..10].copy_from_slice(&u16::from(person.bytes[0x19]).to_le_bytes());
        record[10] = payload[9];
        record[11] = payload[8];
        record[12] = month;
        record[13] = day;
        record[14..16].copy_from_slice(&year.to_le_bytes());
        offer = Some(Offer {
            recipient: owner,
            record,
        });
    }
    let mut next = scheduler.clone();
    if next.records.len() != usize::from(next.capacity) {
        return Err("planificador truncado".into());
    }
    let mut at = next.head;
    let mut seen = vec![false; usize::from(next.capacity)];
    while at < next.capacity {
        let i = usize::from(at);
        if seen[i] {
            return Err("planificador cíclico".into());
        }
        seen[i] = true;
        at = next.records[i].next
    }
    if next.free_head >= next.capacity && next.capacity > u16::MAX - 128 {
        return Err("capacidad de tareas agotada".into());
    }
    let id = next.add_task(due);
    let task = &mut next.records[usize::from(id)];
    task.kind = 0x15;
    task.payload[..14].copy_from_slice(&payload[..14]);
    *scheduler = next;
    Ok((offer, clear))
}
