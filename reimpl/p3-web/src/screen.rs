//! COPIA de `p3-client/src/device.rs` (sin `winit`): hasta que el dispositivo de pantalla suba a `p3-mvp` (pendiente, para no tocar el cliente nativo desde WEB-3).
//! Dispositivo de pantalla de la plataforma para la capa SGL portada (`p3_render::sgl_screen`): lo que en el original era el
//! objeto DirectDraw de `ddraw_Dll.dll` (0x7C bytes, ctor `0x05024630`). Tiene un búfer trasero RGBA del tamaño del modo; `present`
//! lo pasa al frontal, que es lo que muestra la ventana nativa. La lógica de la DLL alrededor (`InitScreen`, `SwapBuffer`,
//! `SwitchScreenMode`…) es la de [`SglScreen`], con paridad contra la DLL.
use p3_render::sgl_screen::{DeviceMode, ScreenDevice, ScreenHost, SglScreen};

/// Superficies del dispositivo.
#[derive(Debug, Default, Clone)]
pub struct NativeScreen {
    pub hwnd: u32,
    pub mode: DeviceMode,
    /// Búfer trasero (donde se dibuja) y frontal (lo presentado), RGBA de `mode.width × mode.height`.
    pub back: Vec<u8>,
    pub front: Vec<u8>,
    /// Se presentó un fotograma nuevo desde la última vez que la ventana lo recogió.
    pub fresh: bool,
    pub active: bool,
    /// Capturas pedidas por `GrabScreen` (formato, nombre).
    pub grabs: Vec<(i32, String)>,
    /// Avisos de `sgl_MessageBox` con pantalla.
    pub messages: Vec<String>,
}

impl NativeScreen {
    fn resize(&mut self) {
        let n = (self.mode.width.max(0) as usize) * (self.mode.height.max(0) as usize) * 4;
        self.back.resize(n, 0);
        self.front.resize(n, 0);
    }

    pub fn size(&self) -> (usize, usize) {
        (self.mode.width.max(0) as usize, self.mode.height.max(0) as usize)
    }
}

impl ScreenDevice for NativeScreen {
    fn attach(&mut self, hwnd: u32, _a3: u32) {
        self.hwnd = hwnd;
    }
    fn set_mode(&mut self, mode: DeviceMode) -> bool {
        // Profundidades que admitía el juego (16 por defecto, 32 en las opciones); la imagen se guarda siempre en RGBA.
        if mode.width <= 0 || mode.height <= 0 || !matches!(mode.bpp, 8 | 15 | 16 | 24 | 32) {
            return false;
        }
        self.mode = mode;
        self.resize();
        true
    }
    fn get_mode(&mut self) -> DeviceMode {
        self.mode
    }
    fn activate(&mut self) -> bool {
        self.active = self.mode.width > 0;
        self.active
    }
    fn destroy(&mut self) {
        self.active = false;
    }
    fn present(&mut self, _flags: u32) -> i32 {
        std::mem::swap(&mut self.back, &mut self.front);
        // DirectDraw con `Flip` deja en el trasero el contenido anterior; el juego repinta solo lo sucio, así que se conserva
        // el último fotograma también en el trasero (como el `Blt` de copia).
        self.back.copy_from_slice(&self.front);
        self.fresh = true;
        0
    }
    fn blit_back_to_front(&mut self, _width: i32, _height: i32, _flags: u32) {
        self.front.copy_from_slice(&self.back);
        self.fresh = true;
    }
    fn front_rect(&self) -> [i32; 4] {
        [0, 0, self.mode.width, self.mode.height]
    }
    fn windowed(&self) -> i32 {
        self.mode.windowed
    }
    fn wait_blit(&mut self) {}
    fn grab(&mut self, format: i32, name: &str) -> u32 {
        self.grabs.push((format, name.to_string()));
        1
    }
    fn message_box(&mut self, text: &str, _caption: &str, _style: u32) -> u32 {
        eprintln!("SGL: {text}");
        self.messages.push(text.to_string());
        1
    }
}

/// Anfitrión de [`SglScreen`]: reloj de cuadros y fuente de copia (la ventana se encarga del ritmo).
#[derive(Debug, Default)]
pub struct NativeHost {
    pub render_source: i32,
    pub frames: u64,
}

impl ScreenHost for NativeHost {
    fn set_render_source(&mut self, handle: i32) -> i32 {
        std::mem::replace(&mut self.render_source, handle)
    }
    fn frame_tick(&mut self) {
        self.frames += 1;
    }
    fn frame_start(&mut self) {}
    fn wait_vsync(&mut self) {}
    fn os_message_box(&mut self, text: &str, _caption: &str, _style: u32) -> u32 {
        eprintln!("SGL: {text}");
        1
    }
}

/// Pantalla SGL con el dispositivo nativo ya iniciada a `width × height × 16` (`InitScreen` con el modo del juego).
pub fn init_screen(width: i32, height: i32, windowed: bool) -> Result<SglScreen<NativeScreen>, String> {
    let mut screen = SglScreen::new();
    let mut mode = p3_render::sgl_screen::ScreenMode { width, height, bpp: 16, refresh: 60 };
    match screen.init_screen(NativeScreen::default(), 1, i32::from(windowed), 0, Some(&mut mode)) {
        Ok(Some(_)) => Ok(screen),
        Ok(None) => Err("InitScreen: el dispositivo no se activó".into()),
        Err(_) => Err("InitScreen: fallo de la DLL".into()),
    }
}

/// Dispositivo activo.
pub fn active(screen: &mut SglScreen<NativeScreen>) -> Option<&mut NativeScreen> {
    let i = screen.active?;
    screen.devices.get_mut(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn swap_buffer_presents_back_buffer() {
        let mut screen = init_screen(4, 2, true).unwrap();
        let mut host = NativeHost::default();
        active(&mut screen).unwrap().back[0] = 7;
        screen.swap_buffer(&mut host, 1).unwrap();
        let d = active(&mut screen).unwrap();
        assert!(d.fresh);
        assert_eq!((d.front[0], d.back[0], d.size()), (7, 7, (4, 2)));
        assert_eq!(host.frames, 1);
        assert_eq!(screen.switch_screen_mode(8, 4, 16), Ok(1));
        assert_eq!(active(&mut screen).unwrap().size(), (8, 4));
    }
}
