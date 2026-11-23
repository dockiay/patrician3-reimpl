//! Edificios en obra o en destrucción (J02): compone sobre el lienzo ya dibujado lo que hace `0x550770` (silueta oscura del paquete de la etiqueta 1 y revelado
//! del sprite de abajo arriba con el borde difuminado) más las llamas de los paquetes globales de las etiquetas 3 y 4, ejecutando el código portado
//! (`fire_state::update`/`draw_fire`, `fire_pack::load`) sobre una memoria plana con la disposición de la estructura de la hoja.
//!
//! No es una reconstrucción validada en juego: la región del sprite se obtiene comparando el lienzo con y sin el edificio (no se porta el recorte del
//! original), `rect` es el rectángulo completo de esa región y el estado inicial de un edificio quemándose (signo `0xFE`) es una suposición.
use crate::fire_pack::{self, PackEnv};
use crate::fire_state::{self, FireEnv, Gfx, PlaceEnv};
use crate::scene_math::Mem;
use crate::Texture;
use p3_assets::let_sheet::LetSheet;
use std::collections::HashMap;

#[derive(Default)]
pub struct FlatMem(pub HashMap<u32, u8>);
impl Mem for FlatMem {
    fn r8(&mut self, a: u32) -> u8 {
        *self.0.get(&a).unwrap_or(&0)
    }
    fn w8(&mut self, a: u32, v: u8) {
        self.0.insert(a, v);
    }
}

struct Heap(u32);
impl PackEnv for Heap {
    fn alloc(&mut self, n: u32) -> u32 {
        let a = self.0;
        self.0 += (n + 15) & !15;
        a
    }
    fn free(&mut self, _: u32) {}
    fn surface(&mut self, _data: u32, _slot: u32, _a: u32, _b: u32) {}
}

pub const LARGE: u32 = 0x006E_11E8;
pub const SMALL: u32 = 0x006E_1200;
/// Asas de superficie: 1 hoja principal del edificio, 2 paquete de la etiqueta 1, 3 llamas grandes, 4 llamas pequeñas.
const H_MAIN: u32 = 1;
const H_PACK: u32 = 2;
const H_LARGE: u32 = 3;
const H_SMALL: u32 = 4;

/// Datos de un `.let` que aportan los trozos del fuego (etiquetas 3 y 4) y los paquetes por edificio.
pub struct FireArt {
    pub large: Option<Texture>,
    pub small: Option<Texture>,
    mem: FlatMem,
}

/// Lienzo con las primitivas del juego.
struct CanvasGfx<'a> {
    canvas: &'a mut [u8],
    cw: usize,
    ch: usize,
    surfaces: HashMap<u32, &'a Texture>,
    cur: u32,
    color: u32,
}
impl Gfx for CanvasGfx<'_> {
    fn select(&mut self, surface: u32) {
        self.cur = surface;
    }
    fn color(&mut self) -> u32 {
        self.color
    }
    fn set_color(&mut self, c: u32) {
        self.color = c;
    }
    fn blit(&mut self, sx: i32, sy: i32, dx: i32, dy: i32, w: i32, h: i32) {
        if std::env::var("P3_PROGRESS_DEBUG").is_ok() {
            eprintln!("blit surf={} s=({sx},{sy}) d=({dx},{dy}) {w}x{h} color={:08x}", self.cur, self.color);
        }
        let Some(tex) = self.surfaces.get(&self.cur) else { return };
        let (ca, cr, cg, cb) = ((self.color >> 24) & 0xFF, (self.color >> 16) & 0xFF, (self.color >> 8) & 0xFF, self.color & 0xFF);
        for j in 0..h {
            for i in 0..w {
                let (x, y, tx, ty) = (sx + i, sy + j, dx + i, dy + j);
                if x < 0 || y < 0 || x as usize >= tex.width || y as usize >= tex.height || tx < 0 || ty < 0 || tx as usize >= self.cw || ty as usize >= self.ch {
                    continue;
                }
                let s = (y as usize * tex.width + x as usize) * 4;
                let a = u32::from(tex.rgba[s + 3]) * ca / 255;
                if a == 0 {
                    continue;
                }
                let d = (ty as usize * self.cw + tx as usize) * 4;
                for (k, m) in [cr, cg, cb].into_iter().enumerate() {
                    let src = u32::from(tex.rgba[s + k]) * m / 255;
                    self.canvas[d + k] = ((u32::from(self.canvas[d + k]) * (255 - a) + src * a) / 255) as u8;
                }
                self.canvas[d + 3] = 255;
            }
        }
    }
}

struct Env<'a> {
    rng: &'a mut crate::anim::CrtRand,
    mem: *mut FlatMem,
}
impl FireEnv for Env<'_> {
    fn f5506d0(&mut self, x: u32, y: u32, k: u8, p8: u32) {
        struct Nop;
        impl PlaceEnv for Nop {
            fn clip(&mut self, _: [i32; 4]) {}
        }
        // SAFETY: `mem` apunta a la memoria viva de la llamada en curso; `update` no la mantiene prestada durante el gancho.
        fire_state::place_tables(unsafe { &mut *self.mem }, x, y, k, p8, &mut Nop);
    }
    fn rand(&mut self) -> u32 {
        self.rng.next()
    }
    fn repaint(&mut self, _x: u32, _y: u32) {}
}

pub struct Request {
    /// Pixel del origen del edificio (semilla para hallar su región en la diferencia).
    pub origin: (i32, i32),
    pub tile: u16,
    pub a: u32,
    pub b: u32,
    pub burning: bool,
}

fn region(full: &[u8], base: &[u8], w: usize, h: usize, seed: (i32, i32)) -> Option<Vec<usize>> {
    let diff = |i: usize| full[i * 4..i * 4 + 3] != base[i * 4..i * 4 + 3];
    let mut start = None;
    'outer: for r in 0..60i32 {
        for dy in -r..=r {
            for dx in -r..=r {
                if dx.abs().max(dy.abs()) != r {
                    continue;
                }
                let (x, y) = (seed.0 + dx, seed.1 + dy);
                if x >= 0 && y >= 0 && (x as usize) < w && (y as usize) < h && diff(y as usize * w + x as usize) {
                    start = Some(y as usize * w + x as usize);
                    break 'outer;
                }
            }
        }
    }
    let s = start?;
    let mut seen = vec![false; w * h];
    let mut out = vec![s];
    seen[s] = true;
    let mut k = 0;
    while k < out.len() {
        let i = out[k];
        k += 1;
        let (x, y) = ((i % w) as i32, (i / w) as i32);
        for dy in -2..=2i32 {
            for dx in -2..=2i32 {
                let (nx, ny) = (x + dx, y + dy);
                if nx < 0 || ny < 0 || nx as usize >= w || ny as usize >= h {
                    continue;
                }
                let j = ny as usize * w + nx as usize;
                if !seen[j] && diff(j) {
                    seen[j] = true;
                    out.push(j);
                }
            }
        }
    }
    Some(out)
}

impl FireArt {
    /// Carga los paquetes globales de fuego del primer `.let` con trozos de etiquetas 3 y 4 (como el cargador de clases `0x54E310`).
    pub fn load(sheets: &[(&[u8], LetSheet)], mut tex: impl FnMut(&[u8]) -> Option<Texture>) -> Option<FireArt> {
        let (data, sheet) = sheets.iter().find(|(_, s)| s.chunks.iter().any(|c| c.tag == 3) && s.chunks.iter().any(|c| c.tag == 4))?;
        let streams = LetSheet::chunk_streams(data).ok()?;
        let mut mem = FlatMem::default();
        let base = 0x1000_0000u32;
        for (i, b) in data.iter().enumerate() {
            mem.w8(base + i as u32, *b);
        }
        let mut heap = Heap(0x4000_0000);
        let mut out = [None, None];
        for (tag, addr, slot) in [(3u32, LARGE, 0), (4, SMALL, 1)] {
            let (idx, (_, off)) = streams.iter().enumerate().find(|(_, (t, _))| *t == tag)?;
            let cur = 0x2000_0000 + 4 * slot as u32;
            mem.w32(cur, base + *off as u32);
            fire_pack::ctor(&mut mem, addr);
            fire_pack::load(&mut mem, addr, cur, 0, 0x300_0000, &mut heap);
            out[slot] = sheet.chunks[idx].texture.as_ref().and_then(|t| tex(&t.path));
        }
        let [large, small] = out;
        Some(FireArt { large, small, mem })
    }

    /// Compone `requests` sobre `full` (lienzo con los edificios) usando `base` (lienzo sin ellos). Devuelve cuántos edificios se recompusieron.
    pub fn compose(&self, full: &mut [u8], base: &[u8], w: usize, h: usize, requests: &[Request], sheets: &HashMap<u16, (&[u8], &LetSheet)>, mut tex: impl FnMut(&[u8]) -> Option<Texture>, rng: &mut crate::anim::CrtRand) -> usize {
        let mut done = 0;
        for r in requests {
            let Some(&(data, sheet)) = sheets.get(&r.tile) else { continue };
            let Some(pix) = region(full, base, w, h, r.origin) else { continue };
            let (mut x0, mut y0, mut x1, mut y1) = (usize::MAX, usize::MAX, 0, 0);
            for &i in &pix {
                let (x, y) = (i % w, i / w);
                x0 = x0.min(x);
                y0 = y0.min(y);
                x1 = x1.max(x);
                y1 = y1.max(y);
            }
            let (rw, rh) = (x1 - x0 + 1, y1 - y0 + 1);
            let mut main = Texture { width: rw, height: rh, rgba: vec![0; rw * rh * 4] };
            for &i in &pix {
                let (x, y) = (i % w - x0, i / w - y0);
                main.rgba[(y * rw + x) * 4..(y * rw + x) * 4 + 4].copy_from_slice(&[full[i * 4], full[i * 4 + 1], full[i * 4 + 2], 255]);
                full[i * 4..i * 4 + 4].copy_from_slice(&base[i * 4..i * 4 + 4]);
            }
            let Some(chunk) = sheet.chunks.iter().find(|c| c.tag == 1) else { continue };
            let Some(pack_tex) = chunk.texture.as_ref().and_then(|t| tex(&t.path)) else { continue };
            // Memoria: tablas de fuego globales + estructura de la hoja + paquete de la etiqueta 1 cargado con el lector del original.
            let mut m = FlatMem(self.mem.0.clone());
            let (obj, pack, state, rect, pa, pb, px, py) = (0x5000_0000u32, 0x5000_1000u32, 0x5000_2000u32, 0x5000_3000u32, 0x5000_3100u32, 0x5000_3110u32, 0x5000_3120u32, 0x5000_3130u32);
            let Ok(streams) = LetSheet::chunk_streams(data) else { continue };
            let Some(idx) = sheet.chunks.iter().position(|c| c.tag == 1) else { continue };
            let base_in = 0x1800_0000u32;
            for (i, b) in data.iter().enumerate() {
                m.w8(base_in + i as u32, *b);
            }
            let cur = 0x5000_3200;
            m.w32(cur, base_in + streams[idx].1 as u32);
            fire_pack::ctor(&mut m, pack);
            fire_pack::load(&mut m, pack, cur, 0, 0x300_0000, &mut Heap(0x4100_0000));
            m.w32(obj + 0xC, H_MAIN);
            m.w32(obj + 0x18, pack);
            m.w16(pack + 0xC, H_PACK as u16);
            m.w32(pack + 0xC, H_PACK);
            m.w16(obj + 0x24, sheet.field_24 as u16);
            m.w16(obj + 0x28, sheet.tile_height as u16);
            m.w8(obj + 0x2A, sheet.strips_a);
            m.w8(obj + 0x2B, sheet.strips_b);
            let strips = 0x5000_4000u32;
            m.w32(obj + 0x10, strips);
            for (i, f) in sheet.strips.iter().enumerate() {
                for k in 0..4 {
                    m.w16(strips + 8 * i as u32 + 2 * k, f[k as usize] as u16);
                }
            }
            for (k, v) in [0u16, 0, rw as u16, rh as u16].into_iter().enumerate() {
                m.w16(rect + 2 * k as u32, v);
            }
            m.w32(pa, r.a);
            m.w32(pb, r.b);
            m.w32(px, x0 as u32);
            m.w32(py, (y0 + rh) as u32);
            m.w16(state + 4, 0);
            m.w16(state + 6, 0);
            // Signo `0xFE` (< -1): con `draw_fire` revela el sprite sin dibujar llamas; en `update` con indicador 1 fija las marcas (y el signo) al azar.
            for k in 0..4 {
                m.w8(state + 0xC + k, 0xFF);
            }
            m.w8(state + 0xE, 0xFE);
            if std::env::var("P3_PROGRESS_DEBUG").is_ok() {
                eprintln!("progreso: a={} b={} burning={} region={rw}x{rh} f24={} th={} strips_a={}", r.a, r.b, r.burning, sheet.field_24, sheet.tile_height, sheet.strips_a);
            }
            let mp: *mut FlatMem = &mut m;
            let mut env = Env { rng, mem: mp };
            // SAFETY: `m` vive durante la llamada; el entorno solo la usa dentro de `update` mediante el puntero.
            fire_state::update(unsafe { &mut *mp }, state, pa, pb, px, py, u8::from(r.burning), obj, &mut env);
            let mut surfaces: HashMap<u32, &Texture> = HashMap::new();
            surfaces.insert(H_MAIN, &main);
            surfaces.insert(H_PACK, &pack_tex);
            if let Some(t) = &self.large {
                surfaces.insert(H_LARGE, t);
            }
            if let Some(t) = &self.small {
                surfaces.insert(H_SMALL, t);
            }
            // Las tablas de fuego usan sus asas `+0xC`.
            m.w32(LARGE + 0xC, H_LARGE);
            m.w32(SMALL + 0xC, H_SMALL);
            let mut gfx = CanvasGfx { canvas: full, cw: w, ch: h, surfaces, cur: 0, color: 0xFFFF_FFFF };
            let drew = fire_state::draw_fire(&mut m, state, obj, rect, px, py, &mut gfx);
            if std::env::var("P3_PROGRESS_DEBUG").is_ok() {
                eprintln!("array={:?} pack f0={} f1={} large tab ox={} oy={} nrec={} piece0={:?} rect={rw}x{rh}", chunk.array, chunk.f0, chunk.f1, m.r16(LARGE + 0x10) as i16, m.r16(LARGE + 0x12) as i16, m.r16(LARGE + 0x14), { let t = m.r32(LARGE + 4); (m.r16(t), m.r16(t + 2), m.r16(t + 4), m.r16(t + 6)) });
                eprintln!("draw_fire={drew} cur={} tgt={} sign={} marks={:?}", m.r16(state + 4), m.r16(state + 6), m.r8(state + 0xE) as i8, (0..4).map(|k| m.r8(state + 0xC + k)).collect::<Vec<_>>());
            }
            done += 1;
        }
        done
    }
}
