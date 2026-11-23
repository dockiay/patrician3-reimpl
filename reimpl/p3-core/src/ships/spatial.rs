//! Índice espacial de colisiones del combate: alta 0x00619450, límites 0x006197A0, baja 0x00619560 y
//! actualización 0x006197F0. Capacidad preasignada (múltiplo de 32); el crecimiento (0x0061A1D0) es aparte.
//!
//! Modela el historial de extremos por eje y las matrices de solape; un par candidato no es un impacto.
//! Referencia: `research/systems/barcos/colisiones`. Fixture: `oracle/x86_battle_spatial.py`.

pub const END: u16 = 0xFFFF;

fn words(capacity: usize) -> Vec<Vec<u32>> {
    vec![vec![0; capacity / 32]; capacity]
}

#[derive(Clone, Debug)]
pub struct Axis {
    pub head: u16,
    pub coordinates: Vec<i16>,
    pub next: Vec<u16>,
    pub previous: Vec<u16>,
    pub overlap: Vec<Vec<u32>>,
}

fn set(row: &mut [u32], bit: usize, on: bool) {
    if on {
        row[bit >> 5] |= 1 << (bit & 31);
    } else {
        row[bit >> 5] &= !(1 << (bit & 31));
    }
}

impl Axis {
    fn new(capacity: usize) -> Self {
        Axis {
            head: END,
            coordinates: vec![0; 2 * capacity],
            next: vec![0; 2 * capacity],
            previous: vec![0; 2 * capacity],
            overlap: words(capacity),
        }
    }

    fn add(&mut self, slot: usize, minimum: i16, maximum: i16) {
        let (low, high) = (slot * 2, slot * 2 + 1);
        self.coordinates[low] = minimum;
        self.coordinates[high] = maximum;
        self.next[high] = self.head;
        self.previous[high] = low as u16;
        if self.head != END {
            self.previous[self.head as usize] = high as u16;
        }
        self.next[low] = high as u16;
        self.previous[low] = END;
        self.head = low as u16;
    }

    fn unlink(&mut self, endpoint: usize) {
        let following = self.next[endpoint];
        let before = self.previous[endpoint];
        if self.head as usize == endpoint {
            self.head = following;
        } else {
            self.next[before as usize] = following;
        }
        if following != END {
            self.previous[following as usize] = before;
        }
    }

    fn remove(&mut self, slot: usize) {
        self.unlink(2 * slot + 1);
        self.unlink(2 * slot);
        for w in self.overlap[slot].iter_mut() {
            *w = 0;
        }
        // La limpieza de columnas del original enmascara el índice a 13 bits.
        let bit = slot & 0x1FFF;
        if bit < self.overlap.len() {
            for row in self.overlap.iter_mut() {
                set(row, bit, false);
            }
        }
    }

    /// Intercambios adyacentes repetidos y estables, con actualización de matrices según la paridad.
    fn sort(&mut self) {
        if self.head == END {
            return;
        }
        loop {
            let mut changed = false;
            let mut before = END;
            let mut left = self.head;
            let mut right = self.next[left as usize];
            while right != END {
                let following = self.next[right as usize];
                if self.coordinates[right as usize] < self.coordinates[left as usize] {
                    changed = true;
                    if before == END {
                        self.head = right;
                    } else {
                        self.next[before as usize] = right;
                    }
                    self.next[left as usize] = following;
                    self.previous[left as usize] = right;
                    self.next[right as usize] = left;
                    self.previous[right as usize] = before;
                    if following != END {
                        self.previous[following as usize] = left;
                    }
                    let (a, b) = ((right >> 1) as usize, (left >> 1) as usize);
                    if right & 1 == 0 && left & 1 != 0 {
                        set(&mut self.overlap[a], b, true);
                        set(&mut self.overlap[b], a, true);
                    } else if right & 1 != 0 && left & 1 == 0 {
                        set(&mut self.overlap[a], b, false);
                        set(&mut self.overlap[b], a, false);
                    }
                    before = right;
                } else {
                    before = left;
                    left = right;
                }
                right = following;
            }
            if !changed {
                return;
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct CollisionIndex {
    pub capacity: usize,
    pub span: u16,
    pub free_head: u16,
    pub free_next: Vec<u16>,
    pub types: Vec<u8>,
    pub references: Vec<u16>,
    pub x: Axis,
    pub y: Axis,
    pub pairs: Vec<Vec<u32>>,
}

impl CollisionIndex {
    pub fn new(capacity: usize) -> Result<Self, &'static str> {
        if capacity == 0 || capacity % 32 != 0 || capacity > 32736 {
            return Err("capacidad preasignada: múltiplo de 32, como máximo 32736");
        }
        let mut free_next: Vec<u16> = (1..capacity as u16).collect();
        free_next.push(END);
        Ok(CollisionIndex {
            capacity,
            span: 0,
            free_head: 0,
            free_next,
            types: vec![0; capacity],
            references: vec![0; capacity],
            x: Axis::new(capacity),
            y: Axis::new(capacity),
            pairs: words(capacity),
        })
    }

    /// 0x00619450. `None` si la lista libre está vacía (el original ampliaría con 0x0061A1D0).
    pub fn add(&mut self, kind: u8, reference: u8, xmin: i16, ymin: i16, xmax: i16, ymax: i16) -> Option<u16> {
        if self.free_head == END {
            return None;
        }
        let slot = self.free_head as usize;
        self.free_head = self.free_next[slot];
        self.types[slot] = kind;
        self.references[slot] = u16::from(reference);
        self.x.add(slot, xmin, xmax);
        self.y.add(slot, ymin, ymax);
        self.span = self.span.max(slot as u16 + 1);
        Some(slot as u16)
    }

    /// 0x006197A0.
    pub fn bounds(&mut self, slot: u16, xmin: i16, ymin: i16, xmax: i16, ymax: i16) {
        let s = slot as usize;
        self.x.coordinates[2 * s] = xmin;
        self.x.coordinates[2 * s + 1] = xmax;
        self.y.coordinates[2 * s] = ymin;
        self.y.coordinates[2 * s + 1] = ymax;
    }

    /// 0x00619560.
    pub fn remove(&mut self, slot: u16) {
        let s = slot as usize;
        self.free_next[s] = self.free_head;
        self.free_head = slot;
        self.x.remove(s);
        self.y.remove(s);
        if slot + 1 == self.span {
            self.span -= 1; // sin explorar huecos libres anteriores
        }
    }

    /// 0x006197F0: ordena ambos ejes y despacha los pares candidatos fila a fila; el callback puede
    /// modificar los bits de pares, que se releen en vivo. Devuelve las llamadas en orden.
    pub fn update<F: FnMut(&mut Self, u16, u16)>(&mut self, mut callback: F) -> Vec<(u16, u16)> {
        self.x.sort();
        self.y.sort();
        let mut calls = Vec::new();
        let blocks = (self.span as usize + 31) >> 5; // muestreado una vez antes del bucle de filas
        let mut row = 0usize;
        while row < self.span as usize {
            for block in 0..blocks {
                self.pairs[row][block] = self.x.overlap[row][block] & self.y.overlap[row][block];
                for bit in 0..32 {
                    let other = block * 32 + bit;
                    if self.pairs[row][block] & (1 << bit) != 0 {
                        calls.push((row as u16, other as u16));
                        callback(self, row as u16, other as u16);
                    }
                }
            }
            row += 1;
        }
        calls
    }

    /// Bytes canónicos del estado completo (mismo orden que el generador del fixture).
    pub fn canonical(&self) -> Vec<u8> {
        let mut o = Vec::new();
        for v in [self.free_head, self.span, self.x.head, self.y.head] {
            o.extend(v.to_le_bytes());
        }
        o.extend(&self.types);
        for v in &self.references {
            o.extend(v.to_le_bytes());
        }
        for v in &self.free_next {
            o.extend(v.to_le_bytes());
        }
        for a in [&self.x, &self.y] {
            for v in &a.coordinates {
                o.extend(v.to_le_bytes());
            }
            for v in &a.next {
                o.extend(v.to_le_bytes());
            }
            for v in &a.previous {
                o.extend(v.to_le_bytes());
            }
        }
        for matrix in [&self.x.overlap, &self.y.overlap, &self.pairs] {
            for block in 0..self.capacity / 32 {
                for row in matrix.iter() {
                    o.extend(row[block].to_le_bytes());
                }
            }
        }
        o
    }
}
