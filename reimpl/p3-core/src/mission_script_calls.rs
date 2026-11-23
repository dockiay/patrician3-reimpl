//! Funciones del original que llaman los comandos de guion de misión (`mission_script_flat`) y que aún no
//! tenían puerto, traducidas sobre memoria plana con las direcciones del juego.
//!
//! - `0x00518640` [`ship_add_passengers`] y `0x005186D0` [`ship_remove_passengers`] (`0xF1`).
//! - `0x00533210` [`find_nearest_town_excluding`] (`0x6F`).
//! - `0x00531260` [`date_to_ticks`] (`0x50`, `0x5C`).
//! - `0x0051BEB0` [`town_scarcest_produced_ware`] (`0x2F`).
//! - `0x005182B0` [`recompute_load`] sobre los bytes del barco (puerto de `ships::cargo`).
//! - `0x0050E000` [`create_docked_ship_for_merchant`] (`0x67`), con `0x00507D60` (modo 1), `0x0052A8D0`,
//!   `0x0051A4E0`/`0x0051A750` (armamento mínimo) y el resto de la fábrica de `ships::factory`.
//!
//! Contraste: `oracle/x86_c10_script_calls.py` y `tests/c10_script_calls_x86.rs`.
use crate::flatmem::Mem;
use crate::mission_script_flat::{mark_changed, SHIPS};

const SHIP_SIZE: usize = 0x180;

/// Peso de un pasajero (`·5·5·5 << 4`).
fn passenger_weight(n: u8) -> i32 {
    i32::from(n) * 2000
}

/// `ship_recompute_load` (`0x005182B0`) sobre la imagen del barco en memoria; devuelve el peso libre.
pub fn recompute_load(m: &mut dyn Mem, ship: u32) -> u32 {
    let mut image: Vec<u8> = (0..SHIP_SIZE as u32).map(|k| m.r8(ship + k)).collect();
    let before = image.clone();
    let free = crate::ships::cargo::recompute_load(&mut image);
    for (k, (a, b)) in before.iter().zip(&image).enumerate() {
        if a != b {
            m.w8(ship + k as u32, *b);
        }
    }
    free as u32
}

/// `ship_add_passengers` (`0x00518640`, thiscall, `ret 4`): sube `n` pasajeros (`+0x156`, byte) si el peso libre
/// (`capacidad − pasajeros·2000 − equipo +0x11C − carga +0x118`, con signo) alcanza `n·2000`; devuelve `AL`
/// (1 si suben). El original suma a `+0x118` el peso del total de pasajeros, no solo el de los nuevos.
pub fn ship_add_passengers(m: &mut dyn Mem, ship: u32, n: u32) -> u8 {
    let have = m.r8(ship + 0x156);
    let free = (m.r32(ship + 0x10) as i32)
        .wrapping_sub(passenger_weight(have))
        .wrapping_sub(m.r32(ship + 0x11c) as i32)
        .wrapping_sub(m.r32(ship + 0x118) as i32);
    if free < passenger_weight(n as u8) {
        return 0;
    }
    let total = have.wrapping_add(n as u8);
    m.w8(ship + 0x156, total);
    let load = m.r32(ship + 0x118);
    m.w32(ship + 0x118, (passenger_weight(total) as u32).wrapping_add(load));
    mark_changed(m, ship, 4);
    1
}

/// `ship_remove_passengers` (`0x005186D0`, thiscall, `ret 4`): baja `n` pasajeros. Con menos de `n` los deja en
/// 0 y recuenta la bodega; si la carga `+0x118` no supera `n·2000` también recuenta; si no, le resta el peso de
/// los pasajeros que **quedan** (como el original).
pub fn ship_remove_passengers(m: &mut dyn Mem, ship: u32, n: u32) {
    let have = m.r8(ship + 0x156);
    let n = n as u8;
    if have < n {
        m.w8(ship + 0x156, 0);
        recompute_load(m, ship);
    } else {
        let left = have - n;
        m.w8(ship + 0x156, left);
        let load = m.r32(ship + 0x118) as i32;
        if load <= passenger_weight(n) {
            recompute_load(m, ship);
        } else {
            m.w32(ship + 0x118, load.wrapping_sub(passenger_weight(left)) as u32);
        }
    }
    mark_changed(m, ship, 4);
}

/// `find_nearest_town_excluding` (`0x00533210`, thiscall sobre el mundo, `ret 0x14`).
///
/// Argumentos: ciudad de origen, ciudades excluidas (el original recibe un puntero; aquí sus valores, de los que
/// se leen los `count` primeros), `count` (con signo), `alternative` (byte bajo) y barco (palabra baja). Elige la
/// ciudad de menor distancia (`matriz[i·n + origen]`, `World+0x6C`) que no esté excluida; con un barco válido de
/// tipo `> 1` salta las ciudades con el bit 0 de `Town+0x2C8`. Con `alternative`, recorre de nuevo las ciudades
/// a distancia `< mejor + 2` y devuelve la primera aceptable en la que un contador que empieza en la hora del
/// mundo (`+0x14`) sea múltiplo de 3 (el contador avanza solo en esas ciudades).
pub fn find_nearest_town_excluding(
    m: &mut dyn Mem,
    world: u32,
    from: u32,
    excluded: &[u32],
    count: u32,
    alternative: u32,
    ship: u32,
) -> Result<u32, String> {
    let ship = ship as u16;
    let mut kind = 0u8;
    let has_ship = ship != 0xffff && ship < m.r16(SHIPS + 0xf4);
    if has_ship {
        let s = m.r32(SHIPS + 4).wrapping_add(u32::from(ship) * SHIP_SIZE as u32);
        kind = m.r8(s + 0xe);
    }
    let count = count as i32;
    let is_excluded = |i: u32| -> Result<bool, String> {
        for k in 0..count.max(0) as usize {
            let x = *excluded.get(k).ok_or("0x533210: más ciudades excluidas que las pasadas")?;
            if x == i {
                return Ok(true);
            }
        }
        Ok(false)
    };
    let towns = m.r32(world + 0x10);
    let towns_base = m.r32(0x006d_e508);
    let blocked = |m: &mut dyn Mem, i: u32| has_ship && m.r8(towns_base.wrapping_add(0x2c8).wrapping_add(i.wrapping_mul(0x9f8))) & 1 != 0 && kind > 1;
    let mut best = 0x0fff_ffffu32;
    let mut result = 0u32;
    for i in 0..towns {
        if i == from {
            continue;
        }
        let matrix = m.r32(world + 0x6c);
        let d = u32::from(m.r8(matrix.wrapping_add(i.wrapping_mul(towns)).wrapping_add(from)));
        if d >= best || blocked(m, i) || is_excluded(i)? {
            continue;
        }
        result = i;
        best = d;
    }
    if alternative as u8 != 0 {
        let mut counter = m.r32(0x006d_e4b4);
        for i in 0..towns {
            if i == from {
                continue;
            }
            let matrix = m.r32(world + 0x6c);
            let d = u32::from(m.r8(matrix.wrapping_add(i.wrapping_mul(towns)).wrapping_add(from)));
            if d >= best.wrapping_add(2) {
                continue;
            }
            let rem = counter % 3;
            counter = counter.wrapping_add(1);
            if rem != 0 || blocked(m, i) {
                continue;
            }
            if !is_excluded(i)? {
                return Ok(i);
            }
        }
    }
    Ok(result)
}

/// `date_to_ticks` (`0x00531260`, `ret 0xC`): `((año·365 + días antes del mes + día − 1) << 8)` con las tablas
/// `0x00672D78` (días acumulados) y `0x00672D94` (días del mes, el día se acota a ellos); un mes `> 11` vale el
/// año siguiente y un día `<= 1` no suma.
pub fn date_to_ticks(m: &mut dyn Mem, day: u32, month: u32, year: u32) -> u32 {
    let mut t = (year & 0xffff).wrapping_mul(365);
    let month = month as u8;
    if month > 0xb {
        return t.wrapping_add(0x16d) << 8;
    }
    let at = u32::from(month) * 2;
    t = t.wrapping_add(u32::from(m.r16(0x0067_2d78 + at)));
    let day = day as u8;
    if day <= 1 {
        return t << 8;
    }
    let len = m.r16(0x0067_2d94 + at);
    let d = if u16::from(day) > len { u32::from(len) } else { u32::from(day) };
    t.wrapping_add(d).wrapping_sub(1) << 8
}

/// `town_scarcest_produced_ware` (`0x0051BEB0`, thiscall sobre la ciudad): de las 20 mercancías (sin la 6) con
/// producción (`Town+0x844+16·w != 0`) y existencias por debajo del umbral `+0x4F4+16·w`, la de menor cociente
/// `existencias / umbral +0x4F0+16·w` (`f32`, por debajo de 400); `0x1F` si ninguna.
pub fn town_scarcest_produced_ware(m: &mut dyn Mem, town: u32) -> u32 {
    let mut best = 400.0f32;
    let mut result = 0x1fu32;
    for w in 0..20u32 {
        if w == 6 || m.r16(town + 0x844 + 0x10 * w) == 0 {
            continue;
        }
        let stock = m.r32(town + 4 + 4 * w) as i32;
        if stock >= m.r32(town + 0x4f4 + 0x10 * w) as i32 {
            continue;
        }
        let ratio = stock as f32 / m.r32(town + 0x4f0 + 0x10 * w) as i32 as f32;
        if ratio < best {
            best = ratio;
            result = w;
        }
    }
    result
}

/// Recursos de `0x0050E000` que no están en la memoria plana: banco de nombres (`0x006DDAA0`, su tamaño es
/// `0x006DDB78`) y puerto de cada ciudad (`0x006DDBB0[GameWorld+0x18+ciudad]`).
pub struct DockedShipResources<'a> {
    pub ship_names: &'a [Vec<u8>],
    pub ports: &'a [(u16, u16)],
}

/// Tripulación inicial por tipo (`0x00673660`).
const START_CREW: [u16; 4] = [5, 8, 10, 12];

fn read_ship(m: &mut dyn Mem, at: u32) -> Vec<u8> {
    (0..SHIP_SIZE as u32).map(|k| m.r8(at + k)).collect()
}
fn write_ship(m: &mut dyn Mem, at: u32, image: &[u8]) {
    for (k, b) in image.iter().enumerate() {
        m.w8(at + k as u32, *b);
    }
}

/// `create_docked_ship_for_merchant` (`0x0050E000`, thiscall sobre el gestor naval `0x006DD7A0`, `ret 0x10`).
///
/// Argumentos: tipo (el original recibe un puntero a una palabra que puede rebajar a 1), armamento mínimo,
/// ciudad (byte) y comerciante (byte). Reserva un barco libre (`0x00507D60` modo 1: lista de atracados
/// `+0xE8`, estado 0), lo coloca en el puerto, le da las estadísticas del astillero de la ciudad, lo arma hasta
/// el mínimo subiendo niveles si hace falta, lo pone en la cadena del comerciante (`+0xE`) en estado 3 con
/// nombre del banco y fecha de botadura, y suma uno a `Merchant+0x478`. Devuelve `(índice, tipo final)`.
///
/// Si no hay barcos libres el original crea 64 en un bloque nuevo; aquí se escriben a continuación de la tabla
/// (el puntero `+4` no cambia) y sus bytes no inicializados por el original quedan a cero.
pub fn create_docked_ship_for_merchant(
    m: &mut dyn Mem,
    manager: u32,
    kind: u32,
    minimum: u32,
    town: u32,
    who: u32,
    r: &DockedShipResources<'_>,
) -> Result<(u16, u16), String> {
    let (town, who) = (town & 0xff, who & 0xff);
    let mut kind = kind as u16;
    let towns = m.r32(0x006d_e508);
    let tp = towns.wrapping_add(town.wrapping_mul(0x9f8));
    if who >= u32::from(m.r16(0x006d_e4aa)) {
        return Err(format!("0x50E000: comerciante {who} fuera de la tabla"));
    }
    let merchant = m.r32(0x006d_e518).wrapping_add(who.wrapping_mul(0x650));
    if m.r32(tp + 0x2c8) & 1 != 0 && kind & 2 != 0 {
        kind = 1;
    }
    if kind & 0xff >= 4 {
        return Err(format!("0x50E000: tipo de barco {kind} fuera de la tabla del astillero"));
    }
    // 0x00507D60, modo 1.
    let mut count = m.r16(manager + 0xf4);
    if m.r16(manager + 0xe2) >= count {
        if count > u16::MAX - 0x40 {
            return Err("0x50E000: tabla de barcos llena".into());
        }
        m.w16(manager + 0xe2, count);
        let base = m.r32(manager + 4);
        for i in count..count + 0x40 {
            let s = base.wrapping_add(u32::from(i) * SHIP_SIZE as u32);
            write_ship(m, s, &[0; SHIP_SIZE]);
            m.w16(s + 6, i + 1);
            m.w16(s + 8, 0xffff);
            m.w16(s + 4, 0xffff);
            m.w16(s + 0x134, 0xff);
        }
        count += 0x40;
        m.w16(manager + 0xf4, count);
    }
    let id = m.r16(manager + 0xe2);
    let at = m.r32(manager + 4).wrapping_add(u32::from(id) * SHIP_SIZE as u32);
    let mut ship = read_ship(m, at);
    m.w16(manager + 0xe2, u16::from_le_bytes([ship[6], ship[7]]));
    let tick = m.r32(0x006d_e4b4);
    crate::ships::factory::reset_ship(&mut ship, tick).ok_or("barco truncado")?;
    ship[0x137] = 0;
    ship[0x134..0x136].copy_from_slice(&0u16.to_le_bytes());
    ship[6..8].copy_from_slice(&m.r16(manager + 0xe8).to_le_bytes());
    m.w16(manager + 0xe8, id);
    let &(x, y) = r.ports.get(town as usize).ok_or("0x50E000: puerto de la ciudad ausente")?;
    crate::ships::factory::init_ship_at_town(&mut ship, town as u8, x, y, tick).ok_or("barco truncado")?;
    ship[0x13b] = town as u8;
    // 0x0052A8D0 sobre la ciudad.
    let mut t = crate::town::Town::default();
    for (k, b) in t.0.iter_mut().enumerate() {
        *b = m.r8(tp + k as u32);
    }
    crate::ships::factory::init_ship_stats(&t, &mut ship, kind as u8).ok_or("0x50E000: nivel de astillero fuera de la tabla")?;
    if minimum > 0 {
        crate::ships::cargo::recompute_load(&mut ship);
        let level = crate::economy::production::shipyard_level(&t);
        crate::ships::factory::arm_to_minimum(&mut ship, minimum, level).ok_or("barco truncado")?;
        while minimum > u32::from_le_bytes(ship[0x120..0x124].try_into().expect("barco completo")) && ship[0xf] < 2 {
            crate::ships::factory::upgrade_level(&mut ship, 1).ok_or("0x50E000: nivel de barco fuera de la tabla")?;
            crate::ships::factory::arm_to_minimum(&mut ship, minimum, level).ok_or("barco truncado")?;
        }
    }
    let hull = ship[0x14..0x18].to_vec();
    ship[0x18..0x1c].copy_from_slice(&hull);
    ship[0] = who as u8;
    ship[0x3d] |= 4;
    ship[4..6].copy_from_slice(&m.r16(merchant + 0xe).to_le_bytes());
    m.w16(merchant + 0xe, id);
    ship[0x13a] = if m.r16(merchant + 8) == 0 { m.r8(merchant + 0x19) } else { town as u8 };
    ship[0x134..0x136].copy_from_slice(&3u16.to_le_bytes());
    if m.r32(0x006d_f2f0) & 2 != 0 {
        ship[0x138..0x13a].copy_from_slice(&0x15u16.to_le_bytes());
    }
    let crew = START_CREW[usize::from(ship[0xe] & 3)];
    ship[0x40..0x42].copy_from_slice(&crew.to_le_bytes());
    crate::ships::cargo::recompute_load(&mut ship);
    crate::ships::factory::recompute_weapon_slots(&mut ship).ok_or("0x50E000: nivel de barco fuera de la tabla")?;
    let upkeep = u16::from(tick as u8).wrapping_add(id).wrapping_add(0xfa0);
    ship[0x3e..0x40].copy_from_slice(&(if upkeep as i16 > 0x700 { 0x700 } else { upkeep }).to_le_bytes());
    let mut cursor = m.r16(manager + 0xee);
    let step = m.r16(manager + 0xf0);
    if r.ship_names.len() != usize::from(m.r16(0x006d_db78)) {
        return Err("0x50E000: banco de nombres distinto de 0x6DDB78".into());
    }
    let name = crate::ships::factory::next_ship_name_index(&mut cursor, step, r.ship_names.len()).ok_or("0x50E000: el original no sale del bucle de nombres")?;
    m.w16(manager + 0xee, cursor);
    ship[0x15e..0x160].copy_from_slice(&name.to_le_bytes());
    let bytes = &r.ship_names[usize::from(name)];
    let size = bytes.iter().position(|&v| v == 0).unwrap_or(bytes.len()).min(31);
    ship[0x160..0x160 + size].copy_from_slice(&bytes[..size]);
    ship[0x160 + size] = 0;
    ship[0x3c] |= 1;
    let age = (u32::from(id & 3) + 2) * 365 + (t.u32(0x31c) & 0xff) + town + u32::from(kind);
    ship[0x4c..0x50].copy_from_slice(&tick.wrapping_sub(age << 8).to_le_bytes());
    write_ship(m, at, &ship);
    let built = m.r16(merchant + 0x478);
    m.w16(merchant + 0x478, built.wrapping_add(1));
    Ok((id, kind))
}
