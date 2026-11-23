//! Plantillas de los negocios (0x004D49D0 y 0x004D3670).
use crate::numeric::ftol_i32;
const PER_BUILDING: [u16; 21] = [
    50, 40, 30, 15, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 15, 15, 30,
];

/// Contadores total, intactos y protegidos; las tres multiplicaciones truncan a u16.
pub fn building_capacities(kind: u8, counts: [u16; 3]) -> Option<[u16; 3]> {
    let n = *PER_BUILDING.get(kind as usize)?;
    Some([
        counts[2].wrapping_mul(n),
        counts[1].wrapping_mul(n),
        counts[0].wrapping_mul(n),
    ])
}
/// Resultado del ajuste: negativo devuelve empleados y positivo registra una vacante.
/// El original carga 0x671FC0 como f32 (0,4), antes de truncar con __ftol.
pub fn employee_delta(protected: u16, intact: u16, factor: f32, employees: u16) -> i32 {
    let target = ftol_i32(f64::from(intact) * f64::from(factor) + f64::from(0.4f32));
    target.min(i32::from(protected)).wrapping_sub(i32::from(employees))
}

/// Recuento diario de una oficina. El orden de las dos listas es significativo.
/// Devuelve (edificios intactos, empleados del tipo 14).
pub fn update(
    town: &mut crate::town::Town,
    site: &crate::buildings::Site,
    building_head: u16,
    business_head: u16,
    businesses: &mut [[u8; 20]],
    labour: &mut super::world::LabourShortage,
) -> Result<(u16, u16), String> {
    use crate::town::{fac, Town};
    let read = |a: &[u8], p: usize| u16::from_le_bytes([a[p], a[p + 1]]);
    let n = usize::from(town.u16(0x776));
    if site.records.len() != n * 8 {
        return Err("registros de ciudad truncados".into());
    }
    let siege = town.flags() & 0x10 != 0;
    let mut counts = [[0u16; 3]; 21];
    let mut intact = 0u16;
    let mut seen = vec![false; n];
    let mut i = usize::from(building_head);
    while i < n {
        if seen[i] {
            return Err("ciclo en edificios".into());
        }
        seen[i] = true;
        let b = &site.records[i * 8..i * 8 + 8];
        let kind = usize::from(b[3]);
        if kind < 21 {
            counts[kind][0] = counts[kind][0].wrapping_add(1);
        }
        if b[6] == 0 {
            intact = intact.wrapping_add(1);
            if kind < 21 {
                counts[kind][1] = counts[kind][1].wrapping_add(1);
                if siege {
                    let at = usize::from(town.u16(0x7ac)) * usize::from(b[0]) + usize::from(b[1]);
                    if *site.cells.get(at).ok_or("celda fuera del plano")? >= 0x80 {
                        counts[kind][2] = counts[kind][2].wrapping_add(1);
                    }
                }
            }
        }
        i = usize::from(read(b, 4));
    }
    if siege {
        counts[5][2] = counts[5][1];
    } else {
        for c in &mut counts[4..] {
            c[2] = c[1];
        }
    }
    let mut chain = Vec::new();
    let mut seen = vec![false; businesses.len()];
    i = usize::from(business_head);
    while i < businesses.len() {
        if seen[i] {
            return Err("ciclo en negocios".into());
        }
        seen[i] = true;
        if businesses[i][6] >= 21 {
            return Err("tipo de negocio fuera de tabla".into());
        }
        if businesses[i][7] != town.index() {
            return Err("negocio de otra ciudad".into());
        }
        chain.push(i);
        i = usize::from(read(&businesses[i], 8));
    }
    let mut total = 0u16;
    for i in chain {
        let f = &mut businesses[i];
        let kind = usize::from(f[6]);
        let caps = building_capacities(kind as u8, counts[kind]).unwrap();
        counts[kind] = [0; 3];
        for (j, c) in caps.into_iter().enumerate() {
            f[10 + 2 * j..12 + 2 * j].copy_from_slice(&c.to_le_bytes());
        }
        let factor = f32::from_bits(u32::from_le_bytes(f[16..20].try_into().unwrap()));
        let delta = employee_delta(caps[0], caps[1], factor, read(f, 4));
        if delta > 0 {
            super::employment::record_labour_shortage(labour, !(i as u16), delta as i16);
        } else if delta < 0 {
            let mut proxy = town.clone();
            proxy.set_u16(Town::fac(0, fac::EMPLOYEES), read(f, 4));
            super::employment::release_workers(&mut proxy, 0, delta.wrapping_neg() as u32, true);
            f[4..6].copy_from_slice(&proxy.u16(Town::fac(0, fac::EMPLOYEES)).to_le_bytes());
            for j in 0..4 {
                town.set_pop(j, proxy.pop(j));
            }
        }
        if kind == 14 {
            total = total.wrapping_add(read(f, 4));
        }
    }
    Ok((intact, total))
}
