//! Lector acotado de cabeceras y side-info Layer III; sin ejecutar DLL.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameError {
    Truncated,
    InvalidHeader,
    UnsupportedVersion,
    InvalidSideInfo,
    ReservoirUnderflow,
    FormatChanged,
}
#[derive(Debug, Clone, Copy, Default)]
pub struct ChannelInfo {
    pub part_bits: usize,
    pub big_values: usize,
    pub global_gain: u8,
    pub scale_compress: u16,
    pub window_switch: bool,
    pub block_type: u8,
    pub mixed: bool,
    pub tables: [u8; 3],
    pub subblock_gain: [u8; 3],
    pub region0: u8,
    pub region1: u8,
    pub preflag: bool,
    pub scale_mode: bool,
    pub count1_table: u8,
}
#[derive(Debug, Clone)]
pub struct Frame<'a> {
    pub offset: usize,
    pub length: usize,
    /// Cantidad de bytes presentes de la trama; puede ser menor que `length` en el final.
    pub available_length: usize,
    pub version: u8,
    pub rate_index: u8,
    pub sample_rate: u32,
    pub channels: usize,
    pub mode: u8,
    pub extension: u8,
    pub main_data_begin: usize,
    pub scfsi: [[bool; 4]; 2],
    pub granules: usize,
    pub info: [[ChannelInfo; 2]; 2],
    pub main_data: &'a [u8],
}
struct Bits<'a> {
    data: &'a [u8],
    bit: usize,
}
impl Bits<'_> {
    fn read(&mut self, n: usize) -> Result<u32, FrameError> {
        if self.bit.checked_add(n).is_none_or(|v| v > self.data.len() * 8) {
            return Err(FrameError::Truncated);
        }
        let mut x = 0;
        for _ in 0..n {
            x = (x << 1) | u32::from((self.data[self.bit / 8] >> (7 - self.bit % 8)) & 1);
            self.bit += 1;
        }
        Ok(x)
    }
}
/// Lee una trama completa en el desplazamiento exacto. No busca sincronización.
pub fn parse_frame(input: &[u8], offset: usize) -> Result<Frame<'_>, FrameError> {
    parse(input, offset, false)
}

/// Lee también la última trama incompleta si conserva cabecera e información lateral.
/// Los bits de datos ausentes nunca se inventan; el decoder comprueba su disponibilidad.
pub fn parse_final_frame(input: &[u8], offset: usize) -> Result<Frame<'_>, FrameError> {
    parse(input, offset, true)
}

fn parse(input: &[u8], offset: usize, allow_incomplete: bool) -> Result<Frame<'_>, FrameError> {
    let h = input
        .get(offset..offset.checked_add(4).ok_or(FrameError::Truncated)?)
        .ok_or(FrameError::Truncated)?;
    let word = u32::from_be_bytes(h.try_into().unwrap());
    if word >> 21 != 0x7ff || (word >> 17) & 3 != 1 {
        return Err(FrameError::InvalidHeader);
    }
    let version_bits = (word >> 19) & 3;
    let version = match version_bits {
        3 => 1,
        2 => 0,
        0 => return Err(FrameError::UnsupportedVersion),
        _ => return Err(FrameError::InvalidHeader),
    };
    let bitrate_index = ((word >> 12) & 15) as usize;
    let rate_index = ((word >> 10) & 3) as usize;
    if bitrate_index == 0 || bitrate_index == 15 || rate_index == 3 {
        return Err(FrameError::InvalidHeader);
    }
    let br = if version == 1 {
        [0, 32, 40, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320, 0][bitrate_index]
    } else {
        [0, 8, 16, 24, 32, 40, 48, 56, 64, 80, 96, 112, 128, 144, 160, 0][bitrate_index]
    };
    let sample_rate = [44100, 48000, 32000][rate_index] / if version == 1 { 1 } else { 2 };
    let length = ((if version == 1 { 144000 } else { 72000 }) * br / sample_rate + ((word >> 9) & 1)) as usize;
    let end = offset.checked_add(length).ok_or(FrameError::Truncated)?;
    let available_end = if allow_incomplete { end.min(input.len()) } else { end };
    let frame = input.get(offset..available_end).ok_or(FrameError::Truncated)?;
    let mode = ((word >> 6) & 3) as u8;
    let channels = if mode == 3 { 1 } else { 2 };
    let extension = ((word >> 4) & 3) as u8;
    let crc = if (word >> 16) & 1 == 0 { 2 } else { 0 };
    let side_len = if version == 1 {
        if channels == 1 {
            17
        } else {
            32
        }
    } else if channels == 1 {
        9
    } else {
        17
    };
    let side_start = 4 + crc;
    let side_end = side_start + side_len;
    let mut bits = Bits {
        data: frame.get(side_start..side_end).ok_or(FrameError::Truncated)?,
        bit: 0,
    };
    let main_data_begin = bits.read(if version == 1 { 9 } else { 8 })? as usize;
    bits.read(if version == 1 {
        if channels == 1 {
            5
        } else {
            3
        }
    } else if channels == 1 {
        1
    } else {
        2
    })?;
    let mut scfsi = [[false; 4]; 2];
    if version == 1 {
        for row in scfsi.iter_mut().take(channels) {
            for value in row {
                *value = bits.read(1)? != 0;
            }
        }
    }
    let granules = if version == 1 { 2 } else { 1 };
    let mut info = [[ChannelInfo::default(); 2]; 2];
    for row in info.iter_mut().take(granules) {
        for (ch, slot) in row.iter_mut().take(channels).enumerate() {
            let mut c = ChannelInfo {
                part_bits: bits.read(12)? as usize,
                big_values: bits.read(9)? as usize,
                global_gain: bits.read(8)? as u8,
                scale_compress: bits.read(if version == 1 { 4 } else { 9 })? as u16,
                window_switch: bits.read(1)? != 0,
                ..ChannelInfo::default()
            };
            if c.big_values > 288 {
                return Err(FrameError::InvalidSideInfo);
            }
            if c.window_switch {
                c.block_type = bits.read(2)? as u8;
                c.mixed = bits.read(1)? != 0;
                if c.block_type == 0 {
                    return Err(FrameError::InvalidSideInfo);
                }
                c.tables[0] = bits.read(5)? as u8;
                c.tables[1] = bits.read(5)? as u8;
                for v in &mut c.subblock_gain {
                    *v = bits.read(3)? as u8;
                }
                c.region0 = if c.block_type == 2 && !c.mixed { 8 } else { 7 };
                c.region1 = 20 - c.region0;
            } else {
                for v in &mut c.tables {
                    *v = bits.read(5)? as u8;
                }
                c.region0 = bits.read(4)? as u8;
                c.region1 = bits.read(3)? as u8;
            }
            c.preflag = if version == 1 {
                bits.read(1)? != 0
            } else {
                !(mode == 1 && extension & 1 != 0 && ch == 1) && c.scale_compress >= 500
            };
            c.scale_mode = bits.read(1)? != 0;
            c.count1_table = bits.read(1)? as u8;
            *slot = c;
        }
    }
    Ok(Frame {
        offset,
        length,
        available_length: frame.len(),
        version,
        rate_index: rate_index as u8,
        sample_rate,
        channels,
        mode,
        extension,
        main_data_begin,
        scfsi,
        granules,
        info,
        main_data: &frame[side_end..],
    })
}
/// Salta una etiqueta ID3v2 completa. No trata etiquetas truncadas como audio.
pub fn audio_start(input: &[u8]) -> Result<usize, FrameError> {
    if !input.starts_with(b"ID3") {
        return Ok(0);
    }
    let h = input.get(..10).ok_or(FrameError::Truncated)?;
    if h[6..10].iter().any(|x| x & 0x80 != 0) {
        return Err(FrameError::InvalidHeader);
    }
    let size = h[6..10].iter().fold(0usize, |n, &x| (n << 7) | usize::from(x));
    let end = 10usize
        .checked_add(size)
        .and_then(|n| n.checked_add(if h[5] & 0x10 != 0 { 10 } else { 0 }))
        .ok_or(FrameError::Truncated)?;
    if end > input.len() {
        return Err(FrameError::Truncated);
    }
    Ok(end)
}
