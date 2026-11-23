//! Selección de objetivos y armas del barco local (`LocalMapShip::update_weapons`, 0x0061DD4F) y estimación agregada
//! de daño (0x00620F0B). La selección es una función pura del estado del combate; el ejecutor aplica sus efectos.
//! Fixture: `oracle/x86_battle_weapons.py`.

use super::projectile::{adjusted_damage, heading, ShotContext};

const WEAPON_A: [i32; 6] = [32, 32, 77, 77, 96, 58];
const WEAPON_B: [i32; 6] = [60, 80, 60, 80, 90, 90];
const WEAPON_R: [i32; 6] = [480, 160, 480, 160, 120, 120];
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

fn isqrt(d2: i32) -> Result<i32, &'static str> {
    if d2 < 0 {
        Err("raíz de un cuadrado desbordado (ruta matherr de la CRT no modelada)")
    } else {
        Ok(f64::from(d2).sqrt() as i32)
    }
}

/// Vista de sólo lectura del combate para elegir objetivos.
pub struct SelectView<'a> {
    /// Bytes escalares del combate (cabezas de lista `+0x10`/`+0x65C`, lados `+0x559`, `+0x5D`, modo directo `+0xAC8`).
    pub battle: &'a [u8],
    /// Lista enlazada de barcos por bando (`battle+0xC`): siguiente identificador local.
    pub next: &'a [u16],
    /// Barco local (0x170 bytes) por identificador local.
    pub local: &'a dyn Fn(u16) -> &'a [u8],
    /// Barco mundial (0x180 bytes) por índice.
    pub world: &'a dyn Fn(u32) -> &'a [u8],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DirectShot {
    pub band: u8,
    pub target: u16,
    /// Raíz entera de la distancia² elegida.
    pub length: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WeaponPlan {
    /// Nada que hacer (guardas, o sin candidatos).
    None,
    /// Resolución directa: estimar y aplicar el daño agregado por banda.
    Direct(Vec<DirectShot>),
    /// Física normal: andanadas `(banda, punto de destino)`.
    Volleys(Vec<(u8, (i16, i16))>),
}

/// Predicción de cuatro aproximaciones del blanco: devuelve `(dx, dy)` final y la distancia² al inicio de la última.
pub(super) fn predict(dx0: i32, dy0: i32, velocity: (i32, i32), direct: bool) -> Result<(i32, i32, i32), &'static str> {
    let (vx, vy) = if direct {
        (velocity.0 >> 4, velocity.1 >> 4)
    } else {
        velocity
    };
    let (mut dx, mut dy) = (dx0, dy0);
    let mut last = 0;
    for _ in 0..4 {
        last = dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx));
        let length = isqrt(last)?;
        let t = length.wrapping_shl(8) / 0x530;
        dy = dy0.wrapping_add(vy.wrapping_mul(t) >> 16);
        dx = dx0.wrapping_add(vx.wrapping_mul(t) >> 16);
    }
    Ok((dx, dy, last))
}

/// Selecciona objetivos y armas (0x0061DD4F). `skip_policy` es el parámetro booleano; `filter` es 0x0061F02F.
pub fn plan_weapons(
    v: &SelectView,
    own: &[u8],
    skip_policy: bool,
    filter: &mut dyn FnMut(u16) -> Result<bool, &'static str>,
) -> Result<WeaponPlan, &'static str> {
    let world_own = (v.world)(r32(own, 4) as u32);
    if r32(world_own, 0x120) == 0 || own[0x12A] & 0x2E != 0 {
        return Ok(WeaponPlan::None);
    }
    if own[0x130] != 0 && own[0x131] != 0 {
        return Ok(WeaponPlan::None);
    }
    let side = v.battle[0x559 + usize::from(own[0x13D])];
    let own_id = r16(own, 0x138);
    let heading_own = own[0x11];
    let (own_x, own_y) = (i16s(own, 0x0A), i16s(own, 0x0E));
    let direct = v.battle[0xAC8] != 0;
    let (mut cand0, mut cand1) = (END, END);
    let (mut best0, mut best1) = (0x7FFF_FFFFi32, 0x7FFF_FFFFi32);
    let (mut d0, mut d1) = ((0i32, 0i32), (0i32, 0i32)); // (dx, dy)
                                                         // Cabezas: bando propio y bando contrario.
    let own_head = if side == 0 {
        r16(v.battle, 0x10)
    } else {
        r16(v.battle, 0x65C)
    };
    let enemy_head = if side == 0 {
        r16(v.battle, 0x65C)
    } else {
        r16(v.battle, 0x10)
    };

    // Pasada 1: obstáculos del propio bando.
    let mut id = own_head;
    while id != END {
        if id != own_id {
            let other = (v.local)(id);
            if other[0x12A] & 6 == 0 {
                let dx0 = i16s(other, 0x0A) - own_x;
                let dy0 = i16s(other, 0x0E) - own_y;
                let (dx, dy, _) = predict(dx0, dy0, (r32(other, 0x14), r32(other, 0x18)), direct)?;
                let angle = heading(dx, dy);
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
                let aggressive = v.battle[0x5D + usize::from(own[0x13D])] != 0;
                let flags = other[0x12A];
                let world_other = (v.world)(r32(other, 4) as u32);
                if aggressive && flags & 0x20 != 0 && world_other[0x136] & 0x40 == 0 {
                    skip = true;
                }
                if !skip && aggressive && flags & 8 != 0 {
                    let target = (v.local)(r16(other, 0x128));
                    let crew_target = r16((v.world)(r32(target, 4) as u32), 0x40) as i32;
                    let crew_other = r16(world_other, 0x40) as i32;
                    if crew_other * 2 < crew_target {
                        skip = true;
                    }
                }
                if !skip && id != r16(own, 0x128) && !aggressive {
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
                let angle = heading(dx, dy);
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
    if !skip_policy {
        return Ok(if volleys.is_empty() {
            WeaponPlan::None
        } else {
            WeaponPlan::Volleys(volleys)
        });
    }
    if cand0 != END || cand1 != END {
        return Ok(if volleys.is_empty() {
            WeaponPlan::None
        } else {
            WeaponPlan::Volleys(volleys)
        });
    }

    // Pasada 3 (parámetro verdadero y sin candidatos): el enemigo más cercano por semiplano angular.
    let mut id = enemy_head;
    while id != END {
        let other = (v.local)(id);
        if other[0x12A] & 6 == 0 {
            let dx = i16s(other, 0x0A) - own_x;
            let dy = i16s(other, 0x0E) - own_y;
            let d2 = dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx));
            let angle = heading(dx, dy);
            if angle.wrapping_sub(heading_own) < 0x80 {
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

/// Resultado de 0x00620F0B.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Estimate {
    pub damage: i32,
    /// Armas válidas de la banda (también escrito por el original en el puntero de recuento).
    pub count: u8,
    /// Máscara de armas pesadas (código >= 4) por posición, invertida para la banda 1.
    pub mask: u8,
}

/// `shl` de un registro de 8 bits por CL: las posiciones de 8 en adelante desbordan a cero.
fn shl8(n: u8) -> u8 {
    if n < 8 {
        1 << n
    } else {
        0
    }
}

/// 0x00620F0B: daño agregado de una banda a `length` (sólo 16 bits) con los mismos ajustes que un disparo.
pub fn estimate(
    weapons: &[u8; 24],
    band: u8,
    length: i32,
    shot: &ShotContext,
    battle: &mut [u8],
) -> Result<Estimate, &'static str> {
    let first = usize::from(band) * 2;
    let (mut total, mut count, mut mask) = (0i32, 0u8, 0u8);
    if first < 24 {
        for k in 0..(0x17 - first) / 4 + 1 {
            for slot in 0..2 {
                let weapon = weapons[first + 4 * k + slot];
                if weapon < 6 {
                    let w = usize::from(weapon);
                    let base = WEAPON_B[w] * WEAPON_A[w];
                    let divisor = (WEAPON_R[w] * 4 - WEAPON_R[w]).wrapping_neg().wrapping_mul(2);
                    let product = (length & 0xFFFF).wrapping_mul(base);
                    if divisor == 0 {
                        return Err("excepción de división (#DE)");
                    }
                    let part = base.wrapping_add(product / divisor);
                    total = total.wrapping_add(if part < 0 { 0 } else { part });
                    if weapon >= 4 {
                        mask |= shl8(count);
                    }
                    count = count.wrapping_add(1);
                }
            }
        }
    }
    if band == 1 && count > 0 {
        let old = mask;
        mask = 0;
        for i in 0..count {
            if u32::from(old) & (1u32 << u32::from((count - 1 - i) & 31)) != 0 {
                mask |= shl8(i);
            }
        }
    }
    if count == 0 {
        return Ok(Estimate { damage: 0, count, mask });
    }
    Ok(Estimate {
        damage: adjusted_damage(total, shot, battle),
        count,
        mask,
    })
}

/// Efectos que el ejecutor delega: el daño al casco (0x0061F96F), la estimación (0x00620F0B), los avisos y las andanadas.
pub trait WeaponEffects {
    fn estimate(&mut self, battle: &mut [u8], band: u8, length: i32) -> Result<i32, &'static str>;
    /// Aplica `amount` al barco local `target` con bajas -1 y máscara 3; devuelve AL.
    fn damage(&mut self, battle: &mut [u8], target: u16, amount: i32) -> Result<u8, &'static str>;
    fn notify(&mut self, code: u8);
    fn volley(&mut self, battle: &mut [u8], band: u8, target: (i16, i16)) -> Result<(), &'static str>;
}

/// Ejecuta el plan de `plan_weapons` con el orden y los contadores del original. `own` es el barco que dispara y
/// `target_flags` devuelve las banderas `+0x12A` de un barco local.
pub fn execute_plan<E: WeaponEffects>(
    plan: &WeaponPlan,
    own: &mut [u8],
    battle: &mut [u8],
    target_flags: &dyn Fn(u16) -> u8,
    fx: &mut E,
) -> Result<(), &'static str> {
    match plan {
        WeaponPlan::None => {}
        WeaponPlan::Volleys(volleys) => {
            for (band, target) in volleys {
                fx.volley(battle, *band, *target)?;
            }
        }
        WeaponPlan::Direct(shots) => {
            for shot in shots {
                let amount = fx.estimate(battle, shot.band, shot.length)?;
                let result = fx.damage(battle, shot.target, amount)?;
                own[0x130 + usize::from(shot.band)] = 0x96;
                let side = usize::from(battle[0x559 + usize::from(own[0x13D])]);
                let counter = 0xACA + side;
                if battle[counter] < 0x7F {
                    battle[counter] += 1;
                }
                if target_flags(shot.target) & 0x20 != 0 {
                    battle[counter] = 0xFF;
                }
                if usize::from(battle[0xAD2]) == side + 1 {
                    fx.notify(if result == 2 { 0x0B } else { 9 });
                }
            }
        }
    }
    Ok(())
}
