//! C08: fundar una oficina comercial del jugador (`0x5D`, `Town::op_found_trading_office`, `0x0051B890`).
//!
//! La valoración y el cobro son el puerto con oráculo `Site::found_trading_office`: sobre una copia de la ciudad y del comerciante
//! valora la obra de tipo 0 (ladrillo, madera, hierro y el extra), descuenta lo que falta de las existencias de la ciudad, retira lo que
//! aportan los barcos atracados del comerciante (`0x004F5450`) y le carga el coste. El alta de la oficina es la del estado vivo
//! (`Sim::create_live_office`, la misma de la expansión de la IA: pool, cadenas de ciudad y comerciante, vistas, carta global `0x0D`).
//! Límites: la ventana original no se ha contrastado con captura; el pool de oficinas crece 0x20 entradas como el original.
use crate::office_take::{take_ships, ShipLink};
use crate::Sim;
use p3_core::buildings::evaluate::{EvalHost, EvalShip};
use p3_core::buildings::office::{FoundEnv, FoundHost};
use p3_core::buildings::Site;
use p3_core::economy::prices::{buy_price, Market};
use p3_core::pump::Operation;
use p3_core::town::Town;

const WARES: [usize; 4] = [19, 11, 12, 17];

struct Found<'a> {
    town: &'a Town,
    create: bool,
    takes: Vec<(u32, u32, i32)>,
}

impl EvalHost for Found<'_> {
    fn get_merchant(&mut self, _: u32) {}
    fn find_office(&mut self, _: u32, _: u32, flag: u32) -> Option<[i32; 4]> {
        // Sin oficina previa (se comprueba antes); con el indicador pide el alta.
        self.create |= flag != 0;
        None
    }
    fn buy_price(&mut self, ware: u32, _: u32, qty: u32) -> i32 {
        let t = self.town;
        let w = ware as usize;
        buy_price(w, &Market { trading: t.more_flags() & 0x100 != 0, stock: t.stock(w), thresholds: std::array::from_fn(|i| t.threshold(w, i)), marked_ware: t.u8(0x6f0) }, qty as i32)
    }
}

impl FoundHost for Found<'_> {
    fn take_wares_from_ships(&mut self, ware: u32, owner: u32, quantity: i32) {
        self.takes.push((ware, owner, quantity));
    }
}

/// Resultado de valorar la fundación sobre copias.
struct Plan {
    town: Town,
    gold: i32,
    income: i32,
    takes: Vec<(u32, u32, i32)>,
    links: Vec<ShipLink>,
    head: u16,
    cost: i32,
}

impl Sim {
    /// Valora la fundación para `merchant` en `city` sin cambiar nada: `None` si no hay nada que hacer (ya hay oficina o no se puede construir).
    fn found_office_plan(&self, merchant: u32, city: usize) -> Result<Option<Plan>, String> {
        let count = self.c08_merchant_views_synced()?;
        if merchant >= count as u32 || city >= self.economy.world.towns.len() {
            return Ok(None);
        }
        // Ya tiene oficina en la ciudad: la función no hace nada.
        let offices = self.economy.world.offices.len();
        let mut at = usize::from(self.economy.world.towns[city].u16(0x784));
        let mut steps = 0;
        while at < offices {
            if usize::from(self.economy.offices[at].owner) == merchant as usize {
                return Ok(None);
            }
            at = usize::from(self.economy.world.offices[at].next);
            steps += 1;
            if steps > offices {
                return Err("cadena urbana de oficinas cíclica".into());
            }
        }
        // Flota viva: barco, enlace y carga.
        let ship_count = self.economy.cargo_ships.len();
        if self.host.expedition_ship_images.len() != ship_count || self.host.navigation.as_ref().is_some_and(|n| n.snapshot.ships.len() != ship_count * 0x180) {
            return Err("flota incompleta".into());
        }
        let mut links = Vec::with_capacity(ship_count);
        for i in 0..ship_count {
            let image = match &self.host.navigation {
                Some(nav) => nav.snapshot.ships.get(i * 0x180..(i + 1) * 0x180),
                None => self.host.expedition_ship_images[i].as_deref(),
            };
            links.push(match image {
                Some(b) if b.len() >= 0x180 => ShipLink { next: u16::from_le_bytes([b[4], b[5]]), state: u16::from_le_bytes([b[0x134], b[0x135]]), convoy: u16::from_le_bytes([b[8], b[9]]) },
                Some(_) => return Err("imagen de barco truncada".into()),
                None => ShipLink { next: u16::MAX, state: u16::MAX, convoy: u16::MAX },
            });
        }
        let eval_ships: Vec<EvalShip> = self.economy.cargo_ships.iter().zip(&links).map(|(s, l)| EvalShip { town: s.town, next: l.next, state: l.state, wares: WARES.map(|w| s.stock[w]) }).collect();
        let me = &self.rep_merchants[merchant as usize];
        let (gold0, income0) = (me.i32(0), me.i32(0x4c0));
        let (mut gold, mut income) = (gold0, income0);
        let head = me.u16(0xe);
        let original = self.economy.world.towns[city].clone();
        let mut town = original.clone();
        let mut host = Found { town: &original, create: false, takes: Vec::new() };
        let mut env = FoundEnv { merchant_count: count as u16, ship_count: ship_count as u16, ship_head: head, mayor: original.u8(0x6f1), ships: &eval_ships, merchant_gold: &mut gold, merchant_income: &mut income };
        Site::found_trading_office(&mut town, merchant, &mut env, &mut host);
        let Found { create, takes, .. } = host;
        Ok(create.then_some(Plan { town, gold, income, takes, links, head, cost: gold0.wrapping_sub(gold) }))
    }

    /// Coste de fundar una oficina del comerciante en la ciudad; `None` si ya tiene o no se puede (faltan materiales o dinero).
    pub fn found_office_quote(&self, merchant: u32, city: usize) -> Result<Option<i32>, String> {
        Ok(self.found_office_plan(merchant, city)?.map(|p| p.cost))
    }

    pub(crate) fn c08_found_office(&mut self, op: Operation) -> Result<(), String> {
        let count = self.c08_merchant_views_synced()?;
        let merchant = op.0[1];
        let city = op.0[2] as usize;
        let Some(Plan { town, gold, income, takes, links, head, .. }) = self.found_office_plan(merchant, city)? else {
            return Ok(());
        };
        // Materiales de los barcos (sobre copia: si algo falla no cambia nada).
        let mut cargo = self.economy.cargo_ships.clone();
        let mut changed: Vec<usize> = Vec::new();
        for (ware, owner, quantity) in takes {
            if quantity <= 0 {
                continue;
            }
            let (_, touched) = take_ships(&mut cargo, &links, head, owner as u8, ware as usize, quantity)?;
            for i in touched {
                if !changed.contains(&i) {
                    changed.push(i);
                }
            }
        }
        // Alta de la oficina en las vistas vivas (pool, cadenas, comerciante y ciudad); lee las ciudades y comerciantes vivos.
        let tick = self.idle.clock.tick;
        self.create_live_office(merchant as u8, city as u8, tick)?;
        // Cobro y existencias de la ciudad: solo lo que escribe la función (las cuatro mercancías, el dinero y los ingresos del comerciante).
        for w in WARES {
            let slot = 4 + 4 * w;
            let v = town.u32(slot);
            self.economy.world.towns[city].set_u32(slot, v);
        }
        if !self.host.debt_changed_towns.contains(&city) {
            self.host.debt_changed_towns.push(city);
        }
        let mut merchants = self.rep_merchants[..count].to_vec();
        merchants[merchant as usize].set_i32(0, gold);
        merchants[merchant as usize].set_i32(0x4c0, income);
        self.c08_publish_merchants(merchants);
        // Barcos que aportaron material.
        for &i in &changed {
            let s = &cargo[i];
            let write = |image: &mut [u8]| {
                for w in 0..24 {
                    image[0x54 + 4 * w..0x58 + 4 * w].copy_from_slice(&s.stock[w].to_le_bytes());
                }
                image[0x3c..0x3e].copy_from_slice(&s.dirty.to_le_bytes());
                for (p, v) in [(0x114, s.cargo_value), (0x118, s.cargo_weight), (0x11c, s.equipment_weight), (0x120, s.gun_power)] {
                    image[p..p + 4].copy_from_slice(&v.to_le_bytes());
                }
            };
            if let Some(image) = self.host.expedition_ship_images.get_mut(i).and_then(Option::as_mut) {
                write(image);
            }
            if let Some(nav) = self.host.navigation.as_mut() {
                write(&mut nav.snapshot.ships[i * 0x180..(i + 1) * 0x180]);
            }
            if !self.host.debt_changed_ships.contains(&i) {
                self.host.debt_changed_ships.push(i);
            }
        }
        self.economy.cargo_ships = cargo;
        Ok(())
    }
}
