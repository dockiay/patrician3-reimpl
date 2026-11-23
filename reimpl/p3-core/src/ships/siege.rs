//! Barco de asedio (clase con vtable 0x0067AAD0): selección de objetivo (0x006247B0, vtable +4) y rejilla de 64×64 en
//! `ship+0x164` (0x00625B00 busca la celda ocupada más cercana; 0x005FFF40 convierte coordenadas del mapa en celda).
//! Fixtures: `oracle/x86_siege_pick.py`.

use super::boarding::{artillery_strength, BoardingHost};
use super::projectile::heading;
use super::steer::turn;
use super::step::{dec, div, peer, target_weight, world_slot, x_of, y_of, Env, StepEvent, StepOutcome};
use super::{r16, r32, w16, w32};
use crate::numeric::ftol_i32;
use crate::rng::sea_battle_next;

type E = &'static str;

/// Torres de la defensa (`battle+0x1EC4`, cuatro enteros): suman `0xC80000 / valor` al sorteo cuando el bando está en el mapa 0.
pub const TOWER_SLOTS: usize = 0x1EC4;

/// 0x006247B0: como la del barco de mar, pero en el mapa 0 las cuatro torres compiten con los barcos. Si el sorteo cae en una
/// torre (o no hay barcos) el objetivo es `-2 - índice` de la primera torre viva (índice 4 si ninguna lo está).
pub fn pick_target(
    ship: &mut [u8],
    me: usize,
    battle: &mut [u8],
    ships: &[Vec<u8>],
    worlds: &[u8],
    next: &[u16],
) -> Result<(), E> {
    if ship[0x161] != 0 {
        return Ok(());
    }
    let side = usize::from(ship[0x13D]);
    let map0 = battle[0x559 + side] == 0;
    let head = if map0 { r16(battle, 0x65C) } else { r16(battle, 0x10) };
    let weight_of = |id: u16| -> Result<Option<i32>, E> {
        let s = if usize::from(id) == me {
            &*ship
        } else {
            peer(ships, usize::from(id))?
        };
        if s[0x12A] & 6 != 0 {
            return Ok(None);
        }
        let range = world_slot(worlds, r32(s, 4))?;
        Ok(Some(target_weight(s, &worlds[range])))
    };
    let mut total: i32 = 0;
    let mut id = head;
    while id != 0xFFFF {
        if let Some(w) = weight_of(id)? {
            total = total.wrapping_add(w);
        }
        id = *next.get(usize::from(id)).ok_or("lista de barcos truncada")?;
    }
    if map0 {
        for k in 0..4 {
            let tower = r32(battle, TOWER_SLOTS + 4 * k) as i32;
            if tower > 0 {
                total = total.wrapping_add((0xC8_0000i64 / i64::from(tower)) as i32);
            }
        }
    }
    if total > 0 {
        let mut seed = u32::from_le_bytes(battle[0xADC..0xAE0].try_into().unwrap());
        let value = sea_battle_next(&mut seed);
        battle[0xADC..0xAE0].copy_from_slice(&seed.to_le_bytes());
        total = (value % (total as u32)).wrapping_add(1) as i32;
    }
    let mut chosen = head;
    while chosen != 0xFFFF {
        if let Some(w) = weight_of(chosen)? {
            total = total.wrapping_sub(w);
            if total < 1 {
                break;
            }
        }
        chosen = *next.get(usize::from(chosen)).ok_or("lista de barcos truncada")?;
    }
    if chosen == 0xFFFF && map0 {
        let first = (0..4)
            .find(|&k| (r32(battle, TOWER_SLOTS + 4 * k) as i32) > 0)
            .unwrap_or(4) as u16;
        chosen = 0xFFFEu16.wrapping_sub(first);
    }
    w16(ship, 0x128, chosen);
    Ok(())
}

/// Inicio de la rejilla de 64×64 bytes del barco de asedio (`ship+0x164`, índice `x*64 + y`).
pub const GRID: usize = 0x164;
/// Tabla de celdas transitables de la batalla (`battle+0x1EE0`, 4.096 bytes).
pub const PASSABLE: usize = 0x1EE0;
const SIDE: i32 = 0x40;

/// 0x005FFF40: celda de la rejilla (cada eje se trunca a 16 bits y se limita a 63) para unas coordenadas del mapa.
pub fn cell_of(x: u32, y: u32) -> (u16, u16) {
    let cx = ((x << 6) / 0x1100) as u16;
    let cy = (y.wrapping_mul(0x40) / 0xF02) as u16;
    (cx.min(0x3F), cy.min(0x3F))
}

/// 0x00625B00: desplaza (x, y) a la celda no nula más cercana recorriendo anillos en rombo alrededor de ella.
pub fn nearest_cell(ship: &[u8], x: &mut u16, y: &mut u16) -> Result<(), E> {
    let (mut dx, mut dy) = (0i32, 0i32);
    for _ in 0..40_000 {
        let (a, b) = (i32::from(*x) + dx, i32::from(*y) + dy);
        if (0..SIDE).contains(&a) && (0..SIDE).contains(&b) && ship[GRID + (a * SIDE + b) as usize] != 0 {
            *x = a as u16;
            *y = b as u16;
            return Ok(());
        }
        if dx == 0 {
            if dy < 1 {
                dx = 1;
            } else {
                dx -= 1;
                dy -= 1;
            }
        } else if dx > 0 {
            if dy < 0 {
                dx += 1;
                dy += 1;
            } else {
                dx -= 1;
                dy += 1;
            }
        } else if dy > 0 {
            dx -= 1;
            dy -= 1;
        } else {
            dx += 1;
            dy -= 1;
        }
    }
    Err("rejilla sin celdas transitables (el original no termina)")
}

/// Vecinos de la inundación: cuatro ortogonales (coste 2) y cuatro diagonales (coste 3).
const DIRS: [(i32, i32); 8] = [(0, -1), (1, 0), (0, 1), (-1, 0), (1, -1), (1, 1), (-1, 1), (-1, -1)];

/// 0x006258A0: rellena la rejilla de `ship+0x164` con la distancia (en bytes, desde 1) a la celda más cercana a (wx, wy). Las
/// celdas no transitables quedan a 0 y las no alcanzadas a 0xFF. El orden de las pilas importa: la suma de bytes puede dar la
/// vuelta, y el original procesa cada lista desde su última inserción.
pub fn flood(ship: &mut [u8], battle: &[u8], wx: u16, wy: u16) -> Result<(), E> {
    for i in 0..0x1000 {
        ship[GRID + i] = if battle[PASSABLE + i] != 0 { 0xFF } else { 0 };
    }
    let (mut x, mut y) = cell_of(u32::from(wx), u32::from(wy));
    nearest_cell(ship, &mut x, &mut y)?;
    ship[GRID + usize::from(x) * 64 + usize::from(y)] = 1;
    let mut current: Vec<(u16, u16)> = vec![(x, y)];
    // pendientes a 2 pasos (`local_4c`), a 1 (`local_48[0]`) y a 0 (`local_48[1]`)
    let mut queues: [Vec<(u16, u16)>; 3] = [vec![], vec![], vec![]];
    loop {
        while let Some((cx, cy)) = current.pop() {
            for (n, &(dx, dy)) in DIRS.iter().enumerate() {
                let (nx, ny) = (dx + i32::from(cx), dy + i32::from(cy));
                if !(0..SIDE).contains(&nx) || !(0..SIDE).contains(&ny) {
                    continue;
                }
                let here = ship[GRID + usize::from(cx) * 64 + usize::from(cy)];
                let value = here.wrapping_add(if n < 4 { 0 } else { 1 }).wrapping_add(2);
                let at = GRID + (nx * SIDE + ny) as usize;
                if value < ship[at] {
                    ship[at] = value;
                    queues[if n < 4 { 1 } else { 2 }].push((nx as u16, ny as u16));
                }
            }
        }
        let mut found = false;
        for _ in 0..3 {
            let [two, one, zero] = std::mem::take(&mut queues);
            current = two;
            queues = [one, zero, vec![]];
            if !current.is_empty() {
                found = true;
                break;
            }
        }
        if !found {
            return Ok(());
        }
    }
}

/// Desplazamientos (x, y) de la rejilla para las ocho direcciones cercanas (de 0x20 en 0x20, empezando por `y - 1`).
const NEAR: [(i32, i32); 8] = [(0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1)];
/// Las dieciséis lejanas (de 0x10 en 0x10, empezando por `y - 2`).
const FAR: [(i32, i32); 16] = [
    (0, -2),
    (1, -2),
    (2, -2),
    (2, -1),
    (2, 0),
    (2, 1),
    (2, 2),
    (1, 2),
    (0, 2),
    (-1, 2),
    (-2, 2),
    (-2, 1),
    (-2, 0),
    (-2, -1),
    (-2, -2),
    (-1, -2),
];
/// Mapa de preferencias de rumbo del barco (`ship+0x27`, 256 direcciones).
pub const COURSE: usize = 0x27;

fn raise(ship: &mut [u8], first: i32, up: i32, down: i32, count: i32, floor: u8) {
    for i in 0..count {
        for at in [
            first.wrapping_add(up).wrapping_add(i),
            first.wrapping_add(down).wrapping_sub(i),
        ] {
            let cell = &mut ship[COURSE + (at & 0xFF) as usize];
            if *cell < floor {
                *cell = floor;
            }
        }
    }
}

/// 0x00625BF0: marca en `ship+0x27` los rumbos bloqueados o desaconsejados según los muros que rodean al barco en la rejilla.
/// Si el destino (`+0x20/+0x22`) cambió de celda, recalcula antes la inundación de distancias.
pub fn course_field(ship: &mut [u8], battle: &[u8]) -> Result<(), E> {
    let word = |s: &[u8], at: usize| i32::from(r16(s, at) as i16);
    if (ship[0x12A] & 1 == 0 && r16(ship, 0x20) != 0xFFFF) || r16(ship, 0x1164) == 0xFFFF {
        let (tx, ty) = cell_of(word(ship, 0x20) as u32, word(ship, 0x22) as u32);
        if tx != r16(ship, 0x1164) || ty != r16(ship, 0x1166) {
            w16(ship, 0x1166, ty);
            w16(ship, 0x1164, tx);
            flood(ship, battle, r16(ship, 0x20), r16(ship, 0x22))?;
        }
    }
    let (cx, cy) = cell_of(word(ship, 0xA) as u32, word(ship, 0xE) as u32);
    let at = |ship: &[u8], x: i32, y: i32| -> Option<u8> {
        if (0..SIDE).contains(&x) && (0..SIDE).contains(&y) {
            Some(ship[GRID + (x * SIDE + y) as usize])
        } else {
            None
        }
    };
    let here = match at(ship, i32::from(cx), i32::from(cy)) {
        Some(v) if v != 0 => v,
        _ => 0xFF,
    };
    let aiming = ship[0x12A] & 1 == 0 && r16(ship, 0x20) != 0xFFFF;
    for (k, &(dx, dy)) in NEAR.iter().enumerate() {
        let angle = (k as i32) * 0x20;
        let value = match at(ship, i32::from(cx) + dx, i32::from(cy) + dy) {
            Some(0) => {
                for i in 0..0x20 {
                    ship[COURSE + ((angle + i) & 0xFF) as usize] = 0xC0;
                    ship[COURSE + ((angle - i) & 0xFF) as usize] = 0xC0;
                }
                continue;
            }
            Some(v) => v,
            None => 0xFF,
        };
        if aiming {
            if value >= here {
                raise(ship, angle, 0, 0, 0x11, 0x40);
            }
            if i32::from(value) == i32::from(here) - 1 {
                raise(ship, angle, 0, 0, 0x11, 0x10);
            }
        }
    }
    for (k, &(dx, dy)) in FAR.iter().enumerate() {
        let angle = (k as i32) * 0x10;
        if at(ship, i32::from(cx) + dx, i32::from(cy) + dy) == Some(0) {
            raise(ship, angle, 0, 0, 0x10, 0x40);
            raise(ship, angle, 0x10, -0x10, 0x30, 0x10);
        }
    }
    Ok(())
}

/// Cajas de colisión por tipo de barco (`L"7BFT"`), comunes con el barco de mar.
impl<H: BoardingHost> Env<'_, H> {
    /// 0x00621770 (`SiegeShip::update`, vtable 0x0067AAD0 +0). `limit` es el dato global `[0x006DE4A0+0x10]` que decide
    /// si un barco sin bando (0xFF) puede salir del mapa.
    pub(super) fn run_siege(&mut self, ship: &mut [u8], limit: u32) -> Result<StepOutcome, E> {
        if self.battle.len() < 0x1EE0 || ship.len() < 0x1170 {
            return Err("batalla o barco de asedio truncados");
        }
        let probe = ship[0x11].wrapping_add(0x10);
        if ship[0x12A] & 4 != 0 && probe & 0x18 != 0 {
            ship[0x11] = if probe & 0x1F < 0x10 {
                ship[0x11].wrapping_sub(1)
            } else {
                ship[0x11].wrapping_add(1)
            };
        }
        if ship[0x12A] & 4 != 0 && r32(ship, 0x1C) != 0 {
            if r32(ship, 0x15C) != 0 {
                let speed = r32(ship, 0x1C);
                let (floor, drop) = if self.battle[0xAC8] == 0 {
                    (0x17A, 0x179)
                } else {
                    (0x1791, 0x1790)
                };
                if speed < floor {
                    w32(ship, 0x1C, 0);
                    return Ok(StepOutcome::Done);
                }
                w32(ship, 0x1C, speed - drop);
            }
            w16(ship, 0x20, 0xFFFF);
            let level = ship[0x26];
            ship[0x26] = 0;
            self.do_steer(ship, false)?;
            ship[0x26] = level;
        }
        if ship[0x12A] & 6 != 0 {
            return Ok(StepOutcome::Done);
        }
        dec(ship, 0x149);
        let (x, y) = (x_of(ship), y_of(ship));
        if x < 10 || y < 10 || x > 0x10F6 || y > 0xEF8 {
            if ship[0x13D] != 0xFF || u32::from(self.battle[0x1E98]) < limit {
                ship[0x12A] |= 2;
                self.battle[0xAC4] = 0;
                let id = r16(ship, 0x138);
                w16(self.battle, 0xAC6, id);
                if r32(ship, 0x134) != 0 {
                    self.ev.push(StepEvent::Release(r32(ship, 0x134)));
                }
                w32(ship, 0x134, 0);
                let near = *self
                    .ctx
                    .slots_near
                    .get(usize::from(id))
                    .ok_or("hueco espacial ausente")?;
                let far = *self
                    .ctx
                    .slots_far
                    .get(usize::from(id))
                    .ok_or("hueco espacial ausente")?;
                self.ev.push(StepEvent::SpatialRemove(near));
                self.ev.push(StepEvent::SpatialRemove(far));
            }
            return Ok(StepOutcome::Done);
        }
        dec(ship, 0x130);
        dec(ship, 0x131);
        dec(ship, 0x161);
        if self.battle[0xAC8] != 0 {
            for at in [0x130, 0x131, 0x161] {
                ship[at] = if ship[at] < 0x10 { 0 } else { ship[at] - 0xF };
            }
        }
        if ship[0x12A] & 8 != 0 {
            let target = usize::from(r16(ship, 0x128));
            if peer(self.ships, target)?[0x12A] & 6 == 0 {
                if r16(ship, 0x138) < r16(ship, 0x128) {
                    self.board(ship, target)?;
                }
                if r16(ship, 0x128) != 0xFFFF {
                    if ship[0x26] != 0 && ship[0x149] == 0 {
                        ship[0x26] -= 1;
                        ship[0x149] = 0x4B;
                    }
                    let other = peer(self.ships, usize::from(r16(ship, 0x128)))?;
                    let aim = heading(
                        i32::from((r16(ship, 0xA) as i16).wrapping_sub(r16(other, 0xA) as i16)),
                        i32::from((r16(ship, 0xE) as i16).wrapping_sub(r16(other, 0xE) as i16)),
                    );
                    let own = ship[0x11];
                    let delta = own.wrapping_sub(aim);
                    if (own == aim || delta > 0x47) && (delta < 0x81 || delta > 0xB7) {
                        if (delta > 0x48 && delta < 0x80) || delta > 0xB8 {
                            ship[0x11] = ship[0x11].wrapping_sub(2);
                        }
                    } else {
                        ship[0x11] = ship[0x11].wrapping_add(2);
                    }
                    let speed = r32(ship, 0x1C);
                    w32(ship, 0x1C, speed - (speed >> 4));
                }
                w16(ship, 0x20, 0xFFFF);
                self.do_steer(ship, false)?;
                self.tail(ship, false)?;
                return Ok(StepOutcome::Done);
            }
            w16(ship, 0x128, 0xFFFF);
            ship[0x12A] &= !8;
        }
        let side = usize::from(ship[0x13D]);
        let map = self.battle[0x559 + side];
        let aggressive = self.battle[0x5D + side] != 0;
        if ship[0x12A] & 1 == 0 && self.battle[0x5C] == 0 && !aggressive && r16(ship, 0x128) == 0xFFFF {
            return self.idle(ship);
        }
        if ship[0x161] == 0 && aggressive {
            self.siege_policy(ship, map)?;
        }
        if map == 1 && ship[0x12A] & 1 != 0 && r32(self.battle, TOWER_SLOTS) == 0 {
            ship[0x12A] &= !1;
        }
        if ship[0x12A] & 1 == 0 {
            if r16(ship, 0x128) == 0xFFFF {
                w16(ship, 0x20, 0xFFFF);
                if aggressive {
                    pick_target(ship, self.me, self.battle, self.ships, self.worlds, self.ctx.next)?;
                    course_field(ship, self.battle)?;
                }
                let level = ship[0x26];
                self.do_steer(ship, aggressive)?;
                Self::settle_level(ship, level);
                self.tail(ship, true)?;
                return Ok(StepOutcome::Done);
            }
            course_field(ship, self.battle)?;
            if self.siege_approach(ship, map, aggressive)? {
                return Ok(StepOutcome::Done);
            }
        } else {
            course_field(ship, self.battle)?;
            self.siege_retreat(ship, map)?;
        }
        self.refresh_boxes(ship, false)?;
        ship[COURSE..COURSE + 0x100].fill(0);
        Ok(if r16(ship, 0x128) != 0xFFFF || aggressive {
            StepOutcome::Weapons
        } else {
            StepOutcome::Done
        })
    }
}

impl<H: BoardingHost> Env<'_, H> {
    /// Decisión de huir (bit 1) del barco de asedio. Solo con `+0x161 == 0` y bando agresivo.
    fn siege_policy(&mut self, ship: &mut [u8], map: u8) -> Result<(), E> {
        if map > 1 {
            return Err("mapa de bando fuera de rango");
        }
        let b32 = |s: &Self, at: usize| -> u32 { r32(s.battle, at) as u32 };
        let b16 = |s: &Self, at: usize| -> u32 { u32::from(r16(s.battle, at)) };
        let world = self.world_of(ship)?.to_vec();
        if map == 0 {
            let enemy = r32(self.battle, 0xAA4);
            let mut limit = enemy.wrapping_mul(0x8C) as u32;
            let strength = enemy.wrapping_mul(0x1A4) as u32;
            let max_health = r32(&world, 0x14);
            if (max_health.wrapping_mul(2) as u32) < strength {
                limit = div(max_health.wrapping_shl(1), 3)? as u32;
            }
            if limit.wrapping_shl(2) < max_health as u32 {
                limit = (max_health >> 2) as u32;
            }
            let quit = u32::from(self.battle[0xAB0]);
            let (a90, a94) = (b32(self, 0xA90), b32(self, 0xA94));
            let (a9a, a9c) = (b16(self, 0xA9A), b16(self, 0xA9C));
            let health = r32(&world, 0x18);
            if r32(ship, 0x144) == health && self.battle[0xAC9] == 0 {
                ship[0x12A] &= !1;
                return Ok(());
            }
            let flee = if a90.wrapping_mul(quit) < a94 && a9c.wrapping_mul(quit) < a9a.wrapping_mul(2) {
                true
            } else if a90.wrapping_mul(quit).wrapping_mul(4) < a94 {
                true
            } else if a9c.wrapping_mul(quit).wrapping_mul(3) < a9a {
                true
            } else if (health as u32) < limit {
                true
            } else {
                r32(&world, 0x120) == 0 && u32::from(r16(&world, 0x40)) < a9a.wrapping_mul(2)
            };
            if flee {
                ship[0x12A] |= 1;
                self.battle[0xAC9] = 1;
            } else {
                ship[0x12A] &= !1;
            }
        } else {
            let range = world_slot(self.worlds, r32(ship, 4))?;
            let strength = artillery_strength(&self.worlds[range], self.ctx.captains) as u32;
            let (x, y) = (x_of(ship), y_of(ship));
            let far_x = r32(self.battle, 0xAB4 + 4).wrapping_sub(x).wrapping_abs() > 200;
            let far_y = r32(self.battle, 0xABC + 4).wrapping_sub(y).wrapping_abs() > 200;
            if strength < b32(self, 0xA94) && strength < b32(self, 0xA90) && (far_x || far_y) {
                ship[0x12A] |= 1;
            } else {
                ship[0x12A] &= !1;
            }
        }
        Ok(())
    }

    /// Persecución del objetivo (barco o torre); `true` si el paso termina sin armas.
    fn siege_approach(&mut self, ship: &mut [u8], map: u8, aggressive: bool) -> Result<bool, E> {
        let target_id = r16(ship, 0x128);
        let ships_count = r16(self.battle, 0x14);
        let is_ship = target_id < ships_count;
        let mut give_up = false;
        if is_ship {
            let t = peer(self.ships, usize::from(target_id))?;
            if t[0x12A] & 6 == 0 {
                if aggressive {
                    let t_map = self.battle[0x559 + usize::from(t[0x13D])];
                    if t_map == map {
                        give_up = true;
                    } else if r16(t, 0x128) < ships_count && t[0x12A] & 8 != 0 {
                        let crew = u32::from(r16(self.world_of(t)?, 0x40));
                        let its_target = self.lookup(ship, usize::from(r16(t, 0x128)))?;
                        let other = u32::from(r16(self.world_of(its_target)?, 0x40));
                        if other.wrapping_mul(2) < crew {
                            give_up = true;
                        }
                    }
                }
            } else {
                give_up = true;
            }
        } else if r32(
            self.battle,
            TOWER_SLOTS + 4 * usize::from(0xFFFEu16.wrapping_sub(target_id)),
        ) == 0
        {
            give_up = true;
        }
        if give_up {
            w16(ship, 0x20, 0xFFFF);
            if !aggressive {
                w16(ship, 0x128, 0xFFFF);
                ship[0x148] = ship[0x26];
                w16(ship, 0x24, u16::from(ship[0x11]));
            } else {
                pick_target(ship, self.me, self.battle, self.ships, self.worlds, self.ctx.next)?;
            }
            let level = ship[0x26];
            let flag = !(!aggressive && self.battle[0x5C] == 0);
            self.do_steer(ship, flag)?;
            Self::settle_level(ship, level);
            self.tail(ship, true)?;
            return Ok(true);
        }
        let (tx, ty, tvx, tvy, t_map);
        if is_ship {
            let t = peer(self.ships, usize::from(target_id))?;
            (tx, ty, tvx, tvy) = (r16(t, 0xA), r16(t, 0xE), r32(t, 0x14), r32(t, 0x18));
            t_map = self.battle[0x559 + usize::from(t[0x13D])];
        } else {
            let k = 2 * usize::from(0xFFFEu16.wrapping_sub(target_id));
            (tx, ty, tvx, tvy) = (r16(self.battle, 0x1E9E + k), r16(self.battle, 0x1EA6 + k), 0, 0);
            t_map = 1;
        }
        w16(ship, 0x20, tx);
        w16(ship, 0x22, ty);
        let base_x = i32::from(tx as i16) - x_of(ship);
        let base_y = i32::from(ty as i16) - y_of(ship);
        let (mut lead_x, mut lead_y) = (base_x, base_y);
        if is_ship {
            for _ in 0..4 {
                let distance = lead_x.wrapping_mul(lead_x).wrapping_add(lead_y.wrapping_mul(lead_y));
                if distance < 0 {
                    return Err("raíz de un valor negativo");
                }
                let reach = ftol_i32(f64::from(distance).sqrt());
                let reach = div(reach.wrapping_shl(8), 0x530)?;
                let (vx, vy) = if self.battle[0xAC8] == 0 {
                    (tvx, tvy)
                } else {
                    (tvx >> 4, tvy >> 4)
                };
                lead_y = base_y.wrapping_add(vy.wrapping_mul(reach) >> 16);
                lead_x = base_x.wrapping_add(vx.wrapping_mul(reach) >> 16);
            }
        }
        let range = (lead_x.wrapping_mul(lead_x).wrapping_add(lead_y.wrapping_mul(lead_y))) as u32;
        // 0x0061F02F sobre el objetivo, solo si es un barco
        let boardable = if is_ship {
            let t = peer(self.ships, usize::from(target_id))?.clone();
            Some(self.board_filter(ship, &t, false)?)
        } else {
            None
        };
        let not_boardable = boardable != Some(true);
        if t_map != map {
            let (x, y) = (r16(ship, 0xA) as i16, r16(ship, 0xE) as i16);
            if x < 400 && (!is_ship || not_boardable || 0 < lead_x) {
                let fill = (400 - i32::from(x)).min(0xFF) as u8;
                ship[0xA7..0xA7 + 0x80].fill(fill);
            }
            if y < 400 && (target_id > ships_count || not_boardable || 0 < lead_y) {
                for i in 0..0x80usize {
                    ship[COURSE + ((i + 0xC0) & 0xFF)] = 0xFF;
                }
            }
            if x > 0xF70 && (target_id > ships_count || not_boardable || lead_x < 0) {
                ship[COURSE..COURSE + 0x80].fill(0xFF);
            }
            if y > 0xD72 && (target_id > ships_count || not_boardable || lead_y < 0) {
                ship[0x67..0x67 + 0x80].fill(0xFF);
            }
        }
        let (near, wide): (u32, u32) = if ship[0x13C] == 0 || !is_ship {
            (0x31A88, 0x38400)
        } else {
            (0x15F90, 0x1FA40)
        };
        if range < near {
            if map == t_map {
                if ship[0x149] == 0 && ship[0x26] != 0 {
                    ship[0x26] -= 1;
                    ship[0x149] = 0x4B;
                }
                w16(ship, 0x20, 0xFFFF);
            } else {
                ship[0x12A] |= 0x10;
            }
        }
        if range < wide && ship[0x12A] & 0x10 != 0 && !(boardable == Some(true)) {
            w16(ship, 0x20, 0xFFFF);
            let mut away = heading(i32::from(lead_y as i16), -i32::from(lead_x as i16));
            if ship[0x11].wrapping_sub(away).wrapping_add(0x40) > 0x80 {
                away = away.wrapping_add(0x80);
            }
            let clear = |s: &[u8], a: u8| s[COURSE + usize::from(a)];
            if clear(ship, away) > 0x80 && clear(ship, away.wrapping_add(0x80)) < clear(ship, away) {
                away = away.wrapping_add(0x80);
            }
            if clear(ship, away) != 0 {
                let diff = away.wrapping_sub(ship[0x11]);
                let half = if diff & 0x80 == 0 {
                    diff >> 1
                } else {
                    (diff >> 1).wrapping_sub(0x80)
                };
                let mut best = ship[0x11].wrapping_add(half);
                let mut lowest = 0xFFu8;
                let mut k: u8 = 1;
                loop {
                    if clear(ship, away.wrapping_add(k)) == 0 || clear(ship, away.wrapping_sub(k)) == 0 {
                        break;
                    }
                    k = k.wrapping_add(1);
                    if clear(ship, away.wrapping_add(k)) < lowest {
                        lowest = clear(ship, away.wrapping_add(k));
                        best = away.wrapping_add(k);
                    }
                    if clear(ship, away.wrapping_sub(k)) < lowest {
                        lowest = clear(ship, away.wrapping_sub(k));
                        best = away.wrapping_sub(k);
                    }
                    if k == 0x80 {
                        break;
                    }
                }
                away = best;
            }
            let own = ship[0x11];
            if i32::from(away) - i32::from(own) > 1 || i32::from(own) - i32::from(away) > 1 {
                let range = world_slot(self.worlds, r32(ship, 4))?;
                let direct = self.battle[0xAC8] != 0;
                turn(ship, &self.worlds[range], self.ctx.captains, direct, away)?;
            }
            if ship[0x149] == 0 {
                let own = usize::from(ship[0x11]);
                let open = |s: &[u8], a: usize| s[COURSE + (a & 0xFF)];
                if open(ship, own) <= 0x80 || open(ship, own + 1) <= 0x80 || open(ship, own.wrapping_sub(1)) <= 0x80 {
                    if ship[0x26] < 4 {
                        ship[0x26] += 1;
                        ship[0x149] = 0x4B;
                    }
                } else if ship[0x26] != 0 {
                    ship[0x26] -= 1;
                    ship[0x149] = 0x4B;
                }
            }
            self.do_steer(ship, false)?;
        } else {
            ship[0x12A] &= !0x10;
            if ship[0x149] == 0 && ship[COURSE + usize::from(ship[0x11])] > 0x80 && ship[0x26] != 0 {
                ship[0x26] -= 1;
                ship[0x149] = 0x4B;
            }
            let level = ship[0x26];
            self.do_steer(ship, true)?;
            if level != ship[0x26] {
                if ship[0x149] == 0 && ship[COURSE + usize::from(ship[0x11])] < 0x81 {
                    ship[0x149] = 0x4B;
                } else {
                    ship[0x26] = level;
                }
            }
        }
        Ok(false)
    }

    /// Huida del barco de asedio hacia el centro de su mapa (mapa 0) o hacia el punto de salida del bando (mapa 1).
    fn siege_retreat(&mut self, ship: &mut [u8], map: u8) -> Result<(), E> {
        if map > 1 {
            return Err("mapa de bando fuera de rango");
        }
        let (x, y) = (x_of(ship), y_of(ship));
        let (mut dx, mut dy);
        if map == 0 {
            if x < 400 || x > 0x2070 || y < 400 || y > 0x1C74 {
                dx = x - 0x880;
                dy = y - 0x781;
            } else {
                dx = x.wrapping_sub(r32(self.battle, 0xAB4 + 4) as i32);
                dy = y.wrapping_sub(r32(self.battle, 0xABC + 4) as i32);
            }
        } else {
            dx = (r32(self.battle, 0xAB4 + 4) as i32).wrapping_sub(x);
            dy = (r32(self.battle, 0xABC + 4) as i32).wrapping_sub(y);
        }
        if dx < 0 {
            dy = div((-x).wrapping_mul(dy), dx)?;
            dx = -x;
        } else if dx < 1 {
            dy = if dy < 0 { -y } else { 0x1E04 - y };
        } else {
            dy = div((0x2200 - x).wrapping_mul(dy), dx)?;
            dx = 0x2200 - x;
        }
        if dy < -y {
            dx = div(dx.wrapping_mul(-y), dy)?;
            dy = -y;
        } else if 0x1E04 - y < dy {
            dx = div((0x1E04 - y).wrapping_mul(dx), dy)?;
            dy = 0x1E04 - y;
        }
        w16(ship, 0x20, (x.wrapping_add(dx)) as u16);
        w16(ship, 0x22, (y.wrapping_add(dy)) as u16);
        let level = ship[0x26];
        self.do_steer(ship, true)?;
        Self::settle_level(ship, level);
        Ok(())
    }
}

/// Un paso de `SiegeShip::update` (0x00621770) para el barco `me` de la tabla. `param` es el argumento de pila del original
/// y `limit` el dato global `[0x006DE4A0+0x10]`.
pub fn siege_ship_step<H: BoardingHost>(
    me: usize,
    param: u8,
    limit: u32,
    battle: &mut [u8],
    ships: &mut [Vec<u8>],
    worlds: &mut [u8],
    ctx: &super::step::StepContext,
    host: &mut H,
    events: &mut Vec<StepEvent>,
) -> Result<StepOutcome, E> {
    let mut ship = std::mem::take(ships.get_mut(me).ok_or("barco local ausente")?);
    let result = Env {
        me,
        battle,
        ships: &mut *ships,
        worlds,
        ctx,
        host,
        ev: events,
        param,
    }
    .run_siege(&mut ship, limit);
    ships[me] = ship;
    result
}
