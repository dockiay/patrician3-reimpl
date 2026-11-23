//! Selección de objetivos y armas del barco de asedio (`SiegeShip::update_weapons`, 0x00624950). Como la del barco de mar
//! (`weapons.rs`), pero el mapa 0 añade las cuatro torres de la defensa (`battle+0x1EC4`) como objetivos y el filtro de
//! enemigos es el de abordaje simple. El daño a las torres (0x0060130B) y los avisos quedan como ganchos del ejecutor.
//! Fixture: `oracle/x86_siege_weapons.py`.

use super::projectile::heading;
use super::siege::TOWER_SLOTS;
use super::weapons::{predict, DirectShot, WeaponPlan};

type E = &'static str;
const END: u16 = 0xFFFF;

fn r16(b: &[u8], o: usize) -> u16 {
    u16::from_le_bytes([b[o], b[o + 1]])
}
fn r32(b: &[u8], o: usize) -> i32 {
    i32::from_le_bytes(b[o..o + 4].try_into().unwrap())
}
fn i16s(b: &[u8], o: usize) -> i32 {
    i32::from(r16(b, o) as i16)
}
fn isqrt(d2: i32) -> Result<i32, E> {
    if d2 < 0 {
        Err("raíz de un cuadrado desbordado (ruta matherr de la CRT no modelada)")
    } else {
        Ok(f64::from(d2).sqrt() as i32)
    }
}

/// Vista de sólo lectura del combate para elegir objetivos.
pub struct SiegeView<'a> {
    pub battle: &'a [u8],
    pub next: &'a [u16],
    pub local: &'a dyn Fn(u16) -> &'a [u8],
    pub world: &'a dyn Fn(u32) -> &'a [u8],
}

/// Rumbo del original a partir de las diferencias, truncadas a 16 bits con signo como hace el código en línea.
fn aim(dx: i32, dy: i32) -> u8 {
    heading(i32::from(dx as i16), i32::from(dy as i16))
}

/// 0x00624950. `skip_policy` es el parámetro de pila; `filter` es 0x0061F02F(propio, enemigo, 1).
pub fn plan_siege_weapons(
    v: &SiegeView,
    own: &[u8],
    skip_policy: bool,
    filter: &mut dyn FnMut(u16) -> Result<bool, E>,
) -> Result<WeaponPlan, E> {
    let world_own = (v.world)(r32(own, 4) as u32);
    if r32(world_own, 0x120) == 0 || own[0x12A] & 0x2E != 0 {
        return Ok(WeaponPlan::None);
    }
    if own[0x130] != 0 && own[0x131] != 0 {
        return Ok(WeaponPlan::None);
    }
    let map = v.battle[0x559 + usize::from(own[0x13D])];
    let aggressive = v.battle[0x5D + usize::from(own[0x13D])] != 0;
    let own_id = r16(own, 0x138);
    let heading_own = own[0x11];
    let (own_x, own_y) = (i16s(own, 0x0A), i16s(own, 0x0E));
    let direct = v.battle[0xAC8] != 0;
    let (mut cand0, mut cand1) = (END, END);
    let (mut best0, mut best1) = (0x7FFF_FFFFi32, 0x7FFF_FFFFi32);
    let (mut d0, mut d1) = ((0i32, 0i32), (0i32, 0i32));
    let own_head = if map == 0 {
        r16(v.battle, 0x10)
    } else {
        r16(v.battle, 0x65C)
    };
    let enemy_head = if map == 0 {
        r16(v.battle, 0x65C)
    } else {
        r16(v.battle, 0x10)
    };
    let tower = |k: usize| r32(v.battle, TOWER_SLOTS + 4 * k);
    let tower_delta = |k: usize| -> (i32, i32) {
        (
            i32::from(r16(v.battle, 0x1E9E + 2 * k)) - own_x,
            i32::from(r16(v.battle, 0x1EA6 + 2 * k)) - own_y,
        )
    };

    // Pasada 1: barcos del propio bando como obstáculos.
    let mut id = own_head;
    while id != END {
        if id != own_id {
            let other = (v.local)(id);
            if other[0x12A] & 6 == 0 {
                let dx0 = i16s(other, 0x0A) - own_x;
                let dy0 = i16s(other, 0x0E) - own_y;
                let (dx, dy, _) = predict(dx0, dy0, (r32(other, 0x14), r32(other, 0x18)), direct)?;
                let angle = aim(dx, dy);
                let d2 = dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx));
                let tolerance = if d2 < 0x559 { 0x37 } else { 2000 / isqrt(d2)? };
                let a = i32::from(angle.wrapping_sub(heading_own));
                let edx = a + tolerance;
                if edx > 0x37 && a - tolerance < 0x49 {
                    if d2 < best1 {
                        best1 = d2 - 0x50;
                    }
                } else if edx > 0xB7 && a - tolerance < 0xC9 && d2 < best0 {
                    best0 = d2 - 0x50;
                }
            }
        }
        id = v.next[usize::from(id)];
    }

    // Pasada 2: enemigos.
    let mut id = enemy_head;
    while id != END {
        let other = (v.local)(id);
        if other[0x12A] & 6 == 0 {
            let mut skip = false;
            if !skip_policy {
                if id != r16(own, 0x128) && !aggressive {
                    skip = true;
                }
                if !skip && filter(id)? {
                    skip = true;
                }
            }
            if !skip {
                let dx0 = i16s(other, 0x0A) - own_x;
                let dy0 = i16s(other, 0x0E) - own_y;
                let (dx, dy, last) = predict(dx0, dy0, (r32(other, 0x14), r32(other, 0x18)), direct)?;
                let angle = aim(dx, dy);
                let tolerance = if last < 0x64 { 0x37 } else { 500 / isqrt(last)? };
                let a = i32::from(angle.wrapping_sub(heading_own));
                let edx = a + tolerance;
                let dd = dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx));
                if edx > 0x37 && a - tolerance < 0x49 {
                    if dd < best1 && dd <= 0x38400 {
                        best1 = dd;
                        cand1 = id;
                        d1 = (dx, dy);
                    }
                } else if edx > 0xB7 && a - tolerance < 0xC9 && dd < best0 && dd <= 0x38400 {
                    best0 = dd;
                    cand0 = id;
                    d0 = (dx, dy);
                }
            }
        }
        id = v.next[usize::from(id)];
    }
    // Torres (mapa 0): objetivo propio, o cualquiera con parámetro verdadero o bando agresivo.
    if map == 0 {
        for k in 0..4usize {
            if tower(k) > 0 && (k as u16 == 0xFFFEu16.wrapping_sub(r16(own, 0x128)) || skip_policy || aggressive) {
                let (dx, dy) = tower_delta(k);
                let angle = aim(dx, dy).wrapping_sub(heading_own);
                let dd = dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx));
                if angle > 0x37 && angle < 0x49 {
                    if dd < best1 && dd <= 0x38400 {
                        best1 = dd;
                        cand1 = 0xFFFEu16.wrapping_sub(k as u16);
                        d1 = (dx, dy);
                    }
                } else if angle > 0xB7 && angle < 0xC9 && dd < best0 && dd <= 0x38400 {
                    best0 = dd;
                    cand0 = 0xFFFEu16.wrapping_sub(k as u16);
                    d0 = (dx, dy);
                }
            }
        }
    }

    let target = |band: usize, d: (i32, i32)| -> (u8, (i16, i16)) {
        (
            band as u8,
            (own_x.wrapping_add(d.0) as i16, own_y.wrapping_add(d.1) as i16),
        )
    };
    if direct {
        let mut shots = Vec::new();
        if cand0 != END && own[0x130] == 0 {
            shots.push(DirectShot {
                band: 0,
                target: cand0,
                length: isqrt(best0)?,
            });
        }
        if cand1 != END && own[0x131] == 0 {
            shots.push(DirectShot {
                band: 1,
                target: cand1,
                length: isqrt(best1)?,
            });
        }
        return Ok(if shots.is_empty() {
            WeaponPlan::None
        } else {
            WeaponPlan::Direct(shots)
        });
    }
    let mut volleys = Vec::new();
    if cand0 != END && own[0x130] == 0 {
        volleys.push(target(0, d0));
    }
    if cand1 != END && own[0x131] == 0 {
        volleys.push(target(1, d1));
    }
    if !skip_policy || cand0 != END || cand1 != END {
        return Ok(if volleys.is_empty() {
            WeaponPlan::None
        } else {
            WeaponPlan::Volleys(volleys)
        });
    }

    // Pasada 3 (parámetro verdadero y sin candidatos): lo más cercano por semiplano, torres incluidas.
    let mut id = enemy_head;
    while id != END {
        let other = (v.local)(id);
        if other[0x12A] & 6 == 0 {
            let dx = i16s(other, 0x0A) - own_x;
            let dy = i16s(other, 0x0E) - own_y;
            let d2 = dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx));
            if aim(dx, dy).wrapping_sub(heading_own) < 0x80 {
                if d2 < best1 {
                    best1 = d2;
                    cand1 = id;
                    d1 = (dx, dy);
                }
            } else if d2 < best0 {
                best0 = d2;
                cand0 = id;
                d0 = (dx, dy);
            }
        }
        id = v.next[usize::from(id)];
    }
    if map == 0 {
        for k in 0..4usize {
            if tower(k) > 0 {
                let (dx, dy) = tower_delta(k);
                let d2 = dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx));
                if aim(dx, dy).wrapping_sub(heading_own) < 0x80 {
                    if d2 < best1 {
                        best1 = d2;
                        cand1 = k as u16;
                        d1 = (dx, dy);
                    }
                } else if d2 < best0 {
                    best0 = d2;
                    cand0 = k as u16;
                    d0 = (dx, dy);
                }
            }
        }
    }
    let (mut c0, mut c1) = (cand0, cand1);
    if own[0x130] != 0 {
        c0 = END;
    }
    if own[0x131] != 0 {
        c1 = END;
    }
    if c0 != END {
        if c1 != END && !(best0 >= best1) {
            c1 = END;
        }
        if c1 == END && own[0x130] == 0 {
            return Ok(WeaponPlan::Volleys(vec![target(0, d0)]));
        }
    }
    if c1 != END && own[0x131] == 0 {
        return Ok(WeaponPlan::Volleys(vec![target(1, d1)]));
    }
    Ok(WeaponPlan::None)
}

/// Efectos que el ejecutor delega.
pub trait SiegeEffects {
    /// 0x00620F0B con recuento y máscara nulos.
    fn estimate(&mut self, battle: &mut [u8], band: u8, length: i32) -> Result<i32, E>;
    /// 0x0061F96F sobre un barco local (bajas -1, máscara 3).
    fn ship_damage(&mut self, battle: &mut [u8], target: u16, amount: i32) -> Result<(), E>;
    /// 0x0060130B: golpe a la torre `index` (el original pasa `0xFE - id`).
    fn tower_damage(&mut self, battle: &mut [u8], index: u8, amount: i32) -> Result<(), E>;
    fn notify(&mut self, code: u8);
    fn volley(&mut self, battle: &mut [u8], band: u8, target: (i16, i16)) -> Result<(), E>;
}

/// Ejecuta el plan con el orden del original. `target_flags` da las banderas `+0x12A` de un barco local.
pub fn execute_siege_plan<X: SiegeEffects>(
    plan: &WeaponPlan,
    own: &mut [u8],
    battle: &mut [u8],
    target_flags: &dyn Fn(u16) -> u8,
    fx: &mut X,
) -> Result<(), E> {
    match plan {
        WeaponPlan::None => {}
        WeaponPlan::Volleys(volleys) => {
            for (band, target) in volleys {
                fx.volley(battle, *band, *target)?;
            }
        }
        WeaponPlan::Direct(shots) => {
            for shot in shots {
                let map = usize::from(battle[0x559 + usize::from(own[0x13D])]);
                let counter = 0xACA + map;
                if battle[counter] < 0x7F {
                    battle[counter] += 1;
                }
                let is_ship = shot.target < r16(battle, 0x14);
                if is_ship && target_flags(shot.target) & 0x20 != 0 {
                    battle[counter] = 0xFF;
                }
                let amount = fx.estimate(battle, shot.band, shot.length)?;
                if is_ship {
                    fx.ship_damage(battle, shot.target, amount)?;
                } else {
                    fx.tower_damage(battle, 0xFEu8.wrapping_sub(shot.target as u8), amount)?;
                }
                let side = usize::from(battle[0x559 + usize::from(own[0x13D])]);
                if usize::from(battle[0xAD2]) == side + 1 {
                    fx.notify(0x0E);
                }
                own[0x130 + usize::from(shot.band)] = 0x96;
            }
        }
    }
    Ok(())
}
