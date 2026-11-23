//! Carros del cochero (tipo 31) sobre las rutas `.nodes` de la ciudad: aparición con `spawn_types::spawn_t31` (paridad x86), avance con `carts::step`
//! (paridad x86) y la lista global `[0x6E6280]` que lee la esquiva `walk::shore_value` (`0x570BA0`, paridad x86).
//!
//! Qué lee el original y no tenemos: el carro se crea con un índice de entrada de la tabla `0x6DD73C` (entradas de 24 bytes; `+9` = lado/ciudad de la
//! entrada, `+0x16` = fracción de progreso 0..255, que `0x5708A0`-`0x570920` copia a `+0x4C` y con la hora a `+0x44`). La entrada la aporta quien
//! llama ([`CartEntry`]); el modelo no la deduce de ningún guardado. Suposiciones no validadas en juego: `DAT_006DFC1C = 0` y `DAT_006DFBC4 = BC4`
//! (campos de `CartField`), lado actual (`escena+0xC324`) igual al de la entrada, ancla de dibujo.
use crate::ambience::SceneMem;
use crate::anim::CrtRand;
use crate::carts::{self, Cart, CartCtx, CartEnv, CartLink, Route};
use crate::mover::SetInfo;
use crate::scene_math::Mem;
use crate::spawn_types::{spawn_t31, SpawnArgs, Table, TypeEnv};
use std::collections::HashMap;

const SCENE: u32 = 0x5005_0000;
const TAB1: u32 = 0x5100_0000;
const TAB2: u32 = 0x5101_0000;
const PTS1: u32 = 0x5110_0000;
const PTS2: u32 = 0x5120_0000;
const ENTRIES: u32 = 0x5130_0000;
const SETS: u32 = 0x5200_0000;
const SIDE_ENTRY: u32 = 0x5140_0000;
pub const OBJ0: u32 = 0x5000_1000;
const HEAD: u32 = 0x006E_6280;

/// Ciudad de la vista y datos de partida que lee `spawn_t31` cuando el carro sale de ella (`+9` de la entrada distinto de `escena+0xC324`):
/// identificador de cada ciudad (`0x006DE4B8`), símbolo del mapa por identificador (`0x006DDB90 + 0x34 * id`, `StadtSymbolXPos/YPos`) y
/// tamaño en píxeles de la vista (`escena+0xC31C`, `+0xC320`).
#[derive(Clone, Debug, Default)]
pub struct CartTown {
    pub town: u8,
    pub order: Vec<u8>,
    pub symbols: Vec<(i32, i32)>,
    pub size: (u32, u32),
}

/// Entrada de la tabla de 24 bytes `0x6DD73C` que gobierna un carro: bytes `+9` y `+0x16`.
#[derive(Clone, Copy, Debug)]
pub struct CartEntry {
    pub side: u8,
    pub frac: u8,
}

struct Flat(HashMap<u32, u8>);
impl Mem for Flat {
    fn r8(&mut self, a: u32) -> u8 {
        *self.0.get(&a).unwrap_or(&0)
    }
    fn w8(&mut self, a: u32, v: u8) {
        self.0.insert(a, v);
    }
}

pub struct ActiveCart {
    pub addr: u32,
    pub cart: Cart,
    pub link: CartLink,
    pub variant: usize,
    pub route: u8,
}

pub struct CartField {
    mem: Flat,
    pub carts: Vec<ActiveCart>,
    /// Rutas `(orilla a, orilla b)` por índice 0..3 (claves `.nodes` 1 y 2).
    routes: Vec<(Vec<(i32, i32)>, Vec<(i32, i32)>)>,
    sets: Vec<u32>,
    recs: Vec<Vec<(u8, u8)>>,
    pub entries: Vec<CartEntry>,
    /// `DAT_006DFBC4` y `DAT_006DFC1C` (suposición).
    pub bc4: u32,
    pub fc1c: u32,
    /// Ciudad de la vista; sin ella se supone que es la de destino de cada entrada (`+9`).
    pub town: Option<CartTown>,
    next: u32,
}

fn fill_set(mem: &mut Flat, base: u32, s: &SetInfo) -> Vec<(u8, u8)> {
    let dirs = s.directions.max(1);
    mem.w32(base + 0x14, dirs);
    let (a18, a1c, ents) = (base + 0x100, base + 0x200, base + 0x300);
    mem.w32(base + 0x18, a18);
    mem.w32(base + 0x1C, a1c);
    mem.w32(base + 0x2C, ents);
    let frames = 0x100u32;
    for d in 0..dirs {
        mem.w32(a18 + 4 * d, base + 0x1000 + d * 0x1400);
        mem.w32(a1c + 4 * d, base + 0x1000 + d * 0x1400 + 0xA00);
    }
    let _ = frames;
    let mut recs = vec![];
    for (k, st) in s.states.iter().enumerate() {
        mem.w32(ents + 12 * k as u32 + 8, st.id);
        recs.push((st.first as u8, st.last as u8));
    }
    recs
}

impl CartField {
    /// `nodes`: claves 1 y 2 de `.nodes`; hacen falta al menos tres rutas de cada una.
    pub fn new(nodes: &[Vec<Vec<(i32, i32)>>; 3], sets: &[SetInfo]) -> Option<Self> {
        let n = nodes[1].len().min(nodes[2].len());
        if n < 3 || sets.is_empty() {
            return None;
        }
        let mut f = CartField { mem: Flat(HashMap::new()), carts: vec![], routes: vec![], sets: vec![], recs: vec![], entries: vec![], bc4: 1000, fc1c: 0, town: None, next: 0 };
        let m = &mut f.mem;
        m.w32(SCENE + 0xC674, TAB1);
        m.w32(SCENE + 0xC678, TAB2);
        for k in 0..3usize {
            let (a, b) = (&nodes[1][k], &nodes[2][k]);
            if a.is_empty() || a.len() != b.len() || a.len() > 255 {
                return None;
            }
            m.w32(TAB1 + 8 * k as u32, a.len() as u32);
            m.w32(TAB1 + 8 * k as u32 + 4, PTS1 + 0x10000 * k as u32);
            m.w32(TAB2 + 8 * k as u32, b.len() as u32);
            m.w32(TAB2 + 8 * k as u32 + 4, PTS2 + 0x10000 * k as u32);
            for (i, p) in a.iter().enumerate() {
                m.w32(PTS1 + 0x10000 * k as u32 + 8 * i as u32, p.0 as u32);
                m.w32(PTS1 + 0x10000 * k as u32 + 8 * i as u32 + 4, p.1 as u32);
            }
            for (i, p) in b.iter().enumerate() {
                m.w32(PTS2 + 0x10000 * k as u32 + 8 * i as u32, p.0 as u32);
                m.w32(PTS2 + 0x10000 * k as u32 + 8 * i as u32 + 4, p.1 as u32);
            }
            f.routes.push((a.clone(), b.clone()));
        }
        for (i, s) in sets.iter().enumerate() {
            let base = SETS + 0x10000 * i as u32;
            let recs = fill_set(&mut f.mem, base, s);
            f.sets.push(base);
            f.recs.push(recs);
        }
        Some(f)
    }

    fn sync_list(&mut self) {
        let mut next = 0;
        for c in self.carts.iter().rev() {
            self.mem.w32(c.addr + 0x60, next);
            next = c.addr;
        }
        self.mem.w32(HEAD, next);
    }

    /// Crea un carro con la entrada `entry` (índice en `entries`), con `spawn_t31`. Devuelve su posición en `carts`.
    pub fn spawn(&mut self, entry: usize, rng: &mut CrtRand, clock: u32) -> Option<usize> {
        if entry >= self.entries.len() {
            return None;
        }
        for (i, e) in self.entries.iter().enumerate() {
            let a = ENTRIES + 24 * i as u32;
            self.mem.w8(a + 9, e.side);
            self.mem.w8(a + 0x16, e.frac);
        }
        match self.town.clone() {
            Some(t) => {
                self.mem.w32(SCENE + 0xC324, u32::from(t.town));
                self.mem.w32(SCENE + 0xC31C, t.size.0);
                self.mem.w32(SCENE + 0xC320, t.size.1);
                for (i, id) in t.order.iter().enumerate() {
                    self.mem.w8(0x006D_E4B8 + i as u32, *id);
                }
                for (id, (x, y)) in t.symbols.iter().enumerate() {
                    self.mem.w32(0x006D_DB90 + 0x34 * id as u32, *x as u32);
                    self.mem.w32(0x006D_DB94 + 0x34 * id as u32, *y as u32);
                }
            }
            None => self.mem.w32(SCENE + 0xC324, u32::from(self.entries[entry].side)),
        }
        self.mem.w8(SIDE_ENTRY, 0);
        let addr = OBJ0 + 0x100 * self.next;
        let mut o = SceneMem::sized(0x100);
        let sets = self.sets.clone();
        let table = Table { count: sets.len() as u32, sets: &sets, kinds: &[] };
        let a = SpawnArgs { x: 0, y: 0, heading: 0, link: 0, variant: entry as u32 };
        self.sync_list();
        let routes = self.routes.clone();
        let mut env = SpawnEnv { mem: &mut self.mem, rng, clock, routes: &routes, ok: true };
        spawn_t31(&mut o, &a, &table, addr, &mut env);
        // `spawn_t31` termina sin escribir la cabeza de la lista cuando una división entre cero detendría al original.
        if self.mem.r32(HEAD) != addr {
            return None;
        }
        let route = o.r8(0x3C);
        let cart = read_cart(&o);
        for i in 0..0x80usize {
            self.mem.w8(addr + i as u32, o.r8(i));
        }
        self.next += 1;
        let variant = o.r8(0x2C) as usize;
        self.carts.insert(0, ActiveCart { addr, cart, link: CartLink { dir: o.r8(0x3A), tag: entry as i32, done: false }, variant, route });
        self.sync_list();
        self.refresh(clock);
        Some(0)
    }

    /// `0x5708A0`-`0x570920`: para cada carro copia la fracción de su entrada (`255 - frac` si el sentido no es 0) a `+0x4C` y la hora a `+0x44`.
    pub fn refresh(&mut self, clock: u32) {
        for c in &mut self.carts {
            let Some(e) = self.entries.get(c.link.tag as usize) else { continue };
            c.cart.frac = if c.cart.dir != 0 { 0xFFu8.wrapping_sub(e.frac) } else { e.frac };
            c.cart.start = clock;
        }
    }

    /// `0x570120` para cada carro con la hora `clock`; los que terminan (o salen de la vista) se retiran.
    pub fn step(&mut self, clock: u32, view: (u32, u32, u32, u32)) {
        let mut keep = vec![];
        let mut carts = std::mem::take(&mut self.carts);
        for mut c in carts.drain(..) {
            let (a, b) = &self.routes[usize::from(c.route)];
            let ctx = CartCtx { routes_present: true, route: Route { a, b }, time: clock, fc1c: self.fc1c, bc4: self.bc4, view, recs: &self.recs[c.variant] };
            let mut links = [c.link.clone()];
            let mut env = NoEnv;
            let alive = carts::step(&mut c.cart, &mut links, &ctx, &mut env);
            c.link = links[0].clone();
            self.mem.w16(c.addr + 0xE, c.cart.px as u16);
            self.mem.w16(c.addr + 0x12, c.cart.py as u16);
            if alive && !c.cart.done {
                keep.push(c);
            }
        }
        self.carts = keep;
        self.sync_list();
    }

    /// `0x570BA0(x, y, ruta)`: distancia mínima a los carros de esa ruta (16 bits bajos); `None` si no hay carros.
    pub fn shore(&self, x: i32, y: i32, route: u8) -> Option<u16> {
        struct Ro<'a>(&'a HashMap<u32, u8>);
        impl Mem for Ro<'_> {
            fn r8(&mut self, a: u32) -> u8 {
                *self.0.get(&a).unwrap_or(&0)
            }
            fn w8(&mut self, _a: u32, _v: u8) {
                unreachable!("0x570BA0 solo lee")
            }
        }
        let mut ro = Ro(&self.mem.0);
        let head = ro.r32(HEAD);
        if head == 0 {
            return None;
        }
        Some(crate::walk::shore_value(&mut ro, head, x as u32, y as u32, route))
    }

    /// Posición de dibujo de cada carro: `(variante del juego, rumbo, fotograma, x, y)` con `x, y` = `+0xE`, `+0x12`.
    pub fn frames(&self) -> Vec<(usize, u8, u8, i32, i32)> {
        self.carts.iter().map(|c| (c.variant, c.cart.heading, c.cart.frame, i32::from(c.cart.px), i32::from(c.cart.py))).collect()
    }
}

struct NoEnv;
impl CartEnv for NoEnv {
    fn remove(&mut self) {}
    fn add(&mut self, _dpy: i32) {}
}

pub fn read_cart(o: &SceneMem) -> Cart {
    Cart {
        x: o.r32(0xC),
        y: o.r32(0x10),
        px: o.r16(0xE) as i16,
        py: o.r16(0x12) as i16,
        heading: o.r8(0x14),
        frame: o.r8(0x15),
        x30: o.r32(0x30),
        y30: o.r32(0x34),
        idx: o.r8(0x38),
        prev: o.r8(0x39),
        dir: o.r8(0x3A),
        n: o.r8(0x3B),
        dur: o.r32(0x40),
        start: o.r32(0x44),
        prog: o.r32(0x48),
        frac: o.r8(0x4C),
        vx: o.r32(0x50) as i32,
        vy: o.r32(0x54) as i32,
        h58: o.r8(0x58),
        done: o.r8(0x5A) != 0,
        s5b: o.r8(0x5B),
        s5c: o.r8(0x5C),
        s5d: o.r8(0x5D),
    }
}

struct SpawnEnv<'a> {
    mem: &'a mut Flat,
    rng: &'a mut CrtRand,
    clock: u32,
    routes: &'a [(Vec<(i32, i32)>, Vec<(i32, i32)>)],
    ok: bool,
}

impl SpawnEnv<'_> {
    fn route_of<'b>(&'b self, o: &SceneMem) -> Route<'b> {
        let r = &self.routes[usize::from(o.r8(0x3C))];
        Route { a: &r.0, b: &r.1 }
    }
}

macro_rules! unused {
    ($($name:ident($($arg:ident: $t:ty),*) $(-> $ret:ty)?;)*) => {
        $(fn $name(&mut self, $($arg: $t),*) $(-> $ret)? { let _ = ($(&$arg),*); unreachable!(concat!(stringify!($name), ": no la usa el tipo 31")) })*
    };
}

impl TypeEnv for SpawnEnv<'_> {
    fn rand(&mut self) -> u32 {
        self.rng.next()
    }
    fn f559cf0(&mut self, _set: u32) {}
    fn f559790(&mut self, _set: u32) {}
    fn g(&mut self, which: u32, _arg: u32) -> u32 {
        assert_eq!(which, 2, "el tipo 31 solo lee el reloj");
        self.clock
    }
    fn rd8(&mut self, a: u32) -> u8 {
        self.mem.r8(a)
    }
    fn rd16(&mut self, a: u32) -> u16 {
        self.mem.r16(a)
    }
    fn rd32(&mut self, a: u32) -> u32 {
        self.mem.r32(a)
    }
    fn wr8(&mut self, a: u32, v: u8) {
        self.mem.w8(a, v)
    }
    fn wr16(&mut self, a: u32, v: u16) {
        self.mem.w16(a, v)
    }
    fn wr32(&mut self, a: u32, v: u32) {
        self.mem.w32(a, v)
    }
    fn scene_r32(&mut self, off: u32) -> u32 {
        self.mem.r32(SCENE + off)
    }
    fn scene_r8(&mut self, off: u32) -> u8 {
        self.mem.r8(SCENE + off)
    }
    fn scene_r16(&mut self, off: u32) -> u16 {
        self.mem.r16(SCENE + off)
    }
    fn f4d8db0(&mut self, idx: u32) -> u32 {
        ENTRIES + 24 * idx
    }
    fn f570800(&mut self, o: &mut SceneMem) {
        let mut c = read_cart(o);
        let r = self.route_of(o);
        carts::retarget(&mut c, &r);
        o.w32(0x50, c.vx as u32);
        o.w32(0x54, c.vy as u32);
        o.w8(0x58, c.h58);
    }
    fn f5709c0(&mut self, o: &SceneMem) -> u8 {
        let c = read_cart(o);
        carts::heading_next(&c, &self.route_of(o))
    }
    unused! {
        f557730(set: u32, n: u32) -> u8;
        f559260(set: u32) -> bool;
        f59b5b0(which: u32);
        f559810(set: u32);
        f55e620(args: [u32; 10]);
        rec_byte(side: u8, off: u32) -> u8;
        rec_word(side: u8, off: u32) -> u16;
        f55a290(a: [u32; 4]);
        f559130(a: u32, set: u32);
        bump_c58c();
        v1c();
        shift_w(d: i32);
        f559370(set: u32);
        f59a260();
        f_place(which: u32, o: &mut SceneMem, a: u32, b: u32);
        f557730_2c(set: u32, n: u32) -> u8;
        f55e250(set: u32);
        f55e430(set: u32);
        f571da0();
        f58cfb0(a: u32, b: u32) -> (u32, u32);
        f563090(o: &mut SceneMem);
        f57af60(set: u32, flag: u32);
        f5986a0(a: u16, b: u16);
        registry_push(obj: u32);
        f58d030(x: i32, y: i32) -> (u32, u32);
        f630cc0(side: u8, a: u32, b: u32, size: u32);
        f58b740(a: &mut u32, b: &mut u32, dir: u8);
        f56a240(o: &mut SceneMem);
    }
}
