//! Recursos, decodificación y mezcla nativos fuera de línea. La salida sigue siendo un puerto.
pub mod files;
pub mod ima;
pub mod lowpass;
pub mod mixer;
pub mod mp3_band;
pub mod mp3_huffman;
pub mod mp3_hybrid;
mod mp3_imdct_long;
mod mp3_imdct_short;
mod mp3_poly_dct;
mod mp3_poly_window;
pub mod mp3_polyphase;
pub mod mp3_requantize;
pub mod mp3_scales;
pub mod mp3_stereo;
pub mod playback;
pub mod stream;
pub mod timing;
pub mod wave;

use crate::effects::EffectsBackend;
use crate::Backend;
use files::{ResourceArena, ResourceError, ResourceFiles, ResourceLimits, Search};
use std::sync::Arc;
use wave::{WaveError, WaveInfo};

#[derive(Debug, Clone)]
pub struct WaveLease {
    bytes: Arc<[u8]>,
    filled: usize,
    info: WaveInfo,
}
impl WaveLease {
    pub fn decode_miles_ima(&self, max_frames: usize) -> Result<ima::Pcm16, ima::DecodeError> {
        ima::decode_miles_defined(self.bytes(), max_frames)
    }
    pub fn decode_ima(&self, options: ima::DecodeOptions) -> Result<ima::Pcm16, ima::DecodeError> {
        ima::decode(self.bytes(), options)
    }
    pub fn info(&self) -> &WaveInfo {
        &self.info
    }
    pub fn bytes(&self) -> &[u8] {
        &self.bytes[..self.filled]
    }
    pub fn data(&self) -> &[u8] {
        &self.bytes[self.info.data.clone()]
    }
}

/// Propiedad nativa explícita. Un dispositivo conserva la lease mientras usa los
/// datos; liberar el token de búfer no invalida una reproducción ya adjuntada.
/// `mixer::OfflineDevice` conserva el PCM decodificado; la salida física es externa.
pub trait EffectDevice: Backend {
    fn allocate_sample(&mut self, driver: u32) -> u32;
    fn init_sample(&mut self, handle: u32);
    fn release_sample(&mut self, handle: u32);
    fn set_sample_volume(&mut self, handle: u32, volume: i32);
    fn attach_wave(&mut self, handle: u32, wave: WaveLease, block: u32) -> i32;
    fn start_sample(&mut self, handle: u32);
    fn delete_effect_list_object(&mut self);
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NativeError {
    Resource(ResourceError),
    Wave(WaveError),
    UnsupportedFileReadArgument(u32),
    Mixer(mixer::MixerError),
    ArchiveState,
    UnsupportedOpenMode(u32),
    UnsupportedSampleName,
    SampleSize { requested: u32, filled: usize },
    UnsupportedFilter(u32),
    UnsupportedStreamService(u32),
    MissingEffectList,
    Catalog,
    Stream(stream::StreamError),
    UnsupportedStreamBuffer(u32),
    UnsupportedFilterStage(u32),
    UnknownFilterKey(u32),
}
impl From<ResourceError> for NativeError {
    fn from(e: ResourceError) -> Self {
        Self::Resource(e)
    }
}

pub struct NativeEffects<D> {
    pub device: D,
    pub files: ResourceFiles,
    pub arena: ResourceArena,
    /// Token lógico del contenedor montado; cero elige únicamente archivos sueltos.
    pub archive_token: u32,
    /// Por defecto, bit0 del contenedor original: suelto primero, luego CPR.
    pub archive_search: Search,
    errors: Vec<NativeError>,
    sample_file: Option<u32>,
    sample_archive: bool,
    /// Objeto auxiliar opcional, propiedad del cliente; sus canales comparten dispositivo.
    pub effect_list: Option<crate::effects::SoundEffectList>,
    pub stream_limits: stream::StreamLimits,
    pub mp3_backend: stream::Mp3Backend,
}
impl<D> NativeEffects<D> {
    pub fn new(device: D, files: ResourceFiles, limits: ResourceLimits) -> Self {
        Self {
            device,
            files,
            arena: ResourceArena::new(limits),
            archive_token: 1,
            archive_search: Search::LooseFirst,
            errors: vec![],
            sample_file: None,
            sample_archive: false,
            effect_list: None,
            stream_limits: stream::StreamLimits::default(),
            mp3_backend: stream::Mp3Backend::default(),
        }
    }
    pub fn take_errors(&mut self) -> Vec<NativeError> {
        std::mem::take(&mut self.errors)
    }
    fn resource_result<T: Default>(&mut self, result: Result<T, ResourceError>) -> T {
        match result {
            Ok(value) => value,
            Err(error) => {
                self.errors.push(error.into());
                T::default()
            }
        }
    }
    pub fn wave_lease(&self, buffer: u32) -> Result<WaveLease, NativeError> {
        let (bytes, filled) = self.arena.lease(buffer)?;
        // Se valida el prefijo realmente leído. No interpretar ceros de una
        // reserva como datos de audio si el archivo se truncó durante la carga.
        let info = WaveInfo::parse(&bytes[..filled]).map_err(NativeError::Wave)?;
        Ok(WaveLease { bytes, filled, info })
    }
}

impl<D: Backend> Backend for NativeEffects<D> {
    fn sample_status(&mut self, h: u32) -> u32 {
        self.device.sample_status(h)
    }
    fn stream_status(&mut self, h: u32) -> u32 {
        self.device.stream_status(h)
    }
    fn stream_volume(&mut self, h: u32) -> i32 {
        self.device.stream_volume(h)
    }
    fn set_stream_volume(&mut self, h: u32, v: i32) {
        self.device.set_stream_volume(h, v);
    }
    fn sample_pan(&mut self, h: u32) -> i32 {
        self.device.sample_pan(h)
    }
    fn set_sample_pan(&mut self, h: u32, v: u16) {
        self.device.set_sample_pan(h, v);
    }
    fn pause_stream(&mut self, h: u32, v: u32) {
        self.device.pause_stream(h, v);
    }
    fn close_stream(&mut self, h: u32) {
        self.device.close_stream(h);
    }
    fn stop_sample(&mut self, h: u32) {
        self.device.stop_sample(h);
    }
    fn end_sample(&mut self, h: u32) {
        self.device.end_sample(h);
    }
    fn clear_sound_slot_tables(&mut self) {
        self.device.clear_sound_slot_tables();
    }
}
impl<D: EffectDevice> EffectsBackend for NativeEffects<D> {
    fn allocate_sample(&mut self, driver: u32) -> u32 {
        self.device.allocate_sample(driver)
    }
    fn init_sample(&mut self, h: u32) {
        self.device.init_sample(h);
    }
    fn release_sample(&mut self, h: u32) {
        self.device.release_sample(h);
    }
    fn set_sample_volume(&mut self, h: u32, v: i32) {
        self.device.set_sample_volume(h, v);
    }
    fn free_locked_buffer(&mut self, buffer: u32) {
        let result = self.arena.free(buffer);
        self.resource_result(result);
    }
    fn allocate_locked_buffer(&mut self, size: u32) -> u32 {
        let result = self.arena.allocate(size);
        self.resource_result(result)
    }
    fn arc_create_file(&mut self, archive: u32, name: &str) -> u32 {
        let result = if archive == 0 {
            self.files.read(name, Search::LooseOnly)
        } else if archive == self.archive_token {
            self.files.read(name, self.archive_search)
        } else {
            Err(ResourceError::UnknownToken(archive))
        }
        .and_then(|bytes| self.arena.open(bytes));
        self.resource_result(result)
    }
    fn arc_file_length(&mut self, file: u32) -> u32 {
        let result = self.arena.length(file);
        self.resource_result(result)
    }
    fn arc_read(&mut self, file: u32, buffer: u32, size: u32) -> u32 {
        let result = self.arena.read(file, buffer, size);
        self.resource_result(result)
    }
    fn arc_release_file(&mut self, file: u32) {
        let result = self.arena.close(file);
        self.resource_result(result);
    }
    fn file_read(&mut self, name: &str, flags: u32) -> u32 {
        if flags != 0 {
            self.errors.push(NativeError::UnsupportedFileReadArgument(flags));
            return 0;
        }
        let result = self
            .files
            .read(name, Search::LooseOnly)
            .and_then(|bytes| self.arena.from_bytes(bytes));
        self.resource_result(result)
    }
    fn set_sample_file(&mut self, h: u32, buffer: u32, block: u32) -> i32 {
        match self.wave_lease(buffer) {
            Ok(lease) => self.device.attach_wave(h, lease, block),
            Err(error) => {
                self.errors.push(error);
                0
            }
        }
    }
    fn start_sample(&mut self, h: u32) {
        self.device.start_sample(h);
    }
    fn delete_effect_list_object(&mut self) {
        self.device.delete_effect_list_object();
    }
}
pub mod mp3_decoder;
pub mod mp3_frame;

pub mod async_hstream;
pub mod async_input;
pub mod async_playback;
pub mod hstream;
pub mod hstream_buffers;
pub mod progressive_playback;
pub mod sample_input;
