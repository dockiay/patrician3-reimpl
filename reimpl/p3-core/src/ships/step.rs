//! Paso por tic del barco local (`LocalMapShip::update`, 0x0061A9E0) y selección de objetivo (0x0061DBBF, vtable +4).
//!
//! El original recibe el barco, la batalla (campos por desplazamiento), la tabla de barcos locales, los barcos del mundo y
//! dos arrays de la batalla (siguiente de la lista y huecos del índice espacial). Los efectos externos (liberar el objeto
//! gráfico, mover o quitar cajas del índice espacial) se devuelven como [`StepEvent`] en el orden en que ocurren, y la
//! llamada final a las armas (vtable +8, 0x0061DD4F) como [`StepOutcome::Weapons`]. Fixture: `oracle/x86_battle_step.py`.

use super::boarding::{boarding_step, Boarding, BoardingHost, BoardingStep, Captains};
use super::projectile::heading;
use super::steer::{steer, turn};
use super::wind::table_sine;
use super::{r16, r32, w16, w32};
use crate::numeric::ftol_i32;
use crate::rng::sea_battle_next;

type E = &'static str;

/// Semianchura de la caja espacial por tipo de barco (`L"7BFT"`).
const HALF: [i32; 4] = [0x37, 0x42, 0x46, 0x54];

/// Efecto externo del paso.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepEvent {
    /// Método virtual +4 del objeto 0x006E51AC+0x3F8 con el campo `ship+0x134`.
    Release(i32),
    /// 0x00619560.
    SpatialRemove(u16),
    /// 0x006197A0 (los límites se truncan a 16 bits dentro del índice).
    SpatialBounds {
        slot: u16,
        x0: i32,
        y0: i32,
        x1: i32,
        y1: i32,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepOutcome {
    /// El paso termina sin llamar a las armas.
    Done,
    /// El paso termina llamando al método virtual +8 (armas, 0x0061DD4F).
    Weapons,
}

/// Datos de la batalla que el paso lee y no modifica.
pub struct StepContext<'a> {
    /// Siguiente barco de cada lista (`battle+0xC`, un `u16` por barco).
    pub next: &'a [u16],
    /// Hueco espacial de cada barco en `battle+0xA74` y `battle+0xA78`.
    pub slots_near: &'a [u16],
    pub slots_far: &'a [u16],
    pub captains: Captains<'a>,
    /// Comerciante humano (`word [+8] == 0`) del propietario del barco mundial.
    pub human_owner: &'a dyn Fn(u8) -> bool,
    /// Dificultad (`settings+0x12`).
    pub difficulty: u8,
    /// Método virtual 10 de la batalla (0x00611730).
    pub virtual_10: bool,
    /// La batalla está en pantalla (rama gráfica del abordaje, no modelada).
    pub view_active: bool,
}

pub(super) fn world_slot(worlds: &[u8], index: i32) -> Result<std::ops::Range<usize>, E> {
    let i = usize::try_from(index).map_err(|_| "barco mundial fuera de rango")?;
    let start = i.checked_mul(0x180).ok_or("barco mundial fuera de rango")?;
    if start + 0x180 > worlds.len() {
        return Err("barco mundial fuera de rango");
    }
    Ok(start..start + 0x180)
}

pub(super) fn peer(ships: &[Vec<u8>], id: usize) -> Result<&Vec<u8>, E> {
    match ships.get(id) {
        Some(s) if s.len() >= 0x170 => Ok(s),
        _ => Err("barco local ausente o igual al propio"),
    }
}

/// Peso de un enemigo para la selección de objetivo (0x0061EFDF).
pub(super) fn target_weight(ship: &[u8], world: &[u8]) -> i32 {
    if ship[0x12A] & 0x40 != 0 && r32(world, 0x118) == 0 {
        return 0;
    }
    let guns = r32(world, 0x120);
    if guns == 0 {
        return 1;
    }
    let shifted = guns.wrapping_shl(16);
    let health = r32(world, 0x18);
    if health > 0 {
        shifted / health
    } else {
        shifted
    }
}

/// 0x0061DBBF (vtable +4): elige al azar, ponderando por el peso, un enemigo de la lista contraria y lo guarda en `+0x128`.
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
    let chosen = choose_target(ship, me, battle, ships, worlds, next)?;
    w16(ship, 0x128, chosen);
    Ok(())
}

fn choose_target(
    ship: &[u8],
    me: usize,
    battle: &mut [u8],
    ships: &[Vec<u8>],
    worlds: &[u8],
    next: &[u16],
) -> Result<u16, E> {
    let side = usize::from(ship[0x13D]);
    let head = if battle[0x559 + side] == 0 {
        r16(battle, 0x65C)
    } else {
        r16(battle, 0x10)
    };
    let weight_of = |id: u16| -> Result<Option<i32>, E> {
        let s = if usize::from(id) == me {
            ship
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
    if head != 0xFFFF {
        let mut id = head;
        loop {
            if let Some(w) = weight_of(id)? {
                total = total.wrapping_add(w);
            }
            id = *next.get(usize::from(id)).ok_or("lista de barcos truncada")?;
            if id == 0xFFFF {
                break;
            }
        }
        if total > 0 {
            let mut seed = u32::from_le_bytes(battle[0xADC..0xAE0].try_into().unwrap());
            let value = sea_battle_next(&mut seed);
            battle[0xADC..0xAE0].copy_from_slice(&seed.to_le_bytes());
            total = (value % (total as u32)).wrapping_add(1) as i32;
        }
    }
    let mut chosen = head;
    if chosen != 0xFFFF {
        loop {
            if let Some(w) = weight_of(chosen)? {
                total = total.wrapping_sub(w);
                if total < 1 {
                    break;
                }
            }
            chosen = *next.get(usize::from(chosen)).ok_or("lista de barcos truncada")?;
            if chosen == 0xFFFF {
                break;
            }
        }
    }
    Ok(chosen)
}

pub(super) struct Env<'a, H> {
    pub(super) me: usize,
    pub(super) battle: &'a mut [u8],
    pub(super) ships: &'a mut [Vec<u8>],
    pub(super) worlds: &'a mut [u8],
    pub(super) ctx: &'a StepContext<'a>,
    pub(super) host: &'a mut H,
    pub(super) ev: &'a mut Vec<StepEvent>,
    pub(super) param: u8,
}

pub(super) fn x_of(ship: &[u8]) -> i32 {
    i32::from(r16(ship, 0xA) as i16)
}
pub(super) fn y_of(ship: &[u8]) -> i32 {
    i32::from(r16(ship, 0xE) as i16)
}
pub(super) fn dec(ship: &mut [u8], at: usize) {
    if ship[at] != 0 {
        ship[at] -= 1;
    }
}
pub(super) fn div(a: i32, b: i32) -> Result<i32, E> {
    a.checked_div(b).ok_or("excepción de división (#DE)")
}

impl<H: BoardingHost> Env<'_, H> {
    pub(super) fn map_of(&self, ship: &[u8]) -> u8 {
        self.battle[0x559 + usize::from(ship[0x13D])]
    }
    pub(super) fn aggressive(&self, ship: &[u8]) -> bool {
        self.battle[0x5D + usize::from(ship[0x13D])] != 0
    }
    pub(super) fn world_of(&self, ship: &[u8]) -> Result<&[u8], E> {
        Ok(&self.worlds[world_slot(self.worlds, r32(ship, 4))?])
    }

    /// Barco local por número: el propio (que se está modificando) o uno de la tabla.
    pub(super) fn lookup<'s>(&'s self, ship: &'s [u8], id: usize) -> Result<&'s [u8], E> {
        if id == self.me {
            Ok(ship)
        } else {
            Ok(peer(self.ships, id)?)
        }
    }

    /// 0x006264C0 con los argumentos habituales del paso: (parámetro, `battle+0x671`, bandera).
    pub(super) fn do_steer(&mut self, ship: &mut [u8], flag: bool) -> Result<(), E> {
        let range = world_slot(self.worlds, r32(ship, 4))?;
        let direct = self.battle[0xAC8] != 0;
        let throttle = self.battle[0x671];
        steer(
            ship,
            &mut self.worlds[range],
            self.ctx.captains,
            direct,
            self.param,
            throttle,
            flag,
        )
    }

    /// El nivel de velocidad solo puede cambiar una vez cada 0x4B tics.
    pub(super) fn settle_level(ship: &mut [u8], before: u8) {
        if ship[0x26] != before {
            if ship[0x149] == 0 {
                ship[0x149] = 0x4B;
            } else {
                ship[0x26] = before;
            }
        }
    }

    /// Bloques repetidos del original: caja cercana y caja lejana en el índice espacial. Con `forced` no se mira `flags&6`.
    pub(super) fn refresh_boxes(&mut self, ship: &[u8], forced: bool) -> Result<(), E> {
        let id = usize::from(r16(ship, 0x138));
        let near = *self.ctx.slots_near.get(id).ok_or("hueco espacial ausente")?;
        let far = *self.ctx.slots_far.get(id).ok_or("hueco espacial ausente")?;
        if forced || ship[0x12A] & 6 == 0 {
            let kind = self.world_of(ship)?[0xE];
            let half = *HALF.get(usize::from(kind)).ok_or("tipo de barco fuera de la tabla")?;
            let (x, y) = (x_of(ship), y_of(ship));
            self.ev.push(StepEvent::SpatialBounds {
                slot: near,
                x0: x - half,
                y0: y - half,
                x1: x + half,
                y1: y + half,
            });
            self.ev.push(StepEvent::SpatialBounds {
                slot: far,
                x0: x - half - 0x168,
                y0: y - half - 0x168,
                x1: x + 0x168 + half,
                y1: y + 0x168 + half,
            });
        } else {
            let park = (id as i32).wrapping_mul(-100);
            self.ev.push(StepEvent::SpatialBounds {
                slot: near,
                x0: park,
                y0: -100,
                x1: park,
                y1: -100,
            });
            self.ev.push(StepEvent::SpatialBounds {
                slot: far,
                x0: park,
                y0: -100,
                x1: park,
                y1: -100,
            });
        }
        Ok(())
    }

    pub(super) fn can_capture(&self, ship: &[u8], cand: &[u8]) -> bool {
        let lookup = |index: u32| -> &[u8] {
            let start = index as usize * 0x180;
            &self.worlds[start..start + 0x180]
        };
        let boarding = Boarding {
            battle: self.battle,
            virtual_10: self.ctx.virtual_10,
            world: &lookup,
            captains: self.ctx.captains,
        };
        boarding.can_capture(ship, cand, false)
    }

    pub(super) fn board_filter(&self, ship: &[u8], cand: &[u8], flag: bool) -> Result<bool, E> {
        let lookup = |index: u32| -> &[u8] {
            let start = index as usize * 0x180;
            &self.worlds[start..start + 0x180]
        };
        let boarding = Boarding {
            battle: self.battle,
            virtual_10: self.ctx.virtual_10,
            world: &lookup,
            captains: self.ctx.captains,
        };
        boarding.board_filter(ship, cand, flag)
    }

    /// 0x00611149: ¿queda un barco activo en la lista del mapa 0 y puede el bando huir hacia ella?
    pub(super) fn flee_possible(&self, ship: &[u8], side: u8) -> Result<bool, E> {
        if self.ctx.virtual_10 || self.battle[0x559 + usize::from(side)] == 0 || self.battle[0xACB] >= 4 {
            return Ok(false);
        }
        let mut id = r16(self.battle, 0x10);
        while id != 0xFFFF {
            if self.lookup(ship, usize::from(id))?[0x12A] & 0x66 == 0 {
                return Ok(true);
            }
            id = *self.ctx.next.get(usize::from(id)).ok_or("lista de barcos truncada")?;
        }
        Ok(false)
    }

    pub(super) fn pick(&mut self, ship: &mut [u8]) -> Result<(), E> {
        if ship[0x161] != 0 {
            return Ok(());
        }
        let chosen = choose_target(ship, self.me, self.battle, self.ships, self.worlds, self.ctx.next)?;
        w16(ship, 0x128, chosen);
        Ok(())
    }

    pub(super) fn tail(&mut self, ship: &mut [u8], forced: bool) -> Result<(), E> {
        self.refresh_boxes(ship, forced)?;
        ship[0x27..0x127].fill(0);
        Ok(())
    }

    pub(super) fn run(&mut self, ship: &mut [u8]) -> Result<StepOutcome, E> {
        if self.battle.len() < 0xAE0 || ship.len() < 0x170 {
            return Err("batalla o barco truncados");
        }
        // Balanceo del rumbo con el bit 4
        let probe = ship[0x11].wrapping_add(0x10);
        if ship[0x12A] & 4 != 0 && probe & 0x18 != 0 {
            ship[0x11] = if probe & 0x1F < 0x10 {
                ship[0x11].wrapping_sub(1)
            } else {
                ship[0x11].wrapping_add(1)
            };
        }
        if ship[0x12A] & 4 != 0 && r32(ship, 0x1C) != 0 {
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
        if x < 10 || y < 10 || x > 0x21F6 || y > 0x1DFA {
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
        let map = self.map_of(ship);
        if ship[0x12A] & 0x20 != 0 && self.battle[0xACB - usize::from(map)] == 0xFF {
            ship[0x12A] &= !0x20;
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
        let crew = r16(self.world_of(ship)?, 0x40);
        if ship[0x12A] & 0x20 == 0 && crew != 0 {
            self.active(ship)
        } else {
            if ship[0x26] != 0 && ship[0x149] == 0 {
                ship[0x26] -= 1;
                ship[0x149] = 0x4B;
            }
            w16(ship, 0x20, 0xFFFF);
            self.do_steer(ship, false)?;
            self.tail(ship, false)?;
            Ok(StepOutcome::Done)
        }
    }

    /// Abordaje en curso (0x0061F1BF) con los dos barcos y sus barcos del mundo.
    pub(super) fn board(&mut self, ship: &mut [u8], target: usize) -> Result<(), E> {
        let mine = world_slot(self.worlds, r32(ship, 4))?;
        let other_index = r32(peer(self.ships, target)?, 4);
        let theirs = world_slot(self.worlds, other_index)?;
        if mine == theirs {
            return Err("los dos barcos comparten barco mundial");
        }
        let (world_this, world_target) = if mine.start < theirs.start {
            let (a, b) = self.worlds.split_at_mut(theirs.start);
            (&mut a[mine.clone()], &mut b[..0x180])
        } else {
            let (a, b) = self.worlds.split_at_mut(mine.start);
            (&mut b[..0x180], &mut a[theirs.clone()])
        };
        let target_ship = self
            .ships
            .get_mut(target)
            .filter(|s| s.len() >= 0x170)
            .ok_or("barco local ausente")?;
        boarding_step(
            BoardingStep {
                battle: self.battle,
                this: ship,
                target: target_ship,
                world_this,
                world_target,
                captains: self.ctx.captains,
                human_owner: self.ctx.human_owner,
                difficulty: self.ctx.difficulty,
                virtual_10: self.ctx.virtual_10,
                view_active: self.ctx.view_active,
            },
            &mut *self.host,
        )
    }

    pub(super) fn active(&mut self, ship: &mut [u8]) -> Result<StepOutcome, E> {
        let side = ship[0x13D];
        let map = self.map_of(ship);
        let aggressive = self.aggressive(ship);
        if self.battle[0x5C] == 0 && !aggressive && r16(ship, 0x128) == 0xFFFF && ship[0x12A] & 1 == 0 {
            return self.idle(ship);
        }
        if map == 1 && ship[0x12A] & 0x20 != 0 && self.battle[0xACA] == 0xFF {
            ship[0x12A] &= !0x20;
        }
        if ship[0x161] == 0 && aggressive {
            self.policy(ship, map)?;
        }
        if ship[0x12A] & 1 == 0 {
            if r16(ship, 0x128) == 0xFFFF {
                w16(ship, 0x20, 0xFFFF);
                if aggressive {
                    self.pick(ship)?;
                }
                let level = ship[0x26];
                self.do_steer(ship, aggressive)?;
                Self::settle_level(ship, level);
                self.tail(ship, true)?;
                return Ok(StepOutcome::Done);
            }
            if self.approach(ship, side, map, aggressive)? {
                return Ok(StepOutcome::Done);
            }
        } else {
            self.retreat(ship, map)?;
        }
        self.refresh_boxes(ship, false)?;
        ship[0x27..0x127].fill(0);
        Ok(if r16(ship, 0x128) != 0xFFFF || self.aggressive(ship) {
            StepOutcome::Weapons
        } else {
            StepOutcome::Done
        })
    }

    /// Barco sin objetivo ni orden de ataque: sigue el rumbo ordenado en `+0x24`.
    pub(super) fn idle(&mut self, ship: &mut [u8]) -> Result<StepOutcome, E> {
        let order = r16(ship, 0x24) as i16;
        if order >= 0 {
            if order > 0xFF {
                return Err("rumbo ordenado fuera de dominio");
            }
            if u16::from(ship[0x11]) == order as u16 {
                w16(ship, 0x20, 0xFFFF);
            } else {
                let angle = order as u8;
                let dx = table_sine(angle) >> 10;
                w16(ship, 0x20, (r16(ship, 0xA) as i16).wrapping_add(dx as i16) as u16);
                let dy = table_sine(angle.wrapping_add(0x40)) >> 10;
                w16(ship, 0x22, (r16(ship, 0xE) as i16).wrapping_sub(dy as i16) as u16);
            }
        }
        if ship[0x149] == 0 {
            if ship[0x148] < ship[0x26] {
                ship[0x26] -= 1;
                ship[0x149] = 0x4B;
            } else if ship[0x26] < ship[0x148] {
                ship[0x26] += 1;
                ship[0x149] = 0x4B;
            }
        }
        self.do_steer(ship, false)?;
        self.tail(ship, false)?;
        Ok(StepOutcome::Done)
    }

    /// Decisión de huir (bit 1) o rendirse (bit 0x20), y umbral de fuerza. Solo con `+0x161 == 0` y bando agresivo.
    pub(super) fn policy(&mut self, ship: &mut [u8], map: u8) -> Result<(), E> {
        if map > 1 {
            return Err("mapa de bando fuera de rango");
        }
        let m = usize::from(map);
        let side = ship[0x13D];
        let b = |s: &Self, at: usize| -> u32 { u32::from_le_bytes(s.battle[at..at + 4].try_into().unwrap()) };
        let h = |s: &Self, at: usize| -> u32 { u32::from(r16(s.battle, at)) };
        let enemy = r32(self.battle, 0xAA0 + (1 - m) * 4);
        let world = self.world_of(ship)?.to_vec();
        let max_health = r32(&world, 0x14);
        let mut limit = (enemy.wrapping_mul(0x8C)) as u32;
        let strength = (enemy.wrapping_mul(0x1A4)) as u32;
        if self.battle[0xAB0 + m] < 2 {
            let twice = max_health.wrapping_mul(2) as u32;
            if twice < strength {
                limit = (div(max_health.wrapping_shl(1), 3)?) as u32;
            }
        } else if (max_health as u32) < strength {
            limit = div(max_health, 3)? as u32;
        }
        if limit.wrapping_shl(2) < max_health as u32 {
            limit = (max_health >> 2) as u32;
        }
        let health = r32(&world, 0x18) as u32;
        let guns = r32(&world, 0x120);
        let crew = u32::from(r16(&world, 0x40));
        let owner = u16::from(world[0]);
        if m == 0 {
            let quit = self.battle[0xAB0] as u32;
            let participant = r16(self.battle, 0x35A + usize::from(side) * 2);
            if participant != 0xFF && (r32(ship, 0x144) != r32(&world, 0x18) || self.battle[0xAC9] != 0) {
                let a90 = b(self, 0xA90);
                let a94 = b(self, 0xA94);
                let (a9a, a9c) = (h(self, 0xA9A), h(self, 0xA9C));
                let mut flee = false;
                if a90.wrapping_mul(quit).wrapping_mul(2) < a94 && a9c.wrapping_mul(quit) < a9a.wrapping_mul(2) {
                    flee = true;
                } else if a90.wrapping_mul(quit).wrapping_mul(4) < a94 {
                    flee = true;
                } else if a9c.wrapping_mul(quit).wrapping_mul(3) < a9a {
                    flee = true;
                } else if health < limit {
                    flee = true;
                } else if guns == 0 && crew < a9a.wrapping_mul(2) {
                    flee = true;
                } else if r16(self.battle, 0x35A) != 0xFE || owner != r16(self.battle, 0x35C) {
                    ship[0x12A] &= !1;
                }
                if flee {
                    ship[0x12A] |= 1;
                    self.battle[0xAC9] = 1;
                }
            }
        } else {
            let quit = u32::from(self.battle[0xAB1]);
            let (a98, a9e) = (h(self, 0xA98), h(self, 0xA9E));
            let (a90, a94) = (b(self, 0xA90), b(self, 0xA94));
            if a9e.wrapping_mul(quit).wrapping_mul(3) < a98 {
                ship[0x12A] |= 1;
            } else if a94.wrapping_mul(quit).wrapping_mul(4) < a90 {
                let participant = r16(self.battle, 0x35A + usize::from(side) * 2);
                let mut stay = participant == 0xFF;
                if !stay {
                    let range = world_slot(self.worlds, r32(ship, 4))?;
                    let speed = super::boarding::effective_speed(&self.worlds[range], self.ctx.captains)?;
                    let a8 = b(self, 0xAA8);
                    let scaled = speed.wrapping_mul(10);
                    stay = a8.wrapping_mul(9) < scaled;
                }
                stay = stay
                    || r32(ship, 0x144) == r32(&world, 0x18)
                    || self.battle[0xACA] == 0xFF
                    || self.battle[0xACB] != 0;
                if stay || !self.flee_possible(ship, side)? {
                    ship[0x12A] |= 1;
                } else {
                    ship[0x12A] |= 0x20;
                }
            } else if a94.wrapping_mul(quit).wrapping_mul(2) < a90 && a9e.wrapping_mul(quit) < a98.wrapping_mul(2) {
                ship[0x12A] |= 1;
            } else if health < limit {
                ship[0x12A] |= 1;
            } else if guns == 0 && crew < a98.wrapping_mul(2) {
                ship[0x12A] |= 1;
            } else if r16(self.battle, 0x35C) != 0xFE || owner != r16(self.battle, 0x35A) {
                ship[0x12A] &= !1;
            }
        }
        Ok(())
    }

    /// Persecución del objetivo; devuelve `true` si el paso termina aquí (sin armas).
    pub(super) fn approach(&mut self, ship: &mut [u8], _side: u8, map: u8, aggressive: bool) -> Result<bool, E> {
        let target_id = usize::from(r16(ship, 0x128));
        let (t_flags, t_side, t_map);
        {
            let t = peer(self.ships, target_id)?;
            t_flags = t[0x12A];
            t_side = t[0x13D];
            t_map = self.battle[0x559 + usize::from(t_side)];
        }
        let mut give_up = false;
        if t_flags & 6 == 0 {
            if aggressive {
                if t_map == map {
                    give_up = true;
                } else {
                    let t = peer(self.ships, target_id)?;
                    let twin = self.world_of(t)?;
                    let weight = target_weight(t, twin);
                    if t_flags & 0x20 == 0 || self.can_capture(ship, t) || weight != 0 {
                        if t_flags & 8 != 0 {
                            let crew = u32::from(r16(twin, 0x40));
                            let its_target = self.lookup(ship, usize::from(r16(t, 0x128)))?;
                            let other = u32::from(r16(self.world_of(its_target)?, 0x40));
                            if other.wrapping_mul(2) < crew {
                                give_up = true;
                            }
                        }
                    } else {
                        give_up = true;
                    }
                }
            }
        } else {
            give_up = true;
        }
        if give_up {
            w16(ship, 0x20, 0xFFFF);
            if !aggressive {
                w16(ship, 0x128, 0xFFFF);
                ship[0x148] = ship[0x26];
                w16(ship, 0x24, u16::from(ship[0x11]));
            } else {
                self.pick(ship)?;
            }
            let level = ship[0x26];
            let flag = !(!aggressive && self.battle[0x5C] == 0);
            self.do_steer(ship, flag)?;
            Self::settle_level(ship, level);
            self.tail(ship, true)?;
            return Ok(true);
        }
        let (tx, ty, tvx, tvy) = {
            let t = peer(self.ships, target_id)?;
            (r16(t, 0xA), r16(t, 0xE), r32(t, 0x14), r32(t, 0x18))
        };
        w16(ship, 0x20, tx);
        w16(ship, 0x22, ty);
        let base_x = i32::from(tx as i16) - x_of(ship);
        let base_y = i32::from(ty as i16) - y_of(ship);
        let (mut lead_x, mut lead_y) = (base_x, base_y);
        for _ in 0..4 {
            let distance = lead_x.wrapping_mul(lead_x).wrapping_add(lead_y.wrapping_mul(lead_y));
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
        let range = (lead_x.wrapping_mul(lead_x).wrapping_add(lead_y.wrapping_mul(lead_y))) as u32;
        if t_map != map {
            let t = peer(self.ships, target_id)?.clone();
            let (x, y) = (r16(ship, 0xA) as i16, r16(ship, 0xE) as i16);
            let not_boardable = !self.board_filter(ship, &t, false)?;
            if x < 400 && (not_boardable || 0 < lead_x) {
                let fill = (400 - i32::from(x)).min(0xFF) as u8;
                ship[0xA7..0xA7 + 0x80].fill(fill);
            }
            if x < 400 && (not_boardable || 0 < lead_y) {
                for i in 0..0x80usize {
                    ship[0x27 + ((i + 0xC0) & 0xFF)] = 0xFF;
                }
            }
            if x > 0x2070 && (not_boardable || lead_x < 0) {
                ship[0x27..0x27 + 0x80].fill(0xFF);
            }
            if y > 0x1C74 && (not_boardable || lead_y < 0) {
                ship[0x67..0x67 + 0x80].fill(0xFF);
            }
        }
        let (near, wide): (u32, u32) = if ship[0x13C] == 0 {
            (0x2B110, 0x38400)
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
        let mut avoid = false;
        if range < wide && ship[0x12A] & 0x10 != 0 {
            let t = peer(self.ships, target_id)?.clone();
            avoid = !self.board_filter(ship, &t, true)?;
        }
        if avoid {
            w16(ship, 0x20, 0xFFFF);
            let mut away = heading(i32::from(lead_y as i16), -i32::from(lead_x as i16));
            if ship[0x11].wrapping_sub(away).wrapping_add(0x40) > 0x80 {
                away = away.wrapping_add(0x80);
            }
            let clear = |s: &[u8], a: u8| s[0x27 + usize::from(a)];
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
            if ship[0x149] == 0 && ship[0x26] < 4 {
                ship[0x26] += 1;
                ship[0x149] = 0x4B;
            }
            self.do_steer(ship, false)?;
        } else {
            ship[0x12A] &= !0x10;
            let level = ship[0x26];
            self.do_steer(ship, true)?;
            Self::settle_level(ship, level);
        }
        Ok(false)
    }

    /// Huida hacia el borde del mapa más lejano del enemigo.
    pub(super) fn retreat(&mut self, ship: &mut [u8], map: u8) -> Result<(), E> {
        if map > 1 {
            return Err("mapa de bando fuera de rango");
        }
        let (x, y) = (x_of(ship), y_of(ship));
        let (mut dx, mut dy);
        if x < 400 || x > 0x2070 || y < 400 || y > 0x1C74 {
            dx = x - 0x1100;
            dy = y - 0xF02;
        } else {
            let other = 1 - usize::from(map);
            dx = x.wrapping_sub(r32(self.battle, 0xAB4 + other * 4));
            dy = y.wrapping_sub(r32(self.battle, 0xABC + other * 4));
        }
        if dx < 0 {
            let quotient = div((-x).wrapping_mul(dy), dx)?;
            dy = quotient;
            dx = -x;
        } else if dx < 1 {
            dy = if dy < 0 { -y } else { 0x1E04 - y };
        } else {
            let quotient = div((0x2200 - x).wrapping_mul(dy), dx)?;
            dy = quotient;
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

/// Un paso de `LocalMapShip::update` para el barco `me` de la tabla. `param` es el argumento de pila del original.
pub fn local_ship_step<H: BoardingHost>(
    me: usize,
    param: u8,
    battle: &mut [u8],
    ships: &mut [Vec<u8>],
    worlds: &mut [u8],
    ctx: &StepContext,
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
    .run(&mut ship);
    ships[me] = ship;
    result
}
