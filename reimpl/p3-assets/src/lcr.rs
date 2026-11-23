//! Conjuntos de sprites de criaturas `.lcr` (`entity_sprite_loader_load`, `0x00558770`).
//!
//! Disposición (cada entero es un `u32` little-endian salvo que se diga lo contrario):
//!
//! * `00 06`, dos bytes de cabecera; `+4` clase de entidad (`sprite_entity_factory`, `0x00557D10`: 0 pobre, 1 medio, 2 rico, 3 niños, 4 mendigo…),
//!   `+8`, `+0xC`, `+0x10` campos sin significado conocido (se guardan tal cual) y `+0x14` número de direcciones `N` (potencia de dos).
//! * Desde `+0x18`: imágenes por segundo (el intervalo del original es `1000 / fps`, mínimo 1 ms), un byte «con celdas» y la cabecera
//!   extra de la clase (`vtable+4`: `4·k` bytes; `k = 1` para las clases 0-4 de las figuras que caminan).
//! * Tabla de estados: recuento y tripletas `{primero, último, id}` (`0xC` bytes por estado, `0x557730` busca por `id`).
//! * Capa de máscara: `u32 L` y ruta `.aim`; si `L != 0` cada fotograma lleva su rectángulo de máscara.
//! * Por dirección: recuento de fotogramas y, por fotograma, ancla `(ax, ay)`, máscara opcional (6 palabras), celdas opcionales
//!   (`u32 K` y `K` rectángulos `x, y, ancho, alto`) y el rectángulo de la hoja `x0, y0, x1, y1` (inclusivo).
//! * Ruta `.aim` de la hoja (`u32 L2`), recuento de registros extra (`vtable+8`) y esos registros (aquí solo se cuentan).
//!
//! La dirección visible de una entidad con rumbo `h` (byte) es `((h·N + 0x80) >> 8) & (N-1)` (`0x559260`).

use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum LcrError {
    BadMagic,
    Truncated,
    TrailingBytes(usize),
    /// El número de direcciones no es potencia de dos o es cero.
    BadDirections(u32),
}

impl fmt::Display for LcrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LcrError::BadMagic => write!(f, "el archivo no empieza por 00 06"),
            LcrError::Truncated => write!(f, "archivo truncado"),
            LcrError::TrailingBytes(n) => write!(f, "{n} bytes tras el último registro"),
            LcrError::BadDirections(n) => write!(f, "número de direcciones no válido: {n}"),
        }
    }
}

impl std::error::Error for LcrError {}

/// Un estado de animación: fotogramas `first..=last` de cada dirección, identificado por `id` (los 5 bits bajos del código de estado).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct State {
    pub first: u32,
    pub last: u32,
    pub id: u32,
}

/// Rectángulo de la hoja con su desplazamiento respecto al ancla: `{x, y, ancho, alto, dx, dy}` (`i16` como en el original).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SheetRect {
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
    pub dx: i16,
    pub dy: i16,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Frame {
    pub anchor: (u32, u32),
    pub rect: SheetRect,
    /// Rectángulo de la capa de máscara (sombra), si la hoja la tiene.
    pub mask: Option<SheetRect>,
    /// Rectángulos de celdas ocupadas (`x, y, ancho, alto` con desplazamiento) y el índice del que contiene el ancla.
    pub cells: Vec<SheetRect>,
    pub anchor_cell: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Lcr {
    pub kind: u32,
    pub field_8: u32,
    pub field_c: u32,
    pub field_10: u32,
    pub directions: u32,
    pub interval_ms: u32,
    pub has_cells: bool,
    pub extra_header: Vec<u32>,
    pub states: Vec<State>,
    pub mask_path: Vec<u8>,
    pub dirs: Vec<Vec<Frame>>,
    pub path: Vec<u8>,
    pub extra_records: u32,
}

struct Reader<'a> {
    b: &'a [u8],
    o: usize,
}

impl Reader<'_> {
    fn u32(&mut self) -> Result<u32, LcrError> {
        let v = self.b.get(self.o..self.o + 4).ok_or(LcrError::Truncated)?;
        self.o += 4;
        Ok(u32::from_le_bytes(v.try_into().unwrap()))
    }
    fn bytes(&mut self, n: usize) -> Result<&[u8], LcrError> {
        let v = self.b.get(self.o..self.o.checked_add(n).ok_or(LcrError::Truncated)?).ok_or(LcrError::Truncated)?;
        self.o += n;
        Ok(v)
    }
}

impl Lcr {
    /// Interpreta un `.lcr`. `extra_header_words` es el tamaño (en palabras) de la cabecera extra de la clase; `strict` exige que el
    /// archivo termine justo tras los registros extra (con registros extra solo se cuenta su número y se ignora el resto).
    pub fn parse(b: &[u8], extra_header_words: usize) -> Result<Self, LcrError> {
        if b.len() < 2 || b[0] != 0 || b[1] != 6 {
            return Err(LcrError::BadMagic);
        }
        let mut r = Reader { b, o: 4 };
        let kind = r.u32()?;
        let field_8 = r.u32()?;
        let field_c = r.u32()?;
        let field_10 = r.u32()?;
        let directions = r.u32()?;
        if directions == 0 || !directions.is_power_of_two() {
            return Err(LcrError::BadDirections(directions));
        }
        let fps = r.u32()?;
        let has_cells = *r.bytes(1)?.first().unwrap() != 0;
        let interval_ms = if fps != 0 { 1000 / fps } else { fps };
        let interval_ms = interval_ms.max(1);
        let extra_header = (0..extra_header_words).map(|_| r.u32()).collect::<Result<Vec<_>, _>>()?;
        let count = r.u32()? as usize;
        let mut states = Vec::new();
        for _ in 0..count {
            states.push(State { first: r.u32()?, last: r.u32()?, id: r.u32()? });
        }
        let l = r.u32()? as usize;
        let mask_path = r.bytes(l)?.to_vec();
        let has_mask = l != 0;
        let mut dirs = Vec::new();
        for _ in 0..directions {
            let n = r.u32()? as usize;
            let mut frames = Vec::new();
            for _ in 0..n {
                let ax = r.u32()?;
                let ay = r.u32()?;
                let mask = if has_mask {
                    let (x, y, w, h) = (r.u32()? as u16, r.u32()? as u16, r.u32()? as u16, r.u32()? as u16);
                    let (ox, oy) = (r.u32()?, r.u32()?);
                    Some(SheetRect { x, y, w, h, dx: (ox as i16).wrapping_sub(ax as i16), dy: (oy as i16).wrapping_sub(ay as i16) })
                } else {
                    None
                };
                let mut cells = Vec::new();
                let mut anchor_cell = 0;
                if has_cells {
                    let k = r.u32()?;
                    for j in 0..k {
                        let (x, y, w, h) = (r.u32()? as u16, r.u32()? as u16, r.u32()? as u16, r.u32()? as u16);
                        if (x as i32) <= ax as i32 && (ax as i32) < w as i32 + x as i32 {
                            anchor_cell = j;
                        }
                        cells.push(SheetRect { x, y, w, h, dx: x.wrapping_sub(ax as u16) as i16, dy: y.wrapping_sub(ay as u16) as i16 });
                    }
                }
                let (x0, y0, x1, y1) = (r.u32()? as u16, r.u32()? as u16, r.u32()? as u16, r.u32()? as u16);
                let rect = SheetRect {
                    x: x0,
                    y: y0,
                    w: x1.wrapping_sub(x0).wrapping_add(1),
                    h: y1.wrapping_sub(y0).wrapping_add(1),
                    dx: x0.wrapping_sub(ax as u16) as i16,
                    dy: y0.wrapping_sub(ay as u16) as i16,
                };
                frames.push(Frame { anchor: (ax, ay), rect, mask, cells, anchor_cell });
            }
            dirs.push(frames);
        }
        let l2 = r.u32()? as usize;
        let path = r.bytes(l2)?.to_vec();
        let extra_records = r.u32()?;
        Ok(Lcr { kind, field_8, field_c, field_10, directions, interval_ms, has_cells, extra_header, states, mask_path, dirs, path, extra_records })
    }

    /// Dirección que se muestra con el rumbo `heading` (`0x559260`).
    pub fn direction_of(&self, heading: u8) -> usize {
        (((heading as u32 * self.directions + 0x80) >> 8) & (self.directions - 1)) as usize
    }

    /// Estado cuyo `id` es `id & 0x1F`.
    pub fn state_index(&self, id: u32) -> Option<usize> {
        self.states.iter().position(|s| s.id == (id & 0x1F))
    }

    /// Ruta `.aim` sin NUL final.
    pub fn sheet_path(&self) -> &[u8] {
        self.path.split(|&c| c == 0).next().unwrap_or(&[])
    }
}

/// Palabras de la cabecera extra de cada clase (`vtable+4` de la clase, que avanza el cursor de lectura), **medidas en el ejecutable**:
/// `oracle/x86_sprite_set.py` ejecuta el cargador real `0x558770` sobre los 267 `.lcr` del contenedor inglés y mide cuánto avanza el cursor.
/// Cubre las clases con ficheros; la 12 y la 33 no tienen ninguno en el contenedor. Las cuatro hojas `*_schaden00_segel00` (clase 13)
/// cargan con 5 palabras en el resto de su clase, pero el original las procesa con objetos estáticos de casco que el oráculo no inicia.
pub fn extra_header_words(kind: u32) -> Option<usize> {
    match kind {
        0..=4 | 7 | 14 | 20 | 22 | 24 | 25 | 28 | 30..=32 | 34..=36 => Some(1),
        8 | 18 => Some(2),
        5 | 6 | 21 | 26 | 27 | 29 => Some(3),
        23 => Some(4),
        9..=11 | 13 | 19 => Some(5),
        15 => Some(6),
        16 | 17 => Some(21),
        _ => None,
    }
}

impl Lcr {
    /// Imagen canónica del conjunto de sprites tal como la deja el cargador `0x558770` en memoria (sin punteros): cabecera de ocho palabras
    /// `{+4, +8, +0xC, +0x10, direcciones, estados, con_celdas, máx. celdas}`, recuentos por dirección, fotogramas de 12 bytes
    /// `{x, y, w, h, dx, dy}`, indicador y fotogramas de máscara, celdas (`K`, índice del ancla y rectángulos) y estados.
    /// Es la que compara `tests/sprite_sets.rs` con el original.
    pub fn set_image(&self) -> Vec<u8> {
        let mut o = Vec::new();
        let w32 = |o: &mut Vec<u8>, v: u32| o.extend_from_slice(&v.to_le_bytes());
        let rect = |o: &mut Vec<u8>, r: &SheetRect| {
            for v in [r.x, r.y, r.w, r.h] {
                o.extend_from_slice(&v.to_le_bytes());
            }
            for v in [r.dx, r.dy] {
                o.extend_from_slice(&v.to_le_bytes());
            }
        };
        let max_cells = self.dirs.iter().flatten().map(|f| f.cells.len() as u32).max().unwrap_or(0);
        for v in [self.field_c, self.kind, self.field_10, self.interval_ms, self.directions, self.states.len() as u32, self.has_cells as u32, if self.has_cells { max_cells } else { 0 }] {
            w32(&mut o, v);
        }
        for d in &self.dirs {
            w32(&mut o, d.len() as u32);
        }
        for d in &self.dirs {
            for f in d {
                rect(&mut o, &f.rect);
            }
        }
        let masked = !self.mask_path.is_empty();
        w32(&mut o, masked as u32);
        if masked {
            for d in &self.dirs {
                for f in d {
                    rect(&mut o, f.mask.as_ref().expect("máscara"));
                }
            }
        }
        if self.has_cells {
            for d in &self.dirs {
                for f in d {
                    w32(&mut o, f.cells.len() as u32);
                    w32(&mut o, f.anchor_cell);
                    for c in &f.cells {
                        rect(&mut o, c);
                    }
                }
            }
        }
        for s in &self.states {
            for v in [s.first, s.last, s.id] {
                w32(&mut o, v);
            }
        }
        o
    }
}
