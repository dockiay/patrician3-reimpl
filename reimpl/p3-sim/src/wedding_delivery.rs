//! Tarea 0x15 (`0x004E7C74`) y fábrica de entrega sin armamento mínimo
//! (`0x0050E000`, único modo pedido por la boda). Cartas y UI son efectos
//! explícitos que el integrador debe publicar con las imágenes navales.
//!
//! Incluye los datos familiares, selección de uno/dos regalos según tonelaje,
//! fábrica naval, cadenas de propietario y puerto, carga, nombres y conversión
//! a tarea 0x07. No implementa el modo de fábrica con armamento mínimo distinto
//! de cero. El crecimiento inicializa a cero bytes libres no serializados.
//! `notice` contiene los argumentos efectivos (u16/u8) de 0x548CA0, sin las
//! mitades altas residuales que el llamante original deja en registros.
use p3_core::{
    ai::reputation::Merchant,
    scheduler::TaskRecord,
    ships::{
        cargo,
        factory::{self, HanseFactory},
    },
    town::Town,
};

pub struct WeddingResources<'a> {
    pub tick: u32,
    pub operations_mode: u32,
    pub ship_names: &'a [Vec<u8>],
    pub ports: &'a [(u16, u16)],
    pub local_player: u32,
    pub local_controls_home: bool,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WeddingEffects {
    pub letters: Vec<(i32, [u8; 16])>,
    pub notice: [u32; 6],
    pub show_window: bool,
    pub delivered: Vec<u16>,
}
fn r16(b: &[u8], p: usize) -> u16 {
    u16::from_le_bytes([b[p], b[p + 1]])
}
fn r32(b: &[u8], p: usize) -> u32 {
    u32::from_le_bytes(b[p..p + 4].try_into().unwrap())
}
fn w16(b: &mut [u8], p: usize, v: u16) {
    b[p..p + 2].copy_from_slice(&v.to_le_bytes())
}
fn w32(b: &mut [u8], p: usize, v: u32) {
    b[p..p + 4].copy_from_slice(&v.to_le_bytes())
}
const CREW: [u16; 4] = [5, 8, 10, 12];
const UNITS: [u8; 24] = [0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1];

fn create_docked(
    factory: &mut HanseFactory,
    docked: &mut u16,
    kind: &mut u16,
    city: u8,
    owner: u8,
    towns: &[Town],
    merchants: &mut [Merchant],
    r: &WeddingResources<'_>,
) -> Result<u16, String> {
    let town = towns.get(usize::from(city)).ok_or("ciudad de boda inexistente")?;
    let &(x, y) = r.ports.get(usize::from(city)).ok_or("puerto de boda ausente")?;
    let merchant = merchants
        .get_mut(usize::from(owner))
        .ok_or("comerciante de boda inexistente")?;
    if town.flags() & 1 != 0 && *kind & 2 != 0 {
        *kind = 1
    }
    if *kind >= 4 {
        return Err("tipo de regalo naval inválido".into());
    }
    if usize::from(factory.free_ship) >= factory.ships.len() {
        let count = factory.ships.len();
        if count + 64 > usize::from(u16::MAX) {
            return Err("pool naval agotado".into());
        }
        factory.free_ship = count as u16;
        for i in count..count + 64 {
            let mut ship = vec![0; 0x180];
            w16(&mut ship, 4, u16::MAX);
            w16(&mut ship, 6, (i + 1) as u16);
            w16(&mut ship, 8, u16::MAX);
            w16(&mut ship, 0x134, 0xff);
            factory.ships.push(ship)
        }
    }
    let id = factory.free_ship;
    let ship = factory.ships.get_mut(usize::from(id)).ok_or("ranura naval ausente")?;
    if ship.len() != 0x180 || r16(ship, 0x134) < 0x17 {
        return Err("ranura naval ocupada".into());
    }
    factory.free_ship = r16(ship, 6);
    factory::reset_ship(ship, r.tick).ok_or("reinicio naval inválido")?;
    ship[0x137] = 0;
    w16(ship, 0x134, 0);
    w16(ship, 6, *docked);
    *docked = id;
    factory::init_ship_at_town(ship, city, x, y, r.tick).ok_or("puerto inválido")?;
    ship[0x13b] = city;
    factory::init_ship_stats(town, ship, *kind as u8).ok_or("estadísticas navales inválidas")?;
    let hull = r32(ship, 0x14);
    w32(ship, 0x18, hull);
    ship[0] = owner;
    ship[0x3d] |= 4;
    w16(ship, 4, merchant.u16(0xe));
    merchant.set_u16(0xe, id);
    ship[0x13a] = if merchant.u16(8) == 0 {
        merchant.bytes[0x19]
    } else {
        city
    };
    w16(ship, 0x134, 3);
    if r.operations_mode & 2 != 0 {
        w16(ship, 0x138, 0x15)
    }
    let crew = CREW[usize::from(ship[0xe] & 3)];
    w16(ship, 0x40, crew);
    cargo::recompute_load(ship);
    factory::recompute_weapon_slots(ship).ok_or("ranuras de arma inválidas")?;
    let maintenance = (u16::from(r.tick as u8)).wrapping_add(id).wrapping_add(4000);
    w16(ship, 0x3e, if maintenance as i16 > 0x700 { 0x700 } else { maintenance });
    let name = factory::next_ship_name_index(&mut factory.name_cursor, factory.name_step, r.ship_names.len())
        .ok_or("banco de nombres navales no accesible")?;
    w16(ship, 0x15e, name);
    let bytes = &r.ship_names[usize::from(name)];
    let size = bytes.iter().position(|&v| v == 0).unwrap_or(bytes.len()).min(31);
    ship[0x160..0x160 + size].copy_from_slice(&bytes[..size]);
    ship[0x160 + size] = 0;
    ship[0x3c] |= 1;
    let age = (u32::from(id & 3) + 2) * 365 + (town.u32(0x31c) & 255) + u32::from(city) + u32::from(*kind);
    w32(ship, 0x4c, r.tick.wrapping_sub(age << 8));
    merchant.set_u16(0x478, merchant.u16(0x478).wrapping_add(1));
    Ok(id)
}

fn amount(ship: &mut [u8], ware: usize, seed: u32, shift: u32) -> u16 {
    let scaled = ((seed & 15) + 15) * 170;
    let mut quantity = p3_core::numeric::ftol_i32(
        f64::from(scaled as f32) / f64::from(f32::from_bits(p3_core::ships::convoy_absorb::CARGO_SCALE_BITS[ware])),
    );
    quantity = if quantity < 100 {
        0
    } else {
        quantity.min((r32(ship, 0x10) as i32) >> shift)
    };
    let (stock, display) = if UNITS[ware] != 0 {
        let n = quantity / 200;
        (n.wrapping_mul(200), n as u16)
    } else {
        let n = quantity / 2000;
        (n.wrapping_mul(2000), (n as u16).wrapping_mul(10))
    };
    w32(ship, 0x54 + ware * 4, stock as u32);
    display
}

/// Prepara estado familiar, barcos y cambio de tarea 0x15→0x07 atómicamente.
/// `None` es la salida sin boda; los efectos devueltos siguen pendientes de
/// publicar. No presume que se haya entregado ninguna carta o aviso.
pub fn deliver(
    task: &mut TaskRecord,
    factory: &mut HanseFactory,
    docked_head: &mut u16,
    merchants: &mut [Merchant],
    towns: &[Town],
    resources: &WeddingResources<'_>,
) -> Result<Option<WeddingEffects>, String> {
    if task.kind != 0x15 {
        return Err("tarea distinta de boda".into());
    }
    let owner = usize::from(task.u8_at(0x15));
    let Some(person) = merchants.get(owner) else {
        return Ok(None);
    };
    if person.bytes.len() != 0x650 {
        return Err("comerciante de boda truncado".into());
    }
    if person.i32(0x28) as u32 <= resources.tick {
        return Ok(None);
    }
    if factory.ships.iter().any(|s| s.len() != 0x180) || factory.ships.len() > usize::from(u16::MAX) {
        return Err("flota de boda inválida".into());
    }
    let mut next = factory.clone();
    let mut people = merchants.to_vec();
    let mut record = *task;
    let mut docked = *docked_head;
    let person = &mut people[owner];
    let home = person.bytes[0x19];
    let age = u16::from_le_bytes([task.u8_at(0xc), task.u8_at(0xd)]);
    person.set_i32(0x28, resources.tick.wrapping_sub(u32::from(age) << 8) as i32);
    person.set_i32(0x2c, resources.tick as i32);
    for (to, from) in [(0x27, 0xf), (0x30, 0x10), (0x31, 0x12), (0x32, 0x13), (0x33, 0x14)] {
        person.bytes[to] = task.u8_at(from)
    }
    let mut letter = [0u8; 16];
    letter[1] = 0x80;
    letter[2..4].fill(255);
    letter[4] = 0x23;
    letter[5] = home;
    w16(&mut letter, 8, owner as u16);
    w16(
        &mut letter,
        10,
        u16::from(task.u8_at(0x10)) * 256 + u16::from(task.u8_at(0x11)),
    );
    w16(&mut letter, 12, u16::from(task.u8_at(0x12)));
    let spouse_age = if person.bytes[0x1a] != 0 {
        (age / 365) as u8
    } else {
        (resources.tick.wrapping_sub(person.i32(0x10) as u32) / 0x16d00) as u8
    };
    w16(&mut letter, 14, u16::from(spouse_age));
    let mut letters = vec![(-2, letter)];
    let mut total = 0i32;
    let mut cursor = usize::from(person.u16(0xe));
    let mut seen = vec![false; next.ships.len()];
    while cursor < next.ships.len() {
        if seen[cursor] {
            return Err("lista naval del cónyuge cíclica".into());
        }
        seen[cursor] = true;
        let ship = &next.ships[cursor];
        total = total.wrapping_add((r32(ship, 0x10) as i32) / 40000);
        cursor = usize::from(r16(ship, 4))
    }
    let score = total.wrapping_sub(i32::from(task.u8_at(0x13))).max(0);
    let mut types = match score {
        0..=5 => [0, 255],
        6 => [1, 255],
        7 | 9 => [2, 255],
        8 => [1, 0],
        10 => [2, 0],
        _ => [2, 1],
    };
    letter[4] = 0x25;
    letter[5] = task.u8_at(0x12);
    let mut quantities = [0u16; 2];
    let mut delivered = Vec::new();
    for j in 0..2 {
        if types[j] > 3 {
            continue;
        }
        let id = create_docked(
            &mut next,
            &mut docked,
            &mut types[j],
            task.u8_at(0x12),
            owner as u8,
            towns,
            &mut people,
            resources,
        )?;
        delivered.push(id);
        let ship = &mut next.ships[usize::from(id)];
        let health = (r32(ship, 0x14) as i32) >> 5;
        let factor = (i32::from(task.u8_at(0x13)) - i32::from(task.u8_at(0x14)))
            .wrapping_mul(3)
            .wrapping_add(23);
        w32(ship, 0x18, health.wrapping_mul(factor) as u32);
        let first = usize::from(task.u8_at(0xe) & 15);
        let free = cargo::recompute_load(ship);
        w32(ship, 0x54 + first * 4, free as u32);
        quantities[j] = if free <= 0 {
            w32(ship, 0x54 + first * 4, 0);
            0
        } else {
            amount(ship, first, u32::from(id), 1)
        };
        let second = usize::from(task.u8_at(0xe) >> 4);
        let second = if second == first {
            if first == 12 {
                3
            } else {
                12
            }
        } else {
            second
        };
        quantities[j] = quantities[j].wrapping_add(amount(ship, second, resources.tick, 2));
        let crew = CREW[usize::from(ship[0xe] & 3)];
        w16(ship, 0x40, crew);
        cargo::recompute_load(ship);
    }
    for j in 0..2 {
        w16(&mut letter, 8 + 2 * j, types[j]);
        if types[j] <= 3 {
            w16(&mut letter, 12 + 2 * j, quantities[j])
        }
    }
    letters.push((owner as i32, letter));
    let spouse = u32::from(task.u8_at(0x10)) * 256 + u32::from(task.u8_at(0x11));
    let notice = [owner as u32, 0, 0, u32::from(home), 0x17, spouse];
    let show_window = resources.local_player == owner as u32 || resources.local_controls_home;
    record.kind = 7;
    for (at, value) in [(8, owner as u32), (12, u32::from(home)), (16, 0)] {
        for (k, v) in value.to_le_bytes().iter().enumerate() {
            record.set_u8_at(at + k, *v)
        }
    }
    *factory = next;
    *docked_head = docked;
    merchants.clone_from_slice(&people);
    *task = record;
    Ok(Some(WeddingEffects {
        letters,
        notice,
        show_window,
        delivered,
    }))
}
