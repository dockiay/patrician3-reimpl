//! Constructor, títulos de edificio y cierre. Los tokens C++ no son punteros Rust.
use crate::banks::BankResources;
use crate::{Backend, SoundManager, SAMPLES};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleError {
    MissingWord { address: u32 },
    InvalidToken { token: u32 },
    AllocationFailed,
    Capacity,
}

/// El adaptador es dueño de las asignaciones. Los arrays CString incluyen una
/// cabecera count en token-4. Sus constructores/destructores C++ son servicios.
pub trait LifecycleMemory {
    fn read_word(&self, address: u32) -> Option<u32>;
    fn allocate_cstrings(&mut self, count: u16) -> u32;
    fn write_cstring(&mut self, array: u32, index: u16, text: &str) -> Result<(), LifecycleError>;
    fn destroy_cstrings(&mut self, array: u32, count: u32) -> Result<(), LifecycleError>;
    fn free_heap(&mut self, token: u32) -> Result<(), LifecycleError>;
}

pub trait LifecycleBackend: Backend {
    fn release_sample(&mut self, handle: u32);
    fn free_locked_buffer(&mut self, buffer: u32);
    /// Solicitud equivalente a Sleep(1000), no una espera ejecutada por la crate.
    fn wait_shutdown(&mut self, milliseconds: u32);
    fn destroy_service_object(&mut self, object: u32, flags: u32);
    fn close_audio_driver(&mut self, driver: u32);
    fn shutdown_audio_library(&mut self);
}

/// Globales 0x006DCCDC y 0x006DCCD0, suministrados por el cliente.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioGlobals {
    pub service_object: u32,
    pub service_flag: u32,
}

impl SoundManager {
    /// 0x004419A0. Conserva todos los bytes que el constructor original no escribe.
    /// No presupone memoria inicialmente cero; no es seguro reinicializar un
    /// gestor con recursos vivos sin que el cliente resuelva primero su propiedad.
    pub fn initialize(&mut self) -> &mut Self {
        for offset in [
            0, 4, 8, 12, 16, 20, 0x68, 0x74, 0x78, 0x120, 0x124, 0x128, 0x12c, 0x130, 0x134, 0x138, 0x1c8, 0x1cc,
            0x1d0, 0x1d4,
        ] {
            self.put(offset, 0);
        }
        self.image[0xd0..0x120].fill(0);
        self.image[0x18c..0x1b4].fill(0);
        self.image[0x1b4..0x1b6].fill(0);
        for offset in [0x1b8, 0x1bc, 0x1c0] {
            self.put(offset, 80);
        }
        for offset in [
            0x1c5, 0x1c6, 0x1c7, 0x1d8, 0x1d9, 0x1da, 0x1db, 0x1dd, 0x1de, 0x1df, 0x1e0,
        ] {
            self.image[offset] = 0;
        }
        self.image[0x1c4] = 3;
        self.image[0x1dc] = 1;
        self
    }

    /// 0x00441CD0. Devuelve los títulos como datos Rust para el consumidor de voz;
    /// el adaptador conserva la asignación identificada por +0xCC. Volver a cargar
    /// sobrescribe el token sin liberar el anterior, igual que el original.
    pub fn load_building_music_config(
        &mut self,
        memory: &mut impl LifecycleMemory,
        resources: &mut impl BankResources,
    ) -> Result<Vec<String>, LifecycleError> {
        const FILE: &str = "scripts/sound.ini";
        let count = resources.integer(FILE, "Gebaeude", "Titel", 0) as u16;
        let array = memory.allocate_cstrings(count);
        self.put(0xcc, array);
        if array == 0 && count != 0 {
            return Err(LifecycleError::AllocationFailed);
        }
        let mut titles = Vec::new();
        titles
            .try_reserve_exact(count as usize)
            .map_err(|_| LifecycleError::Capacity)?;
        for index in 0..count {
            let text = resources
                .text(FILE, "Gebaeude", &format!("Titel{index}"), 4096)
                .unwrap_or_default();
            memory.write_cstring(array, index, &text)?;
            titles.push(text);
        }
        let mode = resources.integer(FILE, "Gebaeude", "m_BMusicVolumeInBuildings", 0) as u8;
        self.image[0x1c4] = if mode == 0 { 2 } else { mode };
        Ok(titles)
    }

    fn destroy_string_table(memory: &mut impl LifecycleMemory, token: u32) -> Result<(), LifecycleError> {
        let header = token.wrapping_sub(4);
        let count = memory
            .read_word(header)
            .ok_or(LifecycleError::MissingWord { address: header })?;
        memory.destroy_cstrings(token, count)?;
        memory.free_heap(header)
    }

    /// 0x00441AC0. No es idempotente: conserva +0x12C e incrementa +0x138 después
    /// de liberar sus listas. Un adaptador real debe rechazar tokens ya liberados.
    pub fn shutdown(
        &mut self,
        backend: &mut impl LifecycleBackend,
        memory: &mut impl LifecycleMemory,
        globals: &mut AudioGlobals,
    ) -> Result<(), LifecycleError> {
        let ids = self.word(0x124);
        if ids != 0 {
            memory.free_heap(ids)?;
        }
        self.put(0x124, 0);
        let names = self.word(0x74);
        if names != 0 {
            Self::destroy_string_table(memory, names)?;
        }
        self.put(0x74, 0);
        for offset in [0x128, 0x78] {
            let token = self.word(offset);
            if token != 0 {
                memory.free_heap(token)?;
            }
            self.put(offset, 0);
        }
        self.put(0x134, 0);
        self.image[0x1b4..0x1b6].fill(0);
        if self.enabled() {
            for i in 0..SAMPLES {
                let offset = 0x18 + 4 * i;
                backend.end_sample(self.word(offset));
                backend.release_sample(self.word(offset));
                self.put(offset, 0);
            }
            for i in 0..SAMPLES {
                backend.free_locked_buffer(self.word(0xd0 + 4 * i));
            }
            for offset in [8, 12, 16, 0, 4, 20] {
                if self.word(offset) != 0 {
                    self.close(backend, offset);
                }
            }
            backend.wait_shutdown(1000);
            if globals.service_object != 0 {
                backend.destroy_service_object(globals.service_object, 1);
            }
            globals.service_flag = 0;
            backend.close_audio_driver(self.word(0x68));
            self.put(0x68, 0);
        }
        self.put(0x130, 0);
        for i in 0..(self.word(0x138) as i32).max(0) as u32 {
            let address = self.word(0x12c).wrapping_add(i.wrapping_mul(4));
            let token = memory
                .read_word(address)
                .ok_or(LifecycleError::MissingWord { address })?;
            memory.free_heap(token)?;
        }
        self.put(0x138, self.word(0x138).wrapping_add(1));
        memory.free_heap(self.word(0x12c))?;
        self.image[0x1c7] = 0;
        self.image[0x1c6] = 0;
        let titles = self.word(0xcc);
        if titles != 0 {
            Self::destroy_string_table(memory, titles)?;
        }
        self.put(0xcc, 0);
        Ok(())
    }

    /// 0x00441AB0. El apagado global se solicita incluso si no había driver.
    pub fn destroy(
        &mut self,
        backend: &mut impl LifecycleBackend,
        memory: &mut impl LifecycleMemory,
        globals: &mut AudioGlobals,
    ) -> Result<(), LifecycleError> {
        self.shutdown(backend, memory, globals)?;
        backend.shutdown_audio_library();
        Ok(())
    }
}
