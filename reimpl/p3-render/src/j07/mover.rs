//! Figuras que se mueven por la ciudad: reglas de movimiento, `TownMover`, `TownPathMover` y `TownWalker` (`0x559EB0`–`0x55FFC0`).
//!
//! `TownMover` (vtable `0x674E00`) es un `SpriteEntity` (`0x557A00`) con reglas de movimiento en `+0x20` (`MoveRules`, vtable
//! `0x674BC4` sobre la base `0x674B74`) y el estado del paso: `+0x38`/`+0x3A` desplazamiento, `+0x3C`/`+0x3D` dirección pedida y
//! actual, `+0x40`..`+0x48` acumuladores y `+0x54`..`+0x57` banderas. `TownWalker` (vtable `0x674E90`) añade `+0x58` estado,
//! `+0x59` figura subida a un carro, `+0x64` oculta, `+0x68` opacidad de aparición y `+0x82`/`+0x84` desplazamiento del dibujo;
//! `TownPathMover` (vtable `0x674C38`) añade `+0x6A` bandera de camino terminado.

use super::entity::{dir_index, sx16, PIXEL_TO_CELL};
use super::sheet::{SGL_DRAW_BITMAP_RECT, SGL_SET_RENDER_SOURCE};
use super::sprite::{deleting_dtor, UI};
use super::{at32, cdecl, free, vcall, Host, Mem, Ret};

pub const SPRITE_ENTITY_CTOR: u32 = 0x0055_7A00;
pub const SPRITE_ENTITY_DTOR: u32 = 0x0055_7A70;
pub const SET_DRAW_MASK_FRAME: u32 = 0x0055_9520;
pub const SET_ERASE: u32 = 0x0055_95F0;
pub const LINKS_DRAW_NEXT: u32 = 0x0055_99D0;
pub const LINKS_DRAW_NEXT_MASK: u32 = 0x0055_9A10;
pub const MOVE_RULES_BASE_CTOR: u32 = 0x0055_A110;
pub const MOVE_RULES_CTOR: u32 = 0x0055_A260;
pub const MOVE_RULES_DTOR: u32 = 0x0055_A280;
pub const PATH_MOVER_UPDATE: u32 = 0x0055_B6A0;
pub const PATH_MOVER_DTOR: u32 = 0x0055_B1A0;
pub const MOVER_CTOR: u32 = 0x0055_C630;
pub const MOVER_DTOR: u32 = 0x0055_C6D0;
pub const SPAWN_TOWN_ENTITY: u32 = 0x0059_0610;
pub const EFFECT_SET_FADE: u32 = 0x0057_AF40;
pub const SGL_SET_CONSTANT_COLOR: u32 = 0x004B_B870;

fn add16(m: &mut dyn Mem, a: u32, d: u16) {
    let v = m.r16(a).wrapping_add(d);
    m.w16(a, v);
}

fn sub16(m: &mut dyn Mem, a: u32, d: u16) {
    let v = m.r16(a).wrapping_sub(d);
    m.w16(a, v);
}

/// `0x55C770`: vuelve a la dirección actual y borra el paso en curso.
fn mover_reset_motion(m: &mut dyn Mem, this: u32) {
    let d = m.r8(this + 0x14);
    m.w8(this + 0x3C, d);
    m.w8(this + 0x3D, d);
    m.w16(this + 0x38, 0);
    m.w16(this + 0x3A, 0);
    for k in [0x40, 0x44, 0x48] {
        m.w32(this + k, 0);
    }
    for k in 0x54..0x58 {
        m.w8(this + k, 0);
    }
}

/// `0x55C6D0`: destructor de `TownMover`.
fn mover_dtor(m: &mut dyn Mem, h: &mut dyn Host, this: u32) -> u32 {
    m.w32(this, 0x0067_4E00);
    m.w32(this + 8, 0x0067_4DFC);
    m.w32(this + 0x1C, 0x0067_4DF0);
    m.w32(this + 0x20, 0x0067_4E50);
    mover_reset_motion(m, this);
    m.w32(this + 0x2C, 0x0067_4E54);
    h.call(m, MOVE_RULES_DTOR, this + 0x20, &[]);
    h.call(m, SPRITE_ENTITY_DTOR, this, &[])
}

/// `0x55E050`: borra el estado de paso propio de `TownWalker`.
fn walker_reset(m: &mut dyn Mem, this: u32) {
    m.w16(this + 0x38, 0);
    m.w16(this + 0x3A, 0);
    m.w8(this + 0x58, 0);
    m.w8(this + 0x70, 0);
    m.w8(this + 0x7A, 0);
    m.w16(this + 0x68, 0);
}

/// `0x559EB0` (this, plano de movimiento): avanza el camino y, si acaba de terminar, saca la figura de la parte activa de su lista.
///
/// Las listas por categoría están en `UI+0xC5A0` (punteros), `UI+0xC5B8` (inicio de la parte activa) y `UI+0xC5C0` (número activo).
fn path_mover_update_and_repartition(m: &mut dyn Mem, h: &mut dyn Host, this: u32, plan: u32) -> u8 {
    let was = m.r8(this + 0x6A);
    let r = h.call(m, PATH_MOVER_UPDATE, this, &[plan]) as u8;
    let cat = match m.r32(plan + 8) {
        0 => 1u32,
        1 => 2,
        2 => 3,
        4 => 0,
        _ => return r,
    };
    let ui = m.r32(UI);
    let first = u32::from(m.r16(ui + 0xC5B8 + cat * 2));
    let end = first + u32::from(m.r16(ui + 0xC5C0 + cat * 2));
    if was != 1 || m.r8(this + 0x6A) != 0 || first >= end {
        return r;
    }
    let list = m.r32(ui + 0xC5A0 + cat * 4);
    let Some(k) = (first..end).find(|&k| m.r32(list.wrapping_add(k * 4)) == this) else { return r };
    if k != first {
        let v = m.r32(list.wrapping_add(first * 4));
        m.w32(list.wrapping_add(k * 4), v);
        let ui = m.r32(UI);
        let f = u32::from(m.r16(ui + 0xC5B8 + cat * 2));
        let l = m.r32(ui + 0xC5A0 + cat * 4);
        m.w32(l.wrapping_add(f * 4), this);
    }
    let ui = m.r32(UI);
    sub16(m, ui + 0xC5C0 + cat * 2, 1);
    let ui = m.r32(UI);
    add16(m, ui + 0xC5B8 + cat * 2, 1);
    r
}

/// `0x55B030` (stdcall: tipo, x, y, parámetro, en coordenadas de casilla): huella o polvo bajo una figura, salvo en agua y muros.
fn spawn_footstep_effect(m: &mut dyn Mem, h: &mut dyn Host, a: &[u32], esp: u32) {
    let ui = m.r32(UI);
    let mode = m.r32(ui + 0x504);
    if mode <= 1 || mode >= 6 || m.r8(ui + 0xC594) == 0 {
        return;
    }
    let kind = a[0] & 0xFF;
    let e = if a[4] as u8 != 0 {
        h.call(m, SPAWN_TOWN_ENTITY, ui, &[a[1] << 16, a[2].wrapping_add(0xD) << 16, 0x12, kind + 0x46_0000, a[3], 0, 0])
    } else {
        let cell = esp - 8;
        h.call(m, PIXEL_TO_CELL, ui, &[a[1], a[2], cell]);
        let ui = m.r32(UI);
        let idx = m.r32(ui + 0xC314).wrapping_mul(m.r32(cell + 4)).wrapping_add(m.r32(cell));
        let grid = m.r32(ui + 0xC308);
        let tile = u32::from(m.r16(grid.wrapping_add(idx.wrapping_mul(4))));
        let objs = m.r32(ui + 0x98);
        let obj = m.r32(objs.wrapping_add(tile * 4));
        let t = vcall(m, h, obj, 0x50, &[]);
        if t == 0x10 || (0x17..=0x18).contains(&t) {
            return;
        }
        let ui = m.r32(UI);
        h.call(m, SPAWN_TOWN_ENTITY, ui, &[a[1] << 16, a[2] << 16, 0x12, kind, a[3], 0, 0])
    };
    h.call(m, EFFECT_SET_FADE, e, &[0x3E8, (-0x12i32) as u32]);
    m.w8(e + 0x30, 0xB4);
}

/// `0x55E070` (this, índice, conjunto): máscara del caminante (con el desplazamiento del dibujo si va a pie) y siguiente enlace.
fn walker_draw_shadow(m: &mut dyn Mem, h: &mut dyn Host, this: u32, a: &[u32]) -> u32 {
    if vcall(m, h, this, 0x60, &[0]) as u8 != 0 && m.r32(this + 0x64) == 0 {
        let foot = m.r8(this + 0x59) == 0;
        let (dx, dy) = (m.r16(this + 0x82), m.r16(this + 0x84));
        if foot {
            add16(m, this + 0xE, dx);
            add16(m, this + 0x12, dy);
        }
        cdecl(m, h, SET_DRAW_MASK_FRAME, &[a[1], this + 8]);
        if m.r8(this + 0x59) == 0 {
            let (dx, dy) = (m.r16(this + 0x82), m.r16(this + 0x84));
            sub16(m, this + 0xE, dx);
            sub16(m, this + 0x12, dy);
        }
    }
    h.call(m, LINKS_DRAW_NEXT_MASK, this, &[a[0]])
}

/// `0x55E0E0` (this, parte, conjunto): dibuja la parte `parte` de la animación actual con la opacidad de aparición.
fn walker_draw_sprite(m: &mut dyn Mem, h: &mut dyn Host, this: u32, a: &[u32]) -> u32 {
    if vcall(m, h, this, 0x60, &[0]) as u8 != 0 && m.r32(this + 0x64) == 0 {
        let fade = m.r16(this + 0x68);
        if fade != 0 && fade < 0x8000 {
            cdecl(m, h, SGL_SET_CONSTANT_COLOR, &[(u32::from(fade >> 7) & 0xFF) << 24 | 0xFF_FFFF]);
        }
        let set = a[1];
        let n = m.r32(set + 0x14);
        let i = dir_index(m.r8(this + 0x14), n);
        let anims = m.r32(set + 0x24);
        let an = m.r32(anims.wrapping_add(i * 4));
        let k = u32::from(m.r8(this + 0x15));
        let parts = m.r32(an.wrapping_add(k * 12 + 8));
        let p = parts.wrapping_add((a[0] & 0xFF) * 12);
        let ui = m.r32(UI);
        let (ox, oy) = (m.r32(ui + 0xC650), m.r32(ui + 0xC654));
        let tex = m.r32(set + 0x34);
        cdecl(m, h, SGL_SET_RENDER_SOURCE, &[tex]);
        let cart = m.r8(this + 0x59) != 0;
        if cart {
            add16(m, p + 0xA, (-0x39i16) as u16);
        } else {
            let (dx, dy) = (m.r16(this + 0x82), m.r16(this + 0x84));
            add16(m, p + 8, dx);
            add16(m, p + 0xA, dy);
        }
        let (hh, w) = (u32::from(m.r16(p + 6)), u32::from(m.r16(p + 4)));
        let ui = m.r32(UI);
        let r = vcall(m, h, ui, 0x70, &[]);
        let y = r.wrapping_add(sx16(m, this + 0x12).wrapping_add(sx16(m, p + 0xA)).wrapping_sub(oy));
        let ui = m.r32(UI);
        let r = vcall(m, h, ui, 0x6C, &[]);
        let x = r.wrapping_add(sx16(m, this + 0xE).wrapping_add(sx16(m, p + 8)).wrapping_sub(ox));
        let (rx, ry) = (u32::from(m.r16(p)), u32::from(m.r16(p + 2)));
        cdecl(m, h, SGL_DRAW_BITMAP_RECT, &[rx, ry, x, y, w, hh]);
        if m.r8(this + 0x59) != 0 {
            add16(m, p + 0xA, 0x39);
        } else {
            let (dx, dy) = (m.r16(this + 0x82), m.r16(this + 0x84));
            sub16(m, p + 8, dx);
            sub16(m, p + 0xA, dy);
        }
        if m.r16(this + 0x68) < 0x8000 {
            cdecl(m, h, SGL_SET_CONSTANT_COLOR, &[u32::MAX]);
        }
    }
    h.call(m, LINKS_DRAW_NEXT, this, &[a[0]])
}

/// Desplaza el fotograma actual (y su máscara) del caminante antes de borrarlo (`sign` 1) o lo restaura después (`sign` -1).
fn walker_shift_frame(m: &mut dyn Mem, h: &mut dyn Host, this: u32, set: u32, di: u32, sign: i16) {
    let s = |v: u16| (v as i16).wrapping_mul(sign) as u16;
    let part = u32::from(m.r8(this + 0x15)) * 12;
    if m.r8(this + 0x59) != 0 {
        let f = at32(m, set + 0x18, di);
        add16(m, f.wrapping_add(part + 0xA), s((-0x46i16) as u16));
        add16(m, this + 0x12, s(0xD));
        return;
    }
    if vcall(m, h, this, 0x68, &[]) as u8 == 4 {
        let f = at32(m, set + 0x18, di);
        add16(m, f.wrapping_add(part + 0xA), s((-0xBi16) as u16));
        add16(m, this + 0x12, s(0xB));
        return;
    }
    for tab in [0x18, 0x1C] {
        let t = m.r32(set + tab);
        if tab == 0x1C && t == 0 {
            break;
        }
        let dx = m.r16(this + 0x82);
        let f = m.r32(t.wrapping_add(di));
        let part = u32::from(m.r8(this + 0x15)) * 12;
        add16(m, f.wrapping_add(part + 8), s(dx));
        let dy = m.r16(this + 0x84);
        let f = at32(m, set + tab, di);
        let part = u32::from(m.r8(this + 0x15)) * 12;
        add16(m, f.wrapping_add(part + 0xA), s(dy));
    }
}

/// `0x55E250` (this, conjunto, extra): borra el caminante de las casillas que ocupa su fotograma desplazado.
fn walker_remove_from_tile_lists(m: &mut dyn Mem, h: &mut dyn Host, this: u32, a: &[u32]) {
    let set = a[0];
    let n = m.r32(set + 0x14);
    let di = dir_index(m.r8(this + 0x14), n) * 4;
    walker_shift_frame(m, h, this, set, di, 1);
    cdecl(m, h, SET_ERASE, &[set, this + 8, a[1]]);
    walker_shift_frame(m, h, this, set, di, -1);
}

pub const SPAWN_FOOTSTEP: u32 = 0x0055_B030;
pub const FIND_ANIMATION_INDEX: u32 = 0x0055_76C0;

/// Seno en 16.16 de un ángulo de 256 pasos con la tabla de cuarto de vuelta `0x674C98` (65 valores).
fn sin_t(m: &mut dyn Mem, c: u32) -> u32 {
    const T: u32 = 0x0067_4C98;
    if c < 0x80 {
        if c <= 0x40 { m.r32(T + c * 4) } else { m.r32(T + (0x80 - c) * 4) }
    } else if c - 0x80 <= 0x40 {
        m.r32(T + (c - 0x80) * 4).wrapping_neg()
    } else {
        m.r32(T + (0x100 - c) * 4).wrapping_neg()
    }
}

/// `0x55D0B0` (this, conjunto, en carro, dx, dy): huellas de la figura según la parte de su animación, si está cerca de la vista.
///
/// Las llamadas a `0x55B030` llevan en los 24 bits altos del parámetro de dirección lo que quedaba en el registro (el original
/// solo carga el byte bajo); el porte reproduce esos bits tal como quedan con las llamadas interceptadas, que no tocan ECX.
fn emit_footsteps(m: &mut dyn Mem, h: &mut dyn Host, this: u32, a: &[u32]) {
    let ui = m.r32(UI);
    let mode = m.r32(ui + 0x504);
    if mode <= 1 || mode >= 6 || m.r8(ui + 0xC594) == 0 {
        return;
    }
    let x = sx16(m, this + 0xE);
    let (ox, oy) = (m.r32(ui + 0xC650), m.r32(ui + 0xC654));
    let y = sx16(m, this + 0x12);
    if x.wrapping_add(0xC8) < ox || y.wrapping_add(0xC8) < oy {
        return;
    }
    if x > m.r32(ui + 0x2C).wrapping_add(ox).wrapping_add(0xC8) || y > m.r32(ui + 0x30).wrapping_add(oy).wrapping_add(0xC8) {
        return;
    }
    let set = a[0];
    let kind = m.r32(set + 8);
    let rules = this + 0x20;
    let hi = |v: u32, d: u8| (v & 0xFFFF_FF00) | u32::from(d);
    if kind == 0x23 {
        let dir = u32::from(m.r8(this + 0x14));
        let s = ((sin_t(m, dir).wrapping_mul(11)) as i32 >> 16) as u32;
        let c = ((sin_t(m, (dir + 0x40) & 0xFF).wrapping_mul(7).wrapping_neg()) as i32 >> 16) as u32;
        let part = m.r8(this + 0x15);
        // (código, signo de x, signo de y, ¿ECX alto tras la llamada anterior?)
        let steps: &[(u32, bool, bool)] = match part {
            0 => &[(0xC3, true, true), (0xC4, false, false)],
            10 => &[(0xC4, true, true), (0xC3, false, false)],
            22 => &[(0xC3, false, false)],
            26 => &[(0xC3, true, true)],
            27 => &[(0xC4, false, false)],
            33 => &[(0xC4, true, true)],
            53 => &[(0xC3, true, true), (0xC4, true, true), (0xC3, false, false), (0xC4, false, false)],
            _ => return,
        };
        let mut prev: Option<u32> = None;
        for (i, &(code, px, py)) in steps.iter().enumerate() {
            let (x, y) = (sx16(m, this + 0xE), sx16(m, this + 0x12));
            let d = m.r8(this + 0x14);
            let xx = if px { x.wrapping_add(s) } else { x.wrapping_sub(s) }.wrapping_add(a[2]);
            let yy = if py { y.wrapping_add(c) } else { y.wrapping_sub(c) }.wrapping_add(a[3]);
            let p = match (i, prev) {
                (0, _) => u32::from(d),
                // Parte 10: el original empuja EAX (el retorno de la primera llamada) con la dirección en AL.
                (_, Some(r)) if part == 10 => hi(r, d),
                _ => hi(rules, d),
            };
            prev = Some(h.call(m, SPAWN_FOOTSTEP, rules, &[code, xx, yy, p, 0]));
        }
        return;
    }
    let cart = a[1] as u8 != 0;
    let (dx, dy) = if cart { (0, 0) } else { (a[2], a[3]) };
    let (k0, k1) = if cart { (0xA0, 0xA1) } else if kind == 3 { (0xC5, 0xC6) } else { (0xC3, 0xC4) };
    let (kind0, kind1) = (hi(set, k0), hi(this, k1));
    let state = m.r8(this + 0x34);
    let quiet = match state {
        2 | 6 | 11 | 19 => false,
        5 | 13 | 15 | 20 | 131 | 132 | 140 | 146 | 236 | 238 => true,
        _ => return,
    };
    let key = (y & 0xFFFF_FF00) | u32::from(state & 0x1F);
    let idx = h.call(m, FIND_ANIMATION_INDEX, this + 0x2C, &[set, key]) & 0xFF;
    let tab = m.r32(set + 0x2C);
    let base = m.r32(tab.wrapping_add(idx * 12));
    let diff = u32::from(m.r8(this + 0x15)).wrapping_sub(base);
    let pos = |m: &mut dyn Mem| (sx16(m, this + 0xE).wrapping_add(dx), sx16(m, this + 0x12).wrapping_add(dy));
    if !quiet {
        if diff == 9 {
            let (xx, yy) = pos(m);
            let d = m.r8(this + 0x14);
            h.call(m, SPAWN_FOOTSTEP, rules, &[kind1, xx, yy, hi(idx * 3, d), a[1]]);
        } else if diff == 4 {
            let (xx, yy) = pos(m);
            let d = m.r8(this + 0x14);
            h.call(m, SPAWN_FOOTSTEP, rules, &[kind0, xx, yy, u32::from(d), a[1]]);
        }
        return;
    }
    if diff != 0 {
        return;
    }
    let (xx, yy) = pos(m);
    let d = m.r8(this + 0x14);
    h.call(m, SPAWN_FOOTSTEP, rules, &[kind0, xx, yy, hi(tab, d), a[1]]);
    let (xx, yy) = pos(m);
    let d = m.r8(this + 0x14);
    h.call(m, SPAWN_FOOTSTEP, rules, &[kind1, xx, yy, hi(rules, d), a[1]]);
}

pub const SIEGE_TABLE: u32 = 0x006D_F270;
pub const SIEGE_GRIDS: u32 = 0x006D_E508;
pub const FLOW_DISTANCE: u32 = 0x0062_B9C0;
pub const TARGET_DISTANCE: u32 = 0x0063_0760;
pub const SET_CELL_MARK: u32 = 0x0063_0CC0;
pub const STEP_CELL: u32 = 0x0058_B740;
pub const BEGIN_STEP: u32 = 0x0056_0750;

/// Tipo de la casilla `(x, y)` del plano de la ciudad (vtable `+0x50` de su objeto), o `None` fuera del plano.
fn tile_kind(m: &mut dyn Mem, h: &mut dyn Host, x: u32, y: u32) -> Option<u32> {
    let ui = m.r32(UI);
    let w = m.r32(ui + 0xC314);
    if x >= w || y >= m.r32(ui + 0xC318) {
        return None;
    }
    let grid = m.r32(ui + 0xC308);
    let tile = u32::from(m.r16(grid.wrapping_add(w.wrapping_mul(y).wrapping_add(x).wrapping_mul(4))));
    let objs = m.r32(ui + 0x98);
    let obj = m.r32(objs.wrapping_add(tile * 4));
    Some(vcall(m, h, obj, 0x50, &[]))
}

/// Byte de la rejilla del asedio para la casilla `(x, y)` (el bit 7 marca casillas de la muralla).
fn siege_cell(m: &mut dyn Mem, side: u8, x: u32, y: u32) -> u8 {
    let g = m.r32(SIEGE_GRIDS).wrapping_add(u32::from(side) * 319 * 8 + 0x7A4);
    let row = m.r32(g + 8).wrapping_mul(x).wrapping_add(m.r32(g + 0x68));
    m.r8(row.wrapping_add(y))
}

/// `0x55FFF0` (this, x, y, distancia mínima, marca, parámetro, distancia preferida, no usado): elige la casilla vecina con
/// mejor distancia del campo de flujo del asedio en cinco abanicos de dirección a cada lado y empieza el paso hacia ella.
///
/// Las variables locales que el original no inicializa (mejor dirección y casilla) valen 0, como en la pila de la prueba.
fn walker_pick_next_tile(m: &mut dyn Mem, h: &mut dyn Host, this: u32, a: &[u32], esp: u32) -> u8 {
    let (x, y) = (a[1], a[2]);
    let side = m.r8(this + 0x6A);
    let sg = m.r32(SIEGE_TABLE).wrapping_add(u32::from(side) * 1033 * 4 + 8);
    let s = m.r32(sg);
    if s == 0 {
        return 0;
    }
    let mut flag = false;
    let mut d0 = h.call(m, FLOW_DISTANCE, s, &[x, y]);
    if d0 as u16 == 0xFFFF {
        return 0;
    }
    let w80 = |m: &mut dyn Mem| u32::from(m.r16(this + 0x80));
    let r = h.call(m, TARGET_DISTANCE, s, &[x, y]) & 0xFFFF;
    let near_wall = if r as i32 <= w80(m) as i32 + 1 { !matches!(tile_kind(m, h, x, y), Some(t) if t != 0x18) } else { false };
    let s78 = m.r8(s + 0x78) != 0;
    if near_wall && !s78 && m.r8(this + 0x7B) == 0 {
        d0 = d0.wrapping_add(2);
        flag = true;
    }
    // Rama `0x5600E4`/`0x5600EF`: con el asedio en marcha y la figura sin la bandera 1, se acerca al objetivo.
    if s78 && m.r8(this + 0x7B) & 1 == 0 {
        let r = h.call(m, TARGET_DISTANCE, s, &[x, y]) & 0xFFFF;
        if (r as i32) < w80(m) as i32 + 0xA {
            let r2 = h.call(m, TARGET_DISTANCE, s, &[x, y]) as u16;
            if u32::from(r2) > w80(m) {
                if !matches!(tile_kind(m, h, x, y), Some(t) if t != 0x18) {
                    d0 = d0.wrapping_add(2);
                }
                flag = true;
            }
        }
    }
    let marked = |m: &mut dyn Mem| m.r8(s + 0x79) != 0 && m.r8(this + 0x7B) & 1 != 0;
    let mut best: u32 = if marked(m) && (siege_cell(m, side, x, y) as i8) < 0 { 0 } else { 0xFFFF };
    let (pcx, pcy) = (esp - 0x1C, esp - 0x20);
    let (mut bx, mut by, mut bdir) = (0u32, 0u32, 0u8);
    let mut k: u8 = 0;
    loop {
        for right in [false, true] {
            let dir0 = m.r8(this + 0x14);
            let dir = if right { dir0.wrapping_add(k) } else { dir0.wrapping_sub(k) };
            m.w32(pcx, x);
            m.w32(pcy, y);
            cdecl(m, h, STEP_CELL, &[pcx, pcy, (x & 0xFFFF_FF00) | u32::from(dir)]);
            let (cx, cy) = (m.r32(pcx), m.r32(pcy));
            let mut cand = h.call(m, FLOW_DISTANCE, s, &[cx, cy]);
            if flag && !matches!(tile_kind(m, h, cx, cy), Some(t) if t != 0x18) {
                cand = cand.wrapping_add(1);
            }
            let c16 = cand as u16;
            if marked(m) && siege_cell(m, side, x, y) & 0x80 != 0 {
                if c16 <= best as u16 || (siege_cell(m, side, cx, cy) as i8) >= 0 {
                    continue;
                }
                if vcall(m, h, this, 0x5C, &[cx, cy, a[5]]) as u8 == 0 {
                    continue;
                }
                best = cand;
                bx = cx;
                by = cy;
                // El lado derecho también resta: así lo hace el original.
                bdir = m.r8(this + 0x14).wrapping_sub(k);
                continue;
            }
            if c16 >= best as u16 || c16 < a[3] as u16 {
                continue;
            }
            let r = h.call(m, TARGET_DISTANCE, s, &[cx, cy]) as u16;
            let w = w80(m) as u16;
            let close = if right { r < w } else { r <= w };
            if close && !matches!(tile_kind(m, h, cx, cy), Some(t) if t != 0x18) && m.r8(s + 0x78) == 0 && m.r8(this + 0x7B) != 1 {
                continue;
            }
            if vcall(m, h, this, 0x5C, &[cx, cy, a[5]]) as u8 == 0 {
                continue;
            }
            let pref = a[6] as u16;
            if c16 > pref || (c16 == pref && c16 < d0 as u16) {
                best = cand;
                bx = cx;
                by = cy;
                let d = m.r8(this + 0x14);
                bdir = if right { d.wrapping_add(k) } else { d.wrapping_sub(k) };
            }
        }
        k = k.wrapping_add(0x20);
        if k > 0x80 {
            break;
        }
    }
    let mut turn = bdir.wrapping_sub(m.r8(this + 0x14));
    if turn > 0x80 {
        turn = turn.wrapping_neg();
    }
    let ok = if marked(m) && siege_cell(m, side, x, y) & 0x80 != 0 {
        let b = best as u16;
        let dx = if b < 0x7FFF { b } else { 0 };
        let di = d0 as u16;
        (b < 0x7FFF && di < dx) || (di == dx && turn <= 0x40 && u32::from(dx) >= u32::from(a[3] as u16) + 2)
    } else {
        let (dx, di) = (d0 as u16, best as u16);
        di < dx || (di == dx && turn <= 0x40 && u32::from(dx) >= u32::from(a[3] as u16) + 2)
    };
    if !ok {
        return 0;
    }
    m.w8(this + 0x3C, bdir);
    h.call(m, SET_CELL_MARK, s, &[bx, by, a[4]]);
    m.w8(this + 0x70, 1);
    h.call(m, BEGIN_STEP, this, &[bx, by, a[0]]);
    1
}

pub fn run(f: u32, m: &mut dyn Mem, h: &mut dyn Host, ecx: u32, a: &[u32], esp: u32) -> Option<Ret> {
    let r = match f {
        0x559EB0 => Ret::U8(path_mover_update_and_repartition(m, h, ecx, a[0])),
        0x55A110 => {
            m.w32(ecx, 0x0067_4B74);
            Ret::U32(ecx)
        }
        0x55A120 => {
            m.w32(ecx, 0x0067_4B74);
            Ret::None
        }
        0x55A240 | 0x55DF40 => {
            m.w32(ecx, if f == 0x55A240 { 0x0067_4B74 } else { 0x0067_4E54 });
            if a[0] & 1 != 0 {
                free(m, h, ecx);
            }
            Ret::U32(ecx)
        }
        0x55A260 => {
            h.call(m, MOVE_RULES_BASE_CTOR, ecx, &[]);
            m.w32(ecx, 0x0067_4BC4);
            Ret::U32(ecx)
        }
        0x55A280 => {
            m.w32(ecx, 0x0067_4BC4);
            m.w32(ecx, 0x0067_4B74);
            Ret::None
        }
        0x55B030 => {
            spawn_footstep_effect(m, h, a, esp);
            Ret::None
        }
        0x55B140 => Ret::U32(deleting_dtor(m, h, ecx, MOVE_RULES_DTOR, a[0])),
        0x55B160 => {
            h.call(m, MOVER_CTOR, ecx, &[]);
            m.w32(ecx, 0x0067_4C38);
            m.w32(ecx + 8, 0x0067_4C30);
            m.w32(ecx + 0x1C, 0x0067_4C24);
            m.w32(ecx + 0x20, 0x0067_4C20);
            m.w32(ecx + 0x2C, 0x0067_4C14);
            m.w8(ecx + 0x6A, 0);
            Ret::U32(ecx)
        }
        0x55B1A0 => {
            m.w32(ecx, 0x0067_4C38);
            m.w32(ecx + 8, 0x0067_4C30);
            m.w32(ecx + 0x1C, 0x0067_4C24);
            m.w32(ecx + 0x20, 0x0067_4C20);
            m.w32(ecx + 0x2C, 0x0067_4C14);
            Ret::U32(mover_dtor(m, h, ecx))
        }
        0x55C5D0 => Ret::U32(deleting_dtor(m, h, ecx, PATH_MOVER_DTOR, a[0])),
        0x55C5F0 | 0x55C600 | 0x55C610 | 0x55C620 => {
            let d = [8, 0x1C, 0x20, 0x2C][((f - 0x55C5F0) / 0x10) as usize];
            Ret::U32(deleting_dtor(m, h, ecx.wrapping_sub(d), PATH_MOVER_DTOR, a[0]))
        }
        0x55C630 => {
            h.call(m, SPRITE_ENTITY_CTOR, ecx, &[]);
            h.call(m, MOVE_RULES_CTOR, ecx + 0x20, &[]);
            m.w8(ecx + 0x34, 0);
            m.w8(ecx + 0x35, 0);
            m.w32(ecx + 0x30, 0);
            m.w32(ecx + 0x20, 0x0067_4E50);
            m.w32(ecx, 0x0067_4E00);
            m.w32(ecx + 8, 0x0067_4DFC);
            m.w32(ecx + 0x1C, 0x0067_4DF0);
            m.w32(ecx + 0x2C, 0x0067_4DE4);
            mover_reset_motion(m, ecx);
            Ret::U32(ecx)
        }
        0x55C6D0 => Ret::U32(mover_dtor(m, h, ecx)),
        0x55C770 => {
            mover_reset_motion(m, ecx);
            Ret::None
        }
        0x55D080 => {
            let inst = if ecx == 0 { 0 } else { ecx + 8 };
            Ret::U32(cdecl(m, h, SET_ERASE, &[a[0], inst, 0]))
        }
        0x55D0B0 => {
            emit_footsteps(m, h, ecx, a);
            Ret::None
        }
        0x55FFF0 => Ret::U8(walker_pick_next_tile(m, h, ecx, a, esp)),
        0x55E8C0 => Ret::U8(super::walker::walker_update(m, h, ecx, esp)),
        0x55DF60 => Ret::U32(deleting_dtor(m, h, ecx, MOVER_DTOR, a[0])),
        0x55DF80 | 0x55DF90 | 0x55DFA0 | 0x55DFB0 => {
            let d = [8, 0x1C, 0x20, 0x2C][((f - 0x55DF80) / 0x10) as usize];
            Ret::U32(deleting_dtor(m, h, ecx.wrapping_sub(d), MOVER_DTOR, a[0]))
        }
        0x55DFC0 => {
            h.call(m, MOVER_CTOR, ecx, &[]);
            m.w32(ecx, 0x0067_4E90);
            walker_reset(m, ecx);
            m.w32(ecx + 8, 0x0067_4E88);
            m.w32(ecx + 0x1C, 0x0067_4E7C);
            m.w32(ecx + 0x20, 0x0067_4E78);
            m.w32(ecx + 0x2C, 0x0067_4E6C);
            Ret::U32(ecx)
        }
        0x55E010 => {
            m.w32(ecx, 0x0067_4E90);
            m.w32(ecx + 8, 0x0067_4E88);
            m.w32(ecx + 0x1C, 0x0067_4E7C);
            m.w32(ecx + 0x20, 0x0067_4E78);
            m.w32(ecx + 0x2C, 0x0067_4E6C);
            walker_reset(m, ecx);
            Ret::U32(mover_dtor(m, h, ecx))
        }
        0x55E050 => {
            walker_reset(m, ecx);
            Ret::None
        }
        0x55E070 => Ret::U32(walker_draw_shadow(m, h, ecx, a)),
        0x55E0E0 => Ret::U32(walker_draw_sprite(m, h, ecx, a)),
        0x55E250 => {
            walker_remove_from_tile_lists(m, h, ecx, a);
            Ret::None
        }
        0x55FFC0 => {
            let s = m.r8(ecx + 8);
            if s != 2 && s != 0x13 && s != 0xB {
                m.w16(ecx + 0xC, 0);
                m.w16(ecx + 0xE, 0);
            }
            if s == 0x8C {
                m.w16(ecx + 0xC, 0);
                m.w16(ecx + 0xE, 1);
            }
            Ret::U8(s)
        }
        _ => return None,
    };
    Some(r)
}
