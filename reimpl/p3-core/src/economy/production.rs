//! Producción diaria de las instalaciones de la ciudad (`produce_*`, 0x0050E690–0x0050FC99).
//!
//! Cada función suma la producción de los empleados a las existencias, a la producción del día
//! (`+0xC4`) y al historial de 8 días (`+0x13C`), y en `production_values` (`+0x490`) la que tendría
//! con `cap` empleados (lo que muestra el mercado). Los insumos se apuntan como consumo de empresas
//! (`+0x64`); si faltan, la producción baja en proporción y `input_shortage` (`+0`) pasa a 1.
//! La aritmética es la del original: enteros de 32 bits que desbordan, divisiones truncadas y, donde el
//! original usa desplazamientos lógicos, conversión a `u32`.
//!
//! El original escribe `production_values` y lee el umbral del aceite en la ciudad de la tabla global
//! que indica el byte `+7` de la instalación, y lo demás en la ciudad que recibe; aquí son la misma
//! ciudad, como en las llamadas del tick diario.

use crate::town::{off, Town};

const W_GRAIN: usize = 0;
const W_MEAT: usize = 1;
const W_FISH: usize = 2;
const W_BEER: usize = 3;
const W_SALT: usize = 4;
const W_HONEY: usize = 5;
const W_WINE: usize = 7;
const W_CLOTH: usize = 8;
const W_SKINS: usize = 9;
const W_WHALE_OIL: usize = 10;
const W_TIMBER: usize = 11;
const W_TOOLS: usize = 12;
const W_LEATHER: usize = 13;
const W_WOOL: usize = 14;
const W_PITCH: usize = 15;
const W_PIG_IRON: usize = 16;
const W_HEMP: usize = 17;
const W_POTTERY: usize = 18;
const W_BRICKS: usize = 19;

/// Día dentro del historial de 8 días.
pub fn history_day(tick: u32) -> usize {
    ((tick >> 8) & 7) as usize
}

fn hist(t: &mut Town, w: usize, day: usize, v: i32) {
    t.set_u16(off::PRODUCTION_HISTORY + 16 * w + 2 * day, v as u16);
}
fn add_business(t: &mut Town, w: usize, v: i32) {
    t.add_i32(off::BUSINESS_CONSUMPTION + 4 * w, v);
}
fn add_produced(t: &mut Town, w: usize, v: i32) {
    t.add_i32(off::DAILY_PRODUCTION + 4 * w, v);
}
fn add_value(t: &mut Town, w: usize, v: i32) {
    t.add_i32(off::PRODUCTION_VALUES + 4 * w, v);
}
fn add_stock(t: &mut Town, w: usize, v: i32) {
    t.add_i32(off::STOCK + 4 * w, v);
}
fn shortage(t: &mut Town) {
    t.set_u16(off::INPUT_SHORTAGE, 1);
}
/// Reparto proporcional redondeado hacia arriba del original: `(a·b + c − 1) / c`.
fn ceil_share(a: i32, b: i32, c: i32) -> i32 {
    a.wrapping_mul(b).wrapping_add(c).wrapping_sub(1) / c
}

/// Empleados y eficiencia de la instalación `f`.
fn staff(t: &Town, f: usize) -> (i32, i32) {
    (t.employees(f) as i32, t.efficiency(f))
}

/// Factor de cosecha de miel, grano y vino según invierno (`0x2`) y las banderas `0x2000`/`0x4000`.
fn winter(t: &Town) -> bool {
    t.flags() & 2 != 0
}

/// `produce_hunting_lodge` (0x0050ED70): pieles con herramientas y cáñamo.
pub fn hunting_lodge(t: &mut Town, f: usize, tick: u32, cap: i32) {
    let day = history_day(tick);
    let (e, eff) = staff(t, f);
    let a = eff.wrapping_mul(20) / 3;
    add_value(t, W_SKINS, (a.wrapping_mul(cap) as u32 >> 10) as i32);
    let mut produced = ((2 * e) / 3).wrapping_add(e.wrapping_mul(a)) >> 10;
    add_produced(t, W_SKINS, produced);
    let need = produced / 20;
    add_business(t, W_TOOLS, need);
    add_business(t, W_HEMP, need);
    let tools = t.stock(W_TOOLS);
    let (mut used_tools, mut used_hemp) = (need, need);
    if used_tools > tools {
        used_tools = tools;
        shortage(t);
        produced = used_tools.wrapping_mul(20);
        used_hemp = produced / 20;
    }
    let hemp = t.stock(W_HEMP);
    if used_hemp > hemp {
        used_hemp = hemp;
        shortage(t);
        produced = used_hemp.wrapping_mul(20);
        used_tools = produced / 20;
    }
    t.set_stock(W_TOOLS, tools.wrapping_sub(used_tools));
    t.set_stock(W_HEMP, hemp.wrapping_sub(used_hemp));
    add_stock(t, W_SKINS, produced);
    hist(t, W_SKINS, day, produced);
}

/// `produce_fishermans_hut` (0x0050E690): pescado con sal y cáñamo, y aceite de ballena. Con
/// `limit_oil` el aceite solo se guarda mientras las existencias no lleguen al umbral 2.
pub fn fishermans_hut(t: &mut Town, f: usize, tick: u32, cap: u32, limit_oil: bool) {
    let day = history_day(tick);
    let (e, eff) = staff(t, f);
    let cap = (cap & 0xFFFF) as i32;
    let whaling = t.i32(off::FIELD_2CC);
    add_value(t, W_FISH, eff.wrapping_mul(cap).wrapping_mul(200) / 3 >> 10);
    add_value(t, W_WHALE_OIL, whaling.wrapping_mul(cap).wrapping_mul(27) >> 10);
    let mut fish = e.wrapping_mul(eff).wrapping_mul(200) / 3 >> 10;
    add_produced(t, W_FISH, fish);
    let mut oil = (e.wrapping_mul(whaling).wrapping_mul(27) as u32 >> 10) as i32;
    add_produced(t, W_WHALE_OIL, oil);
    let mut salt_need = fish / 5;
    let mut hemp_need = fish / 10;
    add_business(t, W_SALT, salt_need);
    add_business(t, W_HEMP, hemp_need);
    let salt = t.stock(W_SALT);
    if salt_need > salt {
        shortage(t);
        if salt == 0 {
            (salt_need, fish, oil, hemp_need) = (0, 0, 0, 0);
        } else {
            hemp_need = ceil_share(salt, hemp_need, salt_need);
            oil = ceil_share(salt, oil, salt_need);
            fish = ceil_share(salt, fish, salt_need);
            salt_need = salt;
        }
    }
    let hemp = t.stock(W_HEMP);
    if hemp_need > hemp {
        shortage(t);
        if hemp == 0 {
            (salt_need, fish, oil, hemp_need) = (0, 0, 0, 0);
        } else {
            salt_need = ceil_share(hemp, salt_need, hemp_need);
            oil = ceil_share(hemp, oil, hemp_need);
            fish = ceil_share(hemp, fish, hemp_need);
            hemp_need = hemp;
        }
    }
    if !limit_oil || t.threshold(W_WHALE_OIL, 2) > t.stock(W_WHALE_OIL) {
        add_stock(t, W_WHALE_OIL, oil);
        hist(t, W_WHALE_OIL, day, oil);
    }
    add_stock(t, W_FISH, fish);
    hist(t, W_FISH, day, fish);
    t.set_stock(W_SALT, salt.wrapping_sub(salt_need));
    t.set_stock(W_HEMP, hemp.wrapping_sub(hemp_need));
}

/// `produce_brewery` (0x0050E8B0): cerveza con grano y madera; la reserva también consume.
pub fn brewery(t: &mut Town, f: usize, tick: u32, idle: u32, cap: u32) {
    let day = history_day(tick);
    let (e, eff) = staff(t, f);
    let a = eff.wrapping_mul(47);
    add_value(t, W_BEER, ((cap & 0xFFFF) as i32).wrapping_mul(a) >> 10);
    let mut beer = e.wrapping_mul(a) >> 10;
    add_produced(t, W_BEER, beer);
    let mut grain_need = beer / 4;
    let mut timber_need = beer / 10;
    let reserve = ((idle & 0xFFFF) as i32).wrapping_mul(eff).wrapping_mul(47) >> 10;
    // La parte de la reserva lleva grano y madera intercambiados (1/10 y 1/4).
    add_business(t, W_TIMBER, (reserve / 4).wrapping_add(timber_need));
    add_business(t, W_GRAIN, (reserve / 10).wrapping_add(grain_need));
    let grain = t.stock(W_GRAIN);
    if grain_need > grain {
        grain_need = grain;
        shortage(t);
        beer = grain.wrapping_mul(4);
        timber_need = beer / 10;
    }
    let timber = t.stock(W_TIMBER);
    if timber_need > timber {
        timber_need = timber;
        shortage(t);
        beer = timber.wrapping_mul(10);
        grain_need = beer / 4;
    }
    t.set_stock(W_TIMBER, timber.wrapping_sub(timber_need));
    t.set_stock(W_GRAIN, grain.wrapping_sub(grain_need));
    add_stock(t, W_BEER, beer);
    hist(t, W_BEER, day, beer);
}

/// `produce_workshop` (0x0050FBC0): herramientas con madera y arrabio (1:1).
pub fn workshop(t: &mut Town, f: usize, tick: u32, cap: i32) {
    let day = history_day(tick);
    let (e, eff) = staff(t, f);
    add_value(t, W_TOOLS, (eff.wrapping_mul(cap).wrapping_mul(20) as u32 >> 10) as i32);
    let mut made = e.wrapping_mul(eff).wrapping_mul(20) >> 10;
    add_business(t, W_TIMBER, made);
    add_business(t, W_PIG_IRON, made);
    add_produced(t, W_TOOLS, made);
    let (mut timber_used, mut iron_used) = (made, made);
    let timber = t.stock(W_TIMBER);
    if made > timber {
        timber_used = timber;
        shortage(t);
        made = timber_used;
        iron_used = timber_used;
    }
    let iron = t.stock(W_PIG_IRON);
    if iron < iron_used {
        iron_used = iron;
        shortage(t);
        made = iron_used;
        timber_used = iron_used;
    }
    add_stock(t, W_TOOLS, made);
    hist(t, W_TOOLS, day, made);
    let left = t.stock(W_TIMBER).wrapping_sub(timber_used);
    t.set_stock(W_TIMBER, left);
    t.set_stock(W_PIG_IRON, t.stock(W_PIG_IRON).wrapping_sub(iron_used));
    if left < 0 {
        t.set_stock(W_TIMBER, 0);
    }
}

/// `produce_apiary` (0x0050EA00): miel, sin insumos.
pub fn apiary(t: &mut Town, f: usize, tick: u32, cap: i32) {
    let day = history_day(tick);
    let (e, eff) = staff(t, f);
    let flags = t.flags();
    let mut k: i32 = if winter(t) { 2 } else { 4 };
    if flags & 0x2000 != 0 {
        k >>= 1;
    } else if flags & 0x4000 != 0 {
        k *= 3;
    }
    let x = eff.wrapping_mul(k).wrapping_mul(cap).wrapping_mul(40) as u32;
    add_value(t, W_HONEY, (x / 3 >> 12) as i32);
    let honey = e.wrapping_mul(eff).wrapping_mul(k).wrapping_mul(40) / 3 >> 12;
    add_stock(t, W_HONEY, honey);
    add_produced(t, W_HONEY, honey);
    hist(t, W_HONEY, day, honey);
}

/// `produce_farm_grain` (0x0050EAD0): grano; sin empleados no hace nada.
pub fn farm_grain(t: &mut Town, f: usize, tick: u32, cap: i32) {
    let day = history_day(tick);
    if t.employees(f) == 0 {
        return;
    }
    let (e, eff) = staff(t, f);
    let flags = t.flags();
    let mut k: i32 = if winter(t) { 4 } else { 6 };
    if flags & 0x2000 != 0 {
        k /= 3;
    } else if flags & 0x4000 != 0 {
        k *= 2;
    }
    let x = eff.wrapping_mul(k).wrapping_mul(cap).wrapping_mul(200) as u32;
    add_value(t, W_GRAIN, (x / 18432) as i32);
    let y = e.wrapping_mul(eff).wrapping_mul(k);
    let grain = if y > 0x80_0000 {
        (y / 6144).wrapping_mul(200) / 3
    } else {
        y.wrapping_mul(200) / 3 / 6144
    };
    add_stock(t, W_GRAIN, grain);
    add_produced(t, W_GRAIN, grain);
    hist(t, W_GRAIN, day, grain);
}

/// `produce_farm_cattle` (0x0050F460): carne y cuero con sal y madera.
pub fn farm_cattle(t: &mut Town, f: usize, tick: u32, cap: i32) {
    let day = history_day(tick);
    let (e, eff) = staff(t, f);
    let q = (eff.wrapping_mul(cap).wrapping_mul(50) as u32 / 3) as i32;
    add_value(t, W_MEAT, q >> 10);
    add_value(t, W_LEATHER, q.wrapping_mul(40) / 100 >> 10);
    let mut meat = e.wrapping_mul(eff).wrapping_mul(50) / 3 >> 10;
    let mut leather = meat.wrapping_mul(40) / 100;
    add_produced(t, W_LEATHER, leather);
    add_produced(t, W_MEAT, meat);
    let mut salt_need = (leather / 10).wrapping_add(meat / 4);
    let mut timber_need = leather / 5;
    add_business(t, W_SALT, salt_need);
    add_business(t, W_TIMBER, timber_need);
    let salt = t.stock(W_SALT);
    if salt_need > salt {
        shortage(t);
        if salt <= 0 {
            (salt_need, timber_need, leather, meat) = (0, 0, 0, 0);
        } else {
            meat = ceil_share(salt, meat, salt_need);
            leather = ceil_share(salt, leather, salt_need);
            timber_need = ceil_share(salt, timber_need, salt_need);
            salt_need = salt;
        }
    }
    let timber = t.stock(W_TIMBER);
    if timber_need > timber {
        shortage(t);
        if timber <= 0 {
            (salt_need, timber_need, leather, meat) = (0, 0, 0, 0);
        } else {
            meat = ceil_share(timber, meat, timber_need);
            leather = ceil_share(timber, leather, timber_need);
            salt_need = ceil_share(timber, salt_need, timber_need);
            timber_need = timber;
        }
    }
    add_stock(t, W_MEAT, meat);
    hist(t, W_MEAT, day, meat);
    add_stock(t, W_LEATHER, leather);
    hist(t, W_LEATHER, day, leather);
    t.set_stock(W_SALT, salt.wrapping_sub(salt_need));
    t.set_stock(W_TIMBER, timber.wrapping_sub(timber_need));
}

/// `produce_sawmill` (0x0050ECD0): madera, sin insumos.
pub fn sawmill(t: &mut Town, f: usize, tick: u32, cap: i32) {
    let day = history_day(tick);
    let (e, eff) = staff(t, f);
    let timber = e.wrapping_mul(eff).wrapping_mul(400) / 3 >> 10;
    add_value(
        t,
        W_TIMBER,
        (eff.wrapping_mul(cap).wrapping_mul(400) as u32 / 3072) as i32,
    );
    add_stock(t, W_TIMBER, timber);
    add_produced(t, W_TIMBER, timber);
    hist(t, W_TIMBER, day, timber);
}

/// `produce_weaving_mill` (0x0050EEB0): paño con lana (1:1).
pub fn weaving_mill(t: &mut Town, f: usize, tick: u32, cap: i32) {
    let day = history_day(tick);
    let (e, eff) = staff(t, f);
    let a = eff.wrapping_mul(20);
    add_value(t, W_CLOTH, (a.wrapping_mul(cap) as u32 >> 10) as i32);
    let mut cloth = e.wrapping_mul(a) >> 10;
    add_produced(t, W_CLOTH, cloth);
    add_business(t, W_WOOL, cloth);
    let wool = t.stock(W_WOOL);
    if wool < cloth {
        shortage(t);
        cloth = wool;
    }
    t.set_stock(W_WOOL, wool.wrapping_sub(cloth));
    add_stock(t, W_CLOTH, cloth);
    hist(t, W_CLOTH, day, cloth);
}

/// `produce_saltworks` (0x0050EF50): sal con madera (8/10); la reserva también consume madera.
pub fn saltworks(t: &mut Town, f: usize, tick: u32, idle: u32, cap: u32) {
    let day = history_day(tick);
    let (e, eff) = staff(t, f);
    let a = eff.wrapping_mul(167) / 5;
    let reserve = e.wrapping_add((idle & 0xFFFF) as i32).wrapping_mul(a) >> 10;
    add_business(t, W_TIMBER, reserve.wrapping_mul(8) / 10);
    add_value(t, W_SALT, ((cap & 0xFFFF) as i32).wrapping_mul(a) >> 10);
    let mut salt = e.wrapping_mul(a) >> 10;
    add_produced(t, W_SALT, salt);
    let mut timber_need = salt.wrapping_mul(8) / 10;
    let timber = t.stock(W_TIMBER);
    if timber_need > timber {
        timber_need = timber;
        shortage(t);
        salt = timber_need.wrapping_mul(10) >> 3;
    }
    t.set_stock(W_TIMBER, timber.wrapping_sub(timber_need));
    add_stock(t, W_SALT, salt);
    hist(t, W_SALT, day, salt);
}

/// `produce_iron_smelter` (0x0050F650): arrabio, sin insumos.
pub fn iron_smelter(t: &mut Town, f: usize, tick: u32, cap: i32) {
    let day = history_day(tick);
    let (e, eff) = staff(t, f);
    let a = eff.wrapping_mul(100) / 3;
    add_value(t, W_PIG_IRON, (a.wrapping_mul(cap) as u32 >> 10) as i32);
    let iron = e.wrapping_mul(a) >> 10;
    add_stock(t, W_PIG_IRON, iron);
    add_produced(t, W_PIG_IRON, iron);
    hist(t, W_PIG_IRON, day, iron);
}

/// `produce_farm_sheep` (0x0050F060): lana, sin insumos.
pub fn farm_sheep(t: &mut Town, f: usize, tick: u32, cap: i32) {
    let day = history_day(tick);
    let (e, eff) = staff(t, f);
    add_value(
        t,
        W_WOOL,
        (eff.wrapping_mul(cap).wrapping_mul(167) as u32 / 5 >> 10) as i32,
    );
    let wool = e.wrapping_mul(eff).wrapping_mul(167) / 5 >> 10;
    add_stock(t, W_WOOL, wool);
    add_produced(t, W_WOOL, wool);
    hist(t, W_WOOL, day, wool);
}

/// `produce_vineyard` (0x0050F100): vino, calculado en 64 bits.
pub fn vineyard(t: &mut Town, f: usize, tick: u32, cap: u32) {
    let day = history_day(tick);
    let flags = t.flags();
    let mut k: i64 = if winter(t) { 2 } else { 4 };
    if flags & 0x2000 != 0 {
        k /= 2;
    } else if flags & 0x4000 != 0 {
        k = k.wrapping_mul(2);
    }
    let a = (t.efficiency(f) as i64).wrapping_mul(k);
    let grow = |x: i64| (x.wrapping_mul(0x43) / 5 >> 12) as i32;
    add_value(t, W_WINE, grow(a.wrapping_mul(cap as i64)));
    let wine = grow(a.wrapping_mul(t.employees(f) as i64));
    add_stock(t, W_WINE, wine);
    add_produced(t, W_WINE, wine);
    hist(t, W_WINE, day, wine);
}

/// `produce_pottery` (0x0050F220): cerámica con madera (1/2).
pub fn pottery(t: &mut Town, f: usize, tick: u32, cap: u32) {
    let day = history_day(tick);
    let (e, eff) = staff(t, f);
    let a = eff.wrapping_mul(67) / 5;
    add_value(t, W_POTTERY, ((cap & 0xFFFF) as i32).wrapping_mul(a) >> 10);
    let mut made = e.wrapping_mul(a) >> 10;
    let mut timber_need = made / 2;
    add_business(t, W_TIMBER, timber_need);
    let timber = t.stock(W_TIMBER);
    if timber_need > timber {
        timber_need = timber;
        shortage(t);
        made = timber_need.wrapping_mul(2);
    }
    t.set_stock(W_TIMBER, timber.wrapping_sub(timber_need));
    add_stock(t, W_POTTERY, made);
    add_produced(t, W_POTTERY, made);
    hist(t, W_POTTERY, day, made);
}

/// `produce_brickworks` (0x0050F2E0): ladrillos con madera (1/2).
pub fn brickworks(t: &mut Town, f: usize, tick: u32, cap: i32) {
    let day = history_day(tick);
    let (e, eff) = staff(t, f);
    let a = eff.wrapping_mul(134);
    add_value(t, W_BRICKS, (a.wrapping_mul(cap) as u32 >> 10) as i32);
    let mut made = e.wrapping_mul(a) >> 10;
    let mut timber_need = made / 2;
    add_business(t, W_TIMBER, timber_need);
    add_produced(t, W_BRICKS, made);
    let timber = t.stock(W_TIMBER);
    if timber_need > timber {
        timber_need = timber;
        shortage(t);
        made = timber_need.wrapping_mul(2);
    }
    t.set_stock(W_TIMBER, timber.wrapping_sub(timber_need));
    add_stock(t, W_BRICKS, made);
    hist(t, W_BRICKS, day, made);
}

/// `produce_pitchmaker` (0x0050F390): brea con madera (1:1); la reserva también consume madera.
pub fn pitchmaker(t: &mut Town, f: usize, tick: u32, idle: u32, cap: u32) {
    let day = history_day(tick);
    let (e, eff) = staff(t, f);
    let a = eff.wrapping_mul(67) / 5;
    add_business(
        t,
        W_TIMBER,
        e.wrapping_add((idle & 0xFFFF) as i32).wrapping_mul(a) >> 10,
    );
    add_value(t, W_PITCH, ((cap & 0xFFFF) as i32).wrapping_mul(a) >> 10);
    let mut pitch = e.wrapping_mul(a) >> 10;
    let timber = t.stock(W_TIMBER);
    if pitch > timber {
        pitch = timber;
        shortage(t);
    }
    t.set_stock(W_TIMBER, timber.wrapping_sub(pitch));
    add_stock(t, W_PITCH, pitch);
    add_produced(t, W_PITCH, pitch);
    hist(t, W_PITCH, day, pitch);
}

/// `produce_farm_hemp` (0x0050EBF0): cáñamo, sin insumos.
pub fn farm_hemp(t: &mut Town, f: usize, tick: u32, cap: i32) {
    let day = history_day(tick);
    let flags = t.flags();
    let k: i32 = if winter(t) {
        3
    } else if flags & 0x2000 != 0 {
        4
    } else if flags & 0x4000 != 0 {
        9
    } else {
        6
    };
    let (e, eff) = staff(t, f);
    let a = eff.wrapping_mul(k);
    add_value(t, W_HEMP, (a.wrapping_mul(cap).wrapping_mul(50) as u32 / 18432) as i32);
    let hemp = e.wrapping_mul(a).wrapping_mul(50) / 3 / 6144;
    add_stock(t, W_HEMP, hemp);
    add_produced(t, W_HEMP, hemp);
    hist(t, W_HEMP, day, hemp);
}

/// Armas de mano (mercancías 20–23) por empleado y eficiencia, en 1/65536 (`0x006735E4`).
pub const WEAPON_RATE: [i8; 4] = [32, 40, 26, 20];
/// Tope de armas de barco en el almacén de la ciudad, por 20 (`0x00672CB4`).
pub const SHIP_WEAPON_MAX: [u16; 6] = [1000, 1000, 2000, 2000, 2000, 1000];
/// Insumos por cada 100 armas de barco: madera, cuero, herramientas y cáñamo (`0x006735C4`, `CC`,
/// `D4`, `DC`).
pub const SHIP_WEAPON_TIMBER: [u8; 6] = [50, 50, 50, 50, 25, 50];
pub const SHIP_WEAPON_LEATHER: [u8; 6] = [10, 10, 10, 10, 5, 10];
pub const SHIP_WEAPON_TOOLS: [u8; 6] = [10, 10, 10, 10, 30, 60];
pub const SHIP_WEAPON_HEMP: [u8; 6] = [10, 10, 10, 10, 5, 10];

/// `get_shipyard_level` (0x0052BB00): nivel del astillero, `experiencia / 420000 + 2`, como mucho 5.
pub fn shipyard_level(t: &Town) -> u8 {
    let level = ((t.u32(off::SHIPYARD_EXPERIENCE) as u64 * 0x09FC_8735) >> 46) as u32 + 2;
    if level >= 6 {
        5
    } else {
        level as u8
    }
}

/// Factor de insumo: existencias / necesidad en `f32`, como `fild` + `fstp dword` + `fdiv`.
fn input_ratio(stock: i32, need: i32) -> f32 {
    ((stock as f32) as f64 / ((need as u32) as f64 as f32) as f64) as f32
}

/// `(int)(cantidad · factor)` con la cantidad redondeada antes a `f32`.
fn scale(amount: i32, factor: f32) -> i32 {
    crate::numeric::ftol_i32((amount as f32) as f64 * factor as f64)
}

/// `produce_weaponsmith` (0x0050F6E0): fabrica `product` (20–23 armas de mano, 24–29 armas de
/// barco, 30 sables de abordaje) con madera, cuero, herramientas y cáñamo. Si falta algún insumo la
/// producción baja en proporción al más escaso y `weaponsmith_shortage` (`+0x775`) anota cuáles
/// (bits 1, 2, 4, 8; `0x10` si el producto no existe).
pub fn weaponsmith(t: &mut Town, f: usize, tick: u32, product: u32) {
    let day = history_day(tick);
    if t.efficiency(f) == 0 {
        t.set_i32(Town::fac(f, crate::town::fac::EFFICIENCY), 0x400);
    }
    let (e, eff) = staff(t, f);
    for (i, rate) in WEAPON_RATE.iter().enumerate() {
        add_produced(t, 20 + i, (*rate as i32).wrapping_mul(e).wrapping_mul(eff) >> 16);
    }
    t.set_u8(off::WEAPONSMITH_SHORTAGE, 0);
    let kind = product.wrapping_sub(20);
    // Necesidades de madera, cuero, cáñamo y herramientas, y cantidad fabricada.
    let (timber, leather, hemp, tools, made, ship);
    if kind < 4 {
        let m = (WEAPON_RATE[kind as usize] as i32).wrapping_mul(e).wrapping_mul(eff) >> 14;
        (timber, leather, hemp, tools, made, ship) = (m.wrapping_mul(10), m, m, m, m, false);
    } else if kind < 10 {
        let k = (kind - 4) as usize;
        if t.i32(off::SHIP_WEAPONS + 4 * k) > SHIP_WEAPON_MAX[k] as i32 * 20 {
            return;
        }
        let mut work = e * 71;
        if k < 5 {
            work += work;
        }
        let m = eff.wrapping_mul(work) >> 10;
        let part = |rate: u8| (rate as i32).wrapping_mul(m) / 100;
        timber = part(SHIP_WEAPON_TIMBER[k]);
        leather = part(SHIP_WEAPON_LEATHER[k]);
        hemp = part(SHIP_WEAPON_HEMP[k]);
        tools = part(SHIP_WEAPON_TOOLS[k]);
        (made, ship) = (m, true);
    } else if kind == 10 {
        let m = e.wrapping_mul(eff).wrapping_shl(5) >> 14;
        (timber, leather, hemp, tools, made, ship) = (m.wrapping_mul(10), m, m, m, m, true);
    } else {
        t.set_u8(off::WEAPONSMITH_SHORTAGE, 0x10);
        return;
    }
    add_business(t, W_TIMBER, timber);
    add_business(t, W_LEATHER, leather);
    add_business(t, W_HEMP, hemp);
    add_business(t, W_TOOLS, tools);
    let mut factor = 1.0f32;
    let mut missing = 0u8;
    for (bit, ware, need) in [
        (1u8, W_TIMBER, timber),
        (2, W_LEATHER, leather),
        (4, W_HEMP, hemp),
        (8, W_TOOLS, tools),
    ] {
        let stock = t.stock(ware);
        if stock < need {
            if stock > 0 {
                let r = input_ratio(stock, need);
                // La primera razón se toma tal cual; las demás solo si son menores (`fcomp`).
                if bit == 1 || factor > r {
                    factor = r;
                }
            } else {
                factor = 0.0;
                missing |= bit;
            }
        }
    }
    t.set_u8(off::WEAPONSMITH_SHORTAGE, missing);
    let (mut timber, mut leather, mut hemp, mut tools, mut made) = (timber, leather, hemp, tools, made);
    if factor < 1.0 {
        timber = scale(timber, factor);
        leather = scale(leather, factor);
        hemp = scale(hemp, factor);
        tools = scale(tools, factor);
        made = scale(made, factor);
    }
    if !ship {
        add_stock(t, product as usize, made);
        hist(t, product as usize, day, made);
    } else if kind < 10 {
        t.add_i32(off::SHIP_WEAPONS + 4 * (kind as usize - 4), made);
    } else {
        t.add_i32(off::CUTLASSES, made / 10);
    }
    t.set_stock(W_LEATHER, t.stock(W_LEATHER).wrapping_sub(leather));
    t.set_stock(W_HEMP, t.stock(W_HEMP).wrapping_sub(hemp));
    t.set_stock(W_TIMBER, t.stock(W_TIMBER).wrapping_sub(timber));
    t.set_stock(W_TOOLS, t.stock(W_TOOLS).wrapping_sub(tools));
}

/// `weaponsmith_choose_product` (0x00510AA0): mantiene la mercancía prioritaria de la ciudad si es
/// un arma permitida; si no, elige la más escasa entre las armas de mano (frente a su umbral 2), las
/// armas de barco (frente a su tope según habitantes) y los sables. `0xFF` si ninguna baja de 1024.
pub fn weaponsmith_choose_product(t: &Town, f: usize) -> u32 {
    let level = shipyard_level(t) as i32;
    let (level, last_hand) = match level {
        l if l <= 1 => (2, 0x15),
        2 => (2, 0x15),
        l => (l, 0x16 + (l > 4) as u32),
    };
    let first = 0x14 + (t.stock(W_PIG_IRON) <= 0) as u32;
    let priority = t.u32(off::WEAPON_PRIORITY_WARE);
    if priority >= first && priority <= last_hand {
        return priority;
    }
    let mut best = 0xFFu32;
    let mut lowest = 0x400i32;
    for w in first..=last_hand {
        let stock = t.stock(w as usize);
        let limit = t.threshold(w as usize, 2);
        if stock <= limit {
            let r = (stock << 10) / limit;
            if r < lowest {
                lowest = r;
                best = w;
            }
        }
    }
    let town = t.u8(Town::fac(f, crate::town::fac::TOWN)) as i32;
    let base = ((-2 - town) & 3) + (t.i32(off::CITIZENS_TOTAL) >> 9) + 5;
    for i in 0..=level as usize {
        let r = (t.i32(off::SHIP_WEAPONS + 4 * i) << 9) / (SHIP_WEAPON_MAX[i] as i32).wrapping_mul(base);
        if r < lowest {
            lowest = r;
            best = i as u32 + 0x18;
        }
    }
    let x = t.f32(off::SHIPYARD_SOMETHING);
    let k = if x > 0.0 {
        (crate::numeric::ftol_i32(x as f64) >> 2).min(15)
    } else {
        0
    };
    let ships = t.i8(off::CURRENT_SHIP_LEVEL) as i32 + 2;
    let divisor = (ships + k + ships * 4) as u32;
    if (((t.i32(off::CUTLASSES) << 9) as u32 / divisor) as i32) < lowest {
        best = 0x1E;
    }
    best & 0xFF
}
