//! Batalla dentro de la ciudad durante un asedio con la vista abierta (`TownSiege+0x77 != 0`): campo de rutas por
//! fotograma, colocación y aparición de figuras, fotograma de la vista y cierre. Continúa
//! [`crate::economy::siege_view`] (campo, cola por niveles, marcadores de entrada y prefijo de la apertura) sobre la
//! misma memoria del proceso original ([`Mem`]) y el mismo anfitrión ([`SiegeViewHost`]):
//!
//! * `0x00632E90` saca el primer nodo de la cola por niveles; `0x00633060` lo saca y expande sus ocho vecinas
//!   (columnas a coste 2, filas dobles a coste 3, `+0x100` si solo se pasa sin la prueba cercana); `0x0062B9C0`
//!   es la consulta de las figuras: expande hasta que la casilla tiene valor y devuelve la mitad.
//! * `0x0062F290` decide si una figura puede ponerse en una casilla (campo, plano, puerta y distancia).
//! * `0x00593580` (de la vista) dice si la casilla está libre para una entidad; `0x00561080` si el paso en un rumbo
//!   da con una pieza de puerta.
//! * `0x006307B0` (defensores en espiral), `0x00630A80` (marcadores de las tres puertas), `0x006311E0` (milicia de
//!   la puerta), `0x0062D170` (asaltantes), `0x00631120`/`0x00630DC0` (reencolado de las figuras de la puerta).
//! * `0x0062C730` completo, `0x00631A50` (fotograma de la vista) y `0x0062F5A0` (cierre).
//!
//! El registro de asedio por ciudad (`[0x006DF270] + 0x1024 * ciudad`) guarda las listas de figuras vivas: `+0xC` y
//! `+0x10` cabezas de las colas de la puerta, `+0x14` defensores (clase 8), `+0x410`, `+0x80C` y `+0xC08` milicia
//! (clases 9, 0xA y 0xB), cuántas de cada en `+0x1004..+0x100B` y contadores por puerta en `+0x100E/+0x100F`.
//! Ese estado no se guarda en la partida (`0x006335A0` solo escribe cabecera, pares, acuerdos, pedidos y escena).
use crate::economy::siege_view::{flow_cell, ftol_sqrt, height, is_cell_passable, queue_push, view, width, SiegeViewHost, RING_DX, TOWNS, TOWN_SIZE};
use crate::flatmem::Mem;

/// Puntero global a los registros de asedio por ciudad.
pub const SIEGE_RECORDS: u32 = 0x006D_F270;
/// Tamaño de cada registro de asedio.
pub const SIEGE_RECORD_SIZE: u32 = 0x1024;
/// Rumbos (`0x00677928`): 0x00, 0x20, ..., 0xE0.
pub const HEADING: [u8; 8] = [0x00, 0x20, 0x40, 0x60, 0x80, 0xA0, 0xC0, 0xE0];
/// Reloj de la partida (`0x006DE4B4`).
pub const TICK: u32 = 0x006D_E4B4;
/// Reloj de fotogramas (`0x006DCCF8`).
pub const FRAME_CLOCK: u32 = 0x006D_CCF8;

/// Registro de asedio de la ciudad de la escena.
pub fn siege_record(m: &mut dyn Mem, s: u32) -> u32 {
    let city = u32::from(m.r8(s + 0x5D3));
    m.r32(SIEGE_RECORDS).wrapping_add(city.wrapping_mul(SIEGE_RECORD_SIZE))
}

/// Ciudad de la escena en la tabla de ciudades.
pub fn town(m: &mut dyn Mem, s: u32) -> u32 {
    let city = u32::from(m.r8(s + 0x5D3));
    m.r32(TOWNS).wrapping_add(city.wrapping_mul(TOWN_SIZE))
}

/// `town_get_building_record` (`0x0051F1F0`): registro de edificio `id` de la ciudad, o 0 fuera de la tabla.
pub fn building_record(m: &mut dyn Mem, town: u32, id: u32) -> u32 {
    if id < u32::from(m.r16(town + 0x776)) {
        m.r32(town + 0x75C).wrapping_add(id.wrapping_mul(8))
    } else {
        0
    }
}

/// Celda del plano de la ciudad (`+0x80C`, paso `+0x7AC`): `plano[paso * x + y]`.
fn plan_cell(m: &mut dyn Mem, town: u32, x: u32, y: u32) -> u8 {
    let stride = m.r32(town + 0x7AC);
    let plan = m.r32(town + 0x80C);
    m.r8(stride.wrapping_mul(x).wrapping_add(plan).wrapping_add(y))
}

/// Nodo de la cola por niveles.
fn node(m: &mut dyn Mem, s: u32, i: u16) -> u32 {
    m.r32(s + 0x5D8).wrapping_add(u32::from(i) * 6)
}

/// Reajuste de los niveles al sacar un nodo (común a `0x00632E90` y `0x00633060`). Devuelve `true` si la cola se
/// ha vaciado y liberado.
fn advance_levels(m: &mut dyn Mem, h: &mut dyn SiegeViewHost, s: u32, next: u16) -> Option<bool> {
    let link = |m: &mut dyn Mem, at: u32| {
        let i = m.r16(s + at);
        let n = node(m, s, i);
        m.r16(n + 4)
    };
    if link(m, 0x59A) == next {
        let q = m.r32(s + 0x5D8);
        let base = m.r16(s + 0x5A0);
        m.w16(s + 0x5A0, base.wrapping_add(2));
        if q != 0 {
            h.free(m, q);
        }
        m.w32(s + 0x5D8, 0);
        m.w16(s + 0x590, 0);
        m.w16(s + 0x592, 0);
        return Some(true);
    }
    if link(m, 0x598) == next {
        let t = m.r16(s + 0x59A);
        let base = m.r16(s + 0x5A0);
        m.w16(s + 0x5A0, base.wrapping_add(2));
        for at in [0x594, 0x596, 0x598] {
            m.w16(s + at, t);
        }
        return Some(false);
    }
    if link(m, 0x596) == next {
        let (a, t) = (m.r16(s + 0x598), m.r16(s + 0x59A));
        let base = m.r16(s + 0x5A0);
        m.w16(s + 0x5A0, base.wrapping_add(2));
        m.w16(s + 0x594, a);
        m.w16(s + 0x596, t);
        m.w16(s + 0x598, t);
        return Some(false);
    }
    if link(m, 0x594) == next {
        let (a, b, t) = (m.r16(s + 0x596), m.r16(s + 0x598), m.r16(s + 0x59A));
        let base = m.r16(s + 0x5A0);
        m.w16(s + 0x5A0, base.wrapping_add(1));
        m.w16(s + 0x594, a);
        m.w16(s + 0x596, b);
        m.w16(s + 0x598, t);
        return Some(false);
    }
    None
}

/// `town_siege_distance_queue_pop` (`0x00632E90`): saca el primer nodo cuyo valor no supera el nivel base `+0x5A0`
/// (los demás vuelven a la cola) y ajusta los niveles; devuelve su casilla.
pub fn queue_pop(m: &mut dyn Mem, h: &mut dyn SiegeViewHost, s: u32) -> (u16, u16) {
    loop {
        let head = m.r16(s + 0x592);
        let n = node(m, s, head);
        let x = m.r16(n);
        let n = node(m, s, head);
        let y = m.r16(n + 2);
        let n = node(m, s, head);
        let next = m.r16(n + 4);
        m.w16(s + 0x592, next);
        match advance_levels(m, h, s, next) {
            Some(true) => {
                m.w8(s + 0x5D5, 0);
                return (x, y);
            }
            Some(false) => {
                m.w8(s + 0x5D5, 0);
                return (x, y);
            }
            None => {
                let base = m.r16(s + 0x5A0);
                let cell = flow_cell(m, s, u32::from(x), u32::from(y));
                if base >= m.r16(cell) {
                    m.w8(s + 0x5D5, 0);
                    return (x, y);
                }
                queue_push(m, h, s, u32::from(x), u32::from(y));
            }
        }
    }
}

/// Expansión de las ocho vecinas de la casilla `(x, y)` (cuerpo común de `0x00633060` y `0x0062B9C0`).
fn expand_from(m: &mut dyn Mem, h: &mut dyn SiegeViewHost, s: u32, x: u16, y: u16) {
    let (mut cx, mut cy) = (u32::from(x), u32::from(y));
    let cell = flow_cell(m, s, cx, cy);
    let d = m.r16(cell);
    h.step(m, &mut cx, &mut cy, HEADING[7]);
    let mut k: u8 = 0;
    for row in 0..2u16 {
        let extra = u32::from(row);
        let mut p = flow_cell(m, s, cx, cy);
        let end = k.wrapping_add(4);
        if k < end {
            let mut j = usize::from(k);
            for _ in 0..end.wrapping_sub(k) {
                if cx < width(m) && cy < height(m) {
                    let v = m.r16(p);
                    if v == 0 {
                        let near = m.r8(s + 0x6F) != 0xFF;
                        if is_cell_passable(m, h, s, cx, cy, near) {
                            m.w16(p, row.wrapping_add(d).wrapping_add(2));
                            queue_push(m, h, s, cx, cy);
                        } else if m.r8(s + 0x6F) != 0xFF && is_cell_passable(m, h, s, cx, cy, false) {
                            m.w16(p, row.wrapping_add(d).wrapping_add(0x102));
                            queue_push(m, h, s, cx, cy);
                        } else {
                            m.w16(p, 0xFFFF);
                        }
                    } else if v != 0xFFFF {
                        let c = u32::from(d).wrapping_add(extra);
                        let mut far = true;
                        if (u32::from(v) as i32) > (c.wrapping_add(2) as i32) {
                            let near = m.r8(s + 0x6F) != 0xFF;
                            if is_cell_passable(m, h, s, cx, cy, near) {
                                far = false;
                                let now = m.r16(p);
                                m.w16(p, row.wrapping_add(d).wrapping_add(2));
                                if now <= d {
                                    queue_push(m, h, s, cx, cy);
                                }
                            }
                        }
                        if far && m.r8(s + 0x6F) != 0xFF {
                            let now = u32::from(m.r16(p));
                            if (now as i32) > (c.wrapping_add(0x102) as i32) && is_cell_passable(m, h, s, cx, cy, false) {
                                m.w16(p, row.wrapping_add(d).wrapping_add(0x102));
                            }
                        }
                    }
                }
                let dx = RING_DX[j & 7] as u32;
                cx = cx.wrapping_add(dx);
                cy = cy.wrapping_add(crate::economy::siege_view::RING_DY[j & 7] as u32);
                let off = m.r32(s + 0x630 + 4 * j as u32);
                p = p.wrapping_add(off.wrapping_add(dx).wrapping_mul(2));
                j += 1;
            }
        }
        h.step(m, &mut cx, &mut cy, HEADING[1]);
        k = end;
    }
}

/// `town_siege_distance_field_expand_step` (`0x00633060`): saca un nodo de la cola y expande sus vecinas.
pub fn expand_step(m: &mut dyn Mem, h: &mut dyn SiegeViewHost, s: u32) {
    let (x, y) = queue_pop(m, h, s);
    expand_from(m, h, s, x, y);
}

/// `town_siege_get_flow_field_distance` (`0x0062B9C0`): distancia (mitad del valor del campo) de la casilla al
/// destino del asedio, expandiendo la cola hasta que la casilla tenga valor; `0xFFFF` sin vista, fuera del mapa o
/// si la cola se agota antes.
pub fn flow_distance(m: &mut dyn Mem, h: &mut dyn SiegeViewHost, s: u32, x: u32, y: u32) -> u16 {
    if m.r8(s + 0x77) == 0 {
        return 0xFFFF;
    }
    let (x, y) = (x & 0xFFFF, y & 0xFFFF);
    if x >= width(m) || y >= height(m) {
        return 0xFFFF;
    }
    loop {
        let cell = flow_cell(m, s, x, y);
        let v = m.r16(cell);
        if v != 0 {
            return v >> 1;
        }
        if m.r16(s + 0x590) == 0 {
            return 0xFFFF;
        }
        expand_step(m, h, s);
    }
}

/// Valor del campo en `(x, y)`, expandiendo mientras valga 0 y la cola exista; `None` si la cola se agota.
fn settle(m: &mut dyn Mem, h: &mut dyn SiegeViewHost, s: u32, x: u32, y: u32) -> Option<u16> {
    loop {
        let cell = flow_cell(m, s, x, y);
        let v = m.r16(cell);
        if v != 0 {
            return Some(v);
        }
        if m.r16(s + 0x590) == 0 {
            return None;
        }
        expand_step(m, h, s);
    }
}

/// `town_siege_can_place_unit_at_cell` (`0x0062F290`): ¿puede ponerse una figura en `(x, y)`? `near` (`arg3`) pide
/// distancia a la puerta y `inside` (`arg4 == 1`) el lado de dentro de la muralla según el plano.
pub fn can_place_unit(m: &mut dyn Mem, h: &mut dyn SiegeViewHost, s: u32, x: u32, y: u32, near: bool, inside: u8) -> bool {
    if m.r8(s + 0x77) == 0 {
        return false;
    }
    let (cx, cy) = (x & 0xFFFF, y & 0xFFFF);
    if cx >= width(m) || cy >= height(m) {
        return false;
    }
    if settle(m, h, s, cx, cy).is_none() {
        return false;
    }
    let plan_inside = |m: &mut dyn Mem| {
        let t = town(m, s);
        plan_cell(m, t, x, y) >> 7 != 0
    };
    if inside == 1 {
        if m.r8(s + 0x78) != 0 {
            return true;
        }
        return plan_inside(m);
    }
    if m.r16(s + 0x3C) != 0 {
        if m.r16(s + 8) == 0xFFFF {
            return true;
        }
        return plan_inside(m);
    }
    let t = town(m, s);
    if plan_cell(m, t, x, y) & 0x80 != 0 {
        return false;
    }
    if m.r16(s + 0x10) == m.r16(s + 8) && m.r16(s + 0x12) == m.r16(s + 0xA) {
        if m.r8(s + 0x78) != 0 && near {
            return true;
        }
        if m.r8(s + 0x77) != 0 {
            let close = cx < width(m) && cy < height(m);
            if close {
                if let Some(v) = settle(m, h, s, cx, cy) {
                    if v >> 1 < 8 {
                        return false;
                    }
                }
            }
            if m.r8(s + 0x77) != 0 && cx < width(m) && cy < height(m) {
                if let Some(v) = settle(m, h, s, cx, cy) {
                    if v >> 1 < 0xC && near {
                        return false;
                    }
                }
            }
        }
    }
    let dx = (i32::from(m.r16(s + 8) as i16) - i32::from(x as u16 as i16)).wrapping_mul(2);
    let dy = i32::from(m.r16(s + 0xA) as i16) - i32::from(y as u16 as i16);
    let d = ftol_sqrt(dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx)));
    if d as u16 > 0x19 {
        return true;
    }
    let v = view(m);
    let idx = m.r32(v + 0xC314).wrapping_mul(y).wrapping_add(x);
    let layer = m.r32(v + 0xC308);
    let ground = m.r16(layer.wrapping_add(idx.wrapping_mul(4)));
    let tiles = m.r32(v + 0x98);
    let object = m.r32(tiles.wrapping_add(u32::from(ground) * 4));
    h.tile_kind(m, object) != 0x18
}

/// `class22_is_cell_walkable_for_entity` (`0x00593580`) sobre la memoria de la vista: sin pieza superior (o de
/// paso 0x30/0x31), sin entidades de otro enlace (`+0x16 != link`, cadena `0x00559970` de la capa `+0xC310` y la
/// tabla `+0x3FC`) y con suelo 0x10, 0x17, 0x18 o 0x28 (este último no si `flag == 0xFF`).
pub fn cell_walkable_flat(m: &mut dyn Mem, h: &mut dyn SiegeViewHost, col: u32, row: u32, flag: u8, link: u16) -> bool {
    let v = view(m);
    let idx = m.r32(v + 0xC314).wrapping_mul(row).wrapping_add(col);
    let upper = m.r32(v + 0xC30C);
    let up = m.r16(upper.wrapping_add(idx.wrapping_mul(4)));
    if up != 0 {
        let tiles = m.r32(v + 0x98);
        let object = m.r32(tiles.wrapping_add(u32::from(up) * 4));
        let k = h.tile_kind(m, object);
        if !(0x30..=0x31).contains(&k) {
            return false;
        }
    }
    let v = view(m);
    let links = m.r32(v + 0xC310).wrapping_add(idx.wrapping_mul(4));
    let mut slot = m.r8(links + 2);
    let first = m.r16(links);
    let table = m.r32(v + 0x3FC);
    let mut entity = m.r32(table.wrapping_add(u32::from(first) * 4));
    while entity != 0 {
        if m.r16(entity + 0x16) != link {
            return false;
        }
        let at = m.r32(entity + 4).wrapping_add(u32::from(slot) * 4);
        let next = m.r16(at);
        if next == 0 {
            break;
        }
        slot = m.r8(at + 2);
        let v = view(m);
        let table = m.r32(v + 0x3FC);
        entity = m.r32(table.wrapping_add(u32::from(next) * 4));
    }
    let v = view(m);
    let ground_layer = m.r32(v + 0xC308);
    let ground = m.r16(ground_layer.wrapping_add(idx.wrapping_mul(4)));
    let tiles = m.r32(v + 0x98);
    let object = m.r32(tiles.wrapping_add(u32::from(ground) * 4));
    let k = h.tile_kind(m, object);
    if flag == 0xFF && k == 0x28 {
        return false;
    }
    matches!(k, 0x10 | 0x17 | 0x18 | 0x28)
}

/// `0x00593580` por el anfitrión o, si no la resuelve, sobre la memoria de la vista.
pub fn cell_walkable(m: &mut dyn Mem, h: &mut dyn SiegeViewHost, col: u32, row: u32, flag: u8, link: u16) -> bool {
    match h.cell_walkable(m, col, row, flag, link) {
        Some(v) => v,
        None => cell_walkable_flat(m, h, col, row, flag, link),
    }
}

/// `scene_step_hits_special_object` (`0x00561080`): el paso desde `(x, y)` en el rumbo `dir` (doble en 0x20 y
/// 0xE0) da con una pieza superior 0x29, 0x2E o 0x2F.
pub fn step_hits_gate(m: &mut dyn Mem, h: &mut dyn SiegeViewHost, x: u32, y: u32, dir: u8) -> bool {
    let (mut a, mut b) = (x, y);
    h.step(m, &mut a, &mut b, dir);
    if dir == HEADING[1] || dir == HEADING[7] {
        h.step(m, &mut a, &mut b, dir);
    }
    let v = view(m);
    if a >= m.r32(v + 0xC314) || b >= m.r32(v + 0xC318) {
        return false;
    }
    let idx = m.r32(v + 0xC314).wrapping_mul(b).wrapping_add(a);
    let upper = m.r32(v + 0xC30C);
    let up = m.r16(upper.wrapping_add(idx.wrapping_mul(4)));
    if up == 0 {
        return false;
    }
    let tiles = m.r32(v + 0x98);
    let object = m.r32(tiles.wrapping_add(u32::from(up) * 4));
    let k = h.tile_kind(m, object);
    k == 0x29 || (0x2E..=0x2F).contains(&k)
}

/// Entrada `k` de una lista de figuras del registro de asedio (`base` = `+0x14`, `+0x410`, `+0x80C` o `+0xC08`).
fn record_list(rec: u32, base: u32, k: u16) -> u32 {
    rec.wrapping_add(base).wrapping_add(u32::from(k) * 4)
}

/// `town_siege_spawn_defenders_around_cell` (`0x006307B0`): defensores (clase 8) en espiral alrededor de `(x, y)`
/// hasta llenar el cupo `+0x232` del registro (`+0x1004`), en casillas con campo, libres y donde se pueden poner;
/// guarda cada uno en la lista `+0x14`. No hace nada con la escena en espera reciente (`+0x3A` y el registro
/// empezado hace menos de 0x1B80 ticks) o al atacar (`+0x3C`). Para tras 100 vueltas.
pub fn spawn_defenders_around_cell(m: &mut dyn Mem, h: &mut dyn SiegeViewHost, s: u32, x: u32, y: u32) {
    if m.r16(s + 0x3A) != 0 {
        let rec = siege_record(m, s);
        let start = m.r32(rec).wrapping_add(0x1B80);
        if start > m.r32(TICK) {
            return;
        }
    }
    if m.r16(s + 0x3C) != 0 {
        return;
    }
    let (mut x, mut y) = (x, y);
    let mut heading: u8 = 0;
    let mut ring: u8 = 0;
    let mut steps: u8 = 0;
    loop {
        let mut more = false;
        let rec = siege_record(m, s);
        if m.r16(rec + 0x1004) < m.r16(s + 0x232) {
            more = true;
            if x < width(m) && y < height(m) && {
                let cell = flow_cell(m, s, x, y);
                m.r16(cell) != 0xFFFF
            } && cell_walkable(m, h, x, y, 0, 0xFFFF) && can_place_unit(m, h, s, x, y, true, 1) {
                let (px, py) = h.cell_to_pixel(m, x, y);
                let city = u32::from(m.r8(s + 0x5D3));
                let entity = h.spawn_entity(m, [(px as u32) << 16, (py as u32) << 16, 8, city + 0x10000, u32::from(HEADING[1]), 0, 0]);
                let rec = siege_record(m, s);
                let n = m.r16(rec + 0x1004);
                m.w32(record_list(rec, 0x14, n), entity);
                m.w16(rec + 0x1004, n.wrapping_add(1));
            }
        }
        let mut check = true;
        if steps < ring {
            h.step(m, &mut x, &mut y, heading);
            steps = steps.wrapping_add(1);
        } else if heading < 0xC0 && ring > 0 {
            heading = heading.wrapping_add(0x40);
            h.step(m, &mut x, &mut y, heading);
            steps = 1;
        } else {
            heading = 0xC0;
            check = true;
        }
        if check && heading == 0xC0 && steps == ring {
            h.step(m, &mut x, &mut y, HEADING[5]);
            ring = ring.wrapping_add(1);
            heading = 0;
            steps = 0;
        }
        if ring > 100 || !more {
            return;
        }
    }
}

/// `town_siege_spawn_gate_markers` (`0x00630A80`): marcadores (clase 0x19) de las tres puertas `+0x244..+0x24C`
/// en `+0x5FC..+0x604`, junto a la casilla del registro de edificio y con el rumbo según el lado de la puerta y el
/// plano; el de la puerta atacada (`+0x6D`) lleva `0xFF00` en el código y no se crea con la escena visible
/// (`+0x78`). Sin puerta elegida (`+8 == 0xFFFF`) no hace nada.
pub fn spawn_gate_markers(m: &mut dyn Mem, h: &mut dyn SiegeViewHost, s: u32) {
    if m.r16(s + 8) == 0xFFFF {
        return;
    }
    let mut id = u32::from(m.r8(s + 0x5D3));
    if m.r16(s + 0x42) < 0x200 {
        id = id.wrapping_add(0xFF_0000);
    }
    for k in 0..3u8 {
        let t = town(m, s);
        let gate = m.r32(s + 0x244 + 4 * u32::from(k));
        let rec = building_record(m, t, gate);
        let side = m.r8(rec + 3);
        let a = side == 0x37 || side == 0x34;
        let col = if a { u32::from(m.r8(rec)).wrapping_add(1) } else { u32::from(m.r8(rec)).wrapping_sub(1) };
        let row = u32::from(m.r8(rec + 1)).wrapping_sub(2);
        let t = town(m, s);
        let inside = plan_cell(m, t, col, row.wrapping_add(2)) & 0x80 != 0;
        let heading = match (inside, a) {
            (true, true) => HEADING[7],
            (true, false) => HEADING[1],
            (false, true) => HEADING[3],
            (false, false) => HEADING[5],
        };
        if k == m.r8(s + 0x6D) {
            if m.r8(s + 0x78) != 0 {
                continue;
            }
            id |= 0xFF00;
        } else {
            id &= 0x00FF_00FF;
        }
        let (px, py) = h.cell_to_pixel(m, col, row);
        let py = py.wrapping_sub(if heading & 0x40 != 0 { 8 } else { 7 });
        let mut px = px.wrapping_add(if heading & 0x40 != 0 { 2 } else { 5 });
        if heading < 0x80 {
            px = px.wrapping_add(1);
        }
        let entity = h.spawn_entity(m, [(px as u32) << 16, (py as u32) << 16, 0x19, id, u32::from(heading), 0, 0]);
        m.w32(s + 0x5FC + 4 * u32::from(k), entity);
    }
}

/// Reajuste del rumbo de la milicia: si el paso no da con la puerta prueba ±0x40; si ninguno, `ok = false`.
fn militia_heading(m: &mut dyn Mem, h: &mut dyn SiegeViewHost, col: u32, row: u32, heading: &mut u8, ok: &mut bool) {
    if step_hits_gate(m, h, col, row, *heading) {
        return;
    }
    let up = heading.wrapping_add(0x40);
    if step_hits_gate(m, h, col, row, up) {
        *heading = up;
        return;
    }
    let down = heading.wrapping_sub(0x40);
    if step_hits_gate(m, h, col, row, down) {
        *heading = down;
        return;
    }
    *ok = false;
}

/// `town_siege_spawn_defender_militia` (`0x006311E0`): milicia de la ciudad (cupos `+0x999..+0x99B` menos las
/// vivas `+0x1006..+0x100A`) en dos pasadas desde las dos casillas de la puerta (`+0x5A2/+0x5A4` y `+0x5A6/+0x5A8`)
/// siguiendo el paso por la puerta; la clase (9, 0xA o 0xB) sale de `rand` sobre lo que queda de cada tipo, y cada
/// figura va a su lista (`+0x410`, `+0x80C`, `+0xC08`), suma su cuenta y el contador de la pasada `+0x100E/F`.
pub fn spawn_defender_militia(m: &mut dyn Mem, h: &mut dyn SiegeViewHost, s: u32) {
    if m.r16(s + 8) == 0xFFFF {
        return;
    }
    let rec = siege_record(m, s);
    let t = town(m, s);
    let total = (i32::from(m.r8(t + 0x99B)) - i32::from(m.r16(rec + 0x100A)) - i32::from(m.r16(rec + 0x1008)) - i32::from(m.r16(rec + 0x1006)))
        .wrapping_add(i32::from(m.r8(t + 0x99A)))
        .wrapping_add(i32::from(m.r8(t + 0x999)));
    if total <= 0 {
        return;
    }
    let (mut col, mut row) = (u32::from(m.r16(s + 0x5A2)), u32::from(m.r16(s + 0x5A4)));
    let gate_side = |m: &mut dyn Mem| {
        let t = town(m, s);
        let slot = u32::from(m.r8(s + 0x6D));
        let gate = m.r32(s + 0x244 + 4 * slot);
        let r = building_record(m, t, gate);
        m.r8(r + 3)
    };
    let mut heading = if gate_side(m) == 0x38 { HEADING[7] } else { HEADING[1] };
    let mut remaining = total;
    for pass in 0..2u32 {
        let mut ok = true;
        for _ in 0..3 {
            militia_heading(m, h, col, row, &mut heading, &mut ok);
            h.step(m, &mut col, &mut row, heading);
        }
        let half = remaining >> 1;
        let mut i: u32 = 0;
        while ((i & 0xFFFF) as i32) < half {
            if ok {
                let r = h.rand(m) as i32;
                let base = total.wrapping_sub((total >> 1).wrapping_mul(pass as i32)).wrapping_sub(i as i32);
                let rem = r.wrapping_rem(base);
                let rec = siege_record(m, s);
                let t = town(m, s);
                let c9 = m.r8(t + 0x999);
                let n9 = m.r16(rec + 0x1006);
                let kind: u32 = if ((rem as u32 & 0xFFFF) as i32) < i32::from(c9) - i32::from(n9) {
                    1
                } else {
                    let ta = i32::from(m.r8(t + 0x99A)) - i32::from(m.r16(rec + 0x1008));
                    let cx = n9.wrapping_sub(u16::from(c9)).wrapping_add(rem as u16);
                    if i32::from(cx) >= ta {
                        3
                    } else {
                        2
                    }
                };
                let (px, py) = h.cell_to_pixel(m, col, row);
                let city = u32::from(m.r8(s + 0x5D3));
                let (class, list) = match kind {
                    1 => (9, 0x410),
                    2 => (0xA, 0x80C),
                    _ => (0xB, 0xC08),
                };
                let entity = h.spawn_entity(m, [(px as u32) << 16, (py as u32) << 16, class, city + 0x10080, u32::from(HEADING[1]), 0, 0]);
                let rec = siege_record(m, s);
                let n = m.r16(rec + 0x1004 + 2 * kind);
                m.w32(record_list(rec, list, n), entity);
                m.w16(rec + 0x1004 + 2 * kind, n.wrapping_add(1));
                let c = m.r8(rec + 0x100E + pass);
                m.w8(rec + 0x100E + pass, c.wrapping_add(1));
                militia_heading(m, h, col, row, &mut heading, &mut ok);
                h.step(m, &mut col, &mut row, heading);
            }
            i = i.wrapping_add(1);
        }
        col = u32::from(m.r16(s + 0x5A6));
        row = u32::from(m.r16(s + 0x5A8));
        heading = if gate_side(m) == 0x38 { HEADING[3] } else { HEADING[5] };
        remaining = remaining.wrapping_add(1);
    }
}

/// Espiral de casillas compartida por las funciones que crean figuras alrededor de una casilla: `ring` vueltas,
/// `steps` pasos dados en el lado actual y su rumbo (0x00, 0x40, 0x80, 0xC0; al cerrar la vuelta un paso 0xA0).
struct Spiral {
    x: u32,
    y: u32,
    heading: u8,
    ring: u8,
    steps: u8,
}

impl Spiral {
    fn advance(&mut self, m: &mut dyn Mem, h: &mut dyn SiegeViewHost) {
        if self.steps < self.ring {
            h.step(m, &mut self.x, &mut self.y, self.heading);
            self.steps = self.steps.wrapping_add(1);
        } else if self.heading < 0xC0 && self.ring > 0 {
            self.heading = self.heading.wrapping_add(0x40);
            h.step(m, &mut self.x, &mut self.y, self.heading);
            self.steps = 1;
        } else {
            self.heading = 0xC0;
        }
        if self.heading == 0xC0 && self.steps == self.ring {
            h.step(m, &mut self.x, &mut self.y, HEADING[5]);
            self.ring = self.ring.wrapping_add(1);
            self.heading = 0;
            self.steps = 0;
        }
    }
}

/// Casilla apta para una figura del asalto: dentro del mapa, con campo, libre y colocable.
fn attacker_cell(m: &mut dyn Mem, h: &mut dyn SiegeViewHost, s: u32, x: u32, y: u32, near: bool) -> bool {
    if x >= width(m) || y >= height(m) {
        return false;
    }
    let cell = flow_cell(m, s, x, y);
    if m.r16(cell) == 0xFFFF {
        return false;
    }
    cell_walkable(m, h, x, y, 0, 0xFFFF) && can_place_unit(m, h, s, x, y, near, 0)
}

/// `town_siege_spawn_attackers_around_cell` (`0x0062D170`): figuras del asalto en espiral alrededor de `(x, y)`.
///
/// Sin espera ni ataque en curso (`+0x42 == 0`, `+0x78 == 0`) y con puerta, primero los dos portadores de las
/// casillas exentas (clase 0x20, `+0x5CA..+0x5D0`, rumbo `+0x5D2`) en `+0x210`. Luego, en cada casilla de la
/// espiral, si `units` lo pide: asaltantes (clase 8, `+0x7C`, cuántos `+0x18` hasta `+0x21E`; código `+0x20000` si
/// el asedio está en espera reciente) y tres tipos más (clases 9, 0xA y 0xB en `+0x11C`, `+0x174`, `+0x1BC`, cuántos
/// `+0x1A..+0x1E` hasta `+0x222..+0x22A`); siempre los portadores que falten (clase 0x20) y, si `+0x61C`, el ariete
/// (clase 0x23, `+0x218`). Cada tipo activo da su paso de espiral. Para tras 100 vueltas o sin nada que crear. Sin
/// espera (`+0x3A == 0`) crea al final cuatro guardias (clase 8, `+0x608`) en `+0x46/+0x4E` o `+0x56/+0x5E`.
pub fn spawn_attackers_around_cell(m: &mut dyn Mem, h: &mut dyn SiegeViewHost, s: u32, x: u32, y: u32, units: bool) {
    let mut carriers: u8 = 0;
    let recent = if m.r16(s + 0x3A) != 0 || m.r16(s + 0x3C) != 0 {
        let rec = siege_record(m, s);
        m.r32(rec).wrapping_add(0x1B80) > m.r32(TICK)
    } else {
        false
    };
    if m.r16(s + 0x42) == 0 && m.r8(s + 0x78) == 0 && m.r16(s + 8) != 0xFFFF {
        for k in 0..2u32 {
            let col = u32::from(m.r16(s + 0x5CA + 2 * k));
            let row = u32::from(m.r16(s + 0x5CE + 2 * k));
            let (px, py) = h.cell_to_pixel(m, col, row);
            let city = u32::from(m.r8(s + 0x5D3));
            let heading = u32::from(m.r8(s + 0x5D2));
            let entity = h.spawn_entity(m, [(px as u32) << 16, (py as u32) << 16, 0x20, city + 0x100 + 0x80 * k, heading, 0, 0]);
            m.w32(s + 0x210 + 4 * k, entity);
        }
        carriers = 2;
    }
    let mut sp = Spiral { x, y, heading: 0, ring: 0, steps: 0 };
    let spawn_at = |m: &mut dyn Mem, h: &mut dyn SiegeViewHost, sp: &Spiral, class: u32, id: u32| {
        let (px, py) = h.cell_to_pixel(m, sp.x, sp.y);
        h.spawn_entity(m, [(px as u32) << 16, (py as u32) << 16, class, id, u32::from(HEADING[1]), 0, 0])
    };
    loop {
        let mut more = false;
        // Asaltantes y los otros tres tipos: (cuántos, cupo, lista, clase).
        for (k, (count, cap, list, class)) in [(0x18u32, 0x21Eu32, 0x7Cu32, 8u32), (0x1A, 0x222, 0x11C, 9), (0x1C, 0x226, 0x174, 0xA), (0x1E, 0x22A, 0x1BC, 0xB)].into_iter().enumerate() {
            if !units || m.r16(s + count) >= m.r16(s + cap) {
                continue;
            }
            more = true;
            if attacker_cell(m, h, s, sp.x, sp.y, k == 0) {
                let city = u32::from(m.r8(s + 0x5D3));
                let id = if k == 0 && recent { city + 0x20000 } else { city };
                let entity = spawn_at(m, h, &sp, class, id);
                let n = m.r16(s + count);
                m.w32(s + list + 4 * u32::from(n), entity);
                m.w16(s + count, n.wrapping_add(1));
            }
            sp.advance(m, h);
        }
        if carriers < 2 && m.r16(s + 8) != 0xFFFF {
            more = true;
            if attacker_cell(m, h, s, sp.x, sp.y, false) {
                let city = u32::from(m.r8(s + 0x5D3));
                let entity = spawn_at(m, h, &sp, 0x20, city + (u32::from(carriers) << 7));
                m.w32(s + 0x210 + 4 * u32::from(carriers), entity);
                carriers = carriers.wrapping_add(1);
            }
            sp.advance(m, h);
        }
        if m.r8(s + 0x61C) != 0 && m.r32(s + 0x218) == 0 {
            more = true;
            if attacker_cell(m, h, s, sp.x, sp.y, false) {
                let city = u32::from(m.r8(s + 0x5D3));
                let entity = spawn_at(m, h, &sp, 0x23, city);
                m.w32(s + 0x218, entity);
            }
            sp.advance(m, h);
        }
        if sp.ring > 100 || !more {
            break;
        }
    }
    if m.r16(s + 0x3A) != 0 {
        return;
    }
    for k in 0..4u32 {
        let (col, row) = if m.r16(s + 0x42) > m.r16(s + 0x44) {
            (u32::from(m.r16(s + 0x46 + 2 * k)), u32::from(m.r16(s + 0x4E + 2 * k)))
        } else {
            (u32::from(m.r16(s + 0x56 + 2 * k)), u32::from(m.r16(s + 0x5E + 2 * k)))
        };
        let (px, py) = h.cell_to_pixel(m, col, row);
        let city = u32::from(m.r8(s + 0x5D3));
        let entity = h.spawn_entity(m, [(px as u32) << 16, (py as u32) << 16, 8, city + 0x30000 + 0x10000 * k, u32::from(HEADING[1]), 0, 0]);
        m.w32(s + 0x608 + 4 * k, entity);
    }
}

/// `town_walker_propagate_queue_rank` (`0x005614E0`): rango `+0x7A` de la figura y de las siguientes de su cola
/// (`+0x5C`), cada una uno más, hasta una que ya lo tenga o lo supere.
pub fn propagate_queue_rank(m: &mut dyn Mem, mut obj: u32, mut v: u8) {
    if m.r8(obj + 0x7A) == v {
        return;
    }
    loop {
        m.w8(obj + 0x7A, v);
        obj = m.r32(obj + 0x5C);
        if obj == 0 || m.r8(obj + 0x7A) > v {
            return;
        }
        v = v.wrapping_add(1);
        if m.r8(obj + 0x7A) == v {
            return;
        }
    }
}

/// Pasos máximos del recorrido de [`queue_walker_at_entry`].
pub const QUEUE_WALK_STEPS: u32 = 1 << 16;

/// `town_siege_queue_walker_at_entry` (`0x00630DC0`): la figura `entity` recorre las piezas de puerta desde
/// `(col, row)` hasta una de las dos casillas de la puerta y entra en la cola de esa puerta (`registro+0xC/+0x10`,
/// ordenada por `+0x5A`, enlazada por `+0x5C`), con su rumbo de salida en `+0x79` y la máscara `+0x70` marcada en la
/// casilla de partida. `resume` (`arg4 != 0`) parte del rumbo guardado `+0x79`; si no, del primero (desde 0xE0) que
/// da con la puerta. Si no llega ni dando la vuelta, la vista retira la figura. `exit` es el rumbo de salida de una
/// llamada anterior (byte de pila sin iniciar en el original cuando la figura empieza ya en la puerta).
pub fn queue_walker_at_entry(m: &mut dyn Mem, h: &mut dyn SiegeViewHost, s: u32, entity: u32, col: u8, row: u8, resume: bool, exit: &mut u8) {
    let (c0, r0) = (u32::from(col), u32::from(row));
    let first = if resume {
        m.r8(entity + 0x79)
    } else {
        let mut d = HEADING[7];
        // Sin pieza de puerta alrededor el original no termina; se limita a una vuelta completa por rumbos.
        let mut tries = 0;
        while !step_hits_gate(m, h, c0, r0, d) && tries < 0x100 {
            d = d.wrapping_add(0x40);
            tries += 1;
        }
        d
    };
    let (mut x, mut y) = (c0, r0);
    let mut bl = first;
    let mut dir = bl;
    let mut count: u8 = 0;
    let mut retried = false;
    let mut walked: u32 = 0;
    loop {
        // Un recorrido de puerta que no llega a la puerta tampoco termina en el original; se corta como si diera
        // la vuelta.
        walked += 1;
        if walked > QUEUE_WALK_STEPS {
            h.release_entity(m, entity);
            return;
        }
        let at_a = x == u32::from(m.r16(s + 0x5A2)) && y == u32::from(m.r16(s + 0x5A4));
        let at_b = x == u32::from(m.r16(s + 0x5A6)) && y == u32::from(m.r16(s + 0x5A8));
        if at_a || at_b {
            break;
        }
        let mut go = step_hits_gate(m, h, x, y, dir);
        if !go {
            let d1 = bl.wrapping_sub(0x40);
            if step_hits_gate(m, h, x, y, d1) {
                bl = d1;
                dir = bl;
                go = true;
            } else {
                bl = bl.wrapping_add(0x40);
                if step_hits_gate(m, h, x, y, bl) {
                    dir = bl;
                    go = true;
                }
            }
        }
        if go {
            h.step(m, &mut x, &mut y, dir);
            count = count.wrapping_add(1);
            if count == 1 {
                *exit = bl.wrapping_add(0x80);
            }
            continue;
        }
        bl = first.wrapping_add(0x80);
        x = c0;
        y = r0;
        count = 0;
        if retried {
            h.release_entity(m, entity);
            return;
        }
        dir = bl;
        retried = true;
    }
    let gate: u32 = if x == u32::from(m.r16(s + 0x5A2)) && y == u32::from(m.r16(s + 0x5A4)) { 0 } else { 1 };
    let rec = siege_record(m, s);
    let n = m.r8(rec + 0x100E + gate);
    m.w8(rec + 0x100E + gate, n.wrapping_add(1));
    let mut prev = 0;
    let mut e = m.r32(rec + 0xC + 4 * gate);
    while e != 0 && m.r8(e + 0x5A) < count {
        prev = e;
        e = m.r32(e + 0x5C);
    }
    m.w32(entity + 0x5C, e);
    if prev == 0 {
        let rec = siege_record(m, s);
        m.w32(rec + 0xC + 4 * gate, entity);
    } else {
        m.w32(prev + 0x5C, entity);
    }
    m.w8(entity + 0x79, *exit);
    if m.r8(entity + 0x70) != 0 {
        let rank = m.r8(entity + 0x5A);
        let back = m.r8(entity + 0x3C) == *exit;
        m.w8(entity + 0x5A, if back { rank.wrapping_sub(1) } else { rank.wrapping_add(1) });
    } else {
        m.w8(entity + 0x5A, count);
    }
    propagate_queue_rank(m, entity, 3);
    let wait = m.r16(s + 0x3A);
    let visible = if wait != 0 { wait > m.r16(s + 0x6A) } else { m.r16(s + 0x21E) > 0 };
    let mask = m.r32(s + 0x70);
    if visible && mask != 0 && c0 < width(m) && r0 < height(m) {
        let at = mask.wrapping_add(width(m).wrapping_mul(r0).wrapping_add(c0));
        let b = m.r8(at);
        m.w8(at, b | 1);
    }
}

/// `town_siege_requeue_gate_units` (`0x00631120`): recalcula las casillas de la puerta y vuelve a encolar en ellas
/// las figuras de las dos colas (vaciadas antes, con sus contadores `+0x100E/+0x100F` a cero), desde su casilla
/// `+0x6C/+0x6E` y su rumbo guardado. `exit` como en [`queue_walker_at_entry`].
pub fn requeue_gate_units(m: &mut dyn Mem, h: &mut dyn SiegeViewHost, s: u32, exit: &mut u8) {
    if m.r16(s + 8) == 0xFFFF {
        return;
    }
    crate::economy::siege_view::compute_gate_cells(m, s);
    let mut heads = [0u32; 2];
    for (k, head) in heads.iter_mut().enumerate() {
        let rec = siege_record(m, s);
        let at = rec + 0xC + 4 * k as u32;
        *head = m.r32(at);
        m.w32(at, 0);
        let rec = siege_record(m, s);
        m.w8(rec + 0x100E + k as u32, 0);
    }
    for head in heads {
        let mut e = head;
        while e != 0 {
            let (col, row, next) = (m.r8(e + 0x6C), m.r8(e + 0x6E), m.r32(e + 0x5C));
            queue_walker_at_entry(m, h, s, e, col, row, true, exit);
            e = next;
        }
    }
}

/// `a + (b - a) * f / 0x10000` con división truncada hacia cero (el `cdq`/`and 0xFFFF`/`sar 16` del original).
fn lerp(a: u16, b: u16, f: i32) -> u32 {
    let d = (i32::from(b) - i32::from(a)).wrapping_mul(f);
    let q = d.wrapping_add(if d < 0 { 0xFFFF } else { 0 }) >> 16;
    q.wrapping_add(i32::from(a)) as u32
}

/// División entera con signo del original; por cero el original fallaría (se toma 0).
fn idiv(a: i32, b: i32) -> i32 {
    if b == 0 {
        0
    } else {
        a.wrapping_div(b)
    }
}

/// Siembra extra del campo en `(x, y)` si está en el mapa, conservando el objetivo `+0x10/+0x12`.
fn seed_extra(m: &mut dyn Mem, h: &mut dyn SiegeViewHost, s: u32, x: u16, y: u16) {
    if u32::from(x) >= width(m) || u32::from(y) >= height(m) {
        return;
    }
    let (ox, oy) = (m.r16(s + 0x10), m.r16(s + 0x12));
    m.w16(s + 0x10, x);
    m.w16(s + 0x12, y);
    crate::economy::siege_view::seed_flow_field(m, h, s, false);
    m.w16(s + 0x10, ox);
    m.w16(s + 0x12, oy);
}

/// Duración del asalto (`+0x58C`, y `+0x30` del jefe `+0x574`): `n` por el factor global `0x006DFBC4`.
fn set_assault_time(m: &mut dyn Mem, s: u32, n: u32) {
    let v = n.wrapping_mul(m.r32(0x006D_FBC4));
    m.w32(s + 0x58C, v);
    let leader = m.r32(s + 0x574);
    if leader != 0 {
        m.w32(leader + 0x30, v);
    }
}

/// `town_siege_rebuild_view_state` (`0x0062C730`) completo: el prefijo de
/// [`crate::economy::siege_view::rebuild_view_state_prefix`] y, con el asedio a la vista, el campo de rutas hacia el
/// destino de la fase (retirada `+0x42`, rendición `+0x3A`, ataque `+0x3C` o asedio en curso), los marcadores de los
/// puntos de entrada, defensores y asaltantes en la casilla interpolada según el avance de la fase; luego marcadores
/// de puerta y milicia si sigue visible, y el reencolado de las figuras de la puerta (`exit` como en
/// [`queue_walker_at_entry`]).
pub fn rebuild_view_state(m: &mut dyn Mem, h: &mut dyn SiegeViewHost, s: u32, exit: &mut u8) {
    use crate::economy::siege_view::{rebuild_view_state_prefix, reset_flow_field_to_cell as reset, update_entry_point_markers as markers};
    if !rebuild_view_state_prefix(m, h, s) {
        return;
    }
    let r = |m: &mut dyn Mem, at: u32| m.r16(s + at);
    let retreat = r(m, 0x42);
    if retreat != 0 {
        m.w16(s + 0x3C, 0);
        let n = u32::from(r(m, 0x66)).wrapping_sub(u32::from(retreat));
        set_assault_time(m, s, n);
        let mut f: i32;
        let show;
        if retreat > r(m, 0x44) {
            let (a, b) = (i32::from(r(m, 0x44)), i32::from(r(m, 0x66)));
            f = idiv((a - i32::from(retreat)) << 17, b - a).wrapping_add(0x20000);
            let (a0, a1, a2) = (u32::from(r(m, 0x14)), u32::from(r(m, 0x16)), 0);
            reset(m, h, s, a0, a1, a2);
            let (x, y) = (r(m, 0x14), r(m, 0x16));
            seed_extra(m, h, s, x.wrapping_add(1), y);
            let (x, y) = (r(m, 0x14), r(m, 0x16));
            seed_extra(m, h, s, x.wrapping_sub(1), y);
            let (x, y) = (r(m, 0xC), r(m, 0xE));
            seed_extra(m, h, s, x, y);
            if f >= 0x10000 {
                f = 0x10000;
                show = true;
            } else {
                show = false;
            }
        } else if m.r8(s + 0x6F) != 0xFF {
            let slot = u32::from(m.r8(s + 0x6F));
            let d = i32::from(m.r16(s + 0x254 + 8 * slot));
            f = idiv((d - i32::from(retreat)) << 16, i32::from(r(m, 0x66)) - d).wrapping_add(0x10000);
            let t = town(m, s);
            let id = m.r32(s + 0x258 + 8 * slot);
            let a = building_record(m, t, id);
            let b = building_record(m, t, id);
            let (bx, by) = (u32::from(m.r8(a)), u32::from(m.r8(b + 1)));
            reset(m, h, s, bx, by, 1);
            let (x, y) = (r(m, 0xC), r(m, 0xE));
            seed_extra(m, h, s, x, y);
            show = true;
        } else {
            f = 0x10000 - idiv(i32::from(retreat) << 16, i32::from(r(m, 0x66)));
            if f > 0xF000 {
                f = 0xF000;
            }
            let (a0, a1, a2) = (u32::from(r(m, 0x5CA)), u32::from(r(m, 0x5CE)), 0);
            reset(m, h, s, a0, a1, a2);
            let (x, y) = (r(m, 0x5CC), r(m, 0x5D0));
            seed_extra(m, h, s, x, y);
            let (x, y) = (r(m, 0xC), r(m, 0xE));
            seed_extra(m, h, s, x, y);
            show = true;
        }
        markers(m, h, s, show);
        let (a0, a1) = (u32::from(r(m, 8)), u32::from(r(m, 0xA)));
        spawn_defenders_around_cell(m, h, s, a0, a1);
        let x = lerp(r(m, 0x36), r(m, 0x10), f);
        let y = lerp(r(m, 0x38), r(m, 0x12), f);
        spawn_attackers_around_cell(m, h, s, x, y, true);
    } else if r(m, 0x3A) != 0 {
        let wait = r(m, 0x3A);
        let span = r(m, 0x6A);
        if wait >= span {
            let f = idiv((i32::from(span) - i32::from(wait)) << 16, i32::from(span)).wrapping_add(0x10000);
            let (a0, a1, a2) = (u32::from(r(m, 0x36)), u32::from(r(m, 0x38)), 0);
            reset(m, h, s, a0, a1, a2);
            markers(m, h, s, true);
            if r(m, 8) != 0xFFFF {
                let x = lerp(r(m, 8), r(m, 0x36), f);
                let y = lerp(r(m, 0xA), r(m, 0x38), f);
                spawn_attackers_around_cell(m, h, s, x, y, true);
                let x = lerp(r(m, 0xC), r(m, 0x36), f);
                let y = lerp(r(m, 0xE), r(m, 0x38), f);
                spawn_defenders_around_cell(m, h, s, x, y);
            }
        } else {
            let f = idiv(i32::from(wait) << 16, i32::from(span));
            let (a0, a1, a2) = (u32::from(r(m, 0x3E)), u32::from(r(m, 0x40)), 0);
            reset(m, h, s, a0, a1, a2);
            let x = lerp(r(m, 0x3E), r(m, 0x36), f);
            let y = lerp(r(m, 0x40), r(m, 0x38), f);
            spawn_defenders_around_cell(m, h, s, x, y);
        }
    } else if r(m, 0x3C) != 0 {
        let attack = r(m, 0x3C);
        let span = r(m, 0x68);
        let half = span >> 1;
        let mut direct = false;
        let f;
        if attack >= half {
            if i32::from(attack) - 0x100 <= i32::from(half) {
                let (a0, a1, a2) = (u32::from(r(m, 0x3E)), u32::from(r(m, 0x40)), 0);
                reset(m, h, s, a0, a1, a2);
                markers(m, h, s, true);
                let (a0, a1, a2) = (u32::from(r(m, 0x3E)), u32::from(r(m, 0x40)), false);
                spawn_attackers_around_cell(m, h, s, a0, a1, a2);
                direct = true;
                f = 0;
            } else {
                f = idiv((i32::from(span) - i32::from(attack) - 0x100) << 16, i32::from(half));
                let (a0, a1, a2) = (u32::from(r(m, 0x3E)), u32::from(r(m, 0x40)), 0);
                reset(m, h, s, a0, a1, a2);
            }
        } else {
            f = idiv(i32::from(attack) << 16, i32::from(half));
            let (a0, a1, a2) = (u32::from(r(m, 0x36)), u32::from(r(m, 0x38)), 0);
            reset(m, h, s, a0, a1, a2);
        }
        if !direct {
            markers(m, h, s, true);
            let (bx, by) = if r(m, 8) == 0xFFFF { (r(m, 0x36), r(m, 0x38)) } else { (r(m, 8), r(m, 0xA)) };
            let x = lerp(bx, r(m, 0x3E), f);
            let y = lerp(by, r(m, 0x40), f);
            spawn_attackers_around_cell(m, h, s, x, y, true);
        }
        let n = u32::from(r(m, 0x3C));
        set_assault_time(m, s, n);
    } else {
        let v = u32::from(r(m, 0x66)).wrapping_mul(m.r32(0x006D_FBC4)).wrapping_mul(2);
        m.w32(s + 0x58C, v);
        let leader = m.r32(s + 0x574);
        if leader != 0 {
            m.w32(leader + 0x30, v);
        }
        if m.r8(s + 0x78) != 0 {
            let (a0, a1, a2) = (u32::from(r(m, 8)), u32::from(r(m, 0xA)), 0);
            reset(m, h, s, a0, a1, a2);
        } else {
            let (a0, a1, a2) = (u32::from(r(m, 0x5CA)), u32::from(r(m, 0x5CE)), 0);
            reset(m, h, s, a0, a1, a2);
            let (x, y) = (r(m, 0x5CC), r(m, 0x5D0));
            seed_extra(m, h, s, x, y);
            let (x, y) = (r(m, 0xC), r(m, 0xE));
            seed_extra(m, h, s, x, y);
        }
        markers(m, h, s, true);
        let (a0, a1) = (u32::from(r(m, 8)), u32::from(r(m, 0xA)));
        spawn_defenders_around_cell(m, h, s, a0, a1);
        let x = lerp(r(m, 0x36), r(m, 8), 0xF000);
        let y = lerp(r(m, 0x38), r(m, 0xA), 0xF000);
        spawn_attackers_around_cell(m, h, s, x, y, true);
    }
    let wait = r(m, 0x3A);
    let visible = if wait != 0 { wait > r(m, 0x6A) } else { r(m, 0x21E) > 0 };
    if visible {
        spawn_gate_markers(m, h, s);
        spawn_defender_militia(m, h, s);
    }
    requeue_gate_units(m, h, s, exit);
}

/// Tope de pasos al recorrer una cola de la puerta (el original no tiene tope y no termina con una cola cíclica).
pub const GATE_QUEUE_STEPS: u32 = 1 << 16;

/// Quita la entidad de las dos colas de la puerta del registro (`+0xC`/`+0x10`, enlace `+0x5C`, cuántas en
/// `+0x100E`/`+0x100F`): común a `0x00634D80` y `0x0062DFB0` cuando la entidad está encolada (`+0x59`).
fn gate_queue_unlink(m: &mut dyn Mem, rec: u32, e: u32) {
    for i in 0..2u32 {
        if m.r32(rec + 0xC + 4 * i) == e {
            let next = m.r32(e + 0x5C);
            m.w32(rec + 0xC + 4 * i, next);
            let n = m.r8(rec + 0x100E + i);
            m.w8(rec + 0x100E + i, n.wrapping_sub(1));
            return;
        }
    }
    for i in 0..2u32 {
        let mut c = m.r32(rec + 0xC + 4 * i);
        let mut steps = 0;
        while c != 0 && steps < GATE_QUEUE_STEPS {
            steps += 1;
            let next = m.r32(c + 0x5C);
            if next == 0 {
                break;
            }
            if next == e {
                let after = m.r32(e + 0x5C);
                m.w32(c + 0x5C, after);
                let n = m.r8(rec + 0x100E + i);
                m.w8(rec + 0x100E + i, n.wrapping_sub(1));
                return;
            }
            c = next;
        }
    }
}

/// Busca `e` en la lista `list` de `count` entradas (índice de un byte, como el original) y la quita cambiándola
/// por la última. Devuelve si estaba.
fn swap_remove(m: &mut dyn Mem, list: u32, count_at: u32, e: u32) -> Option<u8> {
    let n = m.r16(count_at);
    for k in 0..n {
        let i = k as u8;
        if m.r32(list + 4 * u32::from(i)) == e {
            let last = n - 1;
            m.w16(count_at, last);
            let at = m.r16(count_at);
            let v = m.r32(list + 4 * u32::from(at));
            m.w32(list + 4 * u32::from(i), v);
            let at = m.r16(count_at);
            m.w32(list + 4 * u32::from(at), 0);
            return Some(i);
        }
    }
    None
}

fn dec16(m: &mut dyn Mem, at: u32) {
    let v = m.r16(at);
    m.w16(at, v.wrapping_sub(1));
}

/// Listas por tipo de figura 0..3 (clases 8, 9, 0xA y 0xB): lista y cuenta de la escena, contador de la escena que
/// baja si la figura cuenta (`+0x58`), lista y cuenta del registro y contador de la escena para las del registro.
const UNIT_LISTS: [(u32, u32, u32, u32, u32, u32); 4] = [
    (0x7C, 0x18, 0x22, 0x14, 0x1004, 0x2C),
    (0x11C, 0x1A, 0x24, 0x410, 0x1006, 0x2E),
    (0x174, 0x1C, 0x26, 0x80C, 0x1008, 0x30),
    (0x1BC, 0x1E, 0x28, 0xC08, 0x100A, 0x32),
];

/// `town_siege_unregister_entity` (`0x0062DFB0`): la escena olvida una figura destruida de tipo `kind` (0..3
/// defensores y milicia, 4 cabecillas `+0x1F8`, 5 jefe `+0x574`, 6 porteadores `+0x210/+0x214`, 7 ariete `+0x218`).
/// Las de tipo 0..3 salen de las colas de la puerta si están encoladas, y de la lista de la escena o, si no está, de
/// la del registro; un guardia (`+0x88`) solo deja libre su puesto `+0x608 + 4 * [+0x89]`.
pub fn unregister_entity(m: &mut dyn Mem, s: u32, e: u32, kind: u8) {
    if kind < 4 && m.r8(e + 0x59) != 0 {
        let rec = siege_record(m, s);
        gate_queue_unlink(m, rec, e);
    }
    match kind {
        0..=3 => {
            if kind == 0 && m.r8(e + 0x88) != 0 {
                let slot = u32::from(m.r8(e + 0x89));
                m.w32(s + 0x608 + 4 * slot, 0);
                return;
            }
            let (list, count, dec, rlist, rcount, rdec) = UNIT_LISTS[usize::from(kind)];
            if swap_remove(m, s + list, s + count, e).is_some() {
                if m.r8(e + 0x58) != 0 {
                    dec16(m, s + dec);
                }
                return;
            }
            let rec = siege_record(m, s);
            if swap_remove(m, rec + rlist, rec + rcount, e).is_some() && m.r8(e + 0x58) != 0 {
                dec16(m, s + rdec);
            }
        }
        4 => {
            let n = m.r16(s + 0x20);
            for k in 0..n {
                let i = k as u8;
                if m.r32(s + 0x1F8 + 4 * u32::from(i)) == e {
                    let last = n - 1;
                    m.w16(s + 0x20, last);
                    if last == u16::from(m.r8(s + 0x578)) {
                        m.w8(s + 0x578, i);
                    }
                    let v = m.r32(s + 0x1F8 + 4 * u32::from(last));
                    m.w32(s + 0x1F8 + 4 * u32::from(i), v);
                    m.w32(s + 0x1F8 + 4 * u32::from(last), 0);
                    if m.r8(e + 0x2C) != 0 {
                        dec16(m, s + 0x2A);
                    }
                    return;
                }
            }
        }
        5 => m.w32(s + 0x574, 0),
        6 => {
            if m.r32(s + 0x210) == e {
                m.w32(s + 0x210, 0);
            } else {
                m.w32(s + 0x214, 0);
            }
        }
        7 => m.w32(s + 0x218, 0),
        _ => {}
    }
}

/// `siege_record_remove_unit` (`0x00634D80`): con la escena abierta (`registro+8`) pasa a
/// [`unregister_entity`]; si no, quita la figura de las colas de la puerta y de la lista del registro de su tipo.
pub fn siege_record_remove_unit(m: &mut dyn Mem, rec: u32, e: u32, kind: u8) {
    let s = m.r32(rec + 8);
    if s != 0 {
        unregister_entity(m, s, e, kind);
        return;
    }
    if m.r8(e + 0x59) != 0 {
        gate_queue_unlink(m, rec, e);
    }
    if kind < 4 {
        let (_, _, _, rlist, rcount, _) = UNIT_LISTS[usize::from(kind)];
        swap_remove(m, rec + rlist, rec + rcount, e);
    }
}

/// Tipo de figura de asedio de una clase de entidad de la vista (tabla de `0x00593D50`): 8..0xB defensores y milicia,
/// 0x15 cabecilla, 0x16 jefe, 0x20 porteador, 0x23 ariete, 0x24 (tipo 8, sin lista).
pub fn siege_unit_kind(class: u8) -> Option<u8> {
    Some(match class {
        8..=0xB => class - 8,
        0x15 => 4,
        0x16 => 5,
        0x20 => 6,
        0x23 => 7,
        0x24 => 8,
        _ => return None,
    })
}

/// Parte de asedio del destructor de entidades de la vista (`0x00593D50`): con el tipo de la clase, quita la figura
/// del registro de la ciudad de la vista (`[vista+0xBF2C]`).
pub fn remove_destroyed_entity(m: &mut dyn Mem, e: u32, class: u8) {
    if let Some(kind) = siege_unit_kind(class) {
        let v = view(m);
        let city = m.r32(v + 0xBF2C);
        let rec = m.r32(SIEGE_RECORDS).wrapping_add(city.wrapping_mul(SIEGE_RECORD_SIZE));
        siege_record_remove_unit(m, rec, e, kind);
    }
}

/// `siege_record_forget_unit_entities` (`0x006358A0`): vacía colas y listas del registro sin destruir las figuras
/// (porte de [`crate::economy::siege_record`]).
pub use crate::economy::siege_record::forget_unit_entities;

/// Tope de retiradas por lista en el cierre: el original repite mientras la lista no se vacíe, y eso depende de que
/// el destructor de la entidad la quite ([`remove_destroyed_entity`]).
pub const RELEASE_STEPS: u32 = 1 << 12;

/// `town_siege_release_view` (`0x0062F5A0`): cierre de la escena de la batalla. Si la vista sigue en la ciudad y
/// `destroy`, retira (método `+4` de `vista+0x3F8`) guardias, porteadores, ariete, figuras de las cuatro listas,
/// cabecillas (y su `+0x74`), jefe (y su `+0x64`), figuras del registro, marcadores de entrada y de puerta,
/// transeúntes de la vista si había espera o milicia, y el efecto `+0x628`; si no, solo olvida los punteros. Después
/// libera cola y campo, borra cuentas y el bloque `+0x70`. Sin la escena viva (`+0x7A`) solo olvida el registro.
pub fn release_view(m: &mut dyn Mem, h: &mut dyn SiegeViewHost, s: u32, destroy: bool) {
    let rec = siege_record(m, s);
    if m.r8(s + 0x7A) == 0 {
        forget_unit_entities(m, rec);
        return;
    }
    m.w8(s + 0x7A, 0);
    let v = view(m);
    let city = u32::from(m.r8(s + 0x5D3));
    if m.r32(v + 0xC324) == city && destroy {
        let release = |m: &mut dyn Mem, h: &mut dyn SiegeViewHost, at: u32| {
            let e = m.r32(at);
            if e != 0 {
                h.release_entity(m, e);
            }
        };
        for k in 0..4 {
            release(m, h, s + 0x608 + 4 * k);
        }
        release(m, h, s + 0x210);
        release(m, h, s + 0x214);
        release(m, h, s + 0x218);
        for (list, count, ..) in UNIT_LISTS {
            let mut n = 0;
            while m.r16(s + count) != 0 && n < RELEASE_STEPS {
                n += 1;
                let e = m.r32(s + list);
                h.release_entity(m, e);
            }
        }
        let mut n = 0;
        while m.r16(s + 0x20) != 0 && n < RELEASE_STEPS {
            n += 1;
            let first = m.r32(s + 0x1F8);
            let extra = m.r32(first + 0x74);
            if extra != 0 {
                h.release_entity(m, extra);
                let first = m.r32(s + 0x1F8);
                m.w32(first + 0x74, 0);
            }
            let first = m.r32(s + 0x1F8);
            h.release_entity(m, first);
        }
        let chief = m.r32(s + 0x574);
        if chief != 0 {
            let extra = m.r32(chief + 0x64);
            if extra != 0 {
                h.release_entity(m, extra);
                let chief = m.r32(s + 0x574);
                m.w32(chief + 0x64, 0);
            }
            let chief = m.r32(s + 0x574);
            h.release_entity(m, chief);
        }
        for (_, _, _, list, count, _) in UNIT_LISTS {
            let mut n = 0;
            loop {
                let rec = siege_record(m, s);
                if m.r16(rec + count) == 0 || n >= RELEASE_STEPS {
                    break;
                }
                n += 1;
                let e = m.r32(rec + list);
                h.release_entity(m, e);
            }
        }
        for k in 0..8 {
            let at = s + 0x5DC + 4 * k;
            let e = m.r32(at);
            if e != 0 {
                let v = view(m);
                if m.r32(v + 0xC324) == u32::from(m.r8(s + 0x5D3)) {
                    h.release_entity(m, e);
                }
                m.w32(at, 0);
            }
        }
        for k in 0..3 {
            let at = s + 0x5FC + 4 * k;
            let e = m.r32(at);
            if e != 0 {
                if view(m) != 0 {
                    h.release_entity(m, e);
                }
                m.w32(at, 0);
            }
        }
        if m.r16(s + 0x3A) != 0 || m.r16(s + 0x21E) != 0 {
            for k in 0..4u32 {
                let v = view(m);
                let n = m.r16(v + 0xC5C8 + 2 * k).wrapping_add(m.r16(v + 0xC5C0 + 2 * k));
                for _ in 0..n {
                    let v = view(m);
                    let list = m.r32(v + 0xC5A0 + 4 * k);
                    let i = u32::from(m.r16(v + 0xC5B8 + 2 * k));
                    let e = m.r32(list.wrapping_add(4 * i));
                    h.release_entity(m, e);
                }
            }
        }
        let fx = m.r32(s + 0x628);
        if fx != 0 {
            h.release_entity(m, fx);
            m.w32(s + 0x628, 0);
        }
    } else {
        for at in [0x608, 0x60C, 0x610, 0x614, 0x210, 0x214, 0x218, 0x18, 0x1C, 0x574, 0x5FC, 0x600, 0x604, 0x628] {
            m.w32(s + at, 0);
        }
        m.w16(s + 0x20, 0);
        for k in 0..8 {
            m.w32(s + 0x5DC + 4 * k, 0);
        }
        forget_unit_entities(m, rec);
    }
    let q = m.r32(s + 0x5D8);
    if q != 0 {
        h.free(m, q);
    }
    m.w32(s + 0x5D8, 0);
    m.w16(s + 0x590, 0);
    m.w16(s + 0x592, 0);
    let field = m.r32(s + 0x59C);
    if field != 0 {
        h.free(m, field);
    }
    m.w32(s + 0x59C, 0);
    m.w16(s + 0x10, 0xFFFF);
    m.w16(s + 0x12, 0xFFFF);
    m.w8(s + 0x77, 0);
    for i in 0..5 {
        m.w16(s + 0x18 + 2 * i, 0);
        let rec = siege_record(m, s);
        m.w16(rec + 0x1004 + 2 * i, 0);
        m.w16(s + 0x22 + 2 * i, 0);
        m.w16(s + 0x2C + 2 * i, 0);
    }
    let block = m.r32(s + 0x70);
    if block != 0 {
        h.free(m, block);
    }
    m.w32(s + 0x70, 0);
}

/// La escena se ve en la vista: en espera (`+0x3A`) mientras supera `+0x6A`; sin espera, con asaltantes (`+0x21E`).
fn view_wanted(m: &mut dyn Mem, s: u32) -> bool {
    let wait = m.r16(s + 0x3A);
    if wait != 0 {
        wait > m.r16(s + 0x6A)
    } else {
        m.r16(s + 0x21E) > 0
    }
}

/// Número de fotogramas del sprite de una entidad (`[0x006E2E70][+0x2C] + 4`; 0 fuera de la tabla `0x006E2E6C`).
fn sprite_frames(m: &mut dyn Mem, e: u32) -> u32 {
    let n = m.r32(0x006E_2E6C);
    let c = u32::from(m.r8(e + 0x2C));
    let sprite = if c >= n {
        0
    } else {
        let table = m.r32(0x006E_2E70);
        m.r32(table.wrapping_add(4 * c))
    };
    m.r32(sprite.wrapping_add(4))
}

/// `town_siege_update_town_view` (`0x00631A50`): fotograma de la batalla con la vista abierta. Cierra la escena si
/// la vista está en otra ciudad; cada 0x100 ticks la desmonta y vuelve a montarla ([`rebuild_view_state`]) si se
/// ve. Saca cabecillas (clase 0x15, lista `+0x1F8` hasta `+0x22E`) cuando hay jefe y el rectángulo de 4x4 casillas
/// delante de `(+0x36, +0x38 + 2)` está libre de la máscara `+0x70`; sin espera ni ataque repone la milicia de las
/// dos puertas (clases 9, 0xA y 0xB hasta los cupos `+0x236..+0x23E`) si la cabeza de la cola no está esperando.
/// Con torre (`+0x250`, edificio `+0x6C`) y daño `+0x240` echa humo (clase 0x12, con viento) a intervalos del reloj
/// de fotogramas (`+0x620`/`+0x624`) y mantiene el fuego `+0x628` con el nivel de daño. `exit` como en
/// [`rebuild_view_state`].
pub fn update_town_view(m: &mut dyn Mem, h: &mut dyn SiegeViewHost, s: u32, exit: &mut u8) {
    use crate::ships::wind::table_sine;
    let city = u32::from(m.r8(s + 0x5D3));
    let v = view(m);
    if m.r32(v + 0xC324) != city {
        release_view(m, h, s, true);
        return;
    }
    if m.r32(TICK).wrapping_sub(m.r32(s + 0x618)) > 0x100 && m.r8(s + 0x7A) != 0 {
        release_view(m, h, s, true);
        let t = m.r32(TICK);
        m.w32(s + 0x618, t);
    }
    if m.r8(s + 0x7A) == 0 {
        if !view_wanted(m, s) {
            return;
        }
        rebuild_view_state(m, h, s, exit);
    }
    // Cabecillas que salen de detrás del jefe.
    if m.r32(s + 0x574) != 0 && m.r16(s + 0x20) < m.r16(s + 0x22E) {
        let (mut ox, mut oy) = (u32::from(m.r16(s + 0x36)), u32::from(m.r16(s + 0x38)) + 2);
        let mut free = true;
        let mut j = 0u8;
        while j < 4 {
            let (mut cx, mut cy) = (ox, oy);
            let mut i = 0u8;
            while i < 4 {
                let v = view(m);
                let w = m.r32(v + 0xC314);
                if cx < w && cy < m.r32(v + 0xC318) && view_wanted(m, s) {
                    let mask = m.r32(s + 0x70);
                    if mask != 0 && (cx as i16) >= 0 && (cy as i16) >= 0 {
                        let (x, y) = (cx as i16 as i32 as u32, cy as i16 as i32 as u32);
                        let v = view(m);
                        if x < w && y < m.r32(v + 0xC318) && m.r8(y.wrapping_mul(w).wrapping_add(x).wrapping_add(mask)) >= 1 {
                            free = false;
                            i = 3;
                            j = 3;
                        }
                    }
                }
                h.step(m, &mut cx, &mut cy, HEADING[7]);
                i += 1;
            }
            h.step(m, &mut ox, &mut oy, HEADING[1]);
            j += 1;
        }
        if free {
            let code = (u32::from(m.r8(s + 0x76)) << 8) + city;
            let e = h.spawn_entity(m, [0, 0, 0x15, code, u32::from(HEADING[1]), 0, 0]);
            let n = u32::from(m.r16(s + 0x20));
            m.w32(s + 0x1F8 + 4 * n, e);
            let t = m.r32(s + 0x58C);
            let e = m.r32(s + 0x1F8 + 4 * n);
            m.w32(e + 0x60, t);
            m.w16(s + 0x20, n as u16 + 1);
        }
    }
    // Milicia de refresco en las dos puertas.
    if m.r16(s + 0x3C) == 0 && m.r16(s + 0x3A) == 0 {
        let (a, b) = (u32::from(m.r16(s + 0x5A2)), u32::from(m.r16(s + 0x5A4)));
        let p1 = h.cell_to_pixel(m, a, b);
        let (a, b) = (u32::from(m.r16(s + 0x5A6)), u32::from(m.r16(s + 0x5A8)));
        let p2 = h.cell_to_pixel(m, a, b);
        for (class, count, cap, list) in [(9u32, 0x1006u32, 0x236u32, 0x410u32), (0xA, 0x1008, 0x23A, 0x80C), (0xB, 0x100A, 0x23E, 0xC08)] {
            for (g, (px, py)) in [p1, p2].into_iter().enumerate() {
                let g = g as u32;
                let rec = siege_record(m, s);
                if m.r16(rec + count) >= m.r16(s + cap) {
                    continue;
                }
                let head = m.r32(rec + 0xC + 4 * g);
                if head != 0 {
                    let k = m.r8(head + 0x5A);
                    if k == 0 || (k == 1 && m.r8(head + 0x34) == 2) {
                        continue;
                    }
                }
                let e = h.spawn_entity(m, [(px as u32) << 16, (py as u32) << 16, class, city + 0x10080, u32::from(HEADING[1]), 0, 0]);
                let rec = siege_record(m, s);
                let n = m.r16(rec + count);
                m.w32(record_list(rec, list, n), e);
                m.w16(rec + count, n.wrapping_add(1));
                let k = m.r8(rec + 0x100E + g);
                m.w8(rec + 0x100E + g, k.wrapping_add(1));
            }
        }
    }
    // Torre dañada: humo y fuego.
    let towers = m.r32(s + 0x250);
    if towers == 0 {
        m.w32(s + 0x628, 0);
        return;
    }
    let slot = m.r8(s + 0x6C);
    if u16::from(slot) >= m.r16(s + 0x242) {
        return;
    }
    let t = town(m, s);
    let id = m.r32(towers + 12 * u32::from(slot) + 8);
    let a = building_record(m, t, id);
    let bx = u32::from(m.r8(a));
    let id = m.r32(towers + 12 * u32::from(slot) + 8);
    let b = building_record(m, t, id);
    let by = u32::from(m.r8(b.wrapping_add(1)));
    let (px, py) = h.cell_to_pixel(m, bx, by);
    let damage = i32::from(m.r16(s + 0x240));
    if damage * 3 < 0x20000 {
        let next = m.r32(s + 0x620);
        let clock = m.r32(FRAME_CLOCK);
        if clock > next {
            m.w32(s + 0x624, next);
            let r1 = (h.rand(m) as i32).wrapping_rem(0xCF0F0);
            let y = r1.wrapping_add(((py as u32) << 16) as i32).wrapping_sub(0x67878);
            let r2 = (h.rand(m) as i32).wrapping_rem(0x14_0000);
            let x = r2.wrapping_add(((px.wrapping_sub(10) as u32) << 16) as i32);
            let e = h.spawn_entity(m, [x as u32, y as u32, 0x12, 0x40_0024, u32::from(HEADING[1]), 0, 0]);
            let damage = i32::from(m.r16(s + 0x240));
            let clock = m.r32(FRAME_CLOCK);
            m.w32(s + 0x620, (((damage * 1200) >> 16) as u32).wrapping_add(clock).wrapping_add(0xC8));
            m.w8(e + 0x30, (((damage * 345) >> 16) as u8).wrapping_add(0x19));
            let (direction, strength) = h.wind(m);
            let dir = direction as u8;
            let speed = (strength & 0xFF) as i32;
            let cos = table_sine(dir.wrapping_add(0x40)).wrapping_mul(speed).wrapping_neg();
            let sin = table_sine(dir).wrapping_mul(speed);
            m.w32(e + 0x40, (sin.wrapping_add(500) / 1000) as u32);
            m.w32(e + 0x44, (cos.wrapping_add(500) / 1000) as u32);
        } else {
            let at = (((damage * 1200) >> 16) as u32).wrapping_add(clock).wrapping_add(0xC8);
            if at < next {
                m.w32(s + 0x620, at);
            }
        }
    }
    let damage = i32::from(m.r16(s + 0x240));
    if damage * 3 >= 0x10000 {
        let fx = m.r32(s + 0x628);
        if fx != 0 {
            h.release_entity(m, fx);
            m.w32(s + 0x628, 0);
        }
        return;
    }
    let level = 3u8.wrapping_sub(((damage * 9) >> 16) as u8);
    let fx = m.r32(s + 0x628);
    if fx != 0 {
        if u32::from(level) == sprite_frames(m, fx) {
            h.place_entity(m, fx, px as u32, py as u32, 0);
        } else {
            h.release_entity(m, fx);
            m.w32(s + 0x628, 0);
        }
    }
    if m.r32(s + 0x628) == 0 {
        let e = h.spawn_entity(m, [(px as u32) << 16, (py as u32) << 16, 0x12, (u32::from(level) << 8) + 0x40_00B0, u32::from(HEADING[1]), 0, 0]);
        m.w32(s + 0x628, e);
    }
}
