//! Actualización por fotograma del barco del puerto (tipo 13): `0x56D060`, ranura 9 de la vtable `0x6758E8`.
//!
//! Observado y probado en emulador (`oracle/x86_harbor_sprite.py`, 3.000 casos con el x86 real). Hace cuatro cosas:
//! 1. **Fundido de la marca de selección** (`+0x36` = 2 sube, 3+ baja; `+0x44` en pasos de 0x1E cada 20 ms).
//! 2. **Repintados** de la marca, del anillo y del nombre según las banderas del barco del mundo (`+0x3C` del registro).
//! 3. **Balanceo en el muelle**: si el barco del mundo está en los estados 0, 1, 6, 7, 8, 9 o 0xC y venció `+0x40`,
//!    `+0x38`/`+0x3C` se mueven `rand()%3-1` con topes ±2 y ±1, se recoloca el humo (`+0x4C`, método +0x48) en el
//!    anclaje del rumbo, se marca un píxel junto al casco (`0x5986A0`, `class22_put_pixel_at_map_position`: valor 0xFF en la capa
//!    `escena+0x5A0` si `escena+0x524` está activa) y se programa el siguiente balanceo
//!    `rand()%4000` ms más tarde.
//! 4. **Contador `+0x8C`** (se descuenta 10 por cada 40 ms transcurridos desde `+0x90`, con `0x585DC0`).
//!
//! Los campos usan las posiciones del `this` del método (objeto + 4), como [`crate::harbor_ships::HarborShip`]. Las llamadas
//! externas se sustituyen por [`SpriteEnv`]; `UnionRect` se reproduce aquí con la semántica de Windows.

/// Rectángulo de Windows (`left, top, right, bottom`).
pub type Rect = [i32; 4];

/// Estado del sprite que lee y escribe `0x56D060`.
#[derive(Clone, Debug, Default)]
pub struct SpriteState {
    /// `+0xE`, `+0x12`: posición entera (parte alta de `x`/`y`).
    pub px: i16,
    pub py: i16,
    /// `+0x14`
    pub heading: u8,
    /// `+0x28`
    pub class: u8,
    /// `+0x34`: identificador del barco del mundo.
    pub id: u16,
    /// `+0x36`: 0 sin marca, 1 marca fija, 2 aparece, 3+ desaparece.
    pub flag36: u8,
    /// `+0x38`, `+0x3C`: desplazamiento del balanceo.
    pub off_x: i32,
    pub off_y: i32,
    /// `+0x40`: próximo instante (ms) del fundido o del balanceo.
    pub next: u32,
    /// `+0x44`: opacidad de la marca.
    pub f44: u8,
    /// `+0x46`: segunda marca (hoja `0x6E190C`).
    pub f46: u8,
    /// `+0x48`: si no es 0, se repinta el anillo.
    pub f48: u32,
    /// `+0x4C`: hay emisor de humo enganchado.
    pub has_emitter: bool,
    /// `+0x8C`, `+0x90`, `+0x94`
    pub counter: i16,
    pub counter_at: u32,
    pub counter_on: u8,
    /// `objeto + 0x34` (= `this + 0x30`): color del propietario.
    pub colour: u32,
}

/// Datos del mundo y de la escena que lee la función.
pub struct SpriteCtx<'a> {
    /// `DAT_006DCCF8`: reloj en ms.
    pub clock: u32,
    /// Registro del barco (`DAT_006DD7A4 + id * 0x180`): propietario (byte 0), banderas `+0x3C` y estado `+0x134`.
    pub owner: u8,
    pub flags: u16,
    pub kind: u16,
    /// `DAT_006DFC14`, `DAT_006DE4AA`
    pub player: u32,
    pub owner_count: u16,
    /// Valor del método +0x70 de la escena (desplazamiento vertical de la vista).
    pub view_lift: i32,
    /// `escena+0xC650`, `escena+0xC654`
    pub cam_x: i32,
    pub cam_y: i32,
    /// Anchura y altura de las hojas `[0x6E1924]` (marca) y `[0x6E190C]` (segunda marca): `u16` en `+4`/`+6`.
    pub sheet_a: (u16, u16),
    pub sheet_b: (u16, u16),
    /// `0x6E1938 + (clase / 20) * 4`
    pub lift: i32,
    /// `DAT_006E2D58` (solo el byte bajo).
    pub b2d58: u32,
    /// `0x6E1964 + clase * 0x40`: número de direcciones de la hoja.
    pub ndirs: u32,
    /// Tabla de anclajes `[0x6E1864 + (clase / 20) * 0x18]`: `(x, y)` por dirección.
    pub anchors: &'a [(i16, i16)],
    /// `0x6C4000 + (clase / 20) * 4`: divisor de la distancia del píxel al casco.
    pub pixel_div: i32,
    /// `DAT_006E51B0` (float): escala vertical del píxel.
    pub pixel_scale: f32,
}

pub trait SpriteEnv {
    fn rand(&mut self) -> i32;
    /// `0x56E000`: nivel de daño (el resultado no se usa aquí).
    fn damage(&mut self);
    /// `0x56CA30`
    fn refresh(&mut self);
    /// `0x56B7A0(&rect)`: rectángulo del nombre / marca del jugador.
    fn label_rect(&mut self) -> Rect;
    /// `0x56BAA0(&rect)`: rectángulo del anillo de selección.
    fn ring_rect(&mut self) -> Rect;
    /// `0x559370(&out, hoja de la clase, objeto+0xC)`: saca el sprite de las celdas (antes de mover). Durante la llamada la
    /// posición `+0xE`/`+0x12` del objeto vale `(x, y)`, ya con el balanceo sumado.
    fn unlink(&mut self, class: u8, x: i16, y: i16);
    /// `0x559130(&out, hoja de la clase, objeto+0xC)`: lo vuelve a enlazar con el balanceo nuevo en `(x, y)`.
    fn link(&mut self, class: u8, x: i16, y: i16);
    /// `0x59B5B0(&rect)` de la escena.
    fn repaint(&mut self, r: Rect);
    /// `0x5986A0(x, y)` (`class22_put_pixel_at_map_position`, palabras bajas): píxel 0xFF en la capa `escena+0x5A0`.
    fn map_pixel(&mut self, x: i16, y: i16);
    /// `0x585DC0` sobre `this + 0x50`.
    fn counter(&mut self);
    /// `0x5303C0(propietario)`: palabra `+8` del registro devuelto.
    fn owner_word(&mut self, owner: u8) -> u16;
    /// `0x4B2200` del minimapa (`[0x6E51C0]`).
    fn minimap_visible(&mut self) -> bool;
    /// `0x59C930(objeto, &rect)` del minimapa.
    fn minimap_rect(&mut self) -> Rect;
    /// `0x4B9650(&rect)` sobre `0x6DA5F0`.
    fn minimap_repaint(&mut self, r: Rect);
    /// Método +0x48 del emisor `+0x4C`: `(x << 16, y << 16, 0)`.
    fn emit(&mut self, x: i32, y: i32);
}

fn empty(r: &Rect) -> bool {
    r[2] <= r[0] || r[3] <= r[1]
}

/// `UnionRect` de user32.
pub fn union_rect(a: &Rect, b: &Rect) -> Rect {
    match (empty(a), empty(b)) {
        (true, true) => [0; 4],
        (true, false) => *b,
        (false, true) => *a,
        _ => [a[0].min(b[0]), a[1].min(b[1]), a[2].max(b[2]), a[3].max(b[3])],
    }
}

/// Rectángulo de la marca (hoja `0x6E1924`).
fn mark_rect(s: &SpriteState, c: &SpriteCtx) -> Rect {
    let (w, h) = (i32::from(c.sheet_a.0), i32::from(c.sheet_a.1));
    let top = c.view_lift
        .wrapping_add(i32::from(s.py))
        .wrapping_sub(c.lift)
        .wrapping_add(s.off_y)
        .wrapping_sub(h)
        .wrapping_sub((c.b2d58 & 0xFF) as i32)
        .wrapping_sub(c.cam_y);
    let left = i32::from(s.px).wrapping_sub(w >> 1).wrapping_add(s.off_x).wrapping_sub(c.cam_x);
    [left, top, left.wrapping_add(w), top.wrapping_add(h)]
}

/// Rectángulo de la segunda marca (hoja `0x6E190C`), por debajo del casco.
fn mark_b_rect(s: &SpriteState, c: &SpriteCtx) -> Rect {
    let (w, h) = (i32::from(c.sheet_b.0), i32::from(c.sheet_b.1));
    let top = c.view_lift
        .wrapping_add(i32::from(s.py))
        .wrapping_sub(c.lift)
        .wrapping_add(s.off_y)
        .wrapping_add(h)
        .wrapping_sub(c.cam_y);
    let left = i32::from(s.px).wrapping_sub(w >> 1).wrapping_add(s.off_x).wrapping_sub(c.cam_x);
    [left, top, left.wrapping_add(w), top.wrapping_add(h)]
}

/// Seno de la tabla `0x675788` (65 entradas, cuartos de vuelta), como la del puerto.
fn sin_t(u: u8) -> i32 {
    p3_core::ships::harbor_list::sin_t(u)
}

/// `0x56D060`. Devuelve siempre 1.
pub fn update(s: &mut SpriteState, c: &SpriteCtx, env: &mut dyn SpriteEnv) -> u8 {
    // 1. Fundido de la marca.
    if s.flag36 > 1 && s.next < c.clock {
        if s.flag36 == 2 {
            s.f44 = s.f44.wrapping_add(0x1E);
            if s.f44 >= 0xA0 {
                s.flag36 = 3;
            }
        } else {
            s.f44 = s.f44.wrapping_add(0xE2);
            if s.f44 <= 0x1E {
                s.flag36 = 0;
            }
        }
        env.repaint(mark_rect(s, c));
        s.next = c.clock.wrapping_add(0x14);
    }
    env.damage();
    let f = c.flags;
    if s.f48 != 0 && (f & 2 != 0 || f & 0x100 != 0 || f & 0x400 != 0 || f & 0x10 != 0) {
        let r = env.ring_rect();
        env.repaint(r);
    }
    if f & 1 != 0 {
        let r = env.label_rect();
        env.repaint(r);
    }
    if f & 0x400 != 0 {
        s.colour = if u32::from(c.owner) == c.player {
            0xff00_00ff
        } else if c.kind == 0x12 {
            0xff28_2828
        } else if u16::from(c.owner) < c.owner_count && env.owner_word(c.owner) == 0 {
            0xffff_0000
        } else {
            0xffff_ffff
        };
        if env.minimap_visible() {
            let r = env.minimap_rect();
            env.minimap_repaint(r);
        }
        if s.counter != 0 {
            env.counter();
            s.counter = 0;
        }
        if s.flag36 != 0 {
            env.repaint(mark_rect(s, c));
        }
        env.refresh();
        let r = env.label_rect();
        env.repaint(r);
    }
    // 3. Balanceo.
    if matches!(c.kind, 0 | 1 | 6 | 7 | 8 | 9 | 0xC) && c.clock >= s.next {
        env.unlink(s.class, s.px.wrapping_add(s.off_x as i16), s.py.wrapping_add(s.off_y as i16));
        let mine = u32::from(c.owner) == c.player;
        let old_label = if mine { Some(env.label_rect()) } else { None };
        let old_mark = (s.flag36 != 0).then(|| mark_rect(s, c));
        let old_b = (s.f46 != 0).then(|| mark_b_rect(s, c));
        s.off_x = s.off_x.wrapping_add(env.rand() % 3 - 1);
        s.off_y = s.off_y.wrapping_add(env.rand() % 3 - 1);
        s.off_x = s.off_x.clamp(-2, 2);
        s.off_y = s.off_y.clamp(-1, 1);
        env.link(s.class, s.px.wrapping_add(s.off_x as i16), s.py.wrapping_add(s.off_y as i16));
        if let Some(old) = old_label {
            let r = env.label_rect();
            env.repaint(union_rect(&r, &old));
        }
        if let Some(old) = old_mark {
            env.repaint(union_rect(&mark_rect(s, c), &old));
        }
        if let Some(old) = old_b {
            env.repaint(union_rect(&mark_b_rect(s, c), &old));
        }
        if s.has_emitter {
            let n = c.ndirs;
            let idx = ((u32::from(s.heading).wrapping_mul(n).wrapping_add(0x80) >> 8) as u8) & (n as u8).wrapping_sub(1);
            let (ax, ay) = c.anchors[usize::from(idx)];
            let y = i32::from(s.py).wrapping_add(s.off_y).wrapping_add(i32::from(ay));
            let x = i32::from(s.px).wrapping_add(s.off_x).wrapping_add(i32::from(ax));
            env.emit(x.wrapping_shl(16), y.wrapping_shl(16));
        }
        let r = env.rand();
        let d = r % c.pixel_div;
        let h = s.heading;
        let a = sin_t(h).wrapping_mul(d);
        let side = (f64::from(a as f32) * f64::from(0.8f32) / -65536.0) as i32;
        let bx = (r % 10).wrapping_sub(side).wrapping_sub(5);
        let b = sin_t(h.wrapping_add(0x40)).wrapping_mul(d);
        let along = (f64::from(b as f32) * f64::from(c.pixel_scale) * f64::from(0.8f32) / 65536.0) as i32;
        let (cy, dx) = if c.clock & 1 != 0 {
            ((s.off_y as i16).wrapping_sub(along as i16), s.px.wrapping_add(bx as i16))
        } else {
            ((s.off_y as i16).wrapping_add(along as i16), s.px.wrapping_sub(bx as i16))
        };
        env.map_pixel(dx.wrapping_add(s.off_x as i16), cy.wrapping_add(s.py));
        s.next = c.clock.wrapping_add((env.rand() % 4000) as u32);
    }
    env.refresh();
    // 4. Contador.
    if s.counter != 0 && s.counter_on != 0 && c.clock > s.counter_at {
        let steps = (c.clock - s.counter_at) / 40 + 1;
        let v = (s.counter as u16).wrapping_sub((steps.wrapping_mul(10)) as u16) as i16;
        s.counter = v.max(0);
        s.counter_at = s.counter_at.wrapping_add(0x28);
        if s.counter_at <= c.clock {
            s.counter_at = c.clock.wrapping_add(0x28);
        }
        env.counter();
    }
    1
}
