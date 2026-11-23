//! Bucles por fotograma (`+0x14`), fotograma de disparo (`+0x64`) y avance hacia el punto de reunión o el objetivo
//! (`+0x58`) de las clases de figuras de la batalla en la ciudad, salvo la del luchador de la cola (clase 0,
//! `0x00577950`, en [`super::fighter`]).
//!
//! Todas llaman al bucle común (`0x0055E8C0`, [`super::update::walker_update`]) con su conjunto de sprites y un
//! umbral de la distancia del campo que depende del tramo del asalto: hacia la casilla de reunión (`+0x10 == +0x14`),
//! en la salida (`+0x36`), en la segunda salida (`+0x3E`) o en la marcha (resto), proporcional a lo que queda del
//! avance de la escena (`+0x42`, `+0x44`, `+0x66`, `+0x3C`, `+0x68`, puertas `+0x254`).
//!
//! | Clase | `+0x14` | `+0x64` | `+0x58` | Conjunto |
//! |---|---|---|---|---|
//! | 1 (arqueros) | `0x005674F0` | `0x00567730` | común | `+0x88` en `0x006E174C` |
//! | 2 (ballesteros) | `0x005624C0` | `0x005626E0` | común | `+0x88` en `0x006E16E0` |
//! | 3 (artilleros) | `0x00572BA0` | `0x00572DC0` | común | `+0x88` en `0x006E2DD0` |
//! | 4 (lanceros) | `0x005710A0` | por defecto | `0x005719A0` | `+0x91` en `0x006E2DAC` |
//! | 7 (infantería) | `0x0056A7F0` | por defecto | `0x0056AA20` (nada) | `+0x88` en `0x006E1854` |
//! | 7 (caballería, `0x23`) | `0x0056F1A0` | por defecto | `0x0056F410` | `+0x88` en `0x006E2D8C` |
use super::footsteps::sin16;
use super::update::walker_update;
use super::walker::{begin_step, clear_mark, mark_at_least, set_mark};
use super::{common, dispatch, s16, siege_active, siege_of, FigureHost, DIRS};
use crate::economy::siege_battle::flow_distance;
use crate::economy::siege_view::{reset_flow_field_to_cell, update_entry_point_markers};
use crate::flatmem::Mem;

/// Conjunto de sprites del índice en el byte `at` de la tabla `(recuento, tabla)` o 0 fuera de ella.
pub fn set_of(m: &mut dyn Mem, at: u32, count: u32, table: u32) -> u32 {
    let index = m.r8(at);
    if u32::from(index) < m.r32(count) {
        let t = m.r32(table);
        m.r32(t.wrapping_add(u32::from(index) * 4))
    } else {
        0
    }
}

/// `byte [[tabla] + i]`.
fn byte_of(m: &mut dyn Mem, table: u32, i: u32) -> u8 {
    let t = m.r32(table);
    m.r8(t.wrapping_add(i))
}

fn scene(m: &mut dyn Mem, w: u32) -> u32 {
    let city = m.r8(w + 0x6A);
    let rec = siege_of(m, city);
    m.r32(rec + 8)
}

fn r16(m: &mut dyn Mem, a: u32) -> u32 {
    u32::from(m.r16(a))
}

fn flow(m: &mut dyn Mem, h: &mut dyn FigureHost, s: u32, x: u32, y: u32) -> u32 {
    u32::from(flow_distance(m, h, s, x, y))
}

/// Cociente sin signo del original (la división entre cero no se da con escenas válidas: aquí da 0).
fn udiv(a: u32, b: u32) -> u32 {
    a.checked_div(b).unwrap_or(0)
}

/// Distancia de la salida `+0x36/+0x38` por el campo.
fn exit_flow(m: &mut dyn Mem, h: &mut dyn FigureHost, s: u32) -> u32 {
    let (x, y) = (r16(m, s + 0x36), r16(m, s + 0x38));
    flow(m, h, s, x, y)
}

/// Distancia de la segunda salida: la casilla de reunión `+8/+0xA` o, sin ella (`0xFFFF`), la salida.
fn second_exit_flow(m: &mut dyn Mem, h: &mut dyn FigureHost, s: u32) -> u32 {
    if m.r16(s + 8) == 0xFFFF {
        exit_flow(m, h, s)
    } else {
        let (x, y) = (r16(m, s + 8), r16(m, s + 0xA));
        flow(m, h, s, x, y)
    }
}

/// Umbral hacia la reunión: `(+0x42 - +0x44) * d / (+0x66 - +0x44)`.
fn toward(m: &mut dyn Mem, s: u32, d: u32) -> u32 {
    let a = r16(m, s + 0x44);
    let t = r16(m, s + 0x42).wrapping_sub(a).wrapping_mul(d);
    udiv(t, r16(m, s + 0x66).wrapping_sub(a))
}

/// Umbral de la segunda salida de los tiradores: `(+0x3C - +0x68/2 - 0x100) * d / (+0x68/2)`.
fn second_exit_shooters(m: &mut dyn Mem, s: u32, d: u32) -> u32 {
    let half = r16(m, s + 0x68) >> 1;
    let t = r16(m, s + 0x3C).wrapping_sub(half).wrapping_sub(0x100).wrapping_mul(d);
    udiv(t, half)
}

/// Umbral de la segunda salida de la tropa: `(+0x3C - +0x68/2 + 0x100) * d / ((+0x68 + 0x200) / 2)`.
fn second_exit_troops(m: &mut dyn Mem, s: u32, d: u32) -> u32 {
    let c = r16(m, s + 0x68);
    let t = r16(m, s + 0x3C).wrapping_sub(c >> 1).wrapping_add(0x100).wrapping_mul(d);
    udiv(t, (c + 0x200) >> 1)
}

/// Umbral de la marcha con la distancia `e`: proporcional a `+0x42/+0x66` o, con puerta asaltada (`+0x6F`), al
/// tramo que queda tras la puerta (`+0x254`) con medio margen hasta `+0x44` (máximo `0x100`); `None` si ya pasó.
fn march(m: &mut dyn Mem, s: u32, e: u32) -> Option<u32> {
    let g = m.r8(s + 0x6F);
    if g == 0xFF {
        return Some(udiv(r16(m, s + 0x42).wrapping_mul(e), r16(m, s + 0x66)));
    }
    let gd = r16(m, s + u32::from(g) * 8 + 0x254);
    let mut c = (r16(m, s + 0x44).wrapping_sub(gd) as i32 >> 1) as u32;
    if c > 0x100 {
        c = 0x100;
    }
    let a = r16(m, s + 0x42);
    if a <= gd.wrapping_add(c) {
        return None;
    }
    let t = a.wrapping_sub(gd).wrapping_sub(c).wrapping_mul(e);
    Some(udiv(t, r16(m, s + 0x66).wrapping_sub(gd).wrapping_sub(c)))
}

fn at(m: &mut dyn Mem, s: u32, x: u32, y: u32) -> bool {
    m.r16(s + 0x10) == m.r16(s + x) && m.r16(s + 0x12) == m.r16(s + y)
}

/// Bucle de los tiradores (clases 1, 2 y 3): `guarded` (clase 1) solo avanza en la segunda salida si el avance
/// `+0x3C` supera `+0x68/2 + 0x100`.
fn shooter_update(m: &mut dyn Mem, h: &mut dyn FigureHost, w: u32, set: u32, guarded: bool) -> u8 {
    let city = m.r8(w + 0x6A);
    let rec = siege_of(m, city);
    if !siege_active(m, rec) {
        return walker_update(m, h, w, set, 1, 0, 1, 0, 0);
    }
    let s = m.r32(rec + 8);
    if at(m, s, 0x14, 0x16) {
        let d = exit_flow(m, h, s);
        let t = toward(m, s, d);
        return walker_update(m, h, w, set, 1, t, 1, 0, 0);
    }
    if at(m, s, 0x36, 0x38) {
        return walker_update(m, h, w, set, 1, 0, 1, 0, 0);
    }
    if at(m, s, 0x3E, 0x40) {
        if guarded && r16(m, s + 0x3C) as i32 <= ((r16(m, s + 0x68) >> 1) + 0x100) as i32 {
            return walker_update(m, h, w, set, 1, 0, 1, 0, 0);
        }
        let d = second_exit_flow(m, h, s);
        let t = second_exit_shooters(m, s, d);
        return walker_update(m, h, w, set, 1, t, 1, 0, 0);
    }
    let d = exit_flow(m, h, s);
    let e = d - d.min(8);
    let t = march(m, s, e).unwrap_or(0);
    walker_update(m, h, w, set, 1, t.wrapping_add(8), 8, 0, 0)
}

/// `0x005674F0` (clase 1, `+0x14`).
pub fn archer_update(m: &mut dyn Mem, h: &mut dyn FigureHost, w: u32) -> u8 {
    let set = set_of(m, w + 0x88, 0x006E_174C, 0x006E_1750);
    shooter_update(m, h, w, set, true)
}

/// `0x005624C0` (clase 2, `+0x14`).
pub fn crossbow_update(m: &mut dyn Mem, h: &mut dyn FigureHost, w: u32) -> u8 {
    let set = set_of(m, w + 0x88, 0x006E_16E0, 0x006E_16E4);
    shooter_update(m, h, w, set, false)
}

/// `0x00572BA0` (clase 3, `+0x14`).
pub fn gunner_update(m: &mut dyn Mem, h: &mut dyn FigureHost, w: u32) -> u8 {
    let set = set_of(m, w + 0x88, 0x006E_2DD0, 0x006E_2DD4);
    shooter_update(m, h, w, set, false)
}

/// Fotogramas de disparo de un tirador: tablas de bytes por conjunto (primer y segundo fotograma, cuenta atrás).
struct Volley {
    first: u32,
    second: u32,
    reload: u32,
}

/// Cuenta atrás del disparo en los dos fotogramas: `Some(fotograma)` si aún no dispara (devuelve el fotograma, o el
/// anterior para repetirlo mientras cuenta), `None` si toca disparar.
fn volley_wait(m: &mut dyn Mem, w: u32, f: u8, v: &Volley) -> Option<u8> {
    let i = u32::from(m.r8(w + 0x88));
    let a = u32::from(byte_of(m, v.first, i)) + 1;
    let b = u32::from(byte_of(m, v.second, i)) + 1;
    if u32::from(f) != a && u32::from(f) != b {
        return Some(f);
    }
    let c = m.r8(w + 0x89);
    if c == 0 {
        let n = byte_of(m, v.reload, i);
        m.w8(w + 0x89, n);
        return Some(f.wrapping_sub(1));
    }
    m.w8(w + 0x89, c - 1);
    if c - 1 != 0 {
        return Some(f.wrapping_sub(1));
    }
    None
}

/// Código del proyectil: casilla del objetivo `+0x7C/+0x7E` en los 16 bits altos y la clase de impacto en los bajos
/// (`0x808` contra figuras, `0x83E` contra edificios y la reunión, `0x3E08` de los atacantes).
fn volley_code(m: &mut dyn Mem, h: &mut dyn FigureHost, w: u32) -> u32 {
    let (x, y) = (i32::from(m.r16(w + 0x7C)), i32::from(m.r16(w + 0x7E)));
    let (col, row) = h.pixel_to_cell(m, x, y);
    let code = (col << 8).wrapping_add(row) << 16;
    if m.r8(w + 0x7B) != 0 {
        return code.wrapping_add(0x3E08);
    }
    let s = scene(m, w);
    if m.r8(s + 0x6F) != 0xFF {
        return code.wrapping_add(0x808);
    }
    match u32::from(m.r8(w + 0x78)).wrapping_sub(0x80) {
        0 => code.wrapping_add(0x808),
        1..=3 => code.wrapping_add(0x83E),
        4 if m.r16(s + 0x242) != 0 => code.wrapping_add(0x83E),
        4 => code.wrapping_add(0x808),
        _ => code,
    }
}

fn any_nonzero(m: &mut dyn Mem, s: u32, at: &[u32]) -> bool {
    at.iter().any(|&a| m.r16(s + a) != 0)
}

fn volley_sound(m: &mut dyn Mem, h: &mut dyn FigureHost, w: u32, list: u32, offset: i32) {
    if h.sound_list_size(m, list) == 0 {
        return;
    }
    let n = h.sound_list_size(m, list) as i32;
    let r = (h.rand(m) as i32).wrapping_rem(n);
    let (x, y) = (s16(m, w + 0xE), s16(m, w + 0x12));
    h.play_sound(m, x, y, r.wrapping_sub(offset) as u32);
}

fn launch(m: &mut dyn Mem, h: &mut dyn FigureHost, w: u32, code: u32, heading: u32) -> u32 {
    let (x, y) = (m.r32(w + 0xC), m.r32(w + 0x10));
    let e = h.spawn_entity(m, [x, y, 0x17, code, heading, 0, 0]);
    let city = m.r8(w + 0x6A);
    m.w8(e + 0x2C, city);
    e
}

/// `0x00567730` (clase 1, `+0x64`): flecha (rumbo 4 hacia la puerta o sin defensas en la escena, si no 0).
pub fn archer_frame(m: &mut dyn Mem, h: &mut dyn FigureHost, w: u32, frame: u32) -> u8 {
    let f = frame as u8;
    if let Some(r) = volley_wait(m, w, f, &Volley { first: 0x006E_1758, second: 0x006E_175C, reload: 0x006E_1760 }) {
        return r;
    }
    let code = volley_code(m, h, w);
    let b = m.r8(w + 0x7B);
    let s = scene(m, w);
    let high = if b == 0 {
        if m.r8(s + 0x6F) != 0xFF {
            true
        } else {
            let walls = (m.r16(s + 0x232) != 0 && m.r8(s + 0x78) != 0) || any_nonzero(m, s, &[0x236, 0x23A, 0x23E]);
            !walls
        }
    } else {
        b == 1 && !any_nonzero(m, s, &[0x21E, 0x222, 0x226, 0x22A])
    };
    launch(m, h, w, code, if high { 4 } else { 0 });
    volley_sound(m, h, w, 0xEE70, 0x1190);
    f
}

/// `0x005626E0` (clase 2, `+0x64`): virote (rumbo 5 o 1).
pub fn crossbow_frame(m: &mut dyn Mem, h: &mut dyn FigureHost, w: u32, frame: u32) -> u8 {
    let f = frame as u8;
    if let Some(r) = volley_wait(m, w, f, &Volley { first: 0x006E_16EC, second: 0x006E_16F0, reload: 0x006E_16F4 }) {
        return r;
    }
    let code = volley_code(m, h, w);
    let b = m.r8(w + 0x7B);
    let s = scene(m, w);
    let high = if b == 1 && (m.r8(s + 0x6F) != 0xFF || !any_nonzero(m, s, &[0x232, 0x236, 0x23A, 0x23E])) {
        true
    } else {
        b == 0 && !any_nonzero(m, s, &[0x21E, 0x222, 0x226, 0x22A])
    };
    launch(m, h, w, code, if high { 5 } else { 1 });
    volley_sound(m, h, w, 0xEE52, 0x11AE);
    f
}

/// `0x00572DC0` (clase 3, `+0x64`): bala (rumbo 2) y fogonazo (clase `0x12`) en la boca del arma, 34 px por el seno
/// del rumbo en x y 22 px por el coseno en y.
pub fn gunner_frame(m: &mut dyn Mem, h: &mut dyn FigureHost, w: u32, frame: u32) -> u8 {
    let f = frame as u8;
    if let Some(r) = volley_wait(m, w, f, &Volley { first: 0x006E_2DDC, second: 0x006E_2DE0, reload: 0x006E_2DE4 }) {
        return r;
    }
    let code = volley_code(m, h, w);
    launch(m, h, w, code, 2);
    let hd = m.r8(w + 0x14);
    let c = sin16(hd.wrapping_add(0x40));
    let sn = sin16(hd);
    let x = m.r32(w + 0xC).wrapping_add(sn.wrapping_mul(34) as u32);
    let y = m.r32(w + 0x10).wrapping_sub(c.wrapping_mul(22) as u32);
    let id = ((code & 0xFF00) << 8).wrapping_add(0x8_0022);
    h.spawn_entity(m, [x, y, 0x12, id, u32::from(hd), 0, 0]);
    volley_sound(m, h, w, 0xEE84, 0x117C);
    f
}

/// `0x00571DA0(ecx = lancero)`: se empareja con el atacante de la cola `+0x7C` más cercano (`dx² + (dy·17/11)²`)
/// libre (`+0x58 == 0`, sin pareja `+0x8C`), si la cabeza de su puerta (`+0xC`/`+0x10` del contenedor según el
/// bando `+0x90`) está libre y no hay salida (`+0x78`).
pub fn pikeman_acquire(m: &mut dyn Mem, w: u32) {
    let city = m.r8(w + 0x6A);
    let rec = siege_of(m, city);
    let s = m.r32(rec + 8);
    if s == 0 || m.r8(s + 0x78) != 0 {
        return;
    }
    let side = u32::from(m.r8(w + 0x90));
    let head = m.r32(rec.wrapping_add(0x10).wrapping_sub(4 * side));
    if head == 0 || m.r8(head + 0x58) != 0 || m.r8(head + 0x5A) != 0 {
        return;
    }
    let count = m.r16(s + 0x18);
    if count == 0 {
        return;
    }
    let mut best = i32::MAX;
    let mut pick = 0xFFu8;
    let mut i = 0u8;
    loop {
        let e = m.r32(s + 0x7C + 4 * u32::from(i));
        if e != 0 && m.r8(e + 0x58) == 0 && m.r32(e + 0x8C) == 0 {
            let dx = s16(m, w + 0xE).wrapping_sub(s16(m, e + 0xE));
            let dy = s16(m, w + 0x12).wrapping_sub(s16(m, e + 0x12));
            let q = dy.wrapping_mul(17) / 11;
            let d = q.wrapping_mul(q).wrapping_add(dx.wrapping_mul(dx));
            if d < best {
                best = d;
                pick = i;
            }
        }
        i = i.wrapping_add(1);
        if u16::from(i) >= count {
            break;
        }
    }
    if pick != 0xFF {
        let e = m.r32(s + 0x7C + 4 * u32::from(pick));
        m.w32(w + 0x88, e);
        m.w32(e + 0x8C, w);
    }
}

/// `0x005710A0` (clase 4, `+0x14`).
pub fn pikeman_update(m: &mut dyn Mem, h: &mut dyn FigureHost, w: u32) -> u8 {
    let set = set_of(m, w + 0x91, 0x006E_2DAC, 0x006E_2DB0);
    let city = m.r8(w + 0x6A);
    let rec = siege_of(m, city);
    if !siege_active(m, rec) {
        if m.r32(rec + 8) == 0 {
            return 0;
        }
        return walker_update(m, h, w, set, 4, 0, 1, 0, 0);
    }
    let s = m.r32(rec + 8);
    let p = m.r32(w + 0x88);
    if p != 0 && m.r8(p + 0x58) != 0 {
        m.w32(w + 0x88, 0);
        pikeman_acquire(m, w);
    }
    if at(m, s, 0x14, 0x16) {
        let d = exit_flow(m, h, s);
        let t = toward(m, s, d);
        return walker_update(m, h, w, set, 4, t, 1, 0, 0);
    }
    if at(m, s, 0x36, 0x38) {
        return if m.r8(s + 0x79) != 0 { walker_update(m, h, w, set, 4, 0, 1, 0, 0) } else { walker_update(m, h, w, set, 4, 0, 1, 1, 1) };
    }
    if at(m, s, 0x3E, 0x40) {
        let d = second_exit_flow(m, h, s);
        let t = second_exit_troops(m, s, d);
        return walker_update(m, h, w, set, 4, t, 1, 0, 0);
    }
    let d = exit_flow(m, h, s);
    if m.r8(s + 0x6F) == 0xFF {
        let t = march(m, s, d).unwrap_or(0);
        return if t > 1 { walker_update(m, h, w, set, 4, t, 1, 0, 0) } else { walker_update(m, h, w, set, 4, t, 1, 0, 4) };
    }
    let t = march(m, s, d).map_or(10, |t| t.max(10));
    walker_update(m, h, w, set, 4, t, 1, 0, 0)
}

/// Rumbo de ocho direcciones hacia `(dx, dy)` sin escalar: domina x si `268·|dx| > 1000·|dy|`, y si
/// `640·|dy| > 1000·|dx|`.
pub(crate) fn heading_to(dx: i32, dy: i32) -> u8 {
    let (ax, ay) = (dx.wrapping_abs(), dy.wrapping_abs());
    if ax.wrapping_mul(268) > ay.wrapping_mul(1000) {
        return if dx > 0 { DIRS[2] } else { DIRS[6] };
    }
    if ay.wrapping_mul(640) > ax.wrapping_mul(1000) {
        return if dy > 0 { DIRS[4] } else { DIRS[0] };
    }
    match (dy > 0, dx > 0) {
        (true, true) => DIRS[3],
        (true, false) => DIRS[5],
        (false, true) => DIRS[1],
        (false, false) => DIRS[7],
    }
}

/// Paso hacia `(dx, dy)` (cuadrado de la distancia `d2`) por el campo de la escena: prueba el rumbo directo y, a
/// más de 34 px, los dos vecinos (`+0x20`, `-0x20`); la casilla no debe estar marcada, el paso debe ser posible
/// (`0x00561330`) y no alejarse por el campo. Marca la casilla nueva, arranca el paso (`0x00560750`) y desmarca la
/// de la posición. Devuelve 2 si se mueve y 0 si no.
fn step_toward(m: &mut dyn Mem, h: &mut dyn FigureHost, w: u32, s: u32, dx: i32, dy: i32, d2: i32, set: impl Fn(&mut dyn Mem) -> u32) -> u8 {
    let mut dir = heading_to(dx, dy);
    let (px, py) = (s16(m, w + 0xE), s16(m, w + 0x12));
    let (c, r) = h.pixel_to_cell(m, px, py);
    let here = flow(m, h, s, c, r) as u16;
    let mut tries = 0;
    let target = loop {
        let (mut c2, mut r2) = (c, r);
        h.step(m, &mut c2, &mut r2, dir);
        if !mark_at_least(m, s, c2, r2, 1) && common::can_step_to_tile(m, h, w, c, r, dir) && flow(m, h, s, c2, r2) as u16 <= here {
            break (c2, r2);
        }
        tries += 1;
        match tries {
            1 if d2 <= 0x484 => return 0,
            1 => dir = dir.wrapping_add(0x20),
            2 => dir = dir.wrapping_add(0xC0),
            _ => return 0,
        }
    };
    m.w8(w + 0x3C, dir);
    set_mark(m, s, target.0, target.1, 1);
    let set = set(m);
    begin_step(m, h, w, target.0, target.1, set);
    let (px, py) = (s16(m, w + 0xE), s16(m, w + 0x12));
    let (c3, r3) = h.pixel_to_cell(m, px, py);
    clear_mark(m, s, c3, r3, 1);
    2
}

/// `0x005719A0(ecx = lancero, cualquiera)` (clase 4, `+0x58`): sin salida ni puerta asaltada, camina hacia su punto
/// de reunión (`+0x5CA/+0x5CE` según el bando), reorientando el campo hacia él si la escena apuntaba a otro sitio.
pub fn pikeman_rally(m: &mut dyn Mem, h: &mut dyn FigureHost, w: u32, any: u8) -> u8 {
    let s = scene(m, w);
    if any != 0 || s == 0 || m.r8(s + 0x78) != 0 || m.r8(s + 0x6F) != 0xFF {
        return 0;
    }
    let side = u32::from(m.r8(w + 0x90));
    let (rx, ry) = (r16(m, s + 0x5CA + 2 * side), r16(m, s + 0x5CE + 2 * side));
    let (tx, ty) = h.cell_to_pixel(m, rx, ry);
    let dx = tx.wrapping_sub(s16(m, w + 0xE));
    let dy = ty.wrapping_sub(s16(m, w + 0x12));
    let d2 = dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx));
    if d2 <= 100 || d2 > 0x70E40 {
        return 0;
    }
    if !(r16(m, s + 0x10) == rx && r16(m, s + 0x12) == ry) {
        reset_flow_field_to_cell(m, h, s, rx, ry, 0);
        let (a, b) = (r16(m, s + 0xC), r16(m, s + 0xE));
        super::update::seed_cell(m, h, s, a, b, 0);
        update_entry_point_markers(m, h, s, true);
    }
    step_toward(m, h, w, s, dx, dy, d2, |m: &mut dyn Mem| set_of(m, w + 0x91, 0x006E_2DAC, 0x006E_2DB0))
}

/// Bucle de la tropa (clases 7): `riders` (`0x0056F1A0`) en la salida levanta polvo (clase `0x12`, variante 8, con
/// desvanecimiento `0x0057AF40`) al andar o luchar (estados 2 y 5) y pide el avance libre.
fn troop_update(m: &mut dyn Mem, h: &mut dyn FigureHost, w: u32, set: u32, riders: bool) -> u8 {
    let city = m.r8(w + 0x6A);
    let rec = siege_of(m, city);
    if !siege_active(m, rec) {
        return walker_update(m, h, w, set, 1, 0, 1, 0, 0);
    }
    let s = m.r32(rec + 8);
    if at(m, s, 0x14, 0x16) {
        let d = exit_flow(m, h, s);
        let t = toward(m, s, d);
        return walker_update(m, h, w, set, 1, t, 1, 0, 0);
    }
    if at(m, s, 0x36, 0x38) {
        if m.r8(s + 0x79) != 0 {
            return walker_update(m, h, w, set, 1, 0, 1, 0, 0);
        }
        if riders && matches!(m.r8(w + 0x34), 2 | 5) {
            let (x, y) = ((s16(m, w + 0xE) as u32) << 16, (s16(m, w + 0x12) as u32) << 16);
            let hd = u32::from(m.r8(w + 0x14));
            let e = h.spawn_entity(m, [x, y, 0x12, 8, hd, 0, 0]);
            dispatch(m, h, 0x0057_AF40, e, &[1000, 0xFFFF_FF80]);
        }
        return walker_update(m, h, w, set, 1, 0, 1, 1, 0);
    }
    if at(m, s, 0x3E, 0x40) {
        let d = second_exit_flow(m, h, s);
        let t = second_exit_troops(m, s, d);
        return walker_update(m, h, w, set, 1, t, 1, 0, 0);
    }
    let d = exit_flow(m, h, s);
    let e = d - d.min(16);
    let t = march(m, s, e).unwrap_or(0);
    walker_update(m, h, w, set, 1, t.wrapping_add(16), 16, 0, 0)
}

/// `0x0056A7F0` (clase 7, `+0x14`).
pub fn infantry_update(m: &mut dyn Mem, h: &mut dyn FigureHost, w: u32) -> u8 {
    let set = set_of(m, w + 0x88, 0x006E_1854, 0x006E_1858);
    troop_update(m, h, w, set, false)
}

/// `0x0056F1A0` (clase 7 de jinetes, `+0x14`).
pub fn rider_update(m: &mut dyn Mem, h: &mut dyn FigureHost, w: u32) -> u8 {
    let set = set_of(m, w + 0x88, 0x006E_2D8C, 0x006E_2D90);
    troop_update(m, h, w, set, true)
}

/// `0x0056F410(ecx = jinete, cualquiera)` (`+0x58`): sin asalto en marcha (`+0x3A == 0`), carga hacia el objetivo de
/// la escena `+0x218`.
pub fn rider_charge(m: &mut dyn Mem, h: &mut dyn FigureHost, w: u32, any: u8) -> u8 {
    let s = scene(m, w);
    if any != 0 || s == 0 || m.r16(s + 0x3A) != 0 {
        return 0;
    }
    let e = m.r32(s + 0x218);
    if e == 0 {
        return 0;
    }
    let dx = s16(m, e + 0xE).wrapping_sub(s16(m, w + 0xE));
    let dy = s16(m, e + 0x12).wrapping_sub(s16(m, w + 0x12));
    let d2 = dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx));
    if d2 <= 100 || d2 > 0x70E40 {
        return 0;
    }
    step_toward(m, h, w, s, dx, dy, d2, |m: &mut dyn Mem| set_of(m, w + 0x88, 0x006E_2D8C, 0x006E_2D90))
}
