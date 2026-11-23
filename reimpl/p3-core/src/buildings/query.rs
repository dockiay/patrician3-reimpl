//! Consultas de solo lectura sobre el plano urbano (C05): validez de los puntos del plano, límites de
//! capillas y hospitales y espacio libre dentro de las murallas.

use super::Site;
use crate::town::{off, Town};

/// `town_plan_is_plan_point_valid` (0x004FCFE0): el punto `k` (0..3) cae dentro de la rejilla.
pub fn is_plan_point_valid(t: &Town, k: u32) -> bool {
    if k > 3 {
        return false;
    }
    let o = 0x7B0 + 2 * k as usize;
    (t.u8(o) as i32) < t.i32(off::PLAN_ROWS) && (t.u8(o + 1) as i32) < t.i32(off::PLAN_STRIDE)
}

/// Cociente con signo entre 10.000 (la división por multiplicación mágica del original redondea a cero).
fn per_ten_thousand(v: i32) -> i32 {
    v / 10_000
}

/// `town_can_build_another_chapel` (0x00525CA0).
pub fn can_build_another_chapel(t: &Town) -> bool {
    if is_plan_point_valid(t, 1) {
        return false;
    }
    if is_plan_point_valid(t, 2) && t.u16(off::WALL_HEAD) < t.u16(off::BUILDING_RECORD_COUNT) {
        return false;
    }
    per_ten_thousand(t.i32(off::CITIZENS_TOTAL).wrapping_add(5000)) > t.u8(0x770) as i32
}

/// `town_can_build_another_hospital` (0x00525D10).
pub fn can_build_another_hospital(t: &Town) -> bool {
    if is_plan_point_valid(t, 1) {
        return false;
    }
    if is_plan_point_valid(t, 2) && t.u16(off::WALL_HEAD) < t.u16(off::BUILDING_RECORD_COUNT) {
        return false;
    }
    per_ten_thousand(t.i32(off::CITIZENS_TOTAL)) >= t.u8(0x772) as i32
}

impl Site {
    /// `town_plan_get_free_space_percent_inside_walls` (0x004FAC70). Una celda con el bit 7 y sin el 0x20
    /// cuenta según el tipo de la celda siguiente, que el original lee sin comprobar el final de la rejilla.
    pub fn free_space_percent_inside_walls(&self, t: &Town) -> u32 {
        let total = t.i32(off::PLAN_ROWS).wrapping_mul(t.i32(off::PLAN_STRIDE));
        let (mut inside, mut counted) = (0u32, 0u32);
        for i in 0..total.max(0) as usize {
            let c = self.cells[i];
            if c & 0x80 == 0 {
                continue;
            }
            if c & 0x20 != 0 {
                inside += 1;
                counted += 1;
            } else {
                let kind = self.cells[i + 1] & 0x1F;
                if kind == 0xB || (kind > 0x11 && kind <= 0x1E) {
                    counted += 1;
                }
            }
        }
        if total <= 0 || counted == 0 {
            return 100;
        }
        inside.wrapping_mul(100) / counted
    }
}

/// Bytes de 0x00672B3C indexados por tipo de edificio: distinto de cero si el edificio cuenta como posible
/// destino al elegir una posición al azar.
const PICKABLE: [u8; 256] = [
    0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 130, 30, 2, 40, 80, 50, 50, 80, 30, 30, 50, 30,
    80, 30, 100, 80, 80, 50, 10, 10, 30, 80, 80, 80, 80, 80, 80, 50, 50, 50, 100, 80, 80, 40, 80, 40, 40, 80, 80, 50,
    20, 70, 250, 150, 20, 50, 40, 100, 25, 40, 25, 40, 20, 20, 2, 4, 4, 0, 0, 0, 200, 250, 25, 200, 100, 100, 100, 200,
    100, 50, 200, 50, 200, 50, 50, 200, 200, 100, 25, 25, 50, 200, 200, 200, 100, 100, 100, 50, 50, 50, 100, 250, 250,
    200, 200, 200, 200, 150, 200, 100, 25, 200, 250, 200, 200, 5, 0, 50, 200, 0, 200, 0, 0, 0, 25, 50, 50, 0, 0, 0, 3,
    4, 1, 2, 11, 19, 12, 17, 3, 4, 80, 32, 9, 10, 130, 52, 11, 13, 30, 12, 12, 14, 40, 16, 28, 31, 120, 48, 36, 37, 60,
    24, 38, 39, 90, 36, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 6, 6, 6, 20, 9, 2, 3,
    12, 5, 0, 1, 11, 8, 4, 16, 14,
];

/// Enlaces de una oficina que recorre la elección de posición (`+0x2CA`, `+0x2CE`, `+0x2D0`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct OfficeHeads {
    pub next: u16,
    pub buildings_head: u16,
    pub houses_head: u16,
}

impl Site {
    /// `town_pick_random_building_position` (0x00525B90): devuelve (fila, columna) del edificio número
    /// `rnd % registros` (o, con `rnd == u32::MAX`, uno derivado de `Town+0x31C`) recorriendo la lista pública
    /// y después los edificios y las casas de cada oficina; (0xFF, 0xFF) si no hay tantos.
    pub fn pick_random_building_position(
        &self,
        t: &Town,
        offices: &[OfficeHeads],
        office_count: u16,
        rnd: u32,
    ) -> (u8, u8) {
        let count = t.u16(off::BUILDING_RECORD_COUNT);
        let mut left = if rnd < u32::MAX {
            rnd % count as u32
        } else if count < 0x80 {
            t.u32(0x31C) & 0x3F
        } else {
            t.u32(0x31C) & 0x7F
        };
        let (mut row, mut col) = (0xFFu8, 0xFFu8);
        let mut cur = t.u16(off::PUBLIC_BUILDING_HEAD);
        let mut office = t.u16(off::FIRST_OFFICE);
        let mut buildings = true;
        while left > 0 {
            if cur < count {
                let r = &self.records[cur as usize * 8..cur as usize * 8 + 8];
                if PICKABLE[r[3] as usize] != 0 {
                    row = r[0];
                    col = r[1];
                    left -= 1;
                }
                cur = u16::from_le_bytes([r[4], r[5]]);
            } else if office < office_count {
                let o = offices[office as usize];
                office = o.next;
                cur = if buildings { o.buildings_head } else { o.houses_head };
            } else if !buildings {
                break;
            } else {
                buildings = false;
                office = t.u16(off::FIRST_OFFICE);
            }
        }
        (row, col)
    }
}

impl Site {
    /// `town_plan_complete_street_cells_near_center` (0x004FB580): recorre anillos cada vez mayores
    /// alrededor del centro del plano (`Town+0x800`, `+0x801`) y convierte las celdas `0x82` y `0x83` en
    /// `0x81`, hasta 16. Con `notify` avisa al mapa visual de cada celda cambiada. Devuelve cuántas
    /// cambió. Cada anillo se recorre en cuatro tramos (columna izquierda hacia abajo, fila inferior
    /// de dos en dos hacia la derecha, columna derecha hacia arriba y fila superior hacia la izquierda).
    pub fn complete_street_cells_near_center(
        &mut self,
        t: &Town,
        notify: bool,
        host: &mut dyn super::housing::HousingHost,
    ) -> u32 {
        let stride = t.i32(off::PLAN_STRIDE);
        let rows = t.i32(off::PLAN_ROWS);
        let row0 = t.u8(0x800) as i32;
        let col0 = t.u8(0x801) as i32;
        let at =
            |r: i32, c: i32| usize::try_from(stride as i64 * r as i64 + c as i64).expect("celda fuera de la rejilla");
        let mut count = 0u32;
        let first = at(row0, col0);
        if self.cells[first] == 0x82 || self.cells[first] == 0x83 {
            self.cells[first] = 0x81;
            count = 1;
        }
        let col_m1 = col0 - 1;
        let mut base = col_m1;
        let mut r = 1i32;
        loop {
            let mut ebx = base;
            let mut edi = row0 - ((col_m1 & r) & 1) - (r >> 1);
            let mut width = r;
            let mut height = r;
            let bottom_in = edi + r < rows;
            if !bottom_in {
                height = rows - edi - 1;
            }
            let top_in = edi >= 0;
            if !top_in {
                height += edi;
                edi = 0;
            }
            let right_in = ebx + r * 2 < stride;
            if !right_in {
                width = (stride - ebx - 1) >> 1;
            }
            let left_in = ebx >= 0;
            if !left_in {
                let e = ebx - 1;
                width += (e - (e >> 31)) >> 1;
                ebx &= 1;
            }
            let mut esi = at(edi, ebx) as i64;
            let hit = |s: &mut Self,
                       esi: i64,
                       row: i32,
                       col: i32,
                       count: &mut u32,
                       host: &mut dyn super::housing::HousingHost|
             -> bool {
                let c = s.cells[esi as usize];
                if c == 0x82 || c == 0x83 {
                    s.cells[esi as usize] = 0x81;
                    if notify {
                        host.add_map_object(
                            row as u32,
                            col as u32,
                            0x17,
                            row.wrapping_add(row.wrapping_add(1).wrapping_mul(col)) as u32,
                            0xFF,
                        );
                    }
                    *count += 1;
                    if *count >= 0x10 {
                        return true;
                    }
                }
                false
            };
            // columna izquierda, hacia abajo
            if left_in {
                let mut left = height;
                while left > 0 {
                    left -= 1;
                    if hit(self, esi, edi, ebx, &mut count, host) {
                        return count;
                    }
                    edi += 1;
                    esi += stride as i64;
                }
            } else {
                edi += height;
                esi = at(edi, ebx) as i64;
            }
            // fila inferior, hacia la derecha de dos en dos
            if bottom_in {
                let mut left = width;
                while left > 0 {
                    left -= 1;
                    if hit(self, esi, edi, ebx, &mut count, host) {
                        return count;
                    }
                    ebx += 2;
                    esi += 2;
                }
            } else {
                ebx += width * 2;
                esi = at(edi, ebx) as i64;
            }
            // columna derecha, hacia arriba
            if right_in {
                let mut left = height;
                while left > 0 {
                    left -= 1;
                    if hit(self, esi, edi, ebx, &mut count, host) {
                        return count;
                    }
                    edi -= 1;
                    esi -= stride as i64;
                }
            } else {
                edi -= height;
                esi = at(edi, ebx) as i64;
            }
            // fila superior, hacia la izquierda de dos en dos
            if top_in {
                let mut left = width;
                while left > 0 {
                    left -= 1;
                    if hit(self, esi, edi, ebx, &mut count, host) {
                        return count;
                    }
                    ebx -= 2;
                    esi -= 2;
                }
            }
            base -= 1;
            r += 1;
            if !(top_in || bottom_in || left_in || right_in) {
                return count;
            }
        }
    }
}

impl Site {
    /// `town_list_business_buildings_outside_walls` (0x00525220): índices de los edificios de las listas de
    /// cada oficina (desde la primera de la ciudad) cuya celda no tiene el bit `0x80` (calle). El original
    /// devuelve un bloque de `u16` cuyo primer elemento es el recuento.
    pub fn list_business_buildings_outside_walls(
        &self,
        t: &Town,
        offices: &[OfficeHeads],
        office_count: u16,
    ) -> Vec<u16> {
        let count = t.u16(off::BUILDING_RECORD_COUNT);
        let stride = t.i32(off::PLAN_STRIDE) as i64;
        let mut out = Vec::new();
        let mut office = t.u16(off::FIRST_OFFICE);
        if office >= office_count {
            return out;
        }
        loop {
            let o = offices[office as usize];
            let mut at = o.buildings_head;
            while at < count {
                let r = &self.records[at as usize * 8..at as usize * 8 + 8];
                let cell =
                    self.cells[usize::try_from(r[0] as i64 * stride + r[1] as i64).expect("celda fuera de la rejilla")];
                if cell & 0x80 == 0 {
                    out.push(at);
                }
                at = u16::from_le_bytes([r[4], r[5]]);
            }
            office = o.next;
            if office >= office_count {
                return out;
            }
        }
    }
}

/// Ficha de un edificio en un punto (`0x28` bytes que `town_get_building_info_at` rellena).
pub type BuildingInfo = [u8; 0x28];

/// Consulta a la oficina.
pub trait InfoHost {
    /// `office_get_building_info_at` (0x005008C0) de la oficina `office`: rellena la ficha y devuelve si
    /// el punto es de un edificio suyo.
    fn office_building_info(&mut self, office: u16, info: &mut BuildingInfo, row: u32, col: u32) -> bool;
}

impl Site {
    /// `town_get_building_info_at` (0x00524780): ficha del edificio que ocupa (`row`, `col`). Prueba las
    /// oficinas de la ciudad una a una y, si ninguna lo reconoce, busca en la lista pública y cuenta los
    /// edificios públicos de su tipo. Devuelve si lo encontró.
    pub fn building_info_at(
        &self,
        t: &Town,
        info: &mut BuildingInfo,
        row: u32,
        col: u32,
        offices: &[OfficeHeads],
        office_count: u16,
        host: &mut dyn InfoHost,
    ) -> bool {
        *info = [0; 0x28];
        info[6] = 0xFF;
        info[7] = 0xFF;
        info[0x18] = 0xFF;
        info[0x19] = 0xFF;
        info[0x22] = 0x1B;
        info[0x23] = 0xFF;
        let mut office = t.u16(off::FIRST_OFFICE);
        while office < office_count {
            let next = offices[office as usize].next;
            if host.office_building_info(office, info, row, col) {
                if t.u32(off::TOWN_FLAGS) & 0x10 != 0 && info[0x22] != 5 {
                    let stride = t.i32(off::PLAN_STRIDE) as i64;
                    let cell = self.cells
                        [usize::try_from(stride * row as i64 + col as i64).expect("celda fuera de la rejilla")];
                    if cell < 0x80 {
                        info[5] = 0;
                    }
                }
                return true;
            }
            office = next;
        }
        let count = t.u16(off::BUILDING_RECORD_COUNT);
        let mut counts = [0u16; 21];
        let mut found = false;
        let mut at = t.u16(off::PUBLIC_BUILDING_HEAD);
        while at < count {
            let r = &self.records[at as usize * 8..at as usize * 8 + 8];
            at = u16::from_le_bytes([r[4], r[5]]);
            if row == r[0] as u32 && col == r[1] as u32 {
                found = true;
                info[0x22] = r[3];
            }
            if r[3] < 0x15 {
                counts[r[3] as usize] = counts[r[3] as usize].wrapping_add(1);
            }
        }
        let ty = info[0x22];
        if ty < 0x15 {
            info[6] = 0xFF;
            info[7] = 0xFF;
            info[8..0x10].fill(0);
            info[0x18] = 0xFF;
            info[0x19] = 0xFF;
            info[0x1A] = 0;
            info[0x1B] = 0;
            info[0x20..0x22].copy_from_slice(&counts[ty as usize].to_le_bytes());
        }
        found
    }
}

/// Celda de pozo que busca el plano (`0x84`).
const WELL_CELL: u8 = 0x84;

impl Site {
    /// `town_plan_find_nearest_well_site` (0x004FA7F0): recorre anillos crecientes alrededor de `(row, col)`
    /// hasta una celda de pozo que tenga otra a cada lado, según la paridad de la celda (rejilla escalonada).
    /// Si encuentra una, la escribe en `row` y `col`; si agota los cuatro lados, no las toca. El original lee
    /// los vecinos sin comprobar el borde de la rejilla; aquí eso es un `panic`.
    pub fn find_nearest_well_site(&self, rows: i32, stride: i32, row: &mut u8, col: &mut u8) {
        let cell = |i: i32| self.cells[usize::try_from(i).expect("celda fuera de la rejilla")];
        let is_site = |i: i32, odd: bool| {
            let next = cell(i + 1) == WELL_CELL;
            let (a, b) = if odd {
                (
                    next && cell(i + stride - 1) == WELL_CELL,
                    cell(i - 1) == WELL_CELL && cell(i + stride + 1) == WELL_CELL,
                )
            } else {
                (
                    next && cell(i - stride - 1) == WELL_CELL,
                    cell(i - 1) == WELL_CELL && cell(i - stride + 1) == WELL_CELL,
                )
            };
            a || b
        };
        let (x, y) = (*row as i32, *col as i32);
        let mut r = 1i32;
        loop {
            let (mut len_row, mut len_col) = (r, r);
            let mut v14 = y - r;
            let mut ebx = x - (((y - 1) & r) & 1) - (r >> 1);
            let mut v18 = ebx;
            let mut edx = rows;
            let down;
            if ebx + r < edx {
                edx = len_row;
                down = true;
            } else {
                edx = rows - ebx - 1;
                down = false;
                len_row = edx;
            }
            let up;
            if ebx < 0 {
                edx += ebx;
                ebx = 0;
                up = false;
                len_row = edx;
                v18 = ebx;
            } else {
                up = true;
            }
            let mut ebp;
            let right;
            if (y - r) + 2 * r < stride {
                ebp = len_col;
                right = true;
            } else {
                ebp = (stride - (y - r) - 1) >> 1;
                right = false;
                len_col = ebp;
            }
            let left;
            if y - r < 0 {
                let e = v14 - 1;
                ebp += (e - (e >> 31)) >> 1;
                left = false;
                len_col = ebp;
                v14 &= 1;
            } else {
                left = true;
            }
            let mut idx = stride.wrapping_mul(ebx).wrapping_add(v14);
            let found = |v18: i32, v14: i32, row: &mut u8, col: &mut u8| {
                *row = v18 as u8;
                *col = v14 as u8;
            };
            // lado 1: hacia abajo
            if !left {
                ebx += len_row;
                idx = stride.wrapping_mul(ebx).wrapping_add(v14);
                v18 = ebx;
            } else if len_row > 0 {
                let mut n = len_row;
                loop {
                    let hit = cell(idx) == WELL_CELL;
                    n -= 1;
                    if hit && is_site(idx, idx & 1 != 0) {
                        found(v18, v14, row, col);
                        return;
                    }
                    idx = idx.wrapping_add(stride);
                    v18 += 1;
                    if n <= 0 {
                        break;
                    }
                }
                ebx = v18;
            }
            // lado 2: hacia la derecha
            if down {
                if len_col > 0 {
                    let mut n = len_col;
                    loop {
                        let hit = cell(idx) == WELL_CELL;
                        n -= 1;
                        if hit && is_site(idx, idx & 1 != 0) {
                            found(v18, v14, row, col);
                            return;
                        }
                        idx = idx.wrapping_add(2);
                        v14 += 2;
                        if n <= 0 {
                            break;
                        }
                    }
                    ebx = v18;
                }
            } else {
                v14 += len_col * 2;
                idx = stride.wrapping_mul(ebx).wrapping_add(v14);
            }
            // lado 3: hacia arriba
            if right {
                if len_row > 0 {
                    let mut n = len_row;
                    loop {
                        let hit = cell(idx) == WELL_CELL;
                        n -= 1;
                        if hit && is_site(idx, v14 & 1 != 0) {
                            found(v18, v14, row, col);
                            return;
                        }
                        idx = idx.wrapping_sub(stride);
                        v18 -= 1;
                        if n <= 0 {
                            break;
                        }
                    }
                }
            } else {
                ebx -= len_row;
                idx = stride.wrapping_mul(ebx).wrapping_add(v14);
                v18 = ebx;
            }
            // lado 4: hacia la izquierda
            if up && len_col > 0 {
                let mut n = len_col;
                loop {
                    let hit = cell(idx) == WELL_CELL;
                    n -= 1;
                    if hit && is_site(idx, v14 & 1 != 0) {
                        found(v18, v14, row, col);
                        return;
                    }
                    idx = idx.wrapping_sub(2);
                    v14 -= 2;
                    if n <= 0 {
                        break;
                    }
                }
            }
            r += 1;
            if !(up || down || left || right) {
                return;
            }
        }
    }

    /// `town_pick_new_well_site` (0x005250D0): parte de `(Town+0x800, Town+0x801)` y, según `Town+0x789`,
    /// se desplaza a un lado del plano (con vuelta por módulo) antes de buscar el pozo más cercano.
    /// Devuelve la fila y la columna resultantes.
    pub fn pick_new_well_site(&self, t: &Town) -> (u8, u8) {
        let b = t.u8(0x789);
        let a = if b < 1 { 1 } else { b as i32 - 1 };
        let (mut row, mut col) = (t.u8(0x800), t.u8(0x801));
        let (d_row, d_col) = if a & 4 != 0 {
            (1 - 2 * (a & 1), 1 - (a & 2))
        } else if a & 2 == 0 {
            (0, 1 - 2 * (a & 1))
        } else {
            (1 - 2 * (a & 1), 0)
        };
        let step = (a >> 3) * 8 + 0x19;
        let (rows, stride) = (t.i32(off::PLAN_ROWS), t.i32(off::PLAN_STRIDE));
        let wrap = |cur: u8, delta: i32, limit: i32| -> u8 {
            let c = cur as i32;
            if 2 - delta > c {
                ((delta & 0xFF) % (limit - 4) + 2) as u8
            } else if limit >= c + delta + 2 {
                cur.wrapping_add(delta as u8)
            } else {
                let p = ((delta as u8 as i8 as i16) * 3) as u8;
                ((p.wrapping_add(cur)) as i32 % (limit - 4) + 2) as u8
            }
        };
        row = wrap(row, d_row * step, rows);
        col = wrap(col, d_col * step, stride);
        self.find_nearest_well_site(rows, stride, &mut row, &mut col);
        (row, col)
    }
}
