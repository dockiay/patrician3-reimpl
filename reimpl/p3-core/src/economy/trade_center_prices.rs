//! Precios de centros 0x52D4F0/0x52D6B0; tablas aportadas por el anfitrión.
pub struct Tables<'a> {
    pub base: &'a [f32; 24],
    pub units: &'a [u8; 24],
    pub sell_caps: &'a [i32; 4],
    pub buy_caps: &'a [i32; 4],
}
fn i32at(b: &[u8], p: usize) -> i32 {
    i32::from_le_bytes(b[p..p + 4].try_into().unwrap())
}
fn factor(value: i64, price: f32) -> i64 {
    let product = (value as f32 as f64) * (price as f64);
    if !product.is_finite() || product >= 9223372036854775808.0 || product < -9223372036854775808.0 {
        i64::MIN
    } else {
        product as i64
    }
}
fn total(high: i64, low: i64, capacity: i32, position: i32, amount: i32) -> Result<i32, &'static str> {
    if capacity == 0 {
        return Err("capacidad de precios nula");
    }
    let slope = high
        .wrapping_sub(low)
        .checked_div(i64::from(capacity))
        .ok_or("división de precios inválida")?;
    let value = high
        .wrapping_sub(slope.wrapping_mul(i64::from(position)))
        .wrapping_mul(i64::from(amount));
    Ok((value >> 32) as i32)
}
/// 52D6B0 no modifica centro ni cantidad. Se respeta el redondeo a f32 de
/// los coeficientes fijos antes de la multiplicación x87 por precio base.
pub fn buy(center: &[u8], slot: usize, amount: i32, mode: bool, t: &Tables<'_>) -> Result<i32, &'static str> {
    if center.len() != 44 || slot >= 2 {
        return Err("centro o mercancía inválidos");
    }
    let kind = usize::from(center[5]);
    let ware = usize::from(center[slot]);
    if kind >= 3 || ware >= 24 {
        return Err("tabla de precios fuera de rango");
    }
    let (a, b) = match (kind, mode) {
        (0 | 1, false) => (0x14ccccccci64, 0x100000000i64),
        (0 | 1, true) => (0x133333333i64, 0xe6666666i64),
        (2, _) => (0, 0),
        _ => (1, 1),
    };
    let price = t.base[ware];
    if !price.is_finite() {
        return Err("precio no finito");
    }
    total(
        factor(a, price),
        factor(b, price),
        t.buy_caps[kind],
        i32at(center, 0xc + slot * 4).wrapping_sub(amount / 2),
        amount,
    )
}
/// 52D4F0 devuelve ingreso y modifica el saldo de mercancía no vendida;
/// el centro solo se consulta. Las cantidades se redondean al lote original.
pub fn sell(center: &[u8], slot: usize, amount: &mut i32, mode: bool, t: &Tables<'_>) -> Result<i32, &'static str> {
    if center.len() != 44 {
        return Err("centro inválido");
    }
    let slot = slot & 1;
    let kind = usize::from(center[5]);
    let ware = usize::from(center[2 + slot]);
    if kind >= 3 || ware >= 24 {
        return Err("tabla de precios fuera de rango");
    }
    let capacity = t.sell_caps[kind];
    if capacity == 0 {
        return Err("capacidad de precios nula");
    }
    let mut accepted = capacity.wrapping_sub(i32at(center, 0x14 + slot * 4));
    let leftover = if *amount < accepted {
        accepted = *amount;
        0
    } else {
        let rest = amount.wrapping_sub(accepted);
        let lot = if t.units[ware] == 0 { 2000 } else { 200 };
        (rest / lot).wrapping_mul(lot)
    };
    let (a, b) = match (kind, mode) {
        (0 | 1, false) => (0x180000000i64, 0x133333333i64),
        (0 | 1, true) => (0x199999999i64, 0x14ccccccci64),
        (2, _) => (0x200000000i64, 0x180000000i64),
        _ => (1, 1),
    };
    let price = t.base[ware];
    if !price.is_finite() {
        return Err("precio no finito");
    }
    let revenue = total(
        factor(a, price),
        factor(b, price),
        capacity,
        (accepted / 2).wrapping_add(i32at(center, 0x14 + slot * 4)),
        accepted,
    )?;
    *amount = leftover;
    Ok(revenue)
}
