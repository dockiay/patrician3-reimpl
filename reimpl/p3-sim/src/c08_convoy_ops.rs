//! C08: operaciones de convoy del jugador sobre la flota viva de navegación: `0x15` (barco sale como convoy nuevo,
//! `0x00509B90`), `0x16` (disolver, `0x0050A580`), `0x17` (sacar un barco del convoy, `0x00539270`) y `0x18` (unir un
//! barco atracado a un convoy, `0x00509D10`). Los portes de `p3_core::ships` trabajan sobre los bytes de la instantánea
//! de navegación; después se publican las vistas con `publish_navigation_delta` (barcos, convoyes, cabezas de lista,
//! capitanes) y los comerciantes que cambió el reembolso de una disolución.
//! Límites: convoyes de estado 6 y 9/0xA con tareas (admisión hanseática), rutas propias que haya que liberar, tablas
//! de convoyes llenas (`0x00509AE0`) y avisos de reembolso a otros comerciantes devuelven un error y no cambian nada.
use crate::Sim;
use p3_core::pump::Operation;
use p3_core::ships::convoy_join::{join_convoy, JoinHost};
use p3_core::ships::remove::{RemoveWorld, MERCHANT_SIZE, TOWN_SIZE};
use p3_core::ships::tick::{ConvoyHost, TickWorld};

const SHIP_SIZE: usize = 0x180;
const CONVOY_SIZE: usize = 0x3c;

/// Anfitrión de las llamadas externas de estas operaciones: todo lo no conectado queda como error.
#[derive(Default)]
struct Host {
    error: Option<String>,
}

impl Host {
    fn fail(&mut self, what: &str) {
        self.error.get_or_insert_with(|| format!("efecto de convoy no conectado: {what}"));
    }
}

fn clear_route(w: &mut TickWorld, ship: u16, host: &mut Host) {
    let at = usize::from(ship) * SHIP_SIZE;
    if w.remove.ships[at + 0x24..at + 0x28] != [0; 4] {
        host.fail("el barco tiene ruta propia");
        return;
    }
    struct NoRoute;
    impl p3_core::ships::route::RouteHost for NoRoute {
        fn points_of(&mut self, _: u32) -> u32 {
            0
        }
        fn free(&mut self, _: u32) {}
        fn ui_route_cleared(&mut self, _: u16) {}
    }
    p3_core::ships::route::clear_route(&mut w.remove.ships[at..at + SHIP_SIZE], ship, w.player, false, &mut NoRoute);
}

impl ConvoyHost for Host {
    fn hook(&mut self, name: &'static str, _args: &[u32]) -> u32 {
        self.fail(name);
        0
    }
    fn read(&mut self, _: u32, _: u8) -> u32 {
        self.fail("lectura de ruta");
        0
    }
    fn insert_order(&mut self, w: &mut TickWorld, ship: u16, after: u16) {
        p3_core::ships::order::insert_from_tick(w, ship, after);
    }
    fn remove_order_with_world(&mut self, w: &mut TickWorld, ship: u16) {
        p3_core::ships::order::remove_from_tick(w, ship);
    }
    fn clear_ship_route(&mut self, w: &mut TickWorld, ship: u16) {
        clear_route(w, ship, self);
    }
}

impl JoinHost for Host {
    fn clear_route(&mut self, w: &mut TickWorld, ship: u16) {
        clear_route(w, ship, self);
    }
    fn ui_ship_joined(&mut self, _ship: u16) {}
    fn hanse_join_allowed(&mut self, _w: &mut TickWorld, _convoy: u16, _ship: u16) -> bool {
        self.fail("admisión hanseática (convoy en estado 9)");
        false
    }
    fn task_record(&mut self, _task: u16) -> Option<&mut [u8]> {
        self.fail("tarea del convoy");
        None
    }
    fn merchant_joined(&mut self, _owner: u8) {}
}

impl p3_core::ships::ai_depart::AiDepartHost for Host {
    fn clear_route(&mut self, w: &mut TickWorld, ship: u16) {
        clear_route(w, ship, self);
    }
}

impl Sim {
    /// `0x15..=0x18`; ver el módulo. Todo o nada: ante un error la flota queda como estaba.
    pub(crate) fn c08_convoy_operation(&mut self, op: Operation) -> Result<(), String> {
        self.c08_merchant_views_synced()?;
        let nav = self.host.navigation.as_ref().ok_or("navegación ausente para el convoy")?;
        let before = nav.snapshot.clone();
        let mut staged = nav.clone();
        let player = nav.player;
        let merchant_count = self.economy.world.merchant_count;
        let mut merchants = self.rep_merchants.clone();
        let mut merchant_bytes = merchants.iter().take(usize::from(merchant_count)).flat_map(|m| m.bytes.iter().copied()).collect::<Vec<_>>();
        let mut town_bytes = self.economy.world.towns.iter().flat_map(|t| t.0.iter().copied()).collect::<Vec<_>>();
        let mut host = Host::default();
        let mut disband_merchants = None;
        let tick = self.idle.clock.tick;
        let (kind, a, b, c, d) = (op.0[0] & 0xff, op.0[1], op.0[2], op.0[3], op.0[4]);
        {
            let snapshot = &mut staged.snapshot;
            let count = usize::from(u16::from_le_bytes([snapshot.manager[0xf4], snapshot.manager[0xf5]]));
            let ncv = usize::from(u16::from_le_bytes([snapshot.manager[0xf6], snapshot.manager[0xf7]]));
            if snapshot.ships.len() != count * SHIP_SIZE || snapshot.convoys.len() != ncv * CONVOY_SIZE {
                return Err("flota de navegación truncada".into());
            }
            match kind {
                0x16 | 0x17 => {
                    // 0x17 con el líder disuelve el convoy, si no saca al barco
                    let ship = a as usize;
                    let convoy = if kind == 0x16 {
                        a as usize
                    } else {
                        if ship >= count {
                            return Ok(());
                        }
                        usize::from(u16::from_le_bytes([snapshot.ships[ship * SHIP_SIZE + 8], snapshot.ships[ship * SHIP_SIZE + 9]]))
                    };
                    if convoy >= ncv {
                        return Ok(());
                    }
                    let leader = u16::from_le_bytes([snapshot.convoys[convoy * CONVOY_SIZE + 0x10], snapshot.convoys[convoy * CONVOY_SIZE + 0x11]]);
                    if kind == 0x16 || usize::from(leader) == ship {
                        use p3_core::economy::captain_operations::RetirementHost;
                        let mut pool = p3_core::ships::captains::CaptainPool {
                            entries: snapshot.captains.clone(),
                            capacity: u16::from_le_bytes([snapshot.manager[0xf2], snapshot.manager[0xf3]]),
                            free_head: u16::from_le_bytes([snapshot.manager[0xe0], snapshot.manager[0xe1]]),
                        };
                        let mut raw = merchant_bytes.clone();
                        for (i, m) in self.economy.world.merchants.iter().take(usize::from(merchant_count)).enumerate() {
                            raw[i * MERCHANT_SIZE..i * MERCHANT_SIZE + 4].copy_from_slice(&m.gold.to_le_bytes());
                            raw[i * MERCHANT_SIZE + 0x4c8..i * MERCHANT_SIZE + 0x4cc].copy_from_slice(&m.income.to_le_bytes());
                        }
                        let mut disband = crate::RetirementDisband {
                            manager: snapshot.manager.clone(),
                            merchants: raw,
                            merchant_count,
                            tick,
                            player,
                            distances: self.host.navigation_ai.clone(),
                            notices: Vec::new(),
                        };
                        let mut towns = self.economy.world.towns.clone();
                        disband.disband_convoy(convoy as u16, &mut snapshot.ships, &mut snapshot.convoys, &mut towns, &mut pool)?;
                        if !disband.notices.is_empty() {
                            return Err("el convoy llevaba barcos ajenos: los avisos de reembolso no están conectados".into());
                        }
                        snapshot.manager = disband.manager;
                        snapshot.captains = pool.entries;
                        snapshot.manager[0xe0..0xe2].copy_from_slice(&pool.free_head.to_le_bytes());
                        disband_merchants = Some(disband.merchants);
                        for (t, src) in self.economy.world.towns.iter().zip(towns.iter()) {
                            if t.0 != src.0 {
                                return Err("la disolución cambia una ciudad: no conectado".into());
                            }
                        }
                    } else {
                        let mut remove = RemoveWorld {
                            manager: &mut snapshot.manager,
                            ships: &mut snapshot.ships,
                            captains: &mut snapshot.captains,
                            convoys: &mut snapshot.convoys,
                            merchants: &mut merchant_bytes,
                            towns: &mut town_bytes,
                            pool_limit: 0,
                            merchant_count,
                            town_count: self.economy.world.towns.len() as u32,
                            ui_a: false,
                            ui_b: false,
                        };
                        let mut world = TickWorld { remove: &mut remove, time: tick, player };
                        p3_core::ships::convoy::remove_from_convoy(&mut world, ship as u16, &mut host, 0);
                    }
                }
                0x15 | 0x18 => {
                    let mut remove = RemoveWorld {
                        manager: &mut snapshot.manager,
                        ships: &mut snapshot.ships,
                        captains: &mut snapshot.captains,
                        convoys: &mut snapshot.convoys,
                        merchants: &mut merchant_bytes,
                        towns: &mut town_bytes,
                        pool_limit: 0,
                        merchant_count,
                        town_count: self.economy.world.towns.len() as u32,
                        ui_a: false,
                        ui_b: false,
                    };
                    let mut world = TickWorld { remove: &mut remove, time: tick, player };
                    if kind == 0x18 {
                        join_convoy(&mut world, a, b, &mut host).map_err(|e| format!("unir al convoy: {e:?}"))?;
                    } else {
                        let phase = c;
                        p3_core::ships::ai_depart::create_convoy(&mut world, a as u16, b != 0, phase, d, &mut host)
                            .map_err(|e| format!("convoy nuevo: {e:?}"))?;
                    }
                }
                _ => unreachable!(),
            }
        }
        if let Some(e) = host.error {
            return Err(e);
        }
        if let Some(raw) = disband_merchants {
            merchant_bytes = raw;
        }
        let changed_captains = (before.captains != staged.snapshot.captains).then(|| staged.snapshot.captains.clone());
        crate::navigation::refresh_lists(&mut staged.snapshot)?;
        for (dst, src) in merchants.iter_mut().zip(merchant_bytes.chunks_exact(MERCHANT_SIZE)) {
            dst.bytes.copy_from_slice(src);
        }
        let _ = TOWN_SIZE;
        // los capitanes se publican aparte (fábrica y bandas piratas); la instantánea conserva los viejos hasta entonces
        if changed_captains.is_some() {
            staged.snapshot.captains = before.captains.clone();
        }
        self.c08_publish_merchants(merchants);
        self.host.navigation = Some(staged);
        self.host.publish_navigation_delta(&before, &mut self.economy.world, &mut self.economy.cargo_ships);
        if let Some(after) = changed_captains {
            self.c08_publish_captains(&before.captains, &after)?;
        }
        Ok(())
    }
}
