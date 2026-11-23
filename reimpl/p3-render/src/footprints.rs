//! Huellas de las figuras de la multitud en la vista de ciudad: el paso libre (`0x55C820`) llama a `0x55D0B0` cuando cambia el
//! fotograma, y este a `0x55B030`, que crea el efecto tipo `0x12` con `0x590610` (código `0xC3`/`0xC4`, `0xC5`/`0xC6` en la clase 3,
//! `0xA0`/`0xA1` en carro), le pone el fundido `0x57AF40(1000, -18)` y la opacidad `0xB4`.
//!
//! Las dos funciones son los portes de J07 (`j07::mover`, paridad x86) y se ejecutan sobre una memoria mínima con la disposición del
//! original: la escena (`0x6E51AC`) con el grado gráfico de la ciudad (`+0x504`, que `0x58B7F0` copia de `Town+0x9BC` mediante
//! `0x591EC0`), la opción `+0xC594`, la vista (`+0xC650`/`+0xC654`, `+0x2C`/`+0x30`) y la capa de suelo (`+0xC308`, paso `+0xC314`);
//! la figura con sus `0x70` bytes y el conjunto (`+8` clase, `+0x28`/`+0x2C` estados, como `0x558770`). `0x5576C0` también es el porte de
//! J07; `0x58D030` es [`Geom::pixel_to_cell`] y la clase de la ficha (virtual `+0x50`) es la de la capa 0 de la escena. La creación del
//! efecto (`0x590610`) y el fundido se devuelven como [`Footprint`] para que el cliente cree el efecto.

use crate::geom::Geom;
use crate::j07::{self, Host, Mem};
use crate::mover::SetInfo;
use crate::walk::{FreeWalker, WalkScene};
use std::collections::HashMap;

/// Condiciones de la vista que miran `0x55D0B0` y `0x55B030`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FootprintView {
    /// `escena+0x504`: grado gráfico de la ciudad (`Town+0x9BC`, o 0 si la opción `0x6CBDF0+0x190` es menor que 1). Hay huellas con 2..=5.
    pub grade: u32,
    /// `escena+0xC594`.
    pub detail: u8,
    /// Ventana de la vista: origen (`+0xC650`, `+0xC654`) y tamaño (`+0x2C`, `+0x30`) en píxeles del plano. Solo dejan huella las figuras a
    /// menos de 200 píxeles de ella.
    pub view: (u32, u32, u32, u32),
}

/// Un efecto de huella pedido a `0x590610(escena, x, y, 0x12, código, parámetro, 0, 0)`, con el fundido y la opacidad que le pone `0x55B030`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Footprint {
    /// Posición 16.16.
    pub x: u32,
    pub y: u32,
    /// Código del juego de efectos (`0xC3`, `0xC4`...).
    pub code: u32,
    /// Parámetro (el rumbo en el byte bajo; los bits altos son los restos del registro del original).
    pub param: u32,
    /// `0x57AF40(espera, paso)`.
    pub fade: (u32, u32),
    /// Byte `+0x30` tras la creación.
    pub alpha: u8,
}

const UI: u32 = 0x006E_51AC;
const SCENE: u32 = 0x1000_0000;
const WALKER: u32 = 0x2000_0000;
const SET: u32 = 0x3000_0000;
const STATES: u32 = 0x3100_0000;
const GRID: u32 = 0x4000_0000;
const OBJS: u32 = 0x5000_0000;
const OBJ: u32 = 0x5100_0000;
const VTABLE: u32 = 0x5200_0000;
const TILE_KIND_FN: u32 = 0x5300_0000;
const EFFECT: u32 = 0x6000_0000;
const STACK: u32 = 0x7000_0000;
const SPAWN_TOWN_ENTITY: u32 = 0x0059_0610;
const EFFECT_SET_FADE: u32 = 0x0057_AF40;

#[derive(Default)]
struct Flat(HashMap<u32, u8>);

impl Mem for Flat {
    fn r8(&mut self, a: u32) -> u8 {
        self.0.get(&a).copied().unwrap_or(0)
    }
    fn w8(&mut self, a: u32, v: u8) {
        self.0.insert(a, v);
    }
}

struct FootHost<'a> {
    scene: &'a dyn WalkScene,
    geom: &'a Geom,
    mask: &'a [u8],
    tile_kind: u32,
    out: Vec<Footprint>,
}

impl Host for FootHost<'_> {
    fn call(&mut self, m: &mut dyn Mem, target: u32, ecx: u32, a: &[u32]) -> u32 {
        match target {
            0x0055_76C0 | 0x0055_B030 => match j07::run(target, m, self, ecx, a, STACK) {
                Some(j07::Ret::U8(v)) => u32::from(v),
                Some(j07::Ret::U32(v)) => v,
                _ => 0,
            },
            j07::entity::PIXEL_TO_CELL => {
                let ((c, r), _) = self.geom.pixel_to_cell(a[0] as i32, a[1] as i32, self.mask);
                m.w32(a[2], c);
                m.w32(a[2] + 4, r);
                // Una sola ficha (la 1) en la casilla pedida; su clase es la de la capa 0 de la escena.
                let cols = m.r32(SCENE + 0xC314);
                m.w16(GRID.wrapping_add(cols.wrapping_mul(r).wrapping_add(c).wrapping_mul(4)), 1);
                self.tile_kind = self.scene.kind0(c, r);
                0
            }
            TILE_KIND_FN => self.tile_kind,
            SPAWN_TOWN_ENTITY => {
                self.out.push(Footprint { x: a[0], y: a[1], code: a[3], param: a[4], fade: (0, 0), alpha: 0 });
                EFFECT
            }
            EFFECT_SET_FADE => {
                if let Some(f) = self.out.last_mut() {
                    f.fade = (a[0], a[1]);
                }
                0
            }
            _ => 0,
        }
    }
}

/// `0x55D0B0(figura, conjunto, 0, 0, 0)` del paso libre: huellas que deja la figura `w` (del conjunto `set`, clase `class`).
pub fn emit(
    w: &FreeWalker,
    class: u32,
    set: &SetInfo,
    fv: FootprintView,
    scene: &dyn WalkScene,
    geom: &Geom,
    mask: &[u8],
) -> Vec<Footprint> {
    if !(2..=5).contains(&fv.grade) || fv.detail == 0 {
        return Vec::new();
    }
    let mut m = Flat::default();
    m.w32(UI, SCENE);
    m.w32(SCENE + 0x504, fv.grade);
    m.w8(SCENE + 0xC594, fv.detail);
    let view = fv.view;
    m.w32(SCENE + 0xC650, view.0);
    m.w32(SCENE + 0xC654, view.1);
    m.w32(SCENE + 0x2C, view.2);
    m.w32(SCENE + 0x30, view.3);
    m.w32(SCENE + 0xC314, geom.cols);
    m.w32(SCENE + 0xC308, GRID);
    m.w32(SCENE + 0x98, OBJS);
    m.w32(OBJS + 4, OBJ);
    m.w32(OBJ, VTABLE);
    m.w32(VTABLE + 0x50, TILE_KIND_FN);
    for (i, b) in w.to_bytes().iter().enumerate() {
        m.w8(WALKER + i as u32, *b);
    }
    m.w32(SET + 8, class);
    m.w32(SET + 0x28, set.states.len() as u32);
    m.w32(SET + 0x2C, STATES);
    for (i, s) in set.states.iter().enumerate() {
        let a = STATES + i as u32 * 12;
        m.w32(a, s.first);
        m.w32(a + 4, s.last);
        m.w32(a + 8, s.id);
    }
    let mut h = FootHost { scene, geom, mask, tile_kind: 0, out: Vec::new() };
    j07::run(0x0055_D0B0, &mut m, &mut h, WALKER, &[SET, 0, 0, 0], STACK);
    let alpha = m.r8(EFFECT + 0x30);
    for f in &mut h.out {
        f.alpha = alpha;
    }
    h.out
}

#[cfg(test)]
mod tests {
    use super::*;
    use p3_assets::lcr::State;

    struct Ground(u32);
    impl WalkScene for Ground {
        fn size(&self) -> (u32, u32) {
            (64, 128)
        }
        fn kind0(&self, _: u32, _: u32) -> u32 {
            self.0
        }
        fn kind1(&self, _: u32, _: u32) -> Option<u32> {
            None
        }
        fn street(&self, _: u32, _: u32) -> bool {
            true
        }
        fn entities(&self, _: u32, _: u32) -> Vec<(u16, u32)> {
            Vec::new()
        }
    }

    fn walker(frame: u8, state: u8) -> FreeWalker {
        let mut w = FreeWalker::from_bytes(&[0u8; 0x70]);
        w.x = 300 << 16;
        w.y = 200 << 16;
        w.px = 300;
        w.py = 200;
        w.heading = 0x40;
        w.frame = frame;
        w.anim.state = state;
        w
    }

    #[test]
    fn walking_frames_leave_left_then_right_print_off_paved_ground() {
        let geom = Geom { cols: 64, rows: 128, cell_w: 34, cell_h: 21, off_x: 17, off_y: 10 };
        let mask = crate::geom::detect_mask();
        let set = SetInfo { interval: 100, directions: 8, counts: vec![10; 8], states: vec![State { first: 0, last: 9, id: 2 }] };
        let fv = FootprintView { grade: 3, detail: 1, view: (0, 0, 2000, 2000) };
        let snow = Ground(0);
        let left = emit(&walker(4, 2), 0, &set, fv, &snow, &geom, &mask);
        assert_eq!(left.len(), 1);
        assert_eq!((left[0].code, left[0].x, left[0].y, left[0].param as u8), (0xC3, 300 << 16, 200 << 16, 0x40));
        assert_eq!((left[0].fade, left[0].alpha), ((1000, (-18i32) as u32), 0xB4));
        assert_eq!(emit(&walker(9, 2), 0, &set, fv, &snow, &geom, &mask)[0].code & 0xFF, 0xC4);
        assert!(emit(&walker(5, 2), 0, &set, fv, &snow, &geom, &mask).is_empty());
        // Clase 3: `0xC5`/`0xC6`.
        assert_eq!(emit(&walker(4, 2), 3, &set, fv, &snow, &geom, &mask)[0].code & 0xFF, 0xC5);
        // Calle (`0x10`, `0x17`, `0x18`): ninguna huella.
        for k in [0x10, 0x17, 0x18] {
            assert!(emit(&walker(4, 2), 0, &set, fv, &Ground(k), &geom, &mask).is_empty(), "{k:#x}");
        }
        assert_eq!(emit(&walker(4, 2), 0, &set, fv, &Ground(0x16), &geom, &mask).len(), 1);
        // Fuera de los grados 2..=5 o sin la opción, nada.
        for g in [0, 1, 6] {
            assert!(emit(&walker(4, 2), 0, &set, FootprintView { grade: g, ..fv }, &snow, &geom, &mask).is_empty());
        }
        // Lejos de la ventana (más de 200 píxeles), nada.
        let far = FootprintView { view: (600, 0, 400, 400), ..fv };
        assert!(emit(&walker(4, 2), 0, &set, far, &snow, &geom, &mask).is_empty());
    }
}
