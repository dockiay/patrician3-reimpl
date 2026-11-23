//! Órdenes de jugador sobre barcos, sobre la imagen de memoria del original: contratar (`0x00537C20`) y despedir marineros (`0x00537DD0`),
//! despedir al capitán (`0x00538000`), fijar el objetivo de un barco (`0x00538800`) y pasar marineros (`0x00538930`) o sables (`0x005389C0`) entre
//! dos barcos. Las llamadas a métodos de barco, comerciante y ciudad van por [`OpsHost`] con la dirección del original; los recálculos que el
//! original encadena con `jmp` (`0x005182B0`) son llamadas finales.
use crate::economy::ops_misc::OpsHost;
use crate::flatmem::Mem;

const WORLD: u32 = 0x006d_e4a0;
const G_MERCHANTS: u32 = 0x006d_e4aa;
const G_TOWNS: u32 = 0x006d_e4b0;
const G_TOWN_TABLE: u32 = 0x006d_e508;
const LETTER_COUNT: u32 = 0x006d_d892;
const SHIP_COUNT: u32 = 0x006d_d894;
const SHIP_TABLE: u32 = 0x006d_d7a4;
const CONVOY_COUNT: u32 = 0x006d_d896;
const CONVOY_TABLE: u32 = 0x006d_d7a8;
const CONVOY_LIST: u32 = 0x006d_d7a0;
const NEAR_LIMIT: u32 = 0x006d_d89c;

fn ship_at(m: &mut dyn Mem, index: u32) -> u32 {
    m.r32(SHIP_TABLE).wrapping_add(index.wrapping_mul(0x180))
}

fn town_at(m: &mut dyn Mem, index: u32) -> u32 {
    m.r32(G_TOWN_TABLE).wrapping_add(index.wrapping_mul(0x9f8))
}

/// `0x00537C20`: contrata `op+8` marineros (limitados por la capacidad y la plantilla disponible del comerciante) para el barco `op+4`.
pub fn hire_sailors(m: &mut dyn Mem, host: &mut dyn OpsHost, op: u32) {
    let index = m.r32(op + 4);
    if index >= u32::from(m.r16(SHIP_COUNT)) {
        return;
    }
    let ship = ship_at(m, index);
    let town = u32::from(m.r8(ship + 0x39));
    if town >= m.r32(G_TOWNS) {
        return;
    }
    let room = host.call(m, 0x5184f0, ship, &[]);
    if room < m.r32(op + 8) {
        m.w32(op + 8, room);
    }
    let owner = m.r8(ship);
    if u16::from(owner) < m.r16(G_MERCHANTS) {
        let merchant = host.call(m, 0x5303c0, WORLD, &[u32::from(owner)]);
        let limit = host.call(m, 0x4f6ca0, merchant, &[town]) & 0xff;
        if m.r32(op + 8) > limit {
            m.w32(op + 8, limit);
        }
        let amount = m.r32(op + 8);
        host.call(m, 0x4f6bd0, merchant, &[town, amount]);
    }
    let crew = m.r16(ship + 0x40);
    let amount16 = m.r16(op + 8);
    if crew == 0 {
        let letters = m.r16(LETTER_COUNT);
        let captain = m.r16(ship + 0x42);
        m.w8(ship + 0x3f, if captain < letters { 4 } else { 3 });
        let flags = m.r8(ship + 0x3d);
        m.w8(ship + 0x3d, flags | 2);
        m.w16(ship + 0x40, amount16);
    } else {
        m.w16(ship + 0x40, amount16.wrapping_add(crew));
    }
    host.call(m, 0x517770, ship, &[2]);
    let convoy = m.r16(ship + 8);
    if convoy < m.r16(CONVOY_COUNT) {
        let at = m.r32(CONVOY_TABLE).wrapping_add(u32::from(convoy) * 60 + 0x18);
        let v = m.r32(at);
        m.w32(at, v | 2);
    }
    let town_base = town_at(m, town);
    let pool = m.r32(town_base + 0x2d4);
    let free = m.r32(town_base + 0x2e4);
    let amount = m.r32(op + 8);
    if free > amount {
        m.w32(town_base + 0x2d4, pool.wrapping_sub(amount));
        m.w32(town_base + 0x2e4, free.wrapping_sub(amount));
    } else {
        m.w32(town_base + 0x2d4, pool.wrapping_sub(free));
        m.w32(town_base + 0x2e4, 0);
    }
    host.call(m, 0x5182b0, ship, &[]);
}

/// `0x00537DD0`: despide `op+8` marineros del barco `op+4` (todos si no hay más), los devuelve a la ciudad y reduce la moral.
pub fn dismiss_sailors(m: &mut dyn Mem, host: &mut dyn OpsHost, op: u32) {
    let index = m.r32(op + 4);
    if index >= u32::from(m.r16(SHIP_COUNT)) {
        return;
    }
    let ship = ship_at(m, index);
    let crew = m.r16(ship + 0x40);
    if crew == 0 || m.r16(ship + 0x134) >= 4 {
        return;
    }
    let town = u32::from(m.r8(ship + 0x39));
    if town >= m.r32(G_TOWNS) {
        return;
    }
    let town_base = town_at(m, town);
    let amount = m.r32(op + 8);
    if u32::from(crew) < amount {
        let free = m.r32(town_base + 0x2e4).wrapping_add(u32::from(crew));
        let pool = m.r32(town_base + 0x2d4);
        m.w32(town_base + 0x2e4, free);
        let crew_now = u32::from(m.r16(ship + 0x40));
        m.w32(town_base + 0x2d4, pool.wrapping_add(crew_now));
        m.w16(ship + 0x40, 0);
        m.w16(ship + 0x3e, 0);
    } else {
        let free = m.r32(town_base + 0x2e4).wrapping_add(amount);
        let pool = m.r32(town_base + 0x2d4);
        m.w32(town_base + 0x2e4, free);
        let amount2 = m.r32(op + 8);
        m.w32(town_base + 0x2d4, pool.wrapping_add(amount2));
        let crew = m.r16(ship + 0x40);
        let divisor = u32::from(crew) + 1;
        let scaled = amount2.wrapping_mul(5) << 9;
        let by = m.r16(op + 8);
        m.w16(ship + 0x40, crew.wrapping_sub(by));
        let quotient = scaled / divisor;
        let morale = m.r16(ship + 0x3e).wrapping_sub(quotient as u16);
        m.w16(ship + 0x3e, if (morale as i16) < 0 { 0 } else { morale });
    }
    host.call(m, 0x5182b0, ship, &[]);
    host.call(m, 0x517770, ship, &[0x202]);
}

/// `0x00538000`: despide al capitán del barco `op+4` (libera la carta del capitán y baja la moral).
pub fn dismiss_captain(m: &mut dyn Mem, host: &mut dyn OpsHost, op: u32) {
    let index = m.r32(op + 4);
    if index >= u32::from(m.r16(SHIP_COUNT)) {
        return;
    }
    let ship = ship_at(m, index);
    if m.r16(ship + 0x42) >= m.r16(LETTER_COUNT) || m.r16(ship + 0x134) >= 4 {
        return;
    }
    let town = u32::from(m.r8(ship + 0x39));
    if town >= m.r32(G_TOWNS) {
        return;
    }
    let owner = m.r8(ship);
    if u16::from(owner) < m.r16(G_MERCHANTS) {
        let merchant = host.call(m, 0x5303c0, WORLD, &[u32::from(owner)]);
        m.w8(merchant + 0x1f, 0);
    }
    let town_base = town_at(m, town);
    host.call(m, 0x526b00, town_base, &[ship + 0x42]);
    let divisor = i32::from(m.r16(ship + 0x40)) + 1;
    let delta = (-2560i32 / divisor) as u16;
    let morale = m.r16(ship + 0x3e).wrapping_add(delta);
    m.w16(ship + 0x3e, morale);
    if (m.r8(ship + 0x3f) as i8) < 0 {
        m.w16(ship + 0x3e, 0);
    }
    let convoy = m.r16(ship + 8);
    let flags = m.r16(ship + 0x3c);
    m.w16(ship + 0x3c, flags | 0x202);
    if convoy >= m.r16(CONVOY_COUNT) {
        return;
    }
    let record = m.r32(CONVOY_TABLE).wrapping_add(u32::from(convoy) * 60);
    if u32::from(m.r16(record + 0x10)) != m.r32(op + 4) {
        return;
    }
    host.call(m, 0x50a580, CONVOY_LIST, &[u32::from(convoy)]);
}

/// `0x00538800`: el barco `op+4` toma como objetivo al barco `op+8` si los tipos son compatibles y está lo bastante cerca.
pub fn set_ship_target(m: &mut dyn Mem, op: u32) {
    let count = u32::from(m.r16(SHIP_COUNT));
    let a = m.r32(op + 4);
    let b = m.r32(op + 8);
    if a >= count || b >= count {
        return;
    }
    let (sa, sb) = (ship_at(m, a), ship_at(m, b));
    let ta = m.r16(sa + 0x134);
    if ta != 0xf && ta != 0x10 {
        return;
    }
    let flag = m.r8(sa + 0x15c);
    let tb = m.r16(sb + 0x134);
    let compatible = if flag != 0 {
        tb == 0xf || tb == 0x10 || tb == 0x12
    } else {
        tb == 0x12 || (tb == 0xf && m.r8(sb + 0x15c) != 0)
    };
    if !compatible {
        return;
    }
    let dx = i32::from(m.r16(sb + 0x1e) as i16) - i32::from(m.r16(sa + 0x1e) as i16);
    let dy = i32::from(m.r16(sb + 0x22) as i16) - i32::from(m.r16(sa + 0x22) as i16);
    let distance = dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx));
    if distance > i32::from(m.r16(NEAR_LIMIT)) {
        return;
    }
    let target = m.r16(op + 8);
    m.w16(sa + 0x3a, target);
}

fn two_ships(m: &mut dyn Mem, op: u32) -> Option<(u32, u32)> {
    let count = u32::from(m.r16(SHIP_COUNT));
    let (a, b) = (m.r32(op + 4), m.r32(op + 8));
    if a >= count || b >= count {
        return None;
    }
    Some((ship_at(m, a), ship_at(m, b)))
}

/// `0x00538930`: pasa `op+0xC` marineros del barco `op+4` al `op+8`, sin pasar de los que hay ni de la capacidad libre del destino.
pub fn transfer_sailors(m: &mut dyn Mem, host: &mut dyn OpsHost, op: u32) {
    let Some((a, b)) = two_ships(m, op) else { return };
    let have = u32::from(m.r16(a + 0x40));
    if have < m.r32(op + 0xc) {
        m.w32(op + 0xc, have);
    }
    let room = host.call(m, 0x5184f0, b, &[]);
    if m.r32(op + 0xc) > room {
        m.w32(op + 0xc, room);
    }
    let n = m.r16(op + 0xc);
    let v = m.r16(a + 0x40).wrapping_sub(n);
    m.w16(a + 0x40, v);
    let v = m.r16(b + 0x40).wrapping_add(n);
    m.w16(b + 0x40, v);
    host.call(m, 0x517770, a, &[2]);
    host.call(m, 0x517770, b, &[2]);
    host.call(m, 0x5182b0, a, &[]);
    host.call(m, 0x5182b0, b, &[]);
}

/// `0x005389C0`: pasa `op+0xC` sables del barco `op+4` al `op+8`; el destino no acepta más de un décimo de su capacidad de carga.
pub fn transfer_cutlasses(m: &mut dyn Mem, host: &mut dyn OpsHost, op: u32) {
    let Some((a, b)) = two_ships(m, op) else { return };
    let have = u32::from(m.r16(a + 0x154));
    if have < m.r32(op + 0xc) {
        m.w32(op + 0xc, have);
    }
    let cap = host.call(m, 0x5182b0, b, &[]);
    let want = m.r32(op + 0xc).wrapping_mul(5).wrapping_mul(2);
    if cap < want {
        let hi = (i64::from(0x6666_6667i32) * i64::from(cap as i32)) >> 32;
        let mut q = (hi as i32) >> 2;
        q = q.wrapping_add(((q as u32) >> 31) as i32);
        m.w32(op + 0xc, q as u32);
    }
    let n = m.r16(op + 0xc);
    let v = m.r16(a + 0x154).wrapping_sub(n);
    m.w16(a + 0x154, v);
    let v = m.r16(b + 0x154).wrapping_add(n);
    m.w16(b + 0x154, v);
    host.call(m, 0x517770, a, &[0x14]);
    host.call(m, 0x517770, b, &[0x14]);
    host.call(m, 0x5182b0, a, &[]);
    host.call(m, 0x5182b0, b, &[]);
}
