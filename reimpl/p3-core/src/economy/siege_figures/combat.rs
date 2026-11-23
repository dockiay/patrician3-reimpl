//! Final de las animaciones de las figuras (método `+8` del estado de animación `entidad + 0x2C`): reinicio del
//! contador de pasos (`0x0055FFC0`), golpe del luchador de la cola (`0x00577CB0`) y máquina de estados del
//! piquero (`0x00571350`).
//!
//! El estado de animación `a` está en `entidad + 0x2C`: `a+8` animación actual, `a+9` siguiente, `a+0xC/+0xE` paso
//! y pasos (`entidad+0x38/+0x3A`), `a+0x10` rumbo de destino (`+0x3C`), `a+0x3E` ciudad (`+0x6A`), `a+0x48` objetivo
//! y `a+0x4C` su clase de lista (`+0x74`, `+0x78`), `a+0x5C`/`a+0x60` pareja (`+0x88`/`+0x8C`) y `a+0x64` bando (`+0x90`).
use super::classes::pikeman_acquire;
use super::sprite::{anim_select, anim_set_default, walker_insert, walker_remove};
use super::update::seed_cell;
use super::walker::clear_mark;
use super::{dispatch, s16, siege_of, vcall, FigureHost, DIRS};
use crate::economy::siege_view::{reset_flow_field_to_cell, update_entry_point_markers};
use crate::flatmem::Mem;

fn r16(m: &mut dyn Mem, a: u32) -> u32 {
    u32::from(m.r16(a))
}

fn inc16(m: &mut dyn Mem, a: u32) {
    let v = m.r16(a).wrapping_add(1);
    m.w16(a, v);
}

/// `town_anim_state_reset_frame` (`0x0055FFC0`, y sus saltos `0x00567720`, `0x005626D0`, `0x00572DB0`): salvo en
/// las animaciones 2, `0x13` y `0xB` pone a cero el paso y los pasos; en `0x8C`, un paso.
pub fn anim_reset(m: &mut dyn Mem, a: u32) {
    let st = m.r8(a + 8);
    if !matches!(st, 2 | 0x13 | 0xB) {
        m.w16(a + 0xC, 0);
        m.w16(a + 0xE, 0);
    }
    if st == 0x8C {
        m.w16(a + 0xC, 0);
        m.w16(a + 0xE, 1);
    }
}

/// Registro de asedio de la ciudad del estado de animación.
fn record(m: &mut dyn Mem, a: u32) -> u32 {
    let city = m.r8(a + 0x3E);
    siege_of(m, city)
}

/// `0x00577CB0(ecx = estado, conjunto)`: al acabar el golpe (animación `0xF`) el luchador abate a su objetivo si
/// sigue en su lista y quedan bajas por cubrir: defensores (clase con `0x80`: listas del registro `+0x14..`,
/// cupo `+0x1004` menos bajas `+0x2C` y `+0x232`) o asaltantes (listas de la escena `+0x7C..`, cupo `+0x18` menos
/// bajas `+0x22` y `+0x21E`; cabecillas `+0x1F8` y jefe `+0x574` derriban `+0x2C`). Después reinicia el paso.
pub fn fighter_hit(m: &mut dyn Mem, a: u32, set: u32) {
    let rec = record(m, a);
    let s = m.r32(rec + 8);
    let who = m.r32(a + 0x48);
    if m.r8(a + 8) == 0xF {
        let kind = m.r8(a + 0x4C);
        if kind & 0x80 != 0 {
            let k = kind & 0x7F;
            m.w8(a + 0x4C, k);
            let k = u32::from(k);
            let count = r16(m, rec + 0x1004 + 2 * k);
            let done = r16(m, s + 0x2C + 2 * k).wrapping_add(r16(m, s + 0x232 + 4 * k));
            if (done as i32) < count as i32 && count != 0 {
                let mut e = set;
                let mut i = 0u8;
                loop {
                    let list = match k {
                        0 => Some(0x14),
                        1 => Some(0x410),
                        2 => Some(0x80C),
                        3 => Some(0xC08),
                        _ => None,
                    };
                    if let Some(l) = list {
                        e = m.r32(rec + l + 4 * u32::from(i));
                    }
                    if k != 4 && e == who {
                        if m.r8(who + 0x58) == 0 {
                            m.w8(who + 0x58, 1);
                            let k = u32::from(m.r8(a + 0x4C));
                            inc16(m, s + 0x2C + 2 * k);
                        }
                        break;
                    }
                    i = i.wrapping_add(1);
                    if u32::from(i) >= r16(m, rec + 0x1004 + 2 * k) {
                        break;
                    }
                }
            }
        } else if kind == 5 {
            if m.r32(s + 4) == 1 && m.r32(s + 0x574) == who && m.r8(who + 0x2C) == 0 {
                m.w8(who + 0x2C, 1);
                m.w32(s + 4, 0);
            }
        } else {
            let k = u32::from(kind);
            let n = r16(m, s + 0x18 + 2 * k);
            let done = r16(m, s + 0x22 + 2 * k).wrapping_add(r16(m, s + 0x21E + 4 * k));
            if (done as i32) < n as i32 && n != 0 {
                let mut e = set;
                let mut i = 0u8;
                loop {
                    let at = u32::from(i) * 4;
                    if kind == 4 {
                        let l = m.r32(s + 0x1F8 + at);
                        if l == who {
                            if m.r8(l + 0x2C) == 0 {
                                m.w8(l + 0x2C, 1);
                                let k = u32::from(m.r8(a + 0x4C));
                                inc16(m, s + 0x22 + 2 * k);
                            }
                            break;
                        }
                    } else {
                        match kind {
                            0 => e = m.r32(s + 0x7C + at),
                            1 => e = m.r32(s + 0x11C + at),
                            2 => e = m.r32(s + 0x174 + at),
                            3 => e = m.r32(s + 0x1BC + at),
                            _ => {}
                        }
                        if e == who {
                            if m.r8(who + 0x58) == 0 {
                                m.w8(who + 0x58, 1);
                                let k = u32::from(m.r8(a + 0x4C));
                                inc16(m, s + 0x22 + 2 * k);
                            }
                            break;
                        }
                    }
                    i = i.wrapping_add(1);
                    if u32::from(i) >= n {
                        break;
                    }
                }
            }
        }
    }
    anim_reset(m, a);
}

/// Señal de victoria del bando (`0x00590610`, clase `0x12` variante `0x81`, desvanecida en `0xEA60` ms) y giro
/// de vuelta (`0x97`).
fn victory_or_wait(m: &mut dyn Mem, h: &mut dyn FigureHost, a: u32, s: u32) -> u8 {
    if m.r8(s + 0x79) != 0 {
        return 0xF6;
    }
    if m.r16(s + 0x3A) == 0 {
        return 0x18;
    }
    let (x, y, hd) = (m.r32(a - 0x20), m.r32(a - 0x1C), u32::from(m.r8(a - 0x18)));
    let e = h.spawn_entity(m, [x, y, 0x12, 0x81, hd, 0, 0]);
    dispatch(m, h, 0x0057_AF40, e, &[0xEA60, 0xFFFF_FF80]);
    0x97
}

/// `siege_fighter_on_anim_finished` (`0x00571350(ecx = estado, conjunto)`): estados del piquero (clase 4) al acabar
/// cada animación: llegada al punto de reunión (5: se orienta a `+0x5D2` y espera `0x96` reorientando el campo),
/// golpe a la cabeza de la cola (`0xF`), espera (`0x19`/`0x95`/`0x18`/`0x96`), entrega de la pareja (`0x91`:
/// la tumba con `0xB1` y deja una señal `0x82`; `0xF5`: la levanta), giro (`0x97`) y huida (`0xF6`). Después
/// reinicia el paso y, salvo con la siguiente animación 2, 5, `0xB` o `0xD`, deja un paso.
pub fn pikeman_anim_finished(m: &mut dyn Mem, h: &mut dyn FigureHost, a: u32, set: u32) {
    let rec = record(m, a);
    let s = m.r32(rec + 8);
    let w = a - 0x2C;
    let side = u32::from(m.r8(a + 0x64));
    let head = |m: &mut dyn Mem| m.r32(rec.wrapping_add(0x10).wrapping_sub(4 * side));
    let next: Option<u8> = match m.r8(a + 8) {
        5 => {
            let mut next = None;
            if s != 0 && m.r8(s + 0x78) == 0 {
                let (px, py) = (s16(m, a - 0x1E), s16(m, a - 0x1A));
                let (c, r) = h.pixel_to_cell(m, px, py);
                if c == r16(m, s + 0x5CA + 2 * side) && r == r16(m, s + 0x5CE + 2 * side) {
                    let d = m.r8(s + 0x5D2);
                    m.w8(a + 0x10, d);
                    if d != m.r8(a - 0x18) {
                        next = Some(5);
                    } else {
                        anim_set_default(m, a, 0x96);
                        let (x, y) = (r16(m, s + 0x5CC - 2 * side), r16(m, s + 0x5D0 - 2 * side));
                        reset_flow_field_to_cell(m, h, s, x, y, 0);
                        let (sx, sy) = (r16(m, s + 0xC), r16(m, s + 0xE));
                        seed_cell(m, h, s, sx, sy, 0);
                        update_entry_point_markers(m, h, s, true);
                    }
                }
            }
            next
        }
        st @ (0xF | 0x19 | 0x95) => {
            if st == 0xF {
                let e = head(m);
                if e != 0 && m.r8(e + 0x5A) == 0 && m.r8(e + 0x58) == 0 {
                    let k = u32::from(vcall(m, h, e, 0x68, &[]) as u8);
                    let left = r16(m, rec + 0x1004 + 2 * k).wrapping_sub(r16(m, s + 0x2C + 2 * k));
                    if (r16(m, s + 0x232 + 4 * k) as i32) < left as i32 {
                        let e = head(m);
                        m.w8(e + 0x58, 1);
                        inc16(m, s + 0x2C + 2 * k);
                    }
                }
            }
            if s == 0 || m.r8(s + 0x78) != 0 {
                Some(0xF5)
            } else {
                let e = head(m);
                if e != 0 && m.r8(e + 0x58) == 0 && m.r8(e + 0x5A) == 0 {
                    Some(0xF)
                } else if e != 0 {
                    Some(0x19)
                } else {
                    Some(0xF5)
                }
            }
        }
        st @ (0x18 | 0x96) => {
            if st == 0x96 && s != 0 {
                let (px, py) = (s16(m, a - 0x1E), s16(m, a - 0x1A));
                let (c, r) = h.pixel_to_cell(m, px, py);
                clear_mark(m, s, c, r, 1);
            }
            if s != 0 && m.r8(s + 0x78) == 0 {
                if m.r32(a + 0x5C) == 0 {
                    pikeman_acquire(m, w);
                }
                Some(0x18)
            } else {
                Some(victory_or_wait(m, h, a, s))
            }
        }
        0x91 => {
            let p = m.r32(a + 0x60);
            let set_p = vcall(m, h, p, 0x4C, &[]);
            let rect = walker_remove(m, h, p, set_p, true);
            m.w8(p + 0x58, 1);
            m.w16(p + 0x68, 1);
            m.w8(p + 0x14, DIRS[5]);
            let set_p = vcall(m, h, p, 0x4C, &[]);
            let f = anim_select(m, p + 0x2C, set_p, 0xB1);
            m.w8(p + 0x15, f);
            let set_p = vcall(m, h, p, 0x4C, &[]);
            walker_insert(m, h, p, set_p, rect);
            let (x, y, hd) = (m.r32(a - 0x20), m.r32(a - 0x1C), u32::from(m.r8(a - 0x18)));
            m.w32(a + 0x60, 0);
            m.w8(a + 0x2C, 0);
            let e = h.spawn_entity(m, [x, y, 0x12, 0x82, hd, 0, 0]);
            dispatch(m, h, 0x0057_AF40, e, &[0, 0xFFFF_FFC1]);
            Some(0x18)
        }
        0x97 => {
            let rect = walker_remove(m, h, w, set, true);
            let d = m.r8(a - 0x18).wrapping_add(0x80);
            m.w8(a - 0x18, d);
            walker_insert(m, h, w, set, rect);
            Some(0xD)
        }
        0xF5 => {
            let p = m.r32(a + 0x60);
            let set_p = vcall(m, h, p, 0x4C, &[]);
            let rect = walker_remove(m, h, p, set_p, true);
            let hd = m.r8(a - 0x18);
            m.w8(p + 0x14, hd);
            let set_p = vcall(m, h, p, 0x4C, &[]);
            let f = anim_select(m, p + 0x2C, set_p, 2);
            m.w8(p + 0x15, f);
            let set_p = vcall(m, h, p, 0x4C, &[]);
            walker_insert(m, h, p, set_p, rect);
            let p = m.r32(a + 0x60);
            m.w32(p + 0x8C, 0);
            m.w32(a + 0x60, 0);
            if s != 0 && m.r8(s + 0x78) == 0 {
                Some(0x18)
            } else {
                Some(victory_or_wait(m, h, a, s))
            }
        }
        0xF6 => {
            m.w32(a + 0x60, 0);
            Some(5)
        }
        _ => None,
    };
    if let Some(n) = next {
        anim_set_default(m, a, n);
    }
    anim_reset(m, a);
    if !matches!(m.r8(a + 9), 2 | 5 | 0xB | 0xD) {
        m.w16(a + 0xC, 0);
        m.w16(a + 0xE, 1);
    }
}
