//! Intersección del proyectil con el casco (`0x0060A73C`): devuelve la máscara de banda (AL) del original.
//!
//! Aritmética entera de 32 bits con desbordamiento, sin `sin()`: usa el cuarto de seno de `wind::SINE_QUARTER`.
//! Los cuatro cascos son la tabla de 0x0067AB30 (cinco puntos i16 por tipo).

use super::wind::table_sine;

/// Cascos de cinco puntos: proa-izquierda, proa, proa-derecha, popa-derecha, popa-izquierda.
pub const HULLS: [[(i16, i16); 5]; 4] = [
    [(-19, -31), (0, -55), (19, -31), (19, 47), (-19, 47)],
    [(-21, -36), (0, -66), (21, -36), (19, 55), (-19, 55)],
    [(-25, -14), (0, -54), (25, -14), (19, 67), (-19, 67)],
    [(-22, -25), (0, -67), (22, -25), (19, 81), (-19, 81)],
];

/// Paso de velocidad hacia el segmento: parte entera del Q16 más el signo; en modo directo se multiplica por 16.
pub fn velocity_step(raw: i32, direct: bool) -> i32 {
    let result = (raw >> 16).wrapping_add(raw.signum());
    if direct {
        result.wrapping_shl(4)
    } else {
        result
    }
}

fn cross(a: (i32, i32), b: (i32, i32)) -> i32 {
    a.0.wrapping_mul(b.1).wrapping_sub(a.1.wrapping_mul(b.0))
}

fn accepted(numerator: i32, denominator: i32) -> bool {
    let (n, d) = if denominator < 0 {
        (numerator.wrapping_neg(), denominator.wrapping_neg())
    } else {
        (numerator, denominator)
    };
    0 <= n && n <= d
}

/// Prueba de una arista con las dos razones normalizadas por separado; el denominador cero no se rechaza.
fn edge_test(previous: (i32, i32), current: (i32, i32), segment: (i32, i32)) -> bool {
    let reverse = (previous.0.wrapping_sub(current.0), previous.1.wrapping_sub(current.1));
    let forward = (current.0.wrapping_sub(previous.0), current.1.wrapping_sub(previous.1));
    accepted(cross(segment, previous), cross(segment, reverse))
        && accepted(cross(previous, forward), cross(segment, forward))
}

/// Posiciones y velocidad en punto fijo Q16 (parte alta = entero con signo de 16 bits).
#[derive(Clone, Copy, Debug)]
pub struct HullQuery {
    pub hull_type: usize,
    pub heading: u8,
    pub ship: (u32, u32),
    pub projectile: (u32, u32),
    pub velocity: (i32, i32),
    pub direct: bool,
}

/// Máscara de banda: 0 sin impacto, 1/2 una banda, 3 ambas.
pub fn hit_mask(q: &HullQuery) -> u8 {
    let sine = table_sine(q.heading);
    let cosine = table_sine(q.heading.wrapping_add(64));
    let ship = ((q.ship.0 >> 16) as i16 as i32, (q.ship.1 >> 16) as i16 as i32);
    let proj = (
        (q.projectile.0 >> 16) as i16 as i32,
        (q.projectile.1 >> 16) as i16 as i32,
    );
    let mut points = [(0i32, 0i32); 5];
    for (k, &(x, y)) in HULLS[q.hull_type].iter().enumerate() {
        let (x, y) = (i32::from(x), i32::from(y));
        let rx = cosine.wrapping_mul(x).wrapping_sub(sine.wrapping_mul(y)) >> 16;
        let ry = cosine.wrapping_mul(y).wrapping_add(sine.wrapping_mul(x)) >> 16;
        points[k] = (
            rx.wrapping_add(ship.0).wrapping_sub(proj.0),
            ry.wrapping_add(ship.1).wrapping_sub(proj.1),
        );
    }
    let segment = (
        velocity_step(q.velocity.0, q.direct),
        velocity_step(q.velocity.1, q.direct),
    );
    let mut previous = points[4];
    let mut mask = 0u8;
    for (edge, &current) in points.iter().enumerate() {
        if edge_test(previous, current, segment) {
            match edge {
                0 | 4 => mask |= 1,
                1 | 2 => mask |= 2,
                _ => mask = 3,
            }
        }
        previous = current;
    }
    mask
}
