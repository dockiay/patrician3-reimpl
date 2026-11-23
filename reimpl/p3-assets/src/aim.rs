//! Imágenes `.aim` (`AIM.dll`, `FUN_10004660` y descodificadores de subcapa) y escritor compatible ([`encode`], [`deflate`]).
//!
//! Un `.aim` empieza por `"AIM\0"`; la subcapa de imagen arranca en el desplazamiento 41 y su primer byte es el tipo. Se
//! reproducen los tipos `0x16` (sin comprimir), `0x1a`/`0x1b`/`0x1c` (texturas de bloques), `0x21` (cuatro canales) y `0x22`
//! (8 bits con paleta) y `0x1e` (JPEG baseline 4:2:0 con las operaciones enteras de `ijl11.dll`).
//!
//! El compresor es un LZ77 por bits (`FUN_10011070`): palabras `u32` leídas de bit bajo a alto. Cabecera `i32`: 0 = vacío,
//! negativo = copia literal de `-n` bytes, positivo = longitud de salida. Cada símbolo empieza con un bit: 0 = literal de 8 bits;
//! 1 = copia con 3 bits de clase `k` (`k + 1` bits de distancia, base `2^(k+1) - 2`, distancia real = valor + 1) y longitud
//! `2 + v` donde `v` se lee en grupos de 2, 3, 4... bits mientras el grupo salga lleno.

use std::fmt;
mod jpeg;

pub const SUBLAYER_OFFSET: usize = 41;
/// Tope de bytes de un bloque y de una imagen; el corpus inglés queda muy por debajo.
pub const MAX_BYTES: usize = 1 << 28;

#[derive(Debug, PartialEq, Eq)]
pub enum AimError {
    BadMagic,
    Truncated,
    /// Tipo de subcapa sin descodificador.
    Unsupported(u8),
    /// JPEG mal formado o que contradice las dimensiones de la subcapa AIM.
    InvalidJpeg,
    /// El perfil JPEG no es baseline de tres componentes 4:2:0 en un único scan.
    UnsupportedJpeg,
    /// Un bloque comprimido produce menos bytes de los que exige la imagen.
    ShortBlock,
    /// Tamaño declarado por encima del límite del modelo (el original reservaría sin comprobar).
    TooLarge,
}

impl fmt::Display for AimError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AimError::BadMagic => write!(f, "no empieza por AIM"),
            AimError::Truncated => write!(f, "archivo truncado"),
            AimError::Unsupported(t) => write!(f, "subcapa {t:#x} sin soporte"),
            AimError::InvalidJpeg => write!(f, "JPEG inválido"),
            AimError::UnsupportedJpeg => write!(f, "perfil JPEG sin soporte"),
            AimError::ShortBlock => write!(f, "bloque comprimido corto"),
            AimError::TooLarge => write!(f, "tamaño declarado excesivo"),
        }
    }
}

/// Lector de bits idéntico al del original: palabras `u32` little-endian, bit bajo primero; más allá del final lee ceros.
struct Bits<'a> {
    data: &'a [u8],
    pos: usize,
    word: u32,
    left: u32,
}

impl<'a> Bits<'a> {
    fn new(data: &'a [u8]) -> Self {
        Bits { data, pos: 0, word: 0, left: 0 }
    }

    fn refill(&mut self) {
        let mut w = [0u8; 4];
        for (i, b) in w.iter_mut().enumerate() {
            *b = self.data.get(self.pos + i).copied().unwrap_or(0);
        }
        self.pos += 4;
        self.word = u32::from_le_bytes(w);
        self.left = 32;
    }

    #[inline]
    fn take(&mut self, n: u32) -> u32 {
        // Camino habitual: los `n` bits están en la palabra actual (mismo resultado que el bucle general de abajo).
        if n < 32 && n <= self.left {
            let value = self.word & ((1u32 << n) - 1);
            self.word >>= n;
            self.left -= n;
            return value;
        }
        let mut value = 0u32;
        let mut got = 0;
        while got < n {
            if self.left == 0 {
                self.refill();
            }
            let k = (n - got).min(self.left);
            let part = if k == 32 { self.word } else { self.word & ((1u32 << k) - 1) };
            value |= part << got;
            self.word = if k == 32 { 0 } else { self.word >> k };
            self.left -= k;
            got += k;
        }
        value
    }
}

/// Descomprime un bloque y devuelve los bytes producidos (la cabecera es la longitud de salida).
pub fn inflate(src: &[u8]) -> Vec<u8> {
    let Some(head) = src.get(..4) else { return Vec::new() };
    let count = i32::from_le_bytes([head[0], head[1], head[2], head[3]]);
    if count == 0 {
        return Vec::new();
    }
    if count < 0 {
        let n = (count.unsigned_abs() as usize).min(src.len() - 4);
        return src[4..4 + n].to_vec();
    }
    let total = (count as usize).min(MAX_BYTES);
    let mut out: Vec<u8> = Vec::with_capacity(total.min(1 << 20));
    let mut bits = Bits::new(&src[4..]);
    while out.len() < total {
        // El original sigue leyendo memoria ajena; el modelo se detiene al agotar el bloque (la salida queda corta).
        if bits.pos > bits.data.len() + 4 {
            break;
        }
        if bits.take(1) == 0 {
            out.push(bits.take(8) as u8);
            continue;
        }
        let class = bits.take(3);
        let nbits = class + 1;
        let distance = bits.take(nbits) + (1 << nbits) - 2 + 1;
        let mut len = 2u32;
        let mut width = 2;
        loop {
            let mask = (1u32 << width) - 1;
            let v = bits.take(width) & mask;
            len += v;
            if v != mask {
                break;
            }
            width += 1;
        }
        let (distance, len) = (distance as usize, len as usize);
        if distance <= out.len() {
            // Copia de lo ya producido (puede solaparse consigo misma cuando la distancia es menor que la longitud).
            let from = out.len() - distance;
            if distance >= len {
                out.extend_from_within(from..from + len);
            } else {
                out.reserve(len);
                for i in 0..len {
                    let b = out[from + i];
                    out.push(b);
                }
            }
        } else {
            for _ in 0..len {
                let from = out.len().wrapping_sub(distance);
                // El original lee memoria previa al bloque; con datos válidos no ocurre.
                let b = out.get(from).copied().unwrap_or(0);
                out.push(b);
            }
        }
    }
    out.truncate(total);
    out
}

/// Imagen descodificada con los campos de `AIM_IMAGE` que observa el EXE.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Image {
    pub pixels: Vec<u8>,
    /// 256 entradas de 4 bytes; solo el tipo `0x22`.
    pub palette: Option<Vec<u8>>,
    pub width: u32,
    pub height: u32,
    /// `AIM_IMAGE+0x10`: bytes por píxel de la rejilla (0 en formatos de bloques).
    pub bytes_per_pixel: u32,
    /// `AIM_IMAGE+0x14`: código de formato.
    pub format: u32,
    /// `AIM_IMAGE+0x18`: paso por fila o, en formatos de bloques, tamaño total.
    pub pitch: u32,
}

fn rd32(b: &[u8], at: usize) -> Result<u32, AimError> {
    b.get(at..at + 4).map(|s| u32::from_le_bytes([s[0], s[1], s[2], s[3]])).ok_or(AimError::Truncated)
}

/// Cursor de la capa (`FUN_10008c20`): devuelve la posición y avanza.
struct Cursor<'a> {
    data: &'a [u8],
    at: usize,
}

impl<'a> Cursor<'a> {
    fn take(&mut self, n: usize) -> Result<&'a [u8], AimError> {
        let end = self.at.checked_add(n).ok_or(AimError::Truncated)?;
        let s = self.data.get(self.at..end.min(self.data.len())).ok_or(AimError::Truncated)?;
        self.at = end;
        Ok(s)
    }

    /// `[u32 n][n bytes]`, devuelto el bloque a descomprimir.
    fn block(&mut self) -> Result<&'a [u8], AimError> {
        let n = rd32(self.data, self.at)? as usize;
        self.at += 4;
        self.take(n)
    }
}

fn blocks(cur: &mut Cursor, total: usize, chunk: usize, out: &mut Vec<u8>) -> Result<(), AimError> {
    let mut remaining = total;
    while remaining > 0 {
        let want = remaining.min(chunk);
        let got = inflate(cur.block()?);
        if got.len() < want {
            return Err(AimError::ShortBlock);
        }
        out.extend_from_slice(&got[..want]);
        remaining -= want;
    }
    Ok(())
}

/// Tamaño de la imagen en formatos de bloques (`FUN_100033d0`, casos `0xd`, `0xf` y `0x11`).
fn block_size(format: u32, w: u32, h: u32) -> u32 {
    match format {
        0xd => (w.wrapping_mul(4).wrapping_mul(h)) / 8,
        0xf | 0x11 => w.wrapping_mul(h),
        _ => 0,
    }
}

pub fn decode(file: &[u8]) -> Result<Image, AimError> {
    if file.get(..3) != Some(b"AIM") {
        return Err(AimError::BadMagic);
    }
    let kind = *file.get(SUBLAYER_OFFSET).ok_or(AimError::Truncated)?;
    let mut cur = Cursor { data: file, at: SUBLAYER_OFFSET };
    match kind {
        0x1e => jpeg::decode(&file[SUBLAYER_OFFSET..]),
        0x16 => {
            let h = cur.take(0x12)?;
            let (w, ht, pitch) = (rd32(h, 6)?, rd32(h, 10)?, rd32(h, 14)?);
            let pixels = cur.take(pitch as usize * ht as usize)?.to_vec();
            Ok(Image { pixels, palette: None, width: w, height: ht, bytes_per_pixel: 3, format: if h[5] == 1 { 5 } else { 6 }, pitch })
        }
        0x1a | 0x1b | 0x1c => {
            let format = match kind {
                0x1a => 15,
                0x1b => 16,
                _ => 17,
            };
            let h = cur.take(0xe)?;
            let (w, ht) = (rd32(h, 6)?, rd32(h, 10)?);
            let size = block_size(format, w, ht);
            let mut pixels = Vec::new();
            if h[5] & 1 == 0 {
                let n = rd32(h, 1)? as usize;
                pixels.extend_from_slice(cur.take(n.saturating_sub(0xe))?);
            } else {
                let total = rd32(cur.take(4)?, 0)? as usize;
                let mut done = 0;
                while done < total {
                    let got = inflate(cur.block()?);
                    if got.is_empty() {
                        return Err(AimError::ShortBlock);
                    }
                    done += got.len();
                    pixels.extend_from_slice(&got);
                }
            }
            Ok(Image { pixels, palette: None, width: w, height: ht, bytes_per_pixel: 0, format, pitch: size })
        }
        0x21 => {
            let h = cur.take(0x16)?;
            let (w, ht, pitch) = (rd32(h, 10)?, rd32(h, 14)?, rd32(h, 18)?);
            let format = if h[5] & 1 == 0 { 4 } else { 3 };
            let size = pitch as usize * ht as usize;
            if size > MAX_BYTES {
                return Err(AimError::TooLarge);
            }
            let mut pixels = vec![0u8; size];
            if h[5] & 4 == 0 {
                let per_plane = size / 4;
                for plane in 0..4 {
                    let mut remaining = per_plane;
                    let mut at = 0;
                    while remaining > 0 {
                        let want = remaining.min(0x10000);
                        let got = inflate(cur.block()?);
                        if got.len() < want {
                            return Err(AimError::ShortBlock);
                        }
                        for (i, b) in got[..want].iter().enumerate() {
                            pixels[(at + i) * 4 + plane] = *b;
                        }
                        at += want;
                        remaining -= want;
                    }
                }
            } else {
                let mut data = Vec::new();
                blocks(&mut cur, size, 0x10000, &mut data)?;
                pixels.copy_from_slice(&data);
            }
            // Ajuste de profundidad por canal; el orden de bytes del píxel es b0..b3 con cabecera +8, +7, +6, +9.
            let depth = [h[8], h[7], h[6], h[9]];
            let shift = depth.map(|d| 8u32.wrapping_sub(u32::from(d)));
            if shift.iter().any(|s| *s != 0) {
                let fill = |d: u8| -> [u8; 2] { [0, (0xffu32 >> (u32::from(d) & 31)) as u8] };
                let mut tables = depth.map(fill);
                if h[9] == 0 {
                    tables[3] = [0xff, 0xff];
                }
                for px in pixels.chunks_exact_mut(4) {
                    for c in 0..4 {
                        let v = (u32::from(px[c]) << (shift[c] & 31)) & 0xff;
                        px[c] = v as u8 | tables[c][(v >> 7) as usize];
                    }
                }
            }
            Ok(Image { pixels, palette: None, width: w, height: ht, bytes_per_pixel: 4, format, pitch })
        }
        0x22 => {
            let h = cur.take(0x15)?;
            let (w, ht, pitch) = (rd32(h, 9)?, rd32(h, 13)?, rd32(h, 17)?);
            let mut palette = inflate(cur.block()?);
            palette.resize(0x400, 0);
            let mut pixels = Vec::new();
            blocks(&mut cur, pitch as usize * ht as usize, 0x10000, &mut pixels)?;
            Ok(Image { pixels, palette: Some(palette), width: w, height: ht, bytes_per_pixel: 1, format: 1, pitch })
        }
        other => Err(AimError::Unsupported(other)),
    }
}

/// Escritor de bits inverso de [`Bits`]: palabras `u32` little-endian, bit bajo primero.
struct BitWriter {
    out: Vec<u8>,
    word: u64,
    used: u32,
}

impl BitWriter {
    fn put(&mut self, value: u32, n: u32) {
        debug_assert!(n <= 32);
        let v = if n == 32 { u64::from(value) } else { u64::from(value & ((1u32 << n) - 1)) };
        self.word |= v << self.used;
        self.used += n;
        while self.used >= 32 {
            self.out.extend_from_slice(&(self.word as u32).to_le_bytes());
            self.word >>= 32;
            self.used -= 32;
        }
    }

    fn finish(mut self) -> Vec<u8> {
        if self.used > 0 {
            self.out.extend_from_slice(&(self.word as u32).to_le_bytes());
        }
        self.out
    }
}

/// Distancia máxima que admite el código de copia (clase 7: `2^9 - 2 + 255 + 1`).
pub const MAX_DISTANCE: usize = 510;
const MAX_MATCH: usize = 1 << 12;

/// Comprime un bloque en el formato que lee [`inflate`] (y `FUN_10011070` de `AIM.dll`): LZ77 voraz con ventana de
/// [`MAX_DISTANCE`] bytes. Un bloque vacío se escribe con cabecera 0.
pub fn deflate(data: &[u8]) -> Vec<u8> {
    let mut head = (data.len() as i32).to_le_bytes().to_vec();
    if data.is_empty() {
        return head;
    }
    let mut w = BitWriter { out: Vec::new(), word: 0, used: 0 };
    // Cadenas por los dos primeros bytes: `last[h]` es la posición más reciente con ese par y `prev[p]` la anterior.
    const NONE: usize = usize::MAX;
    let mut last = vec![NONE; 1 << 16];
    let mut prev = vec![NONE; data.len()];
    let mut inserted = 0;
    let pair = |p: usize| usize::from(data[p]) << 8 | usize::from(data[p + 1]);
    let mut i = 0;
    while i < data.len() {
        while inserted < i && inserted + 1 < data.len() {
            let h = pair(inserted);
            prev[inserted] = last[h];
            last[h] = inserted;
            inserted += 1;
        }
        let (mut best_len, mut best_dist) = (0usize, 0usize);
        let limit = (data.len() - i).min(MAX_MATCH);
        if limit >= 2 {
            let mut cand = last[pair(i)];
            while cand != NONE && i - cand <= MAX_DISTANCE {
                let d = i - cand;
                let mut l = 0;
                // La copia del original es byte a byte: con solape repite el patrón, igual que comparar con `data`.
                while l < limit && data[i + l] == data[cand + l] {
                    l += 1;
                }
                if l > best_len {
                    best_len = l;
                    best_dist = d;
                    if l == limit {
                        break;
                    }
                }
                cand = prev[cand];
            }
        }
        if best_len < 2 {
            w.put(0, 1);
            w.put(u32::from(data[i]), 8);
            i += 1;
            continue;
        }
        let e = (best_dist - 1) as u32;
        let class = (0..8u32).find(|k| e <= (1u32 << (k + 2)) - 3).unwrap();
        w.put(1, 1);
        w.put(class, 3);
        w.put(e - ((1u32 << (class + 1)) - 2), class + 1);
        let mut rest = (best_len - 2) as u32;
        let mut width = 2;
        loop {
            let mask = (1u32 << width) - 1;
            if rest >= mask {
                w.put(mask, width);
                rest -= mask;
                width += 1;
            } else {
                w.put(rest, width);
                break;
            }
        }
        i += best_len;
    }
    head.extend_from_slice(&w.finish());
    head
}

fn push_block(out: &mut Vec<u8>, data: &[u8]) {
    let b = deflate(data);
    out.extend_from_slice(&(b.len() as u32).to_le_bytes());
    out.extend_from_slice(&b);
}

fn put32(out: &mut [u8], at: usize, v: u32) {
    out[at..at + 4].copy_from_slice(&v.to_le_bytes());
}

/// Escribe un `.aim` que `AIM.dll` (y [`decode`]) leen con los mismos campos de `AIM_IMAGE` y los mismos píxeles.
///
/// Cabecera común a los 3.409 `.aim` ingleses: `"AIM\0"`, versión 3, tamaño restante, `"lala"` y ceros hasta el byte 41.
/// Subcapa según `format`: 1 → `0x22` (paleta), 3/4 → `0x21` continuo con profundidad 8 en los cuatro canales (los píxeles
/// ya llevan aplicado el reescalado, así que el original no vuelve a tocarlos), 5/6 → `0x16` sin comprimir, 15/16/17 →
/// `0x1a`/`0x1b`/`0x1c` comprimidos. Los bloques se cortan cada `0x10000` bytes, como en el corpus.
pub fn encode(img: &Image) -> Result<Vec<u8>, AimError> {
    let mut sub: Vec<u8>;
    let need = |n: usize| if img.pixels.len() < n { Err(AimError::ShortBlock) } else { Ok(&img.pixels[..n]) };
    match img.format {
        1 => {
            let pal = img.palette.as_deref().ok_or(AimError::Truncated)?;
            if pal.len() < 0x400 {
                return Err(AimError::Truncated);
            }
            sub = vec![0u8; 0x15];
            sub[0] = 0x22;
            sub[5..9].copy_from_slice(&[8, 8, 8, 8]);
            put32(&mut sub, 9, img.width);
            put32(&mut sub, 13, img.height);
            put32(&mut sub, 17, img.pitch);
            push_block(&mut sub, &pal[..0x400]);
            for c in need(img.pitch as usize * img.height as usize)?.chunks(0x10000) {
                push_block(&mut sub, c);
            }
        }
        3 | 4 => {
            sub = vec![0u8; 0x16];
            sub[0] = 0x21;
            sub[5] = 4 | 2 | u8::from(img.format == 3);
            sub[6..10].copy_from_slice(&[8, 8, 8, 8]);
            put32(&mut sub, 10, img.width);
            put32(&mut sub, 14, img.height);
            put32(&mut sub, 18, img.pitch);
            for c in need(img.pitch as usize * img.height as usize)?.chunks(0x10000) {
                push_block(&mut sub, c);
            }
        }
        5 | 6 => {
            sub = vec![0u8; 0x12];
            sub[0] = 0x16;
            sub[5] = if img.format == 5 { 1 } else { 2 };
            put32(&mut sub, 6, img.width);
            put32(&mut sub, 10, img.height);
            put32(&mut sub, 14, img.pitch);
            sub.extend_from_slice(need(img.pitch as usize * img.height as usize)?);
        }
        15 | 16 | 17 => {
            sub = vec![0u8; 0xe];
            sub[0] = 0x1a + (img.format - 15) as u8;
            sub[5] = 1;
            put32(&mut sub, 6, img.width);
            put32(&mut sub, 10, img.height);
            sub.extend_from_slice(&(img.pixels.len() as u32).to_le_bytes());
            for c in img.pixels.chunks(0x10000) {
                push_block(&mut sub, c);
            }
        }
        other => return Err(AimError::Unsupported(other as u8)),
    }
    let n = sub.len() as u32;
    put32(&mut sub, 1, n);
    let mut file = vec![0u8; SUBLAYER_OFFSET];
    file[..5].copy_from_slice(b"AIM\0\x03");
    put32(&mut file, 5, (SUBLAYER_OFFSET + sub.len() - 4) as u32);
    file[9..13].copy_from_slice(b"lala");
    file.extend_from_slice(&sub);
    Ok(file)
}
