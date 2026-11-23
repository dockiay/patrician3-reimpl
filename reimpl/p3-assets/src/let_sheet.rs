//! Hojas de sprites `.let` (`sprite_reader_load_sheet`, `0x00551000`, y trozos extra, `0x005513F0`).
//!
//! Cada valor `u8`/`u16` ocupa una ranura de 4 bytes; el original lo guarda con signo extendido (en las 934 muestras la parte alta es
//! la extensión de signo del valor de 16 bits), de modo que los campos con signo se escriben igual. Disposición:
//!
//! * `00 52`, `u16` identificador de tile, `u8` (campo `+0x2D`), clave de color `u32`, `u16` `+0x22`, `u16` `+0x24`, ancho, alto,
//!   `u8` a, `u8` b (ranuras de 4 bytes).
//! * Matriz contada: `u32 K` y `K` `u32`.
//! * `a + b - 1` tiras de cuatro `u16` (ranuras de 4 bytes).
//! * Ruta `.aim` fuente: `u32 L` y `L` bytes.
//! * Trozos extra: `u32 N` y `N` trozos. Cada trozo es `u32` etiqueta, matriz contada con `u16` de recuento, `u16 f0`, `u16 f1`,
//!   `u16 m`, `m` marcos de cuatro `u16` y, si `L2 != 0`, `u32 L2`, `L2` bytes de ruta y un `u32` (índice de textura).
//!   Las etiquetas 0, 1 y 2 las trata el EXE (animación, mapa de niebla `fogmaps/` y tabla de cinco huecos); las demás, de 3 a 8 en
//!   las muestras, las lee un método virtual de la clase derivada que no se ha identificado, y el EXE las salta con este mismo formato.

use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum LetError {
    BadMagic,
    Truncated,
    /// El contador de tiras `a + b` es cero (el original leería un recuento negativo).
    EmptyStrips,
    TrailingBytes(usize),
    /// Un valor no cabe en el formato.
    TooLarge,
}

impl fmt::Display for LetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LetError::BadMagic => write!(f, "la hoja no empieza por 00 52"),
            LetError::Truncated => write!(f, "hoja truncada"),
            LetError::EmptyStrips => write!(f, "cero tiras"),
            LetError::TrailingBytes(n) => write!(f, "{n} bytes tras el último trozo"),
            LetError::TooLarge => write!(f, "valor demasiado grande"),
        }
    }
}

impl std::error::Error for LetError {}

/// Un marco o tira: cuatro `u16`.
pub type Frame = [i16; 4];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Texture {
    /// Ruta (Latin-1) tal como está en el archivo, con su NUL si lo tiene.
    pub path: Vec<u8>,
    pub index: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Chunk {
    pub tag: u32,
    pub array: Vec<u32>,
    pub f0: i16,
    pub f1: i16,
    pub frames: Vec<Frame>,
    pub texture: Option<Texture>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LetSheet {
    pub tile_id: u16,
    pub field_2d: u8,
    pub color_key: u32,
    pub field_22: i16,
    pub field_24: i16,
    pub tile_width: i16,
    pub tile_height: i16,
    pub strips_a: u8,
    pub strips_b: u8,
    pub matrix: Vec<u32>,
    pub strips: Vec<Frame>,
    /// Ruta del `.aim` fuente, sin el NUL final.
    pub source: Vec<u8>,
    pub chunks: Vec<Chunk>,
}

struct Reader<'a> {
    b: &'a [u8],
    o: usize,
}

impl Reader<'_> {
    fn u32(&mut self) -> Result<u32, LetError> {
        let v = self.b.get(self.o..self.o + 4).ok_or(LetError::Truncated)?;
        self.o += 4;
        Ok(u32::from_le_bytes(v.try_into().unwrap()))
    }
    /// `u16` en una ranura de 4 bytes.
    fn slot16(&mut self) -> Result<i16, LetError> {
        let v = self.b.get(self.o..self.o + 2).ok_or(LetError::Truncated)?;
        if self.b.len() < self.o + 4 {
            return Err(LetError::Truncated);
        }
        self.o += 4;
        Ok(i16::from_le_bytes(v.try_into().unwrap()))
    }
    fn frames(&mut self, n: usize) -> Result<Vec<Frame>, LetError> {
        (0..n).map(|_| Ok([self.slot16()?, self.slot16()?, self.slot16()?, self.slot16()?])).collect()
    }
    fn bytes(&mut self, n: usize) -> Result<&[u8], LetError> {
        let v = self.b.get(self.o..self.o.checked_add(n).ok_or(LetError::Truncated)?).ok_or(LetError::Truncated)?;
        self.o += n;
        Ok(v)
    }
}

impl LetSheet {
    pub fn parse(b: &[u8]) -> Result<Self, LetError> {
        if b.len() < 2 || b[0] != 0 || b[1] != 0x52 {
            return Err(LetError::BadMagic);
        }
        let mut r = Reader { b, o: 2 };
        let tile_id = u16::from_le_bytes(b.get(2..4).ok_or(LetError::Truncated)?.try_into().unwrap());
        r.o = 4;
        let field_2d = *b.get(4).ok_or(LetError::Truncated)?;
        r.o = 8;
        let color_key = r.u32()?;
        let field_22 = r.slot16()?;
        let field_24 = r.slot16()?;
        let tile_width = r.slot16()?;
        let tile_height = r.slot16()?;
        let strips_a = r.slot16()? as u8;
        let strips_b = r.slot16()? as u8;
        let k = r.u32()? as usize;
        let matrix = (0..k).map(|_| r.u32()).collect::<Result<Vec<_>, _>>()?;
        let count = (strips_a as usize + strips_b as usize).checked_sub(1).ok_or(LetError::EmptyStrips)?;
        let strips = r.frames(count)?;
        let l = r.u32()? as usize;
        let source = r.bytes(l)?.to_vec();
        let n = r.u32()?;
        let mut chunks = Vec::new();
        for _ in 0..n {
            let tag = r.u32()?;
            let len = r.slot16()? as u16 as usize;
            let array = (0..len).map(|_| r.u32()).collect::<Result<Vec<_>, _>>()?;
            let f0 = r.slot16()?;
            let f1 = r.slot16()?;
            let m = r.slot16()? as u16 as usize;
            let frames = r.frames(m)?;
            let l2 = r.u32()? as usize;
            let texture = if l2 == 0 {
                None
            } else {
                let path = r.bytes(l2)?.to_vec();
                Some(Texture { path, index: r.u32()? })
            };
            chunks.push(Chunk { tag, array, f0, f1, frames, texture });
        }
        if r.o != b.len() {
            return Err(LetError::TrailingBytes(b.len() - r.o));
        }
        Ok(LetSheet { tile_id, field_2d, color_key, field_22, field_24, tile_width, tile_height, strips_a, strips_b, matrix, strips, source, chunks })
    }

    /// Etiqueta y desplazamiento (justo después de la etiqueta) de cada trozo extra del fichero: el flujo que reciben los lectores de trozos del EXE
    /// (`0x5852A0` para las etiquetas 0/2 y para las 3 y 4 de `0x54E310`). Recorre el mismo formato que `parse`.
    pub fn chunk_streams(b: &[u8]) -> Result<Vec<(u32, usize)>, LetError> {
        let sheet = Self::parse(b)?;
        let mut r = Reader { b, o: 8 };
        r.u32()?;
        for _ in 0..5 {
            r.slot16()?;
        }
        r.slot16()?;
        let k = r.u32()? as usize;
        r.o += 4 * k;
        r.o += 16 * (usize::from(sheet.strips_a) + usize::from(sheet.strips_b) - 1);
        let l = r.u32()? as usize;
        r.o += l;
        let n = r.u32()?;
        let mut out = Vec::new();
        for _ in 0..n {
            let tag = r.u32()?;
            out.push((tag, r.o));
            let len = r.slot16()? as u16 as usize;
            r.o += 4 * len;
            r.slot16()?;
            r.slot16()?;
            let m = r.slot16()? as u16 as usize;
            r.o += 16 * m;
            let l2 = r.u32()? as usize;
            if l2 != 0 {
                r.o += l2 + 4;
            }
        }
        Ok(out)
    }

    pub fn write(&self) -> Result<Vec<u8>, LetError> {
        fn slot(out: &mut Vec<u8>, v: i16) {
            out.extend_from_slice(&i32::from(v).to_le_bytes());
        }
        fn frames(out: &mut Vec<u8>, f: &[Frame]) {
            for fr in f {
                for &v in fr {
                    slot(out, v);
                }
            }
        }
        let mut o = vec![0, 0x52];
        o.extend_from_slice(&self.tile_id.to_le_bytes());
        o.extend_from_slice(&u32::from(self.field_2d).to_le_bytes());
        o.extend_from_slice(&self.color_key.to_le_bytes());
        for v in [self.field_22, self.field_24, self.tile_width, self.tile_height, i16::from(self.strips_a), i16::from(self.strips_b)] {
            slot(&mut o, v);
        }
        o.extend_from_slice(&u32::try_from(self.matrix.len()).map_err(|_| LetError::TooLarge)?.to_le_bytes());
        for v in &self.matrix {
            o.extend_from_slice(&v.to_le_bytes());
        }
        if self.strips.len() + 1 != self.strips_a as usize + self.strips_b as usize {
            return Err(LetError::TooLarge);
        }
        frames(&mut o, &self.strips);
        o.extend_from_slice(&u32::try_from(self.source.len()).map_err(|_| LetError::TooLarge)?.to_le_bytes());
        o.extend_from_slice(&self.source);
        o.extend_from_slice(&u32::try_from(self.chunks.len()).map_err(|_| LetError::TooLarge)?.to_le_bytes());
        for c in &self.chunks {
            o.extend_from_slice(&c.tag.to_le_bytes());
            slot(&mut o, i16::try_from(c.array.len()).map_err(|_| LetError::TooLarge)?);
            for v in &c.array {
                o.extend_from_slice(&v.to_le_bytes());
            }
            slot(&mut o, c.f0);
            slot(&mut o, c.f1);
            slot(&mut o, i16::try_from(c.frames.len()).map_err(|_| LetError::TooLarge)?);
            frames(&mut o, &c.frames);
            match &c.texture {
                None => o.extend_from_slice(&0u32.to_le_bytes()),
                Some(t) => {
                    o.extend_from_slice(&u32::try_from(t.path.len()).map_err(|_| LetError::TooLarge)?.to_le_bytes());
                    o.extend_from_slice(&t.path);
                    o.extend_from_slice(&t.index.to_le_bytes());
                }
            }
        }
        Ok(o)
    }
}
