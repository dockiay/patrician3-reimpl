//! Persecución naval `ships::update_pursuit` (0x0050B030).
//! Efectos externos se devuelven al anfitrión para aplicarlos transaccionalmente.
use super::{r16, r32, remove::SHIP_SIZE, w16};
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PursuitEffect {
    ClearRoute(u16),
    Battle(u16, u16),
    Plan(u16, i16, i16),
}
/// La distancia máxima es el word `manager+0xFC`; los rectángulos proceden
/// de la tabla 0x00673564 del ejecutable inglés. No ejecuta combate ni ruta.
pub fn update_pursuit(
    manager: &[u8],
    ships: &mut [u8],
    ship: u16,
    town_count: u32,
    tick: u32,
    zones: &[(i16, i16, i16, i16)],
) -> Result<(bool, Option<PursuitEffect>), &'static str> {
    if manager.len() < 0xfe {
        return Err("gestor truncado");
    }
    let count = r16(manager, 0xf4);
    if ship >= count || ships.len() < usize::from(count) * SHIP_SIZE {
        return Err("barcos truncados");
    }
    let o = usize::from(ship) * SHIP_SIZE;
    let target = r16(ships, o + 0x3a);
    let clear = |s: &mut [u8]| w16(s, o + 0x3a, u16::MAX);
    if target >= count {
        clear(ships);
        return Ok((false, None));
    }
    let state = r16(ships, o + 0x134);
    if !matches!(state, 0xf | 0x10 | 0x16) {
        clear(ships);
        return Ok((false, None));
    }
    let t = usize::from(target) * SHIP_SIZE;
    let tstate = r16(ships, t + 0x134);
    let pirate = ships[o + 0x15c] != 0;
    if pirate && u32::from(ships[t + 0x39]) < town_count {
        clear(ships);
        return Ok((false, Some(PursuitEffect::ClearRoute(ship))));
    }
    let needs_links = if pirate {
        match tstate {
            0xf | 0x10 => false,
            0x12 => true,
            _ => {
                clear(ships);
                return Ok((false, None));
            }
        }
    } else {
        match tstate {
            0x12 => true,
            0xf if ships[t + 0x15c] != 0 => false,
            _ => {
                clear(ships);
                return Ok((false, None));
            }
        }
    };
    if needs_links && r16(ships, t + 0xa) >= count && r16(ships, t + 0xc) >= count {
        clear(ships);
        return Ok((false, None));
    }
    let x = r16(ships, o + 0x1e) as i16;
    let y = r16(ships, o + 0x22) as i16;
    let dx = (r16(ships, t + 0x1e) as i16 as i32) - (x as i32);
    let dy = (r16(ships, t + 0x22) as i16 as i32) - (y as i32);
    let distance = dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy));
    if distance > i32::from(r16(manager, 0xfc)) {
        if state != 0x16 {
            clear(ships)
        }
        return Ok((false, None));
    }
    if distance < 400 {
        clear(ships);
        return Ok((true, Some(PursuitEffect::Battle(ship, target))));
    }
    if ships[o + 0xe] > 1
        && zones
            .iter()
            .any(|&(x1, y1, x2, y2)| x >= x1 && x <= x2 && y >= y1 && y <= y2)
    {
        clear(ships);
        return Ok((false, Some(PursuitEffect::ClearRoute(ship))));
    }
    if tick & 7 == 0 {
        let px = r32(ships, t + 0x1c).wrapping_add(r32(ships, t + 0x28).wrapping_mul(4));
        let py = r32(ships, t + 0x20).wrapping_add(r32(ships, t + 0x2c).wrapping_mul(4));
        return Ok((
            false,
            Some(PursuitEffect::Plan(ship, (px >> 16) as i16, (py >> 16) as i16)),
        ));
    }
    Ok((false, None))
}
