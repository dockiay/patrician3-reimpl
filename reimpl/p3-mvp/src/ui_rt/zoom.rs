//! Modern city-view magnification. UI and the system/game cursor keep their size.
//! Only the scene's mouse-event point is mapped back to the unscaled image.
use p3_render::scene_math::Mem;

const SCENE: u32 = 0x006E_51AC;
const DRAW: u32 = 0x0058_8880;
const MOUSE: u32 = 0x0058_7AA0;
const SCROLL: u32 = 0x0059_0AE0;
/// Ranura +0x40 de las ventanas: copia `+0x2C..+0x38` (ancho, alto, +0x34) a `[esp+4]` (thiscall, `ret 4`).
const SCENE_SIZE: u32 = 0x0040_2470;

/// `P3_ZOOM_ALWAYS_RESIZE`: recrea el búfer del suelo en cada cambio de tamaño (comportamiento anterior; diagnóstico).
fn always_resize() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| std::env::var_os("P3_ZOOM_ALWAYS_RESIZE").is_some())
}

/// `P3_ZOOM_INCREMENTAL`: no pasa a la ruta virtual hasta que se usa la rueda (comportamiento anterior; diagnóstico).
fn always_incremental() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| std::env::var_os("P3_ZOOM_INCREMENTAL").is_some())
}

pub struct Zoom {
    pub enabled: bool,
    percent: i32,
    remainder: i32,
    rect: [i32; 4],
    origin: (f64, f64),
    raw: Vec<u8>,
    skip: Option<u32>,
    virtualized: bool,
    source_size: (u32, i32, i32),
    canvas: Option<super::Surface>,
    anchor: Option<(f64, f64, (i32, i32))>,
    /// Tamaño con el que se creó el búfer del suelo del renderizador (renderizador, ancho, alto): solo se recrea si hace falta uno mayor.
    allocated: (u32, i32, i32),
    /// Durante el clic de la escena con el lienzo virtual: escena y su tamaño físico (escena, ancho, alto).
    mouse_res: Option<(u32, u32, u32)>,
}

impl Default for Zoom {
    fn default() -> Self {
        Self {
            enabled: false,
            percent: 100,
            remainder: 0,
            rect: [0; 4],
            origin: (0.0, 0.0),
            raw: Vec::new(),
            skip: None,
            virtualized: false,
            source_size: (0, 0, 0),
            canvas: None,
            anchor: None,
            allocated: (0, 0, 0),
            mouse_res: None,
        }
    }
}

impl Zoom {
    pub fn percent(&self) -> i32 {
        self.percent
    }

    /// Actual world-rendering extent, before scaling into the physical viewport.
    pub fn rendered_size(&self) -> (i32, i32) {
        (self.source_size.1, self.source_size.2)
    }

    /// Last full scene image, useful for diagnosing coverage below 100%.
    pub fn source_canvas(&self) -> Option<&super::Surface> {
        self.canvas.as_ref()
    }

    /// Display coordinate to the scene's original coordinate; UI stays physical.
    pub fn scene_point(&self, p: (i32, i32)) -> (i32, i32) {
        if self.enabled && self.contains(p) {
            self.source(p)
        } else {
            p
        }
    }

    fn configure(&mut self, rect: [i32; 4]) {
        if self.rect != rect {
            self.rect = rect;
            self.origin = (0.0, 0.0);
            self.raw.clear();
            self.source_size = (0, 0, 0);
            self.anchor = None;
        }
    }

    fn contains(&self, p: (i32, i32)) -> bool {
        let [l, t, r, b] = self.rect;
        (l..r).contains(&p.0) && (t..b).contains(&p.1)
    }

    fn source(&self, p: (i32, i32)) -> (i32, i32) {
        let [l, t, r, b] = self.rect;
        let scale = self.percent as f64 / 100.0;
        // Avoid moving an exact pixel boundary left/up due to roundoff after
        // changing scales (e.g. 300 * (1 - 1/1.25) = 59.999999999999986).
        let pixel = |v: f64| (v + 1e-9).floor() as i32;
        let (right, bottom) = if self.virtualized && self.source_size.1 > 0 {
            (l + self.source_size.1, t + self.source_size.2)
        } else {
            (
                l + ((r - l) * 100 + self.percent.min(100) - 1) / self.percent.min(100),
                t + ((b - t) * 100 + self.percent.min(100) - 1) / self.percent.min(100),
            )
        };
        (
            (l + pixel(self.origin.0 + (p.0 - l) as f64 / scale)).clamp(l, right - 1),
            (t + pixel(self.origin.1 + (p.1 - t) as f64 / scale)).clamp(t, bottom - 1),
        )
    }

    fn wheel(&mut self, delta: i32, p: (i32, i32)) {
        self.remainder += delta;
        let steps = self.remainder / 120;
        self.remainder %= 120;
        let old = self.percent as f64 / 100.0;
        let base = if steps > 0 {
            self.percent / 25 * 25
        } else if steps < 0 {
            (self.percent + 24) / 25 * 25
        } else {
            self.percent
        };
        self.percent = (base + steps * 25).clamp(50, 250);
        let new = self.percent as f64 / 100.0;
        let [l, t, r, b] = self.rect;
        self.origin.0 = (self.origin.0 + (p.0 - l) as f64 * (1.0 / old - 1.0 / new))
            .clamp(0.0, ((r - l) as f64 * (1.0 - 1.0 / new)).max(0.0));
        self.origin.1 = (self.origin.1 + (p.1 - t) as f64 * (1.0 / old - 1.0 / new))
            .clamp(0.0, ((b - t) as f64 * (1.0 - 1.0 / new)).max(0.0));
    }

    fn restore(&self, surface: &mut super::Surface) {
        let [l, t, r, b] = self.rect;
        let w = (r - l) as usize;
        if self.raw.len() != w * (b - t) as usize * 4 {
            return;
        }
        for y in t..b {
            let dst = (y as usize * surface.width + l as usize) * 4;
            let src = (y - t) as usize * w * 4;
            surface.rgba[dst..dst + w * 4].copy_from_slice(&self.raw[src..src + w * 4]);
        }
    }

    fn magnify(&mut self, surface: &mut super::Surface) {
        let [l, t, r, b] = self.rect;
        let w = (r - l) as usize;
        self.raw.resize(w * (b - t) as usize * 4, 0);
        for y in t..b {
            let src = (y as usize * surface.width + l as usize) * 4;
            let dst = (y - t) as usize * w * 4;
            self.raw[dst..dst + w * 4].copy_from_slice(&surface.rgba[src..src + w * 4]);
        }
        if self.percent == 100 {
            return;
        }
        // Nearest neighbour preserves the original pixel art, with exactly the
        // same inverse mapping as picking. Reuse the raw frame across draws.
        let xs: Vec<usize> = (l..r).map(|x| (self.source((x, t)).0 - l) as usize * 4).collect();
        for y in t..b {
            let sy = (self.source((l, y)).1 - t) as usize * w * 4;
            let dst = (y as usize * surface.width + l as usize) * 4;
            for (x, &sx) in xs.iter().enumerate() {
                surface.rgba[dst + x * 4..dst + x * 4 + 4].copy_from_slice(&self.raw[sy + sx..sy + sx + 4]);
            }
        }
    }
}

impl super::Runtime<'_> {
    // The original camera assumes that the viewport fits inside the tile grid.
    // Leave room for the extra anchor tile and the staggered bottom row; asking
    // for more makes its upper camera bound negative and reads adjacent memory.
    fn zoom_minimum(&self, m: &mut dyn Mem, scene: u32, rect: [i32; 4]) -> i32 {
        let tw = (m.r32(scene + 0x510) as i32).max(1);
        let th = (m.r32(scene + 0x514) as i32 + 1).max(1);
        let available_w = m.r32(scene + 0xC31C) as i32 - m.r32(scene + 0x4D0) as i32 - tw;
        let available_h = m.r32(scene + 0xC320) as i32 - m.r32(scene + 0x4D4) as i32 - 2 * th;
        if available_w <= 0 || available_h <= 0 {
            return 100;
        }
        let ceil = |n: i32, d: i32| (n * 100 + d - 1) / d;
        50.max(ceil(rect[2] - rect[0], available_w))
            .max(ceil(rect[3] - rect[1], available_h))
            .min(100)
    }

    /// Resize the original scene renderer, retaining physical window geometry
    /// for the window manager. One extra tile allows sub-tile cursor anchoring.
    fn zoom_virtual_size(&mut self, m: &mut dyn Mem, scene: u32) {
        let [l, t, r, b] = self.zoom.rect;
        self.zoom.percent = self.zoom.percent.max(self.zoom_minimum(m, scene, self.zoom.rect));
        let tw = (m.r32(scene + 0x510) as i32).max(1);
        let th = (m.r32(scene + 0x514) as i32 + 1).max(1);
        let p = self.zoom.percent.min(100);
        let w = ((r - l) * 100 + p - 1) / p + tw;
        let h = ((b - t) * 100 + p - 1) / p + th;
        let geometry = [m.r32(scene + 0x2C), m.r32(scene + 0x30), m.r32(scene + 0x34)];
        let clip = self.clip.clone();
        let renderer = m.r32(scene + 0x530);
        if self.zoom.source_size != (renderer, w, h) {
            // Recrear el búfer del suelo cuesta ~100 ms (y 3-4 pasos de rueda seguidos congelaban la imagen): un búfer ya creado
            // más grande vale para un tamaño menor, porque el suelo se invalida entero en cada dibujo.
            let (a_renderer, aw, ah) = self.zoom.allocated;
            if always_resize() || a_renderer != renderer || w > aw || h > ah {
                // Se crea ya para el alejamiento máximo, así los pasos siguientes de la rueda no recrean nada.
                let pmin = self.zoom_minimum(m, scene, self.zoom.rect).min(p);
                let (bw, bh) = if always_resize() { (w, h) } else { (((r - l) * 100 + pmin - 1) / pmin + tw, ((b - t) * 100 + pmin - 1) / pmin + th) };
                self.zoom.skip = Some(0x0059_8700);
                self.dispatch(m, 0x0059_8700, scene, &[bw as u32, bh as u32, geometry[2]]);
                self.zoom.allocated = (renderer, bw, bh);
            }
            self.zoom.source_size = (renderer, w, h);
            self.zoom.raw.clear();
        }
        m.w32(scene + 0x2C, w as u32);
        m.w32(scene + 0x30, h as u32);
        if let Some((world_x, world_y, point)) = self.zoom.anchor.take() {
            let scale = self.zoom.percent as f64 / 100.0;
            let target_x = world_x - (point.0 - l) as f64 / scale;
            let target_y = world_y - (point.1 - t) as f64 / scale;
            let x = m.r32(scene + 0xC644) as i32
                + ((target_x - m.r32(scene + 0xC650) as i32 as f64) / tw as f64).floor() as i32;
            let y = m.r32(scene + 0xC640) as i32
                + 2 * ((target_y - m.r32(scene + 0xC654) as i32 as f64) / th as f64).floor() as i32;
            self.zoom.skip = Some(SCROLL);
            self.dispatch(m, SCROLL, scene, &[x as u32, y as u32]);
            self.zoom.origin = (
                (target_x - m.r32(scene + 0xC650) as i32 as f64).clamp(0.0, w as f64 - (r - l) as f64 / scale),
                (target_y - m.r32(scene + 0xC654) as i32 as f64).clamp(0.0, h as f64 - (b - t) as f64 / scale),
            );
        }
        // Resolution changes and ordinary scrolling also have to clamp against
        // the rendered extent, not the smaller physical window dimensions.
        self.zoom.skip = Some(SCROLL);
        let camera = [m.r32(scene + 0xC644), m.r32(scene + 0xC640)];
        self.dispatch(m, SCROLL, scene, &camera);
        for (offset, value) in [0x2C, 0x30, 0x34].into_iter().zip(geometry) {
            m.w32(scene + offset, value);
        }
        self.clip = clip;
    }

    fn zoom_virtual_draw(&mut self, m: &mut dyn Mem, scene: u32, args: &[u32]) -> u32 {
        self.zoom_virtual_size(m, scene);
        let [l, t, r, b] = self.zoom.rect;
        let (_, w, h) = self.zoom.source_size;
        let (sw, sh) = ((l + w + 284) as usize, (t + h) as usize);
        let mut canvas = self
            .zoom
            .canvas
            .take()
            .unwrap_or_else(|| super::Surface::new(sw, sh, [0, 0, 0, 255]));
        if (canvas.width, canvas.height) != (sw, sh) {
            canvas = super::Surface::new(sw, sh, [0, 0, 0, 255]);
        }
        canvas.color = self.surface.color;
        canvas.source = self.surface.source;
        canvas.mode = self.surface.mode;
        canvas.font = self.surface.font;
        canvas.text_mode = self.surface.text_mode;
        let physical = std::mem::replace(&mut self.surface, canvas);
        self.canvas_active = true;
        let clip = self.clip.clone();
        let saved = [
            m.r32(scene + 0x2C),
            m.r32(scene + 0x30),
            m.r32(super::display::RES_X),
            m.r32(super::display::RES_Y),
        ];
        m.w32(scene + 0x2C, w as u32);
        m.w32(scene + 0x30, h as u32);
        m.w32(super::display::RES_X, sw as u32);
        m.w32(super::display::RES_Y, sh as u32);
        self.clip.reset_clipper();
        self.clip.set_clip_rect([0, 0, sw as i32, sh as i32]);
        self.clip.add_clip_rect([l, t, l + w, t + h]);
        // The scrolling ground buffer has its own damage clipper. Its cached
        // rectangles belong to the old viewport; rebuild the complete source
        // texture as well as the displayed scene to avoid transparent holes.
        let buffer = m.r32(scene + 0x530);
        if m.r32(buffer) == 0x0067_787C {
            self.dispatch(m, 0x0058_6230, buffer, &[]);
        }
        self.zoom.skip = Some(DRAW);
        let result = self.dispatch(m, DRAW, scene, args);
        let canvas = std::mem::replace(&mut self.surface, physical);
        self.canvas_active = false;
        self.screen_buffer(self.render_dest);
        m.w32(scene + 0x2C, saved[0]);
        m.w32(scene + 0x30, saved[1]);
        m.w32(super::display::RES_X, saved[2]);
        m.w32(super::display::RES_Y, saved[3]);
        self.clip = clip;
        self.surface.color = canvas.color;
        self.surface.source = canvas.source;
        self.surface.mode = canvas.mode;
        self.surface.font = canvas.font;
        self.surface.text_mode = canvas.text_mode;
        let xs: Vec<usize> = (l..r).map(|x| self.zoom.source((x, t)).0 as usize * 4).collect();
        for y in t..b {
            let sy = self.zoom.source((l, y)).1 as usize * canvas.width * 4;
            let dst = (y as usize * self.surface.width + l as usize) * 4;
            for (x, &sx) in xs.iter().enumerate() {
                self.surface.rgba[dst + x * 4..dst + x * 4 + 4].copy_from_slice(&canvas.rgba[sy + sx..sy + sx + 4]);
            }
        }
        self.zoom.canvas = Some(canvas);
        self.zoom_mirror_scene();
        result
    }

    /// Con dos búferes de pantalla la imagen ampliada debe quedar en los dos. Durante el desplazamiento de cámara (cada paso de
    /// rueda con ancla) el original dibuja la escena en el delantero (`0x592E60`) y `present_frame` copia después las zonas
    /// sucias del trasero al delantero: si la ampliación quedó solo en uno, esas zonas traían píxeles viejos sin ampliar y se veía
    /// un mosaico de trozos (vídeo de Sergio a 2560x1440, 14/11). Se copia el rectángulo de la escena al otro búfer.
    fn zoom_mirror_scene(&mut self) {
        if !self.two_buffers || self.target_tex.is_some() || self.canvas_active || self.front.rgba.len() != self.surface.rgba.len() {
            return;
        }
        let [l, t, r, b] = self.zoom.rect;
        let w = self.surface.width;
        for y in t.max(0) as usize..(b.max(0) as usize).min(self.surface.height) {
            let (a, z) = ((y * w + l.max(0) as usize) * 4, (y * w + (r.max(0) as usize).min(w)) * 4);
            if z > a {
                self.front.rgba[a..z].copy_from_slice(&self.surface.rgba[a..z]);
            }
        }
    }

    fn zoom_game_active(&self, m: &mut dyn Mem) -> bool {
        let gs = m.r32(super::display::GAME_SCREEN);
        let list = super::game_window::SCREEN_LIST;
        let head = m.r32(list + 0xC);
        gs != 0 && m.r32(list + 0x10) != 0 && m.r32(head + 8) == gs
    }

    fn zoom_rect(&self, m: &mut dyn Mem) -> Option<[i32; 4]> {
        let scene = m.r32(SCENE);
        if scene == 0 {
            return None;
        }
        let (l, t) = (m.r32(scene + 0x14) as i32, m.r32(scene + 0x18) as i32);
        let r = l
            .saturating_add(m.r32(scene + 0x2C) as i32)
            .min(self.surface.width as i32);
        let b = t
            .saturating_add(m.r32(scene + 0x30) as i32)
            .min(self.surface.height as i32);
        (l >= 0 && t >= 0 && r > l && b > t).then_some([l, t, r, b])
    }

    pub(super) fn zoom_wheel(&mut self, m: &mut dyn Mem, delta: i32, p: (i32, i32)) -> bool {
        if !self.zoom.enabled {
            return false;
        }
        if !self.zoom_game_active(m) {
            self.zoom.remainder = 0;
            return false;
        }
        let Some(rect) = self.zoom_rect(m) else {
            return false;
        };
        self.zoom.configure(rect);
        let scene = m.r32(SCENE);
        if !self.zoom.contains(p)
            || self.dispatch(m, 0x004B_2200, scene, &[]) as u8 == 0
            || self.dispatch(m, 0x0046_2410, 0, &[]) != 0
        {
            self.zoom.remainder = 0;
            return false;
        }
        let source = self.zoom.source(p);
        let anchor = (
            m.r32(scene + 0xC650) as i32 as f64 + (source.0 - rect[0]) as f64,
            m.r32(scene + 0xC654) as i32 as f64 + (source.1 - rect[1]) as f64,
            p,
        );
        self.zoom.wheel(delta, p);
        self.zoom.percent = self.zoom.percent.max(self.zoom_minimum(m, scene, rect));
        if self.zoom.percent < 100 {
            self.zoom.virtualized = true;
        }
        if self.zoom.virtualized {
            self.zoom.anchor = Some(anchor);
        }
        // Invalidate the scene, including when returning to 100%. Its damage
        // propagates to overlapping windows, without repainting unrelated UI.
        self.dispatch(m, 0x0044_1710, scene, &[]);
        true
    }

    pub(super) fn zoom_native(&mut self, m: &mut dyn Mem, f: u32, ecx: u32, esp: u32) -> Option<u32> {
        // 0x4621F0 visits each popup's vt+0x124 to subtract its physical
        // rectangle from the city clipper. Applying those screen coordinates
        // to the source image makes the excluded area move/scale with zoom,
        // leaving frozen city pixels outside the actual popup. Paint the city
        // source completely; the window manager draws the physical UI after it.
        if f == 0x0046_21F0
            && self.zoom.enabled
            && (self.zoom.percent != 100 || self.zoom.virtualized)
            && self.stack.contains(&DRAW)
        {
            return Some(0);
        }
        // Tamaño de la escena (`0x402470`, ranura +0x40) pedido desde fuera del código de la escena durante un clic con el zoom
        // alejado. El clic necesita el tamaño del lienzo para elegir objetos, pero lo que abre centra su ventana con la escena:
        // la grúa (`class22_on_click_map_object` 0x594DD0 → 0x59AE40 → `trade_parchment_open` 0x4A9690) dejaba «Mercancías
        // comerciales» abajo a la derecha, cortada (captura de Sergio, 15/11). A quien no es la escena se le da el tamaño físico.
        if f == SCENE_SIZE {
            if let Some((scene, w, h)) = self.zoom.mouse_res {
                let caller = m.r32(esp);
                if ecx == scene && !(0x0058_5000..0x005A_0000).contains(&caller) {
                    let out = m.r32(esp + 4);
                    m.w32(out, w);
                    m.w32(out + 4, h);
                    let depth = m.r32(scene + 0x34);
                    m.w32(out + 8, depth);
                    return Some(out);
                }
            }
        }
        if self.zoom.skip == Some(f) {
            self.zoom.skip = None;
            return None;
        }
        // La ruta «virtual» repinta el suelo y la escena enteros en cada fotograma y no depende de zonas sucias ni de copias de
        // desplazamiento. Sergio veía mosaicos de fotogramas viejos al mover el ratón nada más entrar en la ciudad, que desaparecían
        // para siempre tras usar la rueda (esa ruta); se usa desde el primer fotograma de la partida. Cuesta unos 4 ms a 1080p.
        if self.zoom.enabled && !self.zoom.virtualized && f == 0x004B_8A40 && !always_incremental() && self.zoom_game_active(m) && m.r32(SCENE) != 0 {
            self.zoom.virtualized = true;
        }
        if self.zoom.enabled
            && self.zoom.virtualized
            && f == SCROLL
            && ecx == m.r32(SCENE)
            && self.zoom.source_size.1 > 0
        {
            self.sp = self.sp.min(esp.wrapping_sub(0x100));
            let saved = [m.r32(ecx + 0x2C), m.r32(ecx + 0x30)];
            m.w32(ecx + 0x2C, self.zoom.source_size.1 as u32);
            m.w32(ecx + 0x30, self.zoom.source_size.2 as u32);
            let args = [m.r32(esp + 4), m.r32(esp + 8)];
            self.zoom.skip = Some(f);
            let result = self.dispatch(m, f, ecx, &args);
            m.w32(ecx + 0x2C, saved[0]);
            m.w32(ecx + 0x30, saved[1]);
            return Some(result);
        }
        if self.zoom.enabled && f == 0x0059_8700 && ecx == m.r32(SCENE) {
            self.zoom.source_size = (0, 0, 0);
            self.zoom.allocated = (0, 0, 0);
        }
        // A scaled image cannot share the original incremental screen cache.
        // Redraw the scene (and overlapping dialogs) every frame. Invalidating
        // the whole screen would also repaint translucent UI over itself.
        if self.zoom.enabled
            && (self.zoom.percent != 100 || self.zoom.virtualized)
            && f == 0x004B_8A40
            && self.zoom_game_active(m)
        {
            // Solo con la ciudad a la vista (`+0x48`, lo que lee `0x4B2200`): con la carta náutica abierta la escena está oculta
            // y marcarla sucia ensuciaba toda la zona de la carta en cada fotograma.
            let scene = m.r32(SCENE);
            if scene != 0 && m.r8(scene + 0x48) != 0 {
                self.dispatch(m, 0x0044_1710, scene, &[]);
            }
        }
        if !self.zoom.enabled || ![DRAW, MOUSE].contains(&f) || ecx != m.r32(SCENE) {
            return None;
        }
        // Escena oculta (carta náutica abierta, avance rápido): el original sale sin dibujar (`0x5888B7`, `0x4B2200` lee `+0x48`).
        // La ruta del zoom pegaba igualmente su lienzo, con la última imagen de la ciudad, sobre la zona de la escena: asomaba la
        // ciudad al mover la carta y, donde la carta no se repintaba, quedaban restos (pájaros en avance rápido).
        if m.r8(ecx + 0x48) == 0 {
            return None;
        }
        let rect = self.zoom_rect(m)?;
        self.zoom.configure(rect);
        if self.zoom.percent == 100 && self.zoom.raw.is_empty() && !self.zoom.virtualized {
            return None;
        }
        // Keep the recursive dispatch below the original caller's stack.
        self.sp = self.sp.min(esp.wrapping_sub(0x100));
        if f == MOUSE {
            if self.zoom.virtualized {
                self.zoom_virtual_size(m, ecx);
            }
            let point = m.r32(esp + 4);
            let p = (m.r32(point) as i32, m.r32(point + 4) as i32);
            if !self.zoom.contains(p) {
                return None;
            }
            let q = self.zoom.source(p);
            m.w32(point, q.0 as u32);
            m.w32(point + 4, q.1 as u32);
            let args = [point, m.r32(esp + 8)];
            let saved = [
                m.r32(ecx + 0x2C),
                m.r32(ecx + 0x30),
                m.r32(super::display::RES_X),
                m.r32(super::display::RES_Y),
            ];
            if self.zoom.virtualized {
                m.w32(ecx + 0x2C, self.zoom.source_size.1 as u32);
                m.w32(ecx + 0x30, self.zoom.source_size.2 as u32);
                m.w32(super::display::RES_X, (rect[0] + self.zoom.source_size.1 + 284) as u32);
                m.w32(super::display::RES_Y, (rect[1] + self.zoom.source_size.2) as u32);
            }
            if self.zoom.virtualized {
                self.zoom.mouse_res = Some((ecx, saved[0], saved[1]));
            }
            self.zoom.skip = Some(f);
            let result = self.dispatch(m, f, ecx, &args);
            self.zoom.mouse_res = None;
            m.w32(point, p.0 as u32);
            m.w32(point + 4, p.1 as u32);
            m.w32(ecx + 0x2C, saved[0]);
            m.w32(ecx + 0x30, saved[1]);
            m.w32(super::display::RES_X, saved[2]);
            m.w32(super::display::RES_Y, saved[3]);
            return Some(result);
        }
        if self.zoom.virtualized {
            let args: Vec<u32> = (0..4).map(|i| m.r32(esp + 4 + i * 4)).collect();
            return Some(self.zoom_virtual_draw(m, ecx, &args));
        }
        self.zoom.restore(&mut self.surface);
        let args: Vec<u32> = (0..4).map(|i| m.r32(esp + 4 + i * 4)).collect();
        self.zoom.skip = Some(f);
        let result = self.dispatch(m, f, ecx, &args);
        self.zoom.magnify(&mut self.surface);
        self.zoom_mirror_scene();
        if self.zoom.percent == 100 {
            self.zoom.raw.clear();
        }
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wheel_accumulates_small_deltas_anchors_and_clamps() {
        let mut z = Zoom::default();
        z.configure([0, 42, 740, 768]);
        let p = (320, 300);
        z.wheel(60, p);
        assert_eq!(z.percent(), 100);
        z.wheel(60, p);
        assert_eq!(z.percent(), 125);
        assert_eq!(z.source(p), p);
        z.wheel(120 * 100, p);
        assert_eq!(z.percent(), 250);
        assert_eq!(z.source(p), p);
        z.wheel(-120 * 100, p);
        assert_eq!(z.percent(), 50);
        assert_eq!(z.origin, (0.0, 0.0));
        assert_eq!(z.source((739, 767)), (1478, 1492));
        z.wheel(240, p);
        assert_eq!(z.percent(), 100);
        assert_eq!(z.source((739, 767)), (739, 767));
        assert!(!z.contains((740, 300)));
        assert!(!z.contains((200, 41)));
        z.percent = 58; // A city-bound minimum between the normal wheel stops.
        z.wheel(120, p);
        assert_eq!(z.percent(), 75);
        z.percent = 58;
        z.wheel(240, p);
        assert_eq!(z.percent(), 100);
    }

    #[test]
    fn magnification_and_picking_agree_and_raw_frame_survives_repainting() {
        let mut s = super::super::Surface::new(16, 12, [9, 8, 7, 255]);
        for y in 2..10usize {
            for x in 1..11usize {
                let i = (y * s.width + x) * 4;
                s.rgba[i..i + 4].copy_from_slice(&[x as u8, y as u8, 0, 255]);
            }
        }
        let original = s.rgba.clone();
        let mut z = Zoom::default();
        z.configure([1, 2, 11, 10]);
        z.wheel(120 * 4, (5, 6));
        z.magnify(&mut s);
        assert_ne!(s.rgba, original);
        for y in 0..12 {
            for x in 0..16 {
                let p = (x as i32, y as i32);
                let q = if z.contains(p) { z.source(p) } else { p };
                let a = (y * 16 + x) * 4;
                let b = (q.1 as usize * 16 + q.0 as usize) * 4;
                assert_eq!(s.rgba[a..a + 4], original[b..b + 4]);
            }
        }
        let scaled = s.rgba.clone();
        for _ in 0..40 {
            z.restore(&mut s);
            z.magnify(&mut s);
        }
        assert_eq!(s.rgba, scaled, "the image must never magnify itself");
        z.wheel(-120 * 4, (5, 6));
        z.restore(&mut s);
        z.magnify(&mut s);
        assert_eq!(s.rgba, original);
    }
}
