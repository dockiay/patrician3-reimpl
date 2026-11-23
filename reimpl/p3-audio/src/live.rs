//! Mezclador en tiempo real para el cliente: lo que hacía `mss32.dll` (Miles Sound System) detrás de las funciones `AIL_*`.
//!
//! El enlazador (`p3-mvp/src/ui_rt/miles.rs`) traduce cada importación `AIL_*` del EXE a una llamada de [`Mixer`]; el
//! cliente nativo (`p3-client`) abre la salida de audio del sistema y le pide muestras con [`Mixer::render`]. Sin
//! dispositivo, `render` también sirve para pruebas fuera de línea.
//!
//! Contrato propio (no paridad bit a bit con Miles): volúmenes y panorámica 0..127 de Miles (64 = centro), estados
//! `SMP_DONE` = 2, `SMP_PLAYING` = 4, `SMP_STOPPED` = 8, bucles de stream con 0 = infinitos, posiciones en milisegundos
//! `floor(fotogramas·1000/frecuencia)`. Remuestreo lineal a la frecuencia de salida y mezcla en coma flotante. Los efectos
//! (WAV PCM8/16 e IMA ADPCM de Miles) se descodifican al cargarlos con [`crate::native::stream::decode_stream`]; los streams
//! MP3 (Symphonia) se descodifican en un hilo aparte y empiezan a sonar con el primer tramo, sin parar el fotograma.
use crate::native::stream::{decode_stream, StreamLimits};
use std::collections::HashMap;
use std::io::Cursor;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock, RwLock};
use symphonia::core::codecs::audio::AudioDecoderOptions;
use symphonia::core::formats::{probe::Hint, FormatOptions, TrackType};
use symphonia::core::io::MediaSourceStream;

pub const SMP_DONE: u32 = 2;
pub const SMP_PLAYING: u32 = 4;
pub const SMP_STOPPED: u32 = 8;
/// Tope de muestras PCM16 en la caché de streams (64 Mi muestras = 128 MiB, unos 12 minutos de estéreo a 44.100 Hz).
pub const CACHE_SAMPLES: usize = 64 << 20;

/// PCM16 intercalado que puede seguir creciendo mientras se descodifica.
#[derive(Default)]
pub struct Track {
    /// (canales, frecuencia), en cuanto se conocen.
    format: OnceLock<(u16, u32)>,
    samples: RwLock<Vec<i16>>,
    done: AtomicBool,
}

impl Track {
    fn finished(format: Option<(u16, u32)>, samples: Vec<i16>) -> Arc<Self> {
        let t = Self::default();
        if let Some(f) = format {
            let _ = t.format.set(f);
        }
        *t.samples.write().unwrap_or_else(|e| e.into_inner()) = samples;
        t.done.store(true, Ordering::Release);
        Arc::new(t)
    }
    fn is_done(&self) -> bool {
        self.done.load(Ordering::Acquire)
    }
    fn frames(&self) -> usize {
        let ch = self.format.get().map_or(1, |f| f.0.max(1)) as usize;
        self.samples.read().map_or(0, |s| s.len() / ch)
    }
    /// Descodifica `bytes` (MP3 progresivo; lo demás de una vez) y lo va publicando. Es el trabajo del hilo de
    /// descodificación nativo; en el navegador (sin hilos) lo reparte [`Mixer::pump_decode`] en pasos.
    fn decode_into(self: &Arc<Self>, bytes: Arc<[u8]>, name: &str) {
        DecodeJob::new(self.clone(), bytes, name).step(usize::MAX);
    }
}

/// Descodificador MP3 que se puede detener y reanudar: cada [`Mp3Job::step`] procesa un número acotado de tramas.
struct Mp3Job {
    format: Box<dyn symphonia::core::formats::FormatReader>,
    decoder: Box<dyn symphonia::core::codecs::audio::AudioDecoder>,
    id: u32,
    pending: Vec<i16>,
    spec: Option<(u16, u32)>,
    chunk: usize,
}

impl Mp3Job {
    fn open(bytes: Arc<[u8]>) -> Option<Self> {
        let mss = MediaSourceStream::new(Box::new(Cursor::new(bytes)), Default::default());
        let mut hint = Hint::new();
        hint.with_extension("mp3");
        let format = symphonia::default::get_probe()
            .probe(&hint, mss, FormatOptions::default(), Default::default())
            .ok()?;
        let track = format.default_track(TrackType::Audio)?;
        let params = track.codec_params.as_ref()?.audio()?.clone();
        let id = track.id;
        let decoder = symphonia::default::get_codecs()
            .make_audio_decoder(&params, &AudioDecoderOptions::default())
            .ok()?;
        Some(Self {
            format,
            decoder,
            id,
            pending: Vec::new(),
            spec: None,
            // Primer tramo pequeño (empieza a sonar enseguida), después medio segundo cada vez.
            chunk: 8192,
        })
    }
    fn flush(&mut self, track: &Track) {
        if !self.pending.is_empty() {
            track
                .samples
                .write()
                .unwrap_or_else(|e| e.into_inner())
                .extend_from_slice(&self.pending);
            self.pending.clear();
        }
    }
    /// Procesa hasta `budget` paquetes; verdadero cuando no queda nada por descodificar.
    fn step(&mut self, track: &Track, budget: usize) -> bool {
        for _ in 0..budget {
            let Ok(Some(packet)) = self.format.next_packet() else {
                self.flush(track);
                return true;
            };
            if packet.track_id != self.id {
                continue;
            }
            let Ok(audio) = self.decoder.decode(&packet) else { continue };
            let (ch, rate) = (audio.spec().channels().count() as u16, audio.spec().rate());
            match self.spec {
                None => {
                    if !matches!(ch, 1 | 2) || rate == 0 {
                        self.flush(track);
                        return true;
                    }
                    self.spec = Some((ch, rate));
                    let _ = track.format.set((ch, rate));
                    self.chunk = self.chunk.max(rate as usize * ch as usize / 8);
                }
                // Cambio de formato a mitad: se corta ahí.
                Some(f) if f != (ch, rate) => {
                    self.flush(track);
                    return true;
                }
                _ => {}
            }
            let at = self.pending.len();
            self.pending.resize(at + audio.samples_interleaved(), 0);
            audio.copy_to_slice_interleaved(&mut self.pending[at..]);
            if self.pending.len() >= self.chunk {
                self.flush(track);
                self.chunk = rate as usize * ch as usize / 2;
            }
        }
        // Presupuesto agotado: publica lo descodificado para que pueda sonar.
        self.flush(track);
        false
    }
}

/// Trabajo de descodificación de un stream: abre el archivo en el primer paso y marca la pista como terminada al acabar.
struct DecodeJob {
    track: Arc<Track>,
    bytes: Option<Arc<[u8]>>,
    name: String,
    mp3: Option<Mp3Job>,
}

impl DecodeJob {
    fn new(track: Arc<Track>, bytes: Arc<[u8]>, name: &str) -> Self {
        Self {
            track,
            bytes: Some(bytes),
            name: name.to_string(),
            mp3: None,
        }
    }
    /// Avanza hasta `budget` paquetes MP3 (los demás formatos, de una vez); verdadero cuando la pista está terminada.
    fn step(&mut self, budget: usize) -> bool {
        if let Some(bytes) = self.bytes.take() {
            let limits = StreamLimits {
                max_encoded_bytes: 64 << 20,
                ..Default::default()
            };
            if !self.name.ends_with(".mp3") || bytes.starts_with(b"RIFF") {
                if let Ok(p) = decode_stream(bytes, &self.name, limits) {
                    let _ = self.track.format.set((p.channels, p.sample_rate));
                    *self.track.samples.write().unwrap_or_else(|e| e.into_inner()) = p.samples;
                }
                self.track.done.store(true, Ordering::Release);
                return true;
            }
            match Mp3Job::open(bytes) {
                Some(j) => self.mp3 = Some(j),
                None => {
                    self.track.done.store(true, Ordering::Release);
                    return true;
                }
            }
        }
        let finished = match self.mp3.as_mut() {
            Some(j) => j.step(&self.track, budget),
            None => true,
        };
        if finished {
            self.track.done.store(true, Ordering::Release);
        }
        finished
    }
}

struct Voice {
    track: Arc<Track>,
    /// Duración en ms conocida antes de terminar de descodificar (cabeceras MP3).
    estimated_ms: Option<u32>,
    state: u32,
    /// Posición en fotogramas de la fuente (fraccionaria por el remuestreo).
    cursor: f64,
    volume: i32,
    pan: i32,
    /// Bucles: 1 = una vez, 0 = infinito; `left` cuenta los que quedan incluyendo el actual.
    loops: u32,
    left: u32,
    /// Posición pedida antes de conocer el formato (ms).
    seek_ms: Option<u32>,
    /// Paso bajo de un polo: frecuencia de corte en Hz y estado por canal.
    lowpass: Option<f32>,
    lp_state: [f32; 2],
    is_stream: bool,
}

impl Voice {
    fn new(is_stream: bool) -> Self {
        Self {
            track: Track::finished(None, Vec::new()),
            estimated_ms: None,
            state: SMP_DONE,
            cursor: 0.0,
            volume: 127,
            pan: 64,
            loops: 1,
            left: 1,
            seek_ms: None,
            lowpass: None,
            lp_state: [0.0; 2],
            is_stream,
        }
    }
    fn rate(&self) -> Option<u32> {
        self.track.format.get().map(|f| f.1.max(1))
    }
    fn total_ms(&self) -> u32 {
        match (self.rate(), self.track.is_done()) {
            (Some(r), true) => (self.track.frames() as u64 * 1000 / u64::from(r)) as u32,
            (Some(r), false) => self
                .estimated_ms
                .unwrap_or((self.track.frames() as u64 * 1000 / u64::from(r)) as u32),
            (None, _) => self.estimated_ms.unwrap_or(0),
        }
    }
    fn current_ms(&self) -> u32 {
        match (self.seek_ms, self.rate()) {
            (Some(ms), _) => ms,
            (None, Some(r)) => (self.cursor as u64 * 1000 / u64::from(r)) as u32,
            (None, None) => 0,
        }
    }
    /// Aplica una posición pendiente en cuanto se conoce la frecuencia.
    fn apply_seek(&mut self) {
        if let (Some(ms), Some(r)) = (self.seek_ms, self.rate()) {
            self.cursor = (u64::from(ms) * u64::from(r) / 1000) as f64;
            self.seek_ms = None;
        }
    }
}

#[derive(Default)]
struct State {
    voices: HashMap<u32, Voice>,
    next: u32,
    master: i32,
    /// Streams ya abiertos por nombre, el más reciente al final (la música y los ambientes se repiten, y el juego abre cada
    /// archivo dos veces: duración y reproducción). Se descartan los más antiguos por encima de [`CACHE_SAMPLES`].
    cache: Vec<(String, Arc<Track>)>,
    /// Fotogramas mezclados desde el principio y pico de la mezcla (diagnóstico).
    rendered_frames: u64,
    peak: f32,
}

/// Mezclador compartido entre el hilo del juego (órdenes `AIL_*`) y el de la salida de audio.
pub struct Mixer {
    rate: u32,
    state: Mutex<State>,
    /// Sin hilos (navegador): la descodificación de MP3 avanza a pasos con [`Mixer::pump_decode`].
    cooperative: AtomicBool,
    jobs: Mutex<Vec<DecodeJob>>,
    service: Mutex<Option<ServiceHook>>,
}

/// Gancho que el enlazador llama en cada orden `AIL_*` ([`Mixer::service`]).
pub type ServiceHook = Arc<dyn Fn() + Send + Sync>;

/// Duración aproximada de un MP3 recorriendo sus cabeceras de trama (sin descodificar).
pub fn mp3_duration_ms(bytes: &[u8]) -> Option<u32> {
    let mut i = 0usize;
    // Etiqueta ID3v2 al principio.
    if bytes.len() > 10 && &bytes[..3] == b"ID3" {
        let s = &bytes[6..10];
        i = 10
            + ((s[0] as usize & 0x7f) << 21
                | (s[1] as usize & 0x7f) << 14
                | (s[2] as usize & 0x7f) << 7
                | (s[3] as usize & 0x7f));
    }
    let (mut samples, mut rate) = (0u64, 0u32);
    while i + 4 <= bytes.len() {
        let h = u32::from_be_bytes(bytes[i..i + 4].try_into().unwrap());
        if h >> 21 != 0x7ff {
            i += 1;
            continue;
        }
        let version = (h >> 19) & 3; // 3 = MPEG1, 2 = MPEG2, 0 = MPEG2.5
        let layer = (h >> 17) & 3; // 1 = Layer III
        let br_index = (h >> 12) & 0xf;
        let sr_index = (h >> 10) & 3;
        let padding = (h >> 9) & 1;
        if version == 1 || layer != 1 || br_index == 0 || br_index == 15 || sr_index == 3 {
            i += 1;
            continue;
        }
        const BR1: [u32; 15] = [0, 32, 40, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320];
        const BR2: [u32; 15] = [0, 8, 16, 24, 32, 40, 48, 56, 64, 80, 96, 112, 128, 144, 160];
        const SR: [u32; 3] = [44100, 48000, 32000];
        let mpeg1 = version == 3;
        let sr = SR[sr_index as usize]
            >> if mpeg1 {
                0
            } else if version == 2 {
                1
            } else {
                2
            };
        let br = if mpeg1 { BR1 } else { BR2 }[br_index as usize] * 1000;
        let per = if mpeg1 { 1152 } else { 576 };
        let len = (per / 8 * br / sr + padding) as usize;
        if len < 4 {
            i += 1;
            continue;
        }
        samples += u64::from(per);
        rate = sr;
        i += len;
    }
    (rate != 0).then(|| (samples * 1000 / u64::from(rate)) as u32)
}

impl Mixer {
    pub fn new(rate: u32) -> Arc<Self> {
        Arc::new(Self {
            rate: rate.max(8000),
            state: Mutex::new(State {
                next: 1,
                master: 127,
                ..Default::default()
            }),
            cooperative: AtomicBool::new(cfg!(target_arch = "wasm32")),
            jobs: Mutex::new(Vec::new()),
            service: Mutex::new(None),
        })
    }
    /// Descodificación sin hilos: por defecto activa en `wasm32` (no hay `thread::spawn`) y apagada en el resto.
    pub fn set_cooperative(&self, on: bool) {
        self.cooperative.store(on, Ordering::Relaxed);
    }
    pub fn cooperative(&self) -> bool {
        self.cooperative.load(Ordering::Relaxed)
    }
    /// Avanza las descodificaciones pendientes (modo cooperativo) con un presupuesto total de paquetes MP3, repartido a
    /// partes iguales (mínimo 2 por trabajo). Devuelve verdadero si todavía queda trabajo.
    pub fn pump_decode(&self, budget: usize) -> bool {
        let mut jobs = self.jobs.lock().unwrap_or_else(|e| e.into_inner());
        let each = (budget / jobs.len().max(1)).max(2);
        jobs.retain_mut(|j| !j.step(each));
        !jobs.is_empty()
    }
    /// Instala el gancho de servicio (en el navegador, bombea el anillo de audio).
    pub fn set_service(&self, hook: Option<ServiceHook>) {
        *self.service.lock().unwrap_or_else(|e| e.into_inner()) = hook;
    }
    /// El enlazador lo llama al recibir cada orden `AIL_*`: el juego las consulta también durante las cargas largas, y así el
    /// anillo no se vacía aunque no se presente ningún fotograma.
    pub fn service(&self) {
        let hook = self.service.lock().unwrap_or_else(|e| e.into_inner()).clone();
        if let Some(h) = hook {
            h();
        }
    }
    pub fn rate(&self) -> u32 {
        self.rate
    }
    fn with<R>(&self, f: impl FnOnce(&mut State) -> R) -> R {
        let mut s = self.state.lock().unwrap_or_else(|e| e.into_inner());
        f(&mut s)
    }
    fn voice<R: Default>(&self, h: u32, f: impl FnOnce(&mut Voice) -> R) -> R {
        self.with(|s| s.voices.get_mut(&h).map(f).unwrap_or_default())
    }

    // ---- muestras (HSAMPLE) ----
    pub fn allocate(&self, stream: bool) -> u32 {
        self.with(|s| {
            let h = s.next;
            s.next += 1;
            s.voices.insert(h, Voice::new(stream));
            h
        })
    }
    pub fn release(&self, h: u32) {
        self.with(|s| s.voices.remove(&h));
    }
    /// `AIL_init_sample`: para y deja la muestra sin datos, volumen y panorámica por defecto.
    pub fn init(&self, h: u32) {
        self.voice(h, |v| *v = Voice::new(v.is_stream));
    }
    /// `AIL_set_sample_file`/`AIL_set_named_sample_file`: descodifica el archivo (WAV o MP3) en el acto. Falso si no se entiende.
    pub fn load(&self, h: u32, bytes: &[u8], name: &str) -> bool {
        let pcm = decode_stream(
            Arc::from(bytes),
            &name.to_ascii_lowercase(),
            StreamLimits {
                max_encoded_bytes: 64 << 20,
                ..Default::default()
            },
        )
        .ok();
        let ok = pcm.is_some();
        let track = match pcm {
            Some(p) => Track::finished(Some((p.channels, p.sample_rate)), p.samples),
            None => Track::finished(None, Vec::new()),
        };
        self.voice(h, |v| {
            v.track = track;
            v.estimated_ms = None;
            v.cursor = 0.0;
            v.seek_ms = None;
            v.state = SMP_DONE;
            v.left = v.loops;
        });
        ok
    }
    /// `AIL_start_sample`/`AIL_start_stream`: desde el principio (o desde la posición fijada antes de empezar).
    pub fn start(&self, h: u32) {
        self.voice(h, |v| {
            if v.seek_ms.is_none() {
                v.cursor = 0.0;
            }
            v.apply_seek();
            v.left = v.loops;
            v.lp_state = [0.0; 2];
            v.state = SMP_PLAYING;
        });
    }
    /// `AIL_stop_sample`: pausa (la posición se conserva).
    pub fn stop(&self, h: u32) {
        self.voice(h, |v| {
            if v.state == SMP_PLAYING {
                v.state = SMP_STOPPED;
            }
        });
    }
    /// `AIL_resume_sample` / reanudar un stream pausado.
    pub fn resume(&self, h: u32) {
        self.voice(h, |v| {
            if v.state == SMP_STOPPED {
                v.state = SMP_PLAYING;
            }
        });
    }
    /// `AIL_end_sample`: termina ya.
    pub fn end(&self, h: u32) {
        self.voice(h, |v| v.state = SMP_DONE);
    }
    pub fn status(&self, h: u32) -> u32 {
        self.with(|s| s.voices.get(&h).map_or(SMP_DONE, |v| v.state))
    }
    pub fn set_volume(&self, h: u32, vol: i32) {
        self.voice(h, |v| v.volume = vol.clamp(0, 127));
    }
    pub fn volume(&self, h: u32) -> i32 {
        self.with(|s| s.voices.get(&h).map_or(0, |v| v.volume))
    }
    pub fn set_pan(&self, h: u32, pan: i32) {
        self.voice(h, |v| v.pan = pan.clamp(0, 127));
    }
    pub fn pan(&self, h: u32) -> i32 {
        self.with(|s| s.voices.get(&h).map_or(64, |v| v.pan))
    }
    pub fn set_loops(&self, h: u32, n: u32) {
        self.voice(h, |v| {
            v.loops = n;
            v.left = n;
        });
    }
    /// `(total, actual)` en milisegundos.
    pub fn ms_position(&self, h: u32) -> (u32, u32) {
        self.with(|s| s.voices.get(&h).map_or((0, 0), |v| (v.total_ms(), v.current_ms())))
    }
    pub fn set_ms_position(&self, h: u32, ms: u32) {
        self.voice(h, |v| {
            v.seek_ms = Some(ms);
            v.apply_seek();
        });
    }
    /// Paso bajo (filtro «LowPass Filter» de Miles) con frecuencia de corte en Hz; `None` lo quita.
    pub fn set_lowpass(&self, h: u32, cutoff: Option<f32>) {
        self.voice(h, |v| v.lowpass = cutoff.filter(|c| c.is_finite() && *c > 0.0));
    }
    pub fn lowpass(&self, h: u32) -> Option<f32> {
        self.with(|s| s.voices.get(&h).and_then(|v| v.lowpass))
    }
    pub fn set_master(&self, vol: i32) {
        self.with(|s| s.master = vol.clamp(0, 127));
    }
    pub fn master(&self) -> i32 {
        self.with(|s| s.master)
    }

    // ---- streams (HSTREAM) ----
    /// `AIL_open_stream`: abre un archivo para reproducir en una voz propia; la descodificación sigue en otro hilo.
    pub fn open_stream(&self, name: &str, bytes: Arc<[u8]>) -> u32 {
        let key = name.to_ascii_lowercase().replace('\\', "/");
        let estimated = if key.ends_with(".mp3") {
            mp3_duration_ms(&bytes)
        } else {
            None
        };
        let track = self.with(|s| {
            let track = match s.cache.iter().position(|(k, _)| *k == key) {
                Some(i) => s.cache.remove(i).1,
                None => {
                    let track = Arc::new(Track::default());
                    let (out, name) = (track.clone(), key.clone());
                    if self.cooperative() {
                        let job = DecodeJob::new(out, bytes, &name);
                        self.jobs.lock().unwrap_or_else(|e| e.into_inner()).push(job);
                    } else {
                        let job = move || out.decode_into(bytes, &name);
                        if let Err(e) = std::thread::Builder::new().name("p3-audio-decode".into()).spawn(job) {
                            eprintln!("p3-audio: sin hilo de descodificación ({e})");
                        }
                    }
                    track
                }
            };
            s.cache.push((key, track.clone()));
            // Descarta los más antiguos que ya no suenan mientras se pase del tope.
            let size = |t: &Arc<Track>| t.samples.read().map_or(0, |v| v.len());
            let mut total: usize = s.cache.iter().map(|(_, t)| size(t)).sum();
            let mut i = 0;
            while total > CACHE_SAMPLES && i + 1 < s.cache.len() {
                if Arc::strong_count(&s.cache[i].1) == 1 {
                    total -= size(&s.cache[i].1);
                    s.cache.remove(i);
                } else {
                    i += 1;
                }
            }
            track
        });
        let h = self.allocate(true);
        self.voice(h, |v| {
            v.track = track;
            v.estimated_ms = estimated;
        });
        h
    }
    /// Espera a que termine la descodificación de la voz (pruebas); falso si no hay sonido.
    pub fn wait_decoded(&self, h: u32) -> bool {
        let Some(track) = self.with(|s| s.voices.get(&h).map(|v| v.track.clone())) else {
            return false;
        };
        while !track.is_done() {
            if self.cooperative() {
                self.pump_decode(usize::MAX);
            } else {
                std::thread::sleep(std::time::Duration::from_millis(2));
            }
        }
        track.frames() > 0
    }
    /// Fotogramas mezclados desde el principio y pico absoluto de la salida (diagnóstico).
    pub fn stats(&self) -> (u64, f32) {
        self.with(|s| (s.rendered_frames, s.peak))
    }
    pub fn playing(&self) -> usize {
        self.with(|s| s.voices.values().filter(|v| v.state == SMP_PLAYING).count())
    }

    /// Mezcla `out.len() / 2` fotogramas estéreo intercalados en coma flotante (−1..1).
    pub fn render(&self, out: &mut [f32]) {
        out.fill(0.0);
        let rate = f64::from(self.rate);
        let frames = out.len() / 2;
        let out_rate = self.rate as f32;
        self.with(|s| {
            let master = s.master as f32 / 127.0;
            for v in s.voices.values_mut() {
                if v.state != SMP_PLAYING {
                    continue;
                }
                let Some(&(channels, src_rate)) = v.track.format.get() else {
                    if v.track.is_done() {
                        v.state = SMP_DONE;
                    }
                    continue;
                };
                v.apply_seek();
                let track = v.track.clone();
                let done = track.is_done();
                let Ok(samples) = track.samples.read() else { continue };
                let ch = channels.max(1) as usize;
                let n = samples.len() / ch;
                if n == 0 {
                    if done {
                        v.state = SMP_DONE;
                    }
                    continue;
                }
                let step = f64::from(src_rate) / rate;
                // Volumen lineal 0..127 y panorámica 0..127 de Miles (64 = las dos salidas a pleno).
                let g = v.volume as f32 / 127.0 * master;
                let pan = v.pan as f32;
                let (gl, gr) = (
                    g * if pan <= 64.0 { 1.0 } else { (127.0 - pan) / 63.0 },
                    g * if pan >= 64.0 { 1.0 } else { pan / 64.0 },
                );
                let alpha = v
                    .lowpass
                    .map(|c| 1.0 - (-2.0 * std::f32::consts::PI * c / out_rate).exp());
                let at = |k: usize, c: usize| f32::from(samples[k * ch + c.min(ch - 1)]) / 32768.0;
                for f in 0..frames {
                    let mut i = v.cursor as usize;
                    if i >= n {
                        if !done {
                            // El descodificador todavía no ha llegado: silencio sin avanzar.
                            break;
                        }
                        // Fin del pase: otro bucle o terminar.
                        if v.loops == 0 || v.left > 1 {
                            if v.left > 1 {
                                v.left -= 1;
                            }
                            v.cursor -= n as f64;
                            if v.cursor < 0.0 || v.cursor as usize >= n {
                                v.cursor = 0.0;
                            }
                            i = v.cursor as usize;
                        } else {
                            v.state = SMP_DONE;
                            v.cursor = n as f64;
                            break;
                        }
                    }
                    let t = (v.cursor - i as f64) as f32;
                    let j = if i + 1 < n { i + 1 } else { i };
                    let mut l = at(i, 0) + (at(j, 0) - at(i, 0)) * t;
                    let mut r = at(i, 1) + (at(j, 1) - at(i, 1)) * t;
                    if let Some(a) = alpha {
                        v.lp_state[0] += a * (l - v.lp_state[0]);
                        v.lp_state[1] += a * (r - v.lp_state[1]);
                        l = v.lp_state[0];
                        r = v.lp_state[1];
                    }
                    out[2 * f] += l * gl;
                    out[2 * f + 1] += r * gr;
                    v.cursor += step;
                }
            }
            let mut peak = s.peak;
            for x in out.iter_mut() {
                peak = peak.max(x.abs());
                *x = x.clamp(-1.0, 1.0);
            }
            s.peak = peak;
            s.rendered_frames += frames as u64;
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn wav(rate: u32, samples: &[i16]) -> Vec<u8> {
        let data: Vec<u8> = samples.iter().flat_map(|s| s.to_le_bytes()).collect();
        let mut w = Vec::new();
        w.extend_from_slice(b"RIFF");
        w.extend_from_slice(&(36 + data.len() as u32).to_le_bytes());
        w.extend_from_slice(b"WAVEfmt ");
        w.extend_from_slice(&16u32.to_le_bytes());
        w.extend_from_slice(&1u16.to_le_bytes());
        w.extend_from_slice(&1u16.to_le_bytes());
        w.extend_from_slice(&rate.to_le_bytes());
        w.extend_from_slice(&(rate * 2).to_le_bytes());
        w.extend_from_slice(&2u16.to_le_bytes());
        w.extend_from_slice(&16u16.to_le_bytes());
        w.extend_from_slice(b"data");
        w.extend_from_slice(&(data.len() as u32).to_le_bytes());
        w.extend_from_slice(&data);
        w
    }

    #[test]
    fn sample_plays_resampled_and_finishes() {
        let m = Mixer::new(44100);
        let h = m.allocate(false);
        assert!(m.load(h, &wav(22050, &[16384; 2205]), "x.wav"));
        assert_eq!(m.ms_position(h), (100, 0));
        m.start(h);
        assert_eq!(m.status(h), SMP_PLAYING);
        let mut out = vec![0.0; 2 * 2000];
        m.render(&mut out);
        assert!(
            (out[0] - 0.5).abs() < 1e-3 && (out[1] - 0.5).abs() < 1e-3,
            "{:?}",
            &out[..2]
        );
        assert_eq!(m.ms_position(h).1, 45);
        m.render(&mut out);
        m.render(&mut out);
        assert_eq!(m.status(h), SMP_DONE);
    }

    #[test]
    fn volume_pan_stop_and_loops() {
        let m = Mixer::new(22050);
        let h = m.allocate(false);
        assert!(m.load(h, &wav(22050, &[16384; 100]), "x.wav"));
        m.set_volume(h, 0);
        m.set_pan(h, 127);
        m.set_loops(h, 3);
        m.start(h);
        let mut out = vec![0.0; 2 * 250];
        m.render(&mut out);
        assert!(out.iter().all(|x| *x == 0.0));
        assert_eq!(m.status(h), SMP_PLAYING);
        m.set_volume(h, 127);
        m.render(&mut out);
        assert_eq!(out[0], 0.0);
        assert!(out[1] > 0.4);
        assert_eq!(m.status(h), SMP_DONE);
        let h2 = m.allocate(false);
        assert!(m.load(h2, &wav(22050, &[1000; 100]), "x.wav"));
        m.start(h2);
        m.stop(h2);
        m.render(&mut out);
        assert_eq!(m.status(h2), SMP_STOPPED);
        m.resume(h2);
        assert_eq!(m.status(h2), SMP_PLAYING);
    }

    #[test]
    fn stream_decodes_in_background_and_loops_forever() {
        let m = Mixer::new(22050);
        let s = m.open_stream("musik/a.wav", Arc::from(wav(22050, &[2000; 50]).as_slice()));
        assert!(m.wait_decoded(s));
        m.set_loops(s, 0);
        m.start(s);
        let mut out = vec![0.0; 2 * 1000];
        m.render(&mut out);
        assert_eq!(m.status(s), SMP_PLAYING);
        assert!(out[2 * 999] > 0.0);
        m.set_ms_position(s, 1);
        assert_eq!(m.ms_position(s), (2, 0));
    }
}
