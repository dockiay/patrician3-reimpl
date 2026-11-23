//! Servicio aislado de buffers HSTREAM (`0x21122020` y `0x21122260`).
//! El archivo y el proveedor se suministran por puertos; no hay dispositivo.

/// Puertos que el servicio llama de manera síncrona. El puntero de archivo es
/// propio del puerto; `provider_discontinuity` corresponde al seek ASI -2.
pub trait StreamIo {
    fn read(&mut self, output: &mut [u8]) -> usize;
    fn seek(&mut self, offset: u32);
    fn provider_discontinuity(&mut self) -> Result<(), ServiceError>;
}

/// Archivo completo y proveedor MP3 nativo: enlaza el aviso ASI -2.
/// La entrega de buffers comprimidos al decoder sigue siendo una frontera aparte;
/// este adaptador no acredita la decodificación progresiva.
pub struct MemoryMp3Io<'a, 'b> {
    pub input: &'a [u8],
    pub position: usize,
    pub decoder: &'b mut super::mp3_decoder::ByteStream<'a>,
}
impl StreamIo for MemoryMp3Io<'_, '_> {
    fn read(&mut self, output: &mut [u8]) -> usize {
        if self.position >= self.input.len() {
            return 0;
        }
        let n = output.len().min(self.input.len().saturating_sub(self.position));
        output[..n].copy_from_slice(&self.input[self.position..self.position + n]);
        self.position += n;
        n
    }
    fn seek(&mut self, offset: u32) {
        self.position = offset as usize;
    }
    fn provider_discontinuity(&mut self) -> Result<(), ServiceError> {
        self.decoder.discontinuity().map_err(ServiceError::Provider)
    }
}

/// Frontera del sample: el buffer se entrega antes de reutilizar su memoria.
pub trait SampleSink {
    fn buffer_ready(&mut self) -> Option<u32>;
    fn load_buffer(&mut self, slot: u32, compressed: &[u8]);
}

/// Operaciones de sample/arranque que no implementa este servicio aislado.
pub trait PauseSample: SampleSink {
    fn status(&mut self) -> u32;
    fn end_sample(&mut self);
    fn resume_sample(&mut self);
    fn start_stream(&mut self);
}

/// Control para una única fuente normal; el arranque se mantiene como puerto.
pub struct StreamController {
    pub buffers: BufferService,
    pub pause: super::hstream::PauseControl,
}
impl StreamController {
    pub fn set_paused<T: PauseSample>(&mut self, sample: &mut T, paused: bool) -> Result<(), ServiceError> {
        use super::hstream::PauseAction;
        if self.buffers.error {
            return Err(ServiceError::SourceError);
        }
        self.pause.started = self.buffers.started;
        let status = if paused && !(self.pause.control == 1 && !self.pause.started) {
            sample.status()
        } else {
            0
        };
        for action in self.pause.set_paused(paused, status) {
            match action {
                PauseAction::QuerySampleStatus => {}
                PauseAction::EndSample => sample.end_sample(),
                PauseAction::ResumeSample => sample.resume_sample(),
                PauseAction::StartStream => sample.start_stream(),
                PauseAction::Service => {
                    if self.buffers.dispatch(sample, self.pause.control)?.is_some() {
                        self.buffers.dispatch(sample, self.pause.control)?;
                    }
                }
            }
        }
        self.pause.started = self.buffers.started;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServiceError {
    InvalidConfiguration,
    InvalidState,
    SourceError,
    Provider(super::mp3_decoder::DecodeError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamBuffer {
    pub bytes: Vec<u8>,
    pub used: usize,
}

/// Subdominio seguro: fuente en memoria de tamaño conocido, índices válidos,
/// puntos de bucle dentro del archivo y una única llamada de servicio activa.
pub struct BufferService {
    pub buffers: [StreamBuffer; 3],
    pub order: [usize; 3],
    pub chunk_size: usize,
    pub total: u32,
    pub loop_start: u32,
    pub loop_end: u32,
    pub loop_padding: u32,
    pub end_padding: u32,
    pub pad_count: u32,
    pub logical_position: u32,
    /// Offset inicial de los datos dentro del archivo físico.
    pub file_base: u32,
    pub file_position: u32,
    pub loops: u32,
    pub error: bool,
    pub end_state: u32,
    pub started: bool,
    pub budget_default: u32,
    pub budget: u32,
    pub codec_format: bool,
    pub provider_open: bool,
}
impl BufferService {
    pub fn new(total: u32, capacity: usize, chunk_size: usize) -> Result<Self, ServiceError> {
        if capacity == 0
            || capacity > i32::MAX as usize / 3
            || chunk_size == 0
            || chunk_size > i32::MAX as usize
            || total == 0
            || total >= i32::MAX as u32
        {
            return Err(ServiceError::InvalidConfiguration);
        }
        Ok(Self {
            buffers: std::array::from_fn(|_| StreamBuffer {
                bytes: vec![0; capacity],
                used: 0,
            }),
            order: [0, 1, 2],
            chunk_size,
            total,
            loop_start: 0,
            loop_end: total,
            loop_padding: 0,
            end_padding: 0,
            pad_count: u32::MAX,
            logical_position: 0,
            file_base: 0,
            file_position: 0,
            loops: 1,
            error: false,
            end_state: 0,
            started: false,
            budget_default: 0,
            budget: 0,
            codec_format: false,
            provider_open: false,
        })
    }
    pub(crate) fn valid(&self) -> bool {
        let capacity = self.buffers[0].bytes.len();
        capacity > 0
            && capacity <= i32::MAX as usize / 3
            && self.chunk_size > 0
            && self.chunk_size <= i32::MAX as usize
            && self.total > 0
            && self.total < i32::MAX as u32
            && self.loop_start < self.total
            && self.loop_end <= i32::MAX as u32
            && (self.loop_end > self.total || self.loop_start < self.loop_end)
            && self.file_position <= self.total
            && self.file_position <= self.loop_end
            && self.file_base.checked_add(self.total).is_some()
            && self.budget <= i32::MAX as u32
            && self.budget_default <= i32::MAX as u32
            && self.order.iter().all(|&i| i < 3)
            && self.order[0] != self.order[1]
            && self.order[1] != self.order[2]
            && self.order[0] != self.order[2]
            && self.order[1] == (self.order[0] + 1) % 3
            && self.order[2] == (self.order[1] + 1) % 3
            && self.end_state <= 2
            && self
                .buffers
                .iter()
                .all(|b| b.bytes.len() == capacity && b.used <= capacity)
    }
    fn fill<T: StreamIo>(&mut self, io: &mut T, index: usize) -> Result<usize, ServiceError> {
        let mut iterations = 0;
        loop {
            iterations += 1;
            if iterations > 2 {
                return Err(ServiceError::InvalidState);
            }
            let capacity = self.buffers[index].bytes.len();
            let used = self.buffers[index].used;
            let remaining = (self.total - self.file_position).min(self.loop_end - self.file_position) as usize;
            let requested = remaining.min(self.chunk_size).min(capacity - used);
            let got = if requested == 0 {
                0
            } else {
                io.read(&mut self.buffers[index].bytes[used..used + requested])
            };
            if got > requested {
                return Err(ServiceError::InvalidState);
            }
            self.file_position += got as u32;
            self.buffers[index].used += got;
            if got != requested {
                self.error = true;
            }
            let mut amount = got;
            if self.file_position == self.loop_end {
                if self.loops == 1 {
                    self.loop_end = i32::MAX as u32;
                    continue;
                }
                if self.codec_format {
                    if self.pad_count == u32::MAX {
                        self.pad_count = 0;
                    }
                    if self.pad_count > self.loop_padding {
                        return Err(ServiceError::InvalidState);
                    }
                    let n = ((self.loop_padding - self.pad_count) as usize).min(capacity - self.buffers[index].used);
                    let start = self.buffers[index].used;
                    self.buffers[index].bytes[start..start + n].fill(0);
                    self.buffers[index].used += n;
                    self.pad_count += n as u32;
                    amount += n;
                    if self.pad_count != self.loop_padding {
                        return Ok(amount);
                    }
                }
                if self.loops != 0 {
                    self.loops -= 1;
                }
                self.file_position = self.loop_start;
                self.logical_position = self.loop_start;
                self.pad_count = u32::MAX;
                io.seek(self.file_base + self.loop_start);
                if self.provider_open {
                    io.provider_discontinuity()?;
                }
            }
            if self.file_position == self.total {
                if self.loops == 1 {
                    self.end_state = 1;
                    return Ok(amount);
                }
                if self.codec_format {
                    if self.pad_count == u32::MAX {
                        self.pad_count = 0;
                    }
                    if self.pad_count > self.end_padding {
                        return Err(ServiceError::InvalidState);
                    }
                    let n = ((self.end_padding - self.pad_count) as usize).min(capacity - self.buffers[index].used);
                    let start = self.buffers[index].used;
                    self.buffers[index].bytes[start..start + n].fill(0);
                    self.buffers[index].used += n;
                    self.pad_count += n as u32;
                    amount += n;
                    if self.pad_count != self.end_padding {
                        return Ok(amount);
                    }
                }
                if self.loops != 0 {
                    self.loops -= 1;
                }
                self.file_position = 0;
                self.pad_count = u32::MAX;
                io.seek(self.file_base);
                if self.provider_open {
                    io.provider_discontinuity()?;
                }
            }
            return Ok(amount);
        }
    }
    /// `sample_info` son los cuatro valores consultados al sample. `fill_all`
    /// omite el presupuesto, como el segundo argumento no nulo del original.
    pub fn service<T: StreamIo>(
        &mut self,
        io: &mut T,
        sample_info: [u32; 4],
        fill_all: bool,
    ) -> Result<u32, ServiceError> {
        if !self.valid() {
            return Err(ServiceError::InvalidState);
        }
        if self.error || self.end_state != 0 {
            return Err(ServiceError::SourceError);
        }
        let second = sample_info[0] == sample_info[1] || sample_info[2] == sample_info[3];
        let third = sample_info[0] == sample_info[1] && sample_info[2] == sample_info[3];
        if second {
            self.budget = self.budget_default;
        }
        let mut written = 0u32;
        let stages = 1 + usize::from(second) + usize::from(third);
        for slot in 0..stages {
            loop {
                let n = self.fill(io, self.order[slot])? as u32;
                if self.error {
                    return Err(ServiceError::SourceError);
                }
                written = written.checked_add(n).ok_or(ServiceError::InvalidState)?;
                if n == 0 || (!fill_all && written >= self.budget) {
                    break;
                }
            }
        }
        self.budget = 0;
        Ok(written)
    }

    /// `0x21121380` para una fuente de archivo normal y sin callback de fin.
    /// El sample conserva o copia los bytes durante `load_buffer`.
    pub fn dispatch<T: SampleSink>(&mut self, sample: &mut T, control: u32) -> Result<Option<u32>, ServiceError> {
        if !self.valid() {
            return Err(ServiceError::InvalidState);
        }
        let index = self.order[0];
        let used = self.buffers[index].used;
        if (used == 0 && self.end_state != 1) || control != 1 {
            return Ok(None);
        }
        let Some(slot) = sample.buffer_ready() else {
            return Ok(None);
        };
        sample.load_buffer(slot, &self.buffers[index].bytes[..used]);
        self.logical_position = self.logical_position.wrapping_add(used as u32);
        self.started = true;
        if self.end_state == 1 && used == 0 {
            self.end_state = 2;
        }
        self.buffers[index].used = 0;
        self.order = [self.order[1], self.order[2], (self.order[2] + 1) % 3];
        Ok(Some(slot))
    }
}

/// Sample simulado que entrega los bytes comprimidos al decoder progresivo.
/// Un búfer vacío señala EOF; no representa una salida de sonido.
pub struct ProgressiveSample {
    pub decoder: super::mp3_decoder::ProgressiveMp3,
    pub max_buffer: usize,
    pub input_error: Option<super::mp3_decoder::DecodeError>,
}
impl ProgressiveSample {
    pub fn new(limit: usize, max_buffer: usize) -> Result<Self, super::mp3_decoder::DecodeError> {
        if max_buffer == 0 || max_buffer > limit {
            return Err(super::mp3_decoder::DecodeError::InvalidLimit);
        }
        Ok(Self {
            decoder: super::mp3_decoder::ProgressiveMp3::new(limit)?,
            max_buffer,
            input_error: None,
        })
    }
    pub fn read(
        &mut self,
        output: &mut [u8],
    ) -> Result<super::mp3_decoder::ProgressiveRead, super::mp3_decoder::DecodeError> {
        if let Some(error) = self.input_error.clone() {
            return Err(error);
        }
        let mut result = self.decoder.read(output)?;
        if result.status == super::mp3_decoder::ProgressiveStatus::NeedInput
            && self.decoder.remaining_capacity() < self.max_buffer
        {
            let error = super::mp3_decoder::DecodeError::InvalidLimit;
            self.input_error = Some(error.clone());
            if result.written == 0 {
                return Err(error);
            }
            result.status = super::mp3_decoder::ProgressiveStatus::ErrorPending;
        }
        Ok(result)
    }
}
impl SampleSink for ProgressiveSample {
    fn buffer_ready(&mut self) -> Option<u32> {
        (self.input_error.is_none() && self.decoder.remaining_capacity() >= self.max_buffer).then_some(0)
    }
    fn load_buffer(&mut self, _slot: u32, bytes: &[u8]) {
        if bytes.is_empty() {
            self.decoder.finish();
            return;
        }
        if bytes.len() > self.max_buffer {
            self.input_error = Some(super::mp3_decoder::DecodeError::InvalidLimit);
            return;
        }
        if let Err(e) = self.decoder.push(bytes) {
            self.input_error = Some(e);
        }
    }
}

/// Archivo en memoria y aviso de discontinuidad al decoder alimentado por sample.
pub struct ProgressiveFileIo<'a, 'b> {
    pub input: &'a [u8],
    pub position: usize,
    pub decoder: &'b mut super::mp3_decoder::ProgressiveMp3,
}
impl StreamIo for ProgressiveFileIo<'_, '_> {
    fn read(&mut self, output: &mut [u8]) -> usize {
        if self.position >= self.input.len() {
            return 0;
        }
        let n = output.len().min(self.input.len() - self.position);
        output[..n].copy_from_slice(&self.input[self.position..self.position + n]);
        self.position += n;
        n
    }
    fn seek(&mut self, offset: u32) {
        self.position = offset as usize;
    }
    fn provider_discontinuity(&mut self) -> Result<(), ServiceError> {
        self.decoder.discontinuity().map_err(ServiceError::Provider)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProgressiveSeekError {
    Timing(super::timing::TimingError),
    Provider(super::mp3_decoder::DecodeError),
    InvalidState,
}
impl StreamController {
    /// Seek temporal de una fuente normal con proveedor abierto.
    /// El cursor físico lo mantiene el puerto de archivo; la muestra y sus tres
    /// buffers se vacían como en la rama comprobada de HSTREAM.
    pub fn seek_progressive_ms(
        &mut self,
        sample: &mut ProgressiveSample,
        file_position: &mut usize,
        timing: super::timing::MilesTiming,
        ms: u32,
    ) -> Result<super::hstream::SeekResult, ProgressiveSeekError> {
        if !self.buffers.valid() || self.buffers.error || !self.buffers.provider_open || sample.input_error.is_some() {
            return Err(ProgressiveSeekError::InvalidState);
        }
        if !matches!(timing.format_flags, 1 | 3)
            || !matches!(timing.codec_byte_rate, Some(1..))
            || sample
                .decoder
                .format()
                .is_some_and(|(_, channels)| timing.format_flags != if channels == 2 { 3 } else { 1 })
        {
            return Err(ProgressiveSeekError::Timing(super::timing::TimingError::Format));
        }
        let requested_byte = timing.seek_bytes(ms).map_err(ProgressiveSeekError::Timing)?;
        let aligned_byte = timing
            .align_with_provider(requested_byte, self.buffers.total, true)
            .map_err(ProgressiveSeekError::Timing)?;
        if aligned_byte > self.buffers.loop_end {
            return Err(ProgressiveSeekError::InvalidState);
        }
        let physical = self
            .buffers
            .file_base
            .checked_add(aligned_byte)
            .ok_or(ProgressiveSeekError::InvalidState)?;
        sample
            .decoder
            .reset_input(aligned_byte == 0)
            .map_err(ProgressiveSeekError::Provider)?;
        *file_position = physical as usize;
        self.buffers.pad_count = u32::MAX;
        self.buffers.logical_position = aligned_byte;
        self.buffers.file_position = aligned_byte;
        self.buffers.started = false;
        self.buffers.order = [0, 1, 2];
        for buffer in &mut self.buffers.buffers {
            buffer.used = 0;
        }
        self.buffers.end_state = 0;
        self.buffers.budget = self.buffers.budget_default;
        if self.pause.control != 1 {
            self.pause.control = 0;
        }
        self.pause.started = false;
        Ok(super::hstream::SeekResult {
            requested_byte,
            aligned_byte,
        })
    }
}
