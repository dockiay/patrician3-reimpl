//! Rejilla del plano urbano (`town_plan`): una celda por byte, en filas de `stride` bytes con las
//! posiciones alternas intercaladas (el original resta una fila cuando el índice es impar).
//!
//! Los bits bajos de la celda (`& 0x3F`) son el tipo de suelo; `0x20` marca la celda como ocupada o
//! reservada y `0x80` la calle. Las funciones de aquí solo mueven esos bits, igual que el original, y
//! no comprueban límites: un índice fuera de la rejilla es un error del llamante (aquí, un pánico).

/// Anchos (`0x00672F64`) y altos (`0x00672F74`) de las huellas, por clase de huella.
pub const FOOTPRINT_W: [i8; 16] = [2, 3, 3, 4, 4, 1, 2, 4, 2, 6, 6, 5, 3, 1, 1, 1];
pub const FOOTPRINT_H: [i8; 16] = [3, 2, 4, 3, 5, 1, 2, 6, 6, 2, 4, 5, 3, 1, 1, 1];
/// Clase de huella de cada tipo de edificio (`0x00672F84`), tipos 0–0x3F.
pub const FOOTPRINT_KIND: [i8; 64] = [
    3, 7, 0, 0, 4, 0, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 10, 3, 2, 3, 0, 0,
    3, 0, 5, 3, 2, 2, 3, 6, 7, 6, 6, 6, 6, 6, 6, 8, 9, 8, 9, 0, 0, 0, 44, 27, 11, 8,
];

fn ix(i: i64) -> usize {
    usize::try_from(i).expect("índice de celda negativo")
}

/// La rejilla con su `stride` y número de filas (`Town+0x7AC` y `Town+0x7A8`).
pub struct Grid<'a> {
    pub stride: i32,
    pub rows: i32,
    pub cells: &'a mut Vec<u8>,
}

impl Grid<'_> {
    /// Lee la celda `i`. Más allá de la rejilla el original lee la memoria contigua (aquí nada: 0), como
    /// el oráculo, cuyo bloque de celdas está a cero tras `rows * stride`.
    pub fn get(&self, i: i64) -> u8 {
        self.cells.get(ix(i)).copied().unwrap_or(0)
    }
    /// Escribe la celda `i`; fuera de la rejilla el original escribe en memoria contigua que no pertenece
    /// al plano (y que nada lee de vuelta salvo `get`, que ya devuelve 0), así que se descarta.
    pub fn set(&mut self, i: i64, v: u8) {
        if let Some(c) = self.cells.get_mut(ix(i)) {
            *c = v;
        }
    }
    fn s(&self) -> i64 {
        self.stride as i64
    }
    /// `cell = (cell & keep) | set`.
    fn mask(&mut self, i: i64, keep: u8, set: u8) {
        let v = self.get(i);
        self.set(i, (v & keep) | set);
    }
    /// Reserva la celda (`& 0xE0 | 0x20`).
    fn reserve(&mut self, i: i64) {
        self.mask(i, 0xE0, 0x20);
    }
    /// Reserva solo si el suelo es del tipo `0x11`.
    fn reserve_if_11(&mut self, i: i64) {
        if self.get(i) & 0x3F == 0x11 {
            self.reserve(i);
        }
    }

    /// `town_plan_close_wall_diagonal_gaps` (0x004FDCE0): une con muralla (`0x0D`) las diagonales que
    /// quedan abiertas alrededor de `i`.
    pub fn close_wall_diagonal_gaps(&mut self, i: i64) {
        let (s, d) = (self.s(), 0x0D);
        if i & 1 == 0 {
            if self.get(i + 1) == d && self.get(i - s - 2) == d {
                self.set(i, d);
                self.set(i - s - 1, d);
            }
            if self.get(i + s) == d && self.get(i - s - 3) == d {
                self.set(i - 1, d);
                self.set(i - 2, d);
            }
            if self.get(i - s + 1) == d && self.get(i + s - 2) == d {
                self.set(i, d);
                self.set(i - 1, d);
            }
            if self.get(i - s) == d && self.get(i - 3) == d {
                self.set(i - s - 1, d);
                self.set(i - 2, d);
            }
        } else {
            if self.get(i + s + 1) == d && self.get(i - s - 2) == d {
                self.set(i, d);
                self.set(i - 1, d);
            }
            if self.get(i + s) == d && self.get(i - 3) == d {
                self.set(i + s - 1, d);
                self.set(i - 2, d);
            }
            if self.get(i + 1) == d && self.get(i + s - 2) == d {
                self.set(i, d);
                self.set(i + s - 1, d);
            }
            if self.get(i - s) == d && self.get(i + s - 3) == d {
                self.set(i - 1, d);
                self.set(i - 2, d);
            }
        }
    }

    /// `town_plan_can_fit_footprint` (0x004FB8F0): ¿caben en la rejilla, con todas sus celdas ya
    /// reservadas (`0x20`), la huella `kind` con esquina en (`row`, `col`)?
    pub fn can_fit_footprint(&self, row: i32, col: i32, kind: usize) -> bool {
        let w = FOOTPRINT_W[kind] as i32;
        let h = FOOTPRINT_H[kind] as i32;
        let parity = col & 1;
        if w > parity + row * 2 + 1 {
            return false;
        }
        if self.rows * 2 < parity + row * 2 + h {
            return false;
        }
        if h + w > col + 2 {
            return false;
        }
        let (mut row, mut col, mut parity) = (row as i64, col as i64, parity);
        for _ in 0..w {
            let mut pos = self.s() * row + col;
            for _ in 0..h {
                if self.get(pos) & 0x20 == 0 {
                    return false;
                }
                if parity != 0 {
                    pos += self.s() - 1;
                    parity = 0;
                } else {
                    pos -= 1;
                    parity = 1;
                }
            }
            col -= 1;
            parity = (col & 1) as i32;
            row -= parity as i64;
        }
        true
    }

    /// Recorrido en diagonal hacia la izquierda: baja un índice y, si el nuevo es impar, una fila.
    fn step_odd_up(&self, pos: i64) -> i64 {
        let n = pos - 1;
        if n & 1 != 0 {
            n - self.s()
        } else {
            n
        }
    }

    /// Murallas 0x37/0x34 (`wall_a`) y 0x38/0x35 (`wall_b`): dos columnas de cinco celdas con los
    /// bits de borde `4`/`3` y las celdas de los extremos reservadas.
    fn clear_wall(&mut self, idx: i64, wall_a: bool) {
        let s = self.s();
        let p = idx & 1;
        let mut pos = idx;
        for _ in 0..2 {
            for (n, (keep, set)) in [(0xE0, 0x20), (0xC4, 0x04), (0xC3, 0x03), (0xC3, 0x03), (0xC4, 0x04)]
                .into_iter()
                .enumerate()
            {
                let _ = n;
                self.mask(pos, keep, set);
                if wall_a {
                    let odd = pos & 1 != 0;
                    pos -= 1;
                    if odd {
                        pos += s;
                    }
                } else {
                    pos = self.step_odd_up(pos);
                }
            }
            self.reserve(pos);
            pos = if wall_a {
                if p != 0 {
                    idx - 1
                } else {
                    idx - s - 1
                }
            } else if p != 0 {
                idx - 1 + s
            } else {
                idx - 1
            };
        }
        if wall_a {
            self.reserve_if_11(idx - s - 2);
            self.reserve_if_11(idx + 3 * s - 4);
            self.reserve_if_11(if p == 1 { idx + 1 + s } else { idx + 1 });
            self.reserve_if_11(idx + (p + 1) * s - 7);
        } else {
            self.reserve_if_11(idx + s - 2);
            self.reserve_if_11(idx - 3 * s - 4);
            self.reserve_if_11(if p == 1 { idx + 1 } else { idx - s + 1 });
            self.reserve_if_11(idx - (2 - p) * s - 7);
        }
    }

    /// `town_plan_clear_building_footprint` (0x004FACD0): al demoler el edificio de tipo `ty` con
    /// esquina (`row`, `col`) quita las reservas de su huella y las celdas vecinas. `extra` es el
    /// séptimo byte del registro y `special` el byte global `0x0067792F`.
    pub fn clear_building_footprint(&mut self, row: u32, col: u32, ty: u32, extra: u32, special: u8) {
        let s = self.s();
        let idx = s * row as i64 + col as i64;
        if ty >= 0x32 {
            match ty {
                0x37 | 0x34 => self.clear_wall(idx, true),
                0x38 | 0x35 => self.clear_wall(idx, false),
                _ => {
                    self.reserve(idx);
                    self.reserve(idx - 1);
                    self.reserve(idx - 2);
                    let p = idx & 1;
                    self.reserve(if p == 1 { idx + s - 1 } else { idx - s - 1 });
                    for d in [-s - 2, -s, s, s - 2] {
                        self.reserve_if_11(idx + d);
                    }
                    let x = if p == 1 { idx + 1 + s } else { idx + 1 };
                    for d in [0, -4, -4 - s, -s] {
                        self.reserve_if_11(x + d);
                    }
                    self.close_wall_diagonal_gaps(idx);
                }
            }
            return;
        }
        if ty == 5 {
            self.set(idx, 0x0E);
            self.set(idx - 1, 0x1E);
            self.set(idx - 2, 0x1E);
            if idx & 1 != 0 {
                for d in [s - 1, s - 2, s - 3] {
                    self.set(idx + d, 0x1E);
                }
            } else {
                self.set(idx - 3, 0x1E);
                self.set(idx + 2 * s - 2, 0x1E);
                self.set(idx - s - 1, 0x1E);
            }
            return;
        }
        if ty == 0x30 || ty == 0x31 {
            self.set(idx, 0x1C);
            self.set(idx - 1, 0x1E);
            self.set(idx - 2, 0x1E);
            self.set(if col & 1 != 0 { idx + s - 1 } else { idx - s - 1 }, 0x1E);
            return;
        }
        let mut kind = FOOTPRINT_KIND[ty as usize] as i64;
        if ty == 0x2F {
            self.set(idx, 0x0F);
            self.set(idx - 1, 0x1E);
            self.set(idx - 2, 0x1E);
            self.set(if col & 1 != 0 { idx + s - 1 } else { idx - s - 1 }, 0x1E);
        } else if (0x15..0x1E).contains(&ty) && extra as u8 == special {
            kind = 1;
        }
        let kind = kind as usize;
        let mut base = idx;
        for _ in 0..FOOTPRINT_W[kind] {
            let mut pos = base;
            for _ in 0..FOOTPRINT_H[kind] {
                self.mask(pos, 0x80, 0x20);
                let odd = pos & 1 != 0;
                pos -= 1;
                if odd {
                    pos += s;
                }
            }
            base -= 1;
            if base & 1 != 0 {
                base -= s;
            }
        }
        // Marca en cada celda vecina qué huellas (0-4) caben aún.
        let (kw, kh) = (FOOTPRINT_W[kind] as i32, FOOTPRINT_H[kind] as i32);
        let (row, col) = (row as i32, col as i32);
        for e in 0..5usize {
            let bit = 1u8 << e;
            let (we, he) = (FOOTPRINT_W[e] as i32, FOOTPRINT_H[e] as i32);
            let (columns, lines) = (kw + we - 1, kh + he - 1);
            let odd = col & 1 != 0;
            let mut row0 = (we - 1) / 2 + row - (he - 1) / 2;
            if odd && (we - 1) & 1 != 0 {
                row0 += 1;
            }
            if !odd && (he - 1) & 1 != 0 {
                row0 -= 1;
            }
            let mut col0 = col + we + he - 2;
            for _ in 0..columns.max(0) {
                let (mut r, mut c) = (row0, col0);
                for _ in 0..lines.max(0) {
                    if self.can_fit_footprint(r, c, e) {
                        let at = s * r as i64 + c as i64;
                        let v = self.get(at);
                        self.set(at, v | bit);
                    }
                    c -= 1;
                    if c & 1 == 0 {
                        r += 1;
                    }
                }
                if col0 & 1 == 0 {
                    row0 -= 1;
                }
                col0 -= 1;
            }
        }
    }
}
