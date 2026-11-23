//! Máscaras de solares del plano (C05): `town_plan_build_valid_site_mask` (0x004FD0D0),
//! `town_plan_build_house_site_mask` (0x004FD4A0) y `town_plan_build_type0_fixed_site_mask` (0x004FD5E0).
//!
//! Cada una reserva `filas * columnas` bytes, los pone a cero, anota `Town+0x7A4 = 0` y marca con `1` las celdas
//! que ocupa una huella alrededor de cada celda del plano que cumple su condición. La máscara está traspuesta:
//! la celda (fila, columna) cae en `columna * filas + fila`. El original escribe sin comprobar límites; aquí un
//! índice fuera de la máscara es un pánico (los casos de prueba se limitan a los que el original no desborda).

use super::Site;
use crate::town::{off, Town};

/// `0x00672F64`: repeticiones externas de la huella por clase.
const OUTER: [i8; 16] = [2, 3, 3, 4, 4, 1, 2, 4, 2, 6, 6, 5, 3, 1, 1, 1];
/// `0x00672F74`: repeticiones internas.
const INNER: [i8; 16] = [3, 2, 4, 3, 5, 1, 2, 6, 6, 2, 4, 5, 3, 1, 1, 1];
/// `0x00672F84`: clase de huella por tipo de edificio (`>= 0x10`: no hay máscara).
const CLASS: [i8; 64] = [
    3, 7, 0, 0, 4, 0, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 10, 3, 2, 3, 0, 0,
    3, 0, 5, 3, 2, 2, 3, 6, 7, 6, 6, 6, 6, 6, 6, 8, 9, 8, 9, 0, 0, 0, 44, 27, 11, 8,
];

/// Tabla de despacho de `0x004FD460` (tipo `0..=0x38` → manejador).
#[derive(Clone, Copy, PartialEq, Eq)]
enum Selector {
    /// Libera la máscara y devuelve nulo.
    Release,
    /// Una sola huella en el punto del plano (tipos `1` y `0x2E`).
    Fixed,
    /// Celdas con `celda & máscara == valor`.
    Match { mask: u32, value: u32 },
    /// Como `Match` con `1 << clase | 0x20` (más `0x80` si la bandera es distinta de cero).
    Shift20,
    /// Como `Match` con `1 << clase | 0xA0` en máscara y valor.
    ShiftA0,
    /// Máscara y valor son el tipo y la bandera de la llamada.
    Raw,
}

fn selector(ty: u32, flag: u32) -> Selector {
    use Selector::*;
    let fixed = |value| Match { mask: 0x3F, value };
    match ty {
        1 | 0x2E => Fixed,
        3 => fixed(0x1D),
        4 | 6..=0x14 => Shift20,
        5 => fixed(0x0E),
        0x1F => fixed(0x19),
        0x20 => fixed(0x1B),
        0x21 => fixed(0x14),
        0x22 => fixed(0x1A),
        0x23 => fixed(0x16),
        0x24 => fixed(0x17),
        0x25 => fixed(0x15),
        0x26 | 0x29 | 0x2A | 0x2B | 0x2C => ShiftA0,
        0x27 => fixed(0x18),
        0x2F => fixed(0x0F),
        0x30 => fixed(0x1C),
        0x2D | 0x34 | 0x35 => Raw,
        0..=0x38 => Release,
        _ => {
            let _ = flag;
            Raw
        }
    }
}

/// Lo que pide la reserva de la máscara.
pub trait MaskHost {
    /// `operator new` (0x0064F7B9): `false` si no hay memoria.
    fn alloc_mask(&mut self, bytes: u32) -> bool;
}

/// Marca `outer * inner` celdas desde `pos` con la huella escalonada del original (la paridad de la columna
/// alterna en cada paso y avanza una fila cada dos).
fn stamp(mask: &mut [u8], rows: i32, mut pos: i32, mut parity: i32, outer: i32, inner: i32) {
    for _ in 0..outer.max(0) {
        let (mut p, mut q) = (pos, parity);
        for _ in 0..inner.max(0) {
            mask[usize::try_from(p).expect("la huella se sale de la máscara")] = 1;
            p = p.wrapping_add(q.wrapping_sub(rows));
            q = q.wrapping_sub(1) & 1;
        }
        pos = pos.wrapping_add(parity.wrapping_sub(rows).wrapping_sub(1));
        parity = parity.wrapping_sub(1) & 1;
    }
}

impl Site {
    fn plan_mask_buffer(t: &mut Town, host: &mut dyn MaskHost) -> Option<(Vec<u8>, i32, i32)> {
        let rows = t.i32(off::PLAN_ROWS);
        let stride = t.i32(off::PLAN_STRIDE);
        if !host.alloc_mask(rows.wrapping_mul(stride) as u32) {
            return None;
        }
        t.set_u32(0x7A4, 0);
        Some((vec![0u8; rows.wrapping_mul(stride) as usize], rows, stride))
    }

    /// Posición en la máscara de la celda `index` de la rejilla y su paridad de columna.
    fn mask_position(index: i32, rows: i32, stride: i32) -> (i32, i32) {
        let (row, col) = (index as u32 / stride as u32, index as u32 % stride as u32);
        (
            (rows as u32).wrapping_mul(col).wrapping_add(row) as i32,
            (col & 1) as i32,
        )
    }

    /// Recorre la rejilla de atrás adelante y estampa la huella en cada celda con `celda & mask == value`.
    fn stamp_matching(
        &self,
        mask: &mut [u8],
        rows: i32,
        stride: i32,
        cond: impl Fn(u8) -> bool,
        outer: i32,
        inner: i32,
    ) {
        let total = rows.wrapping_mul(stride);
        for index in (0..total).rev() {
            if cond(self.cells[index as usize]) {
                let (pos, parity) = Self::mask_position(index, rows, stride);
                stamp(mask, rows, pos, parity, outer, inner);
            }
        }
    }

    /// `town_plan_build_valid_site_mask(tipo, bandera)`.
    pub fn plan_build_valid_site_mask(
        &self,
        t: &mut Town,
        ty: u32,
        flag: u32,
        host: &mut dyn MaskHost,
    ) -> Option<Vec<u8>> {
        let class = CLASS[ty as usize & 63];
        if class >= 0x10 {
            return None;
        }
        let (outer, inner) = (OUTER[class as usize] as i32, INNER[class as usize] as i32);
        let (mut mask, rows, stride) = Self::plan_mask_buffer(t, host)?;
        let sel = selector(ty, flag);
        let (and, value) = match sel {
            Selector::Release => return None,
            Selector::Fixed => {
                let (row, col) = if ty == 0x2E {
                    (t.u8(0x7A4 + 0x64), t.u8(0x7A4 + 0x65))
                } else {
                    (t.u8(0x7A4 + 0x66), t.u8(0x7A4 + 0x67))
                };
                let pos = rows.wrapping_mul(col as i32).wrapping_add(row as i32);
                stamp(&mut mask, rows, pos, (col & 1) as i32, outer, inner);
                return Some(mask);
            }
            Selector::Match { mask, value } => (mask, value),
            Selector::Shift20 => {
                let v = (1u32 << (class as u32 & 31)) | 0x20 | if flag != 0 { 0x80 } else { 0 };
                (v, v)
            }
            Selector::ShiftA0 => {
                let v = (1u32 << (class as u32 & 31)) | 0xA0;
                (v, v)
            }
            Selector::Raw => (ty, flag),
        };
        self.stamp_matching(&mut mask, rows, stride, |c| c as u32 & and == value, outer, inner);
        Some(mask)
    }

    /// `town_plan_build_house_site_mask`: huellas de 2 x 3 en las celdas con los bits `0xA1` y de 3 x 2 en las de `0xA2`.
    pub fn plan_build_house_site_mask(&self, t: &mut Town, host: &mut dyn MaskHost) -> Option<Vec<u8>> {
        let (mut mask, rows, stride) = Self::plan_mask_buffer(t, host)?;
        let total = rows.wrapping_mul(stride);
        for index in (0..total).rev() {
            let c = self.cells[index as usize];
            let (pos, parity) = Self::mask_position(index, rows, stride);
            if c & 0xA1 == 0xA1 {
                stamp(&mut mask, rows, pos, parity, 2, 3);
            }
            if c & 0xA2 == 0xA2 {
                stamp(&mut mask, rows, pos, parity, 3, 2);
            }
        }
        Some(mask)
    }

    /// `town_plan_build_type0_fixed_site_mask`: una huella de 4 x 3 en el punto fijo `Town+0x802/0x803`.
    pub fn plan_build_type0_fixed_site_mask(&self, t: &mut Town, host: &mut dyn MaskHost) -> Option<Vec<u8>> {
        let (mut mask, rows, _) = Self::plan_mask_buffer(t, host)?;
        let (row, col) = (t.u8(0x802) as i32, t.u8(0x803) as i32);
        stamp(&mut mask, rows, rows.wrapping_mul(col).wrapping_add(row), col & 1, 4, 3);
        Some(mask)
    }
}
