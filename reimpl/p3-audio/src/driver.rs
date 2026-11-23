//! Apertura y negociación del dispositivo, con Miles y servicio como puertos.
use crate::lifecycle::AudioGlobals;
use crate::{Backend, SoundManager, MUSIC, SAMPLES};

pub struct DeviceState {
    pub rate: i32,
    pub bits: i32,
    pub channels: i32,
    /// WAVEFORMATEX de 18 bytes; cbSize se conserva, no se pone a cero.
    pub wave_format: [u8; 18],
    pub globals: AudioGlobals,
}

pub struct WaveOpen {
    pub result: i32,
    /// Valor del parámetro de salida, incluso cuando result indica fallo.
    pub driver: u32,
}

pub struct FilterCandidate {
    pub provider: u32,
    /// None significa que el proveedor no escribió un nombre conocido.
    pub name: Option<String>,
}

pub trait DriverBackend {
    fn set_redist_directory(&mut self, path: &str);
    fn register_shutdown(&mut self);
    fn startup(&mut self) -> i32;
    fn last_error(&mut self) -> String;
    fn show_startup_error(&mut self, message: &str);
    fn set_preference(&mut self, id: u32, value: i32);
    fn get_preference(&mut self, id: u32) -> i32;
    fn wave_out_open(&mut self, current_driver: u32, format: &[u8; 18]) -> WaveOpen;
    fn digital_configuration(&mut self, driver: u32) -> String;
    fn close_driver(&mut self, driver: u32);
    fn allocate_sample(&mut self, driver: u32) -> u32;
    fn allocate_effect_list(&mut self) -> u32;
    fn construct_effect_list(&mut self, storage: u32) -> u32;
    /// Una sola enumeración, cursor inicial cero; no se busca un segundo filtro.
    fn first_filter(&mut self, current_provider: u32) -> FilterCandidate;
    fn open_filter(&mut self, provider: u32, driver: u32);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverError {
    AttemptLimit { attempts: u32 },
    MissingFilterName,
}

impl DeviceState {
    fn update_format(&mut self) {
        let align = (self.bits / 8).wrapping_mul(self.channels);
        self.wave_format[0..2].copy_from_slice(&1u16.to_le_bytes());
        self.wave_format[2..4].copy_from_slice(&(self.channels as u16).to_le_bytes());
        self.wave_format[4..8].copy_from_slice(&self.rate.to_le_bytes());
        self.wave_format[8..12].copy_from_slice(&align.wrapping_mul(self.rate).to_le_bytes());
        self.wave_format[12..14].copy_from_slice(&(align as u16).to_le_bytes());
        self.wave_format[14..16].copy_from_slice(&(self.bits as u16).to_le_bytes());
    }
}

impl SoundManager {
    /// 0x00441E10. Conserva salida del backend, reintentos, fallos parciales y
    /// campos globales. max_attempts acota un proveedor que no aplica preferencias.
    pub fn open_audio_driver(
        &mut self,
        backend: &mut impl DriverBackend,
        state: &mut DeviceState,
        max_attempts: u32,
    ) -> Result<(), DriverError> {
        backend.set_redist_directory("./win32");
        backend.register_shutdown();
        if backend.startup() == 0 {
            let message = backend.last_error();
            backend.show_startup_error(&message);
        }
        backend.set_preference(15, 0);
        let mut attempts = 0;
        loop {
            if state.rate < 11025 {
                self.put(0x68, 0);
                return Ok(());
            }
            if attempts == max_attempts {
                return Err(DriverError::AttemptLimit { attempts });
            }
            attempts += 1;
            state.update_format();
            let result = backend.wave_out_open(self.word(0x68), &state.wave_format);
            self.put(0x68, result.driver);
            if result.result == 0 {
                let name = backend.digital_configuration(self.word(0x68));
                if backend.get_preference(15) == 0 && format!("Device: {name}").contains("Emulated") {
                    backend.close_driver(self.word(0x68));
                    backend.set_preference(15, 1);
                    // El original vuelve a establecerla en el destino del salto.
                    backend.set_preference(15, 1);
                    continue;
                }
                break;
            }
            if backend.get_preference(15) == 0 {
                backend.set_preference(15, 1);
                continue;
            }
            state.rate /= 2;
            if state.rate < 11025 && state.bits == 16 {
                state.bits = 8;
                state.rate = 44100;
            }
        }
        for i in 0..SAMPLES {
            let sample = backend.allocate_sample(self.word(0x68));
            self.put(0x18 + 4 * i, sample);
        }
        state.globals.service_flag = self.word(0x68);
        if state.globals.service_object == 0 {
            let storage = backend.allocate_effect_list();
            state.globals.service_object = if storage == 0 {
                0
            } else {
                backend.construct_effect_list(storage)
            };
        }
        let filter = backend.first_filter(self.word(0x6c));
        self.put(0x6c, filter.provider);
        let name = filter.name.ok_or(DriverError::MissingFilterName)?;
        if name == "LowPass Filter" {
            backend.open_filter(self.word(0x6c), self.word(0x68));
        } else {
            self.put(0x6c, 0);
        }
        Ok(())
    }

    /// 0x004429D0. El puerto agenda el trabajo; no debe ejecutarlo dentro de esta
    /// llamada. Si falta música se borra +0x120, pero no se cancela el trabajo.
    pub fn start_service(&mut self, backend: &mut impl ServiceBackend) {
        if !self.enabled() {
            return;
        }
        let token = backend.schedule_music_service();
        self.put(0x120, token);
        if token != 0 && self.word(MUSIC) == 0 {
            self.put(0x120, 0);
        }
    }
}

pub trait ServiceBackend: Backend {
    fn schedule_music_service(&mut self) -> u32;
    fn presentation_time(&mut self) -> Option<u32>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceError {
    ClockUnavailable,
    ZeroInterval,
}

/// Dependencia no catalogada 0x00442090, 146 bytes. Cada poll ejecuta una vuelta
/// de su espera activa. No crea un hilo, no duerme y puede devolver Pending.
pub struct MusicService {
    interval: u32,
    start: u32,
    step: u32,
    last_step: u32,
    volume: i32,
    done: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceProgress {
    Pending,
    Done,
}

impl MusicService {
    pub fn begin(manager: &mut SoundManager, backend: &mut impl ServiceBackend) -> Result<Self, ServiceError> {
        let volume = manager.word(0x1b8) as i32;
        let interval = if volume == 0 { 0 } else { (5000 / volume) as u32 };
        let start = backend.presentation_time().ok_or(ServiceError::ClockUnavailable)?;
        let volume = manager.word(0x1b8) as i32;
        let mut service = Self {
            interval,
            start,
            step: 0,
            last_step: u32::MAX,
            volume,
            done: false,
        };
        if volume <= 0 {
            service.finish(manager, backend);
        }
        Ok(service)
    }

    fn finish(&mut self, manager: &mut SoundManager, backend: &mut impl ServiceBackend) {
        manager.close(backend, MUSIC); // También pausa/cierra handle cero.
        manager.put(0x120, 0);
        self.done = true;
    }

    pub fn poll(
        &mut self,
        manager: &mut SoundManager,
        backend: &mut impl ServiceBackend,
    ) -> Result<ServiceProgress, ServiceError> {
        if self.done {
            return Ok(ServiceProgress::Done);
        }
        if self.step != self.last_step {
            let volume = (self.volume as u32).wrapping_sub(self.step) as i32;
            backend.set_stream_volume(manager.word(MUSIC), volume);
            self.last_step = self.step;
        }
        let now = backend.presentation_time().ok_or(ServiceError::ClockUnavailable)?;
        if self.interval == 0 {
            return Err(ServiceError::ZeroInterval);
        }
        self.step = now.wrapping_sub(self.start) / self.interval;
        self.volume = manager.word(0x1b8) as i32;
        if (self.step as i32) < self.volume {
            Ok(ServiceProgress::Pending)
        } else {
            self.finish(manager, backend);
            Ok(ServiceProgress::Done)
        }
    }
}
