//! Efectos de la vista (clase `0x12`: humo, polvo, fuego, marcadores de los puntos de entrada): conjunto propio
//! (`+0x2C` en la lista `0x006E2E6C`, tipo en la tabla `0x006E2E74`), desfase del fotograma (`0x0057AF60`), fundido
//! (`0x0057AF40`) y paso por fotograma (`0x0057A8C0`, método `+0x14`).
//!
//! El paso es el de `p3_render::effect_step` (contrastado allí con un anfitrión abstracto) sobre la memoria de la
//! vista: avance de animación `0x00557340`, borrado/alta `0x005595F0`/`0x00559790`, medida `0x00559370` y repintado.
use super::sprite::{anim_advance, clip, dir_index, erase, frame_union, place, set_by, union_rect};
use super::{common, view, FigureHost, CLOCK, SIEGES};
use crate::flatmem::Mem;

/// Lista de conjuntos de los efectos (`{cuántos, conjuntos}`) y tabla de tipos por conjunto.
pub const SETS: u32 = 0x006E_2E6C;
pub const KINDS: u32 = 0x006E_2E74;

/// Conjunto del efecto (`0x0057B250`, `+0x4C`).
pub fn effect_set(m: &mut dyn Mem, e: u32) -> u32 {
    set_by(m, e + 0x2C, SETS)
}

/// Tipo del conjunto del efecto (`[0x006E2E74][+0x2C]`).
pub fn effect_kind(m: &mut dyn Mem, e: u32) -> u8 {
    let t = m.r32(KINDS);
    let i = u32::from(m.r8(e + 0x2C));
    m.r8(t.wrapping_add(i))
}

/// `0x0057AF40(ecx = efecto, espera, paso)`: fundido de la opacidad `+0x30` (paso `+0x34`, espera `+0x3C`).
pub fn set_fade(m: &mut dyn Mem, e: u32, wait: u32, step: u32) {
    m.w32(e + 0x34, step);
    m.w32(e + 0x3C, wait);
    if (step as i32) > 0 {
        m.w8(e + 0x30, 1);
    }
}

/// `0x0057AF60(ecx = efecto, conjunto, sentido)`: desfase temporal del fotograma actual en la tabla del conjunto
/// (altura `+0x2E` estirada y desplazamientos fijos de algunos tipos); `on` lo aplica y `!on` lo deshace.
pub fn stretch(m: &mut dyn Mem, e: u32, set: u32, on: bool) {
    let sgn: i32 = if on { 1 } else { -1 };
    let hd = m.r8(e + 0x14);
    let d = dir_index(m, set, hd);
    let kind = effect_kind(m, e);
    let row = |m: &mut dyn Mem| {
        let t = m.r32(set + 0x18);
        m.r32(t.wrapping_add(d * 4)).wrapping_add(u32::from(m.r8(e + 0x15)) * 12)
    };
    let add = |m: &mut dyn Mem, a: u32, v: i32| {
        let w = m.r16(a);
        m.w16(a, w.wrapping_add(v as u16));
    };
    let h = m.r16(e + 0x2E);
    if h != 0 {
        if m.r32(set + 0x1C) != 0 || matches!(kind, 0x20 | 0x21 | 0x22 | 0x24 | 0x25) {
            let r = row(m);
            add(m, r + 6, i32::from(h).wrapping_mul(sgn));
            let r = row(m);
            add(m, r + 4, (i32::from(h as i16) * sgn) / 5);
        }
        let r = row(m);
        add(m, r + 0xA, -(i32::from(h as i16).wrapping_mul(sgn)));
    }
    match kind {
        0x24 => {
            let r = row(m);
            add(m, r + 6, 48 * sgn);
            let f = i32::from(m.r8(e + 0x15));
            let r = row(m);
            add(m, r + 4, (f * sgn * 2) / 5);
            let r = row(m);
            add(m, r + 0xA, -48 * sgn);
        }
        0x30 | 0x11 | 0xB0 => {
            let r = row(m);
            add(m, r + 0xA, -48 * sgn);
        }
        0x81 => {
            let r = row(m);
            add(m, r + 0xA, -11 * sgn);
        }
        _ => {}
    }
}

/// `0x00613070(ecx = 0x006E55D0, &a, &b)`: segundo byte del parámetro del jugador actual (`+0x3FC`, `+0x671`); sin
/// jugador (`0xFF`) no escribe y el original deja la variable sin iniciar (aquí 0).
fn player_param(m: &mut dyn Mem) -> u8 {
    let o = 0x006E_55D0;
    let i = m.r8(o + 0x3FC);
    if i == 0xFF {
        return 0;
    }
    let p = m.r32(o + 4 * u32::from(i));
    m.r8(p + 0x671)
}

/// `0x0057A8C0` (método `+0x14` de los efectos): fundido, animación, desplazamiento `+0x40/+0x44` por el tiempo del
/// fotograma y redibujado. Devuelve 0 cuando el efecto termina.
pub fn effect_update(m: &mut dyn Mem, h: &mut dyn FigureHost, e: u32) -> u8 {
    let set = effect_set(m, e);
    let kind = effect_kind(m, e);
    let v = view(m);
    let dt = m.r32(v + 0x414);
    if dt > 2000 && kind < 0x80 {
        return 0;
    }
    if m.r32(e + 0x34) != 0 {
        let wait = m.r32(e + 0x3C);
        if wait > dt {
            m.w32(e + 0x3C, wait - dt);
        } else {
            m.w32(e + 0x3C, 0);
            let x = (dt.wrapping_sub(wait) as i32).wrapping_mul(m.r32(e + 0x34) as i32).wrapping_add(i32::from(m.r16(e + 0x38) as i16));
            let q = x / 16000;
            let inc = q.wrapping_mul(16);
            m.w16(e + 0x38, x.wrapping_sub(q.wrapping_mul(16000)) as u16);
            if inc != 0 {
                let s = i32::from(m.r8(e + 0x30)).wrapping_add(inc);
                if s < 1 {
                    return 0;
                }
                if s < 0xFF {
                    let o = m.r8(e + 0x30);
                    m.w8(e + 0x30, o.wrapping_add(inc as u8));
                } else {
                    m.w8(e + 0x30, 0xFF);
                    m.w32(e + 0x34, 0);
                }
            }
        }
    } else if kind == 0xC7 {
        let city = m.r32(v + 0xC324);
        let town = m.r32(0x006D_E508).wrapping_add(city.wrapping_mul(0x9F8));
        if m.r32(town + 0x2C8) & 0x0400_0000 == 0 {
            m.w32(e + 0x3C, 0);
            m.w32(e + 0x34, 0xFFFF_FF80);
        }
    }
    let frame = anim_advance(m, h, e + 0x20, set, e + 8, 0);
    if kind == 0x50 && frame != m.r8(e + 0x15) {
        let left = m.r32(e + 0x24).wrapping_sub(m.r32(CLOCK));
        let b = u32::from(player_param(m));
        let t = left.wrapping_mul(9) / (b + 8);
        let now = m.r32(CLOCK);
        m.w32(e + 0x24, t.wrapping_add(now));
    }
    let dt = { let v = view(m); m.r32(v + 0x414) };
    let x = m.r32(e + 0x40).wrapping_mul(dt).wrapping_add(m.r32(e + 0xC));
    let y = m.r32(e + 0x44).wrapping_mul(dt).wrapping_add(m.r32(e + 0x10));
    let old = m.r8(e + 0x15);
    if frame == old && m.r8(e + 0x30) == m.r8(e + 0x31) && (x >> 16) as u16 == m.r16(e + 0xE) && (y >> 16) as u16 == m.r16(e + 0x12) {
        let a = m.r8(e + 0x30);
        m.w32(e + 0x10, y);
        m.w32(e + 0xC, x);
        m.w8(e + 0x31, a);
        return 1;
    }
    if frame < old && (kind as i8) >= 0 && m.r8(e + 0x28) == 2 {
        return 0;
    }
    if frame == 0x14 && old != 0x14 {
        let city = { let v = view(m); m.r8(v + 0xC324) };
        if let Some(f) = linked_fighter(m, e, kind, city) {
            common::mark_target_engaged(m, f);
        }
    }
    if { let v = view(m); m.r8(v + 0xC594) } == 0 && (0x40..=0x43).contains(&kind) {
        m.w8(e + 0x15, frame);
        m.w32(e + 0xC, x);
        m.w32(e + 0x10, y);
        return 1;
    }
    stretch(m, e, set, true);
    let r1 = if kind & 0x40 == 0 { erase(m, h, true, set, e + 8).unwrap_or_default() } else { frame_union(m, set, e + 8) };
    stretch(m, e, set, false);
    m.w32(e + 0xC, x);
    m.w8(e + 0x15, frame);
    m.w32(e + 0x10, y);
    stretch(m, e, set, true);
    if kind & 0x40 == 0 {
        place(m, h, set, e + 8, Some(r1));
    } else {
        let r2 = frame_union(m, set, e + 8);
        let u = union_rect(&r1, &r2);
        clip(m, h, &u);
    }
    stretch(m, e, set, false);
    let a = m.r8(e + 0x30);
    m.w8(e + 0x31, a);
    1
}

/// `0x0057A4A0(ecx = efecto, x, y, rumbo)` (método `+0x48`): lleva el efecto al píxel `(x, y)` con el rumbo dado,
/// con borrado y alta en la vista; algunos tipos bajan 48 u 11 píxeles.
pub fn effect_move(m: &mut dyn Mem, h: &mut dyn FigureHost, e: u32, x: u32, y: u32, heading: u8) {
    if s16_eq(m, e + 0xE, x) && s16_eq(m, e + 0x12, y) && heading == m.r8(e + 0x14) {
        return;
    }
    let kind = effect_kind(m, e);
    if { let v = view(m); m.r8(v + 0xC594) } == 0 && (0x40..=0x43).contains(&kind) {
        m.w32(e + 0xC, x << 16);
        m.w32(e + 0x10, y << 16);
        m.w8(e + 0x14, heading);
        return;
    }
    let set = effect_set(m, e);
    super::sprite::invalidate(m, h, None, set, e + 8);
    stretch(m, e, set, true);
    let flat = kind & 0x40 != 0;
    if !flat {
        super::sprite::unregister_cells(m, h, set, e + 8);
    } else {
        let r = frame_union(m, set, e + 8);
        clip(m, h, &r);
    }
    stretch(m, e, set, false);
    m.w32(e + 0xC, x << 16);
    let mut yy = y << 16;
    m.w32(e + 0x10, yy);
    if matches!(kind, 0x24 | 0x30 | 0x11 | 0xB0) {
        yy = yy.wrapping_add(0x30_0000);
        m.w32(e + 0x10, yy);
    } else if kind == 0x81 {
        yy = yy.wrapping_add(0xB_0000);
        m.w32(e + 0x10, yy);
    }
    m.w8(e + 0x14, heading);
    stretch(m, e, set, true);
    if !flat {
        super::sprite::register_cells(m, h, set, e + 8);
    } else {
        let r = frame_union(m, set, e + 8);
        clip(m, h, &r);
    }
    stretch(m, e, set, false);
    super::sprite::invalidate(m, h, None, set, e + 8);
}

fn s16_eq(m: &mut dyn Mem, a: u32, v: u32) -> bool {
    i32::from(m.r16(a) as i16) == v as i32
}

/// Lista de efectos planos de la vista (`+0xC634` vector, `+0xC638` cuántos, `+0xC63A` capacidad): alta `0x0059A260`.
pub fn flat_list_add(m: &mut dyn Mem, h: &mut dyn FigureHost, e: u32) {
    let v = view(m);
    let n = m.r16(v + 0xC638);
    if n == m.r16(v + 0xC63A) {
        let cap = n.wrapping_add(0x40);
        m.w16(v + 0xC63A, cap);
        let old = m.r32(v + 0xC634);
        let p = h.realloc(m, old, u32::from(cap) * 4);
        m.w32(v + 0xC634, p);
    }
    let arr = m.r32(v + 0xC634);
    let n = m.r16(v + 0xC638);
    m.w32(arr.wrapping_add(u32::from(n) * 4), e);
    m.w16(v + 0xC638, n.wrapping_add(1));
}

/// Baja de la lista de efectos planos (`0x0059A2C0`): el último ocupa su hueco.
pub fn flat_list_remove(m: &mut dyn Mem, e: u32) {
    let v = view(m);
    let n = m.r16(v + 0xC638);
    let arr = m.r32(v + 0xC634);
    let mut i = 0u16;
    while i < n {
        if m.r32(arr.wrapping_add(u32::from(i) * 4)) == e {
            let last = n - 1;
            m.w16(v + 0xC638, last);
            let w = m.r32(arr.wrapping_add(u32::from(last) * 4));
            m.w32(arr.wrapping_add(u32::from(i) * 4), w);
            return;
        }
        i += 1;
    }
}

/// Figura de la lista `kind - 0x28` del registro de asedio de la ciudad de la vista cuyo efecto (`+0x64`) es `e`.
fn linked_fighter(m: &mut dyn Mem, e: u32, kind: u8, city: u8) -> Option<u32> {
    if m.r32(SIEGES) == 0 || u32::from(city) >= m.r32(0x006D_E4B0) || !(0x28..=0x2A).contains(&kind) {
        return None;
    }
    let rec = m.r32(SIEGES).wrapping_add(u32::from(city) * 0x1024);
    let (list, count) = match kind {
        0x28 => (0x410, 0x1006),
        0x29 => (0x80C, 0x1008),
        _ => (0xC08, 0x100A),
    };
    let n = m.r16(rec + count);
    let mut i = 0u8;
    while u16::from(i) < n {
        let f = m.r32(rec + list + 4 * u32::from(i));
        if m.r32(f + 0x64) == e {
            return Some(f);
        }
        i = i.wrapping_add(1);
    }
    None
}

/// `0x0057A650` (método `+0x44`): retira el efecto de la vista (repinta, suelta la figura que lo llevaba con
/// `0x00561F50` y sale de las casillas o de la lista de efectos planos).
pub fn effect_remove(m: &mut dyn Mem, h: &mut dyn FigureHost, e: u32) {
    let kind = effect_kind(m, e);
    let v = view(m);
    if m.r8(v + 0xC594) == 0 && (0x40..=0x43).contains(&kind) {
        return;
    }
    let set = effect_set(m, e);
    let city = m.r8(v + 0xC324);
    stretch(m, e, set, true);
    let (a, mask) = super::sprite::frame_rects(m, set, e + 8);
    clip(m, h, &a);
    if let Some(b) = mask {
        clip(m, h, &b);
    }
    if let Some(f) = linked_fighter(m, e, kind, city) {
        m.w32(f + 0x64, 0);
    }
    if kind & 0x40 == 0 {
        super::sprite::unregister_cells(m, h, set, e + 8);
    } else {
        flat_list_remove(m, e);
    }
    stretch(m, e, set, false);
}

/// `0x0057A150(ecx = efecto, x, y, rumbo, id, variante)` (método `+0x40`): elige el conjunto del tipo `variante &
/// 0xFF` (con subtipo `variante >> 8` en los tipos 4..7, 0x40..0x43, 0x50, 0x80, 0xB0 y 0xC7; al azar entre los que
/// valen, el primero si la vista no tiene `+0xC594`), arranca la animación 2 y da de alta el efecto en la vista. Los
/// puntos del minimapa (`0x005986A0`, tipos 0 y 2) no se portan: solo pintan la capa del minimapa.
pub fn effect_init(m: &mut dyn Mem, h: &mut dyn FigureHost, e: u32, a: [u32; 5]) {
    let [x, y, heading, id, variant] = a;
    let want = variant as u8;
    let subtype = if matches!(want, 4..=7 | 0x40..=0x43 | 0x50 | 0x80 | 0xB0 | 0xC7) { (variant >> 8) as u8 } else { 0 };
    m.w8(e + 0x2C, 0);
    let mut matches = 0u8;
    let mut i = 0u8;
    while u32::from(i) < m.r32(SETS) {
        let t = m.r32(KINDS);
        if m.r8(t.wrapping_add(u32::from(i))) == want {
            let ok = subtype == 0 || {
                let s = if u32::from(i) < m.r32(SETS) {
                    let p = m.r32(SETS + 4);
                    m.r32(p.wrapping_add(4 * u32::from(i)))
                } else {
                    0
                };
                u32::from(subtype) == m.r32(s.wrapping_add(4))
            };
            if ok {
                matches = matches.wrapping_add(1);
                let r = h.rand(m) as i32;
                if r % i32::from(matches) == 0 {
                    m.w8(e + 0x2C, i);
                    if { let v = view(m); m.r8(v + 0xC594) } == 0 {
                        i = (m.r32(SETS) as u8).wrapping_sub(1);
                    }
                }
            }
        }
        i = i.wrapping_add(1);
    }
    let set = effect_set(m, e);
    let f = super::sprite::anim_select(m, e + 0x20, set, 2);
    m.w8(e + 0x15, f);
    let t = m.r32(set + 0x10).wrapping_add(m.r32(CLOCK));
    m.w32(e + 0x24, t);
    super::spawn::links_alloc(m, h, e, set);
    m.w16(e + 0x16, id as u16);
    m.w32(e + 0xC, x);
    m.w32(e + 0x10, y);
    m.w8(e + 0x14, heading as u8);
    if matches!(want, 0x24 | 0x30 | 0x11 | 0xB0) {
        m.w32(e + 0x10, y.wrapping_add(0x30_0000));
    } else if want == 0x81 {
        m.w32(e + 0x10, y.wrapping_add(0xB_0000));
    }
    let k = effect_kind(m, e);
    m.w16(e + 0x2E, if k & 0x20 != 0 { (variant >> 16) as u16 } else { 0 });
    if { let v = view(m); m.r8(v + 0xC594) } == 0 && (0x40..=0x43).contains(&want) {
        return;
    }
    stretch(m, e, set, true);
    if want & 0x40 == 0 {
        place(m, h, set, e + 8, None);
    } else {
        let r = frame_union(m, set, e + 8);
        clip(m, h, &r);
        flat_list_add(m, h, e);
    }
    stretch(m, e, set, false);
}
