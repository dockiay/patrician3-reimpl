//! Contratos existentes de 0x52C6C0. La creación aleatoria sigue aparte.
use super::trade_center_convoy::{Request, State};
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContractState {
    pub trade: State,
    pub contracts: Vec<[u8; 16]>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Report {
    pub slot: u8,
    pub direction: u8,
    pub quantity: i32,
    pub cost: i32,
    pub expiry: u32,
    pub completed: bool,
}
pub trait Host: Clone {
    fn free_capacity(&mut self, ship: &mut [u8]) -> Result<i32, String>;
    fn add_cargo(&mut self, ship: &mut [u8], index: u16, ware: u8, amount: i32, cost: i32) -> Result<(), String>;
    fn report(&mut self, report: &Report) -> Result<(), String>;
}
#[derive(Clone)]
pub struct Prepared<H> {
    pub state: ContractState,
    pub host: H,
    pub report: Option<Report>,
}
fn r16(b: &[u8], p: usize) -> u16 {
    u16::from_le_bytes([b[p], b[p + 1]])
}
fn r32(b: &[u8], p: usize) -> i32 {
    i32::from_le_bytes(b[p..p + 4].try_into().unwrap())
}
fn w32(b: &mut [u8], p: usize, n: i32) {
    b[p..p + 4].copy_from_slice(&n.to_le_bytes())
}
fn chain(state: &State) -> Result<Vec<usize>, String> {
    let mut out = vec![];
    let mut seen = vec![false; state.ships.len()];
    let mut n = r16(&state.convoy, 0xa);
    while usize::from(n) < state.ships.len() {
        let i = usize::from(n);
        if seen[i] {
            return Err("ciclo de barcos de contrato".into());
        }
        seen[i] = true;
        if state.ships[i].len() != 0x180 {
            return Err("barco de contrato truncado".into());
        }
        out.push(i);
        n = r16(&state.ships[i], 6)
    }
    Ok(out)
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Created {
    pub state: ContractState,
    pub contract: [u8; 16],
}
/// Crea el contrato cuando no hay otro para el propietario. La presentación
/// textual se delega al anfitrión y debe confirmarse junto al estado preparado.
pub fn create(
    before: &ContractState,
    request: Request,
    time: u32,
    populations: &[i32],
    base_prices: &[f32; 24],
) -> Result<Option<Created>, String> {
    let mut s = before.clone();
    if s.trade.center.len() != 44 || s.trade.convoy.len() != 0x3c {
        return Err("imágenes de contrato inválidas".into());
    }
    if s.trade.center[4] >= 0x7f || request.mode != 2 || !request.allow_contract || s.trade.center[5] >= 2 {
        return Ok(None);
    }
    if usize::from(r16(&s.trade.center, 0x1c)) != s.contracts.len()
        || r32(&s.trade.center, 0x28) != s.contracts.len() as i32
    {
        return Err("conteo de contratos incompatible".into());
    }
    let owner = u16::from(s.trade.convoy[0]);
    if s.contracts.iter().any(|c| r16(c, 0) == owner) {
        return Ok(None);
    }
    let mut rng = crate::rng::LcgRandom::seeded(time);
    let slot = rng.next_below(2) as u8;
    let mut population = 0i64;
    for &n in populations {
        population = population.wrapping_add(i64::from(n))
    }
    let factor = i64::from(rng.next_below(0x8000) + 0x8000);
    let amount = population
        .wrapping_mul(factor)
        .wrapping_div(50)
        .wrapping_shr(16)
        .wrapping_add(500) as i32;
    let direction = rng.next_below(2) as u8;
    let multiplier = if direction == 0 {
        80000 + rng.next_below(30000)
    } else {
        160000 + rng.next_below(20000)
    };
    let ware = usize::from(s.trade.center[usize::from(slot) + if direction == 0 { 0 } else { 2 }]);
    if ware >= 24 || !base_prices[ware].is_finite() {
        return Err("precio de contrato inválido".into());
    }
    let product = f64::from(multiplier as f32) * f64::from(base_prices[ware]) * 200.0;
    let raw = if !product.is_finite() || product >= 9223372036854775808.0 || product < -9223372036854775808.0 {
        i64::MIN
    } else {
        product as i64
    };
    let price = raw.wrapping_div(100000) as i32;
    let mut contract = [0u8; 16];
    contract[..2].copy_from_slice(&owner.to_le_bytes());
    contract[2] = slot;
    contract[3] = direction;
    contract[4..8].copy_from_slice(&amount.to_le_bytes());
    contract[8..12].copy_from_slice(&price.to_le_bytes());
    contract[12..16].copy_from_slice(&time.wrapping_add(0xb400).to_le_bytes());
    s.contracts.push(contract);
    let count = r16(&s.trade.center, 0x1c)
        .checked_add(1)
        .ok_or("conteo de contratos desbordado")?;
    s.trade.center[0x1c..0x1e].copy_from_slice(&count.to_le_bytes());
    let count32 = r32(&s.trade.center, 0x28).wrapping_add(1);
    w32(&mut s.trade.center, 0x28, count32);
    Ok(Some(Created { state: s, contract }))
}
/// No publica sobre las entradas. La rama de crear un contrato se rechaza
/// hasta que el llamante disponga del generador y presentación contrastados.
pub fn prepare<H: Host>(
    before: &ContractState,
    host: &H,
    request: Request,
    before_sales: bool,
    units: &[u8; 24],
) -> Result<Prepared<H>, String> {
    let mut s = before.clone();
    let mut h = host.clone();
    if s.trade.center.len() != 44 || s.trade.convoy.len() != 0x3c || s.trade.merchant.len() != 0x650 {
        return Err("imágenes de contrato inválidas".into());
    }
    if s.trade.center[4] >= 0x7f {
        return Ok(Prepared {
            state: s,
            host: h,
            report: None,
        });
    }
    if usize::from(r16(&s.trade.center, 0x1c)) != s.contracts.len()
        || r32(&s.trade.center, 0x28) != s.contracts.len() as i32
    {
        return Err("conteo de contratos incompatible".into());
    }
    let owner = u16::from(s.trade.convoy[0]);
    let index = s.contracts.iter().position(|c| r16(c, 0) == owner);
    let Some(index) = index else {
        if !before_sales && request.mode == 2 && request.allow_contract && s.trade.center[5] < 2 {
            return Err("creación de contrato comercial pendiente".into());
        }
        return Ok(Prepared {
            state: s,
            host: h,
            report: None,
        });
    };
    let mut contract = s.contracts[index];
    let slot = usize::from(contract[2]);
    let direction = contract[3];
    if slot >= 2 {
        return Err("ranura de contrato inválida".into());
    }
    let mut remaining = r32(&contract, 4);
    let unit_price = r32(&contract, 8);
    if remaining < 0 {
        return Err("cantidad de contrato negativa".into());
    }
    let expiry = u32::from_le_bytes(contract[12..16].try_into().unwrap());
    let mut quantity = 0i32;
    let mut cost = 0i32;
    if direction != 0 && before_sales {
        let ware = usize::from(s.trade.center[2 + slot]);
        if ware >= 24 {
            return Err("mercancía de contrato inválida".into());
        }
        for id in chain(&s.trade)? {
            if remaining == 0 {
                break;
            }
            if r32(&s.trade.ships[id], 0x18) <= 0 {
                continue;
            }
            let at = 0x54 + ware * 4;
            let stock = r32(&s.trade.ships[id], at);
            if stock <= 0 {
                w32(&mut s.trade.ships[id], at, 0)
            } else {
                let taken = stock.min(remaining.checked_mul(200).ok_or("cantidad de contrato excesiva")?);
                let value = unit_price.wrapping_mul(taken / 200);
                let money = r32(&s.trade.convoy, 0x20).wrapping_add(value);
                w32(&mut s.trade.convoy, 0x20, money);
                let total = r32(&s.trade.merchant, 0x4b0).wrapping_add(value);
                w32(&mut s.trade.merchant, 0x4b0, total);
                w32(&mut s.trade.ships[id], at, stock - taken);
                remaining = remaining.wrapping_sub(taken / 200);
                quantity = quantity.wrapping_add(taken);
                cost = cost.wrapping_add(value)
            }
            h.free_capacity(&mut s.trade.ships[id])?;
        }
    } else if direction == 0 && !before_sales {
        let ware = usize::from(s.trade.center[slot]);
        if ware >= 24 {
            return Err("mercancía de contrato inválida".into());
        }
        let lot = if units[ware] == 0 { 2000 } else { 200 };
        let mut free = 0i32;
        for id in chain(&s.trade)? {
            if r32(&s.trade.ships[id], 0x18) > 0 {
                free = free.wrapping_add((h.free_capacity(&mut s.trade.ships[id])? / 200).wrapping_mul(200))
            }
        }
        let mut amount = free.min(remaining.checked_mul(200).ok_or("cantidad de contrato excesiva")?);
        let mut value;
        let mut attempts = 0;
        loop {
            attempts += 1;
            if attempts > 128 {
                return Err("presupuesto de contrato no converge".into());
            }
            amount = (amount / lot).wrapping_mul(lot);
            value = (amount / lot).wrapping_mul(unit_price);
            let budget = r32(&s.trade.convoy, 0x20);
            if value == 0 || value <= budget {
                break;
            }
            amount = ((i64::from(budget) * i64::from(amount)) / i64::from(value)) as i32;
        }
        let money = r32(&s.trade.convoy, 0x20).wrapping_sub(value);
        w32(&mut s.trade.convoy, 0x20, money);
        let total = r32(&s.trade.merchant, 0x4b4).wrapping_add(value);
        w32(&mut s.trade.merchant, 0x4b4, total);
        remaining = remaining.wrapping_sub(amount / 200);
        quantity = amount;
        cost = value;
        let mut left = amount;
        let mut left_cost = value;
        for id in chain(&s.trade)? {
            if left <= 0 {
                break;
            }
            if r32(&s.trade.ships[id], 0x18) <= 0 {
                continue;
            }
            let available = (h.free_capacity(&mut s.trade.ships[id])? / lot).wrapping_mul(lot);
            if available >= left {
                h.add_cargo(&mut s.trade.ships[id], id as u16, ware as u8, left, left_cost)?;
                left = 0
            } else if available > 0 {
                let share =
                    ((i64::from(available) * i64::from(left_cost) + i64::from(left - 1)) / i64::from(left)) as i32;
                h.add_cargo(&mut s.trade.ships[id], id as u16, ware as u8, available, share)?;
                left -= available;
                left_cost = left_cost.wrapping_sub(share)
            }
        }
    }
    let report = if quantity != 0 {
        let r = Report {
            slot: slot as u8,
            direction,
            quantity,
            cost,
            expiry,
            completed: remaining == 0,
        };
        h.report(&r)?;
        Some(r)
    } else {
        None
    };
    if remaining == 0 && quantity != 0 {
        s.contracts.remove(index);
        let n = r32(&s.trade.center, 0x28).wrapping_sub(1);
        w32(&mut s.trade.center, 0x28, n);
        let n = r16(&s.trade.center, 0x1c).wrapping_sub(1);
        s.trade.center[0x1c..0x1e].copy_from_slice(&n.to_le_bytes())
    } else {
        w32(&mut contract, 4, remaining);
        s.contracts[index] = contract
    }
    Ok(Prepared {
        state: s,
        host: h,
        report,
    })
}
