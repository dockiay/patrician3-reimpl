//! Orden y condiciones de la actualización decenal `0x004DDA40`.
use crate::{pump::Operation, scheduler::TaskRecord};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Event {
    ExpireMail(u16),
    RecountIncoming,
    CaptainEvents,
    AssignTownSlots,
    TavernPassenger,
    CriminalInvestigation,
    TownCelebration,
    Plague,
    Fire,
    YearAuctions,
    NotableDemand,
    WarehouseTheft,
}

#[derive(Clone, Copy, Debug)]
pub struct Context {
    pub day_of_year: u16,
    pub merchants: u16,
    pub towns: u32,
    pub network_client: bool,
    pub world_8a: u8,
    pub operations_mode: u32,
}

/// Cada llamada se ejecuta en su posición exacta; el anfitrión debe conectar sus efectos reales.
pub trait Host {
    type Error;
    fn event(&mut self, event: Event, task: &mut TaskRecord) -> Result<(), Self::Error>;
    fn enqueue(&mut self, operation: Operation) -> Result<(), Self::Error>;
    fn rand(&mut self) -> Result<u32, Self::Error>;
    fn captain_candidate(&mut self, merchant: u8) -> Result<bool, Self::Error>;
}

pub fn update<H: Host>(task: &mut TaskRecord, ctx: Context, host: &mut H) -> Result<(), H::Error> {
    if ctx.merchants != 0 {
        host.event(Event::ExpireMail(ctx.day_of_year % ctx.merchants), task)?;
    }
    let step = task.u32_at(8).wrapping_add(1);
    task.set_u32_at(8, if step > 2 { 0 } else { step });
    host.event(Event::RecountIncoming, task)?;
    host.event(Event::CaptainEvents, task)?;
    if ctx.network_client || u32::from(ctx.merchants) <= ctx.towns {
        return Ok(());
    }
    if ctx.world_8a != 1 {
        for event in [
            Event::AssignTownSlots,
            Event::TavernPassenger,
            Event::CriminalInvestigation,
            Event::TownCelebration,
        ] {
            host.event(event, task)?;
        }
    }
    match task.u32_at(8) {
        0 => host.event(Event::Plague, task)?,
        1 => host.event(Event::Fire, task)?,
        _ => {}
    }
    if ctx.day_of_year < 10 {
        host.event(Event::YearAuctions, task)?;
    }
    if ctx.operations_mode & 0xc != 0 && ctx.world_8a == 0 {
        host.enqueue(Operation([0xb8, 0, 0, 0, 0]))?;
    }
    if ctx.world_8a != 0 {
        return Ok(());
    }
    host.event(Event::NotableDemand, task)?;
    host.event(Event::WarehouseTheft, task)?;
    if ctx.merchants != 0 {
        let mut index = (host.rand()? % u32::from(ctx.merchants)) as u8;
        let mut attempts = host.rand()? & 7;
        loop {
            attempts += 1;
            if host.captain_candidate(index)? {
                break;
            }
            index = ((u32::from(index) + 1) % u32::from(ctx.merchants)) as u8;
            if attempts >= 10 {
                break;
            }
        }
    }
    Ok(())
}

/// `0x004DCD90`: avanza el candidato y, cuando procede, prepara una investigación 0x81.
/// La operación se despacha después; ante datos incompletos no cambia tarea ni CRT.
pub fn criminal_investigation(
    task: &mut TaskRecord,
    merchants: &[crate::ai::reputation::Merchant],
    count: u16,
    first_town: Option<&crate::town::Town>,
    rng: &mut Option<crate::rng::CrtRand>,
) -> Result<Option<Operation>, &'static str> {
    if count == 0 || usize::from(count) > merchants.len() {
        return Err("tabla de comerciantes incompleta");
    }
    let town = first_town.ok_or("primera ciudad ausente")?;
    let next = task.u32_at(0xc).wrapping_add(1 + (town.u32(0x40) & 1)) % u32::from(count);
    let mut record = *task;
    record.set_u32_at(0xc, next);
    let merchant = &merchants[next as usize];
    if merchant.bytes.len() < 0x470 {
        return Err("comerciante truncado");
    }
    let mut random = *rng;
    let result = if merchant.i32(0x46c) <= 100_000 {
        None
    } else {
        let generator = random.as_mut().ok_or("estado CRT ausente")?;
        let limit = (u32::from(count) * 205) / 36;
        if (generator.rand() as u32 & 0x3ff) > limit {
            None
        } else {
            let level = (generator.rand() as u32 & 3) + 5;
            let delay = (generator.rand() as u32 & 0x3ff) + 0x100;
            Some(Operation([0x81, next, level, delay, u32::from(merchant.bytes[0x19])]))
        }
    };
    *task = record;
    *rng = random;
    Ok(result)
}

/// `0x004DC6B0`: prepara la operación de pasajero 0x7A.
/// `ship_group_count` es el resultado de `0x00506290`; la matriz ausente
/// reproduce el valor 0xFF de `0x00531B70`. El CRT solo se publica si existe
/// al menos un destino: con lista vacía el original lee pila sin inicializar.
pub fn tavern_passenger(
    towns: &[crate::town::Town],
    distances: Option<&[u8]>,
    ship_group_count: u16,
    tick: u32,
    rng: &mut Option<crate::rng::CrtRand>,
) -> Result<Operation, &'static str> {
    let n = towns.len();
    if n == 0 || n > 40 {
        return Err("número de ciudades de pasajero inválido");
    }
    if distances.is_some_and(|d| d.len() != n * n) {
        return Err("matriz de distancias inválida");
    }
    let mut random = *rng;
    let generator = random.as_mut().ok_or("estado CRT ausente")?;
    let origin = generator.rand() as usize % n;
    let distance = |a: usize, b: usize| distances.map_or(0xff, |d| d[a * n + b]);
    let mut candidates = Vec::new();
    for city in (0..n).rev() {
        if distance(origin, city) <= 2 {
            continue;
        }
        candidates.push(city);
        let flags = towns[city].u32(0x2c8);
        if flags & 0xe00 != 0 {
            candidates.push(city)
        }
        if flags & 0x400 != 0 {
            candidates.push(city)
        }
    }
    if candidates.is_empty() {
        return Err("destino de pasajero indeterminado en el original");
    }
    let pick = ((generator.rand() as u32 & 0xff) * candidates.len() as u32) >> 8;
    let dest = candidates[pick as usize];
    let due = (tick | 0xff).wrapping_add(((generator.rand() as u32 & 3) + 11) << 8);
    let factor = (generator.rand() as u32 % 15) + 32;
    let base = (u32::from(distance(origin, dest)) << 8).wrapping_mul(factor) / 40;
    let price = (u32::from(ship_group_count) + 32).wrapping_mul(base) >> 5;
    *rng = random;
    Ok(Operation([0x7a, origin as u32, dest as u32, due, price]))
}

/// `0x00536570`, operación 0x7A: publica la oferta salvo que la taberna
/// esté reservada por un comerciante distinto de cero.
pub fn apply_tavern_passenger(towns: &mut [crate::town::Town], merchant_count: u16, op: Operation) -> Option<usize> {
    if op.0[0] != 0x7a {
        return None;
    }
    let city = usize::try_from(op.0[1]).ok()?;
    if city >= towns.len() || usize::try_from(op.0[2]).ok()? >= towns.len() {
        return None;
    }
    let town = &mut towns[city];
    let reserved = town.u8(0x83f);
    if reserved != 0 && u16::from(reserved) < merchant_count {
        return None;
    }
    town.set_u8(0x83f, 0xff);
    town.set_u32(0x838, op.0[3]);
    town.set_u16(0x836, op.0[4] as u16);
    town.set_u8(0x834, op.0[2] as u8);
    Some(city)
}

/// `0x00506290`: cuenta barcos en las cinco listas de bandas. El original
/// contaría dos veces una entrada compartida; solo se rechazan ciclos.
pub fn count_band_ships(heads: &[Option<u16>; 5], ships: &[u8]) -> Result<u16, &'static str> {
    if ships.len() % 0x180 != 0 || ships.len() / 0x180 > u16::MAX as usize {
        return Err("flota de bandas truncada");
    }
    let count = ships.len() / 0x180;
    let mut total = 0u16;
    for head in heads.iter().flatten() {
        let mut at = usize::from(*head);
        let mut seen = vec![false; count];
        while at < count {
            if seen[at] {
                return Err("lista de banda cíclica");
            }
            seen[at] = true;
            total = total.wrapping_add(1);
            let p = at * 0x180 + 4;
            at = usize::from(u16::from_le_bytes([ships[p], ships[p + 1]]));
        }
    }
    Ok(total)
}

/// `0x004DC800`: prepara hasta ocho cambios de plaza de taberna (0x79).
/// Regiones primaria y alternativa de la tabla fija 0x00673730.
pub fn assign_town_slots(
    towns: &[crate::town::Town],
    locations: &[crate::ai::settlement::SettlementLocation; 40],
    rng: &mut Option<crate::rng::CrtRand>,
) -> Result<Operation, &'static str> {
    const REGIONS: [[u8; 2]; 20] = [
        [4, 3],
        [0, 1],
        [0, 2],
        [0, 3],
        [0, 4],
        [1, 0],
        [1, 2],
        [1, 3],
        [1, 4],
        [2, 0],
        [2, 1],
        [2, 3],
        [2, 4],
        [3, 0],
        [3, 1],
        [3, 2],
        [3, 4],
        [4, 0],
        [4, 1],
        [4, 2],
    ];
    let n = towns.len();
    if n == 0 || n > 40 {
        return Err("número de ciudades de plazas inválido");
    }
    let mut regions = Vec::with_capacity(n);
    for t in towns {
        let id = usize::from(t.u8(0x2c1));
        regions.push(locations.get(id).ok_or("región de ciudad ausente")?.kind)
    }
    let mut random = *rng;
    let mut used = [false; 20];
    let mut selected = vec![false; n];
    let mut pairs = Vec::<(u8, u8)>::new();
    for city in (0..n).rev() {
        let slot = usize::from(towns[city].u8(0x82c));
        if slot < 20 {
            if used[slot] {
                if pairs.len() == 8 {
                    return Err("demasiadas plazas duplicadas");
                };
                pairs.push((city as u8, 0xff));
            } else {
                used[slot] = true
            }
            selected[city] = true;
        }
    }
    for slot in 0..20 {
        if used[slot] {
            continue;
        }
        if pairs.len() >= 8 {
            break;
        }
        let primary = REGIONS[slot][0];
        let alternative = REGIONS[slot][1];
        let city = if let Some(i) = (0..n).find(|&i| regions[i] == primary && !selected[i]) {
            i
        } else {
            let start = random.as_mut().ok_or("estado CRT ausente")?.rand() as usize % n;
            if let Some(i) = (start..n).find(|&i| regions[i] == primary) {
                i
            } else {
                let start = random.as_mut().ok_or("estado CRT ausente")?.rand() as usize % n;
                (0..n)
                    .map(|i| (start + i) % n)
                    .find(|&i| regions[i] != alternative)
                    .ok_or("selección de plaza entraría en bucle en el original")?
            }
        };
        selected[city] = true;
        used[slot] = true;
        pairs.push((city as u8, slot as u8));
    }
    let mut raw = [0xffu8; 16];
    for (i, (city, slot)) in pairs.into_iter().enumerate() {
        raw[i * 2] = city;
        raw[i * 2 + 1] = slot
    }
    let mut words = [0u32; 5];
    words[0] = 0x79;
    for i in 0..4 {
        words[i + 1] = u32::from_le_bytes(raw[i * 4..i * 4 + 4].try_into().unwrap())
    }
    *rng = random;
    Ok(Operation(words))
}

/// Despacho de 0x79: aplica los ocho pares en orden; las ciudades inválidas se ignoran.
pub fn apply_town_slots(towns: &mut [crate::town::Town], op: Operation) {
    if op.0[0] != 0x79 {
        return;
    }
    for word in &op.0[1..] {
        for pair in word.to_le_bytes().chunks_exact(2) {
            if let Some(town) = towns.get_mut(usize::from(pair[0])) {
                town.set_u8(0x82c, pair[1]);
            }
        }
    }
}

/// `0x004DCA90`: intenta generar una operación de peste 0x7C.
pub fn plague(
    towns: &[crate::town::Town],
    day_of_year: u16,
    tick: u32,
    rng: &mut Option<crate::rng::CrtRand>,
) -> Result<Option<Operation>, &'static str> {
    if towns.is_empty() || towns.len() > 256 {
        return Err("número de ciudades para peste inválido");
    }
    let mut random = *rng;
    let generator = random.as_mut().ok_or("estado CRT ausente")?;
    let city = generator.rand() as usize % towns.len();
    let town = &towns[city];
    if town.u32(0x2c8) & 8 != 0 {
        *rng = random;
        return Ok(None);
    }
    let roads = i32::from(town.u16(0x790)) * 100 < i32::from(town.u16(0x792)) * 75;
    let beds = (i32::from(town.u8(0x771)) + i32::from(town.u8(0x773))) * 50_000 < town.i32(0x2d4).wrapping_mul(8);
    let risk = i32::from(roads) + i32::from(beds);
    if generator.rand() % 100 >= risk * 2 + 1 {
        *rng = random;
        return Ok(None);
    }
    let delay = (generator.rand() as u32 & 3) + 1;
    let start = day_of_year.wrapping_add(delay as u16);
    let end = tick.wrapping_add(((generator.rand() as u32 % 10) + delay + (risk as u32) * 10) << 8);
    let mut name = [0u8; 4];
    for b in &mut name {
        *b = (generator.rand() as u8 & 31).wrapping_add(0x31)
    }
    let strength = (town.i32(0x2d4) / 40) as u16;
    *rng = random;
    Ok(Some(Operation([
        0x7c,
        u32::from_le_bytes(name),
        end,
        u32::from(strength) << 16,
        u32::from(start) | (city as u32) << 16,
    ])))
}

/// Operación 0x7C: programa la tarea de peste con los 16 bytes del mensaje.
pub fn apply_plague(
    op: Operation,
    scheduler: &mut crate::scheduler::Scheduler,
    day_of_year: u16,
    year: u16,
    tick: u32,
) -> Result<(), &'static str> {
    if op.0[0] != 0x7c {
        return Err("operación de peste incorrecta");
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
    let day = (op.0[4] & 0xffff) as u16;
    let due = if day < day_of_year {
        (tick | 0xff).wrapping_add(1)
    } else {
        (u32::from(year) * 365 + u32::from(day)) << 8
    };
    let mut next = scheduler.clone();
    let id = next.add_task(due);
    let task = &mut next.records[usize::from(id)];
    task.kind = 0x1c;
    for i in 0..4 {
        task.payload[i * 4..i * 4 + 4].copy_from_slice(&op.0[i + 1].to_le_bytes())
    }
    *scheduler = next;
    Ok(())
}

/// `0x004DCBF0`: intenta elegir un edificio y emitir un incendio 0x7D.
pub fn fire(
    towns: &[crate::town::Town],
    sites: &[crate::buildings::Site],
    tick: u32,
    rng: &mut Option<crate::rng::CrtRand>,
) -> Result<Option<Operation>, &'static str> {
    if towns.is_empty() || towns.len() > 256 {
        return Err("número de ciudades para incendio inválido");
    }
    let mut random = *rng;
    let generator = random.as_mut().ok_or("estado CRT ausente")?;
    let city = generator.rand() as usize % towns.len();
    let town = &towns[city];
    if town.u32(0x2c8) & 0x8000 != 0 {
        *rng = random;
        return Ok(None);
    }
    let high = i32::from(town.u8(0x789)) * 5000 < town.i32(0x2d4).wrapping_mul(8);
    let chance = if high { 5 } else { 1 };
    if generator.rand() % 100 >= chance {
        *rng = random;
        return Ok(None);
    }
    let strength = (town.i32(0x2d4) / if high { 800 } else { 300 }) as u16;
    let due = tick.wrapping_add(0x200 + (generator.rand() as u32 & 0xfff));
    let attempts = (generator.rand() as usize & 63) + 1;
    let site = sites.get(city).ok_or("plano de incendio ausente")?;
    let count = usize::from(town.u16(0x776)).min(attempts);
    if site.records.len() < count * 8 {
        return Err("registros de incendio truncados");
    }
    let mut target = None;
    for record in site.records.chunks_exact(8).take(count) {
        if record[3] < 0x1e
            && u32::from(record[0]) < town.u32(0x7a8)
            && u32::from(record[1]) < town.u32(0x7ac)
            && record[6] == 0
        {
            target = Some((record[0], record[1]))
        }
    }
    *rng = random;
    Ok(target.map(|(x, y)| {
        Operation([
            0x7d,
            due,
            u32::from(strength) << 16,
            u32::from(x) | (u32::from(y) << 8) | ((city as u32) << 16),
            0,
        ])
    }))
}

/// `0x7D`: programa el incendio en su fecha o inmediatamente si ya pasó.
pub fn apply_fire(op: Operation, scheduler: &mut crate::scheduler::Scheduler, tick: u32) -> Result<(), &'static str> {
    if op.0[0] != 0x7d {
        return Err("operación de incendio incorrecta");
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
    let mut next = scheduler.clone();
    let id = next.add_task(tick.max(op.0[1]));
    let task = &mut next.records[usize::from(id)];
    task.kind = 0x1d;
    for i in 0..4 {
        task.payload[i * 4..i * 4 + 4].copy_from_slice(&op.0[i + 1].to_le_bytes())
    }
    *scheduler = next;
    Ok(())
}

/// `0x004DD400`: genera las operaciones anuales de subasta de barcos.
/// Conserva el CRT si el planificador es cíclico o el original no terminaría.
pub fn year_auctions(
    scheduler: &crate::scheduler::Scheduler,
    towns: &[crate::town::Town],
    tick: u32,
    year: u16,
    rng: &mut Option<crate::rng::CrtRand>,
) -> Result<Vec<Operation>, &'static str> {
    let mut at = usize::from(scheduler.head);
    let capacity = usize::from(scheduler.capacity);
    if scheduler.records.len() < capacity {
        return Err("planificador de subastas truncado");
    };
    let mut seen = vec![false; capacity];
    let mut due = tick;
    while at < capacity {
        if seen[at] {
            return Err("lista de tareas de subasta cíclica");
        };
        seen[at] = true;
        let task = &scheduler.records[at];
        if task.kind == 0x0a {
            due = task.due
        }
        at = usize::from(task.next)
    }
    let limit = (u32::from(year) + 1).wrapping_mul(365).wrapping_shl(8);
    let mut random = *rng;
    let generator = random.as_mut().ok_or("estado CRT ausente")?;
    let mut operations = Vec::new();
    loop {
        let interval = 0x7b00 + (generator.rand() as u32 & 0x7fff);
        let next = due.wrapping_add(interval);
        if next < due {
            return Err("fecha de subasta desbordada");
        };
        due = next;
        if due >= limit {
            break;
        }
        if towns.is_empty() || !towns.iter().any(|t| t.u32(0x2c8) & 0xc0 != 0) {
            return Err("sin ciudades aptas para subasta");
        };
        let mut city = generator.rand() as usize % towns.len();
        while towns[city].u32(0x2c8) & 0xc0 == 0 {
            city = if city == 0 { towns.len() - 1 } else { city - 1 }
        }
        operations.push(Operation([0x38, due, city as u32, 0, 0]));
    }
    *rng = random;
    Ok(operations)
}

/// Operación 0x38: programa la subasta solo si faltan al menos 0x2800 ticks.
pub fn apply_year_auction(
    op: Operation,
    scheduler: &mut crate::scheduler::Scheduler,
    tick: u32,
    town_count: u32,
) -> Result<bool, &'static str> {
    if op.0[0] != 0x38 {
        return Err("operación de subasta incorrecta");
    };
    if op.0[1] < tick.wrapping_add(0x2800) {
        return Ok(false);
    }
    if town_count == 0 {
        return Err("subasta sin ciudades");
    }
    if scheduler.records.len() != usize::from(scheduler.capacity) {
        return Err("planificador truncado");
    };
    let mut at = scheduler.head;
    let mut seen = vec![false; usize::from(scheduler.capacity)];
    while at < scheduler.capacity {
        let i = usize::from(at);
        if seen[i] {
            return Err("lista de tareas cíclica");
        };
        seen[i] = true;
        at = scheduler.records[i].next;
    }
    if scheduler.free_head >= scheduler.capacity && scheduler.capacity > u16::MAX - 128 {
        return Err("capacidad de tareas agotada");
    };
    let mut next = scheduler.clone();
    let id = next.add_task(op.0[1]);
    let task = &mut next.records[usize::from(id)];
    task.kind = 0x0a;
    task.set_u8_at(0x14, 0);
    task.set_u8_at(0x15, (op.0[2] % town_count) as u8);
    *scheduler = next;
    Ok(true)
}

/// `0x004DD530`: prepara una petición de un notable al Ältermann.
/// Conserva el CRT ante referencias de comerciante fuera de la tabla.
pub fn notable_demand(
    towns: &[crate::town::Town],
    merchants: &[crate::ai::reputation::Merchant],
    alderman: u8,
    tick: u32,
    rng: &mut Option<crate::rng::CrtRand>,
) -> Result<Option<Operation>, &'static str> {
    if usize::from(alderman) >= merchants.len() {
        return Ok(None);
    };
    let boss = &merchants[usize::from(alderman)];
    if boss.bytes.len() < 10 {
        return Err("Ältermann truncado");
    };
    if boss.u16(8) != 0 || towns.len() < 17 {
        return Ok(None);
    }
    if towns.len() > u32::MAX as usize || merchants.len() > u16::MAX as usize {
        return Err("tablas de notable fuera de rango");
    };
    let mut random = *rng;
    let generator = random.as_mut().ok_or("estado CRT ausente")?;
    if generator.rand() & 3 != 0 {
        *rng = random;
        return Ok(None);
    }
    let mut op = [0u8; 20];
    op[..4].copy_from_slice(&0x84u32.to_le_bytes());
    let mut select = ((generator.rand() as usize % towns.len()).wrapping_sub(16) & 255) as i32;
    for (i, t) in towns.iter().enumerate() {
        if t.u32(0x2c8) & 0x40 == 0 && t.u32(0x76c) & 0x100 != 0 {
            op[8] = i as u8;
            select -= 1;
            if select < 0 {
                break;
            }
        }
    }
    op[9] = 5 + (generator.rand() as u8 & 1);
    let mut owner = generator.rand() as usize % towns.len();
    if owner == usize::from(op[8]) {
        owner += 1
    }
    let person = merchants.get(owner).ok_or("comerciante de petición ausente")?;
    if person.bytes.len() < 0x1e {
        return Err("comerciante de petición truncado");
    };
    op[10] = person.bytes[0x1c];
    op[11] = person.bytes[0x1d];
    let mut select = generator.rand() % 12;
    for (i, t) in towns.iter().enumerate() {
        if t.u32(0x2c8) & 0x40 == 0 {
            continue;
        }
        let candidate = usize::from(t.u8(0x6f1));
        if candidate > merchants.len() {
            let slot = (255usize.wrapping_sub(candidate)) & 3;
            op[15] = i as u8;
            op[13] = t.u8(0x6d4 + slot);
            op[14] = t.u8(0x6d8 + slot);
            op[16] = t.u8(0x6e0 + slot);
            op[17] = t.u8(0x6e4 + slot)
        } else {
            let m = merchants
                .get(candidate)
                .ok_or("notable fuera de la tabla de comerciantes")?;
            if m.bytes.len() < 0x650 {
                return Err("notable truncado");
            };
            if m.u16(8) == 0 {
                continue;
            }
            op[15] = i as u8;
            op[13] = m.bytes[0x1c];
            op[14] = m.bytes[0x1d];
            op[16] = m.get_merchant_epithet_index();
            op[17] = m.bytes[0x39c + usize::from(m.bytes[0x19])];
        }
        select -= 1;
        if select < 0 {
            break;
        }
    }
    if generator.rand() & 7 != 0 {
        *rng = random;
        return Ok(None);
    };
    let due = tick.wrapping_add(((generator.rand() as u32 & 7) + 5) << 8);
    op[4..8].copy_from_slice(&due.to_le_bytes());
    op[12] = 6;
    let mut words = [0u32; 5];
    for i in 0..5 {
        words[i] = u32::from_le_bytes(op[i * 4..i * 4 + 4].try_into().unwrap())
    }
    *rng = random;
    Ok(Some(Operation(words)))
}

/// `0x0053FCA0`, operación 0x84: programa una petición de la Hansa (0x30).
pub fn apply_notable_demand(
    op: Operation,
    scheduler: &mut crate::scheduler::Scheduler,
    tick: u32,
) -> Result<(), &'static str> {
    if op.0[0] != 0x84 {
        return Err("operación de petición incorrecta");
    };
    if scheduler.records.len() != usize::from(scheduler.capacity) {
        return Err("planificador truncado");
    };
    let mut at = scheduler.head;
    let mut seen = vec![false; usize::from(scheduler.capacity)];
    while at < scheduler.capacity {
        let i = usize::from(at);
        if seen[i] {
            return Err("lista de tareas cíclica");
        };
        seen[i] = true;
        at = scheduler.records[i].next;
    }
    if scheduler.free_head >= scheduler.capacity && scheduler.capacity > u16::MAX - 128 {
        return Err("capacidad de tareas agotada");
    };
    let mut bytes = [0u8; 20];
    for i in 0..5 {
        bytes[4 * i..4 * i + 4].copy_from_slice(&op.0[i].to_le_bytes())
    }
    let mut next = scheduler.clone();
    let id = next.add_task(tick.wrapping_add(0x100).max(op.0[1]));
    let task = &mut next.records[usize::from(id)];
    task.kind = 0x30;
    task.set_u32_at(8, tick);
    for i in 0..6 {
        task.set_u8_at(0xc + i, bytes[0xc + i])
    }
    for i in 0..4 {
        task.set_u8_at(0x12 + i, bytes[8 + i])
    }
    *scheduler = next;
    Ok(())
}

/// `0x004DD790`: prepara robos en oficinas de comerciantes humanos.
/// La aritmética entera conserva los desbordamientos observados en x86.
pub fn warehouse_theft(
    merchants: &[crate::ai::reputation::Merchant],
    offices: &[u8],
    rng: &mut Option<crate::rng::CrtRand>,
) -> Result<Vec<Operation>, &'static str> {
    if offices.len() % 0x44c != 0 || merchants.len() > u16::MAX as usize {
        return Err("tablas de robo truncadas");
    };
    let count = offices.len() / 0x44c;
    let mut random = *rng;
    let mut operations = Vec::new();
    const UNITS: [u8; 19] = [0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 0, 0, 1];
    for owner in (0..merchants.len()).rev() {
        let merchant = &merchants[owner];
        if merchant.bytes.len() < 14 {
            return Err("comerciante de robo truncado");
        };
        if merchant.u16(8) != 0 {
            continue;
        }
        let mut at = usize::from(merchant.u16(0xc));
        let mut seen = vec![false; count];
        while at < count {
            if seen[at] {
                return Err("lista de oficinas de robo cíclica");
            };
            seen[at] = true;
            let o = &offices[at * 0x44c..(at + 1) * 0x44c];
            let next = usize::from(u16::from_le_bytes([o[0x2c8], o[0x2c9]]));
            let capacity = i32::from(u16::from_le_bytes([o[0x2ec], o[0x2ed]]));
            let expected = ((capacity - 50) / 200 + 1) as i16 as i32;
            let level = i32::from(o[0x2c7]);
            if expected > level {
                let mut total = 0i32;
                for j in 0..20 {
                    total = total.wrapping_add(i32::from_le_bytes(o[4 + 4 * j..8 + 4 * j].try_into().unwrap()))
                }
                let excess = total / 2000 - 50;
                if excess > 0 {
                    let threshold = ((expected - 1).wrapping_mul(excess) / (level + 1))
                        .wrapping_sub(excess)
                        .wrapping_shl(7)
                        / 25;
                    let g = random.as_mut().ok_or("estado CRT ausente")?;
                    if (g.rand() & 0xfff) < threshold {
                        let mut op = [0u8; 20];
                        op[..4].copy_from_slice(&0x64u32.to_le_bytes());
                        op[4..10].fill(255);
                        op[10] = owner as u8;
                        op[11] = o[0x2c6];
                        let delay = ((g.rand() as u16) & 0x7ff) + 100;
                        op[18..20].copy_from_slice(&delay.to_le_bytes());
                        let mut slot = 0usize;
                        for j in 0..19 {
                            let amount = i32::from_le_bytes(o[4 + 4 * j..8 + 4 * j].try_into().unwrap());
                            if amount <= 0 {
                                continue;
                            }
                            let r = g.rand();
                            let factor = (r & 15) + ((r >> 4) & 7) + 4;
                            let product = factor.wrapping_mul(total).wrapping_mul(total) >> 8;
                            let quantity = (product / 10) / if UNITS[j] == 0 { 2000 } else { 200 };
                            if quantity > i32::from(op[12 + slot]) {
                                op[12 + slot] = quantity as u8;
                                op[4 + slot] = j as u8;
                                slot = (slot + 1) % 6
                            }
                        }
                        let mut words = [0u32; 5];
                        for j in 0..5 {
                            words[j] = u32::from_le_bytes(op[j * 4..j * 4 + 4].try_into().unwrap())
                        }
                        operations.push(Operation(words));
                    }
                }
            }
            at = next;
        }
    }
    *rng = random;
    Ok(operations)
}

/// Operación 0x64: crea la tarea 0x33 copiando literalmente los 16 bytes.
pub fn apply_warehouse_theft(
    op: Operation,
    scheduler: &mut crate::scheduler::Scheduler,
    tick: u32,
) -> Result<(), &'static str> {
    if op.0[0] != 0x64 {
        return Err("operación de robo incorrecta");
    };
    if scheduler.records.len() != usize::from(scheduler.capacity) {
        return Err("planificador truncado");
    };
    let mut at = scheduler.head;
    let mut seen = vec![false; usize::from(scheduler.capacity)];
    while at < scheduler.capacity {
        let i = usize::from(at);
        if seen[i] {
            return Err("lista de tareas cíclica");
        };
        seen[i] = true;
        at = scheduler.records[i].next;
    }
    if scheduler.free_head >= scheduler.capacity && scheduler.capacity > u16::MAX - 128 {
        return Err("capacidad de tareas agotada");
    };
    let mut data = [0u8; 16];
    for j in 0..4 {
        data[j * 4..j * 4 + 4].copy_from_slice(&op.0[j + 1].to_le_bytes())
    }
    let delay = u16::from_le_bytes([data[14], data[15]]);
    let mut next = scheduler.clone();
    let id = next.add_task(tick.wrapping_add(u32::from(delay)));
    let task = &mut next.records[usize::from(id)];
    task.kind = 0x33;
    task.payload = data;
    *scheduler = next;
    Ok(())
}
