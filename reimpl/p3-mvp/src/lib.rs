//! Sesión visual experimental: conecta recursos locales y el simulador existente.
//! No acredita equivalencia con el juego completo.
use p3_assets::{cpr::Archive, town::TownMap};
use p3_core::economy::trade::{office_trade_with_reputation, TradeSide};
use p3_save::model::SaveModel;
use p3_sim::Sim;
use serde_json::{json, Value};
use std::path::Path;
pub mod audio;
pub mod harbor;
pub mod harbor_fx;
pub mod hostfs;
pub mod lang_cpr;
pub mod navmap;
pub mod headless;
pub mod markup;
pub mod ui_rt;
pub mod net;
pub mod scene;
mod screens;
mod screens_trade;
mod screens_baths;
mod screens_weapons;
mod screens_guild;
mod screens_found;
mod screens_statistics;
pub mod siege_view;
pub mod town_fx;
pub mod world;

/// Tabla de tipo de edificio a clase gráfica del ejecutable inglés, 0x00672FC0.
/// `0x00672D28`: estatua de cada ciudad (matriz[1] de las hojas `statue_*.let`), indexada por el id de ciudad de StadtDaten.ini.
const STATUE_KEY: [u8; 40] = [
    19, 19, 8, 15, 15, 14, 14, 16, 7, 16, 7, 3, 3, 10, 0, 6, 12, 12, 13, 13, 10, 11, 4, 11, 6, 8, 2, 5, 1, 0, 5, 17,
    17, 1, 2, 4, 9, 18, 18, 9,
];
pub const BUILDING_GRAPHIC: [u8; 58] = [
    0x2c, 0x1b, 0x0b, 0x08, 0x5d, 0x57, 0x58, 0x5b, 0x59, 0x53, 0x56, 0x64, 0x5a, 0x5e, 0x60, 0x55, 0x5c, 0x63, 0x62,
    0x61, 0x54, 0x51, 0x51, 0x51, 0x50, 0x50, 0x50, 0x2a, 0x2a, 0x2a, 0x66, 0x04, 0x03, 0x0b, 0x07, 0x00, 0x02, 0x05,
    0x09, 0x33, 0x52, 0x65, 0x67, 0x68, 0x69, 0x37, 0x01, 0x5f, 0x29, 0x29, 0x29, 0x29, 0x2f, 0x2f, 0x2e, 0x2f, 0x2f,
    0x00,
];

#[derive(Clone, Copy)]
struct BattleClock {
    pump: p3_core::pump::BattleTimePump,
    pending: Option<u16>,
    mode: u32,
}
/// Efectos opcionales de `render_fx`: precipitación `(nieve, cuenta)` y grietas `(x, y, intensidad)` en casillas con edificio.
#[derive(Default, Clone)]
pub struct Effects {
    pub weather: Option<(bool, i32)>,
    pub cracks: Vec<(u32, u32, u32)>,
    pub seed: u32,
    /// Pide al conductor del original (`0x59BAF0`) las grietas de los negocios del jugador en la ciudad actual y las suma a `cracks`.
    pub conductor: bool,
}

/// Memoria plana por segmentos para ejecutar funciones del original sobre el estado de la partida (lo no escrito vale 0).
struct Image(Vec<(u32, Vec<u8>)>);
impl p3_render::scene_math::Mem for Image {
    fn r8(&mut self, a: u32) -> u8 {
        self.0
            .iter()
            .find(|(b, d)| a >= *b && ((a - *b) as usize) < d.len())
            .map_or(0, |(b, d)| d[(a - *b) as usize])
    }
    fn w8(&mut self, a: u32, v: u8) {
        if let Some((b, d)) = self
            .0
            .iter_mut()
            .find(|(b, d)| a >= *b && ((a - *b) as usize) < d.len())
        {
            d[(a - *b) as usize] = v
        }
    }
}
struct CrackLog(Vec<(u32, u32, u32)>);
impl p3_render::crack_conductor::ConductorEnv for CrackLog {
    fn crack(&mut self, x: u32, y: u32, p: u32) {
        self.0.push((x, y, p))
    }
    fn sweep(&mut self) {}
}

/// Disparadores de sonido que aporta el cliente en cada fotograma (además de los del puerto, que calcula el motor).
#[derive(Default, Clone, Debug)]
pub struct SoundRequest {
    /// `(x, y, clase de ficha)` de los animadores visibles (`render` → `sound_sites`): 0x594C00.
    pub sites: Vec<(i32, i32, u16)>,
    /// Edificios derrumbados este fotograma, en pantalla: 0x599040.
    pub collapses: Vec<(i32, i32)>,
    /// Sonidos del mapa a lanzar (1..=5) sin datos propios: `0x459790(clase, 0, 0, 0, 0)`.
    pub map_kinds: Vec<u32>,
    /// Sonidos del mapa con sus argumentos `(clase, id, x, y, extra)`: 1 = barco elegido (`0x45101B`, id del barco),
    /// 4 = ruta marcada (`0x451DD4`, id y posición del barco), 3 = tiempo (`0x459527`), 2 = avistamiento (`0x50D5F9`).
    pub map_events: Vec<(u32, u32, i32, i32, u32)>,
    /// Vista del mapa de navegación `(x, y, ancho, alto)` en coordenadas del mapa; con ella suenan también las entidades de tiempo
    /// visibles (`0x459380`) y se reajustan las colas 3/4 (`0x45A470`).
    pub map_view: Option<[i32; 4]>,
}

pub struct Session {
    pub sim: Sim,
    /// Tablero de sonido de la vista de ciudad (se crea al primer `town_sounds`).
    sound_board: Option<audio::SoundBoard>,
    /// Humo, hundimiento y sonido de los barcos del puerto (`0x62018B`), que avanzan con el reloj de la vista.
    harbor_fx: std::cell::RefCell<harbor_fx::HarborFx>,
    effects_volume: u8,
    /// Derrumbes en pantalla pendientes de sonar en el siguiente `town_sounds` (`queue_collapse_sound`).
    pending_collapses: Vec<(i32, i32)>,
    /// Baños (`BathHouseBribeDialog`, singleton del original): marca de la última visita y escaños donde ya falló un soborno hoy.
    pub(crate) bath_visit: (u32, [bool; 4]),
    battle_selected: u8,
    paused_battle: Option<p3_save::tail::RebuiltInitialCommercialBattle>,
    battle_clock: Option<BattleClock>,
    /// Inicio de la batalla automática que avanza a la vez que el reloj
    /// (`advance_tick_beside_auto_battle`). Mientras existe, la simulación no
    /// tiene encuentro pendiente y la batalla vive solo en `paused_battle`.
    concurrent_start: Option<p3_core::ships::battle::CommercialBattleStart>,
    battle_texts: Option<p3_sim::battle_text::BattleTextBank>,
    letter_texts: Option<p3_sim::letter_text::LetterTextBank>,
    /// Imagen del EXE original para el enlazador de interfaz ([`ui_rt`]); `None` sin `Patrician3.exe`.
    original_exe: Option<Vec<u8>>,
    model: SaveModel,
    archive: p3_assets::cpr::Bytes,
    changed_ships: Vec<usize>,
    planner: Option<p3_core::route::finder::RoutePlanner>,
    pub city: usize,
    pub office: usize,
    pub owner: usize,
    /// Entradas explícitas de la tabla `0x6DD73C` para los carros del cochero (tipo 31). Vacío = los transportes terrestres de la partida
    /// (`land_transport_carts`, tareas `0x0C` del planificador guardado).
    pub cart_entries: Vec<p3_render::cart_field::CartEntry>,
    /// Grado gráfico de la vista de ciudad (`escena+0x504`) para las huellas; `None` = `Town+0x9BC` de la ciudad mostrada.
    pub town_grade: Option<u32>,
    /// Carta náutica desplazable (`navmap_frame`): texturas e imagen naval en caché.
    navmap: Option<navmap::NavState>,
    /// Carta náutica con el dibujo completo de referencia (sin texturas convertidas, copias rápidas ni capa guardada):
    /// sirve para comprobar que el dibujo normal da la misma imagen (`tests/navmap_cache.rs`).
    pub navmap_reference: bool,
}
impl Session {
    /// Imagen de diagnóstico de solo lectura para contrastes de C10; no avanza la batalla.
    #[doc(hidden)]
    pub fn battle_diagnostic_snapshot(&self) -> Option<&p3_save::tail::RebuiltInitialCommercialBattle> {
        self.paused_battle.as_ref()
    }

    // Estos datos pertenecen al anfitrión, no al archivo S05. Solo se mueven
    // una vez que la reconstrucción y la selección de oficina han tenido éxito.
    fn transfer_host_context(from: &mut Sim, to: &mut Sim) {
        // Estos adaptadores reconstruyen las vistas de una sesión viva: no son
        // una nueva carga del jugador, por lo que no invierten otra vez la lista.
        to.economy.world.tick_entries = std::mem::take(&mut from.economy.world.tick_entries);
        let (a, b) = (&mut from.host, &mut to.host);
        b.first_discovery_flag = a.first_discovery_flag;
        b.investigation_rng = a.investigation_rng;
        if a.route_stop_runtime.is_some() {
            b.route_stop_runtime = a.route_stop_runtime.take();
        }
        b.mission_texts = a.mission_texts.clone();
        b.pirate_scene_texts = a.pirate_scene_texts.take();
        b.discovery_locations = a.discovery_locations.clone();
        b.mission_flat_rdata = a.mission_flat_rdata.clone();
        b.pirate_group_thresholds = a.pirate_group_thresholds;
        b.town_pirate_piece_bytes = a.town_pirate_piece_bytes;
        b.celebration_state = a.celebration_state;
        b.investigation_local_mode = a.investigation_local_mode;
        b.ai_work_residue = a.ai_work_residue;
        b.ai_work_residue_by_tick = a.ai_work_residue_by_tick.clone();
        b.headless_human_battles = a.headless_human_battles;
        b.siege_stack_tail = a.siege_stack_tail;
        b.siege_stack_tails = a.siege_stack_tails.clone();
        b.siege_start_clocks = a.siege_start_clocks.clone();
        b.reminder_hotseat_gate = a.reminder_hotseat_gate;
        b.operations_mode = a.operations_mode;
        b.siege_headless_march = a.siege_headless_march;
        b.weather_speed = a.weather_speed;
        b.statistics = a.statistics.take();
        b.election_texts = a.election_texts.take();
        b.hunt_texts = a.hunt_texts.take();
        b.expedition_return_resources = a.expedition_return_resources.take();
        b.expedition_return_map_dirty = a.expedition_return_map_dirty;
        // La matriz de la IA y el planificador no forman parte de S05.
        if b.navigation_ai.is_none() {
            b.navigation_ai = a.navigation_ai.take()
        }
        if b.navigation_planner.is_none() {
            b.navigation_planner = a.navigation_planner.take()
        }
        macro_rules! move_pending {($($field:ident),* $(,)?)=>{$(b.$field=std::mem::take(&mut a.$field);)*}}
        b.battle_news_stamp = a.battle_news_stamp;
        move_pending!(
            battle_pirate_news,
            wedding_notifications,
            wedding_windows,
            famine_notifications,
            weather_ice_notifications,
            council_notifications,
            mission_completion_notifications,
            mission_script_notifications,
            mission_completion_ui,
            mission_failure_ui,
            reminder_notices,
            auction_notices,
            auction_finished,
            auction_results,
            celebration_notifications,
            celebration_windows,
            plague_notifications,
            plague_windows,
            fire_notifications,
            fire_windows,
            investigation_notifications,
            hanse_notifications,
            debt_notifications,
            debt_seizure_notifications,
            hanse_vote_notifications,
            hanse_vote_windows,
            land_transport_notifications,
            expedition_notifications,
            expedition_windows,
            expedition_return_notices,
            expedition_return_map_notices,
            expedition_return_map_windows,
            mayor_statue_visuals,
            shipyard_presentation,
            election_notifications,
            election_announcements
        );
    }
    /// Suministra los bytes observados del sufijo dinámico de las aldeas.
    /// Es estado del proceso, no del S05; configure primero el EXE inglés.
    pub fn set_expedition_indigenous_suffix(&mut self, suffix: &[u8]) -> Result<(), String> {
        let resources = self
            .sim
            .host
            .expedition_return_resources
            .as_mut()
            .ok_or("recursos del ejecutable de expedición ausentes")?;
        resources.texts.set_indigenous_suffix(suffix.to_vec());
        Ok(())
    }
    /// Usa únicamente los datos de texto del ejecutable inglés verificado.
    pub fn configure_battle_texts(&mut self, bytes: &[u8]) -> Result<(), String> {
        let battle = p3_sim::battle_text::BattleTextBank::from_english_exe(bytes)?;
        let scripts = p3_sim::mission_text::ScriptTextBank::from_english_exe(bytes)?;
        self.sim.host.pirate_scene_texts = Some(battle.clone());
        self.battle_texts = Some(battle);
        self.letter_texts = Some(p3_sim::letter_text::LetterTextBank::from_english_exe(bytes)?);
        self.original_exe = Some(bytes.to_vec());
        self.sim.host.mission_texts = Some(scripts);
        self.sim.configure_expedition_return_exe(bytes)?;
        let image = p3_assets::exe::PeImage::parse(bytes).map_err(str::to_string)?;
        self.sim.configure_discovery_locations(bytes)?;
        let mut thresholds = [0u8; 256];
        for (i, value) in thresholds.iter_mut().enumerate() {
            *value = image.byte_at(0x673658 + i as u32)?;
        }
        self.sim.host.pirate_group_thresholds = Some(thresholds);
        self.sim.host.town_pirate_piece_bytes = Some((image.byte_at(0x677929)?, image.byte_at(0x67792f)?));
        Ok(())
    }
    pub fn open(save: &[u8], archive: impl Into<p3_assets::cpr::Bytes>) -> Result<Self, String> {
        Self::open_impl(save, archive, false, None, false)
    }
    /// Abre una batalla inicial comercial o de convoyes en pausa para inspección y guardado.
    /// No permite avanzar ni realizar acciones hasta disponer del tick de combate.
    pub fn open_with_paused_battle(save: &[u8], archive: impl Into<p3_assets::cpr::Bytes>) -> Result<Self, String> {
        Self::open_impl(save, archive, true, None, false)
    }
    /// Como `open_with_paused_battle`, con estado CRT externo explícito para
    /// reconstruir proyectiles. La partida original no guarda dicho estado.
    pub fn open_with_paused_battle_and_crt(save: &[u8], archive: impl Into<p3_assets::cpr::Bytes>, crt_state: u32) -> Result<Self, String> {
        Self::open_impl(save, archive, true, Some(crt_state), false)
    }
    /// Abre solo para diagnóstico un asalto pirata ya guardado. El estado CRT
    /// debe proceder del anfitrión: el formato S05 no lo incluye. No habilita
    /// el reloj ni el guardado general de la sesión.
    pub fn open_with_paused_pirate_attack(save: &[u8], archive: impl Into<p3_assets::cpr::Bytes>, crt_state: u32) -> Result<Self, String> {
        Self::open_impl(save, archive, true, Some(crt_state), true)
    }
    fn open_impl(save: &[u8], archive: impl Into<p3_assets::cpr::Bytes>, allow_paused: bool, crt_state: Option<u32>, town_attack: bool) -> Result<Self, String> {
        let archive: p3_assets::cpr::Bytes = archive.into();
        Archive::parse(&archive).map_err(|e| format!("CPR: {e}"))?;
        let model = SaveModel::parse(save).map_err(|e| format!("partida: {e:?}"))?;
        let active = model.sea_battles.first().copied().unwrap_or(0) != 0;
        if active && !allow_paused {
            return Err("las partidas con batallas activas aún no se pueden simular".into());
        }
        let mut sim = Sim::from_save_bytes(save).map_err(|e| e.to_string())?;
        if let Some(state) = crt_state {
            sim.configure_crt_state(state)
        }
        // Hay ediciones con distintos bancos: si no existe el inglés, la
        // simulación deja pendientes las tareas que necesitan ese recurso.
        let _ = sim.configure_hanse_fleet_archives(&[&*archive], "scripts/Namen_eng.ini", false, 0);
        // Guiones de misión de la tarea 0x1B (`missions_addon/`, lista `[Missions]` del INI inglés).
        let _ = sim.configure_mission_script_archives(&[&*archive], "scripts/missions_eng.ini");
        let office = model
            .world
            .offices
            .iter()
            .enumerate()
            .find_map(|(i, r)| {
                if r.len() != p3_save::office::OFFICE_LEN {
                    return None;
                }
                let o = sim.persisted.get(i)?;
                let owner = usize::from(o.stock.owner);
                let city = usize::from(o.stock.town);
                if city < sim.economy.world.towns.len() && sim.rep_merchants.get(owner).is_some_and(|m| m.u16(8) == 0) {
                    Some((i, owner, city))
                } else {
                    None
                }
            })
            .ok_or("la partida no contiene una oficina de un comerciante humano")?;
        sim.economy.world.local_merchant = office.1 as u32;
        if let Some(resources) = sim.host.expedition_resources.as_mut() {
            resources.player = office.1 as u16;
        }
        let block = model.ships.write();
        let ships = p3_save::ships::Ships::parse(&block).map_err(|e| format!("barcos: {e:?}"))?;
        let candidates = (0..ships.ship_slots())
            .filter_map(|i| {
                let image = ships.ship_image(i)?;
                let state = u16::from_le_bytes([image[0x134], image[0x135]]);
                let route = ships.saved_route(i).ok()?;
                let cid = u16::from_le_bytes([image[8], image[9]]) as usize;
                if cid < ships.convoys.len() {
                    let convoy = ships.convoy_image(cid);
                    if u16::from_le_bytes([convoy[0x10], convoy[0x11]]) as usize != i {
                        return None;
                    }
                }
                if usize::from(image[0]) == office.1
                    && ((matches!(state, 2 | 0xf)
                        && matches!(
                            route,
                            p3_save::ships::SavedRoute::Owned(_) | p3_save::ships::SavedRoute::Shared { .. }
                        ))
                        || ((state == 3 || (state == 0xf && image[0x38] == 0xff))
                            && matches!(route, p3_save::ships::SavedRoute::None)))
                {
                    Some(i)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        let snapshot = Some(ships.navigation_snapshot().map_err(str::to_string)?);
        if town_attack && (!active || sim.pending_town_attack_state(0x59000000, 0).is_err()) {
            return Err("la partida no contiene un asalto pirata compatible".into());
        }
        let paused_battle = if active && !town_attack {
            let frame = model
                .current_basic_sea_battles()
                .map_err(|e| format!("batalla activa no compatible: {e:?}"))?;
            if frame.objects.len() != 1 {
                return Err("batalla activa no compatible con pausa".into());
            }
            let naval = snapshot.as_ref().expect("imagen naval");
            let rebuilt = if let Ok(image) = frame.initial_commercial_state(0, 0x59000000, 0) {
                image
                    .rebuild(&naval.ships, 0x59000000)
                    .map_err(|e| format!("batalla activa no compatible: {e:?}"))?
            } else {
                let image = frame
                    .initial_convoy_state(0, 0x59000000, 0)
                    .map_err(|e| format!("batalla activa no compatible: {e:?}"))?;
                p3_save::model::validate_initial_convoy_battle(naval, &image.save_input())?;
                if image.bitmap.is_empty() {
                    image
                        .rebuild_convoy(&naval.ships, 0x59000000)
                        .map_err(|e| format!("batalla activa no compatible: {e:?}"))?
                } else {
                    let state = crt_state.ok_or("la carga de proyectiles requiere estado CRT explícito")?;
                    let mut rng = p3_core::rng::CrtRand { state };
                    let result = image
                        .rebuild_convoy_with_projectiles(&naval.ships, 0x59000000, || rng.rand() as u32)
                        .map_err(|e| format!("batalla activa no compatible: {e:?}"))?;
                    sim.configure_crt_state(rng.state);
                    result
                }
            };
            Some(rebuilt)
        } else {
            None
        };
        let mut session = Self {
            sound_board: None,
            harbor_fx: Default::default(),
            effects_volume: 80,
            pending_collapses: Vec::new(),
            bath_visit: (0, [false; 4]),
            cart_entries: Vec::new(),
            town_grade: None,
            navmap: None,
            navmap_reference: false,
            sim,
            battle_selected: 0xff,
            paused_battle,
            battle_clock: None,
            concurrent_start: None,
            battle_texts: None,
            letter_texts: None,
            original_exe: None,
            model,
            archive,
            changed_ships: Vec::new(),
            planner: None,
            office: office.0,
            owner: office.1,
            city: office.2,
        };
        if let Some(snapshot) = snapshot {
            let resources = session
                .sim
                .host
                .expedition_resources
                .as_ref()
                .ok_or("faltan recursos para reanudar la navegación")?;
            let ports = resources
                .ports
                .iter()
                .map(|p| p.map(|(x, y)| (x as i16, y as i16)).ok_or("puerto ausente".to_string()))
                .collect::<Result<Vec<_>, _>>()?;
            let mut nav = None;
            for &candidate in &candidates {
                let route = ships.saved_route(candidate).map_err(str::to_string)?;
                let shared = if let p3_save::ships::SavedRoute::Shared { origin, destination } = route {
                    let result = session.route_between(origin as usize, destination as usize)?;
                    Some(
                        result["points"]
                            .as_array()
                            .ok_or("ruta inválida")?
                            .iter()
                            .map(|p| {
                                let x = p[0]
                                    .as_i64()
                                    .and_then(|v| i16::try_from(v).ok())
                                    .ok_or("coordenada X inválida")?;
                                let y = p[1]
                                    .as_i64()
                                    .and_then(|v| i16::try_from(v).ok())
                                    .ok_or("coordenada Y inválida")?;
                                Ok((x, y))
                            })
                            .collect::<Result<Vec<_>, String>>()?,
                    )
                } else {
                    None
                };
                if let Some(existing) = nav.as_mut() {
                    let existing: &mut p3_sim::navigation::LiveVoyage = existing;
                    existing.add_resumed(candidate as u16, shared)?
                } else {
                    nav = Some(p3_sim::navigation::LiveVoyage::resume(
                        snapshot.clone(),
                        candidate as u16,
                        ports.clone(),
                        office.1 as u32,
                        shared,
                    )?)
                }
            }
            let nav = nav.unwrap_or_else(|| p3_sim::navigation::LiveVoyage::idle(snapshot, ports, office.1 as u32));
            for &candidate in &candidates {
                let at = candidate * 0x180;
                session.sim.host.expedition_ship_images[candidate] = Some(nav.snapshot.ships[at..at + 0x180].to_vec());
            }
            session.sim.host.navigation = Some(nav);
        }
        if town_attack {
            session.sim.install_pending_town_attack_diagnostic(0x59000000)?;
            session.sim.stage_loaded_pirate_boundary_baseline(&session.model)?;
        }
        Ok(session)
    }
    /// Aplica un contacto de costa explícito a la escena pausada. El
    /// anfitrión suministra proyección y tablas verificadas; no avanza el frame.
    pub fn apply_paused_pirate_coast<F,G>(&mut self,local:usize,weights:&[u8;45],shapes:&[[(i16,i16);5];4],
        sine:&[i32;65],angles:&[u8;65],sound_possible:bool,pixel_to_cell:F,cell_to_pixel:G)->Result<bool,String>
    where F:FnMut(i32,i32)->Result<(u16,u16),&'static str>,G:FnMut(u16,u16)->(u32,u32){
        self.sim.apply_pirate_coast_contact(local,weights,shapes,sine,angles,sound_possible,pixel_to_cell,cell_to_pixel)
    }
    /// Contacto explícito con geometría del CPR y tablas del EXE inglés.
    /// La rama de sonido y el despacho automático del frame siguen pendientes.
    pub fn apply_paused_pirate_coast_from_exe(&mut self,local:usize,exe:&[u8],sound_possible:bool)->Result<bool,String>{
        use sha2::{Digest,Sha256};
        if format!("{:x}",Sha256::digest(exe))!="fe436dc5f8addc4d3aa437eb48a8bc6415d9ba5577c1027d06db9bb915eddece"{return Err("ejecutable inglés no reconocido para costa".into())}
        let image=p3_assets::exe::PeImage::parse(exe).map_err(str::to_owned)?;
        let mut weights=[0u8;45];for (i,v) in weights.iter_mut().enumerate(){*v=image.byte_at(0x67aa70+i as u32)?}
        let mut angles=[0u8;65];for (i,v) in angles.iter_mut().enumerate(){*v=image.byte_at(0x67aa28+i as u32)?}
        let mut sine=[0i32;65];for (i,v) in sine.iter_mut().enumerate(){let p=0x67a920+i as u32*4;*v=i32::from_le_bytes([image.byte_at(p)?,image.byte_at(p+1)?,image.byte_at(p+2)?,image.byte_at(p+3)?]);}
        let mut shapes=[[(0i16,0i16);5];4];for (kind,shape) in shapes.iter_mut().enumerate(){for (i,v) in shape.iter_mut().enumerate(){let p=0x67a8d0+kind as u32*20+i as u32*4;*v=(i16::from_le_bytes([image.byte_at(p)?,image.byte_at(p+1)?]),i16::from_le_bytes([image.byte_at(p+2)?,image.byte_at(p+3)?]));}}
        let projection=self.sim.host.pirate_scene_projection.ok_or("proyección de guarida ausente")?;
        if (projection.width,projection.height,projection.x_offset,projection.y_offset)!=(34,21,17,11){return Err("proyección de costa no contrastada".into())}
        let active=self.sim.host.town_attack_active.as_ref().ok_or("asalto activo ausente")?;
        let read=|at:usize|->Result<u32,String>{Ok(u32::from_le_bytes(active.object.get(at..at+4).ok_or("escena truncada")?.try_into().unwrap()))};
        let (cols,rows)=(read(0x2ee0)?,read(0x2ee4)?);if (cols,rows)!=(128,256){return Err("dimensiones de costa no contrastadas".into())}
        let geom=p3_render::geom::Geom{cols,rows,cell_w:34,cell_h:21,off_x:projection.x_offset,off_y:projection.y_offset};let mask=p3_render::geom::detect_mask();
        self.sim.apply_pirate_coast_contact(local,&weights,&shapes,&sine,&angles,sound_possible,
            |x,y|{let ((a,b),_)=geom.pixel_to_cell(x,y,&mask);Ok((a as u16,b as u16))},
            |x,y|{let (a,b)=geom.cell_to_pixel(i32::from(x),u32::from(y));(a as u32,b as u32)})
    }

    /// Aplica solo la recarga de una pieza de defensa en pausa diagnóstica.
    pub fn apply_paused_pirate_tower_cooldown(&mut self,piece:usize)->Result<(),String>{
        self.sim.apply_pirate_tower_cooldown(piece)
    }

    /// Daño directo de un barco a la torre en pausa diagnóstica.
    pub fn apply_paused_pirate_ship_tower_damage(&mut self,piece:usize,damage:i32)->Result<bool,String>{
        self.sim.apply_pirate_ship_tower_damage(piece,damage)
    }

    /// Ejecuta armas instantáneas de un barco de asedio contra torres.
    pub fn fire_paused_pirate_siege_ship_at_towers(&mut self,id:usize)->Result<usize,String>{
        self.sim.fire_pirate_siege_ship_at_towers(id)
    }

    /// Avanza explícitamente un barco de asedio en pausa diagnóstica.
    pub fn step_paused_pirate_siege_ship(&mut self,id:usize)->Result<usize,String>{
        self.sim.step_pirate_siege_ship(id)
    }

    /// Despacha las parejas espaciales usando casco y geometría verificados.
    pub fn dispatch_paused_pirate_siege_pairs_from_exe(&mut self,exe:&[u8],sound_possible:bool)->Result<(usize,usize),String>{
        use sha2::{Digest,Sha256};
        if format!("{:x}",Sha256::digest(exe))!="fe436dc5f8addc4d3aa437eb48a8bc6415d9ba5577c1027d06db9bb915eddece"{return Err("ejecutable inglés no reconocido para costa".into())}
        let image=p3_assets::exe::PeImage::parse(exe).map_err(str::to_owned)?;
        let mut weights=[0u8;45];for (i,v) in weights.iter_mut().enumerate(){*v=image.byte_at(0x67aa70+i as u32)?}
        let mut angles=[0u8;65];for (i,v) in angles.iter_mut().enumerate(){*v=image.byte_at(0x67aa28+i as u32)?}
        let mut sine=[0i32;65];for (i,v) in sine.iter_mut().enumerate(){let p=0x67a920+i as u32*4;*v=i32::from_le_bytes([image.byte_at(p)?,image.byte_at(p+1)?,image.byte_at(p+2)?,image.byte_at(p+3)?]);}
        let mut shapes=[[(0i16,0i16);5];4];for (kind,shape) in shapes.iter_mut().enumerate(){for (i,v) in shape.iter_mut().enumerate(){let p=0x67a8d0+kind as u32*20+i as u32*4;*v=(i16::from_le_bytes([image.byte_at(p)?,image.byte_at(p+1)?]),i16::from_le_bytes([image.byte_at(p+2)?,image.byte_at(p+3)?]));}}
        let projection=self.sim.host.pirate_scene_projection.ok_or("proyección de guarida ausente")?;
        if (projection.width,projection.height,projection.x_offset,projection.y_offset)!=(34,21,17,11){return Err("proyección de costa no contrastada".into())}
        let active=self.sim.host.town_attack_active.as_ref().ok_or("asalto activo ausente")?;
        let read=|at:usize|->Result<u32,String>{Ok(u32::from_le_bytes(active.object.get(at..at+4).ok_or("escena truncada")?.try_into().unwrap()))};
        let (cols,rows)=(read(0x2ee0)?,read(0x2ee4)?);if cols!=128 || !matches!(rows,226|256){return Err("dimensiones de costa no contrastadas".into())}
        let fine_projection=rows==256;
        let geom=p3_render::geom::Geom{cols,rows:256,cell_w:34,cell_h:21,off_x:projection.x_offset,off_y:projection.y_offset};let mask=p3_render::geom::detect_mask();
        self.sim.dispatch_pirate_siege_pairs(&weights,&shapes,&sine,&angles,sound_possible,
            |x,y|{if !fine_projection{return Err("proyección fina de costa no contrastada")}let ((a,b),_)=geom.pixel_to_cell(x,y,&mask);Ok((a as u16,b as u16))},
            |x,y|{let (a,b)=geom.cell_to_pixel(i32::from(x),u32::from(y));(a as u32,b as u32)})
    }

    /// Avanza un frame urbano acotado usando recursos del ejecutable verificado.
    pub fn advance_paused_pirate_siege_frame_from_exe(&mut self,delta:u32,exe:&[u8],sound_possible:bool)->Result<(usize,usize,usize),String>{
        use sha2::{Digest,Sha256};
        if format!("{:x}",Sha256::digest(exe))!="fe436dc5f8addc4d3aa437eb48a8bc6415d9ba5577c1027d06db9bb915eddece"{return Err("ejecutable inglés no reconocido para asedio".into())}
        let image=p3_assets::exe::PeImage::parse(exe).map_err(str::to_owned)?;
        let mut weights=[0u8;45];for (i,v) in weights.iter_mut().enumerate(){*v=image.byte_at(0x67aa70+i as u32)?}
        let mut angles=[0u8;65];for (i,v) in angles.iter_mut().enumerate(){*v=image.byte_at(0x67aa28+i as u32)?}
        let mut sine=[0i32;65];for (i,v) in sine.iter_mut().enumerate(){let p=0x67a920+i as u32*4;*v=i32::from_le_bytes([image.byte_at(p)?,image.byte_at(p+1)?,image.byte_at(p+2)?,image.byte_at(p+3)?]);}
        let mut shapes=[[(0i16,0i16);5];4];for (kind,shape) in shapes.iter_mut().enumerate(){for (i,v) in shape.iter_mut().enumerate(){let p=0x67a8d0+kind as u32*20+i as u32*4;*v=(i16::from_le_bytes([image.byte_at(p)?,image.byte_at(p+1)?]),i16::from_le_bytes([image.byte_at(p+2)?,image.byte_at(p+3)?]));}}
        let projection=self.sim.host.pirate_scene_projection.ok_or("proyección de guarida ausente")?;
        if (projection.width,projection.height,projection.x_offset,projection.y_offset)!=(34,21,17,11){return Err("proyección de costa no contrastada".into())}
        let active=self.sim.host.town_attack_active.as_ref().ok_or("asalto activo ausente")?;
        let read=|at:usize|->Result<u32,String>{Ok(u32::from_le_bytes(active.object.get(at..at+4).ok_or("escena truncada")?.try_into().unwrap()))};
        let (cols,rows)=(read(0x2ee0)?,read(0x2ee4)?);if cols!=128 || !matches!(rows,226|256){return Err("dimensiones de costa no contrastadas".into())}
        let fine_projection=rows==256;
        let geom=p3_render::geom::Geom{cols,rows:256,cell_w:34,cell_h:21,off_x:projection.x_offset,off_y:projection.y_offset};let mask=p3_render::geom::detect_mask();
        self.sim.advance_pirate_siege_frame(delta,image.byte_at(0x672ccc)?,image.byte_at(0x672cc4)?,&weights,&shapes,&sine,&angles,sound_possible,
            |x,y|{if !fine_projection{return Err("proyección fina de costa no contrastada")}let ((a,b),_)=geom.pixel_to_cell(x,y,&mask);Ok((a as u16,b as u16))},
            |x,y|{let (a,b)=geom.cell_to_pixel(i32::from(x),u32::from(y));(a as u32,b as u32)})
    }

    /// Disparo instantáneo acotado, sin defensores ni efectos externos.
    /// El anfitrión suministra el factor original; no avanza el frame.
    pub fn apply_paused_pirate_tower_instant(&mut self,piece:usize,power:u8)->Result<bool,String>{
        self.sim.apply_pirate_tower_instant(piece,power)
    }

    /// Disparo instantáneo con defensores y tabla angular del EXE verificado.
    pub fn apply_paused_pirate_tower_instant_from_exe(&mut self,piece:usize,power:u8,exe:&[u8])->Result<bool,String>{
        use sha2::{Digest,Sha256};
        if format!("{:x}",Sha256::digest(exe))!="fe436dc5f8addc4d3aa437eb48a8bc6415d9ba5577c1027d06db9bb915eddece"{return Err("ejecutable inglés no reconocido para torres".into())}
        let image=p3_assets::exe::PeImage::parse(exe).map_err(str::to_owned)?;let mut angles=[0u8;65];
        for (i,value) in angles.iter_mut().enumerate(){*value=image.byte_at(0x67aa28+i as u32)?}
        self.sim.apply_pirate_tower_instant_with_defenders(piece,power,&angles)
    }

    /// Crea un proyectil de torre con la tabla angular del EXE verificado.
    pub fn spawn_paused_pirate_tower_projectile_from_exe(&mut self,piece:usize,power:u8,exe:&[u8])->Result<bool,String>{
        use sha2::{Digest,Sha256};
        if format!("{:x}",Sha256::digest(exe))!="fe436dc5f8addc4d3aa437eb48a8bc6415d9ba5577c1027d06db9bb915eddece"{return Err("ejecutable inglés no reconocido para torres".into())}
        let image=p3_assets::exe::PeImage::parse(exe).map_err(str::to_owned)?;let mut angles=[0u8;65];for (i,value) in angles.iter_mut().enumerate(){*value=image.byte_at(0x67aa28+i as u32)?}
        self.sim.spawn_pirate_tower_projectile(piece,power,&angles)
    }

    /// Avanza una vez un proyectil de torre en pausa diagnóstica.
    pub fn advance_paused_pirate_tower_projectile(&mut self,id:usize)->Result<bool,String>{self.sim.advance_pirate_tower_projectile(id)}

    /// Aplica la pareja barco–proyectil de torre en pausa diagnóstica.
    pub fn apply_paused_pirate_tower_projectile_ship_contact(&mut self,ship:u16,projectile:u16)->Result<bool,String>{self.sim.apply_pirate_tower_projectile_ship_contact(ship,projectile)}

    /// Prepara una copia S05 experimental sin confirmar el guardado general.
    /// El serializador rechaza las modificaciones aún no reconciliadas.
    pub fn stage_paused_pirate_attack_save(&self) -> Result<Vec<u8>, String> {
        self.sim.stage_populated_pirate_attack().map(|model| model.write())
    }
    /// Ciudad y plano actuales (diagnóstico y comprobaciones de construcción).
    /// Número del fichero de la ciudad actual (`iso/towns/<id>.*`).
    /// Sonidos de la vista de ciudad en el instante `clock_ms`: los disparadores del puerto (llegada/salida de barcos) pasan por las ranuras
    /// con paridad x86 y cada muestra aceptada se decodifica a `dir/sonido_N.wav`. Devuelve `[{file, id, volume, pan}]` para que el cliente las mezcle.
    /// Registra un edificio derribado en el punto de pantalla `(x, y)` (0x599040); suena en el siguiente `town_sounds`.
    /// Volumen de efectos del jugador (`LSSOUND` de `P2.CFG`, ver [`audio::effects_volume`]); lo usan los sonidos del mapa.
    pub fn set_effects_volume(&mut self, v: u8) {
        self.effects_volume = v;
        if let Some(b) = self.sound_board.as_mut() {
            b.map_volume = v;
        }
    }

    pub fn queue_collapse_sound(&mut self, x: i32, y: i32) {
        self.pending_collapses.push((x, y));
    }

    pub fn town_sounds(&mut self, clock_ms: u32, dir: &Path, req: &SoundRequest) -> Result<Value, String> {
        let archive = Archive::parse(&self.archive).map_err(|e| format!("CPR: {e}"))?;
        let (_, sounds) = self.harbor_scene(&archive, self.town_file_id(), clock_ms);
        let board = match self.sound_board.take() {
            Some(b) => b,
            None => audio::SoundBoard::new(audio::SoundCatalog::from_archive(&archive)?, Default::default()),
        };
        let mut board = board;
        board.map_volume = self.effects_volume;
        board.set_clock(clock_ms);
        let mut played = Vec::new();
        // Sonido del barco ajeno de `0x62018B` (lista 60100), pedido al dibujar la vista.
        let view_sounds = std::mem::take(&mut self.harbor_fx.borrow_mut().sounds);
        for k in sounds.iter().chain(view_sounds.iter()) {
            played.extend(board.play_at(&archive, k.x, k.y, k.sample_id));
        }
        let pending = std::mem::take(&mut self.pending_collapses);
        for &(x, y) in req.collapses.iter().chain(pending.iter()) {
            played.extend(board.collapse(&archive, x, y));
        }
        for &(x, y, kind) in &req.sites {
            board.queue_building(&archive, x, y, kind);
        }
        let before = board.requests.len();
        board.update(&archive);
        played.extend(board.requests[before..].iter().cloned());
        for &k in &req.map_kinds {
            played.extend(board.map_ambient(&archive, k, 0, 0, 0, 0));
        }
        for &(class, id, x, y, extra) in &req.map_events {
            played.extend(board.map_ambient(&archive, class, id, x, y, extra));
        }
        let mut adjust = Vec::new();
        if let Some([x, y, w, h]) = req.map_view {
            board.set_map_view(x, y, w, h);
            if let Some(nav) = self.sim.host.navigation.as_ref() {
                played.extend(board.map_weather(&archive, &nav.snapshot.manager));
            }
            adjust = board.map_frame();
        }
        let stop_all = std::mem::take(&mut board.stopped);
        let mut out = Vec::new();
        for r in played {
            let name = format!("sonido_{}_{}.wav", clock_ms, r.id);
            let pcm = board.catalog.render(&archive, r.id)?;
            std::fs::write(dir.join(&name), pcm.to_wav()).map_err(|e| format!("sonido: {e}"))?;
            out.push(json!({"file": name, "id": r.id, "volume": r.volume, "pan": r.pan}));
        }
        self.sound_board = Some(board);
        let adjust: Vec<Value> = adjust.iter().map(|r| json!({"id": r.id, "volume": r.volume, "pan": r.pan})).collect();
        Ok(json!({"sounds": out, "adjust": adjust, "stop_all": stop_all}))
    }
    /// Transportes terrestres que `0x004D8F00` (`spawn_land_transport_entities`, paridad x86) convierte en carros del cochero al abrir la
    /// vista de la ciudad `town` (posición): tareas `0x0C` del planificador `0x6DD73C` con el bit 0 de `+0xB`, en la fase 1 (cargando en
    /// `+8`) o 6 (llegando a `+9`). Devuelve `(índice de la tarea, entrada del carro)` en el orden en que el original los crea.
    pub fn land_transport_carts(&self, town: u8) -> Vec<(u16, p3_render::cart_field::CartEntry)> {
        use p3_core::economy::sched_queries::{spawn_land_transport_entities, G_TICK};
        const SCHED: u32 = 0x006D_D73C;
        const RECORDS: u32 = 0x1000_0000;
        let sch = &self.sim.host.council_scheduler;
        let mut m = p3_core::flatmem::SparseMem::default();
        let mut head = Vec::with_capacity(0x10);
        head.extend_from_slice(&RECORDS.to_le_bytes());
        head.extend_from_slice(&sch.dispatching.to_le_bytes());
        head.extend_from_slice(&sch.head.to_le_bytes());
        head.extend_from_slice(&sch.free_head.to_le_bytes());
        head.extend_from_slice(&sch.capacity.to_le_bytes());
        m.seed(SCHED, &head);
        for (i, r) in sch.records.iter().enumerate() {
            let mut raw = Vec::with_capacity(0x18);
            raw.extend_from_slice(&r.due.to_le_bytes());
            raw.extend_from_slice(&r.next.to_le_bytes());
            raw.extend_from_slice(&r.kind.to_le_bytes());
            raw.extend_from_slice(&r.payload);
            m.seed(RECORDS + 0x18 * i as u32, &raw);
        }
        m.seed(G_TICK, &self.sim.economy.world.tick.to_le_bytes());
        // `0x00677929`: segundo byte de la tabla de rumbos `0x677928` (00 20 40 .. E0), argumento de rumbo del emisor.
        m.seed(0x0067_7929, &[0x20]);
        spawn_land_transport_entities(&mut m, SCHED, u32::from(town))
            .into_iter()
            .filter_map(|c| {
                let r = sch.records.get(c[3] as usize)?;
                Some((c[3] as u16, p3_render::cart_field::CartEntry { side: r.u8_at(9), frac: r.u8_at(0x16) }))
            })
            .collect()
    }
    pub fn town_file_id(&self) -> u8 {
        self.sim.host.expedition_town_ids[self.city]
    }
    pub fn debug_town(&self) -> (&p3_core::town::Town, &p3_core::buildings::Site) {
        (
            &self.sim.economy.world.towns[self.city],
            &self.sim.host.sites[self.city],
        )
    }
    /// ¿Se puede empezar un edificio de `ty` en (`row`, `col`)? Costes en materiales de ciudad (`0x00521900`) y comprobación del plano (`0x004FD020`, `0x004FB9C0`).
    pub fn build_check(&self, ty: u8, row: u8, col: u8) -> Result<Value, String> {
        use p3_core::buildings::{
            grid::Grid,
            tables::{BRICK, EXTRA_COST, IRON, TIMBER},
        };
        let (town, site) = self.debug_town();
        if ty > 0x35 {
            return Err("tipo de edificio inválido".into());
        }
        let (rows, stride) = (
            town.i32(p3_core::town::off::PLAN_ROWS),
            town.i32(p3_core::town::off::PLAN_STRIDE),
        );
        if i32::from(row) >= rows || i32::from(col) >= stride {
            return Err("casilla fuera del plano".into());
        }
        let mut cells = site.cells.clone();
        // Las casillas junto al borde del plano hacen que el original lea fuera de la rejilla: aquí cuentan como no válidas.
        let placeable = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            Grid {
                stride,
                rows,
                cells: &mut cells,
            }
            .can_place_building_at(u32::from(ty), u32::from(row), u32::from(col), 0)
        }))
        .unwrap_or(0);
        let matches = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            site.cell_matches_building_type(town, u32::from(row), u32::from(col), u32::from(ty), 0, 0xe0)
        }))
        .unwrap_or(false);
        let t = usize::from(ty);
        Ok(
            json!({"type":ty,"row":row,"col":col,"cell":site.cells[usize::try_from(stride).unwrap()*usize::from(row)+usize::from(col)],"placeable":placeable,"matches":matches,"cost":{"brick":i32::from(BRICK[t])*2000,"timber":i32::from(TIMBER[t])*2000,"iron":i32::from(IRON[t])*200,"extra":i32::from(EXTRA_COST[t])*2000},"gold":self.sim.economy.merchants[self.owner].i32(0)}),
        )
    }
    /// Orden de construcción del jugador en la ciudad actual; devuelve el estado nuevo. Las dependencias que el puerto no admite devuelven error sin cambiar nada.
    pub fn build(&mut self, ty: u8, row: u8, col: u8) -> Result<Value, String> {
        if self.sim.host.town_attack_active.is_some() { return Err("asalto urbano activo en pausa".into()); }
        let check = self.build_check(ty, row, col)?;
        if !check["matches"].as_bool().unwrap_or(false) {
            return Err("no se puede construir en esa casilla".into());
        }
        let before = self.sim.economy.merchants[self.owner].i32(0);
        let merchant = u8::try_from(self.owner).map_err(|_| "comerciante inválido")?;
        if ty >= 0x30 {
            // Torres y piezas de defensa (0x30..=0x35): la orden 0x22 del planificador de defensa.
            let town = self.sim.economy.world.towns[self.city].index();
            let op = p3_core::pump::Operation([
                0x22,
                u32::from_le_bytes([merchant, town, row, col]),
                u32::from_le_bytes([ty, 0xe0, 0, 0]),
                0,
                0,
            ]);
            self.sim.start_tower_work(self.city, &op)?;
        } else {
            self.sim.start_building_work(self.city, merchant, row, col, ty, 0)?
        }
        let mut state = self.state();
        state["gold_before"] = json!(before);
        Ok(state)
    }
    /// Nombre de la ciudad (`NamenStaedte_*.txt` del CPR, bytes Latin-1) o `null` si el banco no está cargado.
    fn town_name(&self, city: usize) -> Value {
        self.sim.town_names(city).map_or(Value::Null, |(name, _)| {
            json!(name.iter().map(|&b| b as char).collect::<String>())
        })
    }
    pub fn state(&self) -> Value {
        let s = self.sim.summary();
        let office = &self.sim.persisted[self.office].stock;
        let town = &self.sim.economy.world.towns[self.city];
        let offices = self
            .sim
            .persisted
            .iter()
            .enumerate()
            .filter_map(|(i, o)| {
                let owner = usize::from(o.stock.owner);
                let city = usize::from(o.stock.town);
                if self
                    .model
                    .world
                    .offices
                    .get(i)
                    .is_some_and(|r| r.len() == p3_save::office::OFFICE_LEN)
                    && city < self.sim.economy.world.towns.len()
                    && self.sim.rep_merchants.get(owner).is_some_and(|m| m.u16(8) == 0)
                {
                    Some(
                        json!({"office":i,"owner":owner,"city":city,"town_id":self.sim.host.expedition_town_ids[city]}),
                    )
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        let ships=self.sim.economy.cargo_ships.iter().enumerate().filter_map(|(i,c)|{let image=self.sim.host.expedition_ship_images.get(i)?.as_ref()?;if usize::from(c.owner)!=self.owner{return None}let state=u16::from_le_bytes([image[0x134],image[0x135]]);let x=i16::from_le_bytes([image[0x1e],image[0x1f]]);let y=i16::from_le_bytes([image[0x22],image[0x23]]);Some(json!({"id":i,"owner":c.owner,"town":c.town,"state":state,"x":x,"y":y,"cargo":c.stock,"capacity":c.capacity}))}).collect::<Vec<_>>();
        let destinations = self
            .sim
            .host
            .expedition_town_ids
            .iter()
            .enumerate()
            .map(|(i, id)| json!({"city":i,"town_id":id,"name":self.town_name(i)}))
            .collect::<Vec<_>>();
        let famine_notifications = self
            .sim
            .host
            .famine_notifications
            .iter()
            .map(|n| json!({"merchant":n.merchant,"city":n.town,"local":n.local}))
            .collect::<Vec<_>>();
        json!({"game_end":self.sim.host.game_end_events.last().map(|e|json!({"objective":e.objective,"kind":e.kind,"winner":e.winner,"goal":e.goal,"town":e.notification_home,"timed_out":e.timed_out})),"game_end_stopped":self.sim.host.game_end_stopped,"battle_paused":self.paused_battle.is_some()||self.sim.host.clock_before_battle||self.sim.host.encounter_battle.is_some()||self.sim.host.town_attack_active.is_some(),"battle_ready":self.paused_battle.is_some(),"city":self.city,"town_id":self.sim.host.expedition_town_ids[self.city],"town_name":self.town_name(self.city),"owner":self.owner,"office":self.office,"offices":offices,"ships":ships,"destinations":destinations,"day":s.day,"month":s.month+1,"year":s.year,"tick":s.tick,"gold":self.sim.economy.merchants[self.owner].i32(0),"population":[town.pop(0),town.pop(1),town.pop(2),town.pop(3)],"office_stock":office.stock,"town_stock":(0..24).map(|w|town.stock(w)).collect::<Vec<_>>(),"buildings":self.buildings(),"famine_notifications":famine_notifications,"hash":s.hash})
    }
    /// Ventana «Transporte por tierra» del almacén (`barn_content_window`): ciudad unida por tierra a la de la oficina
    /// (`0x00525630`), cargas disponibles, carros, valor, escolta exigida y coste de la elegida
    /// ([`p3_core::economy::land_transport_send::quote`], paridad x86). `loads` son cargas por mercancía (0..20).
    pub fn land_transport_quote(&self, loads: &[u8; 20], escort: u8) -> Result<Value, String> {
        use p3_core::economy::land_transport_send as lt;
        let ids = &self.sim.host.expedition_town_ids;
        let dest = lt::land_partner(&self.sim.economy.world.towns, ids, self.city);
        let stock = &self.sim.persisted[self.office].stock.stock;
        let q = lt::quote(loads, Some(&stock[..]), escort.min(2));
        let slots: u32 = (0..20).map(|g| u32::from(q.loads[g]) * lt::unit(g) / 200).sum();
        let gold = self.sim.economy.merchants[self.owner].i32(0);
        let costs: Vec<u32> = (0..3).map(|l| lt::escort_cost(l, q.wagons).unwrap_or(0)).collect();
        Ok(json!({
            "origin": self.city, "origin_name": self.town_name(self.city),
            "destination": dest, "destination_name": dest.map_or(Value::Null, |d| self.town_name(usize::from(d))),
            "loads": q.loads, "available": q.available, "units": (0..20).map(lt::unit).collect::<Vec<_>>(),
            "wagons": q.wagons, "value": q.value, "required": q.required, "escort": escort.min(2), "cost": q.cost, "costs": costs,
            "slots": slots, "max_slots": lt::MAX_SLOTS, "gold": gold,
            "can_send": dest.is_some() && q.loads.iter().any(|&l| l != 0) && slots <= lt::MAX_SLOTS && i64::from(gold) >= i64::from(q.cost),
        }))
    }

    /// Envía las cargas por tierra (`0x005EB3C0`): emboscada si la escolta es menor que la exigida, una operación `0x3A` por
    /// mercancía y su ejecución (`Sim::start_land_transport`, paridad x86 con el despachador real). La tirada usa el CRT
    /// compartido si la sesión lo tiene; si no, un CRT propio sembrado con el reloj (sin paridad: el guardado no trae el estado).
    pub fn send_land_transport(&mut self, loads: &[u8; 20], escort: u8) -> Result<Value, String> {
        use p3_core::economy::land_transport_send as lt;
        if self.paused_battle.is_some() || self.sim.host.clock_before_battle || self.sim.host.encounter_battle.is_some() || self.sim.host.town_attack_active.is_some() {
            return Err("batalla activa en pausa".into());
        }
        let ids = self.sim.host.expedition_town_ids.clone();
        let dest = lt::land_partner(&self.sim.economy.world.towns, &ids, self.city).ok_or("esta ciudad no tiene camino por tierra")?;
        let stock = self.sim.persisted[self.office].stock.stock;
        let escort = escort.min(2);
        let q = lt::quote(loads, Some(&stock[..]), escort);
        let slots: u32 = (0..20).map(|g| u32::from(q.loads[g]) * lt::unit(g) / 200).sum();
        if slots > lt::MAX_SLOTS {
            return Err(format!("demasiada carga: {slots} de {} unidades de 200", lt::MAX_SLOTS));
        }
        if q.loads.iter().all(|&l| l == 0) {
            return Err("no hay nada que enviar".into());
        }
        if i64::from(self.sim.economy.merchants[self.owner].i32(0)) < i64::from(q.cost) {
            return Err("oro insuficiente para la escolta".into());
        }
        let w = lt::Window { origin: self.city as u8, destination: dest, loads: q.loads, cost: q.cost, chosen: escort, required: q.required, wagons: q.wagons, player: self.owner as u8 };
        let tick = self.sim.idle.clock.tick;
        let mut shared = self.sim.host.investigation_rng.take();
        let local = shared.is_none();
        let mut rng = shared.take().unwrap_or(p3_core::rng::CrtRand { state: tick });
        let sent = lt::send(&w, &mut || rng.rand() as u32).ok_or("división entre cero en el envío")?;
        if !local {
            self.sim.host.investigation_rng = Some(rng);
        }
        let mut tasks = Vec::new();
        for op in &sent.operations {
            if let Some(t) = self.sim.start_land_transport(*op)? {
                tasks.push(t)
            }
        }
        self.sim.economy.offices[self.office] = self.sim.persisted[self.office].stock.clone();
        Ok(json!({"tasks": tasks, "ambush": sent.ambush, "lost": sent.lost, "local_rand": local, "destination": dest, "state": self.state()}))
    }

    /// Transportes terrestres en curso del jugador (tareas `0x0C`): origen, destino, mercancía, cargas, fase y progreso.
    pub fn land_transports(&self) -> Value {
        let q = &self.sim.host.council_scheduler;
        let list: Vec<Value> = q
            .records
            .iter()
            .enumerate()
            .filter(|(_, r)| r.kind == 0x0C && usize::from(r.u8_at(0xA)) == self.owner)
            .map(|(i, r)| {
                let loads = u16::from_le_bytes([r.u8_at(0x12), r.u8_at(0x13)]);
                json!({"task": i, "origin": r.u8_at(8), "destination": r.u8_at(9), "good": r.u8_at(0x14), "loads": loads,
                       "phase": r.u8_at(0x15), "progress": r.u8_at(0x16), "cart": r.u8_at(0xB) & 1 != 0, "due": r.due})
            })
            .collect();
        json!(list)
    }

    /// Selecciona únicamente oficinas humanas existentes en la partida.
    pub fn select_office(&mut self, index: usize) -> Result<Value, String> {
        if self.sim.host.town_attack_active.is_some() { return Err("asalto urbano activo en pausa".into()); }
        // Las oficinas fundadas en esta sesión aún no están en el modelo guardado: la ranura libre es la de la cadena libre del pool vivo.
        let free = match &self.sim.host.office_pool {
            Some(pool) => {
                let mut at = usize::from(pool.free);
                let mut found = index >= pool.records.len();
                let mut steps = 0;
                while at < pool.records.len() && steps <= pool.records.len() && !found {
                    found = at == index;
                    at = usize::from(u16::from_le_bytes([pool.records[at][0x2c8], pool.records[at][0x2c9]]));
                    steps += 1;
                }
                found
            }
            None => self.model.world.offices.get(index).is_none_or(|r| r.len() != p3_save::office::OFFICE_LEN),
        };
        if free {
            return Err("oficina libre o inexistente".into());
        }
        let office = self.sim.persisted.get(index).ok_or("oficina ausente")?;
        let owner = usize::from(office.stock.owner);
        let city = usize::from(office.stock.town);
        if city >= self.sim.economy.world.towns.len()
            || !self.sim.rep_merchants.get(owner).is_some_and(|m| m.u16(8) == 0)
        {
            return Err("oficina no controlada por un jugador humano".into());
        }
        self.office = index;
        self.owner = owner;
        self.city = city;
        self.sim.economy.world.local_merchant = owner as u32;
        if let Some(resources) = self.sim.host.expedition_resources.as_mut() {
            resources.player = owner as u16
        }
        if let Some(nav) = self.sim.host.navigation.as_mut() {
            nav.player = owner as u32
        }
        Ok(self.state())
    }
    /// Calcula una ruta de puerto a puerto sin iniciar movimiento.
    pub fn preview_route(&mut self, destination: usize) -> Result<Value, String> {
        self.route_between(self.city, destination)
    }
    fn route_between(&mut self, origin: usize, destination: usize) -> Result<Value, String> {
        self.route_from(origin, destination, None)
    }
    /// Planificador de rutas con los datos de navegación del CPR, sin calcular ninguna ruta (el original los carga al abrir
    /// la partida; aquí se crean cuando algo los necesita: rutas, regreso de expediciones, tormentas e icebergs).
    fn ensure_planner(&mut self) -> Result<(), String> {
        use p3_core::route::{
            finder::RoutePlanner,
            planner::{load_nav_data, NavFiles},
        };
        // La flota completa ya tiene el registro de ciudades necesario para
        // identificar llegadas; no lo sustituir por un planificador vacío.
        if self.planner.is_none() {
            self.planner = self.sim.host.navigation_planner.clone();
        }
        if self.planner.is_none() {
            struct Files([Vec<u8>; 3]);
            impl NavFiles for Files {
                fn open(&mut self, i: usize) -> Option<Vec<u8>> {
                    self.0.get(i).cloned()
                }
            }
            let archive = Archive::parse(&self.archive).map_err(|e| format!("CPR: {e}"))?;
            let read = |name: &str| -> Result<Vec<u8>, String> {
                let e = archive.find(name.as_bytes()).ok_or_else(|| format!("falta {name}"))?;
                Ok(archive.read(e).ok_or("navegación truncada")?.to_vec())
            };
            let grid = read("navdata/nav_matrix.dat")?;
            let vector = read("navdata/nav_vec.dat")?;
            let matrix = read("navdata/matrix_int.dat")?;
            if grid.len() < 4 || vector.len() < 4 {
                return Err("datos de navegación truncados".into());
            }
            let width = i16::from_le_bytes([grid[0], grid[1]]);
            let height = i16::from_le_bytes([grid[2], grid[3]]);
            let n = i16::from_le_bytes([vector[0], vector[1]]);
            if width <= 0
                || height <= 0
                || n <= 0
                || grid.len() != 4 + width as usize * height as usize
                || vector.len() != 4 + n as usize * 4
                || matrix.len() != n as usize * n as usize * 6
            {
                return Err("dimensiones de navegación inválidas".into());
            }
            let mut files = Files([grid, vector, matrix]);
            let nav = load_nav_data(&mut files, 0xdd0).ok_or("navegación inválida")?;
            if nav.matrix.iter().flatten().any(|&(_, next)| next < 0 || next >= n) {
                return Err("matriz de navegación contiene nodos inválidos".into());
            }
            for target in 0..n as usize {
                let mut state = vec![0u8; n as usize];
                state[target] = 2;
                for start in 0..n as usize {
                    let mut node = start;
                    let mut path = Vec::new();
                    while state[node] == 0 {
                        state[node] = 1;
                        path.push(node);
                        node = nav.matrix[node][target].1 as usize;
                    }
                    if state[node] == 1 {
                        return Err("matriz de navegación con ciclo".into());
                    }
                    for node in path {
                        state[node] = 2;
                    }
                }
            }
            self.planner = Some(RoutePlanner::new(nav, vec![], vec![], 0, vec![], vec![]));
        }
        self.sim.host.navigation_planner = self.planner.clone();
        Ok(())
    }
    fn route_from(&mut self, origin: usize, destination: usize, position: Option<(i16, i16)>) -> Result<Value, String> {
        use p3_core::route::{
            finder::RouteOut,
        };
        let resources = self
            .sim
            .host
            .expedition_resources
            .as_ref()
            .ok_or("faltan posiciones de puerto en los recursos")?;
        let port = |i: usize| -> Result<(i16, i16), String> {
            let (x, y) = resources.ports.get(i).copied().flatten().ok_or("puerto inexistente")?;
            Ok((
                i16::try_from(x).map_err(|_| "puerto X fuera de rango")?,
                i16::try_from(y).map_err(|_| "puerto Y fuera de rango")?,
            ))
        };
        let start = match position {
            Some(p) => p,
            None => port(origin)?,
        };
        let end = port(destination)?;
        if position.is_none() && destination == origin {
            return Err("elige una ciudad distinta".into());
        }
        self.ensure_planner()?;
        let result = self
            .planner
            .as_mut()
            .expect("planificador inicializado")
            .build_route_from_points(&[start, end]);
        self.sim.host.navigation_planner = self.planner.clone();
        match result.route {
            RouteOut::Built(points) => Ok(json!({"origin":origin,"destination":destination,"points":points})),
            _ => Err("no se ha podido calcular la ruta".into()),
        }
    }
    /// Orden naval limitada a un barco humano en puerto o subordinado de convoy.
    pub fn sail(&mut self, id: usize, destination: usize) -> Result<Value, String> {
        if self.paused_battle.is_some() || self.sim.host.clock_before_battle || self.sim.host.encounter_battle.is_some() || self.sim.host.town_attack_active.is_some()
        {
            return Err("batalla activa en pausa".into());
        }
        if let Some(nav) = &self.sim.host.navigation {
            if !nav.active() {
                let bytes = self.save()?;
                let mut resumed = Session::open(&bytes, self.archive.clone())?;
                resumed.select_office(self.office)?;
                Self::transfer_host_context(&mut self.sim, &mut resumed.sim);
                resumed.battle_texts = self.battle_texts.take();
                resumed.letter_texts = self.letter_texts.take();
                resumed.cart_entries = std::mem::take(&mut self.cart_entries);
                resumed.town_grade = self.town_grade;
                if self.planner.is_some() {
                    resumed.planner = self.planner.take()
                }
                *self = resumed;
            }
        }
        let vessel = self
            .sim
            .host
            .expedition_ship_images
            .get(id)
            .and_then(Option::as_ref)
            .ok_or("barco inexistente")?;
        if usize::from(vessel[0]) != self.owner {
            return Err("el barco debe ser propio".into());
        }
        let state = u16::from_le_bytes([vessel[0x134], vessel[0x135]]);
        let at_sea = state == 0xf;
        let phase = matches!(state, 2 | 3)
            && self
                .sim
                .host
                .navigation
                .as_ref()
                .is_some_and(|n| u16::try_from(id).ok().is_some_and(|id| n.manages_ship(id)));
        let reversing = phase
            && ((state == 2 && usize::from(vessel[0x37]) == destination)
                || (state == 3 && usize::from(vessel[0x39]) == destination));
        let position = (
            i16::from_le_bytes([vessel[0x1e], vessel[0x1f]]),
            i16::from_le_bytes([vessel[0x22], vessel[0x23]]),
        );
        if !at_sea && !phase && usize::from(vessel[0x39]) != self.city {
            return Err("el barco debe estar en la ciudad seleccionada o navegando".into());
        }
        let route = if reversing {
            json!({"points":[]})
        } else if at_sea || phase {
            self.route_from(self.city, destination, Some(position))?
        } else {
            self.preview_route(destination)?
        };
        let points = route["points"]
            .as_array()
            .ok_or("ruta inválida")?
            .iter()
            .map(|p| {
                let x = p[0]
                    .as_i64()
                    .and_then(|v| i16::try_from(v).ok())
                    .ok_or("coordenada X inválida")?;
                let y = p[1]
                    .as_i64()
                    .and_then(|v| i16::try_from(v).ok())
                    .ok_or("coordenada Y inválida")?;
                Ok((x, y))
            })
            .collect::<Result<Vec<_>, String>>()?;
        let ports = self
            .sim
            .host
            .expedition_resources
            .as_ref()
            .ok_or("faltan puertos")?
            .ports
            .iter()
            .map(|p| p.map(|(x, y)| (x as i16, y as i16)).ok_or("puerto ausente".to_string()))
            .collect::<Result<Vec<_>, _>>()?;
        let id = u16::try_from(id).map_err(|_| "barco inválido")?;
        let dest = u8::try_from(destination).map_err(|_| "destino inválido")?;
        if let Some(nav) = self.sim.host.navigation.as_mut() {
            let before = nav.snapshot.clone();
            if at_sea {
                nav.reroute_at_sea(
                    id,
                    dest,
                    points,
                    &self.sim.economy.world.towns,
                    &mut self.sim.rep_merchants,
                    self.sim.idle.clock.tick,
                )?
            } else if phase {
                nav.reverse_phase(
                    id,
                    dest,
                    points,
                    &self.sim.economy.world.towns,
                    &mut self.sim.rep_merchants,
                    self.sim.idle.clock.tick,
                )?
            } else if nav.ship == u16::MAX {
                let baseline = nav.baseline.clone();
                let mut next = p3_sim::navigation::LiveVoyage::begin_with_merchants(
                    nav.snapshot.clone(),
                    id,
                    dest,
                    points,
                    nav.ports.clone(),
                    &self.sim.economy.world.towns,
                    &mut self.sim.rep_merchants,
                    self.owner as u32,
                    self.sim.idle.clock.tick,
                )?;
                next.baseline = baseline;
                *nav = next
            } else if nav.ship == id || nav.additional.iter().any(|v| v.ship == id) {
                // La flota completa puede seguir administrando un viaje que ya
                // llegó. Para una nueva orden reutiliza el inicio C07 existente y
                // reemplaza solo esa entrada, sin perder los demás recorridos.
                let next = p3_sim::navigation::LiveVoyage::begin_with_merchants(
                    nav.snapshot.clone(),
                    id,
                    dest,
                    points,
                    nav.ports.clone(),
                    &self.sim.economy.world.towns,
                    &mut self.sim.rep_merchants,
                    self.owner as u32,
                    self.sim.idle.clock.tick,
                )?;
                nav.snapshot = next.snapshot;
                if nav.ship == id {
                    nav.convoy = next.convoy;
                    nav.points = next.points
                } else {
                    let current = nav
                        .additional
                        .iter_mut()
                        .find(|v| v.ship == id)
                        .expect("viaje existente");
                    current.convoy = next.convoy;
                    current.points = next.points;
                }
            } else {
                nav.add_order_with_merchants(
                    id,
                    dest,
                    points,
                    &self.sim.economy.world.towns,
                    &mut self.sim.rep_merchants,
                    self.sim.idle.clock.tick,
                )?
            };
            self.sim.host.publish_navigation_delta(
                &before,
                &mut self.sim.economy.world,
                &mut self.sim.economy.cargo_ships,
            );
            let gold = self.sim.rep_merchants[self.owner].i32(0);
            self.sim.economy.merchants[self.owner].set_i32(0, gold);
            self.sim.economy.world.merchants[self.owner].gold = gold;
            return Ok(self.state());
        }
        if at_sea {
            return Err("el viaje debe estar gestionado por la sesión".into());
        }
        let saved = self.save()?;
        let parsed = p3_save::Save::parse(&saved).map_err(|e| format!("guardado: {e}"))?;
        let ships = p3_save::ships::Ships::parse(parsed.block_named("ships").ok_or("bloque naval ausente")?)
            .map_err(|e| format!("barcos: {e:?}"))?;
        let snapshot = ships.navigation_snapshot().map_err(str::to_string)?;
        let nav = p3_sim::navigation::LiveVoyage::begin_with_merchants(
            snapshot,
            id,
            dest,
            points,
            ports,
            &self.sim.economy.world.towns,
            &mut self.sim.rep_merchants,
            self.owner as u32,
            self.sim.idle.clock.tick,
        )?;
        let baseline = nav.baseline.clone();
        let gold = self.sim.rep_merchants[self.owner].i32(0);
        self.sim.economy.merchants[self.owner].set_i32(0, gold);
        self.sim.economy.world.merchants[self.owner].gold = gold;
        self.sim.host.navigation = Some(nav);
        self.sim.host.navigation_error = None;
        self.sim.host.publish_navigation_delta(
            &baseline,
            &mut self.sim.economy.world,
            &mut self.sim.economy.cargo_ships,
        );
        Ok(self.state())
    }
    /// Representación esquemática de la rejilla de navegación del CPR.
    pub fn render_navigation(&self, path: &Path) -> Result<(), String> {
        let grid = &self.planner.as_ref().ok_or("planificador no inicializado")?.nav.grid;
        let mut rgba = Vec::with_capacity(grid.cells.len() * 4);
        for &cell in &grid.cells {
            rgba.extend_from_slice(if cell == 0 {
                &[30, 75, 101, 255]
            } else {
                &[86, 103, 65, 255]
            })
        }
        p3_render::write_png(path, grid.width as usize, grid.height as usize, &rgba)
            .map_err(|e| format!("mapa de navegación: {e}"))
    }
    /// Mapa del mundo: escribe `images/vollansichtskarte.bmp` como PNG y devuelve las ciudades (con su nombre y posición en
    /// píxeles de esa imagen) y todos los barcos de la partida en el mismo espacio. Los nombres son los del comentario que
    /// precede a cada sección de `scripts/StadtDaten.ini`; el dibujo de símbolos y rótulos lo hace el cliente.
    pub fn render_world(&self, path: &Path) -> Result<Value, String> {
        let archive = Archive::parse(&self.archive).map_err(|e| format!("CPR: {e}"))?;
        let entry = archive
            .find(b"images/vollansichtskarte.bmp")
            .ok_or("falta images/vollansichtskarte.bmp")?;
        let (w, h, rgba) = p3_render::decode_bmp(archive.read(entry).ok_or("mapa comprimido o truncado")?)
            .ok_or("BMP del mapa no admitido")?;
        p3_render::write_png(path, w, h, &rgba).map_err(|e| format!("imagen: {e}"))?;
        let ini = archive
            .find(b"scripts/StadtDaten.ini")
            .and_then(|e| archive.read(e))
            .ok_or("falta scripts/StadtDaten.ini")?;
        let towns = world::parse_towns(&String::from_utf8_lossy(ini));
        let ids = &self.sim.host.expedition_town_ids;
        let here = ids.get(self.city).copied();
        let cities = ids
            .iter()
            .enumerate()
            .filter_map(|(city, id)| {
                let t = towns.iter().find(|t| t.id == usize::from(*id))?;
                let mut v = world::town_json(t, Some(*id) == here);
                v["city"] = json!(city);
                Some(v)
            })
            .collect::<Vec<_>>();
        let ships=self.sim.economy.cargo_ships.iter().enumerate().filter_map(|(i,c)|{let image=self.sim.host.expedition_ship_images.get(i)?.as_ref()?;let x=i16::from_le_bytes([image[0x1e],image[0x1f]]);let y=i16::from_le_bytes([image[0x22],image[0x23]]);let state=u16::from_le_bytes([image[0x134],image[0x135]]);if x<0||y<0||i32::from(x)>=w as i32||i32::from(y)>=h as i32{return None}Some(json!({"id":i,"owner":c.owner,"own":usize::from(c.owner)==self.owner,"x":x,"y":y,"state":state,"at_sea":state==0xf,"convoy":u16::from_le_bytes([image[8],image[9]]),"kind":image[0xe]}))}).collect::<Vec<_>>();
        Ok(json!({"width":w,"height":h,"cities":cities,"ships":ships}))
    }
    /// Cadenas que dibuja `town_view_redraw_all_buildings` (0x00521580).
    /// Los marcadores son diagnósticos, no sustituyen a los sprites originales.
    /// Casillas `(x, y)` (con el origen del plano) de los edificios dibujables de la ciudad actual; sirve para colocar grietas en `Effects`.
    pub fn building_cells(&self) -> Vec<(u32, u32)> {
        self.buildings()
            .iter()
            .filter_map(|b| {
                Some((
                    u32::try_from(b["x"].as_i64()?).ok()?,
                    u32::try_from(b["y"].as_i64()?).ok()?,
                ))
            })
            .collect()
    }
    /// Grietas que pide el conductor del original (`0x59BAF0`, con paridad x86) para los negocios del jugador en la ciudad actual: `(x, y, intensidad)`.
    /// La imagen se arma con las ciudades vivas de `sim`, las oficinas de `sim.persisted` y los negocios del guardado (sin los cambios de la simulación).
    pub fn conductor_cracks(&self) -> Vec<(u32, u32, u32)> {
        self.conductor_cracks_for(self.owner, self.city)
    }
    /// Igual con otro comerciante y otra ciudad (el original solo evalúa al jugador local; sirve para ejercitar el conductor con negocios de la IA).
    pub fn conductor_cracks_for(&self, owner: usize, city: usize) -> Vec<(u32, u32, u32)> {
        use p3_render::crack_conductor as cc;
        use p3_render::scene_math::Mem;
        const TOWNS: u32 = 0x1000_0000;
        const OFFICES: u32 = 0x2000_0000;
        const TYPES: u32 = 0x3000_0000;
        const SITES: u32 = 0x4000_0000;
        const SCENE: u32 = 0x5000_0000;
        const MGR: u32 = 0x006D_E4A0;
        let towns = &self.sim.economy.world.towns;
        let mut tb = Vec::new();
        for (i, t) in towns.iter().enumerate() {
            let mut b = t.0.to_vec();
            let n = self.sim.host.sites.get(i).map_or(0, |s| s.records.len() / 8) as u16;
            b[0x75C..0x760].copy_from_slice(&(SITES + 0x10000 * i as u32).to_le_bytes());
            b[0x776..0x778].copy_from_slice(&n.to_le_bytes());
            tb.extend_from_slice(&b)
        }
        let mut ob = Vec::new();
        for (i, o) in self.sim.persisted.iter().enumerate() {
            let ok = self
                .model
                .world
                .offices
                .get(i)
                .is_some_and(|r| r.len() == p3_save::office::OFFICE_LEN);
            let mut d = if ok {
                p3_save::office::decode_office(&o.to_save_v85()).to_vec()
            } else {
                vec![0; 0x44C]
            };
            d.resize(0x44C, 0);
            if !ok {
                d[0x2C4..0x2C6].copy_from_slice(&0xFFFFu16.to_le_bytes())
            }
            ob.extend_from_slice(&d)
        }
        let tyb: Vec<u8> = self
            .model
            .world
            .businesses
            .iter()
            .flat_map(|r| {
                let mut r = r.clone();
                r.resize(0x14, 0);
                r
            })
            .collect();
        let mut img = Image(vec![
            (MGR, vec![0; 0x80]),
            (0x006D_FC14, vec![0; 4]),
            (0x006D_E4B0, vec![0; 4]),
            (TOWNS, tb),
            (OFFICES, ob),
            (TYPES, tyb),
            (SCENE, vec![0; 0xC400]),
            (0x7000_0000, vec![0; 0x40]),
        ]);
        // Solo faltan los segmentos de edificios por ciudad: uno por ciudad.
        for (i, s) in self.sim.host.sites.iter().enumerate() {
            img.0.push((SITES + 0x10000 * i as u32, s.records.clone()))
        }
        img.w32(MGR + 0x68, TOWNS);
        img.w32(MGR + 0x70, TYPES);
        img.w32(MGR + 0x74, OFFICES);
        img.w32(MGR + 0x10, towns.len() as u32);
        img.w16(MGR + 8, self.sim.persisted.len() as u16);
        img.w16(MGR + 0xA, self.sim.rep_merchants.len() as u16);
        img.w16(MGR + 6, self.model.world.businesses.len() as u16);
        img.w32(0x006D_E4B0, towns.len() as u32);
        img.w32(0x006D_FC14, owner as u32);
        img.w32(SCENE + 0xC324, city as u32);
        let mut log = CrackLog(vec![]);
        cc::conduct(&mut img, SCENE, 0x7000_0000, &mut log);
        log.0
    }
    /// Estado de obra o de incendio que `0x00521310` entrega a `0x005891C0` (y por él al virtual `+0x68` de la clase del edificio: `0x550550`/`0x550770`):
    /// con `work != 0` y tipo distinto de `0x28`, en obra `(a, b) = (T[tipo] - work, T[tipo])` (`T` = `0x00672BB4`, bytes de `p3_core::buildings::tables::RECORD_BYTE_6`)
    /// y en la lista de edificios en destrucción (`Town+0x76A`) `(a, b) = (work, 0x32)` con indicador 1. Devuelve `(x, y, tipo, a, b, quemándose)`.
    pub fn building_progress(&self) -> Vec<(u8, u8, u8, u32, u32, bool)> {
        self.buildings()
            .iter()
            .filter_map(|b| {
                let (x, y, kind, work) = (
                    b["x"].as_u64()? as u8,
                    b["y"].as_u64()? as u8,
                    b["kind"].as_u64()? as u8,
                    b["work"].as_u64()? as u32,
                );
                if work == 0 || kind == 0x28 {
                    return None;
                }
                let burning = b["burning"].as_bool()?;
                let total = u32::from(*p3_core::buildings::tables::RECORD_BYTE_6.get(usize::from(kind))?);
                Some(if burning {
                    (x, y, kind, work, 0x32, true)
                } else {
                    (x, y, kind, total.wrapping_sub(work), total, false)
                })
            })
            .collect()
    }
    fn buildings(&self) -> Vec<Value> {
        let town = &self.sim.economy.world.towns[self.city];
        let records = &self.sim.host.sites[self.city].records;
        let count = records.len() / 8;
        let mut heads = vec![town.u16(0x762), town.u16(0x764), town.u16(0x768), town.u16(0x76a)];
        for (i, o) in self.sim.persisted.iter().enumerate() {
            if usize::from(o.stock.town) != self.city
                || self
                    .model
                    .world
                    .offices
                    .get(i)
                    .is_none_or(|r| r.len() != p3_save::office::OFFICE_LEN)
            {
                continue;
            }
            let data = p3_save::office::decode_office(&o.to_save_v85());
            heads.push(u16::from_le_bytes([data[0x2ce], data[0x2cf]]));
            heads.push(u16::from_le_bytes([data[0x2d0], data[0x2d1]]));
        }
        let mut seen = vec![false; count];
        let mut out = Vec::new();
        for (list, mut cursor) in heads.into_iter().enumerate() {
            while usize::from(cursor) < count {
                let i = usize::from(cursor);
                if seen[i] {
                    break;
                }
                seen[i] = true;
                let r = &records[i * 8..i * 8 + 8];
                out.push(json!({"record":i,"x":r[0],"y":r[1],"owner":r[2],"kind":r[3],"work":r[6],"variant":r[7],"burning":list==3}));
                cursor = u16::from_le_bytes([r[4], r[5]]);
            }
        }
        out
    }
    /// Objetos cuya hoja depende del estilo de la ciudad (`Region` de StadtDaten.ini) y de datos del plano:
    /// iglesia (clase 4: estilo `Region`, fase `Town+0x7A3`), ayuntamiento (3: estilo `Region`, regla inferida),
    /// torres de artillería (41: estilo `Region`, variante 0) y puertas (47: tres piezas en fila, estilo en matriz[3]).
    /// Si la hoja del estilo no existe se usa el 2, como `0x005928A0`.
    fn matrix_objects(
        &self,
        archive: &Archive,
        town_id: u8,
        origin: (i32, i32),
        map: &TownMap,
    ) -> Vec<(u8, i32, i32, Vec<Vec<(usize, u32)>>)> {
        let region = archive
            .find(b"scripts/StadtDaten.ini")
            .and_then(|e| archive.read(e))
            .map(|d| world::parse_towns(&String::from_utf8_lossy(d)))
            .and_then(|t| t.into_iter().find(|t| t.id == usize::from(town_id)))
            .map_or(2, |t| t.region);
        let level = self.sim.economy.world.towns[self.city].u8(0x7a3) as u32;
        let step = |x: i32, y: i32, dir: u32| -> (i32, i32) {
            if dir == 0x20 {
                (x + (y & 1), y - 1)
            } else {
                let ny = y - 1;
                (x - (ny & 1), ny)
            }
        };
        let mut out = Vec::new();
        for b in self.buildings() {
            let (Some(kind), Some(x), Some(y)) = (b["kind"].as_u64(), b["x"].as_i64(), b["y"].as_i64()) else {
                continue;
            };
            let (x, y) = (x as i32 + origin.0, y as i32 + origin.1);
            if x < 0 || y < 0 || x >= map.width as i32 || y >= map.height as i32 {
                continue;
            }
            let styled = |index: usize, rest: &[(usize, u32)]| -> Vec<Vec<(usize, u32)>> {
                [region, 2]
                    .iter()
                    .map(|&g| {
                        let mut r = vec![(index, g)];
                        r.extend_from_slice(rest);
                        r
                    })
                    .collect()
            };
            match kind {
                0x1f => out.push((4, x, y, vec![vec![(1, region), (2, level)]])),
                0x20 => out.push((3, x, y, styled(1, &[]))),
                0x2d => {
                    let variant = b["variant"].as_u64().unwrap_or(0) as u32;
                    let owner = b["owner"].as_u64().unwrap_or(255);
                    // 0x00521310: con propietario válido, clase 55 con el índice del registro; si no, la estatua de la ciudad (clase 10, clave DAT_00672d28[town_id]).
                    if owner < self.sim.economy.merchants.len() as u64 {
                        out.push((55, x, y, vec![vec![(1, variant)]]))
                    } else if let Some(&key) = STATUE_KEY.get(usize::from(town_id)) {
                        out.push((10, x, y, vec![vec![(1, u32::from(key))]]))
                    }
                }
                0x30 | 0x32 => out.push((41, x, y, styled(1, &[(2, 0)]))),
                0x37 | 0x38 => {
                    let dir = b["variant"].as_u64().unwrap_or(0) as u32;
                    let (mut gx, mut gy) = (x, y);
                    for piece in 0..3u32 {
                        out.push((47, gx, gy, styled(3, &[(1, piece), (2, dir)])));
                        for _ in 0..2 {
                            (gx, gy) = step(gx, gy, dir)
                        }
                    }
                }
                _ => {}
            }
        }
        out
    }
    /// Edificios en obra o en destrucción: se vuelve a dibujar la ciudad sin ellos y se compone encima el revelado de `0x550770` (ver `p3_render::progress`).
    #[allow(clippy::too_many_arguments)]
    fn compose_progress<'a>(
        &self,
        archive: &'a Archive<'a>,
        scene: &mut p3_render::Scene<'a>,
        objects: &[(u8, i32, i32, Option<(u8, u8)>)],
        matrix_objects: &[(u8, i32, i32, Vec<Vec<(usize, u32)>>)],
        map0: TownMap,
        origin: (i32, i32),
        placed: &[(i32, i32, u16)],
        w: usize,
        h: usize,
        rgba: &mut [u8],
        town_id: u8,
    ) -> usize {
        use p3_render::progress::{FireArt, Request};
        let progress = self.building_progress();
        let key = |x: u8, y: u8| (i32::from(x) + origin.0, i32::from(y) + origin.1);
        let mut requests = Vec::new();
        let mut hidden = std::collections::HashSet::new();
        for &(x, y, _, a, b, burning) in &progress {
            let (cx, cy) = key(x, y);
            let Some(&(_, _, tile)) = placed.iter().find(|p| p.0 == cx && p.1 == cy) else {
                continue;
            };
            let (px, py) = p3_render::cell_to_screen(cx, cy);
            hidden.insert((cx, cy));
            requests.push(Request {
                origin: (px + p3_render::TILE_W, py + p3_render::TILE_H - 10),
                tile,
                a,
                b,
                burning,
            });
        }
        if requests.is_empty() {
            return 0;
        }
        let kept: Vec<_> = objects
            .iter()
            .filter(|o| !hidden.contains(&(o.1, o.2)))
            .cloned()
            .collect();
        let mut scene2 = p3_render::Scene::new(archive);
        let mut map2 = map0;
        scene2.draw_unambiguous_objects(&mut [], 0, 0, &kept);
        scene2.draw_matrix_objects(&mut [], 0, 0, matrix_objects);
        scene2.apply_objects(&mut map2);
        // Misma ciudad sin los edificios en obra: las texturas son casi todas las de `scene`, ya decodificadas.
        let (_, _, base) = scene2.render_with_textures_of(scene, &map2, &[0, 1]);
        let mut raw: Vec<(&[u8], p3_assets::let_sheet::LetSheet)> = Vec::new();
        for entry in &archive.entries {
            let name = entry.name.to_ascii_lowercase();
            if !(name.starts_with(b"iso/let/") || name.starts_with(b"iso\\let\\")) {
                continue;
            }
            if let Some(d) = archive.read(entry) {
                if let Ok(sh) = p3_assets::let_sheet::LetSheet::parse(d) {
                    raw.push((d, sh))
                }
            }
        }
        let own = |t: &p3_render::Texture| p3_render::Texture {
            width: t.width,
            height: t.height,
            rgba: t.rgba.clone(),
        };
        let Some(art) = FireArt::load(&raw, |p| scene.texture_of(p).map(own)) else {
            return 0;
        };
        let mut by_tile: std::collections::HashMap<u16, (&[u8], &p3_assets::let_sheet::LetSheet)> = Default::default();
        for (d, sh) in &raw {
            by_tile.entry(sh.tile_id).or_insert((*d, sh));
        }
        let mut rng = p3_render::anim::CrtRand(u32::from(town_id));
        art.compose(
            rgba,
            &base,
            w,
            h,
            &requests,
            &by_tile,
            |p| scene.texture_of(p).map(own),
            &mut rng,
        )
    }
    pub fn render(&self, path: &Path) -> Result<Value, String> {
        self.render_at(path, 0)
    }
    /// Dibuja la ciudad con las animaciones en el reloj `clock_ms` (milisegundos del juego): se avanzan en pasos de 100 ms desde 0
    /// con el `rand` del CRT sembrado por la ciudad, de modo que el mismo reloj da siempre la misma imagen.
    pub fn render_at(&self, path: &Path, clock_ms: u32) -> Result<Value, String> {
        self.render_fx(path, clock_ms, &Effects::default())
    }
    /// `render_at` con precipitación y grietas encima de la ciudad (ver [`Effects`]).
    /// Edificios de la ciudad actual para el mapa (`(gráfico, x, y, variante)` en casillas del mapa), como los coloca `render_fx`.
    fn town_objects(&self, map: &TownMap, origin: (i32, i32)) -> Vec<(u8, i32, i32, Option<(u8, u8)>)> {
        self
            .buildings()
            .iter()
            .filter_map(|b| {
                let k = b["kind"].as_u64()? as usize;
                if matches!(k, 0x1f | 0x20 | 0x2d | 0x30 | 0x32 | 0x37 | 0x38) {
                    return None;
                }
                let graphic = *BUILDING_GRAPHIC.get(k)?;
                let x = b["x"].as_i64()? as i32 + origin.0;
                let y = b["y"].as_i64()? as i32 + origin.1;
                if x < 0 || y < 0 || x >= map.width as i32 || y >= map.height as i32 {
                    return None;
                }
                let variant = if (0x15..=0x1d).contains(&k) {
                    Some((
                        if b["work"].as_u64()? == 0 {
                            ((k - 0x15) % 3) as u8
                        } else {
                            0
                        },
                        b["variant"].as_u64()? as u8,
                    ))
                } else if k == 0x36 {
                    Some((b["variant"].as_u64()? as u8, 0))
                } else {
                    None
                };
                Some((graphic, x, y, variant))
            })
            .collect::<Vec<_>>()
    }
    /// Mapa de la ciudad actual con los edificios colocados en la capa 1 (`0x0058DB40`), como lo dibuja `render_fx`.
    fn placed_town_map<'a>(&self, archive: &'a Archive<'a>) -> Result<(TownMap, p3_render::Scene<'a>, u8), String> {
        let id = self.sim.host.expedition_town_ids[self.city];
        let name = format!("iso/towns/{id}.town");
        let entry = archive.find(name.as_bytes()).ok_or_else(|| format!("no existe {name}"))?;
        let mut map = TownMap::parse(archive.read(entry).ok_or("mapa comprimido o truncado")?).map_err(|e| format!("mapa: {e:?}"))?;
        let mut scene = p3_render::Scene::new(archive);
        let origin = map.start_point.unwrap_or((0, 0));
        let objects = self.town_objects(&map, origin);
        scene.draw_unambiguous_objects(&mut [], 0, 0, &objects);
        let matrix_objects = self.matrix_objects(archive, id, origin, &map);
        scene.draw_matrix_objects(&mut [], 0, 0, &matrix_objects);
        scene.apply_objects(&mut map);
        Ok((map, scene, id))
    }
    pub fn render_fx(&self, path: &Path, clock_ms: u32, fx: &Effects) -> Result<Value, String> {
        let (v, (w, h, rgba)) = self.render_fx_rgba(clock_ms, fx)?;
        p3_render::write_png(path, w, h, &rgba).map_err(|e| format!("imagen: {e}"))?;
        Ok(v)
    }
    /// `render_fx` sin fichero: devuelve la imagen entera de la ciudad en RGBA (`ancho`, `alto`, píxeles). Lo usa `p3-client`.
    pub fn render_fx_rgba(&self, clock_ms: u32, fx: &Effects) -> Result<(Value, (usize, usize, Vec<u8>)), String> {
        let archive = Archive::parse(&self.archive).map_err(|e| format!("CPR: {e}"))?;
        let id = self.sim.host.expedition_town_ids[self.city];
        let name = format!("iso/towns/{id}.town");
        let entry = archive
            .find(name.as_bytes())
            .ok_or_else(|| format!("no existe {name}"))?;
        let mut map = TownMap::parse(archive.read(entry).ok_or("mapa comprimido o truncado")?)
            .map_err(|e| format!("mapa: {e:?}"))?;
        let mut scene = p3_render::Scene::new(&archive);
        let origin = map.start_point.unwrap_or((0, 0));
        let objects = self.town_objects(&map, origin);
        let map0 = map.clone();
        scene.draw_unambiguous_objects(&mut [], 0, 0, &objects);
        let placed0 = scene.placed_objects().to_vec();
        let matrix_objects = self.matrix_objects(&archive, id, origin, &map);
        scene.draw_matrix_objects(&mut [], 0, 0, &matrix_objects);
        let drawn = scene.apply_objects(&mut map);
        let mut rng = p3_render::anim::CrtRand(id as u32);
        let mut t = 0u32;
        loop {
            scene.advance_animations(t, &mut rng);
            if t >= clock_ms {
                break;
            }
            t = (t + 100).min(clock_ms)
        }
        let shown = scene.animators.iter().filter(|a| a.shown.is_some()).count();
        let (w, h, mut rgba) = scene.render(&map, &[0, 1]);
        let progress_drawn = self.compose_progress(
            &archive,
            &mut scene,
            &objects,
            &matrix_objects,
            map0,
            origin,
            &placed0,
            w,
            h,
            &mut rgba,
            id,
        );
        let (figures_drawn, footprints_drawn) = self.draw_figures(&archive, &mut scene, id, &map, &mut rgba, w, h, clock_ms);
        let ships_drawn = self.draw_harbor_ships(&archive, &mut scene, id, &mut rgba, w, h, clock_ms);
        let fx_drawn = self.draw_effects(&archive, &mut scene, &objects, &map, &mut rgba, w, h, clock_ms, fx)?;
        // Marcadores de los puntos de entrada de la vista de asedio abierta (`Session::open_siege_view`).
        let (siege_drawn, siege_figures) = if self.sim.siege_view(self.city).is_some() {
            let figures = scene.load_figures();
            let fighters = self.draw_siege_figures(&mut scene, &figures, &mut rgba, w, h, (0, 21));
            (self.draw_siege_markers(&mut scene, &figures, &mut rgba, w, h, (0, 21)), fighters)
        } else {
            (0, 0)
        };
        let (x, y) = p3_render::cell_to_screen(map.view_start.0 as i32, map.view_start.1 as i32);
        // Sitios de sonido de edificio: cada animador visible llama a 0x594C00 con su posición en pantalla y su clase de ficha (0x552C10 y vecinos).
        let sound_sites: Vec<Value> = scene
            .animators
            .iter()
            .filter(|a| a.shown.is_some())
            .filter_map(|a| {
                // Clase de ficha (`LetSheet::field_2d`, método virtual +0x50): índice de la lista de sonido de edificio.
                let kind = scene.tile_kind(a.sheet)?;
                let (cx, cy) = p3_render::cell_to_screen(a.cell.0, a.cell.1);
                Some(json!([cx - x, cy - y, kind]))
            })
            .collect();
        // Edificios en llamas: 0x550A70 llama a 0x594C00 con la clase 0 (lista 10000, «Feuer») en la posición de las llamas.
        let mut sound_sites = sound_sites;
        for &(bx, by, _, _, _, burning) in &self.building_progress() {
            if burning {
                let (px, py) = p3_render::cell_to_screen(i32::from(bx) + origin.0, i32::from(by) + origin.1);
                sound_sites.push(json!([px + p3_render::TILE_W - x, py + p3_render::TILE_H - 10 - y, 0]));
            }
        }
        Ok((
            json!({"width":w,"height":h,"view_start":map.view_start,"origin":origin,"focus":[x+p3_render::TILE_W,y+p3_render::TILE_H],"buildings_drawn":drawn,"buildings_total":objects.len()+self.buildings().iter().filter(|b|matches!(b["kind"].as_u64(),Some(0x1f|0x20|0x2d|0x30|0x32|0x37|0x38))).count(),"unresolved":scene.unresolved.iter().map(|(k,n,v)|json!([k,n,v])).collect::<Vec<_>>(),"missing_sheets":scene.missing_sheets,"bad_frames":scene.bad_frames,"unsupported_textures":scene.unsupported_textures,"animators":scene.animators.len(),"animations_shown":shown,"bad_animators":scene.bad_animators,"flame_groups":scene.flame_groups,"animations":scene.animators.iter().map(|a|json!({"cell":[a.cell.0,a.cell.1],"sheet":a.sheet,"tag":a.tag,"kind":format!("{:?}",a.kind),"shown":a.shown.map(|d|json!([d.anim,d.frame]))})).collect::<Vec<_>>(),"clock_ms":clock_ms,"sound_sites":sound_sites,"figures_drawn":figures_drawn,"footprints_drawn":footprints_drawn,"ships_drawn":ships_drawn,"progress_drawn":progress_drawn,"effects":fx_drawn,"siege_markers":siege_drawn,"siege_figures":siege_figures}), (w, h, rgba)),
        )
    }
    /// Precipitación y grietas sobre la imagen ya dibujada. La nieve y las grietas usan los juegos de sprites reales (clases 29 y 34); la lluvia, el segmento de `0x5759B0`.
    /// Las grietas necesitan la huella del objeto del mapa en esa casilla: se toma de la ficha `.let` del edificio (`strips_a`, `strips_b`; si hay varias fichas de la misma clase, la primera).
    /// Los objetos no avanzan con el tiempo (`0x576300`/`0x575B00` siguen sin portar): es el reparto inicial.
    fn draw_effects(
        &self,
        archive: &Archive,
        scene: &mut p3_render::Scene,
        objects: &[(u8, i32, i32, Option<(u8, u8)>)],
        map: &TownMap,
        rgba: &mut [u8],
        w: usize,
        h: usize,
        clock_ms: u32,
        fx: &Effects,
    ) -> Result<Value, String> {
        if fx.weather.is_none() && fx.cracks.is_empty() && !fx.conductor {
            return Ok(json!(null));
        }
        let mut wx = scene::Weather::new(archive, w as u32, h as u32, fx.seed);
        wx.set_projection(map.width as u32);
        if let Some((snow, count)) = fx.weather {
            wx.set_capacity(count.max(1), false, snow);
            wx.precipitate([0, 0, w as i32, h as i32], count, u32::MAX, snow)
        }
        let precip_start = wx.snapshot();
        let mut sheets: std::collections::HashMap<u8, (u8, u8)> = Default::default();
        for entry in &archive.entries {
            let name = entry.name.to_ascii_lowercase();
            if !name.starts_with(b"iso/let/") && !name.starts_with(b"iso\\let\\") {
                continue;
            }
            if let Some(d) = archive.read(entry) {
                if let Ok(sh) = p3_assets::let_sheet::LetSheet::parse(d) {
                    sheets.entry(sh.field_2d).or_insert((sh.strips_a, sh.strips_b));
                }
            }
        }
        let mut foot = scene::Footprints::default();
        for &(g, x, y, _) in objects {
            if let Some(&(a, b)) = sheets.get(&g) {
                foot.place(map.width as u32 * y as u32 + x as u32, a, b)
            }
        }
        let mut placed = 0;
        let mut requested = fx.cracks.clone();
        if fx.conductor {
            requested.extend(self.conductor_cracks())
        }
        for &(x, y, p) in &requested {
            if foot.cells.contains_key(&(map.width as u32 * y + x)) {
                wx.crack(x, y, p, clock_ms, &mut foot);
                placed += 1
            }
        }
        let mut t = 0u32;
        while t < clock_ms {
            t = (t + 100).min(clock_ms);
            wx.advance(t)
        }
        let moved = wx.snapshot()["objects"] != precip_start["objects"];
        let (snow, rain, cracks) = wx.draw(scene, rgba, w, h);
        Ok(
            json!({"snow_drawn":snow,"rain_drawn":rain,"cracks_requested":requested.len(),"conductor":fx.conductor,"cracks_placed":placed,"cracks_drawn":cracks,"advanced_ms":clock_ms,"objects_moved":moved}),
        )
    }
    /// Reparto inicial de peatones sobre `.spawn` (clave 1) y las calles del plano, y dibujo en espera. La ocupación (`0x593580`) se aproxima con
    /// «casilla sin objeto en la capa 1»; la posición en el lienzo se calibró a ojo (desplazamiento 21 px).
    fn draw_figures(
        &self,
        archive: &Archive,
        scene: &mut p3_render::Scene,
        id: u8,
        map: &TownMap,
        rgba: &mut [u8],
        w: usize,
        h: usize,
        clock_ms: u32,
    ) -> (usize, usize) {
        let Some(entry) = archive.find(format!("iso/towns/{id}.spawn").as_bytes()) else {
            return (0, 0);
        };
        let Some(data) = archive.read(entry) else { return (0, 0) };
        let Ok(sections) = p3_assets::town::parse_spawn(data) else {
            return (0, 0);
        };
        let Some(points) = sections.iter().find(|s| s.key == 1).map(|s| s.points.clone()) else {
            return (0, 0);
        };
        let (town, site) = self.debug_town();
        let stride = town.i32(p3_core::town::off::PLAN_STRIDE).max(0) as u32;
        let geom = p3_render::geom::Geom {
            cols: map.width as u32,
            rows: map.height as u32,
            cell_w: 34,
            cell_h: 21,
            off_x: 17,
            off_y: 10,
        };
        let env = p3_render::walkers::PickEnv {
            cols: map.width as u32,
            rows: map.height as u32,
            stride,
            plan: &site.cells,
            points: &points,
            geom: &geom,
            view: (0, 0, 0, 0),
        };
        let figures = scene.load_figures();
        let mut rng = p3_render::anim::CrtRand(id as u32);
        let pops = [town.pop(3), town.pop(2), town.pop(1), town.pop(0)];
        let width = map.width as usize;
        let walkers = p3_render::walkers::populate_pedestrians(
            &env,
            pops,
            &|k| figures.get(&k).map_or(0, |v| v.len() as u32),
            true,
            &mut || rng.next(),
            &mut |x, y, flag| {
                let i = y as usize * width + x as usize;
                let (Some(c0), Some(c1)) = (map.layers[0].get(i), map.layers[1].get(i)) else {
                    return false;
                };
                let l1 = if c1.tile == 0 {
                    None
                } else {
                    Some(scene.tile_kind(c1.tile).unwrap_or(0xFFFF))
                };
                p3_render::walkers::spawn_cell_free(l1, scene.tile_kind(c0.tile).unwrap_or(0xFFFF), flag)
            },
        );
        // Multitud animada: caminar libre con la lógica de 0x55C820 desde el reparto inicial durante `clock_ms`.
        let cells = (map.width as usize) * (map.height as usize);
        let mut kind0 = Vec::with_capacity(cells);
        let mut kind1 = Vec::with_capacity(cells);
        for i in 0..cells {
            kind0.push(
                map.layers[0]
                    .get(i)
                    .and_then(|c| scene.tile_kind(c.tile))
                    .unwrap_or(0xFFFF),
            );
            kind1.push(map.layers[1].get(i).and_then(|c| {
                if c.tile == 0 {
                    None
                } else {
                    Some(scene.tile_kind(c.tile).unwrap_or(0xFFFF))
                }
            }));
        }
        let crowd_scene = p3_render::crowd::CrowdScene {
            cols: map.width as u32,
            rows: map.height as u32,
            kind0,
            kind1,
            plan: site.cells.clone(),
            stride,
            occupants: Default::default(),
        };
        let mut points8 = [(0xFFu8, 0xFFu8); 8];
        for (k, p) in points8.iter_mut().enumerate() {
            *p = (town.u8(0x7D0 + 2 * k), town.u8(0x7D1 + 2 * k))
        }
        let crowd_town = p3_render::crowd::CrowdTown {
            flags: town.u32(0x2C8),
            ref_time: town.u32(0x2D0),
            first_building: town.u16(p3_core::town::off::PUBLIC_BUILDING_HEAD),
            building_count: town.u16(p3_core::town::off::BUILDING_RECORD_COUNT),
            records: site.records.clone(),
            points: points8,
            clock: 0,
        };
        let mut crowd = p3_render::crowd::Crowd::new(crowd_scene, crowd_town, geom, &figures, id as u32);
        if let Some(e) = archive.find(format!("iso/towns/{id}.nodes").as_bytes()) {
            if let Some(d) = archive.read(e) {
                if let Ok(sections) = p3_assets::town::parse_nodes(d) {
                    crowd.set_nodes(&sections)
                }
            }
        }
        for w0 in &walkers {
            let ((cx, cy), _) = crowd.geom.pixel_to_cell(w0.x >> 16, w0.y >> 16, &crowd.mask);
            crowd.add(w0.kind, w0.variant as usize, (cx as i32, cy), w0.heading)
        }
        // Carros del cochero: los transportes terrestres de la partida (`0x004D8F00`), salvo que se den entradas explícitas.
        if !self.cart_entries.is_empty() {
            crowd.spawn_carts(&figures, self.cart_entries.clone());
        } else {
            let entries: Vec<_> = self.land_transport_carts(self.city as u8).into_iter().map(|(_, e)| e).collect();
            if !entries.is_empty() {
                let order: Vec<u8> = self.sim.host.expedition_town_ids.clone();
                let mut symbols = Vec::new();
                if let Some(ini) = archive.find(b"scripts/StadtDaten.ini").and_then(|e| archive.read(e)) {
                    for t in crate::world::parse_towns(&String::from_utf8_lossy(ini)) {
                        if symbols.len() <= t.id {
                            symbols.resize(t.id + 1, (0, 0));
                        }
                        symbols[t.id] = t.symbol;
                    }
                }
                let size = (crowd.geom.width_px() as u32, crowd.geom.height_px() as u32);
                crowd.spawn_carts_in(&figures, entries, Some(p3_render::cart_field::CartTown { town: self.city as u8, order, symbols, size }));
            }
        }
        // Huellas (`0x55D0B0`/`0x55B030`, J07) con el grado gráfico de la ciudad: efectos de la clase 18 que se desvanecen.
        crowd.footprint_view = Some(p3_render::footprints::FootprintView {
            grade: self.town_grade.unwrap_or(u32::from(town.u8(0x9BC))),
            detail: 1,
            // El cliente dibuja el plano entero: la ventana es todo el lienzo.
            view: (0, 0, w as u32, h as u32),
        });
        let effect_sets = figures.get(&18).cloned().unwrap_or_default();
        let effect_infos: Vec<p3_render::mover::SetInfo> = effect_sets.iter().map(p3_render::mover::SetInfo::from_lcr).collect();
        let mut prints = town_fx::TownFootprints::default();
        let mut t = 0u32;
        while t < clock_ms {
            crowd.tick(33);
            t += 33;
            prints.step(&effect_infos, t);
            let new = std::mem::take(&mut crowd.footprints);
            prints.add(&new, &effect_sets, &effect_infos, t);
        }
        // En el suelo, debajo de las figuras.
        let mut printed = 0;
        for e in prints.effects() {
            if let Some(set) = effect_sets.get(e.set) {
                if scene.draw_lcr_frame_faded(rgba, w, h, set, e.mem.r8(0x14), e.frame(), e.x(), e.y() + 21, e.alpha()) {
                    printed += 1
                }
            }
        }
        let mut n = scene.draw_walkers(rgba, w, h, &crowd.walkers(), &figures, (0, 21), true);
        if let Some(field) = &crowd.carts {
            if let Some(sets) = figures.get(&31) {
                for (variant, heading, frame, x, y) in field.frames() {
                    if let Some(set) = sets.get(variant) {
                        if scene.draw_lcr_frame(rgba, w, h, set, heading, frame, x, y + 21) {
                            n += 1
                        }
                    }
                }
            }
        }
        (n, printed)
    }
    /// Decodifica el ambiente optativo con el puerto IMA y lo escribe como PCM.
    /// Ejecuta el constructor de escena `0x589360` sobre el contenedor de recursos (sin ventanas ni dibujo) y resume las cargas de `.let`/`.lcr`.
    pub fn scene_init(&self) -> Result<Value, String> {
        let archive = Archive::parse(&self.archive).map_err(|e| format!("CPR: {e}"))?;
        Ok(scene::run_scene_init(&archive, 1024, 768))
    }
    /// Precipitación de la escena (lluvia o nieve) sobre los conjuntos del contenedor: `capacity` objetos, `count` apariciones en `rect` y semilla del azar del CRT.
    pub fn weather(&self, snow: bool, capacity: i32, count: i32, rect: [i32; 4], seed: u32) -> Result<Value, String> {
        let archive = Archive::parse(&self.archive).map_err(|e| format!("CPR: {e}"))?;
        let mut w = scene::Weather::new(&archive, 1024, 768, seed);
        w.set_capacity(capacity, false, snow);
        w.precipitate(rect, count, u32::MAX, snow);
        Ok(w.snapshot())
    }
    pub fn ambient(&self, path: &Path) -> Result<(), String> {
        let archive = Archive::parse(&self.archive).map_err(|e| format!("CPR: {e}"))?;
        let entry = archive
            .find(b"musik/Add/Marktlong1.wav")
            .ok_or("el CPR no contiene el ambiente de mercado")?;
        let data = archive.read(entry).ok_or("ambiente comprimido o truncado")?;
        let pcm = p3_audio::native::ima::decode_miles_defined(data, 44_100 * 600)
            .map_err(|e| format!("decodificación del ambiente: {e:?}"))?;
        let size = u32::try_from(pcm.samples.len().checked_mul(2).ok_or("audio demasiado grande")?)
            .map_err(|_| "audio demasiado grande")?;
        let block = pcm.channels.checked_mul(2).ok_or("canales inválidos")?;
        let rate = pcm
            .sample_rate
            .checked_mul(u32::from(block))
            .ok_or("frecuencia inválida")?;
        let mut wav = Vec::with_capacity(size as usize + 44);
        wav.extend_from_slice(b"RIFF");
        wav.extend_from_slice(&(size + 36).to_le_bytes());
        wav.extend_from_slice(b"WAVEfmt ");
        wav.extend_from_slice(&16u32.to_le_bytes());
        wav.extend_from_slice(&1u16.to_le_bytes());
        wav.extend_from_slice(&pcm.channels.to_le_bytes());
        wav.extend_from_slice(&pcm.sample_rate.to_le_bytes());
        wav.extend_from_slice(&rate.to_le_bytes());
        wav.extend_from_slice(&block.to_le_bytes());
        wav.extend_from_slice(&16u16.to_le_bytes());
        wav.extend_from_slice(b"data");
        wav.extend_from_slice(&size.to_le_bytes());
        for sample in pcm.samples {
            wav.extend_from_slice(&sample.to_le_bytes())
        }
        std::fs::write(path, wav).map_err(|e| format!("ambiente: {e}"))
    }
    pub fn trade(&mut self, side: TradeSide, ware: usize, quantity: i32) -> Result<Value, String> {
        if self.paused_battle.is_some() || self.sim.host.clock_before_battle || self.sim.host.encounter_battle.is_some() || self.sim.host.town_attack_active.is_some()
        {
            return Err("batalla activa en pausa".into());
        }
        if ware >= 24 || quantity <= 0 || quantity > 100_000 {
            return Err("mercancía o cantidad fuera de rango".into());
        }
        if self.sim.economy.world.towns[self.city].stock(ware) < 0
            || self.sim.persisted[self.office].stock.stock[ware] < 0
            || self.sim.economy.merchants[self.owner].i32(0) < 0
        {
            return Err("existencias o saldo negativos; operación no disponible".into());
        }
        let old_gold = self.sim.economy.merchants[self.owner].i32(0);
        let mut amount = quantity;
        office_trade_with_reputation(
            &mut self.sim.economy.world.towns,
            &mut self.sim.persisted[self.office].stock,
            &mut self.sim.economy.merchants[self.owner],
            side,
            ware,
            &mut amount,
            self.sim.commercial_sell_factor,
        );
        self.sim.economy.offices[self.office] = self.sim.persisted[self.office].stock.clone();
        self.sim.rep_merchants[self.owner] = self.sim.economy.merchants[self.owner].clone();
        self.sim.economy.world.merchants[self.owner].gold = self.sim.economy.merchants[self.owner].i32(0);
        self.sim.economy.world.offices[self.office]
            .weapons
            .copy_from_slice(&self.sim.economy.offices[self.office].stock[20..24]);
        Ok(
            json!({"quantity":amount,"gold_change":i64::from(self.sim.economy.merchants[self.owner].i32(0))-i64::from(old_gold),"state":self.state()}),
        )
    }
    /// Comercio directo de un barco atracado con el mercado de su ciudad.
    pub fn trade_ship(&mut self, id: usize, side: TradeSide, ware: usize, quantity: i32) -> Result<Value, String> {
        if self.paused_battle.is_some() || self.sim.host.clock_before_battle || self.sim.host.encounter_battle.is_some() || self.sim.host.town_attack_active.is_some()
        {
            return Err("batalla activa en pausa".into());
        }
        if ware >= 20 || quantity <= 0 || quantity > 100_000 {
            return Err("mercancía o cantidad fuera de rango".into());
        }
        let lot = if p3_core::economy::prices::SMALL_LOT_WARES[ware] {
            200
        } else {
            2000
        };
        if quantity % lot != 0 {
            return Err(format!("la cantidad debe ser múltiplo de {lot}"));
        }
        let image = self
            .sim
            .host
            .expedition_ship_images
            .get(id)
            .and_then(Option::as_ref)
            .ok_or("barco inexistente")?;
        if image.len() < 0x180 {
            return Err("imagen naval truncada".into());
        }
        let state = u16::from_le_bytes([image[0x134], image[0x135]]);
        let convoy = u16::from_le_bytes([image[8], image[9]]) as usize;
        let cargo = self.sim.economy.cargo_ships.get(id).ok_or("carga inexistente")?;
        if usize::from(cargo.owner) != self.owner || usize::from(cargo.town) != self.city || state >= 2 {
            return Err("el barco debe pertenecer al comerciante y estar atracado en esta ciudad".into());
        }
        if convoy < self.sim.host.debt_convoy_flags.len()
            && self
                .sim
                .host
                .expedition_convoy_images
                .get(convoy)
                .is_none_or(|c| c.len() < 0x1c)
            || self.sim.host.navigation.as_ref().is_some_and(|n| {
                n.snapshot.ships.len() < (id + 1) * 0x180
                    || convoy < self.sim.host.debt_convoy_flags.len() && n.snapshot.convoys.len() < (convoy + 1) * 0x3c
            })
        {
            return Err("vistas navales incompletas".into());
        }
        let same_cargo = |v: &[u8]| {
            (0..24).all(|w| {
                v[0x54 + 4 * w..0x58 + 4 * w] == cargo.stock[w].to_le_bytes()
                    && v[0xb4 + 4 * w..0xb8 + 4 * w] == cargo.average_price[w].to_bits().to_le_bytes()
            })
        };
        if !same_cargo(image)
            || self
                .sim
                .host
                .navigation
                .as_ref()
                .is_some_and(|n| !same_cargo(&n.snapshot.ships[id * 0x180..(id + 1) * 0x180]))
        {
            return Err("vistas de carga comercial desincronizadas".into());
        }
        if let Some(&flags) = self.sim.host.debt_convoy_flags.get(convoy) {
            let image = &self.sim.host.expedition_convoy_images[convoy];
            let stored = u32::from_le_bytes(image[0x18..0x1c].try_into().expect("convoy comprobado"));
            if stored != flags
                || self.sim.host.navigation.as_ref().is_some_and(|n| {
                    u32::from_le_bytes(
                        n.snapshot.convoys[convoy * 0x3c + 0x18..convoy * 0x3c + 0x1c]
                            .try_into()
                            .expect("convoy comprobado"),
                    ) != flags
                })
            {
                return Err("vistas de convoy comercial desincronizadas".into());
            }
        }
        if cargo.stock[ware] < 0
            || self.sim.economy.world.towns[self.city].stock(ware) < 0
            || self.sim.economy.merchants[self.owner].i32(0) < 0
        {
            return Err("existencias o saldo negativos".into());
        }
        let old_gold = self.sim.economy.merchants[self.owner].i32(0);
        let before = cargo.clone();
        let mut amount = quantity;
        let mut flags = self.sim.host.debt_convoy_flags.get(convoy).copied();
        self.sim.economy.trade_ship(
            id,
            self.city,
            side,
            ware,
            &mut amount,
            self.sim.commercial_sell_factor,
            flags.as_mut(),
        );
        if self.sim.economy.cargo_ships[id] == before {
            amount = 0
        } else if !self.changed_ships.contains(&id) {
            self.changed_ships.push(id)
        }
        // El reloj y las tareas consultan imágenes navales además de la vista
        // económica; publicar la misma carga y banderas en todas las vistas.
        if amount > 0 {
            let cargo = &self.sim.economy.cargo_ships[id];
            let write_cargo = |image: &mut [u8]| {
                image[0x3c..0x3e].copy_from_slice(&cargo.dirty.to_le_bytes());
                for w in 0..24 {
                    image[0x54 + 4 * w..0x58 + 4 * w].copy_from_slice(&cargo.stock[w].to_le_bytes());
                    image[0xb4 + 4 * w..0xb8 + 4 * w].copy_from_slice(&cargo.average_price[w].to_bits().to_le_bytes());
                }
                for (at, value) in [
                    (0x114, cargo.cargo_value),
                    (0x118, cargo.cargo_weight),
                    (0x11c, cargo.equipment_weight),
                    (0x120, cargo.gun_power),
                ] {
                    image[at..at + 4].copy_from_slice(&value.to_le_bytes());
                }
            };
            write_cargo(
                self.sim.host.expedition_ship_images[id]
                    .as_mut()
                    .expect("barco comprobado"),
            );
            if let Some(nav) = self.sim.host.navigation.as_mut() {
                write_cargo(&mut nav.snapshot.ships[id * 0x180..(id + 1) * 0x180]);
            }
        }
        if let Some(flags) = flags {
            self.sim.host.debt_convoy_flags[convoy] = flags;
            if let Some(image) = self.sim.host.expedition_convoy_images.get_mut(convoy) {
                image[0x18..0x1c].copy_from_slice(&flags.to_le_bytes())
            }
            if let Some(nav) = self.sim.host.navigation.as_mut() {
                nav.snapshot.convoys[convoy * 0x3c + 0x18..convoy * 0x3c + 0x1c].copy_from_slice(&flags.to_le_bytes())
            }
            if !self.sim.host.debt_changed_convoys.contains(&convoy) {
                self.sim.host.debt_changed_convoys.push(convoy)
            }
        }
        self.sim.rep_merchants[self.owner] = self.sim.economy.merchants[self.owner].clone();
        Ok(
            json!({"quantity":amount,"gold_change":i64::from(self.sim.economy.merchants[self.owner].i32(0))-i64::from(old_gold),"state":self.state()}),
        )
    }
    pub fn advance(&mut self) -> Value {
        self.advance_clock(None)
    }
    /// Como [`Self::advance`], pero `ticks` del reloj (256 por día; la carta náutica avanza de poco en poco).
    pub fn advance_ticks(&mut self, ticks: u32) -> Value {
        self.advance_clock(Some(ticks))
    }
    fn advance_clock(&mut self, ticks: Option<u32>) -> Value {
        if self.sim.host.game_end_stopped {
            return json!({"state":self.state(),"errors":[],"inactive":{}});
        }
        if self.paused_battle.is_some() || self.sim.host.clock_before_battle || self.sim.host.encounter_battle.is_some() || self.sim.host.town_attack_active.is_some()
        {
            return json!({"state":self.state(),"errors":["batalla activa en pausa: simulación pendiente"],"inactive":{}});
        }
        if let Some(e) = &self.sim.host.navigation_error {
            return json!({"state":self.state(),"errors":[format!("navegación detenida: {e}")],"inactive":{}});
        }
        let chase = self.sim.host.navigation.as_ref().is_some_and(|nav| {
            let count = u16::from_le_bytes([nav.snapshot.manager[0xf4], nav.snapshot.manager[0xf5]]);
            std::iter::once(nav.ship)
                .chain(nav.additional.iter().map(|v| v.ship))
                .any(|id| {
                    let at = usize::from(id) * 0x180;
                    nav.snapshot
                        .ships
                        .get(at + 0x3a..at + 0x3c)
                        .is_some_and(|v| u16::from_le_bytes([v[0], v[1]]) < count)
                })
        });
        if chase && self.sim.host.navigation_planner.is_none() {
            let count = self.sim.economy.world.towns.len();
            if count > 1 {
                let other = (self.city + 1) % count;
                let _ = self.route_between(self.city, other);
            }
        }
        // Tormentas e icebergs (tarea 0x0D) consultan el mapa de navegación.
        if self.sim.host.navigation_planner.is_none() && self.sim.host.navigation.is_some() {
            if let Err(e) = self.ensure_planner() {
                return json!({"state":self.state(),"errors":[format!("navegación: {e}")],"inactive":{}});
            }
        }
        let report = match ticks {
            None => self.sim.advance_days(1),
            Some(t) => self.sim.advance_ticks(t),
        };
        json!({"state":self.state(),"errors":report.errors,"inactive":report.inactive})
    }
    /// Reconstruye la flota completa y activa su IA desde el estado guardado
    /// de esta sesión. Si falla, conserva la sesión anterior.
    pub fn enable_full_fleet(&mut self) -> Result<Value, String> {
        if self.paused_battle.is_some() || self.sim.host.clock_before_battle || self.sim.host.encounter_battle.is_some() || self.sim.host.town_attack_active.is_some()
        {
            return Err("batalla activa en pausa".into());
        }
        let saved = self.save()?;
        let model = SaveModel::parse(&saved).map_err(|e| format!("partida: {e:?}"))?;
        let mut sim = Sim::from_save_bytes(&saved).map_err(|e| e.to_string())?;
        sim.configure_hanse_fleet_archives(&[&*self.archive], "scripts/Namen_eng.ini", false, self.owner as u16)?;
        let count = sim.resume_saved_fleet(&model, self.owner as u32)?;
        Self::transfer_host_context(&mut self.sim, &mut sim);
        self.model = model;
        self.sim = sim;
        self.changed_ships.clear();
        self.planner = None;
        Ok(json!({"ships_resumed":count,"state":self.state()}))
    }
    /// Avanza hasta el encuentro y devuelve sus efectos sin abrir la interfaz.
    /// El texto debe estar localizado por el anfitrión. Al interrumpirse queda
    /// pendiente la fase de batalla; no se puede guardar ni avanzar todavía.
    /// Captura la base privada del tick interrumpido y reanuda el ataque.
    /// La escena y sus efectos deben resolverse antes de guardar la sesión.
    pub fn resume_pirate_mission(&mut self) -> Result<p3_core::tick::AdvanceEnd, String> {
        if self.paused_battle.is_some() || self.battle_clock.is_some() || self.sim.host.town_attack_active.is_some() {
            return Err("batalla naval pendiente en la sesión".into());
        }
        if !self.sim.has_pirate_boundary_baseline() {
            self.sim.stage_pirate_boundary_baseline(&self.model)?;
        }
        self.sim.resume_pirate_boundary()
    }

    /// Consume 0x94 con las consultas de interfaz aportadas por el anfitrión.
    pub fn dispatch_pirate_frame(
        &mut self,
        ui_active: bool,
        convoy_ui: bool,
        gui_present: bool,
        time_mode: u32,
    ) -> Result<(), String> {
        self.sim
            .dispatch_town_attack_frame_operation(ui_active, convoy_ui, gui_present, time_mode)
    }

    /// Publica la conciliación solo tras atender el frame, B4 y sus efectos.
    pub fn finish_pirate_mission(&mut self) -> Result<(), String> {
        let model = self.sim.stage_completed_pirate_attack()?;
        Sim::from_save_bytes(&model.write()).map_err(|e| format!("verificación de recarga: {e}"))?;
        self.sim.confirm_completed_pirate_attack(&model)?;
        self.model = model;
        Ok(())
    }

    pub fn advance_until_encounter(&mut self, target: u32, dialog_text: Vec<u8>) -> Result<Value, String> {
        self.advance_until_encounter_impl(target, dialog_text, true)
    }
    /// Como `advance_until_encounter` pero sin el campo `state` (resumen + SHA): para el bucle sin pantalla, que no lo lee.
    pub fn advance_until_encounter_lean(&mut self, target: u32, dialog_text: Vec<u8>) -> Result<Value, String> {
        self.advance_until_encounter_impl(target, dialog_text, false)
    }
    fn advance_until_encounter_impl(&mut self, target: u32, dialog_text: Vec<u8>, with_state: bool) -> Result<Value, String> {
        if self.paused_battle.is_some() || self.sim.host.clock_before_battle || self.sim.host.encounter_battle.is_some() || self.sim.host.town_attack_active.is_some()
        {
            return Err("batalla activa en pausa".into());
        }
        let needs_planner = self.sim.host.navigation.as_ref().is_some_and(|nav| {
            let m = &nav.snapshot.manager;
            if m.len() < 0xf6 {
                return false;
            }
            let count = u16::from_le_bytes([m[0xf4], m[0xf5]]);
            let captains = u16::from_le_bytes([m[0xf2], m[0xf3]]);
            captains != 0
                && (0..16).any(|i| {
                    let p = 0x20 + i * 12;
                    u16::from_le_bytes([m[p], m[p + 1]]) < count
                })
                && nav.snapshot.routes.iter().any(|r| {
                    matches!(
                        r,
                        Some(p3_save::ships::SavedRoute::Owned(_) | p3_save::ships::SavedRoute::Shared { .. })
                    )
                })
        });
        if needs_planner && self.sim.host.navigation_planner.is_none() && self.sim.economy.world.towns.len() > 1 {
            let other = (self.city + 1) % self.sim.economy.world.towns.len();
            let _ = self.route_between(self.city, other);
        }
        let end = self.sim.advance_until_encounter(
            target,
            p3_sim::EncounterConfig {
                slot: 0,
                battle_address: 0x59000000,
                scene_busy: false,
                dialog_text,
            },
        )?;
        let Some(battle) = self.sim.host.encounter_battle.as_ref() else {
            let mut out = json!({"encounter":null,"reached":end==p3_core::tick::AdvanceEnd::Reached});
            if with_state {
                out["state"] = self.state();
            }
            return Ok(out);
        };
        let notifications = battle
            .notifications
            .iter()
            .map(|event| match event {
                p3_core::ships::battle::BattleStartEvent::Sync => json!({"kind":"sync"}),
                p3_core::ships::battle::BattleStartEvent::Notice {
                    merchant,
                    x,
                    y,
                    kind,
                    ship,
                } => json!({"kind":"notice","merchant":merchant,"x":x,"y":y,"type":kind,"ship":ship}),
            })
            .collect::<Vec<_>>();
        let dialog = battle
            .dialog
            .as_ref()
            .map(|d| json!({"player":d.player,"opponent":d.opponent,"slot":d.slot}));
        let mut out = json!({"encounter":{"dialog":dialog,"notifications":notifications,"operations":battle.operations,"open_for_player_requested":battle.open_for_player_requested},"reached":false});
        if with_state {
            out["state"] = self.state();
        }
        Ok(out)
    }
    /// Completa el tick que creó el encuentro con un gestor sin listas
    /// auxiliares. La batalla y sus efectos de interfaz siguen pendientes.
    pub fn complete_encounter_tick(&mut self) -> Result<Value, String> {
        let frame = self
            .model
            .current_basic_sea_battles()
            .map_err(|e| format!("gestor de batallas: {e:?}"))?;
        if !frame.objects.is_empty() || (!frame.pairs.is_empty() && self.sim.idle.battle_manager.is_none()) {
            return Err("gestor de batallas ocupado o con listas auxiliares no reconstruidas".into());
        }
        let mut manager = vec![0u8; 0xc40];
        self.sim.complete_encounter_tick(&mut manager)?;
        Ok(json!({"state":self.state(),"battle_pending":true}))
    }
    /// Despacha la operación 0x96 encolada; el cliente recibe los efectos
    /// externos que aún debe aplicar al reloj y la interfaz.
    pub fn dispatch_encounter_policy(&mut self) -> Result<Value, String> {
        use p3_core::ships::battle::BattlePolicyEffect;
        let effects = self
            .sim
            .dispatch_encounter_policy(&mut self.battle_selected, self.owner as u32)?;
        let effects = effects
            .into_iter()
            .map(|e| match e {
                BattlePolicyEffect::SlowTime => "slow_time",
                BattlePolicyEffect::NormalTime => "normal_time",
                BattlePolicyEffect::OpenForPlayer => "open_for_player",
            })
            .collect::<Vec<_>>();
        Ok(json!({"state":self.state(),"selected":self.battle_selected,"effects":effects}))
    }
    /// Conserva las tablas y barcos recién creados por el encuentro: no pasa
    /// por el lector de partidas, que reconstruiría su estado transitorio.
    pub fn activate_encounter_battle(&mut self) -> Result<Value, String> {
        if self.paused_battle.is_some() || self.sim.host.clock_before_battle || self.sim.host.town_attack_active.is_some() {
            return Err("batalla activa o tick incompleto".into());
        }
        let start = self
            .sim
            .host
            .encounter_battle
            .as_ref()
            .ok_or("no hay encuentro pendiente")?;
        if !start.operations.is_empty()
            || start.dialog.is_some()
            || start.open_for_player_requested
            || start.object.get(0xac8) != Some(&1)
        {
            return Err("hay efectos de encuentro pendientes".into());
        }
        start.initial_save_input().map_err(str::to_string)?;
        let state = p3_save::tail::SavedInitialCommercialBattle {
            object: start.object.clone(),
            next: start.next.clone(),
            previous: start.previous.clone(),
            local_ships: start.local_ships.clone(),
            bitmap: vec![],
            projectiles: vec![],
        };
        self.paused_battle = Some(p3_save::tail::RebuiltInitialCommercialBattle {
            state,
            collisions: start.collisions.clone(),
            spatial_0: start.spatial_0.clone(),
            spatial_1: start.spatial_1.clone(),
        });
        Ok(json!({"state":self.state(),"battle_ready":true}))
    }
    /// Cambia la política de una batalla reconstruida y devuelve los efectos
    /// externos que el cliente debe ejecutar. La batalla permanece pausada.
    pub fn set_paused_battle_policy(&mut self, merchant: u16, policy: u8) -> Result<Value, String> {
        use p3_core::ships::battle::{set_battle_participant_policy, BattlePolicyEffect};
        if policy > 1 {
            return Err("política de batalla inválida".into());
        }
        let battle = self.paused_battle.as_mut().ok_or("no hay batalla reconstruida")?;
        let o = &battle.state.object;
        if !(0..usize::from(o[0x558])).any(|i| u16::from_le_bytes([o[0x35a + 2 * i], o[0x35b + 2 * i]]) == merchant) {
            return Err("participante de batalla inexistente".into());
        }
        let effects = set_battle_participant_policy(
            &mut battle.state.object,
            &mut battle.state.local_ships,
            &battle.state.next,
            &mut self.battle_selected,
            self.owner as u32,
            merchant,
            policy,
        )
        .map_err(str::to_string)?;
        let effects = effects
            .into_iter()
            .map(|e| match e {
                BattlePolicyEffect::SlowTime => "slow_time",
                BattlePolicyEffect::NormalTime => "normal_time",
                BattlePolicyEffect::OpenForPlayer => "open_for_player",
            })
            .collect::<Vec<_>>();
        Ok(json!({"state":self.state(),"selected":self.battle_selected,"effects":effects}))
    }
    /// Parámetros del reloj de batalla: milisegundos por tick mundial y divisor del modo 1 de la simulación
    /// (`OPS+0x8D4/+0x8D8`, cargados del guardado como 0x00549F40 y cambiados por la velocidad elegida) y si la
    /// partida es por turnos. Los consume `p3_core::pump::BattleTimePump`.
    pub fn battle_time_settings(&self) -> (u32, u32, u32) {
        let t = &self.sim.host.time_mode.pump;
        let (ms, div) = (t.ms_per_tick, t.mode1_divisor);
        let hotseat = u32::from(self.sim.host.expedition_resources.as_ref().is_some_and(|r| r.hotseat));
        (ms, div, hotseat)
    }
    /// Velocidad elegida en la ventana de opciones (niveles 0 a 5, 0x0045DF6D): la simulación aplica el 0xC8 y el
    /// reloj de batalla, la vista de asedio y el guardado usan el intervalo nuevo.
    pub fn set_game_speed(&mut self, level: usize) -> Result<Value, String> {
        self.sim.set_game_speed(level, 0)?;
        let (ms, div, hotseat) = self.battle_time_settings();
        Ok(json!({"level": level, "ms_per_tick": ms, "mode1_divisor": div, "hotseat": hotseat}))
    }
    /// Hay una batalla activada y automática (`+0xAC8 == 1`, ningún
    /// participante en control directo) que puede avanzar junto al reloj.
    pub fn auto_battle_live(&self) -> bool {
        self.paused_battle.as_ref().is_some_and(|b| {
            let o = &b.state.object;
            o.get(0xac8) == Some(&1)
                && o.len() > 0x5d + usize::from(o[0x558])
                && o[0x5d..0x5d + usize::from(o[0x558])].iter().all(|&p| p != 0)
        })
    }
    /// Avanza un tick mundial mientras una batalla automática sigue viva, como
    /// el original: `0x00612D20` reanuda el reloj cuando todas las batallas son
    /// automáticas y estas reciben sus pasos por la operación 0x94. La batalla
    /// no se toca aquí; el llamador la avanza con `advance_paused_battle`.
    /// Si el tick crea otro encuentro, se devuelve igual que `advance_until_encounter`
    /// y la batalla viva queda separada del nuevo encuentro (`concurrent_start`).
    pub fn advance_tick_beside_auto_battle(&mut self) -> Result<Value, String> {
        if !self.auto_battle_live() {
            return Err("no hay batalla automática activa".into());
        }
        if self.battle_clock.is_some() || self.sim.host.clock_before_battle || self.sim.host.town_attack_active.is_some() {
            return Err("reloj de batalla o tick incompleto pendientes".into());
        }
        if self.concurrent_start.is_none() {
            let start = self.sim.host.encounter_battle.take().ok_or("encuentro activado ausente")?;
            self.concurrent_start = Some(start);
        } else if self.sim.host.encounter_battle.is_some() {
            return Err("hay otro encuentro pendiente".into());
        }
        let target = self.sim.idle.clock.tick.checked_add(1).ok_or("reloj fuera de rango")?;
        let battle = self.paused_battle.take();
        let result = self.advance_until_encounter_lean(target, Vec::new());
        self.paused_battle = battle;
        result
    }
    /// Separa un encuentro recién creado mientras se termina la batalla viva
    /// sin reloj (dos batallas simultáneas aún no se admiten) y lo devuelve.
    pub fn take_pending_encounter(&mut self) -> Option<p3_core::ships::battle::CommercialBattleStart> {
        self.sim.host.encounter_battle.take()
    }
    /// Restituye el encuentro separado por `take_pending_encounter`.
    pub fn restore_pending_encounter(&mut self, start: p3_core::ships::battle::CommercialBattleStart) -> Result<(), String> {
        if self.sim.host.encounter_battle.is_some() || self.paused_battle.is_some() {
            return Err("no se puede restituir el encuentro con otra batalla activa".into());
        }
        self.sim.host.encounter_battle = Some(start);
        Ok(())
    }
    /// Avanza el combate reconstruido sin interfaz; si alcanza una rama aún
    /// no conectada, conserva la batalla y la flota sin cambios.
    pub fn advance_paused_battle(&mut self, ticks: u16) -> Result<Value, String> {
        if self.battle_clock.is_some_and(|c| c.pending.is_some()) {
            return Err("hay un avance de batalla encolado".into());
        }
        let mut out = self.step_battle(ticks)?;
        out["state"] = self.state();
        Ok(out)
    }
    /// Igual que `advance_paused_battle` pero sin el campo `state` (resumen + SHA del estado completo):
    /// solo lo piden quienes lo consumen; los pasos internos de batalla no.
    pub fn advance_paused_battle_lean(&mut self, ticks: u16) -> Result<Value, String> {
        if self.battle_clock.is_some_and(|c| c.pending.is_some()) {
            return Err("hay un avance de batalla encolado".into());
        }
        self.step_battle(ticks)
    }
    fn step_battle(&mut self, ticks: u16) -> Result<Value, String> {
        let mut battle = self
            .paused_battle
            .as_ref()
            .ok_or("no hay batalla reconstruida")?
            .clone();
        let nav = self.sim.host.navigation.as_ref().ok_or("flota no reconstruida")?;
        let mut before = nav.snapshot.clone();
        let mut snapshot = before.clone();
        let original_ship_count = before.ships.len() / 0x180;
        let words = self.sim.rep_merchants.iter().map(|m| m.u16(8)).collect::<Vec<_>>();
        let towns = self.sim.rep_merchants.iter().map(|m| m.bytes[0x19]).collect::<Vec<_>>();
        let reports_enabled = self
            .sim
            .rep_merchants
            .iter()
            .map(|m| m.bytes[0x39] != 5)
            .collect::<Vec<_>>();
        if self.sim.economy.world.merchants.len() != self.sim.rep_merchants.len() {
            return Err("cuentas de comerciantes desincronizadas".into());
        }
        let mut merchant_data = Vec::with_capacity(self.sim.rep_merchants.iter().map(|m| m.bytes.len()).sum());
        for m in &self.sim.rep_merchants {
            merchant_data.extend_from_slice(&m.bytes);
        }
        for (i, m) in self.sim.economy.world.merchants.iter().enumerate() {
            merchant_data[i * 0x650..i * 0x650 + 4].copy_from_slice(&m.gold.to_le_bytes());
        }
        // Una orden nueva puede tener descriptor vivo y caché de puntos antes
        // de reflejarse en `routes`; recuperar ambas piezas antes del abordaje.
        let mut routes = before.routes.clone();
        for (i, route) in routes.iter_mut().enumerate() {
            let at = i * 0x180;
            if at + 0x3a > before.ships.len() {
                return Err("rutas navales truncadas".into());
            }
            if matches!(route, Some(p3_save::ships::SavedRoute::None)) && before.ships[at + 0x24..at + 0x28] != [0; 4] {
                let id = u16::try_from(i).map_err(|_| "identificador naval excesivo")?;
                if let Some(points) = nav.route_for(id) {
                    *route = Some(if before.ships[at + 0x36] != 0 {
                        p3_save::ships::SavedRoute::Owned(points.to_vec())
                    } else {
                        p3_save::ships::SavedRoute::Shared {
                            origin: before.ships[at + 0x37],
                            destination: before.ships[at + 0x38],
                        }
                    })
                }
            }
        }
        let hotseat = self.sim.host.expedition_resources.as_ref().is_some_and(|r| r.hotseat);
        // Con la política sin pantalla no hay jugador que decida en la batalla (abordaje incluido): sus barcos
        // siguen la ruta automática de la IA.
        let player = if self.sim.host.headless_human_battles { u32::MAX } else { self.owner as u32 };
        let env = p3_sim::battle_runtime::Environment {
            player,
            hotseat,
            merchants: &words,
            merchant_data: &merchant_data,
            routes: &routes,
            alderman_town: self.sim.economy.world.alderman_town,
            town_count: self.sim.economy.world.towns.len() as u32,
            merchant_towns: &towns,
            report_enabled: &reports_enabled,
            report_texts: self.battle_texts.as_ref(),
            fleets: &before.convoys,
            captains: &before.captains,
            manager: &before.manager,
            captain_free_head: u16::from_le_bytes([before.manager[0xe0], before.manager[0xe1]]),
            difficulty: self.sim.host.battle_difficulty,
        };
        let mut capture_state = self.sim.prepare_battle_capture()?;
        let mut rng = self.sim.host.investigation_rng;
        let outcome = p3_sim::battle_runtime::advance_with_capture_services(
            &mut battle,
            &mut snapshot.ships,
            env,
            u32::from(ticks),
            &mut rng,
            &mut capture_state,
        )?;
        let destroyed = outcome.destroyed;
        if let Some(manager) = &outcome.manager_data {
            snapshot.manager.clone_from(manager)
        }
        let new_ship_count = snapshot.ships.len() / 0x180;
        if new_ship_count < original_ship_count || snapshot.ships.len() % 0x180 != 0 {
            return Err("crecimiento de captura inválido".into());
        }
        for i in original_ship_count..new_ship_count {
            let mut empty = vec![0; 0x180];
            empty[0] = 255;
            empty[4..6].fill(255);
            empty[6..8].copy_from_slice(&((i + 1) as u16).to_le_bytes());
            empty[8..10].fill(255);
            empty[0x134..0x136].copy_from_slice(&255u16.to_le_bytes());
            before.ships.extend_from_slice(&empty);
            before.routes.push(None);
        }
        snapshot.routes.resize(new_ship_count, None);
        for (i, image) in snapshot.ships.chunks_exact(0x180).enumerate() {
            if u16::from_le_bytes([before.ships[i * 0x180 + 0x134], before.ships[i * 0x180 + 0x135]]) >= 0x17
                && u16::from_le_bytes([image[0x134], image[0x135]]) < 0x17
            {
                snapshot.routes[i] = Some(p3_save::ships::SavedRoute::None)
            }
        }
        let captured_heads: [Option<u16>; 5] = std::array::from_fn(|i| {
            capture_state.bands[i]
                .as_ref()
                .map(|b| u16::from_le_bytes([b.header[4], b.header[5]]))
        });
        if let Some(pool) = &outcome.captain_pool {
            if snapshot.captains.len() != pool.entries.len() {
                return Err("pool de capitanes truncado".into());
            }
            if let Some(factory) = &self.sim.host.expedition_factory {
                if factory.captains.iter().flatten().copied().collect::<Vec<_>>() != before.captains
                    || factory.free_captain != u16::from_le_bytes([before.manager[0xe0], before.manager[0xe1]])
                {
                    return Err("pool de capitanes desincronizado".into());
                }
            }
            if let Some(pirates) = &self.sim.host.hanse_pirates {
                if pirates.ship_captains.len() != original_ship_count || pirates.next.len() != original_ship_count {
                    return Err("bandas piratas desincronizadas".into());
                }
            }
            snapshot.captains.clone_from(&pool.entries);
            snapshot.manager[0xe0..0xe2].copy_from_slice(&pool.free_head.to_le_bytes());
        }
        for &id in &outcome.cleared_routes {
            let route = snapshot
                .routes
                .get_mut(usize::from(id))
                .ok_or("ruta capturada ausente")?;
            *route = Some(p3_save::ships::SavedRoute::None);
        }
        if let Some(fleets) = &outcome.fleet_data {
            if fleets.len() != snapshot.convoys.len() {
                return Err("convoyes de captura truncados".into());
            }
            snapshot.convoys.clone_from(fleets);
            p3_sim::navigation::refresh_lists(&mut snapshot)?;
        }
        let old_battle = self.paused_battle.as_ref().expect("batalla comprobada");
        if old_battle
            .state
            .local_ships
            .iter()
            .zip(&battle.state.local_ships)
            .any(|(old, new)| old[0x13d] != new[0x13d])
        {
            p3_save::model::validate_initial_convoy_battle(&snapshot, &battle.state.save_input())?;
        }
        let mut loss_merchants = self.sim.rep_merchants.clone();
        for (merchant, record) in loss_merchants.iter_mut().zip(merchant_data.chunks_exact(0x650)) {
            merchant.bytes.copy_from_slice(record)
        }
        let mut loss_pool = self.sim.host.message_pool.clone();
        let date = self.sim.idle.clock.calendar;
        if (!outcome.loss_notices.is_empty()
            || !outcome.reports.is_empty()
            || !outcome.news.is_empty()
            || outcome.merchant_data.is_some())
            && self.sim.economy.merchants.len() != loss_merchants.len()
        {
            return Err("comerciantes desincronizados".into());
        }
        if let Some(records) = &outcome.merchant_data {
            if records.len() != loss_merchants.len() * 0x650 {
                return Err("datos de botín truncados".into());
            }
            for (i, record) in records.chunks_exact(0x650).enumerate() {
                loss_merchants[i].bytes.copy_from_slice(record)
            }
        }
        // Los informes 0x29 se intercalan con las cartas en el orden en que el cierre los envió.
        let add_report = |pool: &mut _, merchants: &mut Vec<_>, report: &p3_core::ships::finish_report::BattleReport| -> Result<(), String> {
            let merchant: &mut _ = merchants
                .get_mut(usize::from(report.recipient))
                .ok_or("destinatario de informe ausente")?;
            p3_state::messages::add_battle_report(
                pool,
                merchant,
                date.day,
                date.month,
                date.year,
                report.town,
                &report.text,
                &[u32::MAX, 0x2800, 0x1400, 0xa00, 0x500, 0],
            )?;
            Ok(())
        };
        let mut next_report = 0;
        for (index, news) in outcome.news.iter().enumerate() {
            while next_report < outcome.reports.len() && outcome.report_positions.get(next_report).is_some_and(|&p| p <= index) {
                add_report(&mut loss_pool, &mut loss_merchants, &outcome.reports[next_report])?;
                next_report += 1;
            }
            match news {
                p3_sim::battle_runtime::BattleNews::Global(letter) => p3_state::messages::add_global_inline_letter(
                    &mut loss_pool,
                    date.day,
                    date.month,
                    date.year,
                    letter,
                )?,
                p3_sim::battle_runtime::BattleNews::Loot { recipient, record } => {
                    if let Some(merchant) = loss_merchants.get_mut(usize::from(*recipient)) {
                        p3_state::messages::add_battle_loot(
                            &mut loss_pool,
                            merchant,
                            date.day,
                            date.month,
                            date.year,
                            record,
                            &[u32::MAX, 0x2800, 0x1400, 0xa00, 0x500, 0],
                        )?;
                    }
                }
                p3_sim::battle_runtime::BattleNews::CapturedCaptain { recipient, record } => {
                    if let Some(merchant) = loss_merchants.get_mut(usize::from(*recipient)) {
                        p3_state::messages::add_pirate_captain_record(
                            &mut loss_pool,
                            merchant,
                            date.day,
                            date.month,
                            date.year,
                            record,
                        )?;
                    }
                }
            }
        }
        for report in &outcome.reports[next_report..] {
            add_report(&mut loss_pool, &mut loss_merchants, report)?;
        }
        for notice in &outcome.loss_notices {
            let merchant = loss_merchants
                .get_mut(usize::from(notice.recipient))
                .ok_or("destinatario de bajas ausente")?;
            p3_state::messages::add_battle_loss(
                &mut loss_pool,
                merchant,
                date.day,
                date.month,
                date.year,
                notice.opponent,
                notice.losses,
            )?;
        }
        let mut new_manager = None;
        let mut merchant_bytes = None;
        let mut town_bytes = None;
        let mut scheduler = None;
        let mut new_hunt_losses: Option<std::collections::BTreeMap<usize, Vec<p3_save::tail::HuntLoss>>> = None;
        let hunt_edited = capture_state.hunt_tasks.iter().any(|(slot, record)| {
            self.sim.host.council_scheduler.records.get(usize::from(*slot)) != Some(record)
        });
        if !capture_state.hunt_added.is_empty() || hunt_edited {
            // 0x540836/0x540B00: capture inside a hunt expedition. The node goes in front of the
            // list at task+8 and `+0xE` may take the "captain found" mark.
            let mut tasks = self.sim.host.council_scheduler.clone();
            let mut lists = self.sim.host.hunt_losses.clone();
            for (slot, record) in &capture_state.hunt_tasks {
                let live = tasks.records.get_mut(usize::from(*slot)).ok_or("tarea de la caza ausente")?;
                let mut expected = *live;
                expected.set_u8_at(0xe, record.u8_at(0xe));
                if expected != *record {
                    return Err("tarea de la caza cambiada fuera de la captura".into());
                }
                *live = *record;
            }
            for (slot, loss) in &capture_state.hunt_added {
                let record = tasks.records.get_mut(usize::from(*slot)).ok_or("tarea de la caza ausente")?;
                p3_sim::battle_manager::prepend_hunt_losses(record, usize::from(*slot), &mut lists, &[*loss]);
            }
            scheduler = Some(tasks);
            new_hunt_losses = Some(lists);
        }
        if !outcome.alderman_reports.is_empty() || !outcome.sponsor_tasks.is_empty() {
            // 0x4EC864 and the type-5 sponsor task (0x4D8CF0) ran inside the capture; no other task
            // change precedes them in this advance. Reports never match a type-5 record.
            let mut tasks = scheduler.take().unwrap_or_else(|| self.sim.host.council_scheduler.clone());
            let count = u16::try_from(self.sim.economy.merchants.len()).map_err(|_| "demasiados comerciantes")?;
            for &(ship, credited, look) in &outcome.alderman_reports {
                tasks.report_captured_look(u32::from(ship), u32::from(credited), count, look)?;
            }
            for task in &outcome.sponsor_tasks {
                task.write(&mut tasks, self.sim.idle.clock.tick);
            }
            scheduler = Some(tasks);
        }
        let mut new_merchant_groups = None;
        let mut new_band_heads = None;
        let mut new_route_stops = None;
        let mut delivered_pirate_notice = false;
        if destroyed {
            // S02 no guarda punteros: la presencia de una ruta propia o
            // compartida se reconstruye con un identificador opaco estable.
            let cvs = [
                u16::from_le_bytes([battle.state.object[0x65e], battle.state.object[0x65f]]),
                u16::from_le_bytes([battle.state.object[0x660], battle.state.object[0x661]]),
            ];
            for (i, route) in snapshot.routes.iter_mut().enumerate() {
                if !outcome.cleared_routes.contains(&(i as u16))
                    && matches!(route, Some(p3_save::ships::SavedRoute::None))
                    && matches!(
                        routes.get(i),
                        Some(Some(
                            p3_save::ships::SavedRoute::Owned(_) | p3_save::ships::SavedRoute::Shared { .. }
                        ))
                    )
                {
                    *route = routes[i].clone()
                }
            }
            for (i, route) in snapshot.routes.iter().enumerate() {
                let at = i * 0x180;
                if at + 0x28 > snapshot.ships.len() {
                    return Err("rutas navales truncadas".into());
                }
                let cv = u16::from_le_bytes([snapshot.ships[at + 8], snapshot.ships[at + 9]]);
                if cvs.contains(&cv)
                    && cv != u16::MAX
                    && snapshot.ships[at + 0x24..at + 0x28] == [0; 4]
                    && matches!(
                        route,
                        Some(p3_save::ships::SavedRoute::Owned(_) | p3_save::ships::SavedRoute::Shared { .. })
                    )
                {
                    let token = 0x7f00_0000u32 | u32::try_from(i + 1).map_err(|_| "índice de ruta excesivo")?;
                    snapshot.ships[at + 0x24..at + 0x28].copy_from_slice(&token.to_le_bytes());
                }
            }
            let frame = self
                .model
                .current_basic_sea_battles()
                .map_err(|e| format!("gestor de batalla: {e:?}"))?;
            let initial = p3_sim::battle_manager::groups_from_save(&frame).map_err(str::to_string)?;
            let (mut bm, mut groups) = if let Some(runtime) = &self.sim.idle.battle_manager {
                (runtime.manager.clone(), runtime.groups.clone())
            } else {
                let mut bm = vec![0; 0xc40];
                if !initial.is_empty() {
                    bm[0xc30..0xc34].copy_from_slice(&1u32.to_le_bytes());
                    bm[0xc34..0xc38].copy_from_slice(&(initial.len() as u32).to_le_bytes())
                }
                (bm, initial.clone())
            };
            let slot = usize::from(u16::from_le_bytes([
                battle.state.object[0x662],
                battle.state.object[0x663],
            ]));
            if slot >= 255 || bm[slot * 4..slot * 4 + 4] != [0; 4] {
                return Err("ranura de batalla no disponible".into());
            }
            bm[slot * 4..slot * 4 + 4].copy_from_slice(&1u32.to_le_bytes());
            bm[0x404] = bm[0x404].max((slot + 1) as u8);
            let mut merchants = loss_merchants
                .iter()
                .flat_map(|m| m.bytes.iter().copied())
                .collect::<Vec<_>>();
            let mut merchant_groups = self.sim.host.merchant_ship_groups.clone();
            let mut pirate_notices = Vec::new();
            if self.sim.economy.world.merchants.len() != self.sim.rep_merchants.len()
                || self.sim.economy.merchants.len() != self.sim.rep_merchants.len()
            {
                return Err("comerciantes desincronizados".into());
            }
            // loss_merchants ya incluye el oro inicial y el botín de este avance.
            let mut towns = self
                .sim
                .economy
                .world
                .towns
                .iter()
                .flat_map(|t| t.0.iter().copied())
                .collect::<Vec<_>>();
            let mut tasks = scheduler.take().unwrap_or_else(|| self.sim.host.council_scheduler.clone());
            let saved_band_ptrs = snapshot.manager[0xc..0x20].to_vec();
            let mut band_heads = captured_heads;
            for j in 0..5 {
                let token = if band_heads[j].is_some() {
                    0x7e00_0000u32 + j as u32
                } else {
                    0
                };
                snapshot.manager[0xc + j * 4..0x10 + j * 4].copy_from_slice(&token.to_le_bytes());
            }
            let captain = self.sim.capture_captain_resources();
            let mut hunt_losses = new_hunt_losses.take().unwrap_or_else(|| self.sim.host.hunt_losses.clone());
            let mut route_stops = self.sim.host.route_stop_runtime.clone();
            let mut context = p3_sim::battle_manager::DestroyWorld {
                manager: &mut bm,
                groups: &mut groups,
                navigation: &mut snapshot.manager,
                ships: &mut snapshot.ships,
                convoys: &mut snapshot.convoys,
                captains: &mut snapshot.captains,
                routes: &mut snapshot.routes,
                pirate_notices: &mut pirate_notices,
                merchants: &mut merchants,
                merchant_groups: &mut merchant_groups,
                band_heads: &mut band_heads,
                towns: &mut towns,
                scheduler: &mut tasks,
                tick: self.sim.idle.clock.tick,
                player: self.owner as u32,
                task_limit: u16::MAX,
                has_hanse_task: false,
                hunt_losses: &mut hunt_losses,
                captain,
                route_stops: &mut route_stops,
            };
            p3_sim::battle_manager::destroy_resolved(&mut battle.state.object, &mut context).map_err(str::to_string)?;
            for notice in &pirate_notices {
                let at = usize::from(notice.recipient) * 0x650;
                let image = merchants.get_mut(at..at + 0x650).ok_or("destinatario pirata ausente")?;
                let mut merchant = p3_core::ai::reputation::Merchant { bytes: image.to_vec() };
                delivered_pirate_notice |= p3_state::messages::add_pirate_captain_notice(
                    &mut loss_pool,
                    &mut merchant,
                    date.day,
                    date.month,
                    date.year,
                    self.sim.economy.world.alderman_town,
                    notice.look,
                    notice.captain,
                    notice.ship,
                )?;
                image.copy_from_slice(&merchant.bytes);
            }
            snapshot.manager[0xc..0x20].copy_from_slice(&saved_band_ptrs);
            p3_sim::navigation::refresh_lists(&mut snapshot)?;
            new_band_heads = Some(band_heads);
            new_manager = Some(p3_sim::battle_manager::Runtime {
                manager: bm,
                groups,
                initial,
                original_block: self.model.sea_battles.clone(),
                error: None,
                rewrite: true,
            });
            merchant_bytes = Some(merchants);
            town_bytes = Some(towns);
            scheduler = Some(tasks);
            new_hunt_losses = Some(hunt_losses);
            new_route_stops = Some(route_stops);
            new_merchant_groups = Some(merchant_groups);
        }
        if let Some(manager) = new_manager.as_ref() {
            self.sim.validate_resolved_battle_manager(manager)?;
        }
        if new_merchant_groups
            .as_ref()
            .is_some_and(|groups| groups.len() != self.sim.host.merchant_ship_groups.len())
        {
            return Err("grupos de comerciante desincronizados".into());
        }
        let final_pool =
            if before.captains != snapshot.captains || before.manager[0xe0..0xe2] != snapshot.manager[0xe0..0xe2] {
                if let Some(factory) = &self.sim.host.expedition_factory {
                    if factory.captains.iter().flatten().copied().collect::<Vec<_>>() != before.captains
                        || factory.free_captain != u16::from_le_bytes([before.manager[0xe0], before.manager[0xe1]])
                    {
                        return Err("pool de capitanes desincronizado".into());
                    }
                }
                Some(p3_core::ships::captains::CaptainPool {
                    capacity: (snapshot.captains.len() / 16) as u16,
                    entries: snapshot.captains.clone(),
                    free_head: u16::from_le_bytes([snapshot.manager[0xe0], snapshot.manager[0xe1]]),
                })
            } else {
                None
            };
        let mut removed = Vec::new();
        for (i, ship) in snapshot.ships.chunks_exact(0x180).enumerate() {
            let at = i * 0x180;
            if u16::from_le_bytes([before.ships[at + 0x134], before.ships[at + 0x135]]) < 0x17
                && u16::from_le_bytes([ship[0x134], ship[0x135]]) == 0xff
            {
                if !matches!(before.routes.get(i), Some(Some(_))) {
                    return Err("descriptor de baja ausente".into());
                }
                snapshot.routes[i] = None;
                removed.push(i)
            }
        }
        if removed
            .iter()
            .any(|&i| usize::from(before.ships[i * 0x180]) >= self.sim.rep_merchants.len())
            && self.sim.host.hanse_pirates.is_none()
        {
            return Err("bandas piratas ausentes".into());
        }
        if let (Some(heads), Some(pirates)) = (new_band_heads, self.sim.host.hanse_pirates.as_ref()) {
            if heads != pirates.heads {
                let missions = self
                    .sim
                    .host
                    .mission_pirates
                    .as_ref()
                    .ok_or("datos de banda ausentes")?;
                for j in 0..5 {
                    match (pirates.heads[j], heads[j], missions[j]) {
                        (Some(old), Some(_), Some(m))
                            if m.first_ship == old || Some(m.first_ship) == captured_heads[j] => {}
                        (None, None, None) => {}
                        _ => return Err("bandas piratas desincronizadas".into()),
                    }
                }
            }
        }
        let band_links = (0..original_ship_count)
            .filter(|&i| {
                !removed.contains(&i)
                    && before.ships[i * 0x180 + 4..i * 0x180 + 6] != snapshot.ships[i * 0x180 + 4..i * 0x180 + 6]
            })
            .collect::<Vec<_>>();
        if let Some(factory) = &self.sim.host.expedition_factory {
            for &i in &band_links {
                let old = &before.ships[i * 0x180 + 4..i * 0x180 + 6];
                let now = &snapshot.ships[i * 0x180 + 4..i * 0x180 + 6];
                let current = factory
                    .ships
                    .get(i)
                    .and_then(|s| s.get(4..6))
                    .ok_or("barco de fábrica ausente")?;
                if current != old && current != now {
                    return Err("enlace de fábrica desincronizado".into());
                }
            }
        }
        if !removed.is_empty() {
            if let Some(factory) = &self.sim.host.expedition_factory {
                if factory.ships.len() != original_ship_count {
                    return Err("pool de barcos desincronizado".into());
                }
            }
            if let Some(pirates) = &self.sim.host.hanse_pirates {
                if pirates.ship_captains.len() != original_ship_count || pirates.next.len() != original_ship_count {
                    return Err("bandas piratas desincronizadas".into());
                }
            }
        }
        if let Some(heads) = new_band_heads {
            for (b, head) in capture_state.bands.iter_mut().zip(heads) {
                if let (Some(b), Some(head)) = (b, head) {
                    b.header[4..6].copy_from_slice(&head.to_le_bytes())
                }
            }
        }
        self.sim
            .validate_battle_capture_publication(&capture_state, &snapshot)?;
        // Todas las comprobaciones han terminado; publicar las vistas juntas.
        self.sim.commit_battle_capture(capture_state, &snapshot);
        if let Some(factory) = self.sim.host.expedition_factory.as_mut() {
            for &i in &band_links {
                factory.ships[i][4..6].copy_from_slice(&snapshot.ships[i * 0x180 + 4..i * 0x180 + 6]);
            }
            for &i in &removed {
                factory.ships[i] = snapshot.ships[i * 0x180..(i + 1) * 0x180].to_vec()
            }
            if !removed.is_empty() {
                factory.free_ship = u16::from_le_bytes([snapshot.manager[0xe2], snapshot.manager[0xe3]])
            }
        }
        let nav = self.sim.host.navigation.as_mut().expect("flota comprobada");
        nav.snapshot = snapshot;
        for &id in &outcome.cleared_routes {
            nav.forget_captured_route(id);
        }
        for &id in &removed {
            nav.forget_captured_route(id as u16);
        }
        if let (Some(heads), Some(pirates)) = (new_band_heads, self.sim.host.hanse_pirates.as_mut()) {
            pirates.heads = heads;
            if let Some(missions) = self.sim.host.mission_pirates.as_mut() {
                for j in 0..5 {
                    if let (Some(head), Some(mission)) = (heads[j], missions[j].as_mut()) {
                        mission.first_ship = head;
                    }
                }
            }
        }
        if let Some(pool) = &final_pool {
            if let Some(factory) = self.sim.host.expedition_factory.as_mut() {
                factory.captains = pool.entries.chunks_exact(16).map(|v| v.try_into().unwrap()).collect();
                factory.free_captain = pool.free_head;
            }
        }
        self.sim.host.investigation_rng = rng;
        if let (Some(groups), Some(merchants)) = (&new_merchant_groups, &merchant_bytes) {
            for (i, record) in merchants.chunks_exact(0x650).enumerate() {
                if groups[i] != self.sim.host.merchant_ship_groups[i]
                    || record[0x60c..0x622] != self.sim.rep_merchants[i].bytes[0x60c..0x622]
                {
                    if !self.sim.host.merchant_ship_groups_changed.contains(&i) {
                        self.sim.host.merchant_ship_groups_changed.push(i)
                    }
                }
            }
            self.sim.host.merchant_ship_groups = groups.clone();
        }
        if let Some(merchants) = merchant_bytes {
            for (i, record) in merchants.chunks_exact(0x650).enumerate() {
                self.sim.rep_merchants[i].bytes.copy_from_slice(record);
                self.sim.economy.merchants[i].bytes.copy_from_slice(record);
                self.sim.economy.world.merchants[i].gold = i32::from_le_bytes(record[..4].try_into().unwrap());
            }
        } else if !outcome.loss_notices.is_empty()
            || !outcome.reports.is_empty()
            || !outcome.news.is_empty()
            || outcome.merchant_data.is_some()
        {
            for (i, merchant) in loss_merchants.into_iter().enumerate() {
                for range in [
                    0..4,
                    8..12,
                    0xe..0x10,
                    0x16..0x18,
                    0x1b..0x1c,
                    0x478..0x47a,
                    0x47c..0x480,
                    0x4b8..0x4bc,
                ] {
                    self.sim.economy.merchants[i].bytes[range.clone()].copy_from_slice(&merchant.bytes[range]);
                }
                self.sim.economy.world.merchants[i].gold = merchant.i32(0);
                self.sim.rep_merchants[i] = merchant;
            }
        }
        if delivered_pirate_notice
            || !outcome.loss_notices.is_empty()
            || !outcome.reports.is_empty()
            || !outcome.news.is_empty()
        {
            self.sim.host.message_pool = loss_pool
        }
        if let Some(towns) = town_bytes {
            for (t, record) in self.sim.economy.world.towns.iter_mut().zip(towns.chunks_exact(0x9f8)) {
                t.0.copy_from_slice(record)
            }
        }
        if let Some(lists) = new_hunt_losses {
            self.sim.host.hunt_losses = lists;
        }
        if let Some(stops) = new_route_stops {
            self.sim.host.route_stop_runtime = stops;
        }
        if let Some(tasks) = scheduler {
            self.sim.host.council_scheduler = tasks
        }
        if let Some(manager) = new_manager {
            self.sim
                .install_resolved_battle_manager(manager)
                .expect("gestor validado antes de publicar");
            self.sim.host.encounter_battle = None;
            self.concurrent_start = None;
            self.paused_battle = None;
            self.battle_selected = 0xff;
        } else {
            self.paused_battle = Some(battle)
        }
        self.sim
            .host
            .publish_navigation_delta(&before, &mut self.sim.economy.world, &mut self.sim.economy.cargo_ships);
        if final_pool.is_some() || !removed.is_empty() {
            if let Some(pirates) = self.sim.host.hanse_pirates.as_mut() {
                if let Some(pool) = &final_pool {
                    pirates.captains = pool.entries.chunks_exact(16).map(|v| v.try_into().unwrap()).collect()
                }
                let nav = self.sim.host.navigation.as_ref().expect("flota comprobada");
                for (i, ship) in nav.snapshot.ships.chunks_exact(0x180).enumerate() {
                    pirates.ship_captains[i] = u16::from_le_bytes([ship[0x42], ship[0x43]]);
                    pirates.next[i] = u16::from_le_bytes([ship[4], ship[5]]);
                }
            }
        }
        let capture_events = outcome
            .capture_events
            .iter()
            .map(|event| match event {
                p3_core::ships::capture_world::WorldCaptureEvent::UiShipState(id, state) => {
                    json!(["ui_ship_state", id, state])
                }
                p3_core::ships::capture_world::WorldCaptureEvent::MapShipState(id, state) => {
                    json!(["map_ship_state", id, state])
                }
            })
            .collect::<Vec<_>>();
        Ok(
            json!({"battle_advanced":ticks,"battle_destroyed":destroyed,"capture_events":capture_events}),
        )
    }
    /// Ejecuta primero la operación 0x94 que quedó encolada en el bombeo
    /// anterior y después calcula la próxima a partir del reloj de pared.
    pub fn pump_battle_time(&mut self, now: u32, mode: u32) -> Result<Value, String> {
        if self.paused_battle.is_none() {
            return Err("no hay batalla reconstruida".into());
        }
        let mut clock = self.battle_clock.unwrap_or(BattleClock {
            pump: p3_core::pump::BattleTimePump {
                enabled: true,
                base: now,
            },
            pending: None,
            mode,
        });
        clock.mode = mode;
        let at = 4 + p3_save::SETTINGS_LEN;
        let ms = u32::from_le_bytes(self.model.header[at..at + 4].try_into().unwrap());
        let raw_div = u32::from_le_bytes(self.model.header[at + 4..at + 8].try_into().unwrap());
        let div = if raw_div > 4 { 2 } else { raw_div };
        let hotseat = u32::from(self.sim.host.expedition_resources.as_ref().is_some_and(|r| r.hotseat));
        if (mode == 1 && div == 0) || (mode == 0 && hotseat == 0 && ms == 0) {
            return Err("división por cero en reloj de batalla".into());
        }
        let mut staged_manager = if let Some(manager) = self.sim.idle.battle_manager.as_ref() {
            let mut image = manager.manager.clone();
            let mut groups = manager.groups.clone();
            let effects = p3_sim::battle_manager::tick(
                &mut image,
                &mut groups,
                false,
                p3_sim::battle_manager::ViewState {
                    busy: false,
                    current_scene_matches: false,
                },
            )
            .map_err(str::to_string)?;
            Some((image, groups, effects))
        } else {
            None
        };
        let mut advanced = 0;
        let mut time_request = None;
        let mut capture_events = Vec::<Value>::new();
        if let Some(ticks) = clock.pending {
            let battle = self.paused_battle.as_ref().unwrap();
            let closed = u16::from_le_bytes([battle.state.object[0x15c], battle.state.object[0x15d]]) == u16::MAX;
            if !closed {
                match self.step_battle(ticks) {
                    Ok(result) => {
                        if let Some(events) = result["capture_events"].as_array() {
                            capture_events.extend(events.iter().cloned())
                        }
                    }
                    Err(error) => {
                        self.battle_clock = None;
                        return Err(error);
                    }
                }
                advanced = ticks
            }
            if self.paused_battle.is_none() {
                self.battle_clock = None;
                return Ok(
                    json!({"state":self.state(),"battle_advanced":advanced,"battle_destroyed":true,"queued":null,"time_request":null,"manager_effects":[],"capture_events":capture_events}),
                );
            }
            let direct = self.paused_battle.as_ref().unwrap().state.object[0xac8] != 0;
            time_request =
                p3_core::pump::battle_time_mode_request(mode, false, false, &[Some(direct)]).map(|op| op.0[3]);
            clock.pending = None;
            clock.pump.enabled = true;
        }
        let mut effects = Vec::new();
        if let Some((image, groups, events)) = staged_manager.take() {
            let manager = self.sim.idle.battle_manager.as_mut().unwrap();
            manager.manager = image;
            manager.groups = groups;
            effects = events;
        }
        let op = clock
            .pump
            .schedule(now, mode, hotseat, ms, div)
            .map_err(str::to_string)?;
        clock.pending = op.map(|op| op.0[1] as u16);
        let queued = clock.pending;
        self.battle_clock = Some(clock);
        Ok(
            json!({"state":self.state(),"battle_advanced":advanced,"queued":queued,"time_request":time_request,"capture_events":capture_events,"manager_effects":effects.iter().map(|e|format!("{e:?}")).collect::<Vec<_>>() }),
        )
    }
    /// Despacha la operación pendiente sin crear otra; útil al pausar o guardar.
    pub fn drain_battle_time(&mut self) -> Result<Value, String> {
        let Some(clock) = self.battle_clock else {
            return Ok(json!({"state":self.state(),"battle_advanced":0,"time_request":null}));
        };
        let mut advanced = 0;
        let mut time_request = None;
        let mut capture_events = Vec::<Value>::new();
        if let Some(ticks) = clock.pending {
            let battle = self.paused_battle.as_ref().ok_or("batalla ausente")?;
            let closed = u16::from_le_bytes([battle.state.object[0x15c], battle.state.object[0x15d]]) == u16::MAX;
            if !closed {
                match self.step_battle(ticks) {
                    Ok(result) => {
                        if let Some(events) = result["capture_events"].as_array() {
                            capture_events.extend(events.iter().cloned())
                        }
                    }
                    Err(error) => {
                        self.battle_clock = None;
                        return Err(error);
                    }
                }
                advanced = ticks
            }
            if let Some(battle) = &self.paused_battle {
                let direct = battle.state.object[0xac8] != 0;
                time_request = p3_core::pump::battle_time_mode_request(clock.mode, false, false, &[Some(direct)])
                    .map(|op| op.0[3]);
            }
        }
        self.battle_clock = None;
        Ok(
            json!({"state":self.state(),"battle_advanced":advanced,"time_request":time_request,"capture_events":capture_events}),
        )
    }
    /// La escritura es transaccional: no modifica el modelo base si falla.
    pub fn save(&self) -> Result<Vec<u8>, String> {
        if self.concurrent_start.is_some() {
            return Err("hay una batalla automática avanzando con el reloj; termínala antes de guardar".into());
        }
        self.sim.require_no_active_town_attack_save()?;
        if self.battle_clock.is_some_and(|c| c.pending.is_some()) {
            return Err("hay un avance de batalla encolado; ejecuta el siguiente bombeo antes de guardar".into());
        }
        if self.sim.host.clock_before_battle {
            return Err("no se puede guardar un tick de encuentro incompleto".into());
        }
        if self.sim.host.encounter_battle.as_ref().is_some_and(|b| {
            !b.operations.is_empty()
                || b.dialog.is_some()
                || b.open_for_player_requested
                || b.object.get(0xac8) != Some(&1)
        }) {
            return Err("no se puede guardar con efectos de encuentro pendientes".into());
        }
        if let Some(e) = &self.sim.host.navigation_error {
            return Err(format!("no se puede guardar una navegación interrumpida: {e}"));
        }
        // La sesión sin batalla activa usa la transacción completa de C10.
        // El comercio debe estar reflejado en la imagen naval antes de guardar.
        if self.paused_battle.is_none()
            && self.sim.host.encounter_battle.is_none()
            && (self.changed_ships.is_empty() || self.sim.host.navigation.is_some())
        {
            if let Some(nav) = &self.sim.host.navigation {
                for &id in &self.changed_ships {
                    let image = nav
                        .snapshot
                        .ships
                        .get(id * 0x180..(id + 1) * 0x180)
                        .ok_or("barco comercial ausente")?;
                    let cargo = self.sim.economy.cargo_ships.get(id).ok_or("carga comercial ausente")?;
                    for w in 0..24 {
                        if image[0x54 + 4 * w..0x58 + 4 * w] != cargo.stock[w].to_le_bytes()
                            || image[0xb4 + 4 * w..0xb8 + 4 * w] != cargo.average_price[w].to_bits().to_le_bytes()
                        {
                            return Err("carga comercial desincronizada".into());
                        }
                    }
                }
            }
            let mut model = self.model.clone();
            self.sim.stage_all(&mut model)?;
            let bytes = model.write();
            Sim::from_save_bytes(&bytes).map_err(|e| format!("verificación de recarga: {e}"))?;
            self.sim.confirm_full_save(&model)?;
            return Ok(bytes);
        }
        let mut model = self.model.clone();
        self.sim.persist_loaded_ship_normalization(&mut model)?;
        self.sim.persist_town_history(&mut model)?;
        self.sim.persist_trade_centers(&mut model)?;
        self.sim.persist_session_world_extras(&mut model)?;
        self.sim.persist_battle_manager(&mut model)?;
        if let Some(battle) = self
            .paused_battle
            .as_ref()
            .filter(|_| self.sim.host.encounter_battle.is_none())
        {
            let before = self
                .model
                .current_basic_sea_battles()
                .map_err(|e| format!("batalla: {e:?}"))?;
            let body = before.body(0).ok_or("batalla anterior ausente")?;
            let nav = self.sim.host.navigation.as_ref().ok_or("flota de batalla ausente")?;
            model.replace_paused_convoy_battle_with_fleet(body, &battle.state.save_input(), &nav.snapshot)?;
        }
        for city in 0..self.sim.economy.world.towns.len() {
            self.sim.stage_construction_city(&mut model, city)?;
        }
        self.sim.persist_sieges(&mut model)?;
        for &id in &self.changed_ships {
            let before = model.ships.ship_image(id)?;
            let mut after = before.clone();
            let cargo = self.sim.economy.cargo_ships.get(id).ok_or("barco ausente")?;
            for w in 0..24 {
                after[0x54 + 4 * w..0x58 + 4 * w].copy_from_slice(&cargo.stock[w].to_le_bytes());
                after[0xb4 + 4 * w..0xb8 + 4 * w].copy_from_slice(&cargo.average_price[w].to_bits().to_le_bytes());
            }
            after[0x114..0x118].copy_from_slice(&cargo.cargo_value.to_le_bytes());
            after[0x3c..0x3e].copy_from_slice(&cargo.dirty.to_le_bytes());
            model.ships.apply_ship_image(id, &before, &after)?;
        }
        if let Some(nav) = &self.sim.host.navigation {
            if let Some(battle) = &self.sim.host.encounter_battle {
                if battle.object.len() < 0x664 {
                    return Err("batalla truncada".into());
                }
                let slot = u16::from_le_bytes([battle.object[0x662], battle.object[0x663]]);
                let slot = u8::try_from(slot).map_err(|_| "ranura de batalla inválida")?;
                if let Some(manager) = &self.sim.idle.battle_manager {
                    model.apply_initial_convoy_battle_with_pairs(
                        &nav.baseline,
                        &nav.snapshot,
                        &battle.initial_save_input()?,
                        slot,
                        &p3_sim::battle_manager::groups_to_save(&manager.groups),
                    )?;
                } else {
                    model.apply_initial_convoy_battle_transition(
                        &nav.baseline,
                        &nav.snapshot,
                        &battle.initial_save_input()?,
                        slot,
                    )?;
                }
                if let Some(advanced) = &self.paused_battle {
                    let before = model
                        .current_basic_sea_battles()
                        .map_err(|e| format!("batalla inicial: {e:?}"))?
                        .body(0)
                        .ok_or("batalla inicial ausente")?
                        .to_vec();
                    model.replace_paused_convoy_battle_with_fleet(
                        &before,
                        &advanced.state.save_input(),
                        &nav.snapshot,
                    )?;
                }
            } else {
                use p3_save::layout::{CONVOY, CONVOY_OBJECT_LEN, SHIP_CORE, SHIP_REST, SHIP_STATE};
                model
                    .ships
                    .apply_navigation_captains(&nav.baseline.captains, &nav.snapshot.captains)?;
                model
                    .ships
                    .apply_navigation_heads(&nav.baseline.manager, &nav.snapshot.manager)?;
                model
                    .ships
                    .apply_navigation_weather(&nav.baseline.manager, &nav.snapshot.manager)?;
                for i in 0..model.ships.ships.len() {
                    let at = i * 0x180;
                    let old = &nav.baseline.ships[at..at + 0x180];
                    let now = &nav.snapshot.ships[at..at + 0x180];
                    let route_changed = nav.baseline.routes.get(i) != nav.snapshot.routes.get(i);
                    if old == now && !route_changed {
                        continue;
                    }
                    if u16::from_le_bytes([now[0x134], now[0x135]]) == 0xff {
                        if route_changed
                            && !(matches!(nav.baseline.routes.get(i), Some(Some(_)))
                                && matches!(nav.snapshot.routes.get(i), Some(None)))
                        {
                            return Err("ruta de baja modificada".into());
                        }
                        if u16::from_le_bytes([old[0x134], old[0x135]]) == 0x15 {
                            model.ships.apply_expired_weather_ship(i, old, now)?
                        } else {
                            model.ships.apply_removed_commercial_ship_with_route(
                                i,
                                old,
                                now,
                                nav.baseline.routes.get(i).and_then(Option::as_ref),
                            )?
                        }
                        continue;
                    }
                    if route_changed {
                        let block = model.ships.write();
                        let saved = p3_save::ships::Ships::parse(&block).map_err(|e| format!("rutas: {e:?}"))?;
                        let stored = saved.saved_route(i).map_err(|e| format!("ruta {i}: {e}"))?;
                        if nav.baseline.routes.get(i).and_then(Option::as_ref) != Some(&stored)
                            && nav.snapshot.routes.get(i).and_then(Option::as_ref) != Some(&stored)
                        {
                            return Err(format!("ruta {i} modificada por otro sistema"));
                        }
                    }
                    let before = model.ships.ship_image(i)?;
                    let mut after = before.clone();
                    for seg in SHIP_CORE
                        .iter()
                        .chain(SHIP_REST.iter())
                        .chain(std::iter::once(&SHIP_STATE))
                    {
                        for j in usize::from(seg.obj)..usize::from(seg.obj + seg.len) {
                            if old[j] != now[j] {
                                if !(0x24..0x28).contains(&j) && before[j] != old[j] && before[j] != now[j] {
                                    return Err(format!("barco {i} modificado por otro sistema en +0x{j:X}"));
                                }
                                after[j] = now[j];
                            }
                        }
                    }
                    if route_changed
                        && matches!(nav.snapshot.routes.get(i), Some(Some(p3_save::ships::SavedRoute::None)))
                    {
                        after[0x24..0x28].fill(0);
                        after[0x36] = 0;
                        model
                            .ships
                            .apply_ship_route(i, &before, &after, &[], self.sim.economy.world.towns.len())?
                    } else {
                        let points = u16::try_from(i).ok().and_then(|id| nav.route_for(id)).or_else(|| {
                            match nav.snapshot.routes.get(i) {
                                Some(Some(p3_save::ships::SavedRoute::Owned(p))) => Some(p.as_slice()),
                                _ => None,
                            }
                        });
                        if let Some(points) = points {
                            model.ships.apply_ship_route(
                                i,
                                &before,
                                &after,
                                points,
                                self.sim.economy.world.towns.len(),
                            )?
                        } else {
                            if route_changed {
                                return Err("ruta modificada no reconstruida".into());
                            }
                            model.ships.apply_ship_image(i, &before, &after)?
                        }
                    }
                }
                for i in 0..model.ships.convoys.len() {
                    let at = i * CONVOY_OBJECT_LEN;
                    let old = &nav.baseline.convoys[at..at + CONVOY_OBJECT_LEN];
                    let now = &nav.snapshot.convoys[at..at + CONVOY_OBJECT_LEN];
                    if old == now {
                        continue;
                    }
                    let record = &mut model.ships.convoys[i];
                    let before = p3_save::layout::image(&CONVOY, CONVOY_OBJECT_LEN, record);
                    for seg in &CONVOY {
                        let o = usize::from(seg.obj);
                        let p = usize::from(seg.pos);
                        for k in 0..usize::from(seg.len) {
                            if old[o + k] != now[o + k] {
                                if before[o + k] != old[o + k] && before[o + k] != now[o + k] {
                                    return Err(format!("convoy {i} modificado por otro sistema"));
                                }
                                record[p + k] = now[o + k];
                            }
                        }
                    }
                }
            }
        } else if self.sim.host.encounter_battle.is_some() {
            return Err("flota de batalla ausente".into());
        }
        self.sim.persist_calendar(&mut model);
        let bytes = model.write();
        Sim::from_save_bytes(&bytes).map_err(|e| format!("verificación de recarga: {e}"))?;
        self.sim.confirm_pirate_band_save(&model)?;
        if self.paused_battle.is_none() && self.sim.host.encounter_battle.is_none() {
            self.sim.confirm_battle_manager_save(&model)
        }
        Ok(bytes)
    }
}

impl Session {
    /// Texto con marcas del juego (`\\l \\r \\c \\fN \\C`...) maquetado y dibujado por el código original
    /// (`TextLayout` `0x004209E0`/`0x00420A10`) con las fuentes de `fonts.ini`, a través de [`ui_rt`]. Devuelve el PNG en base64.
    pub fn ui_text(&self, text: &[u8], width: u32, color: u32, font: u8) -> Result<serde_json::Value, String> {
        let exe = self.original_exe.as_deref().ok_or("falta Patrician3.exe junto al CPR (o P3_ORIGINAL_EXE)")?;
        let archive = p3_assets::cpr::Archive::parse(&self.archive).map_err(|e| format!("{e:?}"))?;
        let r = ui_rt::render_text(exe, &archive, text, width, color, font)?;
        Ok(serde_json::json!({
            "width": r.width,
            "height": r.height,
            "png": r.png_base64,
            "missing": r.missing.iter().map(|a| format!("{a:#x}")).collect::<Vec<_>>(),
        }))
    }
}
