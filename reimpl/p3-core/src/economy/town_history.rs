//! Historial diario de información urbana (0x00535450) en formato S04.

/// Actualiza la tabla de banderas y las ocho ranuras de conocimiento.
/// `offices[i]` contiene las ciudades donde el comerciante i tiene oficina;
/// `distances` es la matriz cuadrada de distancias entre ciudades.
pub fn daily_serialized(
    history: &mut Vec<u8>,
    tick: u32,
    flags: &[u32],
    kinds: &[u16],
    offices: &[Vec<u8>],
    distances: &[u8],
) -> Result<(), &'static str> {
    let n = flags.len();
    if n > 40 || kinds.len() > 255 || offices.len() != kinds.len() || distances.len() != n * n {
        return Err("datos de historial incompatibles");
    }
    if offices.iter().flatten().any(|&city| usize::from(city) >= n) {
        return Err("ciudad de oficina inválida");
    }
    if history.len() < 0xa08 {
        return Err("historial truncado");
    }
    let mut table = history[..0xa00].to_vec();
    let mut slots: [u8; 8] = history[0xa00..0xa08].try_into().unwrap();
    let mut ages: [Option<[u8; 40]>; 8] = [None; 8];
    let mut cursor = 0xa08;
    for i in 0..8 {
        if slots[i] != 0xff {
            let raw = history.get(cursor..cursor + 40).ok_or("ranura de historial truncada")?;
            ages[i] = Some(raw.try_into().unwrap());
            cursor += 40;
        }
    }
    if cursor != history.len() {
        return Err("resto inesperado en historial");
    }
    let day = ((tick >> 8) & 15) as usize;
    for (i, &flag) in flags.iter().enumerate() {
        let value = if flag & 0x7c0000 >= 0x600000 {
            flag & !0x20000
        } else {
            flag
        };
        let at = i * 64 + day * 4;
        table[at..at + 4].copy_from_slice(&value.to_le_bytes());
    }
    for i in 0..8 {
        let owner = usize::from(slots[i]);
        if owner >= kinds.len() {
            continue;
        }
        if kinds[owner] != 0 {
            slots[i] = 0xff;
            ages[i] = None;
            continue;
        }
        let values = ages[i].as_mut().ok_or("ranura de historial ausente")?;
        for age in &mut values[..n] {
            *age = age.wrapping_add(1)
        }
        for &city in &offices[owner] {
            values[usize::from(city)] = 0
        }
        for city in 0..n {
            if values[city] == 0 {
                for target in 0..n {
                    let limit = (distances[city * n + target] >> 4) + 1;
                    if limit < values[target] {
                        values[target] = limit
                    }
                }
            } else if values[city] > 15 {
                values[city] = 15
            }
        }
    }
    for (owner, &kind) in kinds.iter().enumerate() {
        if kind != 0 || slots.contains(&(owner as u8)) {
            continue;
        }
        if let Some(i) = slots.iter().position(|&slot| usize::from(slot) >= kinds.len()) {
            slots[i] = owner as u8;
            ages[i] = Some([15; 40]);
        }
    }
    let mut out = table;
    out.extend_from_slice(&slots);
    for value in ages.into_iter().flatten() {
        out.extend_from_slice(&value)
    }
    *history = out;
    Ok(())
}
