//! Planificador de rutas marítimas (A03, `RoutePlanner`, global `g_class35`): rejilla de navegación, conversión de coordenadas y
//! ajuste a mar. Referencia: x86 original en emulación (`reimpl/oracle/x86_planner.py`).

/// Datos de navegación del planificador: rejilla de bytes (0 = mar libre) y escala de conversión del mundo a celdas.
#[derive(Clone, Debug, PartialEq)]
pub struct NavGrid {
    /// `+0x14`: `width * height` bytes; distinto de cero es obstáculo.
    pub cells: Vec<u8>,
    /// `+0x18` y `+0x1A` (palabras con signo).
    pub width: i16,
    pub height: i16,
    /// Global `0x0066E0D0` (de fábrica `0xDD0`): el mundo mide `escala/1000` unidades por celda.
    pub scale: i32,
}

impl NavGrid {
    fn cell(&self, x: i32, y: i32) -> u8 {
        let w = self.width as i32;
        self.cells.get((y * w + x) as usize).copied().unwrap_or(0)
    }

    /// `(v * 1000 + escala / 2) / escala`: celda de la rejilla de una coordenada del mundo (división con signo del original).
    pub(crate) fn to_cell(&self, v: i16) -> i32 {
        (v as i32 * 1000 + (self.scale >> 1)) / self.scale
    }

    /// `0x004474D0` (`ret 4`): el punto `(x, y)` del mundo cae dentro de la rejilla y su celda es mar libre.
    pub fn is_point_navigable(&self, x: i16, y: i16) -> bool {
        let gx = self.to_cell(x);
        let gy = self.to_cell(y);
        if gy >= self.height as i32 || gx >= self.width as i32 || y < 0 || x < 0 || gy < 0 || gx < 0 {
            return false;
        }
        self.cell(gx, gy) == 0
    }

    /// `0x00448450` (`ret 8`): si la celda `(x, y)` (ya en celdas) está libre devuelve `true` sin cambiar nada; si no, busca por
    /// anillos de radio creciente hasta `radius` la celda libre más cercana (distancia al cuadrado, gana la primera hallada) y la
    /// deja en `point`. `false` si no hay ninguna o el punto cae fuera.
    pub fn snap_to_sea(&self, point: &mut (i16, i16), radius: u8) -> bool {
        let (x, y) = (point.0 as i32, point.1 as i32);
        let (w, h) = (self.width as i32, self.height as i32);
        if x < 0 || y < 0 || x > w || y > h {
            return false;
        }
        if self.cell(x, y) == 0 {
            return true;
        }
        let radius = radius as i32;
        if radius < 1 {
            return false;
        }
        let mut best = i32::MAX;
        let mut found = (0i32, 0i32);
        let mut k = 1i32;
        loop {
            let mut dx = -k;
            while dx <= k {
                let mut dy = -k;
                while dy <= k {
                    let cx = (x + dx) as i16 as i32;
                    let cy = (y + dy) as i16 as i32;
                    if cy >= 0 && cx >= 0 && cy < h && cx < w && self.cell(cx, cy) == 0 {
                        let d = dx * dx + dy * dy;
                        if d < best {
                            best = d;
                            found = (cx, cy);
                        }
                    }
                    dy = if dx != -k && dx != k && dy != k { k } else { dy + 1 };
                }
                dx += 1;
            }
            k += 1;
            if best <= k * k {
                break;
            }
            if k > radius {
                if best >= i32::MAX {
                    return false;
                }
                break;
            }
        }
        *point = (found.0 as i16, found.1 as i16);
        true
    }
}

/// Muros verticales de la rejilla de navegación (`0x0066D6E4`, `0x0066D704`, `0x0066D724`): `x`, `y` mínimo e `y` máximo.
const WALL_X: [u16; 15] = [
    187, 214, 232, 221, 281, 282, 308, 322, 350, 406, 407, 463, 468, 471, 621,
];
const WALL_X_LO: [u16; 15] = [357, 0, 316, 200, 249, 304, 328, 0, 280, 320, 168, 153, 0, 187, 22];
const WALL_X_HI: [u16; 15] = [471, 132, 471, 354, 296, 319, 471, 274, 289, 471, 204, 180, 78, 471, 471];
/// Muros horizontales (`0x0066D6C8`, `0x0066D690`, `0x0066D6AC`): `y`, `x` mínimo y `x` máximo.
const WALL_Y: [u16; 13] = [80, 82, 132, 133, 141, 202, 234, 271, 274, 285, 285, 346, 409];
const WALL_Y_LO: [u16; 13] = [0, 167, 0, 472, 452, 294, 190, 260, 0, 346, 236, 0, 107];
const WALL_Y_HI: [u16; 13] = [50, 420, 56, 639, 472, 387, 248, 291, 49, 355, 256, 73, 639];

/// Un muro (coordenada fija `fixed` en el eje `s`, tramo `lo..=hi` en el eje `t`) corta el segmento `(s1, t1)-(s2, t2)` si la
/// coordenada fija cae entre los extremos, los rangos se solapan y el producto cruzado cae dentro del tramo (aritmética entera con
/// signo del original, sin normalizar el signo de `ds`).
fn wall_blocks(fixed: i32, lo: i32, hi: i32, (s1, t1): (i32, i32), (s2, t2): (i32, i32)) -> bool {
    if (fixed < s1 && fixed < s2) || (fixed > s1 && fixed > s2) {
        return false;
    }
    if (t1 < lo && t2 < lo) || (t1 > hi && t2 > hi) {
        return false;
    }
    let (ds, dt) = (s2 - s1, t2 - t1);
    let num = (fixed - s1).wrapping_mul(dt).wrapping_add(ds >> 1);
    if num < (lo - t1).wrapping_mul(ds) {
        return false;
    }
    num <= (hi - t1).wrapping_mul(ds)
}

impl NavGrid {
    /// `0x004445A0` (cdecl `grid, &ancho, fin, inicio`): ¿hay línea libre entre dos celdas? Comprueba los 15 muros verticales y 13
    /// horizontales fijos y recorre después la línea (Bresenham) sin mirar el inicio ni el final. Si es libre suma uno al contador
    /// global `0x006CBDC4` y devuelve `true`. Dos puntos iguales son libres sin contar.
    ///
    /// Con empate exacto del error, si la celda del paso en el eje mayor está libre el original avanza en diagonal sin mirar la
    /// celda diagonal; si está ocupada, mira solo la diagonal (puede pasar entre dos obstáculos).
    pub fn is_line_clear(&self, start: (i16, i16), end: (i16, i16), counter: &mut u32) -> bool {
        let (x1, y1) = (start.0 as i32, start.1 as i32);
        let (x2, y2) = (end.0 as i32, end.1 as i32);
        if x1 == x2 && y1 == y2 {
            return true;
        }
        if x1 != x2 {
            for i in 0..15 {
                if wall_blocks(
                    WALL_X[i] as i32,
                    WALL_X_LO[i] as i32,
                    WALL_X_HI[i] as i32,
                    (x1, y1),
                    (x2, y2),
                ) {
                    return false;
                }
            }
        }
        if y1 != y2 {
            for i in 0..13 {
                if wall_blocks(
                    WALL_Y[i] as i32,
                    WALL_Y_LO[i] as i32,
                    WALL_Y_HI[i] as i32,
                    (y1, x1),
                    (y2, x2),
                ) {
                    return false;
                }
            }
        }
        let (dx, dy) = (x2 - x1, y2 - y1);
        let (adx, ady) = (dx.abs(), dy.abs());
        let sx = dx.signum();
        let sy = dy.signum();
        let w = self.width as i32;
        let mut pos = y1 * w + x1;
        let step_y = w * sy;
        let occupied = |p: i32| self.cells.get(p as usize).copied().unwrap_or(0) != 0;
        if adx >= ady {
            let (limit, big, small) = (adx - 1, 2 * adx, 2 * ady);
            let mut err = adx;
            for _ in 0..limit.max(0) {
                pos += sx;
                err += small;
                if err < big {
                    if occupied(pos) {
                        return false;
                    }
                    continue;
                }
                err -= big;
                if err == 0 {
                    if !occupied(pos) {
                        pos += step_y;
                        continue;
                    }
                    pos += step_y;
                    if occupied(pos) {
                        return false;
                    }
                    continue;
                }
                pos += step_y;
                if occupied(pos) {
                    return false;
                }
            }
        } else {
            let (limit, big, small) = (ady - 1, 2 * ady, 2 * adx);
            let mut err = ady;
            for _ in 0..limit.max(0) {
                err += small;
                pos += step_y;
                if err < big {
                    if occupied(pos) {
                        return false;
                    }
                    continue;
                }
                err -= big;
                if err == 0 {
                    if !occupied(pos) {
                        pos += sx;
                        continue;
                    }
                    pos += sx;
                    if occupied(pos) {
                        return false;
                    }
                    continue;
                }
                pos += sx;
                if occupied(pos) {
                    return false;
                }
            }
        }
        *counter = counter.wrapping_add(1);
        true
    }
}

impl NavGrid {
    /// Coste de conexión de un punto a un nodo: `ftol(sqrt((dx² + dy²) · 2³²) + 0.5)`, es decir, la distancia con 16 bits
    /// fraccionarios (`0x0063A6A0` multiplica en 64 bits, `0x0063AAE4` es la raíz y `0x00639C28` el `__ftol`).
    fn connection_cost(dx: i32, dy: i32) -> i32 {
        let d2 = dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx));
        let wide = (d2 as i64).wrapping_shl(32);
        crate::numeric::ftol_i32((wide as f64).sqrt() + 0.5)
    }

    /// `0x00444800` (cdecl `origen, destino, nodos, filas, &nodos, rejilla, &ancho`, devuelve un objeto de 8 bytes): elige el par
    /// de nodos precalculados entre los que unir `src` con `dst`.
    ///
    /// Para cada extremo reúne los nodos visibles con `is_line_clear` y su coste de conexión, en una lista ordenada de mayor a
    /// menor (a igualdad, el nuevo va detrás). Recorre el producto `A × B` y se queda con el menor
    /// `coste_A + coste_B + filas[A][B]`; en empate de total gana el par con coste interno menor o igual. Sin candidatos
    /// devuelve `(0, 0)`. Los índices iniciales son 0 y los mejores costes `i32::MAX`.
    pub fn select_nodes(
        &self,
        src: (i32, i32),
        dst: (i32, i32),
        nodes: &[(i16, i16)],
        rows: &[Vec<i32>],
        counter: &mut u32,
    ) -> (i32, i32) {
        let build = |origin: (i32, i32), counter: &mut u32| -> Vec<(i32, i16)> {
            let mut list: Vec<(i32, i16)> = Vec::new();
            for (k, node) in nodes.iter().enumerate() {
                let dx = origin.0.wrapping_sub(node.0 as i32);
                let dy = origin.1.wrapping_sub(node.1 as i32);
                if !self.is_line_clear(*node, (origin.0 as i16, origin.1 as i16), counter) {
                    continue;
                }
                let cost = Self::connection_cost(dx, dy);
                let at = list.iter().position(|e| e.0 < cost).unwrap_or(list.len());
                list.insert(at, (cost, k as i16));
            }
            list
        };
        let list_a = build(src, counter);
        let list_b = build(dst, counter);
        let (mut best_total, mut best_inner) = (i32::MAX, i32::MAX);
        let mut best = (0i32, 0i32);
        for a in &list_a {
            let row = &rows[a.1 as usize];
            for b in &list_b {
                let inner = row[b.1 as usize];
                let total = b.0.wrapping_add(a.0).wrapping_add(inner);
                if total > best_total || (total == best_total && best_inner < inner) {
                    continue;
                }
                best_total = total;
                best = (a.1 as i32, b.1 as i32);
                best_inner = inner;
            }
        }
        best
    }
}

/// Ficheros de navegación del mapa 0 (`scripts/scrollmaps.ini`, claves `Navigationsmatrix`, `Navigationsvektor`, `Navpointmatrix`).
pub trait NavFiles {
    /// `0x004BE060` + `0x00402760`: resuelve el nombre de la clave `which` (0 rejilla, 1 vector, 2 matriz) y abre el fichero.
    fn open(&mut self, which: usize) -> Option<Vec<u8>>;
}

/// Datos de navegación cargados (`RoutePlanner+4/+0xC/+0x14/+0x18/+0x1A/+0x1C`).
#[derive(Clone, Debug, PartialEq)]
pub struct NavData {
    pub grid: NavGrid,
    /// Coordenadas de los nodos (`+0xC`, pares `i16`).
    pub nodes: Vec<(i16, i16)>,
    /// Cuenta del vector (`+0x1C`, `i16` con signo).
    pub node_count: i16,
    /// Matriz de costes: `n` filas de `n` ranuras `(coste i32, siguiente nodo i16)`; el original reserva 8 bytes por ranura y lee 6.
    pub matrix: Vec<Vec<(i32, i16)>>,
}

fn take_i16(data: &[u8], at: usize) -> i16 {
    let b = |k: usize| data.get(at + k).copied().unwrap_or(0);
    i16::from_le_bytes([b(0), b(1)])
}

/// `0x00444D30`: carga la rejilla (`i16` ancho, `i16` alto y `ancho × alto` bytes), el vector de nodos (`i16 n`, `i16` sin uso y
/// `n × (i16 x, i16 y)`) y la matriz (`n × n` ranuras de 6 bytes). `None` si falta alguno de los tres ficheros.
///
/// Límites del original: las tres lecturas usan la longitud del fichero menos 4 (la matriz, 6 bytes por ranura) y no el tamaño
/// reservado: un fichero más largo desborda el montón, uno más corto deja ceros aquí (basura en el original). Un tamaño negativo
/// o un número de nodos negativo reservaría una cantidad enorme y aquí devuelve `None`.
pub fn load_nav_data(files: &mut dyn NavFiles, scale: i32) -> Option<NavData> {
    let grid_file = files.open(0)?;
    let (w, h) = (take_i16(&grid_file, 0), take_i16(&grid_file, 2));
    let size = w as i32 * h as i32;
    if size < 0 {
        return None;
    }
    let mut cells = vec![0u8; size as usize];
    let body = grid_file.get(4..).unwrap_or(&[]);
    let n_copy = body.len().min(cells.len());
    cells[..n_copy].copy_from_slice(&body[..n_copy]);
    let vec_file = files.open(1)?;
    let count = take_i16(&vec_file, 0);
    if count < 0 {
        return None;
    }
    let mut nodes = vec![(0i16, 0i16); count as usize];
    let body = vec_file.get(4..).unwrap_or(&[]);
    for (k, node) in nodes.iter_mut().enumerate() {
        *node = (take_i16(body, 4 * k), take_i16(body, 4 * k + 2));
    }
    let matrix_file = files.open(2)?;
    let n = count as usize;
    let mut matrix = Vec::with_capacity(n);
    let mut pos = 0usize;
    for _ in 0..n {
        let mut row = Vec::with_capacity(n);
        for _ in 0..n {
            let mut slot = [0u8; 6];
            let avail = matrix_file.len().saturating_sub(pos).min(6);
            slot[..avail].copy_from_slice(&matrix_file[pos..pos + avail]);
            pos += avail;
            row.push((
                i32::from_le_bytes(slot[..4].try_into().unwrap()),
                i16::from_le_bytes(slot[4..].try_into().unwrap()),
            ));
        }
        matrix.push(row);
    }
    Some(NavData {
        grid: NavGrid {
            cells,
            width: w,
            height: h,
            scale,
        },
        nodes,
        node_count: count,
        matrix,
    })
}

/// Casilla de la tabla de ciudades `0x006DDB90` (52 bytes por casilla): rectángulo de 22×22 desde `(x, y)` y punto de anclaje.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TownSlot {
    pub x: i32,
    pub y: i32,
    pub anchor_x: i32,
    pub anchor_y: i32,
}

/// Valor devuelto cuando ninguna casilla contiene el punto.
pub const TOWN_NOT_FOUND: u32 = 0x28;

/// `0x00446D50`: busca, en el orden de `0x006DE4B8`, la casilla cuyo rectángulo contiene el punto (extremo izquierdo/superior
/// excluidos) o cuyo anclaje es el punto. Devuelve la posición en el orden (no el índice de casilla) o `0x28`; si `out` existe,
/// se escribe primero el punto de entrada y, al acertar, el anclaje.
pub fn find_town_at_point(
    slots: &[TownSlot],
    order: &[u8],
    count: u32,
    point: (i16, i16),
    out: Option<&mut (i16, i16)>,
) -> u32 {
    let mut out = out;
    if let Some(o) = out.as_deref_mut() {
        *o = point;
    }
    let (px, py) = (point.0 as i32, point.1 as i32);
    for k in 0..count {
        let s = &slots[order[k as usize] as usize];
        let inside = s.x + 0x16 > px && s.y + 0x16 > py && s.x < px && s.y < py;
        if inside || (s.anchor_x == px && s.anchor_y == py) {
            if let Some(o) = out {
                *o = (s.anchor_x as i16, s.anchor_y as i16);
            }
            return k;
        }
    }
    TOWN_NOT_FOUND
}

/// `0x00446F20`: sustituye la esquina `b` de la ruta `a → b → c` por dos puntos que la redondean.
///
/// Cada tramo se acorta hacia su punto medio una distancia `r = min(|ángulo|/180 · min(|ab|/2, |bc|/2), 10)` (en el original, el
/// ángulo es `180 − acos(d1x/|d1|)·180/π − acos(d2x/|d2|)·180/π` en grados; si su valor absoluto es menor que 0,5, la esquina es
/// recta y solo se añaden `a` (si `include_start` es falso) y `b`). Los resultados salen por `ftol(x + 0,5)`; si ambos puntos
/// coinciden solo se añade el primero.
///
/// Precondición: `a != b` y `b != c`. Con puntos coincidentes el original calcula `acos(NaN)` y entra en el manejador matemático
/// del CRT, que este puerto no reproduce (el corpus del oráculo los excluye). `acos` se toma de `f64`; el x87 usa `fpatan`
/// (menos de 1 ulp de diferencia), por eso las coincidencias se miden contra el x86 y no se afirman por construcción.
pub fn smooth_corner(a: (i16, i16), b: (i16, i16), c: (i16, i16), include_start: bool, out: &mut Vec<(i16, i16)>) {
    use crate::numeric::ftol_i32;
    const PI: f64 = 3.14159265358979;
    let f = |v: i16| v as f64 as f32;
    let (bx, by) = (f(b.0), f(b.1));
    let ma = (
        ((a.0 as i32 + b.0 as i32) as f64 / 2.0) as f32,
        ((a.1 as i32 + b.1 as i32) as f64 / 2.0) as f32,
    );
    let mc = (
        (f(c.0) as f64 + bx as f64) as f64 / 2.0,
        (f(c.1) as f64 + by as f64) as f64 / 2.0,
    );
    let mc = (mc.0 as f32, mc.1 as f32);
    let d1x = (bx as f64 - ma.0 as f64) as f32;
    let d1y = (by as f64 - ma.1 as f64) as f32;
    let d2x = (mc.0 as f64 - bx as f64) as f32;
    let d2y = (mc.1 as f64 - by as f64) as f32;
    let len1 = (d1y as f64 * d1y as f64 + d1x as f64 * d1x as f64).sqrt();
    let len2 = (d2y as f64 * d2y as f64 + d2x as f64 * d2x as f64).sqrt();
    let a1 = (d1x as f64 / len1).acos() * 180.0 / PI;
    let a2 = (d2x as f64 / len2).acos() * 180.0 / PI;
    let angle = 180.0 - a1 - a2;
    if angle.abs() < 0.5 {
        if !include_start {
            out.push(a);
        }
        out.push(b);
        return;
    }
    let t1 = (angle / 180.0).abs() as f32;
    let s1 = if d1x != 0.0 {
        (d1y as f64 / d1x as f64) as f32
    } else {
        0.0
    };
    let s2 = if d2x != 0.0 {
        (d2y as f64 / d2x as f64) as f32
    } else {
        0.0
    };
    let lm = if len1 < len2 { len1 } else { len2 };
    let mut r = t1 as f64 * lm;
    if r > 10.0 {
        r = 10.0;
    }
    let r2 = r * r;
    let split = |s: f32, dx: f32| -> (f64, f64) {
        let u = (r2 / (s as f64 * s as f64 + 1.0)).sqrt();
        let v = r2 - u * u;
        let mut w = if !(v < 0.0) { v.sqrt() } else { 0.0 };
        let mut u = u;
        if dx == 0.0 {
            u = 0.0;
            w = r;
        }
        (u, w)
    };
    let (u1, w1) = split(s1, d1x);
    let toward = |m: f32, centre: f32, d: f64| -> i32 {
        let v = if m <= centre || m.is_nan() {
            centre as f64 - d
        } else {
            centre as f64 + d
        };
        ftol_i32(v + 0.5)
    };
    let p1 = (toward(ma.0, bx, u1), toward(ma.1, by, w1));
    let (u2, w2) = split(s2, d2x);
    let p2 = (toward(mc.0, bx, u2), toward(mc.1, by, w2));
    let w = |v: i32| v as i16;
    out.push((w(p1.0), w(p1.1)));
    if (w(p1.0), w(p1.1)) != (w(p2.0), w(p2.1)) {
        out.push((w(p2.0), w(p2.1)));
    }
}
