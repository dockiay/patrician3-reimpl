//! Recuento de edificios y reparto de viviendas (C05, parte C11).
//!
//! `town_recount_buildings` (0x00520BD0) cuenta los edificios de las listas de construcción y pública,
//! suma la capacidad de vivienda de las casas de cada oficina y reparte entre ellas la población de los
//! tres grupos (ricos, acomodados, pobres). `town_assign_house_levels_from_population` (0x00520840)
//! sube o baja el nivel de cada casa de una lista según cuánta población hay en su grupo; las casas en
//! obra vuelven al primer nivel de su grupo.

use super::tables::MAP_OBJECT_PARAM;
use super::Site;
use crate::town::{off, Town};

/// Capacidad de una casa de cada grupo (`0x00672A18`).
const HOUSE_CAPACITY: [u16; 3] = [80, 140, 280];
/// Umbrales de ocupación para el nivel 0, 1 y 2 de cada grupo (`0x00672A20`).
const LEVEL_LIMIT: [[u16; 3]; 3] = [[20, 50, 80], [35, 100, 140], [70, 200, 280]];
/// Primer tipo de edificio de cada grupo de casas.
const BASE_TYPE: [u8; 4] = [0x15, 0x18, 0x1B, 0x1E];

/// Campos de una oficina que lee o escribe el recuento.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct OfficeHousing {
    /// `+0x2CA`: siguiente oficina.
    pub next: u16,
    /// `+0x2D0`: primera casa de la oficina.
    pub houses_head: u16,
    /// `+0x2D4`: número de casas.
    pub house_count: u16,
    /// `+0x2D8`: capacidad de vivienda por grupo.
    pub capacity: [u16; 3],
    /// `+0x2DE`: habitantes asignados por grupo.
    pub assigned: [u16; 3],
    /// `+0x2E4`: nivel de casa que ocupan los de cada grupo (0–4).
    pub level: [u8; 3],
}

/// Avisos al mapa visual.
pub trait HousingHost {
    /// `class22_add_map_object` (0x00590680): (fila, columna, parámetro, nivel y auxiliar, propietario).
    fn add_map_object(&mut self, row: u32, col: u32, param: u32, packed: u32, owner: u32);
}

/// Entorno del recuento.
pub struct HousingCtx<'a> {
    pub offices: &'a mut [OfficeHousing],
    /// Número de oficinas (`0x006DE4A8`).
    pub office_count: u16,
    /// Propietario (`+0xC324`) del objeto `0x006E51AC`, si existe.
    pub visual_owner: Option<u32>,
    pub host: &'a mut dyn HousingHost,
}

fn div3_signed(x: i32) -> i32 {
    x / 3
}

fn imul(a: i32, b: i32) -> i32 {
    a.wrapping_mul(b)
}

impl Site {
    /// `town_assign_house_levels_from_population` (0x00520840). `people` son los habitantes de cada
    /// grupo en las casas de la lista que empieza en `head`.
    pub fn assign_house_levels(
        &mut self,
        town: u8,
        count: usize,
        people: &mut [u16; 3],
        head: u16,
        cx: &mut HousingCtx,
    ) {
        let visible = cx.visual_owner == Some(town as u32);
        let mut houses = [0u32; 3];
        let mut i = head as usize;
        while i < count {
            if self.rec(i, 6) == 0 {
                let g = (self.rec(i, 3) as u32).wrapping_sub(0x15) / 3;
                if g < 3 {
                    houses[g as usize] += 1;
                }
            }
            i = self.next(i) as usize;
        }
        // Reparto lineal (`ra`) y resto (`rb`) de la ocupación por casa.
        let mut ra = [0f32; 3];
        let mut rb = [0f32; 3];
        for g in 0..3 {
            let n = houses[g];
            if n == 0 {
                people[g] = 0;
                ra[g] = 0.0;
                rb[g] = 0.0;
            } else {
                let per = HOUSE_CAPACITY[g] as u32;
                let cap = n.wrapping_mul(per);
                let cur = people[g] as u32;
                if cur >= cap {
                    rb[g] = per as f32;
                    ra[g] = 0.0;
                } else {
                    let f_cur = cur as f32;
                    let c = n as f32;
                    let tri = ((c as f64 + 1.0) * c as f64 * 0.5) as f32;
                    let per_f = per as f32;
                    let half = (per_f as f64 * c as f64 * 0.5) as f32;
                    let value: f32;
                    if half as f64 > f_cur as f64 {
                        // Menos de la mitad de la capacidad: reparto lineal entre las casas.
                        value = (f_cur as f64 / tri as f64) as f32;
                        rb[g] = 0.0;
                    } else if half as f64 + half as f64 <= f_cur as f64 {
                        // Capacidad completa (solo por redondeo de f32: `cur < cap`).
                        ra[g] = 0.0;
                        rb[g] = per_f;
                        continue;
                    } else {
                        let x = ((2.0 * f_cur as f64 - cap as f32 as f64) * per_f as f64 / f_cur as f64) as f32;
                        value = ((f_cur as f64 - x as f64 * c as f64) / tri as f64) as f32;
                        rb[g] = x;
                    }
                    ra[g] = if value < 0.0 { 0.0 } else { value };
                }
            }
        }
        let mut level = [0usize; 3];
        let mut i = head as usize;
        while i < count {
            let ty = self.rec(i, 3);
            if self.rec(i, 6) != 0 {
                if (0x15..0x1E).contains(&ty) {
                    let q = div3_signed(ty as i32 - 0x15);
                    self.set_rec(i, 3, ((q + 7) * 3) as u8);
                }
            } else {
                let g = (ty as u32).wrapping_sub(0x15) / 3;
                if g < 3 {
                    let g = g as usize;
                    let ratio = (ra[g] as f64 + rb[g] as f64) as f32;
                    rb[g] = ratio;
                    while ratio as f64 > LEVEL_LIMIT[g][level[g]] as f32 as f64 && level[g] < 2 {
                        level[g] += 1;
                    }
                    let new = BASE_TYPE[g].wrapping_add(level[g] as u8);
                    if visible {
                        if new != ty {
                            self.set_rec(i, 3, new);
                            let param = MAP_OBJECT_PARAM[new as usize] as i32 as u32;
                            let packed = (self.rec(i, 7) as u32) << 16 | level[g] as u32;
                            cx.host.add_map_object(
                                self.rec(i, 0) as u32,
                                self.rec(i, 1) as u32,
                                param,
                                packed,
                                self.rec(i, 2) as u32,
                            );
                        }
                    } else {
                        self.set_rec(i, 3, new);
                    }
                }
            }
            i = self.next(i) as usize;
        }
    }

    /// `town_recount_buildings` (0x00520BD0).
    pub fn recount_buildings(&mut self, t: &mut Town, cx: &mut HousingCtx) {
        let count = Self::record_count(t);
        let town = t.u8(off::TOWN_INDEX);
        // Matriz [grupo][nivel de casa] de casas con hueco, objetivo por celda, y población por repartir.
        let mut mat = [0i32; 15];
        let mut target = [0i32; 15];
        let mut surplus = [0i32; 3];
        let mut total = [0i32; 3];
        t.0[0x77E..0x784].fill(0);
        for o in [0x770, 0x771, 0x772, 0x773, 0x789, 0x788, 0x78A] {
            t.set_u8(o, 0);
        }
        t.set_u16(0x2FE, 0);
        // Edificios en obra.
        let mut i = t.u16(off::CONSTRUCTION_HEAD) as usize;
        while i < count {
            match self.rec(i, 3) {
                0x28 => t.set_u8(0x789, t.u8(0x789).wrapping_add(1)),
                0x29 => t.set_u8(0x772, t.u8(0x772).wrapping_add(1)),
                0x2C => t.set_u8(0x770, t.u8(0x770).wrapping_add(1)),
                _ => {}
            }
            i = self.next(i) as usize;
        }
        // Edificios públicos terminados.
        let mut i = t.u16(off::PUBLIC_BUILDING_HEAD) as usize;
        while i < count {
            let s = self.rec(i, 3) as i32 - 0x15;
            if (0..=0x17).contains(&s) {
                match s {
                    0..=2 => t.add_u16(0x77E, HOUSE_CAPACITY[0]),
                    3..=5 => t.add_u16(0x780, HOUSE_CAPACITY[1]),
                    6..=8 => t.add_u16(0x782, HOUSE_CAPACITY[2]),
                    19 => t.set_u8(0x789, t.u8(0x789).wrapping_add(1)),
                    20 => {
                        t.set_u8(0x773, t.u8(0x773).wrapping_add(1));
                        t.set_u8(0x772, t.u8(0x772).wrapping_add(1));
                    }
                    23 => {
                        t.set_u8(0x771, t.u8(0x771).wrapping_add(1));
                        t.set_u8(0x770, t.u8(0x770).wrapping_add(1));
                    }
                    _ => {}
                }
            }
            i = self.next(i) as usize;
        }
        // Capacidad nueva frente a la que había (`0x778`), y viviendas por grupo.
        for g in 0..3 {
            let new = t.u16(0x77E + 2 * g);
            let old = t.u16(0x778 + 2 * g);
            if new != 0 {
                if new <= old {
                    t.set_u16(0x778 + 2 * g, new);
                } else if old > 4 {
                    t.set_u16(0x778 + 2 * g, old - 4);
                    mat[5 * g + 2] = new as i32 - (old - 4) as i32;
                } else {
                    mat[5 * g + 2] = new as i32;
                    t.set_u16(0x778 + 2 * g, 0);
                }
            } else {
                t.set_u16(0x778 + 2 * g, 0);
            }
            t.set_u16(off::HOUSING + 2 * g, new);
            total[g] = t.u16(0x778 + 2 * g) as i32;
        }
        // Casas de cada oficina.
        let offices = cx.office_count as usize;
        let mut o = t.u16(off::FIRST_OFFICE) as usize;
        while o < offices {
            let next = cx.offices[o].next as usize;
            let mut house = cx.offices[o].houses_head as usize;
            cx.offices[o].capacity = [0; 3];
            cx.offices[o].house_count = 0;
            while house < count {
                let g = div3_signed(self.rec(house, 3) as i32 - 0x15);
                if (0..3).contains(&g) {
                    let g = g as usize;
                    let c = &mut cx.offices[o];
                    c.capacity[g] = c.capacity[g].wrapping_add(HOUSE_CAPACITY[g]);
                    c.house_count = c.house_count.wrapping_add(1);
                }
                house = self.next(house) as usize;
            }
            for g in 0..3 {
                let c = &mut cx.offices[o];
                let lvl = c.level[g] as usize;
                c.assigned[g] = if (c.level[g] as u16) < c.assigned[g] {
                    c.assigned[g] - c.level[g] as u16
                } else {
                    0
                };
                if c.assigned[g] >= c.capacity[g] {
                    c.assigned[g] = c.capacity[g];
                } else {
                    mat[5 * g + lvl] += (c.capacity[g] - c.assigned[g]) as i32;
                }
                total[g] += c.assigned[g] as i32;
                t.add_u16(off::HOUSING + 2 * g, c.capacity[g]);
            }
            o = next;
        }
        // Población por repartir en cada grupo y objetivo por nivel.
        for g in 0..3 {
            surplus[g] = t.i32(off::POPULATION + 4 * g).wrapping_sub(total[g]);
            if surplus[g] < 0 {
                target[5 * g..5 * g + 5].fill(surplus[g]);
                continue;
            }
            let mut weight = 0i32;
            for j in 0..5 {
                weight = weight.wrapping_add(imul(mat[5 * g + j], 5 - j as i32));
            }
            if weight == 0 {
                target[5 * g..5 * g + 5].fill(1);
            } else {
                for j in 0..5 {
                    let v = imul(imul(mat[5 * g + j], 5 - j as i32), surplus[g]);
                    target[5 * g + j] = v.checked_div(weight).expect("división por cero en el original");
                }
            }
        }
        // Reparto entre las oficinas.
        let mut o = t.u16(off::FIRST_OFFICE) as usize;
        while o < offices {
            let next = cx.offices[o].next as usize;
            for g in 0..3 {
                let c = &mut cx.offices[o];
                let j = 5 * g + c.level[g] as usize;
                let want = target[j];
                if want < 0 {
                    let have = c.assigned[g] as i32;
                    if have > 0 {
                        let t_g = total[g];
                        let q = (t_g.wrapping_sub(imul(have, want)).wrapping_sub(1))
                            .checked_div(t_g)
                            .expect("división por cero en el original");
                        let owed = surplus[g].wrapping_neg();
                        if owed > q {
                            surplus[g] += q;
                            c.assigned[g] = (have - q) as u16;
                        } else {
                            surplus[g] = 0;
                            c.assigned[g] = (have - owed) as u16;
                        }
                    }
                } else if want > 0 {
                    let room = c.capacity[g] as i32 - c.assigned[g] as i32;
                    if room > 0 {
                        let mut q = imul(want, room)
                            .checked_div(mat[j])
                            .expect("división por cero en el original");
                        if q >= room {
                            q = room;
                        }
                        if q > surplus[g] {
                            c.assigned[g] = (surplus[g] as u16).wrapping_add(c.assigned[g]);
                            surplus[g] = 0;
                        } else {
                            c.assigned[g] = c.assigned[g].wrapping_add(q as u16);
                            surplus[g] -= q;
                        }
                    }
                }
            }
            let mut assigned = cx.offices[o].assigned;
            let head = cx.offices[o].houses_head;
            self.assign_house_levels(town, count, &mut assigned, head, cx);
            cx.offices[o].assigned = assigned;
            o = next;
        }
        // Lo que sobra o falta se reparte entre las casas públicas.
        for g in 0..3 {
            let a = t.u16(0x77E + 2 * g) as i32;
            let b = t.u16(0x778 + 2 * g) as i32;
            if surplus[g] > 0 {
                let room = a - b;
                if room < surplus[g] {
                    surplus[g] -= room;
                    t.set_u16(0x778 + 2 * g, a as u16);
                } else {
                    t.set_u16(0x778 + 2 * g, (surplus[g] as u16).wrapping_add(b as u16));
                }
            } else if surplus[g] < 0 {
                let owed = -surplus[g];
                if owed > b {
                    surplus[g] += b;
                    t.set_u16(0x778 + 2 * g, 0);
                } else {
                    t.set_u16(0x778 + 2 * g, (surplus[g] as u16).wrapping_add(b as u16));
                }
            }
        }
        let mut people = [t.u16(0x778), t.u16(0x77A), t.u16(0x77C)];
        let head = t.u16(off::PUBLIC_BUILDING_HEAD);
        self.assign_house_levels(town, count, &mut people, head, cx);
        for g in 0..3 {
            t.set_u16(0x778 + 2 * g, people[g]);
        }
    }
}
