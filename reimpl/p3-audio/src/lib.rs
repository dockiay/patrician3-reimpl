//! Control de canales y fundidos, más recursos nativos sin dispositivo ni hilos.
//! El control no hace E/S; native solo lee los archivos montados explícitamente.
//!
//! Los handles son identificadores opacos del backend. La imagen conserva los bytes
//! del estado original para contrastar las transiciones; initialize conserva los
//! bytes que el constructor deja sin escribir. No convierte direcciones del EXE
//! en punteros nativos.

pub mod banks;
pub mod driver;
pub mod effects;
pub mod filter;
pub mod lifecycle;
pub mod live;
pub mod ring;
pub mod lowpass;
pub mod map_ambient;
pub mod native;
pub mod region;
pub mod samples;
pub mod slots;
pub mod streams;
pub mod triggers;
pub mod voice;
pub mod web;

pub const IMAGE_SIZE: usize = 0x1e4;
const SAMPLES: usize = 20;
const AMBIENT: usize = 0;
const AMBIENT_OUT: usize = 4;
const MUSIC: usize = 8;
const MUSIC_OUT: usize = 12;
const BUILDING: usize = 16;
const VOICE: usize = 20;
const DRIVER: usize = 0x68;
const MUSIC_VOLUME: usize = 0x1b8;
const EFFECTS_VOLUME: usize = 0x1bc;
const AMBIENT_FADE: usize = 0x1c6;
const MUSIC_FADE: usize = 0x1c7;

/// Servicios externos al controlador. El backend define la vida de sus handles.
/// Los corpus de control guionizan estos servicios; los contrastes del backend
/// nativo y de rutinas Miles tienen su propio alcance documentado.
pub trait Backend {
    fn sample_status(&mut self, handle: u32) -> u32;
    fn stream_status(&mut self, handle: u32) -> u32;
    fn stream_volume(&mut self, handle: u32) -> i32;
    fn set_stream_volume(&mut self, handle: u32, volume: i32);
    fn sample_pan(&mut self, handle: u32) -> i32;
    fn set_sample_pan(&mut self, handle: u32, pan: u16);
    fn pause_stream(&mut self, handle: u32, pause: u32);
    fn close_stream(&mut self, handle: u32);
    fn stop_sample(&mut self, handle: u32);
    fn end_sample(&mut self, handle: u32);
    fn clear_sound_slot_tables(&mut self);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FadeError {
    /// El original llega a DIV con divisor cero (volumen fuera de dominio).
    ZeroInterval,
}

#[derive(Clone)]
pub struct SoundManager {
    image: [u8; IMAGE_SIZE],
}

impl SoundManager {
    /// Estado suministrado por el adaptador. Los handles deben pertenecer al backend.
    pub fn from_image(image: [u8; IMAGE_SIZE]) -> Self {
        Self { image }
    }

    pub fn image(&self) -> &[u8; IMAGE_SIZE] {
        &self.image
    }

    fn word(&self, offset: usize) -> u32 {
        u32::from_le_bytes(self.image[offset..offset + 4].try_into().unwrap())
    }

    fn put(&mut self, offset: usize, value: u32) {
        self.image[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
    }

    fn enabled(&self) -> bool {
        self.word(DRIVER) != 0
    }

    fn close(&mut self, backend: &mut impl Backend, offset: usize) {
        let handle = self.word(offset);
        backend.pause_stream(handle, 1);
        backend.close_stream(handle);
        self.put(offset, 0);
    }

    /// 0x00442130. Sin dispositivo devuelve cero y conserva el contador almacenado.
    pub fn count_playing_samples(&mut self, backend: &mut impl Backend) -> u16 {
        if !self.enabled() {
            return 0;
        }
        let mut count = 0u16;
        self.image[0x1b4..0x1b6].copy_from_slice(&count.to_le_bytes());
        for i in 0..SAMPLES {
            if backend.sample_status(self.word(0x18 + 4 * i)) == 4 {
                count += 1;
                self.image[0x1b4..0x1b6].copy_from_slice(&count.to_le_bytes());
            }
        }
        count
    }

    /// 0x00442180. Solo música actual, ambiente actual y edificio, en ese orden.
    pub fn close_finished_streams(&mut self, backend: &mut impl Backend) {
        if self.enabled() {
            for offset in [MUSIC, AMBIENT, BUILDING] {
                let handle = self.word(offset);
                if handle != 0 && backend.stream_status(handle) == 2 {
                    self.close(backend, offset);
                }
            }
        }
    }

    /// 0x00442210. Puede devolver true sin dispositivo si hay un handle residual.
    pub fn is_ambient_stream_active(&mut self, backend: &mut impl Backend) -> bool {
        self.close_finished_streams(backend);
        self.word(AMBIENT) != 0
    }

    /// 0x00442230.
    pub fn is_music_stream_active(&mut self, backend: &mut impl Backend) -> bool {
        self.close_finished_streams(backend);
        self.word(MUSIC) != 0
    }

    /// 0x00442D10. Conserva los canales de edificio y voz, y la bandera de ambiente.
    pub fn stop_all_streams(&mut self, backend: &mut impl Backend) {
        if self.enabled() {
            for offset in [MUSIC, AMBIENT, MUSIC_OUT, AMBIENT_OUT] {
                if self.word(offset) != 0 {
                    self.close(backend, offset);
                }
            }
            self.image[MUSIC_FADE] = 0;
        }
    }

    fn scaled_volume(&self, target: usize, volume: usize) -> i32 {
        (self.image[target] as i32).wrapping_mul(self.word(volume) as i32) / 100
    }

    /// 0x00442D90. Compara el valor sin saturar antes de enviar el volumen al backend.
    pub fn set_music_volume(&mut self, backend: &mut impl Backend, volume: i32) {
        if !self.enabled() || self.word(MUSIC_VOLUME) as i32 == volume {
            return;
        }
        let (handle, target) = if self.word(MUSIC) != 0 {
            (self.word(MUSIC), 0x1da)
        } else {
            (self.word(MUSIC_OUT), 0x1db)
        };
        if handle != 0 {
            let scaled = (self.image[target] as i32).wrapping_mul(volume) / 100;
            if backend.stream_volume(handle) != scaled {
                backend.set_stream_volume(handle, scaled.clamp(0, 127));
            }
        }
        self.put(MUSIC_VOLUME, volume as u32);
    }

    /// 0x00442E30. Guarda incluso valores fuera de 0..127; satura solo la llamada.
    pub fn set_voice_volume(&mut self, backend: &mut impl Backend, volume: i32) {
        if self.enabled() {
            self.put(0x1c0, volume as u32);
            let handle = self.word(VOICE);
            if handle != 0 && backend.stream_volume(handle) != volume {
                backend.set_stream_volume(handle, volume.clamp(0, 127));
            }
        }
    }

    fn sample_slot(&self, handle: u32) -> Option<usize> {
        (0..SAMPLES).find(|i| self.word(0x18 + 4 * i) == handle)
    }

    /// 0x004430D0. Solo modifica la primera coincidencia, también para handle cero.
    pub fn clear_sample_slot_flags(&mut self, handle: u32) {
        if self.enabled() {
            if let Some(i) = self.sample_slot(handle) {
                self.image[0x18c + i] = 0;
                self.image[0x1a0 + i] = 0;
            }
        }
    }

    /// 0x00443110. El primer ID 0x1bbc decide el resultado aunque haya duplicados.
    pub fn is_sound_1bbc_playing(&self, backend: &mut impl Backend) -> bool {
        self.enabled()
            && (0..SAMPLES)
                .find(|i| self.word(0x7c + 4 * i) == 0x1bbc)
                .is_some_and(|i| backend.sample_status(self.word(0x18 + 4 * i)) == 4)
    }

    /// 0x004438D0. Inversión seguida de truncado a u16, sin saturación a 0..127.
    pub fn set_sample_pan(&self, backend: &mut impl Backend, handle: u32, pan: i32) {
        if self.enabled() && handle != 0 && self.sample_slot(handle).is_some() {
            let pan = if self.image[0x1dd] != 0 {
                127i32.wrapping_sub(pan)
            } else {
                pan
            } as u16;
            if backend.sample_pan(handle) != pan as i32 {
                backend.set_sample_pan(handle, pan);
            }
        }
    }

    /// 0x00443940. El adaptador indica si existe la tabla auxiliar de efectos.
    pub fn stop_all_samples(&mut self, backend: &mut impl Backend, has_effect_list: bool) {
        if self.enabled() {
            if has_effect_list {
                backend.clear_sound_slot_tables();
            }
            for i in 0..SAMPLES {
                let handle = self.word(0x18 + 4 * i);
                if backend.sample_status(handle) == 4 {
                    backend.stop_sample(handle);
                    backend.end_sample(handle);
                }
                self.image[0x18c + i] = 0;
                self.image[0x1a0 + i] = 0;
            }
        }
    }

    fn fade_steps(target: i32, now: u32, start: u32) -> Result<u32, FadeError> {
        let interval = if target == 0 { 5000 } else { 5000 / target } as u32;
        if interval == 0 {
            return Err(FadeError::ZeroInterval);
        }
        Ok(now.wrapping_sub(start) / interval)
    }

    fn update_pair(&mut self, backend: &mut impl Backend, now: u32, music: bool) -> Result<(), FadeError> {
        let (current, outgoing, flag, target, start, volume) = if music {
            (MUSIC, MUSIC_OUT, MUSIC_FADE, 0x1da, 0x1d0, MUSIC_VOLUME)
        } else {
            (AMBIENT, AMBIENT_OUT, AMBIENT_FADE, 0x1d8, 0x1c8, EFFECTS_VOLUME)
        };
        if self.image[flag] == 0 {
            return Ok(());
        }
        self.image[flag] = 0;
        let handle = self.word(current);
        if handle != 0 {
            let level = self.scaled_volume(target, volume).min(127);
            self.image[flag] = 1;
            let steps = Self::fade_steps(level, now, self.word(start))?;
            if steps != 0 {
                let mut next = (steps as i32).wrapping_add(backend.stream_volume(handle));
                if next > level {
                    next = level;
                    self.image[flag] = 0;
                }
                backend.set_stream_volume(handle, next);
                self.put(start, now);
            }
        }
        let handle = self.word(outgoing);
        if handle != 0 {
            let level = self.scaled_volume(target + 1, volume).min(127);
            self.image[flag] = 1;
            let steps = Self::fade_steps(level, now, self.word(start + 4))?;
            if steps != 0 {
                let next = backend.stream_volume(handle).wrapping_sub(steps as i32);
                if music && self.image[0x1dc] == 0 && next <= self.image[0x1c4] as i32 {
                    let floor = self.image[0x1c4] as i32;
                    self.image[flag] = 0;
                    if backend.stream_volume(handle) > floor {
                        backend.set_stream_volume(handle, floor);
                    }
                } else if next <= 0 {
                    // La bandera se limpia antes de las llamadas solo para música.
                    if music {
                        self.image[flag] = 0;
                    }
                    self.close(backend, outgoing);
                    self.image[flag] = 0;
                } else {
                    backend.set_stream_volume(handle, next);
                    self.put(start + 4, now);
                }
            }
        }
        Ok(())
    }

    /// 0x00444030. Reloj de presentación explícito, con resta u32 que admite vuelta.
    /// A diferencia de otros métodos, el original no comprueba aquí el dispositivo.
    pub fn update_fades(&mut self, backend: &mut impl Backend, now: u32) -> Result<(), FadeError> {
        self.update_pair(backend, now, false)?;
        self.update_pair(backend, now, true)?;
        let handle = self.word(BUILDING);
        if self.image[0x1de] != 0 && handle != 0 && backend.stream_status(handle) == 2 {
            self.close(backend, BUILDING);
        }
        Ok(())
    }

    /// 0x00444370. No comprueba el dispositivo; sustituye el canal saliente anterior.
    pub fn fade_out_music(&mut self, backend: &mut impl Backend, now: u32, mode: u8) {
        self.fade_out(backend, now, true, mode);
    }

    /// 0x004443D0.
    pub fn fade_out_ambient(&mut self, backend: &mut impl Backend, now: u32) {
        self.fade_out(backend, now, false, 0);
    }

    fn fade_out(&mut self, backend: &mut impl Backend, now: u32, music: bool, mode: u8) {
        let (current, outgoing, flag, target, start) = if music {
            (MUSIC, MUSIC_OUT, MUSIC_FADE, 0x1da, 0x1d4)
        } else {
            (AMBIENT, AMBIENT_OUT, AMBIENT_FADE, 0x1d8, 0x1cc)
        };
        if self.word(current) != 0 {
            if self.word(outgoing) != 0 {
                self.close(backend, outgoing);
            }
            self.put(outgoing, self.word(current));
            self.put(current, 0);
            self.put(start, now);
            self.image[flag] = 1;
            self.image[target + 1] = self.image[target];
            if music {
                self.image[0x1dc] = mode;
            }
        }
    }
}
