//! Pantalla y Direct3D de la capa SGL (`ddraw_Dll.dll`): `InitScreen`, `SwitchScreenMode`, `SwapBuffer`, `CleanUp`,
//! `GrabScreen`, `MessageBox`/`EnableMessageBoxes`, `SetArchive`, `GetProjectionData`, `DrawScene`, `Begin3D`/`Finish3D`
//! y `InitHardware`.
//!
//! La DLL envuelve un objeto dispositivo de la plataforma (0x7C bytes, ctor `0x05024630`; DirectDraw/Direct3D). Aquí ese
//! objeto es el rasgo [`ScreenDevice`], que el cliente implementa sobre su ventana; [`SglScreen`] reproduce la lógica de
//! la DLL a su alrededor: qué métodos llama, en qué orden, con qué argumentos y qué devuelve. El oráculo
//! `reimpl/oracle/x86_sgl_screen.py` ejecuta la DLL con un dispositivo simulado que anota cada llamada y
//! `tests/sgl_screen.rs` compara las anotaciones y los resultados.
//!
//! Hechos de la DLL que se conservan:
//! - `[0x05080B40]` (pantalla principal) no la escribe ninguna instrucción: vale 0. Por eso `CleanUp` (=
//!   `DestroyActiveScreen` `0x05019F30`) o un `InitScreen` cuyo `activate` falla, con una pantalla activa, leen la tabla
//!   de un objeto nulo y fallan: aquí devuelven [`Fault`]. Sin pantalla activa no hacen nada (y el `InitScreen` fallido no
//!   destruye el dispositivo nuevo).
//! - `SwapBuffer`, `SwitchScreenMode` y `GrabScreen` sin pantalla activa también acceden a un nulo según los bits; se
//!   devuelve [`Fault`] en los mismos casos.
//! - `DrawScene` devuelve 0 si no hay mundo 3D (`[0x0506E928]`); el EXE nunca llama a `CreateWorld`, así que siempre 0.
//! - `Begin3D`/`Finish3D` son `ret` y `InitHardware` inicializa subsistemas de la DLL sin estado visible para el EXE
//!   salvo el resultado (1 si hay dispositivo DirectDraw): aquí lo da el anfitrión.
//! - `GetProjectionData` repite exactamente las operaciones x87 (ver [`projection_data`]).

/// Fallo de la DLL original (acceso a puntero nulo). El cliente no debe provocarlo; se modela para conservar la paridad.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fault;

/// Modo que la DLL pasa al dispositivo (`+0xC4`) y lee de él (`+0xC8`): ancho, alto, bits y ventana (0/1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DeviceMode {
    pub width: i32,
    pub height: i32,
    pub bpp: i32,
    pub windowed: i32,
}

/// Estructura de modo de `InitScreen` (los 16 primeros bytes): la cuarta palabra es la frecuencia, que la DLL guarda pero
/// no pasa al dispositivo.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScreenMode {
    pub width: i32,
    pub height: i32,
    pub bpp: i32,
    pub refresh: i32,
}

/// Modo por defecto (`0x05080EEC`) que `InitScreen` rellena si no recibe ninguno.
pub const DEFAULT_MODE: ScreenMode = ScreenMode { width: 640, height: 480, bpp: 16, refresh: 60 };

/// Ajustes leídos de la configuración de la DLL (`0x05080B8C`, `0x05080B90`, `0x05080BEC/F0/F8/F4`); 0 = sin ajuste.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ScreenConfig {
    pub force_fullscreen: bool,
    pub force_windowed: bool,
    pub width: i32,
    pub height: i32,
    pub bpp: i32,
    pub refresh: i32,
}

/// Objeto dispositivo de la plataforma. Cada método corresponde a una entrada de su tabla en la DLL.
pub trait ScreenDevice {
    /// `+0xBC (hwnd, a3)`: asocia la ventana.
    fn attach(&mut self, hwnd: u32, a3: u32);
    /// `+0xC4`: fija el modo; falso si no se puede.
    fn set_mode(&mut self, mode: DeviceMode) -> bool;
    /// `+0xC8`: modo actual.
    fn get_mode(&mut self) -> DeviceMode;
    /// `+0xCC`: activa el dispositivo; verdadero si pasa a ser la pantalla activa.
    fn activate(&mut self) -> bool;
    /// `+0x00 (1)`: destruye el dispositivo.
    fn destroy(&mut self);
    /// `+0xC0 (flags)`: presenta el búfer trasero; -1 si no puede (entonces la DLL copia).
    fn present(&mut self, flags: u32) -> i32;
    /// `+0x48 (0,0,0,0,ancho,alto,flags)`: copia el búfer trasero sobre el frontal.
    fn blit_back_to_front(&mut self, width: i32, height: i32, flags: u32);
    /// Rectángulo de la superficie frontal (`[dev+0x6C]+0x60`: x0, y0, x1, y1).
    fn front_rect(&self) -> [i32; 4];
    /// Campo `+0x50`: modo ventana actual.
    fn windowed(&self) -> i32;
    /// `0x05025B00`: espera a que acabe la copia pendiente (bit 3 de `SwapBuffer`).
    fn wait_blit(&mut self);
    /// `[superficie frontal]+0x34 (formato, nombre)`: guarda una captura.
    fn grab(&mut self, format: i32, name: &str) -> u32;
    /// `0x05025C60 (texto, "SGL", 0x10)`: aviso modal dentro de la pantalla.
    fn message_box(&mut self, text: &str, caption: &str, style: u32) -> u32;
}

/// Servicios del anfitrión que no dependen de un dispositivo concreto.
pub trait ScreenHost {
    /// `SetRenderSource (0x0501D4A0)`: cambia la fuente de copia y devuelve la anterior.
    fn set_render_source(&mut self, handle: i32) -> i32;
    /// `0x0501DF70` / `0x0501DF50`: reloj de cuadros (medir el último y empezar el siguiente).
    fn frame_tick(&mut self);
    fn frame_start(&mut self);
    /// `0x05025B60`: espera al retrazado vertical con el objeto DirectDraw global (bit 1 de `SwapBuffer`).
    fn wait_vsync(&mut self);
    /// `MessageBoxA(0, texto, "SGL", 0x10)` cuando no hay pantalla.
    fn os_message_box(&mut self, text: &str, caption: &str, style: u32) -> u32;
}

/// Estado de pantalla de la DLL.
pub struct SglScreen<D: ScreenDevice> {
    pub devices: Vec<D>,
    /// Índice del dispositivo activo (`[0x05080B3C]`).
    pub active: Option<usize>,
    pub config: ScreenConfig,
    pub default_mode: ScreenMode,
    /// `[0x05080B68]`.
    pub archive: i32,
    /// `[0x05080F08]`: número de la próxima captura sin nombre.
    pub grab_counter: i32,
    /// `[0x0505FE6C]`: avisos habilitados (1 al cargar).
    pub message_boxes: i32,
    /// Trece contadores de cuadro (`0x05064278..0x050642AC`) que `SwapBuffer` pone a cero.
    pub frame_counters: [u32; 13],
}

impl<D: ScreenDevice> Default for SglScreen<D> {
    fn default() -> Self {
        Self::new()
    }
}

impl<D: ScreenDevice> SglScreen<D> {
    pub fn new() -> Self {
        SglScreen {
            devices: Vec::new(),
            active: None,
            config: ScreenConfig::default(),
            default_mode: DEFAULT_MODE,
            archive: 0,
            grab_counter: 0,
            message_boxes: 1,
            frame_counters: [0; 13],
        }
    }

    fn active_dev(&mut self) -> Option<&mut D> {
        let i = self.active?;
        self.devices.get_mut(i)
    }

    /// `sgl_InitScreen (hwnd, windowed, a3, mode)` `0x05019DF0`. Con `mode == None` usa (y modifica) el modo por defecto
    /// de la DLL. Devuelve el índice del dispositivo nuevo o `None` si no se activó.
    pub fn init_screen(&mut self, dev: D, hwnd: u32, windowed: i32, a3: u32, mode: Option<&mut ScreenMode>)
        -> Result<Option<usize>, Fault> {
        let cfg = self.config;
        let m: &mut ScreenMode = match mode {
            Some(m) => m,
            None => {
                self.default_mode = DEFAULT_MODE;
                &mut self.default_mode
            }
        };
        let mut win = if cfg.force_fullscreen { 0 } else { windowed };
        if cfg.force_windowed {
            win = 1;
        }
        if cfg.width != 0 {
            m.width = cfg.width;
        }
        if cfg.height != 0 {
            m.height = cfg.height;
        }
        if cfg.bpp != 0 {
            m.bpp = cfg.bpp;
        }
        if cfg.refresh != 0 {
            m.refresh = cfg.refresh;
        }
        let dm = DeviceMode { width: m.width, height: m.height, bpp: m.bpp, windowed: win };
        self.devices.push(dev);
        let i = self.devices.len() - 1;
        let d = &mut self.devices[i];
        d.attach(hwnd, a3);
        d.set_mode(dm);
        if d.activate() {
            self.active = Some(i);
            return Ok(Some(i));
        }
        // Rama de error: igual que DestroyActiveScreen con la pantalla principal nula.
        if self.active.is_some() {
            return Err(Fault);
        }
        Ok(None)
    }

    /// `sgl_CleanUp` = `DestroyActiveScreen` `0x05019F30`.
    pub fn clean_up(&mut self) -> Result<(), Fault> {
        if self.active.is_some() {
            return Err(Fault);
        }
        Ok(())
    }

    /// `sgl_SwitchScreenMode (&{ancho, alto, bits})` `0x05019F90`: 1 si cambia; si no, vuelve al modo anterior y 0.
    pub fn switch_screen_mode(&mut self, width: i32, height: i32, bpp: i32) -> Result<i32, Fault> {
        let i = self.active.ok_or(Fault)?;
        let d = &mut self.devices[i];
        let saved = d.get_mode();
        let new = DeviceMode { width, height, bpp, windowed: d.windowed() };
        if d.set_mode(new) && d.activate() {
            return Ok(1);
        }
        d.set_mode(saved);
        d.activate();
        Ok(0)
    }

    /// `sgl_SwapBuffer (flags)` `0x0501A1B0`. Bit 0: presentar (si falla, copiar); sin bit 0: copiar. Bit 3: esperar a la
    /// copia; bit 1: esperar al retrazado. Siempre devuelve 0.
    pub fn swap_buffer<H: ScreenHost>(&mut self, host: &mut H, flags: u32) -> Result<u32, Fault> {
        if let Some(d) = self.active_dev() {
            let copy = flags & 1 == 0 || d.present(flags) == -1;
            if copy {
                let prev = host.set_render_source(-3);
                let r = d.front_rect();
                d.blit_back_to_front(r[2] - r[0], r[3] - r[1], flags);
                host.set_render_source(prev);
            }
        }
        if flags & 8 != 0 {
            self.active_dev().ok_or(Fault)?.wait_blit();
        }
        if flags & 2 != 0 {
            host.wait_vsync();
        }
        host.frame_tick();
        host.frame_start();
        self.frame_counters = [0; 13];
        Ok(0)
    }

    /// `sgl_GrabScreen (formato, nombre)` `0x0501A390`. Sin nombre usa `grabs/Grab_%.4d` con el contador. Formatos 1-2
    /// `.bmp`, 3-6 `.tga`, 110-200 en decenas `.jpg`; el resto sin extensión.
    pub fn grab_screen(&mut self, format: i32, name: Option<&str>) -> u32 {
        let mut s = match name {
            Some(n) => n.to_string(),
            None => {
                let n = self.grab_counter;
                self.grab_counter = self.grab_counter.wrapping_add(1);
                format!("grabs/Grab_{}", c_04d(n))
            }
        };
        s.push_str(grab_extension(format));
        match self.active_dev() {
            Some(d) => d.grab(format, &s),
            None => 0,
        }
    }

    /// `sgl_EnableMessageBoxes (v)` `0x05015D70`.
    pub fn enable_message_boxes(&mut self, v: i32) {
        self.message_boxes = v;
    }

    /// `sgl_MessageBox (texto)` `0x05015D30`: 0 si están deshabilitados; si no, aviso de la pantalla o del sistema.
    pub fn message_box<H: ScreenHost>(&mut self, host: &mut H, text: &str) -> u32 {
        if self.message_boxes == 0 {
            return 0;
        }
        match self.active_dev() {
            Some(d) => d.message_box(text, "SGL", 0x10),
            None => host.os_message_box(text, "SGL", 0x10),
        }
    }

    /// `sgl_SetArchive (v)` `0x05002D80`: devuelve el anterior.
    pub fn set_archive(&mut self, v: i32) -> i32 {
        std::mem::replace(&mut self.archive, v)
    }
}

/// `"%.4d"` del CRT.
fn c_04d(n: i32) -> String {
    if n < 0 {
        format!("-{:04}", (n as i64).unsigned_abs())
    } else {
        format!("{:04}", n)
    }
}

/// Extensión que `GrabScreen` añade según el formato (tabla `0x0501A490`).
pub fn grab_extension(format: i32) -> &'static str {
    match format {
        1 | 2 => ".bmp",
        3..=6 => ".tga",
        110..=200 if format % 10 == 0 => ".jpg",
        _ => "",
    }
}

/// Servicios de `sgl_InitHardware` `0x05019D50`.
pub trait HardwareHost {
    /// `AIM_SETVENDOR(0, 0)` de AIM.dll.
    fn aim_set_vendor(&mut self) -> bool;
    /// `arc_SetVendor(0, 0)` de Archiver.dll.
    fn arc_set_vendor(&mut self) -> bool;
    /// Arranque de los subsistemas de la DLL (texturas, fuentes, 3D...) y búsqueda del controlador DirectDraw
    /// (`0x05002EB0`); verdadero si hay controlador (`[0x05080E54] != 0`).
    fn init_driver(&mut self) -> bool;
}

/// `sgl_InitHardware` `0x05019D50`: 0 si falla un `SetVendor`; si no, 1 si hay controlador. La comprobación previa
/// `0x05010C30` siempre da 1 y la bandera de «ya iniciado» `[0x05080F00]` nunca se escribe, así que siempre arranca.
pub fn init_hardware<H: HardwareHost>(host: &mut H) -> i32 {
    if !host.aim_set_vendor() || !host.arc_set_vendor() {
        return 0;
    }
    host.init_driver() as i32
}

/// `sgl_DrawScene` `0x05019970` sin mundo 3D: 0.
pub fn draw_scene_without_world() -> u32 {
    0
}

/// `sgl_Begin3D` `0x0500F810` y `sgl_Finish3D` `0x0500F820`: no hacen nada.
pub fn begin_3d() {}
pub fn finish_3d() {}

/// Precisión de la FPU x87 al llamar a `GetProjectionData`. Direct3D deja `Single` al crear el dispositivo (salvo
/// `DDSCL_FPUPRESERVE`); `Extended` es la de arranque de un proceso Win32.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FpuPrecision {
    Single,
    Extended,
}

/// Entradas de `GetProjectionData`: `[0x0506E8DC]`, `[0x0506E8E4]`, `[0x0506E7F8]`, `[0x0506E7F4]`, `[0x0506E95C]` y
/// `[0x0506E958]` (bits de `f32`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProjectionInput {
    pub a: u32,
    pub b: u32,
    pub c: u32,
    pub d: u32,
    pub sx: u32,
    pub sy: u32,
}

/// `sgl_GetProjectionData (out)` `0x05019760`: doce `f32` (como bits) en el orden de `out[0..12]`. Repite la secuencia
/// x87: `i1 = 1/sx`, `i2 = 1/sy` en registro; `out[4] = a*i1`, `out[5] = b*i2`, `out[0] = c*i1`, `out[1] = d*i2`,
/// `out[2] = 1/out[0]` (desde la memoria), `out[3] = 1/(d*i2)` (desde el registro, sin redondear a `f32`),
/// `out[6] = c`, `out[7] = d`, `out[8] = 1/c`, `out[9] = 1/d`, `out[10] = a`, `out[11] = b`.
pub fn projection_data(i: ProjectionInput, fpu: FpuPrecision) -> [u32; 12] {
    let p = match fpu {
        FpuPrecision::Single => 24,
        FpuPrecision::Extended => 64,
    };
    let one = X::load(0x3F80_0000);
    let i1 = one.div(X::load(i.sx), p);
    let i2 = one.div(X::load(i.sy), p);
    let mut o = [0u32; 12];
    o[10] = i.a;
    o[6] = i.c;
    o[11] = i.b;
    o[7] = i.d;
    o[4] = X::load(i.a).mul(i1, p).store();
    o[5] = X::load(i.b).mul(i2, p).store();
    o[0] = X::load(i.c).mul(i1, p).store();
    let dy = X::load(i.d).mul(i2, p);
    o[1] = dy.store();
    o[2] = one.div(X::load(o[0]), p).store();
    o[3] = one.div(dy, p).store();
    o[8] = one.div(X::load(o[6]), p).store();
    o[9] = one.div(X::load(o[7]), p).store();
    o
}

/// Valor de registro x87 con la mantisa redondeada a `p` bits y exponente sin límite práctico.
#[derive(Debug, Clone, Copy)]
enum X {
    Zero(bool),
    Inf(bool),
    /// Bits `f32` que se guardarían.
    NaN(u32),
    /// `mant` con el bit 63 a uno; valor = `mant * 2^(exp - 63)`.
    Fin { neg: bool, exp: i32, mant: u64 },
}

/// NaN por defecto del x87 (`real indefinite`) guardado como `f32`.
const DEFAULT_NAN: u32 = 0xFFC0_0000;

impl X {
    fn load(bits: u32) -> X {
        let neg = bits >> 31 != 0;
        let e = ((bits >> 23) & 0xFF) as i32;
        let f = bits & 0x7F_FFFF;
        match (e, f) {
            (0, 0) => X::Zero(neg),
            (0xFF, 0) => X::Inf(neg),
            (0xFF, _) => X::NaN(bits | 0x40_0000),
            (0, _) => {
                let lz = (f as u64).leading_zeros() as i32; // f < 2^23
                X::Fin { neg, exp: -126 - (lz - 40), mant: (f as u64) << lz }
            }
            _ => X::Fin { neg, exp: e - 127, mant: ((f | 0x80_0000) as u64) << 40 },
        }
    }

    /// Redondea `n` (no nulo; valor = `n * 2^(e - 127)` con su bit alto en cualquier posición) a `p` bits, al par.
    fn round(neg: bool, mut e: i32, n: u128, sticky: bool, p: u32) -> X {
        let lz = n.leading_zeros();
        let n = n << lz;
        e -= lz as i32;
        let drop = 128 - p;
        let mut m = n >> drop;
        let rest = n & ((1u128 << drop) - 1);
        let half = 1u128 << (drop - 1);
        if rest > half || (rest == half && (sticky || m & 1 == 1)) {
            m += 1;
        }
        if m >> p != 0 {
            m >>= 1;
            e += 1;
        }
        X::Fin { neg, exp: e, mant: (m as u64) << (64 - p) }
    }

    fn mul(self, o: X, p: u32) -> X {
        use X::*;
        match (self, o) {
            (NaN(b), _) | (_, NaN(b)) => NaN(b),
            (Zero(a), Inf(b)) | (Inf(a), Zero(b)) => {
                let _ = (a, b);
                NaN(DEFAULT_NAN)
            }
            (Inf(a), Inf(b)) | (Inf(a), Fin { neg: b, .. }) | (Fin { neg: a, .. }, Inf(b)) => Inf(a ^ b),
            (Zero(a), Zero(b)) | (Zero(a), Fin { neg: b, .. }) | (Fin { neg: a, .. }, Zero(b)) => Zero(a ^ b),
            (Fin { neg: na, exp: ea, mant: ma }, Fin { neg: nb, exp: eb, mant: mb }) => {
                let n = ma as u128 * mb as u128; // valor = n * 2^(ea + eb - 126)
                X::round(na ^ nb, ea + eb + 1, n, false, p)
            }
        }
    }

    fn div(self, o: X, p: u32) -> X {
        use X::*;
        match (self, o) {
            (NaN(b), _) | (_, NaN(b)) => NaN(b),
            (Zero(_), Zero(_)) | (Inf(_), Inf(_)) => NaN(DEFAULT_NAN),
            (Inf(a), Zero(b)) | (Inf(a), Fin { neg: b, .. }) => Inf(a ^ b),
            (Fin { neg: a, .. }, Zero(b)) => Inf(a ^ b),
            (Zero(a), Inf(b)) | (Zero(a), Fin { neg: b, .. }) | (Fin { neg: a, .. }, Inf(b)) => Zero(a ^ b),
            (Fin { neg: na, exp: ea, mant: ma }, Fin { neg: nb, exp: eb, mant: mb }) => {
                // Ocho bits más que los 65 de `(ma << 64) / mb`, para que quede guarda al redondear a 64.
                let (num, d) = ((ma as u128) << 64, mb as u128);
                let (q1, r1) = (num / d, num % d);
                let q = (q1 << 8) | ((r1 << 8) / d); // valor = q * 2^(ea - eb - 72)
                let r = (r1 << 8) % d;
                X::round(na ^ nb, ea - eb - 72 + 127, q, r != 0, p)
            }
        }
    }

    /// `fst dword`: a `f32` al par, con subnormales y desbordamiento a infinito.
    fn store(self) -> u32 {
        match self {
            X::Zero(n) => (n as u32) << 31,
            X::Inf(n) => ((n as u32) << 31) | 0x7F80_0000,
            X::NaN(b) => b,
            X::Fin { neg, exp, mant } => {
                let s = (neg as u32) << 31;
                if exp >= -126 {
                    let X::Fin { exp, mant, .. } = X::round(neg, exp + 64, mant as u128, false, 24) else { unreachable!() };
                    if exp > 127 {
                        return s | 0x7F80_0000;
                    }
                    s | (((exp + 127) as u32) << 23) | (((mant >> 40) as u32) & 0x7F_FFFF)
                } else {
                    // k = round(valor * 2^149) = round(mant * 2^(exp + 86))
                    let shift = -(exp + 86); // > 40
                    if shift > 100 {
                        return s;
                    }
                    let (mant, shift) = (mant as u128, shift as u32);
                    let k = mant >> shift;
                    let rest = mant & ((1u128 << shift) - 1);
                    let half = 1u128 << (shift - 1);
                    let k = if rest > half || (rest == half && k & 1 == 1) { k + 1 } else { k };
                    s | k as u32
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grab_names() {
        assert_eq!(grab_extension(1), ".bmp");
        assert_eq!(grab_extension(6), ".tga");
        assert_eq!(grab_extension(100), "");
        assert_eq!(grab_extension(110), ".jpg");
        assert_eq!(grab_extension(115), "");
        assert_eq!(c_04d(7), "0007");
        assert_eq!(c_04d(-7), "-0007");
    }

    #[test]
    fn projection_simple() {
        let f = |v: f32| v.to_bits();
        let o = projection_data(
            ProjectionInput { a: f(2.0), b: f(3.0), c: f(4.0), d: f(5.0), sx: f(2.0), sy: f(4.0) },
            FpuPrecision::Single,
        );
        assert_eq!(o[4], f(1.0));
        assert_eq!(o[5], f(0.75));
        assert_eq!(o[0], f(2.0));
        assert_eq!(o[1], f(1.25));
        assert_eq!(o[2], f(0.5));
        assert_eq!(o[3], f(0.8));
        assert_eq!(o[8], f(0.25));
    }
}
