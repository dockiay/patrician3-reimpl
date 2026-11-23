//! Búsqueda de presa pirata 0x0050D9E0 en las dos cadenas ordenadas por X.
use super::{r16, remove::SHIP_SIZE, w16};
/// Rechaza listas corruptas que el original recorrería indefinidamente.
pub fn find_prey(
    manager: &[u8],
    ships: &mut [u8],
    ship: u16,
    zones: &[(i16, i16, i16, i16)],
) -> Result<bool, &'static str> {
    if manager.len() < 0x100 {
        return Err("gestor truncado");
    }
    let count = r16(manager, 0xf4);
    if ship >= count || ships.len() < usize::from(count) * SHIP_SIZE {
        return Err("barcos truncados");
    }
    let at = usize::from(ship) * SHIP_SIZE;
    let x = r16(ships, at + 0x1e) as i16;
    let y = r16(ships, at + 0x22) as i16;
    let excluded = |x: i16, y: i16| {
        zones
            .iter()
            .any(|&(x0, y0, x1, y1)| x >= x0 && x <= x1 && y >= y0 && y <= y1)
    };
    let distance = |a: i16, b: i16| {
        let dx = i32::from(a) - i32::from(x);
        let dy = i32::from(b) - i32::from(y);
        dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy))
    };
    let old = r16(ships, at + 0x50);
    if old < count {
        let p = usize::from(old) * SHIP_SIZE;
        let state = r16(ships, p + 0x134);
        let px = r16(ships, p + 0x1e) as i16;
        let py = r16(ships, p + 0x22) as i16;
        if matches!(state, 0xf | 0x10) && distance(px, py) < i32::from(r16(manager, 0xfc)) && !excluded(px, py) {
            return Ok(true);
        }
    }
    let mut best = i32::from(r16(manager, 0xfa));
    let range = i32::from(r16(manager, 0xfe));
    let mut found = u16::MAX;
    for (link, lower) in [(0xa, true), (0xc, false)] {
        let mut id = r16(ships, at + link);
        let mut seen = vec![false; usize::from(count)];
        while id < count {
            let i = usize::from(id);
            if seen[i] {
                return Err("lista de presas cíclica");
            }
            seen[i] = true;
            let p = i * SHIP_SIZE;
            let px = r16(ships, p + 0x1e) as i16;
            let py = r16(ships, p + 0x22) as i16;
            if (lower && i32::from(px) < i32::from(x) - range) || (!lower && i32::from(px) > i32::from(x) + range) {
                break;
            }
            let d = distance(px, py);
            if r16(ships, p + 0x134) != 0x15 && d < best && !excluded(px, py) {
                best = d;
                found = id
            }
            id = r16(ships, p + link);
        }
    }
    w16(ships, at + 0x50, found);
    Ok(found < count)
}
/// Efecto diferido de `pirate_check_attack`: limpiar la ruta actual.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttackEffect {
    ClearRoute,
}

/// Decisión de ataque 0x00515360. La velocidad de batalla es la rutina 0x00612930;
/// el llamador aplica la limpieza de ruta al mismo tiempo que el cambio de objetivo.
pub fn check_attack(
    w: &mut super::tick::TickWorld,
    ship: u16,
    difficulty: u8,
    route: Option<&[(i16, i16)]>,
    zones: &[(i16, i16, i16, i16)],
) -> Result<(bool, Option<AttackEffect>), &'static str> {
    use super::{
        boarding::{effective_speed, Captains},
        r32,
        remove::{CONVOY_SIZE, MERCHANT_SIZE},
        w16,
    };
    let count = r16(w.remove.manager, 0xf4);
    let cap = r16(w.remove.manager, 0xf6);
    let capcount = r16(w.remove.manager, 0xf2);
    let o = usize::from(ship) * SHIP_SIZE;
    if ship >= count
        || w.remove.ships.len() < usize::from(count) * SHIP_SIZE
        || w.remove.captains.len() < usize::from(capcount) * 16
    {
        return Err("flota o capitanes truncados");
    }
    let target = r16(w.remove.ships, o + 0x50);
    w16(w.remove.ships, o + 0x52, u16::MAX);
    if target >= count {
        return Ok((false, None));
    }
    let t = usize::from(target) * SHIP_SIZE;
    let owner = w.remove.ships[t];
    let pirate = w.remove.ships[o + 0x15d];
    let valid = owner != pirate
        && u16::from(owner) < w.remove.merchant_count
        && matches!(r16(w.remove.ships, t + 0x134), 0xf | 0x10);
    if !valid {
        w16(w.remove.ships, o + 0x50, u16::MAX);
        return Ok((false, None));
    }
    let m = usize::from(owner) * MERCHANT_SIZE;
    if m + MERCHANT_SIZE > w.remove.merchants.len() {
        return Err("comerciante truncado");
    }
    let kind = r16(w.remove.merchants, m + 8);
    if kind != 0 {
        if kind & 4 != 0 {
            w16(w.remove.ships, o + 0x50, u16::MAX);
            return Ok((false, None));
        }
        if r16(w.remove.ships, o + 0x138) > 0 {
            return Ok((false, None));
        }
    } else {
        let rank = usize::from(w.remove.merchants[m + 0x19]);
        if u32::from(w.remove.merchants[m + 0x39c + rank]) + u32::from(difficulty) < 2 {
            w16(w.remove.ships, o + 0x50, u16::MAX);
            return Ok((false, None));
        }
        if r16(w.remove.ships, o + 0x138) > 0x900 {
            return Ok((false, None));
        }
    }
    if u16::from(pirate) < w.remove.merchant_count {
        return Ok((true, None));
    }
    let own_cv = r16(w.remove.ships, o + 8);
    if own_cv >= cap {
        return Ok((false, None));
    }
    let captains = Captains {
        table: w.remove.captains,
        count: capcount,
    };
    let chain = |id: u16| -> Result<Vec<usize>, &'static str> {
        let b = usize::from(id) * CONVOY_SIZE;
        if b + CONVOY_SIZE > w.remove.convoys.len() {
            return Err("convoy truncado");
        }
        let mut id = r16(w.remove.convoys, b + 0xa);
        let mut seen = vec![false; usize::from(count)];
        let mut out = Vec::new();
        while id < count {
            let i = usize::from(id);
            if seen[i] {
                return Err("convoy cíclico");
            }
            seen[i] = true;
            let at = i * SHIP_SIZE;
            out.push(at);
            id = r16(w.remove.ships, at + 6)
        }
        Ok(out)
    };
    let own = chain(own_cv)?;
    let mut own_speed = 0u32;
    let mut own_crew = 0u32;
    let mut own_guns = 0u32;
    let mut capacity = false;
    for &p in &own {
        let s = &w.remove.ships[p..p + SHIP_SIZE];
        own_speed = own_speed.max(effective_speed(s, captains)?);
        let crew = u32::from(r16(s, 0x40));
        let guns = r32(s, 0x120) as u32;
        own_crew = own_crew.wrapping_add(crew);
        own_guns = own_guns.wrapping_add(if (crew as i32) > (guns as i32) { crew } else { guns });
        if r32(s, 0x10).wrapping_sub(r32(s, 0x11c)).wrapping_sub(r32(s, 0x118)) >= 2000 {
            capacity = true
        }
    }
    let target_cv = r16(w.remove.ships, t + 8);
    let (mut speed, mut crew, mut guns, mut cargo) = (0x7fff_ffffu32, 0u32, 0u32, false);
    if target_cv >= cap {
        let s = &w.remove.ships[t..t + SHIP_SIZE];
        speed = effective_speed(s, captains)?;
        guns = r32(s, 0x120) as u32;
        crew = u32::from(r16(s, 0x40));
        cargo = r32(s, 0x118) > 0
    } else {
        for p in chain(target_cv)? {
            let s = &w.remove.ships[p..p + SHIP_SIZE];
            speed = speed.min(effective_speed(s, captains)?);
            let c = u32::from(r16(s, 0x40));
            let g = r32(s, 0x120) as u32;
            crew = crew.wrapping_add(c);
            guns = guns.wrapping_add(if (c as i32) > (g as i32) { c } else { g });
            cargo |= r32(s, 0x118) > 0
        }
    }
    if speed == 0x7fff_ffff
        || speed.wrapping_mul(19) > own_speed.wrapping_mul(20)
        || guns.wrapping_add(crew).wrapping_mul(7) > own_guns.wrapping_add(own_crew).wrapping_mul(10)
        || !cargo
        || !capacity
    {
        w16(w.remove.ships, o + 0x50, u16::MAX);
        return Ok((false, None));
    }
    let tx = r16(w.remove.ships, t + 0x1e) as i16;
    let ty = r16(w.remove.ships, t + 0x22) as i16;
    if zones
        .iter()
        .any(|&(x0, y0, x1, y1)| tx >= x0 && tx <= x1 && ty >= y0 && ty <= y1)
    {
        w16(w.remove.ships, o + 0x50, u16::MAX);
        return Ok((false, None));
    }
    let x = r16(w.remove.ships, o + 0x1e) as i16;
    let y = r16(w.remove.ships, o + 0x22) as i16;
    let square = |x: i16, y: i16, a: i16, b: i16| {
        let dx = i32::from(x) - i32::from(a);
        let dy = i32::from(y) - i32::from(b);
        dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy))
    };
    if square(tx, ty, x, y) <= 400 {
        return Ok((true, Some(AttackEffect::ClearRoute)));
    }
    if let Some(&(rx, ry)) = route.and_then(|r| r.last()) {
        if square(rx, ry, tx, ty) < 10 {
            return Ok((false, None));
        }
    }
    w16(w.remove.ships, o + 0x52, target);
    Ok((false, Some(AttackEffect::ClearRoute)))
}
