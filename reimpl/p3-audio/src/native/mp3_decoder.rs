//! MP3 Layer III escalar: tramas, depósito y etapas nativas hasta PCM16.
use super::mp3_band::{self, BandError, BandParams};
use super::mp3_frame::{ChannelInfo, Frame, FrameError};
use super::mp3_huffman::{self, HuffmanError, HuffmanParams};
use super::mp3_hybrid::{self, HybridChannel, HybridError};
use super::mp3_polyphase::{PolyphaseError, PolyphaseState};
use super::mp3_scales::{self, ScaleError, ScaleParams};
use super::mp3_stereo::{self, StereoError, StereoParams};
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeError {
    Frame(FrameError),
    Scale(ScaleError),
    Huffman(HuffmanError),
    Band(BandError),
    Stereo(StereoError),
    Hybrid(HybridError),
    Polyphase(PolyphaseError),
    InvalidRegions,
    UnsupportedMixedMono,
    InvalidLimit,
    UnsupportedSeek,
}

/// PCM de un prefijo solicitado; `next_offset` sigue a la última trama leída.
/// Si el límite corta una trama, el resto de sus muestras no se conserva.
#[derive(Debug, Clone)]
pub struct PcmPrefix {
    pub sample_rate: u32,
    pub channels: usize,
    pub samples: Vec<i16>,
    pub next_offset: usize,
    pub last_frame_incomplete: bool,
}

/// Descifra desde el archivo hasta el número de fotogramas PCM solicitado.
/// Permite una última trama incompleta si conserva todos los bits consumidos.
/// No implementa seek ni lectura progresiva.
pub fn decode_prefix(input: &[u8], frames: usize) -> Result<PcmPrefix, DecodeError> {
    if frames == 0 {
        return Err(DecodeError::InvalidLimit);
    }
    let mut offset = super::mp3_frame::audio_start(input)?;
    let first = super::mp3_frame::parse_final_frame(input, offset)?;
    let channels = first.channels;
    let rate = first.sample_rate;
    let count = frames.checked_mul(channels).ok_or(DecodeError::InvalidLimit)?;
    let mut decoder = Decoder::default();
    let mut samples = Vec::new();
    let mut last_frame_incomplete = false;
    while samples.len() < count {
        let frame = super::mp3_frame::parse_final_frame(input, offset)?;
        let out = decoder.decode_frame(&frame)?;
        let take = (count - samples.len()).min(out.len());
        samples.extend_from_slice(&out[..take]);
        offset = offset
            .checked_add(frame.available_length)
            .ok_or(DecodeError::InvalidLimit)?;
        last_frame_incomplete = frame.available_length != frame.length;
    }
    Ok(PcmPrefix {
        sample_rate: rate,
        channels,
        samples,
        next_offset: offset,
        last_frame_incomplete,
    })
}
impl From<FrameError> for DecodeError {
    fn from(e: FrameError) -> Self {
        Self::Frame(e)
    }
}
impl From<ScaleError> for DecodeError {
    fn from(e: ScaleError) -> Self {
        Self::Scale(e)
    }
}
impl From<HuffmanError> for DecodeError {
    fn from(e: HuffmanError) -> Self {
        Self::Huffman(e)
    }
}
impl From<BandError> for DecodeError {
    fn from(e: BandError) -> Self {
        Self::Band(e)
    }
}
impl From<StereoError> for DecodeError {
    fn from(e: StereoError) -> Self {
        Self::Stereo(e)
    }
}
impl From<HybridError> for DecodeError {
    fn from(e: HybridError) -> Self {
        Self::Hybrid(e)
    }
}
impl From<PolyphaseError> for DecodeError {
    fn from(e: PolyphaseError) -> Self {
        Self::Polyphase(e)
    }
}
#[derive(Clone)]
pub struct Decoder {
    reservoir: Vec<u8>,
    format: Option<(u8, u8, usize)>,
    scales: [[[u32; 62]; 2]; 2],
    signs: [[u8; 576]; 2],
    overlap: [[u32; 576]; 2],
    poly: [PolyphaseState; 2],
}
impl Default for Decoder {
    fn default() -> Self {
        Self {
            reservoir: Vec::new(),
            format: None,
            scales: [[[0; 62]; 2]; 2],
            signs: [[0; 576]; 2],
            overlap: [[0; 576]; 2],
            poly: [
                PolyphaseState::new(0, false).unwrap(),
                PolyphaseState::new(0, false).unwrap(),
            ],
        }
    }
}
fn params(f: &Frame<'_>, c: ChannelInfo) -> BandParams {
    BandParams {
        version: f.version,
        sample_rate_index: f.rate_index,
        global_gain: c.global_gain,
        preflag: c.preflag,
        scale_mode: c.scale_mode,
        subblock_gain: c.subblock_gain,
        window_switch: c.window_switch,
        block_type: c.block_type,
        mixed: c.mixed,
    }
}
fn regions(f: &Frame<'_>, c: ChannelInfo) -> Result<[usize; 3], DecodeError> {
    let end = c.big_values * 2;
    let table = &mp3_band::BANDS[usize::from(f.version) * 3 + usize::from(f.rate_index)];
    if c.window_switch {
        let first = if c.block_type == 2 { 36 } else { usize::from(table[8]) };
        return Ok([end.min(first), end, end]);
    }
    let a = usize::from(c.region0) + 1;
    let b = a + usize::from(c.region1) + 1;
    if a >= 23 || b >= 23 {
        return Err(DecodeError::InvalidRegions);
    }
    Ok([end.min(usize::from(table[a])), end.min(usize::from(table[b])), end])
}
impl Decoder {
    /// Seek a byte cero de Mp3dec: conserva los bancos polifásicos y escalas,
    /// pero borra el solapamiento, la posición y la fase.
    pub fn rewind(&mut self) {
        self.reservoir.clear();
        self.reset_synthesis();
    }
    fn reset_synthesis(&mut self) {
        self.overlap = [[0; 576]; 2];
        for channel in &mut self.poly {
            channel.position = 0;
            channel.phase = false;
        }
    }
    /// Una trama completa. Un error conserva el estado previo del decoder.
    pub fn decode_frame(&mut self, f: &Frame<'_>) -> Result<Vec<i16>, DecodeError> {
        let mut next = self.clone();
        let out = next.decode_inner(f, false)?;
        *self = next;
        Ok(out)
    }
    fn decode_frame_zero_extended(&mut self, f: &Frame<'_>) -> Result<Vec<i16>, DecodeError> {
        let mut next = self.clone();
        let out = next.decode_inner(f, true)?;
        *self = next;
        Ok(out)
    }
    fn decode_inner(&mut self, f: &Frame<'_>, extend: bool) -> Result<Vec<i16>, DecodeError> {
        if f.version > 1
            || f.rate_index > 2
            || !(1..=2).contains(&f.channels)
            || f.granules != if f.version == 1 { 2 } else { 1 }
            || f.info.iter().flatten().any(|c| c.big_values > 288)
        {
            return Err(FrameError::InvalidSideInfo.into());
        }
        let format = (f.version, f.rate_index, f.channels);
        if self.format.is_some_and(|x| x != format) {
            return Err(FrameError::FormatChanged.into());
        }
        if f.main_data_begin > self.reservoir.len() {
            return Err(FrameError::ReservoirUnderflow.into());
        }
        let mut main = self.reservoir[self.reservoir.len() - f.main_data_begin..].to_vec();
        main.extend_from_slice(f.main_data);
        // La opción experimental limita las lecturas que el original deja
        // avanzar tras una lectura corta; el puerto ordinario conserva su guard.
        if extend {
            main.resize(main.len() + 4096, 0);
        }
        let valid_bits = main.len() * 8;
        // La lectura count1 original puede adelantarse al final declarado.
        main.extend_from_slice(&[0; 8]);
        let mut bit = 0usize;
        let mut pcm = Vec::with_capacity(f.granules * 576 * f.channels);
        for gr in 0..f.granules {
            let mut raw = [[0u32; 576]; 2];
            let mut used = [0usize; 2];
            let mut cfg = [params(f, ChannelInfo::default()); 2];
            for ch in 0..f.channels {
                let c = f.info[gr][ch];
                let end = bit.checked_add(c.part_bits).ok_or(FrameError::Truncated)?;
                if end > valid_bits {
                    return Err(FrameError::Truncated.into());
                }
                let sp = ScaleParams {
                    start_bit: bit,
                    end_bit: end,
                    granule: gr as u8,
                    compress: c.scale_compress,
                    window_switch: c.window_switch,
                    block_type: c.block_type,
                    mixed: c.mixed,
                    scfsi: f.scfsi[ch],
                };
                let prior = self.scales[gr][ch];
                let previous = self.scales[0][ch];
                let scales = if f.version == 1 {
                    mp3_scales::read_mpeg1(&main, sp, prior, &previous)?
                } else {
                    mp3_scales::read_mpeg2(&main, sp, prior, f.mode == 1 && f.extension & 1 != 0 && ch == 1)?
                };
                self.scales[gr][ch] = scales.values;
                let hp = HuffmanParams {
                    start_bit: scales.end_bit,
                    end_bit: end,
                    regions: regions(f, c)?,
                    tables: c.tables,
                    count1_table: c.count1_table,
                };
                let h = mp3_huffman::decode(&main, hp, self.signs[ch])?;
                if h.consumed_bit > valid_bits {
                    return Err(FrameError::Truncated.into());
                }
                self.signs[ch] = h.sign;
                used[ch] = h.used;
                cfg[ch] = params(f, c);
                raw[ch] = mp3_band::spectrum(cfg[ch], &scales.values, &h.magnitude, &h.sign, h.used)?;
                bit = end;
            }
            let mut reordered = if f.channels == 2 {
                mp3_stereo::transform(
                    StereoParams {
                        channels: cfg,
                        joint: f.mode == 1,
                        mid_side: f.extension & 2 != 0,
                        intensity: f.extension & 1 != 0,
                        scale_compress: f.info[gr][1].scale_compress,
                    },
                    &raw[0],
                    &raw[1],
                    &self.scales[gr][1],
                )?
            } else {
                if cfg[0].window_switch && cfg[0].block_type == 2 && cfg[0].mixed {
                    return Err(DecodeError::UnsupportedMixedMono);
                }
                let mut out = [[0; 576]; 2];
                mp3_band::reorder_mono(cfg[0], &raw[0], &mut out[0])
                    .map_err(|e| DecodeError::Stereo(StereoError::Reorder(e)))?;
                out
            };
            let conf = std::array::from_fn(|i| HybridChannel {
                window_switch: cfg[i].window_switch,
                block_type: cfg[i].block_type,
                mixed: cfg[i].mixed,
            });
            let active = mp3_hybrid::active_subbands(conf, used)?;
            let mut hybrid = [[0; 576]; 2];
            for ch in 0..f.channels {
                hybrid[ch] = mp3_hybrid::hybrid_channel(conf[ch], active, &mut reordered[ch], &mut self.overlap[ch])?;
            }
            for t in 0..18 {
                let mut channels = [[0i16; 32]; 2];
                for ch in 0..f.channels {
                    channels[ch] = self.poly[ch].step(&hybrid[ch], t)?;
                }
                for i in 0..32 {
                    for channel in channels.iter().take(f.channels) {
                        pcm.push(channel[i]);
                    }
                }
            }
        }
        self.reservoir.extend_from_slice(f.main_data);
        if self.reservoir.len() > 511 {
            self.reservoir.drain(..self.reservoir.len() - 511);
        }
        self.format = Some(format);
        Ok(pcm)
    }
}

/// Lector de bytes PCM16 con seek del proveedor. Conserva PCM entre llamadas.
/// La entrada está completa en memoria; no implementa HSTREAM ni entrada progresiva.
pub struct ByteStream<'a> {
    input: &'a [u8],
    start: usize,
    offset: usize,
    source_position: usize,
    decoder: Decoder,
    pending: Vec<u8>,
    cursor: usize,
    error: Option<DecodeError>,
    resync: bool,
    scan: bool,
    pub sample_rate: u32,
    pub channels: usize,
}
impl<'a> ByteStream<'a> {
    pub fn new(input: &'a [u8]) -> Result<Self, DecodeError> {
        let offset = super::mp3_frame::audio_start(input)?;
        let frame = super::mp3_frame::parse_final_frame(input, offset)?;
        Ok(Self {
            input,
            start: offset,
            offset,
            source_position: offset + frame.available_length - frame.main_data.len(),
            decoder: Decoder::default(),
            pending: Vec::new(),
            cursor: 0,
            error: None,
            resync: false,
            scan: false,
            sample_rate: frame.sample_rate,
            channels: frame.channels,
        })
    }
    /// Devuelve el cursor al inicio y aplica el reinicio parcial observado en x86.
    pub fn rewind(&mut self) {
        self.offset = self.start;
        self.decoder.rewind();
        self.pending.clear();
        self.cursor = 0;
        self.error = None;
        self.resync = false;
        self.scan = false;
    }
    /// Reinicia síntesis y depósito sin descartar la salida pendiente (seek especial -2).
    /// Tras ella se vuelve a sincronizar desde la posición física de entrada.
    pub fn discontinuity(&mut self) -> Result<(), DecodeError> {
        if let Some(error) = self.error.clone() {
            return Err(error);
        }
        self.decoder.rewind();
        self.offset = self.source_position;
        self.resync = true;
        self.scan = true;
        Ok(())
    }
    /// Posición de entrada comprimida del proveedor; `0xffff_fffe` conserva el PCM pendiente.
    /// `0xffff_ffff` descarta la salida pendiente y conserva la posición física.
    pub fn seek_input(&mut self, position: u32) -> Result<(), DecodeError> {
        if position == 0 {
            self.rewind();
            return Ok(());
        }
        if position == 0xffff_fffe {
            return self.discontinuity();
        }
        if position == 0xffff_ffff {
            if let Some(error) = self.error.clone() {
                return Err(error);
            }
            self.decoder.rewind();
            self.offset = self.source_position;
            self.pending.clear();
            self.cursor = 0;
            self.resync = true;
            self.scan = true;
            return Ok(());
        }
        self.decoder.rewind();
        self.offset = (position as usize).min(self.input.len());
        self.pending.clear();
        self.cursor = 0;
        self.error = None;
        self.resync = true;
        self.scan = true;
        Ok(())
    }
    /// Llena el buffer como el proceso x86: el tramo posterior al EOF se pone a cero.
    /// Si una trama no está definida, entrega primero los bytes pendientes y deja
    /// el error para la siguiente llamada; no interpreta el error como EOF limpio.
    pub fn read(&mut self, dst: &mut [u8]) -> Result<usize, DecodeError> {
        if dst.is_empty() {
            return Ok(0);
        }
        let mut written = 0;
        loop {
            let count = (dst.len() - written).min(self.pending.len() - self.cursor);
            dst[written..written + count].copy_from_slice(&self.pending[self.cursor..self.cursor + count]);
            written += count;
            self.cursor += count;
            if written == dst.len() {
                return Ok(written);
            }
            if let Some(error) = self.error.clone() {
                if written > 0 {
                    return Ok(written);
                }
                return Err(error);
            }
            if self.offset == self.input.len() {
                dst[written..].fill(0);
                return Ok(written);
            }
            if self.scan {
                let mut at = self.offset;
                let mut found = None;
                while at + 1 < self.input.len() {
                    if self.input[at] != 0xff {
                        at += 1;
                    } else if self.input[at + 1] & 0xe0 != 0xe0 {
                        at += 2;
                    } else {
                        if super::mp3_frame::parse_final_frame(self.input, at)
                            .is_ok_and(|f| f.sample_rate == self.sample_rate && f.channels == self.channels)
                        {
                            found = Some(at);
                            break;
                        }
                        at += 4;
                    }
                }
                self.offset = found.unwrap_or(self.input.len());
                self.source_position = self.offset;
                self.scan = false;
                continue;
            }
            let result = (|| {
                let f = super::mp3_frame::parse_final_frame(self.input, self.offset)?;
                if self.resync && f.main_data_begin > self.decoder.reservoir.len() {
                    self.decoder.reservoir.extend_from_slice(f.main_data);
                    if self.decoder.reservoir.len() > 511 {
                        self.decoder.reservoir.drain(..self.decoder.reservoir.len() - 511);
                    }
                    self.offset = self
                        .offset
                        .checked_add(f.available_length)
                        .ok_or(DecodeError::InvalidLimit)?;
                    self.source_position = self.offset;
                    return Ok::<Option<Vec<u8>>, DecodeError>(None);
                }
                let pcm = self.decoder.decode_frame(&f)?;
                self.resync = false;
                self.offset = self
                    .offset
                    .checked_add(f.available_length)
                    .ok_or(DecodeError::InvalidLimit)?;
                self.source_position = self.offset;
                Ok::<Option<Vec<u8>>, DecodeError>(Some(pcm.iter().flat_map(|x| x.to_le_bytes()).collect()))
            })();
            match result {
                Ok(Some(pcm)) => {
                    self.pending = pcm;
                    self.cursor = 0;
                }
                Ok(None) => {}
                Err(error) => {
                    self.error = Some(error);
                }
            }
        }
    }
}

/// Resultado de lectura progresiva: jamás confunde falta temporal de datos con EOF.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgressiveStatus {
    Filled,
    NeedInput,
    End,
    /// Se entregó PCM anterior; el siguiente `read` devuelve el error guardado.
    ErrorPending,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProgressiveRead {
    pub written: usize,
    pub status: ProgressiveStatus,
}

/// Decodificador alimentado por fragmentos comprimidos, con memoria acotada.
/// No presupone que una lectura corta de la fuente sea EOF.
pub struct ProgressiveMp3 {
    input: Vec<u8>,
    limit: usize,
    ended: bool,
    at_start: bool,
    format: Option<(u32, usize)>,
    decoder: Decoder,
    pending: Vec<u8>,
    cursor: usize,
    error: Option<DecodeError>,
    scan: bool,
    resync: bool,
}
impl ProgressiveMp3 {
    pub fn new(limit: usize) -> Result<Self, DecodeError> {
        if limit < 4 {
            return Err(DecodeError::InvalidLimit);
        }
        Ok(Self {
            input: Vec::new(),
            limit,
            ended: false,
            at_start: true,
            format: None,
            decoder: Decoder::default(),
            pending: Vec::new(),
            cursor: 0,
            error: None,
            scan: false,
            resync: false,
        })
    }
    pub fn buffered_input(&self) -> usize {
        self.input.len()
    }
    pub fn format(&self) -> Option<(u32, usize)> {
        self.format
    }
    pub fn remaining_capacity(&self) -> usize {
        self.limit - self.input.len()
    }
    pub fn push(&mut self, bytes: &[u8]) -> Result<(), DecodeError> {
        if self.ended || bytes.len() > self.remaining_capacity() {
            return Err(DecodeError::InvalidLimit);
        }
        self.input
            .try_reserve(bytes.len())
            .map_err(|_| DecodeError::InvalidLimit)?;
        self.input.extend_from_slice(bytes);
        Ok(())
    }
    pub fn finish(&mut self) {
        self.ended = true;
    }
    /// Emula la primera trama corta con cola cero solo cuando el decoder la
    /// acepta íntegramente. En cualquier otro caso no altera el estado.
    /// No generaliza a tramas posteriores ni datos internos sin inicializar.
    pub fn try_original_short_first_frame(&mut self) -> bool {
        if self.ended || self.at_start || self.scan || self.format.is_some() || self.input.is_empty() {
            return false;
        }
        let Ok(partial) = super::mp3_frame::parse_final_frame(&self.input, 0) else {
            return false;
        };
        if partial.available_length == partial.length || partial.length > self.limit {
            return false;
        }
        let mut padded = self.input.clone();
        padded.resize(partial.length, 0);
        let Ok(frame) = super::mp3_frame::parse_frame(&padded, 0) else {
            return false;
        };
        let mut decoder = self.decoder.clone();
        let Ok(pcm) = decoder.decode_frame_zero_extended(&frame) else {
            return false;
        };
        self.format = Some((frame.sample_rate, frame.channels));
        self.decoder = decoder;
        self.input.clear();
        self.scan = true;
        self.pending = pcm.iter().flat_map(|sample| sample.to_le_bytes()).collect();
        self.cursor = 0;
        true
    }
    /// -2 reinicia síntesis y depósito; conserva PCM pendiente y bytes no consumidos.
    pub fn discontinuity(&mut self) -> Result<(), DecodeError> {
        if let Some(error) = self.error.clone() {
            return Err(error);
        }
        self.decoder.rewind();
        self.resync = true;
        self.scan = true;
        self.at_start = false;
        Ok(())
    }
    /// Nuevo origen comprimido tras seek HSTREAM; solo cero permite una etiqueta inicial.
    pub fn reset_input(&mut self, zero: bool) -> Result<(), DecodeError> {
        if let Some(error) = self.error.clone() {
            return Err(error);
        }
        self.decoder.rewind();
        self.input.clear();
        self.pending.clear();
        self.cursor = 0;
        self.ended = false;
        self.at_start = zero;
        self.scan = !zero;
        self.resync = !zero;
        Ok(())
    }
    pub fn read(&mut self, output: &mut [u8]) -> Result<ProgressiveRead, DecodeError> {
        use super::mp3_frame::{audio_start, parse_final_frame, parse_frame, FrameError};
        let mut written = 0;
        loop {
            let n = (output.len() - written).min(self.pending.len() - self.cursor);
            output[written..written + n].copy_from_slice(&self.pending[self.cursor..self.cursor + n]);
            written += n;
            self.cursor += n;
            if written == output.len() {
                return Ok(ProgressiveRead {
                    written,
                    status: ProgressiveStatus::Filled,
                });
            }
            if let Some(error) = self.error.clone() {
                if written > 0 {
                    return Ok(ProgressiveRead {
                        written,
                        status: ProgressiveStatus::ErrorPending,
                    });
                }
                return Err(error);
            }
            if self.at_start {
                if self.input.len() < 3 && b"ID3".starts_with(&self.input) && !self.ended {
                    return Ok(ProgressiveRead {
                        written,
                        status: ProgressiveStatus::NeedInput,
                    });
                }
                match audio_start(&self.input) {
                    Ok(n) => {
                        self.input.drain(..n);
                        self.at_start = false;
                    }
                    Err(FrameError::Truncated) if !self.ended => {
                        return Ok(ProgressiveRead {
                            written,
                            status: ProgressiveStatus::NeedInput,
                        })
                    }
                    Err(e) => {
                        self.error = Some(e.into());
                        continue;
                    }
                }
            }
            if self.input.is_empty() {
                if self.ended {
                    output[written..].fill(0);
                    return Ok(ProgressiveRead {
                        written,
                        status: ProgressiveStatus::End,
                    });
                }
                return Ok(ProgressiveRead {
                    written,
                    status: ProgressiveStatus::NeedInput,
                });
            }
            if self.scan {
                let mut at = 0;
                loop {
                    if at + 1 >= self.input.len() {
                        self.input.drain(..at.min(self.input.len()));
                        if self.ended {
                            self.input.clear();
                            self.scan = false;
                            break;
                        }
                        return Ok(ProgressiveRead {
                            written,
                            status: ProgressiveStatus::NeedInput,
                        });
                    }
                    if self.input[at] != 0xff {
                        at += 1;
                        continue;
                    }
                    if self.input[at + 1] & 0xe0 != 0xe0 {
                        at += 2;
                        continue;
                    }
                    if at + 4 > self.input.len() && !self.ended {
                        self.input.drain(..at);
                        return Ok(ProgressiveRead {
                            written,
                            status: ProgressiveStatus::NeedInput,
                        });
                    }
                    match parse_final_frame(&self.input, at) {
                        Ok(f)
                            if self
                                .format
                                .is_none_or(|(rate, ch)| f.sample_rate == rate && f.channels == ch) =>
                        {
                            self.input.drain(..at);
                            self.scan = false;
                            break;
                        }
                        Err(FrameError::Truncated) if !self.ended => {
                            self.input.drain(..at);
                            return Ok(ProgressiveRead {
                                written,
                                status: ProgressiveStatus::NeedInput,
                            });
                        }
                        _ => at += 4,
                    }
                }
                continue;
            }
            let frame = if self.ended {
                parse_final_frame(&self.input, 0)
            } else {
                parse_frame(&self.input, 0)
            };
            let f = match frame {
                Ok(f) => f,
                Err(FrameError::Truncated) if !self.ended => {
                    return Ok(ProgressiveRead {
                        written,
                        status: ProgressiveStatus::NeedInput,
                    })
                }
                Err(e) => {
                    self.error = Some(e.into());
                    continue;
                }
            };
            if self
                .format
                .is_some_and(|(rate, ch)| rate != f.sample_rate || ch != f.channels)
            {
                self.error = Some(FrameError::FormatChanged.into());
                continue;
            }
            self.format = Some((f.sample_rate, f.channels));
            let length = f.available_length;
            let result = if self.resync && f.main_data_begin > self.decoder.reservoir.len() {
                self.decoder.reservoir.extend_from_slice(f.main_data);
                if self.decoder.reservoir.len() > 511 {
                    self.decoder.reservoir.drain(..self.decoder.reservoir.len() - 511);
                }
                Ok(None)
            } else {
                self.decoder.decode_frame(&f).map(Some)
            };
            match result {
                Ok(pcm) => {
                    self.input.drain(..length);
                    self.scan = true;
                    if let Some(pcm) = pcm {
                        self.resync = false;
                        self.pending = pcm.iter().flat_map(|x| x.to_le_bytes()).collect();
                        self.cursor = 0;
                    }
                }
                Err(e) => {
                    self.error = Some(e);
                }
            }
        }
    }
}
