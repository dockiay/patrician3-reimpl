//! 0x52BFE0/0x52C019: flujo comercial preparado; contratos y texto son callbacks.
use super::trade_center_prices::{self, Tables};
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct State {
    pub center: Vec<u8>,
    pub convoy: Vec<u8>,
    pub ships: Vec<Vec<u8>>,
    pub merchant: Vec<u8>,
}
#[derive(Clone, Copy, Debug)]
pub struct Request {
    pub convoy: u16,
    pub waypoint: u16,
    pub mode: u16,
    pub allow_contract: bool,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Report {
    pub sold: [i32; 2],
    pub revenue: [i32; 2],
    pub bought: [i32; 2],
    pub cost: [i32; 2],
    pub last_ware: Option<u8>,
}
pub trait Host: Clone {
    fn contract(&mut self, state: &mut State, request: Request, before_sales: bool) -> Result<bool, String>;
    fn free_capacity(&mut self, ship: &mut [u8]) -> Result<i32, String>;
    fn add_cargo(&mut self, ship: &mut [u8], index: u16, ware: u8, amount: i32, cost: i32) -> Result<(), String>;
    fn report(&mut self, state: &State, request: Request, report: &Report) -> Result<(), String>;
}
#[derive(Clone)]
pub struct Prepared<H> {
    pub state: State,
    pub host: H,
    pub contract_created: bool,
    pub report: Report,
}
fn r16(b: &[u8], p: usize) -> u16 {
    u16::from_le_bytes([b[p], b[p + 1]])
}
fn r32(b: &[u8], p: usize) -> i32 {
    i32::from_le_bytes(b[p..p + 4].try_into().unwrap())
}
fn w32(b: &mut [u8], p: usize, v: i32) {
    b[p..p + 4].copy_from_slice(&v.to_le_bytes())
}
fn chain(state: &State) -> Result<Vec<usize>, String> {
    let mut out = vec![];
    let mut seen = vec![false; state.ships.len()];
    let mut cur = r16(&state.convoy, 0xa);
    while usize::from(cur) < state.ships.len() {
        let i = usize::from(cur);
        if seen[i] {
            return Err("ciclo de barcos de expedición".into());
        }
        seen[i] = true;
        let s = &state.ships[i];
        if s.len() != 0x180 {
            return Err("barco de expedición truncado".into());
        }
        out.push(i);
        cur = r16(s, 6)
    }
    Ok(out)
}
fn valid(s: &State) -> Result<(), String> {
    if s.center.len() != 44 || s.convoy.len() != 0x3c || s.merchant.len() != 0x650 {
        return Err("imágenes comerciales incompatibles".into());
    }
    Ok(())
}
/// Ejecuta sobre copias; una dependencia no conectada debe devolver Err y no
/// publicar ni el estado ni el anfitrión resultante. `tables` son datos reales
/// del EXE cargados por el llamante; no se contienen recursos comerciales.
pub fn prepare<H: Host>(before: &State, host: &H, req: Request, tables: &Tables<'_>) -> Result<Prepared<H>, String> {
    valid(before)?;
    if req.waypoint >= 4 {
        return Err("punto de expedición inválido".into());
    }
    let mut s = before.clone();
    let mut h = host.clone();
    let mut report = Report {
        sold: [0; 2],
        revenue: [0; 2],
        bought: [0; 2],
        cost: [0; 2],
        last_ware: None,
    };
    if s.center[4] >= 0x7f {
        return Ok(Prepared {
            state: s,
            host: h,
            contract_created: false,
            report,
        });
    }
    h.contract(&mut s, req, true)?;
    valid(&s)?;
    for id in chain(&s)? {
        if r32(&s.ships[id], 0x18) <= 0 {
            continue;
        }
        for slot in 0..2 {
            let ware = usize::from(s.center[2 + slot]);
            if ware >= 24 {
                return Err("mercancía deseada inválida".into());
            }
            let at = 0x54 + ware * 4;
            let amount = r32(&s.ships[id], at);
            if amount <= 0 {
                w32(&mut s.ships[id], at, 0);
                continue;
            }
            let mut left = amount;
            let price =
                trade_center_prices::sell(&s.center, slot, &mut left, req.mode == 1, tables).map_err(str::to_string)?;
            w32(&mut s.ships[id], at, left);
            let money = r32(&s.convoy, 0x20).wrapping_add(price);
            w32(&mut s.convoy, 0x20, money);
            let total = r32(&s.merchant, 0x4b0).wrapping_add(price);
            w32(&mut s.merchant, 0x4b0, total);
            report.sold[slot] = report.sold[slot].wrapping_add(amount.wrapping_sub(left));
            report.revenue[slot] = report.revenue[slot].wrapping_add(price)
        }
        h.free_capacity(&mut s.ships[id])?;
    }
    let created = h.contract(&mut s, req, false)?;
    valid(&s)?;
    let mut free = 0i32;
    for id in chain(&s)? {
        if r32(&s.ships[id], 0x18) > 0 {
            free = free.wrapping_add((h.free_capacity(&mut s.ships[id])? / 200).wrapping_mul(200))
        }
    }
    let flags = s.convoy[0x34 + usize::from(req.waypoint)];
    for pass in 0..3 {
        if free < 200 {
            break;
        }
        let slot = pass & 1;
        if flags & (1 << slot) == 0 {
            continue;
        }
        let ware = usize::from(s.center[slot]);
        if ware >= 24 {
            continue;
        }
        report.last_ware = Some(ware as u8);
        let mut amount = if pass == 0 && flags & 2 != 0 { free / 2 } else { free };
        amount = amount.min(r32(&s.center, 0xc + slot * 4));
        let lot = if tables.units[ware] == 0 { 2000 } else { 200 };
        let mut cost;
        let mut attempts = 0;
        loop {
            attempts += 1;
            if attempts > 128 {
                return Err("ajuste de presupuesto comercial no converge".into());
            }
            amount = (amount / lot).wrapping_mul(lot);
            cost = trade_center_prices::buy(&s.center, slot, amount, req.mode == 1, tables).map_err(str::to_string)?;
            let budget = r32(&s.convoy, 0x20);
            if cost == 0 || cost <= budget {
                break;
            }
            amount = ((i64::from(budget) * i64::from(amount)) / i64::from(cost)) as i32;
        }
        let budget = r32(&s.convoy, 0x20).wrapping_sub(cost);
        w32(&mut s.convoy, 0x20, budget);
        let total = r32(&s.merchant, 0x4b4).wrapping_add(cost);
        w32(&mut s.merchant, 0x4b4, total);
        let stock = r32(&s.center, 0xc + slot * 4).wrapping_sub(amount);
        w32(&mut s.center, 0xc + slot * 4, stock);
        report.bought[slot] = report.bought[slot].wrapping_add(amount);
        report.cost[slot] = report.cost[slot].wrapping_add(cost);
        let mut remaining = amount;
        let mut remaining_cost = cost;
        for id in chain(&s)? {
            if remaining <= 0 {
                break;
            }
            if r32(&s.ships[id], 0x18) <= 0 {
                continue;
            }
            let available = (h.free_capacity(&mut s.ships[id])? / lot).wrapping_mul(lot);
            if available >= remaining {
                h.add_cargo(&mut s.ships[id], id as u16, ware as u8, remaining, remaining_cost)?;
                free = free.wrapping_sub(remaining);
                remaining = 0
            } else if available > 0 {
                let share = ((i64::from(available) * i64::from(remaining_cost) + i64::from(remaining - 1))
                    / i64::from(remaining)) as i32;
                h.add_cargo(&mut s.ships[id], id as u16, ware as u8, available, share)?;
                remaining_cost = remaining_cost.wrapping_sub(share);
                free = free.wrapping_sub(available);
                remaining = remaining.wrapping_sub(available)
            }
        }
    }
    for id in chain(&s)? {
        if r32(&s.ships[id], 0x18) > 0 {
            h.free_capacity(&mut s.ships[id])?;
        }
    }
    if report.sold.iter().chain(report.bought.iter()).any(|&v| v != 0) {
        h.report(&s, req, &report)?
    }
    Ok(Prepared {
        state: s,
        host: h,
        contract_created: created,
        report,
    })
}
