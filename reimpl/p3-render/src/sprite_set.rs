//! Conjunto de sprites en memoria de la escena: lo que deja el cargador `0x558770` a partir de un `.lcr` (formato medido en el ejecutable;
//! `oracle/x86_sprite_set.py` compara los bytes de la memoria, con sus punteros, contra el cargador real).
//!
//! Disposición del conjunto (`set`): `+4` campo C, `+8` clase, `+0xC` campo 10, `+0x10` intervalo (ms), `+0x14` direcciones `N`, `+0x18` tabla `N` de
//! punteros a los fotogramas `{x, y, w, h, dx, dy}` (12 bytes), `+0x1C` ídem de máscara (0 si no hay), `+0x20` tabla `N` de recuentos, `+0x24` tabla `N` de
//! `{K, celda_del_ancla, ptr}` (12 bytes por fotograma) si `+0x30` es 1, `+0x28` recuento de estados, `+0x2C` estados `{primero, último, id}`,
//! `+0x34`/`+0x38` asas de las rutas de hoja y de máscara (las da `0x557BB0`) y `+0x3C` máximo de celdas por fotograma.

use crate::scene_math::Mem;
use p3_assets::lcr::{Lcr, SheetRect};

pub trait SetMem: Mem {
    /// `0x64F7B9` (`operator new`) seguido de `_memset(0)`.
    fn alloc(&mut self, size: u32) -> u32;
    /// Tablas por clase que rellena la cabecera extra (`vtable+4`): añade el valor `value` al final de la tabla número `table` de la clase `kind`
    /// (el original amplía con `0x63A29F`, un `realloc`, el vector global de la clase y escribe la entrada nueva).
    fn class_table_push(&mut self, kind: u32, table: usize, value: u32);
}

/// Cabecera extra de las clases que guardan sus parámetros en tablas globales (`0x577710`, `0x5672B0`, `0x562280`, `0x572960`, `0x579B40`, `0x564660`,
/// `0x576530`): número de tablas y tamaño de cada entrada (1 byte; la clase 29 guarda dos palabras en una sola tabla de 8 bytes). La primera palabra se salta.
/// El resto de clases usan `0x558E90`: la primera palabra es un recuento `n` y se saltan `n + 1` palabras.
pub fn class_tables(kind: u32) -> Option<usize> {
    match kind {
        8 | 18 => Some(1),
        21 => Some(2),
        9..=11 => Some(4),
        29 => Some(1),
        _ => None,
    }
}

fn rect(m: &mut dyn Mem, a: u32, r: &SheetRect) {
    m.w16(a, r.x);
    m.w16(a + 2, r.y);
    m.w16(a + 4, r.w);
    m.w16(a + 6, r.h);
    m.w16(a + 8, r.dx as u16);
    m.w16(a + 10, r.dy as u16);
}

/// Carga `l` en `set` con el mismo orden de reservas que `0x558770`. `mask_handle` y `sheet_handle` son las asas que `0x557BB0` devuelve.
pub fn load_set(env: &mut dyn SetMem, set: u32, l: &Lcr, mask_handle: u32, sheet_handle: u32) {
    let n = l.directions;
    env.w32(set + 8, l.kind);
    env.w32(set + 4, l.field_c);
    env.w32(set + 0xC, l.field_10);
    env.w32(set + 0x14, n);
    let frames_tab = env.alloc(n << 2);
    env.w32(set + 0x18, frames_tab);
    let counts_tab = env.alloc(n << 2);
    env.w32(set + 0x20, counts_tab);
    env.w32(set + 0x10, l.interval_ms);
    env.w8(set + 0x30, l.has_cells as u8);
    let mut cells_tab = 0;
    if l.has_cells {
        cells_tab = env.alloc(n << 2);
        env.w32(set + 0x24, cells_tab);
    }
    if let Some(tables) = class_tables(l.kind) {
        for t in 0..tables {
            let v = l.extra_header.get(1 + t).copied().unwrap_or(0);
            env.class_table_push(l.kind, t, v);
            if l.kind == 29 {
                env.class_table_push(l.kind, 1, l.extra_header.get(2).copied().unwrap_or(0));
            }
        }
    }
    env.w32(set + 0x28, l.states.len() as u32);
    if !l.states.is_empty() {
        let st = env.alloc(l.states.len() as u32 * 12);
        env.w32(set + 0x2C, st);
        for (i, s) in l.states.iter().enumerate() {
            let a = st + i as u32 * 12;
            env.w32(a, s.first);
            env.w32(a + 4, s.last);
            env.w32(a + 8, s.id);
        }
    }
    let masked = !l.mask_path.is_empty();
    let mut mask_tab = 0;
    if masked {
        mask_tab = env.alloc(n << 2);
        env.w32(set + 0x1C, mask_tab);
        env.w32(set + 0x38, mask_handle);
    }
    let mut max_cells = 0u32;
    for (d, frames) in l.dirs.iter().enumerate() {
        let d = d as u32;
        let cnt = frames.len() as u32;
        env.w32(counts_tab + d * 4, cnt);
        let fr = env.alloc(cnt * 12);
        env.w32(frames_tab + d * 4, fr);
        let mr = if masked {
            let a = env.alloc(cnt * 12);
            env.w32(mask_tab + d * 4, a);
            a
        } else {
            0
        };
        let cr = if l.has_cells {
            let a = env.alloc(cnt * 12);
            env.w32(cells_tab + d * 4, a);
            a
        } else {
            0
        };
        for (i, f) in frames.iter().enumerate() {
            let i = i as u32;
            if masked {
                rect(env, mr + i * 12, f.mask.as_ref().expect("máscara"));
            }
            if l.has_cells {
                let k = f.cells.len() as u32;
                env.w32(cr + i * 12, k);
                env.w32(cr + i * 12 + 4, 0);
                let list = env.alloc(k * 12);
                env.w32(cr + i * 12 + 8, list);
                max_cells = max_cells.max(k);
                for (j, c) in f.cells.iter().enumerate() {
                    rect(env, list + j as u32 * 12, c);
                }
                env.w32(cr + i * 12 + 4, f.anchor_cell);
            }
            rect(env, fr + i * 12, &f.rect);
        }
    }
    env.w32(set + 0x3C, max_cells);
    env.w32(set + 0x34, sheet_handle);
}
