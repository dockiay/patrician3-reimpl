//! Dependencias de la preparación de misiones del alcalde.

/// Constructor de Town 0x0051B100. Conserva los huecos que el original no escribe.
/// No asigna buffers: en esta representación los punteros se guardan separados.
pub fn construct_town(town: &mut crate::town::Town, player: u16, home_town: u8, game_mode: u8) {
    let previous_index = town.u8(0x2c0);
    let bytes = &mut town.0;
    for (start, end) in [
        (0, 0x6dc),
        (0x6f3, 0x6f4),
        (0x6fc, 0x700),
        (0x75c, 0x762),
        (0x76c, 0x775),
        (0x776, 0x784),
        (0x786, 0x78b),
        (0x78c, 0x79c),
        (0x79d, 0x7b0),
        (0x7f0, 0x800),
        (0x80c, 0x81c),
        (0x824, 0x82c),
        (0x82d, 0x82e),
        (0x835, 0x836),
        (0x83c, 0x83d),
        (0x83f, 0x994),
        (0x996, 0x9a0),
        (0x9a4, 0x9be),
        (0x9c0, 0x9c5),
        (0x9c6, 0x9ea),
        (0x9ec, 0x9f8),
    ] {
        bytes[start..end].fill(0);
    }
    for (start, end) in [
        (0x6f0, 0x6f3),
        (0x762, 0x76c),
        (0x784, 0x786),
        (0x79c, 0x79d),
        (0x7b0, 0x7f0),
        (0x800, 0x80c),
        (0x81c, 0x822),
        (0x82c, 0x82d),
        (0x82e, 0x835),
        (0x83d, 0x83f),
        (0x994, 0x996),
        (0x9a2, 0x9a4),
    ] {
        bytes[start..end].fill(0xff);
    }
    let council_size = if player > 255 {
        5
    } else if previous_index == home_town && game_mode == 0 {
        7
    } else {
        6
    };
    bytes[0x6e4..0x6e8].copy_from_slice(&[council_size, 5, 4, 4]);
    for offset in [0x786, 0x78c, 0x78e, 0x7a4] {
        bytes[offset] = 1;
    }
    for i in 0..21 {
        bytes[0x846 + 16 * i] = i as u8;
    }
}

/// Longitud de ruta usada en la matriz de ciudades (0x00516930).
/// Una ruta nula devuelve cero; una ruta existente de cero o un punto devuelve uno.
pub fn town_route_length(points: Option<&[(i16, i16)]>) -> i32 {
    let Some(points) = points else { return 0 };
    let mut sum = 0i32;
    for pair in points.windows(2) {
        let dx = (i32::from(pair[0].0) - i32::from(pair[1].0)).abs();
        let dy = (i32::from(pair[0].1) - i32::from(pair[1].1)).abs();
        let distance = dx.max(dy).wrapping_mul(16).wrapping_add(dx.min(dy).wrapping_mul(7));
        sum = sum.wrapping_add(distance)
    }
    let scaled = sum.wrapping_mul(32) / 0x208;
    scaled.wrapping_add(0x120) >> 8
}

/// Reconstruye o amplía la matriz de distancias (0x00531BD0). El planificador
/// entrega la ruta usada para cada par y el orden de llamadas es el original.
pub fn build_town_distance_matrix<F>(
    old: Option<&[u8]>,
    town_ids: &[u8],
    locations: &[SettlementLocation; 40],
    mut route: F,
) -> Result<Vec<u8>, &'static str>
where
    F: FnMut((i16, i16), (i16, i16)) -> Result<Option<Vec<(i16, i16)>>, &'static str>,
{
    let n = town_ids.len();
    if n == 0 || n > 40 || town_ids.iter().any(|&id| id >= 40) {
        return Err("lista de ciudades inválida");
    }
    let mut matrix = vec![0; n * n];
    let first = if let Some(old) = old {
        let count = n - 1;
        if old.len() != count * count {
            return Err("matriz anterior inválida");
        }
        for row in 0..count {
            matrix[row * n..row * n + count].copy_from_slice(&old[row * count..(row + 1) * count])
        }
        count
    } else {
        0
    };
    for i in first..n {
        matrix[i * n + i] = 0;
        let a = locations[usize::from(town_ids[i])];
        for j in 0..i {
            let b = locations[usize::from(town_ids[j])];
            let points = route((a.x as i16, a.y as i16), (b.x as i16, b.y as i16))?;
            let distance = town_route_length(points.as_deref()) as u8;
            matrix[i * n + j] = distance;
            matrix[j * n + i] = distance;
        }
    }
    Ok(matrix)
}

/// Puente con el planificador vivo; conserva tanto rutas compartidas como
/// calculadas y revierte la modificación de rutas compartidas si falla.
pub fn build_town_distance_matrix_with_planner(
    old: Option<&[u8]>,
    town_ids: &[u8],
    locations: &[SettlementLocation; 40],
    planner: &mut crate::route::finder::RoutePlanner,
) -> Result<Vec<u8>, &'static str> {
    use crate::route::{finder::RouteOut, planner::find_town_at_point};
    let n = town_ids.len();
    if planner.town_count as usize != n
        || planner.town_order.len() < n
        || planner
            .town_order
            .iter()
            .take(n)
            .any(|&i| usize::from(i) >= planner.town_slots.len())
        || planner.town_table.len() < n * n
    {
        return Err("tabla de rutas incompleta");
    }
    let mut staged = planner.clone();
    let matrix = build_town_distance_matrix(old, town_ids, locations, |a, b| {
        let x = find_town_at_point(&staged.town_slots, &staged.town_order, staged.town_count, a, None);
        let y = find_town_at_point(&staged.town_slots, &staged.town_order, staged.town_count, b, None);
        if x < staged.town_count && y < staged.town_count {
            let slot = (x as usize) * n + y as usize;
            let id = staged
                .town_table
                .get(slot)
                .and_then(|v| *v)
                .ok_or("ruta compartida ausente")?;
            if staged.routes.get(id).is_none_or(Vec::is_empty) {
                return Err("descriptor de ruta ausente");
            }
        } else {
            let grid = &staged.nav.grid;
            if grid.scale <= 0
                || grid.width <= 0
                || grid.height <= 0
                || grid.cells.len() < (grid.width as usize) * (grid.height as usize)
            {
                return Err("rejilla de navegación incompleta");
            }
            if a == b {
                return Err("extremos de ruta coincidentes fuera de puerto");
            }
        }
        match staged.find_route(&[a, b]).route {
            RouteOut::Shared(id) => Ok(Some(staged.routes[id].clone())),
            RouteOut::Built(points) => Ok(Some(points)),
            RouteOut::Unset => Err("el planificador no devolvió ruta"),
        }
    })?;
    *planner = staged;
    Ok(matrix)
}

/// Valores predeterminados de instalaciones, oferta e historial (0x00545B20).
pub fn init_town_defaults(town: &mut crate::town::Town) {
    for i in 0..24 {
        town.set_u32(0x64 + 4 * i, 1);
        town.set_u32(0xc4 + 4 * i, 1);
        town.set_u32(0x310 + 4 * i, 1);
        for (j, value) in [1, 2, 3, 4].into_iter().enumerate() {
            town.set_u32(0x4f0 + 16 * i + 4 * j, value)
        }
    }
    town.set_u16(0x996, 0);
    town.set_u16(0x994, u16::MAX);
    town.set_u16(0x2c6, 0x28);
    for offset in [0x9a4, 0x9a8, 0x9ac, 0x9b0] {
        town.set_u32(offset, 0)
    }
    for i in 0..21 {
        let p = 0x840 + 16 * i;
        town.set_u8(p + 6, i as u8);
        town.set_u8(p + 7, town.u8(0x2c0));
        town.set_u16(p + 10, 0);
        town.set_u32(p + 12, 0)
    }
    town.set_u16(0x9a2, u16::MAX);
    town.set_u32(0x300, 0x00320032);
    town.set_u32(0x304, 0x00320032);
}

/// Parte previa a la carga del plano de 0x0051E8D0; el llamador debe ejecutar
/// después `Site::plan_load_from_file` con el recurso de la ciudad elegida.
pub fn reset_town_buildings(town: &mut crate::town::Town, site: &mut crate::buildings::Site) {
    site.records.clear();
    town.set_u32(0x75c, 0);
    for offset in [0x762, 0x764, 0x768, 0x784] {
        town.set_u16(offset, u16::MAX)
    }
    for offset in [0x78c, 0x78e, 0x786] {
        town.set_u16(offset, 1)
    }
    for offset in [0x760, 0x776, 0x790, 0x792] {
        town.set_u16(offset, 0)
    }
    for offset in [0x774, 0x770, 0x771, 0x772, 0x773, 0x789, 0x788, 0x78a] {
        town.set_u8(offset, 0)
    }
    town.set_u32(0x76c, 0);
}

/// Experiencia y niveles iniciales del astillero (0x00545BE0).
pub fn init_town_shipyard(town: &mut crate::town::Town, year: i32) {
    const THRESHOLDS: [[u32; 4]; 4] = [
        [0, 100, 300, 1050],
        [0, 100, 600, 900],
        [0, 200, 400, 800],
        [300, 500, 600, 1200],
    ];
    let value = if year < 1350 {
        0
    } else if year > 1400 {
        1_400_000
    } else {
        ((year - 1350) * 28_000) as u32
    };
    town.set_u32(0x810, value);
    town.set_u32(0x814, 0);
    town.set_u32(0x818, 0);
    town.set_u16(0x81c, u16::MAX);
    town.set_u16(0x81e, u16::MAX);
    for i in (0..4).rev() {
        let mut level = -1i8;
        for threshold in THRESHOLDS[i] {
            if value < threshold * 2800 {
                break;
            }
            level += 1
        }
        town.set_u8(0x824 + i, level as u8);
        town.set_u8(0x828 + i, 0)
    }
}

/// Pool de capitanes usado por las tabernas y la fábrica naval.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TavernCaptainPool {
    pub captains: Vec<[u8; 16]>,
    pub free_head: u16,
}

/// Añade un capitán a la cabeza de la taberna (0x00526A50 y 0x005097C0).
#[allow(clippy::too_many_arguments)]
pub fn add_tavern_captain(
    town: &mut crate::town::Town,
    pool: &mut TavernCaptainPool,
    pirate: bool,
    tick: u32,
    day_of_year: u16,
    day: u8,
    population: u32,
    first_names: u16,
    last_names: u16,
) -> Result<u16, &'static str> {
    if first_names == 0 || last_names == 0 || pool.captains.len() > usize::from(u16::MAX) {
        return Err("pool o banco de nombres inválido");
    }
    let mut staged = pool.clone();
    let free = usize::from(staged.free_head);
    if free > staged.captains.len() {
        return Err("cabeza libre de capitán inválida");
    }
    if free == staged.captains.len() {
        let end = free
            .checked_add(64)
            .filter(|&n| n <= usize::from(u16::MAX))
            .ok_or("pool de capitanes lleno")?;
        for i in free..end {
            let mut cap = [0xff; 16];
            cap[..2].copy_from_slice(&((i + 1) as u16).to_le_bytes());
            staged.captains.push(cap)
        }
    }
    let cap = &mut staged.captains[free];
    staged.free_head = u16::from_le_bytes([cap[0], cap[1]]);
    cap[..2].fill(0xff);
    crate::ships::factory::init_captain(
        cap,
        free as u32,
        !pirate,
        tick,
        day_of_year,
        day,
        population,
        first_names,
        last_names,
    )
    .ok_or("capitán inválido")?;
    cap[..2].copy_from_slice(&town.u16(0x82e).to_le_bytes());
    if !pirate {
        let wage = (u16::from(cap[9]) + u16::from(cap[10]) + u16::from(cap[11])) / 50 + 10 + u16::from(cap[8] % 11);
        cap[12..14].copy_from_slice(&wage.to_le_bytes())
    }
    town.set_u16(0x82e, free as u16);
    *pool = staged;
    Ok(free as u16)
}

/// Inicio de una ciudad fundada durante la partida (0x005458D0, modo 0).
/// Las entradas de suministro se devuelven en orden de lista, sin punteros x86.
pub fn init_founded_town(
    town: &mut crate::town::Town,
    site: &crate::buildings::Site,
    town_count: u32,
    full: u32,
    half: u32,
) -> Result<Vec<crate::economy::world::TickEntry>, &'static str> {
    const SCALE: [u32; 17] = [
        2048, 942, 1638, 2048, 1229, 1331, 1638, 1229, 1536, 1024, 1024, 1229, 4096, 1024, 1024, 1024, 1024,
    ];
    let rows = town.i32(0x7a8);
    let cols = town.i32(0x7ac);
    if rows < 0 || cols < 0 || i64::from(rows) * i64::from(cols) > site.cells.len() as i64 {
        return Err("plano de ciudad inválido");
    }
    let divisor = town_count.wrapping_mul(3) >> 4;
    if divisor == 0 && full & 0x1c0000 != 0 {
        return Err("división por cero original al crear suministros");
    }
    init_town_defaults(town);
    for i in 0..4 {
        town.set_u16(0x848 + 16 * i, 0x400)
    }
    for i in 0..17 {
        let bit = 1u32 << i;
        town.set_u16(
            0x888 + 16 * i,
            if full & bit != 0 {
                0x400
            } else if half & bit != 0 {
                0x300
            } else {
                0
            },
        )
    }
    if full & 0x20000 != 0 {
        town.set_u16(0x898, 0x300);
        town.set_u32(0x2cc, 0x400)
    } else if half & 0x20000 != 0 {
        town.set_u16(0x898, 0x300);
        town.set_u32(0x2cc, 0x300)
    } else {
        town.set_u32(0x2cc, 0)
    }
    let mut entries = Vec::new();
    for (bit, kind, multiplier) in [(0x40000, 4u8, 200u32), (0x80000, 16, 500), (0x100000, 13, 700)] {
        if full & bit != 0 {
            let rate = ((1024 / divisor).wrapping_mul(multiplier) >> 10) as u16;
            let rate = rate.min(1024);
            entries.insert(
                0,
                crate::economy::world::TickEntry {
                    amount: 0,
                    sum: 0,
                    max: rate,
                    factor: rate,
                    facility_type: kind,
                    town: town.u8(0x2c0),
                },
            )
        }
    }
    if full & 0x200000 != 0 {
        town.set_u32(0x2c8, town.u32(0x2c8) | 0x08000000)
    }
    town.set_u16(0x9a0, 10);
    town.set_u16(0x844, 0);
    town.set_u32(0x998, 0);
    town.set_u32(0x99c, 0);
    for offset in [0x850, 0x860, 0x870] {
        town.set_u32(offset, 0x400)
    }
    town.set_u16(0x854, 0);
    town.set_u16(0x864, 5);
    town.set_u16(0x874, 2);
    for (i, &scale) in SCALE.iter().enumerate() {
        let p = 0x880 + 16 * i;
        let efficiency = u32::from(town.u16(p + 8));
        town.set_u16(p + 4, 0);
        town.set_u32(p, (efficiency * scale) >> 10)
    }
    if town.u32(0x2cc) > 0 {
        town.set_u32(0x890, u32::from(town.u16(0x898)))
    }
    town.set_u32(0x2d4, 38);
    for (i, value) in [4, 8, 16, 10].into_iter().enumerate() {
        town.set_u32(0x2d8 + 4 * i, value)
    }
    for i in 0..24 {
        town.set_u32(4 + 4 * i, 0)
    }
    let (total, built) = site.count_street_cells(town);
    town.set_u16(0x792, total);
    town.set_u16(0x790, built);
    Ok(entries)
}

/// Inicializa concejales y candidatos (0x005289B0), incluyendo los índices
/// que el original usa en la segunda lista de nombres.
pub fn init_town_council(
    towns: &mut [crate::town::Town],
    city: usize,
    first_count: u16,
    last_count: u16,
    tick: u32,
    mode: u32,
    seed: u32,
    step: u32,
) -> Result<(), &'static str> {
    if city >= towns.len() || !(8..=256).contains(&first_count) || !(8..=256).contains(&last_count) {
        return Err("banco de nombres o ciudad inválidos");
    }
    let others = towns.to_vec();
    let town = &mut towns[city];
    town.set_u32(0x6fc, tick.wrapping_add(seed & 0x3fff));
    town.set_u8(0x6f3, 0);
    let mut value = seed;
    for base in [0x6d4usize, 0x6d8] {
        let count = if base == 0x6d4 {
            u32::from(first_count)
        } else {
            u32::from(last_count)
        };
        for i in 0..4 {
            value %= count;
            let mut k = 0usize;
            while k < i {
                if town.u8(base + k) == value as u8 {
                    value += 1;
                    if value >= count {
                        value = 0
                    }
                    k = 0
                } else {
                    k += 1
                }
            }
            town.set_u8(base + i, value as u8);
            value = value.wrapping_add(step).wrapping_add(i as u32)
        }
    }
    let mut toggle = true;
    let mut checked = 0u32;
    'members: for i in 0..4 {
        let mut cursor = others.len();
        while cursor > 0 {
            cursor -= 1;
            if cursor == usize::from(town.u8(0x2c0)) {
                continue;
            }
            let other = &others[cursor];
            for j in 0..4 {
                if other.u8(0x6d4 + j) == town.u8(0x6d4 + i) && other.u8(0x6d8 + j) == town.u8(0x6d8 + i) {
                    if toggle {
                        let v = town.u8(0x6d4 + i).wrapping_add(1);
                        town.set_u8(0x6d4 + i, if v >= 0x50 { 0 } else { v })
                    } else {
                        let v = town.u8(0x6d8 + i).wrapping_add(1);
                        town.set_u8(0x6d8 + i, if u16::from(v) >= last_count { 0 } else { v })
                    }
                    toggle = !toggle;
                    cursor = others.len()
                }
            }
            if checked > 100000 {
                break 'members;
            }
            checked += 1;
        }
    }
    town.set_u32(0x6dc, u32::MAX);
    town.set_u8(0x6e0, 3);
    for i in 1..4 {
        town.set_u8(0x6e0 + i, 2)
    }
    for i in 0..4 {
        value %= u32::from(last_count);
        town.set_u8(0x6e8 + i, value as u8);
        let mut k = 0usize;
        while k < i {
            if town.u8(0x6e4 + k) == value as u8 {
                value += 1;
                if value >= u32::from(first_count) {
                    value = 0
                }
                k = 1
            } else {
                k += 1
            }
        }
        value = value.wrapping_add(step).wrapping_add(i as u32)
    }
    for i in 0..4 {
        value %= u32::from(last_count);
        let mut k = 0usize;
        while k < 4 + i {
            let previous = if k < 4 { town.u8(0x6d8 + k) } else { town.u8(0x6e8 + k) };
            if previous == value as u8 {
                value += 1;
                if value >= u32::from(last_count) {
                    value = 0
                }
                k = 1
            } else {
                k += 1
            }
        }
        town.set_u8(0x6ec + i, value as u8);
        value = value.wrapping_add(step).wrapping_add(i as u32)
    }
    for i in 0..4 {
        town.set_u8(0x6e4 + i, 5)
    }
    town.set_u32(0x6f4, 0);
    town.set_u32(0x6f8, 10);
    town.set_u32(0x700, if mode == 0 { 0 } else { 250000 });
    town.0[0x704..0x75c].fill(0);
    Ok(())
}

/// `find_next_unused_map_location` (0x00515A60): busca una de las 40
/// posiciones, avanzando desde `start` y dando una vuelta como máximo. Si
/// todas están ocupadas, el original devuelve cero. Los cinco destinos de
/// bandas piratas ausentes se expresan como `None`.
pub fn find_next_unused_map_location(
    start: u32,
    town_ids: &[u8],
    pirate_targets: &[Option<u8>; 5],
) -> Result<u8, &'static str> {
    if town_ids.len() > 40 || town_ids.iter().any(|&id| id >= 40) {
        return Err("lista de ciudades fuera del mapa");
    }
    let mut occupied = [false; 40];
    for &id in town_ids {
        occupied[usize::from(id)] = true
    }
    for id in pirate_targets.iter().flatten() {
        if *id < 40 {
            occupied[usize::from(*id)] = true
        }
    }
    let begin = if start < 40 { start as usize } else { 0 };
    for offset in 0..40 {
        let i = (begin + offset) % 40;
        if !occupied[i] {
            return Ok(i as u8);
        }
    }
    Ok(0)
}

/// Imagen de una posición del mapa global en 0x006DDBB0 (registros de 52 bytes).
#[derive(Clone, Copy, Debug)]
pub struct SettlementLocation {
    pub x: i32,
    pub y: i32,
    pub kind: u8,
}

/// Parámetros de la operación de fundación sintetizada por la misión 0x32/0.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FoundMissionRequest {
    pub full: u32,
    pub map_id: u8,
    pub plan_id: u8,
    pub city: u8,
    pub owner: u8,
}

/// Parte anterior a `0x00531D50` en el inicio de la misión de fundación.
pub fn begin_founding_mission(
    task: &mut crate::scheduler::TaskRecord,
    city: u8,
    merchant_count: u16,
) -> Result<FoundMissionRequest, &'static str> {
    if task.kind != 0x32
        || task.u8_at(0xc) != 0
        || task.u8_at(0xf) != 0
        || u16::from(task.u8_at(0xd)) >= merchant_count
        || city >= 40
        || task.u8_at(0x14) >= 40
    {
        return Err("misión de fundación incompatible");
    }
    let request = FoundMissionRequest {
        full: task.u32_at(0x10),
        map_id: task.u8_at(0x14),
        plan_id: task.u8_at(0x15),
        city,
        owner: task.u8_at(0xd),
    };
    task.set_u8_at(0xf, 1);
    task.set_u8_at(0x16, city);
    Ok(request)
}

/// Parte posterior al alta: asigna el alcalde y programa la tarea urbana 0x39.
pub fn finish_founding_mission(
    scheduler: &mut crate::scheduler::Scheduler,
    index: u16,
    town: &mut crate::town::Town,
    tick: u32,
) -> Result<(), &'static str> {
    let task = scheduler
        .records
        .get(usize::from(index))
        .filter(|_| index < scheduler.capacity)
        .ok_or("misión ausente")?;
    if task.kind != 0x32 || task.u8_at(0xc) != 0 || task.u8_at(0xf) != 1 || town.u8(0x2c0) != task.u8_at(0x16) {
        return Err("ciudad y misión incompatibles");
    }
    if scheduler.free_head >= scheduler.capacity && scheduler.capacity > u16::MAX - 128 {
        return Err("planificador lleno");
    }
    let owner = task.u8_at(0xd);
    let city = task.u8_at(0x16);
    let id = scheduler.add_task(tick.wrapping_add(0x100));
    let r = &mut scheduler.records[usize::from(id)];
    r.kind = 0x39;
    r.set_u8_at(8, city);
    scheduler.dispatching = 1;
    town.set_u8(0x6f1, owner);
    Ok(())
}

/// Tareas programadas por `0x00531D50` al crear o reinicializar una ciudad.
/// La primera es la elección para ciudades hanseáticas; la segunda es el
/// mantenimiento inicial en modo partida. Solo publica al terminar ambas.
pub fn schedule_founded_town(
    scheduler: &mut crate::scheduler::Scheduler,
    city: u8,
    flags: u32,
    mode: u8,
    half: u32,
    tick: u32,
    year: u16,
) -> Result<(), &'static str> {
    use crate::calendar::{Calendar, TICKS_PER_YEAR};
    if city >= 40 || tick > i32::MAX as u32 - TICKS_PER_YEAR - 0xb400 {
        return Err("fecha o ciudad no admitida");
    }
    if scheduler.records.len() < usize::from(scheduler.capacity) {
        return Err("planificador truncado");
    }
    let mut ids = Vec::new();
    let mut seen = vec![false; usize::from(scheduler.capacity)];
    let mut cursor = scheduler.head;
    while cursor < scheduler.capacity {
        let i = usize::from(cursor);
        if seen[i] {
            return Err("lista de tareas cíclica");
        }
        seen[i] = true;
        ids.push(i);
        cursor = scheduler.records[i].next;
    }
    let mut staged = scheduler.clone();
    if flags & 0x40 != 0 {
        let mut day = (3 * (u32::from(city) + 1) + (half >> 22)) % 365;
        let mut changes = 0;
        loop {
            let mut hit = false;
            for &i in &ids {
                let r = &scheduler.records[i];
                if r.kind != 2 {
                    continue;
                }
                let other = u32::from(u16::from_le_bytes([r.u8_at(8), r.u8_at(9)]));
                if other <= day + 4 && other + 4 >= day {
                    day += 9;
                    if day >= 365 {
                        day -= 365
                    }
                    hit = true;
                    changes += 1;
                    break;
                }
            }
            if !hit {
                break;
            }
            if changes > 365 * ids.len().max(1) {
                return Err("sin fecha libre para elección");
            }
        }
        let days = u32::from(year) * 365 + day;
        if days > i32::MAX as u32 >> 8 {
            return Err("año de elección no admitido");
        }
        let mut due = days << 8;
        if due < tick + 0xa00 {
            due = due
                .checked_add(TICKS_PER_YEAR)
                .filter(|&v| v <= i32::MAX as u32)
                .ok_or("fecha de elección fuera de rango")?
        }
        if staged.free_head >= staged.capacity && staged.capacity > u16::MAX - 128 {
            return Err("planificador lleno");
        }
        let id = staged.add_task(due);
        let r = &mut staged.records[usize::from(id)];
        let c = Calendar::for_tick(due);
        r.kind = 2;
        r.set_u8_at(8, c.day_of_year as u8);
        r.set_u8_at(9, (c.day_of_year >> 8) as u8);
        r.set_u8_at(0xe, city);
        r.set_u8_at(0xf, c.day);
        r.set_u8_at(0x10, c.month + 1);
    }
    if mode == 0 {
        if staged.free_head >= staged.capacity && staged.capacity > u16::MAX - 128 {
            return Err("planificador lleno");
        }
        let id = staged.add_task(tick.wrapping_add(0xb400));
        let r = &mut staged.records[usize::from(id)];
        r.kind = 0x2f;
        r.set_u32_at(8, 0);
        r.set_u32_at(0xc, u32::from(city));
        r.set_u32_at(0x10, 0);
        r.set_u32_at(0x14, tick);
    }
    *scheduler = staged;
    Ok(())
}

/// Inventarios de una oficina y enlace a la siguiente oficina de la ciudad.
#[derive(Clone, Debug)]
pub struct SettlementOffice {
    pub stock: [i32; 20],
    pub demand: [i32; 20],
    pub next: u16,
}

/// Estado leído al validar tareas de misión 0x32.
pub struct AldermanMissionWorld<'a> {
    pub merchant_count: u16,
    pub merchants: &'a [crate::ai::reputation::Merchant],
    pub tick: u32,
    pub day_of_year: u16,
    pub notification_window_active: bool,
    pub towns: &'a [crate::town::Town],
    pub town_ids: &'a [u8],
    pub ships: &'a [Vec<u8>],
    pub captains: &'a [[u8; 16]],
    pub pirate_targets: &'a [Option<u8>; 5],
}

/// Efectos de `scheduled_tasks_alderman_mission_completed` (0x004EC354).
pub struct AldermanCompletion {
    pub owner: usize,
    pub merchant: crate::ai::reputation::Merchant,
    pub letter: [u8; 16],
    pub direct: bool,
    pub notification: Option<[u32; 6]>,
    pub dialog_requested: bool,
}

pub struct AldermanHumanFailure {
    pub owner: usize,
    pub recipient: usize,
    pub player: u32,
    pub merchant: crate::ai::reputation::Merchant,
    pub task: crate::scheduler::TaskRecord,
    pub followup: Option<crate::scheduler::TaskRecord>,
    pub letter: [u8; 16],
    pub notification: [u32; 6],
    pub dialog_requested: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PirateTerminal {
    Wait,
    Success,
    Abort {
        notification: Option<[u32; 6]>,
        dialog_requested: bool,
    },
}

/// Transiciones por desaparición de banda y estados terminales (0x004EB85D).
/// `None` conserva las fases con banda activa aún no cubiertas aquí.
pub fn pirate_mission_terminal(
    task: &mut crate::scheduler::TaskRecord,
    merchant: &mut crate::ai::reputation::Merchant,
    targets: &[Option<u8>; 5],
    mayor_town: u8,
    player: u32,
) -> Result<Option<PirateTerminal>, &'static str> {
    if task.kind != 0x32 || task.u8_at(0xc) != 3 || task.u8_at(0xf) == 0 || task.due >= task.u32_at(8) {
        return Ok(None);
    }
    if merchant.bytes.len() <= 0xe1 {
        return Err("comerciante truncado");
    }
    let phase = task.u8_at(0x12);
    if phase <= 1 {
        let band = usize::from(task.u8_at(0x10));
        if targets.get(band).copied().flatten() == Some(task.u8_at(0x11)) {
            return Ok(None);
        }
        task.set_u8_at(0x12, 3);
        task.due = task.due.wrapping_add(if phase == 0 { 0x100 } else { 1 });
        return Ok(Some(PirateTerminal::Wait));
    }
    merchant.bytes[0xe1] = 0xff;
    if phase == 2 {
        return Ok(Some(PirateTerminal::Success));
    }
    let owner = u32::from(task.u8_at(0xd));
    let human = merchant.u16(8) == 0;
    Ok(Some(PirateTerminal::Abort {
        notification: human.then_some([owner, 0, 0, u32::from(mayor_town), 0x1e, owner]),
        dialog_requested: human && player == owner,
    }))
}

/// Aproximación a una guarida pirata durante la fase 0 de la misión 0x32.
/// Las reposiciones se entregan ordenadas; el llamador debe ejecutarlas sobre
/// copias antes de publicar la tarea y el comerciante.
#[derive(Clone, Debug)]
pub struct PirateMissionApproach {
    pub task: crate::scheduler::TaskRecord,
    pub merchant: crate::ai::reputation::Merchant,
    pub replenishments: Vec<(usize, u32)>,
}
pub fn prepare_pirate_mission_approach(
    task: &crate::scheduler::TaskRecord,
    merchant: &crate::ai::reputation::Merchant,
    targets: &[Option<u8>; 5],
    locations: &[SettlementLocation; 40],
    ships: &[Vec<u8>],
    head: u16,
) -> Result<Option<PirateMissionApproach>, &'static str> {
    if task.kind != 0x32
        || task.u8_at(0xc) != 3
        || task.u8_at(0xf) == 0
        || task.u8_at(0x12) != 0
        || task.due >= task.u32_at(8)
    {
        return Ok(None);
    }
    let target = task.u8_at(0x11);
    if targets.get(usize::from(task.u8_at(0x10))).copied().flatten() != Some(target) {
        return Ok(None);
    }
    let location = locations.get(usize::from(target)).ok_or("destino pirata ausente")?;
    if merchant.bytes.len() <= 0xe1 {
        return Err("comerciante truncado");
    }
    let mut next = *task;
    let mut merchant = merchant.clone();
    let mut delay = 0x40u32;
    let mut seen = vec![false; ships.len()];
    let mut at = usize::from(head);
    let mut calls = Vec::new();
    let mut found = false;
    while at < ships.len() {
        if seen[at] {
            return Err("barcos de comerciante cíclicos");
        }
        seen[at] = true;
        let ship = &ships[at];
        if ship.len() < 0x136 {
            return Err("barco truncado");
        }
        let x = i32::from(i16::from_le_bytes([ship[0x1e], ship[0x1f]]));
        let y = i32::from(i16::from_le_bytes([ship[0x22], ship[0x23]]));
        let dx = x.wrapping_sub(location.x);
        let dy = y.wrapping_sub(location.y);
        let distance = dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy));
        if distance < 200 {
            found = true;
            next.set_u8_at(0x12, 1);
            merchant.bytes[0xe1] = target;
            for (i, band) in targets.iter().enumerate() {
                if band.is_some() {
                    calls.push((i, u32::from(target)))
                }
            }
        } else if distance < 0x1000 {
            let n = if distance > 0x400 {
                22
            } else if distance > 0x200 {
                12
            } else if distance > 0x100 {
                6
            } else {
                2
            };
            delay = delay.min(n)
        }
        at = usize::from(u16::from_le_bytes([ship[4], ship[5]]));
    }
    if !found {
        return Ok(None);
    }
    next.due = next.due.wrapping_add(delay);
    Ok(Some(PirateMissionApproach {
        task: next,
        merchant,
        replenishments: calls,
    }))
}

/// Seguimiento sin encuentro: calcula la próxima consulta de los barcos del
/// comerciante. `None` reserva la aparición de piratas o el inicio de combate.
pub fn track_pirate_mission(
    task: &mut crate::scheduler::TaskRecord,
    targets: &[Option<u8>; 5],
    locations: &[SettlementLocation; 40],
    ships: &[Vec<u8>],
    head: u16,
) -> Result<Option<bool>, &'static str> {
    if task.kind != 0x32 || task.u8_at(0xc) != 3 || task.u8_at(0xf) == 0 || task.due >= task.u32_at(8) {
        return Ok(None);
    }
    let phase = task.u8_at(0x12);
    if phase > 1 {
        return Ok(None);
    }
    let band = usize::from(task.u8_at(0x10));
    let target = task.u8_at(0x11);
    if targets.get(band).copied().flatten() != Some(target) {
        return Ok(None);
    }
    let location = locations.get(usize::from(target)).ok_or("destino pirata ausente")?;
    let mut next = *task;
    let mut tracked = u16::from_le_bytes([next.u8_at(0x14), next.u8_at(0x15)]);
    let pos = |ship: &[u8]| -> Result<(i32, i32), &'static str> {
        if ship.len() < 0x136 {
            return Err("barco truncado");
        }
        Ok((
            i32::from(i16::from_le_bytes([ship[0x1e], ship[0x1f]])),
            i32::from(i16::from_le_bytes([ship[0x22], ship[0x23]])),
        ))
    };
    if phase == 1 {
        if let Some(ship) = ships.get(usize::from(tracked)) {
            if pos(ship)? != (location.x, location.y) {
                tracked = u16::MAX;
                next.set_u8_at(0x14, 0xff);
                next.set_u8_at(0x15, 0xff)
            }
        }
    }
    let mut delay = if phase == 0 { 0x40u32 } else { 0x1f };
    let mut seen = vec![false; ships.len()];
    let mut at = usize::from(head);
    while at < ships.len() {
        if seen[at] {
            return Err("barcos de comerciante cíclicos");
        }
        seen[at] = true;
        let ship = &ships[at];
        let (x, y) = pos(ship)?;
        let dx = x.wrapping_sub(location.x);
        let dy = y.wrapping_sub(location.y);
        let distance = dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy));
        let state = u16::from_le_bytes([ship[0x134], ship[0x135]]);
        if phase == 0 {
            if distance < 200 {
                return Ok(None);
            }
            if distance < 0x1000 {
                let n = if distance > 0x400 {
                    22
                } else if distance > 0x200 {
                    12
                } else if distance > 0x100 {
                    6
                } else {
                    2
                };
                delay = delay.min(n)
            }
        } else if at != usize::from(tracked) && state == 0xf {
            if distance == 0 {
                return Ok(None);
            }
            if distance < 0x40 {
                delay = 1
            } else if distance < 0xe1 {
                delay &= 3
            } else if distance < 0x640 {
                delay &= 7
            }
        }
        at = usize::from(u16::from_le_bytes([ship[4], ship[5]]));
    }
    next.due = next.due.wrapping_add(delay);
    *task = next;
    Ok(Some(true))
}

/// Vencimiento humano de los cinco tipos de misión.
pub fn fail_alderman_mission_human(
    task: &crate::scheduler::TaskRecord,
    merchants: &[crate::ai::reputation::Merchant],
    merchant_count: u16,
    towns: &[crate::town::Town],
    mayor: u8,
    mayor_town: u8,
    player: u32,
    tick: u32,
) -> Result<Option<AldermanHumanFailure>, &'static str> {
    if task.kind != 0x32 || task.u8_at(0xf) == 0 || task.due < task.u32_at(8) {
        return Ok(None);
    }
    let owner = usize::from(task.u8_at(0xd));
    if owner >= usize::from(merchant_count) {
        return Ok(None);
    }
    let mut merchant = merchants.get(owner).ok_or("comerciante ausente")?.clone();
    if merchant.bytes.len() < 0x650 || towns.len() > 40 {
        return Err("reputación incompleta");
    }
    if merchant.u16(8) != 0 {
        return Ok(None);
    }
    let kind = task.u8_at(0xc);
    if !matches!(kind, 0 | 1 | 2 | 3 | 4) {
        return Ok(None);
    }
    let home = usize::from(merchant.bytes[0x19]);
    if home >= 40 {
        return Err("ciudad natal inválida");
    }
    let names = if u16::from(mayor) < merchant_count {
        let m = merchants.get(usize::from(mayor)).ok_or("alcalde ausente")?;
        if m.bytes.len() < 0x1e {
            return Err("alcalde truncado");
        }
        [m.bytes[0x1c], m.bytes[0x1d]]
    } else {
        let town = towns.get(usize::from(mayor_town)).ok_or("ciudad del alcalde ausente")?;
        let i = usize::from((!mayor) & 3);
        [town.u8(0x6d4 + i), town.u8(0x6d8 + i)]
    };
    let env = crate::ai::reputation::RepEnv {
        town_count: towns.len() as u32,
        towns,
        renown: [0; 3],
        byte_52c: 0,
    };
    merchant.subtract_buildings_reputation_flagged_towns(&env);
    let mut letter = super::misc::blank_record();
    letter[4] = 0x70;
    letter[5] = mayor_town;
    letter[8] = kind;
    letter[9] = merchant
        .get_merchant_epithet_index()
        .wrapping_add(merchant.bytes[0x39c + home].wrapping_shl(5));
    letter[10] = merchant.bytes[0x1c];
    letter[11] = merchant.bytes[0x1d];
    letter[12] = names[0];
    letter[13] = names[1];
    match kind {
        0 => letter[14] = task.u8_at(0x16),
        1 => {
            letter[14] = task.u8_at(0x10);
            letter[15] = task.u8_at(0x11)
        }
        2 => {
            letter[14] = task.u8_at(0x12);
            letter[15] = task.u8_at(0x14)
        }
        3 => {
            letter[14] = task.u8_at(0x11);
            merchant.bytes[0xe1] = 0xff
        }
        4 => letter[14] = task.u8_at(0x12),
        _ => {}
    }
    let followup = if kind == 1 {
        let mut r = crate::scheduler::TaskRecord {
            due: task.due.wrapping_add(1),
            kind: task.kind,
            ..Default::default()
        };
        r.set_u32_at(8, task.due.wrapping_add(0xba00));
        r.set_u8_at(0xc, 1);
        r.set_u8_at(0xd, task.u8_at(0x10));
        for offset in 0xe..=0x12 {
            r.set_u8_at(offset, task.u8_at(offset))
        }
        r.set_u8_at(0x13, 0xff);
        r.set_u32_at(0x14, u32::MAX);
        Some(r)
    } else {
        None
    };
    let mut next = *task;
    next.set_u8_at(0xc, 5);
    next.due = tick.wrapping_add(0x1c00);
    next.set_u32_at(8, next.due.wrapping_add(1));
    Ok(Some(AldermanHumanFailure {
        owner,
        recipient: owner,
        player,
        merchant,
        task: next,
        followup,
        letter,
        notification: [owner as u32, 0, 0, u32::from(mayor_town), 0x1e, owner as u32],
        dialog_requested: player == owner as u32,
    }))
}

/// Inserta la tarea adicional sin perder los enlaces modificados por el planificador.
pub fn apply_human_failure_task(
    s: &mut crate::scheduler::Scheduler,
    effect: &mut AldermanHumanFailure,
) -> Result<(), &'static str> {
    if s.records.len() < usize::from(s.capacity) || s.head >= s.capacity {
        return Err("planificador truncado");
    }
    let original = s.head;
    if let Some(followup) = effect.followup {
        let mut seen = vec![false; usize::from(s.capacity)];
        let mut at = s.head;
        while at < s.capacity {
            let i = usize::from(at);
            if seen[i] {
                return Err("tareas cíclicas");
            }
            seen[i] = true;
            at = s.records[i].next
        }
        if s.free_head < s.capacity && seen[usize::from(s.free_head)] {
            return Err("ranura libre activa");
        }
        if s.free_head > s.capacity {
            return Err("cabeza libre fuera de capacidad");
        }
        if s.free_head == s.capacity && s.capacity > u16::MAX - crate::scheduler::GROWTH {
            return Err("planificador sin capacidad");
        }
        let index = s.add_task(followup.due);
        let source = s.records[usize::from(s.head)];
        let r = &mut s.records[usize::from(index)];
        r.kind = source.kind;
        r.set_u32_at(8, source.due.wrapping_add(0xba00));
        r.set_u8_at(0xc, 1);
        r.set_u8_at(0xd, source.u8_at(0x10));
        for offset in 0xe..=0x12 {
            r.set_u8_at(offset, source.u8_at(offset))
        }
        r.set_u8_at(0x13, 0xff);
        r.set_u32_at(0x14, u32::MAX);
        // El original vuelve a leer la cabeza: si due+1 se desborda, modifica la
        // tarea recién insertada y deja intacta la que estaba vencida.
        let current = &mut s.records[usize::from(s.head)];
        effect.recipient = usize::from(current.u8_at(0xd));
        effect.notification[0] = effect.recipient as u32;
        effect.notification[5] = effect.recipient as u32;
        effect.dialog_requested = effect.player == effect.recipient as u32;
        current.set_u8_at(0xc, 5);
        current.due = effect.task.due;
        current.set_u32_at(8, effect.task.u32_at(8));
    } else {
        let next = s.records[usize::from(original)].next;
        s.records[usize::from(original)] = effect.task;
        s.records[usize::from(original)].next = next;
    }
    Ok(())
}

/// Vencimiento de una misión asignada a IA: aplica la pérdida de reputación
/// y programa su fase de cierre; la rama humana se trata por separado.
pub fn fail_alderman_mission_ai(
    task: &mut crate::scheduler::TaskRecord,
    merchant: &mut crate::ai::reputation::Merchant,
    towns: &[crate::town::Town],
    tick: u32,
) -> Result<bool, &'static str> {
    if task.kind != 0x32 || task.u8_at(0xf) == 0 || task.due < task.u32_at(8) {
        return Ok(false);
    }
    if merchant.bytes.len() < 0x650 || towns.len() > 40 {
        return Err("datos de reputación incompletos");
    }
    if merchant.u16(8) == 0 {
        return Ok(false);
    }
    let env = crate::ai::reputation::RepEnv {
        town_count: towns.len() as u32,
        towns,
        renown: [0; 3],
        byte_52c: 0,
    };
    merchant.subtract_buildings_reputation_flagged_towns(&env);
    task.set_u8_at(0xc, 5);
    task.due = tick.wrapping_add(0x1c00);
    task.set_u32_at(8, task.due.wrapping_add(1));
    Ok(true)
}

/// `ticks_to_date` 0x005311D0 usa desplazamiento y división con signo;
/// en ticks altos puede producir mes cero y leer el byte previo a la tabla.
fn mission_ticks_to_date(tick: u32) -> (u8, u8, u16) {
    let days = (tick as i32) >> 8;
    let year = (days / 365) as u16;
    let day_of_year = days.wrapping_sub(i32::from(year) * 365);
    if day_of_year < 0 {
        return ((day_of_year as u8).wrapping_sub(24).wrapping_add(1), 0, year);
    }
    let mut month = 0usize;
    while month < 12 && i32::from(crate::calendar::MONTH_START[month]) <= day_of_year {
        month += 1
    }
    let start = if month == 0 {
        24
    } else {
        crate::calendar::MONTH_START[month - 1] as u8
    };
    (
        (day_of_year as u8).wrapping_sub(start).wrapping_add(1),
        month as u8,
        year,
    )
}

/// Inicio de misión terrestre (tipo 1): modifica dos banderas urbanas y
/// produce la carta 0x80. Los efectos se calculan antes de publicar cambios.
pub fn start_alderman_road_mission(
    task: &mut crate::scheduler::TaskRecord,
    towns: &mut [crate::town::Town],
    town_ids: &[u8],
    merchants: &[crate::ai::reputation::Merchant],
    merchant_count: u16,
    mayor: u8,
    mayor_town: u8,
    tick: u32,
) -> Result<(usize, [u8; 16]), &'static str> {
    const ROUTES: [[u8; 3]; 7] = [
        [3, 4, 80],
        [9, 10, 130],
        [11, 13, 30],
        [12, 14, 40],
        [28, 31, 120],
        [36, 37, 60],
        [38, 39, 90],
    ];
    if task.kind != 0x32 || task.u8_at(0xc) != 1 || task.u8_at(0xf) != 0 {
        return Err("fase de misión incompatible");
    }
    let a = usize::from(task.u8_at(0x10));
    let b = usize::from(task.u8_at(0x11));
    if a >= towns.len() || b >= towns.len() || a >= town_ids.len() || b >= town_ids.len() {
        return Err("ciudad de ruta ausente");
    }
    let names = if u16::from(mayor) < merchant_count {
        let m = merchants.get(usize::from(mayor)).ok_or("alcalde ausente")?;
        if m.bytes.len() < 0x1e {
            return Err("alcalde truncado");
        }
        [m.bytes[0x1c], m.bytes[0x1d]]
    } else {
        let town = towns.get(usize::from(mayor_town)).ok_or("ciudad del alcalde ausente")?;
        let i = if u16::from(town.u8(0x6f1)) < merchant_count {
            0
        } else {
            usize::from((!mayor) & 3)
        };
        [town.u8(0x6d4 + i), town.u8(0x6d8 + i)]
    };
    let owner = usize::from(task.u8_at(0xd));
    if owner >= usize::from(merchant_count) || owner >= merchants.len() {
        return Err("propietario de misión ausente");
    }
    let delay = ROUTES
        .iter()
        .find(|r| (town_ids[a] == r[0] && town_ids[b] == r[1]) || (town_ids[a] == r[1] && town_ids[b] == r[0]))
        .map(|r| u32::from(r[2]));
    let duration = delay.map_or(0x26c00, |d| (d + (d >> 1) + 20) << 8);
    let deadline = tick.wrapping_add(duration);
    let (day, month, year) = mission_ticks_to_date(deadline);
    let mut letter = super::misc::blank_record();
    letter[4] = 0x80;
    letter[5] = mayor_town;
    letter[8] = names[0];
    letter[9] = names[1];
    letter[10] = a as u8;
    letter[11] = b as u8;
    letter[12] = owner as u8;
    letter[13] = day;
    let ym = year.wrapping_mul(12).wrapping_add(u16::from(month)).wrapping_sub(1);
    letter[14..16].copy_from_slice(&ym.to_le_bytes());
    towns[a].set_u32(0x76c, towns[a].u32(0x76c) | 0x8000);
    towns[b].set_u32(0x76c, towns[b].u32(0x76c) | 0x8000);
    task.set_u8_at(0xf, 1);
    task.set_u32_at(8, deadline);
    task.due = task.due.wrapping_add(0x100);
    Ok((owner, letter))
}

/// Inicio de misión de banda (tipo 3): fija el plazo y produce las cartas
/// 0x6E y 0x84 con la posición y los datos del comerciante asignado.
pub fn start_alderman_pirate_mission(
    task: &mut crate::scheduler::TaskRecord,
    merchants: &[crate::ai::reputation::Merchant],
    merchant_count: u16,
    mayor_town: u8,
    tick: u32,
    locations: &[SettlementLocation; 40],
) -> Result<(usize, [[u8; 16]; 2]), &'static str> {
    if task.kind != 0x32 || task.u8_at(0xc) != 3 || task.u8_at(0xf) != 0 {
        return Err("fase de misión incompatible");
    }
    let owner = usize::from(task.u8_at(0xd));
    if owner >= usize::from(merchant_count) {
        return Err("propietario de misión ausente");
    }
    let merchant = merchants.get(owner).ok_or("comerciante ausente")?;
    if merchant.bytes.len() < 0x650 {
        return Err("comerciante truncado");
    }
    let home = usize::from(merchant.bytes[0x19]);
    if home >= 40 {
        return Err("ciudad natal inválida");
    }
    let loc = locations
        .get(usize::from(task.u8_at(0x11)))
        .ok_or("destino pirata ausente")?;
    let deadline = tick.wrapping_add(0x1e00) | 0xff;
    let (day, month, year) = mission_ticks_to_date(deadline);
    let mut first = super::misc::blank_record();
    first[4] = 0x6e;
    first[5] = mayor_town;
    first[10..12].copy_from_slice(&u16::MAX.to_le_bytes());
    first[12..14].copy_from_slice(&(loc.x as u16).to_le_bytes());
    first[14..16].copy_from_slice(&(loc.y as u16).to_le_bytes());
    let mut second = first;
    second[4] = 0x84;
    second[8] = merchant.get_merchant_epithet_index();
    second[9] = merchant.bytes[0x39c + home];
    second[10] = merchant.bytes[0x1d];
    second[11] = merchant.bytes[0x1c];
    second[12] = day;
    second[13] = month.wrapping_sub(1);
    second[14..16].copy_from_slice(&year.to_le_bytes());
    task.set_u8_at(0xf, 1);
    task.set_u32_at(8, deadline);
    task.set_u8_at(0x14, 0xff);
    task.set_u8_at(0x15, 0xff);
    task.due = task.due.wrapping_add(0x100);
    Ok((owner, [first, second]))
}

/// Calcula el ascenso y la carta global 0x6F; el anfitrión publica la carta,
/// el aviso directo y la posible ventana junto con el comerciante modificado.
pub fn complete_alderman_mission(
    task: &crate::scheduler::TaskRecord,
    merchants: &[crate::ai::reputation::Merchant],
    merchant_count: u16,
    towns: &[crate::town::Town],
    mayor: u8,
    mayor_town: u8,
    player: u32,
) -> Result<AldermanCompletion, &'static str> {
    let owner = usize::from(task.u8_at(0xd));
    let mut merchant = merchants.get(owner).ok_or("comerciante ausente")?.clone();
    if merchant.bytes.len() < 0x650 {
        return Err("comerciante truncado");
    }
    let home = usize::from(merchant.bytes[0x19]);
    if home >= 40 {
        return Err("ciudad natal inválida");
    }
    let mayor_names = if u16::from(mayor) < merchant_count {
        let m = merchants.get(usize::from(mayor)).ok_or("alcalde ausente")?;
        if m.bytes.len() < 0x1e {
            return Err("alcalde truncado");
        }
        [m.bytes[0x1c], m.bytes[0x1d]]
    } else {
        let town = towns.get(usize::from(mayor_town)).ok_or("ciudad del alcalde ausente")?;
        let i = usize::from((!mayor) & 3);
        [town.u8(0x6d4 + i), town.u8(0x6d8 + i)]
    };
    let kind = task.u8_at(0xc);
    let mut record = super::misc::blank_record();
    record[4] = 0x6f;
    record[5] = mayor_town;
    record[8] = kind;
    merchant.bytes[0x37] = merchant.bytes[0x37].wrapping_add(1);
    if merchant.bytes[0x37] == 4 {
        record[8] |= 0x80
    }
    record[9] = merchant
        .get_merchant_epithet_index()
        .wrapping_add(merchant.bytes[0x39c + home].wrapping_shl(5));
    record[10] = merchant.bytes[0x1c];
    record[11] = merchant.bytes[0x1d];
    record[12] = mayor_names[0];
    record[13] = mayor_names[1];
    match kind {
        0 => {
            record[14] = task.u8_at(0x16);
            merchant.bytes[0x37] = merchant.bytes[0x37].wrapping_add(1);
            if merchant.bytes[0x37] == 4 {
                record[8] |= 0x80
            }
        }
        1 => {
            record[14] = task.u8_at(0x10);
            record[15] = task.u8_at(0x11)
        }
        2 => {
            record[14] = task.u8_at(0x12);
            record[15] = task.u8_at(0x14)
        }
        3 => record[14] = task.u8_at(0x11),
        4 => record[14] = task.u8_at(0x12),
        _ => {}
    }
    let direct = merchant.u16(8) == 0;
    let notification = direct.then_some([owner as u32, 0, 0, u32::from(mayor_town), 0x1f, owner as u32]);
    Ok(AldermanCompletion {
        owner,
        merchant,
        letter: record,
        direct,
        notification,
        dialog_requested: direct && player == owner as u32,
    })
}

/// Última entrega terrestre y sus dos obras (0x004EB475). El llamador aporta
/// los bytes iniciales de las coordenadas: el original lee locales de pila sin inicializar.
pub fn finish_road_materials(
    task: &mut crate::scheduler::TaskRecord,
    towns: &mut [crate::town::Town],
    sites: &mut [crate::buildings::Site],
    mut row: u8,
    mut col: u8,
    merchant_count: u16,
) -> Result<(), &'static str> {
    if merchant_count > 255 {
        return Err("0xff identifica a un comerciante");
    }
    if task.u8_at(0x12) != 1 {
        return Err("etapa terrestre distinta");
    }
    let amounts = std::array::from_fn::<u32, 4, _>(|i| {
        u32::from(task.u8_at(0x14 + i)) + (u32::from((task.u8_at(0x13) >> (2 * i)) & 3) << 8)
    });
    let costs = [3u32, 4, 1, 2];
    if !(0..4).all(|i| amounts[i] >= costs[i]) {
        return Err("materiales insuficientes");
    }
    let cities = [usize::from(task.u8_at(0x10)), usize::from(task.u8_at(0x11))];
    for &city in &cities {
        let t = towns.get(city).ok_or("ciudad de ruta ausente")?;
        let s = sites.get(city).ok_or("plano de ruta ausente")?;
        let rows = t.i32(0x7a8);
        let cols = t.i32(0x7ac);
        if rows <= 0
            || cols <= 0
            || rows > 256
            || cols > 256
            || s.cells.len() < (rows * cols) as usize
            || s.records.len() < usize::from(t.u16(0x776)) * 8
            || t.u16(0x760) > t.u16(0x776)
            || t.u16(0x776) > u16::MAX - 128
        {
            return Err("plano de ruta inválido");
        }
    }
    let mut high = task.u8_at(0x13);
    for i in 0..4 {
        let remaining = amounts[i] - costs[i];
        task.set_u8_at(0x14 + i, remaining as u8);
        high = (high & !(3 << (2 * i))) | (((remaining >> 8) as u8 & 3) << (2 * i))
    }
    task.set_u8_at(0x13, high);
    task.set_u8_at(0x12, 0);
    task.due = task.due.wrapping_add(0x180);
    struct TownBuild;
    impl crate::buildings::construction::BuildHost for TownBuild {
        fn call(&mut self, _: crate::buildings::construction::Hook, _: &[u32]) -> u32 {
            unreachable!("la obra municipal 0x27 no requiere servicios externos")
        }
        fn office_building_head(&mut self, _: u32) -> u16 {
            unreachable!()
        }
    }
    let mut host = TownBuild;
    let (mut gold, mut income) = (0, 0);
    let classes = [0u8; 256];
    let mut ctx = crate::buildings::construction::BuildCtx {
        merchant_count,
        visual_owner: None,
        special: 0,
        owner_class: &classes,
        merchant_gold: &mut gold,
        merchant_income: &mut income,
        host: &mut host,
    };
    for city in cities {
        let t = &mut towns[city];
        let s = &mut sites[city];
        s.find_nearest_cell_for_type(t, &mut row, &mut col, 0x27, 0, true, 0);
        if i32::from(row) >= t.i32(0x7a8) || i32::from(col) >= t.i32(0x7ac) {
            return Err("coordenada de ruta fuera del plano");
        }
        s.start_building_construction(t, &mut ctx, 0xff, row, col, 0x27, 0);
    }
    if task.due >= task.u32_at(8) {
        task.set_u32_at(8, task.due.wrapping_add(0x200))
    }
    Ok(())
}

/// Solo permite escoger coordenadas nativas cuando todas las entradas posibles
/// del local no inicializado llevan a la misma casilla en ambas ciudades.
pub fn finish_road_materials_stable(
    task: &mut crate::scheduler::TaskRecord,
    towns: &mut [crate::town::Town],
    sites: &mut [crate::buildings::Site],
    merchant_count: u16,
) -> Result<(), &'static str> {
    if task.u8_at(0x10) == task.u8_at(0x11) {
        return Err("la ruta repite ciudad");
    }
    for offset in [0x10, 0x11] {
        let city = usize::from(task.u8_at(offset));
        let t = towns.get(city).ok_or("ciudad de ruta ausente")?;
        let s = sites.get(city).ok_or("plano de ruta ausente")?;
        let rows = t.i32(0x7a8);
        let cols = t.i32(0x7ac);
        if rows <= 0
            || cols <= 0
            || rows > 256
            || cols > 256
            || s.cells.len() < (rows * cols) as usize
            || i32::from(t.u8(0x800)) >= rows
            || i32::from(t.u8(0x801)) >= cols
        {
            return Err("plano de ruta inválido");
        }
        let mut target = None;
        for y in 0..rows {
            for x in 0..cols {
                let (mut row, mut col) = (y as u8, x as u8);
                if !s.find_nearest_cell_for_type(t, &mut row, &mut col, 0x27, 0, true, 0) {
                    return Err("casilla de ruta depende de la pila original");
                }
                match target {
                    None => target = Some((row, col)),
                    Some(p) if p == (row, col) => {}
                    _ => return Err("casilla de ruta depende de la pila original"),
                }
            }
        }
    }
    finish_road_materials(task, towns, sites, 0, 0, merchant_count)
}

/// Datos que lee 0x0053A3E0 de la cadena de barcos del comerciante.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RoadDeliveryShip {
    pub next: u16,
    pub town: u8,
    pub flags: u8,
    pub stock: [i32; 24],
}

/// Validación y asignación de la operación 0x4B (0x0053B440). Si acepta,
/// devuelve el índice para que el anfitrión ejecute 0x004EBF54 en la misma transacción.
pub fn accept_alderman_mission(
    s: &mut crate::scheduler::Scheduler,
    op: &crate::pump::Operation,
    merchants: &[crate::ai::reputation::Merchant],
    merchant_count: u16,
) -> Result<Option<u16>, &'static str> {
    let owner = op.0[1];
    let index = op.0[2];
    if owner >= u32::from(merchant_count) || index >= u32::from(s.capacity) {
        return Ok(None);
    }
    if s.records.len() < usize::from(s.capacity) {
        return Err("planificador truncado");
    }
    let task = s.records[index as usize];
    if task.kind != 0x32 || u16::from(task.u8_at(0xd)) < merchant_count || task.u8_at(0xc) >= 5 {
        return Ok(None);
    }
    let merchant = merchants.get(owner as usize).ok_or("comerciante ausente")?;
    if merchant.bytes.len() <= 0x37 {
        return Err("comerciante truncado");
    }
    if merchant.bytes[0x37] == 0 {
        return Ok(None);
    }
    let mut seen = vec![false; usize::from(s.capacity)];
    let mut at = s.head;
    while at < s.capacity {
        let i = usize::from(at);
        if seen[i] {
            return Err("tareas cíclicas");
        }
        seen[i] = true;
        let t = &s.records[i];
        if t.kind == 0x32 && u32::from(t.u8_at(0xd)) == owner {
            return Ok(None);
        }
        at = t.next
    }
    s.records[index as usize].set_u8_at(0xd, owner as u8);
    Ok(Some(index as u16))
}

/// Operación 0x3B: entrega materiales desde la oficina y luego barcos locales.
/// Devuelve los índices cuyo cálculo de carga debe ejecutarse.
pub fn deliver_road_materials(
    task: &mut crate::scheduler::TaskRecord,
    op: &crate::pump::Operation,
    town_count: usize,
    merchant_count: u16,
    office: Option<&mut [i32; 24]>,
    ships: &mut [RoadDeliveryShip],
    head: u16,
) -> Result<Vec<usize>, &'static str> {
    let mut bytes = [0u8; 20];
    for (i, v) in op.0.iter().enumerate() {
        bytes[i * 4..i * 4 + 4].copy_from_slice(&v.to_le_bytes())
    }
    let city = bytes[8];
    let owner = bytes[9];
    if usize::from(city) >= town_count
        || task.kind != 0x32
        || task.u8_at(0xc) != 1
        || task.u8_at(0xd) != owner
        || (task.u8_at(0x10) != city && task.u8_at(0x11) != city)
        || task.u8_at(0x12) == 0
    {
        return Ok(Vec::new());
    }
    let code = bytes[10];
    let index = if code < 4 {
        usize::from(code)
    } else {
        match code {
            11 => 0,
            19 => 1,
            12 => 2,
            _ => 3,
        }
    };
    let ware = [11usize, 19, 12, 17][index];
    let lot = if [false, false, true, false][index] { 200 } else { 2000 };
    let requested = i32::from(u16::from_le_bytes([bytes[6], bytes[7]]));
    let mut remaining = requested;
    let mut dirty = Vec::new();
    if u16::from(owner) < merchant_count {
        if let Some(stock) = office {
            let available = stock[ware] / lot;
            let taken = if available > remaining {
                let t = remaining;
                remaining = 0;
                t
            } else {
                remaining = remaining.wrapping_sub(available);
                available
            };
            stock[ware] = stock[ware].wrapping_sub(taken.wrapping_mul(lot));
        }
        let mut at = usize::from(head);
        let mut seen = vec![false; ships.len()];
        while at < ships.len() && remaining > 0 {
            if seen[at] {
                return Err("barcos de comerciante cíclicos");
            }
            seen[at] = true;
            let ship = &mut ships[at];
            if ship.town == city && ship.flags & 1 == 0 && ship.stock[ware] > 0 {
                let available = ship.stock[ware] / lot;
                let taken = if available >= remaining {
                    let t = remaining;
                    remaining = 0;
                    t
                } else {
                    remaining = remaining.wrapping_sub(available);
                    available
                };
                ship.stock[ware] = ship.stock[ware].wrapping_sub(taken.wrapping_mul(lot));
                dirty.push(at)
            }
            at = usize::from(ship.next);
        }
    }
    let shift = 2 * index;
    let old = u32::from(task.u8_at(0x14 + index)) + (u32::from((task.u8_at(0x13) >> shift) & 3) << 8);
    let value = old.wrapping_add(requested.wrapping_sub(remaining) as u32);
    task.set_u8_at(0x14 + index, value as u8);
    task.set_u8_at(
        0x13,
        (task.u8_at(0x13) & !(3 << shift)) | (((value >> 8) as u8 & 3) << shift),
    );
    Ok(dirty)
}

/// Espera las instalaciones públicas de los extremos de una ruta (0x004EB62D).
/// Devuelve si las dos ya existen; actualiza el vencimiento incluso al terminar.
pub fn wait_road_installations(
    task: &mut crate::scheduler::TaskRecord,
    towns: &[crate::town::Town],
    sites: &[crate::buildings::Site],
) -> Result<bool, &'static str> {
    let mut found = [false; 2];
    let due = task.due.wrapping_add(0x200);
    task.due = due;
    if due >= task.u32_at(8) {
        task.set_u32_at(8, due.wrapping_add(1))
    }
    for (i, offset) in [0x10, 0x11].into_iter().enumerate() {
        let city = usize::from(task.u8_at(offset));
        let town = towns.get(city).ok_or("ciudad de ruta ausente")?;
        let site = sites.get(city).ok_or("plano de ruta ausente")?;
        let count = usize::from(town.u16(0x776));
        if site.records.len() < count * 8 {
            return Err("edificios de ruta truncados");
        }
        let mut seen = vec![false; count];
        let mut at = usize::from(town.u16(0x764));
        while at < count {
            if seen[at] {
                return Err("edificios de ruta cíclicos");
            }
            seen[at] = true;
            if site.rec(at, 3) == 0x27 {
                found[i] = true;
                break;
            }
            at = usize::from(site.next(at));
        }
        if !found[i] {
            break;
        }
    }
    Ok(found[0] && found[1])
}

/// `is_alderman_mission_still_valid` (0x00532C50).
pub fn alderman_mission_still_valid(
    scheduler: &crate::scheduler::Scheduler,
    index: u16,
    w: &AldermanMissionWorld<'_>,
) -> Result<bool, &'static str> {
    if index >= scheduler.capacity {
        return Ok(true);
    }
    let t = scheduler
        .records
        .get(usize::from(index))
        .ok_or("planificador truncado")?;
    if t.kind != 0x32 || u16::from(t.u8_at(0xd)) <= w.merchant_count || t.u8_at(0xc) >= 5 {
        return Ok(true);
    }
    let v = match t.u8_at(0xc) {
        0 => {
            let id = t.u8_at(0x14);
            !w.town_ids.contains(&id) && !w.pirate_targets.iter().flatten().any(|&x| x == id)
        }
        1 => {
            let a = usize::from(t.u8_at(0x10));
            if a >= w.towns.len() {
                false
            } else if w.towns[a].u32(0x76c) & 0x8000 != 0 {
                false
            } else {
                let b = usize::from(t.u8_at(0x11));
                w.towns.get(b).ok_or("segunda ciudad ausente")?.u32(0x76c) & 0x8000 == 0
            }
        }
        2 => {
            let ship = usize::from(u16::from_le_bytes([t.u8_at(0x10), t.u8_at(0x11)]));
            if let Some(s) = w.ships.get(ship) {
                if s.len() < 0x136 {
                    return Err("barco truncado");
                }
                let state = u16::from_le_bytes([s[0x134], s[0x135]]);
                let captain = usize::from(u16::from_le_bytes([s[0x42], s[0x43]]));
                if state != 0x12 {
                    false
                } else if let Some(c) = w.captains.get(captain) {
                    u16::from(c[2]) == u16::from_le_bytes([t.u8_at(0x12), t.u8_at(0x13)])
                        && u16::from(c[3]) == u16::from_le_bytes([t.u8_at(0x14), t.u8_at(0x15)])
                } else {
                    false
                }
            } else {
                false
            }
        }
        3 => {
            let i = usize::from(t.u8_at(0x10));
            w.pirate_targets.get(i).and_then(|x| *x) == Some(t.u8_at(0x11))
        }
        4 => {
            let i = usize::from(t.u8_at(0x12));
            w.towns.get(i).is_some_and(|town| {
                town.i32(0x2d4) < i32::from(u16::from_le_bytes([t.u8_at(0x10), t.u8_at(0x11)]) >> 1)
            })
        }
        _ => true,
    };
    Ok(v)
}

/// Operación 0x86: invalida las tareas 0x32 que ya no son aplicables.
pub fn invalidate_alderman_missions(
    scheduler: &mut crate::scheduler::Scheduler,
    w: &AldermanMissionWorld<'_>,
) -> Result<usize, &'static str> {
    let mut staged = scheduler.clone();
    let mut seen = vec![false; usize::from(staged.capacity)];
    let mut at = staged.head;
    let mut changed = 0;
    while at < staged.capacity {
        let i = usize::from(at);
        if seen[i] {
            return Err("tareas cíclicas");
        }
        seen[i] = true;
        let t = staged.records.get(i).ok_or("planificador truncado")?;
        let next = t.next;
        if t.kind == 0x32
            && u16::from(t.u8_at(0xd)) >= w.merchant_count
            && t.u8_at(0xc) < 5
            && !alderman_mission_still_valid(&staged, at, w)?
        {
            staged.records[i].set_u8_at(0xc, 5);
            changed += 1
        }
        at = next;
    }
    *scheduler = staged;
    Ok(changed)
}

/// Fases acreditadas de `st_alderman_mission` (0x004EAFE4): espera,
/// selección y los inicios 2 y 4. `None` indica una fase aún no portada.
pub fn tick_alderman_mission_partial(
    scheduler: &mut crate::scheduler::Scheduler,
    w: &AldermanMissionWorld<'_>,
) -> Result<Option<bool>, &'static str> {
    let index = scheduler.head;
    if index >= scheduler.capacity {
        return Err("tarea ausente");
    }
    let t = scheduler
        .records
        .get(usize::from(index))
        .ok_or("planificador truncado")?;
    if t.kind != 0x32 {
        return Err("tipo de tarea incorrecto");
    }
    if u16::from(t.u8_at(0xd)) < w.merchant_count {
        if t.u8_at(0xf) != 0 {
            let deadline = t.u32_at(8);
            if t.due >= deadline {
                return Ok(None);
            }
            if t.u8_at(0xc) == 5 {
                return if w.notification_window_active {
                    Ok(None)
                } else {
                    Ok(Some(false))
                };
            }
            let due = match t.u8_at(0xc) {
                0 => {
                    let city = w
                        .towns
                        .get(usize::from(t.u8_at(0x16)))
                        .ok_or("ciudad de misión ausente")?;
                    if city.u32(0x76c) & 0x100 != 0 {
                        return Ok(None);
                    }
                    t.due.wrapping_add(0x200)
                }
                1 => {
                    let a = usize::from(t.u8_at(0x10));
                    let b = usize::from(t.u8_at(0x11));
                    if a >= w.towns.len() || b >= w.towns.len() {
                        return Ok(Some(false));
                    }
                    let count = t.u8_at(0x12);
                    if count == 0 {
                        return Ok(None);
                    }
                    let cost = [3u32, 4, 1, 2];
                    let packed = t.u8_at(0x13);
                    let quantities = std::array::from_fn::<u32, 4, _>(|i| {
                        u32::from(t.u8_at(0x14 + i)) + (u32::from((packed >> (2 * i)) & 3) << 8)
                    });
                    let enough = (0..4).all(|i| quantities[i] >= cost[i]);
                    if enough && count == 1 {
                        return Ok(None);
                    }
                    let r = &mut scheduler.records[usize::from(index)];
                    r.due = r.due.wrapping_add(0x180);
                    if enough {
                        let mut high = packed;
                        for i in 0..4 {
                            let remaining = quantities[i] - cost[i];
                            r.set_u8_at(0x14 + i, remaining as u8);
                            high = (high & !(3 << (2 * i))) | (((remaining >> 8) as u8 & 3) << (2 * i));
                        }
                        r.set_u8_at(0x13, high);
                        r.set_u8_at(0x12, count - 1);
                        if r.due >= deadline
                            && (0..4).all(|i| quantities[i] - cost[i] >= cost[i] * u32::from(count - 1))
                        {
                            r.set_u32_at(8, r.due.wrapping_add(0x200));
                        }
                    }
                    return Ok(Some(true));
                }
                2 => {
                    if t.u8_at(0x16) != 0 {
                        return Ok(None);
                    }
                    let ship = usize::from(u16::from_le_bytes([t.u8_at(0x10), t.u8_at(0x11)]));
                    let valid = if let Some(s) = w.ships.get(ship) {
                        if s.len() < 0x136 {
                            return Err("barco truncado");
                        }
                        let captain = usize::from(u16::from_le_bytes([s[0x42], s[0x43]]));
                        u16::from_le_bytes([s[0x134], s[0x135]]) == 0x12
                            && w.captains.get(captain).is_some_and(|c| {
                                u16::from(c[2]) == u16::from_le_bytes([t.u8_at(0x12), t.u8_at(0x13)])
                                    && u16::from(c[3]) == u16::from_le_bytes([t.u8_at(0x14), t.u8_at(0x15)])
                            })
                    } else {
                        false
                    };
                    if !valid {
                        scheduler.records[usize::from(index)].set_u8_at(0x16, 2)
                    }
                    scheduler.records[usize::from(index)].due.wrapping_add(0x400)
                }
                4 => {
                    let city = w
                        .towns
                        .get(usize::from(t.u8_at(0x12)))
                        .ok_or("ciudad de misión ausente")?;
                    let target = u16::from_le_bytes([t.u8_at(0x10), t.u8_at(0x11)]);
                    if city.i32(0x2d4) >= i32::from(target) {
                        return Ok(None);
                    }
                    let later = t.due.wrapping_add(0x700);
                    if later <= deadline {
                        later
                    } else {
                        let next = t.due.wrapping_add(1);
                        if next >= deadline {
                            next
                        } else {
                            deadline.wrapping_sub(1)
                        }
                    }
                }
                _ => return Ok(None),
            };
            scheduler.records[usize::from(index)].due = due;
            return Ok(Some(true));
        }
        let kind = t.u8_at(0xc);
        let population = if kind == 4 {
            Some(
                w.towns
                    .get(usize::from(t.u8_at(0x12)))
                    .ok_or("ciudad de misión ausente")?
                    .u16(0x2d4),
            )
        } else {
            None
        };
        if kind != 2 && kind != 4 && kind < 5 {
            return Ok(None);
        }
        let t = &mut scheduler.records[usize::from(index)];
        t.set_u8_at(0xf, 1);
        if kind == 2 {
            t.set_u32_at(8, w.tick.wrapping_add(0x1e00) | 0xff)
        }
        if let Some(pop) = population {
            t.set_u32_at(8, w.tick.wrapping_add(0x16d00) | 0xff);
            let value = pop.wrapping_add(2000) / 100 * 100;
            t.set_u8_at(0x10, value as u8);
            t.set_u8_at(0x11, (value >> 8) as u8)
        }
        t.due = t.due.wrapping_add(0x100);
        return Ok(Some(true));
    }
    if t.u8_at(0xc) >= 5 || !alderman_mission_still_valid(scheduler, index, w)? {
        return Ok(Some(false));
    }
    if t.u8_at(0xe) >= 0x3f {
        if usize::from(w.merchant_count) > w.merchants.len() || w.merchant_count > 256 || w.towns.len() > 40 {
            return Err("comerciantes o ciudades incompletos");
        }
        if w.merchants
            .iter()
            .take(usize::from(w.merchant_count))
            .any(|m| m.bytes.len() < 0x650)
        {
            return Err("comerciante truncado");
        }
        let mut selected = t.u8_at(0xd);
        for i in (0..usize::from(w.merchant_count)).rev() {
            let m = &w.merchants[i];
            let rank = m.bytes[0x37];
            if m.u16(8) != 0
                && rank > 0
                && rank < 4
                && (m.calc_total_reputation_score(w.towns.len() as i32, w.towns) as i16) > 0
            {
                selected = i as u8
            }
        }
        let t = &mut scheduler.records[usize::from(index)];
        t.set_u8_at(0xd, selected);
        t.set_u8_at(0xf, 0);
        t.due = t.due.wrapping_add(u32::from(w.day_of_year)).wrapping_add(0x300);
        return Ok(Some(u16::from(selected) < w.merchant_count));
    }
    let t = &mut scheduler.records[usize::from(index)];
    t.set_u8_at(0xe, t.u8_at(0xe).wrapping_add(2));
    t.due = t.due.wrapping_add(0x200);
    Ok(Some(true))
}

/// Despacho de la operación 0x85 (0x0053FD10): crea una tarea 0x32 si no
/// existe otra con las claves equivalentes de ese tipo de misión.
pub fn schedule_unique_alderman_task(
    scheduler: &mut crate::scheduler::Scheduler,
    tick: u32,
    op: &crate::pump::Operation,
) -> Result<bool, &'static str> {
    let mut bytes = [0u8; 20];
    for (i, v) in op.0.iter().enumerate() {
        bytes[i * 4..i * 4 + 4].copy_from_slice(&v.to_le_bytes())
    }
    let kind = bytes[8];
    if kind >= 5 {
        return Ok(false);
    }
    if scheduler.records.len() < usize::from(scheduler.capacity) {
        return Err("planificador truncado");
    }
    let mut seen = vec![false; usize::from(scheduler.capacity)];
    let mut at = scheduler.head;
    while at < scheduler.capacity {
        let i = usize::from(at);
        if seen[i] {
            return Err("tareas cíclicas");
        }
        seen[i] = true;
        let t = &scheduler.records[i];
        if t.kind == 0x32 && t.u8_at(0xc) == kind {
            let same = match kind {
                0 => {
                    t.u8_at(0x14) == bytes[0x10]
                        || t.u32_at(0x10) == u32::from_le_bytes(bytes[0xc..0x10].try_into().unwrap())
                }
                1 | 3 => t.u8_at(0x10) == bytes[0xc] || t.u8_at(0x11) == bytes[0xd],
                2 => t.u8_at(0x10) == bytes[0xc] && t.u8_at(0x11) == bytes[0xd],
                4 => t.u8_at(0x12) == bytes[0xe],
                _ => false,
            };
            if same {
                return Ok(false);
            }
        }
        at = t.next;
    }
    if scheduler.free_head < scheduler.capacity && seen[usize::from(scheduler.free_head)] {
        return Err("ranura libre activa");
    }
    if scheduler.free_head >= scheduler.capacity && scheduler.capacity > u16::MAX - crate::scheduler::GROWTH {
        return Err("planificador sin capacidad");
    }
    let mut next = scheduler.clone();
    let index = next.add_task(tick.wrapping_add(0x100));
    let record = &mut next.records[usize::from(index)];
    record.kind = 0x32;
    record.payload.copy_from_slice(&bytes[4..20]);
    *scheduler = next;
    Ok(true)
}

/// Selección de 0x00532E30. Rechaza las listas inválidas o cíclicas y el mapa
/// completamente ocupado, para el que el bucle del original no termina.
pub fn determine_new_settlement(
    towns: &[crate::town::Town],
    town_ids: &[u8],
    offices: &[SettlementOffice],
    pirate_targets: &[Option<u8>; 5],
    locations: &[SettlementLocation; 40],
) -> Result<(u32, u8), &'static str> {
    const GOODS: [u8; 20] = [9, 10, 5, 6, 13, 8, 0, 16, 12, 4, 255, 11, 7, 10, 15, 19, 14, 20, 17, 18];
    const CATEGORIES: [u8; 20] = [2, 0, 1, 5, 2, 3, 5, 2, 5, 4, 1, 5, 5, 0, 0, 4, 1, 3, 2, 2];
    if towns.len() != town_ids.len()
        || town_ids.len() > 40
        || town_ids.iter().any(|&i| i >= 40)
        || offices.len() > u16::MAX as usize
    {
        return Err("datos de asentamiento inválidos");
    }
    let mut stock = [0i32; 20];
    let mut demand = [0i32; 20];
    for town in towns {
        for i in 0..20 {
            stock[i] = stock[i]
                .wrapping_add(town.i32(4 + 4 * i))
                .wrapping_add(town.i32(0x670 + 4 * i));
            demand[i] = demand[i]
                .wrapping_add(town.i32(0x64 + 4 * i))
                .wrapping_add(town.i32(0x310 + 4 * i));
        }
        let mut pos = usize::from(town.u16(0x784));
        let mut seen = vec![false; offices.len()];
        while pos < offices.len() {
            if seen[pos] {
                return Err("oficinas cíclicas");
            }
            seen[pos] = true;
            let o = &offices[pos];
            for i in 0..20 {
                stock[i] = stock[i].wrapping_add(o.stock[i]);
                demand[i] = demand[i].wrapping_add(o.demand[i]);
            }
            pos = usize::from(o.next);
        }
    }
    let mut scores = [i32::MAX; 4];
    let mut chosen = [11usize, 0, 1, 3];
    for i in 0..20 {
        if demand[i] >= 1024 {
            let score = stock[i] / (demand[i] >> 10);
            for j in (1..4).rev() {
                if scores[j - 1] <= score {
                    break;
                }
                scores[j] = scores[j - 1];
                scores[j - 1] = score;
                chosen[j] = chosen[j - 1];
                chosen[j - 1] = i;
            }
        }
    }
    let mut mask = 0u32;
    let mut count = 3;
    let mut i = 0;
    while i < count {
        let g = chosen[i];
        let bit = (1u32 << ((u32::from(GOODS[g]).wrapping_sub(3)) & 31)) & 0x00ff_ffff;
        if mask & bit == 0 {
            mask |= bit
        } else {
            count = 4
        }
        if g == 10 {
            mask |= 0x40020
        }
        i += 1;
    }
    let kind = CATEGORIES[chosen[0]];
    let mut best = 0i32;
    let mut position = 255u8;
    let mut matched = false;
    let first = find_next_unused_map_location(0, town_ids, pirate_targets)?;
    let occupied = |id: u8| town_ids.contains(&id) || pirate_targets.iter().flatten().any(|&v| v == id);
    if occupied(first) {
        return Err("mapa sin posiciones libres");
    }
    let mut candidate = first;
    loop {
        let loc = locations[usize::from(candidate)];
        let same = loc.kind == kind;
        if !matched || same {
            if same && !matched {
                matched = true;
                best = 0
            }
            let mut nearest = [0x7f7f7f7fi32; 5];
            for &id in town_ids {
                let other = locations[usize::from(id)];
                let dx = loc.x.wrapping_sub(other.x);
                let dy = loc.y.wrapping_sub(other.y);
                let d = dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy));
                for j in (0..5).rev() {
                    if nearest[j] <= d {
                        break;
                    }
                    if j < 4 {
                        nearest[j + 1] = nearest[j]
                    }
                    nearest[j] = d;
                }
            }
            let total = nearest.iter().fold(0i32, |a, &b| a.wrapping_add(b));
            if total > best {
                best = total;
                position = candidate
            }
        }
        let next = find_next_unused_map_location(u32::from(candidate) + 1, town_ids, pirate_targets)?;
        if next < candidate {
            break;
        }
        candidate = next;
    }
    Ok((mask, position))
}

/// Programación 0x005326E0 en la frontera acreditada sin bandas piratas.
/// Devuelve las operaciones 0x86 y 0x85 en el orden original; no las ejecuta.
/// Las ramas que recorren bandas se rechazan para evitar omitir misiones.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MissionPirateBand {
    pub first_ship: u16,
    pub target: u8,
    pub reserve_strength: u16,
    pub status: u8,
}

pub struct PreparedAldermanMissions {
    pub operations: Vec<crate::pump::Operation>,
    pub pirates: [Option<MissionPirateBand>; 5],
}

pub fn prepare_alderman_missions_without_pirates(
    year: u16,
    month: u8,
    towns: &[crate::town::Town],
    town_ids: &[u8],
    offices: &[SettlementOffice],
    pirate_targets: &[Option<u8>; 5],
    locations: &[SettlementLocation; 40],
) -> Result<Vec<crate::pump::Operation>, &'static str> {
    if pirate_targets.iter().any(Option::is_some) {
        return Err("bandas presentes");
    }
    Ok(prepare_alderman_missions(
        year,
        month,
        towns,
        town_ids,
        offices,
        &[None; 5],
        &[],
        &[],
        0,
        0,
        locations,
    )?
    .operations)
}

/// Prepara las cinco clases de misión, incluyendo efectos sobre bandas.
pub fn prepare_alderman_missions(
    year: u16,
    month: u8,
    towns: &[crate::town::Town],
    town_ids: &[u8],
    offices: &[SettlementOffice],
    pirates: &[Option<MissionPirateBand>; 5],
    ships: &[Vec<u8>],
    captains: &[[u8; 16]],
    merchant_count: u16,
    difficulty: u8,
    locations: &[SettlementLocation; 40],
) -> Result<PreparedAldermanMissions, &'static str> {
    let mut pirates = *pirates;
    let pirate_targets = std::array::from_fn(|i| pirates[i].map(|p| p.target));
    const ROUTES: [[u8; 4]; 7] = [
        [3, 4, 80, 32],
        [9, 10, 130, 52],
        [11, 13, 30, 12],
        [12, 14, 40, 16],
        [28, 31, 120, 48],
        [36, 37, 60, 24],
        [38, 39, 90, 36],
    ];
    const LOC_GOODS: [u8; 40] = [
        19, 27, 20, 25, 15, 14, 26, 24, 21, 16, 7, 22, 3, 27, 0, 6, 12, 23, 10, 13, 10, 12, 11, 11, 25, 8, 2, 19, 1,
        23, 5, 17, 26, 22, 13, 4, 19, 18, 24, 9,
    ];
    // La primera tabla contiene 13 meses; algunas rutas leen más allá de ella.
    // Los valores siguientes corresponden a la sección .rdata del EXE inglés.
    const MONTH_DATA: [u16; 132] = [
        0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334, 365, 0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31,
        65535, 0, 6689, 11816, 0, 19200, 13107, 16179, 39322, 16281, 52429, 15692, 0, 0, 4826, 20929, 21845, 16341, 51,
        2560, 384, 18, 256, 256, 25, 65024, 65, 65, 65, 11, 0, 19200, 28836, 16253, 0, 16256, 47186, 16158, 2621,
        16087, 34079, 15979, 0, 0, 26214, 16294, 0, 16256, 13107, 16179, 43691, 16298, 43691, 16426, 43691, 16554, 0,
        16512, 43691, 16426, 43691, 16426, 43691, 16298, 0, 16512, 0, 16512, 43691, 16298, 0, 16512, 0, 16512, 43691,
        16298, 43691, 16298, 43691, 16298, 0, 16384, 43691, 16298, 43691, 16170, 0, 16384, 43691, 16298, 43691, 16170,
        0, 16512, 43691, 16298, 0, 0, 0, 16512, 43691, 16426, 43691, 16298, 0, 16384,
    ];
    if month >= 12 || towns.len() != town_ids.len() || towns.len() > 40 {
        return Err("datos de mundo no admitidos");
    }
    let mk = |payload: [u8; 16]| {
        let mut words = [0u32; 5];
        words[0] = 0x85;
        for i in 0..4 {
            words[i + 1] = u32::from_le_bytes(payload[i * 4..i * 4 + 4].try_into().unwrap())
        }
        crate::pump::Operation(words)
    };
    let mut out = vec![crate::pump::Operation([0x86, 0, 0, 0, 0])];
    let mut payload = [0u8; 16];
    if towns.len() < 26 {
        let (goods, location) = determine_new_settlement(towns, town_ids, offices, &pirate_targets, locations)?;
        let location_goods = if location == 0xff {
            0x40
        } else {
            *LOC_GOODS.get(usize::from(location)).ok_or("posicion ausente")?
        };
        let mut m = u32::from(month) + 25;
        let mut y = u32::from(year);
        while m > 12 {
            m -= 12;
            y = y.wrapping_add(1)
        }
        let due = y
            .wrapping_mul(365)
            .wrapping_add(u32::from(MONTH_DATA[m as usize]))
            .wrapping_shl(8);
        payload[..4].copy_from_slice(&due.to_le_bytes());
        payload[4] = 0;
        payload[5] = 0xff;
        payload[6] = 0;
        payload[7] = 0;
        payload[8..12].copy_from_slice(&goods.to_le_bytes());
        payload[12] = location;
        payload[13] = location_goods;
        payload[14] = 0xff;
        out.push(mk(payload));
    }
    let mut best = 0u32;
    let mut route = None;
    for r in ROUTES {
        let mut a = None;
        let mut b = None;
        for (i, t) in towns.iter().enumerate() {
            if t.u32(0x76c) & 0x8100 == 0x100 {
                if town_ids[i] == r[0] {
                    a = Some(i)
                } else if town_ids[i] == r[1] {
                    b = Some(i)
                }
            }
        }
        if let (Some(a), Some(b)) = (a, b) {
            let x = towns[a].i32(0x2d4);
            let y = towns[b].i32(0x2d4);
            let diff = if x > y { x.wrapping_sub(y) } else { y.wrapping_sub(x) } as u32;
            if diff >= best {
                best = diff;
                route = Some((a, b, r[2]));
            }
        }
    }
    if let Some((a, b, delay)) = route {
        let mut m = u32::from(month) + u32::from(delay);
        let mut y = u32::from(year);
        if m >= 13 {
            m -= 12;
            y = y.wrapping_add(1)
        }
        let offset = *MONTH_DATA.get(m as usize).ok_or("mes de ruta fuera de tabla")?;
        let due = y.wrapping_mul(365).wrapping_add(u32::from(offset)).wrapping_shl(8);
        payload[..4].copy_from_slice(&due.to_le_bytes());
        payload[4] = 1;
        payload[5] = 0xff;
        payload[6] = 0;
        payload[7] = 0;
        payload[8] = a as u8;
        payload[9] = b as u8;
        payload[10] = delay;
        payload[11..16].fill(0);
        out.push(mk(payload));
    }
    // Tipo 2: el barco elegible de la banda con más potencia.
    let mut strength = 1000u32;
    let mut selected = None;
    for band in pirates.iter().flatten() {
        let mut power = u32::from(band.reserve_strength);
        let mut ship = usize::from(band.first_ship);
        let mut candidate = None;
        let mut seen = vec![false; ships.len()];
        while ship < ships.len() {
            if seen[ship] {
                return Err("cadena pirata cíclica");
            }
            seen[ship] = true;
            let b = &ships[ship];
            if b.len() < 0x15e {
                return Err("barco pirata truncado");
            }
            let captain = usize::from(u16::from_le_bytes([b[0x42], b[0x43]]));
            if captain < captains.len() && u16::from(b[0x15d]) >= merchant_count {
                candidate = Some(ship)
            }
            power = power.wrapping_add(u32::from_le_bytes(b[0x120..0x124].try_into().unwrap()));
            ship = usize::from(u16::from_le_bytes([b[4], b[5]]));
        }
        if power > strength {
            if let Some(id) = candidate {
                strength = power;
                selected = Some(id)
            }
        }
    }
    if let Some(id) = selected {
        let mut m = u32::from(month) + 3;
        let mut y = u32::from(year);
        if m >= 13 {
            m -= 12;
            y = y.wrapping_add(1)
        }
        let due = y
            .wrapping_mul(365)
            .wrapping_add(u32::from(MONTH_DATA[m as usize]))
            .wrapping_shl(8);
        let b = &ships[id];
        let ci = usize::from(u16::from_le_bytes([b[0x42], b[0x43]]));
        payload[..4].copy_from_slice(&due.to_le_bytes());
        payload[4] = 2;
        payload[5] = 0xff;
        payload[6] = 0;
        payload[7] = 0;
        payload[8..10].copy_from_slice(&(id as u16).to_le_bytes());
        payload[10..12].copy_from_slice(&u16::from(captains[ci][2]).to_le_bytes());
        payload[12..14].copy_from_slice(&u16::from(captains[ci][3]).to_le_bytes());
        if difficulty < 2 {
            let wanted = usize::from(b[0x158]);
            let group = if wanted < 5 && pirates[wanted].is_some() {
                wanted
            } else {
                pirates.iter().position(Option::is_some).ok_or("banda ausente")?
            };
            pirates[group].as_mut().unwrap().status = difficulty + 2;
        }
        out.push(mk(payload));
    }
    // Tipo 3: la primera banda con más de dos barcos y el recuento máximo.
    let mut most = 2usize;
    let mut group = None;
    for (i, band) in pirates.iter().enumerate() {
        if let Some(band) = band {
            let mut pos = usize::from(band.first_ship);
            let mut count = 0;
            let mut seen = vec![false; ships.len()];
            while pos < ships.len() {
                if seen[pos] {
                    return Err("cadena pirata cíclica");
                }
                seen[pos] = true;
                let b = &ships[pos];
                if b.len() < 6 {
                    return Err("barco pirata truncado");
                }
                count += 1;
                pos = usize::from(u16::from_le_bytes([b[4], b[5]]));
            }
            if count > most {
                most = count;
                group = Some(i)
            }
        }
    }
    if let Some(i) = group {
        let mut m = u32::from(month) + 7;
        let mut y = u32::from(year);
        if m >= 13 {
            m -= 12;
            y = y.wrapping_add(1)
        }
        let due = y
            .wrapping_mul(365)
            .wrapping_add(u32::from(MONTH_DATA[m as usize]))
            .wrapping_shl(8);
        payload[..4].copy_from_slice(&due.to_le_bytes());
        payload[4] = 3;
        payload[5] = 0xff;
        payload[6] = 0;
        payload[7] = 0;
        payload[8] = i as u8;
        payload[9] = pirates[i].unwrap().target;
        payload[10] = 0;
        out.push(mk(payload));
    }
    let mut value = 2001u32;
    let mut target = None;
    for (i, t) in towns.iter().enumerate() {
        let pop = t.u32(0x2d4);
        if t.u32(0x76c) & 0x100 != 0 && pop < value {
            value = pop;
            target = Some(i)
        }
    }
    if let Some(i) = target {
        let due = (u32::from(year) + 1)
            .wrapping_mul(365)
            .wrapping_add(u32::from(MONTH_DATA[usize::from(month)]))
            .wrapping_shl(8);
        let price = ((value + 2000) / 250 * 250) as u16;
        payload[..4].copy_from_slice(&due.to_le_bytes());
        payload[4] = 4;
        payload[5] = 0xff;
        payload[6] = 0;
        payload[7] = 0;
        payload[8..10].copy_from_slice(&price.to_le_bytes());
        payload[10] = i as u8;
        out.push(mk(payload));
    }
    Ok(PreparedAldermanMissions {
        operations: out,
        pirates,
    })
}
