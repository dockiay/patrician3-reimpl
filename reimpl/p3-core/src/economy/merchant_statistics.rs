//! Captura de estadísticas de comerciante (0x0054C520).
const PRICES: [u32; 24] = [
    0x3d6147af, 0x3ef50482, 0x3e6154ca, 0x3e322d0e, 0x3e11eb85, 0x3f0ccccd, 0x3fb33333, 0x3f8ccccd, 0x3f845a1d,
    0x40587ae1, 0x3ed33333, 0x3ce147af, 0x3fa39581, 0x3f8f5c29, 0x3ee147af, 0x3e8e5604, 0x3ee147af, 0x3e6147af,
    0x3f5ae147, 0x3d236e30, 0x41200000, 0x41000000, 0x41400000, 0x41600000,
];
fn u16at(b: &[u8], p: usize) -> u16 {
    u16::from_le_bytes([b[p], b[p + 1]])
}
fn u32at(b: &[u8], p: usize) -> u32 {
    u32::from_le_bytes(b[p..p + 4].try_into().unwrap())
}
fn put32(b: &mut [u8], p: usize, v: u32) {
    b[p..p + 4].copy_from_slice(&v.to_le_bytes())
}
fn put16(b: &mut [u8], p: usize, v: u16) {
    b[p..p + 2].copy_from_slice(&v.to_le_bytes())
}

/// Captura un comerciante y reinicia sus acumuladores, con datos de oficinas y
/// negocios en imágenes originales. Las reputaciones son entradas explícitas.
pub fn capture(
    merchant: &mut [u8],
    offices: &[Vec<u8>],
    facilities: &[[u8; 20]],
    captain_count: u16,
    home_reputation: i16,
    total_reputation: i16,
) -> Result<[u8; 0x6c], &'static str> {
    if merchant.len() != 0x650 {
        return Err("comerciante truncado");
    }
    let mut office_order = Vec::new();
    let mut seen = vec![false; offices.len()];
    let mut next = usize::from(u16at(merchant, 0xc));
    while next < offices.len() {
        if seen[next] {
            return Err("cadena de oficinas cíclica");
        }
        seen[next] = true;
        let office = &offices[next];
        if office.len() != 0x44c {
            return Err("oficina truncada");
        }
        office_order.push(next);
        next = usize::from(u16at(office, 0x2c8));
    }
    let mut business_counts = Vec::with_capacity(office_order.len());
    for &i in &office_order {
        let mut count = 0u32;
        let mut seen = vec![false; facilities.len()];
        let mut next = usize::from(u16at(&offices[i], 0x2cc));
        while next < facilities.len() {
            if seen[next] {
                return Err("cadena de negocios cíclica");
            }
            seen[next] = true;
            let record = &facilities[next];
            count = count.wrapping_add(u32::from(u16at(record, 4)));
            next = usize::from(u16at(record, 8));
        }
        business_counts.push(count);
    }
    let mut out = [0u8; 0x6c];
    let flt = |v: i32| (v as f32).to_bits();
    put32(&mut out, 0, flt(u32at(merchant, 0x46c) as i32));
    put32(&mut out, 4, flt((u32at(merchant, 0) as i32).max(0)));
    put32(&mut out, 0x10, flt(u32at(merchant, 0x498) as i32));
    for &(src, dst, float) in &[
        (0x484, 8, false),
        (0x494, 0xc, false),
        (0x49c, 0x14, true),
        (0x4a0, 0x18, true),
        (0x4a4, 0x1c, false),
        (0x48c, 0x20, false),
        (0x4ac, 0x24, false),
        (0x4c0, 0x28, false),
        (0x4c4, 0x2c, false),
        (0x4c8, 0x30, false),
        (0x4b8, 0x34, false),
        (0x4bc, 0x38, false),
        (0x4b0, 0x3c, false),
        (0x4b4, 0x40, false),
        (0x4cc, 0x44, false),
    ] {
        let v = u32at(merchant, src);
        put32(&mut out, dst, if float { flt(v as i32) } else { v });
        put32(merchant, src, 0);
    }
    let (mut quantity, mut value) = (0f32, 0f32);
    let (mut buildings, mut houses, mut count, mut workers) = (0u16, 0u16, 0u16, 0u32);
    for (j, &i) in office_order.iter().enumerate() {
        let office = &offices[i];
        count = count.wrapping_add(1);
        for (w, &price) in PRICES.iter().enumerate() {
            let q = u32at(office, 0xc4 + 4 * w) as i32;
            if q > 0 {
                let q = q as f32;
                quantity += q;
                value = ((value as f64) + (q as f64) * f64::from(f32::from_bits(price))) as f32;
            }
        }
        let mut capacity = u32::from(u16at(office, 0x2ec));
        if u16at(office, 0x2f0) == 0 {
            capacity = capacity.wrapping_sub(50)
        }
        capacity /= 200;
        let b = u32::from(u16at(office, 0x2d2));
        if capacity < b {
            buildings = buildings.wrapping_add((b - capacity) as u16)
        }
        houses = houses.wrapping_add(u16at(office, 0x2d4));
        if u16at(office, 0x2f2) < captain_count {
            workers = workers.wrapping_add(1)
        }
        workers = workers
            .wrapping_add(u32::from(office[0x2c7]))
            .wrapping_add(business_counts[j]);
    }
    put32(&mut out, 0x48, quantity.to_bits());
    put32(&mut out, 0x4c, value.to_bits());
    put32(&mut out, 0x50, u32at(merchant, 0x470));
    put16(&mut out, 0x54, u16at(merchant, 0x478));
    put16(&mut out, 0x56, buildings);
    put16(&mut out, 0x58, houses);
    put16(&mut out, 0x5a, count);
    put32(&mut out, 0x5c, workers);
    put32(&mut out, 0x60, u32at(merchant, 0x474));
    put32(&mut out, 0x64, (f32::from(home_reputation)).to_bits());
    put32(&mut out, 0x68, (f32::from(total_reputation)).to_bits());
    Ok(out)
}

/// Suma dos muestras con la aritmética y tamaños del original.
pub fn accumulate(dst: &mut [u8; 0x6c], src: &[u8; 0x6c]) {
    for p in [0, 4, 0x10, 0x14, 0x18, 0x48, 0x4c, 0x64, 0x68] {
        let value = f32::from_bits(u32at(dst, p)) + f32::from_bits(u32at(src, p));
        put32(dst, p, value.to_bits());
    }
    for p in [
        8, 0xc, 0x1c, 0x20, 0x24, 0x28, 0x2c, 0x30, 0x34, 0x38, 0x3c, 0x40, 0x44, 0x50, 0x5c, 0x60,
    ] {
        let v = u32at(dst, p).wrapping_add(u32at(src, p));
        put32(dst, p, v);
    }
    for p in [0x54, 0x56, 0x58, 0x5a] {
        let v = u16at(dst, p).wrapping_add(u16at(src, p));
        put16(dst, p, v);
    }
}
/// Promedia una muestra acumulada; los ingresos y gastos siguen siendo sumas.
pub fn average(record: &mut [u8; 0x6c], count: u16) -> Result<(), &'static str> {
    if count == 0 {
        return Err("divisor cero");
    }
    let n = u32::from(count);
    let half = n / 2;
    let inverse = 1.0f32 / (n as f32);
    for p in [0, 4, 0x10, 0x48, 0x4c, 0x64, 0x68] {
        let v = (f32::from_bits(u32at(record, p)) * inverse).to_bits();
        put32(record, p, v);
    }
    let signed = (u32at(record, 0x50) as i32).wrapping_add(half as i32) / (n as i32);
    put32(record, 0x50, signed as u32);
    for p in [0x5c, 0x60] {
        let v = u32at(record, p).wrapping_add(half) / n;
        put32(record, p, v);
    }
    for p in [0x54, 0x56, 0x58, 0x5a] {
        let v = ((u32::from(u16at(record, p)) + half) / n) as u16;
        put16(record, p, v);
    }
    Ok(())
}

/// Avanza los tres anillos y, cuando corresponde, antepone un año.
/// `item` usa el formato serializado de `MerchantStatsHistory` (0x0054CBD0).
pub fn advance_history(
    item: &mut Vec<u8>,
    sample: &[u8; 0x6c],
    week: bool,
    month: bool,
    year: bool,
    current_month: u8,
) -> Result<(), &'static str> {
    const SIZE: usize = 0x6c;
    const BASE: usize = 8 + 80 * SIZE;
    const DAYS: [usize; 12] = [31, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30];
    if item.len() < BASE || current_month >= 12 {
        return Err("histórico de comerciante inválido");
    }
    let years = usize::from(u16at(item, 6));
    if item.len() != BASE + years * SIZE {
        return Err("años de histórico truncados");
    }
    let day = usize::from(u16at(item, 2));
    let weekly = usize::from(item[4]);
    let monthly = usize::from(item[5]);
    if day >= 32 || weekly >= 24 || monthly >= 24 {
        return Err("cursor de histórico inválido");
    }
    if year && years == usize::from(u16::MAX) {
        return Err("demasiados años");
    }
    let mut next = item.clone();
    let day = (day + 1) % 32;
    put16(&mut next, 2, day as u16);
    next[8 + day * SIZE..8 + (day + 1) * SIZE].copy_from_slice(sample);
    if week {
        let weekly = (weekly + 1) % 24;
        next[4] = weekly as u8;
        let mut sum = [0u8; SIZE];
        for j in 0..7 {
            let i = (day + 32 - j) % 32;
            let src: &[u8; SIZE] = next[8 + i * SIZE..8 + (i + 1) * SIZE].try_into().unwrap();
            accumulate(&mut sum, src)
        }
        average(&mut sum, 7)?;
        let at = 8 + (32 + weekly) * SIZE;
        next[at..at + SIZE].copy_from_slice(&sum);
    }
    let monthly = if month {
        let m = (monthly + 1) % 24;
        next[5] = m as u8;
        let count = DAYS[usize::from(current_month)];
        let mut sum = [0u8; SIZE];
        for j in 0..count {
            let i = (day + 32 - j) % 32;
            let src: &[u8; SIZE] = next[8 + i * SIZE..8 + (i + 1) * SIZE].try_into().unwrap();
            accumulate(&mut sum, src)
        }
        average(&mut sum, count as u16)?;
        let at = 8 + (56 + m) * SIZE;
        next[at..at + SIZE].copy_from_slice(&sum);
        m
    } else {
        monthly
    };
    if year {
        let mut sum = [0u8; SIZE];
        for j in 0..12 {
            let i = (monthly + 24 - j) % 24;
            let at = 8 + (56 + i) * SIZE;
            let src: &[u8; SIZE] = next[at..at + SIZE].try_into().unwrap();
            accumulate(&mut sum, src)
        }
        average(&mut sum, 12)?;
        next.splice(BASE..BASE, sum);
        put16(&mut next, 6, (years + 1) as u16);
    }
    *item = next;
    Ok(())
}

/// Construye los anillos iniciales que serializa `0x0054CBD0`.
pub fn new_history(owner: u16, sample: &[u8; 0x6c]) -> Vec<u8> {
    const SIZE: usize = 0x6c;
    let mut out = vec![0; 8 + 80 * SIZE];
    put16(&mut out, 0, owner);
    let mut copied = [0u8; SIZE];
    accumulate(&mut copied, sample);
    for (start, count) in [(0, 32), (32, 24), (56, 24)] {
        for i in 0..count {
            let at = 8 + (start + i) * SIZE;
            let mut record = if start == 0 && i == 0 { *sample } else { copied };
            if i > 0 {
                record[..4].fill(0)
            }
            out[at..at + SIZE].copy_from_slice(&record);
        }
    }
    out
}
