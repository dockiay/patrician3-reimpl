//! Recuento de edificios y negocios de oficina (`0x004FF2D0`).
//! Los efectos del gestor de negocios se inyectan como puerto para conservar el
//! orden de las llamadas; el integrador debe implementar reserva y liberación.
use crate::{buildings::Site, town::Town};

pub trait RecountFacilities {
    fn capacity(&self) -> usize;
    fn facility(&self, index: u16) -> Result<[u8; 20], String>;
    fn resize(&mut self, index: u16, town: &Town, count: i16) -> Result<(), String>;
    fn set_next(&mut self, index: u16, next: u16) -> Result<(), String>;
    /// Devuelve los trabajadores antes de desenlazar la instalación.
    fn release_workers(&mut self, index: u16, town: &mut Town) -> Result<(), String>;
    /// Devuelve la ranura al pool tras desenlazarla.
    fn release_slot(&mut self, index: u16, town: &mut Town) -> Result<(), String>;
    /// Reserva e inicializa una instalación; la llamada siguiente le asigna el enlace.
    fn create(&mut self, kind: u8, city: u8, count: i16, town: &Town, next: u16) -> Result<u16, String>;
}
fn get16(a: &[u8], at: usize) -> u16 {
    u16::from_le_bytes([a[at], a[at + 1]])
}
fn put16(a: &mut [u8], at: usize, v: u16) {
    a[at..at + 2].copy_from_slice(&v.to_le_bytes())
}
fn add16(a: &mut [u8], at: usize, v: u16) {
    let n = get16(a, at).wrapping_add(v);
    put16(a, at, n)
}
fn adjacent(row: u8, col: u8) -> [(u8, u8); 20] {
    let mut p = [(0, 0); 20];
    p[0] = (row.wrapping_add(1), col.wrapping_add(6));
    p[1] = (row.wrapping_add(col & 1).wrapping_sub(2), col.wrapping_add(7));
    p[2] = (row.wrapping_sub(3), col.wrapping_sub(2));
    p[3] = (row.wrapping_add(col & 1).wrapping_add(3), col.wrapping_sub(3));
    for a in 4..20 {
        let (mut r, mut c) = p[a - 4];
        if a % 2 == 0 {
            let odd = c & 1;
            c = c.wrapping_sub(1);
            if odd == 1 {
                r = r.wrapping_add(1)
            }
        } else {
            c = c.wrapping_sub(1);
            if c & 1 == 1 {
                r = r.wrapping_sub(1)
            }
        }
        p[a] = (r, c);
    }
    p
}
/// Recuenta las listas urbanas, agrupa tipos y sincroniza la lista de negocios.
/// La reserva y la devolución de trabajadores deben ser realizadas por `host`.
pub fn recount(
    office: &mut [u8; 0x44c],
    town: &mut Town,
    site: &mut Site,
    host: &mut dyn RecountFacilities,
) -> Result<(), String> {
    let n = usize::from(town.u16(0x776));
    if site.records.len() != n * 8 {
        return Err("registros de ciudad truncados".into());
    }
    let owner = get16(office, 0x2c4);
    let city = office[0x2c6];
    let mut count = [0u16; 21];
    put16(office, 0x2d2, 0);
    if get16(office, 0x2f0) > 0 {
        put16(office, 0x2ec, 0);
        put16(office, 0x2ee, 50)
    } else {
        put16(office, 0x2ec, 50);
        put16(office, 0x2ee, 0)
    }
    let mut seen = vec![false; n];
    let mut i = usize::from(town.u16(0x762));
    while i < n {
        if seen[i] {
            return Err("ciclo en obras".into());
        }
        seen[i] = true;
        if u16::from(site.rec(i, 2)) == owner && site.rec(i, 3) == 30 {
            add16(office, 0x2ee, 200)
        }
        i = usize::from(site.next(i));
    }
    seen.fill(false);
    let mut i = usize::from(get16(office, 0x2ce));
    let mut building = Vec::new();
    while i < n {
        if seen[i] {
            return Err("ciclo en edificios de oficina".into());
        }
        seen[i] = true;
        building.push(i);
        let ty = site.rec(i, 3);
        if ty < 21 {
            count[usize::from(ty)] = count[usize::from(ty)].wrapping_add(1)
        } else if ty == 30 {
            site.set_rec(i, 6, 0);
            add16(office, 0x2ec, 200)
        }
        add16(office, 0x2d2, 1);
        i = usize::from(site.next(i));
    }
    for (pos, &i) in building.iter().enumerate() {
        let ty = site.rec(i, 3);
        if (0x118710u32 & (1u32 << (ty & 31))) == 0 || ty >= 21 {
            continue;
        }
        // 0x004FF443: la marca 0xFF se escribe antes de mirar el recuento del tipo.
        site.set_rec(i, 7, 0xff);
        if (count[usize::from(ty)] as i16) <= 0 {
            continue;
        }
        let points = adjacent(site.rec(i, 0), site.rec(i, 1));
        for &j in &building[pos + 1..] {
            if site.rec(j, 3) == ty && points.contains(&(site.rec(j, 0), site.rec(j, 1))) {
                // 0x004FF525: el original guarda el índice del sucesor inmediato de `i`
                // (EBX no avanza en el bucle interior), no el del vecino encontrado.
                site.set_rec(i, 7, (building[pos + 1] as u8) & 0x7f);
                break;
            }
        }
    }
    put16(office, 0x2d4, 0);
    for k in 0..3 {
        put16(office, 0x2d8 + 2 * k, 0)
    }
    seen.fill(false);
    let mut i = usize::from(get16(office, 0x2d0));
    while i < n {
        if seen[i] {
            return Err("ciclo en casas de oficina".into());
        }
        seen[i] = true;
        site.set_rec(i, 2, owner as u8);
        if site.rec(i, 6) == 0 {
            add16(office, 0x2d4, 1)
        }
        let g = (i32::from(site.rec(i, 3)) - 21) / 3;
        if !(0..3).contains(&g) {
            return Err("tipo de casa fuera de tabla".into());
        }
        let at = 0x2d8 + 2 * g as usize;
        let cap = [80u16, 140, 280][g as usize];
        add16(office, at, cap);
        i = usize::from(site.next(i));
    }
    let mut previous = None;
    let mut next = get16(office, 0x2cc);
    let mut visited = vec![false; host.capacity()];
    while usize::from(next) < host.capacity() {
        let i = next;
        if visited[usize::from(i)] {
            return Err("ciclo en negocios".into());
        }
        visited[usize::from(i)] = true;
        let f = host.facility(i)?;
        let ty = usize::from(f[6]);
        if ty >= 21 {
            return Err("tipo de negocio fuera de tabla".into());
        }
        let number = count[ty] as i16;
        if number > 0 {
            host.resize(i, town, number)?;
            count[ty] = 0xffff;
            previous = Some(i);
            next = get16(&f, 8)
        } else {
            host.release_workers(i, town)?;
            next = get16(&host.facility(i)?, 8);
            if let Some(p) = previous {
                host.set_next(p, next)?
            } else {
                put16(office, 0x2cc, next);
            }
            host.set_next(i, 0xffff)?;
            host.release_slot(i, town)?;
        }
    }
    for ty in (1..21).rev() {
        let number = count[ty] as i16;
        if number > 0 {
            let i = host.create(ty as u8, city, number, town, get16(office, 0x2cc))?;
            put16(office, 0x2cc, i);
        }
    }
    Ok(())
}

/// Gestor nativo del recuento para instalaciones de la ciudad actual.
pub struct NativeFacilities<'a> {
    pub pool: &'a mut super::facility_pool::FacilityPool,
    /// Imagen del bloque anterior justo antes de su primera relocalización.
    pub relocated: Option<&'a mut Option<Vec<[u8; 20]>>>,
}
impl RecountFacilities for NativeFacilities<'_> {
    fn capacity(&self) -> usize {
        self.pool.records.len()
    }
    fn facility(&self, i: u16) -> Result<[u8; 20], String> {
        self.pool
            .records
            .get(i as usize)
            .copied()
            .ok_or_else(|| "índice de negocio".into())
    }
    fn resize(&mut self, i: u16, t: &Town, n: i16) -> Result<(), String> {
        super::facility_recount::set_building_count(&mut self.pool.records[i as usize], t, i32::from(n))
            .map_err(String::from)
    }
    fn set_next(&mut self, i: u16, n: u16) -> Result<(), String> {
        put16(&mut self.pool.records[i as usize], 8, n);
        Ok(())
    }
    fn release_workers(&mut self, i: u16, t: &mut Town) -> Result<(), String> {
        let f = &self.pool.records[i as usize];
        if f[7] != t.index() {
            return Err("negocio de otra ciudad".into());
        }
        let mut proxy = t.clone();
        let n = get16(f, 4);
        proxy.set_u16(Town::fac(0, crate::town::fac::EMPLOYEES), n);
        super::employment::release_workers(&mut proxy, 0, u32::from(n), true);
        put16(
            &mut self.pool.records[i as usize],
            4,
            proxy.u16(Town::fac(0, crate::town::fac::EMPLOYEES)),
        );
        for j in 0..4 {
            t.set_pop(j, proxy.pop(j));
        }
        Ok(())
    }
    fn release_slot(&mut self, i: u16, t: &mut Town) -> Result<(), String> {
        let mut towns = vec![Town::default(); usize::from(t.index()) + 1];
        towns[usize::from(t.index())] = t.clone();
        self.pool.release(i, &mut towns)?;
        *t = towns[usize::from(t.index())].clone();
        Ok(())
    }
    fn create(&mut self, kind: u8, city: u8, n: i16, t: &Town, next: u16) -> Result<u16, String> {
        if usize::from(self.pool.free) >= self.pool.records.len() {
            if let Some(snapshot) = self.relocated.as_deref_mut() {
                if snapshot.is_none() {
                    *snapshot = Some(self.pool.records.clone());
                }
            }
        }
        let i = self.pool.allocate()?;
        super::facility_recount::init(&mut self.pool.records[i as usize], t, city, kind, i32::from(n))
            .map_err(String::from)?;
        put16(&mut self.pool.records[i as usize], 8, next);
        Ok(i)
    }
}
