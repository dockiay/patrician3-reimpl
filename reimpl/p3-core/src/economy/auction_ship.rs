//! Fase 0 de la subasta de barcos (tarea `0x0A`, `st_auction` 0x004E2CD4, caso 0 en 0x004E2D1A).
//!
//! El original crea el barco subastado con `ShipManager::create_ship` (0x00507D60) en modo 2
//! (estado `0x0C`, a la cabeza de la lista `gestor+0xE8`), le da tipo y estadísticas de astillero
//! (0x0052A8D0), ranuras de arma (0x005181D0), carga (0x005182B0), un daño pseudoaleatorio
//! derivado del tick y un nombre (0x00509A50). La tarea pasa a la fase 1, vence un tick después
//! y guarda el índice del barco en `+8` (lotes `+0xA`/`+0xC` vacíos).
//!
//! La función trabaja sobre copias: si algo no cuadra devuelve error y no cambia nada.
use crate::{scheduler::TaskRecord, ships::factory, town::Town};

/// Vista mínima del gestor de barcos (`0x006DD7A0`) que la fase 0 lee y escribe.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LotFleet {
    /// Registros de 0x180 bytes (`gestor+4`); su número es `gestor+0xF4`.
    pub ships: Vec<Vec<u8>>,
    /// Cabeza de la lista libre (`gestor+0xE2`).
    pub free_ship: u16,
    /// Cabeza de la lista de barcos en puerto / en venta (`gestor+0xE8`).
    pub docked_head: u16,
    /// Cursor y paso del generador de nombres (`gestor+0xEE`, `gestor+0xF0`).
    pub name_cursor: u16,
    pub name_step: u16,
}

/// Datos de partida que no pertenecen al gestor.
pub struct LotResources<'a> {
    /// Tick global (`0x006DE4B4`).
    pub tick: u32,
    /// Coordenadas enteras del puerto de cada ciudad (`0x006DDBB0`/`0x006DDBB4` vía `0x006DE4B8`).
    pub ports: &'a [Option<(u16, u16)>],
    /// Banco de nombres de barco (`0x006DDAA0`, recuento `0x006DDB78`).
    pub names: &'a [Vec<u8>],
}

fn r16(b: &[u8], at: usize) -> u16 {
    u16::from_le_bytes([b[at], b[at + 1]])
}
fn w16(b: &mut [u8], at: usize, v: u16) {
    b[at..at + 2].copy_from_slice(&v.to_le_bytes())
}

/// Ejecuta la fase 0 sobre `task` y `fleet`. Devuelve el índice del barco creado.
pub fn create_lot_ship(
    task: &mut TaskRecord,
    towns: &[Town],
    fleet: &mut LotFleet,
    r: &LotResources<'_>,
) -> Result<u16, &'static str> {
    if task.kind != 0x0a || task.u8_at(0x14) != 0 {
        return Err("no es la fase 0 de una subasta de barco");
    }
    if towns.is_empty() {
        return Err("subasta sin ciudades");
    }
    if fleet.ships.iter().any(|s| s.len() != 0x180) {
        return Err("barco de subasta truncado");
    }
    let mut t = *task;
    let mut f = fleet.clone();
    t.payload[6] = 0x00;
    t.payload[7] = 0x0e;
    t.set_u8_at(0x14, 1);
    t.set_u8_at(0x16, 0);
    if usize::from(t.u8_at(0x15)) >= towns.len() {
        t.set_u8_at(0x15, ((t.due & 0xffff) % towns.len() as u32) as u8);
    }
    let city = t.u8_at(0x15);
    let town = &towns[usize::from(city)];
    let port = r.ports.get(usize::from(city)).copied().flatten().ok_or("puerto de subasta ausente")?;
    // 0x00507D60: reserva, con crecimiento de 0x40 registros si la lista libre está agotada.
    let count = f.ships.len();
    if usize::from(f.free_ship) >= count {
        if count + 0x40 > usize::from(u16::MAX) {
            return Err("pool naval fuera de rango");
        }
        f.free_ship = count as u16;
        for i in count..count + 0x40 {
            let mut ship = vec![0u8; 0x180];
            w16(&mut ship, 4, u16::MAX);
            w16(&mut ship, 6, (i + 1) as u16);
            w16(&mut ship, 8, u16::MAX);
            w16(&mut ship, 0x134, 0xff);
            f.ships.push(ship);
        }
    }
    let id = f.free_ship;
    let p = usize::from(id);
    if r16(&f.ships[p], 0x134) < 0x17 {
        return Err("ranura libre de subasta ocupada");
    }
    f.free_ship = r16(&f.ships[p], 6);
    let ship = &mut f.ships[p];
    factory::reset_ship(ship, r.tick).ok_or("barco de subasta truncado")?;
    ship[0x137] = 0;
    // Modo 2: estado 0x0C, sello +0x4C y entrada a la cabeza de gestor+0xE8.
    w16(ship, 0x134, 0x0c);
    let stamp = r
        .tick
        .wrapping_sub(u32::from(ship[0x4c]) << 8)
        .wrapping_sub(u32::from(id) << 8);
    ship[0x4c..0x50].copy_from_slice(&stamp.to_le_bytes());
    w16(ship, 6, f.docked_head);
    f.docked_head = id;
    t.payload[0..2].copy_from_slice(&id.to_le_bytes());
    factory::init_ship_at_town(ship, city, port.0, port.1, r.tick).ok_or("barco de subasta truncado")?;
    ship[0x13b] = city;
    t.payload[2..6].fill(0xff);
    let kind = (if town.u8(0x2c8) & 1 != 0 { r.tick & 1 } else { r.tick & 3 }) as u8;
    factory::init_ship_stats(town, ship, kind).ok_or("estadísticas de barco de subasta fuera de tabla")?;
    factory::recompute_weapon_slots(ship).ok_or("ranuras de barco de subasta inválidas")?;
    crate::ships::cargo::recompute_load(ship);
    ship[0x3d] |= 4;
    ship[0] = 0xff;
    w16(ship, 4, u16::MAX);
    let wear = ((r.tick as i32 >> 8).wrapping_add(r.tick as i32) & 0x7f) + 0x81;
    let hull = i32::from_le_bytes(ship[0x14..0x18].try_into().unwrap());
    ship[0x18..0x1c].copy_from_slice(&(wear.wrapping_mul(hull) >> 8).to_le_bytes());
    // 0x00509A50: nombre aleatorio (como mucho 31 caracteres y terminador).
    let index = factory::next_ship_name_index(&mut f.name_cursor, f.name_step, r.names.len())
        .ok_or("generador de nombres de subasta sin salida")?;
    let ship = &mut f.ships[p];
    w16(ship, 0x15e, index);
    let name = &r.names[usize::from(index)];
    let n = name.iter().position(|&b| b == 0).unwrap_or(name.len()).min(31);
    ship[0x160..0x160 + n].copy_from_slice(&name[..n]);
    ship[0x160 + n] = 0;
    ship[0x3c] |= 1;
    t.due = t.due.wrapping_add(1);
    *task = t;
    *fleet = f;
    Ok(id)
}

/// Efectos de la adjudicación de un lote naval que el llamante entrega fuera del gestor.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ShipAward {
    /// Cartas para `0x004D6530`: destinatario y registro de 16 bytes (constructor `0x0051A850`).
    pub letters: Vec<(u8, [u8; 16])>,
    /// Presentación `0x005C6740`: comprador, barco, precio y ciudad.
    pub result: Option<super::auction::AuctionResult>,
    /// Antiguo dueño cuyos grupos poda `0x004F5900`.
    pub prune: Option<u8>,
    /// Órdenes `0x00535760` (objeto local de cinco dobles palabras).
    pub orders: Vec<[u32; 5]>,
    /// Baja `0x004D4DD0` del índice `barco+0x132` en la lista `0x006DD728`.
    pub unlist: Option<u16>,
}

fn letter_base() -> [u8; 16] {
    let mut l = [0u8; 16];
    l[1] = 0x80;
    l[2..6].fill(0xff);
    l
}

/// Datos del mundo que la adjudicación lee.
pub struct AwardWorld<'a> {
    pub towns: &'a [Town],
    pub tick: u32,
    pub merchant_count: u16,
    /// Recuento de la lista `0x006DD728` (palabra `0x006DD72A`).
    pub list_count: u16,
}

/// `ShipManager::transfer_ship` (0x00509250) con comprador válido (`new_owner < merchant_count`),
/// el único caso que usa la subasta. `credit` se abona al dueño anterior y `price` se cobra al nuevo.
fn transfer_ship(
    w: &AwardWorld<'_>,
    ships: &mut [Vec<u8>],
    convoys: &mut [Vec<u8>],
    merchants: &mut [Vec<u8>],
    ship: u16,
    new_owner: u8,
    credit: i32,
    price: i32,
    out: &mut ShipAward,
) -> Result<(), &'static str> {
    let count = ships.len();
    let id = usize::from(ship);
    if id >= count || r16(&ships[id], 0x134) >= 0x11 {
        return Ok(());
    }
    let mc = usize::from(w.merchant_count);
    let tc = w.towns.len();
    let old = ships[id][0];
    if usize::from(new_owner) >= mc {
        return Err("traspaso naval sin comprador no portado");
    }
    if usize::from(old) < mc {
        let m = &merchants[usize::from(old)];
        let mut link = None::<usize>; // None = cabeza del comerciante
        let mut cur = r16(m, 0xe);
        while usize::from(cur) < count {
            if cur == ship {
                let next = r16(&ships[id], 4);
                match link {
                    None => w16(&mut merchants[usize::from(old)], 0xe, next),
                    Some(prev) => w16(&mut ships[prev], 4, next),
                }
                let m = &mut merchants[usize::from(old)];
                let gold = i32::from_le_bytes(m[0..4].try_into().unwrap()).wrapping_add(credit);
                m[0..4].copy_from_slice(&gold.to_le_bytes());
                let sold = i32::from_le_bytes(m[0x4b8..0x4bc].try_into().unwrap()).wrapping_add(credit);
                m[0x4b8..0x4bc].copy_from_slice(&sold.to_le_bytes());
                break;
            }
            link = Some(usize::from(cur));
            cur = r16(&ships[usize::from(cur)], 4);
        }
        ships[id][0x15d] = 0xff;
        // 0x00494990 / 0x004778A0: listas de ventanas, solo interfaz.
        ships[id][0] = 0xff;
        out.prune = Some(old);
    }
    let buyer = usize::from(new_owner);
    ships[id][0] = new_owner;
    let head = r16(&merchants[buyer], 0xe);
    w16(&mut ships[id], 4, head);
    let mut town = merchants[buyer][0x19];
    w16(&mut merchants[buyer], 0xe, ship);
    let skip_flagged = |mut t: u8| -> Result<u8, &'static str> {
        let mut guard = 0;
        while w.towns.get(usize::from(t)).ok_or("puerto base fuera de rango")?.u8(0x2c8) & 1 != 0 {
            t = ((u32::from(t) + 1) % tc as u32) as u8;
            guard += 1;
            if guard > tc {
                return Err("sin puerto base válido");
            }
        }
        Ok(t)
    };
    if ships[id][0xe] >= 2 {
        town = skip_flagged(town)?;
    }
    let m = &mut merchants[buyer];
    let gold = i32::from_le_bytes(m[0..4].try_into().unwrap()).wrapping_sub(price);
    m[0..4].copy_from_slice(&gold.to_le_bytes());
    let bought = i32::from_le_bytes(m[0x4bc..0x4c0].try_into().unwrap()).wrapping_add(price);
    m[0x4bc..0x4c0].copy_from_slice(&bought.to_le_bytes());
    let convoy = usize::from(r16(&ships[id], 8));
    let order = |t: u8| [0, u32::from(ship), u32::from(t), 0, 0];
    if convoy < convoys.len() {
        let c = &mut convoys[convoy];
        if r16(c, 0x10) == ship {
            c[0] = new_owner;
            if usize::from(c[0x1f]) >= tc {
                if c[0x14] & 4 != 0 {
                    town = skip_flagged(town)?;
                }
                out.orders.push(order(town));
            }
            convoys[convoy][0x14] &= 0xfd;
        } else if c[0] != new_owner {
            out.orders.push(order(town));
        }
    } else if usize::from(ships[id][0x39]) >= tc && usize::from(ships[id][0x38]) >= tc {
        out.orders.push(order(town));
    }
    let s = &mut ships[id];
    s[0x136] &= 0xfc;
    let listed = r16(s, 0x132);
    if listed < w.list_count {
        out.unlist = Some(listed);
        w16(s, 0x132, u16::MAX);
    }
    if r16(s, 0x134) == 0xc {
        w16(s, 0x134, 0);
    }
    let flags = r16(s, 0x3c) | 0x420;
    w16(s, 0x3c, flags);
    s[0x124..0x12c].fill(0);
    s[0x136] = 0;
    s[0x13a] = town;
    w16(s, 0x138, 0);
    Ok(())
}

/// Fase 7 con lote naval existente (0x004E31D6..0x004E343E) y paso al lote siguiente.
/// Trabaja sobre copias: si algo no cuadra devuelve error sin cambiar nada.
pub fn award_lot_ship(
    task: &mut TaskRecord,
    w: &AwardWorld<'_>,
    ships: &mut [Vec<u8>],
    convoys: &mut [Vec<u8>],
    merchants: &mut [Vec<u8>],
) -> Result<ShipAward, &'static str> {
    if task.kind != 0x0a || task.u8_at(0x14) != 7 {
        return Err("no es la adjudicación de una subasta de barco");
    }
    let tc = w.towns.len();
    let mc = usize::from(w.merchant_count);
    if tc == 0 || ships.iter().any(|s| s.len() != 0x180) || convoys.iter().any(|c| c.len() < 0x20) {
        return Err("datos de adjudicación naval incompletos");
    }
    if merchants.len() < mc || merchants[..mc].iter().any(|m| m.len() < 0x4c0) {
        return Err("comerciantes de adjudicación incompletos");
    }
    let mut t = *task;
    let mut sh = ships.to_vec();
    let mut cv = convoys.to_vec();
    let mut me = merchants.to_vec();
    let mut out = ShipAward::default();
    let id = u16::from_le_bytes([t.u8_at(8), t.u8_at(9)]);
    if usize::from(id) >= sh.len() {
        return Err("lote naval inexistente");
    }
    if usize::from(t.u8_at(0x15)) >= tc {
        t.set_u8_at(0x15, (u32::from(t.u8_at(0x15)) % tc as u32) as u8);
    }
    t.set_u8_at(8, 0xff);
    t.set_u8_at(9, 0xff);
    let city = t.u8_at(0x15);
    let buyer = t.u8_at(0x17);
    let price = u16::from_le_bytes([t.u8_at(0xe), t.u8_at(0xf)]);
    let p = i32::from(price);
    let credit = p * 9 / 10;
    let s = usize::from(id);
    let owner = sh[s][0];
    let ship_name = r16(&sh[s], 0x15e);
    let name = |m: &Vec<u8>| [m[0x1c], m[0x1d]];
    if usize::from(buyer) < mc {
        let seller = if usize::from(owner) < mc {
            let b = name(&me[usize::from(buyer)]);
            let o = &me[usize::from(owner)];
            let mut l = letter_base();
            l[4] = 0x2c + u8::from(owner == buyer);
            l[5] = city;
            l[8] = b[0];
            l[9] = b[1];
            l[10..12].copy_from_slice(&ship_name.to_le_bytes());
            l[12..14].copy_from_slice(&(credit as u16).to_le_bytes());
            l[14..16].copy_from_slice(&((p / 10) as u16).to_le_bytes());
            if r16(o, 8) == 0 {
                out.letters.push((owner, l));
            }
            (u16::from(o[0x1c]) << 8).wrapping_add(u16::from(o[0x1d]))
        } else {
            let low = w.tick.wrapping_add(u32::from(id));
            (((low << 8) & 0x3f00) + (w.tick & 0x3f)) as u16
        };
        if owner != buyer {
            let mut l = letter_base();
            l[4] = 0x5d;
            l[5] = city;
            l[8..10].copy_from_slice(&seller.to_le_bytes());
            l[10..12].copy_from_slice(&ship_name.to_le_bytes());
            l[12] = sh[s][0xe] & 3;
            l[14..16].copy_from_slice(&((p / 10) as u16).to_le_bytes());
            out.letters.push((buyer, l));
        }
        transfer_ship(w, &mut sh, &mut cv, &mut me, id, buyer, credit, p, &mut out)?;
    } else {
        let mut l = letter_base();
        l[4] = 0x2e;
        l[5] = city;
        if let Some(o) = me.get(usize::from(owner)).filter(|_| usize::from(owner) < mc) {
            l[8] = o[0x1c];
            l[10] = o[0x1d];
        }
        l[12..14].copy_from_slice(&ship_name.to_le_bytes());
        out.letters.push((owner, l));
        sh[s][0x3c] |= 0x20;
        w16(&mut sh[s], 0x134, 0);
    }
    out.result = Some(super::auction::AuctionResult { buyer, object: id, price, city, building: false });
    super::auction::skip_missing_lot(&mut t, w.towns, sh.len() as u16)?;
    *task = t;
    ships.clone_from_slice(&sh);
    convoys.clone_from_slice(&cv);
    merchants.clone_from_slice(&me);
    Ok(out)
}
