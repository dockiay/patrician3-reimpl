//! Voz, música de edificio y reanudación del fondo; sin salida sonora.
use crate::filter;
use crate::streams::StreamBackend;
use crate::{SoundManager, AMBIENT, AMBIENT_OUT, BUILDING, EFFECTS_VOLUME, MUSIC, MUSIC_FADE, MUSIC_OUT, VOICE};

#[derive(Debug, Clone, Copy)]
pub struct BuildingMusic<'a> {
    /// 0xffff selecciona file directamente; cualquier otro valor indexa titles.
    pub index: u16,
    pub file: &'a str,
    pub flag: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VoiceError {
    MissingTitle { index: u16 },
    MissingFilterKey,
}

impl SoundManager {
    /// 0x00442280. La lista de títulos sustituye el puntero de la imagen +0xCC.
    pub fn play_building_music(
        &mut self,
        backend: &mut impl StreamBackend,
        titles: &[&str],
        now: u32,
        request: BuildingMusic<'_>,
    ) -> Result<(), VoiceError> {
        if !self.enabled() {
            return Ok(());
        }
        self.close_finished_streams(backend);
        if self.word(BUILDING) != 0 && request.index != u16::MAX {
            return Ok(());
        }
        if request.index == u16::MAX {
            // A diferencia de stop_building_music, el original llama incluso con 0.
            self.close(backend, BUILDING);
        }
        if self.word(EFFECTS_VOLUME) == 0 || (self.image[0x1de] == 0 && request.flag & 1 != 0) {
            return Ok(());
        }
        let file = if request.index == u16::MAX {
            request.file
        } else {
            titles
                .get(request.index as usize)
                .copied()
                .ok_or(VoiceError::MissingTitle { index: request.index })?
        };
        let stream = backend.open_stream(self.word(0x68), file, 0x3e800);
        self.put(BUILDING, stream);
        if stream == 0 {
            return Ok(());
        }
        backend.set_stream_volume(stream, (self.word(EFFECTS_VOLUME) as i32).clamp(0, 127));
        if self.image[0x1de] != 0 {
            let provider = self.word(0x6c);
            let key = filter::last_key(backend, provider);
            for offset in [AMBIENT, AMBIENT_OUT] {
                let ambient = self.word(offset);
                if ambient != 0 {
                    // Sin ambientes no se lee la clave, aunque la enumeración sea vacía.
                    let key = key.map_err(|_| VoiceError::MissingFilterKey)?;
                    let sample = backend.stream_sample(ambient);
                    backend.set_sample_processor(sample, 1, provider);
                    let sample = backend.stream_sample(ambient);
                    backend.set_filter_preference(sample, key, 400f32.to_bits());
                }
            }
        }
        self.fade_out_music(backend, now, 0);
        self.image[0x1e0] = 1;
        backend.start_stream(stream);
        if self.image[0x1de] == 0 {
            backend.set_stream_loop_count(stream, 0);
        }
        Ok(())
    }

    /// 0x00442460. El volumen es i8, no u8. Un segundo inicio no cierra VOICE.
    pub fn play_speech_stream(
        &mut self,
        backend: &mut impl StreamBackend,
        file: &str,
        volume: i8,
        has_effect_list: bool,
    ) {
        if !self.enabled() || self.word(0x1c0) == 0 {
            return;
        }
        let scaled = (volume as i32).wrapping_mul(self.word(0x1c0) as i32) / 100;
        self.stop_all_samples(backend, has_effect_list);
        if self.word(MUSIC_OUT) != 0 {
            self.close(backend, MUSIC_OUT);
        }
        for offset in [MUSIC, AMBIENT] {
            let stream = self.word(offset);
            if stream != 0 {
                backend.pause_stream(stream, 1);
            }
        }
        if self.word(AMBIENT_OUT) != 0 {
            self.close(backend, AMBIENT_OUT);
        }
        let stream = backend.open_stream(self.word(0x68), file, 0x3e800);
        self.put(VOICE, stream);
        if stream != 0 {
            backend.set_stream_volume(stream, scaled.clamp(0, 127));
            backend.start_stream(stream);
            self.image[0x1df] = 1;
        }
    }

    /// 0x004425D0. Reanuda solo el estado 8; la bandera de voz se borra con driver.
    pub fn end_speech_and_resume(&mut self, backend: &mut impl StreamBackend, now: u32) {
        if !self.enabled() {
            return;
        }
        if self.word(VOICE) != 0 {
            self.close(backend, VOICE);
        }
        let music = self.word(MUSIC);
        if music != 0 && backend.stream_status(music) == 8 {
            backend.set_stream_volume(music, 0);
            backend.pause_stream(music, 0);
            // Sin mutación/reentrada el handle sigue siendo no nulo: no hay restauración.
            self.image[MUSIC_FADE] = 1;
            self.put(0x1d0, now);
        }
        let ambient = self.word(AMBIENT);
        if ambient != 0 && backend.stream_status(ambient) == 8 {
            backend.pause_stream(ambient, 0);
        }
        self.image[0x1df] = 0;
    }

    /// 0x00442690. Una voz ausente o cuyo estado no sea 2 no dispara la reanudación.
    pub fn update_speech(&mut self, backend: &mut impl StreamBackend, now: u32) {
        if self.enabled() {
            let voice = self.word(VOICE);
            if voice != 0 && backend.stream_status(voice) == 2 {
                self.end_speech_and_resume(backend, now);
            }
        }
    }

    /// 0x004438A0. No limpia la bandera +0x1E0 ni inicia otros canales.
    pub fn stop_building_music(&mut self, backend: &mut impl StreamBackend) {
        if self.enabled() && self.word(BUILDING) != 0 {
            self.close(backend, BUILDING);
        }
    }
}
