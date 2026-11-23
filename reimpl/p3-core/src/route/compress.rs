//! Compresor del borde del motor (`0x0054C410` → núcleo `0x006381E0`), el que escribe los `.rou` y otros ficheros del juego.
//! Referencia: x86 original en emulación (`reimpl/oracle/x86_route.py`).
//!
//! Formato (el que lee [`super::rou::decompress`]): cabecera `i32` con el tamaño original (positivo, flujo de bits en palabras
//! `u32`) o su negativo (bytes sin comprimir). El flujo codifica literales (`0` + 8 bits) y coincidencias (`1`, cubo de distancia de
//! 3 bits, bits de distancia y longitud con ampliación progresiva). Busca por cadenas de hash de los dos primeros bytes con una
//! ventana de 510 posiciones, sin umbral de longitud mínima, y vuelve al bloque sin comprimir si el flujo llega a ocupar tanto
//! como la entrada en una escritura de palabra.

const WINDOW: usize = 0x1FE;

struct BitWriter {
    out: Vec<u8>,
    count: u32,
    buffer: u32,
    input_len: usize,
}

impl BitWriter {
    /// `0x00638728`: añade `bits` bits de `value` (menos significativos primero). `false` si el flujo ya no es menor que la entrada.
    fn put(&mut self, value: u32, bits: u32) -> bool {
        self.buffer |= value.checked_shl(self.count).unwrap_or(0);
        self.count += bits;
        if self.count < 32 {
            return true;
        }
        self.out.extend_from_slice(&self.buffer.to_le_bytes());
        if self.input_len <= self.out.len() {
            return false;
        }
        if self.count == 32 {
            self.count = 0;
            self.buffer = 0;
        } else {
            self.count -= 32;
            self.buffer = value >> (bits - self.count);
        }
        true
    }
}

#[derive(Clone, Copy)]
struct Entry {
    pos: usize,
    next: Option<usize>,
    newer: Option<usize>,
}

struct Chains {
    heads: Vec<Option<usize>>,
    pool: Vec<Entry>,
    cursor: usize,
    processed: usize,
}

impl Chains {
    fn hash(src: &[u8], p: usize) -> usize {
        src[p] as usize | (src.get(p + 1).copied().unwrap_or(0) as usize) << 8
    }

    /// `0x00638434`: inserta la posición `p` y, con la ventana llena, retira la entrada más antigua.
    fn insert(&mut self, src: &[u8], p: usize) {
        if self.processed >= WINDOW {
            if self.cursor == WINDOW {
                self.cursor = 0;
            }
            let old = self.pool[self.cursor];
            match old.newer {
                None => {
                    let h = Self::hash(src, old.pos);
                    self.heads[h] = None;
                }
                Some(n) => self.pool[n].next = None,
            }
        }
        let h = Self::hash(src, p);
        let old_head = self.heads[h];
        if let Some(o) = old_head {
            self.pool[o].newer = Some(self.cursor);
        }
        self.heads[h] = Some(self.cursor);
        self.pool[self.cursor] = Entry {
            pos: p,
            next: old_head,
            newer: None,
        };
        self.cursor += 1;
    }
}

fn raw_block(src: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(src.len() + 4);
    out.extend_from_slice(&(-(src.len() as i32)).to_le_bytes());
    out.extend_from_slice(src);
    out
}

/// `0x0054C410`: comprime `src` y devuelve el bloque completo (cabecera incluida). Menos de 16 bytes, o un flujo que no es menor que
/// la entrada, dan el bloque sin comprimir. Con `src` vacío el original no está definido (aquí devuelve el bloque vacío).
pub fn compress(src: &[u8]) -> Vec<u8> {
    let len = src.len();
    if len < 0x10 {
        return raw_block(src);
    }
    let mut w = BitWriter {
        out: Vec::with_capacity(len),
        count: 0,
        buffer: 0,
        input_len: len,
    };
    let mut chains = Chains {
        heads: vec![None; 0x10000],
        pool: vec![
            Entry {
                pos: 0,
                next: None,
                newer: None
            };
            WINDOW
        ],
        cursor: 0,
        processed: 0,
    };
    macro_rules! put {
        ($v:expr, $b:expr) => {
            if !w.put($v, $b) {
                return raw_block(src);
            }
        };
    }
    put!(len as u32, 32);
    let mut p = 0usize;
    put!((src[0] as u32) << 1, 9);
    chains.insert(src, 0);
    p += 1;
    chains.processed += 1;
    let last = len - 1;
    while p < last {
        // 0x006384D4: elige literal o coincidencia en `p`
        let advance;
        let mut best_pos = 0usize;
        let mut best_len = 0usize;
        let mut matched = false;
        let first = src[p] as usize | (src[p + 1] as usize) << 8;
        if (first >> 8) == (first & 0xFF) {
            let mut k = 1usize;
            let mut q = p + 2;
            while q < len && src[q] == src[p + k] {
                q += 1;
                k += 1;
            }
            if k >= 2 {
                best_pos = p;
                best_len = k;
                put!((src[p] as u32) << 1, 9);
                chains.insert(src, p);
                p += 1;
                chains.processed += 1;
                matched = true;
            }
        }
        if !matched {
            match chains.heads[first] {
                None => {
                    put!((src[p] as u32) << 1, 9);
                    advance = 1;
                    for _ in 0..advance {
                        chains.insert(src, p);
                        p += 1;
                        chains.processed += 1;
                    }
                    continue;
                }
                Some(mut e) => {
                    best_len = 0;
                    loop {
                        let cand = chains.pool[e].pos;
                        let mut q = p + 2;
                        let mut k = 2usize;
                        while q < len && src[q] == src[cand + k] {
                            q += 1;
                            k += 1;
                        }
                        if q >= len {
                            best_pos = cand;
                            best_len = k;
                            break;
                        }
                        if best_len < k {
                            best_pos = cand;
                            best_len = k;
                        }
                        match chains.pool[e].next {
                            Some(n) => e = n,
                            None => break,
                        }
                    }
                }
            }
        }
        // 0x006385AA: distancia y longitud
        let d = (p - best_pos - 1) as u32;
        match d {
            0..=1 => put!(1 | d << 4, 5),
            2..=5 => put!(3 | (d - 2) << 4, 6),
            6..=0xD => put!(5 | (d - 6) << 4, 7),
            0xE..=0x1D => put!(7 | (d - 0xE) << 4, 8),
            0x1E..=0x3D => put!(9 | (d - 0x1E) << 4, 9),
            0x3E..=0x7D => put!(0xB | (d - 0x3E) << 4, 10),
            0x7E..=0xFD => put!(0xD | (d - 0x7E) << 4, 11),
            _ => put!(0xF | (d - 0xFE) << 4, 12),
        }
        let total = best_len;
        let mut rest = best_len as u32 - 2;
        let mut width = 2u32;
        loop {
            let ones = (1u32 << width) - 1;
            if rest < ones {
                break;
            }
            rest -= ones;
            put!(ones, width);
            width += 1;
        }
        put!(rest, width);
        for _ in 0..total {
            chains.insert(src, p);
            p += 1;
            chains.processed += 1;
        }
    }
    if p != len {
        put!((src[p] as u32) << 1, 9);
    }
    w.out.extend_from_slice(&w.buffer.to_le_bytes());
    w.out
}
