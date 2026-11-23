//! Selector de idioma de la ventana de Opciones, como una fila más del original.
//!
//! El idioma del juego lo llevan fijo el EXE (cadenas de interfaz) y el CPR (`scripts/*_eng.*`, misiones, nombres de ciudades y
//! barcos, créditos, tutorial). El código elevado está atado al EXE inglés, así que aquí solo cambia el CPR; el cliente
//! (`p3-client::idioma`) lo prepara al arrancar. El selector guarda la elección en `idioma.txt` y se aplica al reiniciar.
//!
//! La ventana original no tiene sitio para otra fila: sus 13 filas (`options_window_on_open` `0x0045CE70`, de 32 en 32 píxeles
//! desde y = 110) llegan al borde del panel. Con el selector activo se suben 4 píxeles y se juntan 2 por fila
//! ([`Runtime::language_compact_rows`]); la fila nueva queda debajo de «Autosave» y se dibuja con las mismas piezas que las
//! demás: etiqueta alineada a la derecha en x = 494 (fuente de las etiquetas), flechas doradas de
//! `images/options/Kugel_komplett.tga` en x = 510 y 718, y el valor centrado en x = 628 (fuente de los valores).
use std::{collections::HashMap, path::PathBuf, rc::Rc};

/// Parte superior de las flechas de cada fila de la ventana original (pantalla lógica de 800×600).
const ROWS: [i32; 13] = [110, 142, 174, 206, 242, 272, 302, 329, 365, 397, 429, 461, 493];
/// Fila del idioma (parte superior de sus flechas) tras juntar las del original.
pub const ROW_Y: i32 = 493 - 4 - 2 * 12 + 30;
/// Flechas izquierda y derecha (mismas x que las del original) y su tamaño.
pub const LEFT_X: i32 = 510;
pub const RIGHT_X: i32 = 718;
const ARROW: (i32, i32) = (29, 28);
/// Etiqueta (alineada a la derecha) y valor (centrado), como en `options_window_draw` (`0x0045E920`).
const LABEL_X: i32 = 494;
const VALUE_X: i32 = 628;
/// Hoja de botones de Opciones: filas de 28 píxeles (− + ← →) y columnas de 29 (la primera, normal).
const SHEET: &str = "images/options/kugel_komplett.tga";
const SHEET_LEFT: i32 = 0x54;
const SHEET_RIGHT: i32 = 0x70;
/// Objetos hijos de la ventana de Opciones que se colocan por filas: 20 botones de flecha (`0xE8` bytes), los tres
/// deslizadores con sus piezas (`0x3B0`, `0xD8`) y los botones de sus extremos.
const CHILDREN: [(u32, u32, u32); 5] = [(0x2A8, 0xE8, 20), (0x1990, 0x3B0, 3), (0x24A0, 0xD8, 3), (0x2728, 0xE8, 3), (0x29E0, 0xE8, 3)];
const SET_OFFSET: u32 = 0x0040_25F0;

#[derive(Debug, Default)]
pub struct LanguageOptions {
    pub enabled: bool,
    /// (código, nombre mostrado): el primero es el del CPR base.
    pub options: Vec<(String, String)>,
    /// Posición del idioma en uso en esta ejecución y del elegido (guardado).
    pub active: usize,
    pub pending: usize,
    pub path: Option<PathBuf>,
    /// La ventana de Opciones está abierta (lo marcan los ganchos de `video_options.rs`).
    pub open: bool,
    /// Último fotograma en que la ventana de Opciones dio señales de vida (`beat`).
    seen: Option<u64>,
    /// Cadenas del EXE inglés (por dirección del literal) → texto del otro idioma (`scripts/gen_language_pack.py`).
    pub strings: Option<Rc<HashMap<u32, Vec<u8>>>>,
    /// Caracteres añadidos al juego de caracteres de las fuentes (ñ ¿ ¡ ...), con sus celdas en el CPR derivado.
    pub extra_charset: Vec<u8>,
    /// Dirección del literal inglés → dirección de su traducción en memoria (`TEXT_BASE`): las tablas de punteros a cadenas se reescriben
    /// para que el código del juego, que copia o dibuja esas cadenas sin pasar por una rutina interceptada, lea ya el texto traducido.
    redirect: HashMap<u32, u32>,
    /// Ranuras de datos del EXE que guardan punteros a literales con traducción: las únicas que se reescriben. Se fijan al cargar
    /// el paquete (ver `load_language_pack`); el resto de datos es estado del juego y no se toca.
    slots: Vec<u32>,
    last_scan: u64,
    prev_down: bool,
    /// Fuentes que usa `options_window_draw` para etiquetas y valores (se anotan al dibujar la ventana).
    pub label_font: Option<u32>,
    pub value_font: Option<u32>,
}

impl LanguageOptions {
    /// Ranuras de punteros que reescribe el idioma (para pruebas y diagnóstico).
    pub fn slots(&self) -> &[u32] {
        &self.slots
    }
    pub fn enable(&mut self, options: Vec<(String, String)>, active: usize, path: Option<PathBuf>) {
        self.enabled = options.len() > 1;
        self.active = active.min(options.len().saturating_sub(1));
        self.pending = self.active;
        self.options = options;
        self.path = path;
    }

    /// La ventana de Opciones está viva en el fotograma `presents`.
    pub fn beat(&mut self, presents: u64) {
        self.seen = Some(presents);
    }

    /// Al presentar: la fila solo se ve si la ventana dio señales en este fotograma o el anterior.
    pub fn refresh(&mut self, presents: u64) {
        self.open = self.seen.is_some_and(|s| presents.saturating_sub(s) <= 2);
    }

    /// Procesa un fotograma de ratón. Devuelve `true` si el idioma elegido cambió.
    pub fn click(&mut self, cursor: (i32, i32), down: bool) -> bool {
        let edge = down && !self.prev_down;
        self.prev_down = down;
        if !edge || self.options.len() < 2 {
            return false;
        }
        let (x, y) = cursor;
        if !(ROW_Y..ROW_Y + ARROW.1).contains(&y) {
            return false;
        }
        let n = self.options.len();
        let next = if (LEFT_X..LEFT_X + ARROW.0).contains(&x) {
            (self.pending + n - 1) % n
        } else if (RIGHT_X..RIGHT_X + ARROW.0).contains(&x) {
            (self.pending + 1) % n
        } else {
            return false;
        };
        self.pending = next;
        true
    }

    /// Fotograma sin ventana de Opciones: olvida el botón pulsado.
    pub fn close_frame(&mut self) {
        self.prev_down = false;
    }

    pub fn save(&self) {
        let (Some(path), Some((code, _))) = (&self.path, self.options.get(self.pending)) else { return };
        let tmp = path.with_extension("tmp");
        let fs = crate::hostfs::current();
        if let Some(dir) = path.parent() {
            let _ = fs.create_dir_all(dir);
        }
        if let Err(e) = fs.write(&tmp, format!("{code}\n").as_bytes()).and_then(|()| fs.rename(&tmp, path)) {
            eprintln!("no se pudo guardar el idioma: {e}");
        }
    }
}

/// Zona de memoria donde se guardan los textos traducidos (libre: el montón empieza en 0x2000_0000, el bloqueo de pantalla en 0x4000_0000).
const TEXT_BASE: u32 = 0x6000_0000;
/// Código del EXE y sus datos (`.rdata` y `.data` con su parte sin inicializar).
const CODE_START: u32 = 0x0040_1000;
const DATA_START: u32 = 0x0066_A000;
const DATA_END: u32 = 0x006F_2000;

impl super::Runtime<'_> {
    /// Fotograma de Opciones: atiende el ratón y, si la ventana está abierta, devuelve la superficie con la fila dibujada.
    pub(super) fn language_overlay(&mut self, base: &super::Surface) -> Option<super::Surface> {
        if !self.language.enabled || !self.language.open {
            self.language.prev_down = false;
            return None;
        }
        let down = self.input.down[usize::from(super::win32::VK_LBUTTON)];
        if self.language.click(self.input.cursor, down) {
            self.language.save();
        }
        let sheet = self.assets.defs.iter().find(|(_, d)| d.file.eq_ignore_ascii_case(SHEET)).map(|(h, _)| *h);
        // Fuentes del original; si aún no se han visto, las más parecidas por altura.
        let by_height = |assets: &super::Assets, min: u32| {
            assets.fonts.iter().filter(|(_, f)| f.cell.1 >= min).min_by_key(|(h, f)| (f.cell.1, **h)).map(|(h, _)| *h)
        };
        let label_font = self.language.label_font.filter(|f| self.assets.fonts.contains_key(f)).or_else(|| by_height(&self.assets, 16))?;
        let value_font = self.language.value_font.filter(|f| self.assets.fonts.contains_key(f)).unwrap_or(label_font);
        let mut s = super::Surface::new(base.width, base.height, [0, 0, 0, 0xFF]);
        s.rgba.copy_from_slice(&base.rgba);
        s.color = 0xFFFF_FFFF;
        if let Some(sheet) = sheet {
            for (x, sy) in [(LEFT_X, SHEET_LEFT), (RIGHT_X, SHEET_RIGHT)] {
                s.blit(&mut self.assets, sheet, (0, sy, x, ROW_Y, ARROW.0, ARROW.1));
            }
        }
        let spanish = self.language.options.get(self.language.active).is_some_and(|o| o.0 == "spa");
        s.font = label_font;
        s.text_mode = 2;
        s.text(&mut self.assets, LABEL_X, ROW_Y + 2, if spanish { b"Idioma" } else { b"Language" });
        s.font = value_font;
        s.text_mode = 3;
        let code = self.language.options.get(self.language.pending).map_or("", |o| o.0.as_str()).to_owned();
        let fallback = self.language.options.get(self.language.pending).map_or("?", |o| o.1.as_str()).to_owned();
        let font = self.assets.fonts.get(&value_font);
        let mut value = native_name(&code).map_or_else(|| fallback.into_bytes(), |n| fit_charset(n, font));
        if self.language.pending != self.language.active {
            // Se aplica al reiniciar: el aviso va junto al valor si cabe entre las flechas.
            let mut hint = value.clone();
            hint.extend_from_slice(if spanish { b" (al reiniciar)" } else { b" (on restart)" });
            if font.is_some_and(|f| f.text_width(&hint) <= RIGHT_X - LEFT_X - ARROW.0 - 6) {
                value = hint;
            }
        }
        s.text(&mut self.assets, VALUE_X, ROW_Y + 4, &value);
        Some(s)
    }

    /// Tras `options_window_on_open`: sube 4 píxeles las filas del original y las junta 2 por fila para que quepa la del idioma.
    /// Las etiquetas y valores se dibujan a la altura de su botón (`window_get_offset`, `+0x20`); los botones y deslizadores, en
    /// la posición que guarda la lista de hijos de la ventana (`+0x94`, entradas de 16 bytes: hijo, x, y, z; `+0xC0` hijos), que
    /// también sirve para saber dónde se pulsa. Se cambian las dos.
    pub(super) fn language_compact_rows(&mut self, m: &mut dyn p3_render::scene_math::Mem, window: u32) {
        let shift = |y: i32| ROWS.iter().position(|&r| r == y).map(|k| y - 4 - 2 * k as i32);
        let (list, count) = (m.r32(window + 0x94), m.r32(window + 0xC0).min(256));
        let own = window + 0x2A8..window + 0x2C98;
        for j in 0..count {
            let entry = list + j * 16;
            if !own.contains(&m.r32(entry)) {
                continue;
            }
            if let Some(y) = shift(m.r32(entry + 8) as i32) {
                m.w32(entry + 8, y as u32);
            }
        }
        for (base, size, count) in CHILDREN {
            for i in 0..count {
                let child = window + base + i * size;
                let (x, y) = (m.r32(child + 0x20) as i32, m.r32(child + 0x24) as i32);
                if let Some(y) = shift(y) {
                    self.dispatch(m, SET_OFFSET, child, &[x as u32, y as u32]);
                }
            }
        }
    }

    /// `sgl_DrawText` de `options_window_draw`: anota las fuentes de etiquetas (x = 494) y valores (x = 628).
    pub(super) fn language_note_font(&mut self, x: i32) {
        if !self.language.enabled || !self.stack.contains(&0x0045_E920) {
            return;
        }
        match x {
            LABEL_X => self.language.label_font = Some(self.surface.font),
            VALUE_X => self.language.value_font = Some(self.surface.font),
            _ => {}
        }
    }
}

/// Nombre de cada idioma en su propio idioma (Latin-1, como las fuentes del juego).
fn native_name(code: &str) -> Option<&'static [u8]> {
    Some(match code {
        "eng" => b"English",
        "spa" => b"Espa\xF1ol",
        "ger" | "deu" => b"Deutsch",
        "fre" | "fra" => b"Fran\xE7ais",
        "ita" => b"Italiano",
        "pol" => b"Polski",
        "dut" | "nld" => b"Nederlands",
        _ => return None,
    })
}

/// Sustituye por su letra sin tilde los caracteres que la fuente no tiene (con el CPR inglés no hay ñ).
fn fit_charset(name: &[u8], font: Option<&p3_render::sgl_font::Font>) -> Vec<u8> {
    let has = |c: u8| font.is_none_or(|f| c < 0x80 || f.map[usize::from(c)] != 0);
    name.iter()
        .map(|&c| match c {
            _ if has(c) => c,
            0xF1 => b'n',
            0xE7 => b'c',
            _ => b'?',
        })
        .collect()
}

fn unhex(s: &str) -> Option<Vec<u8>> {
    if s.len() % 2 != 0 {
        return None;
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(s.get(i..i + 2)?, 16).ok()).collect()
}

impl super::Runtime<'_> {
    /// Carga un paquete de idioma (`H sha1 código`, `S dirección texto-hex`, `P dirección bytes-hex`): las `P` se copian en la memoria
    /// del EXE (tablas de maquetación) y las `S` solo si en esa dirección hay de verdad el comienzo de una cadena. Devuelve
    /// (cadenas aceptadas, cadenas descartadas, tablas).
    pub fn load_language_pack(&mut self, mem: &mut super::PageMem, text: &str) -> (usize, usize, usize) {
        let mut strings = HashMap::new();
        let (mut skipped, mut tables) = (0, 0);
        for line in text.lines() {
            let mut it = line.split_whitespace();
            // `S dirección` sin texto es una traducción vacía (sufijos «house»/«houses» que el EXE español deja en blanco).
            let (Some(kind), Some(addr)) = (it.next(), it.next()) else { continue };
            let hex = it.next().unwrap_or("");
            let (Some(addr), Some(bytes)) = (u32::from_str_radix(addr, 16).ok(), unhex(hex)) else { continue };
            match kind {
                "S" if mem.u8_at(addr.wrapping_sub(1)) == 0 && mem.u8_at(addr) != 0 => {
                    strings.insert(addr, bytes);
                }
                "S" => skipped += 1,
                "X" => self.language.extra_charset = bytes,
                "P" => {
                    mem.write(addr, &bytes);
                    tables += 1;
                }
                _ => {}
            }
        }
        // Traducciones a memoria y reescritura de las tablas de punteros (también las que el juego reescribe luego: ver `language_tick`).
        let mut at = TEXT_BASE;
        let mut keys: Vec<_> = strings.keys().copied().collect();
        keys.sort_unstable();
        for k in keys {
            let t = &strings[&k];
            mem.write(at, t);
            mem.write(at + t.len() as u32, &[0]);
            self.language.redirect.insert(k, at);
            at += (t.len() as u32 + 4) & !3;
        }
        // Ranuras que de verdad son punteros a esos literales: las que ya lo contienen en la imagen del EXE (tablas inicializadas)
        // y los destinos absolutos que el código rellena con un inmediato (`mov dword ptr [dir], literal`, `C7 05`: los
        // constructores que reescriben tablas tras la carga). Antes se barría todo `.rdata`/`.data` en cada pasada y cualquier
        // palabra del estado del juego que coincidiera con la dirección de un literal quedaba pisada con la de su traducción:
        // las cabeceras de las listas de barcos (`0x6DD888`, dos `u16` seguidos) valieron `0x006B0020` el 15/11/2026 y los
        // barcos de Sergio dejaron de moverse (`p3-juego.log`: `memoria 0x6dd888: 0x6b0020 -> 0x600150ec`).
        let mut slots = Vec::new();
        for va in (DATA_START..DATA_END).step_by(4) {
            if self.language.redirect.contains_key(&mem.u32_at(va)) {
                slots.push(va);
            }
        }
        for at in CODE_START..DATA_START - 10 {
            if mem.u8_at(at) == 0xC7 && mem.u8_at(at + 1) == 0x05 {
                let (dest, value) = (mem.u32_at(at + 2), mem.u32_at(at + 6));
                if (DATA_START..DATA_END).contains(&dest) && self.language.redirect.contains_key(&value) {
                    slots.push(dest);
                }
            }
        }
        slots.sort_unstable();
        slots.dedup();
        self.language.slots = slots;
        self.redirect_slots(mem);
        p3_assets::exe::set_translations(strings.clone());
        let n = strings.len();
        self.language.strings = (n > 0).then(|| Rc::new(strings));
        (n, skipped, tables)
    }

    /// Reescribe los punteros a literales ingleses con traducción, solo en las ranuras fijadas al cargar el paquete.
    pub fn redirect_slots(&mut self, m: &mut dyn p3_render::scene_math::Mem) {
        if self.language.redirect.is_empty() {
            return;
        }
        for i in 0..self.language.slots.len() {
            let va = self.language.slots[i];
            let v = m.r32(va);
            if let Some(&n) = self.language.redirect.get(&v) {
                m.w32(va, n);
            }
        }
    }

    /// Vuelve a pasar `redirect_slots` cada cierto número de fotogramas: los constructores del juego reescriben algunas tablas con
    /// direcciones inmediatas después de la carga.
    pub(super) fn language_tick(&mut self, m: &mut dyn p3_render::scene_math::Mem) {
        if self.language.redirect.is_empty() || self.presents.wrapping_sub(self.language.last_scan) < 30 && self.language.last_scan != 0 {
            return;
        }
        self.language.last_scan = self.presents.max(1);
        self.redirect_slots(m);
    }

    /// Cadena C en `p`: la traducida si `p` es un literal del EXE con traducción, si no la de la memoria.
    pub(super) fn tr_cstr(&self, m: &mut dyn p3_render::scene_math::Mem, p: u32) -> Vec<u8> {
        match self.language.strings.as_ref().and_then(|t| t.get(&p)) {
            Some(t) => t.clone(),
            None => super::cstr(m, p),
        }
    }
}
