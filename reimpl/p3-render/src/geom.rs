//! Geometría de la escena de ciudad para las figuras: rumbo de un vector (`0x0055C170`), velocidad por rumbo (`0x0055C200`), paso a la casilla
//! vecina (`0x0058B740`), casilla a píxel (`0x0058CFB0`) y píxel a casilla (`0x0058D030`).
//!
//! Las casillas forman una rejilla en diamante con filas alternas desplazadas: la columna crece hacia la derecha y la fila `y` ocupa la banda
//! `y >> 1` (cada banda mide `cell_h + 1` píxeles). Un rumbo es un byte con ocho valores de 0x20 en 0x20: `0x00` arriba, `0x40` derecha, `0x80` abajo
//! y `0xC0` izquierda; los intermedios son las diagonales. `detect.aim` (`DetectBitmap`, 36×23) dice, para cada píxel de una casilla, si está dentro (`≥ 0xE1`)
//! o a qué vecina pertenece (el rumbo hacia ella).

/// Máscara `images/module_stadtkarte/detect.aim`: por fila, `(valor, repeticiones)` de izquierda a derecha; ancho 36, alto 23, un borde de un píxel.
/// Es un molde geométrico del rombo de 34×21; `tests/geom.rs` lo contrasta con el archivo real cuando hay `P3_CPR`.
pub const DETECT_RUNS: [[(u8, u8); 3]; 23] = [
    [(0xE0, 18), (0x20, 18), (0, 0)],
    [(0xE0, 16), (0xFF, 4), (0x20, 16)],
    [(0xE0, 15), (0xFF, 6), (0x20, 15)],
    [(0xE0, 13), (0xFF, 10), (0x20, 13)],
    [(0xE0, 12), (0xFF, 12), (0x20, 12)],
    [(0xE0, 10), (0xFF, 16), (0x20, 10)],
    [(0xE0, 9), (0xFF, 18), (0x20, 9)],
    [(0xE0, 7), (0xFF, 22), (0x20, 7)],
    [(0xE0, 6), (0xFF, 24), (0x20, 6)],
    [(0xE0, 4), (0xFF, 28), (0x20, 4)],
    [(0xE0, 3), (0xFF, 30), (0x20, 3)],
    [(0xC0, 1), (0xFF, 34), (0x40, 1)],
    [(0xA0, 3), (0xFF, 30), (0x60, 3)],
    [(0xA0, 4), (0xFF, 28), (0x60, 4)],
    [(0xA0, 6), (0xFF, 24), (0x60, 6)],
    [(0xA0, 7), (0xFF, 22), (0x60, 7)],
    [(0xA0, 9), (0xFF, 18), (0x60, 9)],
    [(0xA0, 10), (0xFF, 16), (0x60, 10)],
    [(0xA0, 12), (0xFF, 12), (0x60, 12)],
    [(0xA0, 13), (0xFF, 10), (0x60, 13)],
    [(0xA0, 15), (0xFF, 6), (0x60, 15)],
    [(0xA0, 16), (0xFF, 4), (0x60, 16)],
    [(0xA0, 18), (0x60, 18), (0, 0)],
];
pub const MASK_W: usize = 36;
pub const MASK_H: usize = 23;

/// La máscara desplegada, fila a fila.
pub fn detect_mask() -> Vec<u8> {
    let mut out = Vec::with_capacity(MASK_W * MASK_H);
    for row in DETECT_RUNS {
        for (v, n) in row {
            out.extend(std::iter::repeat(v).take(n as usize));
        }
    }
    out
}

/// `0x0055C170`: rumbo del vector `(dx, dy)` (la arista está donde `4·|dx| = 15·|dy|`, y `11·|dx| = 7·|dy|` separa vertical de diagonal).
pub fn heading_from_delta(dx: i32, dy: i32) -> u8 {
    let (ax, ay) = (dx.unsigned_abs(), dy.unsigned_abs());
    let (a4, b15) = (ax.wrapping_mul(4), ay.wrapping_mul(15));
    if a4 <= b15 {
        if ax.wrapping_mul(11) < ay.wrapping_mul(7) {
            if dy >= 0 { 0x80 } else { 0x00 }
        } else if dx < 0 {
            if dy >= 0 { 0xA0 } else { 0xE0 }
        } else if dy >= 0 {
            0x60
        } else {
            0x20
        }
    } else if dx >= 0 {
        0x40
    } else {
        0xC0
    }
}

/// `0x0055C200`: velocidad `(vx, vy)` (en la unidad de posición de 16.16) de una figura con rapidez `speed` que va con el rumbo `dir`.
/// `None` si el rumbo no es múltiplo de 0x20 (el original no toca la velocidad).
pub fn velocity(dir: u8, speed: i32) -> Option<(i32, i32)> {
    let m = |k: i32| speed.wrapping_mul(k);
    let udiv = |v: i32, d: u32| ((v as u32) / d) as i32;
    let sdiv = |v: i32, d: i32| v.wrapping_div(d);
    Some(match dir {
        0x00 => (0, sdiv(m(-0xB000), 0x2981)),
        0x20 => (udiv(m(0xB50), 0x271), sdiv(m(-0x7C73), 0x2981)),
        0x40 => (udiv(m(0x1000), 0x271), 0),
        0x60 => (udiv(m(0xB50), 0x271), udiv(m(0x7C73), 0x2981)),
        0x80 => (0, udiv(m(0xB000), 0x2981)),
        0xA0 => (sdiv(m(-0xB50), 0x271), udiv(m(0x7C73), 0x2981)),
        0xC0 => (sdiv(m(-0x1000), 0x271), 0),
        0xE0 => (sdiv(m(-0xB50), 0x271), sdiv(m(-0x7C73), 0x2981)),
        _ => return None,
    })
}

/// Parámetros de la escena que usan las funciones: `Class22 +0x510/+0x514` (ancho de casilla y alto menos uno), `+0x4D0/+0x4D4` (desplazamiento de las filas pares) y las
/// dimensiones del mapa en casillas (`+0xC314/+0xC318`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Geom {
    pub cols: u32,
    pub rows: u32,
    pub cell_w: i32,
    pub cell_h: i32,
    pub off_x: i32,
    pub off_y: i32,
}

impl Geom {
    /// Ancho del mapa en píxeles (`+0xC31C`).
    pub fn width_px(&self) -> i32 {
        self.cell_w.wrapping_mul(self.cols as i32)
    }
    /// Alto del mapa en píxeles (`+0xC320`).
    pub fn height_px(&self) -> i32 {
        ((self.rows >> 1) as i32).wrapping_mul(self.cell_h.wrapping_add(1))
    }

    /// `0x0058B740`: mueve la casilla `(col, row)` un paso hacia `dir`; devuelve si sigue dentro del mapa. Los valores se envuelven como `u32`.
    pub fn step(&self, col: &mut u32, row: &mut u32, dir: u8) -> bool {
        match dir {
            0x00 => *row = row.wrapping_sub(2),
            0x40 => *col = col.wrapping_add(1),
            0x80 => *row = row.wrapping_add(2),
            0xC0 => *col = col.wrapping_sub(1),
            0x20 => {
                *col = col.wrapping_add(*row & 1);
                *row = row.wrapping_sub(1);
            }
            0x60 => {
                let r = row.wrapping_add(1);
                *row = r;
                *col = col.wrapping_add(!r & 1);
            }
            0xA0 => {
                let r = row.wrapping_add(1);
                *row = r;
                *col = col.wrapping_sub(r & 1);
            }
            0xE0 => {
                let r = row.wrapping_sub(1);
                *row = r;
                *col = col.wrapping_sub(r & 1);
            }
            _ => {}
        }
        *col < self.cols && *row < self.rows
    }

    /// `0x0058CFB0`: píxel del pie de la casilla `(col, row)`.
    pub fn cell_to_pixel(&self, col: i32, row: u32) -> (i32, i32) {
        let band = (row >> 1) as i32;
        if row & 1 != 0 {
            (col.wrapping_add(1).wrapping_mul(self.cell_w), self.cell_h.wrapping_add(1).wrapping_mul(band).wrapping_add(self.cell_h))
        } else {
            (
                self.cell_w.wrapping_mul(col).wrapping_add(self.off_x),
                self.cell_h.wrapping_add(1).wrapping_mul(band).wrapping_sub(self.off_y).wrapping_add(self.cell_h),
            )
        }
    }

    /// `0x0058D030`: casilla que contiene el píxel `(x, y)` según `mask` (la de `detect_mask`, paso 36). El segundo valor es `false` si el punto
    /// se salió del mapa (se recorta) o cae en una casilla fuera de él.
    pub fn pixel_to_cell(&self, x: i32, y: i32, mask: &[u8]) -> ((u32, u32), bool) {
        let mut ok = true;
        let (w, h) = (self.width_px(), self.height_px());
        let mut x = x;
        let mut y = y;
        if x < 0 {
            x = 0;
            ok = false;
        } else if x as u32 >= w as u32 {
            x = w.wrapping_sub(1);
            ok = false;
        }
        if y < 0 {
            y = 0;
            ok = false;
        } else if y as u32 >= h as u32 {
            y = h.wrapping_sub(1);
            ok = false;
        }
        let band_h = self.cell_h.wrapping_add(1);
        let band = y / band_h;
        let mut row = (band as u32).wrapping_mul(2);
        let mut col = (x / self.cell_w) as u32;
        let mx = x.wrapping_sub(self.cell_w.wrapping_mul(col as i32)).wrapping_add(1);
        let my = y.wrapping_sub(band_h.wrapping_mul(band)).wrapping_add(1);
        let code = mask[(my as usize) * MASK_W + mx as usize];
        if code < 0xE1 {
            let inside = self.step(&mut col, &mut row, code);
            ok &= inside;
            if !inside {
                col = if (col as i32) < 0 { 0 } else if col >= self.cols { self.cols - 1 } else { col };
                row = if (row as i32) < 0 { 0 } else if row >= self.rows { self.rows - 1 } else { row };
            }
        }
        ((col, row), ok)
    }
}
