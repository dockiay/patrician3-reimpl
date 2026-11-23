//! Conexión portable de ciudad, oficinas, impuestos y reputación.
//!
//! El reloj y el resto de sistemas siguen perteneciendo al anfitrión: este módulo ofrece las
//! operaciones que `TickSystems::close_day` y `TickSystems::town_daily` pueden llamar en su fase.
use super::ship_trade::{self, ConvoyTrade, ShipCargo};
use super::taxes::{self, Accounts};
use super::town_tick;
use super::trade::{self, OfficeStock, TradeSide};
use super::world::{EconomyWorld, TownHost};
use crate::ai::reputation::Merchant;

pub struct EconomySystem {
    pub world: EconomyWorld,
    pub accounts: Accounts,
    /// Mismos índices que `world.merchants`; contiene dinero y reputación originales.
    pub merchants: Vec<Merchant>,
    /// Mismos índices que `world.offices` y `accounts.offices`.
    pub offices: Vec<OfficeStock>,
    /// Mismos índices que `world.ships`; estado de carga del barco.
    pub cargo_ships: Vec<ShipCargo>,
}

impl EconomySystem {
    fn merchant_state_to_world(&mut self) {
        assert_eq!(self.world.merchants.len(), self.merchants.len());
        for (dst, src) in self.world.merchants.iter_mut().zip(&self.merchants) {
            dst.gold = src.i32(0);
            dst.income = src.i32(0x4c8);
        }
    }
    fn world_to_merchant_state(&mut self) {
        for (dst, src) in self.merchants.iter_mut().zip(&self.world.merchants) {
            dst.set_i32(0, src.gold);
            dst.set_i32(0x4c8, src.income);
        }
    }
    fn office_weapons_to_world(&mut self) {
        assert_eq!(self.world.offices.len(), self.offices.len());
        for (dst, src) in self.world.offices.iter_mut().zip(&self.offices) {
            dst.weapons.copy_from_slice(&src.stock[20..24]);
        }
    }
    fn world_to_office_weapons(&mut self) {
        for (dst, src) in self.offices.iter_mut().zip(&self.world.offices) {
            dst.stock[20..24].copy_from_slice(&src.weapons);
        }
    }
    /// Corresponde a la fase diaria de C01 tras actualizar el calendario.
    pub fn close_day(&mut self, day_of_month: u8, week_start: bool) {
        assert_eq!(self.accounts.offices.len(), self.world.offices.len());
        assert_eq!(self.accounts.merchants.len(), self.world.merchants.len());
        self.merchant_state_to_world();
        taxes::charge_daily_with_reputation(
            &mut self.world,
            &mut self.accounts,
            &mut self.merchants,
            day_of_month,
            week_start,
        );
    }
    /// Tick de una ciudad. Los métodos de `host` siguen representando los sistemas externos.
    pub fn town_daily(&mut self, town: usize, argument: u32, host: &mut dyn TownHost) {
        self.merchant_state_to_world();
        self.office_weapons_to_world();
        town_tick::handle_town_tick(&mut self.world, town, argument, host);
        self.world_to_merchant_state();
        self.world_to_office_weapons();
    }
    /// Opera con un barco existente y sincroniza el dinero del dueño con el mundo de ciudades.
    pub fn trade_ship(
        &mut self,
        ship: usize,
        town: usize,
        side: TradeSide,
        ware: usize,
        quantity: &mut i32,
        difficulty: f32,
        convoy_dirty: Option<&mut u32>,
    ) {
        let owner = self.cargo_ships[ship].owner as usize;
        assert_eq!(self.world.ships[ship].owner as usize, owner);
        ship_trade::trade_ship(
            &mut self.world.towns,
            &mut self.cargo_ships[ship],
            &mut self.merchants[owner],
            town,
            ware,
            quantity,
            side,
            difficulty,
            convoy_dirty,
        );
        self.world.merchants[owner].gold = self.merchants[owner].i32(0);
    }
    /// Comercio agregado de un convoy existente. El anfitrión valida su cadena e índices.
    pub fn trade_convoy(
        &mut self,
        convoy: &mut ConvoyTrade,
        next: &[u16],
        owner: usize,
        town: usize,
        ware: usize,
        amount: i32,
        side: TradeSide,
        difficulty: f32,
    ) {
        ship_trade::trade_convoy(
            &mut self.world.towns,
            &mut self.cargo_ships,
            next,
            convoy,
            &mut self.merchants[owner],
            owner as u16,
            town,
            ware,
            amount,
            side,
            difficulty,
        );
        self.world.merchants[owner].gold = self.merchants[owner].i32(0);
    }
    /// Sueldos de un negocio de la oficina. El anfitrión suministra sus campos y la ciudad.
    pub fn pay_business_wages(
        &mut self,
        office: usize,
        kind: usize,
        employees: u16,
        auxiliary: u16,
        workers: &mut super::office::OfficeWorkers,
    ) {
        let owner = self.offices[office].owner as usize;
        let town = self.offices[office].town as usize;
        assert_eq!(self.offices[office].owner, self.accounts.offices[office].owner);
        assert_eq!(self.offices[office].town, self.accounts.offices[office].town);
        super::office::pay_wages(
            kind,
            employees,
            auxiliary,
            self.world.towns[town].u32(0x2cc),
            &mut self.merchants[owner],
            &mut self.accounts.offices[office],
            workers,
        );
    }
    /// Compra de insumos activada por la oficina; el llamante decide cuándo la activa el tick.
    pub fn buy_business_inputs(&mut self, office: usize) {
        let owner = self.offices[office].owner as usize;
        assert_eq!(self.offices[office].owner, self.accounts.offices[office].owner);
        assert_eq!(self.offices[office].town, self.accounts.offices[office].town);
        trade::buy_business_inputs(
            &mut self.world.towns,
            &mut self.offices[office],
            &mut self.accounts.offices[office],
            &mut self.merchants[owner],
        );
        self.world.merchants[owner].gold = self.merchants[owner].i32(0);
        self.world.offices[office]
            .weapons
            .copy_from_slice(&self.offices[office].stock[20..24]);
    }
    /// Comercio de una oficina controlada por la IA; el anfitrión decide su fase.
    pub fn ai_office_trade(&mut self, office: usize, difficulty: f32) {
        let owner = self.offices[office].owner as usize;
        assert_eq!(self.offices[office].owner, self.accounts.offices[office].owner);
        assert_eq!(self.offices[office].town, self.accounts.offices[office].town);
        trade::ai_office_trade(
            &mut self.world.towns,
            &mut self.offices[office],
            &mut self.accounts.offices[office],
            &mut self.merchants[owner],
            difficulty,
        );
        self.world.merchants[owner].gold = self.merchants[owner].i32(0);
        self.world.offices[office]
            .weapons
            .copy_from_slice(&self.offices[office].stock[20..24]);
    }
    /// Ejecuta la rama comercial del tick de oficina en su punto original.
    pub fn office_commerce_phase(&mut self, office: usize, difficulty: f32) {
        let owner = self.offices[office].owner as usize;
        let decision = trade::office_commerce_phase(
            self.offices[office].automation_flags,
            self.world.merchants[owner].kind == 0,
        );
        self.offices[office].automation_flags = decision.flags;
        if decision.ai {
            self.ai_office_trade(office, difficulty);
        }
        if decision.inputs {
            self.buy_business_inputs(office);
        }
        if decision.administrator {
            trade::administrator_trade(
                &mut self.world.towns,
                &mut self.offices[office],
                &mut self.accounts.offices[office],
                &mut self.merchants[owner],
                difficulty,
            );
            self.world.merchants[owner].gold = self.merchants[owner].i32(0);
            self.world.offices[office]
                .weapons
                .copy_from_slice(&self.offices[office].stock[20..24]);
        }
    }
    /// Ejecuta compraventa de una oficina existente, con la reputación del comerciante.
    pub fn trade_office(&mut self, office: usize, side: TradeSide, ware: usize, quantity: &mut i32, difficulty: f32) {
        let owner = self.offices[office].owner as usize;
        assert_eq!(self.offices[office].owner, self.accounts.offices[office].owner);
        assert_eq!(self.offices[office].town, self.accounts.offices[office].town);
        trade::office_trade_with_reputation(
            &mut self.world.towns,
            &mut self.offices[office],
            &mut self.merchants[owner],
            side,
            ware,
            quantity,
            difficulty,
        );
        self.world.merchants[owner].gold = self.merchants[owner].i32(0);
        self.world.offices[office]
            .weapons
            .copy_from_slice(&self.offices[office].stock[20..24]);
    }
}

/// Enlaza el reloj C01 con las fases de impuestos y ciudad de `EconomySystem`.
/// El resto de fases se delegan a `other`. El anfitrión debe coordinar las modificaciones de sus
/// demás sistemas sobre el mismo estado; esta frontera no conecta por sí sola barcos u operaciones.
pub struct EconomyTickBridge<'a, S: crate::tick::TickSystems> {
    pub economy: &'a mut EconomySystem,
    pub town_host: &'a mut dyn TownHost,
    pub other: &'a mut S,
}

impl<S: crate::tick::TickSystems> EconomyTickBridge<'_, S> {
    fn sync_clock(&mut self) {
        let clock = *self.other.clock();
        self.economy.world.tick = clock.tick;
        self.economy.world.day_of_month = clock.calendar.day;
        self.economy.world.month = clock.calendar.month;
        self.economy.world.day_of_year = clock.calendar.day_of_year;
    }
}

impl<S: crate::tick::TickSystems> crate::tick::TickSystems for EconomyTickBridge<'_, S> {
    fn clock(&mut self) -> &mut crate::tick::WorldClock {
        self.other.clock()
    }
    fn time_token(&self) -> u32 {
        self.other.time_token()
    }
    fn close_day(&mut self, week: bool) {
        self.sync_clock();
        let day = self.other.clock().calendar.day;
        self.town_host.before_day_close(
            &self.economy.world,
            &mut self.economy.accounts,
            &mut self.economy.merchants,
        );
        self.economy.close_day(day, week);
        self.town_host
            .after_day_close(&self.economy.accounts, &self.economy.merchants);
    }
    fn town_daily(&mut self, town: u32, day_of_year: u16) {
        self.sync_clock();
        self.economy
            .town_daily(town as usize, day_of_year as u32, self.town_host);
    }
    fn statistics_daily(&mut self) {
        self.other.statistics_daily();
    }
    fn town_history_daily(&mut self, tick: u32) {
        self.other.town_history_daily(tick);
    }
    fn merchant_daily(&mut self, merchant: u32) {
        self.other.merchant_daily(merchant);
    }
    fn trade_center_daily(&mut self, center: u32) {
        self.other.trade_center_daily(center);
    }
    fn siege_daily(&mut self, town: u32) {
        self.other.siege_daily(town);
    }
    fn ships_tick(&mut self, tick: u32) {
        self.town_host.ships_tick_with_offices(
            &mut self.economy.world,
            &mut self.economy.merchants,
            &mut self.economy.cargo_ships,
            &mut self.economy.offices,
            &mut self.economy.accounts.offices,
            tick,
        );
        self.other.ships_tick(tick);
    }
    fn scheduled_tasks_tick(&mut self, tick: u32) {
        self.town_host.dispatch_scheduled_tasks_fleet(
            &mut self.economy.world,
            &mut self.economy.merchants,
            &[],
            &mut self.economy.offices,
            &mut self.economy.cargo_ships,
            2.0,
            tick,
        );
        self.other.scheduled_tasks_tick(tick);
    }
    fn harbor_scene_sync(&mut self) {
        self.other.harbor_scene_sync();
    }
    fn sea_battles_tick(&mut self) {
        self.other.sea_battles_tick();
    }
    fn siege_scene(&mut self, town: u32) {
        self.other.siege_scene(town);
    }
    fn merchant_gold(&self, merchant: u32) -> i32 {
        self.economy.merchants[merchant as usize].i32(0)
    }
    fn set_merchant_gold(&mut self, merchant: u32, gold: i32) {
        let index = merchant as usize;
        self.economy.merchants[index].set_i32(0, gold);
        self.economy.world.merchants[index].gold = gold;
    }
}
