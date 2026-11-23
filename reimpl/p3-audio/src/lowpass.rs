//! Filtro de pantallas y transición entre ambiente, música y edificio.
use crate::filter;
use crate::streams::StreamBackend;
use crate::voice::{BuildingMusic, VoiceError};
use crate::{SoundManager, AMBIENT, AMBIENT_OUT, MUSIC, MUSIC_FADE, MUSIC_OUT};

impl SoundManager {
    /// 0x004436E0. Conserva el byte enable, no lo normaliza a bool. No crea hilos.
    pub fn set_lowpass_filter(
        &mut self,
        backend: &mut impl StreamBackend,
        titles: &[&str],
        now: u32,
        enable: u8,
        building: u8,
        has_effect_list: bool,
    ) -> Result<(), VoiceError> {
        if !self.enabled() {
            return Ok(());
        }
        self.image[0x1c5] = enable;
        if enable != 0 {
            let provider = self.word(0x6c);
            let key = filter::last_key(backend, provider);
            for offset in [AMBIENT, AMBIENT_OUT] {
                let ambient = self.word(offset);
                if ambient != 0 {
                    let key = key.map_err(|_| VoiceError::MissingFilterKey)?;
                    let sample = backend.stream_sample(ambient);
                    backend.set_sample_processor(sample, 1, provider);
                    let sample = backend.stream_sample(ambient);
                    backend.set_filter_preference(sample, key, 400f32.to_bits());
                }
            }
            self.stop_all_samples(backend, has_effect_list);
            // El índice se amplía desde u8: nunca es 0xffff. El NULL del fichero
            // original no se consume en esta ruta; se usa exclusivamente titles.
            self.play_building_music(
                backend,
                titles,
                now,
                BuildingMusic {
                    index: building as u16,
                    file: "",
                    flag: 0,
                },
            )?;
        } else {
            for offset in [AMBIENT, AMBIENT_OUT] {
                let ambient = self.word(offset);
                if ambient != 0 {
                    let sample = backend.stream_sample(ambient);
                    backend.set_sample_processor(sample, 1, 0);
                }
            }
            if self.word(MUSIC) == 0 && self.word(MUSIC_OUT) != 0 {
                self.put(MUSIC, self.word(MUSIC_OUT));
                self.image[0x1da] = self.image[0x1db];
                self.put(MUSIC_OUT, 0);
            }
            if self.word(MUSIC) != 0 {
                self.image[MUSIC_FADE] = 1;
                self.put(0x1d0, now);
            }
            if self.image[0x1de] == 0 {
                self.stop_building_music(backend);
            }
            self.image[0x1df] = 0;
            self.image[0x1e0] = 0;
        }
        Ok(())
    }
}
