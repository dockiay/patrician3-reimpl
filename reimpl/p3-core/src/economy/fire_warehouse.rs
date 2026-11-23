//! Pérdidas de `Office::lose_wares_in_warehouse_fire` (0x00500C30).
const UNITS: [u8; 24] = [0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1];

/// Aplica la pérdida a una imagen de oficina y devuelve el contenido de carta 0x75.
/// `names` contiene los índices de nombre y apellido del propietario.
pub fn lose(image: &mut [u8], tick: u32, names: [u8; 2]) -> Result<Option<[u8; 8]>, &'static str> {
    if image.len() != 0x44c {
        return Err("oficina de incendio truncada");
    }
    let mut stock = [0i32; 24];
    for (i, v) in stock.iter_mut().enumerate() {
        *v = i32::from_le_bytes(image[4 + i * 4..8 + i * 4].try_into().unwrap())
    }
    let capacity = u16::from_le_bytes([image[0x2ec], image[0x2ed]]) as i32;
    if stock.iter().any(|&v| v < 0) {
        return Err("existencias negativas en incendio de almacén");
    }
    let total = stock
        .iter()
        .try_fold(0i32, |a, &b| a.checked_add(b))
        .ok_or("existencias de almacén fuera de rango")?;
    let size = capacity * 2000;
    let mut budget = if total >= size {
        200
    } else {
        let seed = (tick.wrapping_add(u32::from(u16::from_le_bytes([image[0x2e2], image[0x2e3]]))) & 63) as i32;
        let factor = total.wrapping_mul(200) / capacity;
        let mut n = seed.wrapping_mul(factor) >> 6;
        if total.wrapping_sub(n) > size - 400000 {
            n = 400000
        }
        n
    };
    let mut least = 0usize;
    for i in 1..24 {
        if stock[i] < stock[least] {
            least = i
        }
    }
    let mut selected = [least; 3];
    for i in 0..24 {
        for k in (0..3).rev() {
            if stock[selected[k]] < stock[i] {
                if k < 2 {
                    selected[k + 1] = selected[k]
                }
                selected[k] = i
            } else {
                break;
            }
        }
    }
    let mut words = [0xff00u16; 3];
    let mut changed = false;
    for k in (0..3).rev() {
        let i = selected[k];
        let amount = (budget / (k as i32 + 1)).min(stock[i]);
        let scale = match UNITS[i] {
            0 => 2000,
            1 => 200,
            _ => 10,
        };
        let quantity = (amount / scale).min(200);
        let removed = quantity * scale;
        if removed > 0 {
            stock[i] = stock[i].wrapping_sub(removed);
            budget = budget.wrapping_sub(removed);
            words[k] = ((i as u16) << 8) | (quantity as u8 as u16);
            changed = true
        }
    }
    if !changed {
        return Ok(None);
    }
    for (i, v) in stock.iter().enumerate() {
        image[4 + i * 4..8 + i * 4].copy_from_slice(&v.to_le_bytes())
    }
    let mut payload = [0u8; 8];
    for (i, v) in words.iter().enumerate() {
        payload[i * 2..i * 2 + 2].copy_from_slice(&v.to_le_bytes())
    }
    payload[6..8].copy_from_slice(&u16::from_be_bytes(names).to_le_bytes());
    Ok(Some(payload))
}
