//! Estado de animación de las figuras de la ciudad (`MoverAnimState`, `0x00557730`, `0x00557340`, `0x00557700`, `0x005576B0`).
//!
//! El estado guarda el momento del próximo cambio (`+4`), la animación actual (`+8`: 5 bits de id y 3 de sentido) y la siguiente (`+9`).
//! El conjunto de sprites (`.lcr`) aporta el intervalo entre fotogramas, las direcciones, los fotogramas de cada dirección y la tabla de
//! estados `{primero, último, id}`. El método virtual `+8` («animación terminada», que elige la siguiente) lo pone quien llama.

use p3_assets::lcr::{Lcr, State};

/// Los datos del conjunto de sprites que consulta la animación.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SetInfo {
    pub interval: u32,
    pub directions: u32,
    pub counts: Vec<u32>,
    pub states: Vec<State>,
}

impl SetInfo {
    pub fn from_lcr(l: &Lcr) -> Self {
        SetInfo { interval: l.interval_ms, directions: l.directions, counts: l.dirs.iter().map(|d| d.len() as u32).collect(), states: l.states.clone() }
    }
    /// Dirección que se muestra con el rumbo `heading` (`(h·N + 0x80) >> 8 & (N-1)`).
    pub fn direction_of(&self, heading: u8) -> usize {
        (((heading as u32).wrapping_mul(self.directions).wrapping_add(0x80) >> 8) & self.directions.wrapping_sub(1)) as usize
    }
    pub fn index_of(&self, id: u8) -> Option<usize> {
        self.states.iter().position(|s| s.id == (id & 0x1F) as u32)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct MoverAnim {
    pub next: u32,
    pub state: u8,
    pub target: u8,
}

impl MoverAnim {
    /// `0x005576B0`: fija la animación siguiente.
    pub fn set_target(&mut self, code: u8) {
        self.target = code;
    }

    /// `0x00557730`: empieza la animación `code`; devuelve su fotograma inicial (`0xFF` si el conjunto no la tiene).
    pub fn select(&mut self, set: &SetInfo, code: u8, clock: u32) -> u8 {
        let Some(i) = set.index_of(code) else { return 0xFF };
        self.state = code;
        let t = clock.wrapping_add(set.interval);
        if t < self.next {
            self.next = t;
        }
        let s = set.states[i];
        match code & 0xE0 {
            0 | 0xC0 => {
                self.target = code;
                s.first as u8
            }
            0x20 | 0xA0 => {
                self.target = code;
                s.last as u8
            }
            0x40 | 0x80 => {
                self.target = code.wrapping_add(0x20);
                s.first as u8
            }
            _ => {
                self.target = code.wrapping_sub(0x20);
                s.last as u8
            }
        }
    }

    /// `0x00557700`: tras `hook` (la animación terminó), empieza la animación siguiente o, si no existe, la actual.
    pub fn restart_default(&mut self, set: &SetInfo, clock: u32, hook: &mut dyn FnMut(&mut MoverAnim)) {
        hook(self);
        if self.select(set, self.target, clock) == 0xFF {
            self.select(set, self.state, clock);
        }
    }

    /// `0x00557340`: devuelve el fotograma que corresponde al reloj. `counter`, si lo hay, baja una unidad en cada cambio.
    pub fn advance(&mut self, set: &SetInfo, heading: u8, frame: u8, counter: Option<&mut u8>, clock: u32, hook: &mut dyn FnMut(&mut MoverAnim)) -> u8 {
        let dir = set.direction_of(heading);
        if clock < self.next {
            return frame;
        }
        if let Some(c) = counter {
            *c = c.wrapping_sub(1);
        }
        let counts = set.counts[dir];
        let st = set.states[set.index_of(self.state).expect("animación fuera de la tabla")];
        let mut f = frame;
        let finished = match self.state & 0xE0 {
            0 | 0x40 | 0x80 => {
                f = f.wrapping_add(1);
                !((f as u32) < counts && f < (st.last as u8).wrapping_add(1))
            }
            0x20 | 0x60 | 0xE0 => {
                f = f.wrapping_sub(1);
                !((f as u32) < counts && (st.first as u8).wrapping_sub(1) < f)
            }
            _ => true,
        };
        if finished {
            hook(self);
            let r = self.select(set, self.target, clock);
            f = if r != 0xFF { r } else { self.select(set, self.state, clock) };
        }
        if counts <= f as u32 {
            f = 0;
        }
        self.next = if self.state != 6 { set.interval.wrapping_add(clock) } else { (set.interval >> 1).wrapping_add(clock) };
        f
    }
}
