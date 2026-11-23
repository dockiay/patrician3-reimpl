//! Retirada de mercancías de los barcos del comerciante presentes en una ciudad (0x004F5450). Referencia: x86 original.

/// Barco con los campos que lee la función: ciudad `+0x39`, siguiente `+4`, estado `+0x134` y existencias `+0x54`.
#[derive(Clone, Debug, Default)]
pub struct CargoShip {
    pub town: u8,
    pub next: u16,
    pub state: u16,
    pub stock: [i32; 20],
}

/// Recuento de bodega de un barco (0x005182B0, gancho).
pub trait CargoHost {
    fn recount_cargo(&mut self, ship: usize);
}

/// `0x004F5450` (thiscall, `ret 0xC`): descuenta `*amount` unidades del bien `good` de los barcos de la lista de
/// `first_ship` (campo `+0xE`) que están en `town` con estado menor que 4 y distinto de 2. `ship_count` es la palabra baja
/// de 0x006DD894. Sin cantidad positiva no hace nada.
pub fn take_goods_from_ships(
    first_ship: u16,
    ship_count: u16,
    ships: &mut [CargoShip],
    good: u32,
    town: u32,
    amount: &mut i32,
    host: &mut dyn CargoHost,
) {
    if *amount <= 0 || first_ship >= ship_count {
        return;
    }
    let mut cur = first_ship;
    loop {
        let i = cur as usize;
        let next = ships[i].next;
        let s = &mut ships[i];
        if s.town as u32 == town && s.state < 4 && s.state != 2 {
            let have = s.stock.get(good as usize).copied().unwrap_or(0);
            if have < *amount {
                *amount = amount.wrapping_sub(have);
                s.stock[good as usize] = 0;
                host.recount_cargo(i);
            } else {
                s.stock[good as usize] = have.wrapping_sub(*amount);
                *amount = 0;
                host.recount_cargo(i);
                return;
            }
        }
        cur = next;
        if cur >= ship_count {
            return;
        }
    }
}
