//! Orden y reprogramación de la tarea de clima diario `0x004E4984`.
use crate::scheduler::TaskRecord;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WeatherStep {
    Ice,
    Seasonal,
    TownMarker,
    SummerPhenomena,
    Icebergs,
    StormMarkers,
}
/// Calcula los servicios invocados y actualiza los dos campos de la tarea.
/// No ejecuta los servicios: el anfitrión debe preparar todos sus efectos antes
/// de confirmar la transacción.
pub fn daily_plan(task: &mut TaskRecord, day_of_year: u16, network_client: u32) -> Vec<WeatherStep> {
    let mut steps = Vec::with_capacity(4);
    if (0x3b..0x14d).contains(&day_of_year) {
        steps.push(WeatherStep::Seasonal)
    } else {
        steps.push(WeatherStep::Ice)
    }
    if day_of_year % 4 == 0 {
        steps.push(WeatherStep::TownMarker)
    }
    if network_client == 0 {
        if (0x5a..0x14e).contains(&day_of_year) {
            if day_of_year % 3 == 0 {
                steps.push(WeatherStep::SummerPhenomena)
            }
        } else if (u32::from(day_of_year) + 2) % 3 == 0 {
            steps.push(WeatherStep::Icebergs)
        }
    }
    steps.push(WeatherStep::StormMarkers);
    task.due = task.due.wrapping_add(0x100);
    task.set_u32_at(8, task.u32_at(8).wrapping_add(1));
    steps
}

/// `0x004F34A4`: ajustes de principio de mes fuera de invierno.
pub fn seasonal_ice(day: u8, month: u8, towns: &mut [crate::town::Town]) {
    if day != 1 {
        return;
    }
    let value = match month {
        4 => Some(0),
        9 => Some(1),
        2 => Some(6),
        _ => None,
    };
    if let Some(value) = value {
        for t in towns {
            t.0[0x9bc] = value;
            if month == 2 {
                t.0[0x9bd] = 1
            }
        }
    }
}

/// `0x004F3264`: bit de tormenta próxima en cada ciudad. Las ubicaciones
/// se indexan por los identificadores que conserva GameWorld, no por el
/// orden de las ciudades. Solo lee el gestor naval y no modifica C07.
pub fn update_storm_markers(
    towns: &mut [crate::town::Town],
    town_ids: &[u8],
    locations: &[crate::ai::settlement::SettlementLocation],
    manager: &[u8],
) -> Result<(), &'static str> {
    if manager.len() < 0xe0
        || town_ids.len() < towns.len()
        || town_ids[..towns.len()]
            .iter()
            .any(|&i| usize::from(i) >= locations.len())
    {
        return Err("datos de tormenta urbana incompletos");
    }
    for (town, &id) in towns.iter_mut().zip(town_ids) {
        let loc = locations[usize::from(id)];
        let mut near = false;
        for i in 0..16 {
            let at = 0x20 + i * 12;
            let word = |j| u16::from_le_bytes([manager[j], manager[j + 1]]);
            if word(at) == u16::MAX || word(at + 8) != 0 {
                continue;
            }
            let x = i32::from(word(at + 4) as i16);
            let y = i32::from(word(at + 6) as i16);
            if loc.x.wrapping_sub(x).wrapping_abs() <= 150 && loc.y.wrapping_sub(y).wrapping_abs() <= 150 {
                near = true;
                break;
            }
        }
        if near {
            town.0[0x9c4] |= 2
        } else {
            town.0[0x9c4] &= !2
        }
    }
    Ok(())
}

/// `0x004F3354`: cambia la ciudad marcada con un LCG local sembrado por
/// Town[0]+0x124. El CRT global no se consume. El original no sale si solo
/// existe una ciudad y ya estaba marcada: aquí se rechaza sin cambios.
pub fn update_town_marker(
    towns: &mut [crate::town::Town],
    marker: &mut u8,
    day_of_year: u16,
) -> Result<(), &'static str> {
    let old = *marker;
    let count = towns.len();
    if !(0x3a..0x14e).contains(&day_of_year) {
        if usize::from(old) < count {
            towns[usize::from(old)].0[0x9c4] &= !1;
            *marker = 0xff
        }
        return Ok(());
    }
    if count == 0 || count > 40 {
        return Err("ciudades de marcador meteorológico inválidas");
    }
    if count == 1 && old == 0 {
        return Err("marcador meteorológico sin alternativa");
    }
    let mut rng = crate::rng::LcgRandom::seeded(towns[0].u32(0x124));
    let mut selected = None;
    for _ in 0..1_000_000 {
        let value = rng.next_below(count as u32) as u8;
        if value != old {
            selected = Some(value);
            break;
        }
    }
    let next = selected.ok_or("generador meteorológico sin salida")?;
    if usize::from(old) < count {
        towns[usize::from(old)].0[0x9c4] &= !1
    }
    towns[usize::from(next)].0[0x9c4] |= 1;
    *marker = next;
    Ok(())
}

/// Puerto que se congela y momento en que debe ejecutarse la tarea 0x35.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FreezePort {
    pub city: u16,
    pub due: u32,
}

/// `0x004E45C4`: acumulación y deshielo diario con el LCG local original.
/// Devuelve las tareas de deshielo que el anfitrión debe insertar y los avisos
/// asociados. El error conserva las imágenes originales de las ciudades.
pub fn ice_tick(
    towns: &mut [crate::town::Town],
    ids: &[u8],
    locations: &[crate::ai::settlement::SettlementLocation],
    day: u16,
    tick: u32,
    speed: u32,
) -> Result<Vec<FreezePort>, &'static str> {
    if towns.len() < 10
        || ids.len() < towns.len()
        || ids[..towns.len()].iter().any(|&i| usize::from(i) >= locations.len())
    {
        return Err("datos de heladas incompletos");
    }
    let seed = (towns[9].u16(0x304) as i16 as i32 as u32)
        .wrapping_add(towns[7].u32(0x31c))
        .wrapping_add(towns[3].u32(0x2e0))
        .wrapping_add(towns[0].u32(0x14));
    let mut rng = crate::rng::LcgRandom::seeded(seed);
    let mut staged = towns.to_vec();
    let mut frozen = Vec::new();
    let winter = day < 0x3a || day >= 0x14d;
    for (city, t) in staged.iter_mut().enumerate() {
        let prior = t.0[0x9bd];
        t.0[0x9bc] = if (prior as i8) < 0 { 5 } else { prior };
        let loc = locations[usize::from(ids[city])];
        let factor = (loc.x as u32)
            .wrapping_sub((loc.y as u32).wrapping_mul(2))
            .wrapping_add(0xe10)
            / 50;
        if day > 0x39 && day < 0x14d {
            t.set_u32(0x9b8, 0);
            t.set_u32(0x9c0, 0);
            t.0[0x9c4] &= !4
        } else if (0x20..=0x39).contains(&day) {
            let value = t.u32(0x9b8);
            if value != 0 {
                if factor == 0 {
                    return Err("divisor geográfico de heladas nulo");
                }
                let random = rng.next_below(factor);
                let adjustment = crate::numeric::ftol_i32(f64::from(factor) * -2.5) as u32;
                let next = value.wrapping_sub(random.wrapping_sub(adjustment) / 3);
                t.set_u32(0x9b8, if (next as i32) < 0 { 0 } else { next });
            }
        } else {
            let value = t.u32(0x9b8).wrapping_add(rng.next_below(60).wrapping_add(factor) / 3);
            t.set_u32(0x9b8, value)
        }
        let timer = t.u32(0x9c0);
        if timer > 0 {
            let next = timer.wrapping_sub(0x100);
            if next as i32 <= 0 {
                t.set_u32(0x9c0, 0);
                t.0[0x9c4] &= !4
            } else {
                t.set_u32(0x9c0, next);
                t.0[0x9c4] |= 4
            }
        }
        let accum = t.u32(0x9b8);
        if accum == 0 && !winter {
            t.0[0x9bd] = 1;
            continue;
        }
        t.0[0x9c4] &= !1;
        let level = ((accum >> 9) as u8).wrapping_add(2);
        let cold = level > 5;
        t.0[0x9bd] = if cold { level | 0x80 } else { level };
        if t.0[0x9bd] & 0x7f > t.0[0x9bc] {
            let mut duration = rng
                .next_below(100)
                .wrapping_add(factor.wrapping_mul(4))
                .wrapping_add(0x100);
            if (3..5).contains(&speed) {
                let mut units = duration >> 8;
                if units as u8 != 0 {
                    units = units.wrapping_add(1)
                }
                let divisor = if speed == 3 { 30 } else { 20 };
                duration = (60u32.wrapping_mul(units) / divisor).wrapping_shl(8)
            }
            t.set_u32(0x9c0, duration);
            t.0[0x9c4] |= 4
        }
        if cold && t.flags() & 0x0400_0000 == 0 && rng.next_below(0x6400) < 0x180 {
            t.set_flags(t.flags() | 0x0400_0000);
            let due = tick.wrapping_add(u32::from((t.0[0x9bd] & 0x7f).wrapping_add(1)) << 8);
            frozen.push(FreezePort { city: city as u16, due })
        }
    }
    towns.clone_from_slice(&staged);
    Ok(frozen)
}

/// `spawn_winter_icebergs` (0x004E43E4), con la tabla original solapada de
/// palabras. La rejilla y el azar CRT pertenecen al anfitrión; las operaciones
/// devueltas aún necesitan despacharse al gestor naval.
pub fn winter_icebergs<F>(
    day: u16,
    rng: &mut crate::rng::CrtRand,
    manager: &[u8],
    ships: &[u8],
    mut navigable: F,
) -> Result<Vec<crate::pump::Operation>, &'static str>
where
    F: FnMut(i16, i16) -> bool,
{
    const TABLE: [i16; 12] = [30, 90, 330, 0, 0, 152, 36, 36, 90, 364, 69, 37];
    if manager.len() < 0xf6 {
        return Err("gestor naval de icebergs truncado");
    };
    let count = usize::from(u16::from_le_bytes([manager[0xf4], manager[0xf5]]));
    if ships.len() < count * 0x180 {
        return Err("barcos de icebergs truncados");
    }
    let mut random = *rng;
    let mut ops = Vec::new();
    for i in 0..6 {
        let probability = i32::from(TABLE[i]).wrapping_mul(32767);
        let roll = random.rand().wrapping_mul(1024);
        if probability < roll {
            continue;
        }
        let day = i32::from(day);
        if day > i32::from(TABLE[i + 1]) && day < i32::from(TABLE[i + 2]) {
            break;
        }
        let mut found = None;
        for _ in 0..24 {
            let base_x = TABLE[i + 3].wrapping_mul(3);
            let base_y = TABLE[i + 4].wrapping_mul(3);
            let dx = random.rand().wrapping_mul(i32::from(TABLE[i + 5])).wrapping_mul(3) / 32768;
            let dy = random.rand().wrapping_mul(i32::from(TABLE[i + 6])).wrapping_mul(3) / 32768;
            let x = base_x.wrapping_add(dx as i16);
            let y = base_y.wrapping_add(dy as i16);
            if !navigable(x, y) {
                continue;
            }
            let x = x.wrapping_add(0x44);
            let y = y.wrapping_add(0x44);
            let mut near = false;
            for k in 0..16 {
                let off = 0x20 + k * 12;
                let id = usize::from(u16::from_le_bytes([manager[off], manager[off + 1]]));
                if id >= count {
                    continue;
                }
                let at = id * 0x180;
                let sx = i16::from_le_bytes([ships[at + 0x1e], ships[at + 0x1f]]);
                let sy = i16::from_le_bytes([ships[at + 0x22], ships[at + 0x23]]);
                let dx = i32::from(x) - i32::from(sx);
                let dy = i32::from(y) - i32::from(sy);
                if dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy)) < 5000 {
                    near = true;
                    break;
                }
            }
            if !near {
                found = Some((x, y));
                break;
            }
        }
        let Some((x, y)) = found else { break };
        let life = 0xa00u32 + (random.rand() as u32 * 0x500) / 32767;
        ops.push(crate::pump::Operation([
            0x78,
            2,
            life,
            x as i32 as u32,
            y as i32 as u32,
        ]));
    }
    *rng = random;
    Ok(ops)
}

/// `spawn_summer_weather` (0x004E3F44). Devuelve como máximo una operación
/// 0x78; la consulta de navegación es la misma del planificador naval C07.
pub fn summer_weather<F>(
    day: u16,
    queue_count: u16,
    scale: i32,
    rng: &mut crate::rng::CrtRand,
    manager: &[u8],
    ships: &[u8],
    mut navigable: F,
) -> Result<Vec<crate::pump::Operation>, &'static str>
where
    F: FnMut(i16, i16) -> bool,
{
    const POINTS: [(i16, i16, i16, i16); 15] = [
        (74, 312, 117, 38),
        (72, 352, 39, 47),
        (184, 136, 87, 47),
        (240, 184, 46, 150),
        (288, 280, 158, 40),
        (56, 80, 112, 47),
        (56, 128, 128, 182),
        (320, 320, 38, 22),
        (344, 248, 94, 30),
        (392, 136, 54, 110),
        (425, 81, 45, 55),
        (472, 81, 38, 14),
        (504, 57, 38, 22),
        (0, 0, 167, 80),
        (392, 0, 30, 54),
    ];
    if manager.len() < 0xf6 {
        return Err("gestor naval de tormentas truncado");
    }
    let count = usize::from(u16::from_le_bytes([manager[0xf4], manager[0xf5]]));
    if ships.len() < count * 0x180 {
        return Err("barcos de tormentas truncados");
    }
    let mut random = *rng;
    let choice = (random.rand() * 14 / 32768) as usize;
    let phase = ((f32::from(day) * f32::from_bits(0x3de38e39)).to_bits() & 0xf0) != 0;
    let kind = if phase { 0 } else { 1 };
    let limit = if (phase && choice < 5) || (!phase && choice < 2) {
        f32::from_bits(0x3eaa7efa)
    } else if (phase && choice < 13) || (!phase && choice < 10) {
        0.5
    } else {
        1.0
    };
    if 52u32.wrapping_sub(u32::from(queue_count)) < 5 {
        *rng = random;
        return Ok(Vec::new());
    }
    let roll = random.rand();
    if f64::from(limit) * 32767.0 < f64::from(roll) {
        *rng = random;
        return Ok(Vec::new());
    }
    let point_index = if phase {
        choice
    } else {
        [13, 14, 5, 6, 7, 8, 9, 10, 11, 12, 0, 1, 2, 3, 4][choice]
    };
    let (bx, by, w, h) = POINTS[point_index];
    let life = 0xa00 + (random.rand() as u32 * 0x500) / 32767;
    for _ in 0..5 {
        let dx = random.rand().wrapping_mul(i32::from(w)) / 32768;
        let dy = random.rand().wrapping_mul(i32::from(h)) / 32768;
        let tx = bx.wrapping_add(dx as i16);
        let ty = by.wrapping_add(dy as i16);
        let world = |p: i16| (((i32::from(p).wrapping_mul(scale).wrapping_add(500)) / 1000).wrapping_add(0x44)) as i16;
        let x = world(tx);
        let y = world(ty);
        if !navigable(x.wrapping_sub(0x44), y.wrapping_sub(0x44)) {
            continue;
        }
        let mut near = false;
        for k in 0..16 {
            let off = 0x20 + k * 12;
            let id = usize::from(u16::from_le_bytes([manager[off], manager[off + 1]]));
            if id >= count {
                continue;
            }
            let at = id * 0x180;
            let sx = i16::from_le_bytes([ships[at + 0x1e], ships[at + 0x1f]]);
            let sy = i16::from_le_bytes([ships[at + 0x22], ships[at + 0x23]]);
            let dx = i32::from(x) - i32::from(sx);
            let dy = i32::from(y) - i32::from(sy);
            if dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy)) < 5000 {
                near = true;
                break;
            }
        }
        if !near {
            *rng = random;
            return Ok(vec![crate::pump::Operation([
                0x78,
                kind,
                life,
                x as i32 as u32,
                y as i32 as u32,
            ])]);
        }
    }
    *rng = random;
    Ok(Vec::new())
}
