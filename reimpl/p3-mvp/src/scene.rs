//! Constructor de escena `0x589360` ejecutado sobre el contenedor de recursos real.
//! `ArchiveInitEnv` implementa los externos del constructor con datos del `.cpr`: las listas `iso/let/files.lst` e `iso/lcr/files.lst`,
//! la comprobación de nombres (`arc_CreateFile`) y la carga de cada `.let`/`.lcr`. Las ventanas auxiliares y el gestor de objetos del
//! original son identificadores opacos (sin dibujo); el orden por clave (`obj_key`) y los contadores `escena+0x104` no se modelan.
use p3_assets::cpr::{fold_name, Archive};
use p3_render::ambience::SceneMem;
use p3_render::scene_init::{init_scene, Globals, InitEnv};
use serde_json::{json, Value};

pub struct ArchiveInitEnv<'a> {
    archive: &'a Archive<'a>,
    which: u32,
    next: u32,
    next_idx: u32,
    pub listed_let: usize,
    pub listed_lcr: usize,
    pub loaded_let: Vec<Vec<u8>>,
    pub failed_let: Vec<Vec<u8>>,
    pub loaded_lcr: Vec<Vec<u8>>,
    pub failed_lcr: Vec<Vec<u8>>,
    pub rejected: Vec<Vec<u8>>,
    /// Clases (`.lcr` +4) sin cabecera conocida en `p3_assets::lcr::extra_header_words`, y cuántos ficheros tiene cada una.
    pub lcr_unknown_kinds: std::collections::BTreeMap<u32, usize>,
    pub lcr_parse_errors: usize,
    /// Conjuntos de sprites ya interpretados, por clase y en el orden de carga.
    pub sets: std::collections::BTreeMap<u32, Vec<p3_assets::lcr::Lcr>>,
    registered: Vec<u16>,
    windows: u32,
}

impl<'a> ArchiveInitEnv<'a> {
    pub fn new(archive: &'a Archive<'a>) -> Self {
        Self {
            archive,
            which: 0,
            next: 0x1000,
            next_idx: 0,
            listed_let: 0,
            listed_lcr: 0,
            loaded_let: vec![],
            failed_let: vec![],
            loaded_lcr: vec![],
            failed_lcr: vec![],
            rejected: vec![],
            lcr_unknown_kinds: Default::default(),
            lcr_parse_errors: 0,
            sets: Default::default(),
            registered: vec![],
            windows: 0,
        }
    }
    fn file(&self, path: &[u8]) -> Option<&'a [u8]> {
        let want = fold_name(path);
        self.archive
            .entries
            .iter()
            .find(|e| fold_name(&e.name) == want)
            .and_then(|e| self.archive.read(e))
    }
    fn prefix(&self) -> &'static [u8] {
        if self.which == 1 {
            b"iso/let/"
        } else {
            b"iso/lcr/"
        }
    }
}

impl InitEnv for ArchiveInitEnv<'_> {
    fn ini(&mut self) {}
    fn vcall8(&mut self) -> u8 {
        0
    }
    fn alloc(&mut self, _size: u32) -> u32 {
        self.next += 0x100;
        self.next
    }
    fn ctor_window(&mut self, _o: u32) {
        self.windows += 1;
    }
    fn reg_window(&mut self, _o: u32) {}
    fn v68(&mut self, _w: u32, _x: i32, _y: i32, _z: i32) {}
    fn setflag(&mut self, _w: u32, _v: u32) {}
    fn f4bbb00(&mut self, _a: u32, _b: u32, _c: u32) {}
    fn f4bb520(&mut self) -> u32 {
        0
    }
    fn map_file(&mut self, which: u32) -> Option<Vec<u8>> {
        self.which = which;
        let data = self.file(if which == 1 {
            b"iso/let/files.lst"
        } else {
            b"iso/lcr/files.lst"
        })?;
        let n = data.split(|&b| b == 0).filter(|s| !s.is_empty()).count();
        if which == 1 {
            self.listed_let = n;
        } else {
            self.listed_lcr = n;
        }
        Some(data.to_vec())
    }
    fn unmap(&mut self, _which: u32) {}
    fn match_name(&mut self, name: &[u8]) -> bool {
        let ok = self.file(&[self.prefix(), name].concat()).is_some();
        if !ok {
            self.rejected.push(name.to_vec());
        }
        ok
    }
    fn p551300(&mut self) -> u32 {
        self.next_idx += 1;
        self.next_idx - 1
    }
    fn p5512d0(&mut self) -> u32 {
        0
    }
    fn f54f7e0(&mut self, _a: u32, _b: u32, _c: u32) -> u32 {
        self.next += 0x100;
        self.next
    }
    fn f586d90(&mut self, _h: u32, idx: u32) {
        self.registered.push(idx as u16);
    }
    fn v34(&mut self, _idx: u32, name: &[u8]) -> bool {
        let ok = self
            .file(&[b"iso/let/".as_slice(), name].concat())
            .is_some_and(|d| p3_assets::let_sheet::LetSheet::parse(d).is_ok());
        if ok {
            self.loaded_let.push(name.to_vec());
        } else {
            self.failed_let.push(name.to_vec());
        }
        ok
    }
    fn f586e50(&mut self, _idx: u32, _flag: u32) {}
    fn build_iter(&mut self, which: u32) {
        self.which = which;
    }
    fn iter_open(&mut self) -> bool {
        false
    }
    fn iter_next(&mut self) -> bool {
        false
    }
    fn iter_get(&mut self) -> Vec<u8> {
        vec![]
    }
    fn iter_close(&mut self) {}
    fn f586e40(&mut self) {}
    fn list_get(&mut self, _n: usize) -> Vec<u16> {
        self.registered.clone()
    }
    fn list_set(&mut self, _list: &[u16]) {}
    fn obj_key(&mut self, _h: u16) -> u8 {
        0
    }
    fn p558d00(&mut self) -> u32 {
        0
    }
    fn make(&mut self, _ty: u32) -> u32 {
        self.next += 0x100;
        self.next
    }
    fn f568c40(&mut self, _o: u32) {}
    fn v10(&mut self, _o: u32, name: &[u8]) {
        let mut ok = false;
        if let Some(d) = self
            .file(&[b"iso/lcr/".as_slice(), name].concat())
            .filter(|d| d.len() >= 8)
        {
            let kind = u32::from_le_bytes([d[4], d[5], d[6], d[7]]);
            match p3_assets::lcr::extra_header_words(kind) {
                Some(w) => match p3_assets::lcr::Lcr::parse(d, w) {
                    Ok(l) => {
                        self.sets.entry(kind).or_default().push(l);
                        ok = true;
                    }
                    Err(_) => self.lcr_parse_errors += 1,
                },
                None => *self.lcr_unknown_kinds.entry(kind).or_default() += 1,
            }
        }
        if ok {
            self.loaded_lcr.push(name.to_vec());
        } else {
            self.failed_lcr.push(name.to_vec());
        }
    }
    fn set_ratio(&mut self, _r: f32) {}
    fn f59da40(&mut self, _a: u32, _b: u32) {}
    fn ctor_48(&mut self, _o: u32) {}
    fn ctor_0c(&mut self, _o: u32) {}
    fn v0c(&mut self, _o: u32, _a: u32, _b: u32, _c: u32, _d: u32) {}
    fn ctor_1c(&mut self, _o: u32) {}
}

/// Ejecuta el constructor de escena sobre el contenedor y resume qué recursos carga. `width`/`height` son las dimensiones de la ventana de juego.
pub fn run_scene_init(archive: &Archive<'_>, width: u32, height: u32) -> Value {
    let mut env = ArchiveInitEnv::new(archive);
    let mut mem = SceneMem::sized(0xD000);
    mem.w32(0x510, 1);
    let flag = init_scene(&mut mem, &Globals { width, height }, &mut env);
    let names = |v: &[Vec<u8>]| {
        v.iter()
            .take(8)
            .map(|n| String::from_utf8_lossy(n).to_string())
            .collect::<Vec<_>>()
    };
    json!({
        "flag": flag,
        "windows": env.windows,
        "let": {"listed": env.listed_let, "loaded": env.loaded_let.len(), "failed": env.failed_let.len(), "failed_sample": names(&env.failed_let)},
        "lcr": {"listed": env.listed_lcr, "loaded": env.loaded_lcr.len(), "failed": env.failed_lcr.len(), "failed_sample": names(&env.failed_lcr)},
        "rejected_names": env.rejected.len(),
        "lcr_unknown_kinds": env.lcr_unknown_kinds.iter().map(|(k, n)| (k.to_string(), *n)).collect::<std::collections::BTreeMap<_, _>>(),
        "lcr_parse_errors": env.lcr_parse_errors,
    })
}

/// Rumbo inicial de los objetos nuevos (`[0x67792B]`, dato del ejecutable inglés; lo comprueba `oracle/x86_factory.py`).
const DIR_BYTE: u8 = 0x60;

/// Precipitación de la escena: lluvia (tipo 33) y nieve (tipo 29) con los objetos, el registro de enlaces y los conjuntos de sprites reales del contenedor.
/// Ejecuta `0x58B090` (capacidad) y `0x58AD30` (reparto) del original (`oracle/x86_weather_world.py`). No avanza los objetos con el tiempo todavía
/// (`0x576300`/`0x575B00` necesitan los virtuales de la escena `+0x48`/`+0x70`, sin portar) y el bit de nieve del registro de la ciudad se pasa a mano.
pub struct Weather {
    /// Ventana de la escena `(ancho, alto, desplazamiento vertical)`: los tres virtuales `+0x44`, `+0x48`, `+0x70`.
    pub window: (u32, u32, u32),
    /// `escena+0x414` (movimiento por tick; como en la multitud, se supone 1).
    pub budget: u32,
    /// Objetos de precipitación que devolvieron 0 en el último avance (nieve fundida).
    pub retired: usize,
    pub mem: SceneMem,
    pub world: p3_render::world::World,
    /// Juegos de sprites de la clase 34 (señales de aviso, `warnzeichen.lcr`): el original usa el único que hay.
    pub crack_sets: Vec<p3_assets::lcr::Lcr>,
}

/// Objetos del mapa con su huella en casillas (`+0x2A`/`+0x2B` de la ficha `.let`, es decir `strips_a`/`strips_b`; virtuales `+0x48`/`+0x4C`).
/// El identificador es el índice más uno.
#[derive(Default)]
pub struct Footprints {
    pub cells: std::collections::HashMap<u32, u16>,
    pub sizes: Vec<(u8, u8)>,
}

impl Footprints {
    pub fn place(&mut self, cell_index: u32, width: u8, height: u8) {
        self.sizes.push((width, height));
        self.cells.insert(cell_index, self.sizes.len() as u16);
    }
}

impl p3_render::world::MapObjects for Footprints {
    fn cell(&mut self, idx: u32) -> u16 {
        self.cells.get(&idx).copied().unwrap_or(0)
    }
    fn width(&mut self, h: u16) -> i32 {
        h.checked_sub(1)
            .and_then(|i| self.sizes.get(usize::from(i)))
            .map_or(0, |s| i32::from(s.0))
    }
    fn height(&mut self, h: u16) -> i32 {
        h.checked_sub(1)
            .and_then(|i| self.sizes.get(usize::from(i)))
            .map_or(0, |s| i32::from(s.1))
    }
}

impl Weather {
    /// Ejecuta el constructor de escena sobre `archive` (para cargar los conjuntos) y prepara la escena de precipitación: ventana `width` x `height`, sin desplazamiento.
    pub fn new(archive: &Archive<'_>, width: u32, height: u32, seed: u32) -> Self {
        let mut env = ArchiveInitEnv::new(archive);
        let mut mem = SceneMem::sized(0xD000);
        mem.w32(0x510, 1);
        init_scene(&mut mem, &Globals { width, height }, &mut env);
        let mut crt = p3_core::rng::CrtRand::default();
        crt.srand(seed);
        let mut world = p3_render::world::World::new(DIR_BYTE, Box::new(move || crt.rand()));
        world.snow_sets = env.sets.remove(&29).unwrap_or_default();
        let crack_sets = env.sets.remove(&34).unwrap_or_default();
        world.crack_interval = crack_sets.first().map_or(0, |l| l.interval_ms);
        mem.w32(0x2C, width);
        mem.w32(0x30, height);
        Weather {
            window: (width, height, 0),
            budget: 1,
            retired: 0,
            mem,
            world,
            crack_sets,
        }
    }

    /// Avanza la precipitación y las grietas hasta `now_ms` con los pasos del original (`0x576300`, `0x575B00`, `0x57FEB0`, `0x57FFB0`, paridad x86 en `p3-render`).
    /// El valor de retorno de las funciones de repintado (`0x559370/0x559130/0x5593C0`) se supone 1 y `0x4B9650` no hace nada visible.
    pub fn advance(&mut self, now_ms: u32) {
        use p3_render::precip_step::{clip_window, crack_step, rain_step, snow_step, sweep_cracks, StepEnv};
        use p3_render::scene_math::Mem;
        const SC: u32 = 0x5000_0000;
        const OBJ: u32 = SC + 0x10000;
        const SETS: u32 = SC + 0x11000;
        struct Flat {
            a: Vec<u8>,
            g: std::collections::HashMap<u32, u8>,
        }
        impl Mem for Flat {
            fn r8(&mut self, x: u32) -> u8 {
                if (SC..SC + 0x20000).contains(&x) {
                    self.a[(x - SC) as usize]
                } else {
                    *self.g.get(&x).unwrap_or(&0)
                }
            }
            fn w8(&mut self, x: u32, v: u8) {
                if (SC..SC + 0x20000).contains(&x) {
                    self.a[(x - SC) as usize] = v;
                } else {
                    self.g.insert(x, v);
                }
            }
        }
        struct Env((u32, u32, u32));
        impl StepEnv for Env {
            fn win_w(&mut self) -> u32 {
                self.0 .0
            }
            fn win_h(&mut self) -> u32 {
                self.0 .1
            }
            fn win_y(&mut self) -> u32 {
                self.0 .2
            }
            fn repaint(&mut self, _: u32) -> u32 {
                1
            }
            fn mark(&mut self, _: [i32; 4]) {}
        }
        let mut f = Flat {
            a: vec![0; 0x20000],
            g: Default::default(),
        };
        f.w32(0x006E_51AC, SC);
        f.w32(0x006D_CCF8, now_ms);
        f.w32(SC + 0x414, self.budget);
        for off in [0x14u32, 0x18, 0x2C, 0x30, 0xC650, 0xC654] {
            let v = self.mem.r32(off as usize);
            f.w32(SC + off, v);
        }
        // Tablas de clase de la nieve (`0x6E2E30/34/38`).
        let n = self.world.snow_sets.len() as u32;
        f.w32(0x006E_2E30, n);
        f.w32(0x006E_2E34, SETS);
        f.w32(0x006E_2E38, SETS + 0x400);
        for (k, l) in self.world.snow_sets.iter().enumerate() {
            let k = k as u32;
            let set = SETS + 0x100 + 0x40 * k;
            f.w32(SETS + 4 * k, set);
            let cnt = SETS + 0x800 + 8 * k;
            f.w32(cnt, l.dirs.first().map_or(0, |d| d.len() as u32));
            f.w32(set + 0xC, l.field_10);
            f.w32(set + 0x10, l.interval_ms);
            f.w32(set + 0x20, cnt);
            f.w32(SETS + 0x400 + 8 * k, l.extra_header.get(1).copied().unwrap_or(0));
            f.w32(SETS + 0x400 + 8 * k + 4, l.extra_header.get(2).copied().unwrap_or(0));
        }
        let mut env = Env(self.window);
        let slots = self.mem.r32(0xC7CC) as usize;
        let mut retired = 0;
        for i in 0..slots {
            let h = self.mem.r32(0xC6F0 + 4 * i);
            let Some(o) = self.world.objects.get_mut(&h) else {
                continue;
            };
            let len = o.mem.0.len().min(0x40);
            for k in 0..len {
                f.a[(OBJ - SC) as usize + k] = o.mem.0[k];
            }
            let ret = match o.ty {
                29 => {
                    let rand = &mut self.world.rand;
                    snow_step(&mut f, OBJ, &mut env, &mut || rand() as u32)
                }
                33 => {
                    let mut e2 = Env(self.window);
                    rain_step(&mut f, OBJ, &mut env, &mut |m, r| clip_window(m, SC, r, &mut e2))
                }
                _ => continue,
            };
            if o.ty == 29 && ret == 0 {
                retired += 1;
            }
            for k in 0..len {
                o.mem.0[k] = f.a[(OBJ - SC) as usize + k];
            }
        }
        self.retired = retired;
        // Grietas.
        if let Some(set) = self.crack_sets.first() {
            f.w32(0x006E_2EE0, set.interval_ms);
            f.w32(0x006E_2EF0, SETS + 0x900);
            f.w32(SETS + 0x900, set.dirs.first().map_or(0, |d| d.len() as u32));
        }
        let handles = self.world.cracks.clone();
        let mut gone: Vec<u32> = vec![];
        let vec_at = SETS + 0xA00;
        f.w32(0x006E_2F14, vec_at);
        f.w32(0x006E_2F18, vec_at + 4 * handles.len() as u32);
        for (k, h) in handles.iter().enumerate() {
            f.w32(vec_at + 4 * k as u32, *h);
        }
        let objects = std::cell::RefCell::new(&mut self.world.objects);
        // Cada grieta se copia a OBJ, se avanza y se devuelve.
        sweep_cracks(
            &mut f,
            &mut |m, h| {
                let mut ob = objects.borrow_mut();
                let Some(o) = ob.get_mut(&h) else { return false };
                for k in 0..0x40 {
                    m.w8(OBJ + k, o.mem.0.get(k as usize).copied().unwrap_or(0));
                }
                let alive = crack_step(m, OBJ, &mut Env((0, 0, 0)));
                for k in 0..0x40usize {
                    if k < o.mem.0.len() {
                        o.mem.0[k] = m.r8(OBJ + k as u32);
                    }
                }
                alive
            },
            &mut |m, h| {
                gone.push(h);
                let (lo, hi) = (m.r32(0x006E_2F14), m.r32(0x006E_2F18));
                let mut items: Vec<u32> = (lo..hi).step_by(4).map(|a| m.r32(a)).collect();
                items.retain(|&x| x != h);
                for (k, v) in items.iter().enumerate() {
                    m.w32(lo + 4 * k as u32, *v);
                }
                m.w32(0x006E_2F18, lo + 4 * items.len() as u32);
            },
        );
        drop(objects);
        self.world.cracks.retain(|h| !gone.contains(h));
        for h in gone {
            self.world.objects.remove(&h);
        }
    }

    /// Proyección del `iso.ini` inglés (`TileSize = 34 21`, `DownVector = 17 11`) y ancho de la rejilla de objetos del mapa, para las grietas.
    pub fn set_projection(&mut self, columns: u32) {
        for (off, v) in [
            (0x510, 34u32),
            (0x514, 21),
            (0x4D0, 17),
            (0x4D4, 11),
            (0x14, 0),
            (0x18, 0),
            (0xC314, columns),
        ] {
            self.mem.w32(off, v);
        }
        self.world.projection = Some([34, 21, 17, 11]);
    }

    /// `0x59BC90(x, y, p)`: grieta (señal de aviso) en la casilla `(x, y)` con intensidad `p`; reloj de la escena en milisegundos.
    pub fn crack(&mut self, x: u32, y: u32, p: u32, clock_ms: u32, map: &mut dyn p3_render::world::MapObjects) {
        self.world.clock = clock_ms;
        let (m, world) = (&self.mem, &mut self.world);
        p3_render::crack::spawn_crack(m, x, y, p, &mut p3_render::world::CrackCtx { world, scene: m, map });
    }

    /// Dibuja la precipitación y las grietas sobre `canvas`. Nieve: fotograma del juego de la clase 29 (`obj+0x24`); lluvia: segmento de `0x5759B0`
    /// (gris `0x969696` con alfa `0xB4`, o `dy*40+0x9C` si `dy < 7`); grietas: fotograma del juego de la clase 34.
    pub fn draw(
        &self,
        scene: &mut p3_render::Scene,
        canvas: &mut [u8],
        width: usize,
        height: usize,
    ) -> (usize, usize, usize) {
        let (mut snow, mut rain, mut cracks) = (0, 0, 0);
        let n = self.mem.r32(0xC7CC) as usize;
        for i in 0..n {
            let Some(o) = self.world.objects.get(&self.mem.r32(0xC6F0 + 4 * i)) else {
                continue;
            };
            let (x, y) = (i32::from(o.mem.r16(0xE) as i16), i32::from(o.mem.r16(0x12) as i16));
            match o.ty {
                29 => {
                    if let Some(set) = self.world.snow_sets.get(usize::from(o.mem.r8(0x24))) {
                        snow += usize::from(scene.draw_lcr_frame(
                            canvas,
                            width,
                            height,
                            set,
                            o.mem.r8(0x14),
                            o.mem.r8(0x15),
                            x,
                            y,
                        ));
                    }
                }
                33 => {
                    let (dx, dy) = (i32::from(o.mem.r8(0x21) as i8), i32::from(o.mem.r8(0x23) as i8));
                    let alpha = if dy < 7 {
                        (dy.wrapping_mul(40).wrapping_add(0x9C)) as u8
                    } else {
                        0xB4
                    };
                    draw_line(
                        canvas,
                        width,
                        height,
                        (x, y),
                        (x + dx, y + dy),
                        [0x96, 0x96, 0x96],
                        alpha,
                    );
                    rain += 1;
                }
                _ => {}
            }
        }
        if let Some(set) = self.crack_sets.first() {
            for h in &self.world.cracks {
                let Some(o) = self.world.objects.get(h) else { continue };
                let (x, y) = (i32::from(o.mem.r16(0xE) as i16), i32::from(o.mem.r16(0x12) as i16));
                cracks +=
                    usize::from(scene.draw_lcr_frame(canvas, width, height, set, o.mem.r8(0x14), o.mem.r8(0x15), x, y));
            }
        }
        (snow, rain, cracks)
    }

    pub fn set_capacity(&mut self, count: i32, reset: bool, snow: bool) {
        p3_render::precip::set_capacity(&mut self.mem, count, reset, snow, &mut self.world);
    }

    /// `0x58AD30(rect, cuenta)`; `limit` es el número de ciudades (`[0x6DE4B0]`).
    pub fn precipitate(&mut self, rect: [i32; 4], count: i32, limit: u32, snow: bool) {
        p3_render::precip::fill_rect(&mut self.mem, rect, count, limit, snow, &mut self.world);
    }

    /// Objetos activos: tipo, posición en píxeles (parte alta de la coordenada 16.16), fotograma y enlace.
    pub fn snapshot(&self) -> Value {
        let n = self.mem.r32(0xC7CC) as usize;
        let objs: Vec<Value> = (0..n)
            .filter_map(|i| {
                let h = self.mem.r32(0xC6F0 + 4 * i);
                let o = self.world.objects.get(&h)?;
                Some(json!({"slot": i, "type": o.ty, "x": o.mem.r16(0xE) as i16, "y": o.mem.r16(0x12) as i16, "frame": o.mem.r8(0x15), "link": o.mem.r16(0x16)}))
            })
            .collect();
        json!({"capacity": n, "next_slot": self.mem.r32(0xC7BC), "high_water": self.mem.r32(0xC7C0), "per_object_budget": self.mem.r32(0xC7C4), "objects": objs,
               "registry": {"max": self.world.registry.max, "capacity": self.world.registry.table.len()}, "snow_sets": self.world.snow_sets.len()})
    }
}

/// Segmento con mezcla alfa (Bresenham) sobre un lienzo RGBA.
fn draw_line(canvas: &mut [u8], width: usize, height: usize, a: (i32, i32), b: (i32, i32), rgb: [u8; 3], alpha: u8) {
    let (mut x, mut y) = a;
    let (dx, dy) = ((b.0 - a.0).abs(), -(b.1 - a.1).abs());
    let (sx, sy) = (if a.0 < b.0 { 1 } else { -1 }, if a.1 < b.1 { 1 } else { -1 });
    let mut err = dx + dy;
    loop {
        if x >= 0 && y >= 0 && (x as usize) < width && (y as usize) < height {
            let i = 4 * (y as usize * width + x as usize);
            for k in 0..3 {
                canvas[i + k] = ((u32::from(canvas[i + k]) * (255 - u32::from(alpha))
                    + u32::from(rgb[k]) * u32::from(alpha))
                    / 255) as u8;
            }
        }
        if (x, y) == b {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
}
