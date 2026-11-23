//! Selección regional y temporizadores de GameScreen; sin ventanas ni RNG propio.
use crate::streams::{PlayMusic, StreamBackend};
use crate::{FadeError, SoundManager};

pub mod catalog;

pub const SCREEN_AUDIO_SIZE: usize = 0x50;
const BASE: usize = 0x287c;

#[derive(Clone, Copy, Debug)]
pub struct Track<'a> {
    pub file: &'a str,
    pub volume: u8,
}

pub struct Catalog<'a> {
    pub region: &'a [Track<'a>],
    pub sea: &'a [Track<'a>],
    pub town_atmosphere: [Track<'a>; 3],
    pub sea_atmosphere: [Track<'a>; 2],
    pub battle: Track<'a>,
    pub town: &'a [Track<'a>],
}

/// Valores estables durante una llamada. regions usa índice mundial, mientras
/// que town_ids traduce el índice de ciudad del argumento de start_town_music.
pub struct Scene<'a> {
    pub now: u32,
    pub tick: u32,
    pub music_enabled: u8,
    pub nav_screen: u32,
    pub battle_screen: u32,
    pub viewed_town: u32,
    pub towns_count: u32,
    pub regions: &'a [u8],
    pub town_ids: &'a [u8],
    pub phase: u32,
    pub flags_a: u32,
    pub flags_b: u32,
    pub battle: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreenError {
    MissingTrack { index: u32 },
    MissingTown { index: u32 },
    RandomUnavailable,
    InvalidRandom(u16),
    SeekLimit { draws: u32 },
    MissingFilterKey,
    Fade(FadeError),
}

#[derive(Debug, Clone, Copy)]
pub enum Window {
    Navigation,
    Town,
}

/// El cliente debe usar el flujo CRT compartido, no sembrar otro generador para
/// audio. Las consultas conservan su orden y pueden devolver valores sucesivos.
pub trait ScreenHost {
    fn random(&mut self) -> Option<u16>;
    fn top_window(&mut self) -> u32;
    fn visible(&mut self, window: Window) -> u8;
    fn weather_flag1(&mut self) -> u8;
    fn month(&mut self, tick: u32) -> u8;
}

pub trait ScreenServices: ScreenHost {
    fn flags(&self) -> [u8; 3]; // voz, fundido música, fundido ambiente
    fn update_speech(&mut self);
    fn update_fades(&mut self) -> Result<(), ScreenError>;
    fn stop_samples(&mut self);
    fn music_active(&mut self) -> bool;
    fn music_remaining(&mut self) -> u32;
    fn duration(&mut self, file: &str) -> u32;
    fn music(&mut self, track: Track<'_>, replace: u8) -> u32;
    fn ambient(&mut self, track: Track<'_>, position: u32) -> Result<u32, ScreenError>;
}

/// Conexión a los controles ya portados, con reloj fijo y servicios explícitos.
/// now debe coincidir con Scene::now durante toda la llamada.
pub struct NativeScreenServices<'a, B, H> {
    pub manager: &'a mut SoundManager,
    pub backend: &'a mut B,
    pub host: &'a mut H,
    pub now: u32,
    pub has_effect_list: bool,
}

impl<B, H: ScreenHost> ScreenHost for NativeScreenServices<'_, B, H> {
    fn random(&mut self) -> Option<u16> {
        self.host.random()
    }
    fn top_window(&mut self) -> u32 {
        self.host.top_window()
    }
    fn visible(&mut self, w: Window) -> u8 {
        self.host.visible(w)
    }
    fn weather_flag1(&mut self) -> u8 {
        self.host.weather_flag1()
    }
    fn month(&mut self, tick: u32) -> u8 {
        self.host.month(tick)
    }
}

impl<B: StreamBackend, H: ScreenHost> ScreenServices for NativeScreenServices<'_, B, H> {
    fn flags(&self) -> [u8; 3] {
        [
            self.manager.image()[0x1df],
            self.manager.image()[0x1c7],
            self.manager.image()[0x1c6],
        ]
    }
    fn update_speech(&mut self) {
        self.manager.update_speech(self.backend, self.now);
    }
    fn update_fades(&mut self) -> Result<(), ScreenError> {
        self.manager
            .update_fades(self.backend, self.now)
            .map_err(ScreenError::Fade)
    }
    fn stop_samples(&mut self) {
        self.manager.stop_all_samples(self.backend, self.has_effect_list);
    }
    fn music_active(&mut self) -> bool {
        self.manager.is_music_stream_active(self.backend)
    }
    fn music_remaining(&mut self) -> u32 {
        self.manager.get_music_remaining_ms(self.backend)
    }
    fn duration(&mut self, file: &str) -> u32 {
        self.manager.get_file_duration_ms(self.backend, file)
    }
    fn music(&mut self, track: Track<'_>, replace: u8) -> u32 {
        self.manager.play_music(
            self.backend,
            track.file,
            self.now,
            PlayMusic {
                volume: track.volume,
                replace,
                looping: 0,
                force: 0,
            },
        )
    }
    fn ambient(&mut self, track: Track<'_>, position: u32) -> Result<u32, ScreenError> {
        self.manager
            .play_ambient_stream(self.backend, track.file, self.now, position, track.volume)
            .map_err(|_| ScreenError::MissingFilterKey)
    }
}

pub struct ScreenAudio {
    image: [u8; SCREEN_AUDIO_SIZE],
}

impl ScreenAudio {
    pub fn from_image(image: [u8; SCREEN_AUDIO_SIZE]) -> Self {
        Self { image }
    }
    pub fn image(&self) -> &[u8; SCREEN_AUDIO_SIZE] {
        &self.image
    }
    fn byte(&self, offset: usize) -> u8 {
        self.image[offset - BASE]
    }
    fn set_byte(&mut self, offset: usize, value: u8) {
        self.image[offset - BASE] = value;
    }
    fn word(&self, offset: usize) -> u32 {
        u32::from_le_bytes(self.image[offset - BASE..offset - BASE + 4].try_into().unwrap())
    }
    fn put(&mut self, offset: usize, value: u32) {
        self.image[offset - BASE..offset - BASE + 4].copy_from_slice(&value.to_le_bytes());
    }
    fn random(host: &mut impl ScreenHost) -> Result<f64, ScreenError> {
        let r = host.random().ok_or(ScreenError::RandomUnavailable)?;
        if r > 32767 {
            return Err(ScreenError::InvalidRandom(r));
        }
        // El cociente se almacena como f32; es exacto para rand/32768.
        Ok((r as f32 / 32768.0) as f64)
    }
    fn scale(value: u32, ratio: f64) -> u32 {
        ((value as i32 as f32 as f64 * ratio).trunc() as i64) as u32
    }
    fn delay(ratio: f64, scale: f64) -> u32 {
        (ratio * scale).trunc() as i64 as u32
    }
    fn track<'a>(tracks: &'a [Track<'a>], index: u32) -> Result<Track<'a>, ScreenError> {
        tracks
            .get(index as usize)
            .copied()
            .ok_or(ScreenError::MissingTrack { index })
    }
    fn bank_index(&self, region: usize, ratio: f64) -> u32 {
        let prefix: u32 = (0..region).map(|r| self.byte(0x2889 + r) as u32).sum();
        prefix + Self::scale(self.byte(0x2889 + region) as u32, ratio)
    }
    fn sea_music(host: &mut impl ScreenHost, scene: &Scene<'_>) -> bool {
        let nav = host.top_window() == scene.nav_screen;
        let battle = host.top_window() == scene.battle_screen;
        let visible = host.visible(Window::Navigation) != 0;
        nav | battle | visible
    }
    fn music_deadlines(&mut self, now: u32, duration: u32, delay: u32, base: u32) {
        self.put(0x28c4, now.wrapping_add(duration).wrapping_sub(5000));
        self.put(
            0x2884,
            now.wrapping_sub(delay).wrapping_add(duration).wrapping_add(base),
        );
    }
    fn schedule_existing(&mut self, host: &mut impl ScreenServices, scene: &Scene<'_>) -> Result<(), ScreenError> {
        let ratio = Self::random(host)?;
        let remaining = host.music_remaining();
        self.put(0x28c4, scene.now.wrapping_add(remaining).wrapping_sub(5000));
        let remaining = host.music_remaining();
        self.put(
            0x2884,
            scene
                .now
                .wrapping_sub(Self::delay(ratio, -10000.0))
                .wrapping_add(remaining)
                .wrapping_add(10000),
        );
        Ok(())
    }
    /// 0x0042C080. Comparaciones de plazos u32, duración restante i32.
    pub fn update(
        &mut self,
        host: &mut impl ScreenServices,
        scene: &Scene<'_>,
        bank: &Catalog<'_>,
    ) -> Result<(), ScreenError> {
        let [voice, music_fade, ambient_fade] = host.flags();
        if voice != 0 {
            host.update_speech();
            return Ok(());
        }
        if music_fade | ambient_fade != 0 {
            host.update_fades()?;
        }
        if scene.music_enabled != 0 && self.word(0x2884) < scene.now {
            let ratio = Self::random_after_mode(host, scene)?;
            let (sea, ratio) = ratio;
            let index = if sea {
                Self::scale(self.byte(0x28c8) as u32, ratio)
            } else if scene.viewed_town >= scene.towns_count {
                self.bank_index(5, ratio)
            } else {
                let region = *scene
                    .regions
                    .get(scene.viewed_town as usize)
                    .ok_or(ScreenError::MissingTown {
                        index: scene.viewed_town,
                    })?;
                if region <= 4 {
                    self.bank_index(region as usize, ratio)
                } else {
                    0
                }
            };
            if index != self.byte(0x2888) as u32 {
                let sea = Self::sea_music(host, scene);
                let track = Self::track(if sea { bank.sea } else { bank.region }, index)?;
                let duration = host.music(track, 0);
                let delay = Self::delay(Self::random(host)?, -15000.0);
                self.music_deadlines(scene.now, duration, delay, 40000);
                self.set_byte(0x2888, index as u8);
            } else {
                self.put(0x2884, scene.now.wrapping_add(20000));
                self.put(0x28c4, scene.now.wrapping_sub(1));
            }
        }
        if self.word(0x28c4) >= scene.now {
            return Ok(());
        }
        let remaining = host.music_remaining();
        if remaining as i32 > 5000 {
            self.put(0x28c4, self.word(0x28c4).wrapping_add(remaining).wrapping_sub(5000));
            return Ok(());
        }
        let mut weather = false;
        let night;
        let town = host.visible(Window::Town) != 0;
        let track = if town {
            night = scene.phase > 1 && scene.phase < 6;
            weather = host.weather_flag1() != 0;
            if scene.battle != 255 {
                Some(Track {
                    volume: 100,
                    ..bank.battle
                })
            } else if weather {
                Some(bank.town_atmosphere[2])
            } else if night {
                Some(bank.town_atmosphere[1])
            } else if (scene.flags_a | scene.flags_b) & 0xff000000 == 0 {
                Some(bank.town_atmosphere[0])
            } else {
                None
            }
        } else {
            let month = host.month(scene.tick);
            night = !(1..=10).contains(&month);
            Some(bank.sea_atmosphere[usize::from(night)])
        };
        let duration = track.map_or(0, |t| host.duration(t.file));
        self.put(0x28b4, duration);
        self.put(0x28b8, duration);
        if duration == 0 {
            return Ok(());
        }
        let position = if scene.music_enabled != 0 {
            Self::scale(duration, Self::random(host)?)
        } else {
            0
        };
        self.put(0x28b8, position);
        // El original vuelve a consultar visibilidad al reproducir.
        let track = if host.visible(Window::Town) != 0 {
            if scene.battle != 255 {
                Track {
                    volume: 100,
                    ..bank.battle
                }
            } else {
                bank.town_atmosphere[if weather { 2 } else { usize::from(night) }]
            }
        } else {
            bank.sea_atmosphere[usize::from(night)]
        };
        let remaining = host.ambient(track, position)?;
        self.put(0x28c4, scene.now.wrapping_add(remaining).wrapping_sub(5000));
        Ok(())
    }
    fn random_after_mode(host: &mut impl ScreenServices, scene: &Scene<'_>) -> Result<(bool, f64), ScreenError> {
        let sea = Self::sea_music(host, scene);
        Ok((sea, Self::random(host)?))
    }
    fn seek(&mut self, host: &mut impl ScreenServices, scene: &Scene<'_>, limit: u32) -> Result<(), ScreenError> {
        let needed = self.word(0x2884).wrapping_sub(scene.now);
        if needed == 0 {
            return Ok(());
        }
        for _ in 0..limit {
            let position = Self::scale(self.word(0x28b4), Self::random(host)?);
            self.put(0x28b8, position);
            if self.word(0x28b4).wrapping_sub(position) >= needed {
                return Ok(());
            }
        }
        Err(ScreenError::SeekLimit { draws: limit })
    }
    /// 0x0042C770. seek_limit limita explícitamente el bucle original, que puede
    /// no terminar. Agotarlo es un límite seguro, no un retorno del x86 original.
    pub fn start_atmosphere(
        &mut self,
        host: &mut impl ScreenServices,
        scene: &Scene<'_>,
        bank: &Catalog<'_>,
        request: Atmosphere,
    ) -> Result<(), ScreenError> {
        if request.stop != 0 {
            host.stop_samples();
        }
        match request.mode {
            2 => {
                let ratio = Self::random(host)?;
                let index = self.bank_index(5, ratio);
                let duration = host.music(Self::track(bank.region, index)?, 1);
                self.music_deadlines(scene.now, duration, Self::delay(ratio, -5000.0), 15000);
                self.set_byte(0x2888, index as u8);
            }
            1 => {
                let month = host.month(scene.tick);
                let winter = !(1..=10).contains(&month);
                let track = bank.sea_atmosphere[usize::from(winter)];
                let duration = host.duration(track.file);
                self.put(0x28b4, duration);
                let active = host.music_active();
                if !active || scene.towns_count < scene.viewed_town || request.force != 0 {
                    let delay = Self::delay(Self::random(host)?, -5000.0);
                    self.put(0x2884, scene.now.wrapping_sub(delay).wrapping_add(15000));
                    self.put(0x28b8, duration);
                    if duration == 0 {
                        return Ok(());
                    }
                    self.seek(host, scene, request.seek_limit)?;
                    let remaining = host.ambient(track, self.word(0x28b8))?;
                    self.put(0x28c4, scene.now.wrapping_add(remaining));
                } else {
                    self.schedule_existing(host, scene)?;
                }
            }
            0 => {
                if host.music_active() && request.force == 0 {
                    return self.schedule_existing(host, scene);
                }
                let delay = Self::delay(Self::random(host)?, -5000.0);
                self.put(0x2884, scene.now.wrapping_sub(delay).wrapping_add(15000));
                let track = if host.weather_flag1() != 0 {
                    Some(bank.town_atmosphere[2])
                } else if scene.phase > 1 && scene.phase < 6 {
                    Some(bank.town_atmosphere[1])
                } else if (scene.flags_a | scene.flags_b) & 0xff000000 == 0 {
                    Some(bank.town_atmosphere[0])
                } else {
                    None
                };
                let duration = track.map_or(0, |t| host.duration(t.file));
                self.put(0x28b4, duration);
                self.put(0x28b8, duration);
                if duration == 0 {
                    host.ambient(Track { file: "", volume: 100 }, 0)?;
                    return Ok(());
                }
                if scene.music_enabled != 0 {
                    self.seek(host, scene, request.seek_limit)?;
                } else {
                    self.put(0x28b8, 0);
                }
                // Inversión original: duración de 2894 -> reproducir 2890 y viceversa.
                let track = bank.town_atmosphere[if host.weather_flag1() != 0 {
                    2
                } else if scene.phase > 1 && scene.phase < 6 {
                    0
                } else {
                    1
                }];
                let remaining = host.ambient(track, self.word(0x28b8))?;
                self.put(0x28c4, scene.now.wrapping_add(remaining).wrapping_sub(5000));
            }
            _ => {}
        }
        Ok(())
    }
    /// 0x0042CD90. A diferencia de update, aplica town_ids antes de buscar pista.
    pub fn start_town_music(
        &mut self,
        host: &mut impl ScreenServices,
        scene: &Scene<'_>,
        bank: &Catalog<'_>,
        town: u8,
    ) -> Result<(), ScreenError> {
        if town as u32 >= scene.towns_count {
            return Ok(());
        }
        let id = *scene
            .town_ids
            .get(town as usize)
            .ok_or(ScreenError::MissingTown { index: town as u32 })?;
        let duration = host.music(Self::track(bank.town, id as u32)?, 0);
        let delay = Self::delay(Self::random(host)?, -5000.0);
        self.music_deadlines(scene.now, duration, delay, 15000);
        Ok(())
    }
}

#[derive(Clone, Copy)]
pub struct Atmosphere {
    pub mode: u8,
    pub stop: u8,
    pub force: u8,
    pub seek_limit: u32,
}
