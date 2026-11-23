//! CSoundEffectList: cuatro canales, veinte nombres y diez búferes residentes FIFO.
use crate::Backend;
use std::collections::VecDeque;

pub const EFFECT_LIST_SIZE: usize = 0x84;
pub const CLASS_NAME: &str = "CSoundEffectList";
/// Metadatos de la tabla original, no archivos de audio.
pub const DEFAULT_EFFECT_FILES: [&str; 20] = [
    "musik\\MenueSounds\\okbottun_menu.WAV",
    "musik\\MenueSounds\\alleButtons.WAV",
    "musik\\SeekarteWaves\\hawk.wav",
    "musik\\SeekarteWaves\\hawk.wav",
    "musik\\SeekarteWaves\\hawk.wav",
    "musik\\SeekarteWaves\\hawk.wav",
    "musik\\SeekarteWaves\\hawk.wav",
    "sound/sound02.wav",
    "sound/sound03.wav",
    "sound/sound04.wav",
    "sound/sound05.wav",
    "sound/sound06.wav",
    "sound/sound07.wav",
    "sound/sound08.wav",
    "sound/sound09.wav",
    "sound/sound10.wav",
    "sound/sound11.wav",
    "sound/sound12.wav",
    "sound/sound13.wav",
    "sound/sound14.wav",
];

pub struct EffectGlobals {
    pub driver: u32,
    pub volume_percent: i32,
    pub archive: u32,
}

pub trait EffectsBackend: Backend {
    fn allocate_sample(&mut self, driver: u32) -> u32;
    fn init_sample(&mut self, handle: u32);
    fn release_sample(&mut self, handle: u32);
    fn set_sample_volume(&mut self, handle: u32, volume: i32);
    fn free_locked_buffer(&mut self, buffer: u32);
    fn allocate_locked_buffer(&mut self, size: u32) -> u32;
    fn arc_create_file(&mut self, archive: u32, name: &str) -> u32;
    fn arc_file_length(&mut self, file: u32) -> u32;
    fn arc_read(&mut self, file: u32, buffer: u32, size: u32) -> u32;
    fn arc_release_file(&mut self, file: u32);
    fn file_read(&mut self, name: &str, flags: u32) -> u32;
    fn set_sample_file(&mut self, handle: u32, buffer: u32, block: u32) -> i32;
    fn start_sample(&mut self, handle: u32);
    /// Solicitud de scalar deleting dtor. El cliente descarta el objeto Rust.
    fn delete_effect_list_object(&mut self);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EffectsError {
    InvalidPlayIndex(i32),
    InconsistentQueue,
}

pub struct SoundEffectList {
    image: [u8; EFFECT_LIST_SIZE],
    order: VecDeque<i32>,
    files: [String; 20],
}

impl SoundEffectList {
    pub fn from_image(image: [u8; EFFECT_LIST_SIZE], order: VecDeque<i32>) -> Self {
        Self::with_files(image, order, DEFAULT_EFFECT_FILES.map(str::to_owned))
    }
    /// El adaptador puede suministrar la tabla de nombres. No se lee el EXE.
    pub fn with_files(image: [u8; EFFECT_LIST_SIZE], order: VecDeque<i32>, files: [String; 20]) -> Self {
        Self { image, order, files }
    }
    pub fn image(&self) -> &[u8; EFFECT_LIST_SIZE] {
        &self.image
    }
    pub fn order(&self) -> &VecDeque<i32> {
        &self.order
    }
    fn word(&self, off: usize) -> u32 {
        u32::from_le_bytes(self.image[off..off + 4].try_into().unwrap())
    }
    fn put(&mut self, off: usize, value: u32) {
        self.image[off..off + 4].copy_from_slice(&value.to_le_bytes());
    }
    fn queue_clear(&mut self) {
        self.order.clear();
        for off in [0x68, 0x6c, 0x70, 0x74, 0x78] {
            self.put(off, 0);
        }
    }
    fn queue_ctor(&mut self) {
        self.queue_clear();
        self.put(0x64, 0x67cf80);
        self.put(0x7c, 10);
    }
    fn check_queue(&self) -> Result<(), EffectsError> {
        if self.word(0x70) as usize == self.order.len() {
            Ok(())
        } else {
            Err(EffectsError::InconsistentQueue)
        }
    }
    fn finish_ctor(&mut self, b: &mut impl EffectsBackend, g: &EffectGlobals) -> Result<(), EffectsError> {
        for i in 0..4 {
            self.put(4 + 4 * i, 0);
            if g.driver != 0 {
                let handle = b.allocate_sample(g.driver);
                self.put(4 + 4 * i, handle);
                b.init_sample(handle);
            }
        }
        for i in 0..10 {
            self.load_effect(b, g, i)?;
        }
        self.set_volume_percent(b, g, g.volume_percent, 0);
        Ok(())
    }
    /// 0x004C4130. Este constructor sí limpia los 24 slots antes de liberar.
    pub fn initialize_default(&mut self, b: &mut impl EffectsBackend, g: &EffectGlobals) -> Result<(), EffectsError> {
        self.put(0, 0x670b08);
        self.queue_ctor();
        self.put(0, 0x671360);
        self.image[4..0x64].fill(0);
        self.release_all(b);
        self.finish_ctor(b, g)
    }
    /// 0x004C4240. La base ignora el nombre. Conserva los slots hasta release_all;
    /// el cliente debe suministrar un estado previo conocido, no memoria sin definir.
    pub fn initialize_named(
        &mut self,
        b: &mut impl EffectsBackend,
        g: &EffectGlobals,
        _name: &str,
    ) -> Result<(), EffectsError> {
        self.put(0, 0x670b08);
        self.queue_ctor();
        self.put(0, 0x671360);
        self.release_all(b);
        self.finish_ctor(b, g)
    }
    /// 0x004C4460. Conserva la segunda consulta si la primera devuelve 2.
    pub fn release_all(&mut self, b: &mut impl EffectsBackend) {
        for i in 0..4 {
            let off = 4 + 4 * i;
            let h = self.word(off);
            if h != 0 && (b.sample_status(h) != 2 || b.sample_status(h) != 1) {
                b.end_sample(h);
                b.release_sample(h);
                self.put(off, 0);
            }
        }
        // El original recorre diez slots aunque la tabla y load_effect admiten veinte.
        // No reparar aquí la posible fuga de los IDs 10..19: es paridad, no política.
        for i in 0..10 {
            let off = 0x14 + 4 * i;
            let buffer = self.word(off);
            if buffer != 0 {
                b.free_locked_buffer(buffer);
            }
            self.put(off, 0);
        }
        self.queue_clear();
    }
    /// 0x004C4340. Las dos tablas virtuales son tokens, no vtables Rust.
    pub fn destroy(&mut self, b: &mut impl EffectsBackend) {
        self.put(0, 0x671360);
        self.release_all(b);
        self.put(0x64, 0x67cf80);
        self.queue_clear();
        self.put(0, 0x670b08);
    }
    /// 0x004C4780. Solo el bit cero solicita liberar el propio objeto.
    pub fn deleting_destroy(&mut self, b: &mut impl EffectsBackend, flags: u32) {
        self.destroy(b);
        if flags & 1 != 0 {
            b.delete_effect_list_object();
        }
    }
    /// 0x004C4540. Segundo argumento ignorado; producto i32 con vuelta, sin saturar.
    pub fn set_volume_percent(&mut self, b: &mut impl EffectsBackend, g: &EffectGlobals, volume: i32, _selector: i32) {
        if g.driver == 0 {
            return;
        }
        let scaled = volume.wrapping_mul(127) / 100;
        for i in 0..4 {
            let h = self.word(4 + 4 * i);
            if h != 0 {
                b.set_sample_volume(h, scaled);
            }
        }
    }
    /// 0x004C4620. Una carga fallida también entra en FIFO. No refresca aciertos.
    pub fn load_effect(
        &mut self,
        b: &mut impl EffectsBackend,
        g: &EffectGlobals,
        index: i32,
    ) -> Result<(), EffectsError> {
        if !(0..20).contains(&index) {
            return Ok(());
        }
        let off = 0x14 + 4 * index as usize;
        if self.word(off) != 0 {
            return Ok(());
        }
        if self.word(0x70) as i32 >= 10 {
            self.check_queue()?;
            let old = self.order.pop_front().ok_or(EffectsError::InconsistentQueue)?;
            self.put(0x70, self.order.len() as u32);
            if old != index && (0..20).contains(&old) {
                let slot = 0x14 + 4 * old as usize;
                b.free_locked_buffer(self.word(slot));
                self.put(slot, 0);
            }
        }
        let name = self.files[index as usize].clone();
        if name.is_empty() {
            return Ok(());
        }
        self.put(off, 0);
        let file = b.arc_create_file(g.archive, &name);
        if file != 0 {
            let size = b.arc_file_length(file);
            let buffer = b.allocate_locked_buffer(size);
            self.put(off, buffer);
            if buffer != 0 {
                b.arc_read(file, buffer, size);
            }
        }
        b.arc_release_file(file); // También si create_file devolvió cero.
        if self.word(off) == 0 {
            let buffer = b.file_read(&name, 0);
            self.put(off, buffer);
        }
        self.check_queue()?;
        self.order.push_back(index);
        self.put(0x70, self.order.len() as u32);
        Ok(())
    }
    /// 0x004C45A0. true significa petición de arranque, no un retorno del x86 (void).
    pub fn play(&mut self, b: &mut impl EffectsBackend, g: &EffectGlobals, index: i32) -> Result<bool, EffectsError> {
        let slot = (0..4).find(|&i| {
            let h = self.word(4 + 4 * i);
            h != 0 && b.sample_status(h) == 2
        });
        let Some(slot) = slot else {
            return Ok(false);
        };
        if !(0..20).contains(&index) {
            return Err(EffectsError::InvalidPlayIndex(index));
        }
        let off = 0x14 + 4 * index as usize;
        if self.word(off) == 0 {
            self.load_effect(b, g, index)?;
        }
        let buffer = self.word(off);
        if buffer == 0 {
            return Ok(false);
        }
        let h = self.word(4 + 4 * slot);
        if b.set_sample_file(h, buffer, 1) != 0 {
            b.start_sample(h);
            Ok(true)
        } else {
            Ok(false)
        }
    }
    /// 0x004C4710. Tamaño de la tabla, no el número residente.
    pub fn effect_count(&self) -> u32 {
        20
    }
    /// 0x004C4750. Contenido del CString devuelto, sin reproducir su ABI C++.
    pub fn class_name(&self) -> &'static str {
        CLASS_NAME
    }
}
