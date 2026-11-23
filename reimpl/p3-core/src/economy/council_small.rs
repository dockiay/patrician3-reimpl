//! Funciones pequeñas del consejo, el alcalde, la taberna y los préstamos (C08), sobre la imagen de memoria del original.
//! Los globales (`0x006DE4AA` comerciantes, `0x006DE4B0` ciudades, `0x006DE4A8` recuento de sitios, `0x006DFC14` jugador local, el objeto
//! `0x006DE4A0` con sus punteros) se leen de la propia imagen. `town` es la dirección de una `Town` (`0x9F8` bytes).
use crate::ai::{guild::GuildMerchant, reputation::Merchant};
use crate::economy::sched_queries::{find_mayor_election, merchant_addr, G_MERCHANTS, G_PLAYER};
use crate::flatmem::Mem;

const WORLD: u32 = 0x006d_e4a0;
const G_TOWNS: u32 = 0x006d_e4b0;
const G_SITES: u32 = 0x006d_e4a8;
const TOWN_TABLE: u32 = 0x006d_e508;
const SCHED: u32 = 0x006d_d73c;
const MAYOR_TABLE: u32 = 0x0067_37b4;

fn mc(m: &mut dyn Mem) -> u32 {
    u32::from(m.r16(G_MERCHANTS))
}

/// `get_merchant_epithet_index` (`0x004F9430`) sobre el comerciante en `mer` (solo lee `+0x15`, `+0x1B`, `+0x46C`).
fn epithet_of(m: &mut dyn Mem, mer: u32) -> u8 {
    let mut b = Merchant { bytes: vec![0u8; 0x650] };
    b.bytes[0x15] = m.r8(mer + 0x15);
    b.bytes[0x1b] = m.r8(mer + 0x1b);
    b.set_i32(0x46c, m.r32(mer + 0x46c) as i32);
    b.get_merchant_epithet_index()
}

/// `0x005286E0`: vacía la economía y el consejo de la ciudad (`0x00527890` + valores iniciales del consejo). Devuelve `town`.
pub fn reset_economy_and_council(m: &mut dyn Mem, town: u32) -> u32 {
    for k in 0..0x2c0 {
        m.w8(town + k, 0);
    }
    m.w32(town + 0x2c8, 0);
    m.w32(town + 0x2d0, 0);
    for k in 0..0x18u32 {
        m.w32(town + 4 + 4 * k, 0);
        m.w32(town + 0x370 + 4 * k, 0);
        m.w32(town + 0x3d0 + 4 * k, 0);
        m.w32(town + 0x430 + 4 * k, 0);
        m.w32(town + 0x490 + 4 * k, 0);
        for j in 0..4 {
            m.w32(town + 0x4f0 + 16 * k + 4 * j, 0);
        }
        m.w32(town + 0x670 + 4 * k, 0);
    }
    m.w32(town + 0x6d0, 0);
    for (o, v) in [(0x6e4u32, 5u8), (0x6e5, 5), (0x6f0, 0xff), (0x6f1, 0xff), (0x6f2, 0xff), (0x6f3, 0)] {
        m.w8(town + o, v);
    }
    m.w32(town + 0x6fc, 0);
    for o in 0x6d4..0x6dc {
        m.w8(town + o, 0);
    }
    m.w8(town + 0x6e6, 4);
    m.w8(town + 0x6e7, 4);
    if mc(m) < 0x100 {
        if m.r8(0x006d_e52c + 2) == m.r8(town + 0x2c0) && m.r8(0x006d_e52c + 1) == 0 {
            m.w8(town + 0x6e4, 7);
            return town;
        }
        m.w8(town + 0x6e4, 6);
    }
    town
}

/// `town_get_next_mayor_election_date` (`0x005295F0`): día, mes y año de la próxima elección de alcalde de la ciudad (tarea tipo 2).
pub fn next_mayor_election_date(m: &mut dyn Mem, town: u32) -> (u8, u8, u16) {
    let owner = u32::from(m.r8(town + 0x2c0));
    let task = find_mayor_election(m, SCHED, owner);
    let year = m.r16(WORLD + 2);
    if task == 0 {
        return (0x1f, 0x0c, year);
    }
    let (day, month) = (m.r8(task + 0xf), m.r8(task + 0x10));
    let year = if m.r16(task + 8) < m.r16(WORLD + 4) { year.wrapping_add(1) } else { year };
    (day, month, year)
}

/// `get_alderman_epithet_index` (`0x00529DC0`): título del comerciante que presenta la elección (`0x6DE52D`) o, si no es un comerciante,
/// byte de la ciudad en `0x6DE52E`.
pub fn alderman_epithet_index(m: &mut dyn Mem) -> u8 {
    let idx = u32::from(m.r8(0x006d_e52d));
    if idx < mc(m) {
        let mer = merchant_addr(m, idx);
        return epithet_of(m, mer);
    }
    let base = m.r32(TOWN_TABLE);
    let town = base.wrapping_add(u32::from(m.r8(0x006d_e52e)).wrapping_mul(0x9f8));
    m.r8(town.wrapping_add(!idx & 3).wrapping_add(0x6e0))
}

/// `town_get_mayor_reputation` (`0x00529660`): reputación entera del alcalde en su ciudad, o el byte de la tabla `0x6737B4`.
pub fn mayor_reputation(m: &mut dyn Mem, town: u32) -> u32 {
    let b = u32::from(m.r8(town + 0x6f1));
    if b < mc(m) {
        let owner = u32::from(m.r8(town + 0x2c0));
        let mer = merchant_addr(m, b);
        let town_count = m.r32(G_TOWNS);
        let mut g = GuildMerchant { gold: 0, flags: 0, office_head: 0, home: 0, guild_mask: 0, income: 0, reputation: Vec::new() };
        for i in 0..town_count.min(16) {
            g.reputation.push(f32::from_bits(m.r32(mer + 0x2fc + 4 * i)));
        }
        return i32::from(g.reputation_int(town_count, owner) as i16) as u32;
    }
    u32::from(m.r8(MAYOR_TABLE + (!b & 3)))
}

/// `town_get_mayor_epithet_index` (`0x005296B0`): título del alcalde o el byte `+0x6E0..` de la ciudad.
pub fn mayor_epithet_index(m: &mut dyn Mem, town: u32) -> u8 {
    let b = u32::from(m.r8(town + 0x6f1));
    if b < mc(m) {
        let mer = merchant_addr(m, b);
        return epithet_of(m, mer);
    }
    m.r8(town + 0x6e0 + (!b & 3))
}

/// `op_set_town_mayor_from_merchant` (`0x0053AEC0`): `op+4` comerciante, `op+8` ciudad, `op+0x10` clave de forzado `0xBADEAFFE`.
pub fn op_set_town_mayor_from_merchant(m: &mut dyn Mem, op: u32) {
    let merchant = m.r32(op + 4);
    let town_index = m.r32(op + 8);
    if merchant < mc(m) && town_index < m.r32(G_TOWNS) {
        let base = m.r32(TOWN_TABLE);
        let town = base.wrapping_add(town_index.wrapping_mul(0x9f8));
        if mc(m) <= u32::from(m.r8(town + 0x6f1)) {
            let mer = merchant_addr(m, merchant);
            let home = u32::from(m.r8(mer + 0x19));
            if m.r8(mer + 0x39c + home) > 4 || m.r32(op + 0x10) == 0xbade_affe {
                let v = m.r8(op + 4);
                m.w8(town + 0x6f1, v);
            }
        }
    }
}

/// `town_get_council_block_serialized_size` (`0x00528790`).
pub const COUNCIL_BLOCK_SIZE: u32 = 0x88;

/// `op_repay_loan` (`0x00539F30`): `op+4` comerciante prestatario, `op+8` índice de la tarea de préstamo.
pub fn op_repay_loan(m: &mut dyn Mem, op: u32) {
    let id = m.r32(op + 8);
    let task = if u32::from(m.r16(SCHED + 0xc)) < id { 0 } else { m.r32(SCHED).wrapping_add(id.wrapping_mul(0x18)) };
    if task == 0 || m.r16(task + 6) != 1 {
        return;
    }
    let borrower = m.r32(op + 4);
    if borrower != u32::from(m.r8(task + 0x11)) {
        return;
    }
    let amount = m.r32(task + 8);
    if borrower < mc(m) {
        let mer = merchant_addr(m, borrower);
        let v = m.r32(mer).wrapping_sub(amount);
        m.w32(mer, v);
        m.w8(task + 0x11, 0xff);
    }
    let lender = u32::from(m.r8(task + 0x10));
    if lender < mc(m) {
        let mer = merchant_addr(m, lender);
        let v = m.r32(mer).wrapping_add(amount);
        m.w32(mer, v);
        m.w8(task + 0x10, 0xff);
    }
}

/// `town_deserialize_tavern_block` (`0x00525E30`): copia el bloque de taberna (0x14 bytes) a `+0x82C`; devuelve `src + 0x14`.
pub fn deserialize_tavern_block(m: &mut dyn Mem, town: u32, src: u32) -> u32 {
    for (d, s, n) in [
        (0x82cu32, 0u32, 1u32),
        (0x82d, 1, 1),
        (0x82e, 2, 2),
        (0x830, 4, 4),
        (0x834, 8, 1),
        (0x835, 9, 1),
        (0x836, 10, 2),
        (0x838, 12, 4),
        (0x83c, 0x10, 1),
        (0x83d, 0x11, 1),
        (0x83e, 0x12, 1),
        (0x83f, 0x13, 1),
    ] {
        for k in 0..n {
            let v = m.r8(src + s + k);
            m.w8(town + d + k, v);
        }
    }
    src + 0x14
}

/// `town_reset_tavern_state` (`0x00525EE0`).
pub fn reset_tavern_state(m: &mut dyn Mem, town: u32) {
    m.w8(town + 0x83c, 0);
    m.w8(town + 0x83d, 0xff);
    m.w8(town + 0x83e, 0xff);
    m.w8(town + 0x83f, 0);
    m.w8(town + 0x82c, 0xff);
    m.w8(town + 0x82d, 0);
    m.w16(town + 0x82e, 0xffff);
    m.w16(town + 0x830, 0xffff);
    m.w16(town + 0x832, 0xffff);
    m.w8(town + 0x834, 0xff);
    m.w8(town + 0x835, 0);
}

/// `0x00530400`: sitio `i` (`0x44C` bytes cada uno) o 0 si el índice no es menor que el recuento `0x6DE4A8`.
fn site_addr(m: &mut dyn Mem, i: u32) -> u32 {
    if i >= u32::from(m.r16(WORLD + 8)) {
        return 0;
    }
    m.r32(WORLD + 0x74).wrapping_add(i.wrapping_mul(0x44c))
}

/// `town_tavern_thief_available_to_player` (`0x00526120`).
pub fn tavern_thief_available(m: &mut dyn Mem, town: u32) -> bool {
    let b = m.r8(town + 0x83c);
    if b == 0xff {
        let index = u32::from(m.r16(town + 0x784));
        let site = site_addr(m, index);
        return m.r16(site.wrapping_add(0x2ca)) < m.r16(G_SITES);
    }
    u32::from(b) == m.r32(G_PLAYER)
}

/// `town_tavern_ship_task_available_to_player` (`0x005261A0`).
pub fn tavern_ship_task_available(m: &mut dyn Mem, town: u32) -> bool {
    let b = m.r8(town + 0x83f);
    b == 0xff || u32::from(b) == m.r32(G_PLAYER)
}

/// `town_tavern_weapons_dealer_available_to_player` (`0x00526280`): byte bajo del resultado.
pub fn tavern_weapons_dealer(m: &mut dyn Mem, town: u32) -> bool {
    let player = m.r32(G_PLAYER);
    let owner = u32::from(m.r8(town + 0x83d));
    if m.r8(town + 0x2c8) & 0x10 == 0 {
        let towns = m.r32(TOWN_TABLE);
        let own = u32::from(m.r8(town + 0x2c0));
        if m.r16(towns.wrapping_add(0x874).wrapping_add(own.wrapping_mul(0x9f8))) != 0 {
            let any_stock = (0..4u32).any(|k| (m.r32(town + 0x54 + 4 * k) as i32) > 9);
            if !any_stock && player != owner {
                return false;
            }
        }
    }
    if owner != 0xff {
        return owner == player;
    }
    true
}

/// `town_tavern_ware_offer_available` (`0x005265E0`): hay una mercancía con excedente suficiente para ofrecerla en la taberna.
pub fn tavern_ware_offer_available(m: &mut dyn Mem, town: u32) -> bool {
    let c = u32::from(m.r8(town + 0x83e));
    if mc(m) <= c || c == m.r32(G_PLAYER) {
        for i in 0..0x14u32 {
            if (1u32 << i) & 0x47faf != 0 {
                let limit = (m.r32(town + 0x4f4 + 16 * i) as i32).wrapping_add(40000);
                if limit < m.r32(town + 4 + 4 * i) as i32 {
                    return true;
                }
            }
        }
    }
    false
}

/// `town_tavern_supply_contract_available` (`0x00526880`): una de las dos tareas de contrato de la ciudad (`+0x830`) es del jugador local.
pub fn tavern_contract_available(m: &mut dyn Mem, town: u32) -> bool {
    for k in 0..2u32 {
        let index = u32::from(m.r16(town + 0x830 + 2 * k));
        if index < u32::from(m.r16(SCHED + 0xc)) {
            let task = m.r32(SCHED).wrapping_add(index.wrapping_mul(24));
            if m.r8(task + 0x16) as u32 == m.r32(G_PLAYER) {
                return true;
            }
        }
    }
    false
}

/// `town_find_tavern_pirate_captain` (`0x005261D0`): primer capitán pirata de la lista de la ciudad (`+0x82E`) del jugador local; si no hay, el primero
/// libre (propietario `0xFF`). `0xFFFF` si no hay ninguno. Tabla de capitanes en `[0x006DD7A0]`, recuento en `0x006DD892`.
pub fn tavern_pirate_captain(m: &mut dyn Mem, town: u32) -> u32 {
    const TABLE: u32 = 0x006d_d7a0;
    const COUNT: u32 = 0x006d_d892;
    let head = u32::from(m.r16(town + 0x82e));
    for want_free in [false, true] {
        let mut i = head;
        while i < u32::from(m.r16(COUNT)) {
            let e = m.r32(TABLE).wrapping_add(i << 4);
            if is_pirate_kind(m, e) {
                let owner = u32::from(m.r8(e + 0xf));
                if (want_free && owner == 0xff) || (!want_free && owner == m.r32(G_PLAYER)) {
                    return i;
                }
            }
            i = u32::from(m.r16(e));
        }
    }
    0xffff
}

/// `town_find_free_tavern_captain` (`0x005269A0`) sobre la imagen plana: capitán no pirata de la lista de la ciudad con propietario `owner`
/// y, si no, con propietario `0xFF`; `0xFFFF` si no hay.
pub fn tavern_captain(m: &mut dyn Mem, town: u32, owner: u16) -> u32 {
    let head = u32::from(m.r16(town + 0x82e));
    for want_free in [false, true] {
        let mut i = head;
        while i < u32::from(m.r16(0x006d_d892)) {
            let e = m.r32(0x006d_d7a0).wrapping_add(i << 4);
            if m.r8(e + 8) <= 0x20 {
                let tag = u32::from(m.r8(e + 0xf));
                if (!want_free && tag == u32::from(owner)) || (want_free && tag == 0xff) {
                    return i;
                }
            }
            i = u32::from(m.r16(e));
        }
    }
    0xffff
}

/// `auto_trader_is_pirate_kind` (`0x004FE150`).
pub fn is_pirate_kind(m: &mut dyn Mem, ptr: u32) -> bool {
    m.r8(ptr + 8) > 0x20
}
