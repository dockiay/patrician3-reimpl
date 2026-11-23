//! Piezas de creación naval utilizadas por la expedición de la Liga.
use crate::town::Town;
const LEVELS: [[u8; 4]; 4] = [[15, 19, 23, 25], [28, 31, 34, 35], [45, 48, 52, 55], [55, 59, 65, 70]];
const SLOTS: [u16; 12] = [0, 5, 29, 0, 5, 61, 240, 252, 1021, 252, 1021, 4095];
/// Estadísticas básicas del astillero (0x0052a8d0).
pub fn init_ship_stats(town: &Town, ship: &mut [u8], kind: u8) -> Option<()> {
    if ship.len() < 0x180 || kind >= 4 {
        return None;
    }
    let k = kind as usize;
    let level = (town.u8(0x824 + k) as i8).max(0) as usize;
    if level >= 4 {
        return None;
    }
    let points = (u16::from(LEVELS[k][level]) + u16::from(town.u8(0x828 + k))).min(u16::from(LEVELS[k][3]));
    ship[0xe] = kind;
    ship[0xf] = 0;
    ship[0x10..0x14].copy_from_slice(&(u32::from(points) * 2000).to_le_bytes());
    ship[0x14..0x18].copy_from_slice(&(u32::from(points) * 0xaf0).to_le_bytes());
    Some(())
}
/// Recalcula ranuras de artillería (0x005181d0).
pub fn recompute_weapon_slots(ship: &mut [u8]) -> Option<()> {
    if ship.len() < 0x180 || ship[0xf] > 2 {
        return None;
    }
    let mut mask = SLOTS[usize::from(ship[0xe] & 3) * 3 + usize::from(ship[0xf])];
    for i in 0..6 {
        let slot = &mut ship[0x13c + i * 4..0x140 + i * 4];
        match mask & 3 {
            0 => slot.fill(7),
            1 => {
                for j in [0, 2] {
                    if slot[j] > 1 && slot[j] != 5 {
                        slot[j] = 255
                    }
                }
                slot[1] = 7;
                slot[3] = 7
            }
            _ => {
                for j in [1, 3] {
                    if slot[j] > 1 && slot[j] != 5 {
                        slot[j] = 255
                    }
                }
                for j in [0, 2] {
                    if slot[j] > 1 && slot[j] != 5 {
                        if slot[j] < 6 {
                            slot[j + 1] = 6
                        } else {
                            slot[j] = 255
                        }
                    }
                }
            }
        }
        mask >>= 2
    }
    Some(())
}
/// Peso por tipo de arma (`0x00672CB4`, u16).
const WEAPON_WEIGHT: [u16; 10] = [1000, 1000, 2000, 2000, 2000, 1000, 8224, 19789, 14944, 0];
/// Potencia por tipo de arma (`0x00672CC8`).
const WEAPON_POWER: [u8; 10] = [9, 10, 22, 24, 30, 18, 0, 0, 60, 80];
fn ship_i32(ship: &[u8], at: usize) -> i32 {
    i32::from_le_bytes(ship[at..at + 4].try_into().expect("barco completo"))
}
fn set_ship_i32(ship: &mut [u8], at: usize, v: i32) {
    ship[at..at + 4].copy_from_slice(&v.to_le_bytes())
}
/// `ship_arm_to_minimum` (0x0051a4e0, thiscall, `ret 8`): monta armas del nivel `level` (2..=4; de 1 en la
/// última pasada) hasta que la potencia `+0x120` alcanza `minimum` o no queda peso libre. Tres pasadas sobre
/// las 12 ranuras dobles `+0x13C`: huecos dobles libres, ranuras de nivel inferior (sustituyendo, y quitando
/// la segunda mitad si es de nivel inferior) y medias ranuras libres con armas de nivel 1. Las comparaciones
/// con la potencia son sin signo y las de peso con signo, como en el original.
pub fn arm_to_minimum(ship: &mut [u8], minimum: u32, level: u8) -> Option<()> {
    if ship.len() < 0x180 {
        return None;
    }
    let weak = |ship: &[u8]| minimum > ship_i32(ship, 0x120) as u32;
    if !weak(ship) {
        return Some(());
    }
    let mut free = ship_i32(ship, 0x10).wrapping_sub(ship_i32(ship, 0x118)).wrapping_sub(ship_i32(ship, 0x11c));
    if free < i32::from(WEAPON_WEIGHT[0]) || level < 2 {
        return Some(());
    }
    let mut bl = level.min(4);
    let mount = |ship: &mut [u8], free: &mut i32, kind: u8| {
        let w = i32::from(WEAPON_WEIGHT[usize::from(kind)]);
        set_ship_i32(ship, 0x11c, ship_i32(ship, 0x11c).wrapping_add(w));
        *free = free.wrapping_sub(w);
        set_ship_i32(ship, 0x120, ship_i32(ship, 0x120).wrapping_add(i32::from(WEAPON_POWER[usize::from(kind)])));
    };
    let unmount = |ship: &mut [u8], free: &mut i32, kind: u8| {
        let k = usize::from(kind);
        set_ship_i32(ship, 0x120, ship_i32(ship, 0x120).wrapping_sub(i32::from(*WEAPON_POWER.get(k).expect("arma de nivel inferior"))));
        let w = i32::from(WEAPON_WEIGHT[k]);
        set_ship_i32(ship, 0x11c, ship_i32(ship, 0x11c).wrapping_sub(w));
        *free = free.wrapping_add(w);
    };
    for k in (0..24).step_by(2) {
        if ship[0x13c + k] == 0xff && ship[0x13d + k] == 0xff {
            if i32::from(WEAPON_WEIGHT[usize::from(bl)]) > free {
                break;
            }
            ship[0x13c + k] = bl;
            ship[0x13d + k] = 6;
            mount(ship, &mut free, bl);
            if !weak(ship) {
                return Some(());
            }
        }
    }
    for k in (0..24).step_by(2) {
        let first = ship[0x13c + k];
        let second = ship[0x13d + k];
        if first >= bl || second == 7 || free < i32::from(WEAPON_WEIGHT[0]) && second > 7 {
            continue;
        }
        unmount(ship, &mut free, first);
        if bl > ship[0x13d + k] {
            unmount(ship, &mut free, ship[0x13d + k]);
        }
        ship[0x13c + k] = bl;
        ship[0x13d + k] = 6;
        mount(ship, &mut free, bl);
        if !weak(ship) {
            return Some(());
        }
    }
    bl = bl.min(1);
    for k in 0..24 {
        if ship[0x13c + k] == 0xff {
            if i32::from(WEAPON_WEIGHT[usize::from(bl)]) > free {
                return Some(());
            }
            ship[0x13c + k] = bl;
            mount(ship, &mut free, bl);
            if !weak(ship) {
                return Some(());
            }
        }
    }
    Some(())
}
/// `ship_upgrade_level` (0x0051a750, thiscall, `ret 4`): sube hasta `n` niveles (`+0xF`, máximo 2); cada nivel
/// resta `2000·k` a la capacidad `+0x10` y suma `2800·k` al casco máximo `+0x14` (k = 3, 5, 8, 10 según el
/// tipo); marca `+0x3C |= 0x104` y recalcula ranuras y carga.
pub fn upgrade_level(ship: &mut [u8], n: u8) -> Option<()> {
    if ship.len() < 0x180 {
        return None;
    }
    let steps = 2u8.wrapping_sub(ship[0xf]).min(n);
    if steps == 0 {
        return Some(());
    }
    let per = [3i32, 5, 8, 10][usize::from(ship[0xe] & 3)];
    ship[0xf] = ship[0xf].wrapping_add(steps);
    let k = i32::from(steps) * per;
    let marks = u16::from_le_bytes([ship[0x3c], ship[0x3d]]) | 0x104;
    ship[0x3c..0x3e].copy_from_slice(&marks.to_le_bytes());
    set_ship_i32(ship, 0x14, ship_i32(ship, 0x14).wrapping_add(k.wrapping_mul(2800)));
    set_ship_i32(ship, 0x10, ship_i32(ship, 0x10).wrapping_sub(k.wrapping_mul(2000)));
    recompute_weapon_slots(ship)?;
    super::cargo::recompute_load(ship);
    Some(())
}
/// Tripulantes adicionales permitidos (0x005184f0); recalcula la carga.
pub fn additional_sailors(ship: &mut [u8]) -> Option<i32> {
    if ship.len() < 0x180 {
        return None;
    }
    let free = super::cargo::recompute_load(ship);
    let kind = usize::from(ship[0xe] & 3);
    let cap = i32::from_le_bytes(ship[0x10..0x14].try_into().ok()?);
    let sailors = i32::from(u16::from_le_bytes(ship[0x40..0x42].try_into().ok()?));
    let complement = (cap / 2000 + i32::from(ship[0xf]) * [3, 5, 8, 10][kind]).max(20) - sailors;
    if free < 0 {
        return Some(0);
    }
    let room = free / 400 + ([10, 16, 30, 24][kind] - sailors).max(0);
    Some(complement.min(room))
}
/// `ship_reset_to_defaults` (0x00517e00).
pub fn reset_ship(ship: &mut [u8], tick: u32) -> Option<()> {
    if ship.len() < 0x180 {
        return None;
    }
    let saved = ship[0x137];
    ship[..0x180].fill(0);
    ship[0x137] = saved;
    ship[0] = 255;
    for i in [4, 6, 8, 10, 12, 0x3a, 0x3c, 0x42, 0x132, 0x15e] {
        ship[i..i + 2].fill(255)
    }
    ship[0x38] = 1;
    ship[0x39] = 255;
    ship[0x3e..0x40].copy_from_slice(&1000u16.to_le_bytes());
    ship[0x44..0x48].copy_from_slice(&tick.to_le_bytes());
    ship[0x4c..0x50].copy_from_slice(&tick.to_le_bytes());
    ship[0x50..0x54].fill(255);
    ship[0x134..0x136].copy_from_slice(&255u16.to_le_bytes());
    ship[0x13c..0x154].fill(7);
    ship[0x159] = 255;
    ship[0x15d] = 255;
    Some(())
}
/// `ship_init_at_town` (0x00516b10); coordenadas enteras del puerto.
pub fn init_ship_at_town(ship: &mut [u8], town: u8, x: u16, y: u16, tick: u32) -> Option<()> {
    if ship.len() < 0x180 {
        return None;
    }
    ship[8..14].fill(255);
    ship[0x1c..0x36].fill(0);
    ship[0x1e..0x20].copy_from_slice(&x.to_le_bytes());
    ship[0x22..0x24].copy_from_slice(&y.to_le_bytes());
    ship[0x36] = 0;
    ship[0x37] = town;
    ship[0x38] = town;
    ship[0x39] = town;
    ship[0x3a..0x3e].fill(255);
    ship[0x3e..0x42].fill(0);
    ship[0x42..0x44].fill(255);
    ship[0x44..0x48].copy_from_slice(&tick.to_le_bytes());
    ship[0x48..0x4c].fill(0);
    ship[0x50..0x54].fill(255);
    Some(())
}
/// Inicialización de capitán (0x004fdf50). Los tamaños de los bancos de nombres
/// son datos del recurso y no deben inventarse si no están disponibles.
pub fn init_captain(
    cap: &mut [u8],
    index: u32,
    flag: bool,
    tick: u32,
    year: u16,
    day: u8,
    population: u32,
    first_names: u16,
    last_names: u16,
) -> Option<u32> {
    if cap.len() < 16 || first_names == 0 || last_names == 0 {
        return None;
    }
    let y = u32::from(year);
    let t = tick & 255;
    let d = u32::from(day);
    cap[2] = (t.wrapping_add(y).wrapping_add(index) % u32::from(first_names)) as u8;
    cap[3] = (d.wrapping_add(t).wrapping_add(index >> 1).wrapping_add(y) % u32::from(last_names)) as u8;
    let age = ((population.wrapping_add(d).wrapping_add(t).wrapping_add(y) & 31) + 48) * 0xb680
        + ((d.wrapping_add(y).wrapping_add(index) & 31) * 0x700);
    let hire = if age < tick { tick - age } else { 0 };
    cap[4..8].copy_from_slice(&hire.to_le_bytes());
    let scaled = |x: u32, k: f32| ((x as f32 as f64 * k as f64) as f32).to_bits() as u8;
    let a = scaled(t.wrapping_add(index), 0.13579000532627106f32);
    let mut remaining = 600 - u32::from(a);
    let b = u32::from(scaled(index.wrapping_sub(t), 2468.1201171875f32)).min(remaining) as u8;
    remaining -= u32::from(b);
    let c = u32::from(scaled(y.wrapping_sub(index), 12345.2001953125f32)).min(remaining) as u8;
    let category = scaled(t.wrapping_add(index), 0.12345000356435776f32);
    cap[9] = a;
    cap[10] = b;
    cap[11] = c;
    cap[14] = 0;
    cap[15] = 255;
    if flag {
        cap[8] = category % 11;
        cap[12..14].fill(0);
        Some(u32::from(category / 11))
    } else {
        cap[8] = category;
        let wage = (u32::from(a) + u32::from(b) + u32::from(c)) / 50 + 10 + u32::from(category % 11);
        cap[12..14].copy_from_slice(&(wage as u16).to_le_bytes());
        Some(wage)
    }
}
/// Inicialización de convoy hanseático con barcos ya enlazados y estados 9/10.
/// Rechaza ciclos, huecos y divisiones imposibles antes de modificar los datos.
pub fn init_hanse_convoy(convoy: &mut [u8], ships: &mut [Option<Vec<u8>>], ship_count: u16, leader: u16) -> Option<()> {
    if convoy.len() < 0x3c || usize::from(ship_count) > ships.len() || leader >= ship_count {
        return None;
    }
    let mut cursor = leader;
    let mut seen = vec![false; usize::from(ship_count)];
    let mut chain = Vec::new();
    let mut speed = u32::MAX;
    while cursor < ship_count {
        let i = usize::from(cursor);
        if seen[i] {
            return None;
        }
        seen[i] = true;
        let ship = ships[i].as_ref()?;
        if ship.len() < 0x180 {
            return None;
        }
        let status = u16::from_le_bytes([ship[0x134], ship[0x135]]);
        if !matches!(status, 9 | 10) {
            return None;
        }
        speed = speed.min(super::motion::get_speed(ship).ok()?);
        chain.push(i);
        cursor = u16::from_le_bytes([ship[6], ship[7]])
    }
    let leader_ship = ships[usize::from(leader)].as_ref()?;
    let owner = leader_ship[0];
    let town = leader_ship[0x39];
    let from = leader_ship[0x37];
    let dest = leader_ship[0x38];
    let status = u16::from_le_bytes([leader_ship[0x134], leader_ship[0x135]]);
    let counter = u16::from_le_bytes([leader_ship[0x138], leader_ship[0x139]]);
    convoy[0] = owner;
    convoy[1] = 0;
    convoy[2..4].fill(255);
    convoy[4..8].fill(0);
    convoy[0xa..0xc].copy_from_slice(&leader.to_le_bytes());
    convoy[0xc..0x10].copy_from_slice(&speed.to_le_bytes());
    convoy[0x10..0x12].copy_from_slice(&leader.to_le_bytes());
    convoy[0x12..0x14].copy_from_slice(&status.to_le_bytes());
    convoy[0x14..0x16].copy_from_slice(&0x80u16.to_le_bytes());
    convoy[0x16..0x18].copy_from_slice(&counter.to_le_bytes());
    let flags = u32::from_le_bytes(convoy[0x18..0x1c].try_into().ok()?) | 0x4421;
    convoy[0x18..0x1c].copy_from_slice(&flags.to_le_bytes());
    convoy[0x1c..0x1e].copy_from_slice(&u16::from(town).to_le_bytes());
    convoy[0x1e] = from;
    convoy[0x1f] = dest;
    convoy[0x20..0x24].fill(0);
    convoy[0x24..0x3a].fill(255);
    let mut capacity = 0u32;
    for i in chain {
        let ship = ships[i].as_mut()?;
        convoy[1] |= ship[0xe];
        if ship[0] == owner {
            capacity = capacity.wrapping_add(u32::from_le_bytes(ship[0x10..0x14].try_into().ok()?))
        } else {
            convoy[0x14] &= 0xfd
        }
        if ship[0x39] != town {
            ship[0x3c] |= 8;
            ship[0x39] = town
        }
        ship[0x138..0x13a].copy_from_slice(&counter.to_le_bytes());
        ship[0x37] = from;
        ship[0x38] = dest;
        ship[0x134..0x136].copy_from_slice(&status.to_le_bytes());
    }
    convoy[4..8].copy_from_slice(&capacity.to_le_bytes());
    Some(())
}

/// Avanza el generador compartido de índices de nombre (`0x0050DF00`).
/// Rechaza el ciclo sin salida que el original ejecutaría indefinidamente.
pub fn next_ship_name_index(cursor: &mut u16, step: u16, count: usize) -> Option<u16> {
    let mut next = *cursor;
    let mut seen = [false; 307];
    loop {
        next = next.wrapping_add(step) % 307;
        let i = usize::from(next);
        if i < count {
            *cursor = next;
            return Some(next);
        }
        if seen[i] {
            return None;
        }
        seen[i] = true
    }
}

/// Estado naval mínimo para crear un convoy de la Liga. Los huecos libres
/// conservan sus enlaces; los vectores crecen como en el original.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HanseFactory {
    pub ships: Vec<Vec<u8>>,
    pub captains: Vec<[u8; 16]>,
    pub convoys: Vec<Vec<u8>>,
    pub free_ship: u16,
    pub free_captain: u16,
    pub free_convoy: u16,
    pub convoy_head: u16,
    pub name_cursor: u16,
    pub name_step: u16,
}
/// Recursos y calendario que no pueden deducirse del bloque de barcos.
pub struct HanseFactoryResources<'a> {
    pub ship_names: &'a [Vec<u8>],
    pub first_names: u16,
    pub last_names: u16,
    pub day_of_year: u16,
    pub day: u8,
    pub tick: u32,
    pub port_x: u16,
    pub port_y: u16,
}
/// `create_pirate_band_convoy` (0x0050ac40), con crecimiento de vectores.
/// La operación se prepara en copias: si falta una reserva o recurso no muta nada.
pub fn create_hanse_convoy(
    state: &mut HanseFactory,
    towns: &[Town],
    merchants: &mut [crate::ai::reputation::Merchant],
    city: u8,
    task_id: u16,
    two: bool,
    hunt: bool,
    resources: &HanseFactoryResources<'_>,
) -> Option<u16> {
    let town = towns.get(usize::from(city))?;
    let population = towns.get(usize::from(resources.day_of_year & 15))?.u32(0x2e0);
    if resources.ship_names.is_empty() || resources.first_names == 0 || resources.last_names == 0 {
        return None;
    }
    let owners = crate::economy::hanse_expedition::factory_owners(towns, merchants, two)?;
    if owners.iter().any(|&i| usize::from(i) >= merchants.len()) {
        return None;
    }
    let mut next = state.clone();
    let mut ms = merchants.to_vec();
    if usize::from(next.free_convoy) >= next.convoys.len() {
        if usize::from(next.free_convoy) != next.convoys.len() || next.convoys.len() + 32 > u16::MAX as usize {
            return None;
        }
        let start = next.convoys.len();
        for i in start..start + 32 {
            let mut c = vec![0u8; 0x3c];
            c[8..10].copy_from_slice(&((i + 1) as u16).to_le_bytes());
            c[0x12..0x14].fill(255);
            next.convoys.push(c)
        }
    }
    let cv = usize::from(next.free_convoy);
    let convoy = next.convoys.get_mut(cv)?;
    if convoy.len() < 0x3c || u16::from_le_bytes([convoy[0x12], convoy[0x13]]) < 0x17 {
        return None;
    }
    let id = next.free_convoy;
    next.free_convoy = u16::from_le_bytes([convoy[8], convoy[9]]);
    convoy[8..12].fill(255);
    for owner in owners {
        if usize::from(next.free_ship) >= next.ships.len() {
            if usize::from(next.free_ship) != next.ships.len() || next.ships.len() + 64 > u16::MAX as usize {
                return None;
            }
            let start = next.ships.len();
            for i in start..start + 64 {
                let mut ship = vec![0u8; 0x180];
                ship[4..6].fill(255);
                ship[6..8].copy_from_slice(&((i + 1) as u16).to_le_bytes());
                ship[8..10].fill(255);
                ship[0x134..0x136].copy_from_slice(&255u16.to_le_bytes());
                next.ships.push(ship)
            }
        }
        let index = usize::from(next.free_ship);
        let ship = next.ships.get_mut(index)?;
        if ship.len() < 0x180 || u16::from_le_bytes([ship[0x134], ship[0x135]]) < 0x17 {
            return None;
        }
        let ship_id = next.free_ship;
        next.free_ship = u16::from_le_bytes([ship[6], ship[7]]);
        reset_ship(ship, resources.tick)?;
        ship[0x137] = 0;
        ship[0x134..0x136].copy_from_slice(&0x12u16.to_le_bytes());
        ship[0x4c..0x50].copy_from_slice(
            &resources
                .tick
                .wrapping_sub((resources.tick & 255) * 0x400)
                .to_le_bytes(),
        );
        init_ship_at_town(ship, city, resources.port_x, resources.port_y, resources.tick)?;
        ship[0x13b] = city;
        let (kind, bonus) = if town.flags() & 1 != 0 { (1, 5u32) } else { (2, 8u32) };
        init_ship_stats(town, ship, kind)?;
        ship[0xf] = 2;
        let hull = u32::from_le_bytes(ship[0x14..0x18].try_into().ok()?).wrapping_add(bonus * 0xaf0);
        ship[0x14..0x18].copy_from_slice(&hull.to_le_bytes());
        ship[0x18..0x1c].copy_from_slice(&hull.to_le_bytes());
        let capacity = u32::from_le_bytes(ship[0x10..0x14].try_into().ok()?).wrapping_sub(bonus * 2000);
        ship[0x10..0x14].copy_from_slice(&capacity.to_le_bytes());
        let m = &mut ms[usize::from(owner)];
        m.add_social_reputation_ship_hull_work(u32::from(city), hull);
        ship[0] = owner;
        ship[4..6].copy_from_slice(&m.u16(0xe).to_le_bytes());
        m.set_u16(0xe, ship_id);
        let name_index = next_ship_name_index(&mut next.name_cursor, next.name_step, resources.ship_names.len())?;
        let name = &resources.ship_names[usize::from(name_index)];
        ship[0x15e..0x160].copy_from_slice(&next.name_cursor.to_le_bytes());
        let n = name.iter().position(|&b| b == 0).unwrap_or(name.len()).min(31);
        ship[0x160..0x160 + n].copy_from_slice(&name[..n]);
        ship[0x160 + n] = 0;
        ship[0x3c] |= 1;
        recompute_weapon_slots(ship)?;
        let sailors = additional_sailors(ship)? as u16;
        ship[0x40..0x42].copy_from_slice(&sailors.to_le_bytes());
        ship[0x154..0x156].copy_from_slice(&sailors.to_le_bytes());
        if usize::from(next.free_captain) >= next.captains.len() {
            if usize::from(next.free_captain) != next.captains.len() || next.captains.len() + 64 > u16::MAX as usize {
                return None;
            }
            let start = next.captains.len();
            for i in start..start + 64 {
                let mut cap = [255u8; 16];
                cap[..2].copy_from_slice(&((i + 1) as u16).to_le_bytes());
                next.captains.push(cap)
            }
        }
        let cap = next.captains.get_mut(usize::from(next.free_captain))?;
        let cap_id = next.free_captain;
        next.free_captain = u16::from_le_bytes([cap[0], cap[1]]);
        cap[0..2].fill(255);
        init_captain(
            cap,
            u32::from(cap_id),
            true,
            resources.tick,
            resources.day_of_year,
            resources.day,
            population,
            resources.first_names,
            resources.last_names,
        )?;
        let wage = (u16::from(cap[9]) + u16::from(cap[10]) + u16::from(cap[11])) / 50 + 10 + u16::from(cap[8] % 11);
        cap[12..14].copy_from_slice(&wage.to_le_bytes());
        ship[0x42..0x44].copy_from_slice(&cap_id.to_le_bytes());
        for pair in ship[0x13c..0x154].chunks_exact_mut(2) {
            if pair[0] > 7 {
                if pair[1] < 8 {
                    pair[0] = 0
                } else {
                    pair[0] = 3;
                    pair[1] = 6
                }
            }
        }
        super::cargo::recompute_load(ship);
        ship[8..10].copy_from_slice(&id.to_le_bytes());
        ship[6..8].copy_from_slice(&convoy[10..12]);
        ship[0x134..0x136].copy_from_slice(&(if hunt { 10u16 } else { 9 }).to_le_bytes());
        convoy[10..12].copy_from_slice(&ship_id.to_le_bytes());
    }
    let leader = u16::from_le_bytes([convoy[10], convoy[11]]);
    let mut images = next.ships.iter().cloned().map(Some).collect::<Vec<_>>();
    init_hanse_convoy(convoy, &mut images, u16::try_from(next.ships.len()).ok()?, leader)?;
    for (dest, src) in next.ships.iter_mut().zip(images) {
        *dest = src?
    }
    convoy[8..10].copy_from_slice(&next.convoy_head.to_le_bytes());
    next.convoy_head = id;
    convoy[0x12..0x14].copy_from_slice(&(if hunt { 10u16 } else { 9 }).to_le_bytes());
    convoy[0x16..0x18].fill(0);
    let mut cursor = leader;
    let status = if hunt { 10u16 } else { 9 };
    while usize::from(cursor) < next.ships.len() {
        let ship = &mut next.ships[usize::from(cursor)];
        ship[0x3c] |= 0x20;
        ship[0x134..0x136].copy_from_slice(&status.to_le_bytes());
        ship[0x138..0x13a].fill(0);
        cursor = u16::from_le_bytes([ship[6], ship[7]])
    }
    convoy[2..4].copy_from_slice(&task_id.to_le_bytes());
    if hunt {
        convoy[0x16..0x18].copy_from_slice(&0x900u16.to_le_bytes())
    }
    *state = next;
    merchants.clone_from_slice(&ms);
    Some(id)
}
