//! Animadores de la escena de ciudad (J02). Puerto de las cinco clases de `SpriteAnimState` del original.
//!
//! Estado de 0x14 bytes con el reloj del fotograma `[0x6DCCF8]` en milisegundos:
//! `+4` siguiente cambio, `+8` fotograma, `+9` estado (0 parado, 1 reproduciendo, 2 terminado, 3 inicio; bit 7 = hacia atrás),
//! `+0xC` índice de animación (6 bits bajos) y marcas (bit 7 = en espera, bit 6 = dibujo pendiente), `+0x10` grupo y `+0x11` repeticiones.
//!
//! Cada animación procede de un trozo `.let` con etiqueta >= 3; el número de valores de su `array` fija la clase del animador
//! (`0x552EF0`): 2 = simple (`0x551860`), 4 = reposo (`0x54EBD0`), 5 = azar (`0x552200`), 7 = agrupado (`0x54EE30`),
//! 9 = agrupado con repeticiones (`0x552500`).

/// Constantes de `0x673D98..0x673D9B`.
pub const ST_HOLD: u8 = 0;
pub const ST_PLAY: u8 = 1;
pub const ST_DONE: u8 = 2;
pub const ST_START: u8 = 3;

/// Datos de una animación: los valores del `array` del trozo y el número de fotogramas.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Anim {
    /// `array[0]`: milisegundos por fotograma.
    pub duration: u32,
    /// `array[1]`: si termina en el último fotograma esperando (y no en «terminado»).
    pub looping: bool,
    /// `array[2]`: animación siguiente por defecto (reposo, byte bajo) o espera entre intentos (azar, agrupado).
    pub interval: u32,
    /// `array[3]`: periodo del azar (1 de cada `period`).
    pub period: u32,
    /// `array[4]`: espera tras terminar.
    pub after: u32,
    /// `array[5]`: grupo de la animación.
    pub group: u32,
    /// `array[6]`: grupo siguiente (byte bajo).
    pub next_group: u32,
    /// `array[7]`: repeticiones y `array[8]`: margen aleatorio de repeticiones.
    pub repeat: u32,
    pub repeat_rand: u32,
    pub frames: u16,
}

impl Anim {
    /// Construye una animación desde el `array` de un trozo y su número de fotogramas.
    pub fn from_array(array: &[u32], frames: usize) -> Self {
        let g = |i: usize| array.get(i).copied().unwrap_or(0);
        Anim { duration: g(0), looping: g(1) != 0, interval: g(2), period: g(3), after: g(4), group: g(5), next_group: g(6), repeat: g(7), repeat_rand: g(8), frames: frames as u16 }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Simple,
    Idle,
    Random,
    Grouped,
    GroupedState,
}

impl Kind {
    /// Clase según el número de valores del `array` (`0x552EF0`).
    pub fn from_array_len(len: usize) -> Option<Kind> {
        Some(match len {
            2 => Kind::Simple,
            4 => Kind::Idle,
            5 => Kind::Random,
            7 => Kind::Grouped,
            9 => Kind::GroupedState,
            _ => return None,
        })
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AnimState {
    pub next: u32,
    pub frame: u8,
    pub status: u8,
    pub flags: u8,
    pub group: u8,
    pub repeat: u8,
}

/// Dibujo pedido: índice de animación y fotograma.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Draw {
    pub anim: usize,
    pub frame: u8,
}

impl AnimState {
    /// Avanza el reloj de la animación `a`; devuelve el estado resultante (0 = nada que dibujar).
    fn advance(&mut self, a: &Anim, clock: u32) -> u8 {
        if self.status == ST_START {
            self.status = ST_PLAY;
            return ST_PLAY;
        }
        if clock < self.next {
            self.status = (self.status & 0x80) | ST_HOLD;
            return self.status;
        }
        let elapsed = clock - self.next;
        self.next = clock.wrapping_add(a.duration);
        let n = (elapsed / a.duration).wrapping_add(1);
        let f = self.frame;
        if self.status & 0x80 != 0 {
            if (f as i32).wrapping_sub(n as i32) < 0 {
                self.frame = 0;
                self.status = ST_DONE;
            } else {
                self.frame = f.wrapping_sub(n as u8);
                self.status = ST_PLAY | 0x80;
            }
        } else if (f as u32).wrapping_add(n) < a.frames as u32 {
            self.frame = f.wrapping_add(n as u8);
            self.status = ST_PLAY;
        } else {
            self.frame = (a.frames as u8).wrapping_sub(1);
            self.status = if a.looping { ST_PLAY | 0x80 } else { ST_DONE };
        }
        self.status
    }

    fn restart_with(&mut self, anims: &[Anim], idx: usize, clock: u32) {
        self.frame = 0;
        self.next = anims[idx].duration.wrapping_add(clock);
        self.status = ST_START;
    }

    /// Elige al azar una animación del grupo actual (`0x54EE30` / `0x552500`).
    fn pick_in_group(&self, anims: &[Anim], count: usize, r: u32) -> usize {
        let n = anims[..count].iter().filter(|a| a.group == u32::from(self.group)).count() as u32;
        let k = ((r as i32) % (n as i32)) as u8 as usize;
        let mut seen = 0;
        for (i, a) in anims[..count].iter().enumerate() {
            if a.group == u32::from(self.group) {
                seen += 1;
                if seen > k {
                    return i;
                }
            }
        }
        count
    }

    /// Actualiza y devuelve el dibujo pedido. `restart` es el último argumento del original (`param_8`), `count` el número de animaciones.
    pub fn update(&mut self, kind: Kind, anims: &[Anim], count: usize, restart: bool, clock: u32, mut rand: impl FnMut() -> u32) -> Option<Draw> {
        let cur = (self.flags & 0x3F) as usize;
        if self.flags & 0x80 != 0 {
            return self.waiting(kind, anims, count, restart, clock, &mut rand);
        }
        let b = self.advance(&anims[cur], clock);
        if b == 0 {
            return None;
        }
        let draw = Draw { anim: cur, frame: self.frame };
        if !restart {
            self.flags |= 0xC0;
            return Some(draw);
        }
        if self.status & 0x7F == ST_DONE {
            let a = &anims[cur];
            match kind {
                Kind::Simple | Kind::Idle => self.flags |= 0x80,
                Kind::Random => {
                    self.next = a.after.wrapping_add(clock);
                    self.flags |= 0x80;
                }
                Kind::Grouped => {
                    self.flags |= 0x80;
                    self.group = a.next_group as u8;
                    self.next = a.after.wrapping_add(clock);
                }
                Kind::GroupedState => {
                    self.repeat = self.repeat.wrapping_sub(1);
                    if self.repeat == 0 {
                        self.flags |= 0x80;
                        self.group = a.next_group as u8;
                        self.next = a.after.wrapping_add(clock);
                    } else {
                        self.frame = 0;
                        self.next = a.duration.wrapping_add(clock);
                        self.status = ST_START;
                    }
                }
            }
        }
        Some(draw)
    }

    fn waiting(&mut self, kind: Kind, anims: &[Anim], count: usize, restart: bool, clock: u32, rand: &mut impl FnMut() -> u32) -> Option<Draw> {
        let cur = (self.flags & 0x3F) as usize;
        match kind {
            Kind::Simple => {
                if !restart {
                    return None;
                }
                self.frame = 0;
                self.next = anims[cur].duration.wrapping_add(clock);
                self.status = ST_START;
                self.flags &= 0x3F;
                Some(Draw { anim: cur, frame: 0 })
            }
            Kind::Idle => {
                if !restart {
                    return None;
                }
                let a = anims[cur];
                let b = if a.period == 0 || rand() % a.period != 0 { a.interval as u8 } else { ((rand() as i32) % (count as i32)) as u8 };
                self.flags = b;
                self.restart_with(anims, b as usize, clock);
                Some(Draw { anim: cur, frame: 0 })
            }
            Kind::Random => {
                if self.next > clock || !restart {
                    return None;
                }
                let a = anims[cur];
                if rand() % a.period != 0 {
                    return self.retry(anims, a.interval, clock);
                }
                let idx = (((rand() as i32) % (count as i32)) as u8) as usize;
                self.flags = idx as u8;
                self.restart_with(anims, idx, clock);
                Some(Draw { anim: idx, frame: 0 })
            }
            Kind::Grouped | Kind::GroupedState => {
                if self.next > clock || !restart {
                    return None;
                }
                let a = anims[cur];
                let r = rand();
                if r % a.period != 0 {
                    return self.retry(anims, a.interval, clock);
                }
                let idx = self.pick_in_group(anims, count, r);
                self.flags = idx as u8;
                self.restart_with(anims, idx, clock);
                if kind == Kind::GroupedState {
                    let b = anims[idx];
                    self.repeat = b.repeat as u8;
                    if b.repeat_rand != 0 {
                        self.repeat = self.repeat.wrapping_add((rand() % b.repeat_rand) as u8);
                    }
                }
                Some(Draw { anim: idx, frame: self.frame })
            }
        }
    }

    /// El azar no salió: redibuja si había dibujo pendiente y espera `interval` más.
    fn retry(&mut self, anims: &[Anim], interval: u32, clock: u32) -> Option<Draw> {
        let _ = anims;
        let mut draw = None;
        if self.flags & 0x40 != 0 {
            self.flags &= 0xBF;
            draw = Some(Draw { anim: (self.flags & 0x3F) as usize, frame: self.frame });
        }
        self.next = interval.wrapping_add(clock);
        draw
    }
}

/// `rand()` del CRT de Microsoft (`0x639C03`, verificado en C02): congruencial lineal con 15 bits de salida.
#[derive(Clone, Copy, Debug)]
pub struct CrtRand(pub u32);

impl CrtRand {
    pub fn next(&mut self) -> u32 {
        self.0 = self.0.wrapping_mul(214013).wrapping_add(2531011);
        (self.0 >> 16) & 0x7FFF
    }
}

impl AnimState {
    /// Estado inicial de los constructores (`0x54EB10`, `0x551810`, `0x5524E0`): espera con dibujo pendiente.
    pub fn new() -> Self {
        AnimState { next: 0, frame: 0, status: 0, flags: 0xC0, group: 0, repeat: 1 }
    }
}
