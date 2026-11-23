//! J04: ranuras de sonido con tiempo de la escena Class22 (paridad x86: 0x595B00, 0x594AA0, 0x594770).
//! `mem` es la imagen de la escena con los desplazamientos absolutos del original (`escena+0xC998` en adelante).
//! Los servicios del gestor de sonido (0x443150, 0x443690, `AIL_sample_ms_position`, el reloj y el azar del CRT) entran por [`SlotEnv`].
use crate::triggers::{pan_volume, route, PanScene, Route};

/// Tamaño mínimo de la imagen de escena que cubre estos campos (la tabla de avisos por tipo llega hasta 0xC7EC + 4·65535).
pub const SCENE_SIZE: usize = 0x4D000;
const POS_SLOTS: usize = 0xC998;
const EXCL_SLOTS: usize = 0xCB30;
const STRIDE: usize = 0x18;

/// Servicios externos de las ranuras.
pub trait SlotEnv {
    /// `0x4B2200`: el sistema de sonido está listo.
    fn ready(&mut self) -> bool;
    /// `0x443690`: número de muestras de la lista `list`.
    fn list_count(&mut self, list: u32) -> u32;
    /// `_rand()`.
    fn rand(&mut self) -> u32;
    /// `0x443150(id, a, b, volumen, panorámica, 0, 0)`: manejador de la muestra (0 si no suena).
    fn play(&mut self, id: u32, a: u32, b: u32, volume: u32, pan: u32) -> u32;
    /// `AIL_sample_ms_position`: duración total en ms de la muestra del manejador.
    fn sample_ms(&mut self, handle: u32) -> u32;
    /// `DAT_006DCCF8`: reloj de presentación en ms.
    fn clock(&mut self) -> u32;
    /// `AIL_sample_status`: 2 = terminada.
    fn sample_status(&mut self, handle: u32) -> u32;
    /// `0x4430D0`: libera las marcas de la ranura de muestra.
    fn clear_flags(&mut self, handle: u32);
    /// `0x443000`: volumen de la muestra.
    fn set_volume(&mut self, handle: u32, volume: u32);
    /// `0x4438D0`: panorámica de la muestra.
    fn set_pan(&mut self, handle: u32, pan: u32);
}

#[derive(Debug)]
pub enum SlotError {
    /// El x86 se detendría con #DE (PanDist/VolDist cero) o por desbordamiento de la división.
    PanDivision,
    /// `c7e4`/`c7e8` cero en el reparto del siguiente aviso.
    IntervalZero,
    /// Una ranura ocupada y vencida sin muestra: el original lee el manejador nulo (acceso a la dirección 0).
    NullHandle,
}

pub struct SoundScene {
    pub mem: Vec<u8>,
}

impl SoundScene {
    pub fn new() -> Self {
        Self { mem: vec![0; SCENE_SIZE] }
    }
    fn r32(&self, o: usize) -> u32 {
        u32::from_le_bytes(self.mem[o..o + 4].try_into().unwrap())
    }
    fn w32(&mut self, o: usize, v: u32) {
        self.mem[o..o + 4].copy_from_slice(&v.to_le_bytes());
    }
    fn r16(&self, o: usize) -> u16 {
        u16::from_le_bytes(self.mem[o..o + 2].try_into().unwrap())
    }
    fn pan_scene(&self) -> PanScene {
        PanScene {
            view_w: self.r32(0x2C) as i32,
            view_h: self.r32(0x30) as i32,
            origin_x: self.r32(0xC650) as i32,
            origin_y: self.r32(0xC654) as i32,
            pan_dist: self.r32(0xC7D4) as i32,
            vol_dist: self.r32(0xC7D8) as i32,
        }
    }
    fn pan_vol(&self, x: i32, y: i32) -> Result<(u8, u8), SlotError> {
        pan_volume(&self.pan_scene(), x, y).ok_or(SlotError::PanDivision)
    }

    /// `0x595B00` (`class22_play_sound_at_position`): devuelve el byte bajo del retorno.
    pub fn play_at_position(&mut self, env: &mut dyn SlotEnv, x: i32, y: i32, id: u32) -> Result<u8, SlotError> {
        if !env.ready() {
            return Ok(0);
        }
        if let Route::Exclusive(id) = route(id) {
            return self.play_exclusive(env, u32::from(id));
        }
        let (pan, vol) = self.pan_vol(x, y)?;
        if vol != 0 {
            let mut i = 4;
            while self.mem[POS_SLOTS + i * STRIDE + 0x16] != 0 {
                i += 1;
                if i > 0xD {
                    return Ok(0);
                }
            }
            let base = POS_SLOTS + i * STRIDE;
            let h = env.play(id & 0xFFFF, 0, 1, u32::from(vol), u32::from(pan));
            self.w32(base + 0xC, h);
            if h != 0 {
                let ms = env.sample_ms(h);
                let end = env.clock().wrapping_add(ms);
                self.w32(base, x as u32);
                self.w32(base + 0x10, end);
                self.w32(base + 4, y as u32);
                self.mem[base + 0x16] = 1;
                self.mem[base + 0x14..base + 0x16].copy_from_slice(&(id as u16).to_le_bytes());
            }
        }
        Ok(1)
    }

    /// `0x594AA0` (`class22_play_ship_sound_exclusive`): cuatro ranuras, sin repetir el mismo ID activo.
    pub fn play_exclusive(&mut self, env: &mut dyn SlotEnv, id: u32) -> Result<u8, SlotError> {
        for i in 0..4 {
            let b = EXCL_SLOTS + i * STRIDE;
            if self.mem[b + 0x16] != 0 && u32::from(self.r16(b + 0x14)) == id {
                return Ok(0);
            }
        }
        let Some(i) = (0..4).find(|&i| self.mem[EXCL_SLOTS + i * STRIDE + 0x16] == 0) else {
            return Ok(0);
        };
        let base = EXCL_SLOTS + i * STRIDE;
        let h = env.play(id, 0, 1, 0xFFFF_FFFF, 0x40);
        self.w32(base + 0xC, h);
        if h != 0 {
            let ms = env.sample_ms(h);
            self.mem[base + 0x16] = 1;
            self.mem[base + 0x14..base + 0x16].copy_from_slice(&(id as u16).to_le_bytes());
            let end = env.clock().wrapping_add(ms);
            self.w32(base + 0x10, end);
        }
        Ok(1)
    }

    /// `0x594770` (`class22_start_pending_building_sounds`): arranca el sonido de cada una de las cuatro ranuras de edificio marcadas y sin muestra.
    pub fn start_pending_building_sounds(&mut self, env: &mut dyn SlotEnv) -> Result<(), SlotError> {
        for k in 0..4 {
            let base = POS_SLOTS + k * STRIDE;
            if self.mem[base + 0x16] == 0 || self.r32(base + 0xC) != 0 {
                continue;
            }
            let kind = u32::from(self.r16(base + 0x14));
            let first = kind.wrapping_mul(self.r32(0xC7E0)).wrapping_add(self.r32(0xC7DC));
            let n = env.list_count(first) as i32;
            let pick = if n > 1 { env.rand() as i32 % n } else { 0 };
            let (x, y) = (self.r32(base) as i32, self.r32(base + 4) as i32);
            let (pan, vol) = self.pan_vol(x, y)?;
            let h = env.play(first.wrapping_add(pick as u32), 1, 1, u32::from(vol), u32::from(pan));
            self.w32(base + 0xC, h);
            if h == 0 {
                self.mem[base..base + STRIDE].fill(0);
            } else {
                let ms = env.sample_ms(h);
                let end = env.clock().wrapping_add(ms);
                self.w32(base + 0x10, end);
                if kind as u16 != 0 {
                    let (e8, e4) = (self.r32(0xC7E8), self.r32(0xC7E4));
                    let r = env.rand();
                    let r8 = r.checked_rem(e8).ok_or(SlotError::IntervalZero)?;
                    let r4 = r.checked_rem(e4).ok_or(SlotError::IntervalZero)?;
                    self.w32(base + 0x10, end.wrapping_add(r8).wrapping_add(e8 >> 1));
                    let table = 0xC7EC + (kind as u16 as usize) * 4;
                    self.w32(table, (e4 >> 1).wrapping_add(end).wrapping_add(r4));
                }
            }
        }
        Ok(())
    }

    fn expire(&mut self, env: &mut dyn SlotEnv, base: usize) -> Result<(), SlotError> {
        if self.mem[base + 0x16] != 0 && self.r32(base + 0x10) < env.clock() {
            let h = self.r32(base + 0xC);
            if h == 0 {
                return Err(SlotError::NullHandle);
            }
            if env.sample_status(h) == 2 {
                env.clear_flags(h);
                self.mem[base..base + STRIDE].fill(0);
            }
        }
        Ok(())
    }

    /// `0x5948D0` (`class22_update_sound_channels`): libera las muestras terminadas y, si la cámara se movió, reajusta volumen y panorámica.
    pub fn update_channels(&mut self, env: &mut dyn SlotEnv) -> Result<(), SlotError> {
        for k in 0..17 {
            self.expire(env, POS_SLOTS + k * STRIDE)?;
        }
        if self.r32(0xC640) != self.r32(0xC648) || self.r32(0xC644) != self.r32(0xC64C) {
            for k in 0..17 {
                let base = POS_SLOTS + k * STRIDE;
                let h = self.r32(base + 0xC);
                if h != 0 {
                    let (pan, vol) = self.pan_vol(self.r32(base) as i32, self.r32(base + 4) as i32)?;
                    env.set_volume(h, u32::from(vol));
                    env.set_pan(h, u32::from(pan));
                }
            }
            for k in 0..4 {
                self.expire(env, EXCL_SLOTS + k * STRIDE)?;
            }
        }
        Ok(())
    }

    /// `0x594C00` (`class22_queue_building_sound`): reserva una de las cuatro ranuras para el sonido ambiental de un edificio, prefiriendo los más cercanos.
    pub fn queue_building_sound(&mut self, env: &mut dyn SlotEnv, x: i32, y: i32, kind: u16) {
        let t = u32::from(kind);
        if env.list_count(self.r32(0xC7E0).wrapping_mul(t).wrapping_add(self.r32(0xC7DC))) == 0 {
            return;
        }
        let cy = self.r32(0xC654);
        let ux = (x as u32).wrapping_add(self.r32(0xC650));
        let uy = (y as u32).wrapping_add(cy.wrapping_sub(self.r32(0x18)));
        let mut slots = if self.mem[0x56C] == 0 || kind == 0 { 0 } else { 1 };
        if self.r32(0xC7EC + (kind as usize) * 4) > env.clock() {
            return;
        }
        slots = 4 - slots;
        let busy = |s: &Self, i: usize| s.mem[POS_SLOTS + i * STRIDE + 0x16] != 0;
        if slots > 0 {
            let mut i = 0;
            loop {
                if !busy(self, i) {
                    break;
                }
                i += 1;
                if i >= slots {
                    break;
                }
            }
            if i == 4 {
                return;
            }
        }
        let half_w = (self.r32(0x2C) as i32) / 2;
        let half_h = (self.r32(0x30) as i32) / 2;
        let dx = (ux.wrapping_sub(half_w.wrapping_add(self.r32(0xC650) as i32) as u32) as i32).wrapping_abs();
        let dy = (uy.wrapping_sub(half_h.wrapping_add(cy as i32) as u32) as i32).wrapping_abs();
        let d = dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx)) as u32;
        let radius = self.r32(0xC7D0);
        if d > radius.wrapping_mul(radius) {
            return;
        }
        let store = |s: &mut Self, i: usize| {
            let base = POS_SLOTS + i * STRIDE;
            s.w32(base + 8, d);
            s.w32(base, ux);
            s.w32(base + 4, uy);
            s.mem[base + 0x14..base + 0x16].copy_from_slice(&kind.to_le_bytes());
            s.mem[base + 0x16] = 1;
        };
        if kind != 0 {
            for i in 0..slots {
                let base = POS_SLOTS + i * STRIDE;
                if self.r32(base + 0xC) == 0 && self.mem[base + 0x16] != 0 {
                    if self.r32(base + 8) <= d {
                        return;
                    }
                    store(self, i);
                    return;
                }
            }
        }
        for i in 0..slots {
            let base = POS_SLOTS + i * STRIDE;
            if self.r32(base + 0xC) == 0 && self.mem[base + 0x16] == 0 {
                store(self, i);
                return;
            }
        }
        for i in 0..slots {
            let base = POS_SLOTS + i * STRIDE;
            if self.r32(base + 0xC) == 0 && self.r32(base + 8) > d {
                store(self, i);
                return;
            }
        }
    }

    /// `0x599040` (cola final, `class22_remove_building_with_effects`): sonido de derrumbe en el punto de pantalla `(x, y)`.
    /// `[escena+0xC994]` es `BuildingCollapseList`; consume el azar solo si el volumen no es cero. El caso de otro jugador
    /// (`FUN_004b9730 != DAT_006cbb40`) lo decide el llamador con `local_player`.
    pub fn play_collapse(&mut self, env: &mut dyn SlotEnv, x: i32, y: i32, local_player: bool) -> Result<bool, SlotError> {
        if !env.ready() || !local_player {
            return Ok(false);
        }
        let list = self.r32(0xC994);
        let n = env.list_count(list);
        if n == 0 {
            return Ok(false);
        }
        let (pan, vol) = self.pan_vol(x, y)?;
        if vol == 0 {
            return Ok(false);
        }
        let r = env.rand();
        env.play(r % n + list, 0, 0, u32::from(vol), u32::from(pan));
        Ok(true)
    }
}

impl Default for SoundScene {
    fn default() -> Self {
        Self::new()
    }
}
