//! IJL11 baseline JPEG path used by the five English town-27 AIM images.
//!
//! Independently reconstructed from AIM.dll 100062b0 / IJL11 600125b0,
//! 60015a80, 60013450, 600074b0 and 6000b7f0. No vendor code or tables of
//! decoded commercial pixels are embedded. The decimal AAN constants, word
//! wrapping, DC DWORD store and two color-rounding paths are observable parts
//! of the original decoder, rather than interchangeable JPEG approximations.

use super::{rd32, AimError, Image, MAX_BYTES};

const ZIGZAG: [usize; 64] = [
    0, 1, 8, 16, 9, 2, 3, 10, 17, 24, 32, 25, 18, 11, 4, 5, 12, 19, 26, 33, 40, 48, 41, 34, 27, 20, 13, 6, 7, 14, 21,
    28, 35, 42, 49, 56, 57, 50, 43, 36, 29, 22, 15, 23, 30, 37, 44, 51, 58, 59, 52, 45, 38, 31, 39, 46, 53, 60, 61, 54,
    47, 55, 62, 63,
];

// IJL 60025588: six decimal places, zigzag order. Multiplication by
// 60025788 (1/16) and 60025580 (32768), addition of 0.5, then truncation.
// Exact rational arithmetic avoids dependence on the host floating-point mode.
const AAN: [u32; 64] = [
    125000, 173380, 173380, 163320, 240485, 163320, 146984, 226532, 226532, 146984, 125000, 203873, 213388, 203873,
    125000, 98212, 173380, 192044, 192044, 173380, 98212, 67650, 136224, 163320, 172835, 163320, 136224, 67650, 34566,
    93833, 128320, 146984, 146984, 128320, 93833, 34566, 47944, 88388, 115485, 125000, 115485, 88388, 47944, 45162,
    79547, 98212, 98212, 79547, 45162, 40645, 67650, 77165, 67650, 40645, 34566, 53152, 53152, 34566, 27158, 36612,
    27158, 18707, 18707, 9558,
];

fn scaled_quantizer(value: u8, zigzag: usize) -> i16 {
    ((u64::from(value) * u64::from(AAN[zigzag]) * 2048 + 500000) / 1000000) as i16
}

#[derive(Default)]
struct Huffman {
    counts: [u8; 16],
    values: Vec<u8>,
}

impl Huffman {
    fn symbol(&self, bits: &mut Bits<'_>) -> Result<u8, AimError> {
        let (mut code, mut first, mut at) = (0u32, 0u32, 0usize);
        for &count in &self.counts {
            code = code * 2 + bits.get(1)?;
            if code >= first && code - first < u32::from(count) {
                return self
                    .values
                    .get(at + (code - first) as usize)
                    .copied()
                    .ok_or(AimError::InvalidJpeg);
            }
            first = (first + u32::from(count)) * 2;
            at += count as usize;
        }
        Err(AimError::InvalidJpeg)
    }
}

struct Bits<'a> {
    data: &'a [u8],
    at: usize,
    value: u32,
    left: u32,
}

impl Bits<'_> {
    fn get(&mut self, n: u32) -> Result<u32, AimError> {
        while self.left < n {
            let b = *self.data.get(self.at).ok_or(AimError::Truncated)?;
            self.at += 1;
            if b == 0xff {
                if self.data.get(self.at) != Some(&0) {
                    return Err(AimError::InvalidJpeg);
                }
                self.at += 1;
            }
            self.value = (self.value << 8) | u32::from(b);
            self.left += 8;
        }
        self.left -= n;
        Ok((self.value >> self.left) & ((1 << n) - 1))
    }

    fn signed(&mut self, n: u32) -> Result<i32, AimError> {
        if n == 0 {
            return Ok(0);
        }
        let value = self.get(n)? as i32;
        Ok(if value < 1 << (n - 1) {
            value - ((1 << n) - 1)
        } else {
            value
        })
    }

    fn marker(&mut self, expected: u8) -> Result<(), AimError> {
        self.left = 0;
        if self.data.get(self.at) != Some(&0xff) {
            return Err(AimError::InvalidJpeg);
        }
        while self.data.get(self.at) == Some(&0xff) {
            self.at += 1;
        }
        if self.data.get(self.at) != Some(&expected) {
            return Err(AimError::InvalidJpeg);
        }
        self.at += 1;
        Ok(())
    }
}

fn add(a: i16, b: i16) -> i16 {
    a.wrapping_add(b)
}
fn sub(a: i16, b: i16) -> i16 {
    a.wrapping_sub(b)
}
fn mul_hi(a: i16, constant: i16, shift: u32) -> i16 {
    ((i32::from(a.wrapping_shl(shift)) * i32::from(constant)) >> 16) as i16
}

// One lane of the two MMX passes at 60013450. Every intermediate is a WORD.
fn idct_line(x: [i16; 8]) -> [i16; 8] {
    let a = add(x[2], x[6]);
    let b = sub(mul_hi(sub(x[2], x[6]), 23040, 2), a);
    let c = add(x[0], x[4]);
    let d = sub(x[0], x[4]);
    let (e0, e3, e1, e2) = (add(c, a), sub(c, a), add(d, b), sub(d, b));
    let a = sub(x[5], x[3]);
    let b = sub(x[1], x[7]);
    let c = add(x[5], x[3]);
    let d = add(x[1], x[7]);
    let m = mul_hi(a, -21408, 3);
    let n = mul_hi(b, 17728, 2);
    let p = mul_hi(add(a, b), 30272, 2);
    let o = mul_hi(sub(d, c), 23040, 2);
    let t = add(d, c);
    let u = sub(add(p, m), t);
    let v = sub(o, u);
    let z = add(sub(n, p), v);
    [
        add(e0, t),
        add(e1, u),
        add(e2, v),
        sub(e3, z),
        add(e3, z),
        sub(e2, v),
        sub(e1, u),
        sub(e0, t),
    ]
}

fn idct(mut block: [i16; 64]) -> [u8; 64] {
    for c in 0..8 {
        let line = idct_line(std::array::from_fn(|r| block[r * 8 + c]));
        for r in 0..8 {
            block[r * 8 + c] = line[r];
        }
    }
    let mut out = [0; 64];
    for r in 0..8 {
        let line = idct_line(std::array::from_fn(|c| block[r * 8 + c]));
        for c in 0..8 {
            out[r * 8 + c] = ((i32::from(line[c]) >> 4) + 128).clamp(0, 255) as u8;
        }
    }
    out
}

fn block(
    bits: &mut Bits<'_>,
    dc: &Huffman,
    ac: &Huffman,
    quant: &[i16; 64],
    previous: &mut i32,
) -> Result<[u8; 64], AimError> {
    let n = u32::from(dc.symbol(bits)?);
    if n > 11 {
        return Err(AimError::InvalidJpeg);
    }
    *previous = previous.wrapping_add(bits.signed(n)?);
    let scaled_dc = (previous.wrapping_mul(i32::from(quant[0])).wrapping_add(64)) >> 7;
    let mut coeffs = [0; 64];
    coeffs[0] = scaled_dc as i16;
    // 60015ba0 MOVD [coeffs],mm5 writes 32 bits, including the sign WORD.
    // AC[1] retains it unless that coefficient is explicitly present in the scan.
    coeffs[1] = (scaled_dc >> 16) as i16;
    let mut k = 1;
    while k < 64 {
        let t = ac.symbol(bits)?;
        if t == 0 {
            break;
        }
        if t == 0xf0 {
            k += 16;
            if k > 64 {
                return Err(AimError::InvalidJpeg);
            }
            continue;
        }
        let n = u32::from(t & 15);
        k += usize::from(t >> 4);
        if n == 0 || n > 10 || k >= 64 {
            return Err(AimError::InvalidJpeg);
        }
        let value = bits.signed(n)?;
        coeffs[ZIGZAG[k]] = (value.wrapping_mul(i32::from(quant[k])).wrapping_add(64) >> 7) as i16;
        k += 1;
    }
    Ok(idct(coeffs))
}

fn color(y: u8, cb: u8, cr: u8, partial: bool) -> [u8; 4] {
    let (y, cb, cr) = (i32::from(y), i32::from(cb), i32::from(cr));
    let (r, b) = if partial {
        // 6000b7f0 scalar clipping of a partial MCU; these constants differ
        // observably from the full 16x16 MMX conversion at 600074b0.
        ((359 * cr - 45813) >> 8, (454 * cb - 57937) >> 8)
    } else {
        (((359 * cr) >> 8) - 179, ((454 * cb) >> 8) - 227)
    };
    let g = 88 * cb + 183 * cr - 34806;
    let g = if partial { g >> 8 } else { i32::from((g as i16) >> 8) };
    [
        (y + r).clamp(0, 255) as u8,
        (y - g).clamp(0, 255) as u8,
        (y + b).clamp(0, 255) as u8,
        255,
    ]
}

fn be16(data: &[u8], at: usize) -> Result<usize, AimError> {
    let b = data.get(at..at + 2).ok_or(AimError::Truncated)?;
    Ok(usize::from(u16::from_be_bytes([b[0], b[1]])))
}

pub(super) fn decode(layer: &[u8]) -> Result<Image, AimError> {
    let header = layer.get(..19).ok_or(AimError::Truncated)?;
    let (width, height) = (rd32(header, 11)?, rd32(header, 15)?);
    let pitch = width.checked_mul(4).ok_or(AimError::TooLarge)?;
    let size = u64::from(pitch) * u64::from(height);
    if size > MAX_BYTES as u64 {
        return Err(AimError::TooLarge);
    }
    if width == 0 || height == 0 {
        return Err(AimError::InvalidJpeg);
    }
    // Only flags used by this corpus. Transparency thresholds are a separate
    // AIM operation; fail explicitly rather than silently misdecode them.
    if header[5] & !1 != 0 || header[7..11] != [0; 4] {
        return Err(AimError::UnsupportedJpeg);
    }
    let declared = rd32(header, 1)? as usize;
    if declared < 19 {
        return Err(AimError::InvalidJpeg);
    }
    let jpeg = layer.get(19..declared).ok_or(AimError::Truncated)?;
    if jpeg.get(..2) != Some(&[0xff, 0xd8]) {
        return Err(AimError::InvalidJpeg);
    }
    let mut at = 2;
    let mut quant: [Option<[i16; 64]>; 4] = [None; 4];
    let mut huffman: [Option<Huffman>; 8] = std::array::from_fn(|_| None);
    let mut components = Vec::new();
    let mut restart_interval = 0;
    let selectors;
    loop {
        if jpeg.get(at) != Some(&0xff) {
            return Err(AimError::InvalidJpeg);
        }
        while jpeg.get(at) == Some(&0xff) {
            at += 1;
        }
        let marker = *jpeg.get(at).ok_or(AimError::Truncated)?;
        at += 1;
        let len = be16(jpeg, at)?;
        if len < 2 {
            return Err(AimError::InvalidJpeg);
        }
        let data = jpeg.get(at + 2..at + len).ok_or(AimError::Truncated)?;
        at += len;
        match marker {
            0xdb => {
                let mut pos = 0;
                while pos < data.len() {
                    let table = data[pos];
                    pos += 1;
                    if table >> 4 != 0 {
                        return Err(AimError::UnsupportedJpeg);
                    }
                    if table >= 4 {
                        return Err(AimError::InvalidJpeg);
                    }
                    let values = data.get(pos..pos + 64).ok_or(AimError::Truncated)?;
                    if values.contains(&0) {
                        return Err(AimError::InvalidJpeg);
                    }
                    let q = std::array::from_fn(|i| scaled_quantizer(values[i], i));
                    quant[table as usize] = Some(q);
                    pos += 64;
                }
            }
            0xc4 => {
                let mut pos = 0;
                while pos < data.len() {
                    let table = data[pos];
                    pos += 1;
                    let class = table >> 4;
                    let id = table & 15;
                    if class > 1 || id > 3 {
                        return Err(AimError::InvalidJpeg);
                    }
                    let counts: [u8; 16] = data.get(pos..pos + 16).ok_or(AimError::Truncated)?.try_into().unwrap();
                    pos += 16;
                    let count: usize = counts.iter().map(|v| usize::from(*v)).sum();
                    if count > 256 || count == 0 {
                        return Err(AimError::InvalidJpeg);
                    }
                    let mut slots = 1i32;
                    for n in counts {
                        slots = slots * 2 - i32::from(n);
                        if slots < 0 {
                            return Err(AimError::InvalidJpeg);
                        }
                    }
                    let values = data.get(pos..pos + count).ok_or(AimError::Truncated)?.to_vec();
                    pos += count;
                    huffman[usize::from(class * 4 + id)] = Some(Huffman { counts, values });
                }
            }
            0xc0 => {
                if !components.is_empty() || data.len() != 15 {
                    return Err(AimError::UnsupportedJpeg);
                }
                if data[0] != 8 || data[5] != 3 {
                    return Err(AimError::UnsupportedJpeg);
                }
                if be16(data, 1)? as u32 != height || be16(data, 3)? as u32 != width {
                    return Err(AimError::InvalidJpeg);
                }
                for i in 0..3 {
                    let c = &data[6 + i * 3..9 + i * 3];
                    if c[1] != if i == 0 { 0x22 } else { 0x11 } {
                        return Err(AimError::UnsupportedJpeg);
                    }
                    if c[2] >= 4 || components.iter().any(|(id, _)| *id == c[0]) {
                        return Err(AimError::InvalidJpeg);
                    }
                    components.push((c[0], c[2]));
                }
            }
            0xdd => {
                if data.len() != 2 {
                    return Err(AimError::InvalidJpeg);
                }
                restart_interval = be16(data, 0)?;
            }
            0xda => {
                if data.len() != 10 || data[0] != 3 || components.len() != 3 {
                    return Err(AimError::UnsupportedJpeg);
                }
                if data[7..] != [0, 63, 0] {
                    return Err(AimError::UnsupportedJpeg);
                }
                let mut s = [(0usize, 0usize); 3];
                for i in 0..3 {
                    if data[1 + i * 2] != components[i].0 {
                        return Err(AimError::UnsupportedJpeg);
                    }
                    let t = data[2 + i * 2];
                    if t >> 4 > 3 || t & 15 > 3 {
                        return Err(AimError::InvalidJpeg);
                    }
                    s[i] = ((t >> 4) as usize, (t & 15) as usize + 4);
                }
                selectors = s;
                break;
            }
            0xe0..=0xef | 0xfe => {}
            _ => return Err(AimError::UnsupportedJpeg),
        }
    }
    for i in 0..3 {
        if quant[components[i].1 as usize].is_none()
            || huffman[selectors[i].0].is_none()
            || huffman[selectors[i].1].is_none()
        {
            return Err(AimError::InvalidJpeg);
        }
    }
    let (w, h) = (width as usize, height as usize);
    let mut pixels = vec![0; size as usize];
    let mut bits = Bits {
        data: jpeg,
        at,
        value: 0,
        left: 0,
    };
    let mut previous = [0; 3];
    let (mut mcu_index, mut next_restart) = (0, 0);
    for my in 0..h.div_ceil(16) {
        for mx in 0..w.div_ceil(16) {
            if mcu_index > 0 && restart_interval > 0 && mcu_index % restart_interval == 0 {
                bits.marker(0xd0 + next_restart)?;
                next_restart = (next_restart + 1) & 7;
                previous = [0; 3];
            }
            let mut blocks = [[0; 64]; 6];
            let mut bi = 0;
            for ci in 0..3 {
                for _ in 0..if ci == 0 { 4 } else { 1 } {
                    blocks[bi] = block(
                        &mut bits,
                        huffman[selectors[ci].0].as_ref().unwrap(),
                        huffman[selectors[ci].1].as_ref().unwrap(),
                        quant[components[ci].1 as usize].as_ref().unwrap(),
                        &mut previous[ci],
                    )?;
                    bi += 1;
                }
            }
            let partial = mx * 16 + 16 > w || my * 16 + 16 > h;
            for yy in 0..16.min(h - my * 16) {
                for xx in 0..16.min(w - mx * 16) {
                    let lum = blocks[(yy / 8) * 2 + xx / 8][(yy % 8) * 8 + xx % 8];
                    let cb = blocks[4][(yy / 2) * 8 + xx / 2];
                    let cr = blocks[5][(yy / 2) * 8 + xx / 2];
                    let mut rgba = color(lum, cb, cr, partial);
                    // AIM 100064b2: flag bit0 keeps IJL RGBX; cleared swaps R/B.
                    if header[5] & 1 == 0 {
                        rgba.swap(0, 2);
                    }
                    let offset = ((my * 16 + yy) * w + mx * 16 + xx) * 4;
                    pixels[offset..offset + 4].copy_from_slice(&rgba);
                }
            }
            mcu_index += 1;
        }
    }
    bits.marker(0xd9)?;
    Ok(Image {
        pixels,
        palette: None,
        width,
        height,
        bytes_per_pixel: 4,
        format: 4,
        pitch,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use sha2::{Digest, Sha256};

    struct Rng(u32);
    impl Rng {
        fn next(&mut self) -> u32 {
            let mut x = self.0;
            x ^= x << 13;
            x ^= x >> 17;
            x ^= x << 5;
            self.0 = x;
            x
        }
    }

    fn reference() -> serde_json::Value {
        serde_json::from_str(include_str!("../../tests/fixtures/aim_jpeg_kernels.json")).unwrap()
    }

    #[test]
    fn idct_matches_543_original_x86_blocks() {
        let mut inputs = vec![[0; 64]];
        for index in [0, 1, 7, 8, 63] {
            for value in [-32768, -2048, -1, 1, 2047, 32767] {
                let mut a = [0; 64];
                a[index] = value;
                inputs.push(a);
            }
        }
        let mut rng = Rng(0x494a4c11);
        for _ in 0..512 {
            inputs.push(std::array::from_fn(|_| rng.next() as i16));
        }
        let mut hash = Sha256::new();
        for a in &inputs {
            hash.update(idct(*a));
        }
        let r = reference();
        assert_eq!(inputs.len() as u64, r["idct_cases"].as_u64().unwrap());
        assert_eq!(format!("{:x}", hash.finalize()), r["idct_sha256"].as_str().unwrap());
    }

    #[test]
    fn all_16320_quantizers_match_original_x86() {
        let mut hash = Sha256::new();
        for q in 1..=255 {
            let mut natural = [0i16; 64];
            for k in 0..64 {
                natural[ZIGZAG[k]] = scaled_quantizer(q, k);
            }
            for v in natural {
                hash.update(v.to_le_bytes());
            }
        }
        assert_eq!(
            format!("{:x}", hash.finalize()),
            reference()["quant_sha256"].as_str().unwrap()
        );
    }

    #[test]
    fn full_and_partial_color_match_original_x86() {
        for partial in [false, true] {
            let mut rng = Rng(0x494a4c11);
            let mut hash = Sha256::new();
            for index in 0..264 {
                let y: [u8; 256] = if index < 8 {
                    [if index & 1 != 0 { 255 } else { 0 }; 256]
                } else {
                    std::array::from_fn(|_| rng.next() as u8)
                };
                let cb: [u8; 64] = if index < 8 {
                    [if index & 2 != 0 { 255 } else { 0 }; 64]
                } else {
                    std::array::from_fn(|_| rng.next() as u8)
                };
                let cr: [u8; 64] = if index < 8 {
                    [if index & 4 != 0 { 255 } else { 0 }; 64]
                } else {
                    std::array::from_fn(|_| rng.next() as u8)
                };
                let side = if partial { 8 } else { 16 };
                for yy in 0..side {
                    for xx in 0..side {
                        let i = if partial {
                            yy * 8 + xx
                        } else {
                            ((yy / 8) * 2 + xx / 8) * 64 + (yy % 8) * 8 + xx % 8
                        };
                        let c = (yy / 2) * 8 + xx / 2;
                        hash.update(color(y[i], cb[c], cr[c], partial));
                    }
                }
            }
            let key = if partial {
                "color_partial_sha256"
            } else {
                "color_full_sha256"
            };
            assert_eq!(
                format!("{:x}", hash.finalize()),
                reference()[key].as_str().unwrap(),
                "{key}"
            );
        }
    }
}
