//! Modo de vídeo del juego en el enlazador y opción de resoluciones modernas (hilo «Añadir resoluciones modernas al juego»,
//! 12/11/2026). Documento: `reimpl/RESOLUCIONES_MODERNAS.md`.
//!
//! **Modo original** ([`DisplayState::modern`] = `None`, por defecto). Todo cambio de modo del juego pasa por
//! `set_ddraw_resolution(hwnd, config*)` (`0x004BCCC0`): menús a 800×600 (`0x004638D6`, `0x0042D010`, `0x0045E537`) y partida a
//! 800×600, 1024×768 o 1280×1024 según las opciones (`0x00464027`, `0x00423C25`, `0x0045FE9E`, `0x00432D7C`). Aquí deja lo mismo que
//! el original (`0x004BCE5A..0x004BCE79`: ancho y alto de la configuración en `0x006DAB4C/44` y `0x006DCCE0/E4`) y además cambia
//! el tamaño de [`super::Surface`] al modo pedido, que es lo que hacía DirectDraw con el monitor. Ninguna función del juego cambia.
//! Las dos funciones de colocación (`0x00429A40`, `0x00429FE0`) se ejecutan desde `display_lifted.rs`: el mismo código x86 elevado
//! de nuevo con la retirada real de cada llamada virtual (`tools/ui_rt_display.py`); la versión de `p3_core::lifted` desplazaba la
//! pila en la rama de 1280×1024, que sus trazas no recorrían.
//!
//! **Modo moderno** (`modern = Some((ancho, alto))`). Los menús siguen a 800×600 (la ventana los amplía sin deformar). Al entrar en
//! partida, `game_screen_apply_resolution_layout` (`0x00429A40`) se envuelve: la resolución del juego pasa a ser la pedida y la
//! propia función original coloca la interfaz, porque casi todo lo calcula a partir de la barra vertical derecha (`+0xC40`) y de
//! `0x006DCCE4`. Lo único que el original resuelve con tres casos fijos (ancho 800, 1024 o 1280) es:
//!
//! - la pieza superior izquierda de la barra (`+0x8E0` de 516 px, `+0x9B8` de 740, `+0xA90` de 996) y el relleno inferior de la
//!   barra lateral (`+0xD18` de 284×168, `+0xDF0` de 284×424);
//! - el desplazamiento de la pieza superior derecha (`+0x808`), la barra vertical (`+0xC40`) y la horizontal (`+0xB68`);
//! - el reparto de los cuatro elementos de la barra superior (`0x00429FE0`, que con otro ancho usa una variable sin iniciar).
//!
//! Los tres casos salen del ancho de la pieza superior izquierda (`[vt+0x44]`, el del fotograma de su animación). Para otro ancho
//! esa pieza pasa a medir `ancho − 284` (se responde eso a su `get_width`) y el original se ejecuta en dos pasadas: la primera
//! con el ancho estándar mayor que cabe (inserta sus piezas, prepara el mapa a pantalla completa de ese tamaño y coloca las tres
//! barras a partir de la pieza) y la segunda con el ancho real (rama por defecto: recoloca todo lo demás y da a la escena de
//! ciudad `(ancho − 284) × (alto − 42)`). Las piezas se alargan al dibujarse: los tablones de la barra superior se repiten entre
//! la pieza original y su remate derecho, y el relleno inferior se apila hasta el borde de la pantalla.
use p3_render::scene_math::Mem;

/// Resolución del juego (`0x006DCCE0` × `0x006DCCE4`) y tamaño de la ventana (`0x006DAB4C` × `0x006DAB44`).
pub const RES_X: u32 = 0x006D_CCE0;
pub const RES_Y: u32 = 0x006D_CCE4;
pub const WINDOW_W: u32 = 0x006D_AB4C;
pub const WINDOW_H: u32 = 0x006D_AB44;
/// Puntero global a `GameScreen`.
pub const GAME_SCREEN: u32 = 0x006C_BB40;

pub const SET_DDRAW_RESOLUTION: u32 = 0x004B_CCC0;
pub const APPLY_RESOLUTION_LAYOUT: u32 = 0x0042_9A40;
pub const REPOSITION_TOP_BAR: u32 = 0x0042_9FE0;
const WINDOW_SET_SIZE: u32 = 0x0040_24B0;
const WINDOW_GET_WIDTH: u32 = 0x004B_DAC0;
const INSERT_CHILD_AT: u32 = 0x004B_4C90;

/// Hijos de `GameScreen` que dependen del ancho.
pub const TOP_RIGHT: u32 = 0x808;
pub const VERTICAL_BAR: u32 = 0xC40;
pub const HORIZONTAL_BAR: u32 = 0xB68;

/// Juego de piezas de una resolución estándar del original.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Base {
    /// Ancho de pantalla del original al que pertenece.
    pub width: u32,
    /// Pieza superior izquierda (desplazamiento en `GameScreen`) y su ancho.
    pub top_left: u32,
    pub top_left_w: i32,
    /// Columna de la textura donde empieza el tramo de tablones que se repite (una junta) y su largo (múltiplo del adorno, 39 px).
    pub tile: (i32, i32),
    /// Relleno inferior de la barra lateral (desplazamiento y alto), si lo hay.
    pub filler: Option<(u32, i32)>,
}

/// Remate derecho de la pieza superior izquierda: de la última junta al final (igual en las tres).
pub const TOP_TAIL: i32 = 258;
pub const TOP_BAR_H: i32 = 42;

pub const BASES: [Base; 3] = [
    Base { width: 1280, top_left: 0xA90, top_left_w: 996, tile: (464, 273), filler: Some((0xDF0, 424)) },
    Base { width: 1024, top_left: 0x9B8, top_left_w: 740, tile: (208, 273), filler: Some((0xD18, 168)) },
    Base { width: 800, top_left: 0x8E0, top_left_w: 516, tile: (258, 234), filler: None },
];

/// Juego de piezas para un ancho: el estándar mayor que cabe.
pub fn base_for(width: u32) -> Base {
    BASES.into_iter().find(|b| width >= b.width).unwrap_or(BASES[2])
}

/// Anchos que el original coloca por sí mismo.
pub fn is_standard_width(width: u32) -> bool {
    matches!(width, 800 | 1024 | 1280)
}

/// Tramos `(columna de textura, x de destino relativa, ancho)` que alargan la pieza superior izquierda hasta `bar_w` píxeles.
/// Vacío si la pieza ya mide eso o más.
pub fn top_bar_spans(base: &Base, bar_w: i32) -> Vec<(i32, i32, i32)> {
    let mut out = Vec::new();
    if bar_w <= base.top_left_w {
        return out;
    }
    let tail_src = base.top_left_w - TOP_TAIL;
    let tail_x = bar_w - TOP_TAIL;
    let mut x = tail_src;
    while x < tail_x {
        let n = base.tile.1.min(tail_x - x);
        out.push((base.tile.0, x, n));
        x += n;
    }
    out.push((tail_src, tail_x, TOP_TAIL));
    out
}

/// Tramos `(y de destino relativa, alto)` que repiten el relleno inferior desde su borde hasta `rows` filas por debajo de su origen.
pub fn filler_spans(piece_h: i32, rows: i32) -> Vec<(i32, i32)> {
    let mut out = Vec::new();
    let mut y = piece_h;
    while piece_h > 0 && y < rows {
        let n = piece_h.min(rows - y);
        out.push((y, n));
        y += n;
    }
    out
}

/// Resolución de juego para un monitor: su tamaño dividido por el menor entero que deja el alto en 1600 o menos (4K → 1920×1080
/// al doble; 2560×1440 y 3440×1440 se quedan como están), sin bajar de 1024×720. Devuelve (ancho, alto, factor).
pub fn auto_resolution(monitor_w: u32, monitor_h: u32) -> (u32, u32, u32) {
    let mut k = 1;
    while monitor_h / k > 1600 && monitor_w / (k + 1) >= 1024 && monitor_h / (k + 1) >= 720 {
        k += 1;
    }
    (monitor_w / k, monitor_h / k, k)
}

/// Estado del modo de vídeo.
#[derive(Debug, Default)]
pub struct DisplayState {
    /// Resolución moderna de la partida; `None` = comportamiento original.
    pub modern: Option<(u32, u32)>,
    /// Último modo fijado (por `set_ddraw_resolution` o por la colocación moderna).
    pub mode: (u32, u32),
    /// La última colocación de `GameScreen` fue la moderna con un ancho que el original no conoce: pieza superior izquierda
    /// (objeto) y el ancho que ocupa.
    pub extended: Option<(u32, u32)>,
    /// Cambios de modo desde el arranque.
    pub mode_changes: u32,
    /// Piezas alargadas: (objeto, (textura, x, y de su origen)) del último dibujo que las tocó.
    pub chrome: Vec<(u32, (u32, i32, i32))>,
    /// Función cuya próxima llamada no se intercepta (la del propio gancho al original).
    skip: Option<u32>,
}

impl super::Runtime<'_> {
    /// Deja el modo `w × h` como `set_ddraw_resolution`: globales del juego y superficie del enlazador.
    pub fn set_video_mode(&mut self, m: &mut dyn Mem, w: u32, h: u32) {
        m.w32(WINDOW_W, w);
        m.w32(WINDOW_H, h);
        m.w32(RES_X, w);
        m.w32(RES_Y, h);
        if self.display.mode != (w, h) {
            self.display.mode = (w, h);
            self.display.mode_changes += 1;
        }
        let (sw, sh) = (w as usize, h as usize);
        if sw != 0 && sh != 0 && (self.surface.width, self.surface.height) != (sw, sh) {
            let s = &mut self.surface;
            s.width = sw;
            s.height = sh;
            s.rgba = [0, 0, 0, 0xFF].repeat(sw * sh);
        }
    }

    fn vcall(&mut self, m: &mut dyn Mem, obj: u32, slot: u32, args: &[u32]) -> u32 {
        let vt = m.r32(obj);
        let f = m.r32(vt.wrapping_add(slot));
        self.dispatch(m, f, obj, args)
    }

    /// Llama a la función original saltándose este gancho.
    fn original(&mut self, m: &mut dyn Mem, f: u32, ecx: u32, args: &[u32]) -> u32 {
        self.display.skip = Some(f);
        let r = self.dispatch(m, f, ecx, args);
        self.display.skip = None;
        r
    }

    /// Nativas del modo de vídeo; se consultan antes que las de `menus.rs`.
    pub(super) fn display_native(&mut self, m: &mut dyn Mem, f: u32, ecx: u32, esp: u32) -> Option<u32> {
        if self.display.skip == Some(f) {
            self.display.skip = None;
            return None;
        }
        if f == SET_DDRAW_RESOLUTION {
            let cfg = m.r32(esp + 8);
            let (w, h) = (m.r32(cfg + 8), m.r32(cfg + 0xC));
            self.display.extended = None;
            self.display.chrome.clear();
            self.set_video_mode(m, w, h);
            return Some(1);
        }
        let (w, h) = self.display.modern?;
        let gs = m.r32(GAME_SCREEN);
        if gs == 0 || ecx < gs || ecx >= gs + 0x1000 {
            return None;
        }
        match f {
            // Ancho de la pieza superior izquierda con la colocación moderna.
            WINDOW_GET_WIDTH => self.display.extended.filter(|(bar, _)| *bar == ecx).map(|(_, bar_w)| bar_w),
            APPLY_RESOLUTION_LAYOUT if ecx == gs => Some(self.modern_layout(m, gs, w, h)),
            // Con un ancho no estándar el original reparte la barra con una variable sin iniciar (`0x0042A048`): se le presenta el
            // estándar de base, cuya pieza superior ya mide `ancho − 284`.
            REPOSITION_TOP_BAR if ecx == gs && !is_standard_width(m.r32(RES_X)) => {
                let real = m.r32(RES_X);
                m.w32(RES_X, base_for(real).width);
                let r = self.original(m, f, ecx, &[]);
                m.w32(RES_X, real);
                Some(r)
            }
            // `GameScreen::set_size(ancho, alto, 0)` tras la colocación: los llamantes pasan el modo de las opciones.
            WINDOW_SET_SIZE if ecx == gs => {
                m.w32(esp + 4, w);
                m.w32(esp + 8, h);
                None
            }
            _ if self.display.extended.is_some() => self.chrome_draw(m, f, ecx, esp, gs, w, h),
            _ => None,
        }
    }

    /// `game_screen_apply_resolution_layout` con la resolución moderna. Ver el módulo.
    fn modern_layout(&mut self, m: &mut dyn Mem, gs: u32, w: u32, h: u32) -> u32 {
        self.set_video_mode(m, w, h);
        self.display.chrome.clear();
        let base = base_for(w);
        if is_standard_width(w) {
            self.display.extended = None;
            return self.original(m, APPLY_RESOLUTION_LAYOUT, gs, &[]);
        }
        // La barra lateral va pegada al borde derecho y la pieza superior izquierda ocupa el resto.
        let side_w = self.vcall(m, gs + TOP_RIGHT, 0x44, &[]);
        let x = w.saturating_sub(side_w);
        self.display.extended = Some((gs + base.top_left, x));
        if let Some((filler, _)) = base.filler {
            let y = self.vcall(m, gs + filler, 0x8C, &[]);
            self.vcall(m, gs + filler, 0x78, &[x, y]);
        }
        // Primera pasada: el original con el ancho estándar de base (coloca las tres barras según el ancho de la pieza).
        m.w32(RES_X, base.width);
        self.original(m, APPLY_RESOLUTION_LAYOUT, gs, &[]);
        m.w32(RES_X, w);
        // Segunda pasada: rama por defecto del original con el ancho real (quita las ocho piezas, reinserta las tres barras y
        // recoloca escena, carta, minimapa y botones a partir de ellas).
        let r = self.original(m, APPLY_RESOLUTION_LAYOUT, gs, &[]);
        // Las piezas del ancho de base, en el mismo orden en que las deja el original: tras las tres barras.
        self.dispatch(m, INSERT_CHILD_AT, gs, &[3, gs + base.top_left]);
        if let Some((filler, _)) = base.filler {
            self.dispatch(m, INSERT_CHILD_AT, gs, &[3, gs + filler]);
        }
        r
    }

    /// Dibujo de la pieza superior izquierda o del relleno inferior con la colocación moderna: tras el dibujo original se
    /// repite la textura hasta cubrir el ancho o el alto que el original no tiene.
    fn chrome_draw(&mut self, m: &mut dyn Mem, f: u32, ecx: u32, esp: u32, gs: u32, w: u32, h: u32) -> Option<u32> {
        let base = base_for(w);
        let top = ecx == gs + base.top_left;
        let filler = base.filler.filter(|(o, _)| ecx == gs + o);
        if !top && filler.is_none() {
            return None;
        }
        let vt = m.r32(ecx);
        if f != m.r32(vt.wrapping_add(0x9C)) {
            return None;
        }
        let args = [m.r32(esp + 4), m.r32(esp + 8), m.r32(esp + 12), m.r32(esp + 16)];
        self.surface.last_blit = None;
        let r = self.original(m, f, ecx, &args);
        // El original deja restaurada la textura activa: la de la pieza y su posición salen de su último volcado. Al repintar
        // solo una zona (el ratón sobre la interfaz, un aviso), ese volcado es un trozo recortado: el origen de la pieza es
        // destino − origen en la textura. Si la zona no toca la pieza original no hay volcado, pero puede tocar la parte
        // alargada: se usa lo guardado del último dibujo completo de esa pieza.
        let piece = match self.surface.last_blit {
            Some((source, (sx, sy, dx, dy, _, _))) => (source, dx - sx, dy - sy),
            None => match self.display.chrome.iter().find(|(o, _)| *o == ecx) {
                Some(&(_, p)) => p,
                None => return Some(r),
            },
        };
        self.display.chrome.retain(|(o, _)| *o != ecx);
        self.display.chrome.push((ecx, piece));
        let (source, x0, y0) = piece;
        let size = self.assets.image(source).map(|i| (i.width as i32, i.height as i32));
        let (color, mode) = (self.surface.color, self.surface.mode);
        self.surface.color = 0xFFFF_FFFF;
        self.surface.mode = 1;
        if top && size == Some((base.top_left_w, TOP_BAR_H)) {
            let bar_w = self.display.extended.map_or(0, |e| e.1 as i32);
            for (sx, dx, n) in top_bar_spans(&base, bar_w) {
                self.blit_clipped(source, (sx, 0, x0 + dx, y0, n, TOP_BAR_H));
            }
        }
        if let Some((_, piece_h)) = filler.filter(|(_, ph)| size.is_some_and(|s| s.1 == *ph)) {
            let piece_w = size.map_or(0, |s| s.0);
            for (dy, n) in filler_spans(piece_h, h as i32 - y0) {
                self.blit_clipped(source, (0, 0, x0, y0 + dy, piece_w, n));
            }
        }
        self.surface.color = color;
        self.surface.mode = mode;
        Some(r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_by_width() {
        assert_eq!(base_for(1920).width, 1280);
        assert_eq!(base_for(1280).width, 1280);
        assert_eq!(base_for(1279).width, 1024);
        assert_eq!(base_for(1024).width, 1024);
        assert_eq!(base_for(960).width, 800);
        assert_eq!(base_for(640).width, 800);
    }

    /// Los tramos cubren exactamente de la última junta de la pieza al final de la barra, sin huecos ni solapes, y acaban en el remate.
    #[test]
    fn top_bar_is_covered() {
        for base in BASES {
            assert!(top_bar_spans(&base, base.top_left_w).is_empty());
            for bar_w in [base.top_left_w + 1, base.top_left_w + 82, 1636, 2276, 3556] {
                if bar_w <= base.top_left_w {
                    continue;
                }
                let spans = top_bar_spans(&base, bar_w);
                let mut x = base.top_left_w - TOP_TAIL;
                for (sx, dx, n) in &spans[..spans.len() - 1] {
                    assert_eq!((*sx, *dx), (base.tile.0, x));
                    assert!(*n > 0 && *n <= base.tile.1 && sx + n <= base.top_left_w);
                    x += n;
                }
                assert_eq!(x, bar_w - TOP_TAIL);
                assert_eq!(*spans.last().unwrap(), (base.top_left_w - TOP_TAIL, bar_w - TOP_TAIL, TOP_TAIL));
            }
        }
    }

    #[test]
    fn filler_reaches_the_bottom() {
        assert!(filler_spans(424, 424).is_empty());
        assert!(filler_spans(424, 120).is_empty());
        assert_eq!(filler_spans(424, 480), vec![(424, 56)]);
        assert_eq!(filler_spans(424, 840), vec![(424, 416)]);
        assert_eq!(filler_spans(168, 600), vec![(168, 168), (336, 168), (504, 96)]);
    }

    #[test]
    fn auto_resolution_keeps_the_interface_readable() {
        assert_eq!(auto_resolution(1920, 1080), (1920, 1080, 1));
        assert_eq!(auto_resolution(2560, 1440), (2560, 1440, 1));
        assert_eq!(auto_resolution(3440, 1440), (3440, 1440, 1));
        assert_eq!(auto_resolution(3840, 2160), (1920, 1080, 2));
        assert_eq!(auto_resolution(5120, 2880), (2560, 1440, 2));
        assert_eq!(auto_resolution(1366, 768), (1366, 768, 1));
    }
}
