//! Gobierno del barco local: giro hacia un rumbo (0x006271D7) y avance con rumbo, aceleración y esquiva (0x006264C0).
//! Fixture: `oracle/x86_battle_steer.py`.

use super::boarding::{effective_speed, Captains};
use super::projectile::heading;
use super::wind::table_sine;

// Desplazamientos dentro del barco local (0x170 bytes)
const WORLD: usize = 4;
const X: usize = 8;
const Y: usize = 0xC;
const HEADING16: usize = 0x10;
const HEADING: usize = 0x11;
const VX: usize = 0x14;
const VY: usize = 0x18;
const SPEED: usize = 0x1C;
const DEST_X: usize = 0x20;
const DEST_Y: usize = 0x22;
const LEVEL: usize = 0x26;
const CLEAR: usize = 0x27;
const BORDER: usize = 0x127;

fn i32_at(b: &[u8], o: usize) -> i32 {
    i32::from_le_bytes(b[o..o + 4].try_into().unwrap())
}
fn set_i32(b: &mut [u8], o: usize, v: i32) {
    b[o..o + 4].copy_from_slice(&v.to_le_bytes());
}
fn i16_at(b: &[u8], o: usize) -> i16 {
    i16::from_le_bytes([b[o], b[o + 1]])
}
fn u16_at(b: &[u8], o: usize) -> u16 {
    u16::from_le_bytes([b[o], b[o + 1]])
}
fn set_u16(b: &mut [u8], o: usize, v: u16) {
    b[o..o + 2].copy_from_slice(&v.to_le_bytes());
}

fn clear_at(ship: &[u8], angle: u8) -> u8 {
    ship[CLEAR + usize::from(angle)]
}

/// Suma y máximo de la holgura (mínimo 8 por rumbo) entre `from` y `to` inclusive, avanzando de uno en uno.
fn sweep(ship: &[u8], from: u8, to: u8) -> (u16, u8) {
    let (mut sum, mut max) = (0u16, 0u8);
    let stop = to.wrapping_add(1);
    let mut at = from;
    while at != stop {
        let c = clear_at(ship, at);
        sum = sum.wrapping_add(if c < 9 { 8 } else { u16::from(c) });
        if max < c {
            max = c;
        }
        at = at.wrapping_add(1);
    }
    (sum, max)
}

/// 0x006271D7: gira el rumbo de 16 bits hacia el sector `target`.
pub fn turn(ship: &mut [u8], world: &[u8], captains: Captains, direct: bool, target: u8) -> Result<(), &'static str> {
    let factor: u32 = match world[0x0E] & 3 {
        0 => 0x100,
        1 => 0xE6,
        2 => 0x100,
        _ => 0xCD,
    };
    let speed = i32_at(ship, SPEED);
    let effective = effective_speed(world, captains)? as i32;
    let divisor = effective.wrapping_mul(0x334) as u32;
    if divisor == 0 {
        return Err("excepción de división (#DE)");
    }
    let mut step =
        (factor.wrapping_add((factor.wrapping_mul(speed as u32) / divisor) & 0xFFFF)).wrapping_mul(400) / 1000;
    if let Some(skill) = captains.byte(u32::from(u16_at(world, 0x42)), 9) {
        step = step.wrapping_mul(u32::from(skill) + 0x352) / 0x352;
    }
    let max_health = i32_at(world, 0x14);
    if max_health != 0 {
        if max_health == -1 && i32_at(world, 0x18).wrapping_mul(100) == i32::MIN {
            return Err("excepción de división (#DE)");
        }
        let percent = (i32_at(world, 0x18).wrapping_mul(100) / max_health) as u32;
        let low = percent as u8;
        if low < 0x3C {
            if low < 0x1F {
                step = step.wrapping_mul(0x11) / 0x14;
            } else {
                step = step.wrapping_mul((percent & 0xFF) + 0x8C) / 200;
            }
        }
    }
    if direct {
        step <<= 4;
    }
    let step16 = step as u16;
    let aim = u16::from(target).wrapping_mul(0x100).wrapping_add(0x80);
    let delta = aim.wrapping_sub(u16_at(ship, HEADING16));
    let current_hi = ship[HEADING];
    let (dir_first, dir_second) = if delta < 0x8000 {
        ((current_hi, target), (target, current_hi))
    } else {
        ((target, current_hi), (current_hi, target))
    };
    let snaps = if delta < 0x8000 {
        u32::from(delta) < step
    } else {
        0x10000 - u32::from(delta) < step
    };
    if snaps {
        set_u16(ship, HEADING16, aim);
        return Ok(());
    }
    let (first_sum, first_max) = sweep(ship, dir_first.0, dir_first.1);
    let second = if first_max == 0 {
        0xFFFF
    } else {
        let (sum, max) = sweep(ship, dir_second.0, dir_second.1);
        if max == 0 {
            0
        } else {
            sum
        }
    };
    let total_first = if first_max == 0 { 0 } else { first_sum };
    let heading16 = u16_at(ship, HEADING16);
    let toward = second < total_first;
    let value = if delta < 0x8000 {
        if toward {
            heading16.wrapping_sub(step16)
        } else {
            heading16.wrapping_add(step16)
        }
    } else if toward {
        heading16.wrapping_add(step16)
    } else {
        heading16.wrapping_sub(step16)
    };
    set_u16(ship, HEADING16, value);
    Ok(())
}

fn level_adjust(ship: &mut [u8], delta: i8, upper: u8) {
    let level = ship[LEVEL];
    ship[LEVEL] = (i16::from(level) + i16::from(delta)).clamp(0, i16::from(upper)) as u8;
}

/// 0x006264C0: velocidad hacia el rumbo deseado, integración del movimiento y elección del rumbo con esquiva.
pub fn steer(
    ship: &mut [u8],
    world: &mut [u8],
    captains: Captains,
    direct: bool,
    desired: u8,
    throttle: u8,
    avoid: bool,
) -> Result<(), &'static str> {
    let mut angle = desired.wrapping_sub(ship[HEADING]);
    if angle > 0x80 {
        angle = angle.wrapping_neg();
    }
    let effective = effective_speed(world, captains)? as i32;
    let base = effective.wrapping_mul(0x334);
    let drag = ((((base >> 4) as u32)
        .wrapping_mul(u32::from(angle))
        .wrapping_mul(u32::from(angle))) as i32)
        >> 10;
    let scaled = base.wrapping_sub(drag).wrapping_mul(i32::from(ship[LEVEL]));
    let mut target = ((scaled >> 2).wrapping_mul(i32::from(throttle))) / 5;
    let speed = i32_at(ship, SPEED);
    let limit = if direct { 0x1790 } else { 0x179 };
    if target.wrapping_sub(speed) <= limit {
        if speed.wrapping_sub(target) > limit {
            target = speed.wrapping_sub(limit);
        }
    } else {
        target = speed.wrapping_add(limit);
    }
    set_i32(ship, SPEED, target);
    let speed = target;
    let world_speed = ((speed / 0x334) << 6) / 0x73;
    if i32_at(world, 0x30) != world_speed {
        set_i32(world, 0x30, world_speed);
        let flags = u16_at(world, 0x3C) | 0x80;
        set_u16(world, 0x3C, flags);
    }
    let h = ship[HEADING];
    let (mut vx, mut vy) = (
        (speed >> 8).wrapping_mul(table_sine(h)) >> 8,
        (speed >> 8)
            .wrapping_neg()
            .wrapping_mul(table_sine(h.wrapping_add(0x40)))
            >> 8,
    );
    if direct {
        vx <<= 4;
        vy <<= 4;
    }
    set_i32(ship, VX, vx);
    set_i32(ship, VY, vy);
    set_i32(ship, X, i32_at(ship, X).wrapping_add(vx));
    set_i32(ship, Y, i32_at(ship, Y).wrapping_add(vy));
    let dest_x = i16_at(ship, DEST_X);
    if dest_x < 0 {
        return Ok(());
    }
    let dest_y = i16_at(ship, DEST_Y);
    let dx = i32::from(dest_x) - i32::from(i16_at(ship, 0x0A));
    let dy = i32::from(dest_y) - i32::from(i16_at(ship, 0x0E));
    let mut aim = heading(i32::from(dx as i16), i32::from(dy as i16));
    let distance = dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy));
    if avoid {
        let (limit_x, limit_y): (i32, i32) = if ship[BORDER] == 0 {
            (0x1100, 0xF02)
        } else {
            (0x2200, 0x1E04)
        };
        if distance > 0xAF8F
            || dest_x < 0x65
            || dest_y < 0x65
            || limit_x - 100 <= i32::from(dest_x)
            || limit_y - 100 <= i32::from(dest_y)
        {
            if ship[LEVEL] < 4 {
                ship[LEVEL] += 1;
            }
            let off = aim.wrapping_sub(desired);
            if off > 0x5F && off < 0xA1 {
                aim = if ship[HEADING].wrapping_sub(desired) < 0x80 {
                    desired.wrapping_add(0x5F)
                } else {
                    desired.wrapping_add(0xA1)
                };
            }
        }
    }
    let mut chosen = aim;
    if clear_at(ship, aim) != 0 {
        let offset = ship[HEADING].wrapping_sub(aim);
        let nudge = if offset & 0x80 == 0 {
            (offset >> 4) as i8
        } else {
            ((offset >> 4) as i8).wrapping_sub(0x10)
        };
        chosen = aim.wrapping_add(nudge as u8);
        let mut c: i8 = 1;
        let mut best: u8 = 0xFF;
        loop {
            if clear_at(ship, aim.wrapping_add(c as u8)) == 0 || clear_at(ship, aim.wrapping_sub(c as u8)) == 0 {
                break;
            }
            c = c.wrapping_add(1);
            if avoid {
                for sign in [1i8, -1] {
                    let at = if sign > 0 {
                        aim.wrapping_add(c as u8)
                    } else {
                        aim.wrapping_sub(c as u8)
                    };
                    let relative = at.wrapping_sub(desired);
                    if relative > 0x5F && relative < 0xA1 {
                        let spread = (0x80 - i32::from(relative)).abs();
                        let wanted = 0x20 - ((spread >> 1) & 0xFF);
                        if i32::from(clear_at(ship, at)) < wanted {
                            ship[CLEAR + usize::from(at)] = (0x20 - (spread >> 1)) as u8;
                        }
                    }
                }
            }
            let plus = aim.wrapping_add(c as u8);
            if clear_at(ship, plus) < best {
                best = clear_at(ship, plus);
                chosen = plus;
            }
            let minus = aim.wrapping_sub(c as u8);
            if clear_at(ship, minus) < best {
                best = clear_at(ship, minus);
                chosen = minus;
            }
            if c == i8::MIN {
                break;
            }
        }
    }
    if avoid && distance < 0xAF90 && dest_x > 100 && dest_y > 100 && dest_x < 0x219C && dest_y < 0x1DA0 {
        if distance < 0x24 {
            level_adjust(ship, -1, 4);
        } else {
            let mut turn_angle = chosen.wrapping_sub(ship[HEADING]);
            if turn_angle > 0x80 {
                turn_angle = turn_angle.wrapping_neg();
            }
            if turn_angle > 0x40 {
                turn_angle = 0x40u8.wrapping_sub(turn_angle);
            }
            let ratio = (table_sine(turn_angle) >> 8) as u32;
            let level = ship[LEVEL];
            if ratio == 0 {
                if level < 4 {
                    ship[LEVEL] += 1;
                }
            } else {
                let value = ((((distance.wrapping_mul(0x100)) as u32) / ratio) << 8) / ratio;
                let value = value as i32;
                if value < 0xAF90 {
                    if value < 0x80AE {
                        if value < 0x4DA9 {
                            if value < 0x1BE4 {
                                if level != 0 {
                                    ship[LEVEL] -= 1;
                                }
                            } else if level < 2 {
                                if level == 0 {
                                    ship[LEVEL] += 1;
                                }
                            } else {
                                ship[LEVEL] -= 1;
                            }
                        } else if level < 3 {
                            if level < 2 {
                                ship[LEVEL] += 1;
                            }
                        } else {
                            ship[LEVEL] -= 1;
                        }
                    } else if level < 4 {
                        if level < 3 {
                            ship[LEVEL] += 1;
                        }
                    } else {
                        ship[LEVEL] -= 1;
                    }
                } else if level < 4 {
                    ship[LEVEL] += 1;
                }
            }
        }
    }
    turn(ship, world, captains, direct, chosen)?;
    for b in &mut ship[CLEAR..CLEAR + 0x100] {
        *b = 0;
    }
    let _ = WORLD;
    Ok(())
}
