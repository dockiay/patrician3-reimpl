//! Barcos en puerto de `ships::tick` (0x00506720): recorre la lista A (`manager+0xE8`) y aplica el estado de cada barco.
//!
//! Estados de la lista A: 0/10 llegada y trámites del puerto, 1 cuenta atrás, 2 salida, 3 llegada a puerto, 4 baja de la
//! lista, 5 y 8/9 escena del puerto, 0xC sin cambios, 0x11 reinicio en la ciudad o baja; el resto da de baja el barco.
//! Aún sin portar: la lista del mar, los convoyes y los estados 0xF/0x12.., que siguen en el original.
//! Campos nuevos del barco: `+0x30` ritmo, `+0x37`/`+0x39` ciudades, `+0x3C`/`+0x3D`/`+0x3E` marcas y contadores, `+0x44`
//! hora, `+0x50`/`+0x18` restos al reiniciar, `+0x136` indicadores, `+0x138` contador de fase, `+0x15C` ruta comercial.
//! Gestor: `+0x100..+0x106` contadores de barcos del jugador por lista (aquí solo el primero).

use super::motion::get_speed;
use super::remove::{remove_ship, RemoveEvent, RemoveWorld, MERCHANT_SIZE, SHIP_SIZE, TOWN_SIZE};
use super::{r16, r32, w16, w32, DivideError};

/// Llamadas que el estado hace a funciones aún fuera de este módulo.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TickCall {
    /// 0x00514000 para cada uno de los cinco punteros del gestor (solo con el argumento `'o'`).
    Pointer(u32),
    /// 0x0044F940 con el barco.
    Ui(u32),
    /// 0x00517170: reponer la posición en la ciudad actual.
    ResetPosition(u32),
    /// 0x006135C0 (barco, 0) y 0x006132C0 (barco) y 0x00613920 (barco): escena del puerto.
    HarborShow(u32),
    HarborLeave(u32),
    HarborHide(u32),
    /// 0x0052AC80 sobre la ciudad `+0x39` con el barco.
    TownReceive(u32, u32),
    /// 0x005090E0 `set_sail` con el barco.
    SetSail(u32),
    /// 0x00519C90 `finish_docking`.
    FinishDocking(u32),
    /// 0x00518860 y 0x00518B80: trámites del jugador y de la IA con el barco.
    HumanDock(u32),
    AiDock(u32),
    /// 0x00535400 (ciudad, dueño).
    Notify(u32, u32),
    /// Eventos de `remove_ship` y `clear_route` (516840).
    Remove(RemoveEvent),
    ClearRoute(u32),
}

/// `town_append_ship_to_repair_queue` (0x0052AC80). El original recorre el
/// enlace +6 hasta el primer índice fuera de la capacidad; no duplica barcos.
/// Las listas corruptas se rechazan en vez de entrar en un bucle infinito.
pub fn append_repair_ship(w: &mut TickWorld, town: u32, ship: u16) -> Result<(), &'static str> {
    let t = (town as usize).checked_mul(TOWN_SIZE).ok_or("ciudad inválida")?;
    if t + TOWN_SIZE > w.remove.towns.len() || w.remove.manager.len() < 0xf6 {
        return Err("ciudad o gestor inválido");
    }
    let count = r16(w.remove.manager, 0xf4);
    let mut next = r16(w.remove.towns, t + 0x81e);
    let mut previous = None;
    let mut visited = vec![false; count as usize];
    while next < count {
        if next == ship {
            return Ok(());
        }
        let i = next as usize;
        if visited[i] {
            return Err("cola de reparación cíclica");
        }
        visited[i] = true;
        let at = i * SHIP_SIZE + 6;
        if at + 2 > w.remove.ships.len() {
            return Err("barco de reparación inexistente");
        }
        previous = Some(at);
        next = r16(w.remove.ships, at);
    }
    if let Some(at) = previous {
        w16(w.remove.ships, at, ship)
    } else {
        w16(w.remove.towns, t + 0x81e, ship)
    }
    Ok(())
}

pub trait TickHost {
    fn call(&mut self, call: TickCall);
    /// La actualización de cuadrilla necesita las imágenes vivas del mundo.
    fn pirate_group_with_world(&mut self, _world: &mut TickWorld, index: u32) {
        self.call(TickCall::Pointer(index))
    }
    fn town_receive_with_world(&mut self, _world: &mut TickWorld, town: u32, ship: u32) {
        self.call(TickCall::TownReceive(town, ship))
    }
    /// Selección explícita para anfitriones que todavía no ejecutan toda la flota.
    fn process_ship(&mut self, _ship: u32) -> bool {
        true
    }
    /// Se ejecuta antes de mover el enlace de puerto al mar, como en el original.
    fn set_sail(&mut self, _world: &mut TickWorld, ship: u32) {
        self.call(TickCall::SetSail(ship));
    }
    fn finish_docking(&mut self, _world: &mut TickWorld, ship: u32) {
        self.call(TickCall::FinishDocking(ship));
    }
    /// 0x00519EB0 con el enlace de la lista: devuelve `true` si ya sacó el barco de la lista.
    fn ai_state2(&mut self, ship: u32) -> bool;
    /// Igual que `ai_state2`, con el mundo y el enlace de la lista de atracados para anfitriones que ejecutan `ai_depart`.
    fn ai_state2_with_world(&mut self, _world: &mut TickWorld, ship: u32, _link: Link) -> bool {
        self.ai_state2(ship)
    }
    fn ai_docked_with_world(&mut self, _world: &mut TickWorld, ship: u32) {
        self.call(TickCall::AiDock(ship));
    }
}

pub struct TickWorld<'a, 'b> {
    pub remove: &'a mut RemoveWorld<'b>,
    /// `0x006DE4B4`.
    pub time: u32,
    /// `0x006DFC14`.
    pub player: u32,
}

#[derive(Clone, Copy)]
pub enum Link {
    Manager(usize),
    Ship(usize),
}

fn town_flags(w: &TickWorld, ship: usize) -> u32 {
    let town = w.remove.ships[ship + 0x39] as usize;
    r32(w.remove.towns, town * TOWN_SIZE + 0x2C8) as u32
}

/// Cuenta de `+0x3E` (2 bytes con `+0x3F`) cuando ha pasado más de `0x100` ticks desde `+0x44`.
fn age(w: &mut TickWorld, s: usize) {
    let ships = &mut *w.remove.ships;
    if (r32(ships, s + 0x44) as u32).wrapping_add(0x100) < w.time {
        let v = r16(ships, s + 0x3E).wrapping_add(1);
        w16(ships, s + 0x3E, v);
        if (v as i16) < 0x700 {
            if ships[s + 0x3E] == 0 {
                ships[s + 0x3D] |= 2;
            }
        } else {
            w16(ships, s + 0x3E, 0x700);
        }
    }
}

fn is_human(w: &TickWorld, owner: u8) -> bool {
    r16(w.remove.merchants, owner as usize * MERCHANT_SIZE + 8) == 0
}

/// Lista A de `ships::tick` (`manager+0xE8`). `arg` es el byte que recibe la función (`'o'` = 0x6F). Devuelve
/// `Err(DivideError)` si `get_speed` falla. Usa `remove_ship` real; las demás llamadas pasan por `host`.
pub fn tick_docked(w: &mut TickWorld, arg: u8, host: &mut dyn TickHost) -> Result<(), DivideError> {
    w16(w.remove.manager, 0x100, 0);
    w16(w.remove.manager, 0x102, 0);
    w16(w.remove.manager, 0x104, 0);
    w16(w.remove.manager, 0x106, 0);
    if arg == b'o' {
        for k in 0..5 {
            if r32(w.remove.manager, 0xC + 4 * k) != 0 {
                host.pirate_group_with_world(w, k as u32);
            }
        }
    }
    let count = r16(w.remove.manager, 0xF4);
    let mut link = Link::Manager(0xE8);
    loop {
        let index = match link {
            Link::Manager(o) => r16(w.remove.manager, o),
            Link::Ship(o) => r16(w.remove.ships, o),
        };
        if index >= count {
            return Ok(());
        }
        let s = index as usize * SHIP_SIZE;
        let id = index as u32;
        if !host.process_ship(id) {
            link = Link::Ship(s + 6);
            continue;
        }
        let state = r16(w.remove.ships, s + 0x134);
        let player = w.remove.ships[s] as u32 == w.player;
        let advance = Link::Ship(s + 6);
        let set_link = |w: &mut TickWorld, v: u16| match link {
            Link::Manager(o) => w16(w.remove.manager, o, v),
            Link::Ship(o) => w16(w.remove.ships, o, v),
        };
        let sea_remove = |w: &mut TickWorld, host: &mut dyn TickHost| {
            for e in remove_ship(w.remove, id) {
                host.call(TickCall::Remove(e));
            }
        };
        let count_player = |w: &mut TickWorld| {
            let v = r16(w.remove.manager, 0x100).wrapping_add(1);
            w16(w.remove.manager, 0x100, v);
        };
        link = match state {
            10 | 0 => {
                if state == 10 {
                    w16(w.remove.ships, s + 0x134, 0);
                }
                if player {
                    count_player(w);
                }
                age(w, s);
                if is_human(w, w.remove.ships[s]) {
                    if w.remove.ships[s + 0x136] & 1 == 0 {
                        let (here, home) = (w.remove.ships[s + 0x37], w.remove.ships[s + 0x39]);
                        if here == home {
                            if w.time & 0xFF == 0 {
                                host.call(TickCall::Notify(home as u32, w.remove.ships[s] as u32));
                            }
                        } else {
                            host.call(TickCall::Notify(home as u32, w.remove.ships[s] as u32));
                            w.remove.ships[s + 0x37] = home;
                        }
                        advance
                    } else {
                        host.call(TickCall::HumanDock(id));
                        advance
                    }
                } else {
                    host.ai_docked_with_world(w, id);
                    if r16(w.remove.ships, s + 0x134) == 2 && host.ai_state2_with_world(w, id, link) {
                        link
                    } else {
                        advance
                    }
                }
            }
            1 => {
                if player {
                    count_player(w);
                }
                age(w, s);
                let phase = r16(w.remove.ships, s + 0x138);
                if phase > 0 {
                    w16(w.remove.ships, s + 0x138, phase - 1);
                } else {
                    w.remove.ships[s + 0x3C] |= 0x20;
                    w16(w.remove.ships, s + 0x134, 0);
                }
                advance
            }
            2 => {
                if player {
                    count_player(w);
                }
                let phase = r16(w.remove.ships, s + 0x138);
                if phase < 0x40 {
                    w16(w.remove.ships, s + 0x138, phase + 1);
                    if phase == 0 {
                        if town_flags(w, s) & 0x0400_0200 != 0 {
                            w.remove.ships[s + 0x3C] |= 0x20;
                            w16(w.remove.ships, s + 0x134, 1);
                            w16(w.remove.ships, s + 0x138, 0x100);
                            host.call(TickCall::ClearRoute(id));
                        } else {
                            host.call(TickCall::HarborShow(id));
                            w.remove.ships[s + 0x3C] |= 0x80;
                            let speed = get_speed(&w.remove.ships[s..s + SHIP_SIZE])?;
                            w32(w.remove.ships, s + 0x30, (speed >> 1) as i32);
                        }
                    }
                    advance
                } else {
                    host.set_sail(w, id);
                    let next = r16(w.remove.ships, s + 6);
                    set_link(w, next);
                    let sea = r16(w.remove.manager, 0xEA);
                    w16(w.remove.ships, s + 6, sea);
                    w16(w.remove.manager, 0xEA, index);
                    link
                }
            }
            3 => {
                if player {
                    count_player(w);
                }
                let phase = r16(w.remove.ships, s + 0x138);
                if phase < 0x40 {
                    w16(w.remove.ships, s + 0x138, phase + 1);
                    if phase == 0 {
                        let speed = get_speed(&w.remove.ships[s..s + SHIP_SIZE])?;
                        w.remove.ships[s + 0x3D] |= 0x10;
                        w.remove.ships[s + 0x15C] = 0;
                        w32(w.remove.ships, s + 0x30, (speed >> 1) as i32);
                        host.call(TickCall::HarborLeave(id));
                    }
                } else {
                    host.finish_docking(w, id);
                }
                advance
            }
            4 => {
                if player {
                    count_player(w);
                }
                w16(w.remove.ships, s + 0x138, 0);
                let next = r16(w.remove.ships, s + 6);
                set_link(w, next);
                w16(w.remove.ships, s + 6, 0xFFFF);
                w16(w.remove.ships, s + 0x134, 6);
                if w.remove.ships[s + 0x136] == 1 {
                    w.remove.ships[s + 0x136] = 2;
                }
                w.remove.ships[s + 0x3C] |= 0x20;
                let town = w.remove.ships[s + 0x39] as u32;
                host.town_receive_with_world(w, town, id);
                link
            }
            5 => {
                host.call(TickCall::HarborHide(id));
                w.remove.ships[s + 0x3C] |= 0x20;
                w16(w.remove.ships, s + 0x138, 0);
                w16(w.remove.ships, s + 0x134, 0);
                advance
            }
            8 | 9 => {
                let phase = r16(w.remove.ships, s + 0x138);
                if phase < 0x20 {
                    w16(w.remove.ships, s + 0x138, phase + 1);
                    if phase == 0 {
                        host.call(TickCall::HarborShow(id));
                        w.remove.ships[s + 0x3C] |= 0x80;
                        let speed = get_speed(&w.remove.ships[s..s + SHIP_SIZE])?;
                        w32(w.remove.ships, s + 0x30, (speed >> 1) as i32);
                    } else if phase + 1 >= 0x20 {
                        w.remove.ships[s + 0x3C] |= 0x80;
                        w32(w.remove.ships, s + 0x30, 0);
                    }
                }
                advance
            }
            0xC => advance,
            0x11 => {
                let next = r16(w.remove.ships, s + 6);
                set_link(w, next);
                w16(w.remove.ships, s + 6, 0xFFFF);
                w16(w.remove.ships, s + 8, 0xFFFF);
                let owner = w.remove.ships[s] as u16;
                if owner < w.remove.merchant_count && is_human(w, owner as u8) {
                    host.call(TickCall::Ui(id));
                    host.call(TickCall::ResetPosition(id));
                    w32(w.remove.ships, s + 0x50, -1);
                    w32(w.remove.ships, s + 0x18, 0);
                    w16(w.remove.ships, s + 0x40, 0);
                } else {
                    sea_remove(w, host);
                }
                link
            }
            _ => {
                sea_remove(w, host);
                link
            }
        };
    }
}

/// Lo que la lista del mar pide fuera de las listas y de los campos del barco.
pub trait SeaHost {
    fn call(&mut self, call: SeaCall);
    /// Verdadero si el anfitrión solo conoce los barcos con viaje vivo: los barcos humanos en estado 0xF sin ruta
    /// se recorren igualmente (como en el original).
    fn walk_idle_human(&self) -> bool {
        false
    }
    /// 0x00519D90 con el estado del mundo (la carta 0x1B de pérdida del barco humano se construye con sus campos).
    fn prepare_dead_with_world(&mut self, _world: &mut TickWorld, ship: u32) {
        self.call(SeaCall::PrepareDead(ship));
    }
    /// Selección explícita para anfitriones que todavía no ejecutan toda la flota.
    fn process_ship(&mut self, _ship: u32) -> bool {
        true
    }
    /// Permite actualizar la lista ordenada en el mismo momento que el original.
    fn resort(&mut self, _world: &mut TickWorld, ship: u32) {
        self.call(SeaCall::Resort(ship));
    }
    fn clear_route(&mut self, _world: &mut TickWorld, ship: u32) {
        self.call(SeaCall::Remove(RemoveEvent::ClearRoute(ship)));
    }
    fn order_remove(&mut self, _world: &mut TickWorld, ship: u32) {
        self.call(SeaCall::Remove(RemoveEvent::OrderRemove(ship)));
    }
    /// `remove_ship` 0x507FB0 en el tick marítimo. Por defecto ejecuta el port y traduce sus eventos.
    fn remove_ship_with_world(&mut self, world: &mut TickWorld, ship: u32) {
        for e in remove_ship(world.remove, ship) {
            self.call(SeaCall::Remove(e));
        }
    }
    /// `advance_along_route` (0x00516CC0): distinto de cero si sigue en ruta.
    fn advance(&mut self, world: &mut TickWorld, ship: u32) -> bool;
    /// `update_pursuit` (0x0050B030): distinto de cero si ya sacó el barco de la lista.
    fn pursuit(&mut self, ship: u32) -> bool;
    fn pursuit_with_world(&mut self, _world: &mut TickWorld, ship: u32) -> bool {
        self.pursuit(ship)
    }
    /// `ship_ai_divert_to_nearest_open_town` (0x0051A2C0, `except`): la IA expulsada de una ciudad cerrada busca el
    /// puerto abierto más cercano (`ai_dock::nearest_port_plan`) y planifica la ruta (`plan_route`, 0x005172A0).
    fn divert_to_nearest_port_with_world(&mut self, _world: &mut TickWorld, ship: u32, except: u8) {
        self.call(SeaCall::Arrive(ship, except))
    }
    /// 0x0050BA80: distinto de cero si el barco ya no necesita volver a la lista del mar.
    fn fleet_check(&mut self, ship: u32) -> bool;
    /// 0x005062F0 con un parámetro de salida: índice del convoy creado.
    fn convoy_add(&mut self, ship: u32) -> u16;
    /// 0x006130B0 (ciudad, dueño): ranura de escena o 0xFF.
    fn scene_slot(&mut self, town: u8, owner: u8) -> u8;
    /// 0x00611D80: ranura nueva de escena.
    fn new_scene_slot(&mut self) -> u8;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SeaCall {
    Remove(RemoveEvent),
    /// 0x0050D0C0 (reordenar por x) y 0x0050CE50 (insertar con la cabeza de la lista).
    Resort(u32),
    Insert(u32, u16),
    /// 0x00453DF0 (interfaz) y 0x00519D90 (barco).
    UiRefresh,
    PrepareDead(u32),
    /// 0x00548CA0 (dueño, 0, x, y, 0x16, palabra +0x15E).
    Report {
        owner: u16,
        x: u16,
        y: u16,
        word: u16,
    },
    /// 0x005062F0 sobre `manager+0xEC`.
    ConvoyHead(u32),
    /// 0x0060234C(ranura, barco) y 0x005FBFE0(ranura, barco, ciudad).
    SceneAdd(u8, u32),
    SceneNew(u8, u32, u8),
    /// 0x004D5900(barco), 0x0051A2C0(barco, ciudad) y 0x00535400(ciudad, dueño).
    Pool(u32),
    Arrive(u32, u8),
    Notify(u8, u8),
}

/// Lista del mar de `ships::tick` (`manager+0xEA`). Termina donde empiezan los convoyes (0x5070FD).
pub fn tick_sea(w: &mut TickWorld, host: &mut dyn SeaHost) {
    let local = w.time & 0x1F == 0;
    let count = r16(w.remove.manager, 0xF4);
    let mut link = Link::Manager(0xEA);
    loop {
        let index = match link {
            Link::Manager(o) => r16(w.remove.manager, o),
            Link::Ship(o) => r16(w.remove.ships, o),
        };
        if index >= count {
            return;
        }
        let s = index as usize * SHIP_SIZE;
        let id = index as u32;
        // El anfitrión solo conoce los barcos con viaje vivo. Un barco humano en estado 0xF sin ruta (por ejemplo
        // tras una batalla) también recorre `advance_along_route` en el original: pierde vida cada 4 ticks y se
        // reordena por x (ConvoyMar, día 258: los empates de x dependen de ese orden de reordenación).
        let idle_human = host.walk_idle_human()
            && r16(w.remove.ships, s + 0x134) == 0xF
            && r32(w.remove.ships, s + 0x24) == 0
            && r16(w.remove.ships, s + 0x3A) >= count
            && is_human(w, w.remove.ships[s]);
        if !host.process_ship(id) && !idle_human {
            link = Link::Ship(s + 6);
            continue;
        }
        let state = r16(w.remove.ships, s + 0x134);
        let advance = Link::Ship(s + 6);
        let set_link = |w: &mut TickWorld, v: u16| match link {
            Link::Manager(o) => w16(w.remove.manager, o, v),
            Link::Ship(o) => w16(w.remove.ships, o, v),
        };
        let remove = |w: &mut TickWorld, host: &mut dyn SeaHost| host.remove_ship_with_world(w, id);
        let owner = w.remove.ships[s];
        link = match state {
            0xA | 0xB | 0x16 => {
                w.remove.ships[s + 0x3D] |= 1;
                w16(w.remove.ships, s + 0x134, 0x11);
                w32(w.remove.ships, s + 0x18, 0);
                link
            }
            0xF => {
                if owner as u32 == w.player {
                    let v = r16(w.remove.manager, 0x102).wrapping_add(1);
                    w16(w.remove.manager, 0x102, v);
                }
                if local {
                    let v = r16(w.remove.ships, s + 0x3E).wrapping_sub(1);
                    w16(w.remove.ships, s + 0x3E, v);
                    if w.remove.ships[s + 0x3E] == 0xFF {
                        w.remove.ships[s + 0x3D] |= 2;
                        if (w.remove.ships[s + 0x3F] as i8) < -1 {
                            w.remove.ships[s + 0x3F] = 0xFF;
                        }
                    }
                }
                if host.advance(w, id) {
                    if r16(w.remove.ships, s + 0x3A) != 0xFFFF && host.pursuit_with_world(w, id) {
                        link
                    } else {
                        host.resort(w, id);
                        advance
                    }
                } else if r32(w.remove.ships, s + 0x18) <= 0 {
                    host.clear_route(w, id);
                    host.order_remove(w, id);
                    let next = r16(w.remove.ships, s + 6);
                    set_link(w, next);
                    w16(w.remove.ships, s + 6, 0xFFFF);
                    host.call(SeaCall::UiRefresh);
                    w.remove.ships[s + 0x3C] |= 0x20;
                    w16(w.remove.ships, s + 0x134, 0x11);
                    w32(w.remove.ships, s + 0x18, 0);
                    if (owner as u16) < w.remove.merchant_count {
                        w32(w.remove.ships, s + 0x50, -1);
                        host.prepare_dead_with_world(w, id);
                        if !is_human(w, owner) {
                            remove(w, host);
                        } else {
                            host.call(SeaCall::Report {
                                owner: owner as u16,
                                x: r16(w.remove.ships, s + 0x1E),
                                y: r16(w.remove.ships, s + 0x22),
                                word: r16(w.remove.ships, s + 0x15E),
                            });
                        }
                    }
                    link
                } else {
                    let flags = town_flags(w, s);
                    let first = flags & 0x0400_0200 != 0
                        || (flags & 0x400 != 0
                            && (owner as u16) < w.remove.merchant_count
                            && w.remove.ships[s + 0x136] & 1 == 1);
                    host.clear_route(w, id);
                    if first {
                        if w.remove.ships[s + 0x15C] != 0 && flags & 0x0400_0000 == 0 {
                            host.order_remove(w, id);
                            let next = r16(w.remove.ships, s + 6);
                            set_link(w, next);
                            w16(w.remove.ships, s + 6, 0xFFFF);
                            if !host.fleet_check(id) {
                                let head = r16(w.remove.manager, 0xEA);
                                host.call(SeaCall::Insert(id, head));
                                w16(w.remove.ships, s + 6, head);
                                w16(w.remove.manager, 0xEA, index);
                            }
                            w.remove.ships[s + 0x39] = 0xFF;
                        } else {
                            w.remove.ships[s + 0x37] = w.remove.ships[s + 0x39];
                            w.remove.ships[s + 0x39] = 0xFF;
                            w16(w.remove.ships, s + 0x134, 0xF);
                            if (owner as u16) < w.remove.merchant_count {
                                if !is_human(w, owner) {
                                    host.divert_to_nearest_port_with_world(w, id, 0xFF);
                                } else if w.remove.ships[s + 0x136] != 0 {
                                    host.call(SeaCall::Pool(id));
                                }
                            }
                        }
                    } else {
                        host.order_remove(w, id);
                        let next = r16(w.remove.ships, s + 6);
                        set_link(w, next);
                        w16(w.remove.ships, s + 6, 0xFFFF);
                        if w.remove.ships[s + 0x15C] != 0 && r16(w.remove.ships, s + 0x134) == 0x14 {
                            let convoy = host.convoy_add(id);
                            host.call(SeaCall::Notify(w.remove.ships[s + 0x39], owner));
                            host.order_remove(w, id);
                            w16(
                                w.remove.convoys,
                                convoy as usize * super::remove::CONVOY_SIZE + 0x14,
                                0x12,
                            );
                            let town = w.remove.ships[s + 0x39];
                            let slot = host.scene_slot(town, owner);
                            if slot < 0xFF {
                                host.call(SeaCall::SceneAdd(slot, id));
                            } else {
                                let new = host.new_scene_slot();
                                host.call(SeaCall::SceneNew(new, id, town));
                            }
                        } else {
                            w.remove.ships[s + 0x3C] |= 0x20;
                            w16(w.remove.ships, s + 0x134, 3);
                            w16(w.remove.ships, s + 0x138, 0);
                            let head = r16(w.remove.manager, 0xE8);
                            w16(w.remove.ships, s + 6, head);
                            w16(w.remove.manager, 0xE8, index);
                        }
                    }
                    link
                }
            }
            0x11 => {
                let next = r16(w.remove.ships, s + 6);
                set_link(w, next);
                w16(w.remove.ships, s + 6, 0xFFFF);
                host.order_remove(w, id);
                w32(w.remove.ships, s + 0x50, -1);
                if (owner as u16) < w.remove.merchant_count && is_human(w, owner) {
                    link
                } else {
                    host.call(SeaCall::Remove(RemoveEvent::UiA(id)));
                    host.call(SeaCall::Remove(RemoveEvent::UiB(id)));
                    remove(w, host);
                    link
                }
            }
            0x12 => {
                let next = r16(w.remove.ships, s + 6);
                set_link(w, next);
                w16(w.remove.ships, s + 6, 0xFFFF);
                if r32(w.remove.ships, s + 0x18) > 0 {
                    host.call(SeaCall::ConvoyHead(id));
                } else {
                    host.order_remove(w, id);
                    remove(w, host);
                }
                link
            }
            _ => {
                remove(w, host);
                link
            }
        };
    }
}

/// Estado de convoy que `tick_convoys` todavía no porta.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Unported(pub u16);

/// Fallo de `tick_convoys`: estado sin portar o división entre cero en `Convoy::advance`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConvoyError {
    Unported(u16),
    Divide,
}

/// Lo que el bucle de convoyes pide fuera de sus listas. Cada llamada lleva el nombre y los argumentos con que el
/// oráculo la registra; el valor devuelto es el booleano de la rutina o, en las que sacan un elemento de una lista,
/// el valor que queda en el enlace.
pub trait ConvoyHost {
    fn hook(&mut self, name: &'static str, args: &[u32]) -> u32;
    /// Tramo de tareas de `Convoy::advance` (0x00502285..0x00502331): el barco `ship` acaba de salir
    /// del convoy (`ship+8` aún apunta a él). Si `convoy+2` es una tarea 0x34 en fase >= 7 de ese convoy,
    /// el original antepone a `tarea+8` el nodo `{0x672856, dueño, ship+0x15E}` ([`super::convoy_report`]).
    /// Por defecto no hace nada (los anfitriones sin planificador).
    fn convoy_ship_finished_with_world(&mut self, _world: &mut TickWorld, _convoy: u16, _ship: u16) {}
    /// `remove_from_x_order` 0x0050D040 sobre las listas vivas. Por defecto solo se registra.
    fn remove_order_with_world(&mut self, _world: &mut TickWorld, ship: u16) {
        self.hook("order", &[u32::from(ship)]);
    }
    /// `ships::allocate_captain` 0x005097C0 sobre el pool vivo. Por defecto solo se registra.
    fn allocate_captain_with_world(&mut self, _world: &mut TickWorld, flag: u8) -> u16 {
        self.hook("cv_5097c0", &[u32::from(flag)]) as u16
    }
    /// 0x503B50 necesita imágenes, contexto de descubrimiento, CRT y cola.
    fn plan_expedition_with_world(&mut self, _world: &mut TickWorld, convoy: u16) {
        self.hook("cv_503b50", &[u32::from(convoy)]);
    }
    /// La absorción cambia la flota en este mismo punto del recorrido.
    fn absorb_convoy_with_world(&mut self, _world: &mut TickWorld, object: u16, convoy: u16) {
        self.hook("cv_514b80", &[u32::from(object), u32::from(convoy)]);
    }
    fn engage_pirate_with_world(&mut self, _world: &mut TickWorld, convoy: u16) {
        self.hook("cv_50bc40", &[u32::from(convoy)]);
    }
    /// `ship_ai_trade_for_next_destination` (0x00519B50) de un barco de la IA al cambiar el destino del convoy.
    fn trade_for_next_destination_with_world(&mut self, _world: &mut TickWorld, ship: u16) {
        self.hook("cv_retarget", &[u32::from(ship)]);
    }
    /// `ship_ai_divert_to_nearest_open_town` (0x0051A2C0) sobre el barco principal de un convoy de la IA.
    fn divert_to_nearest_port_with_world(&mut self, _world: &mut TickWorld, ship: u32, except: u32) {
        self.hook("arrive", &[ship, except]);
    }
    fn pursuit_with_world(&mut self, _world: &mut TickWorld, ship: u16) -> bool {
        self.hook("pursuit", &[u32::from(ship)]) & 0xff != 0
    }
    fn ship_delivery_with_world(&mut self, _world: &mut TickWorld, owner: u8, count: u32, success: u32) {
        self.hook("cv_4f9050", &[u32::from(owner), count, success]);
    }
    fn convoy_ai_dock_with_world(&mut self, _world: &mut TickWorld, convoy: u16, _leader: u16) {
        self.hook("ai_dock", &[u32::from(convoy)]);
    }
    fn find_prey_with_world(&mut self, _world: &mut TickWorld, ship: u16) -> bool {
        self.hook("cv_50d9e0", &[u32::from(ship)]) & 0xff != 0
    }
    fn pirate_attack_with_world(&mut self, _world: &mut TickWorld, ship: u16) -> bool {
        self.hook("cv_515360", &[u32::from(ship)]) & 0xff != 0
    }
    fn process_convoy(&mut self, _convoy: u16) -> bool {
        true
    }
    fn process_convoy_with_world(&mut self, _world: &mut TickWorld, convoy: u16) -> bool {
        self.process_convoy(convoy)
    }
    fn begin_departure(&mut self, _world: &mut TickWorld, convoy: u16) -> u32 {
        self.hook("cv_501eb0", &[u32::from(convoy)])
    }
    fn set_half_speed(&mut self, _world: &mut TickWorld, convoy: u16) {
        self.hook("cv_503cf0", &[u32::from(convoy)]);
    }
    fn insert_order(&mut self, _world: &mut TickWorld, ship: u16, after: u16) {
        self.hook("insert", &[u32::from(ship), u32::from(after)]);
    }
    fn recompute_ship(&mut self, _world: &mut TickWorld, ship: u16) {
        self.hook("cv_5182b0", &[u32::from(ship)]);
    }
    fn refresh_convoy(&mut self, _world: &mut TickWorld, convoy: u16) {
        self.hook("convoy_refresh", &[u32::from(convoy)]);
    }
    fn set_velocity(&mut self, _world: &mut TickWorld, ship: u16, speed: u32, x: u16, y: u16) {
        self.hook("cv_517410", &[u32::from(ship), speed, u32::from(x), u32::from(y)]);
    }
    fn set_motion(&mut self, _world: &mut TickWorld, time: u32, speed: u32, convoy: u16, heading: u8) {
        self.hook("cv_50a240", &[time, speed, u32::from(convoy), u32::from(heading)]);
    }
    fn clear_ship_route(&mut self, _world: &mut TickWorld, ship: u16) {
        self.hook("clear", &[u32::from(ship)]);
    }
    fn resort_ship(&mut self, _world: &mut TickWorld, ship: u16) {
        self.hook("resort", &[u32::from(ship)]);
    }
    fn remove_order(&mut self, _world: &mut TickWorld, ship: u16) {
        self.hook("order", &[u32::from(ship)]);
    }
    fn finish_docking(&mut self, _world: &mut TickWorld, ship: u16) {
        self.hook("finish", &[u32::from(ship)]);
    }
    /// Lectura de la memoria de rutas del original (punteros de `ship+0x24`), little endian, `width` en bytes.
    fn read(&mut self, addr: u32, width: u8) -> u32;
    /// Globales y tablas de `0x505C90`; sin ellas el planificador de destino sigue siendo el gancho `cv_505c90`.
    fn plan_env(&mut self) -> Option<super::convoy_plan::PlanEnv> {
        None
    }
    fn plan_env_with_world(&mut self, _world: &TickWorld) -> Option<super::convoy_plan::PlanEnv> {
        self.plan_env()
    }
    /// `0x005172A0` (modo 0xFF): fija el punto de destino del barco principal.
    fn set_destination(&mut self, _world: &mut TickWorld, ship: u16, x: u32, y: u32) {
        self.hook("cv_5172a0", &[0xFF, x, y, u32::from(ship)]);
    }
}

/// Enlace de lista de barcos de un convoy que una rutina externa modifica.
#[derive(Clone, Copy)]
enum ListSlot {
    ConvoyHead(usize),
    ShipNext(usize),
}

/// Bucle de convoyes de `ships::tick` (0x005070FD..0x005079DA). `arg` es el byte de la función; `local` indica
/// `time & 0x1F == 0`. Solo el estado 0x14 tras `Convoy::advance` queda sin portar (lee un registro sin inicializar).
pub fn tick_convoys(w: &mut TickWorld, arg: u8, host: &mut dyn ConvoyHost) -> Result<(), ConvoyError> {
    use super::remove::CONVOY_SIZE;
    const TOWN_LIST: usize = 0x820;
    let player = w.player;
    let local = (w.time & 0x1F == 0) as u32;
    // `slot`: convoy cuyo enlace `+8` es el siguiente; `None` es `manager+0xEC`.
    let mut slot: Option<usize> = None;
    let mut guard = 0;
    loop {
        let cur = match slot {
            None => r16(w.remove.manager, 0xEC),
            Some(p) => r16(w.remove.convoys, p * CONVOY_SIZE + 8),
        };
        if cur >= r16(w.remove.manager, 0xF6) || guard > 0x10000 {
            return Ok(());
        }
        guard += 1;
        let c = cur as usize;
        let b = c * CONVOY_SIZE;
        if !host.process_convoy_with_world(w, cur) {
            slot = Some(c);
            continue;
        }
        let count = r16(w.remove.manager, 0xF4);
        let set_slot = |w: &mut TickWorld, slot: Option<usize>, v: u16| match slot {
            None => w16(w.remove.manager, 0xEC, v),
            Some(p) => w16(w.remove.convoys, p * CONVOY_SIZE + 8, v),
        };
        let _set_list = |w: &mut TickWorld, l: ListSlot, v: u16| match l {
            ListSlot::ConvoyHead(k) => w16(w.remove.convoys, k * CONVOY_SIZE + 0xA, v),
            ListSlot::ShipNext(sh) => w16(w.remove.ships, sh * SHIP_SIZE + 6, v),
        };
        let head = r16(w.remove.convoys, b + 0xA);
        if head >= count {
            let next = r16(w.remove.convoys, b + 8);
            set_slot(w, slot, next);
            w16(w.remove.convoys, b + 8, 0xFFFF);
            super::convoy::release_convoy(w, cur, host);
            continue;
        }
        let main = r16(w.remove.convoys, b + 0x10);
        if main >= count {
            super::convoy_drop::dissolve_convoy(w, cur, host)?;
            continue;
        }
        let m = main as usize;
        let state = r16(w.remove.convoys, b + 0x12);
        // Recorre la lista de barcos del convoy por `+6`.
        let chain = |w: &TickWorld| {
            let mut v = Vec::new();
            let mut a = r16(w.remove.convoys, b + 0xA);
            while a < count && v.len() < 0x10000 {
                v.push(a as usize);
                a = r16(w.remove.ships, a as usize * SHIP_SIZE + 6);
            }
            v
        };
        let human = |w: &TickWorld, owner: u8| r16(w.remove.merchants, owner as usize * MERCHANT_SIZE + 8) == 0;
        let bump = |w: &mut TickWorld, off: usize| {
            let v = r16(w.remove.manager, off).wrapping_add(1);
            w16(w.remove.manager, off, v);
        };
        match state {
            0 => {
                let owner = w.remove.ships[m * SHIP_SIZE];
                if owner as u32 == player {
                    bump(w, 0x104);
                }
                if r16(w.remove.convoys, b + 0x14) & 0x10 != 0 {
                    super::convoy_drop::dissolve_convoy(w, cur, host)?;
                    continue;
                }
                let o = m * SHIP_SIZE;
                if !human(w, owner) {
                    host.convoy_ai_dock_with_world(w, cur, main);
                    if r16(w.remove.ships, o + 0x138) < 0x200 {
                        w16(w.remove.ships, o + 0x138, 0x200);
                    }
                } else if w.remove.ships[o + 0x136] & 1 != 0 {
                    let mut done = false;
                    if r16(w.remove.ships, o + 0x132) < w.remove.pool_limit
                        && super::convoy_drop::pool_ok(w, m as u16)
                        && super::convoy_drop::dock_ok(w, m as u16)
                    {
                        host.hook("cv_dock", &[cur as u32]);
                        done = true;
                    }
                    if !done {
                        w.remove.ships[o + 0x3D] |= 8;
                        w.remove.ships[o + 0x136] &= 0xFC;
                        let town = w.remove.ships[o + 0x39] as u32;
                        host.hook("report", &[owner as u32, 0, 0, town, 0x22, main as u32]);
                    }
                }
                let (to, from) = (w.remove.ships[o + 0x39], w.remove.ships[o + 0x38]);
                if to != from {
                    w.remove.ships[o + 0x37] = to;
                    w16(w.remove.ships, o + 0x134, 1);
                    let mut ls = ListSlot::ConvoyHead(c);
                    let mut guard2 = 0;
                    loop {
                        let sh = match ls {
                            ListSlot::ConvoyHead(k) => r16(w.remove.convoys, k * CONVOY_SIZE + 0xA),
                            ListSlot::ShipNext(x) => r16(w.remove.ships, x * SHIP_SIZE + 6),
                        };
                        if sh >= count || guard2 > 0x10000 {
                            break;
                        }
                        guard2 += 1;
                        let so = sh as usize * SHIP_SIZE;
                        let sowner = w.remove.ships[so];
                        if sowner as u32 == player {
                            bump(w, 0x100);
                        }
                        if !human(w, sowner) {
                            let target = w.remove.convoys[b + 0x1F];
                            let cur39 = w.remove.ships[so + 0x39];
                            w.remove.ships[so + 0x38] = target;
                            if target != cur39 {
                                host.trade_for_next_destination_with_world(w, sh);
                                ls = ListSlot::ShipNext(sh as usize);
                            } else {
                                super::convoy::remove_from_convoy(w, sh, host, 0);
                            }
                        } else {
                            let town = w.remove.ships[so + 0x39] as u32;
                            host.hook("notify", &[town, sowner as u32]);
                            ls = ListSlot::ShipNext(sh as usize);
                        }
                    }
                    host.refresh_convoy(w, cur);
                } else {
                    for sh in chain(w) {
                        let so = sh * SHIP_SIZE;
                        let due = r32(w.remove.ships, so + 0x44) as u32;
                        if due.wrapping_add(0x100) < w.time {
                            let v = r16(w.remove.ships, so + 0x3E).wrapping_add(1);
                            w16(w.remove.ships, so + 0x3E, v);
                            if (v as i16) >= 0x700 {
                                w16(w.remove.ships, so + 0x3E, 0x700);
                            } else if v & 0xFF == 0 {
                                let f = r16(w.remove.ships, so + 0x3C) | 0x200;
                                w16(w.remove.ships, so + 0x3C, f);
                            }
                        }
                    }
                }
                slot = Some(c);
            }
            1 => {
                if w.remove.ships[m * SHIP_SIZE] as u32 == player {
                    bump(w, 0x104);
                }
                w16(w.remove.convoys, b + 0x16, 0);
                for s in chain(w) {
                    let o = s * SHIP_SIZE;
                    let ships = &mut *w.remove.ships;
                    let f138 = r16(ships, o + 0x138);
                    ships[o + 0x3C] |= 0x20;
                    w16(ships, o + 0x134, 1);
                    if f138 != 0 {
                        w16(ships, o + 0x138, f138 - 1);
                    }
                    let f138 = r16(ships, o + 0x138);
                    if f138 > r16(w.remove.convoys, b + 0x16) {
                        w16(w.remove.convoys, b + 0x16, f138);
                    }
                    let v = r16(ships, o + 0x3E).wrapping_add(1);
                    w16(ships, o + 0x3E, v);
                    if (v as i16) >= 0x700 {
                        w16(ships, o + 0x3E, 0x700);
                    } else if v & 0xFF == 0 {
                        ships[o + 0x3D] |= 2;
                    }
                }
                if r16(w.remove.convoys, b + 0x16) == 0 {
                    let cv = &mut *w.remove.convoys;
                    cv[b + 0x1E] = cv[b + 0x1C];
                    let diff = (cv[b + 0x1F] as u16).wrapping_sub(r16(cv, b + 0x1C));
                    let st = if diff != 0 { 2 } else { 0 };
                    w16(cv, b + 0x12, st);
                    for s in chain(w) {
                        w16(w.remove.ships, s * SHIP_SIZE + 0x134, st);
                    }
                }
                slot = Some(c);
            }
            2 => {
                if w.remove.convoys[b] as u32 == player {
                    bump(w, 0x104);
                }
                let wait = r16(w.remove.convoys, b + 0x16);
                if wait == 0 {
                    if host.begin_departure(w, cur) & 0xFF != 0 {
                        host.hook("show", &[main as u32, 0]);
                    }
                    host.set_half_speed(w, cur);
                    w32(w.remove.convoys, b + 0x18, 0x20);
                } else {
                    w16(w.remove.convoys, b + 0x16, wait.wrapping_add(1));
                    if wait < 0x40 {
                        w16(w.remove.ships, m * SHIP_SIZE + 0x138, wait.wrapping_add(1));
                    } else {
                        w16(w.remove.convoys, b + 0x12, 0xF);
                        w16(w.remove.convoys, b + 0x16, 0);
                        w16(w.remove.convoys, b + 0x1C, 0xFF);
                        w32(w.remove.convoys, b + 0x18, -1);
                        let sea = r16(w.remove.manager, 0xEA) as u32;
                        host.insert_order(w, main, sea as u16);
                        w.remove.convoys[b + 0x14] |= 2;
                        for s in chain(w) {
                            let so = s * SHIP_SIZE;
                            host.recompute_ship(w, s as u16);
                            let f = r16(w.remove.ships, so + 0x3C) | 0x28;
                            w16(w.remove.ships, so + 0x3C, f);
                            w16(w.remove.ships, so + 0x134, 0xF);
                            w.remove.ships[so + 0x39] = 0xFF;
                            if w.remove.ships[so] != w.remove.convoys[b] {
                                w.remove.convoys[b + 0x14] &= 0xFD;
                            }
                        }
                        if w.remove.convoys[b + 0x14] & 2 == 2 {
                            host.plan_expedition_with_world(w, cur);
                        }
                    }
                }
                slot = Some(c);
            }
            3 => {
                if w.remove.convoys[b] as u32 == player {
                    bump(w, 0x104);
                }
                let wait = r16(w.remove.convoys, b + 0x16);
                if wait >= 0x40 {
                    w16(w.remove.convoys, b + 0x12, 0);
                    for s in chain(w) {
                        host.finish_docking(w, s as u16);
                    }
                    let owner = w.remove.convoys[b];
                    if (owner as u16) < w.remove.merchant_count {
                        let amount = r32(w.remove.convoys, b + 0x20);
                        let mo = owner as usize * MERCHANT_SIZE;
                        if amount > 0 {
                            let v = r32(w.remove.merchants, mo).wrapping_add(amount);
                            w32(w.remove.merchants, mo, v);
                        }
                        if human(w, owner) && w.remove.ships[m * SHIP_SIZE + 0x136] & 1 == 0 {
                            let town = r16(w.remove.convoys, b + 0x1C) as u32;
                            host.hook("report", &[owner as u32, 0, 0, town, 3, main as u32]);
                        }
                    }
                    w32(w.remove.convoys, b + 0x20, 0);
                    w16(w.remove.ships, m * SHIP_SIZE + 0x138, 0);
                    w16(w.remove.convoys, b + 0x16, 0);
                    let ok = super::convoy::pick_main(w, cur, host);
                    let flags = r16(w.remove.convoys, b + 0x14);
                    if !(ok && flags & 2 != 0 && flags & 0x10 == 0) {
                        super::convoy_drop::dissolve_convoy(w, cur, host)?;
                        continue;
                    }
                } else {
                    w16(w.remove.convoys, b + 0x16, wait + 1);
                    if wait == 0 {
                        host.hook("leave", &[main as u32]);
                        for s in chain(w) {
                            let f = r16(w.remove.ships, s * SHIP_SIZE + 0x3C) | 0x1000;
                            w16(w.remove.ships, s * SHIP_SIZE + 0x3C, f);
                            w.remove.ships[s * SHIP_SIZE + 0x15C] = 0;
                        }
                    }
                    w16(w.remove.ships, m * SHIP_SIZE + 0x138, r16(w.remove.convoys, b + 0x16));
                }
                slot = Some(c);
            }
            4 => {
                let town = r16(w.remove.convoys, b + 0x1C) as u32;
                if town >= w.remove.town_count {
                    w16(w.remove.convoys, b + 0x12, 0xF);
                    slot = Some(c);
                    continue;
                }
                w16(w.remove.convoys, b + 0x12, 6);
                for s in chain(w) {
                    w.remove.ships[s * SHIP_SIZE + 0x3C] |= 0x20;
                    w16(w.remove.ships, s * SHIP_SIZE + 0x134, 6);
                }
                let next = r16(w.remove.convoys, b + 8);
                set_slot(w, slot, next);
                let t = town as usize * TOWN_SIZE + TOWN_LIST;
                let old = r16(w.remove.towns, t);
                w16(w.remove.convoys, b + 8, old);
                w16(w.remove.towns, t, cur);
            }
            9 => {
                super::convoy_drop::state9(w, cur);
                slot = Some(c);
            }
            0xA => {
                let v = r16(w.remove.convoys, b + 0x16);
                if v != 0 {
                    w16(w.remove.convoys, b + 0x16, v - 1);
                }
                slot = Some(c);
            }
            0xB | 0x13 => slot = Some(c),
            0xF | 0x16 => {
                if w.remove.convoys[b] as u32 == player {
                    bump(w, 0x106);
                }
                let advanced = super::convoy::convoy_advance(w, cur, local, host).map_err(|_| ConvoyError::Divide)?;
                let main = r16(w.remove.convoys, b + 0x10);
                if main >= count {
                    continue;
                }
                let m = main as usize;
                let o = m * SHIP_SIZE;
                if advanced {
                    if r16(w.remove.ships, o + 8) != cur {
                        super::convoy::pick_main(w, cur, host);
                        continue;
                    }
                    if r16(w.remove.ships, o + 0x3A) != 0xFFFF {
                        if host.pursuit_with_world(w, main) {
                            continue;
                        }
                        let tgt = r16(w.remove.ships, o + 0x3A);
                        if r32(w.remove.ships, o + 0x24) == 0 || tgt >= count {
                            w32(w.remove.ships, o + 0x28, 0);
                            w32(w.remove.ships, o + 0x2C, 0);
                        }
                    }
                    host.resort_ship(w, main);
                    slot = Some(c);
                    continue;
                }
                if r16(w.remove.convoys, b + 0x12) == 0x14 {
                    return Err(ConvoyError::Unported(0x14));
                }
                host.remove_order(w, main);
                for s in chain(w) {
                    let so = s * SHIP_SIZE;
                    if r16(w.remove.ships, so + 0xA) < count || r16(w.remove.ships, so + 0xC) < count {
                        host.remove_order(w, s as u16);
                    }
                }
                slot = Some(c);
            }
            0x12 => {
                let o = m * SHIP_SIZE;
                if r16(w.remove.ships, o + 0xA) == 0xFFFF && r16(w.remove.ships, o + 0xC) == 0xFFFF {
                    host.hook("insert", &[main as u32, 0xFFFF]);
                }
                let wait = r16(w.remove.convoys, b + 0x16);
                if wait != 0 {
                    w16(w.remove.convoys, b + 0x16, wait - 1);
                    w16(w.remove.ships, o + 0x138, wait - 1);
                }
                let flags = r16(w.remove.convoys, b + 0x14);
                // `follow`: se salta al tramo 507BAE (el objetivo está en estado 0xF o 0x10, o lo aceptan 50D9E0 y 515360).
                let mut follow = false;
                let mut retry = true;
                if flags & 0x40 != 0 {
                    w16(w.remove.convoys, b + 0x14, flags & 0xFFBF);
                    let tgt = r16(w.remove.ships, o + 0x50);
                    if tgt < count {
                        let to = tgt as usize * SHIP_SIZE;
                        let towner = w.remove.ships[to] as u16;
                        if towner < w.remove.merchant_count {
                            let st = r16(w.remove.ships, to + 0x134);
                            if st == 0xF || st == 0x10 {
                                follow = true;
                            }
                        }
                    }
                } else if arg & 3 == 0 && host.find_prey_with_world(w, main) && host.pirate_attack_with_world(w, main) {
                    follow = true;
                } else {
                    retry = true;
                }
                if follow {
                    let tgt = r16(w.remove.ships, o + 0x50);
                    if tgt < count && r16(w.remove.ships, tgt as usize * SHIP_SIZE + 0x134) == 0xF {
                        let prev = cur;
                        let next = r16(w.remove.convoys, b + 8);
                        set_slot(w, slot, next);
                        w16(w.remove.convoys, b + 8, 0xFFFF);
                        host.engage_pirate_with_world(w, prev);
                        continue;
                    }
                }
                if retry {
                    if r32(w.remove.ships, o + 0x24) == 0
                        && match host.plan_env_with_world(w) {
                            Some(env) => super::convoy_plan::plan_destination(w, cur, slot, &env, host)?,
                            None => host.hook("cv_505c90", &[]) & 0xFF != 0,
                        }
                    {
                        continue;
                    }
                    if super::convoy::convoy_advance(w, cur, 1, host).map_err(|_| ConvoyError::Divide)? {
                        let main = r16(w.remove.convoys, b + 0x10);
                        host.resort_ship(w, main);
                        slot = Some(c);
                    } else {
                        let next = r16(w.remove.convoys, b + 8);
                        set_slot(w, slot, next);
                        w16(w.remove.convoys, b + 8, 0xFFFF);
                        let main = r16(w.remove.convoys, b + 0x10);
                        if main >= count {
                            super::convoy::release_convoy(w, cur, host);
                        } else {
                            host.remove_order_with_world(w, main);
                            let town = w.remove.convoys[b + 0x1C];
                            w.remove.convoys[b + 0x1E] = town;
                            let s = host.hook("new_slot", &[]) as u8;
                            host.hook("scene_new", &[s as u32, main as u32, town as u32]);
                        }
                    }
                }
            }
            _ => {
                let mut g = 0;
                while r16(w.remove.convoys, b + 0xA) < count && g < 0x10000 {
                    g += 1;
                    let sh = r16(w.remove.convoys, b + 0xA);
                    super::convoy::remove_from_convoy(w, sh, host, 0);
                }
                super::convoy::release_convoy(w, cur, host);
            }
        }
    }
}
