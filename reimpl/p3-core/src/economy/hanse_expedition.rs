//! Fases acreditadas de las expediciones de la Liga (0x004E09D4).
use crate::{ai::reputation::Merchant, pump::Operation, scheduler::TaskRecord, town::Town};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExpeditionShip {
    pub next: u16,
    pub town: u8,
    pub state: u16,
}

/// Solicitud que la búsqueda entrega al planificador naval.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HuntRouteRequest {
    pub town: u8,
    pub x: i16,
    pub y: i16,
    pub leader: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HuntReturn {
    Finished,
    EnRoute(HuntRouteRequest),
    Arrived(HuntRouteRequest),
}

/// Fase 10 con convoy activo. La llegada exige además emitir el informe y
/// liberar la lista de bajas; este helper solo señala esa transición.
pub fn hunt_return(
    r: &mut TaskRecord,
    convoy: &[u8],
    ship: &mut [u8],
    leader: u16,
    port: (i16, i16),
) -> Option<HuntReturn> {
    if r.kind != 0x34 || r.u8_at(0x15) != 10 || convoy.len() < 0x14 || ship.len() != 0x180 {
        return None;
    }
    if u16::from_le_bytes([convoy[0x12], convoy[0x13]]) != 0x16 {
        return Some(HuntReturn::Finished);
    }
    let x = i16::from_le_bytes([ship[0x1e], ship[0x1f]]);
    let y = i16::from_le_bytes([ship[0x22], ship[0x23]]);
    let distance = ((i32::from(port.0) - i32::from(x)).abs() + (i32::from(port.1) - i32::from(y)).abs()) >> 1;
    ship[0x3a..0x3c].fill(255);
    let request = HuntRouteRequest {
        town: r.u8_at(0x14),
        x: 0,
        y: 0,
        leader,
    };
    if distance > 70 {
        r.due = r.due.wrapping_add(70);
        Some(HuntReturn::EnRoute(request))
    } else if distance > 10 {
        r.due = r.due.wrapping_add((distance - 7) as u32);
        Some(HuntReturn::EnRoute(request))
    } else {
        Some(HuntReturn::Arrived(request))
    }
}

/// Rama de fase 9 con un objetivo ya seleccionado y un convoy activo.
/// El llamador verifica previamente el plazo y la cadena de convoyes. El
/// descriptor y sus puntos se pasan aparte: un puntero x86 no es portable.
pub fn hunt_existing_target(
    r: &mut TaskRecord,
    ships: &mut [Vec<u8>],
    convoy_count: u16,
    leader: u16,
    tick: u32,
    target_route: Option<&[(i16, i16)]>,
) -> Option<HuntRouteRequest> {
    if r.kind != 0x34 || r.u8_at(0x15) != 9 || tick > r.u32_at(0x10) || ships.iter().any(|s| s.len() != 0x180) {
        return None;
    }
    let li = usize::from(leader);
    let lead = ships.get(li)?;
    let target = u16::from_le_bytes([lead[0x3a], lead[0x3b]]);
    let ti = usize::from(target);
    let enemy = ships.get(ti)?;
    let valid = u16::from_le_bytes([enemy[0x134], enemy[0x135]]) == 0x12
        && u16::from_le_bytes([enemy[8], enemy[9]]) < convoy_count;
    if !valid {
        ships[li][0x3a..0x3c].fill(255);
        r.due = r.due.wrapping_add(0x20);
        return Some(HuntRouteRequest {
            town: r.u8_at(0x14),
            x: 0,
            y: 0,
            leader,
        });
    }
    let x = i16::from_le_bytes([enemy[0x1e], enemy[0x1f]]);
    let y = i16::from_le_bytes([enemy[0x22], enemy[0x23]]);
    let (dest_x, dest_y) = target_route
        .and_then(|p| p.get((tick & 7) as usize))
        .copied()
        .unwrap_or((x, y));
    let leader_x = i16::from_le_bytes([lead[0x1e], lead[0x1f]]);
    let dx = i32::from(x) - i32::from(leader_x);
    let delta = if dx <= 0 {
        if dx > -0x47 {
            1 - dx
        } else {
            0x46
        }
    } else if dx < 0x6c {
        dx + 1
    } else {
        0x6b
    };
    r.due = r.due.wrapping_add(delta as u32);
    Some(HuntRouteRequest {
        town: 255,
        x: dest_x,
        y: dest_y,
        leader,
    })
}

/// Selecciona el primer líder pirata enlazado en el mar, en orden de banda y
/// de la cadena de cada banda. No modifica la tarea si encuentra ciclos o
/// imágenes incompletas. El original no comprueba su estado hasta la visita
/// posterior a esta primera selección.
pub fn hunt_select_target(
    r: &mut TaskRecord,
    ships: &mut [Vec<u8>],
    convoys: &[Vec<u8>],
    bands: &[Option<u16>; 5],
    leader: u16,
    tick: u32,
    routes: &[Option<Vec<(i16, i16)>>],
) -> Option<HuntRouteRequest> {
    if r.kind != 0x34
        || r.u8_at(0x15) != 9
        || tick > r.u32_at(0x10)
        || ships.len() > u16::MAX as usize
        || ships.iter().any(|s| s.len() != 0x180)
        || convoys.iter().any(|c| c.len() < 0x12)
    {
        return None;
    }
    let li = usize::from(leader);
    let lead = ships.get(li)?;
    if u16::from_le_bytes([lead[0x3a], lead[0x3b]]) < (ships.len() as u16) {
        return None;
    }
    let mut selected = None;
    if tick < r.u32_at(0x10) {
        for head in bands.iter().flatten() {
            let mut cursor = *head;
            let mut seen = vec![false; ships.len()];
            while usize::from(cursor) < ships.len() {
                let i = usize::from(cursor);
                if seen[i] {
                    return None;
                }
                seen[i] = true;
                let ship = &ships[i];
                let cv = usize::from(u16::from_le_bytes([ship[8], ship[9]]));
                let prev = u16::from_le_bytes([ship[0xa], ship[0xb]]);
                let next = u16::from_le_bytes([ship[0xc], ship[0xd]]);
                if cv < convoys.len()
                    && (usize::from(prev) < ships.len() || usize::from(next) < ships.len())
                    && u16::from_le_bytes([convoys[cv][0x10], convoys[cv][0x11]]) == cursor
                {
                    selected = Some(cursor);
                    break;
                }
                cursor = u16::from_le_bytes([ship[4], ship[5]])
            }
            if selected.is_some() {
                break;
            }
        }
    }
    if let Some(target) = selected {
        let enemy = &ships[usize::from(target)];
        let x = i16::from_le_bytes([enemy[0x1e], enemy[0x1f]]);
        let y = i16::from_le_bytes([enemy[0x22], enemy[0x23]]);
        let (dest_x, dest_y) = routes
            .get(usize::from(target))
            .and_then(Option::as_ref)
            .and_then(|p| p.get((tick & 7) as usize))
            .copied()
            .unwrap_or((x, y));
        let leader_x = i16::from_le_bytes([ships[li][0x1e], ships[li][0x1f]]);
        let dx = i32::from(x) - i32::from(leader_x);
        let delta = if dx <= 0 {
            if dx > -0x47 {
                1 - dx
            } else {
                0x46
            }
        } else if dx < 0x6c {
            dx + 1
        } else {
            0x6b
        };
        ships[li][0x3a..0x3c].copy_from_slice(&target.to_le_bytes());
        r.due = r.due.wrapping_add(delta as u32);
        Some(HuntRouteRequest {
            town: 255,
            x: dest_x,
            y: dest_y,
            leader,
        })
    } else {
        r.due = r.due.wrapping_add(0x20);
        Some(HuntRouteRequest {
            town: r.u8_at(0x14),
            x: 0,
            y: 0,
            leader,
        })
    }
}

/// Boycot y bloqueo: patrullas (3/6) y espera por satisfacción (4).
/// Rechaza listas incompletas o cíclicas antes de modificar el registro.
pub fn patrol(
    r: &mut TaskRecord,
    towns: &mut [Town],
    merchants: &[Merchant],
    ships: &[ExpeditionShip],
    ship_count: u16,
    network: bool,
) -> Option<Vec<Operation>> {
    if !matches!(r.kind, 0x23 | 0x24 | 0x34) || !matches!(r.u8_at(0x15), 3 | 4 | 6) {
        return None;
    }
    let city = usize::from(r.u8_at(0x14));
    if city >= towns.len() {
        return None;
    }
    let phase = r.u8_at(0x15);
    if phase == 4 {
        if towns[city].i16(0x304) > -21 {
            r.due = r.due.wrapping_add(0x100)
        } else {
            towns[city].set_u8(0x6f0, 255);
            r.set_u8_at(0x15, 5);
            r.due = r.due.wrapping_add(1)
        }
        return Some(Vec::new());
    }
    if phase == 6 && (r.u32_at(0x10) < r.due || towns[city].i16(0x304) < -10) {
        towns[city].set_u8(0x6f0, 255);
        r.set_u8_at(0x15, 5);
        r.due = r.due.wrapping_add(5);
        return Some(Vec::new());
    }
    let mut hits = Vec::new();
    for (i, m) in merchants.iter().enumerate() {
        if m.u16(8) != 0 {
            continue;
        }
        let mut cursor = m.u16(0xe);
        let mut seen = vec![false; ships.len()];
        while cursor < ship_count {
            let k = usize::from(cursor);
            let ship = ships.get(k)?;
            if seen[k] {
                return None;
            }
            seen[k] = true;
            if usize::from(ship.town) == city && ship.state == 3 {
                hits.push(i as u32);
                break;
            }
            cursor = ship.next;
        }
    }
    towns[city].set_flags(towns[city].flags() | 0x400);
    let operations = if network {
        Vec::new()
    } else {
        hits.iter()
            .map(|&m| Operation([0x81, m, 1, 0x100, city as u32]))
            .collect()
    };
    let next = r.due.wrapping_add(0x45);
    r.due = if hits.is_empty() {
        next
    } else {
        (next | 255).wrapping_add(1)
    };
    if r.due > r.u32_at(0x10) && phase != 6 {
        r.set_u8_at(0x15, 6);
        r.set_u32_at(0x10, r.due.wrapping_add(0x1c00));
    }
    Some(operations)
}

/// Sanciona cada ranura incumplida y agrupa por destinatario la carta 0x4c.
/// La caza requiere una imagen completa del banco de piratas y capitanes.
pub fn penalty(
    r: &mut TaskRecord,
    towns: &mut [Town],
    merchants: &mut [Merchant],
    tick: u32,
    bands: Option<&crate::economy::hanse_petition::PiratePortraits>,
) -> Option<Vec<(usize, [u8; 8])>> {
    if !matches!(r.kind, 0x23 | 0x24 | 0x34) || r.u8_at(0x15) != 2 {
        return None;
    }
    let city = usize::from(r.u8_at(0x14));
    if city >= towns.len() || towns.len() > 24 {
        return None;
    }
    for m in merchants.iter() {
        if m.bytes.len() < 0x650 || usize::from(m.bytes[0x19]) >= 24 {
            return None;
        }
    }
    let mut portrait = None;
    if r.kind == 0x34 {
        let bands = bands?;
        for head in bands.heads {
            let Some(mut ship) = head else { continue };
            let mut seen = vec![false; bands.next.len()];
            while usize::from(ship) < bands.next.len() {
                let i = usize::from(ship);
                if seen[i] {
                    return None;
                }
                seen[i] = true;
                let captain = usize::from(*bands.ship_captains.get(i)?);
                if let Some(c) = bands.captains.get(captain) {
                    portrait = Some((c[2], c[3]));
                    break;
                }
                ship = bands.next[i];
            }
            if portrait.is_some() {
                break;
            }
        }
    }
    let env = crate::ai::reputation::RepEnv {
        town_count: towns.len() as u32,
        towns,
        renown: [0; 3],
        byte_52c: 0,
    };
    let mut letters = Vec::new();
    for i in 0..8 {
        let merchant = usize::from(r.u8_at(8 + i));
        if merchant < merchants.len() {
            let m = &mut merchants[merchant];
            m.apply_severe_reputation_penalty(&env, 255);
            let mut count = 1u8;
            for j in i + 1..8 {
                if usize::from(r.u8_at(8 + j)) == merchant {
                    r.set_u8_at(8 + j, 255);
                    m.apply_severe_reputation_penalty(&env, 255);
                    count = count.wrapping_add(1)
                }
            }
            let mut payload = [0u8; 8];
            payload[0] = m.bytes[0x39c + usize::from(m.bytes[0x19])];
            payload[1] = m.bytes[0x1c];
            payload[2] = m.bytes[0x1d];
            payload[3] = if r.kind == 0x34 { 2 } else { (r.kind - 0x23) as u8 };
            payload[4] = city as u8;
            payload[5] = count;
            letters.push((merchant, payload));
        }
        r.set_u8_at(8 + i, 255)
    }
    if r.kind == 0x34 {
        r.set_u32_at(8, 0);
        r.set_u8_at(0xe, 0);
        if let Some((a, b)) = portrait {
            r.set_u8_at(0xc, a);
            r.set_u8_at(0xd, b)
        }
        r.set_u8_at(0x15, 7);
        r.due = r.due.wrapping_add(1)
    } else if r.kind == 0x24 {
        towns[city].set_flags(towns[city].flags() | 0x200);
        r.set_u8_at(0x15, 4);
        r.due = r.due.wrapping_add(0xa00)
    } else {
        r.set_u8_at(0x15, 3);
        r.due = r.due.wrapping_add(0x40);
        r.set_u32_at(0x10, tick.wrapping_add(0xa00))
    }
    Some(letters)
}

/// Selección de barcos y cartas de la primera fase. Los recuentos se reciben
/// del puerto de `merchant_count_active_ships_div5`, todavía no conectado aquí.
pub fn select(
    r: &mut TaskRecord,
    merchants: &[Merchant],
    scores: &[u16],
    bands: Option<&crate::economy::hanse_petition::PiratePortraits>,
) -> Option<Vec<(usize, [u8; 8])>> {
    if !matches!(r.kind, 0x23 | 0x24 | 0x34) || r.u8_at(0x15) != 0 || scores.len() != merchants.len() {
        return None;
    }
    if merchants
        .iter()
        .any(|m| m.bytes.len() < 0x650 || usize::from(m.bytes[0x19]) >= 24)
    {
        return None;
    }
    let mut portrait = None;
    if r.kind == 0x34 {
        let bands = bands?;
        for head in bands.heads {
            let Some(mut ship) = head else { continue };
            let mut seen = vec![false; bands.next.len()];
            while usize::from(ship) < bands.next.len() {
                let i = usize::from(ship);
                if seen[i] {
                    return None;
                }
                seen[i] = true;
                let cap = usize::from(*bands.ship_captains.get(i)?);
                if let Some(c) = bands.captains.get(cap) {
                    portrait = Some((c[2], c[3]));
                    break;
                }
                ship = bands.next[i]
            }
            if portrait.is_some() {
                break;
            }
        }
    }
    let mut entries = Vec::<(u8, u16)>::new();
    for (i, m) in merchants.iter().enumerate() {
        if m.u16(8) != 0 {
            continue;
        }
        let score = scores[i];
        if score == 1 {
            entries.push((i as u8, score))
        } else if score >= 2 && m.u16(0x478) > 10 {
            entries.push((i as u8, score));
            entries.push((i as u8, m.u16(0x478) >> 1));
        }
    }
    // El bucle original no compara nunca las dos primeras posiciones.
    for start in 1..entries.len().saturating_sub(1) {
        let mut swapped = false;
        for i in (start..entries.len() - 1).rev() {
            if entries[i].1 < entries[i + 1].1 {
                entries.swap(i, i + 1);
                swapped = true
            }
        }
        if !swapped {
            break;
        }
    }
    r.set_u8_at(0x15, 1);
    r.due = r.due.wrapping_add(0x100) & 0xffffff00;
    let limit = if r.kind == 0x23 { 2 } else { 8 };
    for i in 0..8 {
        r.set_u8_at(8 + i, entries.get(i).filter(|_| i < limit).map_or(255, |v| v.0));
    }
    let mut letters = Vec::new();
    for (i, m) in merchants.iter().enumerate() {
        if m.u16(8) != 0 {
            continue;
        }
        let mut count = 0u8;
        for j in 0..8 {
            if usize::from(r.u8_at(8 + j)) == i {
                if r.kind == 0x23 && count != 0 {
                    r.set_u8_at(8 + j, 255)
                } else {
                    count += 1
                }
            }
        }
        let mut payload = [0u8; 8];
        payload[0] = m.bytes[0x39c + usize::from(m.bytes[0x19])];
        payload[1] = m.bytes[0x1c];
        payload[2] = m.bytes[0x1d];
        payload[3] = if r.kind == 0x34 { 2 } else { (r.kind - 0x23) as u8 };
        payload[4] = r.u8_at(0x14);
        payload[5] = count;
        if let Some((a, b)) = portrait {
            payload[6] = a;
            payload[7] = b
        }
        letters.push((i, payload));
    }
    Some(letters)
}

/// `ship_can_lead_convoy` (0x00519c40); imagen de memoria, no serializada.
pub fn ship_eligible(ship: &[u8], convoys: u16, captains: u16) -> Option<bool> {
    if ship.len() < 0x180 {
        return None;
    }
    let word = |i| u16::from_le_bytes([ship[i], ship[i + 1]]);
    let dword = |i| i32::from_le_bytes(ship[i..i + 4].try_into().unwrap());
    Some(
        word(0x40) >= 20
            && (word(8) < convoys || dword(0x18) >= (dword(0x14) >> 1))
            && dword(0x120) >= 0x43
            && word(0x42) < captains
            && ship[0x15c] == 0,
    )
}

/// Recuentos sobre imágenes de memoria completas; detecta cadenas cíclicas o truncadas.
pub fn fleet_scores(
    r: &TaskRecord,
    towns: &[Town],
    merchants: &[Merchant],
    ships: &[Option<Vec<u8>>],
    ship_count: u16,
    convoys: u16,
    captains: u16,
) -> Option<Vec<u16>> {
    let city = towns.get(usize::from(r.u8_at(0x14)))?;
    if usize::from(ship_count) > ships.len() {
        return None;
    }
    let mask = (city.u8(0x2c8) & 1) * 2;
    let mut scores = Vec::with_capacity(merchants.len());
    for m in merchants {
        if m.bytes.len() < 0x650 {
            return None;
        }
        if m.u16(8) != 0 {
            scores.push(0);
            continue;
        }
        let mut n = 0u16;
        let mut cursor = m.u16(0xe);
        let mut seen = vec![false; usize::from(ship_count)];
        while cursor < ship_count {
            let i = usize::from(cursor);
            if seen[i] {
                return None;
            }
            seen[i] = true;
            let ship = ships[i].as_deref()?;
            if ship.len() < 0x180 {
                return None;
            }
            let word = |o| u16::from_le_bytes([ship[o], ship[o + 1]]);
            let hull = i32::from_le_bytes(ship[0x18..0x1c].try_into().ok()?);
            let state = word(0x134);
            if hull > 0
                && (state < 4 || state == 0xf || state == 0x10)
                && ship[0xe] & mask == 0
                && (r.kind == 0x23 || ship_eligible(ship, convoys, captains)?)
            {
                n = n.wrapping_add(1)
            }
            cursor = word(4);
        }
        scores.push(n / 5)
    }
    Some(scores)
}

/// Efectos externos de la fase 1. El convoy debe haberse creado con el puerto
/// completo de `create_pirate_band_convoy` antes de confirmar esta transición.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BeginEffects {
    pub notifications: Vec<(u16, u32, u8)>,
    pub event_window: bool,
    pub create: (u8, u16, bool, bool),
}
pub fn begin(
    r: &mut TaskRecord,
    towns: &mut [Town],
    merchants: &[Merchant],
    hotseat: bool,
    player: u16,
    task_id: u16,
    convoy: u16,
) -> Option<BeginEffects> {
    if !matches!(r.kind, 0x23 | 0x24 | 0x34) || r.u8_at(0x15) != 1 {
        return None;
    }
    let city = usize::from(r.u8_at(0x14));
    let town = towns.get_mut(city)?;
    if merchants.iter().any(|m| m.bytes.len() < 10) {
        return None;
    }
    if r.kind != 0x34 {
        town.set_flags(town.flags() | 0x400)
    }
    let mut notifications = Vec::new();
    if r.kind == 0x24 {
        if hotseat {
            for m in merchants {
                if m.u16(8) == 0 {
                    notifications.push((0, u32::MAX, city as u8))
                }
            }
        } else {
            notifications.push((player, 1, city as u8))
        }
    }
    let event_window = r.kind == 0x24;
    let create = (city as u8, task_id, r.kind != 0x23, r.kind == 0x34);
    r.set_u8_at(0x16, convoy as u8);
    r.set_u8_at(0x17, (convoy >> 8) as u8);
    r.set_u8_at(0x15, 2);
    r.due = r.due.wrapping_add(0x900);
    Some(BeginEffects {
        notifications,
        event_window,
        create,
    })
}

/// Cierre de fase 5 cuando el índice de convoy ya no designa una ranura válida.
/// Devuelve las cartas 0x5c; el planificador debe retirar la tarea.
pub fn finish_without_convoy(
    r: &TaskRecord,
    towns: &mut [Town],
    merchants: &[Merchant],
    convoy_count: u16,
) -> Option<Vec<(usize, [u8; 8])>> {
    if !matches!(r.kind, 0x23 | 0x24 | 0x34) || r.u8_at(0x15) != 5 {
        return None;
    }
    let convoy = u16::from_le_bytes([r.u8_at(0x16), r.u8_at(0x17)]);
    if convoy < convoy_count {
        return None;
    }
    let town = towns.get_mut(usize::from(r.u8_at(0x14)))?;
    if merchants.iter().any(|m| m.bytes.len() < 0x1e) {
        return None;
    }
    let blocked = town.flags() & 0x200 != 0;
    town.set_flags(town.flags() & !0x600);
    let mut letters = Vec::new();
    for (i, m) in merchants.iter().enumerate() {
        if m.u16(8) != 0 {
            continue;
        }
        let mut p = [0u8; 8];
        p[0] = m.bytes[0x1c];
        p[2] = m.bytes[0x1d];
        p[4] = r.u8_at(0x14);
        p[5] = u8::from(blocked);
        letters.push((i, p))
    }
    Some(letters)
}

/// Fase 5 con convoy válido; desembarca sus barcos en la lista del gestor.
/// El llamante debe persistir conjuntamente convoy, barcos, cabeza y cartas.
pub fn finish_convoy(
    r: &TaskRecord,
    towns: &mut [Town],
    merchants: &[Merchant],
    convoy: &mut [u8],
    ships: &mut [Option<Vec<u8>>],
    ship_count: u16,
    docked: &mut u16,
) -> Option<Vec<(usize, [u8; 8])>> {
    if !matches!(r.kind, 0x23 | 0x24 | 0x34)
        || r.u8_at(0x15) != 5
        || convoy.len() < 0x3c
        || usize::from(ship_count) > ships.len()
    {
        return None;
    }
    let city = usize::from(r.u8_at(0x14));
    if city >= towns.len() || merchants.iter().any(|m| m.bytes.len() < 0x1e) {
        return None;
    }
    let mut chain = Vec::new();
    let mut cursor = u16::from_le_bytes([convoy[0xa], convoy[0xb]]);
    let mut seen = vec![false; usize::from(ship_count)];
    while cursor < ship_count {
        let i = usize::from(cursor);
        if seen[i] {
            return None;
        }
        seen[i] = true;
        let ship = ships[i].as_deref()?;
        if ship.len() < 0x180 {
            return None;
        }
        chain.push(i);
        cursor = u16::from_le_bytes([ship[6], ship[7]])
    }
    let blocked = towns[city].flags() & 0x200 != 0;
    let mut letters = Vec::new();
    for (i, m) in merchants.iter().enumerate() {
        if m.u16(8) != 0 {
            continue;
        }
        let count = chain
            .iter()
            .filter(|&&j| usize::from(ships[j].as_ref().unwrap()[0]) == i)
            .count() as u16;
        let mut p = [0u8; 8];
        p[0] = m.bytes[0x1c];
        p[2] = m.bytes[0x1d];
        p[4] = city as u8;
        p[5] = u8::from(blocked);
        p[6..8].copy_from_slice(&count.to_le_bytes());
        letters.push((i, p))
    }
    towns[city].set_flags(towns[city].flags() & !0x600);
    for i in chain {
        let ship = ships[i].as_mut().unwrap();
        ship[0x3c] |= 0x60;
        ship[6..8].copy_from_slice(&docked.to_le_bytes());
        ship[8..10].copy_from_slice(&u16::MAX.to_le_bytes());
        ship[0x134..0x136].copy_from_slice(&0u16.to_le_bytes());
        ship[0x138..0x13a].copy_from_slice(&0x20u16.to_le_bytes());
        *docked = i as u16
    }
    convoy[0xa..0xc].copy_from_slice(&cursor.to_le_bytes());
    Some(letters)
}

/// Comprueba la cadena activa sin seguir punteros inválidos ni ciclos.
pub fn convoy_is_active(id: u16, head: u16, convoys: &[Vec<u8>]) -> Option<bool> {
    if usize::from(id) >= convoys.len() {
        return None;
    }
    let mut cursor = head;
    let mut seen = vec![false; convoys.len()];
    if usize::from(cursor) >= convoys.len() {
        return None;
    }
    loop {
        if cursor == id {
            return Some(true);
        }
        let i = usize::from(cursor);
        if seen[i] {
            return None;
        }
        seen[i] = true;
        let convoy = convoys.get(i)?;
        if convoy.len() < 10 {
            return None;
        }
        cursor = u16::from_le_bytes([convoy[8], convoy[9]]);
        if usize::from(cursor) >= convoys.len() {
            return Some(false);
        }
    }
}

/// Fase 7 de caza: salida de puerto y propagación del estado del convoy.
pub fn hunt_depart(
    r: &mut TaskRecord,
    convoy: &mut [u8],
    ships: &mut [Option<Vec<u8>>],
    ship_count: u16,
) -> Option<()> {
    if r.kind != 0x34 || r.u8_at(0x15) != 7 || convoy.len() < 0x3c || usize::from(ship_count) > ships.len() {
        return None;
    }
    let mut cursor = u16::from_le_bytes([convoy[10], convoy[11]]);
    let mut chain = Vec::new();
    let mut seen = vec![false; usize::from(ship_count)];
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
        chain.push(i);
        cursor = u16::from_le_bytes([ship[6], ship[7]])
    }
    let status = u16::from_le_bytes([convoy[0x12], convoy[0x13]]);
    let speed = if status == 11 {
        let n = u16::from_le_bytes([convoy[0x16], convoy[0x17]]).wrapping_add(1);
        convoy[0x16..0x18].copy_from_slice(&n.to_le_bytes());
        if n > 0x3f {
            r.set_u8_at(0x15, 8)
        }
        1
    } else {
        convoy[0x12..0x14].copy_from_slice(&11u16.to_le_bytes());
        convoy[0x18] |= 0x20;
        convoy[0x16..0x18].copy_from_slice(&0u16.to_le_bytes());
        u32::from_le_bytes(convoy[0xc..0x10].try_into().ok()?) >> 1
    };
    for i in chain {
        let ship = ships[i].as_mut()?;
        ship[0x138..0x13a].copy_from_slice(&convoy[0x16..0x18]);
        if u16::from_le_bytes([ship[0x134], ship[0x135]]) != 11 {
            ship[0x3c] |= 0xa0;
            ship[0x134..0x136].copy_from_slice(&11u16.to_le_bytes());
            ship[0x30..0x34].copy_from_slice(&speed.to_le_bytes())
        }
    }
    r.due = r.due.wrapping_add(1);
    Some(())
}

/// Operación 0x81 (0x0053fa80): crea una tarea 0x05; su resolución es otra fase.
pub fn dispatch_investigation(
    op: &crate::pump::Operation,
    tick: u32,
    merchants: &[Merchant],
    scheduler: &mut crate::scheduler::Scheduler,
) -> Option<u16> {
    if op.0[0] != 0x81 {
        return None;
    }
    let owner = usize::try_from(op.0[1]).ok()?;
    let merchant = merchants.get(owner)?;
    if merchant.bytes.len() <= 0x19 {
        return None;
    }
    let due = (tick.wrapping_add(op.0[3]).wrapping_add(0x100) & 0xffffff00) | 0x80;
    let id = scheduler.add_task(due);
    let r = &mut scheduler.records[usize::from(id)];
    r.kind = 5;
    r.set_u8_at(8, owner as u8);
    r.set_u8_at(9, op.0[4] as u8);
    r.set_u8_at(10, 0);
    r.set_u8_at(11, merchant.bytes[0x19]);
    r.set_u32_at(0xc, tick);
    r.set_u8_at(0x10, (op.0[2] as u8).min(7));
    r.set_u8_at(0x11, 0);
    Some(id)
}

/// Fase 9: vencimiento de la persecución o espera por batalla naval.
pub fn hunt_wait(r: &mut TaskRecord, tick: u32, convoy: Option<&[u8]>) -> Option<()> {
    if r.kind != 0x34 || r.u8_at(0x15) != 9 {
        return None;
    }
    if r.u32_at(0x10) < tick {
        r.set_u8_at(0x15, 10);
        r.due = r.due.wrapping_add(1);
        return Some(());
    }
    let c = convoy?;
    if c.len() < 0x14 || u16::from_le_bytes([c[0x12], c[0x13]]) != 0x14 {
        return None;
    }
    r.due = r.due.wrapping_add(0x40);
    Some(())
}

/// Acusación inicial por delitos 1 y 6 (0x004e5b84), con dos muestras CRT explícitas.
/// El flujo CRT no se almacena en S05; el llamante debe suministrar el estado real.
pub fn begin_investigation(
    r: &mut TaskRecord,
    town_count: usize,
    merchants: &[Merchant],
    random: [u16; 2],
) -> Option<Option<(usize, u8, [u8; 8])>> {
    let crime = r.u8_at(0x10);
    if r.kind != 5 || !matches!(crime, 1 | 6) || r.u8_at(0x11) != 0 {
        return None;
    }
    let owner = usize::from(r.u8_at(8));
    let target = r.u8_at(9);
    let court = r.u8_at(11);
    if usize::from(target) >= town_count || usize::from(court) >= town_count {
        return None;
    }
    let m = merchants.get(owner)?;
    if m.bytes.len() < 0x1e {
        return None;
    }
    let message = if m.u16(8) == 0 {
        let mut p = [0u8; 8];
        p[0] = m.bytes[0x1c];
        if crime == 1 {
            p[1] = m.bytes[0x1d];
            p[2] = 1;
            p[3] = target
        } else {
            p[2] = m.bytes[0x1d];
            p[4] = 6
        }
        Some((owner, court, p))
    } else {
        None
    };
    r.set_u8_at(0x11, 1);
    for (offset, n) in [(0x12, random[0]), (0x14, random[1])] {
        r.set_u8_at(offset, n as u8);
        r.set_u8_at(offset + 1, (n >> 8) as u8)
    }
    r.due = r.due.wrapping_add(((r.due & 7) + 8) * 0x100) | 0x80;
    Some(message)
}

/// Resultado del juicio por boicot cuando no está activa la sanción en mercancía.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InvestigationVerdict {
    pub keep: bool,
    pub letter: Option<(usize, u8, u8, [u8; 8])>,
    pub notify: bool,
    pub towns: Vec<usize>,
}
pub fn investigation_verdict(
    r: &mut TaskRecord,
    towns: &mut [Town],
    merchants: &mut [Merchant],
    tick: u32,
    mut stock: Option<&mut [i32; 24]>,
) -> Option<InvestigationVerdict> {
    let crime = r.u8_at(0x10);
    if r.kind != 5 || !matches!(crime, 1 | 6) || r.u8_at(0x11) != 1 {
        return None;
    }
    let owner = usize::from(r.u8_at(8));
    let target = usize::from(r.u8_at(9));
    let court = usize::from(r.u8_at(11));
    let m = merchants.get_mut(owner)?;
    if m.bytes.len() < 0x650 || court >= towns.len() || target >= towns.len() {
        return None;
    }
    let home = usize::from(m.bytes[0x19]);
    if home >= towns.len() {
        return None;
    }
    let special = towns[home].flags() & 0x20000 != 0;
    let ware = ((towns[home].flags() >> 18) & 31) as usize;
    if special && (ware >= 24 || (crime == 1 && stock.is_none()) || (crime == 6 && towns.len() > 255)) {
        return None;
    }
    let mut bribes = 0;
    let mut changed = Vec::new();
    for city in [court, target] {
        if city == target && target != court && bribes > 1 {
            break;
        }
        for i in 0..4 {
            if towns[city].u8(0x6dc + i) == owner as u8 {
                towns[city].set_u8(0x6dc + i, 255);
                bribes += 1;
                if !changed.contains(&city) {
                    changed.push(city)
                }
            }
        }
    }
    let roll = u16::from_le_bytes([r.u8_at(0x12), r.u8_at(0x13)]);
    let acquit = bribes > 1 || (bribes == 1 && roll < 0x4e21);
    let human = m.u16(8) == 0;
    if acquit {
        let letter = if human {
            let mut p = [0u8; 8];
            p[0] = m.bytes[0x1d];
            p[1] = m.bytes[0x1c];
            p[4] = target as u8;
            p[6] = crime;
            Some((owner, court as u8, 0x4a, p))
        } else {
            None
        };
        return Some(InvestigationVerdict {
            keep: false,
            letter,
            notify: false,
            towns: changed,
        });
    }
    if crime == 1 {
        m.subtract_social_reputation_event(court as u32, 1)
    }
    let random = u16::from_le_bytes([r.u8_at(0x14), r.u8_at(0x15)]);
    let factor = if crime == 1 {
        u32::from(random % 21) + 30
    } else {
        u32::from(random % 7) + 2
    };
    let value = m.i32(0x46c).max(5000) as i64;
    let fine = (((value * i64::from(factor) / 500000) + 1) * 500).min(32750000) as u32;
    let mut goods = None;
    if special && crime == 1 {
        let lot = if crate::economy::prices::SMALL_LOT_WARES[ware] {
            200u32
        } else {
            2000
        };
        let amount = ((fine as f64) / (crate::economy::prices::BASE_PRICE[ware] as f64)) as u32;
        let amount = amount - (amount % lot);
        let stocks = stock.as_deref_mut()?;
        if amount <= stocks[ware] as u32 {
            stocks[ware] = stocks[ware].wrapping_sub(amount as i32);
            r.set_u8_at(0x11, 2);
            goods = Some(amount / lot)
        }
    }
    let letter = if human {
        let mut p = [0u8; 8];
        if let Some(quantity) = goods {
            p[0] = 1;
            p[2] = ware as u8;
            p[3] = quantity as u8
        } else {
            p[2..4].copy_from_slice(&((fine / 500) as u16).to_le_bytes())
        }
        p[4] = crime;
        p[5] = m.bytes[0x1c];
        p[6] = m.bytes[0x1d];
        if crime == 1 {
            p[7] = target as u8
        }
        Some((owner, court as u8, 0x4b, p))
    } else {
        None
    };
    if goods.is_some() {
        return Some(InvestigationVerdict {
            keep: false,
            letter,
            notify: human,
            towns: changed,
        });
    }
    r.due = r.due.wrapping_add(0x1400);
    r.kind = 1;
    r.set_u32_at(8, fine & 0x7fffffff);
    r.set_u32_at(0xc, tick & 0xffffff00);
    r.set_u8_at(0x10, 255);
    r.set_u8_at(0x11, owner as u8);
    r.set_u8_at(0x12, 0);
    r.set_u8_at(0x13, 0);
    r.set_u8_at(0x14, court as u8);
    r.set_u8_at(0x15, 0);
    r.set_u8_at(0x16, 0);
    r.set_u8_at(0x17, 0);
    Some(InvestigationVerdict {
        keep: true,
        letter,
        notify: human,
        towns: changed,
    })
}

/// Propietarios de los barcos de la Liga elegidos por 0x0050ac40.
/// Con empate prevalece el comerciante más tardío; la creación usa orden inverso.
pub fn factory_owners(towns: &[Town], merchants: &[Merchant], two: bool) -> Option<Vec<u8>> {
    if merchants.len() > u16::MAX as usize {
        return None;
    }
    let mut owners = [0u8; 2];
    let mut scores = [0u16; 2];
    for (i, m) in merchants.iter().enumerate() {
        if m.bytes.len() < 0x650 {
            return None;
        }
        if m.u16(8) == 0 {
            continue;
        }
        let town = towns.get(usize::from(m.bytes[0x19]))?;
        if town.flags() & 0x40 == 0 {
            continue;
        }
        let score = m.u16(0x478);
        if scores[1] <= score {
            if score < scores[0] {
                owners[1] = i as u8;
                scores[1] = score
            } else {
                owners[1] = owners[0];
                owners[0] = i as u8;
                scores[1] = scores[0];
                scores[0] = score
            }
        }
    }
    Some(if two {
        vec![owners[1], owners[0]]
    } else {
        vec![owners[0]]
    })
}

/// Fase 8 y búsqueda inicial sin bandas: prepara la salida y solicita ruta al puerto.
/// La solicitud debe ejecutarse con `ship::plan_route` antes de confirmar la transacción.
pub fn hunt_search_start_no_targets(
    r: &mut TaskRecord,
    convoy: &mut [u8],
    ships: &mut [Option<Vec<u8>>],
    ship_count: u16,
    sea_head: u16,
    port: (u16, u16),
    tick: u32,
    convoy_id: u16,
) -> Option<u16> {
    if r.kind != 0x34 || r.u8_at(0x15) != 8 || convoy.len() < 0x3c || usize::from(ship_count) > ships.len() {
        return None;
    }
    for ship in ships.iter().take(usize::from(ship_count)) {
        match ship {
            Some(s) if s.len() == 0x180 => {}
            None => {}
            _ => return None,
        }
    }
    let leader = u16::from_le_bytes([convoy[0x10], convoy[0x11]]);
    if leader >= ship_count {
        return None;
    }
    let mut cursor = u16::from_le_bytes([convoy[10], convoy[11]]);
    let mut chain = Vec::new();
    let mut seen = vec![false; usize::from(ship_count)];
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
        chain.push(i);
        cursor = u16::from_le_bytes([ship[6], ship[7]])
    }
    if ships[usize::from(leader)].as_ref()?.len() < 0x180 {
        return None;
    }
    let deadline = r.due.wrapping_add(0x1f00).wrapping_add(u32::from(convoy_id) * 0x4b);
    if deadline < tick {
        return None;
    }
    convoy[0x1e] = convoy[0x1c];
    convoy[0x1c..0x1e].copy_from_slice(&255u16.to_le_bytes());
    convoy[0x16..0x18].fill(0);
    convoy[0x12..0x14].copy_from_slice(&0x16u16.to_le_bytes());
    convoy[0x18] |= 0x20;
    let ship = ships[usize::from(leader)].as_mut()?;
    ship[0x1c..0x20].copy_from_slice(&(u32::from(port.0) << 16).to_le_bytes());
    ship[0x20..0x24].copy_from_slice(&(u32::from(port.1) << 16).to_le_bytes());
    ship[0x30..0x34].fill(0);
    let mut raw = ships
        .iter()
        .take(usize::from(ship_count))
        .flat_map(|s| s.clone().unwrap_or_else(|| vec![0; 0x180]))
        .collect::<Vec<_>>();
    crate::ships::order::XOrder {
        ships: &mut raw,
        count: ship_count,
        sea_head,
    }
    .insert(leader, u32::from(sea_head));
    for (i, image) in ships.iter_mut().take(usize::from(ship_count)).enumerate() {
        if let Some(ship) = image {
            ship.copy_from_slice(&raw[i * 0x180..(i + 1) * 0x180])
        }
    }
    for i in chain {
        let ship = ships[i].as_mut()?;
        ship[0x138..0x13a].fill(0);
        ship[0x1c..0x20].copy_from_slice(&(u32::from(port.0) << 16).to_le_bytes());
        ship[0x20..0x24].copy_from_slice(&(u32::from(port.1) << 16).to_le_bytes());
        let flags = u16::from_le_bytes([ship[0x3c], ship[0x3d]]) | 0x20a8;
        ship[0x3c..0x3e].copy_from_slice(&flags.to_le_bytes());
        ship[0x134..0x136].copy_from_slice(&0x16u16.to_le_bytes());
        ship[0x30..0x34].fill(0);
        ship[0x39] = 255
    }
    r.set_u8_at(0x15, 9);
    ships[usize::from(leader)].as_mut()?[0x3a..0x3c].fill(255);
    r.set_u32_at(0x10, deadline);
    r.due = r.due.wrapping_add(0x20);
    Some(leader)
}
