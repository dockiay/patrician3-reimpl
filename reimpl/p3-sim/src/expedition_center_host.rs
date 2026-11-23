//! Adaptador transaccional para visitar un centro desde 0x12.
//! Los textos de contratos y comercio aún se rechazan; nunca se descartan.
use crate::{
    expedition_visit::{CenterHost, CenterOutcome, VisitState},
    trade_center_text::{self, TradeReportTexts},
    trade_contract_text::{self, ContractTexts},
};
use p3_core::{
    calendar::Calendar,
    economy::{
        prices::SMALL_LOT_WARES,
        ship_trade::ShipCargo,
        trade_center_contract::{self, ContractState},
        trade_center_convoy::{self, Request, State},
        trade_center_discovery::Effect,
        trade_center_prices::Tables,
    },
};

#[derive(Clone)]
pub struct PriceTables {
    pub base: [f32; 24],
    pub units: [u8; 24],
    pub sell_caps: [i32; 4],
    pub buy_caps: [i32; 4],
}
#[derive(Clone)]
pub struct NativeCenterHost {
    pub convoy: u16,
    pub convoys: Vec<u8>,
    pub ships: Vec<Vec<u8>>,
    pub captains: Vec<u8>,
    pub town_count: u32,
    pub tick: u32,
    pub player: u32,
    pub hotseat: bool,
    pub date: Calendar,
    pub renown_small: i16,
    pub tables: PriceTables,
    pub report_texts: Option<TradeReportTexts>,
    /// Texto de ventas/compras acumulado para la carta 0x63.
    pub trade_text: Vec<u8>,
    pub contract_texts: Option<ContractTexts>,
    pub contract_text: Vec<u8>,
    pub populations: Vec<i32>,
}
#[derive(Clone)]
struct TradeHost {
    contracts: Vec<[u8; 16]>,
    units: [u8; 24],
    convoy: u16,
    dirty: bool,
    report_texts: Option<TradeReportTexts>,
    trade_text: Vec<u8>,
    contract_texts: Option<ContractTexts>,
    contract_text: Vec<u8>,
    populations: Vec<i32>,
    tick: u32,
    base_prices: [f32; 24],
    contract_context: Option<(Vec<u8>, i32, i32)>,
}
impl TradeHost {
    fn capacity(&mut self, ship: &mut [u8]) -> Result<i32, String> {
        if ship.len() != 0x180 {
            return Err("barco comercial truncado".into());
        }
        Ok(p3_core::ships::cargo::recompute_load(ship))
    }
    fn add(&mut self, ship: &mut [u8], ware: u8, amount: i32, cost: i32) -> Result<(), String> {
        let w = usize::from(ware);
        if ship.len() != 0x180 || w >= 24 {
            return Err("carga comercial inválida".into());
        }
        if amount <= 0 {
            return Ok(());
        }
        if (self.units[w] != 0) != SMALL_LOT_WARES[w] {
            return Err("tabla de lotes incompatible con carga naval".into());
        }
        let cv = u16::from_le_bytes([ship[8], ship[9]]);
        if cv != self.convoy {
            return Err("barco comercial en otro convoy".into());
        }
        let stock_at = 0x54 + 4 * w;
        let price_at = 0xb4 + 4 * w;
        let mut cargo = ShipCargo::default();
        cargo.stock[w] = i32::from_le_bytes(ship[stock_at..stock_at + 4].try_into().unwrap());
        cargo.average_price[w] = f32::from_le_bytes(ship[price_at..price_at + 4].try_into().unwrap());
        cargo.add_with_cost(w, amount, cost);
        // 0x517FE0 persiste el total truncado al lote, no solo lo usa para el precio.
        let lot = if w >= 20 {
            10
        } else if SMALL_LOT_WARES[w] {
            200
        } else {
            2000
        };
        cargo.stock[w] = (cargo.stock[w] / lot).wrapping_mul(lot);
        ship[stock_at..stock_at + 4].copy_from_slice(&cargo.stock[w].to_le_bytes());
        ship[price_at..price_at + 4].copy_from_slice(&cargo.average_price[w].to_le_bytes());
        let flags = u16::from_le_bytes([ship[0x3c], ship[0x3d]]) | 4;
        ship[0x3c..0x3e].copy_from_slice(&flags.to_le_bytes());
        self.dirty = true;
        Ok(())
    }
}
impl trade_center_contract::Host for TradeHost {
    fn free_capacity(&mut self, s: &mut [u8]) -> Result<i32, String> {
        self.capacity(s)
    }
    fn add_cargo(&mut self, s: &mut [u8], _: u16, w: u8, a: i32, c: i32) -> Result<(), String> {
        self.add(s, w, a, c)
    }
    fn report(&mut self, report: &trade_center_contract::Report) -> Result<(), String> {
        let texts = self
            .contract_texts
            .as_ref()
            .ok_or("texto de contrato comercial pendiente")?;
        let (center, price, remaining) = self.contract_context.as_ref().ok_or("contexto de contrato ausente")?;
        self.contract_text = trade_contract_text::append_existing(
            texts,
            center,
            report,
            *price,
            remaining.wrapping_sub(report.quantity / 200),
            &self.contract_text,
        )?;
        Ok(())
    }
}
impl trade_center_convoy::Host for TradeHost {
    fn contract(&mut self, state: &mut State, req: Request, before_sales: bool) -> Result<bool, String> {
        let input = ContractState {
            trade: state.clone(),
            contracts: self.contracts.clone(),
        };
        let owner = u16::from(input.trade.convoy[0]);
        if let Some(contract) = input
            .contracts
            .iter()
            .find(|c| u16::from_le_bytes([c[0], c[1]]) == owner)
        {
            self.contract_context = Some((
                input.trade.center.clone(),
                i32::from_le_bytes(contract[8..12].try_into().unwrap()),
                i32::from_le_bytes(contract[4..8].try_into().unwrap()),
            ));
        } else {
            self.contract_context = None;
            if !before_sales {
                if let Some(created) =
                    trade_center_contract::create(&input, req, self.tick, &self.populations, &self.base_prices)?
                {
                    let texts = self
                        .contract_texts
                        .as_ref()
                        .ok_or("texto de contrato comercial pendiente")?;
                    self.contract_text = trade_contract_text::append_created(
                        texts,
                        &input.trade.center,
                        &created.contract,
                        &self.contract_text,
                    )?;
                    *state = created.state.trade;
                    self.contracts = created.state.contracts;
                    return Ok(true);
                }
            }
        }
        let out = trade_center_contract::prepare(&input, self, req, before_sales, &self.units)?;
        *state = out.state.trade;
        *self = out.host;
        self.contracts = out.state.contracts;
        Ok(false)
    }
    fn free_capacity(&mut self, s: &mut [u8]) -> Result<i32, String> {
        self.capacity(s)
    }
    fn add_cargo(&mut self, s: &mut [u8], _: u16, w: u8, a: i32, c: i32) -> Result<(), String> {
        self.add(s, w, a, c)
    }
    fn report(&mut self, state: &State, req: Request, report: &trade_center_convoy::Report) -> Result<(), String> {
        let texts = self
            .report_texts
            .as_ref()
            .ok_or("texto de comercio de expedicion pendiente")?;
        self.trade_text = trade_center_text::append_report(
            texts,
            &self.units,
            &state.center,
            req.waypoint,
            report,
            &self.trade_text,
        )?;
        Ok(())
    }
}
impl CenterHost for NativeCenterHost {
    fn visit(
        &mut self,
        state: &mut VisitState,
        center: usize,
        waypoint: usize,
        mode: u16,
        allow_contract: bool,
    ) -> Result<CenterOutcome, String> {
        let mut s = state.clone();
        let mut host = self.clone();
        if host.populations.len() != host.town_count as usize {
            return Err("poblaciones de contrato incompletas".into());
        }
        let old = s.centers.get(center).ok_or("centro ausente")?.clone();
        let combined = host.ships.concat();
        let discovery = p3_state::trade_center_discovery::prepare(
            &old,
            host.convoy,
            &host.convoys,
            &combined,
            &host.captains,
            &s.merchants,
            &s.message_pool,
            host.town_count,
            host.tick,
            host.player,
            host.hotseat,
            (host.date.day, host.date.month, host.date.year),
            host.renown_small,
        )?;
        let discovered = discovery.is_some();
        if let Some(p) = discovery {
            s.centers[center] = p.center;
            s.merchants = p.merchants;
            s.message_pool = p.message_pool;
            for e in p.presentation {
                match e {
                    Effect::Notify(args) => s.notifications.push(args.map(u32::from)),
                    Effect::Window { kind, owner } => s.windows.push([kind, u32::from(owner), 0]),
                    _ => return Err("presentación de descubrimiento inválida".into()),
                }
            }
        }
        let cv_at = usize::from(host.convoy) * 0x3c;
        let cv = host
            .convoys
            .get(cv_at..cv_at + 0x3c)
            .ok_or("convoy de centro ausente")?
            .to_vec();
        let owner = usize::from(cv[0]);
        let person = owner;
        if person >= s.merchants.len() {
            return Err("dueño de convoy comercial ausente".into());
        }
        let merchant = s
            .merchants
            .get(person)
            .ok_or("comerciante de centro ausente")?
            .bytes
            .clone();
        let stored = &s.centers[center];
        let n = usize::from(u16::from_le_bytes([stored[28], stored[29]]));
        let contracts = stored[30..]
            .chunks_exact(16)
            .map(|v| v.try_into().expect("contrato validado"))
            .collect::<Vec<[u8; 16]>>();
        if contracts.len() != n {
            return Err("contratos de centro truncados".into());
        }
        let mut image = vec![0; 44];
        image[..30].copy_from_slice(&stored[..30]);
        image[0x28..0x2c].copy_from_slice(&(n as u32).to_le_bytes());
        let trade = State {
            center: image,
            convoy: cv,
            ships: host.ships.clone(),
            merchant,
        };
        let adapter = TradeHost {
            contracts,
            units: host.tables.units,
            convoy: host.convoy,
            dirty: false,
            report_texts: host.report_texts.clone(),
            trade_text: host.trade_text.clone(),
            contract_texts: host.contract_texts.clone(),
            contract_text: host.contract_text.clone(),
            populations: host.populations.clone(),
            tick: host.tick,
            base_prices: host.tables.base,
            contract_context: None,
        };
        let t = &host.tables;
        let tables = Tables {
            base: &t.base,
            units: &t.units,
            sell_caps: &t.sell_caps,
            buy_caps: &t.buy_caps,
        };
        let req = Request {
            convoy: host.convoy,
            waypoint: u16::try_from(waypoint).map_err(|_| "waypoint excesivo")?,
            mode,
            allow_contract,
        };
        let output = trade_center_convoy::prepare(&trade, &adapter, req, &tables)?;
        let mut cv = output.state.convoy;
        if output.host.dirty {
            let flags = u32::from_le_bytes(cv[0x18..0x1c].try_into().unwrap()) | 4;
            cv[0x18..0x1c].copy_from_slice(&flags.to_le_bytes())
        }
        host.convoys[cv_at..cv_at + 0x3c].copy_from_slice(&cv);
        host.ships = output.state.ships;
        host.trade_text = output.host.trade_text.clone();
        host.contract_text = output.host.contract_text.clone();
        s.merchants[person].bytes = output.state.merchant;
        let mut serialized = output.state.center[..30].to_vec();
        for c in output.host.contracts {
            serialized.extend_from_slice(&c)
        }
        s.centers[center] = serialized;
        *state = s;
        *self = host;
        Ok(CenterOutcome {
            discovered,
            contract_created: output.contract_created,
        })
    }
}

#[cfg(test)]
mod cargo_x86_tests {
    use super::*;
    fn unhex(s: &str) -> Vec<u8> {
        s.as_bytes()
            .chunks_exact(2)
            .map(|p| u8::from_str_radix(std::str::from_utf8(p).unwrap(), 16).unwrap())
            .collect()
    }
    #[test]
    fn cargo_and_convoy_flag_match_original() {
        let v: serde_json::Value =
            serde_json::from_str(include_str!("../tests/fixtures/c10_trade_cargo_host.json")).unwrap();
        for c in v["cases"].as_array().unwrap() {
            let w = c["ware"].as_u64().unwrap() as usize;
            let stock = c["stock"].as_i64().unwrap() as i32;
            let amount = c["amount"].as_i64().unwrap() as i32;
            let mut ship = vec![0; 0x180];
            ship[0x54 + 4 * w..0x58 + 4 * w].copy_from_slice(&stock.to_le_bytes());
            ship[0xb4 + 4 * w..0xb8 + 4 * w].copy_from_slice(&21.25f32.to_le_bytes());
            let mut host = TradeHost {
                contracts: vec![],
                units: SMALL_LOT_WARES.map(u8::from),
                convoy: 0,
                dirty: false,
                report_texts: None,
                trade_text: vec![],
                contract_texts: None,
                contract_text: vec![],
                populations: vec![],
                tick: 0,
                base_prices: [0.0; 24],
                contract_context: None,
            };
            host.add(&mut ship, w as u8, amount, 1234).unwrap();
            let mut convoy = vec![0; 0x3c];
            if host.dirty {
                convoy[0x18] |= 4;
            }
            assert_eq!(
                ship,
                unhex(c["ship"].as_str().unwrap()),
                "ware {w}, stock {stock}, amount {amount}"
            );
            assert_eq!(convoy, unhex(c["convoy"].as_str().unwrap()), "ware {w}");
        }
    }
}
