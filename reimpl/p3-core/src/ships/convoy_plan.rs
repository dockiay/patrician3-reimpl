//! `0x00505C90`: destino y ruta de un convoy que no avanza por su orden (estado 0x12, reintento).
//!
//! Suma el estado de los barcos del convoy, elige un punto de destino (puerto libre, puerto del objeto de la facción, barco objetivo
//! o punto de ruta aleatorio) y lo fija en el barco principal (`0x005172A0`). Si el barco ya está en el puerto de su objeto, saca el
//! barco del orden (`0x0050D040`), lo absorbe en el objeto (`0x00514B80`) y devuelve cierto.
//!
//! Los globales que el original lee fuera del gestor van en [`PlanEnv`]; `0x5172A0` y `0x514B80` son ganchos del anfitrión.

use super::remove::{CONVOY_SIZE, SHIP_SIZE};
use super::tick::{ConvoyError, ConvoyHost, TickWorld};
use super::{r16, r32, w16, w32};
use crate::route::planner::{find_town_at_point, TownSlot, TOWN_NOT_FOUND};

/// Tamaño de la tabla de puertos `0x006DDB90`.
pub const PORT_SLOTS: usize = 40;
/// `0x00673660`: tripulación mínima por clase de casco.
pub const CREW_MIN: [u8; 4] = [5, 8, 10, 12];
/// `0x00673564`: tres rectángulos `(x_min, y_min, x_max, y_max)` cuyo interior desvía al barco objetivo a la ruta aleatoria.
pub const RECTS: [(i16, i16, i16, i16); 3] = [(442, 1521, 658, 1691), (2265, 147, 2332, 453), (1497, 1263, 1578, 1444)];
const GUARD: usize = 0x10000;

/// Globales y tablas que consulta `0x505C90` fuera del gestor de barcos.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PlanEnv {
    /// `[0x006CC3E8] + 0x13`.
    pub difficulty: u8,
    /// `0x00672CC8` (solo el byte bajo).
    pub g672cc8: u32,
    /// `0x006DE4AA`.
    pub merchant_count: u16,
    /// `0x006DE4A4`.
    pub clock: u32,
    /// `0x006DE4B0`.
    pub town_count: u32,
    /// `0x006DE4B4`.
    pub time: u32,
    /// `0x0066E0D0`.
    pub scale: i32,
    /// Tabla `0x006DDB90` (rectángulo y ancla) y su byte de ocupación `+0x2F`.
    pub slots: Vec<TownSlot>,
    pub slot_flag: Vec<u8>,
    /// `0x006DE4B8`: orden de las casillas.
    pub order: Vec<u8>,
    /// Banderas `+0x2C8` de las 16 primeras ciudades de `0x006DE508`.
    pub town_flags: Vec<u32>,
    /// Tipo (`+0xE`) de cada uno de los cinco objetos `0x006DD7AC`, si existen.
    pub objects: [Option<u8>; 5],
    /// Objeto `0x006CBDC8`: número de puntos de ruta por ciudad (`+0x20`) y lista de puntos empaquetados (`+0x24`).
    pub route_counts: Vec<u32>,
    pub route_points: Vec<Vec<u32>>,
}

/// `0x0046C380`: generador del objeto local (semilla, `next(mod)`).
struct Lcg(u32);

impl Lcg {
    fn next(&mut self, modulus: u32) -> Result<u32, ConvoyError> {
        let mut v = 0xA205_B063u32.wrapping_sub(self.0.wrapping_mul(0x44BF_19B3));
        let odd = v & 1 != 0;
        v >>= 1;
        if odd {
            v |= 0x8000_0000;
        }
        self.0 = v;
        if modulus == 0 {
            return Err(ConvoyError::Divide);
        }
        Ok(v % modulus)
    }
}

/// `0x0051A470`: primer objeto vigente a partir del que indica el barco (`ship+0x158`).
pub fn first_object(objects: &[Option<u8>; 5], preferred: u8) -> u8 {
    if (preferred as usize) < 5 && objects[preferred as usize].is_some() {
        return preferred;
    }
    for k in 0..5u8 {
        if objects[k as usize].is_some() {
            return k;
        }
    }
    0
}

/// `0x005159C0`: puerto libre número `arg % libres`; 0 si todos están ocupados.
pub fn free_port(env: &PlanEnv, arg: u32) -> u8 {
    let mut used = [false; PORT_SLOTS];
    for (i, u) in used.iter_mut().enumerate() {
        *u = env.slot_flag.get(i).copied().unwrap_or(0) != 0;
    }
    for k in (0..env.town_count as usize).rev() {
        let s = env.order.get(k).copied().unwrap_or(0) as usize;
        if s < PORT_SLOTS {
            used[s] = true;
        }
    }
    for kind in env.objects.iter().flatten() {
        if (*kind as usize) < PORT_SLOTS {
            used[*kind as usize] = true;
        }
    }
    let free: Vec<u8> = (0..PORT_SLOTS as u8).filter(|&i| !used[i as usize]).collect();
    if free.is_empty() {
        0
    } else {
        free[(arg % free.len() as u32) as usize]
    }
}

fn div1000(a: i32) -> i32 {
    let hi = ((a as i64 * 0x1062_4DD3i64) >> 32) as i32;
    let q = hi >> 6;
    q.wrapping_add(((q as u32) >> 31) as i32)
}

fn sext16(v: i32) -> i32 {
    v as i16 as i32
}

/// `slot`: `None` es `manager+0xEC`; `Some(c)` es el enlace `+8` del convoy `c`.
pub fn plan_destination(
    w: &mut TickWorld,
    cv: u16,
    slot: Option<usize>,
    env: &PlanEnv,
    host: &mut dyn ConvoyHost,
) -> Result<bool, ConvoyError> {
    let b = cv as usize * CONVOY_SIZE;
    let count = r16(w.remove.manager, 0xF4);
    let ship = |i: u16| i as usize * SHIP_SIZE;
    // Recorrido de los barcos del convoy.
    let (mut n, mut flag_a, mut flag_b) = (0u32, false, false);
    let (mut sum_a, mut sum_b) = (0i32, 0i32);
    let mut s = r16(w.remove.convoys, b + 0xA);
    let mut g = 0;
    if s < count {
        loop {
            let o = ship(s);
            let sh = &mut *w.remove.ships;
            let c3f = sh[o + 0x3F] as i8;
            let next = r16(sh, o + 6);
            if c3f < 2 {
                sh[o + 0x3F] = env.difficulty.wrapping_add(c3f as u8).wrapping_add(1);
            }
            n += 1;
            let crew = CREW_MIN[(sh[o + 0xE] & 3) as usize] as u16;
            let mut wanted = r16(sh, o + 0x40) < crew;
            if !wanted {
                let ecx = (r32(sh, o + 0x18) as i32 >> 3).wrapping_mul(5).wrapping_mul(2);
                wanted = ecx < r32(sh, o + 0x14) as i32 || (env.g672cc8 & 0xFF) as i32 * 2 > r32(sh, o + 0x120) as i32;
            }
            let mut counted = true;
            if wanted {
                if (sh[o + 0x15D] as u16) >= env.merchant_count {
                    flag_a = true;
                    counted = false;
                }
            }
            if counted {
                let loss = r32(sh, o + 0x118) as i32;
                if loss > 0 {
                    flag_a = true;
                } else {
                    let ecx = (r32(sh, o + 0x10) as i32).wrapping_sub(r32(sh, o + 0x11C) as i32);
                    sum_b = sum_b.wrapping_add(ecx.wrapping_sub(loss));
                    sum_a = sum_a.wrapping_add(ecx);
                }
            }
            if (sh[o + 0x136] as i8) < 0 {
                flag_b = true;
            }
            s = next;
            g += 1;
            if s >= count || g >= GUARD {
                break;
            }
        }
    }
    sum_a = if sum_a >= 0xEA60 { 0x7530 } else { sum_a >> 1 };
    flag_a |= sum_a > sum_b;
    let main = r16(w.remove.convoys, b + 0x10);
    let mo = ship(main);
    if flag_b {
        flag_a = false;
        if w.remove.ships[mo + 0x136] & 0x40 != 0 && sum_b < 0x7D0 {
            let mut s = r16(w.remove.convoys, b + 0xA);
            let mut g = 0;
            while s < count && g < GUARD {
                let o = ship(s);
                let next = r16(w.remove.ships, o + 6);
                w32(w.remove.ships, o + 0x18, 1);
                s = next;
                g += 1;
            }
        }
    }
    let mainx = r16(w.remove.ships, mo + 0x1E) as i16 as i32;
    let mainy = r16(w.remove.ships, mo + 0x22) as i16 as i32;
    let (mut tx, mut ty) = if r16(w.remove.ships, mo + 0x1E) == 0x3D2 {
        (0x220, 0x368)
    } else {
        (0x3D2, 0x317)
    };
    let anchor = |k: u8| {
        let s = env.slots.get(k as usize).copied().unwrap_or_default();
        (s.anchor_x, s.anchor_y)
    };
    let mut random_branch = false;
    let mut done = false;
    let limit = r16(w.remove.manager, 0xF8) as u32;
    if n > limit && n > 3 {
        let mut idx = (((w.remove.ships[mo + 0x42] & 7) as u32)
            .wrapping_add(env.clock >> 2)
            .wrapping_add((main & 1) as u32))
            & 0xF;
        for i in 0..16usize {
            if env.town_flags.get(i).copied().unwrap_or(0) & 0x40 != 0 {
                if idx == 0 {
                    let (x, y) = anchor(env.order.get(i).copied().unwrap_or(0));
                    tx = x;
                    ty = y;
                    break;
                }
                idx -= 1;
            }
        }
        if tx == mainx && ty == mainy {
            let (x, y) = anchor(free_port(env, env.time & 0x3F));
            tx = x;
            ty = y;
        }
        done = true;
    } else if flag_a {
        let obj = first_object(&env.objects, w.remove.ships[mo + 0x158]);
        match env.objects[obj as usize] {
            None => {
                let v = r16(w.remove.ships, mo + 0x138).wrapping_add(0x700);
                w16(w.remove.ships, mo + 0x138, v);
            }
            Some(kind) => {
                let s = env.slots.get(kind as usize).copied().unwrap_or_default();
                // El original compara la palabra de la tabla (sin signo) con la posición con signo.
                let (px, py) = (s.anchor_x & 0xFFFF, s.anchor_y & 0xFFFF);
                if px == mainx && py == mainy {
                    let next = r16(w.remove.convoys, b + 8);
                    match slot {
                        None => w16(w.remove.manager, 0xEC, next),
                        Some(p) => w16(w.remove.convoys, p * CONVOY_SIZE + 8, next),
                    }
                    w16(w.remove.convoys, b + 8, 0xFFFF);
                    host.remove_order(w, main);
                    host.absorb_convoy_with_world(w, u16::from(obj), cv);
                    return Ok(true);
                }
                tx = px;
                ty = py;
                let v = r16(w.remove.ships, mo + 0x138).wrapping_add(0x700);
                w16(w.remove.ships, mo + 0x138, v);
            }
        }
        done = true;
    } else {
        let target = r16(w.remove.ships, mo + 0x52);
        if target < count {
            let to = ship(target);
            if r16(w.remove.ships, to + 0x134) == 0xF {
                let (x, y) = (
                    r16(w.remove.ships, to + 0x1E) as i16,
                    r16(w.remove.ships, to + 0x22) as i16,
                );
                let found = find_town_at_point(&env.slots, &env.order, env.town_count, (x, y), None);
                if found == TOWN_NOT_FOUND {
                    let inside = RECTS
                        .iter()
                        .any(|&(x0, y0, x1, y1)| x >= x0 && x <= x1 && y >= y0 && y <= y1);
                    if inside {
                        random_branch = true;
                    } else {
                        tx = x as i32;
                        ty = y as i32;
                        done = true;
                    }
                } else {
                    random_branch = true;
                }
            } else {
                random_branch = true;
            }
        } else {
            random_branch = true;
        }
    }
    if random_branch && !done {
        let mut rng = Lcg(env.time);
        let val = w.remove.ships[mo + 0x159] as u32;
        let town = if env.town_count <= val {
            rng.next(env.town_count)?
        } else {
            val
        } as usize;
        let total = env.route_counts.get(town).copied().unwrap_or(0);
        if total != 0 {
            let mut k = rng.next(total)?;
            let mut g = 0;
            loop {
                let packed = env
                    .route_points
                    .get(town)
                    .and_then(|l| l.get(k as usize))
                    .copied()
                    .unwrap_or(0);
                let sx = sext16(packed as i32);
                let sy = sext16((packed >> 16) as i32);
                let x = div1000(sx.wrapping_mul(env.scale).wrapping_add(0x1F4)).wrapping_add(0x44);
                let y = div1000(sy.wrapping_mul(env.scale).wrapping_add(0x1F4)).wrapping_add(0x44);
                k += 1;
                tx = sext16(x);
                ty = sext16(y);
                if k >= total {
                    k = 0;
                }
                g += 1;
                if tx != mainx || ty != mainy || g >= GUARD {
                    break;
                }
            }
        }
    }
    host.set_destination(w, main, tx as u32, ty as u32);
    let main = r16(w.remove.convoys, b + 0x10);
    let second = r16(w.remove.convoys, b + 0xA);
    if w.remove.ships[ship(main) + 0x36] == 0 && main != second {
        w32(w.remove.ships, ship(second) + 0x18, 1);
    }
    Ok(false)
}
