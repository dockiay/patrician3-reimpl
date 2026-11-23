//! Actualización diaria de TradeCenter (0x0052D7D0) sobre el formato S04.
//! La lista se representa por sus cargas serializadas, sin punteros x86.

/// `trade_center_matches_waypoint` (0x0052BE90).
///
/// Antes de expirar, compara con la posición de la tabla original. Después
/// busca el punto de descubrimiento más cercano al destino y comprueba que
/// quede a menos de cinco unidades, o siete si `wide` tiene su byte bajo activo.
/// Los puntos y la posición deben proceder de la partida y del EXE; el original
/// desreferencia una posición anterior al array si este está vacío.
pub fn matches_waypoint(
    id: u8,
    expiry: u32,
    tick: u32,
    target: (u16, u16),
    waypoint: (u16, u16),
    discovery_points: &[(u16, u16)],
    wide: u8,
) -> Result<bool, &'static str> {
    if id >= 0x7f {
        return Ok(false);
    }
    if expiry <= tick {
        return Ok(waypoint == target);
    }
    let squared = |a: (u16, u16), b: (u16, u16)| {
        let dx = i32::from(a.0).wrapping_sub(i32::from(b.0));
        let dy = i32::from(a.1).wrapping_sub(i32::from(b.1));
        dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy))
    };
    let mut nearest = None;
    let mut distance = i32::MAX;
    for &point in discovery_points {
        let candidate = squared(waypoint, point);
        if candidate < distance {
            distance = candidate;
            nearest = Some(point);
        }
    }
    let nearest = nearest.ok_or("sin punto de descubrimiento válido")?;
    let radius = if wide == 0 { 5u32 } else { 7u32 };
    Ok((squared(nearest, target) as u32) < radius * radius)
}

/// Devuelve el número de contratos eliminados. Cada eliminación solicita un
/// aviso de interfaz si la ventana original está abierta; su presentación es
/// responsabilidad del anfitrión.
pub fn daily_serialized(center: &mut Vec<u8>, populations: &[u32], tick: u32) -> Result<usize, &'static str> {
    if center.len() < 30 {
        return Err("centro comercial truncado");
    }
    let count = u16::from_le_bytes([center[28], center[29]]) as usize;
    let expected = 30usize
        .checked_add(count.checked_mul(16).ok_or("cuenta inválida")?)
        .ok_or("cuenta inválida")?;
    if center.len() != expected {
        return Err("lista de contratos truncada");
    }
    let kind = center[5] as usize;
    let (divisor, cap) = match kind {
        0 => (30, 100_000i32),
        1 => (40, 200_000),
        2 => (50, 0),
        _ => return Err("tipo de centro no admitido"),
    };
    let population = populations.iter().fold(0u32, |sum, &n| sum.wrapping_add(n));
    // sqrt(u32) se representa con margen suficiente en f64 para obtener la
    // raíz entera exacta; dividir después reproduce la conversión x87 a entero.
    let increase = ((f64::from(population).sqrt() as u32 / divisor) * 200) as i32;
    let mut next = center[..30].to_vec();
    for index in 0..2 {
        let supply = 12 + index * 4;
        let demand = 20 + index * 4;
        let available = i32::from_le_bytes(next[supply..supply + 4].try_into().unwrap())
            .wrapping_add(increase)
            .min(cap);
        let absorbed = i32::from_le_bytes(next[demand..demand + 4].try_into().unwrap())
            .wrapping_sub(increase)
            .max(0);
        next[supply..supply + 4].copy_from_slice(&available.to_le_bytes());
        next[demand..demand + 4].copy_from_slice(&absorbed.to_le_bytes());
    }
    let mut removed = 0;
    for item in center[30..].chunks_exact(16) {
        let amount = u32::from_le_bytes(item[4..8].try_into().unwrap());
        let expiry = u32::from_le_bytes(item[12..16].try_into().unwrap());
        if expiry < tick || amount == 0 {
            removed += 1;
        } else {
            next.extend_from_slice(item);
        }
    }
    next[28..30].copy_from_slice(&((count - removed) as u16).to_le_bytes());
    *center = next;
    Ok(removed)
}
