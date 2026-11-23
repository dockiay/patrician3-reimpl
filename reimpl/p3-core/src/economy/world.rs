//! Estado global que lee el tick diario de ciudad y frontera con los sistemas aún no reimplementados.

use crate::town::Town;

/// Huecos de mano de obra (`0x006DD628`, 0x100 bytes): instalación y trabajadores que faltan.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LabourShortage {
    pub count: u32,
    /// Tipo de instalación de la ciudad, o `-1 - índice` para los negocios de comerciantes.
    pub types: [u16; 63],
    pub amounts: [u16; 63],
}

impl Default for LabourShortage {
    fn default() -> Self {
        LabourShortage {
            count: 0,
            types: [0; 63],
            amounts: [0; 63],
        }
    }
}

/// Campos del comerciante que usa la ciudad (registro de 0x650 bytes).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Merchant {
    /// `+0`: dinero.
    pub gold: i32,
    /// `+8`: 0 = jugador humano.
    pub kind: u16,
    /// `+0x19`: ciudad natal.
    pub home_town: u8,
    /// `+0x4C8`: ingresos acumulados.
    pub income: i32,
}

/// Campos de la oficina que usa la ciudad (registro de 0x44C bytes).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Office {
    /// `+0x54`: existencias de armas (mercancías 20–23).
    pub weapons: [i32; 4],
    /// `+0x2CA`: siguiente oficina de la ciudad.
    pub next: u16,
    /// `+0x2CC`: primer negocio del comerciante en la ciudad.
    pub first_facility: u16,
}

/// Negocio de un comerciante (registro de 20 bytes).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MerchantFacility {
    /// `+4`
    pub employees: u16,
    /// `+7`
    pub town: u8,
    /// `+8`: siguiente negocio de la misma oficina.
    pub next: u16,
}

/// Campos del barco que usa la ciudad (registro de 0x180 bytes).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Ship {
    /// `+0`: comerciante dueño.
    pub owner: u8,
    /// `+0x134`: estado.
    pub state: u16,
}

/// Entrada de suministro diario de la ciudad (lista enlazada en `Town+0x9B4`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TickEntry {
    /// `+4`: cantidad añadida hoy.
    pub amount: i32,
    /// `+8`: consumo mundial de la mercancía, sin el bit de signo.
    pub sum: i32,
    /// `+0xC`: factor máximo.
    pub max: u16,
    /// `+0xE`: factor actual (1024 = 100 %).
    pub factor: u16,
    /// `+0x10`: tipo de instalación (da la mercancía).
    pub facility_type: u8,
    /// `+0x11`: ciudad.
    pub town: u8,
}

/// Lo que el tick diario de ciudad lee y escribe fuera de la propia ciudad.
#[derive(Clone, Debug, Default)]
pub struct EconomyWorld {
    /// Tabla de ciudades (`0x006DE508`); su número es `0x006DE4B0`.
    pub towns: Vec<Town>,
    /// `0x006DE4B4`
    pub tick: u32,
    /// `GameWorld+1`
    pub month: u8,
    /// `GameWorld+0`, día del mes.
    pub day_of_month: u8,
    /// `GameWorld+4`
    pub day_of_year: u16,
    /// `0x006DD894`
    pub ship_count: u16,
    pub ships: Vec<Ship>,
    /// `GameWorld+0xA`
    pub merchant_count: u16,
    pub merchants: Vec<Merchant>,
    /// `GameWorld+8`
    pub office_count: u16,
    pub offices: Vec<Office>,
    /// `GameWorld+6`
    pub merchant_facility_count: u16,
    pub merchant_facilities: Vec<MerchantFacility>,
    pub labour: LabourShortage,
    /// Byte `+0x17` del objeto de opciones (`0x006CC3E8`); `None` sin partida.
    pub difficulty: Option<u8>,
    /// `0x006DF270` no nulo: hay asedios.
    pub sieges: bool,
    /// `GameWorld+0x8A`
    pub world_8a: u8,
    /// `GameWorld+0x8D/0x8E`, representante y ciudad de la Hansa.
    pub alderman_merchant: u8,
    pub alderman_town: u8,
    /// `0x006DF31C`
    pub network_client: u32,
    /// `0x006DFC14`: comerciante del jugador local.
    pub local_merchant: u32,
    /// Entradas de suministro de cada ciudad, en el orden de la lista.
    pub tick_entries: Vec<Vec<TickEntry>>,
}

/// Llamadas a sistemas que aún no están reimplementados (construcción, oficinas, astillero, asedio,
/// mensajes). Cada método corresponde a una llamada del original en el mismo punto.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScheduledOfficeUpdate {
    pub index: usize,
    pub administrator: u16,
    pub flags: u16,
    pub experience: Option<u8>,
    pub wage: Option<u16>,
    pub image: Option<Vec<u8>>,
}

pub trait TownHost {
    /// Interrupción solicitada por el anfitrión antes de la fase de batalla.
    fn interrupt_before_sea_battle(&self) -> bool {
        false
    }
    /// Paso naval en el punto del reloj original, con acceso al estado económico vivo.
    fn ships_tick_live(
        &mut self,
        _world: &mut EconomyWorld,
        _merchants: &mut [crate::ai::reputation::Merchant],
        _ships: &mut Vec<super::ship_trade::ShipCargo>,
        _tick: u32,
    ) -> bool {
        false
    }
    /// Variante con existencias de oficina vivas para los puentes que las poseen.
    fn ships_tick_with_offices(
        &mut self,
        world: &mut EconomyWorld,
        merchants: &mut [crate::ai::reputation::Merchant],
        ships: &mut Vec<super::ship_trade::ShipCargo>,
        _offices: &mut [super::trade::OfficeStock],
        _accounts: &mut [super::taxes::OfficeAccounts],
        tick: u32,
    ) -> bool {
        self.ships_tick_live(world, merchants, ships, tick)
    }
    /// Una imagen de oficinas completa es necesaria únicamente cuando se activa la IA naval.
    fn ship_ai_enabled(&self) -> bool {
        false
    }
    fn ships_tick_with_images(
        &mut self,
        world: &mut EconomyWorld,
        merchants: &mut [crate::ai::reputation::Merchant],
        ships: &mut Vec<super::ship_trade::ShipCargo>,
        _images: &mut [u8],
        tick: u32,
    ) -> bool {
        self.ships_tick_live(world, merchants, ships, tick)
    }
    /// Igual que `ships_tick_with_images`, con las imágenes en un `Vec` que el anfitrión puede intercambiar con su
    /// propia memoria en vez de copiarlas a la ida y a la vuelta. Si devuelve `false`, el contenido de `images`
    /// queda sin especificar (el llamante lo descarta).
    fn ships_tick_with_office_images(
        &mut self,
        world: &mut EconomyWorld,
        merchants: &mut [crate::ai::reputation::Merchant],
        ships: &mut Vec<super::ship_trade::ShipCargo>,
        images: &mut Vec<u8>,
        tick: u32,
    ) -> bool {
        self.ships_tick_with_images(world, merchants, ships, images.as_mut_slice(), tick)
    }
    fn ship_office_sync_error(&mut self, _error: &str) {}
    /// Sincroniza las copias de oficina y comerciante antes y después del cierre diario.
    fn before_day_close(
        &mut self,
        _world: &EconomyWorld,
        _accounts: &mut super::taxes::Accounts,
        _merchants: &mut [crate::ai::reputation::Merchant],
    ) {
    }
    fn after_day_close(&mut self, _accounts: &super::taxes::Accounts, _merchants: &[crate::ai::reputation::Merchant]) {}
    /// `office_update_business_employees` (0x004FFD40); devuelve el `u16` que suma la ciudad.
    fn office_update_business_employees(&mut self, world: &mut EconomyWorld, office: u16) -> u16;
    /// Estado calculado por el anfitrión tras el recuento, si está conectado.
    fn employment_office_count(&self, _office: u16) -> Option<u16> {
        None
    }
    fn employment_business(&self, _index: u16) -> Option<[u8; 20]> {
        None
    }
    /// Sincronización opcional de la transacción de obras del anfitrión.
    fn before_facility(&mut self, _world: &mut EconomyWorld, _town: usize, _facility: usize) {}
    fn after_facility(&mut self, _world: &mut EconomyWorld, _town: usize, _facility: usize) {}
    fn construction_prepare(&mut self, _world: &EconomyWorld) {}
    fn construction_business_efficiency(&mut self, _index: u16, _efficiency: i32) {}
    fn construction_assets(&self, _town: u8) -> Option<(crate::buildings::Site, super::facility_pool::FacilityPool)> {
        None
    }
    fn construction_commit(
        &mut self,
        _town: u8,
        _site: crate::buildings::Site,
        _pool: super::facility_pool::FacilityPool,
        _offices: &[Option<Vec<u8>>],
        _letters: &[(u32, u8, u16, u16)],
        _merchants: &mut [crate::ai::reputation::Merchant],
    ) -> Result<(), String> {
        Err("obras no conectadas".into())
    }
    fn construction_changed(&mut self) -> bool {
        false
    }
    /// `Office::daily_tick` (0x004FFF20).
    fn office_daily_tick(&mut self, world: &mut EconomyWorld, office: u16, arg: u32);
    /// Publica los enlaces tras rotar la lista diaria: todos los consumidores
    /// deben recorrer el mismo orden que `Town+0x784` y `Office+0x2CA`.
    fn office_links_changed(&mut self, _world: &EconomyWorld) {}
    /// `town_try_add_type_2f_building_by_employees` (0x00524920).
    fn try_add_type_2f_building(&mut self, town: &mut Town, employees: u32);
    /// `town_plan_count_street_cells` (0x004FA6F0), cada 16 días.
    fn count_street_cells(&mut self, town: &mut Town);
    /// `Town::recount_buildings` (0x00520BD0).
    fn recount_buildings(&mut self, town: &mut Town);
    /// Vista actual tras el recuento; el puente de oficina publica estos campos
    /// antes de que otras ciudades, rentas o estadísticas lean el registro.
    fn office_housing(&self, _office: usize) -> Option<crate::buildings::housing::OfficeHousing> {
        None
    }
    /// `Town::recount_defense_buildings` (0x00524C30), solo si la capacidad defensiva es 0.
    fn recount_defense_buildings(&mut self, town: &mut Town);
    /// `church_daily_update` (0x004FE877).
    fn church_daily_update(&mut self, town: &mut Town, town_index: u8);
    /// `town_update_tavern_state_daily` (0x00525F30).
    fn tavern_daily(&mut self, town: &mut Town);
    /// Variante con reloj y tabla de oficinas; el puerto anterior sigue disponible.
    fn tavern_daily_with_world(&mut self, _world: &EconomyWorld, town: &mut Town) {
        self.tavern_daily(town)
    }
    /// `town_ai_propose_council_offer` (0x0051E070).
    fn council_offer(&mut self, town: &mut Town);
    /// El planificador y el reloj están disponibles para anfitriones que los conecten.
    fn council_offer_with_world(&mut self, _world: &EconomyWorld, town: &mut Town) {
        self.council_offer(town)
    }
    /// Permite despachar tareas conectadas; las restantes deben conservarse.
    fn dispatch_scheduled_tasks(
        &mut self,
        _world: &mut EconomyWorld,
        _merchants: &mut [crate::ai::reputation::Merchant],
        _tick: u32,
    ) {
    }
    fn dispatch_scheduled_tasks_full(
        &mut self,
        world: &mut EconomyWorld,
        merchants: &mut [crate::ai::reputation::Merchant],
        _offices: &[super::extra_tax::TaxOffice],
        tick: u32,
    ) {
        self.dispatch_scheduled_tasks(world, merchants, tick)
    }
    /// Variante que permite actualizar existencias vivas de oficinas.
    fn dispatch_scheduled_tasks_assets(
        &mut self,
        world: &mut EconomyWorld,
        merchants: &mut [crate::ai::reputation::Merchant],
        offices: &[super::extra_tax::TaxOffice],
        _stocks: &mut [super::trade::OfficeStock],
        tick: u32,
    ) {
        self.dispatch_scheduled_tasks_full(world, merchants, offices, tick)
    }
    fn dispatch_scheduled_tasks_fleet(
        &mut self,
        world: &mut EconomyWorld,
        merchants: &mut [crate::ai::reputation::Merchant],
        offices: &[super::extra_tax::TaxOffice],
        stocks: &mut [super::trade::OfficeStock],
        _ships: &mut Vec<super::ship_trade::ShipCargo>,
        _difficulty: f32,
        tick: u32,
    ) {
        self.dispatch_scheduled_tasks_assets(world, merchants, offices, stocks, tick)
    }
    /// Imágenes de oficinas vigentes para servicios de tareas que consultan administradores.
    fn needs_scheduled_office_images(&self, _tick: u32) -> bool {
        false
    }
    fn set_scheduled_office_images(&mut self, _images: Vec<Vec<u8>>) {}
    /// Cambios preparados por tareas al administrador de una oficina.
    fn take_scheduled_office_updates(&mut self) -> Vec<ScheduledOfficeUpdate> {
        Vec::new()
    }
    /// `town_daily_construction_planner` (0x0051D870).
    fn construction_planner(&mut self, town: &mut Town);
    fn construction_planner_with_world(&mut self, _world: &EconomyWorld, town: &mut Town) {
        self.construction_planner(town)
    }
    /// `Town::advance_construction` (0x0051FF30), desde los constructores.
    fn advance_construction(&mut self, town: &mut Town, workers: u16);
    /// Obras del astillero (0x005083B0).
    fn shipyard_construction(&mut self, town_index: u8, work: u32);
    /// Reparaciones del astillero (0x00508AA0).
    fn shipyard_repairs(&mut self, town_index: u8, work: u32);
    /// Variante con ciudad viva; los anfitriones antiguos conservan ambos callbacks.
    fn shipyard_advance_with_town(&mut self, town: &mut Town, work: u32) {
        self.shipyard_construction(town.index(), work);
        self.shipyard_repairs(town.index(), work)
    }
    /// El puente de oficinas aporta las imágenes de comerciantes y el buzón vivo.
    fn shipyard_advance_transaction(
        &mut self,
        town: &mut Town,
        work: u32,
        _merchants: &mut [crate::ai::reputation::Merchant],
    ) -> Result<(), String> {
        self.shipyard_advance_with_town(town, work);
        Ok(())
    }
    /// `town_get_wall_expansion_level` (0x005252F0).
    fn wall_expansion_level(&mut self, town: &Town) -> u8;
    /// `find_or_create_office` (0x005308A0) para un alcalde humano.
    fn find_or_create_office(&mut self, world: &mut EconomyWorld, merchant: u8, town_index: u8) -> Option<usize>;
    /// `siege_record_get_pitch_demand` (0x00636920).
    fn siege_pitch_demand(&mut self, town_index: u8) -> i32;
    /// La ciudad viva permite reconstruir el objetivo de una escena cargada.
    fn siege_pitch_demand_for_town(&mut self, town: &Town) -> i32 {
        self.siege_pitch_demand(town.index())
    }
    /// Mensaje de hambruna (`add_message` con la carta 0x35).
    fn famine_message(&mut self, town_index: u8);
    /// Variante con fecha viva para insertar la carta global en el buzón.
    fn famine_message_with_world(&mut self, _world: &EconomyWorld, town_index: u8) {
        self.famine_message(town_index)
    }
    /// Aviso de hambruna a un comerciante humano de la ciudad (`do_notification` y ventanas).
    fn famine_notify(&mut self, merchant: u16, town_index: u8, local: bool);
}
