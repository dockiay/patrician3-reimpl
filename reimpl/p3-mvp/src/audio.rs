//! J04: catálogo de muestras (`scripts/sound.ini` del `.cpr`) y conversión a WAV PCM de 16 bits.
//! Un disparador de `p3_audio::triggers` produce un ID de muestra; aquí se resuelve su archivo y se decodifica
//! (IMA de Miles con `decode_miles_defined`, o PCM sin comprimir). Sin dispositivo de salida: lo reproduce el cliente.
use std::collections::BTreeMap;

use p3_assets::cpr::Archive;
use p3_audio::native::wave::WaveInfo;

/// Una muestra del INI: archivo (separadores `\`) y volumen (`Loudness`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sample {
    pub file: String,
    pub loudness: u32,
}

/// IDs `[SAMPLEn]` de `sound.ini`.
#[derive(Clone, Debug, Default)]
pub struct SoundCatalog {
    pub samples: BTreeMap<u32, Sample>,
    /// `[SoundListN] Count=` (`FUN_00443690`).
    pub lists: BTreeMap<u32, u32>,
}

impl SoundCatalog {
    pub fn parse(ini: &str) -> Self {
        let mut samples = BTreeMap::new();
        let mut lists = BTreeMap::new();
        let mut cur: Option<u32> = None;
        let mut cur_list: Option<u32> = None;
        for line in ini.lines() {
            let l = line.trim();
            if let Some(name) = l.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
                cur_list = name.strip_prefix("SoundList").and_then(|n| n.parse().ok());
                cur = name.strip_prefix("SAMPLE").and_then(|n| n.parse().ok());
                if let Some(id) = cur {
                    samples.entry(id).or_insert(Sample { file: String::new(), loudness: 100 });
                }
            } else if let (Some(id), Some((k, v))) = (cur_list, l.split_once('=')) {
                if k.trim().eq_ignore_ascii_case("count") {
                    lists.insert(id, v.trim().parse().unwrap_or(0));
                }
            } else if let (Some(id), Some((k, v))) = (cur, l.split_once('=')) {
                let Some(s) = samples.get_mut(&id) else { continue };
                match k.trim().to_ascii_lowercase().as_str() {
                    "samplefile" => s.file = v.trim().to_string(),
                    "loudness" => s.loudness = v.trim().parse().unwrap_or(100),
                    _ => {}
                }
            }
        }
        samples.retain(|_, s| !s.file.is_empty());
        Self { samples, lists }
    }

    /// Catálogo del contenedor del juego (`scripts/sound.ini`).
    pub fn from_archive(archive: &Archive) -> Result<Self, String> {
        let e = archive.find(b"scripts/sound.ini").ok_or("el CPR no contiene scripts/sound.ini")?;
        let data = archive.read(e).ok_or("sound.ini comprimido o truncado")?;
        // El INI es Windows-1252/Latin-1 (p. ej. «Stürze»): cada byte es un carácter.
        Ok(Self::parse(&data.iter().map(|&b| char::from(b)).collect::<String>()))
    }
}

/// WAV PCM de 16 bits ya decodificado.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pcm {
    pub channels: u16,
    pub sample_rate: u32,
    pub samples: Vec<i16>,
}

impl Pcm {
    pub fn to_wav(&self) -> Vec<u8> {
        let size = (self.samples.len() * 2) as u32;
        let block = self.channels * 2;
        let mut wav = Vec::with_capacity(size as usize + 44);
        wav.extend_from_slice(b"RIFF");
        wav.extend_from_slice(&(size + 36).to_le_bytes());
        wav.extend_from_slice(b"WAVEfmt ");
        wav.extend_from_slice(&16u32.to_le_bytes());
        wav.extend_from_slice(&1u16.to_le_bytes());
        wav.extend_from_slice(&self.channels.to_le_bytes());
        wav.extend_from_slice(&self.sample_rate.to_le_bytes());
        wav.extend_from_slice(&(self.sample_rate * u32::from(block)).to_le_bytes());
        wav.extend_from_slice(&block.to_le_bytes());
        wav.extend_from_slice(&16u16.to_le_bytes());
        wav.extend_from_slice(b"data");
        wav.extend_from_slice(&size.to_le_bytes());
        for s in &self.samples {
            wav.extend_from_slice(&s.to_le_bytes());
        }
        wav
    }
}

/// Decodifica un `.wav` del juego: IMA de Miles, o PCM de 16 bits sin comprimir.
pub fn decode_wav(bytes: &[u8]) -> Result<Pcm, String> {
    if let Ok(p) = p3_audio::native::ima::decode_miles_defined(bytes, 44_100 * 600) {
        return Ok(Pcm { channels: p.channels, sample_rate: p.sample_rate, samples: p.samples });
    }
    let info = WaveInfo::parse(bytes).map_err(|e| format!("WAV: {e:?}"))?;
    if info.format_tag != 1 || info.bits_per_sample != 16 {
        return Err(format!("WAV formato {} de {} bits sin decodificador", info.format_tag, info.bits_per_sample));
    }
    let data = &bytes[info.data.clone()];
    let samples = data.chunks_exact(2).map(|c| i16::from_le_bytes([c[0], c[1]])).collect();
    Ok(Pcm { channels: info.channels, sample_rate: info.sample_rate, samples })
}

impl SoundCatalog {
    /// Resuelve y decodifica la muestra `id` del contenedor.
    pub fn render(&self, archive: &Archive, id: u32) -> Result<Pcm, String> {
        let s = self.samples.get(&id).ok_or_else(|| format!("ID {id} sin muestra"))?;
        let path: Vec<u8> = s.file.chars().map(|c| if c == '\\' { b'/' } else { c as u32 as u8 }).collect();
        let shown = String::from_utf8_lossy(&path).into_owned();
        let e = archive.find(&path).ok_or_else(|| format!("{shown} no está en el CPR"))?;
        let data = archive.read(e).ok_or_else(|| format!("{shown} comprimido o truncado"))?;
        decode_wav(data)
    }
}

/// Volumen de efectos de `P2.CFG` como lo lee `0x45CFCF`: `GetPrivateProfileIntA("OPTIONS", "LSSOUND", 80, ".\\P2.CFG")` y, si el valor queda
/// fuera de `0..=255`, 80. `0x463D20` lo copia (byte bajo) a `[0x6CC3E8]+0x23`, el volumen que usan los sonidos del mapa. Sin archivo, 80.
pub fn effects_volume(cfg: Option<&[u8]>) -> u8 {
    const DEFAULT: i64 = 80;
    let Some(cfg) = cfg else { return DEFAULT as u8 };
    let text: String = cfg.iter().map(|&b| char::from(b)).collect();
    let mut section = false;
    let mut value = DEFAULT;
    for line in text.lines() {
        let l = line.trim();
        if let Some(name) = l.strip_prefix('[').and_then(|r| r.split(']').next()) {
            section = name.trim().eq_ignore_ascii_case("OPTIONS");
        } else if let (true, Some((k, v))) = (section, l.split_once('=')) {
            if k.trim().eq_ignore_ascii_case("LSSOUND") {
                // GetPrivateProfileInt: signo opcional y dígitos iniciales; sin dígitos, 0.
                let v = v.trim();
                let (neg, digits) = match v.strip_prefix('-') {
                    Some(r) => (true, r),
                    None => (false, v),
                };
                let n: i64 = digits.chars().take_while(char::is_ascii_digit).fold(0i64, |a, c| (a * 10 + i64::from(c as u8 - b'0')) & 0xFFFF_FFFF);
                value = if neg { -n } else { n };
                break;
            }
        }
    }
    if (0..=255).contains(&value) {
        value as u8
    } else {
        DEFAULT as u8
    }
}

/// Una muestra que el tablero pide reproducir: el cliente la decodifica y la mezcla en su salida.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlayRequest {
    pub id: u32,
    /// 0..=127 (`0x594B50`); 255 en los canales exclusivos (`0xFFFFFFFF`).
    pub volume: u32,
    /// 0..=127; 64 es el centro.
    pub pan: u32,
}

/// Escena de sonido de la vista de ciudad sin dispositivo: ranuras de `p3_audio::slots` más el catálogo.
/// Registra las peticiones de `play_sound` (0x443150) y simula su fin con la duración real de la muestra.
pub struct SoundBoard {
    pub scene: p3_audio::slots::SoundScene,
    pub catalog: SoundCatalog,
    durations: BTreeMap<u32, u32>,
    pcm_ms: BTreeMap<u32, u32>,
    started: BTreeMap<u32, (u32, u32, u32)>,
    next_handle: u32,
    clock: u32,
    rng: u32,
    pub requests: Vec<PlayRequest>,
    /// Escena de sonido del mapa de navegación (`0x459790`, `0x45A470`, `0x459380`).
    pub map: p3_audio::map_ambient::MapScene,
    /// Volumen general de efectos (`DAT_006CC3E8+0x23`): `LSSOUND` de `P2.CFG` ([`effects_volume`]); 80 sin archivo, como el original.
    pub map_volume: u8,
    /// `stop_all` (0x443940) pedido por la clase 1 desde la última consulta.
    pub stopped: bool,
}

impl SoundBoard {
    /// Valores de `scripts/iso.ini` (`GebaeudeMinDist`, `PanDist`, `VolDist`, `GebaeudeSoundListBase`, `SoundsProGebaeude`, `GebaeudeTypLockTime`, `GebaeudeKanalLockTime`).
    pub fn new(catalog: SoundCatalog, durations: BTreeMap<u32, u32>) -> Self {
        let mut scene = p3_audio::slots::SoundScene::new();
        let w = |m: &mut Vec<u8>, o: usize, v: u32| m[o..o + 4].copy_from_slice(&v.to_le_bytes());
        w(&mut scene.mem, 0x2C, 1024);
        w(&mut scene.mem, 0x30, 768);
        for (o, v) in [(0xC7D0, 300), (0xC7D4, 500), (0xC7D8, 750), (0xC7DC, 10000), (0xC7E0, 10), (0xC7E4, 6000), (0xC7E8, 4000)] {
            w(&mut scene.mem, o, v);
        }
        // Listas de iso.ini: derrumbe, selección de barco, iglesia y entrada/salida de barcos.
        for (o, v) in [(0xC994, 9000), (0xCBA4, 1000), (0xCB98, 12000), (0xCBA0, 13000)] {
            w(&mut scene.mem, o, v);
        }
        // `[Navmap0]` de los scripts (`0x44A9F5`): `PanDist=500`, `VolDist=750` → `+0x2B30`, `+0x2B34`; vista por defecto 1024×768.
        let mut map = p3_audio::map_ambient::MapScene::new();
        map.w32(0x2B30, 500);
        map.w32(0x2B34, 750);
        map.w32(0x2C, 1024);
        map.w32(0x30, 768);
        Self { scene, catalog, durations, pcm_ms: BTreeMap::new(), started: BTreeMap::new(), next_handle: 1, clock: 0, rng: 1, requests: vec![], map, map_volume: 80, stopped: false }
    }

    pub fn set_clock(&mut self, ms: u32) {
        self.clock = ms;
    }

    /// Cámara: origen del mapa en pantalla (`+0xC650`, `+0xC654`).
    pub fn set_camera(&mut self, x: i32, y: i32) {
        self.scene.mem[0xC650..0xC654].copy_from_slice(&x.to_le_bytes());
        self.scene.mem[0xC654..0xC658].copy_from_slice(&y.to_le_bytes());
    }

    /// Duración en ms de una muestra, medida sobre el PCM decodificado (`render`).
    pub fn measure(&mut self, archive: &Archive, id: u32) -> u32 {
        if let Some(d) = self.durations.get(&id).or(self.pcm_ms.get(&id)) {
            return *d;
        }
        let ms = self
            .catalog
            .render(archive, id)
            .map(|p| (p.samples.len() as u64 * 1000 / (u64::from(p.sample_rate.max(1)) * u64::from(p.channels.max(1)))) as u32)
            .unwrap_or(0);
        self.pcm_ms.insert(id, ms);
        ms
    }

    /// Sonido posicional (`class22_play_sound_at_position`); devuelve las peticiones nuevas.
    pub fn play_at(&mut self, archive: &Archive, x: i32, y: i32, id: u32) -> Vec<PlayRequest> {
        let first = self.requests.len();
        let mut scene = std::mem::replace(&mut self.scene, p3_audio::slots::SoundScene::new());
        let mut env = BoardEnv { b: self, archive };
        let _ = scene.play_at_position(&mut env, x, y, id);
        self.scene = scene;
        self.requests[first..].to_vec()
    }

    /// Sonido de derrumbe de un edificio (0x599040) en el punto de pantalla `(x, y)`.
    pub fn collapse(&mut self, archive: &Archive, x: i32, y: i32) -> Vec<PlayRequest> {
        let first = self.requests.len();
        let mut scene = std::mem::replace(&mut self.scene, p3_audio::slots::SoundScene::new());
        let mut env = BoardEnv { b: self, archive };
        let _ = scene.play_collapse(&mut env, x, y, true);
        self.scene = scene;
        self.requests[first..].to_vec()
    }

    /// Sonido ambiental de edificio (0x594C00) en `(x, y)` con la clase de ficha `kind`; arranca en el siguiente `update`.
    pub fn queue_building(&mut self, archive: &Archive, x: i32, y: i32, kind: u16) {
        let mut scene = std::mem::replace(&mut self.scene, p3_audio::slots::SoundScene::new());
        let mut env = BoardEnv { b: self, archive };
        scene.queue_building_sound(&mut env, x, y, kind);
        self.scene = scene;
    }

    /// Vista del mapa de navegación: cámara `(x, y)` en el mapa (`+0x29E4`, `+0x29E6`) y tamaño `w × h` (`+0x2C`, `+0x30`).
    /// Activa los ajustes por fotograma (`+0x2B28`).
    pub fn set_map_view(&mut self, x: i32, y: i32, w: i32, h: i32) {
        let m = &mut self.map;
        m.w16(0x29E4, x as u16);
        m.w16(0x29E6, y as u16);
        m.w32(0x2C, w as u32);
        m.w32(0x30, h as u32);
        m.mem[0x2B28] = 1;
    }

    /// Sonido del mapa `0x459790(clase, id, x, y, extra)` con la planificación del original (`p3_audio::map_ambient`, paridad x86):
    /// colas por clase, sin repetir una `id` que sigue sonando, caducidad, paneo/volumen por distancia (clases 3 y 4) y la exclusiva
    /// de avistamientos (`+0x2D14`). Devuelve las muestras nuevas. El volumen general (`DAT_006CC3E8+0x23`) es [`Self::map_volume`] (`LSSOUND` de `P2.CFG`).
    pub fn map_ambient(&mut self, archive: &Archive, class: u32, id: u32, x: i32, y: i32, extra: u32) -> Vec<PlayRequest> {
        let first = self.requests.len();
        let mut map = std::mem::take(&mut self.map);
        let (clock, volume) = (self.clock, self.map_volume);
        map.play_ambient(class, id, x, y, extra, clock, volume, &mut BoardEnv { b: self, archive });
        self.map = map;
        self.requests[first..].to_vec()
    }

    /// `0x45A470` (cada fotograma del mapa): suelta la exclusiva de avistamientos al terminar y reajusta volumen/panorámica de las
    /// colas 3 y 4 con la cámara actual. Devuelve los ajustes `(id, volumen, panorámica)` de las muestras que siguen sonando.
    pub fn map_frame(&mut self) -> Vec<PlayRequest> {
        let mut map = std::mem::take(&mut self.map);
        let mut env = FrameEnv { b: self, out: vec![] };
        map.update_frame(&mut env);
        let out = env.out;
        self.map = map;
        out
    }

    /// `0x459380` con las 16 entidades de tiempo del gestor naval (`+0x20`, 12 bytes cada una) y el número de ciudades (`+0xF4`):
    /// cada tormenta/calma visible cuyo plazo venció pide su sonido de clase 3. Devuelve las muestras nuevas.
    pub fn map_weather(&mut self, archive: &Archive, manager: &[u8]) -> Vec<PlayRequest> {
        if manager.len() < 0xF6 {
            return vec![];
        }
        let u16_at = |o: usize| u16::from_le_bytes([manager[o], manager[o + 1]]);
        let mut ents = [p3_audio::map_ambient::WeatherEntity::default(); 16];
        for (i, e) in ents.iter_mut().enumerate() {
            let o = 0x20 + 12 * i;
            *e = p3_audio::map_ambient::WeatherEntity {
                key: u32::from_le_bytes(manager[o..o + 4].try_into().unwrap()),
                x: u16_at(o + 4) as i16,
                y: u16_at(o + 6) as i16,
                kind: u16_at(o + 8),
                active: u16_at(o + 10),
            };
        }
        let mut host = WeatherCalls::default();
        let clock = self.clock;
        self.map.prepare_weather(&ents, u16_at(0xF4), clock, &mut host);
        host.calls.into_iter().flat_map(|(c, id, x, y, extra)| self.map_ambient(archive, c, id, x, y, extra)).collect()
    }

    /// Mantenimiento por fotograma: libera las muestras terminadas y reajusta volumen/panorámica.
    pub fn update(&mut self, archive: &Archive) {
        let mut scene = std::mem::replace(&mut self.scene, p3_audio::slots::SoundScene::new());
        let mut env = BoardEnv { b: self, archive };
        let _ = scene.update_channels(&mut env);
        let _ = scene.start_pending_building_sounds(&mut env);
        self.scene = scene;
    }
}

/// Ajustes de `0x45A470` sobre las muestras que siguen sonando.
struct FrameEnv<'a> {
    b: &'a mut SoundBoard,
    out: Vec<PlayRequest>,
}

impl FrameEnv<'_> {
    fn adjust(&mut self, handle: u32, f: impl FnOnce(&mut PlayRequest)) {
        let Some(&(t0, ms, id)) = self.b.started.get(&handle) else { return };
        if self.b.clock.wrapping_sub(t0) >= ms {
            return;
        }
        let at = match self.out.iter().position(|r| r.id == id) {
            Some(i) => i,
            None => {
                self.out.push(PlayRequest { id, volume: 0, pan: 0x40 });
                self.out.len() - 1
            }
        };
        f(&mut self.out[at]);
    }
}

impl p3_audio::map_ambient::MapFrameEnv for FrameEnv<'_> {
    fn sample_status(&mut self, handle: u32) -> u32 {
        match self.b.started.get(&handle) {
            Some(&(t0, ms, _)) if self.b.clock.wrapping_sub(t0) < ms => 4,
            _ => 2,
        }
    }
    fn set_volume(&mut self, handle: u32, volume: u8) {
        self.adjust(handle, |r| r.volume = u32::from(volume));
    }
    fn set_pan(&mut self, handle: u32, pan: u8) {
        self.adjust(handle, |r| r.pan = u32::from(pan));
    }
}

/// Llamadas de `0x459380` a `0x459790`; el repintado y los sprites los hace el dibujo del mapa.
#[derive(Default)]
struct WeatherCalls {
    calls: Vec<(u32, u32, i32, i32, u32)>,
}

impl p3_audio::map_ambient::WeatherHost for WeatherCalls {
    fn minimap_repaint(&mut self, _: [i32; 4]) {}
    fn map_repaint(&mut self, _: [i32; 4]) {}
    fn ambient(&mut self, class: u32, id: u32, x: i32, y: i32, extra: u32) {
        self.calls.push((class, id, x, y, extra));
    }
    fn hide_sprite(&mut self, _: usize) {}
}

impl p3_audio::map_ambient::MapSoundEnv for BoardEnv<'_> {
    fn rand(&mut self) -> u32 {
        p3_audio::slots::SlotEnv::rand(self)
    }
    fn list_count(&mut self, list: u32) -> u32 {
        p3_audio::slots::SlotEnv::list_count(self, list)
    }
    fn play(&mut self, id: u32, a: u32, b: u32, volume: u32, pan: u32) -> u32 {
        // 0xFFFFFFFF: canal exclusivo a volumen pleno.
        p3_audio::slots::SlotEnv::play(self, id, a, b, volume.min(255), pan)
    }
    fn sample_ms(&mut self, handle: u32) -> u32 {
        p3_audio::slots::SlotEnv::sample_ms(self, handle)
    }
    fn clear_flags(&mut self, handle: u32) {
        p3_audio::slots::SlotEnv::clear_flags(self, handle)
    }
    fn stop_all(&mut self) {
        self.b.started.clear();
        self.b.stopped = true;
    }
}

struct BoardEnv<'a> {
    b: &'a mut SoundBoard,
    archive: &'a Archive<'a>,
}

impl p3_audio::slots::SlotEnv for BoardEnv<'_> {
    fn ready(&mut self) -> bool {
        true
    }
    fn list_count(&mut self, list: u32) -> u32 {
        self.b.catalog.lists.get(&list).copied().unwrap_or(0)
    }
    fn rand(&mut self) -> u32 {
        // `_rand` del CRT de Microsoft: LCG de 32 bits.
        self.b.rng = self.b.rng.wrapping_mul(214013).wrapping_add(2531011);
        (self.b.rng >> 16) & 0x7FFF
    }
    fn play(&mut self, id: u32, _a: u32, _b: u32, volume: u32, pan: u32) -> u32 {
        if !self.b.catalog.samples.contains_key(&id) {
            return 0;
        }
        let h = self.b.next_handle;
        self.b.next_handle += 1;
        let ms = self.b.measure(self.archive, id);
        self.b.started.insert(h, (self.b.clock, ms, id));
        self.b.requests.push(PlayRequest { id, volume, pan });
        h
    }
    fn sample_ms(&mut self, handle: u32) -> u32 {
        self.b.started.get(&handle).map_or(0, |s| s.1)
    }
    fn clock(&mut self) -> u32 {
        self.b.clock
    }
    fn sample_status(&mut self, handle: u32) -> u32 {
        match self.b.started.get(&handle) {
            Some(&(t0, ms, _)) if self.b.clock.wrapping_sub(t0) < ms => 4,
            _ => 2,
        }
    }
    fn clear_flags(&mut self, handle: u32) {
        self.b.started.remove(&handle);
    }
    fn set_volume(&mut self, _: u32, _: u32) {}
    fn set_pan(&mut self, _: u32, _: u32) {}
}
