//! `Vto.dll` sustituida: los vídeos de las ventanas de suceso (ascensos de rango, boda, piratas, asedio...).
//!
//! No es Bink: `Vto.dll` es un reproductor propio de Ascaron para `Videos\*.sks` («SK10»). Cada archivo es una cabecera de
//! 32 bytes (`"SK10"`, ancho, alto, fotogramas por segundo, número de fotogramas y tres palabras más), una tabla de
//! `n + 1` pares `(desplazamiento, tamaño)` y los fotogramas, cada uno un JPEG completo (JFIF, 400x300 a 15 fps en todos).
//! No llevan sonido. La DLL los lee en un hilo con un anillo de fotogramas y los descodifica con `ijl11.dll`
//! (`DIBChannels = 4`, `DIBColor = IJL_RGBA_FPX`) directamente sobre la textura de memoria que le pasa el juego
//! (`sgl_LockTexture(tex, info, 2)` / `sgl_UnlockTexture`).
//!
//! Los JPEG están guardados con rojo y azul cambiados: `ijl11.dll` escribe R, G, B, 255 (comprobado ejecutando la DLL
//! original en Unicorn) y la textura es B, G, R, A, así que en pantalla salen bien. Aquí se escriben los tres canales en el
//! mismo orden que da el descodificador y alfa 255, igual que el original.
//!
//! Contrato de cada función (leído en `Vto.dll`, `reimpl/VIDEO_SUCESOS.md`):
//! - `SKDN_InitVideo(ruta, _, ms_búfer)` → manejador o 0 si no abre o no es «SK10».
//! - `SKDN_GetVideoInfos(h, info)` → 1 si `h` es 0; si no copia la cabecera (`+0..+0x20`), los indicadores (`+0x20`), el
//!   último fotograma mostrado (`+0x24`) y los saltados (`+0x28`) y devuelve 0.
//! - `SKDN_StartVideo(h, indicadores, _)` → guarda `indicadores | 2`, último mostrado = -1 y el instante de inicio
//!   (`timeGetTime`); 0 = bien. Bit 0: repetir en bucle; bit 2: volver a pintar aunque no cambie el fotograma.
//! - `SKDN_RenderVideoFrame(h, tex)` → fotograma = (ahora − inicio) / (1000 / fps). Pasado el último, se queda en el
//!   último y marca fin. Devuelve 0 si pintó uno nuevo, 50 si no hay fotograma nuevo, 100 al terminar (el juego cierra
//!   el vídeo y libera la textura, `0x46AD3B`) y 1 si falla la descodificación.
//! - `SKDN_CloseVideo(h)` libera.
use super::Runtime;
use p3_render::scene_math::Mem;
use std::collections::HashMap;
use std::path::PathBuf;

/// Primer manejador que se entrega al juego (solo lo guarda y lo devuelve; nunca lo lee como puntero).
const FIRST_HANDLE: u32 = 0x5654_0001;
/// `"SK10"` en little endian.
const MAGIC: u32 = 0x3031_4B53;

/// Un archivo `.sks` ya leído.
pub struct Sks {
    /// Cabecera tal cual: magia, ancho, alto, fps, fotogramas y tres palabras más.
    pub header: [u32; 8],
    /// Fotogramas: (desplazamiento, tamaño) dentro de `data`.
    pub frames: Vec<(u32, u32)>,
    pub data: Vec<u8>,
}

impl Sks {
    pub fn parse(data: Vec<u8>) -> Option<Self> {
        let r = |o: usize| data.get(o..o + 4).map(|b| u32::from_le_bytes(b.try_into().unwrap()));
        let mut header = [0u32; 8];
        for (i, h) in header.iter_mut().enumerate() {
            *h = r(4 * i)?;
        }
        if header[0] != MAGIC {
            return None;
        }
        let n = header[4] as usize;
        // La DLL lee la tabla entera (`n * 8 + 8` bytes) y falla si no está.
        r(0x20 + 8 * n + 4)?;
        let frames = (0..n).map(|i| (r(0x20 + 8 * i).unwrap(), r(0x24 + 8 * i).unwrap())).collect();
        Some(Sks { header, frames, data })
    }

    pub fn width(&self) -> u32 {
        self.header[1]
    }
    pub fn height(&self) -> u32 {
        self.header[2]
    }
    pub fn fps(&self) -> u32 {
        self.header[3]
    }

    /// JPEG del fotograma `i` (vacío si la tabla apunta fuera del archivo; la DLL leería menos y fallaría al descodificar).
    pub fn jpeg(&self, i: usize) -> &[u8] {
        let Some(&(o, n)) = self.frames.get(i) else { return &[] };
        let (o, n) = (o as usize, n as usize);
        self.data.get(o..o.saturating_add(n)).unwrap_or(&[])
    }
}

/// Fotograma descodificado: ancho, alto y tres bytes por píxel en el orden del JPEG (que en estos vídeos es B, G, R).
pub fn decode_frame(jpeg: &[u8]) -> Option<(u32, u32, Vec<u8>)> {
    let mut d = jpeg_decoder::Decoder::new(jpeg);
    let px = d.decode().ok()?;
    let info = d.info()?;
    let (w, h) = (u32::from(info.width), u32::from(info.height));
    let rgb = match info.pixel_format {
        jpeg_decoder::PixelFormat::RGB24 => px,
        jpeg_decoder::PixelFormat::L8 => px.iter().flat_map(|&v| [v, v, v]).collect(),
        _ => return None,
    };
    Some((w, h, rgb))
}

struct Video {
    sks: Sks,
    /// `+0x20`: indicadores de `SKDN_StartVideo` (con el bit 1 siempre puesto).
    flags: u32,
    /// `+0x3C`: instante de inicio (`timeGetTime`).
    start: u32,
    /// `+0xF4`: último fotograma pintado (-1 al empezar).
    last: i32,
    /// `+0x28`: fotogramas saltados.
    dropped: u32,
    /// Bloque `[píxeles, paleta, ancho, alto, paso]` para `sgl_LockTexture`, en memoria emulada.
    info: u32,
}

#[derive(Default)]
pub struct VtoState {
    /// Carpetas donde buscar `Videos\...` (la del juego, la del CPR y la del EXE). Vacío = las de Miles.
    pub dirs: Vec<PathBuf>,
    /// Sin vídeos (`P3_SIN_VIDEO`): `SKDN_InitVideo` falla y la ventana sale con la imagen fija, como antes.
    pub disabled: bool,
    /// Traza de llamadas (`P3_VTO_TRACE`).
    pub trace: bool,
    videos: HashMap<u32, Video>,
    next: u32,
}

impl VtoState {
    pub fn from_env() -> Self {
        Self { disabled: std::env::var_os("P3_SIN_VIDEO").is_some(), trace: std::env::var_os("P3_VTO_TRACE").is_some(), ..Default::default() }
    }

    /// Vídeos abiertos ahora mismo (diagnóstico y pruebas).
    pub fn open_count(&self) -> usize {
        self.videos.len()
    }
}

impl Runtime<'_> {
    /// Archivo `.sks` de la instalación (búsqueda sin distinguir mayúsculas, como la música de Miles).
    fn vto_file(&self, name: &str) -> Option<Vec<u8>> {
        let mut finder = super::miles::MilesState::default();
        finder.dirs = if self.vto.dirs.is_empty() { self.miles.dirs.clone() } else { self.vto.dirs.clone() };
        let path = finder.find_file(name)?;
        crate::hostfs::current().read(&path).ok()
    }

    /// Las seis funciones de `Vto.dll` que importa el juego (todas `cdecl`); `None` si `func` no es de `Vto.dll`.
    pub(super) fn vto_import(&mut self, m: &mut dyn Mem, func: &str, arg: &dyn Fn(&mut dyn Mem, u32) -> u32) -> Option<u32> {
        let name = func.strip_prefix("?SKDN_")?.split('@').next()?.to_string();
        let r = match name.as_str() {
            "Init" | "ShutDown" => 0,
            "InitVideo" => {
                let p = arg(m, 0);
                let path = super::cstr(m, p);
                let text = String::from_utf8_lossy(&path).into_owned();
                let sks = if self.vto.disabled { None } else { self.vto_file(&text).and_then(Sks::parse) };
                if self.vto.trace {
                    eprintln!("SKDN_InitVideo: «{text}» {}", if sks.is_some() { "abierto" } else { "no encontrado o no es SK10" });
                }
                match sks {
                    Some(sks) if sks.fps() != 0 => {
                        let info = self.heap_take(32);
                        super::zero_fill(m, info, 32);
                        if self.vto.next == 0 {
                            self.vto.next = FIRST_HANDLE;
                        }
                        let h = self.vto.next;
                        self.vto.next += 1;
                        self.vto.videos.insert(h, Video { sks, flags: 0, start: 0, last: -1, dropped: 0, info });
                        h
                    }
                    _ => 0,
                }
            }
            "GetVideoInfos" => {
                let (h, out) = (arg(m, 0), arg(m, 1));
                match self.vto.videos.get(&h) {
                    Some(v) => {
                        for (i, w) in v.sks.header.iter().enumerate() {
                            m.w32(out + 4 * i as u32, *w);
                        }
                        m.w32(out + 0x20, v.flags);
                        m.w32(out + 0x24, v.last as u32);
                        m.w32(out + 0x28, v.dropped);
                        0
                    }
                    None => 1,
                }
            }
            "StartVideo" => {
                let (h, flags) = (arg(m, 0), arg(m, 1));
                let now = self.time_get_time();
                match self.vto.videos.get_mut(&h) {
                    Some(v) => {
                        v.flags = flags | 2;
                        v.last = -1;
                        v.dropped = 0;
                        v.start = now;
                        0
                    }
                    None => 1,
                }
            }
            "RenderVideoFrame" => {
                let (h, tex) = (arg(m, 0), arg(m, 1));
                self.vto_render(m, h, tex)
            }
            "CloseVideo" => {
                if let Some(v) = self.vto.videos.remove(&arg(m, 0)) {
                    self.heap_free(m, v.info);
                }
                0
            }
            _ => return None,
        };
        if self.vto.trace {
            let a: Vec<String> = (0..3).map(|i| format!("{:x}", arg(m, i))).collect();
            eprintln!("SKDN_{name}({}) = {r:#x}", a.join(", "));
        }
        Some(r)
    }

    /// `SKDN_RenderVideoFrame` (`0x10001580`).
    fn vto_render(&mut self, m: &mut dyn Mem, h: u32, tex: u32) -> u32 {
        let now = self.time_get_time();
        let Some(v) = self.vto.videos.get_mut(&h) else { return 1 };
        let count = v.sks.frames.len() as i32;
        if count == 0 {
            return 1;
        }
        let ms = (1000 / v.sks.fps()) as i32;
        let mut frame = (now.wrapping_sub(v.start) as i32) / ms;
        if frame < 0 {
            frame = 0;
        }
        // Sin bucle, el hilo lector marca el fin al pedir el fotograma `n` (`0x100011DC`); con todo el archivo en memoria ya
        // está marcado cuando el reloj pasa del último.
        let mut ended = false;
        if frame >= count {
            if v.flags & 1 != 0 {
                frame %= count;
            } else {
                frame = count - 1;
                ended = true;
            }
        }
        let skipped = frame - v.last - 1;
        if skipped > 0 {
            v.dropped = v.dropped.wrapping_add(skipped as u32);
        }
        if skipped <= -1 && v.flags & 4 == 0 {
            v.last = frame;
            return if ended { 100 } else { 50 };
        }
        v.last = frame;
        let info = v.info;
        let decoded = decode_frame(v.sks.jpeg(frame as usize));
        let end_code = if ended { 100 } else { 0 };
        let a = [tex, info, 2];
        if self.sgl_call(m, "sgl_LockTexture", 0, &|_, i| a.get(i as usize).copied().unwrap_or(0)) == 0 {
            return end_code;
        }
        let Some((jw, jh, px)) = decoded else {
            let a = [tex];
            self.sgl_call(m, "sgl_UnlockTexture", 0, &|_, i| a.get(i as usize).copied().unwrap_or(0));
            return 1;
        };
        // Como `ijl11.dll`: `DIBWidth`/`DIBHeight` del JPEG y `DIBPadBytes = paso − ancho_textura * 4`.
        let (buf, tw, th, pitch) = (m.r32(info), m.r32(info + 8), m.r32(info + 12), m.r32(info + 16));
        let stride = (jw * 4).wrapping_add(pitch.wrapping_sub(tw * 4));
        let limit = pitch.saturating_mul(th);
        let mut row = vec![0u8; jw as usize * 4];
        for y in 0..jh {
            let at = y.wrapping_mul(stride);
            if at.saturating_add(jw * 4) > limit {
                break;
            }
            let src = &px[(y * jw * 3) as usize..((y + 1) * jw * 3) as usize];
            for (d, s) in row.chunks_exact_mut(4).zip(src.chunks_exact(3)) {
                d.copy_from_slice(&[s[0], s[1], s[2], 255]);
            }
            m.write_bytes(buf + at, &row);
        }
        let a = [tex];
        self.sgl_call(m, "sgl_UnlockTexture", 0, &|_, i| a.get(i as usize).copied().unwrap_or(0));
        end_code
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sks(n: u32) -> Vec<u8> {
        let mut d = Vec::new();
        for w in [MAGIC, 4, 2, 15, n, 0, 0, 0] {
            d.extend_from_slice(&w.to_le_bytes());
        }
        let base = 0x20 + 8 * (n + 1);
        for i in 0..=n {
            d.extend_from_slice(&(base + i).to_le_bytes());
            d.extend_from_slice(&u32::from(i < n).to_le_bytes());
        }
        d.extend((0..n).map(|i| i as u8));
        d
    }

    #[test]
    fn parses_header_and_table() {
        let s = Sks::parse(sks(3)).unwrap();
        assert_eq!((s.width(), s.height(), s.fps(), s.frames.len()), (4, 2, 15, 3));
        assert_eq!(s.jpeg(2), &[2]);
        assert!(Sks::parse(b"SK11".repeat(20)).is_none());
        let mut short = sks(3);
        short.truncate(0x30);
        assert!(Sks::parse(short).is_none());
    }
}
