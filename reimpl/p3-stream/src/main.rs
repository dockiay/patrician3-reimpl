//! `p3-stream`: el juego nativo sin ventana, servido a un navegador por WebSocket (vídeo H.264, sonido PCM, entrada remota).
//!
//! El cliente (iPhone Safari incluido) no descarga el EXE, el CPR ni el wasm: solo recibe fotogramas y sonido y envía ratón,
//! teclado y toques. La partida vive en este proceso; si el navegador se cierra, el juego sigue y se vuelve a entrar donde se dejó.
//! Ver `reimpl/STREAMING.md`.
mod game;
mod hub;
mod server;
mod video;

use std::path::PathBuf;

pub struct Opts {
    pub exe: PathBuf,
    pub cpr: PathBuf,
    pub game_dir: PathBuf,
    pub port: u16,
    pub bind: String,
    pub password: String,
    pub modern: Option<(u32, u32)>,
    pub fps: u32,
    pub stream_fps: u32,
    pub kbps: u32,
    pub audio_rate: u32,
    pub no_sound: bool,
    pub frames: u64,
    pub dump_h264: Option<PathBuf>,
    /// `--idioma CÓDIGO`: fuerza un idioma (si no, el de `idioma.txt` de la carpeta del juego).
    pub language: Option<String>,
    /// `--idiomas CARPETA` (repetible): más sitios donde buscar `p2arch0_<código>.cpr` e `idioma/<código>.pack`.
    pub language_dirs: Vec<PathBuf>,
}

fn usage() -> ! {
    eprintln!(
        "uso: p3-stream --exe Patrician3.exe --cpr p2arch0_eng.cpr [--juego CARPETA] [--puerto 8090] [--escuchar 127.0.0.1]\n\
         \x20      [--moderna ANCHOxALTO] [--fps 60] [--fps-video 30] [--kbps 3000] [--audio-hz 22050] [--sin-sonido] [--idioma spa] [--idiomas CARPETA]\n\
         La contraseña es obligatoria: variable P3_PASSWORD (o --clave-env NOMBRE). Sin ella el servidor no arranca."
    );
    std::process::exit(2);
}

fn parse() -> Opts {
    let mut o = Opts {
        exe: PathBuf::new(),
        cpr: PathBuf::new(),
        game_dir: PathBuf::new(),
        port: 8090,
        bind: "127.0.0.1".into(),
        password: String::new(),
        modern: None,
        fps: 60,
        stream_fps: 30,
        kbps: 3000,
        audio_rate: 22050,
        no_sound: false,
        frames: 0,
        dump_h264: None,
        language: None,
        language_dirs: Vec::new(),
    };
    let mut env_name = "P3_PASSWORD".to_string();
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        let mut val = || args.next().unwrap_or_else(|| usage());
        match a.as_str() {
            "--exe" => o.exe = val().into(),
            "--cpr" => o.cpr = val().into(),
            "--juego" => o.game_dir = val().into(),
            "--puerto" => o.port = val().parse().unwrap_or_else(|_| usage()),
            "--escuchar" => o.bind = val(),
            "--moderna" => {
                let v = val();
                let (w, h) = v.split_once(['x', 'X']).unwrap_or_else(|| usage());
                o.modern = Some((w.parse().unwrap_or_else(|_| usage()), h.parse().unwrap_or_else(|_| usage())));
            }
            "--fps" => o.fps = val().parse().unwrap_or_else(|_| usage()),
            "--fps-video" => o.stream_fps = val().parse().unwrap_or_else(|_| usage()),
            "--kbps" => o.kbps = val().parse().unwrap_or_else(|_| usage()),
            "--audio-hz" => o.audio_rate = val().parse().unwrap_or_else(|_| usage()),
            "--sin-sonido" => o.no_sound = true,
            "--fotogramas" => o.frames = val().parse().unwrap_or_else(|_| usage()),
            "--volcar-h264" => o.dump_h264 = Some(val().into()),
            "--idioma" => o.language = Some(val()),
            "--idiomas" => o.language_dirs.push(val().into()),
            "--clave-env" => env_name = val(),
            _ => usage(),
        }
    }
    if o.exe.as_os_str().is_empty() || o.cpr.as_os_str().is_empty() {
        usage();
    }
    if o.game_dir.as_os_str().is_empty() {
        o.game_dir = o.cpr.parent().map(PathBuf::from).unwrap_or_else(|| ".".into());
    }
    o.password = std::env::var(&env_name).unwrap_or_default();
    if o.password.len() < 6 {
        eprintln!("p3-stream: falta la contraseña (variable {env_name}, mínimo 6 caracteres). Es para tu uso, no para repartir el juego.");
        std::process::exit(2);
    }
    o.fps = o.fps.clamp(10, 120);
    o.stream_fps = o.stream_fps.clamp(5, o.fps);
    o
}

fn main() {
    let o = parse();
    let hub = hub::Hub::new(o.audio_rate);
    if let Err(e) = server::start(&o, hub.clone()) {
        eprintln!("p3-stream: servidor: {e}");
        std::process::exit(1);
    }
    video::start_encoder(&o, hub.clone());
    if let Err(e) = game::run(&o, hub) {
        eprintln!("p3-stream: {e}");
        std::process::exit(1);
    }
}
