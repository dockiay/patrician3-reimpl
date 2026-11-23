//! Retorno al mar del convoy tras un combate (0x0050BFD0).
//! Primera vía: convoy conservado, líder válido y todos sus miembros vivos.
use super::{cargo::recompute_load, order::XOrder};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Effects {
    pub recount_incoming: bool,
    pub refresh_ship: Option<u16>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PirateCaptainNotice {
    pub recipient: u8,
    pub ship: u16,
    pub captain: u16,
    pub look: u16,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DissolvedEffects {
    pub recount_incoming: bool,
    pub refresh_ships: Vec<u16>,
    pub removed_routes: Vec<u16>,
    pub npc_losses: Vec<u16>,
    pub sunk_ships: Vec<u16>,
    pub pirate_notices: Vec<PirateCaptainNotice>,
}

/// 0x00504970: reconstruye por ciudad las 24 mercancías que llevan los
/// barcos sueltos en el mar y los convoyes comerciales en marcha.
/// Las listas cíclicas se rechazan sin modificar las ciudades.
pub fn recount_incoming(manager: &[u8], ships: &[u8], convoys: &[u8], towns: &mut [u8]) -> Result<(), &'static str> {
    if manager.len() < 0xf8 || ships.len() % 0x180 != 0 || convoys.len() % 0x3c != 0 || towns.len() % 0x9f8 != 0 {
        return Err("tablas truncadas");
    }
    let r16 = |b: &[u8], o: usize| u16::from_le_bytes([b[o], b[o + 1]]);
    let ns = usize::from(r16(manager, 0xf4));
    let nc = usize::from(r16(manager, 0xf6));
    let nt = towns.len() / 0x9f8;
    if ns > ships.len() / 0x180 || nc > convoys.len() / 0x3c {
        return Err("flota truncada");
    }
    let mut staged = towns.to_vec();
    for town in staged.chunks_exact_mut(0x9f8) {
        town[0x670..0x6d0].fill(0)
    }
    let add = |staged: &mut [u8], ship: usize, town: usize| {
        if town < nt {
            for ware in 0..24 {
                let to = town * 0x9f8 + 0x670 + ware * 4;
                let from = ship * 0x180 + 0x54 + ware * 4;
                let a = u32::from_le_bytes(staged[to..to + 4].try_into().unwrap());
                let b = u32::from_le_bytes(ships[from..from + 4].try_into().unwrap());
                staged[to..to + 4].copy_from_slice(&a.wrapping_add(b).to_le_bytes())
            }
        }
    };
    let mut seen = vec![false; ns];
    let mut id = r16(manager, 0xea);
    while usize::from(id) < ns {
        let i = usize::from(id);
        if seen[i] {
            return Err("lista marítima cíclica");
        }
        seen[i] = true;
        add(&mut staged, i, usize::from(ships[i * 0x180 + 0x38]));
        id = r16(ships, i * 0x180 + 6)
    }
    let mut seen_cv = vec![false; nc];
    let mut cv = r16(manager, 0xec);
    while usize::from(cv) < nc {
        let i = usize::from(cv);
        if seen_cv[i] {
            return Err("lista de convoyes cíclica");
        }
        seen_cv[i] = true;
        let at = i * 0x3c;
        if r16(convoys, at + 0x12) == 15 {
            let town = usize::from(convoys[at + 0x1f]);
            if town < nt {
                seen.fill(false);
                let mut ship = r16(convoys, at + 0xa);
                while usize::from(ship) < ns {
                    let j = usize::from(ship);
                    if seen[j] {
                        return Err("convoy cíclico");
                    }
                    seen[j] = true;
                    add(&mut staged, j, town);
                    ship = r16(ships, j * 0x180 + 6)
                }
            }
        }
        cv = r16(convoys, at + 8)
    }
    towns.copy_from_slice(&staged);
    Ok(())
}

/// Los cambios son atómicos. Se rechazan bajas, disolución, tareas hanseáticas
/// y rutas que requieran liberar memoria o limpiar tareas externas.
pub fn return_preserved(
    manager: &mut [u8],
    ships: &mut [u8],
    convoys: &mut [u8],
    id: u16,
    merchant_count: u16,
    credited: u8,
    player: u32,
    task_limit: u16,
    has_hanse_task: bool,
) -> Result<Effects, &'static str> {
    if manager.len() < 0xf8 || ships.len() % 0x180 != 0 || convoys.len() % 0x3c != 0 {
        return Err("flota truncada");
    }
    let r16 = |b: &[u8], o: usize| u16::from_le_bytes([b[o], b[o + 1]]);
    let n = usize::from(r16(manager, 0xf4));
    if n > ships.len() / 0x180 {
        return Err("tabla de barcos truncada");
    }
    let at = usize::from(id) * 0x3c;
    let cv = convoys.get(at..at + 0x3c).ok_or("convoy ausente")?;
    let owner = cv[0];
    let merchant = u16::from(owner) < merchant_count;
    if has_hanse_task {
        return Err("tarea hanseática pendiente");
    }
    if merchant && cv[0x14] & 0x10 != 0 {
        return Err("disolución pendiente");
    }
    let main = r16(cv, 0x10);
    let mut next = r16(cv, 0xa);
    let mut seen = vec![false; n];
    let mut members = Vec::new();
    while usize::from(next) < n {
        let i = usize::from(next);
        if seen[i] {
            return Err("convoy cíclico");
        }
        seen[i] = true;
        let s = &ships[i * 0x180..(i + 1) * 0x180];
        if i32::from_le_bytes(s[0x18..0x1c].try_into().unwrap()) < 1 {
            return Err("bajas de convoy pendientes");
        }
        if next != main && u32::from_le_bytes(s[0x24..0x28].try_into().unwrap()) != 0 {
            return Err("liberación de ruta pendiente");
        }
        if !merchant && r16(s, 0x132) < task_limit {
            return Err("limpieza de tarea de ruta pendiente");
        }
        members.push(next);
        next = r16(s, 6)
    }
    if usize::from(main) >= n || !members.contains(&main) || r16(ships, usize::from(main) * 0x180 + 0x40) == 0 {
        return Err("selección de líder pendiente");
    }
    let mut m = manager.to_vec();
    let mut s = ships.to_vec();
    let mut c = convoys.to_vec();
    c[at + 0x14] &= !0x20;
    let state = if merchant { 15u16 } else { 18u16 };
    c[at + 0x12..at + 0x14].copy_from_slice(&state.to_le_bytes());
    if !merchant {
        c[at + 0x16..at + 0x18].copy_from_slice(&0x0a00u16.to_le_bytes());
        c[at + 0x14] |= 2
    }
    c[at + 0x1c] = 0xff;
    c[at + 0x1d] = 0;
    let speed = u32::from_le_bytes(c[at + 0xc..at + 0x10].try_into().unwrap());
    for member in members {
        let pos = usize::from(member) * 0x180;
        let ship = &mut s[pos..pos + 0x180];
        ship[0x39] = 0xff;
        ship[0x3c..0x3e].fill(0xff);
        if member != main {
            ship[2..4].fill(0);
            ship[0x34..0x36].fill(0)
        }
        if !merchant {
            ship[0x136] &= 0xfc
        }
        ship[0x134..0x136].copy_from_slice(&state.to_le_bytes());
        recompute_load(ship);
        if merchant {
            ship[0x30..0x34].copy_from_slice(&speed.to_le_bytes())
        }
    }
    let head = r16(&m, 0xec);
    c[at + 8..at + 10].copy_from_slice(&head.to_le_bytes());
    c[at + 0x18..at + 0x1c].fill(0xff);
    m[0xec..0xee].copy_from_slice(&id.to_le_bytes());
    let sea = r16(&m, 0xea);
    XOrder {
        ships: &mut s,
        count: n as u16,
        sea_head: sea,
    }
    .insert(main, u32::from(sea));
    manager.copy_from_slice(&m);
    ships.copy_from_slice(&s);
    convoys.copy_from_slice(&c);
    Ok(Effects {
        recount_incoming: u16::from(credited) < merchant_count,
        refresh_ship: (u32::from(owner) == player).then_some(main),
    })
}

/// Retorno con disolución de un convoy comercial vivo. Conserva rutas
/// existentes. Conserva hundidos humanos en estado 0x11; para bajas automáticas
/// sin ruta se necesita la variante con contexto. Las demás bajas,
/// la planificación, las rutas y tareas hanseáticas aún se rechazan.
pub fn return_dissolved(
    manager: &mut [u8],
    ships: &mut [u8],
    convoys: &mut [u8],
    merchants: &mut [u8],
    id: u16,
    credited: u8,
    player: u32,
    town_count: u32,
    task_limit: u16,
    has_hanse_task: bool,
) -> Result<DissolvedEffects, &'static str> {
    return_dissolved_inner(
        manager,
        ships,
        convoys,
        merchants,
        id,
        credited,
        player,
        town_count,
        task_limit,
        has_hanse_task,
        None,
    )
}
/// Variante que tramita bajas automáticas sin listas auxiliares activas. El
/// llamador debe soltar los descriptores lógicos de `removed_routes` e informar
/// las misiones de `sunk_ships` con los capitanes anteriores y entregar `pirate_notices` al confirmar.
pub fn return_dissolved_with_removal(
    manager: &mut [u8],
    ships: &mut [u8],
    convoys: &mut [u8],
    merchants: &mut [u8],
    id: u16,
    credited: u8,
    player: u32,
    task_limit: u16,
    has_hanse_task: bool,
    captains: &mut [u8],
    towns: &mut [u8],
    pool_limit: u16,
) -> Result<DissolvedEffects, &'static str> {
    if towns.len() % 0x9f8 != 0 {
        return Err("ciudades truncadas");
    }
    let mut staged_cap = captains.to_vec();
    let mut staged_towns = towns.to_vec();
    let town_count = (towns.len() / 0x9f8) as u32;
    let result = return_dissolved_inner(
        manager,
        ships,
        convoys,
        merchants,
        id,
        credited,
        player,
        town_count,
        task_limit,
        has_hanse_task,
        Some((&mut staged_cap, &mut staged_towns, pool_limit, None, None)),
    )?;
    captains.copy_from_slice(&staged_cap);
    towns.copy_from_slice(&staged_towns);
    Ok(result)
}
/// Como la variante anterior, incluyendo los once grupos de barcos de cada comerciante.
/// Los vectores conservan su capacidad lógica; los recuentos en merchant indican la parte activa.
/// El llamador debe informar las misiones usando `sunk_ships`, el estado anterior de capitanes y entregar `pirate_notices`.
pub fn return_dissolved_with_groups(
    manager: &mut [u8],
    ships: &mut [u8],
    convoys: &mut [u8],
    merchants: &mut [u8],
    id: u16,
    credited: u8,
    player: u32,
    task_limit: u16,
    has_hanse_task: bool,
    captains: &mut [u8],
    towns: &mut [u8],
    pool_limit: u16,
    groups: &mut [[Vec<u16>; 11]],
) -> Result<DissolvedEffects, &'static str> {
    if towns.len() % 0x9f8 != 0 || groups.len() != merchants.len() / 0x650 {
        return Err("tablas de baja truncadas");
    }
    let mut staged_cap = captains.to_vec();
    let mut staged_towns = towns.to_vec();
    let mut staged_groups = groups.to_vec();
    let town_count = (towns.len() / 0x9f8) as u32;
    let result = return_dissolved_inner(
        manager,
        ships,
        convoys,
        merchants,
        id,
        credited,
        player,
        town_count,
        task_limit,
        has_hanse_task,
        Some((
            &mut staged_cap,
            &mut staged_towns,
            pool_limit,
            Some(&mut staged_groups),
            None,
        )),
    )?;
    captains.copy_from_slice(&staged_cap);
    towns.copy_from_slice(&staged_towns);
    groups.clone_from_slice(&staged_groups);
    Ok(result)
}
/// Añade las cabezas de las cinco bandas al contexto, sin dereferenciar punteros x86.
pub fn return_dissolved_with_bands(
    manager: &mut [u8],
    ships: &mut [u8],
    convoys: &mut [u8],
    merchants: &mut [u8],
    id: u16,
    credited: u8,
    player: u32,
    task_limit: u16,
    has_hanse_task: bool,
    captains: &mut [u8],
    towns: &mut [u8],
    pool_limit: u16,
    groups: &mut [[Vec<u16>; 11]],
    bands: &mut [Option<u16>; 5],
) -> Result<DissolvedEffects, &'static str> {
    if towns.len() % 0x9f8 != 0 || groups.len() != merchants.len() / 0x650 {
        return Err("tablas de baja truncadas");
    }
    let mut caps = captains.to_vec();
    let mut cities = towns.to_vec();
    let mut lists = groups.to_vec();
    let mut heads = *bands;
    let town_count = (towns.len() / 0x9f8) as u32;
    let result = return_dissolved_inner(
        manager,
        ships,
        convoys,
        merchants,
        id,
        credited,
        player,
        town_count,
        task_limit,
        has_hanse_task,
        Some((&mut caps, &mut cities, pool_limit, Some(&mut lists), Some(&mut heads))),
    )?;
    captains.copy_from_slice(&caps);
    towns.copy_from_slice(&cities);
    groups.clone_from_slice(&lists);
    *bands = heads;
    Ok(result)
}
fn return_dissolved_inner(
    manager: &mut [u8],
    ships: &mut [u8],
    convoys: &mut [u8],
    merchants: &mut [u8],
    id: u16,
    credited: u8,
    player: u32,
    town_count: u32,
    task_limit: u16,
    has_hanse_task: bool,
    mut removal: Option<(
        &mut Vec<u8>,
        &mut Vec<u8>,
        u16,
        Option<&mut Vec<[Vec<u16>; 11]>>,
        Option<&mut [Option<u16>; 5]>,
    )>,
) -> Result<DissolvedEffects, &'static str> {
    if manager.len() < 0xf8
        || ships.len() % 0x180 != 0
        || convoys.len() % 0x3c != 0
        || merchants.len() % 0x650 != 0
        || merchants.len() / 0x650 > u16::MAX as usize
    {
        return Err("tablas truncadas");
    }
    let r16 = |b: &[u8], o: usize| u16::from_le_bytes([b[o], b[o + 1]]);
    let n = usize::from(r16(manager, 0xf4));
    let nc = usize::from(r16(manager, 0xf6));
    let nm = merchants.len() / 0x650;
    let at = usize::from(id) * 0x3c;
    if n > ships.len() / 0x180 || nc > convoys.len() / 0x3c || usize::from(id) >= nc {
        return Err("flota truncada");
    }
    let owner = convoys[at];
    // Convoy NPC: ensure_leader 0x5017F0 falla cuando no queda ningún barco vivo y 0x50C049 salta
    // al mismo bucle 0x50C5A6, sin consulta 0x503B00. Solo se admite sin supervivientes.
    let npc = usize::from(owner) >= nm;
    if npc {
        let mut cursor = r16(convoys, at + 0xa);
        let mut seen = vec![false; n];
        while usize::from(cursor) < n {
            let i = usize::from(cursor);
            if seen[i] {
                return Err("convoy cíclico");
            }
            seen[i] = true;
            if i32::from_le_bytes(ships[i * 0x180 + 0x18..i * 0x180 + 0x1c].try_into().unwrap()) >= 1 {
                return Err("disolución pirata con supervivientes pendiente");
            }
            cursor = r16(ships, i * 0x180 + 6);
        }
    } else if convoys[at + 0x14] & 0x10 == 0 && usize::from(r16(convoys, at + 0xa)) < n {
        return Err("convoy no disoluble");
    }
    if has_hanse_task && !npc {
        return Err("tarea hanseática pendiente");
    }
    let mut m = manager.to_vec();
    let mut s = ships.to_vec();
    let mut c = convoys.to_vec();
    let mut ms = merchants.to_vec();
    let mut seen = vec![false; n];
    let mut refresh = Vec::new();
    let mut removed_routes = Vec::new();
    let mut npc_losses = Vec::new();
    let mut sunk_ships = Vec::new();
    let mut pirate_notices = Vec::new();
    let mut losses = 0u32;
    c[at + 0x14] &= !0x20;
    if npc {
        // Efectos de ensure_leader fallido: fuera del orden x el líder en 0xF/0x10/0x12 y líder 0xFFFF.
        let main = r16(&c, at + 0x10);
        if usize::from(main) < n && matches!(r16(&s, usize::from(main) * 0x180 + 0x134), 0xf | 0x10 | 0x12) {
            let head = r16(&m, 0xea);
            XOrder { ships: &mut s, count: n as u16, sea_head: head }.remove(main);
        }
        c[at + 0x10..at + 0x12].fill(0xff);
        c[at + 0x12..at + 0x14].copy_from_slice(&0x12u16.to_le_bytes());
        c[at + 0x16..at + 0x18].copy_from_slice(&0x0a00u16.to_le_bytes());
    } else {
        c[at + 0x12..at + 0x14].copy_from_slice(&15u16.to_le_bytes());
    }
    let mut cur = r16(&c, at + 0xa);
    while usize::from(cur) < n {
        let i = usize::from(cur);
        if seen[i] {
            return Err("convoy cíclico");
        }
        seen[i] = true;
        let pos = i * 0x180;
        let next = r16(&s, pos + 6);
        c[at + 0xa..at + 0xc].copy_from_slice(&next.to_le_bytes());
        let ship = &mut s[pos..pos + 0x180];
        if i32::from_le_bytes(ship[0x18..0x1c].try_into().unwrap()) < 1 {
            sunk_ships.push(cur);
            let owner = usize::from(ship[0]);
            if owner >= nm {
                if usize::from(credited) < nm {
                    let cap = r16(ship, 0x42);
                    let has_captain = cap < r16(&m, 0xf2);
                    if has_captain {
                        if town_count > 40 {
                            return Err("ciudades de reputación excesivas");
                        }
                        let (caps, _, _, _, _) = removal.as_ref().ok_or("capitán de baja ausente")?;
                        let at = usize::from(cap) * 16;
                        let person = caps.get(at..at + 16).ok_or("capitán de baja truncado")?;
                        pirate_notices.push(PirateCaptainNotice {
                            recipient: credited,
                            ship: cur,
                            captain: cap,
                            look: (u16::from(person[2]) << 8) | u16::from(person[3]),
                        });
                    }
                    let at = usize::from(credited) * 0x650;
                    let mut merchant = crate::ai::reputation::Merchant {
                        bytes: ms[at..at + 0x650].to_vec(),
                    };
                    let env = crate::ai::reputation::RepEnv {
                        town_count,
                        towns: &[],
                        renown: [0, 0, -512],
                        byte_52c: 0,
                    };
                    merchant.reward_pirate_ship_defeated(&env, u32::from(has_captain));
                    ms[at..at + 0x650].copy_from_slice(&merchant.bytes)
                }
                npc_losses.push(cur)
            }
            ship[0x3c..0x3e].fill(0xff);
            ship[8..10].fill(0xff);
            if owner < nm && r16(&ms, owner * 0x650 + 8) == 0 {
                ship[6..8].fill(0xff);
                ship[0x134..0x136].copy_from_slice(&0x11u16.to_le_bytes());
            } else {
                if ship[0x24..0x28] != [0; 4] {
                    removed_routes.push(cur)
                }
                let (caps, towns, limit, groups, bands) = removal.as_mut().ok_or("baja automática pendiente")?;
                if caps.len() != usize::from(r16(&m, 0xf2)) * 16 || towns.len() != town_count as usize * 0x9f8 {
                    return Err("tablas de baja truncadas");
                }
                let cap = r16(ship, 0x42);
                let count = r16(&m, 0xf2);
                if cap < count {
                    let mut id = r16(&m, 0xe0);
                    let mut seen = vec![false; usize::from(count)];
                    while id < count {
                        if id == cap {
                            break;
                        }
                        let i = usize::from(id);
                        if seen[i] {
                            return Err("lista libre de capitanes cíclica");
                        }
                        seen[i] = true;
                        id = r16(caps, i * 16)
                    }
                }
                if r16(ship, 0x132) < *limit {
                    return Err("ranura de baja pendiente");
                }
                if owner < nm {
                    if let Some(groups) = groups.as_deref() {
                        for j in 0..11 {
                            let count = usize::from(r16(&ms, owner * 0x650 + 0x60c + j * 2) & 0x7fff);
                            let list = &groups[owner][j];
                            if list.len() < count || list[..count].iter().any(|&id| usize::from(id) >= n) {
                                return Err("grupo de comerciante inválido");
                            }
                        }
                    } else if (0..11).any(|j| r16(&ms, owner * 0x650 + 0x60c + j * 2) & 0x7fff != 0) {
                        return Err("listas de comerciante pendientes");
                    }
                }

                // La rutina original recorre estas cadenas sin límite; validar ciclos antes de llamarla.
                let walk = |mut cursor: u16, offset: usize| -> Result<bool, &'static str> {
                    let mut seen = vec![false; n];
                    let mut found = false;
                    while usize::from(cursor) < n {
                        let i = usize::from(cursor);
                        if seen[i] {
                            return Err("lista naval cíclica");
                        }
                        seen[i] = true;
                        found |= cursor == cur;
                        cursor = r16(&s, i * 0x180 + offset)
                    }
                    Ok(found)
                };
                for head in [r16(&m, 0xe2), r16(&m, 0xe6), r16(&m, 0xe8), r16(&m, 0xea)] {
                    walk(head, 6)?;
                }
                if owner < nm {
                    walk(r16(&ms, owner * 0x650 + 0xe), 4)?;
                }
                for t in 0..town_count as usize {
                    walk(r16(towns, t * 0x9f8 + 0x81c), 6)?;
                    walk(r16(towns, t * 0x9f8 + 0x81e), 6)?;
                }
                if let Some(heads) = bands.as_deref() {
                    for j in 0..5 {
                        let pointer = u32::from_le_bytes(m[0xc + j * 4..0x10 + j * 4].try_into().unwrap());
                        if (pointer != 0) != heads[j].is_some() {
                            return Err("banda y puntero desincronizados");
                        }
                        if pointer != 0
                            && (0..j).any(|k| {
                                u32::from_le_bytes(m[0xc + k * 4..0x10 + k * 4].try_into().unwrap()) == pointer
                            })
                        {
                            return Err("puntero de banda compartido");
                        }
                        if let Some(head) = heads[j] {
                            walk(head, 4)?;
                        }
                    }
                }
                let mut band_next = r16(&s, pos + 4);
                // 0x50C886 separa el enlace del convoy antes de remove_ship.
                s[pos + 6..pos + 8].fill(0xff);
                let mut rw = super::remove::RemoveWorld {
                    manager: &mut m,
                    ships: &mut s,
                    captains: caps,
                    convoys: &mut c,
                    merchants: &mut ms,
                    towns,
                    pool_limit: *limit,
                    merchant_count: nm as u16,
                    town_count,
                    ui_a: false,
                    ui_b: false,
                };
                let events = super::remove::remove_ship(&mut rw, u32::from(cur));
                for event in events {
                    match event {
                        super::remove::RemoveEvent::ClearRoute(_) => {
                            let ship = &mut rw.ships[pos..pos + 0x180];
                            if ship[0x24..0x28] != [0; 4] && ship[0x36] != 0 {
                                ship[0x36] = 0
                            }
                            ship[0x24..0x28].fill(0);
                            ship[0x34..0x36].fill(0);
                            ship[2..4].fill(0)
                        }
                        super::remove::RemoveEvent::OrderRemove(_) => {
                            let head = r16(rw.manager, 0xea);
                            super::order::XOrder {
                                ships: rw.ships,
                                count: n as u16,
                                sea_head: head,
                            }
                            .remove(cur)
                        }
                        super::remove::RemoveEvent::ConvoyRelease(pointer, _) => {
                            let heads = bands.as_deref_mut().ok_or("banda de baja pendiente")?;
                            let j = (0..5)
                                .find(|&j| {
                                    u32::from_le_bytes(rw.manager[0xc + j * 4..0x10 + j * 4].try_into().unwrap())
                                        == pointer
                                })
                                .ok_or("banda desconocida")?;
                            let mut cursor = heads[j].ok_or("banda ausente")?;
                            let mut previous = None;
                            while usize::from(cursor) < n {
                                if cursor == cur {
                                    if let Some(prev) = previous {
                                        let at = usize::from(prev) * 0x180 + 4;
                                        rw.ships[at..at + 2].copy_from_slice(&band_next.to_le_bytes())
                                    } else {
                                        heads[j] = Some(band_next)
                                    }
                                    band_next = u16::MAX;
                                    break;
                                }
                                previous = Some(cursor);
                                cursor = r16(rw.ships, usize::from(cursor) * 0x180 + 4);
                            }
                        }
                        super::remove::RemoveEvent::MerchantChanged(owner) => {
                            if let Some(groups) = groups.as_deref_mut() {
                                let owner = owner as usize;
                                let at = owner * 0x650;
                                let mut merchant = crate::ai::reputation::Merchant {
                                    bytes: rw.merchants[at..at + 0x650].to_vec(),
                                };
                                merchant.prune_foreign_ships_from_groups(owner as u8, &mut groups[owner], &|id| {
                                    rw.ships[usize::from(id) * 0x180]
                                });
                                rw.merchants[at..at + 0x650].copy_from_slice(&merchant.bytes);
                            }
                        }
                        _ => return Err("efecto de baja pendiente"),
                    }
                }
            }
            losses += 1;
            cur = next;
            continue;
        }
        let route = u32::from_le_bytes(ship[0x24..0x28].try_into().unwrap());
        let ship_owner = usize::from(ship[0]);
        if ship_owner < nm {
            let mo = ship_owner * 0x650;
            if r16(&ms, mo + 8) != 0 && u32::from(ship[0x38]) >= town_count {
                if route != 0 {
                    return Err("liberación de ruta pendiente");
                }
                ship[0x38] = ms[mo + 0x19];
                ship[0x13a] = ms[mo + 0x19];
                ship[2..4].fill(0);
                ship[0x34..0x36].fill(0)
            }
            if route == 0 && u32::from(ship[0x38]) < town_count {
                return Err("planificación de ruta pendiente");
            }
        } else if r16(ship, 0x132) < task_limit {
            return Err("limpieza de tarea de ruta pendiente");
        }
        ship[0x3c..0x3e].fill(0xff);
        ship[8..10].fill(0xff);
        let balance = i32::from_le_bytes(c[at + 0x20..at + 0x24].try_into().unwrap());
        if balance > 0 && ship[0] == owner {
            let mo = usize::from(owner) * 0x650;
            let gold = i32::from_le_bytes(ms[mo..mo + 4].try_into().unwrap()).wrapping_add(balance);
            ms[mo..mo + 4].copy_from_slice(&gold.to_le_bytes());
            c[at + 0x20..at + 0x24].fill(0)
        }
        ship[0x134..0x136].copy_from_slice(&15u16.to_le_bytes());
        if u32::from(ship[0]) == player {
            refresh.push(cur)
        }
        recompute_load(ship);
        ship[0x39] = 0xff;
        if r16(ship, 0x42) >= r16(&m, 0xf2) {
            ship[0x3d] |= 0x10;
            ship[0x15c] = 0
        }
        if ship_owner < nm {
            let speed = if route == 0 {
                0
            } else {
                super::motion::get_speed(ship)
                    .map_err(|_| "velocidad inválida")?
                    .wrapping_mul(64)
                    / 115
                    + 4
            };
            ship[0x30..0x34].copy_from_slice(&speed.to_le_bytes())
        } else {
            ship[0x136] &= 0xfc
        }
        let sea = r16(&m, 0xea);
        XOrder {
            ships: &mut s,
            count: n as u16,
            sea_head: sea,
        }
        .insert(cur, u32::from(sea));
        s[pos + 6..pos + 8].copy_from_slice(&sea.to_le_bytes());
        m[0xea..0xec].copy_from_slice(&cur.to_le_bytes());
        cur = next;
    }
    if losses != 0 {
        if !npc {
            let owner_at = usize::from(owner) * 0x650 + 0x1b;
            ms[owner_at] = u32::from(ms[owner_at]).saturating_sub(losses) as u8;
        }
        if usize::from(credited) < nm {
            let at = usize::from(credited) * 0x650 + 0x1b;
            let value = ms[at];
            ms[at] = if 250u32.wrapping_sub(u32::from(value)) > losses {
                value.wrapping_add(losses as u8)
            } else {
                250
            };
        }
        // 0x50C99B: con convoy NPC, recompensa adicional sin capitán al acreditado.
        if npc && usize::from(credited) < nm {
            reward(&mut ms, credited, town_count);
        }
    }
    // release_convoy normaliza la cabeza incluso si ya no señala un barco.
    c[at + 0xa..at + 0xc].fill(0xff);
    // El original retira el convoy de la lista activa y lo inserta en libres.
    let mut seen_cv = vec![false; nc];
    let mut cursor = r16(&m, 0xec);
    let mut prev = None;
    while usize::from(cursor) < nc {
        let i = usize::from(cursor);
        if seen_cv[i] {
            return Err("lista de convoyes cíclica");
        }
        seen_cv[i] = true;
        let following = r16(&c, i * 0x3c + 8);
        if cursor == id {
            if let Some(p) = prev {
                c[p * 0x3c + 8..p * 0x3c + 10].copy_from_slice(&following.to_le_bytes())
            } else {
                m[0xec..0xee].copy_from_slice(&following.to_le_bytes())
            }
            break;
        }
        prev = Some(i);
        cursor = following
    }
    seen_cv.fill(false);
    cursor = r16(&m, 0xe4);
    let free = cursor;
    let mut already = false;
    while usize::from(cursor) < nc {
        let i = usize::from(cursor);
        if seen_cv[i] {
            return Err("lista libre cíclica");
        }
        seen_cv[i] = true;
        if cursor == id {
            already = true;
            break;
        }
        cursor = r16(&c, i * 0x3c + 8)
    }
    if !already {
        c[at + 8..at + 10].copy_from_slice(&free.to_le_bytes());
        m[0xe4..0xe6].copy_from_slice(&id.to_le_bytes())
    }
    c[at + 0x12..at + 0x14].copy_from_slice(&0xffu16.to_le_bytes());
    manager.copy_from_slice(&m);
    ships.copy_from_slice(&s);
    convoys.copy_from_slice(&c);
    merchants.copy_from_slice(&ms);
    Ok(DissolvedEffects {
        recount_incoming: usize::from(credited) < nm,
        refresh_ships: refresh,
        removed_routes,
        npc_losses,
        sunk_ships,
        pirate_notices,
    })
}

/// Efectos de `return_npc_with_losses` que confirma el llamante: informes 0x4EC864 de
/// `sunk_ships` (con las imágenes previas), descriptores de `removed_routes` y recuento.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NpcReturnEffects {
    pub sunk_ships: Vec<u16>,
    pub removed_routes: Vec<u16>,
    pub recount_incoming: bool,
}

/// Rama NPC de 0x0050BFD0 (0x50C0EE..0x50C3CB, 0x50C538 y la cola 0x50C8F6) para un convoy
/// sin comerciante con barcos hundidos. `ensure_leader` 0x5017F0 se ejecuta al principio y, si el
/// líder no sobrevive al recorrido, otra vez al final (líder hundido con supervivientes; un capitán
/// nuevo usa 0x5097C0(1) con `captain`). Atómica. Rechaza: convoy sin barco vivo (es la disolución
/// 0x50C5A6), ranuras del pool de tareas, efectos de `remove_ship` sin traducir, capitanes que
/// sobreviven a la baja, asignación sin recursos o con la lista libre agotada y el convoy vaciado
/// por las bajas (inalcanzable: `ensure_leader` solo acepta un líder vivo, que no se da de baja).
#[allow(clippy::too_many_arguments)]
pub fn return_npc_with_losses(
    manager: &mut [u8],
    ships: &mut [u8],
    convoys: &mut [u8],
    merchants: &mut [u8],
    id: u16,
    credited: u8,
    task_limit: u16,
    captains: &mut Vec<u8>,
    towns: &mut [u8],
    band_heads: &mut [Option<u16>; 5],
    captain: Option<crate::economy::pirate_group_callbacks::CaptainResources>,
) -> Result<NpcReturnEffects, &'static str> {
    if manager.len() < 0xf8
        || ships.len() % 0x180 != 0
        || convoys.len() % 0x3c != 0
        || merchants.len() % 0x650 != 0
        || towns.len() % 0x9f8 != 0
    {
        return Err("tablas truncadas");
    }
    let r16 = |b: &[u8], o: usize| u16::from_le_bytes([b[o], b[o + 1]]);
    let n = usize::from(r16(manager, 0xf4));
    let nc = usize::from(r16(manager, 0xf6));
    let nm = merchants.len() / 0x650;
    let town_count = (towns.len() / 0x9f8) as u32;
    let at = usize::from(id) * 0x3c;
    if n > ships.len() / 0x180 || usize::from(id) >= nc || at + 0x3c > convoys.len() {
        return Err("flota truncada");
    }
    if usize::from(convoys[at]) < nm {
        return Err("convoy comercial en rama NPC");
    }
    if captains.len() != usize::from(r16(manager, 0xf2)) * 16 || town_count > 40 {
        return Err("tablas de baja truncadas");
    }
    let alive = |s: &[u8], i: u16| i32::from_le_bytes(s[usize::from(i) * 0x180 + 0x18..][..4].try_into().unwrap()) >= 1;
    let mut seen = vec![false; n];
    let mut cur = r16(convoys, at + 0xa);
    while usize::from(cur) < n {
        let i = usize::from(cur);
        if seen[i] {
            return Err("convoy cíclico");
        }
        seen[i] = true;
        cur = r16(ships, i * 0x180 + 6);
    }
    let mut m = manager.to_vec();
    let mut s = ships.to_vec();
    let mut c = convoys.to_vec();
    let mut ms = merchants.to_vec();
    let mut caps = captains.to_vec();
    let mut tw = towns.to_vec();
    let mut heads = *band_heads;
    c[at + 0x14] &= !0x20;
    // 0x50C02F: ensure_leader; si falla, el original disuelve (0x50C5A6).
    if !ensure_leader(&mut m, &mut s, &mut c, &mut ms, &mut caps, &mut tw, id, task_limit, captain)? {
        return Err("convoy NPC sin barco vivo: disolución");
    }
    c[at + 0x12..at + 0x14].copy_from_slice(&0x12u16.to_le_bytes());
    c[at + 0x16..at + 0x18].copy_from_slice(&0x0a00u16.to_le_bytes());
    c[at + 0x1c] = 0xff;
    c[at + 0x1d] = 0;
    let mut effects = NpcReturnEffects { sunk_ships: Vec::new(), removed_routes: Vec::new(), recount_incoming: false };
    let mut losses = 0u32;
    // [esp+0x13] de 0x50BFD0: el líder no se ha visto vivo en el recorrido.
    let mut lost_main = true;
    // Enlace actual: None = convoy+0xA, Some(i) = barco i +6.
    let mut link: Option<usize> = None;
    loop {
        let cur = match link {
            None => r16(&c, at + 0xa),
            Some(p) => r16(&s, p * 0x180 + 6),
        };
        if usize::from(cur) >= n {
            break;
        }
        let pos = usize::from(cur) * 0x180;
        s[pos + 0x3c..pos + 0x3e].fill(0xff);
        s[pos + 0x39] = 0xff;
        s[pos + 0x136] &= 0xfc;
        if r16(&s, pos + 0x132) < task_limit {
            return Err("limpieza de tarea de ruta pendiente");
        }
        let route = s[pos + 0x24..pos + 0x28] != [0; 4];
        if !alive(&s, cur) {
            if cur == r16(&c, at + 0x10) {
                lost_main = true;
            }
            effects.sunk_ships.push(cur);
            let next = [s[pos + 6], s[pos + 7]];
            match link {
                None => c[at + 0xa..at + 0xc].copy_from_slice(&next),
                Some(p) => s[p * 0x180 + 6..p * 0x180 + 8].copy_from_slice(&next),
            }
            if route {
                effects.removed_routes.push(cur)
            }
            // clear_route 0x516840: el llamante libera el descriptor propio.
            s[pos + 0x36] = 0;
            s[pos + 0x24..pos + 0x28].fill(0);
            s[pos + 0x34..pos + 0x36].fill(0);
            s[pos + 2..pos + 4].fill(0);
            let mut band_next = r16(&s, pos + 4);
            let mut rw = super::remove::RemoveWorld {
                manager: &mut m,
                ships: &mut s,
                captains: &mut caps,
                convoys: &mut c,
                merchants: &mut ms,
                towns: &mut tw,
                pool_limit: task_limit,
                merchant_count: nm as u16,
                town_count,
                ui_a: false,
                ui_b: false,
            };
            for event in super::remove::remove_ship(&mut rw, u32::from(cur)) {
                match event {
                    super::remove::RemoveEvent::ClearRoute(_) => {}
                    super::remove::RemoveEvent::OrderRemove(_) => {
                        let head = r16(rw.manager, 0xea);
                        XOrder { ships: rw.ships, count: n as u16, sea_head: head }.remove(cur)
                    }
                    super::remove::RemoveEvent::ConvoyRelease(pointer, _) => {
                        let j = (0..5)
                            .find(|&j| {
                                u32::from_le_bytes(rw.manager[0xc + j * 4..0x10 + j * 4].try_into().unwrap()) == pointer
                            })
                            .ok_or("banda desconocida")?;
                        let mut cursor = heads[j].ok_or("banda ausente")?;
                        let mut previous = None;
                        let mut guard = vec![false; n];
                        while usize::from(cursor) < n {
                            if guard[usize::from(cursor)] {
                                return Err("cadena de banda cíclica");
                            }
                            guard[usize::from(cursor)] = true;
                            if cursor == cur {
                                if let Some(prev) = previous {
                                    let at = usize::from(prev) * 0x180 + 4;
                                    rw.ships[at..at + 2].copy_from_slice(&band_next.to_le_bytes())
                                } else {
                                    heads[j] = Some(band_next)
                                }
                                band_next = u16::MAX;
                                break;
                            }
                            previous = Some(cursor);
                            cursor = r16(rw.ships, usize::from(cursor) * 0x180 + 4);
                        }
                    }
                    _ => return Err("efecto de baja pirata pendiente"),
                }
            }
            if usize::from(credited) < nm {
                // remove_ship deja +0x42=0xFFFF: la recompensa va sin capitán y no hay cartas 0x54/0x2A.
                if r16(&s, pos + 0x42) < r16(&m, 0xf2) {
                    return Err("carta de capitán pirata pendiente");
                }
                reward(&mut ms, credited, town_count);
            }
            losses += 1;
            continue;
        }
        if cur == r16(&c, at + 0x10) {
            lost_main = false;
        } else {
            if route {
                effects.removed_routes.push(cur)
            }
            s[pos + 0x36] = 0;
            s[pos + 0x24..pos + 0x28].fill(0);
            s[pos + 0x34..pos + 0x36].fill(0);
            s[pos + 2..pos + 4].fill(0);
        }
        s[pos + 0x134..pos + 0x136].copy_from_slice(&0x12u16.to_le_bytes());
        recompute_load(&mut s[pos..pos + 0x180]);
        link = Some(usize::from(cur));
    }
    c[at + 0x14] |= 2;
    if usize::from(r16(&c, at + 0xa)) >= n {
        return Err("convoy pirata vaciado por bajas pendiente");
    }
    // 0x50C3B3: el líder no apareció vivo, se vuelve a elegir.
    if lost_main && !ensure_leader(&mut m, &mut s, &mut c, &mut ms, &mut caps, &mut tw, id, task_limit, captain)? {
        return Err("convoy NPC sin líder tras las bajas");
    }
    let head = r16(&m, 0xec);
    c[at + 8..at + 10].copy_from_slice(&head.to_le_bytes());
    c[at + 0x18..at + 0x1c].fill(0xff);
    m[0xec..0xee].copy_from_slice(&id.to_le_bytes());
    let sea = r16(&m, 0xea);
    let main = r16(&c, at + 0x10);
    if usize::from(main) < n {
        XOrder { ships: &mut s, count: n as u16, sea_head: sea }.insert(main, u32::from(sea));
    }
    if losses != 0 && usize::from(credited) < nm {
        let mo = usize::from(credited) * 0x650 + 0x1b;
        let value = ms[mo];
        ms[mo] = if 250u32.wrapping_sub(u32::from(value)) > losses { value.wrapping_add(losses as u8) } else { 250 };
        reward(&mut ms, credited, town_count);
    }
    effects.recount_incoming = usize::from(credited) < nm;
    manager.copy_from_slice(&m);
    ships.copy_from_slice(&s);
    convoys.copy_from_slice(&c);
    merchants.copy_from_slice(&ms);
    *captains = caps;
    towns.copy_from_slice(&tw);
    *band_heads = heads;
    Ok(effects)
}

/// `ensure_leader` 0x5017F0 (`convoy::pick_main`) sobre las listas vivas de un convoy NPC.
#[allow(clippy::too_many_arguments)]
fn ensure_leader(
    manager: &mut [u8],
    ships: &mut [u8],
    convoys: &mut [u8],
    merchants: &mut [u8],
    captains: &mut Vec<u8>,
    towns: &mut [u8],
    id: u16,
    task_limit: u16,
    captain: Option<crate::economy::pirate_group_callbacks::CaptainResources>,
) -> Result<bool, &'static str> {
    let before = (manager.to_vec(), ships.to_vec(), convoys.to_vec(), captains.clone());
    match pick_leader(manager, ships, convoys, merchants, captains, towns, id, task_limit, captain)? {
        Some(ok) => Ok(ok),
        None => {
            // 0x5097C0 con la lista libre agotada amplía el pool en 64 entradas antes de asignar.
            // La asignación es lo único de pick_main que lee el pool: se repite sobre el pool ampliado.
            manager.copy_from_slice(&before.0);
            ships.copy_from_slice(&before.1);
            convoys.copy_from_slice(&before.2);
            *captains = before.3;
            let old = u16::from_le_bytes([manager[0xf2], manager[0xf3]]);
            let new = old.checked_add(64).ok_or("pool de capitanes no ampliable")?;
            for i in old..new {
                let mut entry = [0xffu8; 16];
                entry[..2].copy_from_slice(&(i + 1).to_le_bytes());
                captains.extend_from_slice(&entry);
            }
            manager[0xf2..0xf4].copy_from_slice(&new.to_le_bytes());
            pick_leader(manager, ships, convoys, merchants, captains, towns, id, task_limit, captain)?
                .ok_or("pool de capitanes no ampliable")
        }
    }
}

/// Un intento de `pick_main`; `None` si la asignación necesita ampliar el pool.
#[allow(clippy::too_many_arguments)]
fn pick_leader(
    manager: &mut [u8],
    ships: &mut [u8],
    convoys: &mut [u8],
    merchants: &mut [u8],
    captains: &mut [u8],
    towns: &mut [u8],
    id: u16,
    task_limit: u16,
    captain: Option<crate::economy::pirate_group_callbacks::CaptainResources>,
) -> Result<Option<bool>, &'static str> {
    let merchant_count = (merchants.len() / 0x650) as u16;
    let town_count = (towns.len() / 0x9f8) as u32;
    let mut hooks = super::convoy::LeaderHooks::new(captain, true);
    let mut remove = super::remove::RemoveWorld {
        manager,
        ships,
        captains,
        convoys,
        merchants,
        towns,
        pool_limit: task_limit,
        merchant_count,
        town_count,
        ui_a: false,
        ui_b: false,
    };
    let mut tick = super::tick::TickWorld { remove: &mut remove, time: 0, player: u32::MAX };
    let ok = super::convoy::pick_main(&mut tick, id, &mut hooks);
    if hooks.needs_growth {
        return Ok(None);
    }
    if hooks.unsupported {
        return Err("efecto de líder pirata pendiente");
    }
    for ship in hooks.inserts {
        super::order::insert_from_tick(&mut tick, ship, 0xFFFF);
    }
    Ok(Some(ok))
}

/// `merchant_reward_pirate_ship_defeated` 0x4F8D90 sin capitán (`0x672DDE` = -512).
fn reward(ms: &mut [u8], credited: u8, town_count: u32) {
    let mo = usize::from(credited) * 0x650;
    let mut merchant = crate::ai::reputation::Merchant { bytes: ms[mo..mo + 0x650].to_vec() };
    let env = crate::ai::reputation::RepEnv { town_count, towns: &[], renown: [0, 0, -512], byte_52c: 0 };
    merchant.reward_pirate_ship_defeated(&env, 0);
    ms[mo..mo + 0x650].copy_from_slice(&merchant.bytes);
}

/// Byte 0x0067285B: marca `tarea+0xE` de la caza cuando no la tenía.
pub const HUNT_MARK: u8 = 2;
/// Byte 0x0067285C: marca `tarea+0xE` cuando el convoy vuelve sin líder aceptable.
pub const HUNT_UNLED_MARK: u8 = 3;
/// Byte 0x00672854: clase de los nodos de bajas de la caza.
pub const HUNT_LOSS_CLASS: u8 = 0;

/// Nodo de 8 bytes (`operator new(8)` + 0x4F3980 + 0x4F3AD0) que 0x50C41A antepone a la lista
/// `tarea+8` por cada barco hundido: `+4` clase, `+5` marca (dueño), `+6` texto (`ship+0x15E`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HuntLoss {
    pub mark: u8,
    pub class: u8,
    pub text: u16,
}

/// Retorno de [`return_hunt_expedition`]: efectos comunes y nodos de bajas en orden de alta
/// (cada uno se antepone, así que el último es la cabeza de `tarea+8`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HuntEffects {
    pub effects: Effects,
    pub losses: Vec<HuntLoss>,
    /// Comerciantes avisados por `remove_ship` (poda de grupos 0x4F5900), en orden.
    pub pruned: Vec<u8>,
    /// Barcos cuya ruta (`+0x24`) quitó `clear_route` 0x516840; el llamante suelta su descriptor
    /// (el original lo libera si `+0x36 != 0`).
    pub removed_routes: Vec<u16>,
    /// Ranuras del pool de paradas 0x6DD728 que `remove_ship` libera con 0x4D4DD0(ranura, 0), en
    /// orden; no tocan barcos ni convoyes, así que el llamante las aplica después.
    pub released_stops: Vec<u16>,
}

/// `clear_route` 0x516840 sin interfaz: quita la ruta y su propiedad, `+0x34` y `+2/+3`.
fn clear_hunt_route(ship: &mut [u8], index: u16, removed: &mut Vec<u16>) {
    if ship[0x24..0x28] != [0; 4] {
        ship[0x36] = 0;
        ship[0x24..0x28].fill(0);
        removed.push(index);
    }
    ship[0x34..0x36].fill(0);
    ship[2..4].fill(0);
}

/// Rama 0x50C06F..0x50C0DA + 0x50C3CB..0x50C8EA de 0x0050BFD0 para un convoy comercial con
/// expedición hanseática de caza activa (0x503B00 verdadera, `task` es la tarea 0x34 de
/// `convoy+2`): el convoy no se disuelve. Convoy y supervivientes pasan a 0x16; si el acreditado
/// no es comerciante, `task+0x15 = 0xA` y `task+0xE` toma [`HUNT_MARK`] cuando valía 0; sin líder
/// aceptable (+0x14 bit 0x10) `task+0x15 = 0xA` y `task+0xE` = [`HUNT_UNLED_MARK`] siempre.
///
/// Cada barco hundido sale del convoy en estado 0x11 (ruta limpia, `+8/+0x50/+0x52` a 0xFFFF) y
/// produce un [`HuntLoss`]; si su dueño no es humano además se da de baja con `remove_ship`
/// 0x507FB0 (orden X, capitán, flota del comerciante y poda de grupos 0x4F5900). Si el líder no
/// sobrevive, `convoy+0x10 = 0xFFFF` y `ensure_leader` 0x5017F0 elige otro (o deja el convoy vacío
/// sin líder). Guardar los nodos en `task+8` es del llamante (C10); soltar los descriptores de
/// `removed_routes` y liberar `released_stops` (`task_limit` es el recuento 0x6DD72A del pool)
/// también. Atómica. Rechaza otros efectos de `remove_ship` y efectos de `ensure_leader` que no
/// sean de listas.
#[allow(clippy::too_many_arguments)]
pub fn return_hunt_expedition(
    manager: &mut [u8],
    ships: &mut [u8],
    convoys: &mut [u8],
    merchants: &mut [u8],
    id: u16,
    credited: u8,
    player: u32,
    task_limit: u16,
    task: &mut crate::scheduler::TaskRecord,
    captains: &mut [u8],
    towns: &mut [u8],
    groups: &mut [[Vec<u16>; 11]],
) -> Result<HuntEffects, &'static str> {
    if manager.len() < 0xf8
        || ships.len() % 0x180 != 0
        || convoys.len() % 0x3c != 0
        || merchants.len() % 0x650 != 0
        || towns.len() % 0x9f8 != 0
        || towns.len() / 0x9f8 > 40
    {
        return Err("flota truncada");
    }
    let r16 = |b: &[u8], o: usize| u16::from_le_bytes([b[o], b[o + 1]]);
    let n = usize::from(r16(manager, 0xf4));
    let nm = merchants.len() / 0x650;
    let merchant_count = nm as u16;
    let town_count = (towns.len() / 0x9f8) as u32;
    let at = usize::from(id) * 0x3c;
    if n > ships.len() / 0x180 || at + 0x3c > convoys.len() || groups.len() != nm {
        return Err("flota truncada");
    }
    if captains.len() != usize::from(r16(manager, 0xf2)) * 16 {
        return Err("pool de capitanes truncado");
    }
    let owner = convoys[at];
    if usize::from(owner) >= nm {
        return Err("expedición hanseática sin comerciante");
    }
    if task.kind != 0x34 {
        return Err("tarea de expedición inválida");
    }
    let mut m = manager.to_vec();
    let mut s = ships.to_vec();
    let mut c = convoys.to_vec();
    let mut ms = merchants.to_vec();
    let mut caps = captains.to_vec();
    let mut tw = towns.to_vec();
    let mut gs = groups.to_vec();
    // 0x50C04E: sin líder aceptable el original marca la disolución, pero la tarea la convierte en
    // retorno (0x50C098).
    let unled = c[at + 0x14] & 0x10 != 0;
    c[at + 0x14] &= !0x20;
    let mut t = *task;
    if unled {
        t.set_u8_at(0xe, HUNT_UNLED_MARK);
        t.set_u8_at(0x15, 0xa);
    } else if u16::from(credited) >= merchant_count {
        if t.u8_at(0xe) == 0 {
            t.set_u8_at(0xe, HUNT_MARK);
        }
        t.set_u8_at(0x15, 0xa);
    }
    c[at + 0x12..at + 0x14].copy_from_slice(&0x16u16.to_le_bytes());
    c[at + 0x1c] = 0xff;
    c[at + 0x1d] = 0;
    let speed = [c[at + 0xc], c[at + 0xd], c[at + 0xe], c[at + 0xf]];
    let main = r16(&c, at + 0x10);
    let mut losses = Vec::new();
    let mut pruned = Vec::new();
    let mut removed_routes = Vec::new();
    let mut released_stops = Vec::new();
    // [esp+0x13]: el líder no se ha visto vivo.
    let mut lost = true;
    let mut seen = vec![false; n];
    // Enlace actual: None = convoy+0xA, Some(i) = barco i +6.
    let mut link: Option<usize> = None;
    loop {
        let cur = match link {
            None => r16(&c, at + 0xa),
            Some(p) => r16(&s, p * 0x180 + 6),
        };
        let i = usize::from(cur);
        if i >= n {
            break;
        }
        if seen[i] {
            return Err("convoy cíclico");
        }
        seen[i] = true;
        let pos = i * 0x180;
        s[pos + 0x39] = 0xff;
        s[pos + 0x3c..pos + 0x3e].fill(0xff);
        if i32::from_le_bytes(s[pos + 0x18..pos + 0x1c].try_into().unwrap()) >= 1 {
            // 0x50C4DC: superviviente.
            if cur == main {
                lost = false;
            } else {
                clear_hunt_route(&mut s[pos..pos + 0x180], cur, &mut removed_routes);
            }
            s[pos + 0x134..pos + 0x136].copy_from_slice(&0x16u16.to_le_bytes());
            recompute_load(&mut s[pos..pos + 0x180]);
            s[pos + 0x30..pos + 0x34].copy_from_slice(&speed);
            link = Some(i);
            continue;
        }
        // 0x50C407: hundido.
        clear_hunt_route(&mut s[pos..pos + 0x180], cur, &mut removed_routes);
        losses.push(HuntLoss { mark: s[pos], class: HUNT_LOSS_CLASS, text: r16(&s, pos + 0x15e) });
        let next = [s[pos + 6], s[pos + 7]];
        match link {
            None => c[at + 0xa..at + 0xc].copy_from_slice(&next),
            Some(p) => s[p * 0x180 + 6..p * 0x180 + 8].copy_from_slice(&next),
        }
        for o in [8, 0x50, 0x52] {
            s[pos + o..pos + o + 2].fill(0xff);
        }
        s[pos + 0x134..pos + 0x136].copy_from_slice(&0x11u16.to_le_bytes());
        let ship_owner = usize::from(s[pos]);
        let human = ship_owner < nm && r16(&ms, ship_owner * 0x650 + 8) == 0;
        if human {
            // 0x50C4CC: el humano queda fuera de las listas, sin remove_ship.
            continue;
        }
        let mut rw = super::remove::RemoveWorld {
            manager: &mut m,
            ships: &mut s,
            captains: &mut caps,
            convoys: &mut c,
            merchants: &mut ms,
            towns: &mut tw,
            pool_limit: task_limit,
            merchant_count,
            town_count,
            ui_a: false,
            ui_b: false,
        };
        for event in super::remove::remove_ship(&mut rw, u32::from(cur)) {
            match event {
                super::remove::RemoveEvent::ClearRoute(_) => {
                    let ship = &mut rw.ships[pos..pos + 0x180];
                    ship[0x24..0x28].fill(0);
                    ship[0x34..0x36].fill(0);
                    ship[2..4].fill(0)
                }
                super::remove::RemoveEvent::OrderRemove(_) => {
                    let head = r16(rw.manager, 0xea);
                    XOrder { ships: rw.ships, count: n as u16, sea_head: head }.remove(cur)
                }
                super::remove::RemoveEvent::MerchantChanged(o) => {
                    let o = o as usize;
                    let mo = o * 0x650;
                    let mut merchant = crate::ai::reputation::Merchant { bytes: rw.merchants[mo..mo + 0x650].to_vec() };
                    let ships_now: &[u8] = rw.ships;
                    merchant.prune_foreign_ships_from_groups(o as u8, gs.get_mut(o).ok_or("grupo de comerciante ausente")?, &|id| {
                        ships_now[usize::from(id) * 0x180]
                    });
                    rw.merchants[mo..mo + 0x650].copy_from_slice(&merchant.bytes);
                    pruned.push(o as u8);
                }
                super::remove::RemoveEvent::PoolRelease(slot) => released_stops.push(slot),
                _ => return Err("efecto de baja de expedición pendiente"),
            }
        }
    }
    // 0x50C51F: líder perdido y ensure_leader 0x5017F0 del comerciante.
    if lost {
        c[at + 0x10..at + 0x12].fill(0xff);
    }
    {
        let mut hooks = super::convoy::LeaderHooks::new(None, false);
        let mut rw = super::remove::RemoveWorld {
            manager: &mut m,
            ships: &mut s,
            captains: &mut caps,
            convoys: &mut c,
            merchants: &mut ms,
            towns: &mut tw,
            pool_limit: task_limit,
            merchant_count,
            town_count,
            ui_a: false,
            ui_b: false,
        };
        let mut tick = super::tick::TickWorld { remove: &mut rw, time: 0, player: u32::MAX };
        super::convoy::pick_main(&mut tick, id, &mut hooks);
        if hooks.unsupported {
            return Err("efecto de líder de expedición pendiente");
        }
    }
    // 0x50C538: lista de convoyes y orden X del líder.
    let head = r16(&m, 0xec);
    c[at + 8..at + 10].copy_from_slice(&head.to_le_bytes());
    c[at + 0x18..at + 0x1c].fill(0xff);
    m[0xec..0xee].copy_from_slice(&id.to_le_bytes());
    let main = r16(&c, at + 0x10);
    let refresh_ship = if usize::from(main) < n {
        let sea = r16(&m, 0xea);
        XOrder { ships: &mut s, count: n as u16, sea_head: sea }.insert(main, u32::from(sea));
        (u32::from(owner) == player).then_some(main)
    } else {
        None
    };
    manager.copy_from_slice(&m);
    ships.copy_from_slice(&s);
    convoys.copy_from_slice(&c);
    merchants.copy_from_slice(&ms);
    captains.copy_from_slice(&caps);
    towns.copy_from_slice(&tw);
    groups.clone_from_slice(&gs);
    *task = t;
    Ok(HuntEffects {
        effects: Effects { recount_incoming: u16::from(credited) < merchant_count, refresh_ship },
        losses,
        pruned,
        removed_routes,
        released_stops,
    })
}
