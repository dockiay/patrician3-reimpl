//! Ficheros de ciudad: mapa `.town`, puntos de aparición `.spawn` y caminos `.nodes`
//! (`class22_load_map_file` `0x0058E8C0`, `class22_read_map_start_point` `0x0058E760`, `class22_load_town_scene_data` `0x0058EB30`).
//!
//! `.town`: `u16` cabecera, `u32` ancho, `u32` alto, dos capas de `ancho * alto` celdas de 4 bytes (`u16`, `u8`, `u8`) y un pie:
//! primer par `u32` (posición inicial de la vista) y, si el fichero lo trae, segundo par `i32` (punto de inicio).
//! `.spawn`: secciones `{clave u32, n u32, n × (u32, u32)}` hasta el final. `.nodes`: `{clave u32, n u32, n × {m u32, m × (u32, u32)}}`.

use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum TownError {
    Truncated,
    /// El tamaño declarado no cabe en memoria o desborda el cálculo.
    TooLarge,
    /// Hay bytes tras la última sección (un fichero que el original leería a medias).
    TrailingBytes(usize),
}

impl fmt::Display for TownError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TownError::Truncated => write!(f, "fichero truncado"),
            TownError::TooLarge => write!(f, "tamaño excesivo"),
            TownError::TrailingBytes(n) => write!(f, "{n} bytes sobrantes"),
        }
    }
}

struct Reader<'a> {
    data: &'a [u8],
    at: usize,
}

impl Reader<'_> {
    fn u16(&mut self) -> Result<u16, TownError> {
        let s = self.data.get(self.at..self.at + 2).ok_or(TownError::Truncated)?;
        self.at += 2;
        Ok(u16::from_le_bytes([s[0], s[1]]))
    }

    fn u32(&mut self) -> Result<u32, TownError> {
        let s = self.data.get(self.at..self.at + 4).ok_or(TownError::Truncated)?;
        self.at += 4;
        Ok(u32::from_le_bytes([s[0], s[1], s[2], s[3]]))
    }

    fn left(&self) -> usize {
        self.data.len() - self.at
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub tile: u16,
    pub b2: u8,
    pub b3: u8,
}

/// Mayor identificador de ciudad cuyo mapa conserva sus dimensiones al cargarse.
pub const MAX_SIZED_TOWN_ID: u32 = 0xC0;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TownMap {
    pub header: u16,
    pub width: u32,
    pub height: u32,
    pub layers: [Vec<Cell>; 2],
    pub view_start: (u32, u32),
    /// Segundo par del pie (`class22_read_map_start_point`); ausente en ficheros más cortos.
    pub start_point: Option<(i32, i32)>,
}

impl TownMap {
    pub fn parse(data: &[u8]) -> Result<Self, TownError> {
        let mut r = Reader { data, at: 0 };
        let header = r.u16()?;
        let (width, height) = (r.u32()?, r.u32()?);
        let cells = (width as usize).checked_mul(height as usize).ok_or(TownError::TooLarge)?;
        if cells.checked_mul(8).ok_or(TownError::TooLarge)? > r.left() {
            return Err(TownError::Truncated);
        }
        let layer = |r: &mut Reader| -> Vec<Cell> {
            (0..cells)
                .map(|_| {
                    let s = &r.data[r.at..r.at + 4];
                    r.at += 4;
                    Cell { tile: u16::from_le_bytes([s[0], s[1]]), b2: s[2], b3: s[3] }
                })
                .collect()
        };
        let layers = [layer(&mut r), layer(&mut r)];
        let view_start = (r.u32()?, r.u32()?);
        let start_point = if r.left() >= 8 { Some((r.u32()? as i32, r.u32()? as i32)) } else { None };
        match r.left() {
            0 => Ok(TownMap { header, width, height, layers, view_start, start_point }),
            n => Err(TownError::TrailingBytes(n)),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(10 + 8 * self.layers[0].len() + 16);
        out.extend_from_slice(&self.header.to_le_bytes());
        out.extend_from_slice(&self.width.to_le_bytes());
        out.extend_from_slice(&self.height.to_le_bytes());
        for layer in &self.layers {
            for c in layer {
                out.extend_from_slice(&c.tile.to_le_bytes());
                out.push(c.b2);
                out.push(c.b3);
            }
        }
        out.extend_from_slice(&self.view_start.0.to_le_bytes());
        out.extend_from_slice(&self.view_start.1.to_le_bytes());
        if let Some((x, y)) = self.start_point {
            out.extend_from_slice(&x.to_le_bytes());
            out.extend_from_slice(&y.to_le_bytes());
        }
        out
    }

    /// Dimensiones que deja el cargador (comparación con signo `cmp 0xC0; jle`): con identificador superior a `0xC0` quedan a cero aunque el fichero se lea.
    pub fn loaded_size(&self, town_id: u32) -> (u32, u32) {
        if (town_id as i32) > MAX_SIZED_TOWN_ID as i32 {
            (0, 0)
        } else {
            (self.width, self.height)
        }
    }
}

/// `.spawn`: sección de puntos por clave.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpawnSection {
    pub key: u32,
    pub points: Vec<(u32, u32)>,
}

/// `.nodes`: sección de caminos por clave.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeSection {
    pub key: u32,
    pub paths: Vec<Vec<(u32, u32)>>,
}

/// Claves máximas que caben en las tablas del objeto `Class22` sin pisar otra (el original no las comprueba).
pub const MAX_SPAWN_KEY: u32 = 4;
pub const MAX_NODE_KEY: u32 = 5;

fn points(r: &mut Reader, n: u32) -> Result<Vec<(u32, u32)>, TownError> {
    if (n as usize).checked_mul(8).ok_or(TownError::TooLarge)? > r.left() {
        return Err(TownError::Truncated);
    }
    (0..n).map(|_| Ok((r.u32()?, r.u32()?))).collect()
}

fn put_points(out: &mut Vec<u8>, pts: &[(u32, u32)]) {
    for (a, b) in pts {
        out.extend_from_slice(&a.to_le_bytes());
        out.extend_from_slice(&b.to_le_bytes());
    }
}

pub fn parse_spawn(data: &[u8]) -> Result<Vec<SpawnSection>, TownError> {
    let mut r = Reader { data, at: 0 };
    let mut out = Vec::new();
    while r.left() > 0 {
        let key = r.u32()?;
        let n = r.u32()?;
        out.push(SpawnSection { key, points: points(&mut r, n)? });
    }
    Ok(out)
}

pub fn spawn_to_bytes(sections: &[SpawnSection]) -> Vec<u8> {
    let mut out = Vec::new();
    for s in sections {
        out.extend_from_slice(&s.key.to_le_bytes());
        out.extend_from_slice(&(s.points.len() as u32).to_le_bytes());
        put_points(&mut out, &s.points);
    }
    out
}

pub fn parse_nodes(data: &[u8]) -> Result<Vec<NodeSection>, TownError> {
    let mut r = Reader { data, at: 0 };
    let mut out = Vec::new();
    while r.left() > 0 {
        let key = r.u32()?;
        let n = r.u32()?;
        if (n as usize).checked_mul(4).ok_or(TownError::TooLarge)? > r.left() {
            return Err(TownError::Truncated);
        }
        let mut paths = Vec::with_capacity(n as usize);
        for _ in 0..n {
            let m = r.u32()?;
            paths.push(points(&mut r, m)?);
        }
        out.push(NodeSection { key, paths });
    }
    Ok(out)
}

pub fn nodes_to_bytes(sections: &[NodeSection]) -> Vec<u8> {
    let mut out = Vec::new();
    for s in sections {
        out.extend_from_slice(&s.key.to_le_bytes());
        out.extend_from_slice(&(s.paths.len() as u32).to_le_bytes());
        for p in &s.paths {
            out.extend_from_slice(&(p.len() as u32).to_le_bytes());
            put_points(&mut out, p);
        }
    }
    out
}
