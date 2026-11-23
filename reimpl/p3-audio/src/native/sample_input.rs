//! Callback de entrada ASI de Miles, 0x211119c0, con dos búferes de sample.
//! Mantiene separadas la falta temporal de entrada y una marca de búfer terminado.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SampleInputBuffer {
    pub data: Vec<u8>,
    pub position: usize,
    pub done: bool,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SampleInputError {
    InvalidActive,
    PositionOutOfRange,
    OffsetOutOfRange,
    Capacity,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SampleInput {
    pub buffers: [SampleInputBuffer; 2],
    pub active: usize,
    pub last_ready: u32,
    pub buffer_mode: u32,
    pub starved: bool,
    pub end_callback_present: bool,
    pub queued_end_callbacks: u32,
}
impl Default for SampleInput {
    fn default() -> Self {
        Self {
            buffers: std::array::from_fn(|_| SampleInputBuffer::default()),
            active: 0,
            last_ready: u32::MAX - 1,
            buffer_mode: 2,
            starved: false,
            end_callback_present: false,
            queued_end_callbacks: 0,
        }
    }
}
impl SampleInput {
    /// Selección de búfer libre de `0x21114180`.
    pub fn buffer_ready(&mut self) -> Result<Option<usize>, SampleInputError> {
        if self.active > 1 {
            return Err(SampleInputError::InvalidActive);
        }
        if self.last_ready == u32::MAX - 1 {
            self.buffers[1].done = false;
            self.last_ready = u32::MAX;
            return Ok(Some(0));
        }
        if self.last_ready == u32::MAX {
            self.last_ready = self.active as u32;
            return Ok(Some(1));
        }
        if self.last_ready == self.active as u32 {
            return Ok(None);
        }
        self.last_ready = self.active as u32;
        Ok(Some(1 - self.active))
    }
    /// Copia hasta dos búferes. `offset` cambia el cursor del búfer activo, no
    /// es un desplazamiento absoluto del archivo. Si falta el segundo, conserva
    /// la salida parcial y marca `starved` como el callback x86.
    pub fn fetch(&mut self, output: &mut [u8], offset: Option<usize>) -> Result<usize, SampleInputError> {
        if self.active > 1 {
            return Err(SampleInputError::InvalidActive);
        }
        if self.buffers.iter().any(|b| b.position > b.data.len()) {
            return Err(SampleInputError::PositionOutOfRange);
        }
        if let Some(offset) = offset {
            if offset > self.buffers[self.active].data.len() {
                return Err(SampleInputError::OffsetOutOfRange);
            }
            self.buffers[self.active].position = offset;
        }
        let current = &mut self.buffers[self.active];
        let first = output.len().min(current.data.len() - current.position);
        output[..first].copy_from_slice(&current.data[current.position..current.position + first]);
        current.position += first;
        if first == output.len() {
            return Ok(first);
        }
        if self.end_callback_present {
            self.queued_end_callbacks = self.queued_end_callbacks.wrapping_add(1);
        }
        if self.buffer_mode != 2 {
            return Ok(first);
        }
        let next = 1 - self.active;
        if self.buffers[next].done {
            return Ok(first);
        }
        if self.buffers[next].data.is_empty() || self.buffers[next].position != 0 {
            self.starved = true;
            return Ok(first);
        }
        self.active = next;
        let b = &mut self.buffers[next];
        let count = (output.len() - first).min(b.data.len());
        output[first..first + count].copy_from_slice(&b.data[..count]);
        b.position = count;
        Ok(first + count)
    }
    /// Copia la entrada para que el productor pueda reutilizar su búfer HSTREAM.
    pub fn load(&mut self, index: usize, data: &[u8]) -> Result<(), SampleInputError> {
        if index > 1 {
            return Err(SampleInputError::InvalidActive);
        }
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(data.len())
            .map_err(|_| SampleInputError::Capacity)?;
        owned.extend_from_slice(data);
        self.buffers[index] = SampleInputBuffer {
            data: owned,
            position: 0,
            done: data.is_empty(),
        };
        self.starved = false;
        Ok(())
    }
}
