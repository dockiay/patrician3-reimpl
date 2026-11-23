//! Vista de ciudad abierta durante un asedio (`TownSiege+0x77 != 0`).
//!
//! Sin pantalla la escena del asedio nunca tiene vista y las órdenes `0x90` que mueven a los asaltantes se
//! rechazan. Un cliente con la ciudad cargada abre una [`SiegeTownView`] con la rejilla real del mapa (capas 0 y
//! 1 de la vista, clase de cada ficha), los puntos de reunión (lista 4 del `.spawn`) y las casillas de los tres
//! caminos de puertas (`.nodes`); desde ese momento las órdenes de la ciudad ejecutan los portes con paridad x86
//! de `p3_core::economy::siege_view`: reinicio y siembra del campo de rutas (`0x0062B860`, `0x0062BE90`, cola
//! `0x00632A20`) y marcadores de los puntos de entrada (`0x0062DD70`).
//!
//! La vista vive en memoria propia en [`VIEW_BASE`] con la disposición del objeto `Class22` del original:
//! dimensiones `+0xC314/+0xC318`, capas `+0xC308/+0xC30C` (celdas de 4 bytes), tabla de fichas `+0x98` (objetos
//! `[vtabla, clase]`, el método virtual `+0x50` devuelve la clase), puntos de reunión `+0xC698/+0xC6AC` y tabla de
//! caminos `+0xC674`. El campo de rutas y la cola se reservan en un montón propio tras la vista; los punteros de la
//! escena (`+0x59C`, `+0x5D8`) apuntan ahí. Los marcadores (`0x00590610`, clase `0x12`) son [`Marker`] del
//! cliente, que los dibuja.
//!
//! Abrir la vista ejecuta `town_siege_rebuild_view_state` (`0x0062C730`) completo con paridad x86
//! ([`siege_battle::rebuild_view_state`]): camino de acceso, puntos de reunión, carriles, entrada exterior, máscara,
//! casillas de la puerta, jefe, campo de rutas hacia el destino de la fase, defensores, asaltantes, marcadores de
//! puerta, milicia y reencolado. [`crate::Sim::step_siege_view`] ejecuta el fotograma `0x00631A50` y
//! [`crate::Sim::close_siege_view`] el cierre `0x0062F5A0`, que retira las figuras: el destructor de cada entidad
//! (`0x00593D50`) las quita de las listas con [`siege_battle::remove_destroyed_entity`]. La parte viva del registro de
//! asedio ([`RUNTIME`]: colas de la puerta y listas de figuras), que el juego no guarda, vive en la vista entre
//! llamadas.
//!
//! Límites (ver `CENSO_C08.md`): las figuras no se mueven ni combaten (sus métodos por fotograma, `0x005674F0`,
//! `0x005710A0`, `0x00577950` y demás, no están portados) ni se apuntan en la capa de enlaces `+0xC310`; los
//! transeúntes que huyen (`0x00598150`) solo se anotan; las entidades guardan su posición en `+0xC/+0x10`; `rand` es
//! el flujo del CRT de la simulación (`investigation_rng`) cuando está configurado y si no un LCG propio de la vista;
//! el viento es el de la escena del puerto (`0x00618D10`, [`HarborWind`]) con la orientación del mar de la ciudad,
//! sin las consultas que hace el propio puerto al dibujarse; `0x006DFBC4` es el intervalo del tick de la simulación
//! (`OPS+0x8D4`, 234 ms a velocidad normal, [`crate::NullHost::time_mode`]).
use std::collections::BTreeMap;

use p3_core::economy::harbor_wind::HarborWind;
use p3_core::economy::siege_battle;
use p3_core::economy::siege_figures::{entities, vcall, FigureHost};
use p3_core::economy::siege_view::{SiegeViewHost, VIEW};
use p3_core::flatmem::Mem;

/// Dirección de la vista en la memoria de las órdenes.
pub const VIEW_BASE: u32 = 0x6400_0000;
const HEADER: u32 = 0xCA00;
/// Montón para los conjuntos de sprites de las figuras (tablas de fotogramas de todas las clases) y sus objetos.
pub const SPRITE_SPACE: u32 = 0x80_0000;
/// Bytes de cada entidad de la vista en el montón propio (las figuras usan hasta `+0x89`).
const ENTITY_SIZE: u32 = 0x100;
/// Bytes vivos del registro de asedio de la ciudad que no se guardan (`+0xC..+0x1010`: colas de la puerta, listas de
/// figuras, cuántas y contadores por puerta).
pub const RUNTIME: std::ops::Range<usize> = 0xC..0x1010;
/// Tablas globales de las clases de figuras (conjuntos de sprites por clase, `0x006E1600..0x006E3000`), propias de la vista.
pub const GLOBALS: std::ops::Range<u32> = 0x006E_1600..0x006E_3000;
/// Clases cuyas figuras son objetos vivos de la vista (aparición `+0x40`, bucle `+0x14` y baja `+0x44` portados):
/// luchadores de la cola, arqueros, ballesteros, arcabuceros, efectos (humo, polvo, marcadores), proyectiles,
/// lanceros, infantería y jinetes.
pub const LIVE_CLASSES: [u32; 11] = [8, 9, 0xA, 0xB, 0x12, 0x15, 0x16, 0x17, 0x20, 0x23, 0x24];

/// Geometría de la rejilla de la vista que el cliente conoce (`0x0058B740`, `0x0058D030`, `0x0058CFB0`).
pub trait SiegeViewClient {
    fn step(&mut self, col: &mut u32, row: &mut u32, dir: u8) -> bool;
    fn pixel_to_cell(&mut self, x: i32, y: i32) -> (u32, u32);
    fn cell_to_pixel(&mut self, col: u32, row: u32) -> (i32, i32);
    /// Viento del mapa local (`0x00618D10`): dirección y fuerza (byte bajo de cada uno).
    fn wind(&mut self) -> (u32, u32) {
        (0, 0)
    }
}

/// Entidad creada en la vista por `0x00590610(x << 16, y << 16, clase, variante, 0x20, 0, 0)`: marcadores de los
/// puntos de entrada (clase `0x12`, variante `0x80` con subtipo) y jefe del asalto (clase `0x16`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Marker {
    pub entity: u32,
    /// Píxel del mundo de la casilla (`0x0058CFB0`).
    pub x: i32,
    pub y: i32,
    /// Clase de entidad (`0x12`, efecto) y variante (`0x80` con subtipo en el byte alto).
    pub class: u32,
    pub variant: u32,
    /// Último fundido pedido (`0x0057AF40(entidad, a, b)`): `(0, 0x100)` aparece, `(0, 0xFFFFFF00)` se apaga.
    pub fade: Option<(u32, u32)>,
    /// Rumbo (el de creación, `0x00590610` quinto argumento; en las figuras vivas, el actual `+0x14`).
    pub heading: u8,
    /// Figura viva de la vista (objeto de la fábrica que se mueve con su bucle por fotograma).
    pub object: bool,
    /// Fotograma actual (`+0x15`) e índice del conjunto de sprites de su clase.
    pub frame: u8,
    pub set_index: u8,
    /// Opacidad de los efectos vivos (`+0x30`, 0xFF opaco).
    pub alpha: u8,
}

impl Marker {
    /// El marcador se está apagando (la escena ya no lo apunta).
    pub fn fading_out(&self) -> bool {
        self.fade.is_some_and(|(_, b)| (b as i32) < 0)
    }
}

/// Datos de la ciudad con los que se abre la vista.
pub struct ViewMap<'a> {
    pub width: u32,
    pub height: u32,
    /// Celdas `(ficha, b2, b3)` de las capas 0 (suelo) y 1 (objetos), `índice = y * width + x`.
    pub layers: [&'a [(u16, u8, u8)]; 2],
    /// Clase de la ficha (`LetSheet::field_2d`, método virtual `+0x50`); las fichas sin clase cuentan 0.
    pub kinds: &'a BTreeMap<u16, u32>,
    /// Puntos de reunión `(x, y)` (lista 4 del `.spawn`).
    pub rally: &'a [(u32, u32)],
    /// Caminos de las claves 1, 2 y 3 del `.nodes` (píxeles del mundo), en `vista+0xC670 + 4 * clave`.
    pub nodes: [&'a [Vec<(i32, i32)>]; 3],
}

pub struct SiegeTownView {
    pub city: usize,
    bytes: Vec<u8>,
    /// Comienzo del montón propio (dirección).
    pub heap: u32,
    cursor: u32,
    sizes: BTreeMap<u32, u32>,
    free: Vec<(u32, u32)>,
    /// Escena viva (0x650 bytes) mientras la vista está abierta; sus punteros apuntan a esta vista.
    pub scene: Option<Vec<u8>>,
    /// Escena anterior a la apertura, para cerrar.
    pub(crate) closed: Option<Vec<u8>>,
    pub markers: BTreeMap<u32, Marker>,
    /// Veces que la apertura hizo huir a los transeúntes (`0x00598150`).
    pub walkers_fled: u32,
    /// Parte viva del registro de asedio de la ciudad ([`RUNTIME`]): figuras de la batalla entre fotogramas.
    pub runtime: Vec<u8>,
    /// Estado del `rand` de la vista (`0x00639C03`, LCG del CRT). Con la simulación, el flujo del CRT que comparte con
    /// el resto del juego (`investigation_rng`) entra y sale de aquí en cada ejecución.
    pub rand_state: u32,
    /// Viento de la escena del puerto (`0x00618D10`, `+0x7EEC/+0x7EF0/+0x7EF1`), que empieza reiniciado.
    pub harbor_wind: HarborWind,
    /// Orientación del mar de la ciudad (`AusrichtungDesMeeres`, `TownData+0x31`); sin ella el viento lo da el cliente.
    pub sea_direction: Option<u8>,
    /// Byte de salida del reencolado (`[esp+0x13]` de `0x00630DC0`, sin iniciar en el original).
    pub(crate) exit: u8,
    /// Fotogramas de la batalla ejecutados (`0x00631A50`).
    pub frames: u32,
    /// Memoria del puente reutilizada entre fotogramas.
    pub(crate) cache: Option<crate::siege_ops::FrameCache>,
    /// Tablas globales de las clases ([`GLOBALS`]) y tablas virtuales de sus objetos ([`entities::vtable_span`]).
    globals: Vec<u8>,
    vtables: Vec<u8>,
    /// Métodos virtuales sin portar que llamaron las figuras (devuelven 0).
    pub unported: std::collections::BTreeSet<u32>,
    /// Sonidos pedidos por las figuras (`0x00595B00`), sin reproducir.
    pub sounds: u32,
    client: Box<dyn SiegeViewClient>,
}

impl SiegeTownView {
    pub fn new(city: usize, map: &ViewMap, client: Box<dyn SiegeViewClient>) -> Result<Self, String> {
        let cells = map.width.checked_mul(map.height).ok_or("vista demasiado grande")?;
        if map.layers.iter().any(|l| l.len() != cells as usize) {
            return Err("capas de la vista con otro tamaño".into());
        }
        let ids = map.layers.iter().flat_map(|l| l.iter().map(|c| c.0)).max().unwrap_or(0) as u32 + 1;
        let layer0 = HEADER;
        let layer1 = layer0 + cells * 4;
        let table = layer1 + cells * 4;
        let objects = table + ids * 4;
        let vtable = objects + ids * 8;
        let rally = vtable + 0x60;
        let paths = rally + 8 * map.rally.len() as u32;
        let node_bytes: u32 = map.nodes.iter().map(|k| 8 * k.len() as u32 + k.iter().map(|p| 8 * p.len() as u32).sum::<u32>()).sum();
        // Capa de enlaces de entidades por casilla (`+0xC310`, vacía: las figuras no se apuntan en ella) y tabla de
        // entidades (`+0x3FC`) para la casilla libre (`0x00593580`).
        let links = (paths + node_bytes + 0x40 + 0xF) & !0xF;
        let entities = links + cells * 4;
        let heap = (entities + 0x40 + 0xFFF) & !0xFFF;
        // Campo `u16` por casilla, cola de nodos de 6 bytes que crece, y margen.
        // y los conjuntos de sprites de las clases de figuras con sus figuras ([`SPRITE_SPACE`]).
        let size = heap + 0x2000 + cells * 2 + cells * 6 * 4 + 0x10000 + SPRITE_SPACE;
        let mut v = SiegeTownView {
            city,
            bytes: vec![0; size as usize],
            heap: VIEW_BASE + heap,
            cursor: VIEW_BASE + heap + 0x1000,
            sizes: BTreeMap::new(),
            free: Vec::new(),
            scene: None,
            closed: None,
            markers: BTreeMap::new(),
            walkers_fled: 0,
            runtime: vec![0; RUNTIME.len()],
            rand_state: 1,
            harbor_wind: HarborWind::default(),
            sea_direction: None,
            exit: 0,
            frames: 0,
            cache: None,
            globals: vec![0; GLOBALS.len()],
            vtables: vec![0; entities::vtable_span().len()],
            unported: Default::default(),
            sounds: 0,
            client,
        };
        v.run_local(|m, _| entities::install_vtables(m))?;
        let put = |v: &mut Self, at: u32, x: u32| v.bytes[at as usize..at as usize + 4].copy_from_slice(&x.to_le_bytes());
        for (at, x) in [(0xC314, map.width), (0xC318, map.height), (0xC308, VIEW_BASE + layer0), (0xC30C, VIEW_BASE + layer1), (0x98, VIEW_BASE + table), (0xC698, map.rally.len() as u32), (0xC6AC, VIEW_BASE + rally), (0xC310, VIEW_BASE + links), (0x3FC, VIEW_BASE + entities), (0xC324, city as u32), (0xBF2C, city as u32)] {
            put(&mut v, at, x);
        }
        for (k, layer) in map.layers.iter().enumerate() {
            let base = if k == 0 { layer0 } else { layer1 };
            for (i, c) in layer.iter().enumerate() {
                let at = (base + 4 * i as u32) as usize;
                v.bytes[at..at + 2].copy_from_slice(&c.0.to_le_bytes());
                v.bytes[at + 2] = c.1;
                v.bytes[at + 3] = c.2;
            }
        }
        for id in 0..ids {
            put(&mut v, table + 4 * id, VIEW_BASE + objects + 8 * id);
            put(&mut v, objects + 8 * id, VIEW_BASE + vtable);
            put(&mut v, objects + 8 * id + 4, map.kinds.get(&(id as u16)).copied().unwrap_or(0));
        }
        for (i, &(x, y)) in map.rally.iter().enumerate() {
            put(&mut v, rally + 8 * i as u32, x);
            put(&mut v, rally + 8 * i as u32 + 4, y);
        }
        let mut at = paths;
        for (key, list) in map.nodes.iter().enumerate() {
            let table = at;
            put(&mut v, 0xC674 + 4 * key as u32, VIEW_BASE + table);
            at += 8 * list.len() as u32;
            for (k, path) in list.iter().enumerate() {
                put(&mut v, table + 8 * k as u32, path.len() as u32);
                put(&mut v, table + 8 * k as u32 + 4, VIEW_BASE + at);
                for &(x, y) in path {
                    put(&mut v, at, x as u32);
                    put(&mut v, at + 4, y as u32);
                    at += 8;
                }
            }
        }
        Ok(v)
    }

    /// Bytes ocupados del montón propio.
    pub fn heap_used(&self) -> u32 {
        self.cursor - self.heap
    }

    /// Lee bytes de la vista o de su montón (para pruebas y diagnóstico).
    pub fn read(&self, at: u32, len: usize) -> Option<&[u8]> {
        let o = at.checked_sub(VIEW_BASE)? as usize;
        self.bytes.get(o..o + len)
    }

    /// Campo de rutas de la escena abierta (`+0x59C`, `u16` por casilla), si está reservado.
    pub fn flow_field(&self) -> Option<Vec<u16>> {
        let scene = self.scene.as_ref()?;
        let p = u32::from_le_bytes(scene[0x59c..0x5a0].try_into().unwrap());
        if p == 0 {
            return None;
        }
        let w = u32::from_le_bytes(self.bytes[0xC314..0xC318].try_into().unwrap());
        let h = u32::from_le_bytes(self.bytes[0xC318..0xC31C].try_into().unwrap());
        let raw = self.read(p, (w * h * 2) as usize)?;
        Some(raw.chunks(2).map(|c| u16::from_le_bytes([c[0], c[1]])).collect())
    }

    /// Ejecuta `f` con la memoria compuesta (vista + `inner`) y el anfitrión de la vista.
    pub(crate) fn run<R>(&mut self, inner: &mut dyn Mem, f: impl FnOnce(&mut dyn Mem, &mut dyn SiegeViewHost) -> R) -> Result<R, String> {
        let SiegeTownView { city, bytes, cursor, sizes, free, markers, walkers_fled, rand_state, harbor_wind, sea_direction, client, globals, vtables, unported, sounds, .. } = self;
        let limit = VIEW_BASE + bytes.len() as u32;
        let mut mem = ViewMem { inner, bytes, globals, vtables };
        let mut host = Host { limit, cursor, sizes, free, markers, walkers_fled, rand_state, wind: (harbor_wind, *sea_direction, *city as u16), client: client.as_mut(), unported, sounds, error: None };
        let out = f(&mut mem, &mut host);
        match host.error {
            Some(e) => Err(e),
            None => Ok(out),
        }
    }

    /// Ejecuta `f` solo sobre la memoria propia de la vista (fuera de ella se lee 0 y no se escribe): carga de los
    /// conjuntos de sprites de las figuras con [`p3_core::economy::siege_figures::sets`].
    pub fn run_local<R>(&mut self, f: impl FnOnce(&mut dyn Mem, &mut dyn FigureHost) -> R) -> Result<R, String> {
        let mut none = NoMem;
        let SiegeTownView { city, bytes, cursor, sizes, free, markers, walkers_fled, rand_state, harbor_wind, sea_direction, client, globals, vtables, unported, sounds, .. } = self;
        let limit = VIEW_BASE + bytes.len() as u32;
        let mut mem = ViewMem { inner: &mut none, bytes, globals, vtables };
        let mut host = Host { limit, cursor, sizes, free, markers, walkers_fled, rand_state, wind: (harbor_wind, *sea_direction, *city as u16), client: client.as_mut(), unported, sounds, error: None };
        let out = f(&mut mem, &mut host);
        match host.error {
            Some(e) => Err(e),
            None => Ok(out),
        }
    }

    /// Ejecuta `f` con la memoria compuesta y el anfitrión de las figuras (bucle de la lista de entidades).
    pub(crate) fn run_figures<R>(&mut self, inner: &mut dyn Mem, f: impl FnOnce(&mut dyn Mem, &mut dyn FigureHost) -> R) -> Result<R, String> {
        let SiegeTownView { city, bytes, cursor, sizes, free, markers, walkers_fled, rand_state, harbor_wind, sea_direction, client, globals, vtables, unported, sounds, .. } = self;
        let limit = VIEW_BASE + bytes.len() as u32;
        let mut mem = ViewMem { inner, bytes, globals, vtables };
        let mut host = Host { limit, cursor, sizes, free, markers, walkers_fled, rand_state, wind: (harbor_wind, *sea_direction, *city as u16), client: client.as_mut(), unported, sounds, error: None };
        let out = f(&mut mem, &mut host);
        match host.error {
            Some(e) => Err(e),
            None => Ok(out),
        }
    }
}

/// Lectura de la memoria propia de la vista fuera de una orden (posición de las figuras vivas).
struct LocalMem<'a>(&'a [u8]);

impl Mem for LocalMem<'_> {
    fn r8(&mut self, a: u32) -> u8 {
        self.0.get(a.wrapping_sub(VIEW_BASE) as usize).copied().unwrap_or(0)
    }
    fn w8(&mut self, _: u32, _: u8) {}
}

/// Memoria vacía fuera de la vista (carga de conjuntos).
struct NoMem;

impl Mem for NoMem {
    fn r8(&mut self, _: u32) -> u8 {
        0
    }
    fn w8(&mut self, _: u32, _: u8) {}
}

/// Memoria de las órdenes con la vista: la región de la vista y el puntero global `[0x006E51AC]` salen de aquí; el
/// resto, de `inner`.
struct ViewMem<'a> {
    inner: &'a mut dyn Mem,
    bytes: &'a mut Vec<u8>,
    globals: &'a mut Vec<u8>,
    vtables: &'a mut Vec<u8>,
}

impl ViewMem<'_> {
    fn local(&self, a: u32) -> Option<usize> {
        let o = a.wrapping_sub(VIEW_BASE) as usize;
        (o < self.bytes.len()).then_some(o)
    }
    /// Byte de las tablas propias (globales de clase o tablas virtuales).
    fn table(&mut self, a: u32) -> Option<&mut u8> {
        if GLOBALS.contains(&a) {
            return self.globals.get_mut((a - GLOBALS.start) as usize);
        }
        let span = entities::vtable_span();
        if span.contains(&a) {
            return self.vtables.get_mut((a - span.start) as usize);
        }
        None
    }
}

impl Mem for ViewMem<'_> {
    fn r8(&mut self, a: u32) -> u8 {
        if let Some(o) = self.local(a) {
            return self.bytes[o];
        }
        if let Some(b) = self.table(a) {
            return *b;
        }
        let k = a.wrapping_sub(VIEW);
        if k < 4 {
            return VIEW_BASE.to_le_bytes()[k as usize];
        }
        self.inner.r8(a)
    }
    fn w8(&mut self, a: u32, v: u8) {
        if let Some(o) = self.local(a) {
            self.bytes[o] = v;
            return;
        }
        if let Some(b) = self.table(a) {
            *b = v;
            return;
        }
        assert!(a.wrapping_sub(VIEW) >= 4, "escritura en el puntero de la vista");
        self.inner.w8(a, v)
    }
    fn w8_seed(&mut self, a: u32) {
        if self.local(a).is_some() || self.table(a).is_some() {
            return;
        }
        self.inner.w8_seed(a)
    }
}

struct Host<'a> {
    limit: u32,
    cursor: &'a mut u32,
    sizes: &'a mut BTreeMap<u32, u32>,
    free: &'a mut Vec<(u32, u32)>,
    markers: &'a mut BTreeMap<u32, Marker>,
    walkers_fled: &'a mut u32,
    rand_state: &'a mut u32,
    /// Viento del puerto, orientación del mar y ciudad de la escena (`+0x5C`).
    wind: (&'a mut HarborWind, Option<u8>, u16),
    client: &'a mut dyn SiegeViewClient,
    unported: &'a mut std::collections::BTreeSet<u32>,
    sounds: &'a mut u32,
    error: Option<String>,
}

impl Host<'_> {
    fn take(&mut self, size: u32) -> u32 {
        let size = size.max(1);
        // Se reutiliza un bloque libre del mismo tamaño (el campo de rutas se reserva de nuevo en cada reinicio).
        if let Some(k) = self.free.iter().position(|&(_, s)| s == size) {
            let (p, _) = self.free.swap_remove(k);
            self.sizes.insert(p, size);
            return p;
        }
        let p = *self.cursor;
        let end = (p + size + 0xF) & !0xF;
        if end > self.limit {
            self.error.get_or_insert_with(|| "montón de la vista de asedio agotado".into());
            return 0;
        }
        *self.cursor = end;
        self.sizes.insert(p, size);
        p
    }
}

impl SiegeViewHost for Host<'_> {
    fn tile_kind(&mut self, m: &mut dyn Mem, object: u32) -> u32 {
        m.r32(object.wrapping_add(4))
    }
    fn step(&mut self, _: &mut dyn Mem, col: &mut u32, row: &mut u32, dir: u8) -> bool {
        self.client.step(col, row, dir)
    }
    fn pixel_to_cell(&mut self, _: &mut dyn Mem, x: i32, y: i32) -> (u32, u32) {
        self.client.pixel_to_cell(x, y)
    }
    fn cell_to_pixel(&mut self, _: &mut dyn Mem, col: u32, row: u32) -> (i32, i32) {
        self.client.cell_to_pixel(col, row)
    }
    fn alloc(&mut self, _: &mut dyn Mem, size: u32) -> u32 {
        self.take(size)
    }
    fn realloc(&mut self, m: &mut dyn Mem, block: u32, size: u32) -> u32 {
        let p = self.take(size);
        if block != 0 && p != 0 {
            let keep = self.sizes.get(&block).copied().unwrap_or(0).min(size);
            for i in 0..keep {
                let b = m.r8(block + i);
                m.w8(p + i, b);
            }
            if let Some(s) = self.sizes.remove(&block) {
                self.free.push((block, s));
            }
        }
        p
    }
    fn free(&mut self, _: &mut dyn Mem, block: u32) {
        if let Some(s) = self.sizes.remove(&block) {
            self.free.push((block, s));
        }
    }
    fn spawn_entity(&mut self, m: &mut dyn Mem, args: [u32; 7]) -> u32 {
        if args[6] == 0 && LIVE_CLASSES.contains(&args[2]) {
            // Figura viva: objeto de la fábrica en la lista de entidades de la vista (`0x00590610` completo).
            let entity = entities::spawn(m, self, args);
            if entity != 0 {
                let mut marker = Marker { entity, x: 0, y: 0, class: args[2], variant: args[3], fade: None, heading: 0, object: true, frame: 0, set_index: 0, alpha: 0xFF };
                refresh(m, &mut marker);
                self.markers.insert(entity, marker);
            }
            return entity;
        }
        let entity = self.take(ENTITY_SIZE);
        if entity == 0 {
            return 0;
        }
        for i in 0..ENTITY_SIZE {
            m.w8(entity + i, 0);
        }
        m.w32(entity + 0xC, args[0]);
        m.w32(entity + 0x10, args[1]);
        self.markers.insert(entity, Marker { entity, x: (args[0] as i32) >> 16, y: (args[1] as i32) >> 16, class: args[2], variant: args[3], fade: None, heading: args[4] as u8, object: false, frame: 0, set_index: 0, alpha: 0xFF });
        entity
    }
    fn rand(&mut self, _: &mut dyn Mem) -> u32 {
        *self.rand_state = self.rand_state.wrapping_mul(214_013).wrapping_add(2_531_011);
        (*self.rand_state >> 16) & 0x7FFF
    }
    fn wind(&mut self, m: &mut dyn Mem) -> (u32, u32) {
        let (state, sea, town) = (&mut self.wind.0, self.wind.1, self.wind.2);
        match sea {
            Some(sea) => {
                let (dir, force) = state.sample(m.r32(0x006D_E4B4), town, sea);
                (u32::from(dir), u32::from(force))
            }
            None => self.client.wind(),
        }
    }
    fn place_entity(&mut self, m: &mut dyn Mem, entity: u32, x: u32, y: u32, z: u32) {
        // Método `+0x48`: la entidad pasa al píxel `(x, y)` (en las tropas vivas es `0x00550370`, que no hace nada;
        // en los efectos, `0x0057A4A0`).
        if self.markers.get(&entity).is_some_and(|k| k.object) {
            vcall(m, self, entity, 0x48, &[x, y, z]);
            if let Some(k) = self.markers.get_mut(&entity) {
                refresh(m, k);
            }
            return;
        }
        m.w32(entity + 0xC, x << 16);
        m.w32(entity + 0x10, y << 16);
        if let Some(marker) = self.markers.get_mut(&entity) {
            marker.x = x as i32;
            marker.y = y as i32;
        }
    }
    fn make_walkers_flee(&mut self, _: &mut dyn Mem) {
        *self.walkers_fled += 1;
    }
    fn release_entity(&mut self, m: &mut dyn Mem, entity: u32) {
        // Destructor de la entidad (`0x00593D50`): las figuras del asedio salen de las listas de la escena o del
        // registro de la ciudad de la vista. Las figuras vivas antes salen de su casilla (`+0x44`) y de la lista de
        // entidades de la vista (hueco de su identificador `+0x16`).
        if let Some(marker) = self.markers.remove(&entity) {
            if marker.object {
                vcall(m, self, entity, 0x44, &[]);
                let list = m.r32(VIEW) + entities::LIST;
                let id = u32::from(m.r16(entity + 0x16));
                if id <= m.r32(list + 0xC) {
                    let arr = m.r32(list + 4);
                    if arr != 0 && m.r32(arr + 4 * id) == entity {
                        m.w32(arr + 4 * id, 0);
                    }
                }
            }
            siege_battle::remove_destroyed_entity(m, entity, marker.class as u8);
            self.free(m, entity);
        }
    }
    fn set_fade(&mut self, m: &mut dyn Mem, entity: u32, a: u32, b: u32) {
        if let Some(marker) = self.markers.get_mut(&entity) {
            marker.fade = Some((a, b));
            if marker.object && marker.class == 0x12 {
                // `0x0057AF40` sobre el efecto vivo: su paso por fotograma hace el fundido.
                p3_core::economy::siege_figures::effect::set_fade(m, entity, a, b);
            }
        }
    }
}

impl FigureHost for Host<'_> {
    fn play_sound(&mut self, _: &mut dyn Mem, _: i32, _: i32, _: u32) {
        *self.sounds += 1;
    }
    fn virt(&mut self, _: &mut dyn Mem, f: u32, _: u32, _: &[u32]) -> u32 {
        self.unported.insert(f);
        0
    }
}

/// Posición (`+0xE`, `+0x12`: parte entera de `+0xC/+0x10`), rumbo `+0x14`, fotograma `+0x15` e índice de conjunto
/// (`+0x90` en los luchadores, `+0x91` en los lanceros, `+0x36` en los proyectiles, `+0x2C` en los efectos, `+0x31` en la
/// máquina de asedio, `+0x35` en el ariete, `+0x88` en el resto) de una
/// figura viva.
fn refresh(m: &mut dyn Mem, k: &mut Marker) {
    let e = k.entity;
    k.x = i32::from(m.r16(e + 0xE) as i16);
    k.y = i32::from(m.r16(e + 0x12) as i16);
    k.heading = m.r8(e + 0x14);
    k.frame = m.r8(e + 0x15);
    k.set_index = m.r8(e + match k.class {
        8 => 0x90,
        0x20 => 0x91,
        0x17 => 0x36,
        0x12 => 0x2C,
        0x15 => 0x31,
        0x16 => 0x35,
        _ => 0x88,
    });
    if k.class == 0x12 {
        k.alpha = m.r8(e + 0x30);
    }
}

/// Apertura completa (`0x0062C730`) sobre la escena en `at`: la memoria `inner` trae ciudades, registros de
/// edificio, plano, reloj y registros de asedio (`[0x006DF270]`, con la parte viva de la vista restaurada).
pub(crate) fn open_scene(view: &mut SiegeTownView, inner: &mut dyn Mem, at: u32) -> Result<(), String> {
    let mut exit = view.exit;
    view.run(inner, |m, h| siege_battle::rebuild_view_state(m, h, at, &mut exit))?;
    view.exit = exit;
    Ok(())
}

/// Un fotograma de la batalla (`0x00631A50`).
pub(crate) fn frame_scene(view: &mut SiegeTownView, inner: &mut dyn Mem, at: u32) -> Result<(), String> {
    let mut exit = view.exit;
    view.run(inner, |m, h| siege_battle::update_town_view(m, h, at, &mut exit))?;
    view.exit = exit;
    // Bucle de la lista de entidades (`0x00568C80`, desde el fotograma de la vista): las figuras vivas se mueven,
    // disparan y luchan.
    view.run_figures(inner, |m, h| entities::update_all(m, h))?;
    let SiegeTownView { markers, .. } = view;
    let mut mem = LocalMem(&view.bytes);
    for k in markers.values_mut().filter(|k| k.object) {
        refresh(&mut mem, k);
    }
    view.frames += 1;
    Ok(())
}

/// Cierre (`0x0062F5A0` con destrucción): retira las figuras de la vista y libera campo y cola.
pub(crate) fn close_scene(view: &mut SiegeTownView, inner: &mut dyn Mem, at: u32) -> Result<(), String> {
    view.run(inner, |m, h| siege_battle::release_view(m, h, at, true))
}
