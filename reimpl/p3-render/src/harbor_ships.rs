//! Aparición de los barcos del puerto (tipo 13): `0x56C7B0` (`objeto.método(x, y, rumbo, enlace, selector)`).
//!
//! Observado y probado en emulador (`oracle/x86_harbor_ship.py`, 3.000 casos con el x86 real). Los campos usan las posiciones del `this` del método
//! (`objeto + 4` de la fábrica). Las llamadas externas (`0x5303C0`, `0x559CC0`, `0x559790`, `0x4885A0`, `0x56CA30`, `0x56CB90`, `0x6087F0`, `0x568A70`,
//! `0x590610`, `0x56E000`) se sustituyen por [`ShipEnv`] y se registran.

#[derive(Clone, Debug, Default)]
pub struct HarborShip {
    /// `+0xC`, `+0x10`
    pub x: u32,
    pub y: u32,
    /// `+0xE`, `+0x12`: parte alta de `x` e `y`.
    pub px: i16,
    pub py: i16,
    pub heading: u8,
    pub link: u16,
    /// `+0x20`, `+0x24`: posición entera de partida.
    pub sx: i32,
    pub sy: i32,
    /// `+0x28`: clase de sprite.
    pub class: u8,
    /// `+0x2C`: enlace con el barco anterior de la lista.
    pub prev: u32,
    /// `+0x30`: color ARGB.
    pub colour: u32,
    /// `+0x34`: identificador de barco.
    pub id: u16,
    /// `+0x36`, `+0x44`
    pub flag36: u8,
    pub f44: u8,
    /// `+0x38`, `+0x3C`: desplazamiento fijo (lo escribe otra parte).
    pub off_x: i32,
    pub off_y: i32,
    /// `+0x48`, `+0x4C`
    pub f48: u32,
    pub f4c: u32,
}

pub struct ShipCtx<'a> {
    /// Primer byte del registro del barco (`DAT_006DD7A4 + id * 0x180`) y su palabra en `+0x134`.
    pub owner: u8,
    pub kind: u16,
    /// `DAT_006DFC14`, `DAT_006DE4AA`
    pub player: u32,
    pub owner_count: u16,
    /// `0x6E198C + k * 0x40`: contador por entrada.
    pub counts: &'a [u32],
    /// `0x6E1964 + clase * 0x40`
    pub ndirs: u32,
    /// Puntos de anclaje de la tabla `0x6E1864 + (clase / 20) * 0x18`: `(x, y)` por dirección.
    pub anchors: &'a [(i16, i16)],
    /// `DAT_006E59CC != 0xFF`
    pub has_extra: bool,
    /// `DAT_006E2D50`
    pub list_head: u32,
}

pub trait ShipEnv {
    /// `0x5303C0(propietario)`: palabra en `+8` del registro.
    fn owner_word(&mut self, owner: u8) -> u16;
    /// `0x559CC0(máximo)`
    fn begin(&mut self, max: u32);
    /// `0x559790(tabla de clase, rejilla, 0)`
    fn occupy(&mut self, class: u8);
    /// `0x4885A0(&lista, 0, 1)`: identificadores de barcos especiales.
    fn special(&mut self) -> Vec<u16>;
    fn init_a(&mut self);
    fn init_b(&mut self);
    /// `0x6087F0(id)`
    fn extra(&mut self, id: u16) -> u32;
    /// `0x568A70(enlace)`
    fn anchor_id(&mut self, link: u16) -> u32;
    /// `0x590610(x, y, 0x1E, base, 0x40, ancla, 0)`
    fn attach(&mut self, x: u32, y: u32, anchor: u32) -> u32;
    fn finish(&mut self);
}

/// `0x56C7B0`.
pub fn spawn(s: &mut HarborShip, c: &ShipCtx, x: u32, y: u32, heading: u8, link: u16, selector: u32, env: &mut dyn ShipEnv) {
    let g = (selector >> 16) as u8;
    s.id = selector as u16;
    s.class = ((selector >> 24) as u8).wrapping_add(g.wrapping_mul(0x14));
    s.colour = if u32::from(c.owner) == c.player {
        0xff00_00ff
    } else if c.kind == 0x12 {
        0xff28_2828
    } else if u16::from(c.owner) < c.owner_count && env.owner_word(c.owner) == 0 {
        0xffff_0000
    } else {
        0xffff_ffff
    };
    let mut max = 0u32;
    for k in 0..20 {
        let v = c.counts[usize::from(g) * 20 + k];
        if v > max {
            max = v;
        }
    }
    s.sy = i32::from((y >> 16) as i16);
    s.sx = i32::from((x >> 16) as i16);
    env.begin(max);
    s.y = y;
    s.x = x;
    s.px = (x >> 16) as i16;
    s.py = (y >> 16) as i16;
    s.heading = heading;
    s.link = link;
    env.occupy(s.class);
    s.prev = c.list_head;
    let list = env.special();
    if list.iter().any(|&v| v == s.id) {
        s.flag36 = 1;
        s.f44 = 0xff;
    }
    env.init_a();
    env.init_b();
    if c.has_extra {
        s.f48 = env.extra(s.id);
    }
    let idx = ((u32::from(heading) * c.ndirs + 0x80) >> 8) as u8 & (c.ndirs as u8).wrapping_sub(1);
    let (ax, ay) = c.anchors[usize::from(idx)];
    let bx = (i32::from(s.px) + s.off_x + i32::from(ax)) << 16;
    let by = (i32::from(ay) + i32::from(s.py) + s.off_y) << 16;
    let anchor = env.anchor_id(s.link);
    s.f4c = env.attach(bx as u32, by as u32, anchor);
    env.finish();
}

/// Estado de los campos que lee y escribe `0x56E000` (en el marco del objeto base, no del `this` de la aparición).
#[derive(Clone, Debug, Default)]
pub struct ShipCondition {
    /// `+0x12`, `+0x16`: parte alta de `x` y de `y` (marco del objeto base: `this - 4`).
    pub px: i16,
    pub py: i16,
    /// `+0x18`: rumbo.
    pub heading: u8,
    /// `+0x1A`: enlace.
    pub link: u16,
    /// `+0x2C`: clase de sprite (el nivel de daño ocupa `clase % 20 / 5`).
    pub class: u8,
    /// `+0x3C`, `+0x40`
    pub off_x: i32,
    pub off_y: i32,
    /// `+0x49`: nivel mostrado de casco (0..4).
    pub shown: u8,
    /// `+0x50`: ancla del humo (0 = ninguna).
    pub smoke: u32,
}

pub trait ConditionEnv {
    /// Método virtual `+0x4C` y `0x5595F0`: saca el barco de la rejilla.
    fn remove(&mut self);
    /// Método virtual `+0x4C`, `0x559790` y `0x59B5B0`: lo vuelve a meter y marca el rectángulo.
    fn add(&mut self);
    /// Método `+4` del subobjeto `escena+0x3F8` con el ancla.
    fn detach(&mut self, handle: u32);
    fn anchor_id(&mut self, link: u16) -> u32;
    fn attach(&mut self, x: u32, y: u32, anchor: u32) -> u32;
    /// Método virtual `+0x1C`.
    fn notify(&mut self);
}

/// `0x56E000`: nivel de daño del casco (`hull * 100 / max`) → clase de sprite, humo y aviso.
pub fn update_condition(
    s: &mut ShipCondition,
    hull: i32,
    max_hull: i32,
    pct_flag: bool,
    ndirs: &dyn Fn(u8) -> u32,
    anchor: &dyn Fn(u8, u8) -> (i16, i16),
    env: &mut dyn ConditionEnv,
) {
    let mut pct = hull.wrapping_mul(100).wrapping_div(max_hull);
    if pct != 0 {
        pct -= 1;
    }
    if pct > 99 {
        pct = 99;
    }
    let mut level = 0i8;
    let mut shown = 4u8;
    if !pct_flag {
        shown = (pct / 10) as u8;
        if shown > 4 {
            shown = 4;
        }
    } else {
        level = 3 - (pct / 25) as i8;
    }
    let cur = ((s.class % 20) / 5) as i8;
    if level != cur {
        let new_class = (level.wrapping_mul(5)).wrapping_sub(cur.wrapping_mul(5)).wrapping_add(s.class as i8) as u8;
        env.remove();
        s.class = new_class;
        env.add();
        if level == 3 && s.smoke != 0 {
            env.detach(s.smoke);
            s.smoke = 0;
        } else if s.smoke == 0 {
            let nd = ndirs(s.class);
            let idx = ((u32::from(s.heading) * nd + 0x80) >> 8) as u8 & (nd as u8).wrapping_sub(1);
            let (ax, ay) = anchor(s.class, idx);
            let bx = (i32::from(s.px) + s.off_x + i32::from(ax)) << 16;
            let by = (i32::from(ay) + i32::from(s.py) + s.off_y) << 16;
            let a = env.anchor_id(s.link);
            s.smoke = env.attach(bx as u32, by as u32, a);
        }
    }
    if shown != s.shown {
        s.shown = shown;
        env.notify();
    }
}
