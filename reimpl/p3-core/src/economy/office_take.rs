//! Extracción de `office_take_ware_from_stock_and_ships` (`0x00500A70`).
use super::prices::SMALL_LOT_WARES;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ship {
    pub town: u8,
    pub state: u16,
    pub next: u16,
    pub stock: i32,
}

/// Devuelve la cantidad sin conseguir y los barcos que requieren recalcular carga.
/// La lista y cantidades inválidas se rechazan antes de publicar cambios.
pub fn take(
    stock: &mut i32,
    ships: &mut [Ship],
    head: u16,
    town: u8,
    ware: usize,
    amount: i32,
) -> Result<(i32, Vec<usize>), &'static str> {
    if ware >= SMALL_LOT_WARES.len() || amount < 0 || *stock < 0 {
        return Err("mercancía o cantidad inválida");
    }
    let mut out = ships.to_vec();
    let mut left = amount;
    let mut changed = Vec::new();
    if *stock < left {
        let lot = if SMALL_LOT_WARES[ware] { 200 } else { 2000 };
        let mut i = head;
        let mut steps = 0;
        while usize::from(i) < out.len() {
            if steps >= out.len() {
                return Err("ciclo en lista de barcos");
            };
            steps += 1;
            let ship = &mut out[usize::from(i)];
            let next = ship.next;
            if ship.town == town && ship.state < 4 && ship.stock > 0 {
                if left > ship.stock {
                    left -= ship.stock;
                    ship.stock = 0
                } else {
                    ship.stock -= left;
                    left = 0;
                    ship.stock = (ship.stock / lot) * lot
                }
                changed.push(usize::from(i));
            }
            i = next;
        }
    }
    let new_stock = if *stock >= left {
        let result = *stock - left;
        left = 0;
        result
    } else {
        left -= *stock;
        0
    };
    *stock = new_stock;
    ships.clone_from_slice(&out);
    Ok((left, changed))
}
