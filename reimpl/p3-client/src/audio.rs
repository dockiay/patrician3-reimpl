//! Salida de sonido del cliente: abre la tarjeta del sistema (WASAPI en Windows, ALSA en Linux con `--features sonido`) y la
//! alimenta con [`p3_audio::live::Mixer`], que recibe las órdenes `AIL_*` del juego original (`p3_mvp::ui_rt::miles`).
//!
//! Sin dispositivo (o con `--sin-sonido` / `P3_SIN_SONIDO`) el juego sigue como antes: el gestor de sonido queda sin
//! dispositivo y no suena nada.
use p3_audio::live::Mixer;
use std::sync::Arc;

/// Salida abierta: mientras viva, suena el mezclador.
pub struct AudioOut {
    pub mixer: Arc<Mixer>,
    #[cfg(any(windows, feature = "sonido"))]
    _stream: cpal::Stream,
}

/// Abre la salida predeterminada a su frecuencia nativa y devuelve el mezclador que la alimenta.
#[cfg(any(windows, feature = "sonido"))]
pub fn open() -> Result<AudioOut, String> {
    use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
    let host = cpal::default_host();
    let device = host.default_output_device().ok_or("no hay salida de sonido")?;
    let config = device
        .default_output_config()
        .map_err(|e| format!("configuración de sonido: {e}"))?;
    let format = config.sample_format();
    let config: cpal::StreamConfig = config.into();
    let channels = usize::from(config.channels.max(1));
    let mixer = Mixer::new(config.sample_rate.0);
    let err = |e: cpal::StreamError| eprintln!("sonido: {e}");
    // Mezcla estéreo y la reparte entre los canales del dispositivo (mono: media; más de dos: el resto en silencio).
    fn fill<T: Copy>(mixer: &Mixer, scratch: &mut Vec<f32>, out: &mut [T], channels: usize, conv: impl Fn(f32) -> T) {
        let frames = out.len() / channels;
        scratch.resize(frames * 2, 0.0);
        mixer.render(scratch);
        for (f, frame) in out.chunks_mut(channels).enumerate() {
            let (l, r) = (scratch[2 * f], scratch[2 * f + 1]);
            for (c, s) in frame.iter_mut().enumerate() {
                *s = conv(match (channels, c) {
                    (1, _) => 0.5 * (l + r),
                    (_, 0) => l,
                    (_, 1) => r,
                    _ => 0.0,
                });
            }
        }
    }
    let m = mixer.clone();
    let mut scratch = Vec::new();
    let stream = match format {
        cpal::SampleFormat::F32 => device.build_output_stream(
            &config,
            move |out: &mut [f32], _| fill(&m, &mut scratch, out, channels, |x| x),
            err,
            None,
        ),
        cpal::SampleFormat::I16 => device.build_output_stream(
            &config,
            move |out: &mut [i16], _| fill(&m, &mut scratch, out, channels, |x| (x * 32767.0) as i16),
            err,
            None,
        ),
        cpal::SampleFormat::U16 => device.build_output_stream(
            &config,
            move |out: &mut [u16], _| {
                fill(&m, &mut scratch, out, channels, |x| {
                    ((x * 32767.0) as i32 + 32768) as u16
                })
            },
            err,
            None,
        ),
        cpal::SampleFormat::I32 => device.build_output_stream(
            &config,
            move |out: &mut [i32], _| {
                fill(&m, &mut scratch, out, channels, |x| {
                    (f64::from(x) * 2147483647.0) as i32
                })
            },
            err,
            None,
        ),
        other => return Err(format!("formato de sonido no admitido: {other:?}")),
    }
    .map_err(|e| format!("salida de sonido: {e}"))?;
    stream.play().map_err(|e| format!("arrancar el sonido: {e}"))?;
    eprintln!(
        "sonido: {} a {} Hz, {channels} canales ({format:?})",
        device.name().unwrap_or_default(),
        config.sample_rate.0
    );
    Ok(AudioOut { mixer, _stream: stream })
}

#[cfg(not(any(windows, feature = "sonido")))]
pub fn open() -> Result<AudioOut, String> {
    Err("compilado sin salida de sonido (en Linux: --features sonido)".into())
}

/// Conecta el sonido al enlazador: el gestor de sonido original abre Miles sobre el mezclador. Las carpetas son donde buscar la
/// música suelta (`Musik\...`): la del juego, la del CPR y la del EXE. Devuelve la salida (hay que conservarla) o `None`.
pub fn attach(rt: &mut p3_mvp::ui_rt::Runtime<'_>, folders: &[&std::path::Path], disabled: bool) -> Option<AudioOut> {
    if disabled || std::env::var_os("P3_SIN_SONIDO").is_some() {
        eprintln!("sonido: desactivado");
        return None;
    }
    match open() {
        Ok(out) => {
            rt.miles.dirs = p3_mvp::ui_rt::miles::default_dirs(folders);
            rt.miles.mixer = Some(out.mixer.clone());
            eprintln!("sonido: música buscada en {:?}", rt.miles.dirs);
            Some(out)
        }
        Err(e) => {
            eprintln!("sonido: {e}; el juego sigue sin sonido");
            None
        }
    }
}
