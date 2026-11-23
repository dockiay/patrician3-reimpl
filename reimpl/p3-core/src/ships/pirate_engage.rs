//! Preparación de `ships::engage_target` (0x0050BC40) antes de SeaBattle::start.
//! El anfitrión implementa la baja de orden, copia de ruta, reserva e inicio.
use super::{
    r16, r32,
    remove::{CONVOY_SIZE, SHIP_SIZE},
    tick::TickWorld,
    w16, w32,
};

pub trait EngageHost {
    fn remove_order(&mut self, w: &mut TickWorld, ship: u16) -> Result<(), String> {
        super::order::remove_from_tick(w, ship);
        Ok(())
    }
    fn clear_targets(&mut self, w: &mut TickWorld, ship: u16) -> Result<(), String> {
        let n = r16(w.remove.manager, 0xf4);
        clear_player_targets(
            w.remove.ships,
            w.remove.merchants,
            n,
            w.remove.merchant_count,
            w.player,
            u32::from(ship),
        )
        .map_err(str::to_owned)
    }
    fn copy_route(&mut self, w: &mut TickWorld, ship: u16, source: Option<u16>) -> Result<(), String>;
    fn unlink_sea(&mut self, w: &mut TickWorld, ship: u16) -> Result<(), String>;
    fn refresh_ui(&mut self, ship: u16);
    fn create_convoy(&mut self, w: &mut TickWorld, ship: u16) -> Result<u16, String>;
    fn allocate_battle(&mut self) -> Result<u8, String>;
    fn start_battle(&mut self, w: &mut TickWorld, slot: u8, attacker: u16, target: u16) -> Result<(), String>;
}

/// Los cambios de barcos y convoyes se publican juntos solo al completar todos
/// los callbacks; el anfitrión debe hacer transaccionales sus propios efectos.
pub fn engage_target(w: &mut TickWorld, cv: u16, host: &mut dyn EngageHost) -> Result<(), String> {
    let mut manager = w.remove.manager.to_vec();
    let mut ships = w.remove.ships.to_vec();
    let mut captains = w.remove.captains.to_vec();
    let mut convoys = w.remove.convoys.to_vec();
    let mut merchants = w.remove.merchants.to_vec();
    let mut towns = w.remove.towns.to_vec();
    {
        let mut remove = super::remove::RemoveWorld {
            manager: &mut manager,
            ships: &mut ships,
            captains: &mut captains,
            convoys: &mut convoys,
            merchants: &mut merchants,
            towns: &mut towns,
            pool_limit: w.remove.pool_limit,
            merchant_count: w.remove.merchant_count,
            town_count: w.remove.town_count,
            ui_a: w.remove.ui_a,
            ui_b: w.remove.ui_b,
        };
        let mut staged = TickWorld {
            remove: &mut remove,
            time: w.time,
            player: w.player,
        };
        engage_inner(&mut staged, cv, host)?;
    }
    w.remove.manager.copy_from_slice(&manager);
    w.remove.ships.copy_from_slice(&ships);
    w.remove.captains.copy_from_slice(&captains);
    w.remove.convoys.copy_from_slice(&convoys);
    w.remove.merchants.copy_from_slice(&merchants);
    w.remove.towns.copy_from_slice(&towns);
    Ok(())
}
fn engage_inner(w: &mut TickWorld, cv: u16, host: &mut dyn EngageHost) -> Result<(), String> {
    if w.remove.manager.len() < 0xf8 {
        return Err("gestor truncado".into());
    }
    let n = r16(w.remove.manager, 0xf4);
    let nc = r16(w.remove.manager, 0xf6);
    if cv >= nc
        || w.remove.ships.len() < usize::from(n) * SHIP_SIZE
        || w.remove.convoys.len() < usize::from(nc) * CONVOY_SIZE
    {
        return Err("flota truncada".into());
    }
    let b = usize::from(cv) * CONVOY_SIZE;
    let main = r16(w.remove.convoys, b + 0x10);
    if main >= n {
        return Err("líder pirata inválido".into());
    }
    host.remove_order(w, main)?;
    host.clear_targets(w, main)?;
    w16(w.remove.convoys, b + 0x12, 0x14);
    let p = usize::from(main) * SHIP_SIZE;
    let heading = w.remove.ships[p + 1];
    let mut id = r16(w.remove.convoys, b + 0xa);
    let mut seen = vec![false; usize::from(n)];
    while id < n {
        let i = usize::from(id);
        if seen[i] {
            return Err("convoy pirata cíclico".into());
        }
        seen[i] = true;
        let at = i * SHIP_SIZE;
        w.remove.ships[at + 1] = heading;
        id = r16(w.remove.ships, at + 6)
    }
    let target = r16(w.remove.ships, p + 0x50);
    if target >= n {
        return Err("objetivo pirata inválido".into());
    }
    w16(w.remove.ships, p + 0x50, u16::MAX);
    w16(w.remove.ships, p + 0x52, u16::MAX);
    let t = usize::from(target) * SHIP_SIZE;
    let target_cv = r16(w.remove.ships, t + 8);
    let other = if target_cv < nc {
        if target_cv == cv {
            return Err("mismo convoy en ambos bandos".into());
        }
        let c = usize::from(target_cv) * CONVOY_SIZE;
        let lead = r16(w.remove.convoys, c + 0x10);
        let source = (lead < n).then_some(lead);
        let target_heading = w.remove.ships[t + 1];
        let mut id = r16(w.remove.convoys, c + 0xa);
        let mut seen = vec![false; usize::from(n)];
        while id < n {
            let i = usize::from(id);
            if seen[i] {
                return Err("convoy objetivo cíclico".into());
            }
            seen[i] = true;
            let at = i * SHIP_SIZE;
            let next = r16(w.remove.ships, at + 6);
            w.remove.ships[at + 1] = target_heading;
            let state = if u16::from(w.remove.ships[at]) < w.remove.merchant_count {
                0x14
            } else {
                0x12
            };
            w16(w.remove.ships, at + 0x134, state);
            w16(w.remove.ships, at + 0x3c, u16::MAX);
            host.copy_route(w, id, source)?;
            id = next
        }
        let mut cursor = r16(w.remove.manager, 0xec);
        let mut previous = None;
        let mut visited = vec![false; usize::from(nc)];
        while cursor < nc && cursor != target_cv {
            let j = usize::from(cursor);
            if visited[j] {
                return Err("lista de convoyes cíclica".into());
            }
            visited[j] = true;
            previous = Some(j * CONVOY_SIZE + 8);
            cursor = r16(w.remove.convoys, j * CONVOY_SIZE + 8)
        }
        if cursor == target_cv {
            let next = r16(w.remove.convoys, c + 8);
            if let Some(at) = previous {
                w16(w.remove.convoys, at, next)
            } else {
                w16(w.remove.manager, 0xec, next)
            }
            w16(w.remove.convoys, c + 8, u16::MAX);
            if lead < n {
                host.remove_order(w, lead)?;
                host.refresh_ui(lead)
            }
        }
        if r16(w.remove.convoys, c + 0x12) == 0x12 {
            host.clear_targets(w, lead)?
        } else {
            w16(w.remove.convoys, c + 0x12, 0x14)
        }
        target_cv
    } else {
        host.unlink_sea(w, target)?;
        if u32::from(w.remove.convoys[b]) == w.player {
            host.refresh_ui(main)
        }
        let state = if u16::from(w.remove.ships[t]) < w.remove.merchant_count {
            0x14
        } else {
            0x12
        };
        w16(w.remove.ships, t + 0x134, state);
        if state == 0x12 {
            host.clear_targets(w, target)?
        }
        w16(w.remove.ships, t + 0x3c, u16::MAX);
        let other = host.create_convoy(w, target)?;
        if other >= nc {
            return Err("reserva de convoy inválida".into());
        }
        let c = usize::from(other) * CONVOY_SIZE;
        w.remove.convoys[c] = w.remove.ships[t];
        w32(w.remove.convoys, c + 0x18, r32(w.remove.convoys, c + 0x18) | 0x400);
        w.remove.convoys[c + 1] = w.remove.ships[t + 0xe];
        w32(w.remove.convoys, c + 4, 0);
        w16(w.remove.convoys, c + 0x12, 0x14);
        w16(w.remove.convoys, c + 0x14, 0x12);
        w16(w.remove.convoys, c + 0x16, 0);
        w32(w.remove.convoys, c + 0x18, -1);
        w16(w.remove.convoys, c + 0x1c, u16::from(w.remove.ships[t + 0x37]));
        w.remove.convoys[c + 0x1e] = w.remove.ships[t + 0x39];
        w.remove.convoys[c + 0x1f] = w.remove.ships[t + 0x38];
        other
    };
    let slot = host.allocate_battle()?;
    if slot == u8::MAX {
        return Err("sin ranura de batalla".into());
    }
    let other_lead = r16(w.remove.convoys, usize::from(other) * CONVOY_SIZE + 0x10);
    if other_lead >= n {
        return Err("líder objetivo inválido".into());
    }
    host.remove_order(w, main)?;
    host.remove_order(w, other_lead)?;
    host.start_battle(w, slot, main, other_lead)?;
    Ok(())
}

/// Selección de convoyes para `SeaBattle::start` cuando los dos bandos ya
/// tienen convoy, sin expansión especial de barcos humanos y con `skip=0`.
/// `0xFE` representa un bando especial y acepta propietarios mezclados.
/// Los barcos que pertenecen al participante contrario se marcan en +0x12A.
fn select_pirate_sides(
    ships: &[u8],
    convoys: &[u8],
    convoy_ids: [u16; 2],
    participants: [u16; 2],
) -> Result<[Vec<u16>; 2], &'static str> {
    if ships.len() % SHIP_SIZE != 0 || convoys.len() % CONVOY_SIZE != 0 {
        return Err("flota truncada");
    }
    let count = ships.len() / SHIP_SIZE;
    if count > usize::from(u16::MAX) {
        return Err("flota demasiado grande");
    }
    let mut selected = [Vec::new(), Vec::new()];
    for side in 0..2 {
        let cv = usize::from(convoy_ids[side]) * CONVOY_SIZE;
        if cv + CONVOY_SIZE > convoys.len() {
            return Err("convoy truncado");
        }
        let mut id = r16(convoys, cv + 0xa);
        let mut seen = vec![false; count];
        while usize::from(id) < count {
            let i = usize::from(id);
            if seen[i] {
                return Err("lista de barcos cíclica");
            }
            seen[i] = true;
            let at = i * SHIP_SIZE;
            if participants[side] == 0xfe || u16::from(ships[at]) == participants[side] {
                selected[side].push(id)
            }
            id = r16(ships, at + 6);
        }
    }
    Ok(selected)
}
pub fn build_pirate_battle_sides(
    ships: &[u8],
    convoys: &[u8],
    captains: &[u8],
    captain_count: u16,
    convoy_ids: [u16; 2],
    participants: [u16; 2],
    battle_address: u32,
    spatial_ids: &[(u16, u16)],
) -> Result<[super::battle::LocalBattleSide; 2], &'static str> {
    use super::battle::build_battle_side;
    let selected = select_pirate_sides(ships, convoys, convoy_ids, participants)?;
    let n = selected[0].len();
    if spatial_ids.len() != n + selected[1].len() {
        return Err("índices espaciales incompatibles");
    }
    let mut sides = [
        build_battle_side(
            ships,
            captains,
            captain_count,
            &selected[0],
            battle_address,
            &spatial_ids[..n],
            0,
            0,
        )?,
        build_battle_side(
            ships,
            captains,
            captain_count,
            &selected[1],
            battle_address,
            &spatial_ids[n..],
            1,
            n as u16,
        )?,
    ];
    for side in 0..2 {
        if participants[side] == 0xfe {
            for local in &mut sides[side].ships {
                let id = r32(local, 4) as u32 as usize;
                if u16::from(ships[id * SHIP_SIZE]) == participants[1 - side] {
                    local[0x12a] |= 1
                }
            }
        }
    }
    Ok(sides)
}

/// Variante que reserva de forma transaccional las dos entradas espaciales
/// de cada barco, ampliando el pool en bloques como en el inicio original.
pub fn build_pirate_sides_growing(
    ships: &[u8],
    convoys: &[u8],
    captains: &[u8],
    captain_count: u16,
    convoy_ids: [u16; 2],
    participants: [u16; 2],
    battle_address: u32,
    pool: &mut super::battle::BattleCollisionPool,
) -> Result<[super::battle::LocalBattleSide; 2], &'static str> {
    let selected = select_pirate_sides(ships, convoys, convoy_ids, participants)?;
    let total = selected[0].len() + selected[1].len();
    if total > usize::from(u16::MAX) {
        return Err("demasiados barcos locales");
    }
    let mut staged = pool.clone();
    let mut spatial = Vec::with_capacity(total);
    for i in 0..total {
        spatial.push((
            staged.add_growing(0, i as u8, 0, 0, 0, 0)?,
            staged.add_growing(6, i as u8, 0, 0, 0, 0)?,
        ))
    }
    let sides = build_pirate_battle_sides(
        ships,
        convoys,
        captains,
        captain_count,
        convoy_ids,
        participants,
        battle_address,
        &spatial,
    )?;
    *pool = staged;
    Ok(sides)
}

/// Completa la parte del inicio que registra ambas listas de un encuentro
/// entre convoyes. El prefijo y la clasificación deben haberse ejecutado;
/// no implementa avisos, apertura de pantalla, viento ni resolución.
/// La probabilidad +0x1E94 se ajusta por los miembros descartados por dueño.
pub fn initialize_pirate_roster(
    object: &mut [u8],
    ships: &[u8],
    convoys: &[u8],
    captains: &[u8],
    captain_count: u16,
    convoy_ids: [u16; 2],
    battle_address: u32,
    pool: &mut super::battle::BattleCollisionPool,
) -> Result<[super::battle::LocalBattleSide; 2], &'static str> {
    if object.len() < 0x1e96 {
        return Err("objeto de batalla truncado");
    }
    if object[0x12e8] != 0 {
        return Err("omisión de segundo bando no implementada");
    }
    let participants = [r16(object, 0x35a), r16(object, 0x35c)];
    let selected = select_pirate_sides(ships, convoys, convoy_ids, participants)?;
    if selected.iter().any(Vec::is_empty) {
        return Err("bando vacío no implementado");
    }
    let mut staged = object.to_vec();
    let mut next_pool = pool.clone();
    let sides = build_pirate_sides_growing(
        ships,
        convoys,
        captains,
        captain_count,
        convoy_ids,
        participants,
        battle_address,
        &mut next_pool,
    )?;
    for side in 0..2 {
        if participants[side] != 0xfe && staged[0xad2] == if side == 0 { 2 } else { 1 } {
            let cv = usize::from(convoy_ids[side]) * CONVOY_SIZE;
            let mut id = r16(convoys, cv + 0xa);
            while usize::from(id) < ships.len() / SHIP_SIZE {
                let at = usize::from(id) * SHIP_SIZE;
                if u16::from(ships[at]) != participants[side] {
                    let old = r16(&staged, 0x1e94);
                    let low = i32::from(old & 0x3ff);
                    let adjusted = 1000 - ((1000 - low) * 950 + 500) / 1000;
                    w16(&mut staged, 0x1e94, (old & 0xfc00).wrapping_add(adjusted as u16));
                }
                id = r16(ships, at + 6);
            }
        }
    }
    let n0 = sides[0].ships.len() as u16;
    let n1 = sides[1].ships.len() as u16;
    w16(&mut staged, 0x10, 0);
    w16(&mut staged, 0x12, u16::MAX);
    w16(&mut staged, 0x14, n0 + n1);
    w16(&mut staged, 0x658, n0);
    w16(&mut staged, 0x65a, n1);
    w16(&mut staged, 0x65c, n0);
    for side in 0..2 {
        let cv = usize::from(convoy_ids[side]) * CONVOY_SIZE;
        w32(&mut staged, 0xa84 + side * 4, i32::from(r16(convoys, cv + 0xa)));
    }
    staged[0xab0] = 1;
    staged[0xab1] = if participants == [0xfe, 0xff] { 0xff } else { 1 };
    staged[0x12e8] = 1;
    // 0x60611C restaura skip al saltar miembros del segundo convoy. El bucle
    // termina al construir su último seleccionado: no examina descartados finales.
    if participants[1] != 0xfe {
        let last = *selected[1].last().ok_or("segundo bando vacío")?;
        let mut id = r16(convoys, usize::from(convoy_ids[1]) * CONVOY_SIZE + 0xa);
        while id != last {
            let at = usize::from(id) * SHIP_SIZE;
            if u16::from(ships[at]) != participants[1] {
                staged[0x12e8] = object[0x12e8]
            }
            id = r16(ships, at + 6);
        }
    }
    object.copy_from_slice(&staged);
    *pool = next_pool;
    Ok(sides)
}

/// Inicia una batalla entre dos convoyes ya preparados y controlados por IA.
/// Devuelve la apertura de vista al anfitrión; no modifica listas mundiales,
/// no reserva la ranura y no publica por sí misma el resultado en S05.
#[allow(clippy::too_many_arguments)]
pub fn start_existing_ai_pirate_convoys(
    manager: &[u8],
    ships: &[u8],
    convoys: &[u8],
    merchants: &[u8],
    captains: &[u8],
    attacker: u16,
    target: u16,
    slot: u8,
    battle_address: u32,
    side0_pirate_band: bool,
) -> Result<super::battle::CommercialBattleStart, &'static str> {
    start_existing_ai_pirate_convoys_with_policy(
        manager, ships, convoys, merchants, captains, attacker, target, slot, battle_address, side0_pirate_band, false,
    )
}

/// Como `start_existing_ai_pirate_convoys`; con `human_as_ai` el bando humano sin reloj se resuelve como IA
/// (precedente hotseat de `classify_battle_sides`, ver C07_HEADLESS_HUMAN_BATTLES.md).
#[allow(clippy::too_many_arguments)]
pub fn start_existing_ai_pirate_convoys_with_policy(
    manager: &[u8],
    ships: &[u8],
    convoys: &[u8],
    merchants: &[u8],
    captains: &[u8],
    attacker: u16,
    target: u16,
    slot: u8,
    battle_address: u32,
    side0_pirate_band: bool,
    human_as_ai: bool,
) -> Result<super::battle::CommercialBattleStart, &'static str> {
    use super::battle::{self, BattleCollisionPool, BattleStartPrefix, CommercialBattleStart};
    use super::remove::MERCHANT_SIZE;
    if slot == u8::MAX || manager.len() < 0xf8 || merchants.len() % MERCHANT_SIZE != 0 {
        return Err("gestor, comerciantes o ranura inválidos");
    }
    let n = r16(manager, 0xf4);
    let nc = r16(manager, 0xf6);
    let cap = r16(manager, 0xf2);
    if attacker >= n
        || target >= n
        || ships.len() < usize::from(n) * SHIP_SIZE
        || convoys.len() < usize::from(nc) * CONVOY_SIZE
    {
        return Err("flota truncada");
    }
    let mc = merchants.len() / MERCHANT_SIZE;
    if mc > usize::from(u16::MAX) {
        return Err("demasiados comerciantes");
    }
    let mut object = vec![0; 0x1e98];
    let mut slots = vec![0; 0x405];
    battle::construct_basic_battle(&mut object, &mut slots, battle_address, u32::from(slot))
        .ok_or("constructor truncado")?;
    if battle::start_battle_prefix(&mut object, ships, attacker, target, 0)? != BattleStartPrefix::Continue {
        return Err("batalla rechazada");
    }
    let mut notifications = battle::classify_battle_sides(
        &mut object,
        convoys,
        merchants,
        mc as u16,
        side0_pirate_band,
        false,
        attacker,
        target,
    )?;
    if object[0xac8] == 0 {
        if !human_as_ai {
            return Err("batalla con jugador humano no implementada");
        }
        // Política sin pantalla: el bando humano se resuelve como IA, igual que hace el original en partidas de
        // turnos con dos humanos distintos (`classify_battle_sides`, `hotseat`). Se omiten diálogo y avisos.
        object[0x5d] = 1;
        object[0x5e] = 1;
        object[0xac8] = 1;
        notifications.clear();
    }
    let convoy_ids = [
        r16(ships, usize::from(attacker) * SHIP_SIZE + 8),
        r16(ships, usize::from(target) * SHIP_SIZE + 8),
    ];
    if convoy_ids.iter().any(|&cv| cv >= nc) {
        return Err("convoy ausente");
    }
    let mut pool = BattleCollisionPool::with_capacity(0)?;
    let [a, b] = initialize_pirate_roster(
        &mut object,
        &ships[..usize::from(n) * SHIP_SIZE],
        &convoys[..usize::from(nc) * CONVOY_SIZE],
        captains,
        cap,
        convoy_ids,
        battle_address,
        &mut pool,
    )?;
    let mut local_ships = a.ships;
    local_ships.extend(b.ships);
    let mut next = a.next;
    next.extend(b.next);
    let mut previous = a.previous;
    previous.extend(b.previous);
    let mut spatial_0 = a.spatial_0;
    spatial_0.extend(b.spatial_0);
    let mut spatial_1 = a.spatial_1;
    spatial_1.extend(b.spatial_1);
    object[0x5c] = 0;
    w16(&mut object, 0x15c, 0);
    w16(&mut object, 0x15e, 0);
    object[0x1e90] = 0xff;
    object[0x1e91] = 0xff;
    let special_side = if r16(&object, 0x35a) == 0xff {
        Some(0)
    } else if r16(&object, 0x35c) == 0xff {
        Some(1)
    } else {
        None
    };
    if let Some(side) = special_side {
        let leader = r16(convoys, usize::from(convoy_ids[side]) * CONVOY_SIZE + 0x10);
        if leader >= n {
            return Err("líder del bando sin propietario inválido");
        }
        let captain = r16(ships, usize::from(leader) * SHIP_SIZE + 0x42);
        if captain < cap {
            let at = usize::from(captain) * 16;
            if at + 4 > captains.len() {
                return Err("capitán truncado");
            }
            object[0x1e90] = captains[at + 2];
            object[0x1e91] = captains[at + 3];
        }
    }

    w16(&mut object, 0x20, pool.free_head);
    w16(&mut object, 0x34, pool.x_head);
    w16(&mut object, 0x44, pool.y_head);
    w16(&mut object, 0x54, pool.high_water);
    w16(&mut object, 0x56, pool.kinds.len() as u16);
    w16(&mut object, 0x58, (pool.kinds.len() * 2) as u16);
    w16(&mut object, 0x5a, pool.matrices[0].len() as u16);
    let first = battle::advance_battle_rng(r32(&object, 0xadc) as u32);
    let mut wind = (first & 3) as i32 + 5;
    let x = r16(&object, 0xad6) as i16;
    let y = r16(&object, 0xada) as i16;
    for i in 0..16usize {
        let at = 0x20 + i * 12;
        if r16(manager, at) < n {
            let dx = i32::from(r16(manager, at + 4) as i16) - i32::from(x);
            let dy = i32::from(r16(manager, at + 6) as i16) - i32::from(y);
            if dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy)) < 0x385 {
                match r16(manager, at + 8) {
                    0 => wind += 3,
                    1 => wind -= 3,
                    _ => {}
                }
            }
        }
    }
    let final_rng = battle::advance_battle_rng(first);
    w32(&mut object, 0xadc, final_rng as i32);
    object[0x670] = final_rng as u8;
    object[0x671] = wind.clamp(2, 10) as u8;
    Ok(CommercialBattleStart {
        object,
        local_ships,
        next,
        previous,
        spatial_0,
        spatial_1,
        collisions: pool,
        route_cache_invalidations: Vec::new(),
        notifications,
        operations: Vec::new(),
        open_for_player_requested: true,
        dialog: None,
    })
}

/// `0x0050CFB0`: limpia la primera de las dos referencias de cada barco del
/// jugador que apunta al objetivo. La lista comercial se recorre por +0x04.
/// Los ciclos y las imágenes truncadas se rechazan sin publicar cambios.
pub fn clear_player_targets(
    ships: &mut [u8],
    merchants: &[u8],
    ship_count: u16,
    merchant_count: u16,
    player: u32,
    target: u32,
) -> Result<(), &'static str> {
    use super::remove::MERCHANT_SIZE;
    if player >= u32::from(merchant_count) {
        return Ok(());
    }
    if ships.len() < usize::from(ship_count) * SHIP_SIZE
        || merchants.len() < usize::from(merchant_count) * MERCHANT_SIZE
    {
        return Err("flota o comerciantes truncados");
    }
    let mut staged = ships.to_vec();
    let mut id = r16(merchants, player as usize * MERCHANT_SIZE + 0xe);
    let mut seen = vec![false; usize::from(ship_count)];
    while id < ship_count {
        let i = usize::from(id);
        if seen[i] {
            return Err("lista de comerciante cíclica");
        }
        seen[i] = true;
        let at = i * SHIP_SIZE;
        if u32::from(r16(&staged, at + 0x50)) == target {
            w16(&mut staged, at + 0x50, u16::MAX)
        } else if u32::from(r16(&staged, at + 0x52)) == target {
            w16(&mut staged, at + 0x52, u16::MAX)
        }
        id = r16(&staged, at + 4);
    }
    ships.copy_from_slice(&staged);
    Ok(())
}

/// Recurso adicional requerido tras copiar una ruta propia. El anfitrión debe
/// crear el descriptor y copiar los puntos antes de publicar la transacción.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleRouteCopy {
    Unchanged,
    Shared,
    Owned {
        source_descriptor: u32,
        new_descriptor: u32,
    },
}
/// `0x005176C0` para las imágenes de barco. El descriptor nuevo lo reserva el
/// anfitrión; el contenido de la ruta propia queda expresado en el resultado.
pub fn copy_battle_route(
    destination: &mut [u8],
    source: &[u8],
    new_descriptor: Option<u32>,
) -> Result<BattleRouteCopy, &'static str> {
    if destination.len() < SHIP_SIZE || source.len() < SHIP_SIZE {
        return Err("barco truncado");
    }
    if r32(destination, 0x24) != 0 {
        return Ok(BattleRouteCopy::Unchanged);
    }
    let descriptor = r32(source, 0x24) as u32;
    if descriptor == 0 {
        return Ok(BattleRouteCopy::Unchanged);
    }
    let owned = source[0x36] != 0;
    let pointer = if owned {
        new_descriptor
            .filter(|&v| v != 0)
            .ok_or("descriptor de ruta propia ausente")?
    } else {
        descriptor
    };
    let mut staged = destination.to_vec();
    for &(off, len) in &[
        (0x1c, 4usize),
        (0x20, 4),
        (0x28, 4),
        (0x2c, 4),
        (0x30, 4),
        (0x34, 2),
        (2, 2),
    ] {
        staged[off..off + len].copy_from_slice(&source[off..off + len])
    }
    staged[0x36] = u8::from(owned);
    w32(&mut staged, 0x24, pointer as i32);
    destination.copy_from_slice(&staged);
    Ok(if owned {
        BattleRouteCopy::Owned {
            source_descriptor: descriptor,
            new_descriptor: pointer,
        }
    } else {
        BattleRouteCopy::Shared
    })
}

/// Copia de puntos que el integrador debe completar para una ruta propia.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PendingOwnedRouteCopy {
    pub ship: u16,
    pub source: u16,
    pub source_descriptor: u32,
    pub new_descriptor: u32,
}
/// Resultado provisional: el llamante debe reservar la batalla, aplicar los
/// efectos de rutas y UI, y publicar todos los cambios en una transacción S05.
#[derive(Debug, Clone)]
pub struct PreparedAiPirateEncounter {
    pub battle: super::battle::CommercialBattleStart,
    pub owned_routes: Vec<PendingOwnedRouteCopy>,
    pub shared_routes: Vec<(u16, u16)>,
    pub ui_ships: Vec<u16>,
}
struct AiEncounterHost<'a> {
    slot: u8,
    address: u32,
    band: bool,
    human_as_ai: bool,
    descriptors: &'a [u32],
    battle: Option<super::battle::CommercialBattleStart>,
    routes: Vec<PendingOwnedRouteCopy>,
    shared: Vec<(u16, u16)>,
    ui: Vec<u16>,
}
impl EngageHost for AiEncounterHost<'_> {
    fn copy_route(&mut self, w: &mut TickWorld, ship: u16, source: Option<u16>) -> Result<(), String> {
        let at = usize::from(ship) * SHIP_SIZE;
        if at + SHIP_SIZE > w.remove.ships.len() {
            return Err("barco truncado".into());
        }
        if r32(w.remove.ships, at + 0x24) != 0 {
            return Ok(());
        }
        let source = source.ok_or("origen de ruta ausente")?;
        let from = usize::from(source) * SHIP_SIZE;
        if from + SHIP_SIZE > w.remove.ships.len() {
            return Err("origen de ruta truncado".into());
        }
        let original = w.remove.ships[from..from + SHIP_SIZE].to_vec();
        let token = self.descriptors.get(usize::from(ship)).copied();
        if original[0x36] != 0 && r32(&original, 0x24) != 0 {
            let token = token.filter(|&v| v != 0).ok_or("descriptor de ruta propia ausente")?;
            let count = usize::from(r16(w.remove.manager, 0xf4));
            if (0..count).any(|i| r32(w.remove.ships, i * SHIP_SIZE + 0x24) as u32 == token)
                || self.routes.iter().any(|r| r.new_descriptor == token)
            {
                return Err("descriptor de ruta ya utilizado".into());
            }
        }
        match copy_battle_route(&mut w.remove.ships[at..at + SHIP_SIZE], &original, token).map_err(str::to_owned)? {
            BattleRouteCopy::Owned {
                source_descriptor,
                new_descriptor,
            } => self.routes.push(PendingOwnedRouteCopy {
                ship,
                source,
                source_descriptor,
                new_descriptor,
            }),
            BattleRouteCopy::Shared => self.shared.push((ship, source)),
            BattleRouteCopy::Unchanged => {}
        }
        Ok(())
    }
    fn unlink_sea(&mut self, w: &mut TickWorld, ship: u16) -> Result<(), String> {
        let count = r16(w.remove.manager, 0xf4);
        if ship >= count {
            return Err("barco marítimo inválido".into());
        }
        super::order::remove_from_tick(w, ship);
        let mut current = r16(w.remove.manager, 0xea);
        let mut previous = None;
        let mut seen = vec![false; usize::from(count)];
        while current < count && current != ship {
            let i = usize::from(current);
            if seen[i] {
                return Err("lista marítima cíclica".into());
            }
            seen[i] = true;
            previous = Some(i * SHIP_SIZE + 6);
            current = r16(w.remove.ships, i * SHIP_SIZE + 6)
        }
        if current != ship {
            return Err("barco ausente de la lista marítima".into());
        }
        let at = usize::from(ship) * SHIP_SIZE;
        let next = r16(w.remove.ships, at + 6);
        if let Some(link) = previous {
            w16(w.remove.ships, link, next)
        } else {
            w16(w.remove.manager, 0xea, next)
        }
        w16(w.remove.ships, at + 6, u16::MAX);
        let mut id = r16(w.remove.manager, 0xea);
        seen.fill(false);
        while id < count {
            let i = usize::from(id);
            if seen[i] {
                return Err("lista marítima cíclica".into());
            }
            seen[i] = true;
            let p = i * SHIP_SIZE;
            for off in [0x50, 0x52] {
                if r16(w.remove.ships, p + off) == ship {
                    w16(w.remove.ships, p + off, u16::MAX)
                }
            }
            id = r16(w.remove.ships, p + 6)
        }
        self.ui.push(ship);
        Ok(())
    }
    fn refresh_ui(&mut self, ship: u16) {
        self.ui.push(ship)
    }
    fn create_convoy(&mut self, w: &mut TickWorld, ship: u16) -> Result<u16, String> {
        let count = r16(w.remove.manager, 0xf4);
        let capacity = r16(w.remove.manager, 0xf6);
        let cv = r16(w.remove.manager, 0xe4);
        if ship >= count {
            return Err("barco inválido".into());
        }
        if cv >= capacity {
            return Err("ampliación del pool de convoyes no implementada".into());
        }
        let p = usize::from(ship) * SHIP_SIZE;
        let b = usize::from(cv) * CONVOY_SIZE;
        if p + SHIP_SIZE > w.remove.ships.len() || b + CONVOY_SIZE > w.remove.convoys.len() {
            return Err("flota truncada".into());
        }
        let next = r16(w.remove.convoys, b + 8);
        w16(w.remove.manager, 0xe4, next);
        w16(w.remove.convoys, b + 8, u16::MAX);
        w16(w.remove.convoys, b + 0xa, ship);
        w.remove.convoys[b] = w.remove.ships[p];
        w.remove.convoys[b + 1] = w.remove.ships[p + 0xe];
        w16(w.remove.convoys, b + 0x1c, u16::from(w.remove.ships[p + 0x39]));
        w.remove.convoys[b + 0x1e] = w.remove.ships[p + 0x37];
        w.remove.convoys[b + 0x1f] = w.remove.ships[p + 0x38];
        w32(w.remove.convoys, b + 4, r32(w.remove.ships, p + 0x10));
        w32(w.remove.convoys, b + 0x18, r32(w.remove.convoys, b + 0x18) | 0x400);
        w16(w.remove.ships, p + 8, cv);
        super::ai_depart::init_convoy(w, cv, ship, 2).map_err(|e| format!("inicialización de convoy: {e:?}"))?;
        w16(w.remove.convoys, b + 0x16, 0x100);
        w16(w.remove.convoys, b + 0x12, r16(w.remove.ships, p + 0x134));
        Ok(cv)
    }
    fn allocate_battle(&mut self) -> Result<u8, String> {
        if self.slot == u8::MAX {
            Err("sin ranura de batalla".into())
        } else {
            Ok(self.slot)
        }
    }
    fn start_battle(&mut self, w: &mut TickWorld, slot: u8, attacker: u16, target: u16) -> Result<(), String> {
        if self.battle.is_some() {
            return Err("batalla duplicada".into());
        }
        self.battle = Some(
            start_existing_ai_pirate_convoys_with_policy(
                w.remove.manager,
                w.remove.ships,
                w.remove.convoys,
                w.remove.merchants,
                w.remove.captains,
                attacker,
                target,
                slot,
                self.address,
                self.band,
                self.human_as_ai,
            )
            .map_err(str::to_owned)?,
        );
        Ok(())
    }
}
/// Resultado aislado de sacar un barco suelto del mar y crearle convoy.
/// El llamante publica imágenes y efectos de presentación como una sola transacción.
#[derive(Debug, Clone)]
pub struct PreparedSingleShipConvoy {
    pub manager: Vec<u8>,
    pub ships: Vec<u8>,
    pub captains: Vec<u8>,
    pub convoys: Vec<u8>,
    pub merchants: Vec<u8>,
    pub towns: Vec<u8>,
    pub convoy: u16,
    pub ui_ships: Vec<u16>,
}

/// 0x00505B60 seguido de 0x005062F0; `output_initial` es el valor
/// previo de la variable de salida, que el original copia a convoy+8.
/// No reserva la escena de asalto ni abre ventanas; falla sin cambios si hace
/// falta ampliar el pool de convoyes o si las listas de entrada son inválidas.
pub fn prepare_single_ship_convoy(
    w: &TickWorld,
    ship: u16,
    output_initial: u16,
) -> Result<PreparedSingleShipConvoy, String> {
    prepare_single_ship_convoy_inner(w, ship, output_initial, false)
}

/// Rama de 0x004EAFE4 para barco mercante suelto. La escena y la
/// persistencia las completa la transacción llamante.
pub fn prepare_pirate_mission_single_ship_convoy(
    w: &TickWorld,
    ship: u16,
    output_initial: u16,
) -> Result<PreparedSingleShipConvoy, String> {
    prepare_single_ship_convoy_inner(w, ship, output_initial, true)
}

fn prepare_single_ship_convoy_inner(
    w: &TickWorld,
    ship: u16,
    output_initial: u16,
    mission: bool,
) -> Result<PreparedSingleShipConvoy, String> {
    if w.remove.manager.len() < 0xf8 {
        return Err("gestor truncado".into());
    }
    let n = r16(w.remove.manager, 0xf4);
    let nc = r16(w.remove.manager, 0xf6);
    if w.remove.ships.len() < usize::from(n) * SHIP_SIZE || w.remove.convoys.len() < usize::from(nc) * CONVOY_SIZE {
        return Err("flota truncada".into());
    }
    let mut manager = w.remove.manager.to_vec();
    let mut ships = w.remove.ships.to_vec();
    let mut captains = w.remove.captains.to_vec();
    let mut convoys = w.remove.convoys.to_vec();
    let mut merchants = w.remove.merchants.to_vec();
    let mut towns = w.remove.towns.to_vec();
    let mut host = AiEncounterHost {
        slot: 0,
        address: 0,
        band: false,
        human_as_ai: false,
        descriptors: &[],
        battle: None,
        routes: Vec::new(),
        shared: Vec::new(),
        ui: Vec::new(),
    };
    let convoy = {
        let mut remove = super::remove::RemoveWorld {
            manager: &mut manager,
            ships: &mut ships,
            captains: &mut captains,
            convoys: &mut convoys,
            merchants: &mut merchants,
            towns: &mut towns,
            pool_limit: w.remove.pool_limit,
            merchant_count: w.remove.merchant_count,
            town_count: w.remove.town_count,
            ui_a: w.remove.ui_a,
            ui_b: w.remove.ui_b,
        };
        let mut staged = TickWorld {
            remove: &mut remove,
            time: w.time,
            player: w.player,
        };
        host.unlink_sea(&mut staged, ship)?;
        if mission {
            let p = usize::from(ship) * SHIP_SIZE;
            let bits = r16(staged.remove.ships, p + 0x3c) | 0x20;
            w16(staged.remove.ships, p + 0x3c, bits);
            w16(staged.remove.ships, p + 0x134, 0x14);
            super::order::remove_from_tick(&mut staged, ship);
        }
        host.create_convoy(&mut staged, ship)?
    };
    let at = usize::from(convoy) * CONVOY_SIZE;
    w16(&mut convoys, at + 8, output_initial);
    if mission {
        w16(&mut convoys, at + 0x14, 0x12);
    }
    Ok(PreparedSingleShipConvoy {
        manager,
        ships,
        captains,
        convoys,
        merchants,
        towns,
        convoy,
        ui_ships: host.ui,
    })
}

/// Encadena la preparación original con la construcción de batalla entre dos
/// convoyes de IA o crea un convoy para el objetivo suelto. Rechaza
/// participante humano, crecimiento de pool y errores de ruta sin publicar cambios. La reserva real y efectos son del llamante.
pub fn prepare_ai_pirate_encounter(
    w: &mut TickWorld,
    convoy: u16,
    slot: u8,
    battle_address: u32,
    side0_pirate_band: bool,
    owned_descriptors: &[u32],
) -> Result<PreparedAiPirateEncounter, String> {
    prepare_ai_pirate_encounter_with_policy(w, convoy, slot, battle_address, side0_pirate_band, owned_descriptors, false)
}

/// Como [`prepare_ai_pirate_encounter`]; con `human_as_ai` un convoy de jugador humano entra en la batalla automática
/// como si lo controlara la IA (política sin pantalla, no paridad: sustituye el diálogo y el combate interactivo).
pub fn prepare_ai_pirate_encounter_with_policy(
    w: &mut TickWorld,
    convoy: u16,
    slot: u8,
    battle_address: u32,
    side0_pirate_band: bool,
    owned_descriptors: &[u32],
    human_as_ai: bool,
) -> Result<PreparedAiPirateEncounter, String> {
    let mut host = AiEncounterHost {
        slot,
        address: battle_address,
        band: side0_pirate_band,
        human_as_ai,
        descriptors: owned_descriptors,
        battle: None,
        routes: Vec::new(),
        shared: Vec::new(),
        ui: Vec::new(),
    };
    engage_target(w, convoy, &mut host)?;
    let battle = host.battle.ok_or("no se inició la batalla")?;
    Ok(PreparedAiPirateEncounter {
        battle,
        owned_routes: host.routes,
        shared_routes: host.shared,
        ui_ships: host.ui,
    })
}

/// `0x00503AC0`: el convoy pertenece a la expedición de la Liga si su
/// referencia de tarea apunta a una 0x34 que lo identifica en +0x16.
pub fn is_hanse_expedition(
    convoy: &[u8],
    convoy_id: u16,
    scheduler: &crate::scheduler::Scheduler,
) -> Result<bool, &'static str> {
    if convoy.len() < CONVOY_SIZE {
        return Err("convoy truncado");
    }
    let index = r16(convoy, 2);
    if index >= scheduler.capacity {
        return Ok(false);
    }
    let task = scheduler
        .records
        .get(usize::from(index))
        .ok_or("planificador truncado")?;
    let linked = u16::from_le_bytes([task.u8_at(0x16), task.u8_at(0x17)]);
    Ok(task.kind == 0x34 && linked == convoy_id)
}
