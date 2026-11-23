//! `SeaBattle::update(ticks)` (0x00606CC0, método virtual 5): prólogo de interfaz y cuentas atrás humanas, estadísticas por
//! bando, viento, paso de cada barco, proyectiles, detección del fin y despacho de los pares del índice espacial.
//!
//! Todo lo que toca la interfaz, el índice espacial, las armas y los proyectiles pasa por [`UpdateHost`], que registra el
//! orden de las llamadas del original. Fixture: `oracle/x86_battle_update.py`.

use super::boarding::{artillery_strength, effective_speed, BoardingHost, Captains};
use super::collide::{dispatch_pair, BoardCtx, PairHost, Slots};
use super::step::{local_ship_step, StepContext, StepEvent, StepOutcome};
use super::wind::update_wind;
use super::{r16, r32, w16, w32};

type E = &'static str;

/// Globales del juego que lee o escribe `update` fuera del objeto de combate.
pub struct Globals<'a> {
    /// Jugador activo (0x006DFC14).
    pub player: u32,
    /// Partida de varios jugadores en el mismo equipo (0x006DFC18 distinto de cero).
    pub hotseat: bool,
    /// La interfaz existe (0x006CBDD8 distinto de cero).
    pub interface_present: bool,
    /// `[interfaz+0x5A4] == [0x0066BDC0]`.
    pub interface_matches: bool,
    /// Palabra `+8` de cada comerciante (0 = humano); su longitud es el recuento 0x006DE4AA.
    pub merchant_words: &'a [u16],
    /// Tabla de convoyes (0x006DD7A8, 60 bytes por entrada).
    pub fleets: &'a [u8],
    /// Número de barcos mundiales (0x006DD894).
    pub world_count: u16,
    /// 0x006E59CD: hay un mensaje abierto.
    pub dialog_open: u8,
    /// 0x006E59CC: combate que mira el jugador.
    pub viewed: u8,
}

/// Contexto de solo lectura de los barcos.
pub struct SeaCtx<'a> {
    pub captains: Captains<'a>,
    pub human_owner: &'a dyn Fn(u8) -> bool,
    pub difficulty: u8,
    pub virtual_10: bool,
}

/// Todo lo externo que llama el original, en el orden en que lo llama.
pub trait UpdateHost: BoardingHost {
    /// Instantánea para callbacks de abordaje que consultan otros barcos.
    fn prepare_ship_step(&mut self, _worlds: &[u8]) {}
    /// Publica cambios de listas y de otros barcos realizados por el abordaje.
    fn sync_capture(&mut self, _worlds: &mut [u8], _next: &mut [u16]) {}
    fn ui_busy(&mut self) -> bool;
    fn ui_release(&mut self);
    fn show_message(&mut self, message: &[u8]);
    fn notify_battle(&mut self, slot: u16, x: i16, y: i16);
    fn push_event(&mut self, code: u32, slot: u16, player: u32, flag: u32);
    fn projectile_present(&mut self, index: u16) -> bool;
    fn advance_projectile(&mut self, battle: &mut [u8], ships: &mut [Vec<u8>], worlds: &mut [u8], index: u16);
    fn weapons(&mut self, battle: &mut [u8], ships: &mut [Vec<u8>], worlds: &mut [u8], ship: usize);
    fn index_bounds(&mut self, slot: u16, x0: i32, y0: i32, x1: i32, y1: i32);
    fn index_remove(&mut self, slot: u16);
    /// Liberación del recurso gráfico del barco (`StepEvent::Release`).
    fn release_graphic(&mut self, value: i32);
    /// 0x006197F0: ordena y llama a `f(huecos, a, b, efectos)` por cada pareja candidata; `efectos` es la parte
    /// [`PairHost`] del anfitrión, separada de las tablas de huecos que prestan los `Slots`.
    fn collision_pairs(
        &mut self,
        f: &mut dyn FnMut(&Slots, u16, u16, &mut dyn PairHost) -> Result<(), E>,
    ) -> Result<(), E>;
    /// Destructor con borrado (método virtual 1 con argumento 1).
    fn destroy(&mut self);
    /// `finish` (método virtual 7): `survivors` = queda algún barco útil en la lista del segundo bando.
    fn finish(&mut self, battle: &mut [u8], ships: &mut [Vec<u8>], worlds: &mut [u8], survivors: bool)
        -> Result<(), E>;
}

fn lookup(worlds: &[u8], ship: &[u8]) -> Result<(usize, usize), E> {
    let index = usize::try_from(r32(ship, 4)).map_err(|_| "barco mundial fuera de rango")?;
    let at = index.checked_mul(0x180).ok_or("barco mundial fuera de rango")?;
    if at + 0x180 > worlds.len() {
        return Err("barco mundial fuera de rango");
    }
    Ok((index, at))
}

fn show_pending<H: UpdateHost>(battle: &mut [u8], g: &mut Globals, host: &mut H) {
    host.show_message(&battle[0x12EC..]);
    battle[0x12E9] = 0;
    g.dialog_open = 1;
    battle[0x12EA] = 1;
}

/// Convoy de la ranura: iterar su cadena de barcos mundiales buscando al jugador.
fn fleet_has_player(battle: &[u8], g: &Globals, worlds: &[u8], fleet_word: usize) -> Result<bool, E> {
    let fleet = usize::from(r16(battle, fleet_word));
    let entry = g.fleets.get(fleet * 60..fleet * 60 + 12).ok_or("convoy inexistente")?;
    let mut id = r16(entry, 0xA);
    let mut present = false;
    let mut guard = 0;
    while id < g.world_count {
        let at = usize::from(id) * 0x180;
        let ship = worlds.get(at..at + 0x180).ok_or("barco mundial fuera de rango")?;
        let owner = u16::from(ship[0]);
        let human =
            (g.hotseat && usize::from(owner) < g.merchant_words.len() && g.merchant_words[usize::from(owner)] == 0)
                || g.player == u32::from(owner);
        if human {
            present = true;
        }
        id = r16(ship, 6);
        guard += 1;
        if guard > 70_000 {
            return Err("lista de convoy cíclica");
        }
    }
    Ok(present)
}

/// Bucle de los dos bandos que mide fuerza, tripulación, artillería, velocidad y centro.
fn side_statistics(battle: &mut [u8], ships: &[Vec<u8>], worlds: &[u8], next: &[u16], ctx: &SeaCtx) -> Result<(), E> {
    for offset in [
        0xA94usize, 0xA90, 0xAA4, 0xAA0, 0xAB8, 0xAB4, 0xAC0, 0xABC, 0xAAC, 0xAA8,
    ] {
        w32(battle, offset, 0);
    }
    for offset in [0xA9Ausize, 0xA98, 0xA9E, 0xA9C] {
        w16(battle, offset, 0);
    }
    let mut counts = [0u16; 2];
    for side in 0..2usize {
        let mut id = if side == 0 {
            r16(battle, 0x10)
        } else {
            r16(battle, 0x65C)
        };
        while id != 0xFFFF {
            let ship = ships.get(usize::from(id)).ok_or("barco local ausente")?;
            if ship[0x12A] & 0x26 == 0 {
                let (index, at) = lookup(worlds, ship)?;
                let world = &worlds[at..at + 0x180];
                let strength = artillery_strength(world, ctx.captains);
                w32(
                    battle,
                    0xA90 + side * 4,
                    r32(battle, 0xA90 + side * 4).wrapping_add(strength),
                );
                let crew = r16(world, 0x40);
                if crew > r16(battle, 0xA98 + side * 2) {
                    w16(battle, 0xA98 + side * 2, crew);
                }
                let extra = r16(world, 0x154);
                let boarding = if crew < extra {
                    u32::from(crew) * 2
                } else {
                    u32::from(crew) + u32::from(extra)
                } as u16;
                if boarding > r16(battle, 0xA9C + side * 2) {
                    w16(battle, 0xA9C + side * 2, boarding);
                }
                let guns = r32(world, 0x120) as u32;
                if guns > r32(battle, 0xAA0 + side * 4) as u32 {
                    w32(battle, 0xAA0 + side * 4, guns as i32);
                }
                let speed = effective_speed(world, ctx.captains)?;
                if (r32(battle, 0xAA8 + side * 4) as u32) < speed {
                    w32(battle, 0xAA8 + side * 4, speed as i32);
                }
                let _ = index;
                counts[side] = counts[side].wrapping_add(1);
                let x = i32::from(r16(ship, 0xA) as i16);
                w32(battle, 0xAB4 + side * 4, r32(battle, 0xAB4 + side * 4).wrapping_add(x));
                let y = i32::from(r16(ship, 0xE) as i16);
                w32(battle, 0xABC + side * 4, r32(battle, 0xABC + side * 4).wrapping_add(y));
            }
            id = *next.get(usize::from(id)).ok_or("lista de barcos truncada")?;
        }
    }
    for side in 0..2usize {
        let n = u32::from(counts[side]);
        if n != 0 {
            let x = (r32(battle, 0xAB4 + side * 4) as u32) / n;
            w32(battle, 0xAB4 + side * 4, x as i32);
            let y = (r32(battle, 0xABC + side * 4) as u32) / n;
            w32(battle, 0xABC + side * 4, y as i32);
        } else {
            w32(battle, 0xAB4 + side * 4, 0x1100);
            w32(battle, 0xABC + side * 4, 0xF02);
        }
    }
    Ok(())
}

#[derive(Default)]
struct SideFlags {
    /// `[esp+0x10/0x11]`: el bando solo tiene barcos con bandera 0x40.
    all_flagged: [bool; 2],
    /// `[esp+0x12/0x13]`: algún barco del bando tiene espacio libre.
    spare_room: [bool; 2],
    /// `[esp+0x14/0x15]`: el bando carga algo con la bandera 0x40 y mercancía.
    carrying: [bool; 2],
    /// `[esp+0x16/0x17]`: todos los barcos tienen la bandera 0x20.
    all_surrendered: [bool; 2],
}

/// Un paso de barco por cada elemento de la lista del bando y recogida de banderas.
#[allow(clippy::too_many_arguments)]
fn step_side<H: UpdateHost>(
    side: usize,
    battle: &mut Vec<u8>,
    ships: &mut Vec<Vec<u8>>,
    worlds: &mut Vec<u8>,
    next: &mut [u16],
    near: &[u16],
    far: &[u16],
    g: &Globals,
    ctx: &SeaCtx,
    host: &mut H,
    flags: &mut SideFlags,
) -> Result<(), E> {
    let mut id = if side == 0 {
        r16(battle, 0x10)
    } else {
        r16(battle, 0x65C)
    };
    while id != 0xFFFF {
        let me = usize::from(id);
        let (flag, world_at) = {
            let ship = ships.get(me).ok_or("barco local ausente")?;
            (ship[0x12A], lookup(worlds, ship)?.1)
        };
        if flag & 6 == 0 {
            if flag & 0x20 == 0 {
                flags.all_surrendered[side] = false;
            }
            if flag & 0x40 != 0 {
                if r32(worlds, world_at + 0x118) > 0 {
                    for slot in 0..24usize {
                        if r32(worlds, world_at + 0x54 + slot * 4) != 0 {
                            flags.carrying[side] = true;
                            break;
                        }
                    }
                }
            } else {
                flags.all_flagged[side] = false;
                if flag & 0x20 == 0 {
                    let free = r32(worlds, world_at + 0x10)
                        .wrapping_sub(r32(worlds, world_at + 0x11C))
                        .wrapping_sub(r32(worlds, world_at + 0x118));
                    if free >= 0x7D0 {
                        flags.spare_room[side] = true;
                    }
                }
            }
        }
        let step_ctx = StepContext {
            next,
            slots_near: near,
            slots_far: far,
            captains: ctx.captains,
            human_owner: ctx.human_owner,
            difficulty: ctx.difficulty,
            virtual_10: ctx.virtual_10,
            view_active: u16::from(g.viewed) == r16(battle, 0x662),
        };
        let mut events = Vec::new();
        if flag & 8 != 0 {
            host.prepare_ship_step(worlds);
        }
        let outcome = local_ship_step(me, battle[0x670], battle, ships, worlds, &step_ctx, host, &mut events)?;
        host.sync_capture(worlds, next);
        for event in events {
            match event {
                StepEvent::Release(value) => host.release_graphic(value),
                StepEvent::SpatialRemove(slot) => host.index_remove(slot),
                StepEvent::SpatialBounds { slot, x0, y0, x1, y1 } => host.index_bounds(slot, x0, y0, x1, y1),
            }
        }
        if outcome == StepOutcome::Weapons {
            host.weapons(battle, ships, worlds, me);
        }
        id = *next.get(me).ok_or("lista de barcos truncada")?;
    }
    Ok(())
}

/// Marca como huidos los barcos útiles del bando que pierde (bloques de 0x00607760 y 0x00607800).
fn flee_side(battle: &mut [u8], ships: &mut [Vec<u8>], next: &[u16], participant: usize) -> Result<(), E> {
    let low = u16::from(battle[0x35A + participant * 2]);
    let mut id = if r16(battle, 0x35A) == low {
        r16(battle, 0x65C)
    } else {
        r16(battle, 0x10)
    };
    let count = r16(battle, 0x14);
    while id < count {
        let ship = ships.get_mut(usize::from(id)).ok_or("barco local ausente")?;
        if ship[0x12A] & 0xE == 0 {
            ship[0x12A] |= 2;
            battle[0xAC4] = 0;
            w16(battle, 0xAC6, id);
        }
        id = *next.get(usize::from(id)).ok_or("lista de barcos truncada")?;
    }
    Ok(())
}

/// `SeaBattle::update(ticks)`. Termina con `Ok` salvo que una rama no modelada (pantalla, listas rotas) aborte.
#[allow(clippy::too_many_arguments)]
pub fn sea_update<H: UpdateHost>(
    battle: &mut Vec<u8>,
    ships: &mut Vec<Vec<u8>>,
    worlds: &mut Vec<u8>,
    next: &mut [u16],
    near: &[u16],
    far: &[u16],
    g: &mut Globals,
    ctx: &SeaCtx,
    host: &mut H,
    ticks: u32,
) -> Result<(), E> {
    if battle.len() < 0x1E98 {
        return Err("batalla truncada");
    }
    let mut ticks = ticks;
    // Prólogo: presencia del jugador y mensajes pendientes.
    if battle[0x5C] == 0 {
        if battle[0x1E92] != 0 {
            battle[0x1E92] = 0;
            for fleet_word in [0x65Eusize, 0x660] {
                if fleet_has_player(battle, g, worlds, fleet_word)? {
                    battle[0x1E92] = 1;
                }
            }
            if battle[0x1E92] != 0 && g.interface_present {
                host.notify_battle(r16(battle, 0x662), r16(battle, 0xAD6) as i16, r16(battle, 0xADA) as i16);
                battle[0x1E92] = 0;
            }
        }
        if battle[0x12E9] != 0 {
            if g.dialog_open == 0 {
                show_pending(battle, g, host);
            }
        } else {
            let count = usize::from(battle[0x558]);
            let mut found = 0xFFusize;
            for i in 0..count {
                if g.player == u32::from(r16(battle, 0x35A + 2 * i)) {
                    found = i;
                }
            }
            if count != 0 && found != 0xFF {
                let timer = r16(battle, 0x15C + found * 2);
                if timer != 0 && timer != 0xFFFF && !host.ui_busy() {
                    let flag = if g.interface_matches {
                        g.viewed = battle[0x662];
                        0
                    } else {
                        1
                    };
                    host.push_event(0x96, r16(battle, 0x662), g.player, flag);
                    g.dialog_open = 0;
                    battle[0x12EA] = 0;
                }
            }
        }
    }
    let low = ticks as u16;
    if low == 0 {
        return Ok(());
    }
    if battle[0x12EA] != 0 && !host.ui_busy() {
        g.dialog_open = 0;
        battle[0x12EA] = 0;
    }
    let mut bx = low;
    if battle[0x5C] != 0 {
        if g.dialog_open == 0 && battle[0x12E9] != 0 {
            show_pending(battle, g, host);
        }
        let timer = r16(battle, 0x15C);
        if timer <= bx {
            w16(battle, 0x15C, 0);
            host.destroy();
            return Ok(());
        }
        w16(battle, 0x15C, (u32::from(timer).wrapping_sub(ticks)) as u16);
    } else {
        let pending = battle[0x12E9] != 0;
        if pending && g.dialog_open == 0 {
            show_pending(battle, g, host);
        }
        let count = usize::from(battle[0x558]);
        if !pending {
            let waiting = (0..count).any(|i| r16(battle, 0x15C + 2 * i) > 0);
            if waiting {
                let delta = if bx > 5 { 5u32 } else { ticks };
                ticks = delta;
                for i in 0..count {
                    let timer = r16(battle, 0x15C + 2 * i);
                    if g.player == u32::from(r16(battle, 0x35A + 2 * i))
                        && battle[0x5D + i] == 0
                        && timer != 0
                        && timer != 0xFFFF
                    {
                        if timer <= ticks as u16 {
                            w16(battle, 0x15C + 2 * i, 0xFFFE);
                            if host.ui_busy() {
                                host.ui_release();
                            }
                        } else {
                            w16(battle, 0x15C + 2 * i, (u32::from(timer).wrapping_sub(delta)) as u16);
                        }
                    }
                }
                return Ok(());
            }
        }
        // Velocidad de simulación: todos agresivos (modo directo) o interactiva.
        let mut all_aggressive = true;
        for i in 0..count {
            if battle[0x5D + i] == 0 {
                if g.player == u32::from(r16(battle, 0x35A + 2 * i)) && u16::from(g.viewed) != r16(battle, 0x662) {
                    host.push_event(0x96, r16(battle, 0x662), g.player, 1);
                }
                all_aggressive = false;
            }
        }
        if all_aggressive {
            if g.hotseat {
                ticks = 0xFFF0;
            }
            let total = ticks.wrapping_add(u32::from(battle[0x130F]));
            battle[0xAC8] = 1;
            ticks = total;
            let rest = (total & 0xF) as u8;
            battle[0x130F] = rest;
            ticks = u32::from((total as u16) >> 4);
        } else {
            if (ticks as u16) > 5 {
                ticks = 5;
            }
            battle[0xAC8] = 0;
        }
        bx = ticks as u16;
    }
    if bx == 0 {
        return Ok(());
    }
    for _ in 0..bx {
        side_statistics(battle, ships, worlds, next, ctx)?;
        let counter = battle[0xA8C].wrapping_add(1);
        battle[0xA8C] = counter;
        if u32::from(counter) * 40 >= 1000 {
            battle[0xA8C] = 0;
        }
        if battle[0x5C] == 1 {
            let timer = r16(battle, 0x15C);
            if timer == 0 || timer > 0xF000 {
                let mut survivors = false;
                if r16(battle, 0x65A) > 0 {
                    let mut id = r16(battle, 0x65C);
                    while id != 0xFFFF {
                        let ship = ships.get(usize::from(id)).ok_or("barco local ausente")?;
                        if ship[0x12A] & 0x26 == 0 {
                            let (_, at) = lookup(worlds, ship)?;
                            if r16(worlds, at + 0x40) > 0 {
                                survivors = true;
                            }
                        }
                        id = *next.get(usize::from(id)).ok_or("lista de barcos truncada")?;
                    }
                }
                return host.finish(battle, ships, worlds, survivors);
            }
        }
        update_wind(battle, g.viewed, false);
        let mut flags = SideFlags {
            all_flagged: [true; 2],
            all_surrendered: [true; 2],
            ..Default::default()
        };
        step_side(0, battle, ships, worlds, next, near, far, g, ctx, host, &mut flags)?;
        step_side(1, battle, ships, worlds, next, near, far, g, ctx, host, &mut flags)?;
        for index in 0..r16(battle, 0xACC) {
            if host.projectile_present(index) {
                host.advance_projectile(battle, ships, worlds, index);
            }
        }
        if flags.carrying[0] && flags.spare_room[1] {
            flags.all_flagged[0] = false;
        }
        if flags.carrying[1] && flags.spare_room[0] {
            flags.all_flagged[1] = false;
        }
        let (lost0, lost1) = (flags.all_flagged[0], flags.all_flagged[1]);
        if !lost0 && !lost1 {
            let first = !flags.spare_room[1] && flags.all_surrendered[0] && battle[0x5E] != 0;
            let second = !flags.spare_room[0] && flags.all_surrendered[1] && battle[0x5D] != 0;
            if first {
                if !ctx.virtual_10 && battle[0x5C] == 0 {
                    flee_side(battle, ships, next, 1)?;
                }
            } else if second && !ctx.virtual_10 && battle[0x5C] == 0 {
                flee_side(battle, ships, next, 0)?;
            }
        }
        let over = lost0 || lost1;
        if battle[0x5C] == 0 && over {
            w16(battle, 0x15C, 0xFFFE);
        }
        battle[0x5C] = u8::from(over);
        {
            let board = BoardCtx {
                captains: ctx.captains,
                virtual_10: ctx.virtual_10,
            };
            let view = u16::from(g.viewed) == r16(battle, 0x662);
            host.collision_pairs(&mut |slots, a, b, effects| {
                dispatch_pair(
                    &mut battle[..],
                    slots,
                    &mut ships[..],
                    &mut worlds[..],
                    &board,
                    a,
                    b,
                    view,
                    effects,
                )
            })?;
        }
        if battle[0x12E0] != 0 && r32(battle, 0x66C) != 0 {
            return Err("rama visual de 0x00608550 no modelada");
        }
    }
    Ok(())
}
