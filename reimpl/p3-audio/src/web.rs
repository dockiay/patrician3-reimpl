//! Backend de audio para el navegador (WEB-5): el mezclador sigue en el Worker del juego y alimenta un [`AudioRing`] en memoria
//! compartida que un `AudioWorklet` vacía. No usa hilos ni `cpal`; el cliente nativo (`p3-client/audio.rs`) no cambia.
//!
//! Uso desde el pegamento WASM (`p3-web`):
//! 1. La página crea el `AudioContext` tras un gesto del usuario y envía al Worker su `sampleRate`.
//! 2. El Worker crea [`WebAudio::new`], instala `rt.miles.mixer = Some(web.mixer())` y [`WebAudio::install_service`] (para que
//!    las órdenes `AIL_*` durante cargas largas sigan alimentando el anillo) y devuelve a la página `web.ring_ptr()`.
//! 3. La página carga `web/p3-audio-worklet.js` y llama a `attachRing(ctx, memory, ptr)` de `web/p3-audio-web.js`.
//! 4. En cada `on_present` el Worker llama a [`WebAudio::pump`].
//!
//! El mezclador se rellena solo hasta [`WebAudio::latency_ms`] por delante del consumidor, así que las órdenes del juego
//! (efectos, clics) suenan con esa latencia. Si el consumidor no corre (contexto suspendido), el anillo se llena y el audio
//! del juego deja de avanzar en lugar de acumularse.
use crate::live::Mixer;
use crate::ring::AudioRing;
use std::sync::{Arc, Mutex};

/// Paquetes MP3 descodificados por llamada a `pump` (≈26 ms de audio cada uno: ~13× tiempo real con 8 a 60 fps).
pub const DECODE_BUDGET: usize = 8;
const BLOCK_FRAMES: usize = 2048;

struct Inner {
    mixer: Arc<Mixer>,
    ring: AudioRing,
    scratch: Vec<f32>,
    target_frames: usize,
    decode_budget: usize,
}

impl Inner {
    fn pump(&mut self) {
        self.mixer.pump_decode(self.decode_budget);
        // Con el consumidor parado no se mezcla de más.
        let mut need = self.target_frames.saturating_sub(self.ring.fill());
        while need > 0 {
            let n = need.min(BLOCK_FRAMES).min(self.ring.free());
            if n == 0 {
                break;
            }
            self.scratch.resize(n * 2, 0.0);
            self.mixer.render(&mut self.scratch);
            let wrote = self.ring.write(&self.scratch);
            need -= wrote.min(need);
        }
    }
}

/// Mezclador + anillo + bombeo. Clonable (comparte estado): una copia va al gancho de servicio del enlazador y otra a `on_present`.
#[derive(Clone)]
pub struct WebAudio {
    inner: Arc<Mutex<Inner>>,
    mixer: Arc<Mixer>,
    ring_ptr: usize,
    ring_bytes: usize,
}

impl WebAudio {
    /// `sample_rate` es el del `AudioContext`; `latency_ms` el colchón (150 es un buen punto de partida).
    pub fn new(sample_rate: u32, latency_ms: u32) -> Self {
        let mixer = Mixer::new(sample_rate);
        mixer.set_cooperative(true);
        let rate = mixer.rate();
        let target_frames = (rate as usize * latency_ms.max(10) as usize / 1000).max(256);
        // Capacidad: 4× el colchón (al menos 8192 fotogramas), potencia de 2 por el anillo.
        let ring = AudioRing::new(rate, (target_frames * 4).max(8192));
        let (ring_ptr, ring_bytes) = (ring.ptr() as usize, ring.byte_len());
        let inner = Inner {
            mixer: mixer.clone(),
            ring,
            scratch: Vec::new(),
            target_frames,
            decode_budget: DECODE_BUDGET,
        };
        Self {
            inner: Arc::new(Mutex::new(inner)),
            mixer,
            ring_ptr,
            ring_bytes,
        }
    }
    pub fn mixer(&self) -> Arc<Mixer> {
        self.mixer.clone()
    }
    /// Dirección del anillo en la memoria lineal (para `attachRing`); estable mientras viva `WebAudio`.
    pub fn ring_ptr(&self) -> usize {
        self.ring_ptr
    }
    pub fn ring_bytes(&self) -> usize {
        self.ring_bytes
    }
    pub fn sample_rate(&self) -> u32 {
        self.mixer.rate()
    }
    pub fn latency_ms(&self) -> u32 {
        let g = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        (g.target_frames * 1000 / self.mixer.rate() as usize) as u32
    }
    /// Cambia el colchón; se queda dentro de la capacidad del anillo.
    pub fn set_latency_ms(&self, ms: u32) {
        let mut g = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        let frames = self.mixer.rate() as usize * ms.max(10) as usize / 1000;
        g.target_frames = frames.clamp(256, g.ring.capacity() / 2);
    }
    pub fn set_decode_budget(&self, packets: usize) {
        self.inner.lock().unwrap_or_else(|e| e.into_inner()).decode_budget = packets.max(1);
    }
    /// Descodifica un poco y rellena el anillo hasta el colchón. Barato si ya está lleno.
    pub fn pump(&self) {
        // `try_lock`: si el gancho de servicio ya está bombeando (reentrada), no se espera.
        if let Ok(mut g) = self.inner.try_lock() {
            g.pump();
        }
    }
    /// Conecta [`Mixer::service`] a [`WebAudio::pump`].
    pub fn install_service(&self) {
        let me = self.clone();
        self.mixer.set_service(Some(Arc::new(move || me.pump())));
    }
    pub fn underruns(&self) -> u32 {
        self.inner.lock().unwrap_or_else(|e| e.into_inner()).ring.underruns()
    }
    pub fn buffered_frames(&self) -> usize {
        self.inner.lock().unwrap_or_else(|e| e.into_inner()).ring.fill()
    }
    /// Solo pruebas: lee del anillo como lo haría el worklet.
    pub fn consume(&self, out: &mut [f32]) -> usize {
        self.inner.lock().unwrap_or_else(|e| e.into_inner()).ring.read(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::live::{SMP_DONE, SMP_PLAYING};

    fn wav(rate: u32, samples: &[i16]) -> Vec<u8> {
        let data: Vec<u8> = samples.iter().flat_map(|s| s.to_le_bytes()).collect();
        let mut v = b"RIFF".to_vec();
        v.extend((36 + data.len() as u32).to_le_bytes());
        v.extend(b"WAVEfmt ");
        v.extend(16u32.to_le_bytes());
        v.extend([1, 0, 1, 0]);
        v.extend(rate.to_le_bytes());
        v.extend((rate * 2).to_le_bytes());
        v.extend([2, 0, 16, 0]);
        v.extend(b"data");
        v.extend((data.len() as u32).to_le_bytes());
        v.extend(data);
        v
    }

    #[test]
    fn music_stream_plays_through_the_ring_without_underruns() {
        let web = WebAudio::new(44100, 150);
        let m = web.mixer();
        assert!(m.cooperative());
        let s = m.open_stream("musik/a.wav", Arc::from(wav(44100, &vec![8000; 44100]).as_slice()));
        // La descodificación no corre sola: la mueve el bombeo.
        assert_eq!(m.status(s), SMP_DONE);
        m.set_loops(s, 0);
        m.start(s);
        web.pump();
        assert_eq!(m.status(s), SMP_PLAYING);
        assert_eq!(web.buffered_frames(), 44100 * 150 / 1000);
        // El «worklet» consume bloques de 128 fotogramas y el juego bombea cada 16 ms (≈705 fotogramas).
        let mut out = vec![0.0; 2 * 128];
        let mut got = 0usize;
        let mut since_pump = 0usize;
        while got < 44100 * 3 {
            let n = web.consume(&mut out);
            assert_eq!(n, 128, "vaciado en el fotograma {got}");
            assert!(out.iter().all(|x| (*x - 0.2441).abs() < 0.01), "muestra {}", out[0]);
            got += n;
            since_pump += n;
            if since_pump >= 705 {
                web.pump();
                since_pump = 0;
            }
        }
        assert_eq!(web.underruns(), 0);
        assert_eq!(m.status(s), SMP_PLAYING, "el bucle infinito sigue sonando");
    }

    #[test]
    fn service_hook_keeps_the_ring_fed_and_stopped_consumer_does_not_advance_audio() {
        let web = WebAudio::new(48000, 100);
        web.install_service();
        let m = web.mixer();
        let s = m.open_stream("musik/b.wav", Arc::from(wav(48000, &vec![4000; 48000]).as_slice()));
        m.start(s);
        // Una orden AIL (como las del juego durante una carga) basta para llenar el anillo.
        m.service();
        assert_eq!(web.buffered_frames(), 4800);
        let (before, _) = m.stats();
        for _ in 0..100 {
            m.service();
        }
        assert_eq!(m.stats().0, before, "sin consumidor, el audio no avanza");
        // Latencia ajustable dentro de la capacidad.
        web.set_latency_ms(10_000);
        assert!(web.latency_ms() <= 2 * 100 * 4 / 2 * 10);
    }

    #[test]
    fn pointer_and_header_are_exposed_for_the_worklet() {
        let web = WebAudio::new(44100, 120);
        assert_eq!(web.ring_ptr() % 4, 0);
        assert!(web.ring_bytes() > 8192 * 8);
        let header = unsafe { std::slice::from_raw_parts(web.ring_ptr() as *const u32, crate::ring::HEADER_WORDS) };
        assert_eq!(header[crate::ring::W_MAGIC], crate::ring::MAGIC ^ crate::ring::VERSION);
        assert_eq!(header[crate::ring::W_RATE], 44100);
        assert!(header[crate::ring::W_CAPACITY].is_power_of_two());
    }
}
