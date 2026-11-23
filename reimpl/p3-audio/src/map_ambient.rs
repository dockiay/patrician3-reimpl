//! J04: sonidos del mapa de navegación (`ScrollMap`).
//!
//! - `0x459790` (`scroll_map_play_ambient_sound(clase, id, x, y, extra)`): cinco clases con sus colas de cuatro entradas.
//!   1 = ambiente único (para todo antes de sonar), 2 = avistamientos (lista 2000; si ya sonó uno se encola en `+0x2CD8`),
//!   3 = tiempo (tormenta/calma/iceberg, listas `(extra + 30) * 100`), 4 y 5 = listas 4000 y 5000 con volumen `0.8·v + 0.5`.
//!   Una entrada no se repite mientras siga sonando la misma `id`; las vencidas se paran (`0x4430D0`) y se compactan.
//! - `0x45A5F0`: volumen y panorámica a partir de la **primera** entrada de la cola (el original pasa la base de la lista).
//! - `0x459380` (`scroll_map_prepare_weather_entity_display`): recorre las 16 entidades de tiempo (`0x6DD7C0`), guarda tipo,
//!   tamaño y posición en el mapa, repinta las visibles cuyo plazo `+0x2A28` venció y les pide el sonido de clase 3.
//!   Su segundo bucle (tabla `0x6DDB90`) prueba el bit 0x4000000 de una palabra de 16 bits, así que nunca actúa: solo
//!   oculta los tres sprites `+0x1A28`.
//!
//! Observado y probado en emulador (`oracle/x86_map_ambient.py`, `oracle/x86_map_weather.py`). [`MapScene::mem`] es la imagen del
//! objeto del mapa con los desplazamientos del original; reproduce también los defectos del x86 (la clase 5 escribe la duración y
//! la distancia en la entrada siguiente a la nueva y lee el manejador de esa entrada).

/// Tamaño de la imagen del objeto del mapa que cubre estos campos.
pub const MAP_SIZE: usize = 0x2D20;

/// Servicios del gestor de sonido y del azar.
pub trait MapSoundEnv {
    fn rand(&mut self) -> u32;
    /// `0x443690(lista)`
    fn list_count(&mut self, list: u32) -> u32;
    /// `0x443150(id, a, b, volumen, panorámica, 0, 0)`: puntero al manejador (0 si no suena).
    fn play(&mut self, id: u32, a: u32, b: u32, volume: u32, pan: u32) -> u32;
    /// `AIL_sample_ms_position(*manejador, &total, 0)`: duración total en ms.
    fn sample_ms(&mut self, handle: u32) -> u32;
    /// `0x4430D0(*manejador)`
    fn clear_flags(&mut self, handle: u32);
    /// `0x443940`: para todos los sonidos del catálogo.
    fn stop_all(&mut self);
}

/// Ganchos de `0x45A470` (mantenimiento por fotograma). `handle` es el valor de la entrada (`+0`), que el original desreferencia.
pub trait MapFrameEnv {
    /// `AIL_sample_status(*handle)`; 4 = sonando.
    fn sample_status(&mut self, handle: u32) -> u32;
    /// `0x443000(*handle, volumen)`.
    fn set_volume(&mut self, handle: u32, volume: u8);
    /// `0x4438D0(*handle, panorámica)`.
    fn set_pan(&mut self, handle: u32, pan: u8);
}

/// Servicios de dibujo de `0x459380`.
pub trait WeatherHost {
    /// `0x4B9650(&rect)` sobre `0x6DA5F0`.
    fn minimap_repaint(&mut self, r: [i32; 4]);
    /// `0x453E30(&rect)` del mapa.
    fn map_repaint(&mut self, r: [i32; 4]);
    /// `0x459790(clase, id, x, y, extra)` (para probar `0x459380` por separado).
    fn ambient(&mut self, class: u32, id: u32, x: i32, y: i32, extra: u32);
    /// `0x4AD8D0(0, 0)` sobre el sprite `+0x1A28 + i·0xD8`.
    fn hide_sprite(&mut self, i: usize);
}

/// Una entidad de tiempo de `0x6DD7C0` (12 bytes).
#[derive(Clone, Copy, Debug, Default)]
pub struct WeatherEntity {
    /// `+0`: palabra baja comparada con `DAT_006DD894`.
    pub key: u32,
    /// `+4`, `+6`
    pub x: i16,
    pub y: i16,
    /// `+8`: tipo (0 tormenta, 1, 2); `+0xA`: activa.
    pub kind: u16,
    pub active: u16,
}

pub struct MapScene {
    pub mem: Vec<u8>,
}

impl Default for MapScene {
    fn default() -> Self {
        Self::new()
    }
}

/// `n / 2` con signo como `cdq; sub eax, edx; sar eax, 1`.
fn half(n: i32) -> i32 {
    n.wrapping_sub(n >> 31) >> 1
}

/// `__ftol` (truncamiento).
fn ftol(v: f64) -> i32 {
    v as i64 as i32
}

/// Colas de las clases 2..5: (contador, base de las entradas).
const QUEUES: [(usize, usize); 4] = [(0x2B46, 0x2B94), (0x2B48, 0x2BD4), (0x2B4A, 0x2C14), (0x2B4C, 0x2C54)];

impl MapScene {
    pub fn new() -> Self {
        Self { mem: vec![0; MAP_SIZE] }
    }
    pub fn r32(&self, o: usize) -> u32 {
        u32::from_le_bytes(self.mem[o..o + 4].try_into().unwrap())
    }
    pub fn w32(&mut self, o: usize, v: u32) {
        self.mem[o..o + 4].copy_from_slice(&v.to_le_bytes());
    }
    pub fn r16(&self, o: usize) -> u16 {
        u16::from_le_bytes(self.mem[o..o + 2].try_into().unwrap())
    }
    pub fn w16(&mut self, o: usize, v: u16) {
        self.mem[o..o + 2].copy_from_slice(&v.to_le_bytes());
    }
    fn i16_at(&self, o: usize) -> i32 {
        i32::from(self.r16(o) as i16)
    }

    /// Distancia² al centro de la vista, truncada a 16 bits como la guarda el original (`+8` de la entrada).
    fn dist2(&self, x: i32, y: i32) -> u16 {
        // `ax` (16 bits) se calcula sobre el resultado de `sar`, y la parte alta de `eax` se conserva.
        let mut dy = half(self.r32(0x30) as i32) as u32;
        dy = (dy & 0xFFFF_0000) | u32::from((dy as u16).wrapping_sub(self.r16(0x29E6)).wrapping_add(self.r16(0x18)));
        let dy = (dy as i32).wrapping_add(y);
        let mut dx = half(self.r32(0x2C) as i32) as u32;
        dx = (dx & 0xFFFF_0000) | u32::from((dx as u16).wrapping_sub(self.r16(0x29E4)));
        let dx = (dx as i32).wrapping_add(x);
        dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy)) as u16
    }

    /// `0x45A5F0(entrada, &pan, &vol)`: devuelve `(pan, vol)`.
    pub fn pan_volume(&self, entry: usize) -> (u8, u8) {
        let dx = (-half(self.r32(0x2C) as i32)).wrapping_sub(self.i16_at(0x29E4)).wrapping_add(i32::from(self.r16(entry + 4)));
        let dy = (-half(self.r32(0x30) as i32))
            .wrapping_sub(self.i16_at(0x29E6))
            .wrapping_add(i32::from(self.r16(entry + 6)))
            .wrapping_sub(self.r32(0x18) as i32);
        let r = self.r32(0x2B34) as i32;
        let d = dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx));
        let t = (d.wrapping_shl(7).wrapping_sub(d) as u32) / (r.wrapping_mul(r) as u32);
        let p = (dx.wrapping_shl(6)).wrapping_div(self.r32(0x2B30) as i32).wrapping_add(0x40);
        let vol = if (t as i32) < 0x7F { 0x7Fu8.wrapping_sub(t as u8) } else { 0 };
        let pan = p.clamp(0, 0x7F) as u8;
        (pan, vol)
    }

    /// Quita las entradas vencidas (`+0xC <= reloj`) parándolas. `recheck` = las clases 2..4 vuelven a mirar la entrada desplazada.
    fn expire(&mut self, count_at: usize, base: usize, clock: u32, recheck: bool, env: &mut dyn MapSoundEnv) {
        let mut i = 0usize;
        while i < usize::from(self.r16(count_at)) {
            let e = base + 16 * i;
            if self.r32(e + 0xC) <= clock {
                env.clear_flags(self.r32(e));
                let n = usize::from(self.r16(count_at)).wrapping_sub(i + 1) * 16;
                self.mem.copy_within(e + 16..e + 16 + n, e);
                self.w16(count_at, self.r16(count_at).wrapping_sub(1));
                if recheck {
                    continue;
                }
            }
            i += 1;
        }
    }

    /// Hay en la cola una entrada con la misma `id` que sigue sonando.
    fn busy(&self, count_at: usize, base: usize, id: u16, clock: u32) -> bool {
        (0..usize::from(self.r16(count_at))).any(|i| {
            let e = base + 16 * i;
            clock < self.r32(e + 0xC) && self.r16(e + 0xA) == id
        })
    }

    /// `0x459790`. `clock` es `DAT_006DCCF0`; `volume` el byte `+0x23` de los ajustes (`[0x6CC3E8]`).
    pub fn play_ambient(&mut self, class: u32, id: u32, x: i32, y: i32, extra: u32, clock: u32, volume: u8, env: &mut dyn MapSoundEnv) {
        let r = (env.rand() as i32 as f32 / 32768.0f32) as f64;
        let pick = |n: u32| ftol(f64::from(n as i32 as f32) * r);
        let id16 = id as u16;
        match class {
            1 => {
                if self.mem[0x2D14] != 0 {
                    return;
                }
                env.stop_all();
                let e = 0x2B54 + 16 * usize::from(self.r16(0x2B44));
                self.w16(e + 4, x as u16);
                self.w16(e + 6, y as u16);
                self.w16(e + 0xA, id16);
                self.w16(0x2B38, u16::from(volume));
                let n = env.list_count(1000);
                let h = env.play(pick(n).wrapping_add(1000) as u32, 0, 0, u32::from(volume), 0x40);
                let e = 0x2B54 + 16 * usize::from(self.r16(0x2B44));
                self.w32(e, h);
                if h == 0 {
                    return;
                }
                self.w32(e + 0xC, clock);
                let d = self.dist2(x, y);
                self.w16(0x2B54 + 16 * usize::from(self.r16(0x2B44)) + 8, d);
            }
            2 if self.mem[0x2D14] != 0 => {
                let n = self.r16(0x2B50);
                if n >= 4 {
                    return;
                }
                let e = 0x2CD8 + 16 * usize::from(n);
                self.w16(e + 6, id16);
                self.w16(e, x as u16);
                self.w16(e + 2, y as u16);
                self.w16(0x2B50, n + 1);
            }
            2..=5 => {
                if class != 2 && self.mem[0x2D14] != 0 {
                    return;
                }
                let (count_at, base) = QUEUES[class as usize - 2];
                if self.busy(count_at, base, id16, clock) {
                    return;
                }
                self.expire(count_at, base, clock, class != 5, env);
                let n = self.r16(count_at);
                if n == 4 {
                    return;
                }
                let e = base + 16 * usize::from(n);
                self.w16(e + 4, x as u16);
                self.w16(e + 6, y as u16);
                self.w16(e + 0xA, id16);
                let (h, len_entry, end) = match class {
                    2 => {
                        self.w16(0x2B3A, u16::from(volume));
                        let c = env.list_count(2000);
                        let h = env.play(pick(c).wrapping_add(2000) as u32, 0, 0, 0xFFFF_FFFF, 0x40);
                        self.w32(e, h);
                        if h == 0 {
                            return;
                        }
                        let ms = env.sample_ms(h);
                        (h, e, clock.wrapping_add(ms).wrapping_add(10_000))
                    }
                    3 => {
                        self.w16(0x2B3C, u16::from(volume));
                        let (pan, vol) = self.pan_volume(base);
                        let c = env.list_count(((extra & 0xFFFF) + 30) * 100);
                        let k = pick(c);
                        if vol == 0 {
                            return;
                        }
                        let h = env.play(k.wrapping_add(3000) as u32, 0, 1, u32::from(vol), u32::from(pan));
                        self.w32(e, h);
                        if h == 0 {
                            return;
                        }
                        let ms = env.sample_ms(h);
                        (h, e, clock.wrapping_add(ms).wrapping_add(6000))
                    }
                    4 => {
                        let v = ftol(f64::from(volume) * 0.8 + 0.5);
                        self.w16(0x2B3E, v as u16);
                        let (pan, vol) = self.pan_volume(base);
                        let c = env.list_count(4000);
                        let k = pick(c);
                        if vol == 0 {
                            return;
                        }
                        let h = env.play(k.wrapping_add(4000) as u32, 0, 0, u32::from(vol), u32::from(pan));
                        self.w32(e, h);
                        if h == 0 {
                            return;
                        }
                        let ms = env.sample_ms(h);
                        let t = ftol(r * f64::from(2000.0f32) + f64::from(ms as i32 as f32) + f64::from(4000.0f32));
                        (h, e, clock.wrapping_add(t as u32 & 0xFFFF))
                    }
                    _ => {
                        let v = ftol(f64::from(volume) * 0.8 + 0.5) as u16;
                        self.w16(0x2B40, v);
                        let c = env.list_count(5000);
                        let h = env.play(pick(c).wrapping_add(5000) as u32, 0, 0, u32::from(v), 0x40);
                        self.w32(e, h);
                        if h == 0 {
                            return;
                        }
                        // El original incrementa el contador antes de leer la duración: usa el manejador, la duración y la distancia
                        // de la entrada siguiente.
                        self.w16(count_at, n.wrapping_add(1));
                        let next = base + 16 * usize::from(n.wrapping_add(1));
                        let ms = env.sample_ms(self.r32(next));
                        let t = ftol((r + f64::from(1.0f32)) * f64::from(2000.0f32) + f64::from(ms as i32 as f32));
                        (h, next, clock.wrapping_add(t as u32 & 0xFFFF))
                    }
                };
                let _ = h;
                self.w32(len_entry + 0xC, end);
                let d = self.dist2(x, y);
                self.w16(len_entry + 8, d);
                if class != 5 {
                    self.w16(count_at, n.wrapping_add(1));
                }
                if class == 2 {
                    self.mem[0x2D14] = 1;
                }
            }
            _ => {}
        }
    }

    /// `0x45A470`, por fotograma: suelta `+0x2D14` cuando ninguna entrada de la clase 2 sigue sonando y, con el mapa visible
    /// (`+0x2B28`), ajusta volumen y panorámica de las colas 3 y 4 según su posición. Como el original, guarda el último volumen
    /// calculado en el ajuste de la clase (`+0x2B3C`, `+0x2B3E`).
    pub fn update_frame(&mut self, env: &mut dyn MapFrameEnv) {
        if self.mem[0x2D14] != 0 {
            let n = usize::from(self.r16(0x2B46));
            let playing = (0..n).any(|i| env.sample_status(self.r32(0x2B94 + 16 * i)) == 4);
            if !playing {
                self.mem[0x2D14] = 0;
            }
        }
        if self.mem[0x2B28] == 0 {
            return;
        }
        for (count_at, base, setting) in [(0x2B48, 0x2BD4, 0x2B3C), (0x2B4A, 0x2C14, 0x2B3E)] {
            let mut i = 0;
            while i < usize::from(self.r16(count_at)) {
                let e = base + 16 * i;
                let (pan, vol) = self.pan_volume(e);
                self.w16(setting, u16::from(vol));
                env.set_volume(self.r32(e), vol);
                env.set_pan(self.r32(e), pan);
                i += 1;
            }
        }
    }

    /// `0x459380`. `town_count` es `DAT_006DD894` = `+0xF4` del gestor naval `0x6DD7A0` (número de barcos; la clave de la entidad es
    /// un índice de barco), aunque el nombre del parámetro se conserva; `clock` es `DAT_006DCCF0`.
    pub fn prepare_weather(&mut self, entities: &[WeatherEntity; 16], town_count: u16, clock: u32, host: &mut dyn WeatherHost) {
        let view = [
            self.r32(0x14) as i32,
            self.r32(0x18) as i32,
            (self.r32(0x14) as i32).wrapping_add(self.r32(0x2C) as i32),
            (self.r32(0x18) as i32).wrapping_add(self.r32(0x30) as i32),
        ];
        for k in 0..16 {
            self.mem[0x2A88 + k] = 0xFF;
        }
        for (i, w) in entities.iter().enumerate() {
            if (w.key as u16) >= town_count || w.active == 0 {
                continue;
            }
            let left = i32::from(w.x).wrapping_sub(self.i16_at(0x29E4)).wrapping_sub(0x20);
            let mut top = i32::from(w.y).wrapping_sub(self.i16_at(0x29E6)).wrapping_add(self.r32(0x18) as i32).wrapping_sub(0x20);
            let mut rect = [left, top, left.wrapping_add(0x40), top.wrapping_add(0x40)];
            if w.kind == 0 {
                top = rect[1].wrapping_sub(0x1E);
                rect[1] = top;
                rect[3] = rect[3].wrapping_add(0x1E);
            }
            let (inter, visible) = intersect_rect(&view, &rect);
            rect = inter;
            self.mem[0x2A88 + i] = w.kind as u8;
            let size = match w.kind {
                0 => Some(0x30),
                1 => Some(0x1E),
                2 => Some(0xF),
                _ => None,
            };
            if let Some(s) = size {
                self.mem[0x2A68 + i] = s;
                self.mem[0x2A98 + i] = 0x64;
            }
            self.w16(0x2AB8 + 4 * i, w.x as u16);
            let y = if w.kind == 0 { (w.y as u16).wrapping_sub(0x1E) } else { w.y as u16 };
            self.w16(0x2ABA + 4 * i, y);
            if visible && self.r32(0x2A28 + 4 * i) < clock {
                host.minimap_repaint(rect);
                host.map_repaint(rect);
                let sy = self.i16_at(0x29E6).wrapping_add(self.r32(0x18) as i32).wrapping_add(rect[1]);
                let sx = self.i16_at(0x29E4).wrapping_add(rect[0]);
                host.ambient(3, i as u32, sx, sy, u32::from(w.kind) | u32::from(w.active) << 16);
            }
        }
        for i in 0..3 {
            host.hide_sprite(i);
        }
    }
}

/// `IntersectRect` de user32: `(intersección, no vacía)`; si es vacía devuelve ceros.
pub fn intersect_rect(a: &[i32; 4], b: &[i32; 4]) -> ([i32; 4], bool) {
    let r = [a[0].max(b[0]), a[1].max(b[1]), a[2].min(b[2]), a[3].min(b[3])];
    let empty = |r: &[i32; 4]| r[2] <= r[0] || r[3] <= r[1];
    if empty(a) || empty(b) || empty(&r) {
        ([0; 4], false)
    } else {
        (r, true)
    }
}
