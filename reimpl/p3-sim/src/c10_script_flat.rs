//! C10: comandos de guion de misión sobre imagen plana (`p3_core::mission_script_flat`) dentro de la tarea 0x1B.
//!
//! Siembra el mundo con la disposición del puente C08 (`c08_flat`): cabecera de `0x006DE4A0` (solo los campos
//! que la simulación conoce), ciudades, comerciantes, oficinas, el gestor naval `0x006DD7A0` con barcos y
//! convoyes, el buzón `0x006DD730`, el planificador `0x006DD73C` y el `.rdata` del ejecutable inglés. Lo que no
//! se siembra no se inventa: un comando que lo lea falla (y el guion entero se descarta, como antes).
//!
//! Escrituras admitidas: ciudades, comerciantes, buzón (y cartas de `0x004D6530`), planificador
//! (`0x004D8CF0`), operaciones (`0x0054AA70`), y en barcos, convoyes y oficinas los campos que escriben los
//! comandos (lista [`SHIP_FIELDS`], banderas `+0x18` del convoy, existencias y `+0x2E8` de la oficina). El
//! llamante publica los barcos y convoyes en la flota de navegación (la tabla común que `publish_navigation_delta`
//! reparte a las demás vistas) y las oficinas como actualización de imagen completa. El gestor naval y la
//! cabecera del mundo siguen siendo de solo lectura; cualquier otro cambio se rechaza como error.
//!
//! Se siembran también, desde el guardado, la fecha de inicio (`GameWorld+0xC`), las opciones de partida
//! (`[0x006CC3E8]`, como las deja el cargador `0x00549F40`), la matriz de distancias (`GameWorld+0x6C`) y la
//! dificultad de precios (`0x006DE3D8+0x64`).
use crate::c08_flat::{self, Calls, MERCHANTS, MERCHANT_SIZE, OFFICES, OFFICE_SIZE, RECORDS, RECORDS_B, SPARE_RECORDS, TOWNS, TOWN_SIZE, WORLD};
use crate::RepMerchant;
use p3_core::mission_script_calls as calls;
use p3_core::flatmem::{Mem, SparseMem};
use p3_core::mission_script_flat::{ScriptCalls, SHIPS};
use p3_core::scheduler::Scheduler;
use p3_core::town::Town;

/// Barcos (`0x180` bytes) y convoyes (`0x3C`) del gestor naval.
const SHIPT: u32 = 0x5c00_0000;
const CONVOYS: u32 = 0x5d00_0000;
/// Punteros de las cinco bandas piratas (`SHIPS+0xC..+0x20`): la simulación no las tiene en memoria, así que
/// apuntan a direcciones sin sembrar y cualquier lectura falla en vez de ver «sin banda».
const BAND_SENTINEL: u32 = 0x7f00_0000;
/// Opciones de la partida y matriz de distancias entre ciudades.
const OPTIONS: u32 = 0x5e80_0000;
const MATRIX: u32 = 0x5e90_0000;
/// Objeto de precios (`0x006DE3D8`): `+0x64` es el factor de venta.
const PRICES: u32 = 0x006d_e3d8;
/// Campos del barco (`0x180` bytes) que escriben los comandos de guion: casco `+0x18`, marcas `+0x3C/+0x3D`,
/// modo `+0x3F`, tripulación `+0x40`, carga `+0x54..+0xB4`, valor/peso/equipo/armas `+0x114..+0x124`, banderas
/// `+0x136`, pasajeros `+0x156` y contador `+0x157`.
pub(crate) const SHIP_FIELDS: [(usize, usize); 9] =
    [(0x18, 0x1c), (0x3c, 0x3e), (0x3f, 0x40), (0x40, 0x42), (0x54, 0xb4), (0x114, 0x124), (0x136, 0x137), (0x156, 0x157), (0x157, 0x158)];
/// Campos de la oficina (`0x44C`) que escriben `0x48/0xEE`: existencias `+4..+0x64` y `+0x2E8`.
const OFFICE_FIELDS: [(usize, usize); 2] = [(4, 0x64), (0x2e8, 0x2ec)];
const SHIP_SIZE: usize = 0x180;
const CONVOY_SIZE: usize = 0x3c;
/// Ventana `.rdata` del ejecutable inglés (tablas por mercancía y ciudad).
pub(crate) const RDATA: (u32, u32) = (0x0066_a000, 0x0069_2000);

/// Lo que el guion necesita leer de la simulación.
pub(crate) struct ScriptWorld<'a> {
    pub towns: &'a [Town],
    pub merchants: &'a [RepMerchant],
    pub offices: Option<&'a [Vec<u8>]>,
    pub navigation: Option<&'a p3_save::ships::NavigationSnapshot>,
    pub pool: &'a [u8],
    pub scheduler: &'a Scheduler,
    pub tick: u32,
    pub day_of_year: u16,
    pub world_8a: u8,
    pub alderman_merchant: u8,
    pub alderman_town: u8,
    pub network_client: u32,
    pub local_merchant: u32,
    pub rdata: &'a [u8],
    /// Datos del guardado que la simulación conserva solo para los guiones (`None` si no se cargaron).
    pub start_time: Option<u32>,
    pub settings: Option<&'a [u8]>,
    /// Matriz de distancias `n·n` (`GameWorld+0x6C`).
    pub distances: Option<&'a [u8]>,
    /// Nivel de muralla de cada ciudad (`0x005252F0`).
    pub wall_levels: Option<Vec<u8>>,
    /// Factor de precios de venta (`0x006DE3D8+0x64`).
    pub sell_factor: f32,
    /// Estado de `rand` del CRT (`0x00639C03`); `None` si la simulación no lo conoce.
    pub rng: Option<p3_core::rng::CrtRand>,
    /// Registros de edificios de cada ciudad (`Town+0x75C`) para `0x00525B90`.
    pub sites: Option<&'a [p3_core::buildings::Site]>,
    /// Modo de operaciones `[0x006DF2F0]`.
    pub operations_mode: u32,
    /// Banco de nombres de barco y puerto de cada ciudad, para `0x0050E000`.
    pub ship_names: Option<&'a [Vec<u8>]>,
    pub ports: Option<Vec<(u16, u16)>>,
}

/// Estado publicado por el guion si termina sin error.
pub(crate) struct FlatOut {
    pub towns: Vec<Town>,
    pub changed_towns: Vec<usize>,
    pub merchants: Vec<RepMerchant>,
    pub pool: Vec<u8>,
    /// Planificador leído de memoria (sin la tarea en curso, que el llamante coloca).
    pub scheduler: Scheduler,
    pub operations: Vec<p3_core::pump::Operation>,
    /// Bytes cambiados de la tabla de barcos y de la de convoyes (desplazamiento en la tabla, valor nuevo). Se
    /// aplican byte a byte sobre la flota de navegación para no deshacer las reservas del intérprete estructurado.
    pub ship_writes: Vec<(usize, u8)>,
    pub convoy_writes: Vec<(usize, u8)>,
    /// Oficinas cambiadas: índice e imagen completa.
    pub offices: Vec<(usize, Vec<u8>)>,
    /// Estado de `rand` tras el guion, si alguna llamada lo avanzó.
    pub rng: Option<p3_core::rng::CrtRand>,
    /// Barcos creados por `0x0050E000`: imagen completa de toda la tabla y gestor tras el guion.
    pub created: Option<CreatedShips>,
}

/// Altas navales del guion (`0x0050E000`).
pub(crate) struct CreatedShips {
    pub ids: Vec<u16>,
    pub ships: Vec<u8>,
    pub manager: Vec<u8>,
}

/// Memoria sembrada que anota el primer acceso fuera de lo sembrado en vez de entrar en pánico (devuelve 0 y
/// descarta la escritura); el resultado del guion se rechaza después.
pub(crate) struct Checked {
    pub inner: SparseMem,
    pub error: Option<String>,
}

impl Mem for Checked {
    fn r8(&mut self, a: u32) -> u8 {
        match self.inner.get(a) {
            Some(b) => b,
            None => {
                self.error.get_or_insert_with(|| format!("guion plano: lectura de {a:#x} sin sembrar"));
                0
            }
        }
    }
    fn w8(&mut self, a: u32, v: u8) {
        match self.inner.get_mut(a) {
            Some(b) => *b = v,
            None => {
                self.error.get_or_insert_with(|| format!("guion plano: escritura en {a:#x} sin sembrar"));
            }
        }
    }
}

pub(crate) struct ScriptImage {
    pub mem: Checked,
    host: Host,
    pool_header: [u8; 12],
    before: Vec<(u32, Vec<u8>)>,
    towns: usize,
    merchants: usize,
    /// Barcos, convoyes y oficinas sembrados (escritura limitada a sus campos de guion).
    ships: Option<Vec<u8>>,
    convoys: Option<Vec<u8>>,
    offices: Vec<Vec<u8>>,
}

struct Host {
    calls: Calls,
    ops: Vec<[u32; 5]>,
    wall_levels: Option<Vec<u8>>,
    rng: Option<p3_core::rng::CrtRand>,
    rng_used: bool,
    /// Registros de edificios por ciudad (sin plano: `0x00525B90` no lo lee).
    sites: Option<Vec<p3_core::buildings::Site>>,
    ship_names: Option<Vec<Vec<u8>>>,
    ports: Option<Vec<(u16, u16)>>,
    created: Vec<u16>,
}

/// Memoria con una doble palabra local del llamante (la variable de salida de `0x00515750`).
struct WithLocal<'a> {
    inner: &'a mut dyn Mem,
    at: u32,
    value: [u8; 4],
}

impl Mem for WithLocal<'_> {
    fn r8(&mut self, a: u32) -> u8 {
        match a.wrapping_sub(self.at) {
            k @ 0..=3 => self.value[k as usize],
            _ => self.inner.r8(a),
        }
    }
    fn w8(&mut self, a: u32, v: u8) {
        match a.wrapping_sub(self.at) {
            k @ 0..=3 => self.value[k as usize] = v,
            _ => self.inner.w8(a, v),
        }
    }
}

/// `0x00515750` con índice 0xFFFF solo calcula la media; cualquier llamada a sus ganchos es un error.
struct NoLoadoutHost;

impl p3_core::economy::pirate_loadout::LoadoutHost for NoLoadoutHost {
    fn ship_517770(&mut self, _: u32, _: u32) {
        unreachable!("0x515750 con barco 0xFFFF no reinicia barcos")
    }
    fn ship_5181d0(&mut self, _: u32) {
        unreachable!("0x515750 con barco 0xFFFF no reinicia barcos")
    }
    fn ship_5182b0(&mut self, _: u32) -> u32 {
        unreachable!("0x515750 con barco 0xFFFF no reinicia barcos")
    }
    fn town_52bb00(&mut self, _: u32) -> u32 {
        unreachable!("0x515750 con barco 0xFFFF no reinicia barcos")
    }
    fn ship_51a4e0(&mut self, _: u32, _: u32, _: u32) {
        unreachable!("0x515750 con barco 0xFFFF no reinicia barcos")
    }
    fn ship_5185c0(&mut self, _: u32) -> u32 {
        unreachable!("0x515750 con barco 0xFFFF no reinicia barcos")
    }
}

/// Dirección ficticia de la variable local de `0x10` (fuera de todo lo sembrado).
const LOCAL_SLOT: u32 = 0x7ff0_0000;

fn town_index(addr: u32, towns: usize) -> Result<usize, String> {
    let off = addr.checked_sub(TOWNS).ok_or("ciudad del guion fuera de la tabla")?;
    let i = (off / TOWN_SIZE) as usize;
    if off % TOWN_SIZE != 0 || i >= towns {
        return Err(format!("ciudad del guion fuera de la tabla ({addr:#x})"));
    }
    Ok(i)
}

impl ScriptCalls for Host {
    fn call(&mut self, mem: &mut dyn Mem, addr: u32, ecx: u32, args: &[u32]) -> Result<u32, String> {
        match addr {
            // add_task, find_or_create_office (solo búsqueda) y add_message: puertos del puente C08.
            0x004d_8cf0 | 0x0053_08a0 | 0x004d_6530 | 0x0053_03c0 => {
                let r = self.calls.call(mem, addr, ecx, args);
                match self.calls.error.take() {
                    Some(e) => Err(e),
                    None => Ok(r),
                }
            }
            // enqueue_operation: el registro de cinco dwords va a la cola de operaciones.
            0x0054_aa70 => {
                let record: [u32; 5] = args.try_into().map_err(|_| "operación del guion mal formada")?;
                self.ops.push(record);
                Ok(1)
            }
            // Aviso en pantalla de `0xE8` (`0x00548CA0`): solo presentación.
            0x0054_8ca0 => Ok(0),
            // Puertos con paridad x86 (`p3_core::mission_script_calls`, `economy::letters`, `economy::prices`).
            0x0051_82b0 => Ok(calls::recompute_load(mem, ecx)),
            0x0051_8640 => Ok(u32::from(calls::ship_add_passengers(mem, ecx, args[0]))),
            0x0051_86d0 => {
                calls::ship_remove_passengers(mem, ecx, args[0]);
                Ok(0)
            }
            0x0053_3210 => {
                let [from, a, b, c, count, alt, ship]: [u32; 7] = args.try_into().map_err(|_| "0x533210 mal formada")?;
                calls::find_nearest_town_excluding(mem, ecx, from, &[a, b, c], count, alt, ship)
            }
            0x0053_1260 => Ok(calls::date_to_ticks(mem, args[0], args[1], args[2])),
            0x0051_beb0 => Ok(calls::town_scarcest_produced_ware(mem, ecx)),
            0x004d_7770 => Ok(p3_core::economy::letters::captain_message_status(mem, ecx, args[0], args[1])),
            0x0052_e1d0 => {
                use p3_core::economy::prices::{sell_price, Market};
                let (ware, town, amount) = (args[0] as usize, args[1], args[2] as i32);
                if ware >= 24 {
                    return Err(format!("0x52E1D0: mercancía {ware} fuera de rango"));
                }
                let t = mem.r32(0x006d_e508).wrapping_add(town.wrapping_mul(TOWN_SIZE));
                let market = Market {
                    trading: mem.r8(t + 0x76d) & 1 != 0,
                    stock: mem.r32(t + 4 + 4 * ware as u32) as i32,
                    thresholds: std::array::from_fn(|j| mem.r32(t + 0x4f0 + 0x10 * ware as u32 + 4 * j as u32) as i32),
                    marked_ware: mem.r8(t + 0x6f0),
                };
                let factor = f32::from_bits(mem.r32(ecx + 0x64));
                Ok(sell_price(ware, &market, amount, factor) as u32)
            }
            0x0052_52f0 => {
                let levels = self.wall_levels.as_ref().ok_or("nivel de muralla no disponible para el guion")?;
                Ok(u32::from(levels[town_index(ecx, levels.len())?]))
            }
            // `rand` del CRT: el mismo flujo que el resto de la simulación (`investigation_rng`).
            0x0063_9c03 => {
                let rng = self.rng.as_mut().ok_or("rand del guion sin estado CRT")?;
                self.rng_used = true;
                Ok(rng.rand() as u32)
            }
            // `ship_reset_loadout_for_pirate` (`0x00515750`) con barco 0xFFFF: media del armamento `+0x120` de los
            // barcos de todos los comerciantes, en la variable de salida que empieza en -1.
            0x0051_5750 => {
                let ship = *args.first().ok_or("0x515750 sin argumentos")?;
                if ship != 0xffff {
                    return Err(format!("0x515750 con barco {ship:#x}: el guion solo la llama con 0xFFFF"));
                }
                let mut local = WithLocal { inner: mem, at: LOCAL_SLOT, value: u32::MAX.to_le_bytes() };
                p3_core::economy::pirate_loadout::ship_reset_loadout_for_pirate(&mut local, &mut NoLoadoutHost, ship, LOCAL_SLOT);
                Ok(u32::from_le_bytes(local.value))
            }
            // `town_pick_random_building_position` (`0x00525B90`): `[semilla, x, y]`; el original pone las dos salidas
            // a 0xFF antes de recorrer, así que su valor previo no cuenta.
            0x0052_5b90 => {
                let [seed, _, _]: [u32; 3] = args.try_into().map_err(|_| "0x525B90 mal formada")?;
                let sites = self.sites.as_ref().ok_or("edificios de las ciudades no disponibles para el guion")?;
                let site = sites.get(town_index(ecx, sites.len())?).ok_or("edificios de la ciudad ausentes")?;
                let mut town = Town::default();
                for (k, b) in town.0.iter_mut().enumerate() {
                    *b = mem.r8(ecx + k as u32);
                }
                let count = mem.r16(WORLD + 8);
                let base = mem.r32(WORLD + 0x74);
                let offices = (0..u32::from(count))
                    .map(|i| {
                        let o = base.wrapping_add(i * OFFICE_SIZE);
                        p3_core::buildings::query::OfficeHeads { next: mem.r16(o + 0x2ca), buildings_head: mem.r16(o + 0x2ce), houses_head: mem.r16(o + 0x2d0) }
                    })
                    .collect::<Vec<_>>();
                if site.records.len() < 8 * usize::from(town.u16(0x776)) {
                    return Err("registros de edificios más cortos que Town+0x776".into());
                }
                let (x, y) = site.pick_random_building_position(&town, &offices, count, seed);
                Ok(u32::from(x) | u32::from(y) << 8)
            }
            // `create_docked_ship_for_merchant` (`0x0050E000`) de `0x67`: `[tipo, armamento mínimo, ciudad, comerciante]`.
            0x0050_e000 => {
                let [kind, minimum, town, who]: [u32; 4] = args.try_into().map_err(|_| "0x50E000 mal formada")?;
                if ecx != SHIPS {
                    return Err("0x50E000 fuera del gestor naval".into());
                }
                let names = self.ship_names.as_deref().ok_or("banco de nombres de barco no disponible para el guion")?;
                let ports = self.ports.as_deref().ok_or("puertos de las ciudades no disponibles para el guion")?;
                let res = calls::DockedShipResources { ship_names: names, ports };
                let (id, _) = calls::create_docked_ship_for_merchant(mem, ecx, kind, minimum, town, who, &res)?;
                self.created.push(id);
                Ok(u32::from(id))
            }
            _ => Err(format!("llamada {addr:#x} del guion no conectada en p3-sim")),
        }
    }
}

impl ScriptImage {
    pub(crate) fn seed(w: &ScriptWorld<'_>) -> Result<Self, String> {
        let count = usize::from(u16::try_from(w.merchants.len()).map_err(|_| "demasiados comerciantes")?);
        if w.merchants.iter().any(|m| m.bytes.len() < MERCHANT_SIZE as usize) {
            return Err("comerciantes incompletos para el guion plano".into());
        }
        if w.rdata.len() != (RDATA.1 - RDATA.0) as usize {
            return Err(".rdata del guion plano incompleto".into());
        }
        let mut mem = SparseMem::default();
        mem.seed(RDATA.0, w.rdata);
        let date = p3_core::calendar::Calendar::for_tick(w.tick);
        // Cabecera del mundo: solo los campos conocidos (la fecha de inicio `+0xC` si el guardado la dio).
        mem.seed(WORLD, &[date.day, date.month]);
        mem.seed(WORLD + 2, &date.year.to_le_bytes());
        mem.seed(WORLD + 4, &w.day_of_year.to_le_bytes());
        let offices = w.offices.map_or(0, <[_]>::len);
        mem.seed(WORLD + 8, &(offices as u16).to_le_bytes());
        mem.seed(WORLD + 0xa, &(count as u16).to_le_bytes());
        mem.seed(WORLD + 0x10, &(w.towns.len() as u32).to_le_bytes());
        mem.seed(WORLD + 0x14, &w.tick.to_le_bytes());
        mem.seed(WORLD + 0x68, &TOWNS.to_le_bytes());
        if w.offices.is_some() {
            mem.seed(WORLD + 0x74, &OFFICES.to_le_bytes());
        }
        mem.seed(WORLD + 0x78, &MERCHANTS.to_le_bytes());
        mem.seed(WORLD + 0x8a, &[w.world_8a]);
        mem.seed(WORLD + 0x8d, &[w.alderman_merchant, w.alderman_town]);
        mem.seed(0x006d_f31c, &w.network_client.to_le_bytes());
        mem.seed(0x006d_fc14, &w.local_merchant.to_le_bytes());
        let mut fixed = Vec::new();
        if let Some(start) = w.start_time {
            mem.seed(WORLD + 0xc, &start.to_le_bytes());
            fixed.push((WORLD + 0xc, 4));
        }
        if let Some(settings) = w.settings {
            mem.seed(OPTIONS, settings);
            mem.seed(p3_core::mission_script_flat::OPTIONS_PTR, &OPTIONS.to_le_bytes());
            fixed.push((OPTIONS, settings.len()));
            fixed.push((p3_core::mission_script_flat::OPTIONS_PTR, 4));
        }
        if let Some(d) = w.distances {
            if d.len() != w.towns.len() * w.towns.len() {
                return Err("matriz de distancias del guion de tamaño distinto al de las ciudades".into());
            }
            mem.seed(MATRIX, d);
            mem.seed(WORLD + 0x6c, &MATRIX.to_le_bytes());
            fixed.push((MATRIX, d.len()));
            fixed.push((WORLD + 0x6c, 4));
        }
        mem.seed(PRICES + 0x64, &w.sell_factor.to_le_bytes());
        fixed.push((PRICES + 0x64, 4));
        mem.seed(p3_core::mission_script_flat::OPS, &w.operations_mode.to_le_bytes());
        fixed.push((p3_core::mission_script_flat::OPS, 4));
        if let Some(names) = w.ship_names {
            mem.seed(0x006d_db78, &u16::try_from(names.len()).map_err(|_| "banco de nombres de barco excesivo")?.to_le_bytes());
            fixed.push((0x006d_db78, 2));
        }
        for (i, t) in w.towns.iter().enumerate() {
            mem.seed(TOWNS + TOWN_SIZE * i as u32, &t.0[..]);
        }
        for (i, m) in w.merchants.iter().enumerate() {
            mem.seed(MERCHANTS + MERCHANT_SIZE * i as u32, &m.bytes[..MERCHANT_SIZE as usize]);
        }
        let mut before = Vec::new();
        let mut offices_seeded = Vec::new();
        if let Some(images) = w.offices {
            for (i, o) in images.iter().enumerate() {
                let o = o.get(..OFFICE_SIZE as usize).ok_or("oficina incompleta para el guion plano")?;
                mem.seed(OFFICES + OFFICE_SIZE * i as u32, o);
                offices_seeded.push(o.to_vec());
            }
        }
        let (mut ships, mut convoys) = (None, None);
        if let Some(nav) = w.navigation {
            if nav.manager.len() < 0xf8 {
                return Err("gestor naval incompleto para el guion plano".into());
            }
            let mut manager = nav.manager.clone();
            manager[4..8].copy_from_slice(&SHIPT.to_le_bytes());
            manager[8..12].copy_from_slice(&CONVOYS.to_le_bytes());
            for k in 0..5u32 {
                let at = 0xc + 4 * k as usize;
                manager[at..at + 4].copy_from_slice(&(BAND_SENTINEL + 0x1000 * k).to_le_bytes());
            }
            mem.seed(SHIPS, &manager);
            mem.seed(SHIPT, &nav.ships);
            // 64 registros a continuación por si `0x0050E000` tiene que crecer la tabla (se escriben ahí).
            mem.seed(SHIPT + nav.ships.len() as u32, &vec![0u8; 0x40 * SHIP_SIZE]);
            mem.seed(CONVOYS, &nav.convoys);
            before.push((SHIPS, manager));
            ships = Some(nav.ships.clone());
            convoys = Some(nav.convoys.clone());
        }
        let pool_header = c08_flat::seed_pool(&mut mem, w.pool)?;
        let room = w.scheduler.records.len() + SPARE_RECORDS;
        mem.seed(c08_flat::SCHED, &[0u8; 0x10]);
        mem.seed(RECORDS, &vec![0u8; 0x18 * room]);
        mem.seed(RECORDS_B, &vec![0u8; 0x18 * room]);
        c08_flat::write_scheduler(&mut mem, w.scheduler, room)?;
        for (at, len) in [(WORLD, 0x6), (WORLD + 8, 4), (WORLD + 0x10, 8), (WORLD + 0x68, 4), (WORLD + 0x78, 4), (WORLD + 0x8a, 1), (WORLD + 0x8d, 2), (0x006d_f31c, 4), (0x006d_fc14, 4)] {
            before.push((at, mem.read(at, len)));
        }
        if w.offices.is_some() {
            before.push((WORLD + 0x74, mem.read(WORLD + 0x74, 4)));
        }
        for (at, len) in fixed {
            before.push((at, mem.read(at, len)));
        }
        let calls = Calls::new(w.towns.to_vec(), count, offices, 0, room);
        Ok(ScriptImage {
            mem: Checked { inner: mem, error: None },
            host: Host {
                calls,
                ops: Vec::new(),
                wall_levels: w.wall_levels.clone(),
                rng: w.rng,
                rng_used: false,
                sites: w.sites.map(|sites| sites.iter().map(|s| p3_core::buildings::Site { records: s.records.clone(), cells: Vec::new() }).collect()),
                ship_names: w.ship_names.map(<[_]>::to_vec),
                ports: w.ports.clone(),
                created: Vec::new(),
            },
            pool_header,
            before,
            towns: w.towns.len(),
            merchants: count,
            ships,
            convoys,
            offices: offices_seeded,
        })
    }

    /// Entorno para `execute_in_world_flat`.
    pub(crate) fn script(&mut self, task_word_10: u16) -> p3_core::mission_script_flat::FlatScript<'_> {
        p3_core::mission_script_flat::FlatScript { mem: &mut self.mem, calls: &mut self.host, task_word_10 }
    }

    /// Lee el estado tras un guion terminado sin error; rechaza escrituras en las regiones de solo lectura.
    pub(crate) fn finish(mut self, live_towns: &[Town], merchant_count: u16) -> Result<FlatOut, String> {
        if let Some(e) = self.mem.error.take() {
            return Err(e);
        }
        let created = !self.host.created.is_empty();
        for (base, raw) in &self.before {
            let mut now = self.mem.inner.read(*base, raw.len());
            if created && *base == SHIPS {
                // Campos del gestor que mueve `0x0050E000`: libre `+0xE2`, atracados `+0xE8`, nombre `+0xEE`, cuenta `+0xF4`.
                for at in [0xe2, 0xe8, 0xee, 0xf4] {
                    now[at..at + 2].copy_from_slice(&raw[at..at + 2]);
                }
            }
            if now != *raw {
                return Err(format!("guion plano: escritura en {base:#x} (solo lectura en p3-sim)"));
            }
        }
        let created = if created {
            let manager = self.mem.inner.read(SHIPS, self.before.iter().find(|(b, _)| *b == SHIPS).map_or(0, |(_, r)| r.len()));
            let count = usize::from(u16::from_le_bytes([manager[0xf4], manager[0xf5]]));
            Some(CreatedShips { ids: self.host.created.clone(), ships: self.mem.inner.read(SHIPT, count * SHIP_SIZE), manager })
        } else {
            None
        };
        let ship_writes = match (&self.ships, &created) {
            (Some(raw), None) => check_fields(raw, &self.mem.inner.read(SHIPT, raw.len()), SHIP_SIZE, &SHIP_FIELDS, "barco")?,
            // Con altas la tabla se publica entera; el resto de barcos sigue limitado a los campos del guion.
            (Some(raw), Some(c)) => {
                let mut now = c.ships[..raw.len()].to_vec();
                for &id in &c.ids {
                    let at = usize::from(id) * SHIP_SIZE;
                    if at < raw.len() {
                        now[at..at + SHIP_SIZE].copy_from_slice(&raw[at..at + SHIP_SIZE]);
                    }
                }
                // La cabeza de atracados encadena el barco anterior por `+6` del nuevo, no del viejo: sin más excepciones.
                check_fields(raw, &now, SHIP_SIZE, &SHIP_FIELDS, "barco")?;
                Vec::new()
            }
            (None, _) => Vec::new(),
        };
        let convoy_writes = match &self.convoys {
            Some(raw) => check_fields(raw, &self.mem.inner.read(CONVOYS, raw.len()), CONVOY_SIZE, &[(0x18, 0x1c)], "convoy")?,
            None => Vec::new(),
        };
        let mut offices = Vec::new();
        for (i, raw) in self.offices.iter().enumerate() {
            let now = self.mem.inner.read(OFFICES + OFFICE_SIZE * i as u32, raw.len());
            if !check_fields(raw, &now, raw.len(), &OFFICE_FIELDS, "oficina")?.is_empty() {
                offices.push((i, now));
            }
        }
        let mut towns = live_towns.to_vec();
        let mut changed_towns = Vec::new();
        for (i, t) in towns.iter_mut().enumerate().take(self.towns) {
            let now = self.mem.inner.read(TOWNS + TOWN_SIZE * i as u32, t.0.len());
            if now[..] != t.0[..] {
                t.0.copy_from_slice(&now);
                changed_towns.push(i);
            }
        }
        let mut merchants = (0..self.merchants)
            .map(|i| RepMerchant { bytes: self.mem.inner.read(MERCHANTS + MERCHANT_SIZE * i as u32, MERCHANT_SIZE as usize) })
            .collect::<Vec<_>>();
        let mut pool = c08_flat::read_pool(&mut self.mem.inner, self.pool_header);
        let scheduler = c08_flat::scheduler_from(&mut self.mem.inner);
        let date = p3_core::calendar::Calendar::for_tick(self.mem.inner.r32(WORLD + 0x14));
        for (code, record) in &self.host.calls.letters {
            p3_state::messages::post_message(&mut pool, &mut merchants, merchant_count, date.day, date.month, date.year, *code, record, p3_state::messages::Payload::None, &p3_state::messages::RETENTION)?;
        }
        let operations = self.host.ops.iter().map(|r| p3_core::pump::Operation(*r)).collect();
        let rng = self.host.rng.filter(|_| self.host.rng_used);
        Ok(FlatOut { towns, changed_towns, merchants, pool, scheduler, operations, ship_writes, convoy_writes, offices, rng, created })
    }
}

/// Bytes cambiados entre `before` y `now` (tablas de registros de `size` bytes); error si alguno cae fuera de
/// los `fields` admitidos.
fn check_fields(before: &[u8], now: &[u8], size: usize, fields: &[(usize, usize)], what: &str) -> Result<Vec<(usize, u8)>, String> {
    let mut writes = Vec::new();
    for (k, (a, b)) in before.iter().zip(now).enumerate() {
        if a == b {
            continue;
        }
        let off = k % size;
        if !fields.iter().any(|&(lo, hi)| (lo..hi).contains(&off)) {
            return Err(format!("guion plano: escritura en {what} {} +{off:#x} (solo lectura en p3-sim)", k / size));
        }
        writes.push((k, *b));
    }
    Ok(writes)
}

/// Opciones del guardado tal como quedan en `[0x006CC3E8]` tras el cargador `0x00549F40`: borra `+0x3C..+0x4C`
/// y, salvo con `+0xF == 6`, acota `+8` a 14.
pub(crate) fn loaded_settings(saved: &[u8]) -> Vec<u8> {
    let mut s = saved.to_vec();
    if let Some(tail) = s.get_mut(0x3c..0x4c) {
        tail.fill(0);
    }
    if s.len() > 0xf && s[0xf] != 6 && s[8] > 0xe {
        s[8] = 0xe;
    }
    s
}

/// Lee la ventana `.rdata` del ejecutable inglés.
pub(crate) fn rdata_from_exe(exe: &[u8]) -> Result<Vec<u8>, String> {
    let image = p3_assets::exe::PeImage::parse(exe).map_err(str::to_owned)?;
    (RDATA.0..RDATA.1).map(|a| image.byte_at(a).map_err(str::to_owned)).collect()
}

/// Error de un comando que solo existe sobre la imagen plana: con él merece la pena repetir el guion sembrándola.
pub(crate) fn needs_flat(error: &str) -> bool {
    error
        .strip_prefix("comando 0x")
        .and_then(|rest| rest.split_once(" sin implementar"))
        .and_then(|(op, _)| u8::from_str_radix(op, 16).ok())
        .is_some_and(p3_core::mission_script_flat::handles)
}

impl crate::CouncilExecution<'_> {
    /// Imagen plana del estado vivo para repetir un guion de misión cuyos comandos solo existen sobre ella.
    pub(crate) fn mission_flat_image(&self, s: &Scheduler) -> Result<ScriptImage, String> {
        let rdata = self.mission_flat_rdata.as_deref().ok_or("guion plano sin .rdata del ejecutable")?;
        let count = usize::from(self.world.merchant_count);
        let merchants = self.merchants.get(..count).ok_or("comerciantes del guion plano incompletos")?;
        ScriptImage::seed(&ScriptWorld {
            towns: &self.world.towns,
            merchants,
            offices: self.scheduled_office_images.as_deref(),
            navigation: self.navigation.as_ref().map(|n| &n.snapshot),
            pool: self.pool,
            scheduler: s,
            tick: self.tick,
            day_of_year: self.world.day_of_year,
            world_8a: self.world.world_8a,
            alderman_merchant: self.world.alderman_merchant,
            alderman_town: self.world.alderman_town,
            network_client: self.world.network_client,
            local_merchant: self.world.local_merchant,
            rdata,
            start_time: Some(self.game_end_start),
            settings: self.mission_settings.as_deref(),
            distances: self.mission_distances.filter(|d| d.len() == self.world.towns.len() * self.world.towns.len()),
            wall_levels: (self.sites.len() >= self.world.towns.len())
                .then(|| self.world.towns.iter().zip(self.sites.iter()).map(|(t, site)| site.wall_expansion_level(t)).collect()),
            sell_factor: self.difficulty,
            rng: *self.investigation_rng,
            sites: Some(self.sites),
            operations_mode: self.operations_mode,
            ship_names: self.expedition_resources.as_ref().map(|r| r.ship_names.as_slice()),
            ports: self.expedition_resources.as_ref().and_then(|r| r.ports.iter().map(|p| *p).collect::<Option<Vec<_>>>()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use p3_core::mission_script::{execute_in_world_flat, Outcome};
    use p3_core::scheduler::TaskRecord;

    /// Guion de una orden seguida de una espera (`0xFD`), con `vars` variables.
    fn blob(cmd: &[u8], vars: u16) -> Vec<u8> {
        let mut b = Vec::new();
        b.extend_from_slice(&2u16.to_le_bytes());
        b.extend_from_slice(&vars.to_le_bytes());
        b.extend_from_slice(&12u32.to_le_bytes());
        b.extend_from_slice(&(12 + cmd.len() as u32).to_le_bytes());
        b.extend_from_slice(cmd);
        b.extend_from_slice(&[0xfd, 3, 0, 0]);
        b
    }

    struct Fixture {
        towns: Vec<Town>,
        merchants: Vec<RepMerchant>,
        offices: Vec<Vec<u8>>,
        pool: Vec<u8>,
        scheduler: Scheduler,
        rdata: Vec<u8>,
    }

    fn fixture() -> Fixture {
        let mut town = Town::default();
        town.0[0x784..0x786].copy_from_slice(&0u16.to_le_bytes());
        let mut office = vec![0u8; OFFICE_SIZE as usize];
        office[0x2ca..0x2cc].copy_from_slice(&u16::MAX.to_le_bytes());
        let mut scheduler = Scheduler::new();
        let id = scheduler.add_task(10);
        scheduler.records[usize::from(id)].kind = 0x1b;
        let mut pool = vec![0u8; 12];
        pool[..4].copy_from_slice(&p3_save::tail::MESSAGE_MARKER.to_le_bytes());
        pool[8..10].copy_from_slice(&u16::MAX.to_le_bytes());
        Fixture {
            towns: vec![town, Town::default()],
            merchants: vec![RepMerchant { bytes: vec![0; MERCHANT_SIZE as usize] }],
            offices: vec![office],
            pool,
            scheduler,
            rdata: vec![0; (RDATA.1 - RDATA.0) as usize],
        }
    }

    fn world(f: &Fixture) -> ScriptWorld<'_> {
        ScriptWorld {
            towns: &f.towns,
            merchants: &f.merchants,
            offices: Some(&f.offices),
            navigation: None,
            pool: &f.pool,
            scheduler: &f.scheduler,
            tick: 0x1234,
            day_of_year: 7,
            world_8a: 0,
            alderman_merchant: 0,
            alderman_town: 1,
            network_client: 0,
            local_merchant: 0,
            rdata: &f.rdata,
            start_time: None,
            settings: None,
            distances: None,
            wall_levels: None,
            sell_factor: 2.0,
            rng: None,
            sites: None,
            operations_mode: 0,
            ship_names: None,
            ports: None,
        }
    }

    fn run(image: &mut ScriptImage, cmd: &[u8], vars: &mut [u32]) -> Result<Outcome, String> {
        let mut task = TaskRecord { kind: 0x1b, ..Default::default() };
        let blob = blob(cmd, vars.len() as u16);
        execute_in_world_flat(&blob, &mut task, vars, 0x1234, None, None, None, None, None, None, None, None, None, Some(&mut image.script(0)))
    }

    #[test]
    fn reads_known_world_fields() {
        let f = fixture();
        let mut image = ScriptImage::seed(&world(&f)).unwrap();
        let mut vars = [0u32; 4];
        // 0x56: ciudad del asiento del consejo (`GameWorld+0x8E`).
        assert_eq!(run(&mut image, &[0x56, 2], &mut vars), Ok(Outcome::Waiting));
        assert_eq!(vars[2], 1);
        let out = image.finish(&f.towns, 1).unwrap();
        assert!(out.changed_towns.is_empty() && out.operations.is_empty());
        assert_eq!(out.merchants[0].bytes, f.merchants[0].bytes);
    }

    #[test]
    fn unseeded_reads_fail() {
        let f = fixture();
        let mut image = ScriptImage::seed(&world(&f)).unwrap();
        let mut vars = [0u32; 4];
        // 0x02 lee la fecha de inicio `GameWorld+0xC`, que la simulación no conoce.
        let r = run(&mut image, &[0x02, 1], &mut vars);
        let e = match r {
            Ok(_) => image.finish(&f.towns, 1).err().unwrap(),
            Err(e) => image.mem.error.take().unwrap_or(e),
        };
        assert!(e.contains("sin sembrar"), "{e}");
    }

    #[test]
    fn office_writes_are_published() {
        let f = fixture();
        let mut image = ScriptImage::seed(&world(&f)).unwrap();
        // 0x48: suma a `Office+0x2E8` de la oficina del comerciante 0 en la ciudad 0.
        let mut vars = [0u32, 0, 0, 5];
        assert!(run(&mut image, &[0x48, 0, 1, 2, 3], &mut vars).is_ok());
        let out = image.finish(&f.towns, 1).unwrap();
        assert_eq!(out.offices.len(), 1);
        let (i, office) = &out.offices[0];
        assert_eq!(*i, 0);
        assert_ne!(office[0x2e8..0x2ec], f.offices[0][0x2e8..0x2ec]);
        assert_eq!(office[..0x2e8], f.offices[0][..0x2e8]);
    }

    #[test]
    fn writes_outside_script_fields_are_rejected() {
        let f = fixture();
        let mut image = ScriptImage::seed(&world(&f)).unwrap();
        image.mem.inner.w8(OFFICES + 0x2ca, 7);
        let e = image.finish(&f.towns, 1).err().unwrap();
        assert!(e.contains("solo lectura") && e.contains("+0x2ca"), "{e}");
    }

    #[test]
    fn start_date_and_options_come_from_the_save() {
        let f = fixture();
        let mut settings = vec![0u8; 0x4c];
        settings[0x16] = 3;
        let mut w = world(&f);
        w.start_time = Some(0x0007_5300);
        w.settings = Some(&settings);
        let mut image = ScriptImage::seed(&w).unwrap();
        let mut vars = [0u32; 4];
        assert!(run(&mut image, &[0x02, 1], &mut vars).is_ok(), "{:?}", image.mem.error);
        assert!(run(&mut image, &[0x35, 2], &mut vars).is_ok(), "{:?}", image.mem.error);
        assert_eq!(vars[2], 3);
        image.finish(&f.towns, 1).unwrap();
    }

    #[test]
    fn loader_settings_match_0x549f40() {
        let mut saved = vec![0xffu8; 0x4c];
        saved[0xf] = 0;
        let s = loaded_settings(&saved);
        assert_eq!(s[8], 0xe);
        assert!(s[0x3c..].iter().all(|&b| b == 0) && s[..0x3c].iter().filter(|&&b| b == 0xff).count() == 0x3a);
        saved[0xf] = 6;
        assert_eq!(loaded_settings(&saved)[8], 0xff);
    }

    /// Flota de dos barcos (el 1 libre, cabeza de la lista libre) del comerciante 0.
    fn navigation() -> p3_save::ships::NavigationSnapshot {
        let mut manager = vec![0u8; 0x100];
        manager[0xe2..0xe4].copy_from_slice(&1u16.to_le_bytes());
        manager[0xe8..0xea].copy_from_slice(&0u16.to_le_bytes());
        manager[0xf0..0xf2].copy_from_slice(&1u16.to_le_bytes());
        manager[0xf4..0xf6].copy_from_slice(&2u16.to_le_bytes());
        let mut ships = vec![0u8; 2 * SHIP_SIZE];
        ships[4..6].copy_from_slice(&u16::MAX.to_le_bytes());
        ships[6..8].copy_from_slice(&u16::MAX.to_le_bytes());
        ships[0x120..0x124].copy_from_slice(&90u32.to_le_bytes());
        ships[SHIP_SIZE + 6..SHIP_SIZE + 8].copy_from_slice(&2u16.to_le_bytes());
        ships[SHIP_SIZE + 0x134..SHIP_SIZE + 0x136].copy_from_slice(&0xffu16.to_le_bytes());
        p3_save::ships::NavigationSnapshot {
            manager,
            ships,
            captains: Vec::new(),
            convoys: Vec::new(),
            routes: vec![Some(p3_save::ships::SavedRoute::None), None],
            lists: p3_save::ships::NavigationLists { docked_head: 0, sea_head: u16::MAX, docked: vec![0], sea: Vec::new() },
        }
    }

    #[test]
    fn script_rand_uses_and_returns_the_crt_stream() {
        let f = fixture();
        let mut w = world(&f);
        w.rng = Some(p3_core::rng::CrtRand { state: 77 });
        let mut town = f.towns.clone();
        town[0].0[0x2d4..0x2d8].copy_from_slice(&4000u32.to_le_bytes());
        w.towns = &town;
        let mut image = ScriptImage::seed(&w).unwrap();
        let mut vars = [0u32; 4];
        // 0xEC: peste en la ciudad `vars[0]` (siete llamadas a `rand`: duración, extensión y cuatro letras).
        assert!(run(&mut image, &[0xec, 0], &mut vars).is_ok(), "{:?}", image.mem.error);
        let out = image.finish(&town, 1).unwrap();
        let mut expected = p3_core::rng::CrtRand { state: 77 };
        for _ in 0..6 {
            expected.rand();
        }
        assert_eq!(out.rng, Some(expected));
        assert_eq!(out.operations.len(), 1);
        assert_eq!(out.operations[0].0[0], 0x7c);
        // Sin estado CRT el guion se rechaza.
        let mut image = ScriptImage::seed(&world(&f)).unwrap();
        let e = run(&mut image, &[0xec, 0], &mut vars).unwrap_err();
        assert!(e.contains("CRT"), "{e}");
    }

    #[test]
    fn script_fire_picks_a_building_from_the_site() {
        let f = fixture();
        let mut town = f.towns.clone();
        // Dos registros públicos: 0x764 cabeza, 0x776 cuenta; el tipo 0x10 cuenta como destino (0x672B3C).
        town[0].0[0x764..0x766].copy_from_slice(&0u16.to_le_bytes());
        town[0].0[0x776..0x778].copy_from_slice(&2u16.to_le_bytes());
        town[0].0[0x31c..0x320].copy_from_slice(&1u32.to_le_bytes());
        let site = p3_core::buildings::Site { records: vec![3, 4, 0, 0x10, 1, 0, 0, 0, 5, 6, 0, 0x10, 0xff, 0xff, 0, 0], cells: Vec::new() };
        let sites = vec![site.clone(), site];
        let mut w = world(&f);
        w.towns = &town;
        w.sites = Some(&sites);
        let mut image = ScriptImage::seed(&w).unwrap();
        let mut vars = [0u32; 4];
        // 0xED: incendio en la ciudad 0 con semilla `Town+0x31C & 0x3F` = 1 → primer edificio (3, 4).
        assert!(run(&mut image, &[0xed, 0], &mut vars).is_ok(), "{:?}", image.mem.error);
        let out = image.finish(&town, 1).unwrap();
        assert_eq!(out.operations.len(), 1);
        let op = out.operations[0].0;
        assert_eq!((op[0], op[3] as u8, (op[3] >> 8) as u8), (0x7d, 3, 4));
    }

    #[test]
    fn script_average_armament_reads_every_merchant_fleet() {
        let f = fixture();
        let nav = navigation();
        let mut merchants = f.merchants.clone();
        merchants[0].bytes[0xe..0x10].copy_from_slice(&0u16.to_le_bytes());
        let mut w = world(&f);
        w.navigation = Some(&nav);
        w.merchants = &merchants;
        let mut image = ScriptImage::seed(&w).unwrap();
        let mut vars = [0u32; 4];
        // 0x10: barco del comerciante `vars[0]` con armamento por encima de la media (90): el barco 0.
        assert!(run(&mut image, &[0x10, 0, 2], &mut vars).is_ok(), "{:?}", image.mem.error);
        assert_eq!(vars[2], 0);
        assert!(image.finish(&f.towns, 1).unwrap().created.is_none());
    }

    #[test]
    fn script_ship_creation_publishes_the_whole_table() {
        let f = fixture();
        let nav = navigation();
        let names = vec![b"Kogge".to_vec(), b"Hulk".to_vec()];
        let mut w = world(&f);
        w.navigation = Some(&nav);
        w.ship_names = Some(&names);
        w.ports = Some(vec![(100, 200), (300, 400)]);
        let mut image = ScriptImage::seed(&w).unwrap();
        // 0x67: comerciante vars[0], tipo vars[1], mínimo vars[2], ciudad vars[3]; índice en vars[4].
        let mut vars = [0u32, 1, 0, 1, 9];
        assert!(run(&mut image, &[0x67, 0, 1, 2, 3, 4], &mut vars).is_ok(), "{:?}", image.mem.error);
        assert_eq!(vars[4], 1);
        let out = image.finish(&f.towns, 1).unwrap();
        let c = out.created.expect("alta publicada");
        assert_eq!(c.ids, vec![1]);
        assert_eq!(c.ships.len(), 2 * SHIP_SIZE);
        let ship = &c.ships[SHIP_SIZE..];
        assert_eq!((ship[0], ship[0x39], u16::from_le_bytes([ship[0x134], ship[0x135]])), (0, 1, 3));
        assert_eq!(&ship[0x1e..0x20], &300u16.to_le_bytes());
        assert_eq!(u16::from_le_bytes([c.manager[0xe8], c.manager[0xe9]]), 1);
        assert_eq!(u16::from_le_bytes([c.manager[0xe2], c.manager[0xe3]]), 2);
        assert_eq!(out.merchants[0].u16(0xe), 1);
        // La lista libre se agota: la segunda alta crece la tabla 64 barcos.
        let mut image = ScriptImage::seed(&w).unwrap();
        let mut vars = [0u32, 1, 0, 1, 9];
        run(&mut image, &[0x67, 0, 1, 2, 3, 4], &mut vars).unwrap();
        run(&mut image, &[0x67, 0, 1, 2, 3, 4], &mut vars).unwrap();
        assert_eq!(vars[4], 2);
        let c = image.finish(&f.towns, 1).unwrap().created.unwrap();
        assert_eq!((c.ids.clone(), c.ships.len() / SHIP_SIZE), (vec![1, 2], 0x42));
    }

    #[test]
    fn only_flat_commands_trigger_the_retry() {
        assert!(needs_flat("comando 0x56 sin implementar en índice 3"));
        assert!(!needs_flat("comando 0x37 sin implementar en índice 3"));
        assert!(!needs_flat("límite de instrucciones del guion"));
    }
}
