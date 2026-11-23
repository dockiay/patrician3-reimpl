//! Rellenos del plano de la ciudad (C05): `town_plan_flood_fill_inside_walls` (0x004FD680),
//! `town_plan_flood_mark_outside_region` (0x004FDA10) y `town_plan_mark_defense_piece_fit_flags` (0x004FD830).
//!
//! Los índices de celda son de 16 bits, como en el original: las sumas dan la vuelta en `u16` y un índice fuera
//! de la rejilla es un error del llamante (aquí, un pánico). El centro del plano está en `Town+0x800/0x801`.

use super::Site;
use crate::town::{off, Town};

/// Tabla de salto `0x00673440`: por código de dirección (0..3), ocho ternas de bytes con signo
/// (columnas, filas, clase de huella) con las que `mark_defense_piece_fit_flags` busca encajes.
const FIT_STEPS: [i8; 96] = [
    -3, 2, 0, -3, -5, 0, -3, 1, 1, -3, -5, 1, -3, 2, 3, -3, -5, 3, -3, 4, 4, -3, -5, 4, 1, -3, 0, -5, -3, 0, 2, -3, 1,
    -5, -3, 1, 3, -3, 3, -5, -3, 3, 3, -3, 4, -5, -3, 4, 3, -5, 0, 3, 2, 0, 4, -5, 1, 4, 1, 1, 5, -5, 3, 5, 2, 3, 5,
    -5, 4, 5, 4, 4, -5, 4, 0, 1, 4, 0, -5, 3, 1, 2, 3, 1, -5, 4, 3, 3, 4, 3, -5, 6, 4, 3, 6, 4,
];

impl Site {
    /// Centro del plano: fila y columna del punto desde el que se rellena.
    fn plan_centre(t: &Town) -> (i32, i32) {
        (t.u8(0x800) as i32, t.u8(0x801) as i32)
    }

    /// `town_plan_flood_fill_inside_walls`: marca con `0x80` las celdas alcanzables desde el centro sin cruzar
    /// celdas de muralla, edificio o agua (tipos `5..=10`, `0xC`, `0xD`), tras borrar el bit en toda la rejilla.
    pub fn flood_fill_inside_walls(&mut self, t: &Town) {
        let rows = t.i32(off::PLAN_ROWS);
        let stride = t.i32(off::PLAN_STRIDE);
        let (r0, c0) = Self::plan_centre(t);
        if r0 >= rows || c0 >= stride {
            return;
        }
        let total = rows.wrapping_mul(stride);
        let capacity = ((stride + rows) * 4) as usize;
        let mut g = self.grid(t);
        for i in 0..total as i64 {
            let v = g.get(i);
            g.set(i, v & 0x7F);
        }
        let start = (r0 as u16).wrapping_mul(stride as u16).wrapping_add(c0 as u16);
        let mut queue = vec![0u16; capacity];
        queue[0] = start;
        let v = g.get(start as i64);
        g.set(start as i64, v | 0x80);
        let (mut head, mut tail) = (0usize, 1usize);
        loop {
            let mut p = queue[head];
            head += 1;
            if head >= capacity {
                head = 0;
            }
            for dir in 0..4 {
                p = match dir {
                    0 => p.wrapping_sub(1),
                    1 => p.wrapping_add(2),
                    2 if p & 1 != 0 => p.wrapping_sub(stride as u16),
                    2 => p.wrapping_add(stride as u16),
                    _ => p.wrapping_sub(2),
                };
                if p as i32 >= total {
                    continue;
                }
                let v = g.get(p as i64);
                if v & 0x80 != 0 || matches!(v & 0x3F, 0xC | 6 | 5 | 9 | 0xA | 7 | 8 | 0xD) {
                    continue;
                }
                queue[tail] = p;
                tail += 1;
                if tail >= capacity {
                    tail = 0;
                }
                g.set(p as i64, v | 0x80);
            }
            if tail == head {
                break;
            }
        }
    }

    /// Siembra de la búsqueda de `flood_mark_outside_region`: de la celda `i` (que vale `0x91`) toma la primera
    /// de sus cuatro vecinas lejanas que valga `0x11`, en el orden del original.
    fn outside_seed(g: &super::Grid, stride: i32, i: i32) -> Option<u16> {
        let s2 = stride.wrapping_mul(2);
        let candidates = [i + s2 + 2, i - s2 - 2, i - s2 + 2, i + s2 - 2];
        candidates
            .into_iter()
            .map(|c| c as u16)
            .find(|c| g.get(*c as i64) == 0x11)
    }

    /// `town_plan_flood_mark_outside_region`: desde una celda `0x91` (primero hacia atrás desde el centro y, si
    /// no hay, hacia delante) busca un vecino `0x11` y marca con `0x40` toda la región abierta que lo rodea.
    pub fn flood_mark_outside_region(&mut self, t: &Town) {
        let rows = t.i32(off::PLAN_ROWS);
        let stride = t.i32(off::PLAN_STRIDE);
        let (r0, c0) = Self::plan_centre(t);
        let centre = r0.wrapping_mul(stride).wrapping_add(c0);
        let total = rows.wrapping_mul(stride);
        let mut g = self.grid(t);
        let mut seed = None;
        let mut i = centre;
        while i > 0 {
            i -= 1;
            if g.get(i as i64) == 0x91 {
                seed = Self::outside_seed(&g, stride, i);
                break;
            }
        }
        if seed.is_none() && centre as u32 >= total as u32 {
            return;
        }
        if seed.is_none() {
            let mut i = centre;
            loop {
                i += 1;
                if g.get(i as i64) == 0x91 {
                    seed = Self::outside_seed(&g, stride, i);
                    break;
                }
                if i >= total {
                    break;
                }
            }
        }
        let Some(seed) = seed else { return };
        let mut queue = vec![0u16; total as usize];
        queue[0] = seed;
        let (mut head, mut tail) = (0usize, 1usize);
        while head < tail {
            let c = queue[head];
            head += 1;
            let neighbours = if c & 1 == 1 {
                let below = (stride as u16).wrapping_add(c);
                [
                    c.wrapping_add(1),
                    c.wrapping_sub(1),
                    below.wrapping_sub(1),
                    below.wrapping_add(1),
                ]
            } else {
                let above = c.wrapping_sub(stride as u16);
                [
                    c.wrapping_add(1),
                    c.wrapping_sub(1),
                    above.wrapping_sub(1),
                    above.wrapping_add(1),
                ]
            };
            for n in neighbours {
                let v = g.get(n as i64);
                if v >= 0x40 {
                    continue;
                }
                if v >= 0x20 || v < 5 || v >= 0x11 || v == 0xB {
                    queue[tail] = n;
                    tail += 1;
                }
                g.set(n as i64, v | 0x40);
            }
        }
    }

    /// `town_plan_mark_defense_piece_fit_flags`: decide por qué lado de la muralla está el exterior para la pieza
    /// de (`row`, `col`) y `ty` (`0x37`: izquierda; otra: derecha), borra las marcas `0x40` y deja el bit de
    /// clase en las celdas vecinas donde cabe cada huella de la tabla.
    pub fn mark_defense_piece_fit_flags(&mut self, t: &Town, row: i32, col: i32, ty: u32) {
        let stride = t.i32(off::PLAN_STRIDE);
        let rows = t.i32(off::PLAN_ROWS);
        let at = stride.wrapping_mul(row).wrapping_add(col);
        let left = ty & 0xFF == 0x37;
        let first = if left { at + stride } else { at - stride };
        let g = self.grid(t);
        let code = if g.get(first as i64) & 0x80 == 0x80 {
            if left {
                0
            } else {
                1
            }
        } else if g.get(at as i64 - 4) & 0x80 == 0x80 {
            if left {
                2
            } else {
                3
            }
        } else {
            drop(g);
            self.flood_mark_outside_region(t);
            let mut g = self.grid(t);
            let code = if g.get(first as i64) & 0x40 == 0x40 {
                if left {
                    0
                } else {
                    1
                }
            } else if g.get(at as i64 - 4) & 0x40 == 0x40 {
                if left {
                    2
                } else {
                    3
                }
            } else if left {
                0
            } else {
                1
            };
            for i in 0..rows.wrapping_mul(stride) as i64 {
                let v = g.get(i);
                g.set(i, v & 0xBF);
            }
            code
        };
        for step in 0..8usize {
            // cada paso parte de la esquina de la pieza
            let (mut r, mut c) = (row, col);
            let o = 24 * code + 3 * step;
            let (n1, n2, kind) = (FIT_STEPS[o] as i32, FIT_STEPS[o + 1] as i32, FIT_STEPS[o + 2] as i32);
            if n1 > 0 {
                for _ in 0..n1 {
                    let odd = c & 1;
                    c += 1;
                    if odd != 0 {
                        r += 1;
                    }
                }
            } else {
                for _ in 0..-n1 {
                    c -= 1;
                    if c & 1 != 0 {
                        r -= 1;
                    }
                }
            }
            if n2 > 0 {
                for _ in 0..n2 {
                    c += 1;
                    if c & 1 != 0 {
                        r -= 1;
                    }
                }
            } else {
                for _ in 0..-n2 {
                    let odd = c & 1;
                    c -= 1;
                    if odd != 0 {
                        r += 1;
                    }
                }
            }
            let mut g = self.grid(t);
            if g.can_fit_footprint(r, c, kind as usize) {
                let cell = stride.wrapping_mul(r).wrapping_add(c) as i64;
                let bit = if (kind as u8) & 31 >= 8 {
                    0
                } else {
                    1u8 << ((kind as u8) & 31)
                };
                let v = g.get(cell);
                g.set(cell, v | bit);
            }
        }
    }
}
