//! Gestor de combates: reserva básica `SeaBattles::allocate_basic_battle` (0x00611CD0).
use super::{r32, w32};
/// Asignación y construcción del objeto 0x1E98. El anfitrión conserva la memoria.
pub trait BattleAllocator {
    fn allocate(&mut self, size: u32) -> u32;
    fn construct(&mut self, pointer: u32, slot: u8) -> u32;
}
/// Devuelve la primera ranura libre 0..=0xFE, o 0xFF si no existe.
/// Como el original, un fallo de memoria deja la ranura nula pero devuelve su índice.
pub fn allocate_basic_battle(manager: &mut [u8], host: &mut dyn BattleAllocator) -> Option<u8> {
    if manager.len() < 0x405 {
        return None;
    }
    for slot in 0..255usize {
        if r32(manager, slot * 4) == 0 {
            let raw = host.allocate(0x1e98);
            let object = if raw == 0 { 0 } else { host.construct(raw, slot as u8) };
            w32(manager, slot * 4, object as i32);
            if usize::from(manager[0x404]) < slot + 1 {
                manager[0x404] = (slot + 1) as u8
            }
            return Some(slot as u8);
        }
    }
    Some(0xff)
}
/// Reserva `TownAttackBattle` (0x00611D80), con constructor proporcionado
/// por el anfitrión. Un fallo de asignación conserva el comportamiento del EXE.
pub fn allocate_town_attack_battle(manager: &mut [u8], host: &mut dyn BattleAllocator) -> Option<u8> {
    if manager.len() < 0x405 {
        return None;
    }
    for slot in 0..255usize {
        if r32(manager, slot * 4) == 0 {
            let raw = host.allocate(0x2f24);
            let object = if raw == 0 { 0 } else { host.construct(raw, slot as u8) };
            w32(manager, slot * 4, object as i32);
            if usize::from(manager[0x404]) < slot + 1 {
                manager[0x404] = (slot + 1) as u8
            }
            return Some(slot as u8);
        }
    }
    Some(0xff)
}
/// Constructor derivado de ataque urbano invocado por 0x00611D80. Conserva
/// los bytes no inicializados; todavía no inicia ni serializa una batalla.
pub fn construct_town_attack_battle(object: &mut [u8], manager: &mut [u8], address: u32, index: u32) -> Option<u32> {
    if object.len() < 0x2f24 || manager.len() < 0x400 {
        return None;
    }
    construct_basic_battle(object, manager, address, index)?;
    w32(object, 0, 0x0067aaa0);
    object[0x2f1c..0x2f22].fill(0);
    Some(address)
}
/// Constructor básico `SeaBattle::SeaBattle` (0x00603A70), incluyendo base.
/// Conserva bytes no inicializados y publica la dirección lógica en la tabla.
pub fn construct_basic_battle(object: &mut [u8], manager: &mut [u8], address: u32, index: u32) -> Option<u32> {
    if object.len() < 0x1e98 || manager.len() < 0x400 {
        return None;
    }
    let slot = index as u8;
    w32(object, 0, 0x0067ad44);
    w32(manager, usize::from(slot) * 4, address as i32);
    for &o in &[
        0x04usize, 0x08, 0x0c, 0x18, 0x1c, 0x24, 0x28, 0x2c, 0x30, 0x38, 0x3c, 0x40, 0x48, 0x4c, 0x50,
    ] {
        w32(object, o, 0)
    }
    for &o in &[0x10usize, 0x12, 0x20, 0x34, 0x44, 0x15c, 0x65c, 0xac6, 0x130c] {
        super::w16(object, o, u16::MAX)
    }
    for &o in &[
        0x14usize, 0x16, 0x54, 0x56, 0x58, 0x5a, 0x65a, 0x65e, 0x660, 0xacc, 0xace, 0xad0,
    ] {
        super::w16(object, o, 0)
    }
    super::w16(object, 0x662, u16::from(slot));
    super::w16(object, 0x1e94, 0x800);
    for &(o, v) in &[
        (0xa7c, 0x2200),
        (0xa80, 0x136c),
        (0xae0, -1),
        (0x12e4, 0),
        (0x664, 0),
        (0x668, 0),
        (0x66c, 0),
        (0xa74, 0),
        (0xa78, 0),
    ] {
        w32(object, o, v)
    }
    for &o in &[
        0x558usize, 0x12e0, 0xa8c, 0x12e8, 0x130f, 0xac9, 0xacb, 0xaca, 0x12ea, 0x12e9, 0x1e92,
    ] {
        object[o] = 0
    }
    object[0xac4] = 0xff;
    object[0x1b10..0x1e90].fill(0);
    for i in 0..64 {
        super::w16(object, 0x672 + i * 8, u16::MAX);
        super::w16(object, 0x872 + i * 8, u16::MAX)
    }
    Some(address)
}
/// Cabecera propia de `TownAttackBattle::serialize` (0x005FBEA0).
/// El serializador base escribe inmediatamente después de estos 29 bytes.
/// La marca del segundo bando se actualiza igual que en el original.
pub fn town_attack_save_prefix(object: &mut [u8]) -> Result<[u8; 29], &'static str> {
    if object.len() < 0x2f1a {
        return Err("objeto de ataque urbano truncado");
    }
    if super::r16(object, 0x2f18) != 0 {
        object[0x5e] = 1
    }
    let mut out = [0u8; 29];
    out[0] = object[0x1e98];
    for i in 0..4 {
        let dst = 1 + 7 * i;
        let src = 0x1ec4 + 4 * i;
        out[dst..dst + 4].copy_from_slice(&object[src..src + 4]);
        out[dst + 4] = object[0x1eae + i];
        out[dst + 5] = object[0x1eb2 + i];
        out[dst + 6] = object[0x1ebe + i];
    }
    Ok(out)
}
/// Lee los 29 bytes propios de `TownAttackBattle::deserialize` (0x005FBF30).
/// El llamante continúa con el lector base en el cursor devuelto.
pub fn town_attack_load_prefix(object: &mut [u8], input: &[u8]) -> Result<usize, &'static str> {
    if object.len() < 0x2f1b || input.len() < 29 {
        return Err("ataque urbano o prefijo truncado");
    }
    object[0x1e98] = input[0];
    for i in 0..4 {
        let src = 1 + 7 * i;
        let dst = 0x1ec4 + 4 * i;
        object[dst..dst + 4].copy_from_slice(&input[src..src + 4]);
        object[0x1eae + i] = input[src + 4];
        object[0x1eb2 + i] = input[src + 5];
        object[0x1ebe + i] = input[src + 6];
        object[0x1e99 + i] = 10;
    }
    object[0x2ef8..0x2f1a].fill(0);
    object[0x2f1a] = 1;
    Ok(29)
}
/// Tamaño del formato actual `SeaBattle::serialized_size` (0x00604140).
/// Rechaza listas cíclicas o índices/bitmap ausentes en vez de leer fuera de memoria.
pub fn serialized_battle_size(object: &[u8], nexts: &[u16], bitmap: &[u32]) -> Result<u32, &'static str> {
    if object.len() < 0xad0 {
        return Err("objeto de batalla truncado");
    }
    let n = usize::from(super::r16(object, 0x14));
    let slots = usize::from(super::r16(object, 0xacc));
    let words = usize::from(super::r16(object, 0xace));
    let participants = usize::from(object[0x558]);
    if nexts.len() < n || bitmap.len() < (slots + 31) / 32 {
        return Err("listas de batalla truncadas");
    }
    let mut visits = 0u32;
    for o in [0x10usize, 0x65c] {
        let mut id = super::r16(object, o);
        let mut seen = vec![false; nexts.len()];
        while id != u16::MAX {
            let i = usize::from(id);
            if i >= n || seen[i] {
                return Err("lista de batalla inválida");
            }
            seen[i] = true;
            visits = visits.wrapping_add(1);
            id = nexts[i]
        }
    }
    let active = (0..slots).filter(|&i| bitmap[i / 32] & (1u32 << (i % 32)) != 0).count() as u32;
    Ok(0x431u32
        .wrapping_add(((words + participants + n) as u32).wrapping_mul(4))
        .wrapping_add(visits.wrapping_mul(47))
        .wrapping_add(active.wrapping_mul(31)))
}
/// Recursos de las listas para serializar el formato actual del combate.
pub struct BattleSaveInput<'a> {
    pub object: &'a [u8],
    pub nexts: &'a [u16],
    pub prevs: &'a [u16],
    pub bitmap: &'a [u32],
    pub local_ships: &'a [Vec<u8>],
    pub projectiles: &'a [Vec<u8>],
}
/// Escritor base `SeaBattle::serialize` (0x00604220), versión actual.
/// No serializa el gestor ni la variante de ataque a ciudad.
pub fn serialize_basic_battle(v: &BattleSaveInput<'_>) -> Result<Vec<u8>, &'static str> {
    let o = v.object;
    if o.len() < 0x1e98 {
        return Err("objeto de batalla truncado");
    }
    let n = usize::from(super::r16(o, 0x14));
    let p = usize::from(o[0x558]);
    let slots = usize::from(super::r16(o, 0xacc));
    let words = usize::from(super::r16(o, 0xace));
    if v.prevs.len() < n
        || v.nexts.len() < n
        || v.bitmap.len() < words
        || v.bitmap.len() < (slots + 31) / 32
        || v.local_ships.len() < n
        || v.projectiles.len() < slots
    {
        return Err("listas de batalla truncadas");
    }
    let size = serialized_battle_size(o, v.nexts, v.bitmap)? as usize;
    let mut out = Vec::with_capacity(size);
    for &(at, len) in &[
        (0x558, 1),
        (0x658, 2),
        (0x65a, 2),
        (0x10, 2),
        (0x65c, 2),
        (0x65e, 2),
        (0x660, 2),
        (0xad4, 2),
        (0xad6, 2),
        (0xad8, 2),
        (0xada, 2),
        (0xadc, 4),
        (0x12e8, 1),
        (0x130c, 2),
        (0x130e, 1),
        (0x5c, 1),
        (0x670, 1),
        (0x671, 1),
        (0x1e90, 1),
        (0x1e91, 1),
        (0xaca, 1),
        (0xacb, 1),
        (0x1e94, 2),
        (0xad2, 1),
    ] {
        out.extend_from_slice(&o[at..at + len])
    }
    for i in 0..p {
        out.extend_from_slice(&o[0x35a + 2 * i..0x35c + 2 * i]);
        out.push(o[0x5d + i]);
        out.push(o[0x559 + i]);
        if i < 2 {
            out.extend_from_slice(&o[0xa84 + 4 * i..0xa88 + 4 * i])
        }
    }
    out.extend_from_slice(&(words as u16).to_le_bytes());
    for &word in &v.bitmap[..words] {
        out.extend_from_slice(&word.to_le_bytes())
    }
    for &id in &v.nexts[..n] {
        out.extend_from_slice(&id.to_le_bytes())
    }
    for &id in &v.prevs[..n] {
        out.extend_from_slice(&id.to_le_bytes())
    }
    out.extend_from_slice(&o[0x672..0xa72]);
    for head in [super::r16(o, 0x10), super::r16(o, 0x65c)] {
        let mut id = head;
        while id != u16::MAX {
            let i = usize::from(id);
            let ship = &v.local_ships[i];
            if ship.len() < 0x164 {
                return Err("barco local truncado");
            }
            out.extend_from_slice(&serialize_local_ship(ship)?);
            id = v.nexts[i]
        }
    }
    for i in 0..slots {
        if v.bitmap[i / 32] & (1u32 << (i % 32)) == 0 {
            continue;
        }
        let projectile = &v.projectiles[i];
        if projectile.len() < 0x26 {
            return Err("proyectil truncado");
        }
        for &(at, len) in &[
            (0, 2),
            (2, 2),
            (4, 2),
            (6, 2),
            (8, 2),
            (0xa, 2),
            (0xc, 4),
            (0x10, 4),
            (0x14, 4),
            (0x18, 1),
            (0x1c, 4),
            (0x24, 2),
        ] {
            out.extend_from_slice(&projectile[at..at + len])
        }
    }
    Ok(out)
}
/// Operación 0x9F que puede emitir `ships::start_sea_battle` (0x0050B250).
/// Solo cuenta testigos; no reserva ni inicia el combate. Rechaza listas inválidas.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BattleWitnessNotice {
    pub slot: u16,
    pub count: u32,
}
pub fn battle_witness_notice(
    manager: &[u8],
    ships: &[u8],
    convoys: &[u8],
    merchants: &[u8],
    merchant_count: u16,
    player: u32,
    attacker: u16,
    target: u16,
    slot: u8,
) -> Result<Option<BattleWitnessNotice>, &'static str> {
    use super::r16;
    use super::remove::{CONVOY_SIZE, MERCHANT_SIZE, SHIP_SIZE};
    if manager.len() < 0xfe {
        return Err("gestor truncado");
    }
    let n = r16(manager, 0xf4);
    let nc = r16(manager, 0xf6);
    if attacker >= n
        || target >= n
        || ships.len() < usize::from(n) * SHIP_SIZE
        || convoys.len() < usize::from(nc) * CONVOY_SIZE
        || merchants.len() < usize::from(merchant_count) * MERCHANT_SIZE
    {
        return Err("flota truncada");
    }
    let a = usize::from(attacker) * SHIP_SIZE;
    let t = usize::from(target) * SHIP_SIZE;
    if u16::from(ships[a]) >= merchant_count
        || ships[a + 0x15c] == 0
        || r16(ships, a + 0x134) == 0x16
        || u16::from(ships[t]) >= merchant_count
        || ships[t + 0x15c] != 0
        || player >= u32::from(merchant_count)
    {
        return Ok(None);
    }
    let mut total = 0u32;
    let mut id = r16(merchants, player as usize * MERCHANT_SIZE + 0xe);
    let mut seen = vec![false; usize::from(n)];
    while id < n {
        let i = usize::from(id);
        if seen[i] {
            return Err("lista de comerciante cíclica");
        }
        seen[i] = true;
        let b = i * SHIP_SIZE;
        let mut add = |who: u16| -> Result<(), &'static str> {
            let cv = r16(ships, usize::from(who) * SHIP_SIZE + 8);
            let size = if cv < nc {
                let mut cur = r16(convoys, usize::from(cv) * CONVOY_SIZE + 0xa);
                let mut visited = vec![false; usize::from(n)];
                let mut count = 0u16;
                while cur < n {
                    let j = usize::from(cur);
                    if visited[j] {
                        return Err("lista de convoy cíclica");
                    }
                    visited[j] = true;
                    count = count.wrapping_add(1);
                    cur = r16(ships, j * SHIP_SIZE + 6)
                }
                u32::from(count)
            } else {
                1
            };
            total = total.wrapping_add(size);
            Ok(())
        };
        if player == u32::from(ships[a]) {
            for off in [0x50, 0x52] {
                let who = r16(ships, b + off);
                if who < n {
                    let q = usize::from(who) * SHIP_SIZE;
                    let dx = i32::from(r16(ships, a + 0x1e) as i16) - i32::from(r16(ships, q + 0x1e) as i16);
                    let dy = i32::from(r16(ships, a + 0x22) as i16) - i32::from(r16(ships, q + 0x22) as i16);
                    if dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy)) < i32::from(r16(manager, 0xfc)) {
                        add(who)?
                    }
                }
            }
        } else if r16(ships, b + 0x50) == attacker || r16(ships, b + 0x52) == attacker {
            add(id)?
        }
        id = r16(ships, b + 4);
    }
    Ok((total != 0).then_some(BattleWitnessNotice {
        slot: u16::from(slot),
        count: total,
    }))
}

/// Efectos producidos al retirar convoyes de la navegación para el combate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BattleConvoyPreparation {
    pub route_cache_invalidations: Vec<u16>,
}
/// Tramo de `start_sea_battle` entre la reserva y `SeaBattle::start`, limitado
/// a dos convoyes comerciales ya existentes y distintos, con rutas compartidas o ausentes.
/// La reserva, la operación 0x9F y el inicio del combate son responsabilidad
/// del llamante. En error no modifica las imágenes suministradas.
pub fn prepare_existing_battle_convoys(
    manager: &mut [u8],
    ships: &mut [u8],
    convoys: &mut [u8],
    merchant_count: u16,
    attacker: u16,
    target: u16,
) -> Result<BattleConvoyPreparation, &'static str> {
    use super::order::XOrder;
    use super::remove::{CONVOY_SIZE, SHIP_SIZE};
    use super::{r16, r32, w16, w32};
    if manager.len() < 0xf8 {
        return Err("gestor truncado");
    }
    let n = r16(manager, 0xf4);
    let nc = r16(manager, 0xf6);
    if attacker >= n
        || target >= n
        || ships.len() < usize::from(n) * SHIP_SIZE
        || convoys.len() < usize::from(nc) * CONVOY_SIZE
    {
        return Err("flota truncada");
    }
    let ids = [attacker, target];
    let mut cvs = [0u16; 2];
    for (i, id) in ids.iter().enumerate() {
        let a = usize::from(*id) * SHIP_SIZE;
        if u16::from(ships[a]) >= merchant_count {
            return Err("bando no comercial no implementado");
        }
        cvs[i] = r16(ships, a + 8);
        if cvs[i] >= nc {
            return Err("barco sin convoy existente");
        }
    }
    if cvs[0] == cvs[1] {
        return Err("bandos del mismo convoy");
    }
    let mut s = ships.to_vec();
    let mut c = convoys.to_vec();
    let mut m = manager.to_vec();
    let mut invalidations = Vec::new();
    let sea = r16(&m, 0xea);
    for id in ids {
        XOrder {
            ships: &mut s,
            count: n,
            sea_head: sea,
        }
        .remove(id)
    }
    for (side, id) in ids.into_iter().enumerate() {
        let cv = cvs[side];
        let b = usize::from(cv) * CONVOY_SIZE;
        let mut head = r16(&m, 0xec);
        let mut previous = None;
        let mut seen = vec![false; usize::from(nc)];
        while head < nc && head != cv {
            let i = usize::from(head);
            if seen[i] {
                return Err("lista de convoyes cíclica");
            }
            seen[i] = true;
            previous = Some(i * CONVOY_SIZE + 8);
            head = r16(&c, i * CONVOY_SIZE + 8)
        }
        if head == cv {
            let next = r16(&c, b + 8);
            if let Some(p) = previous {
                w16(&mut c, p, next)
            } else {
                w16(&mut m, 0xec, next)
            }
            w16(&mut c, b + 8, u16::MAX);
            let leader = r16(&c, b + 0x10);
            if leader < n {
                XOrder {
                    ships: &mut s,
                    count: n,
                    sea_head: sea,
                }
                .remove(leader);
                invalidations.push(leader)
            }
        }
        let leader = r16(&c, b + 0x10);
        if leader >= n {
            return Err("líder de convoy inválido");
        }
        let mut sh = r16(&c, b + 0xa);
        let mut visited = vec![false; usize::from(n)];
        let mut members = Vec::new();
        while sh < n {
            let j = usize::from(sh);
            if visited[j] {
                return Err("lista de barcos cíclica");
            }
            visited[j] = true;
            let p = j * SHIP_SIZE;
            members.push(sh);
            sh = r16(&s, p + 6)
        }
        if members.is_empty() {
            return Err("convoy vacío");
        }
        for &member in &members {
            XOrder {
                ships: &mut s,
                count: n,
                sea_head: sea,
            }
            .remove(member)
        }
        let _ = id;
        w16(&mut c, b + 0x12, 0x14);
        let flags = r32(&c, b + 0x18) | 0x20;
        w32(&mut c, b + 0x18, flags);
        for member in members {
            let p = usize::from(member) * SHIP_SIZE;
            let flags = r16(&s, p + 0x3c) | 0x20;
            w16(&mut s, p + 0x3c, flags);
            w16(&mut s, p + 0x134, 0x14);
            w16(&mut s, p + 0x50, u16::MAX);
            w16(&mut s, p + 0x52, u16::MAX);
            let lead = usize::from(leader) * SHIP_SIZE;
            if member != leader && r32(&s, p + 0x24) == 0 && r32(&s, lead + 0x24) != 0 {
                if s[lead + 0x36] != 0 {
                    return Err("copia de ruta propia no implementada");
                }
                for &(off, len) in &[
                    (0x1c, 4),
                    (0x20, 4),
                    (0x28, 4),
                    (0x2c, 4),
                    (0x30, 4),
                    (0x34, 2),
                    (2, 2),
                    (0x24, 4),
                ] {
                    let data = s[lead + off..lead + off + len].to_vec();
                    s[p + off..p + off + len].copy_from_slice(&data)
                }
                s[p + 0x36] = 0
            }
        }
        if s[usize::from(leader) * SHIP_SIZE + 0x15c] != 0 {
            c[b + 0x14] |= 0x20
        }
    }
    for id in ids {
        XOrder {
            ships: &mut s,
            count: n,
            sea_head: sea,
        }
        .remove(id)
    }
    manager.copy_from_slice(&m);
    ships.copy_from_slice(&s);
    convoys.copy_from_slice(&c);
    Ok(BattleConvoyPreparation {
        route_cache_invalidations: invalidations,
    })
}

/// Estado al final del prefijo de `SeaBattle::start` (0x00605420).
/// `Rejected` requiere todavía la llamada virtual de limpieza del original;
/// `Continue` requiere construir barcos locales y terminar el inicio.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleStartPrefix {
    Rejected,
    Continue,
}
pub fn start_battle_prefix(
    object: &mut [u8],
    ships: &[u8],
    attacker: u16,
    target: u16,
    skip_expansion: u8,
) -> Result<BattleStartPrefix, &'static str> {
    use super::remove::SHIP_SIZE;
    use super::{r16, r32, w16, w32};
    let a = usize::from(attacker) * SHIP_SIZE;
    let b = usize::from(target) * SHIP_SIZE;
    if object.len() < 0x130e || a + SHIP_SIZE > ships.len() || b + SHIP_SIZE > ships.len() {
        return Err("objeto o barcos truncados");
    }
    object[0x12e8] = skip_expansion;
    object[0x558] = 2;
    w16(object, 0x35a, u16::from(ships[a]));
    w16(object, 0x35c, u16::from(ships[b]));
    object[0x559] = 0;
    object[0x55a] = 1;
    let as_ = r16(ships, a + 0x134);
    let bs = r16(ships, b + 0x134);
    if as_ == 0x15 || bs == 0x15 || (ships[a] == 0xff && ships[b] == 0xff) {
        w16(object, 0x15c, 0);
        object[0x5c] = 1;
        w16(object, 0x130c, u16::MAX);
        w16(object, 0x658, 0);
        w16(object, 0x65a, 0);
        w16(object, 0x65e, if as_ == 0x15 { u16::MAX } else { r16(ships, a + 8) });
        w16(object, 0x660, if bs == 0x15 { u16::MAX } else { r16(ships, b + 8) });
        return Ok(BattleStartPrefix::Rejected);
    }
    object[0xad2] = u8::from(ships[a] == 0xff || ships[a + 0x15c] != 0)
        | if ships[b] == 0xff || ships[b + 0x15c] != 0 {
            2
        } else {
            0
        };
    let x = r32(ships, a + 0x1c).wrapping_add(r32(ships, b + 0x1c)) >> 1;
    let y = r32(ships, a + 0x20).wrapping_add(r32(ships, b + 0x20)) >> 1;
    w32(object, 0xad4, x);
    w32(object, 0xad8, y);
    w16(object, 0x65e, r16(ships, a + 8));
    w16(object, 0x660, r16(ships, b + 8));
    w32(
        object,
        0xadc,
        ((y as u32 & 0xffff) << 16).wrapping_add(x as u32 & 0xffff) as i32,
    );
    Ok(BattleStartPrefix::Continue)
}
/// Avisos del tramo de clasificación de participantes de `SeaBattle::start`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BattleStartEvent {
    Notice {
        merchant: u16,
        x: u16,
        y: u16,
        kind: u32,
        ship: u32,
    },
    Sync,
}
/// Clasifica los dos bandos ya preparados por `start_battle_prefix`.
/// `side0_pirate_band` es el resultado de la consulta 0x00503AC0.
pub fn classify_battle_sides(
    object: &mut [u8],
    convoys: &[u8],
    merchants: &[u8],
    merchant_count: u16,
    side0_pirate_band: bool,
    hotseat: bool,
    attacker: u16,
    target: u16,
) -> Result<Vec<BattleStartEvent>, &'static str> {
    use super::remove::{CONVOY_SIZE, MERCHANT_SIZE};
    use super::{r16, w16};
    if object.len() < 0xadc || merchants.len() < usize::from(merchant_count) * MERCHANT_SIZE {
        return Err("batalla o comerciantes truncados");
    }
    let side1 = usize::from(r16(object, 0x660)) * CONVOY_SIZE;
    if side1 + CONVOY_SIZE > convoys.len() {
        return Err("convoy truncado");
    }
    if side0_pirate_band {
        object[0x5d] = 1;
        w16(object, 0x35a, 0xfe)
    } else {
        let owner = r16(object, 0x35a);
        object[0x5d] = u8::from(owner >= merchant_count || r16(merchants, usize::from(owner) * MERCHANT_SIZE + 8) != 0)
    }
    let status = r16(convoys, side1 + 0x12);
    if status == 9 || status == 0x16 {
        object[0x5e] = 1;
        w16(object, 0x35c, 0xfe)
    } else {
        let owner = r16(object, 0x35c);
        object[0x5e] = u8::from(owner >= merchant_count || r16(merchants, usize::from(owner) * MERCHANT_SIZE + 8) != 0)
    }
    if hotseat && object[0x5d] == 0 && object[0x5e] == 0 && r16(object, 0x35a) != r16(object, 0x35c) {
        object[0x5d] = 1;
        object[0x5e] = 1
    }
    let mut events = Vec::new();
    if object[0x5d] != 0 && object[0x5e] != 0 {
        object[0xac8] = 1
    } else {
        object[0xac8] = 0;
        let x = r16(object, 0xad6);
        let y = r16(object, 0xada);
        let a = r16(object, 0x35a);
        let b = r16(object, 0x35c);
        if object[0x5d] == 0 {
            events.push(BattleStartEvent::Notice {
                merchant: a,
                x,
                y,
                kind: 14,
                ship: u32::from(attacker),
            })
        }
        if object[0x5e] == 0 && a != b {
            events.push(BattleStartEvent::Notice {
                merchant: b,
                x,
                y,
                kind: 11,
                ship: u32::from(target),
            })
        }
        if !hotseat {
            events.push(BattleStartEvent::Sync)
        }
    }
    Ok(events)
}
/// Constructor completo `init_local_map_ship` (0x0061A4E0). El objeto conserva
/// los bytes que el original no inicializa; `battle_address` es la referencia
/// lógica que administra el anfitrión, no un puntero nativo de Rust.
pub fn construct_local_ship(
    object: &mut [u8],
    world_ship: &[u8],
    world_index: u32,
    battle_address: u32,
    local_index: u16,
    side: u8,
) -> Result<(), &'static str> {
    use super::{r16, r32, w16, w32};
    if object.len() < 0x164 || world_ship.len() < 0x180 {
        return Err("barco local o mundial truncado");
    }
    w32(object, 0, 0x67b0d4);
    w32(object, 4, world_index as i32);
    w32(object, 0x12c, battle_address as i32);
    w16(object, 0x138, local_index);
    w16(object, 0x128, u16::MAX);
    object[0x12a] = 0;
    object[0x130] = 0;
    object[0x131] = 0;
    object[0x132] = 0;
    w32(object, 0x134, 0);
    w32(object, 0x1c, 0);
    object[0x149] = 0;
    w16(object, 0x13a, 0);
    object[0x13d] = side;
    object[0x142] = side;
    object[0x13e] = 0;
    w16(object, 0x140, r16(world_ship, 0x40));
    w32(object, 0x144, r32(world_ship, 0x18));
    object[0x14a] = 1;
    for off in [8, 0xa, 0xc, 0xe] {
        w16(object, off, 0)
    }
    object[0x11] = 0;
    object[0x10] = 0x80;
    object[0x127] = 1;
    object[0x162] = 0;
    object[0x26] = 4;
    object[0x148] = 4;
    object[0x27..0x127].fill(0);
    const W: [i32; 6] = [60, 80, 60, 80, 90, 90];
    const F: [i32; 6] = [32, 32, 77, 77, 96, 58];
    const D: [i32; 6] = [480, 160, 480, 160, 120, 120];
    let (mut sum, mut adjusted) = (0i32, 0i32);
    for &kind in &world_ship[0x13c..0x154] {
        if kind < 6 {
            let i = usize::from(kind);
            let power = W[i] * F[i];
            sum += power;
            adjusted += power - (power * 480) / (D[i] * 6)
        }
    }
    object[0x13c] = u8::from(sum * 301 > adjusted * 605);
    w32(object, 0x14c, -1);
    w32(object, 0x150, 0);
    object[0x160] = 0;
    w32(object, 0x15c, 0);
    w32(object, 0x158, 0);
    object[0x154] = 0xff;
    object[0x161] = 12;
    Ok(())
}

/// Fuerza de artillería `LocalMapShip::get_battle_artillery_strength` 0x0062125B.
/// Conserva los productos de 32 bits y la división firmada del original.
pub fn local_ship_artillery_strength(ship: &[u8], captains: &[u8], captain_count: u16) -> Result<i32, &'static str> {
    use super::{r16, r32};
    if ship.len() < 0x180 || captains.len() < usize::from(captain_count) * 16 {
        return Err("barco o capitanes truncados");
    }
    let mut strength = r32(ship, 0x120).wrapping_mul(r32(ship, 0x18)) >> 10;
    let cap = r16(ship, 0x42);
    if cap < captain_count {
        strength = strength.wrapping_add(i32::from(captains[usize::from(cap) * 16 + 0xb]).wrapping_mul(strength) / 850)
    }
    let upkeep = ship[0x3f] as i8;
    if upkeep < 1 {
        strength = strength.wrapping_add((-(i64::from(strength)) / 20) as i32)
    } else if upkeep > 2 {
        strength = strength.wrapping_add(i32::from(upkeep.min(4) - 2).wrapping_mul(strength) / 20)
    }
    Ok(strength)
}
/// Carga versionada `LocalMapShip::deserialize` (0x0061A880). Reconstruye
/// los campos serializados sobre un objeto ya asignado y devuelve bytes leídos.
/// Ante datos truncados deja el objeto intacto.
pub fn deserialize_local_ship(
    object: &mut [u8],
    input: &[u8],
    version: u32,
    battle_address: u32,
) -> Result<usize, &'static str> {
    use super::w32;
    if object.len() < 0x164 {
        return Err("barco local truncado");
    }
    let mut fields = Vec::new();
    fields.push((0x128, if version > 0x5b { 2 } else { 1 }));
    fields.extend_from_slice(&[(0x12a, 1), (0x130, 1), (0x131, 1), (0x132, 1), (0x13a, 2), (0x13d, 1)]);
    if version > 0x21 {
        fields.extend_from_slice(&[(0x13e, 1), (0x140, 2)])
    }
    if version > 0x23 {
        fields.extend_from_slice(&[(0x142, 1), (0x144, 4)])
    }
    if version > 0x56 {
        fields.push((0x162, 1))
    }
    fields.extend_from_slice(&[
        (0x04, 4),
        (0x08, 2),
        (0x0a, 2),
        (0x0c, 2),
        (0x0e, 2),
        (0x10, 1),
        (0x11, 1),
        (0x14, 4),
        (0x18, 4),
        (0x1c, 4),
        (0x24, 2),
        (0x26, 1),
    ]);
    let required: usize = fields.iter().map(|(_, n)| n).sum();
    if input.len() < required {
        return Err("datos de barco local truncados");
    }
    w32(object, 0x12c, battle_address as i32);
    let mut cursor = 0;
    for (off, n) in fields {
        object[off..off + n].copy_from_slice(&input[cursor..cursor + n]);
        if off == 0x128 && n == 1 {
            object[0x129] = if input[cursor] == 0xff { 0xff } else { 0 }
        }
        cursor += n
    }
    object[0x161] = 0;
    object[0x148] = object[0x26];
    Ok(cursor)
}
/// Escritor `LocalMapShip::serialize` (0x0061A790), formato actual de 47 bytes.
pub fn serialize_local_ship(object: &[u8]) -> Result<Vec<u8>, &'static str> {
    if object.len() < 0x164 {
        return Err("barco local truncado");
    }
    let mut out = Vec::with_capacity(47);
    for &(at, len) in &[
        (0x128, 2),
        (0x12a, 1),
        (0x130, 1),
        (0x131, 1),
        (0x132, 1),
        (0x13a, 2),
        (0x13d, 1),
        (0x13e, 1),
        (0x140, 2),
        (0x142, 1),
        (0x144, 4),
        (0x162, 1),
        (0x04, 4),
        (0x08, 2),
        (0x0a, 2),
        (0x0c, 2),
        (0x0e, 2),
        (0x10, 1),
        (0x11, 1),
        (0x14, 4),
        (0x18, 4),
        (0x1c, 4),
        (0x24, 2),
        (0x26, 1),
    ] {
        out.extend_from_slice(&object[at..at + len])
    }
    Ok(out)
}
/// Primer tramo de los barcos de un bando; los IDs espaciales los asigna el
/// gestor de colisiones y se pasan de forma explícita para conservar su orden.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalBattleSide {
    pub ships: Vec<Vec<u8>>,
    pub next: Vec<u16>,
    pub previous: Vec<u16>,
    pub spatial_0: Vec<u16>,
    pub spatial_1: Vec<u16>,
}
/// Construye y ordena un bando de `SeaBattle::start`. Los índices de
/// barco deben estar ya filtrados por participante y ordenados por convoy.
/// No modifica la batalla ni crea las estructuras reales de colisión.
pub fn build_battle_side(
    ships: &[u8],
    captains: &[u8],
    captain_count: u16,
    world_indices: &[u16],
    battle_address: u32,
    spatial_ids: &[(u16, u16)],
    side: u8,
    start_index: u16,
) -> Result<LocalBattleSide, &'static str> {
    use super::remove::SHIP_SIZE;
    if world_indices.len() != spatial_ids.len()
        || usize::from(start_index) + world_indices.len() > usize::from(u16::MAX)
    {
        return Err("índices espaciales incompatibles");
    }
    let mut ordered = Vec::with_capacity(world_indices.len());
    for (i, &world) in world_indices.iter().enumerate() {
        let at = usize::from(world) * SHIP_SIZE;
        let image = ships.get(at..at + SHIP_SIZE).ok_or("barco mundial ausente")?;
        let mut object = vec![0; 0x164];
        construct_local_ship(
            &mut object,
            image,
            u32::from(world),
            battle_address,
            start_index + i as u16,
            side,
        )?;
        let strength = local_ship_artillery_strength(image, captains, captain_count)?;
        ordered.push((strength, object))
    }
    ordered.sort_by(|a, b| b.0.cmp(&a.0));
    let mut local = Vec::with_capacity(ordered.len());
    for (i, (_, mut object)) in ordered.into_iter().enumerate() {
        super::w16(&mut object, 0x138, start_index + i as u16);
        local.push(object)
    }
    let n = local.len();
    Ok(LocalBattleSide {
        ships: local,
        next: (0..n)
            .map(|i| {
                if i + 1 < n {
                    start_index + (i + 1) as u16
                } else {
                    u16::MAX
                }
            })
            .collect(),
        previous: (0..n)
            .map(|i| if i == 0 { u16::MAX } else { start_index + (i - 1) as u16 })
            .collect(),
        spatial_0: spatial_ids.iter().map(|x| x.0).collect(),
        spatial_1: spatial_ids.iter().map(|x| x.1).collect(),
    })
}
/// Selecciona los miembros que el inicio de una batalla comercial incorpora
/// para cada participante. Rechaza listas cíclicas y participantes especiales,
/// que requieren la investigación pirata del original.
pub fn select_commercial_battle_ships(
    ships: &[u8],
    convoys: &[u8],
    convoy_ids: [u16; 2],
    participants: [u16; 2],
) -> Result<[Vec<u16>; 2], &'static str> {
    use super::r16;
    use super::remove::{CONVOY_SIZE, SHIP_SIZE};
    if ships.len() % SHIP_SIZE != 0 || convoys.len() % CONVOY_SIZE != 0 {
        return Err("flota truncada");
    }
    let n = ships.len() / SHIP_SIZE;
    if n > usize::from(u16::MAX) || participants.iter().any(|&p| p >= 0xfe) {
        return Err("participante especial no implementado");
    }
    let mut result = [Vec::new(), Vec::new()];
    for side in 0..2 {
        let cv = usize::from(convoy_ids[side]) * CONVOY_SIZE;
        if cv + CONVOY_SIZE > convoys.len() {
            return Err("convoy truncado");
        }
        let mut id = r16(convoys, cv + 0xa);
        let mut seen = vec![false; n];
        while usize::from(id) < n {
            let i = usize::from(id);
            if seen[i] {
                return Err("lista de barcos cíclica");
            }
            seen[i] = true;
            let at = i * SHIP_SIZE;
            if u16::from(ships[at]) == participants[side] {
                result[side].push(id)
            }
            id = r16(ships, at + 6)
        }
    }
    Ok(result)
}
/// Selección, construcción y orden de ambos bandos comerciales. Los ID de
/// colisión proceden del anfitrión, dos por cada barco seleccionado.
pub fn build_commercial_battle_sides(
    ships: &[u8],
    convoys: &[u8],
    captains: &[u8],
    captain_count: u16,
    convoy_ids: [u16; 2],
    participants: [u16; 2],
    battle_address: u32,
    spatial_ids: &[(u16, u16)],
) -> Result<[LocalBattleSide; 2], &'static str> {
    let ids = select_commercial_battle_ships(ships, convoys, convoy_ids, participants)?;
    let n = ids[0].len();
    if spatial_ids.len() != n + ids[1].len() {
        return Err("índices espaciales incompatibles");
    }
    let a = build_battle_side(
        ships,
        captains,
        captain_count,
        &ids[0],
        battle_address,
        &spatial_ids[..n],
        0,
        0,
    )?;
    let b = build_battle_side(
        ships,
        captains,
        captain_count,
        &ids[1],
        battle_address,
        &spatial_ids[n..],
        1,
        n as u16,
    )?;
    Ok([a, b])
}
/// Tablas ya reservadas del gestor espacial `LocalMapShipList`.
/// Las coordenadas y enlaces tienen dos extremos por objeto.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BattleCollisionPool {
    pub kinds: Vec<u8>,
    pub owners: Vec<u16>,
    pub free_next: Vec<u16>,
    pub x: Vec<u16>,
    pub x_next: Vec<u16>,
    pub x_previous: Vec<u16>,
    pub y: Vec<u16>,
    pub y_next: Vec<u16>,
    pub y_previous: Vec<u16>,
    pub matrices: [Vec<Vec<u32>>; 3],
    pub free_head: u16,
    pub x_head: u16,
    pub y_head: u16,
    pub high_water: u16,
}
impl BattleCollisionPool {
    /// Inicializa una capacidad ya disponible; la ampliación original es por
    /// bloques y todavía no se reproduce aquí.
    pub fn with_capacity(n: u16) -> Result<Self, &'static str> {
        if n > 0x7fff {
            return Err("capacidad espacial excesiva");
        }
        let z = vec![0; usize::from(n) * 2];
        Ok(Self {
            kinds: vec![0; usize::from(n)],
            owners: vec![0; usize::from(n)],
            free_next: (0..n).map(|i| if i + 1 < n { i + 1 } else { u16::MAX }).collect(),
            x: z.clone(),
            x_next: z.clone(),
            x_previous: z.clone(),
            y: z.clone(),
            y_next: z.clone(),
            y_previous: z,
            matrices: std::array::from_fn(|_| {
                (0..(usize::from(n) + 31) / 32)
                    .map(|_| vec![0; usize::from(n)])
                    .collect()
            }),
            free_head: if n == 0 { u16::MAX } else { 0 },
            x_head: u16::MAX,
            y_head: u16::MAX,
            high_water: 0,
        })
    }
    /// `LocalMapShipList::add_collision_object` (0x00619450), con capacidad
    /// preexistente. Los campos y las listas de extremos siguen el orden x86.
    pub fn add(&mut self, kind: u8, owner: u8, x0: u16, y0: u16, x1: u16, y1: u16) -> Result<u16, &'static str> {
        let id = self.free_head;
        let i = usize::from(id);
        if i >= self.kinds.len() {
            return Err("capacidad espacial agotada");
        }
        self.free_head = self.free_next[i];
        self.kinds[i] = kind;
        self.owners[i] = u16::from(owner);
        let lo = i * 2;
        let hi = lo + 1;
        self.x[lo] = x0;
        self.x[hi] = x1;
        self.y[lo] = y0;
        self.y[hi] = y1;
        for (next, prev, head) in [
            (&mut self.x_next, &mut self.x_previous, &mut self.x_head),
            (&mut self.y_next, &mut self.y_previous, &mut self.y_head),
        ] {
            next[hi] = *head;
            prev[hi] = lo as u16;
            if *head != u16::MAX {
                let h = usize::from(*head);
                if h >= prev.len() {
                    return Err("lista espacial inválida");
                }
                prev[h] = hi as u16
            }
            next[lo] = hi as u16;
            prev[lo] = u16::MAX;
            *head = lo as u16
        }
        self.high_water = self.high_water.max(id + 1);
        Ok(id)
    }
}
/// Conecta selección, altas espaciales y creación de los dos bandos.
/// Cada alta y los dos bandos se confirman juntos; si falta capacidad o un
/// miembro es inválido, se conserva el gestor de colisiones anterior.
pub fn build_commercial_sides_with_collisions(
    ships: &[u8],
    convoys: &[u8],
    captains: &[u8],
    captain_count: u16,
    convoy_ids: [u16; 2],
    participants: [u16; 2],
    battle_address: u32,
    pool: &mut BattleCollisionPool,
) -> Result<[LocalBattleSide; 2], &'static str> {
    let selected = select_commercial_battle_ships(ships, convoys, convoy_ids, participants)?;
    let n = selected[0].len() + selected[1].len();
    if n > usize::from(u16::MAX) {
        return Err("demasiados barcos locales");
    }
    let mut staged = pool.clone();
    let mut spatial = Vec::with_capacity(n);
    for i in 0..n {
        let a = staged.add(0, i as u8, 0, 0, 0, 0)?;
        let b = staged.add(6, i as u8, 0, 0, 0, 0)?;
        spatial.push((a, b))
    }
    let sides = build_commercial_battle_sides(
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
impl BattleCollisionPool {
    /// Reproduce la reserva incremental de 32 objetos para las tablas que maneja
    /// este tipo. Las matrices auxiliares se amplían preservando sus entradas anteriores.
    pub fn grow(&mut self) -> Result<(), &'static str> {
        let old = self.kinds.len();
        if self.free_head != u16::MAX {
            return Err("aún quedan ranuras libres");
        }
        if old % 32 != 0 || old * 2 >= 0xffbf || old + 32 > 0x7fff {
            return Err("capacidad espacial máxima o incompatible");
        }
        let n = old + 32;
        self.kinds.resize(n, 0);
        self.owners.resize(n, 0);
        self.free_next
            .extend((old..n).map(|i| if i + 1 < n { (i + 1) as u16 } else { u16::MAX }));
        for a in [
            &mut self.x,
            &mut self.x_next,
            &mut self.x_previous,
            &mut self.y,
            &mut self.y_next,
            &mut self.y_previous,
        ] {
            a.resize(n * 2, 0)
        }
        for matrix in &mut self.matrices {
            for page in matrix.iter_mut() {
                page.resize(n, 0)
            }
            matrix.push(vec![0; n])
        }
        self.free_head = old as u16;
        Ok(())
    }
    pub fn add_growing(
        &mut self,
        kind: u8,
        owner: u8,
        x0: u16,
        y0: u16,
        x1: u16,
        y1: u16,
    ) -> Result<u16, &'static str> {
        if self.free_head == u16::MAX {
            self.grow()?
        }
        self.add(kind, owner, x0, y0, x1, y1)
    }
}
/// Variante de la creación comercial que amplía las tablas espaciales en
/// bloques cuando se agotan las ranuras.
pub fn build_commercial_sides_growing(
    ships: &[u8],
    convoys: &[u8],
    captains: &[u8],
    captain_count: u16,
    convoy_ids: [u16; 2],
    participants: [u16; 2],
    battle_address: u32,
    pool: &mut BattleCollisionPool,
) -> Result<[LocalBattleSide; 2], &'static str> {
    let selected = select_commercial_battle_ships(ships, convoys, convoy_ids, participants)?;
    let n = selected[0].len() + selected[1].len();
    if n > usize::from(u16::MAX) {
        return Err("demasiados barcos locales");
    }
    let mut staged = pool.clone();
    let mut spatial = Vec::with_capacity(n);
    for i in 0..n {
        spatial.push((
            staged.add_growing(0, i as u8, 0, 0, 0, 0)?,
            staged.add_growing(6, i as u8, 0, 0, 0, 0)?,
        ))
    }
    let sides = build_commercial_battle_sides(
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
impl BattleCollisionPool {
    /// Baja `LocalMapShipList::remove_collision_object` (0x00619560).
    /// Las dos matrices de contactos pierden la fila y columna del objeto; la
    /// tercera matriz se conserva, tal como hace el original.
    pub fn remove(&mut self, id: u16) -> Result<(), &'static str> {
        let i = usize::from(id);
        let n = self.kinds.len();
        if i >= n {
            return Err("objeto espacial inexistente");
        }
        let mut free = self.free_head;
        let mut seen = vec![false; n];
        while free != u16::MAX {
            let f = usize::from(free);
            if f >= n || seen[f] {
                return Err("lista libre inválida");
            }
            if free == id {
                return Err("objeto espacial ya libre");
            }
            seen[f] = true;
            free = self.free_next[f]
        }
        let mut staged = self.clone();
        staged.free_next[i] = staged.free_head;
        staged.free_head = id;
        for (next, prev, head) in [
            (&mut staged.x_next, &mut staged.x_previous, &mut staged.x_head),
            (&mut staged.y_next, &mut staged.y_previous, &mut staged.y_head),
        ] {
            for endpoint in [i * 2 + 1, i * 2] {
                if endpoint >= next.len() || endpoint >= prev.len() {
                    return Err("tabla espacial truncada");
                }
                let next_id = next[endpoint];
                let prev_id = prev[endpoint];
                if usize::from(*head) == endpoint {
                    *head = next_id
                } else {
                    let p = usize::from(prev_id);
                    if p >= next.len() {
                        return Err("enlace espacial anterior inválido");
                    }
                    next[p] = next_id
                }
                if next_id != u16::MAX {
                    let q = usize::from(next_id);
                    if q >= prev.len() {
                        return Err("enlace espacial siguiente inválido");
                    }
                    prev[q] = prev_id
                }
            }
        }
        let page = i / 32;
        let mask = !(1u32 << (i % 32));
        for matrix in staged.matrices.iter_mut().take(2) {
            if page >= matrix.len() || matrix.iter().any(|row| row.len() != n) {
                return Err("matriz espacial truncada");
            }
            for row in matrix.iter_mut() {
                row[i] = 0
            }
            for value in &mut matrix[page] {
                *value &= mask
            }
        }
        if id + 1 == staged.high_water {
            staged.high_water -= 1
        }
        *self = staged;
        Ok(())
    }
}
impl BattleCollisionPool {
    /// Actualiza los límites 0x006197A0. La ordenación y los contactos se
    /// recalculan en pasos separados del gestor original.
    pub fn set_bounds(&mut self, id: u16, x0: u16, y0: u16, x1: u16, y1: u16) -> Result<(), &'static str> {
        let i = usize::from(id) * 2;
        if i + 1 >= self.x.len() || i + 1 >= self.y.len() {
            return Err("objeto espacial inexistente");
        }
        self.x[i] = x0;
        self.x[i + 1] = x1;
        self.y[i] = y0;
        self.y[i + 1] = y1;
        Ok(())
    }
}
/// Ordena por inserciones adyacentes el eje original y actualiza la matriz
/// simétrica al cruzarse extremos opuestos. La comparación es i16 y estricta.
fn sweep_collision_axis(
    bounds: &[u16],
    next: &mut [u16],
    previous: &mut [u16],
    head: &mut u16,
    overlap: &mut [Vec<u32>],
) -> Result<(), &'static str> {
    let len = bounds.len();
    if next.len() != len || previous.len() != len {
        return Err("tablas espaciales incompatibles");
    }
    if *head == u16::MAX {
        return Ok(());
    }
    if usize::from(*head) >= len {
        return Err("cabeza espacial inválida");
    }
    loop {
        let mut changed = false;
        let mut prior = u16::MAX;
        let mut left = *head;
        let mut right = next[usize::from(left)];
        let mut steps = 0usize;
        while right != u16::MAX {
            steps += 1;
            if steps > len || usize::from(right) >= len {
                return Err("lista espacial cíclica o inválida");
            }
            let l = usize::from(left);
            let r = usize::from(right);
            let after = next[r];
            if (bounds[r] as i16) < (bounds[l] as i16) {
                changed = true;
                if prior == u16::MAX {
                    *head = right
                } else {
                    next[usize::from(prior)] = right
                }
                next[l] = after;
                previous[l] = right;
                next[r] = left;
                previous[r] = prior;
                if after != u16::MAX {
                    let a = usize::from(after);
                    if a >= len {
                        return Err("siguiente espacial inválido");
                    }
                    previous[a] = left
                }
                if (r & 1) != (l & 1) {
                    let obj_r = r / 2;
                    let obj_l = l / 2;
                    let set = r & 1 == 0;
                    for (row, col) in [(obj_l, obj_r), (obj_r, obj_l)] {
                        let page = col / 32;
                        let bit = 1u32 << (col % 32);
                        let cell = overlap
                            .get_mut(page)
                            .and_then(|v| v.get_mut(row))
                            .ok_or("matriz espacial truncada")?;
                        if set {
                            *cell |= bit
                        } else {
                            *cell &= !bit
                        }
                    }
                }
                prior = right;
                right = after;
            } else {
                prior = left;
                left = right;
                right = after
            }
        }
        if !changed {
            break;
        }
    }
    Ok(())
}
impl BattleCollisionPool {
    /// Barrido y enumeración de pares 0x006197F0 con consumidor inerte. Devuelve
    /// los pares dirigidos en el orden x86. El callback original puede modificar
    /// el estado durante la enumeración: ese caso aún no se admite aquí.
    pub fn update_pairs_inert(&mut self) -> Result<Vec<(u16, u16)>, &'static str> {
        let mut staged = self.clone();
        sweep_collision_axis(
            &staged.x,
            &mut staged.x_next,
            &mut staged.x_previous,
            &mut staged.x_head,
            &mut staged.matrices[0],
        )?;
        sweep_collision_axis(
            &staged.y,
            &mut staged.y_next,
            &mut staged.y_previous,
            &mut staged.y_head,
            &mut staged.matrices[1],
        )?;
        let n = usize::from(staged.high_water);
        let pages = (n + 31) / 32;
        if n > staged.kinds.len()
            || staged
                .matrices
                .iter()
                .any(|m| m.len() < pages || m.iter().take(pages).any(|p| p.len() < n))
        {
            return Err("matriz de pares truncada");
        }
        let mut pairs = Vec::new();
        for row in 0..n {
            for page in 0..pages {
                let v = staged.matrices[0][page][row] & staged.matrices[1][page][row];
                staged.matrices[2][page][row] = v;
                for bit in 0..32 {
                    if v & (1u32 << bit) != 0 {
                        pairs.push((row as u16, (page * 32 + bit) as u16))
                    }
                }
            }
        }
        *self = staged;
        Ok(pairs)
    }
}
/// Resolutor requerido por la rama barco/barco de 0x00608850. El anfitrión
/// debe ejecutarlo sobre la batalla: este despacho no aplica daño ni abordaje.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleShipPairEffect {
    Collision(u16, u16),
    Boarding(u16, u16),
}
pub fn dispatch_battle_ship_pair(
    kinds: &[u8],
    references: &[u16],
    local_ships: &[Vec<u8>],
    participant_sides: &[u8],
    first: u32,
    second: u32,
) -> Result<Option<BattleShipPairEffect>, &'static str> {
    let a = (first & 0xffff) as usize;
    let b = (second & 0xffff) as usize;
    let kind = *kinds.get(a).ok_or("índice espacial inválido")?;
    if kind != 0 {
        return Ok(None);
    }
    let ai = usize::from(*references.get(a).ok_or("referencia espacial inválida")?);
    let ship = local_ships
        .get(ai)
        .filter(|s| s.len() > 0x13d)
        .ok_or("barco local inválido")?;
    let flags = ship[0x12a];
    if flags & 2 != 0 {
        return Ok(None);
    }
    let target_kind = *kinds.get(b).ok_or("índice espacial inválido")?;
    if target_kind > 6 {
        return Ok(None);
    }
    if target_kind != 0 {
        return Err("tipo espacial aún no conectado");
    }
    let bi = usize::from(*references.get(b).ok_or("referencia espacial inválida")?);
    let target = local_ships
        .get(bi)
        .filter(|s| s.len() > 0x13d)
        .ok_or("barco local inválido")?;
    let aside = *participant_sides
        .get(usize::from(ship[0x13d]))
        .ok_or("participante inválido")?;
    let bside = *participant_sides
        .get(usize::from(target[0x13d]))
        .ok_or("participante inválido")?;
    let (x, y) = (ai as u16, bi as u16);
    if aside == bside {
        return Ok(Some(BattleShipPairEffect::Collision(x, y)));
    }
    if ai >= bi {
        return Ok(None);
    }
    if (flags | target[0x12a]) & 8 != 0 {
        Ok(Some(BattleShipPairEffect::Collision(x, y)))
    } else {
        Ok(Some(BattleShipPairEffect::Boarding(x, y)))
    }
}
impl BattleCollisionPool {
    /// Encadena el barrido y el despacho barco/barco con resolutores inertes.
    /// Si aparece otro tipo o una referencia inválida, conserva el pool intacto.
    /// Los efectos se devuelven sin aplicarlos; para ejecutar el combate es
    /// necesario admitir mutaciones durante cada callback del original.
    pub fn collect_ship_pair_effects_inert(
        &mut self,
        local_ships: &[Vec<u8>],
        participant_sides: &[u8],
    ) -> Result<Vec<BattleShipPairEffect>, &'static str> {
        let mut staged = self.clone();
        let pairs = staged.update_pairs_inert()?;
        let mut effects = Vec::new();
        for (a, b) in pairs {
            if let Some(effect) = dispatch_battle_ship_pair(
                &staged.kinds,
                &staged.owners,
                local_ships,
                participant_sides,
                u32::from(a),
                u32::from(b),
            )? {
                effects.push(effect)
            }
        }
        *self = staged;
        Ok(effects)
    }
}
/// Resultado de la rama de proximidad de `sea_battle_resolve_ship_collision`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NearShipCollision {
    Ignored,
    Separated,
    NeedsHull,
}
/// Aplica la separación para distancias enteras menores que diez. Cuando los
/// centros coinciden o están más lejos devuelve `NeedsHull` sin modificarlos;
/// la geometría de cascos y el reparto de impulso siguen pendientes.
pub fn resolve_near_ship_collision(
    local_ships: &mut [Vec<u8>],
    first: u16,
    second: u16,
) -> Result<NearShipCollision, &'static str> {
    use super::{r16, w16};
    if first >= second {
        return Ok(NearShipCollision::Ignored);
    }
    let a = usize::from(first);
    let b = usize::from(second);
    if b >= local_ships.len() || local_ships[a].len() < 0x10 || local_ships[b].len() < 0x10 {
        return Err("barco local inválido");
    }
    let dx = i32::from(r16(&local_ships[a], 0xa) as i16) - i32::from(r16(&local_ships[b], 0xa) as i16);
    let dy = i32::from(r16(&local_ships[a], 0xe) as i16) - i32::from(r16(&local_ships[b], 0xe) as i16);
    let dist = dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy));
    if dist < 0 {
        return Err("distancia con desbordamiento");
    };
    if dist == 0 || dist >= 100 {
        return Ok(NearShipCollision::NeedsHull);
    }
    let d = (1..=9).rev().find(|&v| v * v <= dist).unwrap();
    let sx = (11 - d) * dx / d;
    let sy = (11 - d) * dy / d;
    for (index, sign) in [(a, -1i32), (b, 1i32)] {
        let x = r16(&local_ships[index], 0xa).wrapping_add((sign * sx) as u16);
        let y = r16(&local_ships[index], 0xe).wrapping_add((sign * sy) as u16);
        w16(&mut local_ships[index], 0xa, x);
        w16(&mut local_ships[index], 0xe, y)
    }
    Ok(NearShipCollision::Separated)
}
/// Efectos cuyo tratamiento se interceptó en el contraste integrado.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PendingShipPairEffect {
    Hull(u16, u16),
    Boarding(u16, u16),
}
impl BattleCollisionPool {
    /// Barrido y separación cercana, suponiendo inertes las consultas de casco y
    /// el abordaje. Para el combate real hay que resolverlos durante el callback.
    pub fn resolve_near_pairs_inert(
        &mut self,
        local_ships: &mut [Vec<u8>],
        participant_sides: &[u8],
    ) -> Result<Vec<PendingShipPairEffect>, &'static str> {
        let mut staged_pool = self.clone();
        let mut staged_ships = local_ships.to_vec();
        let effects = staged_pool.collect_ship_pair_effects_inert(&staged_ships, participant_sides)?;
        let mut pending = Vec::new();
        for effect in effects {
            match effect {
                BattleShipPairEffect::Collision(a, b) => {
                    if resolve_near_ship_collision(&mut staged_ships, a, b)? == NearShipCollision::NeedsHull {
                        pending.push(PendingShipPairEffect::Hull(a, b))
                    }
                }
                BattleShipPairEffect::Boarding(a, b) => pending.push(PendingShipPairEffect::Boarding(a, b)),
            }
        }
        local_ships.clone_from_slice(&staged_ships);
        *self = staged_pool;
        Ok(pending)
    }
}
/// Avanza el generador propio de la batalla (0x0060D259); la semilla se
/// conserva en `SeaBattle+0xADC` y no comparte estado con el RNG general.
pub fn advance_battle_rng(state: u32) -> u32 {
    0xa205b063u32
        .wrapping_sub(state.wrapping_mul(0x44bf19b3))
        .rotate_right(1)
}
/// Reproduce los filtros jerárquicos de bits del original. Cada rama vuelve a
/// leer la palabra cuando corresponde: un callback puede modificarla.
fn dispatch_collision_group<F>(
    pool: &mut BattleCollisionPool,
    row: usize,
    page: usize,
    start: usize,
    width: usize,
    callback: &mut F,
) -> Result<(), &'static str>
where
    F: FnMut(&mut BattleCollisionPool, u16, u16) -> Result<(), &'static str>,
{
    let mask = if width == 32 {
        u32::MAX
    } else {
        ((1u32 << width) - 1) << start
    };
    let value = *pool.matrices[2]
        .get(page)
        .and_then(|p| p.get(row))
        .ok_or("matriz de pares truncada")?;
    if value & mask == 0 {
        return Ok(());
    }
    if width == 1 {
        return callback(pool, row as u16, (page * 32 + start) as u16);
    }
    let half = width / 2;
    dispatch_collision_group(pool, row, page, start, half, callback)?;
    dispatch_collision_group(pool, row, page, start + half, half, callback)
}
impl BattleCollisionPool {
    /// Barrido 0x006197F0 con callback que puede modificar el pool durante cada
    /// pareja. La mutación del pool se confirma solo si toda la llamada tiene
    /// éxito; los efectos externos capturados por el callback son del llamante.
    pub fn update_pairs_with<F>(&mut self, mut callback: F) -> Result<(), &'static str>
    where
        F: FnMut(&mut BattleCollisionPool, u16, u16) -> Result<(), &'static str>,
    {
        let mut staged = self.clone();
        sweep_collision_axis(
            &staged.x,
            &mut staged.x_next,
            &mut staged.x_previous,
            &mut staged.x_head,
            &mut staged.matrices[0],
        )?;
        sweep_collision_axis(
            &staged.y,
            &mut staged.y_next,
            &mut staged.y_previous,
            &mut staged.y_head,
            &mut staged.matrices[1],
        )?;
        let pages = (usize::from(staged.high_water) + 31) / 32;
        let mut row = 0usize;
        while row < usize::from(staged.high_water) {
            if row >= staged.kinds.len() {
                return Err("marca espacial inválida");
            }
            for page in 0..pages {
                let x = *staged.matrices[0]
                    .get(page)
                    .and_then(|p| p.get(row))
                    .ok_or("matriz espacial truncada")?;
                let y = *staged.matrices[1]
                    .get(page)
                    .and_then(|p| p.get(row))
                    .ok_or("matriz espacial truncada")?;
                let cell = staged.matrices[2]
                    .get_mut(page)
                    .and_then(|p| p.get_mut(row))
                    .ok_or("matriz de pares truncada")?;
                *cell = x & y;
                dispatch_collision_group(&mut staged, row, page, 0, 32, &mut callback)?
            }
            row += 1
        }
        *self = staged;
        Ok(())
    }
}
impl BattleCollisionPool {
    /// Despacha encuentros barco/barco durante el barrido original. El callback
    /// puede mutar tablas y barcos; ambos se confirman juntos al completarse.
    /// Los tipos espaciales aún no implementados causan error y rollback.
    pub fn update_ship_pairs_with<F>(
        &mut self,
        local_ships: &mut [Vec<u8>],
        participant_sides: &[u8],
        mut effect: F,
    ) -> Result<(), &'static str>
    where
        F: FnMut(&mut BattleCollisionPool, &mut [Vec<u8>], BattleShipPairEffect) -> Result<(), &'static str>,
    {
        let mut ships = local_ships.to_vec();
        self.update_pairs_with(|pool, a, b| {
            if let Some(item) = dispatch_battle_ship_pair(
                &pool.kinds,
                &pool.owners,
                &ships,
                participant_sides,
                u32::from(a),
                u32::from(b),
            )? {
                effect(pool, &mut ships, item)?
            }
            Ok(())
        })?;
        local_ships.clone_from_slice(&ships);
        Ok(())
    }
}
/// Dependencias del inicio de abordaje 0x00609BA0. El choque puede mover
/// barcos; las decisiones de preferencia y captura pertenecen al anfitrión.
pub trait BoardingHost {
    fn resolve_collision(&mut self, ships: &mut [Vec<u8>], first: u16, second: u16) -> Result<bool, &'static str>;
    fn prefer_boarding(&mut self, ships: &[Vec<u8>], attacker: u16, target: u16) -> Result<bool, &'static str>;
    fn should_capture(&mut self, ships: &[Vec<u8>], attacker: u16, target: u16) -> Result<bool, &'static str>;
}
/// Ejecuta los filtros y las mutaciones de inicio de abordaje. Los barcos se
/// confirman juntos; la política y los efectos externos siguen en el anfitrión.
pub fn try_begin_boarding(
    ships: &mut [Vec<u8>],
    world_ships: &[u8],
    first: u32,
    second: u32,
    direct: bool,
    finished: bool,
    host: &mut dyn BoardingHost,
) -> Result<bool, &'static str> {
    use super::remove::SHIP_SIZE;
    use super::{r32, w16};
    let a = (first & 0xffff) as usize;
    let b = (second & 0xffff) as usize;
    if a >= ships.len() || b >= ships.len() || a == b || ships[a].len() < 0x164 || ships[b].len() < 0x164 {
        return Err("barcos locales inválidos");
    }
    let mut staged = ships.to_vec();
    let collision = host.resolve_collision(&mut staged, a as u16, b as u16)?;
    if !collision && ((staged[a][0x12a] | staged[b][0x12a]) & 0x20 == 0 || !direct) || finished {
        ships.clone_from_slice(&staged);
        return Ok(false);
    }
    fn wants(
        host: &mut dyn BoardingHost,
        local: &[Vec<u8>],
        world: &[u8],
        attacker: usize,
        target: usize,
    ) -> Result<bool, &'static str> {
        if !host.prefer_boarding(local, attacker as u16, target as u16)? {
            return Ok(false);
        }
        if host.should_capture(local, attacker as u16, target as u16)? {
            return Ok(true);
        }
        let id = r32(&local[target], 4) as u32 as usize;
        let off = id.checked_mul(SHIP_SIZE).ok_or("índice de barco mundial inválido")?;
        let image = world.get(off..off + SHIP_SIZE).ok_or("barco mundial inválido")?;
        Ok(r32(image, 0x118) != 0 || local[target][0x12a] & 0x40 == 0)
    }
    let first_wants = wants(host, &staged, world_ships, a, b)?;
    let second_wants = wants(host, &staged, world_ships, b, a)?;
    if first_wants || second_wants {
        staged[a][0x12a] |= 8;
        staged[b][0x12a] |= 8;
        w16(&mut staged[a], 0x128, b as u16);
        w16(&mut staged[b], 0x128, a as u16);
        staged[a][0x14a] = 0;
        staged[b][0x14a] = 1
    }
    ships.clone_from_slice(&staged);
    Ok(first_wants || second_wants)
}
impl BattleCollisionPool {
    /// Encadena el barrido mutante con el choque o el inicio de abordaje. El
    /// anfitrión aporta los resolutores y debe gestionar sus efectos externos.
    pub fn update_ship_boarding_with(
        &mut self,
        ships: &mut [Vec<u8>],
        participant_sides: &[u8],
        world_ships: &[u8],
        direct: bool,
        finished: bool,
        host: &mut dyn BoardingHost,
    ) -> Result<(), &'static str> {
        self.update_ship_pairs_with(ships, participant_sides, |_, ships, effect| {
            match effect {
                BattleShipPairEffect::Collision(a, b) => {
                    host.resolve_collision(ships, a, b)?;
                }
                BattleShipPairEffect::Boarding(a, b) => {
                    try_begin_boarding(ships, world_ships, u32::from(a), u32::from(b), direct, finished, host)?;
                }
            }
            Ok(())
        })
    }
}
/// Política de captura usada por el contacto de abordaje (`0x006212FB`,
/// argumento de modo cero). `battle_query` representa la ranura virtual 10.
pub fn boarding_capture_policy<F>(
    attacker: &[u8],
    target: &[u8],
    world: &[u8],
    battle: &[u8],
    query: &mut F,
) -> Result<bool, &'static str>
where
    F: FnMut() -> Result<bool, &'static str>,
{
    boarding_capture_policy_with_mode(attacker, target, world, battle, false, query)
}
/// Política de captura 0x006212FB; `surrender` activa la rama adicional
/// utilizada al procesar una rendición durante el abordaje.
pub fn boarding_capture_policy_with_mode<F>(
    attacker: &[u8],
    target: &[u8],
    world: &[u8],
    battle: &[u8],
    surrender: bool,
    query: &mut F,
) -> Result<bool, &'static str>
where
    F: FnMut() -> Result<bool, &'static str>,
{
    use super::remove::SHIP_SIZE;
    use super::{r16, r32};
    if attacker.len() < 0x164 || target.len() < 0x164 || battle.len() < 0xacc {
        return Err("estado de abordaje truncado");
    }
    let a = usize::from(attacker[0x13d]);
    let b = usize::from(target[0x13d]);
    if 0x35a + 2 * a + 2 > battle.len()
        || 0x35a + 2 * b + 2 > battle.len()
        || 0x559 + b >= battle.len()
        || 0x5d + a >= battle.len()
    {
        return Err("participante inválido");
    }
    if surrender && (battle[0x559 + b] == 0 || query()? || r16(battle, 0x35a + 2 * b) == 0xff) {
        return Ok(true);
    }
    if battle[0x5d + a] == 0 {
        return Ok(attacker[0x12a] & 0x80 != 0);
    }
    let ai = (r32(attacker, 4) as u32 as usize)
        .checked_mul(SHIP_SIZE)
        .ok_or("índice inválido")?;
    let bi = (r32(target, 4) as u32 as usize)
        .checked_mul(SHIP_SIZE)
        .ok_or("índice inválido")?;
    let sa = world.get(ai..ai + SHIP_SIZE).ok_or("barco mundial ausente")?;
    let sb = world.get(bi..bi + SHIP_SIZE).ok_or("barco mundial ausente")?;
    let crew = r16(sa, 0x40);
    let blades = r16(sa, 0x154);
    let strength = if crew < blades {
        crew.wrapping_mul(2)
    } else {
        crew.wrapping_add(blades)
    };
    if u32::from(strength) < u32::from(r16(sb, 0x40)) * 2 {
        return Ok(false);
    }
    let target_side = battle[0x559 + b];
    if target_side != 0 && !query()? && r16(battle, 0x35a + 2 * b) != 0xff {
        if target[0x12a] & 0x40 == 0 && r16(battle, 0x35a + 2 * a) == 0xff {
            let mode = *battle
                .get(0xaca + usize::from(target_side))
                .ok_or("política inválida")?;
            if mode == 2 || mode == 3 {
                let n = u32::from(r16(target, 0x138))
                    .wrapping_add(u32::from(r16(attacker, 0x138)))
                    .wrapping_add(r32(target, 4) as u32)
                    .wrapping_add(r32(attacker, 4) as u32);
                return Ok(n % 11 < if mode == 2 { 3 } else { 6 });
            }
            if mode > 3 {
                return Ok(true);
            }
        }
        return Ok(false);
    }
    Ok(true)
}
/// Preferencia de abordaje 0x0061F02F para el modo de contacto (argumento 0).
pub fn boarding_preference_policy<F>(
    attacker: &[u8],
    target: &[u8],
    world: &[u8],
    battle: &[u8],
    query: &mut F,
) -> Result<bool, &'static str>
where
    F: FnMut() -> Result<bool, &'static str>,
{
    use super::remove::SHIP_SIZE;
    use super::{r16, r32};
    if attacker.len() < 0x164 || target.len() < 0x164 || battle.len() < 0xacc {
        return Err("estado de abordaje truncado");
    }
    if attacker[0x12a] & 0x20 != 0 {
        return Ok(false);
    }
    let a = usize::from(attacker[0x13d]);
    if 0x5d + a >= battle.len() {
        return Err("participante inválido");
    }
    if battle[0x5d + a] == 0 {
        return Ok(
            (r16(target, 0x138) == r16(attacker, 0x128) || r16(attacker, 0x128) == u16::MAX) && attacker[0x13e] != 0,
        );
    }
    let capture = boarding_capture_policy(attacker, target, world, battle, query)?;
    let ai = (r32(attacker, 4) as u32 as usize)
        .checked_mul(SHIP_SIZE)
        .ok_or("índice inválido")?;
    let bi = (r32(target, 4) as u32 as usize)
        .checked_mul(SHIP_SIZE)
        .ok_or("índice inválido")?;
    let sa = world.get(ai..ai + SHIP_SIZE).ok_or("barco mundial ausente")?;
    let sb = world.get(bi..bi + SHIP_SIZE).ok_or("barco mundial ausente")?;
    if !capture {
        if sb[0x136] & 0x40 != 0 {
            return Ok(false);
        }
        if r32(sa, 0x10).wrapping_sub(r32(sa, 0x11c)).wrapping_sub(r32(sa, 0x118)) < 2000 {
            return Ok(false);
        }
        if target[0x12a] & 0x20 != 0 {
            return Ok(true);
        }
    }
    let crew = r16(sa, 0x40);
    let blades = r16(sa, 0x154);
    let strength = if crew < blades {
        crew.wrapping_mul(2)
    } else {
        crew.wrapping_add(blades)
    };
    Ok(u32::from(r16(sb, 0x40)) * 2 < u32::from(strength))
}
/// Adaptador de políticas nativas de contacto; el choque y la consulta virtual
/// siguen siendo funciones proporcionadas por el anfitrión de batalla.
pub struct ContactBoardingHost<'a, C, Q> {
    pub world: &'a [u8],
    pub battle: &'a [u8],
    pub collision: C,
    pub battle_query: Q,
}
impl<C, Q> BoardingHost for ContactBoardingHost<'_, C, Q>
where
    C: FnMut(&mut [Vec<u8>], u16, u16) -> Result<bool, &'static str>,
    Q: FnMut() -> Result<bool, &'static str>,
{
    fn resolve_collision(&mut self, ships: &mut [Vec<u8>], a: u16, b: u16) -> Result<bool, &'static str> {
        (self.collision)(ships, a, b)
    }
    fn prefer_boarding(&mut self, ships: &[Vec<u8>], a: u16, b: u16) -> Result<bool, &'static str> {
        let attacker = ships.get(usize::from(a)).ok_or("barco local inválido")?;
        let target = ships.get(usize::from(b)).ok_or("barco local inválido")?;
        boarding_preference_policy(attacker, target, self.world, self.battle, &mut self.battle_query)
    }
    fn should_capture(&mut self, ships: &[Vec<u8>], a: u16, b: u16) -> Result<bool, &'static str> {
        let attacker = ships.get(usize::from(a)).ok_or("barco local inválido")?;
        let target = ships.get(usize::from(b)).ok_or("barco local inválido")?;
        boarding_capture_policy(attacker, target, self.world, self.battle, &mut self.battle_query)
    }
}
/// Resultado parcial de un tick cuerpo a cuerpo: la resolución final de
/// captura o saqueo debe ejecutarse en el anfitrión cuando aparece una pareja.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoardingFinish {
    pub winner: u16,
    pub loser: u16,
}
/// Compatibilidad con la rama sin propietario comercial ni capitán.
pub fn boarding_melee_unowned_no_captain(
    ships: &mut [Vec<u8>],
    world: &mut [u8],
    battle: &mut [u8],
    attacker: u16,
    captain_count: u16,
    visible: bool,
) -> Result<Option<BoardingFinish>, &'static str> {
    boarding_melee_without_surrender(ships, world, battle, attacker, &[], captain_count, &[], 1, visible)
}
/// Tick 0x0061F1BF con tripulación, capitanes y dificultad. Rechaza la
/// rendición y el sonido de la vista activa, que requieren efectos externos.
pub fn boarding_melee_without_surrender(
    ships: &mut [Vec<u8>],
    world: &mut [u8],
    battle: &mut [u8],
    attacker: u16,
    captains: &[u8],
    captain_count: u16,
    merchants: &[u8],
    difficulty: u8,
    visible: bool,
) -> Result<Option<BoardingFinish>, &'static str> {
    boarding_melee_core(
        ships,
        world,
        battle,
        attacker,
        captains,
        captain_count,
        merchants,
        difficulty,
        visible,
        false,
    )
}
fn boarding_melee_core(
    ships: &mut [Vec<u8>],
    world: &mut [u8],
    battle: &mut [u8],
    attacker: u16,
    captains: &[u8],
    captain_count: u16,
    merchants: &[u8],
    difficulty: u8,
    visible: bool,
    allow_surrender: bool,
) -> Result<Option<BoardingFinish>, &'static str> {
    use super::remove::SHIP_SIZE;
    use super::{r16, r32, w16};
    let a = usize::from(attacker);
    if a >= ships.len() || ships[a].len() < 0x164 || battle.len() < 0xae0 {
        return Err("batalla truncada");
    }
    if visible && ships[a][0x14a] == 0 {
        return Err("sonido de abordaje no conectado");
    }
    let b = usize::from(r16(&ships[a], 0x128));
    if ships[a][0x26] >= 3 {
        return Ok(None);
    }
    if b >= ships.len() || b == a || ships[b].len() < 0x164 {
        return Err("objetivo inválido");
    }
    if ships[b][0x26] >= 3 {
        return Ok(None);
    }
    if !allow_surrender && (ships[a][0x12a] | ships[b][0x12a]) & 0x20 != 0 {
        return Err("rendición no implementada");
    }
    let ai = (r32(&ships[a], 4) as u32 as usize)
        .checked_mul(SHIP_SIZE)
        .ok_or("índice inválido")?;
    let bi = (r32(&ships[b], 4) as u32 as usize)
        .checked_mul(SHIP_SIZE)
        .ok_or("índice inválido")?;
    if ai == bi || ai + SHIP_SIZE > world.len() || bi + SHIP_SIZE > world.len() {
        return Err("barco mundial inválido");
    }
    for off in [ai, bi] {
        let cap = r16(world, off + 0x42);
        if cap < captain_count && usize::from(cap) * 16 + 16 > captains.len() {
            return Err("capitán ausente");
        }
        let owner = world[off];
        if owner != 0xff && usize::from(owner) * 0x650 + 0x650 > merchants.len() {
            return Err("comerciante ausente");
        }
        if r16(world, off + 0x40) == 0 && cap >= captain_count {
            return Err("tripulación vacía no admitida");
        }
    }
    let mut local = ships.to_vec();
    let mut w = world.to_vec();
    let mut btl = battle.to_vec();
    let sailors_a = r16(&w, ai + 0x40);
    let sailors_b = r16(&w, bi + 0x40);
    let crew_a = u32::from(sailors_a) + u32::from(r16(&w, ai + 0x42) < captain_count);
    let crew_b = u32::from(sailors_b) + u32::from(r16(&w, bi + 0x42) < captain_count);
    let strength = |off: usize, crew: u32| {
        let blades = u32::from(r16(&w, off + 0x154));
        let weapons = crew + crew.min(blades);
        let mut damage = (w[off + 0x3f] as i8).clamp(0, 4);
        let owner = w[off];
        if owner != 0xff && r16(merchants, usize::from(owner) * 0x650 + 8) == 0 {
            if difficulty == 0 {
                damage = damage.wrapping_add(2)
            } else if difficulty == 2 {
                damage = damage.wrapping_sub(2)
            }
        }
        let mut value = ((i32::from(damage) + 18).wrapping_mul(weapons as i32).wrapping_add(10) as u32) / 20;
        let cap = r16(&w, off + 0x42);
        if cap < captain_count {
            let skill = u32::from(captains[usize::from(cap) * 16 + 0xb]);
            value = (skill * 6 / 17 + 100).wrapping_mul(value) / 100
        }
        value.max(1)
    };
    let sa = strength(ai, crew_a);
    let sb = strength(bi, crew_b);
    let direct = btl[0xac8] != 0;
    fn next(battle: &mut [u8]) -> u32 {
        use super::{r32, w32};
        let v = advance_battle_rng(r32(battle, 0xadc) as u32);
        w32(battle, 0xadc, v as i32);
        v
    }
    fn losses(battle: &mut [u8], own: u32, other: u32, other_crew: u32, fraction: u16, direct: bool) -> (u16, u32) {
        let rnd = next(battle) % 26 + 92;
        let ratio = own.wrapping_mul(100) / other;
        let term = ratio.wrapping_add(other_crew.wrapping_add(own).wrapping_mul(3));
        let mut v = rnd.wrapping_mul(term).wrapping_mul(40) / 1000;
        if direct {
            v = v.wrapping_shl(4)
        }
        v = v.wrapping_add(u32::from(fraction));
        (v as u16, v >> 16)
    }
    let (fb, mut lb) = losses(&mut btl, sa, sb, crew_b, r16(&local[b], 0x13a), direct);
    w16(&mut local[b], 0x13a, fb);
    let (fa, mut la) = losses(&mut btl, sb, sa, crew_a, r16(&local[a], 0x13a), direct);
    w16(&mut local[a], 0x13a, fa);
    lb = lb.min(crew_b);
    la = la.min(crew_a);
    for _ in 0..lb {
        let blades = u32::from(r16(&w, bi + 0x154)).min(crew_b);
        if next(&mut btl) % (crew_b * 2) < blades {
            let v = r16(&w, bi + 0x154).wrapping_sub(1);
            w16(&mut w, bi + 0x154, v)
        }
    }
    if lb != 0 && lb == crew_b {
        w16(&mut w, bi + 0x40, 1);
        ships.clone_from_slice(&local);
        world.copy_from_slice(&w);
        battle.copy_from_slice(&btl);
        return Ok(Some(BoardingFinish {
            winner: r16(&local[a], 0x138),
            loser: b as u16,
        }));
    }
    for _ in 0..la {
        let blades = u32::from(r16(&w, ai + 0x154)).min(crew_a);
        if next(&mut btl) % (crew_a * 2) < blades {
            let v = r16(&w, ai + 0x154).wrapping_sub(1);
            w16(&mut w, ai + 0x154, v)
        }
    }
    if la != 0 && la == crew_a {
        w16(&mut w, ai + 0x40, 1);
        ships.clone_from_slice(&local);
        world.copy_from_slice(&w);
        battle.copy_from_slice(&btl);
        return Ok(Some(BoardingFinish {
            winner: b as u16,
            loser: r16(&local[a], 0x138),
        }));
    }
    if la != 0 {
        let f = r16(&w, ai + 0x3c) | 2;
        w16(&mut w, ai + 0x3c, f)
    }
    if lb != 0 {
        let f = r16(&w, bi + 0x3c) | 2;
        w16(&mut w, bi + 0x3c, f)
    }
    w16(&mut w, ai + 0x40, sailors_a.wrapping_sub(la as u16));
    w16(&mut w, bi + 0x40, sailors_b.wrapping_sub(lb as u16));
    let pa = usize::from(local[a][0x13d]);
    let pb = usize::from(local[b][0x13d]);
    if 0x35a + 2 * pa + 2 > btl.len()
        || 0x35a + 2 * pb + 2 > btl.len()
        || 0x5d + pa >= btl.len()
        || 0x5d + pb >= btl.len()
    {
        return Err("participante inválido");
    }
    if btl[0x5d + pa] != 0 && crew_b * 2 > sa * 3 && r16(&btl, 0x35a + 2 * pa) != 0xff {
        local[a][0x12a] |= 0x20
    } else if btl[0x5d + pb] != 0 && crew_a * 2 > sb * 3 && r16(&btl, 0x35a + 2 * pb) != 0xff {
        local[b][0x12a] |= 0x20
    }
    ships.clone_from_slice(&local);
    world.copy_from_slice(&w);
    battle.copy_from_slice(&btl);
    Ok(None)
}
/// Procesa las rendiciones y después el cuerpo a cuerpo. La consulta virtual
/// y la resolución final de captura/saqueo se entregan al anfitrión; no se
/// reproduce el sonido cuando la batalla está visible.
pub fn boarding_melee_with_surrender<F>(
    ships: &mut [Vec<u8>],
    world: &mut [u8],
    battle: &mut [u8],
    attacker: u16,
    captains: &[u8],
    captain_count: u16,
    merchants: &[u8],
    difficulty: u8,
    visible: bool,
    query: &mut F,
) -> Result<Option<BoardingFinish>, &'static str>
where
    F: FnMut() -> Result<bool, &'static str>,
{
    use super::r16;
    let a = usize::from(attacker);
    if a >= ships.len() || ships[a].len() < 0x164 || battle.len() < 0xae0 {
        return Err("batalla truncada");
    }
    if visible && ships[a][0x14a] == 0 {
        return Err("sonido de abordaje no conectado");
    }
    if ships[a][0x26] >= 3 {
        return Ok(None);
    }
    let b = usize::from(r16(&ships[a], 0x128));
    if b >= ships.len() || b == a || ships[b].len() < 0x164 {
        return Err("objetivo inválido");
    }
    if ships[b][0x26] >= 3 {
        return Ok(None);
    }
    let mut local = ships.to_vec();
    let mut w = world.to_vec();
    let mut bt = battle.to_vec();
    let participant = usize::from(local[a][0x13d]);
    let side = *bt.get(0x559 + participant).ok_or("participante inválido")?;
    if side > 1 {
        return Err("bando inválido");
    }
    if local[a][0x12a] & 0x20 != 0 {
        if !boarding_capture_policy_with_mode(&local[b], &local[a], &w, &bt, true, query)? {
            return Ok(Some(BoardingFinish {
                winner: b as u16,
                loser: r16(&local[a], 0x138),
            }));
        }
        bt[0xacb - usize::from(side)] = 0xff
    }
    if local[b][0x12a] & 0x20 != 0 {
        if !boarding_capture_policy_with_mode(&local[a], &local[b], &w, &bt, true, query)? {
            ships.clone_from_slice(&local);
            world.copy_from_slice(&w);
            battle.copy_from_slice(&bt);
            return Ok(Some(BoardingFinish {
                winner: r16(&local[a], 0x138),
                loser: b as u16,
            }));
        }
        bt[0xaca + usize::from(side)] = 0xff
    }
    let finish = boarding_melee_core(
        &mut local,
        &mut w,
        &mut bt,
        attacker,
        captains,
        captain_count,
        merchants,
        difficulty,
        visible,
        true,
    )?;
    ships.clone_from_slice(&local);
    world.copy_from_slice(&w);
    battle.copy_from_slice(&bt);
    Ok(finish)
}
/// Retira dos barcos comerciales sueltos del mar y les crea sendos convoyes
/// temporales con ranuras libres ya reservadas (tramo de 0x0050B250).
/// No reserva la batalla ni llama todavía a `SeaBattle::start`.
pub fn prepare_single_ship_battle(
    manager: &mut [u8],
    ships: &mut [u8],
    convoys: &mut [u8],
    merchant_count: u16,
    attacker: u16,
    target: u16,
) -> Result<BattleConvoyPreparation, &'static str> {
    use super::order::XOrder;
    use super::remove::{RemoveWorld, CONVOY_SIZE, SHIP_SIZE};
    use super::tick::TickWorld;
    use super::{r16, r32, w16, w32};
    if manager.len() < 0xf8 {
        return Err("gestor truncado");
    }
    let n = r16(manager, 0xf4);
    let nc = r16(manager, 0xf6);
    if attacker >= n
        || target >= n
        || attacker == target
        || ships.len() < usize::from(n) * SHIP_SIZE
        || convoys.len() < usize::from(nc) * CONVOY_SIZE
    {
        return Err("flota truncada");
    }
    for id in [attacker, target] {
        let p = usize::from(id) * SHIP_SIZE;
        if u16::from(ships[p]) >= merchant_count || r16(ships, p + 8) < nc {
            return Err("barco no comercial o ya en convoy");
        }
    }
    let mut m = manager.to_vec();
    let mut s = ships.to_vec();
    let mut c = convoys.to_vec();
    let sea = r16(&m, 0xea);
    for id in [attacker, target] {
        XOrder {
            ships: &mut s,
            count: n,
            sea_head: sea,
        }
        .remove(id)
    }
    let mut invalidations = Vec::new();
    for id in [attacker, target] {
        let sea_head = r16(&m, 0xea);
        XOrder {
            ships: &mut s,
            count: n,
            sea_head,
        }
        .remove(id);
        let mut cur = sea_head;
        let mut prev = None;
        let mut seen = vec![false; usize::from(n)];
        while cur < n && cur != id {
            let j = usize::from(cur);
            if seen[j] {
                return Err("lista marítima cíclica");
            }
            seen[j] = true;
            prev = Some(j * SHIP_SIZE + 6);
            cur = r16(&s, j * SHIP_SIZE + 6)
        }
        if cur != id {
            return Err("barco ausente de la lista marítima");
        }
        let p = usize::from(id) * SHIP_SIZE;
        let next = r16(&s, p + 6);
        if let Some(link) = prev {
            w16(&mut s, link, next)
        } else {
            w16(&mut m, 0xea, next)
        }
        w16(&mut s, p + 6, u16::MAX);
        let mut walk = r16(&m, 0xea);
        seen.fill(false);
        while walk < n {
            let j = usize::from(walk);
            if seen[j] {
                return Err("lista marítima cíclica");
            }
            seen[j] = true;
            let q = j * SHIP_SIZE;
            for offset in [0x50, 0x52] {
                if r16(&s, q + offset) == id {
                    w16(&mut s, q + offset, u16::MAX)
                }
            }
            walk = r16(&s, q + 6)
        }
        invalidations.push(id);
        let cv = r16(&m, 0xe4);
        if cv >= nc {
            return Err("ampliación de convoyes no implementada");
        }
        let b = usize::from(cv) * CONVOY_SIZE;
        let next = r16(&c, b + 8);
        w16(&mut m, 0xe4, next);
        w16(&mut c, b + 8, u16::MAX);
        w16(&mut c, b + 0xa, id);
        c[b] = s[p];
        c[b + 1] = s[p + 0xe];
        w16(&mut c, b + 0x1c, u16::from(s[p + 0x39]));
        c[b + 0x1e] = s[p + 0x37];
        c[b + 0x1f] = s[p + 0x38];
        w32(&mut c, b + 4, r32(&s, p + 0x10));
        let f = r32(&c, b + 0x18) | 0x400;
        w32(&mut c, b + 0x18, f);
        w16(&mut s, p + 8, cv);
        w16(&mut c, b + 0xa, id);
        w16(&mut c, b + 0x10, id);
        w16(&mut c, b + 0x14, 2);
        {
            let mut captains = [];
            let mut merchants = [];
            let mut towns = [];
            let mut remove = RemoveWorld {
                manager: &mut m,
                ships: &mut s,
                captains: &mut captains,
                convoys: &mut c,
                merchants: &mut merchants,
                towns: &mut towns,
                pool_limit: 0,
                merchant_count,
                town_count: 0,
                ui_a: false,
                ui_b: false,
            };
            let mut tick = TickWorld {
                remove: &mut remove,
                time: 0,
                player: u32::MAX,
            };
            super::convoy::refresh_from_ships(&mut tick, cv).map_err(|_| "no se pudo inicializar convoy")?;
        }
        w32(&mut c, b + 0x20, 0);
        c[b + 0x24..b + 0x38].fill(0xff);
        w16(&mut c, b + 0x38, u16::MAX);
        w16(&mut c, b + 2, u16::MAX);
        let f = r32(&c, b + 0x18) | 0x4401;
        w32(&mut c, b + 0x18, f);
        let state = r16(&s, p + 0x134);
        w16(&mut c, b + 0x16, 0x100);
        w16(&mut c, b + 0x12, state);
        c[b + 0x14] |= 0x12;
        w16(&mut c, b + 0x12, 0x14);
        w16(&mut s, p + 0x134, 0x14);
        if s[p + 0x15c] != 0 {
            c[b + 0x14] |= 0x20
        }
    }
    let sea_head = r16(&m, 0xea);
    for id in [attacker, target] {
        XOrder {
            ships: &mut s,
            count: n,
            sea_head,
        }
        .remove(id)
    }
    manager.copy_from_slice(&m);
    ships.copy_from_slice(&s);
    convoys.copy_from_slice(&c);
    Ok(BattleConvoyPreparation {
        route_cache_invalidations: invalidations,
    })
}
/// Datos iniciales del combate básico. Los punteros de la imagen son lógicos:
/// las listas y tablas espaciales viven en los campos Rust correspondientes.
#[derive(Debug, Clone)]
pub struct CommercialBattleStart {
    pub object: Vec<u8>,
    pub local_ships: Vec<Vec<u8>>,
    pub next: Vec<u16>,
    pub previous: Vec<u16>,
    pub spatial_0: Vec<u16>,
    pub spatial_1: Vec<u16>,
    pub collisions: BattleCollisionPool,
    pub route_cache_invalidations: Vec<u16>,
    pub notifications: Vec<BattleStartEvent>,
    pub operations: Vec<[u32; 4]>,
    pub open_for_player_requested: bool,
    pub dialog: Option<CommercialBattleDialog>,
}
/// Solicitud de apertura del diálogo; el anfitrión aporta el texto localizado.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommercialBattleDialog {
    pub player: u32,
    pub opponent: u16,
    pub slot: u8,
}
/// Prepara de principio a fin la rama de dos barcos comerciales de IA sueltos.
/// El juego solicita `open_for_player(1)` mediante la virtual 6 (0x00608230):
/// se devuelve como efecto pendiente y no se simula el combate.
/// Sin punteros nativos y sin publicación en la sesión ni escritura S02.
pub fn start_two_commercial_ai_battle(
    manager: &mut [u8],
    ships: &mut [u8],
    convoys: &mut [u8],
    merchants: &[u8],
    captains: &[u8],
    attacker: u16,
    target: u16,
    slot: u8,
    battle_address: u32,
) -> Result<CommercialBattleStart, &'static str> {
    use super::r16;
    use super::remove::{MERCHANT_SIZE, SHIP_SIZE};
    for id in [attacker, target] {
        let p = usize::from(id) * SHIP_SIZE;
        let owner = usize::from(*ships.get(p).ok_or("barco truncado")?);
        let merchant = merchants
            .get(owner * MERCHANT_SIZE..owner * MERCHANT_SIZE + MERCHANT_SIZE)
            .ok_or("comerciante truncado")?;
        if r16(merchant, 8) == 0 {
            return Err("solo se admiten barcos comerciales de IA");
        }
    }
    start_two_commercial_battle(
        manager,
        ships,
        convoys,
        merchants,
        captains,
        attacker,
        target,
        slot,
        battle_address,
        u32::MAX,
    )
}
/// Igual que la rama comercial de `start`, incluyendo avisos humanos, cuando
/// el jugador activo no participa en el encuentro. Sus efectos se devuelven al
/// anfitrión; no ejecuta la llamada a `open_for_player` ni publica la batalla.
pub fn start_two_commercial_battle(
    manager: &mut [u8],
    ships: &mut [u8],
    convoys: &mut [u8],
    merchants: &[u8],
    captains: &[u8],
    attacker: u16,
    target: u16,
    slot: u8,
    battle_address: u32,
    player: u32,
) -> Result<CommercialBattleStart, &'static str> {
    start_two_commercial_battle_impl(
        manager,
        ships,
        convoys,
        merchants,
        captains,
        attacker,
        target,
        slot,
        battle_address,
        player,
        false,
        None,
    )
}
/// Rama con una escena de batalla ya ocupada y sin mapa visible: encola 0x96
/// para el jugador y deja pendiente el marcador de batalla.
pub fn start_two_commercial_battle_player_busy(
    manager: &mut [u8],
    ships: &mut [u8],
    convoys: &mut [u8],
    merchants: &[u8],
    captains: &[u8],
    attacker: u16,
    target: u16,
    slot: u8,
    battle_address: u32,
    player: u32,
) -> Result<CommercialBattleStart, &'static str> {
    start_two_commercial_battle_impl(
        manager,
        ships,
        convoys,
        merchants,
        captains,
        attacker,
        target,
        slot,
        battle_address,
        player,
        true,
        None,
    )
}
/// Inicia la rama de jugador con escena libre y texto ya localizado por el anfitrión.
/// Devuelve la solicitud de apertura; no abre la interfaz por sí misma.
pub fn start_two_commercial_battle_player_free(
    manager: &mut [u8],
    ships: &mut [u8],
    convoys: &mut [u8],
    merchants: &[u8],
    captains: &[u8],
    attacker: u16,
    target: u16,
    slot: u8,
    battle_address: u32,
    player: u32,
    dialog_text: &[u8],
) -> Result<CommercialBattleStart, &'static str> {
    start_two_commercial_battle_impl(
        manager,
        ships,
        convoys,
        merchants,
        captains,
        attacker,
        target,
        slot,
        battle_address,
        player,
        false,
        Some(dialog_text),
    )
}
fn start_two_commercial_battle_impl(
    manager: &mut [u8],
    ships: &mut [u8],
    convoys: &mut [u8],
    merchants: &[u8],
    captains: &[u8],
    attacker: u16,
    target: u16,
    slot: u8,
    battle_address: u32,
    player: u32,
    scene_busy: bool,
    dialog_text: Option<&[u8]>,
) -> Result<CommercialBattleStart, &'static str> {
    use super::remove::{MERCHANT_SIZE, SHIP_SIZE};
    use super::{r16, r32, w16, w32};
    if slot == u8::MAX {
        return Err("ranura de batalla inválida");
    }
    if manager.len() < 0xf8 || merchants.len() % MERCHANT_SIZE != 0 {
        return Err("gestor o comerciantes truncados");
    }
    let mc = merchants.len() / MERCHANT_SIZE;
    if mc > usize::from(u16::MAX) {
        return Err("demasiados comerciantes");
    }
    if player < (mc as u32) && !scene_busy && dialog_text.is_none() {
        return Err("diálogo del jugador no implementado");
    }
    if let Some(text) = dialog_text {
        if text.len() >= 0xb80 || text.contains(&0) {
            return Err("texto de diálogo inválido");
        }
    }
    let n = r16(manager, 0xf4);
    if attacker >= n || target >= n || ships.len() < usize::from(n) * SHIP_SIZE {
        return Err("barcos truncados");
    }
    for id in [attacker, target] {
        let p = usize::from(id) * SHIP_SIZE;
        let owner = usize::from(ships[p]);
        if owner >= mc || ships[p + 0x15c] != 0 {
            return Err("solo se admiten barcos comerciales no piratas");
        }
    }
    let mut m = manager.to_vec();
    let mut s = ships.to_vec();
    let mut c = convoys.to_vec();
    let prep = prepare_single_ship_battle(&mut m, &mut s, &mut c, mc as u16, attacker, target)?;
    let mut obj = vec![0; 0x1e98];
    let mut slots = vec![0; 0x405];
    construct_basic_battle(&mut obj, &mut slots, battle_address, u32::from(slot)).ok_or("constructor truncado")?;
    if start_battle_prefix(&mut obj, &s, attacker, target, 0)? != BattleStartPrefix::Continue {
        return Err("batalla rechazada");
    }
    let events = classify_battle_sides(&mut obj, &c, merchants, mc as u16, false, false, attacker, target)?;
    let open_for_player_requested = obj[0xac8] != 0;
    let convoy_ids = [
        r16(&s, usize::from(attacker) * SHIP_SIZE + 8),
        r16(&s, usize::from(target) * SHIP_SIZE + 8),
    ];
    let participants = [r16(&obj, 0x35a), r16(&obj, 0x35c)];
    let mut pool = BattleCollisionPool::with_capacity(0)?;
    let cap = r16(&m, 0xf2);
    let nc = usize::from(r16(&m, 0xf6));
    let [a, b] = build_commercial_sides_growing(
        &s[..usize::from(n) * SHIP_SIZE],
        &c[..nc * super::remove::CONVOY_SIZE],
        captains,
        cap,
        convoy_ids,
        participants,
        battle_address,
        &mut pool,
    )?;
    if a.ships.len() != 1 || b.ships.len() != 1 {
        return Err("se requieren dos barcos individuales");
    }
    let mut local = a.ships;
    local.extend(b.ships);
    let mut next = a.next;
    next.extend(b.next);
    let mut previous = a.previous;
    previous.extend(b.previous);
    let mut spatial_0 = a.spatial_0;
    spatial_0.extend(b.spatial_0);
    let mut spatial_1 = a.spatial_1;
    spatial_1.extend(b.spatial_1);
    w16(&mut obj, 0x10, 0);
    w16(&mut obj, 0x12, u16::MAX);
    w16(&mut obj, 0x14, 2);
    w16(&mut obj, 0x658, 1);
    w16(&mut obj, 0x65a, 1);
    w16(&mut obj, 0x65c, 1);
    w32(&mut obj, 0xa84, i32::from(attacker));
    w32(&mut obj, 0xa88, i32::from(target));
    obj[0xab0] = 1;
    obj[0xab1] = 1;
    obj[0x12e8] = 1;
    obj[0x5c] = 0;
    let timer0 = if obj[0x5d] == 0 { 250 } else { 0 };
    let timer1 = if obj[0x5e] == 0 { 250 } else { 0 };
    w16(&mut obj, 0x15c, timer0);
    w16(&mut obj, 0x15e, timer1);
    let mut operations = Vec::new();
    let mut dialog = None;
    if player < mc as u32 {
        if participants.iter().any(|&owner| u32::from(owner) == player) {
            obj[0x1e92] = 1
        }
        if !open_for_player_requested {
            for i in 0..2 {
                if u32::from(participants[i]) == player && obj[0x5d + i] == 0 {
                    if scene_busy {
                        operations.push([0x96, u32::from(slot), player, 1]);
                        w16(&mut obj, 0x15c + i * 2, u16::MAX)
                    } else {
                        let text = dialog_text.ok_or("falta texto de diálogo")?;
                        obj[0x1310..0x1310 + text.len()].copy_from_slice(text);
                        w32(&mut obj, 0x12ec, battle_address.wrapping_add(0x1310) as i32);
                        w16(&mut obj, 0x12f0, 1);
                        w16(&mut obj, 0x12f2, 1);
                        w32(&mut obj, 0x12f4, 0x6b2bc0);
                        obj[0x12f8] = 0xff;
                        obj[0x12fb] = 0xff;
                        w32(&mut obj, 0x12fc, 0x3f3504f4);
                        obj[0x12ea] = 1;
                        dialog = Some(CommercialBattleDialog {
                            player,
                            opponent: participants[1 - i],
                            slot,
                        })
                    }
                }
            }
        }
    }
    obj[0x1e90] = 0xff;
    obj[0x1e91] = 0xff;
    w16(&mut obj, 0x20, pool.free_head);
    w16(&mut obj, 0x34, pool.x_head);
    w16(&mut obj, 0x44, pool.y_head);
    w16(&mut obj, 0x54, pool.high_water);
    w16(&mut obj, 0x56, pool.kinds.len() as u16);
    w16(&mut obj, 0x58, (pool.kinds.len() * 2) as u16);
    w16(&mut obj, 0x5a, pool.matrices[0].len() as u16);
    let first_rng = advance_battle_rng(r32(&obj, 0xadc) as u32);
    let mut wind = (first_rng & 3) as i32 + 5;
    let center_x = r16(&obj, 0xad6) as i16;
    let center_y = r16(&obj, 0xada) as i16;
    for i in 0..16usize {
        let at = 0x20 + i * 12;
        if r16(&m, at) < n {
            let dx = i32::from(r16(&m, at + 4) as i16) - i32::from(center_x);
            let dy = i32::from(r16(&m, at + 6) as i16) - i32::from(center_y);
            if dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy)) < 0x385 {
                match r16(&m, at + 8) {
                    0 => wind += 3,
                    1 => wind -= 3,
                    _ => {}
                }
            }
        }
    }
    let final_rng = advance_battle_rng(first_rng);
    w32(&mut obj, 0xadc, final_rng as i32);
    obj[0x670] = final_rng as u8;
    obj[0x671] = wind.clamp(2, 10) as u8;
    manager.copy_from_slice(&m);
    ships.copy_from_slice(&s);
    convoys.copy_from_slice(&c);
    Ok(CommercialBattleStart {
        object: obj,
        local_ships: local,
        next,
        previous,
        spatial_0,
        spatial_1,
        collisions: pool,
        route_cache_invalidations: prep.route_cache_invalidations,
        notifications: events,
        operations,
        open_for_player_requested,
        dialog,
    })
}
impl CommercialBattleStart {
    /// Expone el estado inicial sin proyectiles al escritor de batallas existente.
    /// No publica el bloque ni sustituye otras batallas del gestor.
    pub fn initial_save_input(&self) -> Result<BattleSaveInput<'_>, &'static str> {
        if self.object.len() < 0xad0 {
            return Err("batalla truncada");
        }
        if super::r16(&self.object, 0xacc) != 0 || super::r16(&self.object, 0xace) != 0 {
            return Err("la batalla ya contiene proyectiles");
        }
        Ok(BattleSaveInput {
            object: &self.object,
            nexts: &self.next,
            prevs: &self.previous,
            bitmap: &[],
            local_ships: &self.local_ships,
            projectiles: &[],
        })
    }
}
/// Encadena la decisión de persecución 0x0050B030 con el inicio comercial de
/// dos barcos sueltos. Solo admite la rama de escena ocupada o jugador ajeno;
/// la apertura de vista, UI y publicación del estado quedan en el anfitrión.
/// Cualquier efecto de ruta u otra variante se rechaza de forma atómica.
pub fn start_commercial_battle_from_pursuit(
    manager: &mut [u8],
    ships: &mut [u8],
    convoys: &mut [u8],
    merchants: &[u8],
    captains: &[u8],
    hunter: u16,
    town_count: u32,
    tick: u32,
    zones: &[(i16, i16, i16, i16)],
    slot: u8,
    battle_address: u32,
    player: u32,
    scene_busy: bool,
) -> Result<(bool, Option<CommercialBattleStart>), &'static str> {
    start_commercial_battle_from_pursuit_impl(
        manager,
        ships,
        convoys,
        merchants,
        captains,
        hunter,
        town_count,
        tick,
        zones,
        slot,
        battle_address,
        player,
        scene_busy,
        None,
    )
}
/// Persecución con diálogo localizado para la escena libre.
pub fn start_commercial_battle_from_pursuit_player_free(
    manager: &mut [u8],
    ships: &mut [u8],
    convoys: &mut [u8],
    merchants: &[u8],
    captains: &[u8],
    hunter: u16,
    town_count: u32,
    tick: u32,
    zones: &[(i16, i16, i16, i16)],
    slot: u8,
    battle_address: u32,
    player: u32,
    dialog_text: &[u8],
) -> Result<(bool, Option<CommercialBattleStart>), &'static str> {
    start_commercial_battle_from_pursuit_impl(
        manager,
        ships,
        convoys,
        merchants,
        captains,
        hunter,
        town_count,
        tick,
        zones,
        slot,
        battle_address,
        player,
        false,
        Some(dialog_text),
    )
}
fn start_commercial_battle_from_pursuit_impl(
    manager: &mut [u8],
    ships: &mut [u8],
    convoys: &mut [u8],
    merchants: &[u8],
    captains: &[u8],
    hunter: u16,
    town_count: u32,
    tick: u32,
    zones: &[(i16, i16, i16, i16)],
    slot: u8,
    battle_address: u32,
    player: u32,
    scene_busy: bool,
    dialog_text: Option<&[u8]>,
) -> Result<(bool, Option<CommercialBattleStart>), &'static str> {
    let mut m = manager.to_vec();
    let mut s = ships.to_vec();
    let mut c = convoys.to_vec();
    let (result, effect) = super::pursuit::update_pursuit(&m, &mut s, hunter, town_count, tick, zones)?;
    match effect {
        None => {
            ships.copy_from_slice(&s);
            Ok((result, None))
        }
        Some(super::pursuit::PursuitEffect::Battle(a, b)) => {
            let battle = start_two_commercial_battle_impl(
                &mut m,
                &mut s,
                &mut c,
                merchants,
                captains,
                a,
                b,
                slot,
                battle_address,
                player,
                scene_busy,
                dialog_text,
            )?;
            manager.copy_from_slice(&m);
            ships.copy_from_slice(&s);
            convoys.copy_from_slice(&c);
            Ok((result, Some(battle)))
        }
        _ => Err("efecto de persecución distinto del encuentro"),
    }
}

/// Efectos externos del cambio de política de combate (operación 0x96).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattlePolicyEffect {
    SlowTime,
    NormalTime,
    OpenForPlayer,
}

/// `op_sea_battle_set_manual_control` (0x60FEC9). Las dos listas de barcos
/// locales y el índice seleccionado se validan antes de publicar cambios.
/// Devuelve las llamadas de interfaz/reloj para que el anfitrión las ejecute.
pub fn set_battle_participant_policy(
    object: &mut [u8],
    ships: &mut [Vec<u8>],
    next: &[u16],
    selected: &mut u8,
    player: u32,
    merchant: u16,
    policy: u8,
) -> Result<Vec<BattlePolicyEffect>, &'static str> {
    use super::{r16, w16};
    if object.len() < 0x1e98 {
        return Err("batalla truncada");
    }
    let count = usize::from(object[0x558]);
    if count > 2 {
        return Err("participantes no admitidos");
    }
    let mut lists = Vec::new();
    if policy == 0 {
        for off in [0x10usize, 0x65c] {
            let mut id = r16(object, off);
            let mut seen = vec![false; ships.len()];
            while id != u16::MAX {
                let i = usize::from(id);
                let ship = ships.get(i).ok_or("barco local inexistente")?;
                if seen[i] || i >= next.len() || ship.len() < 0x164 || usize::from(ship[0x13d]) >= count {
                    return Err("lista local inválida");
                };
                seen[i] = true;
                lists.push(i);
                id = next[i]
            }
        }
    }
    let mut staged = object.to_vec();
    let mut local = ships.to_vec();
    let mut slot = *selected;
    let mut effects = Vec::new();
    if player == u32::from(merchant) {
        staged[0x12e0] = 0;
        if policy == 0 {
            slot = staged[0x662]
        } else if u16::from(slot) == r16(&staged, 0x662) {
            slot = 0xff
        }
    }
    let had_timer = (0..count).any(|i| r16(&staged, 0x15c + 2 * i) != 0);
    for i in 0..count {
        if r16(&staged, 0x35a + 2 * i) == merchant {
            staged[0x5d + i] = policy;
            w16(&mut staged, 0x15c + 2 * i, 0)
        }
    }
    let other_manual = (0..count).any(|i| staged[0x5d + i] == 0 && r16(&staged, 0x35a + 2 * i) != merchant);
    if !other_manual {
        if policy == 0 {
            staged[0xac8] = 0;
            effects.push(BattlePolicyEffect::SlowTime)
        } else {
            staged[0xac8] = 1;
            effects.push(BattlePolicyEffect::NormalTime)
        }
    }
    if had_timer && !(0..count).any(|i| r16(&staged, 0x15c + 2 * i) != 0) {
        effects.push(BattlePolicyEffect::OpenForPlayer)
    }
    for i in lists {
        let side = usize::from(local[i][0x13d]);
        if r16(&staged, 0x35a + 2 * side) == merchant {
            let heading = u16::from(local[i][0x11]);
            w16(&mut local[i], 0x24, heading);
            w16(&mut local[i], 0x128, u16::MAX)
        }
    }
    object.copy_from_slice(&staged);
    ships.clone_from_slice(&local);
    *selected = slot;
    Ok(effects)
}

/// Resultado de la rama que espera la decisión humana en `SeaBattle::update`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecisionCountdown {
    pub close_dialog: bool,
}

/// Ejecuta solamente la cuenta atrás humana con la interfaz ocupada.
/// Rechaza la simulación de barcos y cualquier rama de interfaz no cubierta;
/// los campos se dejan intactos al devolver error.
pub fn update_human_decision_countdown(
    object: &mut [u8],
    player: u32,
    delta: u16,
    ui_busy: bool,
) -> Result<DecisionCountdown, &'static str> {
    use super::{r16, w16};
    if object.len() < 0x1e98 {
        return Err("batalla truncada");
    }
    let n = usize::from(object[0x558]);
    if n > 2 || !ui_busy || object[0x5c] != 0 || object[0x1e92] != 0 || object[0x12e9] != 0 || object[0x12ea] != 0 {
        return Err("estado de interfaz o batalla no admitido");
    }
    if delta == 0 {
        return Ok(DecisionCountdown { close_dialog: false });
    }
    if !(0..n).any(|i| r16(object, 0x15c + 2 * i) != 0) {
        return Err("el combate debe avanzar");
    }
    let mut staged = object.to_vec();
    let mut close = false;
    let delta = delta.min(5);
    for i in 0..n {
        let timer = r16(&staged, 0x15c + 2 * i);
        if u32::from(r16(&staged, 0x35a + 2 * i)) == player && staged[0x5d + i] == 0 && timer != 0 && timer != u16::MAX
        {
            if delta < timer {
                w16(&mut staged, 0x15c + 2 * i, timer - delta)
            } else {
                w16(&mut staged, 0x15c + 2 * i, 0xfffe);
                close = true
            }
        }
    }
    object.copy_from_slice(&staged);
    Ok(DecisionCountdown { close_dialog: close })
}

/// Aviso de una batalla que contiene un comerciante humano; la interfaz lo
/// recibe con coordenadas con signo del centro de la batalla.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BattlePresenceNotice {
    pub slot: u16,
    pub x: i16,
    pub y: i16,
}
/// Recalcula el marcador de presencia de ambos convoyes (prefijo de 0x606cc0).
/// Devuelve el aviso a la interfaz, sin abrirla. Rechaza listas inválidas de
/// manera atómica y deja el resto de la actualización al llamante.
pub fn refresh_battle_player_presence(
    object: &mut [u8],
    ships: &[u8],
    convoys: &[u8],
    merchants: &[u8],
    player: u32,
    hotseat: bool,
    interface_visible: bool,
) -> Result<Option<BattlePresenceNotice>, &'static str> {
    use super::r16;
    use super::remove::{CONVOY_SIZE, MERCHANT_SIZE, SHIP_SIZE};
    if object.len() < 0x1e98
        || ships.len() % SHIP_SIZE != 0
        || convoys.len() % CONVOY_SIZE != 0
        || merchants.len() % MERCHANT_SIZE != 0
    {
        return Err("imágenes de batalla truncadas");
    }
    if object[0x5c] != 0 || object[0x1e92] == 0 {
        return Ok(None);
    }
    let n = ships.len() / SHIP_SIZE;
    let mut present = false;
    for off in [0x65eusize, 0x660] {
        let cv = usize::from(r16(object, off));
        if cv >= convoys.len() / CONVOY_SIZE {
            return Err("convoy de batalla inexistente");
        }
        let mut id = r16(convoys, cv * CONVOY_SIZE + 0xa);
        let mut seen = vec![false; n];
        while usize::from(id) < n {
            let i = usize::from(id);
            if seen[i] {
                return Err("lista de convoy cíclica");
            }
            seen[i] = true;
            let at = i * SHIP_SIZE;
            let owner = usize::from(ships[at]);
            if player == owner as u32
                || (hotseat
                    && owner < merchants.len() / MERCHANT_SIZE
                    && r16(merchants, owner * MERCHANT_SIZE + 8) == 0)
            {
                present = true
            }
            id = r16(ships, at + 6)
        }
    }
    object[0x1e92] = u8::from(present);
    if present && interface_visible {
        object[0x1e92] = 0;
        Ok(Some(BattlePresenceNotice {
            slot: r16(object, 0x662),
            x: r16(object, 0xad6) as i16,
            y: r16(object, 0xada) as i16,
        }))
    } else {
        Ok(None)
    }
}

/// Efectos del tramo de `SeaBattle::update` que espera una decisión humana.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HumanBattleWait {
    pub presence: Option<BattlePresenceNotice>,
    pub close_dialog: bool,
}
/// Encadena la actualización de presencia y la cuenta atrás cuando la interfaz
/// está visible y ocupada. No ejecuta aún movimientos, daños ni resolución.
pub fn update_human_battle_wait(
    object: &mut [u8],
    ships: &[u8],
    convoys: &[u8],
    merchants: &[u8],
    player: u32,
    hotseat: bool,
    delta: u16,
) -> Result<HumanBattleWait, &'static str> {
    let mut staged = object.to_vec();
    let presence = refresh_battle_player_presence(&mut staged, ships, convoys, merchants, player, hotseat, true)?;
    let countdown = update_human_decision_countdown(&mut staged, player, delta, true)?;
    object.copy_from_slice(&staged);
    Ok(HumanBattleWait {
        presence,
        close_dialog: countdown.close_dialog,
    })
}

/// Rama sin listas auxiliares ni cierre de interfaz de `handle_class11_tick`
/// (0x006127C0). Su sincronización externa se deja al anfitrión; la imagen del
/// gestor no cambia. No ejecuta `SeaBattle::update`.
pub fn tick_idle_battle_manager(manager: &mut [u8]) -> Result<(), &'static str> {
    if manager.len() < 0xc38 {
        return Err("gestor de batallas truncado");
    }
    if manager[0x42c] != 0 || r32(manager, 0xc34) != 0 {
        return Err("gestor de batallas activo no admitido");
    }
    Ok(())
}
