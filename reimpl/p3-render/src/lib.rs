//! Renderizador de referencia de la vista de ciudad (J01): `.town` + `.let` + `.aim`, sin el juego.
//!
//! Proyección (`class22_cell_to_screen`, `0x0058DAC0`) con `TileSize = 34 21` y `DownVector = 17 11` de `scripts/iso.ini`:
//! las filas se escalonan; con la fila `y` impar la casilla `(x, y)` cae en `(34 x, 22 (y/2) + 22)` y con la par en
//! `(34 (x-1) + 17, 22 (y/2) + 11)`. El sprite se coloca en `(px [+ ancho si el fotograma ≥ a], py - alto del fotograma)` (`sprite_sheet_draw_frame`, `0x00551C80`).

use p3_assets::aim;
use p3_assets::cpr::Archive;
use p3_assets::let_sheet::LetSheet;
use p3_assets::town::TownMap;
use std::collections::HashMap;

pub mod ambient;
pub mod ret_pop;
pub mod ambience;
pub mod debris;
pub mod effect_step;
pub mod fish;
pub mod crack;
pub mod particles;
pub mod scene_init;
pub mod scene_order;
pub mod spawn_types;
pub mod precip;
pub mod scene_math;
pub mod hooks6;
pub mod precip_step;
pub mod crack_conductor;
pub mod fire_state;
pub mod fire_pack;
pub mod progress;
pub mod soldier;
pub mod sprite_set;
pub mod animals;
pub mod anim;
pub mod birds;
pub mod clouds;
pub mod carts;
pub mod cart_field;
pub mod crowd;
pub mod footprints;
pub mod effects;
pub mod factory;
pub mod geom;
pub mod harbor_ships;
pub mod harbor_sprite;
pub mod lifecycle;
pub mod manager;
pub mod mover;
pub mod pets;
pub mod rats;
pub mod scene_grid;
pub mod spawn;
pub mod walk;
pub mod walkers;
pub mod navmap;
pub mod navmap_draw;
pub mod navmap_update;
pub mod sgl;
pub mod sgl_clip;
pub mod sgl_font;
pub mod sgl_slots;
pub mod sgl_screen;
pub mod sgl_tex;
#[allow(unused_parens, unreachable_code, unused_variables, unused_assignments, unused_mut, non_snake_case, dead_code)] // código traducido del x86: avisos de estilo sin efecto
pub mod world;
mod city_fast;
#[allow(unused_parens, unreachable_code, unused_variables, unused_assignments, unused_mut, non_snake_case, dead_code)] // código traducido del x86: avisos de estilo sin efecto
pub mod j07;
pub mod panels;
pub mod minimap;
#[allow(unused_parens, unreachable_code, unused_variables, unused_assignments, unused_mut, non_snake_case, dead_code)] // código traducido del x86: avisos de estilo sin efecto
pub mod j08;

pub const TILE_W: i32 = 34;
pub const TILE_H: i32 = 21;
pub const DOWN_X: i32 = 17;
pub const DOWN_Y: i32 = 11;

/// Parámetros de `class22_cell_to_screen` (`+0x510/+0x514` paso, `+0x4D0/+0x4D4` desplazamiento, `+0x14/+0x18` origen de la vista).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Projection {
    pub step_x: i32,
    pub step_y: i32,
    pub down_x: i32,
    pub down_y: i32,
    pub origin_x: i32,
    pub origin_y: i32,
}

/// Valores de `scripts/iso.ini` (`TileSize = 34 21 0`, `DownVector = 17 11 0`) con el origen en cero.
pub const INI: Projection = Projection {
    step_x: TILE_W,
    step_y: TILE_H,
    down_x: DOWN_X,
    down_y: DOWN_Y,
    origin_x: 0,
    origin_y: 0,
};

impl Projection {
    /// Copia exacta de `0x0058DAC0`: aritmética de 32 bits con desbordamiento, y el desplazamiento `shr` es lógico.
    pub fn cell_to_screen(&self, x: i32, y: i32) -> (i32, i32) {
        let half = ((y as u32) >> 1) as i32;
        let (px, py) = if y & 1 != 0 {
            (
                self.step_x.wrapping_mul(x),
                half.wrapping_mul(self.step_y.wrapping_add(1))
                    .wrapping_add(self.step_y)
                    .wrapping_add(1),
            )
        } else {
            (
                x.wrapping_sub(1).wrapping_mul(self.step_x).wrapping_add(self.down_x),
                self.step_y.wrapping_add(1).wrapping_mul(half).wrapping_add(self.down_y),
            )
        };
        (px.wrapping_add(self.origin_x), py.wrapping_add(self.origin_y))
    }
}

/// Posición de pantalla de una casilla con los valores de `iso.ini` y sin origen de vista.
pub fn cell_to_screen(x: i32, y: i32) -> (i32, i32) {
    INI.cell_to_screen(x, y)
}

/// `sprite_sheet_draw_frame` (`0x00551C80`): `(origen x, origen y, destino x, destino y, ancho, alto)` de un fotograma colocado en `(x, y)`;
/// los fotogramas desde `a` se desplazan su propio ancho a la derecha y todos se alinean por abajo.
pub fn frame_placement(rect: [u16; 4], a: u8, x: i32, y: i32, frame: u8) -> [i32; 6] {
    let [sx, sy, w, h] = rect.map(i32::from);
    let dx = if frame >= a { x.wrapping_add(w) } else { x };
    [sx, sy, dx, y.wrapping_sub(h), w, h]
}

/// Rectángulo completo de una hoja (0x00551C20); no determina variantes ni profundidad.
pub fn footprint_placement(rect:[u16;4],strips_a:u8,first_width:u16,x:i32,y:i32)->[i32;6]{
    [i32::from(rect[0]),i32::from(rect[1]),x.wrapping_sub((i32::from(strips_a)-1).wrapping_mul(i32::from(first_width))),y.wrapping_sub(i32::from(rect[3])),i32::from(rect[2]),i32::from(rect[3])]
}

/// Textura RGBA de 8 bits por canal.
pub struct Texture {
    pub width: usize,
    pub height: usize,
    pub rgba: Vec<u8>,
}

/// Convierte la imagen descodificada en RGBA. El color de clave magenta (`FF 00 FF`) queda transparente.
pub fn to_rgba(img: &aim::Image) -> Option<Texture> {
    let (w, h) = (img.width as usize, img.height as usize);
    let mut rgba = vec![0u8; w * h * 4];
    let pitch = img.pitch as usize;
    match (img.bytes_per_pixel, &img.palette) {
        (1, Some(pal)) => {
            // El orden de la paleta es B, G, R, A. Tabla ya convertida (una paleta corta se rellena con `None`: falla solo si se usa).
            let mut lut = [None; 256];
            for (i, e) in lut.iter_mut().enumerate() {
                if let Some(p) = pal.get(i * 4..i * 4 + 4) {
                    let (r, g, b) = (p[2], p[1], p[0]);
                    *e = Some([r, g, b, if (r, g, b) == (255, 0, 255) { 0 } else { 255 }]);
                }
            }
            for y in 0..h {
                if w == 0 {
                    break;
                }
                let row = img.pixels.get(y * pitch..y * pitch + w)?;
                for (o, &i) in rgba[y * w * 4..(y + 1) * w * 4].chunks_exact_mut(4).zip(row) {
                    o.copy_from_slice(&lut[usize::from(i)].expect("índice de paleta fuera de la paleta"));
                }
            }
        }
        (4, None) => {
            for y in 0..h {
                if w == 0 {
                    break;
                }
                let row = img.pixels.get(y * pitch..y * pitch + w * 4)?;
                for (o, p) in rgba[y * w * 4..(y + 1) * w * 4].chunks_exact_mut(4).zip(row.chunks_exact(4)) {
                    o.copy_from_slice(&[p[2], p[1], p[0], p[3]]);
                }
            }
        }
        _ => return None,
    }
    Some(Texture {
        width: w,
        height: h,
        rgba,
    })
}

pub struct Scene<'a> {
    archive: &'a Archive<'a>,
    sheets: HashMap<u16, LetSheet>,
    /// Todas las hojas `iso/let/*.let` en el orden del archivo (se leen una sola vez por escena).
    all_sheets: Vec<LetSheet>,
    /// Índices en `all_sheets` por clase gráfica (`field_2d`), en el orden del archivo.
    sheets_by_kind: HashMap<u8, Vec<usize>>,
    /// Nombre plegado ([`fold`]) → última entrada conservada del archivo con ese nombre (la que elige `texture`).
    last_kept: HashMap<Vec<u8>, usize>,
    /// Nombre plegado → primera entrada del archivo con ese nombre, conservada o no (la que elige `load_figures`).
    first_any: HashMap<Vec<u8>, usize>,
    textures: HashMap<Vec<u8>, Option<Texture>>,
    pub missing_sheets: usize,
    /// Casillas cuyo fotograma `b2` no existe en su hoja.
    pub bad_frames: usize,
    pub unsupported_textures: usize,
    /// Objetos que `draw_unambiguous_objects` no pudo elegir: `(clase gráfica, hojas candidatas, ¿con variante?)`.
    pub unresolved: Vec<(u8, usize, bool)>,
    /// Objetos elegidos por `draw_*_objects` a la espera de `apply_objects`: `(x, y, ficha de la hoja)`.
    placed: Vec<(i32, i32, u16)>,
    /// Animadores de los objetos colocados (una por grupo de animación de su hoja).
    pub animators: Vec<SceneAnimator>,
    /// Animadores descartados por datos que el original dividiría por cero.
    pub bad_animators: usize,
    /// Grupos de llamas/humo que no se animan sin incendio.
    pub flame_groups: usize,
}

/// Animación de un objeto de la escena: un grupo de trozos `.let` (etiqueta >= 3) con su animador.
#[derive(Clone, Debug)]
pub struct SceneAnimator {
    /// Casilla de origen del objeto.
    pub cell: (i32, i32),
    pub sheet: u16,
    pub tag: u32,
    pub kind: anim::Kind,
    /// Índices en `LetSheet::chunks` de cada animación del grupo.
    pub chunks: Vec<usize>,
    pub anims: Vec<anim::Anim>,
    pub state: anim::AnimState,
    /// Fotograma visible ahora: `(animación, fotograma)`.
    pub shown: Option<anim::Draw>,
}

fn fold(path: &[u8]) -> Vec<u8> {
    path.iter()
        .take_while(|&&b| b != 0)
        .map(|&b| if b == b'\\' { b'/' } else { b.to_ascii_lowercase() })
        .collect()
}

impl<'a> Scene<'a> {
    pub fn new(archive: &'a Archive<'a>) -> Self {
        let mut sheets: HashMap<u16, LetSheet> = HashMap::new();
        let mut all_sheets = Vec::new();
        let mut sheets_by_kind: HashMap<u8, Vec<usize>> = HashMap::new();
        let mut last_kept = HashMap::with_capacity(archive.entries.len());
        let mut first_any = HashMap::with_capacity(archive.entries.len());
        for (i, e) in archive.entries.iter().enumerate() {
            let name = fold(&e.name);
            if name.starts_with(b"iso/let/") && name.ends_with(b".let") {
                if let Some(data) = archive.read(e) {
                    if let Ok(sheet) = LetSheet::parse(data) {
                        sheets_by_kind.entry(sheet.field_2d).or_default().push(all_sheets.len());
                        sheets.entry(sheet.tile_id).or_insert_with(|| sheet.clone());
                        all_sheets.push(sheet);
                    }
                }
            }
            if e.kept {
                last_kept.insert(name.clone(), i);
            }
            first_any.entry(name).or_insert(i);
        }
        Scene {
            archive,
            sheets,
            all_sheets,
            sheets_by_kind,
            last_kept,
            first_any,
            textures: HashMap::new(),
            missing_sheets: 0,
            bad_frames: 0,
            unsupported_textures: 0,
            unresolved: Vec::new(),
            placed: Vec::new(),
            animators: Vec::new(),
            bad_animators: 0,
            flame_groups: 0,
        }
    }

    /// Crea los animadores de los objetos colocados (clase según el `array` del primer trozo de cada grupo, `0x552E80`/`0x552EF0`).
    fn build_animators(&mut self, placed: &[(i32, i32, u16)]) {
        use crate::anim::{Anim, AnimState, Kind};
        self.animators.clear();
        for &(x, y, id) in placed {
            let Some(sheet) = self.sheets.get(&id) else { continue };
            let mut tags: Vec<u32> = sheet.chunks.iter().map(|c| c.tag).filter(|&t| t >= 3).collect();
            tags.sort_unstable();
            tags.dedup();
            for tag in tags {
                let chunks: Vec<usize> = (0..sheet.chunks.len()).filter(|&i| sheet.chunks[i].tag == tag).collect();
                let first = &sheet.chunks[chunks[0]];
                // Las llamas y el humo (`Feuer_*`) las gestiona el estado de incendio del edificio (0x550550), no el animador del grupo.
                if first.texture.as_ref().is_some_and(|t| fold(&t.path).windows(5).any(|w| w == b"feuer")) {
                    self.flame_groups += 1;
                    continue;
                }
                let Some(kind) = Kind::from_array_len(first.array.len()) else { continue };
                let anims: Vec<Anim> = chunks.iter().map(|&i| Anim::from_array(&sheet.chunks[i].array, sheet.chunks[i].frames.len())).collect();
                let needs_period = !matches!(kind, Kind::Simple | Kind::Idle);
                if anims.iter().any(|a| a.duration == 0 || a.frames == 0 || (needs_period && a.period == 0)) {
                    self.bad_animators += 1;
                    if std::env::var_os("P3_ANIM_DEBUG").is_some() {
                        eprintln!("animador descartado: hoja {id} etiqueta {tag} {kind:?} {anims:?}");
                    }
                    continue;
                }
                self.animators.push(SceneAnimator { cell: (x, y), sheet: id, tag, kind, chunks, anims, state: AnimState::new(), shown: None });
            }
        }
    }

    /// Avanza todas las animaciones al reloj `clock` (milisegundos, `[0x6DCCF8]`) con el `rand` dado.
    pub fn advance_animations(&mut self, clock: u32, rng: &mut anim::CrtRand) {
        for a in &mut self.animators {
            let d = a.state.update(a.kind, &a.anims, a.anims.len(), true, clock, || rng.next());
            a.shown = match d {
                Some(d) => Some(d),
                None if a.state.flags & 0x80 != 0 => None,
                None => a.shown,
            };
        }
    }

    pub fn sheet_count(&self) -> usize {
        self.sheets.len()
    }

    /// Dibuja hojas cuya clase gráfica tiene una única hoja en el CPR.
    /// La posición del rectángulo usa `sprite_sheet_draw_footprint_rect`
    /// (0x00551C20). En casas se selecciona la variante y material del
    /// registro con grado gráfico 2; también se resuelven murallas por su
    /// código de estilo. Las demás variantes quedan sin dibujar.
    /// El orden de profundidad original aún no está portado.
    pub fn draw_unambiguous_objects(&mut self,_canvas:&mut [u8],_width:usize,_height:usize,objects:&[(u8,i32,i32,Option<(u8,u8)>)])->usize{
        let sheets:HashMap<u8,Vec<&LetSheet>>=self.sheets_by_kind.iter().map(|(&k,v)|(k,v.iter().map(|&i|&self.all_sheets[i]).collect())).collect();
        let mut sorted=objects.to_vec();sorted.sort_by_key(|&(_,x,y,_)|(y,x));let mut drawn=0;
        for (kind,x,y,variant) in sorted{let Some(choices)=sheets.get(&kind) else{self.unresolved.push((kind,0,variant.is_some()));continue};let sheet=if choices.len()==1{choices[0]}else if let Some((style,material))=variant {let mut found=choices.iter().filter(|s|if kind==0x2e{s.matrix.get(1)==Some(&u32::from(style))&&s.matrix.get(2).copied().unwrap_or(2)==2}else{s.matrix.get(1)==Some(&u32::from(style))&&s.matrix.get(2).is_some_and(|&v|(v==0xe0)==(material==0xe0))&&s.matrix.get(3).copied().unwrap_or(2)==2});let Some(first)=found.next() else{self.unresolved.push((kind,choices.len(),true));continue};if found.next().is_some(){self.unresolved.push((kind,choices.len(),true));continue}*first}else{self.unresolved.push((kind,choices.len(),false));continue};if sheet.strips.is_empty()||sheet.strips_a==0||sheet.tile_width<=0||sheet.tile_height<=0{continue}self.placed.push((x,y,sheet.tile_id));}
        drawn
    }

    /// Dibuja objetos cuya hoja se elige por valores exactos de la matriz del `.let` (el resto de las reglas de
    /// `0x005928A0`: iglesia, ayuntamiento, torres y puertas). Cada objeto trae listas alternativas de requisitos
    /// `(índice de matriz, valor)`; se usa la primera lista que deja exactamente una hoja de la clase.
    /// Lo que no se resuelve se anota en `unresolved`.
    pub fn draw_matrix_objects(&mut self,_canvas:&mut [u8],_width:usize,_height:usize,objects:&[(u8,i32,i32,Vec<Vec<(usize,u32)>>)])->usize{
        let sheets:HashMap<u8,Vec<&LetSheet>>=self.sheets_by_kind.iter().map(|(&k,v)|(k,v.iter().map(|&i|&self.all_sheets[i]).collect())).collect();
        let mut sorted=objects.to_vec();sorted.sort_by_key(|o|(o.2,o.1));let mut drawn=0;
        for (kind,x,y,alternatives) in sorted{
            let empty=Vec::new();let choices=sheets.get(&kind).unwrap_or(&empty);
            let pick=alternatives.iter().find_map(|reqs|{let mut found=choices.iter().filter(|s|reqs.iter().all(|&(i,v)|s.matrix.get(i)==Some(&v)));let first=found.next()?;if found.next().is_some(){None}else{Some(*first)}});
            let Some(sheet)=pick else{self.unresolved.push((kind,choices.len(),true));continue};
            if sheet.strips.is_empty()||sheet.strips_a==0||sheet.tile_width<=0||sheet.tile_height<=0{continue}
            self.placed.push((x,y,sheet.tile_id));
        }
        drawn
    }

    /// Coloca en la capa 1 del mapa los objetos elegidos, como `0x0058DB40`: vacía su huella (`0x0058DF80`) y estampa la ficha con un
    /// fotograma por casilla (`0x0058E180`). El orden de dibujo es entonces el de la cuadrícula (filas de arriba abajo, capa 1 sobre la 0),
    /// con las marcas de solape `b3`. Devuelve cuántos objetos se colocaron.
    pub fn apply_objects(&mut self, map: &mut TownMap) -> usize {
        use crate::scene_grid::{Entities, SceneGrid};
        struct Sheets<'a>(&'a HashMap<u16, LetSheet>);
        impl Entities for Sheets<'_> {
            fn size(&self, id: u16) -> (u32, u32) {
                self.0.get(&id).map_or((0, 0), |s| (u32::from(s.strips_a), u32::from(s.strips_b)))
            }
            fn frame_height(&self, id: u16, frame: u8) -> u16 {
                self.0.get(&id).and_then(|s| s.strips.get(usize::from(frame))).map_or(0, |f| f[3] as u16)
            }
        }
        let mut placed = std::mem::take(&mut self.placed);
        placed.sort_by_key(|&(x, y, _)| (y, x));
        self.build_animators(&placed);
        let mut grid = SceneGrid::new(map.width, map.height, TILE_H as u32);
        grid.layers[0] = std::mem::take(&mut map.layers[0]);
        grid.layers[1] = std::mem::take(&mut map.layers[1]);
        let sheets = Sheets(&self.sheets);
        let mut count = 0;
        for (x, y, id) in placed {
            if x < 0 || y < 0 {
                continue;
            }
            let (w, h) = sheets.size(id);
            grid.clear(&sheets, x as u32, y as u32, w, h, 1, 0, 0);
            grid.place(&sheets, x as u32, y as u32, id, 1);
            count += 1;
        }
        map.layers[0] = std::mem::take(&mut grid.layers[0]);
        map.layers[1] = std::mem::take(&mut grid.layers[1]);
        count
    }


    /// Clase de la ficha (`LetSheet::field_2d`, método virtual `+0x50` de la clase de la ficha).
    pub fn tile_kind(&self, tile: u16) -> Option<u32> {
        self.sheets.get(&tile).map(|s| u32::from(s.field_2d))
    }

    /// Figuras `.lcr` de las clases 0-4 en el orden de `iso/lcr/files.lst`, agrupadas por clase (las variantes de cada clase).
    pub fn load_figures(&self) -> HashMap<u32, Vec<p3_assets::lcr::Lcr>> {
        let mut out: HashMap<u32, Vec<p3_assets::lcr::Lcr>> = HashMap::new();
        let first = |name: &[u8]| self.first_any.get(name).map(|&i| &self.archive.entries[i]);
        let Some(list) = first(b"iso/lcr/files.lst").and_then(|e| self.archive.read(e)) else { return out };
        for name in list.split(|&b| b == 0).filter(|n| !n.is_empty()) {
            let want = [b"iso/lcr/".as_slice(), &name.to_ascii_lowercase()].concat();
            let Some(data) = first(&want).and_then(|e| self.archive.read(e)) else { continue };
            if data.len() < 8 {
                continue;
            }
            let kind = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
            let Some(words) = p3_assets::lcr::extra_header_words(kind) else { continue };
            if let Ok(l) = p3_assets::lcr::Lcr::parse(data, words) {
                out.entry(kind).or_default().push(l);
            }
        }
        out
    }

    /// Dibuja las figuras en su fotograma de espera (estado 2) con el rumbo dado, ordenadas por casilla del pie. `shift` pasa de píxel del
    /// mundo (`0x58CFB0`) al lienzo. Devuelve cuántas se dibujaron.
    pub fn draw_walkers(&mut self, canvas: &mut [u8], width: usize, height: usize, walkers: &[walkers::Walker], figures: &HashMap<u32, Vec<p3_assets::lcr::Lcr>>, shift: (i32, i32), live_frames: bool) -> usize {
        let mut order: Vec<usize> = (0..walkers.len()).collect();
        order.sort_by_key(|&i| (walkers[i].y, walkers[i].x));
        // Las texturas de las figuras que se van a dibujar se decodifican antes, en paralelo (las mismas que cargaría el bucle).
        let mut keys: Vec<Vec<u8>> = Vec::new();
        for &i in &order {
            let w = &walkers[i];
            let Some(set) = figures.get(&w.kind).and_then(|v| v.get(w.variant as usize % v.len().max(1))) else { continue };
            let mut w2 = w.clone();
            if !live_frames {
                w2.frame = set.state_index(2).map_or(0, |s| set.states[s].first as u8);
            }
            if w2.visible(set, (0, 0)).is_some() {
                let key = fold(set.sheet_path());
                if !keys.contains(&key) {
                    keys.push(key);
                }
            }
        }
        self.preload_textures(&keys);
        let mut drawn = 0;
        for i in order {
            let w = &walkers[i];
            let Some(set) = figures.get(&w.kind).and_then(|v| v.get(w.variant as usize % v.len().max(1))) else { continue };
            let first = set.state_index(2).map_or(0, |s| set.states[s].first as u8);
            let mut w2 = w.clone();
            if !live_frames {
                w2.frame = first;
            }
            let Some((f, dx, dy)) = w2.visible(set, (0, 0)) else { continue };
            let path = set.sheet_path().to_vec();
            let rect = [f.rect.x as i16, f.rect.y as i16, f.rect.w as i16, f.rect.h as i16];
            if let Some(tex) = self.texture(&path) {
                blit(canvas, width, height, tex, rect, dx + shift.0, dy + shift.1);
                drawn += 1;
            }
        }
        drawn
    }

    /// Decodifica de antemano, en paralelo, las texturas de los fotogramas `(juego, rumbo, fotograma)` que después dibujarán
    /// `draw_lcr_frame` o `draw_lcr_frame_faded` (solo las de los fotogramas que existen, como hacen ellas).
    pub fn preload_lcr_frames(&mut self, frames: &[(&p3_assets::lcr::Lcr, u8, u8)]) {
        let mut keys: Vec<Vec<u8>> = Vec::new();
        for &(set, heading, frame) in frames {
            if set.dirs.get(set.direction_of(heading)).and_then(|f| f.get(usize::from(frame))).is_some() {
                let key = fold(set.sheet_path());
                if !keys.contains(&key) {
                    keys.push(key);
                }
            }
        }
        self.preload_textures(&keys);
    }

    /// Dibuja un fotograma de un juego de sprites (`.lcr`) con el ancla en `(x, y)`: dirección por rumbo (`0x559260`) y desplazamiento propio del fotograma.
    pub fn draw_lcr_frame(&mut self, canvas: &mut [u8], width: usize, height: usize, set: &p3_assets::lcr::Lcr, heading: u8, frame: u8, x: i32, y: i32) -> bool {
        let Some(frames) = set.dirs.get(set.direction_of(heading)) else { return false };
        let Some(f) = frames.get(usize::from(frame)) else { return false };
        let path = set.sheet_path().to_vec();
        let rect = [f.rect.x as i16, f.rect.y as i16, f.rect.w as i16, f.rect.h as i16];
        match self.texture(&path) {
            Some(tex) => {
                blit_alpha(canvas, width, height, tex, rect, x + i32::from(f.rect.dx), y + i32::from(f.rect.dy));
                true
            }
            None => false,
        }
    }

    /// `draw_lcr_frame` con opacidad global `fade` (el byte `+0x30` de los efectos tipo 18).
    #[allow(clippy::too_many_arguments)]
    pub fn draw_lcr_frame_faded(&mut self, canvas: &mut [u8], width: usize, height: usize, set: &p3_assets::lcr::Lcr, heading: u8, frame: u8, x: i32, y: i32, fade: u8) -> bool {
        let Some(frames) = set.dirs.get(set.direction_of(heading)) else { return false };
        let Some(f) = frames.get(usize::from(frame)) else { return false };
        let path = set.sheet_path().to_vec();
        let rect = [f.rect.x as i16, f.rect.y as i16, f.rect.w as i16, f.rect.h as i16];
        match self.texture(&path) {
            Some(tex) => {
                blit_faded(canvas, width, height, tex, rect, x + i32::from(f.rect.dx), y + i32::from(f.rect.dy), fade);
                true
            }
            None => false,
        }
    }

    /// Textura de una ruta del archivo (con su caché), para quienes componen encima del lienzo.
    pub fn texture_of(&mut self, path: &[u8]) -> Option<&Texture> {
        self.texture(path)
    }
    pub fn sheet_of(&self, tile: u16) -> Option<&LetSheet> {
        self.sheets.get(&tile)
    }
    /// Objetos elegidos a la espera de `apply_objects`: `(x, y, ficha)`.
    pub fn placed_objects(&self) -> &[(i32, i32, u16)] {
        &self.placed
    }

    fn texture(&mut self, path: &[u8]) -> Option<&Texture> {
        let key = fold(path);
        if !self.textures.contains_key(&key) {
            let tex = self
                .last_kept
                .get(&key)
                .map(|&i| &self.archive.entries[i])
                .and_then(|e| self.archive.read(e))
                .and_then(|d| decode_texture(&key, d));
            if tex.is_none() {
                self.unsupported_textures += 1;
            }
            self.textures.insert(key.clone(), tex);
        }
        self.textures.get(&key).and_then(|t| t.as_ref())
    }

    /// Carga en la caché las texturas de los nombres plegados `keys` que aún no estén, decodificándolas en paralelo.
    /// Deja la caché y el contador `unsupported_textures` igual que una llamada a `texture` por cada nombre.
    fn preload_textures(&mut self, keys: &[Vec<u8>]) {
        let mut missing: Vec<&Vec<u8>> = Vec::new();
        for k in keys {
            if !self.textures.contains_key(k) && !missing.contains(&k) {
                missing.push(k);
            }
        }
        let archive = self.archive;
        let data: Vec<Option<&'a [u8]>> = missing.iter().map(|k| self.last_kept.get(*k).map(|&i| &archive.entries[i]).and_then(|e| archive.read(e))).collect();
        let decoded = city_fast::parallel_map(missing.len(), |i| data[i].and_then(|d| decode_texture(missing[i], d)));
        let missing: Vec<Vec<u8>> = missing.into_iter().cloned().collect();
        for (key, tex) in missing.into_iter().zip(decoded) {
            if tex.is_none() {
                self.unsupported_textures += 1;
            }
            self.textures.insert(key, tex);
        }
    }

    /// [`Scene::render`] aprovechando las texturas ya decodificadas de `donor` (otra escena del mismo archivo), que se le
    /// devuelven al acabar sin las que esta escena haya cargado de más: `donor` queda como estaba.
    pub fn render_with_textures_of(&mut self, donor: &mut Scene<'a>, map: &TownMap, layers: &[usize]) -> (usize, usize, Vec<u8>) {
        let own = std::mem::replace(&mut self.textures, std::mem::take(&mut donor.textures));
        let lent: std::collections::HashSet<Vec<u8>> = self.textures.keys().cloned().collect();
        let out = self.render(map, layers);
        let mut back = std::mem::replace(&mut self.textures, own);
        back.retain(|k, _| lent.contains(k));
        donor.textures = back;
        out
    }

    /// Dibuja `layers` (0 = suelo, 1 = objetos) del mapa en un lienzo RGBA del tamaño completo de la rejilla.
    ///
    /// Primero se recorre la rejilla y se anota qué se dibuja y en qué orden; después se decodifican las texturas que falten
    /// y se compone el lienzo por franjas (ver [`city_fast`]). El resultado es el mismo que dibujar casilla a casilla.
    pub fn render(&mut self, map: &TownMap, layers: &[usize]) -> (usize, usize, Vec<u8>) {
        use city_fast::Op;
        let width = (map.width as i32 * TILE_W + TILE_W) as usize;
        let height = ((map.height as i32 / 2 + 2) * (TILE_H + 1) + TILE_H) as usize;
        let shift = (TILE_W, TILE_H);
        // Nombres plegados de las texturas en orden de primer uso, y ruta tal cual → índice.
        let mut keys: Vec<Vec<u8>> = Vec::new();
        let mut key_index: HashMap<Vec<u8>, usize> = HashMap::new();
        let mut path_index: HashMap<Vec<u8>, usize> = HashMap::new();
        let mut intern = |path: &[u8]| -> usize {
            if let Some(&i) = path_index.get(path) {
                return i;
            }
            let key = fold(path);
            let i = *key_index.entry(key.clone()).or_insert_with(|| {
                keys.push(key);
                keys.len() - 1
            });
            path_index.insert(path.to_vec(), i);
            i
        };
        // Animadores visibles por casilla de origen y ficha, en su orden.
        let mut overlays: HashMap<((i32, i32), u16), Vec<usize>> = HashMap::new();
        for (i, a) in self.animators.iter().enumerate() {
            if a.shown.is_some() {
                overlays.entry((a.cell, a.sheet)).or_default().push(i);
            }
        }
        let mut ops: Vec<Op> = Vec::new();
        for &layer in layers {
            for y in 0..map.height as i32 {
                for x in 0..map.width as i32 {
                    let cell = map.layers[layer][(y as usize) * map.width as usize + x as usize];
                    if cell.tile == 0 {
                        continue;
                    }
                    let Some(sheet) = self.sheets.get(&cell.tile) else {
                        self.missing_sheets += 1;
                        continue;
                    };
                    // Cada casilla dibuja su propio fotograma `b2` (0xFF = relleno de la huella, no se dibuja):
                    // `sprite_sheet_draw_frame` (0x00551C80), destino (x [+ancho si b2 >= a], y - alto).
                    if cell.b2 == 0xFF {
                        continue;
                    }
                    let Some(frame) = sheet.strips.get(usize::from(cell.b2)).copied() else {
                        self.bad_frames += 1;
                        continue;
                    };
                    let origin = cell.b2 == sheet.strips_a.wrapping_sub(1);
                    let (px, py) = cell_to_screen(x, y);
                    let [_, _, dx, dy, _, _] = frame_placement(frame.map(|v| v as u16), sheet.strips_a, px + shift.0, py + shift.1, cell.b2);
                    ops.push(Op { tex: intern(&sheet.source), frame, dx, dy });
                    if layer == 1 && origin {
                        // Fotogramas visibles de las animaciones del objeto con origen en esta casilla (`0x585770`): destino
                        // `(px + f0 - sx, py + f1 - alto - sy)` con la tira `a - 1` de la hoja y el rectángulo del fotograma del trozo.
                        let (px, py) = (px + shift.0, py + shift.1);
                        for &i in overlays.get(&((x, y), cell.tile)).map_or(&[][..], |v| v) {
                            let a = &self.animators[i];
                            let Some(d) = a.shown else { continue };
                            let Some(sheet) = self.sheets.get(&a.sheet) else { continue };
                            let chunk = &sheet.chunks[a.chunks[d.anim]];
                            let Some(strip) = sheet.strips.get(usize::from(sheet.strips_a.wrapping_sub(1))).copied() else { continue };
                            let Some(frame) = chunk.frames.get(usize::from(d.frame)).copied() else { continue };
                            let Some(path) = chunk.texture.as_ref().map(|t| &t.path) else { continue };
                            let dx = px + i32::from(chunk.f0) - i32::from(strip[0]);
                            let dy = py + i32::from(chunk.f1) - i32::from(strip[3]) - i32::from(strip[1]);
                            ops.push(Op { tex: intern(path), frame, dx, dy });
                        }
                    }
                }
            }
        }
        self.preload_textures(&keys);
        let texs: Vec<Option<&Texture>> = keys.iter().map(|k| self.textures.get(k).and_then(|t| t.as_ref())).collect();
        let mut canvas = vec![0u8; width * height * 4];
        let runs = city_fast::compose(&mut canvas, width, height, &ops, &texs);
        city_fast::fill_holes_runs(&mut canvas, width, height, &runs);
        (width, height, canvas)
    }
}

/// Decodifica la textura de una entrada del archivo según la extensión de su nombre plegado (`.tga`, `.bmp` o AIM).
fn decode_texture(key: &[u8], d: &[u8]) -> Option<Texture> {
    let lower = key.to_ascii_lowercase();
    if lower.ends_with(b".tga") {
        decode_tga(d).map(|(width, height, rgba)| Texture { width, height, rgba })
    } else if lower.ends_with(b".bmp") {
        // Convención del juego: el magenta (`FF 00 FF`) es la clave de color (en `warnzeichen_tp.bmp` es el índice de paleta más frecuente).
        decode_bmp(d).map(|(width, height, mut rgba)| {
            for p in rgba.chunks_exact_mut(4) {
                if p[..3] == [255, 0, 255] {
                    p[3] = 0;
                }
            }
            Texture { width, height, rgba }
        })
    } else {
        aim::decode(d).ok().and_then(|i| to_rgba(&i))
    }
}

/// Rellena los huecos negros pequeños y rodeados del terreno (celdas que el mapa deja sin fotograma, p. ej. los pies de
/// acantilados) copiando el color de la baldosa vecina más cercana (desplazamientos de 1 a 3 baldosas). No es lo que dibuja
/// el original, que no se ha reconstruido: solo evita rombos negros dentro de la ciudad. Los vacíos grandes o pegados al
/// borde del lienzo (fuera del mapa) no se tocan.
pub fn fill_holes(canvas: &mut [u8], width: usize, height: usize) {
    let mut runs = Vec::new();
    city_fast::black_runs(&canvas[..width * height * 4], width, 0, &mut runs);
    city_fast::fill_holes_runs(canvas, width, height, &runs);
}

/// Como `blit`, pero mezcla por el alfa de la textura (nieve con borde suave).
fn blit_alpha(canvas: &mut [u8], cw: usize, ch: usize, tex: &Texture, frame: [i16; 4], dx: i32, dy: i32) {
    blit_faded(canvas, cw, ch, tex, frame, dx, dy, 255)
}

/// Como `blit_alpha`, con una opacidad global `fade` (0..=255) que multiplica el alfa de la textura (efectos con `+0x30`).
fn blit_faded(canvas: &mut [u8], cw: usize, ch: usize, tex: &Texture, frame: [i16; 4], dx: i32, dy: i32, fade: u8) {
    city_fast::blit_faded(canvas, cw, ch, tex, frame, dx, dy, fade)
}

fn blit(canvas: &mut [u8], cw: usize, ch: usize, tex: &Texture, frame: [i16; 4], dx: i32, dy: i32) {
    let n = (cw * ch * 4).min(canvas.len());
    city_fast::blit_band(&mut canvas[..n], cw, 0, tex, frame, dx, dy)
}

/// Decodifica un TGA sin compresión (tipo 2) de 24 o 32 bits a RGBA; el bit 5 del descriptor indica origen arriba a la izquierda.
pub fn decode_tga(data: &[u8]) -> Option<(usize, usize, Vec<u8>)> {
    if data.len() < 18 || data[1] != 0 || data[2] != 2 {
        return None;
    }
    let (id_len, width, height, bpp, desc) = (usize::from(data[0]), usize::from(u16::from_le_bytes([data[12], data[13]])), usize::from(u16::from_le_bytes([data[14], data[15]])), usize::from(data[16]), data[17]);
    if (bpp != 24 && bpp != 32) || width == 0 || height == 0 {
        return None;
    }
    let step = bpp / 8;
    let body = data.get(18 + id_len..18 + id_len + width * height * step)?;
    let mut rgba = vec![0u8; width * height * 4];
    for y in 0..height {
        let src_row = if desc & 0x20 != 0 { y } else { height - 1 - y };
        for x in 0..width {
            let p = &body[(src_row * width + x) * step..];
            let o = (y * width + x) * 4;
            rgba[o..o + 4].copy_from_slice(&[p[2], p[1], p[0], if step == 4 { p[3] } else { 255 }]);
        }
    }
    Some((width, height, rgba))
}

/// Decodifica un BMP sin compresión de 8 bits con paleta o de 24 bits (como `images/vollansichtskarte.bmp`) a RGBA.
pub fn decode_bmp(data: &[u8]) -> Option<(usize, usize, Vec<u8>)> {
    let u16le = |o: usize| data.get(o..o + 2).map(|b| u16::from_le_bytes([b[0], b[1]]));
    let u32le = |o: usize| data.get(o..o + 4).map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]));
    if data.get(0..2)? != b"BM" || u32le(14)? < 40 || u32le(30)? != 0 {
        return None;
    }
    let (offset, header) = (u32le(10)? as usize, u32le(14)? as usize);
    let width = u32le(18)? as i32;
    let raw_height = u32le(22)? as i32;
    let (bits, colors) = (u16le(28)?, u32le(46)? as usize);
    let (w, h) = (width.unsigned_abs() as usize, raw_height.unsigned_abs() as usize);
    if width <= 0 || w == 0 || h == 0 || w > 16384 || h > 16384 {
        return None;
    }
    let bottom_up = raw_height > 0;
    let bytes_per_pixel = match bits {
        8 | 24 => usize::from(bits / 8),
        _ => return None,
    };
    let stride = (w * bytes_per_pixel + 3) & !3;
    let palette_at = 14 + header;
    let count = if colors == 0 { 256 } else { colors.min(256) };
    let mut rgba = vec![255u8; w * h * 4];
    for row in 0..h {
        let src = offset + (if bottom_up { h - 1 - row } else { row }) * stride;
        let line = data.get(src..src + w * bytes_per_pixel)?;
        for x in 0..w {
            let (b, g, r, a) = if bits == 8 {
                let i = usize::from(line[x]);
                if i >= count {
                    return None;
                }
                let p = data.get(palette_at + 4 * i..palette_at + 4 * i + 4)?;
                // AIM.dll (`AIM_CONVERT_MEMFILE`, comprobado en `oracle/x86_sgl_load.py`) toma el alfa de cada entrada de la paleta
                // como `255 - reservado`: los BMP de las mareas (`Klippen_Flut.bmp`) marcan así su magenta como transparente.
                (p[0], p[1], p[2], 255 - p[3])
            } else {
                (line[3 * x], line[3 * x + 1], line[3 * x + 2], 255)
            };
            rgba[(row * w + x) * 4..(row * w + x) * 4 + 4].copy_from_slice(&[r, g, b, a]);
        }
    }
    Some((w, h, rgba))
}

pub fn write_png(path: &std::path::Path, width: usize, height: usize, rgba: &[u8]) -> std::io::Result<()> {
    let file = std::io::BufWriter::new(std::fs::File::create(path)?);
    let mut enc = png::Encoder::new(file, width as u32, height as u32);
    enc.set_color(png::ColorType::Rgba);
    enc.set_depth(png::BitDepth::Eight);
    let mut w = enc.write_header().map_err(std::io::Error::other)?;
    w.write_image_data(rgba).map_err(std::io::Error::other)
}

#[cfg(test)]
mod bmp_tests {
    use super::decode_bmp;
    fn bmp(bits: u16, w: i32, h: i32, palette: &[[u8; 4]], pixels: &[u8]) -> Vec<u8> {
        let off = 54 + palette.len() * 4;
        let mut v = b"BM".to_vec();
        v.extend(((off + pixels.len()) as u32).to_le_bytes());
        v.extend([0u8; 4]);
        v.extend((off as u32).to_le_bytes());
        v.extend(40u32.to_le_bytes());
        v.extend(w.to_le_bytes());
        v.extend(h.to_le_bytes());
        v.extend(1u16.to_le_bytes());
        v.extend(bits.to_le_bytes());
        v.extend([0u8; 24]);
        for p in palette { v.extend(p); }
        v.extend(pixels);
        v
    }
    #[test]
    fn decodes_8_bit_bottom_up_with_row_padding() {
        // 3x2, filas de 4 bytes (relleno). Paleta BGRA: 0 = rojo, 1 = verde.
        let data = bmp(8, 3, 2, &[[0, 0, 255, 0], [0, 255, 0, 0]], &[1, 1, 1, 0, 0, 0, 1, 0]);
        let (w, h, rgba) = decode_bmp(&data).unwrap();
        assert_eq!((w, h), (3, 2));
        // la primera fila del archivo (1,1,1) es la inferior; la superior es (0,0,1)
        assert_eq!(&rgba[..4], &[255, 0, 0, 255]);
        assert_eq!(&rgba[2 * 4..3 * 4], &[0, 255, 0, 255]);
        assert_eq!(&rgba[3 * 4..4 * 4], &[0, 255, 0, 255]);
    }
    #[test]
    fn decodes_24_bit_and_rejects_garbage() {
        let data = bmp(24, 1, 1, &[], &[10, 20, 30, 0]);
        assert_eq!(decode_bmp(&data).unwrap().2, vec![30, 20, 10, 255]);
        assert!(decode_bmp(b"nope").is_none());
    }
    #[test]
    fn palette_reserved_byte_is_inverted_alpha() {
        // Como AIM.dll: alfa = 255 - byte reservado de cada entrada (0 -> opaco, 0xFF -> transparente, 0x80 -> 0x7F).
        let data = bmp(8, 4, 1, &[[255, 0, 255, 0xFF], [1, 2, 3, 0], [4, 5, 6, 0x80], [7, 8, 9, 0x01]], &[0, 1, 2, 3]);
        let rgba = decode_bmp(&data).unwrap().2;
        assert_eq!(&rgba[0..4], &[255, 0, 255, 0]);
        assert_eq!(&rgba[4..8], &[3, 2, 1, 255]);
        assert_eq!(&rgba[8..12], &[6, 5, 4, 0x7F]);
        assert_eq!(&rgba[12..16], &[9, 8, 7, 0xFE]);
    }
}

#[cfg(test)]
mod hole_tests {
    use super::fill_holes;
    fn canvas(w: usize, h: usize, rgb: [u8; 3]) -> Vec<u8> {
        (0..w * h).flat_map(|_| [rgb[0], rgb[1], rgb[2], 255]).collect()
    }
    #[test]
    fn fills_small_interior_hole_but_not_the_border_void() {
        let (w, h) = (400, 300);
        let mut c = canvas(w, h, [10, 120, 30]);
        for y in 130..150 { for x in 190..230 { c[(y * w + x) * 4..][..3].copy_from_slice(&[0, 0, 0]); } }
        for y in 0..60 { for x in 0..60 { c[(y * w + x) * 4..][..3].copy_from_slice(&[0, 0, 0]); } }
        fill_holes(&mut c, w, h);
        assert_eq!(&c[(140 * w + 200) * 4..][..3], &[10, 120, 30]);
        assert_eq!(&c[(30 * w + 30) * 4..][..3], &[0, 0, 0]);
    }
}
