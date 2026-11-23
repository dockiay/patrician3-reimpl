//! Ruta acreditada de `st_debt_repayment` (0x004DB4E0).
//! El embargo de mercancías y otras rutas se dejan en la cola hasta portarlas.
use crate::{ai::reputation::Merchant, scheduler::TaskRecord};

/// El deudor puede liquidar todo en efectivo; el acreedor es externo o existe.
pub fn can_settle_cash(r: &TaskRecord, merchants: &[Merchant]) -> bool {
    let amount = r.u32_at(8) as i32;
    let debtor = usize::from(r.u8_at(0x11));
    r.kind == 1 && r.u8_at(0x12) == 0 && amount > 0 && merchants.get(debtor).is_some_and(|m| m.i32(0) >= amount)
}
/// Devuelve destinatario, ciudad e importe del aviso 0x13; el llamante elimina la tarea.
pub fn settle_cash(r: &mut TaskRecord, merchants: &mut [Merchant]) -> Option<(u8, u8, u32)> {
    if !can_settle_cash(r, merchants) {
        return None;
    }
    r.set_u8_at(0x13, 0);
    let debtor = r.u8_at(0x11);
    let amount = r.u32_at(8);
    let m = &mut merchants[debtor as usize];
    m.set_i32(0, m.i32(0) - amount as i32);
    let creditor = usize::from(r.u8_at(0x10));
    if let Some(m) = merchants.get_mut(creditor) {
        m.set_i32(0, m.i32(0).wrapping_add(amount as i32));
    }
    Some((debtor, r.u8_at(0x14), amount))
}

/// Aviso previo dirigido a un deudor existente, antes del embargo.
pub fn can_notify_debtor(r: &TaskRecord, merchants: &[Merchant]) -> bool {
    r.kind == 1 && r.u32_at(8) != 0 && r.u8_at(0x12) != 0 && usize::from(r.u8_at(0x11)) < merchants.len()
}
/// Actualiza el registro y devuelve destinatario, ciudad, importe y fecha original.
pub fn notify_debtor(r: &mut TaskRecord, merchants: &[Merchant]) -> Option<(u8, u8, u16, u32)> {
    if !can_notify_debtor(r, merchants) {
        return None;
    }
    let event = (r.u8_at(0x11), r.u8_at(0x14), r.u32_at(8) as u16, r.u32_at(0xc));
    r.set_u8_at(0x12, 0);
    r.due = r.due.wrapping_add(0xa00);
    Some(event)
}

/// Primera resolución con deudor externo y acreedor existente; la fecha está alineada al día.
pub fn can_credit_external(r: &TaskRecord, merchants: &[Merchant]) -> bool {
    r.kind == 1
        && r.u32_at(8) != 0
        && r.u8_at(0x12) != 0
        && r.u8_at(0xc) == 0
        && usize::from(r.u8_at(0x11)) >= merchants.len()
        && usize::from(r.u8_at(0x10)) < merchants.len()
}
/// Abona el importe al acreedor; devuelve datos de la carta 0x26.
pub fn credit_external(r: &TaskRecord, merchants: &mut [Merchant]) -> Option<(u8, u8, u16, u32)> {
    if !can_credit_external(r, merchants) {
        return None;
    }
    let creditor = r.u8_at(0x10);
    let m = &mut merchants[creditor as usize];
    m.set_i32(0, m.i32(0).wrapping_add(r.u32_at(8) as i32));
    Some((creditor, r.u8_at(0x14), r.u32_at(8) as u16, r.u32_at(0xc)))
}

/// Aviso 0x79 a acreedor existente cuando el deudor es externo y la fecha no está alineada.
pub fn can_alert_external(r: &TaskRecord, merchants: &[Merchant]) -> bool {
    r.kind == 1
        && r.u32_at(8) != 0
        && r.u8_at(0x12) != 0
        && r.u8_at(0xc) != 0
        && usize::from(r.u8_at(0x11)) >= merchants.len()
        && usize::from(r.u8_at(0x10)) < merchants.len()
}
/// Limpia el aviso y aplaza la deuda; devuelve datos de la carta 0x79.
pub fn alert_external(r: &mut TaskRecord, merchants: &[Merchant], head: u16) -> Option<(u8, u8, u32, u32)> {
    if !can_alert_external(r, merchants) {
        return None;
    }
    let creditor = r.u8_at(0x10);
    let m = &merchants[creditor as usize];
    let amount = r.u32_at(8);
    let a = u32::from(amount as u16) | (u32::from(head) << 16);
    let b = ((amount >> 10) & 0xc0)
        | u32::from(r.u8_at(0x15))
        | (u32::from(r.u8_at(0x17)) << 8)
        | (u32::from(m.bytes[0x1c]) << 16)
        | (u32::from(m.bytes[0x1d]) << 24);
    let town = r.u8_at(0x14);
    r.set_u8_at(0x12, 0);
    r.due = r.due.wrapping_add(0xa00);
    Some((creditor, town, a, b))
}

/// Pago parcial cuando el anfitrión ha acreditado que no hay bienes embargables.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PartialCash {
    pub debtor: u8,
    pub town: u8,
    pub reported: u32,
    pub notify: bool,
    pub keep: bool,
}
pub fn can_settle_partial_cash(r: &TaskRecord, merchants: &[Merchant]) -> bool {
    let amount = r.u32_at(8) as i32;
    let borrower = usize::from(r.u8_at(0x11));
    r.kind == 1
        && r.u8_at(0x12) == 0
        && amount > 0
        && merchants
            .get(borrower)
            .is_some_and(|m| m.i32(0) >= 0 && m.i32(0) < amount)
}
/// El embargo de bienes también se ejecuta cuando la caja del deudor es negativa.
pub fn can_seize_assets(r: &TaskRecord, merchants: &[Merchant]) -> bool {
    let amount = r.u32_at(8) as i32;
    let borrower = usize::from(r.u8_at(0x11));
    r.kind == 1 && r.u8_at(0x12) == 0 && amount > 0 && merchants.get(borrower).is_some_and(|m| m.i32(0) < amount)
}
pub fn settle_partial_cash(r: &mut TaskRecord, merchants: &mut [Merchant]) -> Option<PartialCash> {
    if !can_settle_partial_cash(r, merchants) {
        return None;
    }
    let debtor = r.u8_at(0x11);
    let creditor = r.u8_at(0x10);
    let amount = r.u32_at(8);
    let town = r.u8_at(0x14);
    let m = &mut merchants[debtor as usize];
    let paid = m.i32(0) as u32;
    let human = m.u16(8) == 0;
    m.set_i32(0, 0);
    let remaining = amount - paid;
    r.set_u8_at(0x13, 0);
    if let Some(m) = merchants.get_mut(creditor as usize) {
        m.set_i32(0, m.i32(0).wrapping_add(paid as i32));
    }
    let keep = usize::from(creditor) < merchants.len() || human;
    if keep {
        r.set_u32_at(8, remaining);
        r.due = r.due.wrapping_add(0xa00);
    }
    let reported = u32::from(paid as u16);
    Some(PartialCash {
        debtor,
        town,
        reported,
        notify: human && reported != 0,
        keep,
    })
}

/// Deuda sin importe o sin deudor ni acreedor: el original termina sin efectos.
pub fn is_terminal(r: &TaskRecord) -> bool {
    r.kind == 1 && (r.u32_at(8) == 0 || (r.u8_at(0x10) == 255 && r.u8_at(0x11) == 255))
}
pub fn missing_debtor_is_terminal(r: &TaskRecord, merchants: &[Merchant]) -> bool {
    r.kind == 1 && r.u8_at(0x12) == 0 && usize::from(r.u8_at(0x11)) >= merchants.len()
}
/// El original trata el importe con signo al cobrar, pero lo suma al acreedor.
pub fn settle_negative_amount(r: &mut TaskRecord, merchants: &mut [Merchant]) -> bool {
    let amount = r.u32_at(8) as i32;
    if r.kind != 1 || r.u8_at(0x12) != 0 || amount >= 0 || usize::from(r.u8_at(0x11)) >= merchants.len() {
        return false;
    }
    r.set_u8_at(0x13, 0);
    if let Some(m) = merchants.get_mut(usize::from(r.u8_at(0x10))) {
        m.set_i32(0, m.i32(0).wrapping_add(amount));
    }
    true
}

/// Resultado de un embargo cuando se ha verificado que no hay barcos y que
/// la oficina suministrada pertenece al deudor en la única ciudad afectada.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OfficeSeizure {
    pub debtor: u8,
    pub town: u8,
    pub reported: u32,
    pub kind: u8,
    pub settled: bool,
    pub notify: bool,
    pub keep: bool,
    pub remaining: i32,
}

/// Vista del barco en la cadena del comerciante para el embargo.
#[derive(Clone, Debug, PartialEq)]
pub struct DebtShip {
    pub cargo: crate::economy::ship_trade::ShipCargo,
    pub next: u16,
    pub state: u16,
    pub name: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ShipSeizure {
    pub debtor: u8,
    pub town: u8,
    pub ship_name: u16,
    pub ship_index: Option<usize>,
    pub kind: u8,
    pub settled: bool,
    pub reported: u32,
    pub letter: bool,
    pub notify: bool,
    pub keep: bool,
    pub remaining: i32,
}

/// Camino de una ciudad sin oficina. `ships` contiene la cadena validada;
/// la búsqueda del barco sigue los índices originales y no admite ciclos.
pub fn seize_ships(
    r: &mut TaskRecord,
    merchants: &mut [Merchant],
    town: &mut crate::town::Town,
    ships: &mut [DebtShip],
    first: u16,
    difficulty: f32,
) -> Option<ShipSeizure> {
    if !can_seize_assets(r, merchants) {
        return None;
    }
    let debtor = r.u8_at(0x11);
    let city = r.u8_at(0x14);
    let creditor = usize::from(r.u8_at(0x10));
    let amount = r.u32_at(8) as i32;
    let human = merchants[usize::from(debtor)].u16(8) == 0;
    let cash = merchants[usize::from(debtor)].i32(0).max(0);
    let mut remaining = amount - cash;
    let mut reported = u32::from(cash as u16);
    let mut name = 0;
    let mut sold = false;
    let mut ship_index = None;
    if cash > 0 {
        merchants[usize::from(debtor)].set_i32(0, 0)
    }
    r.set_u8_at(0x13, 0);
    let mut cursor = first;
    let mut steps = 0;
    while usize::from(cursor) < ships.len() && steps < ships.len() {
        let ship = &mut ships[usize::from(cursor)];
        if ship.cargo.town == city && ship.state < 12 {
            for ware in (0..24).rev() {
                if remaining < 1 {
                    break;
                }
                let quantity = ship.cargo.stock[ware];
                if quantity <= 0 {
                    continue;
                }
                let market = crate::economy::prices::Market {
                    trading: town.more_flags() & 0x100 != 0,
                    stock: town.stock(ware),
                    thresholds: std::array::from_fn(|i| town.threshold(ware, i)),
                    marked_ware: town.u8(0x6f0),
                };
                let price = crate::economy::prices::sell_price(ware, &market, quantity, difficulty);
                if reported.wrapping_add(price as u32) < 65000 {
                    town.set_stock(ware, town.stock(ware).wrapping_add(quantity));
                    ship.cargo.stock[ware] = 0;
                    reported = reported.wrapping_add(price as u32);
                    remaining = remaining.wrapping_sub(price);
                    sold = true;
                }
            }
            if sold {
                name = ship.name;
                ship_index = Some(usize::from(cursor));
                ship.cargo.dirty |= 4;
                ship.cargo.recompute_load();
                break;
            }
        }
        cursor = ship.next;
        steps += 1;
    }
    let settled = sold && remaining < 1;
    if settled {
        reported = reported.wrapping_add(remaining as u32);
        let m = &mut merchants[usize::from(debtor)];
        m.set_i32(0, m.i32(0).wrapping_sub(remaining));
        remaining = 0;
    }
    if let Some(m) = merchants.get_mut(creditor) {
        m.set_i32(0, m.i32(0).wrapping_add(amount).wrapping_sub(remaining));
    }
    let keep = remaining > 0 && (creditor < merchants.len() || human);
    if keep {
        r.set_u32_at(8, remaining as u32);
        r.due = r.due.wrapping_add(0xa00);
    }
    Some(ShipSeizure {
        debtor,
        town: city,
        ship_name: name,
        ship_index,
        kind: if sold { 2 } else { 0 },
        settled,
        reported,
        letter: sold || (human && reported != 0),
        notify: !sold && human && reported != 0,
        keep,
        remaining,
    })
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebtSeizureEvent {
    pub town: u8,
    pub ship_name: u16,
    pub kind: u8,
    pub settled: bool,
    pub reported: u32,
    pub letter: bool,
    pub notify: bool,
    pub ship_index: Option<usize>,
}

/// Dos ciudades con oficinas y flota ya verificadas por el llamante. El acreedor
/// se abona una sola vez, después de las dos posibles visitas del original.
pub fn seize_two_cities(
    r: &mut TaskRecord,
    merchants: &mut [Merchant],
    towns: &mut [crate::town::Town],
    offices: &mut [Option<[i32; 24]>],
    ships: &mut [DebtShip],
    first: u16,
    difficulty: f32,
) -> Option<(bool, Vec<DebtSeizureEvent>)> {
    if !can_seize_assets(r, merchants) {
        return None;
    }
    let borrower = usize::from(r.u8_at(0x11));
    let target = usize::from(r.u8_at(0x14));
    let home = usize::from(merchants[borrower].bytes[0x19]);
    if target >= towns.len() || home >= towns.len() || offices.len() != towns.len() {
        return None;
    }
    let amount = r.u32_at(8) as i32;
    let creditor = usize::from(r.u8_at(0x10));
    let human = merchants[borrower].u16(8) == 0;
    let mut remaining = amount;
    let mut events = Vec::new();
    for city in [target, home] {
        if city == home
            && (home == target || amount.wrapping_mul(9) >= remaining.wrapping_mul(10))
            && !events.is_empty()
        {
            break;
        }
        if city == home && home == target && !events.is_empty() {
            break;
        }
        let mut temp = *r;
        temp.set_u8_at(0x10, 255);
        temp.set_u8_at(0x14, city as u8);
        temp.set_u32_at(8, remaining as u32);
        if let Some(stock) = offices[city].as_mut().filter(|s| s.iter().any(|v| *v > 0)) {
            let x = seize_one_office(&mut temp, merchants, &mut towns[city], stock, difficulty)?;
            remaining = x.remaining;
            events.push(DebtSeizureEvent {
                town: city as u8,
                ship_name: 0,
                kind: x.kind,
                settled: x.settled,
                reported: x.reported,
                letter: x.notify,
                notify: x.notify,
                ship_index: None,
            });
        } else {
            let x = seize_ships(&mut temp, merchants, &mut towns[city], ships, first, difficulty)?;
            remaining = x.remaining;
            events.push(DebtSeizureEvent {
                town: city as u8,
                ship_name: x.ship_name,
                kind: x.kind,
                settled: x.settled,
                reported: x.reported,
                letter: x.letter,
                notify: x.notify,
                ship_index: x.ship_index,
            });
        }
        if remaining <= 0 {
            break;
        }
    }
    r.set_u8_at(0x13, 0);
    if let Some(m) = merchants.get_mut(creditor) {
        m.set_i32(0, m.i32(0).wrapping_add(amount).wrapping_sub(remaining));
    }
    let keep = remaining > 0 && (creditor < merchants.len() || human);
    if keep {
        r.set_u32_at(8, remaining as u32);
        r.due = r.due.wrapping_add(0xa00);
    }
    Some((keep, events))
}
/// Ejecuta el camino acreditado con oficina, sin búsqueda de otras ciudades ni barcos.
pub fn seize_one_office(
    r: &mut TaskRecord,
    merchants: &mut [Merchant],
    town: &mut crate::town::Town,
    stock: &mut [i32; 24],
    difficulty: f32,
) -> Option<OfficeSeizure> {
    let borrower = usize::from(r.u8_at(0x11));
    let creditor = usize::from(r.u8_at(0x10));
    let amount = r.u32_at(8) as i32;
    if r.kind != 1
        || r.u8_at(0x12) != 0
        || amount <= 0
        || borrower >= merchants.len()
        || merchants[borrower].i32(0) >= amount
    {
        return None;
    }
    let debtor = borrower as u8;
    let city = r.u8_at(0x14);
    let human = merchants[borrower].u16(8) == 0;
    let cash = merchants[borrower].i32(0).max(0);
    let mut remaining = amount - cash;
    let mut reported = u32::from(cash as u16);
    let mut sold = false;
    if cash > 0 {
        merchants[borrower].set_i32(0, 0);
    }
    r.set_u8_at(0x13, 0);
    for ware in (0..24).rev() {
        if remaining < 1 {
            break;
        }
        let quantity = stock[ware];
        if quantity <= 0 {
            continue;
        }
        let market = crate::economy::prices::Market {
            trading: town.more_flags() & 0x100 != 0,
            stock: town.stock(ware),
            thresholds: std::array::from_fn(|i| town.threshold(ware, i)),
            marked_ware: town.u8(0x6f0),
        };
        let price = crate::economy::prices::sell_price(ware, &market, quantity, difficulty);
        town.set_stock(ware, town.stock(ware).wrapping_add(quantity));
        stock[ware] = 0;
        sold = true;
        reported = reported.wrapping_add(price as u32);
        remaining = remaining.wrapping_sub(price);
    }
    let mut settled = false;
    if sold && remaining < 1 {
        reported = reported.wrapping_add(remaining as u32);
        merchants[borrower].set_i32(0, merchants[borrower].i32(0).wrapping_sub(remaining));
        remaining = 0;
        settled = true;
    }
    if let Some(m) = merchants.get_mut(creditor) {
        m.set_i32(0, m.i32(0).wrapping_add(amount).wrapping_sub(remaining));
    }
    let keep = remaining > 0 && (creditor < merchants.len() || human);
    if keep {
        r.set_u32_at(8, remaining as u32);
        r.due = r.due.wrapping_add(0xa00);
    }
    Some(OfficeSeizure {
        debtor,
        town: city,
        reported,
        kind: u8::from(sold),
        settled,
        notify: human && (sold || reported != 0),
        keep,
        remaining,
    })
}
