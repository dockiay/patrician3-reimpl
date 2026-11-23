//! Huellas del caminante en la nieve (`town_mover_emit_footsteps`, `0x0055D0B0`): con la estación de la vista
//! `+0x504` entre 2 y 5, la nieve `+0xC594` y la figura cerca de la ventana, deja efectos de pisada
//! (`0x0055B030`, por [`super::dispatch`]) en ciertos fotogramas de la animación.
//!
//! * Clase de conjunto `0x23` (cuadrúpedos): pares `0xC3/0xC4` a ambos lados del rumbo, desplazados por el seno
//!   del rumbo (`11 * sen` en x, `-7 * cos` en y, tabla `0x00674C98`), en los fotogramas 0, 0x0A, 0x16, 0x1A,
//!   0x1B, 0x21 y 0x35.
//! * Resto: una huella (`0xC3`, `0xC5` en la clase 3, `0xA0` con `bandera`) cuatro fotogramas después del inicio
//!   de los estados de andar (2, 6, 0xB, 0x13) o la segunda (`0xC4`, `0xC6`, `0xA1`) a los nueve; en los de
//!   lucha y caída las dos en el primer fotograma.
use super::sprite::anim_find;
use super::{dispatch, s16, view, FigureHost};
use crate::flatmem::Mem;

/// `0x00674C98`: cuarto de onda del seno en 16.16 (`round(sen(i·π/128)·65536)`, `i = 0..=0x40`).
pub const SIN: [i32; 65] = [
    0, 1608, 3216, 4821, 6424, 8022, 9616, 11204, 12785, 14359, 15924, 17479, 19024, 20557, 22078, 23586, 25080, 26558, 28020, 29466, 30893, 32303, 33692,
    35062, 36410, 37736, 39040, 40320, 41576, 42806, 44011, 45190, 46341, 47464, 48559, 49624, 50660, 51665, 52639, 53581, 54491, 55368, 56212, 57022, 57798,
    58538, 59244, 59914, 60547, 61145, 61705, 62228, 62714, 63162, 63572, 63944, 64277, 64571, 64827, 65043, 65220, 65358, 65457, 65516, 65536,
];

/// Seno 16.16 del rumbo (256 pasos por vuelta) con la tabla de cuarto de onda, como el original.
pub fn sin16(a: u8) -> i32 {
    let a = u32::from(a);
    if a < 0x80 {
        if a <= 0x40 {
            SIN[a as usize]
        } else {
            SIN[(0x80 - a) as usize]
        }
    } else {
        let j = a - 0x80;
        let v = if j <= 0x40 { SIN[j as usize] } else { SIN[(0x80 - j) as usize] };
        v.wrapping_neg()
    }
}

/// `0x0055B030(ecx = reglas del caminante, clase, x, y, rumbo, extra)`: efecto de pisada.
fn effect(m: &mut dyn Mem, h: &mut dyn FigureHost, w: u32, kind: u32, x: i32, y: i32, heading: u8, extra: u32) {
    dispatch(m, h, 0x0055_B030, w + 0x20, &[kind, x as u32, y as u32, u32::from(heading), extra]);
}

/// `0x0055D0B0(ecx = caminante, conjunto, bandera, dx, dy)`.
pub fn footsteps(m: &mut dyn Mem, h: &mut dyn FigureHost, w: u32, set: u32, flag: u32, dx: i32, dy: i32) {
    let v = view(m);
    let season = m.r32(v + 0x504);
    if season <= 1 || season >= 6 || m.r8(v + 0xC594) == 0 {
        return;
    }
    let px = s16(m, w + 0xE);
    let py = s16(m, w + 0x12);
    let (sx, sy) = (m.r32(v + 0xC650), m.r32(v + 0xC654));
    if (px.wrapping_add(0xC8) as u32) < sx || (py.wrapping_add(0xC8) as u32) < sy {
        return;
    }
    if px as u32 > m.r32(v + 0x2C).wrapping_add(sx).wrapping_add(0xC8) || py as u32 > m.r32(v + 0x30).wrapping_add(sy).wrapping_add(0xC8) {
        return;
    }
    let class = m.r32(set + 8);
    let heading = m.r8(w + 0x14);
    if class == 0x23 {
        let ox = sin16(heading).wrapping_mul(11) >> 16;
        let oy = sin16(heading.wrapping_add(0x40)).wrapping_mul(7).wrapping_neg() >> 16;
        let a = (px.wrapping_add(ox).wrapping_add(dx), py.wrapping_add(oy).wrapping_add(dy));
        let b = (px.wrapping_sub(ox).wrapping_add(dx), py.wrapping_sub(oy).wrapping_add(dy));
        let steps: &[(u32, (i32, i32))] = match m.r8(w + 0x15) {
            0x00 => &[(0xC3, a), (0xC4, b)],
            0x0A => &[(0xC4, a), (0xC3, b)],
            0x16 => &[(0xC3, b)],
            0x1A => &[(0xC3, a)],
            0x1B => &[(0xC4, b)],
            0x21 => &[(0xC4, a)],
            0x35 => &[(0xC3, a), (0xC4, a), (0xC3, b), (0xC4, b)],
            _ => &[],
        };
        for &(k, (x, y)) in steps {
            effect(m, h, w, k, x, y, heading, 0);
        }
        return;
    }
    let (k1, k2, dx, dy) = if flag as u8 != 0 {
        (0xA0, 0xA1, 0, 0)
    } else if class == 3 {
        (0xC5, 0xC6, dx, dy)
    } else {
        (0xC3, 0xC4, dx, dy)
    };
    let state = m.r8(w + 0x34);
    let walking = matches!(state, 2 | 6 | 0xB | 0x13);
    let fighting = matches!(state, 5 | 0xD | 0xF | 0x14 | 0x83 | 0x84 | 0x8C | 0x92 | 0xEC | 0xEE);
    if !walking && !fighting {
        return;
    }
    let i = anim_find(m, set, u32::from(state & 0x1F));
    let states = m.r32(set + 0x2C);
    let first = m.r32(states.wrapping_add(u32::from(i) * 12));
    let d = u32::from(m.r8(w + 0x15)).wrapping_sub(first);
    let (x, y) = (px.wrapping_add(dx), py.wrapping_add(dy));
    if walking {
        match d {
            4 => effect(m, h, w, k1, x, y, heading, flag),
            9 => effect(m, h, w, k2, x, y, heading, flag),
            _ => {}
        }
    } else if d == 0 {
        effect(m, h, w, k1, x, y, heading, flag);
        effect(m, h, w, k2, x, y, heading, flag);
    }
}
