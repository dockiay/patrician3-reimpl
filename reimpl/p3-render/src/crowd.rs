//! Multitud de figuras libres de la ciudad (J02): reparto `populate`, caminar libre (`walk::free_walk_step`) y fotograma visible.
//!
//! Aplica la lógica con paridad x86 (`0x55C820` y sus funciones) a una escena de datos propios. No está validado en el juego: el presupuesto de
//! movimiento por tick (`escena+0x414`), la cadencia de ticks y el número de enlace `+0x16` de cada figura son suposiciones, y la visibilidad es un gancho vacío
//! y las pisadas (`0x55D0B0`) se calculan con `footprints::emit` si hay `footprint_view`. Las rutas sobre `.nodes` (`add_on_route`, `set_nodes`) se activan con `walk::route_walk_step`/`route_assign` (paridad x86 por
//! separado) cuando la ciudad aporta caminos de las claves 1 y 2; sin ellos el gestor no activa los modos 1 y 2. Las demás familias de entidades no se modelan aquí.

use crate::anim::CrtRand;
use crate::geom::Geom;
use crate::mover::{MoverAnim, SetInfo};
use crate::walk::*;
use crate::walkers::Walker;
use p3_assets::lcr::Lcr;
use std::collections::HashMap;

/// Datos de la ciudad que lee `0x55D9D0` (copia de lo necesario de `Town` y del `Site`).
#[derive(Clone, Debug, Default)]
pub struct CrowdTown {
    pub flags: u32,
    pub ref_time: u32,
    pub first_building: u16,
    pub building_count: u16,
    /// Registros de 8 bytes de los edificios.
    pub records: Vec<u8>,
    pub points: [(u8, u8); 8],
    pub clock: u32,
}

impl TargetEnv for (&CrowdTown, &CrowdScene) {
    fn flags(&self) -> u32 {
        self.0.flags
    }
    fn ref_time(&self) -> u32 {
        self.0.ref_time
    }
    fn chain_start(&self) -> u16 {
        0xFFFF
    }
    fn first_building(&self) -> u16 {
        self.0.first_building
    }
    fn building_count(&self) -> u16 {
        self.0.building_count
    }
    fn building(&self, i: u16) -> Option<[u8; 8]> {
        let o = usize::from(i) * 8;
        self.0.records.get(o..o + 8).map(|r| r.try_into().unwrap())
    }
    fn points(&self) -> [(u8, u8); 8] {
        self.0.points
    }
    fn street(&self, x: u32, y: u32) -> bool {
        WalkScene::street(self.1, x, y)
    }
    fn event_time(&self) -> Option<u32> {
        None
    }
    fn clock(&self) -> u32 {
        self.0.clock
    }
    fn world_count(&self) -> u16 {
        0
    }
    fn world_town(&self, _i: u16) -> Option<(u16, u16)> {
        None
    }
}

/// Rejilla de la ciudad reducida a lo que consulta el caminar: clases de ficha por capa, plano y ocupación por casilla.
#[derive(Clone, Debug, Default)]
pub struct CrowdScene {
    pub cols: u32,
    pub rows: u32,
    pub kind0: Vec<u32>,
    pub kind1: Vec<Option<u32>>,
    pub plan: Vec<u8>,
    pub stride: u32,
    /// `(enlace, clase, rumbo, animación)` de las figuras de cada casilla.
    pub occupants: HashMap<(u32, u32), Vec<(u16, u32, u8, u8)>>,
}

impl WalkScene for CrowdScene {
    fn size(&self) -> (u32, u32) {
        (self.cols, self.rows)
    }
    fn kind0(&self, x: u32, y: u32) -> u32 {
        self.kind0.get((y * self.cols + x) as usize).copied().unwrap_or(0)
    }
    fn kind1(&self, x: u32, y: u32) -> Option<u32> {
        self.kind1.get((y * self.cols + x) as usize).copied().flatten()
    }
    fn street(&self, x: u32, y: u32) -> bool {
        self.plan.get((self.stride.wrapping_mul(x).wrapping_add(y)) as usize).is_some_and(|c| c & 0x80 != 0)
    }
    fn entities(&self, x: u32, y: u32) -> Vec<(u16, u32)> {
        self.occupants.get(&(x, y)).map(|v| v.iter().map(|o| (o.0, o.1)).collect()).unwrap_or_default()
    }
}

/// Una figura libre con su conjunto de sprites.
#[derive(Clone, Debug)]
pub struct Member {
    /// Identificador del gestor (`manager::Manager`); 0 si no lo gobierna.
    pub id: u32,
    pub w: FreeWalker,
    pub kind: u32,
    pub variant: usize,
    /// Estado de ruta sobre `.nodes` (`walk::route_walk_step`); `None` o `kind == 0` camina libre.
    pub route: Option<RouteWalker>,
}

struct World<'a> {
    scene: &'a CrowdScene,
    town: &'a CrowdTown,
    geom: &'a Geom,
    mask: &'a [u8],
    set: &'a SetInfo,
    speed: u32,
    set_kind: u8,
    class: u8,
    clock: u32,
    budget: u32,
    rng: &'a mut CrtRand,
    kind: u32,
    footprint_view: Option<crate::footprints::FootprintView>,
    footprints: &'a mut Vec<crate::footprints::Footprint>,
}

impl LoopWorld for World<'_> {
    fn scene(&self) -> &dyn WalkScene {
        self.scene
    }
    fn target_env(&self) -> &dyn TargetEnv {
        unreachable!("choose_target se resuelve en el mundo")
    }
    fn geom(&self) -> &Geom {
        self.geom
    }
    fn mask(&self) -> &[u8] {
        self.mask
    }
    fn set(&self) -> &SetInfo {
        self.set
    }
    fn speed(&self) -> u32 {
        self.speed
    }
    fn budget(&self) -> u32 {
        self.budget
    }
    fn clock(&self) -> u32 {
        self.clock
    }
    fn hanse(&self) -> bool {
        false
    }
    fn entity_class(&self) -> u8 {
        self.class
    }
    fn entity_fields(&self, cell: (u32, u32), idx: usize) -> (u8, u8) {
        self.scene.occupants.get(&cell).and_then(|v| v.get(idx)).map_or((0, 0), |o| (o.2, o.3))
    }
    fn rand(&mut self) -> u32 {
        self.rng.next()
    }
    fn choose_target(&mut self, st: &mut TargetState) {
        let env = (self.town, self.scene);
        let rng = &mut *self.rng;
        choose_target(&env, self.geom, st, &mut || rng.next());
    }
    fn set_kind(&self) -> u8 {
        self.set_kind
    }
    fn visible(&mut self) -> bool {
        true
    }
    fn invalidate(&mut self) {}
    fn register(&mut self) {}
    fn effects(&mut self, m: &FreeWalker) {
        if let Some(fv) = self.footprint_view {
            let f = crate::footprints::emit(m, self.kind, self.set, fv, self.scene, self.geom, self.mask);
            self.footprints.extend(f);
        }
    }
    fn anim_done(&mut self, a: &mut MoverAnim) {
        free_anim_done(a, &mut || self.rng.next());
    }
}

pub struct Crowd {
    pub members: Vec<Member>,
    pub scene: CrowdScene,
    pub town: CrowdTown,
    pub geom: Geom,
    pub mask: Vec<u8>,
    pub rng: CrtRand,
    /// Reloj de animación en milisegundos.
    pub clock: u32,
    pub budget: u32,
    sets: HashMap<(u32, usize), (SetInfo, u32, u8)>,
    /// Caminos de `.nodes` por clave (1 y 2); cada camino es una lista de puntos `(x, y)` en píxeles.
    pub nodes: [Vec<Vec<(i32, i32)>>; 3],
    /// Vista actual `(origen x, origen y, ancho, alto)` (`escena+0xC650/0xC654/0x2C/0x30`).
    pub view: (u32, u32, u32, u32),
    /// Carros del cochero (tipo 31) sobre las rutas `.nodes`; `None` mientras nadie los cree (`spawn_carts`).
    pub carts: Option<crate::cart_field::CartField>,
    /// Condiciones de las huellas (`0x55D0B0`); `None` no las calcula.
    pub footprint_view: Option<crate::footprints::FootprintView>,
    /// Huellas pedidas por las figuras desde la última vez que se vació la lista, en orden.
    pub footprints: Vec<crate::footprints::Footprint>,
}

/// Mundo de las figuras que siguen una ruta (`walk::route_walk_step`, `route_assign`).
struct RouteWorldCrowd<'a> {
    scene: &'a CrowdScene,
    geom: &'a Geom,
    mask: &'a [u8],
    set: &'a SetInfo,
    speed: u32,
    clock: u32,
    budget: u32,
    nodes: &'a [Vec<Vec<(i32, i32)>>; 3],
    rng: &'a mut CrtRand,
    carts: Option<&'a crate::cart_field::CartField>,
}

impl RouteWorld for RouteWorldCrowd<'_> {
    fn alive(&self) -> bool {
        true
    }
    fn tables(&self) -> bool {
        !self.nodes[1].is_empty() && !self.nodes[2].is_empty()
    }
    fn nodes(&self, kind: u8, route: u8) -> Vec<(i32, i32)> {
        self.nodes.get(usize::from(kind)).and_then(|t| t.get(usize::from(route))).cloned().unwrap_or_default()
    }
    fn scene(&self) -> &dyn WalkScene {
        self.scene
    }
    fn geom(&self) -> &Geom {
        self.geom
    }
    fn mask(&self) -> &[u8] {
        self.mask
    }
    fn set(&self) -> &SetInfo {
        self.set
    }
    fn speed(&self) -> u32 {
        self.speed
    }
    fn budget(&self) -> u32 {
        self.budget
    }
    fn clock(&self) -> u32 {
        self.clock
    }
    fn hanse(&self) -> bool {
        false
    }
    fn harbour_route(&self) -> u8 {
        0
    }
    fn shore(&self, x: i32, y: i32, route: u8) -> Option<u16> {
        self.carts.and_then(|c| c.shore(x, y, route))
    }
    fn invalidate(&mut self) {}
    fn register(&mut self) {}
    fn anim_done(&mut self, a: &mut MoverAnim) {
        free_anim_done(a, &mut || self.rng.next());
    }
}

impl Crowd {
    pub fn new(scene: CrowdScene, town: CrowdTown, geom: Geom, figures: &HashMap<u32, Vec<Lcr>>, seed: u32) -> Self {
        let mut sets = HashMap::new();
        for (&k, v) in figures {
            for (i, l) in v.iter().enumerate() {
                sets.insert((k, i), (SetInfo::from_lcr(l), l.field_c, l.field_8 as u8));
            }
        }
        Crowd { members: Vec::new(), scene, town, geom, mask: crate::geom::detect_mask(), rng: CrtRand(seed), clock: 0, budget: 1, sets, nodes: Default::default(), view: (0, 0, 1024, 768), carts: None, footprint_view: None, footprints: Vec::new() }
    }

    /// Añade una figura (`0x55C7A0` + primer destino `0x55D9D0` + animación de espera) en la casilla `cell` con `heading`.
    pub fn add(&mut self, kind: u32, variant: usize, cell: (i32, u32), heading: u8) {
        let Some((set, speed, set_kind)) = self.sets.get(&(kind, variant)).cloned() else { return };
        let (px, py) = self.geom.cell_to_pixel(cell.0, cell.1);
        let mut raw = [0u8; 0x70];
        raw[0x14] = heading;
        let mut w = FreeWalker::from_bytes(&raw);
        w.x = (px as u32) << 16;
        w.y = (py as u32) << 16;
        w.px = px as i16;
        w.py = py as i16;
        w.want = heading;
        w.aux = heading;
        w.target = (px as u16, py as u16);
        w.link = self.members.len() as u16 + 1;
        w.b7 = 0;
        w.order = Order { b4: 1, b5: 0, b6: 0, b8: 0 };
        let mut m = Member { id: 0, w, kind, variant, route: None };
        self.members.push(m.clone());
        self.rebuild_occupants();
        let i = self.members.len() - 1;
        let mut anim = MoverAnim::default();
        let f = anim.select(&set, 2, self.clock);
        m.w.anim = anim;
        m.w.frame = if f == 0xFF { 0 } else { f };
        self.members[i] = m;
        let _ = speed;
        let _ = set_kind;
        self.retarget(i);
    }

    /// Registra un conjunto de sprites ya resuelto (herramientas y pruebas; `new` lo hace desde los `.lcr`).
    pub fn insert_set(&mut self, kind: u32, variant: usize, set: SetInfo, speed: u32, set_kind: u8) {
        self.sets.insert((kind, variant), (set, speed, set_kind));
    }

    /// Carga los caminos `.nodes` de la ciudad (claves 1 y 2; las demás no las usan las figuras).
    pub fn set_nodes(&mut self, sections: &[p3_assets::town::NodeSection]) {
        self.nodes = Default::default();
        for s in sections {
            if (1..=2).contains(&s.key) {
                self.nodes[s.key as usize] = s.paths.iter().map(|p| p.iter().map(|&(x, y)| (x as i32, y as i32)).collect()).collect();
            }
        }
    }

    /// ¿Hay caminos de los dos tipos (`escena+0xC674` y `+0xC678`)?
    pub fn has_nodes(&self) -> bool {
        !self.nodes[1].is_empty() && !self.nodes[2].is_empty()
    }

    /// Añade una figura que sigue la ruta `route` (`0x55B1D0`): `start` la coloca en el primer nodo del camino del tipo 1; si no, nace fuera de pantalla (`x == 5000`)
    /// sobre el camino del tipo 2. Devuelve el índice de la figura.
    pub fn add_on_route(&mut self, kind: u32, variant: usize, route: u8, start: bool, heading: u8) -> Option<usize> {
        let (set, _, _) = self.sets.get(&(kind, variant)).cloned()?;
        let speed = self.sets[&(kind, variant)].1;
        let mut raw = [0u8; 0x70];
        raw[0x14] = heading;
        let mut w = FreeWalker::from_bytes(&raw);
        if !start {
            w.x = 5000;
            w.y = 5000;
        }
        w.want = heading;
        w.aux = heading;
        w.link = self.members.len() as u16 + 1;
        w.order = Order { b4: 1, b5: 0, b6: 0, b8: 0 };
        let mut rw = RouteWalker::new(w);
        let view = self.view;
        let ok = {
            let mut world = RouteWorldCrowd { scene: &self.scene, geom: &self.geom, mask: &self.mask, set: &set, speed, clock: self.clock, budget: self.budget, nodes: &self.nodes, rng: &mut self.rng, carts: self.carts.as_ref() };
            route_assign(&mut world, &mut rw, route, start, view)
        };
        if ok == 0 {
            return None;
        }
        rw.fw.px = (rw.fw.x >> 16) as i16;
        rw.fw.py = (rw.fw.y >> 16) as i16;
        let mut anim = MoverAnim::default();
        let f = anim.select(&set, 2, self.clock);
        rw.fw.anim = anim;
        rw.fw.frame = if f == 0xFF { 0 } else { f };
        self.members.push(Member { id: 0, w: rw.fw.clone(), kind, variant, route: Some(rw) });
        Some(self.members.len() - 1)
    }

    fn retarget(&mut self, i: usize) {
        let (set, speed, set_kind) = self.sets[&(self.members[i].kind, self.members[i].variant)].clone();
        let mut w = World {
            scene: &self.scene,
            town: &self.town,
            geom: &self.geom,
            mask: &self.mask,
            set: &set,
            speed,
            set_kind,
            class: self.members[i].kind as u8,
            clock: self.clock,
            budget: self.budget,
            rng: &mut self.rng,
            kind: self.members[i].kind,
            footprint_view: None,
            footprints: &mut self.footprints,
        };
        let m = &mut self.members[i].w;
        let mut st = TargetState { pos: (m.px, m.py), target: m.target, timer: m.timer, b55: m.f55, b56: m.f56, b57: m.f57, beggar: w.class == 4 };
        w.choose_target(&mut st);
        m.target = st.target;
        m.timer = st.timer;
        m.f55 = st.b55;
        m.f56 = st.b56;
        m.f57 = st.b57;
    }

    fn rebuild_occupants(&mut self) {
        self.scene.occupants.clear();
        for m in &self.members {
            let ((cx, cy), _) = self.geom.pixel_to_cell(i32::from(m.w.px), i32::from(m.w.py), &self.mask);
            self.scene.occupants.entry((cx, cy)).or_default().push((m.w.link, m.kind, m.w.heading, m.w.anim.state));
        }
    }

    /// Crea los carros del cochero con las `entries` dadas (índice de entrada = posición). Devuelve cuántos aparecieron; 0 si faltan rutas (al menos tres
    /// de las claves 1 y 2 de `.nodes`) o juegos de sprites del tipo 31. A partir de entonces las figuras por ruta esquivan los carros (`0x570BA0`).
    pub fn spawn_carts(&mut self, figures: &HashMap<u32, Vec<Lcr>>, entries: Vec<crate::cart_field::CartEntry>) -> usize {
        self.spawn_carts_in(figures, entries, None)
    }

    /// Como [`Crowd::spawn_carts`], con la ciudad de la vista y los datos de partida que lee `spawn_t31` ([`crate::cart_field::CartTown`]).
    pub fn spawn_carts_in(&mut self, figures: &HashMap<u32, Vec<Lcr>>, entries: Vec<crate::cart_field::CartEntry>, town: Option<crate::cart_field::CartTown>) -> usize {
        let sets: Vec<SetInfo> = figures.get(&31).map(|v| v.iter().map(SetInfo::from_lcr).collect()).unwrap_or_default();
        let Some(mut f) = crate::cart_field::CartField::new(&self.nodes, &sets) else { return 0 };
        f.entries = entries;
        f.town = town;
        let mut n = 0;
        for e in 0..f.entries.len() {
            n += usize::from(f.spawn(e, &mut self.rng, self.clock).is_some());
        }
        self.carts = Some(f);
        n
    }

    /// Un tick de `ms` milisegundos para todas las figuras.
    pub fn tick(&mut self, ms: u32) {
        self.clock = self.clock.wrapping_add(ms);
        if let Some(c) = self.carts.as_mut() {
            c.step(self.clock, self.view);
        }
        self.town.clock = self.town.clock.wrapping_add(ms);
        for i in 0..self.members.len() {
            self.rebuild_occupants();
            let (set, speed, set_kind) = self.sets[&(self.members[i].kind, self.members[i].variant)].clone();
            if self.members[i].route.as_ref().is_some_and(|r| r.kind != 0) {
                let mut rw = self.members[i].route.take().unwrap();
                rw.fw = self.members[i].w.clone();
                let mut world = RouteWorldCrowd {
                    scene: &self.scene,
                    geom: &self.geom,
                    mask: &self.mask,
                    set: &set,
                    speed,
                    clock: self.clock,
                    budget: self.budget,
                    nodes: &self.nodes,
                    rng: &mut self.rng,
                    carts: self.carts.as_ref(),
                };
                route_walk_step(&mut world, &mut rw);
                rw.fw.px = (rw.fw.x >> 16) as i16;
                rw.fw.py = (rw.fw.y >> 16) as i16;
                self.members[i].w = rw.fw.clone();
                self.members[i].route = Some(rw);
                continue;
            }
            let mut w = World {
                scene: &self.scene,
                town: &self.town,
                geom: &self.geom,
                mask: &self.mask,
                set: &set,
                speed,
                set_kind,
                class: self.members[i].kind as u8,
                clock: self.clock,
                budget: self.budget,
                rng: &mut self.rng,
                kind: self.members[i].kind,
                footprint_view: self.footprint_view,
                footprints: &mut self.footprints,
            };
            let mut m = self.members[i].w.clone();
            let keep = free_walk_step(&mut w, &mut m);
            m.px = (m.x >> 16) as i16;
            m.py = (m.y >> 16) as i16;
            self.members[i].w = m;
            let _ = keep;
        }
    }

    /// Las figuras como `Walker` para dibujarlas (`Scene::draw_walkers` toma `frame` y el rumbo).
    pub fn walkers(&self) -> Vec<Walker> {
        self.members
            .iter()
            .map(|m| Walker { kind: m.kind, x: m.w.x as i32, y: m.w.y as i32, heading: m.w.heading, frame: m.w.frame, variant: m.variant as u32, anim: m.w.anim })
            .collect()
    }
}

/// Enlace entre el gestor `manager::Manager` y la multitud: implementa [`crate::manager::ManagerEnv`] para las poblaciones libres (modo 0).
/// Los modos 1 y 2 (rutas `.nodes`) y los tipos sin figura propia (niños, puerto, milicia) conservan solo su identificador: la figura no se dibuja.
pub struct CrowdLink<'a> {
    pub crowd: &'a mut Crowd,
    /// Tipo de entidad de cada objeto creado por la fábrica.
    pub kinds: HashMap<u32, u8>,
    pub next_object: u32,
    pub next_link: u32,
    /// Casilla libre `(x, bandera)` (`0x59A4F0`) a partir de `(a, b, c)` y el azar de la multitud.
    pub pick: &'a mut dyn FnMut(&mut CrtRand, u32, u32, u32) -> Option<(i32, u32)>,
    pub removed: Vec<u32>,
}

impl crate::manager::ManagerEnv for CrowdLink<'_> {
    fn hanse(&mut self) -> bool {
        false
    }
    fn rand(&mut self) -> u32 {
        self.crowd.rng.next()
    }
    fn new_object(&mut self, kind: u8) -> u32 {
        self.next_object += 1;
        self.kinds.insert(self.next_object, kind);
        self.next_object
    }
    fn next_link(&mut self) -> u32 {
        self.next_link += 1;
        self.next_link
    }
    fn register(&mut self, _obj: u32, _link: u32) {}
    fn spawn(&mut self, obj: u32, x: u32, y: u32, heading: u8, _link: u32, mode_route: u32) {
        let Some(&ty) = self.kinds.get(&obj) else { return };
        let Some(class) = crate::manager::CLASS_TYPES.iter().position(|&t| t == ty) else { return };
        let kind = crate::walkers::POPULATE_KINDS[class];
        if mode_route >> 16 != 0 {
            // Modos 1 y 2: figura sobre una ruta de `.nodes` (`mode_route >> 16`: 1 = inicio del camino, 2 = fuera de pantalla).
            if self.crowd.has_nodes() {
                let route = (mode_route & 0xFFFF) as u8;
                let count = (0..).take_while(|&i| self.crowd.sets.contains_key(&(kind, i))).count() as u32;
                let table = crate::spawn::SetTable { count, sets: (0..count).collect() };
                let skind = if ty == 4 { crate::spawn::SpawnKind::Always } else { crate::spawn::SpawnKind::Flagged };
                let rng = &mut self.crowd.rng;
                let (variant, _) = crate::spawn::choose_variant(skind, true, &table, 0, &mut || rng.next());
                if let Some(i) = self.crowd.add_on_route(kind, usize::from(variant), route, mode_route >> 16 == 1, heading) {
                    self.crowd.members[i].id = obj;
                }
            }
            return;
        }
        let count = (0..).take_while(|&i| self.crowd.sets.contains_key(&(kind, i))).count() as u32;
        let table = crate::spawn::SetTable { count, sets: (0..count).collect() };
        let skind = if ty == 4 { crate::spawn::SpawnKind::Always } else { crate::spawn::SpawnKind::Flagged };
        let rng = &mut self.crowd.rng;
        let (variant, _) = crate::spawn::choose_variant(skind, true, &table, 0, &mut || rng.next());
        let ((cx, cy), _) = self.crowd.geom.pixel_to_cell((x >> 16) as i32, (y >> 16) as i32, &self.crowd.mask);
        self.crowd.add(kind, usize::from(variant), (cx as i32, cy), heading);
        if let Some(m) = self.crowd.members.last_mut() {
            m.id = obj;
        }
    }
    fn position(&mut self, obj: u32) -> (u32, u32) {
        self.crowd.members.iter().find(|m| m.id == obj).map_or((0xFFFF, 0xFFFF), |m| (u32::from(m.w.px as u16), u32::from(m.w.py as u16)))
    }
    fn age(&mut self, _obj: u32) -> u32 {
        0
    }
    fn type_index(&mut self, obj: u32) -> u8 {
        self.crowd.members.iter().find(|m| m.id == obj).map_or(0, |m| m.variant as u8)
    }
    fn assign_route(&mut self, _table: u32, obj: u32, route: u32) -> bool {
        // `0x55B1D0(tabla, objeto, ruta, 0)` sobre una figura libre con presencia en pantalla.
        if !self.crowd.has_nodes() {
            return false;
        }
        let Some(i) = self.crowd.members.iter().position(|m| m.id == obj) else { return false };
        if self.crowd.members[i].route.as_ref().is_some_and(|r| r.kind != 0) {
            return false;
        }
        let (kind, variant) = (self.crowd.members[i].kind, self.crowd.members[i].variant);
        let Some((set, speed, _)) = self.crowd.sets.get(&(kind, variant)).cloned() else { return false };
        let mut rw = RouteWalker::new(self.crowd.members[i].w.clone());
        let view = self.crowd.view;
        let ok = {
            let c = &mut *self.crowd;
            let mut world = RouteWorldCrowd { scene: &c.scene, geom: &c.geom, mask: &c.mask, set: &set, speed, clock: c.clock, budget: c.budget, nodes: &c.nodes, rng: &mut c.rng, carts: c.carts.as_ref() };
            route_assign(&mut world, &mut rw, route as u8, false, view)
        };
        if ok == 0 {
            return false;
        }
        self.crowd.members[i].w = rw.fw.clone();
        self.crowd.members[i].route = Some(rw);
        true
    }
    fn remove(&mut self, obj: u32) {
        self.crowd.members.retain(|m| m.id != obj);
        self.removed.push(obj);
    }
    fn free_cell(&mut self, a: u32, b: u32, c: u32) -> Option<(i32, u32)> {
        (self.pick)(&mut self.crowd.rng, a, b, c)
    }
    fn realloc(&mut self, _bytes: u32) {}
    fn after(&mut self, _arg: u32) {}
}

impl Crowd {
    /// Un paso del gestor `0x596EE0` sobre esta multitud (solo poblaciones libres: temporizadores de ruta desactivados).
    pub fn run_manager(&mut self, mgr: &mut crate::manager::Manager, next_object: &mut u32, pick: &mut dyn FnMut(&mut CrtRand, u32, u32, u32) -> Option<(i32, u32)>) {
        if !self.has_nodes() {
            for p in &mut mgr.pops {
                p.t1 = u32::MAX;
                p.t2 = u32::MAX / 2;
            }
        }
        let mut link = CrowdLink { crowd: self, kinds: HashMap::new(), next_object: *next_object, next_link: 0, pick, removed: vec![] };
        mgr.update(&mut link);
        *next_object = link.next_object;
        for o in std::mem::take(&mut link.removed) {
            mgr.forget(o);
        }
    }
}
