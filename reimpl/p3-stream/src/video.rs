//! Codificador H.264 (OpenH264, contenido de pantalla en tiempo real) en su hilo: el juego le entrega copias RGBA y nunca espera.
use crate::hub::Hub;
use crate::Opts;
use openh264::encoder::{BitRate, Encoder, EncoderConfig, FrameRate, UsageType};
use openh264::formats::{RgbaSliceU8, YUVBuffer};
use openh264::OpenH264API;
use std::io::Write;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::Arc;

pub struct Frame {
    pub w: usize,
    pub h: usize,
    pub rgba: Vec<u8>,
    pub key: bool,
}

/// Canal hacia el codificador; el juego usa `try_send` y descarta el fotograma si el codificador va atrasado.
pub type FrameTx = SyncSender<Frame>;

pub fn start_encoder(o: &Opts, hub: Arc<Hub>) {
    let (tx, rx) = sync_channel::<Frame>(2);
    ENCODER_TX.set(tx).ok();
    let (kbps, fps) = (o.kbps, o.stream_fps);
    let dump = o.dump_h264.clone();
    std::thread::Builder::new().name("codificador".into()).spawn(move || encode_loop(rx, hub, kbps, fps, dump)).unwrap();
}

static ENCODER_TX: std::sync::OnceLock<FrameTx> = std::sync::OnceLock::new();

pub fn encoder_tx() -> FrameTx {
    ENCODER_TX.get().expect("codificador no iniciado").clone()
}

fn make_encoder(kbps: u32, fps: u32) -> Result<Encoder, String> {
    let cfg = EncoderConfig::new()
        .usage_type(UsageType::ScreenContentRealTime)
        .bitrate(BitRate::from_bps(kbps * 1000))
        .max_frame_rate(FrameRate::from_hz(fps as f32))
        .skip_frames(false);
    Encoder::with_api_config(OpenH264API::from_source(), cfg).map_err(|e| format!("OpenH264: {e}"))
}

fn encode_loop(rx: Receiver<Frame>, hub: Arc<Hub>, kbps: u32, fps: u32, dump: Option<std::path::PathBuf>) {
    let mut enc: Option<(usize, usize, Encoder)> = None;
    let mut dump = dump.and_then(|p| std::fs::File::create(p).ok());
    let mut out = Vec::new();
    while let Ok(f) = rx.recv() {
        // Dimensiones pares (el formato 4:2:0 lo exige): se recorta la última fila/columna si hace falta.
        let (w, h) = (f.w & !1, f.h & !1);
        if w == 0 || h == 0 {
            continue;
        }
        let rgba = if (w, h) == (f.w, f.h) {
            f.rgba
        } else {
            let mut v = Vec::with_capacity(w * h * 4);
            for y in 0..h {
                v.extend_from_slice(&f.rgba[y * f.w * 4..y * f.w * 4 + w * 4]);
            }
            v
        };
        if enc.as_ref().map(|(ew, eh, _)| (*ew, *eh)) != Some((w, h)) {
            match make_encoder(kbps, fps) {
                Ok(e) => enc = Some((w, h, e)),
                Err(e) => {
                    eprintln!("p3-stream: {e}");
                    continue;
                }
            }
        }
        let (_, _, e) = enc.as_mut().unwrap();
        if f.key {
            e.force_intra_frame();
        }
        let t0 = std::time::Instant::now();
        let yuv = YUVBuffer::from_rgba8_source(RgbaSliceU8::new(&rgba, (w, h)));
        let res = e.encode(&yuv);
        hub.enc_us.fetch_add(t0.elapsed().as_micros() as u64, std::sync::atomic::Ordering::Relaxed);
        hub.enc_n.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        match res {
            Ok(bs) => {
                out.clear();
                bs.write_vec(&mut out);
                let key = matches!(bs.frame_type(), openh264::encoder::FrameType::IDR | openh264::encoder::FrameType::I);
                if out.is_empty() {
                    continue;
                }
                if let Some(d) = dump.as_mut() {
                    let _ = d.write_all(&out);
                }
                hub.send_video(key, w as u32, h as u32, &out);
            }
            Err(e) => eprintln!("p3-stream: codificación: {e}"),
        }
    }
}
