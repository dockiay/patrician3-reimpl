//! `mss32.dll` (Miles Sound System 6) del original sustituida: las 44 importaciones `AIL_*` del EXE sobre el mezclador
//! [`p3_audio::live::Mixer`].
//!
//! El gestor de sonido del juego (`0x006CBDB8`, J04) se ejecuta tal cual en el enlazador y llama a Miles por la IAT; aquí cada
//! llamada se traduce a una orden del mezclador, que el cliente nativo (`p3-client`) saca por la tarjeta de sonido. Sin
//! mezclador (`MilesState::mixer == None`, pruebas y bancos) no se llega aquí: el enlazador conserva los sustitutos que dejan el
//! gestor sin dispositivo, como antes.
//!
//! Los manejadores que el juego guarda son direcciones de bloques del montón emulado: el juego solo lee `HSTREAM+0xC` (la
//! muestra del stream, para ponerle el filtro paso bajo, `0x00443742`). Contrato propio (no la DLL): ver `p3_audio::live`.
//! Los efectos llegan ya en memoria (el juego los lee del CPR con `arc_*` y llama a `AIL_set_named_sample_file`); la música
//! y los ambientes son archivos sueltos (`Musik\...\*.mp3`) que se buscan en [`MilesState::dirs`] sin distinguir mayúsculas.
use super::Runtime;
use p3_audio::live::{Mixer, SMP_DONE};
use p3_render::scene_math::Mem;
use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Proveedor ficticio del filtro «LowPass Filter» (`AIL_enumerate_filters`).
const LOWPASS_PROVIDER: u32 = 0x4C50_0001;
/// Formato que devuelve `AIL_digital_configuration`: `DIG_F_STEREO_16`.
const DIG_F_STEREO_16: u32 = 3;

#[derive(Default)]
pub struct MilesState {
    /// Mezclador del cliente; `None` = sin sonido (el gestor queda sin dispositivo, como hasta ahora).
    pub mixer: Option<Arc<Mixer>>,
    /// Carpetas donde buscar los archivos sueltos que abre Miles (`Musik\...`), en orden.
    pub dirs: Vec<PathBuf>,
    /// Escribe cada llamada en la salida de errores (`P3_AIL_TRACE`).
    pub trace: bool,
    /// Llamadas por función (diagnóstico).
    pub calls: BTreeMap<String, u64>,
    driver: u32,
    /// Manejador emulado (muestra o stream) → voz del mezclador.
    voices: HashMap<u32, u32>,
    /// Stream → su muestra interna (`HSTREAM+0xC`).
    stream_sample: HashMap<u32, u32>,
    /// Voz → (filtro puesto, corte en Hz).
    lowpass: HashMap<u32, (bool, f32)>,
    /// Archivos de `AIL_open_stream` ya leídos (nombre en minúsculas → bytes, `None` si no existe). El juego vuelve a abrir la
    /// música y los ambientes de los edificios cada pocos segundos al moverse por la ciudad (`0x442C60`); leer otra vez el MP3
    /// entero (hasta 7 MB, `Gebaeude\gilde.mp3`) y buscarlo sin distinguir mayúsculas en tres carpetas congelaba el juego de 0,2 a
    /// 1,3 s en el disco duro de Sergio (registro «tirón:» del 20/11, hoja `AIL_open_stream` en el 95 % de las muestras).
    stream_files: HashMap<String, Option<Arc<[u8]>>>,
    stream_files_bytes: usize,
    prefs: HashMap<u32, u32>,
    strings: HashMap<&'static str, u32>,
}

impl MilesState {
    /// Estado vacío; `P3_AIL_TRACE` activa la traza de llamadas.
    pub fn from_env() -> Self {
        Self {
            trace: std::env::var_os("P3_AIL_TRACE").is_some(),
            ..Default::default()
        }
    }

    /// Archivo suelto de la instalación: cada tramo se busca sin distinguir mayúsculas en cada carpeta de [`Self::dirs`].
    pub fn find_file(&self, name: &str) -> Option<PathBuf> {
        let name = name.replace('\\', "/");
        let parts: Vec<&str> = name.split('/').filter(|p| !p.is_empty() && *p != ".").collect();
        if parts.is_empty() || parts.contains(&"..") {
            return None;
        }
        let fs = crate::hostfs::current();
        self.dirs.iter().find_map(|root| {
            let mut at = root.clone();
            for part in &parts {
                let direct = at.join(part);
                at = if fs.exists(&direct) { direct } else { fs.find_ci(&at, part)?.path };
            }
            fs.is_file(&at).then_some(at)
        })
    }
}

/// Nombre de la función sin decoración (`_AIL_start_sample@4` → `start_sample`); `None` si no es de Miles.
fn ail_name(func: &str) -> Option<&str> {
    let f = func.strip_prefix('_').unwrap_or(func);
    let f = f.split('@').next().unwrap_or(f);
    f.strip_prefix("AIL_")
}

impl Runtime<'_> {
    /// Bloque a cero del montón emulado.
    fn miles_block(&mut self, m: &mut dyn Mem, size: u32) -> u32 {
        let at = self.heap_take(size);
        super::zero_fill(m, at, size);
        at
    }

    /// Cadena C fija en memoria emulada (una sola copia por texto).
    fn miles_string(&mut self, m: &mut dyn Mem, text: &'static str) -> u32 {
        if let Some(&at) = self.miles.strings.get(text) {
            return at;
        }
        let at = self.miles_block(m, text.len() as u32 + 1);
        super::write_bytes(m, at, text.as_bytes());
        self.miles.strings.insert(text, at);
        at
    }

    /// Bytes de un archivo suelto (o, si no está suelto, del CPR).
    fn miles_file(&mut self, name: &[u8]) -> Option<Vec<u8>> {
        let text = String::from_utf8_lossy(name).into_owned();
        if let Some(path) = self.miles.find_file(&text) {
            if let Ok(data) = crate::hostfs::current().read(&path) {
                return Some(data);
            }
        }
        self.assets.file(name).map(|d| d.to_vec())
    }

    /// [`Self::miles_file`] con caché para `AIL_open_stream` (ver `MilesState::stream_files`). Tope de 256 MiB: al pasarlo se
    /// vacía y se empieza de nuevo (toda la carpeta `Musik` ocupa unos 76 MB).
    fn stream_file(&mut self, name: &[u8]) -> Option<Arc<[u8]>> {
        let key = String::from_utf8_lossy(name).to_ascii_lowercase().replace('\\', "/");
        if let Some(hit) = self.miles.stream_files.get(&key) {
            return hit.clone();
        }
        let data: Option<Arc<[u8]>> = self.miles_file(name).map(Arc::from);
        let len = data.as_ref().map_or(0, |d| d.len());
        if self.miles.stream_files_bytes + len > 256 << 20 {
            self.miles.stream_files.clear();
            self.miles.stream_files_bytes = 0;
        }
        self.miles.stream_files_bytes += len;
        self.miles.stream_files.insert(key, data.clone());
        data
    }

    fn read_vec(m: &mut dyn Mem, at: u32, len: u32) -> Vec<u8> {
        let mut out = vec![0u8; len as usize];
        m.read_bytes(at, &mut out);
        out
    }

    /// Las importaciones `AIL_*`; `None` si `func` no es de Miles o no hay mezclador.
    pub(super) fn miles_import(
        &mut self,
        m: &mut dyn Mem,
        func: &str,
        arg: &dyn Fn(&mut dyn Mem, u32) -> u32,
    ) -> Option<u32> {
        let name = ail_name(func)?;
        let mixer = self.miles.mixer.clone()?;
        mixer.service();
        *self.miles.calls.entry(name.to_string()).or_default() += 1;
        let voice = |rt: &Self, h: u32| rt.miles.voices.get(&h).copied();
        let r = match name {
            "startup" => 1,
            "shutdown" => 0,
            "set_redist_directory" => arg(m, 0),
            "last_error" => self.miles_string(m, ""),
            "set_preference" => {
                let (n, v) = (arg(m, 0), arg(m, 1));
                self.miles.prefs.insert(n, v).unwrap_or(0)
            }
            "get_preference" => self.miles.prefs.get(&arg(m, 0)).copied().unwrap_or(0),
            // `AIL_waveOutOpen(&driver, &hwaveout, dispositivo, WAVEFORMAT*)`: 0 = abierto.
            "waveOutOpen" => {
                let (pdrv, phwo) = (arg(m, 0), arg(m, 1));
                if self.miles.driver == 0 {
                    self.miles.driver = self.miles_block(m, 0x40);
                }
                if pdrv != 0 {
                    m.w32(pdrv, self.miles.driver);
                }
                if phwo != 0 {
                    m.w32(phwo, 0);
                }
                0
            }
            "waveOutClose" => {
                self.miles.driver = 0;
                0
            }
            // `AIL_digital_configuration(driver, &frecuencia, &formato, texto)`: el juego busca «Emulated» en el texto (`0x00441F2B`).
            "digital_configuration" => {
                let (prate, pfmt, text) = (arg(m, 1), arg(m, 2), arg(m, 3));
                if prate != 0 {
                    m.w32(prate, mixer.rate());
                }
                if pfmt != 0 {
                    m.w32(pfmt, DIG_F_STEREO_16);
                }
                if text != 0 {
                    super::write_bytes(m, text, b"p3-audio\0");
                }
                0
            }
            "set_digital_master_volume" => {
                mixer.set_master(arg(m, 1) as i32);
                0
            }
            "allocate_sample_handle" => {
                let h = self.miles_block(m, 0x100);
                let v = mixer.allocate(false);
                self.miles.voices.insert(h, v);
                h
            }
            "release_sample_handle" => {
                let h = arg(m, 0);
                if let Some(v) = self.miles.voices.remove(&h) {
                    mixer.release(v);
                    self.miles.lowpass.remove(&v);
                }
                0
            }
            "init_sample" => {
                if let Some(v) = voice(self, arg(m, 0)) {
                    mixer.init(v);
                    self.miles.lowpass.remove(&v);
                }
                0
            }
            // `AIL_set_sample_file(muestra, imagen, bloque)` / `AIL_set_named_sample_file(muestra, sufijo, imagen, tamaño, bloque)`.
            "set_sample_file" | "set_named_sample_file" => {
                let named = name == "set_named_sample_file";
                let h = arg(m, 0);
                let (suffix, image, mut size) = if named {
                    (arg(m, 1), arg(m, 2), arg(m, 3))
                } else {
                    (0, arg(m, 1), 0)
                };
                if size == 0 && m.r32(image) == u32::from_le_bytes(*b"RIFF") {
                    size = m.r32(image + 4).saturating_add(8);
                }
                let label = if suffix != 0 {
                    String::from_utf8_lossy(&super::cstr(m, suffix)).into_owned()
                } else {
                    String::new()
                };
                let label = if label.to_ascii_lowercase().ends_with(".mp3") {
                    label
                } else {
                    format!("{label}.wav")
                };
                match voice(self, h) {
                    Some(v) if image != 0 && size != 0 && size < (64 << 20) => {
                        let bytes = Self::read_vec(m, image, size);
                        u32::from(mixer.load(v, &bytes, &label))
                    }
                    _ => 0,
                }
            }
            "start_sample" | "start_stream" => {
                if let Some(v) = voice(self, arg(m, 0)) {
                    mixer.start(v);
                }
                0
            }
            "stop_sample" => {
                if let Some(v) = voice(self, arg(m, 0)) {
                    mixer.stop(v);
                }
                0
            }
            "end_sample" => {
                if let Some(v) = voice(self, arg(m, 0)) {
                    mixer.end(v);
                }
                0
            }
            "sample_status" | "stream_status" => voice(self, arg(m, 0)).map_or(SMP_DONE, |v| mixer.status(v)),
            "set_sample_volume" | "set_stream_volume" => {
                if let Some(v) = voice(self, arg(m, 0)) {
                    mixer.set_volume(v, arg(m, 1) as i32);
                }
                0
            }
            "sample_volume" | "stream_volume" => voice(self, arg(m, 0)).map_or(0, |v| mixer.volume(v) as u32),
            "set_sample_pan" => {
                if let Some(v) = voice(self, arg(m, 0)) {
                    mixer.set_pan(v, arg(m, 1) as i32);
                }
                0
            }
            "sample_pan" => voice(self, arg(m, 0)).map_or(64, |v| mixer.pan(v) as u32),
            "sample_ms_position" | "stream_ms_position" => {
                let (total, current) = voice(self, arg(m, 0)).map_or((0, 0), |v| mixer.ms_position(v));
                let (pt, pc) = (arg(m, 1), arg(m, 2));
                if pt != 0 {
                    m.w32(pt, total);
                }
                if pc != 0 {
                    m.w32(pc, current);
                }
                0
            }
            "set_stream_ms_position" => {
                if let Some(v) = voice(self, arg(m, 0)) {
                    mixer.set_ms_position(v, arg(m, 1));
                }
                0
            }
            "mem_alloc_lock" => {
                let size = arg(m, 0).max(1);
                self.miles_block(m, size)
            }
            "mem_free_lock" => {
                { let p = arg(m, 0); self.heap_free(m, p); }
                0
            }
            // `AIL_file_read(nombre, destino)`: archivo entero en `destino` o en un bloque nuevo; 0 si no existe.
            "file_read" => {
                let (pname, dest) = (arg(m, 0), arg(m, 1));
                let fname = super::cstr(m, pname);
                match self.miles_file(&fname) {
                    Some(data) => {
                        let at = if dest != 0 {
                            dest
                        } else {
                            self.miles_block(m, data.len() as u32 + 1)
                        };
                        super::write_bytes(m, at, &data);
                        at
                    }
                    None => 0,
                }
            }
            // `AIL_open_stream(driver, nombre, memoria)`: stream sobre un archivo suelto (música y ambientes).
            "open_stream" => {
                let pname = arg(m, 1);
                let fname = super::cstr(m, pname);
                match self.stream_file(&fname) {
                    Some(data) => {
                        let v = mixer.open_stream(&String::from_utf8_lossy(&fname), data);
                        let (st, smp) = (self.miles_block(m, 0x40), self.miles_block(m, 0x100));
                        m.w32(st + 0xC, smp);
                        self.miles.voices.insert(st, v);
                        self.miles.voices.insert(smp, v);
                        self.miles.stream_sample.insert(st, smp);
                        st
                    }
                    None => 0,
                }
            }
            "close_stream" => {
                let st = arg(m, 0);
                if let Some(v) = self.miles.voices.remove(&st) {
                    mixer.release(v);
                    self.miles.lowpass.remove(&v);
                }
                if let Some(smp) = self.miles.stream_sample.remove(&st) {
                    self.miles.voices.remove(&smp);
                }
                0
            }
            "pause_stream" => {
                if let Some(v) = voice(self, arg(m, 0)) {
                    if arg(m, 1) != 0 {
                        mixer.stop(v);
                    } else {
                        mixer.resume(v);
                    }
                }
                0
            }
            "set_stream_loop_count" => {
                if let Some(v) = voice(self, arg(m, 0)) {
                    mixer.set_loops(v, arg(m, 1));
                }
                0
            }
            "service_stream" => 0,
            // `AIL_enumerate_filters(&siguiente, &proveedor, &nombre)`: un único filtro, el paso bajo que busca `0x00442043`.
            "enumerate_filters" => {
                let (pnext, pdest, pname) = (arg(m, 0), arg(m, 1), arg(m, 2));
                if m.r32(pnext) != 0 {
                    0
                } else {
                    let text = self.miles_string(m, "LowPass Filter");
                    m.w32(pnext, 1);
                    m.w32(pdest, LOWPASS_PROVIDER);
                    m.w32(pname, text);
                    1
                }
            }
            "open_filter" => u32::from(arg(m, 0) == LOWPASS_PROVIDER),
            // `AIL_enumerate_filter_sample_attributes(proveedor, &siguiente, RIB_INTERFACE_ENTRY*)`: el atributo «Lowpass Cutoff»
            // (el juego se queda con el último nombre y le da 400.0 Hz, `0x00443705`).
            "enumerate_filter_sample_attributes" => {
                let (pnext, dest) = (arg(m, 1), arg(m, 2));
                if m.r32(pnext) != 0 {
                    0
                } else {
                    let text = self.miles_string(m, "Lowpass Cutoff");
                    m.w32(pnext, 1);
                    for (k, v) in [1, text, 1, 2].into_iter().enumerate() {
                        m.w32(dest + 4 * k as u32, v);
                    }
                    1
                }
            }
            // `AIL_set_sample_processor(muestra, etapa, proveedor)`: pone o quita el paso bajo; devuelve el proveedor anterior.
            "set_sample_processor" => {
                let (h, provider) = (arg(m, 0), arg(m, 2));
                match voice(self, h) {
                    Some(v) => {
                        let entry = self.miles.lowpass.entry(v).or_insert((false, 400.0));
                        let previous = if entry.0 { LOWPASS_PROVIDER } else { 0 };
                        entry.0 = provider == LOWPASS_PROVIDER;
                        let (on, cutoff) = *entry;
                        mixer.set_lowpass(v, on.then_some(cutoff));
                        previous
                    }
                    None => 0,
                }
            }
            "set_filter_sample_preference" => {
                let (h, pval) = (arg(m, 0), arg(m, 2));
                if let (Some(v), true) = (voice(self, h), pval != 0) {
                    let cutoff = f32::from_bits(m.r32(pval));
                    let entry = self.miles.lowpass.entry(v).or_insert((false, 400.0));
                    entry.1 = cutoff;
                    let (on, cutoff) = *entry;
                    mixer.set_lowpass(v, on.then_some(cutoff));
                }
                0
            }
            _ => {
                self.import_missing.insert(func.to_string());
                0
            }
        };
        if self.miles.trace {
            let a: Vec<String> = (0..4).map(|i| format!("{:x}", arg(m, i))).collect();
            eprintln!("AIL_{name}({}) = {r:#x}", a.join(", "));
        }
        Some(r)
    }
}

/// Carpetas donde buscar `Musik\...`: la del juego (`--juego`), la del CPR y la del EXE, sin repetir.
pub fn default_dirs(paths: &[&Path]) -> Vec<PathBuf> {
    let mut out: Vec<PathBuf> = Vec::new();
    for p in paths {
        let dir = if p.is_dir() {
            p.to_path_buf()
        } else {
            p.parent().map(Path::to_path_buf).unwrap_or_default()
        };
        let dir = if dir.as_os_str().is_empty() {
            PathBuf::from(".")
        } else {
            dir
        };
        if !out.contains(&dir) {
            out.push(dir);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_and_case_insensitive_lookup() {
        assert_eq!(ail_name("_AIL_start_sample@4"), Some("start_sample"));
        assert_eq!(ail_name("AIL_startup"), Some("startup"));
        assert_eq!(ail_name("timeGetTime"), None);
        let root = std::env::temp_dir().join(format!("p3-miles-{}", std::process::id()));
        std::fs::create_dir_all(root.join("Musik/MenueMusic")).unwrap();
        std::fs::write(root.join("Musik/MenueMusic/hauptmenue.mp3"), b"x").unwrap();
        let st = MilesState {
            dirs: vec![PathBuf::from("/nonexistent"), root.clone()],
            ..Default::default()
        };
        assert!(st.find_file("musik\\menuemusic\\HAUPTMENUE.mp3").is_some());
        assert!(st.find_file("./Musik/MenueMusic/hauptmenue.mp3").is_some());
        assert!(st.find_file("../Musik/x.mp3").is_none());
        let _ = std::fs::remove_dir_all(root);
    }
}
