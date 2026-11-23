//! Siguiente destino del caminante libre (`town_walker_choose_next_target`, `0x0055D9D0`) sobre la memoria del juego.
//!
//! Elige el destino `(+0x50, +0x52)` entre los ocho puntos fijos de la ciudad (`Town+0x7D0`) y las puertas de sus
//! edificios con calle delante (la cadena de `Town+0x764`, siguiendo a las sedes del mundo desde `Town+0x784`): de los
//! que están en la ventana de `0x353 x 0x227` píxeles, el más lejano (la mayor de `|dx|` y `|dy|·17/11`); si alguno
//! queda fuera, uno de ellos al azar (`rand % n`, el `n`-ésimo que sale). Solo cuentan los que no hacen volver
//! atrás en los dos ejes respecto al destino anterior. Con una fiesta en marcha o cercana (`Town+0x2D0`,
//! tarea programada de tipo 7, `0x004E2314`) solo los puntos fijos y sin esa condición. Los mendigos (clase 4)
//! van al primer edificio de tipo `0x1F` mientras dura su turno (`+0x4C`). Sin candidatos, `+0x55 = 3` y
//! `+0x56 = +0x57 = 0`.
//!
//! Es la versión sobre `Mem` de `p3_render::walk::choose_target`.
use super::{s16, vcall, view, FigureHost};
use crate::economy::siege_battle::building_record;
use crate::flatmem::Mem;

/// Reloj del mundo (`0x006DE4B4`).
pub const WORLD_CLOCK: u32 = 0x006D_E4B4;
/// Tareas programadas (`0x006DD73C`): tabla `[+0]`, primera `+8`, recuento `+0xC`; registros de 24 bytes.
pub const TASKS: u32 = 0x006D_D73C;
/// Mundo (`0x006DE4A0`): número de sedes `+8`, tabla `+0x74` de `0x44C` bytes.
pub const WORLD: u32 = 0x006D_E4A0;

/// `scheduled_tasks_find_celebration_in_town` (`0x004E2314`): instante de la primera tarea de tipo 7 en estado 2
/// de la ciudad.
pub fn celebration_time(m: &mut dyn Mem, city: u8) -> Option<u32> {
    let table = m.r32(TASKS);
    let count = u32::from(m.r16(TASKS + 0xC));
    let mut i = u32::from(m.r16(TASKS + 8));
    while i < count {
        let e = table.wrapping_add(i * 24);
        if m.r16(e + 6) == 7 && m.r32(e + 0xC) == u32::from(city) && m.r32(e + 0x10) == 2 {
            return Some(m.r32(e));
        }
        i = u32::from(m.r16(e + 4));
    }
    None
}

/// `0x00530400`: sede `i` del mundo o 0 si no es menor que el recuento.
fn office(m: &mut dyn Mem, i: u32) -> u32 {
    if i >= u32::from(m.r16(WORLD + 8)) {
        return 0;
    }
    m.r32(WORLD + 0x74).wrapping_add(i.wrapping_mul(0x44C))
}

struct Pick {
    px: i32,
    py: i32,
    left_of: bool,
    above: bool,
    count: u16,
    best: u16,
}

impl Pick {
    /// ¿Cuenta el candidato del píxel `(tx, ty)`? Sí, si el destino es la posición actual o no vuelve atrás en ningún
    /// eje (comparación sin signo con la posición, como el original).
    fn ahead(&self, m: &mut dyn Mem, w: u32, tx: i32, ty: i32) -> bool {
        if i32::from(m.r16(w + 0x50)) == self.px && i32::from(m.r16(w + 0x52)) == self.py {
            return true;
        }
        ((tx as u32) < self.px as u32) != self.left_of && ((ty as u32) < self.py as u32) != self.above
    }

    fn consider(&mut self, m: &mut dyn Mem, h: &mut dyn FigureHost, w: u32, tx: i32, ty: i32) {
        let dx = tx.wrapping_sub(self.px).wrapping_abs();
        let dy = ty.wrapping_sub(self.py).wrapping_abs();
        if dx > 0x352 || dy > 0x226 {
            self.count = self.count.wrapping_add(1);
            let n = i32::from(self.count);
            if (h.rand(m) as i32).wrapping_rem(n) == 0 {
                m.w16(w + 0x50, tx as u16);
                m.w16(w + 0x52, ty as u16);
            }
            return;
        }
        if self.count != 0 {
            return;
        }
        let mut d = dy.wrapping_mul(17) / 11;
        if (d & 0xFFFF) < dx {
            d = dx;
        }
        if (d as u16) > self.best {
            self.best = d as u16;
            m.w16(w + 0x50, tx as u16);
            m.w16(w + 0x52, ty as u16);
        }
    }
}

fn wander(m: &mut dyn Mem, w: u32) {
    m.w8(w + 0x55, 3);
    m.w8(w + 0x56, 0);
    m.w8(w + 0x57, 0);
}

/// `0x0055D9D0(ecx = caminante)`.
pub fn choose_next_target(m: &mut dyn Mem, h: &mut dyn FigureHost, w: u32) {
    let v = view(m);
    let city = m.r32(v + 0xC324);
    let town = m.r32(super::TOWNS).wrapping_add(city.wrapping_mul(0x9F8));
    let px = s16(m, w + 0xE);
    let py = s16(m, w + 0x12);
    let mut p = Pick {
        px,
        py,
        left_of: i32::from(m.r16(w + 0x50)) < px,
        above: i32::from(m.r16(w + 0x52)) < py,
        count: 0,
        best: 0,
    };
    let mut chain = u32::from(m.r16(town + 0x784));
    let event = celebration_time(m, city as u8).unwrap_or(0xFFFF_FFFF);
    let clock = m.r32(WORLD_CLOCK);
    let near = clock.wrapping_sub(m.r32(town + 0x2D0)) < 0x300 || event.wrapping_sub(clock) < 0x300;
    if vcall(m, h, w, 0x20, &[]) as u8 == 4 {
        if m.r32(town + 0x2C8) & 0x80_0000 != 0 {
            let c = m.r32(WORLD_CLOCK);
            m.w32(w + 0x4C, c);
        }
        if m.r32(WORLD_CLOCK).wrapping_sub(m.r32(w + 0x4C)) < 0x300 {
            let mut i = u32::from(m.r16(town + 0x764));
            while i < u32::from(m.r16(town + 0x776)) {
                let rec = building_record(m, town, i);
                if m.r8(rec + 3) == 0x1F {
                    let col = u32::from(m.r8(rec));
                    let row = u32::from(m.r8(rec + 1)).wrapping_sub(4);
                    let (tx, ty) = h.cell_to_pixel(m, col, row);
                    m.w16(w + 0x50, tx as u16);
                    m.w16(w + 0x52, ty as u16);
                    wander(m, w);
                    return;
                }
                i = u32::from(m.r16(rec + 4));
            }
        }
    }
    for k in 0..8 {
        let (a, b) = (m.r8(town + 0x7D0 + 2 * k), m.r8(town + 0x7D1 + 2 * k));
        if a == 0xFF || b == 0xFF {
            continue;
        }
        let (tx, ty) = h.cell_to_pixel(m, u32::from(a), u32::from(b));
        if near || p.ahead(m, w, tx, ty) {
            p.consider(m, h, w, tx, ty);
        }
    }
    if !near {
        let mut i = u32::from(m.r16(town + 0x764));
        loop {
            if i >= u32::from(m.r16(town + 0x776)) {
                if chain as u16 >= m.r16(WORLD + 8) {
                    break;
                }
                let o = office(m, chain & 0xFFFF);
                chain = u32::from(m.r16(o + 0x2CA));
                i = u32::from(m.r16(o + 0x2CE));
                continue;
            }
            let rec = building_record(m, town, i);
            if m.r8(rec + 3) < 0x2F {
                let col = u32::from(m.r8(rec));
                let row = u32::from(m.r8(rec + 1)).wrapping_sub(2);
                let here = m.r32(super::TOWNS).wrapping_add(m.r32(v + 0xC324).wrapping_mul(0x9F8)) + 0x7A4;
                let cell = m.r32(here + 8).wrapping_mul(col).wrapping_add(m.r32(here + 0x68)).wrapping_add(row);
                if m.r8(cell) & 0x80 != 0 {
                    let (tx, ty) = h.cell_to_pixel(m, col, row);
                    if p.ahead(m, w, tx, ty) {
                        p.consider(m, h, w, tx, ty);
                    }
                }
            }
            i = u32::from(m.r16(rec + 4));
        }
    }
    if p.count == 0 && p.best == 0 {
        wander(m, w);
    }
}
