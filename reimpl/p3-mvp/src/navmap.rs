//! Carta náutica del cliente: texturas reales de `scripts/textures.ini`, superficie RGBA que ejecuta las llamadas de dibujo de
//! [`p3_render::navmap`] y composición de un fotograma (fondo, tormentas/calmas, barcos y flechas).
//!
//! La lógica (qué se dibuja, dónde, con qué fotograma y color, y el desplazamiento) es la del original con paridad x86. Cada
//! copia (`0x004BB330`) mezcla como `sgl_DrawBitmapRect` del controlador `ddraw_Dll.dll` en superficies de memoria
//! ([`p3_render::sgl`], paridad con la DLL en emulación): modo de `sgl_SetBlendMode` (1 al empezar), color y los indicadores
//! que `0x004B3610` deriva de `textures.ini`. Los rellenos y el texto (número de barcos del convoy, fuente de cifras propia
//! de 3×5 píxeles, no la del juego) siguen con composición alfa.

use p3_assets::cpr::Archive;
use p3_render::navmap::{NavHost, Tex, Textures};
use std::collections::HashMap;

/// Identificadores de `scripts/scrollmaps.ini` (`[Navmap0]`, `[Wetter]`, `[Holk]`…) y la flecha `0x7557`.
pub const BACKGROUND: u32 = 16036;
pub const STORM: u32 = 30003;
pub const CALM: u32 = 30000;
pub const ICEBERG: u32 = 30002;
pub const STORM_SHADOW: u32 = 30001;
pub const HOLK: u32 = 30004;
pub const KOGGE: u32 = 30005;
pub const KRAIER: u32 = 30006;
pub const SCHNIGGE: u32 = 30007;
pub const MARKER: u32 = 30008;
/// `[Ereignis] EreignisID` (iconos de estado, `+0xC4`) y `WareFehltID` (mercancía que falta, `+0xC8`).
pub const EVENTS: u32 = 30009;
pub const SHORTAGE: u32 = 30040;
/// `[Navmap0] ButtonID`: símbolos de ciudad (`+0xD0`, 3 clases × 3 estados de cursor).
pub const TOWN_BUTTON: u32 = 30021;
/// Puntos de ruta (`0x7551`) y marcas (`0x7552`) que [`p3_render::navmap_draw::draw_nav_map`] busca por identificador.
pub const WAYPOINT: u32 = 0x7551;
pub const MARK: u32 = 0x7552;

/// Imagen RGBA de una textura del catálogo.
#[derive(Clone, Debug)]
pub struct Image {
    pub width: usize,
    pub height: usize,
    pub rgba: Vec<u8>,
}

/// Una sección `[TEXn]` de `textures.ini`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TexDef {
    pub file: String,
    pub frames: Vec<[i32; 4]>,
    pub color_key: Option<u32>,
    /// Indicadores de creación de `0x004B3610`: `1 | UseFiltering·0x10 | UseAlpha=true·3 | UseAlpha=mask·0x8000 |
    /// UseColorKey·7 | BlendMode` (`normal`, `multiply`, `add`, `psx` → 1..4 en los bits 24–26).
    pub flags: u32,
}

/// Lee las secciones `[TEXn]` (`TextureFile`, `Count`, `OffsetNSize<i>`, `UseColorKey`, `ColorKey`).
pub fn parse_textures(ini: &str) -> HashMap<u32, TexDef> {
    let mut out = HashMap::new();
    let mut current: Option<(u32, HashMap<String, String>)> = None;
    let flush = |cur: &mut Option<(u32, HashMap<String, String>)>, out: &mut HashMap<u32, TexDef>| {
        if let Some((id, keys)) = cur.take() {
            let count = keys.get("count").and_then(|v| v.trim().parse::<usize>().ok()).unwrap_or(0);
            let frames = (0..count)
                .map(|i| {
                    let v: Vec<i32> = keys
                        .get(&format!("offsetnsize{i}"))
                        .map(|s| s.split_whitespace().filter_map(|n| n.parse().ok()).collect())
                        .unwrap_or_default();
                    [0, 1, 2, 3].map(|k| v.get(k).copied().unwrap_or(0))
                })
                .collect();
            let key = keys.get("usecolorkey").is_some_and(|v| v.trim().eq_ignore_ascii_case("true"));
            let color_key = key
                .then(|| keys.get("colorkey").and_then(|v| u32::from_str_radix(v.trim().trim_start_matches("0x"), 16).ok()))
                .flatten();
            let is = |k: &str, v: &str| keys.get(k).is_some_and(|x| x.trim().eq_ignore_ascii_case(v));
            let blend = ["normal", "multiply", "add", "psx"].iter().position(|m| is("blendmode", m)).map_or(0, |i| (i as u32 + 1) << 24);
            let flags = 1
                | if is("usefiltering", "true") { 0x10 } else { 0 }
                | if is("usealpha", "true") { 3 } else { 0 }
                | if is("usealpha", "mask") { 0x8000 } else { 0 }
                | if key { 7 } else { 0 }
                | blend;
            out.insert(id, TexDef { file: keys.get("texturefile").cloned().unwrap_or_default(), frames, color_key, flags });
        }
    };
    for raw in ini.lines() {
        let line = raw.split(';').next().unwrap_or("").trim();
        if let Some(sec) = line.strip_prefix('[').and_then(|l| l.strip_suffix(']')) {
            flush(&mut current, &mut out);
            current = sec.strip_prefix("TEX").and_then(|n| n.parse().ok()).map(|id| (id, HashMap::new()));
        } else if let (Some((_, keys)), Some((k, v))) = (current.as_mut(), line.split_once('=')) {
            keys.insert(k.trim().to_ascii_lowercase(), v.trim().to_string());
        }
    }
    flush(&mut current, &mut out);
    out
}

/// JPEG de 8 bits (gris o RGB) a RGBA opaco. El original usa la biblioteca de Intel (`ijl11.dll`); el descodificador de aquí
/// puede diferir en algún nivel por píxel (redondeo de la IDCT), no en el contenido.
fn decode_jpeg(data: &[u8]) -> Option<(usize, usize, Vec<u8>)> {
    let mut decoder = jpeg_decoder::Decoder::new(data);
    let pixels = decoder.decode().ok()?;
    let info = decoder.info()?;
    let rgba = match info.pixel_format {
        jpeg_decoder::PixelFormat::RGB24 => pixels.chunks_exact(3).flat_map(|p| [p[0], p[1], p[2], 0xFF]).collect(),
        jpeg_decoder::PixelFormat::L8 => pixels.iter().flat_map(|&l| [l, l, l, 0xFF]).collect(),
        _ => return None,
    };
    Some((usize::from(info.width), usize::from(info.height), rgba))
}

/// BMP de 8 bits con paleta como imagen indexada (`AIM_IMAGE` formato 1, de arriba abajo, paso a 4 bytes): el EXE lee los
/// índices con `sgl_LockTexture` (mapa de zonas del menú principal). `None` si no es de 8 bits sin compresión.
pub fn bmp_indexed(data: &[u8]) -> Option<p3_assets::aim::Image> {
    let rd = |o: usize, n: usize| -> Option<u32> { Some(data.get(o..o + n)?.iter().rev().fold(0u32, |a, &b| (a << 8) | u32::from(b))) };
    if data.get(..2)? != b"BM" || rd(28, 2)? != 8 || rd(30, 4)? != 0 {
        return None;
    }
    let (off, dib) = (rd(10, 4)? as usize, rd(14, 4)? as usize);
    let (w, h) = (rd(18, 4)? as usize, rd(22, 4)? as i32);
    let top_down = h < 0;
    let h = h.unsigned_abs() as usize;
    let colors = match rd(46, 4)? as usize {
        0 => 256,
        n => n.min(256),
    };
    let mut palette = vec![0u8; 1024];
    for i in 0..colors {
        let e = data.get(14 + dib + 4 * i..14 + dib + 4 * i + 4)?;
        palette[4 * i..4 * i + 4].copy_from_slice(&[e[0], e[1], e[2], 0xFF]);
    }
    let pitch = (w + 3) & !3;
    let mut pixels = vec![0u8; pitch * h];
    for y in 0..h {
        let sy = if top_down { y } else { h - 1 - y };
        pixels[y * pitch..y * pitch + w].copy_from_slice(data.get(off + sy * pitch..off + sy * pitch + w)?);
    }
    Some(p3_assets::aim::Image { pixels, palette: Some(palette), width: w as u32, height: h as u32, bytes_per_pixel: 1, format: 1, pitch: pitch as u32 })
}

pub(crate) fn decode(data: &[u8], name: &str) -> Option<Image> {
    let lower = name.to_ascii_lowercase();
    let (width, height, rgba) = if lower.ends_with(".tga") {
        p3_render::decode_tga(data)?
    } else if lower.ends_with(".bmp") {
        p3_render::decode_bmp(data)?
    } else if lower.ends_with(".jpg") {
        decode_jpeg(data)?
    } else {
        let img = p3_assets::aim::decode(data).ok()?;
        let t = p3_render::to_rgba(&img)?;
        (t.width, t.height, t.rgba)
    };
    Some(Image { width, height, rgba })
}

/// Texturas de la carta cargadas del CPR (el manejador de cada una es su identificador).
pub struct NavAssets {
    pub defs: HashMap<u32, TexDef>,
    pub images: HashMap<u32, Image>,
    /// Fuentes de `scripts/fonts.ini` por manejador (`0x006DCD28 + 0xA8·N`, la tabla de objetos `SGLFont` del EXE); su
    /// imagen está en `images` con el mismo manejador.
    pub fonts: HashMap<u32, p3_render::sgl_font::Font>,
    /// Píxeles de cada imagen ya convertidos a `0xAARRGGBB` (el formato de [`p3_render::sgl`]), una sola vez al cargar:
    /// los usa la copia rápida de [`Canvas`]. Lo rellena [`NavAssets::convert`]; una imagen sin entrada se dibuja por el
    /// camino de referencia.
    pub argb: HashMap<u32, Vec<u32>>,
}

/// Manejador de la fuente `[FontN]` (objeto `0x006DCD28 + 0xA8·N`, `0x00401723`).
pub fn font_handle(n: u32) -> u32 {
    0x006D_CD28 + 0xA8 * n
}

/// `0x004C1250`..`0x004C1532`: cada `[FontN]` de `fonts.ini` con `Params=ancho alto espaciado` (`%i %i %i`), `SpaceWidth`
/// (ancho por defecto, 4; se ignora por encima de 50), `Type` (`Kerning` crea la fuente con kerning, si no proporcional) y la
/// textura `Font`. La textura se carga con indicadores 3 (`0x004BCA00(ruta, 3)`).
pub(crate) fn load_fonts(archive: &Archive, defs: &mut HashMap<u32, TexDef>, images: &mut HashMap<u32, Image>) -> HashMap<u32, p3_render::sgl_font::Font> {
    let mut fonts = HashMap::new();
    let Some(ini) = archive.find(b"scripts/fonts.ini").and_then(|e| archive.read(e)) else { return fonts };
    let ini = String::from_utf8_lossy(ini);
    let mut sections: Vec<(u32, HashMap<String, String>)> = Vec::new();
    for line in ini.lines() {
        let line = line.trim();
        if let Some(n) = line.strip_prefix("[Font").and_then(|l| l.strip_suffix(']')).and_then(|n| n.parse().ok()) {
            sections.push((n, HashMap::new()));
        } else if let (Some((_, keys)), Some((k, v))) = (sections.last_mut(), line.split_once('=')) {
            if !k.trim_start().starts_with(';') {
                keys.insert(k.trim().to_ascii_lowercase(), v.trim().to_string());
            }
        }
    }
    for (n, keys) in sections {
        let params: Vec<i32> = keys.get("params").map(|p| p.split_whitespace().filter_map(|v| v.parse().ok()).collect()).unwrap_or_default();
        let (Some(file), [w, h, spacing, ..]) = (keys.get("font"), params.as_slice()) else { continue };
        let Some(img) = archive.find(file.as_bytes()).and_then(|e| archive.read(e)).and_then(|d| decode(d, file)) else { continue };
        let space = keys.get("spacewidth").and_then(|v| v.parse::<u32>().ok()).unwrap_or(4);
        let space = if space <= 0x32 { space as u8 } else { 4 };
        let bgra: Vec<u8> = img.rgba.chunks_exact(4).flat_map(|p| [p[2], p[1], p[0], p[3]]).collect();
        let tex = p3_render::sgl_font::Locked { pixels: &bgra, width: img.width as u32, pitch: img.width * 4 };
        let kerning = keys.get("type").is_some_and(|t| t.eq_ignore_ascii_case("kerning"));
        let font = p3_render::sgl_font::create(p3_render::sgl_font::GAME_CHARSET, &tex, *w as u32, *h as u32, *spacing, space, kerning);
        let handle = font_handle(n);
        defs.insert(handle, TexDef { file: file.clone(), frames: vec![], color_key: None, flags: 3 });
        images.insert(handle, img);
        fonts.insert(handle, font);
    }
    fonts
}

impl NavAssets {
    pub fn load(archive: &Archive) -> Result<Self, String> {
        let ini = archive
            .find(b"scripts/textures.ini")
            .and_then(|e| archive.read(e))
            .ok_or("falta scripts/textures.ini")?;
        let defs = parse_textures(&String::from_utf8_lossy(ini));
        let mut images = HashMap::new();
        let port = p3_render::navmap_draw::PORT_TEXTURE;
        for id in [BACKGROUND, STORM, CALM, ICEBERG, STORM_SHADOW, HOLK, KOGGE, KRAIER, SCHNIGGE, MARKER, p3_render::navmap::ARROW_TEXTURE, EVENTS, SHORTAGE, TOWN_BUTTON, port, WAYPOINT, MARK] {
            let def = defs.get(&id).ok_or_else(|| format!("falta [TEX{id}] en textures.ini"))?;
            let data = archive
                .find(def.file.as_bytes())
                .and_then(|e| archive.read(e))
                .ok_or_else(|| format!("falta {}", def.file))?;
            let mut img = decode(data, &def.file).ok_or_else(|| format!("{} no admitido", def.file))?;
            if let Some(key) = def.color_key {
                let [_, r, g, b] = key.to_be_bytes();
                for p in img.rgba.chunks_exact_mut(4) {
                    if (p[0], p[1], p[2]) == (r, g, b) {
                        p[3] = 0;
                    }
                }
            }
            images.insert(id, img);
        }
        let mut defs = defs;
        let fonts = load_fonts(archive, &mut defs, &mut images);
        let mut assets = Self { defs, images, fonts, argb: HashMap::new() };
        assets.convert();
        Ok(assets)
    }

    /// Convierte todas las imágenes a `0xAARRGGBB` (tras aplicar el color clave). Hay que repetirlo si se cambia `images`.
    pub fn convert(&mut self) {
        self.argb = self.images.iter().map(|(&id, img)| (id, img.rgba.chunks_exact(4).map(|p| u32::from_le_bytes([p[2], p[1], p[0], p[3]])).collect())).collect();
    }

    pub fn tex(&self, id: u32) -> Tex {
        Tex { handle: id, frames: self.defs.get(&id).map(|d| d.frames.clone()).unwrap_or_default() }
    }

    /// Texturas en las ranuras del objeto (`+0xAC..+0xD4`) y las que el original busca por identificador.
    pub fn textures(&self) -> Textures {
        Textures {
            kogge: self.tex(KOGGE),
            kraier: self.tex(KRAIER),
            schnigge: self.tex(SCHNIGGE),
            holk: self.tex(HOLK),
            marker: self.tex(MARKER),
            storm_shadow: self.tex(STORM_SHADOW),
            background: self.tex(BACKGROUND),
            lens: Tex::default(),
            by_id: [STORM, CALM, ICEBERG, p3_render::navmap::ARROW_TEXTURE, WAYPOINT, MARK].iter().map(|&id| (id, self.tex(id))).collect(),
        }
    }
}

/// Cifras 3×5 (bit 2 = columna izquierda).
const DIGITS: [[u8; 5]; 10] = [
    [7, 5, 5, 5, 7],
    [2, 6, 2, 2, 7],
    [7, 1, 7, 4, 7],
    [7, 1, 3, 1, 7],
    [5, 5, 7, 1, 1],
    [7, 4, 7, 1, 7],
    [7, 4, 7, 5, 7],
    [7, 1, 1, 1, 1],
    [7, 5, 7, 5, 7],
    [7, 5, 7, 1, 7],
];

/// Superficie de la vista (`w × h`, RGBA) que ejecuta las primitivas del motor.
/// PPM (P6) de una imagen RGBA.
pub fn ppm_bytes(width: usize, height: usize, rgba: &[u8]) -> Vec<u8> {
    let mut out = format!("P6\n{width} {height}\n255\n").into_bytes();
    for p in rgba.chunks_exact(4) {
        out.extend_from_slice(&p[..3]);
    }
    out
}

pub struct Canvas<'a> {
    pub assets: &'a NavAssets,
    pub width: usize,
    pub height: usize,
    pub rgba: Vec<u8>,
    color: u32,
    selected: u32,
    /// `sgl_SetBlendMode` (`0x004BB7A0`): 1 (`normal`) salvo cuando el dibujo lo cambia.
    mode: u8,
    /// `rand` del CRT (`0x00639C03`): estado propio de la carta (`state·214013 + 2531011`, bits 16..30).
    pub rand_state: u32,
    pub ambient: Vec<(u32, u32, i32, i32, u32)>,
    pub word8: Vec<u16>,
    pub draws: usize,
    /// `sgl_SetFont` y `sgl_SetTextMode`.
    font: u32,
    text_mode: i32,
    /// Dibujo de referencia: cada copia convierte la textura y el destino y mezcla píxel a píxel con
    /// [`p3_render::sgl::draw_bitmap_rect`], como antes de la optimización. Con `false` (lo normal) se usan las texturas ya
    /// convertidas y bucles por rutina, con el mismo resultado (`fast_blit_matches_reference`).
    pub reference: bool,
    /// Órdenes de dibujo apuntadas sin ejecutar mientras se graba la capa fija ([`Canvas::begin_layer`]).
    record: Option<Vec<DrawCmd>>,
}

/// Una orden de dibujo de [`Canvas`] con todo el estado del que depende su resultado (textura, color y modo activos).
#[derive(Clone, Debug, PartialEq)]
enum DrawCmd {
    Background(i32, i32),
    Blit { texture: u32, color: u32, mode: u8, rect: [i32; 6] },
    Fill { color: u32, rect: [i32; 4] },
    /// Coordenadas como bits de `f32` (comparación exacta).
    Line { color: u32, mode: u8, ends: [u32; 4] },
    /// Cifras 3×5 de reserva (texto sin fuente cargada).
    Digits { color: u32, x: i32, y: i32, text: String },
}

/// Capa fija de la carta guardada entre fotogramas: la imagen que dejan las órdenes de dibujo anteriores a los icebergs,
/// contactos, barcos y tiempo (fondo, rutas, símbolos, rótulos, estado y barcos en puerto), junto con esas órdenes. El
/// fotograma siguiente vuelve a calcular las órdenes (la lógica del original se ejecuta entera) y, si son las mismas sobre
/// una vista del mismo tamaño, copia la imagen guardada en vez de ejecutarlas. Como la imagen es función solo de la lista
/// de órdenes (las texturas no cambian), no hay nada que invalidar a mano: desplazar, cambiar de tamaño, elegir barcos,
/// pasar el ratón por una ciudad o que entre un barco en puerto cambian la lista y la capa se redibuja.
#[derive(Default)]
pub struct LayerCache {
    size: (usize, usize),
    cmds: Vec<DrawCmd>,
    rgba: Vec<u8>,
    valid: bool,
    /// Fotogramas en que se reutilizó la capa y en que se redibujó.
    pub hits: u64,
    pub misses: u64,
}

impl<'a> Canvas<'a> {
    pub fn new(assets: &'a NavAssets, width: usize, height: usize) -> Self {
        Self { assets, width, height, rgba: vec![0; width * height * 4], color: 0xFFFF_FFFF, selected: 0, mode: 1, rand_state: 1, ambient: vec![], word8: vec![], draws: 0, font: 0, text_mode: 1, reference: false, record: None }
    }

    /// Empieza a grabar la capa fija: hasta [`Canvas::end_layer`] las órdenes de dibujo se apuntan y no tocan la imagen
    /// (el estado —color, textura, modo, fuente— y los contadores sí se actualizan). No hace nada en el dibujo de referencia.
    pub fn begin_layer(&mut self) {
        if !self.reference {
            self.record = Some(Vec::new());
        }
    }

    /// Cierra la grabación: copia la capa guardada si las órdenes y el tamaño coinciden; si no, las ejecuta y guarda el
    /// resultado. Sin grabación en curso no hace nada. La imagen debe estar como al llamar a `begin_layer`.
    pub fn end_layer(&mut self, cache: &mut LayerCache) {
        let Some(cmds) = self.record.take() else { return };
        if cache.valid && cache.size == (self.width, self.height) && cache.cmds == cmds {
            self.rgba.copy_from_slice(&cache.rgba);
            cache.hits += 1;
            return;
        }
        let state = (self.color, self.selected, self.mode);
        for cmd in &cmds {
            self.run(cmd);
        }
        (self.color, self.selected, self.mode) = state;
        cache.rgba.clone_from(&self.rgba);
        cache.size = (self.width, self.height);
        cache.cmds = cmds;
        cache.valid = true;
        cache.misses += 1;
    }

    fn run(&mut self, cmd: &DrawCmd) {
        match cmd {
            DrawCmd::Background(sx, sy) => self.background_now(*sx, *sy),
            DrawCmd::Blit { texture, color, mode, rect: [sx, sy, dx, dy, w, h] } => {
                (self.selected, self.color, self.mode) = (*texture, *color, *mode);
                self.blit_now(*sx, *sy, *dx, *dy, *w, *h);
            }
            DrawCmd::Fill { color, rect: [x, y, w, h] } => {
                self.color = *color;
                self.fill_now(*x, *y, *w, *h);
            }
            DrawCmd::Line { color, mode, ends } => {
                (self.color, self.mode) = (*color, *mode);
                let [x1, y1, x2, y2] = ends.map(f32::from_bits);
                self.line_now(x1, y1, x2, y2);
            }
            DrawCmd::Digits { color, x, y, text } => {
                self.color = *color;
                self.digits_now(*x, *y, text);
            }
        }
    }

    /// Copia opaca del fondo desde `(sx, sy)` del mapa.
    pub fn background(&mut self, sx: i32, sy: i32) {
        if let Some(rec) = self.record.as_mut() {
            rec.push(DrawCmd::Background(sx, sy));
            return;
        }
        self.background_now(sx, sy)
    }

    fn background_now(&mut self, sx: i32, sy: i32) {
        let Some(bg) = self.assets.images.get(&BACKGROUND) else { return };
        if !self.reference {
            // Por filas: negro opaco fuera del mapa y un tramo copiado tal cual dentro.
            let (x0, x1) = ((-sx).clamp(0, self.width as i32) as usize, (bg.width as i32 - sx).clamp(0, self.width as i32) as usize);
            for (y, row) in self.rgba.chunks_exact_mut(self.width * 4).enumerate() {
                let my = sy + y as i32;
                let inside = my >= 0 && (my as usize) < bg.height && x0 < x1;
                let (a, b) = if inside { (x0, x1) } else { (0, 0) };
                let (left, rest) = row.split_at_mut(4 * a);
                let (mid, right) = rest.split_at_mut(4 * (b - a));
                for p in left.chunks_exact_mut(4).chain(right.chunks_exact_mut(4)) {
                    p.copy_from_slice(&[0, 0, 0, 255]);
                }
                if inside {
                    let s = (my as usize * bg.width + (sx + a as i32) as usize) * 4;
                    mid.copy_from_slice(&bg.rgba[s..s + 4 * (b - a)]);
                }
            }
            return;
        }
        for y in 0..self.height {
            let my = sy + y as i32;
            for x in 0..self.width {
                let mx = sx + x as i32;
                let o = (y * self.width + x) * 4;
                if mx >= 0 && my >= 0 && (mx as usize) < bg.width && (my as usize) < bg.height {
                    let s = (my as usize * bg.width + mx as usize) * 4;
                    self.rgba[o..o + 4].copy_from_slice(&bg.rgba[s..s + 4]);
                } else {
                    self.rgba[o..o + 4].copy_from_slice(&[0, 0, 0, 255]);
                }
            }
        }
    }

    fn put(&mut self, x: i32, y: i32, [r, g, b, a]: [u8; 4]) {
        if x < 0 || y < 0 || x as usize >= self.width || y as usize >= self.height || a == 0 {
            return;
        }
        let o = (y as usize * self.width + x as usize) * 4;
        if a == 255 && !self.reference {
            // `(c·255 + d·0) / 255 = c`.
            self.rgba[o..o + 4].copy_from_slice(&[r, g, b, 255]);
            return;
        }
        let a = u32::from(a);
        for (k, c) in [r, g, b].into_iter().enumerate() {
            let d = u32::from(self.rgba[o + k]);
            self.rgba[o + k] = ((u32::from(c) * a + d * (255 - a)) / 255) as u8;
        }
        self.rgba[o + 3] = 255;
    }

    /// Minimapa en la esquina inferior derecha (escalas `9,23077` y `7,92952`): fondo reducido, rectángulo de la vista y los
    /// pulsos de avistamiento (`0x0046E004`: rombo de radio 2 en gris `nivel`).
    pub fn minimap_inset(&mut self, view: [i32; 4], pulses: &[(i32, i32, i32)]) {
        let (mw, mh) = ((2400.0 / p3_core::ships::spotting::MINIMAP_SCALE.0).ceil() as i32, (1800.0 / p3_core::ships::spotting::MINIMAP_SCALE.1).ceil() as i32);
        let (ox, oy) = (self.width as i32 - mw - 8, self.height as i32 - mh - 8);
        if ox < 0 || oy < 0 {
            return;
        }
        if let Some(bg) = self.assets.images.get(&BACKGROUND) {
            for y in 0..mh {
                for x in 0..mw {
                    let (sx, sy) = ((f64::from(x) * p3_core::ships::spotting::MINIMAP_SCALE.0) as usize, (f64::from(y) * p3_core::ships::spotting::MINIMAP_SCALE.1) as usize);
                    if sx < bg.width && sy < bg.height {
                        let s = (sy * bg.width + sx) * 4;
                        let px = [bg.rgba[s], bg.rgba[s + 1], bg.rgba[s + 2], 255];
                        self.put(ox + x, oy + y, px);
                    }
                }
            }
        }
        let yellow = [255, 226, 136, 255];
        let [x1, y1, x2, y2] = view;
        for x in x1..=x2 {
            self.put(ox + x, oy + y1, yellow);
            self.put(ox + x, oy + y2, yellow);
        }
        for y in y1..=y2 {
            self.put(ox + x1, oy + y, yellow);
            self.put(ox + x2, oy + y, yellow);
        }
        for &(x, y, level) in pulses {
            let g = level.clamp(0, 255) as u8;
            let c = [g, g, g, 255];
            let (x, y) = (ox + x, oy + y);
            for (dx, dy) in [(0, -2), (-1, -1), (0, -1), (1, -1), (-2, 0), (-1, 0), (0, 0), (1, 0), (2, 0), (-1, 1), (0, 1), (1, 1), (0, 2)] {
                self.put(x + dx, y + dy, c);
            }
        }
    }

    /// PPM (P6) del fotograma: lo carga `tk.PhotoImage` sin conversiones.
    pub fn ppm(&self) -> Vec<u8> {
        ppm_bytes(self.width, self.height, &self.rgba)
    }
}

impl NavHost for Canvas<'_> {
    fn color(&mut self, argb: u32) {
        self.color = argb;
    }
    fn select(&mut self, handle: u32) {
        self.selected = handle;
    }
    fn blit(&mut self, sx: i32, sy: i32, dx: i32, dy: i32, w: i32, h: i32) {
        if !self.assets.images.contains_key(&self.selected) {
            return;
        }
        self.draws += 1;
        if let Some(rec) = self.record.as_mut() {
            rec.push(DrawCmd::Blit { texture: self.selected, color: self.color, mode: self.mode, rect: [sx, sy, dx, dy, w, h] });
            return;
        }
        self.blit_now(sx, sy, dx, dy, w, h)
    }
    fn set_color(&mut self, argb: u32) -> u32 {
        std::mem::replace(&mut self.color, argb)
    }
    fn set_texture(&mut self, handle: u32) -> u32 {
        std::mem::replace(&mut self.selected, handle)
    }
    fn blend_mode(&mut self, mode: i32) -> i32 {
        std::mem::replace(&mut self.mode, mode as u8).into()
    }
    fn fill(&mut self, x: i32, y: i32, w: i32, h: i32) {
        if let Some(rec) = self.record.as_mut() {
            rec.push(DrawCmd::Fill { color: self.color, rect: [x, y, w, h] });
            return;
        }
        self.fill_now(x, y, w, h)
    }
    /// `sgl_DrawText` con la fuente activa ([`p3_render::sgl_font`]): cada glifo es una copia de su textura con el color y
    /// la mezcla activos. Sin fuente cargada, cifras 3×5 de reserva.
    fn text(&mut self, x: i32, y: i32, s: &str) {
        let bytes: Vec<u8> = s.chars().map(|c| u8::try_from(u32::from(c)).unwrap_or(b'?')).collect();
        if let Some(font) = self.assets.fonts.get(&self.font) {
            let prev = std::mem::replace(&mut self.selected, self.font);
            for [sx, sy, dx, dy, w, h] in font.draw(x, y, &bytes, self.text_mode) {
                self.blit(sx, sy, dx, dy, w, h);
            }
            self.selected = prev;
            return;
        }
        if let Some(rec) = self.record.as_mut() {
            rec.push(DrawCmd::Digits { color: self.color, x, y, text: s.to_string() });
            return;
        }
        self.digits_now(x, y, s)
    }
    fn font(&mut self, font: u32) {
        self.font = font;
    }
    fn text_mode(&mut self, mode: i32) {
        self.text_mode = mode;
    }
    fn text_width(&mut self, font: u32, s: &str) -> i32 {
        let bytes: Vec<u8> = s.chars().map(|c| u8::try_from(u32::from(c)).unwrap_or(b'?')).collect();
        self.assets.fonts.get(&font).map_or(4 * s.len() as i32, |f| f.text_width(&bytes))
    }
    fn size(&mut self, handle: u32) -> (i32, i32) {
        self.assets.images.get(&handle).map_or((0, 0), |i| (i.width as i32, i.height as i32))
    }
    fn line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
        if let Some(rec) = self.record.as_mut() {
            rec.push(DrawCmd::Line { color: self.color, mode: self.mode, ends: [x1, y1, x2, y2].map(f32::to_bits) });
            return;
        }
        self.line_now(x1, y1, x2, y2)
    }
    fn ambient(&mut self, class: u32, id: u32, x: i32, y: i32, extra: u32) {
        self.ambient.push((class, id, x, y, extra));
    }
    fn merchant_word8(&mut self, m: u8) -> u16 {
        self.word8.get(usize::from(m)).copied().unwrap_or(0)
    }
    fn rand(&mut self) -> u32 {
        self.rand_state = self.rand_state.wrapping_mul(214013).wrapping_add(2531011);
        (self.rand_state >> 16) & 0x7FFF
    }
}

impl Canvas<'_> {
    /// Copia de la textura activa con el color y el modo activos (`sgl_DrawBitmapRect`).
    fn blit_now(&mut self, sx: i32, sy: i32, dx: i32, dy: i32, w: i32, h: i32) {
        use p3_render::sgl;
        let assets = self.assets;
        let Some(img) = assets.images.get(&self.selected) else { return };
        let def = assets.defs.get(&self.selected);
        // Recorte del origen a la textura (el original leería fuera) y del destino a la vista (`0x0508D0C8`).
        let (mut sx, mut sy, mut dx, mut dy, mut w, mut h) = (sx, sy, dx, dy, w, h);
        for (s, d, n) in [(&mut sx, &mut dx, &mut w), (&mut sy, &mut dy, &mut h)] {
            let cut = (-*s).max(-*d).max(0);
            *s += cut;
            *d += cut;
            *n -= cut;
        }
        w = w.min(img.width as i32 - sx).min(self.width as i32 - dx);
        h = h.min(img.height as i32 - sy).min(self.height as i32 - dy);
        if w <= 0 || h <= 0 {
            return;
        }
        if !self.reference {
            if let Some(tex) = assets.argb.get(&self.selected).filter(|t| t.len() == img.width * img.height) {
                let routine = sgl::select_routine(self.mode, self.color, def.map_or(1, |d| d.flags));
                let key = def.and_then(|d| d.color_key).unwrap_or(0);
                self.blit_fast(tex, img.width, routine, key, (sx as usize, sy as usize), (dx as usize, dy as usize), (w as usize, h as usize));
                return;
            }
        }
        let argb = |p: &[u8]| u32::from_le_bytes([p[2], p[1], p[0], p[3]]);
        let (wu, hu) = (w as usize, h as usize);
        let mut texels = Vec::with_capacity(wu * hu);
        let mut region = Vec::with_capacity(wu * hu);
        for y in 0..hu {
            let t = ((sy as usize + y) * img.width + sx as usize) * 4;
            texels.extend(img.rgba[t..t + 4 * wu].chunks_exact(4).map(argb));
            let o = ((dy as usize + y) * self.width + dx as usize) * 4;
            region.extend(self.rgba[o..o + 4 * wu].chunks_exact(4).map(argb));
        }
        let tex = sgl::Texture {
            pixels: &texels,
            pitch: wu,
            key: def.and_then(|d| d.color_key).unwrap_or(0),
            flags: def.map_or(1, |d| d.flags),
        };
        let st = sgl::BlitState { mode: self.mode, color: self.color, clip: None };
        sgl::draw_bitmap_rect(&mut sgl::Surface { pixels: &mut region, pitch: wu }, &tex, &st, (0, 0), (0, 0), (w, h));
        for y in 0..hu {
            let o = ((dy as usize + y) * self.width + dx as usize) * 4;
            for (k, v) in region[y * wu..(y + 1) * wu].iter().enumerate() {
                let [b, g, r, _] = v.to_le_bytes();
                self.rgba[o + 4 * k..o + 4 * k + 4].copy_from_slice(&[r, g, b, 255]);
            }
        }
    }

    /// Lo mismo que el camino de referencia de [`Canvas::blit_now`] (que convierte textura y destino a `u32`, llama a
    /// [`p3_render::sgl::draw_bitmap_rect`] y vuelve a convertir poniendo alfa 255 en todo el rectángulo), sobre la textura
    /// ya convertida y escribiendo en la imagen: copia directa en las rutinas 0 y 1, la mezcla de `0x0508DEBA` en las 2 a 4
    /// (sin tocar el píxel si el alfa de textura × color es 0) y [`p3_render::sgl::blend_pixel`] en las demás. El rectángulo
    /// llega ya recortado.
    #[allow(clippy::too_many_arguments)]
    fn blit_fast(&mut self, tex: &[u32], pitch: usize, routine: u8, key: u32, (sx, sy): (usize, usize), (dx, dy): (usize, usize), (w, h): (usize, usize)) {
        let color = self.color;
        let cb = [color & 0xFF, color >> 8 & 0xFF, color >> 16 & 0xFF, color >> 24].map(|v| v as u16);
        let key = key & 0xFF_FFFF;
        for y in 0..h {
            let src = &tex[(sy + y) * pitch + sx..][..w];
            let o = ((dy + y) * self.width + dx) * 4;
            let dst = self.rgba[o..o + 4 * w].chunks_exact_mut(4);
            match routine {
                0 => {
                    for (d, &s) in dst.zip(src) {
                        d.copy_from_slice(&[(s >> 16) as u8, (s >> 8) as u8, s as u8, 255]);
                    }
                }
                1 => {
                    for (d, &s) in dst.zip(src) {
                        if s != key {
                            d.copy_from_slice(&[(s >> 16) as u8, (s >> 8) as u8, s as u8, 255]);
                        } else {
                            d[3] = 255;
                        }
                    }
                }
                2..=4 => {
                    for (d, &s) in dst.zip(src) {
                        let a = ((s >> 24) as u16 * cb[3] >> 8) as i16;
                        if a != 0 {
                            // `d` es R, G, B; la textura y el color llevan B en el byte bajo.
                            for k in 0..3 {
                                let m = ((s >> (16 - 8 * k) & 0xFF) as u16 * cb[2 - k] >> 8) as i16;
                                let v = i16::from(d[k]);
                                d[k] = (m.wrapping_sub(v).wrapping_mul(a) >> 8).wrapping_add(v) as u8;
                            }
                        }
                        d[3] = 255;
                    }
                }
                _ => {
                    for (d, &s) in dst.zip(src) {
                        let old = u32::from_le_bytes([d[2], d[1], d[0], d[3]]);
                        if let Some(v) = p3_render::sgl::blend_pixel(routine, s, old, color, key) {
                            let [b, g, r, _] = v.to_le_bytes();
                            d[..3].copy_from_slice(&[r, g, b]);
                        }
                        d[3] = 255;
                    }
                }
            }
        }
    }

    fn fill_now(&mut self, x: i32, y: i32, w: i32, h: i32) {
        let [a, r, g, b] = self.color.to_be_bytes();
        for yy in 0..h.max(0) {
            for xx in 0..w.max(0) {
                self.put(x + xx, y + yy, [r, g, b, a]);
            }
        }
    }

    fn digits_now(&mut self, x: i32, y: i32, s: &str) {
        let [a, r, g, b] = self.color.to_be_bytes();
        for (k, ch) in s.bytes().enumerate() {
            let Some(d) = ch.checked_sub(b'0').and_then(|d| DIGITS.get(usize::from(d))) else { continue };
            for (row, bits) in d.iter().enumerate() {
                for col in 0..3 {
                    if bits & (4 >> col) != 0 {
                        self.put(x + 4 * k as i32 + col, y + row as i32, [r, g, b, a]);
                    }
                }
            }
        }
    }

    /// Diagonal de `draw_geometry` (`0x004BB390` → `sgl_DrawLine` de la DLL): rasterizador de `ddraw_Dll.dll`
    /// (`p3_render::sgl::draw_line`, paridad en emulación) con el modo y el color actuales y la vista como recorte. Se
    /// dibuja sobre una copia BGRA del rectángulo que ocupa la línea.
    fn line_now(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
        let r = |v: f32| v.round_ties_even() as i32;
        let (w, h) = (self.width as i32, self.height as i32);
        let (bx0, bx1) = (r(x1).min(r(x2)).max(0), r(x1).max(r(x2)).min(w - 1));
        let (by0, by1) = (r(y1).min(r(y2)).max(0), r(y1).max(r(y2)).min(h - 1));
        if bx0 > bx1 || by0 > by1 {
            return;
        }
        let (rw, rh) = ((bx1 - bx0 + 1) as usize, (by1 - by0 + 1) as usize);
        let mut region = vec![0u32; rw * rh];
        for y in 0..rh {
            for x in 0..rw {
                let o = ((by0 as usize + y) * self.width + bx0 as usize + x) * 4;
                let [r, g, b, a] = [self.rgba[o], self.rgba[o + 1], self.rgba[o + 2], self.rgba[o + 3]];
                region[y * rw + x] = u32::from_le_bytes([b, g, r, a]);
            }
        }
        let (fx, fy) = (bx0 as f32, by0 as f32);
        p3_render::sgl::draw_line(
            &mut p3_render::sgl::Surface { pixels: &mut region, pitch: rw },
            (x1 - fx, y1 - fy, x2 - fx, y2 - fy),
            self.mode,
            self.color,
            Some([-bx0, -by0, w - bx0, h - by0]),
        );
        for y in 0..rh {
            for x in 0..rw {
                let o = ((by0 as usize + y) * self.width + bx0 as usize + x) * 4;
                let [b, g, r, a] = region[y * rw + x].to_le_bytes();
                self.rgba[o..o + 4].copy_from_slice(&[r, g, b, a]);
            }
        }
    }
}

/// Datos de la partida que necesitan las piezas de la carta (`0x0044AB70`): ciudades, oficinas `+0x28CC`, banderas de
/// noticias `0x00535300`, imagen del jugador, selección `0x004885A0` y sus rutas.
#[derive(Default)]
pub struct FrameData {
    /// `0x006DE4B8`: identificador de la ciudad en cada posición.
    pub order: Vec<u8>,
    /// Tabla `0x006DDB90` por identificador.
    pub info: Vec<p3_render::navmap_draw::TownInfo>,
    /// Nombres por posición (`0x006DDA00`).
    pub names: Vec<String>,
    /// `+0x28CC` (`0x00548230`): 1 en las ciudades con oficina del jugador.
    pub offices: Vec<u8>,
    /// `0x00535300(i)` por posición.
    pub flags: Vec<u32>,
    /// Imagen del comerciante del jugador (`0x005303C0`).
    pub merchant: Vec<u8>,
    /// `merchant+0xE` de cada comerciante.
    pub first_ship: Vec<u16>,
    /// Barcos elegidos en el cliente.
    pub selection: Vec<u16>,
    /// `DAT_006DE4B4` (tick del reloj): hora de los contactos perdidos.
    pub tick: u32,
    /// Puntos de la ruta de cada barco elegido (`ship+0x24`).
    pub routes: std::collections::BTreeMap<u32, Vec<(i16, i16)>>,
}

/// Anfitrión de [`p3_render::navmap_draw::draw_nav_map`]: las primitivas van al [`Canvas`] y cada pieza con función propia
/// (rutas `0x0044B330`, símbolos `0x0044BCC0`, estado `0x0044BFD0`, barcos en puerto `0x0044C3B0`, icebergs `0x0044B630`,
/// contactos perdidos `0x004530B0`, barcos `0x0044CBF0` y tiempo `0x0044B970`) se dibuja en su sitio del orden original con
/// los datos de [`FrameData`]. Los rótulos (`0x00452600`) los pone el cliente con los nombres de `StadtDaten.ini`.
struct FrameHost<'c, 'a, 'w> {
    canvas: &'c mut Canvas<'a>,
    obj: &'c mut [u8],
    world: &'c p3_render::navmap::World<'w>,
    tex: &'c Textures,
    list: &'c [u32],
    clock: u32,
    contacts: &'c mut p3_render::navmap_draw::Contacts,
    data: &'c FrameData,
    routes: &'c mut p3_render::navmap_draw::RouteCache,
    assets: &'c NavAssets,
    /// Capa fija guardada: se cierra al llegar a los icebergs, la primera pieza que cambia de un fotograma a otro.
    layer: &'c mut LayerCache,
}

impl NavHost for FrameHost<'_, '_, '_> {
    fn color(&mut self, argb: u32) {
        self.canvas.color(argb)
    }
    fn select(&mut self, handle: u32) {
        self.canvas.select(handle)
    }
    fn blit(&mut self, sx: i32, sy: i32, dx: i32, dy: i32, w: i32, h: i32) {
        self.canvas.blit(sx, sy, dx, dy, w, h)
    }
    fn fill(&mut self, x: i32, y: i32, w: i32, h: i32) {
        self.canvas.fill(x, y, w, h)
    }
    fn text(&mut self, x: i32, y: i32, s: &str) {
        self.canvas.text(x, y, s)
    }
    fn size(&mut self, handle: u32) -> (i32, i32) {
        self.canvas.size(handle)
    }
    fn ambient(&mut self, class: u32, id: u32, x: i32, y: i32, extra: u32) {
        self.canvas.ambient(class, id, x, y, extra)
    }
    fn merchant_word8(&mut self, m: u8) -> u16 {
        self.canvas.merchant_word8(m)
    }
    fn rand(&mut self) -> u32 {
        self.canvas.rand()
    }
    fn set_color(&mut self, argb: u32) -> u32 {
        self.canvas.set_color(argb)
    }
    fn set_texture(&mut self, handle: u32) -> u32 {
        self.canvas.set_texture(handle)
    }
    fn blend_mode(&mut self, mode: i32) -> i32 {
        self.canvas.blend_mode(mode)
    }
    fn line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
        self.canvas.line(x1, y1, x2, y2)
    }
    fn font(&mut self, font: u32) {
        self.canvas.font(font)
    }
    fn text_mode(&mut self, mode: i32) {
        self.canvas.text_mode(mode)
    }
    fn text_width(&mut self, font: u32, s: &str) -> i32 {
        self.canvas.text_width(font, s)
    }
    fn texture_info(&mut self, handle: u32) -> [i32; 3] {
        self.canvas.texture_info(handle)
    }
    fn part(&mut self, part: p3_render::navmap_draw::NavPart, clip: [i32; 4]) {
        use p3_render::navmap as nm;
        use p3_render::navmap_draw::{self as nd, NavPart};
        match part {
            NavPart::Icebergs => {
                self.canvas.end_layer(self.layer);
                nd::draw_iceberg_icons(self.obj, self.tex, self.clock, &mut nd::Carry::default(), self.canvas)
            }
            NavPart::Fading => {
                let mut carry = nd::FadeCarry::default();
                let d = self.data;
                nd::draw_fading_contacts(self.obj, self.world, self.tex, self.contacts, &d.selection, d.tick, &mut carry, self.canvas)
            }
            NavPart::Ships => nm::draw_ships_at_sea(self.obj, self.world, self.tex, self.list, &self.data.selection, self.canvas),
            NavPart::Weather => nm::draw_weather_icons(self.obj, self.tex, self.clock, self.canvas),
            NavPart::Routes => {
                let d = self.data;
                let route = |id: u32| d.routes.get(&id).cloned();
                nd::draw_routes(self.obj, self.world, &d.selection, &route, self.routes, (clip[0], clip[1]), self.canvas)
            }
            NavPart::Markers => {
                let d = self.data;
                let towns = nd::Towns { order: &d.order, info: &d.info, names: &d.names };
                let merchant = |_m: u32| d.merchant.clone();
                let tex = self.assets.tex(TOWN_BUTTON);
                nd::draw_town_markers(self.obj, &towns, &tex, self.world.player, &d.offices, &merchant, &mut nd::Carry::default(), self.canvas)
            }
            NavPart::Status => {
                let d = self.data;
                let towns = nd::Towns { order: &d.order, info: &d.info, names: &d.names };
                let mut flags = |i: u32| d.flags.get(i as usize).copied().unwrap_or(0);
                let (icons, levels) = (self.assets.tex(EVENTS), self.assets.tex(SHORTAGE));
                nd::draw_town_status_icons(self.obj, &towns, &icons, &levels, &mut flags, &mut nd::Carry::default(), self.canvas)
            }
            NavPart::InPort => {
                let d = self.data;
                let towns = nd::Towns { order: &d.order, info: &d.info, names: &d.names };
                let first = |m: u32| d.first_ship.get(m as usize).copied().unwrap_or(0xFFFF);
                let (port, icons) = (self.assets.tex(nd::PORT_TEXTURE), self.assets.tex(EVENTS));
                let mut host = InPortHost { canvas: self.canvas, offices: &d.offices };
                nd::draw_ships_in_port(self.obj, self.world, &towns, &port, &icons, &first, &mut nd::Carry::default(), &mut host)
            }
            NavPart::Labels => {
                let d = self.data;
                let towns = nd::Towns { order: &d.order, info: &d.info, names: &d.names };
                nd::draw_town_labels(self.obj, &towns, self.canvas)
            }
        }
    }
}

/// [`Canvas`] más la consulta de oficina `0x005308A0` del jugador (la única que hace `0x0044C3B0`).
struct InPortHost<'c, 'a> {
    canvas: &'c mut Canvas<'a>,
    offices: &'c [u8],
}

impl NavHost for InPortHost<'_, '_> {
    fn color(&mut self, argb: u32) {
        self.canvas.color(argb)
    }
    fn select(&mut self, handle: u32) {
        self.canvas.select(handle)
    }
    fn blit(&mut self, sx: i32, sy: i32, dx: i32, dy: i32, w: i32, h: i32) {
        self.canvas.blit(sx, sy, dx, dy, w, h)
    }
    fn text(&mut self, x: i32, y: i32, s: &str) {
        self.canvas.text(x, y, s)
    }
    fn text_width(&mut self, font: u32, s: &str) -> i32 {
        self.canvas.text_width(font, s)
    }
    fn size(&mut self, handle: u32) -> (i32, i32) {
        self.canvas.size(handle)
    }
    fn merchant_word8(&mut self, m: u8) -> u16 {
        self.canvas.merchant_word8(m)
    }
    fn office(&mut self, _merchant: u32, town: u32) -> bool {
        self.offices.get(town as usize).is_some_and(|&b| b != 0)
    }
    fn font(&mut self, font: u32) {
        self.canvas.font(font)
    }
    fn text_mode(&mut self, mode: i32) {
        self.canvas.text_mode(mode)
    }
}

/// Estado persistente de la carta entre fotogramas.
pub struct NavState {
    pub assets: NavAssets,
    /// Imagen naval (`manager`, barcos, convoyes) y tick del reloj en que se tomó; se rehace al avanzar si no hay navegación viva.
    cache: Option<(u32, p3_save::ships::NavigationSnapshot)>,
    rand_state: u32,
    started: bool,
    /// Contactos que se desvanecen (`0x004530B0`); los altas (`0x00452F60`) llegarán con los avistamientos de C10.
    contacts: p3_render::navmap_draw::Contacts,
    /// Rutas suavizadas por barco (`0x00448D80`, caché `+0x28A4`).
    routes: p3_render::navmap_draw::RouteCache,
    /// Contactos `ship+0x50`/`+0x52` de los barcos del jugador en el fotograma anterior (enganche de los avistamientos).
    sightings: std::collections::BTreeMap<u16, [u16; 2]>,
    /// Pulsos del minimapa (`0x0046F340`).
    pulses: p3_core::ships::spotting::MinimapPulses,
    /// Planificador propio para reconstruir rutas compartidas cuando la simulación no tiene uno vivo.
    planner: Option<p3_core::route::finder::RoutePlanner>,
    /// Capa fija del fotograma anterior ([`LayerCache`]).
    layer: LayerCache,
}

/// Entrada de un fotograma de [`crate::Session::navmap_frame`].
#[derive(Clone, Debug, Default)]
pub struct NavRequest {
    /// Tamaño de la vista (`+0x2C`, `+0x30`).
    pub width: i32,
    pub height: i32,
    /// Reloj del cliente en ms (`DAT_006DCCF0`).
    pub clock_ms: u32,
    /// Ratón dentro de la vista (desplazamiento por los bordes de `0x0044DCE0`), si está encima.
    pub mouse: Option<(i32, i32)>,
    /// Teclas pulsadas desde el fotograma anterior: izquierda, arriba, derecha, abajo (bit 0 de `0x006DCBE5..E8`).
    pub keys: [bool; 4],
    /// Centrar en un punto del mapa (`0x0044F490`): doble clic en el cliente o el minimapa.
    pub center: Option<(i32, i32)>,
    /// Ticks de simulación a avanzar antes de dibujar (256 por día).
    pub ticks: u32,
    /// Barcos elegidos en el cliente (lista `0x004885A0`): sus rutas se dibujan (`0x0044B330`).
    pub selection: Vec<u16>,
}

fn snapshot_of(session: &crate::Session) -> Result<p3_save::ships::NavigationSnapshot, String> {
    let saved = session.save()?;
    let parsed = p3_save::Save::parse(&saved).map_err(|e| format!("guardado: {e}"))?;
    let ships = p3_save::ships::Ships::parse(parsed.block_named("ships").ok_or("bloque naval ausente")?)
        .map_err(|e| format!("barcos: {e:?}"))?;
    ships.navigation_snapshot().map_err(str::to_string)
}

/// Datos de la partida para las piezas de la carta: ciudades de `StadtDaten.ini` en el orden `0x006DE4B8`, oficinas del
/// jugador (`0x00548230`: recorre `merchant+0xC` por el enlace `+0x2C8` y marca la ciudad `+0x2C6`), banderas de
/// noticias ([`p3_core::economy::c08_town_info::known_flags`], `0x00535300`) y rutas de los barcos elegidos.
fn frame_data(
    sim: &mut p3_sim::Sim,
    owner: usize,
    stadt: &str,
    snap: &p3_save::ships::NavigationSnapshot,
    selection: &[u16],
    own_planner: &mut Option<p3_core::route::finder::RoutePlanner>,
    errors: &mut Vec<String>,
) -> Result<FrameData, String> {
    let parsed = crate::world::parse_towns(stadt);
    let n = sim.economy.world.towns.len();
    let order: Vec<u8> = if sim.host.expedition_town_ids.len() >= n {
        sim.host.expedition_town_ids[..n].to_vec()
    } else {
        (0..n as u8).collect()
    };
    let mut info = vec![p3_render::navmap_draw::TownInfo::default(); parsed.iter().map(|t| t.id + 1).max().unwrap_or(0)];
    for t in &parsed {
        info[t.id] = p3_render::navmap_draw::TownInfo {
            symbol: t.symbol,
            label: t.label,
            event: t.event,
            harbor: t.harbor,
            approach: t.approach,
            river: u8::from(t.river),
            align: t.align,
        };
    }
    let names = order
        .iter()
        .map(|&id| parsed.iter().find(|t| t.id == usize::from(id)).map(|t| t.name.clone()).unwrap_or_default())
        .collect();
    let mut offices = vec![0u8; n];
    if owner < usize::from(sim.idle.clock.merchant_count) {
        if let (Some(m), Some(pool)) = (sim.rep_merchants.get(owner), sim.host.office_pool.as_ref()) {
            let mut at = usize::from(m.u16(0xC));
            let mut steps = 0;
            while at < pool.records.len() && steps <= pool.records.len() {
                let r = &pool.records[at];
                if r.len() < 0x2CA {
                    break;
                }
                if let Some(b) = offices.get_mut(usize::from(r[0x2C6])) {
                    *b = 1;
                }
                at = usize::from(u16::from_le_bytes([r[0x2C8], r[0x2C9]]));
                steps += 1;
            }
        }
    }
    let tick = sim.idle.clock.tick;
    let flags = match sim.host.town_history.as_ref() {
        Some(h) => (0..n)
            .map(|i| {
                let mask = sim.economy.world.towns[i].u32(0x2C8);
                p3_core::economy::c08_town_info::known_flags(h, mask, tick, owner as u32, i as u32).unwrap_or_else(|e| {
                    errors.push(format!("noticias de la ciudad {i}: {e}"));
                    0
                })
            })
            .collect(),
        None => vec![0; n],
    };
    let merchant = sim.rep_merchants.get(owner).map(|m| m.bytes.to_vec()).unwrap_or_default();
    let first_ship = sim.rep_merchants.iter().map(|m| m.u16(0xE)).collect();
    // Rutas: la del viaje vivo si lo hay; si no, la guardada (propia o compartida, reconstruida con el planificador).
    let mut routes = std::collections::BTreeMap::new();
    let mut wanted: Vec<u32> = Vec::new();
    for &sel in selection {
        let at = usize::from(sel) * 0x180;
        let Some(s) = snap.ships.get(at..at + 0x180) else { continue };
        let conv = u16::from_le_bytes([s[8], s[9]]);
        let id = snap
            .convoys
            .get(usize::from(conv) * 0x3C..usize::from(conv) * 0x3C + 0x3C)
            .map_or(u32::from(sel), |c| u32::from(u16::from_le_bytes([c[0x10], c[0x11]])));
        wanted.push(id);
    }
    for id in wanted {
        if let Some(p) = sim.host.navigation.as_ref().and_then(|v| v.route_for(id as u16)) {
            routes.insert(id, p.to_vec());
            continue;
        }
        match snap.routes.get(id as usize).and_then(Option::as_ref) {
            Some(p3_save::ships::SavedRoute::Owned(p)) => {
                routes.insert(id, p.clone());
            }
            Some(p3_save::ships::SavedRoute::Shared { origin, destination }) => {
                let ports = sim.host.navigation.as_ref().map(|v| v.ports.clone()).unwrap_or_default();
                let (Some(&a), Some(&b)) = (ports.get(usize::from(*origin)), ports.get(usize::from(*destination))) else { continue };
                // Sin planificador vivo se construye uno propio de la carta (como `fleet_resume`), sin tocar la simulación.
                if sim.host.navigation_planner.is_none() && own_planner.is_none() {
                    if let Some(f) = sim.host.foundation_navigation.as_ref() {
                        match f.build_planner(&sim.host.expedition_town_ids, 100_000) {
                            Ok(p) => *own_planner = Some(p),
                            Err(e) => errors.push(format!("planificador de la carta: {e}")),
                        }
                    }
                }
                if let Some(planner) = sim.host.navigation_planner.as_mut().or(own_planner.as_mut()) {
                    match planner.find_route(&[a, b]).route {
                        p3_core::route::finder::RouteOut::Shared(k) => {
                            if let Some(p) = planner.routes.get(k) {
                                routes.insert(id, p.clone());
                            }
                        }
                        p3_core::route::finder::RouteOut::Built(p) => {
                            routes.insert(id, p);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    Ok(FrameData { order, info, names, offices, flags, merchant, first_ship, selection: selection.to_vec(), tick, routes })
}

/// Avisos de [`p3_core::ships::spotting`] recogidos para la carta.
#[derive(Default)]
pub struct SpotLog {
    /// Altas de contactos perdidos (`0x00452F60`).
    pub lost: Vec<u16>,
    /// Pulsos de la carta (`0x0046F340`, mundo): `navmap_frame` los pasa a [`p3_core::ships::spotting::MinimapPulses`] y dibuja el minimapa.
    pub pulses: Vec<(u16, u16)>,
    /// Sonidos de avistamiento `0x00459790(2, barco, x, y, 0)`.
    pub sounds: Vec<(u16, i32, i32)>,
}
impl p3_core::ships::spotting::SpotHost for SpotLog {
    fn lost(&mut self, ship: u16) {
        self.lost.push(ship);
    }
    fn pulse(&mut self, x: u16, y: u16) {
        self.pulses.push((x, y));
    }
    fn sound(&mut self, ship: u16, x: i32, y: i32) {
        self.sounds.push((ship, x, y));
    }
}

/// Avistamientos de la carta: `update_spotting_for_human_merchants` 0x0050DC50 (paridad x86, C10) sobre la imagen naval que
/// se dibuja. El original la ejecuta dos veces por paquete de reloj `0xC4` (`execute_operations`, 0x00546A43 y 0x00546A87)
/// y escribe `ship+0x50/+0x52` en la tabla de barcos guardada; la simulación sin pantalla (y el oráculo x86 sin interfaz) no la
/// ejecuta porque falta `[0x6CBDD8]`/`[0x6CC940]`. Aquí corre una vez por fotograma de la carta y sus ranuras viven en `slots`
/// (estado de la carta, se aplican antes de cada pasada), así que la simulación no cambia. **Supuestos**: una pasada por
/// fotograma en lugar de dos por tick y partida de un solo jugador (`[0x6DF2F0] & 2` = 0).
pub fn spot_frame(snap: &mut p3_save::ships::NavigationSnapshot, merchants: &[p3_core::ai::reputation::Merchant], player: usize, slots: &mut std::collections::BTreeMap<u16, [u16; 2]>) -> Result<SpotLog, String> {
    use p3_core::ships::spotting as sp;
    sp::check_preconditions(&snap.manager, &snap.ships, &snap.convoys).map_err(|e| format!("avistamientos: {e}"))?;
    for (&i, v) in slots.iter() {
        let at = usize::from(i) * 0x180;
        if let Some(s) = snap.ships.get_mut(at + 0x50..at + 0x54) {
            s[..2].copy_from_slice(&v[0].to_le_bytes());
            s[2..].copy_from_slice(&v[1].to_le_bytes());
        }
    }
    let list: Vec<sp::SpotMerchant> = merchants.iter().map(|m| sp::SpotMerchant { kind: m.u16(8), sound: m.bytes.get(0x3B).is_some_and(|&b| b != 0), first_ship: m.u16(0xE) }).collect();
    let g = sp::SpotGlobals { ui: true, all_humans: false, player: player as u32 };
    let mut log = SpotLog::default();
    sp::update_spotting_for_human_merchants(&mut snap.manager, &mut snap.ships, &snap.convoys, &list, &g, &mut log);
    slots.clear();
    for (i, s) in snap.ships.chunks_exact(0x180).enumerate() {
        let v = [u16::from_le_bytes([s[0x50], s[0x51]]), u16::from_le_bytes([s[0x52], s[0x53]])];
        if v != [0xFFFF; 2] {
            slots.insert(i as u16, v);
        }
    }
    Ok(log)
}

impl crate::Session {
    /// Fotogramas de la carta en que se reutilizó la capa fija guardada y en que hubo que redibujarla.
    pub fn navmap_layer_hits(&self) -> (u64, u64) {
        self.navmap.as_ref().map_or((0, 0), |s| (s.layer.hits, s.layer.misses))
    }

    /// Un fotograma de la carta náutica (`nav_map_update` 0x0044DCE0): desplazamiento con teclas/bordes (paridad x86), entidades
    /// de tiempo del gestor naval (`0x459380`, con su sonido de clase 3), lista de barcos visibles (`0x004516B0`, sonido de clase 4),
    /// fondo, tormentas/calmas (`0x0044B970`) y barcos en el mar con su flecha (`0x0044CBF0`). Escribe la vista en `path` (PPM) y
    /// devuelve desplazamiento, minimapa, barcos y ciudades visibles, y los sonidos (`{sounds, adjust, stop_all}` como `town_sounds`).
    ///
    /// **Avistamientos (C10)**: la carta dibuja los barcos que la lista de `0x004516B0` admite, que lee `ship+0x50`/`+0x52` de los
    /// barcos propios. Cuando el tick naval de C10 escriba esos contactos en la imagen naval, aparecerán aquí sin más cambios.
    pub fn navmap_frame(&mut self, req: &NavRequest, path: &std::path::Path) -> Result<serde_json::Value, String> {
        let (v, canvas) = self.navmap_frame_rgba(req, path.parent().or(Some(std::path::Path::new("."))))?;
        std::fs::write(path, ppm_bytes(canvas.0, canvas.1, &canvas.2)).map_err(|e| format!("carta: {e}"))?;
        Ok(v)
    }
    /// `navmap_frame` sin fichero: devuelve el fotograma en RGBA (`ancho`, `alto`, píxeles). Con `sound_dir = None` no escribe los
    /// `.wav` de los sonidos pedidos (los anota igual en la respuesta). Lo usa el cliente nativo (`p3-client`).
    pub fn navmap_frame_rgba(&mut self, req: &NavRequest, sound_dir: Option<&std::path::Path>) -> Result<(serde_json::Value, (usize, usize, Vec<u8>)), String> {
        use p3_render::navmap as nm;
        use serde_json::json;
        let mut errors = Vec::new();
        if req.ticks > 0 {
            let r = self.advance_ticks(req.ticks);
            errors.extend(r["errors"].as_array().into_iter().flatten().filter_map(|e| e.as_str().map(str::to_string)));
        }
        let archive = Archive::parse(&self.archive).map_err(|e| format!("CPR: {e}"))?;
        if self.navmap.is_none() {
            self.navmap = Some(NavState { assets: NavAssets::load(&archive)?, cache: None, rand_state: 1, started: false, contacts: Default::default(), routes: Default::default(), planner: None, sightings: Default::default(), pulses: Default::default(), layer: Default::default() });
        }
        // Imagen naval: la viva si hay navegación activa; si no, la del guardado, rehecha cuando avanza el reloj.
        let tick = self.sim.idle.clock.tick;
        let live = self.sim.host.navigation.as_ref().map(|n| n.snapshot.clone());
        let mut snap = match live {
            Some(s) => s,
            None => {
                let stale = self.navmap.as_ref().unwrap().cache.as_ref().is_none_or(|(t, _)| *t != tick);
                if stale {
                    match snapshot_of(self) {
                        Ok(s) => self.navmap.as_mut().unwrap().cache = Some((tick, s)),
                        Err(e) => errors.push(format!("imagen naval: {e}")),
                    }
                }
                self.navmap.as_ref().unwrap().cache.as_ref().map(|(_, s)| s.clone()).ok_or("sin imagen naval")?
            }
        };
        let spots = {
            let state = self.navmap.as_mut().unwrap();
            spot_frame(&mut snap, &self.sim.rep_merchants, self.owner, &mut state.sightings).unwrap_or_else(|e| {
                errors.push(e);
                SpotLog::default()
            })
        };
        let stadt = archive.find(b"scripts/StadtDaten.ini").and_then(|e| archive.read(e)).ok_or("falta scripts/StadtDaten.ini")?;
        let stadt = String::from_utf8_lossy(stadt).into_owned();
        let mut planner = self.navmap.as_mut().unwrap().planner.take();
        let data = frame_data(&mut self.sim, self.owner, &stadt, &snap, &req.selection, &mut planner, &mut errors)?;
        self.navmap.as_mut().unwrap().planner = planner;
        {
            let state = self.navmap.as_mut().unwrap();
            for &ship in &spots.lost {
                p3_render::navmap_draw::add_lost_contact(&mut state.contacts, u32::from(ship), data.tick, true);
            }
        }
        // Pulsos del minimapa: altas de los avistamientos y un repintado (dibujo + envejecer).
        let pulse_draw = {
            let state = self.navmap.as_mut().unwrap();
            for &(x, y) in &spots.pulses {
                state.pulses.add(x, y, true, 0);
            }
            state.pulses.paint()
        };
        let word8: Vec<u16> = self.sim.rep_merchants.iter().map(|m| m.u16(8)).collect();
        let first_ship = self.sim.rep_merchants.get(self.owner).map_or(0xFFFF, |m| m.u16(0xE));
        let (owner, effects) = (self.owner, self.effects_volume);
        let mut board = match self.sound_board.take() {
            Some(b) => b,
            None => crate::audio::SoundBoard::new(crate::audio::SoundCatalog::from_archive(&archive)?, Default::default()),
        };
        board.map_volume = effects;
        board.set_clock(req.clock_ms);
        let state = self.navmap.as_mut().unwrap();
        let (w, h) = (req.width.clamp(64, 2400), req.height.clamp(64, 1800));
        let mut canvas = Canvas::new(&state.assets, w as usize, h as usize);
        canvas.rand_state = state.rand_state;
        canvas.word8 = word8;
        canvas.reference = self.navmap_reference;
        let obj = &mut board.map.mem;
        nm::w32(obj, 0x14, 0);
        nm::w32(obj, 0x18, 0);
        let resized = nm::r32(obj, 0x2C) != w as u32 || nm::r32(obj, 0x30) != h as u32;
        nm::w32(obj, 0x2C, w as u32);
        nm::w32(obj, 0x30, h as u32);
        if !state.started {
            // `0x00449931` (periodo), `0x0044957E` (desplazamiento inicial) y la altura de la flecha (`+0x2B0C`).
            nm::w32(obj, 0x2918, 1500);
            nm::w32(obj, 0x2B0C, 10);
            nm::w32(obj, 0x2AFC, ICEBERG);
            // Color de las rutas (`0x00449959`; `0x0044DCE0` lo aclara a 0xFFA6B9C7 con el cursor encima).
            nm::w32(obj, 0x2920, 0xFF68_7888);
            nm::w16(obj, 0x29E4, 810);
            nm::w16(obj, 0x29E6, 950);
            nm::w32(obj, 0x290C, req.clock_ms);
            state.started = true;
            nm::clamp_scroll(obj, Some(BACKGROUND), &mut canvas);
        } else if resized {
            nm::clamp_scroll(obj, Some(BACKGROUND), &mut canvas);
        }
        if let Some((x, y)) = req.center {
            nm::center_on_point(obj, x, y, Some(BACKGROUND), &mut canvas);
        }
        let k = |b: bool| u8::from(b);
        let mut input = nm::ScrollInput {
            active: true,
            mouse: req.mouse.unwrap_or((w / 2, h / 2)),
            screen: (w, h),
            clock: req.clock_ms,
            keys: [k(req.keys[0]), k(req.keys[1]), k(req.keys[2]), k(req.keys[3])],
            background: BACKGROUND,
        };
        let out = nm::scroll_frame(obj, &mut input, &mut canvas);
        nm::finish_scroll(obj, &out, &mut canvas);
        nm::w32(obj, 0x290C, req.clock_ms);
        let (sx, sy) = (nm::r16(obj, 0x29E4) as i16 as i32, nm::r16(obj, 0x29E6) as i16 as i32);
        // Tiempo: `0x459380` prepara los iconos (+0x2A88…) y pide su sonido.
        let mut played = board.map_weather(&archive, &snap.manager);
        for &(ship, x, y) in &spots.sounds {
            played.extend(board.map_ambient(&archive, 2, u32::from(ship), x, y, 0));
        }
        let tex = state.assets.textures();
        let world = nm::World {
            ships: &snap.ships,
            ship_count: (snap.ships.len() / 0x180) as u16,
            convoys: &snap.convoys,
            convoy_count: (snap.convoys.len() / 0x3C) as u16,
            merchant_count: canvas.word8.len() as u16,
            player: owner as u32,
            first_ship,
        };
        let obj = &mut board.map.mem;
        let list = nm::visible_ships(obj, &world, &tex, (true, true), &mut canvas);
        // `nav_map_draw(0x0044AB70)` en su orden: fondo (fuera del mapa en negro), marcas, piezas, barcos y tiempo.
        // Desde aquí hasta los icebergs las órdenes de dibujo se graban: si repiten las del fotograma anterior se copia la
        // capa guardada (`LayerCache`).
        canvas.begin_layer();
        canvas.background(sx, sy);
        let snapshot = obj.clone();
        let args = p3_render::navmap_draw::FrameArgs { screen: (w, h), check: p3_render::navmap_draw::LCG_FIRST, ..Default::default() };
        let mut host = FrameHost {
            canvas: &mut canvas,
            obj,
            world: &world,
            tex: &tex,
            list: &list,
            clock: req.clock_ms,
            contacts: &mut state.contacts,
            data: &data,
            routes: &mut state.routes,
            assets: &state.assets,
            layer: &mut state.layer,
        };
        p3_render::navmap_draw::draw_nav_map(&snapshot, &tex, &args, &[], &[], &mut host);
        host.canvas.end_layer(host.layer);
        let obj = host.obj;
        let minimap = nm::minimap_rect(obj);
        let weather: Vec<_> = (0..16)
            .filter(|&i| nm::r8(obj, 0x2A88 + i) < 0x10)
            .map(|i| json!({"kind": nm::r8(obj, 0x2A88 + i), "x": nm::r16(obj, 0x2AB8 + 4 * i) as i16, "y": nm::r16(obj, 0x2ABA + 4 * i) as i16}))
            .collect();
        canvas.minimap_inset(minimap, &pulse_draw);
        state.rand_state = canvas.rand_state;
        let ambient = std::mem::take(&mut canvas.ambient);
        let pixels = (canvas.width, canvas.height, std::mem::take(&mut canvas.rgba));
        for (class, id, x, y, extra) in ambient {
            played.extend(board.map_ambient(&archive, class, id, x, y, extra));
        }
        let adjust = board.map_frame();
        let stop_all = std::mem::take(&mut board.stopped);
        let mut sounds = Vec::new();
        for r in played {
            let name = format!("carta_{}_{}.wav", req.clock_ms, r.id);
            if let Some(dir) = sound_dir {
                let pcm = board.catalog.render(&archive, r.id)?;
                std::fs::write(dir.join(&name), pcm.to_wav()).map_err(|e| format!("sonido: {e}"))?;
            }
            sounds.push(json!({"file": name, "id": r.id, "volume": r.volume, "pan": r.pan}));
        }
        let adjust: Vec<_> = adjust.iter().map(|r| json!({"id": r.id, "volume": r.volume, "pan": r.pan})).collect();
        self.sound_board = Some(board);
        let mut seen = std::collections::BTreeSet::new();
        let ships: Vec<_> = list
            .iter()
            .filter(|&&i| seen.insert(i))
            .filter_map(|&i| {
                let s = snap.ships.get(i as usize * 0x180..i as usize * 0x180 + 0x180)?;
                let x = i32::from(i16::from_le_bytes([s[0x1E], s[0x1F]]));
                let y = i32::from(i16::from_le_bytes([s[0x22], s[0x23]]));
                Some(json!({"id": i, "owner": s[0], "own": usize::from(s[0]) == owner, "x": x - sx, "y": y - sy, "kind": s[0xE], "heading": s[1]}))
            })
            .collect();
        let ini = archive.find(b"scripts/StadtDaten.ini").and_then(|e| archive.read(e)).ok_or("falta scripts/StadtDaten.ini")?;
        let towns = crate::world::parse_towns(&String::from_utf8_lossy(ini));
        let ids = &self.sim.host.expedition_town_ids;
        let cities: Vec<_> = towns
            .iter()
            .filter(|t| ids.iter().any(|id| usize::from(*id) == t.id))
            .filter(|t| (sx - 200..sx + w + 200).contains(&t.symbol.0) && (sy - 50..sy + h + 50).contains(&t.symbol.1))
            .map(|t| {
                json!({"id": t.id, "name": t.name, "x": t.symbol.0 - sx, "y": t.symbol.1 - sy,
                       "label_x": t.label.0 - sx, "label_y": t.label.1 - sy, "label_right": t.label_right})
            })
            .collect();
        Ok((json!({
            "scroll": [sx, sy], "size": [w, h], "minimap": minimap, "map_size": [2400, 1800],
            "tick": self.sim.idle.clock.tick, "day": self.sim.idle.clock.tick / 256,
            "ships": ships, "cities": cities, "weather": weather, "live_navigation": self.sim.host.navigation.is_some(),
            "sounds": sounds, "adjust": adjust, "stop_all": stop_all, "errors": errors,
            "sightings": {"lost": spots.lost, "pulses": spots.pulses, "minimap_pulses": pulse_draw},
        }), pixels))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_texture_sections() {
        let ini = "[TEX30039]\r\nTextureFile=images/Animationen_Seekarte/Pfeil01.bmp\r\nUseAlpha=false\r\nCount=1\r\nOffsetNSize0=0 0 24 18\r\nUseColorKey=true\r\nColorKey=0xffff00ff\r\n\r\n[TEX30001]\r\nTextureFile=images/x.tga\r\nCount=2\r\nOffsetNSize1=64 0\t64 59\r\nOffsetNSize0=0 0\t64 59\r\n";
        let d = parse_textures(ini);
        assert_eq!(d[&30039], TexDef { file: "images/Animationen_Seekarte/Pfeil01.bmp".into(), frames: vec![[0, 0, 24, 18]], color_key: Some(0xFFFF_00FF), flags: 7 | 1 });
        assert_eq!(d[&30001].frames, vec![[0, 0, 64, 59], [64, 0, 64, 59]]);
        assert_eq!(d[&30001].color_key, None);
    }

    /// Texturas sintéticas con todas las combinaciones de indicadores que cambian de rutina, alfas variados (0 y 255
    /// incluidos) y píxeles del color clave; el fondo es menor que la vista para que haya zona negra.
    fn synthetic_assets() -> NavAssets {
        let mut seed = 0x1234_5678u32;
        let mut next = move || {
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            (seed >> 16) as u8
        };
        let (mut defs, mut images) = (HashMap::new(), HashMap::new());
        let flag_sets = [1u32, 7 | 1, 3, 0x8001, 0, 1 | 2 << 24];
        for (i, &flags) in flag_sets.iter().enumerate() {
            let (width, height) = (37 + 5 * i, 29 + 3 * i);
            let color_key = (flags & 7 == 7).then_some(0xFFFF_00FF);
            let mut rgba = Vec::with_capacity(width * height * 4);
            for _ in 0..width * height {
                let a = match next() % 4 {
                    0 => 0,
                    1 => 255,
                    _ => next(),
                };
                if color_key.is_some() && next() % 3 == 0 {
                    rgba.extend_from_slice(&[255, 0, 255, 0]);
                } else {
                    rgba.extend_from_slice(&[next(), next(), next(), a]);
                }
            }
            defs.insert(100 + i as u32, TexDef { file: String::new(), frames: vec![], color_key, flags });
            images.insert(100 + i as u32, Image { width, height, rgba });
        }
        let (width, height) = (90, 70);
        let rgba = (0..width * height * 4).map(|k| if k % 4 == 3 { 255 } else { next() }).collect();
        defs.insert(BACKGROUND, TexDef { file: String::new(), frames: vec![], color_key: None, flags: 1 });
        images.insert(BACKGROUND, Image { width, height, rgba });
        let mut assets = NavAssets { defs, images, fonts: HashMap::new(), argb: HashMap::new() };
        assets.convert();
        assets
    }

    /// Guion de órdenes pseudoaleatorias: fondo, copias con todos los modos y colores (blanco opaco, blanco translúcido,
    /// opaco de color y cualquiera) y rectángulos que se salen de la textura y de la vista, rellenos, cifras y líneas.
    fn random_ops(canvas: &mut Canvas, seed: u32, n: usize) {
        let mut seed = seed;
        let mut next = move |m: u32| {
            seed = seed.wrapping_mul(214013).wrapping_add(2531011);
            (seed >> 8) % m
        };
        for _ in 0..n {
            let color = match next(5) {
                0 => 0xFFFF_FFFF,
                1 => 0x00FF_FFFF | next(256) << 24,
                2 => 0xFF00_0000 | next(1 << 24),
                3 => 0x64FF_FFFF,
                _ => next(1 << 16) << 16 | next(1 << 16),
            };
            canvas.color(color);
            canvas.blend_mode([1, 1, 1, 2, 3, 5, 4][next(7) as usize]);
            let pos = |next: &mut dyn FnMut(u32) -> u32| (next(140) as i32 - 30, next(110) as i32 - 30);
            match next(12) {
                0 => {
                    let (x, y) = pos(&mut next);
                    canvas.background(x - 10, y - 10);
                }
                1 => {
                    let (x, y) = pos(&mut next);
                    canvas.fill(x, y, next(20) as i32 - 2, next(20) as i32 - 2);
                }
                2 => {
                    let (x, y) = pos(&mut next);
                    canvas.text(x, y, &format!("{}", next(100000)));
                }
                3 => {
                    let ((x1, y1), (x2, y2)) = (pos(&mut next), pos(&mut next));
                    canvas.line(x1 as f32, y1 as f32, x2 as f32 + 0.5, y2 as f32);
                }
                _ => {
                    canvas.select(if next(8) == 0 { BACKGROUND } else { 100 + next(7) });
                    let (dx, dy) = pos(&mut next);
                    canvas.blit(next(60) as i32 - 10, next(50) as i32 - 10, dx, dy, next(70) as i32 - 5, next(60) as i32 - 5);
                }
            }
        }
    }

    #[test]
    fn fast_blit_matches_reference() {
        let assets = synthetic_assets();
        for seed in 0..40 {
            let mut fast = Canvas::new(&assets, 101, 83);
            let mut full = Canvas::new(&assets, 101, 83);
            full.reference = true;
            random_ops(&mut fast, seed, 400);
            random_ops(&mut full, seed, 400);
            assert_eq!(fast.draws, full.draws);
            assert!(fast.rgba == full.rgba, "semilla {seed}: la imagen rápida difiere de la de referencia");
        }
    }

    /// La capa grabada da la misma imagen que dibujar directamente, tanto al redibujarla como al reutilizarla, deja el
    /// mismo estado de dibujo y solo se reutiliza si las órdenes y el tamaño son los mismos.
    #[test]
    fn recorded_layer_matches_direct_drawing() {
        let assets = synthetic_assets();
        let mut cache = LayerCache::default();
        let run = |cache: &mut LayerCache, size: (usize, usize), seed: u32| {
            let mut direct = Canvas::new(&assets, size.0, size.1);
            direct.reference = true;
            random_ops(&mut direct, seed, 200);
            random_ops(&mut direct, seed + 1000, 50);
            let mut layered = Canvas::new(&assets, size.0, size.1);
            layered.begin_layer();
            random_ops(&mut layered, seed, 200);
            layered.end_layer(cache);
            random_ops(&mut layered, seed + 1000, 50);
            assert_eq!(layered.draws, direct.draws);
            assert!(layered.rgba == direct.rgba, "semilla {seed}, tamaño {size:?}");
        };
        let mut expect = (0, 0);
        for (size, seed, hit) in [((101, 83), 7, false), ((101, 83), 7, true), ((101, 83), 7, true), ((101, 83), 8, false), ((101, 83), 7, false), ((64, 83), 7, false), ((64, 83), 7, true), ((101, 83), 7, false)] {
            run(&mut cache, size, seed);
            if hit {
                expect.0 += 1;
            } else {
                expect.1 += 1;
            }
            assert_eq!((cache.hits, cache.misses), expect, "tamaño {size:?}, semilla {seed}");
        }
    }

    #[test]
    fn spot_frame_keeps_slots_outside_the_simulation_and_reports_losses() {
        let n = 2u16;
        let mut manager = vec![0u8; 0x110];
        manager[0xF4..0xF6].copy_from_slice(&n.to_le_bytes());
        manager[0xFC..0xFE].copy_from_slice(&2500u16.to_le_bytes());
        manager[0xFE..0x100].copy_from_slice(&100u16.to_le_bytes());
        let mut ships = vec![0u8; 0x180 * 2];
        let put = |ships: &mut Vec<u8>, i: usize, owner: u8, x: i16, state: u16, prev: u16, next: u16| {
            let s = &mut ships[i * 0x180..(i + 1) * 0x180];
            s[0] = owner;
            s[4..6].copy_from_slice(&0xFFFFu16.to_le_bytes());
            s[8..10].copy_from_slice(&0xFFFFu16.to_le_bytes());
            s[0xA..0xC].copy_from_slice(&prev.to_le_bytes());
            s[0xC..0xE].copy_from_slice(&next.to_le_bytes());
            s[0x1E..0x20].copy_from_slice(&x.to_le_bytes());
            s[0x22..0x24].copy_from_slice(&500i16.to_le_bytes());
            s[0x50..0x54].fill(0xFF);
            s[0x134..0x136].copy_from_slice(&state.to_le_bytes());
        };
        put(&mut ships, 0, 0, 100, 0xF, 0xFFFF, 1);
        put(&mut ships, 1, 7, 130, 0x12, 0, 0xFFFF);
        let mut snap = p3_save::ships::NavigationSnapshot { manager, ships, captains: vec![], convoys: vec![], routes: vec![], lists: p3_save::ships::NavigationLists { docked_head: 0xFFFF, sea_head: 0xFFFF, docked: vec![], sea: vec![] } };
        let mut m = vec![0u8; 0x650];
        m[0xE..0x10].copy_from_slice(&0u16.to_le_bytes());
        let merchants = vec![p3_core::ai::reputation::Merchant { bytes: m }];
        let mut slots = Default::default();
        let log = spot_frame(&mut snap.clone(), &merchants, 0, &mut slots).unwrap();
        assert_eq!(slots.get(&0), Some(&[1, 0xFFFF]));
        assert_eq!(log.pulses, vec![(130, 500)]);
        assert!(log.lost.is_empty());
        // El pirata se aleja más de sqrt(2500): se pierde en la siguiente pasada aunque la imagen no guardara la ranura.
        snap.ships[0x180 + 0x1E..0x180 + 0x20].copy_from_slice(&400i16.to_le_bytes());
        let log = spot_frame(&mut snap, &merchants, 0, &mut slots).unwrap();
        assert_eq!(log.lost, vec![1]);
        assert!(slots.is_empty());
    }
}
