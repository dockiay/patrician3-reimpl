//! Resoluciones modernas en las flechas de Options, conservando índices válidos
//! para el código original (su selector solo reserva tres resoluciones).
use p3_render::scene_math::Mem;
use std::{collections::HashMap, path::PathBuf};

const OPTIONS: u32 = 0x006C_BDF0;
const SNAPSHOT: u32 = 0x0045_E3D0;
const RESTORE: u32 = 0x0046_0250;
const APPLY: u32 = 0x0045_DF10;
const DRAW: u32 = 0x0045_E920;
const ARROWS: u32 = 0x0040_2BA0;
const ON_OPEN: u32 = 0x0045_CE70;
const UPDATE: u32 = 0x0045_F1D0;
const NEXT: u32 = 0x0040_2CC0;
const PREV: u32 = 0x0040_2D10;
const STANDARD: [(u32, u32); 3] = [(800, 600), (1024, 768), (1280, 1024)];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Choice {
    Auto,
    Size(u32, u32),
}

impl Choice {
    fn base(self) -> u32 {
        match self {
            Self::Size(w, h) => STANDARD.iter().position(|s| *s == (w, h)).unwrap_or(1) as u32,
            Self::Auto => 1,
        }
    }
    fn modern(self, monitor: (u32, u32)) -> Option<(u32, u32)> {
        match self {
            Self::Auto => Some(monitor),
            Self::Size(w, h) => (!STANDARD.contains(&(w, h))).then_some((w, h)),
        }
    }
    fn text(self) -> String {
        match self {
            Self::Auto => "Auto".into(),
            Self::Size(w, h) => format!("{w} x {h}"),
        }
    }
    fn saved(self) -> String {
        match self {
            Self::Auto => "auto".into(),
            Self::Size(w, h) => format!("{w}x{h}"),
        }
    }
    fn parse(s: &str) -> Option<Self> {
        if s.trim() == "auto" {
            return Some(Self::Auto);
        }
        let (w, h) = s.trim().split_once('x')?;
        let (w, h) = (w.parse().ok()?, h.parse().ok()?);
        (w >= 800 && h >= 600 && w <= 7680 && h <= 4320).then_some(Self::Size(w, h))
    }
}

#[derive(Debug)]
pub struct VideoOptions {
    pub enabled: bool,
    pub applied: Option<Choice>,
    pub pending: Option<Choice>,
    pub path: Option<PathBuf>,
    monitor: (u32, u32),
    choices: Vec<Choice>,
    snapshot: Option<Choice>,
    window: Option<u32>,
    restoring: bool,
    skip: Option<u32>,
    labels: HashMap<Choice, u32>,
}

impl Default for VideoOptions {
    fn default() -> Self {
        Self {
            enabled: false,
            applied: None,
            pending: None,
            path: None,
            monitor: (1920, 1080),
            snapshot: None,
            window: None,
            restoring: false,
            skip: None,
            labels: HashMap::new(),
            choices: vec![
                Choice::Size(800, 600),
                Choice::Size(1024, 768),
                Choice::Size(1280, 1024),
                Choice::Size(1366, 768),
                Choice::Size(1600, 900),
                Choice::Size(1920, 1080),
                Choice::Size(2560, 1440),
                Choice::Size(3440, 1440),
                Choice::Size(3840, 2160),
                Choice::Auto,
            ],
        }
    }
}

impl super::Runtime<'_> {
    /// CLI explícita prevalece sobre el modo guardado. El archivo pertenece al
    /// cliente; las tablas y la configuración comercial conservan sus tres índices.
    pub fn enable_video_options(
        &mut self,
        path: Option<PathBuf>,
        monitor: (u32, u32),
        explicit: Option<Option<(u32, u32)>>,
    ) {
        let choice = match explicit {
            Some(None) => Some(Choice::Auto),
            Some(Some((w, h))) => Some(Choice::Size(w, h)),
            None => path
                .as_ref()
                .and_then(|p| crate::hostfs::current().read_to_string(p).ok())
                .and_then(|s| Choice::parse(&s)),
        };
        let (w, h, _) = super::display::auto_resolution(monitor.0, monitor.1);
        let state = &mut self.video_options;
        state.enabled = true;
        state.path = path;
        state.monitor = (w, h);
        state.applied = choice;
        state.pending = choice;
        if let Some(choice) = choice {
            if !state.choices.contains(&choice) {
                state.choices.insert(state.choices.len() - 1, choice);
            }
            self.display.modern = choice.modern(state.monitor);
        }
    }

    /// El arranque original ya ha cargado su configuración al llegar aquí.
    pub fn restore_video_choice(&mut self, m: &mut super::PageMem) {
        if let Some(choice) = self.video_options.applied {
            let cfg = m.u32_at(0x006C_C3E8);
            if cfg != 0 {
                m.w8(cfg + 0x1f, choice.base() as u8);
            }
        }
    }

    fn video_original(&mut self, m: &mut dyn Mem, f: u32, ecx: u32, esp: u32) -> u32 {
        let args: Vec<u32> = (0..p3_render::ret_pop::ret_pop(f))
            .map(|i| m.r32(esp + 4 + i * 4))
            .collect();
        self.video_options.skip = Some(f);
        self.dispatch(m, f, ecx, &args)
    }

    fn video_arrows(&mut self, m: &mut dyn Mem, selector: u32) {
        let Some(choice) = self.video_options.pending else {
            return;
        };
        let i = self
            .video_options
            .choices
            .iter()
            .position(|c| *c == choice)
            .unwrap_or(1);
        for (off, enabled) in [(0x0c, i > 0), (0x10, i + 1 < self.video_options.choices.len())] {
            let button = m.r32(selector + off);
            if button != 0 {
                let vt = m.r32(button);
                let f = m.r32(vt + 0x34);
                self.dispatch(m, f, button, &[u32::from(enabled)]);
            }
        }
    }

    pub(super) fn video_options_native(&mut self, m: &mut dyn Mem, f: u32, ecx: u32, esp: u32) -> Option<u32> {
        if self.video_options.skip == Some(f) {
            self.video_options.skip = None;
            return None;
        }
        // Al abrir Opciones con el selector de idioma, se juntan las filas del original para dejar sitio a la suya.
        if self.language.enabled && f == ON_OPEN && ecx != 0 {
            let r = self.video_original(m, f, ecx, esp);
            self.language_compact_rows(m, ecx);
            return Some(r);
        }
        // `options_window_update` corre cada fotograma mientras la ventana existe: es el latido que mantiene la fila de idioma.
        if self.language.enabled && f == UPDATE && ecx != 0 && ecx == m.r32(OPTIONS) {
            self.language.beat(self.presents);
        }
        if !self.video_options.enabled || ![SNAPSHOT, RESTORE, APPLY, DRAW, ARROWS, NEXT, PREV].contains(&f) {
            return None;
        }
        let window = m.r32(OPTIONS);
        if window == 0 {
            return None;
        }
        let selector = m.r32(window + 0x1cc);
        if ecx == window && (f == SNAPSHOT || f == DRAW) {
            self.language.beat(self.presents);
        }
        if f == SNAPSHOT && ecx == window {
            let r = self.video_original(m, f, ecx, esp);
            let base = m.r32(window + 0x168).min(2) as usize;
            let choice = self
                .video_options
                .applied
                .unwrap_or(Choice::Size(STANDARD[base].0, STANDARD[base].1));
            self.video_options.window = Some(window);
            self.video_options.pending = Some(choice);
            self.video_options.snapshot = Some(choice);
            m.w32(window + 0x168, choice.base());
            m.w32(selector + 4, choice.base());
            self.video_arrows(m, selector);
            return Some(r);
        }
        if self.video_options.window != Some(window) {
            return None;
        }
        if (f == NEXT || f == PREV) && selector != 0 && ecx == selector {
            let current = self.video_options.pending?;
            let i = self
                .video_options
                .choices
                .iter()
                .position(|c| *c == current)
                .unwrap_or(1);
            let i = if f == NEXT {
                (i + 1).min(self.video_options.choices.len() - 1)
            } else {
                i.saturating_sub(1)
            };
            let choice = self.video_options.choices[i];
            self.video_options.pending = Some(choice);
            self.display.modern = choice.modern(self.video_options.monitor);
            m.w32(selector + 4, choice.base());
            m.w32(window + 0x168, choice.base());
            self.video_arrows(m, selector);
            return Some(choice.base());
        }
        if f == ARROWS && selector != 0 && ecx == selector {
            let r = self.video_original(m, f, ecx, esp);
            self.video_arrows(m, selector);
            return Some(r);
        }
        if ecx != window {
            return None;
        }
        match f {
            DRAW => {
                let choice = self.video_options.pending?;
                let data = match self.video_options.labels.get(&choice) {
                    Some(&data) => data,
                    None => {
                        let holder = self.heap_take(4);
                        self.cstring_set(m, holder, choice.text().as_bytes());
                        let data = m.r32(holder);
                        self.video_options.labels.insert(choice, data);
                        data
                    }
                };
                let slot = 0x0069_AE38 + m.r32(window + 0x168).min(2) * 4;
                let saved = m.r32(slot);
                m.w32(slot, data);
                let r = self.video_original(m, f, ecx, esp);
                m.w32(slot, saved);
                Some(r)
            }
            RESTORE => {
                self.video_options.pending = self.video_options.snapshot;
                if let Some(choice) = self.video_options.pending {
                    self.display.modern = choice.modern(self.video_options.monitor);
                }
                self.video_options.restoring = true;
                let r = self.video_original(m, f, ecx, esp);
                self.video_options.restoring = false;
                self.video_arrows(m, selector);
                Some(r)
            }
            APPLY => {
                let r = self.video_original(m, f, ecx, esp);
                if !self.video_options.restoring {
                    self.video_options.applied = self.video_options.pending;
                    if let (Some(path), Some(choice)) = (&self.video_options.path, self.video_options.applied) {
                        let tmp = path.with_extension("tmp");
                        let fs = crate::hostfs::current();
                        let result = fs.write(&tmp, format!("{}\n", choice.saved()).as_bytes())
                            .and_then(|()| fs.rename(&tmp, path));
                        if let Err(e) = result {
                            eprintln!("no se pudo guardar la resolución: {e}");
                        }
                    }
                }
                Some(r)
            }
            _ => None,
        }
    }
}
