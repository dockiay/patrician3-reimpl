//! Adaptador no bloqueante entre el productor de archivos y el servicio HSTREAM.
//! Reanuda una operación de servicio si falta temporalmente una lectura o seek.
use super::async_input::{AsyncInput, InputError, InputState};
use super::hstream_buffers::{BufferService, ServiceError};
use super::mp3_decoder::ProgressiveMp3;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AsyncServiceError {
    Service(ServiceError),
    Input(InputError),
}
impl From<ServiceError> for AsyncServiceError {
    fn from(error: ServiceError) -> Self {
        Self::Service(error)
    }
}
impl From<InputError> for AsyncServiceError {
    fn from(error: InputError) -> Self {
        Self::Input(error)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServicePoll {
    Pending,
    Complete(u32),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FillPhase {
    Start,
    Reading,
    Boundary,
}

/// Estado de una llamada a service, persistente entre esperas del productor.
pub struct AsyncHstream {
    pub buffers: BufferService,
    pub discontinuities: u64,
    pub lock_contentions: u64,
    active: bool,
    stages: usize,
    stage: usize,
    written: u32,
    fill_all: bool,
    phase: FillPhase,
    requested: usize,
    obtained: usize,
    iterations: u8,
}
impl AsyncHstream {
    pub fn new(buffers: BufferService) -> Result<Self, AsyncServiceError> {
        if !buffers.valid() || buffers.loop_padding != 0 || buffers.end_padding != 0 || buffers.file_base != 0 {
            return Err(ServiceError::InvalidConfiguration.into());
        }
        Ok(Self {
            buffers,
            discontinuities: 0,
            lock_contentions: 0,
            active: false,
            stages: 0,
            stage: 0,
            written: 0,
            fill_all: false,
            phase: FillPhase::Start,
            requested: 0,
            obtained: 0,
            iterations: 0,
        })
    }
    pub fn in_progress(&self) -> bool {
        self.active
    }
    /// Abandona una llamada en espera tras un seek de control; el llamante
    /// invalida también los buffers y la generación de la fuente.
    pub fn cancel(&mut self) {
        self.active = false;
        self.phase = FillPhase::Start;
        self.iterations = 0;
    }
    /// Intenta completar una llamada a HSTREAM sin esperar E/S ni cerrojos.
    /// Se llama de nuevo al recibir más datos; no se debe despachar hasta Complete.
    pub fn poll(
        &mut self,
        source: &AsyncInput,
        decoder: &mut ProgressiveMp3,
        info: [u32; 4],
        fill_all: bool,
    ) -> Result<ServicePoll, AsyncServiceError> {
        let b = &mut self.buffers;
        if !b.valid() || b.loop_padding != 0 || b.end_padding != 0 || b.file_base != 0 {
            return Err(ServiceError::InvalidState.into());
        }
        if !self.active {
            if b.error || b.end_state != 0 {
                return Err(ServiceError::SourceError.into());
            }
            let second = info[0] == info[1] || info[2] == info[3];
            let third = info[0] == info[1] && info[2] == info[3];
            if second {
                b.budget = b.budget_default;
            }
            self.stages = 1 + usize::from(second) + usize::from(third);
            self.stage = 0;
            self.written = 0;
            self.fill_all = fill_all;
            self.active = true;
            self.phase = FillPhase::Start;
            self.iterations = 0;
        }
        loop {
            let index = b.order[self.stage];
            match self.phase {
                FillPhase::Start => {
                    self.iterations += 1;
                    if self.iterations > 2 {
                        return Err(ServiceError::InvalidState.into());
                    }
                    let capacity = b.buffers[index].bytes.len();
                    let used = b.buffers[index].used;
                    let remaining = (b.total - b.file_position).min(b.loop_end - b.file_position) as usize;
                    self.requested = remaining.min(b.chunk_size).min(capacity - used);
                    self.obtained = 0;
                    self.phase = if self.requested == 0 {
                        FillPhase::Boundary
                    } else {
                        FillPhase::Reading
                    };
                }
                FillPhase::Reading => {
                    let used = b.buffers[index].used;
                    let r =
                        source.try_read(&mut b.buffers[index].bytes[used..used + self.requested - self.obtained])?;
                    b.file_position = b
                        .file_position
                        .checked_add(r.bytes as u32)
                        .ok_or(ServiceError::InvalidState)?;
                    b.buffers[index].used += r.bytes;
                    self.obtained += r.bytes;
                    if self.obtained < self.requested {
                        if r.state == InputState::Contended {
                            self.lock_contentions = self.lock_contentions.saturating_add(1);
                        }
                        if r.state == InputState::End {
                            b.error = true;
                            return Err(ServiceError::SourceError.into());
                        }
                        return Ok(ServicePoll::Pending);
                    }
                    self.phase = FillPhase::Boundary;
                }
                FillPhase::Boundary => {
                    if b.file_position == b.loop_end {
                        if b.loops == 1 {
                            b.loop_end = i32::MAX as u32;
                            self.phase = FillPhase::Start;
                            continue;
                        }
                        if source.try_seek(u64::from(b.loop_start))?.is_none() {
                            self.lock_contentions = self.lock_contentions.saturating_add(1);
                            return Ok(ServicePoll::Pending);
                        }
                        if b.loops != 0 {
                            b.loops -= 1;
                        }
                        b.file_position = b.loop_start;
                        b.logical_position = b.loop_start;
                        b.pad_count = u32::MAX;
                        if b.provider_open {
                            decoder.discontinuity().map_err(ServiceError::Provider)?;
                            self.discontinuities = self.discontinuities.saturating_add(1);
                        }
                    }
                    if b.file_position == b.total {
                        if b.loops == 1 {
                            b.end_state = 1;
                        } else {
                            if source.try_seek(0)?.is_none() {
                                self.lock_contentions = self.lock_contentions.saturating_add(1);
                                return Ok(ServicePoll::Pending);
                            }
                            if b.loops != 0 {
                                b.loops -= 1;
                            }
                            b.file_position = 0;
                            b.pad_count = u32::MAX;
                            if b.provider_open {
                                decoder.discontinuity().map_err(ServiceError::Provider)?;
                                self.discontinuities = self.discontinuities.saturating_add(1);
                            }
                        }
                    }
                    let amount = self.obtained as u32;
                    self.written = self.written.checked_add(amount).ok_or(ServiceError::InvalidState)?;
                    self.phase = FillPhase::Start;
                    self.iterations = 0;
                    if amount == 0 || (!self.fill_all && self.written >= b.budget) {
                        self.stage += 1;
                        if self.stage == self.stages {
                            b.budget = 0;
                            self.active = false;
                            return Ok(ServicePoll::Complete(self.written));
                        }
                    }
                }
            }
        }
    }
}
