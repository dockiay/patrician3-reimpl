//! Transición acotada de seek temporal para HSTREAM con proveedor MP3 abierto.
//! No implementa el servicio de búferes ni un dispositivo de sonido.
use super::mp3_decoder::{ByteStream, DecodeError};
use super::timing::{MilesTiming, TimingError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SeekError {
    Timing(TimingError),
    Decoder(DecodeError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SeekResult {
    pub requested_byte: u32,
    pub aligned_byte: u32,
}

/// Acciones externas de `0x21122760`, en el orden observado.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartAction {
    StartSample,
    EndSample,
    SeekZero,
    Service,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StartControl {
    pub direct: bool,
    pub started: bool,
    pub pending: u32,
    pub control: u32,
}
impl StartControl {
    pub fn start(&mut self) -> Vec<StartAction> {
        if self.pending == 1 {
            self.pending = 2;
        }
        let mut actions = Vec::new();
        if self.direct {
            if self.started {
                actions.push(StartAction::StartSample);
            }
        } else {
            if self.started {
                actions.extend([StartAction::EndSample, StartAction::SeekZero]);
            }
            actions.push(StartAction::Service);
        }
        self.control = 1;
        actions
    }
}

/// Llamadas externas de la transición; las ejecuta el futuro servicio de audio.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PauseAction {
    QuerySampleStatus,
    EndSample,
    ResumeSample,
    Service,
    StartStream,
}

/// Estado preparado sin error en HSTREAM+0x8c. No abre ni alimenta un sample.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PauseControl {
    pub control: u32,
    pub started: bool,
}
impl PauseControl {
    pub fn set_paused(&mut self, pause: bool, sample_status: u32) -> Vec<PauseAction> {
        let mut actions = Vec::new();
        if pause {
            let status = if self.control == 1 && !self.started {
                4
            } else {
                actions.push(PauseAction::QuerySampleStatus);
                sample_status
            };
            if status == 4 {
                actions.push(PauseAction::EndSample);
                self.control |= 0x18;
            } else {
                self.control |= 8;
            }
        } else if self.control & 7 == 0 {
            actions.push(PauseAction::StartStream);
        } else {
            if self.control & 0x10 != 0 {
                actions.push(PauseAction::ResumeSample);
            }
            self.control &= !0x18;
            actions.push(PauseAction::Service);
        }
        actions
    }
}

/// Une la conversión de HSTREAM con el lector ASI en memoria completa.
/// El origen y el proveedor se representan juntos; no modela ReadFile ni sample.
pub struct Mp3SeekBridge<'a> {
    timing: MilesTiming,
    total: u32,
    stream: ByteStream<'a>,
}
impl<'a> Mp3SeekBridge<'a> {
    pub fn new(input: &'a [u8], timing: MilesTiming) -> Result<Self, SeekError> {
        let total = u32::try_from(input.len()).map_err(|_| SeekError::Timing(TimingError::Range))?;
        let stream = ByteStream::new(input).map_err(SeekError::Decoder)?;
        let expected_flags = if stream.channels == 2 { 3 } else { 1 };
        if timing.format_flags != expected_flags || !matches!(timing.codec_byte_rate, Some(1..)) {
            return Err(SeekError::Timing(TimingError::Format));
        }
        Ok(Self { timing, total, stream })
    }
    pub fn seek_ms(&mut self, ms: u32) -> Result<SeekResult, SeekError> {
        let requested_byte = self.timing.seek_bytes(ms).map_err(SeekError::Timing)?;
        let aligned_byte = self
            .timing
            .align_with_provider(requested_byte, self.total, true)
            .map_err(SeekError::Timing)?;
        self.stream.seek_input(u32::MAX).map_err(SeekError::Decoder)?;
        self.stream.seek_input(aligned_byte).map_err(SeekError::Decoder)?;
        Ok(SeekResult {
            requested_byte,
            aligned_byte,
        })
    }
    pub fn read(&mut self, output: &mut [u8]) -> Result<usize, DecodeError> {
        self.stream.read(output)
    }
}
