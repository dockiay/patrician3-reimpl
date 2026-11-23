//! Dependencias puras del tick diario de cuadrilla pirata.
use crate::flatmem::Mem;

/// Entradas de 0x004FDF50 que no se guardan en el pool de capitanes.
#[derive(Clone, Copy, Debug)]
pub struct CaptainResources {
    pub tick: u32,
    pub day_of_year: u16,
    pub day: u8,
    pub population: u32,
    pub first_names: u16,
    pub last_names: u16,
}
/// `0x005097C0(1)`: reserva e inicializa un capitán con flag=false.
/// La memoria sintética admite extensión in situ: el anfitrión debe reservar
/// capacidad y publicarla atómicamente con el resto del gestor antes de llamar.
pub fn allocate_pirate_captain(m: &mut dyn Mem, r: CaptainResources) -> Result<u16, String> {
    if r.first_names == 0 || r.last_names == 0 {
        return Err("nombres de capitán ausentes".into());
    }
    const MANAGER: u32 = 0x6dd7a0;
    let base = m.r32(MANAGER);
    if base == 0 {
        return Err("pool de capitanes ausente".into());
    }
    let head = m.r16(MANAGER + 0xe0);
    let old = m.r16(MANAGER + 0xf2);
    let cap = if head >= old {
        if head != old || u32::from(old) + 64 > u32::from(u16::MAX) {
            return Err("pool de capitanes no ampliable".into());
        }
        let new = old + 64;
        for i in old..new {
            let at = base + u32::from(i) * 16;
            for k in 0..16 {
                m.w8_seed(at + k);
                m.w8(at + k, 255)
            }
            m.w16(at, i + 1)
        }
        m.w16(MANAGER + 0xf2, new);
        new
    } else {
        old
    };
    if head >= cap {
        return Err("cabecera de capitanes inválida".into());
    }
    let at = base + u32::from(head) * 16;
    let next = m.r16(at);
    let mut record = [0u8; 16];
    for (i, x) in record.iter_mut().enumerate() {
        *x = m.r8(at + i as u32)
    }
    record[..2].fill(255);
    crate::ships::factory::init_captain(
        &mut record,
        u32::from(head),
        false,
        r.tick,
        r.day_of_year,
        r.day,
        r.population,
        r.first_names,
        r.last_names,
    )
    .ok_or("inicialización de capitán inválida")?;
    m.w16(MANAGER + 0xe0, next);
    for (i, &v) in record.iter().enumerate() {
        m.w8(at + i as u32, v)
    }
    Ok(head)
}

/// Nivel de armamento de ciudad `0x0052BB00`, a partir de Town+0x810.
pub fn town_weapon_level(experience: u32) -> u32 {
    let scaled = ((u64::from(experience) * 0x09fc8735u64) >> 46) as u32;
    scaled.saturating_add(2).min(5)
}

/// Armado de un barco, `0x0051A4E0`; el anfitrión suministra meta de potencia
/// y nivel de ciudad. Conserva la semántica entera y los registros de carga.
pub fn arm_ship(ship: &mut [u8], target: u32, level: u32) -> Result<(), String> {
    if ship.len() < 0x154 {
        return Err("barco de armamento truncado".into());
    }
    const WEIGHT: [u32; 8] = [1000, 1000, 2000, 2000, 2000, 1000, 0x2020, 0x4d4d];
    const POWER: [u32; 8] = [9, 10, 22, 24, 30, 18, 0, 0];
    fn get(s: &[u8], i: usize) -> u32 {
        u32::from_le_bytes(s[i..i + 4].try_into().unwrap())
    }
    fn put(s: &mut [u8], i: usize, v: u32) {
        s[i..i + 4].copy_from_slice(&v.to_le_bytes())
    }
    let mut free = get(ship, 0x10)
        .wrapping_sub(get(ship, 0x118))
        .wrapping_sub(get(ship, 0x11c)) as i32;
    let mut power = get(ship, 0x120);
    let mut weight = get(ship, 0x11c);
    let mut class = (level as u8).min(4);
    if target <= power || free < 1000 || class < 2 {
        return Ok(());
    }
    for slot in (0..24).step_by(2) {
        if ship[0x13c + slot] != 255 || ship[0x13d + slot] != 255 {
            continue;
        }
        let w = WEIGHT[usize::from(class)];
        if w as i32 > free {
            break;
        }
        ship[0x13c + slot] = class;
        ship[0x13d + slot] = 6;
        weight = weight.wrapping_add(w);
        free = free.wrapping_sub(w as i32);
        power = power.wrapping_add(POWER[usize::from(class)]);
        if target <= power {
            put(ship, 0x11c, weight);
            put(ship, 0x120, power);
            return Ok(());
        }
    }
    for slot in (0..24).step_by(2) {
        let first = ship[0x13c + slot];
        let second = ship[0x13d + slot];
        if first >= class || second == 7 || (free < 1000 && second > 7) {
            continue;
        }
        let w = WEIGHT[usize::from(first)];
        power = power.wrapping_sub(POWER[usize::from(first)]);
        weight = weight.wrapping_sub(w);
        free = free.wrapping_add(w as i32);
        if class > second {
            if usize::from(second) >= WEIGHT.len() {
                return Err("ranura de armamento fuera de tabla".into());
            }
            let w = WEIGHT[usize::from(second)];
            power = power.wrapping_sub(POWER[usize::from(second)]);
            weight = weight.wrapping_sub(w);
            free = free.wrapping_add(w as i32)
        }
        ship[0x13c + slot] = class;
        ship[0x13d + slot] = 6;
        let w = WEIGHT[usize::from(class)];
        weight = weight.wrapping_add(w);
        free = free.wrapping_sub(w as i32);
        power = power.wrapping_add(POWER[usize::from(class)]);
        if target <= power {
            put(ship, 0x11c, weight);
            put(ship, 0x120, power);
            return Ok(());
        }
    }
    class = class.min(1);
    for slot in 0..24 {
        if ship[0x13c + slot] != 255 {
            continue;
        }
        let w = WEIGHT[usize::from(class)];
        if w as i32 > free {
            break;
        }
        ship[0x13c + slot] = class;
        weight = weight.wrapping_add(w);
        free = free.wrapping_sub(w as i32);
        power = power.wrapping_add(POWER[usize::from(class)]);
        if target <= power {
            break;
        }
    }
    put(ship, 0x11c, weight);
    put(ship, 0x120, power);
    Ok(())
}

/// Reserva 0x00507D60(&id,0,3), con crecimiento sintético del pool de 64.
/// La memoria sintética conserva direcciones; el anfitrión debe relocalizar
/// sus vectores y referencias al publicar el resultado de la transacción.
pub fn allocate_reserve_ship(m: &mut dyn Mem, tick: u32, port: (u16, u16)) -> Result<(u32, u16), String> {
    const M: u32 = 0x6dd7a0;
    let mut count = m.r16(M + 0xf4);
    let mut id = m.r16(M + 0xe2);
    let base = m.r32(M + 4);
    if base == 0 {
        return Err("pool naval ausente".into());
    }
    if id >= count {
        if u32::from(count) + 64 > u32::from(u16::MAX) {
            return Err("pool naval no ampliable".into());
        }
        id = count;
        let new = count + 64;
        for i in count..new {
            let at = base + u32::from(i) * 0x180;
            for k in 0..0x180 {
                m.w8_seed(at + k)
            }
            m.w16(at + 6, i + 1);
            m.w16(at + 8, 0xffff);
            m.w16(at + 4, 0xffff);
            m.w16(at + 0x134, 0xff);
            m.w8(at + 0x160, 0);
            m.w32(at + 0x24, 0);
            m.w8(at + 0x36, 0);
            m.w8(at + 0x137, 0)
        }
        count = new;
        m.w16(M + 0xf4, count);
        m.w16(M + 0xe2, id)
    }
    let addr = base + u32::from(id) * 0x180;
    let mut ship = [0u8; 0x180];
    for (i, v) in ship.iter_mut().enumerate() {
        *v = m.r8(addr + i as u32)
    }
    let next = u16::from_le_bytes([ship[6], ship[7]]);
    crate::ships::factory::reset_ship(&mut ship, tick).ok_or("reinicio naval inválido")?;
    ship[0x137] = 0;
    ship[0x134..0x136].copy_from_slice(&0x12u16.to_le_bytes());
    ship[4..8].fill(255);
    ship[0x4c..0x50].copy_from_slice(&tick.wrapping_sub((tick & 255) << 10).to_le_bytes());
    crate::ships::factory::init_ship_at_town(&mut ship, 0, port.0, port.1, tick).ok_or("puerto inicial inválido")?;
    ship[0x13b] = 0;
    for (i, &v) in ship.iter().enumerate() {
        m.w8(addr + i as u32, v)
    }
    m.w16(M + 0xe2, next);
    Ok((addr, id))
}

/// 0x00515F00: desliga un barco de la lista +4 de la cuadrilla, sin tocar
/// su lista de reserva +6. Rechaza ciclos corruptos en lugar de colgarse.
pub fn unlink_band_ship(m: &mut dyn Mem, band: u32, id: u16) -> Result<(), String> {
    let count = m.r16(0x6dd894);
    let base = m.r32(0x6dd7a4);
    let mut link = band + 0xc;
    let mut cur = m.r16(link);
    let mut seen = vec![false; usize::from(count)];
    while cur < count {
        let i = usize::from(cur);
        if seen[i] {
            return Err("cadena de banda cíclica".into());
        }
        seen[i] = true;
        let addr = base + u32::from(cur) * 0x180;
        if cur == id {
            let next = m.r16(addr + 4);
            m.w16(link, next);
            m.w16(addr + 4, 0xffff);
            return Ok(());
        }
        link = addr + 4;
        cur = m.r16(link)
    }
    Ok(())
}

/// `route_stop_pool_free_route` (0x004D4DD0), argumento de propagación cero.
/// La cadena de paradas debe cerrar sobre `id`; se valida antes de mutar.
pub fn free_route_stop_pool(head: &mut u16, records: &mut [u8], id: u16) -> Result<(), String> {
    const SIZE: usize = 220;
    if records.len() % SIZE != 0 || records.len() / SIZE > usize::from(u16::MAX) {
        return Err("depósito de rutas truncado".into());
    }
    let count = records.len() / SIZE;
    if usize::from(id) >= count {
        return Ok(());
    }
    let mut order = Vec::new();
    let mut seen = vec![false; count];
    let mut cur = id;
    loop {
        let i = usize::from(cur);
        if i >= count || seen[i] {
            return Err("cadena de paradas no circular".into());
        }
        seen[i] = true;
        order.push(i);
        let at = i * SIZE;
        let next = u16::from_le_bytes([records[at], records[at + 1]]);
        if next == id {
            break;
        }
        cur = next
    }
    if order.len() > 1 {
        for &i in &order[..order.len() - 1] {
            records[i * SIZE + 3] = 0
        }
    }
    let last = *order.last().unwrap();
    records[last * SIZE..last * SIZE + 2].copy_from_slice(&head.to_le_bytes());
    *head = id;
    Ok(())
}

/// 4D4DD0(índice,1): antes de liberar, 509030 desvincula de cada parada
/// todos los barcos del pool, incluso los libres, igual que el original.
pub fn free_route_stop_pool_with_followers(
    head: &mut u16,
    records: &mut [u8],
    ships: &mut [u8],
    id: u16,
) -> Result<(), String> {
    const SIZE: usize = 220;
    if records.len() % SIZE != 0 || ships.len() % 0x180 != 0 || records.len() / SIZE > usize::from(u16::MAX) {
        return Err("depósito o flota truncados".into());
    }
    let count = records.len() / SIZE;
    if usize::from(id) >= count {
        return Ok(());
    }
    let mut ids = Vec::new();
    let mut seen = vec![false; count];
    let mut cur = id;
    loop {
        let i = usize::from(cur);
        if i >= count || seen[i] {
            return Err("cadena de paradas no circular".into());
        }
        seen[i] = true;
        ids.push(cur);
        let at = i * SIZE;
        let next = u16::from_le_bytes([records[at], records[at + 1]]);
        if next == id {
            break;
        }
        cur = next
    }
    for ship in ships.chunks_exact_mut(0x180) {
        let id = u16::from_le_bytes([ship[0x132], ship[0x133]]);
        if ids.contains(&id) {
            ship[0x132..0x134].fill(255)
        }
    }
    free_route_stop_pool(head, records, id)
}
