//! Caminar libre de las figuras (J02): funciones pequeñas de `0x0055C820` con paridad x86 (oráculo `oracle/x86_walk_small.py`).

/// `0x0055D910`: rumbo hacia el destino `(tx, ty)` desde `(x, y)` (píxeles, `i16`). Devuelve `(rumbo, auxiliar)` con el rumbo del eje
/// principal en el segundo valor (`+0x3D`): se prefiere el movimiento diagonal cuando el desplazamiento vertical (por 0x11) supera al horizontal (por 0xB).
pub fn direction_to(x: i16, y: i16, tx: u16, ty: u16) -> (u8, u8) {
    let (x, y) = (i32::from(x), i32::from(y));
    let (tx, ty) = (i32::from(tx), i32::from(ty));
    if tx <= x {
        let dx = (x - tx).wrapping_mul(0xB);
        if y < ty {
            (0x60u8.wrapping_add((((dx <= (ty - y).wrapping_mul(0x11)) as u32).wrapping_sub(1) & 0x80) as u8), 0xA0)
        } else {
            (0x20u8.wrapping_add((((dx <= (y - ty).wrapping_mul(0x11)) as u32).wrapping_sub(1) & 0x80) as u8), 0xE0)
        }
    } else {
        let dx = (tx - x).wrapping_mul(0xB);
        if y < ty {
            (0xA0u8.wrapping_add((((dx <= (ty - y).wrapping_mul(0x11)) as u32).wrapping_sub(1) & 0x80) as u8), 0x60)
        } else {
            (((((dx <= (y - ty).wrapping_mul(0x11)) as u32).wrapping_sub(1) & 0x80) as u8).wrapping_sub(0x20), 0x20)
        }
    }
}

/// `0x0055A1C0`: gira un paso (`0x100 / n`) hacia `target`; el rumbo `0x00` salta a `0xE0` y viceversa (tabla de direcciones `0x677928..2F`).
pub fn turn_step(heading: u8, target: u8, n: u8) -> u8 {
    let step = (0x100u32 / u32::from(n)) as u8;
    if heading.wrapping_sub(target) <= target.wrapping_sub(heading) {
        if heading == 0x00 { 0xE0 } else { heading.wrapping_sub(step) }
    } else if heading == 0xE0 {
        0x00
    } else {
        heading.wrapping_add(step)
    }
}

/// `0x0055A290`: bytes `+4..+7` de la orden de movimiento.
pub fn movement_order(a: u8, b: u8, c: u8, d: u8) -> [u8; 4] {
    [b, c, d, a >> 1]
}

/// Lo que el caminar lee de la escena (`Class22`, objetos, plano y entidades por casilla).
pub trait WalkScene {
    /// Columnas y filas de la rejilla (`+0xC314`, `+0xC318`).
    fn size(&self) -> (u32, u32);
    /// Clase de la ficha de suelo (capa 0) y del objeto (capa 1, `None` si no hay), método virtual `+0x50`.
    fn kind0(&self, x: u32, y: u32) -> u32;
    fn kind1(&self, x: u32, y: u32) -> Option<u32>;
    /// Bit `0x80` (calle) de la celda del plano `stride * x + y`.
    fn street(&self, x: u32, y: u32) -> bool;
    /// Entidades registradas en la casilla: `(enlace +0x16, clase +0x20)` en el orden de la cadena.
    fn entities(&self, x: u32, y: u32) -> Vec<(u16, u32)>;
}

/// Orden de movimiento de la figura (`0x55A290`): bytes `+4`, `+5`, `+6` y el `+8` que fija `0x55A2C0`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Order {
    pub b4: u8,
    pub b5: u8,
    pub b6: u8,
    pub b8: u8,
}

/// `0x0055AD90`: ¿puede la figura entrar en la casilla `(x, y)`? `link` es el enlace de la figura (`+0x16`); `check_objects` (`param_4`) activa
/// las reglas de objeto y suelo; `check_entities` (`param_5`) las de entidades de la casilla. Fuera de la rejilla devuelve `true`.
pub fn can_enter(sc: &dyn WalkScene, o: &Order, cell: (u32, u32), link: u16, check_objects: bool, check_entities: bool) -> bool {
    let (x, y) = cell;
    let (cols, rows) = sc.size();
    if x >= cols || y >= rows {
        return true;
    }
    let k0 = sc.kind0(x, y);
    if !check_objects {
        return k0 == 0xE;
    }
    if let Some(k1) = sc.kind1(x, y) {
        match k1 {
            0x2F if o.b6 == 0 || o.b4 == 0 || k0 != 0x18 => return false,
            0x2F | 0x30 | 0x31 => {}
            _ => return false,
        }
    }
    if check_entities {
        for (l, kind) in sc.entities(x, y) {
            if l != link {
                if o.b6 == 0 || kind & 0xFF != 0x1F {
                    return false;
                }
            }
        }
    }
    let street = sc.street(x, y);
    match k0 {
        0x10 | 0x17 | 0x18 => {
            if o.b6 == 0 {
                if street {
                    if o.b5 != 0 {
                        return false;
                    }
                } else if o.b5 == 0 {
                    return false;
                }
            }
            true
        }
        0x28 => {
            if street {
                if o.b5 != 0 {
                    return false;
                }
            } else if o.b5 == 0 {
                return false;
            }
            o.b8 == 0
        }
        _ => false,
    }
}

use crate::geom::Geom;

/// Posición de la figura que lee `0x55A2C0`: X e Y en 16.16 (`+0xC`, `+0x10` de la entidad) y su enlace de casillas (`+0x16`).
#[derive(Clone, Copy, Debug)]
pub struct Pos {
    pub x: u32,
    pub y: u32,
    pub link: u16,
}

/// `0x0055A2C0` (sin el bloque de carriles del puerto, que solo actúa con una ciudad portuaria con barcos): ¿puede la figura dar un paso
/// en el rumbo `dir`? Escribe `order.b8` (casilla de destino transitable según las reglas de objeto/suelo) y devuelve si el paso es posible.
/// El paso se descompone en una o dos pasadas (dos si `order.b7 != 0`); cada una prueba las casillas vecinas por las que cruzaría.
pub fn try_step(sc: &dyn WalkScene, geom: &Geom, mask: &[u8], order: &mut Order, b7: u8, pos: Pos, dir: u8, check_objects: bool, check_entities: bool) -> bool {
    let mut passes = 1u8;
    let mut lx = pos.x >> 16;
    let mut ly = pos.y >> 16;
    let s = u32::from(b7);
    let si = s as i32;
    let div = |a: i32, b: i32| a / b;
    let mut skip_offset = b7 == 0;
    if !skip_offset {
        passes = 2;
        if (0x10..0xF0).contains(&dir) {
            if dir < 0x30 {
                lx = lx.wrapping_add(div(si * -0x11, 0x14) as u32);
                ly = ly.wrapping_add(div(si * -0xB, 0x14) as u32);
            } else if dir < 0x50 {
                ly = ly.wrapping_add(div(si * -0xB, 0x11) as u32);
            } else if dir >= 0x70 {
                if dir >= 0x90 {
                    if dir < 0xB0 {
                        lx = lx.wrapping_add(div(si * 0x11, 0x14) as u32);
                        ly = ly.wrapping_add(div(si * 0xB, 0x14) as u32);
                    } else if dir < 0xD0 {
                        ly = ly.wrapping_add(div(si * 0xB, 0x11) as u32);
                    } else {
                        lx = lx.wrapping_add(div(si * -0x11, 0x14) as u32);
                        ly = ly.wrapping_add(div(si * 0xB, 0x14) as u32);
                    }
                } else {
                    lx = lx.wrapping_add(s);
                }
            } else {
                lx = lx.wrapping_add(div(si * 0x11, 0x14) as u32);
                ly = ly.wrapping_add(div(si * -0xB, 0x14) as u32);
            }
        } else {
            lx = lx.wrapping_add(s.wrapping_neg());
        }
        skip_offset = false;
    }
    let _ = skip_offset;
    loop {
        let ((mut col, mut row), _) = geom.pixel_to_cell(lx as i32, ly as i32, mask);
        if order.b4 == 0 {
            order.b8 = 0;
        } else {
            order.b8 = 1;
            if !can_enter(sc, order, (col, row), pos.link, check_objects, false) {
                order.b8 = 0;
            }
        }
        macro_rules! go {
            ($d:expr) => {{
                geom.step(&mut col, &mut row, $d);
                if !can_enter(sc, order, (col, row), pos.link, check_objects, check_entities) {
                    return false;
                }
            }};
        }
        let east_first = |a: u32, b: u32| a < b;
        if dir >= 0xF0 || dir < 0x10 {
            let (px, _) = geom.cell_to_pixel(col as i32, row);
            if east_first(lx, px as u32) {
                go!(0xE0);
                go!(0x20);
            } else {
                go!(0x20);
                go!(0xE0);
            }
            lx = lx.wrapping_add(s.wrapping_mul(2));
        } else if dir < 0x30 {
            go!(0x20);
            lx = lx.wrapping_add(div(si * 0x11, 10) as u32);
            ly = ly.wrapping_add(div(si * 0xB, 10) as u32);
        } else if dir < 0x50 {
            let (_, py) = geom.cell_to_pixel(col as i32, row);
            if east_first(ly, py as u32) {
                go!(0x20);
                go!(0x60);
            } else {
                go!(0x60);
                go!(0x20);
            }
            ly = ly.wrapping_add((s * 0x16 / 0x11) as u32);
        } else if dir < 0x70 {
            go!(0x60);
            lx = lx.wrapping_add(div(si * -0x11, 10) as u32);
            ly = ly.wrapping_add(div(si * 0xB, 10) as u32);
        } else if dir < 0x90 {
            let (px, _) = geom.cell_to_pixel(col as i32, row);
            if east_first(px as u32, lx) {
                go!(0x60);
                go!(0xA0);
            } else {
                go!(0xA0);
                go!(0x60);
            }
            lx = lx.wrapping_add((s as i32).wrapping_mul(-2) as u32);
        } else if dir < 0xB0 {
            go!(0xA0);
            lx = lx.wrapping_add(div(si * -0x11, 10) as u32);
            ly = ly.wrapping_add(div(si * -0xB, 10) as u32);
        } else if dir < 0xD0 {
            let (_, py) = geom.cell_to_pixel(col as i32, row);
            if east_first(py as u32, ly) {
                go!(0xA0);
                go!(0xE0);
            } else {
                go!(0xE0);
                go!(0xA0);
            }
            ly = ly.wrapping_add(div(si * -0x16, 0x11) as u32);
        } else {
            go!(0xE0);
            lx = lx.wrapping_add(div(si * 0x11, 10) as u32);
            ly = ly.wrapping_add(div(si * -0xB, 10) as u32);
        }
        passes -= 1;
        if passes == 0 {
            return true;
        }
    }
}

/// Indicadores del caminante que lee `0x55D700` (`+0x24`, `+0x26`, `+0x28`).
#[derive(Clone, Copy, Debug, Default)]
pub struct WalkFlags {
    pub f24: u8,
    pub f26: u8,
    pub f28: u8,
}

/// `0x0055D700`: la única entidad «de paso» en la casilla (posición en la cadena de `sc.entities`), o `None` si no hay, hay dos, o la casilla no es
/// transitable para el caminante. Las clases 0-4 y 7 cuentan; la 8 solo si no hay partida con la hanse (`0x633AE0`); cualquier otra anula el resultado.
/// Las entidades con el mismo enlace que la figura no cuentan.
pub fn neighbour(sc: &dyn WalkScene, f: WalkFlags, cell: (u32, u32), link: u16, hanse: bool) -> Option<usize> {
    let (x, y) = cell;
    let k0 = sc.kind0(x, y);
    match k0 {
        0x10 | 0x17 | 0x18 => {
            if f.f26 == 0 && !sc.street(x, y) {
                return None;
            }
        }
        0x28 => {
            if f.f28 != 0 {
                return None;
            }
        }
        _ => return None,
    }
    if let Some(k1) = sc.kind1(x, y) {
        match k1 {
            0x2F if f.f26 == 0 || f.f24 == 0 || k0 != 0x18 => return None,
            0x2F | 0x30 | 0x31 => {}
            _ => return None,
        }
    }
    let mut found = None;
    for (i, (l, kind)) in sc.entities(x, y).into_iter().enumerate() {
        if l == link {
            continue;
        }
        match kind & 0xFF {
            8 if hanse => return None,
            0..=4 | 7 | 8 => {
                if found.is_some() {
                    return None;
                }
                found = Some(i);
            }
            _ => return None,
        }
    }
    found
}

/// Resultado de `0x0055CF60`: el vector de avance hacia el centro de una casilla en pasos de `0x10000/pasos` píxeles.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Advance {
    /// `+0x54`: hay una entidad en la casilla y se rodea.
    pub avoiding: bool,
    /// `+0x38` y `+0x3A`: paso actual (0) y número de pasos.
    pub step: u16,
    pub steps: u16,
    /// `+0x40`, `+0x44`: avance por paso en 16.16 (se conservan los valores previos si el número de pasos es 0).
    pub vx: i32,
    pub vy: i32,
}

/// `0x0055CF60`: parte de la posición `(px, py)` (`+0xE`, `+0x12`), la casilla de destino y la rapidez del conjunto de sprites (`+0xC`).
/// Con una entidad en la casilla el punto de destino se desplaza ±3 y ±4 px para rodearla. `prev` son los `+0x40`/`+0x44` anteriores.
pub fn advance_vector(sc: &dyn WalkScene, geom: &Geom, f: WalkFlags, hanse: bool, link: u16, pos: (i16, i16), cell: (u32, u32), speed: u32, prev: (i32, i32)) -> Advance {
    let (mut tx, mut ty) = geom.cell_to_pixel(cell.0 as i32, cell.1);
    let (tx_u, ty_u) = (&mut tx, &mut ty);
    let avoiding = neighbour(sc, f, cell, link, hanse).is_some();
    let px = i32::from(pos.0) as u32;
    let py = i32::from(pos.1) as u32;
    if avoiding {
        let mut yy = *ty_u as u32;
        yy = if (*tx_u as u32) < px { yy.wrapping_sub(3) } else { yy.wrapping_add(3) };
        if yy < py {
            *tx_u = (*tx_u as u32).wrapping_add(4) as i32;
        } else {
            *tx_u = (*tx_u as u32).wrapping_sub(4) as i32;
        }
        *ty_u = yy as i32;
    }
    let dx = (*tx_u as u32).wrapping_sub(px) as i32;
    let dy = (*ty_u as u32).wrapping_sub(py) as i32;
    let d2 = dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx));
    let t = f64::from(d2).sqrt() * 10000.0 / f64::from(speed) + 0.5;
    // `__ftol` (0x639C28) trunca a 64 bits y se queda con la mitad baja: lo no representable da 0.
    let steps = if t.is_finite() && t.abs() < 9.2e18 { (t as i64) as i32 } else { 0 };
    let (mut vx, mut vy) = (dx, dy);
    if steps != 0 {
        vx = (dx.wrapping_shl(16)).wrapping_div(steps);
        vy = (dy.wrapping_shl(16)).wrapping_div(steps);
    }
    let _ = prev;
    Advance { avoiding, step: 0, steps: steps as u16, vx, vy }
}

/// Datos de la ciudad que lee `0x55D9D0`.
pub trait TargetEnv {
    /// `Town+0x2C8` (indicadores) y `Town+0x2D0` (instante de referencia).
    fn flags(&self) -> u32;
    fn ref_time(&self) -> u32;
    /// `Town+0x784`: primer índice de la cadena de ciudades vecinas.
    fn chain_start(&self) -> u16;
    /// `Town+0x764` y `Town+0x776`: primer registro de edificio y número de registros.
    fn first_building(&self) -> u16;
    fn building_count(&self) -> u16;
    /// Registro de 8 bytes del edificio `i` (`0x51F1F0`): `[x, y, _, tipo, siguiente (u16 LE), ..]`.
    fn building(&self, i: u16) -> Option<[u8; 8]>;
    /// Los ocho puntos fijos `(x, y)` de `Town+0x7D0`, `0xFF` si faltan.
    fn points(&self) -> [(u8, u8); 8];
    /// Celda del plano con la calle (`bit 7`) en `(x, y)`.
    fn street(&self, x: u32, y: u32) -> bool;
    /// Instante del evento de tipo 7 de la ciudad (`0x4E2314`).
    fn event_time(&self) -> Option<u32>;
    /// Reloj `0x6DE4B4`, número de ciudades `0x6DE4A8` y enlaces de la tabla del mundo: `(+0x2CA, +0x2CE)` de la ciudad `i` (`0x530400`).
    fn clock(&self) -> u32;
    fn world_count(&self) -> u16;
    fn world_town(&self, i: u16) -> Option<(u16, u16)>;
}

/// Estado del caminante que modifica `0x55D9D0`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TargetState {
    /// Posición actual `(+0xE, +0x12)` y destino `(+0x50, +0x52)`.
    pub pos: (i16, i16),
    pub target: (u16, u16),
    /// `+0x4C`.
    pub timer: u32,
    /// `+0x55`, `+0x56`, `+0x57`.
    pub b55: u8,
    pub b56: u8,
    pub b57: u8,
    /// ¿Es un mendigo (clase 4, método virtual `+0x20`)?
    pub beggar: bool,
}

/// `0x0055D9D0`: elige el siguiente destino del caminante entre los puntos fijos y las puertas de los edificios de la ciudad (y de las vecinas),
/// el más lejano dentro de la ventana de `0x353 x 0x227` o, si hay más de uno fuera de ella, uno al azar. Con una fiesta cercana (`bVar4`) solo
/// usa los puntos fijos. Devuelve los `rand` consumidos en `rands`.
pub fn choose_target(env: &dyn TargetEnv, geom: &Geom, st: &mut TargetState, rand: &mut dyn FnMut() -> u32) {
    let clock = env.clock();
    let event = env.event_time().unwrap_or(0xFFFF_FFFF);
    let near = clock.wrapping_sub(env.ref_time()) < 0x300 || event.wrapping_sub(clock) < 0x300;
    let (px, py) = (i32::from(st.pos.0), i32::from(st.pos.1));
    let left_of = (i32::from(st.target.0)) < px;
    let above = (i32::from(st.target.1)) < py;
    let mut count: u16 = 0;
    let mut best: u16 = 0;
    if st.beggar {
        if env.flags() & 0x80_0000 != 0 {
            st.timer = clock;
        }
        if clock.wrapping_sub(st.timer) < 0x300 {
            let mut i = env.first_building();
            while i < env.building_count() {
                let Some(r) = env.building(i) else { break };
                if r[3] == 0x1F {
                    let (tx, ty) = geom.cell_to_pixel(i32::from(r[0]), u32::from(r[1]).wrapping_sub(4));
                    st.target = (tx as u16, ty as u16);
                    st.b55 = 3;
                    st.b56 = 0;
                    st.b57 = 0;
                    return;
                }
                i = u16::from_le_bytes([r[4], r[5]]);
            }
        }
    }
    let same_spot = |st: &TargetState| i32::from(st.target.0) == px && i32::from(st.target.1) == py;
    let mut consider = |st: &mut TargetState, tx: i32, ty: i32, count: &mut u16, best: &mut u16, rand: &mut dyn FnMut() -> u32| {
        let dx = tx.wrapping_sub(px).wrapping_abs();
        let dy = ty.wrapping_sub(py).wrapping_abs();
        if dx < 0x353 && dy < 0x227 {
            if *count == 0 {
                let mut d = (dy.wrapping_mul(0x11) / 0xB) as u32 & 0xFFFF;
                if (d as i32) < dx {
                    d = dx as u32;
                }
                if *best < d as u16 {
                    st.target = (tx as u16, ty as u16);
                    *best = d as u16;
                }
            }
        } else {
            *count = count.wrapping_add(1);
            let n = i32::from(*count);
            if (rand() as i32) % n == 0 {
                st.target = (tx as u16, ty as u16);
            }
        }
    };
    for (a, b) in env.points() {
        if a == 0xFF || b == 0xFF {
            continue;
        }
        let (tx, ty) = geom.cell_to_pixel(i32::from(a), u32::from(b));
        let moved = (tx as u32) < px as u32;
        let moved_y = (ty as u32) < py as u32;
        if near || same_spot(st) || (moved != left_of && moved_y != above) {
            consider(st, tx, ty, &mut count, &mut best, rand);
        }
    }
    if !near {
        let mut i = env.first_building();
        let mut town = env.chain_start();
        loop {
            if i < env.building_count() {
                if let Some(r) = env.building(i) {
                    if r[3] < 0x2F {
                        let (cx, cy) = (u32::from(r[0]), u32::from(r[1]).wrapping_sub(2));
                        if env.street(cx, cy) {
                            let (tx, ty) = geom.cell_to_pixel(cx as i32, cy);
                            let moved = (tx as u32) < px as u32;
                            let moved_y = (ty as u32) < py as u32;
                            if same_spot(st) || (moved != left_of && moved_y != above) {
                                consider(st, tx, ty, &mut count, &mut best, rand);
                            }
                        }
                    }
                    i = u16::from_le_bytes([r[4], r[5]]);
                    continue;
                }
            }
            if env.world_count() <= town {
                break;
            }
            let Some((next, first)) = env.world_town(town) else { break };
            town = next;
            i = first;
        }
    }
    if count == 0 && best == 0 {
        st.b55 = 3;
        st.b56 = 0;
        st.b57 = 0;
    }
}

// ---------------------------------------------------------------------------------------------------------------------------------------
// Bucle del caminante libre `0x0055C820`.
// ---------------------------------------------------------------------------------------------------------------------------------------
use crate::mover::{MoverAnim, SetInfo};

/// La figura libre (`mover`): los primeros `0x70` bytes de la entidad. Los bytes que no modela se conservan en `raw`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FreeWalker {
    pub raw: [u8; 0x70],
    /// `+0xC`, `+0x10`: posición 16.16; `+0xE`, `+0x12`: píxel entero.
    pub x: u32,
    pub y: u32,
    pub px: i16,
    pub py: i16,
    /// `+0x14` rumbo, `+0x15` fotograma, `+0x16` enlace de casillas.
    pub heading: u8,
    pub frame: u8,
    pub link: u16,
    /// `+0x24..+0x28`: orden de movimiento; `+0x27` es `b7`.
    pub order: Order,
    pub b7: u8,
    /// `+0x30`, `+0x34`, `+0x35`: animación.
    pub anim: MoverAnim,
    /// `+0x38`, `+0x3A`: paso actual y número de pasos; `+0x3C`, `+0x3D`: rumbo deseado y auxiliar; `+0x40`, `+0x44`: vector; `+0x48`, `+0x4C`.
    pub step: u16,
    pub steps: u16,
    pub want: u8,
    pub aux: u8,
    pub vx: i32,
    pub vy: i32,
    pub v48: u32,
    pub timer: u32,
    /// `+0x50`, `+0x52`: destino.
    pub target: (u16, u16),
    /// `+0x54..+0x57`.
    pub f54: u8,
    pub f55: u8,
    pub f56: u8,
    pub f57: u8,
}

impl FreeWalker {
    pub fn from_bytes(b: &[u8]) -> Self {
        let mut raw = [0u8; 0x70];
        raw.copy_from_slice(&b[..0x70]);
        let u16_at = |o: usize| u16::from_le_bytes([raw[o], raw[o + 1]]);
        let u32_at = |o: usize| u32::from_le_bytes([raw[o], raw[o + 1], raw[o + 2], raw[o + 3]]);
        FreeWalker {
            x: u32_at(0xC),
            y: u32_at(0x10),
            px: u16_at(0xE) as i16,
            py: u16_at(0x12) as i16,
            heading: raw[0x14],
            frame: raw[0x15],
            link: u16_at(0x16),
            order: Order { b4: raw[0x24], b5: raw[0x25], b6: raw[0x26], b8: raw[0x28] },
            b7: raw[0x27],
            anim: MoverAnim { next: u32_at(0x30), state: raw[0x34], target: raw[0x35] },
            step: u16_at(0x38),
            steps: u16_at(0x3A),
            want: raw[0x3C],
            aux: raw[0x3D],
            vx: u32_at(0x40) as i32,
            vy: u32_at(0x44) as i32,
            v48: u32_at(0x48),
            timer: u32_at(0x4C),
            target: (u16_at(0x50), u16_at(0x52)),
            f54: raw[0x54],
            f55: raw[0x55],
            f56: raw[0x56],
            f57: raw[0x57],
            raw,
        }
    }

    pub fn to_bytes(&self) -> [u8; 0x70] {
        let mut r = self.raw;
        let mut p16 = |o: usize, v: u16| r[o..o + 2].copy_from_slice(&v.to_le_bytes());
        p16(0xE, self.px as u16);
        p16(0x12, self.py as u16);
        p16(0x16, self.link);
        p16(0x38, self.step);
        p16(0x3A, self.steps);
        p16(0x50, self.target.0);
        p16(0x52, self.target.1);
        let mut p32 = |o: usize, v: u32| r[o..o + 4].copy_from_slice(&v.to_le_bytes());
        p32(0xC, self.x);
        p32(0x10, self.y);
        p32(0x30, self.anim.next);
        p32(0x40, self.vx as u32);
        p32(0x44, self.vy as u32);
        p32(0x48, self.v48);
        p32(0x4C, self.timer);
        r[0x14] = self.heading;
        r[0x15] = self.frame;
        r[0x24] = self.order.b4;
        r[0x25] = self.order.b5;
        r[0x26] = self.order.b6;
        r[0x27] = self.b7;
        r[0x28] = self.order.b8;
        r[0x34] = self.anim.state;
        r[0x35] = self.anim.target;
        r[0x3C] = self.want;
        r[0x3D] = self.aux;
        r[0x54] = self.f54;
        r[0x55] = self.f55;
        r[0x56] = self.f56;
        r[0x57] = self.f57;
        r
    }

    fn flags(&self) -> WalkFlags {
        WalkFlags { f24: self.order.b4, f26: self.order.b6, f28: self.order.b8 }
    }
    fn pos(&self) -> Pos {
        Pos { x: self.x, y: self.y, link: self.link }
    }
}

/// Lo que el bucle consulta o provoca fuera del caminante.
pub trait LoopWorld {
    fn scene(&self) -> &dyn WalkScene;
    fn target_env(&self) -> &dyn TargetEnv;
    fn geom(&self) -> &Geom;
    fn mask(&self) -> &[u8];
    /// Conjunto de sprites de la figura y su rapidez (`+0xC`).
    fn set(&self) -> &SetInfo;
    fn speed(&self) -> u32;
    /// Presupuesto de movimiento (`escena+0x414`) y reloj de animación.
    fn budget(&self) -> u32;
    fn clock(&self) -> u32;
    /// `0x633AE0`.
    fn hanse(&self) -> bool;
    /// Clase de la entidad (método virtual `+0x20`; 4 = mendigo).
    fn entity_class(&self) -> u8;
    /// `(rumbo +0x14, animación +0x34)` de la `idx`-ésima entidad de la casilla.
    fn entity_fields(&self, cell: (u32, u32), idx: usize) -> (u8, u8);
    fn rand(&mut self) -> u32;
    /// `0x55D9D0` con el entorno de la ciudad de la escena y el `rand` del mundo.
    fn choose_target(&mut self, st: &mut TargetState);
    /// Byte `+8` del conjunto de sprites (indicador que recibe `0x593580`).
    fn set_kind(&self) -> u8;
    /// `0x5593C0`: ¿se ve la figura en pantalla?
    fn visible(&mut self) -> bool;
    /// `0x5595F0` (invalidar casillas), `0x559790` (registrar), `0x55D0B0` (efectos de pisadas).
    fn invalidate(&mut self);
    fn register(&mut self);
    fn effects(&mut self, m: &FreeWalker);
    /// Método virtual «animación terminada»: elige la animación siguiente.
    fn anim_done(&mut self, a: &mut MoverAnim);
}

fn frame_of(w: &mut dyn LoopWorld, m: &mut FreeWalker) -> u8 {
    let clock = w.clock();
    let set = w.set().clone();
    let mut cb = |a: &mut MoverAnim| w.anim_done(a);
    m.anim.advance(&set, m.heading, m.frame, None, clock, &mut cb)
}

/// `0x00593580` aplicado a la casilla `cell`: libre de objetos, con suelo transitable y sin entidades de otro enlace.
fn cell_free(w: &dyn LoopWorld, cell: (u32, u32), link: u16) -> bool {
    let sc = w.scene();
    let k1 = sc.kind1(cell.0, cell.1);
    if k1.is_some_and(|k| !(0x30..0x32).contains(&k)) {
        return false;
    }
    if sc.entities(cell.0, cell.1).iter().any(|&(l, _)| l != link) {
        return false;
    }
    crate::walkers::spawn_cell_free(None, sc.kind0(cell.0, cell.1), w.set_kind())
}

fn try_move(w: &dyn LoopWorld, m: &mut FreeWalker, dir: u8) -> bool {
    let mut order = m.order;
    let ok = try_step(w.scene(), w.geom(), w.mask(), &mut order, m.b7, m.pos(), dir, true, true);
    m.order.b8 = order.b8;
    ok
}

fn run_choose_target(w: &mut dyn LoopWorld, m: &mut FreeWalker) {
    let mut st = TargetState { pos: (m.px, m.py), target: m.target, timer: m.timer, b55: m.f55, b56: m.f56, b57: m.f57, beggar: w.entity_class() == 4 };
    w.choose_target(&mut st);
    m.target = st.target;
    m.timer = st.timer;
    m.f55 = st.b55;
    m.f56 = st.b56;
    m.f57 = st.b57;
}

fn apply_advance(w: &dyn LoopWorld, m: &mut FreeWalker, cell: (u32, u32)) {
    let a = advance_vector(w.scene(), w.geom(), m.flags(), w.hanse(), m.link, (m.px, m.py), cell, w.speed(), (m.vx, m.vy));
    m.f54 = a.avoiding as u8;
    m.step = a.step;
    m.steps = a.steps;
    m.vx = a.vx;
    m.vy = a.vy;
}

/// `0x0055C820`: un paso de caminar libre de la figura. Devuelve el valor de retorno del original (0: la figura no se ve y debe retirarse).
/// No modela el bloque de rapidez doble de las ciudades con puerto (`DAT_006DF270`, presupuesto x2 e intervalo /2).
pub fn free_walk_step(w: &mut dyn LoopWorld, m: &mut FreeWalker) -> u8 {
    let budget = w.budget();
    let mut frame = frame_of(w, m);
    if frame != m.frame {
        w.effects(m);
    }
    if m.heading != m.want {
        return turn_step_loop(w, m, frame);
    }
    let ((cx, cy), _) = w.geom().pixel_to_cell(i32::from(m.px), i32::from(m.py), w.mask());
    if !w.scene().street(cx, cy) {
        return 0;
    }
    if m.step != m.steps {
        return walk_along(w, m, frame, budget);
    }
    // Fin del tramo: elegir el siguiente.
    if m.f56 > 2 || m.f57 > 2 {
        run_choose_target(w, m);
    }
    let (dir, aux) = direction_to(m.px, m.py, m.target.0, m.target.1);
    m.aux = aux;
    let heading = m.heading;
    let mut ahead = (cx, cy);
    w.geom().step(&mut ahead.0, &mut ahead.1, heading);
    let nb = neighbour(w.scene(), m.flags(), ahead, m.link, w.hanse());
    let nb_fields = nb.map(|i| w.entity_fields(ahead, i));
    if m.f55 != 0 {
        m.f55 -= 1;
    }
    if m.aux == heading {
        if nb_fields.is_some_and(|(h, _)| h == heading.wrapping_sub(0x80)) || try_move(w, m, heading) {
            m.want = heading;
            apply_advance(w, m, ahead);
            return 1;
        }
        if nb_fields.is_some_and(|(_, st)| st == 2 || st == 6) {
            if m.anim.state != 0x83 {
                let set = w.set().clone();
                let r = m.anim.select(&set, 0x83, w.clock());
                frame = if r == 0xFF { m.frame } else { r };
            }
            m.want = heading;
            apply_advance(w, m, ahead);
            w.invalidate();
            m.frame = frame;
            w.register();
            return 1;
        }
    }
    let back = |d: u8| d.wrapping_sub(0x80);
    // Alternativas por orden: eje principal, rumbo al destino, su contrario; después con la orden relajada (`b4 = 0`).
    if back(m.aux) == heading || !try_move(w, m, m.aux) {
        if back(dir) == heading || !try_move(w, m, dir) {
            if dir != heading && try_move(w, m, back(dir)) {
                m.want = back(dir);
                m.aux = heading;
            } else {
                m.order.b4 = 0;
                if back(m.aux) == heading || !try_move(w, m, m.aux) {
                    if back(dir) == heading || !try_move(w, m, dir) {
                        if dir != heading && try_move(w, m, back(dir)) {
                            m.want = back(dir);
                            m.aux = heading;
                            m.order.b4 = 1;
                        } else {
                            m.order.b4 = 1;
                            run_choose_target(w, m);
                            let (_, aux) = direction_to(m.px, m.py, m.target.0, m.target.1);
                            m.aux = aux;
                            m.want = m.aux;
                            if !try_move(w, m, m.aux) {
                                let r = w.rand();
                                m.want = (((r & 1) as u8).wrapping_mul(0x80)).wrapping_add(m.aux).wrapping_sub(0x40);
                                if !try_move(w, m, m.want) {
                                    m.want = m.want.wrapping_sub(0x80);
                                    if !try_move(w, m, m.want) {
                                        if m.v48 != 0xFFFF_FFFF {
                                            m.v48 = w.clock();
                                        }
                                        m.want = back(m.aux);
                                        if !try_move(w, m, m.want) {
                                            m.want = heading;
                                            m.v48 = 0xFFFF_FFFF;
                                            return w.visible() as u8;
                                        }
                                    }
                                }
                            }
                            m.step = 0;
                            m.steps = 0;
                        }
                    } else {
                        m.want = dir;
                        m.aux = heading;
                        m.order.b4 = 1;
                    }
                } else {
                    m.want = m.aux;
                    m.aux = heading;
                    m.order.b4 = 1;
                }
            }
        } else {
            m.want = dir;
            m.aux = heading;
        }
    } else {
        m.want = m.aux;
        m.aux = heading;
    }
    let mut cell = (cx, cy);
    w.geom().step(&mut cell.0, &mut cell.1, m.want);
    apply_advance(w, m, cell);
    m.anim.set_target(2);
    1
}

/// Tramo con el rumbo ya alineado: avanza `budget` pasos por el vector de la figura.
fn walk_along(w: &mut dyn LoopWorld, m: &mut FreeWalker, frame: u8, mut budget: u32) -> u8 {
    let (mut nx, mut ny) = (m.x, m.y);
    let st = m.anim.state;
    if st == 6 {
        budget = budget.wrapping_mul(2);
    }
    if st == 2 || st == 6 {
        let left = u32::from(m.steps).wrapping_sub(u32::from(m.step));
        if left < budget {
            budget = left;
        }
        m.step = m.step.wrapping_add(budget as u16);
        nx = nx.wrapping_add((m.vx as u32).wrapping_mul(budget));
        ny = ny.wrapping_add((m.vy as u32).wrapping_mul(budget));
    }
    let moved = if (nx >> 16) as i32 == i32::from(m.px) && (ny >> 16) as i32 == i32::from(m.py) {
        false
    } else {
        let cell = w.geom().pixel_to_cell((nx >> 16) as i32, (ny >> 16) as i32, w.mask()).0;
        if u32::from(m.steps) * 2 < u32::from(m.step) * 3 && m.f54 == 0 && !cell_free(w, cell, m.link) {
            apply_advance(w, m, cell);
            if m.f54 != 0 {
                return 1;
            }
        }
        true
    };
    if frame != m.frame || moved {
        w.invalidate();
        m.x = nx;
        m.y = ny;
        m.frame = frame;
        w.register();
    } else {
        m.x = nx;
        m.y = ny;
    }
    1
}

/// Giro en el sitio hacia el rumbo deseado (`+0x3C`), un octavo de vuelta por cambio de fotograma.
fn turn_step_loop(w: &mut dyn LoopWorld, m: &mut FreeWalker, mut frame: u8) -> u8 {
    if frame != m.frame {
        w.invalidate();
        if (m.want.wrapping_sub(m.heading) as i8) < 0 {
            m.f57 = 0;
            m.heading = m.heading.wrapping_sub(0x20);
            m.f56 = m.f56.wrapping_add(1);
        } else {
            m.f56 = 0;
            m.heading = m.heading.wrapping_add(0x20);
            m.f57 = m.f57.wrapping_add(1);
        }
        if m.anim.target != 2 && m.anim.target != 6 {
            m.anim.set_target(2);
        }
        let set = w.set().clone();
        let past = |id: u8| set.states.iter().find(|s| s.id == u32::from(id)).map_or(0, |s| s.last) < u32::from(frame);
        if (m.anim.state != 2 || past(2)) && (m.anim.state != 6 || past(6)) {
            frame = m.anim.select(&set, 2, w.clock());
        }
        m.frame = frame;
        w.register();
    }
    m.f55 = 3;
    if (m.f56 > 3 || m.f57 > 3) && !w.visible() {
        return 0;
    }
    1
}

/// `0x00559FE0` (animación terminada de una figura libre, `+0x6A == 0`): en la espera (estado 2) elige al azar con `rand & 0x1F` una pausa (0: `0x83`,
/// 1: `0x84`, 2: `5`; el resto deja la siguiente animación como está); en cualquier otro estado vuelve a la espera. Sin oráculo propio.
pub fn free_anim_done(a: &mut MoverAnim, rand: &mut dyn FnMut() -> u32) {
    if a.state == 2 {
        match rand() & 0x1F {
            0 => a.set_target(0x83),
            1 => a.set_target(0x84),
            2 => a.set_target(5),
            _ => {}
        }
        return;
    }
    a.set_target(2);
}

/// `0x0055C170`: rumbo (uno de los ocho múltiplos de `0x20`) de un desplazamiento `(dx, dy)`; la zona horizontal es `4|dx| > 15|dy|`, la vertical
/// `11|dx| < 7|dy|` y entre ambas la diagonal. `dy < 0` es hacia arriba (rumbo `0`).
pub fn route_heading(dx: i32, dy: i32) -> u8 {
    let (a, b) = (dx.unsigned_abs(), dy.unsigned_abs());
    if a.wrapping_mul(4) > b.wrapping_mul(15) {
        return if dx < 0 { 0xC0 } else { 0x40 };
    }
    if a.wrapping_mul(11) < b.wrapping_mul(7) {
        return if dy < 0 { 0x00 } else { 0x80 };
    }
    match (dx < 0, dy < 0) {
        (true, true) => 0xE0,
        (true, false) => 0xA0,
        (false, true) => 0x20,
        (false, false) => 0x60,
    }
}

/// `0x0055C200`: vector de avance `(vx, vy)` (16.16 por tick) del rumbo `heading` para la rapidez `speed` del conjunto de sprites (`+0xC`).
/// `None` si el rumbo no es uno de los ocho (el original no escribe nada). Las divisiones sin signo y con signo son las del original.
pub fn route_vector(heading: u8, speed: u32) -> Option<(i32, i32)> {
    let s = speed as i32;
    let sd = |v: i32, d: i32| v.wrapping_mul(s).wrapping_div(d);
    let ud = |v: i32, d: u32| (s.wrapping_mul(v) as u32 / d) as i32;
    Some(match heading {
        0x00 => (0, sd(-0xb000, 0x2981)),
        0x20 => (ud(0xb50, 0x271), sd(-0x7c73, 0x2981)),
        0x40 => (ud(0x1000, 0x271), 0),
        0x60 => (ud(0xb50, 0x271), ud(0x7c73, 0x2981)),
        0x80 => (0, ud(0xb000, 0x2981)),
        0xA0 => (sd(-0xb50, 0x271), ud(0x7c73, 0x2981)),
        0xC0 => (sd(-0x1000, 0x271), 0),
        0xE0 => (sd(-0xb50, 0x271), sd(-0x7c73, 0x2981)),
        _ => return None,
    })
}

// ---------------------------------------------------------------------------------------------------------------------------------------
// Caminar por rutas de nodos `0x0055B6A0` (figuras de clase 1 y 2 sobre las tablas `.nodes`).
// ---------------------------------------------------------------------------------------------------------------------------------------

/// Figura que sigue una ruta: los `0x78` primeros bytes de la entidad. Comparte la parte común con [`FreeWalker`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RouteWalker {
    pub fw: FreeWalker,
    /// `+0x58..+0x67`: dos puntos laterales (píxeles `x, y`) del tramo.
    pub side: [i32; 4],
    /// `+0x68`: ruta; `+0x69`: nodo actual; `+0x6A`: tipo (0 libre, 1 y 2 por ruta); `+0x6B`: fase (`0xFF` por decidir).
    pub route: u8,
    pub node: u8,
    pub kind: u8,
    pub phase: u8,
    /// `+0x6C` (llegó a la calle), `+0x6D` (marcha rápida), `+0x70`/`+0x74` (desplazamiento lateral en píxeles).
    pub f6c: u8,
    pub fast: u8,
    pub off: (i32, i32),
    ext: [u8; 8],
}

impl RouteWalker {
    /// Figura libre sin ruta (todos los campos de ruta a cero).
    pub fn new(fw: FreeWalker) -> Self {
        let mut b = [0u8; 0x78];
        b[..0x70].copy_from_slice(&fw.to_bytes());
        Self::from_bytes(&b)
    }

    pub fn from_bytes(b: &[u8]) -> Self {
        let fw = FreeWalker::from_bytes(&b[..0x70]);
        let i32_at = |o: usize| i32::from_le_bytes([b[o], b[o + 1], b[o + 2], b[o + 3]]);
        let mut ext = [0u8; 8];
        ext.copy_from_slice(&b[0x70..0x78]);
        RouteWalker {
            fw,
            side: [i32_at(0x58), i32_at(0x5C), i32_at(0x60), i32_at(0x64)],
            route: b[0x68],
            node: b[0x69],
            kind: b[0x6A],
            phase: b[0x6B],
            f6c: b[0x6C],
            fast: b[0x6D],
            off: (i32_at(0x70), i32_at(0x74)),
            ext,
        }
    }

    pub fn to_bytes(&self) -> [u8; 0x78] {
        let mut out = [0u8; 0x78];
        out[..0x70].copy_from_slice(&self.fw.to_bytes());
        out[0x70..].copy_from_slice(&self.ext);
        for (k, v) in self.side.iter().enumerate() {
            out[0x58 + 4 * k..0x5C + 4 * k].copy_from_slice(&v.to_le_bytes());
        }
        out[0x68] = self.route;
        out[0x69] = self.node;
        out[0x6A] = self.kind;
        out[0x6B] = self.phase;
        out[0x6C] = self.f6c;
        out[0x6D] = self.fast;
        out[0x70..0x74].copy_from_slice(&self.off.0.to_le_bytes());
        out[0x74..0x78].copy_from_slice(&self.off.1.to_le_bytes());
        out
    }
}

/// Lo que el caminar por rutas consulta fuera de la figura.
pub trait RouteWorld {
    /// Método virtual `+0x50` de la figura (¿sigue activa?).
    fn alive(&self) -> bool;
    /// Las dos tablas de rutas (`escena+0xC674`, `+0xC678`) existen.
    fn tables(&self) -> bool;
    /// Puntos `(x, y)` de la ruta `route` del tipo `kind` (1 o 2).
    fn nodes(&self, kind: u8, route: u8) -> Vec<(i32, i32)>;
    /// Punto `idx` de la ruta leído sin comprobar el recuento (el original lee más allá del último nodo; `0x55B1D0` lo hace con `i + 2`).
    fn raw_node(&self, kind: u8, route: u8, idx: usize) -> (i32, i32) {
        self.nodes(kind, route).get(idx).copied().unwrap_or((0, 0))
    }
    fn scene(&self) -> &dyn WalkScene;
    fn geom(&self) -> &Geom;
    fn mask(&self) -> &[u8];
    fn set(&self) -> &SetInfo;
    fn speed(&self) -> u32;
    fn budget(&self) -> u32;
    fn clock(&self) -> u32;
    fn hanse(&self) -> bool;
    /// Ruta del puerto que bloquea a las figuras rápidas (`DAT_006DF270 -> +0x76`).
    fn harbour_route(&self) -> u8;
    /// Esquiva de carros `0x570BA0` (el «desplazamiento de orilla»: distancia al carro más cercano de la lista global `DAT_006E6280`, `None` si está vacía).
    fn shore(&self, x: i32, y: i32, route: u8) -> Option<u16>;
    fn invalidate(&mut self);
    fn register(&mut self);
    fn anim_done(&mut self, a: &mut MoverAnim);
}

fn shore_offset(heading: u8, d: u32) -> Option<(i32, i32)> {
    let d = d as i32;
    Some(match heading {
        0x00 => (d, 0),
        0x20 => (d, d * 0xb / 0x11),
        0x40 => (0, d * 0xb / 0x11),
        0x60 => (-d, d * 0xb / 0x11),
        0x80 => (-d, 0),
        0xA0 => (-d, d * -0xb / 0x11),
        0xC0 => (0, d * -0xb / 0x11),
        0xE0 => (d, d * -0xb / 0x11),
        _ => return None,
    })
}

/// Distancia lateral según el valor de orilla `s` (`0x570BA0`): 8 si `s < 0x12`, si no `(0x44 - s) / 6`; `None` si `s > 0x43`.
fn shore_distance(s: Option<u16>) -> Option<u32> {
    let s = s?;
    if s > 0x43 {
        return None;
    }
    Some(if s < 0x12 { 8 } else { (0x44 - u32::from(s)) / 6 })
}

/// `0x0055B6A0` para figuras de tipo 1 y 2 (el tipo 0 delega en `0x55C820`, véase [`free_walk_step`]). Devuelve el valor del original.
pub fn route_walk_step(w: &mut dyn RouteWorld, m: &mut RouteWalker) -> u8 {
    if !w.alive() {
        return 0;
    }
    if m.kind == 0 {
        return 0;
    }
    if m.kind == 2 && m.f6c != 0 {
        return 0;
    }
    if !w.tables() {
        return 0;
    }
    let speed = w.speed();
    // Fase por decidir: calcular los puntos laterales del primer tramo.
    if m.phase == 0xFF {
        let pts = w.nodes(m.kind, m.route);
        let i = usize::from(m.node);
        if i + 1 < pts.len() {
            let (x0, y0) = pts[i];
            let (x1, y1) = pts[i + 1];
            let c = route_heading(x1.wrapping_sub(x0), y1.wrapping_sub(y0));
            if c.wrapping_add(0x40) == m.fw.want {
                let geom = w.geom();
                let ((mut col, mut row), _) = geom.pixel_to_cell(x0, y0, w.mask());
                geom.step(&mut col, &mut row, m.fw.want.wrapping_sub(0x80));
                let p = geom.cell_to_pixel(col as i32, row);
                m.side[0] = p.0;
                m.side[1] = p.1;
                let ((mut col, mut row), _) = geom.pixel_to_cell(x0, y0, w.mask());
                geom.step(&mut col, &mut row, m.fw.want.wrapping_sub(0x40));
                let p = geom.cell_to_pixel(col as i32, row);
                m.side[2] = p.0;
                m.side[3] = p.1;
                m.phase = 1;
            } else {
                m.phase = 0;
            }
        } else {
            m.phase = 0;
        }
    }
    let budget = w.budget();
    let mut set = w.set().clone();
    if m.fast != 0 {
        if !w.hanse() {
            m.fast = 0;
        } else {
            set.interval /= 3;
        }
    }
    let clock = w.clock();
    let frame = {
        let mut cb = |a: &mut MoverAnim| w.anim_done(a);
        m.fw.anim.advance(&set, m.fw.heading, m.fw.frame, None, clock, &mut cb)
    };
    let set = w.set().clone();
    if m.fw.heading != m.fw.want {
        // Giro en el sitio.
        if frame == m.fw.frame {
            return 1;
        }
        w.invalidate();
        m.fw.heading = turn_step(m.fw.heading, m.fw.want, set.directions as u8);
        m.fw.anim.set_target(2);
        m.fw.frame = if m.fw.anim.state == 2 { frame } else { m.fw.anim.select(&set, 2, clock) };
        let mut x = m.fw.x.wrapping_sub((m.off.0 as u32).wrapping_shl(16));
        let mut y = m.fw.y.wrapping_sub((m.off.1 as u32).wrapping_shl(16));
        m.off = (0, 0);
        if let Some(d) = shore_distance(w.shore((x >> 16) as i32, (y >> 16) as i32, m.route)) {
            if let Some((ox, oy)) = shore_offset(m.fw.heading, d) {
                m.off = (ox, oy);
            }
            x = x.wrapping_add((m.off.0 as u32).wrapping_shl(16));
            y = y.wrapping_add((m.off.1 as u32).wrapping_shl(16));
        }
        m.fw.x = x;
        m.fw.y = y;
        w.register();
        return 1;
    }
    // Avance por el vector del tramo.
    let mut nx = m.fw.x.wrapping_sub((m.off.0 as u32).wrapping_shl(16));
    let mut ny = m.fw.y.wrapping_sub((m.off.1 as u32).wrapping_shl(16));
    if m.fw.anim.state == 2 {
        let mul = if m.fast == 0 { 1 } else { 3 };
        nx = nx.wrapping_add((m.fw.vx as u32).wrapping_mul(budget).wrapping_mul(mul));
        ny = ny.wrapping_add((m.fw.vy as u32).wrapping_mul(budget).wrapping_mul(mul));
    }
    m.off = (0, 0);
    if let Some(d) = shore_distance(w.shore((nx >> 16) as i32, (ny >> 16) as i32, m.route)) {
        if let Some((ox, oy)) = shore_offset(m.fw.want, d) {
            m.off = (ox, oy);
        }
        nx = nx.wrapping_add((m.off.0 as u32).wrapping_shl(16));
        ny = ny.wrapping_add((m.off.1 as u32).wrapping_shl(16));
    }
    let changed = !((nx >> 16) as i32 == i32::from(m.fw.px) && (ny >> 16) as i32 == i32::from(m.fw.py));
    if changed {
        let geom = w.geom();
        let mask = w.mask();
        let cur = geom.pixel_to_cell(i32::from(m.fw.px), i32::from(m.fw.py), mask).0;
        let new = geom.pixel_to_cell(i32::from((nx >> 16) as i16), i32::from((ny >> 16) as i16), mask).0;
        let pts = w.nodes(m.kind, m.route);
        let node = usize::from(m.node);
        let (mut tx, mut ty) = pts.get(node).copied().unwrap_or((0, 0));
        match m.phase {
            0 => {}
            1 => (tx, ty) = (m.side[0], m.side[1]),
            _ => (tx, ty) = (m.side[2], m.side[3]),
        }
        let (cols, rows) = w.scene().size();
        if cur != new && m.kind == 1 && new.0 < cols && new.1 < rows && w.scene().street(new.0, new.1) {
            m.f6c = 1;
            if m.fast != 0 && w.harbour_route() == m.route {
                return 0;
            }
        }
        let (vx, vy) = (m.fw.vx, m.fw.vy);
        let (ux, uy) = (nx >> 16, ny >> 16);
        let (tux, tuy) = (tx as u32, ty as u32);
        let ycheck = (vy > 0 && tuy <= uy) || (vy < 0 && uy <= tuy);
        let reached = if vx > 0 {
            if ux < tux { ycheck } else { true }
        } else if vx >= 0 || tux < ux {
            ycheck
        } else {
            true
        };
        if reached {
            if node + 1 < pts.len() && m.f6c == 0 {
                let c;
                if m.phase == 1 {
                    c = route_heading(m.side[2].wrapping_sub(tx), m.side[3].wrapping_sub(ty));
                    m.phase = 2;
                } else {
                    m.node = m.node.wrapping_add(1);
                    let n = usize::from(m.node);
                    c = if m.phase == 2 {
                        m.fw.want.wrapping_sub(0x20)
                    } else {
                        let p = pts.get(n).copied().unwrap_or((0, 0));
                        route_heading(p.0.wrapping_sub(tx), p.1.wrapping_sub(ty))
                    };
                    m.phase = 0xFF;
                }
                m.fw.want = c;
                if let Some((vx, vy)) = route_vector(c, speed) {
                    m.fw.vx = vx;
                    m.fw.vy = vy;
                }
                nx = (m.off.0.wrapping_add(tx) as u32).wrapping_shl(16);
                ny = (m.off.1.wrapping_add(ty) as u32).wrapping_shl(16);
            } else {
                if m.kind != 1 {
                    return 0;
                }
                let p = pts.get(node).copied().unwrap_or((0, 0));
                nx = (p.0 as u32).wrapping_shl(16);
                ny = (p.1 as u32).wrapping_shl(16);
                m.kind = 0;
                m.fw.order.b6 = 0;
            }
        }
    }
    if frame == m.fw.frame && !changed {
        m.fw.x = nx;
        m.fw.y = ny;
        return 1;
    }
    w.invalidate();
    m.fw.x = nx;
    m.fw.y = ny;
    m.fw.frame = frame;
    w.register();
    1
}

/// `0x0055C4A0`: invierte el sentido de una figura de tipo 2 sobre la ruta (siempre activa la marcha rápida `+0x6D`). `nodes` son los puntos de la
/// ruta de la tabla de tipo 1 (`escena+0xC674`, la que consulta el original) y `speed` la rapidez del conjunto de sprites.
pub fn route_reverse(m: &mut RouteWalker, nodes: &[(i32, i32)], speed: u32) {
    m.fast = 1;
    if m.kind == 2 {
        m.kind = 1;
        let count = nodes.len() as u8;
        let (px, py) = (i32::from(m.fw.px), i32::from(m.fw.py));
        if m.node == 0 {
            m.f6c = 1;
            m.node = count.wrapping_sub(1);
        } else {
            m.node = count.wrapping_sub(m.node);
            m.side[0] = px;
            m.side[1] = py;
            if m.phase == 2 {
                let (tx, ty) = nodes.get(usize::from(m.node)).copied().unwrap_or((0, 0));
                m.side[2] = tx;
                m.side[3] = ty;
                m.fw.want = route_heading(tx.wrapping_sub(px), ty.wrapping_sub(py));
                if let Some((vx, vy)) = route_vector(m.fw.want, speed) {
                    m.fw.vx = vx;
                    m.fw.vy = vy;
                }
            } else {
                m.fw.want = m.fw.want.wrapping_sub(0x40);
                m.phase = 2;
                if let Some((vx, vy)) = route_vector(m.fw.want, speed) {
                    m.fw.vx = vx;
                    m.fw.vy = vy;
                }
                m.side[2] = px + if m.fw.vx < 1 { -0x11 } else { 0x11 };
                m.side[3] = py + if m.fw.vy < 1 { -0xb } else { 0xb };
            }
        }
        if m.node == 0 {
            m.f6c = 1;
            m.kind = 2;
        }
    }
}

/// `0x0055B1D0`: asigna a la figura la ruta `route`. Con `start` la coloca en el primer nodo de la ruta de tipo 1 (`escena+0xC674`); sin él busca un
/// punto de la ruta de tipo 2 (`+0xC678`; el barrido lee los puntos `i + 1`): fuera de la vista si la figura está en el centinela `x == 5000`, y si no el tramo en cuya casilla está.
/// `view` es `(+0xC650, +0xC654, +0x2C, +0x30)`. Devuelve el valor del original (1: asignada).
pub fn route_assign(w: &mut dyn RouteWorld, m: &mut RouteWalker, route: u8, start: bool, view: (u32, u32, u32, u32)) -> u8 {
    if !w.tables() {
        return 0;
    }
    let speed = w.speed();
    m.f6c = 0;
    m.off = (0, 0);
    m.phase = 0xFF;
    let set_vec = |m: &mut RouteWalker, h: u8| {
        if let Some((vx, vy)) = route_vector(h, speed) {
            m.fw.vx = vx;
            m.fw.vy = vy;
        }
    };
    if start {
        let pts = w.nodes(1, route);
        let (x0, y0) = pts.first().copied().unwrap_or((0, 0));
        let (x1, y1) = pts.get(1).copied().unwrap_or((0, 0));
        m.fw.x = (x0 as u32).wrapping_shl(16);
        m.fw.y = (y0 as u32).wrapping_shl(16);
        m.fw.heading = route_heading(x1.wrapping_sub(x0), y1.wrapping_sub(y0));
        set_vec(m, m.fw.heading);
        m.route = route;
        m.node = 1;
        m.kind = 1;
        m.fw.order.b6 = 1;
        m.fast = 0;
        return 1;
    }
    let pts = w.nodes(2, route);
    if m.fw.x == 5000 {
        let mut i = 0usize;
        let (mut lx, mut ly) = (0i32, 0i32);
        if pts.len() != 1 {
            loop {
                let Some(&(x, y)) = pts.get(i + 1) else { break };
                lx = x;
                ly = y;
                let (cx, cy) = w.geom().pixel_to_cell(x, y, w.mask()).0;
                if !w.scene().street(cx, cy) {
                    break;
                }
                i += 1;
                if i >= pts.len().wrapping_sub(1) {
                    break;
                }
            }
        }
        let (ux, uy) = (lx as u32, ly as u32);
        let outside = ux.wrapping_add(0x32) <= view.0 || view.0.wrapping_add(view.2).wrapping_add(0x32) <= ux || uy.wrapping_add(0x32) <= view.1 || view.1.wrapping_add(view.3).wrapping_add(0x32) <= uy;
        if outside {
            let (nx, ny) = pts.get(i).copied().unwrap_or((0, 0));
            m.fw.x = ux.wrapping_shl(16);
            m.fw.y = uy.wrapping_shl(16);
            m.fw.heading = route_heading(nx.wrapping_sub(lx), ny.wrapping_sub(ly));
            set_vec(m, m.fw.heading);
            m.node = i as u8;
            m.route = route;
            m.kind = 2;
            m.fw.order.b6 = 1;
            m.fast = 0;
            return 1;
        }
        return 0;
    }
    let mut i = pts.len() as i32 - 2;
    while i >= 0 {
        let (x, y) = pts[i as usize + 1];
        let (px, py) = (i32::from(m.fw.px), i32::from(m.fw.py));
        if px <= x + 0xaa && x - 0xaa <= px && py <= y + 0x6e && y - 0x6e <= py {
            let cell_a = w.geom().pixel_to_cell(x, y, w.mask()).0;
            let cell_b = w.geom().pixel_to_cell(px, py, w.mask()).0;
            if cell_a == cell_b {
                let (nx, ny) = w.raw_node(2, route, i as usize + 2);
                m.fw.want = route_heading(nx.wrapping_sub(x), ny.wrapping_sub(y));
                set_vec(m, m.fw.want);
                m.node = (i as u8).wrapping_add(2);
                m.route = route;
                m.kind = 2;
                m.f6c = 0;
                w.invalidate();
                m.fw.x = (x as u32).wrapping_shl(16);
                m.fw.y = (y as u32).wrapping_shl(16);
                w.register();
                m.fw.order.b6 = 1;
                m.fast = 0;
                m.phase = 0;
                return 1;
            }
            let (sx, sy) = w.geom().cell_to_pixel(cell_b.0 as i32, cell_b.1);
            let (dx, dy) = (x.wrapping_sub(sx), y.wrapping_sub(sy));
            if dx.wrapping_abs().wrapping_mul(0xb) == dy.wrapping_abs().wrapping_mul(0x11) {
                let h = route_heading(dx, dy);
                let mut cell = cell_b;
                let mut k = 0;
                while k < 0xb {
                    if cell == cell_a {
                        m.fw.want = h;
                        set_vec(m, h);
                        m.node = (i as u8).wrapping_add(1);
                        m.route = route;
                        m.kind = 2;
                        m.f6c = 0;
                        w.invalidate();
                        m.fw.x = (sx as u32).wrapping_shl(16);
                        m.fw.y = (sy as u32).wrapping_shl(16);
                        w.register();
                        m.fw.order.b6 = 1;
                        m.fast = 0;
                        return 1;
                    }
                    w.geom().step(&mut cell.0, &mut cell.1, h);
                    if !can_enter(w.scene(), &m.fw.order, cell, 0, true, false) {
                        k = 10;
                    }
                    k += 1;
                }
            }
        }
        i -= 1;
    }
    0
}

/// Lo que además necesita la colocación de una figura (`0x559D60`).
pub trait PlaceWorld: RouteWorld {
    /// `0x55D9D0` con el entorno de la ciudad y el `rand` del mundo.
    fn choose_target(&mut self, st: &mut TargetState);
    /// Clase de la entidad (método virtual `+0x20`; 4 = mendigo).
    fn entity_class(&self) -> u8;
    /// `(+0xC650, +0xC654, +0x2C, +0x30)`: origen y tamaño de la vista.
    fn view(&self) -> (u32, u32, u32, u32);
    /// `0x559CF0`: reserva la tabla de enlaces de la figura (sin efecto en la simulación).
    fn alloc_links(&mut self) {}
}

/// `0x0055C7A0`: deja la figura en `(x, y)` (16.16) con el rumbo y el enlace dados, sin pasos pendientes, elige el primer destino y la registra.
pub fn init_free(w: &mut dyn PlaceWorld, m: &mut RouteWalker, x: u32, y: u32, heading: u8, link: u16) {
    let f = &mut m.fw;
    f.heading = heading;
    f.want = heading;
    f.aux = heading;
    f.y = y;
    f.x = x;
    // `+0xE` y `+0x12` son las mitades altas de `+0xC` y `+0x10`.
    f.px = (x >> 16) as i16;
    f.py = (y >> 16) as i16;
    f.target.1 = f.py as u16;
    f.link = link;
    f.step = 0;
    f.steps = 0;
    f.f54 = 0;
    f.f55 = 0;
    f.f56 = 0;
    f.f57 = 0;
    f.target.0 = f.px as u16;
    let mut st = TargetState { pos: (f.px, f.py), target: f.target, timer: f.timer, b55: 0, b56: 0, b57: 0, beggar: w.entity_class() == 4 };
    w.choose_target(&mut st);
    f.target = st.target;
    f.timer = st.timer;
    f.f55 = st.b55;
    f.f56 = st.b56;
    f.f57 = st.b57;
    w.alloc_links();
    w.register();
}

/// `0x00559D60`: coloca una figura según `mode_word >> 16`: 0, libre en `(x, y)`; 1, al inicio de la ruta `mode_word & 0xFF` de tipo 1; 2, fuera de la
/// vista sobre una ruta de tipo 2 (con las dos siguientes módulo 3 como respaldo y, si ninguna sirve, `+0x6C = 1` y tipo 2).
pub fn place_entity(w: &mut dyn PlaceWorld, m: &mut RouteWalker, x: u32, y: u32, heading: u8, link: u16, mode_word: u32) {
    let set = w.set().clone();
    let clock = w.clock();
    m.fw.anim.select(&set, 2, clock);
    let order = movement_order(0, 1, 0, 0);
    let mut set_order = |m: &mut RouteWalker| {
        m.fw.order.b4 = order[0];
        m.fw.order.b5 = order[1];
        m.fw.order.b6 = order[2];
        m.fw.b7 = order[3];
    };
    let view = w.view();
    match mode_word >> 16 {
        0 => {
            set_order(m);
            init_free(w, m, x, y, heading, link);
        }
        1 => {
            set_order(m);
            route_assign(w, m, mode_word as u8, true, view);
            let (px, py, h) = (m.fw.x, m.fw.y, m.fw.heading);
            init_free(w, m, px, py, h, link);
        }
        2 => {
            set_order(m);
            m.fw.x = 5000;
            let r = (mode_word & 0xFF) as u8;
            if route_assign(w, m, r, false, view) == 0 && route_assign(w, m, ((u32::from(r) + 1) % 3) as u8, false, view) == 0 && route_assign(w, m, ((u32::from(r) + 2) % 3) as u8, false, view) == 0 {
                m.f6c = 1;
                m.kind = 2;
            }
            let (px, py, h) = (m.fw.x, m.fw.y, m.fw.heading);
            init_free(w, m, px, py, h, link);
        }
        _ => {}
    }
}

/// `0x00570BA0(x, y, ruta)` sobre la lista de segmentos de orilla (`+0x60` siguiente, `+0x3C` ruta, `+0xE`/`+0x12` centro, 16 bits con signo): distancia mínima
/// `max(|dx|, |dy| * 17 / 11)` a los segmentos de esa ruta; `0xFFFF` si no hay ninguno. Solo cuentan los 16 bits bajos del retorno (el original deja basura arriba).
pub fn shore_value(m: &mut dyn crate::scene_math::Mem, seg: u32, x: u32, y: u32, route: u8) -> u16 {
    let next = m.r32(seg + 0x60);
    let bx: u16 = if next == 0 { 0xFFFF } else { shore_value(m, next, x, y, route) };
    if m.r8(seg + 0x3C) != route {
        return bx;
    }
    let cx = i32::from(m.r16(seg + 0xE) as i16);
    let cy = i32::from(m.r16(seg + 0x12) as i16);
    let di = ((x & 0xFFFF) as i32).wrapping_sub(cx).wrapping_abs() as u16;
    let ay = ((y & 0xFFFF) as i32).wrapping_sub(cy).wrapping_abs();
    let q = (ay.wrapping_mul(17) / 11) as u16;
    let cand = if di > q { di } else { q };
    if cand < bx {
        cand
    } else {
        bx
    }
}
