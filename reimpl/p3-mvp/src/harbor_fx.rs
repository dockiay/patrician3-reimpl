//! J05: humo, hundimiento y sonido de los barcos del puerto en la vista de ciudad.
//!
//! Cada fotograma de 33 ms (el de la multitud de J02) llama a `p3_core::ships::harbor_view::update_view` (`0x62018B`, paridad x86) por
//! cada barco de la lista del puerto, y avanza los efectos tipo 18 que crea con `p3_render::effect_step::step` (`0x57A8C0`, paridad x86).
//! Los efectos se crean como `0x57A150`: el juego se elige por muestreo de depósito entre los `.lcr` de la clase 18 cuyo código
//! (`cabecera extra[1]`) coincide con `variante & 0xFF`; el humo de daño (código `0x24`, `rauchanimpage`) baja `0x30` píxeles; la animación
//! es la 2 (`0x557730`) y el efecto se retira al terminarla (`+0x28 == 2`, fotograma que vuelve atrás).
//!
//! Viento y propietario: `0x62018B` lee `+0x670`/`+0x671` y la tabla `+0x35A` del objeto de la lista, que en la ciudad es el global
//! `0x6E6220`; esos bytes son puntos de la tabla de entrada (ver [`crate::harbor::ListAliases`]) y se toman de ahí. Con los puertos del
//! juego el viento queda a 0 (la palabra 517 nunca se escribe), así que el humo no deriva, como en el original.
//!
//! Suposiciones (no deducidas del PE): la opción `escena+0xC594` está activa (el constructor `0x58760E` la pone a 1); la lista se
//! reconstruye para cada ciudad, sin los restos de la anterior que el global conservaría; en los códigos con subtipo (hundimiento,
//! `variante >> 8` contra `juego+4`) se toma el juego `subtipo - 1` de los que tienen ese código, en el orden de `files.lst`; los anillos
//! de píxeles (`0x5986A0`) no se dibujan. El estado vive en la sesión y empieza 5 s antes del primer instante pedido; si el reloj
//! retrocede o cambia la ciudad, se reinicia.
use std::collections::BTreeMap;

use p3_assets::lcr::Lcr;
use p3_core::ships::harbor_view::{update_view, LocalShipView, ViewCtx, ViewHost, SHIP_SOUND_LIST};
use p3_render::ambience::SceneMem;
use p3_render::clouds::Rect;
use p3_render::effect_step::{step, EffectCtx, EffectStepEnv};
use p3_render::mover::{MoverAnim, SetInfo};

use crate::harbor::{HarborSound, ListAliases, PlacedShip};

/// Paso de fotograma del puerto (ms).
pub const FRAME_MS: u32 = 33;
/// `DAT_00677929`: último argumento de los emisores.
const EMITTER_ARG: u8 = 32;

/// Un efecto tipo 18 vivo.
pub struct Effect {
    pub mem: SceneMem,
    pub set: usize,
    pub code: u8,
    anim: MoverAnim,
}

impl Effect {
    pub fn x(&self) -> i32 {
        i32::from(self.mem.r16(0xE) as i16)
    }
    pub fn y(&self) -> i32 {
        i32::from(self.mem.r16(0x12) as i16)
    }
    pub fn frame(&self) -> u8 {
        self.mem.r8(0x15)
    }
    pub fn alpha(&self) -> u8 {
        self.mem.r8(0x30)
    }
}

enum Entity {
    /// Entidad de vista del barco (clase 0xD): la dibuja `draw_harbor_ships`.
    View,
    Effect(Effect),
}

/// Estado de los efectos del puerto de una ciudad.
#[derive(Default)]
pub struct HarborFx {
    town: Option<u8>,
    clock: Option<u32>,
    views: BTreeMap<u16, LocalShipView>,
    entities: BTreeMap<u32, Entity>,
    next_id: u32,
    lock: u8,
    rng: u32,
    /// Sonidos del barco ajeno (`0x595B00`) pedidos desde la última consulta: muestra y posición en el lienzo.
    pub sounds: Vec<HarborSound>,
}

struct Host<'a> {
    fx: &'a mut HarborFx,
    sets: &'a [Lcr],
    infos: &'a [SetInfo],
    clock: u32,
    sound_lists: &'a dyn Fn(u32) -> u32,
}

pub(crate) fn rand(rng: &mut u32) -> i32 {
    *rng = rng.wrapping_mul(214013).wrapping_add(2531011);
    ((*rng >> 16) & 0x7FFF) as i32
}

/// `0x57A150` con `escena+0xC594` puesta: elige por muestreo de depósito el juego de la clase 18 cuyo código es `variante & 0xFF` (o el
/// `subtipo - 1` en los códigos con subtipo) y coloca el efecto en `(x, y)` (16.16) con el rumbo `heading` y la animación 2.
#[allow(clippy::too_many_arguments)]
pub fn spawn_effect(sets: &[Lcr], infos: &[SetInfo], rng: &mut u32, clock: u32, x: i32, y: i32, variant: u32, heading: u8) -> Option<Effect> {
    let code = variant as u8;
    let subtype = matches!(code, 4..=7 | 0x40..=0x43 | 0x50 | 0x80 | 0xB0 | 199).then_some((variant >> 8) & 0xFF).filter(|&k| k != 0);
    let matching: Vec<usize> = (0..sets.len()).filter(|&i| sets[i].extra_header.get(1) == Some(&u32::from(code))).collect();
    let set = match subtype {
        Some(k) => *matching.get(k as usize - 1)?,
        None => {
            let mut pick = None;
            for (n, &i) in matching.iter().enumerate() {
                if rand(rng) % (n as i32 + 1) == 0 {
                    pick = Some(i);
                }
            }
            pick?
        }
    };
    let mut mem = SceneMem::sized(0x60);
    let mut anim = MoverAnim::default();
    let first = anim.select(&infos[set], 2, clock);
    mem.w8(0x15, first);
    mem.w8(0x28, 2);
    mem.w32(0xC, x as u32);
    let y = if matches!(code, 0x24 | 0x30 | 0x11 | 0xB0) { (y as u32).wrapping_add(0x30_0000) } else if code == 0x81 { (y as u32).wrapping_add(0xB_0000) } else { y as u32 };
    mem.w32(0x10, y);
    mem.w8(0x14, heading);
    mem.w8(0x30, 0xFF);
    mem.w8(0x31, 0xFF);
    Some(Effect { mem, set, code, anim })
}

/// `0x57A8C0` sobre un efecto de la clase 18 (`infos` de los mismos juegos): devuelve `false` si el efecto debe retirarse.
pub fn step_effect(f: &mut Effect, infos: &[SetInfo], clock: u32, id: u32) -> bool {
    let ctx = EffectCtx { budget: FRAME_MS, time: clock, kind: f.code, no_flag: true, c594_zero: false, harbor_ok: false, lists: [vec![], vec![], vec![]], id };
    let frame = f.frame();
    let mut env = StepEnv { anim: &mut f.anim, set: &infos[f.set], frame, clock };
    step(&mut f.mem, &ctx, &mut env)
}

impl Host<'_> {
    fn spawn(&mut self, x: i32, y: i32, variant: u32) -> Option<Effect> {
        spawn_effect(self.sets, self.infos, &mut self.fx.rng, self.clock, x, y, variant, EMITTER_ARG)
    }

    fn effect(&mut self, e: u32) -> Option<&mut Effect> {
        match self.fx.entities.get_mut(&e) {
            Some(Entity::Effect(f)) => Some(f),
            _ => None,
        }
    }
}

impl ViewHost for Host<'_> {
    fn rand(&mut self) -> i32 {
        rand(&mut self.fx.rng)
    }
    fn create(&mut self, x: i32, y: i32, class: u32, code: u32, _arg: u8) -> u32 {
        let entity = if class == 0x12 {
            match self.spawn(x, y, code) {
                Some(f) => Entity::Effect(f),
                None => return 0,
            }
        } else {
            Entity::View
        };
        self.fx.next_id += 1;
        self.fx.entities.insert(self.fx.next_id, entity);
        self.fx.next_id
    }
    fn set_alpha(&mut self, e: u32, v: u8) {
        if let Some(f) = self.effect(e) {
            f.mem.w8(0x30, v);
            f.mem.w8(0x31, v);
        }
    }
    fn set_drift(&mut self, e: u32, dx: i32, dy: i32) {
        if let Some(f) = self.effect(e) {
            f.mem.w32(0x40, dx as u32);
            f.mem.w32(0x44, dy as u32);
        }
    }
    fn frame(&mut self, e: u32) -> u8 {
        self.effect(e).map_or(0, |f| f.frame())
    }
    fn entity_heading(&mut self, _: u32) -> u8 {
        0
    }
    fn emit(&mut self, e: u32, x: i32, y: i32) {
        if let Some(f) = self.effect(e) {
            f.mem.w32(0xC, (x as u32) << 16);
            f.mem.w32(0x10, (y as u32) << 16);
        }
    }
    fn release(&mut self, e: u32) {
        self.fx.entities.remove(&e);
    }
    fn pose(&mut self, _: u32, _: i16, _: i32, _: u8, _: u8) {}
    fn sound(&mut self, x: i32, y: i32, sample: i32) -> u8 {
        self.fx.sounds.push(HarborSound { sample_id: sample as u32 & 0xFFFF, x, y });
        1
    }
    fn list_count(&mut self, list: u32) -> i32 {
        (self.sound_lists)(list) as i32
    }
    fn pixel(&mut self, _: i16, _: i16) {}
}

struct StepEnv<'a> {
    anim: &'a mut MoverAnim,
    set: &'a SetInfo,
    frame: u8,
    clock: u32,
}

impl EffectStepEnv for StepEnv<'_> {
    fn advance(&mut self) -> u8 {
        self.anim.advance(self.set, 0, self.frame, None, self.clock, &mut |_| {})
    }
    fn f613070(&mut self) -> u32 {
        0
    }
    fn f561f60(&mut self) {}
    fn toggle(&mut self, _: bool) {}
    fn occupy_remove(&mut self, _: &SceneMem) -> Rect {
        Rect { left: 0, top: 0, right: 0, bottom: 0 }
    }
    fn occupy_add(&mut self, _: Rect) {}
    fn measure(&mut self, _: &SceneMem) -> Rect {
        Rect { left: 0, top: 0, right: 0, bottom: 0 }
    }
    fn dirty(&mut self, _: Rect) {}
}

impl HarborFx {
    /// Avanza hasta `clock` con los barcos de la lista. `sets` son los `.lcr` de la clase 18; `sound_lists(lista)` da el número de muestras.
    pub fn advance(&mut self, town: u8, clock: u32, ships: &[PlacedShip], list: ListAliases, player: u32, sets: &[Lcr], sound_lists: &dyn Fn(u32) -> u32) {
        if self.town != Some(town) || self.clock.is_some_and(|c| c > clock) {
            *self = HarborFx { town: Some(town), rng: 1, ..Default::default() };
        }
        let infos: Vec<SetInfo> = sets.iter().map(SetInfo::from_lcr).collect();
        let mut t = self.clock.map_or(clock.saturating_sub(5000), |c| c + FRAME_MS);
        while t <= clock {
            self.frame(t, ships, list, player, sets, &infos, sound_lists);
            self.clock = Some(t);
            t += FRAME_MS;
        }
        if self.clock.is_none() {
            self.clock = Some(clock);
        }
    }

    fn frame(&mut self, clock: u32, ships: &[PlacedShip], list: ListAliases, player: u32, sets: &[Lcr], infos: &[SetInfo], sound_lists: &dyn Fn(u32) -> u32) {
        let mut host = Host { fx: self, sets, infos, clock, sound_lists };
        for s in ships {
            let mut o = host.fx.views.remove(&s.slot).filter(|v| v.world == u32::from(s.world)).unwrap_or(LocalShipView { world: u32::from(s.world), ..Default::default() });
            o.x = s.raw.0;
            o.y = s.raw.1;
            o.heading = s.heading;
            o.sails = s.sails;
            o.flag = s.flag;
            o.owner_slot = 0;
            let owners = [list.owner0];
            let ctx = ViewCtx {
                clock,
                player,
                hull: s.kind,
                max_health: s.health.0,
                health: s.health.1,
                owners: &owners,
                wind_dir: list.wind_dir,
                wind: list.wind,
                // El original divide por la vida máxima: con 0 no hay humo (allí sería una excepción).
                smoke_option: s.health.0 != 0,
                emitter_arg: EMITTER_ARG,
            };
            let mut lock = host.fx.lock;
            update_view(&mut o, &ctx, &mut lock, &mut host);
            host.fx.lock = lock;
            host.fx.views.insert(s.slot, o);
        }
        // Barcos que ya no están en la lista: sus entidades se dan de baja.
        let live: Vec<u16> = ships.iter().map(|s| s.slot).collect();
        let gone: Vec<u16> = self.views.keys().copied().filter(|k| !live.contains(k)).collect();
        for k in gone {
            if let Some(v) = self.views.remove(&k) {
                for e in [v.entity, v.smoke, v.flames] {
                    self.entities.remove(&e);
                }
            }
        }
        // Paso de los efectos (`0x57A8C0`); los emisores del hundimiento los mueve su barco.
        let emitters: Vec<u32> = self.views.values().flat_map(|v| [v.smoke, v.flames]).collect();
        let mut dead = vec![];
        for (&id, e) in self.entities.iter_mut() {
            let Entity::Effect(f) = e else { continue };
            if !step_effect(f, infos, clock, id) && !emitters.contains(&id) {
                dead.push(id);
            }
        }
        for id in dead {
            self.entities.remove(&id);
        }
    }

    /// Efectos vivos, en orden de creación.
    pub fn effects(&self) -> impl Iterator<Item = &Effect> {
        self.entities.values().filter_map(|e| match e {
            Entity::Effect(f) => Some(f),
            Entity::View => None,
        })
    }
}

/// Muestras de la lista del barco ajeno (`ShipSoundList`).
pub fn is_ship_sound(id: u32) -> bool {
    (SHIP_SOUND_LIST..SHIP_SOUND_LIST + 100).contains(&id)
}
