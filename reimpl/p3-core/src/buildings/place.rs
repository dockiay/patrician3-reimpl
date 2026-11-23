//! Colocar un edificio en el plano: marcar su huella, reservar parcelas, trazar calles y conectar el
//! edificio con la red de caminos. Reimplementa `town_plan_place_building` y las funciones que llama.

use super::grid::{Grid, FOOTPRINT_H, FOOTPRINT_KIND, FOOTPRINT_W};
use super::tables::{NEIGHBOUR_START, NEIGHBOUR_STREAM};
use super::Site;
use crate::numeric::idiv;
use crate::town::{off, Town};

/// Entradas de la cola de la búsqueda de caminos (`0xFA0` bytes de 4).
const QUEUE_ENTRIES: usize = 1000;

/// Puntuación de una celda vecina al asfaltar: 1 si está ocupada (`>= 0x20`), 3 si es suelo bajo
/// (`< 5`), −5 en otro caso.
fn road_score(v: u8) -> i32 {
    let v = v & 0x3F;
    if v >= 0x20 {
        1
    } else if v < 5 {
        3
    } else {
        -5
    }
}

/// Umbrales de longitud de borde por clase de huella para asfaltar junto al edificio (los dos bytes
/// que el original rellena en la pila). Las clases 2 y 5 quedan sin inicializar allí: no hay valor
/// reproducible y aquí no asfaltan.
fn border_thresholds(kind: usize) -> Option<(i32, i32)> {
    match kind {
        0 => Some((1, 4)),
        1 => Some((4, 1)),
        3 => Some((1, 4)),
        4 => Some((3, 1)),
        _ => None,
    }
}

impl Grid<'_> {
    fn cell(&self, row: i64, col: i64) -> i64 {
        self.stride as i64 * row + col
    }

    /// `town_plan_stamp_defense_cells` (0x004FBD60): marca con `5` tres celdas de la fila y la que
    /// queda debajo (columna impar) o encima (par).
    pub fn stamp_defense_cells(&mut self, row: i32, col: i32) {
        let s = self.stride as i64;
        let idx = self.cell(row as i64, col as i64);
        for d in [0, -1, -2] {
            self.set(idx + d, 5);
        }
        let at = if col & 1 == 1 { idx - 1 + s } else { idx - 1 - s };
        self.set(at, 5);
    }

    /// `town_plan_update_lot_pair_flags` (0x004FC130): recorre las celdas desde la fila `row` y marca
    /// con `8` y `4` las que forman pareja con la celda de arriba a la izquierda o de abajo.
    pub fn update_lot_pair_flags(&mut self, row: i32) {
        let s = self.stride as i64;
        let remaining = self.rows - row;
        let mut n = if remaining >= 12 { 9 } else { remaining - 3 };
        let first;
        if row < 10 {
            n = n + row - 4;
            first = 3;
        } else {
            first = row - 7;
            n += 6;
        }
        let count = s * n as i64;
        let start = s * first as i64;
        for k in 0..count.max(0) {
            let p = start + k;
            let c = self.get(p);
            if c & 0x21 == 0x21 && self.get(p - s - 2) & 0x21 == 0x21 {
                self.set(p, c | 8);
            }
            let c = self.get(p);
            if c & 0x22 == 0x22 && self.get(p + s - 2) & 0x22 == 0x22 {
                self.set(p, c | 4);
            }
        }
    }

    /// `town_plan_can_place_building_at` (0x004FD020).
    pub fn can_place_building_at(&self, ty: u32, row: u32, col: u32, flag: u8) -> u8 {
        let c = self.get(self.cell((row & 0xFF) as i64, (col & 0xFF) as i64));
        if (0x15..=0x1E).contains(&ty) {
            let mut r = 0;
            if c & 0xA1 == 0xA1 {
                r = 1;
            }
            if c & 0xA2 == 0xA2 {
                r += 2;
            }
            return r;
        }
        let kind = FOOTPRINT_KIND[ty as usize];
        if kind >= 6 {
            return 0;
        }
        let mut mask = (1u8 << kind) | 0x20;
        if flag != 0 {
            mask |= 0x80;
        }
        (c & mask == mask) as u8
    }

    /// `town_plan_cell_flags_0x30_set` (0x004FCBA0).
    pub fn cell_flags_0x30_set(&self, a: u32, b: u32) -> bool {
        self.get(self.cell((a & 0xFF) as i64, (b & 0xFF) as i64)) & 0x30 == 0x30
    }

    /// `town_plan_clear_fit_flags_around` (0x004FBEC0): quita los bits de "cabe la huella `e`"
    /// (0–4) de las celdas que una huella de clase `kind` con esquina en (`row`, `col`) puede tapar.
    pub fn clear_fit_flags_around(&mut self, row: i32, col: i32, kind: usize) {
        let s = self.stride as i64;
        let parity = (col & 1) as i64;
        let (wk, hk) = (FOOTPRINT_W[kind & 0xFF] as i64, FOOTPRINT_H[kind & 0xFF] as i64);
        for e in 0..5usize {
            let (we, he) = (FOOTPRINT_W[e] as i64, FOOTPRINT_H[e] as i64);
            let columns = we + wk - 1;
            let lines = hk + he - 1;
            let edx = we + he + col as i64 - 2;
            let rowpart = (parity - he + 2 * row as i64 + we) >> 1;
            let mut ptr = s * rowpart + edx;
            let mut flag = (!edx) & 1;
            let keep = !(1u8 << e);
            let mut remaining = lines;
            while remaining > 0 {
                let mut q = ptr;
                let mut f = flag;
                for _ in 0..columns.max(0) {
                    let c = self.get(q);
                    if c & 0x20 != 0 {
                        self.set(q, c & keep);
                    }
                    q = if f != 0 { q - 1 - s } else { q - 1 };
                    f = (f == 0) as i64;
                }
                ptr = if flag != 0 { ptr - 1 } else { ptr + s - 1 };
                flag = (flag == 0) as i64;
                remaining -= 1;
            }
        }
    }

    /// `town_plan_restore_fit_flags_around` (0x004FBFF0): vuelve a poner los bits de encaje en las
    /// celdas vecinas de la tabla de clase `kind` que sigan libres para esa huella.
    pub fn restore_fit_flags_around(&mut self, row: i32, col: i32, kind: usize) {
        let s = self.stride as i64;
        let start = if kind & 0xFF != 0 {
            NEIGHBOUR_START[kind & 0xFF] as usize
        } else {
            0
        };
        let end = NEIGHBOUR_START[(kind & 0xFF) + 1] as usize;
        let flag80 = self.get(self.cell(row as i64, col as i64)) & 0x80;
        let mut p = start;
        while p < end {
            let k = NEIGHBOUR_STREAM[p] as u8 as usize;
            let d1 = NEIGHBOUR_STREAM[p + 1] as i32;
            let d2 = NEIGHBOUR_STREAM[p + 2] as i32;
            p += 3;
            let (mut r, mut c) = (row, col);
            if d1 != 0 {
                r += if d1 > 0 { d1 & c & 1 } else { -((c - 1) & d1 & 1) };
                r += d1 / 2;
                c += d1;
            }
            if d2 != 0 {
                r += if d2 > 0 { -((c - 1) & d2 & 1) } else { d2 & c & 1 };
                r += -(d2 / 2);
                c += d2;
            }
            let at = self.cell(r as i64, c as i64);
            if self.get(at) & flag80 == flag80 && self.can_fit_footprint(r, c, k) {
                let v = self.get(at);
                self.set(at, v | (1u8 << k));
            }
        }
        let _ = s;
    }

    /// `town_plan_pave_road_cell` (0x004FC630): de las cuatro celdas candidatas junto a `idx` elige
    /// (por puntuación de sus vecinas, ganando el primer máximo) la que se asfalta y marca tres
    /// celdas contiguas y una más de la fila de al lado.
    pub fn pave_road_cell(&mut self, idx: i32) {
        let s = self.stride as i64;
        let idx = idx as i64;
        let p = idx & 1;
        let at = |g: &Self, i: i64| road_score(g.get(i));
        let cand = [idx, idx + 1 + p * s, idx + 2, idx + 1 - (1 - p) * s];
        let s0 = at(self, idx - 2);
        let s1 = at(self, idx - 1 + if p != 0 { s } else { 0 });
        let s2 = at(self, idx + s);
        let s3 = at(self, idx + 1 + if p != 0 { s } else { 0 });
        let s4 = at(self, idx + 2);
        let s5 = at(self, idx + 1 - if p == 0 { s } else { 0 });
        let s6 = at(self, idx - s);
        let sums = [s0 + s1 + s5, s1 + s2 + s3, s3 + s4 + s5, 2 * s5 + s6];
        let (mut best, mut pos) = (sums[0], cand[0]);
        for k in 1..4 {
            if best < sums[k] {
                best = sums[k];
                pos = cand[k];
            }
        }
        let road = |g: &mut Self, i: i64| {
            let c = g.get(i);
            let t = c & 0x3F;
            if t >= 0x20 || t == 4 {
                g.set(i, (c & 0xC2) | 2);
            }
        };
        road(self, pos);
        road(self, pos - 1);
        road(self, pos - 2);
        let n = if pos & 1 != 0 { s + pos - 1 } else { pos - s - 1 };
        road(self, n);
        let row = idiv(n as i32, self.stride) as u8 as i32;
        let col = n as i32 - row * self.stride;
        self.clear_fit_flags_around(row, col, 6);
    }

    fn clear_marks(&mut self, queue: &[i16]) {
        for pair in queue.chunks(2).rev() {
            let c = self.get(pair[0] as i64);
            self.set(pair[0] as i64, c & 0xBF);
        }
    }

    /// `town_plan_bfs_find_road_path` (0x004FC4A0): búsqueda en anchura desde las celdas de `queue`
    /// (pares celda, padre) hasta una vecina de suelo `<= 3`. Devuelve el desplazamiento (en `i16`)
    /// de la celda desde la que se alcanza, o −1.
    pub fn bfs_find_road_path(&mut self, queue: &mut Vec<i16>) -> i32 {
        let s = self.stride as i64;
        let limit = QUEUE_ENTRIES * 2;
        for pair in queue.chunks(2) {
            let c = self.get(pair[0] as i64);
            self.set(pair[0] as i64, c | 0x40);
        }
        let mut read = 0usize;
        let mut counter = 0i32;
        loop {
            if read >= queue.len() {
                self.clear_marks(queue);
                return -1;
            }
            let c = queue[read] as i64 + 1;
            let west = c - 2;
            let d = if c & 1 != 0 { west - s } else { west + s };
            let neighbours = [c, west, d, d + 2];
            for &nb in &neighbours {
                let cell = self.get(nb);
                let v = cell & 0x7F;
                if v <= 3 {
                    self.clear_marks(queue);
                    return counter;
                }
                if v & 0x60 == 0x20 {
                    self.set(nb, cell | 0x40);
                } else if v != 4 {
                    continue;
                }
                queue.push(nb as i16);
                queue.push(counter as i16);
                if queue.len() >= limit {
                    self.clear_marks(queue);
                    return -1;
                }
            }
            counter += 2;
            read += 2;
            if read >= limit {
                self.clear_marks(queue);
                return -1;
            }
        }
    }

    /// Asfalta la ruta que devolvió la búsqueda: las celdas de la cadena de padres y la última.
    fn pave_found_path(&mut self, queue: &[i16], found: i32) {
        if found < 0 {
            return;
        }
        let last = queue[found as usize] as i32;
        let mut p = queue[found as usize + 1] as i32;
        while p >= 0 {
            self.pave_road_cell(queue[p as usize] as i32);
            p = queue[p as usize + 1] as i32;
        }
        if last >= 0 && (last as i64) < self.rows as i64 * self.stride as i64 {
            self.pave_road_cell(last);
        }
    }

    /// `town_plan_pave_road_from_cell` (0x004FC8C0): desde `cell` abre una calle hasta la red.
    pub fn pave_road_from_cell(&mut self, cell: i32) {
        if self.get(cell as i64) & 0x20 == 0 {
            return;
        }
        let mut queue = vec![cell as i16, -1];
        let found = self.bfs_find_road_path(&mut queue);
        self.pave_found_path(&queue, found);
        let c = self.get(cell as i64);
        if c & 0x20 == 0x20 {
            self.set(cell as i64, c & 0x80);
            let c = self.get(cell as i64);
            self.set(cell as i64, c | 2);
        }
    }

    /// `town_plan_pave_roads_beside_building` (0x004FC1D0): recorre los bordes del edificio y abre
    /// calles donde el borde de suelo `0x0B` es lo bastante largo.
    pub fn pave_roads_beside_building(&mut self, row: i32, col: i32, kind: usize) {
        let Some((t1, t2)) = border_thresholds(kind) else {
            return;
        };
        let s = self.stride as i64;
        let (w, h) = (FOOTPRINT_W[kind] as i64, FOOTPRINT_H[kind] as i64);
        let (row, col) = (row as i64, col as i64);
        let p = col & 1;
        let mut a = (row + p - 1) * s + col + 1;
        let mut b = (row + ((h & col) & 1) + (h >> 1)) * s - h + col;
        for _ in 0..w.max(0) {
            let mut run = 0;
            let mut ea = a;
            a -= 1;
            if a & 1 != 0 {
                a -= s;
            }
            let mut eb = b;
            b -= 1;
            if b & 1 != 0 {
                b -= s;
            }
            while self.get(ea) & 0x3F == 0xB {
                run += 1;
                ea += 1;
                if ea & 1 != 0 {
                    ea -= s;
                }
            }
            let mut saved_b = eb;
            if self.get(eb) & 0x3F == 0xB {
                loop {
                    let odd = eb & 1;
                    run += 1;
                    eb -= 1;
                    if odd != 0 {
                        eb += s;
                    }
                    if self.get(eb) & 0x3F != 0xB {
                        break;
                    }
                }
                saved_b = eb;
            }
            let eb = saved_b;
            if run >= t1 {
                if self.get(ea) & 0x20 != 0 {
                    let mut n = ea + 1;
                    if n & 1 != 0 {
                        n -= s;
                    }
                    if self.get(n) & 0x20 != 0 {
                        self.pave_road_from_cell(ea as i32);
                    }
                }
                if self.get(eb) & 0x20 != 0 {
                    let n = eb - 1 + if eb & 1 != 0 { s } else { 0 };
                    if self.get(n) & 0x20 != 0 {
                        self.pave_road_from_cell(eb as i32);
                    }
                }
            }
        }
        let mut c = (row + p) * s + col + 1;
        let mut d = (row - (((col - 1) & w) & 1) - (w >> 1)) * s - w + col;
        for _ in 0..h.max(0) {
            let mut run = 0;
            let mut ec = c;
            c -= 1;
            if ec & 1 != 0 {
                c += s;
            }
            let mut ed = d;
            d = d - 1 + if d & 1 != 0 { s } else { 0 };
            while self.get(ec) & 0x3F == 0xB {
                let odd = ec & 1;
                run += 1;
                ec += 1;
                if odd != 0 {
                    ec += s;
                }
            }
            while self.get(ed) & 0x3F == 0xB {
                run += 1;
                ed -= 1;
                if ed & 1 != 0 {
                    ed -= s;
                }
            }
            if run >= t2 {
                if self.get(ec) & 0x20 != 0 {
                    let n = ec + 1 + if ec & 1 != 0 { s } else { 0 };
                    if self.get(n) & 0x20 != 0 {
                        self.pave_road_from_cell(ec as i32);
                    }
                }
                if self.get(ed) & 0x20 != 0 {
                    let mut n = ed - 1;
                    if n & 1 != 0 {
                        n -= s;
                    }
                    if self.get(n) & 0x20 != 0 {
                        self.pave_road_from_cell(ed as i32);
                    }
                }
            }
        }
    }

    /// `town_plan_connect_building_to_road` (0x004FC990): sale de todo el perímetro del edificio;
    /// si ya toca un camino (`< 5`) no hace nada, y si no abre la calle más corta.
    pub fn connect_building_to_road(&mut self, row: i32, col: i32, kind: usize) {
        if kind == 5 {
            return;
        }
        let s = self.stride as i64;
        let (w, h) = (FOOTPRINT_W[kind] as i64 - 1, FOOTPRINT_H[kind] as i64 - 1);
        let mut cells: Vec<i16> = Vec::new();
        let first = (s as i16).wrapping_mul(row as i16).wrapping_add(col as i16);
        let mut cur = (if col & 1 == 0 {
            first.wrapping_sub(s as i16)
        } else {
            first
        })
        .wrapping_add(1) as i64;
        cells.push(cur as i16);
        let step = |cur: i64, f: &dyn Fn(i64) -> i64| f(cur);
        for _ in 0..w.max(0) {
            cur = step(cur, &|c| if c & 1 == 0 { c - s - 1 } else { c - 1 });
            cells.push(cur as i16);
        }
        cur -= 2;
        cells.push(cur as i16);
        for _ in 0..h.max(0) {
            cur = if cur & 1 != 0 { s + cur - 1 } else { cur - 1 };
            cells.push(cur as i16);
        }
        cur += s;
        cells.push(cur as i16);
        for _ in 0..w.max(0) {
            cur = if cur & 1 != 0 { s + cur + 1 } else { cur + 1 };
            cells.push(cur as i16);
        }
        cur += 2;
        cells.push(cur as i16);
        for _ in 0..h.max(0) {
            cur = cur + 1 - if cur & 1 == 0 { s } else { 0 };
            cells.push(cur as i16);
        }
        let mut queue: Vec<i16> = Vec::new();
        for &c in &cells {
            queue.push(c);
            queue.push(-1);
        }
        // El original reserva una entrada más tras la última del perímetro; no se lee.
        for pair in queue.chunks(2) {
            if self.get(pair[0] as i64) & 0x7F < 5 {
                return;
            }
        }
        let found = self.bfs_find_road_path(&mut queue);
        self.pave_found_path(&queue, found);
    }
}

impl Site {
    pub(super) fn grid(&mut self, t: &Town) -> Grid<'_> {
        Grid {
            stride: t.i32(off::PLAN_STRIDE),
            rows: t.i32(off::PLAN_ROWS),
            cells: &mut self.cells,
        }
    }

    /// `town_plan_mark_2x2_occupied` (0x004FBDB0): ocupa un bloque de 2×2 con `0x0B` y avisa del cambio.
    pub fn mark_2x2_occupied(&mut self, t: &mut Town, row: i32, col: i32) {
        let mut g = self.grid(t);
        let (mut r0, mut c0) = (row as i64, col as i64);
        for _ in 0..2 {
            let (mut r, mut c) = (r0, c0);
            for _ in 0..2 {
                let at = g.cell(r, c);
                let v = g.get(at);
                g.set(at, (v & 0x8B) | 0x0B);
                r += c & 1;
                c -= 1;
            }
            c0 -= 1;
            r0 -= c0 & 1;
        }
        g.clear_fit_flags_around(row, col, 6);
        t.set_u32(0x7A4, 1);
    }

    /// `town_plan_reserve_adjacent_business_lot` (0x004FBBA0): busca junto a (`a`, `b`) una parcela
    /// libre con dos vecinas en `0x0B` y reserva un bloque de 4×5 en ella. Devuelve su celda o −1.
    pub fn reserve_adjacent_business_lot(&mut self, t: &Town, a: i32, b: i32) -> i32 {
        let mut g = self.grid(t);
        let s = g.stride as i64;
        let (a, b) = (a as i64, b as i64);
        let a2 = (b & 1) + a + 2;
        let rows = [a2, a2 + 2, a + 2, a2 - 3, a2 - 5, a2 - 7, a - 2, a2 - 2];
        let cols = [b - 5, b - 1, b + 4, b + 9, b + 5, b + 1, b - 4, b - 9];
        let idx: Vec<i64> = (0..8).map(|i| rows[i] * s + cols[i]).collect();
        let mut chosen = None;
        let (mut e, mut i) = (-2i64, 0usize);
        while e < 6 {
            if g.get(idx[i]) & 0x30 == 0x30 {
                let t3f = |g: &Grid, k: i64| g.get(idx[(k & 7) as usize]) & 0x3F == 0xB;
                if (t3f(&g, e + 1) && t3f(&g, e)) || (t3f(&g, e + 3) && t3f(&g, e - 4)) {
                    chosen = Some(i);
                    break;
                }
            }
            e += 2;
            i += 2;
        }
        let Some(i) = chosen else { return -1 };
        let (mut r0, mut c0) = (rows[i], cols[i]);
        for _ in 0..4 {
            let (mut r, mut c) = (r0, c0);
            c0 -= 1;
            r0 -= c0 & 1;
            for _ in 0..5 {
                let at = g.cell(r, c);
                let v = g.get(at);
                g.set(at, (v & 0x8B) | 0x0B);
                r += c & 1;
                c -= 1;
            }
        }
        g.clear_fit_flags_around(rows[i] as i32, cols[i] as i32, 4);
        (idx[i] & 0xFFFF) as i32
    }

    /// `town_plan_place_building` (0x004FB9C0): coloca el edificio de `ty` con esquina en (`row`,
    /// `col`). `extra` es el séptimo byte del registro y `special` el byte global `0x0067792F`.
    /// Solo coloca en las clases de huella 0–4 y 6 en adelante cuyo tramo de la tabla existe
    /// (`kind < 6`); las demás no se usan desde el juego.
    pub fn place_building(&mut self, t: &mut Town, row: i32, col: i32, ty: u32, extra: u8, special: u8) {
        let mut kind = FOOTPRINT_KIND[ty as usize] as i64;
        let mask: u32 = match ty {
            5 => 0x0E,
            3 => 0x1D,
            0x2D => {
                let g = self.grid(t);
                let at = g.cell(row as i64, col as i64);
                let v = g.get(at);
                if v & 0x3F != 0x13 {
                    return;
                }
                let mut g = g;
                g.set(at, 0x8B);
                return;
            }
            0..=0x14 => (1u32 << (kind & 31)) | 0x20,
            0x15..=0x1E => {
                if extra == special {
                    kind = 1;
                    0xA2
                } else {
                    0xA1
                }
            }
            _ => (1u32 << (kind & 31)) | 0xA0,
        };
        let mut g = self.grid(t);
        let at = g.cell(row as i64, col as i64);
        if g.get(at) as u32 & mask != mask {
            return;
        }
        let kind = kind as usize;
        let (w, h) = (FOOTPRINT_W[kind] as i64, FOOTPRINT_H[kind] as i64);
        let (mut r0, mut c0) = (row as i64, col as i64);
        for _ in 0..w.max(0) {
            let (mut r, mut c) = (r0, c0);
            c0 -= 1;
            r0 -= c0 & 1;
            for _ in 0..h.max(0) {
                let at = g.cell(r, c);
                let v = g.get(at);
                g.set(at, (v & 0x8B) | 0x0B);
                r += c & 1;
                c -= 1;
            }
        }
        g.clear_fit_flags_around(row, col, kind);
        if ty != 5 {
            drop(g);
            let lot = if kind == 4 {
                self.reserve_adjacent_business_lot(t, row, col)
            } else {
                -1
            };
            let mut g = self.grid(t);
            g.pave_roads_beside_building(row, col, kind);
            g.connect_building_to_road(row, col, kind);
            g.restore_fit_flags_around(row, col, kind);
            g.update_lot_pair_flags(row);
            if (lot as u32) < 0x10000 {
                let v = g.get(lot as i64);
                g.set(lot as i64, (v & 0xB0) | 0x30);
            }
        }
        t.set_u32(0x7A4, 1);
    }

    /// `town_plan_cell_matches_building_type` (0x004FCBD0): ¿sirve la celda (`row`, `col`) para un
    /// edificio de tipo `ty`? `extra` es el séptimo byte del registro y `special` el byte global.
    pub fn cell_matches_building_type(&self, t: &Town, row: u32, col: u32, ty: u32, extra: u8, special: u8) -> bool {
        let stride = t.i32(off::PLAN_STRIDE) as i64;
        let cell = || self.cells[(stride * (row & 0xFF) as i64 + (col & 0xFF) as i64) as usize];
        let ty = ty & 0xFF;
        let exact = |v: u8| cell() & 0x3F == v;
        let masked = |mask: u8| cell() & mask == mask;
        match ty {
            1 => (row & 0xFF) as u8 == t.u8(0x80A) && (col & 0xFF) as u8 == t.u8(0x80B),
            3 => exact(0x1D),
            5 => exact(0x0E),
            4 | 6..=20 | 47 => masked(((1u32 << FOOTPRINT_KIND[ty as usize]) as u8) | 0x20),
            21..=30 => masked(0xA1 + (extra == special) as u8),
            31 => exact(0x19),
            32 => exact(0x1B),
            33 => exact(0x14),
            34 => exact(0x1A),
            35 => exact(0x16),
            36 => exact(0x17),
            37 => exact(0x15),
            38 | 41..=44 => masked(((1u32 << FOOTPRINT_KIND[ty as usize]) as u8) | 0xA0),
            39 => exact(0x18),
            40 => cell() == 0x84,
            45 => cell() & 0xBF == 0x93,
            46 => (row & 0xFF) as u8 == t.u8(0x808) && (col & 0xFF) as u8 == t.u8(0x809),
            48 => exact(0x1C),
            _ => false,
        }
    }
}

impl Site {
    /// `town_plan_find_nearest_cell_for_type` (0x004F9C80): busca, en anillos crecientes alrededor de
    /// (`row`, `col`), la celda más cercana que sirve para un edificio de tipo `ty`, y la deja en
    /// `row`/`col`. Una posición fuera de la rejilla se sustituye por la de partida (`Town+0x800`).
    /// Algunos tipos devuelven directamente un punto del plano (`+0x808`, `+0x80A`, `+0x800`).
    /// Los tipos 52 y 53 usan en el original una máscara sin inicializar y no se admiten; los tipos
    /// 0, 2 y 49–51 y 54 nunca encuentran sitio.
    #[allow(clippy::too_many_arguments)]
    pub fn find_nearest_cell_for_type(
        &self,
        t: &Town,
        row: &mut u8,
        col: &mut u8,
        ty: i32,
        extra: u8,
        flag: bool,
        special: u8,
    ) -> bool {
        if ty >= 0x39 {
            return false;
        }
        let (rows, stride) = (t.i32(off::PLAN_ROWS), t.i32(off::PLAN_STRIDE));
        if *row as i32 >= rows {
            *row = t.u8(0x800);
        }
        if *col as i32 >= stride {
            *col = t.u8(0x801);
        }
        let point = |a: usize| (t.u8(a), t.u8(a + 1));
        let kind_mask = || 1u32 << FOOTPRINT_KIND[ty as usize];
        let (mask, value): (u32, u32) = match ty {
            1 => {
                (*row, *col) = point(0x80A);
                return true;
            }
            45 => {
                (*row, *col) = point(0x800);
                return true;
            }
            46 => {
                (*row, *col) = point(0x808);
                return true;
            }
            0 | 2 | 49..=51 | 54 => return false,
            3 => (0x3F, 0x1D),
            4 | 6..=20 => {
                let m = kind_mask() | if flag { 0xA0 } else { 0x20 };
                (m, m)
            }
            5 => (0x3F, 0x0E),
            21..=30 => {
                let m = 0xA1 + (extra == special) as u32;
                (m, m)
            }
            31 => (0x3F, 0x19),
            32 => (0x3F, 0x1B),
            33 => (0x3F, 0x14),
            34 => (0x3F, 0x1A),
            35 => (0x3F, 0x16),
            36 => (0x3F, 0x17),
            37 => (0x3F, 0x15),
            38 | 41..=44 => {
                let m = kind_mask() | 0xA0;
                (m, m)
            }
            39 => (0x3F, 0x18),
            40 => (0xA0, 0xA0),
            47 => (0x3F, 0x0F),
            48 => (0x3F, 0x1C),
            55 => (0xFF, 7),
            56 => (0xFF, 8),
            _ => panic!("tipo {ty}: máscara sin inicializar en el original"),
        };
        let cell = |y: i32, x: i32| (self.cells[(stride * y + x) as usize] as u32) & mask == value;
        let (r0, c0) = (*row as i32, *col as i32);
        if cell(r0, c0) {
            return true;
        }
        // Anillos de lado creciente. `lead` es la columna de partida del anillo (baja uno por anillo).
        let (mut ring, mut lead) = (1i32, c0 - 1);
        let (mut right, mut lead_adj) = (stride - c0, c0 - 2);
        loop {
            let parity_start = ((c0 - 1) & ring) & 1;
            let mut y = r0 - parity_start - (ring >> 1);
            let mut span = ring; // celdas del lado vertical
            let bottom_inside = y + ring < rows;
            if !bottom_inside {
                span = rows - y - 1;
            }
            let top_inside = y >= 0;
            if !top_inside {
                span += y;
                y = 0;
            }
            let mut x = lead;
            let mut width = ring; // celdas de los lados horizontales
            let right_inside = lead + 2 * ring < stride;
            if !right_inside {
                width = right >> 1;
            }
            let left_inside = lead >= 0;
            if !left_inside {
                width += (lead_adj - (lead_adj >> 31)) >> 1;
                x = lead & 1;
            }
            // Lado izquierdo, bajando.
            if left_inside {
                for _ in 0..span.max(0) {
                    if cell(y, x) {
                        (*row, *col) = (y as u8, x as u8);
                        return true;
                    }
                    y += 1;
                }
            } else {
                y += span;
            }
            // Lado inferior, hacia la derecha.
            if bottom_inside {
                for _ in 0..width.max(0) {
                    if cell(y, x) {
                        (*row, *col) = (y as u8, x as u8);
                        return true;
                    }
                    x += 2;
                }
            } else {
                x += width * 2;
            }
            // Lado derecho, subiendo.
            if right_inside {
                let mut n = span;
                while n > 0 {
                    if cell(y, x) {
                        (*row, *col) = (y as u8, x as u8);
                        return true;
                    }
                    n -= 1;
                    y -= 1;
                }
            } else {
                y -= span;
            }
            // Lado superior, hacia la izquierda.
            if top_inside {
                for _ in 0..width.max(0) {
                    if cell(y, x) {
                        (*row, *col) = (y as u8, x as u8);
                        return true;
                    }
                    x -= 2;
                }
            }
            ring += 1;
            lead -= 1;
            right += 1;
            lead_adj -= 1;
            if !(top_inside || bottom_inside || left_inside || right_inside) {
                return false;
            }
        }
    }
}
