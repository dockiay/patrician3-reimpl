//! J05: barcos del puerto en la vista de ciudad. Reúne los barcos presentes con `harbor_scene_enter_town` (0x6141D0, paridad x86 en `p3-core`)
//! y los coloca como el original: los atracados (marca 0) en el último punto de su ruta de atraque (`.nodes`, clave 0).
//! Los que llegan o zarpan se colocan con el recorrido de tramos (0x617380, paridad x86); su sonido y la actualización de la vista no se reproducen. El nivel de daño (0x56E000)
//! se toma como 0; `DAT_006DFC1C = 0`, `DAT_006DFBC4 = 1000` y el indicador `*(0x6CBDF0 + 0x18C) = 0` son suposiciones.
use p3_assets::cpr::Archive;
use p3_core::ships::harbor_legs::place_on_route;
use p3_core::ships::harbor_list::*;

use crate::Session;

#[derive(Default)]
struct NoHost {
    /// Llamadas al recorrido 0x617380 en el orden en que `enter_town` las hizo; se aplican después con el recorrido portado.
    walks: Vec<WalkArgs>,
    /// Sonidos de llegada/salida que pidió el recorrido (0x59A090), en orden.
    sounds: Vec<HarborSound>,
}
impl SoundHost for NoHost {
    fn ship_sound(&mut self, x: i32, y: i32, z: i32) {
        self.sounds.push(HarborSound {
            sample_id: p3_audio::triggers::harbor_ship_sound_id(HARBOR_SHIP_SOUND_LIST, z as u32),
            x,
            y,
        });
    }
}

/// `ShipInOutSoundList` de `scripts/iso.ini` (`escena+0xCBA0`).
pub const HARBOR_SHIP_SOUND_LIST: u32 = 13000;

/// Disparador de sonido del puerto: ID de muestra (`13001` llegada, `13000` salida) y posición en pantalla.
/// Aún no se reproduce; `p3_audio::triggers` fija las reglas de ID.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HarborSound {
    pub sample_id: u32,
    pub x: i32,
    pub y: i32,
}
impl ListHost for NoHost {
    fn release_view_entity(&mut self, _: u32) {}
    fn clear_ship_flag(&mut self) {}
    fn refresh_lane_progress(&mut self, _: u32) {}
}
impl UpdateHost for NoHost {
    fn place_on_route(&mut self, a: WalkArgs) {
        self.walks.push(a);
    }
    fn update_view(&mut self, _: u16) {}
}

/// Un barco local ya colocado en el plano de la ciudad.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlacedShip {
    pub slot: u16,
    pub world: u16,
    /// Tipo de casco (`barco+0xE`): 0 snaikka, 1 crayer, 2 cog, 3 hulk.
    pub kind: u8,
    pub x: i32,
    /// Posición vertical en el lienzo (`+0xE * 22 / 34`, como `0x62018B`).
    pub y: i32,
    pub heading: u8,
    /// Etapa de vela (`+0x26`): 0 = recogidas.
    pub sails: u8,
    pub flag: u8,
    /// Nivel de daño del casco para el sprite (`schaden0N`): `3 - pct / 25` con `pct = vida * 100 / máximo` (menos 1 si no es 0, tope 99), como `0x56E000`.
    pub damage: u8,
    /// Balanceo en el muelle de `0x56D060` (`+0x38`/`+0x3C`, de -2..2 y -1..1) en el instante pedido.
    pub sway: (i32, i32),
    /// Posición 16.16 del barco local (`+8`, `+0xC`), propietario (`barco+0`) y vida máxima/actual (`barco+0x14`/`+0x18`), para `0x62018B`.
    pub raw: (u32, u32),
    pub owner: u8,
    pub health: (u32, u32),
    /// Se dibuja en la vista (atracado o recorriendo su ruta); los demás de la lista solo se actualizan.
    pub drawn: bool,
}

/// Lo que `0x62018B` lee del objeto de la lista (`barco+0x12C`, el global `0x6E6220` en la vista de ciudad) en desplazamientos pensados
/// para el objeto de batalla. En la lista del puerto esos bytes caen dentro de la tabla de puntos de entrada `+0x266` (palabras `x`, ocho
/// por ranura): el viento `+0x670`/`+0x671` es la palabra 517 (ranura 64, punto 5) y la tabla de propietarios `+0x35A` empieza en la
/// palabra 122 (ranura 15, punto 2). Ninguna otra función escribe ahí en la ciudad (los escritores de `+0x670` son la carga y el viento
/// de batalla, sobre los objetos de `0x6E55D0`), y el global empieza a cero; se reproduce tal cual.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ListAliases {
    pub wind_dir: u8,
    pub wind: u8,
    /// Palabra `+0x35A + 2 * lado` con el lado `+0x13D` = 0 que pone `construct_local_ship` en el puerto.
    pub owner0: u16,
}

impl ListAliases {
    pub fn of(list: &ShipList) -> Self {
        let word = |off: usize| list.entry_x.get((off - 0x266) / 2).copied().unwrap_or(0);
        let w = word(0x670);
        ListAliases { wind_dir: w as u8, wind: (w >> 8) as u8, owner0: word(0x35A) }
    }
}

/// Entorno de [`sway_at`]: solo el azar; los repintados, el minimapa y las celdas no afectan al desplazamiento.
struct SwayEnv(u32);
impl p3_render::harbor_sprite::SpriteEnv for SwayEnv {
    fn rand(&mut self) -> i32 {
        self.0 = self.0.wrapping_mul(214013).wrapping_add(2531011);
        ((self.0 >> 16) & 0x7FFF) as i32
    }
    fn damage(&mut self) {}
    fn refresh(&mut self) {}
    fn label_rect(&mut self) -> p3_render::harbor_sprite::Rect {
        [0; 4]
    }
    fn ring_rect(&mut self) -> p3_render::harbor_sprite::Rect {
        [0; 4]
    }
    fn unlink(&mut self, _: u8, _: i16, _: i16) {}
    fn link(&mut self, _: u8, _: i16, _: i16) {}
    fn repaint(&mut self, _: p3_render::harbor_sprite::Rect) {}
    fn map_pixel(&mut self, _: i16, _: i16) {}
    fn counter(&mut self) {}
    fn owner_word(&mut self, _: u8) -> u16 {
        1
    }
    fn minimap_visible(&mut self) -> bool {
        false
    }
    fn minimap_rect(&mut self) -> p3_render::harbor_sprite::Rect {
        [0; 4]
    }
    fn minimap_repaint(&mut self, _: p3_render::harbor_sprite::Rect) {}
    fn emit(&mut self, _: i32, _: i32) {}
}

/// Fotograma de la vista de ciudad en ms (el mismo paso de 33 ms que la multitud de J02).
const FRAME_MS: u32 = 33;

/// Desplazamiento del balanceo de `0x56D060` tras correr sus fotogramas desde la entrada en la ciudad hasta `clock`.
/// `state` es el estado del barco del mundo (`+0x134`): solo 0, 1, 6, 7, 8, 9 y 0xC se balancean. El azar es un `rand` de MSVC
/// propio de cada ranura (en el juego es el `rand` compartido, cuyo orden de consumo depende de toda la escena: suposición).
pub fn sway_at(state: u16, slot: u16, clock: u32) -> (i32, i32) {
    use p3_render::harbor_sprite::{update, SpriteCtx, SpriteState};
    let ctx = |clock| SpriteCtx {
        clock,
        owner: 0xFF,
        flags: 0,
        kind: state,
        player: 0,
        owner_count: 0,
        view_lift: 0,
        cam_x: 0,
        cam_y: 0,
        sheet_a: (0, 0),
        sheet_b: (0, 0),
        lift: 0,
        b2d58: 0,
        ndirs: 1,
        anchors: &[(0, 0)],
        pixel_div: 1,
        pixel_scale: 1.0,
    };
    let mut s = SpriteState::default();
    let mut env = SwayEnv(u32::from(slot).wrapping_mul(0x9E37) ^ 0x56D0);
    // Fotogramas t = 0, 33, 66… hasta `clock`. Con banderas, marca y contador a cero, un fotograma anterior a `+0x40` no cambia
    // nada, así que se salta directamente al primer fotograma en que vence.
    let mut t = 0u32;
    while t <= clock {
        update(&mut s, &ctx(t), &mut env);
        t = (t + FRAME_MS).max(s.next.div_ceil(FRAME_MS) * FRAME_MS);
    }
    (s.off_x, s.off_y)
}

/// Rutas de las ranuras de barco de la ciudad: sección 0 de su fichero `.nodes` (`escena+0xC670`).
pub fn slot_routes(archive: &Archive, file_id: u8) -> Vec<Vec<(u16, u16)>> {
    let Some(entry) = archive.find(format!("iso/towns/{file_id}.nodes").as_bytes()) else {
        return vec![];
    };
    let Some(data) = archive.read(entry) else { return vec![] };
    let Ok(sections) = p3_assets::town::parse_nodes(data) else {
        return vec![];
    };
    sections
        .iter()
        .find(|s| s.key == 0)
        .map(|s| {
            s.paths
                .iter()
                .map(|p| p.iter().map(|&(x, y)| (x as u16, y as u16)).collect())
                .collect()
        })
        .unwrap_or_default()
}

/// Nivel de daño de `0x56E000` (rama con daño visible) a partir de `max_health` (+0x14) y `current_health` (+0x18) del barco del mundo.
fn hull_level(ships: &[u8], id: usize) -> u8 {
    let rd = |o: usize| ships.get(id * 0x180 + o..id * 0x180 + o + 4).map_or(0, |v| i32::from_le_bytes([v[0], v[1], v[2], v[3]]));
    let (max, cur) = (rd(0x14), rd(0x18));
    if max == 0 {
        return 0;
    }
    let mut pct = cur.wrapping_mul(100).wrapping_div(max);
    if pct != 0 {
        pct -= 1;
    }
    (3 - pct.min(99) / 25).clamp(0, 3) as u8
}

fn u16_at(b: &[u8], o: usize) -> u16 {
    b.get(o..o + 2).map_or(0xFFFF, |v| u16::from_le_bytes([v[0], v[1]]))
}

impl Session {
    /// Barcos presentes en el puerto de la ciudad actual. `file_id` es el número del fichero de la ciudad (`iso/towns/<id>.nodes`).
    pub fn harbor_ships(&self, archive: &Archive, file_id: u8, clock: u32) -> Vec<PlacedShip> {
        self.harbor_scene(archive, file_id, clock).0
    }

    /// Todos los barcos de la lista del puerto (también los que no se dibujan), con los datos que lee `0x62018B`.
    pub fn harbor_list_ships(&self, archive: &Archive, file_id: u8, clock: u32) -> Vec<PlacedShip> {
        self.harbor_scene_all(archive, file_id, clock).0
    }

    /// Como `harbor_list_ships`, con los campos de la lista que lee `0x62018B` ([`ListAliases`]).
    pub fn harbor_list_state(&self, archive: &Archive, file_id: u8, clock: u32) -> (Vec<PlacedShip>, ListAliases) {
        let (ships, _, aliases) = self.harbor_scene_all(archive, file_id, clock);
        (ships, aliases)
    }

    /// Como `harbor_ships`, y además los disparadores de sonido que la escena pide al colocar los barcos.
    pub fn harbor_scene(&self, archive: &Archive, file_id: u8, clock: u32) -> (Vec<PlacedShip>, Vec<HarborSound>) {
        let (mut all, sounds, _) = self.harbor_scene_all(archive, file_id, clock);
        all.retain(|s| s.drawn);
        (all, sounds)
    }

    fn harbor_scene_all(&self, archive: &Archive, file_id: u8, clock: u32) -> (Vec<PlacedShip>, Vec<HarborSound>, ListAliases) {
        let Some(nav) = self.sim.host.navigation.as_ref() else {
            return (vec![], vec![], ListAliases::default());
        };
        let snap = &nav.snapshot;
        let routes = slot_routes(archive, file_id);
        if routes.is_empty() || snap.manager.len() < 0xF8 {
            return (vec![], vec![], ListAliases::default());
        }
        let town = &self.sim.economy.world.towns[self.city];
        let merchants = &self.sim.economy.world.merchants;
        let kinds: Vec<u16> = merchants.iter().map(|m| m.kind).collect();
        let kind_of = move |m: u32| kinds.get(m as usize).copied().unwrap_or(0);
        let world = HarborWorld {
            ships: &snap.ships,
            convoys: &snap.convoys,
            ship_count: u16_at(&snap.manager, 0xF4),
            convoy_count: u16_at(&snap.manager, 0xF6),
            player: self.owner as u32,
            hotseat: 0,
            merchant_count: merchants.len() as u16,
            strict_convoy_flag: 0,
            merchant_word: &kind_of,
        };
        let mut list = ShipList {
            address: 0x5010_0000,
            town: 0xFFFE,
            limit: routes.len() as u32,
            capacity: 0,
            head: 0xFFFF,
            free: 0xFFFF,
            town_ships: vec![0xFFFF; 0x80 * 255 + 1024],
            ..Default::default()
        };
        let scene = SceneRoutes {
            routes: &routes,
            shown_town: self.city as u32,
        };
        let enter = EnterTown {
            first_ship: u16_at(&snap.manager, 0xE8),
            first_convoy: u16_at(&snap.manager, 0xEC),
            town_ship_head: town.u16(0x81E),
            town_convoy_head: town.u16(0x820),
        };
        let clock = SyncClock {
            now: clock,
            smoothing_token: 0,
            divisor: 1000,
        };
        let mut host = NoHost::default();
        if enter_town(
            &mut list,
            &world,
            &scene,
            &mut host,
            &clock,
            0,
            &enter,
            self.city as u16,
            1,
        )
        .is_err()
        {
            return (vec![], vec![], ListAliases::default());
        }
        // Barcos que llegan o zarpan: el recorrido de tramos (0x617380, con paridad x86) los coloca sobre su ruta de entrada o salida.
        let mut moving = std::collections::BTreeSet::new();
        for w in std::mem::take(&mut host.walks) {
            let base = usize::from(w.slot) * 8;
            let (Some(xs), Some(ys), Some(fl)) = (
                list.entry_x.get(base..base + 8),
                list.entry_y.get(base..base + 8),
                list.entry_flags.get(base..base + 8),
            ) else {
                continue;
            };
            let (xs, ys, fl) = (xs.to_vec(), ys.to_vec(), fl.to_vec());
            place_on_route(
                &mut list,
                &mut host,
                w.slot,
                &xs,
                &ys,
                &fl,
                w.len,
                w.distance,
                w.reverse != 0,
            );
            moving.insert(usize::from(w.slot));
        }
        let mut out = Vec::new();
        let mut cur = list.head;
        while cur != 0xFFFF {
            let s = usize::from(cur);
            if let Some(o) = list.ships.get(s).and_then(Option::as_ref) {
                let id = u16_at(o, 4);
                let flag = o[0x12A];
                let drawn = flag == 0 || moving.contains(&s);
                let rd = |off: usize| snap.ships.get(usize::from(id) * 0x180 + off..usize::from(id) * 0x180 + off + 4).map_or(0, |v| u32::from_le_bytes([v[0], v[1], v[2], v[3]]));
                let r32 = |off: usize| u32::from_le_bytes([o[off], o[off + 1], o[off + 2], o[off + 3]]);
                {
                    let y = i32::from(u16_at(o, 0xE) as i16) * 22 / 34;
                    out.push(PlacedShip {
                        slot: cur,
                        world: id,
                        kind: snap.ships.get(usize::from(id) * 0x180 + 0xE).copied().unwrap_or(0),
                        x: i32::from(u16_at(o, 0xA) as i16),
                        y,
                        heading: o[0x11],
                        sails: o[0x26],
                        flag,
                        damage: hull_level(&snap.ships, usize::from(id)),
                        sway: if flag == 0 { sway_at(u16_at(&snap.ships, usize::from(id) * 0x180 + 0x134), cur, clock.now) } else { (0, 0) },
                        raw: (r32(8), r32(0xC)),
                        owner: snap.ships.get(usize::from(id) * 0x180).copied().unwrap_or(0),
                        health: (rd(0x14), rd(0x18)),
                        drawn,
                    });
                }
            }
            cur = list.link12.get(s).copied().unwrap_or(0xFFFF);
        }
        (out, host.sounds, ListAliases::of(&list))
    }
}

impl Session {
    /// Dibuja los barcos atracados con su conjunto `.lcr` (clase 13: tipo, vela y daño en la cabecera extra). Devuelve cuántos se dibujaron.
    pub(crate) fn draw_harbor_ships(
        &self,
        archive: &Archive,
        scene: &mut p3_render::Scene,
        file_id: u8,
        rgba: &mut [u8],
        w: usize,
        h: usize,
        clock: u32,
    ) -> usize {
        let (all, aliases) = self.harbor_list_state(archive, file_id, clock);
        let mut ships: Vec<PlacedShip> = all.iter().filter(|s| s.drawn).cloned().collect();
        ships.sort_by_key(|s| (s.y, s.x));
        let figures = scene.load_figures();
        let Some(sets) = figures.get(&13) else { return 0 };
        let mut drawn = 0;
        let set_of = |s: &PlacedShip| {
            sets.iter().find(|l| {
                l.extra_header.get(1) == Some(&u32::from(s.kind))
                    && l.extra_header.get(2) == Some(&u32::from(s.sails))
                    && l.extra_header.get(4) == Some(&u32::from(s.damage))
            })
        };
        // Texturas de los barcos que se van a dibujar, decodificadas de una vez en paralelo.
        let wanted: Vec<_> = ships.iter().filter_map(|s| Some((set_of(s)?, s.heading, 0u8))).collect();
        scene.preload_lcr_frames(&wanted);
        for s in &ships {
            let Some(set) = set_of(s) else {
                continue;
            };
            if scene.draw_lcr_frame(rgba, w, h, set, s.heading, 0, s.x + s.sway.0, s.y + 21 + s.sway.1) {
                drawn += 1;
            }
        }
        // Humo de daño y hundimiento (`0x62018B` + efectos tipo 18), encima de los barcos.
        let effects = figures.get(&18).map(Vec::as_slice).unwrap_or(&[]);
        let lists = audio_lists(archive);
        let mut fx = self.harbor_fx.borrow_mut();
        fx.advance(file_id, clock, &all, aliases, self.owner as u32, effects, &|l| lists.get(&l).copied().unwrap_or(0));
        for e in fx.effects() {
            if let Some(set) = effects.get(e.set) {
                scene.draw_lcr_frame_faded(rgba, w, h, set, 0, e.frame(), e.x(), e.y() + 21, e.alpha());
            }
        }
        drawn
    }
}

#[cfg(test)]
mod sway_tests {
    use super::sway_at;

    #[test]
    fn docked_ships_sway_within_original_bounds() {
        let mut moved = 0;
        for slot in 0..20u16 {
            for clock in [0, 1_000, 4_000, 20_000, 120_000] {
                let (x, y) = sway_at(0, slot, clock);
                assert!((-2..=2).contains(&x) && (-1..=1).contains(&y));
                moved += usize::from((x, y) != (0, 0));
            }
            // Un estado fuera de 0, 1, 6..9, 0xC no se balancea.
            assert_eq!(sway_at(2, slot, 60_000), (0, 0));
        }
        assert!(moved > 20, "solo {moved} posiciones movidas");
        // Determinista: la misma ranura y el mismo instante dan el mismo desplazamiento.
        assert_eq!(sway_at(0, 3, 50_000), sway_at(0, 3, 50_000));
    }
}

/// `[SoundListN] Count=` de `scripts/sound.ini` (para la lista 60100 de `0x62018B`).
fn audio_lists(archive: &Archive) -> std::collections::BTreeMap<u32, u32> {
    crate::audio::SoundCatalog::from_archive(archive).map(|c| c.lists).unwrap_or_default()
}
