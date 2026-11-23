//! C08: operaciones del registro de asedio en el despachador (`0x00535760`, casos 0x88 a 0x90 salvo 0x8B y 0x8E, que
//! están en `siege_daily`). La venta 0x89 siembra la tabla de barcos (`0x006DD7A4`) con las imágenes vivas y
//! recalcula la carga (`0x005182B0`) de los barcos de los que vende.
//!
//! El registro se siembra en el puente plano desde su forma guardada (`siege_record_serialize`, `0x006335A0`):
//! `+0` (4 bytes), `+4`, `+0x1018` (2), `+0x101A`, `+0x101B`, los pares `(+0x1010+k, +0x1014+k)` para k = 0..3, los
//! acuerdos de entrega (8 bytes, enlace `+8`) y los pedidos (16 bytes, enlace `+0x10`) en el orden de la lista. Tras el
//! puerto se vuelve a serializar igual; la escena (`+8`) no la tocan estas operaciones y se conserva tal cual.
use crate::c08_flat::{guarded, Calls, Flat};
use crate::Sim;
use p3_core::flatmem::{Mem, SparseMem};
use p3_core::pump::Operation;

const SIEGES: u32 = 0x6000_0000;
const STRIDE: u32 = 0x1024;
const NODES: u32 = 0x6100_0000;
const NODE_ROOM: u32 = 0x10_0000;
const SMALL_LOT_FLAGS: u32 = 0x672c14;
/// Objeto `TownSiege` (0x650 bytes) de la escena en curso, sembrado en `SiegeRecord+8`.
const SCENE: u32 = 0x6200_0000;
/// Comienzo de la parte viva del registro de asedio ([`crate::siege_view_live::RUNTIME`]).
const RUNTIME_AT: u32 = crate::siege_view_live::RUNTIME.start as u32;
/// Reloj de fotogramas de la vista (`0x006DCCF8`).
const FRAME_CLOCK: u32 = 0x006d_ccf8;
/// Factor de velocidad del juego (`0x006DFBC4`).
const SPEED_FACTOR: u32 = 0x006d_fbc4;
/// Tabla de barcos (`[0x006DD7A4]`, 0x180 bytes por barco).
const SHIPS: u32 = 0x6300_0000;
/// `0x00672C14`: 1 si la mercancía se mide en unidades de 200.
const WARE_SMALL_UNIT: [u8; 24] = [0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1];

/// Operaciones del registro de asedio que despacha este módulo.
pub const OPERATIONS: [u32; 7] = [0x88, 0x89, 0x8a, 0x8c, 0x8d, 0x8f, 0x90];

pub(crate) struct SiegeLayout {
    /// Bytes que siguen a las listas (indicador y escena).
    tail: Vec<u8>,
}

/// Memoria del puente sembrada para los fotogramas de la batalla: se reutiliza mientras no cambien el día ni los
/// registros de asedio guardados (la parte viva, la escena, el tick y el reloj de fotogramas se escriben en cada uno).
pub(crate) struct FrameCache {
    mem: SparseMem,
    layouts: Vec<SiegeLayout>,
    sieges: Vec<Vec<u8>>,
    day: (u16, u16),
}

fn seed_record(mem: &mut SparseMem, record: &[u8], city: usize, cursor: &mut u32) -> Result<SiegeLayout, String> {
    let len = p3_save::world::siege_record_len(record, city).map_err(|e| format!("registro de asedio: {e:?}"))?;
    if len != record.len() {
        return Err("longitud de asedio incompatible".into());
    }
    let r = SIEGES + STRIDE * city as u32;
    mem.seed(r, &vec![0u8; STRIDE as usize]);
    mem.seed(r, &record[0..4]);
    mem.w8(r + 4, record[4]);
    mem.w8(r + 0x1018, record[5]);
    mem.w8(r + 0x1019, record[6]);
    mem.w8(r + 0x101a, record[7]);
    mem.w8(r + 0x101b, record[8]);
    for k in 0..4u32 {
        mem.w8(r + 0x1010 + k, record[9 + 2 * k as usize]);
        mem.w8(r + 0x1014 + k, record[10 + 2 * k as usize]);
    }
    let agreements = usize::from(record[17]);
    let mut link = r + 0x101c;
    for row in record[18..18 + 8 * agreements].chunks_exact(8) {
        let node = *cursor;
        *cursor += 0x10;
        mem.seed(node, row);
        mem.seed(node + 8, &0u32.to_le_bytes());
        mem.w32(link, node);
        link = node + 8;
    }
    let at = 18 + 8 * agreements;
    let requests = usize::from(record[at]);
    let mut link = r + 0x1020;
    for row in record[at + 1..at + 1 + 16 * requests].chunks_exact(16) {
        let node = *cursor;
        *cursor += 0x20;
        mem.seed(node, row);
        mem.seed(node + 0x10, &0u32.to_le_bytes());
        mem.w32(link, node);
        link = node + 0x10;
    }
    // El indicador de escena se lee de `+8`: solo importa que no sea nulo.
    let tail = record[at + 1 + 16 * requests..].to_vec();
    if tail.first() != Some(&0) {
        mem.w32(r + 8, 1);
    }
    Ok(SiegeLayout { tail })
}

fn serialize_record(mem: &mut SparseMem, city: usize, layout: &SiegeLayout) -> Result<Vec<u8>, String> {
    let r = SIEGES + STRIDE * city as u32;
    let mut out = mem.read(r, 4);
    out.push(mem.r8(r + 4));
    out.extend_from_slice(&mem.r16(r + 0x1018).to_le_bytes());
    out.push(mem.r8(r + 0x101a));
    out.push(mem.r8(r + 0x101b));
    for k in 0..4 {
        out.push(mem.r8(r + 0x1010 + k));
        out.push(mem.r8(r + 0x1014 + k));
    }
    for (head, size, next) in [(0x101c, 8usize, 8u32), (0x1020, 16, 0x10)] {
        let mut rows = Vec::new();
        let mut p = mem.r32(r + head);
        while p != 0 {
            if rows.len() >= 255 {
                return Err("lista del asedio demasiado larga o cíclica".into());
            }
            rows.push(mem.read(p, size));
            p = mem.r32(p + next);
        }
        out.push(rows.len() as u8);
        out.extend(rows.concat());
    }
    out.extend_from_slice(&layout.tail);
    Ok(out)
}

/// Anfitrión de los puertos del registro: oficinas y cartas por el puente; reservas en la región de nodos.
struct SiegeHost<'a> {
    calls: &'a mut Calls,
    cursor: u32,
    /// Barcos cuya carga hay que recalcular (`0x005182B0`), en orden.
    reloaded: Vec<usize>,
}

/// Envoltura que da a los puertos acceso a la memoria y al anfitrión a la vez (la búsqueda de oficina lee memoria).
struct Bridge<'a> {
    mem: crate::c08_flat::SharedMem,
    host: SiegeHost<'a>,
}

impl p3_core::economy::siege_record::SupplyHost for Bridge<'_> {
    fn find_office(&mut self, merchant: u32, town: u32) -> u32 {
        let mut mem = self.mem.clone();
        self.host.calls.call(&mut mem, 0x5308a0, crate::c08_flat::WORLD, &[merchant, town, 0])
    }
    fn recompute_load(&mut self, ship: u32) {
        let index = ship.wrapping_sub(SHIPS) / 0x180;
        if ship < SHIPS || ship.wrapping_sub(SHIPS) % 0x180 != 0 {
            self.host.calls.error.get_or_insert_with(|| "recálculo de carga fuera de la tabla de barcos".into());
        } else if !self.host.reloaded.contains(&(index as usize)) {
            self.host.reloaded.push(index as usize);
        }
    }
}

impl p3_core::economy::siege_record::SupplyLetterHost for Bridge<'_> {
    fn send_letter(&mut self, merchant: u32, record: &[u8; 16]) {
        self.host.calls.letters.push((merchant as i32, *record));
    }
}

impl p3_core::economy::siege_record::AgreementHost for Bridge<'_> {
    fn alloc(&mut self, size: u32) -> u32 {
        let at = self.host.cursor;
        self.host.cursor += (size + 0xf) & !0xf;
        if self.host.cursor > NODES + NODE_ROOM {
            self.host.calls.error.get_or_insert_with(|| "reserva de nodos del asedio agotada".into());
        }
        self.mem.0.borrow_mut().seed(at, &vec![0u8; size as usize]);
        at
    }
}

/// Anfitrión de las órdenes sobre la escena (`0x00634660`, `0x006347B0`, `0x00634820`) sin vista: sin vista
/// (`TownSiege+0x77 == 0`) el original no mueve unidades; los movimientos se rechazan.
struct OrderBridge<'a> {
    mem: crate::c08_flat::SharedMem,
    calls: &'a mut Calls,
    cities: u32,
    /// Vista abierta por el cliente: los movimientos usan los portes de `siege_view` sobre la escena `SCENE`.
    view: Option<&'a mut crate::siege_view_live::SiegeTownView>,
}

impl p3_core::economy::siege_record::OrderHost for OrderBridge<'_> {
    fn town_event(&mut self, town: u32, amount: u32) {
        // `Town::book_transaction(importe, 8)` (0x00529F90) sobre la ciudad sembrada.
        let mut mem = self.mem.clone();
        let tick = mem.r32(0x6de4b4);
        let mut t = p3_core::town::Town(self.mem.0.borrow().read(town, 0x9f8).try_into().expect("ciudad de 0x9F8 bytes"));
        p3_core::economy::militia::book_transaction(&mut t, tick, amount as i32, 8);
        self.mem.0.borrow_mut().seed(town, &t.0[..]);
    }
    fn move_to(&mut self, a: u16, b: u16) {
        let Some(view) = self.view.as_deref_mut() else {
            self.calls.error.get_or_insert_with(|| "orden de asedio con vista (0x0062B860) sin vista abierta".into());
            return;
        };
        let mut mem = self.mem.clone();
        if let Err(e) = view.run(&mut mem, |m, h| p3_core::economy::siege_view::reset_flow_field_to_cell(m, h, SCENE, u32::from(a), u32::from(b), 0)) {
            self.calls.error.get_or_insert(e);
        }
    }
    fn refresh_unit(&mut self, unit: u32) {
        let Some(view) = self.view.as_deref_mut() else {
            self.calls.error.get_or_insert_with(|| "orden de asedio con vista (0x0062DD70) sin vista abierta".into());
            return;
        };
        let mut mem = self.mem.clone();
        // `0x0062DD70` usa el campo de rutas sin comprobarlo; en el original lo reservó la apertura (`0x0062C730`).
        if mem.r32(unit + 0x59c) == 0 {
            self.calls.error.get_or_insert_with(|| "marcadores del asedio sin campo de rutas (lo reserva 0x0062C730, sin portar)".into());
            return;
        }
        if let Err(e) = view.run(&mut mem, |m, h| p3_core::economy::siege_view::update_entry_point_markers(m, h, unit, true)) {
            self.calls.error.get_or_insert(e);
        }
    }
    fn send_letter(&mut self, record: &[u8; 16]) {
        self.calls.letters.push((-2, *record));
    }
    fn finish_order(&mut self, r: u32) {
        // `siege_record_refill_goodwill(1)` (0x00634C50): marca las peticiones hacia la ciudad en todos los registros.
        let mut mem = self.mem.clone();
        let city = mem.r8(r + 4);
        let mut heads = Vec::new();
        for c in 0..self.cities {
            let mut p = mem.r32(SIEGES + STRIDE * c + 0x1020);
            while p != 0 {
                heads.push(p);
                p = mem.r32(p + 0x10);
            }
        }
        let mut rows: Vec<[u8; 16]> = heads.iter().map(|&p| self.mem.0.borrow().read(p, 16).try_into().unwrap()).collect();
        let mut goodwill = mem.r16(r + 0x1018);
        let town = mem.r32(0x6de508).wrapping_add(u32::from(city) * 0x9f8);
        let population = mem.r32(town + 0x2d4) as i32;
        let mut groups = [std::mem::take(&mut rows)];
        p3_core::economy::siege::refill_goodwill(city, &mut goodwill, mem.r8(r + 0x101b), population, mem.r32(0x6de4b4), true, &mut groups, 0);
        for (&p, row) in heads.iter().zip(&groups[0]) {
            self.mem.0.borrow_mut().seed(p, row);
        }
        mem.w16(r + 0x1018, goodwill);
    }
}

impl Sim {
    /// Objeto de escena tal como lo tiene el original: la imagen viva o, si no la hay, los campos guardados con los
    /// indicadores que reconstruye la carga (`0x00627AD0`: `+0x78` visibilidad, `+0x79` retirada).
    fn siege_scene_image(&self, city: usize, record: &[u8]) -> Result<Vec<u8>, String> {
        let saved = p3_save::world::siege_scene(record, city)
            .map_err(|e| format!("escena: {e:?}"))?
            .ok_or("orden de asedio sin escena: el original desreferencia un objeto nulo")?;
        if let Some(open) = self.host.siege_views.get(city).and_then(Option::as_ref).and_then(|v| v.scene.as_ref()) {
            if p3_save::siege_scene::encode(open).map_err(str::to_string)? != saved {
                return Err("escena de la vista y guardada divergentes".into());
            }
            return Ok(open.clone());
        }
        if let Some(live) = self.host.siege_walled_live.get(city).and_then(Option::as_ref) {
            if p3_save::siege_scene::encode(&live.image).map_err(str::to_string)? != saved {
                return Err("escena viva y guardada divergentes".into());
            }
            return Ok(live.image.clone());
        }
        let mut image = vec![0u8; 0x650];
        image[0x5d3] = city as u8;
        p3_save::siege_scene::decode_fields(saved, &mut image).map_err(str::to_string)?;
        let word = |at: usize| u16::from_le_bytes([image[at], image[at + 1]]);
        let visible = word(0x3c) != 0 || word(0x3a) != 0;
        let retreat = self.host.siege_retreat_flags.get(city).copied().flatten().ok_or("indicador de retirada de la escena ausente")?;
        image[0x78] = u8::from(visible);
        image[0x79] = u8::from(visible && retreat);
        image[0x578] = 0xff;
        Ok(image)
    }

    /// Imagen viva de un barco (navegación o flota fuera de ella) y su carga, comprobando que coinciden.
    fn siege_ship_image(&self, i: usize) -> Result<Option<&[u8]>, String> {
        let image = if let Some(nav) = &self.host.navigation {
            nav.snapshot.ships.get(i * 0x180..(i + 1) * 0x180)
        } else {
            self.host.expedition_ship_images.get(i).and_then(Option::as_deref)
        };
        let Some(b) = image else { return Ok(None) };
        if b.len() < 0x180 {
            return Err("imagen de barco truncada".into());
        }
        let cargo = self.economy.cargo_ships.get(i).ok_or("carga naval ausente")?;
        if cargo.town != b[0x39] || (0..24).any(|w| i32::from_le_bytes(b[0x54 + 4 * w..0x58 + 4 * w].try_into().unwrap()) != cargo.stock[w]) {
            return Err("carga naval desincronizada".into());
        }
        Ok(Some(&b[..0x180]))
    }

    /// Tabla de barcos para la venta al asedio; exige imagen para cada barco de la lista del vendedor.
    fn siege_ship_table(&self, head: u16) -> Result<Vec<u8>, String> {
        let count = usize::from(self.economy.world.ship_count);
        if self.economy.cargo_ships.len() != count {
            return Err("flota incompleta".into());
        }
        let mut table = vec![0u8; count * 0x180];
        for i in 0..count {
            if let Some(b) = self.siege_ship_image(i)? {
                table[i * 0x180..(i + 1) * 0x180].copy_from_slice(b);
            }
        }
        let (mut i, mut seen) = (usize::from(head), vec![false; count]);
        while i < count {
            if std::mem::replace(&mut seen[i], true) {
                return Err("ciclo en la lista de barcos del vendedor".into());
            }
            if self.siege_ship_image(i)?.is_none() {
                return Err("barco del vendedor sin imagen".into());
            }
            i = usize::from(u16::from_le_bytes([table[i * 0x180 + 4], table[i * 0x180 + 5]]));
        }
        Ok(table)
    }

    /// Publica la carga vendida de un barco y su recálculo (`0x005182B0`) en la carga y las imágenes vivas.
    fn publish_siege_ship(&mut self, i: usize, image: &[u8]) -> Result<(), String> {
        let cargo = self.economy.cargo_ships.get_mut(i).ok_or("carga naval ausente")?;
        for w in 0..24 {
            cargo.stock[w] = i32::from_le_bytes(image[0x54 + 4 * w..0x58 + 4 * w].try_into().unwrap());
        }
        cargo.recompute_load();
        let s = cargo.clone();
        let write = |b: &mut [u8]| {
            for w in 0..24 {
                b[0x54 + 4 * w..0x58 + 4 * w].copy_from_slice(&s.stock[w].to_le_bytes());
            }
            for (p, v) in [(0x114, s.cargo_value), (0x118, s.cargo_weight), (0x11c, s.equipment_weight), (0x120, s.gun_power)] {
                b[p..p + 4].copy_from_slice(&v.to_le_bytes());
            }
        };
        if let Some(b) = self.host.expedition_ship_images.get_mut(i).and_then(Option::as_mut) {
            write(b)
        }
        if let Some(nav) = self.host.navigation.as_mut() {
            write(&mut nav.snapshot.ships[i * 0x180..(i + 1) * 0x180])
        }
        if !self.host.debt_changed_ships.contains(&i) {
            self.host.debt_changed_ships.push(i)
        }
        Ok(())
    }

    pub(crate) fn apply_siege_record_operation(&mut self, op: Operation) -> Option<Result<(), String>> {
        let code = op.0[0];
        if !OPERATIONS.contains(&code) {
            return None;
        }
        let bytes = op.0.iter().flat_map(|w| w.to_le_bytes()).collect::<Vec<_>>();
        let city = match code {
            0x89 | 0x8a => u32::from(bytes[0xe]),
            0x8f => u32::from(bytes[0x10]),
            0x90 => u32::from(bytes[8]),
            _ => op.0[1],
        } as usize;
        // El original ignora la operación con una ciudad fuera de la tabla.
        if city >= self.economy.world.towns.len() {
            return Some(Ok(()));
        }
        Some(self.siege_record_operation(op, city, &bytes))
    }

    fn siege_record_operation(&mut self, op: Operation, city: usize, bytes: &[u8]) -> Result<(), String> {
        use p3_core::economy::siege_record as sr;
        self.c08_merchant_views_synced()?;
        let record = self.host.founded_world.as_ref().ok_or("mundo de asedio ausente")?.sieges.get(city).ok_or("registro de asedio ausente")?.clone();
        let ships = if op.0[0] == 0x89 {
            let merchant = self.rep_merchants.get(usize::from(bytes[0xd])).ok_or("vendedor del asedio fuera de la tabla")?;
            let head = merchant.u16(0xe);
            if head < self.economy.world.ship_count { Some(self.siege_ship_table(head)?) } else { None }
        } else {
            None
        };
        // 0x90 con libertad (`op+4`), rendición o ataque (`op+9 == op+0xA == 0`) actúa sobre el objeto de escena.
        let scene_order = op.0[0] == 0x90 && (op.0[1] != 0 || (bytes[9] == 0 && bytes[0xa] == 0));
        let scene = if scene_order { Some(self.siege_scene_image(city, &record)?) } else { None };
        let mut flat: Flat = self.flat_seed(op.0)?;
        flat.mem.seed(0x6dd894, &self.economy.world.ship_count.to_le_bytes());
        let mut cursor = NODES;
        // Todos los registros: el cierre de la orden (0x00634C50) recorre las peticiones de todas las ciudades.
        let all = self.host.founded_world.as_ref().unwrap().sieges.clone();
        let mut layouts = Vec::with_capacity(all.len());
        for (c, rec) in all.iter().enumerate() {
            layouts.push(seed_record(&mut flat.mem, rec, c, &mut cursor)?);
        }
        if let Some(table) = &ships {
            flat.mem.seed(SHIPS, table);
            flat.mem.seed(0x6dd7a4, &SHIPS.to_le_bytes());
        }
        if let Some(image) = &scene {
            flat.mem.seed(SCENE, image);
            flat.mem.w32(SIEGES + STRIDE * city as u32 + 8, SCENE);
        }
        flat.mem.seed(0x6df270, &SIEGES.to_le_bytes());
        flat.mem.seed(SMALL_LOT_FLAGS, &WARE_SMALL_UNIT);
        let r = SIEGES + STRIDE * city as u32;
        // Con la vista abierta, la parte viva del registro (figuras de la batalla) vuelve a su sitio.
        if let Some(v) = self.host.siege_views.get(city).and_then(Option::as_ref) {
            flat.mem.seed(r + RUNTIME_AT, &v.runtime);
        }
        let mut town = None;
        let shared = crate::c08_flat::SharedMem(std::rc::Rc::new(std::cell::RefCell::new(std::mem::take(&mut flat.mem))));
        let mut reloaded = Vec::new();
        let mut view = self.host.siege_views.get_mut(city).and_then(Option::take);
        let result = {
            let mut bridge = Bridge { mem: shared.clone(), host: SiegeHost { calls: &mut flat.calls, cursor, reloaded: Vec::new() } };
            let mut mem = shared.clone();
            let outcome = guarded(|| match op.0[0] {
                // Regalo de la ciudad: transacción 10 por lo pedido (con tope en su tesoro) y actitud.
                0x88 => {
                    let mut t = self.economy.world.towns[city].clone();
                    let amount = op.0[2].min(t.u32(0x700));
                    p3_core::economy::militia::book_transaction(&mut t, self.idle.clock.tick, amount as i32, 0xa);
                    town = Some(t);
                    sr::add_goodwill_from_gift(&mut mem, r, amount);
                }
                0x89 => sr::sell_wares(&mut mem, &mut bridge, r, u32::from(bytes[0xd]), u32::from(bytes[0xc]), op.0[1], op.0[2]),
                0x8a => sr::add_delivery_agreement(&mut mem, &mut bridge, r, u32::from(bytes[0xc]), u32::from(bytes[0xd]), u32::from(u16::from_le_bytes([bytes[4], bytes[5]])), u32::from(u16::from_le_bytes([bytes[8], bytes[9]])), u32::from(bytes[0xf])),
                0x8c => sr::accept_supply_request_and_mark_letter(&mut mem, &mut bridge, crate::c08_flat::OP),
                0x8d => sr::deliver_supply_request(&mut mem, &mut bridge, r, u32::from(bytes[8]), u32::from(bytes[0xc]), op.0[4]),
                0x8f => {
                    if let Err(e) = p3_core::economy::c08_siege_levy::levy_town_contribution(&mut mem, bridge.host.calls, crate::c08_flat::OP) {
                        bridge.host.calls.error.get_or_insert(e.to_string());
                    }
                }
                0x90 if scene_order => {
                    let mut orders = OrderBridge { mem: shared.clone(), calls: bridge.host.calls, cities: all.len() as u32, view: view.as_mut() };
                    if op.0[1] != 0 {
                        sr::buy_freedom(&mut mem, &mut orders, r, op.0[1]);
                    } else if bytes[0xb] != 0 {
                        sr::order_surrender(&mut mem, &mut orders, r);
                    } else {
                        sr::order_attack(&mut mem, &mut orders, r);
                    }
                }
                0x90 => {
                    if let Err(e) = p3_core::economy::c08_siege_levy::siege_dialog_action(&mut mem, crate::c08_flat::OP) {
                        bridge.host.calls.error.get_or_insert(e.to_string());
                    }
                }
                _ => unreachable!(),
            });
            reloaded = std::mem::take(&mut bridge.host.reloaded);
            outcome
        };
        if let Some(v) = view.take() {
            self.host.siege_views[city] = Some(v);
        }
        flat.mem = std::rc::Rc::try_unwrap(shared.0).map_err(|_| "memoria del puente aún compartida")?.into_inner();
        result?;
        if let Some(v) = self.host.siege_views.get_mut(city).and_then(Option::as_mut) {
            v.runtime = flat.mem.read(r + RUNTIME_AT, v.runtime.len());
        }
        if let Some(e) = flat.calls.error.take() {
            return Err(e);
        }
        let mut updated = Vec::with_capacity(all.len());
        for (c, layout) in layouts.iter().enumerate() {
            updated.push(serialize_record(&mut flat.mem, c, layout)?);
        }
        let reloaded_ships = reloaded.iter().map(|&i| (i, flat.mem.read(SHIPS + 0x180 * i as u32, 0x180))).collect::<Vec<_>>();
        let image = scene.as_ref().map(|_| flat.mem.read(SCENE, 0x650));
        let open = self.host.siege_views.get(city).and_then(Option::as_ref).is_some_and(|v| v.scene.is_some());
        if let Some(image) = &image {
            if image[0x77] != 0 && !open {
                return Err("escena de asedio con vista".into());
            }
            replace_scene(&mut updated[city], city, image)?;
        }
        self.flat_publish(flat)?;
        if let Some(t) = town {
            self.economy.world.towns[city] = t;
        }
        for (c, rec) in updated.into_iter().enumerate() {
            if self.host.founded_world.as_ref().unwrap().sieges[c] != rec {
                let before = self.host.siege_version(c);
                self.host.founded_world.as_mut().unwrap().sieges[c] = rec;
                self.host.remember_siege_version(c, before);
            }
        }
        for (i, image) in reloaded_ships {
            self.publish_siege_ship(i, &image)?;
        }
        if let Some(mut image) = image {
            if open {
                // La escena viva queda en la vista; fuera de ella se conserva la cerrada con los campos guardados.
                let view = self.host.siege_views[city].as_mut().unwrap();
                view.scene = Some(image.clone());
                let mut closed = view.closed.clone().ok_or("escena cerrada de la vista ausente")?;
                p3_save::siege_scene::decode_fields(&p3_save::siege_scene::encode(&image).map_err(str::to_string)?, &mut closed).map_err(str::to_string)?;
                view.closed = Some(closed.clone());
                image = closed;
            }
            if let Some(live) = self.host.siege_walled_live.get_mut(city).and_then(Option::as_mut) {
                live.image = image.clone();
            }
            if self.host.siege_retreat_flags.len() <= city {
                self.host.siege_retreat_flags.resize(city + 1, None);
            }
            self.host.siege_retreat_flags[city] = Some(image[0x79] != 0);
        }
        Ok(())
    }
}

/// Sustituye la escena guardada al final del registro de asedio de `city` por los campos de `image`.
fn replace_scene(rec: &mut Vec<u8>, city: usize, image: &[u8]) -> Result<(), String> {
    let saved = p3_save::world::siege_scene(rec, city).map_err(|e| format!("escena: {e:?}"))?.ok_or("escena ausente")?.len();
    let at = rec.len() - saved;
    rec.truncate(at);
    rec.extend(p3_save::siege_scene::encode(image).map_err(str::to_string)?);
    Ok(())
}

impl crate::Sim {
    /// Abre la vista de ciudad de `city` durante su asedio: la escena pasa a tener vista (`+0x77 = 1`) y se ejecuta
    /// la apertura completa `0x0062C730`: objetivos, puntos de entrada, puerta, jefe del asalto, campo de rutas hacia el
    /// destino de la fase, defensores, asaltantes, marcadores de puerta, milicia y reencolado. Si el asedio no está a
    /// la vista, la misma función la desmonta (`+0x77 = 0`) y las órdenes siguen el camino sin vista. Con la vista
    /// abierta las órdenes `0x90` de la ciudad mueven a los asaltantes con el campo de rutas y los marcadores, y
    /// [`Self::step_siege_view`] avanza la batalla. Ver los límites en [`crate::siege_view_live`].
    pub fn open_siege_view(&mut self, view: crate::siege_view_live::SiegeTownView) -> Result<(), String> {
        let city = view.city;
        if self.host.siege_views.get(city).and_then(Option::as_ref).is_some() {
            return Err("vista de asedio ya abierta".into());
        }
        let record = self.host.founded_world.as_ref().ok_or("mundo de asedio ausente")?.sieges.get(city).ok_or("registro de asedio ausente")?.clone();
        let closed = self.siege_scene_image(city, &record)?;
        let mut image = closed.clone();
        if self.host.siege_walled_live.get(city).and_then(Option::as_ref).is_none() {
            // Puertas que la carga original (`0x00627AD0`) busca para una escena a la vista: sin muralla
            // (`0x005252F0` devuelve 0xFF) no hay puerta (`+8 = 0xFFFF`).
            let town = self.economy.world.towns.get(city).ok_or("ciudad ausente")?;
            let site = self.host.sites.get(city).ok_or("plano ausente")?;
            let level = site.wall_expansion_level(town);
            if level == 0xff {
                image[8..10].copy_from_slice(&0xffffu16.to_le_bytes());
            } else {
                let gates = p3_core::economy::siege::choose_wall_gate(&site.records, town.u16(0x768), town.u16(0x762), level, 0).map_err(str::to_string)?;
                for (i, gate) in gates.gates.iter().enumerate() {
                    image[0x244 + 4 * i..0x248 + 4 * i].copy_from_slice(&gate.to_le_bytes());
                }
            }
        }
        image[0x77] = 1;
        let mut view = view;
        view.closed = Some(closed);
        let (view, image, record) = self.run_live_scene(view, image, 0, false, crate::siege_view_live::open_scene)?;
        self.publish_live_scene(view, image, record)
    }

    /// Un fotograma de la batalla de la vista abierta de `city` (`0x00631A50`, desde `town_view_update_frame`) con
    /// el reloj de fotogramas `frame_clock` (`0x006DCCF8`): cierre si se fue la vista, desmontaje y nueva apertura
    /// cada 0x100 ticks, cabecillas, milicia de refresco, humo y fuego de la torre.
    pub fn step_siege_view(&mut self, city: usize, frame_clock: u32) -> Result<(), String> {
        let view = self.host.siege_views.get_mut(city).and_then(Option::take).ok_or("vista de asedio cerrada")?;
        let Some(image) = view.scene.clone() else {
            self.host.siege_views[city] = Some(view);
            return Err("vista de asedio sin escena".into());
        };
        match self.run_live_scene(view, image, frame_clock, true, crate::siege_view_live::frame_scene) {
            Ok((view, image, record)) => self.publish_live_scene(view, image, record),
            Err(e) => Err(e),
        }
    }

    /// Cierra la vista de `city` (`0x0062F5A0` con destrucción: retira las figuras, libera campo y cola, `+0x77 = 0`)
    /// y la devuelve; la escena guardada recibe los campos que guarda el juego.
    pub fn close_siege_view(&mut self, city: usize) -> Result<Option<crate::siege_view_live::SiegeTownView>, String> {
        let Some(view) = self.host.siege_views.get_mut(city).and_then(Option::take) else { return Ok(None) };
        let Some(image) = view.scene.clone() else { return Ok(Some(view)) };
        let (view, image, record) = self.run_live_scene(view, image, 0, false, crate::siege_view_live::close_scene)?;
        self.publish_live_scene(view, image, record)?;
        Ok(self.host.siege_views.get_mut(city).and_then(Option::take))
    }

    /// Ejecuta `f` sobre la escena `image` de la vista con la memoria del puente C08: ciudades, registros de edificio,
    /// plano, reloj, todos los registros de asedio (`[0x006DF270]`, la escena en `+8` y la parte viva de la vista) y
    /// el reloj de fotogramas. Devuelve la vista, la escena y el registro de la ciudad serializado con ella.
    fn run_live_scene(
        &mut self,
        mut view: crate::siege_view_live::SiegeTownView,
        image: Vec<u8>,
        frame_clock: u32,
        cached: bool,
        f: fn(&mut crate::siege_view_live::SiegeTownView, &mut dyn Mem, u32) -> Result<(), String>,
    ) -> Result<(crate::siege_view_live::SiegeTownView, Vec<u8>, Vec<u8>), String> {
        // Solo los fotogramas (`cached`) reutilizan la memoria; apertura y cierre la siembran de nuevo.
        let city = view.city;
        let mut run = || -> Result<(Vec<u8>, Vec<u8>), String> {
            let all = self.host.founded_world.as_ref().ok_or("mundo de asedio ausente")?.sieges.clone();
            let date = self.idle.clock.calendar;
            let day = (date.year, date.day_of_year);
            let mut cache = view.cache.take().filter(|c| cached && c.sieges == all && c.day == day);
            if cache.is_none() {
                let mut flat = self.flat_seed([0x90, 0, city as u32, 0, 0])?;
                let mut cursor = NODES;
                let mut layouts = Vec::with_capacity(all.len());
                for (c, rec) in all.iter().enumerate() {
                    layouts.push(seed_record(&mut flat.mem, rec, c, &mut cursor)?);
                }
                flat.mem.seed(0x6df270, &SIEGES.to_le_bytes());
                cache = Some(FrameCache { mem: flat.mem, layouts, sieges: all, day });
            }
            let mut c = cache.unwrap();
            c.mem.seed(crate::c08_flat::WORLD + 0x14, &self.idle.clock.tick.to_le_bytes());
            // Milisegundos por tick (`OPS+0x8D4`, `0x006DFBC4`): constructor 0x00546450 y opcode 0xC8.
            c.mem.seed(SPEED_FACTOR, &self.host.time_mode.pump.ms_per_tick.to_le_bytes());
            let r = SIEGES + STRIDE * city as u32;
            c.mem.seed(SCENE, &image);
            c.mem.w32(r + 8, SCENE);
            c.mem.seed(r + RUNTIME_AT, &view.runtime);
            c.mem.seed(FRAME_CLOCK, &frame_clock.to_le_bytes());
            // `rand` del CRT: el mismo flujo que el resto de la simulación cuando está configurado (un solo hilo en el
            // original). Viento del puerto con la orientación del mar de la ciudad (`0x006DE4B8` → `TownData+0x31`).
            let shared = self.host.investigation_rng;
            if let Some(rng) = shared {
                view.rand_state = rng.state;
            }
            view.sea_direction = self.host.settlement_sea_directions.as_ref().zip(self.host.expedition_town_ids.get(city)).and_then(|(sea, &id)| sea.get(usize::from(id)).copied());
            f(&mut view, &mut c.mem, SCENE)?;
            if shared.is_some() {
                self.host.investigation_rng = Some(p3_core::rng::CrtRand { state: view.rand_state });
            }
            let image = c.mem.read(SCENE, 0x650);
            view.runtime = c.mem.read(r + RUNTIME_AT, view.runtime.len());
            let mut record = serialize_record(&mut c.mem, city, &c.layouts[city])?;
            replace_scene(&mut record, city, &image)?;
            if cached {
                // El registro guardado cambia con la escena: la memoria sigue valiendo para el siguiente fotograma.
                c.sieges[city] = record.clone();
                view.cache = Some(c);
            }
            Ok((image, record))
        };
        match run() {
            Ok((image, record)) => Ok((view, image, record)),
            Err(e) => {
                // La vista sigue abierta con su escena anterior.
                if self.host.siege_views.len() <= city {
                    self.host.siege_views.resize_with(city + 1, || None);
                }
                self.host.siege_views[city] = Some(view);
                Err(e)
            }
        }
    }

    /// Publica la escena viva de la vista: el registro guardado recibe sus campos, la escena cerrada (la que siguen
    /// los portes sin vista) también, y la vista vuelve a su sitio.
    fn publish_live_scene(&mut self, mut view: crate::siege_view_live::SiegeTownView, image: Vec<u8>, record: Vec<u8>) -> Result<(), String> {
        let city = view.city;
        let mut closed = view.closed.clone().ok_or("escena cerrada de la vista ausente")?;
        p3_save::siege_scene::decode_fields(&p3_save::siege_scene::encode(&image).map_err(str::to_string)?, &mut closed).map_err(str::to_string)?;
        if let Some(live) = self.host.siege_walled_live.get_mut(city).and_then(Option::as_mut) {
            live.image = closed.clone();
        }
        view.scene = Some(image);
        view.closed = Some(closed);
        if self.host.founded_world.as_ref().ok_or("mundo de asedio ausente")?.sieges[city] != record {
            let before = self.host.siege_version(city);
            self.host.founded_world.as_mut().unwrap().sieges[city] = record;
            self.host.remember_siege_version(city, before);
        }
        if self.host.siege_views.len() <= city {
            self.host.siege_views.resize_with(city + 1, || None);
        }
        self.host.siege_views[city] = Some(view);
        Ok(())
    }

    /// Vista de asedio abierta de `city`.
    pub fn siege_view(&self, city: usize) -> Option<&crate::siege_view_live::SiegeTownView> {
        self.host.siege_views.get(city).and_then(Option::as_ref)
    }
}
