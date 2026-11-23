//! Trazado de la muralla y alta de sus piezas (C05): `town_plan_trace_wall_pieces` (0x004FA1D0) recorre las
//! celdas `0x0A` del plano desde un punto y emite las piezas; `op_place_town_defense_pieces` (0x00522B20) las
//! convierte en registros de la lista `Town+0x766` y las coloca en la rejilla.

use super::query::is_plan_point_valid;
use super::tables::RECORD_BYTE_6;
use super::Site;
use crate::town::{off, Town};

/// Bytes de tipo de pieza `0x00677928..0x0067792F` (los valores iniciales del EXE).
pub const DEFAULT_KINDS: [u8; 8] = [0x00, 0x20, 0x40, 0x60, 0x80, 0xA0, 0xC0, 0xE0];

pub const WALL_SEGMENT: u8 = 0x36;
const TOWER_LEFT: u8 = 0x37;
const TOWER_RIGHT: u8 = 0x38;

/// Resultado del trazado: piezas `[fila, columna, tipo, byte 7]` y cuántas son torres (`+4` del array).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Trace {
    pub entries: Vec<[u8; 4]>,
    pub towers: u32,
}

/// Dirección de avance: el tramo a la celda `A` (arriba-izquierda), `B` (arriba-derecha), `C` (abajo-izquierda)
/// o `D` (abajo-derecha) de la rejilla escalonada.
#[derive(Clone, Copy)]
enum Step {
    A = 0,
    C = 1,
    D = 2,
    B = 3,
    End = 4,
}

/// Tabla de bytes de la pieza según la dirección anterior (`prev`, 0..3 o 4 al inicio) y la nueva. La
/// original la monta en la pila; dos de sus palabras salen de los primeros `mov` antes de los `push`.
fn piece_byte(kinds: &[u8; 8], prev: usize, step: Step) -> u8 {
    let [g28, g29, g2a, g2b, g2c, g2d, g2e, g2f] = *kinds;
    let table: [[u8; 5]; 5] = [
        [g2f, g2c, g2f, g2a, g2b.wrapping_add(1)],
        [g2c, g29, g2e, g29, g2d.wrapping_add(1)],
        [g2f, g2e, g2f, g28, g2f.wrapping_add(1)],
        [g2a, g29, g28, g29, g29.wrapping_add(1)],
        [
            g2b.wrapping_add(1),
            g2d.wrapping_add(1),
            g2f.wrapping_add(1),
            g29.wrapping_add(1),
            g29,
        ],
    ];
    table[prev][step as usize]
}

impl Site {
    /// `town_plan_trace_wall_pieces`: traza desde el punto `k` del plano. Las celdas que no son `0x0A` y se
    /// pisan quedan marcadas con `0x40`. La capacidad del array es `(filas + columnas) * 2` piezas; pasarla
    /// es un error del llamante (aquí, un pánico), igual que leer o escribir fuera de la rejilla.
    pub fn trace_wall_pieces(&mut self, t: &Town, k: u32, kinds: &[u8; 8]) -> Trace {
        let rows = t.i32(off::PLAN_ROWS);
        let stride = t.i32(off::PLAN_STRIDE);
        let limit = rows.wrapping_mul(stride);
        let capacity = (rows + stride) as usize * 2;
        let mut out = Trace::default();
        let row = t.u8(0x7B0 + 2 * k as usize);
        if row == 0xFF {
            return out;
        }
        let mut b = t.u8(0x7B1 + 2 * k as usize) as i32 + row as i32 * stride;
        let mut prev: i32 = -1;
        let mut dir = 4usize;
        let mut g = self.grid(t);
        if g.get(b as i64) != 0xA {
            return out;
        }
        let mut entries: Vec<[u8; 4]> = Vec::new();
        let at = |entries: &mut Vec<[u8; 4]>, n: usize| -> usize {
            assert!(
                n < capacity,
                "el trazado de muralla desborda el array de {capacity} piezas"
            );
            if entries.len() <= n {
                entries.resize(n + 1, [0; 4]);
            }
            n
        };
        let mark = |g: &mut super::Grid, i: i32| {
            let v = g.get(i as i64);
            if v != 0xA {
                g.set(i as i64, v | 0x40);
            }
        };
        let mut n = 0usize;
        let mut towers = 0u32;
        // Escribe la pieza `n` con su tipo y su posición (las dos divisiones son `idiv` con signo).
        macro_rules! put {
            ($n:expr, $ty:expr, $cell:expr) => {{
                let i = at(&mut entries, $n);
                entries[i][2] = $ty;
                entries[i][0] = ($cell / stride) as u8;
                entries[i][1] = ($cell % stride) as u8;
            }};
        }
        loop {
            if g.get(b as i64) != 0xA {
                let v = g.get(prev as i64);
                g.set(prev as i64, v | 0x40);
                let v = g.get(b as i64);
                g.set(b as i64, v | 0x40);
            }
            put!(n, WALL_SEGMENT, b);
            // A: arriba-izquierda.
            let a = b - stride - 2;
            if a >= 0 && a != prev {
                let c = g.get(a as i64);
                if c == 0xA {
                    entries[n][3] = piece_byte(kinds, dir, Step::A);
                    prev = b;
                    b = a;
                    n += 1;
                    dir = 2;
                    continue;
                }
                if c == 8 {
                    entries[n][3] = piece_byte(kinds, dir, Step::A);
                    dir = 2;
                    n += 1;
                    towers += 1;
                    put!(n, TOWER_RIGHT, a);
                    entries[n][3] = kinds[7];
                    n += 1;
                    let d = (stride + 2) * 3;
                    prev = b - d;
                    b = a - d;
                    mark(&mut g, b);
                    continue;
                }
            }
            // B: arriba-derecha.
            let bb = a + 4;
            if bb >= 0 && bb != prev {
                let c = g.get(bb as i64);
                if c == 0xA {
                    entries[n][3] = piece_byte(kinds, dir, Step::B);
                    prev = b;
                    b = bb;
                    n += 1;
                    dir = 1;
                    continue;
                }
                if c == 9 {
                    entries[n][3] = piece_byte(kinds, dir, Step::B);
                    n += 1;
                    dir = 1;
                    let nb = bb + (2 - stride) * 2;
                    prev = nb;
                    towers += 1;
                    put!(n, TOWER_LEFT, nb);
                    entries[n][3] = kinds[1];
                    n += 1;
                    b = nb - stride + 2;
                    mark(&mut g, b);
                    continue;
                }
            }
            // C: abajo-izquierda.
            let cc = b + stride - 2;
            if cc < limit && cc != prev {
                let c = g.get(cc as i64);
                if c == 0xA {
                    entries[n][3] = piece_byte(kinds, dir, Step::C);
                    prev = b;
                    b = cc;
                    n += 1;
                    dir = 3;
                    continue;
                }
                if c == 7 {
                    entries[n][3] = piece_byte(kinds, dir, Step::C);
                    dir = 3;
                    n += 1;
                    towers += 1;
                    put!(n, TOWER_LEFT, cc);
                    entries[n][3] = kinds[1];
                    n += 1;
                    let d = (stride - 2) * 3;
                    prev = b + d;
                    b = cc + d;
                    mark(&mut g, b);
                    continue;
                }
            }
            // D: abajo-derecha.
            let dd = cc + 4;
            if dd < limit && dd != prev {
                let c = g.get(dd as i64);
                if c == 0xA {
                    entries[n][3] = piece_byte(kinds, dir, Step::D);
                    prev = b;
                    b = dd;
                    n += 1;
                    dir = 0;
                    continue;
                }
                if c == 9 {
                    entries[n][3] = piece_byte(kinds, dir, Step::D);
                    n += 1;
                    dir = 0;
                    let nb = dd + stride * 2 + 4;
                    towers += 1;
                    prev = nb;
                    put!(n, TOWER_RIGHT, nb);
                    entries[n][3] = kinds[7];
                    n += 1;
                    b = nb + stride + 2;
                    mark(&mut g, b);
                    continue;
                }
            }
            // La última pieza cierra el trazado: el recuento es `n + 1`, que es `entries.len()`.
            entries[n][3] = piece_byte(kinds, dir, Step::End);
            break;
        }
        out.entries = entries;
        out.towers = towers;
        out
    }

    /// `op_place_town_defense_pieces`: si la lista `Town+0x766` está vacía y el punto `k` es válido, traza la
    /// muralla desde él, da de alta un registro por pieza (propietario `0xFF - k`) y la coloca con
    /// `place_building`; después invalida el punto del plano. `special` es el byte global `0x0067792F`.
    pub fn place_town_defense_pieces(&mut self, t: &mut Town, k: u32, kinds: &[u8; 8]) {
        if t.u16(off::WALL_HEAD) < 0xFFFF || !is_plan_point_valid(t, k) {
            return;
        }
        let trace = self.trace_wall_pieces(t, k, kinds);
        for e in &trace.entries {
            let i = self.alloc_building_record(t) as usize;
            self.set_rec(i, 0, e[0]);
            self.set_rec(i, 1, e[1]);
            self.set_rec(i, 2, 0xFFu8.wrapping_sub(k as u8));
            self.set_rec(i, 3, e[2]);
            let head = t.u16(off::WALL_HEAD);
            self.set_next(i, head);
            self.set_rec(i, 6, RECORD_BYTE_6[e[2] as usize]);
            self.set_rec(i, 7, e[3]);
            self.place_building(t, e[0] as i32, e[1] as i32, e[2] as u32, 0, kinds[7]);
            t.set_u16(off::WALL_HEAD, i as u16);
        }
        t.set_u8(0x7B0 + 2 * k as usize, 0xFF);
        t.set_u8(0x7B1 + 2 * k as usize, 0xFF);
    }
}
