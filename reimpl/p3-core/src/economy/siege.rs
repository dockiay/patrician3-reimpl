//! Decisión de desgaste e inicio de asedio dentro de `0x006349F0`.
//!
//! Las ayudas de mercancías, acuerdos y solicitudes se describen como llamadas;
//! el llamante debe ejecutarlas en el orden indicado antes de aplicar la decisión.
use crate::rng::CrtRand;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TerritorialPlunder {
    pub taken: bool,
    pub cash: u32,
    pub visits: u16,
    pub last_tick: u32,
}

/// Saqueo sin convoy de 0x633DB0; no incluye barcos ni cartas de capitanes.
pub fn plunder_without_convoy(
    town: &mut crate::town::Town,
    offices: &mut Vec<Vec<u8>>,
    merchants: &[crate::ai::reputation::Merchant],
    tick: u32,
    last_tick: u32,
    visits_initial: u16,
) -> Result<TerritorialPlunder, &'static str> {
    const PRICE: [u32; 20] = [
        0x3d6147af, 0x3ef50482, 0x3e6154ca, 0x3e322d0e, 0x3e11eb85, 0x3f0ccccd, 0x3fb33333, 0x3f8ccccd, 0x3f845a1d,
        0x40587ae1, 0x3ed33333, 0x3ce147af, 0x3fa39581, 0x3f8f5c29, 0x3ee147af, 0x3e8e5604, 0x3ee147af, 0x3e6147af,
        0x3f5ae147, 0x3d236e30,
    ];
    const SMALL: [u8; 20] = [0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0];
    let day = tick >> 8;
    if day <= (last_tick >> 8).wrapping_add(14) {
        return Ok(TerritorialPlunder {
            taken: false,
            cash: 0,
            visits: visits_initial,
            last_tick,
        });
    }
    let (mut t, mut os) = (town.clone(), offices.clone());
    let mut order = Vec::with_capacity(20);
    for w in 0..20 {
        let price = f32::from_bits(PRICE[w]);
        let at = order
            .iter()
            .position(|&v| f32::from_bits(PRICE[v]) < price)
            .unwrap_or(order.len());
        order.insert(at, w);
    }
    let limit = (t
        .u32(0x2e0)
        .wrapping_add(t.u32(0x2dc).wrapping_add(t.u32(0x2d8).wrapping_mul(2)).wrapping_mul(2)))
    .wrapping_mul(16);
    let cash = t.i32(0x700).max(0) as u32;
    let (mut value, mut capacity, mut visits) = (0u32, 20000u32, 0u16);
    fn take(stock: &mut u32, ware: usize, price: u32, cash: u32, value: &mut u32, limit: u32, capacity: &mut u32) {
        let p = f64::from(f32::from_bits(price));
        let mut amount = *stock;
        let total = cash.wrapping_add(*value);
        let cost = crate::numeric::ftol(f64::from(amount) * p) as u32;
        if limit < total.wrapping_add(cost) {
            amount = if total < limit {
                crate::numeric::ftol(f64::from(limit.wrapping_sub(total)) / p) as u32
            } else {
                0
            };
        }
        amount = amount.min(*capacity);
        amount -= amount % 200;
        if amount != 0 {
            *stock = stock.wrapping_sub(amount);
            *capacity = capacity.wrapping_sub(amount);
            *value = value.wrapping_add(crate::numeric::ftol(f64::from(amount) * p) as u32)
        }
        let _ = ware;
    }
    let mut cur = usize::from(t.u16(0x784));
    let mut seen = vec![false; os.len()];
    while cur < os.len() {
        if seen[cur] {
            return Err("cadena de oficinas cíclica");
        }
        seen[cur] = true;
        let office = &mut os[cur];
        if office.len() != 0x44c {
            return Err("oficina truncada");
        }
        let owner = usize::from(u16::from_le_bytes([office[0x2c4], office[0x2c5]]));
        let merchant = merchants.get(owner).ok_or("comerciante ausente")?;
        if cash.wrapping_add(value) < limit && merchant.u16(8) == 0 {
            for &w in &order {
                if SMALL[w] == 0 {
                    continue;
                }
                let at = 4 + 4 * w;
                let mut stock = u32::from_le_bytes(office[at..at + 4].try_into().unwrap());
                take(&mut stock, w, PRICE[w], cash, &mut value, limit, &mut capacity);
                office[at..at + 4].copy_from_slice(&stock.to_le_bytes());
            }
        }
        cur = usize::from(u16::from_le_bytes([office[0x2ca], office[0x2cb]]));
        visits = visits.wrapping_add(1);
    }
    if cash.wrapping_add(value) < limit {
        for &w in &order {
            if SMALL[w] == 0 {
                continue;
            }
            let mut stock = t.u32(4 + 4 * w);
            if stock != 0 {
                take(&mut stock, w, PRICE[w], cash, &mut value, limit, &mut capacity);
                t.set_u32(4 + 4 * w, stock);
            }
        }
    }
    let taken = cash.wrapping_add(value) != 0;
    let last_tick = if taken {
        crate::economy::militia::book_transaction(&mut t, tick, cash as i32, 8);
        day << 8
    } else {
        last_tick
    };
    *town = t;
    *offices = os;
    Ok(TerritorialPlunder {
        taken,
        cash,
        visits,
        last_tick,
    })
}

/// Repone la paciencia y, cuando corresponde, marca todas las peticiones por ciudad destino (0x634C50).
pub fn refill_goodwill(
    city: u8,
    goodwill: &mut u16,
    cooldown: u8,
    population: i32,
    tick: u32,
    fulfilled: bool,
    requests: &mut [Vec<[u8; 16]>],
    incoming_eax: u32,
) -> u32 {
    if fulfilled {
        for rows in requests.iter_mut() {
            for row in rows {
                if row[6] == city {
                    row[9] = 1;
                }
            }
        }
    }
    if cooldown == 0xff {
        let value = (population as u32).wrapping_mul(tick);
        *goodwill = (value % 0xa001) as u16 + 0x2800;
        return value / 0xa001;
    }
    let current = u32::from(*goodwill);
    let upper = if fulfilled { 0 } else { incoming_eax & 0xffff0000 };
    if current >= 0x5000 {
        return upper | current;
    }
    let next = current + if fulfilled { 0x2fff } else { 0x1fff };
    if next > 0x4fff {
        *goodwill = 0x4fff;
        upper | current
    } else {
        *goodwill = next as u16;
        upper | next
    }
}

/// Oferta 0x8B emitida por `handle_start_siege`, tras crear la escena.
pub fn supply_request_offer(
    home: u8,
    player: u8,
    target: u8,
    rank: u8,
    mode: u8,
    same_region: bool,
    requests: &[[u8; 16]],
    tick: u32,
    rng: &mut Option<CrtRand>,
) -> Result<Option<[u32; 5]>, &'static str> {
    if mode == 0
        || rank <= 4
        || home == target
        || !same_region
        || requests.iter().any(|r| r[5] == player && r[7] != 0 && r[8] == 0)
    {
        return Ok(None);
    }
    let mut next = rng.ok_or("estado CRT requerido para petición de asedio")?;
    let a = ((next.rand() % 9 + 8) * 5) as u8;
    let b = ((next.rand() % 9 + 8) * 5) as u8;
    let c = ((next.rand() % 5 + 4) * 5) as u8;
    let d = ((next.rand() % 9 + 8) * 5) as u8;
    let e = ((next.rand() % 5 + 4) * 5) as u8;
    let op = [
        0x8b,
        u32::from_le_bytes([c, e, b, d]),
        u32::from_le_bytes([a, home, player, target]),
        tick.wrapping_add(0x1500) & 0xffffff00,
        0,
    ];
    *rng = Some(next);
    Ok(Some(op))
}

/// Fecha textual de 0x5311D0: mes 1..12; conserva su aritmética firmada y el caso de mes 0.
pub fn supply_request_date(tick: u32) -> (u8, u8) {
    let days = (tick as i32) >> 8;
    let year = (days / 365) as u16;
    let remainder = (days as u32).wrapping_sub(u32::from(year) * 365);
    let mut month = 0u8;
    if remainder < 0x8000_0000 {
        loop {
            month = month.wrapping_add(1);
            if u32::from(crate::calendar::MONTH_START[usize::from(month)]) > remainder {
                break;
            }
        }
    }
    let start = if month == 0 {
        0x18
    } else {
        crate::calendar::MONTH_START[usize::from(month - 1)] as u8
    };
    ((remainder as u8).wrapping_sub(start).wrapping_add(1), month)
}

/// Inserta la solicitud 0x8B y prepara su carta 0x64. Los bytes 10..11 se leen del heap original sin inicializar.
pub fn create_supply_request(
    rows: &mut Vec<[u8; 16]>,
    city: u8,
    merchant: u8,
    target: u8,
    amounts: [u8; 5],
    due: u32,
    heap_tail: [u8; 2],
) -> [u8; 16] {
    let mut row = [0u8; 16];
    row[..5].copy_from_slice(&amounts);
    row[5] = merchant;
    row[6] = target;
    row[10..12].copy_from_slice(&heap_tail);
    row[12..16].copy_from_slice(&due.to_le_bytes());
    let index = if rows.is_empty() {
        0
    } else {
        1 + rows[1..]
            .iter()
            .take_while(|v| u32::from_le_bytes(v[12..16].try_into().unwrap()) < due)
            .count()
    };
    rows.insert(index, row);
    let (day, month) = supply_request_date(due);
    let mut letter = [0u8; 16];
    letter[1] = 0x80;
    letter[2..4].fill(0xff);
    letter[4] = 0x64;
    letter[5] = city;
    letter[8..16].copy_from_slice(&[
        amounts[4], amounts[2], amounts[0], amounts[3], amounts[1], day, month, target,
    ]);
    letter
}

/// Parte numérica de `town_siege_begin` (0x629C60), hasta la primera carta.
/// No inicializa la escena ni reproduce las llamadas posteriores.
pub fn initial_forces(
    image: &mut [u8],
    population: i32,
    difficulty: u8,
    variant: u8,
    strength: u8,
    militia: [u8; 4],
    tick: u32,
) -> Result<(), &'static str> {
    const TABLE: [[i32; 5]; 3] = [[262, 131, 99, 0, 55], [262, 131, 0, 87, 55], [262, 0, 99, 87, 55]];
    const LIMITS: [u16; 5] = [40, 22, 18, 15, 6];
    if image.len() != 0x650 {
        return Err("escena de asedio truncada");
    };
    let factors = TABLE
        .get(usize::from(variant))
        .ok_or("variante de asedio fuera de tabla")?;
    let scale = ((i32::from(difficulty) + 2) * 765 + i32::from(strength) * 2).wrapping_mul(population) / 2550;
    image[0x584..0x588].copy_from_slice(&(tick >> 8).to_le_bytes());
    for i in 0..5 {
        let raw = factors[i].wrapping_mul(scale).wrapping_add(0x7fff) as u32;
        let mut amount = ((raw >> 16) as u16).min(LIMITS[i]);
        if i == 0 {
            amount = amount.max(3)
        }
        let a = 0x21c + 4 * i;
        let d = 0x230 + 4 * i;
        image[a..a + 2].copy_from_slice(&u16::MAX.to_le_bytes());
        image[a + 2..a + 4].copy_from_slice(&amount.to_le_bytes());
        image[d..d + 2].copy_from_slice(&u16::MAX.to_le_bytes());
        if i < 4 {
            image[d + 2..d + 4].copy_from_slice(&u16::from(militia[i]).to_le_bytes());
        }
    }
    Ok(())
}

/// Resultado del comienzo sin murallas ni vista. La entrega de la carta 0x53 queda al llamante.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnwalledStart {
    pub image: Vec<u8>,
    pub flags: u32,
    pub notice_troops: u16,
}
pub fn begin_unwalled(
    population: i32,
    difficulty: u8,
    variant: u8,
    strength: u8,
    militia: [u8; 4],
    tick: u32,
    flags: u32,
    mode: u8,
) -> Result<UnwalledStart, &'static str> {
    let mut image = vec![0u8; 0x650];
    initial_forces(&mut image, population, difficulty, variant, strength, militia, tick)?;
    let read = |image: &[u8], at: usize| u32::from(u16::from_le_bytes([image[at], image[at + 1]]));
    let attackers = std::array::from_fn::<_, 4, _>(|i| read(&image, 0x21e + 4 * i));
    let sum = attackers.iter().sum::<u32>();
    let notice_troops = (sum as u16).wrapping_mul(5);
    let attacker = attackers[0] * 120 + attackers[1] * 30 + attackers[2] * 40 + attackers[3] * 45;
    let defender = u32::from(militia[0]) * 120
        + u32::from(militia[1]) * 30
        + u32::from(militia[2]) * 40
        + u32::from(militia[3]) * 45;
    image[0x22e..0x230].fill(0);
    image[8..10].copy_from_slice(&u16::MAX.to_le_bytes());
    image[0x3c..0x3e].copy_from_slice(&0x1e00u16.to_le_bytes());
    image[0x66..0x68].copy_from_slice(&1u16.to_le_bytes());
    image[0x68..0x6a].copy_from_slice(&0x1e00u16.to_le_bytes());
    image[0x6a..0x6c].copy_from_slice(&1u16.to_le_bytes());
    image[0x6d] = (population % 3) as u8;
    image[0x6f] = 0xff;
    image[0x78] = 1;
    image[0x79] = 1;
    image[0x578] = 0xff;
    image[0x57c..0x580].copy_from_slice(&attacker.to_le_bytes());
    image[0x580..0x584].copy_from_slice(&defender.to_le_bytes());
    image[0x5d4] = (sum / 6).clamp(1, 6) as u8;
    image[0x61c] = mode;
    image[0x620..0x624].fill(0xff);
    image[0x62c] = 0xff;
    Ok(UnwalledStart {
        image,
        flags: flags | 0x10,
        notice_troops,
    })
}

/// Puerta de `handle_start_siege` (0x633AF0), antes de crear la escena.
/// Un ciclo o una tabla truncada se rechazan en vez de reproducir un bucle o acceso inválido.
pub fn start_gate(town: &crate::town::Town, site: &crate::buildings::Site, scene: bool) -> Result<bool, &'static str> {
    if scene || town.u32(0x76c) & 0x100 == 0 {
        return Ok(false);
    }
    let count = usize::from(town.u16(0x776));
    if site.records.len() < count * 8 {
        return Err("registros de defensa truncados");
    }
    let mut hits = 0;
    for (head, building) in [(town.u16(0x768), false), (town.u16(0x762), true)] {
        let mut cursor = usize::from(head);
        let mut seen = vec![false; count];
        while cursor < count {
            if seen[cursor] {
                return Err("lista de defensa cíclica");
            }
            seen[cursor] = true;
            let row = &site.records[cursor * 8..cursor * 8 + 8];
            let kind = row[3];
            if if building {
                kind == 0x34
            } else {
                kind > 0x33 && kind != 0x36
            } {
                hits += 1;
            }
            if !building && hits >= 3 {
                return Ok(true);
            }
            cursor = usize::from(u16::from_le_bytes([row[4], row[5]]));
        }
    }
    Ok(hits >= 3)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DailyCall {
    ChooseWares,
    ProcessAgreements,
    ExpireRequests,
    ShipSurplus,
}

#[derive(Clone, Debug)]
pub struct GoodwillInput<'a> {
    pub disabled: bool,
    pub day: u8,
    pub day_of_year: u16,
    pub year: u16,
    pub difficulty: u8,
    pub town: u8,
    pub town_count: u32,
    pub regions: &'a [u8],
    pub population: i32,
    pub cooldown: u8,
    pub goodwill: u16,
    pub tick: u32,
    pub last_plunder: u32,
    pub scene: bool,
    pub scene_active: bool,
    pub network_client: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GoodwillResult {
    pub calls: Vec<DailyCall>,
    pub goodwill: u16,
    pub operation: Option<[u32; 5]>,
}

/// Calendario de ayudas antes del cálculo de buena voluntad.
pub fn daily_calls(input: &GoodwillInput<'_>) -> Vec<DailyCall> {
    use DailyCall::*;
    let mut calls = Vec::new();
    if input.disabled {
        return calls;
    }
    match input.day {
        1 => calls.push(ChooseWares),
        8 | 22 => calls.push(ProcessAgreements),
        _ => {}
    }
    calls.push(ExpireRequests);
    if input.day_of_year % 7 == 0 && (!input.scene || !input.scene_active) {
        calls.push(ShipSurplus)
    }
    calls
}

/// Parte final y calendario de llamadas del tick; no ejecuta las cuatro ayudas.
/// Para una integración real se actualiza primero el estado con dichas ayudas y
/// se pasa aquí el resultado. Si debe emitir 0x8E, el CRT avanza transaccionalmente.
pub fn goodwill(input: &GoodwillInput<'_>, rng: &mut Option<CrtRand>) -> Result<GoodwillResult, &'static str> {
    let mut result = GoodwillResult {
        calls: daily_calls(input),
        goodwill: input.goodwill,
        operation: None,
    };
    if input.disabled {
        return Ok(result);
    }
    let mut wear = if input.cooldown != 0xff && input.population > 800 {
        ((0x1019 - 0x190000 / input.population) / 50) as u32
    } else {
        0x29
    };
    wear = match input.difficulty {
        0 => wear.wrapping_mul(7).wrapping_add(5) / 10,
        2 => wear.wrapping_mul(3).wrapping_add(1) >> 1,
        _ => wear,
    };
    if wear < u32::from(input.goodwill) {
        result.goodwill = input.goodwill.wrapping_sub(wear as u16);
        return Ok(result);
    }
    if input.goodwill == 0 {
        return Ok(result);
    }
    let town = usize::from(input.town);
    if town >= input.regions.len() || input.town_count as usize > input.regions.len() {
        return Err("regiones de asedio incompletas");
    }
    let mut remaining = input.difficulty.wrapping_add(1);
    for i in 0..input.town_count as usize {
        if !input.scene && input.tick.wrapping_sub(input.last_plunder) > 0x1bff {
            continue;
        }
        if input.regions[i] == input.regions[town] {
            result.goodwill = 1;
            return Ok(result);
        }
        remaining = remaining.wrapping_sub(1);
        if remaining == 0 {
            result.goodwill = 1;
            return Ok(result);
        }
    }
    result.goodwill = 0;
    if input.network_client == 0 {
        let mut next = rng.ok_or("estado CRT requerido para iniciar asedio")?;
        let variant = if input.year < 0x546 {
            0
        } else if input.year < 0x578 {
            next.rand() as u32 & 1
        } else {
            next.rand() as u32 % 3
        };
        result.operation = Some([0x8e, u32::from(input.town), variant, next.rand() as u32 & 0xff, 1]);
        *rng = Some(next);
    }
    Ok(result)
}

/// `0x00635740`: expedición semanal de excedentes a la autoridad territorial.
/// Los umbrales se indexan por ranura de demanda, las existencias por mercancía.
pub fn ship_surplus(
    town: &mut crate::town::Town,
    wares: [u8; 4],
    goodwill: &mut u16,
    cooldown: &mut u8,
) -> Result<(), &'static str> {
    const SMALL: [u8; 24] = [0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1];
    if *cooldown != 0 {
        if *cooldown != 0xff {
            *cooldown -= 1
        }
        return Ok(());
    }
    if *goodwill >= 0x2000 {
        return Ok(());
    }
    if wares.iter().any(|&w| w != 0xff && usize::from(w) >= SMALL.len()) {
        return Err("mercancía de asedio fuera de tabla");
    }
    for (slot, ware) in wares.into_iter().enumerate() {
        if ware == 0xff {
            continue;
        }
        let stock = town.stock(usize::from(ware));
        let threshold = town.i32(0x4f0 + 16 * slot + 4);
        if stock <= threshold {
            continue;
        }
        let base = town.i32(0x4f0 + 16 * slot);
        let unit = if SMALL[usize::from(ware)] != 0 { 200u32 } else { 2000 };
        let count = (stock.wrapping_sub(base) as u32) / unit;
        town.set_stock(usize::from(ware), stock.wrapping_sub(count.wrapping_mul(unit) as i32));
        if *goodwill <= 0x4fff {
            let n = if unit == 2000 { count.wrapping_mul(5) } else { count };
            if n >= 1000 {
                *goodwill = 0x4fff
            } else {
                *goodwill = (u32::from(*goodwill) + n * 512 / 25).min(0x4fff) as u16
            }
        }
        *cooldown = 12;
    }
    Ok(())
}

/// `0x00635220`: selecciona hasta cuatro demandas desde los totales mundiales.
/// En las ranuras sin mercancía el original copia bytes sin inicializar de la
/// pila: los tres últimos se reciben como entrada, no se inventa su valor.
pub fn choose_wares(
    stock: [u32; 24],
    consumption: [u32; 24],
    population: u32,
    old: [u8; 4],
    levels: [u8; 4],
    stack_tail: [u8; 3],
) -> ([u8; 4], [u8; 4]) {
    let empty = old.iter().all(|&v| v == 0xff);
    let mut chosen = [0xff; 4];
    let mut scores = [0u32; 4];
    for i in 0..24 {
        let original = stock[i];
        let (s, c) = if original > 0x800000 {
            (original >> 9, consumption[i] >> 9)
        } else {
            (original, consumption[i])
        };
        let base = if c == 0 {
            original
        } else {
            ((population.wrapping_add(original) % 0x66) + 0xcc).wrapping_mul(s) / c
        };
        let score = base / if i < 20 { 14 } else { u32::MAX };
        if (score > 0xff || empty) && score > scores[3] {
            let mut j = 3;
            loop {
                if j < 3 {
                    chosen[j + 1] = chosen[j];
                    scores[j + 1] = scores[j]
                }
                chosen[j] = i as u8;
                scores[j] = score;
                if j == 0 || score <= scores[j - 1] {
                    break;
                }
                j -= 1;
            }
        }
    }
    if empty {
        for j in 1..4 {
            if scores[j] < 0x100 {
                chosen[j] = 0xff
            }
        }
    }
    let mut next = [24, stack_tail[0], stack_tail[1], stack_tail[2]];
    for j in 0..4 {
        if chosen[j] == 0xff {
            continue;
        }
        next[j] = 1;
        for k in 0..4 {
            if chosen[j] == old[k] {
                next[j] = if levels[k] < 3 { levels[k] + 1 } else { levels[k] }
            }
        }
    }
    (chosen, next)
}

/// `0x00532290`: totales para elegir demandas; rechaza ciclos de oficinas.
/// Las oficinas son imágenes completas de 0x44C bytes, con enlace en +0x2CA.
pub fn world_stock_consumption(
    towns: &[crate::town::Town],
    offices: &[Vec<u8>],
) -> Result<([u32; 24], [u32; 24]), &'static str> {
    if offices.len() > u16::MAX as usize || offices.iter().any(|o| o.len() != 0x44c) {
        return Err("oficinas de asedio inválidas");
    }
    let mut stock = [0u32; 24];
    let mut consumption = [0u32; 24];
    for town in towns {
        for k in 0..24 {
            stock[k] = stock[k].wrapping_add(town.u32(4 + 4 * k));
            consumption[k] = consumption[k]
                .wrapping_add(town.u32(0x64 + 4 * k))
                .wrapping_add(town.u32(0x310 + 4 * k));
        }
        let mut current = town.u16(0x784);
        let mut seen = vec![false; offices.len()];
        while usize::from(current) < offices.len() {
            let i = usize::from(current);
            if seen[i] {
                return Err("cadena de oficinas de asedio cíclica");
            }
            seen[i] = true;
            let office = &offices[i];
            for k in 0..24 {
                consumption[k] = consumption[k].wrapping_add(u32::from_le_bytes(
                    office[0x64 + 4 * k..0x68 + 4 * k].try_into().unwrap(),
                ));
            }
            current = u16::from_le_bytes([office[0x2ca], office[0x2cb]]);
        }
    }
    let count = towns.len() as u32;
    for (ware, minimum) in [
        (8, 800u32),
        (11, 84000),
        (12, 4000),
        (15, 3600),
        (17, 20000),
        (19, 160000),
    ] {
        stock[ware] = stock[ware].saturating_sub(count.wrapping_mul(minimum));
    }
    Ok((stock, consumption))
}

/// `0x00635440`: procesa acuerdos en orden sobre copias y confirma todo al final.
/// Cada registro contiene los ocho bytes serializados; el enlace se representa
/// por su posición en `agreements` y las oficinas conservan su enlace original.
pub fn process_agreements(
    goodwill: &mut u16,
    agreements: &mut Vec<[u8; 8]>,
    town: &crate::town::Town,
    offices: &mut Vec<Vec<u8>>,
    merchants: &mut Vec<crate::ai::reputation::Merchant>,
) -> Result<(), &'static str> {
    const SMALL: [u8; 24] = [0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1];
    if offices.len() > u16::MAX as usize
        || offices.iter().any(|o| o.len() != 0x44c)
        || merchants.iter().any(|m| m.bytes.len() < 0x650)
    {
        return Err("tablas de acuerdos truncadas");
    }
    let (mut goodwill_next, mut rows, mut offices_next, mut merchants_next) =
        (*goodwill, agreements.clone(), offices.clone(), merchants.clone());
    let mut cursor = 0;
    while cursor < rows.len() {
        let row = &mut rows[cursor];
        if row[2] == 0xff {
            row[2] = 0;
            cursor += 1;
            continue;
        }
        let mut at = town.u16(0x784);
        let mut seen = vec![false; offices_next.len()];
        while usize::from(at) < offices_next.len() {
            let i = usize::from(at);
            if seen[i] {
                return Err("cadena de oficinas de acuerdos cíclica");
            }
            seen[i] = true;
            let office = &mut offices_next[i];
            let owner = u16::from_le_bytes([office[0x2c4], office[0x2c5]]);
            if owner == u16::from(row[3]) {
                let ware = usize::from(row[0]);
                if ware >= 24 {
                    return Err("mercancía de acuerdo inválida");
                }
                let qty = u32::from(u16::from_le_bytes([row[4], row[5]]));
                let amount = qty * if SMALL[ware] != 0 { 200 } else { 2000 };
                let p = 4 + ware * 4;
                let stock = u32::from_le_bytes(office[p..p + 4].try_into().unwrap());
                if stock < amount {
                    goodwill_next = if goodwill_next < 0x19b {
                        1
                    } else {
                        goodwill_next - 0x19a
                    };
                } else {
                    let merchant = merchants_next
                        .get_mut(usize::from(row[3]))
                        .ok_or("comerciante de acuerdo ausente")?;
                    let price = u32::from(u16::from_le_bytes([row[6], row[7]]));
                    let payment = qty.wrapping_mul(price) as i32;
                    merchant.set_i32(0, merchant.i32(0).wrapping_add(payment));
                    merchant.set_i32(0x4c8, merchant.i32(0x4c8).wrapping_add(payment));
                    office[p..p + 4].copy_from_slice(&stock.wrapping_sub(amount).to_le_bytes());
                    if goodwill_next < 0x5000 {
                        let n = if SMALL[ware] == 0 { qty * 5 } else { qty };
                        goodwill_next = if n >= 1000 {
                            0x4fff
                        } else {
                            (u32::from(goodwill_next) + n * 512 / 25).min(0x4fff) as u16
                        };
                    }
                    row[2] = row[2].wrapping_add(1);
                }
                break;
            }
            at = u16::from_le_bytes([office[0x2ca], office[0x2cb]]);
        }
        row[1] = row[1].wrapping_sub(1);
        if row[1] == 0 {
            rows.remove(cursor);
        } else {
            cursor += 1
        }
    }
    *goodwill = goodwill_next;
    *agreements = rows;
    *offices = offices_next;
    *merchants = merchants_next;
    Ok(())
}

/// Carta de pago 0x85; el buzón y sus preferencias los resuelve el llamante.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SupplyPayment {
    pub merchant: u8,
    pub home: u8,
    pub without_bonus: bool,
    pub amount: u16,
}

/// `0x00636230`: despacha los nodos vencidos hasta el primero aún vigente.
/// La lista contiene las 16 posiciones serializadas de cada petición.
pub fn expire_requests(
    goodwill: &mut u16,
    requests: &mut Vec<[u8; 16]>,
    population: u32,
    tick: u32,
    merchants: &mut Vec<crate::ai::reputation::Merchant>,
) -> Result<Vec<SupplyPayment>, &'static str> {
    let (mut next_good, mut next_rows, mut next_merchants) = (*goodwill, requests.clone(), merchants.clone());
    let mut letters = Vec::new();
    let mut consumed = 0;
    for row in &next_rows {
        if u32::from_le_bytes(row[12..16].try_into().unwrap()) >= tick {
            break;
        }
        consumed += 1;
        if row[7] == 0 {
            continue;
        }
        if row[8] == 0 {
            next_good = if next_good < 0x19b { 1 } else { next_good - 0x19a };
            continue;
        }
        // Cada constante es el f32 exacto del PE; los productos enteros y esta suma
        // caben exactamente en f64 antes del truncado x87 de este rango de bytes.
        let terms = [
            (1, 2000, 0x3d6147af),
            (3, 200, 0x3e8e5604),
            (2, 200, 0x3fa39581),
            (4, 200, 0x3e322d0e),
            (0, 2000, 0x3ce147af),
        ];
        let mut total = 0f64;
        for (index, unit, bits) in terms {
            total += f64::from(row[index]) * f64::from(unit) * f64::from(f32::from_bits(bits));
        }
        let mut amount = total.trunc() as u32;
        if row[6] != 0xff {
            if row[9] == 0 {
                let divisor = amount.wrapping_mul(3) / 10;
                if divisor == 0 {
                    return Err("división original por cero en petición de asedio");
                }
                amount = amount.wrapping_sub(population % divisor);
            } else {
                let divisor = amount / 5;
                if divisor == 0 {
                    return Err("división original por cero en petición de asedio");
                }
                amount = amount.wrapping_add(amount >> 1).wrapping_add(population % divisor);
            }
        }
        let merchant = next_merchants
            .get_mut(usize::from(row[5]))
            .ok_or("comerciante de petición ausente")?;
        if merchant.bytes.len() < 0x650 {
            return Err("comerciante de petición truncado");
        }
        merchant.set_i32(0, merchant.i32(0).wrapping_add(amount as i32));
        merchant.set_i32(0x4b8, merchant.i32(0x4b8).wrapping_add(amount as i32));
        letters.push(SupplyPayment {
            merchant: row[5],
            home: merchant.bytes[0x19],
            without_bonus: row[9] == 0,
            amount: amount as u16,
        });
    }
    next_rows.drain(..consumed);
    *goodwill = next_good;
    *requests = next_rows;
    *merchants = next_merchants;
    Ok(letters)
}

/// `0x0051D5F0`: retira una unidad de milicia y ajusta empleados y población.
pub fn remove_militia_unit(town: &mut crate::town::Town, kind: usize) -> Result<(), &'static str> {
    if kind >= 4 {
        return Err("tipo de milicia inválido");
    }
    let count = town.u8(0x998 + kind);
    if count == 0 {
        return Ok(());
    }
    town.set_u8(0x998 + kind, count - 1);
    town.set_u16(0x844, town.u16(0x844).wrapping_sub(5));
    let beggars = town.i32(0x2e4).wrapping_add(15);
    town.set_i32(0x2e4, beggars);
    let mut total = beggars;
    let mut remaining = 20i32;
    for offset in [0x2e0, 0x2dc, 0x2d8] {
        let old = town.i32(offset);
        let next = if remaining < old {
            let v = old.wrapping_sub(remaining);
            remaining = 0;
            v
        } else {
            remaining = remaining.wrapping_add(1i32.wrapping_sub(old));
            1
        };
        town.set_i32(offset, next);
        total = total.wrapping_add(next);
    }
    town.set_i32(0x2d4, total);
    Ok(())
}

/// Selección inicial de puerta de `town_siege_begin` (0x629C60).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WallGateChoice {
    pub gates: [u32; 3],
    pub selected: u8,
    pub nearby_towers: u16,
}

/// Recorre las defensas terminadas y después las obras; no construye la escena.
/// Rechaza listas cíclicas y ausencia de puerta en lugar de usar memoria indeterminada.
pub fn choose_wall_gate(
    records: &[u8],
    defense_head: u16,
    construction_head: u16,
    level: u8,
    population: i32,
) -> Result<WallGateChoice, &'static str> {
    if records.len() % 8 != 0 || records.len() / 8 > u16::MAX as usize || level > 2 {
        return Err("defensas inválidas");
    }
    let count = records.len() / 8;
    let next = |i: usize| u16::from_le_bytes([records[8 * i + 4], records[8 * i + 5]]);
    let mut defense = Vec::new();
    let mut seen = vec![false; count];
    let mut cursor = usize::from(defense_head);
    while cursor < count {
        if seen[cursor] {
            return Err("defensas cíclicas");
        }
        seen[cursor] = true;
        defense.push(cursor);
        cursor = usize::from(next(cursor));
    }
    let mut result = WallGateChoice {
        gates: [0; 3],
        selected: 0,
        nearby_towers: 0,
    };
    let mut switched = false;
    let mut cursor = usize::from(defense_head);
    let mut seen = vec![false; count];
    let mut best = u16::MAX;
    let mut ties = 0i32;
    let mut found = false;
    for slot in 0..3 {
        result.gates[slot] = cursor as u32;
        while cursor < count {
            if seen[cursor] {
                return Err("lista de puertas cíclica");
            }
            seen[cursor] = true;
            let row = &records[8 * cursor..8 * cursor + 8];
            if matches!(row[3], 0x34 | 0x35 | 0x37 | 0x38) && row[2] == 0xff - level {
                break;
            }
            let n = usize::from(next(cursor));
            if switched || n < count {
                cursor = n;
            } else {
                switched = true;
                cursor = usize::from(construction_head);
            }
            result.gates[slot] = cursor as u32;
        }
        if cursor >= count {
            break;
        }
        found = true;
        let gate = &records[8 * cursor..8 * cursor + 8];
        let mut score = u16::from(matches!(gate[3], 0x34 | 0x35));
        let mut towers = 0u16;
        for &id in &defense {
            let row = &records[8 * id..8 * id + 8];
            if !matches!(row[3], 0x32 | 0x33) {
                continue;
            }
            let dx = (i32::from(gate[0]) - i32::from(row[0])) * 2;
            let dy = i32::from(gate[1]) - i32::from(row[1]) - 2;
            if dx * dx + dy * dy < 0x49 {
                score = score.wrapping_add(if row[3] == 0x32 { 2 } else { 3 });
                towers = towers.wrapping_add(1);
            }
        }
        if score < best {
            best = score;
            ties = 1;
            result.selected = slot as u8;
            result.nearby_towers = towers;
        } else if score == best {
            ties += 1;
            if population % ties == 0 {
                result.selected = slot as u8;
                result.nearby_towers = towers;
            }
        }
        if slot < 2 {
            let n = usize::from(next(cursor));
            if switched || n < count {
                cursor = n;
            } else {
                switched = true;
                cursor = usize::from(construction_head);
            }
        }
    }
    if found {
        Ok(result)
    } else {
        Err("ciudad amurallada sin puerta acreditada")
    }
}

/// Lista de torres reconstruida por `0x62B2B0`; los tres bytes no escritos
/// de cada entrada conservan el contenido recibido del asignador.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WallTowers {
    pub bytes: Vec<u8>,
    pub count: u16,
    pub allocated: bool,
}

pub fn list_wall_towers(
    records: &[u8],
    defense_head: u16,
    gate_index: u32,
    capacity: u16,
    clock: u32,
    heap_bytes: &[u8],
) -> Result<WallTowers, &'static str> {
    if records.len() % 8 != 0 || records.len() / 8 > u16::MAX as usize || heap_bytes.len() != usize::from(capacity) * 12
    {
        return Err("tabla o memoria de torres inválida");
    }
    let mut out = WallTowers {
        bytes: heap_bytes.to_vec(),
        count: 0,
        allocated: capacity != 0,
    };
    if capacity == 0 {
        return Ok(out);
    }
    let count = records.len() / 8;
    let gate = records
        .chunks_exact(8)
        .nth(gate_index as usize)
        .ok_or("puerta inexistente")?;
    let mut cursor = usize::from(defense_head);
    let mut seen = vec![false; count];
    let mut written = 0u8;
    while cursor < count {
        if seen[cursor] {
            return Err("defensas cíclicas");
        }
        seen[cursor] = true;
        let row = &records[8 * cursor..8 * cursor + 8];
        if row[6] == 0 && matches!(row[3], 0x32 | 0x33) {
            let dx = (i32::from(gate[0]) - i32::from(row[0])) * 2;
            let dy = i32::from(gate[1]) - i32::from(row[1]) - 2;
            if dx * dx + dy * dy < 0x49 {
                let at = usize::from(written) * 12;
                if at + 12 > out.bytes.len() {
                    return Err("reserva de torres insuficiente");
                }
                out.bytes[at] = written;
                out.bytes[at + 4..at + 8].copy_from_slice(&clock.to_le_bytes());
                out.bytes[at + 8..at + 12].copy_from_slice(&(cursor as u32).to_le_bytes());
                written = written.wrapping_add(1);
            }
        }
        cursor = usize::from(u16::from_le_bytes([row[4], row[5]]));
    }
    out.count = u16::from(written);
    Ok(out)
}

/// Calcula las casillas de la puerta elegida (`0x632780`); no crea la vista.
/// Ante una rejilla truncada deja la imagen sin cambios.
pub fn compute_wall_gate_cells(
    image: &mut [u8],
    records: &[u8],
    grid: &[u8],
    stride: usize,
) -> Result<bool, &'static str> {
    if image.len() != 0x650 || records.len() % 8 != 0 {
        return Err("imagen o edificios inválidos");
    }
    let slot = usize::from(image[0x6d]);
    if slot >= 3 {
        return Err("ranura de puerta inválida");
    }
    let at = 0x244 + 4 * slot;
    let gate = u32::from_le_bytes(image[at..at + 4].try_into().unwrap()) as usize;
    if gate >= records.len() / 8 {
        return Ok(false);
    }
    let row = &records[8 * gate..8 * gate + 8];
    let x = u16::from(row[0]);
    let y = u16::from(row[1]);
    let yy = y.wrapping_sub(4);
    let cell = stride
        .checked_mul(usize::from(x))
        .and_then(|v| v.checked_add(usize::from(yy)))
        .and_then(|v| grid.get(v))
        .ok_or("casilla de puerta fuera del plano")?;
    let left = matches!(row[3], 0x34 | 0x37);
    let right = matches!(row[3], 0x35 | 0x38);
    let gate_y = y.wrapping_sub(3);
    let gate_x = if left && gate_y & 1 == 0 {
        x.wrapping_add(1)
    } else if right && gate_y & 1 != 0 {
        x.wrapping_sub(1)
    } else {
        x
    };
    let mut next = image.to_vec();
    fn put(out: &mut [u8], at: usize, value: u16) {
        out[at..at + 2].copy_from_slice(&value.to_le_bytes());
    }
    put(&mut next, 8, gate_x);
    put(&mut next, 0xa, gate_y);
    put(
        &mut next,
        0x5a2,
        if left { x.wrapping_add(2) } else { x.wrapping_sub(2) },
    );
    put(&mut next, 0x5a4, yy);
    put(&mut next, 0x5a6, x);
    put(&mut next, 0x5a8, y);
    if cell & 0x80 != 0 {
        put(&mut next, 0xc, x);
        put(&mut next, 0xe, yy);
        put(
            &mut next,
            0x5ca,
            if left { x.wrapping_add(1) } else { x.wrapping_sub(1) },
        );
        put(
            &mut next,
            0x5cc,
            if left { x.wrapping_add(3) } else { x.wrapping_sub(3) },
        );
        put(&mut next, 0x5ce, y);
        put(&mut next, 0x5d0, yy);
        next[0x5d2] = if left { 0xe0 } else { 0x20 };
    } else {
        put(
            &mut next,
            0xc,
            if left {
                gate_x.wrapping_add(1)
            } else {
                gate_x.wrapping_sub(1)
            },
        );
        put(&mut next, 0xe, yy.wrapping_add(3));
        put(
            &mut next,
            0x5ca,
            if left {
                gate_x.wrapping_sub(1)
            } else {
                gate_x.wrapping_add(1)
            },
        );
        put(
            &mut next,
            0x5cc,
            if left {
                gate_x.wrapping_add(1)
            } else {
                gate_x.wrapping_sub(1)
            },
        );
        put(&mut next, 0x5ce, gate_y);
        put(&mut next, 0x5d0, gate_y.wrapping_sub(4));
        next[0x5d2] = if left { 0x60 } else { 0xa0 };
    }
    image.copy_from_slice(&next);
    Ok(true)
}

/// Resultado de `0x525220`: el tamaño solicitado excluye el contador inicial.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutsideBusinesses {
    pub bytes: Vec<u8>,
    pub allocation_size: usize,
    pub wrote_past_allocation: bool,
}

/// Construye la lista en la memoria suministrada, incluidos los bytes posteriores
/// a la reserva si el original los escribiría. No modifica la entrada al fallar.
pub fn list_outside_businesses(
    records: &[u8],
    offices: &[Vec<u8>],
    office_head: u16,
    grid: &[u8],
    stride: usize,
    heap_bytes: &[u8],
) -> Result<OutsideBusinesses, &'static str> {
    if records.len() % 8 != 0
        || records.len() / 8 > u16::MAX as usize
        || offices.len() > u16::MAX as usize
        || offices.iter().any(|o| o.len() != 0x44c)
    {
        return Err("tablas de negocios inválidas");
    }
    let allocation_size = records.len() / 4;
    if heap_bytes.len() < allocation_size.max(2) {
        return Err("memoria de salida insuficiente");
    }
    let mut bytes = heap_bytes.to_vec();
    bytes[..2].fill(0);
    let mut highest = 2usize;
    let mut count = 0u16;
    let mut office = usize::from(office_head);
    let mut seen_offices = vec![false; offices.len()];
    while office < offices.len() {
        if seen_offices[office] {
            return Err("oficinas cíclicas");
        }
        seen_offices[office] = true;
        let raw = &offices[office];
        let mut building = usize::from(u16::from_le_bytes([raw[0x2ce], raw[0x2cf]]));
        let mut seen_buildings = vec![false; records.len() / 8];
        while building < records.len() / 8 {
            if seen_buildings[building] {
                return Err("negocios cíclicos");
            }
            seen_buildings[building] = true;
            let row = &records[8 * building..8 * building + 8];
            let at = stride
                .checked_mul(usize::from(row[0]))
                .and_then(|v| v.checked_add(usize::from(row[1])))
                .ok_or("coordenada de negocio inválida")?;
            if grid.get(at).ok_or("negocio fuera del plano")? & 0x80 == 0 {
                if count == u16::MAX {
                    return Err("contador original de negocios desbordado");
                }
                count += 1;
                let at = usize::from(count) * 2;
                if at + 2 > bytes.len() {
                    return Err("bytes posteriores al montón no disponibles");
                }
                bytes[at..at + 2].copy_from_slice(&(building as u16).to_le_bytes());
                highest = highest.max(at + 2);
            }
            building = usize::from(u16::from_le_bytes([row[4], row[5]]));
        }
        office = usize::from(u16::from_le_bytes([raw[0x2ca], raw[0x2cb]]));
    }
    bytes[..2].copy_from_slice(&count.to_le_bytes());
    Ok(OutsideBusinesses {
        bytes,
        allocation_size,
        wrote_past_allocation: highest > allocation_size,
    })
}

/// Selección y orden de objetivos de `town_siege_begin` antes de calcular duraciones.
/// La lista `outside` ya contiene los índices de negocios situados fuera de la muralla.
pub fn select_wall_targets(
    records: &[u8],
    gates: [u32; 3],
    chosen: u8,
    outside: &[u16],
    population: i32,
    gate_x: u16,
    gate_y: u16,
) -> Result<Vec<u16>, &'static str> {
    Ok(select_wall_targets_with_storage(records, gates, chosen, outside, population, gate_x, gate_y)?.active)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WallTargets {
    pub active: Vec<u16>,
    pub slots: Vec<u16>,
}

/// Conserva también las ranuras ya descartadas que el original deja escritas.
pub fn select_wall_targets_with_storage(
    records: &[u8],
    gates: [u32; 3],
    chosen: u8,
    outside: &[u16],
    population: i32,
    gate_x: u16,
    gate_y: u16,
) -> Result<WallTargets, &'static str> {
    if records.len() % 8 != 0 || chosen >= 3 || outside.len() > 127 {
        return Err("objetivos de asedio inválidos");
    }
    let count = records.len() / 8;
    if outside.is_empty() {
        return Ok(WallTargets {
            active: Vec::new(),
            slots: Vec::new(),
        });
    }
    if gates.iter().any(|&id| id as usize >= count) || outside.iter().any(|&id| usize::from(id) >= count) {
        return Err("edificio objetivo inexistente");
    }
    let mut result = Vec::new();
    for &id in outside {
        let row = &records[usize::from(id) * 8..usize::from(id) * 8 + 8];
        let mut nearest = 0usize;
        let mut distance = i32::MAX;
        for (j, &gate) in gates.iter().enumerate() {
            let other = &records[gate as usize * 8..gate as usize * 8 + 8];
            let x = i32::from(other[0]) - i32::from(row[0]);
            let y = i32::from(other[1]) - i32::from(row[1]);
            let d = x * x + y * y;
            if d < distance {
                distance = d;
                nearest = j;
            }
        }
        if nearest == usize::from(chosen) && row[3] != 5 {
            result.push(id);
        }
    }
    if result.is_empty() {
        return Ok(WallTargets {
            active: Vec::new(),
            slots: result,
        });
    }
    let initial = result.len() as u8;
    let desired = ((population % 5) as u8)
        .wrapping_add(initial >> 2)
        .wrapping_add(1)
        .min(initial);
    let mut index = 0usize;
    let mut active_len = result.len();
    while index < active_len {
        let divisor = index as i32 - active_len as i32;
        let rem = population.checked_rem(divisor).ok_or("división original inválida")?;
        if rem < i32::from(desired) - index as i32 {
            index += 1;
        } else {
            active_len -= 1;
            result[index] = result[active_len];
        }
    }
    let mut previous: Option<(i32, i32)> = None;
    for first in 0..active_len.saturating_sub(1) {
        let mut best = i32::MIN;
        let mut picked = None;
        for (j, &id) in result[..active_len].iter().enumerate().skip(first) {
            let row = &records[usize::from(id) * 8..usize::from(id) * 8 + 8];
            let delta = (
                (i32::from(gate_x) - i32::from(row[0])) * 2,
                i32::from(gate_y) - i32::from(row[1]),
            );
            let anchor = previous.unwrap_or(delta);
            let dot = anchor
                .0
                .wrapping_mul(delta.0)
                .wrapping_add(anchor.1.wrapping_mul(delta.1));
            if best < dot {
                best = dot;
                picked = Some((j, delta));
            }
        }
        let (picked, picked_delta) = picked.ok_or("orden original de objetivos depende de registro previo")?;
        result.swap(first, picked);
        previous = Some(picked_delta);
    }
    Ok(WallTargets {
        active: result[..active_len].to_vec(),
        slots: result,
    })
}

/// Duraciones de la marcha en un asedio amurallado sin objetivos exteriores.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EmptyWallDurations {
    pub approach: u16,
    pub retreat: u16,
    pub march: u16,
    pub threshold: u16,
}

pub fn empty_wall_durations(x: u16, y: u16, level: u8) -> Result<EmptyWallDurations, &'static str> {
    if level > 2 || x > 127 || y > 225 {
        return Err("puerta fuera del mapa de asedio");
    }
    let distance = [(0i32, 0i32), (127, 0), (0, 225), (127, 225)]
        .into_iter()
        .map(|(cx, cy)| {
            let dx = (i32::from(x) - cx) * 2;
            let dy = i32::from(y) - cy;
            ((f64::from(dx * dx + dy * dy).sqrt() + 0.5).trunc()) as u32
        })
        .min()
        .unwrap();
    let total = distance + 30;
    let approach = (((total * 65) >> 2) as u16).clamp(0x900, 0x1300);
    let retreat = (((distance * 65) >> 2) as u16).clamp(0x900, 0x1300);
    let threshold = u32::from(approach) - (u32::from(approach) * 30 / total);
    Ok(EmptyWallDurations {
        approach,
        retreat,
        march: (u16::from(level) + 1) * 0xa00,
        threshold: threshold as u16,
    })
}

/// Duraciones y puntos de paso del asedio amurallado con negocios exteriores.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WallDurations {
    pub approach: u16,
    pub retreat: u16,
    pub march: u16,
    pub threshold: u16,
    pub milestones: Vec<u16>,
}

pub fn wall_durations(x: u16, y: u16, level: u8, targets: &[(u8, u8)]) -> Result<WallDurations, &'static str> {
    if level > 2 || x > 127 || y > 225 || targets.len() > 127 {
        return Err("recorrido de asedio fuera del mapa admitido");
    }
    fn dist(a: (i32, i32), b: (i32, i32)) -> u32 {
        let dx = (a.0 - b.0) * 2;
        let dy = a.1 - b.1;
        (f64::from(dx * dx + dy * dy).sqrt() + 0.5).trunc() as u32
    }
    fn corner(p: (i32, i32)) -> u32 {
        [(0, 0), (127, 0), (0, 225), (127, 225)]
            .into_iter()
            .map(|c| dist(p, c))
            .min()
            .unwrap()
    }
    let gate = (i32::from(x), i32::from(y));
    let gate_corner = corner(gate);
    let retreat = (((gate_corner * 65) >> 2) as u16).clamp(0x900, 0x1300);
    if targets.is_empty() {
        let empty = empty_wall_durations(x, y, level)?;
        return Ok(WallDurations {
            approach: empty.approach,
            retreat,
            march: empty.march,
            threshold: empty.threshold,
            milestones: Vec::new(),
        });
    }
    let point = |p: (u8, u8)| (i32::from(p.0), i32::from(p.1));
    let first = corner(point(targets[0]));
    let mut total = first + 40 + dist(gate, point(*targets.last().unwrap()));
    for pair in targets.windows(2) {
        total += 10 + dist(point(pair[0]), point(pair[1]));
    }
    let approach = (((total * 65) >> 2) as u16).clamp(0x900, 0x1300);
    let threshold = u32::from(approach) - u32::from(approach) * 30 / total;
    let mut remaining = threshold - threshold * first / (total - 30);
    let mut route = total - 30 - first;
    let mut previous = point(targets[0]);
    let mut milestones = Vec::with_capacity(targets.len());
    for &target in targets {
        let next = point(target);
        let segment = 10 + dist(previous, next);
        if route == 0 {
            return Err("división original de ruta por cero");
        }
        let part = remaining * segment / route;
        route -= segment;
        remaining -= part;
        milestones.push(remaining as u16);
        previous = next;
    }
    Ok(WallDurations {
        approach,
        retreat,
        march: (u16::from(level) + 1) * 0xa00,
        threshold: threshold as u16,
        milestones,
    })
}

/// Puntuaciones finales de `town_siege_begin` para un ataque amurallado.
/// El original recorre el número de torres candidatas anterior al filtrado,
/// por lo que la cola puede contener índices procedentes del montón anterior.
pub fn wall_strength_ratings(
    attacker: [u16; 5],
    defender: [u16; 4],
    records: &[u8],
    tower_slots: &[u8],
    candidates: u16,
) -> Result<(u32, u32), &'static str> {
    if records.len() % 8 != 0 || tower_slots.len() < usize::from(candidates) * 12 {
        return Err("tablas de fuerzas truncadas");
    }
    let a = attacker.map(u32::from);
    let d = defender.map(u32::from);
    let attack = a[0] * 120 + a[1] * 30 + a[2] * 40 + a[3] * 45 + a[4] * 378;
    let mut defense = d[0] * 120 + d[1] * 30 + d[2] * 40 + d[3] * 45;
    for slot in tower_slots.chunks_exact(12).take(usize::from(candidates)) {
        let index = u32::from_le_bytes(slot[8..12].try_into().unwrap()) as usize;
        let row = records
            .chunks_exact(8)
            .nth(index)
            .ok_or("índice de torre no inicializado o inválido")?;
        if row[6] == 0 {
            defense = defense.wrapping_add(if row[3] == 0x32 { 0x2d0 } else { 0x708 });
        }
    }
    Ok((attack, defense))
}

/// Elige la torre activa (`0x62E810`), dando prioridad al tipo 0x32.
/// Las entradas incompletas o de otro tipo se rechazan si el original no
/// encontraría una torre válida al recorrer la lista.
pub fn select_active_wall_tower(
    records: &[u8],
    slots: &[u8],
    count: u16,
    population: i32,
    initial: u8,
) -> Result<u8, &'static str> {
    if records.len() % 8 != 0 || slots.len() < usize::from(count) * 12 || count > 255 || population < 0 {
        return Err("lista de torres no admitida");
    }
    if count == 0 {
        return Ok(initial);
    }
    let mut type_32 = Vec::new();
    let mut type_33 = Vec::new();
    for (i, slot) in slots.chunks_exact(12).take(usize::from(count)).enumerate() {
        let id = u32::from_le_bytes(slot[8..12].try_into().unwrap()) as usize;
        let row = records.chunks_exact(8).nth(id).ok_or("índice de torre inválido")?;
        if row[6] == 0 {
            if row[3] == 0x32 {
                type_32.push(i as u8);
            } else if row[3] == 0x33 {
                type_33.push(i as u8);
            } else {
                return Err("tipo de torre no admitido");
            }
        }
    }
    let choices = if type_32.is_empty() { &type_33 } else { &type_32 };
    if choices.is_empty() {
        return Err("ninguna torre activa válida");
    }
    Ok(choices[population as usize % choices.len()])
}

/// Entradas externas del inicio amurallado sin vista de ciudad.
pub struct WalledStartInput<'a> {
    pub city: u8,
    pub records: &'a [u8],
    pub offices: &'a [Vec<u8>],
    pub defense_head: u16,
    pub construction_head: u16,
    pub office_head: u16,
    pub level: u8,
    pub grid: &'a [u8],
    pub stride: usize,
    pub tower_heap: &'a [u8],
    pub list_heap: &'a [u8],
    pub tower_address: u32,
    pub militia: [u8; 4],
    pub population: i32,
    pub population_0: i32,
    pub population_2: i32,
    pub tick: u32,
    pub wall_clock: u32,
    pub variant: u8,
    pub strength: u8,
    pub difficulty: u8,
    pub mode: u8,
    pub flags: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WalledStart {
    pub image: Vec<u8>,
    pub tower_heap: Vec<u8>,
    pub list_heap: Vec<u8>,
    pub outside_overflow: bool,
    pub flags: u32,
    pub notice_troops: u16,
}

/// Construye una escena sin vista sobre copias; no publica la carta ni la escena.
pub fn begin_walled(input: &WalledStartInput<'_>) -> Result<WalledStart, &'static str> {
    let mut image = vec![0u8; 0x650];
    image[0x5d3] = input.city;
    initial_forces(
        &mut image,
        input.population,
        input.difficulty,
        input.variant,
        input.strength,
        input.militia,
        input.tick,
    )?;
    image[..4].copy_from_slice(&0x10000u32.to_le_bytes());
    image[4..8].copy_from_slice(&0x10000u32.to_le_bytes());
    image[0x61c] = input.mode;
    let gate = choose_wall_gate(
        input.records,
        input.defense_head,
        input.construction_head,
        input.level,
        input.population,
    )?;
    for (i, &index) in gate.gates.iter().enumerate() {
        image[0x244 + 4 * i..0x248 + 4 * i].copy_from_slice(&index.to_le_bytes());
    }
    image[0x6d] = gate.selected;
    let count = usize::from(gate.nearby_towers);
    if count != 0 && input.tower_address == 0 {
        return Err("dirección de torres ausente");
    }
    if input.tower_heap.len() < count * 12 {
        return Err("memoria de torres insuficiente");
    }
    let mut tower_heap = input.tower_heap.to_vec();
    let towers = list_wall_towers(
        input.records,
        input.defense_head,
        gate.gates[usize::from(gate.selected)],
        gate.nearby_towers,
        input.wall_clock,
        &tower_heap[..count * 12],
    )?;
    tower_heap[..count * 12].copy_from_slice(&towers.bytes);
    if towers.allocated {
        image[0x250..0x254].copy_from_slice(&input.tower_address.to_le_bytes());
    }
    image[0x242..0x244].copy_from_slice(&towers.count.to_le_bytes());
    if !compute_wall_gate_cells(&mut image, input.records, input.grid, input.stride)? {
        return Err("puerta inexistente");
    }
    let outside = list_outside_businesses(
        input.records,
        input.offices,
        input.office_head,
        input.grid,
        input.stride,
        input.list_heap,
    )?;
    let outside_count = usize::from(u16::from_le_bytes([outside.bytes[0], outside.bytes[1]]));
    if 2 + outside_count * 2 > outside.bytes.len() {
        return Err("lista exterior truncada");
    }
    let indices = outside.bytes[2..2 + outside_count * 2]
        .chunks_exact(2)
        .map(|v| u16::from_le_bytes([v[0], v[1]]))
        .collect::<Vec<_>>();
    let x = u16::from_le_bytes([image[8], image[9]]);
    let y = u16::from_le_bytes([image[10], image[11]]);
    let selected = select_wall_targets_with_storage(
        input.records,
        gate.gates,
        gate.selected,
        &indices,
        input.population_0,
        x,
        y,
    )?;
    for (i, &id) in selected.slots.iter().enumerate() {
        image[0x258 + 8 * i..0x25c + 8 * i].copy_from_slice(&u32::from(id).to_le_bytes());
    }
    let targets = selected.active;
    image[0x6e] = targets.len() as u8;
    image[0x6f] = if targets.is_empty() { 0xff } else { 0 };
    let positions = targets
        .iter()
        .map(|&id| {
            let at = usize::from(id) * 8;
            (input.records[at], input.records[at + 1])
        })
        .collect::<Vec<_>>();
    let duration = wall_durations(x, y, input.level, &positions)?;
    image[0x66..0x68].copy_from_slice(&duration.approach.to_le_bytes());
    image[0x42..0x44].copy_from_slice(&duration.approach.to_le_bytes());
    image[0x68..0x6a].copy_from_slice(&duration.march.to_le_bytes());
    image[0x6a..0x6c].copy_from_slice(&duration.retreat.to_le_bytes());
    image[0x44..0x46].copy_from_slice(&duration.threshold.to_le_bytes());
    for (i, (&id, &milestone)) in targets.iter().zip(&duration.milestones).enumerate() {
        image[0x258 + 8 * i..0x25c + 8 * i].copy_from_slice(&u32::from(id).to_le_bytes());
        image[0x254 + 8 * i..0x256 + 8 * i].copy_from_slice(&milestone.to_le_bytes());
    }
    let attacker = std::array::from_fn(|i| u16::from_le_bytes([image[0x21e + 4 * i], image[0x21f + 4 * i]]));
    let defender = std::array::from_fn(|i| u16::from_le_bytes([image[0x232 + 4 * i], image[0x233 + 4 * i]]));
    let (attack, defense) = wall_strength_ratings(attacker, defender, input.records, &tower_heap, gate.nearby_towers)?;
    image[0x57c..0x580].copy_from_slice(&attack.to_le_bytes());
    image[0x580..0x584].copy_from_slice(&defense.to_le_bytes());
    image[0x6c] = select_active_wall_tower(
        input.records,
        &tower_heap,
        towers.count,
        input.population_2,
        image[0x6c],
    )?;
    let sum = attacker[..4].iter().map(|&x| u32::from(x)).sum::<u32>();
    image[0x5d4] = (sum / 6).clamp(1, 6) as u8;
    image[0x578] = 0xff;
    image[0x620..0x624].fill(0xff);
    image[0x62c] = 0xff;
    Ok(WalledStart {
        image,
        tower_heap,
        list_heap: outside.bytes,
        outside_overflow: outside.wrote_past_allocation,
        flags: input.flags | 0x10,
        notice_troops: (sum as u16).wrapping_mul(5),
    })
}
