//! El juego en este proceso: arranque del original (`ui_rt::menus::{boot, run}`, como `p3-juego menu`) con una «ventana» virtual.
//! En cada `sgl_SwapBuffer` la imagen pasa por la capa SGL portada, se comprueba si cambió, se entrega al codificador y se
//! recoge la entrada de los navegadores. El ritmo (60 fotogramas por segundo) lo pone este módulo.
use crate::hub::{self, Hub};
use crate::video;
use crate::Opts;
use p3_audio::live::Mixer;
use p3_client::device::{self, NativeHost};
use p3_mvp::ui_rt::win32::{InputState, Msg};
use p3_mvp::ui_rt::{PageMem, Runtime, Surface};
use p3_render::sgl_screen::SglScreen;
use std::collections::VecDeque;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::{Duration, Instant};

fn drain_input(hub: &Hub, size: (usize, usize), queue: &mut VecDeque<Msg>, input: &mut InputState) {
    let mut q = hub.input.lock().unwrap();
    while let Some(ev) = q.pop_front() {
        hub::apply(ev, size, input, queue);
    }
}

/// Mezcla el sonido del juego a ritmo real y lo reparte (PCM i16 estéreo, paquetes de 20 ms).
fn audio_thread(mixer: Arc<Mixer>, hub: Arc<Hub>, rate: u32) {
    let frames = (rate / 50) as usize;
    let mut buf = vec![0f32; frames * 2];
    let mut pcm = vec![0i16; frames * 2];
    let mut next = Instant::now();
    loop {
        mixer.render(&mut buf);
        for (o, s) in pcm.iter_mut().zip(&buf) {
            *o = (s.clamp(-1.0, 1.0) * 32767.0) as i16;
        }
        hub.send_audio(&pcm);
        next += Duration::from_millis(20);
        let now = Instant::now();
        if next > now {
            std::thread::sleep(next - now);
        } else if now - next > Duration::from_millis(200) {
            next = now;
        }
    }
}

pub fn run(o: &Opts, hub: Arc<Hub>) -> Result<(), String> {
    let exe = std::fs::read(&o.exe).map_err(|e| format!("ejecutable original: {e}"))?;
    // Idioma como `p3-juego menu`: CPR del idioma elegido (derivado una vez en `<juego>/idioma/`), paquete de cadenas y selector.
    let language = p3_client::idioma::prepare(&o.exe, &o.cpr, &o.game_dir, o.language.as_deref(), &o.language_dirs);
    eprintln!("idioma: {} ({})", language.code, language.cpr.display());
    let cpr = p3_assets::cpr::Bytes::map_file(&language.cpr).map_err(|e| format!("recursos: {e}"))?;
    let archive = p3_assets::cpr::Archive::parse(&cpr).map_err(|e| format!("CPR: {e:?}"))?;
    let mut mem = PageMem::default();
    let (w, h) = (1024usize, 768usize);
    let mut rt = Runtime::new(&mut mem, &exe, &archive, w, h)?;
    rt.surface = Surface::new(w, h, [0, 0, 0, 0xFF]);
    // Como `p3-juego`: el tope de llamadas cuenta por fotograma y empezar o cargar una partida hace millones sin presentar.
    rt.call_limit = 1_000_000_000;
    rt.menus.game_dir = Some(o.game_dir.clone());
    p3_client::idioma::apply(&mut rt, &mut mem, &language);
    eprintln!("carpeta del juego (partidas en save\\ein): {}", o.game_dir.display());

    // Vídeos de las ventanas de suceso (`Videos\*.sks`, `ui_rt/vto.rs`).
    rt.vto.dirs = p3_mvp::ui_rt::miles::default_dirs(&[o.game_dir.as_path(), o.cpr.as_path(), o.exe.as_path()]);
    if o.no_sound {
        eprintln!("sonido: desactivado");
    } else {
        let mixer = Mixer::new(o.audio_rate);
        rt.miles.dirs = p3_mvp::ui_rt::miles::default_dirs(&[o.game_dir.as_path(), o.cpr.as_path(), o.exe.as_path()]);
        rt.miles.mixer = Some(mixer.clone());
        eprintln!("sonido: música buscada en {:?}", rt.miles.dirs);
        let (hub, rate) = (hub.clone(), o.audio_rate);
        std::thread::Builder::new().name("sonido".into()).spawn(move || audio_thread(mixer, hub, rate)).unwrap();
    }

    rt.display.modern = o.modern;
    rt.enable_video_options(Some(o.game_dir.join("video-mode.txt")), (1920, 1080), o.modern.map(Some));

    // Presentación y entrada.
    rt.zoom.enabled = true;
    let mut screen: SglScreen<device::NativeScreen> = device::init_screen(w as i32, h as i32, true)?;
    let mut host = NativeHost::default();
    let start = Instant::now();
    rt.input.time_ms = Some(0);
    let tx = video::encoder_tx();
    let frame_dt = Duration::from_micros(1_000_000 / u64::from(o.fps));
    let send_dt = Duration::from_micros(1_000_000 / u64::from(o.stream_fps));
    let mut next_frame = Instant::now();
    let mut last_sent: Vec<u8> = Vec::new();
    let mut last_size = (0usize, 0usize);
    let mut last_send = Instant::now() - send_dt;
    let mut stat_t = Instant::now();
    let mut stat_frames = 0u32;
    let mut stat_sent = 0u32;
    let mut stat_video = 0u64;
    let mut stat_audio = 0u64;
    let hub_pump = hub.clone();
    let mut last_pump = Instant::now();
    rt.on_pump = Some(Box::new(move |queue, input| {
        // Durante las cargas no hay `SwapBuffer`: la entrada sigue entrando y se puede cerrar.
        if last_pump.elapsed() < Duration::from_millis(50) {
            return true;
        }
        last_pump = Instant::now();
        input.time_ms = Some(start.elapsed().as_millis() as u32);
        let gs = hub_pump.game_size.load(Ordering::Relaxed);
        drain_input(&hub_pump, ((gs >> 16) as usize, (gs & 0xFFFF) as usize), queue, input);
        !hub_pump.quit.load(Ordering::Relaxed)
    }));
    let hub_present = hub.clone();
    rt.on_present = Some(Box::new(move |surface, queue, input| {
        // Cambio de modo de vídeo (menús 800×600, partida a la resolución elegida): `SwitchScreenMode`, como la DLL.
        let (sw, sh) = (surface.width as i32, surface.height as i32);
        if device::active(&mut screen).map(|d| d.size()) != Some((surface.width, surface.height)) && screen.switch_screen_mode(sw, sh, 16) != Ok(1) {
            eprintln!("SwitchScreenMode {sw}x{sh}: fallo");
            return false;
        }
        if let Some(dev) = device::active(&mut screen) {
            if dev.back.len() == surface.rgba.len() {
                dev.back.copy_from_slice(&surface.rgba);
            }
        }
        if screen.swap_buffer(&mut host, 1).is_err() {
            return false;
        }
        let dev = device::active(&mut screen).unwrap();
        let size = dev.size();
        dev.fresh = false;
        hub_present.game_size.store(((size.0 as u64) << 16) | size.1 as u64, Ordering::Relaxed);
        stat_frames += 1;
        let force = hub_present.want_key.load(Ordering::Relaxed);
        let viewers = hub_present.viewers.load(Ordering::Relaxed) > 0;
        if viewers && (force || (last_send.elapsed() >= send_dt && (size != last_size || dev.front != last_sent))) {
            hub_present.want_key.store(false, Ordering::Relaxed);
            let frame = video::Frame { w: size.0, h: size.1, rgba: dev.front.clone(), key: force || size != last_size };
            if tx.try_send(frame).is_ok() {
                last_sent.clear();
                last_sent.extend_from_slice(&dev.front);
                last_size = size;
                last_send = Instant::now();
                stat_sent += 1;
            }
        }
        input.time_ms = Some(start.elapsed().as_millis() as u32);
        drain_input(&hub_present, size, queue, input);
        // Ritmo: un fotograma cada 1/fps (si vamos atrasados no se acumula deuda).
        next_frame += frame_dt;
        let now = Instant::now();
        if next_frame > now {
            std::thread::sleep(next_frame - now);
        } else if now - next_frame > Duration::from_millis(250) {
            next_frame = now;
        }
        if stat_t.elapsed() >= Duration::from_secs(10) {
            let secs = stat_t.elapsed().as_secs_f64();
            let (v, a) = (hub_present.video_bytes.load(Ordering::Relaxed), hub_present.audio_bytes.load(Ordering::Relaxed));
            eprintln!(
                "estado: {:.1} fps de juego, {:.1} fps de vídeo, vídeo {:.0} kbps, sonido {:.0} kbps, espectadores {}, {}x{}, codificar {:.1} ms/fotograma",
                f64::from(stat_frames) / secs,
                f64::from(stat_sent) / secs,
                (v - stat_video) as f64 * 8.0 / 1000.0 / secs,
                (a - stat_audio) as f64 * 8.0 / 1000.0 / secs,
                hub_present.viewers.load(Ordering::Relaxed),
                size.0,
                size.1,
                hub_present.enc_us.swap(0, Ordering::Relaxed) as f64 / 1000.0 / hub_present.enc_n.swap(0, Ordering::Relaxed).max(1) as f64
            );
            (stat_t, stat_frames, stat_sent, stat_video, stat_audio) = (Instant::now(), 0, 0, v, a);
        }
        !hub_present.quit.load(Ordering::Relaxed)
    }));

    let max = o.frames;
    if max != 0 {
        rt.menus.frame_script = Some(Box::new(move |_, frame| u64::from(frame) < max));
    }
    let booted = p3_mvp::ui_rt::menus::boot(&mut rt, &mut mem);
    if booted {
        rt.restore_video_choice(&mut mem);
    }
    eprintln!("game_main_window_initialize: {}", if booted { "bien" } else { "falló" });
    let r = if booted { Some(p3_mvp::ui_rt::menus::run(&mut rt, &mut mem)) } else { None };
    eprintln!("fotogramas {} resultado {r:x?} abortado {}", rt.menus.frames, rt.aborted);
    eprintln!("importaciones sin sustituto: {:?}", rt.import_missing);
    eprintln!("SGL sin sustituto: {:?}", rt.sgl_missing);
    eprintln!("faltan (función, llamante): {:x?}", rt.missing_from);
    if booted {
        Ok(())
    } else {
        Err("el arranque del original falló".into())
    }
}
