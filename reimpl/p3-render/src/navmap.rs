//! Carta náutica desplazable (`CNavMap`, vtable `0x0066DA48`; «scrollmap» en P3Modding).
//!
//! Funciones portadas con paridad x86 (`oracle/x86_navmap.py`, fixtures `tests/fixtures/navmap_*.json`):
//!
//! - [`center_on_point`]: `0x0044F490` (`nav_map_center_on_point`) y su envoltorio `0x0044F640`. Centra la vista en un punto del
//!   mapa, la limita al tamaño de la textura de fondo (`+0xBC`, `images/Vollansichtskarte.bmp`) y fija el rectángulo de la vista
//!   en el minimapa (`0x0046EBF0`), con las escalas `9.23077` (`0x0066D7D8`) y `7.92952` (`0x0066D7E0`).
//! - [`scroll_frame`]: el bloque de desplazamiento de `nav_map_update` (`0x0044DCE0`, de `0x0044E251` a `0x0044E6DE` y su
//!   final `0x0044E75A`–`0x0044E888`). Las cuatro teclas (`0x006DCBE5..E8`) y los bordes de la pantalla mueven la vista
//!   `(reloj − último fotograma) / periodo · 2400` píxeles en x y `· 1800` en y, con un tope de 128 por fotograma; el periodo
//!   `+0x2918` vale 1500 ms (`0x00449931`). Después recorta al fondo y corrige el paso para el copiado (`0x004538E0`).
//! - [`visible_ships`]: `0x004516B0`. Lista de barcos dibujados en la carta: los propios en el mar (estados 0xF y 0x16; un
//!   convoy aporta su líder) y **los contactos de cada barco propio, `ship+0x50` y `ship+0x52`**. Esos dos campos son los
//!   avistamientos que escribe el tick naval (C10): el punto de enganche de la visibilidad es [`World::ships`].
//!   Ordena por `y` y pide el sonido de clase 4 (`0x00459790`) por cada barco cuyo recuadro cae en la vista.
//! - [`draw_ships_at_sea`]: `0x0044CBF0`. Para cada barco de la lista: marcador `+0xCC` teñido por bando, número de barcos del
//!   convoy, barra de reparación (`+0x136` bit 0) o de abordaje (`+0x15C`), sprite del casco (`+0xB4/+0xB0/+0xAC/+0xB8` por
//!   tipo 0..3, fotograma = rumbo `ship+1`) y, en una segunda pasada, la flecha `0x7557` sobre los barcos elegidos y la del
//!   barco bajo el cursor (`+0x2B14`, opacidad `+0x2B1C`).
//! - [`draw_weather_icons`]: `0x0044B970`. Anima (con `rand`) y dibuja las tormentas (subtipo 0, textura `+0x2AF8` y sombra
//!   `+0xD4` 30 píxeles más abajo con alfa 0x78) y las calmas (subtipo 1, `+0x2B00`) que [`p3_audio`] deja en `+0x2A68..+0x2ABA`
//!   (`0x00459380`).
//!
//! El objeto (`obj`) es la imagen de `CNavMap` con sus desplazamientos originales (la misma que `p3_audio::map_ambient::MapScene`).
//! Los valores locales que el original deja sin iniciar (rectángulos de fotograma cuando el índice cae fuera de la textura) se
//! conservan entre barcos igual que en la pila del x86; empiezan a cero.

/// Tamaño de la imagen del objeto que cubre los campos usados (igual que `MapScene`).
pub const OBJ_SIZE: usize = 0x2D20;
/// Escalas del minimapa (`0x0066D7D8`, `0x0066D7E0`).
pub const MINIMAP_SX: f64 = f64::from_bits(0x4022_7627_7C45_CBBC);
pub const MINIMAP_SY: f64 = f64::from_bits(0x401F_B7D4_1743_E964);
/// Identificador de la flecha de selección (`0x0044CC22`).
pub const ARROW_TEXTURE: u32 = 0x7557;

pub(crate) fn half(n: i32) -> i32 {
    n.wrapping_sub(n >> 31) >> 1
}

/// `__ftol`: `fistp qword` truncando; fuera de rango da el entero indefinido, cuya mitad baja es 0.
pub fn ftol(v: f64) -> i32 {
    if v.is_finite() && v > -9.223_372_036_854_775e18 && v < 9.223_372_036_854_775e18 {
        v.trunc() as i64 as i32
    } else {
        0
    }
}

pub fn r8(m: &[u8], o: usize) -> u8 {
    m.get(o).copied().unwrap_or(0)
}
pub fn r16(m: &[u8], o: usize) -> u16 {
    u16::from_le_bytes([r8(m, o), r8(m, o + 1)])
}
pub fn r32(m: &[u8], o: usize) -> u32 {
    u32::from_le_bytes([r8(m, o), r8(m, o + 1), r8(m, o + 2), r8(m, o + 3)])
}
fn i16_at(m: &[u8], o: usize) -> i32 {
    i32::from(r16(m, o) as i16)
}
fn i32_at(m: &[u8], o: usize) -> i32 {
    r32(m, o) as i32
}
pub fn w8(m: &mut [u8], o: usize, v: u8) {
    if let Some(b) = m.get_mut(o) {
        *b = v
    }
}
pub fn w16(m: &mut [u8], o: usize, v: u16) {
    for (k, b) in v.to_le_bytes().into_iter().enumerate() {
        w8(m, o + k, b)
    }
}
pub fn w32(m: &mut [u8], o: usize, v: u32) {
    for (k, b) in v.to_le_bytes().into_iter().enumerate() {
        w8(m, o + k, b)
    }
}

/// Textura del motor (`+4` manejador, `+0xC` rectángulos `x y ancho alto`, `+0x14` número de fotogramas).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Tex {
    pub handle: u32,
    pub frames: Vec<[i32; 4]>,
}
impl Tex {
    /// `0x0045C080`: el fotograma solo existe con índice menor que el número y manejador no nulo.
    pub fn frame(&self, i: i32) -> Option<(u32, [i32; 4])> {
        if i < self.frames.len() as i32 && self.handle != 0 {
            self.frames.get(i as usize).map(|r| (self.handle, *r))
        } else {
            None
        }
    }
}

/// Texturas de la carta: las cuatro de casco por desplazamiento (`+0xAC` Kogge, `+0xB0` Kraier, `+0xB4` Schnigge, `+0xB8` Holk),
/// el marcador `+0xCC`, la sombra de tormenta `+0xD4` y las que se buscan por identificador (`0x004B3DD0` sobre `0x006DA820`).
#[derive(Clone, Debug, Default)]
pub struct Textures {
    pub kogge: Tex,
    pub kraier: Tex,
    pub schnigge: Tex,
    pub holk: Tex,
    pub marker: Tex,
    pub storm_shadow: Tex,
    /// Fondo `+0xBC` y lupa `+0xC0` (solo los usa [`crate::navmap_draw::draw_nav_map`]).
    pub background: Tex,
    pub lens: Tex,
    pub by_id: Vec<(u32, Tex)>,
}
impl Textures {
    /// Textura de casco por tipo (`ship+0xE`): 0 → `+0xB4`, 1 → `+0xB0`, 2 → `+0xAC`, 3 → `+0xB8` (tablas `0x0044D318` y siguientes).
    pub fn hull(&self, kind: u8) -> Option<&Tex> {
        match kind {
            0 => Some(&self.schnigge),
            1 => Some(&self.kraier),
            2 => Some(&self.kogge),
            3 => Some(&self.holk),
            _ => None,
        }
    }
    pub fn lookup(&self, id: u32) -> Tex {
        self.by_id.iter().find(|(k, _)| *k == id).map(|(_, t)| t.clone()).unwrap_or_default()
    }
}

/// Llamadas del original a la superficie, al minimapa, al sonido y a los comerciantes.
pub trait NavHost {
    /// `0x004BB870(argb)`: color de modulación del siguiente dibujo.
    fn color(&mut self, _argb: u32) {}
    /// `0x004BB9C0(manejador)`: textura activa.
    fn select(&mut self, _handle: u32) {}
    /// `0x004BB330(sx, sy, dx, dy, ancho, alto)`: copia de la textura activa.
    fn blit(&mut self, _sx: i32, _sy: i32, _dx: i32, _dy: i32, _w: i32, _h: i32) {}
    /// `0x004BB430(x, y, ancho, alto)`: rectángulo lleno del color activo.
    fn fill(&mut self, _x: i32, _y: i32, _w: i32, _h: i32) {}
    /// `0x004BB3E0(x, y, texto)`.
    fn text(&mut self, _x: i32, _y: i32, _s: &str) {}
    /// `0x004BBB20(manejador, &tamaño)`.
    fn size(&mut self, handle: u32) -> (i32, i32);
    /// `sgl_GetTextureInfo` completo (tres dobles palabras): ancho, alto y la tercera, que `0x004530B0` pasa a
    /// `sgl_SelectTexture`. **Supuesto**: la tercera es el manejador (el contrato está en la DLL de SGL, no estudiada).
    fn texture_info(&mut self, handle: u32) -> [i32; 3] {
        let (w, h) = self.size(handle);
        [w, h, handle as i32]
    }
    /// `0x0046EBF0(x1, y1, x2, y2)` sobre el minimapa `0x006CC940`.
    fn minimap_view(&mut self, _r: [i32; 4]) {}
    /// `0x004B9650(&rect)` sobre la pantalla `0x006DA5F0`.
    fn repaint(&mut self, _r: [i32; 4]) {}
    /// `0x00453E30(&rect)`: rectángulo sucio de la carta.
    fn dirty(&mut self, _r: [i32; 4]) {}
    /// `0x00459790(clase, id, x, y, extra)`.
    fn ambient(&mut self, _class: u32, _id: u32, _x: i32, _y: i32, _extra: u32) {}
    /// `merchant+8` (u16) del comerciante (`0x005303C0`): distinto de cero en los comerciantes humanos.
    fn merchant_word8(&mut self, _merchant: u8) -> u16 {
        0
    }
    /// `rand` del CRT (`0x00639C03`).
    fn rand(&mut self) -> u32 {
        0
    }
    /// `0x004B3DD0(id)`: búsqueda de textura por identificador (el resultado lo da [`Textures::lookup`]).
    fn lookup(&mut self, _id: u32) {}
    /// `0x005303C0(comerciante)` en `0x004516B0` (el resultado lo da [`World::first_ship`]).
    fn get_merchant(&mut self, _merchant: u32) {}
    /// `sgl_SetFont(0x004BB8F0)` con el manejador de la fuente (identificada por la dirección de su objeto).
    fn font(&mut self, _font: u32) {}
    /// `sgl_SetTextMode(0x004BBA10)`: 1 izquierda, 2 derecha, 3 centrado.
    fn text_mode(&mut self, _mode: i32) {}
    /// `0x004BB390(x1, y1, x2, y2)`: línea en coma flotante (la diagonal de `draw_geometry` `0x004BD680`).
    fn line(&mut self, _x1: f32, _y1: f32, _x2: f32, _y2: f32) {}
    /// `0x004C1B20(texto)` sobre la fuente `font`: ancho en píxeles del texto.
    fn text_width(&mut self, _font: u32, _s: &str) -> i32 {
        0
    }
    /// `0x005308A0(comerciante, ciudad, 0)` sobre `0x006DE4A0`: ¿tiene el comerciante oficina en la ciudad? (el original la
    /// busca sin crearla con el tercer argumento a cero). El comerciante llega en 16 bits: la mitad alta de `ECX` es basura.
    fn office(&mut self, _merchant: u32, _town: u32) -> bool {
        false
    }
    /// `sgl_SetColor` devolviendo el color anterior (el resto de funciones ignora el valor).
    fn set_color(&mut self, argb: u32) -> u32 {
        self.color(argb);
        0xFFFF_FFFF
    }
    /// `sgl_SelectTexture` devolviendo la textura anterior.
    fn set_texture(&mut self, handle: u32) -> u32 {
        self.select(handle);
        0
    }
    /// Pieza de [`crate::navmap_draw::draw_nav_map`] (cada una tiene su función portada; el anfitrión decide con qué datos).
    fn part(&mut self, _part: crate::navmap_draw::NavPart, _clip: [i32; 4]) {}
    /// `window_is_visible(0x004B2200)` de la carta.
    fn window_shown(&mut self) -> bool {
        true
    }
    /// `sgl_IsRectInClippingRegion(0x004BB7C0)`.
    fn clip_visible(&mut self, _r: [i32; 4]) -> bool {
        true
    }
    /// `ddraw_get_rect_clipper(0x004BB520)`.
    fn get_clipper(&mut self) -> u32 {
        0
    }
    /// `sgl_SetActiveClipper(0x004BB780)`; devuelve el anterior.
    fn set_clipper(&mut self, _c: u32) -> u32 {
        0
    }
    /// `ddraw_copy_clipper(0x004BB280)`.
    fn copy_clipper(&mut self, _c: u32) {}
    /// `popup_list_close_all(0x004621F0)`.
    fn close_popups(&mut self) {}
    /// `sgl_SubClipRect_4i(0x004BBAA0)`.
    fn sub_clip(&mut self, _x: i32, _y: i32, _w: i32, _h: i32) {}
    /// `sgl_SetRenderDest(0x004BB9B0)`; devuelve el destino anterior.
    fn set_dest(&mut self, _d: u32) -> u32 {
        0
    }
    /// `ddraw_free_rect_clipper(0x004BB4C0)`.
    fn free_clipper(&mut self, _c: u32) {}
    /// Posición del cursor de la ventana superior (`0x004B9730` y `0x004B77F0`).
    fn cursor(&mut self) -> (i32, i32) {
        (0, 0)
    }
    /// `animation2d_is_timer_running(0x004AD900)` del control incrustado en `obj+off`.
    fn widget_running(&mut self, _off: u32) -> bool {
        false
    }
    /// Dibujo del control `obj+off` (`vtable+0x9C(destino, x, y, z)`).
    fn widget_draw(&mut self, _off: u32, _dest: u32, _args: [u32; 3]) {}
    /// `sgl_Blt(0x004BB350)` de pantalla a pantalla; distinto de cero si falla.
    fn screen_blt(&mut self, _sx: i32, _sy: i32, _dx: i32, _dy: i32, _w: i32, _h: i32, _flags: i32) -> i32 {
        0
    }
    /// `add_dirty_rect_to_clippers(0x004B95E0)` de la lista de pantallas.
    fn add_dirty(&mut self, _r: [i32; 4]) {}
    /// Cursor de la ventana del juego (`0x004B77F0` sobre `DAT_006CBB40`).
    fn game_cursor(&mut self) -> (i32, i32) {
        (0, 0)
    }
    /// Tamaño del cursor actual (`0x004B9540`, `vtable+0x44`/`+0x48`).
    fn cursor_size(&mut self) -> (i32, i32) {
        (0, 0)
    }
    /// Última posición del cursor (`0x004B9430`).
    fn last_cursor(&mut self) -> (i32, i32) {
        (0, 0)
    }
    /// Ventana flotante `DAT_006EE120` (`x, y, ancho, alto` por `vtable+0x6C/+0x70/+0x44/+0x48`), si existe.
    fn tooltip(&mut self) -> Option<[i32; 4]> {
        None
    }
    /// Las dos listas de objetos superpuestos de la ventana del juego (`+0x168`, recuento `+0x488`; `+0x494`, recuento
    /// `+0x7B4`; 0xA0 bytes cada uno), como `x, y, ancho, alto`.
    fn overlays(&mut self) -> (Vec<[i32; 4]>, Vec<[i32; 4]>) {
        (Vec::new(), Vec::new())
    }
    /// Rectángulo que da `0x004B85C0` cuando `0x004B8510` es cierto (ventana del juego).
    fn drag_rect(&mut self) -> Option<[i32; 4]> {
        None
    }
    /// Posición y tamaño del control `obj+off` (`vtable+0x6C`, `+0x70`, `+0x44`, `+0x48`): `x, y, ancho, alto`.
    fn widget_geom(&mut self, _off: u32) -> [i32; 4] {
        [0; 4]
    }
    /// `vtable+0x64(x, y, z)` del control `obj+off`: lo coloca en pantalla.
    fn widget_move(&mut self, _off: u32, _x: i32, _y: i32, _z: i32) {}
    /// `animation2d_set_timer_state(0x004AD8D0, a, b)` del control `obj+off`.
    fn widget_timer(&mut self, _off: u32, _a: u32, _b: u32) {}
    /// `route_planner_is_point_navigable(0x004474D0)` sobre `DAT_006CBDC8` con el punto en 16 bits.
    fn navigable(&mut self, _x: i16, _y: i16) -> bool {
        false
    }
    /// `sgl_SetBlendMode(0x004BB7A0)`; devuelve el modo anterior.
    fn blend_mode(&mut self, _mode: i32) -> i32 {
        0
    }
    /// Llamadas de `nav_map_update` (`0x0044DCE0`) a funciones de la carta sin resultado (ver [`crate::navmap_update::UpdateStep`]).
    fn update_step(&mut self, _step: crate::navmap_update::UpdateStep) {}
    /// `0x0064F7E2` (`operator delete`) de memoria de la carta.
    fn free(&mut self, _ptr: u32) {}
    /// `0x004BBB00(manejador, &píxeles, 1)`: bloquea la textura para leer sus píxeles.
    fn lock_texture(&mut self, _handle: u32) {}
    /// Píxel `i` (doble palabra) de la textura bloqueada; el original lo lee del puntero que devuelve `0x004BBB00`.
    fn texture_pixel(&mut self, _handle: u32, _i: i32) -> u32 {
        0
    }
    /// `0x004BBB10(manejador)`.
    fn unlock_texture(&mut self, _handle: u32) {}
    /// `0x00535300(i)` sobre `0x006DE588`: banderas de noticias conocidas de la ciudad en la posición `i`.
    fn town_flags(&mut self, _i: u32) -> u32 {
        0
    }
    /// `0x004528D0(&{x, y, fotograma}, bit, i)` (`scroll_map_hover_town_status_icon`, sin portar: elige el texto de ayuda
    /// `+0x2904` si el cursor está sobre el icono). Devuelve si lo está.
    fn hover_status_icon(&mut self, _icon: [i32; 6], _bit: u32, _i: u32) -> bool {
        false
    }
    /// Sin ningún icono bajo el cursor y con texto de ayuda: `+0x2904 = ""` (`0x0064F390`) y `0x004B8380(0)` (oculta la ayuda).
    fn clear_tooltip(&mut self) {}
    /// Con algún icono bajo el cursor: método virtual `+0x114(1)` de la carta (muestra la ayuda).
    fn show_tooltip(&mut self) {}
    /// Texto de ayuda `+0x2904` con los días de viaje: `1` asigna el texto en singular (`0x0069B6C8`); otro valor, `Format` con
    /// el de `0x0069B6C4`.
    fn tooltip_days(&mut self, _days: i32) {}
    /// `RoutePlanner::find_route` (`0x00445010`, portado en `p3_core::route::finder`) para los puntos dados: si encontró ruta y
    /// el valor que deja en su salida (ruta del motor).
    fn find_route(&mut self, _points: &[(i16, i16)]) -> (bool, u32) {
        (false, 0)
    }
    /// `ship::route_duration` (`0x005169C0`, portado en `p3_core::ships::route`) del barco con la ruta que dejó [`Self::find_route`].
    fn route_ticks(&mut self, _ship: u32, _route: u32) -> i32 {
        0
    }
    /// Libera la ruta de [`Self::find_route`] (sus puntos y el objeto, `0x0064F7E2`).
    fn free_route(&mut self, _route: u32) {}
    /// `0x00446A00` (`route_point_hit_test_polyline`; portado como [`crate::navmap_update::route_point_hit`]): los nueve
    /// argumentos de pila tal cual.
    /// `enqueue_operation(0x0054AA70)` sobre `DAT_006DF2F0` con un registro de operación de cinco enteros (tipo, barco, …).
    fn enqueue_operation(&mut self, _op: [i32; 5]) {}
    /// Vtable `+0xC8` del objeto `DAT_006CBB10`: ¿hay ya un diálogo de problemas de ruta abierto?
    fn problem_dialog_open(&mut self) -> bool {
        false
    }
    /// `0x00456F80`: abre el diálogo de problemas de ruta.
    fn open_problem_dialog(&mut self) {}
    /// `0x00454F80(a, b)` (sin portar aquí): refresco de la lista de problemas; `0x0045A6B0` lo llama con `(0xFF, 0)`.
    fn refresh_problems(&mut self, _a: u32, _b: u32) {}
    fn route_hit(&mut self, _args: [i32; 9]) -> bool {
        false
    }
}

/// Rectángulo de la vista en el minimapa (`0x0044F54F`–`0x0044F5F7`, igual en `0x0044E7DD`–`0x0044E888`).
pub fn minimap_rect(obj: &[u8]) -> [i32; 4] {
    let y1 = ftol(f64::from(i16_at(obj, 0x29E6)) / MINIMAP_SY);
    let x1 = ftol(f64::from(i16_at(obj, 0x29E4)) / MINIMAP_SX);
    let y2 = ftol(f64::from(i32_at(obj, 0x30)) / MINIMAP_SY + f64::from(y1 as i16));
    let x2 = ftol(f64::from(i32_at(obj, 0x2C)) / MINIMAP_SX + f64::from(x1 as i16));
    [x1, y1, x2, y2]
}

pub(crate) fn view_rect(obj: &[u8]) -> [i32; 4] {
    let (x, y) = (i32_at(obj, 0x14), i32_at(obj, 0x18));
    [x, y, x.wrapping_add(i32_at(obj, 0x2C)), y.wrapping_add(i32_at(obj, 0x30))]
}

/// `0x0044F490(x, y)`. `background` es el manejador de la textura `+0xBC` (`None` si el puntero es nulo: solo guarda el
/// desplazamiento); su tamaño se pide a [`NavHost::size`].
pub fn center_on_point(obj: &mut [u8], x: i32, y: i32, background: Option<u32>, host: &mut dyn NavHost) {
    let w = i32_at(obj, 0x2C);
    let h = i32_at(obj, 0x30);
    w16(obj, 0x29E4, x.wrapping_sub(half(w)) as u16);
    w16(obj, 0x29E6, y.wrapping_sub(half(h)) as u16);
    let Some(bg) = background else { return };
    let (tw, th) = host.size(bg);
    if i16_at(obj, 0x29E4) > tw.wrapping_sub(w) {
        w16(obj, 0x29E4, (tw as u16).wrapping_sub(w as u16));
    }
    if i16_at(obj, 0x29E6) > th.wrapping_sub(h) {
        w16(obj, 0x29E6, (th as u16).wrapping_sub(h as u16));
    }
    if i16_at(obj, 0x29E4) < 0 {
        w16(obj, 0x29E4, 0);
    }
    if i16_at(obj, 0x29E6) < 0 {
        w16(obj, 0x29E6, 0);
    }
    host.minimap_view(minimap_rect(obj));
    host.repaint(view_rect(obj));
    w8(obj, 0x2B26, 0);
}

/// `0x0044F640`: vuelve a centrar en el centro actual (recorta tras cambiar el tamaño de la vista).
pub fn clamp_scroll(obj: &mut [u8], background: Option<u32>, host: &mut dyn NavHost) {
    let cy = (half(i32_at(obj, 0x30)) as u32 & 0xFFFF_0000) | u32::from((half(i32_at(obj, 0x30)) as u16).wrapping_add(r16(obj, 0x29E6)));
    let cx = (half(i32_at(obj, 0x2C)) as u32 & 0xFFFF_0000) | u32::from((half(i32_at(obj, 0x2C)) as u16).wrapping_add(r16(obj, 0x29E4)));
    center_on_point(obj, cx as i32, cy as i32, background, host);
}

/// Entradas de un fotograma de `nav_map_update`.
#[derive(Clone, Copy, Debug, Default)]
pub struct ScrollInput {
    /// Ventana sin diálogo modal y minimapa sin arrastre (`0x0044DEB8`); si no, las teclas solo se consumen.
    pub active: bool,
    /// Ratón en pantalla (`0x004B9730`).
    pub mouse: (i32, i32),
    /// Resolución (`0x006DCCE0`, `0x006DCCE4`).
    pub screen: (i32, i32),
    /// `DAT_006DCCF0` (ms).
    pub clock: u32,
    /// Teclas `0x006DCBE5` (izquierda), `E6` (arriba), `E7` (derecha), `E8` (abajo). Cada fotograma el original las rehace como
    /// `((v >> 1) & 0x7E) | (v & 0xFE)` y desplaza si valían algo; el bit 0 (lo pone el teclado) se consume.
    pub keys: [u8; 4],
    /// Manejador de la textura de fondo (`+0xBC`; el original no comprueba el puntero).
    pub background: u32,
}

/// Resultado del desplazamiento: paso aplicado y si se movió en cada eje (`[esp+0x13]`, `[esp+0x12]`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ScrollOut {
    pub right: i32,
    pub left: i32,
    pub up: i32,
    pub down: i32,
    pub moved_x: bool,
    pub moved_y: bool,
}
impl ScrollOut {
    /// `(dx, dy)` que se pasan a `0x004538E0`.
    pub fn delta(&self) -> (i32, i32) {
        (self.right.wrapping_sub(self.left), self.down.wrapping_sub(self.up))
    }
}

fn decay(k: &mut u8) -> bool {
    let v = *k;
    *k = ((v >> 1) & 0x7E) | (v & 0xFE);
    v != 0
}

fn step(obj: &[u8], clock: u32, k: f32) -> i32 {
    let diff = f64::from(clock.wrapping_sub(r32(obj, 0x290C))) as f32;
    let period = f64::from(i32_at(obj, 0x2918)) as f32;
    let v = ftol(f64::from(diff) / f64::from(period) * f64::from(k));
    v.min(0x80)
}

/// Bloque de desplazamiento de `0x0044DCE0` (sin el copiado ni los repintados, que hace [`finish_scroll`]).
pub fn scroll_frame(obj: &mut [u8], input: &mut ScrollInput, host: &mut dyn NavHost) -> ScrollOut {
    let mut o = ScrollOut::default();
    if !input.active {
        for k in input.keys.iter_mut() {
            decay(k);
        }
        return o;
    }
    let add = |obj: &mut [u8], at: usize, d: i32| {
        let v = r16(obj, at).wrapping_add(d as u16);
        w16(obj, at, v)
    };
    let [left, up, right, down] = &mut input.keys;
    if decay(right) {
        o.right = step(obj, input.clock, 2400.0);
        add(obj, 0x29E4, o.right);
        o.moved_x = true;
    }
    if decay(left) {
        o.left = step(obj, input.clock, 2400.0);
        add(obj, 0x29E4, o.left.wrapping_neg());
        o.moved_x = true;
    }
    if decay(up) {
        o.up = step(obj, input.clock, 1800.0);
        add(obj, 0x29E6, o.up.wrapping_neg());
        o.moved_y = true;
    }
    if decay(down) {
        o.down = step(obj, input.clock, 1800.0);
        add(obj, 0x29E6, o.down);
        o.moved_y = true;
    }
    if !o.moved_x && !o.moved_y {
        let (mx, my) = input.mouse;
        if mx >= input.screen.0.wrapping_sub(1) {
            o.right = step(obj, input.clock, 2400.0);
            o.moved_x = true;
            add(obj, 0x29E4, o.right);
        }
        if mx <= 1 {
            o.left = step(obj, input.clock, 2400.0);
            add(obj, 0x29E4, o.left.wrapping_neg());
            o.moved_x = true;
        }
        if my <= 1 {
            o.up = step(obj, input.clock, 1800.0);
            o.moved_y = true;
            add(obj, 0x29E6, o.up.wrapping_neg());
        }
        if my >= input.screen.1.wrapping_sub(1) {
            o.down = step(obj, input.clock, 1800.0);
            add(obj, 0x29E6, o.down);
            o.moved_y = true;
        }
    }
    let (tw, th) = host.size(input.background);
    let w = i32_at(obj, 0x2C);
    let sx = i16_at(obj, 0x29E4);
    if sx > tw.wrapping_sub(w) {
        let over = sx.wrapping_sub(tw).wrapping_add(w);
        if over == o.right {
            o.moved_x = false;
            o.right = 0;
        } else {
            o.right = o.right.wrapping_add(tw.wrapping_sub(sx).wrapping_sub(w));
        }
        w16(obj, 0x29E4, (tw as u16).wrapping_sub(w as u16));
    }
    let h = i32_at(obj, 0x30);
    let sy = i16_at(obj, 0x29E6);
    if sy > th.wrapping_sub(h) {
        let over = sy.wrapping_sub(th).wrapping_add(h);
        if over == o.down {
            o.moved_y = false;
            o.down = 0;
        } else {
            o.down = o.down.wrapping_add(th.wrapping_sub(sy).wrapping_sub(h));
        }
        w16(obj, 0x29E6, (th as u16).wrapping_sub(h as u16));
    }
    let sx = i16_at(obj, 0x29E4);
    if sx < 0 {
        if sx == o.left.wrapping_neg() {
            o.moved_x = false;
            o.left = 0;
        } else {
            o.left = o.left.wrapping_add(sx);
        }
        w16(obj, 0x29E4, 0);
    }
    let sy = i16_at(obj, 0x29E6);
    if sy < 0 {
        if sy == o.up.wrapping_neg() {
            o.moved_y = false;
            o.up = 0;
        } else {
            o.up = o.up.wrapping_add(sy);
        }
        w16(obj, 0x29E6, 0);
    }
    o
}

/// Final del desplazamiento (`0x0044E75A`–`0x0044E888`): repinta la vista, marca la recolocación de sonidos (`+0x2B28`, solo si
/// cambia `x`: el original prueba `abs(dx)`), `+0x2B26 = 1` y mueve el rectángulo del minimapa. Devuelve si hubo movimiento.
pub fn finish_scroll(obj: &mut [u8], out: &ScrollOut, host: &mut dyn NavHost) -> bool {
    if !(out.moved_x || out.moved_y) {
        return false;
    }
    host.repaint(view_rect(obj));
    if out.delta().0 != 0 {
        w8(obj, 0x2B28, 1);
    }
    w8(obj, 0x2B26, 1);
    host.minimap_view(minimap_rect(obj));
    true
}

/// Mundo naval que lee la carta: tabla de barcos (`0x006DD7A4`, 0x180 bytes), convoyes (`0x006DD7A8`, 0x3C bytes) y comerciantes.
pub struct World<'a> {
    /// Imágenes de barco consecutivas. **Avistamientos**: `+0x50` y `+0x52` de cada barco propio (contactos que escribe el tick
    /// naval de C10); con `0xFFFF` el barco no ve a nadie.
    pub ships: &'a [u8],
    /// `DAT_006DD894` y `DAT_006DD896`.
    pub ship_count: u16,
    pub convoys: &'a [u8],
    pub convoy_count: u16,
    /// `DAT_006DE4AA` y `DAT_006DFC14`.
    pub merchant_count: u16,
    pub player: u32,
    /// `merchant+0xE` del jugador: primer barco de su cadena (`ship+4` es el siguiente).
    pub first_ship: u16,
}
impl World<'_> {
    pub fn ship(&self, i: u32) -> &[u8] {
        let at = (i as usize).saturating_mul(0x180);
        self.ships.get(at..).unwrap_or(&[])
    }
    pub fn convoy(&self, i: u32) -> &[u8] {
        let at = (i as usize).saturating_mul(0x3C);
        self.convoys.get(at..).unwrap_or(&[])
    }
}

fn rect_of(t: Option<&Tex>, i: i32, r: &mut [i32; 4], handle: &mut u32) {
    if let Some((h, f)) = t.and_then(|t| t.frame(i)) {
        *r = f;
        *handle = h;
    }
}

/// `IntersectRect`.
fn intersect(a: &[i32; 4], b: &[i32; 4]) -> Option<[i32; 4]> {
    let r = [a[0].max(b[0]), a[1].max(b[1]), a[2].min(b[2]), a[3].min(b[3])];
    let empty = |r: &[i32; 4]| r[2] <= r[0] || r[3] <= r[1];
    if empty(a) || empty(b) || empty(&r) {
        None
    } else {
        Some(r)
    }
}

/// `0x004516B0`. `shown` es `0x004B2200` (ventana visible), que el original consulta dos veces. Devuelve la lista nueva
/// (`+0x28A4`, recuento `+0x2908`).
pub fn visible_ships(obj: &mut [u8], world: &World, tex: &Textures, shown: (bool, bool), host: &mut dyn NavHost) -> Vec<u32> {
    let mut rect = [0i32; 4];
    let mut handle = 0u32;
    rect_of(Some(&tex.schnigge), 0, &mut rect, &mut handle);
    let (vx, vy) = (i32_at(obj, 0x14), i32_at(obj, 0x18));
    let (vw, vh) = (i32_at(obj, 0x2C), i32_at(obj, 0x30));
    let (sx, sy) = (i16_at(obj, 0x29E4), i16_at(obj, 0x29E6));
    let n = usize::from(world.ship_count);
    let (mut ys, mut ids) = (vec![0i32; n], vec![0u32; n]);
    let mut count = 0usize;
    let push = |ys: &mut Vec<i32>, ids: &mut Vec<u32>, count: usize, y: i32, id: u32| {
        if count < ys.len() {
            ys[count] = y;
            ids[count] = id;
        } else {
            ys.push(y);
            ids.push(id);
        }
    };
    let inside = |x: i32, y: i32, r: &[i32; 4]| {
        x.wrapping_sub(half(r[2])).wrapping_sub(sx) < vx.wrapping_add(vw)
            && y.wrapping_sub(half(r[3])).wrapping_sub(sy) < vy.wrapping_add(vh)
            && half(r[3]).wrapping_sub(sy).wrapping_add(vy).wrapping_add(y) > vy
            && half(r[2]).wrapping_sub(sx).wrapping_add(x) > vx
    };
    if u32::from(world.merchant_count) > world.player {
        host.get_merchant(world.player);
        let mut idx = u32::from(world.first_ship);
        while idx < u32::from(world.ship_count) {
            let s = world.ship(idx);
            let state = r16(s, 0x134);
            if state == 0xF || state == 0x16 {
                for c in [r16(s, 0x50), r16(s, 0x52)] {
                    if c == 0xFFFF {
                        continue;
                    }
                    let t = world.ship(u32::from(c));
                    let st = r16(t, 0x134);
                    if st != 0x16 && st != 0x12 && st != 0xF {
                        continue;
                    }
                    let (x, y) = (i16_at(t, 0x1E), i16_at(t, 0x22));
                    if inside(x, y, &rect) {
                        if shown.0 {
                            push(&mut ys, &mut ids, count, y, u32::from(c));
                        }
                        count += 1;
                    }
                }
                if shown.0 {
                    let conv = r16(s, 8);
                    let lead = if conv < world.convoy_count { Some(u32::from(r16(world.convoy(u32::from(conv)), 0x10))) } else { None };
                    let (id, x, y) = match lead {
                        Some(l) => {
                            let t = world.ship(l);
                            (l, i16_at(t, 0x1E), i16_at(t, 0x22))
                        }
                        None => (idx, i16_at(s, 0x1E), i16_at(s, 0x22)),
                    };
                    if inside(x, y, &rect) {
                        push(&mut ys, &mut ids, count, y, id);
                        count += 1;
                    }
                }
            }
            idx = u32::from(r16(s, 4));
        }
    }
    w32(obj, 0x2908, 0);
    let mut list: Vec<u32> = Vec::new();
    let view = view_rect(obj);
    if count != 0 && shown.1 {
        for k in 0..count {
            let id = ids[k];
            let t = world.ship(id);
            rect_of(tex.hull(r8(t, 0xE)), i32::from(r8(t, 1)), &mut rect, &mut handle);
            let left = i16_at(t, 0x1E).wrapping_sub(half(rect[2])).wrapping_sub(sx).wrapping_sub(5);
            let top = i16_at(t, 0x22).wrapping_sub(half(rect[3])).wrapping_sub(sy).wrapping_add(vy).wrapping_sub(5);
            let r = [left, top, left.wrapping_add(rect[2]).wrapping_add(10), top.wrapping_add(rect[3])];
            if let Some(r) = intersect(&r, &view) {
                host.repaint(r);
                host.dirty(r);
                // `mov cx, [ebx]`: solo la mitad baja es el barco; la alta es la de la dirección del rectángulo local (pila).
                host.ambient(4, id & 0xFFFF, r[0], r[1], 0);
            }
            let y = ys[k];
            let pos = list.iter().position(|&o| i16_at(world.ship(o), 0x22) > y).unwrap_or(list.len());
            list.insert(pos, id);
        }
    }
    w32(obj, 0x2908, list.len() as u32);
    list
}

fn side_colour(world: &World, s: &[u8], host: &mut dyn NavHost) -> (u32, u32, bool) {
    let state = r16(s, 0x134);
    let owner = r8(s, 0);
    let neutral = state == 0x12
        || u16::from(owner) >= world.merchant_count
        || (r8(s, 0x15C) & 1 != 0 && u32::from(owner) != world.player);
    if neutral {
        (0xFF00_0000, 0xFFFF_FFFF, false)
    } else if state == 0x16 {
        (0xFFFF_FF00, 0xFF00_0000, false)
    } else if u32::from(owner) != world.player {
        if host.merchant_word8(owner) != 0 {
            (0xFFFF_FFFF, 0xFF00_0000, false)
        } else {
            (0xFFFF_0000, 0xFFFF_FFFF, false)
        }
    } else {
        (0xFF00_00FF, 0xFFFF_FFFF, true)
    }
}

/// `0x0044CBF0`. `list` es la lista de [`visible_ships`]; `selection` la de `0x004885A0` (barcos elegidos por el jugador).
pub fn draw_ships_at_sea(obj: &[u8], world: &World, tex: &Textures, list: &[u32], selection: &[u16], host: &mut dyn NavHost) {
    host.lookup(ARROW_TEXTURE);
    let arrow = tex.lookup(ARROW_TEXTURE);
    let count = i32_at(obj, 0x2908);
    if count == 0 {
        return;
    }
    let (vx, vy) = (i32_at(obj, 0x14), i32_at(obj, 0x18));
    let (vw, vh) = (i32_at(obj, 0x2C), i32_at(obj, 0x30));
    let (sx, sy) = (i16_at(obj, 0x29E4), i16_at(obj, 0x29E6));
    let mut rect = [0i32; 4];
    let mut handle = 0u32;
    let entry = |k: i32| list.get(k as usize).copied().unwrap_or(0);
    for k in 0..count.max(0) {
        let id = entry(k);
        let s = world.ship(id);
        let kind = r8(s, 0xE);
        let heading = i32::from(r8(s, 1));
        let x = i16_at(s, 0x1E).wrapping_sub(sx).wrapping_add(vx);
        let y = i16_at(s, 0x22).wrapping_sub(sy).wrapping_add(vy);
        rect_of(tex.hull(kind), heading, &mut rect, &mut handle);
        let x0 = x.wrapping_sub(half(rect[2]));
        let y0 = y.wrapping_sub(half(rect[3]));
        let (mw, mh) = host.size(tex.marker.handle);
        host.select(tex.marker.handle);
        let (colour, text_colour, own) = side_colour(world, s, host);
        host.color(colour);
        let mx = if own { x0 } else { rect[2].wrapping_sub(mw).wrapping_add(x0) };
        host.blit(0, 0, mx, y0, mw, mh);
        let conv = r16(s, 8);
        if conv < world.convoy_count {
            let mut n: u8 = 0;
            let mut c = r16(world.convoy(u32::from(conv)), 0xA);
            while c < world.ship_count {
                n = n.wrapping_add(1);
                c = r16(world.ship(u32::from(c)), 6);
            }
            host.color(text_colour);
            host.text(mx.wrapping_add(7), y0.wrapping_add(3), &n.to_string());
        }
        if u32::from(r8(s, 0)) == world.player {
            let bar = if r8(s, 0x136) & 1 != 0 {
                Some(0xFFC8_C800)
            } else if r8(s, 0x15C) != 0 {
                Some(0xFF00_0000)
            } else {
                None
            };
            if let Some(c) = bar {
                host.color(c);
                host.fill(mx.wrapping_add(3), y0.wrapping_add(3), mw.wrapping_sub(6), 2);
            }
        }
        host.color(0xFFFF_FFFF);
        rect_of(tex.hull(kind), heading, &mut rect, &mut handle);
        host.select(handle);
        if y0 < vy.wrapping_add(vh) && x0 < vx.wrapping_add(vw) && rect[3].wrapping_add(y0) > vy && rect[2].wrapping_add(x0) > 0 {
            host.blit(rect[0], rect[1], x0, y0, rect[2], rect[3]);
        }
    }
    host.select(arrow.handle);
    let count = i32_at(obj, 0x2908);
    for k in 0..count.max(0) {
        let id = entry(k);
        let t = world.ship(id);
        let x = i16_at(t, 0x1E).wrapping_sub(sx).wrapping_add(vx);
        let mut y = i16_at(t, 0x22).wrapping_sub(sy).wrapping_add(vy);
        rect_of(tex.hull(r8(t, 0xE)), i32::from(r8(t, 1)), &mut rect, &mut handle);
        y = y.wrapping_sub(half(rect[3]));
        let (colour, _, _) = side_colour(world, t, host);
        host.color(colour);
        let (aw, ah) = host.size(arrow.handle);
        for &sid in selection {
            let conv = r16(world.ship(u32::from(sid)), 8);
            let leader = if conv < world.convoy_count { u32::from(r16(world.convoy(u32::from(conv)), 0x10)) } else { u32::from(sid) };
            if leader == id {
                let dy = y.wrapping_sub(i32::from(r16(obj, 0x2B0C))).wrapping_sub(ah);
                host.blit(0, 0, x.wrapping_sub(half(aw)), dy, aw, ah);
            }
        }
        if u32::from(r8(t, 0)) != world.player && u32::from(r16(obj, 0x2B14)) == id {
            host.color(u32::from(r8(obj, 0x2B1C)) << 24 | 0x80_8080);
            host.blit(0, 0, x.wrapping_sub(half(aw)), y.wrapping_sub(ah), aw, ah);
        }
    }
}

/// Animación de una entidad de tiempo, común a `0x0044B630` y `0x0044B970`: con el plazo `+0x29E8` vencido sortea (`rand`) el
/// siguiente y si crece o mengua; con `+0x2A28` vencido avanza el fotograma `+0x2A78`.
pub fn animate_weather_entry(obj: &mut [u8], i: usize, clock: u32, host: &mut dyn NavHost) {
        let cnt = r8(obj, 0x2A68 + i);
        let interval = r8(obj, 0x2A98 + i);
        if r32(obj, 0x29E8 + 4 * i) < clock {
            let r = host.rand();
            let f = (f64::from(r as i32 as f32) / 32767.0) as f32;
            let v = ftol(f64::from(f32::from(interval)) * f64::from(f32::from(cnt)) * f64::from(f) / -4.0);
            w32(obj, 0x29E8 + 4 * i, clock.wrapping_sub(v as u32));
            // `fcomp 0.5`: C0|C3 (menor o igual, o no ordenado) → crece.
            let grow = !(f64::from(f) > 0.5);
            w8(obj, 0x2AA8 + i, u8::from(grow));
        }
        if r8(obj, 0x2A78 + i) == 0 {
            w8(obj, 0x2AA8 + i, 1);
        }
        if i32::from(r8(obj, 0x2A78 + i)) >= i32::from(cnt) - 1 {
            w8(obj, 0x2AA8 + i, 0);
            w8(obj, 0x2A78 + i, cnt.wrapping_sub(1));
        }
        if r32(obj, 0x2A28 + 4 * i) < clock {
            let f = r8(obj, 0x2A78 + i);
            if r8(obj, 0x2AA8 + i) == 0 {
                if f > 0 {
                    w8(obj, 0x2A78 + i, f - 1);
                }
            } else if f < cnt {
                w8(obj, 0x2A78 + i, f + 1);
            }
            w32(obj, 0x2A28 + 4 * i, clock.wrapping_add(u32::from(interval)));
        }
}

/// `0x0044B970`: tormentas y calmas. `clock` es `DAT_006DCCF0`.
pub fn draw_weather_icons(obj: &mut [u8], tex: &Textures, clock: u32, host: &mut dyn NavHost) {
    let mut rect = [0i32; 4];
    let mut handle = 0u32;
    for i in 0..16usize {
        let kind = r8(obj, 0x2A88 + i);
        if kind >= 0x10 {
            continue;
        }
        animate_weather_entry(obj, i, clock, host);
        let id = match kind {
            0 => r32(obj, 0x2AF8),
            1 => r32(obj, 0x2B00),
            _ => continue,
        };
        host.lookup(id);
        let t = tex.lookup(id);
        let frame = i32::from(r8(obj, 0x2A78 + i));
        rect_of(Some(&t), frame, &mut rect, &mut handle);
        let px = i16_at(obj, 0x2AB8 + 4 * i);
        let py = i16_at(obj, 0x2ABA + 4 * i);
        let x = px.wrapping_sub(i16_at(obj, 0x29E4)).wrapping_sub(half(rect[2]));
        let y = py.wrapping_sub(i16_at(obj, 0x29E6)).wrapping_sub(half(rect[3])).wrapping_add(i32_at(obj, 0x18));
        let (vx, vy) = (i32_at(obj, 0x14), i32_at(obj, 0x18));
        let visible = x < vx.wrapping_add(i32_at(obj, 0x2C))
            && y < vy.wrapping_add(i32_at(obj, 0x30))
            && rect[3].wrapping_add(y) > vy
            && rect[2].wrapping_add(x) > 0;
        if !visible {
            continue;
        }
        if kind == 0 {
            rect_of(Some(&tex.storm_shadow), 0, &mut rect, &mut handle);
            host.select(handle);
            host.color(0x78FF_FFFF);
            host.blit(rect[0], rect[1], x, y.wrapping_add(0x1E), rect[2], rect[3]);
        }
        host.color(0xFFFF_FFFF);
        rect_of(Some(&t), frame, &mut rect, &mut handle);
        host.select(handle);
        host.blit(rect[0], rect[1], x, y, rect[2], rect[3]);
    }
}
