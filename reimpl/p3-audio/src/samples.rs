//! Volumen, selección de canales y caché de muestras.
use crate::filter::{self, FilterBackend};
use crate::{Backend, SoundManager, AMBIENT, AMBIENT_OUT, EFFECTS_VOLUME, MUSIC, SAMPLES};

/// Tablas suministradas por el cargador, sustituyendo los punteros de la imagen.
/// `gains` admite un valor contiguo explícito para contrastar la lectura original
/// de gains[count] cuando falta un ID. Sin ese valor se devuelve un error seguro.
pub struct SampleCatalog<'a> {
    pub ids: &'a [u32],
    pub gains: &'a [u16],
    pub names: &'a [&'a str],
    pub sizes: &'a [u32],
}

pub trait SampleBackend: Backend {
    fn sample_volume(&mut self, handle: u32) -> i32;
    fn set_sample_volume(&mut self, handle: u32, volume: i32);
    fn set_effect_list_volume_percent(&mut self, percent: i32, selector: i32);
}

/// Archivos y búferes son propiedad del adaptador. Sus tokens no son punteros Rust.
pub trait PlaybackBackend: SampleBackend + FilterBackend {
    fn allocate_buffer(&mut self, size: u32) -> u32;
    fn free_buffer(&mut self, buffer: u32);
    fn archive_new(&mut self);
    fn archive_open(&mut self, name: &str, mode: u32) -> bool;
    fn archive_read(&mut self, buffer: u32, size: u32) -> u32;
    fn archive_close(&mut self);
    fn archive_drop(&mut self);
    fn init_sample(&mut self, handle: u32);
    fn set_named_sample_file(&mut self, handle: u32, name: &str, buffer: u32, size: u32, block: u32) -> i32;
    fn start_sample(&mut self, handle: u32);
    fn service_stream(&mut self, handle: u32, fillup: u32);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SampleError {
    DivideByZero,
    DivideOverflow,
    MissingCatalogData { index: usize },
    MissingFilterKey,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SampleSlot(usize);

impl SampleSlot {
    pub fn index(self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PlaySound {
    pub id: u32,
    pub filtered: u8,
    pub reserved: u8,
    pub volume: i32,
    pub pan: i32,
    pub prepare_only: u8,
    pub ignore_locks: u8,
}

fn entry<T: Copy>(values: &[T], index: usize) -> Result<T, SampleError> {
    values
        .get(index)
        .copied()
        .ok_or(SampleError::MissingCatalogData { index })
}

fn divide(numerator: i32, denominator: i32) -> Result<i32, SampleError> {
    if denominator == 0 {
        Err(SampleError::DivideByZero)
    } else {
        numerator.checked_div(denominator).ok_or(SampleError::DivideOverflow)
    }
}

impl SoundManager {
    fn catalog_index(&self, catalog: &SampleCatalog<'_>, id: u32) -> Result<usize, SampleError> {
        let count = (self.word(0x134) as i32).max(0) as usize;
        for i in 0..count {
            if entry(catalog.ids, i)? == id {
                return Ok(i);
            }
        }
        Ok(count)
    }

    /// 0x00442E90. Conserva consultas duplicadas, orden y cambios previos a IDIV.
    pub fn set_effects_volume(
        &mut self,
        backend: &mut impl SampleBackend,
        catalog: &SampleCatalog<'_>,
        volume: i32,
        has_effect_list: bool,
    ) -> Result<(), SampleError> {
        if !self.enabled() {
            return Ok(());
        }
        for i in 0..SAMPLES {
            let index = self.catalog_index(catalog, self.word(0x7c + 4 * i))?;
            if (index as i32) < self.word(0x134) as i32 {
                let handle = self.word(0x18 + 4 * i);
                let old = backend.sample_volume(handle);
                let scaled = if old == 0 {
                    (entry(catalog.gains, index)? as i32).wrapping_mul(volume) / 100
                } else {
                    divide(old.wrapping_mul(volume), self.word(EFFECTS_VOLUME) as i32)?
                };
                if backend.sample_volume(handle) != scaled {
                    backend.set_sample_volume(handle, scaled.clamp(0, 127));
                }
            }
        }
        let (handle, target) = if self.word(AMBIENT) != 0 {
            (self.word(AMBIENT), 0x1d8)
        } else {
            (self.word(AMBIENT_OUT), 0x1d9)
        };
        if handle != 0 {
            let scaled = (self.image[target] as i32).wrapping_mul(volume) / 100;
            if backend.stream_volume(handle) != scaled {
                backend.set_stream_volume(handle, scaled.clamp(0, 127));
            }
        }
        self.put(EFFECTS_VOLUME, volume as u32);
        if has_effect_list {
            backend.set_effect_list_volume_percent(volume.wrapping_mul(100) / 127, -1);
        }
        Ok(())
    }

    /// 0x00443000. Handle ausente: no-op. ID ausente: el original lee gains[count].
    pub fn update_sample_volume(
        &self,
        backend: &mut impl SampleBackend,
        catalog: &SampleCatalog<'_>,
        handle: u32,
        volume: i32,
    ) -> Result<(), SampleError> {
        if self.enabled() {
            if let Some(i) = self.sample_slot(handle) {
                let index = self.catalog_index(catalog, self.word(0x7c + 4 * i))?;
                let gain = entry(catalog.gains, index)? as i32;
                let relative = (self.word(EFFECTS_VOLUME) as i32).wrapping_mul(volume) / 127;
                let scaled = gain.wrapping_mul(relative) / 100;
                if backend.sample_volume(handle) != scaled {
                    backend.set_sample_volume(handle, scaled.clamp(0, 127));
                }
            }
        }
        Ok(())
    }

    pub fn sample_handle(&self, slot: SampleSlot) -> u32 {
        self.word(0x18 + 4 * slot.0)
    }

    fn promote_cache(&mut self, index: usize) {
        for offset in [0x13c, 0xd0] {
            let value = self.word(offset + 4 * index);
            self.image.copy_within(offset..offset + 4 * index, offset + 4);
            self.put(offset, value);
        }
    }

    /// 0x00443150. Devuelve una referencia lógica al slot; el x86 devuelve &slot.
    /// La selección, caché y configuración se portan juntas para conservar el orden.
    pub fn play_sound(
        &mut self,
        backend: &mut impl PlaybackBackend,
        catalog: &SampleCatalog<'_>,
        request: PlaySound,
    ) -> Result<Option<SampleSlot>, SampleError> {
        if !self.enabled()
            || (request.ignore_locks == 0 && self.image[0x1df] & 1 != 0)
            || (request.ignore_locks == 0 && self.image[0x1de] == 0 && self.image[0x1e0] & 1 != 0)
        {
            return Ok(None);
        }
        let volume = if request.volume == -1 { 127 } else { request.volume };
        if request.id == 0x1770 {
            if let Some(i) = (0..SAMPLES).find(|i| self.word(0x7c + 4 * i) == request.id) {
                if backend.sample_status(self.word(0x18 + 4 * i)) == 4 {
                    return Ok(None);
                }
            }
        }
        if self.count_playing_samples(backend) == 20 {
            return Ok(None);
        }
        let index;
        if let Some(hit) = (0..SAMPLES).find(|i| self.word(0x13c + 4 * i) == request.id) {
            self.promote_cache(hit);
            index = self.catalog_index(catalog, request.id)?;
        } else {
            index = self.catalog_index(catalog, request.id)?;
            if index as i32 >= self.word(0x134) as i32 {
                return Ok(None);
            }
            let size = entry(catalog.sizes, index)?;
            let buffer = backend.allocate_buffer(size);
            if buffer == 0 {
                return Ok(None);
            }
            backend.archive_new();
            if !backend.archive_open(entry(catalog.names, index)?, 0x8000) {
                backend.free_buffer(buffer);
                backend.archive_drop();
                return Ok(None);
            }
            if backend.archive_read(buffer, size) != size {
                backend.free_buffer(buffer);
                backend.archive_close();
                backend.archive_drop();
                return Ok(None);
            }
            backend.archive_close();
            let evicted_id = self.word(0x188);
            if let Some(i) = (0..SAMPLES).find(|i| self.word(0x7c + 4 * i) == evicted_id) {
                let handle = self.word(0x18 + 4 * i);
                if backend.sample_status(handle) == 4 || self.image[0x18c + i] != 0 {
                    // Fuga del original: el búfer nuevo no se libera al rechazar la expulsión.
                    backend.archive_drop();
                    return Ok(None);
                }
            }
            backend.free_buffer(self.word(0x11c));
            self.image.copy_within(0xd0..0x11c, 0xd4);
            self.image.copy_within(0x13c..0x188, 0x140);
            self.put(0xd0, buffer);
            self.put(0x13c, request.id);
            let count = u16::from_le_bytes(self.image[0x1b4..0x1b6].try_into().unwrap()).wrapping_add(1);
            self.image[0x1b4..0x1b6].copy_from_slice(&count.to_le_bytes());
            backend.archive_drop();
        }
        // El volumen cero se rechaza después de modificar la caché y el contador.
        if volume == 0 {
            return Ok(None);
        }
        let Some(i) =
            (0..SAMPLES).find(|i| self.image[0x18c + i] == 0 && backend.sample_status(self.word(0x18 + 4 * i)) != 4)
        else {
            return Ok(None);
        };
        let handle = self.word(0x18 + 4 * i);
        backend.init_sample(handle);
        self.put(0x7c + 4 * i, request.id);
        let configured = backend.set_named_sample_file(
            handle,
            entry(catalog.names, index)?,
            self.word(0xd0),
            entry(catalog.sizes, index)?,
            0,
        );
        self.set_sample_pan(backend, handle, request.pan);
        if request.prepare_only == 0 {
            let relative = (self.word(EFFECTS_VOLUME) as i32).wrapping_mul(volume) / 127;
            let scaled = (entry(catalog.gains, index)? as i32).wrapping_mul(relative) / 100;
            backend.set_sample_volume(handle, scaled.clamp(0, 127));
            backend.start_sample(handle);
            for offset in [AMBIENT, MUSIC] {
                let stream = self.word(offset);
                if stream != 0 {
                    backend.service_stream(stream, 0);
                }
            }
        }
        if request.filtered != 0 {
            self.image[0x1a0 + i] = 1;
            if self.image[0x1c5] != 0 {
                let provider = self.word(0x6c);
                let key = filter::last_key(backend, provider).map_err(|_| SampleError::MissingFilterKey)?;
                backend.set_sample_processor(handle, 1, provider);
                backend.set_filter_preference(handle, key, 400f32.to_bits());
            }
        }
        if configured != 0 && request.reserved & 1 != 0 {
            self.image[0x18c + i] = 1;
        }
        Ok(Some(SampleSlot(i)))
    }
}
