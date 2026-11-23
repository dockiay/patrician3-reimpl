//! Música, ambiente y consultas de duración, sin dispositivo ni hilo propietario.
use crate::filter::{self, FilterBackend, FilterError};
use crate::{Backend, SoundManager, AMBIENT, AMBIENT_OUT, EFFECTS_VOLUME, MUSIC, MUSIC_FADE, MUSIC_VOLUME, SAMPLES};

pub trait StreamBackend: Backend + FilterBackend {
    fn open_stream(&mut self, driver: u32, name: &str, buffer_size: u32) -> u32;
    fn set_stream_ms_position(&mut self, handle: u32, position: u32);
    /// Devuelve (total, actual); want_position representa el puntero actual opcional.
    fn stream_ms_position(&mut self, handle: u32, want_position: bool) -> (u32, u32);
    fn start_stream(&mut self, handle: u32);
    fn set_stream_loop_count(&mut self, handle: u32, count: u32);
    /// Relación estructural que el original lee en HSTREAM+0xC. No es un puntero Rust.
    fn stream_sample(&mut self, handle: u32) -> u32;
}

#[derive(Debug, Clone, Copy)]
pub struct PlayMusic {
    pub volume: u8,
    pub replace: u8,
    pub looping: u8,
    pub force: u8,
}

impl SoundManager {
    /// 0x004426C0. La sustitución que abre a la primera devuelve cero aunque inicie.
    pub fn play_music(&mut self, backend: &mut impl StreamBackend, name: &str, now: u32, request: PlayMusic) -> u32 {
        if !self.enabled()
            || (request.force == 0 && (self.word(MUSIC_VOLUME) as i32).wrapping_mul(request.volume as i32) / 100 == 0)
            || self.word(0x120) != 0
            || self.image[0x1e0] != 0
        {
            return 0;
        }
        self.close_finished_streams(backend);
        if request.replace != 0 {
            for i in 0..SAMPLES {
                backend.end_sample(self.word(0x18 + 4 * i));
            }
            self.fade_out_music(backend, now, 1);
            let stream = backend.open_stream(self.word(0x68), name, 0x3e800);
            self.put(MUSIC, stream);
            if stream != 0 {
                self.image[0x1da] = request.volume;
                let scaled = (self.word(MUSIC_VOLUME) as i32).wrapping_mul(request.volume as i32) / 100;
                backend.set_stream_volume(stream, scaled.clamp(0, 127));
                backend.start_stream(stream);
                backend.stream_ms_position(stream, false);
                if request.looping != 0 {
                    backend.set_stream_loop_count(stream, 0);
                }
                for offset in [AMBIENT, AMBIENT_OUT] {
                    if self.word(offset) != 0 {
                        self.close(backend, offset);
                    }
                }
            }
        }
        if self.word(MUSIC) != 0 {
            return 0;
        }
        // También se ejecuta si falla la primera apertura de una sustitución.
        self.fade_out_ambient(backend, now);
        let stream = backend.open_stream(self.word(0x68), name, 0x3e800);
        self.put(MUSIC, stream);
        if stream == 0 {
            return 0;
        }
        backend.set_stream_volume(stream, 0);
        self.image[0x1da] = request.volume;
        // Con un backend sin reentrada el stream recién abierto sigue en MUSIC;
        // la rama original que intentaría restaurar MUSIC_OUT no puede tomarse.
        self.image[MUSIC_FADE] = 1;
        self.put(0x1d0, now);
        backend.start_stream(stream);
        if request.looping != 0 {
            backend.set_stream_loop_count(stream, 0);
        }
        backend.stream_ms_position(stream, false).0
    }

    /// 0x00442A10. La resta total-inicio usa el inicio pedido, incluso si el backend
    /// ajusta su posición efectiva. Un fallo de apertura conserva la transición saliente.
    pub fn play_ambient_stream(
        &mut self,
        backend: &mut impl StreamBackend,
        name: &str,
        now: u32,
        start_ms: u32,
        volume: u8,
    ) -> Result<u32, FilterError> {
        if !self.enabled() || self.image[0x1e0] != 0 {
            return Ok(0);
        }
        self.close_finished_streams(backend);
        if self.word(EFFECTS_VOLUME) == 0 {
            return Ok(0);
        }
        self.fade_out_ambient(backend, now);
        let stream = backend.open_stream(self.word(0x68), name, 0x3e800);
        self.put(AMBIENT, stream);
        if stream == 0 {
            return Ok(0);
        }
        backend.set_stream_ms_position(stream, start_ms);
        let remaining = backend.stream_ms_position(stream, false).0.wrapping_sub(start_ms);
        self.fade_out_music(backend, now, 1);
        backend.set_stream_volume(stream, 0);
        self.image[0x1d8] = volume;
        self.image[0x1c6] = 1;
        self.put(0x1c8, now);
        backend.start_stream(stream);
        backend.set_stream_loop_count(stream, 0);
        if self.image[0x1c5] != 0 {
            let provider = self.word(0x6c);
            let key = filter::last_key(backend, provider)?;
            let sample = backend.stream_sample(stream);
            backend.set_sample_processor(sample, 1, provider);
            let sample = backend.stream_sample(stream);
            backend.set_filter_preference(sample, key, 400f32.to_bits());
        }
        Ok(remaining)
    }

    /// 0x00442C20. u32::MAX es el patrón -1 original cuando no hay dispositivo.
    pub fn get_music_remaining_ms(&self, backend: &mut impl StreamBackend) -> u32 {
        if !self.enabled() {
            return u32::MAX;
        }
        let stream = self.word(MUSIC);
        if stream == 0 {
            return 0;
        }
        let (total, current) = backend.stream_ms_position(stream, true);
        total.wrapping_sub(current)
    }

    /// 0x00442C60. Consulta un stream temporal, lo pausa y lo cierra sin almacenarlo.
    pub fn get_file_duration_ms(&self, backend: &mut impl StreamBackend, name: &str) -> u32 {
        if !self.enabled() {
            return 0;
        }
        let stream = backend.open_stream(self.word(0x68), name, 0x3e800);
        if stream == 0 {
            return 0;
        }
        let total = backend.stream_ms_position(stream, false).0;
        backend.pause_stream(stream, 1);
        backend.close_stream(stream);
        total
    }
}
