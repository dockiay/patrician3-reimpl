//! Texturas de memoria del controlador SGL (`ddraw_Dll.dll`): ranuras `sgl_CreateMemoryTexture`, `sgl_LockTexture`,
//! `sgl_UnlockTexture`, `sgl_PrefetchTexture`, `sgl_ChangeTextureFlags`, `sgl_ChangePalette`, `sgl_SetColorKey`,
//! `sgl_GetColorKey`, `sgl_LoadMemoryTexture` y `sgl_ReLoadMemoryTexture` del EXE (estas dos con [`convert_raw`] y
//! [`TexStore::load_memory_texture`], probadas sobre los `.aim` reales con `oracle/x86_sgl_load.py`).
//!
//! Una textura de memoria es un objeto de 0x5C bytes (tabla `0x050510A0`, base `0x05012E60`): píxeles alineados a
//! 32 bytes (`+0x18`), paleta compartida del almacén `0x05080D48` (`+0x1C`, solo con el indicador `0x20`), ancho y alto
//! de 16 bits (`+0x20`/`+0x22`), bytes por píxel (`+0x24`: 1 con paleta, si no 4), paso alineado a 4 (`+0x28`), color
//! clave (`+8`), indicadores (`+0x34`, siempre con `0x40` = en memoria), bloqueos (`+0x38`), estado (`+0x3A`, bit 8 =
//! pendiente de preparar) y copia de la paleta que se entrega al bloquear (`+0x44`).
//!
//! Al preparar (`0x05013A00` → `0x05013250`, al primer bloqueo y al desbloquear tras escribir) una textura de 32 bits con
//! clave (`indicadores & 7 == 7`) deja alfa 0 en los píxeles del color clave y alfa 255 en el resto. Con paleta, la
//! clave se aplica a las entradas al crear y al cambiar la paleta (también en la tabla que pasa quien llama).
//!
//! Fuera del modelo: la contabilidad de memoria de vídeo (`0x0506E960`), la copia en la tarjeta (`+0x2C`), los mapas
//! MIP y la combinación secreta de teclas de `0x05010C40` que borra las texturas (aquí nunca pulsada).
//! Probado contra la DLL en emulación: `oracle/x86_sgl_tex.py`, `tests/sgl_tex.rs`.

/// Máscara de `sgl_ChangeTextureFlags` (`0x05013BA0`).
pub const CHANGEABLE_FLAGS: u32 = 0x0701_8197;
/// Textura con paleta de 8 bits.
pub const FLAG_PALETTE: u32 = 0x20;
/// Textura en memoria (la pone siempre `0x05013800`).
pub const FLAG_MEMORY: u32 = 0x40;

/// Datos que rellena `sgl_LockTexture` (`+8` ancho, `+0xC` alto, `+0x10` paso; los punteros de `+0` y `+4` son
/// [`MemTexture::pixels`] y [`MemTexture::lock_palette`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LockInfo {
    pub width: u32,
    pub height: u32,
    pub pitch: u32,
}

#[derive(Clone, Debug)]
pub struct MemTexture {
    pub width: u16,
    pub height: u16,
    pub bytes_per_pixel: u8,
    pub pitch: u32,
    /// `pitch * height` bytes (B, G, R, A en 32 bits; índices con paleta).
    pub pixels: Vec<u8>,
    /// Paleta vigente (`+0x1C`), entradas `0xAARRGGBB`.
    pub palette: Option<Vec<u32>>,
    /// Copia de la paleta hecha en el primer bloqueo (`+0x44`); no se actualiza después.
    pub lock_palette: Option<Vec<u32>>,
    pub key: u32,
    pub flags: u32,
    /// Filtro de imagen (`+0x3C`, `sgl_SetImageFilter` al crear).
    pub filter: u8,
    pub locks: u16,
    pub state: u8,
}

impl MemTexture {
    fn texel(&self, i: usize) -> u32 {
        u32::from_le_bytes(self.pixels[i..i + 4].try_into().unwrap())
    }

    /// `0x05013250`: color clave en las texturas de 32 bits.
    fn apply_key(&mut self) {
        if self.flags & 7 != 7 || self.palette.is_some() {
            return;
        }
        let key = self.key & 0xFF_FFFF;
        for y in 0..usize::from(self.height) {
            for x in 0..usize::from(self.width) {
                let i = y * self.pitch as usize + 4 * x;
                let p = self.texel(i) & 0xFF_FFFF;
                let v = if p == key { p } else { p | 0xFF00_0000 };
                self.pixels[i..i + 4].copy_from_slice(&v.to_le_bytes());
            }
        }
    }

    /// `0x05013A00`.
    fn prepare(&mut self) {
        self.apply_key();
        self.state &= !8;
    }

    fn keyed_palette(&self, pal: &mut [u32]) {
        if self.flags & 7 == 7 {
            let key = self.key & 0xFF_FFFF;
            for e in pal.iter_mut() {
                if *e & 0xFF_FFFF == key {
                    *e &= 0xFF_FFFF;
                }
            }
        }
    }

    /// Bit 0 sin contador de mapas en el byte alto: el controlador pone 1 (`0x050134E0`).
    fn fix_mip_byte(&mut self) {
        if self.flags >> 24 == 0 && self.flags & 1 != 0 {
            self.flags |= 1 << 24;
        }
    }

    /// Vista de 32 bits para [`crate::sgl::Texture`] (solo sin paleta).
    pub fn texels(&self) -> Vec<u32> {
        self.pixels.chunks_exact(4).map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect()
    }
}

pub type TexId = usize;

/// Estado de texturas del controlador.
#[derive(Clone, Debug)]
pub struct TexStore {
    /// `[0x05080ACC]`: clave que reciben las texturas nuevas (`sgl_SetColorKey`).
    pub color_key: u32,
    /// `[0x05080B6E]` (`sgl_SetImageFilter`).
    pub filter: u8,
    /// `[0x0505F73C]` (1 al cargar, configurable): si es 0, `+0xB0` quita el bit 8 de los indicadores.
    pub allow_flag8: bool,
    texs: Vec<Option<MemTexture>>,
}

impl Default for TexStore {
    fn default() -> Self {
        TexStore { color_key: 0, filter: 0, allow_flag8: true, texs: Vec::new() }
    }
}

impl TexStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// Bytes de píxeles que retienen todas las texturas vivas (diagnóstico de memoria).
    pub fn pixel_bytes(&self) -> usize {
        self.texs.iter().flatten().map(|t| t.pixels.capacity()).sum()
    }

    pub fn get(&self, id: TexId) -> Option<&MemTexture> {
        self.texs.get(id).and_then(|t| t.as_ref())
    }

    pub fn get_mut(&mut self, id: TexId) -> Option<&mut MemTexture> {
        self.texs.get_mut(id).and_then(|t| t.as_mut())
    }

    fn t(&mut self, id: TexId) -> &mut MemTexture {
        self.get_mut(id).expect("textura inexistente")
    }

    /// Clave que reciben las texturas nuevas (`[0x05080ACC]`).
    pub fn color_key(&self) -> u32 {
        self.color_key
    }

    /// `sgl_SetColorKey` (`0x0501CDC0`): devuelve la clave anterior.
    pub fn set_color_key(&mut self, key: u32) -> u32 {
        std::mem::replace(&mut self.color_key, key & 0xFF_FFFF)
    }

    /// `sgl_GetColorKey` (`0x0501CDE0`).
    pub fn get_color_key(&self, id: TexId) -> u32 {
        let t = self.get(id).expect("textura inexistente");
        if t.flags & 7 != 0 {
            t.key
        } else {
            0
        }
    }

    /// `sgl_CreateMemoryTexture` (`0x0501D220` → `0x05013800`): píxeles a cero; con `0x20`, 8 bits y paleta gris. El
    /// controlador lee la clave de un objeto recién reservado al preparar la paleta; aquí ese valor es 0.
    pub fn create_memory_texture(&mut self, width: u32, height: u32, flags: u32) -> TexId {
        let (w, h) = (width as u16, height as u16);
        let mut t = MemTexture {
            width: w,
            height: h,
            bytes_per_pixel: 4,
            pitch: 0,
            pixels: Vec::new(),
            palette: None,
            lock_palette: None,
            key: 0,
            flags: flags | FLAG_MEMORY,
            filter: self.filter,
            locks: 0,
            state: 0,
        };
        t.fix_mip_byte();
        let mut ramp = None;
        if flags & FLAG_PALETTE != 0 {
            t.bytes_per_pixel = 1;
            ramp = Some((0..256u32).map(|i| 0xFF00_0000 | i * 0x01_0101).collect::<Vec<u32>>());
        }
        t.pitch = (u32::from(t.bytes_per_pixel) * u32::from(w) + 3) & !3;
        t.pixels = vec![0; (t.pitch * u32::from(h)) as usize];
        if let Some(mut pal) = ramp {
            t.keyed_palette(&mut pal);
            t.palette = Some(pal);
        }
        t.state |= 8;
        t.key = self.color_key;
        self.texs.push(Some(t));
        self.texs.len() - 1
    }

    /// Inserta una textura ya construida.
    pub fn insert(&mut self, t: MemTexture) -> TexId {
        self.texs.push(Some(t));
        self.texs.len() - 1
    }

    /// `sgl_FreeMemoryTexture` / `sgl_FreeTexture`.
    pub fn free(&mut self, id: TexId) {
        if let Some(t) = self.texs.get_mut(id) {
            *t = None;
        }
    }

    /// `sgl_LockTexture` (`0x0501D630` → `0x05013DD0`). `flags` se acumula en el estado (bit 2: se escribirá; al
    /// desbloquear se prepara de nuevo). `None` si la textura no tiene tamaño.
    pub fn lock(&mut self, id: TexId, flags: u32) -> Option<LockInfo> {
        let t = self.t(id);
        if t.locks == 0 {
            if t.width == 0 || t.height == 0 || t.bytes_per_pixel == 0 {
                return None;
            }
            t.state |= flags as u8;
            if t.state & 8 != 0 {
                t.prepare();
            }
            if t.state & 2 != 0 {
                t.flags |= FLAG_MEMORY;
            }
        } else {
            t.state |= flags as u8;
        }
        if t.lock_palette.is_none() && t.flags & FLAG_PALETTE != 0 {
            t.lock_palette = Some(t.palette.clone().unwrap_or_else(|| vec![0; 256]));
        }
        t.locks = t.locks.wrapping_add(1);
        Some(LockInfo { width: u32::from(t.width), height: u32::from(t.height), pitch: t.pitch })
    }

    /// `sgl_UnlockTexture` (`0x0501D650` → `0x05013F70`).
    pub fn unlock(&mut self, id: TexId) {
        let t = self.t(id);
        if (t.locks as i16) <= 0 {
            return;
        }
        t.locks -= 1;
        if t.locks == 0 {
            if t.state & 2 != 0 {
                t.prepare();
            }
            t.state &= 8;
        }
    }

    /// `sgl_PrefetchTexture`: sin efecto en memoria (`0x05004030`).
    pub fn prefetch(&mut self, _id: TexId) {}

    /// `sgl_ChangeTextureFlags` (`0x0501D600` → `0x05013BA0`): devuelve los bits cambiables anteriores.
    pub fn change_flags(&mut self, id: TexId, flags: u32) -> u32 {
        let t = self.t(id);
        let old = t.flags;
        t.flags = old ^ ((old ^ flags) & CHANGEABLE_FLAGS);
        t.fix_mip_byte();
        old & CHANGEABLE_FLAGS
    }

    /// Método `+0x2C` (`0x05013BE0`) que llama `sgl_ChangePalette` (que siempre devuelve 0). Aplica la clave también en
    /// `pal`, la tabla de quien llama. `false` si la textura no tiene paleta.
    pub fn change_palette(&mut self, id: TexId, pal: &mut [u32; 256]) -> bool {
        let t = self.t(id);
        if t.state & 8 != 0 {
            t.prepare();
        }
        if t.palette.is_none() {
            return false;
        }
        t.keyed_palette(pal);
        t.palette = Some(pal.to_vec());
        true
    }
}

/// Destino de `AIM_CONVERT_RAW` (`AIM.dll` `0x100029B0`) tal como lo prepara `0x05013500`: formato 1 (8 bits con paleta)
/// si la imagen tiene paleta, si no formato 4 (B, G, R, A); paso `(ancho * bpp + 3) & !3`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RawImage {
    pub width: u16,
    pub height: u16,
    pub bytes_per_pixel: u8,
    pub pitch: u32,
    pub pixels: Vec<u8>,
    /// 256 entradas `0xAARRGGBB` (solo formato 1).
    pub palette: Option<Vec<u32>>,
}

/// `AIM_CONVERT_RAW` de la salida de `AIM_CONVERT_MEMFILE` ([`p3_assets::aim::decode`]). Formatos de origen: 1 (índices,
/// se copian con la paleta), 4 (32 bits, se copia), 6 (24 bits: B, G, R, 0xFF) y 0xF (bloques de 4x4 con alfa explícito
/// de 4 bits, `0x1000A5C0`). Los formatos 0xD y 0x11 no aparecen en el corpus y no se modelan (`None`).
///
/// Bloques 0xF: 8 bytes de alfa (4 bits por píxel, `a * 17`) y 8 de color (`c0`, `c1` en 5:6:5 ampliados solo por
/// desplazamiento, `r << 3`, `g << 2`, `b << 3`, e índices de 2 bits). Si `c0 > c1`, los intermedios son
/// `(2*e0 + e1) / 3` y `(e0 + 2*e1) / 3` truncados (tabla `0x1001B400`); si no, `(e0 + e1) >> 1` y el color 3 es
/// B=0, G=0xFF, R=0xFF. El alfa del bloque sustituye siempre al del color.
pub fn convert_raw(img: &p3_assets::aim::Image) -> Option<RawImage> {
    let (w, h) = (img.width as usize, img.height as usize);
    let pal = img.format == 1;
    let bpp: usize = if pal { 1 } else { 4 };
    let pitch = (w * bpp + 3) & !3;
    let mut out = vec![0u8; pitch * h];
    let sp = img.pitch as usize;
    match img.format {
        1 | 4 | 6 => {
            let sb = if img.format == 6 { 3 } else { bpp };
            for y in 0..h {
                let src = img.pixels.get(y * sp..y * sp + w * sb)?;
                let row = &mut out[y * pitch..y * pitch + w * bpp];
                match img.format {
                    6 => {
                        for (d, s) in row.chunks_exact_mut(4).zip(src.chunks_exact(3)) {
                            d.copy_from_slice(&[s[0], s[1], s[2], 0xFF]);
                        }
                    }
                    _ => row.copy_from_slice(src),
                }
            }
        }
        0xF => {
            let e = |c: u16| [((c & 0x1F) << 3) as u32, (((c >> 5) & 0x3F) << 2) as u32, ((c >> 11) << 3) as u32];
            let (bw, bh) = (w / 4, h / 4);
            for by in 0..bh {
                for bx in 0..bw {
                    let blk = img.pixels.get((by * bw + bx) * 16..(by * bw + bx) * 16 + 16)?;
                    let c0 = u16::from_le_bytes([blk[8], blk[9]]);
                    let c1 = u16::from_le_bytes([blk[10], blk[11]]);
                    let bits = u32::from_le_bytes([blk[12], blk[13], blk[14], blk[15]]);
                    let (e0, e1) = (e(c0), e(c1));
                    let mut cols = [[0u32; 3]; 4];
                    cols[0] = e0;
                    cols[1] = e1;
                    for k in 0..3 {
                        if c0 > c1 {
                            cols[2][k] = (2 * e0[k] + e1[k]) / 3;
                            cols[3][k] = (e0[k] + 2 * e1[k]) / 3;
                        } else {
                            cols[2][k] = (e0[k] + e1[k]) >> 1;
                        }
                    }
                    if c0 <= c1 {
                        cols[3] = [0, 0xFF, 0xFF];
                    }
                    for i in 0..16 {
                        let c = cols[(bits >> (2 * i) & 3) as usize];
                        let a = (blk[i / 2] >> (4 * (i & 1)) & 15) * 17;
                        let o = (by * 4 + i / 4) * pitch + (bx * 4 + i % 4) * 4;
                        out[o..o + 4].copy_from_slice(&[c[0] as u8, c[1] as u8, c[2] as u8, a]);
                    }
                }
            }
        }
        _ => return None,
    }
    let palette = if pal {
        let src = img.palette.as_ref()?;
        let mut p: Vec<u32> = src.chunks_exact(4).take(256).map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect();
        p.resize(256, 0);
        Some(p)
    } else {
        None
    };
    Some(RawImage { width: w as u16, height: h as u16, bytes_per_pixel: bpp as u8, pitch: pitch as u32, pixels: out, palette })
}

impl TexStore {
    /// `0x05013500` (método `+0xB0`) con una imagen descodificada: lo que hacen `sgl_LoadMemoryTexture` (`0x0501CE00`,
    /// textura nueva de la clase base `0x05012E60`, sin `0x40`) y `sgl_ReLoadMemoryTexture` (`0x0501CF10`, misma textura).
    fn set_image(&mut self, t: &mut MemTexture, raw: RawImage, flags: u32) {
        let mut flags = flags;
        if !self.allow_flag8 {
            flags &= !8;
        }
        t.filter = self.filter;
        t.flags = flags;
        t.fix_mip_byte();
        let old_palette = t.palette.take();
        if raw.palette.is_some() {
            t.flags |= FLAG_PALETTE;
        }
        t.width = raw.width;
        t.height = raw.height;
        t.bytes_per_pixel = raw.bytes_per_pixel;
        t.pitch = raw.pitch;
        t.pixels = raw.pixels;
        if flags & 0x2_0000 == 0 {
            t.key = self.color_key;
        }
        if let Some(mut pal) = raw.palette {
            // Con una paleta anterior la DLL conserva su puntero (y su contenido) en vez de registrar la nueva.
            t.palette = Some(match old_palette {
                Some(old) => old,
                None => {
                    t.keyed_palette(&mut pal);
                    pal
                }
            });
        }
        t.apply_key();
    }

    /// `sgl_LoadMemoryTexture (nombre, indicadores)` con la imagen ya leída del archivo y descodificada. La DLL aplaza la
    /// descodificación hasta el primer uso si los indicadores no llevan `0x40` ni `0x10000` (entonces lo hace `0x05013430`
    /// con `indicadores | 0x40`); el resultado es el mismo. `None` si el formato no se modela.
    pub fn load_memory_texture(&mut self, img: &p3_assets::aim::Image, flags: u32) -> Option<TexId> {
        let raw = convert_raw(img)?;
        let mut t = MemTexture {
            width: 0,
            height: 0,
            bytes_per_pixel: 0,
            pitch: 0,
            pixels: Vec::new(),
            palette: None,
            lock_palette: None,
            key: 0,
            flags: 0,
            filter: 0,
            locks: 0,
            state: 0,
        };
        self.set_image(&mut t, raw, flags);
        Some(self.insert(t))
    }

    /// `sgl_ReLoadMemoryTexture (textura, nombre)`: `+0xB0` con `indicadores | 0x10000` y preparación (`0x05013A00`).
    pub fn reload_memory_texture(&mut self, id: TexId, img: &p3_assets::aim::Image) -> Option<()> {
        let raw = convert_raw(img)?;
        let mut t = self.get_mut(id)?.clone();
        let flags = t.flags | 0x1_0000;
        self.set_image(&mut t, raw, flags);
        t.prepare();
        *self.get_mut(id)? = t;
        Some(())
    }
}
