//! Adaptador de muestras/caché de SoundManager. Una sesión Archiver a la vez.
//! Muestras, streams y Lowpass nativos; el modo asíncrono usa un hilo de entrada.
use super::mixer::OfflineDevice;
use super::{EffectDevice, NativeEffects, NativeError};
use crate::effects::EffectGlobals;
use crate::filter::FilterBackend;
use crate::samples::{PlaybackBackend, SampleBackend, SampleCatalog};
use crate::{SoundManager, IMAGE_SIZE, SAMPLES};

impl NativeEffects<OfflineDevice> {
    /// Bootstrap nativo desde imagen cero, no sustituto del arranque Miles completo.
    /// Reserva las veinte voces de forma transaccional; las tablas siguen prestadas.
    /// El catálogo suministrado debe permanecer completo y estable durante la sesión.
    pub fn sample_manager(&mut self, catalog: &SampleCatalog<'_>) -> Result<SoundManager, NativeError> {
        let n = catalog.ids.len();
        if n > i32::MAX as usize || [catalog.gains.len(), catalog.names.len(), catalog.sizes.len()] != [n; 3] {
            return Err(NativeError::Catalog);
        }
        let mut allocated = Vec::with_capacity(SAMPLES);
        for _ in 0..SAMPLES {
            match self.device.allocate(OfflineDevice::DRIVER) {
                Ok(h) => allocated.push(h),
                Err(e) => {
                    for h in allocated {
                        self.device.release(h).expect("voz recién reservada");
                    }
                    return Err(NativeError::Mixer(e));
                }
            }
        }
        let mut manager = SoundManager::from_image([0; IMAGE_SIZE]);
        manager.initialize();
        // El constructor original no escribe estas tablas de IDs. Elegimos un
        // patrón inicial ausente del catálogo para representar ranuras vacías.
        let ids: std::collections::BTreeSet<_> = catalog.ids.iter().copied().collect();
        let empty = (0..=n as u32)
            .find(|id| !ids.contains(id))
            .expect("n+1 candidatos para n IDs");
        for i in 0..SAMPLES {
            manager.put(0x7c + 4 * i, empty);
            manager.put(0x13c + 4 * i, empty);
        }
        for (i, h) in allocated.into_iter().enumerate() {
            manager.put(0x18 + 4 * i, h);
        }
        manager.put(0x68, OfflineDevice::DRIVER);
        manager.put(0x6c, super::lowpass::LOWPASS_PROVIDER);
        manager.put(0x134, n as u32);
        Ok(manager)
    }

    /// Libera solo voces y caché de este gestor. No destruye tablas ni objetos C++.
    /// Las asignaciones fugadas por el algoritmo original siguen en la arena.
    pub fn release_sample_manager(&mut self, manager: &mut SoundManager) {
        for i in 0..SAMPLES {
            let h = manager.word(0x18 + 4 * i);
            if h != 0 {
                if let Err(e) = self.device.release(h) {
                    self.errors.push(NativeError::Mixer(e));
                }
                manager.put(0x18 + 4 * i, 0);
            }
            let buffer = manager.word(0xd0 + 4 * i);
            let result = self.arena.free(buffer);
            self.resource_result(result);
            manager.put(0xd0 + 4 * i, 0);
        }
        manager.put(0x68, 0);
    }

    fn close_sample_file(&mut self) {
        if let Some(file) = self.sample_file.take() {
            let result = self.arena.close(file);
            self.resource_result(result);
        }
    }
}

impl SampleBackend for NativeEffects<OfflineDevice> {
    fn sample_volume(&mut self, h: u32) -> i32 {
        if h == 0 {
            return 0;
        }
        match self.device.volume(h) {
            Ok(v) => v,
            Err(e) => {
                self.errors.push(NativeError::Mixer(e));
                0
            }
        }
    }
    fn set_sample_volume(&mut self, h: u32, v: i32) {
        EffectDevice::set_sample_volume(&mut self.device, h, v);
    }
    fn set_effect_list_volume_percent(&mut self, percent: i32, selector: i32) {
        if let Some(mut list) = self.effect_list.take() {
            let globals = EffectGlobals {
                driver: OfflineDevice::DRIVER,
                volume_percent: percent,
                archive: self.archive_token,
            };
            list.set_volume_percent(self, &globals, percent, selector);
            self.effect_list = Some(list);
        } else {
            self.errors.push(NativeError::MissingEffectList);
        }
    }
}
impl FilterBackend for NativeEffects<OfflineDevice> {
    fn next_filter_attribute(&mut self, provider: u32, cursor: &mut u32, key: &mut Option<u32>) -> bool {
        if provider != super::lowpass::LOWPASS_PROVIDER {
            self.errors.push(NativeError::UnsupportedFilter(provider));
            return false;
        }
        if *cursor != 0 {
            return false;
        }
        *cursor = 1;
        *key = Some(super::lowpass::CUTOFF_KEY);
        true
    }
    fn set_sample_processor(&mut self, h: u32, stage: u32, provider: u32) {
        if h == 0 {
            return;
        }
        if stage != 1 {
            self.errors.push(NativeError::UnsupportedFilterStage(stage));
            return;
        }
        if provider != 0 && provider != super::lowpass::LOWPASS_PROVIDER {
            self.errors.push(NativeError::UnsupportedFilter(provider));
            return;
        }
        if let Err(e) = self.device.set_lowpass(h, provider != 0) {
            self.errors.push(NativeError::Mixer(e));
        }
    }
    fn set_filter_preference(&mut self, h: u32, key: u32, value_bits: u32) {
        if h == 0 {
            return;
        }
        if key != super::lowpass::CUTOFF_KEY {
            self.errors.push(NativeError::UnknownFilterKey(key));
            return;
        }
        if let Err(e) = self.device.set_lowpass_cutoff(h, value_bits) {
            self.errors.push(NativeError::Mixer(e));
        }
    }
}
impl PlaybackBackend for NativeEffects<OfflineDevice> {
    fn allocate_buffer(&mut self, size: u32) -> u32 {
        let result = self.arena.allocate(size);
        self.resource_result(result)
    }
    fn free_buffer(&mut self, buffer: u32) {
        let result = self.arena.free(buffer);
        self.resource_result(result);
    }
    fn archive_new(&mut self) {
        if self.sample_archive {
            self.errors.push(NativeError::ArchiveState);
        }
        self.sample_archive = true;
    }
    fn archive_open(&mut self, name: &str, mode: u32) -> bool {
        if !self.sample_archive {
            self.errors.push(NativeError::ArchiveState);
            return false;
        }
        if mode != 0x8000 {
            self.errors.push(NativeError::UnsupportedOpenMode(mode));
            return false;
        }
        self.close_sample_file();
        let result = self
            .files
            .read(name, self.archive_search)
            .and_then(|data| self.arena.open(data));
        let file = self.resource_result(result);
        self.sample_file = (file != 0).then_some(file);
        file != 0
    }
    fn archive_read(&mut self, buffer: u32, size: u32) -> u32 {
        let Some(file) = self.sample_file else {
            self.errors.push(NativeError::ArchiveState);
            return 0;
        };
        let result = self.arena.read(file, buffer, size);
        self.resource_result(result)
    }
    fn archive_close(&mut self) {
        self.close_sample_file();
    }
    fn archive_drop(&mut self) {
        self.close_sample_file();
        self.sample_archive = false;
    }
    fn init_sample(&mut self, h: u32) {
        EffectDevice::init_sample(&mut self.device, h);
    }
    fn set_named_sample_file(&mut self, h: u32, name: &str, buffer: u32, size: u32, block: u32) -> i32 {
        if !name.to_ascii_lowercase().ends_with(".wav") {
            self.errors.push(NativeError::UnsupportedSampleName);
            return 0;
        }
        match self.wave_lease(buffer) {
            Ok(lease) => {
                if lease.bytes().len() != size as usize {
                    self.errors.push(NativeError::SampleSize {
                        requested: size,
                        filled: lease.bytes().len(),
                    });
                    0
                } else {
                    self.device.attach_wave(h, lease, block)
                }
            }
            Err(e) => {
                self.errors.push(e);
                0
            }
        }
    }
    fn start_sample(&mut self, h: u32) {
        EffectDevice::start_sample(&mut self.device, h);
    }
    fn service_stream(&mut self, h: u32, _fillup: u32) {
        if h != 0 {
            if let Err(e) = self.device.service_stream_handle(h) {
                self.errors.push(NativeError::Mixer(e));
            }
        }
    }
}

impl crate::streams::StreamBackend for NativeEffects<OfflineDevice> {
    fn open_stream(&mut self, driver: u32, name: &str, buffer_size: u32) -> u32 {
        if driver != OfflineDevice::DRIVER {
            self.errors
                .push(NativeError::Mixer(super::mixer::MixerError::UnknownDriver(driver)));
            return 0;
        }
        if !matches!(buffer_size, 0 | 0x3e800) {
            self.errors.push(NativeError::UnsupportedStreamBuffer(buffer_size));
            return 0;
        }
        if name.to_ascii_lowercase().ends_with(".mp3")
            && matches!(
                self.mp3_backend,
                super::stream::Mp3Backend::AsyncNative | super::stream::Mp3Backend::AsyncNativeHstream
            )
        {
            let source = match self.files.open_stream(name, self.archive_search) {
                Ok(source) => source,
                Err(e) => {
                    self.errors.push(NativeError::Resource(e));
                    return 0;
                }
            };
            let result = if self.mp3_backend == super::stream::Mp3Backend::AsyncNativeHstream {
                self.device
                    .open_stream_mp3_async_hstream(driver, source, self.stream_limits)
            } else {
                self.device.open_stream_mp3_async(driver, source, self.stream_limits)
            };
            return match result {
                Ok(h) => h,
                Err(e) => {
                    self.errors.push(NativeError::Mixer(e));
                    0
                }
            };
        }
        let bytes = match self.files.read(name, self.archive_search) {
            Ok(bytes) => bytes,
            Err(e) => {
                self.errors.push(NativeError::Resource(e));
                return 0;
            }
        };
        if name.to_ascii_lowercase().ends_with(".mp3")
            && self.mp3_backend == super::stream::Mp3Backend::NativeProgressive
        {
            return match self
                .device
                .open_stream_mp3_progressive(driver, bytes, self.stream_limits)
            {
                Ok(h) => h,
                Err(e) => {
                    self.errors.push(NativeError::Mixer(e));
                    0
                }
            };
        }
        let pcm = match super::stream::decode_stream(bytes, name, self.stream_limits) {
            Ok(pcm) => pcm,
            Err(e) => {
                self.errors.push(NativeError::Stream(e));
                return 0;
            }
        };
        match self
            .device
            .open_stream_pcm(driver, std::sync::Arc::new(pcm), self.stream_limits)
        {
            Ok(h) => h,
            Err(e) => {
                self.errors.push(NativeError::Mixer(e));
                0
            }
        }
    }
    fn set_stream_ms_position(&mut self, h: u32, position: u32) {
        if h != 0 {
            if let Err(e) = self.device.seek_stream_ms(h, position) {
                self.errors.push(NativeError::Mixer(e));
            }
        }
    }
    fn stream_ms_position(&mut self, h: u32, want_position: bool) -> (u32, u32) {
        match self.device.stream_snapshot(h) {
            Ok(s) => (s.total_ms, if want_position { s.current_ms } else { 0 }),
            Err(e) => {
                if h != 0 {
                    self.errors.push(NativeError::Mixer(e));
                }
                (0, 0)
            }
        }
    }
    fn start_stream(&mut self, h: u32) {
        if h != 0 {
            if let Err(e) = self.device.start_stream_handle(h) {
                self.errors.push(NativeError::Mixer(e));
            }
        }
    }
    fn set_stream_loop_count(&mut self, h: u32, count: u32) {
        if h != 0 {
            if let Err(e) = self.device.stream_loops(h, count) {
                self.errors.push(NativeError::Mixer(e));
            }
        }
    }
    fn stream_sample(&mut self, h: u32) -> u32 {
        match self.device.stream_snapshot(h) {
            Ok(s) => s.sample,
            Err(e) => {
                if h != 0 {
                    self.errors.push(NativeError::Mixer(e));
                }
                0
            }
        }
    }
}
