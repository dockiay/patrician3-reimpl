//! Asedio con la vista de ciudad abierta (`TownSiege+0x77 != 0`): campo de distancias de los asaltantes y
//! marcadores de los puntos de entrada. Lo usan las órdenes del asedio (`0x00634660`, `0x006347B0`,
//! `0x00634820`) a través de [`crate::economy::siege_record::OrderHost`]:
//!
//! * `0x0062B860` reinicia el campo (`+0x59C`, `u16` por casilla de la vista) hacia una casilla y lo siembra
//!   (`0x0062BE90`), con la cola por niveles de `0x00632A20` (`+0x590..+0x5A0`, `+0x5D8`) y la prueba de paso de
//!   `0x0062B410` (capas de la vista `+0xC308/+0xC30C`, puertas `+0x244`, plano de la ciudad).
//! * `0x0062DD70` bloquea las casillas de cada punto de entrada (`0x0062BD50`, relajación recursiva `0x0062C0A0`)
//!   y crea o apaga sus marcadores en la vista (`0x00590610`, `0x0057AF40`).
//!
//! El porte trabaja sobre la memoria del proceso original ([`Mem`]) con sus direcciones: la vista es
//! `[0x006E51AC]` y las ciudades `[0x006DE508]`. Lo que pertenece a la vista o al CRT (tipo de una pieza por su
//! método virtual `+0x50`, geometría `0x0058B740/0x0058CFB0/0x0058D030`, reserva de memoria y entidades) pasa por
//! [`SiegeViewHost`]. Las lecturas fuera del campo (casillas vecinas de un punto en el borde) se hacen en la
//! memoria contigua, como el original.
use crate::flatmem::Mem;

/// Puntero global a la vista de ciudad (`Class22`).
pub const VIEW: u32 = 0x006E_51AC;
/// Puntero global a la tabla de ciudades.
pub const TOWNS: u32 = 0x006D_E508;
pub(crate) const TOWN_SIZE: u32 = 0x9F8;

/// Desplazamientos de las vecinas de los anillos (`0x0067B610`, `0x0067B620`): columnas y filas.
pub(crate) const RING_DX: [i32; 8] = [1, 0, -1, 0, 1, -1, -1, 1];
pub(crate) const RING_DY: [i32; 8] = [0, 2, 0, -2, 2, 2, -2, -2];

/// Llamadas del porte a la vista y al CRT.
pub trait SiegeViewHost {
    /// Método virtual `+0x50` de la pieza `object` de la vista: su tipo.
    fn tile_kind(&mut self, m: &mut dyn Mem, object: u32) -> u32;
    /// `0x0058B740`: un paso en la rejilla; devuelve si sigue dentro.
    fn step(&mut self, m: &mut dyn Mem, col: &mut u32, row: &mut u32, dir: u8) -> bool;
    /// `0x0058D030`: casilla del píxel (escribe columna y fila).
    fn pixel_to_cell(&mut self, m: &mut dyn Mem, x: i32, y: i32) -> (u32, u32);
    /// `0x0058CFB0`: píxel de la casilla.
    fn cell_to_pixel(&mut self, m: &mut dyn Mem, col: u32, row: u32) -> (i32, i32);
    /// `0x0064F7B9`: reserva.
    fn alloc(&mut self, m: &mut dyn Mem, size: u32) -> u32;
    /// `0x0063A29F`: `realloc`.
    fn realloc(&mut self, m: &mut dyn Mem, block: u32, size: u32) -> u32;
    /// `0x0063A1B6`: libera.
    fn free(&mut self, m: &mut dyn Mem, block: u32);
    /// `0x00590610` (`thiscall` sobre la vista): crea una entidad y devuelve su puntero.
    fn spawn_entity(&mut self, m: &mut dyn Mem, args: [u32; 7]) -> u32;
    /// `0x0057AF40(entidad, a, b)`: fundido del marcador.
    fn set_fade(&mut self, m: &mut dyn Mem, entity: u32, a: u32, b: u32);
    /// `0x00598150` (`thiscall` sobre la vista): los transeúntes huyen al empezar el asalto.
    fn make_walkers_flee(&mut self, _m: &mut dyn Mem) {}
    /// Método virtual `+4` del gestor de entidades de la vista (`vista+0x3F8`): retira una entidad.
    fn release_entity(&mut self, _m: &mut dyn Mem, _entity: u32) {}
    /// `0x00593580` (`thiscall` sobre la vista): ¿la casilla está libre para una entidad del enlace `link`? `None`
    /// (por defecto) usa el porte sobre la memoria de la vista ([`crate::economy::siege_battle::cell_walkable_flat`]).
    fn cell_walkable(&mut self, _m: &mut dyn Mem, _col: u32, _row: u32, _flag: u8, _link: u16) -> Option<bool> {
        None
    }
    /// `0x00639C03` (`rand` del CRT).
    fn rand(&mut self, _m: &mut dyn Mem) -> u32 {
        0
    }
    /// `0x00618D10` (`thiscall` sobre el mapa local `0x006E6220`): rumbo y fuerza del viento.
    fn wind(&mut self, _m: &mut dyn Mem) -> (u32, u32) {
        (0, 0)
    }
    /// Método virtual `+0x48` de una entidad de la vista: la coloca en el píxel `(x, y)` (tercer argumento 0).
    fn place_entity(&mut self, _m: &mut dyn Mem, _entity: u32, _x: u32, _y: u32, _z: u32) {}
}

pub(crate) fn view(m: &mut dyn Mem) -> u32 {
    m.r32(VIEW)
}

pub(crate) fn width(m: &mut dyn Mem) -> u32 {
    let v = view(m);
    m.r32(v + 0xC314)
}

pub(crate) fn height(m: &mut dyn Mem) -> u32 {
    let v = view(m);
    m.r32(v + 0xC318)
}

/// Celda del campo `(x, y)` (aritmética de 32 bits, como el original).
pub(crate) fn flow_cell(m: &mut dyn Mem, s: u32, x: u32, y: u32) -> u32 {
    let w = width(m);
    m.r32(s + 0x59C).wrapping_add(w.wrapping_mul(y).wrapping_add(x).wrapping_mul(2))
}

pub(crate) fn ftol_sqrt(d2: i32) -> u32 {
    // `0x0063AAE4` (raíz) y `__ftol`: truncamiento hacia cero.
    let r = f64::from(d2).sqrt();
    if r.is_finite() && r < 2_147_483_648.0 {
        r as i64 as u32
    } else {
        0x8000_0000
    }
}

/// Distancia de la escena (`+8`, `+0xA`, `i16`) a `(col, row)` con la columna al doble.
fn distance(m: &mut dyn Mem, s: u32, col: i32, row: i32) -> u32 {
    let dx = (i32::from(m.r16(s + 8) as i16).wrapping_sub(col)).wrapping_mul(2);
    let dy = i32::from(m.r16(s + 0xA) as i16).wrapping_sub(row);
    ftol_sqrt(dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx)))
}

/// Distancia desde la casilla del objeto `+0x574` (píxel `+0xE`, `+0x12`).
fn leader_distance(m: &mut dyn Mem, h: &mut dyn SiegeViewHost, s: u32, leader: u32) -> u32 {
    let x = i32::from(m.r16(leader + 0xE) as i16);
    let y = i32::from(m.r16(leader + 0x12) as i16);
    let (c, r) = h.pixel_to_cell(m, x, y);
    distance(m, s, i32::from(c as u16 as i16), i32::from(r as u16 as i16))
}

/// `town_siege_is_cell_passable` (0x0062B410).
pub fn is_cell_passable(m: &mut dyn Mem, h: &mut dyn SiegeViewHost, s: u32, x: u32, y: u32, near: bool) -> bool {
    let v = view(m);
    let w = m.r32(v + 0xC314);
    if x >= w || y >= m.r32(v + 0xC318) {
        return false;
    }
    if x == u32::from(m.r16(s + 0x5CA)) && y == u32::from(m.r16(s + 0x5CE)) {
        return true;
    }
    if x == u32::from(m.r16(s + 0x5CC)) && y == u32::from(m.r16(s + 0x5D0)) {
        return true;
    }
    let idx = w.wrapping_mul(y).wrapping_add(x);
    let tiles = m.r32(v + 0x98);
    let layer = m.r32(v + 0xC308);
    let ground = m.r16(layer.wrapping_add(idx.wrapping_mul(4)));
    let object = m.r32(tiles.wrapping_add(u32::from(ground) * 4));
    let k1 = h.tile_kind(m, object);
    let v = view(m);
    let layer = m.r32(v + 0xC30C);
    let upper = m.r16(layer.wrapping_add(idx.wrapping_mul(4)));
    if upper != 0 {
        let tiles = m.r32(v + 0x98);
        let object = m.r32(tiles.wrapping_add(u32::from(upper) * 4));
        let k2 = h.tile_kind(m, object);
        if k2 == 0x2F {
            if !matches!(k1, 0x18 | 0x10 | 0x17) {
                return false;
            }
            // Puerta: la casilla cuenta si está junto a una de las tres puertas; solo la elegida (`+0x6D`) deja
            // pasar, y entonces según `+0x78`.
            for slot in 0..3u8 {
                let gate = m.r32(s + 0x244 + 4 * u32::from(slot));
                let city = u32::from(m.r8(s + 0x5D3));
                let town = m.r32(TOWNS).wrapping_add(city * TOWN_SIZE);
                let rec = if gate < u32::from(m.r16(town + 0x776)) { m.r32(town + 0x75C).wrapping_add(gate * 8) } else { 0 };
                let gx = (u32::from(m.r8(rec)).wrapping_sub(x) as i32).wrapping_abs();
                if gx > 2 {
                    continue;
                }
                let gy = (u32::from(m.r8(rec + 1)).wrapping_sub(y) as i32).wrapping_abs();
                if gy <= 4 {
                    if slot != m.r8(s + 0x6D) {
                        return false;
                    }
                    return m.r8(s + 0x78) != 0;
                }
            }
            return true;
        }
        if !(0x30..=0x31).contains(&k2) {
            return false;
        }
    }
    match k1.wrapping_sub(0x10) {
        0 | 7 | 0x18 => true,
        8 => {
            // Calle (`0x18`): casillas del plano con el bit alto pasan; el resto solo lejos del objetivo.
            let city = u32::from(m.r8(s + 0x5D3));
            let town = m.r32(TOWNS).wrapping_add(city * TOWN_SIZE);
            let stride = m.r32(town + 0x7AC);
            let cells = m.r32(town + 0x80C);
            if (m.r8(stride.wrapping_mul(x).wrapping_add(cells).wrapping_add(y)) as i8) < 0 {
                return true;
            }
            let leader = m.r32(s + 0x574);
            let (xi, yi) = (i32::from(x as u16 as i16), i32::from(y as u16 as i16));
            if m.r8(s + 0x78) == 0 {
                let limit = if leader == 0 {
                    0x19
                } else {
                    let d = leader_distance(m, h, s, leader);
                    if d as u16 <= 0x19 { d } else { 0x19 }
                };
                if !near {
                    return true;
                }
                (distance(m, s, xi, yi) as u16) > (limit as u16)
            } else {
                let lead = if leader == 0 { 0xFFFF } else { leader_distance(m, h, s, leader) };
                if !near {
                    return true;
                }
                let lead = (lead & 0xFFFF) as i32;
                let d = (distance(m, s, xi, yi) & 0xFFFF) as i32;
                if d >= lead + 5 {
                    return true;
                }
                let d = (distance(m, s, xi, yi) & 0xFFFF) as i32;
                d + 5 <= lead
            }
        }
        _ => false,
    }
}

/// `town_siege_distance_queue_push` (0x00632A20): encola `(x, y)` según su valor del campo respecto a la base
/// `+0x5A0` (cuatro niveles con colas en `+0x594..+0x59A`, lista circular de nodos de seis bytes en `+0x5D8`).
pub fn queue_push(m: &mut dyn Mem, h: &mut dyn SiegeViewHost, s: u32, x: u32, y: u32) {
    let (x, y) = (x & 0xFFFF, y & 0xFFFF);
    let cell = flow_cell(m, s, x, y);
    let value = m.r16(cell);
    if value == 0xFFFF {
        return;
    }
    let node = |m: &mut dyn Mem, i: u16| m.r32(s + 0x5D8).wrapping_add(u32::from(i) * 6);
    let cap = m.r16(s + 0x590);
    if cap == 0 {
        m.w16(s + 0x590, 0x32);
        let q = h.realloc(m, 0, 0x12C);
        m.w32(s + 0x5D8, q);
        if q & 0xFFFF_FF00 == 0 {
            m.w32(s + 0x5D8, 0);
            m.w16(s + 0x590, 0);
            m.w16(s + 0x592, 0);
            return;
        }
        let cap = m.r16(s + 0x590);
        for i in 0..cap.saturating_sub(1) {
            let n = node(m, i);
            m.w16(n + 4, i + 1);
        }
        let last = node(m, cap.wrapping_sub(1));
        m.w16(last + 4, 0);
        m.w16(s + 0x592, 0);
        for at in [0x594, 0x596, 0x598, 0x59A] {
            m.w16(s + at, cap.wrapping_sub(1));
        }
    } else {
        let tail = m.r16(s + 0x59A);
        let t = node(m, tail);
        if m.r16(t + 4) == m.r16(s + 0x592) && m.r8(s + 0x5D5) != 0 {
            let grown = cap.wrapping_add(0x32);
            m.w16(s + 0x590, grown);
            let old = m.r32(s + 0x5D8);
            let q = h.realloc(m, old, u32::from(grown) * 6);
            m.w32(s + 0x5D8, q);
            if q & 0xFFFF_FF00 == 0 {
                m.w32(s + 0x5D8, 0);
                m.w16(s + 0x590, 0);
                m.w16(s + 0x592, 0);
                return;
            }
            let grown = m.r16(s + 0x590);
            let first = grown.wrapping_sub(0x32);
            let mut i = first;
            while i32::from(i) < i32::from(grown) - 1 {
                let n = node(m, i);
                m.w16(n + 4, i.wrapping_add(1));
                i = i.wrapping_add(1);
            }
            let head = m.r16(s + 0x592);
            let last = node(m, grown.wrapping_sub(1));
            m.w16(last + 4, head);
            let tail = m.r16(s + 0x59A);
            let t = node(m, tail);
            m.w16(t + 4, first);
        }
    }
    let tail = m.r16(s + 0x59A);
    let t = node(m, tail);
    let free = m.r16(t + 4);
    let base = u32::from(m.r16(s + 0x5A0));
    let value = u32::from(value);
    // Desengancha el nodo libre que sigue a la cola y lo inserta tras la cola de su nivel.
    let mut insert = |m: &mut dyn Mem, level: u32, also: &[u32]| {
        let f = node(m, free);
        let next = m.r16(f + 4);
        m.w16(t + 4, next);
        let at = m.r16(s + level);
        let l = node(m, at);
        let after = m.r16(l + 4);
        m.w16(f + 4, after);
        let l = node(m, at);
        m.w16(l + 4, free);
        for &other in also {
            if m.r16(s + other) == at {
                m.w16(s + other, free);
            }
        }
        m.w16(s + level, free);
        let f = node(m, free);
        m.w16(f, x as u16);
    };
    let level = if value <= base {
        insert(m, 0x594, &[0x59A, 0x598, 0x596]);
        0x594
    } else if value == base + 1 {
        insert(m, 0x596, &[0x59A, 0x598]);
        0x596
    } else if value == base + 2 {
        insert(m, 0x598, &[0x59A]);
        0x598
    } else {
        m.w16(s + 0x59A, free);
        let f = node(m, free);
        m.w16(f, x as u16);
        0x59A
    };
    let at = m.r16(s + level);
    let n = node(m, at);
    m.w16(n + 2, y as u16);
    m.w8(s + 0x5D5, 1);
}

/// `town_siege_seed_flow_field` (0x0062BE90): la casilla objetivo (`+0x10`, `+0x12`) vale `+0x5A0 = 2`; si no
/// es de paso, se buscan por anillos diagonales las primeras casillas de paso en cada una de las cuatro
/// diagonales.
pub fn seed_flow_field(m: &mut dyn Mem, h: &mut dyn SiegeViewHost, s: u32, wide: bool) {
    m.w16(s + 0x5A0, 2);
    let near = m.r8(s + 0x6F) != 0xFF;
    let (tx, ty) = (u32::from(m.r16(s + 0x10)), u32::from(m.r16(s + 0x12)));
    if is_cell_passable(m, h, s, tx, ty, near) {
        let cell = flow_cell(m, s, tx, ty);
        let base = m.r16(s + 0x5A0);
        m.w16(cell, base);
        queue_push(m, h, s, tx, ty);
        m.w8(s + 0x5D5, 0);
        return;
    }
    let mut best: u32 = 0xFFFF;
    let mut done = [false; 4];
    let mut radius: u32 = 1;
    loop {
        let mut dir: u32 = 0x20;
        while dir & 0xFFFF < 0x100 {
            let k = (dir >> 6) as usize;
            if !done[k] {
                let (mut col, mut row) = (u32::from(m.r16(s + 0x10)), u32::from(m.r16(s + 0x12)));
                for _ in 0..(radius & 0xFFFF) {
                    h.step(m, &mut col, &mut row, dir as u8);
                }
                if col < width(m) && row < height(m) {
                    let near = m.r8(s + 0x6F) != 0xFF;
                    if is_cell_passable(m, h, s, col, row, near) {
                        let cell = flow_cell(m, s, col, row);
                        let base = m.r16(s + 0x5A0);
                        best = radius;
                        done[k] = true;
                        m.w16(cell, base);
                        queue_push(m, h, s, col, row);
                    } else {
                        let cell = flow_cell(m, s, col, row);
                        m.w16(cell, 0xFFFF);
                        if is_cell_passable(m, h, s, col, row, false) {
                            done[k] = true;
                        }
                    }
                }
            }
            dir += 0x40;
        }
        if (radius & 0xFFFF) > 2 && !wide {
            break;
        }
        radius = radius.wrapping_add(1);
        if ((best & 0xFFFF) as i32) < ((radius & 0xFFFF) as i32) - 1 {
            break;
        }
    }
    m.w8(s + 0x5D5, 0);
}

/// `town_siege_reset_flow_field_to_cell` (0x0062B860): borra el campo (lo reserva si falta), libera la cola y lo
/// siembra hacia `(a, b)`; deja `+0x77 = 1`.
pub fn reset_flow_field_to_cell(m: &mut dyn Mem, h: &mut dyn SiegeViewHost, s: u32, a: u32, b: u32, wide: u32) {
    let (a, b) = (a & 0xFFFF, b & 0xFFFF);
    let mut wide = wide;
    loop {
        if a >= width(m) || b >= height(m) {
            return;
        }
        if m.r32(s + 0x59C) == 0 {
            let size = height(m).wrapping_mul(width(m)).wrapping_mul(2);
            let p = h.alloc(m, size);
            m.w32(s + 0x59C, p);
        }
        if m.r32(s + 0x59C) & 0xFFFF_FF00 != 0 {
            break;
        }
        m.w32(s + 0x59C, 0);
        wide &= !0xFF;
    }
    let size = height(m).wrapping_mul(width(m)).wrapping_mul(2);
    let flow = m.r32(s + 0x59C);
    for i in 0..size {
        m.w8(flow.wrapping_add(i), 0);
    }
    let q = m.r32(s + 0x5D8);
    m.w8(s + 0x77, 0);
    if q != 0 {
        h.free(m, q);
    }
    m.w32(s + 0x5D8, 0);
    m.w16(s + 0x590, 0);
    m.w16(s + 0x592, 0);
    m.w16(s + 0x10, a as u16);
    m.w16(s + 0x12, b as u16);
    seed_flow_field(m, h, s, wide & 0xFF != 0);
    m.w8(s + 0x77, 1);
}

/// `town_siege_flow_field_relax_cell` (0x0062C0A0): recalcula la casilla `(x, y)` desde sus vecinas y propaga
/// el cambio a las que dependían de ella.
pub fn relax_cell(m: &mut dyn Mem, h: &mut dyn SiegeViewHost, s: u32, x: u32, y: u32) {
    let (mut x, mut y) = (x, y);
    h.step(m, &mut x, &mut y, 0xE0);
    let mut best: u32 = 0xFFFF;
    for k in 0..2u32 {
        for j in 4 * k as usize..4 + 4 * k as usize {
            let cell = flow_cell(m, s, x, y);
            let v = u32::from(m.r16(cell));
            if v != 0 && v + k < best & 0xFFFF {
                best = (k + v) & 0xFFFF;
            }
            x = x.wrapping_add(RING_DX[j] as u32);
            y = y.wrapping_add(RING_DY[j] as u32);
        }
    }
    h.step(m, &mut x, &mut y, 0x80);
    let cell = flow_cell(m, s, x, y);
    let old = m.r16(cell);
    let mut value = old.wrapping_add(2);
    if best == 0xFFFF {
        if old == 0xFFFF || old == 0 {
            return;
        }
        m.w16(cell, 0);
    } else {
        if u32::from(old) == best + 2 {
            return;
        }
        m.w16(cell, (best as u16).wrapping_add(2));
    }
    if value < 3 {
        return;
    }
    h.step(m, &mut x, &mut y, 0xE0);
    for k in 0..2usize {
        for j in 4 * k..4 + 4 * k {
            let cell = flow_cell(m, s, x, y);
            if m.r16(cell) == value {
                relax_cell(m, h, s, x, y);
            }
            x = x.wrapping_add(RING_DX[j] as u32);
            y = y.wrapping_add(RING_DY[j] as u32);
        }
        h.step(m, &mut x, &mut y, 0x20);
        value = value.wrapping_add(1);
    }
}

/// `town_siege_flow_field_block_cell` (0x0062BD50): la casilla deja de ser de paso y se relajan las vecinas
/// que tomaban su valor.
pub fn block_cell(m: &mut dyn Mem, h: &mut dyn SiegeViewHost, s: u32, x: u32, y: u32) {
    let cell = flow_cell(m, s, x, y);
    let old = m.r16(cell);
    let mut value = old.wrapping_add(2);
    m.w16(cell, 0xFFFF);
    if value < 3 {
        return;
    }
    let (mut x, mut y) = (x, y);
    h.step(m, &mut x, &mut y, 0xE0);
    let mut j = 0usize;
    for k in 0..2usize {
        while j < 4 + 4 * k {
            let cell = flow_cell(m, s, x, y);
            if m.r16(cell) == value {
                relax_cell(m, h, s, x, y);
            }
            x = x.wrapping_add(RING_DX[j] as u32);
            y = y.wrapping_add(RING_DY[j] as u32);
            j += 1;
        }
        h.step(m, &mut x, &mut y, 0x20);
        value = value.wrapping_add(1);
    }
}

/// `town_siege_update_entry_point_markers` (0x0062DD70): bloquea en el campo cada punto de entrada (`+0x5D4`
/// puntos en `+0x5AA/+0x5B6` y dos más en `+0x5C2/+0x5C6`) y sus tres vecinas; con `show` crea su marcador
/// (`+0x5DC`), sin él lo apaga.
pub fn update_entry_point_markers(m: &mut dyn Mem, h: &mut dyn SiegeViewHost, s: u32, show: bool) {
    let mut i: u32 = 0;
    while i < u32::from(m.r8(s + 0x5D4)) + 2 {
        let n = u32::from(m.r8(s + 0x5D4));
        let point = |m: &mut dyn Mem| {
            if i < n {
                (u32::from(m.r16(s + 0x5AA + 2 * i)), u32::from(m.r16(s + 0x5B6 + 2 * i)))
            } else {
                let j = i.wrapping_sub(n);
                (u32::from(m.r16(s.wrapping_add(j.wrapping_mul(2)).wrapping_add(0x5C2))), u32::from(m.r16(s.wrapping_add(j.wrapping_mul(2)).wrapping_add(0x5C6))))
            }
        };
        let (mut col, mut row) = point(m);
        block_cell(m, h, s, col, row);
        for dir in [0xE0u8, 0x20, 0x60] {
            h.step(m, &mut col, &mut row, dir);
            block_cell(m, h, s, col, row);
        }
        let slot = s + 0x5DC + 4 * i;
        if show {
            let n = u32::from(m.r8(s + 0x5D4));
            let (col, row) = if i < n {
                (u32::from(m.r16(s + 0x5AA + 2 * i)), u32::from(m.r16(s + 0x5B6 + 2 * i)))
            } else {
                let j = i - n;
                (u32::from(m.r16(s + 0x5C2 + 2 * j)), u32::from(m.r16(s + 0x5C6 + 2 * j)))
            };
            if m.r32(slot) == 0 {
                let (px, py) = h.cell_to_pixel(m, col, row);
                let frame = ((u32::from(m.r8(s + 0x5D4)) + i) << 8 & 0x300) + 0x180;
                let entity = h.spawn_entity(m, [(px as u32) << 16, (py as u32) << 16, 0x12, frame, 0x20, 0, 0]);
                m.w32(slot, entity);
                if i32::from(m.r16(s + 0x42)) > i32::from(m.r16(s + 0x44)) - 0x40 {
                    h.set_fade(m, entity, 0, 0x100);
                }
            }
        } else {
            let entity = m.r32(slot);
            if entity != 0 {
                h.set_fade(m, entity, 0, 0xFFFF_FF00);
            }
            m.w32(slot, 0);
        }
        i += 1;
    }
}

/// Distancia de los puntos de reunión: columnas al doble (la rejilla es de filas alternas), aritmética de 32 bits.
fn rally_distance(x: u32, y: u32, px: u32, py: u32) -> i32 {
    let dx = (x.wrapping_sub(px) as i32).wrapping_mul(2);
    let dy = y.wrapping_sub(py) as i32;
    dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx))
}

/// `town_siege_pick_rally_points` (`0x0062C340`): elige los puntos de entrada del asalto entre los puntos de
/// reunión de la vista (lista 4 del `.spawn`, `+0xC698` cuántos y `+0xC6AC` pares `(x, y)` de 32 bits).
///
/// El primero (`+0x5AA/+0x5B6`) es el más cercano al objetivo `+0x36/+0x38`; los siguientes, hasta `+0x5D4`
/// (que pasa a ser cuántos hay), los demás a distancia `0 < d < 0x4C9` de él; `+0x14/+0x16` recibe su media. Los
/// dos puntos extra (`+0x5C2/+0x5C6`) son el punto de reunión más cercano a la casilla de las puertas
/// `(+0x76 + j + 1) % 3` de la tabla de caminos `+0xC674` (píxel convertido con `0x0058D030`). Sin puntos de
/// reunión no cambia nada. Los contadores son de 8 bits, como en el original.
pub fn pick_rally_points(m: &mut dyn Mem, h: &mut dyn SiegeViewHost, s: u32) {
    let v = view(m);
    if m.r32(v + 0xC698) == 0 {
        return;
    }
    let point = |m: &mut dyn Mem, i: u8| {
        let v = view(m);
        let list = m.r32(v + 0xC6AC).wrapping_add(u32::from(i) * 8);
        (m.r32(list), m.r32(list + 4))
    };
    let count = |m: &mut dyn Mem| {
        let v = view(m);
        m.r32(v + 0xC698)
    };
    let (tx, ty) = (u32::from(m.r16(s + 0x36)), u32::from(m.r16(s + 0x38)));
    let mut best = 0x07FF_FFFFi32;
    let mut i = 0u8;
    while u32::from(i) < count(m) {
        let (px, py) = point(m, i);
        let d = rally_distance(tx, ty, px, py);
        if d < best {
            best = d;
            m.w16(s + 0x5AA, px as u16);
            m.w16(s + 0x5B6, py as u16);
        }
        i = i.wrapping_add(1);
    }
    let mut n = 1u8;
    let mut i = 0u8;
    while u32::from(i) < count(m) {
        let (px, py) = point(m, i);
        let d = rally_distance(u32::from(m.r16(s + 0x5AA)), u32::from(m.r16(s + 0x5B6)), px, py);
        if d < 0x4C9 && n < m.r8(s + 0x5D4) && d != 0 {
            m.w16(s + 0x5AA + 2 * u32::from(n), px as u16);
            m.w16(s + 0x5B6 + 2 * u32::from(n), py as u16);
            n = n.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    m.w16(s + 0x14, 0);
    m.w16(s + 0x16, 0);
    for k in 0..u32::from(n) {
        let (a, b) = (m.r16(s + 0x14), m.r16(s + 0x16));
        let (x, y) = (m.r16(s + 0x5AA + 2 * k), m.r16(s + 0x5B6 + 2 * k));
        m.w16(s + 0x14, a.wrapping_add(x));
        m.w16(s + 0x16, b.wrapping_add(y));
    }
    m.w8(s + 0x5D4, n);
    // `idiv` de 32 bits con el dividendo sin signo de 16.
    let div = i32::from(n);
    let (a, b) = (i32::from(m.r16(s + 0x14)), i32::from(m.r16(s + 0x16)));
    m.w16(s + 0x14, (a / div) as u16);
    m.w16(s + 0x16, (b / div) as u16);
    for j in 0..2u32 {
        let mut best = 0x07FF_FFFFi32;
        let mut i = 0u8;
        while u32::from(i) < count(m) {
            let k = (u32::from(m.r8(s + 0x76)) + j + 1) % 3;
            let v = view(m);
            let table = m.r32(v + 0xC674);
            let at = m.r32(table.wrapping_add(k * 8 + 4));
            let (x, y) = (m.r32(at) as i32, m.r32(at + 4) as i32);
            let (cx, cy) = h.pixel_to_cell(m, x, y);
            let (px, py) = point(m, i);
            let d = rally_distance(cx, cy, px, py);
            if d < best {
                best = d;
                m.w16(s + 0x5C2 + 2 * j, px as u16);
                m.w16(s + 0x5C6 + 2 * j, py as u16);
            }
            i = i.wrapping_add(1);
        }
    }
}

/// Rumbo de la rejilla entre dos píxeles consecutivos de un camino (`0x00677928` + 1, 3, 5, 7).
pub(crate) fn path_heading(x0: i32, y0: i32, x1: i32, y1: i32) -> u8 {
    match (x1 > x0, y1 > y0) {
        (true, true) => 0x60,
        (true, false) => 0x20,
        (false, true) => 0xA0,
        (false, false) => 0xE0,
    }
}

/// Puntero a los nodos del camino `k` de la clave `key` del `.nodes` (`vista+0xC670 + 4 * key`, entradas
/// `(cuántos, puntero)` de 8 bytes).
pub(crate) fn path_nodes(m: &mut dyn Mem, key: u32, k: u32) -> u32 {
    let v = view(m);
    let table = m.r32(v + 0xC670 + 4 * key);
    m.r32(table.wrapping_add(k * 8 + 4))
}

pub(crate) fn node(m: &mut dyn Mem, nodes: u32, i: u32) -> (i32, i32) {
    let at = nodes.wrapping_add(i * 8);
    (m.r32(at) as i32, m.r32(at + 4) as i32)
}

/// `town_siege_select_approach_route` (`0x0062C630`): primer camino de puertas (clave 1 del `.nodes`) con un
/// nodo a distancia `< 0x10` (columnas al doble) de la casilla objetivo `+8/+0xA`. Escribe la casilla de su
/// primer nodo en `+0x36/+0x38` y el camino en `+0x76`; si ninguno se acerca no cambia nada.
pub fn select_approach_route(m: &mut dyn Mem, h: &mut dyn SiegeViewHost, s: u32) {
    for k in 0..3u32 {
        let mut i = 0u8;
        loop {
            let v = view(m);
            let table = m.r32(v + 0xC674);
            if u32::from(i) >= m.r32(table.wrapping_add(k * 8)) {
                break;
            }
            let nodes = m.r32(table.wrapping_add(k * 8 + 4));
            let (x, y) = node(m, nodes, u32::from(i));
            let (cx, cy) = h.pixel_to_cell(m, x, y);
            let dx = (u32::from(m.r16(s + 8)).wrapping_sub(cx) as i32).wrapping_mul(2);
            let dy = u32::from(m.r16(s + 0xA)).wrapping_sub(cy) as i32;
            if dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx)) < 0x10 {
                let first = path_nodes(m, 1, k);
                let (x, y) = node(m, first, 0);
                let (cx, cy) = h.pixel_to_cell(m, x, y);
                m.w16(s + 0x36, cx as u16);
                m.w16(s + 0x38, cy as u16);
                m.w8(s + 0x76, k as u8);
                return;
            }
            i = i.wrapping_add(1);
        }
    }
}

/// `0x00631940`: dos pasos desde `(x, y)` en el rumbo `dir`; ¿es calle (clase `0x18` del suelo)?
fn road_two_steps_ahead(m: &mut dyn Mem, h: &mut dyn SiegeViewHost, mut x: u32, mut y: u32, dir: u8) -> bool {
    h.step(m, &mut x, &mut y, dir);
    h.step(m, &mut x, &mut y, dir);
    let v = view(m);
    let w = m.r32(v + 0xC314);
    if x >= w || y >= m.r32(v + 0xC318) {
        return false;
    }
    let idx = w.wrapping_mul(y).wrapping_add(x);
    let layer = m.r32(v + 0xC308);
    let ground = m.r16(layer.wrapping_add(idx.wrapping_mul(4)));
    let tiles = m.r32(v + 0x98);
    let object = m.r32(tiles.wrapping_add(u32::from(ground) * 4));
    h.tile_kind(m, object) == 0x18
}

/// `town_siege_compute_approach_lanes` (`0x00631730`): para los dos caminos de puertas distintos del elegido
/// (`(+0x76 + j + 1) % 3`), casilla de su último nodo de la clave 2 (`+0x48/+0x50`), de su primer nodo de la clave 1
/// (`+0x46/+0x4E`), rumbo del camino (`+0x74`) y el carril: avance por la calle mientras se acerca al punto extra
/// `+0x5C2/+0x5C6` (`+0x56/+0x5E`) y la casilla a su izquierda (`+0x58/+0x60`).
pub fn compute_approach_lanes(m: &mut dyn Mem, h: &mut dyn SiegeViewHost, s: u32) {
    for j in 0..2u32 {
        let k = (u32::from(m.r8(s + 0x76)) + j + 1) % 3;
        let v = view(m);
        let entry = m.r32(v + 0xC678).wrapping_add(k * 8);
        let last = u32::from(m.r8(entry).wrapping_sub(1));
        let nodes = m.r32(entry + 4);
        let (x, y) = node(m, nodes, last);
        let (cx, cy) = h.pixel_to_cell(m, x, y);
        m.w16(s + 0x48 + 4 * j, cx as u16);
        m.w16(s + 0x50 + 4 * j, cy as u16);
        let first = path_nodes(m, 1, k);
        let (x0, y0) = node(m, first, 0);
        let (mut cx, mut cy) = h.pixel_to_cell(m, x0, y0);
        m.w16(s + 0x46 + 4 * j, cx as u16);
        m.w16(s + 0x4E + 4 * j, cy as u16);
        let first = path_nodes(m, 1, k);
        let (x1, y1) = node(m, first, 1);
        let dir = path_heading(x0, y0, x1, y1);
        m.w8(s + 0x74 + j, dir);
        let mut best = 0x7FFF_FFFFi32;
        if road_two_steps_ahead(m, h, cx, cy, dir) {
            loop {
                h.step(m, &mut cx, &mut cy, dir);
                let ex = u32::from(m.r16(s + 0x5C2 + 2 * j));
                let ey = u32::from(m.r16(s + 0x5C6 + 2 * j));
                let dx = cx.wrapping_sub(ex) as i32;
                let dy = cy.wrapping_sub(ey) as i32;
                let d = dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx));
                if d > best {
                    break;
                }
                best = d;
                if !road_two_steps_ahead(m, h, cx, cy, dir) {
                    break;
                }
            }
        }
        m.w16(s + 0x56 + 4 * j, cx as u16);
        m.w16(s + 0x5E + 4 * j, cy as u16);
        h.step(m, &mut cx, &mut cy, dir.wrapping_sub(0x40));
        m.w16(s + 0x58 + 4 * j, cx as u16);
        m.w16(s + 0x60 + 4 * j, cy as u16);
    }
}

/// Pasos máximos de [`find_outer_entry_cell`]; el original no termina si el camino no vuelve a una casilla de paso.
pub const OUTER_ENTRY_STEPS: u32 = 1 << 16;

/// `town_siege_find_outer_entry_cell` (`0x00630640`): desde la casilla del primer nodo del camino de la clave 3
/// avanza en su rumbo mientras se puede pasar, luego hasta volver a una casilla de paso, y la guarda en
/// `+0x3E/+0x40` (destino de la rendición). Devuelve `false` (sin escribir) si supera [`OUTER_ENTRY_STEPS`].
pub fn find_outer_entry_cell(m: &mut dyn Mem, h: &mut dyn SiegeViewHost, s: u32) -> bool {
    let nodes = path_nodes(m, 3, 0);
    let (x0, y0) = node(m, nodes, 0);
    let (x1, y1) = node(m, nodes, 1);
    let dir = path_heading(x0, y0, x1, y1);
    let (mut cx, mut cy) = h.pixel_to_cell(m, x0, y0);
    let mut steps = 0;
    while is_cell_passable(m, h, s, cx, cy, false) {
        h.step(m, &mut cx, &mut cy, dir);
        steps += 1;
        if steps > OUTER_ENTRY_STEPS {
            return false;
        }
    }
    if !is_cell_passable(m, h, s, cx, cy, false) {
        loop {
            h.step(m, &mut cx, &mut cy, dir);
            steps += 1;
            if steps > OUTER_ENTRY_STEPS {
                return false;
            }
            if is_cell_passable(m, h, s, cx, cy, false) {
                break;
            }
        }
    }
    m.w16(s + 0x3E, cx as u16);
    m.w16(s + 0x40, cy as u16);
    true
}

/// `town_siege_compute_gate_cells` (`0x00632780`): casillas de la puerta elegida (`+0x6D`, registro de edificio
/// `+0x244 + 4 * puerta` de la ciudad `+0x5D3`): objetivo del ataque `+8/+0xA`, casillas `+0xC/+0xE`,
/// `+0x5A2..+0x5A8`, casillas exentas de la prueba de paso `+0x5CA..+0x5D0` y rumbo `+0x5D2`, según el lado de la
/// puerta (`registro+3`: `0x34/0x37` o `0x35/0x38`) y el plano de la ciudad (`+0x7AC` paso, `+0x80C` celdas).
/// Una puerta fuera de la tabla de edificios no cambia nada.
pub fn compute_gate_cells(m: &mut dyn Mem, s: u32) {
    let city = u32::from(m.r8(s + 0x5D3));
    let town = m.r32(TOWNS).wrapping_add(city.wrapping_mul(TOWN_SIZE));
    let slot = u32::from(m.r8(s + 0x6D));
    let gate = m.r32(s.wrapping_add(slot * 4).wrapping_add(0x244));
    if gate >= u32::from(m.r16(town + 0x776)) {
        return;
    }
    let rec = m.r32(town + 0x75C).wrapping_add(gate * 8);
    let (gx, gy, side) = (u16::from(m.r8(rec)), u16::from(m.r8(rec + 1)), m.r8(rec + 3));
    let a = matches!(side, 0x37 | 0x34);
    let b = matches!(side, 0x38 | 0x35);
    let y3 = gy.wrapping_sub(3);
    m.w16(s + 0xA, y3);
    let x = if a && y3 & 1 == 0 {
        gx.wrapping_add(1)
    } else if b && y3 & 1 == 1 {
        gx.wrapping_sub(1)
    } else {
        gx
    };
    m.w16(s + 8, x);
    m.w16(s + 0x5A2, if a { gx.wrapping_add(2) } else { gx.wrapping_sub(2) });
    m.w16(s + 0x5A4, gy.wrapping_sub(4));
    m.w16(s + 0x5A6, gx);
    m.w16(s + 0x5A8, gy);
    m.w16(s + 0xC, gx);
    let e = gy.wrapping_sub(4);
    m.w16(s + 0xE, e);
    let stride = m.r32(town + 0x7AC);
    let plan = m.r32(town + 0x80C);
    let cell = m.r8(stride.wrapping_mul(u32::from(gx)).wrapping_add(plan).wrapping_add(u32::from(e)));
    if cell & 0x80 == 0 {
        let c = m.r16(s + 8);
        m.w16(s + 0xE, e.wrapping_add(3));
        m.w16(s + 0xC, if a { c.wrapping_add(1) } else { c.wrapping_sub(1) });
        let (dir, p, q) = if a { (0x60u8, c.wrapping_sub(1), c.wrapping_add(1)) } else { (0xA0, c.wrapping_add(1), c.wrapping_sub(1)) };
        m.w8(s + 0x5D2, dir);
        let ya = m.r16(s + 0xA);
        m.w16(s + 0x5CC, q);
        m.w16(s + 0x5CA, p);
        m.w16(s + 0x5CE, ya);
        m.w16(s + 0x5D0, ya.wrapping_sub(4));
    } else {
        if a {
            m.w8(s + 0x5D2, 0xE0);
            m.w16(s + 0x5CA, gx.wrapping_add(1));
            m.w16(s + 0x5CC, gx.wrapping_add(3));
        } else {
            m.w8(s + 0x5D2, 0x20);
            m.w16(s + 0x5CA, gx.wrapping_sub(1));
            m.w16(s + 0x5CC, gx.wrapping_sub(3));
        }
        m.w16(s + 0x5CE, gy);
        m.w16(s + 0x5D0, gy.wrapping_sub(4));
    }
}

/// Fase de `town_siege_rebuild_view_state` (`0x0062C730`) hasta `0x0062C90D`, al abrir o reconstruir la vista.
///
/// Escribe los desplazamientos por rumbo de la rejilla (`+0x630`). Si el asedio no está a la vista (`+0x3A` no
/// supera `+0x6A`, o sin espera y `+0x21E == 0`) desmonta la vista: libera cola, campo y `+0x250`, retira los
/// marcadores y pone a cero `+0x77`, `+0x79` y `+0x21E..`; devuelve `false`. Si está a la vista marca el asalto
/// (`+0x7A`, `+0x618`), hace huir a los transeúntes, elige camino, puntos de reunión, carriles y entrada exterior,
/// reserva la máscara `+0x70`, calcula las casillas de la puerta y crea el jefe del asalto (clase `0x16`) en
/// `+0x574`; devuelve `true`: el original sigue creando asaltantes, defensores y milicia (sin portar).
pub fn rebuild_view_state_prefix(m: &mut dyn Mem, h: &mut dyn SiegeViewHost, s: u32) -> bool {
    for (k, dy) in RING_DY.iter().enumerate() {
        let w = width(m);
        m.w32(s + 0x630 + 4 * k as u32, w.wrapping_mul(*dy as u32));
    }
    let wait = m.r16(s + 0x3A);
    let visible = if wait != 0 { wait > m.r16(s + 0x6A) } else { m.r16(s + 0x21E) > 0 };
    if !visible {
        let queue = m.r32(s + 0x5D8);
        if queue != 0 {
            h.free(m, queue);
        }
        let flow = m.r32(s + 0x59C);
        m.w32(s + 0x5D8, 0);
        m.w16(s + 0x590, 0);
        m.w16(s + 0x592, 0);
        if flow != 0 {
            h.free(m, flow);
        }
        let other = m.r32(s + 0x250);
        m.w32(s + 0x59C, 0);
        m.w8(s + 0x77, 0);
        if other != 0 {
            h.free(m, other);
        }
        m.w32(s + 0x250, 0);
        for k in 0..4 {
            m.w16(s + 0x21E + 4 * k, 0);
        }
        m.w8(s + 0x79, 0);
        for k in 0..8 {
            let slot = s + 0x5DC + 4 * k;
            let entity = m.r32(slot);
            if entity != 0 {
                if view(m) != 0 {
                    h.release_entity(m, entity);
                }
                m.w32(slot, 0);
            }
        }
        return false;
    }
    m.w8(s + 0x7A, 1);
    let tick = m.r32(0x006D_E4B4);
    m.w32(s + 0x618, tick);
    h.make_walkers_flee(m);
    select_approach_route(m, h, s);
    pick_rally_points(m, h, s);
    compute_approach_lanes(m, h, s);
    find_outer_entry_cell(m, h, s);
    while m.r32(s + 0x70) == 0 {
        let n = height(m).wrapping_mul(width(m));
        let p = h.alloc(m, n);
        m.w32(s + 0x70, p);
        if p & 0xFFFF_FF00 == 0 {
            m.w32(s + 0x70, 0);
            continue;
        }
        let n = height(m).wrapping_mul(width(m));
        for i in 0..n {
            m.w8(p.wrapping_add(i), 0);
        }
    }
    if m.r16(s + 8) != 0xFFFF {
        compute_gate_cells(m, s);
        if m.r16(s + 0x3A) == 0 && m.r32(s + 4) != 0 {
            let id = (u32::from(m.r8(s + 0x76)) << 8).wrapping_add(u32::from(m.r8(s + 0x5D3)));
            let leader = h.spawn_entity(m, [0, 0, 0x16, id, 0x20, 0, 0]);
            m.w32(s + 0x574, leader);
        }
    }
    true
}
