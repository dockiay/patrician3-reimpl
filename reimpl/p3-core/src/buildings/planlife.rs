//! Ciclo de vida del plano de la ciudad (C05): `town_plan_init_default` (0x004F95C0), `town_plan_free_data`
//! (0x004F9650), `town_plan_reset` (0x004F9930), `town_plan_resize` (0x004F99B0) y `town_plan_load_from_file`
//! (0x004F9B40).
//!
//! El plano vive en `Town+0x7A4`; su rejilla es `Site::cells` (el puntero `plan+0x68` del original es nulo cuando
//! `cells` está vacío). `init_default` no libera la rejilla anterior (el original la pierde); `free_data` y
//! `reset` sí.

use super::planmask::MaskHost;
use super::Site;
use crate::town::{off, Town};

const PLAN: usize = 0x7A4;
/// Versión de plano con la que se carga un archivo de plano.
const FILE_VERSION: u32 = 0x85;
/// Tamaño mínimo de un archivo de plano.
const FILE_MIN: u32 = 0x6C;

/// El archivo de plano (`archive_file_*`) y la reserva de su búfer.
pub trait PlanFileHost {
    /// Abre el archivo con el formato de ruta `attempt` (0: primero, 1: segundo, tras fallar el primero).
    fn open(&mut self, attempt: u32) -> bool;
    /// `archive_file_get_length` (0x00402860).
    fn length(&mut self) -> u32;
    /// `operator new` (0x0064F7B9) del búfer de lectura: `false` si no hay memoria.
    fn alloc(&mut self, bytes: u32) -> bool;
    /// `archive_file_read` (0x004027A0): lo que se lee de verdad (puede ser menos de lo pedido).
    fn read(&mut self, bytes: u32) -> Vec<u8>;
    /// `archive_file_close` (0x00402780).
    fn close(&mut self);
}

struct NoAlloc;

impl MaskHost for NoAlloc {
    fn alloc_mask(&mut self, _bytes: u32) -> bool {
        false
    }
}

impl Site {
    fn plan_reset_tables(t: &mut Town) {
        t.0[PLAN + 0xC..PLAN + 0x14].fill(0xFF);
        t.0[PLAN + 0x14..PLAN + 0x1C].fill(0xFF);
        t.0[PLAN + 0x1C..PLAN + 0x2C].fill(0xFF);
        t.0[PLAN + 0x2C..PLAN + 0x4C].fill(0xFF);
        t.0[PLAN + 0x5C..PLAN + 0x68].fill(0xFF);
    }

    /// `town_plan_init_default`: plano vacío, sin rejilla (no libera la anterior).
    pub fn plan_init_default(&mut self, t: &mut Town) {
        t.set_u32(PLAN + 8, 0);
        t.set_u32(PLAN + 4, 0);
        self.cells = Vec::new();
        Self::plan_reset_tables(t);
        t.0[PLAN + 0x4C..PLAN + 0x5C].fill(0);
        t.set_u32(PLAN, 1);
    }

    /// `town_plan_free_data`.
    pub fn plan_free_data(&mut self) {
        self.cells = Vec::new();
    }

    /// `town_plan_reset`: vuelve a poner los puntos del plano a «sin valor» y libera la rejilla.
    pub fn plan_reset(&mut self, t: &mut Town) {
        t.set_u32(PLAN, 1);
        Self::plan_reset_tables(t);
        self.cells = Vec::new();
    }

    /// `town_plan_resize(filas, columnas)`: una rejilla nueva con el contenido que cabe de la anterior (las
    /// celdas nuevas valen `0x20`) y los puntos del plano que quedan fuera, anulados. Con un tamaño no positivo
    /// la deja vacía y anula los puntos `Town+0x7B0..+0x7B7`. Falla sin tocar nada si no hay memoria.
    pub fn plan_resize(&mut self, t: &mut Town, rows: i32, stride: i32, host: &mut dyn MaskHost) -> bool {
        let size = rows.wrapping_mul(stride);
        let mut grid = Vec::new();
        if size > 0 {
            if !host.alloc_mask(size as u32) {
                return false;
            }
            let (old_rows, old_stride) = (t.i32(off::PLAN_ROWS), t.i32(off::PLAN_STRIDE));
            let copied = old_rows.min(rows);
            grid.reserve(size as usize);
            for r in 0..copied.max(0) as usize {
                let row = &self.cells[r * old_stride as usize..];
                if stride > old_stride {
                    grid.extend_from_slice(&row[..old_stride.max(0) as usize]);
                    grid.resize(grid.len() + (stride - old_stride) as usize, 0x20);
                } else {
                    grid.extend_from_slice(&row[..stride.max(0) as usize]);
                }
            }
            grid.resize(size as usize, 0x20);
        } else {
            t.0[PLAN + 0xC..PLAN + 0x14].fill(0xFF);
        }
        self.cells = grid;
        t.set_i32(off::PLAN_ROWS, rows);
        t.set_i32(off::PLAN_STRIDE, stride);
        for j in 0..0x26 {
            let (r, c) = (PLAN + 0xC + 2 * j, PLAN + 0xD + 2 * j);
            if rows <= t.u8(r) as i32 {
                t.set_u8(r, 0xFF);
            }
            if stride <= t.u8(c) as i32 {
                t.set_u8(c, 0xFF);
            }
        }
        true
    }

    /// `town_plan_load_from_file`: lee el plano del archivo (versión `0x85`); si no existe, es demasiado corto o
    /// la lectura no devuelve todo lo pedido, deja un plano vacío con `plan_resize(0, 0)`.
    pub fn plan_load_from_file(&mut self, t: &mut Town, host: &mut dyn PlanFileHost) {
        let opened = host.open(0) || host.open(1);
        if opened {
            let length = host.length();
            if length >= FILE_MIN && host.alloc(length) {
                let data = host.read(length);
                host.close();
                if data.len() as u32 == length {
                    self.deserialize_plan_data(t, &data, FILE_VERSION);
                    return;
                }
            } else {
                host.close();
            }
        }
        self.plan_resize(t, 0, 0, &mut NoAlloc);
    }
}
