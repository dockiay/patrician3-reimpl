//! Cuadrículas de la vista de ciudad (`Class22`): capa 0 (suelo), capa 1 (objetos) y capa de ocupación (`+0xC308/+0xC30C/+0xC310`).
//! Cada celda guarda la ficha (índice de la hoja o entidad), el fotograma propio `b2` (0xFF = relleno de la huella) y `b3`
//! (marca de solape de objetos altos). Porta, con paridad frente al x86 original (`oracle/x86_scene_grid.py`):
//! `0x0058B740` paso de rejilla, `0x0058E180` colocar objeto, `0x0058DF80` vaciar huella, `0x0058DD70` retirar objeto,
//! `0x0058E650` quitar marcas de solape, `0x00590960` volver al origen de un objeto y `0x00599D40` marcar la ocupación.

use p3_assets::town::Cell;

/// Lo que el original consulta de cada entidad por llamadas virtuales: huella (`+0x48` anchura, `+0x4C` altura en casillas)
/// y tabla de fotogramas (`+0x54`), de la que solo se usa la altura de cada fotograma.
pub trait Entities {
    fn size(&self, id: u16) -> (u32, u32);
    /// Altura en píxeles del fotograma `frame` de la entidad `id` (`u16` en `+6` de su entrada de 8 bytes); 0 si no existe.
    fn frame_height(&self, id: u16, frame: u8) -> u16;
}

pub const STEP_UP2: u8 = 0x00;
pub const STEP_RIGHT: u8 = 0x40;
pub const STEP_DOWN2: u8 = 0x80;
pub const STEP_LEFT: u8 = 0xC0;
pub const STEP_UP_RIGHT: u8 = 0x20;
pub const STEP_DOWN_RIGHT: u8 = 0x60;
pub const STEP_DOWN_LEFT: u8 = 0xA0;
pub const STEP_UP_LEFT: u8 = 0xE0;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SceneGrid {
    pub width: u32,
    pub height: u32,
    /// Altura de casilla en píxeles (`+0x514`).
    pub tile_height: u32,
    pub layers: [Vec<Cell>; 3],
}

impl SceneGrid {
    pub fn new(width: u32, height: u32, tile_height: u32) -> Self {
        let n = (width as usize) * (height as usize);
        let empty = Cell { tile: 0, b2: 0, b3: 0 };
        SceneGrid { width, height, tile_height, layers: [vec![empty; n], vec![empty; n], vec![empty; n]] }
    }

    fn inside(&self, x: u32, y: u32) -> bool {
        x < self.width && y < self.height
    }

    fn index(&self, x: u32, y: u32) -> usize {
        (self.width.wrapping_mul(y).wrapping_add(x)) as usize
    }

    /// `class22_step_cell` (`0x0058B740`): desplaza `(x, y)` una casilla en la dirección (`0x00`..`0xE0`) y dice si sigue dentro.
    pub fn step(&self, x: &mut u32, y: &mut u32, dir: u8) -> bool {
        match dir {
            STEP_UP2 => *y = y.wrapping_sub(2),
            STEP_RIGHT => *x = x.wrapping_add(1),
            STEP_DOWN2 => *y = y.wrapping_add(2),
            STEP_LEFT => *x = x.wrapping_sub(1),
            STEP_UP_RIGHT => {
                *x = x.wrapping_add(*y & 1);
                *y = y.wrapping_sub(1);
            }
            STEP_DOWN_RIGHT => {
                let ny = y.wrapping_add(1);
                *x = x.wrapping_add(!ny & 1);
                *y = ny;
            }
            STEP_DOWN_LEFT => {
                let ny = y.wrapping_add(1);
                *x = x.wrapping_sub(ny & 1);
                *y = ny;
            }
            STEP_UP_LEFT => {
                let ny = y.wrapping_sub(1);
                *x = x.wrapping_sub(ny & 1);
                *y = ny;
            }
            _ => {}
        }
        self.inside(*x, *y)
    }

    /// Marca `b3` de las celdas que un fotograma más alto que la casilla tapa por encima (`0x0058E180`, tramo repetido).
    fn mark_overlap(&mut self, layer: usize, at: usize, frame_height: u16) {
        let th = self.tile_height;
        if u32::from(frame_height) <= th {
            return;
        }
        let n = (u32::from(frame_height) - th) / th;
        let stride = 2 * self.width as usize;
        let mut k = 0u32;
        let mut p = at as isize;
        loop {
            p -= stride as isize;
            if p < 0 {
                break;
            }
            k += 1;
            let cell = &mut self.layers[layer][p as usize];
            if u32::from(cell.b3) < k {
                cell.b3 = k as u8;
            }
            if k > n {
                break;
            }
        }
    }

    /// `0x0058E180`: coloca la entidad `id` con su esquina en `(x, y)`: ficha en toda la huella, fotograma propio en la fila 0 y la
    /// columna 0 (`w - col - 1`, `fila + w - 1`), 0xFF en el resto, y marcas de solape. `layer` 0 o 1.
    pub fn place(&mut self, ents: &dyn Entities, x: u32, y: u32, id: u16, layer: usize) {
        let (w, h) = ents.size(id);
        let (mut sx, mut sy) = (x, y);
        let mut valid = self.inside(x, y);
        for row in 0..h {
            let (rx, ry) = (sx, sy);
            let (mut cx, mut cy) = (sx, sy);
            for col in 0..w {
                if valid {
                    let at = self.index(cx, cy);
                    let b2 = if row == 0 {
                        Some((w as u8).wrapping_sub(col as u8).wrapping_sub(1))
                    } else if col == 0 {
                        Some((row as u8).wrapping_add(w as u8).wrapping_sub(1))
                    } else {
                        None
                    };
                    let cell = &mut self.layers[layer][at];
                    cell.tile = id;
                    match b2 {
                        Some(b2) => {
                            cell.b2 = b2;
                            let fh = ents.frame_height(id, b2);
                            self.mark_overlap(layer, at, fh);
                        }
                        None => cell.b2 = 0xFF,
                    }
                }
                valid = self.step(&mut cx, &mut cy, STEP_UP_LEFT);
            }
            sx = rx;
            sy = ry;
            valid = self.step(&mut sx, &mut sy, STEP_UP_RIGHT);
        }
    }

    /// `0x00599D40`: escribe `value` en `b3` de la capa de ocupación sobre la huella de la entidad `id`.
    pub fn occupy(&mut self, ents: &dyn Entities, x: u32, y: u32, id: u16, value: u8) {
        let (w, h) = ents.size(id);
        let (mut sx, mut sy) = (x, y);
        let mut valid = self.inside(x, y);
        for _ in 0..h {
            let (mut cx, mut cy) = (sx, sy);
            for _ in 0..w {
                if valid {
                    let at = self.index(cx, cy);
                    self.layers[2][at].b3 = value;
                }
                valid = self.step(&mut cx, &mut cy, STEP_UP_LEFT);
            }
            valid = self.step(&mut sx, &mut sy, STEP_UP_RIGHT);
        }
    }
}

impl SceneGrid {
    fn cell_or_empty(&self, layer: usize, x: u32, y: u32) -> Cell {
        self.layers[layer].get(self.index(x, y)).copied().unwrap_or(Cell { tile: 0, b2: 0, b3: 0 })
    }

    /// `0x0058E650`: antes de borrar una celda cuyo fotograma tapa las de arriba, recorre esas celdas (hasta `n + 1` pares de
    /// filas) quitando la marca que le correspondía y rehaciendo la de los objetos altos vecinos.
    fn unmark_overlap(&mut self, ents: &dyn Entities, layer: usize, at: usize, frame_height: u16) {
        let th = self.tile_height;
        if th >= u32::from(frame_height) {
            return;
        }
        let n = (u32::from(frame_height) - th) / th;
        let stride = 2 * self.width as isize;
        let mut p = at as isize - (n as isize + 1) * stride;
        if p < 0 {
            return;
        }
        let end = (self.width as usize) * (self.height as usize);
        let mut k = 0u32;
        loop {
            if p as usize >= end {
                return;
            }
            let cell = self.layers[layer][p as usize];
            if n - k + 1 == u32::from(cell.b3) {
                self.layers[layer][p as usize].b3 = 0;
            }
            if cell.tile != 0 && cell.b2 != 0xFF {
                let fh = ents.frame_height(cell.tile, cell.b2);
                self.mark_overlap(layer, p as usize, fh);
            }
            k += 1;
            p += stride;
            if k > n {
                break;
            }
        }
    }

    /// `0x0058DD70`: retira de la capa la huella de la entidad `tile` con esquina en `(x, y)`: deshace las marcas de solape de lo que
    /// hubiera y deja en cada celda la ficha `replacement` con fotograma 0.
    pub fn remove(&mut self, ents: &dyn Entities, x: u32, y: u32, tile: u16, layer: usize, replacement: u16) {
        if tile == 0 {
            return;
        }
        let (w, h) = ents.size(tile);
        let (mut sx, mut sy) = (x, y);
        let mut valid = self.inside(x, y);
        for _ in 0..h {
            let (rx, ry) = (sx, sy);
            let (mut cx, mut cy) = (sx, sy);
            for _ in 0..w {
                if valid {
                    let at = self.index(cx, cy);
                    let cell = self.layers[layer][at];
                    if cell.tile != 0 && cell.b2 != 0xFF {
                        let fh = ents.frame_height(cell.tile, cell.b2);
                        self.unmark_overlap(ents, layer, at, fh);
                    }
                    let cell = &mut self.layers[layer][at];
                    cell.b2 = 0;
                    cell.tile = replacement;
                }
                valid = self.step(&mut cx, &mut cy, STEP_UP_LEFT);
            }
            sx = rx;
            sy = ry;
            valid = self.step(&mut sx, &mut sy, STEP_UP_RIGHT);
        }
    }

    /// `0x00590960`: desde una celda cualquiera de una huella, vuelve a su esquina de origen.
    pub fn origin(&self, ents: &dyn Entities, x: &mut u32, y: &mut u32, layer: usize) {
        loop {
            if self.cell_or_empty(layer, *x, *y).b2 != 0xFF {
                break;
            }
            if !self.step(x, y, STEP_DOWN2) {
                break;
            }
        }
        let cell = self.cell_or_empty(layer, *x, *y);
        let w = ents.size(cell.tile).0;
        let b2 = u32::from(cell.b2);
        if b2 < w {
            if b2 + 1 < w {
                for _ in 0..(w - b2 - 1) {
                    self.step(x, y, STEP_DOWN_RIGHT);
                }
            }
        } else {
            for _ in 0..(b2 - w + 1) {
                self.step(x, y, STEP_DOWN_LEFT);
            }
        }
    }

    /// `0x0058DF80`: vacía un rectángulo de `w × h` casillas desde `(x, y)`. Un objeto de varias casillas se retira entero desde su
    /// origen con `replacement_origin`; después la ficha de la celda se retira con `replacement` y queda `replacement`.
    #[allow(clippy::too_many_arguments)]
    pub fn clear(&mut self, ents: &dyn Entities, x: u32, y: u32, w: u32, h: u32, layer: usize, replacement: u16, replacement_origin: u16) {
        let (rows, cols) = (h & 0xFFFF, w & 0xFFFF);
        let (mut sx, mut sy) = (x, y);
        let mut valid = self.inside(x, y);
        for _ in 0..rows {
            let (rx, ry) = (sx, sy);
            let (mut cx, mut cy) = (sx, sy);
            for _ in 0..cols {
                if valid {
                    let at = self.index(cx, cy);
                    let tile = self.layers[layer][at].tile;
                    if tile != 0 {
                        let (ew, eh) = ents.size(tile);
                        if ew > 1 || eh > 1 {
                            let (mut px, mut py) = (cx, cy);
                            self.origin(ents, &mut px, &mut py, layer);
                            self.remove(ents, px, py, tile, layer, replacement_origin);
                        }
                        let tile = self.layers[layer][at].tile;
                        self.remove(ents, cx, cy, tile, layer, replacement);
                    }
                    let cell = &mut self.layers[layer][at];
                    cell.b2 = 0;
                    cell.tile = replacement;
                }
                valid = self.step(&mut cx, &mut cy, STEP_UP_LEFT);
            }
            sx = rx;
            sy = ry;
            valid = self.step(&mut sx, &mut sy, STEP_UP_RIGHT);
        }
    }
}
