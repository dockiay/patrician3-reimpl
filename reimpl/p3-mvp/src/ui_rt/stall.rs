//! Muestreo de tirones del cliente en vivo (19/11/2026, «congelados» de Sergio).
//!
//! `p3-juego.log` anota los fotogramas de más de 150 ms («fotograma lento: … juego N») pero no dice qué parte del original los
//! causa. Aquí la pila de llamadas del enlazador ([`super::Runtime`] `stack`) se refleja en una tabla de atómicos (una escritura
//! relajada por llamada) y un hilo aparte la lee cada 5 ms mientras el fotograma en curso dure más de 100 ms. Al presentar un
//! fotograma lento se escribe en el registro qué funciones del original estaban en la pila en la mayoría de las muestras (el
//! camino caliente) y las hojas más vistas. No cambia nada del juego: solo se lee la tabla.
//!
//! Solo lo enciende el cliente ([`super::Runtime::enable_stall_sampler`]); las pruebas y el banco no lo usan. `P3_TIRONES=0` lo
//! apaga. En `wasm32` no hay hilos y no hace nada.

use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering::Relaxed};
use std::sync::Mutex;

/// Profundidad reflejada (las pilas del original en partida rondan 20–60 llamadas).
pub const SLOTS: usize = 192;

pub(super) static ON: AtomicBool = AtomicBool::new(false);
pub(super) static DEPTH: AtomicU32 = AtomicU32::new(0);
pub(super) static FRAMES: [AtomicU32; SLOTS] = [const { AtomicU32::new(0) }; SLOTS];
/// Inicio del fotograma en curso (µs desde [`epoch`]).
static FRAME_START_US: AtomicU64 = AtomicU64::new(0);
static SAMPLES: Mutex<Vec<Vec<u32>>> = Mutex::new(Vec::new());
const MAX_SAMPLES: usize = 400;
/// Una de cada `STRIDE` lecturas se guarda. Al llenarse la tabla se queda una muestra de cada dos y el paso se dobla: un
/// fotograma de 17 s queda cubierto entero (antes solo sus 2 primeros segundos, y el resto de la carga no se veía).
static STRIDE: AtomicU32 = AtomicU32::new(1);
static TICKS: AtomicU32 = AtomicU32::new(0);
/// Tramo de código propio (no del original) en curso: el muestreador lo añade como hoja de la pila, para que un fotograma largo
/// dentro de `sgl_SwapBuffer` diga en qué parte se va (ganchos del remaster, superposiciones, cambio de modo, presentar, esperar).
static TAG: AtomicU32 = AtomicU32::new(0);
const TAG_BASE: u32 = 0xF000_0000;
/// Nombres de los tramos; el índice + 1 es el valor de [`tag`].
pub const TAGS: [&str; 24] = [
    "ganchos", "superposiciones", "al-presentar", "cambio-de-modo", "swap-sgl", "mostrar", "esperar", "entrada", "stats", "lonja",
    "poblacion", "viajeros", "oficina", "abasto", "sucesos", "sucesos-catalogo", "sucesos-dia", "viajeros-dia", "senales", "pausa",
    "idioma", "arranque-sonido", "arranque-ventana", "arranque-juego",
];

/// Guarda de un tramo: al soltarse vuelve el anterior.
pub struct Tag(u32);

impl Drop for Tag {
    fn drop(&mut self) {
        TAG.store(self.0, Relaxed);
    }
}

/// Marca el tramo `name` (uno de [`TAGS`]) hasta que se suelte la guarda. Cuesta dos escrituras atómicas relajadas.
#[inline]
pub fn tag(name: &str) -> Tag {
    let id = TAGS.iter().position(|t| *t == name).map_or(0, |i| i as u32 + 1);
    Tag(TAG.swap(id, Relaxed))
}

fn name(f: u32) -> String {
    match f.checked_sub(TAG_BASE + 1).and_then(|i| TAGS.get(i as usize)) {
        Some(t) => (*t).to_string(),
        None => format!("{f:x}"),
    }
}

/// Informes ya escritos (tope para no llenar el registro).
static REPORTS: AtomicU32 = AtomicU32::new(0);
const MAX_REPORTS: u32 = 60;

#[cfg(not(target_arch = "wasm32"))]
fn epoch() -> &'static std::time::Instant {
    static E: std::sync::OnceLock<std::time::Instant> = std::sync::OnceLock::new();
    E.get_or_init(std::time::Instant::now)
}

#[cfg(not(target_arch = "wasm32"))]
fn now_us() -> u64 {
    epoch().elapsed().as_micros() as u64
}

/// Enciende el muestreo (una vez por proceso).
pub fn enable() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        if std::env::var("P3_TIRONES").is_ok_and(|v| v == "0") || ON.swap(true, Relaxed) {
            return;
        }
        FRAME_START_US.store(now_us(), Relaxed);
        let _ = std::thread::Builder::new().name("p3-tirones".into()).spawn(|| loop {
            std::thread::sleep(std::time::Duration::from_millis(5));
            let start = FRAME_START_US.load(Relaxed);
            if now_us().saturating_sub(start) < 100_000 {
                continue;
            }
            let d = (DEPTH.load(Relaxed) as usize).min(SLOTS);
            let mut stack: Vec<u32> = FRAMES[..d].iter().map(|a| a.load(Relaxed)).collect();
            let t = TAG.load(Relaxed);
            if t != 0 {
                stack.push(TAG_BASE + t);
            }
            if TICKS.fetch_add(1, Relaxed) % STRIDE.load(Relaxed).max(1) != 0 {
                continue;
            }
            if let Ok(mut s) = SAMPLES.lock() {
                if s.len() >= MAX_SAMPLES {
                    let mut k = 0;
                    s.retain(|_| {
                        k += 1;
                        k % 2 == 1
                    });
                    STRIDE.store(STRIDE.load(Relaxed).saturating_mul(2), Relaxed);
                }
                s.push(stack);
            }
        });
    }
}

/// Anota la entrada a una llamada en la profundidad `d` (0 = la primera).
#[inline(always)]
pub(super) fn push(d: usize, f: u32) {
    if d < SLOTS {
        FRAMES[d].store(f, Relaxed);
    }
    DEPTH.store(d as u32 + 1, Relaxed);
}

/// Anota la salida: quedan `d` llamadas en la pila.
#[inline(always)]
pub(super) fn pop(d: usize) {
    DEPTH.store(d as u32, Relaxed);
}

/// Fin de un fotograma (tras presentar y esperar): si duró más de 150 ms y hay muestras, escribe el informe; empieza el siguiente.
pub(super) fn frame_end(frame: u64) {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let now = now_us();
        let start = FRAME_START_US.swap(now, Relaxed);
        let samples = SAMPLES.lock().map(|mut s| std::mem::take(&mut *s)).unwrap_or_default();
        STRIDE.store(1, Relaxed);
        TICKS.store(0, Relaxed);
        let ms = now.saturating_sub(start) / 1000;
        if ms < 150 || samples.is_empty() || REPORTS.fetch_add(1, Relaxed) >= MAX_REPORTS {
            return;
        }
        eprintln!("{}", report(frame, ms, &samples));
        if ms >= 2000 {
            eprintln!("{}", phases(ms, &samples));
        }
    }
    #[cfg(target_arch = "wasm32")]
    let _ = frame;
}

/// Texto del informe: camino común (funciones presentes en al menos 60 % de las muestras, en orden de pila, desde la última que
/// comparten todas) y las cinco hojas más vistas con su porcentaje.
pub fn report(frame: u64, ms: u64, samples: &[Vec<u32>]) -> String {
    use std::collections::HashMap;
    let n = samples.len();
    // Por posición: la función más vista en cada profundidad mientras la comparta al menos el 60 %.
    let mut path = Vec::new();
    for d in 0.. {
        let mut count: HashMap<u32, usize> = HashMap::new();
        for s in samples {
            if let Some(&f) = s.get(d) {
                if s[..d] == path[..] {
                    *count.entry(f).or_default() += 1;
                }
            }
        }
        match count.into_iter().max_by_key(|&(f, c)| (c, f)) {
            Some((f, c)) if c * 10 >= n * 6 => path.push(f),
            _ => break,
        }
    }
    let mut leaves: HashMap<u32, usize> = HashMap::new();
    for s in samples {
        if let Some(&f) = s.last() {
            *leaves.entry(f).or_default() += 1;
        }
    }
    let mut leaves: Vec<_> = leaves.into_iter().collect();
    leaves.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    let tail: Vec<String> = path.iter().skip(path.len().saturating_sub(16)).map(|f| name(*f)).collect();
    let hot: Vec<String> = leaves.iter().take(5).map(|(f, c)| format!("{} {}%", name(*f), c * 100 / n)).collect();
    format!("tirón: fotograma {frame}, {ms} ms, {n} muestras; camino (60 %) …{}; hojas {}", tail.join(">"), hot.join(", "))
}

/// Para fotogramas largos (cargas): el fotograma partido en ocho tramos de tiempo iguales, y de cada uno la hoja más vista y la
/// función más honda que comparte al menos el 60 % de sus muestras. Dice en qué se va cada parte de la carga.
pub fn phases(ms: u64, samples: &[Vec<u32>]) -> String {
    use std::collections::HashMap;
    let n = samples.len();
    let parts = 8.min(n.max(1));
    let mut out = Vec::new();
    for p in 0..parts {
        let chunk = &samples[p * n / parts..(p + 1) * n / parts];
        if chunk.is_empty() {
            continue;
        }
        let mut leaves: HashMap<u32, usize> = HashMap::new();
        for s in chunk {
            *leaves.entry(s.last().copied().unwrap_or(0)).or_default() += 1;
        }
        let (leaf, c) = leaves.into_iter().max_by_key(|&(f, c)| (c, f)).unwrap();
        let mut common = 0u32;
        for d in 0.. {
            let mut count: HashMap<u32, usize> = HashMap::new();
            for s in chunk {
                if let Some(&f) = s.get(d) {
                    *count.entry(f).or_default() += 1;
                }
            }
            match count.into_iter().max_by_key(|&(f, c)| (c, f)) {
                Some((f, c)) if c * 10 >= chunk.len() * 6 => common = f,
                _ => break,
            }
        }
        out.push(format!("{}/{} {}%", name(common), name(leaf), c * 100 / chunk.len()));
    }
    format!("tramos de {} ms (común/hoja): {}", ms / parts as u64, out.join(", "))
}

#[cfg(test)]
mod tests {
    #[test]
    fn phases_split_the_frame_in_time() {
        let mut s = vec![vec![1, 2, 3]; 8];
        s.extend(vec![vec![1, 5, 6]; 8]);
        let r = super::phases(1600, &s);
        assert!(r.starts_with("tramos de 200 ms"), "{r}");
        assert!(r.contains("3/3 100%") && r.ends_with("6/6 100%"), "{r}");
    }

    #[test]
    fn report_shows_common_path_and_leaves() {
        let s = vec![vec![1, 2, 3, 9], vec![1, 2, 3, 9], vec![1, 2, 3, 8], vec![1, 2, 4]];
        let r = super::report(7, 400, &s);
        assert!(r.contains("…1>2>3;"), "{r}");
        assert!(r.contains("hojas 9 50%"), "{r}");
    }
}
