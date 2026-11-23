//! Captura diaria de estadísticas urbanas (0x0054D1D0).
use crate::town::Town;
fn read32(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap())
}
fn write32(bytes: &mut [u8], offset: usize, value: u32) {
    bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes())
}

/// Captura las ciudades y limpia únicamente los acumuladores que limpia el
/// original. Las oficinas son imágenes de 0x44C y siguen la lista de ciudad.
/// Una cadena inválida o cíclica se rechaza antes de cambiar las ciudades.
pub fn capture(towns: &mut [Town], offices: &[Vec<u8>]) -> Result<Vec<u8>, &'static str> {
    let mut chains = Vec::with_capacity(towns.len());
    for town in towns.iter() {
        let mut chain = Vec::new();
        let mut seen = vec![false; offices.len()];
        let mut next = usize::from(town.u16(0x784));
        while next < offices.len() {
            if seen[next] {
                return Err("cadena de oficinas cíclica");
            }
            seen[next] = true;
            let office = &offices[next];
            if office.len() != 0x44c {
                return Err("oficina truncada");
            }
            chain.push(next);
            next = usize::from(u16::from_le_bytes([office[0x2ca], office[0x2cb]]));
        }
        chains.push(chain);
    }
    let mut out = vec![0; towns.len().checked_mul(0x204).ok_or("demasiadas ciudades")?];
    for (i, town) in towns.iter_mut().enumerate() {
        let record = &mut out[i * 0x204..(i + 1) * 0x204];
        write32(record, 0x1fc, u32::from(town.u16(0x996)));
        town.set_u16(0x996, 0);
        for w in 0..24 {
            let p = w * 4;
            write32(record, p, town.u32(0x64 + p).wrapping_add(town.u32(0x310 + p)));
            write32(record, 0x60 + p, town.u32(0xc4 + p));
            write32(record, 0xc0 + p, town.u32(4 + p));
            write32(record, 0x120 + p, town.u32(0x370 + p));
            write32(record, 0x180 + p, town.u32(0x430 + p));
            town.set_u32(0x370 + p, 0);
        }
        for &index in &chains[i] {
            let office = &offices[index];
            for w in 0..24 {
                let p = w * 4;
                write32(record, p, read32(record, p).wrapping_add(read32(office, 0x64 + p)));
                write32(
                    record,
                    0x60 + p,
                    read32(record, 0x60 + p).wrapping_add(read32(office, 0xc4 + p)),
                );
            }
        }
        write32(record, 0x200, town.u32(0x2c8));
        record[0x1e0..0x1f0].copy_from_slice(&town.0[0x2d8..0x2e8]);
        record[0x1f0..0x1f6].copy_from_slice(&town.0[0x300..0x306]);
        record[0x1f6..0x1fc].copy_from_slice(&town.0[0x308..0x30e]);
    }
    Ok(out)
}

/// Promedia una cadena de muestras sobre un destino nuevo. El original suma
/// la milicia sin dividirla y divide las poblaciones como u32; las mercancías
/// y los seis indicadores de satisfacción usan división con signo.
pub fn average(nodes: &[Vec<u8>], limit: usize) -> Result<Option<Vec<u8>>, &'static str> {
    if nodes.is_empty() || limit == 0 {
        return Ok(None);
    }
    if nodes.iter().any(|n| n.len() % 0x204 != 0) {
        return Err("muestra urbana truncada");
    }
    let selected = &nodes[..nodes.len().min(limit)];
    let count = selected.len() as u32;
    let towns = selected[0].len() / 0x204;
    let mut out = vec![0; selected[0].len()];
    for node in selected {
        for i in 0..towns.min(node.len() / 0x204) {
            let src = &node[i * 0x204..(i + 1) * 0x204];
            let dst = &mut out[i * 0x204..(i + 1) * 0x204];
            for p in (0..0x1f0).step_by(4) {
                write32(dst, p, read32(dst, p).wrapping_add(read32(src, p)));
            }
            for p in (0x1f0..0x1fc).step_by(2) {
                let a = u16::from_le_bytes([dst[p], dst[p + 1]]);
                let b = u16::from_le_bytes([src[p], src[p + 1]]);
                dst[p..p + 2].copy_from_slice(&a.wrapping_add(b).to_le_bytes());
            }
            write32(dst, 0x1fc, read32(dst, 0x1fc).wrapping_add(read32(src, 0x1fc)));
            write32(dst, 0x200, read32(dst, 0x200) | read32(src, 0x200));
        }
    }
    for dst in out.chunks_exact_mut(0x204) {
        for p in (0..0x1e0).step_by(4) {
            write32(dst, p, ((read32(dst, p) as i32) / (count as i32)) as u32)
        }
        for p in (0x1e0..0x1f0).step_by(4) {
            write32(dst, p, read32(dst, p) / count)
        }
        for p in (0x1f0..0x1fc).step_by(2) {
            let n = i16::from_le_bytes([dst[p], dst[p + 1]]);
            dst[p..p + 2].copy_from_slice(&((i32::from(n) / (count as i32)) as i16).to_le_bytes());
        }
        write32(dst, 0x200, read32(dst, 0x200) & 0xfbfffff9);
    }
    Ok(Some(out))
}
