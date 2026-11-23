//! C08: puente entre las vistas vivas de la simulación y los puertos planos de `p3_core` (`dyn Mem` con las
//! direcciones del original). Siembra el mundo (`0x006DE4A0`), el planificador (`0x006DD73C`), ciudades,
//! comerciantes, oficinas y la operación en las mismas posiciones que `reimpl/oracle/x86_c08_ops_live.py`;
//! resuelve las llamadas al original que conoce y vuelve a publicar el estado si todo terminó sin error.
use crate::{RepMerchant, Sim};
use p3_core::flatmem::{Mem, SparseMem};
use p3_core::scheduler::{Scheduler, TaskRecord};

pub(crate) const WORLD: u32 = 0x006d_e4a0;
pub(crate) const SCHED: u32 = 0x006d_d73c;
pub(crate) const RECORDS: u32 = 0x5400_0000;
/// Segunda región de registros para reproducir el traslado de la tabla al crecer.
pub(crate) const RECORDS_B: u32 = 0x5380_0000;
pub(crate) const TOWNS: u32 = 0x5500_0000;
pub(crate) const MERCHANTS: u32 = 0x5600_0000;
pub(crate) const OFFICES: u32 = 0x5700_0000;
pub(crate) const OP: u32 = 0x5800_0000;
/// Entradas del buzón en memoria (`0x006DD730` apunta aquí) y bloques de texto u objetos de carta.
pub(crate) const LETTER_SLOTS: u32 = 0x5900_0000;
pub(crate) const LETTER_BLOBS: u32 = 0x5a00_0000;
/// Listas de variables de las tareas 0x1B (`+0xC` apunta aquí).
pub(crate) const TASK_VARS: u32 = 0x5b00_0000;
pub(crate) const POOL: u32 = 0x006d_d730;
/// Registros y celdas de los planos de las ciudades.
const PLANS: u32 = 0x5e00_0000;
/// Ajustes de la partida a los que apunta `0x006CC3E8`.
const OPTIONS: u32 = 0x5f00_0000;
pub(crate) const TOWN_SIZE: u32 = 0x9f8;
pub(crate) const MERCHANT_SIZE: u32 = 0x650;
pub(crate) const OFFICE_SIZE: u32 = 0x44c;
/// Registros de reserva tras la capacidad: el planificador puede crecer durante la operación.
pub(crate) const SPARE_RECORDS: usize = 512;

pub(crate) struct Flat {
    pub mem: SparseMem,
    pub calls: Calls,
    /// Cabecera serializada del buzón (marca y bytes `+0xA..+0xC` que la memoria no guarda).
    pool_header: Option<[u8; 12]>,
    /// Tareas 0x1B: índice, carga útil original (`+0xC` es un puntero en memoria) y número de variables.
    scripts: Vec<(usize, [u8; 16], u32)>,
}

/// Siembra el buzón serializado en el formato de memoria del original.
pub(crate) fn seed_pool(mem: &mut SparseMem, pool: &[u8]) -> Result<[u8; 12], String> {
    let parsed = p3_save::tail::MessagePool::parse(pool).map_err(|e| format!("buzón: {e:?}"))?;
    let header: [u8; 12] = pool[..12].try_into().unwrap();
    let mut entries = Vec::with_capacity(16 * parsed.letters.len());
    let mut blobs = Vec::new();
    for letter in &parsed.letters {
        let raw = &pool[letter.range.clone()];
        let mut entry: [u8; 16] = raw[..16].try_into().unwrap_or([0; 16]);
        match letter.body {
            p3_save::tail::LetterBody::Inline => {}
            p3_save::tail::LetterBody::InlineBlob => {
                let len = u32::from_le_bytes(raw[12..16].try_into().unwrap()) as usize;
                if u32::from_le_bytes(raw[8..12].try_into().unwrap()) != 0 {
                    entry[8..12].copy_from_slice(&(LETTER_BLOBS + blobs.len() as u32).to_le_bytes());
                    blobs.extend_from_slice(&raw[16..16 + len]);
                    blobs.resize((blobs.len() + 15) & !15, 0);
                }
            }
            p3_save::tail::LetterBody::SizedBlob => {
                entry[8..12].copy_from_slice(&(LETTER_BLOBS + blobs.len() as u32).to_le_bytes());
                blobs.extend_from_slice(&raw[8..24]);
                entry[12..16].copy_from_slice(&(LETTER_BLOBS + blobs.len() as u32).to_le_bytes());
                blobs.extend_from_slice(&raw[24..]);
                blobs.resize((blobs.len() + 15) & !15, 0);
            }
        }
        entries.extend_from_slice(&entry);
    }
    mem.seed(LETTER_SLOTS, &entries);
    blobs.resize(blobs.len() + 16, 0);
    mem.seed(LETTER_BLOBS, &blobs);
    let mut object = [0u8; 12];
    object[..4].copy_from_slice(&LETTER_SLOTS.to_le_bytes());
    object[4..6].copy_from_slice(&header[6..8]);
    object[6..8].copy_from_slice(&header[4..6]);
    object[8..10].copy_from_slice(&header[8..10]);
    mem.seed(POOL, &object);
    Ok(header)
}

/// Vuelve a serializar el buzón de memoria como S05 (las cartas con texto guardan 1 en `+8`).
pub(crate) fn read_pool(mem: &mut SparseMem, header: [u8; 12]) -> Vec<u8> {
    let base = mem.r32(POOL);
    let slots = mem.r16(POOL + 6);
    let mut out = header.to_vec();
    out[4..6].copy_from_slice(&slots.to_le_bytes());
    out[6..8].copy_from_slice(&mem.r16(POOL + 4).to_le_bytes());
    out[8..10].copy_from_slice(&mem.r16(POOL + 8).to_le_bytes());
    for i in 0..u32::from(slots) {
        let mut entry = mem.read(base + 16 * i, 16);
        let kind = entry[4];
        match p3_save::tail::letter_body(kind) {
            p3_save::tail::LetterBody::Inline => out.extend_from_slice(&entry),
            p3_save::tail::LetterBody::InlineBlob => {
                let ptr = u32::from_le_bytes(entry[8..12].try_into().unwrap());
                let len = u32::from_le_bytes(entry[12..16].try_into().unwrap());
                entry[8..12].copy_from_slice(&u32::from(ptr != 0).to_le_bytes());
                out.extend_from_slice(&entry);
                if ptr != 0 {
                    out.extend_from_slice(&mem.read(ptr, len as usize));
                }
            }
            p3_save::tail::LetterBody::SizedBlob => {
                let object = u32::from_le_bytes(entry[8..12].try_into().unwrap());
                let block = u32::from_le_bytes(entry[12..16].try_into().unwrap());
                out.extend_from_slice(&entry[..8]);
                let head = mem.read(object, 16);
                let size = u32::from_le_bytes(head[..4].try_into().unwrap());
                out.extend_from_slice(&head);
                out.extend_from_slice(&mem.read(block, size as usize));
            }
        }
    }
    out
}

/// Estado de las llamadas al original que resuelve el puente.
/// Efectos diferidos de `hire_pirate_captain`: comerciantes cuyos grupos hay que podar (`0x004F5900`) y la ruta que fija `0x005172A0`
/// (barco y su imagen en ese instante, de la que sale la duración).
#[derive(Default)]
pub(crate) struct HireOut {
    pub pruned: Vec<u32>,
    pub route: Option<(u32, Vec<u8>)>,
}

/// Recursos de `Captain::init` (`0x004FDF50`) para las altas de capitán del puente (`0x005097C0`).
#[derive(Clone, Copy)]
pub(crate) struct CaptainSeed {
    pub tick: u32,
    pub day_of_year: u16,
    pub day: u8,
    pub population: u32,
    pub first_names: u16,
    pub last_names: u16,
}

/// Cabeza de la lista libre de capitanes (`ShipManager+0xE0`) y capacidad (`+0xF2`) en la memoria plana.
pub(crate) const CAPTAIN_FREE_HEAD: u32 = 0x006d_d880;
pub(crate) const CAPTAIN_CAPACITY: u32 = 0x006d_d892;

pub(crate) struct Calls {
    /// Recursos para dar de alta capitanes; sin ellos `0x005097C0` falla.
    pub captain_seed: Option<CaptainSeed>,
    towns: usize,
    merchants: usize,
    offices: usize,
    /// Copia de las ciudades para los entornos de reputación.
    towns_live: Vec<p3_core::town::Town>,
    church: u8,
    /// Registros que caben en cada región de la tabla.
    room: usize,
    /// Cartas pedidas al original (`0x004D6530`): destinatario y registro de 16 bytes.
    pub letters: Vec<(i32, [u8; 16])>,
    /// Barcos que `ShipManager::remove_ship` (`0x00507FB0`) debe retirar de las vistas navales.
    pub removed_ships: Vec<u32>,
    /// Efectos de `0x0053B850` (contratar capitán pirata) que se aplican sobre las vistas vivas después de la ejecución plana.
    pub hire: HireOut,
    /// Valores de `rand` (0x00639C03) ya sorteados por el llamante, en el orden en que el puerto los pide.
    pub rand_queue: Vec<i32>,
    pub error: Option<String>,
}

pub(crate) fn scheduler_from(mem: &mut dyn Mem) -> Scheduler {
    let base = mem.r32(SCHED);
    let capacity = mem.r16(SCHED + 0xc);
    let mut s = Scheduler::new();
    s.capacity = capacity;
    s.records = vec![TaskRecord::default(); usize::from(capacity)];
    for i in 0..usize::from(capacity) {
        let at = base + 0x18 * i as u32;
        let mut r = TaskRecord { due: mem.r32(at), next: mem.r16(at + 4), kind: mem.r16(at + 6), payload: [0; 16] };
        for k in 0..16 {
            r.payload[k] = mem.r8(at + 8 + k as u32);
        }
        s.records[i] = r;
    }
    s.dispatching = mem.r32(SCHED + 4);
    s.head = mem.r16(SCHED + 8);
    s.free_head = mem.r16(SCHED + 0xa);
    s
}

pub(crate) fn write_scheduler(mem: &mut dyn Mem, s: &Scheduler, room: usize) -> Result<(), String> {
    write_scheduler_at(mem, s, RECORDS, room)
}

/// Escribe la tabla en `base` (una de las dos regiones de registros) y apunta el planificador a ella.
fn write_scheduler_at(mem: &mut dyn Mem, s: &Scheduler, base: u32, room: usize) -> Result<(), String> {
    if s.records.len() > room {
        return Err("crecimiento del planificador mayor que la reserva del puente".into());
    }
    for (i, r) in s.records.iter().enumerate() {
        let at = base + 0x18 * i as u32;
        let mut raw = Vec::with_capacity(0x18);
        raw.extend_from_slice(&r.due.to_le_bytes());
        raw.extend_from_slice(&r.next.to_le_bytes());
        raw.extend_from_slice(&r.kind.to_le_bytes());
        raw.extend_from_slice(&r.payload);
        for (k, b) in raw.into_iter().enumerate() {
            mem.w8(at + k as u32, b);
        }
    }
    mem.w32(SCHED, base);
    mem.w32(SCHED + 4, s.dispatching);
    mem.w16(SCHED + 8, s.head);
    mem.w16(SCHED + 0xa, s.free_head);
    mem.w16(SCHED + 0xc, s.capacity);
    Ok(())
}

fn read_merchant(m: &mut dyn Mem, at: u32) -> RepMerchant {
    RepMerchant { bytes: (0..MERCHANT_SIZE).map(|k| m.r8(at + k)).collect() }
}

fn write_merchant(m: &mut dyn Mem, at: u32, merchant: &RepMerchant) {
    for (k, &b) in merchant.bytes.iter().enumerate() {
        m.w8(at + k as u32, b);
    }
}

impl Calls {
    /// Llamadas sobre la misma disposición de memoria para otros puentes (guiones de misión de C10).
    pub(crate) fn new(towns_live: Vec<p3_core::town::Town>, merchants: usize, offices: usize, church: u8, room: usize) -> Self {
        Calls { towns: towns_live.len(), merchants, offices, towns_live, church, room, letters: Vec::new(), removed_ships: Vec::new(), hire: Default::default(), rand_queue: Vec::new(), captain_seed: None, error: None }
    }

    /// `0x005097C0` y `0x005098B0` con `CaptainPool` sobre la tabla plana; no admite que el pool crezca (la tabla viva tiene otro tamaño).
    fn captain_pool(&mut self, m: &mut dyn Mem, addr: u32, args: &[u32]) -> u32 {
        use p3_core::ships::captains::{CaptainHost, CaptainPool, ENTRY};
        struct Nothing;
        impl CaptainHost for Nothing {
            fn init(&mut self, _: u16, _: bool) {}
        }
        let capacity = m.r16(CAPTAIN_CAPACITY);
        let table = m.r32(0x006d_d7a0);
        let mut pool = CaptainPool { entries: (0..usize::from(capacity) * ENTRY).map(|k| m.r8(table + k as u32)).collect(), capacity, free_head: m.r16(CAPTAIN_FREE_HEAD) };
        let result = if addr == 0x5097c0 {
            if pool.free_head >= pool.capacity {
                return self.fail("el pool de capitanes está lleno: su crecimiento no está conectado".into());
            }
            let Some(seed) = self.captain_seed else {
                return self.fail("recursos de Captain::init no preparados".into());
            };
            let flag = args.first().copied().unwrap_or(0) as u8;
            let index = pool.allocate(flag, &mut Nothing);
            let at = usize::from(index) * ENTRY;
            let ok = p3_core::ships::factory::init_captain(&mut pool.entries[at..at + ENTRY], u32::from(index), flag == 0, seed.tick, seed.day_of_year, seed.day, seed.population, seed.first_names, seed.last_names);
            if ok.is_none() {
                return self.fail("bancos de nombres de capitán vacíos".into());
            }
            u32::from(index)
        } else {
            let slot_at = args.first().copied().unwrap_or(0);
            let mut slot = m.r16(slot_at);
            pool.free(&mut slot);
            m.w16(slot_at, slot);
            0
        };
        for (k, b) in pool.entries.iter().enumerate() {
            m.w8(table + k as u32, *b);
        }
        m.w16(CAPTAIN_FREE_HEAD, pool.free_head);
        result
    }

    fn merchant_at(&self, at: u32) -> Option<usize> {
        let off = at.checked_sub(MERCHANTS)?;
        (off % MERCHANT_SIZE == 0 && ((off / MERCHANT_SIZE) as usize) < self.merchants).then_some((off / MERCHANT_SIZE) as usize)
    }

    fn fail(&mut self, e: String) -> u32 {
        self.error.get_or_insert(e);
        0
    }

    /// Llamadas al original resueltas por el puente; cualquier otra registra un error y devuelve 0.
    pub(crate) fn call(&mut self, m: &mut dyn Mem, addr: u32, ecx: u32, args: &[u32]) -> u32 {
        match addr {
            // calc_ship_repair_cost(barco) de la ciudad `ecx`
            0x52a9e0 => p3_core::economy::ops_misc::repair_cost_flat(m, ecx, args[0]) as u32,
            // add_social_reputation_ship_hull_work(ciudad, casco) del comerciante `ecx`
            0x4f9120 => {
                if self.merchant_at(ecx).is_none() {
                    return self.fail(format!("reputación sobre un comerciante ajeno {ecx:#x}"));
                }
                let bytes: Vec<u8> = (0..MERCHANT_SIZE).map(|i| m.r8(ecx + i)).collect();
                let mut rep = p3_core::ai::reputation::Merchant { bytes };
                rep.add_social_reputation_ship_hull_work(args[0], args[1]);
                for (i, b) in rep.bytes.iter().enumerate() {
                    m.w8(ecx + i as u32, *b);
                }
                0
            }
            // rand del CRT: el llamante sortea antes los valores que el puerto va a pedir
            0x639c03 => {
                if self.rand_queue.is_empty() {
                    return self.fail("rand sin valor preparado".into());
                }
                self.rand_queue.remove(0) as u32
            }
            // ShipManager::allocate_captain(flag) / free_captain(&campo) sobre la tabla plana de capitanes (`[0x006DD7A0]`).
            0x5097c0 | 0x5098b0 => return self.captain_pool(m, addr, args),
            // auto_trader_set_wage_from_trade_skill(capitán).
            0x4fe160 => p3_core::economy::small_fns::auto_trader_set_wage_from_trade_skill(m, ecx),
            // get_merchant
            0x5303c0 => {
                let i = args.first().copied().unwrap_or(u32::MAX) as usize;
                if i < self.merchants {
                    MERCHANTS + MERCHANT_SIZE * i as u32
                } else {
                    self.fail(format!("get_merchant({i}) fuera de la tabla"))
                }
            }
            // scheduled_tasks_get_task(índice): puntero al registro (`0x18 * índice`) o 0 si excede la capacidad.
            0x4d8db0 => {
                let index = args.first().copied().unwrap_or(u32::MAX);
                if index > u32::from(m.r16(SCHED + 0xc)) {
                    0
                } else {
                    m.r32(SCHED).wrapping_add(0x18 * index)
                }
            }
            // get_office
            0x530400 => {
                let i = args.first().copied().unwrap_or(u32::MAX) as usize;
                if i < self.offices {
                    OFFICES + OFFICE_SIZE * i as u32
                } else {
                    self.fail(format!("get_office({i}) fuera de la tabla"))
                }
            }
            // scheduled_tasks_add_task
            0x4d8cf0 => {
                let due = args.first().copied().unwrap_or(0);
                let mut s = scheduler_from(m);
                let old_base = m.r32(SCHED);
                let old_len = s.records.len();
                let id = s.add_task(due);
                // Al crecer, el original copia a un bloque nuevo y libera el viejo: los punteros a registros que el
                // llamante calculó antes siguen escribiendo en el bloque liberado, no en la tabla viva.
                let base = if s.records.len() != old_len { if old_base == RECORDS { RECORDS_B } else { RECORDS } } else { old_base };
                match write_scheduler_at(m, &s, base, self.room) {
                    Ok(()) => base + 0x18 * u32::from(id),
                    Err(e) => self.fail(e),
                }
            }
            // find_or_create_office(comerciante, ciudad, crear): solo búsqueda por la cadena de la ciudad
            // (`Town+0x784`, siguiente en `+0x2CA`, dueño en `+0x2C4`).
            0x5308a0 => {
                let (merchant, town, create) = (args[0] & 0xffff, args[1] & 0xffff, args[2]);
                if town as usize >= self.towns || merchant as usize >= self.merchants {
                    return 0;
                }
                let mut at = m.r16(TOWNS + TOWN_SIZE * town + 0x784);
                let mut steps = 0;
                while usize::from(at) < self.offices {
                    let office = OFFICES + OFFICE_SIZE * u32::from(at);
                    if u32::from(m.r16(office + 0x2c4)) == merchant {
                        return office;
                    }
                    at = m.r16(office + 0x2ca);
                    steps += 1;
                    if steps > self.offices {
                        return self.fail("cadena de oficinas cíclica".into());
                    }
                }
                if create != 0 {
                    return self.fail("creación de oficina desde el puente C08 no conectada".into());
                }
                0
            }
            // town_get_building_record: registro de 8 bytes del plano sembrado (`Town+0x75C`).
            0x51f1f0 => {
                let index = args.first().copied().unwrap_or(u32::MAX);
                if index < u32::from(m.r16(ecx + 0x776)) {
                    m.r32(ecx + 0x75c).wrapping_add(index * 8)
                } else {
                    0
                }
            }
            // op_queue_building_work_task(ciudad, edificio).
            0x4dab00 if args.len() == 2 => {
                if let Err(e) = p3_core::economy::c06_building_sale::queue_building_work_task(m, self, args[0], args[1]) {
                    self.fail(e.to_string());
                }
                0
            }
            0x4ff2d0 => self.fail("recuento de oficina (0x004FF2D0) desde el puente no conectado".into()),
            // letter_init / letter_free_and_reset: objetos de carta temporales sin estado de partida.
            0x51a850 | 0x51a880 => ecx,
            // ticks_to_date: el puerto escribe él mismo los campos de salida.
            0x5311d0 => 0,
            // get_merchant_epithet_index
            0x4f9430 => match self.merchant_at(ecx) {
                Some(_) => u32::from(read_merchant(m, ecx).get_merchant_epithet_index()),
                None => self.fail("epíteto de un comerciante desconocido".into()),
            },
            // merchant_subtract_social_reputation_flagged_towns
            0x4f91c0 => match self.merchant_at(ecx) {
                Some(_) => {
                    let mut merchant = read_merchant(m, ecx);
                    let env = p3_core::ai::reputation::RepEnv { town_count: self.towns_live.len() as u32, towns: &self.towns_live, renown: [0; 3], byte_52c: self.church };
                    merchant.subtract_social_reputation_flagged_towns(&env);
                    write_merchant(m, ecx, &merchant);
                    0
                }
                None => self.fail("reputación de un comerciante desconocido".into()),
            },
            // Métodos de barco de las órdenes de marineros: carga (0x5182B0), faltantes (0x5184F0) y marca de cambios (0x517770).
            0x5182b0 | 0x5184f0 => {
                let mut ship: Vec<u8> = (0..0x180).map(|k| m.r8(ecx + k)).collect();
                let out = if addr == 0x5182b0 { p3_core::ships::cargo::recompute_load(&mut ship) } else { p3_core::ships::ai_dock::needed_crew_image(&mut ship) };
                for (k, b) in ship.iter().enumerate() {
                    m.w8(ecx + k as u32, *b);
                }
                out as u32
            }
            0x517770 if args.len() == 1 => {
                let bits = args[0];
                let v = m.r16(ecx + 0x3c) | bits as u16;
                m.w16(ecx + 0x3c, v);
                let convoy = m.r16(ecx + 8);
                if convoy < m.r16(0x006d_d896) {
                    let at = m.r32(0x006d_d7a8) + 0x3c * u32::from(convoy) + 0x18;
                    let f = m.r32(at) | (bits & 0xffff);
                    m.w32(at, f);
                }
                0
            }
            // merchant_get_hireable_sailors(ciudad) y merchant_consume_sailor_pool(ciudad, n).
            0x4f6ca0 if args.len() == 1 && self.merchant_at(ecx).is_some() => read_merchant(m, ecx).get_hireable_sailors(args[0], &self.towns_live),
            0x4f6bd0 if args.len() == 2 && self.merchant_at(ecx).is_some() => {
                let mut merchant = read_merchant(m, ecx);
                merchant.consume_sailor_pool(args[0], args[1], self.towns_live.len() as u32);
                write_merchant(m, ecx, &merchant);
                0
            }
            // Contratar capitán pirata (0x4E): interfaz, grupos del comerciante, banda, ruta y tripulación.
            0x494990 | 0x4778a0 => 0,
            0x4f5900 => {
                match self.merchant_at(ecx) {
                    Some(i) => self.hire.pruned.push(i as u32),
                    None => return self.fail(format!("poda de grupos sobre un comerciante ajeno {ecx:#x}")),
                }
                0
            }
            0x515ea0 if args.len() == 1 => {
                p3_core::economy::pirate_small::band_append_ship(m, ecx, args[0]);
                0
            }
            0x5172a0 if args.len() == 4 => {
                let image: Vec<u8> = (0..0x180).map(|i| m.r8(ecx + i)).collect();
                self.hire.route = Some((args[3], image));
                0
            }
            0x5184f0 => {
                let mut image: Vec<u8> = (0..0x180).map(|i| m.r8(ecx + i)).collect();
                let needed = p3_core::ships::ai_dock::needed_crew_image(&mut image);
                for (i, b) in image.iter().enumerate() {
                    m.w8(ecx + i as u32, *b);
                }
                needed as u32
            }
            0x5182b0 => {
                let mut image: Vec<u8> = (0..0x180).map(|i| m.r8(ecx + i)).collect();
                p3_core::ships::cargo::recompute_load(&mut image);
                for (i, b) in image.iter().enumerate() {
                    m.w8(ecx + i as u32, *b);
                }
                0
            }
            // ShipManager::ship_add_cargo y OfficeInfo::add_stock (traspaso de mercancía entre barco y oficina, op 0x08).
            0x5180c0 if args.len() == 3 => {
                p3_core::economy::ops_misc::ship_add_cargo(m, self, ecx, args[0], args[1] as i32, args[2]);
                0
            }
            0x4ff6f0 if args.len() == 3 => {
                p3_core::economy::ops_misc::office_add_stock(m, ecx, args[0], args[1] as i32, args[2]);
                0
            }
            0x51a820 if args.len() == 1 => {
                p3_core::economy::pirate_small::ship_set_pirate_patrol_town(m, ecx, args[0]);
                0
            }
            0x64f7b9 | 0x513720 | 0x513d80 => self.fail("el traspaso a una banda pirata nueva no está conectado a la sesión".into()),
            0x507fb0 if args.len() == 1 => {
                self.removed_ships.push(args[0]);
                0
            }
            // add_message en la forma de seis valores de `assign_captain_to_ship`: registro tipo 4 (`0x00537F9D..0x00537FB2`).
            0x4d6530 if args.len() == 6 => {
                let mut record = [0u8; 16];
                record[1] = 0x80;
                record[2] = 0xff;
                record[3] = 0xff;
                record[4] = args[1] as u8;
                record[5] = args[2] as u8;
                record[8..10].copy_from_slice(&(args[3] as u16).to_le_bytes());
                record[10..12].copy_from_slice(&(args[4] as u16).to_le_bytes());
                record[12..14].copy_from_slice(&(args[5] as u16).to_le_bytes());
                self.letters.push((args[0] as i32, record));
                1
            }
            // add_message(destinatario, registro): el puerto pasa los 16 bytes como argumentos.
            0x4d6530 if args.len() == 17 => {
                let mut record = [0u8; 16];
                for (k, v) in args[1..].iter().enumerate() {
                    record[k] = *v as u8;
                }
                self.letters.push((args[0] as i32, record));
                1
            }
            _ => self.fail(format!("llamada {addr:#x} no resuelta por el puente C08")),
        }
    }
}

/// Las cartas viven en la memoria del puente, así que vaciar una entrada no necesita a la simulación.
pub(crate) struct OffersHost<'a> {
    pub mem_error: &'a mut Option<String>,
    pub released: Vec<u32>,
    /// Historial de información de ciudades serializado y matriz de distancias (`0x00535400`).
    pub history: Option<(Vec<u8>, &'a [u8], usize)>,
}

impl p3_core::economy::tavern_offers::TavernOffersHost for OffersHost<'_> {
    fn release_letter(&mut self, entry: u32) {
        self.released.push(entry);
    }
    fn forget_news(&mut self, town: u32, merchant: u32) {
        match self.history.as_mut() {
            Some((history, distances, n)) => {
                if let Err(e) = p3_core::economy::c08_town_info::mark_town_known(history, town, merchant, distances, *n) {
                    self.mem_error.get_or_insert(e.to_string());
                }
            }
            None => {
                self.mem_error.get_or_insert_with(|| "historial de ciudades ausente".into());
            }
        }
    }
}

/// `letter_free_and_reset` (`0x0051AAB0`) sobre una entrada en memoria; el enlace `+6` se conserva.
pub(crate) fn reset_letter(m: &mut dyn Mem, entry: u32) {
    for (k, b) in [0u8, 0, 0xff, 0xff, 0xff, 0].into_iter().enumerate() {
        m.w8(entry + k as u32, b);
    }
    for k in 8..16 {
        m.w8(entry + k, 0);
    }
}

/// Memoria compartida entre un puerto y un anfitrión que no la recibe como argumento (`TavernHost::add_task`): cada
/// acceso toma el préstamo solo durante el byte, así que no hay préstamos solapados.
#[derive(Clone)]
pub(crate) struct SharedMem(pub std::rc::Rc<std::cell::RefCell<SparseMem>>);

impl Mem for SharedMem {
    fn r8(&mut self, a: u32) -> u8 {
        self.0.borrow_mut().r8(a)
    }
    fn w8(&mut self, a: u32, v: u8) {
        self.0.borrow_mut().w8(a, v)
    }
    fn w8_seed(&mut self, a: u32) {
        self.0.borrow_mut().w8_seed(a)
    }
}

pub(crate) struct TavernTasks<'a> {
    pub calls: &'a mut Calls,
    pub mem: SharedMem,
}

impl p3_core::economy::tavern_interaction::TavernHost for TavernTasks<'_> {
    fn add_task(&mut self, due: u32) -> u32 {
        let mut mem = self.mem.clone();
        self.calls.call(&mut mem, 0x4d8cf0, SCHED, &[due])
    }
}

/// Ejecuta un puerto y convierte un acceso fuera de lo sembrado (pánico de `SparseMem`) en error.
pub(crate) fn guarded(port: impl FnOnce()) -> Result<(), String> {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(port)).map_err(|e| {
        let text = e.downcast_ref::<String>().cloned().or_else(|| e.downcast_ref::<&str>().map(|s| s.to_string())).unwrap_or_default();
        format!("puerto C08 fuera del estado sembrado: {text}")
    })
}

impl p3_core::economy::ops_misc::OpsHost for Calls {
    fn call(&mut self, m: &mut dyn Mem, addr: u32, ecx: u32, args: &[u32]) -> u32 {
        Calls::call(self, m, addr, ecx, args)
    }
}

impl Sim {
    /// Siembra el estado vivo; `op` son los 20 bytes de la operación.
    pub(crate) fn flat_seed(&self, op: [u32; 5]) -> Result<Flat, String> {
        let count = usize::from(self.economy.world.merchant_count);
        let towns = &self.economy.world.towns;
        let offices = self.host.scheduled_office_images.as_ref().ok_or("oficinas vivas ausentes")?;
        if self.rep_merchants.len() < count || offices.iter().any(|o| o.len() < OFFICE_SIZE as usize) {
            return Err("vistas incompletas para el puente C08".into());
        }
        let mut mem = SparseMem::default();
        let mut world = vec![0u8; 0x100];
        let date = self.idle.clock.calendar;
        world[0] = date.day;
        world[1] = date.month;
        world[2..4].copy_from_slice(&date.year.to_le_bytes());
        world[4..6].copy_from_slice(&date.day_of_year.to_le_bytes());
        world[8..10].copy_from_slice(&(offices.len() as u16).to_le_bytes());
        world[0xa..0xc].copy_from_slice(&(count as u16).to_le_bytes());
        world[0x10..0x14].copy_from_slice(&(towns.len() as u32).to_le_bytes());
        world[0x14..0x18].copy_from_slice(&self.idle.clock.tick.to_le_bytes());
        world[0x68..0x6c].copy_from_slice(&TOWNS.to_le_bytes());
        world[0x74..0x78].copy_from_slice(&OFFICES.to_le_bytes());
        world[0x78..0x7c].copy_from_slice(&MERCHANTS.to_le_bytes());
        world[0x8a] = self.economy.world.world_8a;
        world[0x8c] = self.host.church_cost_factor;
        world[0x8d] = self.economy.world.alderman_merchant;
        world[0x8e] = self.economy.world.alderman_town;
        mem.seed(WORLD, &world);
        mem.seed(0x6de508, &TOWNS.to_le_bytes());
        mem.seed(0x6df31c, &self.economy.world.network_client.to_le_bytes());
        mem.seed(0x6dfc14, &self.economy.world.local_merchant.to_le_bytes());
        // Planos de cada ciudad: registros de edificios (`+0x75C`) y celdas (`+0x80C`) en regiones propias.
        let mut plan_cursor = PLANS;
        for (i, t) in towns.iter().enumerate() {
            let mut image = t.0.to_vec();
            if let Some(site) = self.host.sites.get(i) {
                let records = plan_cursor;
                mem.seed(records, &site.records);
                plan_cursor = (plan_cursor + site.records.len() as u32 + 0x10f) & !0xff;
                let cells = plan_cursor;
                mem.seed(cells, &site.cells);
                plan_cursor = (plan_cursor + site.cells.len() as u32 + 0x10f) & !0xff;
                image[0x75c..0x760].copy_from_slice(&records.to_le_bytes());
                image[0x80c..0x810].copy_from_slice(&cells.to_le_bytes());
            }
            mem.seed(TOWNS + TOWN_SIZE * i as u32, &image);
        }
        // Objeto de opciones (`[0x006CC3E8]`): los ajustes de la partida, con el byte `+0xD` que lee la venta.
        let mut options = vec![0u8; 0x4c];
        options[0xd] = self.host.game_end_mode;
        mem.seed(OPTIONS, &options);
        mem.seed(0x6cc3e8, &OPTIONS.to_le_bytes());
        for (i, m) in self.rep_merchants[..count].iter().enumerate() {
            mem.seed(MERCHANTS + MERCHANT_SIZE * i as u32, &m.bytes[..MERCHANT_SIZE as usize]);
        }
        for (i, o) in offices.iter().enumerate() {
            mem.seed(OFFICES + OFFICE_SIZE * i as u32, &o[..OFFICE_SIZE as usize]);
        }
        let s = &self.host.council_scheduler;
        mem.seed(SCHED, &[0u8; 0x10]);
        let room = s.records.len() + SPARE_RECORDS;
        mem.seed(RECORDS, &vec![0u8; 0x18 * room]);
        mem.seed(RECORDS_B, &vec![0u8; 0x18 * room]);
        write_scheduler(&mut mem, s, room)?;
        mem.seed(OP, &op.iter().flat_map(|w| w.to_le_bytes()).collect::<Vec<_>>());
        let pool_header = Some(seed_pool(&mut mem, &self.host.message_pool)?);
        // StadtDaten.ini (`0x006DDB90`, 0x34 bytes por ciudad): la región en `+0x30`.
        if let Some(locations) = self.host.settlement_locations.as_ref() {
            for (k, loc) in locations.iter().enumerate() {
                mem.seed(0x6ddbc0 + 0x34 * k as u32, &[loc.kind]);
            }
        }
        // Variables de los guiones: `+0xC` pasa a apuntar a su lista y se restaura al publicar.
        let mut scripts = Vec::new();
        let mut cursor = TASK_VARS;
        let mut words = Vec::new();
        for (i, r) in s.records.iter().enumerate() {
            if r.kind != 0x1b {
                continue;
            }
            let vars = self.host.mission_variables.get(i).and_then(Option::as_ref).ok_or("variables de guion ausentes")?;
            let count = u32::from(u16::from_le_bytes([r.u8_at(0x12), r.u8_at(0x13)]));
            if vars.len() < count as usize {
                return Err("variables de guion truncadas".into());
            }
            let at = RECORDS + 0x18 * i as u32 + 8 + 4;
            for (k, b) in cursor.to_le_bytes().into_iter().enumerate() {
                mem.w8(at + k as u32, b);
            }
            scripts.push((i, r.payload, vars.len() as u32));
            for v in vars {
                words.extend_from_slice(&v.to_le_bytes());
            }
            cursor += 4 * vars.len() as u32;
        }
        words.resize(words.len() + 16, 0);
        mem.seed(TASK_VARS, &words);
        // Recuento de capitanes (`0x006DD892`): las oficinas lo comparan con el índice de su administrador (`+0x2F2`).
        let captain_count = match &self.host.navigation {
            Some(nav) => Some(nav.snapshot.captains.len() / 16),
            None => self.host.expedition_factory.as_ref().map(|f| f.captains.len()),
        };
        if let Some(n) = captain_count {
            mem.seed(0x6dd890, &[0, 0, n as u8, (n >> 8) as u8]);
        }
        let calls = Calls { towns: towns.len(), merchants: count, offices: offices.len(), towns_live: towns.clone(), church: self.host.church_cost_factor, room, letters: Vec::new(), removed_ships: Vec::new(), hire: Default::default(), rand_queue: Vec::new(), captain_seed: None, error: None };
        Ok(Flat { mem, calls, pool_header, scripts })
    }

    /// Publica el estado del puente: ciudades, comerciantes, oficinas, planificador y cartas pedidas.
    pub(crate) fn flat_publish(&mut self, mut flat: Flat) -> Result<(), String> {
        if let Some(e) = flat.calls.error.take() {
            return Err(e);
        }
        let mut scheduler = scheduler_from(&mut flat.mem);
        let mut variables = self.host.mission_variables.clone();
        for (i, payload, len) in &flat.scripts {
            let record = scheduler.records.get_mut(*i).ok_or("tarea de guion desaparecida")?;
            let ptr = record.u32_at(0xc);
            let vars = (0..*len).map(|k| flat.mem.r32(ptr + 4 * k)).collect::<Vec<_>>();
            record.payload[4..8].copy_from_slice(&payload[4..8]);
            if let Some(slot) = variables.get_mut(*i) {
                *slot = Some(vars);
            }
        }
        let towns = (0..flat.calls.towns)
            .map(|i| {
                let mut t = self.economy.world.towns[i].clone();
                let pointers = (t.0[0x75c..0x760].to_vec(), t.0[0x80c..0x810].to_vec());
                t.0.copy_from_slice(&flat.mem.read(TOWNS + TOWN_SIZE * i as u32, TOWN_SIZE as usize));
                // Los punteros a los planos sembrados no son estado de la partida.
                t.0[0x75c..0x760].copy_from_slice(&pointers.0);
                t.0[0x80c..0x810].copy_from_slice(&pointers.1);
                t
            })
            .collect::<Vec<_>>();
        let mut merchants = (0..flat.calls.merchants).map(|i| read_merchant(&mut flat.mem, MERCHANTS + MERCHANT_SIZE * i as u32)).collect::<Vec<_>>();
        let offices = (0..flat.calls.offices).map(|i| flat.mem.read(OFFICES + OFFICE_SIZE * i as u32, OFFICE_SIZE as usize)).collect::<Vec<_>>();
        let mut pool = match flat.pool_header {
            Some(header) => read_pool(&mut flat.mem, header),
            None => self.host.message_pool.clone(),
        };
        let date = self.idle.clock.calendar;
        for (code, record) in &flat.calls.letters {
            p3_state::messages::post_message(&mut pool, &mut merchants, self.economy.world.merchant_count, date.day, date.month, date.year, *code, record, p3_state::messages::Payload::None, &p3_state::messages::RETENTION)?;
        }
        if let Some(images) = self.host.scheduled_office_images.as_mut() {
            for (dst, src) in images.iter_mut().zip(offices) {
                dst[..OFFICE_SIZE as usize].copy_from_slice(&src);
            }
        }
        self.economy.world.towns = towns;
        self.c08_publish_merchants(merchants);
        self.host.message_pool = pool;
        self.host.mission_variables = variables;
        self.host.mission_variables.resize(usize::from(scheduler.capacity), None);
        self.host.council_scheduler = scheduler;
        Ok(())
    }
}

#[cfg(test)]
mod x86_tests {
    use super::*;
    use std::io::Write;

    fn hex(b: &[u8]) -> String {
        b.iter().map(|x| format!("{x:02x}")).collect()
    }

    /// `letter_free_and_reset` (`0x0051AAB0`) contra el ejecutable inglés: mismos 16 bytes de entrada, mismo resultado.
    /// El original además libera con `0x0064F7E2` los bloques externos de ciertos tipos; la memoria plana no los modela,
    /// así que se comprueban los bytes de la carta y que el oráculo no libere nada fuera de esos tipos.
    #[test]
    fn reset_letter_matches_original() {
        let (Ok(exe), Ok(py)) = (std::env::var("P3_EXE"), std::env::var("P3_ORACLE_PY")) else {
            return;
        };
        let mut seed = 0x51aab0u64;
        let mut next = move || {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            (seed >> 33) as u32
        };
        let kinds: Vec<u8> = (0..=0xffu8).collect();
        let mut cases = Vec::new();
        let mut want = Vec::new();
        for i in 0..1024usize {
            let mut entry: Vec<u8> = (0..16).map(|_| next() as u8).collect();
            entry[4] = kinds[i % kinds.len()];
            // punteros 0 o nulos en +8/+12 para los tipos que liberan: el gancho solo registra la llamada
            let mut m = SparseMem::default();
            m.seed(0x1000, &entry);
            reset_letter(&mut m, 0x1000);
            want.push(hex(&m.read(0x1000, 16)));
            cases.push(serde_json::json!({"entry":hex(&entry)}));
        }
        let script = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../oracle/x86_c10_small.py");
        let mut p = std::process::Command::new(py)
            .arg(script)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .unwrap();
        p.stdin
            .take()
            .unwrap()
            .write_all(serde_json::json!({"binary":exe,"fn":"reset_letter","cases":cases}).to_string().as_bytes())
            .unwrap();
        let out = p.wait_with_output().unwrap();
        assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
        let orig: Vec<serde_json::Value> = serde_json::from_slice(&out.stdout).unwrap();
        for (i, (o, w)) in orig.iter().zip(&want).enumerate() {
            // el enlace +6/+7 se conserva en el x86 y en Rust: se compara completo tras volver a poner +6/+7 de la entrada
            let got = o["entry"].as_str().unwrap();
            assert_eq!(&got[..12], &w[..12], "caso {i}: bytes 0..6");
            assert_eq!(&got[16..], &w[16..], "caso {i}: bytes 8..16");
            let entry_in = cases[i]["entry"].as_str().unwrap();
            assert_eq!(&got[12..16], &entry_in[12..16], "caso {i}: enlace +6 conservado por el x86");
        }
    }
}

impl Sim {
    /// Memoria del original con el estado vivo (mundo `0x006DE4A0`, ciudades, comerciantes, oficinas, planificador, buzón)
    /// en las posiciones del puente C08, para ejecutar otros portes que lo leen (p. ej. la interfaz en `p3_mvp::ui_rt`).
    pub fn original_memory_image(&self) -> Result<std::collections::BTreeMap<u32, u8>, String> {
        let mut flat = self.flat_seed([0; 5])?;
        // Tabla de capitanes viva en `[0x006DD7A0]` (registros de 16 bytes; el recuento `0x006DD892` ya viene del puente): la recorren,
        // p. ej., la taberna (`0x005269A0`) y el armero. Sin ella esos bucles leen ceros y no terminan.
        if let Some(factory) = self.host.expedition_factory.as_ref() {
            if !flat.mem.contains(0x006d_d7a0) {
                const CAPTAINS: u32 = 0x6200_0000;
                flat.mem.seed(CAPTAINS, &factory.captains.concat());
                flat.mem.seed(0x006d_d7a0, &CAPTAINS.to_le_bytes());
            }
        }
        // Tabla de identificadores de ciudad `0x006DE4B8` (un byte por posición): `0x00525630` (ciudad unida por tierra, ventana del granero)
        // y otras la indexan; sin ella todas las ciudades tienen id 0 y ninguna tiene socio por tierra.
        if !self.host.expedition_town_ids.is_empty() {
            flat.mem.seed(0x006d_e4b8, &self.host.expedition_town_ids);
        }
        // Gestor naval vivo: barcos (`+4`, 0x180 bytes) y convoyes (`+8`, 0x3C) del guardado, con los recuentos `+0xF4`/`+0xF6`. Los
        // recorren las páginas de la Hansa (convoyes públicos, `0x0050A340`) y del astillero (barco elegido). Las cinco bandas
        // piratas (`+0xC..+0x20`) no están en memoria: apuntan a direcciones sin sembrar, como en el guion plano de C10.
        if let Some(nav) = self.host.navigation.as_ref().map(|n| &n.snapshot).filter(|n| n.manager.len() >= 0xf8) {
            const SHIP_TABLE: u32 = 0x5c00_0000;
            const CONVOY_TABLE: u32 = 0x5d00_0000;
            const BAND_SENTINEL: u32 = 0x7f00_0000;
            let mut manager = nav.manager.clone();
            let captains = flat.mem.contains(0x006d_d7a0).then(|| flat.mem.read(0x006d_d7a0, 4));
            if let Some(c) = captains {
                manager[0..4].copy_from_slice(&c);
            }
            manager[4..8].copy_from_slice(&SHIP_TABLE.to_le_bytes());
            manager[8..12].copy_from_slice(&CONVOY_TABLE.to_le_bytes());
            for k in 0..5u32 {
                let at = 0xc + 4 * k as usize;
                manager[at..at + 4].copy_from_slice(&(BAND_SENTINEL + 0x1000 * k).to_le_bytes());
            }
            // El recuento de capitanes (`+0xF0..+0xF4`) ya viene del puente.
            if flat.mem.contains(0x006d_d892) {
                manager[0xf0..0xf4].copy_from_slice(&flat.mem.read(0x006d_d890, 4));
            }
            flat.mem.seed(0x006d_d7a0, &manager);
            flat.mem.seed(SHIP_TABLE, &nav.ships);
            flat.mem.seed(CONVOY_TABLE, &nav.convoys);
        }
        Ok(flat.mem.into_map())
    }
}
