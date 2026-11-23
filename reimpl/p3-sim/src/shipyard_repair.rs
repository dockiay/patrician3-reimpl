//! Astillero: mejora y reparación de un barco atracado (operación 0x0C → `start_repair`, 0x00508660) y su presupuesto
//! (`town_quote_ship_upgrade`, 0x0052B670), sobre los datos vivos del jugador.
use crate::{navigation, Sim};
use p3_core::economy::world::TownHost;
use p3_core::ai::reputation::Merchant as RepMerchant;
use p3_core::economy::prices::{buy_price, Market};
use p3_core::shipyard::{Merchant, Quote, QuoteCtx, QuoteHost, RepairHost, ShipManager, SHIP_SIZE};
use p3_core::town::Town;

fn u16_at(b: &[u8], o: usize) -> u16 {
    u16::from_le_bytes([b[o], b[o + 1]])
}

/// Existencias de las mercancías 11, 15 y 12 de la oficina (`+4+4·w`).
const STOCK_WARES: [usize; 3] = [11, 15, 12];

fn merchant_view(m: &RepMerchant) -> Merchant {
    Merchant { gold: m.i32(0), spent: m.i32(0x4c4), first_ship: u16_at(&m.bytes, 0xe) }
}

struct QuoteView<'a> {
    town: &'a Town,
    merchants: &'a [RepMerchant],
    office: Option<(u16, u8, [i32; 3])>,
}
impl QuoteHost for QuoteView<'_> {
    fn office_stock(&mut self, owner: u16, town: u16) -> Option<[i32; 3]> {
        self.office.filter(|o| o.0 == owner && u16::from(o.1) == town).map(|o| o.2)
    }
    fn merchant(&mut self, owner: u8) -> Merchant {
        self.merchants.get(usize::from(owner)).map_or_else(Merchant::default, merchant_view)
    }
    fn buy_price(&mut self, ware: u32, _town: u32, amount: i32) -> i32 {
        let w = ware as usize;
        let t = self.town;
        buy_price(w, &Market { trading: t.more_flags() & 0x100 != 0, stock: t.stock(w), thresholds: std::array::from_fn(|i| t.threshold(w, i)), marked_ware: t.u8(0x6f0) }, amount)
    }
}

struct RepairView<'a> {
    towns: &'a mut Vec<Town>,
    merchants: &'a mut Vec<RepMerchant>,
    manager: ShipManager,
    day: u8,
    count: u16,
    office: Option<(u16, u8, [i32; 3])>,
    office_after: Option<[i32; 3]>,
    merchant_buf: Merchant,
    merchant_owner: Option<u8>,
    reputation: Option<(u8, i32)>,
}
impl RepairHost for RepairView<'_> {
    fn town(&mut self, index: u8) -> Option<&mut Town> {
        self.towns.get_mut(usize::from(index))
    }
    fn quote(&mut self, town: u8, ship: u32, request: u32) -> Option<Quote> {
        let t = self.towns.get(usize::from(town))?.clone();
        let mut view = QuoteView { town: &t, merchants: self.merchants, office: self.office };
        let mut cx = QuoteCtx { merchant_count: self.count, day: self.day, host: &mut view };
        let (ok, quote) = self.manager.quote_ship_upgrade(&t, ship, request, &mut cx);
        if ok { quote } else { None }
    }
    fn office_stock(&mut self, owner: u8, town: u8) -> Option<&mut [i32; 3]> {
        let (who, at, stock) = self.office?;
        if who != u16::from(owner) || at != town {
            return None;
        }
        Some(self.office_after.get_or_insert(stock))
    }
    fn merchant(&mut self, owner: u8) -> &mut Merchant {
        if self.merchant_owner != Some(owner) {
            self.merchant_buf = self.merchants.get(usize::from(owner)).map_or_else(Merchant::default, merchant_view);
            self.merchant_owner = Some(owner);
        }
        &mut self.merchant_buf
    }
    fn upgrade_levels(&mut self, _ship: u16, _request: u32) {}
    fn upgrade_image(&mut self, ship: &mut [u8], request: u32) {
        let _ = p3_core::ships::factory::upgrade_level(ship, request.min(255) as u8);
    }
    fn hull_work_reputation(&mut self, town: u8, remaining: i32) {
        self.reputation = Some((town, remaining));
    }
}

impl Sim {
    /// Gestor de barcos vivo (navegación o fábrica) y si procede de la navegación.
    fn yard_manager(&self) -> Result<(ShipManager, bool), String> {
        if let Some(nav) = self.host.navigation.as_ref() {
            let snap = &nav.snapshot;
            if snap.manager.len() < 0xf8 {
                return Err("gestor naval truncado".into());
            }
            let m = ShipManager {
                ships: snap.ships.clone(),
                entries: snap.convoys.clone(),
                docked_head: u16_at(&snap.manager, 0xe8),
                free_entry_head: u16_at(&snap.manager, 0xec),
                ship_count: u16_at(&snap.manager, 0xf4),
                entry_count: u16_at(&snap.manager, 0xf6),
            };
            return Ok((m, true));
        }
        let f = self.host.expedition_factory.as_ref().ok_or("flota de astillero ausente")?;
        let count = u16::try_from(f.ships.len()).map_err(|_| "demasiados barcos")?;
        if f.ships.iter().any(|s| s.len() != SHIP_SIZE) {
            return Err("imágenes de barco truncadas".into());
        }
        let entries = u16::try_from(f.convoys.len()).map_err(|_| "demasiadas entradas navales")?;
        Ok((ShipManager { ships: f.ships.concat(), entries: f.convoys.concat(), docked_head: self.host.expedition_docked_head, free_entry_head: f.convoy_head, ship_count: count, entry_count: entries }, false))
    }

    /// Oficina de `owner` en la ciudad `town`: índice y existencias de las mercancías 11, 15 y 12.
    fn yard_office(&self, owner: u16, town: usize) -> Result<Option<(usize, [i32; 3])>, String> {
        let world = &self.economy.world;
        let Some(t) = world.towns.get(town) else { return Ok(None) };
        let mut at = usize::from(t.u16(0x784));
        let mut seen = vec![false; world.offices.len()];
        while at < world.offices.len() {
            if std::mem::replace(&mut seen[at], true) {
                return Err("cadena urbana de oficinas cíclica".into());
            }
            let stock = self.economy.offices.get(at).ok_or("existencias de oficina ausentes")?;
            if stock.owner == owner {
                return Ok(Some((at, STOCK_WARES.map(|w| stock.stock[w]))));
            }
            at = usize::from(world.offices[at].next);
        }
        Ok(None)
    }

    /// Convoyes de `owner` en el puerto `city` que aún pueden entrar en reparación (`+0x12 < 4`, operación 0x1D): `(entrada, estado, coste de reparar todos sus barcos)`.
    pub fn shipyard_convoys(&self, city: usize, owner: usize) -> Result<Vec<(usize, u16, i32)>, String> {
        let (m, _) = self.yard_manager()?;
        let Some(t) = self.economy.world.towns.get(city) else { return Ok(Vec::new()) };
        let mut out = Vec::new();
        for e in 0..usize::from(m.entry_count) {
            let b = &m.entries[e * 0x3c..(e + 1) * 0x3c];
            if usize::from(b[0]) != owner || usize::from(u16_at(b, 0x1c)) != city || u16_at(b, 0x12) >= 4 {
                continue;
            }
            let (mut at, mut cost, mut steps) = (u16_at(b, 0xa), 0i32, 0);
            if at >= m.ship_count {
                continue;
            }
            while at < m.ship_count && steps <= m.ship_count {
                if m.s32(at, 0x14) == 0 {
                    cost = i32::MIN;
                    break;
                }
                cost = cost.wrapping_add(m.calc_ship_repair_cost(t, at));
                at = m.s16(at, 6);
                steps += 1;
            }
            if cost != i32::MIN {
                out.push((e, u16_at(b, 0x12), cost));
            }
        }
        Ok(out)
    }

    fn run_human_ship_order(&mut self, city: usize, owner: usize, kind: u8, dry: bool) -> Result<Option<(u16, i32)>, String> {
        self.c08_merchant_views_synced()?;
        self.host.human_ship_order = Some((city, owner, kind, dry));
        self.host.human_ship_order_result = None;
        let tick = self.idle.clock.tick;
        let images_before = self.host.scheduled_office_images.clone();
        {
            let mut bridge = p3_state::office::OfficeTownHost {
                offices: &mut self.persisted,
                merchants: &mut self.rep_merchants,
                businesses: &mut self.businesses,
                captain_wages: &mut self.wages,
                difficulty: self.commercial_sell_factor,
                other: &mut self.host,
                completed: Vec::new(),
                errors: Vec::new(),
            };
            bridge.dispatch_scheduled_tasks_fleet(&mut self.economy.world, &mut self.economy.merchants, &[], &mut self.economy.offices, &mut self.economy.cargo_ships, 2.0, tick);
        }
        // Sin tareas vencidas el puente vacía las imágenes de oficina; el pedido no las usa, así que se conservan las de antes (WIN-01).
        if self.host.scheduled_office_images.as_ref().is_none_or(|v| v.is_empty()) {
            self.host.scheduled_office_images = images_before;
        }
        self.host.human_ship_order_result.take().unwrap_or_else(|| Err("pedido naval sin resultado".into()))
    }

    /// Coste de mandar construir un barco de clase `kind` (0..3) para `owner` en `city` (`0x0052A360` sin aplicarlo), o `None` si el astillero lo rechaza.
    pub fn shipyard_order_quote(&mut self, city: usize, owner: usize, kind: u8) -> Result<Option<i32>, String> {
        Ok(self.run_human_ship_order(city, owner, kind, true)?.map(|o| o.1))
    }

    /// Operación 0x2C (`Town::handle_build_ship`, `0x0052A360`): encarga un barco de clase `kind` en `city`; reserva materiales, cobra y lo pone en la cola de obra.
    /// Devuelve `(barco, coste)` o `None` si el astillero lo rechaza.
    pub fn shipyard_order_ship(&mut self, city: usize, owner: usize, kind: u8) -> Result<Option<(u16, i32)>, String> {
        self.run_human_ship_order(city, owner, kind, false)
    }

    /// Si `ship` está en la lista de atracados (la única de la que `start_repair` puede sacarlo).
    pub fn shipyard_is_docked(&self, ship: u32) -> Result<bool, String> {
        let (m, _) = self.yard_manager()?;
        let mut cur = m.docked_head;
        let mut steps = 0;
        while cur < m.ship_count && steps <= m.ship_count {
            if u32::from(cur) == ship {
                return Ok(true);
            }
            cur = m.s16(cur, 6);
            steps += 1;
        }
        Ok(false)
    }

    /// Presupuesto de mejorar `request` niveles el barco `ship` (17 palabras del original) y si el dueño puede pagarlo.
    pub fn shipyard_quote(&self, ship: u32, request: u32) -> Result<Option<(bool, Quote)>, String> {
        let (manager, _) = self.yard_manager()?;
        if ship >= u32::from(manager.ship_count) {
            return Ok(None);
        }
        let i = ship as u16;
        let town = usize::from(manager.s8(i, 0x39));
        let owner = u16::from(manager.s8(i, 0));
        let Some(t) = self.economy.world.towns.get(town) else { return Ok(None) };
        let office = self.yard_office(owner, town)?.map(|(_, s)| (owner, town as u8, s));
        let mut view = QuoteView { town: t, merchants: &self.rep_merchants, office };
        let mut cx = QuoteCtx { merchant_count: self.economy.world.merchant_count, day: self.idle.clock.tick as u8, host: &mut view };
        let (ok, quote) = manager.quote_ship_upgrade(t, ship, request, &mut cx);
        Ok(quote.map(|q| (ok, q)))
    }

    /// Operación 0x0C: `start_repair(ship, request)` sobre el estado vivo; solo publica si termina bien.
    pub fn shipyard_start_repair(&mut self, ship: u32, request: u32) -> Result<(), String> {
        let count = self.c08_merchant_views_synced()?;
        let (before, from_nav) = self.yard_manager()?;
        if ship >= u32::from(before.ship_count) {
            return Ok(());
        }
        let i = ship as u16;
        let town = usize::from(before.s8(i, 0x39));
        let owner = before.s8(i, 0);
        let office = self.yard_office(u16::from(owner), town)?;
        let mut towns = self.economy.world.towns.clone();
        let mut merchants = self.rep_merchants[..count].to_vec();
        let mut view = RepairView {
            towns: &mut towns,
            merchants: &mut merchants,
            manager: before.clone(),
            day: self.idle.clock.tick as u8,
            count: self.economy.world.merchant_count,
            office: office.map(|(_, s)| (u16::from(owner), town as u8, s)),
            office_after: None,
            merchant_buf: Merchant::default(),
            merchant_owner: None,
            reputation: None,
        };
        let mut manager = before.clone();
        manager.start_repair(ship, request, &mut view);
        if manager == before {
            return Ok(());
        }
        let (office_after, buf, reputation) = (view.office_after, view.merchant_buf, view.reputation);
        if let Some((t, remaining)) = reputation {
            if let Some(m) = merchants.get_mut(usize::from(owner)) {
                m.add_social_reputation_ship_hull_work(u32::from(t), remaining as u32);
            }
        }
        if let Some(m) = merchants.get_mut(usize::from(owner)) {
            m.set_i32(0, buf.gold);
            m.set_i32(0x4c4, buf.spent);
        }
        self.yard_publish(&before, manager, from_nav)?;
        self.economy.world.towns = towns;
        self.c08_publish_merchants(merchants);
        if let (Some((at, _)), Some(stock)) = (office, office_after) {
            for (w, v) in STOCK_WARES.iter().zip(stock) {
                self.persisted[at].stock.stock[*w] = v;
                self.economy.offices[at].stock[*w] = v;
                let bytes = v.to_le_bytes();
                if let Some(images) = self.host.scheduled_office_images.as_mut() {
                    images[at][4 + 4 * w..8 + 4 * w].copy_from_slice(&bytes);
                }
                if let Some(pool) = self.host.office_pool.as_mut() {
                    pool.records[at][4 + 4 * w..8 + 4 * w].copy_from_slice(&bytes);
                }
            }
        }
        Ok(())
    }

    /// Escribe en todas las vistas navales los barcos y la lista de atracados que cambió la operación.
    fn yard_publish(&mut self, before: &ShipManager, after: ShipManager, from_nav: bool) -> Result<(), String> {
        let n = usize::from(after.ship_count);
        let changed: Vec<usize> = (0..n).filter(|&k| before.ships[k * SHIP_SIZE..(k + 1) * SHIP_SIZE] != after.ships[k * SHIP_SIZE..(k + 1) * SHIP_SIZE]).collect();
        if from_nav {
            let nav = self.host.navigation.as_mut().ok_or("navegación ausente")?;
            let mut staged = nav.clone();
            staged.snapshot.ships = after.ships.clone();
            staged.snapshot.convoys = after.entries.clone();
            staged.snapshot.manager[0xe8..0xea].copy_from_slice(&after.docked_head.to_le_bytes());
            navigation::refresh_lists(&mut staged.snapshot)?;
            self.host.navigation = Some(staged);
        } else if let Some(f) = self.host.expedition_factory.as_mut() {
            for (dst, src) in f.ships.iter_mut().zip(after.ships.chunks_exact(SHIP_SIZE)) {
                dst.copy_from_slice(src);
            }
        }
        self.host.expedition_docked_head = after.docked_head;
        for &k in &changed {
            let image = &after.ships[k * SHIP_SIZE..(k + 1) * SHIP_SIZE];
            if let Some(b) = self.host.expedition_ship_images.get_mut(k).and_then(Option::as_mut) {
                if b.len() == SHIP_SIZE {
                    b.copy_from_slice(image);
                }
            }
            if let Some(c) = self.economy.cargo_ships.get_mut(k) {
                c.dirty = u16_at(image, 0x3c);
            }
            if !self.host.tavern_changed_ships.contains(&k) {
                self.host.tavern_changed_ships.push(k);
            }
        }
        Ok(())
    }
}
