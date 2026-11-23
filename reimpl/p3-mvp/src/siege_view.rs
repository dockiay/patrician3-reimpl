//! C08: vista de asedio de la ciudad actual en el cliente.
//!
//! [`Session::open_siege_view`] abre la vista del asedio en curso con la rejilla real de la ciudad: capas 0 y 1 del
//! `.town` con los edificios colocados (las que dibuja `render_fx`), clase de cada ficha (`LetSheet::field_2d`,
//! método virtual `+0x50`), puntos de reunión (clave 4 del `.spawn`) y caminos de puertas (claves 1, 2 y 3 del
//! `.nodes`). La geometría es la de las figuras (`p3_render::geom::Geom` de 34 × 21 con la máscara de casillas).
//! Desde ese momento las órdenes `0x90` de la ciudad (libertad, rendición, ataque) recorren los portes con paridad
//! x86 de `p3_core::economy::siege_view` dentro de `p3-sim` y los marcadores de los puntos de entrada se dibujan
//! encima de la ciudad (efectos de la clase 18 con código `0x80` y el subtipo de la variante, como `0x0057A150`).
//!
//! Límites: ver [`p3_sim::siege_view_live`]. Además, los marcadores se dibujan en su primer fotograma de la
//! animación 2 y opacos; los que se están apagando no se dibujan (el fundido `0x0057AF40` no avanza con el reloj).
use p3_assets::cpr::Archive;
use p3_render::geom::{detect_mask, Geom};
use p3_sim::siege_view_live::{SiegeTownView, SiegeViewClient, ViewMap};
use p3_core::economy::siege_figures::{sets, FigureHost};
use serde_json::{json, Value};

use crate::Session;

/// Geometría de la vista en el cliente: la misma que usan las figuras de la ciudad.
struct GeomClient {
    geom: Geom,
    mask: Vec<u8>,
}

impl SiegeViewClient for GeomClient {
    fn step(&mut self, col: &mut u32, row: &mut u32, dir: u8) -> bool {
        self.geom.step(col, row, dir)
    }
    fn pixel_to_cell(&mut self, x: i32, y: i32) -> (u32, u32) {
        self.geom.pixel_to_cell(x, y, &self.mask).0
    }
    fn cell_to_pixel(&mut self, col: u32, row: u32) -> (i32, i32) {
        self.geom.cell_to_pixel(col as i32, row)
    }
}

/// Clase de los efectos de los marcadores y código que eligen (`0x0062DD70` pide `0x80 | subtipo << 8`).
const MARKER_CLASS: u32 = 0x12;
const MARKER_CODE: u32 = 0x80;

impl Session {
    /// Geometría de la vista de la ciudad actual (`+0x510/+0x514`, `+0x4D0/+0x4D4`), como la de las figuras.
    fn view_geom(width: u32, height: u32) -> Geom {
        Geom { cols: width, rows: height, cell_w: 34, cell_h: 21, off_x: 17, off_y: 10 }
    }

    /// Abre la vista de asedio de la ciudad actual (ver el módulo). Devuelve el estado de [`Self::siege_view_state`].
    pub fn open_siege_view(&mut self) -> Result<Value, String> {
        let view = {
            let archive = Archive::parse(&self.archive).map_err(|e| format!("CPR: {e}"))?;
            let (map, scene, id) = self.placed_town_map(&archive)?;
            let layers: [Vec<(u16, u8, u8)>; 2] = std::array::from_fn(|k| map.layers[k].iter().map(|c| (c.tile, c.b2, c.b3)).collect());
            let mut kinds = std::collections::BTreeMap::new();
            for layer in &map.layers {
                for c in layer {
                    if let std::collections::btree_map::Entry::Vacant(e) = kinds.entry(c.tile) {
                        if let Some(k) = scene.tile_kind(c.tile) {
                            e.insert(k);
                        }
                    }
                }
            }
            let file = |ext: &str| archive.find(format!("iso/towns/{id}.{ext}").as_bytes()).and_then(|e| archive.read(e));
            let spawn = file("spawn").map(p3_assets::town::parse_spawn).transpose().map_err(|e| format!("spawn: {e:?}"))?.unwrap_or_default();
            let rally = spawn.iter().find(|s| s.key == 4).map(|s| s.points.clone()).unwrap_or_default();
            let sections = file("nodes").map(p3_assets::town::parse_nodes).transpose().map_err(|e| format!("nodes: {e:?}"))?.unwrap_or_default();
            let nodes: [Vec<Vec<(i32, i32)>>; 3] = std::array::from_fn(|k| {
                sections
                    .iter()
                    .find(|s| s.key == k as u32 + 1)
                    .map(|s| s.paths.iter().map(|p| p.iter().map(|&(x, y)| (x as i32, y as i32)).collect()).collect())
                    .unwrap_or_default()
            });
            // `0x0062C630`, `0x00631730` y `0x00630640` leen tres caminos de la clave 1, dos nodos de cada uno, el
            // último de la clave 2 y dos nodos de la clave 3, sin comprobarlo.
            if nodes[0].len() < 3 || nodes[0].iter().any(|p| p.len() < 2) || nodes[1].len() < 3 || nodes[1].iter().any(Vec::is_empty) || nodes[2].first().is_none_or(|p| p.len() < 2) {
                return Err(format!("caminos de puertas incompletos en {id}.nodes"));
            }
            let client = GeomClient { geom: Self::view_geom(map.width, map.height), mask: detect_mask() };
            let view_map = ViewMap { width: map.width, height: map.height, layers: [&layers[0], &layers[1]], kinds: &kinds, rally: &rally, nodes: [&nodes[0], &nodes[1], &nodes[2]] };
            let mut view = SiegeTownView::new(self.city, &view_map, Box::new(client))?;
            load_figure_sets(&mut view, &scene.load_figures())?;
            view
        };
        self.sim.open_siege_view(view)?;
        Ok(self.siege_view_state().expect("vista recién abierta"))
    }

    /// Cierra la vista de asedio de la ciudad actual (`0x0062F5A0`: retira las figuras); `false` si no estaba abierta.
    pub fn close_siege_view(&mut self) -> bool {
        matches!(self.sim.close_siege_view(self.city), Ok(Some(_)))
    }

    /// Avanza `frames` fotogramas de la batalla de la vista abierta (`0x00631A50`); el reloj de fotogramas
    /// (`0x006DCCF8`) avanza `step` por fotograma desde `clock`. Devuelve el estado de [`Self::siege_view_state`].
    pub fn step_siege_view(&mut self, frames: u32, clock: u32, step: u32) -> Result<Value, String> {
        for k in 0..frames {
            self.sim.step_siege_view(self.city, clock.wrapping_add(k.wrapping_mul(step)))?;
        }
        self.siege_view_state().ok_or_else(|| "vista de asedio cerrada".into())
    }

    /// Estado de la vista de asedio de la ciudad actual: casillas de la escena que fijan los portes y entidades de la vista.
    pub fn siege_view_state(&self) -> Option<Value> {
        let view = self.sim.siege_view(self.city)?;
        let scene = view.scene.as_ref()?;
        let w16 = |at: usize| u16::from_le_bytes([scene[at], scene[at + 1]]);
        let n = usize::from(scene[0x5d4]);
        let entries: Vec<Value> = (0..n.min(6)).map(|i| json!([w16(0x5aa + 2 * i), w16(0x5b6 + 2 * i)])).collect();
        let flow = view.flow_field();
        Some(json!({
            "open": scene[0x77] != 0,
            "attack_target": [w16(8), w16(0xa)],
            "approach": [w16(0x36), w16(0x38)],
            "route": scene[0x76],
            "surrender_target": [w16(0x3e), w16(0x40)],
            "entry_points": entries,
            "extra_points": [[w16(0x5c2), w16(0x5c6)], [w16(0x5c4), w16(0x5c8)]],
            "walkers_fled": view.walkers_fled,
            "heap_used": view.heap_used(),
            "frames": view.frames,
            "sounds": view.sounds,
            "unported": view.unported.iter().map(|f| format!("{f:#x}")).collect::<Vec<_>>(),
            // Cola por niveles del campo (`+0x5D8`, capacidad `+0x590`): la expansión por fotogramas (`0x00633060`) no está portada.
            "queue": {"allocated": scene[0x5d8..0x5dc] != [0; 4], "capacity": w16(0x590)},
            "flow_field": flow.as_ref().map(|f| json!({"cells": f.len(), "reached": f.iter().filter(|&&v| v != 0 && v != 0xffff).count()})),
            "entities": view.markers.values().map(|m| json!({"x": m.x, "y": m.y, "class": m.class, "variant": m.variant, "fading_out": m.fading_out(), "object": m.object, "heading": m.heading, "frame": m.frame, "set": m.set_index})).collect::<Vec<_>>(),
        }))
    }

    /// Dibuja los marcadores de los puntos de entrada de la vista de asedio abierta sobre la ciudad. `shift` pasa del
    /// píxel del mundo al lienzo, como las figuras. Devuelve cuántos se dibujaron.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn draw_siege_markers(&self, scene: &mut p3_render::Scene, figures: &std::collections::HashMap<u32, Vec<p3_assets::lcr::Lcr>>, rgba: &mut [u8], w: usize, h: usize, shift: (i32, i32)) -> usize {
        let Some(view) = self.sim.siege_view(self.city) else { return 0 };
        let sets = figures.get(&18).map(Vec::as_slice).unwrap_or(&[]);
        let matching: Vec<&p3_assets::lcr::Lcr> = sets.iter().filter(|l| l.extra_header.get(1) == Some(&MARKER_CODE)).collect();
        let mut drawn = 0;
        let mut markers: Vec<_> = view.markers.values().filter(|m| m.class == MARKER_CLASS && (m.object || (m.variant & 0xFF == MARKER_CODE && !m.fading_out()))).collect();
        markers.sort_by_key(|m| (m.y, m.x));
        for m in markers {
            // Efectos vivos: su conjunto, fotograma y opacidad (`+0x30`); el resto, el primer fotograma de la animación 2.
            if m.object {
                let Some(set) = sets.get(usize::from(m.set_index)) else { continue };
                if m.alpha != 0 && scene.draw_lcr_frame_faded(rgba, w, h, set, m.heading, m.frame, m.x + shift.0, m.y + shift.1, m.alpha) {
                    drawn += 1;
                }
                continue;
            }
            let subtype = (m.variant >> 8) & 0xFF;
            let Some(set) = subtype.checked_sub(1).and_then(|k| matching.get(k as usize)) else { continue };
            let frame = set.state_index(2).map_or(0, |s| set.states[s].first as u8);
            if scene.draw_lcr_frame_faded(rgba, w, h, set, 0, frame, m.x + shift.0, m.y + shift.1, 0xFF) {
                drawn += 1;
            }
        }
        drawn
    }

    /// Dibuja las figuras de la batalla de la vista abierta (defensores, milicia, asaltantes, cabecillas, jefe,
    /// porteadores y ariete) en su píxel y rumbo, con el primer fotograma de la animación 2 del primer juego de
    /// sprites de su clase. Las que aún no tienen posición (`(0, 0)`, la colocan sus propias rutinas, sin portar) no
    /// se dibujan. Devuelve cuántas se dibujaron.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn draw_siege_figures(&self, scene: &mut p3_render::Scene, figures: &std::collections::HashMap<u32, Vec<p3_assets::lcr::Lcr>>, rgba: &mut [u8], w: usize, h: usize, shift: (i32, i32)) -> usize {
        let Some(view) = self.sim.siege_view(self.city) else { return 0 };
        let mut fighters: Vec<_> = view.markers.values().filter(|m| m.class != MARKER_CLASS && (m.object || FIGHTER_CLASSES.contains(&m.class)) && (m.x, m.y) != (0, 0)).collect();
        fighters.sort_by_key(|m| (m.y, m.x));
        let mut drawn = 0;
        for m in fighters {
            // Las figuras vivas llevan su conjunto, rumbo y fotograma; el resto, la espera del primer conjunto.
            let set = if m.object { figures.get(&m.class).and_then(|v| v.get(usize::from(m.set_index))) } else { figures.get(&m.class).and_then(|v| v.first()) };
            let Some(set) = set else { continue };
            let frame = if m.object { m.frame } else { set.state_index(2).map_or(0, |s| set.states[s].first as u8) };
            if scene.draw_lcr_frame(rgba, w, h, set, m.heading, frame, m.x + shift.0, m.y + shift.1) {
                drawn += 1;
            }
        }
        drawn
    }
}

/// Memoria de la carga de un conjunto (`0x00558770`) en la vista: reservas a cero y tablas por clase de la cabecera
/// extra en las listas globales de la vista.
struct SetEnv<'a> {
    m: &'a mut dyn p3_core::flatmem::Mem,
    h: &'a mut dyn FigureHost,
}

impl p3_render::scene_math::Mem for SetEnv<'_> {
    fn r8(&mut self, a: u32) -> u8 {
        self.m.r8(a)
    }
    fn w8(&mut self, a: u32, v: u8) {
        self.m.w8(a, v)
    }
}

impl p3_render::sprite_set::SetMem for SetEnv<'_> {
    fn alloc(&mut self, size: u32) -> u32 {
        let p = self.h.alloc(self.m, size);
        for i in 0..size {
            self.m.w8(p + i, 0);
        }
        p
    }
    fn class_table_push(&mut self, kind: u32, table: usize, value: u32) {
        sets::push_table(self.m, self.h, kind, table, value as u8);
    }
}

/// Carga en la vista los conjuntos de sprites de las clases de figuras (método `+0x10` de cada clase: alta del
/// conjunto con `0x005629D0` y carga con `0x00558770`) en el orden de `files.lst`; los proyectiles copian además su
/// registro de 12 bytes (`0x005773C0`, palabras 1 a 3 de la cabecera extra).
fn load_figure_sets(view: &mut SiegeTownView, figures: &std::collections::HashMap<u32, Vec<p3_assets::lcr::Lcr>>) -> Result<(), String> {
    let mut kinds: Vec<u32> = figures.keys().copied().filter(|&k| sets::class_list(k).is_some()).collect();
    kinds.sort_unstable();
    view.run_local(|m, h| {
        for kind in kinds {
            let list = sets::class_list(kind).unwrap();
            for l in &figures[&kind] {
                let set = sets::append_set(m, h, list);
                p3_render::sprite_set::load_set(&mut SetEnv { m, h }, set, l, 0, 0);
                if kind == 0x17 {
                    let mut record = [0u8; 12];
                    for (k, w) in (1..4).map(|i| l.extra_header.get(i).copied().unwrap_or(0)).enumerate() {
                        record[4 * k..4 * k + 4].copy_from_slice(&w.to_le_bytes());
                    }
                    sets::push_projectile_record(m, h, record);
                }
            }
        }
    })
}

/// Clases de las figuras de la batalla del asedio (tabla de `0x00593D50`).
const FIGHTER_CLASSES: [u32; 9] = [8, 9, 0xA, 0xB, 0x15, 0x16, 0x20, 0x23, 0x24];
