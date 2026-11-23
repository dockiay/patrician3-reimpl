//! Figuras que caminan por la ciudad (J02): reparto inicial `populate` (`0x00595CA0`) y selección del fotograma visible.
//!
//! Implementado (sin oráculo x86 de la parte aleatoria: el estado de `rand` del original no se conoce, solo la lógica):
//! recuento por clase `(población + 0x31) / 0x32`, rumbo inicial `rand % 4 * 0x40 + 0x20`, variante `rand % nvariantes`
//! si las variantes están activas, posición `casilla -> píxel << 16`. Quedan fuera el caminar libre y por rutas.

use crate::geom::Geom;
use crate::mover::MoverAnim;
use p3_assets::lcr::{Frame, Lcr};

/// Tipos de entidad de las cuatro clases de población (índice 0..3 = población 3..0): `4, 0, 1, 2`.
pub const POPULATE_KINDS: [u32; 4] = [4, 0, 1, 2];

/// Número de figuras de una clase con `population` habitantes (`0x595CA0`).
pub fn class_count(population: i32) -> i32 {
    (population + 0x31) / 0x32
}

#[derive(Clone, Debug)]
pub struct Walker {
    pub kind: u32,
    /// Posición 16.16.
    pub x: i32,
    pub y: i32,
    pub heading: u8,
    pub frame: u8,
    pub variant: u32,
    pub anim: MoverAnim,
}

impl Walker {
    /// Crea una figura en la casilla `(cx, cy)` como `populate`: píxel de la casilla en 16.16, rumbo y variante dados por el azar.
    pub fn spawn(geom: &Geom, kind: u32, cell: (i32, u32), rand_heading: u32, rand_variant: u32, variants: u32, enabled: bool) -> Self {
        let (px, py) = geom.cell_to_pixel(cell.0, cell.1);
        let variant = if enabled && variants > 0 { rand_variant % variants } else { 0 };
        Walker {
            kind,
            x: px << 16,
            y: py << 16,
            heading: ((rand_heading % 4) * 0x40 + 0x20) as u8,
            frame: 0,
            variant,
            anim: MoverAnim::default(),
        }
    }

    /// Fotograma visible y su destino en pantalla `(x_px + dx - vista_x, y_px + dy - vista_y)` (`0x559470`).
    pub fn visible<'a>(&self, lcr: &'a Lcr, view: (i32, i32)) -> Option<(&'a Frame, i32, i32)> {
        let frames = lcr.dirs.get(lcr.direction_of(self.heading))?;
        let f = frames.get(usize::from(self.frame))?;
        Some((f, (self.x >> 16) + i32::from(f.rect.dx) - view.0, (self.y >> 16) + i32::from(f.rect.dy) - view.1))
    }
}

/// Orden de dibujo: por casilla del pie, filas de arriba abajo (`0x591440`).
pub fn draw_order(walkers: &[Walker], geom: &Geom, mask: &[u8]) -> Vec<usize> {
    let mut v: Vec<(u32, u32, usize)> = walkers
        .iter()
        .enumerate()
        .map(|(i, w)| {
            let ((cx, cy), _) = geom.pixel_to_cell(w.x >> 16, w.y >> 16, mask);
            (cy, cx, i)
        })
        .collect();
    v.sort();
    v.into_iter().map(|t| t.2).collect()
}

/// Entradas de `pick_spawn` (`0x0059A4F0`): el `Class22` y el plano de la ciudad reducidos a lo que lee la función.
pub struct PickEnv<'a> {
    pub cols: u32,
    pub rows: u32,
    /// Paso del plano (`plan+8`): la celda de `(x, y)` está en `stride * x + y`.
    pub stride: u32,
    pub plan: &'a [u8],
    /// Puntos de aparición de la clave 0 o 1 (`+0xC69C`).
    pub points: &'a [(u32, u32)],
    pub geom: &'a Geom,
    /// Origen y tamaño de la vista (`+0xC650/+0xC654`, `+0x2C/+0x30`).
    pub view: (u32, u32, u32, u32),
}

/// Casilla de aparición (`0x0059A4F0`) con la clave elegida por `select` (0 o 1), `view_check` (`param_4`) y el indicador de `0x593580`
/// (`param_5`). `free` es `0x593580(casilla, &indicador, 0)`. Devuelve `(encontrada, x, y)`; el par queda escrito aunque no se halle.
/// Primero 200 intentos aleatorios sobre toda la rejilla y después los puntos de la tabla desde un punto de partida aleatorio.
pub fn pick_spawn(env: &PickEnv, select: bool, view_check: bool, flag: bool, rand: &mut dyn FnMut() -> u32, free: &mut dyn FnMut(u32, u32, u8) -> bool) -> (bool, u32, u32) {
    let count = env.points.len() as u32;
    let start = rand() % count;
    let (mut x, mut y) = (0u32, 0u32);
    let mut i: i32 = -200;
    while i < count as i32 {
        if i < 0 {
            x = rand() % env.cols;
            y = rand() % env.rows;
        } else {
            let k = ((i as u32).wrapping_add(start)) % count;
            (x, y) = env.points[k as usize];
        }
        let c = env.plan.get((env.stride.wrapping_mul(x).wrapping_add(y)) as usize).copied().unwrap_or(0) as i8;
        let street = c < 0;
        if street == select && free(x, y, if flag { 0xFF } else { 0 }) {
            let g = env.geom;
            let (px, py) = g.cell_to_pixel(x as i32, y);
            let (px, py) = (px as u32, py);
            let (vx, vy, vw, vh) = env.view;
            let out_of_view = !view_check
                || px.wrapping_add(0x32) < vx
                || vx.wrapping_add(0x32).wrapping_add(vw) < px
                || (py as u32).wrapping_add(0x32) < vy
                || vh.wrapping_add(0x32).wrapping_add(vy) < py as u32;
            if out_of_view {
                break;
            }
        }
        i += 1;
    }
    (i != count as i32, x, y)
}

/// Reparto inicial de peatones (`0x00595CA0`, primera parte): por cada una de las cuatro clases de población, `(población + 0x31) / 0x32`
/// figuras en casillas de calle (`pick_spawn` con clave 1), con rumbo `rand % 4 * 0x40 + 0x20` y, si hay variantes, la variante `rand % n`.
/// `pops[i]` es la población de la clase `i` (`Town+0x2E4-4i`, es decir, `Town::pop(3 - i)`). El orden de los `rand` se ha leído del
/// desensamblado (elegir casilla, rumbo, variante); no hay oráculo del resto del constructor (`0x557D10`, `0x559D60`).
pub fn populate_pedestrians(
    env: &PickEnv,
    pops: [i32; 4],
    variants: &dyn Fn(u32) -> u32,
    variants_enabled: bool,
    rand: &mut dyn FnMut() -> u32,
    free: &mut dyn FnMut(u32, u32, u8) -> bool,
) -> Vec<Walker> {
    let mut out = Vec::new();
    if env.points.is_empty() || env.cols == 0 || env.rows == 0 {
        return out;
    }
    for (idx, &pop) in pops.iter().enumerate() {
        let kind = POPULATE_KINDS[idx];
        for _ in 0..class_count(pop).max(0) {
            let (found, x, y) = pick_spawn(env, true, false, true, rand, free);
            if !found {
                continue;
            }
            let heading = rand();
            let nv = variants(kind);
            let variant = if variants_enabled && nv > 0 { rand() } else { 0 };
            out.push(Walker::spawn(env.geom, kind, (x as i32, y), heading, variant, nv, variants_enabled));
        }
    }
    out
}

/// `0x00593580` sin entidades en la casilla: `layer1` es la clase de ficha (`LetSheet::field_2d`, método virtual `+0x50`) del objeto de la capa 1,
/// o `None` si no hay; `layer0` la de la ficha de suelo. La casilla es válida si no hay objeto (o es de clase `0x30`/`0x31`) y el suelo es de clase
/// `0x10`, `0x17`, `0x18` o `0x28`; con el indicador `0xFF` (el que usa `populate`) el suelo `0x28` queda excluido. Las entidades con `+0x16 != 0`
/// ya presentes en la casilla la bloquean en el original; aquí no se modelan.
pub fn spawn_cell_free(layer1: Option<u32>, layer0: u32, flag: u8) -> bool {
    if let Some(k) = layer1 {
        if !(0x30..0x32).contains(&k) {
            return false;
        }
    }
    match layer0 {
        0x28 => flag != 0xFF,
        0x10 | 0x17 | 0x18 => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn counts() {
        assert_eq!(class_count(0), 0);
        assert_eq!(class_count(1), 1);
        assert_eq!(class_count(0x32), 1);
        assert_eq!(class_count(0x33), 2);
    }
    #[test]
    fn spawn_heading_and_variant() {
        let g = Geom { cols: 36, rows: 23, cell_w: 34, cell_h: 21, off_x: 0, off_y: 0 };
        let w = Walker::spawn(&g, 0, (3, 4), 7, 10, 4, true);
        assert_eq!(w.heading, 0xE0);
        assert_eq!(w.variant, 2);
        assert_eq!(Walker::spawn(&g, 0, (3, 4), 0, 10, 4, false).variant, 0);
        assert_eq!(w.x >> 16, g.cell_to_pixel(3, 4).0);
    }
}
