//! Bucle por fotograma del caminante común (`town_walker_update`, `0x0055E8C0`), que llaman todas las clases de
//! figuras de la batalla con sus argumentos `(conjunto, bits, umbral, mínimo, a4, reglas)`.
//!
//! Ramas, en el orden del original:
//! * efecto de escalera (`+0x64`): se retira al caer (`+0x58`) o, en la cabeza de una cola de puerta con objetivo,
//!   lo fija y termina;
//! * avance de la animación, pisadas (`0x0055D0B0`) y siguiente instante de la animación (`+0x30`);
//! * figuras de cola (`+0x59`): siguen solo junto a una puerta de la muralla (registros de edificio);
//! * estado `0xB1` (espera con cuenta atrás `+0x68`) y caída (`+0x58`, animación `0x91` con sonido);
//! * paseo por la ciudad (bando 1 en calle sin asalto): giros de 0x20, rodeos con el rumbo de reserva `+0x3D` y
//!   reglas `+0x24`, avance por pasos y bloqueo por otras figuras (`0x00593580`);
//! * asalto: salida por los bordes, llegada al destino (marcas, cola de puerta `0x00560CB0`, método `+0x58` de
//!   búsqueda de objetivo, campo de distancias `0x0055FFF0`, reasignación del campo `0x0062B860/0x0062B960/
//!   0x0062DD70`) y animaciones de los estados (tablas `0x0055FDEC` y `0x0055FED8`);
//! * avance final del paso y repintado con giro (`0x0055A1C0`).
//!
//! Las llamadas directas (pareja `0x00579380` y nivel de muralla `0x005252F0`, en [`super::fighter`]) van por
//! [`super::dispatch`].
use super::sprite::{anim_find, anim_select, anim_set_default, walker_insert, walker_remove};
use super::step::{begin_move, check_step, heading_to_target, single_occupant, turn_toward};
use super::walker::{begin_step, clear_mark, pick_next_tile, pick_queue_step, select_state_animation};
use super::{dispatch, ld32, s16, siege_active, siege_of, vcall, view, FigureHost, CLOCK, DIRS, TOWNS};
use crate::economy::siege_battle::{building_record, cell_walkable, flow_distance};
use crate::economy::siege_view::{ftol_sqrt, reset_flow_field_to_cell, seed_flow_field, update_entry_point_markers};
use crate::flatmem::Mem;

/// `0x00571ED0(ecx = figura)`: ¿la cabeza de la cola de su lado (`+0x90`) está quieta en su puesto?
pub fn can_engage(m: &mut dyn Mem, p: u32) -> bool {
    let city = m.r8(p + 0x6A);
    let rec = siege_of(m, city);
    let s = m.r32(rec + 8);
    if s == 0 || m.r8(s + 0x78) != 0 {
        return false;
    }
    let side = u32::from(m.r8(p + 0x90));
    let head = m.r32(rec.wrapping_add(0x10).wrapping_sub(4 * side));
    head != 0 && m.r8(head + 0x58) == 0 && m.r8(head + 0x5A) == 0
}

/// `0x006319D0(ecx = escena)`: distancia (columnas dobles) de la casilla de la entidad seguida `+0x574` al destino;
/// `0xFFFF` sin entidad.
pub fn tracked_distance(m: &mut dyn Mem, h: &mut dyn FigureHost, s: u32) -> u32 {
    let e = m.r32(s + 0x574);
    if e == 0 {
        return 0xFFFF;
    }
    let (x, y) = (s16(m, e + 0xE), s16(m, e + 0x12));
    let (c, r) = h.pixel_to_cell(m, x, y);
    let dx = s16(m, s + 8).wrapping_sub(i32::from(c as u16 as i16)).wrapping_mul(2);
    let dy = s16(m, s + 0xA).wrapping_sub(i32::from(r as u16 as i16));
    ftol_sqrt(dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx)))
}

/// `0x0062B960(ecx = escena, a, b, ancho)`: siembra el campo desde `(a, b)` sin cambiar el destino `+0x10/+0x12`.
pub fn seed_cell(m: &mut dyn Mem, h: &mut dyn FigureHost, s: u32, a: u32, b: u32, wide: u32) {
    let v = view(m);
    if a & 0xFFFF >= m.r32(v + 0xC314) || b & 0xFFFF >= m.r32(v + 0xC318) {
        return;
    }
    let (ox, oy) = (m.r16(s + 0x10), m.r16(s + 0x12));
    m.w16(s + 0x10, a as u16);
    m.w16(s + 0x12, b as u16);
    seed_flow_field(m, h, s, wide as u8 != 0);
    m.w16(s + 0x10, ox);
    m.w16(s + 0x12, oy);
}

/// Reasigna el campo hacia el punto de reunión `lado` de la escena (`+0x5CA/+0x5CE`) si no es ya el destino.
fn retarget(m: &mut dyn Mem, h: &mut dyn FigureHost, s: u32, side: u8) {
    let i = u32::from(side) * 2;
    let (tx, ty) = (m.r16(s + i + 0x5CA), m.r16(s + i + 0x5CE));
    if m.r16(s + 0x10) == tx && m.r16(s + 0x12) == ty {
        return;
    }
    reset_flow_field_to_cell(m, h, s, u32::from(tx), u32::from(ty), 0);
    let (a, b) = (m.r16(s + 0xC), m.r16(s + 0xE));
    seed_cell(m, h, s, u32::from(a), u32::from(b), 0);
    update_entry_point_markers(m, h, s, true);
}

/// Bit de calle del plano de la ciudad del caminante.
fn street(m: &mut dyn Mem, w: u32, col: u32, row: u32) -> bool {
    let town = town_of(m, w);
    let stride = m.r32(town + 0x7AC);
    let plan = m.r32(town + 0x80C);
    m.r8(stride.wrapping_mul(col).wrapping_add(plan).wrapping_add(row)) & 0x80 != 0
}

fn town_of(m: &mut dyn Mem, w: u32) -> u32 {
    let city = m.r8(w + 0x6A);
    m.r32(TOWNS).wrapping_add(u32::from(city) * 0x9F8)
}

/// ¿El paso en `dir` está libre con objetos y figuras (`0x0055A2C0` sobre las reglas `+0x20`)?
fn clear(m: &mut dyn Mem, h: &mut dyn FigureHost, w: u32, dir: u8) -> bool {
    check_step(m, h, w + 0x20, w + 8, dir, true, true)
}

/// ¿Asalto en marcha en el bucle de la figura (`0x00633AE0` y escena `+0x3A == 0`)?
fn assault_running(m: &mut dyn Mem, rec: u32) -> bool {
    siege_active(m, rec) && super::ld16(m, rec + 8, 0x3A) == 0
}

/// `0x0055E8C0(ecx = caminante, conjunto, bits, umbral, mínimo, a4, reglas)`. Devuelve `AL` (0: la figura deja
/// la vista o se queda sin paso).
#[allow(clippy::too_many_arguments)]
pub fn walker_update(m: &mut dyn Mem, h: &mut dyn FigureHost, w: u32, set: u32, bits: u32, threshold: u32, min: u32, a4: u32, rules: u32) -> u8 {
    let mut rules = rules;
    let city = m.r8(w + 0x6A);
    let rec = siege_of(m, city);
    let eff = m.r32(w + 0x64);
    if eff != 0 {
        if m.r8(w + 0x58) != 0 {
            m.w32(w + 0x74, 0);
            h.release_entity(m, eff);
        } else {
            let town = town_of(m, w);
            if m.r8(w + 0x59) != 0 && m.r32(town + 0x40) != 0 {
                if m.r32(rec + 0xC) == w {
                    let t = ld32(m, rec + 8, 0x214);
                    if m.r32(t.wrapping_add(0x8C)) != 0 {
                        m.w32(w + 0x74, t);
                        return 1;
                    }
                }
                if m.r32(rec + 0x10) == w {
                    let t = ld32(m, rec + 8, 0x210);
                    if m.r32(t.wrapping_add(0x8C)) != 0 {
                        m.w32(w + 0x74, t);
                        return 1;
                    }
                }
            }
            anim_set_default(m, eff + 0x20, 2);
            return 1;
        }
    }
    if rules as u8 == 0 {
        let v = if a4 as u8 != 0 { bits & 0xFF } else { 1 };
        rules = (rules & !0xFF) | v;
    }
    let anim = w + 0x2C;
    let mut frame = super::sprite::anim_advance(m, h, anim, set, w + 8, 0);
    if frame != m.r8(w + 0x15) {
        if vcall(m, h, w, 0x60, &[0]) as u8 != 0 {
            let flag = u32::from(m.r8(w + 0x59));
            let (dx, dy) = (s16(m, w + 0x82) as u32, s16(m, w + 0x84) as u32);
            dispatch(m, h, 0x0055_D0B0, w, &[set, flag, dx, dy]);
        }
        if frame != m.r8(w + 0x15) {
            let clock = m.r32(CLOCK);
            let speed = u32::from(m.r8(w + 0x86));
            let mut t = m.r32(w + 0x30).wrapping_sub(clock).wrapping_mul(speed).wrapping_add(0x40) >> 7;
            let st = m.r8(w + 0x34);
            if matches!(st, 5 | 0xD | 0x18 | 0x19) {
                t = t.wrapping_shl(2);
            }
            if st == 2 && m.r8(w + 0x59) != 0 && assault_running(m, rec) {
                t >>= 1;
            }
            let clock = m.r32(CLOCK);
            m.w32(w + 0x30, clock.wrapping_add(t));
        }
    }
    let speed = u32::from(m.r8(w + 0x86));
    let v = view(m);
    let mut stepn = m.r32(v + 0x414).wrapping_shl(7).wrapping_add(speed >> 1) / speed;
    if m.r8(w + 0x34) == 2 && m.r8(w + 0x59) != 0 && assault_running(m, rec) {
        stepn = stepn.wrapping_mul(2);
    }
    let (px, py) = (s16(m, w + 0xE), s16(m, w + 0x12));
    let (col, row) = h.pixel_to_cell(m, px, py);
    if m.r8(w + 0x59) != 0 {
        let busy = vcall(m, h, w, 0x60, &[1]) as u8 != 0;
        if !busy && m.r8(w + 0x5A) > 0x28 && siege_active(m, rec) {
            return 0;
        }
        let town = town_of(m, w);
        let first = u32::from(m.r16(town + 0x768));
        let mut b = building_record(m, town, first);
        loop {
            if b == 0 {
                return 0;
            }
            let near = (i32::from(m.r8(b)).wrapping_sub(col as i32)).wrapping_abs() < 3
                && (i32::from(m.r8(b + 1)).wrapping_sub(row as i32)).wrapping_abs() < 3
                && m.r8(b + 6) == 0
                && !matches!(m.r8(b + 3), 0x30 | 0x31);
            if near {
                let level = dispatch(m, h, 0x0052_52F0, town, &[]) & 0xFF;
                if u32::from(m.r8(b + 2)) == 0xFF - level {
                    break;
                }
            }
            let next = m.r16(b + 4);
            if next >= m.r16(town + 0x776) {
                return 0;
            }
            b = building_record(m, town, u32::from(next));
        }
    }
    let scene = m.r32(rec + 8);
    let state = m.r8(w + 0x34);
    if state == 0xB1 {
        let t = m.r16(w + 0x68);
        let wait = stepn.wrapping_shl(4);
        let shrink = if t == 0 {
            m.w16(w + 0x68, 0xFFFF);
            true
        } else if u32::from(t) > wait {
            m.w16(w + 0x68, t.wrapping_sub(wait as u16));
            true
        } else {
            false
        };
        if shrink {
            let r = walker_remove(m, h, w, set, true);
            walker_insert(m, h, w, set, r);
            return 1;
        }
        let (c, r) = h.pixel_to_cell(m, px, py);
        let (dc, dr) = (u32::from(m.r16(w + 0x6C)), u32::from(m.r16(w + 0x6E)));
        clear_mark(m, scene, dc, dr, bits as u8);
        if m.r8(w + 0x70) != 0 {
            clear_mark(m, scene, c, r, bits as u8);
            m.w8(w + 0x70, 0);
        }
        return 0;
    }
    if m.r8(w + 0x58) != 0 {
        let f = if state == 0x91 {
            frame
        } else {
            if vcall(m, h, w, 0x68, &[]) as u8 != 4 {
                m.w8(w + 0x3C, DIRS[5]);
            }
            let f = anim_select(m, anim, set, 0x91);
            if h.sound_list_size(m, 0xEE48) != 0 {
                let n = h.sound_list_size(m, 0xEE48) as i32;
                let r = h.rand(m) as i32;
                let id = r.wrapping_rem(n).wrapping_sub(0x11B8);
                let (x, y) = (s16(m, w + 0xE), s16(m, w + 0x12));
                h.play_sound(m, x, y, id as u32);
            }
            f
        };
        let r = walker_remove(m, h, w, set, true);
        let d = m.r8(w + 0x3C);
        m.w8(w + 0x14, d);
        m.w8(w + 0x15, f);
        walker_insert(m, h, w, set, r);
        return 1;
    }
    if !siege_active(m, rec)
        && scene != 0
        && m.r16(scene + 0x3A) != 0
        && m.r8(w + 0x7B) == 1
        && m.r16(scene + 0x10) == m.r16(scene + 0x36)
        && m.r16(scene + 0x12) == m.r16(scene + 0x38)
        && m.r8(w + 0x59) == 0
        && !street(m, w, col, row)
    {
        return 0;
    }
    let assault = siege_active(m, rec)
        && !(m.r16(scene + 0x3A) != 0 && m.r16(scene + 0x10) == m.r16(scene + 0x3E) && m.r16(scene + 0x12) == m.r16(scene + 0x40));
    if !assault {
        if m.r8(w + 0x7B) != 1 {
            return 0;
        }
        if street(m, w, col, row) && m.r8(w + 0x59) == 0 {
            return stroll(m, h, w, set, bits, stepn, col, row, frame, scene);
        }
    }
    // Asalto (`0x0055F4E3`).
    if frame != m.r8(w + 0x15) {
        frame = vcall(m, h, w, 0x64, &[u32::from(frame)]) as u8;
    }
    let (px, py) = (s16(m, w + 0xE), s16(m, w + 0x12));
    let (col, row) = h.pixel_to_cell(m, px, py);
    if m.r8(w + 0x7B) & 1 == 0 {
        let s36 = m.r16(scene + 0x36);
        if m.r16(scene + 0x10) == s36 {
            let leave = if m.r16(scene + 0x12) == s36 && col == u32::from(s36) && row == u32::from(s36) {
                true
            } else if m.r16(scene + 0x12) == m.r16(scene + 0x38) {
                let v = view(m);
                let (wd, ht) = (m.r32(v + 0xC314).wrapping_sub(3), m.r32(v + 0xC318).wrapping_sub(5));
                col > wd || col < 1 || row > ht || row < 4
            } else {
                false
            };
            if leave {
                clear_mark(m, scene, col, row, bits as u8);
                let c = vcall(m, h, w, 0x68, &[]) & 0xFF;
                if m.r16(scene + c * 4 + 0x21E) == 0 {
                    return 0;
                }
                let c = vcall(m, h, w, 0x68, &[]) & 0xFF;
                let at = scene + c * 4 + 0x21E;
                let n = m.r16(at).wrapping_sub(1);
                m.w16(at, n);
                return 0;
            }
        }
    }
    if m.r16(w + 0x38) == m.r16(w + 0x3A) {
        if let Some(r) = arrive(m, h, w, set, bits, threshold, min, a4, &mut rules, col, row, scene, &mut frame) {
            return r;
        }
    }
    finish(m, h, w, set, stepn, frame)
}

/// Llegada al final del paso en el asalto (`0x0055F5ED..0x0055FCEF`): fija `frame` y deja el resto a [`finish`]
/// (`None`) o devuelve el valor con que vuelve el original.
#[allow(clippy::too_many_arguments)]
fn arrive(m: &mut dyn Mem, h: &mut dyn FigureHost, w: u32, set: u32, bits: u32, threshold: u32, min: u32, a4: u32, rules: &mut u32, col: u32, row: u32, scene: u32, frame: &mut u8) -> Option<u8> {
    let anim = w + 0x2C;
    let city = m.r8(w + 0x6A);
    let dist: u16 = if scene != 0 {
        if m.r8(w + 0x60) != 0 {
            m.w16(w + 0x80, 0);
        } else {
            let d = tracked_distance(m, h, scene) as u16;
            m.w16(w + 0x80, d);
            if d > 0x19 && m.r8(scene + 0x78) == 0 {
                m.w16(w + 0x80, 0x19);
            }
        }
        flow_distance(m, h, scene, col, row)
    } else {
        a4 as u16
    };
    if m.r8(w + 0x7B) & 1 == 0
        && m.r16(scene + 0x10) == m.r16(scene + 0x3E)
        && m.r16(scene + 0x12) == m.r16(scene + 0x40)
        && col == u32::from(m.r16(scene + 0x3E))
        && row == u32::from(m.r16(scene + 0x40))
    {
        clear_mark(m, scene, col, row, bits as u8);
        return Some(0);
    }
    // `moved` vive en la ranura del sexto argumento; `None` salta directamente a la tabla de los estados.
    enum Next {
        Table,
        Choice(u8, bool),
    }
    let next = if col == u32::from(m.r16(w + 0x6C)) && row == u32::from(m.r16(w + 0x6E)) {
        if m.r8(w + 0x59) != 0 {
            let moved = pick_queue_step(m, h, w, set, col, row, bits as u8);
            *rules = (*rules & !0xFF) | u32::from(moved);
            if moved {
                Next::Choice(0, true)
            } else {
                let bl = vcall(m, h, w, 0x58, &[1]) as u8;
                Next::Choice(bl, false)
            }
        } else {
            let bl;
            let class = vcall(m, h, w, 0x68, &[]) as u8;
            if class == 0 && m.r32(w + 0x8C) != 0 {
                bl = dispatch(m, h, 0x0057_9380, w, &[]) as u8;
                if bl == 0 {
                    let p = m.r32(w + 0x8C);
                    if p != 0 {
                        if m.r32(p + 0x8C) == 0 && can_engage(m, p) {
                            m.w16(w + 0x80, 0);
                        }
                        let rec = siege_of(m, city);
                        let s2 = m.r32(rec + 8);
                        if m.r8(s2 + 0x78) == 0 {
                            let partner = m.r32(w + 0x8C);
                            let side = m.r8(partner + 0x90);
                            retarget(m, h, s2, side);
                        }
                    }
                }
            } else {
                let cond = dist == min as u16 || (dist <= threshold as u16 && vcall(m, h, w, 0x68, &[]) as u8 == 4);
                bl = vcall(m, h, w, 0x58, &[u32::from(cond)]) as u8;
                if vcall(m, h, w, 0x68, &[]) as u8 == 4
                    && bl == 0
                    && m.r8(scene + 0x6F) == 0xFF
                    && m.r16(scene + 0x42) < m.r16(scene + 0x44)
                    && m.r16(scene + 0x3A) == 0
                    && m.r16(scene + 0x3C) == 0
                    && flow_distance(m, h, scene, col, row) < 0x14
                {
                    let side = m.r8(w + 0x90);
                    retarget(m, h, scene, side);
                }
            }
            if bl != 0 {
                Next::Choice(bl, bl == 2)
            } else {
                let moved = pick_next_tile(m, h, w, set, col, row, min as u16, bits as u8, *rules, threshold as u16);
                *rules = (*rules & !0xFF) | u32::from(moved);
                Next::Choice(0, moved)
            }
        }
    } else {
        let (dc, dr) = (u32::from(m.r16(w + 0x6C)), u32::from(m.r16(w + 0x6E)));
        begin_step(m, h, w, dc, dr, set);
        Next::Table
    };
    let (bl, moved) = match next {
        Next::Table => (0, true),
        Next::Choice(bl, moved) => {
            if bl == 1 || bl == 4 {
                *frame = select_state_animation(m, h, w, set, bl == 4) as u8;
                return None;
            }
            (bl, moved)
        }
    };
    let a4b = a4 as u8;
    let reset = |m: &mut dyn Mem| {
        m.w16(w + 0x3A, 1);
        m.w16(w + 0x38, 0);
    };
    if moved {
        // Tabla `0x0055FDEC`.
        let pick_b = |m: &mut dyn Mem| if anim_find(m, set, 0x8C) != 0xFF { 0x8C } else { 0xB };
        let low_b = |m: &mut dyn Mem, frame: &mut u8| {
            if anim_find(m, set, 0x13) != 0xFF {
                *frame = anim_select(m, anim, set, 0x13);
            } else if anim_find(m, set, 0x12) != 0xFF || anim_find(m, set, 0xC) != 0xFF {
                let code = if anim_find(m, set, 0x12) != 0xFF { 0x92 } else { 0xEC };
                *frame = anim_select(m, anim, set, code);
                anim_set_default(m, anim, 0xCC);
                reset(m);
            } else {
                *frame = anim_select(m, anim, set, 2);
                reset(m);
            }
        };
        match m.r8(w + 0x34) {
            2 => {
                if a4b != 0 {
                    let code = pick_b(m);
                    *frame = anim_select(m, anim, set, code);
                    reset(m);
                }
            }
            0x13 => {
                if a4b != 0 {
                    *frame = anim_select(m, anim, set, 0xB);
                }
            }
            0xB => {
                if a4b == 0 {
                    low_b(m, frame);
                }
            }
            0x05 | 0xA4 | 0xB2 | 0xCC => {
                if a4b != 0 {
                    let code = pick_b(m);
                    *frame = anim_select(m, anim, set, code);
                    reset(m);
                } else {
                    *frame = anim_select(m, anim, set, 2);
                }
            }
            0x0D | 0x0F | 0x14 | 0xAC | 0xCE => {
                if a4b != 0 {
                    *frame = anim_select(m, anim, set, 0xB);
                } else {
                    low_b(m, frame);
                }
            }
            0x10 | 0xAE => {
                *frame = anim_select(m, anim, set, 0xEE);
                reset(m);
            }
            _ => {}
        }
        if m.r8(w + 0x70) != 0 && matches!(m.r8(w + 0x34), 2 | 0xB | 0x13) {
            if scene != 0 {
                clear_mark(m, scene, col, row, bits as u8);
            }
            if m.r8(w + 0x59) != 0 {
                let pos = m.r8(w + 0x5A);
                let pos = if m.r8(w + 0x3C) == m.r8(w + 0x79) { pos.wrapping_add(1) } else { pos.wrapping_sub(1) };
                m.w8(w + 0x5A, pos);
            }
            m.w8(w + 0x70, 0);
        }
        return None;
    }
    let mut bl = bl;
    let mut rest = false;
    if bl == 0 {
        let k1 = m.r8(w + 0x7B) & 1;
        if m.r8(w + 0x59) != 0 || (k1 != 0 && scene != 0 && m.r8(scene + 0x79) != 0) || (threshold as u16 > min as u16 && k1 == 0) {
            rest = true;
        } else {
            bl = vcall(m, h, w, 0x58, &[1]) as u8;
        }
    }
    if !rest && (bl == 1 || bl == 4) {
        *frame = select_state_animation(m, h, w, set, bl == 4) as u8;
        return None;
    }
    // Quieto (`0x0055FC01`, tabla `0x0055FED8`).
    reset(m);
    match m.r8(w + 0x34) {
        0x02 | 0x05 | 0xA4 | 0xB2 | 0xCC => *frame = anim_select(m, anim, set, 5),
        0x0B | 0x0D | 0x0F | 0x13 | 0x14 | 0xAC | 0xCE => {
            if anim_find(m, set, 0xD) != 0xFF {
                *frame = anim_select(m, anim, set, 0xD);
            } else {
                let code = if anim_find(m, set, 0x12) != 0xFF { 0x92 } else { 0xEC };
                *frame = anim_select(m, anim, set, code);
                anim_set_default(m, anim, 0xCC);
            }
        }
        0x10 | 0xAE => *frame = anim_select(m, anim, set, 0xEE),
        _ => {}
    }
    None
}

/// Avance final del paso y repintado (`0x0055FCF3`).
fn finish(m: &mut dyn Mem, h: &mut dyn FigureHost, w: u32, set: u32, stepn: u32, frame: u8) -> u8 {
    let mut x = m.r32(w + 0xC);
    let mut y = m.r32(w + 0x10);
    if m.r8(w + 0x14) == m.r8(w + 0x3C) {
        let st = m.r8(w + 0x34);
        let mut n = if st == 0xB { stepn.wrapping_mul(2) } else { stepn };
        if matches!(st, 2 | 0x13 | 0xB) {
            let step = m.r16(w + 0x38);
            let left = u32::from(m.r16(w + 0x3A)).wrapping_sub(u32::from(step));
            if n > left {
                n = left;
            }
            m.w16(w + 0x38, step.wrapping_add(n as u16));
            x = x.wrapping_add(m.r32(w + 0x40).wrapping_mul(n));
            y = y.wrapping_add(m.r32(w + 0x44).wrapping_mul(n));
        }
    }
    if (x >> 16) as i32 == s16(m, w + 0xE) && (y >> 16) as i32 == s16(m, w + 0x12) && frame == m.r8(w + 0x15) {
        m.w32(w + 0xC, x);
        m.w32(w + 0x10, y);
        return 1;
    }
    let r = walker_remove(m, h, w, set, true);
    let target = m.r8(w + 0x3C);
    m.w32(w + 0xC, x);
    m.w32(w + 0x10, y);
    if m.r8(w + 0x14) != target && m.r8(w + 0x15) != frame && !matches!(m.r8(w + 0x34), 0x8E | 0xEE) {
        let n = m.r8(set + 0x14);
        turn_toward(m, w + 0x14, target, n);
    }
    m.w8(w + 0x15, frame);
    walker_insert(m, h, w, set, r);
    1
}

/// Paseo por la ciudad fuera del asalto (`0x0055EEF9..0x0055F4E0`).
#[allow(clippy::too_many_arguments)]
fn stroll(m: &mut dyn Mem, h: &mut dyn FigureHost, w: u32, set: u32, bits: u32, stepn: u32, col: u32, row: u32, frame: u8, scene: u32) -> u8 {
    let anim = w + 0x2C;
    if scene != 0 {
        clear_mark(m, scene, col, row, bits as u8);
        let c = u32::from(m.r16(w + 0x6C));
        // El original pasa la columna de destino como columna y como fila.
        clear_mark(m, scene, c, c, bits as u8);
    }
    if m.r8(w + 0x14) != m.r8(w + 0x3C) {
        if frame != m.r8(w + 0x15) {
            let r = walker_remove(m, h, w, set, true);
            let h14 = m.r8(w + 0x14);
            if (m.r8(w + 0x3C).wrapping_sub(h14) as i8) < 0 {
                m.w8(w + 0x57, 0);
                m.w8(w + 0x14, h14.wrapping_sub(0x20));
                let n = m.r8(w + 0x56).wrapping_add(1);
                m.w8(w + 0x56, n);
            } else {
                m.w8(w + 0x56, 0);
                m.w8(w + 0x14, h14.wrapping_add(0x20));
                let n = m.r8(w + 0x57).wrapping_add(1);
                m.w8(w + 0x57, n);
            }
            m.w8(w + 0x15, frame);
            walker_insert(m, h, w, set, r);
        }
        anim_set_default(m, anim, 2);
        m.w8(w + 0x55, 3);
        if (m.r8(w + 0x56) > 3 || m.r8(w + 0x57) > 3) && vcall(m, h, w, 0x60, &[1]) as u8 == 0 {
            return 0;
        }
        return 1;
    }
    if m.r16(w + 0x38) != m.r16(w + 0x3A) {
        return stroll_step(m, h, w, set, stepn, frame);
    }
    if m.r8(w + 0x56) > 2 || m.r8(w + 0x57) > 2 {
        dispatch(m, h, 0x0055_D9D0, w, &[]);
    }
    let hd = heading_to_target(m, w);
    let straight = m.r8(w + 0x3D) == m.r8(w + 0x14);
    // La casilla de delante es la del rumbo dibujado: el original carga `+0x14` en `AL` antes de pasarlo.
    let (mut c, mut r) = (col, row);
    let ahead = m.r8(w + 0x14);
    h.step(m, &mut c, &mut r, ahead);
    let link = m.r16(w + 0x16);
    let occ = single_occupant(m, h, w, c, r, link);
    if straight {
        let h14 = m.r8(w + 0x14);
        if (occ != 0 && m.r8(occ + 0x14) == h14.wrapping_add(0x80)) || clear(m, h, w, h14) {
            m.w8(w + 0x3C, h14);
            m.w16(w + 0x6C, c as u16);
            m.w16(w + 0x6E, r as u16);
            begin_move(m, h, w, c, r, set);
            anim_set_default(m, anim, 2);
            return 1;
        }
        if occ != 0 && matches!(m.r8(occ + 0x34), 2 | 6) {
            let f = if m.r8(w + 0x34) != 5 {
                let f = anim_select(m, anim, set, 5);
                if f == 0xFF {
                    m.r8(w + 0x15)
                } else {
                    f
                }
            } else {
                frame
            };
            let h14 = m.r8(w + 0x14);
            m.w8(w + 0x3C, h14);
            begin_move(m, h, w, c, r, set);
            let rr = walker_remove(m, h, w, set, true);
            m.w8(w + 0x15, f);
            walker_insert(m, h, w, set, rr);
            return 1;
        }
    }
    // Rodeos: primero con las reglas como están; luego sin respetar puertas (`+0x24 = 0`, a 1 si sale).
    let mut chosen = None;
    for round in 0..2 {
        if round == 1 {
            m.w8(w + 0x24, 0);
        }
        let h14 = m.r8(w + 0x14);
        let d = m.r8(w + 0x3D);
        let cands = [(d.wrapping_sub(0x80) != h14, d), (hd.wrapping_sub(0x80) != h14, hd), (hd != h14, hd.wrapping_add(0x80))];
        for (ok, dir) in cands {
            if ok && clear(m, h, w, dir) {
                chosen = Some(dir);
                break;
            }
        }
        if let Some(dir) = chosen {
            let h14 = m.r8(w + 0x14);
            m.w8(w + 0x3C, dir);
            m.w8(w + 0x3D, h14);
            if round == 1 {
                m.w8(w + 0x24, 1);
            }
            break;
        }
    }
    if chosen.is_none() {
        m.w8(w + 0x24, 1);
        dispatch(m, h, 0x0055_D9D0, w, &[]);
        heading_to_target(m, w);
        let d = m.r8(w + 0x3D);
        m.w8(w + 0x3C, d);
        let mut ok = clear(m, h, w, d);
        if !ok {
            let r = h.rand(m) as u8;
            let d = ((r & 1) << 7).wrapping_add(m.r8(w + 0x3D)).wrapping_sub(0x40);
            m.w8(w + 0x3C, d);
            ok = clear(m, h, w, d);
            if !ok {
                let d = m.r8(w + 0x3C).wrapping_add(0x80);
                m.w8(w + 0x3C, d);
                ok = clear(m, h, w, d);
            }
            if !ok {
                if m.r32(w + 0x48) != u32::MAX {
                    let clock = m.r32(CLOCK);
                    m.w32(w + 0x48, clock);
                }
                let d = m.r8(w + 0x3D).wrapping_add(0x80);
                m.w8(w + 0x3C, d);
                ok = clear(m, h, w, d);
            }
            if !ok {
                let h14 = m.r8(w + 0x14);
                m.w8(w + 0x3C, h14);
                m.w32(w + 0x48, u32::MAX);
                return vcall(m, h, w, 0x60, &[1]) as u8;
            }
        }
        m.w16(w + 0x3A, 0);
        m.w16(w + 0x38, 0);
    }
    let (mut c, mut r) = (col, row);
    let d = m.r8(w + 0x3C);
    h.step(m, &mut c, &mut r, d);
    begin_move(m, h, w, c, r, set);
    m.w16(w + 0x6C, c as u16);
    m.w16(w + 0x6E, r as u16);
    anim_set_default(m, anim, 2);
    1
}

/// Paso en curso del paseo (`0x0055F387`): avance, corte por otra figura en la casilla siguiente y repintado.
fn stroll_step(m: &mut dyn Mem, h: &mut dyn FigureHost, w: u32, set: u32, stepn: u32, frame: u8) -> u8 {
    let st = m.r8(w + 0x34);
    let mut x = m.r32(w + 0xC);
    let mut y = m.r32(w + 0x10);
    let mut n = if st == 6 || st == 0xB { stepn.wrapping_mul(2) } else { stepn };
    if matches!(st, 2 | 6 | 0xB) {
        let step = m.r16(w + 0x38);
        let left = u32::from(m.r16(w + 0x3A)).wrapping_sub(u32::from(step));
        if n > left {
            n = left;
        }
        m.w16(w + 0x38, step.wrapping_add(n as u16));
        x = x.wrapping_add(m.r32(w + 0x40).wrapping_mul(n));
        y = y.wrapping_add(m.r32(w + 0x44).wrapping_mul(n));
    }
    let mut redraw = false;
    if (x >> 16) as i32 != s16(m, w + 0xE) || (y >> 16) as i32 != s16(m, w + 0x12) {
        let (c, r) = h.pixel_to_cell(m, (x >> 16) as i32, (y >> 16) as i32);
        redraw = true;
        let (step, steps) = (i32::from(m.r16(w + 0x38)), i32::from(m.r16(w + 0x3A)));
        if step * 3 > steps * 2 && m.r8(w + 0x54) == 0 {
            let flag = m.r8(set + 8);
            let link = m.r16(w + 0x16);
            if !cell_walkable(m, h, c, r, flag, link) {
                begin_move(m, h, w, c, r, set);
                if m.r8(w + 0x54) != 0 {
                    return 1;
                }
            }
        }
    }
    if frame != m.r8(w + 0x15) || redraw {
        let rr = walker_remove(m, h, w, set, true);
        m.w32(w + 0xC, x);
        m.w32(w + 0x10, y);
        m.w8(w + 0x15, frame);
        walker_insert(m, h, w, set, rr);
        return 1;
    }
    m.w32(w + 0x10, y);
    m.w32(w + 0xC, x);
    1
}
