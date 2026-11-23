//! Operaciones de C08 sobre la imagen de memoria del original: replicar el grupo de selección de barcos (`0x005436D0`), asignar un capitán a
//! un barco (`0x00537EE0`) y poner un edificio a subasta (`0x0053D190`). Las llamadas a otros subsistemas van por [`OpsHost`], con la dirección del
//! original, el `ecx` y los argumentos tal cual los recibe; devuelve lo que devuelve la función original.
use crate::economy::letters::free_town_letter;
use crate::flatmem::Mem;

const G_PLAYER: u32 = 0x006d_fc14;
const WORLD: u32 = 0x006d_e4a0;
const G_MERCHANTS: u32 = 0x006d_e4aa;
const G_TOWNS: u32 = 0x006d_e4b0;
const G_TOWN_TABLE: u32 = 0x006d_e508;
const SHIP_COUNT: u32 = 0x006d_d894;
const SHIP_TABLE_PTR: u32 = 0x006d_d7a4;
const LETTER_COUNT: u32 = 0x006d_d892;
const LETTER_TABLE_PTR: u32 = 0x006d_d7a0;
const SHIP_MANAGER: u32 = 0x006d_d7a0;
const QUEUE: u32 = 0x006d_d730;
const SCHED: u32 = 0x006d_d73c;

pub trait OpsHost {
    /// Llama a la función del original en `addr` (`ecx`, argumentos en orden de la pila) y devuelve `EAX`. Para `0x4D6530` los argumentos son
    /// `[propietario, tipo, ciudad, carta, barco, aux]` (los campos del mensaje que escribe la función).
    fn call(&mut self, m: &mut dyn Mem, addr: u32, ecx: u32, args: &[u32]) -> u32;
}

/// `0x005436D0`: replica en el comerciante `op+4 & 0x7F` el grupo de barcos de la orden.
pub fn replicate_ship_selection_group(m: &mut dyn Mem, host: &mut dyn OpsHost, op: u32) {
    let b4 = m.r8(op + 4);
    let who = u32::from(b4 & 0x7f);
    if who == m.r32(G_PLAYER) {
        return;
    }
    let obj = host.call(m, 0x5303c0, WORLD, &[who]);
    let count = m.r16(SHIP_COUNT);
    let list = op + 6;
    let mut n = 0u32;
    while n < 7 && m.r16(list + 2 * n) < count {
        n += 1;
    }
    let b5 = u32::from(m.r8(op + 5));
    if b4 & 0x80 != 0 {
        // El cuarto argumento conserva los 24 bits altos de EAX (resto de la dirección devuelta y del contador): irrelevante, pero exacto.
        let eax = (obj & 0xffff_0000) | (u32::from(count) & 0xff00) | (b5 >> 7);
        host.call(m, 0x4f5500, obj, &[b5 & 0x7f, n, list, eax, 0]);
    } else {
        host.call(m, 0x4f56c0, obj, &[b5, n, list, 0]);
    }
}

/// `0x00537EE0`: asigna la carta `op+4` (capitán) al barco `op+8` en la ciudad `op+0xC`.
pub fn assign_captain_to_ship(m: &mut dyn Mem, host: &mut dyn OpsHost, op: u32) {
    let letter = m.r32(op + 4);
    let ship_idx = m.r32(op + 8);
    let town_idx = m.r32(op + 0xc);
    if ship_idx >= u32::from(m.r16(SHIP_COUNT)) || letter >= u32::from(m.r16(LETTER_COUNT)) || town_idx >= m.r32(G_TOWNS) {
        return;
    }
    host.call(m, 0x51a850, 0, &[]);
    let ship = m.r32(SHIP_TABLE_PTR).wrapping_add(ship_idx.wrapping_mul(0x180));
    let town = m.r32(G_TOWN_TABLE).wrapping_add(town_idx.wrapping_mul(0x9f8));
    if free_town_letter(m, town, letter) {
        let ltab = m.r32(LETTER_TABLE_PTR);
        let old = m.r16(ship + 0x42);
        m.w16(ltab.wrapping_add(letter << 4), old);
        m.w16(ship + 0x42, letter as u16);
        let owner = u32::from(m.r8(ship));
        host.call(m, 0x4d6530, QUEUE, &[owner, 4, town_idx & 0xff, letter & 0xffff, ship_idx & 0xffff, 0xffff]);
    }
    host.call(m, 0x517770, ship, &[2]);
    host.call(m, 0x51a880, 0, &[]);
}

/// `ShipManager::ship_add_cargo` (`0x005180C0`, `ret 0xC`): añade `amount` unidades de la mercancía `ware` al barco `ship` con el precio `price`
/// (bits de un `f32`); recalcula el precio medio `+0xB4+4·w` y redondea la existencia a lotes (200, 2000 o 10 para las armas).
/// La palabra de control x87 del juego es 53 bits: la aritmética equivale a `f64` con los redondeos a `f32` de cada `fstp dword`.
pub fn ship_add_cargo(m: &mut dyn Mem, host: &mut dyn OpsHost, ship: u32, ware: u32, amount: i32, price: u32) {
    if amount <= 0 {
        return;
    }
    host.call(m, 0x51_7770, ship, &[4]);
    let (stock_at, avg_at) = (ship + 0x54 + ware * 4, ship + 0xb4 + ware * 4);
    let cur = m.r32(stock_at) as i32;
    if cur > 0 {
        let total = cur.wrapping_add(amount);
        m.w32(stock_at, total as u32);
        let avg = f32::from_bits(m.r32(avg_at)) as f64;
        let sum = f64::from(cur as f32) * avg + f64::from(amount as f32) * f64::from(f32::from_bits(price));
        let avg = sum as f32;
        let avg = (f64::from(avg) / f64::from(total as f32)) as f32;
        m.w32(avg_at, avg.to_bits());
    } else {
        m.w32(stock_at, amount as u32);
        m.w32(avg_at, price);
    }
    round_stock_to_lot(m, stock_at, ware);
}

/// Redondeo a lotes de `ship_add_cargo`: mercancías con lote pequeño (200; 10 las de armas, índice >= 0x14) o lote de 2000.
fn round_stock_to_lot(m: &mut dyn Mem, at: u32, ware: u32) {
    let stock = m.r32(at) as i32;
    let rounded = if crate::shipyard::WARE_SMALL_UNIT[ware as usize] != 0 {
        if ware < 0x14 { stock / 200 * 200 } else { stock / 10 * 10 }
    } else {
        stock / 2000 * 2000
    };
    m.w32(at, rounded as u32);
}

/// `OfficeInfo::add_stock` (`0x004FF6F0`, `ret 0xC`): añade `amount` unidades a la existencia `+4+4·w` de la oficina y recalcula el precio medio
/// `+0x3B8+4·w` (la media se hace en doble precisión: cada `fstp qword` del original redondea a `f64`).
pub fn office_add_stock(m: &mut dyn Mem, office: u32, ware: u32, amount: i32, price: u32) {
    if amount <= 0 {
        return;
    }
    let (stock_at, avg_at) = (office + 4 + ware * 4, office + 0x3b8 + ware * 4);
    let cur = m.r32(stock_at) as i32;
    if cur > 0 {
        let total = cur.wrapping_add(amount);
        m.w32(stock_at, total as u32);
        let held = f64::from(f32::from_bits(m.r32(avg_at))) * f64::from(cur);
        let added = f64::from(amount as f32) * f64::from(f32::from_bits(price));
        let avg = ((held + added) / f64::from(total)) as f32;
        m.w32(avg_at, avg.to_bits());
    } else {
        m.w32(stock_at, amount as u32);
        m.w32(avg_at, price);
    }
}

/// `0x0053B5E0` (operación 0x4D): acepta la oferta del comerciante de mercancías de la taberna. `op+0x11` comerciante, `op+0x12` mercancía,
/// `op+0xC` lotes, `op+8` precio total, `op+0xE` barco (si no es válido, la mercancía va a la oficina de `op+0x13`), `op+0x10` vendedor.
/// Carga lo que quepa en el barco y, mientras quede, en los siguientes barcos de su convoy; cobra el precio, resta de las existencias de la
/// ciudad y libera la ranura de oferta `Town+0x83E`.
pub fn accept_trader_offer(m: &mut dyn Mem, host: &mut dyn OpsHost, op: u32) {
    let who = u32::from(m.r8(op + 0x11));
    let ware = u32::from(m.r8(op + 0x12));
    if who >= u32::from(m.r16(G_MERCHANTS)) || ware >= 0x14 {
        return;
    }
    let merchant = host.call(m, 0x53_03c0, WORLD, &[who]);
    let small = crate::shipyard::WARE_SMALL_UNIT[ware as usize] != 0;
    let lots = u32::from(m.r16(op + 0xc));
    let quantity = (lots * 25) as i32 * if small { 8 } else { 80 };
    let price = ((f64::from(m.r32(op + 8) as i32 as f32)) / f64::from(quantity as f32)) as f32;
    let price_bits = price.to_bits();
    let town_of = u32::from(m.r8(op + 0x13));
    let mut left = quantity;
    // ECX y EAX conservan en las llamadas a `0x5308A0` los 16 bits altos de lo calculado antes (`0x006D....` de la dirección del mundo y `5·lotes`).
    let hi_who = (WORLD & 0xffff_0000) | who;
    let hi_town = ((lots * 5) & 0xffff_0000) | town_of;
    if m.r16(op + 0xe) >= m.r16(SHIP_COUNT) {
        let office = host.call(m, 0x53_08a0, WORLD, &[hi_who, hi_town, 0]);
        if office == 0 {
            return;
        }
        office_add_stock(m, office, ware, left, price_bits);
    } else {
        let mut first = true;
        let mut guard = 0u32;
        loop {
            let ship = m.r32(SHIP_TABLE_PTR).wrapping_add(u32::from(m.r16(op + 0xe)).wrapping_mul(0x180));
            if u32::from(m.r8(ship)) == who {
                let free = host.call(m, 0x51_82b0, ship, &[]) as i32;
                let free = free.max(0);
                let put = if free >= left {
                    left
                } else if small {
                    free / 200 * 200
                } else {
                    free / 2000 * 2000
                };
                ship_add_cargo(m, host, ship, ware, put, price_bits);
                host.call(m, 0x51_82b0, ship, &[]);
                left -= put;
            }
            let convoy = m.r16(ship + 8);
            if convoy >= m.r16(0x006d_d896) {
                break;
            }
            let next = if first {
                first = false;
                let table = m.r32(0x006d_d7a8);
                m.r16(table.wrapping_add(u32::from(convoy) * 0x3c + 0xa))
            } else {
                m.r16(ship + 6)
            };
            m.w16(op + 0xe, next);
            guard += 1;
            if next >= m.r16(SHIP_COUNT) || left <= 0 || guard > 0x1_0000 {
                break;
            }
        }
    }
    let total = m.r32(op + 8);
    let (gold, spent) = (m.r32(merchant).wrapping_sub(total), m.r32(merchant + 0x4a0).wrapping_add(total));
    m.w32(merchant, gold);
    m.w32(merchant + 0x4a0, spent);
    let town = m.r32(G_TOWN_TABLE).wrapping_add(town_of.wrapping_mul(0x9f8));
    let at = town + 4 + ware * 4;
    let stock = m.r32(at) as i32;
    m.w32(at, if stock > left { (stock - left) as u32 } else { 0 });
    let seller = u32::from(m.r8(op + 0x10));
    if seller < m.r32(G_TOWNS) {
        let other = host.call(m, 0x53_03c0, WORLD, &[seller]);
        let gold = m.r32(other).wrapping_add(total);
        m.w32(other, gold);
    }
    if m.r8(town + 0x83e) == m.r8(op + 0x11) {
        m.w8(town + 0x83e, 0);
    }
}

/// Suma con saturación de `Merchant+0x16` (límites `0x7FFF` y `0x8000`), como la escribe el original.
fn adjust_reputation(m: &mut dyn Mem, at: u32, delta: i16) {
    let cur = m.r16(at) as i16;
    let value = if delta > 0 {
        if i32::from(cur) < 0x7fff - i32::from(delta) { cur.wrapping_add(delta) as u16 } else { 0x7fff }
    } else if i32::from(cur) + 0x8000 > -i32::from(delta) {
        cur.wrapping_add(delta) as u16
    } else {
        0x8000
    };
    m.w16(at, value);
}

/// `0x0053BC90` (operación 0x50): compra (`op+0xF != 0`) o venta de armas al armero de la taberna. `op+4` precio por arma, `op+8` cantidad pedida,
/// `op+0xC` ciudad, `op+0xD` comerciante, `op+0xE` clase (0..3, mercancías 0x14..0x17) y `op+0x10` barco (si no es válido, la oficina de la ciudad).
/// Recorre el barco y los siguientes barcos del comerciante en su convoy mientras quede cantidad. En la compra el precio medio se recalcula en
/// doble precisión; el ingreso de la venta usa `op+8` entero, como el original. Tras la reputación `+0x16` (+25 en el ejecutable inglés).
pub fn deal_weapons(m: &mut dyn Mem, host: &mut dyn OpsHost, op: u32) {
    if (m.r32(op + 8) as i32) <= 0 || u32::from(m.r8(op + 0xc)) >= m.r32(G_TOWNS) || u32::from(m.r8(op + 0xd)) >= u32::from(m.r16(G_MERCHANTS)) {
        return;
    }
    let town_of = u32::from(m.r8(op + 0xc));
    let who = u32::from(m.r8(op + 0xd));
    let town = m.r32(G_TOWN_TABLE).wrapping_add(town_of.wrapping_mul(0x9f8));
    let merchant = host.call(m, 0x53_03c0, WORLD, &[who]);
    adjust_reputation(m, merchant + 0x16, 25);
    let kind = m.r8(op + 0xe) & 3;
    m.w8(op + 0xe, kind);
    let ware = u32::from(kind) + 0x14;
    let buying = m.r8(op + 0xf) != 0;
    let mut first = true;
    let mut guard = 0u32;
    loop {
        // Cantidad `edi`, existencia y precio medio que se tocan, y el barco (0 si es la oficina).
        let ship;
        let (mut wanted, stock_at, avg_at);
        if m.r16(op + 0x10) < m.r16(SHIP_COUNT) {
            ship = m.r32(SHIP_TABLE_PTR).wrapping_add(u32::from(m.r16(op + 0x10)).wrapping_mul(0x180));
            wanted = if buying {
                let free = host.call(m, 0x51_82b0, ship, &[]) as i32;
                if free > 0 {
                    let all = m.r32(op + 8) as i32;
                    if all.wrapping_mul(10) > free { free / 10 } else { all }
                } else {
                    0
                }
            } else {
                m.r32(op + 8) as i32
            };
            stock_at = ship + ware * 4 + 0x54;
            avg_at = ship + ware * 4 + 0xb4;
        } else {
            ship = 0;
            let office = host.call(m, 0x53_08a0, WORLD, &[who, town_of, 0]);
            if office == 0 {
                return;
            }
            wanted = m.r32(op + 8) as i32;
            stock_at = office + ware * 4 + 4;
            avg_at = office + ware * 4 + 0x3b8;
        }
        let price = m.r32(op + 4) as i32;
        let mut taken_all = false;
        if buying {
            let gold = m.r32(merchant) as i32;
            if gold <= 0 {
                return;
            }
            if price.wrapping_mul(wanted) > gold {
                wanted = gold / price;
            }
            // `0x00526300`: la ciudad entrega como mucho lo que tiene, en decenas.
            let mut amount = wanted.wrapping_mul(10);
            let slot = town + (u32::from(kind) + 0x14) * 4 + 4;
            let have = m.r32(slot) as i32;
            if have < amount {
                amount = have / 10 * 10;
            }
            m.w32(slot, have.wrapping_sub(amount) as u32);
            if amount <= 0 {
                if wanted > 0 {
                    m.w32(op + 8, 0);
                }
                taken_all = true;
            } else {
                let cost = price.wrapping_mul(amount) / 10;
                let spent = (m.r32(merchant + 0x4a0) as i32).wrapping_add(cost);
                m.w32(merchant + 0x4a0, spent as u32);
                m.w32(merchant, (gold.wrapping_sub(cost)) as u32);
                let old = m.r32(stock_at) as i32;
                let new = old.wrapping_add(amount);
                m.w32(stock_at, new as u32);
                let avg = f64::from(f32::from_bits(m.r32(avg_at)));
                let sum = f64::from(cost) + f64::from(old) * avg;
                m.w32(avg_at, ((sum / f64::from(new)) as f32).to_bits());
            }
        } else {
            let mut amount = wanted.wrapping_mul(10);
            let have = m.r32(stock_at) as i32;
            if have < amount {
                wanted = have / 10;
                amount = wanted.wrapping_mul(10);
            }
            if amount > 0 {
                let slot = town + u32::from(kind) * 4 + 0x54;
                let v = (m.r32(slot) as i32).wrapping_add(amount);
                m.w32(slot, v as u32);
            }
            let cur = m.r32(stock_at) as i32;
            m.w32(stock_at, cur.wrapping_sub(amount) as u32);
            let income = (m.r32(op + 8) as i32).wrapping_mul(price);
            let (gold, earned) = ((m.r32(merchant) as i32).wrapping_add(income), (m.r32(merchant + 0x49c) as i32).wrapping_add(income));
            m.w32(merchant, gold as u32);
            m.w32(merchant + 0x49c, earned as u32);
        }
        if !taken_all {
            let left = (m.r32(op + 8) as i32).wrapping_sub(wanted);
            m.w32(op + 8, left as u32);
        }
        if ship != 0 {
            host.call(m, 0x51_82b0, ship, &[]);
            let convoy = m.r16(ship + 8);
            if convoy >= m.r16(0x006d_d896) {
                m.w16(op + 0x10, 0xffff);
            } else {
                let next = if first {
                    first = false;
                    let table = m.r32(0x006d_d7a8);
                    m.r16(table.wrapping_add(u32::from(convoy) * 0x3c + 0xa))
                } else {
                    m.r16(ship + 6)
                };
                m.w16(op + 0x10, next);
                if next >= m.r16(SHIP_COUNT) {
                    return;
                }
                // Siguiente barco del mismo comerciante en la lista del convoy.
                loop {
                    let at = m.r32(SHIP_TABLE_PTR).wrapping_add(u32::from(m.r16(op + 0x10)).wrapping_mul(0x180));
                    if m.r8(at) == m.r8(op + 0xd) {
                        break;
                    }
                    let following = m.r16(at + 6);
                    m.w16(op + 0x10, following);
                    if following >= m.r16(SHIP_COUNT) {
                        return;
                    }
                    guard += 1;
                    if guard > 0x1_0000 {
                        return;
                    }
                }
            }
        }
        if m.r16(op + 0x10) >= m.r16(SHIP_COUNT) {
            return;
        }
        guard += 1;
        if guard > 0x1_0000 {
            return;
        }
    }
}

/// Tablas de precio del armero (`0x00673780..0x6737A0`): base de venta, factor, suelo de venta y las tres de compra (`0x0067378C`, `0x00673794`, `0x0067379C`).
const WEAPON_SELL_BASE: [i32; 4] = [180, 144, 216, 252];
const WEAPON_SELL_FACTOR: [i32; 4] = [80, 64, 96, 112];
const WEAPON_SELL_FLOOR: [i32; 4] = [60, 48, 72, 84];
const WEAPON_BUY_BASE: [i32; 4] = [350, 280, 420, 490];
const WEAPON_BUY_FACTOR: [i32; 4] = [250, 200, 300, 350];
const WEAPON_BUY_FLOOR: [i32; 4] = [70, 56, 84, 98];

/// `x / 10` con el truco de `0x66666667` (división con signo truncada).
fn div10(x: i32) -> i32 {
    x / 10
}

/// Escala `v` por la población/tamaño de la ciudad: `v / 50` si `size < 500` (`0x51EB851F`), si no `v * 10 / size`.
fn scaled_by_town(v: i32, size: i32) -> i32 {
    if size < 500 { v / 50 } else { v.wrapping_mul(10).wrapping_div(size) }
}

/// `0x005263D0` (`Town::weapon_buy_price(i, &units)`): precio por unidad al que el armero compra `*units` unidades del tipo `i & 3`; `0` si sobran
/// menos de 10 en la ciudad.
pub fn weapon_buy_price(m: &mut dyn Mem, town: u32, i: u32, ptr: u32) {
    let k = (i & 3) as usize;
    let stock = m.r32(town + 0x54 + 4 * k as u32) as i32;
    if stock < 10 {
        m.w32(ptr, 0);
        return;
    }
    let units = m.r32(ptr) as i32;
    let used = if units > stock { stock } else { units };
    let value = (stock - (used >> 1)).wrapping_mul(WEAPON_BUY_FACTOR[k]);
    let price = WEAPON_BUY_BASE[k] - scaled_by_town(value, m.r32(town + 0x2d4) as i32);
    m.w32(ptr, price.max(WEAPON_BUY_FLOOR[k]) as u32);
}

/// `0x00526470` (`Town::weapon_prices(kind, stock10, sell, buy, held)`): para las cuatro clases de arma, cuántas decenas hay (`stock10`), el precio de
/// venta `sell` (de `held * 10` unidades) y el precio de compra `buy` de la cantidad que selecciona `kind` (0: 10, 1: 50, 2: 500, otro: toda la existencia).
pub fn weapon_prices(m: &mut dyn Mem, town: u32, kind: u32, stock10: u32, sell: u32, buy: u32, held: u32) {
    for i in 0..4u32 {
        let stock = m.r32(town + 0x54 + 4 * i) as i32;
        let (units_buy, units_sell) = match kind {
            0 => (10, 10),
            1 => (0x32, 0x32),
            2 => (0x1f4, 0x1f4),
            _ => (div10(stock).wrapping_mul(10), (m.r32(held + 4 * i) as i32).wrapping_mul(10)),
        };
        m.w32(buy + 4 * i, units_buy as u32);
        m.w32(sell + 4 * i, units_sell as u32);
        m.w32(stock10 + 4 * i, div10(stock) as u32);
        weapon_buy_price(m, town, i, buy + 4 * i);
        let a = m.r32(sell + 4 * i) as i32;
        let out = if a > 0 {
            let k = (i & 3) as usize;
            let value = (stock + (a >> 1)).wrapping_mul(WEAPON_SELL_FACTOR[k]);
            let price = WEAPON_SELL_BASE[k] - scaled_by_town(value, m.r32(town + 0x2d4) as i32);
            price.max(WEAPON_SELL_FLOOR[k])
        } else {
            0
        };
        m.w32(sell + 4 * i, out as u32);
    }
}

/// `0x0052A9E0` sobre memoria plana: coste de reparar el barco en `ship` en la ciudad `town` (la fórmula de `ShipManager::calc_ship_repair_cost`).
pub fn repair_cost_flat(m: &mut dyn Mem, town: u32, ship: u32) -> i32 {
    let bytes: Vec<u8> = (0..0x9f8).map(|i| m.r8(town + i)).collect();
    let image: Vec<u8> = (0..0x180).map(|i| m.r8(ship + i)).collect();
    let manager = crate::shipyard::ShipManager { ships: image, entries: Vec::new(), docked_head: 0xffff, free_entry_head: 0xffff, ship_count: 1, entry_count: 0 };
    manager.calc_ship_repair_cost(&crate::town::Town::from_bytes(&bytes), 0)
}

/// `0x00539990` (operación 0x1D): el convoy `op+4` entra en reparación (`+0x12 = 4`, solo si aún no estaba en 4 o más); su dueño paga lo que cuesta
/// reparar todos los barcos del convoy (`0x0052A950`), se anota en `Merchant+0x4AC` y la obra del casco suma reputación social en la ciudad.
/// El original arma además una carta local que nunca envía; no tiene efecto. Con una cadena de barcos vacía o `+0x14` a cero el original divide
/// por cero: aquí se sale sin cobrar.
pub fn convoy_repair_pay(m: &mut dyn Mem, host: &mut dyn OpsHost, op: u32) {
    let convoy = m.r32(op + 4);
    if convoy >= u32::from(m.r16(0x006d_d896)) {
        return;
    }
    let c = m.r32(0x006d_d7a8).wrapping_add(convoy.wrapping_mul(0x3c));
    let town = u32::from(m.r16(c + 0x1c));
    if town >= m.r32(G_TOWNS) || m.r16(c + 0x12) >= 4 {
        return;
    }
    m.w16(c + 0x12, 4);
    let count = m.r16(SHIP_COUNT);
    let mut at = m.r16(c + 0xa);
    let first = m.r32(SHIP_TABLE_PTR).wrapping_add(u32::from(at).wrapping_mul(0x180));
    let owner = u32::from(m.r8(first));
    let merchant = host.call(m, 0x53_03c0, WORLD, &[owner]);
    let town_at = m.r32(G_TOWN_TABLE).wrapping_add(town.wrapping_mul(0x9f8));
    let cost = crate::economy::small_fns::town_calc_ship_repair_cost_for_convoy(m, host, town_at, &[first]) as i32;
    let mut hull = 0i32;
    let mut guard = 0u32;
    let mut ships = 0u32;
    while at < count {
        let ship = m.r32(SHIP_TABLE_PTR).wrapping_add(u32::from(at).wrapping_mul(0x180));
        if m.r32(ship + 0x14) == 0 {
            return;
        }
        hull = hull.wrapping_add((m.r32(ship + 0x14) as i32).wrapping_sub(m.r32(ship + 0x18) as i32));
        ships += 1;
        at = m.r16(ship + 6);
        guard += 1;
        if guard > 0x1_0000 {
            return;
        }
    }
    if ships == 0 {
        return;
    }
    let gold = (m.r32(merchant) as i32).wrapping_sub(cost);
    m.w32(merchant, gold as u32);
    let spent = (m.r32(merchant + 0x4ac) as i32).wrapping_add(cost);
    m.w32(merchant + 0x4ac, spent as u32);
    let town_word = u32::from(m.r16(c + 0x1c));
    host.call(m, 0x4f_9120, merchant, &[town_word, hull as u32]);
}

/// Mercancías que consume un barco nuevo (`0x52A360`/`0x52B2C0`): `0xB`, `8`, `0xC`, `0x11` y `0xF`.
const NEW_SHIP_WARES: [u32; 5] = [0xb, 8, 0xc, 0x11, 0xf];
/// Filas de materiales por clase (0..3) y nivel (0..2): 8 enteros por fila (`0x0066DEB0`, `0x0066DF10`, `0x0066DF70`, `0x0066DFD0`).
const NEW_SHIP_ROWS: [[[i32; 8]; 3]; 4] = [
    [[7, 3, 3, 3, 20, 17, 7650, 11414], [9, 3, 3, 3, 20, 20, 8200, 12074], [11, 3, 3, 3, 20, 24, 8800, 12784]],
    [[12, 5, 5, 5, 30, 29, 18270, 24450], [14, 5, 5, 5, 30, 32, 18720, 25010], [16, 5, 5, 5, 30, 34, 19890, 26290]],
    [[18, 3, 4, 4, 40, 46, 16560, 22296], [20, 3, 4, 4, 40, 50, 16500, 22346], [22, 3, 4, 4, 40, 53, 17490, 23446]],
    [[30, 10, 10, 8, 50, 58, 22968, 34442], [33, 10, 10, 8, 50, 64, 23040, 34679], [36, 10, 10, 8, 50, 69, 24840, 36644]],
];
/// Mercancías de unidad pequeña (`0x00672C14`).
const SMALL_UNIT_WARES: [u8; 24] = [0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1];
const NEW_SHIP_LEVEL_BASE: [u8; 16] = [15, 19, 23, 25, 28, 31, 34, 35, 45, 48, 52, 55, 55, 59, 65, 70];
const NEW_SHIP_LEVEL_CAP: [u8; 16] = [25, 28, 31, 34, 35, 45, 48, 52, 55, 55, 59, 65, 70, 0, 0, 80];
const NEW_SHIP_PAIRS: [u8; 24] = [0, 0, 4, 0, 4, 2, 0, 0, 4, 0, 2, 4, 0, 4, 0, 6, 2, 8, 0, 6, 2, 8, 0, 12];

/// `0x0052B2C0` (`Town::quote_new_ship(out, kind, owner)`): presupuesto de mandar construir un barco de clase `kind` (0..3) para `owner`.
/// Escribe en `out`: `+0..0x14` existencias en unidades de cada una de las cinco mercancías, `+0x14` lo que se necesita, `+0x28` lo que falta (limitado por
/// la ciudad), `+0x3C` su precio, `+0x50` coste total, `+0x54` días, `+0x58` nivel de obra, `+0x5C/0x5D` par de tabla y `+0x5E` nivel. Devuelve el byte de AL.
pub fn quote_new_ship(m: &mut dyn Mem, host: &mut dyn OpsHost, town: u32, out: u32, kind_arg: u32, owner_arg: u32) -> u8 {
    if (owner_arg as u16) >= m.r16(0x006d_e4aa) || (kind_arg as u8) >= 4 {
        return 0;
    }
    let kind = u32::from(kind_arg as u8);
    let level_raw = m.r8(town + 0x824 + kind) as i8;
    if level_raw < 0 || m.r32(town + 0x76c) & 0x20000 == 0 {
        return 0;
    }
    if m.r8(town + 0x2c8) & 1 != 0 && kind_arg & 2 == 2 {
        return 0;
    }
    let row = NEW_SHIP_ROWS[kind as usize][(level_raw as usize).min(2)];
    let town_index = u32::from(m.r8(town + 0x2c0));
    let mut ok = true;
    let office = host.call(m, 0x53_08a0, WORLD, &[owner_arg, 0x0066_0000 | town_index, 0]);
    for k in 0..5u32 {
        let v = if office != 0 { m.r32(office + NEW_SHIP_WARES[k as usize] * 4 + 4) } else { 0 };
        m.w32(out + 0x14 + 4 * k, v);
    }
    let owner16 = owner_arg & 0xffff;
    let merchant = host.call(m, 0x53_03c0, WORLD, &[owner16]);
    let count = u32::from(m.r16(SHIP_COUNT));
    let mut at = u32::from(m.r16(merchant + 0xe));
    let mut guard = 0u32;
    while at < count {
        let ship = m.r32(SHIP_TABLE_PTR).wrapping_add(at.wrapping_mul(0x180));
        if u32::from(m.r8(ship + 0x39)) == town_index && m.r16(ship + 0x134) < 4 && m.r8(ship + 0x136) & 1 == 0 {
            for k in 0..5u32 {
                let add = m.r32(ship + NEW_SHIP_WARES[k as usize] * 4 + 0x54);
                let v = m.r32(out + 0x14 + 4 * k).wrapping_add(add);
                m.w32(out + 0x14 + 4 * k, v);
            }
        }
        at = u32::from(m.r16(ship + 4));
        guard += 1;
        if guard > 0x1_0000 {
            return 0;
        }
    }
    for k in 0..5u32 {
        let ware = NEW_SHIP_WARES[k as usize];
        let unit: i32 = if SMALL_UNIT_WARES[ware as usize] != 0 { 200 } else { 2000 };
        let stock = m.r32(out + 0x14 + 4 * k) as i32;
        let avail = stock.wrapping_div(unit);
        m.w32(out + 0x14 + 4 * k, avail as u32);
        m.w32(out + 4 * k, avail as u32);
        let need = row[k as usize];
        if avail >= need {
            m.w32(out + 0x14 + 4 * k, need as u32);
            m.w32(out + 0x3c + 4 * k, 0);
            m.w32(out + 0x28 + 4 * k, 0);
        } else {
            let missing = need - avail;
            m.w32(out + 0x28 + 4 * k, missing as u32);
            let price = host.call(m, 0x52_e430, 0x006d_e3d8, &[ware, town_index, missing.wrapping_mul(unit) as u32]);
            m.w32(out + 0x3c + 4 * k, price);
            let in_town = (m.r32(town + ware * 4 + 4) as i32).wrapping_div(unit);
            if missing > in_town {
                ok = false;
                m.w32(out + 0x28 + 4 * k, in_town as u32);
            }
        }
    }
    let level = i32::from(level_raw);
    let base = i32::from(NEW_SHIP_LEVEL_BASE[((level * 1 + kind as i32 * 4) as usize) & 15]) + i32::from(m.r8(town + 0x828 + kind));
    let cap = i32::from(NEW_SHIP_LEVEL_CAP[(kind * 4) as usize]);
    let work = base.min(cap);
    m.w32(out + 0x58, work as u32);
    let speed = i32::from(m.r16(town + 0x854));
    m.w32(out + 0x54, work.wrapping_mul(28).wrapping_div(speed).wrapping_add(0x1c) as u32);
    let days_base: i32 = if work > 0x28 {
        if work > 0x32 { if work > 0x3c { 0x12c } else { 0x14a } } else { 0x168 }
    } else if work > 0x1e {
        0x186
    } else if work > 0x14 {
        0x1a4
    } else {
        0x1c2
    };
    let mut cost = (days_base * 2).wrapping_add(row[6]);
    cost = cost.wrapping_add(crate::numeric::ftol_i32(f64::from(f32::from_bits(m.r32(town + 0x818)))));
    for k in 0..5u32 {
        cost = cost.wrapping_add(m.r32(out + 0x3c + 4 * k) as i32);
    }
    m.w32(out + 0x50, cost as u32);
    let lv = m.r8(town + 0x824 + kind);
    m.w8(out + 0x5e, lv.min(2));
    m.w8(out + 0x5c, NEW_SHIP_PAIRS[(kind * 6 + 1) as usize]);
    m.w8(out + 0x5d, NEW_SHIP_PAIRS[(kind * 6) as usize]);
    let merchant = host.call(m, 0x53_03c0, WORLD, &[owner16]);
    if (m.r32(merchant) as i32) < cost && m.r16(merchant + 8) == 0 {
        return 0;
    }
    u8::from(ok)
}

/// `0x0053C050` (operación 0x51): entrega mercancía a un contrato de suministro de la taberna (tarea 0x11 `op+0xC`) del comerciante `op+4`.
/// Descuenta `op+0x10` unidades (acotadas a lo que falta) primero de los barcos del comerciante atracados en la ciudad del contrato y, agotados
/// estos, de su oficina; cobra `precio × unidades` (salvo marca `+0x17`) y, al terminar, libera la ranura del contrato en la ciudad.
pub fn deliver_supply_contract(m: &mut dyn Mem, host: &mut dyn OpsHost, op: u32) {
    const SCHED_COUNT: u32 = 0x006d_d748;
    let who = m.r32(op + 4);
    if who >= u32::from(m.r16(G_MERCHANTS)) || m.r32(op + 0xc) >= u32::from(m.r16(SCHED_COUNT)) {
        return;
    }
    let merchant = host.call(m, 0x53_03c0, WORLD, &[who]);
    let index = m.r32(op + 0xc);
    let task = host.call(m, 0x4d_8db0, SCHED, &[index]);
    if m.r16(task + 6) != 0x11 || u32::from(m.r8(task + 8)) >= m.r32(G_TOWNS) || m.r8(task + 9) >= 0x14 || u32::from(m.r8(task + 0x16)) != who {
        return;
    }
    let left = u32::from(m.r16(task + 0x10));
    let requested = m.r32(op + 0x10);
    if m.r32(op + 0x10) > left {
        m.w32(op + 0x10, left);
    }
    let ware = u32::from(m.r8(task + 9));
    let small = crate::shipyard::WARE_SMALL_UNIT[ware as usize] != 0;
    let mut ship_id = u32::from(m.r16(merchant + 0xe));
    // El original pasa a `0x5308A0` el comerciante en CX y conserva en ECX los 16 bits altos de lo último que hubiera allí: la cantidad pedida
    // antes de acotar, y luego el siguiente barco, la mercancía o el puntero del barco servido.
    let mut ecx = requested;
    while m.r32(op + 0x10) != 0 {
        let (stock_at, ship) = if ship_id < u32::from(m.r16(SHIP_COUNT)) {
            let ship = m.r32(SHIP_TABLE_PTR).wrapping_add(ship_id.wrapping_mul(0x180));
            let next = u32::from(m.r16(ship + 4));
            let at = ship + 0x54 + ware * 4;
            if m.r8(ship + 0x39) != m.r8(task + 8) {
                ecx = next;
                ship_id = next;
                continue;
            }
            if (m.r32(at) as i32) <= 0 {
                ecx = ware;
                ship_id = next;
                continue;
            }
            ship_id = next;
            (at, ship)
        } else {
            let town_of = u32::from(m.r8(task + 8));
            let office = host.call(m, 0x53_08a0, WORLD, &[(ecx & 0xffff_0000) | (who & 0xffff), town_of, 0]);
            if office == 0 {
                return;
            }
            (office + ware * 4 + 4, 0)
        };
        let mut amount = m.r32(op + 0x10);
        let unit: i32 = if small { 200 } else { 2000 };
        let stock = m.r32(stock_at) as i32;
        if stock < (amount as i32).wrapping_mul(unit) {
            amount = (stock / unit) as u32;
        }
        m.w32(stock_at, stock.wrapping_sub((amount as i32).wrapping_mul(unit)) as u32);
        let rest = m.r16(task + 0x10).wrapping_sub(amount as u16);
        m.w16(task + 0x10, rest);
        let wanted = m.r32(op + 0x10).wrapping_sub(amount);
        m.w32(op + 0x10, wanted);
        if m.r8(task + 0x17) == 0 {
            let pay = u32::from(m.r16(task + 0x14)).wrapping_mul(amount);
            let (gold, income) = (m.r32(merchant).wrapping_add(pay), m.r32(merchant + 0x49c).wrapping_add(pay));
            m.w32(merchant, gold);
            m.w32(merchant + 0x49c, income);
        }
        if ship != 0 {
            ecx = ship;
            host.call(m, 0x51_82b0, ship, &[]);
            host.call(m, 0x51_7770, ship, &[4]);
        } else {
            m.w32(op + 0x10, 0);
        }
    }
    if m.r16(task + 0x10) > 0 {
        return;
    }
    let town = m.r32(G_TOWN_TABLE).wrapping_add(u32::from(m.r8(task + 8)).wrapping_mul(0x9f8));
    for slot in [0x830, 0x832] {
        if u32::from(m.r16(town + slot)) == m.r32(op + 0xc) {
            m.w16(town + slot, 0xffff);
        }
    }
}

/// `0x0053D190`: busca en las tres listas de edificios de la ciudad `op+4` del comerciante `op+8` el de tipo `op+0xC` y nivel `op+0x10`, y lo
/// pone a subasta en el planificador.
pub fn put_building_up_for_auction(m: &mut dyn Mem, host: &mut dyn OpsHost, op: u32) {
    let town_idx = m.r32(op + 4);
    if town_idx >= m.r32(G_TOWNS) {
        return;
    }
    let merchant = m.r32(op + 8);
    if merchant >= u32::from(m.r16(G_MERCHANTS)) {
        return;
    }
    let town = m.r32(G_TOWN_TABLE).wrapping_add(town_idx.wrapping_mul(0x9f8));
    let obj = host.call(m, 0x5308a0, WORLD, &[merchant & 0xffff, town_idx & 0xffff, 0]);
    if obj == 0 {
        return;
    }
    let a = m.r32(op + 0xc);
    let b = m.r32(op + 0x10);
    if m.r8(town + 0x2c8) & 0x10 != 0 {
        let base = m.r32(town + 0x7ac).wrapping_mul(a).wrapping_add(m.r32(town + 0x80c));
        if m.r8(base.wrapping_add(b)) & 0x80 == 0 {
            return;
        }
    }
    let mut at = obj + 0x2d0;
    let mut stage = 0;
    loop {
        let idx = m.r16(at);
        if idx >= m.r16(town + 0x776) {
            match stage {
                0 => {
                    at = obj + 0x2ce;
                    stage = 1;
                }
                1 => {
                    at = town + 0x762;
                    stage = 2;
                }
                _ => {
                    host.call(m, 0x4ff2d0, obj, &[]);
                    return;
                }
            }
            continue;
        }
        let rec = host.call(m, 0x51f1f0, town, &[u32::from(idx)]);
        if u32::from(m.r8(rec)) == a && u32::from(m.r8(rec + 1)) == b {
            // Los argumentos conservan los bits altos de ECX (`b`) y de EAX (la dirección del registro), como el original.
            let kind = (b & 0xffff_ff00) | u32::from(m.r8(op + 4));
            let at_idx = (rec & 0xffff_0000) | u32::from(m.r16(at));
            host.call(m, 0x4dab00, SCHED, &[kind, at_idx]);
            return;
        }
        at = rec + 4;
    }
}

const G_TICK: u32 = 0x006d_e4b4;
const SCHED_COUNT: u32 = 0x006d_d748;
const G_HANSE_MERCHANT: u32 = 0x006d_e52d;
const G_HANSE_TOWN: u32 = 0x006d_e52e;

/// `0x0053B1F0`: decisión sobre la propuesta del día hanseático de la tarea `op+8`. `code` = `op+0xC` (6 = rechazar). Si no se rechaza, programa la
/// tarea 0x21 con los datos de la propuesta y publica el mensaje (16 bytes) por `0x4D6530`; los buffers que rellena `0x5311D0` quedan a cero.
pub fn decide_hanse_day_proposal(m: &mut dyn Mem, host: &mut dyn OpsHost, op: u32) {
    let id = m.r32(op + 8);
    if id >= u32::from(m.r16(SCHED_COUNT)) {
        return;
    }
    let rec = m.r32(SCHED).wrapping_add(id.wrapping_mul(0x18));
    if m.r16(rec + 6) != 0x30 {
        return;
    }
    let code = m.r32(op + 0xc);
    if code == 6 {
        if m.r8(rec + 0xc) != 6 && m.r8(rec + 0x13) != 5 && m.r8(rec + 0x13) != 6 {
            let who = m.r8(G_HANSE_MERCHANT);
            if u32::from(who) < u32::from(m.r16(G_MERCHANTS)) {
                let obj = host.call(m, 0x5303c0, WORLD, &[u32::from(who)]);
                if m.r16(obj + 8) == 0 {
                    host.call(m, 0x4f91c0, obj, &[]);
                }
            }
        }
        m.w32(rec + 8, 0);
        return;
    }
    let tick = m.r32(G_TICK);
    if m.r32(op + 4) < tick.wrapping_add(0x700) {
        m.w32(op + 4, tick.wrapping_add(0x700));
    }
    let due = m.r32(op + 4) & !0xff;
    let new = host.call(m, 0x4d8cf0, SCHED, &[due]);
    m.w16(new + 6, 0x21);
    let (merchant, town_b) = (m.r8(G_HANSE_MERCHANT), m.r8(G_HANSE_TOWN));
    m.w8(new + 8, town_b);
    let (r12, r13, rc, r14, r15) = (m.r8(rec + 0x12), m.r8(rec + 0x13), m.r8(rec + 0xc), m.r8(rec + 0x14), m.r8(rec + 0x15));
    m.w8(new + 9, r12);
    m.w8(new + 0xa, r13);
    m.w8(new + 0xb, rc);
    m.w8(new + 0xc, r14);
    m.w8(new + 0xd, r15);
    m.w8(new + 0xe, code as u8);
    for k in 0xf..=0x16 {
        m.w8(new + k, 0);
    }
    m.w32(rec + 8, 0);
    host.call(m, 0x51a850, 0, &[]);
    let mut msg = [0u8; 16];
    msg[4] = 0x78;
    msg[5] = town_b;
    if u32::from(merchant) < u32::from(m.r16(G_MERCHANTS)) {
        let obj = host.call(m, 0x5303c0, WORLD, &[u32::from(merchant)]);
        msg[0xe] = m.r8(obj + 0x1c);
        msg[0xf] = m.r8(obj + 0x1d);
        msg[0xd] = host.call(m, 0x4f9430, obj, &[]) as u8;
    } else {
        let town = m.r32(G_TOWN_TABLE).wrapping_add(u32::from(town_b) * 0x9f8);
        let sel = 0xffu8.wrapping_sub(m.r8(town + 0x6f1)) & 3;
        let at = town.wrapping_add(u32::from(sel));
        msg[0xe] = m.r8(at + 0x6d4);
        msg[0xf] = m.r8(at + 0x6d8);
        msg[0xd] = m.r8(at + 0x6e0);
    }
    let due_now = m.r32(new);
    host.call(m, 0x5311d0, WORLD, &[due_now]);
    // `ticks_to_date(vencimiento, &carta[8], &carta[9], &año)`: día y mes de la votación en la carta 0x78.
    let (day, month, _) = crate::economy::tavern_passage::date_from_tick(due_now as i32);
    msg[8] = day;
    msg[9] = month;
    let r9 = m.r8(new + 9);
    msg[0xa] = r9;
    let (re, ra) = (m.r8(new + 0xe), m.r8(new + 0xa));
    msg[0xb] = (re << 4).wrapping_add(ra);
    if ra == 0 {
        msg[0xc] = m.r8(new + 0xc);
    }
    let mut args = vec![0xffff_ffffu32];
    args.extend(msg.iter().map(|b| u32::from(*b)));
    host.call(m, 0x4d6530, QUEUE, &args);
    host.call(m, 0x51a880, 0, &[]);
}

const G_POOL: u32 = 0x006d_d8b4;
const G_HOOK_FLAG: u32 = 0x006e_5558;

fn head_due_of(m: &mut dyn Mem, t: u32) -> u32 {
    let table = m.r32(t);
    let head = u32::from(m.r16(t + 8));
    m.r32(table.wrapping_add(head * 0x18))
}

fn copy(m: &mut dyn Mem, dst: u32, src: u32, n: u32) {
    for k in 0..n {
        let b = m.r8(src.wrapping_add(k));
        m.w8(dst.wrapping_add(k), b);
    }
}

/// `0x004D81E0` (`scheduled_tasks_load`): carga el planificador `t` desde el flujo `stream` (`aux` se pasa tal cual a `0x4F3A40`). Devuelve el cursor final.
/// Anfitrión: `0x64F7B9`/`0x64F7E2` (montón), `0x511000`, `0x511270`, `0x511690`, `0x4F3980`, `0x4F3A40`, `0x5C6C80`, `0x4D8DD0` y `0x5311D0`
/// (que recibe los destinos `rec+0xF`, `rec+0x10` y un tercero que aquí no se emula: 0).
pub fn scheduled_tasks_load(m: &mut dyn Mem, host: &mut dyn OpsHost, t: u32, stream: u32, aux: u32) -> u32 {
    let mut cur = stream;
    for off in [8u32, 0xa, 0xc] {
        copy(m, t + off, cur, 2);
        cur = cur.wrapping_add(2);
    }
    let old = m.r32(t);
    if old != 0 {
        host.call(m, 0x64f7e2, 0, &[old]);
    }
    let n = u32::from(m.r16(t + 0xc));
    if n == 0 {
        m.w32(t, 0);
    } else {
        let table = host.call(m, 0x64f7b9, 0, &[n * 0x18]);
        m.w32(t, table);
        copy(m, table, cur, n * 0x18);
        cur = cur.wrapping_add(n * 0x18);
        if m.r32(G_POOL) == 0 {
            let obj = host.call(m, 0x64f7b9, 0, &[0xc]);
            let pool = if obj != 0 { host.call(m, 0x511000, obj, &[]) } else { 0 };
            m.w32(G_POOL, pool);
        }
        let pool = m.r32(G_POOL);
        host.call(m, 0x511270, pool, &[0]);
        let mut i = 0u32;
        loop {
            let rec = m.r32(t).wrapping_add(i * 0x18);
            let kind = m.r16(rec + 6);
            if kind == 0x1b {
                // `mov ax,[..]` conserva los 16 bits altos de EAX (la dirección del registro).
                let id = (rec & 0xffff_0000) | u32::from(m.r16(rec + 0x10));
                let pool = m.r32(G_POOL);
                let p = host.call(m, 0x511690, pool, &[id]);
                m.w32(rec + 8, p);
                let cnt = u32::from(m.r16(rec + 0x12));
                if cnt == 0 {
                    m.w32(rec + 0xc, 0);
                } else {
                    let blob = host.call(m, 0x64f7b9, 0, &[cnt * 16]);
                    m.w32(rec + 0xc, blob);
                    copy(m, blob, cur, cnt * 4);
                    cur = cur.wrapping_add(cnt * 4);
                }
            } else if kind == 0x34 && m.r8(rec + 0x15) >= 7 && m.r32(rec + 8) != 0 {
                let obj = host.call(m, 0x64f7b9, 0, &[8]);
                let obj = if obj != 0 { host.call(m, 0x4f3980, obj, &[]) } else { 0 };
                m.w32(rec + 8, obj);
                cur = host.call(m, 0x4f3a40, obj, &[cur, aux]);
            }
            i += 1;
            if i >= u32::from(m.r16(t + 0xc)) {
                break;
            }
        }
    }
    let flag = m.r32(G_HOOK_FLAG);
    if flag != 0 {
        host.call(m, 0x5c6c80, flag, &[]);
    }
    // Segunda fase: reajusta los registros de tipo 2 con fecha >= 0x42D56000, moviéndolos a la cabeza.
    let mut link = t + 8;
    if m.r16(link) >= m.r16(t + 0xc) {
        return cur;
    }
    loop {
        let idx = u32::from(m.r16(link));
        let rec = m.r32(t).wrapping_add(idx * 0x18);
        if m.r16(rec + 6) != 2 || m.r32(rec) < 0x42d5_6000 {
            link = rec + 4;
        } else {
            let next = m.r16(rec + 4);
            m.w16(link, next);
            let rem = u32::from(m.r16(rec + 8)) % 365;
            m.w16(rec + 8, rem as u16);
            let head = u32::from(m.r16(t + 8));
            let _ = head;
            let head_due = head_due_of(m, t);
            let q = head_due / 0x16d00;
            let mut due = (q * 365 + rem).wrapping_mul(256).wrapping_add(0xfa);
            m.w32(rec, due);
            let mut hd = head_due_of(m, t);
            while due <= hd {
                due = due.wrapping_add(0x16d00);
                m.w32(rec, due);
                hd = head_due_of(m, t);
            }
            host.call(m, 0x5311d0, WORLD, &[due, rec + 0xf, rec + 0x10, 0]);
            let h = m.r16(t + 8);
            m.w16(rec + 4, h);
            m.w16(t + 8, idx as u16);
            host.call(m, 0x4d8dd0, t, &[]);
        }
        if m.r16(link) >= m.r16(t + 0xc) {
            break;
        }
    }
    cur
}

const G_MERCHANT_TABLE: u32 = 0x006d_e518;

/// `0x005319D0` (`grant_first_discovery_renown`): concede la fama del primer descubrimiento al comerciante cuyo índice apunta `merchant_ptr`,
/// una sola vez por objeto (`this+0x89`). El diálogo que muestra para comerciantes válidos (0x548CA0, 0x469380, 0x64F2C1, 0x469AF0) es presentación y
/// no se modela. El mensaje 0x56 (22 bytes) conserva en `+0x14` los dos bytes bajos del manejador SEH (`0x3486`), residuo de pila del original.
pub fn grant_first_discovery_renown(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, merchant_ptr: u32) {
    if m.r8(this + 0x89) != 0 {
        return;
    }
    let merchant = u32::from(m.r8(merchant_ptr));
    let rec = m.r32(G_MERCHANT_TABLE).wrapping_add(merchant * 0x650);
    host.call(m, 0x4f8b50, rec, &[]);
    m.w8(this + 0x89, 1);
    host.call(m, 0x51a850, 0, &[]);
    let faction = m.r8(rec + 0x19);
    let al = host.call(m, 0x4f9430, rec, &[]) as u8;
    let mut msg = [0u8; 0x16];
    msg[4] = 0x56;
    msg[5] = faction;
    msg[8] = merchant as u8;
    msg[0xa] = al;
    let ax = u16::from(m.r8(rec + u32::from(faction) + 0x39c));
    msg[0xc] = ax as u8;
    msg[0xd] = (ax >> 8) as u8;
    msg[0x14] = 0x86;
    msg[0x15] = 0x34;
    let mut args = vec![0xffff_fffeu32];
    args.extend(msg.iter().map(|b| u32::from(*b)));
    host.call(m, 0x4d6530, QUEUE, &args);
    host.call(m, 0x51a880, 0, &[]);
}

/// `0x0052B110` (`Town::cancel_ship_order(owner, ship)`, operación 0x2F): saca de la cola del astillero de la ciudad un barco encargado y avisa al dueño
/// con la carta 0x14. Si la obra ya empezó (`+0x18 > 0`) solo envía la carta (con `-1` en `+8`); si no, devuelve el 90 % del coste (`+0x48 · 9 / 10`), lo resta
/// de `Merchant+0x4C4` y retira el barco con `ShipManager::remove_ship` (`0x00507FB0`). La carta se publica con `add_message` en la forma de 17 valores
/// (destinatario y los 16 bytes del registro); sus bytes `+6/+7` son basura de pila en el original y aquí valen 0.
pub fn cancel_ship_order(m: &mut dyn Mem, host: &mut dyn OpsHost, town: u32, owner: u32, ship_arg: u32) {
    let count = u32::from(m.r16(SHIP_COUNT));
    if ship_arg >= count {
        return;
    }
    let table = m.r32(SHIP_TABLE_PTR);
    let mut at = u32::from(m.r16(town + 0x81c));
    let mut guard = 0u32;
    while at != ship_arg {
        if at >= count {
            return;
        }
        at = u32::from(m.r16(table.wrapping_add(at.wrapping_mul(0x180)).wrapping_add(6)));
        guard += 1;
        if guard > 0x1_0000 {
            return;
        }
    }
    let ship = table.wrapping_add(ship_arg.wrapping_mul(0x180));
    if owner >= u32::from(m.r16(G_MERCHANTS)) || owner != u32::from(m.r8(ship)) {
        return;
    }
    let merchant = host.call(m, 0x53_03c0, WORLD, &[owner]);
    let tick = m.r32(ship + 0x4c) as i32;
    let (day, month, year) = crate::economy::tavern_passage::date_from_tick(tick);
    let mut rec = [0u8; 16];
    rec[1] = 0x80;
    rec[2] = 0xff;
    rec[3] = 0xff;
    rec[4] = 0x14;
    rec[5] = m.r8(town + 0x2c0);
    rec[0xc] = owner as u8;
    rec[0xd] = (day << 2) | (m.r8(ship + 0xe) & 3);
    let date = (u32::from(year).wrapping_mul(12)).wrapping_add(u32::from(month).wrapping_sub(1)) as u16;
    rec[0xe..0x10].copy_from_slice(&date.to_le_bytes());
    let started = (m.r32(ship + 0x18) as i32) > 0;
    if started {
        rec[8..12].copy_from_slice(&0xffff_ffffu32.to_le_bytes());
    } else {
        let refund = (m.r32(ship + 0x48) as i32).wrapping_mul(9).wrapping_div(10);
        rec[8..12].copy_from_slice(&(refund as u32).to_le_bytes());
        let gold = m.r32(merchant).wrapping_add(refund as u32);
        m.w32(merchant, gold);
        let debt = m.r32(merchant + 0x4c4).wrapping_sub(refund as u32);
        m.w32(merchant + 0x4c4, debt);
        host.call(m, 0x50_7fb0, SHIP_MANAGER, &[ship_arg]);
    }
    let mut args = [0u32; 17];
    args[0] = owner;
    for (k, b) in rec.iter().enumerate() {
        args[1 + k] = u32::from(*b);
    }
    host.call(m, 0x4d_6530, QUEUE, &args);
}

/// Bloque `0x00535CB9` de la tabla de operaciones (operación 0x2F): `op+4` dueño, `op+8` ciudad, `op+0xC` barco → `Town::cancel_ship_order`.
pub fn cancel_ship_order_op(m: &mut dyn Mem, host: &mut dyn OpsHost, op: u32) {
    let town_idx = m.r32(op + 8);
    if town_idx >= m.r32(G_TOWNS) {
        return;
    }
    let owner = m.r32(op + 4);
    let ship = m.r32(op + 0xc);
    let town = m.r32(G_TOWN_TABLE).wrapping_add(town_idx.wrapping_mul(0x9f8));
    cancel_ship_order(m, host, town, owner, ship);
}

const REGION_OF_TOWN: u32 = 0x006d_e4b8;
const TOWN_POSITIONS: u32 = 0x006d_dbb0;
const G_MAP: u32 = 0x006c_bdc8;
const G_SETTINGS: u32 = 0x006c_c3e8;
const G_UI_A: u32 = 0x006c_e6d0;
const G_UI_B: u32 = 0x006c_c950;
const G_CAPTAIN_COUNT: u32 = 0x006d_d892;

/// `ShipManager::transfer_ship(ship, 0xFFFF, 0, 0, town)` (`0x00509250`) cuando el comprador no es un comerciante: el barco pasa a la banda pirata más
/// cercana a la ciudad (o a una nueva si no hay ninguna). Copia literal de `0x00509250..0x005097A1`; las llamadas a otras funciones van al anfitrión.
pub fn transfer_ship_to_pirates(m: &mut dyn Mem, host: &mut dyn OpsHost, ship_arg: u32, town_arg: u32) {
    let count = u32::from(m.r16(SHIP_COUNT));
    if ship_arg >= count {
        return;
    }
    let ship = m.r32(SHIP_TABLE_PTR).wrapping_add(ship_arg.wrapping_mul(0x180));
    if m.r16(ship + 0x134) >= 0x11 {
        return;
    }
    let owner = u32::from(m.r8(ship));
    let merchants = u32::from(m.r16(G_MERCHANTS));
    // Mitad alta de ECX al llamar a la interfaz A: el original solo reescribe CX (`mov cx, estado`), así que el argumento arrastra lo que ECX tuviera.
    let mut ecx_hi = m.r32(SHIP_TABLE_PTR) >> 16;
    if owner < merchants {
        let merchant = host.call(m, 0x53_03c0, WORLD, &[owner]);
        let mut at = u32::from(m.r16(merchant + 0xe));
        let mut link = merchant + 0xe;
        let mut guard = 0u32;
        if at >= count {
            ecx_hi = WORLD >> 16;
        }
        while at < count {
            if at == ship_arg {
                let next = m.r16(ship + 4);
                m.w16(link, next);
                ecx_hi = 0;
                break;
            }
            link = m.r32(SHIP_TABLE_PTR).wrapping_add(at.wrapping_mul(0x180)).wrapping_add(4);
            at = u32::from(m.r16(link));
            ecx_hi = m.r32(SHIP_TABLE_PTR) >> 16;
            guard += 1;
            if guard > 0x1_0000 {
                return;
            }
        }
    }
    let old_owner = m.r8(ship);
    m.w8(ship + 0x15d, old_owner);
    let state = u32::from(m.r16(ship + 0x134));
    let ui_a = m.r32(G_UI_A);
    host.call(m, 0x49_4990, ui_a, &[ship_arg, (ecx_hi << 16) | state]);
    let state = u32::from(m.r16(ship + 0x134));
    let ui_b = m.r32(G_UI_B);
    host.call(m, 0x47_78a0, ui_b, &[ship_arg, state]);
    let was = u32::from(m.r8(ship));
    if was < merchants {
        let merchant = host.call(m, 0x53_03c0, WORLD, &[was]);
        m.w8(ship, 0xff);
        host.call(m, 0x4f_5900, merchant, &[]);
    }
    if town_arg >= m.r32(G_TOWNS) {
        return;
    }
    let region = u32::from(m.r8(REGION_OF_TOWN + town_arg)) * 0x34;
    let ref_x = m.r32(TOWN_POSITIONS + region) as i32;
    let ref_y = m.r32(TOWN_POSITIONS + region + 4) as i32;
    let manager = SHIP_MANAGER;
    let mut best = 0x7fff_ffffi32;
    let mut index = 0xffu32;
    for k in 0..5u32 {
        let band = m.r32(manager + 0xc + 4 * k);
        if band == 0 {
            continue;
        }
        let off = u32::from(m.r8(band + 0xe)) * 0x34;
        let dx = (i32::from(m.r16(TOWN_POSITIONS + off))).wrapping_sub(ref_x);
        let dy = (i32::from(m.r16(TOWN_POSITIONS + off + 4))).wrapping_sub(ref_y);
        let d = dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx));
        if d < best {
            best = d;
            index = k;
        }
    }
    if index >= 5 {
        let raw = host.call(m, 0x64_f7b9, 0, &[0x18]);
        let band = if raw != 0 { host.call(m, 0x51_3720, raw, &[]) } else { 0 };
        m.w32(manager + 0xc, band);
        let tick = m.r32(G_TICK);
        let settings = m.r32(G_SETTINGS);
        let difficulty = u32::from(m.r8(settings + 0x13));
        host.call(m, 0x51_3d80, band, &[ship_arg % 0x28, difficulty, tick.wrapping_add(ship_arg).wrapping_add(town_arg)]);
        index = 0;
    }
    let band = m.r32(manager + 0xc + 4 * index);
    host.call(m, 0x51_5ea0, band, &[ship_arg]);
    let map = m.r32(G_MAP);
    let table = m.r32(map + 0x24);
    let cell = m.r32(table.wrapping_add(town_arg.wrapping_mul(4)));
    let packed = m.r32(cell);
    let scale = m.r32(0x0066_e0d0) as i32;
    let project = |v: i16| -> i32 {
        let t = i64::from(i32::from(v).wrapping_mul(scale).wrapping_add(0x1f4));
        (t / 1000) as i32 + 0x44
    };
    let x = project(packed as u16 as i16) as i16;
    let y = project((packed >> 16) as u16 as i16) as i16;
    host.call(m, 0x51_72a0, ship, &[0xffff, x as i32 as u32, y as i32 as u32, ship_arg]);
    let flags = m.r8(ship + 0x3d) | 4;
    m.w8(ship + 0x3d, flags);
    m.w16(ship + 0x138, 0);
    m.w8(ship, 0xff);
    m.w16(ship + 0x134, 2);
}

/// `0x0053B850` (operación 0x4E, `op_hire_captain`): el jugador contrata al capitán pirata `op+4` de la taberna para el barco `op+8`; el barco pasa a los piratas
/// (`transfer_ship_to_pirates`), recibe `min(faltan, op+0xC+1)` marineros, patrulla la ciudad `op+0x10` y queda ocupado hasta `tick + ((rand % 0xB4) + 0xB4) · 256`.
pub fn hire_pirate_captain(m: &mut dyn Mem, host: &mut dyn OpsHost, op: u32) {
    let count = u32::from(m.r16(SHIP_COUNT));
    let ship_arg = m.r32(op + 8);
    if ship_arg >= count {
        return;
    }
    let ship = m.r32(SHIP_TABLE_PTR).wrapping_add(ship_arg.wrapping_mul(0x180));
    let owner = u32::from(m.r8(ship));
    if owner >= u32::from(m.r16(G_MERCHANTS)) || m.r16(ship + 0x134) > 1 || m.r16(ship + 0x42) < m.r16(G_CAPTAIN_COUNT) {
        return;
    }
    let town = u32::from(m.r8(ship + 0x39));
    if town >= m.r32(G_TOWNS) {
        return;
    }
    let merchant = host.call(m, 0x53_03c0, WORLD, &[owner]);
    if m.r8(merchant + 8) & 4 == 4 {
        return;
    }
    let town_ptr = m.r32(G_TOWN_TABLE).wrapping_add(town.wrapping_mul(0x9f8));
    let captain = m.r32(op + 4);
    if !free_town_letter(m, town_ptr, captain) {
        return;
    }
    let cap16 = m.r16(op + 4);
    m.w16(ship + 0x42, cap16);
    let owner_now = u32::from(m.r8(ship));
    let merchant = host.call(m, 0x53_03c0, WORLD, &[owner_now]);
    let delta = i32::from(m.r16(0x0067_2dd8) as i16);
    let rep = i32::from(m.r16(merchant + 0x16) as i16);
    if delta > 0 {
        if rep < 0x7fff - delta {
            m.w16(merchant + 0x16, (rep + delta) as u16);
        } else {
            m.w16(merchant + 0x16, 0x7fff);
        }
    } else if rep + 0x8000 > -delta {
        m.w16(merchant + 0x16, (rep + delta) as u16);
    } else {
        m.w16(merchant + 0x16, 0x8000);
    }
    let town_now = u32::from(m.r8(ship + 0x39));
    transfer_ship_to_pirates(m, host, ship_arg, town_now);
    let needed = host.call(m, 0x51_84f0, ship, &[]);
    let bonus = m.r32(op + 0xc).wrapping_add(1);
    let add = if needed > bonus { bonus } else { needed };
    let crew = m.r16(ship + 0x40).wrapping_add(add as u16);
    m.w16(ship + 0x40, crew);
    host.call(m, 0x51_82b0, ship, &[]);
    let patrol = m.r32(op + 0x10);
    host.call(m, 0x51_a820, ship, &[patrol]);
    let tick = m.r32(G_TICK);
    m.w32(ship + 0x4c, tick);
    let mut rng = crate::rng::LcgRandom::new();
    rng.set_seed(tick);
    let roll = rng.next_u32() % 0xb4;
    m.w32(ship + 0x4c, tick.wrapping_add((roll + 0xb4) << 8));
}
