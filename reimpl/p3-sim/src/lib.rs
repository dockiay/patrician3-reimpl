//! Simulación sin pantalla (C10): carga un guardado `.pat` original y avanza el reloj del juego con los sistemas
//! ya reimplementados (reloj, ciudades, impuestos y oficinas). Lo que aún no está portado entra por ganchos que no
//! hacen nada y se cuentan en [`Report::inactive`]. La fase diaria del comerciante se ejecuta en modo local;
//! aún quedan dependencias de IA, batallas y asedios. El resultado **no** equivale todavía al juego completo.

mod pirate_b4_session;
mod load_start;
mod factory_nav;
/// Byte 0x00672856 del ejecutable inglés: clase del nodo de un barco que sale de un convoy de caza.
pub const HUNT_CONVOY_LOSS_CLASS:u8=2;
pub mod pirate_capture;
pub mod c10_battle_capture;
pub mod navigation;
pub mod convoy_absorption;
pub mod pirate_group_runtime;
pub mod pirate_active_frame;
pub mod loaded_route;
pub mod pirate_group_native_host;
pub mod pirate_group_create;
pub mod pirate_group_remove;
pub mod pirate_group_effects;
pub mod pirate_group_tick_stage;
pub mod route_stop_runtime;
pub mod pirate_b5_effects;
pub mod expedition_finish_host;
pub mod absorb_hut;
pub mod absorb_ship_refresh;
pub mod battle_opening;
pub mod pirate_b5_stage;
pub mod pirate_b5_context;
pub mod expedition_return_letter;
pub mod trade_center_data;
pub mod trade_center_text;
pub mod trade_contract_text;
pub mod town_event_runtime;
pub mod town_event_live;
pub mod town_event_plan_runtime;
pub mod town_event_spawn_runtime;
pub mod town_pirate_spawn;
pub mod town_pirate_new_band;
mod c10_pirate_b5_runtime;
pub mod expedition_discovery_event;
pub mod expedition_visit;
pub mod expedition_center_host;
pub mod wedding_delivery;
mod wedding_runtime;
mod mission_ship_create;
pub mod ten_day_runtime;
pub mod merchant_daily;
pub mod mission_bank;
pub mod mission_text;
mod merchant_ai_decisions;
mod ai_construction;
pub mod town_plans;
pub mod foundation_nav;
pub mod fleet_resume;
pub mod town_names;
mod office_images;
mod town_history;
mod siege_daily;
pub mod siege_pitch;
pub mod statistics;
pub mod persist;
pub mod lender;
pub mod civic_planner;
pub mod housing_planner;
pub mod defense_planner;
pub mod office_take;
pub mod wall_work;
pub mod well_work;
pub mod live_office;
pub mod live_building;
pub mod battle_manager;
pub mod battle_text;
pub mod letter_text;
pub mod c08_text;
pub mod c08_hunt;
mod c08_operations;
mod c08_flat;
mod tavern_screen;
mod shipyard_repair;
pub use tavern_screen::PAGES as TAVERN_PAGES;
mod c10_script_flat;
mod c08_naval_ops;
mod c08_convoy_ops;
mod c08_admin_ops;
mod c08_found_office;
mod c08_mission_pirate;
mod ai_cash;
mod ai_bankruptcy;
mod siege_ops;
pub mod siege_view_live;
/// Resultado de la fase 10 de la caza calculado sobre copias.
pub(crate) struct HuntStep{record:p3_core::scheduler::TaskRecord,convoy:Vec<u8>,ships:Vec<Option<Vec<u8>>>,id:usize}
pub(crate) struct HuntPrepared{record:p3_core::scheduler::TaskRecord,convoy:Vec<u8>,ships:Vec<Option<Vec<u8>>>,pool:Vec<u8>,merchants:Vec<p3_core::ai::reputation::Merchant>,id:usize}
pub mod battle_spatial;
pub mod battle_runtime;
pub mod merchant_ship_counts;
pub mod auction_runtime;
mod auction_transfer;
mod captain_refill;
mod mayor_schedule;
mod mayor_election;
mod mayor_statue;
mod weather_spawn;
mod ai_ship_cost;
pub mod c10_ship_orders;
pub mod c10_convoy_operations;
mod c10_convoy_runtime;
mod c10_investigation;

use p3_core::ai::reputation::Merchant as RepMerchant;
use p3_core::buildings::counts::OfficeLists;
use p3_core::buildings::housing::{HousingCtx, HousingHost, OfficeHousing};
use p3_core::buildings::Site;
use p3_core::calendar::Calendar;
use p3_core::economy::ship_trade::ShipCargo;
use p3_core::economy::system::{EconomySystem, EconomyTickBridge};
use p3_core::economy::taxes::{Accounts, MerchantAccounts};
use p3_core::economy::trade::OfficeStock;
use p3_core::economy::world::{EconomyWorld, Merchant, MerchantFacility, Office, Ship, TownHost};
use p3_state::office::{BusinessInput, OfficeTownHost, PersistedOffice};
use p3_core::tick::{self, TickSystems, WorldClock};
use p3_core::town::{off, Town};
use p3_save::office::OFFICE_LEN;
use p3_save::ships::Ships;
use p3_save::world::World;
use p3_save::Save;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

#[derive(Debug)]
pub enum SimError {
    Save(String),
}

impl std::fmt::Display for SimError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SimError::Save(s) => write!(f, "guardado: {s}"),
        }
    }
}

fn u16_at(b: &[u8], o: usize) -> u16 {
    u16::from_le_bytes([b[o], b[o + 1]])
}
fn u32_at(b: &[u8], o: usize) -> u32 {
    u32::from_le_bytes(b[o..o + 4].try_into().unwrap())
}

/// Datos de recursos necesarios para crear barcos de la Liga.
#[derive(Clone,Debug,Default)]
pub struct HanseFleetResources {pub ship_names:Vec<Vec<u8>>,pub first_names:u16,pub last_names:u16,pub ports:Vec<Option<(u16,u16)>>,pub hotseat:bool,pub player:u16}

/// Ganchos de ciudad. Los recuentos de edificios y de defensa y el nivel de muralla usan el código de C05 sobre los
/// registros y celdas del guardado; el resto no hace nada y cuenta cuántas veces se le llama.
/// Configuración suministrada por el anfitrión para un encuentro limitado.
#[derive(Clone,Debug)]
pub struct EncounterConfig {pub slot:u8,pub battle_address:u32,pub scene_busy:bool,pub dialog_text:Vec<u8>}

/// Memoria y reloj externos utilizados por el original al iniciar un asedio amurallado.
#[derive(Clone, Debug)]
pub struct WalledSiegeHeap {
    pub tower_bytes: Vec<u8>,
    pub list_bytes: Vec<u8>,
    pub tower_address: u32,
    pub wall_clock: u32,
}

/// Imagen para el asedio sin vista. Tras cargar se reconstruye la lista de
/// índices de torres, sin acreditar sus tiempos de disparo ni la vista visual.
#[derive(Clone, Debug)]
pub struct LiveWalledSiege {
    pub image: Vec<u8>,
    pub tower_bytes: Vec<u8>,
    pub list_bytes: Vec<u8>,
}

/// Cambios de un paso del astillero. Solo conserva los registros modificados.
#[derive(Clone)]
pub struct ShipyardDelta {
    pub ships: Vec<(usize, Vec<u8>)>,
    pub entries: Vec<(usize, Vec<u8>)>,
    pub docked_head: u16,
    pub free_entry_head: u16,
}
impl ShipyardDelta {
    fn between(before:&p3_core::shipyard::ShipManager,after:&p3_core::shipyard::ShipManager)->Self {
        let ships=before.ships.chunks_exact(0x180).zip(after.ships.chunks_exact(0x180)).enumerate().filter_map(|(i,(a,b))|(a!=b).then(||(i,b.to_vec()))).collect();
        let entries=before.entries.chunks_exact(0x3c).zip(after.entries.chunks_exact(0x3c)).enumerate().filter_map(|(i,(a,b))|(a!=b).then(||(i,b.to_vec()))).collect();
        Self{ships,entries,docked_head:after.docked_head,free_entry_head:after.free_entry_head}
    }
    fn apply(&self,manager:&mut p3_core::shipyard::ShipManager){
        for (i,record) in &self.ships{manager.ships[i*0x180..(i+1)*0x180].copy_from_slice(record)}
        for (i,record) in &self.entries{manager.entries[i*0x3c..(i+1)*0x3c].copy_from_slice(record)}
        manager.docked_head=self.docked_head;manager.free_entry_head=self.free_entry_head;
    }
}

/// Transacción del astillero cuando todavía no hay una flota de navegación viva.
#[derive(Clone)]
pub struct DetachedShipyard {
    pub factory_before: p3_core::ships::factory::HanseFactory,
    pub before: p3_core::shipyard::ShipManager,
    pub current: p3_core::shipyard::ShipManager,
    pub history: Vec<ShipyardDelta>,
}

/// Notificación de guion y petición de refresco para el anfitrión visual.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct ScriptNotification {pub args:[u32;6],pub refresh_mail_requested:bool}
/// Aviso de hambruna para el anfitrión visual; la ventana local es un efecto aparte.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct FamineNotification {pub merchant:u16,pub town:u8,pub local:bool}

/// `dst[j]=new[j]` en cada posición `j<n` donde `old[j]!=new[j]`. Compara de ocho en ocho bytes y solo baja al byte
/// en los grupos que difieren (casi todo el registro de un barco que navega sigue igual de un tick al siguiente).
#[inline]
fn patch_changed(dst:&mut [u8],old:&[u8],new:&[u8],n:usize){
    // Con un bloque más corto que `n` se conserva el recorrido byte a byte (y su fallo de índice, si lo hubiera).
    if dst.len()<n||old.len()<n||new.len()<n{for j in 0..n{if old[j]!=new[j]{dst[j]=new[j]}}return}
    let (dst,old,new)=(&mut dst[..n],&old[..n],&new[..n]);
    let mut at=0;
    while at+8<=n{
        let a=u64::from_le_bytes(old[at..at+8].try_into().expect("ocho bytes"));let b=u64::from_le_bytes(new[at..at+8].try_into().expect("ocho bytes"));
        if a!=b{for j in at..at+8{if old[j]!=new[j]{dst[j]=new[j]}}}
        at+=8;
    }
    for j in at..n{if old[j]!=new[j]{dst[j]=new[j]}}
}
/// Bloques de la instantánea naval anteriores a un paso, para `publish_navigation_delta_from`.
#[derive(Clone,Debug,Default)]
pub struct NavBefore {pub manager:Vec<u8>,pub ships:Vec<u8>,pub captains:Vec<u8>,pub convoys:Vec<u8>}
impl NavBefore {
    pub fn capture(&mut self,s:&p3_save::ships::NavigationSnapshot){
        fn put(d:&mut Vec<u8>,s:&[u8]){d.clear();d.extend_from_slice(s)}
        put(&mut self.manager,&s.manager);put(&mut self.ships,&s.ships);put(&mut self.captains,&s.captains);put(&mut self.convoys,&s.convoys);
    }
}
/// Memoria de trabajo que el tick naval y el despacho de tareas reutilizan de un tick al siguiente. No es estado
/// del juego: cada uso la rellena antes de leerla.
#[derive(Clone,Debug,Default)]
pub struct NavScratch {
    pub before:NavBefore,
    changed:Vec<usize>,
    /// Imágenes de expedición anteriores al despacho, seguidas (0x180 por barco, 0x3c por convoy).
    exp_ships:Vec<u8>,exp_ship_some:Vec<bool>,exp_convoys:Vec<u8>,
    /// Ciudades y comerciantes anteriores al tick naval vivo, para deshacerlo si falla.
    towns:Vec<u8>,merchants:Vec<u8>,merchant_ends:Vec<usize>,
}
impl NavScratch {
    /// Copia las imágenes de expedición en bloques seguidos. Devuelve `false` (sin copia útil) si alguna no tiene
    /// el tamaño fijo; el llamante usa entonces la copia completa de siempre.
    fn capture_expedition(&mut self,ships:&[Option<Vec<u8>>],convoys:&[Vec<u8>])->bool{
        if ships.iter().any(|s|s.as_ref().is_some_and(|v|v.len()!=0x180))||convoys.iter().any(|c|c.len()!=0x3c){return false}
        // Sin rellenar con ceros: los huecos sin imagen no se leen (`expedition_ship` mira antes `exp_ship_some`).
        self.exp_ships.resize(ships.len()*0x180,0);self.exp_ship_some.clear();
        for (i,s) in ships.iter().enumerate(){self.exp_ship_some.push(s.is_some());if let Some(v)=s{self.exp_ships[i*0x180..(i+1)*0x180].copy_from_slice(v)}}
        self.exp_convoys.clear();for c in convoys{self.exp_convoys.extend_from_slice(c)}
        true
    }
    fn capture_world(&mut self,towns:&[Town],merchants:&[RepMerchant]){
        self.towns.clear();for t in towns{self.towns.extend_from_slice(&t.0[..])}
        self.merchants.clear();self.merchant_ends.clear();for x in merchants{self.merchants.extend_from_slice(&x.bytes);self.merchant_ends.push(self.merchants.len())}
    }
    fn restore_world(&self,towns:&mut [Town],merchants:&mut [RepMerchant]){
        let size=p3_core::ships::remove::TOWN_SIZE;for (t,src) in towns.iter_mut().zip(self.towns.chunks_exact(size)){t.0.copy_from_slice(src)}
        let mut start=0;for (x,&end) in merchants.iter_mut().zip(&self.merchant_ends){x.bytes.clear();x.bytes.extend_from_slice(&self.merchants[start..end]);start=end}
    }
    fn expedition_ship(&self,i:usize)->Option<&[u8]>{if *self.exp_ship_some.get(i)?{self.exp_ships.get(i*0x180..(i+1)*0x180)}else{None}}
    fn expedition_convoy(&self,i:usize)->Option<&[u8]>{self.exp_convoys.get(i*0x3c..(i+1)*0x3c)}
}
#[derive(Default)]
pub struct NullHost {
    /// Memoria de trabajo del tick naval (no es estado del juego).
    pub nav_scratch:NavScratch,
    /// Pedido de barco del jugador (op 0x2C): ciudad, comerciante y clase; lo consume la siguiente llamada al despacho.
    pub human_ship_order:Option<(usize,usize,u8,bool)>,
    pub human_ship_order_result:Option<Result<Option<(u16,i32)>,String>>,
    /// 0x0E usa el inicio guardado y la clasificación aportada por el anfitrión.
    pub game_end_start:u32,
    pub game_end_mode:u8,
    pub game_end_top_ranked:Option<u16>,
    /// Contenido vivo de GameEndScreen+0x34C, si el anfitrión lo conoce.
    pub game_end_score_flag:Option<u8>,
    pub game_end_stopped:bool,
    pub game_end_events:Vec<p3_core::economy::game_end::Event>,
    /// Recursos acreditados para el regreso de expedición 0x12.
    pub expedition_return_resources: Option<expedition_return_resources::Resources>,
    pub expedition_return_notices: Vec<expedition_finish_host::Notification>,
    pub expedition_return_map_dirty: bool,
    pub expedition_return_map_notices: Vec<[u32;6]>,
    pub expedition_return_map_windows: Vec<[u32;3]>,
    pub land_transport_notifications: Vec<[u32;6]>,
    pub trade_offer_initial: Option<Vec<[u8;16]>>,
    pub trade_offers: Option<Vec<[u8;16]>>,
    pub trade_offer_raw_initial: Option<Vec<u8>>,
    pub trade_offer_versions: std::cell::RefCell<Vec<Vec<u8>>>,

    /// GameWorld+0x89: el lector original normaliza cualquier valor no nulo a 1.
    pub first_discovery_flag:u8,
    /// Byte exacto del S05 de entrada, para detectar escrituras ajenas.
    pub first_discovery_initial:u8,
    /// Indicadores de Operations+0; los bits 0xC identifican la partida en red.
    pub operations_mode: u32,
    pub calls: BTreeMap<&'static str, u64>,
    pub famine_notifications: Vec<FamineNotification>,
    /// Centros comerciales S04 y sus registros al cargar.
    pub trade_centers: Option<Vec<Vec<u8>>>,
    /// Tabla 673858 y puntos Expeditionen.dat; datos externos, nunca fabricados.
    pub discovery_locations:Vec<(i16,i16)>,
    /// Tabla original de límites de actividad para 514000.
    pub pirate_group_thresholds:Option<[u8;256]>,
    pub town_pirate_piece_bytes:Option<(u8,u8)>,
    pub expedition_map_points:Vec<(i16,i16)>,
    /// Copia de WorldClock+8B actualizada antes de la fase naval.
    pub naval_expedition_countdown:u8,
    pub trade_centers_initial: Option<Vec<Vec<u8>>>,
    pub trade_center_versions:Vec<std::collections::BTreeSet<[u8;32]>>,
    /// Registros de asedio al cargar; el estado vivo está en `founded_world`.
    pub siege_initial: Vec<Vec<u8>>,
    pub siege_versions:Vec<std::collections::BTreeSet<[u8;32]>>,
    /// Indicador vivo +0x79, reconstruido al cargar; None si no hay escena.
    pub siege_retreat_flags: Vec<Option<bool>>,
    /// Entradas de montón explícitas para operaciones 0x8E amuralladas.
    pub siege_walled_heap: std::collections::VecDeque<WalledSiegeHeap>,
    /// Componentes no serializados de escenas amuralladas creadas en esta sesión.
    pub siege_walled_live: Vec<Option<LiveWalledSiege>>,
    /// Vistas de ciudad abiertas durante un asedio, por ciudad (las abre el cliente; sin pantalla no hay ninguna).
    pub siege_views: Vec<Option<siege_view_live::SiegeTownView>>,
    /// Cabezas privadas modificadas por incendios, pendientes de aplicar a S02.
    pub burn_office_updates: Vec<(usize,u16,u16)>,
    /// Bytes indeterminados del original al seleccionar demandas mensuales.
    pub siege_stack_tail: Option<[u8;3]>,
    /// Esos bytes por llamada mensual (tick de la ciudad, bytes) tal como los deja el original emulado
    /// en 0x635220; la entrada del mismo tick tiene prioridad sobre `siege_stack_tail` (día 1).
    pub siege_stack_tails: std::collections::VecDeque<(u32,[u8;3])>,
    /// Inicios de asedio medidos en el original (tick, ciudad, reloj de pared `0x6DCCF8`): el asedio amurallado sin
    /// memoria explícita en `siege_walled_heap` toma de aquí el reloj y usa un montón a cero, como el montón del oráculo.
    pub siege_start_clocks: Vec<(u32,u8,u32)>,
    /// Bytes de heap no inicializados para cada nueva petición 0x8B, en orden.
    pub siege_request_heap_bytes: std::collections::VecDeque<[u8;2]>,
    /// Solo en sesiones sin vista: permite los ticks de marcha y aproximación acreditados.
    pub siege_headless_march: bool,
    pub siege_difficulty: u8,
    pub town_history: Option<Vec<u8>>,
    pub town_history_initial: Option<Vec<u8>>,
    pub town_history_versions:std::collections::BTreeSet<[u8;32]>,
    /// Estadísticas `.pst` opcionales; el formato `.pat` no las contiene.
    pub statistics: Option<p3_save::model::StatisticsModel>,
    /// Viaje humano limitado; los demás barcos aún no ejecutan su IA.
    pub navigation: Option<navigation::LiveVoyage>,
    /// Grupos de selección guardados y su estado al cargar.
    /// Depósito de paradas y versiones propias para publicar PoolRelease en S05.
    pub route_stop_runtime:Option<route_stop_runtime::RouteStopRuntime>,
    pub merchant_ship_groups: Vec<p3_save::world::MerchantShipGroups>,
    pub merchant_ship_groups_initial: Vec<p3_save::world::MerchantShipGroups>,
    pub merchant_ship_groups_changed: Vec<usize>,
    pub navigation_planner: Option<p3_core::route::finder::RoutePlanner>,
    pub navigation_error: Option<String>,
    /// Datos externos de comercio y distancias para la IA naval, si se ha configurado.
    pub navigation_ai: Option<p3_core::ships::ai_dock::AiDockData>,
    /// Encuentro solicitado y resultado, todavía sin aplicar a la interfaz.
    pub encounter_config: Option<EncounterConfig>,
    /// Geometria de LIGHTNINGVIEW0/TileSize cargada de los CPR para abrir batallas.
    pub battle_tile_width: Option<i16>,
    pub encounter_battle: Option<p3_core::ships::battle::CommercialBattleStart>,
    /// Asalto urbano vivo que aún no se puede recargar desde S05.
    pub town_attack_active: Option<town_attack_manager::Runtime>,
    pub town_attack_collision: Option<p3_core::ships::spatial::CollisionIndex>,
    pub town_attack_band: Option<u8>,
    pub town_attack_tick_pending: bool,
    pub town_attack_save_unreconciled: bool,
    pub town_attack_clock_target: Option<u32>,
    pub town_attack_manager_processed: bool,
    pub town_attack_frame_effects: Vec<pirate_empty_scene_publish::Effect>,
    pub town_attack_manager_effects: Vec<battle_manager::ManagerEffect>,
    pub town_attack_effects: Vec<pirate_empty_hideout_messages::Effect>,
    /// Noticias globales propias del asalto, con la fecha en que se emitieron.
    #[doc(hidden)] pub town_attack_global_news: Vec<(p3_core::calendar::Calendar,[u8;16])>,
    pub pirate_scene_config: Option<pirate_mission_publish::Config>,
    pub pirate_scene_projection: Option<p3_core::ships::town_attack_mask::TownProjection>,
    pub pirate_scene_texts: Option<battle_text::BattleTextBank>,
    pub town_attack_origin: Vec<u8>,
    pub pending_pirate_attack: Option<(u16,p3_core::scheduler::TaskRecord)>,
    // NullHost permite la construcción externa mediante `..Default::default()`.
    // Estos campos son estado interno de la transacción; el llamante debe usar
    // las operaciones de Sim para conservar sus invariantes.
    #[doc(hidden)] pub pending_pirate_boundary: Option<p3_core::tick::TaskBoundary>,
    #[doc(hidden)] pub pending_pirate_target: Option<(u32,bool)>,
    #[doc(hidden)] pub pirate_boundary_model: Option<p3_save::model::SaveModel>,
    #[doc(hidden)] pub pirate_boundary_live: Option<pirate_empty_scene_publish::LiveState>,
    #[doc(hidden)] pub pirate_active_live: Option<pirate_empty_scene_publish::LiveState>,
    /// Última imagen S05 del gestor vacío guardada por esta sesión.
    pub battle_manager_saved:std::cell::RefCell<Option<Vec<u8>>>,
    pub clock_before_battle: bool,
    /// Registros y celdas de cada ciudad.
    pub sites: Vec<Site>,
    pub housing: Vec<OfficeHousing>,
    pub lists: Vec<OfficeLists>,
    pub office_count: u16,
    pub office_pool: Option<p3_core::economy::office_factory::OfficePool>,
    pub office_pool_initial: Option<p3_core::economy::office_factory::OfficePool>,
    pub scheduled_office_images: Option<Vec<Vec<u8>>>,
    pub scheduled_office_updates:Vec<p3_core::economy::world::ScheduledOfficeUpdate>,
    pub pending_office: Option<(u8,u8,u32)>,
    pub pending_founding: Option<(u16,p3_core::scheduler::TaskRecord)>,
    pub pending_pirate_approach: Option<(u16,p3_core::scheduler::TaskRecord)>,
    pub pending_ai_expansion:Option<(u16,p3_core::scheduler::TaskRecord,p3_core::scheduler::TaskRecord,u8,u8)>,
    pub church_cost_factor: u8,
    /// Ciudad marcada por el clima (GameWorld+0x8F) y puntos de guardado.
    pub weather_marker:Option<u8>,
    pub weather_marker_initial:Option<u8>,
    pub weather_marker_saved:std::cell::RefCell<Option<u8>>,
    /// Siete anuncios del ayuntamiento (GameWorld+0x90) y su procedencia S05.
    pub town_hall_notices:Option<[u8;84]>,
    pub town_hall_notices_initial:Option<[u8;84]>,
    pub town_hall_notices_saved:std::cell::RefCell<Option<[u8;84]>>,
    /// Velocidad de game_settings+4 y avisos sin presentar: (ciudad, deshielo).
    pub weather_speed:u32,
    pub weather_ice_notifications:Vec<(u16,bool)>,
    /// Tareas cargadas del guardado; el despacho de tareas sigue pendiente.
    pub council_scheduler: p3_core::scheduler::Scheduler,
    /// Guiones y variables reconstruidos desde recursos CPR y tareas S05.
    pub mission_bank: Option<mission_bank::MissionBank>,
    pub mission_variables: Vec<Option<Vec<u32>>>,
    /// Listas de bajas de la caza hanseática (`tarea 0x34 + 8`, nodos de 0x004F3980) por ranura, cabeza primero.
    pub hunt_losses: std::collections::BTreeMap<usize,Vec<p3_save::tail::HuntLoss>>,
    pub mission_initial: Vec<Option<p3_save::tail::ScriptSnapshot>>,
    pub mission_saved: std::cell::RefCell<Option<Vec<Option<p3_save::tail::ScriptSnapshot>>>>,
    /// Operaciones del concejo y de expediciones, en el orden de emisión.
    pub council_operations: Vec<p3_core::pump::Operation>,
    pub council_notifications: Vec<(u16,u8)>,
    /// Avisos de misiones completadas y peticiones de presentación pendientes.
    pub mission_completion_notifications: Vec<[u32;6]>,
    /// Aviso 0x12 de F7 y petición de refresco del correo; pendientes de presentación.
    pub mission_script_notifications: Vec<ScriptNotification>,
    pub mission_texts: Option<mission_text::ScriptTextBank>,
    pub mission_completion_ui: Vec<(u16,u8)>,
    pub mission_failure_ui: Vec<(u16,u8)>,
    pub mission_changed_towns: Vec<usize>,
    /// `.rdata` del ejecutable inglés para los comandos de guion sobre imagen plana (`c10_script_flat`).
    pub mission_flat_rdata: Option<Vec<u8>>,
    /// Opciones de partida (`0x4C` bytes) como las deja el cargador `0x00549F40`, para los guiones planos.
    pub mission_settings: Option<Vec<u8>>,
    pub mission_changed_sites: Vec<usize>,
    pub settlement_plans: Option<town_plans::TownPlanBank>,
    /// Mundo de referencia para la matriz y las ciudades fundadas en esta sesión.
    pub founded_world: Option<p3_save::model::WorldModel>,
    /// El CRT no se guarda en S05: se suministra solo con estado y modo local conocidos.
    pub investigation_rng: Option<p3_core::rng::CrtRand>,
    /// Valor de la ranura de pila sin inicializar que lee  x004F5A20 (obra de venta de la IA). None: solo se
    /// acepta la obra si el resultado no depende de ese valor; un valor lo fija como suposición explícita de la sesión.
    pub ai_work_residue: Option<u32>,
    /// Ranura de pila de 0x004F5A20 medida por tick en el original emulado (`ai_work_slots` del oraculo x86);
    /// tiene prioridad sobre `ai_work_residue`.
    pub ai_work_residue_by_tick: Vec<(u32, u32)>,
    /// Campos de tiempo del objeto de operaciones (`OPS`, 0x006DF2F0) como los deja el constructor 0x00546450 y
    /// los cambia el opcode 0xC8: `+0x8D4` (`0x006DFBC4`, milisegundos por tick, 234 a velocidad normal) lo leen
    /// la vista de asedio y sus figuras. Ver [`Sim::apply_time_mode`].
    pub time_mode: p3_core::pump::TimeMode,
    /// Nivel de velocidad elegido en esta sesión (ajustes `+4`, `[0x006CC3E8]+4`); None conserva el guardado.
    pub speed_level: Option<u32>,
    /// Política sin pantalla para el combate pirata contra un convoy humano (ver `PirateEncounterOptions::human_as_ai`).
    pub headless_human_battles: bool,
    /// Modo de turnos Operations+0x928 y avisos pendientes de presentación.
    pub reminder_hotseat_gate:Option<u32>,
    pub reminder_notices:Vec<(u16,u8)>,
    pub auction_notices:Vec<(u16,u8)>,
    pub auction_finished:Vec<u8>,
    pub auction_results:Vec<p3_core::economy::auction::AuctionResult>,
    /// Campos transitorios de fiestas; S05 no contiene su valor.
    pub celebration_state:Option<p3_core::economy::celebration::CelebrationState>,
    /// Presentación solicitada por fiestas humanas ya aplicadas.
    pub celebration_notifications:Vec<(u16,u8)>,
    pub celebration_windows:Vec<(u16,u8)>,
    pub plague_notifications:Vec<(u16,u8)>,
    pub plague_windows:Vec<(u16,u8)>,
    pub fire_notifications:Vec<(u16,u8)>,
    pub fire_windows:Vec<(u16,u8)>,
    /// Avisos y ventanas pendientes de la entrega de barcos de boda.
    pub wedding_notifications:Vec<[u32;6]>,
    pub wedding_windows:Vec<(u16,u8)>,
    pub investigation_local_mode: bool,
    pub investigation_notifications:Vec<(u16,u8)>,
    pub investigation_changed_towns:Vec<usize>,
    /// Avisos 0x21 de la primera petición hanseática: representante, sede y ciudad afectada.
    pub hanse_notifications: Vec<(u16,u8,u8)>,
    /// Notificaciones de cobro parcial: deudor y ciudad.
    pub debt_notifications: Vec<(u16,u8)>,
    pub debt_seizure_notifications: Vec<(u16,u8,u8)>,
    /// Siguientes, estado, nombre y convoy de los barcos cargados desde S05.
    pub debt_ship_metadata: Vec<(u16,u16,u16,u16)>,
    pub debt_changed_ships: Vec<usize>,
    pub debt_convoy_flags: Vec<u32>,
    pub debt_changed_convoys: Vec<usize>,
    /// Barcos con contador de pasajes `+0x157` o marca `+0x3C` cambiados por las operaciones de taberna 0x53/0x54.
    pub tavern_changed_ships: Vec<usize>,
    pub debt_changed_towns: Vec<usize>,
    pub hanse_vote_notifications: Vec<(u16,u8)>,
    pub hanse_vote_windows: Vec<u8>,
    pub hanse_financial_towns: Vec<usize>,
    /// Imágenes completas desde S02, necesarias para evaluar la aptitud de cada barco.
    pub expedition_factory:Option<p3_core::ships::factory::HanseFactory>,
    pub expedition_factory_initial:Option<p3_core::ships::factory::HanseFactory>,
    /// Última imagen S02 que esta sesión guardó con éxito y su fábrica correspondiente.
    pub saved_factory_checkpoint:std::cell::RefCell<Option<(p3_save::model::ShipsModel,p3_core::ships::factory::HanseFactory)>>,
    pub expedition_resources:Option<HanseFleetResources>,
    pub expedition_town_ids:Vec<u8>,
    pub expedition_notifications:Vec<(u16,u32,u8)>,
    pub expedition_windows:Vec<u8>,
    pub expedition_ship_images: Vec<Option<Vec<u8>>>,
    pub expedition_original_ship_images: Vec<Option<Vec<u8>>>,
    pub expedition_convoy_images: Vec<Vec<u8>>,
    pub expedition_docked_head:u16,
    /// Cabeza de barcos en el mar al cargar S02, antes de activar navegación.
    pub expedition_sea_head:u16,
    /// Objetos de estatua pendientes de presentación: ciudad, fila, columna, variante y dueño.
    pub mayor_statue_visuals:Vec<(u8,u8,u8,u8,u8)>,
    pub expedition_convoys_head:u16,
    pub expedition_initial_docked_head:u16,
    pub expedition_changed_ships:Vec<usize>,
    pub ai_fleet_changed_ships:Vec<usize>,
    pub expedition_changed_convoys:Vec<usize>,
    pub expedition_changed_towns: Vec<usize>,
    pub hanse_pirates: Option<p3_core::economy::hanse_petition::PiratePortraits>,
    /// Destinos de bandas leídos de S02; aún no se actualizan por IA pirata.
    pub pirate_targets: Option<[Option<u8>;5]>,
    /// Imágenes de S02 de las bandas y su estado al cargar.
    pub mission_pirates: Option<[Option<p3_core::ai::settlement::MissionPirateBand>;5]>,
    pub mission_pirates_initial: Option<[Option<p3_core::ai::settlement::MissionPirateBand>;5]>,
    /// Últimas bandas confirmadas por un guardado completo de esta sesión.
    pub pirate_bands_saved: std::cell::RefCell<Option<[Option<p3_core::ai::settlement::MissionPirateBand>;5]>>,
    /// Bandas completas de S02: el tick puede modificar también la lista de
    /// reserva, el plano y las piezas que no figuran en la vista de misiones.
    pub pirate_band_images_initial: Option<Vec<Option<p3_save::pirate_bands::PirateBand>>>,
    pub pirate_band_images: Option<Vec<Option<p3_save::pirate_bands::PirateBand>>>,
    pub pirate_band_images_saved: std::cell::RefCell<Option<Vec<Option<p3_save::pirate_bands::PirateBand>>>>,
    /// Las 40 posiciones y regiones de `StadtDaten.ini` y el ajuste S0+0x17.
    pub settlement_locations: Option<[p3_core::ai::settlement::SettlementLocation;40]>,
    pub settlement_sea_directions: Option<[u8;40]>,
    /// Bandera +0x2F de las posiciones del mapa, derivada de AmFluss.
    pub settlement_inland: Option<[u8;40]>,
    pub foundation_navigation: Option<foundation_nav::FoundationNavigation>,
    pub foundation_names: Option<(u16,u16)>,
    /// Longitudes de nombres y contadores del banco de candidatos decenales.
    pub candidate_name_lengths:Option<Vec<u8>>,
    pub candidate_custom_name_lengths:Option<[Vec<u8>;2]>,
    /// Apellidos de recursos y sus veinte ranuras personalizadas del guardado.
    pub family_names:Option<Vec<Vec<u8>>>,
    pub family_custom_names:Option<Vec<Vec<u8>>>,
    pub first_names:Option<[Vec<Vec<u8>>;2]>,
    pub first_custom_names:Option<[Vec<Vec<u8>>;2]>,
    pub candidate_look_counts:Option<(u16,u16)>,
    pub town_names: Option<town_names::TownNameBank>,
    /// Textos de la elección del Ältermann (`0x6B72F0..0x6B7300`) del ejecutable inglés.
    pub election_texts: Option<c08_text::ElectionTexts>,
    pub hunt_texts: Option<c08_hunt::HuntTexts>,
    /// Avisos `0x00548CA0` de la elección (candidato 0xD, ganador 0x11), tal como los recibe la interfaz.
    pub election_notifications: Vec<[u32;6]>,
    /// Anuncio al ganador humano (`0x00469AF0`): (es el comerciante local, ganador, ciudad).
    pub election_announcements: Vec<(bool,u32,u16)>,
    pub pirate_difficulty: u8,
    /// Ajuste SETTINGS+0x12, independiente de la actividad pirata (+0x13).
    pub battle_difficulty: u8,
    pub battle_news_stamp:u32,
    pub battle_pirate_news:Vec<u8>,
    pub council_message_enabled: Vec<bool>,
    /// Preferencias de categoría 2; si faltan se usa la configuración por defecto.
    pub building_message_enabled: Vec<bool>,
    pub message_pool: Vec<u8>,
    pub council_errors: Vec<String>,
    /// Unión de los mapas de descubrimiento, reconstruida al cargar.
    pub discovery_map: Vec<u8>,
    /// Estado interno de una subasta pendiente; el simulador lo administra.
    #[doc(hidden)]
    pub pending_auction_office: Option<(u16,p3_core::scheduler::TaskRecord,usize)>,
    /// Estado interno de un tick interrumpido; el simulador lo administra.
    #[doc(hidden)]
    pub clock_partial_error: Option<String>,
    pub tax_offices: Vec<p3_core::economy::extra_tax::TaxOffice>,
    /// Imagen completa de los negocios; el mundo contiene solo sus campos de empleo.
    pub employment_businesses: Vec<[u8;20]>,
    pub employment_counts: Vec<Option<u16>>,
    pub business_free:u16,
    pub construction_tick:u32,
    pub construction_dirty:bool,
    /// Presentación del astillero recogida durante la simulación sin pantalla.
    pub shipyard_presentation:Vec<p3_state::office::YardPresentation>,
    pub detached_shipyard:Option<DetachedShipyard>,
}

struct NoMap;
impl HousingHost for NoMap {
    fn add_map_object(&mut self, _: u32, _: u32, _: u32, _: u32, _: u32) {}
}
impl p3_core::buildings::church::ChurchHost for NoMap {
    fn add_map_object(&mut self, _:u32, _:u32, _:u32, _:u32, _:u32) { unreachable!("simulación sin vista") }
    fn set_progress(&mut self, _:u32, _:u32, _:u32, _:u32, _:u32) { unreachable!("simulación sin vista") }
}
impl p3_core::buildings::type2f::Type2fHost for NoMap {
    fn add_map_object(&mut self,_:u32,_:u32,_:u32,_:u32,_:u32) { unreachable!("simulación sin vista") }
    fn set_progress(&mut self,_:u32,_:u32,_:u32,_:u32,_:u32) { unreachable!("simulación sin vista") }
}

fn save_version(data:&[u8])->[u8;32] {use sha2::{Digest,Sha256};Sha256::digest(data).into()}

impl NullHost {
    fn siege_pitch_value(&mut self,city:u8,town:Option<&Town>)->i32 {
        let run=||->Result<i32,String>{
            let record=self.founded_world.as_ref().ok_or("mundo de asedio ausente")?.sieges.get(usize::from(city)).ok_or("asedio ausente")?;
            let Some(saved)=p3_save::world::siege_scene(record,usize::from(city)).map_err(|e|format!("escena: {e:?}"))? else{return Ok(0)};
            let live=self.siege_walled_live.get(usize::from(city)).and_then(Option::as_ref);
            let mut image=vec![0;0x650];
            if let Some(live)=live {image.clone_from(&live.image);if p3_save::siege_scene::encode(&image).map_err(str::to_string)?!=saved{return Err("escena viva y guardada divergentes".into())}}
            else {
                p3_save::siege_scene::decode_fields(saved,&mut image).map_err(str::to_string)?;image[0x5d3]=city;
                if image[0x78]==0 && u16_at(&image,0x42)==0 && u32_at(&image,4)!=0 {
                    let town=town.filter(|t|t.index()==city).ok_or("objetivo de asedio no reconstruido")?;
                    let site=self.sites.get(usize::from(city)).ok_or("plano de asedio ausente")?;let count=usize::from(town.u16(0x776));if site.records.len()!=count*8{return Err("registros de asedio incompatibles".into())}
                    for head in [town.u16(0x768),town.u16(0x762)] {let mut id=usize::from(head);let mut seen=vec![false;count];while id<count{if seen[id]{return Err("defensas cíclicas".into())}seen[id]=true;id=usize::from(u16_at(&site.records,id*8+4));}}
                    let level=site.wall_expansion_level(town);if level>2{return Err("objetivo de asedio sin muralla reconstruida".into())}
                    let gates=p3_core::economy::siege::choose_wall_gate(&site.records,town.u16(0x768),town.u16(0x762),level,0).map_err(str::to_string)?;
                    for (i,gate) in gates.gates.iter().enumerate(){image[0x244+4*i..0x248+4*i].copy_from_slice(&gate.to_le_bytes())}
                    if image[0x6d]>=3{return Err("índice de puerta de asedio inválido".into())}
                }
            }
            let site=self.sites.get(usize::from(image[0x5d3])).ok_or("ciudad del asedio ausente")?;
            siege_pitch::demand(&image,&site.records).map_err(str::to_string)
        };
        match run(){Ok(value)=>value,Err(e)=>{self.council_errors.push(format!("brea del asedio ciudad {city}: {e}"));0}}
    }
    /// Copia del registro de asedio antes de un paso. Antes se devolvía su SHA-256; ahora el resumen solo se
    /// calcula en `remember_siege_version` si el registro cambió (comparación directa de bytes, mismo resultado).
    fn siege_version(&self,index:usize)->Option<Vec<u8>>{self.founded_world.as_ref()?.sieges.get(index).cloned()}
    fn remember_siege_version(&mut self,index:usize,before:Option<Vec<u8>>) {
        if let Some(before)=before {if self.founded_world.as_ref().and_then(|w|w.sieges.get(index))!=Some(&before){if self.siege_versions.len()<=index{self.siege_versions.resize_with(index+1,Default::default)}self.siege_versions[index].insert(save_version(&before));}}
    }
    fn reconcile_pirate_band_image(&self,index:usize)->Result<p3_save::pirate_bands::PirateBand,String>{
        let now=self.pirate_band_images.as_ref().and_then(|v|v.get(index)).and_then(Option::as_ref).ok_or("banda viva ausente")?;
        let mission=self.mission_pirates.as_ref().and_then(|v|v.get(index)).and_then(Option::as_ref).ok_or("vista de misión de banda ausente")?;
        let saved=self.pirate_band_images_saved.borrow();let last=saved.as_ref().and_then(|v|v.get(index)).and_then(Option::as_ref);
        let base=self.pirate_band_images_initial.as_ref().and_then(|v|v.get(index)).and_then(Option::as_ref).or(last).unwrap_or(now);
        reconcile_event_band(base,now,mission,last,index)
    }

    /// Propaga solo los bytes modificados por el paso naval a las demás vistas.
    /// `0x00535400` desde el tick naval: cada aviso pone a 0 la edad de la ciudad en la ranura del
    /// dueño y relaja las demás (`0x00535630`). El original lo hace dentro del mismo tick; ningún
    /// otro paso del tick lee el historial entre el aviso y el final del tick naval.
    fn apply_navigation_town_knowledge(&mut self,w:&EconomyWorld){
        let Some(nav)=self.navigation.as_mut() else{return};
        if !nav.events.iter().any(|e|e.starts_with(navigation::KNOWN_TOWN)){return}
        let mut marks=Vec::new();nav.events.retain(|e|match e.strip_prefix(navigation::KNOWN_TOWN).and_then(|v|v.split_once(':')){Some((t,o))=>{marks.push((t.parse::<u32>().unwrap_or(u32::MAX),o.parse::<u32>().unwrap_or(u32::MAX)));false},None=>true});
        let n=w.towns.len();
        let Some(history)=self.town_history.as_mut() else{self.council_errors.push("navegación: historial de ciudades ausente para 0x00535400".into());return};
        let Some(distances)=self.founded_world.as_ref().and_then(|world|world.matrix.get(..n*n)) else{self.council_errors.push("navegación: matriz de ciudades ausente para 0x00535400".into());return};
        let old=history.clone();
        for (town,owner) in marks{if let Err(e)=p3_core::economy::c08_town_info::mark_town_known(history,town,owner,distances,n){self.council_errors.push(format!("navegación: {e}"));return}}
        if *history!=old{self.town_history_versions.insert(save_version(&old));}
    }
    /// Carta 0x1B de `0x00519D90`: el barco humano que se hunde en el mar la recibe por `0x004D6530`.
    fn apply_navigation_ship_lost_letters(&mut self,w:&EconomyWorld,m:&mut [RepMerchant],tick:u32){
        let Some(nav)=self.navigation.as_mut() else{return};
        if !nav.events.iter().any(|e|e.starts_with(navigation::SHIP_LOST_LETTER)){return}
        let mut letters=Vec::new();nav.events.retain(|e|match e.strip_prefix(navigation::SHIP_LOST_LETTER).and_then(|v|v.split_once(':')){Some((o,h))=>{let mut r=[0u8;16];for i in 0..16{r[i]=u8::from_str_radix(&h[2*i..2*i+2],16).unwrap_or(0)}letters.push((o.parse::<i32>().unwrap_or(-9),r));false},None=>true});
        let date=p3_core::calendar::Calendar::for_tick(tick);
        for (owner,record) in letters{
            if let Err(e)=p3_state::messages::post_message(&mut self.message_pool,m,w.merchant_count,date.day,date.month,date.year,owner,&record,p3_state::messages::Payload::None,&p3_state::messages::RETENTION){self.council_errors.push(format!("carta de barco perdido: {e}"));}
        }
    }
    /// Tramo 0x00502285..0x00502331 de `Convoy::advance` (`p3_core::ships::convoy_report`): cada barco que sale de
    /// un convoy de caza (tarea 0x34 en fase >= 7 de ese convoy) antepone el nodo `{0x672856 = 2, dueño, ship+0x15E}` a `tarea+8`.
    fn apply_navigation_convoy_losses(&mut self){
        let Some(nav)=self.navigation.as_mut() else{return};
        if !nav.events.iter().any(|e|e.starts_with(navigation::CONVOY_LOSS)){return}
        let mut losses=Vec::new();nav.events.retain(|e|match e.strip_prefix(navigation::CONVOY_LOSS){Some(v)=>{losses.push(v.split(':').map(|x|x.parse::<u32>().unwrap_or(u32::MAX)).collect::<Vec<_>>());false},None=>true});
        let s=&mut self.council_scheduler;let count=s.records.len().min(usize::from(s.capacity));
        for l in losses{
            let [task,convoy,owner,text]=l[..] else{self.council_errors.push("baja de convoy mal formada".into());continue};
            let slot=task as usize;if slot>=count{continue}
            let r=&mut s.records[slot];if r.kind!=0x34||r.u8_at(0x15)<7||u32::from(u16::from_le_bytes([r.u8_at(0x16),r.u8_at(0x17)]))!=convoy{continue}
            battle_manager::prepend_hunt_losses(r,slot,&mut self.hunt_losses,&[p3_save::tail::HuntLoss{class:HUNT_CONVOY_LOSS_CLASS,mark:owner as u8,text:text as u16}]);
        }
    }
    pub fn publish_navigation_delta(&mut self,before:&p3_save::ships::NavigationSnapshot,w:&mut EconomyWorld,cargo:&mut [ShipCargo]){
        self.publish_navigation_delta_from(&before.manager,&before.ships,&before.captains,&before.convoys,w,cargo)
    }
    /// Igual que `publish_navigation_delta`, con el estado anterior como bloques sueltos: los llamantes de cada tick
    /// guardan solo estos cuatro bloques en memoria reutilizada (`NavScratch`) en vez de clonar la instantánea entera
    /// (rutas y listas incluidas). Los barcos sin cambios se descartan una sola vez y las vistas de la fábrica
    /// recorren únicamente los que cambian; el resultado es el mismo byte a byte.
    fn publish_navigation_delta_from(&mut self,before_manager:&[u8],before_ships:&[u8],before_captains:&[u8],before_convoys:&[u8],w:&mut EconomyWorld,cargo:&mut [ShipCargo]){
        self.apply_navigation_town_knowledge(w);
        self.apply_navigation_convoy_losses();
        if let Some(nav)=self.navigation.as_mut(){
            for (image,route) in nav.snapshot.ships.chunks_exact(0x180).zip(&mut nav.snapshot.routes){
                if u16_at(image,0x134)>=0x17{*route=None}
            }
        }
        let Some(nav)=self.navigation.as_ref() else{return};let after=&nav.snapshot;
        let mut changed=std::mem::take(&mut self.nav_scratch.changed);changed.clear();
        for i in 0..after.ships.len()/0x180 {let at=i*0x180;let new=&after.ships[at..at+0x180];let Some(old)=before_ships.get(at..at+0x180) else{continue};if old==new{continue}changed.push(i);if let Some(slot)=self.expedition_ship_images.get_mut(i){if u16_at(old,0x134)>=0x17&&u16_at(new,0x134)<0x17{*slot=Some(new.to_vec())}else if let Some(dst)=slot{patch_changed(dst,old,new,0x180);}}if let Some(s)=w.ships.get_mut(i){let state=u16_at(new,0x134);s.owner=if state<0x17{new[0]}else{255};s.state=state}if let Some(s)=cargo.get_mut(i){s.owner=new[0];s.town=new[0x39];s.kind=new[0xe];s.capacity=u32_at(new,0x10) as i32;s.dirty=u16_at(new,0x3c);s.crew=u16_at(new,0x40);s.cutlasses=u16_at(new,0x154);s.marines=new[0x156];for k in 0..24{s.stock[k]=u32_at(new,0x54+4*k) as i32;s.average_price[k]=f32::from_bits(u32_at(new,0xb4+4*k));s.guns[k]=new[0x13c+k]}s.cargo_value=u32_at(new,0x114) as i32;s.cargo_weight=u32_at(new,0x118) as i32;s.equipment_weight=u32_at(new,0x11c) as i32;s.gun_power=u32_at(new,0x120) as i32;}if let Some(meta)=self.debt_ship_metadata.get_mut(i){meta.0=u16_at(new,4);meta.1=u16_at(new,0x134);meta.2=u16_at(new,0x15e);meta.3=u16_at(new,8)}}
        for i in 0..after.convoys.len()/0x3c {let at=i*0x3c;let new=&after.convoys[at..at+0x3c];let Some(old)=before_convoys.get(at..at+0x3c) else{continue};if old==new{continue}if let Some(dst)=self.expedition_convoy_images.get_mut(i){patch_changed(dst,old,new,0x3c);}if let Some(flags)=self.debt_convoy_flags.get_mut(i){*flags=u32_at(new,0x18)}}
        self.expedition_docked_head=u16_at(&after.manager,0xe8);
        self.expedition_sea_head=u16_at(&after.manager,0xea);
        self.expedition_convoys_head=u16_at(&after.manager,0xec);
        if let Some(factory)=self.expedition_factory.as_mut(){
            factory.free_ship=u16_at(&after.manager,0xe2);
            for (offset,field) in [(0xe0,&mut factory.free_captain),(0xe4,&mut factory.free_convoy),(0xec,&mut factory.convoy_head),(0xee,&mut factory.name_cursor),(0xf0,&mut factory.name_step)]{
                if before_manager[offset..offset+2]!=after.manager[offset..offset+2]{*field=u16_at(&after.manager,offset)}
            }
            // La fábrica comparte estos objetos con navegación en el original.
            // Publicar solo los bytes cambiados conserva los demás sistemas.
            // Solo los barcos de `changed` difieren; un índice sin imagen de fábrica no se publica (como el `zip` anterior).
            for &i in &changed{
                let at=i*0x180;let (old,new)=(&before_ships[at..at+0x180],&after.ships[at..at+0x180]);let Some(dst)=factory.ships.get_mut(i) else{continue};
                if u16_at(old,0x134)>=0x17 && u16_at(new,0x134)<0x17 {
                    // Una ranura libre S02 solo conserva cinco bytes. Sus
                    // imágenes reconstruidas pueden diferir en los demás.
                    dst.copy_from_slice(new);
                }else{patch_changed(dst,old,new,0x180);}
            }
            for (old,new,dst) in before_convoys.chunks_exact(0x3c).zip(after.convoys.chunks_exact(0x3c)).zip(&mut factory.convoys).map(|((a,b),c)|(a,b,c)){
                if old!=new{patch_changed(dst,old,new,0x3c);}
            }
            for (old,new,dst) in before_captains.chunks_exact(16).zip(after.captains.chunks_exact(16)).zip(&mut factory.captains).map(|((a,b),c)|(a,b,c)){
                if old==new{continue}
                patch_changed(dst,old,new,16);
            }
            for &i in &changed{let at=i*0x180;let new=u16_at(&after.ships,at+0x134);let old=u16_at(before_ships,at+0x134);if old<0x17&&new>=0x17{if let Some(dst)=factory.ships.get_mut(i){dst.copy_from_slice(&after.ships[at..at+0x180])}if let Some(dst)=self.expedition_ship_images.get_mut(i){*dst=None}}}
        }
        self.nav_scratch.changed=changed;
    }
    /// Publica en las demás vistas los barcos cambiados por el astillero
    /// que trabaja antes de disponer de un snapshot de navegación.
    pub fn publish_detached_shipyard_delta(&mut self, old:Option<&p3_core::shipyard::ShipManager>,w:&mut EconomyWorld,cargo:&mut [ShipCargo]){
        let Some(yard)=self.detached_shipyard.as_ref() else{return};
        let before=old.unwrap_or(&yard.before);let after=&yard.current;if before==after{return}
        for i in 0..usize::from(after.ship_count){let at=i*0x180;let old=&before.ships[at..at+0x180];let new=&after.ships[at..at+0x180];if old==new{continue}
            if let Some(Some(dst))=self.expedition_ship_images.get_mut(i){for j in 0..0x180{if old[j]!=new[j]{dst[j]=new[j]}}}
            if let Some(ship)=w.ships.get_mut(i){ship.owner=new[0];ship.state=u16_at(new,0x134)}
            if let Some(ship)=cargo.get_mut(i){ship.owner=new[0];ship.town=new[0x39];ship.kind=new[0xe];ship.capacity=u32_at(new,0x10) as i32;ship.dirty=u16_at(new,0x3c);ship.crew=u16_at(new,0x40);ship.cutlasses=u16_at(new,0x154);ship.marines=new[0x156];for k in 0..24{ship.stock[k]=u32_at(new,0x54+4*k) as i32;ship.average_price[k]=f32::from_bits(u32_at(new,0xb4+4*k));ship.guns[k]=new[0x13c+k]}ship.cargo_value=u32_at(new,0x114) as i32;ship.cargo_weight=u32_at(new,0x118) as i32;ship.equipment_weight=u32_at(new,0x11c) as i32;ship.gun_power=u32_at(new,0x120) as i32;}
            if let Some(meta)=self.debt_ship_metadata.get_mut(i){meta.0=u16_at(new,4);meta.1=u16_at(new,0x134);meta.2=u16_at(new,0x15e);meta.3=u16_at(new,8)}
        }
        for i in 0..usize::from(after.entry_count){let at=i*0x3c;let old=&before.entries[at..at+0x3c];let new=&after.entries[at..at+0x3c];if old!=new{if let Some(flags)=self.debt_convoy_flags.get_mut(i){*flags=u32_at(new,0x18)}}}
    }
    fn hit(&mut self, name: &'static str) {
        *self.calls.entry(name).or_default() += 1;
    }
}

struct ShipRetryBuild { city:usize,owner:usize,office:Option<usize>,before:c10_ship_orders::BuildState,after:c10_ship_orders::PreparedBuild }
struct ShipRetryPrepared {task:p3_core::scheduler::TaskRecord,keep:bool,build:Option<ShipRetryBuild>}
struct RetirementPrepared {
    task:p3_core::scheduler::TaskRecord,keep:bool,snapshot:p3_save::ships::NavigationSnapshot,
    factory:p3_core::ships::factory::HanseFactory,towns:Vec<Town>,images:Vec<Option<Vec<u8>>>,
    convoy_images:Vec<Vec<u8>>,
    merchants:Vec<RepMerchant>,
    message_pool:Vec<u8>,
    pirates:Option<p3_core::economy::hanse_petition::PiratePortraits>,
}
struct TenDayPrepared {state:ten_day_runtime::PreparedTenDay,factory:p3_core::ships::factory::HanseFactory,pirates:Option<p3_core::economy::hanse_petition::PiratePortraits>}
struct TheftPrepared {task:p3_core::scheduler::TaskRecord,towns:Vec<Town>,offices:Vec<Vec<u8>>,stocks:Vec<p3_core::economy::trade::OfficeStock>,merchants:Vec<RepMerchant>,pool:Vec<u8>,updates:Vec<p3_core::economy::world::ScheduledOfficeUpdate>}
struct BurglaryPrepared {core:p3_core::economy::commissioned_burglary::Prepared,stocks:Vec<p3_core::economy::trade::OfficeStock>,pool:Vec<u8>,updates:Vec<p3_core::economy::world::ScheduledOfficeUpdate>}
struct PlaguePrepared {task:p3_core::scheduler::TaskRecord,towns:Vec<Town>,businesses:Vec<[u8;20]>,pool:Vec<u8>,keep:bool,notices:Vec<(u16,u8)>,windows:Vec<(u16,u8)>,offices:Vec<Vec<u8>>,updates:Vec<p3_core::economy::world::ScheduledOfficeUpdate>,captains:p3_core::ships::captains::CaptainPool,pirates:Option<p3_core::economy::hanse_petition::PiratePortraits>}
struct FirePrepared {scheduler:p3_core::scheduler::Scheduler,towns:Vec<Town>,sites:Vec<Site>,pool:Vec<u8>,keep:bool,city:usize,notices:Vec<(u16,u8)>,windows:Vec<(u16,u8)>,offices:Vec<Vec<u8>>,updates:Vec<p3_core::economy::world::ScheduledOfficeUpdate>,businesses:p3_core::economy::facility_pool::FacilityPool,stocks:Vec<p3_core::economy::trade::OfficeStock>,merchants:Vec<RepMerchant>}
struct TownEventPrepared {task:p3_core::scheduler::TaskRecord,city:usize,town:Town,offices:Option<Vec<Vec<u8>>>,stocks:Vec<p3_core::economy::trade::OfficeStock>,merchants:Vec<RepMerchant>,pool:Vec<u8>,updates:Vec<p3_core::economy::world::ScheduledOfficeUpdate>,weapons:Vec<(usize,[i32;4])>,keep:bool,spawn:Option<(town_event_live::Prepared,Vec<Option<p3_save::pirate_bands::PirateBand>>)>}
enum CelebrationPrepared {None,Ai{city:usize,town:Town},Human{city:usize,town:Town,owner:usize,merchant:RepMerchant,office:usize,stock:p3_core::economy::trade::OfficeStock,pool:Vec<u8>,notify:bool,window:bool}}
#[derive(Clone)]
struct RetirementDisband { manager:Vec<u8>, merchants:Vec<u8>, merchant_count:u16, tick:u32, player:u32, distances:Option<p3_core::ships::ai_dock::AiDockData>, notices:Vec<(u8,u8,u8,[u16;4])> }
#[cfg(test)]
mod retirement_disband_tests;
impl p3_core::economy::captain_operations::RetirementHost for RetirementDisband {
    fn disband_convoy(&mut self,id:u16,ships:&mut[u8],convoys:&mut[u8],towns:&mut[Town],pool:&mut p3_core::ships::captains::CaptainPool)->Result<(),String>{
        use p3_core::ships::{convoy_drop::dissolve_convoy,remove::RemoveWorld,tick::{ConvoyHost,TickWorld}};
        struct Hooks { error:Option<String>, distances:Option<p3_core::ships::ai_dock::AiDockData>, notices:Vec<(u8,u8,u8,[u16;4])>, letter_words:[u16;4] }
        impl ConvoyHost for Hooks {
            fn hook(&mut self,name:&'static str,args:&[u32])->u32{match (name,args){("cv_map",[from,to])=>{if let Some(d)=self.distances.as_mut(){return u32::from(p3_core::ships::ai_dock::AiDockHost::distance(d,*from,*to))}self.error.get_or_insert_with(||"matriz de distancias ausente".into());},("msg_refund",[owner,town,ship,name,amount,portrait])=>{self.letter_words=[*amount as u16,*ship as u16,*name as u16,*portrait as u16];self.notices.push((*owner as u8,*town as u8,0x32,self.letter_words))},("msg_total",[owner,town,amount,ship])=>{self.letter_words[0]=*amount as u16;self.letter_words[3]=*ship as u16;self.notices.push((*owner as u8,*town as u8,0x31,self.letter_words))},_=>{self.error.get_or_insert_with(||format!("efecto de disolución pendiente: {name}"));}}0}
            fn read(&mut self,_:u32,_:u8)->u32{self.error.get_or_insert_with(||"lectura de ruta pendiente".into());0}
            fn ship_delivery_with_world(&mut self,w:&mut TickWorld,owner:u8,count:u32,success:u32){
                let at=usize::from(owner)*0x650;
                if w.remove.town_count>40{self.error.get_or_insert_with(||"ciudades de reputación incompatibles".into());return}
                let Some(bytes)=w.remove.merchants.get_mut(at..at+0x650) else{self.error.get_or_insert_with(||"comerciante de convoy inexistente".into());return};
                let mut merchant=RepMerchant{bytes:bytes.to_vec()};let env=p3_core::ai::reputation::RepEnv{town_count:w.remove.town_count,towns:&[],renown:[0;3],byte_52c:0};merchant.apply_ship_delivery_outcome(&env,count,success);bytes.copy_from_slice(&merchant.bytes);
            }
        }
        let count=usize::from(u16_at(&self.manager,0xf4));let ncv=usize::from(u16_at(&self.manager,0xf6));let at=usize::from(id)*0x3c;
        if ships.len()!=count*0x180||convoys.len()!=ncv*0x3c||at+0x3c>convoys.len(){return Err("tablas de convoy truncadas".into())}
        let owner=u16::from(convoys[at]);let town=u16_at(convoys,at+0x1c);let dest=u16::from(convoys[at+0x1f]);
        if owner<self.merchant_count&&convoys[at+0x14]&2==0&&usize::from(town)<towns.len()&&town==dest&&convoys[at+0x1e]!=dest as u8&&self.distances.is_none(){return Err("matriz de distancias ausente".into())}
        if !matches!(u16_at(convoys,at+0x12),0|1|4|5){return Err("estado de disolución pendiente".into())}
        let mut seen=vec![false;count];let mut ship=u16_at(convoys,at+0xa);let leader=u16_at(convoys,at+0x10);let mut found=false;
        while usize::from(ship)<count{let i=usize::from(ship);if seen[i]{return Err("convoy cíclico".into())}seen[i]=true;let s=i*0x180;if u16_at(ships,s+8)!=id{return Err("miembro de convoy incoherente".into())}found|=ship==leader;ship=u16_at(ships,s+6);}
        if !found{return Err("líder ausente del convoy".into())}
        for head in [0xec,0xe4]{let mut seen=vec![false;ncv];let mut cv=u16_at(&self.manager,head);let mut found=false;while usize::from(cv)<ncv{let i=usize::from(cv);if seen[i]{return Err("lista de convoyes cíclica".into())}seen[i]=true;found|=cv==id;cv=u16_at(convoys,i*0x3c+8);}if (head==0xec&&!found)||(head==0xe4&&found){return Err("listas de convoyes incoherentes".into())}}
        let mut town_bytes=towns.iter().flat_map(|t|t.0.iter().copied()).collect::<Vec<_>>();
        let mut remove=RemoveWorld{manager:&mut self.manager,ships,captains:&mut pool.entries,convoys,merchants:&mut self.merchants,towns:&mut town_bytes,pool_limit:0,merchant_count:self.merchant_count,town_count:towns.len() as u32,ui_a:false,ui_b:false};
        let mut world=TickWorld{remove:&mut remove,time:self.tick,player:self.player};let mut hooks=Hooks{error:None,distances:self.distances.clone(),notices:Vec::new(),letter_words:[0;4]};
        dissolve_convoy(&mut world,id,&mut hooks).map_err(|e|format!("disolución: {e:?}"))?;
        if let Some(e)=hooks.error{return Err(e)}
        self.notices.extend(hooks.notices);
        for (town,src) in towns.iter_mut().zip(town_bytes.chunks_exact(0x9f8)){town.0.copy_from_slice(src)}
        Ok(())
    }
}

struct WeatherPrepared {scheduler:p3_core::scheduler::Scheduler,towns:Vec<Town>,marker:Option<u8>,notices:Vec<(u16,bool)>,rng:Option<p3_core::rng::CrtRand>,operations:Vec<p3_core::pump::Operation>}

struct MissionPrepared { notifications:Vec<ScriptNotification>,task:p3_core::scheduler::TaskRecord,vars:Vec<u32>,finished:bool,names:Option<p3_core::mission_script::NameContext>,pool:Option<Vec<u8>>,merchants:Option<Vec<RepMerchant>>,scheduler:Option<p3_core::scheduler::Scheduler>,ship_reserve:Vec<p3_core::mission_script::ShipReserveOp>,operations:Vec<p3_core::pump::Operation>,
    /// Ciudades escritas por comandos sobre imagen plana (`c10_script_flat`) y sus índices.
    flat_towns:Option<(Vec<Town>,Vec<usize>)>,
    /// Bytes de barcos/convoyes y oficinas escritos por esos comandos (`c10_script_flat::FlatOut`).
    flat_ships:Vec<(usize,u8)>,flat_convoys:Vec<(usize,u8)>,flat_offices:Vec<(usize,Vec<u8>)>,
    /// Estado de `rand` del CRT tras las llamadas `0x00639C03` del guion plano.
    flat_rng:Option<p3_core::rng::CrtRand>,
    /// Barcos creados por `0x0050E000` (`0x67`), publicados en todas las vistas navales.
    flat_created:Option<c10_script_flat::CreatedShips> }
/// Efectos de la elección municipal que el despachador publica además de ciudades, comerciantes y buzón.
#[derive(Default)]
struct MayorElectionExtra {operations:Vec<p3_core::pump::Operation>,pirates:Option<[Option<p3_core::ai::settlement::MissionPirateBand>;5]>,notices:Vec<[u32;6]>}
struct AldermanPrepared {task:p3_core::scheduler::TaskRecord,towns:Vec<Town>,merchants:Vec<RepMerchant>,pool:Vec<u8>,player:u8,town:u8,notices:Vec<[u32;6]>,announcements:Vec<(bool,u32,u16)>,extra:MayorElectionExtra}
struct MatchmakerPrepared {task:p3_core::scheduler::TaskRecord,pool:Vec<u8>,merchants:Vec<RepMerchant>}
struct HarvestPrepared {task:p3_core::scheduler::TaskRecord,towns:Vec<Town>,pool:Vec<u8>,merchants:Vec<RepMerchant>,keep:bool}
struct SupplyDeadlinePrepared {task:p3_core::scheduler::TaskRecord,towns:Vec<Town>,pool:Vec<u8>,merchants:Vec<RepMerchant>,keep:bool}
struct ShipyardWeeklyPrepared {towns:Vec<Town>,pool:Vec<u8>,merchants:Vec<RepMerchant>}
struct AuctionShipPrepared {task:p3_core::scheduler::TaskRecord,before:p3_core::economy::auction_ship::LotFleet,after:p3_core::economy::auction_ship::LotFleet,ship:u16}
struct AuctionLotPrepared {task:p3_core::scheduler::TaskRecord,result:Option<p3_core::economy::auction::AuctionResult>,delivery:Option<(usize,RepMerchant,Vec<u8> )>,sale:Option<AuctionSalePrepared>,ship:Option<AuctionShipAwardPrepared>}
struct AuctionShipAwardPrepared {before:p3_core::economy::auction_ship::LotFleet,after:p3_core::economy::auction_ship::LotFleet,merchants:Vec<RepMerchant>,pool:Vec<u8>,
    /// Reventa: grupos podados del antiguo dueño (0x004F5900).
    groups:Option<(usize,p3_save::world::MerchantShipGroups)>,
    /// Parada de ruta liberada (0x004D4DD0).
    route_stops:Option<route_stop_runtime::RouteStopRuntime>}
struct AuctionSalePrepared {snapshot:auction_transfer::TransferSnapshot,merchants:Vec<RepMerchant>,pool:Vec<u8>,updates:Vec<p3_core::economy::world::ScheduledOfficeUpdate>,city:usize}

struct MerchantAiPrepared {
    task:p3_core::scheduler::TaskRecord,
    delay:u32,
    stock_changes:Vec<(usize,[i32;24],u16,Vec<u8>)>,
    grant:Option<(usize,i32)>,
    operations:Vec<p3_core::pump::Operation>,
    rep_factor:Option<(usize,u32)>,
    rng:Option<p3_core::rng::CrtRand>,
    fleet_changes:Vec<(usize,u8,u8,u8)>,
}

fn reconcile_event_band(base:&p3_save::pirate_bands::PirateBand,now:&p3_save::pirate_bands::PirateBand,mission:&p3_core::ai::settlement::MissionPirateBand,last:Option<&p3_save::pirate_bands::PirateBand>,index:usize)->Result<p3_save::pirate_bands::PirateBand,String>{
    let mut shared=base.header;shared[4..6].copy_from_slice(&mission.first_ship.to_le_bytes());shared[6]=mission.target;
    shared[12]=mission.status;shared[14..16].copy_from_slice(&mission.reserve_strength.to_le_bytes());
    let mut desired=now.clone();
    for (start,end) in [(4,6),(6,7),(12,13),(14,16)] {
        if now.header[start..end]!=base.header[start..end]&&now.header[start..end]!=shared[start..end]&&last.is_none_or(|band|now.header[start..end]!=band.header[start..end]){return Err(format!("vistas de banda {index} desincronizadas"))}
        desired.header[start..end].copy_from_slice(&shared[start..end]);
    }
    Ok(desired)
}

struct CouncilExecution<'a> {
    game_end_start:u32,game_end_mode:u8,game_end_top_ranked:Option<u16>,game_end_score_flag:Option<u8>,game_end_stopped:&'a mut bool,game_end_events:&'a mut Vec<p3_core::economy::game_end::Event>,
    return_resources: &'a Option<expedition_return_resources::Resources>,
    shipyard_detached: bool,
    return_pool_limit: Option<u16>,
    return_map_dirty: &'a mut bool,
    return_centers: &'a mut Option<Vec<Vec<u8>>>,
    return_center_versions: &'a mut Vec<std::collections::BTreeSet<[u8;32]>>,
    return_map: &'a mut Vec<u8>,
    return_flag: &'a mut u8,
    return_points: &'a [(i16,i16)],
    return_planner: &'a mut Option<p3_core::route::finder::RoutePlanner>,
    return_notices: &'a mut Vec<expedition_finish_host::Notification>,
    return_map_notices: &'a mut Vec<[u32;6]>,
    return_map_windows: &'a mut Vec<[u32;3]>,
    land_transport_notifications: &'a mut Vec<[u32;6]>,
    trade_offers: &'a mut Option<Vec<[u8;16]>>,

    wedding_notifications:&'a mut Vec<[u32;6]>,wedding_windows:&'a mut Vec<(u16,u8)>,
    siege_headless: bool,
    siege_records:Option<&'a [Vec<u8>]>,
    siege_live:&'a [Option<LiveWalledSiege>],
    town_hall_notices:&'a mut Option<[u8;84]>,
    election_texts:&'a Option<c08_text::ElectionTexts>,hunt_texts:&'a Option<c08_hunt::HuntTexts>,hunt_bands:&'a mut Option<[Option<p3_core::ai::settlement::MissionPirateBand>;5]>,event_band_initial:&'a Option<Vec<Option<p3_save::pirate_bands::PirateBand>>>,event_band_saved:&'a Option<Vec<Option<p3_save::pirate_bands::PirateBand>>>,event_band_images:&'a mut Option<Vec<Option<p3_save::pirate_bands::PirateBand>>>,event_battle_difficulty:u8,event_pirate_difficulty:u8,event_inland:&'a Option<[u8;40]>,event_piece_bytes:Option<(u8,u8)>,election_notifications:&'a mut Vec<[u32;6]>,election_announcements:&'a mut Vec<(bool,u32,u16)>,
    burn_lists: &'a mut Vec<OfficeLists>, burn_housing: &'a mut Vec<OfficeHousing>, burn_updates: &'a mut Vec<(usize,u16,u16)>,
    pending_office:&'a mut Option<(u8,u8,u32)>,pending_founding:&'a mut Option<(u16,p3_core::scheduler::TaskRecord)>,pending_pirate_approach:&'a mut Option<(u16,p3_core::scheduler::TaskRecord)>,pending_pirate_attack:&'a mut Option<(u16,p3_core::scheduler::TaskRecord)>,pending_auction_office:&'a mut Option<(u16,p3_core::scheduler::TaskRecord,usize)>,
    pending_ai_expansion:&'a mut Option<(u16,p3_core::scheduler::TaskRecord,p3_core::scheduler::TaskRecord,u8,u8)>,
    candidate_name_lengths:&'a Option<Vec<u8>>,candidate_look_counts:&'a Option<(u16,u16)>,
    mission_bank:&'a Option<mission_bank::MissionBank>,mission_variables:&'a mut Vec<Option<Vec<u32>>>,hunt_losses:&'a mut std::collections::BTreeMap<usize,Vec<p3_save::tail::HuntLoss>>,
    employment_businesses:&'a mut Vec<[u8;20]>,business_free:&'a mut u16,
    world:&'a mut EconomyWorld, merchants:&'a mut [RepMerchant], sites:&'a mut [Site],navigation:&'a mut Option<navigation::LiveVoyage>,navigation_ai:&'a Option<p3_core::ships::ai_dock::AiDockData>,scheduled_office_images:&'a mut Option<Vec<Vec<u8>>>,scheduled_office_updates:&'a mut Vec<p3_core::economy::world::ScheduledOfficeUpdate>,
    pool:&'a mut Vec<u8>, enabled:&'a [bool], building_enabled:&'a [bool], notifications:&'a mut Vec<(u16,u8)>,celebration_notifications:&'a mut Vec<(u16,u8)>,celebration_windows:&'a mut Vec<(u16,u8)>,plague_notifications:&'a mut Vec<(u16,u8)>,plague_windows:&'a mut Vec<(u16,u8)>,fire_notifications:&'a mut Vec<(u16,u8)>,fire_windows:&'a mut Vec<(u16,u8)>,
    mission_texts:&'a Option<mission_text::ScriptTextBank>,mission_distances:Option<&'a [u8]>,mission_town_names:&'a Option<town_names::TownNameBank>,mission_family_names:&'a Option<Vec<Vec<u8>>>,mission_first_names:&'a Option<[Vec<Vec<u8>>;2]>,weather_marker:&'a mut Option<u8>,weather_speed:u32,weather_notices:&'a mut Vec<(u16,bool)>,rep_scale:u8,operations_mode:u32,/// `0x004FDF50` (capitanes de taberna): nombres D0 y apellidos ordinarios D4, sin el banco pirata D6.
    captain_names:Option<(u16,u16)>,loan_names:Option<(u16,u16)>,ordinary_family_names:Option<u16>,investigation_rng:&'a mut Option<p3_core::rng::CrtRand>,ai_work_residue:Option<u32>,reminder_gate:Option<u32>,reminder_notices:&'a mut Vec<(u16,u8)>,celebration_state:&'a mut Option<p3_core::economy::celebration::CelebrationState>,investigation_local:bool,investigation_notifications:&'a mut Vec<(u16,u8)>,investigation_towns:&'a mut Vec<usize>,
    errors:&'a mut Vec<String>, auction_notices:&'a mut Vec<(u16,u8)>, auction_finished:&'a mut Vec<u8>, auction_results:&'a mut Vec<p3_core::economy::auction::AuctionResult>, offices:&'a [p3_core::economy::extra_tax::TaxOffice], tick:u32, difficulty:f32, letters:Vec<(u8,u16,u8,u32,u32)>,
    hanse_notifications:&'a mut Vec<(u16,u8,u8)>,debt_notifications:&'a mut Vec<(u16,u8)>,
    seizure_notifications:&'a mut Vec<(u16,u8,u8)>, stocks:&'a mut [p3_core::economy::trade::OfficeStock],
    fleet:&'a mut Vec<ShipCargo>, ship_metadata:&'a mut Vec<(u16,u16,u16,u16)>, changed_ships:&'a mut Vec<usize>,
    changed_towns:&'a mut Vec<usize>,
    vote_notifications:&'a mut Vec<(u16,u8)>, vote_windows:&'a mut Vec<u8>,
    financial_towns:&'a mut Vec<usize>,
    convoy_flags:&'a mut Vec<u32>, changed_convoys:&'a mut Vec<usize>,
    pirates:&'a mut Option<p3_core::economy::hanse_petition::PiratePortraits>,
    mission_town_ids:&'a [u8],mission_pirate_targets:&'a mut Option<[Option<u8>;5]>,
    mission_locations:&'a Option<[p3_core::ai::settlement::SettlementLocation;40]>,
    mission_notifications:&'a mut Vec<[u32;6]>,script_notifications:&'a mut Vec<ScriptNotification>,mission_ui:&'a mut Vec<(u16,u8)>,
    mission_failure_ui:&'a mut Vec<(u16,u8)>,
    mission_changed_towns:&'a mut Vec<usize>,mission_flat_rdata:&'a Option<Vec<u8>>,mission_settings:&'a Option<Vec<u8>>,mission_changed_sites:&'a mut Vec<usize>,mayor_statue_visuals:&'a mut Vec<(u8,u8,u8,u8,u8)>,road_build_cached:Option<(p3_core::scheduler::TaskRecord,Vec<Town>,Vec<Site>,[usize;2])>,mission_cached:Option<(u16,p3_core::scheduler::TaskRecord,MissionPrepared)>,
    expedition_factory:&'a mut Option<p3_core::ships::factory::HanseFactory>,expedition_resources:&'a Option<HanseFleetResources>,expedition_notifications:&'a mut Vec<(u16,u32,u8)>,expedition_windows:&'a mut Vec<u8>,expedition_original_images:&'a mut Vec<Option<Vec<u8>>>,
    expedition_operations:&'a mut Vec<p3_core::pump::Operation>,expedition_towns:&'a mut Vec<usize>,expedition_images:&'a mut Vec<Option<Vec<u8>>>,expedition_convoys:&'a mut Vec<Vec<u8>>,expedition_docked:&'a mut u16,expedition_sea_head:u16,expedition_head:&'a mut u16,expedition_changed_ships:&'a mut Vec<usize>,ai_fleet_changed_ships:&'a mut Vec<usize>,expedition_changed_convoys:&'a mut Vec<usize>,
    route_stops:&'a mut Option<route_stop_runtime::RouteStopRuntime>,merchant_groups:&'a mut Vec<p3_save::world::MerchantShipGroups>,merchant_groups_changed:&'a mut Vec<usize>,
}
impl p3_core::economy::captain_offer::Host for RetirementDisband {
    fn disband(&mut self,id:u16,ships:&mut[u8],convoys:&mut[u8],towns:&mut[Town],pool:&mut p3_core::ships::captains::CaptainPool,merchants:&mut[RepMerchant])->Result<(),String>{
        self.merchants=merchants.iter().take(usize::from(self.merchant_count)).flat_map(|m|m.bytes.iter().copied()).collect();
        p3_core::economy::captain_operations::RetirementHost::disband_convoy(self,id,ships,convoys,towns,pool)?;
        for (person,bytes) in merchants.iter_mut().zip(self.merchants.chunks_exact(0x650)){person.bytes.copy_from_slice(bytes)}Ok(())
    }
}
impl CouncilExecution<'_> {
    fn game_end_preview(&self,s:&p3_core::scheduler::Scheduler)->Result<p3_core::economy::game_end::Prepared,String>{
        let month=Calendar::for_tick(self.tick).month;let mut c=p3_core::economy::game_end::Context{start:self.game_end_start,tick:self.tick,month,game_mode:self.game_end_mode,player:self.world.local_merchant,merchant_count:self.world.merchant_count,merchants:self.merchants,towns:&self.world.towns,alderman:self.world.alderman_merchant,top_ranked:self.game_end_top_ranked};match p3_core::economy::game_end::prepare(s,&c){Err(e) if e=="clasificacion final no disponible"=>{let offices=self.scheduled_office_images.as_ref().ok_or("oficinas de clasificacion ausentes")?;c.top_ranked=Some(p3_core::economy::game_end_ranking::winner(self.merchants,&self.world.towns,offices,self.world.merchant_count,self.game_end_score_flag)?);p3_core::economy::game_end::prepare(s,&c)},other=>other}
    }
    fn traveller_deadline_preview(&self,s:&p3_core::scheduler::Scheduler)->Result<(p3_core::economy::tavern_traveller_deadline::Prepared,Vec<u8>),String>{
        let (ships,convoys)=if let Some(nav)=self.navigation.as_ref(){(nav.snapshot.ships.clone(),nav.snapshot.convoys.clone())}else{let f=self.expedition_factory.as_ref().ok_or("flota para pasajeros ausente")?;(f.ships.iter().flatten().copied().collect(),f.convoys.iter().flatten().copied().collect())};
        let count=usize::from(self.world.merchant_count);if count>self.merchants.len()||count>self.world.merchants.len(){return Err("comerciantes para pasajero incompletos".into())}
        let mut people=self.merchants.to_vec();for (person,account) in people.iter_mut().zip(&self.world.merchants).take(count){person.set_i32(0,account.gold);person.set_i32(0x4c8,account.income)}
        let mut out=p3_core::economy::tavern_traveller_deadline::prepare(s,&ships,&convoys,&people,self.world.merchant_count,self.world.towns.len() as u32,self.tick)?;
        let mut pool=self.pool.clone();if let Some(n)=&out.notice{if usize::from(n.recipient)<count{let date=Calendar::for_tick(self.tick);let retention=[u32::MAX,0x2800,0x1400,0xa00,0x500,0];p3_state::messages::post_to_merchant(&mut pool,&mut out.merchants[usize::from(n.recipient)],date.day,date.month,date.year,&n.record,p3_state::messages::Payload::None,&retention)?;}}
        Ok((out,pool))
    }
    /// Prepara la fase inicial de 0x12 sin mutar la tarea ni las vistas vivas.
    /// La fase de regreso permanece rechazada hasta conectar sus efectos.
    fn expedition_return_start_preview(&self,s:&p3_core::scheduler::Scheduler)->Result<(p3_core::scheduler::TaskRecord,bool),String>{
        let mut task=*s.records.get(usize::from(s.head)).ok_or("tarea de expedición ausente")?;
        let (convoys,count)=if let Some(nav)=self.navigation.as_ref(){
            let manager=&nav.snapshot.manager;if manager.len()<0xf8{return Err("gestor de convoyes truncado".into())}
            let count=u16_at(manager,0xf6);if nav.snapshot.convoys.len()!=usize::from(count)*0x3c{return Err("pool de convoyes truncado".into())}
            (nav.snapshot.convoys.clone(),count)
        }else{
            let factory=self.expedition_factory.as_ref().ok_or("pool de convoyes ausente")?;
            let count=u16::try_from(factory.convoys.len()).map_err(|_|"demasiados convoyes")?;
            if factory.convoys.iter().any(|c|c.len()!=0x3c){return Err("convoy truncado".into())}
            (factory.convoys.concat(),count)
        };
        let keep=p3_core::economy::expedition_return_start::start(&mut task,&convoys,count).map_err(str::to_owned)?;
        Ok((task,keep))
    }
    /// Prepara la fase de regreso sin publicar ninguna de sus vistas.
    fn expedition_return_preview(&self,s:&p3_core::scheduler::Scheduler)->Result<expedition_return_stage::Staged<expedition_return_resources::CenterAdapter>,String>{
        let task=s.records.get(usize::from(s.head)).ok_or("tarea 0x12 ausente")?;
        let nav=self.navigation.as_ref().ok_or("navegación viva de expedición ausente")?;
        let resources=self.return_resources.as_ref().ok_or("recursos del ejecutable de expedición ausentes")?;
        let count=self.world.merchant_count;
        let id=u16::from_le_bytes([task.u8_at(8),task.u8_at(9)]);
        let off=usize::from(id)*0x3c;
        let convoy=nav.snapshot.convoys.get(off..off+0x3c).ok_or("convoy de expedición ausente")?;
        let word=|o|u16::from_le_bytes([convoy[o],convoy[o+1]]);
        let points=std::array::from_fn(|j|(word(0x24+j*4),word(0x26+j*4)));
        let locations=resources.trade.locations.to_vec();
        let map_points=self.return_points.iter().map(|&(x,y)|(x as u16,y as u16)).collect::<Vec<_>>();
        let centers=self.return_centers.as_ref().ok_or("centros de expedición ausentes")?.clone();
        if self.world.merchants.len()<usize::from(count)||self.merchants.len()<usize::from(count){return Err("cuentas de expedición incompletas".into())}
        let mut people=self.merchants.to_vec();
        for (person,account) in people.iter_mut().zip(self.world.merchants.iter()).take(usize::from(count)){person.set_i32(0,account.gold);person.set_i32(0x4c8,account.income)}
        let state=expedition_visit::VisitState{merchants:people,global_map:self.return_map.clone(),world_flag:*self.return_flag,message_pool:self.pool.clone(),centers,notifications:vec![],windows:vec![],dirty:false};
        let calendar=p3_core::calendar::Calendar::for_tick(self.tick);
        let input=expedition_visit::VisitInput{waypoints:points,owner:convoy[0],merchant_count:count,player:nav.player,large_renown:resources.first_discovery_renown,tick:self.tick,calendar,mode:word(0x38),dirty_target:false,locations:&locations,discovery_points:&map_points};
        let bank=self.mission_first_names.as_ref().zip(self.mission_family_names.as_ref()).map(|(given,family)|expedition_return_resources::CaptainNameBank{given,family});
        let leader=usize::from(word(0x10));let ship=nav.snapshot.ships.get(leader*0x180+0x160..leader*0x180+0x180);
        let city=if usize::from(convoy[0x1f])<self.world.towns.len(){usize::from(convoy[0x1f])}else if !self.world.towns.is_empty(){usize::from(convoy[0x1e])%self.world.towns.len()}else{return Err("ciudades de expedición ausentes".into())};
        let town=self.mission_town_ids.get(city).zip(self.mission_town_names.as_ref()).and_then(|(&map,bank)|bank.for_map(map)).map(|p|p.0);
        let letter=ship.zip(town).map(|(ship,town)|expedition_return_stage::LetterContext{texts:&resources.texts,ship,town,captain_names:bank.as_ref().map(|b|b as &dyn expedition_return_stage::CaptainNames)});
        let center_host=if points.iter().all(|p|p.0>=0x7fff){expedition_return_resources::CenterAdapter::NoCenters(expedition_return_resources::NoCenters(resources.no_contract_text.clone()))}else{
            let settings=self.expedition_resources.as_ref().ok_or("modo de presentación de expedición ausente")?;
            let trade=&resources.trade;
            let host=expedition_center_host::NativeCenterHost{convoy:id,convoys:nav.snapshot.convoys.clone(),ships:nav.snapshot.ships.chunks_exact(0x180).map(|s|s.to_vec()).collect(),captains:nav.snapshot.captains.clone(),town_count:self.world.towns.len() as u32,tick:self.tick,player:nav.player,hotseat:settings.hotseat,date:calendar,renown_small:trade.renown,report_texts:Some(resources.trade_texts.clone()),trade_text:vec![],contract_texts:Some(resources.contract_texts.clone()),contract_text:resources.contract_header.clone(),populations:self.world.towns.iter().map(|t|t.i32(0x2d4)).collect(),tables:expedition_center_host::PriceTables{base:trade.base,units:trade.units,sell_caps:trade.sell_caps,buy_caps:trade.buy_caps}};
            expedition_return_resources::CenterAdapter::Native{host,no_contract_suffix:resources.no_contract_suffix.clone()}
        };
        expedition_return_stage::prepare(expedition_return_stage::Inputs{nav,towns:&self.world.towns,state:&state,host:&center_host,visit:input,planner:self.return_planner.as_ref(),tick:self.tick,due:task.due,start:task.u32_at(0xc),convoy:id,difficulty:resources.weapon_strength,renown_delta:resources.finish_renown,pool_limit:self.return_pool_limit.ok_or("capacidad del pool naval auxiliar ausente")?,letter})
    }
    fn event_bands_now(&self)->Result<Vec<Option<p3_save::pirate_bands::PirateBand>>,String>{
        let current=self.event_band_images.as_ref().ok_or("bandas de suceso ausentes")?;
        if current.len()!=5{return Err("bandas de suceso incompletas".into())}
        current.iter().enumerate().map(|(i,image)|match image{
            None=>Ok(None),Some(image)=>{
                let mission=self.hunt_bands.as_ref().and_then(|v|v[i].as_ref()).ok_or("vista de misión de banda ausente")?;
                let last=self.event_band_saved.as_ref().and_then(|v|v.get(i)).and_then(Option::as_ref);
                let base=self.event_band_initial.as_ref().and_then(|v|v.get(i)).and_then(Option::as_ref).or(last).unwrap_or(image);
                reconcile_event_band(base,image,mission,last,i).map(Some)
            }
        }).collect()
    }
    fn mayor_statue_preview(&self,s:&p3_core::scheduler::Scheduler)->Result<(p3_core::scheduler::TaskRecord,Vec<Site>,Vec<RepMerchant>,Vec<u8>,mayor_statue::Outcome),String>{
        let mut task=*s.records.get(usize::from(s.head)).ok_or("tarea de estatua ausente")?;
        let count=usize::from(self.world.merchant_count);
        if count>self.merchants.len(){return Err("comerciantes de estatua incompletos".into())}
        let sea_x=if let Some(nav)=self.navigation.as_ref(){
            if nav.snapshot.manager.len()<0xf6{return Err("gestor naval truncado".into())}
            let head=usize::from(u16_at(&nav.snapshot.manager,0xea));let count=usize::from(u16_at(&nav.snapshot.manager,0xf4));
            if head<count{let image=nav.snapshot.ships.get(head*0x180..(head+1)*0x180).ok_or("barco en el mar ausente")?;Some(u32_at(image,0x1c))}else{None}
        }else{
            let head=usize::from(self.expedition_sea_head);
            if head<self.expedition_images.len(){let image=self.expedition_images[head].as_ref().ok_or("barco en el mar ausente")?;if image.len()<0x20{return Err("barco en el mar truncado".into())}Some(u32_at(image,0x1c))}else{None}
        };
        let mut sites=self.sites.to_vec();let mut merchants=self.merchants.to_vec();let mut pool=self.pool.clone();
        let outcome=mayor_statue::advance(&mut task,s,&self.world.towns,&mut sites,&merchants[..count],sea_x,self.tick)?;
        if let Some(letter)=outcome.letter{let date=p3_core::calendar::Calendar::for_tick(self.tick);p3_state::messages::add_global_inline_letter(&mut pool,date.day,date.month,date.year,&letter)?;for merchant in &mut merchants[..count]{p3_state::messages::add_mayor_statue_notice(&mut pool,merchant,date.day,date.month,date.year,&letter)?;}}
        Ok((task,sites,merchants,pool,outcome))
    }
    fn auction_missing_lot_preview(&self,s:&p3_core::scheduler::Scheduler)->Result<AuctionLotPrepared,String>{
        let mut task=*s.records.get(usize::from(s.head)).ok_or("tarea de subasta ausente")?;match p3_core::economy::auction::skip_missing_lot(&mut task,&self.world.towns,self.world.ship_count){
            Ok(())=>Ok(AuctionLotPrepared{task,result:None,delivery:None,sale:None,ship:None}),
            Err(_e) if task.kind==0x0a=>self.auction_ship_award_preview(s),
            Err(_e) if task.kind==0x36=>{
                let(result,owner,self_buy)=match p3_core::economy::auction::unsold_building(&mut task,&self.world.towns,self.sites,self.world.merchant_count){Ok((result,owner))=>(result,owner,false),Err(_)=>{match p3_core::economy::auction::self_bought_building(&mut task,&self.world.towns,self.sites,self.world.merchant_count){Ok(result)=>(result,Some(result.buyer),true),Err(_)=>{let(result,seller)=p3_core::economy::auction::bought_building(&mut task,&self.world.towns,self.sites,self.world.merchant_count).map_err(str::to_owned)?;let pending=self.pending_auction_office.as_ref().filter(|(head,original,_)|*head==s.head&&*original==s.records[usize::from(s.head)]).map(|(_,_,capacity)|*capacity);let sale=self.auction_sale_preview(result,seller,pending)?;return Ok(AuctionLotPrepared{task,result:Some(result),delivery:None,sale:Some(sale),ship:None})}}}};
                let delivery=if let Some(owner)=owner{let index=usize::from(owner);let mut merchant=self.merchants.get(index).ok_or("propietario de subasta ausente")?.clone();if merchant.bytes.len()<0x4c0||self.world.merchants.get(index).is_none(){return Err("propietario de subasta truncado".into())}let mut letter=[0u8;16];letter[4]=if self_buy{0x7d}else{0x7e};letter[5]=result.city;letter[8]=merchant.bytes[0x1c];letter[9]=merchant.bytes[0x1d];if self_buy{let site=&self.sites[usize::from(result.city)];letter[13]=site.records[usize::from(result.object)*8+3];letter[14..16].copy_from_slice(&(result.price/100).to_le_bytes())}let mut pool=self.pool.clone();let date=p3_core::calendar::Calendar::for_tick(self.tick);p3_state::messages::add_building_auction_notice(&mut pool,&mut merchant,date.day,date.month,date.year,&letter)?;if self_buy{let paid=i32::from(result.price);let net=paid*9/10;merchant.set_i32(0,merchant.i32(0).wrapping_add(net).wrapping_sub(paid));merchant.set_i32(0x4b8,merchant.i32(0x4b8).wrapping_add(net));merchant.set_i32(0x4bc,merchant.i32(0x4bc).wrapping_add(paid));}Some((index,merchant,pool))}else{None};Ok(AuctionLotPrepared{task,result:Some(result),delivery,sale:None,ship:None})
            },Err(e)=>Err(e.to_owned())
        }
    }
    /// Fase 7 con lote naval existente (0x004E31D6): cartas, traspaso 0x00509250 y lote siguiente, sobre copias.
    fn auction_ship_award_preview(&self,s:&p3_core::scheduler::Scheduler)->Result<AuctionLotPrepared,String>{
        use p3_core::economy::auction_ship::{award_lot_ship,AwardWorld,LotFleet};
        let mut task=*s.records.get(usize::from(s.head)).ok_or("tarea de subasta ausente")?;
        let factory=self.expedition_factory.as_ref().ok_or("fábrica naval de subasta ausente")?;let count=usize::from(self.world.ship_count);
        if factory.ships.len()!=count||self.world.ships.len()!=count||self.fleet.len()!=count||self.ship_metadata.len()!=count||self.expedition_images.len()!=count{return Err("vistas de flota de subasta incompletas".into())}
        let(ships,convoy_count)=if let Some(nav)=self.navigation.as_ref(){let snap=&nav.snapshot;if snap.manager.len()<0xf8||snap.ships.len()!=count*0x180||snap.routes.len()!=count||usize::from(u16_at(&snap.manager,0xf4))!=count{return Err("gestor naval de subasta desincronizado".into())}(snap.ships.chunks_exact(0x180).map(<[u8]>::to_vec).collect::<Vec<_>>(),usize::from(u16_at(&snap.manager,0xf6)))}else{(factory.ships.iter().enumerate().map(|(i,free)|self.expedition_images[i].as_ref().unwrap_or(free).clone()).collect(),self.expedition_convoys.len())};
        if let Some(p)=self.pirates.as_ref(){if p.next.len()!=count||p.ship_captains.len()!=count{return Err("vistas piratas de subasta incompletas".into())}}
        let lot=usize::from(u16::from_le_bytes([task.u8_at(8),task.u8_at(9)]));let image=ships.get(lot).ok_or("lote naval inexistente")?;
        // Inalcanzable: la fase 0 crea el lote fuera de convoy y la operación 0x36 (0x004DA510) lo exige.
        if usize::from(u16_at(image,8))<convoy_count{return Err("lote naval en convoy: inalcanzable desde la fase 0 y la operación 0x36".into())}
        let list_count=match self.route_stops.as_ref(){Some(r)=>u16_at(r.bytes(),2),None if u16_at(image,0x132)==u16::MAX=>0,None=>return Err("lote naval con parada de ruta sin depósito S05".into())};
        let mc=usize::from(self.world.merchant_count);let people=self.merchants.get(..mc).ok_or("comerciantes de subasta ausentes")?;
        let mut bytes=people.iter().map(|m|m.bytes.clone()).collect::<Vec<_>>();
        let before=LotFleet{ships,free_ship:factory.free_ship,docked_head:*self.expedition_docked,name_cursor:factory.name_cursor,name_step:factory.name_step};
        let mut after=before.clone();
        let w=AwardWorld{towns:&self.world.towns,tick:self.tick,merchant_count:self.world.merchant_count,list_count};
        let award=award_lot_ship(&mut task,&w,&mut after.ships,&mut [],&mut bytes)?;
        // Las órdenes solo salen de un lote navegando (+0x38/+0x39 fuera de rango): la fase 0 y 0x36 lo crean en puerto.
        if !award.orders.is_empty(){return Err("traspaso naval con orden de viaje 0x00535760: inalcanzable con el lote en puerto".into())}
        let mut merchants=self.merchants.to_vec();for(m,b)in merchants.iter_mut().zip(bytes){m.bytes=b}
        let route_stops=match award.unlist{Some(id)=>{let mut r=self.route_stops.clone().expect("validado");r.release(id)?;Some(r)},None=>None};
        let groups=match award.prune{Some(old)=>{let o=usize::from(old);let mut lists=self.merchant_groups.get(o).ok_or("grupos navales del vendedor ausentes")?.clone();
            for(slot,list)in lists.iter().enumerate(){let n=usize::from(merchants[o].u16(0x60c+2*slot)&0x7fff);if n>list.len(){return Err("grupo naval del vendedor truncado".into())}if list[..n].iter().any(|&id|usize::from(id)>=after.ships.len()){return Err("grupo naval del vendedor inválido".into())}}
            // En la poda el lote ya no tiene dueño (+0 = 0xFF).
            let ships=&after.ships;let lot=lot as u16;merchants[o].prune_foreign_ships_from_groups(old,&mut lists,&|id|if id==lot{0xff}else{ships[usize::from(id)][0]});Some((o,lists))},None=>None};
        let mut pool=self.pool.clone();let date=p3_core::calendar::Calendar::for_tick(self.tick);
        for(to,letter)in &award.letters{p3_state::messages::post_message(&mut pool,&mut merchants,self.world.merchant_count,date.day,date.month,date.year,i32::from(*to),letter,p3_state::messages::Payload::None,&p3_state::messages::RETENTION)?;}
        Ok(AuctionLotPrepared{task,result:award.result,delivery:None,sale:None,ship:Some(AuctionShipAwardPrepared{before,after,merchants,pool,groups,route_stops})})
    }
    fn auction_sale_letters(result:p3_core::economy::auction::AuctionResult,seller:u8,site:&Site,merchants:&mut [RepMerchant],pool:&mut Vec<u8>,tick:u32)->Result<(),String>{
        let buyer=usize::from(result.buyer);let old=usize::from(seller);
        if buyer>=merchants.len()||merchants[buyer].bytes.len()<0x3a{return Err("comprador de subasta ausente".into())}
        // El original lee fuera del array el nombre del vendedor público.
        // Solo se admite si el comprador tiene desactivada esa carta.
        if old>=merchants.len(){if merchants[buyer].bytes[0x39]!=5{return Err("nombre de vendedor público fuera del array; carta activa".into())}return Ok(())}
        if merchants[old].bytes.len()<0x1e{return Err("vendedor de subasta ausente".into())}
        let kind=*site.records.get(usize::from(result.object)*8+3).ok_or("edificio de subasta ausente")?;
        let names=[merchants[old].bytes[0x1c],merchants[old].bytes[0x1d],merchants[buyer].bytes[0x1c],merchants[buyer].bytes[0x1d]];let date=p3_core::calendar::Calendar::for_tick(tick);
        for (target,code,order) in [(old,0x76,[0,1,2,3]),(buyer,0x7f,[2,3,0,1])]{let mut letter=[0u8;16];letter[4]=code;letter[5]=result.city;for i in 0..4{letter[8+i]=names[order[i]]}letter[13]=kind;letter[14..16].copy_from_slice(&(result.price/100).to_le_bytes());p3_state::messages::add_building_auction_notice(pool,&mut merchants[target],date.day,date.month,date.year,&letter)?;}
        Ok(())
    }
    fn auction_sale_preview(&self,result:p3_core::economy::auction::AuctionResult,seller:u8,pending:Option<usize>)->Result<AuctionSalePrepared,String>{
        use p3_core::buildings::transfer::TransferMerchant;
        let city=usize::from(result.city);let count=usize::from(self.world.merchant_count);
        let images=self.scheduled_office_images.as_ref().ok_or("oficinas de subasta ausentes")?;
        if images.len()!=self.world.offices.len()||self.stocks.len()<images.len()||self.burn_lists.len()<images.len()||self.burn_housing.len()<images.len(){return Err("vistas de oficina de subasta incompletas".into())}
        if self.merchants.len()<count||self.world.merchants.len()<count||self.merchants[..count].iter().any(|m|m.bytes.len()<0x4c0){return Err("comerciantes de subasta incompletos".into())}
        if usize::from(seller)>=count&&self.merchants[usize::from(result.buyer)].bytes[0x39]!=5{return Err("nombre de vendedor público fuera del array; carta activa".into())}
        let mut facilities=p3_core::economy::facility_pool::FacilityPool{records:self.employment_businesses.clone(),free:*self.business_free};
        if facilities.records.len()!=self.world.merchant_facilities.len(){return Err("negocios de subasta incompletos".into())}
        for (b,f) in facilities.records.iter_mut().zip(&self.world.merchant_facilities){b[4..6].copy_from_slice(&f.employees.to_le_bytes());b[8..10].copy_from_slice(&f.next.to_le_bytes());}
        let accounts=self.merchants[..count].iter().map(|m|TransferMerchant{gold:m.i32(0),sold:m.i32(0x4b8),bought:m.i32(0x4bc)}).collect::<Vec<_>>();
        let town=self.world.towns.get(city).ok_or("ciudad de subasta ausente")?;let site=self.sites.get(city).ok_or("plano de subasta ausente")?;
        let snapshot=auction_transfer::transfer_existing_offices(town,site,images,&accounts,&facilities,result.object,result.buyer,result.price,pending.is_some_and(|capacity|images.len()>capacity))?;
        if snapshot.facilities.records.len()>u16::MAX as usize{return Err("pool de negocios excesivo".into())}
        let mut merchants=self.merchants.to_vec();for (m,a) in merchants.iter_mut().zip(&snapshot.merchants){m.set_i32(0,a.gold);m.set_i32(0x4b8,a.sold);m.set_i32(0x4bc,a.bought)}
        let mut pool=self.pool.clone();if pending.is_none(){Self::auction_sale_letters(result,seller,site,&mut merchants,&mut pool,self.tick)?;}
        let mut updates=Vec::new();for &index in &snapshot.changed_offices{let image=&snapshot.offices[index];let admin=u16_at(image,0x2f2);let person=self.expedition_factory.as_ref().and_then(|f|f.captains.get(usize::from(admin)));updates.push(p3_core::economy::world::ScheduledOfficeUpdate{index,administrator:admin,flags:u16_at(image,0x2d6),experience:person.map(|p|p[0xa]),wage:person.map(|p|u16_at(p,0xc)),image:Some(image.clone())});}
        Ok(AuctionSalePrepared{snapshot,merchants,pool,updates,city})
    }
    fn auction_siege_active(&self,city:usize)->Result<bool,String>{
        let record=self.siege_records.and_then(|v|v.get(city)).ok_or("asedio para tasación ausente")?;let saved=p3_save::world::siege_scene(record,city).map_err(|e|format!("asedio inválido: {e:?}"))?;let live=self.siege_live.get(city).and_then(Option::as_ref);match(saved,live){(None,None)=>Ok(false),(None,Some(_))=>Err("escena de asedio sin registro".into()),(Some(_),Some(scene))=>p3_core::economy::auction::siege_active(Some(&scene.image)).map_err(str::to_owned),(Some(saved),None)=>{let mut image=vec![0u8;0x650];p3_save::siege_scene::decode_fields(saved,&mut image).map_err(str::to_owned)?;p3_core::economy::auction::siege_active(Some(&image)).map_err(str::to_owned)}}
    }
    fn auction_open_preview(&self,s:&p3_core::scheduler::Scheduler)->Result<p3_core::scheduler::TaskRecord,String>{
        let mut task=*s.records.get(usize::from(s.head)).ok_or("tarea de subasta ausente")?;let object=u16::from_le_bytes([task.u8_at(8),task.u8_at(9)]);let price=if task.kind==0x0a{let id=usize::from(object);if id>=usize::from(self.world.ship_count){return Err("barco de subasta fuera de rango".into())}let image=if let Some(nav)=self.navigation.as_ref(){nav.snapshot.ships.get(id*0x180..(id+1)*0x180)}else{self.expedition_images.get(id).and_then(Option::as_deref)}.ok_or("barco de subasta ausente")?;p3_core::economy::auction::ship_opening_price(image).map_err(str::to_owned)?}else{let inside=object&0x8000!=0;let city=usize::from(task.u8_at(0x15));let active=if inside{false}else{self.auction_siege_active(city)?};let town=self.world.towns.get(city).ok_or("ciudad de subasta ausente")?;let site=self.sites.get(city).ok_or("plano de subasta ausente")?;let id=usize::from(object&0x7fff);if id>=usize::from(town.u16(0x776))||site.records.len()<(id+1)*8||site.records[id*8+3]>=64{return Err("edificio de subasta inválido".into())}let flag=self.world.towns.get(usize::from(town.u8(0x2c0))).ok_or("ciudad de referencia ausente")?.i32(0x2cc);let base=site.building_base_value(town,id as u32,flag);p3_core::economy::auction::building_sale_price(base,inside,active)};p3_core::economy::auction::open_bidding(&mut task,price as u16);Ok(task)
    }
    fn auction_announce_preview(&self,s:&p3_core::scheduler::Scheduler)->Result<(p3_core::scheduler::TaskRecord,Vec<Town>,bool,Vec<(u16,u8)>),String>{
        let mut task=*s.records.get(usize::from(s.head)).ok_or("tarea de subasta ausente")?;let city=task.u8_at(0x15);let merchants=self.merchants.get(..usize::from(self.world.merchant_count)).ok_or("comerciantes de subasta ausentes")?;if merchants.iter().any(|m|m.bytes.len()<10){return Err("comerciante de subasta truncado".into())}let controls=merchants.iter().map(|m|m.u16(8)).collect::<Vec<_>>();let mut towns=self.world.towns.clone();let(keep,notices)=p3_core::economy::auction::announce(&mut task,&mut towns,self.world.ship_count,s.capacity,s.head,&controls)?;Ok((task,towns,keep,notices.into_iter().map(|id|(id,city)).collect()))
    }
    /// Publica en todas las vistas navales el barco creado por la fase 0 de la subasta,
    /// como hace el pedido de astillero 0x0B, más la cabeza de la lista `gestor+0xE8`.
    fn publish_auction_ship(&mut self,before:p3_core::economy::auction_ship::LotFleet,after:p3_core::economy::auction_ship::LotFleet,ship:Option<u16>){
        let allocated=ship.map_or(usize::MAX,usize::from);let count=before.ships.len();let new_count=u16::try_from(after.ships.len()).expect("validado");
        if let Some(nav)=self.navigation.as_mut(){if allocated<count{nav.baseline.ships[allocated*0x180..(allocated+1)*0x180].copy_from_slice(&after.ships[allocated]);nav.baseline.routes[allocated]=Some(p3_save::ships::SavedRoute::None)}for image in after.ships.iter().skip(count){nav.baseline.ships.extend_from_slice(image);nav.baseline.routes.push(if u16_at(image,0x134)<0x17{Some(p3_save::ships::SavedRoute::None)}else{None})}nav.snapshot.ships=after.ships.concat();nav.snapshot.routes.resize(after.ships.len(),None);if let Some(route)=nav.snapshot.routes.get_mut(allocated){*route=Some(p3_save::ships::SavedRoute::None)}nav.snapshot.manager[0xe2..0xe4].copy_from_slice(&after.free_ship.to_le_bytes());nav.snapshot.manager[0xe8..0xea].copy_from_slice(&after.docked_head.to_le_bytes());nav.snapshot.manager[0xee..0xf0].copy_from_slice(&after.name_cursor.to_le_bytes());nav.snapshot.manager[0xf4..0xf6].copy_from_slice(&new_count.to_le_bytes());nav.baseline.manager[0xf4..0xf6].copy_from_slice(&new_count.to_le_bytes());}
        self.world.ship_count=new_count;self.world.ships.resize_with(after.ships.len(),||Ship{owner:255,state:255});self.fleet.resize_with(after.ships.len(),ShipCargo::default);self.expedition_images.resize_with(after.ships.len(),||None);self.expedition_original_images.resize_with(after.ships.len(),||None);self.ship_metadata.resize(after.ships.len(),(u16::MAX,255,0,u16::MAX));
        for(i,image)in after.ships.iter().enumerate(){if before.ships.get(i)==Some(image){continue}let state=u16_at(image,0x134);self.world.ships[i]=Ship{owner:if state<0x17{image[0]}else{255},state};self.ship_metadata[i]=(u16_at(image,4),state,u16_at(image,0x15e),u16_at(image,8));self.expedition_images[i]=(state<0x17).then(||image.clone());let cargo=&mut self.fleet[i];*cargo=ShipCargo{owner:image[0],town:image[0x39],kind:image[0xe],capacity:u32_at(image,0x10)as i32,dirty:u16_at(image,0x3c),crew:u16_at(image,0x40),cutlasses:u16_at(image,0x154),marines:image[0x156],..Default::default()};for k in 0..24{cargo.stock[k]=u32_at(image,0x54+4*k)as i32;cargo.average_price[k]=f32::from_bits(u32_at(image,0xb4+4*k));cargo.guns[k]=image[0x13c+k]}cargo.cargo_value=u32_at(image,0x114)as i32;cargo.cargo_weight=u32_at(image,0x118)as i32;cargo.equipment_weight=u32_at(image,0x11c)as i32;cargo.gun_power=u32_at(image,0x120)as i32;}
        if let Some(p)=self.pirates.as_mut(){p.next=after.ships.iter().map(|s|u16_at(s,4)).collect();p.ship_captains=after.ships.iter().map(|s|u16_at(s,0x42)).collect()}
        *self.expedition_docked=after.docked_head;
        let factory=self.expedition_factory.as_mut().expect("validado");factory.ships=after.ships;factory.free_ship=after.free_ship;factory.name_cursor=after.name_cursor;
    }
    /// Fase 0 de la subasta de barcos (0x004E2D1A): crea el barco subastado sobre copias de las vistas navales.
    fn auction_ship_preview(&self,s:&p3_core::scheduler::Scheduler)->Result<AuctionShipPrepared,String>{
        use p3_core::economy::auction_ship::{create_lot_ship,LotFleet,LotResources};
        let mut task=*s.records.get(usize::from(s.head)).ok_or("tarea de subasta ausente")?;
        if self.shipyard_detached{return Err("subasta naval requiere conciliar el astillero sin navegación".into())}
        let factory=self.expedition_factory.as_ref().ok_or("fábrica naval de subasta ausente")?;let count=usize::from(self.world.ship_count);
        if factory.ships.len()!=count||self.world.ships.len()!=count||self.fleet.len()!=count||self.ship_metadata.len()!=count||self.expedition_images.len()!=count{return Err("vistas de flota de subasta incompletas".into())}
        let ships=if let Some(nav)=self.navigation.as_ref(){let snap=&nav.snapshot;if snap.manager.len()<0xf8||snap.ships.len()!=count*0x180||snap.routes.len()!=count||usize::from(u16_at(&snap.manager,0xf4))!=count||u16_at(&snap.manager,0xe2)!=factory.free_ship||u16_at(&snap.manager,0xe8)!=*self.expedition_docked||u16_at(&snap.manager,0xee)!=factory.name_cursor||u16_at(&snap.manager,0xf0)!=factory.name_step{return Err("gestor naval de subasta desincronizado".into())}snap.ships.chunks_exact(0x180).map(<[u8]>::to_vec).collect()}else{factory.ships.iter().enumerate().map(|(i,free)|self.expedition_images[i].as_ref().unwrap_or(free).clone()).collect()};
        if let Some(p)=self.pirates.as_ref(){if p.next.len()!=count||p.ship_captains.len()!=count{return Err("vistas piratas de subasta incompletas".into())}}
        let resources=self.expedition_resources.as_ref().ok_or("recursos navales de subasta ausentes")?;
        let before=LotFleet{ships,free_ship:factory.free_ship,docked_head:*self.expedition_docked,name_cursor:factory.name_cursor,name_step:factory.name_step};
        let mut after=before.clone();
        let id=create_lot_ship(&mut task,&self.world.towns,&mut after,&LotResources{tick:self.tick,ports:&resources.ports,names:&resources.ship_names})?;
        Ok(AuctionShipPrepared{task,before,after,ship:id})
    }
    fn auction_bids_preview(&self,s:&p3_core::scheduler::Scheduler)->Result<p3_core::scheduler::TaskRecord,String>{
        use p3_core::ai::{guild::GuildOffice,bids::{BidOffice,BidTask}};let mut task=*s.records.get(usize::from(s.head)).ok_or("tarea de subasta ausente")?;
        let images=self.scheduled_office_images.as_ref().ok_or("oficinas de subasta ausentes")?;if images.len()!=self.stocks.len()||images.len()!=self.world.offices.len()||images.iter().any(|v|v.len()<0x2ca){return Err("oficinas de subasta incompletas".into())}
        let merchants=self.merchants.get(..usize::from(self.world.merchant_count)).ok_or("comerciantes de subasta ausentes")?;
        if task.kind==0x36{let city=usize::from(task.u8_at(0x15));let town=self.world.towns.get(city).ok_or("ciudad de subasta ausente")?;let site=self.sites.get(city).ok_or("plano de subasta ausente")?;let flag=self.world.towns.get(usize::from(town.u8(0x2c0))).ok_or("ciudad de referencia ausente")?.i32(0x2cc);let siege=self.auction_siege_active(city)?;if self.world.merchant_facilities.len()!=self.employment_businesses.len(){return Err("negocios de subasta incompletos".into())}let offices=images.iter().enumerate().map(|(i,v)|BidOffice{merchant:self.stocks[i].owner,town:self.stocks[i].town,next:u16_at(v,0x2c8),next_in_town:self.world.offices[i].next,task:self.world.offices[i].first_facility}).collect::<Vec<_>>();let businesses=self.world.merchant_facilities.iter().zip(self.employment_businesses.iter()).map(|(b,image)|BidTask{a:b.employees,next:b.next,b:u16_at(image,12)}).collect::<Vec<_>>();auction_runtime::building_bids(&mut task,town,site,merchants,&offices,&businesses,flag,siege)?;return Ok(task)}
        let offices=images.iter().enumerate().map(|(i,v)|GuildOffice{town:self.stocks[i].town,next:u16_at(v,0x2c8)}).collect::<Vec<_>>();let count=usize::from(self.world.ship_count);if self.expedition_images.len()<count{return Err("flota de subasta incompleta".into())}
        let ships=if let Some(nav)=self.navigation.as_ref(){if nav.snapshot.ships.len()<count*0x180{return Err("flota de subasta truncada".into())}nav.snapshot.ships.chunks_exact(0x180).take(count).map(|v|Some(v.to_vec())).collect()}else{self.expedition_images[..count].to_vec()};
        auction_runtime::ship_bids(&mut task,&self.world.towns,merchants,&offices,&ships)?;Ok(task)
    }
    /// Tarea 0x20 (`0x004DBBD0`): un paso de la elección del Ältermann sobre copias del estado.
    fn alderman_election_preview(&self,s:&p3_core::scheduler::Scheduler)->Result<AldermanPrepared,String>{
        use p3_core::economy::alderman_task::{step,ElectionTaskHost,ElectionWorld};
        /// Byte `0x006737B9` del ejecutable inglés (único lector: la tarea 0x20); valor inicial 200.
        const VOTE_SCALE:u8=200;
        struct Host<'h>{offices:Option<Vec<u8>>,network:bool,merchant_count:u16,tick:u32,pool:&'h mut Vec<u8>,texts:Option<&'h c08_text::ElectionTexts>,first:Option<&'h [Vec<Vec<u8>>;2]>,family:Option<&'h Vec<Vec<u8>>>,names:&'h Option<town_names::TownNameBank>,ids:&'h [u8],notices:Vec<[u32;6]>,announcements:Vec<(bool,u32,u16)>,error:Option<String>,prepare_missions:bool}
        impl ElectionTaskHost for Host<'_>{
            fn notification(&mut self,_:&mut ElectionWorld,args:[u32;6]){self.notices.push(args)}
            fn winner_announcement(&mut self,_:&mut ElectionWorld,local:bool,winner:u32,word:u16){self.announcements.push((local,winner,word))}
            fn update_rank(&mut self,w:&mut ElectionWorld,merchant:usize,town:u32){
                if self.error.is_some(){return}
                let Some(offices)=self.offices.as_ref() else{self.error=Some("oficinas electorales ausentes".into());return};
                // 0x004F75F0 con sus efectos: cartas al momento (orden del buzón original) y misiones del alcalde.
                let effects=match mayor_election::update_rank(&mut w.merchants[merchant],w.towns,town as usize,offices,w.player,w.election_town,self.merchant_count,self.network,self.pool){Ok(e)=>e,Err(e)=>{self.error=Some(e);return}};
                let date=p3_core::calendar::Calendar::for_tick(self.tick);
                for (code,record) in effects.messages{if let Err(e)=p3_state::messages::post_message(self.pool,w.merchants,self.merchant_count,date.day,date.month,date.year,code,&record,p3_state::messages::Payload::None,&p3_state::messages::RETENTION){self.error=Some(e);return}}
                if effects.prepare_missions{if self.prepare_missions{self.error=Some("dos programaciones de misiones en la misma elección".into());return}self.prepare_missions=true}
            }
            fn post_messages(&mut self,w:&mut ElectionWorld,candidates:[u8;4],votes:[u8;4],kind:u8,town:u8)->Result<(),String>{
                let texts=self.texts.ok_or("textos de la elección del Ältermann no configurados")?;
                let first=self.first.ok_or("nombres de pila ausentes para la elección")?;let family=self.family.ok_or("apellidos ausentes para la elección")?;
                let date=p3_core::calendar::Calendar::for_tick(self.tick);
                let (names,ids)=(self.names,self.ids);
                let town_name=|k:u32|->Vec<u8>{ids.get(k as usize).and_then(|&id|names.as_ref()?.for_map(id)).map(|(n,_)|n.to_vec()).unwrap_or_default()};
                // 0x005299F0 lee 0x6DE52D/E ya actualizados: el Ältermann nuevo y su ciudad.
                c08_text::post_alderman_election(self.pool,w.merchants,w.towns,self.merchant_count,w.player,(date.day,date.month,date.year),texts,c08_text::NameBanks{first,family},&town_name,candidates,votes,kind,town)
            }
        }
        let mut task=*s.records.get(usize::from(s.head)).ok_or("tarea de elección del Ältermann ausente")?;
        let count=usize::from(self.world.merchant_count);if self.merchants.len()!=count{return Err("comerciantes electorales incompletos".into())}
        let offices=self.scheduled_office_images.as_ref().filter(|images|images.len()==usize::from(self.world.office_count)&&images.iter().all(|v|v.len()>=0x2c5)).map(|images|images.iter().map(|v|v[0x2c4]).collect::<Vec<_>>());
        let mut towns=self.world.towns.clone();let mut merchants=self.merchants.to_vec();let mut pool=self.pool.clone();
        let mut host=Host{offices,network:self.world.network_client!=0,merchant_count:self.world.merchant_count,tick:self.tick,pool:&mut pool,texts:self.election_texts.as_ref(),first:self.mission_first_names.as_ref(),family:self.mission_family_names.as_ref(),names:self.mission_town_names,ids:self.mission_town_ids,notices:Vec::new(),announcements:Vec::new(),error:None,prepare_missions:false};
        let mut world=ElectionWorld{towns:&mut towns,merchants:&mut merchants,player:self.world.alderman_merchant,election_town:self.world.alderman_town,local_player:self.world.local_merchant,vote_scale:VOTE_SCALE};
        step(&mut task,&mut world,&mut host)?;
        if let Some(e)=host.error.take(){return Err(e)}
        let (player,town)=(world.player,world.election_town);let (notices,announcements)=(host.notices,host.announcements);
        let mut extra=MayorElectionExtra::default();
        if host.prepare_missions{let prepared=self.mayor_prepare_missions(&towns)?;extra.pirates=Some(prepared.pirates);extra.operations=prepared.operations;}
        Ok(AldermanPrepared{task,towns,merchants,pool,player,town,notices,announcements,extra})
    }
    /// `0x005326E0` desde la elección: misiones del alcalde con las vistas vivas, como el día de comerciante.
    fn mayor_prepare_missions(&self,towns:&[Town])->Result<p3_core::ai::settlement::PreparedAldermanMissions,String>{
        let locations=self.mission_locations.as_ref().ok_or("faltan las 40 posiciones de StadtDaten.ini")?;let pirates=self.hunt_bands.as_ref().ok_or("bandas piratas ausentes")?;
        let factory=self.expedition_factory.as_ref().ok_or("flota no disponible")?;
        let images=if let Some(nav)=self.navigation.as_ref(){if nav.snapshot.ships.len()%0x180!=0{return Err("flota truncada".into())}nav.snapshot.ships.chunks_exact(0x180).map(|v|v.to_vec()).collect::<Vec<_>>()}else{factory.ships.clone()};
        let captains=if let Some(nav)=self.navigation.as_ref(){if nav.snapshot.captains.len()%16!=0{return Err("capitanes truncados".into())}nav.snapshot.captains.chunks_exact(16).map(|v|v.try_into().unwrap()).collect::<Vec<[u8;16]>>()}else{factory.captains.clone()};
        let images_office=self.scheduled_office_images.as_ref().ok_or("oficinas de misión ausentes")?;if images_office.iter().any(|v|v.len()<0x2cc){return Err("oficinas de misión truncadas".into())}
        let int=|b:&[u8],o:usize|i32::from_le_bytes(b[o..o+4].try_into().unwrap());
        let offices=images_office.iter().map(|image|p3_core::ai::settlement::SettlementOffice{stock:std::array::from_fn(|i|int(image,4+4*i)),demand:std::array::from_fn(|i|int(image,0x64+4*i)),next:u16_at(image,0x2ca)}).collect::<Vec<_>>();
        let date=p3_core::calendar::Calendar::for_tick(self.tick);
        p3_core::ai::settlement::prepare_alderman_missions(date.year,date.month,towns,self.mission_town_ids,&offices,pirates,&images,&captains,self.world.merchant_count,self.event_pirate_difficulty,locations).map_err(str::to_string)
    }
    /// Aplica los efectos de rango de la elección en el orden original: cartas y programación de misiones.
    fn mayor_rank_effects(&self,effects:mayor_election::RankEffects,towns:&[Town],merchants:&mut [RepMerchant],pool:&mut Vec<u8>,extra:&mut MayorElectionExtra)->Result<(),String>{
        let date=p3_core::calendar::Calendar::for_tick(self.tick);
        for (code,record) in effects.messages{p3_state::messages::post_message(pool,merchants,self.world.merchant_count,date.day,date.month,date.year,code,&record,p3_state::messages::Payload::None,&p3_state::messages::RETENTION)?;}
        if effects.prepare_missions{if extra.pirates.is_some(){return Err("dos programaciones de misiones en la misma elección".into())}let prepared=self.mayor_prepare_missions(towns)?;extra.pirates=Some(prepared.pirates);extra.operations.extend(prepared.operations);}
        Ok(())
    }
    fn mayor_election_preview(&self,s:&p3_core::scheduler::Scheduler)->Result<(p3_core::scheduler::Scheduler,Vec<Town>,Vec<RepMerchant>,Vec<u8>,MayorElectionExtra),String>{
        let mut task=*s.records.get(usize::from(s.head)).ok_or("tarea de elección ausente")?;let date=p3_core::calendar::Calendar::for_tick(self.tick);
        let Some(city)=mayor_schedule::schedule(&mut task,date.year,self.world.day_of_year)? else{let mut scheduler=s.clone();scheduler.records[usize::from(s.head)]=task;return Ok((scheduler,self.world.towns.clone(),self.merchants.to_vec(),self.pool.clone(),MayorElectionExtra::default()))};
        let city=usize::from(city);let town=self.world.towns.get(city).ok_or("ciudad electoral ausente")?;let town_index=usize::from(town.u8(0x2c0));let count=usize::from(self.world.merchant_count);if self.merchants.len()!=count{return Err("comerciantes electorales incompletos".into())}
        let old=town.u8(0x6f1);let mut extra=MayorElectionExtra::default();
        if self.merchants.iter().any(|m|m.bytes.len()<0x650){return Err("comerciantes electorales truncados".into())}
        let mut merchants=self.merchants.to_vec();let candidates=mayor_election::candidates(town,self.world.towns.len(),&mut merchants)?;let ids:[u8;4]=candidates[..4].try_into().unwrap();let mut reps=[0i32;4];
        for i in 0..4{let id=usize::from(ids[i]);reps[i]=if let Some(m)=merchants.get(id){let rep=m.f32(0x2fc+town_index*4);if rep>32758.0{32767}else{p3_core::numeric::ftol_i32(rep as f64) as i16 as i32}}else{[59,65,70,80][usize::from(!ids[i]&3)]};}
        let bribes=std::array::from_fn(|i|town.u8(0x6dc+i));let result=mayor_election::ballot(ids,reps,bribes,self.world.merchant_count,self.tick,old,town.u8(0x6f3))?;
        let mut scheduler=s.clone();if result.statue{let total=result.votes.iter().fold(0i32,|a,&v|a.wrapping_add(v)) as u32;let divisor=total.wrapping_add(1);if divisor==0{return Err("divisor de estatua nulo".into())}let winning=result.votes[ids.iter().position(|&v|v==result.winner).ok_or("ganador electoral ausente")?] as u32;let reduction=winning.wrapping_mul(93).wrapping_shl(8)/divisor;let due=self.tick.wrapping_sub(reduction).wrapping_add(0xba00);if due<=self.tick{return Err("fecha de estatua fuera de rango".into())}if scheduler.free_head>scheduler.capacity||scheduler.free_head>=scheduler.capacity&&scheduler.capacity>u16::MAX-128{return Err("crecimiento del planificador de estatua inválido".into())}let id=scheduler.add_task(due);let record=&mut scheduler.records[usize::from(id)];record.kind=0x38;record.set_u8_at(8,town.u8(0x2c0));record.set_u8_at(9,result.winner);}
        let current=&mut scheduler.records[usize::from(s.head)];current.due=task.due;current.payload=task.payload;
        let offices=if usize::from(old)<count||usize::from(result.winner)<count{let images=self.scheduled_office_images.as_ref().ok_or("oficinas electorales ausentes")?;if images.len()!=usize::from(self.world.office_count)||images.iter().any(|v|v.len()<0x2c5){return Err("oficinas electorales truncadas".into())}images.iter().map(|v|v[0x2c4]).collect::<Vec<_>>()}else{Vec::new()};
        let mut towns=self.world.towns.clone();let t=&mut towns[city];t.set_u8(0x6f1,result.winner);t.set_u8(0x6f3,result.terms);for i in 0..4{t.set_u8(0x6dc+i,result.bribes[i]);}
        let mut pool=self.pool.clone();
        if usize::from(old)<count{let mut m=merchants[usize::from(old)].clone();let effects=mayor_election::update_rank(&mut m,&towns,town_index,&offices,self.world.alderman_merchant,self.world.alderman_town,self.world.merchant_count,self.world.network_client!=0,&pool)?;merchants[usize::from(old)]=m;self.mayor_rank_effects(effects,&towns,&mut merchants,&mut pool,&mut extra)?;}
        for t in &mut towns{for (i,rank) in [5,5,4,4].into_iter().enumerate(){t.set_u8(0x6e4+i,rank)}let mayor=t.u8(0x6f1);if usize::from(mayor)>=count{let slot=usize::from(!mayor&3);let rank=if self.world.alderman_town==t.u8(0x2c0)&&usize::from(self.world.alderman_merchant)==slot{7}else{6};t.set_u8(0x6e4+slot,rank)}}
        if usize::from(result.winner)<count{let w=usize::from(result.winner);let mut m=merchants[w].clone();let effects=mayor_election::update_rank(&mut m,&towns,town_index,&offices,self.world.alderman_merchant,self.world.alderman_town,self.world.merchant_count,self.world.network_client!=0,&pool)?;merchants[w]=m;self.mayor_rank_effects(effects,&towns,&mut merchants,&mut pool,&mut extra)?;
            // 0x005291A6: aviso al nuevo alcalde humano; la ventana de evento del jugador local es interfaz.
            if merchants[w].u16(8)==0{let t=u32::from(town.u8(0x2c0));extra.notices.push([u32::from(result.winner),1,0,t,0x18,t]);}}
        if self.world.world_8a==0{
            // 0x00529244: carta 0x47 con el reparto de votos a los humanos de la ciudad.
            let humans=(0..count).filter(|&i|merchants[i].u16(8)==0&&merchants[i].bytes[0x19]==town.u8(0x2c0)).collect::<Vec<_>>();
            // El nombre del ganador se normaliza (0x004F6420) aunque no haya destinatarios.
            if humans.is_empty(){if usize::from(result.winner)<count{mayor_election::refresh_first_name(&mut merchants[usize::from(result.winner)]);}}else{
            let texts=self.election_texts.as_ref().and_then(|t|t.mayor.as_ref()).ok_or("textos de la elección municipal ausentes")?;
            let first=self.mission_first_names.as_ref().ok_or("nombres de pila ausentes")?;let family=self.mission_family_names.as_ref().ok_or("apellidos ausentes")?;let banks=c08_text::NameBanks{first,family};
            let t=&towns[city];let names=self.mission_town_names.as_ref().ok_or("nombres de ciudad ausentes")?;let id=*self.mission_town_ids.get(town_index).ok_or("índice de ciudad electoral ausente")?;let town_name=names.for_map(id).map(|(n,_)|n.to_vec()).unwrap_or_default();
            let npc=|id:u8|{let slot=usize::from(!id&3);(banks.given(u32::from(t.u8(0x6d4+slot))).to_vec(),banks.family(u32::from(t.u8(0x6d8+slot))).to_vec())};
            let candidates:[(Vec<u8>,Vec<u8>,u32);4]=std::array::from_fn(|i|{let id=ids[i];let (g,f)=if usize::from(id)<count{let m=&merchants[usize::from(id)];(banks.given(u32::from(m.bytes[0x1c])).to_vec(),banks.family(u32::from(m.bytes[0x1d])).to_vec())}else{npc(id)};(g,f,result.votes[i] as u32)});
            let winner=if usize::from(result.winner)<count{let m=&mut merchants[usize::from(result.winner)];let given=mayor_election::refresh_first_name(m);(banks.given(u32::from(given)).to_vec(),banks.family(u32::from(m.bytes[0x1d])).to_vec())}else{npc(result.winner)};
            let text=c08_text::mayor_election_letter(texts,&town_name,&candidates,(&winner.0,&winner.1))?;
            let mut record=[0u8;16];record[4]=0x47;record[5]=t.u8(0x2c0);
            for i in humans{p3_state::messages::post_message(&mut pool,&mut merchants,self.world.merchant_count,date.day,date.month,date.year,i as i32,&record,p3_state::messages::Payload::Text(&text),&p3_state::messages::RETENTION)?;}
        }}
        if self.world.world_8a==0{let t=&towns[city];let slot=usize::from(!result.winner&3);let (first,last)=if let Some(m)=merchants.get(usize::from(result.winner)){(m.bytes[0x1c],m.bytes[0x1d])}else{(t.u8(0x6d4+slot),t.u8(0x6d8+slot))};let winner=result.votes[ids.iter().position(|&v|v==result.winner).unwrap()] as u16;let total=result.votes.iter().fold(0i32,|a,&v|a.wrapping_add(v)) as u16;let mut letter=[0u8;16];letter[4]=0x55;letter[5]=t.u8(0x2c0);letter[8..10].copy_from_slice(&u16::from(first).to_le_bytes());letter[10..12].copy_from_slice(&u16::from(last).to_le_bytes());letter[12..14].copy_from_slice(&winner.to_le_bytes());letter[14..16].copy_from_slice(&total.to_le_bytes());p3_state::messages::add_global_inline_letter(&mut pool,date.day,date.month,date.year,&letter)?;}
        Ok((scheduler,towns,merchants,pool,extra))
    }
    fn captain_refill_preview(&self,s:&p3_core::scheduler::Scheduler)->Result<(p3_core::scheduler::TaskRecord,Vec<Town>,p3_core::ai::settlement::TavernCaptainPool,Option<p3_core::economy::hanse_petition::PiratePortraits>),String>{
        use p3_core::ai::settlement::{add_tavern_captain,TavernCaptainPool};
        let mut task=*s.records.get(usize::from(s.head)).ok_or("tarea de reposición ausente")?;
        let factory=self.expedition_factory.as_ref().ok_or("pool de capitanes ausente")?;
        let capacity=u16::try_from(factory.captains.len()).map_err(|_|"pool de capitanes demasiado grande")?;
        if let Some(nav)=self.navigation.as_ref(){let entries=factory.captains.iter().flatten().copied().collect::<Vec<_>>();if nav.snapshot.manager.len()<0xf4||nav.snapshot.captains!=entries||u16_at(&nav.snapshot.manager,0xe0)!=factory.free_captain||u16_at(&nav.snapshot.manager,0xf2)!=capacity{return Err("vistas de capitanes desincronizadas".into())}}
        let flags=captain_refill::availability(&self.world.towns,&factory.captains,self.world.merchant_count,self.world.local_merchant)?;
        let (ships,captains)=if flags.iter().filter(|&&v|v&1!=0).count()>7{([0;2],[0;2])}else{
            let merchants=self.merchants.get(..usize::from(self.world.merchant_count)).ok_or("comerciantes de reposición ausentes")?;
            let count=usize::from(self.world.ship_count);
            let images=if let Some(nav)=self.navigation.as_ref(){if nav.snapshot.ships.len()<count*0x180{return Err("flota de reposición truncada".into())}nav.snapshot.ships.chunks_exact(0x180).take(count).map(|v|Some(v.to_vec())).collect::<Vec<_>>()}else{self.expedition_images.get(..count).ok_or("flota de reposición ausente")?.to_vec()};
            captain_refill::fleet_counts(merchants,&images,factory.captains.len())?
        };
        let actions=captain_refill::plan(&mut task,&flags,ships,captains)?;
        let mut towns=self.world.towns.clone();let mut pool=TavernCaptainPool{captains:factory.captains.clone(),free_head:factory.free_captain};
        if !actions.is_empty(){let(first,last)=self.captain_names.ok_or("banco de nombres de capitán ausente")?;let date=p3_core::calendar::Calendar::for_tick(self.tick);let population=towns.get(usize::from(date.day_of_year&15)).ok_or("ciudad de semilla de capitán ausente")?.u32(0x2e0);for (city,pirate) in actions{add_tavern_captain(&mut towns[city],&mut pool,pirate,self.tick,date.day_of_year,date.day,population,first,last).map_err(str::to_owned)?;}}
        let mut pirates=self.pirates.clone();if let Some(p)=pirates.as_mut(){if p.captains.len()!=factory.captains.len(){return Err("vistas piratas de capitanes truncadas".into())}for (i,old) in factory.captains.iter().enumerate(){for j in 0..16{let new=pool.captains[i][j];if old[j]!=new{if p.captains[i][j]!=old[j]&&p.captains[i][j]!=new{return Err("capitán pirata desincronizado".into())}p.captains[i][j]=new}}}p.captains.extend_from_slice(&pool.captains[factory.captains.len()..]);}
        Ok((task,towns,pool,pirates))
    }
    fn captain_return_preview(&self,s:&p3_core::scheduler::Scheduler)->Result<(p3_core::scheduler::TaskRecord,Vec<Town>,p3_core::ships::captains::CaptainPool,Option<p3_core::economy::hanse_petition::PiratePortraits>,bool),String>{
        let mut task=*s.records.get(usize::from(s.head)).ok_or("tarea de capitán ausente")?;let factory=self.expedition_factory.as_ref().ok_or("pool de capitanes ausente")?;let entries=factory.captains.iter().flatten().copied().collect::<Vec<_>>();let capacity=u16::try_from(factory.captains.len()).map_err(|_|"pool de capitanes demasiado grande")?;
        if let Some(nav)=self.navigation.as_ref(){if nav.snapshot.manager.len()<0xf4||nav.snapshot.captains!=entries||u16_at(&nav.snapshot.manager,0xe0)!=factory.free_captain||u16_at(&nav.snapshot.manager,0xf2)!=capacity{return Err("vistas de capitanes desincronizadas".into())}}
        let mut captains=p3_core::ships::captains::CaptainPool{entries,capacity,free_head:factory.free_captain};let mut towns=self.world.towns.clone();let keep=p3_core::economy::captain_return::advance(&mut task,&mut towns,&mut captains).map_err(str::to_owned)?;
        let mut pirates=self.pirates.clone();if let Some(p)=pirates.as_mut(){if p.captains.len()!=factory.captains.len(){return Err("vistas piratas de capitanes truncadas".into())}for (i,old) in factory.captains.iter().enumerate(){for j in 0..16{let new=captains.entries[i*16+j];if old[j]!=new{if p.captains[i][j]!=old[j]&&p.captains[i][j]!=new{return Err("capitán pirata desincronizado".into())}p.captains[i][j]=new}}}}
        Ok((task,towns,captains,pirates,keep))
    }
    fn ship_retry_candidate(&self,city:usize,owner:usize,kind:u8)->Result<Option<ShipRetryBuild>,String>{
        if self.shipyard_detached{return Err("pedido naval requiere conciliar el astillero sin navegación".into())}
        let town=self.world.towns.get(city).ok_or("ciudad de pedido naval ausente")?;if usize::from(town.u8(0x2c0))!=city{return Err("índice de ciudad naval incoherente".into())}
        let factory=self.expedition_factory.as_ref().ok_or("fábrica naval ausente")?;let count=usize::from(self.world.ship_count);
        if factory.ships.len()!=count||self.world.ships.len()!=count||self.fleet.len()!=count||self.ship_metadata.len()!=count||self.expedition_images.len()!=count{return Err("vistas de flota incompletas".into())}
        let ships=if let Some(nav)=self.navigation.as_ref(){let snap=&nav.snapshot;if snap.manager.len()<0xf8||snap.ships.len()!=count*0x180||snap.routes.len()!=count||usize::from(u16_at(&snap.manager,0xf4))!=count||u16_at(&snap.manager,0xe2)!=factory.free_ship||u16_at(&snap.manager,0xee)!=factory.name_cursor||u16_at(&snap.manager,0xf0)!=factory.name_step{return Err("gestor naval desincronizado".into())}snap.ships.chunks_exact(0x180).map(<[u8]>::to_vec).collect()}else{factory.ships.iter().enumerate().map(|(i,free)|self.expedition_images[i].as_ref().unwrap_or(free).clone()).collect()};
        let mut person=self.merchants.get(owner).ok_or("comerciante naval ausente")?.clone();let account=self.world.merchants.get(owner).ok_or("cuenta naval ausente")?;person.set_i32(0,account.gold);person.set_i32(0x4c8,account.income);
        let mut at=usize::from(town.u16(0x784));let mut seen=vec![false;self.world.offices.len()];let mut office=None;while at<self.world.offices.len(){if seen[at]{return Err("cadena urbana de oficinas cíclica".into())}seen[at]=true;let stock=self.stocks.get(at).ok_or("existencias de oficina ausentes")?;if usize::from(stock.owner)==owner{if usize::from(stock.town)!=city{return Err("oficina naval incoherente".into())}let image=self.scheduled_office_images.as_ref().and_then(|v|v.get(at)).ok_or("imagen de oficina naval ausente")?;if image.len()!=0x44c{return Err("imagen de oficina naval truncada".into())}office=Some(at);break}at=usize::from(self.world.offices[at].next)}
        if let Some(p)=self.pirates.as_ref(){if p.next.len()!=count||p.ship_captains.len()!=count{return Err("vistas piratas incompletas".into())}}
        let resources=self.expedition_resources.as_ref().ok_or("recursos navales ausentes")?;let port=resources.ports.get(city).copied().flatten().ok_or("puerto de astillero ausente")?;
        let before=c10_ship_orders::BuildState{town:town.clone(),merchant:person,office:office.map(|i|self.stocks[i].clone()),ships,free_ship:factory.free_ship,name_cursor:factory.name_cursor,name_step:factory.name_step};
        let Some(after)=c10_ship_orders::prepare(&before,owner as u8,kind,false,&c10_ship_orders::BuildResources{tick:self.tick,port,names:&resources.ship_names})? else{return Ok(None)};
        Ok(Some(ShipRetryBuild{city,owner,office,before,after}))
    }
    /// Publica en todas las vistas el barco, los materiales y el dinero de un pedido de astillero (0x52A360).
    fn publish_ship_build(&mut self,build:ShipRetryBuild){
                let city=build.city;let owner=build.owner;let office=build.office;let before=build.before;let allocated=usize::from(build.after.ship);let after=build.after.state;let count=before.ships.len();let new_count=u16::try_from(after.ships.len()).expect("validado");
                self.world.towns[city]=after.town;self.world.merchants[owner].gold=after.merchant.i32(0);self.merchants[owner]=after.merchant;
                if let Some(i)=office {let stock=after.office.expect("validado");self.stocks[i]=stock;let image=&mut self.scheduled_office_images.as_mut().expect("validado")[i];for k in 0..24{image[4+4*k..8+4*k].copy_from_slice(&self.stocks[i].stock[k].to_le_bytes())}let admin=u16_at(image,0x2f2);let flags=u16_at(image,0x2d6);self.scheduled_office_updates.push(p3_core::economy::world::ScheduledOfficeUpdate{index:i,administrator:admin,flags,experience:None,wage:None,image:Some(image.clone())});}
                if let Some(nav)=self.navigation.as_mut(){if allocated<count{nav.baseline.ships[allocated*0x180..(allocated+1)*0x180].copy_from_slice(&after.ships[allocated]);nav.baseline.routes[allocated]=Some(p3_save::ships::SavedRoute::None)}for image in after.ships.iter().skip(count){nav.baseline.ships.extend_from_slice(image);nav.baseline.routes.push(if u16_at(image,0x134)<0x17{Some(p3_save::ships::SavedRoute::None)}else{None})}nav.snapshot.ships=after.ships.concat();nav.snapshot.routes.resize(after.ships.len(),None);nav.snapshot.routes[allocated]=Some(p3_save::ships::SavedRoute::None);nav.snapshot.manager[0xe2..0xe4].copy_from_slice(&after.free_ship.to_le_bytes());nav.snapshot.manager[0xee..0xf0].copy_from_slice(&after.name_cursor.to_le_bytes());nav.snapshot.manager[0xf4..0xf6].copy_from_slice(&new_count.to_le_bytes());nav.baseline.manager[0xf4..0xf6].copy_from_slice(&new_count.to_le_bytes());}
                self.world.ship_count=new_count;self.world.ships.resize_with(after.ships.len(),||Ship{owner:255,state:255});self.fleet.resize_with(after.ships.len(),ShipCargo::default);self.expedition_images.resize_with(after.ships.len(),||None);self.expedition_original_images.resize_with(after.ships.len(),||None);self.ship_metadata.resize(after.ships.len(),(u16::MAX,255,0,u16::MAX));
                for(i,image)in after.ships.iter().enumerate(){if before.ships.get(i)==Some(image){continue}let state=u16_at(image,0x134);self.world.ships[i]=Ship{owner:if state<0x17{image[0]}else{255},state};self.ship_metadata[i]=(u16_at(image,4),state,u16_at(image,0x15e),u16_at(image,8));self.expedition_images[i]=(state<0x17).then(||image.clone());let cargo=&mut self.fleet[i];*cargo=ShipCargo{owner:image[0],town:image[0x39],kind:image[0xe],capacity:u32_at(image,0x10)as i32,dirty:u16_at(image,0x3c),crew:u16_at(image,0x40),cutlasses:u16_at(image,0x154),marines:image[0x156],..Default::default()};for k in 0..24{cargo.stock[k]=u32_at(image,0x54+4*k)as i32;cargo.average_price[k]=f32::from_bits(u32_at(image,0xb4+4*k));cargo.guns[k]=image[0x13c+k]}cargo.cargo_value=u32_at(image,0x114)as i32;cargo.cargo_weight=u32_at(image,0x118)as i32;cargo.equipment_weight=u32_at(image,0x11c)as i32;cargo.gun_power=u32_at(image,0x120)as i32;}
                if let Some(p)=self.pirates.as_mut(){p.next=after.ships.iter().map(|s|u16_at(s,4)).collect();p.ship_captains=after.ships.iter().map(|s|u16_at(s,0x42)).collect()}
                let factory=self.expedition_factory.as_mut().expect("validado");factory.ships=after.ships;factory.free_ship=after.free_ship;factory.name_cursor=after.name_cursor;
    }
    fn ship_retry_preview(&self,s:&p3_core::scheduler::Scheduler)->Result<ShipRetryPrepared,String>{
        let mut task=*s.records.get(usize::from(s.head)).ok_or("tarea de pedido ausente")?;let owner=usize::from(u16::from_le_bytes([task.u8_at(8),task.u8_at(9)]));let merchant=self.merchants.get(owner);let account=self.world.merchants.get(owner);
        if owner<usize::from(self.world.merchant_count)&&(merchant.is_none_or(|m|m.bytes.len()!=0x650)||account.is_none()){return Err("cuenta de pedido naval incompleta".into())}
        let cash=account.map_or(0,|m|m.gold);let home=merchant.and_then(|m|m.bytes.get(0x19)).copied().unwrap_or(0xff);let mut built=None;let mut error=None;
        let keep=p3_core::economy::ship_build_retry::step(&mut task,self.world.merchant_count,cash,home,self.world.towns.len() as u32,|city,who,kind|{if error.is_some(){return false}match self.ship_retry_candidate(city as usize,usize::from(who),kind){Ok(Some(out))=>{built=Some(out);true},Ok(None)=>false,Err(e)=>{error=Some(e);false}}});
        if let Some(error)=error{return Err(error)}Ok(ShipRetryPrepared{task,keep,build:built})
    }
    fn ai_ship_order_preview(&self,s:&p3_core::scheduler::Scheduler)->Result<(p3_core::scheduler::TaskRecord,Option<p3_core::pump::Operation>),String>{
        use p3_core::economy::ai_ship_orders::{self,ShipBuildCost};
        struct LiveCost<'a>{towns:&'a [Town],merchants:&'a [RepMerchant],images:Option<&'a Vec<Vec<u8>>>,stocks:&'a [OfficeStock],ships:&'a [Option<Vec<u8>>]}
        impl ShipBuildCost for LiveCost<'_>{fn possible(&mut self,city:u8,kind:u8,merchant:u16)->Result<bool,String>{
            let town=self.towns.get(usize::from(city)).ok_or("ciudad de astillero ausente")?;let m=self.merchants.get(usize::from(merchant)).ok_or("comerciante de astillero ausente")?;
            if kind>=4||(town.u8(0x824+usize::from(kind))as i8)<0||town.more_flags()&0x20000==0||town.flags()&1!=0&&kind&2!=0{return Ok(false)}
            let city_index=usize::from(town.u8(0x2c0));let city_town=self.towns.get(city_index).ok_or("índice de ciudad de astillero inválido")?;let images=self.images.ok_or("oficinas de astillero ausentes")?;if images.len()!=self.stocks.len()||images.iter().any(|v|v.len()<0x2cc){return Err(format!("oficinas de astillero truncadas: imágenes {}, existencias {}",images.len(),self.stocks.len()))}
            let mut at=usize::from(city_town.u16(0x784));let mut seen=vec![false;images.len()];let mut office=None;while at<images.len(){if seen[at]{return Err("cadena de oficinas de astillero cíclica".into())}seen[at]=true;let image=&images[at];if u16_at(image,0x2c4)==merchant{if self.stocks[at].owner!=merchant||usize::from(self.stocks[at].town)!=city_index{return Err("oficina de astillero desincronizada".into())}office=Some(&self.stocks[at]);break}at=usize::from(u16_at(image,0x2ca));}
            ai_ship_cost::possible(town,kind,m,office,self.ships)
        }}
        let mut task=*s.records.get(usize::from(s.head)).ok_or("tarea de pedidos navales ausente")?;let count=usize::from(self.world.merchant_count);let merchants=self.merchants.get(..count).ok_or("comerciantes de pedidos navales ausentes")?;
        let count=usize::from(self.world.ship_count);if self.expedition_images.len()<count{return Err("flota de pedidos navales incompleta".into())}let mut next=Vec::with_capacity(count);let mut ships=Vec::with_capacity(count);
        for i in 0..count{let image=if let Some(nav)=self.navigation.as_ref(){nav.snapshot.ships.get(i*0x180..(i+1)*0x180)}else{self.expedition_images[i].as_deref()};if let Some(image)=image{if image.len()<0x180{return Err("barco de pedidos navales truncado".into())}next.push(u16_at(image,4));ships.push(Some(image.to_vec()))}else{next.push(u16::MAX);ships.push(None)}}
        for m in merchants{if m.bytes.len()<0x47a{return Err("comerciante de pedidos navales truncado".into())}if m.u16(8)==0||m.u16(8)&4!=0{continue}let mut at=usize::from(m.u16(0xe));let mut seen=vec![false;count];while at<count{if seen[at]{return Err("cadena naval cíclica".into())}if ships[at].is_none(){return Err("barco de pedidos navales ausente".into())}seen[at]=true;at=usize::from(next[at])}}
        let mut cost=LiveCost{towns:&self.world.towns,merchants,images:self.scheduled_office_images.as_ref(),stocks:self.stocks,ships:&ships};let op=ai_ship_orders::plan(&mut task,&self.world.towns,merchants,&next,self.world.network_client!=0,&mut cost)?;Ok((task,op))
    }
    fn ai_expansion_preview(&self,s:&p3_core::scheduler::Scheduler)->Result<(p3_core::scheduler::TaskRecord,Option<(u16,u16)>),String>{
        use p3_core::economy::ai_expansion::{self,Office,Business};let mut task=*s.records.get(usize::from(s.head)).ok_or("tarea de expansión IA ausente")?;let images=self.scheduled_office_images.as_ref().ok_or("oficinas de expansión ausentes")?;if images.len()!=self.world.offices.len()||images.len()!=self.stocks.len()||images.iter().any(|v|v.len()<0x2ce)||self.world.merchant_facilities.len()!=self.employment_businesses.len(){return Err("datos de expansión IA incompletos".into())}let offices=images.iter().enumerate().map(|(i,image)|Office{owner:self.stocks[i].owner,next_merchant:u16_at(image,0x2c8),next_town:self.world.offices[i].next,first_business:self.world.offices[i].first_facility}).collect::<Vec<_>>();let businesses=self.employment_businesses.iter().zip(&self.world.merchant_facilities).map(|(b,f)|Business{next:f.next,employees:f.employees,maximum:u16_at(b,12)}).collect::<Vec<_>>();let count=usize::from(self.world.merchant_count);let merchants=self.merchants.get(..count).ok_or("comerciantes de expansión ausentes")?;let chosen=ai_expansion::choose(&mut task,&self.world.towns,merchants,&offices,&businesses)?;Ok((task,chosen))
    }
    fn ai_construction_preview(&self)->Result<(Option<p3_core::pump::Operation>,Option<p3_core::rng::CrtRand>),String>{
        let images=self.scheduled_office_images.as_ref().ok_or("imágenes de oficinas de IA ausentes")?;ai_construction::plan(self.world,self.sites,images,self.stocks,self.employment_businesses,self.merchants,self.tick,*self.investigation_rng)
    }
    fn ai_build_interval_preview(&self,s:&p3_core::scheduler::Scheduler)->Result<p3_core::scheduler::TaskRecord,String>{
        use p3_core::economy::ai_build_interval::{self,Office,Business};let mut task=*s.records.get(usize::from(s.head)).ok_or("tarea de construcción IA ausente")?;let count=usize::from(self.world.merchant_count);if self.merchants.len()<count{return Err("comerciantes de IA ausentes".into())}let images=self.scheduled_office_images.as_ref().ok_or("imágenes de oficinas de IA ausentes")?;if images.len()!=self.world.offices.len()||images.iter().any(|v|v.len()<0x2ca)||self.world.merchant_facilities.len()!=self.employment_businesses.len(){return Err("oficinas o negocios de IA incompletos".into())}let offices=self.world.offices.iter().zip(images).map(|(o,image)|Office{next:u16_at(image,0x2c8),first_business:o.first_facility}).collect::<Vec<_>>();let businesses=self.world.merchant_facilities.iter().map(|b|Business{next:b.next,employees:b.employees}).collect::<Vec<_>>();ai_build_interval::recompute(&mut task,&self.world.towns,&self.merchants[..count],&offices,&businesses,self.rep_scale)?;Ok(task)
    }
    fn shipyard_weekly_preview(&self)->Result<ShipyardWeeklyPrepared,String>{
        use p3_core::economy::shipyard_weekly::{self,YardShip};let count=self.expedition_images.len();if self.navigation.as_ref().is_some_and(|n|n.snapshot.ships.len()!=count*0x180){return Err("flota de astilleros incompleta".into())}
        let mut ships=Vec::with_capacity(count);let mut present=Vec::with_capacity(count);for i in 0..count{let image=if let Some(nav)=self.navigation.as_ref(){nav.snapshot.ships.get(i*0x180..(i+1)*0x180)}else{self.expedition_images[i].as_deref()};if let Some(image)=image{if image.len()<0x180{return Err("barco de astillero truncado".into())}ships.push(YardShip{next:u16_at(image,6),capacity:u32_at(image,0x10)as i32});present.push(true)}else{ships.push(YardShip{next:u16::MAX,capacity:0});present.push(false)}}
        for town in &self.world.towns{let mut at=usize::from(town.u16(0x81c));let mut seen=vec![false;count];while at<count{if seen[at]{return Err("cadena de pedidos del astillero cíclica".into())}seen[at]=true;if !present[at]{return Err("barco de astillero ausente".into())}at=usize::from(ships[at].next)}}
        let mut towns=self.world.towns.clone();let notices=shipyard_weekly::update(&mut towns,&ships)?;let mut pool=self.pool.clone();let mut merchants=self.merchants.to_vec();let count=usize::from(self.world.merchant_count);if merchants.len()<count{return Err("comerciantes de astillero ausentes".into())}let date=p3_core::calendar::Calendar::for_tick(self.tick);for city in notices{for merchant in &mut merchants[..count]{p3_state::messages::add_shipyard_notice(&mut pool,merchant,date.day,date.month,date.year,city)?;}}Ok(ShipyardWeeklyPrepared{towns,pool,merchants})
    }
    fn town_notices_preview(&self,s:&p3_core::scheduler::Scheduler)->Result<(p3_core::scheduler::TaskRecord,Vec<Town>,[u8;84]),String>{
        let mut task=*s.records.get(usize::from(s.head)).ok_or("tarea de tablón ausente")?;let mut towns=self.world.towns.clone();let mut board=self.town_hall_notices.ok_or("tablón urbano ausente")?;
        p3_core::economy::town_notices::refresh(&mut task,&mut towns,&mut board,self.tick,self.world.day_of_year)?;Ok((task,towns,board))
    }
    fn supply_deadline_preview(&self,s:&p3_core::scheduler::Scheduler)->Result<SupplyDeadlinePrepared,String>{
        let mut task=*s.records.get(usize::from(s.head)).ok_or("encargo urbano ausente")?;let count=usize::from(self.world.merchant_count);if count>self.merchants.len(){return Err("comerciantes de encargo incompletos".into())}let mut towns=self.world.towns.clone();let mut merchants=self.merchants.to_vec();let(keep,notice)=p3_core::economy::supply_deadline::advance(&mut task,s.head,&mut towns,&mut merchants[..count])?;let mut pool=self.pool.clone();if let Some(notice)=notice{let date=p3_core::calendar::Calendar::for_tick(self.tick);p3_state::messages::add_supply_notice(&mut pool,&mut merchants[notice.recipient],date.day,date.month,date.year,&notice.record)?;}Ok(SupplyDeadlinePrepared{task,towns,pool,merchants,keep})
    }
    fn reminder_preview(&self,s:&p3_core::scheduler::Scheduler)->Result<(p3_core::scheduler::TaskRecord,Option<p3_core::rng::CrtRand>,bool,Option<(u16,u8)>),String>{
        let mut task=*s.records.get(usize::from(s.head)).ok_or("tarea de recordatorio ausente")?;let mut rng=*self.investigation_rng;let(keep,notice)=p3_core::economy::reminder::advance(&mut task,self.merchants,self.world.merchant_count,&mut rng)?;if keep&&self.reminder_gate.is_none(){return Err("modo de turnos de recordatorio ausente".into())}Ok((task,rng,keep,notice))
    }
    fn harvest_preview(&self,s:&p3_core::scheduler::Scheduler)->Result<HarvestPrepared,String>{
        let mut task=*s.records.get(usize::from(s.head)).ok_or("tarea de cosecha ausente")?;let phase=task.u32_at(8);let city=usize::from(u16::from_le_bytes([task.u8_at(0x10),task.u8_at(0x11)]));
        let offices=if matches!(phase,1|4)&&city<self.world.towns.len(){let images=self.scheduled_office_images.as_ref().ok_or("oficinas de cosecha ausentes")?;if images.len()!=self.world.offices.len(){return Err("oficinas de cosecha incompletas".into())}images.as_slice()}else{&[]};
        let mut towns=self.world.towns.clone();let(keep,notices)=p3_core::economy::harvest::advance(&mut task,&mut towns,offices,self.world.day_of_year)?;let mut pool=self.pool.clone();let mut merchants=self.merchants.to_vec();let date=p3_core::calendar::Calendar::for_tick(self.tick);
        for notice in notices{if notice.recipient==-2{p3_state::messages::add_global_inline_letter(&mut pool,date.day,date.month,date.year,&notice.record)?}else if let Some(person)=merchants.get_mut(notice.recipient as usize){p3_state::messages::add_harvest_notice(&mut pool,person,date.day,date.month,date.year,&notice.record)?;}}
        Ok(HarvestPrepared{task,towns,pool,merchants,keep})
    }
    fn dossier_preview(&self,s:&p3_core::scheduler::Scheduler)->Result<MatchmakerPrepared,String>{
        let task=*s.records.get(usize::from(s.head)).ok_or("tarea de dossier ausente")?;
        let first=self.candidate_look_counts.unwrap_or((0,0));let last=self.ordinary_family_names.unwrap_or(0);
        let mut merchants=self.merchants.to_vec();let offer=p3_core::economy::matchmaker::prepare_dossier(&task,&merchants,self.world.towns.len(),first,last)?;let mut pool=self.pool.clone();
        if let Some(offer)=offer{let date=p3_core::calendar::Calendar::for_tick(self.tick);p3_state::messages::add_matchmaker_offer(&mut pool,&mut merchants[offer.recipient],date.day,date.month,date.year,&offer.record)?;}
        Ok(MatchmakerPrepared{task,pool,merchants})
    }
    fn matchmaker_preview(&self,s:&p3_core::scheduler::Scheduler)->Result<MatchmakerPrepared,String>{
        let mut task=*s.records.get(usize::from(s.head)).ok_or("tarea de casamentero ausente")?;
        let parsed=p3_save::tail::MessagePool::parse(self.pool).map_err(|e|format!("pool: {e:?}"))?;
        let mut messages=Vec::with_capacity(usize::from(parsed.slots));for i in 0..usize::from(parsed.slots){messages.push(parsed.letter(i).get(..16).ok_or("carta truncada")?.try_into().expect("longitud comprobada"))}
        let date=p3_core::calendar::Calendar::for_tick(self.tick);let first=self.candidate_look_counts.map(|v|v.0).or_else(||self.expedition_resources.as_ref().map(|v|v.first_names)).unwrap_or(0);let last=self.ordinary_family_names.unwrap_or(0);
        let mut merchants=self.merchants.to_vec();let offers=p3_core::economy::matchmaker::prepare(&mut task,self.tick,date.year,self.world.day_of_year,&self.world.towns,&merchants,&messages,first,last)?;let mut pool=self.pool.clone();
        for offer in offers{p3_state::messages::add_matchmaker_offer(&mut pool,&mut merchants[offer.recipient],date.day,date.month,date.year,&offer.record)?;}
        Ok(MatchmakerPrepared{task,pool,merchants})
    }
    fn mission_script_preview(&self,s:&p3_core::scheduler::Scheduler)->Result<MissionPrepared,String>{
        let i=usize::from(s.head);let mut task=*s.records.get(i).ok_or("tarea de guion ausente")?;
        if task.u32_at(8)==0{task.set_u32_at(0xc,0);return Ok(MissionPrepared{flat_created:None,flat_rng:None,notifications:Vec::new(),task,vars:Vec::new(),finished:true,names:None,pool:None,merchants:None,scheduler:None,ship_reserve:Vec::new(),operations:Vec::new(),flat_towns:None,flat_ships:Vec::new(),flat_convoys:Vec::new(),flat_offices:Vec::new()})}
        let id=u16::from_le_bytes([task.u8_at(0x10),task.u8_at(0x11)]);let bank=self.mission_bank.as_ref().ok_or("recursos de guiones no configurados")?;let blob=bank.get(id).ok_or_else(||format!("guion {id} ausente del CPR"))?;
        let mut vars=self.mission_variables.get(i).and_then(Option::as_ref).ok_or("variables del guion ausentes")?.clone();
        let context=self.world.towns.get(1).map(|t|p3_core::mission_script::TownContext{scheduler_head:s.head,town_count:self.world.towns.len() as u32,adjacent_dword:t.u32(0x2d4)});
        let count=usize::from(self.world.merchant_count);if self.merchants.len()<count{return Err("comerciantes del guion incompletos".into())}let flags=self.merchants[..count].iter().map(|m|m.u16(8)).collect::<Vec<_>>();let populations=self.world.towns.iter().map(|t|t.i32(0x2d4)).collect::<Vec<_>>();let wall_levels=self.world.towns.iter().zip(self.sites.iter()).map(|(t,site)|site.wall_expansion_level(t)).collect::<Vec<_>>();let mut humans=p3_core::mission_script::HumanContext{populations:&populations,merchant_flags:&flags,merchants:Some(&self.merchants[..count]),tasks:Some(&s.records),wall_levels:Some(&wall_levels),town_name_bank:None,game_town_names:None};
        let heads=self.world.towns.iter().map(|t|t.u16(0x784)).collect::<Vec<_>>();let owners=self.offices.iter().map(|o|o.owner).collect::<Vec<_>>();let next=self.offices.iter().map(|o|o.next).collect::<Vec<_>>();let merchant_heads=self.merchants[..count].iter().map(|m|m.u16(0xc)).collect::<Vec<_>>();let merchant_offices=self.scheduled_office_images.as_ref().and_then(|images|images.iter().map(|image|Some((u16::from_le_bytes(image.get(0x2c8..0x2ca)?.try_into().ok()?),*image.get(0x2c6)?))).collect::<Option<Vec<_>>>());let merchant_next=merchant_offices.as_ref().map(|rows|rows.iter().map(|r|r.0).collect::<Vec<_>>());let office_towns=merchant_offices.as_ref().map(|rows|rows.iter().map(|r|r.1).collect::<Vec<_>>());let merchant_chains=merchant_next.as_ref().zip(office_towns.as_ref()).map(|(next,towns)|p3_core::mission_script::MerchantOfficeContext{heads:&merchant_heads,next,towns});let offices=p3_core::mission_script::OfficeContext{town_heads:&heads,owners:&owners,next:&next,merchant_count:self.world.merchant_count,office_count:self.world.office_count,merchant_chains};
        let random=self.navigation.as_ref().and_then(|nav|{let snap=&nav.snapshot;if snap.manager.len()<0xf6{return None}let head=u16_at(&snap.manager,0xea);let count=u16_at(&snap.manager,0xf4);let sea_position=if head<count{let at=usize::from(head)*0x180;let ship=snap.ships.get(at..at+0x24)?;Some((u32_at(ship,0x1c),u32_at(ship,0x20)))}else{None};Some(p3_core::mission_script::RandomContext{scheduler_head:s.head,sea_position})});
        let mut names=self.expedition_factory.as_ref().zip(self.expedition_resources.as_ref()).and_then(|(factory,resources)|{if let Some(nav)=self.navigation.as_ref(){if nav.snapshot.manager.len()<0xf2||u16_at(&nav.snapshot.manager,0xee)!=factory.name_cursor||u16_at(&nav.snapshot.manager,0xf0)!=factory.name_step{return None}}Some(p3_core::mission_script::NameContext{cursor:factory.name_cursor,step:factory.name_step,count:u16::try_from(resources.ship_names.len()).ok()?,scheduler_head:s.head,surname_count:resources.last_names,surname_seed:self.world.towns.get(6).zip(self.world.towns.get(7)).map(|(a,b)|(a.u32(0x31c),b.u32(0x348))),first_counts:*self.candidate_look_counts})});
        let route=self.mission_distances.map(|distances|p3_core::mission_script::RouteContext{town_count:self.world.towns.len(),distances});
        let town_names=self.mission_town_names.as_ref().and_then(|bank|self.mission_town_ids.get(..self.world.towns.len())?.iter().map(|&id|bank.for_map(id).map(|(name,_)|name.to_vec())).collect::<Option<Vec<_>>>()).unwrap_or_default();
        let empty_first_names=[Vec::new(),Vec::new()];
        let format_extras=self.mission_texts.as_ref().map(|t|p3_core::mission_format::LazyFormatExtras::new(move||{let mut f=t.format_static.clone();if let Some(nav)=self.navigation.as_ref(){let n=usize::from(u16::from_le_bytes([nav.snapshot.manager[0xf4],nav.snapshot.manager[0xf5]])).min(nav.snapshot.ships.len()/0x180);f.ship_records=(0..n).map(|i|nav.snapshot.ships[i*0x180+0x160..i*0x180+0x180].split(|&b|b==0).next().unwrap_or_default().to_vec()).collect()}f.local_count=count as u16;
            // `%n/%v`: punteros cacheados del comerciante (+0xE4/+0xE8) en el banco de nombres actual.
            if let (Some(first),Some(family))=(self.mission_first_names.as_ref(),self.mission_family_names.as_ref()){if let Ok(strings)=p3_core::mission_format::merchant_name_strings(&self.merchants[..count],first,family){f.merchant_strings=strings}}
            // `%F` (0x5333C0): Ältermann (`GameWorld+0x8D`, ciudad `+0x8E`) y consejeros de cada ciudad.
            if let Ok(visibility)=self.world.towns.iter().map(|t|p3_core::mission_format::town_alderman_visibility(&t.0[..])).collect::<Result<Vec<_>,_>>(){f.local_current=Some(self.world.alderman_merchant);f.local_alt=self.world.alderman_town;f.local_visibility=visibility}
            f}));let mut letters=p3_core::mission_script::LetterContext{scheduler_head:s.head,resources_available:self.expedition_resources.is_some(),ship_names:self.expedition_resources.as_ref().map_or(&[][..],|r|r.ship_names.as_slice()),town_names:&town_names,family_names:self.mission_family_names.as_deref().unwrap_or(&[]),first_names:self.mission_first_names.as_ref().unwrap_or(&empty_first_names),effects:Vec::new(),merchant_images:&self.merchants[..count],gender_words:self.mission_texts.as_ref().map(|texts|&texts.gender_words),format_extras:format_extras.as_ref().map(|lazy|lazy as &dyn p3_core::mission_format::FormatExtrasSource)};
        let bank_names=self.mission_town_names.as_ref().map(|b|b.map_names());humans.town_name_bank=bank_names.as_deref();humans.game_town_names=if town_names.is_empty(){None}else{Some(&town_names)};let reserve_ops=std::cell::RefCell::new(Vec::new());
        let ship_heads=self.merchants[..count].iter().map(|m|m.u16(0xe)).collect::<Vec<_>>();let ship_flags=self.merchants[..count].iter().map(|m|m.u16(8)).collect::<Vec<_>>();let ships=self.navigation.as_ref().map(|nav|p3_core::mission_script::ShipContext{merchant_heads:&ship_heads,ships:&nav.snapshot.ships,merchant_flags:&ship_flags,reserve:Some(&reserve_ops)});
        let mut harvest=Vec::new();let (task0,vars0,names0)=(task,vars.clone(),names);let mut flat=None;
        // Comandos que solo existen sobre la imagen plana del mundo: se repite el guion entero con ella sembrada.
        let outcome=match p3_core::mission_script::execute_in_world_with_ships(blob,&mut task,&mut vars,self.tick,context,Some(humans),Some(offices),random,names.as_mut(),Some(&mut letters),route,Some(&mut harvest),ships){
            Err(e) if c10_script_flat::needs_flat(&e)&&self.mission_flat_rdata.is_some()=>{
                task=task0;vars=vars0;names=names0;letters.effects.clear();harvest.clear();reserve_ops.borrow_mut().clear();
                let mut image=self.mission_flat_image(s).map_err(|f|format!("{e} ({f})"))?;let word=u16::from_le_bytes([task.u8_at(0x10),task.u8_at(0x11)]);
                let out=p3_core::mission_script::execute_in_world_flat(blob,&mut task,&mut vars,self.tick,context,Some(humans),Some(offices),random,names.as_mut(),Some(&mut letters),route,Some(&mut harvest),ships,Some(&mut image.script(word)));
                match out{Ok(o)=>{flat=Some(image.finish(&self.world.towns,self.world.merchant_count)?);o}Err(e)=>return Err(image.mem.error.take().unwrap_or(e))}
            }
            r=>r?,
        };
        let finished=outcome==p3_core::mission_script::Outcome::Finished;let reserve_ops=reserve_ops.into_inner();
        let base_s=flat.as_ref().map_or(s,|f:&c10_script_flat::FlatOut|&f.scheduler);let scheduler=if harvest.is_empty(){flat.as_ref().map(|f|{let mut next=f.scheduler.clone();let mut t=task;if let Some(r)=next.records.get_mut(i){t.next=r.next;*r=t}next})}else{if self.tick==u32::MAX{return Err("fecha de cosecha desbordada no contrastada".into())}let mut next=base_s.clone();if next.records.len()!=usize::from(next.capacity){return Err("planificador truncado".into())}let mut at=next.head;let mut seen=vec![false;usize::from(next.capacity)];while at<next.capacity{let j=usize::from(at);if seen[j]{return Err("planificador cíclico".into())}seen[j]=true;at=next.records[j].next}for spawn in harvest{if next.free_head>next.capacity||next.free_head==next.capacity&&next.capacity>u16::MAX-128{return Err("capacidad del planificador de cosecha inválida".into())}let slot=usize::from(next.free_head);if self.mission_variables.get(slot).is_some_and(Option::is_some){return Err("variables de guion en ranura libre".into())}next.dispatching=0;let id=next.add_task(self.tick+1);next.dispatching=1;if next.head!=s.head{return Err("alta de cosecha reordena la tarea activa".into())}let child=&mut next.records[usize::from(id)];child.kind=9;child.set_u32_at(8,0);child.set_u32_at(12,spawn.duration);let city=spawn.city.to_le_bytes();child.set_u8_at(0x10,city[0]);child.set_u8_at(0x11,city[1]);child.set_u8_at(0x12,0);child.set_u8_at(0x13,0);child.set_u8_at(0x14,0)}task.next=next.records[i].next;next.records[i]=task;Some(next)};
        let town_recipients=|city:usize|->Result<Vec<usize>,String>{
            let mut at=*heads.get(city).ok_or("ciudad de carta ausente")?;
            let mut seen=vec![false;usize::from(self.world.office_count)];let mut result=Vec::new();
            while at<self.world.office_count {let i=usize::from(at);if seen[i]{return Err("cadena urbana de oficinas cíclica".into())}seen[i]=true;
                let owner=usize::from(*owners.get(i).ok_or("oficina de carta ausente")?);if owner>=count{return Err("dueño de oficina de carta ausente".into())}result.push(owner);
                at=*next.get(i).ok_or("enlace de oficina de carta ausente")?;
            }Ok(result)
        };
        let mut notifications=Vec::new();
        // 0xE7/0xE9/0xF9: la operación 0x82 se publica en la cola salvo en cliente de red (0x004ED12A).
        let (pirates,effects):(Vec<_>,Vec<_>)=std::mem::take(&mut letters.effects).into_iter().partition(|e|matches!(e,p3_core::mission_script::LetterEffect::MissionPirate(_)));letters.effects=effects;
        let mut operations:Vec<p3_core::pump::Operation>=if self.world.network_client!=0{Vec::new()}else{pirates.into_iter().filter_map(|e|match e{p3_core::mission_script::LetterEffect::MissionPirate(r)=>Some(p3_core::pump::Operation(r)),_=>None}).collect()};if let Some(f)=flat.as_mut(){if self.world.network_client==0{operations.extend(std::mem::take(&mut f.operations))}}
        let (pool,merchants)=if !letters.effects.is_empty()||flat.is_some(){
            let (mut pool,mut merchants)=match flat.as_mut(){Some(f)=>{let mut m=self.merchants.to_vec();m[..f.merchants.len()].clone_from_slice(&f.merchants);(std::mem::take(&mut f.pool),m)}None=>(self.pool.clone(),self.merchants.to_vec())};let date=p3_core::calendar::Calendar::for_tick(self.tick);
            for effect in letters.effects {match effect {
                p3_core::mission_script::LetterEffect::MissionPirate(_)=>unreachable!("separadas antes"),
                p3_core::mission_script::LetterEffect::Deliver(letter)=>{
                    let urban=if letter.recipient==u32::MAX-2 {Some(town_recipients(usize::from(letter.town))?)}else{None};
                    if let Some(recipients)=urban.as_ref(){p3_state::messages::add_script_letter_town(&mut pool,&mut merchants,self.world.merchant_count,date.day,date.month,date.year,&letter,recipients)?;}
                    else {p3_state::messages::add_script_letter(&mut pool,&mut merchants,self.world.merchant_count,date.day,date.month,date.year,&letter)?;}
                    if let Some(args)=letter.notification {let player=self.expedition_resources.as_ref().map(|r|u32::from(r.player));let mut refresh=letter.recipient==u32::MAX||player==Some(letter.recipient);if !refresh&&letter.recipient==u32::MAX-2{let query=town_recipients(args[3] as usize)?;refresh=player.is_some_and(|p|query.contains(&(p as usize)))}notifications.push(ScriptNotification{args,refresh_mail_requested:refresh});}
                },
                p3_core::mission_script::LetterEffect::Notify{args,recipient}=>{let player=self.expedition_resources.as_ref().map(|r|u32::from(r.player));notifications.push(ScriptNotification{args,refresh_mail_requested:recipient==u32::MAX||player==Some(recipient)});},
                p3_core::mission_script::LetterEffect::RemoveTreasure{merchant,entry}=>p3_state::messages::remove_treasure_letters(&mut pool,&mut merchants,self.world.merchant_count,merchant,entry)?,
                p3_core::mission_script::LetterEffect::Gold{merchant,amount}=>{let m=merchants.get_mut(merchant as usize).ok_or("comerciante del guion ausente")?;m.set_i32(0,m.i32(0).wrapping_add(amount as i32));if (amount as i32)<0{m.set_i32(0x4bc,m.i32(0x4bc).wrapping_sub(amount as i32))}else{m.set_i32(0x4b8,m.i32(0x4b8).wrapping_add(amount as i32))}},
            }}(Some(pool),Some(merchants))
        }else{(None,None)};
        let flat_rng=flat.as_ref().and_then(|f|f.rng);let flat_created=flat.as_mut().and_then(|f|f.created.take());if let Some(c)=flat_created.as_ref(){self.check_created_ships(c)?;let before=self.expedition_factory.as_ref().map(|f|f.name_cursor);if names.as_ref().is_some_and(|n|Some(n.cursor)!=before){return Err("guion con alta naval y nombres del intérprete estructurado: no contrastado".into())}names=None}let (flat_ships,flat_convoys,flat_offices)=flat.as_mut().map_or((Vec::new(),Vec::new(),Vec::new()),|f|(std::mem::take(&mut f.ship_writes),std::mem::take(&mut f.convoy_writes),std::mem::take(&mut f.offices)));
        if flat_offices.iter().any(|o|o.0>=self.stocks.len()||o.1.len()<0x64){return Err("oficinas del guion plano sin copia viva".into())}
        Ok(MissionPrepared{flat_created,flat_rng,flat_ships,flat_convoys,flat_offices,notifications,task,vars,finished,names,pool,merchants,scheduler,ship_reserve:reserve_ops,operations,flat_towns:flat.map(|f|(f.towns,f.changed_towns)).filter(|(_,c)|!c.is_empty())})
    }
    fn weather_preview(&self,s:&p3_core::scheduler::Scheduler)->Result<WeatherPrepared,String>{
        use p3_core::economy::daily_weather::{self,WeatherStep};
        let index=usize::from(s.head);let mut task=*s.records.get(index).ok_or("tarea meteorológica ausente")?;let steps=daily_weather::daily_plan(&mut task,self.world.day_of_year,self.world.network_client);let mut scheduler=s.clone();let mut notices=Vec::new();let mut towns=self.world.towns.clone();let mut marker=*self.weather_marker;let mut rng=*self.investigation_rng;let mut operations=Vec::new();
        for step in steps{match step {
            WeatherStep::Seasonal=>daily_weather::seasonal_ice(self.world.day_of_month,self.world.month,&mut towns),
            WeatherStep::TownMarker=>daily_weather::update_town_marker(&mut towns,marker.as_mut().ok_or("marcador meteorológico del mundo ausente")?,self.world.day_of_year).map_err(str::to_owned)?,
            WeatherStep::StormMarkers=>{let nav=self.navigation.as_ref().ok_or("gestor meteorológico naval ausente")?;let locations=self.mission_locations.as_ref().ok_or("posiciones meteorológicas ausentes")?;daily_weather::update_storm_markers(&mut towns,self.mission_town_ids,locations,&nav.snapshot.manager).map_err(str::to_owned)?},
            WeatherStep::Ice=>{let locations=self.mission_locations.as_ref().ok_or("posiciones de heladas ausentes")?;let frozen=daily_weather::ice_tick(&mut towns,self.mission_town_ids,locations,self.world.day_of_year,self.tick,self.weather_speed).map_err(str::to_owned)?;for freeze in frozen{if freeze.due<=self.tick{return Err("deshielo con fecha desbordada no contrastado".into())}scheduler.dispatching=0;let id=scheduler.add_task(freeze.due);let record=&mut scheduler.records[usize::from(id)];record.kind=0x35;record.set_u32_at(8,u32::from(freeze.city));scheduler.dispatching=1;notices.push((freeze.city,false))}},
            WeatherStep::Icebergs=>{let random=rng.as_mut().ok_or("estado CRT de icebergs ausente")?;let nav=self.navigation.as_ref().ok_or("gestor naval de icebergs ausente")?;let planner=self.return_planner.as_ref().ok_or("mapa de navegación de icebergs ausente")?;operations.extend(daily_weather::winter_icebergs(self.world.day_of_year,random,&nav.snapshot.manager,&nav.snapshot.ships,|x,y|planner.nav.grid.is_point_navigable(x,y)).map_err(str::to_owned)?)},
            WeatherStep::SummerPhenomena=>{let random=rng.as_mut().ok_or("estado CRT de tormentas ausente")?;let nav=self.navigation.as_ref().ok_or("gestor naval de tormentas ausente")?;let planner=self.return_planner.as_ref().ok_or("mapa de navegación de tormentas ausente")?;let queued=u16::try_from(self.expedition_operations.len()+operations.len()).map_err(|_|"cola de operaciones demasiado grande")?;let grid=&planner.nav.grid;operations.extend(daily_weather::summer_weather(self.world.day_of_year,queued,grid.scale,random,&nav.snapshot.manager,&nav.snapshot.ships,|x,y|grid.is_point_navigable(x,y)).map_err(str::to_owned)?)},
        }}
        // Si la helada amplió la tabla, el original escribe la nueva fecha por el puntero viejo (bloque liberado):
        // la tarea conserva su vencimiento y el despachador la vuelve a ejecutar en el mismo tick.
        if scheduler.capacity==s.capacity{scheduler.records[index].due=task.due;scheduler.records[index].set_u32_at(8,task.u32_at(8));}Ok(WeatherPrepared{scheduler,towns,marker,notices,rng,operations})
    }
    fn merchant_ai_preview(&self,task:p3_core::scheduler::TaskRecord,s:&p3_core::scheduler::Scheduler)->Result<MerchantAiPrepared,String>{
        let id=u16::from_le_bytes([task.u8_at(8),task.u8_at(9)]);let m=self.merchants.get(usize::from(id)).ok_or("comerciante ausente")?;
        let rep_factor=if m.u16(8)&8!=0 {
            let count=usize::from(self.world.merchant_count);
            if self.merchants.len()<count || self.world.towns.len()>40 || self.merchants[..count].iter().any(|m|m.bytes.len()<0x650) {return Err("datos de reputación IA incompletos".into())}
            let local=self.world.local_merchant as usize;
            if self.operations_mode&0xe==0 && local>=count {return Err("jugador local de reputación IA ausente".into())}
            let controls=self.merchants[..count].iter().map(|m|m.u16(8)).collect::<Vec<_>>();
            struct Scores<'a>{merchants:&'a [RepMerchant],towns:&'a [Town]}
            impl p3_core::ai::bids::ScoreHost for Scores<'_>{fn merchant_score(&mut self,i:usize)->i16{self.merchants[i].calc_total_reputation_score(self.towns.len() as i32,self.towns) as i16}}
            let env=p3_core::ai::bids::BaseFactorEnv{scale:self.rep_scale,mode:self.operations_mode,local,merchant_count:self.world.merchant_count,controls:&controls};
            let factor=p3_core::ai::bids::update_base_rep_factor(usize::from(id),&env,&mut Scores{merchants:self.merchants,towns:&self.world.towns});
            Some((usize::from(id),factor.to_bits()))
        }else{None};
        let flags=*m.bytes.get(8).ok_or("comerciante truncado")?;let b547=*m.bytes.get(0x547).ok_or("comerciante truncado")?;
        let person=p3_core::ai::turn::TurnMerchant{gold:m.i32(0),flags,b547};
        let no_offices=m.u16(0xc)>=self.world.office_count&&self.world.towns.len()<100;
        let prepared=p3_core::ai::scheduled_turn::prepare_with_stock(task,&person,self.world.network_client,self.world.world_8a!=0,no_offices).map_err(str::to_owned)?;
        let grant=if prepared.grant {let live=self.world.merchants.get(usize::from(id)).ok_or("cuenta de comerciante ausente")?;if live.gold!=person.gold{return Err("cuenta de comerciante desincronizada".into())}Some((usize::from(id),person.gold.wrapping_add(0x3a98)))}else{None};
        let operations=if prepared.housing||prepared.guild{let images=self.scheduled_office_images.as_ref().ok_or("imágenes de oficinas ausentes")?;crate::merchant_ai_decisions::plan(self.world,self.sites,images,m,id,prepared.housing,prepared.guild)?}else{Vec::new()};
        let mut changes=Vec::new();
        if prepared.stock {
            let images=self.scheduled_office_images.as_ref().ok_or("imágenes de oficinas ausentes")?;
            if images.len()!=self.world.offices.len()||self.stocks.len()<images.len()||images.iter().any(|v|v.len()!=0x44c){return Err("oficinas de IA incompletas".into())}
            let mut offices=images.iter().zip(self.stocks.iter()).map(|(image,stock)|p3_core::ai::stock::StockOffice{town:image[0x2c6],next:u16_at(image,0x2c8),wants:stock.business_consumption,targets:stock.ai_target,marks:stock.automation_flags as u8}).collect::<Vec<_>>();
            let mut seen=vec![false;offices.len()];let mut current=m.u16(0xc);
            while usize::from(current)<offices.len(){let i=usize::from(current);if seen[i]{return Err("cadena de oficinas IA cíclica".into())}seen[i]=true;
                if usize::from(offices[i].town)>=self.world.towns.len()||self.stocks[i].owner!=id||u16_at(&images[i],0x2c4)!=id||self.stocks[i].town!=offices[i].town{return Err("oficina de IA incoherente".into())}current=offices[i].next;
            }
            let town_flags=self.world.towns.iter().map(|t|t.flags()).collect::<Vec<_>>();
            p3_core::ai::stock::update_office_trade_stock_targets(&mut offices,m.u16(0xc),&town_flags);
            for(i,changed)in seen.into_iter().enumerate(){if changed{let office=&offices[i];let mut image=images[i].clone();let flags=(self.stocks[i].automation_flags&0xff00)|u16::from(office.marks);image[0x2d6..0x2d8].copy_from_slice(&flags.to_le_bytes());for (k,value) in office.targets.iter().enumerate(){image[0x354+4*k..0x358+4*k].copy_from_slice(&value.to_le_bytes())}changes.push((i,office.targets,flags,image));}}
        }
        let mut fleet_changes=Vec::new();
        if prepared.arm||prepared.home {
            use p3_core::ai::ships::{Fleet,HomeEnv,OfficeLink};
            let count=usize::from(self.world.ship_count);let images=self.expedition_images.get(..count).ok_or("flota de IA ausente")?;
            let mut fleet=if let Some(nav)=self.navigation.as_ref(){if nav.snapshot.ships.len()<count*0x180{return Err("flota de IA truncada".into())}Fleet{bytes:nav.snapshot.ships[..count*0x180].to_vec()}}else{let mut raw=Vec::with_capacity(count*0x180);for image in images{if let Some(image)=image{if image.len()!=0x180{return Err("barco de IA truncado".into())}raw.extend_from_slice(image)}else{raw.extend_from_slice(&[0u8;0x180])}}Fleet{bytes:raw}};
            let mut at=usize::from(m.u16(0xe));let mut seen=vec![false;count];while at<count{if seen[at]{return Err("cadena naval de IA cíclica".into())}seen[at]=true;let image=images[at].as_ref().ok_or("barco de IA ausente")?;let o=at*0x180;if image[4..6]!=fleet.bytes[o+4..o+6]||[0xe,0xf,0x136,0x13a].into_iter().any(|j|image[j]!=fleet.bytes[o+j]){return Err("vistas navales de IA desincronizadas".into())}at=usize::from(u16_at(&fleet.bytes,o+4));}
            let before=fleet.bytes.clone();
            if prepared.arm{fleet.mark_ship_for_arming(m.u16(0xe));}
            if prepared.home{
                let towns=self.world.towns.len();let home=usize::from(m.bytes[0x19]);if towns==0||towns>40||home>=towns{return Err("ciudad base de IA inválida".into())}
                let offices=self.scheduled_office_images.as_ref().ok_or("oficinas de IA ausentes")?;if offices.len()!=usize::from(self.world.office_count)||offices.iter().any(|image|image.len()<0x2ca){return Err("oficinas de IA truncadas".into())}
                let links=offices.iter().map(|image|OfficeLink{town:image[0x2c6],next:u16_at(image,0x2c8)}).collect::<Vec<_>>();let mut seen=vec![false;links.len()];let mut at=usize::from(m.u16(0xc));while at<links.len(){if seen[at]{return Err("cadena de oficinas IA cíclica".into())}seen[at]=true;if usize::from(links[at].town)>=towns{return Err("ciudad de oficina IA inválida".into())}at=usize::from(links[at].next)}
                let mut flags=[0u32;40];for(i,town)in self.world.towns.iter().enumerate(){flags[i]=town.flags()}
                fleet.assign_ship_home_towns(&HomeEnv{town_count:towns as u32,town_flags:&flags,offices:&links,office_head:m.u16(0xc),ship_head:m.u16(0xe),home:home as u8});
            }
            for i in 0..count{let o=i*0x180;if [0xf,0x136,0x13a].into_iter().any(|j|before[o+j]!=fleet.bytes[o+j]){fleet_changes.push((i,fleet.bytes[o+0xf],fleet.bytes[o+0x136],fleet.bytes[o+0x13a]));}}
        }
        // 0x004F4E50 tras existencias y flota y antes de vivienda y gremio: sus operaciones van delante.
        let mut rng=None;let operations=if prepared.cash{let mut images=self.scheduled_office_images.as_ref().ok_or("imágenes de oficinas ausentes")?.clone();for(i,_,_,image)in &changes{images[*i]=image.clone();}let(mut cash,r)=self.ai_raise_cash(s,id,m,&images)?;rng=r;cash.extend(operations);cash}else{operations};
        Ok(MerchantAiPrepared{task:prepared.task,delay:prepared.delay,stock_changes:changes,grant,operations,rep_factor,fleet_changes,rng})
    }

    fn town_event_plan_preview(&self,s:&p3_core::scheduler::Scheduler)->Result<p3_core::economy::town_event_plan_full::Prepared,String>{
        use p3_core::economy::town_event_prelude::{self,Outcome};
        let task=*s.records.get(usize::from(s.head)).ok_or("tarea anual de sucesos ausente")?;
        let candidate=matches!(town_event_prelude::prepare(&task,&self.world.towns,self.world.day_of_year,self.operations_mode!=0,*self.investigation_rng)?,Outcome::Candidate{..});
        let mut facilities=Vec::new();let mut offices=Vec::new();let mut bands=[None;5];
        if candidate{
            let count=usize::from(self.world.merchant_facility_count);
            if count>self.employment_businesses.len()||count>self.world.merchant_facilities.len(){return Err("negocios del plan urbano incompletos".into())}
            facilities=self.employment_businesses[..count].to_vec();for (image,live) in facilities.iter_mut().zip(&self.world.merchant_facilities){image[4..6].copy_from_slice(&live.employees.to_le_bytes());image[8..10].copy_from_slice(&live.next.to_le_bytes())}
            offices=self.scheduled_office_images.as_ref().ok_or("oficinas del plan urbano ausentes")?.clone();
            if offices.len()!=self.world.offices.len()||self.stocks.len()<offices.len()||offices.iter().any(|v|v.len()!=0x44c){return Err("oficinas del plan urbano incompletas".into())}
            for (i,image) in offices.iter_mut().enumerate(){image[0x2ca..0x2cc].copy_from_slice(&self.world.offices[i].next.to_le_bytes());for k in 0..24{image[0x64+4*k..0x68+4*k].copy_from_slice(&self.stocks[i].business_consumption[k].to_le_bytes())}}
            let live=self.event_bands_now()?;for i in 0..5{bands[i]=live[i].as_ref().map(|b|u16_at(&b.header,14))}
        }
        p3_core::economy::town_event_plan_full::prepare(p3_core::economy::town_event_plan_full::Inputs{scheduler:s,towns:&self.world.towns,merchants:self.merchants,merchant_count:self.world.merchant_count,facilities:&facilities,offices:&offices,bands,facility_byte:0,tick:self.tick,day:self.world.day_of_year,operations_remote:self.operations_mode!=0,rng:*self.investigation_rng})
    }
    fn town_event_preview(&self,s:&p3_core::scheduler::Scheduler)->Result<TownEventPrepared,String>{
        let task=*s.records.get(usize::from(s.head)).ok_or("tarea urbana ausente")?;let city=usize::from(task.u8_at(0x11));let town=self.world.towns.get(city).ok_or("ciudad de suceso ausente")?;
        let mut people=self.merchants.to_vec();if usize::from(self.world.merchant_count)>people.len()||usize::from(self.world.merchant_count)>self.world.merchants.len(){return Err("comerciantes de suceso truncados".into())}for (person,account) in people.iter_mut().zip(self.world.merchants.iter()).take(usize::from(self.world.merchant_count)){person.set_i32(0,account.gold);person.set_i32(0x4c8,account.income)}
        let mut offices=Vec::new();let mut stocks=self.stocks.to_vec();let mut before=Vec::new();if task.u8_at(0x10)==4{offices=self.scheduled_office_images.as_ref().ok_or("oficinas de suceso ausentes")?.clone();if offices.len()!=self.world.offices.len()||stocks.len()<offices.len()||offices.iter().any(|v|v.len()!=0x44c){return Err("oficinas de suceso truncadas".into())}for (i,image) in offices.iter_mut().enumerate(){for k in 0..24{image[4+4*k..8+4*k].copy_from_slice(&stocks[i].stock[k].to_le_bytes())}for k in 0..4{image[0x124+4*k..0x128+4*k].copy_from_slice(&self.world.offices[i].weapons[k].to_le_bytes())}}before=offices.clone();}
        let (ships,radius,hideout)=if matches!(task.u8_at(0x10),2|3){let nav=self.navigation.as_ref().ok_or("flota de suceso ausente")?;let manager=&nav.snapshot.manager;if manager.len()<0xfe{return Err("gestor naval de suceso truncado".into())}let map=*self.mission_town_ids.get(city).ok_or("mapa de ciudad de suceso ausente")?;let loc=self.mission_locations.as_ref().and_then(|v|v.get(usize::from(map))).ok_or("posición de suceso ausente")?;(&nav.snapshot.ships[..],u16_at(manager,0xfc),Some((loc.x,loc.y)))}else{(&[][..],0,None)};
        let out=town_event_runtime::prepare_deferred(town_event_runtime::Inputs{task,town,offices:&offices,merchants:&people,merchant_count:self.world.merchant_count,ships,radius,hideout,tick:self.tick,pool:self.pool})?;
        let spawn=if let Some((target,mode))=out.spawn{
            let nav=self.navigation.as_ref().ok_or("flota de suceso ausente")?;
            let factory=self.expedition_factory.as_ref().ok_or("fábrica de suceso ausente")?;
            let portraits=self.pirates.as_ref().ok_or("retratos de suceso ausentes")?;
            let resources=self.expedition_resources.as_ref().ok_or("recursos navales de suceso ausentes")?;
            let bands=self.event_bands_now()?;
            if self.hunt_bands.is_none()||self.event_band_images.is_none()||self.mission_pirate_targets.is_none(){return Err("vistas de bandas ausentes".into())}
            let count=usize::from(self.world.ship_count);let ncv=nav.snapshot.convoys.len()/0x3c;
            if self.world.ships.len()!=count||self.fleet.len()!=count||self.ship_metadata.len()!=count||self.expedition_images.len()!=count||self.expedition_original_images.len()!=count||self.expedition_convoys.len()!=ncv||self.convoy_flags.len()!=ncv{return Err("vistas navales de suceso incompletas".into())}
            let population=self.world.towns.get(usize::from(self.world.day_of_year&15)).ok_or("población de capitán de suceso ausente")?.u32(0x2e0);
            let port0=resources.ports.first().copied().flatten().ok_or("puerto inicial de suceso ausente")?;
            let town_bytes=self.world.towns.iter().flat_map(|t|t.0.iter().copied()).collect::<Vec<_>>();
            let people_bytes=out.merchants.iter().flat_map(|p|p.bytes.iter().copied()).collect::<Vec<_>>();
            let new_band=if bands.iter().all(Option::is_none){Some(town_pirate_spawn::NewBandResources{
                difficulty:self.event_pirate_difficulty,town_order:self.mission_town_ids,
                occupied:self.event_inland.as_ref().ok_or("ocupación de puertos de banda ausente")?,
                locations:self.mission_locations.as_ref().ok_or("ubicaciones de banda ausentes")?,
                piece_bytes:self.event_piece_bytes.ok_or("bytes del ejecutable para banda ausentes")?,
            })}else{None};
            let fleet=town_pirate_spawn::prepare(town_pirate_spawn::Inputs{manager:&nav.snapshot.manager,ships:&nav.snapshot.ships,captains:&nav.snapshot.captains,convoys:&nav.snapshot.convoys,merchants:&people_bytes,towns:&town_bytes,merchant_count:self.world.merchant_count,bands:&bands,target,mode,resources:town_pirate_spawn::Resources{tick:self.tick,difficulty:self.event_battle_difficulty,alderman:self.world.alderman_town,port0,names:&resources.ship_names,captain:p3_core::economy::pirate_group_callbacks::CaptainResources{tick:self.tick,day_of_year:self.world.day_of_year,day:self.world.day_of_month,population,first_names:resources.first_names,last_names:resources.last_names},player:u32::from(resources.player),new_band}})?;
            let staged=town_event_live::prepare(nav,factory,portraits,&fleet)?;
            Some((staged,fleet.bands))
        }else{None};
        let mut updates=Vec::new();let mut weapons=Vec::new();for (i,image) in out.offices.iter().enumerate(){if image!=&before[i]{for k in 0..24{stocks[i].stock[k]=i32::from_le_bytes(image[4+4*k..8+4*k].try_into().unwrap())}let mut w=[0i32;4];for k in 0..4{w[k]=i32::from_le_bytes(image[0x124+4*k..0x128+4*k].try_into().unwrap())}weapons.push((i,w));updates.push(p3_core::economy::world::ScheduledOfficeUpdate{index:i,administrator:u16_at(image,0x2f2),flags:u16_at(image,0x2d6),experience:None,wage:None,image:Some(image.clone())});}}
        Ok(TownEventPrepared{task:out.task,city,town:out.town,offices:if task.u8_at(0x10)==4{Some(out.offices)}else{None},stocks,merchants:out.merchants,pool:out.pool,updates,weapons,keep:out.keep,spawn})
    }
    fn burglary_preview(&self,s:&p3_core::scheduler::Scheduler)->Result<BurglaryPrepared,String>{
        let images=self.scheduled_office_images.as_ref().ok_or("imagenes de oficinas ausentes")?;if images.len()!=self.world.offices.len()||images.iter().any(|v|v.len()!=0x44c)||self.stocks.len()<images.len()||self.world.merchants.len()<usize::from(self.world.merchant_count){return Err("vistas de robo incompletas".into())}
        let mut offices=images.clone();for(i,stock)in self.stocks.iter().take(images.len()).enumerate(){for k in 0..24{offices[i][4+4*k..8+4*k].copy_from_slice(&stock.stock[k].to_le_bytes())}}
        let mut people=self.merchants.to_vec();for(person,account)in people.iter_mut().zip(&self.world.merchants).take(usize::from(self.world.merchant_count)){person.set_i32(0,account.gold);person.set_i32(0x4c8,account.income)}
        let mut core=p3_core::economy::commissioned_burglary::prepare(s,&self.world.towns,&offices,&people,self.world.merchant_count,self.tick)?;let mut pool=self.pool.clone();let date=Calendar::for_tick(self.tick);let retention=[u32::MAX,0x2800,0x1400,0xa00,0x500,0];for notice in &core.notices{let person=core.merchants.get_mut(usize::from(notice.recipient)).ok_or("destinatario de robo ausente")?;p3_state::messages::post_to_merchant(&mut pool,person,date.day,date.month,date.year,&notice.record,p3_state::messages::Payload::None,&retention)?;}
        let mut stocks=self.stocks.to_vec();let mut updates=Vec::new();for(i,(before,after))in offices.iter().zip(&core.offices).enumerate(){if before!=after{for k in 0..24{stocks[i].stock[k]=i32::from_le_bytes(after[4+4*k..8+4*k].try_into().unwrap())}updates.push(p3_core::economy::world::ScheduledOfficeUpdate{index:i,administrator:u16_at(after,0x2f2),flags:u16_at(after,0x2d6),experience:None,wage:None,image:Some(after.clone())})}}
        Ok(BurglaryPrepared{core,stocks,pool,updates})
    }
    fn theft_preview(&self,s:&p3_core::scheduler::Scheduler)->Result<TheftPrepared,String>{
        let images=self.scheduled_office_images.as_ref().ok_or("imágenes de oficinas ausentes")?;if images.len()!=self.world.offices.len()||images.iter().any(|v|v.len()!=0x44c)||self.stocks.len()<images.len(){return Err("oficinas truncadas".into())}
        let mut task=*s.records.get(usize::from(s.head)).ok_or("tarea de robo ausente")?;let mut offices=images.concat();for(i,stock)in self.stocks.iter().take(images.len()).enumerate(){for k in 0..24{let at=i*0x44c+4+4*k;offices[at..at+4].copy_from_slice(&stock.stock[k].to_le_bytes())}}let mut towns=self.world.towns.clone();let outcome=p3_core::economy::warehouse_theft::resolve(&mut task,&mut towns,&mut offices,self.merchants).map_err(str::to_owned)?;let offices=offices.chunks_exact(0x44c).map(|v|v.to_vec()).collect::<Vec<_>>();let mut stocks=self.stocks.to_vec();let mut merchants=self.merchants.to_vec();let mut pool=self.pool.clone();let mut updates=Vec::new();
        if let Some(out)=outcome{let image=&offices[out.office];let stock=stocks.get_mut(out.office).ok_or("existencias de robo ausentes")?;for k in 0..24{stock.stock[k]=i32::from_le_bytes(image[4+4*k..8+4*k].try_into().unwrap())}if let Some(payload)=out.payload{let merchant=merchants.get_mut(usize::from(out.owner)).ok_or("comerciante de robo ausente")?;let d=p3_core::calendar::Calendar::for_tick(self.tick);p3_state::messages::add_warehouse_theft(&mut pool,merchant,d.day,d.month,d.year,out.city,payload)?;}updates.push(p3_core::economy::world::ScheduledOfficeUpdate{index:out.office,administrator:u16_at(image,0x2f2),flags:u16_at(image,0x2d6),experience:None,wage:None,image:Some(image.clone())});}
        Ok(TheftPrepared{task,towns,offices,stocks,merchants,pool,updates})
    }
    fn fire_preview(&self,s:&p3_core::scheduler::Scheduler)->Result<FirePrepared,String>{
        let images=self.scheduled_office_images.as_ref().ok_or("imágenes de oficinas ausentes")?;if images.len()!=self.world.offices.len()||images.iter().any(|v|v.len()!=0x44c)||self.stocks.len()<images.len(){return Err("oficinas truncadas".into())}let mut offices=images.concat();for(i,stock)in self.stocks.iter().take(images.len()).enumerate(){for k in 0..24{let at=i*0x44c+4+4*k;offices[at..at+4].copy_from_slice(&stock.stock[k].to_le_bytes())}}let before=offices.clone();let mut businesses=p3_core::economy::facility_pool::FacilityPool{records:self.employment_businesses.clone(),free:*self.business_free};if businesses.records.len()!=self.world.merchant_facilities.len(){return Err("negocios incompletos".into())}for(b,f)in businesses.records.iter_mut().zip(&self.world.merchant_facilities){b[4..6].copy_from_slice(&f.employees.to_le_bytes());b[8..10].copy_from_slice(&f.next.to_le_bytes());}
        let mut scheduler=s.clone();let mut towns=self.world.towns.clone();let mut sites=self.sites.to_vec();let names=self.merchants.iter().map(|m|m.bytes.get(0x1c..0x1e).and_then(|b|b.try_into().ok()).unwrap_or([0,0])).collect::<Vec<[u8;2]>>();if self.merchants.iter().take(usize::from(self.world.merchant_count)).any(|m|m.bytes.len()<0x1e){return Err("comerciantes de incendio truncados".into())}let outcome=p3_core::economy::fire::step_with_warehouses(&mut scheduler,&mut towns,&mut sites,&mut offices,&mut businesses,self.world.merchant_count,0xe0,self.tick,&names).map_err(str::to_owned)?;let offices=offices.chunks_exact(0x44c).map(|v|v.to_vec()).collect::<Vec<_>>();let mut updates=Vec::new();for (i,after) in offices.iter().enumerate(){if before[i*0x44c..(i+1)*0x44c]!=after[..]{let admin=u16_at(after,0x2f2);let person=self.expedition_factory.as_ref().and_then(|f|f.captains.get(usize::from(admin)));updates.push(p3_core::economy::world::ScheduledOfficeUpdate{index:i,administrator:admin,flags:u16_at(after,0x2d6),experience:person.map(|p|p[0xa]),wage:person.map(|p|u16_at(p,0xc)),image:Some(after.clone())});}}
        if updates.iter().any(|u|u.index>=self.burn_lists.len()||u.index>=self.burn_housing.len()){return Err("vistas de oficina de incendio ausentes".into())}
        let mut pool=self.pool.clone();let mut notices=Vec::new();let mut windows=Vec::new();let mut stocks=self.stocks.to_vec();let mut merchants=self.merchants.to_vec();
        if let Some(loss)=outcome.warehouse{let stock=stocks.get_mut(loss.office).ok_or("existencias de incendio ausentes")?;let image=&offices[loss.office];for k in 0..24{stock.stock[k]=i32::from_le_bytes(image[4+4*k..8+4*k].try_into().unwrap())}let merchant=merchants.get_mut(usize::from(loss.owner)).ok_or("propietario de almacén ausente")?;let d=p3_core::calendar::Calendar::for_tick(self.tick);p3_state::messages::add_warehouse_fire(&mut pool,merchant,d.day,d.month,d.year,loss.town,loss.payload)?;}
        if outcome.started{let town=&towns[usize::from(outcome.city)];let mut letter=[0u8;16];letter[4]=0x59;letter[5]=town.u8(0x2c0);letter[8..10].copy_from_slice(&u16::from(town.u8(0x2c0)).to_le_bytes());let d=p3_core::calendar::Calendar::for_tick(self.tick);p3_state::messages::add_global_inline_letter(&mut pool,d.day,d.month,d.year,&letter)?;let count=usize::from(self.world.merchant_count);if self.merchants.len()<count||self.merchants[..count].iter().any(|m|m.bytes.len()<10){return Err("comerciantes de incendio incompletos".into())}let humans=self.merchants[..count].iter().enumerate().filter(|(_,m)|m.u16(8)==0).map(|(i,_)|i).collect::<Vec<_>>();if !humans.is_empty(){let local=self.merchants.get(self.world.local_merchant as usize).ok_or("jugador local de incendio ausente")?;if local.bytes.len()<0x1a{return Err("comerciante local truncado".into())}if local.bytes[0x19]==outcome.city{for i in humans{notices.push((i as u16,outcome.city));if i as u32==self.world.local_merchant{windows.push((i as u16,outcome.city))}}}}}
        Ok(FirePrepared{scheduler,towns,sites,pool,keep:outcome.keep,city:usize::from(outcome.city),notices,windows,offices,updates,businesses,stocks,merchants})
    }
    fn plague_preview(&self,s:&p3_core::scheduler::Scheduler)->Result<PlaguePrepared,String>{
        let mut task=*s.records.get(usize::from(s.head)).ok_or("tarea ausente")?;let images=self.scheduled_office_images.as_ref().ok_or("imágenes de oficinas ausentes")?;if images.len()!=self.world.offices.len()||images.iter().any(|v|v.len()!=0x44c){return Err("oficinas truncadas".into())}let mut businesses=self.employment_businesses.clone();if businesses.len()!=self.world.merchant_facilities.len(){return Err("negocios incompletos".into())}for(b,f)in businesses.iter_mut().zip(&self.world.merchant_facilities){b[4..6].copy_from_slice(&f.employees.to_le_bytes());b[8..10].copy_from_slice(&f.next.to_le_bytes());}
        let factory=self.expedition_factory.as_ref().ok_or("pool de capitanes ausente")?;let entries=factory.captains.iter().flatten().copied().collect::<Vec<_>>();let capacity=factory.captains.len().try_into().map_err(|_|"pool de capitanes demasiado grande")?;if let Some(nav)=self.navigation.as_ref(){if nav.snapshot.manager.len()<0xe2||nav.snapshot.captains!=entries||u16_at(&nav.snapshot.manager,0xe0)!=factory.free_captain{return Err("vistas de capitanes desincronizadas".into())}}
        let mut captains=p3_core::ships::captains::CaptainPool{entries,capacity,free_head:factory.free_captain};let mut towns=self.world.towns.clone();let mut offices=images.concat();let outcome=p3_core::economy::plague::step_with_captains(&mut task,&mut towns,&mut offices,&mut businesses,&mut captains,self.tick).map_err(str::to_owned)?;let offices=offices.chunks_exact(0x44c).map(|v|v.to_vec()).collect::<Vec<_>>();let mut updates=Vec::new();for (i,(before,after))in images.iter().zip(&offices).enumerate(){if before[0x2f2..0x2f4]!=after[0x2f2..0x2f4]||before[0x2d6..0x2d8]!=after[0x2d6..0x2d8]{let admin=u16_at(after,0x2f2);let person=captains.entries.get(usize::from(admin)*16..usize::from(admin)*16+16);updates.push(p3_core::economy::world::ScheduledOfficeUpdate{index:i,administrator:admin,flags:u16_at(after,0x2d6),experience:person.map(|p|p[0xa]),wage:person.map(|p|u16_at(p,0xc)),image:None});}}
        let mut pirates=self.pirates.clone();if let Some(p)=pirates.as_mut(){if p.captains.len()!=factory.captains.len(){return Err("vistas piratas de capitanes truncadas".into())}for (i,old)in factory.captains.iter().enumerate(){let new=&captains.entries[i*16..i*16+16];for j in 0..16{if new[j]!=old[j]{if p.captains[i][j]!=old[j]&&p.captains[i][j]!=new[j]{return Err("capitán pirata desincronizado".into())}p.captains[i][j]=new[j]}}}}
        let mut pool=self.pool.clone();let mut notices=Vec::new();let mut windows=Vec::new();
        if outcome.started{let town=&towns[usize::from(outcome.city)];let mut letter=[0u8;16];letter[4]=0x58;letter[5]=town.u8(0x2c0);letter[8..10].copy_from_slice(&u16::from(town.u8(0x2c0)).to_le_bytes());let d=p3_core::calendar::Calendar::for_tick(self.tick);p3_state::messages::add_global_inline_letter(&mut pool,d.day,d.month,d.year,&letter)?;let count=usize::from(self.world.merchant_count);if self.merchants.len()<count||self.merchants[..count].iter().any(|m|m.bytes.len()<10){return Err("comerciantes de peste incompletos".into())}let humans=self.merchants[..count].iter().enumerate().filter(|(_,m)|m.u16(8)==0).map(|(i,_)|i).collect::<Vec<_>>();if !humans.is_empty(){let local=self.merchants.get(self.world.local_merchant as usize).ok_or("jugador local de peste ausente")?;if local.bytes.len()<0x1a{return Err("comerciante local truncado".into())}if local.bytes[0x19]==outcome.city{for i in humans{notices.push((i as u16,outcome.city));if i as u32==self.world.local_merchant{windows.push((i as u16,outcome.city))}}}}}
        Ok(PlaguePrepared{task,towns,businesses,pool,keep:outcome.keep,notices,windows,offices,updates,captains,pirates})
    }
    fn celebration_preview(&self,s:&p3_core::scheduler::Scheduler)->Result<CelebrationPrepared,String>{
        let task=s.records.get(usize::from(s.head)).ok_or("tarea ausente")?;let images=self.scheduled_office_images.as_ref().ok_or("imágenes de oficinas ausentes")?;if images.len()!=self.world.offices.len()||images.iter().any(|o|o.len()!=0x44c){return Err("oficinas truncadas".into())}let raw=images.concat();
        let Some(office)=p3_core::economy::celebration::find_office(task,&self.world.towns,&raw,self.merchants).map_err(str::to_owned)? else{return Ok(CelebrationPrepared::None)};
        let owner=task.u32_at(8) as usize;let city=task.u32_at(0xc) as usize;let mut town=self.world.towns.get(city).ok_or("ciudad de fiesta ausente")?.clone();let mut merchant=self.merchants.get(owner).ok_or("comerciante de fiesta ausente")?.clone();if merchant.bytes.len()<10{return Err("comerciante truncado".into())}
        if merchant.u16(8)!=0{town.set_u32(0x2d0,self.tick);return Ok(CelebrationPrepared::Ai{city,town})}
        let mut stock=self.stocks.get(office).ok_or("existencias de oficina ausentes")?.clone();if usize::from(stock.owner)!=owner||usize::from(stock.town)!=city{return Err("oficina de fiesta desincronizada".into())}
        let outcome=p3_core::economy::celebration::finish_human(task,&mut town,&mut stock,&mut merchant,self.tick).map_err(str::to_owned)?;let mut pool=self.pool.clone();let date=p3_core::calendar::Calendar::for_tick(self.tick);p3_state::messages::add_celebration(&mut pool,&mut merchant,date.day,date.month,date.year,city as u8,&outcome,self.building_enabled.get(owner).copied().unwrap_or(true))?;
        let window=if !outcome.notify{false}else if owner as u32==self.world.local_merchant{true}else{self.expedition_resources.as_ref().ok_or("modo de presentación de fiesta ausente")?.hotseat};
        Ok(CelebrationPrepared::Human{city,town,owner,merchant,office,stock,pool,notify:outcome.notify,window})
    }
    fn ten_day_preview(&self,s:&p3_core::scheduler::Scheduler)->Result<TenDayPrepared,String>{
        let nav=&self.navigation.as_ref().ok_or("flota viva no reconstruida")?.snapshot;let mut factory=self.expedition_factory.clone().ok_or("pool de capitanes ausente")?;
        if self.merchants.len()!=usize::from(self.world.merchant_count)||factory.captains.iter().flatten().copied().collect::<Vec<_>>()!=nav.captains{return Err("vistas decenales desincronizadas".into())}
        let offices=self.scheduled_office_images.as_ref().ok_or("imágenes de oficinas decenales ausentes")?;if offices.len()!=self.world.offices.len()||offices.iter().any(|o|o.len()!=0x44c){return Err("oficinas decenales truncadas".into())}let office_bytes=offices.concat();
        let input=ten_day_runtime::TenDayInputs{snapshot:nav,towns:&self.world.towns,sites:self.sites,merchants:self.merchants,cargo:self.fleet,offices:&office_bytes,band_heads:self.pirates.as_ref().map(|p|p.heads),distance_data:self.navigation_ai.as_ref(),locations:self.mission_locations.as_ref(),message_pool:self.pool,pending_operations:self.expedition_operations.len(),tick:self.tick,day_of_year:self.world.day_of_year,candidate_names:self.candidate_name_lengths.as_deref(),candidate_looks:*self.candidate_look_counts,network_client:self.world.network_client!=0,world_8a:self.world.world_8a,alderman:self.world.alderman_merchant,operations_mode:self.operations_mode,rng:*self.investigation_rng,celebration:*self.celebration_state,player:self.world.local_merchant};
        let state=ten_day_runtime::prepare_ten_day(s,&input)?;
        for image in offices {let cap=usize::from(u16_at(image,0x2f2));if cap<factory.captains.len(){let at=cap*16;if nav.captains[at+0xa]!=state.captains[at+0xa]||nav.captains[at+0xc..at+0xe]!=state.captains[at+0xc..at+0xe]{return Err("capitán decenal compartido con administrador".into())}}}
        let mut pirates=self.pirates.clone();if let Some(p)=pirates.as_mut(){if p.captains.len()!=factory.captains.len(){return Err("vistas piratas de capitanes truncadas".into())}}
        for (i,cap) in factory.captains.iter_mut().enumerate(){for j in 0..16{let at=i*16+j;let old=nav.captains[at];let new=state.captains[at];if old!=new{if let Some(p)=pirates.as_mut(){if p.captains[i][j]!=old&&p.captains[i][j]!=new{return Err(format!("capitán pirata {i} desincronizado"))}p.captains[i][j]=new}cap[j]=new;}}}
        Ok(TenDayPrepared{state,factory,pirates})
    }
    fn retirement_preview(&self,s:&p3_core::scheduler::Scheduler)->Result<RetirementPrepared,String>{
        let mut task=*s.records.get(usize::from(s.head)).ok_or("tarea ausente")?;
        let nav=self.navigation.as_ref().ok_or("flota viva no reconstruida")?;let mut snapshot=nav.snapshot.clone();
        let before=snapshot.clone();let mut factory=self.expedition_factory.clone().ok_or("pool naval ausente")?;
        if snapshot.manager.len()<0xf8||snapshot.ships.len()%0x180!=0||snapshot.convoys.len()%0x3c!=0||snapshot.captains.len()%16!=0{return Err("flota truncada".into())}
        if factory.captains.iter().flatten().copied().collect::<Vec<_>>()!=snapshot.captains||usize::from(u16_at(&snapshot.manager,0xf2))*16!=snapshot.captains.len()||factory.free_captain!=u16_at(&snapshot.manager,0xe0)||factory.ships.len()!=snapshot.ships.len()/0x180{return Err("vistas navales desincronizadas".into())}
        if usize::from(self.world.merchant_count)>self.merchants.len()||usize::from(self.world.merchant_count)>self.world.merchants.len()||self.merchants[..usize::from(self.world.merchant_count)].iter().any(|m|m.bytes.len()!=0x650){return Err("comerciantes truncados".into())}
        let mut pool=p3_core::ships::captains::CaptainPool{entries:snapshot.captains.clone(),capacity:u16_at(&snapshot.manager,0xf2),free_head:u16_at(&snapshot.manager,0xe0)};
        let mut raw_merchants=self.merchants[..usize::from(self.world.merchant_count)].iter().flat_map(|m|m.bytes.iter().copied()).collect::<Vec<_>>();
        for (i,m) in self.world.merchants.iter().take(usize::from(self.world.merchant_count)).enumerate(){let at=i*0x650;raw_merchants[at..at+4].copy_from_slice(&m.gold.to_le_bytes());raw_merchants[at+0x4c8..at+0x4cc].copy_from_slice(&m.income.to_le_bytes())}
        let mut disband=RetirementDisband{manager:snapshot.manager.clone(),merchants:raw_merchants,merchant_count:self.world.merchant_count,tick:self.tick,player:nav.player,distances:self.navigation_ai.clone(),notices:Vec::new()};
        let mut towns=self.world.towns.clone();let keep=p3_core::economy::captain_operations::retirement_tick(&mut task,&mut snapshot.ships,&mut snapshot.convoys,&mut towns,&mut pool,&self.merchants[..usize::from(self.world.merchant_count)],&mut disband)?;
        snapshot.manager=disband.manager;
        let mut merchants=self.merchants.to_vec();for (merchant,src) in merchants.iter_mut().zip(disband.merchants.chunks_exact(0x650)){merchant.bytes.copy_from_slice(src)}
        let mut message_pool=self.pool.clone();let date=Calendar::for_tick(self.tick);let retention=[u32::MAX,0x2800,0x1400,0xa00,0x500,0];for (owner,town,kind,words) in disband.notices{let merchant=merchants.get_mut(usize::from(owner)).ok_or("destinatario de convoy ausente")?;p3_state::messages::add_convoy_disband_notice(&mut message_pool,merchant,date.day,date.month,date.year,town,kind,words,&retention)?;}
        snapshot.captains=pool.entries;snapshot.manager[0xe0..0xe2].copy_from_slice(&pool.free_head.to_le_bytes());factory.free_captain=pool.free_head;
        navigation::refresh_lists(&mut snapshot)?;
        let mut convoy_images=self.expedition_convoys.to_vec();
        if convoy_images.len()!=snapshot.convoys.len()/0x3c||factory.convoys.len()!=convoy_images.len()||self.convoy_flags.len()!=convoy_images.len(){return Err("vistas de convoy truncadas".into())}
        for i in 0..convoy_images.len(){if convoy_images[i].len()!=0x3c||factory.convoys[i].len()!=0x3c{return Err("convoy truncado".into())}for j in 0..0x3c{let at=i*0x3c+j;let old=before.convoys[at];let new=snapshot.convoys[at];if old!=new{for image in [&mut convoy_images[i],&mut factory.convoys[i]]{if image[j]!=old&&image[j]!=new{return Err(format!("convoy {i} desincronizado en +{j:#x}"))}image[j]=new}}}}
        for (offset,field) in [(0xe4,&mut factory.free_convoy),(0xec,&mut factory.convoy_head)]{let old=u16_at(&before.manager,offset);let new=u16_at(&snapshot.manager,offset);if old!=new{if *field!=old&&*field!=new{return Err("cabeza de convoy desincronizada".into())}*field=new}}
        for (offset,field) in [(0xe8,*self.expedition_docked),(0xec,*self.expedition_head)]{let old=u16_at(&before.manager,offset);let new=u16_at(&snapshot.manager,offset);if old!=new&&field!=old&&field!=new{return Err("cabeza naval desincronizada".into())}}
        let mut images=self.expedition_images.to_vec();let mut pirates=self.pirates.clone();
        if let Some(p)=pirates.as_mut(){if p.captains.len()!=factory.captains.len()||p.ship_captains.len()!=factory.ships.len(){return Err("vistas piratas truncadas".into())}}
        for i in 0..factory.captains.len(){for j in 0..16{let at=i*16+j;let old=before.captains[at];let new=snapshot.captains[at];if old!=new{if let Some(p)=pirates.as_mut(){if p.captains[i][j]!=old&&p.captains[i][j]!=new{return Err(format!("capitán pirata {i} desincronizado"))}p.captains[i][j]=new;}factory.captains[i][j]=new;}}}
        for i in 0..factory.ships.len(){let at=i*0x180;let old=&before.ships[at..at+0x180];let new=&snapshot.ships[at..at+0x180];if old==new{continue}if self.world.ships.get(i).is_none()||self.ship_metadata.get(i).is_none(){return Err("vistas de barco truncadas".into())}let image=images.get_mut(i).and_then(Option::as_mut).ok_or("imagen de barco ausente")?;if image.len()!=0x180{return Err("imagen de barco truncada".into())}for j in 0..0x180{if old[j]!=new[j]{if image[j]!=old[j]&&image[j]!=new[j]{return Err(format!("barco {i} desincronizado en +{j:#x}"))}image[j]=new[j];}}if let Some(p)=pirates.as_mut(){let a=u16_at(old,0x42);let b=u16_at(new,0x42);if a!=b{if p.ship_captains[i]!=a&&p.ship_captains[i]!=b{return Err("capitán pirata de barco desincronizado".into())}p.ship_captains[i]=b;}}}
        Ok(RetirementPrepared{task,keep,snapshot,factory,towns,images,convoy_images,merchants,message_pool,pirates})
    }
    fn offer_preview(&self,s:&p3_core::scheduler::Scheduler)->Result<RetirementPrepared,String>{
        let mut task=*s.records.get(usize::from(s.head)).ok_or("tarea ausente")?;
        let nav=self.navigation.as_ref().ok_or("flota viva no reconstruida")?;let mut snapshot=nav.snapshot.clone();
        let before=snapshot.clone();let mut factory=self.expedition_factory.clone().ok_or("pool naval ausente")?;
        if snapshot.manager.len()<0xf8||snapshot.ships.len()%0x180!=0||snapshot.convoys.len()%0x3c!=0||snapshot.captains.len()%16!=0{return Err("flota truncada".into())}
        if factory.captains.iter().flatten().copied().collect::<Vec<_>>()!=snapshot.captains||usize::from(u16_at(&snapshot.manager,0xf2))*16!=snapshot.captains.len()||factory.free_captain!=u16_at(&snapshot.manager,0xe0)||factory.ships.len()!=snapshot.ships.len()/0x180{return Err("vistas navales desincronizadas".into())}
        if usize::from(self.world.merchant_count)>self.merchants.len()||usize::from(self.world.merchant_count)>self.world.merchants.len()||self.merchants[..usize::from(self.world.merchant_count)].iter().any(|m|m.bytes.len()!=0x650){return Err("comerciantes truncados".into())}
        let mut pool=p3_core::ships::captains::CaptainPool{entries:snapshot.captains.clone(),capacity:u16_at(&snapshot.manager,0xf2),free_head:u16_at(&snapshot.manager,0xe0)};
        let mut raw_merchants=self.merchants[..usize::from(self.world.merchant_count)].iter().flat_map(|m|m.bytes.iter().copied()).collect::<Vec<_>>();
        for (i,m) in self.world.merchants.iter().take(usize::from(self.world.merchant_count)).enumerate(){let at=i*0x650;raw_merchants[at..at+4].copy_from_slice(&m.gold.to_le_bytes());raw_merchants[at+0x4c8..at+0x4cc].copy_from_slice(&m.income.to_le_bytes())}
        let mut disband=RetirementDisband{manager:snapshot.manager.clone(),merchants:raw_merchants,merchant_count:self.world.merchant_count,tick:self.tick,player:nav.player,distances:self.navigation_ai.clone(),notices:Vec::new()};
        let mut people=self.merchants.to_vec();for (person,bytes) in people.iter_mut().zip(disband.merchants.chunks_exact(0x650)){person.bytes.copy_from_slice(bytes)}
        let offer=p3_core::economy::captain_offer::prepare(task,&snapshot.ships,&snapshot.convoys,&self.world.towns,&pool,&people,self.world.merchant_count,&disband)?;
        task=offer.task;let keep=offer.keep;snapshot.ships=offer.ships;snapshot.convoys=offer.convoys;let towns=offer.towns;pool=offer.captains;disband=offer.host;snapshot.manager=disband.manager;
        let mut merchants=offer.merchants;let mut message_pool=self.pool.clone();let date=Calendar::for_tick(self.tick);let retention=[u32::MAX,0x2800,0x1400,0xa00,0x500,0];
        for (owner,town,kind,words) in disband.notices{let merchant=merchants.get_mut(usize::from(owner)).ok_or("destinatario de convoy ausente")?;p3_state::messages::add_convoy_disband_notice(&mut message_pool,merchant,date.day,date.month,date.year,town,kind,words,&retention)?;}
        for notice in offer.notices{let merchant=merchants.get_mut(notice.recipient as usize).ok_or("destinatario de oferta ausente")?;p3_state::messages::post_to_merchant(&mut message_pool,merchant,date.day,date.month,date.year,&notice.record,p3_state::messages::Payload::None,&retention)?;}
        snapshot.captains=pool.entries;snapshot.manager[0xe0..0xe2].copy_from_slice(&pool.free_head.to_le_bytes());factory.free_captain=pool.free_head;
        navigation::refresh_lists(&mut snapshot)?;
        let mut convoy_images=self.expedition_convoys.to_vec();
        if convoy_images.len()!=snapshot.convoys.len()/0x3c||factory.convoys.len()!=convoy_images.len()||self.convoy_flags.len()!=convoy_images.len(){return Err("vistas de convoy truncadas".into())}
        for i in 0..convoy_images.len(){if convoy_images[i].len()!=0x3c||factory.convoys[i].len()!=0x3c{return Err("convoy truncado".into())}for j in 0..0x3c{let at=i*0x3c+j;let old=before.convoys[at];let new=snapshot.convoys[at];if old!=new{for image in [&mut convoy_images[i],&mut factory.convoys[i]]{if image[j]!=old&&image[j]!=new{return Err(format!("convoy {i} desincronizado en +{j:#x}"))}image[j]=new}}}}
        for (offset,field) in [(0xe4,&mut factory.free_convoy),(0xec,&mut factory.convoy_head)]{let old=u16_at(&before.manager,offset);let new=u16_at(&snapshot.manager,offset);if old!=new{if *field!=old&&*field!=new{return Err("cabeza de convoy desincronizada".into())}*field=new}}
        for (offset,field) in [(0xe8,*self.expedition_docked),(0xec,*self.expedition_head)]{let old=u16_at(&before.manager,offset);let new=u16_at(&snapshot.manager,offset);if old!=new&&field!=old&&field!=new{return Err("cabeza naval desincronizada".into())}}
        let mut images=self.expedition_images.to_vec();let mut pirates=self.pirates.clone();
        if let Some(p)=pirates.as_mut(){if p.captains.len()!=factory.captains.len()||p.ship_captains.len()!=factory.ships.len(){return Err("vistas piratas truncadas".into())}}
        for i in 0..factory.captains.len(){for j in 0..16{let at=i*16+j;let old=before.captains[at];let new=snapshot.captains[at];if old!=new{if let Some(p)=pirates.as_mut(){if p.captains[i][j]!=old&&p.captains[i][j]!=new{return Err(format!("capitán pirata {i} desincronizado"))}p.captains[i][j]=new;}factory.captains[i][j]=new;}}}
        for i in 0..factory.ships.len(){let at=i*0x180;let old=&before.ships[at..at+0x180];let new=&snapshot.ships[at..at+0x180];if old==new{continue}if self.world.ships.get(i).is_none()||self.ship_metadata.get(i).is_none(){return Err("vistas de barco truncadas".into())}let image=images.get_mut(i).and_then(Option::as_mut).ok_or("imagen de barco ausente")?;if image.len()!=0x180{return Err("imagen de barco truncada".into())}for j in 0..0x180{if old[j]!=new[j]{if image[j]!=old[j]&&image[j]!=new[j]{return Err(format!("barco {i} desincronizado en +{j:#x}"))}image[j]=new[j];}}if let Some(p)=pirates.as_mut(){let a=u16_at(old,0x42);let b=u16_at(new,0x42);if a!=b{if p.ship_captains[i]!=a&&p.ship_captains[i]!=b{return Err("capitán pirata de barco desincronizado".into())}p.ship_captains[i]=b;}}}
        Ok(RetirementPrepared{task,keep,snapshot,factory,towns,images,convoy_images,merchants,message_pool,pirates})
    }
    fn mission_pirate_abort_preview(&self,s:&p3_core::scheduler::Scheduler)->Result<Option<(usize,RepMerchant,Option<[u32;6]>,bool)>,String>{
        use p3_core::ai::settlement::{pirate_mission_terminal,PirateTerminal};let Some(t)=s.records.get(usize::from(s.head))else{return Err("tarea de misión ausente".into())};if t.kind!=0x32||t.u8_at(0xc)!=3||t.u8_at(0xf)==0||t.u8_at(0x12)<3||t.due>=t.u32_at(8)||u16::from(t.u8_at(0xd))>=self.world.merchant_count{return Ok(None)}let owner=usize::from(t.u8_at(0xd));let mut task=*t;let mut merchant=self.merchants.get(owner).ok_or("comerciante ausente")?.clone();let player=self.expedition_resources.as_ref().map_or(self.world.local_merchant,|v|u32::from(v.player));match pirate_mission_terminal(&mut task,&mut merchant,&[None;5],self.world.alderman_town,player).map_err(str::to_string)?{Some(PirateTerminal::Abort{notification,dialog_requested})=>Ok(Some((owner,merchant,notification,dialog_requested))),_=>Ok(None)}
    }
    fn mission_road_build_preview(&self,s:&p3_core::scheduler::Scheduler)->Result<Option<(p3_core::scheduler::TaskRecord,Vec<Town>,Vec<Site>,[usize;2])>,String>{
        let Some(t)=s.records.get(usize::from(s.head))else{return Err("tarea de misión ausente".into())};if t.kind!=0x32||t.u8_at(0xc)!=1||t.u8_at(0xf)==0||t.u8_at(0x12)!=1||t.due>=t.u32_at(8)||u16::from(t.u8_at(0xd))>=self.world.merchant_count{return Ok(None)}let costs=[3u32,4,1,2];if !(0..4).all(|i|u32::from(t.u8_at(0x14+i))+(u32::from((t.u8_at(0x13)>>(2*i))&3)<<8)>=costs[i]){return Ok(None)}let cities=[usize::from(t.u8_at(0x10)),usize::from(t.u8_at(0x11))];if cities.iter().any(|&i|i>=self.world.towns.len()){return Ok(None)}let mut task=*t;let mut towns=self.world.towns.clone();let mut sites=self.sites.to_vec();p3_core::ai::settlement::finish_road_materials_stable(&mut task,&mut towns,&mut sites,self.world.merchant_count).map_err(str::to_string)?;Ok(Some((task,towns,sites,cities)))
    }
    fn mission_human_failure_preview(&self,s:&p3_core::scheduler::Scheduler)->Result<Option<(p3_core::ai::settlement::AldermanHumanFailure,Vec<u8>,Option<(usize,RepMerchant)>)>,String>{
        let Some(task)=s.records.get(usize::from(s.head))else{return Err("tarea de misión ausente".into())};let player=self.expedition_resources.as_ref().map_or(self.world.local_merchant,|c|u32::from(c.player));let Some(mut effect)=p3_core::ai::settlement::fail_alderman_mission_human(task,self.merchants,self.world.merchant_count,&self.world.towns,self.world.alderman_merchant,self.world.alderman_town,player,self.tick).map_err(str::to_string)? else{return Ok(None)};
        let mut scheduler=s.clone();p3_core::ai::settlement::apply_human_failure_task(&mut scheduler,&mut effect).map_err(str::to_string)?;let d=p3_core::calendar::Calendar::for_tick(self.tick);let mut pool=self.pool.clone();let mut other=if effect.recipient!=effect.owner{Some((effect.recipient,self.merchants.get(effect.recipient).ok_or("destinatario ausente")?.clone()))}else{None};let recipient=match other.as_mut(){Some((_,merchant))=>merchant,None=>&mut effect.merchant};p3_state::messages::add_merchant_daily_record(&mut pool,recipient,d.day,d.month,d.year,&effect.letter)?;Ok(Some((effect,pool,other)))
    }
    fn mission_ai_failure_preview(&self,s:&p3_core::scheduler::Scheduler)->Result<Option<(p3_core::scheduler::TaskRecord,usize,RepMerchant)>,String>{
        let Some(t)=s.records.get(usize::from(s.head))else{return Err("tarea de misión ausente".into())};if t.kind!=0x32||u16::from(t.u8_at(0xd))>=self.world.merchant_count||t.u8_at(0xf)==0||t.due<t.u32_at(8){return Ok(None)}
        let owner=usize::from(t.u8_at(0xd));let mut merchant=self.merchants.get(owner).ok_or("comerciante ausente")?.clone();let mut task=*t;if !p3_core::ai::settlement::fail_alderman_mission_ai(&mut task,&mut merchant,&self.world.towns,self.tick).map_err(str::to_string)?{return Ok(None)}Ok(Some((task,owner,merchant)))
    }
    fn mission_pirate_start_preview(&self,s:&p3_core::scheduler::Scheduler)->Result<Option<(p3_core::scheduler::TaskRecord,usize,RepMerchant,Vec<u8>)>,String>{
        let Some(t)=s.records.get(usize::from(s.head))else{return Err("tarea de misión ausente".into())};if t.kind!=0x32||t.u8_at(0xc)!=3||t.u8_at(0xf)!=0||u16::from(t.u8_at(0xd))>=self.world.merchant_count{return Ok(None)}
        let locations=self.mission_locations.as_ref().ok_or("faltan posiciones de StadtDaten.ini")?;let mut task=*t;let (owner,letters)=p3_core::ai::settlement::start_alderman_pirate_mission(&mut task,self.merchants,self.world.merchant_count,self.world.alderman_town,self.tick,locations).map_err(str::to_string)?;
        let mut merchant=self.merchants.get(owner).ok_or("comerciante ausente")?.clone();let mut pool=self.pool.clone();let d=p3_core::calendar::Calendar::for_tick(self.tick);for letter in &letters{p3_state::messages::add_merchant_daily_record(&mut pool,&mut merchant,d.day,d.month,d.year,letter)?;}Ok(Some((task,owner,merchant,pool)))
    }
    fn mission_road_start_preview(&self,s:&p3_core::scheduler::Scheduler)->Result<Option<(p3_core::scheduler::TaskRecord,Vec<Town>,usize,RepMerchant,Vec<u8>,[usize;2])>,String>{
        let Some(t)=s.records.get(usize::from(s.head))else{return Err("tarea de misión ausente".into())};if t.kind!=0x32||t.u8_at(0xc)!=1||t.u8_at(0xf)!=0||u16::from(t.u8_at(0xd))>=self.world.merchant_count{return Ok(None)}
        let cities=[usize::from(t.u8_at(0x10)),usize::from(t.u8_at(0x11))];let mut task=*t;let mut towns=self.world.towns.clone();let (owner,letter)=p3_core::ai::settlement::start_alderman_road_mission(&mut task,&mut towns,self.mission_town_ids,self.merchants,self.world.merchant_count,self.world.alderman_merchant,self.world.alderman_town,self.tick).map_err(str::to_string)?;
        let mut merchant=self.merchants.get(owner).ok_or("comerciante ausente")?.clone();let mut pool=self.pool.clone();let d=p3_core::calendar::Calendar::for_tick(self.tick);p3_state::messages::add_merchant_daily_record(&mut pool,&mut merchant,d.day,d.month,d.year,&letter)?;Ok(Some((task,towns,owner,merchant,pool,cities)))
    }
    fn mission_completion_preview(&self,s:&p3_core::scheduler::Scheduler)->Result<Option<(p3_core::ai::settlement::AldermanCompletion,Vec<u8>)>,String>{
        let Some(t)=s.records.get(usize::from(s.head))else{return Err("tarea de misión ausente".into())};if t.kind!=0x32||u16::from(t.u8_at(0xd))>=self.world.merchant_count||t.u8_at(0xf)==0||t.due>=t.u32_at(8){return Ok(None)}
        let complete=match t.u8_at(0xc){3=>t.u8_at(0x12)==2,1 if t.u8_at(0x12)==0=>{let mut staged=*t;p3_core::ai::settlement::wait_road_installations(&mut staged,&self.world.towns,self.sites).map_err(str::to_string)?},0=>self.world.towns.get(usize::from(t.u8_at(0x16))).ok_or("ciudad de misión ausente")?.u32(0x76c)&0x100!=0,2=>t.u8_at(0x16)==1,4=>self.world.towns.get(usize::from(t.u8_at(0x12))).ok_or("ciudad de misión ausente")?.i32(0x2d4)>=i32::from(u16::from_le_bytes([t.u8_at(0x10),t.u8_at(0x11)])),_=>false};if !complete{return Ok(None)}
        let player=self.expedition_resources.as_ref().map_or(self.world.local_merchant,|c|u32::from(c.player));let mut effect=p3_core::ai::settlement::complete_alderman_mission(t,self.merchants,self.world.merchant_count,&self.world.towns,self.world.alderman_merchant,self.world.alderman_town,player).map_err(str::to_string)?;if t.u8_at(0xc)==3{effect.merchant.bytes[0xe1]=0xff}
        let d=p3_core::calendar::Calendar::for_tick(self.tick);let mut pool=self.pool.clone();p3_state::messages::add_global_inline_letter(&mut pool,d.day,d.month,d.year,&effect.letter)?;if effect.direct{p3_state::messages::add_merchant_daily_record(&mut pool,&mut effect.merchant,d.day,d.month,d.year,&effect.letter)?;}
        Ok(Some((effect,pool)))
    }
    fn tick_mission_preview(&self,s:&mut p3_core::scheduler::Scheduler)->Result<Option<bool>,String>{
        if let Some(t)=s.records.get(usize::from(s.head)){if t.kind==0x32&&t.u8_at(0xc)==3&&t.u8_at(0xf)!=0&&t.u8_at(0x12)<=1&&t.due<t.u32_at(8)&&u16::from(t.u8_at(0xd))<self.world.merchant_count{let targets=self.mission_pirate_targets.as_ref().ok_or("destinos piratas ausentes")?;let mut task=*t;let mut merchant=self.merchants.get(usize::from(t.u8_at(0xd))).ok_or("comerciante ausente")?.clone();if let Some(p3_core::ai::settlement::PirateTerminal::Wait)=p3_core::ai::settlement::pirate_mission_terminal(&mut task,&mut merchant,targets,self.world.alderman_town,u32::MAX).map_err(str::to_string)?{s.records[usize::from(s.head)]=task;return Ok(Some(true))}}}
        if let Some(task)=s.records.get(usize::from(s.head)){if task.kind==0x32&&task.u8_at(0xc)==1&&task.u8_at(0x12)==0&&usize::from(task.u8_at(0x10))<self.world.towns.len()&&usize::from(task.u8_at(0x11))<self.world.towns.len()&&task.u8_at(0xf)!=0&&u16::from(task.u8_at(0xd))<self.world.merchant_count&&task.due<task.u32_at(8){let mut staged=*task;let complete=p3_core::ai::settlement::wait_road_installations(&mut staged,&self.world.towns,self.sites).map_err(str::to_string)?;if complete{return Ok(None)}s.records[usize::from(s.head)]=staged;return Ok(Some(true))}}
        // Estas fases no leen barcos, capitanes ni destinos piratas. No exigir
        // vistas navales evita bloquear misiones urbanas en una sesión sin flota.
        if let Some(t)=s.records.get(usize::from(s.head)){let kind=t.u8_at(0xc);let assigned=u16::from(t.u8_at(0xd))<self.world.merchant_count;let passenger_without_ship=kind==2&&assigned&&(t.u8_at(0xf)==0||t.due>=t.u32_at(8)||t.u8_at(0x16)!=0);let pirate_without_ship=kind==3&&assigned&&(t.u8_at(0xf)==0||t.due>=t.u32_at(8)||t.u8_at(0x12)>=2);if t.kind==0x32&&(matches!(kind,1|4|5)||(kind==0&&assigned)||passenger_without_ship||pirate_without_ship){let targets=[None;5];let world=p3_core::ai::settlement::AldermanMissionWorld{notification_window_active:false,merchant_count:self.world.merchant_count,merchants:self.merchants,tick:self.tick,day_of_year:self.world.day_of_year,towns:&self.world.towns,town_ids:self.mission_town_ids,ships:&[],captains:&[],pirate_targets:&targets};return p3_core::ai::settlement::tick_alderman_mission_partial(s,&world).map_err(str::to_string)}}
        // La validez de las ofertas de fundación y piratas consulta destinos,
        // pero no el pool de barcos ni capitanes.
        if let Some(t)=s.records.get(usize::from(s.head)){if t.kind==0x32&&u16::from(t.u8_at(0xd))>=self.world.merchant_count&&matches!(t.u8_at(0xc),0|3){let targets=self.mission_pirate_targets.as_ref().ok_or("destinos piratas ausentes")?;let world=p3_core::ai::settlement::AldermanMissionWorld{notification_window_active:false,merchant_count:self.world.merchant_count,merchants:self.merchants,tick:self.tick,day_of_year:self.world.day_of_year,towns:&self.world.towns,town_ids:self.mission_town_ids,ships:&[],captains:&[],pirate_targets:targets};return p3_core::ai::settlement::tick_alderman_mission_partial(s,&world).map_err(str::to_string)}}
        let factory=self.expedition_factory.as_ref().ok_or("flota ausente")?;let no_targets=[None;5];let is_passenger=s.records.get(usize::from(s.head)).is_some_and(|t|t.kind==0x32&&t.u8_at(0xc)==2);let targets=if is_passenger{&no_targets}else{self.mission_pirate_targets.as_ref().ok_or("destinos piratas ausentes")?};
        if self.expedition_images.len()!=factory.ships.len(){return Err("imágenes de flota incompletas".into())}
        let ships=self.expedition_images.iter().zip(&factory.ships).map(|(live,fallback)|live.as_ref().unwrap_or(fallback).clone()).collect::<Vec<_>>();
        let world=p3_core::ai::settlement::AldermanMissionWorld{notification_window_active:false,merchant_count:self.world.merchant_count,merchants:self.merchants,tick:self.tick,day_of_year:self.world.day_of_year,towns:&self.world.towns,town_ids:self.mission_town_ids,ships:&ships,captains:&factory.captains,pirate_targets:targets};
        if let Some(t)=s.records.get(usize::from(s.head)){if t.kind==0x32&&t.u8_at(0xc)==3&&t.u8_at(0xf)!=0&&t.u8_at(0x12)<=1&&t.due<t.u32_at(8)&&u16::from(t.u8_at(0xd))<self.world.merchant_count{let band=usize::from(t.u8_at(0x10));if targets.get(band).copied().flatten()==Some(t.u8_at(0x11)){let locations=self.mission_locations.as_ref().ok_or("faltan posiciones de StadtDaten.ini")?;let merchant=self.merchants.get(usize::from(t.u8_at(0xd))).ok_or("comerciante ausente")?;if merchant.bytes.len()<0x10{return Err("comerciante truncado".into())}let mut task=*t;if let Some(keep)=p3_core::ai::settlement::track_pirate_mission(&mut task,targets,locations,&ships,merchant.u16(0xe)).map_err(str::to_string)?{s.records[usize::from(s.head)]=task;return Ok(Some(keep))}}}}
        p3_core::ai::settlement::tick_alderman_mission_partial(s,&world).map_err(str::to_string)
    }
    fn factory_preview(&self,r:&p3_core::scheduler::TaskRecord,task_id:u16)->Option<(p3_core::ships::factory::HanseFactory,Vec<RepMerchant>,p3_core::scheduler::TaskRecord,Vec<Town>,p3_core::economy::hanse_expedition::BeginEffects)> {
        if !self.expedition_coherent(){return None}let config=self.expedition_resources.as_ref()?;let city=usize::from(r.u8_at(0x14));let (x,y)=config.ports.get(city).copied().flatten()?;let mut factory=self.expedition_factory.clone()?;if factory.ships.len()!=self.fleet.len()||factory.convoys.len()!=self.expedition_convoys.len()||factory.convoys.len()!=self.convoy_flags.len(){return None}let mut merchants=self.merchants.to_vec();let mut towns=self.world.towns.clone();let resources=p3_core::ships::factory::HanseFactoryResources{ship_names:&config.ship_names,first_names:config.first_names,last_names:config.last_names,day_of_year:self.world.day_of_year,day:self.world.day_of_month,tick:self.tick,port_x:x,port_y:y};let id=p3_core::ships::factory::create_hanse_convoy(&mut factory,&towns,&mut merchants,city as u8,task_id,r.kind!=0x23,r.kind==0x34,&resources)?;let mut record=*r;let effects=p3_core::economy::hanse_expedition::begin(&mut record,&mut towns,&merchants,config.hotseat,config.player,task_id,id)?;Some((factory,merchants,record,towns,effects))
    }
    fn expedition_coherent(&self)->bool {
        if self.expedition_images.len()!=self.fleet.len() || self.fleet.len()!=self.ship_metadata.len() || self.world.ships.len()!=self.ship_metadata.len(){return false}
        for ((image,ship),&(next,state,_,convoy)) in self.expedition_images.iter().zip(self.fleet.iter()).zip(self.ship_metadata.iter()) {if let Some(img)=image {if img.len()<0x180 || img[0]!=ship.owner || img[0xe]!=ship.kind || u16_at(img,0x40)!=ship.crew || u16_at(img,4)!=next || u16_at(img,8)!=convoy || u16_at(img,0x134)!=state{return false}}}true
    }
    /// Fases 8 y 9 de la caza mientras no exista ninguna banda pirata: el convoy sale al puerto de la ciudad objetivo y espera.
    /// `None` si hay bandas (el seguimiento requiere el movimiento en estado 0x16, aún sin enganchar) o faltan recursos.
    fn hunt_idle_preview(&self,r:&p3_core::scheduler::TaskRecord)->Option<HuntStep> {
        use p3_core::economy::hanse_expedition::{hunt_search_start_no_targets,hunt_select_target,hunt_wait};
        let phase=r.u8_at(0x15);
        if r.kind!=0x34||!matches!(phase,8|9)||!self.expedition_coherent()||self.expedition_convoys.len()!=self.convoy_flags.len(){return None}
        if self.hunt_bands.as_ref()?.iter().any(Option::is_some){return None}
        let id=usize::from(u16::from_le_bytes([r.u8_at(0x16),r.u8_at(0x17)]));
        let mut convoy=self.expedition_convoys.get(id)?.clone();
        if convoy.len()<0x3c{return None}
        let mut ships=self.expedition_images.to_vec();
        let mut record=*r;
        if phase==8 {
            let (px,py)=self.expedition_resources.as_ref()?.ports.get(usize::from(r.u8_at(0x14))).copied().flatten()?;
            hunt_search_start_no_targets(&mut record,&mut convoy,&mut ships,self.world.ship_count,self.expedition_sea_head,(px,py),self.tick,id as u16)?;
        } else if hunt_wait(&mut record,self.tick,Some(&convoy)).is_none() {
            if ships.iter().flatten().any(|v|v.len()!=0x180){return None}
            let mut full:Vec<Vec<u8>>=ships.iter().map(|s|s.clone().unwrap_or_else(||vec![0;0x180])).collect();
            let leader=u16::from_le_bytes([convoy[0x10],convoy[0x11]]);
            let convoys=self.expedition_convoys.to_vec();
            hunt_select_target(&mut record,&mut full,&convoys,&[None;5],leader,self.tick,&[])?;
            ships=full.into_iter().zip(ships.iter()).map(|(v,old)|old.as_ref().map(|_|v)).collect();
        }
        Some(HuntStep{record,convoy,ships,id})
    }
    /// Aplica un paso de caza calculado sobre copias y avisa de lo que cambió.
    fn apply_hunt_step(&mut self,step:HuntStep) -> p3_core::scheduler::TaskRecord {
        let before=self.expedition_images.to_vec();
        for (i,image) in step.ships.into_iter().enumerate(){self.expedition_images[i]=image}
        for (i,(a,b)) in before.iter().zip(self.expedition_images.iter()).enumerate(){if a!=b{let image=b.as_ref().expect("validado");self.ship_metadata[i].1=u16_at(image,0x134);self.world.ships[i].state=u16_at(image,0x134);if !self.expedition_changed_ships.contains(&i){self.expedition_changed_ships.push(i)}}}
        self.expedition_convoys[step.id]=step.convoy;
        self.convoy_flags[step.id]=u32_at(&self.expedition_convoys[step.id],0x18);
        if !self.changed_convoys.contains(&step.id){self.changed_convoys.push(step.id)}
        if !self.expedition_changed_convoys.contains(&step.id){self.expedition_changed_convoys.push(step.id)}
        step.record
    }
    /// Fase 10 de la caza (regreso): `Ok(None)` si el convoy sigue en ruta (falta el planificador de caza) o faltan recursos.
    fn hunt_return_preview(&self,r:&p3_core::scheduler::TaskRecord,task_id:u16)->Result<Option<HuntPrepared>,String> {
        use p3_core::economy::hanse_expedition::{hunt_return,HuntReturn};
        if r.kind!=0x34||r.u8_at(0x15)!=10||!self.expedition_coherent()||self.expedition_convoys.len()!=self.convoy_flags.len(){return Ok(None)}
        let (Some(texts),Some(resources)) = (self.hunt_texts.as_ref(),self.expedition_resources.as_ref()) else{return Ok(None)};
        let id=usize::from(u16::from_le_bytes([r.u8_at(0x16),r.u8_at(0x17)]));
        let Some(mut convoy)=self.expedition_convoys.get(id).cloned() else{return Ok(None)};
        if convoy.len()<0x3c{return Ok(None)}
        let Some(Some((px,py)))=resources.ports.get(usize::from(r.u8_at(0x14))).copied() else{return Ok(None)};
        let leader=u16::from_le_bytes([convoy[0x10],convoy[0x11]]);
        let mut ships=self.expedition_images.to_vec();
        let Some(Some(mut lead))=ships.get(usize::from(leader)).cloned() else{return Ok(None)};
        let mut record=*r;
        let Some(result)=hunt_return(&mut record,&convoy,&mut lead,leader,(px as i16,py as i16)) else{return Ok(None)};
        ships[usize::from(leader)]=Some(lead);
        let mut pool=self.pool.to_vec();let mut merchants=self.merchants.to_vec();
        match result {
            HuntReturn::EnRoute(_)=>return Ok(None),
            HuntReturn::Finished=>{}
            HuntReturn::Arrived(_)=>{
                let (Some(first),Some(family))=(self.mission_first_names.as_ref(),self.mission_family_names.as_ref()) else{return Ok(None)};
                let date=p3_core::calendar::Calendar::for_tick(self.tick);
                let (names,ids)=(self.mission_town_names,self.mission_town_ids);
                let town_name=|k:u32|->Vec<u8>{ids.get(k as usize).and_then(|&i|names.as_ref()?.for_map(i)).map(|(n,_)|n.to_vec()).unwrap_or_default()};
                // El original informa con la lista de bajas (+8 del registro) y la libera.
                let losses=if record.u32_at(8)!=0{self.hunt_losses.get(&usize::from(task_id)).cloned().ok_or("lista de bajas de la caza ausente")?}else{Vec::new()};
                c08_hunt::arrive_hunt(&mut pool,&mut merchants,self.world.merchant_count,(date.day,date.month,date.year),texts,c08_text::NameBanks{first,family},&resources.ship_names,&town_name,self.world.alderman_town,self.world.towns.len() as u32,&mut record,&losses,&mut convoy,&mut ships,self.world.ship_count)?;
            }
        }
        Ok(Some(HuntPrepared{record,convoy,ships,pool,merchants,id}))
    }
    fn expedition_scores(&self,r:&p3_core::scheduler::TaskRecord)->Option<Vec<u16>> {
        if !self.expedition_coherent(){return None}
        let captains=self.pirates.as_ref()?.captains.len().try_into().ok()?;let convoys=self.convoy_flags.len().try_into().ok()?;
        p3_core::economy::hanse_expedition::fleet_scores(r,&self.world.towns,self.merchants,self.expedition_images,self.world.ship_count,convoys,captains)
    }
    fn expedition_ships(&self)->Option<Vec<p3_core::economy::hanse_expedition::ExpeditionShip>> {
        if self.fleet.len()!=self.ship_metadata.len(){return None}
        Some(self.fleet.iter().zip(self.ship_metadata.iter()).map(|(ship,&(next,state,_,_))|p3_core::economy::hanse_expedition::ExpeditionShip{next,town:ship.town,state}).collect())
    }
    fn petition_payload(&self,r:&mut p3_core::scheduler::TaskRecord,alderman:u8,human:bool)->Option<([u8;8],bool)> {
        if r.u8_at(0x13)==2 {Some((p3_core::economy::hanse_petition::pirate_payload(r,self.pirates.as_ref()?,human)?,false))}
        else {p3_core::economy::hanse_petition::first_payload(r,alderman,self.merchants)}
    }
    fn investigation_office(&self,r:&p3_core::scheduler::TaskRecord)->Option<Option<usize>> {
        let owner=usize::from(r.u8_at(8));let merchant=self.merchants.get(owner)?;let home=self.world.towns.get(usize::from(*merchant.bytes.get(0x19)?))?;if home.flags()&0x20000==0{return Some(None)}
        // Para delitos 5..7 el original busca oficina en 0xff; el 15 impone veto.
        // con hasta 255 ciudades el buscador devuelve nulo sin tocar el pool.
        if r.u8_at(17)!=1||c10_investigation::office_city(r.u8_at(0x10),r.u8_at(11)).is_none(){return (self.world.towns.len()<=255).then_some(None)}
        let city=usize::from(r.u8_at(11));let town=self.world.towns.get(city)?;if self.stocks.len()!=self.offices.len()||self.offices.len()!=self.world.offices.len(){return None}let mut cursor=town.u16(0x784);let mut seen=vec![false;self.offices.len()];let mut found=None;
        while usize::from(cursor)<self.offices.len(){let i=usize::from(cursor);if seen[i]{return None}seen[i]=true;if usize::from(self.offices[i].owner)==owner{if found.is_some()||usize::from(self.stocks[i].owner)!=owner||usize::from(self.stocks[i].town)!=city{return None}found=Some(i)}cursor=self.offices[i].next}
        Some(found)
    }
    fn debt_assets(&self,r:&p3_core::scheduler::TaskRecord)->Option<(Vec<Option<[i32;24]>>,Vec<p3_core::economy::debt::DebtShip>,Vec<Option<usize>>)> {
        if !p3_core::economy::debt::can_seize_assets(r,self.merchants) || self.stocks.len()!=self.world.offices.len() || self.offices.len()!=self.world.offices.len() || self.fleet.len()!=self.ship_metadata.len(){return None}
        let borrower=usize::from(r.u8_at(0x11));let m=self.merchants.get(borrower)?;let target=usize::from(r.u8_at(0x14));let home=usize::from(m.bytes[0x19]);
        if target>=self.world.towns.len() || home>=self.world.towns.len(){return None}
        let mut stocks=vec![None;self.world.towns.len()];let mut indices=vec![None;self.world.towns.len()];
        for city in [target,home] {
            if indices[city].is_some(){continue}
            let mut cursor=self.world.towns[city].u16(0x784);let mut seen=vec![false;self.offices.len()];
            while usize::from(cursor)<self.world.offices.len() {let i=usize::from(cursor);if seen[i]{return None}seen[i]=true;
                if usize::from(self.offices[i].owner)==borrower {if indices[city].is_some() || usize::from(self.stocks[i].owner)!=borrower || usize::from(self.stocks[i].town)!=city{return None}indices[city]=Some(i);stocks[city]=Some(self.stocks[i].stock);}
                cursor=self.world.offices[i].next;
            }
            if self.stocks.iter().enumerate().any(|(i,o)|usize::from(o.owner)==borrower && usize::from(o.town)==city && !seen[i]){return None}
        }
        let mut cursor=m.u16(0xe);let mut seen=vec![false;self.fleet.len()];
        while cursor<self.world.ship_count {let i=usize::from(cursor);if i>=seen.len() || seen[i] || usize::from(self.fleet[i].owner)!=borrower{return None}seen[i]=true;cursor=self.ship_metadata[i].0;}
        let ships=self.fleet.iter().zip(self.ship_metadata.iter()).map(|(cargo,&(next,state,name,_))|p3_core::economy::debt::DebtShip{cargo:cargo.clone(),next:if next<self.world.ship_count{next}else{u16::MAX},state,name}).collect();
        Some((stocks,ships,indices))
    }
    fn debt_ships(&self,r:&p3_core::scheduler::TaskRecord)->Option<Vec<p3_core::economy::debt::DebtShip>> {
        if !p3_core::economy::debt::can_settle_partial_cash(r,self.merchants) || self.stocks.len()!=self.world.offices.len() || self.offices.len()!=self.world.offices.len() || self.fleet.len()!=self.ship_metadata.len(){return None}
        let borrower=usize::from(r.u8_at(0x11));let city=usize::from(r.u8_at(0x14));let m=self.merchants.get(borrower)?;
        if city>=self.world.towns.len() || usize::from(m.bytes[0x19])!=city || self.offices.iter().any(|o|usize::from(o.owner)==borrower){return None}
        let mut cursor=m.u16(0xe);let mut seen=vec![false;self.fleet.len()];
        while cursor<self.world.ship_count {let i=usize::from(cursor);if i>=seen.len() || seen[i] || usize::from(self.fleet[i].owner)!=borrower{return None}seen[i]=true;cursor=self.ship_metadata[i].0;}
        if !seen.iter().any(|v|*v){return None}
        Some(self.fleet.iter().zip(self.ship_metadata.iter()).map(|(cargo,&(next,state,name,_))|p3_core::economy::debt::DebtShip{cargo:cargo.clone(),next,state,name}).collect())
    }
    fn debt_office(&self,r:&p3_core::scheduler::TaskRecord)->Option<usize> {
        if !p3_core::economy::debt::can_settle_partial_cash(r,self.merchants) || self.stocks.len()!=self.world.offices.len() || self.offices.len()!=self.world.offices.len(){return None}
        let borrower=usize::from(r.u8_at(0x11));let city=usize::from(r.u8_at(0x14));
        let m=self.merchants.get(borrower)?;let town=self.world.towns.get(city)?;
        if usize::from(m.bytes[0x19])!=city || m.u16(0xe)<self.world.ship_count{return None}
        let mut found=self.stocks.iter().enumerate().filter(|(_,o)|usize::from(o.owner)==borrower);
        let (index,office)=found.next()?;if found.next().is_some() || usize::from(office.town)!=city || usize::from(self.offices[index].owner)!=borrower{return None}
        let mut cursor=town.u16(0x784);let mut steps=0;
        while usize::from(cursor)<self.world.offices.len() && steps<self.world.offices.len(){if usize::from(cursor)==index{return Some(index)}cursor=self.world.offices[usize::from(cursor)].next;steps+=1;}
        None
    }
}
impl p3_core::economy::council_meeting::MeetingHost for CouncilExecution<'_> {
    fn notify_voter(&mut self,m:u16,t:u8){self.notifications.push((m,t));}
    fn free_space_percent(&mut self,t:&Town)->u32{self.sites[t.index() as usize].free_space_percent_inside_walls(t)}
    fn point_valid(&mut self,t:&Town,p:u32)->bool{p3_core::buildings::query::is_plan_point_valid(t,p)}
    fn place_wall(&mut self,t:&mut Town,p:u32){self.sites[t.index() as usize].place_town_defense_pieces(t,p,&[0,0x20,0x40,0x60,0x80,0xa0,0xc0,0xe0]);}
    fn decision_letter(&mut self,m:u8,t:u8,a:u32,b:u32){self.letters.push((0x6b,u16::from(m),t,a,b));}
}
impl p3_core::economy::extra_tax::ExtraTaxHost for CouncilExecution<'_> {
    fn tax_letter(&mut self,m:u16,t:u8,a:u32,b:u32){self.letters.push((0x6c,m,t,a,b));}
}
impl CouncilExecution<'_> {
    fn profit_share_preview(&self,s:&p3_core::scheduler::Scheduler)->Result<profit_share_stage::Prepared,String>{
        let count=usize::from(self.world.merchant_count);if self.merchants.len()<count||self.world.merchants.len()<count{return Err("comerciantes de participación ausentes".into())}
        let mut people=self.merchants[..count].to_vec();for (m,account) in people.iter_mut().zip(self.world.merchants.iter()){m.set_i32(0,account.gold);m.set_i32(0x4c8,account.income)}
        let stock=self.world.towns.first().ok_or("ciudad para participación ausente")?.u32(4);profit_share_stage::prepare(s,&people,self.pool,self.tick,stock)
    }
    fn trade_offer_preview(&self,s:&p3_core::scheduler::Scheduler)->Result<trade_offer_watch_stage::Prepared,String>{
        let task=s.records.get(usize::from(s.head)).ok_or("tarea de oferta ausente")?;
        let offers=self.trade_offers.as_ref().ok_or("ofertas S04 ausentes")?;
        let images=self.scheduled_office_images.as_ref().ok_or("imágenes de oficinas ausentes")?;
        let count=usize::from(self.world.merchant_count);if self.world.merchants.len()<count{return Err("vistas de comerciantes incompletas".into())}
        let mut merchants=self.merchants.get(..count).ok_or("comerciantes ausentes")?.to_vec();
        for (merchant,account) in merchants.iter_mut().zip(self.world.merchants.iter()).take(count){merchant.set_i32(0,account.gold);merchant.set_i32(0x4c8,account.income)}
        trade_offer_watch_stage::prepare(trade_offer_watch_stage::Inputs{task,nav:self.navigation.as_ref(),offers,pool:self.pool,merchants:&merchants,towns:&self.world.towns,office_images:images,stocks:self.stocks})
    }
    fn land_transport_preview(&self,s:&p3_core::scheduler::Scheduler)->Result<land_transport_stage::Prepared,String>{
        let task=s.records.get(usize::from(s.head)).ok_or("tarea terrestre ausente")?;
        let images=self.scheduled_office_images.as_ref().ok_or("imágenes de oficinas terrestres ausentes")?;
        if images.len()!=self.stocks.len()||images.iter().any(|v|v.len()<0x418){return Err("imágenes de oficinas terrestres truncadas".into())}
        let count=usize::from(self.world.merchant_count);if self.world.merchants.len()<count{return Err("vistas de comerciantes terrestres incompletas".into())}let mut merchants=self.merchants.get(..count).ok_or("comerciantes terrestres ausentes")?.to_vec();
        for (merchant,account) in merchants.iter_mut().zip(self.world.merchants.iter()).take(count){merchant.set_i32(0,account.gold);merchant.set_i32(0x4c8,account.income)}
        land_transport_stage::prepare(land_transport_stage::Inputs{task,towns:&self.world.towns,offices:&self.world.offices,stocks:self.stocks,merchants:&merchants,town_indices:self.mission_town_ids,message_pool:self.pool,calendar:p3_core::calendar::Calendar::for_tick(self.tick),scene_active:false})
    }
}
impl p3_core::scheduler::TaskHandlers for CouncilExecution<'_> {
    fn supports_task(&self,kind:u16)->bool{matches!(kind,0x00|0x01|0x02|0x03|0x04|0x05|0x06|0x07|0x08|0x09|0x0a|0x0b|0x0c|0x0d|0x0e|0x0f|0x10|0x11|0x12|0x13|0x14|0x15|0x16|0x17|0x18|0x19|0x1a|0x1b|0x1c|0x1d|0x1e|0x1f|0x20|0x21|0x22|0x23|0x24|0x25|0x26|0x27|0x28|0x29|0x2a|0x2b|0x2c|0x2d|0x2e|0x2f|0x30|0x31|0x32|0x33|0x34|0x35|0x36|0x37|0x38|0x39)}
    fn dispatch_blocked(&mut self,s:&p3_core::scheduler::Scheduler,kind:u16){
        if self.pending_office.is_none()&&self.pending_founding.is_none()&&self.pending_pirate_approach.is_none()&&self.pending_pirate_attack.is_none()&&self.errors.is_empty(){self.errors.push(format!("tarea programada {kind:#04x} en ranura {} no se pudo despachar",s.head));}
    }
    fn can_dispatch(&mut self,s:&p3_core::scheduler::Scheduler,kind:u16)->bool{
        self.mission_cached=None;
        // El despachador original elimina los tipos fuera de la tabla; para
        // los conocidos sin portar conserva la tarea y hace visible el límite.
        if usize::from(kind)>=p3_core::scheduler::KINDS.len(){return true}
        if !self.supports_task(kind){let error=format!("tarea programada {kind:#04x} no implementada");if self.errors.last()!=Some(&error){self.errors.push(error)}return false}
        if kind==0x18{return match self.profit_share_preview(s){Ok(_)=>true,Err(e)=>{let error=format!("tarea 0x18: {e}");if self.errors.last()!=Some(&error){self.errors.push(error)}false}}}
        if kind==0x13{return match self.trade_offer_preview(s){Ok(_)=>true,Err(e)=>{self.errors.push(format!("tarea 0x13: {e}"));false}}}
        if kind==0x0c{return match self.land_transport_preview(s){Ok(_)=>true,Err(e)=>{self.errors.push(format!("tarea 0x0c: {e}"));false}}}
        if kind==0x0e{return match self.game_end_preview(s){Ok(_)=>true,Err(e)=>{let error=format!("tarea 0x0E: {e}");if self.errors.last()!=Some(&error){self.errors.push(error)}false}}}
        if kind==0x0f{return match self.burglary_preview(s){Ok(_)=>true,Err(e)=>{let error=format!("tarea 0x0F: {e}");if self.errors.last()!=Some(&error){self.errors.push(error)}false}}}
        if kind==0x10{return match self.traveller_deadline_preview(s){Ok(_)=>true,Err(e)=>{let error=format!("tarea 0x10: {e}");if self.errors.last()!=Some(&error){self.errors.push(error)}false}}}
        if kind==0x12{let result=match self.expedition_return_start_preview(s){Ok(_)=>Ok(()),Err(e) if e.contains("fase de retorno")=>self.expedition_return_preview(s).map(|_|()),Err(e)=>Err(e)};return match result{Ok(())=>true,Err(e)=>{let error=format!("tarea 0x12: {e}");if self.errors.last()!=Some(&error){self.errors.push(error)}false}}}
        if kind==0x1f{return match self.town_event_plan_preview(s){Ok(_)=>true,Err(e)=>{let error=format!("tarea 0x1F: {e}");if self.errors.last()!=Some(&error){self.errors.push(error)}false}}}
        if kind==0x1e{return match self.town_event_preview(s){Ok(_)=>true,Err(e)=>{let error=format!("tarea 0x1E: {e}");if self.errors.last()!=Some(&error){self.errors.push(error)}false}}}
        if matches!(kind,0x0a|0x36){let phase=s.records.get(usize::from(s.head)).map(|r|r.u8_at(0x14));if phase==Some(1){return true}let result=if phase==Some(0)&&kind==0x0a{Some(self.auction_ship_preview(s).map(|_|()))}else if phase==Some(2){Some(self.auction_announce_preview(s).map(|_|()))}else if phase==Some(3){Some(self.auction_open_preview(s).map(|_|()))}else if phase.is_some_and(|p|(4..=6).contains(&p)){Some(self.auction_bids_preview(s).map(|_|()))}else if phase==Some(7){Some(self.auction_missing_lot_preview(s).map(|_|()))}else if phase==Some(8){Some(s.records.get(usize::from(s.head)).and_then(|r|self.world.towns.get(usize::from(r.u8_at(0x15)))).map(|_|()).ok_or_else(||"ciudad de subasta ausente".to_owned()))}else{None};if let Some(result)=result{return match result{Ok(())=>true,Err(e) if kind==0x36 && e=="comprador sin oficina"=>{let mut task=s.records[usize::from(s.head)];match p3_core::economy::auction::bought_building(&mut task,&self.world.towns,self.sites,self.world.merchant_count){Ok((lot,seller))=>{let marker=(s.head,s.records[usize::from(s.head)],self.scheduled_office_images.as_ref().map_or(0,Vec::len));if !self.pending_auction_office.as_ref().is_some_and(|(head,task,_)|*head==marker.0&&*task==marker.1){let mut merchants=self.merchants.to_vec();let mut pool=self.pool.clone();let site=&self.sites[usize::from(lot.city)];if let Err(error)=Self::auction_sale_letters(lot,seller,site,&mut merchants,&mut pool,self.tick){self.errors.push(format!("cartas de subasta: {error}"));return false}self.merchants.clone_from_slice(&merchants);*self.pool=pool;*self.pending_auction_office=Some(marker)}*self.pending_office=Some((lot.buyer,lot.city,self.tick));false},Err(error)=>{self.errors.push(format!("tarea de subasta 0x36: {error}"));false}}},Err(e)=>{let error=format!("tarea de subasta {kind:#04x}: {e}");if self.errors.last()!=Some(&error){self.errors.push(error)}false}}}let error=format!("tarea de subasta {kind:#04x}: fase {phase:?} no conectada");if self.errors.last()!=Some(&error){self.errors.push(error)}return false}
        if kind==0x38{return match self.mayor_statue_preview(s){Ok(_)=>true,Err(e)=>{let error=format!("tarea 0x38: {e}");if self.errors.last()!=Some(&error){self.errors.push(error)}false}}}
        if kind==0x02{return match self.mayor_election_preview(s){Ok(_)=>true,Err(e)=>{let error=format!("tarea 0x02: {e}");if self.errors.last()!=Some(&error){self.errors.push(error)}false}}}
        if kind==0x08{return match self.captain_refill_preview(s){Ok(_)=>true,Err(e)=>{let error=format!("tarea 0x08: {e}");if self.errors.last()!=Some(&error){self.errors.push(error)}false}}}
        if kind==0x29{return match self.captain_return_preview(s){Ok(_)=>true,Err(e)=>{let error=format!("tarea 0x29: {e}");if self.errors.last()!=Some(&error){self.errors.push(error)}false}}}
        if kind==0x2b{return match self.ai_ship_order_preview(s){Ok(_)=>true,Err(e)=>{let error=format!("tarea 0x2B: {e}");if self.errors.last()!=Some(&error){self.errors.push(error)}false}}}
        if kind==0x2a{let result=(||->Result<bool,String>{let original=*s.records.get(usize::from(s.head)).ok_or("tarea de expansión ausente")?;let(after,chosen)=if let Some((id,before,after,owner,city))=*self.pending_ai_expansion{if id!=s.head||before!=original{return Err("tarea de expansión modificada durante la creación".into())}(after,Some((u16::from(owner),u16::from(city))))}else{self.ai_expansion_preview(s)?};if let Some((owner,city))=chosen{let owner=u8::try_from(owner).map_err(|_|"comerciante de expansión fuera de rango")?;let city=u8::try_from(city).map_err(|_|"ciudad de expansión fuera de rango")?;if !self.stocks.iter().any(|o|o.owner==u16::from(owner)&&o.town==city){*self.pending_ai_expansion=Some((s.head,original,after,owner,city));*self.pending_office=Some((owner,city,self.tick));return Ok(false)}}Ok(true)})();return match result{Ok(v)=>v,Err(e)=>{let error=format!("tarea 0x2A: {e}");if self.errors.last()!=Some(&error){self.errors.push(error)}false}}}
        if kind==0x2c{let result=||->Result<(),String>{if self.world.network_client==0{self.ai_construction_preview()?;}let task=s.records.get(usize::from(s.head)).ok_or("tarea de construcción IA ausente")?;let interval=if self.tick.wrapping_sub(task.u32_at(0x10))>=0x5d00{self.ai_build_interval_preview(s)?.u32_at(0xc)}else{task.u32_at(0xc)};if interval==0{return Err("intervalo de construcción IA nulo".into())}Ok(())};return match result(){Ok(())=>true,Err(e)=>{let error=format!("tarea 0x2C: {e}");if self.errors.last()!=Some(&error){self.errors.push(error)}false}}}
        if kind==0x06{return match self.shipyard_weekly_preview(){Ok(_)=>true,Err(e)=>{let error=format!("tarea 0x06: {e}");if self.errors.last()!=Some(&error){self.errors.push(error)}false}}}
        if kind==0x20{return match self.alderman_election_preview(s){Ok(_)=>true,Err(e)=>{let error=format!("tarea 0x20: {e}");if self.errors.last()!=Some(&error){self.errors.push(error)}false}}}
        if kind==0x22{return match self.town_notices_preview(s){Ok(_)=>true,Err(e)=>{let error=format!("tarea 0x22: {e}");if self.errors.last()!=Some(&error){self.errors.push(error)}false}}}
        if kind==0x11{return match self.supply_deadline_preview(s){Ok(_)=>true,Err(e)=>{let error=format!("tarea 0x11: {e}");if self.errors.last()!=Some(&error){self.errors.push(error)}false}}}
        if kind==0x14{return match self.reminder_preview(s){Ok(_)=>true,Err(e)=>{let error=format!("tarea 0x14: {e}");if self.errors.last()!=Some(&error){self.errors.push(error)}false}}}
        if kind==0x09{return match self.harvest_preview(s){Ok(_)=>true,Err(e)=>{let error=format!("tarea 0x09: {e}");if self.errors.last()!=Some(&error){self.errors.push(error)}false}}}
        if kind==0x15{return match self.wedding_preview(s){Ok(_)=>true,Err(e)=>{let error=format!("tarea 0x15: {e}");if self.errors.last()!=Some(&error){self.errors.push(error)}false}}}
        if kind==0x0d{return match self.weather_preview(s){Ok(_)=>true,Err(e)=>{let error=format!("tarea 0x0D: {e}");if self.errors.last()!=Some(&error){self.errors.push(error)}false}}}
        if kind==0x17{return match self.dossier_preview(s){Ok(_)=>true,Err(e)=>{let error=format!("tarea 0x17: {e}");if self.errors.last()!=Some(&error){self.errors.push(error)}false}}}
        if kind==0x16{return match self.matchmaker_preview(s){Ok(_)=>true,Err(e)=>{let error=format!("tarea 0x16: {e}");if self.errors.last()!=Some(&error){self.errors.push(error)}false}}}
        // `Scheduler::dispatch` llama a `run_task` justo después, sin tocar nada: se guarda el resultado para no repetir el guion.
        if kind==0x1b{return match self.mission_script_preview(s){Ok(out)=>{self.mission_cached=s.records.get(usize::from(s.head)).map(|task|(s.head,*task,out));true},Err(e)=>{let script=s.records.get(usize::from(s.head)).map(|t|u16::from_le_bytes([t.u8_at(0x10),t.u8_at(0x11)]));let error=match script{Some(id)=>format!("tarea 0x1B (guion {id}): {e}"),None=>format!("tarea 0x1B: {e}")};if self.errors.last()!=Some(&error){self.errors.push(error)}false}}}
        if kind==0x04 {
            let Some(task)=s.records.get(usize::from(s.head)) else{return false};
            let merchant=u16::from_le_bytes([task.u8_at(8),task.u8_at(9)]);
            if merchant>=self.world.merchant_count{return true}
            match self.merchants.get(usize::from(merchant)) {
                Some(m) if m.u16(8)==0=>return true,
                Some(_)=>return match self.merchant_ai_preview(*task,s){Ok(_)=>true,Err(e)=>{let error=format!("tarea 0x04: {e}");if self.errors.last()!=Some(&error){self.errors.push(error)}false}},
                None=>{self.errors.push("tarea 0x04: comerciante ausente".into());return false},
            }
        }
        if kind==0x03 {return match self.ten_day_preview(s){Ok(_)=>true,Err(e)=>{self.errors.push(format!("tarea programada 0x03: {e}"));false}}}
        if kind==0x07 {return match self.celebration_preview(s){Ok(_)=>true,Err(e)=>{self.errors.push(format!("tarea programada 0x07: {e}"));false}}}
        if kind==0x1c {return match self.plague_preview(s){Ok(_)=>true,Err(e)=>{self.errors.push(format!("tarea programada 0x1C: {e}"));false}}}
        if kind==0x33 {return match self.theft_preview(s){Ok(_)=>true,Err(e)=>{self.errors.push(format!("tarea programada 0x33: {e}"));false}}}
        if kind==0x1d {return match self.fire_preview(s){Ok(_)=>true,Err(e)=>{self.errors.push(format!("tarea programada 0x1D: {e}"));false}}}
        if kind==0x00 {
            let Some(mut task)=s.records.get(usize::from(s.head)).copied() else{return false};
            let mut towns=self.world.towns.clone();let mut rng=*self.investigation_rng;
            return match p3_core::economy::spice::arrival(&mut task,&mut towns,self.mission_town_ids,self.mission_locations.as_ref(),self.world.network_client!=0,self.operations_mode,&mut rng){Ok(_)=>true,Err(e)=>{let error=format!("tarea 0x00: {e}");if self.errors.last()!=Some(&error){self.errors.push(error)}false}}
        }
        if kind==0x0b{return match self.ship_retry_preview(s){Ok(_)=>true,Err(e)=>{let error=format!("tarea 0x0B: {e}");if self.errors.last()!=Some(&error){self.errors.push(error)}false}}}
        if kind==0x28 {return match self.offer_preview(s){Ok(_)=>true,Err(e)=>{let error=format!("tarea 0x28: {e}");if self.errors.last()!=Some(&error){self.errors.push(error)}false}}}
        if kind==0x27 {return match self.retirement_preview(s){Ok(_)=>true,Err(e)=>{let error=format!("tarea 0x27: {e}");if self.errors.last()!=Some(&error){self.errors.push(error)}false}}}
        if kind==0x19 {
            let Some(mut task)=s.records.get(usize::from(s.head)).copied() else{return false};
            let mut rng=*self.investigation_rng;
            return match p3_core::economy::loan_slots::refresh(&mut task,&self.world.towns,self.world.merchant_count,self.world.network_client!=0,self.world.world_8a,self.expedition_operations.len().min(52) as u16,self.loan_names,&mut rng){Ok(_)=>true,Err(e)=>{let error=format!("tarea 0x19: {e}");if self.errors.last()!=Some(&error){self.errors.push(error)}false}}
        }
        if kind==0x1a {
            if usize::from(self.world.merchant_count)>self.merchants.len() || self.world.towns.len()>40 {
                let error="tarea 0x1a: número de comerciantes o ciudades inválido".to_owned();if self.errors.last()!=Some(&error){self.errors.push(error)}return false
            }
            return true;
        }
        if kind==0x37 {
            let Some(task)=s.records.get(usize::from(s.head)) else{return false};
            let merchant=task.u32_at(0x0c) as usize;
            if merchant>=self.merchants.len(){let error=format!("tarea 0x37: comerciante {merchant} ausente");if self.errors.last()!=Some(&error){self.errors.push(error)}return false}
            return true;
        }
        if kind==0x25 {
            if !self.siege_headless{return false}let Some(task)=s.records.get(usize::from(s.head)).copied()else{return false};
            return match crate::siege_daily::prepare_ruin(task,&self.world.towns,self.sites){Ok(_)=>true,Err(e)=>{let e=format!("tarea de derrumbe 0x25: {e}");if self.errors.last()!=Some(&e){self.errors.push(e)}false}}
        }
        if kind==0x26 {
            if !self.siege_headless { return false }
            if s.records.get(usize::from(s.head)).is_none(){return false}
            return match crate::siege_daily::prepare_burn(s,s.head,&self.world.towns,self.sites,self.world.merchant_count,self.stocks,self.burn_lists,self.burn_housing) {
                Ok(_)=>true, Err(e)=>{let e=format!("tarea de incendio 0x26: {e}");if self.errors.last()!=Some(&e){self.errors.push(e)}false}
            }
        }
        if kind==0x2f {
            use p3_core::ai::town_development::{self,DevelopmentEffect};
            let Some(mut task)=s.records.get(usize::from(s.head)).copied() else{return false};let mut towns=self.world.towns.clone();
            let result=town_development::tick(&mut task,&mut towns,self.sites,self.merchants,self.world.merchant_count,self.world.network_client!=0,self.tick);
            let err=match result {
                Ok(r)=>match r.effect {
                    DevelopmentEffect::CreateOffice{merchant,city} if !self.stocks.iter().any(|o|o.owner==u16::from(merchant)&&o.town==city)=>{*self.pending_office=Some((merchant,city,self.tick));return false},
                    DevelopmentEffect::PlaceLender{city}=>match self.sites.get(usize::from(city)){Some(site)=>crate::lender::plan(&towns[usize::from(city)],site,self.world.merchant_count).err(),None=>Some("plano del prestamista ausente".to_owned())},
                    _=>None,
                },
                Err(e)=>Some(e.to_owned()),
            };
            if let Some(e)=err{let error=format!("tarea 0x2f: {e}");if self.errors.last()!=Some(&error){self.errors.push(error)}return false}return true
        }
        if kind==0x39 {
            let Some(mut task)=s.records.get(usize::from(s.head)).copied() else{return false};
            return match p3_core::buildings::wall_task::tick(&mut task,&self.world.towns,self.sites,self.tick) {
                Ok(result) => {
                    if result.place.is_none(){return true}
                    let city=usize::from(task.u8_at(8));let town=&self.world.towns[city];let site=&self.sites[city];
                    let rows=town.i32(p3_core::town::off::PLAN_ROWS);let cols=town.i32(p3_core::town::off::PLAN_STRIDE);
                    let valid=rows>0 && cols>0 && rows<=255 && cols<=255 && (rows as usize).checked_mul(cols as usize).is_some_and(|n| n<=site.cells.len()) && town.u16(p3_core::town::off::FREE_BUILDING_HEAD)<=town.u16(p3_core::town::off::BUILDING_RECORD_COUNT) && town.u16(p3_core::town::off::BUILDING_RECORD_COUNT)<=u16::MAX-128;
                    if !valid {let e="tarea 0x39: plano o pool inválido".to_owned();if self.errors.last()!=Some(&e){self.errors.push(e)}}valid
                },
                Err(e) => {let e=format!("tarea 0x39: {e}");if self.errors.last()!=Some(&e){self.errors.push(e)}false}
            }
        }
        if kind==0x32 {if let Some(task)=s.records.get(usize::from(s.head)) {if task.u8_at(0xc)==3&&task.u8_at(0xf)!=0&&task.u8_at(0x12)==1&&task.due<task.u32_at(8)&&u16::from(task.u8_at(0xd))<self.world.merchant_count {if let (Some(targets),Some(locations),Some(factory))=(self.mission_pirate_targets.as_ref(),self.mission_locations.as_ref(),self.expedition_factory.as_ref()){let band=usize::from(task.u8_at(0x10));if targets.get(band).copied().flatten()==Some(task.u8_at(0x11))&&self.expedition_images.len()==factory.ships.len(){let ships=self.expedition_images.iter().zip(&factory.ships).map(|(live,fallback)|live.as_ref().unwrap_or(fallback).clone()).collect::<Vec<_>>();if let Some(merchant)=self.merchants.get(usize::from(task.u8_at(0xd))){let mut probe=*task;match p3_core::ai::settlement::track_pirate_mission(&mut probe,targets,locations,&ships,merchant.u16(0xe)){Ok(None)=>{*self.pending_pirate_attack=Some((s.head,*task));return false},Ok(Some(_))=>{},Err(e)=>{self.errors.push(format!("ataque pirata: {e}"));return false}}}}}}}}
        if kind==0x32 {if let Some(task)=s.records.get(usize::from(s.head)) {if task.u8_at(0xc)==3&&task.u8_at(0xf)!=0&&task.u8_at(0x12)==0&&task.due<task.u32_at(8)&&u16::from(task.u8_at(0xd))<self.world.merchant_count {if let (Some(targets),Some(locations),Some(factory))=(self.mission_pirate_targets.as_ref(),self.mission_locations.as_ref(),self.expedition_factory.as_ref()){if self.expedition_images.len()==factory.ships.len(){let ships=self.expedition_images.iter().zip(&factory.ships).map(|(live,fallback)|live.as_ref().unwrap_or(fallback).clone()).collect::<Vec<_>>();let owner=usize::from(task.u8_at(0xd));if let Some(merchant)=self.merchants.get(owner){match p3_core::ai::settlement::prepare_pirate_mission_approach(task,merchant,targets,locations,&ships,merchant.u16(0xe)){Ok(Some(_))=>{*self.pending_pirate_approach=Some((s.head,*task));return false},Ok(None)=>{},Err(e)=>{self.errors.push(format!("aproximación pirata: {e}"));return false}}}}}}}}
        if kind==0x32 {if let Some(task)=s.records.get(usize::from(s.head)){if task.u8_at(0xc)==0&&task.u8_at(0xf)==0&&u16::from(task.u8_at(0xd))<self.world.merchant_count{*self.pending_founding=Some((s.head,*task));return false}}match self.mission_pirate_abort_preview(s){Ok(Some(_))=>return true,Ok(None)=>{},Err(error)=>{if self.errors.last()!=Some(&error){self.errors.push(error)}return false}}self.road_build_cached=None;match self.mission_road_build_preview(s){Ok(Some(result))=>{self.road_build_cached=Some(result);return true},Ok(None)=>{},Err(error)=>{if self.errors.last()!=Some(&error){self.errors.push(error)}return false}}let mut staged=s.clone();return match self.tick_mission_preview(&mut staged){Ok(Some(_))=>true,Ok(None)=>match self.mission_ai_failure_preview(s){Ok(Some(_))=>true,Ok(None)=>match self.mission_human_failure_preview(s){Ok(Some(_))=>true,Ok(None)=>match self.mission_road_start_preview(s){Ok(Some(_))=>true,Ok(None)=>match self.mission_pirate_start_preview(s){Ok(Some(_))=>true,Ok(None)=>match self.mission_completion_preview(s){Ok(Some(_))=>true,Ok(None)=>{let error="misión del alcalde: fase o resolución no implementada".to_owned();if self.errors.last()!=Some(&error){self.errors.push(error)}false},Err(error)=>{if self.errors.last()!=Some(&error){self.errors.push(error)}false}},Err(error)=>{if self.errors.last()!=Some(&error){self.errors.push(error)}false}},Err(error)=>{if self.errors.last()!=Some(&error){self.errors.push(error)}false}},Err(error)=>{if self.errors.last()!=Some(&error){self.errors.push(error)}false}},Err(error)=>{if self.errors.last()!=Some(&error){self.errors.push(error)}false}},Err(error)=>{if self.errors.last()!=Some(&error){self.errors.push(error)}false}}}
        if kind==0x05 {let Some(r)=s.records.get(s.head as usize) else{return false};if usize::from(r.u8_at(9))>=self.world.towns.len()||usize::from(r.u8_at(11))>=self.world.towns.len()||u16::from(r.u8_at(8))>=self.world.merchant_count||r.u8_at(0x11)==0&&r.u8_at(0x10)>15{return true}}
        if kind==0x05 {let Some(r)=s.records.get(s.head as usize) else{return false};if r.u8_at(0x11)!=0 {let Some(office)=self.investigation_office(r) else{return false};let mut record=*r;let mut towns=self.world.towns.clone();let mut merchants=self.merchants.to_vec();let mut stocks=self.stocks.to_vec();let stock=office.map(|i|&mut stocks[i].stock);let Some(verdict)=c10_investigation::verdict(&mut record,&mut towns,&mut merchants,self.tick,stock) else{return false};if let Some((i,town,kind,payload))=verdict.letter{let mut pool=self.pool.clone();let date=p3_core::calendar::Calendar::for_tick(self.tick);let enabled=self.enabled.get(i).copied().unwrap_or(true);if let Err(e)=p3_state::messages::add_hanse_verdict(&mut pool,&mut merchants[i],date.day,date.month,date.year,town,kind,payload,enabled){if self.errors.last()!=Some(&e){self.errors.push(e)}return false}}return true}}
        if kind==0x05 {if !self.investigation_local || self.world.network_client!=0{return false}let Some(mut rng)=*self.investigation_rng else{return false};let Some(r)=s.records.get(s.head as usize) else{return false};let mut task=*r;let random=[rng.rand() as u16,rng.rand() as u16];let Some(message)=c10_investigation::begin(&mut task,self.world.towns.len(),self.merchants,random) else{return false};if let Some((i,town,payload))=message{let mut pool=self.pool.clone();let mut merchant=self.merchants[i].clone();let date=p3_core::calendar::Calendar::for_tick(self.tick);let enabled=self.enabled.get(i).copied().unwrap_or(true);if let Err(e)=c10_investigation::publish_indictment(&mut pool,&mut merchant,date,town,payload,enabled,task.u8_at(16)){if self.errors.last()!=Some(&e){self.errors.push(e)}return false}}return true}
        if matches!(kind,0x23|0x24|0x34){let Some(r)=s.records.get(s.head as usize) else{return false};
            if usize::from(r.u8_at(0x14))>=self.world.towns.len(){return true}
            let phase=r.u8_at(0x15);
            if phase==1 {return self.factory_preview(r,s.head).is_some()}
            if phase==0 {let Some(scores)=self.expedition_scores(r) else{return false};let mut record=*r;let Some(letters)=p3_core::economy::hanse_expedition::select(&mut record,self.merchants,&scores,self.pirates.as_ref()) else{return false};let mut pool=self.pool.clone();let mut merchants=self.merchants.to_vec();let date=p3_core::calendar::Calendar::for_tick(self.tick);for (i,payload) in letters {let enabled=self.enabled.get(i).copied().unwrap_or(true);if let Err(e)=p3_state::messages::add_hanse_expedition_request(&mut pool,&mut merchants[i],date.day,date.month,date.year,self.world.alderman_town,payload,enabled){if self.errors.last()!=Some(&e){self.errors.push(e)}return false}}return true}
            if phase==8 {return kind==0x34&&self.hunt_idle_preview(r).is_some()}
            if phase==10 {if kind!=0x34{return false}let r=*r;return match self.hunt_return_preview(&r,s.head){Ok(Some(_))=>true,Ok(None)=>false,Err(e)=>{if self.errors.last()!=Some(&e){self.errors.push(e)}false}}}
            if phase==9 {if self.hunt_idle_preview(r).is_some(){return true}let id=usize::from(u16::from_le_bytes([r.u8_at(0x16),r.u8_at(0x17)]));let convoy=self.expedition_convoys.get(id).map(Vec::as_slice);let mut record=*r;return p3_core::economy::hanse_expedition::hunt_wait(&mut record,self.tick,convoy).is_some()}
            if phase==7 {if kind!=0x34 || !self.expedition_coherent() || self.expedition_convoys.len()!=self.convoy_flags.len(){return false}let id=usize::from(u16::from_le_bytes([r.u8_at(0x16),r.u8_at(0x17)]));let Some(convoy)=self.expedition_convoys.get(id) else{return false};let mut convoy=convoy.clone();let mut ships=self.expedition_images.to_vec();let mut record=*r;return p3_core::economy::hanse_expedition::hunt_depart(&mut record,&mut convoy,&mut ships,self.world.ship_count).is_some()}
            if phase==5 {if !self.expedition_coherent(){return false}let mut towns=self.world.towns.clone();let count=match u16::try_from(self.expedition_convoys.len()){Ok(n)=>n,Err(_)=>return false};let id=usize::from(u16::from_le_bytes([r.u8_at(0x16),r.u8_at(0x17)]));if id<usize::from(count){match p3_core::economy::hanse_expedition::convoy_is_active(id as u16,*self.expedition_head,self.expedition_convoys){Some(false)=>return true,None=>return false,_=>{}}}let letters=if id>=usize::from(count){p3_core::economy::hanse_expedition::finish_without_convoy(r,&mut towns,self.merchants,count)}else{let mut convoy=self.expedition_convoys[id].clone();let mut ships=self.expedition_images.to_vec();let mut docked=*self.expedition_docked;p3_core::economy::hanse_expedition::finish_convoy(r,&mut towns,self.merchants,&mut convoy,&mut ships,self.world.ship_count,&mut docked)};let Some(letters)=letters else{return false};let mut pool=self.pool.clone();let mut merchants=self.merchants.to_vec();let date=p3_core::calendar::Calendar::for_tick(self.tick);for (i,payload) in letters {let enabled=self.enabled.get(i).copied().unwrap_or(true);if let Err(e)=p3_state::messages::add_hanse_expedition_finished(&mut pool,&mut merchants[i],date.day,date.month,date.year,self.world.alderman_town,payload,enabled){if self.errors.last()!=Some(&e){self.errors.push(e)}return false}}return true}
            if phase==2 {let mut record=*r;let mut towns=self.world.towns.clone();let mut merchants=self.merchants.to_vec();let Some(letters)=p3_core::economy::hanse_expedition::penalty(&mut record,&mut towns,&mut merchants,self.tick,self.pirates.as_ref()) else{return false};
                let mut pool=self.pool.clone();let date=p3_core::calendar::Calendar::for_tick(self.tick);for (i,payload) in letters {let enabled=self.enabled.get(i).copied().unwrap_or(true);if let Err(e)=p3_state::messages::add_hanse_expedition_penalty(&mut pool,&mut merchants[i],date.day,date.month,date.year,self.world.alderman_town,payload,enabled){if self.errors.last()!=Some(&e){self.errors.push(e)}return false}}return true
            }
            let ships=if phase==4{Vec::new()}else{let Some(v)=self.expedition_ships() else{return false};v};
            let mut record=*r;let mut towns=self.world.towns.clone();return p3_core::economy::hanse_expedition::patrol(&mut record,&mut towns,self.merchants,&ships,self.world.ship_count,self.world.network_client!=0).is_some()
        }
        if kind==0x21 {
            let Some(r)=s.records.get(s.head as usize) else{return false};let mut probe=*r;
            if p3_core::economy::hanse_petition::cancel_hanse_vote(r,&self.world.towns) || p3_core::economy::hanse_petition::advance_vote(&mut probe,&self.world.towns) || p3_core::economy::hanse_petition::start_vote(&mut probe,&self.world.towns,self.merchants).is_some(){return true}
            let mut towns=self.world.towns.clone();let Some((_,seat,payload))=p3_core::economy::hanse_petition::finish_vote(&mut probe,&mut towns,self.world.alderman_town,self.tick) else{return false};
            let mut pool=self.pool.clone();let mut merchants=self.merchants.to_vec();let date=p3_core::calendar::Calendar::for_tick(self.tick);
            for (i,m) in merchants.iter_mut().enumerate(){let enabled=self.enabled.get(i).copied().unwrap_or(true);if let Err(e)=p3_state::messages::add_hanse_vote_result(&mut pool,m,date.day,date.month,date.year,seat,payload,enabled){if self.errors.last()!=Some(&e){self.errors.push(e)}return false}}
            return true
        }
        if kind==0x30 {
            let Some(r)=s.records.get(s.head as usize) else{return false};
            if p3_core::economy::hanse_petition::is_terminal(r){return true}
            let alderman=self.world.alderman_merchant;
            if p3_core::economy::hanse_petition::is_ai_terminal(r,alderman,self.merchants){return true}
            if p3_core::economy::hanse_petition::can_ai(r,alderman,self.merchants,&self.world.towns,self.world.alderman_town){
                let mut pool=self.pool.clone();let mut merchants=self.merchants.to_vec();let d=p3_core::calendar::Calendar::for_tick(self.tick);
                let mut task=*r;let event=if let Some(target)=p3_core::economy::hanse_petition::ai_announce(&mut task,alderman,self.merchants,&self.world.towns,self.world.alderman_town){Some((false,self.world.alderman_town,u32::from(target),0))}else{
                    let mut copy=s.clone();p3_core::economy::hanse_petition::ai_schedule(&mut copy,alderman,self.merchants,&self.world.towns,self.world.alderman_town,self.tick).map(|(t,a,b)|(true,t,a,b))
                };let Some((vote,town,a,b))=event else{return false};
                let payload=if !vote{let mut temp=*r;let Some((payload,_))=self.petition_payload(&mut temp,alderman,false) else{return false};Some(payload)}else{None};
                for (i,m) in merchants.iter_mut().enumerate(){let enabled=self.enabled.get(i).copied().unwrap_or(true);let check=if vote{p3_state::messages::add_hanse_vote(&mut pool,m,d.day,d.month,d.year,town,a,b,enabled)}else{p3_state::messages::add_hanse_petition(&mut pool,m,d.day,d.month,d.year,town,payload.expect("validado"),enabled)};
                    if let Err(e)=check{if self.errors.last()!=Some(&e){self.errors.push(e)}return false}
                }return true
            }
            let announce=p3_core::economy::hanse_petition::can_announce(r,alderman,self.merchants);
            let reminder=p3_core::economy::hanse_petition::can_remind(r,alderman,self.merchants);
            let finish=p3_core::economy::hanse_petition::can_finish(r,alderman,self.merchants);
            if !announce && !reminder && !finish{return false}
            let mut pool=self.pool.clone();let mut merchants=self.merchants.to_vec();let d=p3_core::calendar::Calendar::for_tick(self.tick);
            if announce {let mut temp=*r;let Some((payload,directed))=self.petition_payload(&mut temp,alderman,true) else{return false};for (i,m) in merchants.iter_mut().enumerate(){if directed && i!=usize::from(alderman){continue}let enabled=self.enabled.get(i).copied().unwrap_or(true);
                if let Err(e)=p3_state::messages::add_hanse_petition(&mut pool,m,d.day,d.month,d.year,self.world.alderman_town,payload,enabled){if self.errors.last()!=Some(&e){self.errors.push(e)}return false}
            }} else if r.u8_at(0xc)!=6 && !matches!(r.u8_at(0x13),5|6) {let i=alderman as usize;let enabled=self.enabled.get(i).copied().unwrap_or(true);
                let check=if finish {p3_state::messages::add_hanse_reprimand(&mut pool,&mut merchants[i],d.day,d.month,d.year,self.world.alderman_town,enabled)}else{p3_state::messages::add_hanse_reminder(&mut pool,&mut merchants[i],d.day,d.month,d.year,self.world.alderman_town,enabled)};
                if let Err(e)=check{if self.errors.last()!=Some(&e){self.errors.push(e)}return false}
            }
            return true
        }
        if kind==0x01 {
            let Some(r)=s.records.get(s.head as usize) else{return false};
            if p3_core::economy::debt::is_terminal(r) || p3_core::economy::debt::missing_debtor_is_terminal(r,self.merchants){return true}
            {let mut temp=*r;let mut merchants=self.merchants.to_vec();if p3_core::economy::debt::settle_negative_amount(&mut temp,&mut merchants){return true}}
            let notice=p3_core::economy::debt::can_notify_debtor(r,self.merchants);
            let credit=p3_core::economy::debt::can_credit_external(r,self.merchants);
            let alert=p3_core::economy::debt::can_alert_external(r,self.merchants);
            let borrower=r.u8_at(0x11) as usize;
            let assets=self.debt_assets(r);
            let office=self.debt_office(r);
            let ships=self.debt_ships(r);
            let partial=p3_core::economy::debt::can_settle_partial_cash(r,self.merchants) &&
                self.offices.len()==self.world.offices.len() && self.offices.iter().all(|o|usize::from(o.owner)!=borrower) &&
                self.merchants[borrower].u16(0xe)>=self.world.ship_count && usize::from(r.u8_at(0x14))<self.world.towns.len() &&
                usize::from(self.merchants[borrower].bytes[0x19])<self.world.towns.len();
            if !notice && !credit && !alert && !partial && assets.is_none() && office.is_none() && ships.is_none() && !p3_core::economy::debt::can_settle_cash(r,self.merchants){return false}
            let i=if credit || alert{r.u8_at(0x10)}else{r.u8_at(0x11)} as usize;let mut pool=self.pool.clone();let mut merchant=self.merchants[i].clone();
            let date=p3_core::calendar::Calendar::for_tick(self.tick);let enabled=self.enabled.get(i).copied().unwrap_or(true);
            let probe=if let Some((mut stocks,mut ships,_))=assets {let mut task=*r;let mut merchants=self.merchants.to_vec();let mut towns=self.world.towns.clone();let first=merchants[borrower].u16(0xe);let first=if first<self.world.ship_count{first}else{u16::MAX};
                let (_,events)=p3_core::economy::debt::seize_two_cities(&mut task,&mut merchants,&mut towns,&mut stocks,&mut ships,first,self.difficulty).expect("validado");
                let mut result=Ok(false);for e in events {if e.letter{result=p3_state::messages::add_debt_ship_seizure(&mut pool,&mut merchant,date.day,date.month,date.year,e.town,e.ship_name,e.kind,e.settled,e.reported,enabled);if result.is_err(){break}}}result
            }else if let Some(mut ships)=ships {let mut task=*r;let mut merchants=self.merchants.to_vec();let mut town=self.world.towns[usize::from(r.u8_at(0x14))].clone();let first=merchants[borrower].u16(0xe);
                let result=p3_core::economy::debt::seize_ships(&mut task,&mut merchants,&mut town,&mut ships,first,self.difficulty).expect("validado");
                if result.letter {p3_state::messages::add_debt_ship_seizure(&mut pool,&mut merchant,date.day,date.month,date.year,result.town,result.ship_name,result.kind,result.settled,result.reported,enabled)}else{Ok(false)}
            }else if let Some(index)=office {let mut task=*r;let mut merchants=self.merchants.to_vec();let mut town=self.world.towns[usize::from(r.u8_at(0x14))].clone();let mut stock=self.stocks[index].stock;
                let result=p3_core::economy::debt::seize_one_office(&mut task,&mut merchants,&mut town,&mut stock,self.difficulty).expect("validado");
                if result.notify {p3_state::messages::add_debt_seizure(&mut pool,&mut merchant,date.day,date.month,date.year,result.town,result.kind,result.settled,result.reported,enabled)}else{Ok(false)}
            }else if partial {let mut task=*r;let mut merchants=self.merchants.to_vec();let result=p3_core::economy::debt::settle_partial_cash(&mut task,&mut merchants).expect("validado");
                if result.notify {p3_state::messages::add_debt_partial(&mut pool,&mut merchant,date.day,date.month,date.year,result.town,result.reported,enabled)}else{Ok(false)}
            }else if alert {let mut temp=*r;let (_,town,a,b)=p3_core::economy::debt::alert_external(&mut temp,self.merchants,s.head).expect("validado");p3_state::messages::add_debt_alert(&mut pool,&mut merchant,date.day,date.month,date.year,town,a,b,enabled)}else if credit {p3_state::messages::add_debt_credit(&mut pool,&mut merchant,date.day,date.month,date.year,r.u8_at(0x14),r.u32_at(8) as u16,r.u32_at(0xc),enabled)}else if notice {p3_state::messages::add_debt_notice(&mut pool,&mut merchant,date.day,date.month,date.year,r.u8_at(0x14),r.u32_at(8) as u16,r.u32_at(0xc),enabled)} else {p3_state::messages::add_debt_cash(&mut pool,&mut merchant,date.day,date.month,date.year,r.u8_at(0x14),r.u32_at(8),enabled)};
            if let Err(e)=probe{
                if self.errors.last()!=Some(&e){self.errors.push(e)}return false
            }
            return true
        }
        if kind!=0x31{return true}
        let result=if self.offices.len()<self.world.offices.len(){Err("oficinas de recargo incompletas".to_owned())}else{p3_core::economy::extra_tax::validate(s,&self.world.towns,self.merchants,self.offices)};
        if let Err(e)=result{if self.errors.last()!=Some(&e){self.errors.push(e)}return false}true
    }
    fn run_task(&mut self,s:&mut p3_core::scheduler::Scheduler,call:p3_core::scheduler::HandlerCall)->u32 {
        if call.kind==0x18{let out=self.profit_share_preview(s).expect("validado");*s=out.scheduler;self.mission_variables.resize(usize::from(s.capacity),None);for(i,m)in out.merchants.into_iter().enumerate(){self.world.merchants[i].gold=m.i32(0);self.world.merchants[i].income=m.i32(0x4c8);self.merchants[i]=m}*self.pool=out.pool;return u32::from(out.keep)}
        if call.kind==0x13{
            let out=self.trade_offer_preview(s).expect("validado");
            self.stocks.clone_from_slice(&out.stocks);*self.scheduled_office_images=Some(out.office_images);self.merchants[..out.merchants.len()].clone_from_slice(&out.merchants);
            for (m,account) in out.merchants.iter().zip(self.world.merchants.iter_mut()){account.gold=m.i32(0);account.income=m.i32(0x4c8)}
            *self.pool=out.pool;*self.trade_offers=Some(out.offers);*self.navigation=out.nav;s.records[usize::from(s.head)]=out.task;return u32::from(out.keep)
        }
        if call.kind==0x0c{
            let out=self.land_transport_preview(s).expect("validado");
            for(i,stock) in out.stocks.iter().enumerate(){let image=&mut self.scheduled_office_images.as_mut().expect("validado")[i];for w in 0..24{image[4+w*4..8+w*4].copy_from_slice(&stock.stock[w].to_le_bytes());image[0x3b8+w*4..0x3bc+w*4].copy_from_slice(&stock.average_price[w].to_bits().to_le_bytes());}}
            self.stocks.clone_from_slice(&out.stocks);for(i,m)in out.merchants.iter().enumerate(){self.world.merchants[i].gold=m.i32(0);self.merchants[i]=m.clone();}
            *self.pool=out.message_pool;self.land_transport_notifications.extend(out.notifications);s.records[usize::from(s.head)]=out.task;return u32::from(out.keep)
        }
        if call.kind==0x0e{let out=self.game_end_preview(s).expect("validado");s.records[usize::from(s.head)]=out.task;if let Some(event)=out.event{*self.game_end_stopped=true;self.game_end_events.push(event)}return u32::from(out.keep)}
        if call.kind==0x0f{let out=self.burglary_preview(s).expect("validado");*s=out.core.scheduler;self.mission_variables.resize(usize::from(s.capacity),None);self.world.towns=out.core.towns;self.stocks.clone_from_slice(&out.stocks);self.merchants.clone_from_slice(&out.core.merchants);for(account,person)in self.world.merchants.iter_mut().zip(self.merchants.iter()){account.gold=person.i32(0);account.income=person.i32(0x4c8)}*self.pool=out.pool;*self.scheduled_office_images=Some(out.core.offices);self.scheduled_office_updates.extend(out.updates);return 0}
        if call.kind==0x10{
            let(out,pool)=self.traveller_deadline_preview(s).expect("validado");s.records[usize::from(s.head)]=out.task;let ship_id=usize::from(u16::from_le_bytes([out.task.u8_at(8),out.task.u8_at(9)]));let convoy_id=out.ships.get(ship_id*0x180..ship_id*0x180+0x180).map(|ship|usize::from(u16_at(ship,8)));
            // Las imagenes son vistas de los mismos objetos; se publican antes
            // de que otra tarea del mismo tick pueda consultar la flota.
            for (i,new) in out.ships.chunks_exact(0x180).enumerate(){
                if i!=ship_id{continue}
                let old=if let Some(nav)=self.navigation.as_ref(){nav.snapshot.ships[i*0x180..(i+1)*0x180].to_vec()}else{self.expedition_factory.as_ref().expect("validado").ships[i].to_vec()};if old[0x3c..0x3e]==new[0x3c..0x3e]&&old[0x157]==new[0x157]{continue}
                if let Some(nav)=self.navigation.as_mut(){nav.snapshot.ships[i*0x180..(i+1)*0x180].copy_from_slice(new)}
                if let Some(f)=self.expedition_factory.as_mut(){if let Some(dst)=f.ships.get_mut(i){dst[0x3c..0x3e].copy_from_slice(&new[0x3c..0x3e]);dst[0x157]=new[0x157]}}
                if let Some(Some(dst))=self.expedition_images.get_mut(i){dst[0x3c..0x3e].copy_from_slice(&new[0x3c..0x3e]);dst[0x157]=new[0x157]}
                if let Some(ship)=self.fleet.get_mut(i){ship.dirty=u16_at(new,0x3c)}
            }
            for (i,new) in out.convoys.chunks_exact(0x3c).enumerate(){
                if Some(i)!=convoy_id{continue}
                let old=if let Some(nav)=self.navigation.as_ref(){nav.snapshot.convoys[i*0x3c..(i+1)*0x3c].to_vec()}else{self.expedition_factory.as_ref().expect("validado").convoys[i].to_vec()};if old[0x18..0x1c]==new[0x18..0x1c]{continue}
                if let Some(nav)=self.navigation.as_mut(){nav.snapshot.convoys[i*0x3c..(i+1)*0x3c].copy_from_slice(new)}
                if let Some(f)=self.expedition_factory.as_mut(){if let Some(dst)=f.convoys.get_mut(i){dst[0x18..0x1c].copy_from_slice(&new[0x18..0x1c])}}
                if let Some(dst)=self.expedition_convoys.get_mut(i){dst[0x18..0x1c].copy_from_slice(&new[0x18..0x1c])}
                if let Some(flags)=self.convoy_flags.get_mut(i){*flags=u32_at(new,0x18)}
            }
            self.merchants.clone_from_slice(&out.merchants);for (account,person) in self.world.merchants.iter_mut().zip(self.merchants.iter()){account.gold=person.i32(0);account.income=person.i32(0x4c8)}*self.pool=pool;return u32::from(out.keep)
        }
        if call.kind==0x12{
            if let Ok((task,keep))=self.expedition_return_start_preview(s){s.records[usize::from(s.head)]=task;return u32::from(keep)}
            let staged=self.expedition_return_preview(s).expect("validado");
            self.merchants.clone_from_slice(&staged.state.merchants);for (account,person) in self.world.merchants.iter_mut().zip(self.merchants.iter()).take(usize::from(self.world.merchant_count)){account.gold=person.i32(0);account.income=person.i32(0x4c8)}*self.pool=staged.state.message_pool;*self.return_map=staged.state.global_map;*self.return_flag=staged.state.world_flag;
            if let Some(before)=self.return_centers.as_ref(){for (i,(old,new)) in before.iter().zip(&staged.state.centers).enumerate(){if old!=new{if self.return_center_versions.len()<=i{self.return_center_versions.resize_with(i+1,Default::default)}self.return_center_versions[i].insert(save_version(old));}}}
            *self.return_centers=Some(staged.state.centers);*self.return_map_dirty|=staged.state.dirty;self.return_map_notices.extend(staged.state.notifications);self.return_map_windows.extend(staged.state.windows);
            self.world.towns=staged.finish.towns;*self.return_planner=staged.finish.planner;self.return_notices.extend(staged.finish.notifications);*self.navigation=Some(staged.finish.nav);return 0
        }
        if call.kind==0x38{let(task,sites,merchants,pool,out)=self.mayor_statue_preview(s).expect("validado");s.records[usize::from(s.head)]=task;if let Some((row,col,variant,owner))=out.visual{let city=usize::from(task.u8_at(8));self.sites.clone_from_slice(&sites);if !self.mission_changed_sites.contains(&city){self.mission_changed_sites.push(city)}self.mayor_statue_visuals.push((city as u8,row,col,variant,owner));}self.merchants.clone_from_slice(&merchants);*self.pool=pool;return u32::from(out.keep)}
        if call.kind==0x0d{let out=self.weather_preview(s).expect("validado");*s=out.scheduler;self.world.towns=out.towns;*self.weather_marker=out.marker;*self.investigation_rng=out.rng;self.weather_notices.extend(out.notices);self.expedition_operations.extend(out.operations);return 0}
        if call.kind==0x35{let id=s.records[usize::from(s.head)].u32_at(8) as usize;if let Some(town)=self.world.towns.get_mut(id){town.set_flags(town.flags()&!0x0400_0000);self.weather_notices.push((id as u16,true))}return 0}
        if call.kind==0x2d{return u32::from(p3_core::economy::auction::standing_bid(s,self.tick,self.world.merchant_count))}
        if matches!(call.kind,0x0a|0x36){match s.records[usize::from(s.head)].u8_at(0x14){0=>{let prepared=self.auction_ship_preview(s).expect("validado");s.records[usize::from(s.head)]=prepared.task;self.publish_auction_ship(prepared.before,prepared.after,Some(prepared.ship));return 1},1=>p3_core::economy::auction::wait_for_announcement(&mut s.records[usize::from(s.head)]),2=>{let(task,towns,keep,notices)=self.auction_announce_preview(s).expect("validado");s.records[usize::from(s.head)]=task;self.world.towns=towns;self.auction_notices.extend(notices);return u32::from(keep)},3=>{let after=self.auction_open_preview(s).expect("validado");s.records[usize::from(s.head)]=after},7=>{let out=self.auction_missing_lot_preview(s).expect("validado");s.records[usize::from(s.head)]=out.task;if let Some(result)=out.result{self.auction_results.push(result)}if let Some(award)=out.ship{self.publish_auction_ship(award.before,award.after,None);if let Some(r)=award.route_stops{*self.route_stops=Some(r)}if let Some((o,lists))=award.groups{self.merchant_groups[o]=lists;if !self.merchant_groups_changed.contains(&o){self.merchant_groups_changed.push(o)}}self.merchants.clone_from_slice(&award.merchants);for (world,merchant) in self.world.merchants.iter_mut().zip(self.merchants.iter()){world.gold=merchant.i32(0)}*self.pool=award.pool}if let Some((index,merchant,pool))=out.delivery{self.world.merchants[index].gold=merchant.i32(0);self.merchants[index]=merchant;*self.pool=pool}if let Some(sale)=out.sale{*self.pending_auction_office=None;let snapshot=sale.snapshot;for update in &sale.updates{let i=update.index;let image=&snapshot.offices[i];self.burn_lists[i].next=u16_at(image,0x2ca);self.burn_lists[i].building_head=u16_at(image,0x2ce);let housing=&mut self.burn_housing[i];housing.next=u16_at(image,0x2ca);housing.houses_head=u16_at(image,0x2d0);housing.house_count=u16_at(image,0x2d4);for k in 0..3{housing.capacity[k]=u16_at(image,0x2d8+2*k);housing.assigned[k]=u16_at(image,0x2de+2*k);housing.level[k]=image[0x2e4+k]}self.world.offices[i].next=u16_at(image,0x2ca);self.world.offices[i].first_facility=u16_at(image,0x2cc);}self.world.merchant_facility_count=snapshot.facilities.records.len() as u16;self.world.merchant_facilities=snapshot.facilities.records.iter().map(|r|p3_core::economy::world::MerchantFacility{employees:u16_at(r,4),town:r[7],next:u16_at(r,8)}).collect();*self.business_free=snapshot.facilities.free;*self.employment_businesses=snapshot.facilities.records;self.world.towns[sale.city]=snapshot.town;self.sites[sale.city]=snapshot.site;*self.scheduled_office_images=Some(snapshot.offices);self.scheduled_office_updates.extend(sale.updates);self.merchants.clone_from_slice(&sale.merchants);for (world,merchant) in self.world.merchants.iter_mut().zip(self.merchants.iter()){world.gold=merchant.i32(0)}*self.pool=sale.pool;if !self.mission_changed_sites.contains(&sale.city){self.mission_changed_sites.push(sale.city)}}},8=>{let task=s.records[usize::from(s.head)];let city=p3_core::economy::auction::finish(&task,&mut self.world.towns,s.head).expect("validado");self.auction_finished.push(city);return 0},_=>{let after=self.auction_bids_preview(s).expect("validado");s.records[usize::from(s.head)]=after}}return 1}
        if call.kind==0x02{let(scheduler,towns,merchants,pool,extra)=self.mayor_election_preview(s).expect("validado");*s=scheduler;self.mission_variables.resize(usize::from(s.capacity),None);self.world.towns=towns;self.merchants.clone_from_slice(&merchants);*self.pool=pool;self.election_notifications.extend(extra.notices);if let Some(pirates)=extra.pirates{*self.hunt_bands=Some(pirates)}self.expedition_operations.extend(extra.operations);return 1}
        if call.kind==0x08{let(task,towns,captains,pirates)=self.captain_refill_preview(s).expect("validado");s.records[usize::from(s.head)]=task;self.world.towns=towns;let factory=self.expedition_factory.as_mut().expect("validado");factory.captains=captains.captains;factory.free_captain=captains.free_head;if let Some(nav)=self.navigation.as_mut(){nav.snapshot.captains=factory.captains.iter().flatten().copied().collect();nav.snapshot.manager[0xe0..0xe2].copy_from_slice(&factory.free_captain.to_le_bytes());nav.snapshot.manager[0xf2..0xf4].copy_from_slice(&(factory.captains.len() as u16).to_le_bytes())}*self.pirates=pirates;return 1}
        if call.kind==0x29{let(task,towns,captains,pirates,keep)=self.captain_return_preview(s).expect("validado");s.records[usize::from(s.head)]=task;self.world.towns=towns;let factory=self.expedition_factory.as_mut().expect("validado");for(dst,src)in factory.captains.iter_mut().zip(captains.entries.chunks_exact(16)){dst.copy_from_slice(src)}factory.free_captain=captains.free_head;if let Some(nav)=self.navigation.as_mut(){nav.snapshot.captains=captains.entries;nav.snapshot.manager[0xe0..0xe2].copy_from_slice(&captains.free_head.to_le_bytes())}*self.pirates=pirates;return u32::from(keep)}
        if call.kind==0x2b{let(after,op)=self.ai_ship_order_preview(s).expect("validado");s.records[usize::from(s.head)]=after;if let Some(op)=op{self.expedition_operations.push(op)}return 1}
        if call.kind==0x2a{let i=usize::from(s.head);let after=if let Some((id,before,after,_,_))=self.pending_ai_expansion.take(){debug_assert_eq!(id,s.head);debug_assert_eq!(before,s.records[i]);after}else{self.ai_expansion_preview(s).expect("validado").0};s.records[i]=after;return 1}
        if call.kind==0x06{let out=self.shipyard_weekly_preview().expect("validado");self.world.towns=out.towns;*self.pool=out.pool;self.merchants.clone_from_slice(&out.merchants);return 1}
        if call.kind==0x20{let i=usize::from(s.head);let out=self.alderman_election_preview(s).expect("validado");s.records[i]=out.task;self.world.towns=out.towns;self.merchants.clone_from_slice(&out.merchants);*self.pool=out.pool;self.world.alderman_merchant=out.player;self.world.alderman_town=out.town;self.election_notifications.extend(out.notices);self.election_announcements.extend(out.announcements);if let Some(pirates)=out.extra.pirates{*self.hunt_bands=Some(pirates)}self.expedition_operations.extend(out.extra.operations);return 1}
        if call.kind==0x22{let i=usize::from(s.head);let(task,towns,board)=self.town_notices_preview(s).expect("validado");s.records[i]=task;self.world.towns=towns;*self.town_hall_notices=Some(board);return 1}
        if call.kind==0x11{let i=usize::from(s.head);let out=self.supply_deadline_preview(s).expect("validado");s.records[i]=out.task;self.world.towns=out.towns;*self.pool=out.pool;self.merchants.clone_from_slice(&out.merchants);for (world,person) in self.world.merchants.iter_mut().zip(self.merchants.iter()){world.gold=person.i32(0);world.income=person.i32(0x4c8)}return u32::from(out.keep)}
        if call.kind==0x14{let i=usize::from(s.head);let(task,rng,keep,notice)=self.reminder_preview(s).expect("validado");s.records[i]=task;*self.investigation_rng=rng;if let Some(notice)=notice{self.reminder_notices.push(notice)}return u32::from(keep)}
        if call.kind==0x09{let i=usize::from(s.head);let out=self.harvest_preview(s).expect("validado");s.records[i]=out.task;self.world.towns=out.towns;*self.pool=out.pool;self.merchants.clone_from_slice(&out.merchants);return u32::from(out.keep)}
        if call.kind==0x15{if let Some(prepared)=self.wedding_preview(s).expect("validado"){self.apply_wedding(s,prepared);return 1}return 0}
        if call.kind==0x17{let out=self.dossier_preview(s).expect("validado");*self.pool=out.pool;self.merchants.clone_from_slice(&out.merchants);return 0}
        if call.kind==0x16{let i=usize::from(s.head);let out=self.matchmaker_preview(s).expect("validado");s.records[i]=out.task;*self.pool=out.pool;self.merchants.clone_from_slice(&out.merchants);return 1}
        if call.kind==0x1b{let i=usize::from(s.head);let out=match self.mission_cached.take(){Some((head,task,out)) if head==s.head&&s.records.get(i)==Some(&task)=>out,_=>self.mission_script_preview(s).expect("validado")};if out.flat_rng.is_some(){*self.investigation_rng=out.flat_rng}if let Some(c)=out.flat_created{self.apply_created_ships(c)}if let Some((towns,changed))=out.flat_towns{self.world.towns=towns;for c in changed{if !self.mission_changed_towns.contains(&c){self.mission_changed_towns.push(c)}}}self.script_notifications.extend(out.notifications);self.expedition_operations.extend(out.operations);if !out.ship_reserve.is_empty(){let nav=self.navigation.as_mut().expect("validado");for op in out.ship_reserve{p3_core::mission_script::apply_ship_reserve(&mut nav.snapshot.manager,&mut nav.snapshot.ships,op).expect("validado")}}if !out.flat_ships.is_empty()||!out.flat_convoys.is_empty(){let nav=self.navigation.as_mut().expect("validado");for(k,b)in out.flat_ships{nav.snapshot.ships[k]=b}for(k,b)in out.flat_convoys{nav.snapshot.convoys[k]=b}}for(o,image)in out.flat_offices{let images=self.scheduled_office_images.as_mut().expect("validado");for w in 0..24{self.stocks[o].stock[w]=i32::from_le_bytes(image[4+4*w..8+4*w].try_into().expect("oficina completa"))}images[o][..image.len()].copy_from_slice(&image);let full=images[o].clone();self.scheduled_office_updates.push(p3_core::economy::world::ScheduledOfficeUpdate{index:o,administrator:u16_at(&full,0x2f2),flags:u16_at(&full,0x2d6),experience:None,wage:None,image:Some(full)})}if let Some(names)=out.names{self.expedition_factory.as_mut().expect("validado").name_cursor=names.cursor;if let Some(nav)=self.navigation.as_mut(){nav.snapshot.manager[0xee..0xf0].copy_from_slice(&names.cursor.to_le_bytes())}}if let Some(pool)=out.pool{*self.pool=pool}if let Some(merchants)=out.merchants{self.merchants.clone_from_slice(&merchants);for(i,m)in self.merchants.iter().enumerate(){if let Some(w)=self.world.merchants.get_mut(i){w.gold=m.i32(0);w.income=m.i32(0x4c8);}}}if let Some(next)=out.scheduler{self.mission_variables.resize(usize::from(next.capacity),None);*s=next}else{s.records[i]=out.task}self.mission_variables[i]=if out.finished{None}else{Some(out.vars)};return u32::from(!out.finished)}
        if call.kind==0x04 {
            struct PreparedHost(Option<(p3_core::scheduler::TaskRecord,u32)>);
            impl p3_core::ai::scheduled_turn::ScheduledTurnHost for PreparedHost {
                fn update_base_rep_factor(&mut self,_:u16){/* Aplicado al confirmar la preparación transaccional. */}
                fn run_ai_turn(&mut self,_:u16,_:u16,_:u16,task:&mut p3_core::scheduler::TaskRecord)->u32{let (prepared,delay)=self.0.take().expect("validado");*task=prepared;delay}
            }
            let index=usize::from(s.head);let merchant=u16::from_le_bytes([s.records[index].u8_at(8),s.records[index].u8_at(9)]);
            let flags=self.merchants.get(usize::from(merchant)).map(|m|m.u16(8));
            let prepared=if merchant<self.world.merchant_count&&flags!=Some(0){let out=self.merchant_ai_preview(s.records[index],s).expect("validado");for(i,targets,flags,image)in out.stock_changes{self.stocks[i].ai_target=targets;self.stocks[i].automation_flags=flags;self.scheduled_office_images.as_mut().expect("validado")[i]=image.clone();self.scheduled_office_updates.push(p3_core::economy::world::ScheduledOfficeUpdate{index:i,administrator:u16_at(&image,0x2f2),flags,experience:None,wage:None,image:Some(image)});}if let Some((i,gold))=out.grant{self.merchants[i].set_i32(0,gold);self.world.merchants[i].gold=gold;}self.expedition_operations.extend(out.operations);if let Some(r)=out.rng{*self.investigation_rng=Some(r)}for (i,level,marks,home) in out.fleet_changes{let image=self.expedition_images[i].as_mut().expect("validado");image[0xf]=level;image[0x136]=marks;image[0x13a]=home;if let Some(nav)=self.navigation.as_mut(){nav.snapshot.ships[i*0x180+0xf]=level;nav.snapshot.ships[i*0x180+0x136]=marks;nav.snapshot.ships[i*0x180+0x13a]=home}if !self.ai_fleet_changed_ships.contains(&i){self.ai_fleet_changed_ships.push(i)}}if let Some((i,bits))=out.rep_factor{self.merchants[i].bytes[0x464..0x468].copy_from_slice(&bits.to_le_bytes());}Some((out.task,out.delay))}else{None};
            return u32::from(p3_core::ai::scheduled_turn::dispatch(&mut s.records[index],index as u16,self.world.merchant_count,flags,self.tick,self.world.day_of_year,&mut PreparedHost(prepared)).expect("validado"));
        }
        if call.kind==0x1f {let out=self.town_event_plan_preview(s).expect("validado");*s=out.scheduler;self.mission_variables.resize(usize::from(s.capacity),None);self.world.towns=out.towns;*self.investigation_rng=out.rng;self.expedition_operations.extend(out.operation);return u32::from(out.keep)}
        if call.kind==0x1e {
            let out=self.town_event_preview(s).expect("validado");
            s.records[usize::from(s.head)]=out.task;self.world.towns[out.city]=out.town;
            self.stocks.clone_from_slice(&out.stocks);self.merchants.clone_from_slice(&out.merchants);
            for (account,person) in self.world.merchants.iter_mut().zip(self.merchants.iter()){account.gold=person.i32(0);account.income=person.i32(0x4c8)}
            *self.pool=out.pool;if let Some(offices)=out.offices{*self.scheduled_office_images=Some(offices)}
            self.scheduled_office_updates.extend(out.updates);for(i,weapons)in out.weapons{self.world.offices[i].weapons=weapons}
            if let Some((staged,bands))=out.spawn {
                let before=self.navigation.as_ref().expect("validado").snapshot.ships.len()/0x180;
                let after=&staged.navigation.snapshot;let count=after.ships.len()/0x180;
                self.world.ships.resize_with(count,Ship::default);self.fleet.resize_with(count,ShipCargo::default);
                self.ship_metadata.resize(count,(0xffff,0xff,0xffff,0xffff));self.expedition_images.resize_with(count,||None);self.expedition_original_images.resize_with(count,||None);
                let old=&self.navigation.as_ref().expect("validado").snapshot.ships;
                let activated=(0..count).filter(|&i|u16_at(&after.ships,i*0x180+0x134)<0x17&&(i>=before||u16_at(old,i*0x180+0x134)>=0x17)).collect::<Vec<_>>();
                for i in (before..count).chain(activated){
                    let image=&after.ships[i*0x180..(i+1)*0x180];
                    self.world.ships[i].owner=image[0];self.world.ships[i].state=u16_at(image,0x134);
                    self.ship_metadata[i]=(u16_at(image,4),u16_at(image,0x134),u16_at(image,0x15e),u16_at(image,8));
                    if u16_at(image,0x134)<0x17{self.expedition_images[i]=Some(image.to_vec())}
                    let cargo=&mut self.fleet[i];cargo.owner=image[0];cargo.town=image[0x39];cargo.kind=image[0xe];cargo.capacity=u32_at(image,0x10) as i32;cargo.dirty=u16_at(image,0x3c);cargo.crew=u16_at(image,0x40);cargo.cutlasses=u16_at(image,0x154);cargo.marines=image[0x156];for k in 0..24{cargo.stock[k]=u32_at(image,0x54+4*k) as i32;cargo.average_price[k]=f32::from_bits(u32_at(image,0xb4+4*k));cargo.guns[k]=image[0x13c+k]}cargo.cargo_value=u32_at(image,0x114) as i32;cargo.cargo_weight=u32_at(image,0x118) as i32;cargo.equipment_weight=u32_at(image,0x11c) as i32;cargo.gun_power=u32_at(image,0x120) as i32;
                }
                self.world.ship_count=count as u16;
                let old_cv=self.expedition_convoys.len();self.expedition_convoys.resize_with(after.convoys.len()/0x3c,||vec![0;0x3c]);self.convoy_flags.resize(after.convoys.len()/0x3c,0);
                for i in old_cv..self.expedition_convoys.len(){let image=&after.convoys[i*0x3c..(i+1)*0x3c];self.expedition_convoys[i].copy_from_slice(image);self.convoy_flags[i]=u32_at(image,0x18)}
                for (i,band) in bands.iter().enumerate(){if let Some(band)=band{
                    self.hunt_bands.as_mut().expect("validado")[i]=Some(p3_core::ai::settlement::MissionPirateBand{first_ship:u16_at(&band.header,4),target:band.header[6],status:band.header[12],reserve_strength:u16_at(&band.header,14)});
                    self.mission_pirate_targets.as_mut().expect("validado")[i]=Some(band.header[6]);
                }}
                *self.event_band_images=Some(bands);*self.pirates=Some(staged.portraits);*self.expedition_factory=Some(staged.factory);*self.navigation=Some(staged.navigation);
            }
            return u32::from(out.keep)
        }
        if call.kind==0x33 {let out=self.theft_preview(s).expect("validado");s.records[usize::from(s.head)]=out.task;self.world.towns=out.towns;self.stocks.clone_from_slice(&out.stocks);self.merchants.clone_from_slice(&out.merchants);*self.pool=out.pool;*self.scheduled_office_images=Some(out.offices);self.scheduled_office_updates.extend(out.updates);return 0}
        if call.kind==0x1d {
            let out=self.fire_preview(s).expect("validado");
            self.stocks.clone_from_slice(&out.stocks);self.merchants.clone_from_slice(&out.merchants);
            for update in &out.updates {
                let i=update.index;let image=&out.offices[i];
                self.burn_lists[i].next=u16_at(image,0x2ca);self.burn_lists[i].building_head=u16_at(image,0x2ce);
                let housing=&mut self.burn_housing[i];housing.next=u16_at(image,0x2ca);housing.houses_head=u16_at(image,0x2d0);housing.house_count=u16_at(image,0x2d4);
                for k in 0..3 {housing.capacity[k]=u16_at(image,0x2d8+2*k);housing.assigned[k]=u16_at(image,0x2de+2*k);housing.level[k]=image[0x2e4+k];}
                self.world.offices[i].next=u16_at(image,0x2ca);self.world.offices[i].first_facility=u16_at(image,0x2cc);
            }
            self.world.merchant_facility_count=out.businesses.records.len() as u16;
            self.world.merchant_facilities=out.businesses.records.iter().map(|r|p3_core::economy::world::MerchantFacility{employees:u16_at(r,4),town:r[7],next:u16_at(r,8)}).collect();
            *self.business_free=out.businesses.free;*self.employment_businesses=out.businesses.records;
            *self.scheduled_office_images=Some(out.offices);self.scheduled_office_updates.extend(out.updates);
            *s=out.scheduler;self.world.towns=out.towns;self.sites.clone_from_slice(&out.sites);*self.pool=out.pool;
            if !self.mission_changed_sites.contains(&out.city){self.mission_changed_sites.push(out.city)}self.fire_notifications.extend(out.notices);self.fire_windows.extend(out.windows);return u32::from(out.keep)
        }
        if call.kind==0x1c {let out=self.plague_preview(s).expect("validado");s.records[usize::from(s.head)]=out.task;self.world.towns=out.towns;for(b,f)in out.businesses.iter().zip(self.world.merchant_facilities.iter_mut()){f.employees=u16_at(b,4);f.next=u16_at(b,8)}*self.employment_businesses=out.businesses;*self.pool=out.pool;*self.scheduled_office_images=Some(out.offices);self.scheduled_office_updates.extend(out.updates);let factory=self.expedition_factory.as_mut().expect("validado");for (dst,src)in factory.captains.iter_mut().zip(out.captains.entries.chunks_exact(16)){dst.copy_from_slice(src)}factory.free_captain=out.captains.free_head;if let Some(nav)=self.navigation.as_mut(){nav.snapshot.captains=out.captains.entries;nav.snapshot.manager[0xe0..0xe2].copy_from_slice(&out.captains.free_head.to_le_bytes())}*self.pirates=out.pirates;self.plague_notifications.extend(out.notices);self.plague_windows.extend(out.windows);return u32::from(out.keep)}
        if call.kind==0x07 {match self.celebration_preview(s).expect("validado"){CelebrationPrepared::None=>{},CelebrationPrepared::Ai{city,town}=>self.world.towns[city]=town,CelebrationPrepared::Human{city,town,owner,merchant,office,stock,pool,notify,window}=>{self.world.towns[city]=town;self.merchants[owner]=merchant;self.stocks[office]=stock;*self.pool=pool;if notify{self.celebration_notifications.push((owner as u16,city as u8))}if window{self.celebration_windows.push((owner as u16,city as u8))}}}return 0}
        if call.kind==0x03 {
            let out=self.ten_day_preview(s).expect("validado");self.navigation.as_mut().expect("validado").snapshot.captains=out.state.captains;self.world.towns=out.state.towns;self.merchants.clone_from_slice(&out.state.merchants);*self.pool=out.state.message_pool;self.expedition_operations.extend(out.state.operations);*self.investigation_rng=out.state.rng;*self.celebration_state=out.state.celebration;*self.expedition_factory=Some(out.factory);*self.pirates=out.pirates;*s=out.state.scheduler;return 1;
        }
        if call.kind==0x0b {
            let prepared=self.ship_retry_preview(s).expect("validado");s.records[usize::from(s.head)]=prepared.task;
            if let Some(build)=prepared.build {self.publish_ship_build(build)}
            return u32::from(prepared.keep)
        }
        if call.kind==0x27 || call.kind==0x28 {
            let staged=if call.kind==0x27{self.retirement_preview(s)}else{self.offer_preview(s)}.expect("validado");let before=&self.navigation.as_ref().expect("validado").snapshot;
            for i in 0..staged.snapshot.ships.len()/0x180{let at=i*0x180;let old=&before.ships[at..at+0x180];let new=&staged.snapshot.ships[at..at+0x180];if old!=new{self.world.ships[i].owner=new[0];self.world.ships[i].state=u16_at(new,0x134);self.ship_metadata[i].0=u16_at(new,4);self.ship_metadata[i].1=u16_at(new,0x134);self.ship_metadata[i].3=u16_at(new,8);}}
            for i in 0..staged.convoy_images.len(){if self.expedition_convoys[i]!=staged.convoy_images[i]{self.convoy_flags[i]=u32_at(&staged.convoy_images[i],0x18)}}
            *self.expedition_docked=u16_at(&staged.snapshot.manager,0xe8);*self.expedition_head=staged.factory.convoy_head;*self.expedition_convoys=staged.convoy_images;
            let nav=self.navigation.as_mut().expect("validado");nav.snapshot=staged.snapshot;
            for (ship,convoy) in std::iter::once((nav.ship,&mut nav.convoy)).chain(nav.additional.iter_mut().map(|v|(v.ship,&mut v.convoy))){if *convoy!=None{let at=usize::from(ship)*0x180;if at+10<=nav.snapshot.ships.len(){let id=u16_at(&nav.snapshot.ships,at+8);if usize::from(id)>=nav.snapshot.convoys.len()/0x3c{*convoy=None}else{*convoy=Some(id)}}}}
            for(i,merchant)in staged.merchants.iter().enumerate().take(usize::from(self.world.merchant_count)){self.world.merchants[i].gold=i32::from_le_bytes(merchant.bytes[..4].try_into().expect("validado"));self.world.merchants[i].income=i32::from_le_bytes(merchant.bytes[0x4c8..0x4cc].try_into().expect("validado"));}self.merchants.clone_from_slice(&staged.merchants);*self.pool=staged.message_pool;*self.expedition_factory=Some(staged.factory);*self.expedition_images=staged.images;*self.pirates=staged.pirates;self.world.towns=staged.towns;s.records[usize::from(s.head)]=staged.task;return u32::from(staged.keep);
        }

        if call.kind==0x00 {
            let task=&mut s.records[usize::from(s.head)];
            let (keep,op)=p3_core::economy::spice::arrival(task,&mut self.world.towns,self.mission_town_ids,self.mission_locations.as_ref(),self.world.network_client!=0,self.operations_mode,self.investigation_rng).expect("validado");
            if let Some(op)=op{self.expedition_operations.push(op)}return u32::from(keep);
        }

        if call.kind==0x19 {
            let task=&mut s.records[usize::from(s.head)];
            let ops=p3_core::economy::loan_slots::refresh(task,&self.world.towns,self.world.merchant_count,self.world.network_client!=0,self.world.world_8a,self.expedition_operations.len().min(52) as u16,self.loan_names,self.investigation_rng).expect("validado");
            self.expedition_operations.extend(ops);return 1;
        }

        if call.kind==0x25 {
            let index=usize::from(s.head);let(city,task,town,site,keep)=crate::siege_daily::prepare_ruin(s.records[index],&self.world.towns,self.sites).expect("validado");
            s.records[index]=task;self.world.towns[city]=town;self.sites[city]=site;if !self.mission_changed_sites.contains(&city){self.mission_changed_sites.push(city)}return u32::from(keep)
        }
        if call.kind==0x26 {
            let index=usize::from(s.head);let (scheduler,city,task,town,site,office,keep)=crate::siege_daily::prepare_burn(s,s.head,&self.world.towns,self.sites,self.world.merchant_count,self.stocks,self.burn_lists,self.burn_housing).expect("validado");
            *s=scheduler;s.records[index]=task;self.world.towns[city]=town;self.sites[city]=site;
            if let Some((i,businesses,houses))=office {self.burn_lists[i].building_head=businesses;self.burn_housing[i].houses_head=houses;self.burn_updates.push((i,businesses,houses));}
            if !self.mission_changed_sites.contains(&city){self.mission_changed_sites.push(city)}
            return u32::from(keep)
        }
        if call.kind==0x2f {
            use p3_core::ai::town_development::{self,DevelopmentEffect};
            let index=usize::from(s.head);let mut task=s.records[index];let city=task.u32_at(0xc) as usize;
            let result=town_development::tick(&mut task,&mut self.world.towns,self.sites,self.merchants,self.world.merchant_count,self.world.network_client!=0,self.tick).expect("validado");
            if let DevelopmentEffect::AppointMayor{merchant,city}=result.effect {
                self.expedition_operations.push(p3_core::pump::Operation([0x46,u32::from(merchant),u32::from(city),0xcdcdcdcd,0xbadeaffe]));
            }
            if let DevelopmentEffect::PlaceLender{city}=result.effect {
                if let Some(op)=crate::lender::plan(&self.world.towns[usize::from(city)],&self.sites[usize::from(city)],self.world.merchant_count).expect("validado") {self.expedition_operations.push(op)}
            }
            if city<self.world.towns.len()&&!self.mission_changed_sites.contains(&city){self.mission_changed_sites.push(city)}
            s.records[index]=task;return u32::from(result.keep)
        }
        if call.kind==0x39 {
            let index=usize::from(s.head);let mut task=s.records[index];let city=usize::from(task.u8_at(8));
            let result=p3_core::buildings::wall_task::tick(&mut task,&self.world.towns,self.sites,self.tick).expect("validado");
            if let Some(point)=result.place {
                self.sites[city].place_town_defense_pieces(&mut self.world.towns[city],point,&[0,0x20,0x40,0x60,0x80,0xa0,0xc0,0xe0]);
                if !self.mission_changed_sites.contains(&city){self.mission_changed_sites.push(city)}
            }
            s.records[index]=task;return u32::from(result.keep)
        }
        if call.kind==0x32{if let Some((owner,merchant,notification,dialog))=self.mission_pirate_abort_preview(s).expect("validado"){self.merchants[owner]=merchant;if let Some(n)=notification{self.mission_notifications.push(n)}if dialog{self.mission_failure_ui.push((owner as u16,3))}return 0}if let Some((task,towns,sites,cities))=self.road_build_cached.take(){s.records[s.head as usize]=task;self.world.towns=towns;self.sites.clone_from_slice(&sites);for city in cities{if !self.mission_changed_sites.contains(&city){self.mission_changed_sites.push(city)}}return 1}if let Some(keep)=self.tick_mission_preview(s).expect("validado"){return u32::from(keep)}if let Some((task,owner,merchant))=self.mission_ai_failure_preview(s).expect("validado"){s.records[s.head as usize]=task;self.merchants[owner]=merchant;return 1}if let Some((mut effect,pool,other))=self.mission_human_failure_preview(s).expect("validado"){let kind=s.records[s.head as usize].u8_at(0xc);p3_core::ai::settlement::apply_human_failure_task(s,&mut effect).expect("validado");self.mission_notifications.push(effect.notification);if effect.dialog_requested{self.mission_failure_ui.push((effect.recipient as u16,kind))}self.merchants[effect.owner]=effect.merchant;if let Some((recipient,merchant))=other{self.merchants[recipient]=merchant}*self.pool=pool;return 1}if let Some((task,towns,owner,merchant,pool,cities))=self.mission_road_start_preview(s).expect("validado"){s.records[s.head as usize]=task;self.world.towns=towns;self.merchants[owner]=merchant;*self.pool=pool;for city in cities{if !self.mission_changed_towns.contains(&city){self.mission_changed_towns.push(city)}}return 1}if let Some((task,owner,merchant,pool))=self.mission_pirate_start_preview(s).expect("validado"){s.records[s.head as usize]=task;self.merchants[owner]=merchant;*self.pool=pool;return 1}let (effect,pool)=self.mission_completion_preview(s).expect("validado").expect("resolución validada");if let Some(n)=effect.notification{self.mission_notifications.push(n)}if effect.dialog_requested{self.mission_ui.push((effect.owner as u16,s.records[s.head as usize].u8_at(0xc)))}self.merchants[effect.owner]=effect.merchant;*self.pool=pool;return 0}
        if call.kind==0x05 {let r=&mut s.records[s.head as usize];if usize::from(r.u8_at(9))>=self.world.towns.len()||usize::from(r.u8_at(11))>=self.world.towns.len()||u16::from(r.u8_at(8))>=self.world.merchant_count{return 0}if r.u8_at(0x11)==0&&r.u8_at(0x10)>15{r.set_u8_at(0x11,1);return 0}}
        if call.kind==0x05 && s.records[s.head as usize].u8_at(0x11)!=0 {let mut r=s.records[s.head as usize];let office=self.investigation_office(&r).expect("validado");let stock=office.map(|i|&mut self.stocks[i].stock);let verdict=c10_investigation::verdict(&mut r,&mut self.world.towns,self.merchants,self.tick,stock).expect("validado");for city in verdict.towns{if !self.investigation_towns.contains(&city){self.investigation_towns.push(city)}}if let Some((i,town,kind,payload))=verdict.letter{let date=p3_core::calendar::Calendar::for_tick(self.tick);let enabled=self.enabled.get(i).copied().unwrap_or(true);if let Err(e)=p3_state::messages::add_hanse_verdict(self.pool,&mut self.merchants[i],date.day,date.month,date.year,town,kind,payload,enabled){self.errors.push(e)}if verdict.notify{self.investigation_notifications.push((i as u16,town))}}s.records[s.head as usize]=r;return u32::from(verdict.keep)}
        if call.kind==0x05 {let mut r=s.records[s.head as usize];let rng=self.investigation_rng.as_mut().expect("validado");let random=[rng.rand() as u16,rng.rand() as u16];if let Some((i,town,payload))=c10_investigation::begin(&mut r,self.world.towns.len(),self.merchants,random).expect("validado"){let date=p3_core::calendar::Calendar::for_tick(self.tick);let enabled=self.enabled.get(i).copied().unwrap_or(true);if let Err(e)=c10_investigation::publish_indictment(self.pool,&mut self.merchants[i],date,town,payload,enabled,r.u8_at(16)){self.errors.push(e)}self.investigation_notifications.push((i as u16,town));}s.records[s.head as usize]=r;return 1}
        if matches!(call.kind,0x23|0x24|0x34){let mut r=s.records[s.head as usize];let city=usize::from(r.u8_at(0x14));if city>=self.world.towns.len(){return 0}
            if r.u8_at(0x15)==1 {let (factory,merchants,record,towns,effects)=self.factory_preview(&r,s.head).expect("validado");if let Some(nav)=self.navigation.as_mut(){if let Err(e)=factory_nav::publish_factory(nav,self.expedition_factory.as_ref().expect("validado"),&factory){self.errors.push(format!("expedición {:#x}: {e}",call.kind));return 0}}let before=self.expedition_factory.as_ref().expect("validado");self.fleet.resize_with(factory.ships.len(),||ShipCargo{owner:0xff,..Default::default()});self.world.ships.resize(factory.ships.len(),Ship{owner:0xff,state:0xff});self.ship_metadata.resize(factory.ships.len(),(u16::MAX,0xff,0,u16::MAX));self.expedition_images.resize(factory.ships.len(),None);self.expedition_original_images.resize(factory.ships.len(),None);self.world.ship_count=factory.ships.len() as u16;self.expedition_convoys.resize_with(factory.convoys.len(),||vec![0;0x3c]);self.convoy_flags.resize(factory.convoys.len(),0);for i in 0..factory.ships.len(){if before.ships.get(i)!=Some(&factory.ships[i]) && u16_at(&factory.ships[i],0x134)<0x17{let image=&factory.ships[i];let mut cargo=ShipCargo{owner:image[0],town:image[0x39],kind:image[0xe],crew:u16_at(image,0x40),marines:image[0x156],cutlasses:u16_at(image,0x154),capacity:u32_at(image,0x10) as i32,dirty:u16_at(image,0x3c),..Default::default()};for ware in 0..24{cargo.stock[ware]=u32_at(image,0x54+4*ware) as i32;cargo.average_price[ware]=f32::from_bits(u32_at(image,0xb4+4*ware));cargo.guns[ware]=image[0x13c+ware]}self.fleet[i]=cargo;self.world.ships[i]=Ship{owner:image[0],state:u16_at(image,0x134)};self.ship_metadata[i]=(u16_at(image,4),u16_at(image,0x134),u16_at(image,0x15e),u16_at(image,8));self.expedition_images[i]=Some(image.clone());self.expedition_original_images[i]=Some(image.clone());}}
                for i in 0..factory.convoys.len(){if before.convoys.get(i)!=Some(&factory.convoys[i]){self.expedition_convoys[i]=factory.convoys[i].clone();self.convoy_flags[i]=u32_at(&factory.convoys[i],0x18)}}if let Some(pirates)=self.pirates.as_mut(){pirates.next=self.ship_metadata.iter().map(|m|m.0).collect();pirates.ship_captains=self.expedition_images.iter().map(|s|s.as_ref().map_or(u16::MAX,|v|u16_at(v,0x42))).collect();pirates.captains=factory.captains.clone()}self.merchants.clone_from_slice(&merchants);self.world.towns.clone_from(&towns);*self.expedition_head=factory.convoy_head;*self.expedition_factory=Some(factory);self.expedition_notifications.extend(effects.notifications);if effects.event_window{self.expedition_windows.push(city as u8)}if !self.expedition_towns.contains(&city){self.expedition_towns.push(city)}s.records[s.head as usize]=record;return 1}
            if r.u8_at(0x15)==0 {let scores=self.expedition_scores(&r).expect("validado");let letters=p3_core::economy::hanse_expedition::select(&mut r,self.merchants,&scores,self.pirates.as_ref()).expect("validado");let date=p3_core::calendar::Calendar::for_tick(self.tick);for (i,payload) in letters{let enabled=self.enabled.get(i).copied().unwrap_or(true);if let Err(e)=p3_state::messages::add_hanse_expedition_request(self.pool,&mut self.merchants[i],date.day,date.month,date.year,self.world.alderman_town,payload,enabled){self.errors.push(e)}}s.records[s.head as usize]=r;return 1}
            if r.u8_at(0x15)==8 {let step=self.hunt_idle_preview(&r).expect("validado");let record=self.apply_hunt_step(step);s.records[s.head as usize]=record;return 1}
            if r.u8_at(0x15)==9 {if let Some(step)=self.hunt_idle_preview(&r){let record=self.apply_hunt_step(step);s.records[s.head as usize]=record;return 1}}
            if r.u8_at(0x15)==10 {let p=self.hunt_return_preview(&r,s.head).expect("validado").expect("validado");if p.record.u32_at(8)==0{self.hunt_losses.remove(&usize::from(s.head));}let before=self.expedition_images.to_vec();for (i,image) in p.ships.into_iter().enumerate(){self.expedition_images[i]=image}for (i,(a,b)) in before.iter().zip(self.expedition_images.iter()).enumerate(){if a!=b{let image=b.as_ref().expect("validado");self.ship_metadata[i].1=u16_at(image,0x134);self.world.ships[i].state=u16_at(image,0x134);if !self.expedition_changed_ships.contains(&i){self.expedition_changed_ships.push(i)}}}self.expedition_convoys[p.id]=p.convoy;self.convoy_flags[p.id]=u32_at(&self.expedition_convoys[p.id],0x18);if !self.changed_convoys.contains(&p.id){self.changed_convoys.push(p.id)}if !self.expedition_changed_convoys.contains(&p.id){self.expedition_changed_convoys.push(p.id)}*self.pool=p.pool;self.merchants.clone_from_slice(&p.merchants);s.records[s.head as usize]=p.record;if !self.expedition_towns.contains(&city){self.expedition_towns.push(city)}return 0}
            if r.u8_at(0x15)==9 {let id=usize::from(u16::from_le_bytes([r.u8_at(0x16),r.u8_at(0x17)]));let convoy=self.expedition_convoys.get(id).map(Vec::as_slice);p3_core::economy::hanse_expedition::hunt_wait(&mut r,self.tick,convoy).expect("validado");s.records[s.head as usize]=r;return 1}
            if r.u8_at(0x15)==7 {let id=usize::from(u16::from_le_bytes([r.u8_at(0x16),r.u8_at(0x17)]));let before=self.expedition_images.to_vec();p3_core::economy::hanse_expedition::hunt_depart(&mut r,&mut self.expedition_convoys[id],self.expedition_images,self.world.ship_count).expect("validado");for (i,(a,b)) in before.iter().zip(self.expedition_images.iter()).enumerate(){if a!=b{let image=b.as_ref().expect("validado");self.ship_metadata[i].1=u16_at(image,0x134);self.world.ships[i].state=u16_at(image,0x134);if !self.expedition_changed_ships.contains(&i){self.expedition_changed_ships.push(i)}}}self.convoy_flags[id]=u32_at(&self.expedition_convoys[id],0x18);if !self.changed_convoys.contains(&id){self.changed_convoys.push(id)}if !self.expedition_changed_convoys.contains(&id){self.expedition_changed_convoys.push(id)}s.records[s.head as usize]=r;return 1}
            if r.u8_at(0x15)==5 {let count=u16::try_from(self.expedition_convoys.len()).expect("validado");let id=usize::from(u16::from_le_bytes([r.u8_at(0x16),r.u8_at(0x17)]));if id<usize::from(count) && p3_core::economy::hanse_expedition::convoy_is_active(id as u16,*self.expedition_head,self.expedition_convoys)==Some(false){r.due=r.due.wrapping_add(0x20);s.records[s.head as usize]=r;return 1}let letters=if id>=usize::from(count){p3_core::economy::hanse_expedition::finish_without_convoy(&r,&mut self.world.towns,self.merchants,count).expect("validado")}else{let before=self.expedition_images.to_vec();let result=p3_core::economy::hanse_expedition::finish_convoy(&r,&mut self.world.towns,self.merchants,&mut self.expedition_convoys[id],self.expedition_images,self.world.ship_count,self.expedition_docked).expect("validado");for (i,(a,b)) in before.iter().zip(self.expedition_images.iter()).enumerate(){if a!=b{let image=b.as_ref().expect("validado");self.ship_metadata[i].1=u16_at(image,0x134);self.ship_metadata[i].3=u16_at(image,8);if let Some(ship)=self.world.ships.get_mut(i){ship.state=u16_at(image,0x134)}if !self.expedition_changed_ships.contains(&i){self.expedition_changed_ships.push(i)}}}if !self.expedition_changed_convoys.contains(&id){self.expedition_changed_convoys.push(id)}result};let date=p3_core::calendar::Calendar::for_tick(self.tick);for (i,payload) in letters{let enabled=self.enabled.get(i).copied().unwrap_or(true);if let Err(e)=p3_state::messages::add_hanse_expedition_finished(self.pool,&mut self.merchants[i],date.day,date.month,date.year,self.world.alderman_town,payload,enabled){self.errors.push(e)}}if !self.expedition_towns.contains(&city){self.expedition_towns.push(city)}return 0}
            if r.u8_at(0x15)==2 {let letters=p3_core::economy::hanse_expedition::penalty(&mut r,&mut self.world.towns,self.merchants,self.tick,self.pirates.as_ref()).expect("validado");let date=p3_core::calendar::Calendar::for_tick(self.tick);for (i,payload) in letters {let enabled=self.enabled.get(i).copied().unwrap_or(true);if let Err(e)=p3_state::messages::add_hanse_expedition_penalty(self.pool,&mut self.merchants[i],date.day,date.month,date.year,self.world.alderman_town,payload,enabled){self.errors.push(e)}}s.records[s.head as usize]=r;if !self.expedition_towns.contains(&city){self.expedition_towns.push(city)}return 1}
            let ships=if r.u8_at(0x15)==4{Vec::new()}else{self.expedition_ships().expect("validado")};
            let ops=p3_core::economy::hanse_expedition::patrol(&mut r,&mut self.world.towns,self.merchants,&ships,self.world.ship_count,self.world.network_client!=0).expect("validado");
            s.records[s.head as usize]=r;self.expedition_operations.extend(ops);if !self.expedition_towns.contains(&city){self.expedition_towns.push(city)}return 1
        }
        if call.kind==0x21 {let r=&mut s.records[s.head as usize];if let Some(voters)=p3_core::economy::hanse_petition::start_vote(r,&self.world.towns,self.merchants){let seat=self.world.alderman_town;for &v in &voters{self.vote_notifications.push((v,seat))}if !voters.is_empty(){self.vote_windows.push(seat)}return 1}if p3_core::economy::hanse_petition::advance_vote(r,&self.world.towns){return 1}
            let fine=r.u8_at(0x12)>r.u8_at(0x13) && matches!(r.u8_at(0xe),3..=5);
            if let Some((keep,town,payload))=p3_core::economy::hanse_petition::finish_vote(r,&mut self.world.towns,self.world.alderman_town,self.tick){
                if fine {for city in 0..self.world.towns.len(){if !self.financial_towns.contains(&city){self.financial_towns.push(city)}}}
                let d=p3_core::calendar::Calendar::for_tick(self.tick);for (i,m) in self.merchants.iter_mut().enumerate(){let enabled=self.enabled.get(i).copied().unwrap_or(true);if let Err(e)=p3_state::messages::add_hanse_vote_result(self.pool,m,d.day,d.month,d.year,town,payload,enabled){self.errors.push(e)}}return u32::from(keep)
            }return 0
        }
        if call.kind==0x30 {
            let mut r=s.records[s.head as usize];if p3_core::economy::hanse_petition::is_terminal(&r){return 0}let alderman=self.world.alderman_merchant;
            if p3_core::economy::hanse_petition::is_ai_terminal(&r,alderman,self.merchants){return 0}
            if p3_core::economy::hanse_petition::can_ai(&r,alderman,self.merchants,&self.world.towns,self.world.alderman_town){
                let d=p3_core::calendar::Calendar::for_tick(self.tick);let event=if let Some(target)=p3_core::economy::hanse_petition::ai_announce(&mut r,alderman,self.merchants,&self.world.towns,self.world.alderman_town){s.records[s.head as usize]=r;Some((false,self.world.alderman_town,u32::from(target),0))}else{p3_core::economy::hanse_petition::ai_schedule(s,alderman,self.merchants,&self.world.towns,self.world.alderman_town,self.tick).map(|(t,a,b)|(true,t,a,b))};
                if let Some((vote,town,a,b))=event{let payload=if !vote{self.petition_payload(&mut r,alderman,false).map(|v|v.0)}else{None};for (i,m) in self.merchants.iter_mut().enumerate(){let enabled=self.enabled.get(i).copied().unwrap_or(true);let sent=if vote{p3_state::messages::add_hanse_vote(self.pool,m,d.day,d.month,d.year,town,a,b,enabled)}else{p3_state::messages::add_hanse_petition(self.pool,m,d.day,d.month,d.year,town,payload.expect("validado"),enabled)};if let Err(e)=sent{self.errors.push(e)}}return u32::from(!vote)}
                return 1
            }
            if let Some(send)=p3_core::economy::hanse_petition::finish(&r,alderman,self.merchants,&self.world.towns){
                if send {let d=p3_core::calendar::Calendar::for_tick(self.tick);let i=alderman as usize;let enabled=self.enabled.get(i).copied().unwrap_or(true);
                    if let Err(e)=p3_state::messages::add_hanse_reprimand(self.pool,&mut self.merchants[i],d.day,d.month,d.year,self.world.alderman_town,enabled){self.errors.push(e)}
                }return 0
            }
            if let Some(send)=p3_core::economy::hanse_petition::remind(&mut r,alderman,self.merchants){
                s.records[s.head as usize]=r;if send {let d=p3_core::calendar::Calendar::for_tick(self.tick);let i=alderman as usize;let enabled=self.enabled.get(i).copied().unwrap_or(true);
                    if let Err(e)=p3_state::messages::add_hanse_reminder(self.pool,&mut self.merchants[i],d.day,d.month,d.year,self.world.alderman_town,enabled){self.errors.push(e)}
                }return 1
            }
            if let Some(target)=p3_core::economy::hanse_petition::announce(&mut r,alderman,self.merchants){
                let d=p3_core::calendar::Calendar::for_tick(self.tick);
                let (payload,directed)=self.petition_payload(&mut r,alderman,true).expect("validado");s.records[s.head as usize]=r;
                for (i,m) in self.merchants.iter_mut().enumerate(){if directed && i!=usize::from(alderman){continue}let enabled=self.enabled.get(i).copied().unwrap_or(true);
                    if let Err(e)=p3_state::messages::add_hanse_petition(self.pool,m,d.day,d.month,d.year,self.world.alderman_town,payload,enabled){self.errors.push(e)}
                }
                if !directed{self.hanse_notifications.push((u16::from(alderman),self.world.alderman_town,target));}return 1
            }
            return 0
        }
        if call.kind==0x01 {
            let mut r=s.records[s.head as usize];if p3_core::economy::debt::is_terminal(&r) || p3_core::economy::debt::missing_debtor_is_terminal(&r,self.merchants){return 0}
            if p3_core::economy::debt::settle_negative_amount(&mut r,self.merchants){s.records[s.head as usize]=r;return 0}
            if let Some((merchant,town,a,b))=p3_core::economy::debt::alert_external(&mut r,self.merchants,s.head){
                s.records[s.head as usize]=r;let date=p3_core::calendar::Calendar::for_tick(self.tick);let i=merchant as usize;let enabled=self.enabled.get(i).copied().unwrap_or(true);
                if let Err(e)=p3_state::messages::add_debt_alert(self.pool,&mut self.merchants[i],date.day,date.month,date.year,town,a,b,enabled){self.errors.push(e)}return 1
            }
            if let Some((merchant,town,amount,original_tick))=p3_core::economy::debt::credit_external(&r,self.merchants){
                let date=p3_core::calendar::Calendar::for_tick(self.tick);let i=merchant as usize;let enabled=self.enabled.get(i).copied().unwrap_or(true);
                if let Err(e)=p3_state::messages::add_debt_credit(self.pool,&mut self.merchants[i],date.day,date.month,date.year,town,amount,original_tick,enabled){self.errors.push(e)}return 0
            }
            if let Some((merchant,town,amount,original_tick))=p3_core::economy::debt::notify_debtor(&mut r,self.merchants){
                s.records[s.head as usize]=r;let date=p3_core::calendar::Calendar::for_tick(self.tick);let i=merchant as usize;
                let enabled=self.enabled.get(i).copied().unwrap_or(true);
                if let Err(e)=p3_state::messages::add_debt_notice(self.pool,&mut self.merchants[i],date.day,date.month,date.year,town,amount,original_tick,enabled){self.errors.push(e)}
                return 1
            }
            if let Some((mut stocks,mut ships,indices))=self.debt_assets(&r){
                let first=self.merchants[usize::from(r.u8_at(0x11))].u16(0xe);let first=if first<self.world.ship_count{first}else{u16::MAX};
                let (keep,events)=p3_core::economy::debt::seize_two_cities(&mut r,self.merchants,&mut self.world.towns,&mut stocks,&mut ships,first,self.difficulty).expect("validado");
                for (city,index) in indices.iter().enumerate(){if let Some(i)=index{self.stocks[*i].stock=stocks[city].expect("validado");}}
                s.records[s.head as usize]=r;
                for e in events {let city=usize::from(e.town);if !self.changed_towns.contains(&city){self.changed_towns.push(city)}
                    if let Some(index)=e.ship_index{self.fleet[index]=ships[index].cargo.clone();if !self.changed_ships.contains(&index){self.changed_ships.push(index)}let convoy=usize::from(self.ship_metadata[index].3);if let Some(flags)=self.convoy_flags.get_mut(convoy){*flags|=4;if !self.changed_convoys.contains(&convoy){self.changed_convoys.push(convoy)}}}
                    if e.letter {let d=p3_core::calendar::Calendar::for_tick(self.tick);let i=usize::from(r.u8_at(0x11));let enabled=self.enabled.get(i).copied().unwrap_or(true);
                        if let Err(err)=p3_state::messages::add_debt_ship_seizure(self.pool,&mut self.merchants[i],d.day,d.month,d.year,e.town,e.ship_name,e.kind,e.settled,e.reported,enabled){self.errors.push(err)}
                    }
                    if e.notify{let debtor=u16::from(r.u8_at(0x11));self.debt_notifications.push((debtor,e.town));self.seizure_notifications.push((debtor,e.town,u8::from(e.kind!=1)));}
                }return u32::from(keep)
            }
            if let Some(mut ships)=self.debt_ships(&r){
                let city=usize::from(r.u8_at(0x14));let first=self.merchants[usize::from(r.u8_at(0x11))].u16(0xe);
                let result=p3_core::economy::debt::seize_ships(&mut r,self.merchants,&mut self.world.towns[city],&mut ships,first,self.difficulty).expect("validado");
                if let Some(index)=result.ship_index{self.fleet[index]=ships[index].cargo.clone();if !self.changed_ships.contains(&index){self.changed_ships.push(index)}let convoy=usize::from(self.ship_metadata[index].3);if let Some(flags)=self.convoy_flags.get_mut(convoy){*flags|=4;if !self.changed_convoys.contains(&convoy){self.changed_convoys.push(convoy)}}}
                s.records[s.head as usize]=r;
                if result.letter {let d=p3_core::calendar::Calendar::for_tick(self.tick);let i=usize::from(result.debtor);let enabled=self.enabled.get(i).copied().unwrap_or(true);
                    if let Err(e)=p3_state::messages::add_debt_ship_seizure(self.pool,&mut self.merchants[i],d.day,d.month,d.year,result.town,result.ship_name,result.kind,result.settled,result.reported,enabled){self.errors.push(e)}
                }
                if result.notify{self.debt_notifications.push((u16::from(result.debtor),result.town));self.seizure_notifications.push((u16::from(result.debtor),result.town,1));}
                return u32::from(result.keep)
            }
            if let Some(index)=self.debt_office(&r){
                let city=usize::from(r.u8_at(0x14));let result=p3_core::economy::debt::seize_one_office(&mut r,self.merchants,&mut self.world.towns[city],&mut self.stocks[index].stock,self.difficulty).expect("validado");
                s.records[s.head as usize]=r;
                if result.notify {let d=p3_core::calendar::Calendar::for_tick(self.tick);let i=usize::from(result.debtor);let enabled=self.enabled.get(i).copied().unwrap_or(true);
                    if let Err(e)=p3_state::messages::add_debt_seizure(self.pool,&mut self.merchants[i],d.day,d.month,d.year,result.town,result.kind,result.settled,result.reported,enabled){self.errors.push(e)}
                    self.debt_notifications.push((u16::from(result.debtor),result.town));self.seizure_notifications.push((u16::from(result.debtor),result.town,u8::from(result.kind==0)));
                }return u32::from(result.keep)
            }
            if let Some(result)=p3_core::economy::debt::settle_partial_cash(&mut r,self.merchants){
                s.records[s.head as usize]=r;
                if result.notify {let d=p3_core::calendar::Calendar::for_tick(self.tick);let i=result.debtor as usize;let enabled=self.enabled.get(i).copied().unwrap_or(true);
                    if let Err(e)=p3_state::messages::add_debt_partial(self.pool,&mut self.merchants[i],d.day,d.month,d.year,result.town,result.reported,enabled){self.errors.push(e)}
                    self.debt_notifications.push((u16::from(result.debtor),result.town));
                }return u32::from(result.keep)
            }
            if let Some((merchant,town,amount))=p3_core::economy::debt::settle_cash(&mut r,self.merchants){
                s.records[s.head as usize]=r;
                let date=p3_core::calendar::Calendar::for_tick(self.tick);let i=merchant as usize;
                let enabled=self.enabled.get(i).copied().unwrap_or(true);
                if let Err(e)=p3_state::messages::add_debt_cash(self.pool,&mut self.merchants[i],date.day,date.month,date.year,town,amount,enabled){self.errors.push(e)}
            }
            return 0
        }
        let snapshot=self.merchants.to_vec();let mut towns=std::mem::take(&mut self.world.towns);
        let result=if call.kind==0x31 {let offices=self.offices.to_vec();let mut merchants=std::mem::take(&mut self.merchants);let value=p3_core::economy::extra_tax::collect(s,&mut towns,&mut merchants,&offices,self.tick,self);self.merchants=merchants;if let Err(e)=value{self.errors.push(e)}0}else{p3_core::economy::council_meeting::run(s,&mut towns,&snapshot,self.tick,self)};
        self.world.towns=towns;
        let date=p3_core::calendar::Calendar::for_tick(self.tick);
        for (kind,m,t,a,b) in std::mem::take(&mut self.letters) {
            let i=m as usize;let enabled=self.enabled.get(i).copied().unwrap_or(true);
            if let Some(merchant)=self.merchants.get_mut(i) {
                let sent=if kind==0x6c{p3_state::messages::add_extra_tax(self.pool,merchant,date.day,date.month,date.year,t,a,b,enabled)}else{p3_state::messages::add_council_decision(self.pool,merchant,date.day,date.month,date.year,t,a,b,enabled)};if let Err(e)=sent{self.errors.push(e);}
            }
        }
        result
    }
    fn release_mission_script(&mut self,_:u32){unreachable!()}
    fn release_task_object(&mut self,_:u32){unreachable!()}
    fn merchant_count(&self)->u16{self.world.merchant_count}
    fn merchant_task_1a(&mut self,_:&mut p3_core::scheduler::Scheduler,merchant:u16){self.merchants[usize::from(merchant)].update_sailor_pools(&self.world.towns)}
    fn world_tick(&self)->u32{self.tick}
    fn is_network_client(&self)->bool{self.world.network_client!=0}
    fn task_2c_recompute(&mut self,s:&mut p3_core::scheduler::Scheduler){let task=self.ai_build_interval_preview(s).expect("validado");s.records[usize::from(s.head)]=task;}
    fn task_2c_step(&mut self,_:&mut p3_core::scheduler::Scheduler){let(op,rng)=self.ai_construction_preview().expect("validado");if let Some(op)=op{self.expedition_operations.push(op)}*self.investigation_rng=rng;}
    fn hotseat_gate(&self)->u32{self.reminder_gate.unwrap_or(0)}
    fn clear_merchant_bit_608(&mut self,merchant:u32,bit:u32){
        let record=&mut self.merchants[merchant as usize];
        record.set_i32(0x608,(record.i32(0x608) as u32 & !(1u32 << (bit&31))) as i32);
    }
}

/// Comprueba el recorrido antes de aplicar avances que no finalizan edificios.
/// Las finalizaciones necesitan la transacción del pool vivo y se dejan al anfitrión.
fn progress_only(site:&mut Site,town:&mut Town,workers:u16)->Result<bool,String>{
    use p3_core::buildings::advance::{AdvanceCtx,AdvanceHost};
    struct NoCompletion;
    impl AdvanceHost for NoCompletion {
        fn find_office(&mut self,_:u32,_:u32,_:u32)->bool{unreachable!()}
        fn swap_office_head(&mut self,_:usize,_:u16)->u16{unreachable!()}
        fn recount_office(&mut self){unreachable!()}
        fn recount_buildings(&mut self){unreachable!()}
        fn set_progress(&mut self,_:u32,_:u32,_:u32,_:u32,_:u32){unreachable!()}
        fn add_message(&mut self,_:u32,_:u8,_:u8,_:u16,_:u16){unreachable!()}
    }
    let count=usize::from(town.u16(0x776));
    if site.records.len()!=count*8{return Err("registros de obra truncados".into())}
    let mut seen=vec![false;count];let mut i=usize::from(town.u16(0x762));let mut budget=u32::from(workers);
    while budget>0 && i<count {
        if seen[i]{return Err("ciclo en obras".into())}seen[i]=true;
        let r=&site.records[8*i..8*i+8];let next=usize::from(u16_at(r,4));
        if town.flags()&0x10!=0 {
            let at=(town.u32(0x7ac) as usize).checked_mul(r[0] as usize).and_then(|x|x.checked_add(r[1] as usize)).ok_or("celda fuera del plano")?;
            if *site.cells.get(at).ok_or("celda fuera del plano")?<0x80 {i=next;continue}
        }
        if town.flags()&0x800!=0 && r[3]==0x30 {i=next;continue}
        let progress=u32::from(r[6]);if progress<=5 && progress<=budget{return Ok(false)}
        budget=budget.saturating_sub(5);i=next;
    }
    let mut host=NoCompletion;let mut cx=AdvanceCtx{merchant_count:0,visual_owner:None,host:&mut host};
    site.advance_construction(town,&mut cx,u32::from(workers));Ok(true)
}

/// Aplica la fase de clima tras el tick naval, manteniendo las rutas en la
/// copia transaccional que publica el anfitrión al completar la fase.
fn advance_navigation_weather(staged:&mut navigation::LiveVoyage,planner:Option<&p3_core::route::finder::RoutePlanner>,tick:u32)->Result<p3_core::ships::weather::WeatherEffects,String>{
    if tick&7==0 {
                        let manager=&staged.snapshot.manager;
                        if manager.len()<0xf6 {Err("gestor naval truncado".to_string())} else {
                            let count=u16::from_le_bytes([manager[0xf4],manager[0xf5]]);
                            let overlap=(0..16).any(|i|{let at=0x20+i*12;let id=u16::from_le_bytes([manager[at],manager[at+1]]);let ship=usize::from(id)*0x180;id<count && staged.snapshot.ships.get(ship+0x134..ship+0x136)==Some(&[0x14,0][..])});
                            if overlap {Err("fenómeno meteorológico coincide con barco en batalla".to_string())} else {
                                // Las rutas S02 pueden existir con puntero lógico cero.
                                // Copiamos los puntos para que un error posterior revierta todo.
                                let mut routes=(0..usize::from(count)).map(|i|{
                                    let saved=staged.snapshot.routes.get(i).and_then(Option::as_ref);
                                    let present=matches!(saved,Some(p3_save::ships::SavedRoute::Owned(_)|p3_save::ships::SavedRoute::Shared{..}));
                                    let points=staged.route_for(i as u16).filter(|p|!p.is_empty()).map(|p|p.to_vec()).or_else(||match saved{Some(p3_save::ships::SavedRoute::Owned(p))=>Some(p.clone()),_=>None});
                                    (saved.is_some(),present,points)
                                }).collect::<Vec<_>>();
                                let empty=p3_core::route::planner::NavGrid{cells:Vec::new(),width:0,height:0,scale:3536};
                                let grid=planner.map(|p|&p.nav.grid).unwrap_or(&empty);let player=staged.player;
                                let mut changed=Vec::new();let mut notices=Vec::new();
                                let mut replan=|id:u16,ship:&mut [u8],hazard:(i16,i16)|->Result<(),&'static str>{
                                    let (known,present,points)=routes.get_mut(usize::from(id)).ok_or("ruta meteorológica inexistente")?;
                                    let original:[u8;4]=ship[0x24..0x28].try_into().expect("barco validado");
                                    if !*present && original==[0;4]{return if *known{Ok(())}else{Err("estado de ruta meteorológica desconocido")}}
                                    let route=points.as_mut().ok_or("ruta meteorológica de capitán no reconstruida")?;
                                    if original==[0;4]{ship[0x24..0x28].copy_from_slice(&(0x10000+u32::from(id)*16).to_le_bytes())}
                                    let effect=p3_core::ships::weather::replan_around_hazard(ship,route,hazard,id,player,grid)?;
                                    if original==[0;4] && effect.is_none(){ship[0x24..0x28].fill(0)}
                                    if let Some(effect)=effect{if !changed.contains(&id){changed.push(id)}if effect.notify_player{notices.push(id)}}
                                    Ok(())
                                };
                                match p3_core::ships::weather::tick_weather_with_captains(&mut staged.snapshot.manager,&mut staged.snapshot.ships,&mut staged.snapshot.convoys,&mut replan){
                                    Ok(effect)=>{for id in changed{let points=routes[usize::from(id)].2.take().expect("ruta cambiada");staged.snapshot.routes[usize::from(id)]=Some(p3_save::ships::SavedRoute::Owned(points.clone()));if staged.ship==id{staged.points=points}else if let Some(v)=staged.additional.iter_mut().find(|v|v.ship==id){v.points=points}}for id in notices{staged.events.push(format!("ruta:{id}"))}Ok(effect)},Err(e)=>Err(e.to_string())
                                }
                            }
                        }
                    }else{Ok(p3_core::ships::weather::WeatherEffects{redraw:false})}
}

impl NullHost {
    /// Tick naval por copias: todo se calcula sobre copias y se confirma al final. Queda para el tick diario de
    /// cuadrillas piratas (0x6f), que puede ampliar la flota y su base.
    fn ships_tick_with_images_staged(&mut self,w:&mut EconomyWorld,m:&mut [RepMerchant],cargo:&mut Vec<ShipCargo>,images:&mut [u8],tick:u32)->bool {
        if self.navigation_error.is_some()||self.encounter_battle.is_some(){return false}
        let bands=(||->Result<Vec<Option<p3_save::pirate_bands::PirateBand>>,String>{
            let Some(live)=&self.pirate_band_images else{return Ok(vec![None;5])};
            if live.len()!=5{return Err("bandas navales incompletas".into())}
            live.iter().enumerate().map(|(i,b)|if b.is_some(){self.reconcile_pirate_band_image(i).map(Some)}else{Ok(None)}).collect()
        })();
        let bands=match bands{Ok(v)=>v,Err(e)=>{self.ship_office_sync_error(&e);return false}};
        let mut absorption=convoy_absorption::AbsorptionState{bands,day:w.day_of_year,network_client:w.network_client,operations:Vec::new()};
        let mut expeditions=navigation::ExpeditionPlanning{lock:w.world_8a,countdown:self.naval_expedition_countdown,network_client:w.network_client!=0,trade_centers:self.trade_centers.clone().unwrap_or_default(),locations:self.discovery_locations.clone(),map_points:self.expedition_map_points.clone(),rng:self.investigation_rng,scheduler:self.council_scheduler.clone()};
        // Recursos y versiones se toman antes del préstamo mutable de navegación.
        let group_names=self.pirate_captain_name_counts();let group_pool=self.route_stop_runtime.clone();
        let group_resources=self.expedition_resources.clone();let group_locations=self.settlement_locations;
        let group_thresholds=self.pirate_group_thresholds;let group_initial=self.merchant_ship_groups.clone();
        let mut before=std::mem::take(&mut self.nav_scratch.before);
        let Some(nav)=self.navigation.as_mut() else{return false};let Some(ai)=self.navigation_ai.as_mut() else{return false};
        before.capture(&nav.snapshot);let mut staged_ai=ai.clone();staged_ai.offices=images.to_vec();
        staged_ai.convoy_plan=match (self.navigation_planner.as_ref(),self.settlement_inland,self.pirate_targets){
            (Some(p),Some(inland),Some(objects)) if p.town_count as usize==w.towns.len()=>Some(p3_core::ships::convoy_plan::PlanEnv{
                difficulty:self.pirate_difficulty,g672cc8:9,merchant_count:w.merchant_count,clock:u32::from(w.day_of_year),town_count:w.towns.len() as u32,time:tick,scale:p.nav.grid.scale,slots:p.town_slots.clone(),slot_flag:inland.to_vec(),order:p.town_order.clone(),town_flags:w.towns.iter().map(|t|t.u32(0x2c8)).collect(),objects,route_counts:p.sea_counts.clone(),route_points:p.sea_points.iter().map(|row|row.iter().map(|&(x,y)|u32::from(x as u16)|(u32::from(y as u16)<<16)).collect()).collect()
            }),_=>None
        };
        let mut staged=nav.clone();let mut towns=w.towns.clone();let mut merchants=m.to_vec();let mut planner=self.navigation_planner.clone();
        let options=self.encounter_config.as_ref().map(|config|navigation::PirateEncounterOptions{slot:config.slot,battle_address:config.battle_address,side0_pirate_band:false,scheduler:Some(&self.council_scheduler),tile_width:self.battle_tile_width,human_as_ai:self.headless_human_battles});
        let mut group_result=None;
        let result=(||->Result<_,String>{
            if tick as u8==0x6f && absorption.bands.iter().any(Option::is_some){
                let pool=group_pool.as_ref().ok_or("depósito vivo de paradas ausente")?;
                let group=pirate_group_tick_stage::run(&staged,&absorption,&towns,&merchants,pool,&group_initial,pirate_group_tick_stage::Inputs{tick,day:w.day_of_month,day_of_year:w.day_of_year,player:w.local_merchant,alderman_town:w.alderman_town,world_lock:w.world_8a,network:w.network_client,thresholds:group_thresholds,names:group_names,resources:group_resources.as_ref(),locations:group_locations.as_ref()})?;
                staged=group.nav.clone();towns=group.towns.clone();merchants=group.merchants.clone();absorption=group.bands.clone();group_result=Some(group);
            }
            if staged.snapshot.ships.len()>before.ships.len()||staged.snapshot.captains.len()>before.captains.len(){
                if self.expedition_factory.is_none()||staged.snapshot.ships.len()%0x180!=0||staged.snapshot.captains.len()%16!=0||staged.snapshot.routes.len()!=staged.snapshot.ships.len()/0x180{return Err("vistas para crecimiento pirata incompletas".into())}
            }
            // Todo lo que toca el paso es copia local de este tick (se descarta entera con `Err`): no hace falta otra copia de respaldo.
            let battle=staged.step_with_ai_bands_and_expeditions_staged(&mut towns,&mut merchants,tick,planner.as_mut(),&mut staged_ai,options,&mut absorption,&mut expeditions)?;
            advance_navigation_weather(&mut staged,planner.as_ref(),tick).map(|effect|(battle,effect.redraw))
        })();
        match result{
            Ok((battle,redraw))=>{
                // La fábrica escribe primero las ranuras ampliadas en S02;
                // la base naval se extiende con esa misma versión confirmada.
                let old_count=before.ships.len()/0x180;let count=staged.snapshot.ships.len()/0x180;
                if count>old_count||staged.snapshot.captains.len()>before.captains.len(){
                    for i in old_count..count{let image=&staged.snapshot.ships[i*0x180..(i+1)*0x180];staged.baseline.ships.extend_from_slice(image);staged.baseline.routes.push(staged.snapshot.routes[i].clone());let mut old=vec![0;0x180];old[0x134..0x136].copy_from_slice(&0xffu16.to_le_bytes());before.ships.extend_from_slice(&old);}
                    staged.baseline.manager[0xf4..0xf6].copy_from_slice(&(count as u16).to_le_bytes());
                    w.ship_count=count as u16;w.ships.resize_with(count,||p3_core::economy::world::Ship{owner:255,state:255});cargo.resize_with(count,ShipCargo::default);
                    self.expedition_ship_images.resize_with(count,||None);self.expedition_original_ship_images.resize_with(count,||None);self.debt_ship_metadata.resize(count,(u16::MAX,255,0,u16::MAX));
                    if let Some(factory)=self.expedition_factory.as_mut(){
                        for image in staged.snapshot.ships.chunks_exact(0x180).skip(factory.ships.len()){factory.ships.push(image.to_vec())}
                        for image in staged.snapshot.captains.chunks_exact(16).skip(factory.captains.len()){factory.captains.push(image.try_into().expect("capitán completo"))}
                    }
                }
                if self.pirate_band_images.is_some(){
                    if let Some(missions)=self.mission_pirates.as_mut(){for (m,b) in missions.iter_mut().zip(&absorption.bands){match (m,b){(slot,None)=>*slot=None,(Some(m),Some(b))=>{m.first_ship=u16::from_le_bytes([b.header[4],b.header[5]]);m.target=b.header[6];m.status=b.header[12];m.reserve_strength=u16::from_le_bytes([b.header[14],b.header[15]]);},(None,Some(_))=>{}}}}
                    self.pirate_band_images=Some(absorption.bands.clone());
                }
                if let Some(group)=group_result{
                    self.route_stop_runtime=Some(group.pool);self.merchant_ship_groups=group.groups;for id in group.changed_groups{if !self.merchant_ship_groups_changed.contains(&id){self.merchant_ship_groups_changed.push(id)}}
                    if let Some(targets)=self.pirate_targets.as_mut(){for (dst,band) in targets.iter_mut().zip(&absorption.bands){*dst=band.as_ref().map(|b|b.header[6]);}}
                    if let Some(p)=self.hanse_pirates.as_mut(){
                        for (dst,band) in p.heads.iter_mut().zip(&absorption.bands){*dst=band.as_ref().map(|b|u16::from_le_bytes([b.header[4],b.header[5]]));}
                        p.next=staged.snapshot.ships.chunks_exact(0x180).map(|s|u16_at(s,4)).collect();
                        p.ship_captains=staged.snapshot.ships.chunks_exact(0x180).map(|s|u16_at(s,0x42)).collect();
                        p.captains=staged.snapshot.captains.chunks_exact(16).map(|c|c.try_into().expect("capitán completo")).collect();
                    }
                }
                self.council_operations.extend(absorption.operations);self.investigation_rng=expeditions.rng;self.council_scheduler=expeditions.scheduler;
if redraw{staged.events.push("clima:redraw".into())}w.towns=towns;m.clone_from_slice(&merchants);*nav=staged;self.navigation_planner=planner;self.encounter_battle=battle;images.copy_from_slice(&staged_ai.offices);staged_ai.offices.clear();*ai=staged_ai;for (a,b) in w.merchants.iter_mut().zip(m.iter()){a.gold=b.i32(0);a.income=b.i32(0x4c8)}self.apply_navigation_ship_lost_letters(w,m,tick);self.publish_navigation_delta_from(&before.manager,&before.ships,&before.captains,&before.convoys,w,cargo);self.nav_scratch.before=before;true},
            Err(e)=>{self.navigation_error=Some(e.clone());self.council_errors.push(format!("navegación: {e}"));false}
        }
    }
    /// Tick naval sobre el estado vivo. Hace lo mismo que `ships_tick_with_images_staged`, pero el paso escribe
    /// directamente en la flota, las ciudades, los comerciantes y el planificador en vez de en copias que luego se
    /// confirman. Para deshacer un tick fallido guarda solo lo imprescindible: los bloques de la flota (que la
    /// publicación necesita de todos modos), ciudades y comerciantes en memoria reutilizada, las listas, y las rutas
    /// y puntos únicamente si el paso llega a cambiarlos (`VoyageUndo`). Con error el estado queda como antes del tick.
    fn ships_tick_with_images_live(&mut self,w:&mut EconomyWorld,m:&mut [RepMerchant],cargo:&mut Vec<ShipCargo>,images:&mut Vec<u8>,tick:u32)->bool {
        if self.navigation_error.is_some()||self.encounter_battle.is_some(){return false}
        let bands=(||->Result<Vec<Option<p3_save::pirate_bands::PirateBand>>,String>{
            let Some(live)=&self.pirate_band_images else{return Ok(vec![None;5])};
            if live.len()!=5{return Err("bandas navales incompletas".into())}
            live.iter().enumerate().map(|(i,b)|if b.is_some(){self.reconcile_pirate_band_image(i).map(Some)}else{Ok(None)}).collect()
        })();
        let bands=match bands{Ok(v)=>v,Err(e)=>{self.ship_office_sync_error(&e);return false}};
        let mut absorption=convoy_absorption::AbsorptionState{bands,day:w.day_of_year,network_client:w.network_client,operations:Vec::new()};
        let plan=match (self.navigation_planner.as_ref(),self.settlement_inland,self.pirate_targets){
            (Some(p),Some(inland),Some(objects)) if p.town_count as usize==w.towns.len()=>Some(p3_core::ships::convoy_plan::PlanEnv{
                difficulty:self.pirate_difficulty,g672cc8:9,merchant_count:w.merchant_count,clock:u32::from(w.day_of_year),town_count:w.towns.len() as u32,time:tick,scale:p.nav.grid.scale,slots:p.town_slots.clone(),slot_flag:inland.to_vec(),order:p.town_order.clone(),town_flags:w.towns.iter().map(|t|t.u32(0x2c8)).collect(),objects,route_counts:p.sea_counts.clone(),route_points:p.sea_points.iter().map(|row|row.iter().map(|&(x,y)|u32::from(x as u16)|(u32::from(y as u16)<<16)).collect()).collect()
            }),_=>None
        };
        let planner_backup=self.navigation_planner.clone();
        let Some(nav)=self.navigation.as_mut() else{return false};let Some(ai)=self.navigation_ai.as_mut() else{return false};
        // Tablas que el paso solo lee: se prestan al contexto de expediciones y se devuelven al acabar.
        let centers=self.trade_centers.is_some();
        let mut expeditions=navigation::ExpeditionPlanning{lock:w.world_8a,countdown:self.naval_expedition_countdown,network_client:w.network_client!=0,trade_centers:self.trade_centers.take().unwrap_or_default(),locations:std::mem::take(&mut self.discovery_locations),map_points:std::mem::take(&mut self.expedition_map_points),rng:self.investigation_rng,scheduler:self.council_scheduler.clone()};
        let mut scratch=std::mem::take(&mut self.nav_scratch);
        scratch.before.capture(&nav.snapshot);scratch.capture_world(&w.towns,m);
        let lists_backup=nav.snapshot.lists.clone();let events_len=nav.events.len();let mut undo=None;
        // Las imágenes de oficina se intercambian con la memoria de la IA mientras dura el paso (sin copiarlas).
        std::mem::swap(&mut ai.offices,images);let old_plan=std::mem::replace(&mut ai.convoy_plan,plan);
        let options=self.encounter_config.as_ref().map(|config|navigation::PirateEncounterOptions{slot:config.slot,battle_address:config.battle_address,side0_pirate_band:false,scheduler:Some(&self.council_scheduler),tile_width:self.battle_tile_width,human_as_ai:self.headless_human_battles});
        let planner=&mut self.navigation_planner;
        let result=match nav.step_with_ai_bands_and_expeditions_live(&mut w.towns,m,tick,planner.as_mut(),ai,options,&mut absorption,&mut expeditions,&mut undo){
            Ok(battle)=>advance_navigation_weather(nav,planner.as_ref(),tick).map(|effect|(battle,effect.redraw)),
            Err(e)=>Err(e),
        };
        self.trade_centers=centers.then(||std::mem::take(&mut expeditions.trade_centers));self.discovery_locations=std::mem::take(&mut expeditions.locations);self.expedition_map_points=std::mem::take(&mut expeditions.map_points);
        match result{
            Ok((battle,redraw))=>{
                let before=&mut scratch.before;
                // La fábrica escribe primero las ranuras ampliadas en S02;
                // la base naval se extiende con esa misma versión confirmada.
                let old_count=before.ships.len()/0x180;let count=nav.snapshot.ships.len()/0x180;
                if count>old_count||nav.snapshot.captains.len()>before.captains.len(){
                    for i in old_count..count{let image=&nav.snapshot.ships[i*0x180..(i+1)*0x180];nav.baseline.ships.extend_from_slice(image);nav.baseline.routes.push(nav.snapshot.routes[i].clone());let mut old=vec![0;0x180];old[0x134..0x136].copy_from_slice(&0xffu16.to_le_bytes());before.ships.extend_from_slice(&old);}
                    nav.baseline.manager[0xf4..0xf6].copy_from_slice(&(count as u16).to_le_bytes());
                    w.ship_count=count as u16;w.ships.resize_with(count,||p3_core::economy::world::Ship{owner:255,state:255});cargo.resize_with(count,ShipCargo::default);
                    self.expedition_ship_images.resize_with(count,||None);self.expedition_original_ship_images.resize_with(count,||None);self.debt_ship_metadata.resize(count,(u16::MAX,255,0,u16::MAX));
                    if let Some(factory)=self.expedition_factory.as_mut(){
                        for image in nav.snapshot.ships.chunks_exact(0x180).skip(factory.ships.len()){factory.ships.push(image.to_vec())}
                        for image in nav.snapshot.captains.chunks_exact(16).skip(factory.captains.len()){factory.captains.push(image.try_into().expect("capitán completo"))}
                    }
                }
                if self.pirate_band_images.is_some(){
                    if let Some(missions)=self.mission_pirates.as_mut(){for (m,b) in missions.iter_mut().zip(&absorption.bands){match (m,b){(slot,None)=>*slot=None,(Some(m),Some(b))=>{m.first_ship=u16::from_le_bytes([b.header[4],b.header[5]]);m.target=b.header[6];m.status=b.header[12];m.reserve_strength=u16::from_le_bytes([b.header[14],b.header[15]]);},(None,Some(_))=>{}}}}
                    self.pirate_band_images=Some(absorption.bands.clone());
                }
                self.council_operations.extend(absorption.operations);self.investigation_rng=expeditions.rng;self.council_scheduler=expeditions.scheduler;
                if redraw{nav.events.push("clima:redraw".into())}self.encounter_battle=battle;std::mem::swap(&mut ai.offices,images);ai.offices.clear();drop(old_plan);
                for (a,b) in w.merchants.iter_mut().zip(m.iter()){a.gold=b.i32(0);a.income=b.i32(0x4c8)}
                self.apply_navigation_ship_lost_letters(w,m,tick);
                self.nav_scratch.changed=std::mem::take(&mut scratch.changed);
                {let b=&scratch.before;self.publish_navigation_delta_from(&b.manager,&b.ships,&b.captains,&b.convoys,w,cargo);}
                scratch.changed=std::mem::take(&mut self.nav_scratch.changed);self.nav_scratch=scratch;true},
            Err(e)=>{
                // Deshacer: la flota vuelve a los bloques guardados; rutas y puntos solo si el paso llegó a tocarlos.
                let b=&scratch.before;
                nav.snapshot.manager.clear();nav.snapshot.manager.extend_from_slice(&b.manager);nav.snapshot.ships.clear();nav.snapshot.ships.extend_from_slice(&b.ships);
                nav.snapshot.captains.clear();nav.snapshot.captains.extend_from_slice(&b.captains);nav.snapshot.convoys.clear();nav.snapshot.convoys.extend_from_slice(&b.convoys);
                nav.snapshot.lists=lists_backup;
                if let Some(u)=undo{nav.snapshot.routes=u.routes;nav.points=u.points;nav.additional=u.additional;nav.convoy=u.convoy}
                nav.events.truncate(events_len);
                scratch.restore_world(&mut w.towns,m);
                ai.convoy_plan=old_plan;std::mem::swap(&mut ai.offices,images);ai.offices.clear();
                self.navigation_planner=planner_backup;self.nav_scratch=scratch;
                self.navigation_error=Some(e.clone());self.council_errors.push(format!("navegación: {e}"));false}
        }
    }
}
impl TownHost for NullHost {
    fn needs_scheduled_office_images(&self,tick:u32)->bool{let q=&self.council_scheduler;let mut at=q.head;let mut seen=vec![false;usize::from(q.capacity)];while at<q.capacity{let i=usize::from(at);if seen[i]{return true}seen[i]=true;let Some(task)=q.records.get(i)else{return true};if task.due>tick{return false}if matches!(task.kind,2|3|4|7|9|0x0a|0x0b|0x0c|0x0e|0x0f|0x13|0x15|0x1b|0x1c|0x1d|0x1e|0x1f|0x20|0x2a|0x2b|0x2c|0x33|0x36){return true}at=task.next;}false}
    fn set_scheduled_office_images(&mut self,images:Vec<Vec<u8>>){self.scheduled_office_images=Some(images)}
    fn take_scheduled_office_updates(&mut self)->Vec<p3_core::economy::world::ScheduledOfficeUpdate>{std::mem::take(&mut self.scheduled_office_updates)}
    fn interrupt_before_sea_battle(&self)->bool {self.encounter_battle.is_some() || self.navigation_error.is_some()}
    fn ship_ai_enabled(&self)->bool {self.navigation_ai.is_some() && self.navigation.is_some()}
    fn ship_office_sync_error(&mut self,error:&str){self.navigation_error=Some(error.into());self.council_errors.push(format!("navegación: {error}"));}
    fn ships_tick_with_images(&mut self,w:&mut EconomyWorld,m:&mut [RepMerchant],cargo:&mut Vec<ShipCargo>,images:&mut [u8],tick:u32)->bool {
        // El paso diario de cuadrillas piratas (tick 0x6f con alguna banda viva) conserva el camino por copias.
        if tick as u8==0x6f&&self.pirate_band_images.as_ref().is_some_and(|v|v.iter().any(Option::is_some)){self.ships_tick_with_images_staged(w,m,cargo,images,tick)}
        else{
            // Entrada con bloque prestado: el tick vivo necesita un `Vec` propio; con error `images` queda intacto.
            let mut owned=images.to_vec();let ok=self.ships_tick_with_images_live(w,m,cargo,&mut owned,tick);if ok{images.copy_from_slice(&owned)}ok
        }
    }
    fn ships_tick_with_office_images(&mut self,w:&mut EconomyWorld,m:&mut [RepMerchant],cargo:&mut Vec<ShipCargo>,images:&mut Vec<u8>,tick:u32)->bool {
        if tick as u8==0x6f&&self.pirate_band_images.as_ref().is_some_and(|v|v.iter().any(Option::is_some)){self.ships_tick_with_images_staged(w,m,cargo,images.as_mut_slice(),tick)}
        else{self.ships_tick_with_images_live(w,m,cargo,images,tick)}
    }
    fn ships_tick_live(&mut self,w:&mut EconomyWorld,m:&mut [RepMerchant],cargo:&mut Vec<ShipCargo>,tick:u32)->bool{
        if self.navigation_error.is_some() || self.encounter_battle.is_some(){return false}
        let Some(nav)=self.navigation.as_mut() else{return false};
        let before=nav.snapshot.clone();
        // Solo cada ocho ticks se ejecuta el clima: los demás conservan el
        // camino ya transaccional sin copiar el planificador de rutas completo.
        if tick&7!=0 && self.encounter_config.is_none(){
            let result=if nav.active(){nav.step_with_planner(&mut w.towns,m,tick,self.navigation_planner.as_mut())}else{nav.step_idle(&mut w.towns,m,tick)};
            return match result{
                Ok(())=>{self.apply_navigation_ship_lost_letters(w,m,tick);self.publish_navigation_delta(&before,w,cargo);true}
                Err(e)=>{self.navigation_error=Some(e.clone());self.council_errors.push(format!("navegación: {e}"));false}
            }
        }
        let mut staged=nav.clone();let mut towns=w.towns.clone();let mut merchants=m.to_vec();let mut planner=self.navigation_planner.clone();
        let result=if let Some(config)=self.encounter_config.as_ref(){
            let options=navigation::EncounterOptions{slot:config.slot,battle_address:config.battle_address,scene_busy:config.scene_busy,dialog_text:&config.dialog_text};
            staged.step_with_encounter(&mut towns,&mut merchants,tick,planner.as_mut(),options)
        }else if staged.active(){staged.step_with_planner(&mut towns,&mut merchants,tick,planner.as_mut()).map(|()|None)}else{staged.step_idle(&mut towns,&mut merchants,tick).map(|()|None)}
            .and_then(|battle|advance_navigation_weather(&mut staged,planner.as_ref(),tick).map(|effect|(battle,effect.redraw)));
        match result{
            Ok((battle,redraw))=>{if redraw{staged.events.push("clima:redraw".into())}w.towns=towns;m.clone_from_slice(&merchants);self.navigation_planner=planner;*nav=staged;self.encounter_battle=battle;self.apply_navigation_ship_lost_letters(w,m,tick);self.publish_navigation_delta(&before,w,cargo);true}
            Err(e)=>{self.navigation_error=Some(e.clone());self.council_errors.push(format!("navegación: {e}"));false}
        }
    }
    fn construction_prepare(&mut self,w:&EconomyWorld){
        self.construction_tick=w.tick;
        for (r,f) in self.employment_businesses.iter_mut().zip(w.merchant_facilities.iter()){r[4..6].copy_from_slice(&f.employees.to_le_bytes());}
    }
    fn construction_business_efficiency(&mut self,index:u16,efficiency:i32){if let Some(r)=self.employment_businesses.get_mut(index as usize){r[..4].copy_from_slice(&efficiency.to_le_bytes());}}
    fn construction_assets(&self,town:u8)->Option<(Site,p3_core::economy::facility_pool::FacilityPool)>{
        Some((self.sites.get(town as usize)?.clone(),p3_core::economy::facility_pool::FacilityPool{records:self.employment_businesses.clone(),free:self.business_free}))
    }
    fn construction_commit(&mut self,town:u8,site:Site,pool:p3_core::economy::facility_pool::FacilityPool,offices:&[Option<Vec<u8>>],letters:&[(u32,u8,u16,u16)],merchants:&mut [RepMerchant])->Result<(),String>{
        let mut messages=self.message_pool.clone();let mut people=merchants.to_vec();let date=Calendar::for_tick(self.construction_tick);
        for &(owner,city,kind,record) in letters {let i=owner as usize;if let Some(m)=people.get_mut(i){p3_state::messages::add_building_finished(&mut messages,m,date.day,date.month,date.year,city,kind,record,self.building_message_enabled.get(i).copied().unwrap_or(true))?;}}
        let dest=self.sites.get_mut(town as usize).ok_or("plano ausente")?;*dest=site;self.message_pool=messages;merchants.clone_from_slice(&people);
        self.employment_businesses=pool.records;self.business_free=pool.free;
        for (i,img) in offices.iter().enumerate(){if let Some(m)=img {
            if let Some(l)=self.lists.get_mut(i){l.next=u16_at(m,0x2ca);l.building_head=u16_at(m,0x2ce);}
            if let Some(h)=self.housing.get_mut(i){h.next=u16_at(m,0x2ca);h.houses_head=u16_at(m,0x2d0);h.house_count=u16_at(m,0x2d4);for k in 0..3{h.capacity[k]=u16_at(m,0x2d8+2*k);h.assigned[k]=u16_at(m,0x2de+2*k);h.level[k]=m[0x2e4+k];}}
        }}
        self.construction_dirty=true;Ok(())
    }
    fn construction_changed(&mut self)->bool{std::mem::take(&mut self.construction_dirty)}
    fn office_update_business_employees(&mut self, w: &mut EconomyWorld, office: u16) -> u16 {
        let i=office as usize;
        if let Some(value)=self.employment_counts.get_mut(i){*value=None;}
        let Some(o)=w.offices.get(i) else {self.council_errors.push(format!("oficina {i} inexistente"));return 0};
        let Some(list)=self.lists.get(i) else {self.council_errors.push(format!("lista de oficina {i} inexistente"));return 0};
        let first=o.first_facility;
        // El llamante recorre la cadena de oficinas de la ciudad actual.
        let Some(town_index)=w.towns.iter().position(|t| {let mut p=t.u16(0x784);let mut steps=0;while p<w.office_count && steps<w.offices.len(){if p==office{return true}p=w.offices[p as usize].next;steps+=1;}false}) else {self.council_errors.push(format!("oficina {i} sin ciudad"));return 0};
        let (Some(town),Some(site))=(w.towns.get_mut(town_index),self.sites.get(town_index)) else {self.council_errors.push("ciudad de empleo inexistente".into());return 0};
        for (raw,f) in self.employment_businesses.iter_mut().zip(w.merchant_facilities.iter()) {raw[4..6].copy_from_slice(&f.employees.to_le_bytes());}
        match p3_core::economy::office_employment::update(town,site,list.building_head,first,&mut self.employment_businesses,&mut w.labour) {
            Ok((count,total))=>{if let Some(dst)=self.employment_counts.get_mut(i){*dst=Some(count)}for (raw,f) in self.employment_businesses.iter().zip(w.merchant_facilities.iter_mut()){f.employees=u16_at(raw,4);}total}
            Err(e)=>{self.council_errors.push(format!("empleo oficina {i}: {e}"));0}
        }
    }
    fn employment_office_count(&self,o:u16)->Option<u16>{self.employment_counts.get(o as usize).copied().flatten()}
    fn employment_business(&self,i:u16)->Option<[u8;20]>{self.employment_businesses.get(i as usize).copied()}
    fn office_daily_tick(&mut self, _: &mut EconomyWorld, _: u16, _: u32) {
        self.hit("office_daily_tick");
    }
    fn office_links_changed(&mut self,w:&EconomyWorld){
        for (i,o) in w.offices.iter().enumerate(){
            if let Some(h)=self.housing.get_mut(i){h.next=o.next;}
            if let Some(l)=self.lists.get_mut(i){l.next=o.next;}
            if let Some(t)=self.tax_offices.get_mut(i){t.next=o.next;}
        }
    }
    fn try_add_type_2f_building(&mut self, town: &mut Town, employees: u32) {
        let city=usize::from(town.index());let Some(site)=self.sites.get_mut(city) else{self.council_errors.push(format!("edificio 0x2f ciudad {city}: plano ausente"));return};
        match live_building::try_add_type2f(town,site,employees,&mut NoMap){Ok(true)=>{if !self.mission_changed_sites.contains(&city){self.mission_changed_sites.push(city)}},Ok(false)=>{},Err(e)=>self.council_errors.push(format!("edificio 0x2f ciudad {city}: {e}"))}
    }
    fn count_street_cells(&mut self, town: &mut Town) {
        let city=usize::from(town.index());let Some(site)=self.sites.get(city) else{self.council_errors.push(format!("calles ciudad {city}: plano ausente"));return};
        match live_building::count_streets(town,site){Ok(true)=>{if !self.mission_changed_sites.contains(&city){self.mission_changed_sites.push(city)}},Ok(false)=>{},Err(e)=>self.council_errors.push(format!("calles ciudad {city}: {e}"))}
    }
    fn recount_buildings(&mut self, t: &mut Town) {
        let site = &mut self.sites[t.index() as usize];
        let mut cx = HousingCtx { offices: &mut self.housing, office_count: self.office_count, visual_owner: None, host: &mut NoMap };
        site.recount_buildings(t, &mut cx);
    }
    fn office_housing(&self,office:usize)->Option<OfficeHousing>{self.housing.get(office).copied()}
    fn recount_defense_buildings(&mut self, t: &mut Town) {
        self.sites[t.index() as usize].recount_defense_buildings(t, &self.lists, self.office_count);
    }
    fn church_daily_update(&mut self, t: &mut Town, _: u8) {
        if p3_core::economy::church::daily_update(t, self.church_cost_factor) {
            self.sites[t.index() as usize].start_church_upgrade_construction(t, None, &mut NoMap);
        }
    }
    fn tavern_daily(&mut self, _: &mut Town) {
        self.hit("tavern_daily");
    }
    fn tavern_daily_with_world(&mut self, world: &EconomyWorld, town: &mut Town) {
        p3_core::economy::tavern::daily_update(world, town);
    }
    fn council_offer(&mut self, _: &mut Town) {
        self.hit("council_offer");
    }
    fn council_offer_with_world(&mut self, world: &EconomyWorld, town: &mut Town) {
        if let Some(op)=p3_core::economy::council::propose(town,world.tick,&self.council_scheduler) {
            self.council_operations.push(op);
        }
    }
    fn dispatch_scheduled_tasks(&mut self,world:&mut EconomyWorld,merchants:&mut [RepMerchant],tick:u32){
        let offices=self.tax_offices.clone();self.dispatch_scheduled_tasks_full(world,merchants,&offices,tick);
    }
    fn dispatch_scheduled_tasks_full(&mut self,world:&mut EconomyWorld,merchants:&mut [RepMerchant],offices:&[p3_core::economy::extra_tax::TaxOffice],tick:u32){
        self.dispatch_scheduled_tasks_assets(world,merchants,offices,&mut [],tick)
    }
    fn dispatch_scheduled_tasks_assets(&mut self,world:&mut EconomyWorld,merchants:&mut [RepMerchant],offices:&[p3_core::economy::extra_tax::TaxOffice],stocks:&mut [p3_core::economy::trade::OfficeStock],tick:u32){
        self.dispatch_scheduled_tasks_fleet(world,merchants,offices,stocks,&mut Vec::new(),2.0,tick)
    }
    fn dispatch_scheduled_tasks_fleet(&mut self,world:&mut EconomyWorld,merchants:&mut [RepMerchant],offices:&[p3_core::economy::extra_tax::TaxOffice],stocks:&mut [p3_core::economy::trade::OfficeStock],fleet:&mut Vec<ShipCargo>,difficulty:f32,tick:u32){
        let loan_names=self.loan_applicant_name_counts();let ordinary_family_names=self.ordinary_family_name_count();let captain_names=self.pirate_captain_name_counts();
        let human_order=self.human_ship_order.take();let mut human_result=None;
        let mut scheduler=std::mem::take(&mut self.council_scheduler);
        let mut updated=offices.to_vec();for (o,h) in updated.iter_mut().zip(self.housing.iter()){o.houses=h.house_count;}
        let docked_before=self.expedition_docked_head;
        // Estado naval anterior al despacho en memoria reutilizada: solo los bloques que compara la publicación.
        let mut scratch=std::mem::take(&mut self.nav_scratch);let had_navigation=self.navigation.is_some();
        if let Some(nav)=self.navigation.as_ref(){scratch.before.capture(&nav.snapshot)}
        let expedition_flat=had_navigation&&scratch.capture_expedition(&self.expedition_ship_images,&self.expedition_convoy_images);
        let expedition_before=(had_navigation&&!expedition_flat).then(||(self.expedition_ship_images.clone(),self.expedition_convoy_images.clone()));
        let event_band_saved=self.pirate_band_images_saved.borrow().clone();
        let mut host=CouncilExecution{game_end_start:self.game_end_start,game_end_mode:self.game_end_mode,game_end_top_ranked:self.game_end_top_ranked,game_end_score_flag:self.game_end_score_flag,game_end_stopped:&mut self.game_end_stopped,game_end_events:&mut self.game_end_events,return_resources:&self.expedition_return_resources,shipyard_detached:self.detached_shipyard.is_some(),return_pool_limit:self.route_stop_runtime.as_ref().and_then(|p|p3_save::tail::RouteStopPool::parse(p.bytes()).ok()).map(|p|p.count),return_map_dirty:&mut self.expedition_return_map_dirty,return_centers:&mut self.trade_centers,return_center_versions:&mut self.trade_center_versions,return_map:&mut self.discovery_map,return_flag:&mut self.first_discovery_flag,return_points:&self.expedition_map_points,return_planner:&mut self.navigation_planner,return_notices:&mut self.expedition_return_notices,return_map_notices:&mut self.expedition_return_map_notices,return_map_windows:&mut self.expedition_return_map_windows,land_transport_notifications:&mut self.land_transport_notifications,trade_offers:&mut self.trade_offers,wedding_notifications:&mut self.wedding_notifications,wedding_windows:&mut self.wedding_windows,siege_headless:self.siege_headless_march,siege_records:self.founded_world.as_ref().map(|w|w.sieges.as_slice()),siege_live:&self.siege_walled_live,town_hall_notices:&mut self.town_hall_notices,election_texts:&self.election_texts,hunt_texts:&self.hunt_texts,hunt_bands:&mut self.mission_pirates,event_band_initial:&self.pirate_band_images_initial,event_band_saved:&event_band_saved,event_band_images:&mut self.pirate_band_images,event_battle_difficulty:self.battle_difficulty,event_pirate_difficulty:self.pirate_difficulty,event_inland:&self.settlement_inland,event_piece_bytes:self.town_pirate_piece_bytes,election_notifications:&mut self.election_notifications,election_announcements:&mut self.election_announcements,burn_lists:&mut self.lists,burn_housing:&mut self.housing,burn_updates:&mut self.burn_office_updates,pending_office:&mut self.pending_office,pending_founding:&mut self.pending_founding,pending_pirate_approach:&mut self.pending_pirate_approach,pending_pirate_attack:&mut self.pending_pirate_attack,pending_auction_office:&mut self.pending_auction_office,pending_ai_expansion:&mut self.pending_ai_expansion,candidate_name_lengths:&self.candidate_name_lengths,candidate_look_counts:&self.candidate_look_counts,mission_bank:&self.mission_bank,mission_variables:&mut self.mission_variables,hunt_losses:&mut self.hunt_losses,employment_businesses:&mut self.employment_businesses,business_free:&mut self.business_free,world,merchants,sites:&mut self.sites,navigation:&mut self.navigation,navigation_ai:&self.navigation_ai,scheduled_office_images:&mut self.scheduled_office_images,scheduled_office_updates:&mut self.scheduled_office_updates,pool:&mut self.message_pool,enabled:&self.council_message_enabled,building_enabled:&self.building_message_enabled,notifications:&mut self.council_notifications,celebration_notifications:&mut self.celebration_notifications,celebration_windows:&mut self.celebration_windows,plague_notifications:&mut self.plague_notifications,plague_windows:&mut self.plague_windows,fire_notifications:&mut self.fire_notifications,fire_windows:&mut self.fire_windows,hanse_notifications:&mut self.hanse_notifications,debt_notifications:&mut self.debt_notifications,seizure_notifications:&mut self.debt_seizure_notifications,stocks,fleet,ship_metadata:&mut self.debt_ship_metadata,changed_ships:&mut self.debt_changed_ships,changed_towns:&mut self.debt_changed_towns,vote_notifications:&mut self.hanse_vote_notifications,vote_windows:&mut self.hanse_vote_windows,financial_towns:&mut self.hanse_financial_towns,convoy_flags:&mut self.debt_convoy_flags,changed_convoys:&mut self.debt_changed_convoys,pirates:&mut self.hanse_pirates,mission_town_ids:&self.expedition_town_ids,mission_pirate_targets:&mut self.pirate_targets,mission_locations:&self.settlement_locations,mission_notifications:&mut self.mission_completion_notifications,script_notifications:&mut self.mission_script_notifications,mission_ui:&mut self.mission_completion_ui,mission_failure_ui:&mut self.mission_failure_ui,mission_changed_towns:&mut self.mission_changed_towns,mission_flat_rdata:&self.mission_flat_rdata,mission_settings:&self.mission_settings,mission_changed_sites:&mut self.mission_changed_sites,mayor_statue_visuals:&mut self.mayor_statue_visuals,road_build_cached:None,mission_cached:None,expedition_factory:&mut self.expedition_factory,expedition_resources:&self.expedition_resources,expedition_notifications:&mut self.expedition_notifications,expedition_windows:&mut self.expedition_windows,expedition_original_images:&mut self.expedition_original_ship_images,expedition_operations:&mut self.council_operations,expedition_towns:&mut self.expedition_changed_towns,expedition_images:&mut self.expedition_ship_images,expedition_convoys:&mut self.expedition_convoy_images,expedition_docked:&mut self.expedition_docked_head,expedition_sea_head:self.expedition_sea_head,expedition_head:&mut self.expedition_convoys_head,expedition_changed_ships:&mut self.expedition_changed_ships,ai_fleet_changed_ships:&mut self.ai_fleet_changed_ships,expedition_changed_convoys:&mut self.expedition_changed_convoys,route_stops:&mut self.route_stop_runtime,merchant_groups:&mut self.merchant_ship_groups,merchant_groups_changed:&mut self.merchant_ship_groups_changed,mission_texts:&self.mission_texts,mission_distances:self.founded_world.as_ref().map(|w|w.matrix.as_slice()),mission_town_names:&self.town_names,mission_family_names:&self.family_names,mission_first_names:&self.first_names,weather_marker:&mut self.weather_marker,weather_speed:self.weather_speed,weather_notices:&mut self.weather_ice_notifications,rep_scale:self.church_cost_factor,operations_mode:self.operations_mode,loan_names,ordinary_family_names,captain_names,investigation_rng:&mut self.investigation_rng,ai_work_residue:self.ai_work_residue_by_tick.iter().find(|(t,_)|*t==tick).map(|(_,v)|*v).or(self.ai_work_residue),reminder_gate:self.reminder_hotseat_gate,reminder_notices:&mut self.reminder_notices,celebration_state:&mut self.celebration_state,investigation_local:self.investigation_local_mode,investigation_notifications:&mut self.investigation_notifications,investigation_towns:&mut self.investigation_changed_towns,errors:&mut self.council_errors,auction_notices:&mut self.auction_notices,auction_finished:&mut self.auction_finished,auction_results:&mut self.auction_results,offices:&updated,tick,difficulty,letters:Vec::new()};
        if let Some((city,owner,kind,dry))=human_order{let result=match host.ship_retry_candidate(city,owner,kind){Ok(Some(build))=>{let out=(build.after.ship,build.after.cost);if !dry{host.publish_ship_build(build);}Ok(Some(out))},Ok(None)=>Ok(None),Err(e)=>Err(e)};human_result=Some(result);}else{scheduler.dispatch(tick,&mut host);}self.council_scheduler=scheduler;if human_result.is_some(){self.human_ship_order_result=human_result.take();}
        if let (Some(nav),true)=(self.navigation.as_mut(),had_navigation){
            let result=if let Some((ships,convoys))=expedition_before{
                let new_ships:Vec<usize>=(0..ships.len().min(self.expedition_ship_images.len())).filter(|&i|ships[i]!=self.expedition_ship_images[i]).collect();let new_convoys:Vec<usize>=(0..convoys.len().min(self.expedition_convoy_images.len())).filter(|&i|convoys[i]!=self.expedition_convoy_images[i]).collect();
                factory_nav::publish_images(nav,&ships,&self.expedition_ship_images,&new_ships,&convoys,&self.expedition_convoy_images,&new_convoys,docked_before,self.expedition_docked_head)
            }else{
                let new_ships:Vec<usize>=(0..scratch.exp_ship_some.len().min(self.expedition_ship_images.len())).filter(|&i|scratch.expedition_ship(i)!=self.expedition_ship_images[i].as_deref()).collect();
                let new_convoys:Vec<usize>=(0..(scratch.exp_convoys.len()/0x3c).min(self.expedition_convoy_images.len())).filter(|&i|scratch.expedition_convoy(i)!=Some(self.expedition_convoy_images[i].as_slice())).collect();
                factory_nav::publish_images_from(nav,|i|scratch.expedition_ship(i),&self.expedition_ship_images,&new_ships,|i|scratch.expedition_convoy(i),&self.expedition_convoy_images,&new_convoys,docked_before,self.expedition_docked_head)
            };
            if let Err(e)=result{self.council_errors.push(format!("expedición en la flota viva: {e}"))}
        }
        self.nav_scratch.changed=std::mem::take(&mut scratch.changed);
        if had_navigation {let b=&scratch.before;self.publish_navigation_delta_from(&b.manager,&b.ships,&b.captains,&b.convoys,world,fleet)}
        // `publish_navigation_delta_from` deja su lista de trabajo en `self.nav_scratch`; se conserva junto al resto.
        scratch.changed=std::mem::take(&mut self.nav_scratch.changed);self.nav_scratch=scratch;
    }
    fn construction_planner(&mut self, _: &mut Town) {
        self.hit("construction_planner");
    }
    fn construction_planner_with_world(&mut self, world:&EconomyWorld, town:&mut Town) {
        if world.day_of_year & 7 == 2 {return}
        if world.day_of_year & 7 == 7 {
            let city=usize::from(town.index());
            let Some(site)=self.sites.get(city) else{self.council_errors.push(format!("planificador de defensa ciudad {city}: plano ausente"));return};
            match defense_planner::plan(town,site,world,&self.council_scheduler,&mut self.investigation_rng){Ok(ops)=>self.council_operations.extend(ops),Err(e)=>self.council_errors.push(format!("planificador de defensa ciudad {city}: {e}"))}
            return
        }
        if world.day_of_year & 7 == 6 {
            let city=usize::from(town.index());
            let Some(site)=self.sites.get(city) else{self.council_errors.push(format!("planificador de pozo ciudad {city}: plano ausente"));return};
            match p3_core::economy::well_planner::plan(town,site,world.merchant_count){Ok(Some(op))=>self.council_operations.push(op),Ok(None)=>{},Err(e)=>self.council_errors.push(format!("planificador de pozo ciudad {city}: {e}"))}
            return
        }
        if matches!(world.day_of_year & 7,1|3|5){
            let city=usize::from(town.index());
            let Some(site)=self.sites.get(city) else{self.council_errors.push(format!("planificador de vivienda ciudad {city}: plano ausente"));return};
            match housing_planner::plan(town,site,world,&self.council_scheduler,&mut self.investigation_rng){Ok(ops)=>self.council_operations.extend(ops),Err(e)=>self.council_errors.push(format!("planificador de vivienda ciudad {city}: {e}"))}
            return
        }
        if world.day_of_year & 7 == 0 {
            let city=usize::from(town.index());
            let Some(site)=self.sites.get(city) else {self.council_errors.push(format!("planificador de muralla ciudad {city}: plano ausente"));return};
            match wall_work::plan(town,site,world,&self.tax_offices){Ok(ops)=>self.council_operations.extend(ops),Err(e)=>self.council_errors.push(format!("planificador de muralla ciudad {city}: {e}"))}
            return
        }
        if world.day_of_year & 7 != 4 {self.hit("construction_planner");return}
        let city=usize::from(town.index());
        let Some(site)=self.sites.get(city) else {self.council_errors.push(format!("planificador ciudad {city}: plano ausente"));return};
        match civic_planner::plan(town,site,world.merchant_count,&mut self.investigation_rng) {
            Ok(ops)=>self.council_operations.extend(ops),
            Err(e)=>self.council_errors.push(format!("planificador ciudad {city}: {e}")),
        }
    }
    fn advance_construction(&mut self, town: &mut Town, workers: u16) {
        let city=town.index() as usize;
        let result=match self.sites.get_mut(city){Some(site)=>progress_only(site,town,workers),None=>Err("plano de obra ausente".into())};
        match result {Ok(true)=>{},Ok(false)=>self.hit("advance_construction_completion"),Err(e)=>self.council_errors.push(format!("obra ciudad {city}: {e}"))}
    }
    fn shipyard_construction(&mut self, _: u8, _: u32) {
        self.hit("shipyard_construction");
    }
    fn shipyard_repairs(&mut self, _: u8, _: u32) {
        self.hit("shipyard_repairs");
    }
    fn shipyard_advance_transaction(&mut self,town:&mut Town,work:u32,merchants:&mut [RepMerchant])->Result<(),String>{
        let Some(nav)=self.navigation.as_ref() else {
            let factory=self.expedition_factory.as_ref().ok_or("flota de astillero ausente")?;
            let count=u16::try_from(factory.ships.len()).map_err(|_|"demasiados barcos")?;
            let entries=u16::try_from(factory.convoys.len()).map_err(|_|"demasiadas entradas navales")?;
            if factory.ships.iter().any(|s|s.len()!=0x180)||factory.convoys.iter().any(|s|s.len()!=0x3c){return Err("imágenes del astillero truncadas".into())}
            if factory.convoys!=self.expedition_convoy_images||factory.convoy_head!=self.expedition_convoys_head{return Err("entradas navales desincronizadas".into())}
            if town.u16(0x81c)>=count&&town.u16(0x81e)>=count&&town.u16(0x820)>=entries{return Ok(())}
            let before=p3_core::shipyard::ShipManager{ships:factory.ships.concat(),entries:factory.convoys.concat(),docked_head:self.expedition_docked_head,free_entry_head:factory.convoy_head,ship_count:count,entry_count:entries};
            if let Some(detached)=&self.detached_shipyard {if detached.current!=before{return Err("flota modificada durante la transacción de astillero".into())}}
            let out=p3_state::office::advance_shipyard_snapshot(town,&before,merchants,&self.message_pool,&self.building_message_enabled,work,self.construction_tick,self.expedition_resources.as_ref().map_or(u32::MAX,|r|u32::from(r.player)))?;
            if self.expedition_resources.is_none()&&out.presentation.iter().any(|event|matches!(event,p3_state::office::YardPresentation::Notify(_,_,6,_))){return Err("jugador del astillero no configurado para la botadura".into())}
            let first_factory=if self.detached_shipyard.is_none(){Some(factory.clone())}else{None};
            if let Some(factory_before)=first_factory {self.detached_shipyard=Some(DetachedShipyard{factory_before,before:before.clone(),current:before.clone(),history:Vec::new()});}
            let detached=self.detached_shipyard.as_mut().expect("transacción creada");if detached.current!=out.manager{detached.history.push(ShipyardDelta::between(&detached.current,&out.manager));}detached.current=out.manager.clone();
            let factory=self.expedition_factory.as_mut().expect("flota comprobada");
            for (dst,src) in factory.ships.iter_mut().zip(out.manager.ships.chunks_exact(0x180)){dst.copy_from_slice(src)}
            for (dst,src) in factory.convoys.iter_mut().zip(out.manager.entries.chunks_exact(0x3c)){dst.copy_from_slice(src)}
            factory.convoy_head=out.manager.free_entry_head;
            self.expedition_convoy_images=factory.convoys.clone();self.expedition_docked_head=out.manager.docked_head;self.expedition_convoys_head=out.manager.free_entry_head;
            self.message_pool=out.message_pool;self.shipyard_presentation.extend(out.presentation);merchants.clone_from_slice(&out.merchants);*town=out.town;return Ok(())
        };
        let snap=&nav.snapshot;if snap.manager.len()<0xf8{return Err("gestor naval truncado".into())}
        let count=u16_at(&snap.manager,0xf4);let entries=u16_at(&snap.manager,0xf6);
        if town.u16(0x81c)>=count&&town.u16(0x81e)>=count&&town.u16(0x820)>=entries{return Ok(())}
        let manager=p3_core::shipyard::ShipManager{ships:snap.ships.clone(),entries:snap.convoys.clone(),docked_head:u16_at(&snap.manager,0xe8),free_entry_head:u16_at(&snap.manager,0xec),ship_count:count,entry_count:entries};
        let out=p3_state::office::advance_shipyard_snapshot(town,&manager,merchants,&self.message_pool,&self.building_message_enabled,work,self.construction_tick,nav.player)?;
        let mut staged=nav.clone();staged.snapshot.ships=out.manager.ships;staged.snapshot.convoys=out.manager.entries;staged.snapshot.manager[0xe8..0xea].copy_from_slice(&out.manager.docked_head.to_le_bytes());staged.snapshot.manager[0xec..0xee].copy_from_slice(&out.manager.free_entry_head.to_le_bytes());
        navigation::refresh_lists(&mut staged.snapshot)?;
        let factory=self.expedition_factory.as_ref().ok_or("fábrica naval ausente")?;
        if factory.ships.len()!=usize::from(count)||factory.convoys.len()!=usize::from(entries){return Err("fábrica naval desincronizada".into())}
        let mut convoy_images=self.expedition_convoy_images.clone();if convoy_images.len()!=usize::from(entries){return Err("entradas navales desincronizadas".into())}
        for(i,image)in convoy_images.iter_mut().enumerate(){if image.len()!=0x3c{return Err("entrada naval truncada".into())}for j in 0..0x3c{let at=i*0x3c+j;let old=manager.entries[at];let new=staged.snapshot.convoys[at];if old!=new{if image[j]!=old&&image[j]!=new{return Err(format!("entrada naval {i}+{j:x} desincronizada"))}image[j]=new;}}}
        if manager.docked_head!=out.manager.docked_head&&self.expedition_docked_head!=manager.docked_head&&self.expedition_docked_head!=out.manager.docked_head{return Err("lista de atracados desincronizada".into())}
        // La fábrica solo persiste nuevas ranuras; las imágenes de barcos ya
        // existentes se guardan desde el snapshot de navegación.
        self.expedition_convoy_images=convoy_images;self.expedition_docked_head=out.manager.docked_head;
        self.navigation=Some(staged);self.message_pool=out.message_pool;self.shipyard_presentation.extend(out.presentation);merchants.clone_from_slice(&out.merchants);*town=out.town;Ok(())
    }
    fn wall_expansion_level(&mut self, t: &Town) -> u8 {
        self.sites[t.index() as usize].wall_expansion_level(t)
    }
    fn find_or_create_office(&mut self, _: &mut EconomyWorld, _: u8, _: u8) -> Option<usize> {
        self.hit("find_or_create_office");
        None
    }
    fn siege_pitch_demand(&mut self, city: u8) -> i32 {self.siege_pitch_value(city,None)}
    fn siege_pitch_demand_for_town(&mut self,town:&Town)->i32 {self.siege_pitch_value(town.index(),Some(town))}
    fn famine_message(&mut self, _: u8) {
        self.hit("famine_message");
    }
    fn famine_message_with_world(&mut self,world:&EconomyWorld,town:u8){
        let date=Calendar::for_tick(world.tick);let mut record=[0u8;16];record[4]=0x35;record[5]=town;
        if let Err(e)=p3_state::messages::add_global_inline_letter(&mut self.message_pool,date.day,date.month,date.year,&record){self.council_errors.push(format!("carta de hambruna ciudad {town}: {e}"));}
    }
    fn famine_notify(&mut self, merchant:u16, town:u8, local:bool) {
        self.famine_notifications.push(FamineNotification{merchant,town,local});
    }
}

/// Fases del reloj que todavía no tienen sistema propio: no hacen nada y se cuentan.
#[derive(Default)]
pub struct IdleSystems {
    pub clock: WorldClock,
    pub calls: BTreeMap<&'static str, u64>,
    pub battle_manager:Option<battle_manager::Runtime>,
}

impl IdleSystems {
    fn hit(&mut self, name: &'static str) {
        *self.calls.entry(name).or_default() += 1;
    }
}

impl TickSystems for IdleSystems {
    fn clock(&mut self) -> &mut WorldClock {
        &mut self.clock
    }
    fn time_token(&self) -> u32 {
        0
    }
    fn close_day(&mut self, _: bool) {
        unreachable!("lo aplica el puente de economía")
    }
    fn town_daily(&mut self, _: u32, _: u16) {
        unreachable!("lo aplica el puente de economía")
    }
    fn statistics_daily(&mut self) {
        self.hit("statistics_daily");
    }
    fn town_history_daily(&mut self, _: u32) {
        self.hit("town_history_daily");
    }
    fn merchant_daily(&mut self, _: u32) {
        self.hit("merchant_daily");
    }
    fn trade_center_daily(&mut self, _: u32) {
        self.hit("trade_center_daily");
    }
    fn siege_daily(&mut self, _: u32) {
        self.hit("siege_daily");
    }
    fn ships_tick(&mut self, _: u32) {
        self.hit("ships_tick");
    }
    fn scheduled_tasks_tick(&mut self, _: u32) {
        self.hit("scheduled_tasks_tick");
    }
    fn harbor_scene_sync(&mut self) {}
    fn sea_battles_tick(&mut self) {
        if let Some(runtime)=self.battle_manager.as_mut(){
            if runtime.error.is_none(){if let Err(e)=battle_manager::tick(&mut runtime.manager,&mut runtime.groups,true,battle_manager::ViewState{busy:false,current_scene_matches:false}){runtime.error=Some(e.into())}}
        }else{self.hit("sea_battles_tick");}
    }
    fn siege_scene(&mut self, _: u32) {}
    fn merchant_gold(&self, _: u32) -> i32 {
        unreachable!("lo aplica el puente de economía")
    }
    fn set_merchant_gold(&mut self, _: u32, _: i32) {
        unreachable!("lo aplica el puente de economía")
    }
}

pub struct Sim {
    // El S05 contiene una batalla cuya simulación pertenece al anfitrión.
    saved_battle_pending: Option<Vec<u8>>,
    // Asalto urbano validado pero aún sin recursos para reconstruir su escena.
    saved_town_attack_pending: Option<Vec<u8>>,
    pub economy: EconomySystem,
    pub idle: IdleSystems,
    pub host: NullHost,
    /// Oficinas con todos sus campos, negocios resueltos y salarios de administrador (índices de oficina).
    pub persisted: Vec<PersistedOffice>,
    pub rep_merchants: Vec<RepMerchant>,
    pub businesses: Vec<Vec<BusinessInput>>,
    pub wages: Vec<Option<u16>>,
    /// Factor comercial del original; 2.0 por defecto si no se cargan opciones.
    pub commercial_sell_factor: f32,
    /// Cabecera del guardado: día, mes, año y día del año tal como los guarda el juego.
    pub saved_calendar: (u8, u8, u16, u16),
    /// Bytes de comerciante cambiados por las dos copias con valores distintos que no son acumuladores (véase `reconcile_merchants`).
    pub merchant_conflicts: u64,
    /// Veces que economía y oficinas cambiaron a la vez un acumulador con valores distintos (se suman las variaciones).
    pub merchant_double_changes: u64,
}

/// Resumen de un instante de la simulación.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Summary {
    pub tick: u32,
    pub day: u8,
    pub month: u8,
    pub year: u16,
    pub merchants_gold: i64,
    pub population: [i64; 4],
    pub town_money: i64,
    pub stock: [i64; 24],
    pub hash: String,
}

#[derive(Clone, Debug)]
pub struct Report {
    pub start: Summary,
    pub end: Summary,
    pub days: u32,
    /// Ganchos y fases a los que se llamó sin sistema detrás.
    pub inactive: BTreeMap<&'static str, u64>,
    /// Fallos del avance o dependencias aún no cubiertas.
    pub errors: Vec<String>,
}

impl Sim {
    /// Despacha el pedido IA 0x2C sobre imágenes vivas y publica sus efectos juntos.
    /// Operación 0x3A (`op_start_land_transport`, `0x0053A2C0`): crea la tarea de transporte terrestre `0x0C`
    /// ([`p3_core::economy::land_transport_send::start`], paridad x86) y despacha el planificador con el reloj actual
    /// (`0x004D85C0`), que ejecuta en el acto la fase 0 (extracción de la oficina y pago de la escolta).
    /// Devuelve el índice de la tarea, o `None` si la operación no llevaba cargas.
    pub fn start_land_transport(&mut self,op:p3_core::pump::Operation)->Result<Option<u16>,String>{
        let tick=self.idle.clock.tick;
        let mut scheduler=self.host.council_scheduler.clone();
        let idx=p3_core::economy::land_transport_send::start(&mut scheduler,op,tick);
        self.host.mission_variables.resize(usize::from(scheduler.capacity),None);
        self.host.council_scheduler=scheduler;
        if idx.is_none(){return Ok(None)}
        let mut clock=crate::merchant_daily::DailyClock{sim:self,world_ready:false,error:None};
        p3_core::tick::TickSystems::scheduled_tasks_tick(&mut clock,tick);
        if let Some(error)=clock.error{return Err(error)}
        Ok(idx)
    }
    /// Operación 0x36 (`0x004DA510`): el barco entra en una subasta naval de su ciudad o abre una nueva.
    fn register_ship_for_auction(&mut self,op:p3_core::pump::Operation)->Result<(),String>{
        use p3_core::economy::auction_register::{register_ship_for_auction,RegisterWorld};
        let factory=self.host.expedition_factory.as_ref().ok_or("fábrica naval ausente")?;let count=usize::from(self.economy.world.ship_count);
        if factory.ships.len()!=count||self.economy.world.ships.len()!=count||self.economy.cargo_ships.len()!=count||self.host.debt_ship_metadata.len()!=count||self.host.expedition_ship_images.len()!=count{return Err("vistas de flota incompletas".into())}
        let(mut ships,convoy_count)=if let Some(nav)=self.host.navigation.as_ref(){if nav.snapshot.manager.len()<0xf8||nav.snapshot.ships.len()!=count*0x180||u16_at(&nav.snapshot.manager,0xf4) as usize!=count{return Err("gestor naval desincronizado".into())}(nav.snapshot.ships.chunks_exact(0x180).map(<[u8]>::to_vec).collect::<Vec<_>>(),u16_at(&nav.snapshot.manager,0xf6))}else{(factory.ships.iter().enumerate().map(|(i,free)|self.host.expedition_ship_images[i].as_ref().unwrap_or(free).clone()).collect(),u16::try_from(self.host.expedition_convoy_images.len()).map_err(|_|"convoyes excesivos")?)};
        let mc=usize::from(self.economy.world.merchant_count);let people=self.rep_merchants.get(..mc).ok_or("comerciantes ausentes")?;if people.iter().any(|m|m.bytes.len()<0x46c){return Err("comerciante truncado".into())}
        let masks=people.iter().map(|m|m.i32(0x468) as u32).collect::<Vec<_>>();
        let w=RegisterWorld{convoy_count,town_count:self.economy.world.towns.len() as u32,town_masks:&masks,tick:self.idle.clock.tick,options_d:self.host.game_end_mode};
        let mut scheduler=self.host.council_scheduler.clone();
        if register_ship_for_auction(&mut scheduler,&mut ships,&w,op.0[1]).is_none(){return Ok(())}
        let i=op.0[1] as usize;let image=ships.swap_remove(i);let state=u16_at(&image,0x134);
        self.host.mission_variables.resize(usize::from(scheduler.capacity),None);self.host.council_scheduler=scheduler;
        if let Some(nav)=self.host.navigation.as_mut(){nav.snapshot.ships[i*0x180..(i+1)*0x180].copy_from_slice(&image)}
        self.economy.world.ships[i].state=state;self.economy.cargo_ships[i].dirty=u16_at(&image,0x3c);self.host.debt_ship_metadata[i].1=state;
        let factory=self.host.expedition_factory.as_mut().expect("validado");if self.host.expedition_ship_images[i].is_some(){self.host.expedition_ship_images[i]=Some(image)}else{factory.ships[i]=image}
        Ok(())
    }
    fn apply_ai_ship_order(&mut self,op:p3_core::pump::Operation)->Result<(),String>{
        let owner=usize::try_from(op.0[2]).map_err(|_|"propietario naval inválido")?;
        let city=usize::try_from(op.0[3]).map_err(|_|"ciudad naval inválida")?;
        if owner>=usize::from(self.economy.world.merchant_count)||city>=self.economy.world.towns.len()||op.0[1]>=4{return Ok(())}
        let owner_byte=u8::try_from(owner).map_err(|_|"propietario naval fuera de rango")?;
        let merchant=self.rep_merchants.get(owner).ok_or("comerciante naval ausente")?;
        if self.economy.merchants.get(owner).is_none()||self.economy.world.merchants.get(owner).is_none(){return Err("vistas de comerciante naval incompletas".into())}
        let factory=self.host.expedition_factory.as_ref().ok_or("fábrica naval ausente")?;
        let count=usize::from(self.economy.world.ship_count);
        if factory.ships.len()!=count||self.economy.world.ships.len()!=count||self.economy.cargo_ships.len()!=count||self.host.debt_ship_metadata.len()!=count||self.host.expedition_ship_images.len()!=count{return Err("vistas de flota incompletas".into())}
        if self.host.detached_shipyard.is_some(){return Err("pedido naval requiere conciliar primero el astillero sin navegación".into())}
        let ships=if let Some(nav)=self.host.navigation.as_ref(){
            if nav.snapshot.manager.len()<0xf8||nav.snapshot.ships.len()!=count*0x180||nav.snapshot.routes.len()!=count||u16_at(&nav.snapshot.manager,0xf4) as usize!=count||u16_at(&nav.snapshot.manager,0xe2)!=factory.free_ship||u16_at(&nav.snapshot.manager,0xee)!=factory.name_cursor||u16_at(&nav.snapshot.manager,0xf0)!=factory.name_step{return Err("gestor naval desincronizado".into())}
            nav.snapshot.ships.chunks_exact(0x180).map(<[u8]>::to_vec).collect()
        }else{factory.ships.iter().enumerate().map(|(i,free)|self.host.expedition_ship_images[i].as_ref().unwrap_or(free).clone()).collect()};
        let town=&self.economy.world.towns[city];if usize::from(town.u8(0x2c0))!=city{return Err("índice de ciudad naval incoherente".into())}
        let mut at=usize::from(town.u16(0x784));let mut seen=vec![false;self.economy.world.offices.len()];let mut office=None;
        while at<self.economy.world.offices.len(){if seen[at]{return Err("cadena urbana de oficinas cíclica".into())}seen[at]=true;let stock=self.economy.offices.get(at).ok_or("existencias de oficina ausentes")?;if stock.owner==u16::from(owner_byte){if usize::from(stock.town)!=city||self.persisted.get(at).is_none(){return Err("oficina naval incoherente".into())}office=Some(at);break}at=usize::from(self.economy.world.offices[at].next)}
        if let Some(i)=office{if self.host.scheduled_office_images.as_ref().is_some_and(|v|v.get(i).is_none_or(|b|b.len()<100))||self.host.office_pool.as_ref().is_some_and(|v|v.records.get(i).is_none_or(|b|b.len()<100)){return Err("imágenes de oficina naval incompletas".into())}}
        let resources=self.host.expedition_resources.as_ref().ok_or("recursos navales ausentes")?;
        let port=resources.ports.get(city).copied().flatten().ok_or("puerto de astillero ausente")?;
        let before=c10_ship_orders::BuildState{town:town.clone(),merchant:merchant.clone(),office:office.map(|i|self.economy.offices[i].clone()),ships,free_ship:factory.free_ship,name_cursor:factory.name_cursor,name_step:factory.name_step};
        let Some(out)=c10_ship_orders::prepare(&before,owner_byte,op.0[1] as u8,op.0[4]!=0,&c10_ship_orders::BuildResources{tick:self.idle.clock.tick,port,names:&resources.ship_names})? else{return Ok(())};
        let after=out.state;let new_count=u16::try_from(after.ships.len()).map_err(|_|"pool naval demasiado grande")?;
        let mut pirates=self.host.hanse_pirates.clone();if let Some(p)=pirates.as_mut(){if p.next.len()!=count||p.ship_captains.len()!=count{return Err("vistas piratas incompletas".into())}p.next=after.ships.iter().map(|s|u16_at(s,4)).collect();p.ship_captains=after.ships.iter().map(|s|u16_at(s,0x42)).collect();}
        // No fallible work remains: publish the transaction across every live view.
        self.economy.world.towns[city]=after.town;
        self.economy.world.merchants[owner].gold=after.merchant.i32(0);
        self.economy.merchants[owner]=after.merchant.clone();self.rep_merchants[owner]=after.merchant;
        if let Some(i)=office{let stock=after.office.expect("oficina preparada");self.persisted[i].stock=stock.clone();self.economy.offices[i]=stock;for k in 0..24{let bytes=self.economy.offices[i].stock[k].to_le_bytes();if let Some(images)=self.host.scheduled_office_images.as_mut(){images[i][4+4*k..8+4*k].copy_from_slice(&bytes)}if let Some(pool)=self.host.office_pool.as_mut(){pool.records[i][4+4*k..8+4*k].copy_from_slice(&bytes)}}}
        if let Some(nav)=self.host.navigation.as_mut(){
            // The factory persists new slots. Their navigation baseline starts
            // after creation so subsequent movement is reconciled against them.
            let allocated=usize::from(out.ship);if allocated<count{nav.baseline.ships[allocated*0x180..(allocated+1)*0x180].copy_from_slice(&after.ships[allocated]);nav.baseline.routes[allocated]=Some(p3_save::ships::SavedRoute::None);}
            for image in after.ships.iter().skip(count){nav.baseline.ships.extend_from_slice(image);nav.baseline.routes.push(if u16_at(image,0x134)<0x17{Some(p3_save::ships::SavedRoute::None)}else{None});}
            nav.snapshot.ships=after.ships.concat();nav.snapshot.routes.resize(after.ships.len(),None);nav.snapshot.routes[allocated]=Some(p3_save::ships::SavedRoute::None);
            nav.snapshot.manager[0xe2..0xe4].copy_from_slice(&after.free_ship.to_le_bytes());nav.snapshot.manager[0xee..0xf0].copy_from_slice(&after.name_cursor.to_le_bytes());nav.snapshot.manager[0xf4..0xf6].copy_from_slice(&new_count.to_le_bytes());nav.baseline.manager[0xf4..0xf6].copy_from_slice(&new_count.to_le_bytes());
        }
        self.economy.world.ship_count=new_count;
        self.economy.world.ships.resize_with(after.ships.len(),||Ship{owner:255,state:255});self.economy.cargo_ships.resize_with(after.ships.len(),ShipCargo::default);
        self.host.expedition_ship_images.resize_with(after.ships.len(),||None);self.host.expedition_original_ship_images.resize_with(after.ships.len(),||None);self.host.debt_ship_metadata.resize(after.ships.len(),(u16::MAX,255,0,u16::MAX));
        for(i,image)in after.ships.iter().enumerate(){if before.ships.get(i)==Some(image){continue}let state=u16_at(image,0x134);self.economy.world.ships[i]=Ship{owner:if state<0x17{image[0]}else{255},state};self.host.debt_ship_metadata[i]=(u16_at(image,4),state,u16_at(image,0x15e),u16_at(image,8));self.host.expedition_ship_images[i]=(state<0x17).then(||image.clone());
            let cargo=&mut self.economy.cargo_ships[i];*cargo=ShipCargo{owner:if state<0x17{image[0]}else{255},town:image[0x39],kind:image[0xe],capacity:u32_at(image,0x10)as i32,dirty:u16_at(image,0x3c),crew:u16_at(image,0x40),cutlasses:u16_at(image,0x154),marines:image[0x156],..Default::default()};for k in 0..24{cargo.stock[k]=u32_at(image,0x54+4*k)as i32;cargo.average_price[k]=f32::from_bits(u32_at(image,0xb4+4*k));cargo.guns[k]=image[0x13c+k]}cargo.cargo_value=u32_at(image,0x114)as i32;cargo.cargo_weight=u32_at(image,0x118)as i32;cargo.equipment_weight=u32_at(image,0x11c)as i32;cargo.gun_power=u32_at(image,0x120)as i32;
        }
        let factory=self.host.expedition_factory.as_mut().expect("fábrica validada");factory.ships=after.ships;factory.free_ship=after.free_ship;factory.name_cursor=after.name_cursor;self.host.hanse_pirates=pirates;Ok(())
    }
    /// Prepara una oferta 0x7A a partir de la flota, el mapa y el CRT vivos.
    pub fn queue_tavern_passenger(&mut self)->Result<(),String>{
        let snapshot=&self.host.navigation.as_ref().ok_or("flota viva no reconstruida")?.snapshot;let mut rng=self.host.investigation_rng;
        let op=ten_day_runtime::prepare_passenger(snapshot,&self.economy.world.towns,self.host.hanse_pirates.as_ref().map(|p|p.heads),self.host.navigation_ai.as_ref(),self.idle.clock.tick,&mut rng)?;
        self.host.council_operations.push(op);self.host.investigation_rng=rng;Ok(())
    }
    /// Aplica 0x12 a la flota viva y publica juntos capitán, salario y aviso.
    pub fn apply_captain_skill_operation(&mut self,op:p3_core::pump::Operation)->Result<(),String>{
        if op.0[0]!=0x12{return Err("operación de capitán incorrecta".into())}self.apply_captain_operation(op)
    }
    /// Operación 0x37: ingreso de un comerciante en el gremio de la ciudad.
    fn apply_guild_operation(&mut self,op:p3_core::pump::Operation)->Result<(),String>{
        use p3_core::ai::misc::MessageHost;
        struct Letters(Vec<[u8;16]>);
        impl MessageHost for Letters{fn post_message(&mut self,kind:i32,record:&[u8;16]){assert_eq!(kind,-2);self.0.push(*record)}}
        let owner=op.0[1] as usize;let count=usize::from(self.economy.world.merchant_count);if owner>=count{return Ok(())}
        let merchant=self.rep_merchants.get(owner).ok_or("comerciante de gremio ausente")?;let economic=self.economy.merchants.get(owner).ok_or("vista económica de gremio ausente")?;let account=self.economy.world.merchants.get(owner).ok_or("cuenta de gremio ausente")?;
        if self.economy.world.towns.len()>40{return Err("demasiadas ciudades para gremios".into())}
        if merchant.bytes.len()<0x650||economic.bytes.len()<0x650||account.gold!=merchant.i32(0)||economic.i32(0)!=merchant.i32(0){return Err("vistas de comerciante de gremio desincronizadas".into())}
        let mut fields=vec![0..4,0x468..0x46c,0x4bc..0x4c0];let city=op.0[2] as usize;if city<self.economy.world.towns.len(){let at=0x120+12*city;fields.push(at..at+4)}
        if fields.iter().any(|range|merchant.bytes[range.clone()]!=economic.bytes[range.clone()]){return Err("vistas de comerciante de gremio desincronizadas".into())}
        if self.host.discovery_map.len()!=0x64{return Err("mapa global de descubrimiento ausente".into())}
        let mut merchant=merchant.clone();let mut discovery=self.host.discovery_map.clone();let mut pool=self.host.message_pool.clone();let mut letters=Letters(Vec::new());
        merchant.buy_town_map_knowledge(&mut discovery,self.economy.world.towns.len() as u32,self.economy.world.world_8a,op.0[1],op.0[2],op.0[3] as i32,&mut letters);
        let date=self.idle.clock.calendar;for letter in letters.0{p3_state::messages::add_global_inline_letter(&mut pool,date.day,date.month,date.year,&letter)?;}
        self.economy.world.merchants[owner].gold=merchant.i32(0);for range in fields{self.economy.merchants[owner].bytes[range.clone()].copy_from_slice(&merchant.bytes[range]);}self.rep_merchants[owner]=merchant;self.host.discovery_map=discovery;self.host.message_pool=pool;Ok(())
    }

    fn apply_captain_operation(&mut self,op:p3_core::pump::Operation)->Result<(),String>{
        use p3_core::economy::captain_operations::{self,SkillNotice,RetirementNotice,AdministratorNotice};
        enum Notice{Skill(SkillNotice),Retirement(RetirementNotice),Administrator(AdministratorNotice)}
        let nav=&self.host.navigation.as_ref().ok_or("flota viva no reconstruida")?.snapshot;
        let mut factory=self.host.expedition_factory.clone().ok_or("pool de capitanes ausente")?;
        let current=factory.captains.iter().flatten().copied().collect::<Vec<_>>();
        if current!=nav.captains{return Err("vistas de capitanes desincronizadas".into())}
        let mut captains=current.clone();let mut scheduler=self.host.council_scheduler.clone();
        if self.persisted.len()!=self.economy.world.offices.len()||self.persisted.len()!=self.economy.offices.len()||self.persisted.len()!=self.wages.len(){return Err("vistas de oficinas truncadas".into())}
        let mut records=Vec::with_capacity(self.persisted.len());for(i,office)in self.persisted.iter().enumerate(){let mut image=p3_save::office::decode_office(&office.to_save_v85());image[0x2ca..0x2cc].copy_from_slice(&self.economy.world.offices[i].next.to_le_bytes());records.push(image.to_vec());}
        let notice=match op.0[0]{
            0x12=>captain_operations::captain_skill_gain(op,&nav.ships,&mut captains,&self.rep_merchants,self.economy.world.merchant_count,self.economy.world.towns.len()).map_err(str::to_owned)?.map(Notice::Skill),
            0x13=>captain_operations::captain_retirement(op,&nav.ships,&mut captains,&self.rep_merchants,self.economy.world.merchant_count,self.economy.world.towns.len(),self.idle.clock.tick,&mut scheduler).map_err(str::to_owned)?.map(Notice::Retirement),
            0x67=>captain_operations::administrator_skill_gain(op,&self.economy.world.towns,&records,&mut captains,self.economy.world.merchant_count).map_err(str::to_owned)?.map(Notice::Administrator),
            _=>return Err("operación de capitán incorrecta".into()),
        };
        let mut pirates=self.host.hanse_pirates.clone();
        if let Some(p)=pirates.as_mut(){if p.captains.len()!=factory.captains.len(){return Err("capitanes piratas truncados".into())}for (i,cap) in p.captains.iter_mut().enumerate(){for j in 0..16{let at=i*16+j;if current[at]!=captains[at]{if cap[j]!=current[at]&&cap[j]!=captains[at]{return Err(format!("capitán pirata {i} desincronizado en +{j:#x}"))}cap[j]=captains[at];}}}}
        let mut pool=self.host.message_pool.clone();let mut person=None;
        let admin_office=match &notice{Some(Notice::Administrator(n))=>Some(n.office),_=>None};
        if let Some(notice)=notice{let i=match &notice{Notice::Skill(n)=>usize::from(n.merchant),Notice::Retirement(n)=>usize::from(n.merchant),Notice::Administrator(n)=>usize::from(n.merchant)};let mut merchant=self.rep_merchants.get(i).ok_or("comerciante ausente")?.clone();let economic=self.economy.merchants.get(i).ok_or("vista económica ausente")?;if economic.bytes.len()<12||merchant.u16(0xa)!=economic.u16(0xa){return Err("cabezas de cartas desincronizadas".into())}let date=self.idle.clock.calendar;match notice{Notice::Skill(n)=>p3_state::messages::add_captain_skill_notice(&mut pool,&mut merchant,date.day,date.month,date.year,&n)?,Notice::Retirement(n)=>p3_state::messages::add_captain_retirement_notice(&mut pool,&mut merchant,date.day,date.month,date.year,&n)?,Notice::Administrator(n)=>p3_state::messages::add_administrator_skill_notice(&mut pool,&mut merchant,date.day,date.month,date.year,&n)?};person=Some((i,merchant));}
        for (dst,src) in factory.captains.iter_mut().zip(captains.chunks_exact(16)){dst.copy_from_slice(src)}
        let mut wages=self.wages.clone();let mut updates=Vec::new();for (i,office) in records.iter().enumerate(){let id=usize::from(u16_at(office,0x2f2));if id<factory.captains.len(){let cap=&factory.captains[id];let at=id*16;if admin_office==Some(i)||current[at+0xa]!=captains[at+0xa]||current[at+0xc..at+0xe]!=captains[at+0xc..at+0xe]{wages[i]=Some(u16_at(cap,0xc));updates.push((i,cap[0xa]));}}}
        if let Some((i,merchant))=person{self.economy.merchants[i].set_u16(0xa,merchant.u16(0xa));self.rep_merchants[i]=merchant;}
        for(i,skill)in updates{self.persisted[i].stock.administrator_experience=Some(skill);self.economy.offices[i].administrator_experience=Some(skill);}
        self.host.navigation.as_mut().expect("validado").snapshot.captains=captains;self.host.expedition_factory=Some(factory);self.host.hanse_pirates=pirates;self.host.message_pool=pool;self.host.council_scheduler=scheduler;self.wages=wages;Ok(())
    }
    /// Aplica la operación 0x87 manteniendo las vistas de capitanes y S05 sincronizadas.
    fn apply_candidate_operation(&mut self,op:p3_core::pump::Operation)->Result<(),String>{
        let mut factory=self.host.expedition_factory.clone().ok_or("pool de capitanes ausente")?;let old=factory.captains.clone();let entries=old.iter().flatten().copied().collect::<Vec<_>>();let capacity=u16::try_from(old.len()).map_err(|_|"pool de capitanes demasiado grande")?;
        if let Some(nav)=self.host.navigation.as_ref(){if nav.snapshot.manager.len()<0xf4||nav.snapshot.captains!=entries||u16_at(&nav.snapshot.manager,0xe0)!=factory.free_captain||u16_at(&nav.snapshot.manager,0xf2)!=capacity{return Err("vistas de capitanes desincronizadas".into())}}
        let mut pirates=self.host.hanse_pirates.clone();if let Some(p)=pirates.as_ref(){if p.captains!=old{return Err("vista pirata de capitanes desincronizada".into())}}
        let mut cap_pool=p3_core::ships::captains::CaptainPool{entries,capacity,free_head:factory.free_captain};let mut merchants=self.rep_merchants.clone();let date=self.idle.clock.calendar;let names=self.host.foundation_names.unwrap_or((0,0));let result=p3_core::ai::candidate::apply_candidate(op,&mut merchants,&self.economy.world.towns,&mut cap_pool,self.idle.clock.tick,date.day_of_year,date.day,names.0,names.1).map_err(str::to_owned)?;
        let Some(added)=result else{return Ok(())};let economic=self.economy.merchants.get(added.merchant).ok_or("vista económica ausente")?;let before=&self.rep_merchants[added.merchant];if economic.bytes.len()<0x37||economic.u16(0x34)!=before.u16(0x34)||economic.bytes[0x36]!=before.bytes[0x36]||economic.u16(0xa)!=before.u16(0xa){return Err("vista económica de candidato desincronizada".into())}
        let mut messages=self.host.message_pool.clone();if merchants[added.merchant].u16(8)==0{p3_state::messages::add_candidate_birth(&mut messages,&mut merchants[added.merchant],date.day,date.month,date.year,added.town,added.look)?;}
        factory.captains=cap_pool.entries.chunks_exact(16).map(|v|v.try_into().expect("capitán completo")).collect();factory.free_captain=cap_pool.free_head;if let Some(p)=pirates.as_mut(){p.captains=factory.captains.clone()}
        if let Some(nav)=self.host.navigation.as_mut(){nav.snapshot.captains=cap_pool.entries;nav.snapshot.manager[0xe0..0xe2].copy_from_slice(&cap_pool.free_head.to_le_bytes());nav.snapshot.manager[0xf2..0xf4].copy_from_slice(&cap_pool.capacity.to_le_bytes())}
        let m=&merchants[added.merchant];let economic=&mut self.economy.merchants[added.merchant];economic.set_u16(0x34,m.u16(0x34));economic.bytes[0x36]=m.bytes[0x36];economic.set_u16(0xa,m.u16(0xa));self.rep_merchants=merchants;self.host.message_pool=messages;self.host.expedition_factory=Some(factory);self.host.hanse_pirates=pirates;Ok(())
    }
    /// Recalcula la carga entrante con las listas vivas reconstruidas por C07.
    /// Rechaza datos desincronizados y conserva las ciudades ante errores.
    pub fn recount_fleet_incoming_cargo(&mut self)->Result<(),String>{
        let snapshot=&self.host.navigation.as_ref().ok_or("flota viva no reconstruida")?.snapshot;
        if snapshot.ships.len()%0x180!=0{return Err("tabla naval truncada".into())}
        let n=snapshot.ships.len()/0x180;
        if self.economy.cargo_ships.len()!=n{return Err("vistas de carga naval incompletas".into())}
        for (i,image) in snapshot.ships.chunks_exact(0x180).enumerate(){
            for ware in 0..24 {if u32_at(image,0x54+4*ware) as i32!=self.economy.cargo_ships[i].stock[ware]{return Err(format!("carga naval desincronizada: barco {i}, mercancía {ware}"))}}
        }
        let mut towns=self.economy.world.towns.iter().flat_map(|town|town.0.iter().copied()).collect::<Vec<_>>();
        p3_core::ships::return_convoy::recount_incoming(&snapshot.manager,&snapshot.ships,&snapshot.convoys,&mut towns).map_err(str::to_owned)?;
        for (town,image) in self.economy.world.towns.iter_mut().zip(towns.chunks_exact(0x9f8)){town.0[0x670..0x6d0].copy_from_slice(&image[0x670..0x6d0]);}Ok(())
    }
    /// Purga el buzón de un comerciante usando el depósito vivo y mantiene
    /// sincronizada la cabeza de ambas vistas; los errores son transaccionales.
    pub fn purge_merchant_messages(&mut self,index:usize,tick:u32)->Result<usize,String>{
        let mut merchant=self.rep_merchants.get(index).ok_or("comerciante ausente")?.clone();
        let economic=self.economy.merchants.get(index).ok_or("vista económica del comerciante ausente")?;
        if merchant.bytes.len()<12||economic.bytes.len()<12{return Err("comerciante truncado".into())}
        if economic.u16(0xa)!=merchant.u16(0xa){return Err("cabezas de cartas desincronizadas".into())}
        let mut pool=self.host.message_pool.clone();
        let removed=p3_state::messages::purge_expired(&mut pool,&mut merchant,tick)?;
        self.economy.merchants[index].set_u16(0xa,merchant.u16(0xa));self.rep_merchants[index]=merchant;self.host.message_pool=pool;Ok(removed)
    }
    /// Estado explícito del CRT compartido. El guardado original no lo contiene;
    /// una semilla elegida por el anfitrión no demuestra paridad con esa partida.
    pub fn configure_crt_state(&mut self,state:u32){self.host.investigation_rng=Some(p3_core::rng::CrtRand{state});}
    /// Opcode 0xC8 (velocidad y modo de tiempo, 0x00546DCF) sobre [`NullHost::time_mode`] con el reloj de fotogramas
    /// `frame_clock` (0x006DCCF8).
    pub fn apply_time_mode(&mut self,op:[u32;5],frame_clock:u32){self.host.time_mode.apply(op,frame_clock);}
    /// Elección de velocidad en la ventana de opciones (0x0045DF6D/0x00460590): nivel en los ajustes y opcode 0xC8
    /// con `ms_per_tick(level)` (niveles 0 a 5 de la tabla 0x0066DE7C).
    pub fn set_game_speed(&mut self,level:usize,frame_clock:u32)->Result<(),String>{if level>=p3_core::pump::SPEED_TABLE.len(){return Err("nivel de velocidad fuera de la tabla".into())}self.host.speed_level=Some(level as u32);self.apply_time_mode(p3_core::pump::speed_op(level).0,frame_clock);Ok(())}
    /// Escribe en S05 la ciudad y oficinas indicadas y sus negocios tras el tick.
    /// Es una operación acotada, no un guardado completo del simulador.
    pub fn persist_employment_city(&self, model:&mut p3_save::model::SaveModel, city:usize) -> Result<(),String> {
        let mut next=model.clone();
        let raw=next.world.write();let old=p3_save::world::World::parse(&raw).map_err(|e|format!("mundo: {e:?}"))?;
        if city>=old.town_count(){return Err("ciudad inexistente".into())}
        let before=old.town(city);let current=self.economy.world.towns.get(city).ok_or("ciudad de simulación ausente")?;
        let mut after=before.clone();
        after.0[..p3_save::world::ECONOMY_LEN].copy_from_slice(&current.0[..p3_save::world::ECONOMY_LEN]);
        let current_site=self.host.sites.get(city).ok_or("plano simulado ausente")?;
        let mut site=Site{records:old.building_records(city).to_vec(),cells:old.plan_cells(city).to_vec()};
        if before.u16(0x762)!=current.u16(0x762) || site.records.len()!=current_site.records.len(){return Err("lista de obras modificada; se necesita persistencia global".into())}
        let count=site.records.len()/8;let mut seen=vec![false;count];let mut at=usize::from(before.u16(0x762));
        while at<count {
            if seen[at]{return Err("ciclo en obras".into())}seen[at]=true;
            let old_rec=&site.records[at*8..at*8+8];let new_rec=&current_site.records[at*8..at*8+8];
            if old_rec[..6]!=new_rec[..6] || old_rec[7]!=new_rec[7]{return Err("obra finalizada o relocalizada; se necesita persistencia global".into())}
            let next_at=usize::from(u16_at(old_rec,4));site.records[at*8+6]=new_rec[6];at=next_at;
        }
        next.world.towns[city].apply_town(&before,&after,&site)?;
        for (i,office) in self.persisted.iter().enumerate() {
            if office.stock.town as usize!=city || self.host.employment_counts.get(i).copied().flatten().is_none() {continue}
            let dest=next.world.offices.get_mut(i).ok_or("oficina S05 ausente")?;
            *dest=office.to_save_v85().to_vec();
            let owner=office.stock.owner as usize;
            let record=next.world.merchants.get_mut(owner).ok_or("comerciante S05 ausente")?;
            let mut merchant=p3_state::office::PersistedMerchant::from_save_v85(record).ok_or("comerciante S05 truncado")?;
            merchant.merchant=self.rep_merchants.get(owner).ok_or("comerciante simulado ausente")?.clone();
            let live=self.economy.world.merchants.get(owner).ok_or("cuentas de comerciante ausentes")?;merchant.merchant.set_i32(0,live.gold);merchant.merchant.set_i32(0x4c8,live.income);
            *record=merchant.to_save_v85();
            let mut cursor=self.economy.world.offices.get(i).ok_or("oficina simulada ausente")?.first_facility;
            for input in self.businesses.get(i).ok_or("negocios de oficina ausentes")? {
                let k=cursor as usize;let mut b=*self.host.employment_businesses.get(k).ok_or("negocio simulado ausente")?;
                let f=self.economy.world.merchant_facilities.get(k).ok_or("plantilla simulada ausente")?;
                b[4..6].copy_from_slice(&f.employees.to_le_bytes());b[..4].copy_from_slice(&input.efficiency.to_le_bytes());
                *next.world.businesses.get_mut(k).ok_or("negocio S05 ausente")?=b.to_vec();cursor=f.next;
            }
            if usize::from(cursor)<self.economy.world.merchant_facilities.len(){return Err("cadena de negocios incompleta".into())}
        }
        *model=next;Ok(())
    }
    fn reconcile_pirate_band_image(&self,index:usize)->Result<p3_save::pirate_bands::PirateBand,String>{self.host.reconcile_pirate_band_image(index)}

    /// Imagen viva coherente de la banda; no ejecuta el tick naval.
    pub fn pirate_band_image(&self,index:usize)->Result<p3_save::pirate_bands::PirateBand,String>{
        self.reconcile_pirate_band_image(index)
    }

    /// Publica solo la parte de banda de una absorción ya calculada. El
    /// llamante debe integrar por separado barcos, convoy y demás efectos.
    pub fn apply_absorbed_pirate_band(&mut self,index:usize,before:&p3_save::pirate_bands::PirateBand,after:p3_save::pirate_bands::PirateBand)->Result<(),String>{
        let current=self.reconcile_pirate_band_image(index)?;
        if &current!=before{return Err("banda de absorción desincronizada".into())}
        after.validate().map_err(str::to_owned)?;
        if current.header[4..7]!=after.header[4..7]||current.header[12]!=after.header[12]{return Err("la absorción cambia campos de misión no admitidos".into())}
        let reserve=u16::from_le_bytes(after.header[14..16].try_into().unwrap());
        self.host.pirate_band_images.as_mut().unwrap()[index]=Some(after);
        self.host.mission_pirates.as_mut().unwrap()[index].as_mut().unwrap().reserve_strength=reserve;
        Ok(())
    }

    /// Concilia la imagen completa de una banda con la vista parcial que usa
    /// el planificador de misiones. Las vistas deben coincidir donde ambas han
    /// modificado el mismo campo; planos y piezas se reconcilian íntegramente.
    fn stage_pirate_band_images(&self,model:&mut p3_save::model::SaveModel)->Result<(),String>{
        let (initial,live)=match (&self.host.pirate_band_images_initial,&self.host.pirate_band_images){
            (None,None)=>return Ok(()),
            (Some(initial),Some(live))=>(initial,live),
            _=>return Err("imágenes iniciales o vivas de bandas ausentes".into()),
        };
        if initial.len()!=5||live.len()!=5{return Err("imágenes de bandas incompletas".into())}
        if self.host.mission_pirates.is_none()&&initial!=live{return Err("vista de misión de bandas ausente".into())}
        let saved=self.host.pirate_band_images_saved.borrow();
        if saved.as_ref().is_some_and(|bands|bands.len()!=5){return Err("checkpoint de bandas incompleto".into())}
        for i in 0..5 {
            let (Some(base),Some(now))=(&initial[i],&live[i]) else {
                if let (Some(base),None)=(&initial[i],&live[i]){
                    if self.host.mission_pirates.as_ref().is_none_or(|v|v[i].is_some()){return Err(format!("vista de misión de banda {i} no retirada"))}
                    if let Err(first)=p3_save::pirate_bands::replace_optional(&mut model.ships.tail,i,Some(base),None){
                        let last=saved.as_ref().and_then(|bands|bands[i].as_ref()).ok_or(first)?;
                        p3_save::pirate_bands::replace_optional(&mut model.ships.tail,i,Some(last),None).map_err(str::to_owned)?;
                    }
                }else if let (None,Some(_))=(&initial[i],&live[i]){
                    let desired=self.reconcile_pirate_band_image(i)?;
                    if let Err(first)=p3_save::pirate_bands::replace_optional(&mut model.ships.tail,i,None,Some(&desired)){
                        let last=saved.as_ref().and_then(|bands|bands[i].as_ref()).ok_or(first)?;
                        p3_save::pirate_bands::replace_optional(&mut model.ships.tail,i,Some(last),Some(&desired)).map_err(str::to_owned)?;
                    }
                }
                continue
            };
            let last=saved.as_ref().and_then(|bands|bands[i].as_ref());
            let desired=if self.host.mission_pirates.is_some(){self.reconcile_pirate_band_image(i)?}else{now.clone()};
            // Los campos compartidos sin cambios del plano/lista siguen la ruta
            // existente de misiones, que permite mezclar cambios independientes.
            let body_changed=|a:&p3_save::pirate_bands::PirateBand,b:&p3_save::pirate_bands::PirateBand|{
                a.plan!=b.plan||a.pieces!=b.pieces||(0..16).any(|j|!matches!(j,4|5|6|12|14|15)&&a.header[j]!=b.header[j])
            };
            if !body_changed(base,&desired)&&last.is_none_or(|band|!body_changed(band,&desired)){continue}
            if let Err(first)=p3_save::pirate_bands::replace(&mut model.ships.tail,i,base,&desired){
                let Some(last)=last else{return Err(first.into())};
                p3_save::pirate_bands::replace(&mut model.ships.tail,i,last,&desired).map_err(str::to_owned)?;
            }
        }
        Ok(())
    }

    /// Guarda una ciudad y el pool de negocios tras obras; no guarda el reloj ni los demás sistemas.
    pub fn persist_construction_city(&self,model:&mut p3_save::model::SaveModel,city:usize)->Result<(),String>{
        let mut staged=model.clone();
        self.stage_construction_city(&mut staged,city)?;
        self.confirm_pirate_band_save(&staged)?;
        *model=staged;
        Ok(())
    }
    /// Prepara la escritura de ciudad sin confirmar el checkpoint de bandas;
    /// el llamante debe hacerlo solo tras completar su transacción externa.
    pub fn stage_construction_city(&self,model:&mut p3_save::model::SaveModel,city:usize)->Result<(),String>{
        let mut staged=model.clone();self.apply_founded_cities(&mut staged)?;self.apply_created_offices(&mut staged)?;let raw=staged.world.write();let old=p3_save::world::World::parse(&raw).map_err(|e|format!("mundo: {e:?}"))?;
        if city>=old.town_count(){return Err("ciudad inexistente".into())}
        if self.persisted.len()!=staged.world.offices.len() || self.rep_merchants.len()!=staged.world.merchants.len() || self.businesses.len()!=self.economy.world.offices.len(){return Err("modelo incompatible con la simulación".into())}
        let before=old.town(city);let current=self.economy.world.towns.get(city).ok_or("ciudad ausente")?;
        staged.world.towns[city].apply_serialized_town(&before,current,self.host.sites.get(city).ok_or("plano ausente")?)?;
        staged.world.towns[city].update_tick_entries(self.economy.world.tick_entries.get(city).ok_or("suministros de ciudad ausentes")?)?;
        if self.host.debt_changed_towns.iter().chain(&self.host.hanse_financial_towns).chain(&self.host.expedition_changed_towns).chain(&self.host.investigation_changed_towns).chain(&self.host.mission_changed_towns).chain(&self.host.mission_changed_sites).any(|&i|i>=old.town_count()){return Err("ciudad modificada ausente".into())}
        for other in 0..old.town_count() {
            if other==city || (!self.host.debt_changed_towns.contains(&other) && !self.host.hanse_financial_towns.contains(&other) && !self.host.expedition_changed_towns.contains(&other) && !self.host.investigation_changed_towns.contains(&other) && !self.host.mission_changed_towns.contains(&other) && !self.host.mission_changed_sites.contains(&other)){continue}
            let prior=old.town(other);let mut after=prior.clone();let live=self.economy.world.towns.get(other).ok_or("ciudad de embargo ausente")?;
            if self.host.debt_changed_towns.contains(&other){for ware in 0..24 {after.set_stock(ware,live.stock(ware));}}
            if self.host.hanse_financial_towns.contains(&other){after.0[0x700..0x730].copy_from_slice(&live.0[0x700..0x730]);}
            if self.host.expedition_changed_towns.contains(&other){after.set_flags(live.flags());after.set_u8(0x6f0,live.u8(0x6f0));}
            if self.host.mission_changed_towns.contains(&other){after.set_u32(0x76c,live.u32(0x76c));}
            if self.host.investigation_changed_towns.contains(&other){after.0[0x6dc..0x6e0].copy_from_slice(&live.0[0x6dc..0x6e0]);}
            if self.host.mission_changed_sites.contains(&other){staged.world.towns[other].apply_serialized_town(&prior,live,self.host.sites.get(other).ok_or("plano ausente")?)?;}else{let site=Site{records:old.building_records(other).to_vec(),cells:old.plan_cells(other).to_vec()};staged.world.towns[other].apply_town(&prior,&after,&site)?;}
        }
        for (i,office) in self.persisted.iter().enumerate(){if old.office_image(i).is_none(){continue}let mut image=p3_save::office::decode_office(&office.to_save_v85());
            let w=self.economy.world.offices.get(i).ok_or("oficina ausente")?;image[0x2ca..0x2cc].copy_from_slice(&w.next.to_le_bytes());image[0x2cc..0x2ce].copy_from_slice(&w.first_facility.to_le_bytes());staged.world.offices[i]=p3_save::office::encode_office(&image).to_vec();}
        let mut pool=self.host.employment_businesses.clone();if pool.len()!=self.economy.world.merchant_facilities.len(){return Err("pool desincronizado".into())}
        for (r,f) in pool.iter_mut().zip(&self.economy.world.merchant_facilities){r[4..6].copy_from_slice(&f.employees.to_le_bytes());r[8..10].copy_from_slice(&f.next.to_le_bytes());}
        for (i,inputs) in self.businesses.iter().enumerate(){let mut cur=self.economy.world.offices.get(i).map_or(u16::MAX,|o|o.first_facility);for input in inputs{let r=pool.get_mut(cur as usize).ok_or("cadena de negocios incompleta")?;r[..4].copy_from_slice(&input.efficiency.to_le_bytes());cur=u16_at(r,8);}if usize::from(cur)<pool.len(){return Err("cadena de negocios incompleta".into())}}
        staged.world.businesses=pool.iter().map(|r|r.to_vec()).collect();staged.world.header[0x1c..0x1e].copy_from_slice(&self.host.business_free.to_le_bytes());
        for (i,m) in self.rep_merchants.iter().enumerate(){let record=staged.world.merchants.get_mut(i).ok_or("comerciante ausente")?;if self.host.merchant_ship_groups_changed.contains(&i){p3_save::world::apply_merchant_ship_groups(record,&m.bytes,self.host.merchant_ship_groups_initial.get(i).ok_or("grupos iniciales ausentes")?,self.host.merchant_ship_groups.get(i).ok_or("grupos vivos ausentes")?)?;}let mut saved=p3_state::office::PersistedMerchant::from_save_v85(record).ok_or("comerciante truncado")?;saved.merchant=m.clone();let live=self.economy.world.merchants.get(i).ok_or("cuentas de comerciante ausentes")?;saved.merchant.set_i32(0,live.gold);saved.merchant.set_i32(0x4c8,live.income);*record=saved.to_save_v85();}
        staged.message_pool=self.host.message_pool.clone();
        let mission_saved=self.host.mission_saved.borrow();
        let losses:Vec<Option<Vec<p3_save::tail::HuntLoss>>>=(0..self.host.council_scheduler.records.len()).map(|i|self.host.hunt_losses.get(&i).cloned()).collect();if self.host.hunt_losses.keys().any(|&i|i>=losses.len()){return Err("lista de bajas fuera del planificador".into())}
        staged.scheduled_tasks=p3_save::tail::update_scheduled_tasks_with_lists(&staged.scheduled_tasks,&self.host.council_scheduler,&self.host.mission_initial,mission_saved.as_deref(),&self.host.mission_variables,Some(&losses))?;
        // Si un fenómeno se guardó activo y caducó después, libéralo antes de
        // conciliar la fábrica. Solo se admite el registro guardado por esta sesión.
        if let (Some(nav),Some((saved,_)))=(self.host.navigation.as_ref(),self.host.saved_factory_checkpoint.borrow().as_ref()){
            for i in 0..saved.ships.len().min(nav.snapshot.ships.len()/0x180){let current=&nav.snapshot.ships[i*0x180..(i+1)*0x180];if u16_at(current,0x134)!=0xff||saved.ships[i].len()==5||staged.ships.ships.get(i)!=Some(&saved.ships[i]){continue}let prior=staged.ships.ship_image(i)?;if u16_at(&prior,0x134)==0x15{staged.ships.apply_expired_weather_ship(i,&prior,current)?}}
        }
        if let (Some(initial),Some(after))=(&self.host.expedition_factory_initial,&self.host.expedition_factory){
            let saved=self.host.saved_factory_checkpoint.borrow();let before=if self.host.detached_shipyard.is_none(){saved.as_ref().filter(|(ships,_)|ships==&staged.ships||ships==&model.ships).map_or(initial,|(_,factory)|factory)}else{initial};
            let routes=self.host.navigation.as_ref().map(|nav|nav.baseline.routes.as_slice());
            if let Some(yard)=&self.host.detached_shipyard {
                if self.host.navigation.is_some(){return Err("astillero pendiente antes de conectar la navegación".into())}
                if before!=&yard.factory_before{staged.ships.apply_hanse_factory_with_routes(before,&yard.factory_before,routes)?;}
                // Acepta archivos guardados en cualquier paso anterior de esta
                // transacción, pero no mezcla cambios ajenos en los registros.
                use p3_save::layout::{SHIP_CORE,SHIP_REST,SHIP_STATE,CONVOY,CONVOY_OBJECT_LEN};
                let mut touched_ships=vec![false;yard.before.ships.len()];let mut touched_entries=vec![false;yard.before.entries.len()];
                let mut scan=yard.before.clone();
                for delta in &yard.history{for (i,record) in &delta.ships{let at=i*0x180;for j in 0..0x180{touched_ships[at+j]|=scan.ships[at+j]!=record[j]}}for (i,record) in &delta.entries{let at=i*0x3c;for j in 0..0x3c{touched_entries[at+j]|=scan.entries[at+j]!=record[j]}}delta.apply(&mut scan)}
                let mut applied=None;let mut base=yard.before.clone();
                'candidates: for step in 0..=yard.history.len(){
                    if step>0{yard.history[step-1].apply(&mut base)}
                    if staged.ships.header.len()<14||u16_at(&staged.ships.header,8)!=base.docked_head||u16_at(&staged.ships.header,12)!=base.free_entry_head{continue}
                    for i in 0..usize::from(base.ship_count){let at=i*0x180;if !touched_ships[at..at+0x180].iter().any(|v|*v){continue}let saved=staged.ships.ship_image(i)?;for seg in SHIP_CORE.iter().chain(SHIP_REST.iter()).chain(std::iter::once(&SHIP_STATE)){for j in usize::from(seg.obj)..usize::from(seg.obj+seg.len){if touched_ships[at+j]&&saved[j]!=base.ships[at+j]{continue 'candidates}}}}
                    for i in 0..usize::from(base.entry_count){let at=i*CONVOY_OBJECT_LEN;let record=staged.ships.convoys.get(i).ok_or("entrada naval ausente")?;let saved=p3_save::layout::image(&CONVOY,CONVOY_OBJECT_LEN,record);for seg in &CONVOY{for j in usize::from(seg.obj)..usize::from(seg.obj+seg.len){if touched_entries[at+j]&&saved[j]!=base.entries[at+j]{continue 'candidates}}}}
                    let mut trial=staged.ships.clone();if trial.apply_shipyard_lists(&base,&yard.current).is_ok(){applied=Some(trial);break}
                }
                staged.ships=applied.ok_or("el astillero guardado no coincide con ningún estado de la sesión")?;
                let mut baseline=yard.factory_before.clone();
                for (dst,src) in baseline.ships.iter_mut().zip(yard.current.ships.chunks_exact(0x180)){dst.copy_from_slice(src)}
                for (dst,src) in baseline.convoys.iter_mut().zip(yard.current.entries.chunks_exact(0x3c)){dst.copy_from_slice(src)}
                baseline.convoy_head=yard.current.free_entry_head;
                if &baseline!=after{staged.ships.apply_hanse_factory_with_routes(&baseline,after,routes)?;}
            }else if before!=after{
                if let Some(nav)=&self.host.navigation {
                    let prior_routes=if let Some((ships,factory))=saved.as_ref().filter(|(ships,_)|ships==&staged.ships||ships==&model.ships) {
                        let bytes=ships.write();let parsed=p3_save::ships::Ships::parse(&bytes).map_err(|e|format!("rutas confirmadas: {e:?}"))?;
                        (0..factory.ships.len()).map(|i|if u16_at(&factory.ships[i],0x134)<0x17{parsed.saved_route(i).map(Some).map_err(str::to_string)}else{Ok(None)}).collect::<Result<Vec<_>,String>>()?
                    }else{
                        before.ships.iter().enumerate().map(|(i,s)|if u16_at(s,0x134)<0x17{nav.baseline.routes.get(i).cloned().ok_or_else(||"ruta original de fábrica ausente".to_owned())}else{Ok(None)}).collect::<Result<Vec<_>,String>>()?
                    };
                    staged.ships.apply_hanse_factory_navigation(before,after,&prior_routes,&nav.snapshot.routes,self.economy.world.towns.len())?;
                }else{staged.ships.apply_hanse_factory_with_routes(before,after,routes)?;}
            }
        }
        for &i in &self.host.debt_changed_ships {
            let before=staged.ships.ship_image(i)?;let mut after=before.clone();let cargo=self.economy.cargo_ships.get(i).ok_or("carga de barco ausente")?;
            for w in 0..24 {after[0x54+4*w..0x58+4*w].copy_from_slice(&cargo.stock[w].to_le_bytes());}
            after[0x114..0x118].copy_from_slice(&cargo.cargo_value.to_le_bytes());
            after[0x3c..0x3e].copy_from_slice(&cargo.dirty.to_le_bytes());
            staged.ships.apply_ship_image(i,&before,&after)?;
        }
        for &i in &self.host.tavern_changed_ships {
            let before = staged.ships.ship_image(i)?;
            let mut after = before.clone();
            let image = if let Some(nav) = self.host.navigation.as_ref() {
                nav.snapshot.ships.get(i*0x180..(i+1)*0x180).ok_or("barco de taberna ausente")?
            } else {
                self.host.expedition_ship_images.get(i).and_then(Option::as_deref).ok_or("barco de taberna ausente")?
            };
            // +0x118/+0x11C/+0x120 are derived load/equipment/armament caches.
            // The original loader recomputes them; they have no PAT fields.
            // This list also tracks transfers: persist stock and average prices
            // alongside crew, cargo value and equipment, retaining the live cache.
            for (start, end) in [(0x3c, 0x44), (0x54, 0x118), (0x154, 0x156)] {
                after[start..end].copy_from_slice(&image[start..end]);
            }
            after[0x157] = image[0x157];
            staged.ships.apply_ship_image(i, &before, &after)?;
        }
        for &i in &self.host.ai_fleet_changed_ships {let before=staged.ships.ship_image(i)?;let original=self.host.expedition_original_ship_images.get(i).and_then(Option::as_ref).ok_or("barco original de armado ausente")?;let image=if let Some(nav)=self.host.navigation.as_ref(){nav.snapshot.ships.get(i*0x180..(i+1)*0x180).ok_or("barco naval de armado ausente")?}else{self.host.expedition_ship_images.get(i).and_then(Option::as_deref).ok_or("barco de armado ausente")?};let checkpoint=self.host.saved_factory_checkpoint.borrow();let saved=checkpoint.as_ref().and_then(|(ships,_)|ships.ship_image(i).ok());let mut after=before.clone();for j in [0xf,0x136,0x13a]{if before[j]!=original[j]&&before[j]!=image[j]&&saved.as_ref().map(|ship|ship[j])!=Some(before[j]){return Err(format!("barco {i} armado modificado por otro sistema en +{j:#x}"))}after[j]=image[j]}staged.ships.apply_ship_image(i,&before,&after)?;}
        for &i in &self.host.expedition_changed_ships {let before=staged.ships.ship_image(i)?;let image=self.host.expedition_ship_images.get(i).and_then(Option::as_ref).ok_or("barco de expedición ausente")?;let original=self.host.expedition_original_ship_images.get(i).and_then(Option::as_ref).ok_or("barco original ausente")?;let mut after=before.clone();for (start,end) in [(6,10),(0x30,0x34),(0x134,0x136),(0x138,0x13a)]{for j in start..end{if image[j]!=original[j]{after[j]=image[j]}}}after[0x3c]|=image[0x3c]&!original[0x3c];staged.ships.apply_ship_image(i,&before,&after)?;}
        for &i in &self.host.expedition_changed_convoys {let image=self.host.expedition_convoy_images.get(i).ok_or("convoy de expedición ausente")?;let record=staged.ships.convoys.get_mut(i).ok_or("convoy de expedición ausente")?;for (start,end) in [(0xa,0xc),(0x12,0x14),(0x16,0x1c)]{record.get_mut(start..end).ok_or("convoy truncado")?.copy_from_slice(&image[start..end]);}}
        if !self.host.expedition_changed_ships.is_empty()||self.host.expedition_docked_head!=self.host.expedition_initial_docked_head{if staged.ships.header.len()<10{return Err("cabecera de barcos truncada".into())}let old=u16_at(&staged.ships.header,8);if old!=self.host.expedition_initial_docked_head && old!=self.host.expedition_docked_head{return Err("lista de atracados desincronizada".into())}staged.ships.header[8..10].copy_from_slice(&self.host.expedition_docked_head.to_le_bytes());}
        for &i in &self.host.debt_changed_convoys {let record=staged.ships.convoys.get_mut(i).ok_or("convoy ausente")?;let flags=*self.host.debt_convoy_flags.get(i).ok_or("banderas de convoy ausentes")?;record.get_mut(0x18..0x1c).ok_or("convoy truncado")?.copy_from_slice(&flags.to_le_bytes());}
        self.stage_pirate_band_images(&mut staged)?;
        if let (Some(before),Some(after))=(&self.host.mission_pirates_initial,&self.host.mission_pirates){
            let saved=self.host.pirate_bands_saved.borrow();
            if before!=after||saved.as_ref().is_some_and(|last|last!=after){
                let mut effective=*before;for i in 0..5{if effective[i].is_none(){effective[i]=after[i]}}
                if let Err(first)=p3_save::ships::update_pirate_band_statuses(&mut staged.ships.tail,&effective,after){
                    let Some(last)=saved.as_ref() else{return Err(first)};
                    p3_save::ships::update_pirate_band_statuses(&mut staged.ships.tail,last,after)?;
                }
            }
        }
        *model=staged;Ok(())
    }
    /// Emite el voto 0x49 de un comerciante durante una reunión activa.
    pub fn cast_council_vote(&mut self,merchant:usize,city:u8,choice:u32)->bool {
        let Some(town)=self.economy.world.towns.get(city as usize) else{return false};
        p3_core::economy::council::cast_vote(&mut self.host.council_scheduler,town,&self.rep_merchants,self.idle.clock.tick,merchant,city,choice)
    }
    /// Despacha la operación 0x3F para una votación de la Hansa activa.
    pub fn cast_hanse_vote(&mut self,merchant:usize,choice:u32)->bool {
        p3_core::economy::hanse_petition::cast_hanse_ballot(&mut self.host.council_scheduler,&self.economy.world.towns,&self.rep_merchants,self.idle.clock.tick,merchant,choice)
    }
    /// Conecta el contador de apellidos de la configuración de recursos del juego.
    /// Lee del ejecutable inglés los textos de la elección del Ältermann (`0x005299F0`).
    /// Configura las tablas verificadas del ejecutable de expediciones.
    pub fn configure_expedition_return_exe(&mut self,exe:&[u8])->Result<(),String>{self.host.expedition_return_resources=Some(expedition_return_resources::Resources::from_english_exe(exe)?);Ok(())}
    /// Posiciones de descubrimiento de los 30 centros comerciales (`0x673858`, pares de `i16`) del ejecutable inglés; las usa el plan de expedición con preferencia por centro.
    pub fn configure_discovery_locations(&mut self,exe:&[u8])->Result<(),String>{
        let image=p3_assets::exe::PeImage::parse(exe).map_err(str::to_owned)?;
        let mut locations=Vec::with_capacity(30);
        for i in 0..30u32{
            let at=0x673858+4*i;
            let word=|o:u32|image.byte_at(at+o).map_err(str::to_owned);
            locations.push((i16::from_le_bytes([word(0)?,word(1)?]),i16::from_le_bytes([word(2)?,word(3)?])));
        }
        self.host.discovery_locations=locations;self.host.mission_flat_rdata=Some(c10_script_flat::rdata_from_exe(exe)?);Ok(())
    }
    pub fn configure_hunt_texts(&mut self,exe:&[u8])->Result<(),String>{let texts=c08_hunt::HuntTexts::from_english_exe(exe)?;let image=p3_assets::exe::PeImage::parse(exe).map_err(str::to_owned)?;let bytes=(image.byte_at(0x677929).map_err(str::to_owned)?,image.byte_at(0x67792f).map_err(str::to_owned)?);self.host.hunt_texts=Some(texts);self.host.town_pirate_piece_bytes=Some(bytes);Ok(())}
    pub fn configure_election_texts(&mut self,exe:&[u8])->Result<(),String>{self.host.election_texts=Some(c08_text::ElectionTexts::from_english_exe(exe)?);Ok(())}
    pub fn configure_pirate_name_bank(&mut self,config:&[u8])->Result<(),String>{
        let count=p3_core::economy::hanse_petition::family_name_count(config).ok_or("falta Familiennamen en la configuración de nombres")?;
        self.host.hanse_pirates.as_mut().ok_or("bandas piratas del guardado no disponibles")?.default_portrait=Some(count);Ok(())
    }
    /// Carga los guiones privados correspondientes a los identificadores del S05.
    pub fn configure_mission_script_archives(&mut self,archives:&[&[u8]],config:&str)->Result<(),String>{
        self.host.mission_bank=Some(mission_bank::MissionBank::from_archives(archives,config)?);Ok(())
    }
    /// Carga nombres y puertos de los recursos CPR suministrados por el anfitrión.
    /// `name_config` selecciona la edición/idioma; no se mezclan bancos distintos.
    pub fn configure_hanse_fleet_archives(&mut self,archives:&[&[u8]],name_config:&str,hotseat:bool,player:u16)->Result<(),String>{
        let battle_tile_width=battle_opening::tile_width_from_archives(archives)?;let pirate_scene_projection=pirate_hideout_resources::projection_from_archives(archives)?;
        fn entry<'a>(archives:&[&'a [u8]],path:&str)->Result<&'a [u8],String>{for bytes in archives.iter().rev(){let archive=p3_assets::cpr::Archive::parse(bytes).map_err(|e|format!("archivo CPR: {e}"))?;if let Some(e)=archive.find(path.trim_start_matches("./").as_bytes()){return archive.read(e).ok_or("recurso comprimido o truncado".into())}}Err(format!("recurso ausente: {path}"))}
        fn fields(bytes:&[u8])->std::collections::BTreeMap<(String,String),String>{let mut out=std::collections::BTreeMap::new();let mut section=String::new();for raw in bytes.split(|&b|b==b'\n'||b==b'\r'){let line=String::from_utf8_lossy(raw);let line=line.trim();if line.starts_with('[')&&line.ends_with(']'){section=line[1..line.len()-1].to_ascii_lowercase()}else if let Some((k,v))=line.split_once('='){out.insert((section.clone(),k.trim().to_ascii_lowercase()),v.trim().to_owned());}}out}
        self.host.expedition_map_points.clear();
        if let Ok(data)=entry(archives,"scripts/Expeditionen.dat"){
            let count=(data.len()/4)&0xffff;
            self.host.expedition_map_points=data.chunks_exact(4).take(count).map(|b|(i16::from_le_bytes([b[0],b[1]]),i16::from_le_bytes([b[2],b[3]]))).collect();
        }
        let names_ini=fields(entry(archives,name_config)?);let get=|key:&str|names_ini.get(&("personennamen".to_owned(),key.to_owned())).ok_or_else(||format!("falta {key} en {name_config}"));let first_names=get("maennernamen")?.parse::<u16>().map_err(|_|"contador de nombres inválido")?;let last_names=get("familiennamen")?.parse::<u16>().map_err(|_|"contador de apellidos inválido")?;let count=get("schiffsnamen")?.parse::<usize>().map_err(|_|"contador de barcos inválido")?;let path=get("schiffefile")?;let raw=entry(archives,path)?;let mut ship_names=Vec::new();let mut at=0;while at<raw.len()&&ship_names.len()<count{let start=at;while at<raw.len()&&(raw[at]>=0x41||raw[at]==b' '||raw[at]==b'.'){at+=1}ship_names.push(raw[start..at].to_vec());if at<raw.len(){at+=1;while at<raw.len()&&raw[at]<0x41{at+=1}}}if first_names==0||last_names==0||count==0||count>307||ship_names.len()!=count{return Err("banco de nombres incompleto".into())}
        let family_names=(||->Result<Vec<Vec<u8>>,String>{
            let pirate_count=get("piratennamen")?.parse::<usize>().map_err(|_|"contador de piratas inválido")?;
            let total=usize::from(last_names).checked_add(pirate_count).ok_or("contador de apellidos excesivo")?;
            let data=entry(archives,get("familienfile")?)?;let mut parsed=Vec::new();let mut at=0;
            while at<data.len()&&parsed.len()<total{let start=at;while at<data.len()&&(data[at]>=0x41||data[at]==b' '||data[at]==b'.'){at+=1}parsed.push(data[start..at].to_vec());if at<data.len(){at+=1;while at<data.len()&&data[at]<0x41{at+=1}}}
            let custom=self.host.family_custom_names.as_ref().ok_or("apellidos personalizados ausentes")?;
            if total==0||parsed.len()!=total||custom.len()!=20{return Err("banco de apellidos incompleto".into())}
            parsed.extend(custom.iter().cloned());Ok(parsed)
        })().ok();
        let candidate_bank=(||->Result<(Vec<u8>,(u16,u16),[Vec<Vec<u8>>;2]),String>{
            let female=get("frauennamen")?.parse::<u16>().map_err(|_|"contador femenino inválido")?;
            if first_names==0||female==0||first_names>108||female>108{return Err("banco de candidatos fuera de rango".into())}
            let custom=self.host.first_custom_names.as_ref().ok_or("nombres personalizados ausentes")?;let mut lengths=vec![0u8;256];let mut banks=[Vec::new(),Vec::new()];
            for (group,(key,count)) in [("maennerfile",first_names),("frauenfile",female)].into_iter().enumerate(){
                let data=entry(archives,get(key)?)?;let mut parsed=Vec::new();let mut at=0;
                while at<data.len()&&parsed.len()<usize::from(count){let start=at;while at<data.len()&&(data[at]>=0x41||data[at]==b' '||data[at]==b'.'){at+=1}parsed.push(data[start..at].to_vec());if at<data.len(){at+=1;while at<data.len()&&data[at]<0x41{at+=1}}}
                if parsed.len()!=usize::from(count)||custom[group].len()!=20{return Err("banco de nombres incompleto".into())}
                let start=group*128;lengths[start..start+128].fill(parsed[0].len() as u8);for (i,name) in parsed.iter().chain(&custom[group]).enumerate(){lengths[start+i]=name.len() as u8}parsed.extend(custom[group].iter().cloned());banks[group]=parsed;
            }Ok((lengths,(first_names,female),banks))
        })().ok();
        if let Some((lengths,counts,banks))=candidate_bank{self.host.candidate_name_lengths=Some(lengths);self.host.candidate_look_counts=Some(counts);self.host.first_names=Some(banks)}else{self.host.candidate_name_lengths=None;self.host.candidate_look_counts=None;self.host.first_names=None}
        let town_data=entry(archives,"scripts/StadtDaten.ini")?;let locations=merchant_daily::parse_settlement_locations(town_data)?;let sea_directions=merchant_daily::parse_settlement_sea_directions(town_data)?;let inland=merchant_daily::parse_settlement_inland(town_data)?;let foundation_navigation=foundation_nav::FoundationNavigation::from_archives(archives,town_data)?;let town_names=town_names::TownNameBank::from_archives(archives)?;let mut ports=Vec::with_capacity(self.host.expedition_town_ids.len());for &id in &self.host.expedition_town_ids{let loc=locations.get(usize::from(id)).ok_or("puerto ausente")?;ports.push(Some((loc.x as u16,loc.y as u16)))}let plans=town_plans::TownPlanBank::from_archives(archives)?;self.host.pirate_scene_projection=pirate_scene_projection;self.host.settlement_sea_directions=Some(sea_directions);self.host.settlement_locations=Some(locations);self.host.settlement_inland=Some(inland);self.host.foundation_navigation=foundation_navigation;self.host.foundation_names=Some((first_names,last_names));self.host.family_names=family_names;self.host.town_names=town_names;self.host.settlement_plans=Some(plans);self.host.battle_tile_width=battle_tile_width;self.host.expedition_resources=Some(HanseFleetResources{ship_names,first_names,last_names,ports,hotseat,player});self.host.reminder_hotseat_gate=Some(u32::from(hotseat));if let Some(language)=name_config.rsplit_once('_').map(|(_,v)|v){let config=format!("scripts/missions_{language}");if entry(archives,&config).is_ok(){self.configure_mission_script_archives(archives,&config)?;}}Ok(())
    }
    /// Lee los 40 puntos de asentamiento sin exigir bancos de nombres.
    pub fn configure_alderman_map_archives(&mut self,archives:&[&[u8]])->Result<(),String>{
        for bytes in archives.iter().rev(){let archive=p3_assets::cpr::Archive::parse(bytes).map_err(|e|format!("archivo CPR: {e}"))?;if let Some(entry)=archive.find(b"scripts/StadtDaten.ini"){let data=archive.read(entry).ok_or("mapa comprimido o truncado")?;let pirate_scene_projection=pirate_hideout_resources::projection_from_archives(archives)?;let locations=merchant_daily::parse_settlement_locations(data)?;let sea_directions=merchant_daily::parse_settlement_sea_directions(data)?;let inland=merchant_daily::parse_settlement_inland(data)?;let foundation_navigation=foundation_nav::FoundationNavigation::from_archives(archives,data)?;let town_names=town_names::TownNameBank::from_archives(archives)?;let mut foundation_names=None;for names_bytes in archives.iter().rev(){let names_archive=p3_assets::cpr::Archive::parse(names_bytes).map_err(|e|format!("archivo CPR: {e}"))?;if let Some(names_entry)=names_archive.find(b"scripts/Namen_eng.ini"){foundation_names=Some(merchant_daily::parse_foundation_names(names_archive.read(names_entry).ok_or("nombres comprimidos o truncados")?)?);break}}let plans=town_plans::TownPlanBank::from_archives(archives)?;self.host.pirate_scene_projection=pirate_scene_projection;self.host.settlement_sea_directions=Some(sea_directions);self.host.settlement_locations=Some(locations);self.host.settlement_inland=Some(inland);self.host.foundation_navigation=foundation_navigation;self.host.foundation_names=foundation_names;self.host.town_names=town_names;self.host.settlement_plans=Some(plans);return Ok(())}}
        Err("falta scripts/StadtDaten.ini".into())
    }
    /// Busca la configuración en archivos CPR suministrados por el llamante.
    pub fn configure_pirate_name_archives(&mut self,archives:&[&[u8]])->Result<(),String>{
        for bytes in archives.iter().rev(){let archive=p3_assets::cpr::Archive::parse(bytes).map_err(|e|format!("archivo CPR: {e}"))?;
            if let Some(entry)=archive.find(b"scripts/Namen_eng.ini"){let config=archive.read(entry).ok_or("configuración de nombres comprimida o truncada")?;return self.configure_pirate_name_bank(config)}
        }Err("configuración de nombres ausente de los CPR".into())
    }
    /// Acepta una misión e inicia sus efectos en la misma transacción que la operación 0x4B.
    pub fn accept_alderman_mission(&mut self,op:&p3_core::pump::Operation)->Result<(),String>{
        use p3_core::ai::settlement::{accept_alderman_mission,start_alderman_road_mission,start_alderman_pirate_mission};
        if let Some(task)=self.host.council_scheduler.records.get(op.0[2] as usize){if task.kind==0x32&&task.u8_at(0xc)==0{let mut probe=self.host.council_scheduler.clone();if accept_alderman_mission(&mut probe,op,&self.rep_merchants,self.economy.world.merchant_count).map_err(str::to_string)?.is_none(){return Ok(())}return self.accept_founding_mission(op)}}
        let mut scheduler=self.host.council_scheduler.clone();let Some(index)=accept_alderman_mission(&mut scheduler,op,&self.rep_merchants,self.economy.world.merchant_count).map_err(str::to_string)? else{return Ok(())};let mut task=scheduler.records[usize::from(index)];let owner=usize::from(task.u8_at(0xd));let mut towns=self.economy.world.towns.clone();let mut pool=self.host.message_pool.clone();let mut merchant=self.rep_merchants.get(owner).ok_or("comerciante ausente")?.clone();if self.economy.merchants.get(owner).is_none(){return Err("comerciante económico ausente".into())}let date=p3_core::calendar::Calendar::for_tick(self.idle.clock.tick);let mut changed=[None;2];
        if task.u8_at(0xf)==0{match task.u8_at(0xc){
            0=>return Err("fundación de asentamiento aún no implementada".into()),
            1=>{let cities=[usize::from(task.u8_at(0x10)),usize::from(task.u8_at(0x11))];let (_,letter)=start_alderman_road_mission(&mut task,&mut towns,&self.host.expedition_town_ids,&self.rep_merchants,self.economy.world.merchant_count,self.economy.world.alderman_merchant,self.economy.world.alderman_town,self.idle.clock.tick).map_err(str::to_string)?;task.due=task.due.wrapping_sub(0x100);p3_state::messages::add_merchant_daily_record(&mut pool,&mut merchant,date.day,date.month,date.year,&letter)?;changed=[Some(cities[0]),Some(cities[1])]},
            2=>{task.set_u8_at(0xf,1);task.set_u32_at(8,self.idle.clock.tick.wrapping_add(0x1e00)|0xff)},
            3=>{let locations=self.host.settlement_locations.as_ref().ok_or("faltan posiciones de StadtDaten.ini")?;let (_,letters)=start_alderman_pirate_mission(&mut task,&self.rep_merchants,self.economy.world.merchant_count,self.economy.world.alderman_town,self.idle.clock.tick,locations).map_err(str::to_string)?;task.due=task.due.wrapping_sub(0x100);for letter in &letters{p3_state::messages::add_merchant_daily_record(&mut pool,&mut merchant,date.day,date.month,date.year,letter)?;}},
            4=>{let city=towns.get(usize::from(task.u8_at(0x12))).ok_or("ciudad de misión ausente")?;let value=city.u16(0x2d4).wrapping_add(2000)/100*100;task.set_u8_at(0xf,1);task.set_u32_at(8,self.idle.clock.tick.wrapping_add(0x16d00)|0xff);task.set_u8_at(0x10,value as u8);task.set_u8_at(0x11,(value>>8)as u8)},
            _=>unreachable!(),
        }}scheduler.records[usize::from(index)]=task;self.host.council_scheduler=scheduler;self.economy.world.towns=towns;self.economy.merchants[owner].bytes[10..12].copy_from_slice(&merchant.bytes[10..12]);self.rep_merchants[owner]=merchant;self.host.message_pool=pool;for city in changed.into_iter().flatten(){if !self.host.mission_changed_towns.contains(&city){self.host.mission_changed_towns.push(city)}}Ok(())
    }
    /// Aplica la operación 0x3B al planificador, existencias de oficina y flota viva.
    pub fn deliver_road_mission_wares(&mut self,op:&p3_core::pump::Operation)->Result<(),String>{
        use p3_core::ai::settlement::{deliver_road_materials,RoadDeliveryShip};
        let index=(op.0[1]&0xffff)as usize;if index>=usize::from(self.host.council_scheduler.capacity){return Ok(())}let mut task=*self.host.council_scheduler.records.get(index).ok_or("planificador truncado")?;let city=(op.0[2]&255)as u8;let owner=((op.0[2]>>8)&255)as u8;
        if usize::from(city)>=self.economy.world.towns.len()||task.kind!=0x32||task.u8_at(0xc)!=1||task.u8_at(0xd)!=owner||(task.u8_at(0x10)!=city&&task.u8_at(0x11)!=city)||task.u8_at(0x12)==0||u16::from(owner)>=self.economy.world.merchant_count{return Ok(())}
        let merchant=self.rep_merchants.get(usize::from(owner)).ok_or("comerciante ausente")?;if merchant.bytes.len()<0x10{return Err("comerciante truncado".into())}let head=merchant.u16(0xe);let matches=self.economy.offices.iter().enumerate().filter(|(_,o)|o.owner==u16::from(owner)&&o.town==city).map(|(i,_)|i).collect::<Vec<_>>();if matches.len()>1{return Err("oficinas duplicadas".into())}let office_index=matches.first().copied();let mut stock=office_index.map(|i|self.economy.offices[i].stock);
        let count=self.economy.cargo_ships.len();if self.host.expedition_ship_images.len()!=count||self.host.navigation.as_ref().is_some_and(|nav|nav.snapshot.ships.len()!=count*0x180){return Err("flota incompleta".into())}let mut ships=Vec::with_capacity(count);for i in 0..count{let image=if let Some(nav)=&self.host.navigation{nav.snapshot.ships.get(i*0x180..(i+1)*0x180)}else{self.host.expedition_ship_images[i].as_deref()};if let Some(image)=image{if image.len()<0x180{return Err("barco truncado".into())}let cargo=&self.economy.cargo_ships[i];if (0..24).any(|w|u32_at(image,0x54+4*w)as i32!=cargo.stock[w]){return Err("carga naval desincronizada".into())}ships.push(RoadDeliveryShip{next:u16_at(image,4),town:image[0x39],flags:image[0x136],stock:cargo.stock});}else{ships.push(RoadDeliveryShip{next:u16::MAX,town:0xff,flags:1,stock:[0;24]})}}
        let changed=deliver_road_materials(&mut task,op,self.economy.world.towns.len(),self.economy.world.merchant_count,stock.as_mut(),&mut ships,head).map_err(str::to_string)?;
        if let(Some(i),Some(stock))=(office_index,stock){let persisted=self.persisted.get_mut(i).ok_or("oficina persistida ausente")?;persisted.stock.stock=stock;self.economy.offices[i].stock=stock;}
        self.host.council_scheduler.records[index]=task;for i in changed{let cargo=&mut self.economy.cargo_ships[i];cargo.stock=ships[i].stock;cargo.recompute_load();if let Some(Some(image))=self.host.expedition_ship_images.get_mut(i){for w in 0..24{image[0x54+4*w..0x58+4*w].copy_from_slice(&cargo.stock[w].to_le_bytes())}for (at,value)in [(0x114,cargo.cargo_value),(0x118,cargo.cargo_weight),(0x11c,cargo.equipment_weight),(0x120,cargo.gun_power)]{image[at..at+4].copy_from_slice(&value.to_le_bytes())}}if let Some(nav)=self.host.navigation.as_mut(){let image=&mut nav.snapshot.ships[i*0x180..(i+1)*0x180];for w in 0..24{image[0x54+4*w..0x58+4*w].copy_from_slice(&cargo.stock[w].to_le_bytes())}for (at,value)in [(0x114,cargo.cargo_value),(0x118,cargo.cargo_weight),(0x11c,cargo.equipment_weight),(0x120,cargo.gun_power)]{image[at..at+4].copy_from_slice(&value.to_le_bytes())}}if !self.host.debt_changed_ships.contains(&i){self.host.debt_changed_ships.push(i)}}Ok(())
    }
    /// Responde a una oferta del casamentero y encola su operación de pago.
    pub fn answer_matchmaker_offer(&mut self,index:u16)->Result<(),String>{
        let pool=p3_save::tail::MessagePool::parse(&self.host.message_pool).map_err(|e|format!("pool: {e:?}"))?;if index>=pool.slots{return Err("carta fuera del depósito".into())}
        let letter=pool.letter(usize::from(index)).get(..16).ok_or("carta truncada")?.try_into().expect("longitud comprobada");let mut rng=self.host.investigation_rng.ok_or("estado CRT ausente")?;
        let operation=p3_core::economy::matchmaker::answer_offer(letter,u32::from(index),self.idle.clock.tick,self.economy.world.towns.len() as u32,self.economy.world.local_merchant,&mut rng)?;self.host.council_operations.push(operation);self.host.investigation_rng=Some(rng);Ok(())
    }
    /// Responde al dossier y encola la operación de boda `0x60`.
    pub fn answer_matchmaker_dossier(&mut self,index:u16)->Result<(),String>{
        let pool=p3_save::tail::MessagePool::parse(&self.host.message_pool).map_err(|e|format!("pool: {e:?}"))?;if index>=pool.slots{return Err("carta fuera del depósito".into())}
        let letter=pool.letter(usize::from(index)).get(..16).ok_or("carta truncada")?.try_into().expect("longitud comprobada");let mut rng=self.host.investigation_rng.ok_or("estado CRT ausente")?;
        let operation=p3_core::economy::matchmaker::answer_dossier(letter,self.idle.clock.tick,self.economy.world.towns.len() as u32,self.economy.world.local_merchant,&mut rng)?;self.host.council_operations.push(operation);self.host.investigation_rng=Some(rng);Ok(())
    }
    fn accept_supply_order(&mut self,op:p3_core::pump::Operation)->Result<(),String>{
        let mut towns=self.economy.world.towns.clone();let mut scheduler=self.host.council_scheduler.clone();let mut board=self.host.town_hall_notices.ok_or("tablón urbano ausente")?;let date=self.idle.clock.calendar;let Some(notice)=p3_core::economy::supply_order::accept(op,self.idle.clock.tick,date.year,&mut towns,&mut scheduler,&mut board)? else{return Ok(())};
        let mut pool=self.host.message_pool.clone();let mut person=None;if notice.recipient<usize::from(self.economy.world.merchant_count){let owner=notice.recipient;let mut m=self.rep_merchants.get(owner).ok_or("comerciante de encargo ausente")?.clone();if self.economy.merchants.get(owner).is_none(){return Err("vista económica de encargo ausente".into())}p3_state::messages::add_supply_notice(&mut pool,&mut m,date.day,date.month,date.year,&notice.record)?;person=Some((owner,m))}
        if let Some((owner,m))=person{self.rep_merchants[owner]=m.clone();self.economy.merchants[owner]=m}self.economy.world.towns=towns;self.host.mission_variables.resize(usize::from(scheduler.capacity),None);self.host.council_scheduler=scheduler;self.host.town_hall_notices=Some(board);self.host.message_pool=pool;Ok(())
    }
    fn deliver_supply_order(&mut self,mut op:p3_core::pump::Operation)->Result<(),String>{
        use p3_core::ai::settlement::RoadDeliveryShip;
        let owner=op.0[1] as usize;let index=op.0[3] as usize;if owner>=usize::from(self.economy.world.merchant_count)||index>=usize::from(self.host.council_scheduler.capacity){return Ok(())}let task=self.host.council_scheduler.records.get(index).ok_or("planificador truncado")?;let city=task.u8_at(8);if task.kind!=0x11||usize::from(city)>=self.economy.world.towns.len()||task.u8_at(9)>=20||usize::from(task.u8_at(0x16))!=owner{return Ok(())}
        let mut person=self.rep_merchants.get(owner).ok_or("comerciante ausente")?.clone();if self.economy.merchants.get(owner).is_none()||self.economy.world.merchants.get(owner).is_none(){return Err("vistas del comerciante ausentes".into())}let head=person.u16(0xe);
        let matches=self.economy.offices.iter().enumerate().filter(|(_,o)|usize::from(o.owner)==owner&&o.town==city).map(|(i,_)|i).collect::<Vec<_>>();if matches.len()>1{return Err("oficinas duplicadas".into())}let office_index=matches.first().copied();if office_index.is_some_and(|i|i>=self.persisted.len()){return Err("oficina persistida ausente".into())}let mut stock=office_index.map(|i|self.economy.offices[i].stock);
        let count=self.economy.cargo_ships.len();if self.host.expedition_ship_images.len()!=count||self.host.navigation.as_ref().is_some_and(|nav|nav.snapshot.ships.len()!=count*0x180){return Err("flota incompleta".into())}let mut ships=Vec::with_capacity(count);for i in 0..count{let image=if let Some(nav)=&self.host.navigation{nav.snapshot.ships.get(i*0x180..(i+1)*0x180)}else{self.host.expedition_ship_images[i].as_deref()};if let Some(image)=image{if image.len()<0x180{return Err("barco truncado".into())}let cargo=&self.economy.cargo_ships[i];if (0..24).any(|w|u32_at(image,0x54+4*w)as i32!=cargo.stock[w]){return Err("carga naval desincronizada".into())}ships.push(RoadDeliveryShip{next:u16_at(image,4),town:image[0x39],flags:image[0x136],stock:cargo.stock});}else{ships.push(RoadDeliveryShip{next:u16::MAX,town:0xff,flags:1,stock:[0;24]})}}
        let mut scheduler=self.host.council_scheduler.clone();let mut towns=self.economy.world.towns.clone();let changed=p3_core::economy::supply_delivery::deliver(&mut op,&mut scheduler,&mut towns,&mut person,stock.as_mut(),&mut ships,head)?;
        for &i in &changed{let image=if let Some(nav)=&self.host.navigation{&nav.snapshot.ships[i*0x180..(i+1)*0x180]}else{self.host.expedition_ship_images[i].as_deref().ok_or("barco de entrega ausente")?};let convoy=usize::from(u16_at(image,8));if convoy<self.host.expedition_convoy_images.len(){if self.host.expedition_convoy_images[convoy].len()<0x1c||convoy>=self.host.debt_convoy_flags.len(){return Err("convoy de entrega truncado".into())}if self.host.navigation.as_ref().is_some_and(|n|n.snapshot.convoys.len()<(convoy+1)*0x3c){return Err("convoy de navegación truncado".into())}}}

        if let(Some(i),Some(stock))=(office_index,stock){self.persisted[i].stock.stock=stock;self.economy.offices[i].stock=stock;}self.host.council_scheduler=scheduler;self.economy.world.towns=towns;self.economy.world.merchants[owner].gold=person.i32(0);self.rep_merchants[owner]=person.clone();self.economy.merchants[owner]=person;
        for i in changed{let cargo=&mut self.economy.cargo_ships[i];cargo.stock=ships[i].stock;cargo.recompute_load();cargo.dirty|=4;if let Some(Some(image))=self.host.expedition_ship_images.get_mut(i){image[0x3c]|=4;for w in 0..24{image[0x54+4*w..0x58+4*w].copy_from_slice(&cargo.stock[w].to_le_bytes())}for (at,value)in [(0x114,cargo.cargo_value),(0x118,cargo.cargo_weight),(0x11c,cargo.equipment_weight),(0x120,cargo.gun_power)]{image[at..at+4].copy_from_slice(&value.to_le_bytes())}}if let Some(nav)=self.host.navigation.as_mut(){let image=&mut nav.snapshot.ships[i*0x180..(i+1)*0x180];image[0x3c]|=4;for w in 0..24{image[0x54+4*w..0x58+4*w].copy_from_slice(&cargo.stock[w].to_le_bytes())}for (at,value)in [(0x114,cargo.cargo_value),(0x118,cargo.cargo_weight),(0x11c,cargo.equipment_weight),(0x120,cargo.gun_power)]{image[at..at+4].copy_from_slice(&value.to_le_bytes())}}let convoy=if let Some(nav)=&self.host.navigation{usize::from(u16_at(&nav.snapshot.ships,i*0x180+8))}else{usize::from(u16_at(self.host.expedition_ship_images[i].as_ref().expect("validado"),8))};if convoy<self.host.expedition_convoy_images.len(){let image=&mut self.host.expedition_convoy_images[convoy];image[0x18]|=4;self.host.debt_convoy_flags[convoy]|=4;if let Some(nav)=self.host.navigation.as_mut(){nav.snapshot.convoys[convoy*0x3c+0x18]|=4}if !self.host.debt_changed_convoys.contains(&convoy){self.host.debt_changed_convoys.push(convoy)}}if !self.host.debt_changed_ships.contains(&i){self.host.debt_changed_ships.push(i)}}Ok(())
    }
    fn schedule_wedding(&mut self,op:p3_core::pump::Operation)->Result<(),String>{
        if op.0[1]<=self.idle.clock.tick{return Ok(())}let count=usize::from(self.economy.world.merchant_count);if count>self.rep_merchants.len(){return Err("comerciantes incompletos".into())}
        let parsed=p3_save::tail::MessagePool::parse(&self.host.message_pool).map_err(|e|format!("pool: {e:?}"))?;let mut messages=Vec::with_capacity(usize::from(parsed.slots));for i in 0..usize::from(parsed.slots){messages.push(parsed.letter(i).get(..16).ok_or("carta truncada")?.try_into().expect("longitud comprobada"))}
        let mut pool=self.host.message_pool.clone();let mut scheduler=self.host.council_scheduler.clone();let mut merchants=self.rep_merchants[..count].to_vec();let(offer,clear)=p3_core::economy::matchmaker::schedule_wedding(op,self.idle.clock.tick,&merchants,&mut scheduler,&messages)?;
        for(index,width)in clear{let at=parsed.letters[index].range.start;if width==2{pool[at+14..at+16].fill(0)}else{pool[at+15]=0}}
        if let Some(offer)=offer{let owner=offer.recipient;if owner>=self.economy.merchants.len(){return Err("vista económica de comerciante ausente".into())}let date=self.idle.clock.calendar;p3_state::messages::add_matchmaker_offer(&mut pool,&mut merchants[owner],date.day,date.month,date.year,&offer.record)?;self.rep_merchants[owner]=merchants[owner].clone();self.economy.merchants[owner]=merchants[owner].clone()}
        self.host.message_pool=pool;self.host.council_scheduler=scheduler;Ok(())
    }
    fn pay_matchmaker_fee(&mut self,op:p3_core::pump::Operation)->Result<(),String>{
        let owner=(op.0[3]>>24) as usize;let count=usize::from(self.economy.world.merchant_count);if owner>=count{return Ok(())}
        if count>self.rep_merchants.len()||owner>=self.economy.merchants.len()||owner>=self.economy.world.merchants.len(){return Err("vistas de comerciante incompletas".into())}
        let parsed=p3_save::tail::MessagePool::parse(&self.host.message_pool).map_err(|e|format!("pool: {e:?}"))?;let mut messages=Vec::with_capacity(usize::from(parsed.slots));for i in 0..usize::from(parsed.slots){messages.push(parsed.letter(i).get(..16).ok_or("carta truncada")?.try_into().expect("longitud comprobada"))}
        let mut pool=self.host.message_pool.clone();let mut scheduler=self.host.council_scheduler.clone();let mut merchants=self.rep_merchants[..count].to_vec();let matrix=self.host.founded_world.as_ref().map(|w|w.matrix.as_slice());
        let (late,clear)=p3_core::economy::matchmaker::pay_fee(op,self.idle.clock.tick,self.economy.world.towns.len(),matrix,&mut merchants,&mut scheduler,&messages)?;
        if let Some(index)=clear{let at=parsed.letters[index].range.start;pool[at+14..at+16].fill(0)}
        if let Some(offer)=late{let date=self.idle.clock.calendar;p3_state::messages::add_matchmaker_offer(&mut pool,&mut merchants[owner],date.day,date.month,date.year,&offer.record)?;}
        self.rep_merchants[owner]=merchants[owner].clone();self.economy.merchants[owner]=merchants[owner].clone();self.economy.world.merchants[owner].gold=merchants[owner].i32(0);self.host.message_pool=pool;self.host.council_scheduler=scheduler;Ok(())
    }
    /// Despacha una pasada de hasta 52 operaciones; las generadas durante la
    /// pasada y los registros que no caben esperan a la siguiente.
    pub fn dispatch_pending_council_operations(&mut self) -> Vec<String> {
        let mut errors=Vec::new();
        let mut packet=std::mem::take(&mut self.host.council_operations);
        if packet.len()>52 {self.host.council_operations=packet.split_off(52)}
        let mut pending=packet.into_iter();
        while let Some(mut op)=pending.next() {
            if op.0[0]==p3_core::pump::OP_ADVANCE_BATTLES {
                self.host.council_operations.splice(0..0,std::iter::once(op).chain(pending));
                errors.push("operación 0x94 requiere contexto de la escena".into());
                break
            }
            if op.0[0]==p3_core::pump::OP_SET_TIME_MODE {
                self.host.council_operations.splice(0..0,std::iter::once(op).chain(pending));
                errors.push("operación 0xC8 requiere contexto del modo de tiempo".into());
                break
            }
            if op.0[0]==0xb4 {
                // El preparador valida y consume la cabeza solo al publicar
                // sus vistas; las demás operaciones del paquete esperan.
                self.host.council_operations.insert(0,op);
                if let Err(e)=self.dispatch_live_pirate_b4(op){
                    self.host.council_operations.splice(1..1,pending);
                    errors.push(format!("operación B4: {e}"));
                    break
                }
                continue
            }
            if op.0[0]==0xb5 {
                let Some(bands)=self.host.pirate_band_images.as_ref().filter(|v|v.len()==5) else{self.host.council_operations.splice(0..0,std::iter::once(op).chain(pending));errors.push("operación B5: bandas ausentes".into());break};
                let present=std::array::from_fn(|i|bands[i].is_some());
                if p3_core::economy::pirate_b5::decode(op,present).is_none(){continue}
                if let Err(e)=self.apply_pirate_b5_operation(op){self.host.council_operations.splice(0..0,std::iter::once(op).chain(pending));errors.push(format!("operación B5: {e}"));break}continue
            }
            if let Some(result)=self.apply_siege_record_operation(op){if let Err(e)=result{self.host.council_operations.splice(0..0,std::iter::once(op).chain(pending));errors.push(format!("operación de asedio {:#x}: {e}",op.0[0]));break}continue}
            if let Some(result)=self.apply_c08_operation(op){if let Err(e)=result{self.host.council_operations.splice(0..0,std::iter::once(op).chain(pending));errors.push(format!("operación C08 {:#x}: {e}",op.0[0]));break}continue}

            if matches!(op.0[0],0x1e|0x1f){if let Err(e)=self.apply_convoy_expedition_operation(op){self.host.council_operations.splice(0..0,std::iter::once(op).chain(pending));errors.push(format!("expedición {:#x}: {e}",op.0[0]));break}continue}

            if op.0[0]==0x3a {if let Err(e)=self.start_land_transport(op){self.host.council_operations.splice(0..0,std::iter::once(op).chain(pending));errors.push(format!("transporte terrestre 0x3A: {e}"));break}continue}
            if op.0[0]==0x36 {if let Err(e)=self.register_ship_for_auction(op){self.host.council_operations.splice(0..0,std::iter::once(op).chain(pending));errors.push(format!("barco a subasta 0x36: {e}"));break}continue}
            if op.0[0]==0x2c {if let Err(e)=self.apply_ai_ship_order(op){self.host.council_operations.splice(0..0,std::iter::once(op).chain(pending));errors.push(format!("pedido naval 0x2C: {e}"));break}continue}
            if op.0[0]==0x78 {if let Err(e)=self.spawn_weather_operation(op){self.host.council_operations.splice(0..0,std::iter::once(op).chain(pending));errors.push(format!("alta meteorológica 0x78: {e}"));break}continue}
            if op.0[0]==0x37 {if let Err(e)=self.apply_guild_operation(op){self.host.council_operations.splice(0..0,std::iter::once(op).chain(pending));errors.push(format!("ingreso en gremio 0x37: {e}"));break}continue}
            if op.0[0]==0x3d {if let Err(e)=self.accept_supply_order(op){self.host.council_operations.splice(0..0,std::iter::once(op).chain(pending));errors.push(format!("aceptación de encargo 0x3D: {e}"));break}continue}
            if op.0[0]==0x51 {if let Err(e)=self.deliver_supply_order(op){self.host.council_operations.splice(0..0,std::iter::once(op).chain(pending));errors.push(format!("entrega de encargo 0x51: {e}"));break}continue}
            if op.0[0]==0x60 {if let Err(e)=self.schedule_wedding(op){self.host.council_operations.splice(0..0,std::iter::once(op).chain(pending));errors.push(format!("programación de boda 0x60: {e}"));break}continue}
            if op.0[0]==0x72 {if let Err(e)=self.pay_matchmaker_fee(op){self.host.council_operations.splice(0..0,std::iter::once(op).chain(pending));errors.push(format!("pago del casamentero 0x72: {e}"));break}continue}
            if op.0[0]==0x87 {if let Err(e)=self.apply_candidate_operation(op){self.host.council_operations.splice(0..0,std::iter::once(op).chain(pending));errors.push(format!("operación de candidato 0x87: {e}"));break}continue}
            if op.0[0]==0x64 {if let Err(e)=p3_core::economy::ten_day::apply_warehouse_theft(op,&mut self.host.council_scheduler,self.idle.clock.tick){self.host.council_operations.splice(0..0,std::iter::once(op).chain(pending));errors.push(format!("operación de robo 0x64: {e}"));break}continue}
            if op.0[0]==0x7e {
                let calendar=self.idle.clock.calendar;
                match p3_core::economy::town_event_next_year::prepare(&self.host.council_scheduler,&self.economy.world.towns,calendar.year,calendar.day_of_year,op){
                    Ok(scheduler)=>{self.host.mission_variables.resize(usize::from(scheduler.capacity),None);self.host.council_scheduler=scheduler},
                    Err(e)=>{self.host.council_operations.splice(0..0,std::iter::once(op).chain(pending));errors.push(format!("plan anual urbano 0x7E: {e}"));break}
                }
                continue
            }
            if op.0[0]==0x84 {if let Err(e)=p3_core::economy::ten_day::apply_notable_demand(op,&mut self.host.council_scheduler,self.idle.clock.tick){self.host.council_operations.splice(0..0,std::iter::once(op).chain(pending));errors.push(format!("operación de petición 0x84: {e}"));break}continue}
            if op.0[0]==0x38 {if let Err(e)=p3_core::economy::ten_day::apply_year_auction(op,&mut self.host.council_scheduler,self.idle.clock.tick,self.economy.world.towns.len() as u32){self.host.council_operations.splice(0..0,std::iter::once(op).chain(pending));errors.push(format!("operación de subasta 0x38: {e}"));break}continue}
            if op.0[0]==0x7d {if let Err(e)=p3_core::economy::ten_day::apply_fire(op,&mut self.host.council_scheduler,self.idle.clock.tick){self.host.council_operations.splice(0..0,std::iter::once(op).chain(pending));errors.push(format!("operación de incendio 0x7D: {e}"));break}continue}
            if op.0[0]==0x7c {let c=self.idle.clock.calendar;if let Err(e)=p3_core::economy::ten_day::apply_plague(op,&mut self.host.council_scheduler,c.day_of_year,c.year,self.idle.clock.tick){self.host.council_operations.splice(0..0,std::iter::once(op).chain(pending));errors.push(format!("operación de peste 0x7C: {e}"));break}continue}
            if op.0[0]==0x61 {let owner=op.0[1] as usize;let count=usize::from(self.economy.world.merchant_count);if owner>=count{continue}let result=if count>self.rep_merchants.len()||owner>=self.economy.merchants.len()||owner>=self.economy.world.merchants.len(){Err("vistas de comerciante incompletas")}else{p3_core::economy::celebration::apply(op,&mut self.rep_merchants[..count],&mut self.host.council_scheduler)};if let Err(e)=result{self.host.council_operations.splice(0..0,std::iter::once(op).chain(pending));errors.push(format!("operación de fiesta 0x61: {e}"));break}self.economy.merchants[owner]=self.rep_merchants[owner].clone();self.economy.world.merchants[owner].gold=self.rep_merchants[owner].i32(0);continue}
            if op.0[0]==0x79 {p3_core::economy::ten_day::apply_town_slots(&mut self.economy.world.towns,op);continue}
            if op.0[0]==0x7a {p3_core::economy::ten_day::apply_tavern_passenger(&mut self.economy.world.towns,self.economy.world.merchant_count,op);continue}
            if matches!(op.0[0],0x12|0x13|0x67) {if let Err(e)=self.apply_captain_operation(op){self.host.council_operations.splice(0..0,std::iter::once(op).chain(pending));errors.push(format!("operación de capitán {:#x}: {e}",op.0[0]));break}continue}
            if op.0[0]==0x83 {if let Err(e)=p3_core::economy::spice::apply(&mut self.host.council_scheduler,op){self.host.council_operations.splice(0..0,std::iter::once(op).chain(pending));errors.push(format!("llegada de especias 0x83: {e}"));break}continue}

            if op.0[0]==0x7b {p3_core::economy::loan_slots::apply(&mut self.economy.world.towns,op);continue}

            if op.0[0]==0x8b {if let Err(e)=self.create_siege_request(op.0){self.host.council_operations.splice(0..0,std::iter::once(op).chain(pending));errors.push(format!("petición de asedio 0x8B: {e}"));break}continue}
            if op.0[0]==0x8e {
                let city=op.0[1] as usize;
                let scene=self.host.founded_world.as_ref().and_then(|w|w.sieges.get(city)).ok_or("registro de asedio ausente").and_then(|r|p3_save::world::siege_has_scene(r,city).map_err(|_|"registro de asedio inválido"));
                let gate=match (self.economy.world.towns.get(city),self.host.sites.get(city),scene){
                    (Some(town),Some(site),Ok(scene))=>p3_core::economy::siege::start_gate(town,site,scene),
                    _=>Err("ciudad de asedio ausente"),
                };
                match gate{Ok(false)=>continue,Ok(true)=>{},Err(e)=>{self.host.council_operations.splice(0..0,std::iter::once(op).chain(pending));errors.push(format!("inicio de asedio: {e}"));break}}
                let result=self.start_siege(city,op.0[2] as u8,op.0[3] as u8,op.0[4] as u8);
                if let Err(e)=result{self.host.council_operations.splice(0..0,std::iter::once(op).chain(pending));errors.push(format!("inicio de asedio 0x8E: {e}"));break}continue
            }
            if op.0[0]==0x24 {
                let city=op.0[2] as usize;
                if city<self.economy.world.towns.len(){if let Err(e)=self.start_wall_work(city,op.0[1],op.0[3]){self.host.council_operations.splice(0..0,std::iter::once(op).chain(pending));errors.push(format!("obra de muralla: {e}"));break}}
                continue
            }
            if op.0[0]==0x23 {
                let city=op.0[2] as usize;
                let result=match (self.economy.world.towns.get_mut(city),self.host.sites.get_mut(city)) {
                    (Some(town),Some(site))=>civic_planner::wall(town,site,self.economy.world.merchant_count,&op),
                    _=>Err("ciudad o plano de muralla ausente".into()),
                };
                if let Err(e)=result {self.host.council_operations.splice(0..0,std::iter::once(op).chain(pending));errors.push(e);break}
                if !self.host.mission_changed_sites.contains(&city){self.host.mission_changed_sites.push(city)}
                continue
            }
            if op.0[0]==0x27 {
                let city=usize::from(op.0[1] as u8);
                if let Some(town)=self.economy.world.towns.get_mut(city) {
                    if let Err(e)=civic_planner::supply(town,&op){self.host.council_operations.splice(0..0,std::iter::once(op).chain(pending));errors.push(e);break}
                }
                continue
            }
            if op.0[0]==0x22 {
                let bytes=op.0[1].to_le_bytes();let city=usize::from(bytes[1]);
                if op.0[2].to_le_bytes()[0]==0x28 {
                    if let Err(e)=self.start_well_work(city,&op){self.host.council_operations.splice(0..0,std::iter::once(op).chain(pending));errors.push(format!("obra de pozo: {e}"));break}
                    continue
                }
                if (0x04..=0x1d).contains(&op.0[2].to_le_bytes()[0])&&u16::from(bytes[0])<self.economy.world.merchant_count {
                    if let Err(e)=self.start_well_work(city,&op){self.host.council_operations.splice(0..0,std::iter::once(op).chain(pending));errors.push(format!("obra privada: {e}"));break}
                    continue
                }
                if (0x30..=0x35).contains(&op.0[2].to_le_bytes()[0]) {
                    if let Err(e)=self.start_tower_work(city,&op){self.host.council_operations.splice(0..0,std::iter::once(op).chain(pending));errors.push(format!("obra defensiva: {e}"));break}
                    continue
                }
                let result=match (self.economy.world.towns.get_mut(city),self.host.sites.get_mut(city)) {
                    (Some(town),Some(site))=>crate::lender::start(town,site,self.economy.world.merchant_count,&op),
                    _=>Err("ciudad o plano ausente".into()),
                };
                match result {Ok(true)=>{if !self.host.mission_changed_sites.contains(&city){self.host.mission_changed_sites.push(city)}},Ok(false)=>{},Err(e)=>{self.host.council_operations.splice(0..0,std::iter::once(op).chain(pending));errors.push(format!("obra municipal: {e}"));break}}continue
            }
            if op.0[0]==0x46 {
                let merchant=op.0[1] as usize;let city=op.0[2] as usize;
                if merchant<usize::from(self.economy.world.merchant_count) && city<self.economy.world.towns.len() && u16::from(self.economy.world.towns[city].u8(0x6f1))>=self.economy.world.merchant_count {
                    let m=&self.rep_merchants[merchant];let home=usize::from(m.bytes[0x19]);
                    if m.bytes[0x39c+home]>=5 || op.0[4]==0xbadeaffe {self.economy.world.towns[city].set_u8(0x6f1,merchant as u8);if !self.host.mission_changed_sites.contains(&city){self.host.mission_changed_sites.push(city)}}
                }
                continue
            }
            if op.0[0]==0x4b {if let Err(e)=self.accept_alderman_mission(&op){self.host.council_operations.splice(0..0,std::iter::once(op).chain(pending));errors.push(format!("aceptación de misión: {e}"));break}continue}
            if op.0[0]==0x3b {if let Err(e)=self.deliver_road_mission_wares(&op){self.host.council_operations.splice(0..0,std::iter::once(op).chain(pending));errors.push(format!("entrega terrestre: {e}"));break}continue}
            if op.0[0]==0x81 {p3_core::economy::hanse_expedition::dispatch_investigation(&op,self.idle.clock.tick,&self.rep_merchants,&mut self.host.council_scheduler);continue}
            if op.0[0]==0x85 {if let Err(e)=p3_core::ai::settlement::schedule_unique_alderman_task(&mut self.host.council_scheduler,self.idle.clock.tick,&op){self.host.council_operations.splice(0..0,std::iter::once(op).chain(pending));errors.push(format!("misión del alcalde: {e}"));break}continue}
            if op.0[0]==0x86 {
                let result=(||->Result<(),String>{
                    let targets=self.host.pirate_targets.as_ref().ok_or("destinos piratas ausentes")?;
                    let factory=self.host.expedition_factory.as_ref().ok_or("flota ausente")?;
                    let ships=if let Some(nav)=&self.host.navigation{if nav.snapshot.ships.len()%0x180!=0{return Err("flota truncada".into())}nav.snapshot.ships.chunks_exact(0x180).map(|s|s.to_vec()).collect()}else{factory.ships.clone()};
                    let world=p3_core::ai::settlement::AldermanMissionWorld{notification_window_active:false,merchant_count:self.economy.world.merchant_count,merchants:&self.rep_merchants,tick:self.idle.clock.tick,day_of_year:self.economy.world.day_of_year,towns:&self.economy.world.towns,town_ids:&self.host.expedition_town_ids,ships:&ships,captains:&factory.captains,pirate_targets:targets};
                    p3_core::ai::settlement::invalidate_alderman_missions(&mut self.host.council_scheduler,&world).map_err(str::to_string)?;Ok(())
                })();
                if let Err(e)=result{self.host.council_operations.splice(0..0,std::iter::once(op).chain(pending));errors.push(format!("validación de misiones: {e}"));break}continue
            }
            let city=(op.0[3]&255) as usize;
            if let Some(town)=self.economy.world.towns.get_mut(city) {
                p3_core::economy::council::dispatch_offer(town,self.idle.clock.tick,&mut self.host.council_scheduler,&mut op);
            } else {errors.push(format!("oferta del concejo: ciudad {city} inexistente"));}
        }
        errors
    }
    pub fn from_save_bytes(file: &[u8]) -> Result<Sim, SimError> {
        let save = Save::parse(file).map_err(|e| SimError::Save(e.to_string()))?;
        let data = save.block_named("game_world").ok_or_else(|| SimError::Save("falta game_world".into()))?;
        let world = World::parse(data).map_err(|e| SimError::Save(format!("game_world: {e:?}")))?;
        let tick = u32_at(world.header(), 0x18);
        let (month, doy) = world.clock();
        let saved_calendar = (world.header()[0], month, u16_at(world.header(), 2), doy);
        let town_hall_notices: [u8;84]=world.header()[0x72..0xc6].try_into().expect("cabecera validada");
        let custom_slots=save.block_named("name_bank").and_then(mission_text::custom_name_slots);
        let first_custom_names=custom_slots.as_ref().map(|s|[s[0].clone(),s[1].clone()]);
        let family_custom_names=custom_slots.as_ref().map(|s|s[2].clone());
        let candidate_custom_name_lengths=custom_slots.as_ref().map(|s|[s[0].iter().map(|n|n.len() as u8).collect::<Vec<_>>(),s[1].iter().map(|n|n.len() as u8).collect::<Vec<_>>()]);

        let mut founded_world=p3_save::model::WorldModel::parse(data).map_err(|e|SimError::Save(format!("modelo de mundo: {e:?}")))?;
        let siege_initial=founded_world.sieges.clone();
        let mut siege_retreat_flags=Vec::with_capacity(founded_world.sieges.len());
        for (city,record) in founded_world.sieges.iter_mut().enumerate() {
            let flags=p3_state::siege::scene_flags_after_load(record,city,tick).map_err(SimError::Save)?;
            // `0x627AD0` sobrescribe `+0x78` con la visibilidad calculada al cargar; el byte guardado no se conserva.
            if let Some((visible,_))=flags{p3_state::siege::set_loaded_scene_visible(record,city,visible).map_err(SimError::Save)?}
            siege_retreat_flags.push(flags.map(|(_,retreat)|retreat));
        }
        let parsed_rest=p3_save::world_rest::WorldRest::parse(&founded_world.rest).ok();
        let trade_offers=parsed_rest.as_ref().map(|rest|rest.offer_records());
        let trade_offer_raw_initial=parsed_rest.as_ref().map(|rest|founded_world.rest[rest.offers.clone()].to_vec());
        let weather_marker=parsed_rest.as_ref().map(|rest|rest.field_8f);
        let trade_centers=parsed_rest.as_ref().map(|rest| (0..rest.centers.len()).map(|i|rest.center(i).to_vec()).collect::<Vec<_>>());
        let town_history=parsed_rest.as_ref().and_then(|rest|rest.centers.first()).map(|range|founded_world.rest[..range.start].to_vec());
        let office_pool=p3_save::office_factory::pool_from_world(&founded_world).map_err(SimError::Save)?;
        let mut towns: Vec<Town> = (0..world.town_count()).map(|i| world.town(i)).collect();
        // El original recalcula precios al terminar el bloque económico, antes
        // de leer instalaciones y con el gestor de asedios aún sin reservar.
        let price_world=EconomyWorld{month:saved_calendar.1,..Default::default()};let mut price_host=NullHost::default();
        for town in &mut towns {let mut staged=town.clone();staged.0[0x840..0x990].fill(0);let active=staged.threshold(0,0)>1;p3_core::economy::market::update_town_price_thresholds(&price_world,&mut staged,active,&mut price_host);town.0[0x4f0..0x670].copy_from_slice(&staged.0[0x4f0..0x670]);}
        let mut sites=Vec::with_capacity(towns.len());
        for (i,town) in towns.iter_mut().enumerate(){
            let records=world.building_records(i);let count=records.len()/8;
            // El lector original recorre las cadenas libre y pública durante la
            // carga; detectar ciclos antes de ejecutar el puerto evita bucles.
            // La cabeza libre debe terminar exactamente en count: el asignador
            // usa ese valor para ampliar el almacén antes de acceder al registro.
            let mut free=usize::from(town.u16(0x760));let mut free_seen=vec![false;count];
            while free<count {if free_seen[free]{return Err(SimError::Save(format!("cadena de edificios cíclica (libre) en ciudad {i}")))}free_seen[free]=true;let p=8*free+4;free=usize::from(u16::from_le_bytes([records[p],records[p+1]]));}
            if free!=count{return Err(SimError::Save(format!("cadena libre de edificios fuera de rango en ciudad {i}")))}
            let mut at=usize::from(town.u16(0x764));let mut seen=vec![false;count];while at<count{if seen[at]{return Err(SimError::Save(format!("cadena de edificios cíclica en ciudad {i}")))}seen[at]=true;let p=8*at+4;at=usize::from(u16::from_le_bytes([records[p],records[p+1]]));}
            let mut site=Site{records:Vec::new(),cells:Vec::new()};let range=world.towns[i].buildings.clone();let input=&data[range];let consumed=site.deserialize_building_data(town,input,0x85);if consumed!=input.len(){return Err(SimError::Save(format!("datos de edificios incompatibles en ciudad {i}")))}sites.push(site);
        }
        // 0x0051B797..0x0051B7A5 inserta cada nodo leído en la cabeza.
        // S02 conserva el orden del archivo; la reconstrucción viva lo invierte.
        let tick_entries = (0..world.town_count()).map(|i| {let mut entries=world.tick_entries(i);entries.reverse();entries}).collect();

        let mut merchants = Vec::new();
        let mut rep = Vec::new();
        for k in 0..world.merchant_slots() {
            let img = world.merchant_image(k);
            merchants.push(Merchant { gold: u32_at(&img, 0) as i32, kind: u16_at(&img, 8), home_town: img[0x19], income: u32_at(&img, 0x4C8) as i32 });
            rep.push(RepMerchant { bytes: img });
        }

        let (mut offices, mut stocks, mut accounts) = (Vec::new(), Vec::new(), Vec::new());
        for k in 0..world.office_slots() {
            match world.office_image(k) {
                Some(img) => {
                    let saved: [u8; OFFICE_LEN] = world.office(k).try_into().map_err(|_| SimError::Save("oficina de tamaño inesperado".into()))?;
                    let persisted = p3_state::office::PersistedOffice::from_save_v85(&saved);
                    let mut weapons = [0i32; 4];
                    weapons.copy_from_slice(&persisted.stock.stock[20..24]);
                    offices.push(Office { weapons, next: u16_at(&img, 0x2CA), first_facility: u16_at(&img, 0x2CC) });
                    stocks.push(persisted.stock);
                    accounts.push(persisted.accounts);
                }
                None => {
                    offices.push(Office { weapons: [0; 4], next: 0xFFFF, first_facility: 0xFFFF });
                    stocks.push(OfficeStock { owner: 0xFFFF, town: 0xFF, ..Default::default() });
                    accounts.push(p3_core::economy::taxes::OfficeAccounts { owner: 0xFFFF, town: 0xFF, ..Default::default() });
                }
            }
        }

        let (mut housing, mut lists) = (Vec::new(), Vec::new());
        for k in 0..world.office_slots() {
            match world.office_image(k) {
                Some(img) => {
                    housing.push(OfficeHousing {
                        next: u16_at(&img, 0x2CA),
                        houses_head: u16_at(&img, 0x2D0),
                        house_count: u16_at(&img, 0x2D4),
                        capacity: [u16_at(&img, 0x2D8), u16_at(&img, 0x2DA), u16_at(&img, 0x2DC)],
                        assigned: [u16_at(&img, 0x2DE), u16_at(&img, 0x2E0), u16_at(&img, 0x2E2)],
                        level: [img[0x2E4], img[0x2E5], img[0x2E6]],
                    });
                    lists.push(OfficeLists { next: u16_at(&img, 0x2CA), building_head: u16_at(&img, 0x2CE) });
                }
                None => {
                    housing.push(OfficeHousing { next: 0xFFFF, houses_head: 0xFFFF, ..Default::default() });
                    lists.push(OfficeLists { next: 0xFFFF, building_head: 0xFFFF });
                }
            }
        }
        let mut persisted = Vec::new();
        let mut chains = Vec::new();
        for k in 0..world.office_slots() {
            match world.office_image(k) {
                Some(img) => {
                    let saved: [u8; OFFICE_LEN] = world.office(k).try_into().unwrap();
                    persisted.push(PersistedOffice::from_save_v85(&saved));
                    let (mut chain, mut cursor, mut seen) = (Vec::new(), u16_at(&img, 0x2CC) as usize, std::collections::BTreeSet::new());
                    while cursor < world.business_count() && seen.insert(cursor) {
                        let b = world.business(cursor);
                        chain.push(BusinessInput { kind: b[6], efficiency: u32_at(b, 0) as i32, employees: u16_at(b, 4), auxiliary: u16_at(b, 12), capacity: u16_at(b, 14) });
                        cursor = u16_at(b, 8) as usize;
                    }
                    chains.push(chain);
                }
                None => {
                    persisted.push(PersistedOffice::from_save_v85(&[0u8; OFFICE_LEN]));
                    chains.push(Vec::new());
                }
            }
        }
        let employment_businesses = (0..world.business_count()).map(|i| world.business(i).try_into().expect("registro de negocio de 20 bytes")).collect::<Vec<[u8;20]>>();
        let facilities = (0..world.business_count())
            .map(|k| {
                let b = world.business(k);
                MerchantFacility { employees: u16_at(b, 4), town: b[7], next: u16_at(b, 8) }
            })
            .collect::<Vec<_>>();

        let ships_block = save.block_named("ships").ok_or_else(|| SimError::Save("falta ships".into()))?;
        let ships_saved = Ships::parse(ships_block).map_err(|e| SimError::Save(format!("ships: {e:?}")))?;
        let pirate_band_images=p3_save::pirate_bands::read(&ships_block[ships_saved.tail.clone()]).ok();
        let debt_convoy_flags=(0..ships_saved.convoys.len()).map(|i|u32_at(&ships_saved.convoy_image(i),0x18)).collect();
        let expedition_convoy_images:Vec<Vec<u8>>=(0..ships_saved.convoys.len()).map(|i|ships_saved.convoy_image(i)).collect();
        let expedition_docked_head=u16_at(ships_block,8);
        let expedition_sea_head=u16_at(ships_block,10);
        let expedition_convoys_head=u16_at(ships_block,12);
        let (mut ships, mut cargo, mut debt_ship_metadata,mut ship_captains,mut expedition_ship_images,mut expedition_original_ship_images) = (Vec::new(), Vec::new(), Vec::new(),Vec::new(),Vec::new(),Vec::new());
        for k in 0..ships_saved.ship_slots() {
            match ships_saved.ship_image(k) {
                Some(mut img) => {
                    expedition_original_ship_images.push(Some(img.clone()));
                    p3_core::ships::cargo::recompute_load(&mut img);
                    expedition_ship_images.push(Some(img.clone()));
                    ships.push(Ship { owner: img[0], state: u16_at(&img, 0x134) });
                    debt_ship_metadata.push((u16_at(&img,4),u16_at(&img,0x134),u16_at(&img,0x15e),u16_at(&img,8)));
                    ship_captains.push(u16_at(&img,0x42));
                    let mut c = ShipCargo { owner: img[0], town: img[0x39], kind: img[0xE], crew: u16_at(&img, 0x40), marines: img[0x156], cutlasses: u16_at(&img, 0x154), capacity: u32_at(&img, 0x10) as i32, dirty:u16_at(&img,0x3c), ..Default::default() };
                    for w in 0..24 {
                        c.stock[w] = u32_at(&img, 0x54 + 4 * w) as i32;
                        c.average_price[w] = f32::from_bits(u32_at(&img, 0xB4 + 4 * w));
                        c.guns[w] = img[0x13C + w];
                    }
                    // Copia los derivados calculados como en el cargador original.
                    c.cargo_value=u32_at(&img,0x114) as i32;
                    c.cargo_weight=u32_at(&img,0x118) as i32;
                    c.equipment_weight=u32_at(&img,0x11c) as i32;
                    c.gun_power=u32_at(&img,0x120) as i32;
                    cargo.push(c);
                }
                None => {
                    expedition_ship_images.push(None);
                    expedition_original_ship_images.push(None);
                    ships.push(Ship { owner: 0xFF, state: 0xFF });
                    debt_ship_metadata.push((u16::MAX,0xffff,0,0xffff));
                    ship_captains.push(u16::MAX);
                    cargo.push(ShipCargo { owner: 0xFF, ..Default::default() });
                }
            }
        }
        let merchant_ship_next=debt_ship_metadata.iter().map(|entry|entry.0).collect::<Vec<_>>();
        merchant_ship_counts::rebuild(&mut rep,&merchant_ship_next).map_err(|e|SimError::Save(format!("flotas comerciales: {e}")))?;
        // `name_bank_load_from_buffer` (0x511E30) termina refrescando los punteros de nombre de cada comerciante
        // (0x51203A): normaliza `+0x1C` por sexo y cachea `+0xE8/+0xE4`, que no viajan en el guardado.
        if save.block_named("name_bank").is_some(){let count=u16::try_from(merchants.len()).map_err(|_|SimError::Save("demasiados comerciantes".into()))?;p3_core::mission_format::refresh_loaded_merchant_names(&mut rep,count).map_err(SimError::Save)?;}
        let factory_ships=(0..ships_saved.ship_slots()).map(|i|{if let Some(image)=&expedition_ship_images[i]{image.clone()}else{let record=ships_saved.ship(i);let mut image=vec![0u8;0x180];image[0x134..0x136].copy_from_slice(&record[..2]);image[6..8].copy_from_slice(&record[2..4]);image[0x137]=record[4];image}}).collect();
        let factory_captains=ships_block[ships_saved.captains.clone()].chunks_exact(16).map(|v|p3_save::captain::decode_v85(v.try_into().expect("capitán"))).collect();
        let expedition_factory=p3_core::ships::factory::HanseFactory{ships:factory_ships,captains:factory_captains,convoys:expedition_convoy_images.clone(),free_captain:u16_at(ships_block,2),free_ship:u16_at(ships_block,4),free_convoy:u16_at(ships_block,6),convoy_head:expedition_convoys_head,name_cursor:u16_at(ships_block,14),name_step:u16_at(ships_block,16)};
        let pirate_targets=ships_saved.pirate_band_targets().ok();let mission_pirates=ships_saved.pirate_band_missions().ok();
        let hanse_pirates=ships_saved.pirate_band_heads().ok().map(|heads|p3_core::economy::hanse_petition::PiratePortraits{
            heads,next:debt_ship_metadata.iter().map(|m|m.0).collect(),ship_captains,
            captains:expedition_factory.captains.clone(),default_portrait:None,
        });

        let calendar = Calendar::for_tick(tick);
        let economy_world = EconomyWorld {
            towns,
            tick,
            sieges: !founded_world.sieges.is_empty(),
            local_merchant: crate::load_start::local_player(&merchants),
            month: calendar.month,
            day_of_month: calendar.day,
            day_of_year: calendar.day_of_year,
            ship_count: ships_saved.ship_slots() as u16,
            ships,
            merchant_count: merchants.len() as u16,
            merchants,
            office_count: offices.len() as u16,
            offices,
            merchant_facility_count: facilities.len() as u16,
            merchant_facilities: facilities,
            difficulty: Some(save.settings()[0x17]),
            world_8a:u8::from(world.header()[0x12]==0x7f),
            alderman_merchant:world.header()[0x70],alderman_town:world.header()[0x71],
            tick_entries,
            ..Default::default()
        };
        let accounts = Accounts { offices: accounts, merchants: vec![MerchantAccounts::default(); rep.len()] };
        let mut economy = EconomySystem { world: economy_world, accounts, merchants: rep, offices: stocks, cargo_ships: cargo };
        let economy_office_count = economy.world.office_count;
        let mut idle = IdleSystems::default();
        let battle_block=save.block_named("sea_battles").ok_or_else(||SimError::Save("falta el gestor de batallas".into()))?;
        let mut saved_town_attack_pending=None;
        let battle_frame=match p3_save::tail::CurrentSeaBattles::parse(battle_block){
            Ok(frame)=>Some(frame),
            Err(p3_save::tail::TailError::Unsupported("ataque a ciudad o variante de combate"))=>{
                pirate_active_frame::parse_single_active_attack(battle_block,0x54000000,0)
                    .map_err(|e|SimError::Save(format!("la batalla de ataque guardada no está admitida: {e}")))?;
                saved_town_attack_pending=Some(battle_block.to_vec());None
            },
            Err(error)=>return Err(SimError::Save(format!("gestor de batallas inválido: {error:?}"))),
        };
        let saved_battle_pending=battle_frame.as_ref().is_some_and(|f|!f.objects.is_empty()).then(||battle_block.to_vec());
        if let Some(frame)=&battle_frame{if saved_battle_pending.is_none()&&!frame.pairs.is_empty(){idle.battle_manager=Some(battle_manager::Runtime::from_block(battle_block).map_err(SimError::Save)?)}}
        idle.clock = WorldClock {
            calendar,
            merchant_count: economy.world.merchant_count,
            town_count: economy.world.towns.len() as u32,
            tick,
            expedition_countdown: world.header()[0x13],
        };
        let rep_merchants = economy.merchants.clone();
        let mut discovery_map=vec![0u8;0x64];for merchant in &rep_merchants{p3_core::ai::misc::merge_map_objects(&mut discovery_map,&merchant.bytes[0x4d8..0x53c]);}
        let merchant_ship_groups=(0..world.merchant_slots()).map(|i|p3_save::world::merchant_ship_groups(world.merchant(i)).map_err(SimError::Save)).collect::<Result<Vec<_>,_>>()?;
        let mut wages = vec![None; persisted.len()];
        for i in 0..persisted.len(){if let Some(image)=world.office_image(i){let id=usize::from(u16_at(&image,0x2f2));if let Some(cap)=expedition_factory.captains.get(id){wages[i]=Some(u16_at(cap,0xc));persisted[i].stock.administrator_experience=Some(cap[0xa]);economy.offices[i].administrator_experience=Some(cap[0xa]);}}}
        let employment_counts=vec![None;persisted.len()];
        let business_free=u16_at(world.header(),0x1c);
        let tax_offices=economy.world.offices.iter().zip(persisted.iter()).map(|(o,p)|p3_core::economy::extra_tax::TaxOffice{owner:p.accounts.owner,next:o.next,businesses:p.accounts.business_buildings,houses:p.accounts.houses}).collect();
        let tasks = save.block_named("scheduled_tasks").ok_or_else(||SimError::Save("faltan tareas programadas".into()))?;
        if tasks.len()<6 {return Err(SimError::Save("cabecera de tareas truncada".into()))}
        let count=usize::from(u16_at(tasks,4));
        if tasks.len()<6+24*count {return Err(SimError::Save("tabla de tareas truncada".into()))}
        let mut scheduler=p3_core::scheduler::Scheduler::new();scheduler.head=u16_at(tasks,0);scheduler.free_head=u16_at(tasks,2);scheduler.capacity=count as u16;
        for bytes in tasks[6..6+24*count].chunks_exact(24) {
            let mut payload=[0;16];payload.copy_from_slice(&bytes[8..24]);
            scheduler.records.push(p3_core::scheduler::TaskRecord{due:u32_at(bytes,0),next:u16_at(bytes,4),kind:u16_at(bytes,6),payload});
        }
        let mission_initial=p3_save::tail::script_snapshots(tasks).map_err(SimError::Save)?;
        let mission_variables=mission_initial.iter().map(|s|s.as_ref().map(|v|v.variables.clone())).collect();let hunt_losses=p3_save::tail::hunt_loss_lists(tasks).map_err(SimError::Save)?.into_iter().enumerate().filter_map(|(i,l)|l.map(|l|(i,l))).collect();
        let route_stop_runtime=Some(route_stop_runtime::RouteStopRuntime::from_block(save.block_named("route_stop_pool").ok_or_else(||SimError::Save("falta depósito de paradas".into()))?).map_err(SimError::Save)?);
        let message_pool=save.block_named("message_pool").ok_or_else(||SimError::Save("faltan mensajes".into()))?.to_vec();
        let council_message_enabled=vec![true;rep_merchants.len()];
        let building_message_enabled=vec![true;rep_merchants.len()];
        Ok(Sim { saved_battle_pending, saved_town_attack_pending, merchant_conflicts: 0, merchant_double_changes: 0, economy, idle, persisted, rep_merchants, businesses: chains, wages, commercial_sell_factor:match save.settings()[0x10]{0=>f32::from_bits(0x400ccccd),1=>2.0,_=>f32::from_bits(0x3fe66666)},host: NullHost { pirate_group_thresholds:Some(pirate_group_runtime::LEVEL_THRESHOLDS),town_attack_origin:battle_block.to_vec(),game_end_start:u32_at(world.header(),0x14),game_end_mode:save.settings()[0xd],mission_settings:Some(c10_script_flat::loaded_settings(save.settings())),first_discovery_initial:world.header()[0x10],first_discovery_flag:u8::from(world.header()[0x10]!=0),route_stop_runtime,discovery_map,candidate_custom_name_lengths,family_custom_names,first_custom_names,siege_initial,siege_retreat_flags,siege_difficulty:save.settings()[0x16],town_history_initial:town_history.clone(),town_history,trade_offer_initial:trade_offers.clone(),trade_offers,trade_offer_raw_initial,trade_centers_initial:trade_centers.clone(),trade_centers,merchant_ship_groups_initial:merchant_ship_groups.clone(),merchant_ship_groups,scheduled_office_images:Some(office_pool.records.clone()),office_pool_initial:Some(office_pool.clone()),office_pool:Some(office_pool),founded_world:Some(founded_world),sites, housing, lists, office_count: economy_office_count, church_cost_factor: world.header()[0x11],weather_speed:u32_at(save.settings(),4),weather_marker,weather_marker_initial:weather_marker,town_hall_notices:Some(town_hall_notices),town_hall_notices_initial:Some(town_hall_notices), council_scheduler:scheduler, mission_initial, mission_variables, hunt_losses, message_pool,council_message_enabled,building_message_enabled,tax_offices,employment_businesses,employment_counts,business_free,debt_ship_metadata,debt_convoy_flags,hanse_pirates,pirate_targets,mission_pirates,mission_pirates_initial:mission_pirates,pirate_band_images_initial:pirate_band_images.clone(),pirate_band_images,pirate_difficulty:save.settings()[0x13],battle_difficulty:save.settings()[0x12],expedition_town_ids:world.header()[0x20..0x20+world.town_count()].to_vec(),expedition_factory_initial:Some(expedition_factory.clone()),expedition_factory:Some(expedition_factory),expedition_original_ship_images,expedition_ship_images,expedition_convoy_images,expedition_docked_head,expedition_sea_head,expedition_convoys_head,expedition_initial_docked_head:expedition_docked_head,time_mode:{let mut t=p3_core::pump::TimeMode::default();t.load(u32::from_le_bytes(save.header()[..4].try_into().expect("cabecera"))>>16,save.speed());t},..Default::default() }, saved_calendar })
    }

    /// El formato de asalto activo no puede publicarse en S05 hasta que
    /// su destructor y la recarga estén conectados.
    pub fn require_no_active_town_attack_save(&self)->Result<(),String>{
        if self.saved_town_attack_pending.is_some(){return Err("asalto urbano cargado pendiente de reconstrucción".into())}
        if let Some(error)=&self.host.clock_partial_error{return Err(format!("tick incompleto: {error}; hay que recargar"))}
        if self.host.town_attack_tick_pending{return Err("tick de asalto pendiente de completar".into())}
        if self.host.pending_pirate_attack.is_some(){return Err("ataque pirata pendiente de completar".into())}
        if !self.host.town_attack_effects.is_empty()||!self.host.town_attack_frame_effects.is_empty()||!self.host.town_attack_manager_effects.is_empty(){return Err("efectos de asalto pendientes de entregar".into())}
        if self.host.town_attack_save_unreconciled&&self.host.town_attack_active.is_none(){return Err("asalto terminado pendiente de conciliar con S05".into())}
        if self.host.town_attack_active.is_some(){Err("guardado de asalto urbano activo no recargable".into())}else{Ok(())}
    }

    /// Una batalla cargada debe resolverse antes de mover el reloj del mundo.
    pub fn saved_battle_pending(&self)->bool{self.saved_battle_pending.is_some()||self.saved_town_attack_pending.is_some()}
    /// Devuelve los campos persistidos, aún sin instalar recursos ni habilitar
    /// el reloj. La dirección es una identidad del gestor del anfitrión.
    pub fn pending_town_attack_state(&self,address:u32,slot:u8)->Result<p3_save::tail::SavedInitialCommercialBattle,String>{
        let block=self.saved_town_attack_pending.as_deref().ok_or("asalto urbano guardado ausente")?;
        pirate_active_frame::parse_single_active_attack(block,address,slot)
    }
    /// Instala el estado cargado y sus índices navales; deja la máscara y el
    /// reloj protegidos hasta reconstruir los recursos del asalto.
    pub fn install_pending_town_attack(&mut self,address:u32)->Result<(),String>{
        self.install_pending_town_attack_inner(address,false)
    }
    /// Reconstruye solo para diagnóstico una escena cargada con torres; el
    /// frame permanece protegido hasta integrar su comportamiento completo.
    pub fn install_pending_town_attack_diagnostic(&mut self,address:u32)->Result<(),String>{
        self.install_pending_town_attack_inner(address,true)
    }
    fn install_pending_town_attack_inner(&mut self,address:u32,allow_towers:bool)->Result<(),String>{
        if address==0 || address.checked_add(0x2f24).is_none()
            || self.host.town_attack_active.is_some() || self.host.town_attack_collision.is_some()
            || !self.host.council_operations.is_empty() {
            return Err("gestor o dirección de asalto incompatibles".into());
        }
        let block=self.saved_town_attack_pending.as_ref().ok_or("asalto urbano guardado ausente")?.clone();
        let state=pirate_active_frame::parse_single_active_attack(&block,address,0)?;
        let target=state.object[0x1e98];
        if !(250..=254).contains(&target) {return Err("destino de asalto cargado no admitido".into())}
        let band_index=usize::from(254-target);
        let band=self.host.pirate_band_images.as_ref().and_then(|v|v.get(band_index)).and_then(Option::as_ref)
            .ok_or("banda de asalto cargado ausente")?;
        band.validate().map_err(str::to_owned)?;
        let operational=pirate_empty_hideout_scene::has_operational_towers(band)?;
        if band.plan.is_none() || (!allow_towers && operational) {
            return Err("plano o torres de asalto cargado no admitidos".into());
        }
        if state.object[0x660..0x662]!=band.header[8..10] {return Err("convoy defensor cargado desincronizado".into())}
        let nav=self.host.navigation.as_ref().ok_or("flota cargada aún no configurada")?;
        let mut rng=self.host.investigation_rng;
        let battle=if state.bitmap.is_empty(){pirate_active_frame::rebuild_loaded_active_attack(state,&nav.snapshot.ships,address)?}else{
            let trial=rng.as_mut().ok_or("RNG CRT necesario para proyectiles cargados")?;
            pirate_active_frame::rebuild_loaded_active_attack_with_projectiles(state,&nav.snapshot.ships,address,trial)?
        };
        let collision=battle_spatial::into_live(&battle.collisions).map_err(str::to_owned)?;
        let navigation=pirate_active_frame::TownNavigationCaches::newly_loaded(battle.state.local_ships.len()).map_err(str::to_owned)?;
        let mut manager=vec![0u8;0xc40];manager[..4].copy_from_slice(&address.to_le_bytes());manager[0x404]=1;
        let runtime=town_attack_manager::Runtime {manager,object:battle.state.object.clone(),groups:vec![],
            slot:0,address,original_block:block,populated:Some(battle),populated_ticks:2,town_navigation:Some(navigation)};
        self.host.town_attack_active=Some(runtime);self.host.town_attack_collision=Some(collision);self.host.investigation_rng=rng;
        self.host.town_attack_band=Some(band_index as u8);self.host.town_attack_save_unreconciled=true;
        self.saved_town_attack_pending=None;Ok(())
    }
    /// Reconstruye máscara y colisiones de la escena cargada en una copia.
    /// El frame posterior y el reloj siguen protegidos hasta su contraste.
    pub fn rebuild_pending_town_attack_geometry(&mut self,projection:p3_core::ships::town_attack_mask::TownProjection)->Result<(),String>{
        let band=usize::from(self.host.town_attack_band.ok_or("banda de asalto ausente")?);
        let plan=self.host.pirate_band_images.as_ref().and_then(|v|v.get(band)).and_then(Option::as_ref)
            .and_then(|v|v.plan.as_deref()).ok_or("plano de guarida ausente")?;
        let manager=self.host.town_attack_active.as_ref().ok_or("asalto cargado ausente")?;
        let mut battle=manager.populated.clone().ok_or("escena poblada ausente")?;
        pirate_active_frame::rebuild_loaded_hideout_geometry(&mut battle,plan,projection)?;
        let collision=battle_spatial::into_live(&battle.collisions).map_err(str::to_owned)?;
        let manager=self.host.town_attack_active.as_mut().unwrap();
        manager.object=battle.state.object.clone();manager.populated=Some(battle);
        self.host.town_attack_collision=Some(collision);Ok(())
    }
    pub(crate) fn require_resolved_saved_battle(&self)->Result<(),String>{
        if self.saved_town_attack_pending.is_some(){Err("asalto urbano cargado pendiente de reconstrucción".into())}
        else if self.saved_battle_pending.is_some(){Err("batalla naval cargada pendiente de resolución".into())}else{Ok(())}
    }
    /// Comprueba la publicación preparada por el anfitrión antes de mutar la sesión.
    pub fn validate_resolved_battle_manager(&self,manager:&battle_manager::Runtime)->Result<(),String>{
        if self.saved_town_attack_pending.is_some(){return Err("el asalto urbano no admite este cierre naval".into())}
        if let Some(original)=self.saved_battle_pending.as_ref(){
            if manager.original_block.as_slice()!=original.as_slice()||!manager.rewrite||manager.manager.len()<0x405||manager.manager[..0x3fc].iter().any(|&v|v!=0)||manager.manager[0x404]!=0||manager.error.is_some(){return Err("gestor de batalla cargada aún activo, inválido o de otro guardado".into())}
        }
        Ok(())
    }
    /// Instala el gestor ya validado y confirma el final de la batalla cargada.
    pub fn install_resolved_battle_manager(&mut self,manager:battle_manager::Runtime)->Result<(),String>{
        self.validate_resolved_battle_manager(&manager)?;self.idle.battle_manager=Some(manager);self.saved_battle_pending=None;Ok(())
    }

    pub fn summary(&self) -> Summary {
        let w = &self.economy.world;
        let mut population = [0i64; 4];
        let mut stock = [0i64; 24];
        let mut town_money = 0i64;
        let mut hasher = Sha256::new();
        for t in &w.towns {
            for (g, p) in population.iter_mut().enumerate() {
                *p += t.pop(g) as i64;
            }
            for (i, s) in stock.iter_mut().enumerate() {
                *s += t.stock(i) as i64;
            }
            town_money += t.i32(off::MONEY) as i64;
            // TownMap+0 es una marca de dibujo temporal: el serializador
            // original no la guarda y se reconstruye a cero al cargar.
            hasher.update(&t.0[..0x7a4]);
            hasher.update([0u8; 4]);
            hasher.update(&t.0[0x7a8..]);
        }
        for m in &self.economy.merchants {
            // +0x47c es un contador recalculado, no forma parte del guardado. +0xE4..+0xEB son los punteros de nombre
            // cacheados (`mission_format::name_ptr`): tampoco se guardan y la carga los rehace.
            hasher.update(&m.bytes[..0xe4]);
            hasher.update([0u8; 8]);
            hasher.update(&m.bytes[0xec..0x47c]);
            hasher.update([0u8; 4]);
            hasher.update(&m.bytes[0x480..]);
        }
        let c = self.idle.clock.calendar;
        Summary {
            tick: self.idle.clock.tick,
            day: c.day,
            month: c.month,
            year: c.year,
            merchants_gold: self.economy.merchants.iter().map(|m| m.i32(0) as i64).sum(),
            population,
            town_money,
            stock,
            hash: hasher.finalize().iter().map(|b| format!("{b:02x}")).collect(),
        }
    }

    /// Avanza hasta un encuentro admitido, deteniéndose tras el puerto y
    /// antes del tick de batalla. No ejecuta ni resuelve el combate pendiente.
    /// Las notificaciones y el diálogo quedan en `host.encounter_battle`.
    pub fn advance_until_encounter(&mut self,target:u32,config:EncounterConfig)->Result<tick::AdvanceEnd,String>{
        if self.host.clock_before_battle || self.host.encounter_battle.is_some() || self.host.navigation_error.is_some() {
            return Err("hay un tick naval interrumpido".into());
        }
        if self.host.navigation.is_none() || target<=self.idle.clock.tick {return Err("reloj o navegación no disponibles".into())}
        self.host.encounter_config=Some(config);
        let result=self.advance_pumped_until_inner(target,8,false,true);
        self.host.encounter_config=None;
        let end=result?;
        if end==tick::AdvanceEnd::InterruptedBeforeBattle {self.host.clock_before_battle=true}
        if let Some(error)=&self.host.navigation_error {return Err(format!("navegación: {error}"))}
        if end==tick::AdvanceEnd::Reached {let errors=self.dispatch_pending_council_operations();self.host.council_errors.extend(errors);}
        Ok(end)
    }

    /// Completa exclusivamente el tick interrumpido tras crear el encuentro.
    /// El anfitrión debe aportar la imagen real del gestor de batallas; las
    /// listas auxiliares activas se rechazan. La batalla queda pendiente de
    /// simulación y ningún tick posterior se ejecuta por este método.
    pub fn complete_encounter_tick(&mut self,battle_manager:&mut [u8])->Result<(),String>{
        self.require_resolved_saved_battle()?;
        if !self.host.clock_before_battle || self.host.encounter_battle.is_none() || self.host.navigation_error.is_some(){return Err("no hay encuentro listo para completar el tick".into())}
        p3_core::ships::battle::tick_idle_battle_manager(battle_manager).map_err(str::to_string)?;
        self.resume_encounter_after_manager();Ok(())
    }

    /// Completa el mismo tick con las listas temporales del gestor y devuelve
    /// las operaciones de interfaz/liberación que debe atender el anfitrión.
    pub fn complete_encounter_tick_with_manager(&mut self,manager:&mut [u8],groups:&mut Vec<Vec<[u8;8]>>,view:battle_manager::ViewState)->Result<Vec<battle_manager::ManagerEffect>,String>{
        self.require_resolved_saved_battle()?;
        if !self.host.clock_before_battle || self.host.encounter_battle.is_none() || self.host.navigation_error.is_some(){return Err("no hay encuentro listo para completar el tick".into())}
        if self.idle.battle_manager.is_some(){return Err("el reloj ya tiene un gestor de batallas configurado".into())}
        let effects=battle_manager::tick(manager,groups,true,view).map_err(str::to_string)?;
        self.resume_encounter_after_manager();Ok(effects)
    }
    fn resume_encounter_after_manager(&mut self){
        let target=self.idle.clock.tick;
        let (end,errors)={
            let mut offices=OfficeTownHost{offices:&mut self.persisted,merchants:&mut self.rep_merchants,businesses:&mut self.businesses,captain_wages:&mut self.wages,difficulty:self.commercial_sell_factor,other:&mut self.host,completed:Vec::new(),errors:Vec::new()};
            let mut bridge=EconomyTickBridge{economy:&mut self.economy,town_host:&mut offices,other:&mut self.idle};
            let end=tick::resume_before_battle(&mut bridge,target,0,|_|false);
            (end,std::mem::take(&mut offices.errors))
        };
        debug_assert_eq!(end,tick::AdvanceEnd::Reached);
        {
            // El tick del encuentro también ejecuta las escenas de asedio (0x6120xx) del original.
            let mut daily=merchant_daily::DailyClock{sim:self,world_ready:true,error:None};
            let mut town=0;
            while town<tick::TickSystems::clock(&mut daily).town_count{tick::TickSystems::siege_scene(&mut daily,town);town+=1}
            if let Some(error)=daily.error{daily.sim.host.council_errors.push(error)}
        }
        self.host.clock_before_battle=false;
        self.host.council_errors.extend(errors);
    }

    /// Despacha la primera operación de política 0x96 del encuentro. Los
    /// efectos de reloj y apertura de interfaz se devuelven al anfitrión.
    pub fn dispatch_encounter_policy(&mut self,selected:&mut u8,player:u32)->Result<Vec<p3_core::ships::battle::BattlePolicyEffect>,String>{
        if self.host.clock_before_battle{return Err("tick de encuentro incompleto".into())}
        let battle=self.host.encounter_battle.as_mut().ok_or("no hay encuentro")?;
        let op=*battle.operations.first().ok_or("no hay operación pendiente")?;
        if battle.object.len()<0x664{return Err("batalla truncada".into())}
        if op[0]!=0x96||op[1]!=u32::from(u16::from_le_bytes([battle.object[0x662],battle.object[0x663]])){return Err("operación o ranura de batalla no admitida".into())}
        let merchant=u16::try_from(op[2]).map_err(|_|"comerciante inválido")?;let policy=u8::try_from(op[3]).map_err(|_|"política inválida")?;
        let effects=p3_core::ships::battle::set_battle_participant_policy(&mut battle.object,&mut battle.local_ships,&battle.next,selected,player,merchant,policy).map_err(str::to_string)?;
        battle.operations.remove(0);Ok(effects)
    }

    /// Avanza `ticks` ticks del reloj (256 por día) como [`Self::advance_days`]; lo usa la carta náutica del cliente para mover
    /// los barcos entre fotogramas. `days` del informe cuenta los días completos que cruza.
    pub fn advance_ticks(&mut self, ticks: u32) -> Report {
        let start=self.summary();
        let mut errors=Vec::new();
        match self.idle.clock.tick.checked_add(ticks) {
            None=>errors.push("objetivo de reloj fuera de rango".into()),
            Some(target)=>match self.advance_pumped_until(target,8,false){
                Ok(tick::AdvanceEnd::Reached)=>{
                    let mut passes=0usize;
                    while !self.host.council_operations.is_empty(){
                        if passes>=65536{errors.push("la cola de operaciones no terminó".into());break}
                        passes+=1;let pass=self.dispatch_pending_council_operations();if !pass.is_empty(){errors.extend(pass);break}
                    }
                },
                Ok(other)=>errors.push(format!("reloj interrumpido: {other:?}")),
                Err(error)=>errors.push(error),
            }
        }
        errors.extend(std::mem::take(&mut self.host.council_errors));
        let mut inactive:BTreeMap<&'static str,u64>=BTreeMap::new();
        for (k,v) in self.host.calls.iter().chain(self.idle.calls.iter()){*inactive.entry(k).or_default()+=v;}
        let elapsed=self.idle.clock.tick/256-start.tick/256;
        Report{start,end:self.summary(),days:elapsed,inactive,errors}
    }

    /// Avanza `days` días de modo local, con la fase diaria de comerciantes.
    /// Se detiene y comunica cualquier dependencia sin implementar; `days`
    /// en el informe cuenta los días completos realmente avanzados.
    pub fn advance_days(&mut self, days: u32) -> Report {
        let start=self.summary();
        let mut errors=Vec::new();
        let target=days.checked_mul(256).and_then(|ticks|self.idle.clock.tick.checked_add(ticks));
        match target {
            None=>errors.push("objetivo de reloj fuera de rango".into()),
            Some(target)=>match self.advance_pumped_until(target,8,false){
                Ok(tick::AdvanceEnd::Reached)=>{
                    // Se detiene el reloj y se vacían las pasadas pendientes;
                    // S05 no serializa los paquetes de operaciones.
                    let mut passes=0usize;
                    while !self.host.council_operations.is_empty(){
                        if passes>=65536{errors.push("la cola de operaciones no terminó".into());break}
                        passes+=1;let pass=self.dispatch_pending_council_operations();if !pass.is_empty(){errors.extend(pass);break}
                    }
                },
                Ok(other)=>errors.push(format!("reloj interrumpido: {other:?}")),
                Err(error)=>errors.push(error),
            }
        }
        errors.extend(std::mem::take(&mut self.host.council_errors));
        let mut inactive:BTreeMap<&'static str,u64>=BTreeMap::new();
        for (k,v) in self.host.calls.iter().chain(self.idle.calls.iter()){*inactive.entry(k).or_default()+=v;}
        let elapsed=self.idle.clock.tick.saturating_sub(start.tick)/256;
        Report{start,end:self.summary(),days:elapsed,inactive,errors}
    }
}
pub mod expedition_finish_stage;

pub mod expedition_return_stage;

pub mod expedition_return_resources;
pub mod land_transport_stage;
mod land_transport_start_live;
pub mod profit_share_stage;

pub mod trade_offer_watch_stage;
pub mod pirate_mission_approach_stage;
pub mod pirate_mission_attack_stage;
mod pirate_mission_attack_live;
mod pirate_mission_approach_live;

pub mod pirate_empty_hideout_scene;
pub mod town_attack_manager;
pub mod pirate_mission_transaction;
pub mod pirate_empty_band_end;
pub mod pirate_empty_scene_destroy;
pub mod pirate_mission_publish;
mod pirate_mission_frame_live;
mod pirate_scene_snapshot;
pub mod pirate_empty_hideout_messages;
pub mod pirate_empty_scene_publish;
pub mod pirate_empty_scene_save;
pub mod pirate_b4_stage;
pub mod pirate_b4_live;
pub mod pirate_hideout_resources;
