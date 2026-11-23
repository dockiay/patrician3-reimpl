//! Decisiones del caminante común durante el asalto: casilla siguiente por el campo de distancias
//! (`0x0055FFF0`), cola de la puerta (`0x00560CB0`), arranque del paso hacia una casilla (`0x00560750`) y
//! animación del estado (`0x00560970`), con los ayudantes de la escena del asedio que usan: distancia al destino
//! (`0x00630760`), marcas por casilla (`0x00630C40`, `0x00630CC0`, `0x00630D40`) y piezas de puerta en un rumbo
//! (`0x00561080`).
//!
//! Campos del caminante que se usan aquí: `+0x14` rumbo dibujado, `+0x34` estado de animación, `+0x38/+0x3A` paso
//! y pasos, `+0x3C` rumbo de destino, `+0x40/+0x44` velocidad 16.16, `+0x59` en la cola de la puerta, `+0x5A`
//! posición en la cola, `+0x5C` siguiente de la cola, `+0x64` efecto, `+0x6A` ciudad, `+0x6C/+0x6E` casilla de
//! destino, `+0x70` en marcha, `+0x74` objetivo, `+0x79` rumbo de la puerta, `+0x7A` rango en la cola, `+0x7B`
//! bando y `+0x80` radio de llegada.
use super::step::ground_kind;
use super::{s16, scene_active, siege_active, siege_of, sprite, vcall, view, FigureHost, DIRS, TOWNS};
use crate::economy::siege_battle::{flow_distance, propagate_queue_rank};
use crate::flatmem::Mem;

/// `0x00630760(ecx = escena, columna, fila)`: distancia euclídea (columnas dobles) a la casilla de destino
/// `escena+8/+0xA`, truncada.
pub fn distance_to_target(m: &mut dyn Mem, scene: u32, col: u32, row: u32) -> u32 {
    let dx = s16(m, scene + 8).wrapping_sub(i32::from(col as u16 as i16)).wrapping_mul(2);
    let dy = s16(m, scene + 0xA).wrapping_sub(i32::from(row as u16 as i16));
    let v = dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx));
    if v < 0 {
        // `sqrt` de un negativo da NaN y `__ftol` lo convierte en el entero indefinido.
        return 0x8000_0000;
    }
    f64::from(v).sqrt() as i32 as u32
}

/// Índice de la casilla en la máscara de marcas de la escena, si el asalto sigue y está dentro.
fn mark_cell(m: &mut dyn Mem, scene: u32, col: u32, row: u32) -> Option<u32> {
    if !scene_active(m, scene) {
        return None;
    }
    let mask = m.r32(scene + 0x70);
    let (c, r) = (col as u16 as i16, row as u16 as i16);
    if mask == 0 || c < 0 || r < 0 {
        return None;
    }
    let v = view(m);
    let w = m.r32(v + 0xC314);
    let (c, r) = (i32::from(c) as u32, i32::from(r) as u32);
    if c >= w || r >= m.r32(v + 0xC318) {
        return None;
    }
    Some(mask.wrapping_add(r.wrapping_mul(w).wrapping_add(c)))
}

/// `0x00630C40(ecx = escena, columna, fila, valor)`: ¿la marca de la casilla llega a `bits`?
pub fn mark_at_least(m: &mut dyn Mem, scene: u32, col: u32, row: u32, bits: u8) -> bool {
    match mark_cell(m, scene, col, row) {
        Some(a) => m.r8(a) >= bits,
        None => false,
    }
}

/// `0x00630CC0(ecx = escena, columna, fila, bits)`: activa `bits` en la marca de la casilla.
pub fn set_mark(m: &mut dyn Mem, scene: u32, col: u32, row: u32, bits: u8) {
    if let Some(a) = mark_cell(m, scene, col, row) {
        let v = m.r8(a) | bits;
        m.w8(a, v);
    }
}

/// `0x00630D40(ecx = escena, columna, fila, bits)`: borra `bits` de la marca de la casilla.
pub fn clear_mark(m: &mut dyn Mem, scene: u32, col: u32, row: u32, bits: u8) {
    if let Some(a) = mark_cell(m, scene, col, row) {
        let v = m.r8(a) & !bits;
        m.w8(a, v);
    }
}

/// `0x00561080(columna, fila, rumbo)` (`cdecl`): ¿la casilla a un paso en el rumbo (dos en los diagonales
/// `0x20` y `0xE0`) tiene en la capa alta una pieza de puerta (`0x29`, `0x2E`, `0x2F`)?
pub fn gate_tile_ahead(m: &mut dyn Mem, h: &mut dyn FigureHost, col: u32, row: u32, dir: u8) -> bool {
    let (mut c, mut r) = (col, row);
    h.step(m, &mut c, &mut r, dir);
    if dir == DIRS[1] || dir == DIRS[7] {
        h.step(m, &mut c, &mut r, dir);
    }
    let v = view(m);
    let w = m.r32(v + 0xC314);
    if c >= w || r >= m.r32(v + 0xC318) {
        return false;
    }
    let up = m.r32(v + 0xC30C);
    let id = m.r16(up.wrapping_add(w.wrapping_mul(r).wrapping_add(c).wrapping_mul(4)));
    if id == 0 {
        return false;
    }
    let tiles = m.r32(v + 0x98);
    let obj = m.r32(tiles.wrapping_add(u32::from(id) * 4));
    matches!(h.tile_kind(m, obj), 0x29 | 0x2E | 0x2F)
}

/// `0x00560850(ecx = caminante, columna, fila, reglas)` (método `+0x5C` de todas las clases): ¿puede pisar la
/// casilla? No si su marca llega a `reglas`; con `reglas <= 1`, tampoco si en ella hay otra entidad que no sea una
/// figura de la batalla (clases 8..0xB, 0x12, 0x15..0x17, 0x20, 0x23, 0x24).
pub fn tile_free(m: &mut dyn Mem, h: &mut dyn FigureHost, w: u32, col: u32, row: u32, rules: u8) -> bool {
    let city = m.r8(w + 0x6A);
    let rec = siege_of(m, city);
    let scene = m.r32(rec + 8);
    if mark_at_least(m, scene, col, row, rules) {
        return false;
    }
    if rules > 1 {
        return true;
    }
    let v = view(m);
    let width = m.r32(v + 0xC314);
    let (c, r) = (col & 0xFFFF, row & 0xFFFF);
    if c >= width || r >= m.r32(v + 0xC318) {
        return true;
    }
    let cell = m.r32(v + 0xC310).wrapping_add(r.wrapping_mul(width).wrapping_add(c).wrapping_mul(4));
    let mut slot = m.r8(cell + 2);
    let id = m.r16(cell);
    let mut e = super::ld32(m, v + 0x3FC, u32::from(id) * 4);
    let link = m.r16(w + 0x16);
    while e != 0 {
        if m.r16(e + 0x16) != link {
            let class = vcall(m, h, e, 0x20, &[]) as u8;
            if !matches!(class, 8..=0xB | 0x12 | 0x15..=0x17 | 0x20 | 0x23 | 0x24) {
                return false;
            }
        }
        e = super::step::links_next(m, e, &mut slot);
    }
    true
}

/// `0x00560750(ecx = caminante, columna, fila, conjunto)`: rumbo `+0x3C`, pasos y velocidad hacia el píxel de la
/// casilla; la guarda como destino `+0x6C/+0x6E`.
pub fn begin_step(m: &mut dyn Mem, h: &mut dyn FigureHost, w: u32, col: u32, row: u32, set: u32) {
    let (px, py) = h.cell_to_pixel(m, col, row);
    let dx = px.wrapping_sub(s16(m, w + 0xE));
    let dy = py.wrapping_sub(s16(m, w + 0x12));
    let heading = if dx > 3 {
        if dy > 3 {
            0x60
        } else if dy < -3 {
            0x20
        } else {
            0x40
        }
    } else if dx < -3 {
        if dy > 3 {
            0xA0
        } else if dy < -3 {
            0xE0
        } else {
            0xC0
        }
    } else if dy > 0 {
        0x80
    } else {
        0
    };
    m.w8(w + 0x3C, heading);
    let long = dx.wrapping_abs() > 3 || dy.wrapping_abs() > 3;
    let span: u32 = if long { 0x3AB20 } else { 0x53020 };
    let steps = span / m.r32(set + 0xC);
    m.w16(w + 0x38, 0);
    m.w16(w + 0x3A, steps as u16);
    let n = steps as i32;
    let vx = px.wrapping_sub(s16(m, w + 0xE)).wrapping_shl(16).wrapping_div(n);
    m.w32(w + 0x40, vx as u32);
    let vy = py.wrapping_sub(s16(m, w + 0x12)).wrapping_shl(16).wrapping_div(n);
    m.w16(w + 0x6E, row as u16);
    m.w32(w + 0x44, vy as u32);
    m.w16(w + 0x6C, col as u16);
}

/// `0x00560970(ecx = caminante, conjunto, _, _, _, ataque)`: animación del estado `+0x34` (andar, combatir o
/// quieto). En la cabeza de una cola de puerta con objetivo (`escena+0x214`/`+0x210` con `+0x8C`) el caminante
/// lo fija en `+0x74` y usa las variantes de asalto; los de estado de marcha crean además el efecto `0x12` de
/// la escalera (`+0x64`). Devuelve el fotograma inicial (o el actual `+0x15` si el estado no anima).
pub fn select_state_animation(m: &mut dyn Mem, h: &mut dyn FigureHost, w: u32, set: u32, attack: bool) -> u32 {
    m.w16(w + 0x38, 0);
    m.w16(w + 0x3A, 1);
    let mut assault = false;
    // Sin cabeza de cola, el original deja aquí el byte bajo de `this` (el `push ecx` del prólogo).
    let mut variant = w as u8;
    let city = m.r8(w + 0x6A);
    let town = m.r32(TOWNS).wrapping_add(u32::from(city) * 0x9F8);
    if m.r8(w + 0x59) != 0 && m.r32(town + 0x40) != 0 && m.r8(w + 0x5A) == 0 {
        let rec = siege_of(m, city);
        if m.r32(rec + 0xC) == w {
            let t = super::ld32(m, rec + 8, 0x214);
            if t != 0 && m.r32(t + 0x8C) != 0 {
                m.w32(w + 0x74, t);
                assault = true;
            }
        }
        if m.r32(rec + 0x10) == w {
            let t = super::ld32(m, rec + 8, 0x210);
            if t != 0 && m.r32(t + 0x8C) != 0 {
                m.w32(w + 0x74, t);
                assault = true;
            }
        }
        if assault {
            match vcall(m, h, w, 0x68, &[]) as u8 {
                1 => variant = 0x28,
                2 => variant = 0x29,
                3 => variant = 0x2A,
                _ => {}
            }
        }
    }
    let anim = w + 0x2C;
    let state = m.r8(w + 0x34);
    let select = |m: &mut dyn Mem, code: u8| u32::from(sprite::anim_select(m, anim, set, code));
    match state {
        0x02 | 0x05 | 0xB2 | 0xCC => {
            if !assault {
                return select(m, 0x8C);
            }
            let t = m.r32(w + 0x74);
            let base = if t != 0 { t + 8 } else { 0 };
            let heading = m.r8(base + 0xC).wrapping_add(0x80);
            // El rumbo va en `DL`; el resto de `EDX` es lo que quedó del prólogo (la escena si el caminante es la
            // cabeza de la segunda cola, si no la base de los registros de asedio).
            let rec = siege_of(m, city);
            let hi = if m.r32(rec + 0x10) == w { m.r32(rec + 8) } else { m.r32(super::SIEGES) };
            let x = m.r32(w + 0xC);
            let y = m.r32(w + 0x10).wrapping_add(0xD_0000);
            let eff = h.spawn_entity(m, [x, y, 0x12, 0x46_0000 + u32::from(variant), (hi & !0xFF) | u32::from(heading), 0, 0]);
            m.w32(w + 0x64, eff);
            h.virt(m, 0x0057_B110, eff, &[0x8C]);
            sprite::anim_set_default(m, eff + 0x20, 0xF);
            select(m, 0xCC)
        }
        0x0B | 0x0D | 0x0F | 0x13 | 0x14 | 0xAC | 0xCE => {
            if assault {
                return select(m, 0xEC);
            }
            if attack && sprite::anim_find(m, set, 0xE) != 0xFF {
                return select(m, 0x8E);
            }
            if state == 0xF && sprite::anim_find(m, set, 0x14) != 0xFF {
                return select(m, 0x14);
            }
            select(m, 0xF)
        }
        0x10 | 0xAE => select(m, if assault { 0xEE } else { 0x10 }),
        _ => u32::from(m.r8(w + 0x15)),
    }
}

/// ¿El caminante es la cabeza de una de las dos colas de puerta del registro?
fn gate_head(m: &mut dyn Mem, rec: u32, w: u32) -> bool {
    m.r32(rec + 0xC) == w || m.r32(rec + 0x10) == w
}

/// Fija el rango de la figura `next` y propaga a las siguientes (cuerpo en línea de `0x005614E0`).
fn rank_chain(m: &mut dyn Mem, next: u32, v: u8) {
    if next != 0 && m.r8(next + 0x7A) <= v {
        propagate_queue_rank(m, next, v.wrapping_add(1));
    }
}

/// Primer rumbo de `dir`, `dir + 0x40` y `dir - 0x40` con pieza de puerta delante (`0x00561080`).
fn probe(m: &mut dyn Mem, h: &mut dyn FigureHost, col: u32, row: u32, dir: u8) -> Option<u8> {
    [dir, dir.wrapping_add(0x40), dir.wrapping_sub(0x40)].into_iter().find(|&d| gate_tile_ahead(m, h, col, row, d))
}

/// `0x00560CB0(ecx = caminante, conjunto, columna, fila, bits, _)`: paso por la cola de la puerta. Sin asalto en
/// marcha deambula sobre las piezas de puerta (gira al azar con `rand`); en el asalto avanza por la cola según
/// su rango `+0x7A` y su posición `+0x5A`, sin entrar en casillas marcadas con `bits`. Devuelve si arrancó un
/// paso.
pub fn pick_queue_step(m: &mut dyn Mem, h: &mut dyn FigureHost, w: u32, set: u32, col: u32, row: u32, bits: u8) -> bool {
    let city = m.r8(w + 0x6A);
    let rec = siege_of(m, city);
    let scene = m.r32(rec + 8);
    if !siege_active(m, rec) || m.r16(scene + 0x3A) != 0 {
        // Deambular.
        let d = m.r8(w + 0x3C) | 0x20;
        m.w8(w + 0x3C, d);
        if h.rand(m) & 0x1F == 0 {
            m.w8(w + 0x3C, d.wrapping_add(0x80));
        }
        let d = m.r8(w + 0x3C);
        if !gate_tile_ahead(m, h, col, row, d) {
            let turn = if gate_tile_ahead(m, h, col, row, d.wrapping_add(0x40)) {
                Some(0x40u8)
            } else if gate_tile_ahead(m, h, col, row, d.wrapping_sub(0x40)) {
                Some(0xC0)
            } else {
                None
            };
            match turn {
                Some(t) => {
                    m.w8(w + 0x3C, d.wrapping_add(t));
                    let g = m.r8(w + 0x79).wrapping_add(t);
                    m.w8(w + 0x79, g);
                }
                None => m.w8(w + 0x3C, d.wrapping_add(0x80)),
            }
        }
        return finish(m, h, w, set, col, row, bits, scene);
    }
    let moving = vcall(m, h, w, 0x60, &[0]) as u8 != 0;
    let rank = m.r8(w + 0x7A);
    let pos = m.r8(w + 0x5A);
    let busy = moving || rank > pos || gate_head(m, rec, w);
    if !busy || m.r8(scene + 0x79) != 0 {
        let v = pos.wrapping_add(1);
        if rank != v {
            m.w8(w + 0x7A, v);
            let next = m.r32(w + 0x5C);
            rank_chain(m, next, v);
        }
    } else if rank == pos && pos != 0 && (pos > 4 || gate_head(m, rec, w)) {
        // Predecesor en las colas `+0xC` y `+0x10`; sin encontrarlo, el original se queda con la cabeza de la
        // segunda.
        let mut found = m.r32(rec + 0xC);
        let mut list = 0u8;
        while list < 2 {
            if found == 0 {
                found = m.r32(rec + 0x10);
                list += 1;
                continue;
            }
            let next = m.r32(found + 0x5C);
            if next == w {
                break;
            }
            found = next;
        }
        if found == 0 || m.r8(found + 0x5A) >= m.r8(found + 0x7A) {
            m.w8(w + 0x7A, rank.wrapping_sub(1));
        }
    }
    let rank = m.r8(w + 0x7A);
    let pos = m.r8(w + 0x5A);
    if rank < pos {
        if pos <= 4 && !gate_head(m, rec, w) {
            return false;
        }
        let d = m.r8(w + 0x79).wrapping_add(0x80);
        let Some(d) = probe(m, h, col, row, d) else {
            return false;
        };
        let (mut c, mut r) = (col, row);
        h.step(m, &mut c, &mut r, d);
        if mark_at_least(m, scene, c, r, bits) {
            return false;
        }
        m.w8(w + 0x79, d.wrapping_add(0x80));
        m.w8(w + 0x3C, d);
        return finish(m, h, w, set, col, row, bits, scene);
    }
    if rank == pos {
        return false;
    }
    let next = m.r32(w + 0x5C);
    if next != 0 {
        let v = i32::from(pos) + 1;
        let nr = i32::from(m.r8(next + 0x7A));
        if v >= nr {
            let a = pos.wrapping_add(2);
            if m.r8(next + 0x7A) != a {
                m.w8(next + 0x7A, a);
                let n2 = m.r32(next + 0x5C);
                rank_chain(m, n2, a);
            }
            return false;
        }
        if i32::from(m.r8(next + 0x5A)) == v {
            return false;
        }
    }
    let g = m.r8(w + 0x79);
    let Some(d) = probe(m, h, col, row, g) else {
        return false;
    };
    m.w8(w + 0x79, d);
    m.w8(w + 0x3C, d);
    finish(m, h, w, set, col, row, bits, scene)
}

/// Final de `0x00560CB0`: paso a la casilla del rumbo `+0x3C` y marca de la escena.
#[allow(clippy::too_many_arguments)]
fn finish(m: &mut dyn Mem, h: &mut dyn FigureHost, w: u32, set: u32, col: u32, row: u32, bits: u8, scene: u32) -> bool {
    let d = m.r8(w + 0x3C);
    m.w8(w + 0x70, 1);
    let (mut c, mut r) = (col, row);
    h.step(m, &mut c, &mut r, d);
    begin_step(m, h, w, c, r, set);
    if scene != 0 {
        set_mark(m, scene, c, r, bits);
    }
    true
}

/// Bit de calle (`0x80`) del plano de la ciudad del caminante en `(col, row)`.
fn plan_street(m: &mut dyn Mem, w: u32, col: u32, row: u32) -> bool {
    let city = m.r8(w + 0x6A);
    let town = m.r32(TOWNS).wrapping_add(u32::from(city) * 0x9F8);
    let stride = m.r32(town + 0x7AC);
    let plan = m.r32(town + 0x80C);
    m.r8(stride.wrapping_mul(col).wrapping_add(plan).wrapping_add(row)) & 0x80 != 0
}

/// ¿La casilla está fuera de la vista o su pieza de suelo es una puerta (`0x18`)?
fn gate_or_outside(m: &mut dyn Mem, h: &mut dyn FigureHost, col: u32, row: u32) -> bool {
    let v = view(m);
    let w = m.r32(v + 0xC314);
    if col >= w || row >= m.r32(v + 0xC318) {
        return true;
    }
    ground_kind(m, h, w.wrapping_mul(row).wrapping_add(col)) == 0x18
}

/// `0x0055FFF0(ecx = caminante, conjunto, columna, fila, mínimo, bits, reglas, umbral)`: casilla siguiente por el
/// campo de distancias del asedio (`0x0062B9C0`). Prueba los rumbos a ±0, ±0x20, ..., ±0x80 del dibujado; las
/// casillas deben ser transitables (método `+0x5C` con `reglas`). Fuera de las calles del plano (o del lado que no
/// las recorre) baja la distancia sin pasar del `mínimo` y prefiere las que superan el `umbral`; en el plano con
/// la escena `+0x79` el bando 1 sube por las calles. Si mejora (o empata girando poco), fija `+0x3C`, marca la
/// casilla con `bits` y arranca el paso.
#[allow(clippy::too_many_arguments)]
pub fn pick_next_tile(m: &mut dyn Mem, h: &mut dyn FigureHost, w: u32, set: u32, col: u32, row: u32, min: u16, bits: u8, rules: u32, threshold: u16) -> bool {
    let city = m.r8(w + 0x6A);
    let rec = siege_of(m, city);
    let scene = m.r32(rec + 8);
    if scene == 0 {
        return false;
    }
    let mut cur = flow_distance(m, h, scene, col, row);
    if cur == 0xFFFF {
        return false;
    }
    let reach = u32::from(m.r16(w + 0x80));
    let kind = m.r8(w + 0x7B);
    let mut gate = false;
    let d0 = distance_to_target(m, scene, col, row) & 0xFFFF;
    let near = d0 as i32 <= reach as i32 + 1 && gate_or_outside(m, h, col, row);
    if near && m.r8(scene + 0x78) == 0 && kind == 0 {
        cur = cur.wrapping_add(2);
        gate = true;
    } else if m.r8(scene + 0x78) != 0 && kind & 1 == 0 {
        let d = distance_to_target(m, scene, col, row) & 0xFFFF;
        if (d as i32) < reach as i32 + 10 && distance_to_target(m, scene, col, row) as u16 > reach as u16 {
            if gate_or_outside(m, h, col, row) {
                cur = cur.wrapping_add(2);
            }
            gate = true;
        }
    }
    let climb = m.r8(scene + 0x79) != 0 && kind & 1 != 0 && plan_street(m, w, col, row);
    let mut best: u16 = if climb { 0 } else { 0xFFFF };
    let (mut best_col, mut best_row) = (0u32, 0u32);
    let mut best_dir = 0u8;
    let heading = m.r8(w + 0x14);
    let mut delta = 0u8;
    loop {
        for side in 0..2 {
            let dir = if side == 0 { heading.wrapping_sub(delta) } else { heading.wrapping_add(delta) };
            let (mut c, mut r) = (col, row);
            h.step(m, &mut c, &mut r, dir);
            let mut d = flow_distance(m, h, scene, c, r);
            if gate && gate_or_outside(m, h, c, r) {
                d = d.wrapping_add(1);
            }
            if climb {
                if d <= best || !plan_street(m, w, c, r) {
                    continue;
                }
                if vcall(m, h, w, 0x5C, &[c, r, rules]) as u8 == 0 {
                    continue;
                }
                best = d;
                best_col = c;
                best_row = r;
                // El original guarda en los dos lados el rumbo de la izquierda.
                best_dir = heading.wrapping_sub(delta);
                continue;
            }
            if d >= best || d < min {
                continue;
            }
            // Lejos del destino: a la izquierda pasado el radio, a la derecha ya en él (`ja` frente a `jae`).
            let dist = distance_to_target(m, scene, c, r) as u16;
            let far = if side == 0 { dist > reach as u16 } else { dist >= reach as u16 };
            if !far && gate_or_outside(m, h, c, r) && m.r8(scene + 0x78) == 0 && kind != 1 {
                continue;
            }
            if vcall(m, h, w, 0x5C, &[c, r, rules]) as u8 == 0 {
                continue;
            }
            if d > threshold || (d == threshold && d < cur) {
                best = d;
                best_col = c;
                best_row = r;
                best_dir = dir;
            }
        }
        delta = delta.wrapping_add(0x20);
        if delta > 0x80 {
            break;
        }
    }
    let mut turn = best_dir.wrapping_sub(heading);
    if turn > 0x80 {
        turn = turn.wrapping_neg();
    }
    let (lo, hi) = if climb {
        // Sin casilla mejor el original compara con 0 (`best` no llegó a la mitad del rango).
        (cur, if best < 0x7FFF { best } else { 0 })
    } else {
        (best, cur)
    };
    let take = lo < hi || (lo == hi && turn <= 0x40 && u32::from(hi) >= u32::from(min) + 2);
    if !take {
        return false;
    }
    m.w8(w + 0x3C, best_dir);
    set_mark(m, scene, best_col, best_row, bits);
    m.w8(w + 0x70, 1);
    begin_step(m, h, w, best_col, best_row, set);
    true
}
