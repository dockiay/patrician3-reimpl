//! J07: huellas de la multitud en la vista de ciudad.
//!
//! Cada paso libre de una figura (`0x55C820`) que cambia de fotograma ejecuta `0x55D0B0` y `0x55B030` (portes J07, paridad x86) mediante
//! [`p3_render::footprints::emit`]; las huellas que piden se crean aquí como efectos de la clase 18 (`0x57A150`, el mismo reparto de juegos que
//! el humo del puerto: códigos `0xC3`..`0xC6`, `0xA0`/`0xA1`) con el rumbo de la figura, el fundido `0x57AF40(1000, -18)` y la opacidad `0xB4`, y
//! avanzan con `0x57A8C0` (`p3_render::effect_step::step`). Con esos valores la huella dura unos diez segundos.
//!
//! Suposiciones (no deducidas del PE): el grado gráfico de la vista (`escena+0x504`) es `Town+0x9BC` (la opción `0x6CBDF0+0x190` se toma como
//! activa) y `escena+0xC594` vale 1; el `rand` del reparto de juegos es propio de las huellas (en el original es el del CRT, compartido).
use p3_assets::lcr::Lcr;
use p3_render::footprints::Footprint;
use p3_render::mover::SetInfo;

use crate::harbor_fx::{spawn_effect, step_effect, Effect};

/// Huellas vivas de una vista de ciudad.
pub struct TownFootprints {
    effects: Vec<(u32, Effect)>,
    next_id: u32,
    rng: u32,
}

impl Default for TownFootprints {
    fn default() -> Self {
        TownFootprints { effects: Vec::new(), next_id: 0, rng: 1 }
    }
}

impl TownFootprints {
    /// Crea los efectos de las huellas pedidas (`0x590610` + `0x57AF40` + opacidad de `0x55B030`).
    pub fn add(&mut self, prints: &[Footprint], sets: &[Lcr], infos: &[SetInfo], clock: u32) {
        for f in prints {
            let Some(mut e) = spawn_effect(sets, infos, &mut self.rng, clock, f.x as i32, f.y as i32, f.code, f.param as u8) else {
                continue;
            };
            // `0x57AF40(espera, paso)`: `+0x34` paso, `+0x3C` espera; con paso positivo la opacidad empezaría en 1.
            e.mem.w32(0x34, f.fade.1);
            e.mem.w32(0x3C, f.fade.0);
            if (f.fade.1 as i32) > 0 {
                e.mem.w8(0x30, 1);
            }
            e.mem.w8(0x30, f.alpha);
            self.next_id += 1;
            self.effects.push((self.next_id, e));
        }
    }

    /// Un fotograma de 33 ms de todas las huellas; las que se desvanecen del todo se retiran.
    pub fn step(&mut self, infos: &[SetInfo], clock: u32) {
        self.effects.retain_mut(|(id, e)| step_effect(e, infos, clock, *id));
    }

    pub fn effects(&self) -> impl Iterator<Item = &Effect> {
        self.effects.iter().map(|(_, e)| e)
    }

    pub fn len(&self) -> usize {
        self.effects.len()
    }

    pub fn is_empty(&self) -> bool {
        self.effects.is_empty()
    }
}
