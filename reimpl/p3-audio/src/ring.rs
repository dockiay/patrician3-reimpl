//! Anillo de audio de un productor y un consumidor sobre memoria compartida.
//!
//! En el navegador el productor es el Worker del juego (WASM, memoria compartida) y el consumidor un `AudioWorklet` que lee la
//! misma memoria con `Int32Array`/`Float32Array` (`web/p3-audio-worklet.js`). Toda la estructura es un único bloque de palabras
//! de 32 bits atómicas, de modo que su dirección ([`AudioRing::ptr`]) basta para que JS construya las vistas. Fuera de WASM
//! sirve igual (pruebas, otros destinos): [`AudioRing::read`] hace lo mismo que el worklet.
//!
//! Disposición (palabras `u32`; JS la repite en `web/p3-audio-ring.js`, cualquier cambio debe subir [`VERSION`]):
//!
//! | palabra | contenido |
//! |---|---|
//! | 0 | [`MAGIC`] ^ [`VERSION`] |
//! | 1 | capacidad en fotogramas (potencia de 2) |
//! | 2 | posición de escritura, en fotogramas, con desbordamiento |
//! | 3 | posición de lectura, en fotogramas, con desbordamiento |
//! | 4 | frecuencia de muestreo (Hz) |
//! | 5 | contador de vaciados (lecturas que no encontraron datos suficientes) |
//! | 6 | banderas del consumidor: 1 = el worklet está funcionando |
//! | 7 | reservada |
//! | 8.. | `capacidad × 2` muestras f32 (izquierda, derecha) como bits `u32` |
use std::sync::atomic::{AtomicU32, Ordering};

pub const MAGIC: u32 = 0x5033_4152; // "P3AR"
pub const VERSION: u32 = 1;
pub const HEADER_WORDS: usize = 8;
pub const W_MAGIC: usize = 0;
pub const W_CAPACITY: usize = 1;
pub const W_WRITE: usize = 2;
pub const W_READ: usize = 3;
pub const W_RATE: usize = 4;
pub const W_UNDERRUNS: usize = 5;
pub const W_FLAGS: usize = 6;
pub const FLAG_CONSUMER_RUNNING: u32 = 1;

pub struct AudioRing {
    words: Box<[AtomicU32]>,
    mask: u32,
}

impl AudioRing {
    /// `capacity_frames` se redondea a potencia de 2 (mínimo 256).
    pub fn new(sample_rate: u32, capacity_frames: usize) -> Self {
        let cap = capacity_frames.max(256).next_power_of_two();
        let words: Box<[AtomicU32]> = (0..HEADER_WORDS + cap * 2).map(|_| AtomicU32::new(0)).collect();
        words[W_MAGIC].store(MAGIC ^ VERSION, Ordering::Relaxed);
        words[W_CAPACITY].store(cap as u32, Ordering::Relaxed);
        words[W_RATE].store(sample_rate, Ordering::Relaxed);
        Self {
            words,
            mask: cap as u32 - 1,
        }
    }
    /// Dirección del bloque (en WASM, un desplazamiento de la memoria lineal compartida; alineada a 4 bytes).
    pub fn ptr(&self) -> *const u32 {
        self.words.as_ptr().cast()
    }
    pub fn byte_len(&self) -> usize {
        self.words.len() * 4
    }
    pub fn capacity(&self) -> usize {
        self.mask as usize + 1
    }
    pub fn sample_rate(&self) -> u32 {
        self.words[W_RATE].load(Ordering::Relaxed)
    }
    /// Fotogramas escritos y todavía sin leer.
    pub fn fill(&self) -> usize {
        let w = self.words[W_WRITE].load(Ordering::Acquire);
        let r = self.words[W_READ].load(Ordering::Acquire);
        w.wrapping_sub(r).min(self.mask + 1) as usize
    }
    pub fn free(&self) -> usize {
        self.capacity() - self.fill()
    }
    pub fn underruns(&self) -> u32 {
        self.words[W_UNDERRUNS].load(Ordering::Relaxed)
    }
    pub fn consumer_running(&self) -> bool {
        self.words[W_FLAGS].load(Ordering::Relaxed) & FLAG_CONSUMER_RUNNING != 0
    }
    /// Productor: añade `samples` (estéreo intercalado) y devuelve los fotogramas aceptados (los que caben).
    pub fn write(&self, samples: &[f32]) -> usize {
        let frames = (samples.len() / 2).min(self.free());
        let w = self.words[W_WRITE].load(Ordering::Relaxed);
        for f in 0..frames {
            let at = HEADER_WORDS + ((w.wrapping_add(f as u32) & self.mask) as usize) * 2;
            self.words[at].store(samples[2 * f].to_bits(), Ordering::Relaxed);
            self.words[at + 1].store(samples[2 * f + 1].to_bits(), Ordering::Relaxed);
        }
        self.words[W_WRITE].store(w.wrapping_add(frames as u32), Ordering::Release);
        frames
    }
    /// Consumidor (lo que hace el worklet): llena `out` (estéreo intercalado); lo que falte queda en silencio y cuenta un vaciado.
    pub fn read(&self, out: &mut [f32]) -> usize {
        let want = out.len() / 2;
        let frames = want.min(self.fill());
        let r = self.words[W_READ].load(Ordering::Relaxed);
        for f in 0..frames {
            let at = HEADER_WORDS + ((r.wrapping_add(f as u32) & self.mask) as usize) * 2;
            out[2 * f] = f32::from_bits(self.words[at].load(Ordering::Relaxed));
            out[2 * f + 1] = f32::from_bits(self.words[at + 1].load(Ordering::Relaxed));
        }
        out[2 * frames..].fill(0.0);
        self.words[W_READ].store(r.wrapping_add(frames as u32), Ordering::Release);
        if frames < want {
            self.words[W_UNDERRUNS].fetch_add(1, Ordering::Relaxed);
        }
        frames
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wraps_around_and_counts_underruns() {
        let ring = AudioRing::new(48000, 256);
        assert_eq!(ring.capacity(), 256);
        let mut pos = 0f32;
        let mut out = vec![0.0; 2 * 100];
        // Muchas vueltas: las posiciones desbordan el índice pero no el contador.
        for _ in 0..50 {
            let block: Vec<f32> = (0..2 * 100)
                .map(|i| {
                    pos += 1.0;
                    pos + i as f32 * 0.0
                })
                .collect();
            assert_eq!(ring.write(&block), 100);
            assert_eq!(ring.read(&mut out), 100);
            assert_eq!(out, block);
        }
        assert_eq!(ring.underruns(), 0);
        assert_eq!(ring.read(&mut out), 0);
        assert_eq!(ring.underruns(), 1);
        assert!(out.iter().all(|x| *x == 0.0));
        // Escritura que no cabe: se acepta solo lo que hay libre.
        assert_eq!(ring.write(&vec![1.0; 2 * 1000]), 256);
        assert_eq!(ring.free(), 0);
    }

    #[test]
    fn positions_survive_u32_wrap() {
        let ring = AudioRing::new(44100, 256);
        ring.words[W_WRITE].store(u32::MAX - 10, Ordering::Relaxed);
        ring.words[W_READ].store(u32::MAX - 10, Ordering::Relaxed);
        let block: Vec<f32> = (0..2 * 40).map(|i| i as f32).collect();
        assert_eq!(ring.write(&block), 40);
        assert_eq!(ring.fill(), 40);
        let mut out = vec![0.0; 2 * 40];
        assert_eq!(ring.read(&mut out), 40);
        assert_eq!(out, block);
    }
}
