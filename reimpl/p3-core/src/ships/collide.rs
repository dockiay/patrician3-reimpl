//! Contactos entre cascos y con obstáculos del combate naval: contacto de polígonos (0x00609D40), rayo contra casco
//! (0x0060ADBF), choque de dos barcos (0x00608F70), inicio de abordaje (0x00609BA0), obstáculo rectangular (0x0060B357),
//! cercanía (0x0060BC5A), marcado de holguras (0x0060BCDA) y el despachador de pares del índice espacial (0x00608850).
//! Fixture: `oracle/x86_battle_collide.py`.

use super::boarding::{Boarding, Captains};
use super::hull::HULLS;
use super::projectile::{heading, ATAN};
use super::wind::table_sine;
use super::{r16, r32, w16, w32};
use crate::numeric::ftol_i32;

type E = &'static str;

/// Parámetros de choque por tipo de casco: 0x0067AB88, 0x0067AB90 y los ángulos de arista 0x0067AB98.
const MASS: [i32; 4] = [102, 121, 121, 148];
const SPREAD: [i32; 4] = [38, 42, 50, 44];
const EDGE_ANGLE: [[u8; 5]; 4] = [
    [103, 153, 192, 0, 64],
    [105, 151, 193, 0, 63],
    [107, 149, 195, 0, 61],
    [110, 146, 193, 0, 63],
];

fn x_of(ship: &[u8]) -> i32 {
    i32::from(r16(ship, 0xA) as i16)
}
fn y_of(ship: &[u8]) -> i32 {
    i32::from(r16(ship, 0xE) as i16)
}
fn div(a: i32, b: i32) -> Result<i32, E> {
    a.checked_div(b).ok_or("excepción de división (#DE)")
}
fn sin_cos(heading: u8) -> (i32, i32) {
    (table_sine(heading), table_sine(heading.wrapping_add(0x40)))
}
fn rotate((s, c): (i32, i32), (x, y): (i32, i32)) -> (i32, i32) {
    (
        c.wrapping_mul(x).wrapping_sub(s.wrapping_mul(y)) >> 16,
        c.wrapping_mul(y).wrapping_add(s.wrapping_mul(x)) >> 16,
    )
}
fn world_kind(ship: &[u8], worlds: &[u8]) -> Result<usize, E> {
    let at = usize::try_from(r32(ship, 4))
        .ok()
        .and_then(|i| i.checked_mul(0x180))
        .ok_or("barco mundial fuera de rango")?;
    let kind = *worlds.get(at + 0xE).ok_or("barco mundial fuera de rango")?;
    if kind > 3 {
        return Err("tipo de casco fuera de la tabla");
    }
    Ok(usize::from(kind))
}
fn ship_at(ships: &[Vec<u8>], id: usize) -> Result<&Vec<u8>, E> {
    match ships.get(id) {
        Some(s) if s.len() >= 0x170 => Ok(s),
        _ => Err("barco local ausente"),
    }
}

fn hull_point(kind: usize, i: usize) -> (i32, i32) {
    let (x, y) = HULLS[kind][i];
    (i32::from(x), i32::from(y))
}

/// 0x00609D40: puntos de corte entre los contornos de dos cascos; devuelve su media entera.
pub fn hull_contact(ships: &[Vec<u8>], worlds: &[u8], a: usize, b: usize) -> Result<Option<(i32, i32)>, E> {
    let (sa, sb) = (ship_at(ships, a)?, ship_at(ships, b)?);
    let (ka, kb) = (world_kind(sa, worlds)?, world_kind(sb, worlds)?);
    let (ra, rb) = (sin_cos(sa[0x11]), sin_cos(sb[0x11]));
    let offset = (x_of(sb).wrapping_sub(x_of(sa)), y_of(sb).wrapping_sub(y_of(sa)));
    let (mut ax, mut ay) = (0i32, 0i32);
    let (mut dx, mut dy) = rotate(ra, hull_point(ka, 4));
    let (mut sum_x, mut sum_y, mut count) = (0i32, 0i32, 0u8);
    let mut hit = false;
    for i in 0..5 {
        ax = ax.wrapping_add(dx);
        ay = ay.wrapping_add(dy);
        let (px, py) = rotate(ra, hull_point(ka, i));
        dx = px.wrapping_sub(ax);
        dy = py.wrapping_sub(ay);
        let place = |(x, y): (i32, i32)| {
            (
                x.wrapping_add(offset.0).wrapping_sub(ax),
                y.wrapping_add(offset.1).wrapping_sub(ay),
            )
        };
        let (mut sx, mut sy) = place(rotate(rb, hull_point(kb, 4)));
        for j in 0..5 {
            let (ex, ey) = place(rotate(rb, hull_point(kb, j)));
            let mut first = dx.wrapping_mul(sy).wrapping_sub(dy.wrapping_mul(sx));
            let mut second = dx
                .wrapping_mul(sy.wrapping_sub(ey))
                .wrapping_sub(dy.wrapping_mul(sx.wrapping_sub(ex)));
            if second < 0 {
                first = first.wrapping_neg();
                second = second.wrapping_neg();
            }
            if first <= second && first >= 0 {
                let mut third = sx
                    .wrapping_mul(ey.wrapping_sub(sy))
                    .wrapping_sub(sy.wrapping_mul(ex.wrapping_sub(sx)));
                let mut fourth = dx
                    .wrapping_mul(ey.wrapping_sub(sy))
                    .wrapping_sub(dy.wrapping_mul(ex.wrapping_sub(sx)));
                if fourth < 0 {
                    third = third.wrapping_neg();
                    fourth = fourth.wrapping_neg();
                }
                if fourth > 0 && third <= fourth && third >= 0 {
                    hit = true;
                    sum_x = x_of(sa)
                        .wrapping_add(ax)
                        .wrapping_add(div(dx.wrapping_mul(third), fourth)?)
                        .wrapping_add(sum_x);
                    sum_y = y_of(sa)
                        .wrapping_add(ay)
                        .wrapping_add(div(dy.wrapping_mul(third), fourth)?)
                        .wrapping_add(sum_y);
                    count = count.wrapping_add(1);
                }
            }
            sx = ex;
            sy = ey;
        }
    }
    if count != 0 {
        sum_x = div(sum_x, i32::from(count))?;
        sum_y = div(sum_y, i32::from(count))?;
    }
    Ok(if hit { Some((sum_x, sum_y)) } else { None })
}

/// 0x0060ADBF: primer corte del rayo `from -> to` con el casco de `id`; devuelve el punto y el índice de arista.
/// Sin corte, el original deja el punto en `to` y la arista sin escribir (`None`).
pub fn ray_hull(
    ships: &[Vec<u8>],
    worlds: &[u8],
    id: usize,
    from: (i32, i32),
    to: (i32, i32),
) -> Result<((i32, i32), Option<u8>), E> {
    let ship = ship_at(ships, id)?;
    let kind = world_kind(ship, worlds)?;
    let r = sin_cos(ship[0x11]);
    let (dx, dy) = (to.0.wrapping_sub(from.0), to.1.wrapping_sub(from.1));
    let place = |(x, y): (i32, i32)| {
        (
            x.wrapping_add(x_of(ship)).wrapping_sub(from.0),
            y.wrapping_add(y_of(ship)).wrapping_sub(from.1),
        )
    };
    let (mut sx, mut sy) = place(rotate(r, hull_point(kind, 4)));
    let mut best = i32::MAX;
    let mut edge = None;
    for j in 0..5usize {
        let (ex, ey) = place(rotate(r, hull_point(kind, j)));
        let mut first = dx.wrapping_mul(sy).wrapping_sub(dy.wrapping_mul(sx));
        let mut second = dx
            .wrapping_mul(sy.wrapping_sub(ey))
            .wrapping_sub(dy.wrapping_mul(sx.wrapping_sub(ex)));
        if second < 0 {
            first = first.wrapping_neg();
            second = second.wrapping_neg();
        }
        if first <= second && first >= 0 {
            let mut third = sx
                .wrapping_mul(ey.wrapping_sub(sy))
                .wrapping_sub(sy.wrapping_mul(ex.wrapping_sub(sx)));
            let mut fourth = dx
                .wrapping_mul(ey.wrapping_sub(sy))
                .wrapping_sub(dy.wrapping_mul(ex.wrapping_sub(sx)));
            if fourth < 0 {
                third = third.wrapping_neg();
                fourth = fourth.wrapping_neg();
            }
            if third >= 0 && fourth > 0 {
                let ratio = div(third.wrapping_shl(8), fourth)?;
                if ratio < best {
                    best = ratio;
                    edge = Some(((j + 4) % 5) as u8);
                }
            }
        }
        sx = ex;
        sy = ey;
    }
    let point = if best < i32::MAX {
        (
            from.0.wrapping_add(dx.wrapping_mul(best) >> 8),
            from.1.wrapping_add(dy.wrapping_mul(best) >> 8),
        )
    } else {
        to
    };
    Ok((point, edge))
}

/// Un lado del choque (A o B): empuje por el punto de contacto, giro e impulso. Devuelve `Err` si el rayo no cortó el casco
/// (el original lee entonces la pila sin inicializar).
struct Impact {
    spin: i16,
    heading: u8,
    cosine: i32,
}

#[allow(clippy::too_many_arguments)]
fn impact_side(
    ships: &mut [Vec<u8>],
    id: usize,
    kind: usize,
    contact: (i32, i32),
    edge: u8,
    direct: bool,
    scale: (u32, u32),
    first_side: bool,
) -> Result<Impact, E> {
    let (px, py) = {
        let ship = &ships[id];
        (contact.0.wrapping_sub(x_of(ship)), contact.1.wrapping_sub(y_of(ship)))
    };
    let own_heading = ships[id][0x11];
    let (s, c) = sin_cos(own_heading);
    let rel_y = c.wrapping_mul(py).wrapping_sub(s.wrapping_mul(px)) >> 16;
    let rel_x = s.wrapping_mul(py).wrapping_add(c.wrapping_mul(px)) >> 16;
    let angle = heading(rel_x.wrapping_neg(), rel_y.wrapping_neg());
    let offset = *EDGE_ANGLE[kind]
        .get(usize::from(edge))
        .ok_or("arista fuera de la tabla")?;
    let turn = angle.wrapping_sub(offset);
    let push_angle = offset.wrapping_add(own_heading);
    let along = table_sine(turn.wrapping_add(0x40));
    if along > 0 {
        let along = along >> 1;
        let (sh, mul): (u32, bool) = if direct {
            (scale.1, first_side)
        } else {
            (scale.0, first_side)
        };
        let (ps, pc) = sin_cos(push_angle);
        let (dx, dy) = if mul {
            (
                ((ps >> 1).wrapping_mul(along)) >> sh,
                ((pc >> 1).wrapping_mul(along)) >> sh,
            )
        } else {
            (ps.wrapping_mul(along) >> sh, pc.wrapping_mul(along) >> sh)
        };
        let ship = &mut ships[id];
        let x = r32(ship, 8).wrapping_add(dx);
        w32(ship, 8, x);
        let y = r32(ship, 0xC).wrapping_sub(dy);
        w32(ship, 0xC, y);
    }
    let mut torque = table_sine(turn);
    if edge == 0 {
        if torque < 0 {
            torque = 0;
        }
    } else if edge == 1 && torque > 0 {
        torque = 0;
    }
    let spin = (if direct { torque >> 4 } else { torque >> 6 }) as i16;
    let heading16 = r16(&ships[id], 0x10);
    if rel_y < 0 {
        let spread = SPREAD[kind];
        let mut divisor = MASS[kind].wrapping_mul(spread);
        divisor = if direct {
            divisor.wrapping_mul(2)
        } else {
            divisor.wrapping_mul(8)
        };
        let ship = &mut ships[id];
        let speed = r32(ship, 0x1C);
        let moved = div(
            spread.wrapping_sub(rel_x).wrapping_mul(speed).wrapping_mul(rel_y),
            divisor,
        )?
        .wrapping_add(speed);
        w32(ship, 0x1C, moved);
        let x = r32(ship, 8).wrapping_sub(r32(ship, 0x14));
        w32(ship, 8, x);
        let y = r32(ship, 0xC).wrapping_sub(r32(ship, 0x18));
        w32(ship, 0xC, y);
    }
    Ok(Impact {
        spin,
        heading: heading16 as u8,
        cosine: along,
    })
}

/// 0x00608F70: choque de los cascos de `a` y `b` (con `a < b`). `view_active` activa la rama de sonido, no modelada.
pub fn hull_collision(
    battle: &[u8],
    ships: &mut [Vec<u8>],
    worlds: &[u8],
    a: u16,
    b: u16,
    view_active: bool,
) -> Result<bool, E> {
    if b <= a {
        return Ok(false);
    }
    let (ia, ib) = (usize::from(a), usize::from(b));
    let (ka, kb) = (
        world_kind(ship_at(ships, ia)?, worlds)?,
        world_kind(ship_at(ships, ib)?, worlds)?,
    );
    let dx = x_of(&ships[ia]) - x_of(&ships[ib]);
    let dy = y_of(&ships[ia]) - y_of(&ships[ib]);
    let d2 = dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx));
    if d2 < 100 {
        let root = ftol_i32(f64::from(d2).sqrt());
        if root != 0 {
            let ny = div((11 - root).wrapping_mul(dy), root)?;
            let nx = div((11 - root).wrapping_mul(dx), root)?;
            let (nx, ny) = (nx as i16, ny as i16);
            for (id, sign) in [(ia, -1i16), (ib, 1i16)] {
                let ship = &mut ships[id];
                let x = (r16(ship, 0xA) as i16).wrapping_add(nx.wrapping_mul(sign));
                let y = (r16(ship, 0xE) as i16).wrapping_add(ny.wrapping_mul(sign));
                w16(ship, 0xA, x as u16);
                w16(ship, 0xE, y as u16);
            }
            return Ok(true);
        }
    }
    let Some(contact) = hull_contact(ships, worlds, ia, ib)? else {
        return Ok(false);
    };
    let pos_b = (x_of(&ships[ib]), y_of(&ships[ib]));
    let pos_a = (x_of(&ships[ia]), y_of(&ships[ia]));
    let (on_a, edge_a) = ray_hull(ships, worlds, ia, pos_b, contact)?;
    let (on_b, edge_b) = ray_hull(ships, worlds, ib, pos_a, contact)?;
    if view_active {
        return Err("rama de sonido de 0x00608F70 no modelada");
    }
    let (edge_a, edge_b) = (
        edge_a.ok_or("rayo sin corte: pila sin inicializar")?,
        edge_b.ok_or("rayo sin corte: pila sin inicializar")?,
    );
    let direct = battle[0xAC8] != 0;
    let first = impact_side(ships, ia, ka, on_a, edge_a, direct, (13, 11), true)?;
    let second = impact_side(ships, ib, kb, on_b, edge_b, direct, (14, 12), false)?;
    let _ = (first.heading, first.cosine, second.heading, second.cosine);
    let h = r16(&ships[ia], 0x10);
    w16(&mut ships[ia], 0x10, first.spin.wrapping_add(h as i16) as u16);
    let h = r16(&ships[ib], 0x10);
    w16(&mut ships[ib], 0x10, second.spin.wrapping_add(h as i16) as u16);
    Ok(true)
}

/// Contexto para las consultas de abordaje.
pub struct BoardCtx<'a> {
    pub captains: Captains<'a>,
    pub virtual_10: bool,
}

fn with_boarding<R>(battle: &[u8], worlds: &[u8], ctx: &BoardCtx, f: impl FnOnce(&Boarding) -> R) -> R {
    let lookup = |index: u32| -> &[u8] {
        let start = index as usize * 0x180;
        &worlds[start..start + 0x180]
    };
    let boarding = Boarding {
        battle,
        virtual_10: ctx.virtual_10,
        world: &lookup,
        captains: ctx.captains,
    };
    f(&boarding)
}

/// 0x00609BA0: tras el choque, empieza el abordaje entre `a` y `b` si procede.
pub fn boarding_contact(
    battle: &[u8],
    ships: &mut [Vec<u8>],
    worlds: &[u8],
    ctx: &BoardCtx,
    a: u16,
    b: u16,
    view_active: bool,
) -> Result<(), E> {
    let (ia, ib) = (usize::from(a), usize::from(b));
    let collided = hull_collision(battle, ships, worlds, a, b, view_active)?;
    if !collided {
        if ship_at(ships, ia)?[0x12A] & 0x20 == 0 && ship_at(ships, ib)?[0x12A] & 0x20 == 0 {
            return Ok(());
        }
        if battle[0xAC8] == 0 {
            return Ok(());
        }
    }
    if battle[0x5C] != 0 {
        return Ok(());
    }
    let (sa, sb) = (ship_at(ships, ia)?.clone(), ship_at(ships, ib)?.clone());
    let world_of = |ship: &[u8]| -> Result<usize, E> {
        let i = usize::try_from(r32(ship, 4)).map_err(|_| "barco mundial fuera de rango")?;
        if (i + 1) * 0x180 > worlds.len() {
            return Err("barco mundial fuera de rango");
        }
        Ok(i * 0x180)
    };
    let (wa, wb) = (world_of(&sa)?, world_of(&sb)?);
    let mut flagged = false;
    if with_boarding(battle, worlds, ctx, |bd| bd.board_filter(&sa, &sb, false))? {
        let capture = with_boarding(battle, worlds, ctx, |bd| bd.can_capture(&sa, &sb, false));
        if capture || r32(worlds, wb + 0x118) != 0 || sb[0x12A] & 0x40 == 0 {
            flagged = true;
        }
    }
    let second = with_boarding(battle, worlds, ctx, |bd| bd.board_filter(&sb, &sa, false))?;
    if second {
        let capture = with_boarding(battle, worlds, ctx, |bd| bd.can_capture(&sb, &sa, false));
        if (!capture && r32(worlds, wa + 0x118) == 0 && sa[0x12A] & 0x40 != 0) && !flagged {
            return Ok(());
        }
    } else if !flagged {
        return Ok(());
    }
    ships[ia][0x12A] |= 8;
    ships[ib][0x12A] |= 8;
    w16(&mut ships[ia], 0x128, b);
    w16(&mut ships[ib], 0x128, a);
    ships[ia][0x14A] = 0;
    ships[ib][0x14A] = 1;
    Ok(())
}

/// Cajas del índice espacial y tablas por hueco que leen las consultas de obstáculos.
pub struct Slots<'a> {
    /// `[mínimo, máximo]` en x e y de cada hueco, alternados (`battle+0x28` y `battle+0x38`).
    pub x: &'a [i16],
    pub y: &'a [i16],
    /// Tipo (`battle+0x18`) y referencia (`battle+0x1C`) de cada hueco.
    pub kind: &'a [u8],
    pub reference: &'a [u16],
}

impl Slots<'_> {
    fn bx(&self, slot: u16) -> Result<(i32, i32), E> {
        let at = usize::from(slot) * 2;
        match self.x.get(at..at + 2) {
            Some(v) => Ok((i32::from(v[0]), i32::from(v[1]))),
            None => Err("hueco espacial ausente"),
        }
    }
    fn by(&self, slot: u16) -> Result<(i32, i32), E> {
        let at = usize::from(slot) * 2;
        match self.y.get(at..at + 2) {
            Some(v) => Ok((i32::from(v[0]), i32::from(v[1]))),
            None => Err("hueco espacial ausente"),
        }
    }
}

/// 0x0060B357: el barco `id` toca el obstáculo del hueco `slot`; devuelve la dirección de la arista tocada
/// (`Some(Some(dir))`), `Some(None)` si el casco queda dentro de la zona, o `None` sin contacto.
pub fn obstacle_contact(
    battle: &[u8],
    slots: &Slots,
    ships: &[Vec<u8>],
    worlds: &[u8],
    slot: u16,
    id: u16,
) -> Result<Option<Option<u8>>, E> {
    let reference = usize::from(*slots.reference.get(usize::from(slot)).ok_or("hueco espacial ausente")?);
    let base = if slots
        .kind
        .get(usize::from(slot))
        .copied()
        .ok_or("hueco espacial ausente")?
        == 2
    {
        0x876
    } else {
        0x676
    };
    let (width_a, width_b) = (
        i32::from(r16(battle, base + reference * 8) as i16),
        i32::from(r16(battle, base + 2 + reference * 8) as i16),
    );
    let (x0, x1) = slots.bx(slot)?;
    let (y0, y1) = slots.by(slot)?;
    let sum = width_a + width_b;
    let low = |from: i32| -> Result<i32, E> {
        Ok(i32::from(
            (y0 as i16).wrapping_add(div((y1 - y0).wrapping_mul(from), sum)? as i16),
        ))
    };
    let left_y = low(width_b)?; // sVar15
    let right_y = low(width_a)?; // sVar16
    let left_x = i32::from((x0 as i16).wrapping_add(8)); // sVar12
    let right_x = i32::from((x1 as i16).wrapping_sub(8)); // sVar17
    let ship = ship_at(ships, usize::from(id))?;
    let (sx, sy) = (x_of(ship), y_of(ship));
    if !(sx - sy < left_x - left_y
        || sx + sy < left_x + left_y
        || right_x + right_y < sx + sy
        || right_x - right_y < sx - sy)
    {
        return Ok(Some(None));
    }
    let kind = world_kind(ship, worlds)?;
    let (s, c) = sin_cos(ship[0x11]);
    let r = (s, c);
    let mut start = (0i32, 0i32);
    let mut edge = {
        let (x, y) = rotate(r, hull_point(kind, 4));
        (x.wrapping_add(sx), y.wrapping_add(sy))
    };
    let top = i32::from((y0 as i16).wrapping_add(8));
    let bottom = i32::from((y1 as i16).wrapping_sub(8));
    let upper_x = i32::from((x0 as i16).wrapping_add(div((x1 - x0).wrapping_mul(width_b), sum)? as i16));
    let lower_x = i32::from((x0 as i16).wrapping_add(div((x1 - x0).wrapping_mul(width_a), sum)? as i16));
    let _ = (upper_x, lower_x);
    for i in 0..5usize {
        start = (start.0.wrapping_add(edge.0), start.1.wrapping_add(edge.1));
        let (hx, hy) = rotate(r, hull_point(kind, i));
        edge = (
            hx.wrapping_add(sx).wrapping_sub(start.0),
            hy.wrapping_add(sy).wrapping_sub(start.1),
        );
        let mut vertex = (left_x - start.0, left_y - start.1);
        for k in 0..4u8 {
            let previous = vertex;
            vertex = match k {
                0 => (
                    i32::from((x0 as i16).wrapping_add(div((x1 - x0).wrapping_mul(width_b), sum)? as i16)) - start.0,
                    top - start.1,
                ),
                1 => (right_x - start.0, right_y - start.1),
                2 => (
                    i32::from((x0 as i16).wrapping_add(div((x1 - x0).wrapping_mul(width_a), sum)? as i16)) - start.0,
                    bottom - start.1,
                ),
                _ => (left_x - start.0, left_y - start.1),
            };
            let mut first = edge
                .0
                .wrapping_mul(previous.1)
                .wrapping_sub(edge.1.wrapping_mul(previous.0));
            let mut second = edge
                .0
                .wrapping_mul(previous.1.wrapping_sub(vertex.1))
                .wrapping_sub(edge.1.wrapping_mul(previous.0.wrapping_sub(vertex.0)));
            if second < 0 {
                first = first.wrapping_neg();
                second = second.wrapping_neg();
            }
            if first <= second && first >= 0 {
                let mut third = previous
                    .0
                    .wrapping_mul(vertex.1.wrapping_sub(previous.1))
                    .wrapping_sub(previous.1.wrapping_mul(vertex.0.wrapping_sub(previous.0)));
                let mut fourth = edge
                    .0
                    .wrapping_mul(vertex.1.wrapping_sub(previous.1))
                    .wrapping_sub(edge.1.wrapping_mul(vertex.0.wrapping_sub(previous.0)));
                if fourth < 0 {
                    third = third.wrapping_neg();
                    fourth = fourth.wrapping_neg();
                }
                if third <= fourth && third >= 0 {
                    return Ok(Some(Some(k.wrapping_mul(64).wrapping_sub(0x20))));
                }
            }
        }
    }
    Ok(None)
}

/// 0x0060BC5A: ¿está el barco dentro del radio de la caja del hueco?
pub fn near_obstacle(slots: &Slots, ships: &[Vec<u8>], slot: u16, id: u16) -> Result<bool, E> {
    let ship = ship_at(ships, usize::from(id))?;
    let (x0, x1) = slots.bx(slot)?;
    let (y0, y1) = slots.by(slot)?;
    let dx = i32::from((((x1 + x0) >> 1) as i16).wrapping_sub(r16(ship, 0xA) as i16));
    let dy = i32::from((((y1 + y0) >> 1) as i16).wrapping_sub(r16(ship, 0xE) as i16));
    let radius = i32::from(((x1 - x0) >> 1) as i16);
    Ok(dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy)) < radius.wrapping_mul(radius))
}

fn arctan_unit(ratio: i32) -> Result<u16, E> {
    // Cociente en punto fijo 16.16 -> ángulo de 0 a 0x40 (tabla 0x0067ACB8), como los tramos de 0x0060BCDA
    let table = |index: i32| -> Result<u8, E> {
        ATAN.get(usize::try_from(index).map_err(|_| "índice de arcotangente")?)
            .copied()
            .ok_or("índice de arcotangente")
    };
    if ratio < 0 {
        let value = if ratio == -0x10000 || -ratio < 0x10000 {
            u16::from(table((0x200 - ratio) >> 10)?)
        } else {
            0x40u16.wrapping_sub(u16::from(table(
                (0x4000_0000i64 / i64::from(-ratio)) as i32 + 0x80 >> 8,
            )?))
        };
        Ok(0x100u16.wrapping_sub(value))
    } else if ratio < 0x10001 {
        Ok(u16::from(table((ratio + 0x200) >> 10)?))
    } else {
        Ok(0x40u16.wrapping_sub(u16::from(table(
            (0x4000_0000i64 / i64::from(ratio)) as i32 + 0x80 >> 8,
        )?)))
    }
}

/// 0x0060BCDA: levanta holguras (`ship+0x27`) alrededor de la dirección del obstáculo del hueco `slot`.
pub fn mark_obstacle(
    slots: &Slots,
    ships: &mut [Vec<u8>],
    slot: u16,
    id: u16,
    margin: i32,
    reach: i32,
    strength: u8,
) -> Result<(), E> {
    let (x0, x1) = slots.bx(slot)?;
    let (y0, y1) = slots.by(slot)?;
    let ship = ship_at(ships, usize::from(id))?;
    let radius = ((x1 - x0) >> 1).wrapping_add(margin).wrapping_sub(reach);
    let dx = ((x1 + x0) >> 1).wrapping_sub(x_of(ship));
    let dy = ((y1 + y0) >> 1).wrapping_sub(y_of(ship));
    let direction = heading(dx, dy);
    let d2 = dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy));
    if d2 == 0 {
        return Ok(());
    }
    let distance = ftol_i32(f64::from(d2).sqrt());
    let kind = slots
        .kind
        .get(usize::from(slot))
        .copied()
        .ok_or("hueco espacial ausente")?;
    if strength != 0 && distance > radius.wrapping_mul(2) && kind == 6 {
        return Ok(());
    }
    let spread: u16 = if radius < distance {
        let root = ftol_i32(f64::from(d2.wrapping_sub(radius.wrapping_mul(radius))).sqrt());
        arctan_unit(div(radius.wrapping_mul(0x10000), root)?)?
    } else {
        let value = ((radius as i16).wrapping_add(0x40)).wrapping_sub(distance as i16);
        if value > 0x80 {
            0x80
        } else {
            value as u16
        }
    };
    let mut strength = strength;
    if strength == 0 {
        strength = match kind {
            4 => 0x40,
            5 => 0x20,
            6 => 0x80,
            _ => 0,
        };
    }
    let limit = spread as i16;
    let ship = &mut ships[usize::from(id)];
    let (mut acc, mut level) = (0u16, strength);
    let mut step = 0u8;
    while i32::from(step) <= i32::from(limit) {
        acc = acc.wrapping_add(u16::from(strength));
        let mut guard = 0;
        while i32::from(limit) < i32::from(acc) {
            level = level.wrapping_sub(1);
            acc = acc.wrapping_sub(spread);
            guard += 1;
            if guard > 70_000 {
                return Err("bucle sin fin del original");
            }
        }
        for index in [direction.wrapping_add(step), direction.wrapping_sub(step)] {
            let cell = &mut ship[0x27 + usize::from(index)];
            if *cell < level {
                *cell = level;
            }
        }
        step = step.wrapping_add(1);
        if step == 0 {
            return Err("bucle sin fin del original");
        }
    }
    Ok(())
}

/// Efectos externos del despachador: prueba de casco del proyectil (0x0060A73C), impacto (0x00603470) y daño (0x0061F96F).
pub trait PairHost {
    fn hit_mask(
        &mut self,
        battle: &mut [u8],
        ships: &mut [Vec<u8>],
        worlds: &mut [u8],
        projectile: u16,
        ship: u16,
    ) -> u8;
    fn impact(
        &mut self,
        battle: &mut [u8],
        ships: &mut [Vec<u8>],
        worlds: &mut [u8],
        projectile: u16,
        ship: u16,
        mask: u8,
    );
    fn damage(&mut self, battle: &mut [u8], ships: &mut [Vec<u8>], worlds: &mut [u8], ship: u16, amount: i32);
}

fn aggressive(battle: &[u8], ship: &[u8]) -> bool {
    r16(ship, 0x128) != 0xFFFF
        || battle[0x5D + usize::from(ship[0x13D])] != 0
        || battle[0x5C] != 0
        || ship[0x12A] & 1 != 0
}

/// 0x00608850: pareja de huecos del índice espacial; `a` debe ser un barco y `b` cualquier tipo.
#[allow(clippy::too_many_arguments)]
pub fn dispatch_pair<H: PairHost + ?Sized>(
    battle: &mut [u8],
    slots: &Slots,
    ships: &mut [Vec<u8>],
    worlds: &mut [u8],
    ctx: &BoardCtx,
    a: u16,
    b: u16,
    view_active: bool,
    host: &mut H,
) -> Result<(), E> {
    let kind = |slot: u16| {
        slots
            .kind
            .get(usize::from(slot))
            .copied()
            .ok_or("hueco espacial ausente")
    };
    let id_of = |slot: u16| {
        slots
            .reference
            .get(usize::from(slot))
            .copied()
            .ok_or("hueco espacial ausente")
    };
    if kind(a)? != 0 {
        return Ok(());
    }
    let ref_a = id_of(a)?;
    let flags_a = ship_at(ships, usize::from(ref_a))?[0x12A];
    if flags_a & 2 != 0 {
        return Ok(());
    }
    let side_map = |battle: &[u8], ship: &[u8]| battle[0x559 + usize::from(ship[0x13D])];
    match kind(b)? {
        0 => {
            let ref_b = id_of(b)?;
            let (sa, sb) = (ship_at(ships, usize::from(ref_a))?, ship_at(ships, usize::from(ref_b))?);
            if side_map(battle, sa) == side_map(battle, sb) {
                hull_collision(battle, ships, worlds, ref_a, ref_b, view_active)?;
            } else if ref_a < ref_b {
                if (sb[0x12A] | flags_a) & 8 != 0 {
                    hull_collision(battle, ships, worlds, ref_a, ref_b, view_active)?;
                } else {
                    boarding_contact(battle, ships, worlds, ctx, ref_a, ref_b, view_active)?;
                }
            }
        }
        1 => {
            let projectile = id_of(b)?;
            let mask = host.hit_mask(battle, ships, worlds, projectile, ref_a);
            if mask != 0 {
                host.impact(battle, ships, worlds, projectile, ref_a, mask);
            }
        }
        2 => {
            let Some(found) = obstacle_contact(battle, slots, ships, worlds, b, ref_a)? else {
                return Ok(());
            };
            let direction = found.unwrap_or(flags_a);
            let speed = r32(&ships[usize::from(ref_a)], 0x1C);
            host.damage(battle, ships, worlds, ref_a, speed >> 10);
            let direct = battle[0xAC8] != 0;
            let shift = if direct { 10 } else { 12 };
            let ship = &mut ships[usize::from(ref_a)];
            let (sin, cos) = sin_cos(direction);
            let speed = r32(ship, 0x1C);
            let x = r32(ship, 8).wrapping_add(((speed >> 4).wrapping_mul(sin)) >> shift);
            w32(ship, 8, x);
            let dy = ((speed >> 4).wrapping_mul(cos)) >> shift;
            let y = r32(ship, 0xC).wrapping_sub(dy);
            w32(ship, 0xC, y);
            let delta = ship[0x11].wrapping_sub(direction).wrapping_add(0x80);
            let step: u8 = if direct { 8 } else { 2 };
            if delta != 0 && delta <= 0x3F {
                ship[0x11] = ship[0x11].wrapping_add(step);
            } else if delta > 0xC0 {
                ship[0x11] = ship[0x11].wrapping_sub(step);
            }
            let speed = r32(ship, 0x1C);
            w32(ship, 0x1C, speed - (speed >> if direct { 4 } else { 6 }));
            if view_active {
                return Err("rama visual de 0x00608850 (sonido y polvo) no modelada");
            }
        }
        3 => {
            if obstacle_contact(battle, slots, ships, worlds, b, ref_a)?.is_some() {
                let direct = battle[0xAC8] != 0;
                let ship = &mut ships[usize::from(ref_a)];
                let speed = r32(ship, 0x1C);
                w32(ship, 0x1C, speed - (speed >> if direct { 4 } else { 6 }));
                if view_active {
                    return Err("rama visual de 0x00608850 (sonido y polvo) no modelada");
                }
            }
        }
        4 | 5 => {
            if aggressive(battle, ship_at(ships, usize::from(ref_a))?) && near_obstacle(slots, ships, b, ref_a)? {
                mark_obstacle(slots, ships, b, ref_a, 0x28, 200, 0)?;
            }
        }
        6 => {
            let ref_b = id_of(b)?;
            if ref_a == ref_b {
                return Ok(());
            }
            let sa = ship_at(ships, usize::from(ref_a))?.clone();
            if !aggressive(battle, &sa) {
                return Ok(());
            }
            let sb = ship_at(ships, usize::from(ref_b))?.clone();
            let same = side_map(battle, &sa) == side_map(battle, &sb);
            if !same && with_boarding(battle, worlds, ctx, |bd| bd.board_filter(&sa, &sb, false))? {
                return Ok(());
            }
            if !near_obstacle(slots, ships, b, ref_a)? {
                return Ok(());
            }
            if same {
                mark_obstacle(slots, ships, b, ref_a, 0x28, 0x168, 0x20)?;
            } else {
                mark_obstacle(slots, ships, b, ref_a, 0xB4, 0x168, 0)?;
            }
        }
        _ => {}
    }
    Ok(())
}
