//! `RoutePlanner::find_route` (`0x00445010`) y `build_route_from_points` (`0x00445D50`): rutas marítimas entre puntos del mundo.
//!
//! Ambas funciones comparten el cuerpo salvo en dos puntos: `find_route` devuelve la ruta precalculada de la tabla ciudad→ciudad
//! cuando los dos extremos caen en ciudades (y su pasada de suavizado de polilíneas salta puntos repetidos), mientras que
//! `build_route_from_points` siempre calcula. Referencia: x86 original en emulación (`reimpl/oracle/x86_planner.py`).
//!
//! Límites de este puerto (el original no los comprueba):
//! - los puntos consecutivos de la ruta deben ser distintos al llegar a `smooth_corner` (ver allí);
//! - `find_route` con una polilínea cuyos puntos son todos iguales deja en el original un valor de pila sin inicializar;
//! - con menos de dos puntos de entrada el original lee más allá de la lista;
//! - el resultado de fallo escribe el primer punto de la ruta `tabla[6 · ciudades]`, que debe existir.

use super::planner::{find_town_at_point, smooth_corner, NavData, NavGrid, TownSlot};
use std::sync::Arc;

type Point = (i16, i16);

/// Qué deja la función en el puntero de salida.
#[derive(Clone, Debug, PartialEq)]
pub enum RouteOut {
    /// Sin escribir (los fallos de ajuste a mar devuelven 0 sin tocar la salida).
    Unset,
    /// Ruta compartida de la tabla ciudad→ciudad (índice en `routes`); el primer punto de ese descriptor se reescribe.
    Shared(usize),
    /// Descriptor nuevo, con su búfer ajustado al tamaño.
    Built(Vec<Point>),
}

/// Resultado: `status` es AL (0 = ruta compartida o fallo, 1 = descriptor nuevo; decide quién libera la memoria).
#[derive(Clone, Debug, PartialEq)]
pub struct FoundRoute {
    pub status: u8,
    pub route: RouteOut,
}

/// Estado del planificador que leen estas funciones.
#[derive(Clone, Debug)]
pub struct RoutePlanner {
    /// Datos de navegación cargados. Son de solo lectura tras la carga, así que se comparten entre las copias del planificador
    /// (las copias transaccionales de cada tick no duplican la rejilla ni la matriz); `Arc::make_mut` los separa si hay que escribirlos.
    pub nav: Arc<NavData>,
    /// Costes de la matriz (`filas[a][b]`), derivados de `nav.matrix`; compartidos igual que `nav`.
    rows: Arc<Vec<Vec<i32>>>,
    pub town_slots: Vec<TownSlot>,
    pub town_order: Vec<u8>,
    /// Global `0x006DE4B0`.
    pub town_count: u32,
    /// Descriptores de ruta (primer punto reescribible); la tabla ciudad→ciudad apunta a ellos.
    /// Compartidos entre copias del planificador (copia al escribir, como `nav`).
    pub routes: Arc<Vec<Vec<Point>>>,
    /// `+0x10`: `origen · ciudades + destino` → índice en `routes` (`None` = puntero nulo).
    /// Compartida entre copias del planificador (son `ciudades²` casillas y el planificador se copia dos veces por tick);
    /// solo se sustituye entera al reconstruir o ampliar las tablas.
    pub town_table: Arc<Vec<Option<usize>>>,
    /// `+0x1E`: número de ciudades con tablas calculadas.
    pub registered: u16,
    /// `+0x20` y `+0x24`: por ciudad, cuántos puntos de mar cercanos se guardaron y los puntos (en celdas de la rejilla).
    pub sea_counts: Vec<u32>,
    pub sea_points: Arc<Vec<Vec<Point>>>,
    /// Global `0x006CBDC4`.
    pub line_counter: u32,
}

impl RoutePlanner {
    pub fn new(
        nav: NavData,
        town_slots: Vec<TownSlot>,
        town_order: Vec<u8>,
        town_count: u32,
        routes: Vec<Vec<Point>>,
        town_table: Vec<Option<usize>>,
    ) -> Self {
        let rows = Arc::new(nav.matrix.iter().map(|r| r.iter().map(|s| s.0).collect()).collect());
        RoutePlanner {
            nav: Arc::new(nav),
            rows,
            town_slots,
            town_order,
            town_count,
            routes: Arc::new(routes),
            town_table: Arc::new(town_table),
            registered: 0,
            sea_counts: Vec::new(),
            sea_points: Arc::new(Vec::new()),
            line_counter: 0,
        }
    }

    /// `((v − 0x44) · 1000 + escala/2) / escala` truncado a palabra: celda de la rejilla de una coordenada del mundo.
    fn world_to_cell(&self, v: i16) -> i16 {
        let s = self.nav.grid.scale;
        (((v as i32 - 0x44) * 1000 + (s >> 1)) / s) as i16
    }

    /// `(x · escala + 500) / 1000 + 0x44` truncado a palabra: coordenada del mundo de un nodo (en celdas).
    fn node_to_world(&self, node: usize) -> Point {
        let s = self.nav.grid.scale;
        let (nx, ny) = self.nav.nodes[node];
        let conv = |v: i16| ((v as i32 * s + 500) / 1000 + 0x44) as i16;
        (conv(nx), conv(ny))
    }

    /// Escribe el primer punto de la ruta `tabla[6 · ciudades]` (rama de fallo de los dos ajustes a mar).
    fn write_failure_point(&mut self, p: Point) {
        if let Some(&Some(id)) = self.town_table.get(6 * self.town_count as usize) {
            // Solo se separa la tabla compartida si el punto cambia de verdad.
            if self.routes[id].first().is_some_and(|first| *first != p) {
                Arc::make_mut(&mut self.routes)[id][0] = p;
            }
        }
    }

    /// `0x00445010`.
    pub fn find_route(&mut self, input: &[Point]) -> FoundRoute {
        self.plan(input, true)
    }

    /// `0x00445D50`.
    pub fn build_route_from_points(&mut self, input: &[Point]) -> FoundRoute {
        self.plan(input, false)
    }

    /// Ruta entre dos puntos del mundo ya convertidos y ajustados a celdas. `None` si algún extremo no cae en el mar.
    fn segment_nodes(&mut self, start: Point, end: Point) -> Option<Option<Vec<Point>>> {
        let mut g0 = (self.world_to_cell(start.0), self.world_to_cell(start.1));
        let mut g1 = (self.world_to_cell(end.0), self.world_to_cell(end.1));
        if !self.nav.grid.snap_to_sea(&mut g0, 0x32) || !self.nav.grid.snap_to_sea(&mut g1, 0x32) {
            return None;
        }
        // is_line_clear(rejilla, &ancho, fin = g0, inicio = g1)
        if self.nav.grid.is_line_clear(g1, g0, &mut self.line_counter) {
            return Some(None);
        }
        let (n0, n1) = self.nav.grid.select_nodes(
            (g0.0 as i32, g0.1 as i32),
            (g1.0 as i32, g1.1 as i32),
            &self.nav.nodes,
            &self.rows,
            &mut self.line_counter,
        );
        let (n0, n1) = (n0 as usize, n1 as usize);
        let mut path = vec![self.node_to_world(n0)];
        let mut k = n0;
        while k != n1 {
            k = self.nav.matrix[k][n1].1 as usize;
            path.push(self.node_to_world(k));
        }
        Some(Some(path))
    }

    fn plan(&mut self, input: &[Point], town_shortcut: bool) -> FoundRoute {
        let mut route: Vec<Point> = Vec::new();
        if input.len() < 3 {
            let (p0, p1) = (input[0], input[1]);
            let t0 = find_town_at_point(&self.town_slots, &self.town_order, self.town_count, p0, None);
            let t1 = find_town_at_point(&self.town_slots, &self.town_order, self.town_count, p1, None);
            if town_shortcut && t0 < self.town_count && t1 < self.town_count {
                let id = self.town_table[(self.town_count * t0 + t1) as usize]
                    .expect("puntero nulo en la tabla ciudad→ciudad");
                if self.routes[id][0] != p0 {
                    Arc::make_mut(&mut self.routes)[id][0] = p0;
                }
                return FoundRoute {
                    status: 0,
                    route: RouteOut::Shared(id),
                };
            }
            match self.segment_nodes(p0, p1) {
                None => {
                    self.write_failure_point(p0);
                    return FoundRoute {
                        status: 0,
                        route: RouteOut::Unset,
                    };
                }
                Some(None) => {
                    route.push(p0);
                    route.push(p1);
                }
                Some(Some(path)) => {
                    route.push(p0);
                    let mut poly = vec![p0];
                    poly.extend(path);
                    poly.push(p1);
                    for i in 1..poly.len() - 1 {
                        smooth_corner(poly[i - 1], poly[i], poly[i + 1], false, &mut route);
                    }
                    route.push(p1);
                }
            }
        } else {
            let mut t: Vec<Point> = vec![input[0]];
            for i in 1..input.len() {
                let (s, e) = (input[i - 1], input[i]);
                match self.segment_nodes(s, e) {
                    None => {
                        self.write_failure_point(s);
                        return FoundRoute {
                            status: 0,
                            route: RouteOut::Unset,
                        };
                    }
                    Some(None) => t.push(e),
                    Some(Some(path)) => {
                        t.extend(path);
                        t.push(e);
                    }
                }
            }
            route.push(t[0]);
            let cnt = t.len();
            let last;
            if town_shortcut {
                // find_route: salta los puntos repetidos al buscar el segundo y el siguiente
                let mut a = t[0];
                let mut j = 1;
                while j < cnt && t[j] == a {
                    j += 1;
                }
                last = t[cnt - 1];
                if j < cnt {
                    let mut b = t[j];
                    for &c in &t[2..cnt] {
                        if c != b {
                            smooth_corner(a, b, c, true, &mut route);
                            a = b;
                            b = c;
                        }
                    }
                }
            } else {
                let (mut a, mut b) = (t[0], t[1]);
                for &c in &t[2..cnt] {
                    smooth_corner(a, b, c, true, &mut route);
                    a = b;
                    b = c;
                }
                last = t[cnt - 1];
            }
            route.push(last);
        }
        // Elimina puntos consecutivos repetidos (memmove que quita el anterior y no revisa el siguiente).
        let mut prev = (-1i32, -1i32);
        let mut i = 0;
        while i < route.len() {
            let cur = (route[i].0 as i32, route[i].1 as i32);
            if cur == prev && i > 0 {
                route.remove(i - 1);
            }
            prev = cur;
            i += 1;
        }
        FoundRoute {
            status: 1,
            route: RouteOut::Built(route),
        }
    }
}

/// Rectángulos fijos `0x0066D648` (`x0, y0, x1, y1` del mundo) donde no se guardan puntos de mar.
const SEA_EXCLUDED: [(i32, i32, i32, i32); 3] =
    [(442, 1521, 658, 1691), (2265, 147, 2332, 453), (1497, 1263, 1578, 1444)];

impl RoutePlanner {
    /// Puerto de una casilla de la tabla de ciudades: punto de anclaje como palabras.
    fn port_of(&self, slot: u8) -> Point {
        let s = &self.town_slots[slot as usize];
        (s.anchor_x as i16, s.anchor_y as i16)
    }

    /// `0x00448780`: busca para la ciudad de la posición `town` (en el orden de `0x006DE4B8`) hasta diez puntos de mar cercanos
    /// accesibles, avanzando por anillos de radio 200, 225, 250… y umbral de longitud 6000 + 750 por anillo.
    ///
    /// Recorre la rejilla de 16 en 16 celdas, ajusta cada celda a mar (radio 7), descarta rectángulos fijos y zonas de ciudades, mide
    /// la longitud de la ruta hasta el puerto (`29 · lado mayor + 12 · lado menor` por tramo) y guarda los puntos (en celdas) que no
    /// superan el umbral por delante de los demás. El original no tiene tope de anillos: `max_rings` evita que un mapa sin
    /// suficientes puntos no termine (devuelve `false` sin tocar las tablas).
    pub fn compute_town_nearby_sea_points(&mut self, town: u8, max_rings: u32) -> bool {
        let (w, h) = (self.nav.grid.width as i32, self.nav.grid.height as i32);
        let total_cells = w * h;
        let cap = (((total_cells + ((total_cells >> 31) & 0xFF)) >> 8) as usize).max(1);
        let mut a: Vec<Point> = vec![(0, 0); cap];
        let mut b: Vec<u32> = vec![0; cap];
        let port = self.port_of(self.town_order[town as usize]);
        let (mut outer, mut inner, mut thr) = (200i32, 0i32, 6000u32);
        let mut kept = 0usize;
        let mut total = 0usize;
        let mut ring = 0;
        loop {
            // pasada previa: los pendientes que ahora caben bajo el umbral pasan delante
            let mut idx = kept;
            while idx < total {
                if b[idx] < thr {
                    if idx != kept {
                        a.swap(kept, idx);
                        b[idx] = b[kept];
                    }
                    kept += 1;
                }
                idx += 1;
            }
            let mut x = 0i32;
            while x < w {
                let mut y = 0i32;
                while y < h {
                    let mut cell = (x as i16, y as i16);
                    if self.nav.grid.snap_to_sea(&mut cell, 7) {
                        let s = self.nav.grid.scale;
                        let conv = |v: i16| ((v as i32 * s + 500) / 1000 + 0x44) as i16;
                        let wp = (conv(cell.0), conv(cell.1));
                        let (dx, dy) = (wp.0 as i32 - port.0 as i32, wp.1 as i32 - port.1 as i32);
                        let d2 = (dy * dy + dx * dx) as u32;
                        if d2 <= (outer * outer) as u32 && ((inner * inner) as u32) < d2 {
                            let (wx, wy) = (wp.0 as i32, wp.1 as i32);
                            let in_rect = SEA_EXCLUDED
                                .iter()
                                .any(|r| r.0 < wx && r.1 < wy && wx < r.2 && wy < r.3);
                            let near_town = self.town_slots.iter().take(40).any(|t| {
                                (t.x < wx && t.y < wy && wx < t.x + 0x16 && wy < t.y + 0x16)
                                    || (t.anchor_x == wx && t.anchor_y == wy)
                            });
                            if !in_rect && !near_town {
                                let found = self.find_route(&[wp, port]);
                                let pts: Option<&[Point]> = match &found.route {
                                    RouteOut::Unset => None,
                                    RouteOut::Shared(id) => Some(&self.routes[*id]),
                                    RouteOut::Built(r) => Some(r),
                                };
                                if let Some(pts) = pts {
                                    let mut score = 0u32;
                                    for pair in pts.windows(2) {
                                        let dx = (pair[0].0 as i32 - pair[1].0 as i32).abs();
                                        let dy = (pair[0].1 as i32 - pair[1].1 as i32).abs();
                                        let (big, small) = if dx < dy { (dy, dx) } else { (dx, dy) };
                                        score = score.wrapping_add((big * 0x1d + small * 0xc) as u32);
                                    }
                                    if score < thr {
                                        total += 1;
                                        if a.len() < total {
                                            a.resize(total, (0, 0));
                                            b.resize(total, 0);
                                        }
                                        a[total - 1] = a[kept];
                                        b[total - 1] = b[kept];
                                        a[kept] = cell;
                                        kept += 1;
                                    } else {
                                        if a.len() <= total {
                                            a.resize(total + 1, (0, 0));
                                            b.resize(total + 1, 0);
                                        }
                                        a[total] = cell;
                                        b[total] = score;
                                        total += 1;
                                    }
                                }
                            }
                        }
                    }
                    y += 16;
                }
                x += 16;
            }
            inner = outer;
            outer += 25;
            thr += 0x2ee;
            if kept > 9 {
                if self.sea_counts.len() <= town as usize {
                    self.sea_counts.resize(town as usize + 1, 0);
                    Arc::make_mut(&mut self.sea_points).resize(town as usize + 1, Vec::new());
                }
                self.sea_counts[town as usize] = kept as u32;
                Arc::make_mut(&mut self.sea_points)[town as usize] = a[..kept].to_vec();
                return true;
            }
            ring += 1;
            if ring >= max_rings {
                return false;
            }
        }
    }

    /// Copia el resultado de `build_route_from_points` a una casilla de tabla (la variable de salida conserva el valor anterior si
    /// la llamada falla, por eso se arrastra entre iteraciones).
    fn route_slot(&mut self, last: &mut Option<usize>, pts: [Point; 2]) -> Option<usize> {
        if let RouteOut::Built(r) = self.build_route_from_points(&pts).route {
            Arc::make_mut(&mut self.routes).push(r);
            *last = Some(self.routes.len() - 1);
        }
        *last
    }

    /// `0x004482C0`: reconstruye la tabla `n × n` de rutas ciudad→ciudad con `build_route_from_points` entre los puertos de cada
    /// par y los puntos de mar cercanos de cada ciudad. `max_rings` como en `compute_town_nearby_sea_points`.
    pub fn build_town_route_table(&mut self, max_rings: u32) -> bool {
        let n = self.town_count as usize;
        self.town_table = Arc::new(vec![None; n * n]);
        self.registered = n as u16;
        let mut last: Option<usize> = None;
        for i in 0..n {
            let pi = self.port_of(self.town_order[i]);
            for j in 0..n {
                let pj = self.port_of(self.town_order[j]);
                let slot = self.route_slot(&mut last, [pi, pj]);
                Arc::make_mut(&mut self.town_table)[n * i + j] = slot;
            }
        }
        self.sea_counts = vec![0; n];
        self.sea_points = Arc::new(vec![Vec::new(); n]);
        (0..n).all(|t| self.compute_town_nearby_sea_points(t as u8, max_rings))
    }

    /// `0x00448040`: amplía las tablas al añadir una ciudad nueva (casilla `new_slot`, la última de `town_order`).
    pub fn add_town_routes(&mut self, new_slot: u8, max_rings: u32) -> bool {
        let n = self.town_count as usize;
        let old = self.town_table.clone();
        let mut table: Vec<Option<usize>> = vec![None; n * n];
        let mut last: Option<usize> = None;
        let newport = self.port_of(new_slot);
        for r in 0..n.saturating_sub(1) {
            for c in 0..n - 1 {
                table[n * r + c] = old.get((n - 1) * r + c).copied().flatten();
            }
            let pr = self.port_of(self.town_order[r]);
            table[n * r + n - 1] = self.route_slot(&mut last, [pr, newport]);
        }
        for c in 0..n {
            let pc = self.port_of(self.town_order[c]);
            table[(n - 1) * n + c] = self.route_slot(&mut last, [newport, pc]);
        }
        let old_counts = std::mem::take(&mut self.sea_counts);
        let old_points = std::mem::take(&mut self.sea_points);
        let old_registered = self.registered as usize;
        self.sea_counts = vec![0; n];
        self.sea_points = Arc::new(vec![Vec::new(); n]);
        self.town_table = Arc::new(table);
        for t in 0..n {
            if t < old_registered {
                self.sea_counts[t] = old_counts[t];
                Arc::make_mut(&mut self.sea_points)[t] = old_points[t].clone();
            } else if !self.compute_town_nearby_sea_points(t as u8, max_rings) {
                return false;
            }
        }
        self.registered = n as u16;
        true
    }
}

/// Anillo de ocho puntos alrededor de un puerto (`0x0066D660`, desplazamientos `(x, y)` de palabras con signo).
pub const PORT_RING: [Point; 8] = [
    (0, -40),
    (26, -26),
    (40, 0),
    (26, 26),
    (0, 40),
    (-26, 26),
    (-40, 0),
    (-26, -26),
];

/// `0x00447580`: inserta `a` en `route` en el índice `p5` y, si hace falta, el tramo del anillo del puerto `origin` que va del punto
/// del anillo más cercano a `a` al más cercano a `route[p4]`; los puntos entre `p5` y `p4` se sustituyen. Elige el sentido con
/// menos puntos del anillo entre los que no cruzan celdas de tierra; si ninguno es transitable, o los dos puntos comparten el mismo
/// tramo del anillo, solo inserta `a`. Requiere `p5 <= p4 < route.len()` (el original reserva memoria con esa suposición).
///
/// Límites del original conservados: un obstáculo en el último punto comprobado cuenta como transitable; los recuentos de puntos del
/// anillo se comparan como `u16`; y las celdas se calculan con la escala fija `0xDD0` (no la global).
pub fn insert_port_ring(nav: &NavGrid, origin: Point, route: &[Point], a: Point, p4: usize, p5: usize) -> Vec<Point> {
    let b = route[p4];
    let dist = |p: Point, k: usize| -> u32 {
        let dx = (p.0 as i32 - PORT_RING[k].0 as i32) - origin.0 as i32;
        let dy = (p.1 as i32 - PORT_RING[k].1 as i32) - origin.1 as i32;
        dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx)) as u32
    };
    // Pareja del anillo de `a`: mejor y segundo.
    let (mut best_d, mut best, mut second) = (u32::MAX, 0i32, 0i32);
    let mut second_d = u32::MAX;
    for k in 0..8 {
        let d = dist(a, k);
        if d < best_d {
            second_d = best_d;
            second = best;
            best_d = d;
            best = k as i32;
        } else if d < second_d {
            second_d = d;
            second = k as i32;
        }
    }
    let swap = (second < best && second != 0 && best != 7)
        || (second == 6 && best == 7)
        || (second == 7 && best == 0)
        || (second == 0 && best == 1);
    let (a0, a1) = if swap { (second, best) } else { (best, second) };
    // Pareja del anillo de `b`.
    let (mut bd, mut bb, mut bs) = (u32::MAX, 0i32, 0i32);
    let mut sd = u32::MAX;
    for k in 0..8 {
        let d = dist(b, k);
        if d < bd {
            bs = bb;
            sd = bd;
            bd = d;
            bb = k as i32;
        } else if d < sd {
            bs = k as i32;
            sd = d;
        }
    }
    let swap = (bb < bs && bs != 7 && bb != 0) || (bs == 7 && bb == 6) || (bs == 0 && bb == 7) || (bs == 1 && bb == 0);
    let (mut b0, mut b1) = if swap { (bs, bb) } else { (bb, bs) };

    let only_a = || -> Vec<Point> {
        let mut out = route[..p5].to_vec();
        out.push(a);
        out.extend_from_slice(&route[p5..]);
        out
    };
    if a0 == b1 || a1 == b0 {
        return only_a();
    }
    let ring_point = |k: i32| -> Point {
        let r = k.rem_euclid(8) as usize;
        (
            (PORT_RING[r].0 as i32 + origin.0 as i32) as i16,
            (PORT_RING[r].1 as i32 + origin.1 as i32) as i16,
        )
    };
    let blocked = |k: i32| -> bool {
        let r = k.rem_euclid(8) as usize;
        let sx = (PORT_RING[r].0 as i32 + origin.0 as i32 - 0x44) as i16;
        let sy = (PORT_RING[r].1 as i32 + origin.1 as i32 - 0x44) as i16;
        let cx = (sx as i32 * 1000 + 0x6E8) / 0xDD0;
        let cy = (sy as i32 * 1000 + 0x6E8) / 0xDD0;
        let (w, h) = (nav.width as i32, nav.height as i32);
        cx >= w || cy >= h || cy < 0 || cx < 0 || sy < 0 || sx < 0 || nav.cells[(w * cy + cx) as usize] != 0
    };
    if a0 < b0 || a0 == 0 {
        b0 -= 8;
    }
    if b1 < a1 || b1 == 0 {
        b1 += 8;
    }
    let mut count_down: i32 = 0;
    let mut count_up: i32 = 0;
    if b0 <= a0 {
        count_down = a0 - b0 + 1;
    }
    let forward = if b1 < a1 {
        true
    } else {
        count_up = b1 - a1 + 1;
        (count_up as u16) <= (count_down as u16)
    };
    if forward {
        if a1 <= b1 && (a1..=b1).any(|k| blocked(k)) {
            let mut u = a0;
            loop {
                if u < b0 || blocked(u) {
                    break;
                }
                u -= 1;
            }
            if b0 < u {
                return only_a();
            }
            count_up = 0xFFFF;
        }
    } else if b0 <= a0 {
        let mut hit = false;
        for k in (b0..=a0).rev() {
            if blocked(k) {
                hit = true;
                break;
            }
        }
        if hit {
            let mut u = a1;
            loop {
                if blocked(u) {
                    break;
                }
                u += 1;
                if u > b1 {
                    break;
                }
            }
            if u < b1 {
                return only_a();
            }
            count_down = 0xFFFF;
        }
    }
    let mut out = route[..p5].to_vec();
    out.push(a);
    if (count_down as u16) < (count_up as u16) {
        if b0 <= a0 {
            out.extend((b0..=a0).rev().map(ring_point));
        }
    } else {
        out.extend((a1..=b1).map(ring_point));
    }
    out.extend_from_slice(&route[p4..]);
    out
}
