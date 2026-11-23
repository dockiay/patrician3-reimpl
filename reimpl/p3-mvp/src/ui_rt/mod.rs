//! Enlazador de la interfaz portada: ejecuta las funciones de interfaz transcritas (`p3_render::j07`/`j08`, `p3_render::panels`
//! y `p3_core::lifted`) sobre una memoria con la imagen del EXE original, llamándose entre sí por dirección como hace el x86.
//!
//! Lo que no es código del juego se sustituye aquí:
//! - **CRT** estático del EXE (`memcpy`, `strlen`, `atoi`, `new`...): rutinas nativas.
//! - **SGL** (`ddraw_Dll.dll`): las ranuras `0x006DA8xx..0x006DAA9x` que rellena `sgl_load_driver_dll` apuntan a direcciones
//!   ficticias `SGL_BASE + ranura`; cada una se ejecuta sobre [`Surface`] con las rutinas de `p3_render::sgl` y `sgl_font`.
//! - Estado que el arranque del original crea (fuentes `0x006DCD28 + 0xA8·N`, gestor de texturas `0x006DA820`, iconos de
//!   `0x0042EDB7`): se siembra con los mismos datos del CPR (`fonts.ini`, `textures.ini`).
//!
//! Las funciones que faltan se anotan en [`Runtime::missing`] y devuelven 0: así se ve qué impide dibujar una ventana entera.
//!
//! Convención de pila: cada llamada recibe un marco nuevo en la pila propia (`[esp]` = retorno ficticio, `[esp+4..]` = argumentos).
//! Las funciones de `p3_core::lifted` llaman con `args = []` cuando el elevador no dedujo cuántos hay; entonces empujaron ya el
//! retorno en su pila, y la última escritura de 32 bits es ese hueco: el llamado usa esa misma pila (como el x86).

mod clock;
pub mod stall;
pub mod profile;
pub mod core_table;
pub mod hand_table;
pub mod crt;
pub mod fix_lifted;
mod ship_lifted;
mod trade_lifted;
pub mod ship_panel_lifted;
pub mod names_lifted;
pub mod newgame_lifted;
pub mod script_lifted;
pub mod ship_orders_lifted;
pub mod church_lifted;
pub mod grp_cy2_lifted;
pub mod grp_bat2_lifted;
pub mod grp_bat2_native;
pub mod grp_lib2_lifted;
pub mod grp_lib2_native;
pub mod grp_snd_lifted;
pub mod grp_rain_lifted;
pub mod grp_sie_lifted;
pub mod grp_lib_lifted;
pub mod grp_lib_native;
pub mod grp_sea_lifted;
pub mod grp_nav_lifted;
pub mod grp_tat_lifted;
pub mod grp_sail_lifted;
pub mod grp_bld_lifted;
pub mod grp_ui_lifted;
pub mod grp_ops_lifted;
pub mod grp_bat_lifted;
pub mod grp_tk_lifted;
pub mod grp_cy_lifted;
pub mod ayto_lifted;
pub mod hanse_lifted;
pub mod lender_lifted;
mod barn_lifted;
pub mod office_lifted;
mod tavern_lifted;
pub mod lib_lifted;
pub mod win32;
pub mod game_window;
pub mod menus;
pub mod display;
pub mod language;
pub mod video_options;
mod blit;
/// Autocomprobación de la ruta vectorial de `blit` (ver `blit::self_check`).
pub fn blit_self_check() -> Result<u32, String> {
    blit::self_check()
}
mod lines;
pub mod zoom;
pub mod miles;
pub mod vto;
pub mod display_lifted;
pub mod shipyard_lifted;
pub mod activities_lifted;

use crate::navmap::{decode, load_fonts, parse_textures, Image, TexDef};
use p3_assets::cpr::Archive;
use p3_render::j07::{Host, Ret};
use std::collections::{BTreeMap, HashMap};

const PAGE: u32 = 0x1000;
/// Constructores de objetos globales que se ejecutan al arrancar (lista cerrada: ejecutar todos los del CRT activa, p. ej., la
/// caché INI y cambia la carga de fuentes).
const PORTED_CTORS: [u32; 1] = [0x004B_8740];
/// Pila propia del enlazador (crece hacia abajo).
pub const STACK_TOP: u32 = 0x0FF0_0000;
/// Montón (`new`/`malloc`), por encima de todo lo del EXE.
const HEAP_BASE: u32 = 0x2000_0000;
/// Tamaño desde el que un bloque liberado devuelve sus páginas con [`Runtime::set_low_memory`].
const TRIM_MIN: u32 = 0x4000;
/// Espejo de la pantalla que entrega `sgl_LockTexture(-4)`; libre entre el montón (`0x2000_0000`…) y los manejadores.
const SCREEN_LOCK_BASE: u32 = 0x4000_0000;
// `0x59E8E0` desplaza la lectura 2*(delta/4) en X y delta/2 en Y. Delta es la diferencia de dos u8:
// hasta 126/127 píxeles, más uno de guarda. Un margen de 64 deja muestras fuera del espejo actualizado.
const SCREEN_LOCK_MARGIN: i32 = 128;
/// Direcciones ficticias de las rutinas de la DLL SGL: `SGL_BASE + ranura`.
const SGL_BASE: u32 = 0xE000_0000;
/// Retorno ficticio que se deja en `[esp]` de cada marco nuevo.
const RET_MAGIC: u32 = 0xDEAD_0000;
/// Direcciones ficticias de las funciones importadas (IAT): `IMPORT_BASE + entrada`.
const IMPORT_BASE: u32 = 0xD000_0000;

/// Dispersión barata para tablas con clave `u32` (direcciones y manejadores): el enlazador consulta varias en cada llamada del
/// original y SipHash, el de `HashMap` por defecto, pesaba un 4-5 % de la carga. Ninguna tabla se recorre en un orden del que dependa
/// el resultado (con el `RandomState` de antes ese orden ya cambiaba en cada ejecución).
#[derive(Default, Clone, Copy)]
pub struct U32Hasher(u64);
impl std::hash::Hasher for U32Hasher {
    fn finish(&self) -> u64 {
        self.0
    }
    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.0 = (self.0 ^ u64::from(b)).wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    fn write_u32(&mut self, v: u32) {
        let x = (u64::from(v) ^ self.0).wrapping_mul(0x9E37_79B9_7F4A_7C15);
        self.0 = x ^ (x >> 32);
    }
}
pub type U32Map<V> = HashMap<u32, V, std::hash::BuildHasherDefault<U32Hasher>>;

/// Memoria por páginas de 4 KiB; leer donde nada se escribió da 0.
pub struct PageMem {
    // Every u32 address fits this 8 MiB directory on a 64-bit host. Untouched
    // pages remain None: guest memory stays sparse, without hashing each load.
    pages: Vec<Option<Box<[u8; PAGE as usize]>>>,
}

impl Default for PageMem {
    fn default() -> Self { Self { pages: vec![None; 1 << 20] } }
}

impl PageMem {
    /// Ranura de la página de `a`. El directorio tiene siempre `2^20` entradas (una por página de 4 KiB del espacio de 32 bits) y
    /// no cambia de tamaño, así que el índice `a / PAGE` nunca se sale.
    #[inline(always)]
    fn slot(&self, a: u32) -> &Option<Box<[u8; PAGE as usize]>> {
        debug_assert_eq!(self.pages.len(), 1 << 20);
        // SAFETY: `a / PAGE < 2^20 == self.pages.len()` (ver arriba; `Default` es el único constructor).
        unsafe { self.pages.get_unchecked((a / PAGE) as usize) }
    }
    #[inline(always)]
    fn page(&mut self, a: u32) -> &mut [u8; PAGE as usize] {
        debug_assert_eq!(self.pages.len(), 1 << 20);
        // SAFETY: como en `slot`.
        let slot = unsafe { self.pages.get_unchecked_mut((a / PAGE) as usize) };
        match slot {
            Some(p) => p,
            None => Self::new_page(slot),
        }
    }
    /// Páginas escritas alguna vez, por dirección (diagnóstico: volcados de memoria para cotejar con el x86).
    pub fn mapped_pages(&self) -> impl Iterator<Item = (u32, &[u8])> {
        self.pages.iter().enumerate().filter_map(|(i, p)| p.as_ref().map(|p| (i as u32 * PAGE, &p[..])))
    }
    #[cold]
    #[inline(never)]
    fn new_page(slot: &mut Option<Box<[u8; PAGE as usize]>>) -> &mut [u8; PAGE as usize] {
        slot.insert(Box::new([0; PAGE as usize]))
    }
    /// Suelta las páginas que caen enteras dentro de `[a, a + len)`; leerlas después da 0, como si nunca se hubieran escrito.
    pub fn release_pages(&mut self, a: u32, len: u32) {
        let first = a.div_ceil(PAGE);
        let end = (u64::from(a) + u64::from(len)) / u64::from(PAGE);
        for i in u64::from(first)..end.min(1 << 20) {
            self.pages[i as usize] = None;
        }
    }
    /// Páginas reservadas (4 KiB cada una).
    pub fn page_count(&self) -> usize {
        self.pages.iter().filter(|p| p.is_some()).count()
    }
    #[inline(always)]
    pub fn read_byte(&self, a: u32) -> u8 {
        self.slot(a).as_ref().map_or(0, |p| p[(a % PAGE) as usize])
    }
    pub fn write(&mut self, a: u32, data: &[u8]) {
        let mut at = a;
        let mut rest = data;
        while !rest.is_empty() {
            let offset = (at % PAGE) as usize;
            let n = rest.len().min(PAGE as usize - offset);
            self.page(at)[offset..offset + n].copy_from_slice(&rest[..n]);
            rest = &rest[n..];
            at = at.wrapping_add(n as u32);
        }
    }
    /// Escribe una palabra: una sola búsqueda de página si no cruza el borde (lo habitual); si lo cruza, como [`Self::write`].
    #[inline(always)]
    fn put<const N: usize>(&mut self, a: u32, v: [u8; N]) {
        let offset = (a % PAGE) as usize;
        if offset <= PAGE as usize - N {
            self.page(a)[offset..offset + N].copy_from_slice(&v);
        } else {
            self.write(a, &v);
        }
    }
    pub fn read(&self, a: u32, n: usize) -> Vec<u8> {
        let mut out = vec![0; n];
        self.read_into(a,&mut out);
        out
    }
    fn read_into(&self, a: u32, out: &mut [u8]) {
        let mut at = a;
        let mut rest = out;
        while !rest.is_empty() {
            let offset = (at % PAGE) as usize;
            let n = rest.len().min(PAGE as usize - offset);
            if let Some(page) = self.pages[(at / PAGE) as usize].as_ref() {
                rest[..n].copy_from_slice(&page[offset..offset + n]);
            } else {
                rest[..n].fill(0);
            }
            rest = &mut rest[n..];
            at = at.wrapping_add(n as u32);
        }
    }
    #[inline(always)]
    fn word<const N: usize>(&self, a: u32) -> [u8; N] {
        let offset = (a % PAGE) as usize;
        if offset <= PAGE as usize - N {
            return self.slot(a).as_ref().map_or([0; N], |p| p[offset..offset + N].try_into().unwrap());
        }
        std::array::from_fn(|i| self.read_byte(a.wrapping_add(i as u32)))
    }
    pub fn cstr(&self, a: u32) -> Vec<u8> {
        let mut out = Vec::new();
        let mut at = a;
        while out.len() < 0x10000 {
            let b = self.read_byte(at);
            if b == 0 {
                break;
            }
            out.push(b);
            at = at.wrapping_add(1);
        }
        out
    }
    #[inline(always)]
    pub fn u8_at(&self, a: u32) -> u8 {
        self.read_byte(a)
    }
    /// Compara dos cadenas C sin copiarlas (ver `Mem::cmp_cstr`).
    pub fn cstr_cmp(&self, a: u32, b: u32, fold: bool, max: usize) -> std::cmp::Ordering {
        static ZERO: [u8; PAGE as usize] = [0; PAGE as usize];
        let mut done = 0usize;
        while done < max {
            // Tramo que cabe en la página de cada cadena: se compara por palabras de 8 bytes mientras sean idénticas y sin
            // terminador (los nombres de textura comparten prefijos largos), y byte a byte desde la primera diferencia.
            let (pa, pb) = (a.wrapping_add(done as u32), b.wrapping_add(done as u32));
            let (oa, ob) = ((pa % PAGE) as usize, (pb % PAGE) as usize);
            let n = (PAGE as usize - oa).min(PAGE as usize - ob).min(max - done);
            let xa = self.pages[(pa / PAGE) as usize].as_ref().map_or(&ZERO[..n], |p| &p[oa..oa + n]);
            let xb = self.pages[(pb / PAGE) as usize].as_ref().map_or(&ZERO[..n], |p| &p[ob..ob + n]);
            let mut k = 0;
            while k + 8 <= n {
                let x = u64::from_le_bytes(xa[k..k + 8].try_into().unwrap());
                let y = u64::from_le_bytes(xb[k..k + 8].try_into().unwrap());
                if x != y || x.wrapping_sub(0x0101_0101_0101_0101) & !x & 0x8080_8080_8080_8080 != 0 {
                    break;
                }
                k += 8;
            }
            while k < n {
                let (mut p, mut q) = (xa[k], xb[k]);
                if p == 0 || q == 0 {
                    return (p != 0).cmp(&(q != 0));
                }
                if fold {
                    p = p.to_ascii_lowercase();
                    q = q.to_ascii_lowercase();
                }
                if p != q {
                    return p.cmp(&q);
                }
                k += 1;
            }
            done += n;
        }
        std::cmp::Ordering::Equal
    }
    /// FNV-1a de toda la memoria escrita (dirección de página y contenido; las páginas a cero cuentan como no escritas). Sirve para
    /// comprobar que una optimización deja el mismo estado del juego, no solo la misma imagen.
    pub fn content_hash(&self) -> u64 {
        self.content_hash_outside(0..0)
    }
    /// Como [`Self::content_hash`] sin las páginas de `skip` (direcciones): por ejemplo la pila del enlazador, donde quedan
    /// marcos muertos que no son estado del juego.
    pub fn content_hash_outside(&self, skip: std::ops::Range<u32>) -> u64 {
        let mut h = 0xcbf2_9ce4_8422_2325u64;
        for (i, p) in self.pages.iter().enumerate() {
            let Some(p) = p else { continue };
            if skip.contains(&(i as u32 * PAGE)) {
                continue;
            }
            if p.iter().all(|&b| b == 0) {
                continue;
            }
            for b in (i as u32).to_le_bytes().iter().chain(p.iter()) {
                h = (h ^ u64::from(*b)).wrapping_mul(0x0000_0100_0000_01b3);
            }
        }
        h
    }
    #[inline(always)]
    pub fn u32_at(&self, a: u32) -> u32 {
        u32::from_le_bytes(self.word(a))
    }
    pub fn set_u32(&mut self, a: u32, v: u32) {
        self.write(a, &v.to_le_bytes());
    }

    /// Proyecta las secciones del PE en su dirección virtual (`ImageBase + VirtualAddress`).
    pub fn load_pe(&mut self, exe: &[u8]) -> Result<u32, String> {
        let u16_at = |o: usize| exe.get(o..o + 2).map(|b| u16::from_le_bytes([b[0], b[1]])).ok_or("PE truncado");
        let u32_at = |o: usize| exe.get(o..o + 4).map(|b| u32::from_le_bytes(b.try_into().unwrap())).ok_or("PE truncado");
        if exe.get(..2) != Some(b"MZ") {
            return Err("el ejecutable no es un PE".into());
        }
        let pe = u32_at(0x3c)? as usize;
        if exe.get(pe..pe + 4) != Some(b"PE\0\0") {
            return Err("el ejecutable no es un PE".into());
        }
        let sections = usize::from(u16_at(pe + 6)?);
        let opt = pe + 24;
        let opt_size = usize::from(u16_at(pe + 20)?);
        let base = u32_at(opt + 28)?;
        for s in 0..sections {
            let h = opt + opt_size + 40 * s;
            let (va, raw_size, raw_ptr) = (u32_at(h + 12)?, u32_at(h + 16)? as usize, u32_at(h + 20)? as usize);
            if let Some(data) = exe.get(raw_ptr..raw_ptr + raw_size) {
                self.write(base + va, data);
            }
        }
        Ok(base)
    }

    /// Importaciones del PE: `(entrada de la IAT, "dll!nombre")`.
    pub fn pe_imports(exe: &[u8]) -> Vec<(u32, String)> {
        let u16_at = |o: usize| exe.get(o..o + 2).map_or(0, |b| u16::from_le_bytes([b[0], b[1]]));
        let u32_at = |o: usize| exe.get(o..o + 4).map_or(0, |b| u32::from_le_bytes(b.try_into().unwrap()));
        let pe = u32_at(0x3c) as usize;
        let opt = pe + 24;
        let opt_size = usize::from(u16_at(pe + 20));
        let base = u32_at(opt + 28);
        let sections: Vec<(u32, u32, u32)> = (0..usize::from(u16_at(pe + 6)))
            .map(|s| {
                let h = opt + opt_size + 40 * s;
                (u32_at(h + 12), u32_at(h + 8).max(u32_at(h + 16)), u32_at(h + 20))
            })
            .collect();
        let off = |rva: u32| sections.iter().find(|(va, size, _)| (*va..va + size).contains(&rva)).map(|(va, _, raw)| (rva - va + raw) as usize);
        let cstr = |o: usize| exe[o..].iter().take_while(|&&b| b != 0).map(|&b| char::from(b)).collect::<String>();
        let mut out = Vec::new();
        let Some(mut d) = off(u32_at(opt + 104)) else { return out };
        loop {
            let (lookup, name, first) = (u32_at(d), u32_at(d + 12), u32_at(d + 16));
            if name == 0 {
                break;
            }
            let dll = off(name).map(cstr).unwrap_or_default();
            let mut i = 0;
            if let Some(t) = off(if lookup != 0 { lookup } else { first }) {
                loop {
                    let e = u32_at(t + 4 * i);
                    if e == 0 {
                        break;
                    }
                    let func = if e & 0x8000_0000 != 0 { format!("#{}", e & 0xffff) } else { off(e + 2).map(cstr).unwrap_or_default() };
                    out.push((base + first + 4 * i as u32, format!("{dll}!{func}")));
                    i += 1;
                }
            }
            d += 20;
        }
        out
    }
}

impl p3_render::scene_math::Mem for PageMem {
    fn release(&mut self, a: u32, len: u32) { self.release_pages(a, len); }
    fn core_view(&mut self) -> Option<&mut dyn p3_core::flatmem::Mem> { Some(self) }
    fn read_bytes(&mut self, a:u32, out:&mut [u8]) { self.read_into(a,out); }
    fn write_bytes(&mut self, a:u32, data:&[u8]) { self.write(a,data); }
    fn cmp_cstr(&mut self, a: u32, b: u32, fold: bool, max: usize) -> Option<std::cmp::Ordering> { Some(self.cstr_cmp(a, b, fold, max)) }
    fn r8(&mut self, a: u32) -> u8 {
        self.read_byte(a)
    }
    fn r16(&mut self, a: u32) -> u16 { u16::from_le_bytes(self.word(a)) }
    fn r32(&mut self, a: u32) -> u32 { self.u32_at(a) }
    fn w8(&mut self, a: u32, v: u8) {
        self.page(a)[(a % PAGE) as usize] = v;
    }
    fn w16(&mut self, a: u32, v: u16) { self.put(a, v.to_le_bytes()); }
    fn w32(&mut self, a: u32, v: u32) {
        LAST.with(|l| l.set(a));
        self.put(a, v.to_le_bytes());
    }
}

impl p3_core::flatmem::Mem for PageMem {
    fn release(&mut self, a: u32, len: u32) { self.release_pages(a, len); }
    fn as_any_mut(&mut self) -> Option<&mut dyn std::any::Any> { Some(self) }
    fn read_bytes(&mut self, a:u32, out:&mut [u8]) { self.read_into(a,out); }
    fn write_bytes(&mut self, a:u32, data:&[u8]) { self.write(a,data); }
    fn cmp_cstr(&mut self, a: u32, b: u32, fold: bool, max: usize) -> Option<std::cmp::Ordering> { Some(self.cstr_cmp(a, b, fold, max)) }
    fn r8(&mut self, a: u32) -> u8 {
        self.read_byte(a)
    }
    fn r16(&mut self, a: u32) -> u16 { u16::from_le_bytes(self.word(a)) }
    fn r32(&mut self, a: u32) -> u32 { self.u32_at(a) }
    fn w8(&mut self, a: u32, v: u8) {
        self.page(a)[(a % PAGE) as usize] = v;
    }
    fn w16(&mut self, a: u32, v: u16) { self.put(a, v.to_le_bytes()); }
    fn w32(&mut self, a: u32, v: u32) {
        LAST.with(|l| l.set(a));
        self.put(a, v.to_le_bytes());
    }
}

/// Recursos del CPR que las rutinas SGL necesitan: texturas de `textures.ini` (cargadas al usarlas) y fuentes de `fonts.ini`.
pub struct Assets<'a> {
    pub(crate) archive: &'a Archive<'a>,
    pub defs: U32Map<TexDef>,
    pub images: U32Map<Image>,
    pub fonts: U32Map<p3_render::sgl_font::Font>,
    /// Texturas de `sgl_LoadMemoryTexture` aún sin descodificar (ver [`Assets::materialize`]): el manejador ya existe, la imagen
    /// se obtiene del CPR la primera vez que alguien la pide.
    pending: U32Map<PendingImage>,
    /// Búsquedas ya hechas en el índice del CPR (nombre con `/` → entrada): `Archive::find` recorre el índice entero y cargar
    /// una partida pide miles de archivos. El archivo no cambia en marcha.
    found: std::cell::RefCell<HashMap<Vec<u8>, Option<&'a p3_assets::cpr::Entry>>>,
    /// Imágenes de `images` que se pueden soltar y volver a descodificar del CPR tal cual (WEB-6, memoria): manejador →
    /// (receta, último uso, bytes RGBA). Solo se lleva la cuenta con un tope puesto ([`Assets::set_image_budget`]).
    live: U32Map<(PendingImage, u64, usize)>,
    live_bytes: usize,
    clock: u64,
    image_budget: Option<usize>,
    image_budget_max: usize,
    thrash: u64,
    /// Texturas que hubo que descodificar otra vez por haberse soltado (diagnóstico: si crece en cada fotograma, el tope es pequeño).
    pub redecodes: u64,
    /// Modo de mezcla propio (byte `+0x37`) de las texturas de `sgl_CreateMemoryTexture`: el controlador lo toma de los
    /// indicadores (`>> 24`) o pone 1 si llevan el bit 0 (`0x05013896`); sin ninguno queda 0, copia directa. `DrawBitmapRect`
    /// y `StretchBitmapRect` leen el modo de la textura ORIGEN (`0x050147BA`), no el de `SetBlendMode` (15/11).
    pub own_mode: U32Map<u8>,
}

/// Textura cargada de archivo cuya descodificación se aplaza hasta el primer uso.
struct PendingImage {
    /// Nombre tal como lo pidió el juego (bytes) y su versión de texto, la que mira [`decode`] para elegir formato.
    raw: Vec<u8>,
    file: String,
    /// Clave de color (r, g, b) que `sgl_LoadMemoryTexture` aplicaba al cargar: esos píxeles quedan transparentes.
    key: Option<(u8, u8, u8)>,
    /// Ya estuvo descodificada y se soltó por el tope de memoria (ver [`Assets::set_image_budget`]): valor del reloj de usos
    /// en ese momento.
    evicted: Option<u64>,
}

/// ¿Se puede aplazar la descodificación de este archivo sin cambiar si la carga tiene éxito? Solo cuando la cabecera basta para
/// saber que [`decode`] no falla: TGA (las mismas comprobaciones que `p3_render::decode_tga`, que después no puede fallar) y AIM
/// de los tipos comprimidos del juego, cuyo cuerpo no se valida aquí (`examples/ui_texture_probe.rs` comprueba sobre el CPR entero
/// que todo archivo aceptado se descodifica). BMP y JPG se descodifican al cargar como antes (fallan según su contenido).
/// `P3_EAGER_TEXTURES` desactiva el aplazamiento (diagnóstico).
pub fn lazy_decodable(data: &[u8], name: &str) -> bool {
    static EAGER: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    if *EAGER.get_or_init(|| std::env::var_os("P3_EAGER_TEXTURES").is_some()) {
        return false;
    }
    let lower = name.to_ascii_lowercase();
    if lower.ends_with(".tga") {
        if data.len() < 18 || data[1] != 0 || data[2] != 2 {
            return false;
        }
        let (id_len, width, height, bpp) = (usize::from(data[0]), usize::from(u16::from_le_bytes([data[12], data[13]])), usize::from(u16::from_le_bytes([data[14], data[15]])), usize::from(data[16]));
        (bpp == 24 || bpp == 32) && width != 0 && height != 0 && data.len() >= 18 + id_len + width * height * (bpp / 8)
    } else if lower.ends_with(".bmp") || lower.ends_with(".jpg") {
        false
    } else {
        data.get(..3) == Some(b"AIM") && matches!(data.get(p3_assets::aim::SUBLAYER_OFFSET), Some(0x21 | 0x22))
    }
}

/// Imagen de `AIM_CONVERT_MEMFILE` + `AIM_CONVERT_RAW` (AIM.dll) para un TGA sin compresión, como la prepara `sgl_LoadMemoryTexture`
/// para bloquearla: formato 4, 32 bits B, G, R, A de arriba abajo (el orden de bytes del propio TGA) y paso `ancho * 4`. Los de 24
/// bits reciben alfa 0xFF. Comprobado ejecutando AIM.dll en Unicorn sobre `images/Schiffe/page_mschnigge01.tga` (256x256x32,
/// origen abajo): 65.536 píxeles iguales, también los transparentes con color. Lo necesita la prueba de píxel de los barcos de la
/// carta (`nav_map_handle_point_click`, `0x0044FE26`): bloquea `page_m*.tga` y lee un dword distinto de cero bajo el cursor; sin
/// bloqueo leía un puntero viejo y no se podía seleccionar un barco ni ordenar atacar a un pirata con el botón derecho.
fn tga_as_aim(data: &[u8]) -> Option<p3_assets::aim::Image> {
    let (w, h, rgba) = p3_render::decode_tga(data)?;
    let mut pixels = rgba;
    for p in pixels.chunks_exact_mut(4) {
        p.swap(0, 2);
    }
    Some(p3_assets::aim::Image { pixels, palette: None, width: w as u32, height: h as u32, bytes_per_pixel: 4, format: 4, pitch: (w * 4) as u32 })
}

/// [`decode`] de `crate::navmap`, público para la comprobación del CPR entero.
pub fn decode_image(data: &[u8], name: &str) -> Option<Image> {
    decode(data, name)
}

fn default_image_budget() -> Option<usize> {
    if let Some(mb) = std::env::var("P3_IMAGE_BUDGET_MB").ok().and_then(|v| v.parse::<usize>().ok()) {
        return (mb != 0).then_some(mb << 20);
    }
    cfg!(target_arch = "wasm32").then_some(48 << 20)
}

impl<'a> Assets<'a> {
    pub fn load(archive: &'a Archive<'a>) -> Result<Self, String> {
        let ini = archive.find(b"scripts/textures.ini").and_then(|e| archive.read(e)).ok_or("falta scripts/textures.ini")?;
        let mut defs = parse_textures(&String::from_utf8_lossy(ini));
        let mut images = HashMap::new();
        let fonts = load_fonts(archive, &mut defs, &mut images);
        Ok(Self { archive, defs: defs.into_iter().collect(), images: images.into_iter().collect(), fonts: fonts.into_iter().collect(), pending: U32Map::default(), found: Default::default(), live: U32Map::default(), live_bytes: 0, clock: 0, image_budget: default_image_budget(), image_budget_max: default_image_budget().map_or(0, |b| b * 4), thrash: 0, redecodes: 0, own_mode: U32Map::default() })
    }

    /// Número de secciones `[FontN]` de `fonts.ini` (la mayor N + 1).
    pub fn font_sections(&self) -> u32 {
        let Some(ini) = self.file(b"scripts/fonts.ini") else { return 0 };
        String::from_utf8_lossy(&ini)
            .lines()
            .filter_map(|l| l.trim().strip_prefix("[Font").and_then(|l| l.strip_suffix(']')).and_then(|n| n.parse::<u32>().ok()))
            .max()
            .map_or(0, |n| n + 1)
    }

    /// Contenido de un archivo del CPR (nombre con `/` o `\\`, sin distinguir mayúsculas).
    pub fn file(&self, name: &[u8]) -> Option<Vec<u8>> {
        self.file_transient(name).map(std::borrow::Cow::into_owned)
    }

    /// Bytes de un archivo para usarlos y soltarlos: con el CPR por tramos (web) no quedan guardados en su caché.
    pub fn file_transient(&self, name: &[u8]) -> Option<std::borrow::Cow<'a, [u8]>> {
        let name: Vec<u8> = name.iter().map(|&b| if b == b'\\' { b'/' } else { b }).collect();
        let archive: &'a Archive<'a> = self.archive;
        let entry = *self.found.borrow_mut().entry(name).or_insert_with_key(|name| archive.find(name));
        entry.and_then(|e| archive.read_transient(e))
    }

    /// Como [`Assets::file`] pero sin copiar: los bytes del archivo dentro del CPR.
    pub fn file_bytes(&self, name: &[u8]) -> Option<&'a [u8]> {
        let name: Vec<u8> = name.iter().map(|&b| if b == b'\\' { b'/' } else { b }).collect();
        let archive: &'a Archive<'a> = self.archive;
        let entry = *self.found.borrow_mut().entry(name).or_insert_with_key(|name| archive.find(name));
        entry.and_then(|e| archive.read(e))
    }

    /// Descodifica ahora la textura `id` si su carga estaba aplazada; después está en `images` como si se hubiera descodificado al
    /// cargarla. Todo acceso a `images` por manejador pasa antes por aquí (o por [`Assets::image`]). Entrar en partida carga unas
    /// 3.500 texturas (806 MB en RGBA: todos los edificios y figuras de la ciudad) y dibuja solo una parte.
    pub fn materialize(&mut self, id: u32) {
        if !self.live.is_empty() {
            if let Some(l) = self.live.get_mut(&id) {
                self.clock += 1;
                l.1 = self.clock;
                return;
            }
        }
        if self.pending.is_empty() {
            return;
        }
        let Some(p) = self.pending.remove(&id) else { return };
        let img = self.file_transient(&p.raw).and_then(|data| decode(&data, &p.file));
        let Some(mut img) = img else {
            // No debería ocurrir (ver `lazy_decodable`): solo con un CPR dañado. La textura queda sin imagen y no se dibuja.
            eprintln!("textura aplazada {:?}: no se pudo descodificar", p.file);
            return;
        };
        if let Some((r, g, b)) = p.key {
            for px in img.rgba.chunks_exact_mut(4) {
                if (px[0], px[1], px[2]) == (r, g, b) {
                    px[3] = 0;
                }
            }
        }
        if let Some(at) = p.evicted {
            self.redecodes += 1;
            // Soltada y pedida otra vez enseguida: lo que se dibuja no cabe en el tope y se estaría descodificando en cada
            // fotograma. Cada 64 veces el tope crece un octavo (hasta cuatro veces el inicial) hasta que deja de ocurrir.
            if self.clock - at < 4096 {
                self.thrash += 1;
                if self.thrash % 64 == 0 {
                    if let Some(b) = self.image_budget.as_mut() {
                        *b = (*b + *b / 8).min(self.image_budget_max);
                    }
                }
            }
        }
        let bytes = img.rgba.len();
        self.images.insert(id, img);
        self.track(id, p, bytes);
    }

    /// Tope en bytes para las imágenes RGBA que se pueden rehacer desde el CPR (`None`: sin tope, todo queda descodificado como
    /// siempre). Al pasarlo se sueltan las que llevan más tiempo sin usarse; vuelven a `pending` y se descodifican de nuevo, con
    /// la misma clave de color, la próxima vez que alguien las pida: los píxeles que se dibujan son los mismos.
    /// Por defecto no hay tope salvo en `wasm32` (48 MiB) o con `P3_IMAGE_BUDGET_MB` (0 = sin tope).
    pub fn set_image_budget(&mut self, bytes: Option<usize>) {
        self.image_budget = bytes;
        self.image_budget_max = bytes.map_or(0, |b| b * 4);
        if bytes.is_none() {
            self.live.clear();
            self.live_bytes = 0;
        }
        self.evict(u32::MAX);
    }

    /// Suma de los tamaños de los archivos del CPR pedidos hasta ahora por nombre (diagnóstico: es lo que retenía la caché del
    /// CPR por tramos cuando toda lectura quedaba guardada).
    pub fn requested_bytes(&self) -> usize {
        self.found.borrow().values().flatten().map(|e| e.size as usize).sum()
    }

    /// Tope vigente (puede haber crecido, ver `materialize`).
    pub fn image_budget(&self) -> Option<usize> {
        self.image_budget
    }

    /// Bytes RGBA de las imágenes que se pueden soltar ahora mismo.
    pub fn evictable_bytes(&self) -> usize {
        self.live_bytes
    }

    /// Anota que `id` acaba de descodificarse con la receta `p` y suelta las más antiguas si se supera el tope.
    fn track(&mut self, id: u32, p: PendingImage, bytes: usize) {
        if self.image_budget.is_none() {
            return;
        }
        self.clock += 1;
        if let Some(old) = self.live.insert(id, (p, self.clock, bytes)) {
            self.live_bytes -= old.2;
        }
        self.live_bytes += bytes;
        self.evict(id);
    }

    fn evict(&mut self, keep: u32) {
        let Some(budget) = self.image_budget else { return };
        while self.live_bytes > budget {
            let Some(oldest) = self.live.iter().filter(|(id, _)| **id != keep).min_by_key(|(_, l)| l.1).map(|(id, _)| *id) else { break };
            let (mut p, _, bytes) = self.live.remove(&oldest).unwrap();
            self.live_bytes -= bytes;
            self.images.remove(&oldest);
            p.evicted = Some(self.clock);
            self.pending.insert(oldest, p);
        }
    }

    /// La imagen de `id` va a cambiar, moverse o desaparecer: deja de poder rehacerse desde el CPR.
    fn pin(&mut self, id: u32) {
        if let Some((_, _, bytes)) = self.live.remove(&id) {
            self.live_bytes -= bytes;
        }
    }

    /// Imagen de la textura `id` (con la clave de color ya aplicada), cargándola del CPR la primera vez.
    pub fn image(&mut self, id: u32) -> Option<&Image> {
        self.materialize(id);
        if !self.images.contains_key(&id) {
            let def = self.defs.get(&id)?;
            let data = self.archive.find(def.file.as_bytes()).and_then(|e| self.archive.read_transient(e))?;
            let mut img = decode(&data, &def.file)?;
            // Sin `UseAlpha` en `textures.ini`, el cargador del original pide alfa (indicadores 3) a los TGA de 32 bits (los BMP quedan en 1).
            let alpha_tga = def.flags & 7 == 1 && def.file.to_ascii_lowercase().ends_with(".tga") && data.get(16) == Some(&32);
            if alpha_tga {
                self.defs.get_mut(&id)?.flags |= 3;
            }
            let def = self.defs.get(&id)?;
            if let Some(key) = def.color_key {
                let [_, r, g, b] = key.to_be_bytes();
                for p in img.rgba.chunks_exact_mut(4) {
                    if (p[0], p[1], p[2]) == (r, g, b) {
                        p[3] = 0;
                    }
                }
            }
            self.images.insert(id, img);
        }
        self.images.get(&id)
    }
}

/// Superficie de dibujo (RGBA) con el estado de la DLL SGL.
pub struct Surface {
    pub width: usize,
    pub height: usize,
    pub rgba: Vec<u8>,
    pub color: u32,
    pub source: u32,
    /// Último `blit` pedido (textura y rectángulos), para quien alarga piezas tras su dibujo original (`display.rs`).
    pub last_blit: Option<(u32, (i32, i32, i32, i32, i32, i32))>,
    pub mode: u8,
    pub font: u32,
    pub text_mode: i32,
    /// Destino en una textura de memoria (rasterizador por software de la DLL). En pantalla, `false`: ver `blit`.
    pub in_texture: bool,
}

impl Surface {
    pub fn new(width: usize, height: usize, background: [u8; 4]) -> Self {
        Self { width, height, rgba: background.repeat(width * height), color: 0xFFFF_FFFF, source: 0, last_blit: None, mode: 1, font: 0, text_mode: 1, in_texture: false }
    }

    /// `sgl_DrawBitmapRect(sx, sy, dx, dy, w, h)` con la textura activa, el color y la mezcla (`p3_render::sgl::draw_bitmap_rect`).
    pub fn blit(&mut self, assets: &mut Assets, handle: u32, (sx, sy, dx, dy, w, h): (i32, i32, i32, i32, i32, i32)) {
        use p3_render::sgl;
        if assets.image(handle).is_none() {
            return;
        }
        let def = assets.defs.get(&handle).map(|d| (d.color_key.unwrap_or(0), d.flags));
        let Some(img) = assets.images.get(&handle) else { return };
        self.last_blit = Some((handle, (sx, sy, dx, dy, w, h)));
        let (mut sx, mut sy, mut dx, mut dy, mut w, mut h) = (sx, sy, dx, dy, w, h);
        for (s, d, n) in [(&mut sx, &mut dx, &mut w), (&mut sy, &mut dy, &mut h)] {
            let cut = (-*s).max(-*d).max(0);
            *s += cut;
            *d += cut;
            *n -= cut;
        }
        w = w.min(img.width as i32 - sx).min(self.width as i32 - dx);
        h = h.min(img.height as i32 - sy).min(self.height as i32 - dy);
        if w <= 0 || h <= 0 {
            return;
        }
        let (key, flags) = def.unwrap_or((0, 1));
        // The native bitmap path reads the source texture blend byte (+0x37).
        // Its multiply mode is essential for the white-backed ship illustrations.
        let texture_mode = (flags >> 24) as u8;
        let mode = assets.own_mode.get(&handle).copied().unwrap_or(if texture_mode == 0 { self.mode } else { texture_mode });
        let routine = sgl::select_routine(mode, self.color, flags);
        // The screen path applies the DirectDraw source key in every blend mode.
        // Keep the previous bridge's keyed marker behaviour, including its alpha.
        let keyed = !self.in_texture && flags & 7 == 7;
        // Textura con alfa sin clave (indicadores `& 7 == 3`) en un modo de producto (rutinas 7..10, modos 2 y 5, que no
        // miran el alfa): en pantalla el original no dibuja los texeles de alfa 0. Las sombras de los soldados de la ciudad
        // (`_X_schwertk_NEU_shd_TP_8.bmp` y las de arco, ballesta y mosquete) son BMP de 8 bits con fondo magenta de alfa 0
        // que `0x591C50` pinta con `sgl_SetBlendMode(2)` (producto con el destino) y que el juego carga con indicadores 3;
        // sin este salto el fondo magenta multiplicaba el suelo y dejaba un bloque rosa bajo los pies de los guardias.
        // Las sombras AIM de los demás vecinos llevan indicadores 7 y ya se saltan por la clave.
        let skip_clear = !self.in_texture && flags & 7 == 3 && (7..=10).contains(&routine);
        // Producto con color de alfa < 0xFF (rutina 7) sobre una textura con alfa: en pantalla el resultado se mezcla con el destino
        // por alfa del texel × alfa del color. Es la sombra del humo de los barcos dañados (`0x579EC0`, códigos 0x20-0x25 de los
        // efectos tipo 18, llamada desde el pase de sombras `0x591C50` con `sgl_SetBlendMode(2)`): pinta el propio fotograma del humo
        // desplazado con color `(opacidad·100/255) << 24`, negro con alfa ≤ 0x64. Sin la mezcla, el producto por un color negro dejaba
        // un círculo negro sólido bajo cada nube (17/11); con ella es un oscurecimiento suave del agua de como mucho el 39 %.
        let shade = !self.in_texture && flags & 3 == 3 && routine == 7 && self.color >> 24 != 0xFF;
        if shade {
            let ca = self.color >> 24;
            let cb = self.color.to_le_bytes(); // [b, g, r, a]
            let cr = [cb[2], cb[1], cb[0]].map(u32::from);
            for y in 0..h as usize {
                let src = ((sy as usize + y) * img.width + sx as usize) * 4;
                let dst = ((dy as usize + y) * self.width + dx as usize) * 4;
                for x in 0..w as usize {
                    let s = &img.rgba[src + x * 4..src + x * 4 + 4];
                    let a = u32::from(s[3]) * ca / 255;
                    if a == 0 {
                        continue;
                    }
                    let d = &mut self.rgba[dst + x * 4..dst + x * 4 + 3];
                    for i in 0..3 {
                        let dv = u32::from(d[i]);
                        let prod = (u32::from(s[i]) * cr[i] >> 8) * dv >> 8;
                        d[i] = ((dv * (255 - a) + prod * a) / 255) as u8;
                    }
                }
            }
            return;
        }
        for y in 0..h as usize {
            let src = ((sy as usize + y) * img.width + sx as usize) * 4;
            let dst = ((dy as usize + y) * self.width + dx as usize) * 4;
            let source = &img.rgba[src..src + w as usize * 4];
            let target = &mut self.rgba[dst..dst + w as usize * 4];
            if skip_clear {
                // Tramos de texeles con alfa distinto de 0, mezclados igual que el resto.
                let mut x = 0;
                let n = w as usize;
                while x < n {
                    if source[x * 4 + 3] == 0 {
                        x += 1;
                        continue;
                    }
                    let start = x;
                    while x < n && source[x * 4 + 3] != 0 {
                        x += 1;
                    }
                    blit::row(&mut target[start * 4..x * 4], &source[start * 4..x * 4], routine, self.color, key, keyed);
                }
                continue;
            }
            blit::row(target,source,routine,self.color,key,keyed);
        }
    }

    /// `sgl_StretchBitmapRect(sx, sy, dx, dy, w, h, w2, h2)` (`0x004BBA80`): copia escalada con la textura activa, el color y la mezcla
    /// (`p3_render::sgl::draw_scaled_rect`, vecino más cercano salvo `UseFiltering`). `clip` en coordenadas de la superficie.
    #[allow(clippy::too_many_arguments)]
    pub fn stretch(&mut self, assets: &mut Assets, handle: u32, (sx, sy): (i32, i32), (dx, dy): (i32, i32), (w, h): (i32, i32), (w2, h2): (i32, i32), clip: [i32; 4]) {
        use p3_render::sgl;
        if assets.image(handle).is_none() {
            return;
        }
        let (key, flags) = assets.defs.get(&handle).map_or((0, 1), |d| (d.color_key.unwrap_or(0), d.flags));
        let Some(img) = assets.images.get(&handle) else { return };
        // El origen tiene que contener el rectángulo leído (el controlador no lo comprueba: fuera sería memoria ajena).
        if sx < 0 || sy < 0 || w < 0 || h < 0 || sx + w > img.width as i32 || sy + h > img.height as i32 {
            return;
        }
        let texture_mode = (flags >> 24) as u8;
        let mode = assets.own_mode.get(&handle).copied().unwrap_or(if texture_mode == 0 { self.mode } else { texture_mode });
        let px = |p: &[u8]| u32::from_le_bytes([p[2], p[1], p[0], p[3]]);
        let src: Vec<u32> = img.rgba.chunks_exact(4).map(px).collect();
        let mut dst: Vec<u32> = self.rgba.chunks_exact(4).map(px).collect();
        let clip = [clip[0].max(0), clip[1].max(0), clip[2].min(self.width as i32), clip[3].min(self.height as i32)];
        if clip[2] <= clip[0] || clip[3] <= clip[1] {
            return;
        }
        let tex = sgl::Texture { pixels: &src, pitch: img.width, key, flags };
        let st = sgl::BlitState { mode, color: self.color, clip: Some(clip) };
        let mut out = sgl::Surface { pixels: &mut dst, pitch: self.width };
        sgl::draw_scaled_rect(&mut out, &tex, &st, (img.width as u32, img.height as u32), (sx, sy), (dx, dy), (w, h), (w2, h2));
        for (d, v) in self.rgba.chunks_exact_mut(4).zip(dst) {
            let [b, g, r, a] = v.to_le_bytes();
            d.copy_from_slice(&[r, g, b, a]);
        }
    }

    /// Copia un rectángulo de la propia superficie (origen y destino pueden solaparse, como `memmove`), recortado a sus límites.
    pub fn copy_rect(&mut self, sx: i32, sy: i32, dx: i32, dy: i32, w: i32, h: i32) {
        let (mut sx, mut sy, mut dx, mut dy, mut w, mut h) = (sx, sy, dx, dy, w, h);
        for (s, d, n, limit) in [(&mut sx, &mut dx, &mut w, self.width as i32), (&mut sy, &mut dy, &mut h, self.height as i32)] {
            let cut = (-*s).max(-*d).max(0);
            *s += cut;
            *d += cut;
            *n -= cut;
            *n = (*n).min(limit - *s).min(limit - *d);
        }
        if w <= 0 || h <= 0 {
            return;
        }
        let row = |y: i32, x: i32| (y as usize * self.width + x as usize) * 4;
        let rows: Box<dyn Iterator<Item = i32>> = if dy > sy { Box::new((0..h).rev()) } else { Box::new(0..h) };
        for y in rows {
            let src = row(sy + y, sx);
            let dst = row(dy + y, dx);
            self.rgba.copy_within(src..src + w as usize * 4, dst);
        }
    }

    /// Rectángulo lleno del color activo (mezcla alfa).
    pub fn fill(&mut self, x: i32, y: i32, w: i32, h: i32) {
        let [a, r, g, b] = self.color.to_be_bytes();
        for yy in y.max(0)..(y + h).min(self.height as i32) {
            for xx in x.max(0)..(x + w).min(self.width as i32) {
                let o = (yy as usize * self.width + xx as usize) * 4;
                let a = u32::from(a);
                for (k, c) in [r, g, b].into_iter().enumerate() {
                    let d = u32::from(self.rgba[o + k]);
                    self.rgba[o + k] = ((u32::from(c) * a + d * (255 - a)) / 255) as u8;
                }
            }
        }
    }

    /// `sgl_DrawText(x, y, texto)` con la fuente y el modo activos.
    pub fn text(&mut self, assets: &mut Assets, x: i32, y: i32, s: &[u8]) {
        let Some(font) = assets.fonts.get(&self.font) else { return };
        let glyphs = font.draw(x, y, s, self.text_mode);
        let handle = self.font;
        for [sx, sy, dx, dy, w, h] in glyphs {
            self.blit(assets, handle, (sx, sy, dx, dy, w, h));
        }
    }

    /// Guarda la superficie como PNG.
    pub fn write_png(&self, path: &std::path::Path) -> Result<(), String> {
        p3_render::write_png(path, self.width, self.height, &self.rgba).map_err(|e| e.to_string())
    }
}

/// Enlazador: estado del CRT/SGL sustituidos y registro de lo que falta.
/// Carga del desenrollado que corta una ejecución agotada (ver [`Runtime::call`]).
struct AbortRun;

pub struct Runtime<'a> {
    pub assets: Assets<'a>,
    pub surface: Surface,
    pub heap: u32,
    /// Presentaciones (`sgl_SwapBuffer`) desde el arranque (diagnóstico `P3_HEAPLOG=N`: cima del montón cada N).
    pub presents: u64,
    abort_logged: bool,
    /// Se ha usado alguna vez una resolución moderna grande: el tope del montón ya no vuelve a 128 MiB.
    /// Mover la flecha de resolución en Opciones cambia `display.modern` antes de aplicar; con la partida
    /// ya cargada a 2560x1440 (≈136 MiB) eso cerraba el juego (14/11, 23:41).
    heap_big: bool,
    sp: u32,
    /// Direcciones llamadas sin porte ni sustituto, con su número de llamadas.
    pub missing: BTreeMap<u32, u32>,
    /// Llamadas resueltas por porte (dirección → veces).
    /// Modo de poca memoria (ver [`Runtime::set_low_memory`]).
    low_memory: bool,
    pub ported: BTreeMap<u32, u32>,
    /// Operaciones del juego que un guion de prueba pide encolar (registro de 20 bytes, como las del jugador); se encolan al
    /// presentar el fotograma. Compartido para que los ganchos `frame_script`/`on_present` las dejen.
    pub operaciones_guion: std::rc::Rc<std::cell::RefCell<Vec<[u32; 5]>>>,
    /// Cuenta en caliente de `ported` (tabla de dispersión: el árbol ordenado costaba un 4 % de la carga de una ciudad). Se vuelca
    /// en `ported` al volver de cada [`Runtime::call`], que es donde lo leen las pruebas y los informes.
    ported_hot: U32Map<u32>,
    /// Opt-in diagnostic timing (`P3_UI_PROFILE`): address -> (calls, self ns, inclusive ns).
    /// Leave disabled for FPS comparisons; instrumenting each call has overhead.
    pub profile: Option<BTreeMap<u32, (u64, u128, u128)>>,
    profile_children: Vec<u128>,
    profile_from_frame: u32,
    depth: u32,
    /// Pila de llamadas en curso (para atribuir lo que falta).
    pub stack: Vec<u32>,
    /// Recortes 2D de la DLL (`p3_render::sgl_clip`, paridad con `ddraw_Dll.dll`).
    pub clip: p3_render::sgl_clip::SglClip,
    render_dest: u32,
    /// Textura que hace de destino de dibujo (su imagen vive en `surface` mientras tanto) y la pantalla apartada.
    target_tex: Option<u32>,
    screen_stash: Option<(usize, usize, Vec<u8>)>,
    /// Espejos en memoria emulada de la pantalla para `sgl_LockTexture(-4)` sin destino en textura, por tamaño de superficie
    /// (textura de `tex_store`, búfer), y el que está bloqueado ahora.
    /// El otro búfer de pantalla. Como el original, hay dos: el trasero (`-1`), donde se compone, y el delantero (`-3`), el que
    /// se ve. [`Runtime::surface`] es siempre el destino vigente y aquí queda el otro (`screen_is_front` dice cuál es cuál; solo
    /// se usan sus píxeles). `present_frame` (`0x4B8FF0`) copia las zonas sucias del trasero al delantero con `sgl_Blt_RC`; el
    /// desplazamiento de la ciudad (`0x592E60`) y el de la carta náutica (`0x4538E0`) copian y dibujan directamente en el
    /// delantero. `P3_ONE_BUFFER` vuelve a la superficie única.
    front: Surface,
    /// La superficie de pantalla vigente es el búfer delantero.
    screen_is_front: bool,
    /// El delantero recibió algo desde la última presentación (si no, se presenta el trasero entero).
    front_fresh: bool,
    /// Fondo del búfer trasero bajo el cursor del ratón (lo dibuja `screen_list_run_frame` al final, `0x4B8E11` → `0x4AD540`):
    /// (era el delantero, x, y, ancho, alto, píxeles). Se repone tras presentar para que el trasero no guarde el cursor.
    cursor_under: Vec<(bool, usize, usize, usize, usize, Vec<u8>)>,
    /// En este fotograma se dibujó directamente en el delantero (desplazamiento de carta o ciudad con `sgl_Blt` y la franja
    /// nueva). El trasero no lo recibe; tras presentar se copia el delantero entero al trasero para que las zonas sucias que se
    /// copien después no devuelvan la imagen de antes del desplazamiento.
    front_direct: bool,
    two_buffers: bool,
    /// [`Runtime::surface`] es el lienzo del zoom, no la pantalla: el cambio de búfer espera a que vuelva.
    canvas_active: bool,
    screen_locks: HashMap<(usize, usize), (usize, u32)>,
    screen_locked: Option<(usize, u32)>,
    /// Caja del espejo de la pantalla bloqueada que tiene ya los píxeles (y hay que devolver al desbloquear).
    screen_lock_live: Option<[i32; 4]>,
    /// Pila de entrada de cada llamada en curso (para los puentes `jmp [ranura]`).
    esps: Vec<u32>,
    force_frame: Option<u32>,
    /// Quién llamó a cada dirección que falta (llamante → veces).
    pub missing_from: BTreeMap<(u32, u32), u32>,
    /// Con `Some`, cada `new`/`malloc` anota aquí el bloque que entrega (diagnóstico: `oracle/x86_ui_dump_call.py` repite esas
    /// direcciones en el x86 para comparar la memoria sin diferencias de asignador).
    pub alloc_log: Option<Vec<u32>>,
    /// Llamadas de `p3_core::lifted` cuyo número de argumentos deducido no coincide con los que retira el destino real:
    /// (función llamante, dirección de retorno) → (palabras reales, deducidas). Entrada de `tools/ui_rt_fix.py`.
    pub arg_mismatch: BTreeMap<(u32, u32), (u32, u32)>,
    /// Rutinas de la DLL SGL llamadas sin sustituto.
    pub sgl_missing: std::collections::BTreeSet<String>,
    /// Texturas de memoria de la DLL (`p3_render::sgl_tex`): estado, y por manejador su identificador, el espejo de los píxeles en la
    /// memoria emulada (lo que `sgl_LockTexture` entrega al llamante) y la copia de la paleta del primer bloqueo.
    tex_store: p3_render::sgl_tex::TexStore,
    mem_tex: U32Map<(usize, u32, u32)>,
    /// Manejadores de `mem_tex` que vienen de un archivo (no se rehace su imagen RGBA al desbloquear).
    loaded_tex: std::collections::HashSet<u32>,
    /// Llamadas hechas y tope (un porte que no termina deja de llamar al pasarlo).
    pub calls: u64,
    pub call_limit: u64,
    prof: u64,
    missing_log: bool,
    /// Avisos de diagnóstico (función ausente, importación o SGL sin sustituto, aborto) para clientes sin salida de error, como el
    /// navegador. Con `Some`, cada aviso se entrega una vez.
    pub diag: Option<Box<dyn FnMut(String)>>,
    /// Bloques de `malloc`/`new` vivos (dirección → tamaño redondeado) y listas libres por tamaño (`heap_take`/`heap_free`).
    heap_blocks: U32Map<u32>,
    /// `.ini` del CPR ya analizados (ruta → tabla; `None` si el archivo no existe). Ver [`Runtime::ini_value`].
    ini_cache: HashMap<Vec<u8>, Option<HashMap<(Vec<u8>, Vec<u8>), Vec<u8>>>>,
    /// `P3_SGL_LOG`: escribe cada llamada sgl sustituida con sus seis primeras palabras de pila.
    sgl_log: bool,
    watch: Vec<u32>,
    watch_mem: Option<(bool, u32, u32)>,
    watch_mem_last: u32,
    heap_free_lists: U32Map<Vec<u32>>,
    /// Bloques libres grandes (`>= TRIM_MIN`) como `(tamaño, dirección)`: se reutilizan con el menor que quepa y se parten
    /// (ver [`Runtime::heap_take`]).
    heap_free_big: std::collections::BTreeSet<(u32, u32)>,
    /// Los mismos bloques por dirección (tamaño), para juntar los vecinos al liberar.
    heap_free_big_at: std::collections::BTreeMap<u32, u32>,
    /// Se superó un tope (profundidad, llamadas o montón): el resultado no es fiable.
    pub aborted: bool,
    /// Escribe cada llamada en stderr (`P3_UI_TRACE`).
    pub trace: bool,
    /// Textos dibujados (`sgl_DrawText`/`ui_render_text_at`) en orden: posición y texto. Las pruebas cotejan con ellos las cifras de
    /// una página contra el cliente.
    pub texts: Vec<(i32, i32, String)>,
    core: U32Map<core_table::CoreFn>,
    hand: U32Map<hand_table::HandFn>,
    sgl: U32Map<std::rc::Rc<str>>,
    imports: U32Map<std::rc::Rc<str>>,
    /// Contenedor activo de la DLL (`sgl_SetArchive`) y manejadores de texturas y fuentes creadas en ejecución.
    sgl_archive: u32,
    next_handle: u32,
    /// Archivos abiertos con `arc_CreateFile`: manejador → (datos, posición).
    files: U32Map<(Vec<u8>, usize)>,
    next_file: u32,
    /// Archivos proyectados con `arc_CreateMemMapped`: ruta → (dirección, tamaño) de su única copia.
    mapped: HashMap<Vec<u8>, (u32, u32)>,
    /// Importaciones llamadas sin sustituto ("dll!nombre").
    pub import_missing: std::collections::BTreeSet<String>,
    /// `mss32.dll` sustituida (`miles.rs`): mezclador del cliente y manejadores de Miles.
    pub miles: miles::MilesState,
    /// `Vto.dll` sustituida (`vto.rs`): vídeos `Videos\*.sks` de las ventanas de suceso.
    pub vto: vto::VtoState,
    /// Estado del arranque, el menú principal y la barra lateral (`menus.rs`).
    pub menus: menus::MenuState,
    /// Cola de mensajes de Windows (`PeekMessageA`/`GetMessageA`); la llena la ventana nativa ([`Runtime::on_present`]) o
    /// [`Runtime::post_message`]. Ver `win32.rs` y `reimpl/UI_PANTALLA_COMPLETA.md`.
    pub messages: std::collections::VecDeque<win32::Msg>,
    /// Ratón, teclado y reloj que leen `GetCursorPos`, `GetKeyState`, `GetAsyncKeyState` y `timeGetTime`.
    pub input: win32::InputState,
    /// Gancho de `sgl_SwapBuffer`: presenta [`Runtime::surface`] y recoge la entrada; `false` encola `WM_QUIT`.
    pub on_present: Option<win32::Present>,
    /// Recoge eventos durante cargas largas sin presentar ni ejecutar otro frame.
    pub on_pump: Option<Box<dyn FnMut(&mut std::collections::VecDeque<win32::Msg>, &mut win32::InputState) -> bool>>,
    /// `CreateProcessA` del juego: solo lo usa la introducción (`game_main_window_initialize` `0x00463821`:
    /// `binkplay.exe Intro_1.bik Intro_2.bik Intro_3.bik /d408 /b2 /k ...`, y luego espera con `WaitForSingleObject`).
    /// Recibe la línea de órdenes y devuelve si lanzó (y esperó) el proceso; sin gancho, falla como hasta ahora y no hay intro.
    pub on_create_process: Option<Box<dyn FnMut(&str) -> bool>>,
    /// Modo de vídeo y opción de resoluciones modernas (`display.rs`).
    pub display: display::DisplayState,
    pub video_options: video_options::VideoOptions,
    pub language: language::LanguageOptions,
    pub zoom: zoom::Zoom,
}

/// Tabla de exportaciones que lee `sgl_load_driver_dll` (`GetProcAddress`): registros de 0x44 bytes `[ranura u32][nombre 0x40]`
/// en los datos del EXE (180 entradas, en torno a `0x006BCD70`). Se busca por su contenido para no fijar direcciones.
/// BMP de 24 bits de abajo arriba (filas alineadas a 4 bytes) a partir de RGBA.
fn bmp24(w: usize, h: usize, rgba: &[u8]) -> Vec<u8> {
    let row = (w * 3 + 3) & !3;
    let size = 54 + row * h;
    let mut out = Vec::with_capacity(size);
    out.extend_from_slice(b"BM");
    out.extend_from_slice(&(size as u32).to_le_bytes());
    out.extend_from_slice(&[0; 4]);
    out.extend_from_slice(&54u32.to_le_bytes());
    out.extend_from_slice(&40u32.to_le_bytes());
    out.extend_from_slice(&(w as i32).to_le_bytes());
    out.extend_from_slice(&(h as i32).to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&24u16.to_le_bytes());
    out.extend_from_slice(&[0; 4]);
    out.extend_from_slice(&((row * h) as u32).to_le_bytes());
    out.extend_from_slice(&[0; 16]);
    for y in (0..h).rev() {
        let start = out.len();
        for p in rgba[y * w * 4..(y + 1) * w * 4].chunks_exact(4) {
            out.extend_from_slice(&[p[2], p[1], p[0]]);
        }
        out.resize(start + row, 0);
    }
    out
}

fn sgl_exports(mem: &PageMem) -> Vec<(String, u32)> {
    let (lo, hi) = (0x0066_A000u32, 0x006F_2000u32);
    let data = mem.read(lo, (hi - lo) as usize);
    let Some(at) = data.windows(21).position(|w| w == b"sgl_SetConstantColor\0") else { return Vec::new() };
    let is_name = |o: usize| data.get(o..o + 4).is_some_and(|p| p == b"sgl_" || p == b"ddra");
    let mut start = at;
    while start >= 0x44 && is_name(start - 0x44) {
        start -= 0x44;
    }
    let mut out = Vec::new();
    let mut o = start;
    while is_name(o) && o >= 4 {
        let name: Vec<u8> = data[o..o + 0x40].iter().copied().take_while(|&b| b != 0).collect();
        let slot = u32::from_le_bytes(data[o - 4..o].try_into().unwrap());
        if (0x006D_A800..0x006D_AB00).contains(&slot) {
            out.push((String::from_utf8_lossy(&name).into_owned(), slot));
        }
        o += 0x44;
    }
    out
}

/// Lado que reciben las funciones de `p3_core::lifted`: misma memoria y mismo enlazador.
struct CoreSide<'r, 'a> {
    rt: &'r mut Runtime<'a>,
}

impl p3_core::economy::ops_misc::OpsHost for CoreSide<'_, '_> {
    fn call(&mut self, m: &mut dyn p3_core::flatmem::Mem, addr: u32, ecx: u32, args: &[u32]) -> u32 {
        // La memoria del enlazador es siempre el mismo `PageMem`: se le llama directamente en vez de apilar un adaptador
        // (`CoreMem`) que añadía un salto indirecto más a cada lectura y escritura de todo lo que cuelga de esta llamada.
        if let Some(page) = m.as_any_mut().and_then(|a| a.downcast_mut::<PageMem>()) {
            return self.rt.dispatch(page, addr, ecx, args);
        }
        let mut mem = CoreMem(m);
        self.rt.dispatch(&mut mem, addr, ecx, args)
    }
}

/// Pone a cero `n` bytes desde `at` por tramos (lo mismo que un `w8(…, 0)` por byte, sin una llamada por byte: `operator new`
/// de un búfer de imagen eran millones).
pub(crate) fn zero_fill(m: &mut dyn p3_render::scene_math::Mem, at: u32, n: u32) {
    static ZERO: [u8; PAGE as usize] = [0; PAGE as usize];
    let mut done = 0u32;
    while done < n {
        let step = (n - done).min(PAGE);
        m.write_bytes(at.wrapping_add(done), &ZERO[..step as usize]);
        done += step;
    }
}

/// Vista de una memoria de `p3_core` como memoria de `p3_render` (son el mismo `PageMem`).
struct CoreMem<'m>(&'m mut dyn p3_core::flatmem::Mem);
impl p3_render::scene_math::Mem for CoreMem<'_> {
    fn core_view(&mut self) -> Option<&mut dyn p3_core::flatmem::Mem> { Some(self.0) }
    fn release(&mut self, a: u32, len: u32) { self.0.release(a, len); }
    fn read_bytes(&mut self, a:u32, out:&mut [u8]) { self.0.read_bytes(a,out); }
    fn write_bytes(&mut self, a:u32, data:&[u8]) { self.0.write_bytes(a,data); }
    fn cmp_cstr(&mut self, a: u32, b: u32, fold: bool, max: usize) -> Option<std::cmp::Ordering> { self.0.cmp_cstr(a, b, fold, max) }
    fn r8(&mut self, a: u32) -> u8 {
        self.0.r8(a)
    }
    fn r16(&mut self, a: u32) -> u16 { self.0.r16(a) }
    fn r32(&mut self, a: u32) -> u32 { self.0.r32(a) }
    fn w8(&mut self, a: u32, v: u8) {
        self.0.w8(a, v)
    }
    fn w16(&mut self, a: u32, v: u16) { self.0.w16(a, v) }
    fn w32(&mut self, a: u32, v: u32) {
        self.0.w32(a, v)
    }
}
struct RenderMem<'m>(&'m mut dyn p3_render::scene_math::Mem);
fn with_core_mem<T>(m: &mut dyn p3_render::scene_math::Mem, f: impl FnOnce(&mut dyn p3_core::flatmem::Mem) -> T) -> T {
    if m.core_view().is_some() {
        f(m.core_view().unwrap())
    } else {
        f(&mut RenderMem(m))
    }
}

/// Caja de lectura de `0x59E8E0(pixels, x, y, clip, pitch)`: el puntero inicial no es una coordenada.
fn screen_lock_bounds(m: &mut dyn p3_render::scene_math::Mem, this: u32, args: &[u32], w: usize, h: usize) -> Option<[i32; 4]> {
    if args.len() < 4 { return None; }
    let n = (m.r32(this.wrapping_add(0x2C)) as i32).clamp(0, 0x1770);
    let (cx, cy) = (m.r32(this.wrapping_add(0xBBC4)) as i32, m.r32(this.wrapping_add(0xBBC8)) as i32);
    let clip = [0u32, 4, 8, 12].map(|o| m.r32(args[3].wrapping_add(o)) as i32);
    let mut bb = [i32::MAX, i32::MAX, i32::MIN, i32::MIN];
    for k in 0..n as u32 {
        let e = this.wrapping_add(0x34 + k * 8);
        let x = (i32::from(m.r32(e) as u16 as i16) << 3).wrapping_sub(cx).wrapping_sub(args[1] as i32);
        let y = (i32::from((m.r32(e) >> 16) as u16 as i16) << 3).wrapping_sub(cy).wrapping_sub(args[2] as i32);
        let r = [x.max(clip[0]), y.max(clip[1]), x.saturating_add(8).min(clip[2]), y.saturating_add(8).min(clip[3])];
        if r[2] > r[0] && r[3] > r[1] {
            bb = [bb[0].min(r[0]), bb[1].min(r[1]), bb[2].max(r[2]), bb[3].max(r[3])];
        }
    }
    let bb = [(bb[0].saturating_sub(SCREEN_LOCK_MARGIN)).max(0), (bb[1].saturating_sub(SCREEN_LOCK_MARGIN)).max(0), bb[2].saturating_add(SCREEN_LOCK_MARGIN).min(w as i32), bb[3].saturating_add(SCREEN_LOCK_MARGIN).min(h as i32)];
    (bb[2] > bb[0] && bb[3] > bb[1]).then_some(bb)
}

#[cfg(test)]
mod shadow_bmp_tests {
    use super::*;

    /// BMP de 8 bits como las sombras de los soldados: índice 0 negro opaco, índice 5 magenta con reservado 255 (alfa 0).
    fn shadow_bmp() -> Vec<u8> {
        let (w, h) = (4i32, 1i32);
        let mut pal = vec![[0u8; 4]; 256];
        pal[5] = [255, 0, 255, 255];
        let pixels = [5u8, 0, 0, 5];
        let off = 54 + 256 * 4;
        let mut v = b"BM".to_vec();
        v.extend(((off + pixels.len()) as u32).to_le_bytes());
        v.extend([0u8; 4]);
        v.extend((off as u32).to_le_bytes());
        v.extend(40u32.to_le_bytes());
        v.extend(w.to_le_bytes());
        v.extend(h.to_le_bytes());
        v.extend(1u16.to_le_bytes());
        v.extend(8u16.to_le_bytes());
        v.extend([0u8; 24]);
        for p in &pal {
            v.extend(p);
        }
        v.extend(pixels);
        v
    }

    /// `0x591C50` pinta las sombras con `sgl_SetBlendMode(2)` y color `0x64FFFFFF`: el fondo magenta (alfa 0) de una sombra
    /// cargada con indicadores 3 no debe tocar el suelo; el negro sí lo oscurece.
    #[test]
    fn multiply_shadow_skips_transparent_magenta_on_screen() {
        let bmp = shadow_bmp();
        let cpr = p3_assets::cpr::build(&[(b"scripts/textures.ini", b""), (b"s.bmp", &bmp)]).unwrap();
        let archive = Archive::parse(&cpr).unwrap();
        let mut assets = Assets::load(&archive).unwrap();
        let img = decode(&bmp, "s.bmp").unwrap();
        assets.images.insert(1, img);
        assets.defs.insert(1, TexDef { file: "s.bmp".into(), frames: vec![], color_key: None, flags: 3 });
        let ground = [195u8, 180, 167, 255];
        for mode in [2u8, 5] {
            let mut s = Surface::new(4, 1, ground);
            s.mode = mode;
            s.color = 0x64FF_FFFF;
            s.blit(&mut assets, 1, (0, 0, 0, 0, 4, 1));
            let px: Vec<&[u8]> = s.rgba.chunks_exact(4).collect();
            assert_eq!(px[0], &ground, "modo {mode}: el magenta transparente no se dibuja");
            assert_eq!(px[3], &ground, "modo {mode}: el magenta transparente no se dibuja");
            assert_eq!(&px[1][..3], &[0, 0, 0], "modo {mode}: la sombra negra oscurece");
        }
        // En una textura de memoria (rasterizador por software, `oracle/x86_sgl_blit.py`) el producto no mira el alfa: este
        // rosa es el que se veía bajo los guardias.
        let mut t = Surface::new(4, 1, ground);
        t.mode = 2;
        t.color = 0x64FF_FFFF;
        t.in_texture = true;
        t.blit(&mut assets, 1, (0, 0, 0, 0, 4, 1));
        assert_eq!(&t.rgba[..3], &[194, 0, 166]);
    }

    /// Sombra del humo de un barco dañado (`0x579EC0`): fotograma gris con alfa, modo 2 y color negro con alfa `0x17`. En pantalla
    /// oscurece el agua según alfa del texel × alfa del color, en vez de dejarla negra (círculo negro bajo el humo, 17/11).
    #[test]
    fn smoke_shadow_darkens_by_color_alpha_on_screen() {
        let cpr = p3_assets::cpr::build(&[(b"scripts/textures.ini", b"")]).unwrap();
        let archive = Archive::parse(&cpr).unwrap();
        let mut assets = Assets::load(&archive).unwrap();
        let img = Image { width: 2, height: 1, rgba: vec![200, 200, 200, 255, 200, 200, 200, 0] };
        assets.images.insert(1, img);
        assets.defs.insert(1, TexDef { file: "humo.aim".into(), frames: vec![], color_key: None, flags: 3 });
        let water = [100u8, 110, 120, 255];
        let mut s = Surface::new(2, 1, water);
        s.mode = 2;
        s.color = 0x1700_0000;
        s.blit(&mut assets, 1, (0, 0, 0, 0, 2, 1));
        // 0x17 = 23: el agua queda al 91 %, no negra; el texel transparente no la toca.
        assert_eq!(&s.rgba[..4], &[90, 100, 109, 255]);
        assert_eq!(&s.rgba[4..], &water);
    }
}

#[cfg(test)]
mod memory_bridge_tests {
    use super::*;

    #[test]
    fn screen_lock_padding_covers_extreme_wave_gradients() {
        for a in 0..=255i32 {
            for b in 0..=255i32 {
                let delta = a - b;
                let dx = 2 * (delta / 4);
                let dy = delta / 2;
                assert!(dx.abs() + 1 <= SCREEN_LOCK_MARGIN);
                assert!(dy.abs() + 1 <= SCREEN_LOCK_MARGIN);
            }
        }
    }

    #[test]
    fn screen_lock_bounds_use_native_argument_order_and_keep_the_copy_regional() {
        let mut mem = PageMem::default();
        let (this, clip) = (0x5900_0000u32, 0x5A00_0000u32);
        mem.set_u32(this + 0x2C, 1);
        mem.set_u32(this + 0x34, 32 | (32 << 16));
        for (i, v) in [0, 0, 512, 512].into_iter().enumerate() { mem.set_u32(clip + 4 * i as u32, v); }
        let [l, t, r, b] = screen_lock_bounds(&mut mem, this, &[SCREEN_LOCK_BASE, 0, 0, clip, 2048], 512, 512).unwrap();
        assert!(l <= 130 && t <= 129 && r > 382 && b > 383, "faltan muestras de gradientes extremos");
        assert!((r - l) * (b - t) < 512 * 512 / 2, "no copiar toda la pantalla por defecto");
        let shifted = screen_lock_bounds(&mut mem, this, &[SCREEN_LOCK_BASE, 16, 24, clip, 2048], 512, 512).unwrap();
        assert_eq!(shifted, [l - 16, t - 24, r - 16, b - 24]);
    }

    #[test]
    fn sgl_blt_copy_rect_moves_overlapping_rectangles_in_every_direction_and_clips() {
        let make = || {
            let mut s = Surface::new(8, 6, [0, 0, 0, 255]);
            for y in 0..6 {
                for x in 0..8 {
                    let o = (y * 8 + x) * 4;
                    s.rgba[o..o + 4].copy_from_slice(&[x as u8, y as u8, 7, 255]);
                }
            }
            s
        };
        let px = |s: &Surface, x: usize, y: usize| [s.rgba[(y * 8 + x) * 4], s.rgba[(y * 8 + x) * 4 + 1]];
        // Hacia arriba y hacia abajo (solape vertical), hacia la izquierda y la derecha.
        for (sx, sy, dx, dy) in [(0, 2, 0, 0), (0, 0, 0, 2), (2, 0, 0, 0), (0, 0, 2, 1), (3, 3, 1, 1)] {
            let (orig, mut s) = (make(), make());
            s.copy_rect(sx, sy, dx, dy, 5, 3);
            for y in 0..3usize {
                for x in 0..5usize {
                    assert_eq!(px(&s, dx as usize + x, dy as usize + y), px(&orig, sx as usize + x, sy as usize + y), "{sx},{sy}->{dx},{dy} en {x},{y}");
                }
            }
        }
        // Recortes: fuera de la superficie no escribe ni lee.
        let (orig, mut s) = (make(), make());
        s.copy_rect(-2, -1, 1, 1, 20, 20);
        assert_eq!(px(&s, 3, 2), px(&orig, 0, 0));
        s.copy_rect(0, 0, 100, 100, 4, 4);
    }

    #[test]
    fn same_storage_survives_nested_core_render_calls_and_fallback_memories() {
        let mut page=PageMem::default();
        with_core_mem(&mut page,|core| {
            core.w32(0xffe,0x12345678);
            let mut render=CoreMem(core);
            with_core_mem(&mut render,|nested| {
                assert_eq!(nested.r32(0xffe),0x12345678);
                nested.w16(0x1000,0x9abc);
            });
        });
        assert_eq!(page.u32_at(0xffe),0x9abc5678);
        let data:Vec<u8>=(0..10000).map(|i| (i*73) as u8).collect();
        LAST.with(|last| last.set(0x1357));
        with_core_mem(&mut page,|core| {
            core.write_bytes(u32::MAX-4000,&data);
            let mut render=CoreMem(core);
            let mut output=vec![0xcc;data.len()];
            p3_render::scene_math::Mem::read_bytes(&mut render,u32::MAX-4000,&mut output);
            assert_eq!(output,data);
        });
        assert_eq!(LAST.with(|last| last.get()),0x1357,"bulk byte writes must not impersonate a stack w32");
        let mut missing=[0xcc;32];
        p3_core::flatmem::Mem::read_bytes(&mut page,0x30000,&mut missing);
        assert_eq!(missing,[0;32]);
        // A foreign render memory need not implement or expose the core trait.
        struct Foreign([u8;8]);
        impl p3_render::scene_math::Mem for Foreign {
            fn r8(&mut self,a:u32)->u8 {self.0[a as usize]}
            fn w8(&mut self,a:u32,v:u8) {self.0[a as usize]=v;}
        }
        let mut other=Foreign([0;8]);
        with_core_mem(&mut other,|core| {
            core.w32(1,0x91e7ab23);
            assert_eq!(core.r32(1),0x91e7ab23);
            let mut bytes=[0;4];
            core.read_bytes(1,&mut bytes);
            assert_eq!(bytes,[0x23,0xab,0xe7,0x91]);
            core.write_bytes(5,&[13,14,15]);
        });
        assert_eq!(other.0,[0,0x23,0xab,0xe7,0x91,13,14,15]);
    }
}
impl p3_core::flatmem::Mem for RenderMem<'_> {
    fn read_bytes(&mut self, a:u32, out:&mut [u8]) { self.0.read_bytes(a,out); }
    fn write_bytes(&mut self, a:u32, data:&[u8]) { self.0.write_bytes(a,data); }
    fn cmp_cstr(&mut self, a: u32, b: u32, fold: bool, max: usize) -> Option<std::cmp::Ordering> { self.0.cmp_cstr(a, b, fold, max) }
    fn r8(&mut self, a: u32) -> u8 {
        self.0.r8(a)
    }
    fn r16(&mut self, a: u32) -> u16 { self.0.r16(a) }
    fn r32(&mut self, a: u32) -> u32 { self.0.r32(a) }
    fn w8(&mut self, a: u32, v: u8) {
        self.0.w8(a, v)
    }
    fn w16(&mut self, a: u32, v: u16) { self.0.w16(a, v) }
    fn w32(&mut self, a: u32, v: u32) {
        self.0.w32(a, v)
    }
}

/// `0x004445A0` `is_line_clear(grid, &ancho, fin, inicio)` sobre la memoria emulada: misma lógica que
/// `p3_core::route::planner::NavGrid::is_line_clear` (probada contra el x86 en 24.000 rectas), pero leyendo las celdas de la
/// rejilla del original en lugar de copiarlas. `fin` e `inicio` apuntan a pares `(x, y)` de `i16`. Si la recta es libre suma uno al
/// contador global `0x006CBDC4`. Sustituye al elevado (`fix_lifted::f_004445a0`), que pesaba ~8.000 instrucciones por llamada.
pub fn is_line_clear_mem(m: &mut dyn p3_render::scene_math::Mem, grid: u32, width_ptr: u32, end_ptr: u32, start_ptr: u32) -> bool {
    const WALL_X: [i32; 15] = [187, 214, 232, 221, 281, 282, 308, 322, 350, 406, 407, 463, 468, 471, 621];
    const WALL_X_LO: [i32; 15] = [357, 0, 316, 200, 249, 304, 328, 0, 280, 320, 168, 153, 0, 187, 22];
    const WALL_X_HI: [i32; 15] = [471, 132, 471, 354, 296, 319, 471, 274, 289, 471, 204, 180, 78, 471, 471];
    const WALL_Y: [i32; 13] = [80, 82, 132, 133, 141, 202, 234, 271, 274, 285, 285, 346, 409];
    const WALL_Y_LO: [i32; 13] = [0, 167, 0, 472, 452, 294, 190, 260, 0, 346, 236, 0, 107];
    const WALL_Y_HI: [i32; 13] = [50, 420, 56, 639, 472, 387, 248, 291, 49, 355, 256, 73, 639];
    fn wall_blocks(fixed: i32, lo: i32, hi: i32, (s1, t1): (i32, i32), (s2, t2): (i32, i32)) -> bool {
        if (fixed < s1 && fixed < s2) || (fixed > s1 && fixed > s2) {
            return false;
        }
        if (t1 < lo && t2 < lo) || (t1 > hi && t2 > hi) {
            return false;
        }
        let (ds, dt) = (s2 - s1, t2 - t1);
        let num = (fixed - s1).wrapping_mul(dt).wrapping_add(ds >> 1);
        if num < (lo - t1).wrapping_mul(ds) {
            return false;
        }
        num <= (hi - t1).wrapping_mul(ds)
    }
    let (x1, y1) = (i32::from(m.r16(start_ptr) as i16), i32::from(m.r16(start_ptr.wrapping_add(2)) as i16));
    let (x2, y2) = (i32::from(m.r16(end_ptr) as i16), i32::from(m.r16(end_ptr.wrapping_add(2)) as i16));
    if x1 == x2 && y1 == y2 {
        return true;
    }
    if x1 != x2 && (0..15).any(|i| wall_blocks(WALL_X[i], WALL_X_LO[i], WALL_X_HI[i], (x1, y1), (x2, y2))) {
        return false;
    }
    if y1 != y2 && (0..13).any(|i| wall_blocks(WALL_Y[i], WALL_Y_LO[i], WALL_Y_HI[i], (y1, x1), (y2, x2))) {
        return false;
    }
    let (dx, dy) = (x2 - x1, y2 - y1);
    let (adx, ady) = (dx.abs(), dy.abs());
    let (sx, sy) = (dx.signum(), dy.signum());
    let w = m.r32(width_ptr) as i32;
    let mut pos = y1.wrapping_mul(w).wrapping_add(x1);
    let step_y = w.wrapping_mul(sy);
    let mut occupied = |m: &mut dyn p3_render::scene_math::Mem, p: i32| m.r8(grid.wrapping_add(p as u32)) != 0;
    // Eje mayor (`major`) y menor: el paso mayor avanza siempre `step_major`, el menor `step_minor` cuando el error desborda.
    let (limit, big, small, step_major, step_minor, mut err) =
        if adx >= ady { (adx - 1, 2 * adx, 2 * ady, sx, step_y, adx) } else { (ady - 1, 2 * ady, 2 * adx, step_y, sx, ady) };
    for _ in 0..limit.max(0) {
        pos = pos.wrapping_add(step_major);
        err += small;
        if err < big {
            if occupied(m, pos) {
                return false;
            }
            continue;
        }
        err -= big;
        if err == 0 {
            if !occupied(m, pos) {
                pos = pos.wrapping_add(step_minor);
                continue;
            }
            pos = pos.wrapping_add(step_minor);
            if occupied(m, pos) {
                return false;
            }
            continue;
        }
        pos = pos.wrapping_add(step_minor);
        if occupied(m, pos) {
            return false;
        }
    }
    let n = m.r32(0x006C_BDC4);
    m.w32(0x006C_BDC4, n.wrapping_add(1));
    true
}

/// `0x00448450` `NavGrid::snap_to_sea(this = ECX, punto: *i16[2], radio)` (`ret 8`) sobre la memoria emulada: misma lógica que
/// `p3_core::route::planner::NavGrid::snap_to_sea` (probada contra el x86), leyendo la rejilla del objeto (`+0x14` celdas, `+0x18` ancho
/// y `+0x1A` alto, `i16`). Sustituye al elevado `fix_lifted::f_00448450`.
pub fn snap_to_sea_mem(m: &mut dyn p3_render::scene_math::Mem, this: u32, point_ptr: u32, radius: u32) -> bool {
    let (x, y) = (i32::from(m.r16(point_ptr) as i16), i32::from(m.r16(point_ptr.wrapping_add(2)) as i16));
    let (w, h) = (i32::from(m.r16(this.wrapping_add(0x18)) as i16), i32::from(m.r16(this.wrapping_add(0x1A)) as i16));
    if x < 0 || y < 0 || x > w || y > h {
        return false;
    }
    let cells = m.r32(this.wrapping_add(0x14));
    let cell = |m: &mut dyn p3_render::scene_math::Mem, x: i32, y: i32| m.r8(cells.wrapping_add(y.wrapping_mul(w).wrapping_add(x) as u32));
    if cell(m, x, y) == 0 {
        return true;
    }
    let radius = (radius & 0xFF) as i32;
    if radius < 1 {
        return false;
    }
    let mut best = i32::MAX;
    let mut found = (0i32, 0i32);
    let mut k = 1i32;
    loop {
        let mut dx = -k;
        while dx <= k {
            let mut dy = -k;
            while dy <= k {
                let cx = (x + dx) as i16 as i32;
                let cy = (y + dy) as i16 as i32;
                if cy >= 0 && cx >= 0 && cy < h && cx < w && cell(m, cx, cy) == 0 {
                    let d = dx * dx + dy * dy;
                    if d < best {
                        best = d;
                        found = (cx, cy);
                    }
                }
                dy = if dx != -k && dx != k && dy != k { k } else { dy + 1 };
            }
            dx += 1;
        }
        k += 1;
        if best <= k * k {
            break;
        }
        if k > radius {
            if best >= i32::MAX {
                return false;
            }
            break;
        }
    }
    m.w16(point_ptr, found.0 as u16);
    m.w16(point_ptr.wrapping_add(2), found.1 as u16);
    true
}

fn cstr(m: &mut dyn p3_render::scene_math::Mem, a: u32) -> Vec<u8> {
    cstr_max(m, a, 0x10000)
}

/// Como [`cstr`] pero lee como mucho `max` bytes (las comparaciones acotadas de las cadenas largas no las copian enteras).
fn cstr_max(m: &mut dyn p3_render::scene_math::Mem, a: u32, max: usize) -> Vec<u8> {
    let mut out = Vec::new();
    let mut at = a;
    while out.len() < max {
        let b = m.r8(at);
        if b == 0 {
            break;
        }
        out.push(b);
        at = at.wrapping_add(1);
    }
    out
}

/// Compara dos cadenas C de la memoria emulada como lo hacía `cstr(a).cmp(&cstr(b))` (bytes sin signo, la más corta antes, tope de
/// `0x10000` bytes por cadena), con o sin plegado ASCII a minúsculas, pero sin copiarlas: lee por tramos y para en la primera
/// diferencia. `_stricmp` se llama millones de veces al abrir la ciudad (búsqueda lineal de la caché de texturas `0x54F290`).
fn cmp_cstr(m: &mut dyn p3_render::scene_math::Mem, a: u32, b: u32, fold: bool) -> std::cmp::Ordering {
    if let Some(order) = m.cmp_cstr(a, b, fold, 0x10000) {
        return order;
    }
    const STEP: usize = 32;
    let (mut x, mut y) = ([0u8; STEP], [0u8; STEP]);
    let mut done = 0usize;
    while done < 0x10000 {
        m.read_bytes(a.wrapping_add(done as u32), &mut x);
        m.read_bytes(b.wrapping_add(done as u32), &mut y);
        for i in 0..STEP {
            let (mut p, mut q) = (x[i], y[i]);
            if p == 0 || q == 0 {
                // Fin de una de las dos: la que acaba antes es menor (prefijo); si acaban a la vez, iguales.
                return (p != 0).cmp(&(q != 0));
            }
            if fold {
                p = p.to_ascii_lowercase();
                q = q.to_ascii_lowercase();
            }
            if p != q {
                return p.cmp(&q);
            }
        }
        done += STEP;
    }
    std::cmp::Ordering::Equal
}

/// `_cfltcvt` del CRT de MSVC: `e`/`E` con exponente de tres cifras, `f` fija y `g`/`G` (precisión 0 = 1) sin quitar ceros
/// (eso lo hace `_cropzeros`). Infinitos y NaN como el CRT (`1.#INF`, `-1.#IND`, `1.#QNAN`).
fn crt_cfltcvt(v: f64, fmt: u8, prec: usize) -> String {
    if !v.is_finite() {
        let sign = if v.is_sign_negative() { "-" } else { "" };
        return format!("{sign}{}", if v.is_infinite() { "1.#INF" } else if v.is_sign_negative() { "1.#IND" } else { "1.#QNAN" });
    }
    let exp_form = |v: f64, prec: usize| {
        let s = format!("{:.*e}", prec, v);
        let (mant, exp) = s.split_once('e').unwrap();
        let e: i32 = exp.parse().unwrap();
        format!("{mant}e{}{:03}", if e < 0 { '-' } else { '+' }, e.abs())
    };
    match fmt {
        b'e' | b'E' => exp_form(v, prec),
        b'g' | b'G' => {
            let p = prec.max(1);
            if v == 0.0 {
                return format!("{:.*}", p - 1, v);
            }
            // Exponente tras redondear a `p` cifras significativas.
            let e: i32 = format!("{:.*e}", p - 1, v).split_once('e').unwrap().1.parse().unwrap();
            if e < -4 || e >= p as i32 {
                exp_form(v, p - 1)
            } else {
                format!("{:.*}", (p as i32 - 1 - e).max(0) as usize, v)
            }
        }
        _ => format!("{:.*}", prec, v),
    }
}

/// `atof` del CRT: prefijo numérico válido (espacios, signo, cifras, punto, exponente); el resto se ignora.
fn crt_atof(s: &str) -> f64 {
    let t = s.trim_start();
    let b = t.as_bytes();
    let mut end = 0;
    let mut best = 0;
    if end < b.len() && (b[end] == b'+' || b[end] == b'-') {
        end += 1;
    }
    let mut digits = false;
    while end < b.len() && b[end].is_ascii_digit() {
        end += 1;
        digits = true;
        best = end;
    }
    if end < b.len() && b[end] == b'.' {
        end += 1;
        while end < b.len() && b[end].is_ascii_digit() {
            end += 1;
            digits = true;
            best = end;
        }
        if digits {
            best = end;
        }
    }
    if digits && end < b.len() && (b[end] == b'e' || b[end] == b'E') {
        let mut e = end + 1;
        if e < b.len() && (b[e] == b'+' || b[e] == b'-') {
            e += 1;
        }
        if e < b.len() && b[e].is_ascii_digit() {
            while e < b.len() && b[e].is_ascii_digit() {
                e += 1;
            }
            best = e;
        }
    }
    t[..best].parse().unwrap_or(0.0)
}

fn write_bytes(m: &mut dyn p3_render::scene_math::Mem, at: u32, b: &[u8]) {
    for (i, v) in b.iter().enumerate() {
        m.w8(at.wrapping_add(i as u32), *v);
    }
}

/// Lee una cadena de un `CString` de MFC (`[this]` = datos).
fn cstring_get(m: &mut dyn p3_render::scene_math::Mem, this: u32) -> Vec<u8> {
    let data = m.r32(this);
    if data == 0 { Vec::new() } else { cstr(m, data) }
}

impl<'a> Runtime<'a> {
    /// Datos nuevos de `CString` (cabecera `-12`: referencias, longitud, reservado) con `s`; los guarda en `[this]`.
    fn cstring_set(&mut self, m: &mut dyn p3_render::scene_math::Mem, this: u32, s: &[u8]) {
        let n = s.len() as u32;
        let at = self.heap;
        self.heap = self.heap.wrapping_add((n + 13 + 15) & !15);
        m.w32(at, 1);
        m.w32(at + 4, n);
        m.w32(at + 8, n);
        write_bytes(m, at + 12, s);
        m.w8(at + 12 + n, 0);
        m.w32(this, at + 12);
    }

    /// `sprintf` del CRT: formato en `fmt`, argumentos variables desde `[first]`.
    fn printf(m: &mut dyn p3_render::scene_math::Mem, fmt: u32, first: u32, table: Option<&HashMap<u32, Vec<u8>>>) -> Vec<u8> {
        let lit = |m: &mut dyn p3_render::scene_math::Mem, p: u32| table.and_then(|t| t.get(&p).cloned()).unwrap_or_else(|| cstr(m, p));
        let f = lit(m, fmt);
        let words: Vec<u32> = (0..64).map(|i| m.r32(first + 4 * i)).collect();
        let mut k = 0usize;
        crt::format(&f, || {
            k += 1;
            words.get(k - 1).copied().unwrap_or(0)
        }, |p| lit(m, p))
    }
}

impl<'a> Runtime<'a> {
    /// Memoria con el EXE, ranuras SGL, fuentes y gestor de texturas sembrados; superficie `w × h`.
    pub fn new(mem: &mut PageMem, exe: &[u8], archive: &'a Archive<'a>, width: usize, height: usize) -> Result<Self, String> {
        mem.load_pe(exe)?;
        grp_lib_native::register_exe(exe);
        let mut assets = Assets::load(archive)?;
        // Las fuentes las crea el propio arranque (`init_fonts`), no la lectura directa de `fonts.ini`.
        for h in assets.fonts.keys().copied().collect::<Vec<_>>() {
            assets.images.remove(&h);
            assets.defs.remove(&h);
        }
        assets.fonts.clear();
        let mut rt = Self {
            assets,
            surface: Surface::new(width, height, [0, 0, 0, 0]),
            heap: HEAP_BASE,
            presents: 0,
            abort_logged: false,
            heap_big: false,
            sp: STACK_TOP,
            missing: BTreeMap::new(),
            ported: BTreeMap::new(),
            operaciones_guion: Default::default(),
            profile: std::env::var_os("P3_UI_PROFILE").map(|_| BTreeMap::new()),
            profile_children: Vec::new(),
            profile_from_frame: std::env::var("P3_UI_PROFILE_FROM").ok().and_then(|s| s.parse().ok()).unwrap_or(0),
            depth: 0,
            stack: Vec::new(),
            esps: Vec::new(),
            force_frame: None,
            clip: Default::default(),
            render_dest: 0,
            target_tex: None,
            screen_stash: None,
            front: Surface::new(0, 0, [0, 0, 0, 255]),
            front_fresh: false,
            cursor_under: Vec::new(),
            front_direct: false,
            screen_is_front: false,
            canvas_active: false,
            two_buffers: std::env::var_os("P3_ONE_BUFFER").is_none(),
            screen_locks: HashMap::new(),
            screen_locked: None,
            ported_hot: U32Map::default(),
            low_memory: std::env::var_os("P3_LOW_MEMORY").is_some(),
            screen_lock_live: None,
            missing_from: BTreeMap::new(),
            alloc_log: None,
            arg_mismatch: BTreeMap::new(),
            sgl_missing: Default::default(),
            tex_store: Default::default(),
            mem_tex: Default::default(),
            loaded_tex: Default::default(),
            calls: 0,
            call_limit: 2_000_000,
            missing_log: std::env::var_os("P3_MISSING").is_some(),
            diag: None,
            heap_blocks: U32Map::default(),
            ini_cache: HashMap::new(),
            sgl_log: std::env::var_os("P3_SGL_LOG").is_some(),
            watch_mem: std::env::var("P3_WATCHMEM").ok().and_then(|v| {
                let h = |x: &str| u32::from_str_radix(x.trim().trim_start_matches("0x"), 16).ok();
                match v.strip_prefix('*') {
                    Some(r) => {
                        let (b, o) = r.split_once('+').unwrap_or((r, "0"));
                        Some((true, h(b)?, h(o)?))
                    }
                    None => Some((false, h(&v)?, 0)),
                }
            }),
            watch_mem_last: 0,
            watch: std::env::var("P3_WATCH").unwrap_or_default().split(',').filter_map(|x| u32::from_str_radix(x.trim().trim_start_matches("0x"), 16).ok()).collect(),
            heap_free_lists: U32Map::default(),
            heap_free_big: std::collections::BTreeSet::new(),
            heap_free_big_at: std::collections::BTreeMap::new(),
            prof: std::env::var("P3_PROF").ok().and_then(|v| v.parse().ok()).unwrap_or(0),
            aborted: false,
            trace: std::env::var_os("P3_UI_TRACE").is_some(),
            texts: Vec::new(),
            core: core_table::CORE.iter().chain(lib_lifted::LIB.iter()).chain(fix_lifted::FIX.iter()).chain(ship_panel_lifted::SHIPS.iter()).chain(names_lifted::NAMES.iter()).chain(newgame_lifted::NEWGAME.iter()).chain(script_lifted::SCRIPT.iter()).chain(display_lifted::DISPLAY.iter()).chain(ship_lifted::SHIP.iter()).chain(trade_lifted::TRADE.iter()).chain(shipyard_lifted::SHIPYARD.iter()).chain(ship_orders_lifted::SHIP_ORDERS.iter()).chain(church_lifted::CHURCH.iter()).chain(ayto_lifted::AYTO.iter()).chain(hanse_lifted::HANSE.iter()).chain(lender_lifted::LENDER.iter()).chain(barn_lifted::BARN.iter()).chain(office_lifted::OFFICE.iter()).chain(tavern_lifted::TAVERN.iter()).chain(grp_lib_lifted::GRP_LIB.iter()).chain(grp_lib_native::GRP_LIB_NATIVE.iter()).chain(grp_sea_lifted::GRP_SEA.iter()).chain(grp_ui_lifted::GRP_UI.iter()).chain(grp_ops_lifted::GRP_OPS.iter()).chain(grp_bat_lifted::GRP_BAT.iter()).chain(grp_tk_lifted::GRP_TK.iter()).chain(grp_cy_lifted::GRP_CY.iter()).chain(grp_cy2_lifted::GRP_CY2.iter()).chain(grp_bat2_lifted::GRP_BAT2.iter()).chain(grp_bat2_native::GRP_BAT2_NATIVE.iter()).chain(grp_lib2_lifted::GRP_LIB2.iter()).chain(grp_lib2_native::GRP_LIB2_NATIVE.iter()).chain(grp_snd_lifted::GRP_SND.iter()).chain(grp_rain_lifted::GRP_RAIN.iter()).chain(grp_sie_lifted::GRP_SIE.iter()).chain(activities_lifted::ACTIVITIES.iter()).chain(grp_nav_lifted::GRP_NAV.iter()).chain(grp_tat_lifted::GRP_TAT.iter()).chain(grp_sail_lifted::GRP_SAIL.iter()).chain(grp_bld_lifted::GRP_BLD.iter()).copied().collect(),
            hand: hand_table::HAND.iter().copied().collect(),
            sgl: U32Map::default(),
            imports: U32Map::default(),
            sgl_archive: 0,
            next_handle: 0x7000_0000,
            files: U32Map::default(),
            next_file: 0x100,
            mapped: HashMap::new(),
            import_missing: Default::default(),
            miles: miles::MilesState::from_env(),
            vto: vto::VtoState::from_env(),
            menus: Default::default(),
            messages: Default::default(),
            input: Default::default(),
            on_present: None,
            on_pump: None,
            on_create_process: None,
            display: Default::default(),
            video_options: Default::default(),
            language: Default::default(),
            zoom: Default::default(),
        };
        for (name, slot) in sgl_exports(mem) {
            mem.set_u32(slot, SGL_BASE + slot);
            rt.sgl.insert(SGL_BASE + slot, name.into());
        }
        for (iat, name) in PageMem::pe_imports(exe) {
            mem.set_u32(iat, IMPORT_BASE + iat);
            rt.imports.insert(IMPORT_BASE + iat, name.into());
        }
        rt.static_initializers(mem);
        rt.seed_textures(mem);
        rt.init_fonts(mem);
        Ok(rt)
    }

    /// Inicializadores estáticos del CRT (`__xc_a..__xc_z`, `0x00692004..0x0069288C`) que solo copian dobles palabras entre
    /// globales (`mov reg,[imm]` / `mov [imm],reg|imm` / `mov reg,imm` / `ret`): p. ej. `0x004C1050` deja en `0x006DD59C` el
    /// tipo de fuente por defecto. Los que construyen objetos o registran `atexit` no se ejecutan.
    fn static_initializers(&mut self, mem: &mut PageMem) -> usize {
        let mut done = 0;
        for slot in (0x0069_2004..0x0069_288C).step_by(4) {
            let f = mem.u32_at(slot);
            let mut regs = [0u32; 8];
            let mut writes = Vec::new();
            // `mov ecx, objeto; call ctor; push dtor; call atexit; pop ecx; ret`: objeto global construido con su constructor
            // (solo los de [`PORTED_CTORS`]: la lista de pantallas `0x006DA5F0` con `0x004B8740`). `atexit` no se registra.
            if mem.u8_at(f) == 0xB9 && mem.u8_at(f + 5) == 0xE8 && mem.u8_at(f + 10) == 0x68 && mem.u8_at(f + 15) == 0xE8 {
                let obj = mem.u32_at(f + 1);
                let ctor = (f + 10).wrapping_add(mem.u32_at(f + 6));
                let atexit = (f + 20).wrapping_add(mem.u32_at(f + 16));
                if atexit == 0x0063_978E && PORTED_CTORS.contains(&ctor) {
                    self.call(mem, ctor, obj, &[]);
                    done += 1;
                }
                continue;
            }
            let mut pc = f;
            let ok = loop {
                let op = mem.u8_at(pc);
                let imm = |o: u32| mem.u32_at(pc + o);
                match op {
                    0xA1 => {
                        regs[0] = mem.u32_at(imm(1));
                        pc += 5;
                    }
                    0xA3 => {
                        writes.push((imm(1), regs[0]));
                        pc += 5;
                    }
                    0xB8..=0xBF => {
                        regs[usize::from(op - 0xB8)] = imm(1);
                        pc += 5;
                    }
                    0x8B | 0x89 if mem.u8_at(pc + 1) & 0xC7 == 0x05 => {
                        let r = usize::from((mem.u8_at(pc + 1) >> 3) & 7);
                        let at = mem.u32_at(pc + 2);
                        if op == 0x8B {
                            regs[r] = mem.u32_at(at);
                        } else {
                            writes.push((at, regs[r]));
                        }
                        pc += 6;
                    }
                    0xC7 if mem.u8_at(pc + 1) == 0x05 => {
                        writes.push((mem.u32_at(pc + 2), mem.u32_at(pc + 6)));
                        pc += 10;
                    }
                    0xC3 => break true,
                    _ => break false,
                }
            };
            if ok {
                for (at, v) in writes {
                    mem.set_u32(at, v);
                }
                done += 1;
            }
        }
        done
    }

    fn alloc(&mut self, mem: &mut PageMem, size: u32) -> u32 {
        let at = self.heap;
        self.heap = self.heap.wrapping_add((size.max(1) + 15) & !15);
        mem.write(at, &vec![0; size as usize]);
        at
    }

    /// Fuentes `SGLFont` `0x006DCD28 + 0xA8·N` como en el arranque: constructor `0x004C1060` y carga `0x004C1230` (ranura 2,
    /// lee `fonts.ini` con la caché INI y crea la fuente de la DLL con `sgl_MemoryFont_Create*`).
    /// `PostMessage` a la ventana principal (hora y cursor de [`Runtime::input`]).
    pub fn post_message(&mut self, message: u32, wparam: u32, lparam: u32) {
        win32::post(&mut self.messages, &self.input, message, wparam, lparam);
    }

    /// Rellena el espejo de la pantalla bloqueada (32 bits B, G, R, A) justo antes de que `water_ripple_render_refraction`
    /// (`0x59E8E0`) lo lea, y solo en la caja que tocan sus ondas. La llamada recibe `(pixels, x, y, clip, pitch)`; se usan
    /// los desplazamientos y el recorte, no el puntero inicial. Se añade margen para las lecturas desplazadas por las ondas.
    /// La pantalla entera costaba ~11 ms por fotograma a 2560x1440.
    fn screen_lock_fill(&mut self, m: &mut dyn p3_render::scene_math::Mem, this: u32, args: &[u32]) {
        let Some((id, buf)) = self.screen_locked else { return };
        if self.screen_lock_live.is_some() || args.len() < 4 {
            return;
        }
        let (w, h) = (self.surface.width, self.surface.height);
        let Some(t) = self.tex_store.get_mut(id) else { return };
        if usize::from(t.width) != w || usize::from(t.height) != h {
            return;
        }
        let full = std::env::var_os("P3_LOCK_FULL").is_some();
        let bb = if full { Some([0, 0, w as i32, h as i32]) } else { screen_lock_bounds(m, this, args, w, h) };
        let Some(bb) = bb else {
            self.screen_lock_live = Some([0; 4]);
            return;
        };
        let [l, t0, r, b] = bb.map(|v| v as usize);
        let pitch = t.pitch as usize;
        for y in t0..b {
            let row = &mut t.pixels[y * pitch + l * 4..y * pitch + r * 4];
            for (d, p) in row.chunks_exact_mut(4).zip(self.surface.rgba[(y * w + l) * 4..(y * w + r) * 4].chunks_exact(4)) {
                d.copy_from_slice(&[p[2], p[1], p[0], p[3]]);
            }
            m.write_bytes(buf + (y * pitch + l * 4) as u32, row);
        }
        self.screen_lock_live = Some(bb);
    }

    /// Pone en [`Runtime::surface`] el búfer de pantalla que pide el destino `d` (`-3` delantero, `-4` el vigente, el resto
    /// trasero) intercambiando los píxeles con el otro. No actúa mientras el destino es una textura o el lienzo del zoom.
    fn screen_buffer(&mut self, d: u32) {
        if !self.two_buffers || self.target_tex.is_some() || self.canvas_active || d == 0xFFFF_FFFC {
            return;
        }
        let front = d == 0xFFFF_FFFD;
        if front {
            self.front_fresh = true;
        }
        if front != self.screen_is_front {
            if self.front.rgba.len() != self.surface.rgba.len() {
                self.front.rgba.clone_from(&self.surface.rgba);
            }
            std::mem::swap(&mut self.surface.rgba, &mut self.front.rgba);
            self.screen_is_front = front;
        }
    }

    /// Uno o dos búferes de pantalla (como `P3_ONE_BUFFER`); para comparar en pruebas. Llamar antes del primer fotograma.
    pub fn set_two_buffers(&mut self, on: bool) {
        self.two_buffers = on;
    }

    /// Captura de pantalla de Ctrl+S (`sgl_GrabScreen`): BMP de 24 bits en `grabs/grabNNNN.bmp` (primer número libre).
    fn grab_screen(&mut self) {
        let Some(dir) = self.menus.host_path("grabs/x").and_then(|p| p.parent().map(|d| d.to_path_buf())) else { return };
        let fs = crate::hostfs::current();
        if !fs.exists(&dir) && fs.create_dir_all(&dir).is_err() {
            return;
        }
        let Some(path) = (0..10_000).map(|n| dir.join(format!("grab{n:04}.bmp"))).find(|p| !fs.exists(p)) else { return };
        let (w, h) = (self.surface.width, self.surface.height);
        let _ = fs.write(&path, &bmp24(w, h, &self.surface.rgba));
    }

    /// `sgl_SwapBuffer`: llama al gancho de presentación; si pide salir, encola `WM_QUIT` (una vez).
    fn present(&mut self) {
        let Some(mut hook) = self.on_present.take() else { return };
        let tramo = stall::tag("superposiciones");
        // Se muestra el delantero. Las pantallas que componen en el trasero y presentan sin pasar por `present_frame` no lo
        // tocan: entonces se copia el trasero entero.
        let two = self.two_buffers && self.target_tex.is_none() && !self.canvas_active;
        if two {
            self.screen_buffer(self.render_dest);
            if !self.front_fresh || self.front.rgba.len() != self.surface.rgba.len() {
                if self.screen_is_front {
                    if self.front.rgba.len() == self.surface.rgba.len() {
                        self.surface.rgba.copy_from_slice(&self.front.rgba);
                    }
                } else {
                    self.front.rgba.clone_from(&self.surface.rgba);
                }
            }
            self.front_fresh = self.screen_is_front;
            (self.front.width, self.front.height) = (self.surface.width, self.surface.height);
        }
        if two && std::env::var_os("P3_BUFCHECK").is_some() {
            let w = self.surface.width;
            let (mut n, mut bb) = (0usize, [usize::MAX, usize::MAX, 0, 0]);
            for (i, (a, b)) in self.front.rgba.chunks_exact(4).zip(self.surface.rgba.chunks_exact(4)).enumerate() {
                if a[..3] != b[..3] {
                    n += 1;
                    bb = [bb[0].min(i % w), bb[1].min(i / w), bb[2].max(i % w), bb[3].max(i / w)];
                }
            }
            eprintln!("búferes: {n} píxeles distintos {bb:?} delantero_vigente={} zonas={:?}", self.screen_is_front, self.clip.passes(None));
        }
        self.language.refresh(self.presents);
        let overlay = if !(self.language.enabled && self.language.open) {
            self.language.close_frame();
            None
        } else {
            let base = if two && !self.screen_is_front { &self.front } else { &self.surface };
            let (w, h, rgba) = (base.width, base.height, base.rgba.clone());
            let mut copy = Surface::new(w, h, [0, 0, 0, 0xFF]);
            copy.rgba = rgba;
            self.language_overlay(&copy)
        };
        let shown = match &overlay {
            Some(o) => o,
            None if two && !self.screen_is_front => &self.front,
            None => &self.surface,
        };
        drop(tramo);
        let tramo = stall::tag("al-presentar");
        let keep = hook(shown, &mut self.messages, &mut self.input);
        drop(tramo);
        if two && std::mem::take(&mut self.front_direct) && self.front.rgba.len() == self.surface.rgba.len() {
            if self.screen_is_front {
                self.front.rgba.copy_from_slice(&self.surface.rgba);
            } else {
                self.surface.rgba.copy_from_slice(&self.front.rgba);
            }
        }
        self.restore_under_cursor();
        if stall::ON.load(std::sync::atomic::Ordering::Relaxed) {
            stall::frame_end(self.presents);
        }
        // En vivo el tope de llamadas es por fotograma: el bucle del original no termina nunca.
        self.calls = 0;
        self.presents += 1;
        thread_local!(static HEAPLOG: Option<u64> = std::env::var("P3_HEAPLOG").ok().and_then(|v| v.parse().ok()));
        if let Some(n) = HEAPLOG.with(|h| *h) {
            if self.presents % n == 0 {
                let free: u64 = self.heap_free_lists.iter().map(|(k, v)| u64::from(*k) * v.len() as u64).sum::<u64>() + self.heap_free_big.iter().map(|(k, _)| u64::from(*k)).sum::<u64>();
                eprintln!("montón: fotograma {} cima {:#x} ({} MiB) bloques vivos {} libres {} KiB", self.presents, self.heap, (self.heap - HEAP_BASE) >> 20, self.heap_blocks.len(), free >> 10);
            }
        }
        self.on_present = Some(hook);
        if !keep && !self.messages.iter().any(|m| m.message == win32::WM_QUIT) {
            self.post_message(win32::WM_QUIT, 0, 0);
        }
    }

    /// Modo de poca memoria (WEB-6, móviles): los bloques grandes que el juego libera devuelven sus páginas en lugar de quedar
    /// reservados hasta que alguien pida otro del mismo tamaño (al entrar en una ciudad quedaban así unos 50 MiB). No cambia lo
    /// que se dibuja en los bancos; sí el contenido de la memoria ya liberada (ceros), así que el hash de memoria no es
    /// comparable con el de una ejecución sin este modo. Apagado por defecto; `P3_LOW_MEMORY` lo enciende en nativo.
    /// Enciende el muestreo de tirones ([`stall`]): cada fotograma de más de 150 ms escribe en stderr (`p3-juego.log`) qué
    /// funciones del original lo ocupaban. Solo para el cliente en vivo.
    pub fn enable_stall_sampler(&mut self) {
        stall::enable();
    }

    pub fn set_low_memory(&mut self, on: bool) {
        self.low_memory = on;
    }

    /// Reparto de la memoria que retiene el enlazador, en bytes (diagnóstico de WEB-6; no cambia nada):
    /// páginas de la memoria emulada, imágenes RGBA de `assets`, píxeles de `tex_store`, superficies de pantalla.
    pub fn memory_report(&self, mem: &PageMem) -> [(&'static str, usize); 12] {
        let images: usize = self.assets.images.values().map(|i| i.rgba.capacity()).sum();
        let mirrors: usize = self.mem_tex.values().filter_map(|&(id, _, _)| self.tex_store.get(id)).map(|t| t.pixels.len()).sum();
        [
            ("guest_pages", mem.page_count() * PAGE as usize),
            ("guest_heap_top", (self.heap - HEAP_BASE) as usize),
            ("guest_heap_free_lists", self.heap_free_lists.iter().map(|(k, v)| *k as usize * v.len()).sum::<usize>() + self.heap_free_big.iter().map(|(k, _)| *k as usize).sum::<usize>()),
            ("guest_heap_free_big", self.heap_free_big.iter().map(|(k, _)| *k as usize).sum()),
            ("guest_mapped_files", self.mapped.values().map(|v| v.1 as usize).sum()),
            ("cpr_files_requested", self.assets.requested_bytes()),
            ("images_rgba", images),
            ("images_rgba_evictable", self.assets.evictable_bytes()),
            ("tex_store", self.tex_store.pixel_bytes()),
            ("tex_store_mirrored_in_guest", mirrors),
            ("surfaces", self.surface.rgba.capacity() + self.front.rgba.capacity() + self.screen_stash.as_ref().map_or(0, |s| s.2.capacity())),
            ("files_open", self.files.values().map(|f| f.0.capacity()).sum()),
        ]
    }

    pub fn init_fonts(&mut self, mem: &mut PageMem) {
        let count = self.assets.font_sections();
        for n in 0..count {
            // `0x0046398E`: el nombre del INI va por valor como `CString` y después el número de fuente.
            let file = self.alloc(mem, 4);
            let path = b"scripts\\fonts.ini".to_vec();
            self.cstring_set(mem, file, &path);
            let file = mem.u32_at(file);
            let obj = 0x006D_CD28 + 0xA8 * n;
            self.call(mem, 0x004C_1060, obj, &[]);
            let load = mem.u32_at(mem.u32_at(obj) + 8);
            self.call(mem, load, obj, &[file, n]);
        }
    }

    /// Gestor de texturas `0x006DA820` (`0x004B3DD0`: `+4` objetos, `+8` identificadores, `+0x10` cuántas) con todas las de
    /// `textures.ini`; cada objeto lleva en `+4` el manejador (el identificador). Iconos en línea del texto (`0x0042EDB7`).
    fn seed_textures(&mut self, mem: &mut PageMem) {
        let mut ids: Vec<u32> = self.assets.defs.keys().copied().filter(|id| !self.assets.fonts.contains_key(id)).collect();
        ids.sort_unstable();
        let n = ids.len() as u32;
        let objs = self.alloc(mem, 4 * n);
        let tab = self.alloc(mem, 4 * n);
        let mut by_id = HashMap::new();
        for (i, id) in ids.iter().enumerate() {
            let obj = self.alloc(mem, 0x20);
            mem.set_u32(obj + 4, *id);
            mem.set_u32(objs + 4 * i as u32, obj);
            mem.set_u32(tab + 4 * i as u32, *id);
            by_id.insert(*id, obj);
        }
        mem.set_u32(0x006D_A820 + 4, objs);
        mem.set_u32(0x006D_A820 + 8, tab);
        mem.set_u32(0x006D_A820 + 0x10, n);
        for (global, id) in [(0x006C_C37C, 0x3E8E), (0x006C_C380, 0x3E9E), (0x006C_C384, 0x3E9F)] {
            if let Some(obj) = by_id.get(&id) {
                mem.set_u32(global, *obj);
            }
        }
    }

    /// Llama a la función del original en `f` con `ecx` y argumentos de pila, y devuelve EAX.
    pub fn call(&mut self, mem: &mut PageMem, f: u32, ecx: u32, args: &[u32]) -> u32 {
        LAST.with(|l| l.set(0));
        if self.depth > 0 {
            return self.dispatch(mem, f, ecx, args);
        }
        // Al agotar `call_limit`, la profundidad o el montón, `dispatch` desenrolla hasta aquí (`AbortRun`): los bucles del código
        // elevado no dependen siempre de lo que devuelven sus llamadas y seguirían girando con ceros. Se restaura la pila de llamadas.
        let (stack, esps, sp) = (self.stack.len(), self.esps.len(), self.sp);
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| self.dispatch(mem, f, ecx, args)));
        for (f, n) in self.ported_hot.drain() {
            *self.ported.entry(f).or_default() += n;
        }
        match result {
            Ok(r) => r,
            Err(e) if e.is::<AbortRun>() => {
                self.stack.truncate(stack);
                self.esps.truncate(esps);
                stall::pop(self.stack.len());
                self.sp = sp;
                self.depth = 0;
                self.profile_children.clear();
                0
            }
            Err(e) => {
                // Un fallo del original (p. ej. la excepción de división de `x86rt`) cierra el cliente: el registro anota antes
                // en qué funciones del original estaba, la última es la que falló (15/11, cierre al salir de una batalla naval).
                let text = e.downcast_ref::<&str>().map(|s| s.to_string()).or_else(|| e.downcast_ref::<String>().cloned()).unwrap_or_default();
                let line = format!("fallo del original «{text}» en {:x?}", &self.stack[self.stack.len().saturating_sub(24)..]);
                if let Some(d) = self.diag.as_mut() {
                    d(line.clone());
                }
                eprintln!("{line}");
                std::panic::resume_unwind(e)
            }
        }
    }

    /// Sonda de inventario (`examples/ui_probe.rs`): deja la pila de llamadas, la pila y el montón como recién creados para que una
    /// función que haya abortado o entrado en pánico no contamine la siguiente.
    pub fn reset_probe(&mut self) {
        self.depth = 0;
        self.stack.clear();
        self.esps.clear();
        self.sp = STACK_TOP;
        self.heap = HEAP_BASE;
        self.aborted = false;
    }

    /// Reserva `size` bytes en el montón del enlazador (como `operator new`).
    pub fn new_object(&mut self, mem: &mut PageMem, size: u32) -> u32 {
        self.alloc(mem, size)
    }

    /// Estilos de botón (`0x006CBE08 + 0xE8 * i`, uno por entrada de `./scripts/buttons.ini`): el arranque (`0x00424DA4..0x00424DD2`) llama
    /// a `button_base_load_layout` (`0x004C6D20`) con el nombre del `.ini`, `[0x006DF31C]` y el índice. Los controles hijos se copian de ellos.
    pub fn load_button_styles(&mut self, mem: &mut PageMem) {
        let count = mem.u8_at(0x0066_DE5A);
        // Inicializador estático `0x00460730`: `eh vector constructor iterator(0x6CBE08, 0xE8, 6, ctor 0x4C6910, dtor 0x4C6A30)`.
        self.call(mem, 0x0063_9E9C, 0, &[0x006C_BE08, 0xE8, 6, 0x004C_6910, 0x004C_6A30]);
        let ini = mem.cstr(0x0066_DE5C);
        // `0x00424DA9`: `push índice; push hueco` y el `CString` se construye en el hueco: `load_layout(CString, índice)`, `ret 8`.
        for i in 0..u32::from(count) {
            let name = self.cstring(mem, &ini);
            self.call(mem, 0x004C_6D20, 0x006C_BE08 + 0xE8 * i, &[name, i]);
        }
    }

    /// Crea la escena global `[0x006E51AC]` (Class22, `0xCBA8` bytes, constructor `0x00586FF0`) como el arranque del juego
    /// (`0x00424DD4`..`0x00424E00`): reserva, construye y guarda el puntero. Las ventanas de edificios la leen al abrirse.
    pub fn create_scene(&mut self, mem: &mut PageMem) -> u32 {
        self.load_button_styles(mem);
        // Texturas del pergamino (`TEXLIST16000` de `./scripts/parchment.ini`: fondos, botón de cerrar...): las registra
        // `parchment_base_load_layout` (`0x00462190`) al cargar las ventanas de pergamino de la pantalla de juego.
        self.call(mem, 0x0046_2190, 0, &[]);
        let scene = self.new_object(mem, 0xCBA8);
        self.call(mem, 0x0058_6FF0, scene, &[]);
        // La escena ocupa la pantalla: posición (`+0x14`) 0 y tamaño (`+0x2C`, `+0x30`) el de la superficie; las ventanas se centran en él.
        let (w, h) = (self.surface.width as u32, self.surface.height as u32);
        mem.set_u32(scene + 0x2C, w);
        mem.set_u32(scene + 0x30, h);
        // Región de recorte inicial: la pantalla entera (el juego la fija al abrir DirectDraw).
        self.clip.add_clip_rect([0, 0, w as i32, h as i32]);
        mem.set_u32(0x006E_51AC, scene);
        scene
    }

    /// `CString` de MFC con `s` (puntero a los datos, cabecera en `-12`), como el que un llamante pasa por valor.
    pub fn cstring(&mut self, mem: &mut PageMem, s: &[u8]) -> u32 {
        let holder = self.alloc(mem, 4);
        self.cstring_set(mem, holder, s);
        mem.u32_at(holder)
    }

    /// Siembra el estado de una partida abierta en las direcciones del original: la imagen de [`p3_sim::Sim::original_memory_image`] y
    /// el factor de venta de la dificultad comercial (`0x006DE3D8+0x64`, que `get_sell_price` lee de `this`).
    pub fn seed_session(&mut self, mem: &mut PageMem, session: &crate::Session) -> Result<usize, String> {
        let image = session.sim.original_memory_image()?;
        self.seed_bytes(mem, &image);
        // Banco de nombres `0x006DDAA0` (ciudades, pila, apellidos): lo carga el original con `0x005121C0` al crear/cargar la partida.
        self.call(mem, 0x0051_21C0, 0x006D_DAA0, &[]);
        // Y, como el cargador (`0x0051203A..0x00512069`), los punteros de nombre de pila y apellido de cada comerciante (`+0xE8`, `+0xE4`):
        // la imagen de `p3-sim` los deja como índices etiquetados, no como direcciones del banco.
        for i in 0..mem.u32_at(0x006D_E4AA) & 0xFFFF {
            let merchant = self.call(mem, 0x0053_03C0, 0x006D_E4A0, &[i]);
            self.call(mem, 0x004F_6420, merchant, &[]);
            self.call(mem, 0x004F_6450, merchant, &[]);
        }
        mem.write(0x006D_E3D8 + 0x64, &session.sim.commercial_sell_factor.to_le_bytes());
        Ok(image.len())
    }

    /// Siembra bytes sueltos (p. ej. [`p3_sim::Sim::original_memory_image`]: el estado de la partida en las direcciones del original).
    pub fn seed_bytes(&mut self, mem: &mut PageMem, bytes: &BTreeMap<u32, u8>) {
        let mut run: Vec<u8> = Vec::new();
        let mut start = 0u32;
        for (&a, &b) in bytes {
            if !run.is_empty() && a != start.wrapping_add(run.len() as u32) {
                mem.write(start, &run);
                run.clear();
            }
            if run.is_empty() {
                start = a;
            }
            run.push(b);
        }
        if !run.is_empty() {
            mem.write(start, &run);
        }
    }

    /// Copia una cadena terminada en cero al montón.
    pub fn string(&mut self, mem: &mut PageMem, s: &[u8]) -> u32 {
        let at = self.alloc(mem, s.len() as u32 + 1);
        mem.write(at, s);
        at
    }

    fn dispatch(&mut self, m: &mut dyn p3_render::scene_math::Mem, f: u32, ecx: u32, args: &[u32]) -> u32 {
        if self.profile.is_none() || self.menus.frames < self.profile_from_frame {
            return self.dispatch_inner(m, f, ecx, args);
        }
        self.profile_children.push(0);
        let start = clock::Instant::now();
        let value = self.dispatch_inner(m, f, ecx, args);
        let elapsed = start.elapsed().as_nanos();
        let children = self.profile_children.pop().unwrap();
        if let Some(parent) = self.profile_children.last_mut() { *parent += elapsed; }
        let row = self.profile.as_mut().unwrap().entry(f).or_default();
        row.0 += 1;
        row.1 += elapsed.saturating_sub(children);
        row.2 += elapsed;
        value
    }

    fn dispatch_inner(&mut self, m: &mut dyn p3_render::scene_math::Mem, f: u32, ecx: u32, args: &[u32]) -> u32 {
        // Llamante de `p3_core::lifted` sin argumentos deducidos: ya empujó el retorno; el llamado usa su pila.
        // Con argumentos: si el llamante elevado ya los dejó en su pila (retorno empujado en la última palabra escrita y los argumentos
        // justo encima, idénticos), el llamado usa esa pila. Un marco nuevo en `self.sp` podía pisar los locales del llamante, que no
        // actualiza `self.sp` (p. ej. `credits_window_load_layout` perdía un local con `0x4BE07F`, la dirección de retorno del llamado).
        // `entity_cell_links_insert_sorted` (0x559A50, cdecl: casilla, identificador, ranura) llama al método virtual +0x30 de la entidad
        // nueva y de la cabeza. El original nunca la llama con una entidad sin registrar (`ents[id] == 0` es una violación de acceso);
        // aquí un sprite sin id (efecto/barco del puerto) llegaba con id 0, la llamada iba a 0 y el orden de la cadena quedaba roto
        // (ciclos en `links_remove`, bucles de millones de vueltas). Sin entidad no hay nada que enlazar.
        if f == 0x559a50 {
            let last = self.last_w32(m);
            let ui = m.r32(0x6e51ac);
            let ents = m.r32(ui.wrapping_add(0x3fc));
            let id = m.r32(last.wrapping_add(8)) & 0xffff;
            let head = m.r32(last.wrapping_add(4));
            let head_id = if head != 0 { m.r32(head) & 0xffff } else { 0 };
            if ents != 0 && (m.r32(ents.wrapping_add(id * 4)) == 0 || (head_id != 0 && m.r32(ents.wrapping_add(head_id * 4)) == 0)) {
                return 0;
            }
        }
        let caller_frame = {
            let last = self.last_w32(m);
            let ret = m.r32(last);
            // La pila viva del llamante queda por debajo de su propio marco de entrada: una última escritura por encima es de un
            // llamado anterior ya terminado (p. ej. un porte a mano que llama dos veces seguidas sin escribir en la pila).
            let top = self.esps.last().copied().unwrap_or(u32::MAX);
            let in_code = last != 0 && last < top && (0x0040_1000..0x0066_A000).contains(&ret);
            if in_code {
                // Solo funciones conocidas: una ranura nula o nativa no dice cuántos argumentos retira la llamada real.
                let real = if (0x0040_1000..0x0066_A000).contains(&f) { p3_render::ret_pop::ret_pop(f) } else { args.len() as u32 };
                if real != args.len() as u32 && !self.sgl.contains_key(&f) && !self.imports.contains_key(&f) {
                    if let Some(caller) = self.stack.last() {
                        self.arg_mismatch.insert((*caller, ret), (real, args.len() as u32));
                    }
                }
            }
            // Con argumentos deducidos, la pila del llamante vale si los tiene justo encima del retorno: así el llamado ve también los
            // que el elevador no dedujo (llamadas virtuales con más argumentos de los contados, p. ej. `set_position(x, y, z)`).
            let same = in_code && args.iter().enumerate().all(|(i, a)| m.r32(last + 4 + 4 * i as u32) == *a);
            // Salto de cola de un elevado (`pop …; mov [esp+4], v; jmp [reg+ranura]`): la última escritura es el argumento
            // reescrito, justo encima del retorno del llamante original. El elevador emite la llamada sin argumentos, así que
            // el llamado debe usar esa pila; con un marco nuevo leía basura. `ship_side_menu_update_mode_controls` (0x492580)
            // termina así con `set_visible(0)` del botón de comercio de armas (`+0x3724`, «Weapons»): con basura quedaba visible
            // encima del dibujo del barco del panel lateral en lugar de sus ranuras de armas.
            let tail = !in_code && args.is_empty() && last != 0 && last.wrapping_sub(4) == top && {
                let r = m.r32(last.wrapping_sub(4));
                r == RET_MAGIC || (0x0040_1000..0x0066_A000).contains(&r)
            };
            if tail {
                Some(last.wrapping_sub(4))
            } else {
                (in_code && same).then_some(last)
            }
        };
        let saved = self.sp;
        // Salto de una función-puente (`jmp [ranura]` de los `sgl_*` o de la IAT): sin argumentos ni retorno nuevos,
        // el destino usa la pila de entrada del puente.
        let thunk_frame = (args.is_empty() && caller_frame.is_none() && (self.sgl.contains_key(&f) || self.imports.contains_key(&f)))
            .then(|| self.esps.last().copied())
            .flatten();
        let forced = self.force_frame.take();
        if let Some(frame) = forced {
            m.w32(frame, RET_MAGIC);
            self.sp = frame.wrapping_sub(64);
        }
        let esp = match forced.or(caller_frame).or(thunk_frame) {
            Some(esp) => esp,
            None => {
                let last = self.last_w32(m);
                if last != 0 && last < self.sp && last > self.sp.wrapping_sub(0x10_0000) {
                    self.sp = last; // por debajo de la pila viva del llamante
                }
                // Y por debajo del marco del llamante con margen para los locales que los portes a mano usan bajo su `esp`.
                if let Some(&top) = self.esps.last() {
                    if top < self.sp.wrapping_add(0x40) && top > self.sp.wrapping_sub(0x10_0000) {
                        self.sp = top.wrapping_sub(0x40);
                    }
                }
                self.sp = self.sp.wrapping_sub(4 * (args.len() as u32 + 1) + 64);
                let esp = self.sp;
                m.w32(esp, RET_MAGIC);
                for (i, a) in args.iter().enumerate() {
                    m.w32(esp + 4 + 4 * i as u32, *a);
                }
                esp
            }
        };
        // Camino corto de `_stricmp`/`_mbsicmp`/`strcmp`: hojas puras que ninguna etapa de `builtin` intercepta antes de su rama.
        // El marco (pila y `self.sp`) ya está puesto igual que por el camino general; aquí solo se ahorra el resto de la
        // contabilidad, que no deja rastro (pilas de diagnóstico que se apilan y desapilan, tablas que no contienen la dirección).
        // Con cualquier diagnóstico activo, o si toca atender la ventana o abortar, se sigue por el camino general.
        if matches!(f, 0x0064_3DB0 | 0x0063_C47B | 0x0063_9CF0)
            && self.prof == 0
            && !self.trace
            && self.watch.is_empty()
            && self.watch_mem.is_none()
            && (self.calls + 1) & 0x3fff != 0
            && self.depth < 400
            && self.calls < self.call_limit
            && self.heap <= HEAP_BASE + 0x0800_0000
        {
            self.calls += 1;
            p3_core::x86rt::FRET.with(|q| q.borrow_mut().clear());
            let (a, b) = (m.r32(esp + 4), m.r32(esp + 8));
            let r = match cmp_cstr(m, a, b, f != 0x0063_9CF0) {
                std::cmp::Ordering::Less => u32::MAX,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Greater => 1,
            };
            self.sp = saved;
            return r;
        }
        let arg = |m: &mut dyn p3_render::scene_math::Mem, i: u32| m.r32(esp + 4 + 4 * i);
        self.depth += 1;
        if stall::ON.load(std::sync::atomic::Ordering::Relaxed) {
            stall::push(self.stack.len(), f);
        }
        self.stack.push(f);
        // `P3_PROF=n`: cada n llamadas imprime la pila de llamadas del original (para localizar bucles que no terminan).
        if self.prof != 0 && self.calls % self.prof == 0 {
            eprintln!("prof {} t={:?} ecx={ecx:#x} pila={:x?}", self.calls, clock::Instant::now(), self.stack);
        }
        // `P3_WATCH=0xA,0xB`: anuncia cada llamada a esas direcciones con `this`, la pila de entrada y quién llama.
        if self.watch.contains(&f) {
            let w: Vec<u32> = (0..6u32).map(|i| m.r32(esp + 4 * i)).collect();
            eprintln!("vigila {f:#x} ecx={ecx:#x} [ecx+94..a0]={:x?} pila={w:x?} desde={:x?}", (m.r32(ecx.wrapping_add(0x94)), m.r32(ecx.wrapping_add(0x98)), m.r32(ecx.wrapping_add(0x9c))), &self.stack[self.stack.len().saturating_sub(6)..]);
        }
        self.esps.push(esp);
        self.calls += 1;
        // Crear el mundo o cargar una campaña puede pasar varios segundos sin
        // SwapBuffer. Windows necesita que se atiendan sus eventos entretanto.
        if self.calls & 0x3fff == 0 {
            if let Some(mut pump) = self.on_pump.take() {
                // Poll de Windows actualiza el reloj del host. El juego conserva
                // el instante del frame: mover su objetivo durante el catch-up
                // puede impedir que una simulación más lenta llegue a él.
                let frame_time = self.input.time_ms;
                let keep = pump(&mut self.messages, &mut self.input);
                self.input.time_ms = frame_time;
                self.on_pump = Some(pump);
                if !keep {
                    m.w8(0x006D_A846, 1);
                    std::panic::resume_unwind(Box::new(AbortRun));
                }
            }
        }
        // Cola de retornos x87: el llamante elevado con `--fpu-exit` recoge con `Cpu::fret` el ST0 que deja el llamado (ver abajo).
        p3_core::x86rt::FRET.with(|q| q.borrow_mut().clear());
        if self.trace {
            let name = self.sgl.get(&f).or_else(|| self.imports.get(&f)).map_or("", |s| &**s);
            eprintln!("{:indent$}{f:#x} {name} ecx={ecx:#x} args={args:x?} esp={esp:#x}", "", indent = self.depth as usize);
        }
        // The old 128 MiB guest-heap watermark predates modern resolutions.
        // A real 4K new game crosses it while creating its larger image planes.
        // Keep a finite guard, with room for high-resolution buffers.
        // El tope no baja nunca: la memoria ya reservada a alta resolución sigue ahí aunque se elija otra más pequeña.
        self.heap_big |= self.display.modern.is_some_and(|(w,h)| w>1280 || h>1024);
        let heap_budget = if self.heap_big { 0x1000_0000 } else { 0x0800_0000 };
        // `water_ripple_render_refraction`: solo toca la pantalla bloqueada si hay ondas (`[this+0x2C] > 0`).
        if f == 0x0059_E8E0 && self.screen_locked.is_some() && m.r32(ecx.wrapping_add(0x2C)) as i32 > 0 {
            self.screen_lock_fill(m, ecx, args);
        }
        let r = if self.depth > 400 || self.calls > self.call_limit || self.heap > HEAP_BASE + heap_budget {
            self.aborted = true;
            // En vivo (ventana con gancho de presentación) se anota siempre: el registro del cliente explica así un cierre (14/11).
            if self.missing_log || (self.on_present.is_some() && !self.abort_logged) {
                self.abort_logged = true;
                if let Some(d) = self.diag.as_mut() {
                    d(format!("aborta: profundidad {} llamadas {} montón {:#x}", self.depth, self.calls, self.heap));
                }
                eprintln!("aborta: profundidad {} llamadas {} montón {:#x} pila={:x?}", self.depth, self.calls, self.heap, &self.stack[self.stack.len().saturating_sub(24)..]);
            }
            *self.missing.entry(0xFFFF_FFFF).or_default() += 1;
            std::panic::resume_unwind(Box::new(AbortRun))
        } else if let Some(r) = self.builtin(m, f, ecx, esp, &arg) {
            r
        } else if let Some(func) = self.core.get(&f).copied() {
            *self.ported_hot.entry(f).or_default() += 1;
            let mut regs = [0u32; 8];
            regs[1] = ecx;
            regs[4] = esp;
            // `class75_load_texture_list` (`0x004B3EA0`) reserva 0x1014 bytes con `_chkstk`; el elevado ya baja `esp` él mismo, aquí
            // solo se deja sitio por debajo para los marcos de sus llamados.
            if f == 0x004B_3EA0 {
                self.sp = self.sp.min(esp).wrapping_sub(0x1014 + 0x100);
            }
            let mut side = CoreSide { rt: self };
            p3_core::x86rt::FPU_EXIT.with(|c| c.set((0, 0)));
            let r0 = with_core_mem(m, |mem| func(mem, &mut side, regs))[0];
            // Si el elevado termina con un valor en la pila x87 (devuelve un float en ST0), el llamante lo recoge con `fret`.
            let (depth, bits) = p3_core::x86rt::FPU_EXIT.with(|c| c.replace((0, 0)));
            p3_core::x86rt::FRET.with(|q| {
                let mut q = q.borrow_mut();
                q.clear();
                if depth > 0 {
                    q.push_back(bits);
                }
            });
            r0
        } else if let Some(func) = self.hand.get(&f).copied() {
            // Portes a mano de `p3-core` con firma `(m, host, this, esp)`: reciben la pila de entrada.
            *self.ported_hot.entry(f).or_default() += 1;
            let mut side = CoreSide { rt: self };
            with_core_mem(m, |mem| func(mem, &mut side, ecx, esp))
        } else if let Some(r) = {
            // Los portes a mano leen `a[i]` según su firma: se les dan 16 palabras de la pila (los argumentos y lo que siga).
            // En la pila de Rust salvo que la llamada traiga más de 16 argumentos (no reserva memoria en cada llamada).
            let mut fixed = [0u32; 16];
            let long: Vec<u32>;
            let full: &[u32] = if args.len() <= 16 {
                for (i, w) in fixed.iter_mut().enumerate() {
                    *w = m.r32(esp + 4 + 4 * i as u32);
                }
                &fixed
            } else {
                long = (0..args.len() as u32).map(|i| m.r32(esp + 4 + 4 * i)).collect();
                &long
            };
            p3_render::j07::run(f, m, self, ecx, full, esp).or_else(|| p3_render::panels::run(f, m, self, ecx, full, esp))
        } {
            *self.ported_hot.entry(f).or_default() += 1;
            match r {
                Ret::None => 0,
                Ret::U8(v) => u32::from(v),
                Ret::U16(v) => u32::from(v),
                Ret::U32(v) => v,
            }
        } else {
            *self.missing.entry(f).or_default() += 1;
            let parent = self.stack.len().checked_sub(2).map_or(0, |i| self.stack[i]);
            let n = self.missing_from.entry((f, parent)).or_default();
            *n += 1;
            // `P3_MISSING`: anuncia cada función ausente la primera vez (útil cuando la ejecución no llega al resumen final).
            if *n == 1 && self.missing_log {
                eprintln!("falta {f:#x} <- {parent:#x} ecx={ecx:#x} retorno={:#x}", m.r32(esp));
            }
            if *n == 1 {
                if let Some(d) = self.diag.as_mut() {
                    d(format!("falta {f:#x} <- {parent:#x}"));
                }
            }
            0
        };
        if self.watch.contains(&f) {
            eprintln!("vuelve {f:#x} -> {r:#x}");
        }
        // `P3_WATCHMEM=DIR` o `*PUNTERO+DESP` (hex): anuncia la función que, al volver, deja cambiada esa palabra.
        if let Some((ind, base, off)) = self.watch_mem {
            let a = if ind { m.r32(base).wrapping_add(off) } else { base };
            let v = m.r32(a);
            if v != self.watch_mem_last {
                eprintln!("memoria {a:#x}: {:#x} -> {v:#x} al volver de {f:#x} pila={:x?}", self.watch_mem_last, &self.stack[self.stack.len().saturating_sub(8)..]);
                self.watch_mem_last = v;
            }
        }
        self.stack.pop();
        if stall::ON.load(std::sync::atomic::Ordering::Relaxed) {
            stall::pop(self.stack.len());
        }
        self.esps.pop();
        self.depth -= 1;
        self.sp = saved;
        r
    }

    /// `malloc`/`operator new`: reutiliza un bloque libre del mismo tamaño redondeado o avanza el montón.
    ///
    /// Los bloques grandes se buscan por «el menor libre que quepa» y se parten: el guardado (`operations_save_game`,
    /// `0x005472F0`) pide en cada autoguardado búferes del tamaño de la partida, que cambia un poco cada vez. Con solo la
    /// lista del tamaño exacto ninguno se reutilizaba y la cima subía ~3 MiB por guardado hasta el tope del montón: el
    /// cliente se cerraba tras una hora de juego («aborta ... montón 0x30000020», 17/11).
    fn heap_take(&mut self, size: u32) -> u32 {
        let rounded = (size.max(1) + 15) & !15;
        let reuse = if rounded >= TRIM_MIN {
            let found = self.heap_free_big.range((rounded, 0)..).next().copied();
            found.map(|(have, at)| {
                self.heap_free_big.remove(&(have, at));
                self.heap_free_big_at.remove(&at);
                (at, have)
            })
        } else {
            self.heap_free_lists.get_mut(&rounded).and_then(|v| v.pop()).map(|at| (at, rounded))
        };
        let at = match reuse {
            Some((at, have)) => {
                if have > rounded {
                    self.heap_free_block(at + rounded, have - rounded);
                }
                at
            }
            None => {
                let at = self.heap;
                self.heap = self.heap.wrapping_add(rounded);
                at
            }
        };
        self.heap_blocks.insert(at, rounded);
        at
    }

    /// Devuelve a las listas libres un bloque no registrado en `heap_blocks` (`rounded` múltiplo de 16). Los grandes se juntan
    /// con los grandes libres contiguos: el guardado siguiente pide algo más que el anterior y cabe en el hueco unido.
    fn heap_free_block(&mut self, at: u32, rounded: u32) {
        if rounded >= TRIM_MIN {
            let (mut at, mut rounded) = (at, rounded);
            if let Some((&prev, &size)) = self.heap_free_big_at.range(..at).next_back() {
                if prev.wrapping_add(size) == at {
                    self.heap_free_big_at.remove(&prev);
                    self.heap_free_big.remove(&(size, prev));
                    at = prev;
                    rounded += size;
                }
            }
            let end = at.wrapping_add(rounded);
            if let Some(size) = self.heap_free_big_at.remove(&end) {
                self.heap_free_big.remove(&(size, end));
                rounded += size;
            }
            self.heap_free_big.insert((rounded, at));
            self.heap_free_big_at.insert(at, rounded);
        } else {
            self.heap_free_lists.entry(rounded).or_default().push(at);
        }
    }

    fn heap_free(&mut self, m: &mut dyn p3_render::scene_math::Mem, p: u32) {
        thread_local!(static NO_REUSE: bool = std::env::var_os("P3_NO_REUSE").is_some());
        if let Some(rounded) = self.heap_blocks.remove(&p) {
            // `P3_NO_REUSE`: no recicla bloques (diagnóstico de usos tras liberar).
            if NO_REUSE.with(|n| *n) {
                return;
            }
            // Poca memoria (móvil): las páginas enteras de un bloque grande liberado se devuelven al sistema. Al reutilizarlo,
            // quien lo pide lo pone a cero igualmente; solo cambia lo que leería un uso tras liberar (ceros en vez del
            // contenido viejo), por eso es opcional (ver [`Runtime::set_low_memory`]).
            if self.low_memory && rounded >= TRIM_MIN {
                m.release(p, rounded);
            }
            self.heap_free_block(p, rounded);
        }
    }

    fn last_w32(&self, _m: &mut dyn p3_render::scene_math::Mem) -> u32 {
        LAST.with(|l| l.get())
    }

    /// CRT y SGL sustituidos.
    fn builtin(&mut self, m: &mut dyn p3_render::scene_math::Mem, f: u32, ecx: u32, esp: u32, arg: &dyn Fn(&mut dyn p3_render::scene_math::Mem, u32) -> u32) -> Option<u32> {
        let _ = ecx;
        if let Some(name) = self.sgl.get(&f).cloned() {
            return Some(self.sgl_call(m, &name, esp, arg));
        }
        if let Some(name) = self.imports.get(&f).cloned() {
            return Some(self.import_call(m, &name, arg));
        }
        // `class75_load_texture(CString, id)` (`0x004B3C30`): si el identificador solo está sembrado (objeto mínimo de
        // `seed_textures`, sin vtable ni rectángulos), se anula esa entrada para que el original lo cargue de `textures.ini`.
        if f == 0x004B_3C30 {
            let (id, n, objs, ids) = (arg(m, 1), m.r32(ecx + 0x10), m.r32(ecx + 4), m.r32(ecx + 8));
            for i in 0..n {
                if m.r32(ids + 4 * i) == id {
                    let obj = m.r32(objs + 4 * i);
                    if m.r32(obj) == 0 {
                        m.w32(ids + 4 * i, 0xFFFF_FFFE);
                    }
                    break;
                }
            }
        }
        if let Some(r) = self.video_options_native(m, f, ecx, esp) {
            return Some(r);
        }
        if let Some(r) = self.zoom_native(m, f, ecx, esp) {
            return Some(r);
        }
        if let Some(r) = self.display_native(m, f, ecx, esp) {
            return Some(r);
        }
        if let Some(r) = self.menu_native(m, f, ecx, esp) {
            return Some(r);
        }
        if let Some(r) = self.window_native(m, f, ecx, esp) {
            return Some(r);
        }
        Some(match f {
            // `is_line_clear` (`0x004445A0`, cdecl; grid, &ancho, fin, inicio): porte nativo sobre la memoria emulada.
            0x0044_45A0 => {
                let a = [arg(m, 0), arg(m, 1), arg(m, 2), arg(m, 3)];
                u32::from(is_line_clear_mem(m, a[0], a[1], a[2], a[3]))
            }
            // `texture_cache_get_or_load` (`0x0054F290`, cdecl; ruta, salida, clave, banderas, modo): el porte de
            // `p3_render::j07::sprite` con su búsqueda lineal hecha aquí de una vez. Al abrir la ciudad se llama unas 6.000 veces
            // sobre una caché de más de mil nombres: ocho millones de `_stricmp` que, uno a uno por el enlazador, eran la mayor
            // partida de la carga. `_stricmp` no tiene efectos, así que solo cambia lo que el enlazador dejaba escrito por debajo de
            // la pila viva (un marco muerto por comparación). Se cuentan las mismas llamadas; con diagnósticos activos, o si en el
            // tramo toca atender la ventana o se alcanza el tope, sigue el porte de siempre.
            0x0054_F290
                if self.prof == 0
                    && !self.trace
                    && self.watch.is_empty()
                    && self.watch_mem.is_none()
                    && self.profile.is_none()
                    && {
                        let after = self.calls + u64::from(m.r32(p3_render::j07::sprite::TEXTURE_COUNT)) + 1;
                        after <= self.call_limit && after >> 14 == self.calls >> 14
                    } =>
            {
                use p3_render::j07::sprite::{LOAD_TEXTURE, TEXTURES, TEXTURE_COUNT};
                let (path, out, key, flags, mode) = (arg(m, 0), arg(m, 1), arg(m, 2), arg(m, 3), arg(m, 4));
                *self.ported_hot.entry(f).or_default() += 1;
                let (count, arr) = (m.r32(TEXTURE_COUNT), m.r32(TEXTURES));
                let mut found = None;
                let mut compared = 0u64;
                for i in 0..count {
                    let name = m.r32(arr + i * 0x10 + 0xC);
                    compared += 1;
                    if cmp_cstr(m, name, path, true).is_eq() {
                        found = Some((i, m.r32(arr + i * 0x10)));
                        break;
                    }
                }
                self.calls += compared;
                match found {
                    Some((i, tex)) if tex != 0 => {
                        m.w32(out, tex);
                        let e = arr + (i << 4) + 4;
                        let v = m.r32(e) | flags;
                        m.w32(e, v);
                        1
                    }
                    _ => {
                        m.w32(out, 0);
                        let tex = self.dispatch(m, LOAD_TEXTURE, u32::MAX, &[path, key, flags, mode]);
                        m.w32(out, tex);
                        u32::from(tex != 0)
                    }
                }
            }
            // `NavGrid::snap_to_sea` (`0x00448450`, thiscall, `ret 8`; punto, radio): porte nativo sobre la memoria emulada.
            0x0044_8450 => {
                let a = [arg(m, 0), arg(m, 1)];
                u32::from(snap_to_sea_mem(m, ecx, a[0], a[1]))
            }
            // `_chkstk` (`0x0063B920`): sondeo de pila de los marcos grandes; aquí cada llamada tiene su propia pila.
            0x0063_B920 => 0,
            // `Merchant::get_merchant_epithet_index` (`0x004F9430`): el porte de `p3-core` trabaja sobre una copia de los `0x650` bytes.
            0x004F_9430 => {
                let merchant = p3_core::ai::reputation::Merchant { bytes: (0..0x650).map(|i| m.r8(ecx.wrapping_add(i))).collect() };
                u32::from(merchant.get_merchant_epithet_index())
            }
            // Retrollamada del pergamino «Disband convoy» / sacar un barco del convoy (`0x004950D0`, cdecl, `ret`), que deja
            // `ship_side_menu_confirm_leave_or_disband_convoy` (`0x494DC0`) en el diálogo `0x41D9B0`: no está en el catálogo y faltaba
            // en el enlazador, así que «Yes» cerraba el pergamino sin hacer nada. Con el botón «Yes» (`[0x6CBB10]+0x5A4 == [0x66BDC0]`)
            // encola la operación 0x16 (disolver el convoy) si la parte baja del argumento es la etiqueta `menú+0x5898`, o 0x17 (sacar el
            // barco) si es `menú+0x589A`; la parte alta va al campo `+4` del registro de operación (convoy o barco).
            0x0049_50D0 => {
                let a = arg(m, 0);
                let dialog = m.r32(0x006C_BB10);
                if m.r32(dialog.wrapping_add(0x5A4)) == m.r32(0x0066_BDC0) {
                    let menu = m.r32(0x006C_E6D0);
                    let tag = a & 0xFFFF;
                    let op = if tag == u32::from(m.r16(menu + 0x5898)) {
                        Some(0x16)
                    } else if tag == u32::from(m.r16(menu + 0x589A)) {
                        Some(0x17)
                    } else {
                        None
                    };
                    if let Some(op) = op {
                        // Registro de operación local (0x14 bytes en el original, `[esp+4]`): por debajo de la pila de entrada.
                        self.sp = self.sp.min(esp.wrapping_sub(0x80));
                        let rec = esp.wrapping_sub(0x40);
                        self.dispatch(m, 0x0053_5740, rec, &[]);
                        m.w32(rec, op);
                        m.w32(rec + 4, a >> 16);
                        self.dispatch(m, 0x0054_AA70, 0x006D_F2F0, &[rec]);
                    }
                }
                0
            }
            // `screen_window_draw` (`0x004B58C0`, `ret 0x10`): dibuja la pantalla contenedora y recorre sus hijos (lista de 0x10 bytes
            // por hijo en `+0x94`, cuenta en `+0xC0`, hijo superior `+0xD4` al final). Nativa porque la versión elevada deduce un solo
            // argumento para el dibujo virtual de cada hijo (`+0x9C`, que retira cuatro) y abandona el bucle tras el primero.
            0x004B_58C0 => {
                let a = [arg(m, 0), arg(m, 1), arg(m, 2), arg(m, 3)];
                // Los marcos de los llamados van por debajo de la pila de entrada: `self.sp` no baja con los llamantes elevados y,
                // dentro de un bucle modal (`screen_list_run_frame`), un marco en `self.sp` pisaba sus locales (el resultado del fotograma).
                self.sp = self.sp.min(esp.wrapping_sub(0x40));
                self.dispatch(m, 0x004B_18F0, ecx, &a);
                let vt = |m: &mut dyn p3_render::scene_math::Mem, o: u32, slot: u32| {
                    let table = m.r32(o);
                    m.r32(table.wrapping_add(slot))
                };
                let visible = vt(m, ecx, 0xC8);
                if self.dispatch(m, visible, ecx, &[]) & 0xFF == 0 {
                    return Some(0);
                }
                let saved = self.dispatch(m, 0x004B_B9B0, ecx, &[a[0]]);
                // Posición de la pantalla en la pila por debajo de la entrada (los marcos de los llamados van por debajo de
                // `esp - 0x40`). Antes se tomaban 16 bytes del montón en cada dibujo, sin devolverlos.
                let pos = esp.wrapping_sub(0x20);
                let get_pos = vt(m, ecx, 0x60);
                self.dispatch(m, get_pos, ecx, &[pos]);
                let p = [m.r32(pos), m.r32(pos + 4), m.r32(pos + 8)];
                let draw_child = |rt: &mut Self, m: &mut dyn p3_render::scene_math::Mem, i: u32| {
                    let e = m.r32(ecx + 0x94).wrapping_add(i * 0x10);
                    let child = m.r32(e);
                    let draw = vt(m, child, 0x9C);
                    let at = [a[0], m.r32(e + 4).wrapping_add(p[0]), m.r32(e + 8).wrapping_add(p[1]), m.r32(e + 12).wrapping_add(p[2])];
                    rt.dispatch(m, draw, child, &at);
                    child
                };
                let mut i = 0;
                while (i as i32) < m.r32(ecx + 0xC0) as i32 {
                    let list = m.r32(ecx + 0x94);
                    if m.r32(list.wrapping_add(i * 0x10)) != 0 && i != m.r32(ecx + 0xD4) {
                        let child = draw_child(self, m, i);
                        if m.r32(0x006D_CB94) != 0 {
                            let r = [0, 0, m.r32(0x006D_CCE0), m.r32(0x006D_CCE4)];
                            self.dispatch(m, 0x004B_B800, ecx, &r);
                        }
                        let clear = vt(m, child, 0xDC);
                        self.dispatch(m, clear, child, &[]);
                    }
                    i += 1;
                }
                let top = m.r32(ecx + 0xD4);
                if top != u32::MAX {
                    draw_child(self, m, top);
                }
                if saved != 0 {
                    self.dispatch(m, 0x004B_B9B0, ecx, &[saved]);
                }
                let clear = vt(m, ecx, 0xDC);
                self.dispatch(m, clear, ecx, &[])
            }
            // `get_merchant` (`0x005303C0`, `ret 4`): `*(this + 0x78) + índice * 0x650`.
            0x0053_03C0 => {
                let index = arg(m, 0);
                m.r32(ecx.wrapping_add(0x78)).wrapping_add(index.wrapping_mul(0x650))
            }
            // El porte de movimiento ya contrastado con x86 evita elevar SHLD y
            // conserva las componentes fijas y el rumbo del barco original.
            0x0051_7410 => {
                let mut ship: Vec<u8> = (0..0x180).map(|i| m.r8(ecx + i)).collect();
                p3_core::ships::motion::set_velocity_towards(&mut ship, arg(m, 0) as i32, arg(m, 1) as i16, arg(m, 2) as i16)
                    .expect("excepción de división del original");
                for off in [1, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f] {
                    m.w8(ecx + off as u32, ship[off]);
                }
                u32::from(ship[1])
            }
            // ceil (CRT, cdecl, argumento double en la pila): devuelve el resultado en ST0, que el llamante elevado recoge con `fret`
            0x0063_A5CE => {
                let x = f64::from_bits(u64::from(arg(m, 0)) | (u64::from(arg(m, 1)) << 32));
                p3_core::x86rt::FRET.with(|q| q.borrow_mut().push_back(x.ceil().to_bits()));
                0
            }
            // operator new / malloc / operator delete / free
            0x0064_F7B9 | 0x0063_CB41 => {
                let size = arg(m, 0);
                if size > 0x0100_0000 && self.trace {
                    eprintln!("asignación enorme {size:#x} desde {:x?}", self.stack);
                }
                let at = self.heap_take(size);
                zero_fill(m, at, size);
                if let Some(log) = self.alloc_log.as_mut() { log.push(at); }
                at
            }
            // operator delete / free: el bloque vuelve a una lista libre por tamaño (solo los de `heap_take`). Sin reciclar, cargar una
            // partida y dibujar unos fotogramas de ciudad agotaba los 128 MiB del montón.
            0x0064_F7E2 | 0x0063_CB7F => {
                { let p = arg(m, 0); self.heap_free(m, p); }
                0
            }
            // `free` con bloqueo 0x0063A1B6/0x0063E410 y la lista libre de CString 0x0064F142: sin reciclar
            0x0063_A1B6 | 0x0063_E410 | 0x0064_F142 => 0,
            // _lock / _unlock del CRT y destructores de listas MFC y de vectores (`eh vector destructor iterator`): sin efecto aquí
            0x0064_1427 | 0x0064_1488 | 0x0064_EB5D | 0x0063_9E28 => 0,
            // `decompress_packed_trade_route` (`0x006387E0`, `pushad`..`popad`, sin argumentos): descomprime `[0x006C7B20]` en `[0x006C7B24]`
            // y deja el tamaño en `[0x006C7B2C]`. Porte con paridad x86 `p3_core::route::rou::decompress` (A03); el flujo comprimido ocupa
            // como mucho 9 bits por byte más la cabecera. Un flujo mal formado deja el tamaño a 0 (el original leería fuera del búfer).
            0x0063_87E0 => {
                let (src, dst) = (m.r32(0x006C_7B20), m.r32(0x006C_7B24));
                let header = m.r32(src) as i32;
                let len = (header.unsigned_abs() as usize).min(0x0400_0000);
                let need = 4 + if header < 0 { len } else { len + len / 8 + 16 };
                let data: Vec<u8> = (0..need as u32).map(|i| m.r8(src.wrapping_add(i))).collect();
                let size = match p3_core::route::rou::decompress(&data) {
                    Some((out, size)) => {
                        write_bytes(m, dst, &out);
                        size
                    }
                    None => 0,
                };
                m.w32(0x006C_7B2C, size);
                0
            }
            // `compress_buffer_core` (`0x006381E0`, `pushad`..`popad`, sin argumentos; lo prepara `compress_buffer` `0x0054C410`): comprime
            // `[0x006C7B28]` bytes de `[0x006C7B20]` en `[0x006C7B24]` y deja el tamaño del bloque (cabecera incluida) en `[0x006C7B2C]`.
            // Porte con paridad x86 `p3_core::route::compress::compress` (B03).
            0x0063_81E0 => {
                let (src, dst, len) = (m.r32(0x006C_7B20), m.r32(0x006C_7B24), m.r32(0x006C_7B28));
                let data: Vec<u8> = (0..len.min(0x0400_0000)).map(|i| m.r8(src.wrapping_add(i))).collect();
                let out = p3_core::route::compress::compress(&data);
                write_bytes(m, dst, &out);
                m.w32(0x006C_7B2C, out.len() as u32);
                0
            }
            // `strrchr` (`0x0063C920`, cdecl): última aparición del carácter (también el terminador) o 0.
            0x0063_C920 => {
                let (p, ch) = (arg(m, 0), arg(m, 1) as u8);
                let s = cstr(m, p);
                if ch == 0 {
                    p.wrapping_add(s.len() as u32)
                } else {
                    s.iter().rposition(|&b| b == ch).map_or(0, |i| p.wrapping_add(i as u32))
                }
            }
            // Constructor de elemento `0x0051E760` del vector `0x006DDB90` (inicializador estático): CString vacía en `+0x28`.
            0x0051_E760 => {
                let empty = m.r32(0x006C_7CCC);
                m.w32(ecx.wrapping_add(0x28), empty);
                ecx
            }
            // _stricmp / _mbsicmp / strcmp
            0x0064_3DB0 | 0x0063_C47B | 0x0063_9CF0 => {
                let (a, b) = (arg(m, 0), arg(m, 1));
                match cmp_cstr(m, a, b, f != 0x0063_9CF0) {
                    std::cmp::Ordering::Less => u32::MAX,
                    std::cmp::Ordering::Equal => 0,
                    std::cmp::Ordering::Greater => 1,
                }
            }
            // realloc(p, n): bloque nuevo con la copia; el viejo vuelve a la lista libre
            0x0063_A29F => {
                let (p, n) = (arg(m, 0), arg(m, 1));
                let at = self.heap_take(n);
                let old = self.heap_blocks.get(&p).copied().unwrap_or(n);
                for i in 0..n {
                    let b = if p != 0 && i < old { m.r8(p.wrapping_add(i)) } else { 0 };
                    m.w8(at + i, b);
                }
                self.heap_free(m, p);
                if let Some(log) = self.alloc_log.as_mut() { log.push(at); }
                at
            }
            // CString::~CString / Release (MFC): el montón del enlazador no libera
            0x0064_F253 => 0,
            // CStringArray destructor: element CString destructors use the same
            // arena policy as 0x64F253; release the array storage itself.
            0x0064_E7FC => {
                m.w32(ecx, 0x0067_D0F0);
                let elements = m.r32(ecx + 4);
                self.heap_free(m, elements);
                0
            }
            // Uncatalogued cdecl letter-response callback (0x4D6C50..82).
            // DialogParchment stores its button result at +0x5A4; the CRT
            // callback forwards a nonnegative result to message_answer_to_operation.
            0x004D_6C50 => {
                let slot = arg(m, 0);
                let count = u32::from(m.r8(0x006D_D736)) | (u32::from(m.r8(0x006D_D737)) << 8);
                if slot >= count {
                    count
                } else {
                    let dialog = m.r32(0x006C_BB10);
                    let answer = m.r32(dialog + 0x5A4).wrapping_sub(m.r32(0x0066_BDC0));
                    if answer as i32 >= 0 {
                        self.dispatch(m, 0x004D_6C90, 0x006D_D730, &[slot, answer])
                    } else {
                        answer
                    }
                }
            }
            // Coma flotante de `printf` del CRT (punteros que pone `_cfltcvt_init`, ver `menus::construct_globals`). Son rutinas con x87
            // y aritmética decimal de 80 bits (`$I10_OUTPUT`); se sustituyen en nativo. `_cfltcvt(double*, buf, formato, precisión, mayúsculas)`.
            0x0063_F39A => {
                let (p, buf, fmt, prec, caps) = (arg(m, 0), arg(m, 1), arg(m, 2) as u8, arg(m, 3) as i32, arg(m, 4) != 0);
                let v = f64::from_bits(u64::from(m.r32(p)) | (u64::from(m.r32(p.wrapping_add(4))) << 32));
                let mut text = crt_cfltcvt(v, fmt, prec.max(0) as usize).into_bytes();
                if caps {
                    text.make_ascii_uppercase();
                }
                text.push(0);
                write_bytes(m, buf, &text);
                buf
            }
            // `_cropzeros(buf)`: quita los ceros finales de la parte decimal (y el punto si se queda solo), respetando el exponente.
            0x0063_F044 => {
                let buf = arg(m, 0);
                let text = cstr(m, buf);
                if let Some(dot) = text.iter().position(|&c| c == b'.') {
                    let exp = text[dot..].iter().position(|&c| c == b'e' || c == b'E').map_or(text.len(), |i| dot + i);
                    let mut end = exp;
                    while end > dot + 1 && text[end - 1] == b'0' {
                        end -= 1;
                    }
                    if end == dot + 1 {
                        end = dot;
                    }
                    let mut out = text[..end].to_vec();
                    out.extend_from_slice(&text[exp..]);
                    out.push(0);
                    write_bytes(m, buf, &out);
                }
                0
            }
            // `_forcdecpt(buf)`: asegura el punto decimal (bandera `#`), delante del exponente si lo hay.
            0x0063_EFEA => {
                let buf = arg(m, 0);
                let text = cstr(m, buf);
                if !text.contains(&b'.') {
                    let at = text.iter().position(|&c| c == b'e' || c == b'E').unwrap_or(text.len());
                    let mut out = text[..at].to_vec();
                    out.push(b'.');
                    out.extend_from_slice(&text[at..]);
                    out.push(0);
                    write_bytes(m, buf, &out);
                }
                0
            }
            // `_positive(double*)`.
            0x0063_F092 => {
                let p = arg(m, 0);
                let v = f64::from_bits(u64::from(m.r32(p)) | (u64::from(m.r32(p.wrapping_add(4))) << 32));
                u32::from(v > 0.0)
            }
            // `_fassign(es_double, destino, texto)` (`scanf` con `%f`/`%lf`).
            0x0063_F0AA => {
                let (dbl, dst, src) = (arg(m, 0), arg(m, 1), arg(m, 2));
                let text = String::from_utf8_lossy(&cstr(m, src)).into_owned();
                let v = crt_atof(&text);
                if dbl != 0 {
                    let b = v.to_bits();
                    m.w32(dst, b as u32);
                    m.w32(dst.wrapping_add(4), (b >> 32) as u32);
                } else {
                    m.w32(dst, (v as f32).to_bits());
                }
                0
            }
            // memcpy, memmove
            0x0063_A790 | 0x0063_AEC0 => {
                let (d, s, n) = (arg(m, 0), arg(m, 1), arg(m, 2));
                let buf: Vec<u8> = (0..n).map(|i| m.r8(s.wrapping_add(i))).collect();
                for (i, b) in buf.into_iter().enumerate() {
                    m.w8(d.wrapping_add(i as u32), b);
                }
                d
            }
            // memset
            0x0063_9F20 => {
                let (d, v, n) = (arg(m, 0), arg(m, 1), arg(m, 2));
                for i in 0..n {
                    m.w8(d.wrapping_add(i), v as u8);
                }
                d
            }
            // `CString::Mid(nFirst, nCount)` (`0x0064D0C0`, `ret 0xC`): devuelve por un `CString*` oculto (argumento 0) la subcadena
            // recortada como MFC (índices negativos a 0, cuenta limitada a lo que queda).
            0x0064_D0C0 => {
                let (ret, first, count) = (arg(m, 0), arg(m, 1) as i32, arg(m, 2) as i32);
                let text = cstring_get(m, ecx);
                let first = first.max(0) as usize;
                let first = first.min(text.len());
                let count = (count.max(0) as usize).min(text.len() - first);
                let piece = text[first..first + count].to_vec();
                self.cstring_set(m, ret, &piece);
                ret
            }
            // `operator+(const CString&, char)` (`0x0064CB60`) y `operator+(char, const CString&)` (`0x0064CBF0`), `ret 0xC`: el resultado va
            // al `CString*` oculto (argumento 0). Los llama la taberna al componer noticias (`0x00526B40`).
            0x0064_CB60 | 0x0064_CBF0 => {
                let ret = arg(m, 0);
                let (s, ch) = if f == 0x0064_CB60 { (arg(m, 1), arg(m, 2) as u8) } else { (arg(m, 2), arg(m, 1) as u8) };
                let mut text = cstring_get(m, s);
                if f == 0x0064_CB60 {
                    text.push(ch);
                } else {
                    text.insert(0, ch);
                }
                m.w32(ret, 0);
                self.cstring_set(m, ret, &text);
                ret
            }
            // `0x004B6C10` (`ret 0xC`, sin nombre en el catálogo; la llama el reenvío `0x004B9300`): copia +0x118 a +0x11C y, si +0x120 está
            // activo, marca +0x121. Ausente tras cancelar la obra con Esc o clic derecho.
            0x004B_6C10 => {
                let v = m.r32(ecx + 0x118);
                m.w32(ecx + 0x11C, v);
                if m.r8(ecx + 0x120) != 0 {
                    m.w8(ecx + 0x121, 1);
                }
                0
            }
            // `CRect::NormalizeRect` (`0x006592A3`, `ret`): ordena izquierda/derecha y arriba/abajo. `class22_invalidate_selection_rect_outline`
            // (`0x005987E0`) la llama sobre el rectángulo de la selección; ausente, el contorno de la parcela de obra no se invalidaba.
            0x0065_92A3 => {
                for k in [0u32, 4] {
                    let (a, b) = (m.r32(ecx + k) as i32, m.r32(ecx + k + 8) as i32);
                    if a > b {
                        m.w32(ecx + k, b as u32);
                        m.w32(ecx + k + 8, a as u32);
                    }
                }
                0
            }
            // `CString::CString(char)` (`0x0064CB40`, `ret 4`): construye en `this` una cadena de un carácter (`0x64F313(1, &carácter)`);
            // la lista de contratos del astillero (página 2) lo usa al componer filas.
            0x0064_CB40 => {
                let ch = arg(m, 0) as u8;
                m.w32(ecx, 0);
                self.cstring_set(m, ecx, &[ch]);
                ecx
            }
            // `CString::operator+=(const CString&)` (`0x0064F61F`, `ret 4`) y `operator+=(LPCSTR)` (`0x0064F5E3`, `ret 4`; nula = nada):
            // los dos acaban en `ConcatInPlace` (`0x64F584`), que añade al final; devuelven `this`. Así compone
            // `load_save_screen_save_game_to_slot` (`0x433A80`) «prefijo + nombre + .pat».
            0x0064_F61F | 0x0064_F5E3 => {
                let src = arg(m, 0);
                let tail = if f == 0x0064_F61F { cstring_get(m, src) } else if src == 0 { Vec::new() } else { self.tr_cstr(m, src) };
                let mut text = cstring_get(m, ecx);
                text.extend(tail);
                self.cstring_set(m, ecx, &text);
                ecx
            }
            // CString::operator+=(char), ret 4: el cargador de mapas construye los
            // cuarenta nombres carácter a carácter. Leer el byte antes de reservar.
            0x0064_F60A => {
                let ch = arg(m, 0) as u8;
                let mut text = cstring_get(m, ecx);
                text.push(ch);
                self.cstring_set(m, ecx, &text);
                ecx
            }
            // `CString::Delete(nIndex, nCount)` (`0x0064CC80`, `ret 8`): quita `nCount` caracteres desde `nIndex`; devuelve la nueva longitud.
            0x0064_CC80 => {
                let (index, count) = (arg(m, 0) as i32, arg(m, 1) as i32);
                let mut text = cstring_get(m, ecx);
                let index = (index.max(0) as usize).min(text.len());
                if count > 0 && index < text.len() {
                    let end = (index + count as usize).min(text.len());
                    text.drain(index..end);
                    self.cstring_set(m, ecx, &text);
                }
                text.len() as u32
            }
            // `0x0040E2F0`: constructor de un elemento de créditos (vtable `0x66AF74`, campos a cero salvo `+0x38 = 1`, `+0x3C = 4`).
            0x0040_E2F0 => {
                m.w32(ecx, 0x0066_AF74);
                for off in [4, 8, 0xC, 0x10, 0x18, 0x40] {
                    m.w32(ecx + off, 0);
                }
                m.w8(ecx + 0x38, 1);
                m.w32(ecx + 0x3C, 4);
                ecx
            }
            // `CString::Find(sz, nStart)` (`0x0064D3D0`, `ret 8`): índice de la primera aparición desde `nStart`, o −1. Las ventanas de
            // créditos lo llaman ~1,3 millones de veces; elevado tarda minutos.
            0x0064_D3D0 => {
                let (sub, start) = (arg(m, 0), arg(m, 1) as i32);
                let text = cstring_get(m, ecx);
                let sub = cstr(m, sub);
                if start < 0 || start as usize > text.len() {
                    u32::MAX
                } else {
                    let hay = &text[start as usize..];
                    let pos = if sub.is_empty() { Some(0) } else { hay.windows(sub.len()).position(|w| w == sub.as_slice()) };
                    pos.map_or(u32::MAX, |p| (p as u32) + start as u32)
                }
            }
            // ticks_to_date `0x5311D0` (stdcall; ticks, día*, mes*, año*): el porte con paridad `tavern_passage::date_from_tick`. Lo pide el
            // balance de la oficina (`0x5DC2A0`, página 0).
            0x0053_11D0 => {
                let (day, month, year) = p3_core::economy::tavern_passage::date_from_tick(arg(m, 0) as i32);
                let (d, mo, y) = (arg(m, 1), arg(m, 2), arg(m, 3));
                m.w8(d, day);
                m.w8(mo, month);
                m.w8(y, year as u8);
                m.w8(y + 1, (year >> 8) as u8);
                0
            }
            // find_letter `0x4D7900` (thiscall del montón de cartas `0x6DD730`; carta, clave): el porte con paridad
            // `tavern_interaction::find_letter`. Sin él la taberna (página 9, `0x5A74B0`) recibía 0 para siempre y no acababa de recorrer la cadena.
            0x004D_7900 => {
                let (a, k) = (arg(m, 0), arg(m, 1));
                p3_core::economy::tavern_interaction::find_letter(&mut RenderMem(m), a, k)
            }
            // get_office `0x530400` (thiscall del mundo `0x6DE4A0`; índice): como el porte con paridad `small_fns::get_office` (registros de 0x44C en `+0x74`, cuenta en `+8`). Sin él, la
            // página de producción del mercado (`0x5DE960`) recorría la lista de oficinas de la ciudad con un puntero nulo para siempre.
            0x0053_0400 => {
                let i = arg(m, 0);
                if i >= m.r32(ecx + 8) & 0xFFFF {
                    0
                } else {
                    m.r32(ecx + 0x74).wrapping_add(i.wrapping_mul(0x44c))
                }
            }
            // _strrchr `0x63C920` (cdecl; cadena, carácter): última aparición del byte bajo del carácter, incluido el cero final (`repne scasb` hacia atrás);
            // 0 si no está. La página 9 de la taberna (`0x5D7FD0`) parte con ella el nombre del remitente de la carta.
            0x0063_C920 => {
                let (s, ch) = (arg(m, 0), arg(m, 1) as u8);
                let mut end = s;
                while m.r8(end) != 0 {
                    end = end.wrapping_add(1);
                }
                let mut p = end;
                loop {
                    if m.r8(p) == ch {
                        break p;
                    }
                    if p == s {
                        break 0;
                    }
                    p = p.wrapping_sub(1);
                }
            }
            // _memcmp `0x63B3E0` (cdecl; a, b, n): −1/0/1 según el primer byte distinto sin signo (`sbb eax,eax; sbb eax,-1`). Existencias de la
            // oficina (`0x5DBF70`, página 1).
            0x0063_B3E0 => {
                let (a, b, n) = (arg(m, 0), arg(m, 1), arg(m, 2));
                let mut r = 0u32;
                for i in 0..n {
                    let (x, y) = (m.r8(a.wrapping_add(i)), m.r8(b.wrapping_add(i)));
                    if x != y {
                        r = if x < y { u32::MAX } else { 1 };
                        break;
                    }
                }
                r
            }
            // _strncmp `0x00642B30` (cdecl; a, b, n): compara como mucho `n` bytes sin signo y se detiene en el fin de cadena: −1/0/1.
            0x0064_2B30 => {
                let (a, b, n) = (arg(m, 0), arg(m, 1), arg(m, 2));
                let mut r = 0u32;
                for i in 0..n {
                    let (x, y) = (m.r8(a.wrapping_add(i)), m.r8(b.wrapping_add(i)));
                    if x != y {
                        r = if x < y { u32::MAX } else { 1 };
                        break;
                    }
                    if x == 0 {
                        break;
                    }
                }
                r
            }
            // strlen
            0x0063_ABE0 => {
                let p = arg(m, 0);
                self.tr_cstr(m, p).len() as u32
            }
            // strcpy, strcat
            0x0063_ADD0 | 0x0063_ADE0 => {
                let (d, s) = (arg(m, 0), arg(m, 1));
                let src = self.tr_cstr(m, s);
                let start = if f == 0x0063_ADE0 { cstr(m, d).len() as u32 } else { 0 };
                for (i, b) in src.iter().chain(std::iter::once(&0)).enumerate() {
                    m.w8(d.wrapping_add(start + i as u32), *b);
                }
                d
            }
            // get_sell_price `0x52E1D0` / get_buy_price `0x52E430` (thiscall de `0x6DE3D8`; mercancía, ciudad, cantidad): los portes
            // con paridad de `p3_core::economy::prices` sobre el registro de la ciudad sembrado en `[0x6DE508]`.
            0x0052_E1D0 | 0x0052_E430 => {
                let (ware, town, amount) = (arg(m, 0) as usize, arg(m, 1), arg(m, 2) as i32);
                let t = m.r32(0x006D_E508).wrapping_add(town.wrapping_mul(0x9f8));
                if t >> 16 == 0 || ware >= 20 {
                    0
                } else {
                    let w = ware as u32;
                    let market = p3_core::economy::prices::Market {
                        trading: m.r32(t + 0x76c) & 0x100 != 0,
                        stock: m.r32(t + 4 + 4 * w) as i32,
                        thresholds: std::array::from_fn(|i| m.r32(t + 0x4f0 + 16 * w + 4 * i as u32) as i32),
                        marked_ware: m.r8(t + 0x6f0),
                    };
                    if f == 0x0052_E430 {
                        p3_core::economy::prices::buy_price(ware, &market, amount) as u32
                    } else {
                        p3_core::economy::prices::sell_price(ware, &market, amount, f32::from_bits(m.r32(ecx + 0x64))) as u32
                    }
                }
            }
            // _rand `0x639C03`: el generador del CRT (semilla por hilo; aquí una sola, inicial 1).
            0x0063_9C03 => RAND.with(|r| {
                let v = r.get().wrapping_mul(0x343FD).wrapping_add(0x269EC3);
                r.set(v);
                (v >> 16) & 0x7FFF
            }),
            // CString::Left(n) `0x64D230` (thiscall; resultado por puntero oculto, n).
            0x0064_D230 => {
                let (out, n) = (arg(m, 0), (arg(m, 1) as i32).max(0) as usize);
                let text = cstring_get(m, ecx);
                m.w32(out, 0);
                self.cstring_set(m, out, &text[..n.min(text.len())]);
                out
            }
            // Sonido de las ventanas de edificio (lista de sonidos, filtro paso bajo) y `sound_manager_play_sound` `0x443150` (la conversación de
            // los baños, `0x5B1D80`): sin audio en el enlazador; 0 es lo que devuelve el original sin dispositivo (`[esi+0x68] == 0`, `0x44347C`).
            // Con mezclador (`self.miles.mixer`, cliente nativo) se ejecuta el original sobre `miles.rs`.
            0x0044_36E0 | 0x0044_39B0 | 0x0044_3150 if self.miles.mixer.is_none() => 0,
            // get_merchant_reputation_in_town_int `0x4F9580` (thiscall del comerciante; ciudad): `Merchant+0x2FC+4·ciudad` truncado, con
            // tope 0x7FFF por encima de 32758.0 (`0x672F58`) y 0 si la ciudad no existe.
            0x004F_9580 => {
                let town = arg(m, 0);
                if m.r32(0x006D_E4B0) <= town {
                    0
                } else {
                    let v = f32::from_bits(m.r32(ecx + 0x2fc + 4 * town));
                    if v > 32758.0 || v.is_nan() {
                        0x7FFF
                    } else {
                        (v as i32) as u32 & 0xFFFF
                    }
                }
            }
            // Listas del prestamista sobre las tareas programadas (thiscall de la lista de tareas): deudas del comerciante `0x4D9100`
            // (comerciante, salida de la cuenta), ofertas de reparto `0x4D9320` y reparto del jugador `0x4D9620` (salida de la cuenta).
            // Portes con paridad de `p3_core::economy::task_lists` sobre la misma memoria; el montón es el del enlazador.
            // `0x4D9210` (deudas que otros tienen con el comerciante: préstamos concedidos, página 5 del prestamista) usa el mismo porte.
            0x004D_9100 | 0x004D_9210 | 0x004D_9320 | 0x004D_9620 => {
                use p3_core::economy::task_lists as tl;
                let (a0, a1) = (arg(m, 0), arg(m, 1));
                let mut heap = BumpHeap(&mut self.heap);
                let mut mem = RenderMem(m);
                match f {
                    0x004D_9100 => tl::collect_debts_owed_to_merchant(&mut mem, &mut heap, ecx, a0, a1),
                    0x004D_9210 => tl::collect_debts_owed_by_merchant(&mut mem, &mut heap, ecx, a0, a1),
                    0x004D_9320 => tl::build_profit_share_offer_list(&mut mem, &mut heap, ecx, a0),
                    _ => tl::build_player_profit_share_list(&mut mem, &mut heap, ecx, a0),
                }
            }
            // town_build_loan_offers_for_player `0x51CE80` (thiscall de la ciudad; registros de 3·0x10 bytes, byte de cambios): porte con
            // paridad `p3_core::economy::loan_offers::build` sobre copias de la ciudad y de los comerciantes (`[0x6DE4A0+0x78]`, cuenta
            // `0x6DE4AA`), jugador `[0x6DFC14]` y número de ciudades `[0x6DE4B0]`.
            0x0051_CE80 => {
                let (recs_at, changed_at) = (arg(m, 0), arg(m, 1));
                let mut town = p3_core::town::Town::default();
                for i in 0..p3_core::town::TOWN_SIZE {
                    town.0[i] = m.r8(ecx.wrapping_add(i as u32));
                }
                let count = u32::from(m.r8(0x006D_E4AA)) | u32::from(m.r8(0x006D_E4AB)) << 8;
                let table = m.r32(0x006D_E4A0 + 0x78);
                let merchants: Vec<_> = (0..count.min(0x40))
                    .map(|i| {
                        let base = table.wrapping_add(i * 0x650);
                        p3_core::ai::reputation::Merchant { bytes: (0..0x650).map(|k| m.r8(base.wrapping_add(k))).collect() }
                    })
                    .collect();
                let player = m.r32(0x006D_FC14) as u8;
                let towns = m.r32(0x006D_E4B0);
                let mut recs: Vec<u8> = (0..0x30).map(|k| m.r8(recs_at.wrapping_add(k))).collect();
                let mut changed = false;
                let ret = p3_core::economy::loan_offers::build(&town, &merchants, player, towns, &mut recs, &mut changed);
                for (k, b) in recs.iter().enumerate() {
                    m.w8(recs_at.wrapping_add(k as u32), *b);
                }
                m.w8(changed_at, u8::from(changed));
                ret as u32
            }
            // quote_new_ship `0x52B2C0` (thiscall de la ciudad; salida, clase, dueño; `ret 0xC`): presupuesto de barco nuevo del astillero,
            // el porte con paridad x86 de `p3_core::economy::ops_misc` (oficina, comerciante y precio de compra vuelven al enlazador).
            0x0052_B2C0 => {
                let (out, kind, owner) = (arg(m, 0), arg(m, 1), arg(m, 2));
                let mut mem = RenderMem(m);
                let mut side = CoreSide { rt: self };
                u32::from(p3_core::economy::ops_misc::quote_new_ship(&mut mem, &mut side, ecx, out, kind, owner))
            }
            // `0x0050A340` (thiscall del gestor naval `0x6DD7A0`): lista de convoyes públicos (contador u16 + índices) en un bloque nuevo; el porte
            // con paridad `p3_core::economy::small_fns::build_public_convoy_index_list` sobre la tabla de convoyes sembrada en `[this+8]`.
            0x0050_A340 => {
                let mut mem = RenderMem(m);
                let mut side = CoreSide { rt: self };
                p3_core::economy::small_fns::build_public_convoy_index_list(&mut mem, &mut side, ecx)
            }
            // `calc_ship_repair_cost` `0x0052A9E0` (thiscall de la ciudad, barco; `ret 4`): la fórmula con paridad `repair_cost_flat`.
            0x0052_A9E0 => {
                let ship = arg(m, 0);
                let mut mem = RenderMem(m);
                p3_core::economy::ops_misc::repair_cost_flat(&mut mem, ecx, ship) as u32
            }
            // `town_calc_ship_repair_cost_for_convoy` `0x0052A950` (thiscall de la ciudad, barco; `ret 4`): suma del convoy sobre `0x52A9E0`.
            0x0052_A950 => {
                let ship = arg(m, 0);
                let mut mem = RenderMem(m);
                let mut side = CoreSide { rt: self };
                p3_core::economy::small_fns::town_calc_ship_repair_cost_for_convoy(&mut mem, &mut side, ecx, &[ship])
            }
            // `town_estimate_ship_repair_days` `0x0052AAD0` (thiscall de la ciudad, barco, indicador; `ret 8`): `ShipManager::estimate_ship_repair_days`
            // sobre copias de las tablas de barcos y convoyes del gestor `0x6DD7A0`.
            0x0052_AAD0 => {
                let (ship, flag) = (arg(m, 0), arg(m, 1));
                let (ships_at, convoys_at) = (m.r32(0x006D_D7A4), m.r32(0x006D_D7A8));
                let (n_ships, n_convoys) = (m.r16(0x006D_D894), m.r16(0x006D_D896));
                let town: Vec<u8> = (0..p3_core::town::TOWN_SIZE as u32).map(|i| m.r8(ecx.wrapping_add(i))).collect();
                let manager = p3_core::shipyard::ShipManager {
                    ships: (0..u32::from(n_ships) * 0x180).map(|i| m.r8(ships_at.wrapping_add(i))).collect(),
                    entries: (0..u32::from(n_convoys) * 0x3c).map(|i| m.r8(convoys_at.wrapping_add(i))).collect(),
                    docked_head: 0xffff,
                    free_entry_head: 0xffff,
                    ship_count: n_ships,
                    entry_count: n_convoys,
                };
                let index = ship.wrapping_sub(ships_at) / 0x180;
                if index >= u32::from(n_ships) {
                    0
                } else {
                    let day = m.r8(0x006D_E4B4);
                    manager.estimate_ship_repair_days(&p3_core::town::Town::from_bytes(&town), day, index as u16, flag) as u32
                }
            }
            // `op_transfer_ware_ship_office` `0x00538100` (thiscall del registro de operación; `ret 0`): mover mercancía entre el barco y la oficina
            // (botones «<» y «>» del pergamino de comercio, modo 0). Porte con paridad x86 `p3_core::economy::small_fns`; faltaba en el enlazador y
            // la orden encolada por el clic se descartaba sin tocar existencias.
            0x0053_8100 => {
                let mut mem = RenderMem(m);
                let mut side = CoreSide { rt: self };
                p3_core::economy::small_fns::op_transfer_ware_ship_office(&mut mem, &mut side, ecx);
                0
            }
            // `op_rename_ship` `0x0053CCE0` (thiscall del registro de operación, con marco propio en pila; botón «Change name» del astillero, página 5):
            // el porte con paridad x86 `p3_core::economy::seh_frame_fns` escribe el nombre en `ship+0x160` y lo registra en el banco `0x6DDAA0`.
            // Faltaba en el enlazador y el cambio de nombre se descartaba.
            0x0053_CCE0 => {
                let mut side = CoreSide { rt: self };
                with_core_mem(m, |mem| p3_core::economy::seh_frame_fns::op_rename_ship(mem, &mut side, ecx, esp));
                0
            }
            // `message_notice_for_new_message` `0x004D7B10` (stdcall; comerciante, mensaje; marco SEH): aviso (sonido y texto del `0x42BB20`) de
            // una carta recién entregada, p. ej. el reembolso al cancelar un contrato del astillero. Porte con paridad x86 de
            // `p3_core::economy::seh_frame_fns`; sin él la entrega de la carta (`0x4D6680`) llamaba a una función ausente.
            0x004D_7B10 => {
                let mut side = CoreSide { rt: self };
                with_core_mem(m, |mem| p3_core::economy::seh_frame_fns::message_notice_for_new_message(mem, &mut side, ecx, esp))
            }
            // `op_repair_ship` `0x0052ACD0` (thiscall del registro de operación, marco SEH; botón «Repair» del astillero, página 0): porte con
            // paridad x86 `p3_core::economy::seh_frame_fns::op_repair_ship`. Faltaba en el enlazador y la orden de reparación se descartaba.
            0x0052_ACD0 => {
                let mut side = CoreSide { rt: self };
                with_core_mem(m, |mem| p3_core::economy::seh_frame_fns::op_repair_ship(mem, &mut side, ecx, esp))
            }
            // `op_move_wares_convoy_office` `0x005396B0` (convoy ↔ oficina, modos 3/4 del pergamino): mismo caso que `0x538100`.
            0x0053_96B0 => {
                let mut mem = RenderMem(m);
                let mut side = CoreSide { rt: self };
                p3_core::economy::small_fns::op_move_wares_convoy_office(&mut mem, &mut side, ecx);
                0
            }
            // `ShipManager::ship_add_cargo` `0x005180C0` (thiscall del barco; mercancía, cantidad, precio; `ret 0xC`): carga con precio medio ponderado.
            // Porte con paridad x86 `p3_core::economy::ops_misc::ship_add_cargo`; lo llama `0x538100` al pasar mercancía de la oficina al barco.
            0x0051_80C0 => {
                let (ware, amount, price) = (arg(m, 0), arg(m, 1), arg(m, 2));
                let mut mem = RenderMem(m);
                let mut side = CoreSide { rt: self };
                p3_core::economy::ops_misc::ship_add_cargo(&mut mem, &mut side, ecx, ware, amount as i32, price);
                0
            }
            // `OfficeInfo::add_stock` `0x004FF6F0` (thiscall de la oficina; mercancía, cantidad, precio; `ret 0xC`): existencia y precio medio de la oficina.
            0x004F_F6F0 => {
                let (ware, amount, price) = (arg(m, 0), arg(m, 1), arg(m, 2));
                let mut mem = RenderMem(m);
                p3_core::economy::ops_misc::office_add_stock(&mut mem, ecx, ware, amount as i32, price);
                0
            }
            // `ShipCargo::recompute_load` `0x005182B0` (thiscall del barco, 0x180 bytes): ajusta lotes y recalcula pesos y valor.
            0x0051_82B0 => {
                let mut ship: Vec<u8> = (0..0x180u32).map(|k| m.r8(ecx.wrapping_add(k))).collect();
                let out = p3_core::ships::cargo::recompute_load(&mut ship);
                for (k, b) in ship.iter().enumerate() {
                    m.w8(ecx.wrapping_add(k as u32), *b);
                }
                out as u32
            }
            // `ship_get_target_crew` `0x005185C0` (thiscall del barco): tripulación objetivo, con el mínimo de 20.
            0x0051_85C0 => {
                let ship: Vec<u8> = (0..0x14u32).map(|k| m.r8(ecx.wrapping_add(k))).collect();
                u32::from(p3_core::economy::pirate_group_tick::target_crew(&ship).unwrap_or(0))
            }
            // `needed_crew` `0x005184F0` (thiscall del barco, 0x180 bytes): tripulación que falta; actualiza la imagen del barco.
            0x0051_84F0 => {
                let mut ship: Vec<u8> = (0..0x180u32).map(|k| m.r8(ecx.wrapping_add(k))).collect();
                let out = p3_core::ships::ai_dock::needed_crew_image(&mut ship);
                for (k, b) in ship.iter().enumerate() {
                    m.w8(ecx.wrapping_add(k as u32), *b);
                }
                out as u32
            }
            // abs(n)
            0x0063_9F78 => (arg(m, 0) as i32).wrapping_abs() as u32,
            // strtol(s, endptr, base): espacios, signo, prefijo `0x`/`0` con base 0 o 16; `*endptr` tras el último dígito.
            0x0063_9F83 => {
                let (sp, end, base) = (arg(m, 0), arg(m, 1), arg(m, 2));
                let t = cstr_max(m, sp, 64);
                let mut i = t.iter().take_while(|c| c.is_ascii_whitespace()).count();
                let neg = t.get(i) == Some(&b'-');
                if matches!(t.get(i), Some(b'-' | b'+')) {
                    i += 1;
                }
                let mut radix = if base == 0 { 10 } else { base };
                if (base == 0 || base == 16) && t.get(i) == Some(&b'0') && matches!(t.get(i + 1), Some(b'x' | b'X')) {
                    radix = 16;
                    i += 2;
                } else if base == 0 && t.get(i) == Some(&b'0') {
                    radix = 8;
                }
                let start = i;
                let mut v: i64 = 0;
                while let Some(d) = t.get(i).and_then(|c| (*c as char).to_digit(radix)) {
                    v = (v * i64::from(radix) + i64::from(d)).min(i64::from(u32::MAX));
                    i += 1;
                }
                if end != 0 {
                    m.w32(end, sp + if i == start { 0 } else { i as u32 });
                }
                let v = if neg { (-v).max(i64::from(i32::MIN)) } else { v.min(i64::from(i32::MAX)) };
                v as u32
            }
            // atoi
            0x0063_9CDA => {
                let p = arg(m, 0);
                let s = cstr(m, p);
                let t = String::from_utf8_lossy(&s);
                let t = t.trim_start();
                let end = t.char_indices().find(|&(i, c)| !(c.is_ascii_digit() || (i == 0 && (c == '-' || c == '+')))).map_or(t.len(), |(i, _)| i);
                t[..end].parse::<i64>().unwrap_or(0) as i32 as u32
            }
            // CString::CString(LPCSTR) / operator=(LPCSTR) (MFC, thiscall): devuelve this
            0x0064_F2C1 | 0x0064_F390 => {
                let p = arg(m, 0);
                let s = if p >> 16 == 0 { Vec::new() } else { self.tr_cstr(m, p) };
                self.cstring_set(m, ecx, &s);
                ecx
            }
            // operator+ de CString (cdecl, resultado por puntero oculto): `0x64F436` (CString, CString), `0x64F49C` (CString, LPCSTR),
            // `0x64F510` (LPCSTR, CString). El resultado se construye en `*arg0`.
            0x0064_F436 | 0x0064_F49C | 0x0064_F510 => {
                let (out, a, b) = (arg(m, 0), arg(m, 1), arg(m, 2));
                let table = self.language.strings.clone();
                let text = |m: &mut dyn p3_render::scene_math::Mem, p: u32, is_cstring: bool| {
                    if is_cstring {
                        cstring_get(m, p)
                    } else if p >> 16 == 0 {
                        Vec::new()
                    } else {
                        table.as_ref().and_then(|t| t.get(&p).cloned()).unwrap_or_else(|| cstr(m, p))
                    }
                };
                let mut joined = text(m, a, f != 0x0064_F510);
                joined.extend(text(m, b, f != 0x0064_F49C));
                m.w32(out, 0);
                self.cstring_set(m, out, &joined);
                out
            }
            // CString::Replace(LPCSTR old, LPCSTR new): número de sustituciones
            0x0064_CE80 => {
                let (a, b) = (arg(m, 0), arg(m, 1));
                let (old, new) = (cstr(m, a), cstr(m, b));
                let cur = cstring_get(m, ecx);
                if old.is_empty() {
                    return Some(0);
                }
                let (mut out, mut n, mut i) = (Vec::new(), 0, 0);
                while i < cur.len() {
                    if cur[i..].starts_with(&old) {
                        out.extend_from_slice(&new);
                        i += old.len();
                        n += 1;
                    } else {
                        out.push(cur[i]);
                        i += 1;
                    }
                }
                if n > 0 {
                    self.cstring_set(m, ecx, &out);
                }
                n
            }
            // CString::Format(CString*, fmt, ...) (cdecl)
            0x0064_DF80 => {
                let (this, fmt) = (arg(m, 0), arg(m, 1));
                let table = self.language.strings.clone();
                let out = Self::printf(m, fmt, esp + 12, table.as_deref());
                self.cstring_set(m, this, &out);
                0
            }
            // sscanf(str, fmt, ...): número de asignaciones
            0x0063_9D74 => {
                let (sp, fp) = (arg(m, 0), arg(m, 1));
                let (input, fmt) = (cstr(m, sp), cstr(m, fp));
                let vals = crt::scan(&input, &fmt);
                for (i, v) in vals.iter().enumerate() {
                    let dst = arg(m, 2 + i as u32);
                    match v {
                        crt::Scanned::Int(x) => m.w32(dst, *x),
                        crt::Scanned::F32(x) => m.w32(dst, x.to_bits()),
                        crt::Scanned::F64(x) => {
                            let b = x.to_bits();
                            m.w32(dst, b as u32);
                            m.w32(dst + 4, (b >> 32) as u32);
                        }
                        crt::Scanned::Char(c) => m.w8(dst, *c),
                        crt::Scanned::Str(t) => {
                            write_bytes(m, dst, t);
                            m.w8(dst + t.len() as u32, 0);
                        }
                    }
                }
                vals.len() as u32
            }
            // _strnicmp(a, b, n)
            0x0064_8DD0 => {
                let (a, b, n) = (arg(m, 0), arg(m, 1), arg(m, 2) as usize);
                let (mut x, mut y) = (cstr_max(m, a, n.min(0x10000)), cstr_max(m, b, n.min(0x10000)));
                x.push(0);
                y.push(0);
                let mut r = 0i32;
                for i in 0..n.min(x.len()).min(y.len()) {
                    let (p, q) = (x[i].to_ascii_lowercase(), y[i].to_ascii_lowercase());
                    if p != q || p == 0 {
                        r = i32::from(p) - i32::from(q);
                        break;
                    }
                }
                r as u32
            }
            // `0x004C4710`: `mov eax, 0x14; ret` (tamaño constante que consulta `0x004B1650`)
            0x004C_4710 => 0x14,
            // strchr
            0x0063_BA90 => {
                let (p, c) = (arg(m, 0), arg(m, 1) as u8);
                let s = cstr(m, p);
                if c == 0 {
                    p + s.len() as u32
                } else {
                    s.iter().position(|&b| b == c).map_or(0, |i| p + i as u32)
                }
            }
            // _strlwr (en el sitio)
            0x0063_C115 => {
                let p = arg(m, 0);
                let s = cstr(m, p).to_ascii_lowercase();
                write_bytes(m, p, &s);
                p
            }
            // strstr
            0x0063_AD50 | 0x0063_B4A3 => {
                let (h, n) = (arg(m, 0), arg(m, 1));
                let (hs, ns) = (cstr(m, h), cstr(m, n));
                if ns.is_empty() {
                    h
                } else {
                    hs.windows(ns.len()).position(|w| w == ns.as_slice()).map_or(0, |i| h + i as u32)
                }
            }
            _ => return None,
        })
    }

    /// Rectángulo de una ranura de recortes: cuatro enteros (`_4i`) o un puntero a cuatro.
    fn rect_arg(&self, m: &mut dyn p3_render::scene_math::Mem, name: &str, arg: &dyn Fn(&mut dyn p3_render::scene_math::Mem, u32) -> u32) -> [i32; 4] {
        if name.ends_with("_4i") {
            [arg(m, 0) as i32, arg(m, 1) as i32, arg(m, 2) as i32, arg(m, 3) as i32]
        } else {
            let p = arg(m, 0);
            [m.r32(p) as i32, m.r32(p + 4) as i32, m.r32(p + 8) as i32, m.r32(p + 12) as i32]
        }
    }

    /// Textura nueva desde un archivo del CPR (`.tga`, `.bmp`, `.aim`); 0 si no existe.
    fn load_texture_file(&mut self, name: &[u8]) -> u32 {
        let text = String::from_utf8_lossy(name).into_owned();
        let Some(data) = self.assets.file_transient(name) else { return 0 };
        if lazy_decodable(&data, &text) {
            self.next_handle += 1;
            self.assets.pending.insert(self.next_handle, PendingImage { raw: name.to_vec(), file: text.clone(), key: None, evicted: None });
        } else {
            let Some(img) = decode(&data, &text) else { return 0 };
            self.next_handle += 1;
            // Ya se sabe que se descodifica: desde aquí es como una aplazada ya pedida (se puede soltar y rehacer).
            let bytes = img.rgba.len();
            self.assets.images.insert(self.next_handle, img);
            self.assets.track(self.next_handle, PendingImage { raw: name.to_vec(), file: text.clone(), key: None, evicted: None }, bytes);
        }
        self.assets.defs.insert(self.next_handle, TexDef { file: text, frames: vec![], color_key: None, flags: 1 });
        self.next_handle
    }

    /// `timeGetTime`: el reloj del cliente o, sin él, `P3_TIME` (0 por defecto, pruebas deterministas).
    pub(super) fn time_get_time(&self) -> u32 {
        match self.input.time_ms {
            Some(t) => t,
            None => std::env::var("P3_TIME").ok().and_then(|v| v.parse().ok()).unwrap_or(0),
        }
    }

    /// Funciones importadas: `Archiver.dll` sobre el CPR; el resto se anota.
    fn import_call(&mut self, m: &mut dyn p3_render::scene_math::Mem, name: &str, arg: &dyn Fn(&mut dyn p3_render::scene_math::Mem, u32) -> u32) -> u32 {
        let func = name.split('!').nth(1).unwrap_or(name);
        if let Some(r) = self.menu_import(m, func, arg) {
            return r;
        }
        if let Some(r) = self.miles_import(m, func, arg) {
            return r;
        }
        // Vídeos de los sucesos (`Vto.dll`, `vto.rs`). Sin el archivo `.sks` `SKDN_InitVideo` da 0 y `SKDN_GetVideoInfos(0)`
        // falla: `0x46AA40` sigue entonces el camino del original sin vídeo (imagen fija con el pergamino dentro del marco).
        if let Some(r) = self.vto_import(m, func, arg) {
            return r;
        }
        match func {
            "arc_CreateFile" => {
                let p = arg(m, 1);
                let path = cstr(m, p);
                match self.assets.file(&path) {
                    Some(data) => {
                        self.next_file += 1;
                        self.files.insert(self.next_file, (data, 0));
                        self.next_file
                    }
                    None => 0,
                }
            }
            "arc_ReleaseFile" => u32::from(self.files.remove(&arg(m, 0)).is_some()),
            "arc_FileLength" => self.files.get(&arg(m, 0)).map_or(u32::MAX, |f| f.0.len() as u32),
            "arc_GetPos" => self.files.get(&arg(m, 0)).map_or(u32::MAX, |f| f.1 as u32),
            "arc_Seek" => {
                let (h, pos) = (arg(m, 0), arg(m, 1));
                match self.files.get_mut(&h) {
                    Some(f) => {
                        f.1 = (pos as usize).min(f.0.len());
                        f.1 as u32
                    }
                    None => u32::MAX,
                }
            }
            "arc_Read" => {
                let (h, buf, n) = (arg(m, 0), arg(m, 1), arg(m, 2));
                let Some((data, pos)) = self.files.get_mut(&h) else { return u32::MAX };
                let take = (n as usize).min(data.len() - *pos);
                let chunk = data[*pos..*pos + take].to_vec();
                *pos += take;
                for (i, b) in chunk.iter().enumerate() {
                    m.w8(buf.wrapping_add(i as u32), *b);
                }
                take as u32
            }
            // En el original la vista apunta dentro del contenedor ya proyectado en memoria (`cpr_OpenArchiveRO`, solo lectura) y el
            // juego no la libera (p. ej. la caché INI `0x004BE1E0` proyecta `textures.ini` en cada clave que no está en caché). Aquí
            // cada archivo se copia una vez y las proyecciones siguientes devuelven la misma copia: copiarlo cada vez agotaba el
            // montón del enlazador (la oficina comercial lo pedía unas 3.800 veces al cargar las caras de su maqueta).
            "arc_CreateMemMapped" => {
                let (archive, p, out_data, out_size) = (arg(m, 0), arg(m, 1), arg(m, 2), arg(m, 3));
                let path = cstr(m, p);
                // Contenedor 0: archivo suelto en la carpeta del juego (partidas `save/ein/*.pat`). Se lee cada vez (puede cambiar).
                // Si no está en la carpeta (o no hay carpeta), del CPR como con el contenedor abierto: las pruebas que no abren el
                // contenedor de texturas (`[0x006DCCC8]` = 0) cargan así el banco de nombres.
                let loose = (archive == 0).then(|| self.menus.find_files(&String::from_utf8_lossy(&path))).and_then(|f| f.first().and_then(|f| crate::hostfs::current().read(f).ok()));
                if let Some(data) = loose {
                    let at = self.heap;
                    self.heap = self.heap.wrapping_add((data.len() as u32 + 16) & !15);
                    write_bytes(m, at, &data);
                    if out_data != 0 {
                        m.w32(out_data, at);
                    }
                    if out_size != 0 {
                        m.w32(out_size, data.len() as u32);
                    }
                    return at;
                }
                if self.trace {
                    eprintln!("    mapped {} cached={}", String::from_utf8_lossy(&path), self.mapped.contains_key(&path));
                }
                let (at, len) = match self.mapped.get(&path) {
                    Some(&v) => v,
                    None => {
                        let Some(data) = self.assets.file(&path) else { return 0 };
                        let at = self.heap;
                        self.heap = self.heap.wrapping_add((data.len() as u32 + 16) & !15);
                        for (i, b) in data.iter().enumerate() {
                            m.w8(at + i as u32, *b);
                        }
                        self.mapped.insert(path, (at, data.len() as u32));
                        (at, data.len() as u32)
                    }
                };
                if out_data != 0 {
                    m.w32(out_data, at);
                }
                if out_size != 0 {
                    m.w32(out_size, len);
                }
                at
            }
            "arc_ReleaseMemMapped" => 1,
            "lstrlenA" => {
                let p = arg(m, 0);
                if p == 0 { 0 } else { cstr(m, p).len() as u32 }
            }
            // Los `.ini` del juego (`./scripts/...`) salen del CPR; sin archivo o sin clave se copia el valor por defecto, como Windows.
            "GetPrivateProfileStringA" => {
                let (section, key, d, buf, size, file) = (arg(m, 0), arg(m, 1), arg(m, 2), arg(m, 3), arg(m, 4), arg(m, 5));
                let found = self.ini_value(m, file, section, key);
                if self.trace {
                    let (s, k) = (cstr(m, section), cstr(m, key));
                    eprintln!("    ini [{}] {} = {:?}", String::from_utf8_lossy(&s), String::from_utf8_lossy(&k), found.as_ref().map(|v| String::from_utf8_lossy(v).into_owned()));
                }
                let mut def = found.unwrap_or_else(|| if d == 0 { Vec::new() } else { cstr(m, d) });
                def.truncate(size.saturating_sub(1) as usize);
                write_bytes(m, buf, &def);
                m.w8(buf + def.len() as u32, 0);
                def.len() as u32
            }
            "GetPrivateProfileIntA" => {
                let (section, key, d, file) = (arg(m, 0), arg(m, 1), arg(m, 2), arg(m, 3));
                match self.ini_value(m, file, section, key) {
                    Some(v) => String::from_utf8_lossy(&v).trim().parse::<i64>().map_or(d, |n| n as u32),
                    None => d,
                }
            }
            "WritePrivateProfileStringA" => {
                let (section, key, value, file) = (arg(m, 0), arg(m, 1), arg(m, 2), arg(m, 3));
                self.write_user_ini(m, section, key, value, file)
            }
            "InterlockedIncrement" | "InterlockedDecrement" => {
                let p = arg(m, 0);
                let v = if func == "InterlockedIncrement" { m.r32(p).wrapping_add(1) } else { m.r32(p).wrapping_sub(1) };
                m.w32(p, v);
                v
            }
            // Ratón y reloj: sin entrada real; `SetRect(r, l, t, rt, b)` rellena el rectángulo.
            "SetRect" => {
                let r = arg(m, 0);
                for k in 0..4 {
                    let v = arg(m, 1 + k);
                    m.w32(r + 4 * k, v);
                }
                1
            }
            "IsRectEmpty" => {
                let r = arg(m, 0);
                let (l, t, rt, b) = (m.r32(r) as i32, m.r32(r + 4) as i32, m.r32(r + 8) as i32, m.r32(r + 12) as i32);
                u32::from(rt <= l || b <= t)
            }
            // `InflateRect(rect, dx, dy)`: ensancha dx/dy por cada lado (pergamino de comercio, `trade_parchment_*`).
            "InflateRect" => {
                let (r, dx, dy) = (arg(m, 0), arg(m, 1), arg(m, 2));
                for (k, d) in [dx.wrapping_neg(), dy.wrapping_neg(), dx, dy].into_iter().enumerate() {
                    let v = m.r32(r + 4 * k as u32).wrapping_add(d);
                    m.w32(r + 4 * k as u32, v);
                }
                1
            }
            "OffsetRect" => {
                let (r, dx, dy) = (arg(m, 0), arg(m, 1), arg(m, 2));
                for (k, d) in [dx, dy, dx, dy].into_iter().enumerate() {
                    let v = m.r32(r + 4 * k as u32).wrapping_add(d);
                    m.w32(r + 4 * k as u32, v);
                }
                1
            }
            // The chart uses this result to invalidate moving hulls and to
            // restore their old bounds next frame. Read both inputs first:
            // Win32 callers may use either source as the destination.
            "IntersectRect" => {
                let (d, a, b) = (arg(m, 0), arg(m, 1), arg(m, 2));
                let rd = |m: &mut dyn p3_render::scene_math::Mem, r: u32| [0, 1, 2, 3].map(|k| m.r32(r + 4 * k) as i32);
                let (a, b) = (rd(m, a), rd(m, b));
                let r = [a[0].max(b[0]), a[1].max(b[1]), a[2].min(b[2]), a[3].min(b[3])];
                let intersects = r[0] < r[2] && r[1] < r[3];
                for (k, v) in (if intersects { r } else { [0; 4] }).into_iter().enumerate() {
                    m.w32(d + 4 * k as u32, v as u32);
                }
                u32::from(intersects)
            }
            "UnionRect" => {
                let (d, a, b) = (arg(m, 0), arg(m, 1), arg(m, 2));
                let rd = |m: &mut dyn p3_render::scene_math::Mem, r: u32| [0, 1, 2, 3].map(|k| m.r32(r + 4 * k) as i32);
                let (x, y) = (rd(m, a), rd(m, b));
                let empty = |r: &[i32; 4]| r[2] <= r[0] || r[3] <= r[1];
                let u = if empty(&x) { y } else if empty(&y) { x } else { [x[0].min(y[0]), x[1].min(y[1]), x[2].max(y[2]), x[3].max(y[3])] };
                for (k, v) in u.into_iter().enumerate() {
                    m.w32(d + 4 * k as u32, v as u32);
                }
                u32::from(!empty(&u))
            }
            "Sleep" => 0,
            // `CreateProcessA(app, línea, ...)`: la intro en Bink (`binkplay.exe`), si el cliente pone [`Runtime::on_create_process`].
            // Rellena `PROCESS_INFORMATION` (`arg 9`) con manejadores ficticios; `WaitForSingleObject` y `CloseHandle` ya dan 0.
            "CreateProcessA" => {
                let (line, pi) = (arg(m, 1), arg(m, 9));
                let line = String::from_utf8_lossy(&cstr(m, line)).into_owned();
                let launched = match self.on_create_process.as_mut() {
                    Some(f) => f(&line),
                    None => false,
                };
                if launched {
                    for (k, v) in [0x5052_0001u32, 0x5052_0002, 1, 1].into_iter().enumerate() {
                        m.w32(pi + 4 * k as u32, v);
                    }
                }
                u32::from(launched)
            }
            // Mensajes de Windows (`win32.rs`): la cola la llena la ventana nativa en cada presentación.
            "PeekMessageA" => {
                let (msg, min, max, remove) = (arg(m, 0), arg(m, 2), arg(m, 3), arg(m, 4));
                match win32::peek(&mut self.messages, min, max, remove & 1 != 0) {
                    Some(x) => {
                        for (k, w) in x.words().into_iter().enumerate() {
                            m.w32(msg + 4 * k as u32, w);
                        }
                        1
                    }
                    None => 0,
                }
            }
            // `GetMessageA` espera a que haya un mensaje: presenta (la ventana recoge entrada) hasta que llegue; 0 con `WM_QUIT`.
            "GetMessageA" => {
                let (msg, min, max) = (arg(m, 0), arg(m, 2), arg(m, 3));
                let x = loop {
                    if let Some(x) = win32::peek(&mut self.messages, min, max, true) {
                        break x;
                    }
                    if self.on_present.is_none() {
                        break win32::Msg { hwnd: win32::MAIN_HWND, message: win32::WM_QUIT, ..Default::default() };
                    }
                    self.present();
                };
                for (k, w) in x.words().into_iter().enumerate() {
                    m.w32(msg + 4 * k as u32, w);
                }
                u32::from(x.message != win32::WM_QUIT)
            }
            // El juego no llama a `TranslateMessage` (`screen_list_run_frame` despacha directamente): el texto sale de `ToAscii`.
            "TranslateMessage" => 0,
            // `DispatchMessageA(msg)`: al mapa de mensajes de la ventana principal (`game_window.rs`).
            "DispatchMessageA" => {
                let p = arg(m, 0);
                self.dispatch_window_message(m, p)
            }
            // `GetKeyboardState(buf)` y `ToAscii(vk, scan, estado, buf, flags)` de `virtual_key_to_ascii` (`0x004C0F50`).
            "GetKeyboardState" => {
                let buf = arg(m, 0);
                for vk in 0..256u32 {
                    let i = vk as usize;
                    let v = (if self.input.down[i] { 0x80 } else { 0 }) | u8::from(self.input.toggled[i]);
                    m.w8(buf + vk, v);
                }
                1
            }
            "ToAscii" => {
                let (vk, state, out) = (arg(m, 0), arg(m, 2), arg(m, 3));
                let st = |m: &mut dyn p3_render::scene_math::Mem, k: u8| m.r8(state + u32::from(k));
                let shift = st(m, win32::VK_SHIFT) & 0x80 != 0;
                let caps = st(m, win32::VK_CAPITAL) & 1 != 0;
                match win32::to_ascii(vk as u8, shift, caps) {
                    Some(c) => {
                        m.w8(out, c);
                        m.w8(out + 1, 0);
                        1
                    }
                    None => 0,
                }
            }
            "GetClientRect" => {
                let r = arg(m, 1);
                let (w, h) = (self.surface.width as u32, self.surface.height as u32);
                for (k, v) in [0, 0, w, h].into_iter().enumerate() {
                    m.w32(r + 4 * k as u32, v);
                }
                1
            }
            "PtInRect" => {
                let (r, x, y) = (arg(m, 0), arg(m, 1) as i32, arg(m, 2) as i32);
                let v = [0, 1, 2, 3].map(|k| m.r32(r + 4 * k) as i32);
                u32::from(x >= v[0] && x < v[2] && y >= v[1] && y < v[3])
            }
            // Captura y cursor del sistema: los lleva la ventana nativa.
            "SetCapture" | "ReleaseCapture" | "SetCursor" | "ShowCursor" => 0,
            "GetCursorPos" => {
                let p = arg(m, 0);
                m.w32(p, self.input.cursor.0 as u32);
                m.w32(p + 4, self.input.cursor.1 as u32);
                1
            }
            "GetKeyState" => self.input.key_state(arg(m, 0)),
            "GetAsyncKeyState" => {
                let vk = arg(m, 0);
                self.input.async_key_state(vk)
            }
            "ClientToScreen" | "ScreenToClient" => 1,
            // `SetCursorPos` (lo usa `clamp_cursor_to_screen` 0x4BD340): mueve el cursor que ve el juego.
            "SetCursorPos" => {
                self.input.cursor = (arg(m, 0) as i32, arg(m, 1) as i32);
                1
            }
            "timeGetTime" => self.time_get_time(),
            // Secciones críticas: el enlazador es de un solo hilo. `InterlockedExchange(dst, v)` devuelve el valor anterior.
            "EnterCriticalSection" | "LeaveCriticalSection" | "InitializeCriticalSection" | "DeleteCriticalSection" => 0,
            "InterlockedExchange" => {
                let (p, v) = (arg(m, 0), arg(m, 1));
                let old = m.r32(p);
                m.w32(p, v);
                old
            }
            // `CopyRect(dst, src)`: copia los cuatro enteros del rectángulo.
            "CopyRect" => {
                let (d, src) = (arg(m, 0), arg(m, 1));
                for k in 0..4 {
                    let v = m.r32(src + 4 * k);
                    m.w32(d + 4 * k, v);
                }
                1
            }
            "SetRectEmpty" => {
                let r = arg(m, 0);
                for k in 0..4 {
                    m.w32(r + 4 * k, 0);
                }
                1
            }
            _ => {
                if self.import_missing.insert(name.to_string()) {
                    if let Some(d) = self.diag.as_mut() {
                        d(format!("importación sin sustituto: {name}"));
                    }
                }
                0
            }
        }
    }

    /// Valor de `clave` en la sección `section` del `.ini` `file` (ruta relativa del juego, tomada del CPR); `None` si falta.
    pub(super) fn ini_value(&mut self, m: &mut dyn p3_render::scene_math::Mem, file: u32, section: u32, key: u32) -> Option<Vec<u8>> {
        static NO_INI: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
        if *NO_INI.get_or_init(|| std::env::var_os("P3_NO_INI").is_some()) {
            return None;
        }
        let mut path = cstr(m, file);
        while path.starts_with(b"./") || path.starts_with(b".\\") {
            path.drain(..2);
        }
        // Cada `.ini` del CPR se analiza una sola vez: (sección, clave) en minúsculas → primer valor del archivo. Antes cada consulta
        // copiaba el archivo y lo recorría entero (decenas de miles de consultas al entrar en partida). El CPR no cambia en marcha.
        let section = cstr_max(m, section, 64).to_ascii_lowercase();
        let key = cstr(m, key).to_ascii_lowercase();
        // `P2.CFG` (ajustes del jugador) vive en la carpeta del juego y lo reescribe [`Self::write_user_ini`]: se lee del disco cada vez.
        if let Some(host) = self.user_ini_path(&path) {
            if let Ok(data) = crate::hostfs::current().read(&host) {
                return profile::parse(&data).remove(&(section, key));
            }
        }
        if !self.ini_cache.contains_key(&path) {
            let parsed = self.assets.file(&path).map(|data| profile::parse(&data));
            self.ini_cache.insert(path.clone(), parsed);
        }
        let table = self.ini_cache.get(&path)?.as_ref()?;
        table.get(&(section, key)).cloned()
    }

    /// Ruta en disco de un `.ini` del usuario ([`profile::is_user_ini`]) relativo a la carpeta del juego; `None` para los del CPR.
    fn user_ini_path(&self, path: &[u8]) -> Option<std::path::PathBuf> {
        if !profile::is_user_ini(path) {
            return None;
        }
        self.menus.host_path(&String::from_utf8_lossy(path))
    }

    /// `WritePrivateProfileStringA(sección, clave, valor, archivo)` (stdcall): solo `P2.CFG` junto al juego (opciones `0x0045DF10`,
    /// nueva partida `0x00499BE0`); sección nula vacía la caché de Windows (aquí no hay) y devuelve 1. Otros archivos: 0, como antes.
    fn write_user_ini(&mut self, m: &mut dyn p3_render::scene_math::Mem, section: u32, key: u32, value: u32, file: u32) -> u32 {
        if section == 0 {
            return 1;
        }
        let mut path = cstr(m, file);
        while path.starts_with(b"./") || path.starts_with(b".\\") {
            path.drain(..2);
        }
        let Some(host) = self.user_ini_path(&path) else { return 0 };
        let section = cstr_max(m, section, 64);
        let key = (key != 0).then(|| cstr(m, key));
        let value = (value != 0).then(|| cstr(m, value));
        let fs = crate::hostfs::current();
        let old = fs.read(&host).unwrap_or_default();
        let new = profile::write(&old, &section, key.as_deref(), value.as_deref());
        u32::from(new == old || fs.write(&host, &new).is_ok())
    }

    /// Vuelve a calcular la imagen RGBA de una textura de memoria desde sus píxeles (con paleta o de 32 bits BGRA).
    /// Textura cargada de archivo (`sgl_LoadMemoryTexture`) que el EXE bloquea después (p. ej. el mapa de zonas del menú
    /// principal, `images/options/hmenu_colormap.bmp`, cuyos índices son los botones): se registra como textura de memoria
    /// con sus índices y paleta originales (BMP de 8 bits, de abajo arriba → de arriba abajo, paso a 4 bytes).
    fn register_loaded_texture(&mut self, h: u32) {
        let Some(file) = self.assets.defs.get(&h).map(|d| d.file.clone()).filter(|f| !f.is_empty()) else { return };
        let Some(data) = self.assets.file(file.as_bytes()) else { return };
        let flags = self.assets.defs.get(&h).map_or(1, |d| d.flags);
        let lower = file.to_ascii_lowercase();
        let aim = if lower.ends_with(".bmp") {
            crate::navmap::bmp_indexed(&data)
        } else if lower.ends_with(".tga") {
            tga_as_aim(&data)
        } else {
            p3_assets::aim::decode(&data).ok()
        };
        let Some(aim) = aim else { return };
        let Some(id) = self.tex_store.load_memory_texture(&aim, flags) else { return };
        let size = self.tex_store.get(id).map_or(0, |t| t.pixels.len() as u32);
        let (buf, pal) = self.take_tex_block(size);
        self.mem_tex.insert(h, (id, buf, pal));
        self.loaded_tex.insert(h);
    }

    /// `sgl_SetRenderDest`: cambia la superficie de dibujo entre la pantalla y una textura (devuelve la imagen de la anterior a
    /// `assets.images` y a su copia de `tex_store`).
    /// `sgl_DrawBitmapRect` con los recortes 2D de la DLL: una pasada por rectángulo del recortador o del rectángulo de
    /// recorte (`SglClip::passes`), como los rasterizadores de `ddraw_Dll` (`0x05001A90`/`0x05001DD0`).
    /// El cursor del ratón se dibuja en el búfer trasero, recortado a las zonas sucias (su posición vieja y la nueva), y
    /// `present_frame` copia esas zonas al delantero. La posición vieja solo se limpia si alguna ventana repinta debajo; en avance
    /// rápido la pantalla de juego sale de la lista (`0x4239C0`) y nadie repinta la ciudad, así que el trasero acumulaba una
    /// pluma por fotograma y la copia de la zona vieja la devolvía al delantero (filas de plumas al mover el ratón). Se guarda
    /// el fondo del trasero antes de cada dibujo del cursor y se repone tras presentar: el delantero recibe el cursor y el
    /// trasero sigue limpio, como si el cursor fuera una capa aparte. Solo con dos búferes (con uno, el trasero es lo que se ve).
    fn save_under_cursor(&mut self, dx: i32, dy: i32, w: i32, h: i32) {
        if !self.two_buffers || self.target_tex.is_some() || self.canvas_active {
            return;
        }
        // Cursor: `0x4AD540` llamado directamente desde `screen_list_run_frame` (`0x4B8A40`), no desde una ventana.
        let n = self.stack.len();
        let cursor = (1..n).rev().take(8).any(|k| self.stack[k] == 0x004A_D540 && self.stack[k - 1] == 0x004B_8A40);
        if !cursor {
            return;
        }
        let (sw, sh) = (self.surface.width as i32, self.surface.height as i32);
        let (l, t, r, b) = (dx.max(0), dy.max(0), (dx + w).min(sw), (dy + h).min(sh));
        if r <= l || b <= t || self.surface.rgba.len() != (sw * sh * 4) as usize {
            return;
        }
        let (l, t, rw, rh, sw) = (l as usize, t as usize, (r - l) as usize, (b - t) as usize, sw as usize);
        let mut px = Vec::with_capacity(rw * rh * 4);
        for y in t..t + rh {
            px.extend_from_slice(&self.surface.rgba[(y * sw + l) * 4..(y * sw + l + rw) * 4]);
        }
        self.cursor_under.push((self.screen_is_front, l, t, rw, rh, px));
    }

    /// Repone, en orden inverso, el fondo guardado por [`Runtime::save_under_cursor`] en el búfer donde se dibujó el cursor.
    fn restore_under_cursor(&mut self) {
        while let Some((was_front, l, t, rw, rh, px)) = self.cursor_under.pop() {
            let (sw, len) = (self.surface.width, self.surface.rgba.len());
            let buf = if was_front == self.screen_is_front { &mut self.surface } else { &mut self.front };
            if buf.rgba.len() != len || l + rw > sw || (t + rh) * sw * 4 > len {
                continue;
            }
            for (k, y) in (t..t + rh).enumerate() {
                buf.rgba[(y * sw + l) * 4..(y * sw + l + rw) * 4].copy_from_slice(&px[k * rw * 4..(k + 1) * rw * 4]);
            }
        }
    }

    fn blit_clipped(&mut self, source: u32, (sx, sy, dx, dy, w, h): (i32, i32, i32, i32, i32, i32)) {
        for c in self.clip.passes(Some([dx, dy, dx + w, dy + h])) {
            let Some([cl, ct, cr, cb]) = c else {
                self.surface.blit(&mut self.assets, source, (sx, sy, dx, dy, w, h));
                continue;
            };
            let (l, t, r, b) = (dx.max(cl), dy.max(ct), (dx + w).min(cr), (dy + h).min(cb));
            if r > l && b > t {
                self.surface.blit(&mut self.assets, source, (sx + l - dx, sy + t - dy, l, t, r - l, b - t));
            }
        }
    }

    /// Ejecuta `f` con la pantalla como destino y vuelve después a la textura que estuviera elegida (`sgl_SwapBuffer`, `sgl_Blt_RC`).
    fn on_screen(&mut self, f: impl FnOnce(&mut Self)) {
        let Some(t) = self.target_tex else { return f(self) };
        // `0xFFFF_FFFC` deja la textura sin cambiar de búfer de pantalla (`screen_buffer` la ignora): se vuelve al que estuviera.
        self.select_target(0xFFFF_FFFC);
        f(self);
        self.select_target(t);
    }

    /// `P3_TEXDUMP=carpeta`: guarda la imagen RGBA de la textura `h` (diagnóstico).
    fn debug_dump_texture(&mut self, h: u32, why: &str) {
        let Some(dir) = std::env::var_os("P3_TEXDUMP") else { return };
        if let Some(img) = self.assets.images.get(&h) {
            let path = std::path::Path::new(&dir).join(format!("{h:08x}_{}_{}x{}_{why}.rgba", self.presents, img.width, img.height));
            let _ = std::fs::write(path, &img.rgba);
        }
    }

    fn select_target(&mut self, d: u32) {
        let screen = d == 0 || d >= 0xFFFF_FFF0;
        if !screen {
            self.assets.materialize(d);
        }
        let new = (!screen && self.assets.images.contains_key(&d)).then_some(d);
        if new == self.target_tex {
            if screen {
                self.screen_buffer(d);
            }
            return;
        }
        if let Some(t) = self.target_tex.take() {
            self.target_to_tex_store();
            let rgba = std::mem::take(&mut self.surface.rgba);
            self.assets.pin(t);
            self.assets.images.insert(t, Image { width: self.surface.width, height: self.surface.height, rgba });
            self.debug_dump_texture(t, "destino");
            if let Some((w, h, rgba)) = self.screen_stash.take() {
                (self.surface.width, self.surface.height, self.surface.rgba) = (w, h, rgba);
            }
            self.surface.in_texture = false;
        }
        if screen {
            self.screen_buffer(d);
        }
        if let Some(t) = new {
            let Some(img) = self.assets.images.remove(&t) else { return };
            self.assets.pin(t);
            let screen = std::mem::take(&mut self.surface.rgba);
            self.screen_stash = Some((self.surface.width, self.surface.height, screen));
            (self.surface.width, self.surface.height, self.surface.rgba) = (img.width, img.height, img.rgba);
            self.target_tex = Some(t);
            self.surface.in_texture = true;
        }
    }

    /// Copia la superficie (destino vigente) a los píxeles de su textura en memoria (32 bits B, G, R, A).
    fn target_to_tex_store(&mut self) {
        let Some(t) = self.target_tex else { return };
        let Some(&(id, _, _)) = self.mem_tex.get(&t) else { return };
        let Some(tex) = self.tex_store.get_mut(id) else { return };
        if tex.bytes_per_pixel != 4 || usize::from(tex.width) != self.surface.width || usize::from(tex.height) != self.surface.height {
            return;
        }
        for y in 0..self.surface.height {
            for x in 0..self.surface.width {
                let o = (y * self.surface.width + x) * 4;
                let [r, g, b, a] = [self.surface.rgba[o], self.surface.rgba[o + 1], self.surface.rgba[o + 2], self.surface.rgba[o + 3]];
                let i = y * tex.pitch as usize + 4 * x;
                tex.pixels[i..i + 4].copy_from_slice(&[b, g, r, a]);
            }
        }
    }

    /// Espejo en memoria emulada de una textura en memoria (píxeles y paleta, 0x400 bytes más): bloque del montón del juego, para que
    /// `free_mem_texture` lo devuelva. Antes avanzaba el montón sin devolver nada y cada cambio de resolución (Opciones ↔ ciudad a
    /// 2560×1440 recrea los búferes de la escena, ~100 MiB) lo agotaba a los pocos Esc seguidos (14/11).
    fn take_tex_block(&mut self, size: u32) -> (u32, u32) {
        let pal_off = (size + 15) & !15;
        let buf = self.heap_take(pal_off + 0x400);
        (buf, buf.wrapping_add(pal_off))
    }

    /// `sgl_FreeMemoryTexture` / `sgl_FreeTexture`: libera la textura del almacén, su imagen RGBA del lado del cliente y el bloque del
    /// invitado. Un manejador que no es una textura en memoria (archivo diferido, fuente) se deja como estaba.
    fn free_mem_texture(&mut self, m: &mut dyn p3_render::scene_math::Mem, handle: u32) {
        if self.target_tex == Some(handle) {
            return;
        }
        let Some((id, buf, _)) = self.mem_tex.remove(&handle) else { return };
        self.assets.own_mode.remove(&handle);
        self.tex_store.free(id);
        self.assets.pin(handle);
        self.assets.pending.remove(&handle);
        self.assets.images.remove(&handle);
        self.loaded_tex.remove(&handle);
        self.heap_free(m, buf);
    }

    fn refresh_mem_texture(&mut self, handle: u32) {
        let Some(&(id, _, _)) = self.mem_tex.get(&handle) else { return };
        let Some(t) = self.tex_store.get(id) else { return };
        let (w, h) = (usize::from(t.width), usize::from(t.height));
        let mut rgba = Vec::with_capacity(w * h * 4);
        for y in 0..h {
            for x in 0..w {
                let argb = match &t.palette {
                    Some(pal) if t.bytes_per_pixel == 1 => pal[usize::from(t.pixels[y * t.pitch as usize + x])],
                    _ => {
                        let i = y * t.pitch as usize + 4 * x;
                        u32::from_le_bytes(t.pixels[i..i + 4].try_into().unwrap())
                    }
                };
                let [b, g, r, a] = argb.to_le_bytes();
                rgba.extend_from_slice(&[r, g, b, a]);
            }
        }
        self.assets.pin(handle);
        self.assets.images.insert(handle, Image { width: w, height: h, rgba });
    }

    fn sgl_call(&mut self, m: &mut dyn p3_render::scene_math::Mem, name: &str, esp: u32, arg: &dyn Fn(&mut dyn p3_render::scene_math::Mem, u32) -> u32) -> u32 {
        if self.sgl_log {
            let a: Vec<u32> = (0..6).map(|k| arg(m, k)).collect();
            let d = self.assets.defs.get(&a[0]).map(|d| (d.file.clone(), d.flags, d.color_key));
            eprintln!("sgl {name} {a:x?} def={d:?} mem={} destino={:x?} pila={:x?}", self.mem_tex.contains_key(&a[0]), self.target_tex, &self.stack[self.stack.len().saturating_sub(8)..]);
        }
        if self.screen_is_front
            && self.two_buffers
            && self.target_tex.is_none()
            && !self.canvas_active
            && matches!(name, "sgl_Blt" | "sgl_DrawBitmapRect" | "sgl_FillSolidRect" | "sgl_DrawText" | "sgl_DrawText_Rect" | "ui_render_text_at" | "sgl_DrawLine")
        {
            self.front_direct = true;
        }
        let s = &mut self.surface;
        let i = |m: &mut dyn p3_render::scene_math::Mem, k: u32| arg(m, k) as i32;
        let _ = esp;
        match name {
            // Destino de dibujo: 0 = pantalla. Los destinos en textura se dibujan aún en la pantalla (se anota cuántas veces).
            // Destino de dibujo: una textura creada en memoria recibe lo que se dibuje hasta el siguiente cambio (búfer de la escena de
            // ciudad, minimapa); 0 y los negativos (-1 trasero, -3 delantero) son la pantalla.
            "sgl_SetRenderDest" => {
                let d = arg(m, 0);
                self.select_target(d);
                std::mem::replace(&mut self.render_dest, d)
            }
            // Presentación y modo 3D: la superficie ya es el destino; no hay nada que presentar ni limpiar aquí.
            // `sgl_SetColorKey`: la clave de color ya la aplica la carga de texturas según `textures.ini`.
            "sgl_Begin3D" | "sgl_Finish3D" | "sgl_SetZoomFactor" | "sgl_ClearBuffer" => 0,
            // `sgl_SetColorKey` (`0x0501CDC0`): clave de las texturas que se carguen después; devuelve la anterior.
            "sgl_SetColorKey" => {
                if self.trace {
                    eprintln!("  SetColorKey {:#x}", arg(m, 0));
                }
                self.tex_store.set_color_key(arg(m, 0))
            }
            // Fin de fotograma (`present_frame` 0x4B8FF0 → 0x4BB740): la ventana nativa muestra la superficie y recoge la entrada.
            // El intercambio y la copia entre búferes actúan siempre sobre la pantalla, aunque el destino vigente sea una textura: la
            // batalla naval deja elegida la del minimapa (`0x004AD540`) al acabar el fotograma, y antes se presentaba esa textura
            // (260×227) en vez de la pantalla (15/11).
            "sgl_SwapBuffer" => {
                // Operaciones que pidió un guion de prueba (`operaciones_guion`): `operation_record_init` (`0x00535740`), las
                // palabras del registro y `enqueue_operation` (`0x0054AA70`), como las encola la interfaz del original.
                let guion = std::mem::take(&mut *self.operaciones_guion.borrow_mut());
                for op in guion {
                    self.sp = self.sp.min(esp.wrapping_sub(0x80));
                    let rec = esp.wrapping_sub(0x40);
                    self.dispatch(m, 0x0053_5740, rec, &[]);
                    for (k, v) in op.iter().enumerate() {
                        m.w32(rec + 4 * k as u32, *v);
                    }
                    self.dispatch(m, 0x0054_AA70, 0x006D_F2F0, &[rec]);
                }
                self.on_screen(|rt| rt.present());
                0
            }
            // `sgl_GrabScreen(2, 0)`: atajo Ctrl+S del original (`present_frame` `0x004B9029`, tras `CreateDirectoryA("grabs")`).
            // Guarda la pantalla en `grabs/grabNNNN.bmp` de la carpeta del juego (24 bits; el nombre que usaba la DLL SGL no se
            // conoce). Antes la llamada caía en una exportación ausente y no guardaba nada (17/11).
            "sgl_GrabScreen" => {
                self.grab_screen();
                0
            }
            "sgl_SetConstantColor" => std::mem::replace(&mut s.color, arg(m, 0)),
            "sgl_SetRenderSource" | "sgl_SelectTexture" => std::mem::replace(&mut s.source, arg(m, 0)),
            "sgl_SetBlendMode" => u32::from(std::mem::replace(&mut s.mode, arg(m, 0) as u8)),
            "sgl_SetFont" => std::mem::replace(&mut s.font, arg(m, 0)),
            "sgl_SetTextMode" => std::mem::replace(&mut s.text_mode, i(m, 0)) as u32,
            // Copia escalada: el minimapa de la batalla naval se compone a 513×452 y se reduce a 260×227 (`0x0059CEB0`); sin ella
            // la textura del minimapa quedaba negra (15/11).
            "sgl_StretchBitmapRect" => {
                let (sx, sy, dx, dy, w, h, w2, h2) = (i(m, 0), i(m, 1), i(m, 2), i(m, 3), i(m, 4), i(m, 5), i(m, 6), i(m, 7));
                let source = s.source;
                let full = [0, 0, self.surface.width as i32, self.surface.height as i32];
                for c in self.clip.passes(Some([dx, dy, dx + w2, dy + h2])) {
                    self.surface.stretch(&mut self.assets, source, (sx, sy), (dx, dy), (w, h), (w2, h2), c.unwrap_or(full));
                }
                0
            }
            "sgl_DrawBitmapRect" => {
                let r = (i(m, 0), i(m, 1), i(m, 2), i(m, 3), i(m, 4), i(m, 5));
                self.save_under_cursor(r.2, r.3, r.4, r.5);
                let source = self.surface.source;
                self.blit_clipped(source, r);
                0
            }
            "sgl_DrawLine" => {
                let points = (f32::from_bits(arg(m,0)), f32::from_bits(arg(m,1)), f32::from_bits(arg(m,2)), f32::from_bits(arg(m,3)));
                for clip in self.clip.passes(None) {
                    self.surface.line(points, clip);
                }
                0
            }
            "sgl_FillSolidRect" => {
                let (x, y, w, h) = (i(m, 0), i(m, 1), i(m, 2), i(m, 3));
                for c in self.clip.passes(Some([x, y, x + w, y + h])) {
                    let [l, t, r, b] = c.map_or([x, y, x + w, y + h], |c| [x.max(c[0]), y.max(c[1]), (x + w).min(c[2]), (y + h).min(c[3])]);
                    if r > l && b > t {
                        self.surface.fill(l, t, r - l, b - t);
                    }
                }
                0
            }
            "ui_render_text_at" | "sgl_DrawText" => {
                self.language_tick(m);
                let (x, y, p) = (i(m, 0), i(m, 1), arg(m, 2));
                self.language_note_font(x);
                let text = self.tr_cstr(m, p);
                if self.trace || std::env::var_os("P3_TEXTLOG").is_some() {
                    eprintln!("  texto ({x},{y}) fuente {:#x} color {:#x}: {:?}", self.surface.font, self.surface.color, String::from_utf8_lossy(&text));
                }
                self.texts.push((x, y, String::from_utf8_lossy(&text).into_owned()));
                // El texto respeta la región de repintado igual que los bitmaps. Sin este recorte se vuelve a
                // componer el alfa de las letras sobre el fotograma anterior, fuera del fondo restaurado, y sus
                // bordes se saturan a blanco con cada fotograma.
                let handle = self.surface.font;
                let glyphs = self.assets.fonts.get(&handle).map(|f| f.draw(x, y, &text, self.surface.text_mode)).unwrap_or_default();
                for [sx, sy, dx, dy, w, h] in glyphs {
                    self.blit_clipped(handle, (sx, sy, dx, dy, w, h));
                }
                0
            }
            // `sgl_DrawText_Rect(x, y, rect*, texto)` (`ddraw_Dll` `0x0500F100`): como `sgl_DrawText`, recortado al rectángulo
            // `[izq, arr, der, ab]` (listas de la pantalla de carga, celdas de tabla). También alinea según `sgl_SetTextMode`: pasa el
            // ancho a `0x0500EEE0`, que dibuja con `0x0500EDB0` (2 derecha, 3 centrado, leído de `0x05099534`). Sin alinear,
            // «Loading game...» centrado en x = 579 empezaba allí y el rectángulo lo cortaba en «Loading g».
            "sgl_DrawText_Rect" => {
                self.language_tick(m);
                let (x, y, r, p) = (i(m, 0), i(m, 1), arg(m, 2), arg(m, 3));
                let text = self.tr_cstr(m, p);
                let rect = [m.r32(r) as i32, m.r32(r + 4) as i32, m.r32(r + 8) as i32, m.r32(r + 12) as i32];
                if self.trace || std::env::var_os("P3_TEXTLOG").is_some() {
                    eprintln!("  texto en rect ({x},{y}) {rect:?}: {:?}", String::from_utf8_lossy(&text));
                }
                let handle = self.surface.font;
                let glyphs = self.assets.fonts.get(&handle).map(|f| f.draw(x, y, &text, self.surface.text_mode)).unwrap_or_default();
                for [sx, sy, dx, dy, w, h] in glyphs {
                    let (l, t) = (dx.max(rect[0]), dy.max(rect[1]));
                    let (rr, b) = ((dx + w).min(rect[2]), (dy + h).min(rect[3]));
                    if rr > l && b > t {
                        self.blit_clipped(handle, (sx + l - dx, sy + t - dy, l, t, rr - l, b - t));
                    }
                }
                0
            }
            "sgl_StrLen" => {
                let p = arg(m, 0);
                let text = cstr(m, p);
                let w = self.assets.fonts.get(&s.font).map_or(0, |f| f.text_width(&text)) as u32;
                if self.trace {
                    eprintln!("  ancho {:?} fuente {:#x} = {w}", String::from_utf8_lossy(&text), s.font);
                }
                w
            }
            "sgl_SetArchive" => std::mem::replace(&mut self.sgl_archive, arg(m, 0)),
            "sgl_LoadMemoryTexture" | "sgl_LoadTexture" => {
                let p = arg(m, 0);
                let name = cstr(m, p);
                // Segundo argumento: indicadores de la textura (1 opaca, 3 con alfa...), los que el original deduce de `textures.ini`.
                let flags = arg(m, 1);
                let h = self.load_texture_file(&name);
                // Indicadores con los tres bits bajos (clave de color): la imagen cargada usa la clave vigente de `sgl_SetColorKey`.
                let key = self.tex_store.color_key();
                if let Some(def) = self.assets.defs.get_mut(&h) {
                    def.flags = flags;
                    if flags & 7 == 7 {
                        def.color_key = Some(0xFF00_0000 | key);
                    }
                }
                // Igual que `Assets::image` con las claves de `textures.ini`: los píxeles de la clave quedan transparentes.
                if flags & 7 == 7 {
                    let [_, r, g, b] = (0xFF00_0000 | key).to_be_bytes();
                    // Una imagen soltada que ya tenía otra clave aplicada: se rehace antes, para que queden las dos como si
                    // nunca se hubiera soltado.
                    if self.assets.pending.get(&h).is_some_and(|p| p.evicted.is_some() && p.key.is_some_and(|k| k != (r, g, b))) {
                        self.assets.materialize(h);
                    }
                    if let Some(p) = self.assets.pending.get_mut(&h) {
                        p.key = Some((r, g, b));
                    }
                    // Viva y sin clave todavía: la receta pasa a llevar esta. Con otra clave ya aplicada deja de poder rehacerse.
                    match self.assets.live.get_mut(&h) {
                        Some(l) if l.0.key.is_none() || l.0.key == Some((r, g, b)) => l.0.key = Some((r, g, b)),
                        Some(_) => self.assets.pin(h),
                        None => {}
                    }
                    if let Some(img) = self.assets.images.get_mut(&h) {
                        for p in img.rgba.chunks_exact_mut(4) {
                            if (p[0], p[1], p[2]) == (r, g, b) {
                                p[3] = 0;
                            }
                        }
                    }
                }
                if self.trace {
                    eprintln!("  textura {:?} indicadores {flags:#x} -> {h:#x}", String::from_utf8_lossy(&name));
                }
                h
            }
            "sgl_ReLoadMemoryTexture" | "sgl_ReLoadTexture" => {
                let (h, p) = (arg(m, 0), arg(m, 1));
                let name = cstr(m, p);
                let new = self.load_texture_file(&name);
                self.assets.materialize(new);
                if let Some(img) = self.assets.images.remove(&new) {
                    self.assets.pin(new);
                    self.assets.pin(h);
                    self.assets.pending.remove(&h);
                    self.assets.images.insert(h, img);
                }
                h
            }
            "sgl_FreeMemoryTexture" | "sgl_FreeTexture" => {
                let h = arg(m, 0);
                self.free_mem_texture(m, h);
                1
            }
            "sgl_FreeFont" => 1,
            "sgl_MemoryFont_CreateKerning" | "sgl_MemoryFont_CreateProportional" | "sgl_MemoryFont_CreateFixed" => {
                let (charset, tex, w, h) = (arg(m, 0), arg(m, 1), arg(m, 2), arg(m, 3));
                let (spacing, space) = if name == "sgl_MemoryFont_CreateFixed" { (0, 4) } else { (i(m, 4), arg(m, 5) as u8) };
                let mut chars = cstr(m, charset);
                if chars.len() == p3_render::sgl_font::GAME_CHARSET.len() { chars.extend_from_slice(&self.language.extra_charset); }
                self.assets.materialize(tex);
                let Some(img) = self.assets.images.get(&tex) else { return 0 };
                let bgra: Vec<u8> = img.rgba.chunks_exact(4).flat_map(|p| [p[2], p[1], p[0], p[3]]).collect();
                let locked = p3_render::sgl_font::Locked { pixels: &bgra, width: img.width as u32, pitch: img.width * 4 };
                let font = p3_render::sgl_font::create(&chars, &locked, w, h, spacing, space, name == "sgl_MemoryFont_CreateKerning");
                let image = Image { width: img.width, height: img.height, rgba: img.rgba.clone() };
                self.next_handle += 1;
                let handle = self.next_handle;
                self.assets.images.insert(handle, image);
                self.assets.defs.insert(handle, TexDef { file: String::new(), frames: vec![], color_key: None, flags: 3 });
                self.assets.fonts.insert(handle, font);
                handle
            }
            // Recortes: rectángulos `[izq, arr, der, ab]` en 4 enteros (`_4i`) o por puntero (`_r` y sin sufijo).
            "sgl_IsRectInClippingRegion" | "sgl_IsRectInClippingRegion_r" | "sgl_IsRectInClippingRegion_4i" => {
                let r = self.rect_arg(m, name, arg);
                u32::from(self.clip.is_rect_in(r))
            }
            "sgl_AddClipRect_4i" | "sgl_AddClipRect_r" => {
                let r = self.rect_arg(m, name, arg);
                self.clip.add_clip_rect(r);
                0
            }
            "sgl_SubClipRect_4i" | "sgl_SubClipRect_r" => {
                let r = self.rect_arg(m, name, arg);
                self.clip.sub_clip_rect(r);
                0
            }
            "sgl_SetClipRect" | "sgl_SetClipRect_4i" | "sgl_SetClipRect_r" => {
                let r = self.rect_arg(m, name, arg);
                self.clip.set_clip_rect(r);
                0
            }
            "sgl_GetClipRect_r" => {
                let p = arg(m, 0);
                for (k, v) in self.clip.clip.iter().enumerate() {
                    m.w32(p + 4 * k as u32, *v as u32);
                }
                0
            }
            "sgl_GetClipRect_4i" => {
                for k in 0..4 {
                    let p = arg(m, k);
                    m.w32(p, self.clip.clip[k as usize] as u32);
                }
                0
            }
            "sgl_ResetClipper" => {
                self.clip.reset_clipper();
                0
            }
            "sgl_CreateClipper" | "sgl_GetRectClipper" => self.clip.create_clipper() as u32 + 1,
            "sgl_SetActiveClipper" => {
                let h = arg(m, 0);
                let prev = self.clip.set_active((h != 0).then(|| h as usize - 1));
                prev as u32 + 1
            }
            "sgl_FreeClipper" | "sgl_FreeRectClipper" => {
                let h = arg(m, 0);
                if h != 0 {
                    self.clip.free_clipper(h as usize - 1);
                }
                0
            }
            "sgl_CopyClipper" => {
                let h = arg(m, 0);
                if h != 0 {
                    self.clip.copy_clipper(h as usize - 1);
                }
                0
            }
            "sgl_Set2dClippingMode" => self.clip.set_mode(i(m, 0)) as u32,
            "sgl_Get2dClippingMode" => self.clip.mode as u32,
            // Textura vacía `ancho × alto` (fondo transparente).
            "sgl_CreateMemoryTexture" => {
                let (w, h) = (arg(m, 0) as usize, arg(m, 1) as usize);
                self.next_handle += 1;
                let img = Image { width: w, height: h, rgba: vec![0; w.saturating_mul(h).min(1 << 26) * 4] };
                self.assets.images.insert(self.next_handle, img);
                self.assets.defs.insert(self.next_handle, TexDef { file: String::new(), frames: vec![], color_key: None, flags: arg(m, 2) });
                let f = arg(m, 2);
                self.assets.own_mode.insert(self.next_handle, if f >> 24 != 0 { (f >> 24) as u8 } else { (f & 1) as u8 });
                let handle = self.next_handle;
                let id = self.tex_store.create_memory_texture(arg(m, 0), arg(m, 1), arg(m, 2));
                let size = self.tex_store.get(id).map_or(0, |t| t.pixels.len() as u32);
                let (buf, pal) = self.take_tex_block(size);
                self.mem_tex.insert(handle, (id, buf, pal));
                self.refresh_mem_texture(handle);
                handle
            }
            // `sgl_LockTexture(tex, info*, flags)` (`0x0501D630`): rellena `[píxeles, paleta, ancho, alto, paso]` y devuelve 1; los
            // píxeles son el espejo en memoria emulada, que se vuelve a leer al desbloquear.
            "sgl_LockTexture" => {
                let (mut h, info, flags) = (arg(m, 0), arg(m, 1), arg(m, 2));
                // -4: el destino de dibujo vigente (`class22_draw_tile_layers` escribe el suelo en el búfer de la escena).
                if h == 0xFFFF_FFFC {
                    if let Some(t) = self.target_tex {
                        h = t;
                    }
                }
                // Sin destino en textura, -4 es la propia pantalla: `class22_draw` (`0x5889D9`) la bloquea para la refracción de las
                // ondas del agua (`water_ripple_render_refraction` 0x59E8E0), que lee y escribe sus píxeles. Se entrega un espejo de
                // 32 bits (B, G, R, A) de la superficie y se vuelve a leer al desbloquear.
                if h == 0xFFFF_FFFC && self.target_tex.is_none() {
                    let (w, hh) = (self.surface.width, self.surface.height);
                    let (id, buf) = match self.screen_locks.get(&(w, hh)) {
                        Some(&e) => e,
                        None => {
                            // Un solo espejo, en una zona fija fuera del montón del juego: con el zoom el lienzo cambia de tamaño a
                            // cada paso de la rueda y un búfer por tamaño agotaba el montón en 4K.
                            for (_, (old, _)) in self.screen_locks.drain() {
                                self.tex_store.free(old);
                            }
                            let id = self.tex_store.create_memory_texture(w as u32, hh as u32, 0);
                            let buf = SCREEN_LOCK_BASE;
                            self.screen_locks.insert((w, hh), (id, buf));
                            (id, buf)
                        }
                    };
                    let Some(li) = self.tex_store.lock(id, flags) else { return 0 };
                    let t = self.tex_store.get_mut(id).unwrap();
                    if t.locks == 1 {
                        // El espejo no se rellena aquí: copiar la pantalla entera de ida y vuelta costaba ~12 ms por fotograma a
                        // 2560x1440 y casi nunca hay ondas. Lo rellena `screen_lock_fill` cuando `0x59E8E0` va a usarlo.
                        self.screen_locked = Some((id, buf));
                        self.screen_lock_live = None;
                    }
                    m.w32(info, buf);
                    m.w32(info + 4, 0);
                    m.w32(info + 8, li.width);
                    m.w32(info + 12, li.height);
                    m.w32(info + 16, li.pitch);
                    return 1;
                }
                if Some(h) == self.target_tex {
                    self.target_to_tex_store();
                }
                if !self.mem_tex.contains_key(&h) {
                    self.register_loaded_texture(h);
                }
                let Some(&(id, buf, pal)) = self.mem_tex.get(&h) else {
                    if self.sgl_missing.insert(format!("{name} (textura no creada en memoria)")) {
                        if let Some(d) = self.diag.as_mut() {
                            d(format!("SGL sin sustituto: {name} (textura no creada en memoria)"));
                        }
                    }
                    return 0;
                };
                let Some(li) = self.tex_store.lock(id, flags) else { return 0 };
                let t = self.tex_store.get(id).unwrap();
                if t.locks == 1 {
                    m.write_bytes(buf,&t.pixels);
                }
                let has_pal = t.lock_palette.is_some();
                if let Some(lp) = &t.lock_palette {
                    for (i, e) in lp.iter().enumerate() {
                        m.w32(pal + 4 * i as u32, *e);
                    }
                }
                m.w32(info, buf);
                m.w32(info + 4, if has_pal { pal } else { 0 });
                m.w32(info + 8, li.width);
                m.w32(info + 12, li.height);
                m.w32(info + 16, li.pitch);
                1
            }
            "sgl_UnlockTexture" => {
                let mut h = arg(m, 0);
                if h == 0xFFFF_FFFC && self.target_tex.is_none() {
                    if let Some((id, buf)) = self.screen_locked {
                        let w = self.surface.width;
                        if let Some(t) = self.tex_store.get_mut(id) {
                            if t.locks == 1 {
                                self.screen_locked = None;
                            }
                            if let (true, Some(bb), true) = (t.locks == 1, self.screen_lock_live.take(), usize::from(t.width) == w && usize::from(t.height) == self.surface.height) {
                                let [l, t0, r, b] = bb.map(|v| v as usize);
                                let pitch = t.pitch as usize;
                                for y in t0..b {
                                    let row = &mut t.pixels[y * pitch + l * 4..y * pitch + r * 4];
                                    m.read_bytes(buf + (y * pitch + l * 4) as u32, row);
                                    for (p, d) in row.chunks_exact(4).zip(self.surface.rgba[(y * w + l) * 4..(y * w + r) * 4].chunks_exact_mut(4)) {
                                        d.copy_from_slice(&[p[2], p[1], p[0], 255]);
                                    }
                                }
                            }
                        }
                        self.tex_store.unlock(id);
                    }
                    return 0;
                }
                if h == 0xFFFF_FFFC {
                    if let Some(t) = self.target_tex {
                        h = t;
                    }
                }
                if let Some(&(id, buf, _)) = self.mem_tex.get(&h) {
                    if let Some(t) = self.tex_store.get_mut(id) {
                        m.read_bytes(buf,&mut t.pixels);
                    }
                    self.tex_store.unlock(id);
                    // Las cargadas de archivo conservan su imagen RGBA (con clave de color) tal como se descodificó.
                    if !self.loaded_tex.contains(&h) {
                        self.refresh_mem_texture(h);
                    }
                    self.debug_dump_texture(h, "unlock");
                    // El destino vigente vive en la superficie: se recarga con lo que se escribió durante el bloqueo.
                    if Some(h) == self.target_tex {
                        self.assets.pin(h);
                        if let Some(img) = self.assets.images.remove(&h) {
                            if img.width == self.surface.width && img.height == self.surface.height {
                                self.surface.rgba = img.rgba;
                            }
                        }
                    }
                }
                0
            }
            // `sgl_ChangePalette(tex, paleta*)`: aplica la clave de color a la tabla del llamante y a la textura (devuelve 0 siempre).
            "sgl_ChangePalette" => {
                let (h, pal) = (arg(m, 0), arg(m, 1));
                if let Some(&(id, _, _)) = self.mem_tex.get(&h) {
                    let mut table = [0u32; 256];
                    for (i, e) in table.iter_mut().enumerate() {
                        *e = m.r32(pal + 4 * i as u32);
                    }
                    if self.tex_store.change_palette(id, &mut table) {
                        for (i, e) in table.iter().enumerate() {
                            m.w32(pal + 4 * i as u32, *e);
                        }
                    }
                    self.refresh_mem_texture(h);
                }
                0
            }
            // `sgl_GetColorKey(tex)` (`0x0501CDE0`): la clave de la textura si sus indicadores tienen alguno de los bits 0..2, si no 0.
            "sgl_GetColorKey" => {
                let h = arg(m, 0);
                if let Some(&(id, _, _)) = self.mem_tex.get(&h) {
                    if self.tex_store.get(id).is_some() { self.tex_store.get_color_key(id) } else { 0 }
                } else {
                    match self.assets.defs.get(&h) {
                        Some(d) if d.flags & 7 != 0 => d.color_key.map_or(0, |k| k & 0xFF_FFFF),
                        _ => 0,
                    }
                }
            }
            // `sgl_GetConstantColor` (`0x0500F7D0`): el color de `sgl_SetConstantColor`.
            "sgl_GetConstantColor" => self.surface.color,
            // `sgl_Blt_RC(0)` (`present_frame`): copia las zonas sucias del búfer trasero al delantero; aquí hay una sola superficie.
            // `sgl_PrefetchTexture`: sube la textura a la tarjeta; sin efecto en memoria.
            "sgl_PrefetchTexture" => 0,
            // `sgl_Blt_RC(0)` (`present_frame` 0x4B8FF0, con destino `-3` y el recortador de zonas sucias activo): copia esas zonas
            // del búfer trasero al delantero.
            "sgl_Blt_RC" if self.target_tex.is_some() => {
                self.on_screen(|rt| {
                    rt.sgl_call(m, "sgl_Blt_RC", esp, arg);
                });
                0
            }
            "sgl_Blt_RC" => {
                if self.two_buffers && self.target_tex.is_none() && !self.canvas_active && self.screen_is_front && self.front.rgba.len() == self.surface.rgba.len() {
                    let (w, h) = (self.surface.width, self.surface.height);
                    for c in self.clip.passes(None) {
                        let [l, t, r, b] = c.map_or([0, 0, w as i32, h as i32], |c| [c[0].max(0), c[1].max(0), c[2].min(w as i32), c[3].min(h as i32)]);
                        if r > l && b > t {
                            for y in t as usize..b as usize {
                                let (a, z) = ((y * w + l as usize) * 4, (y * w + r as usize) * 4);
                                self.surface.rgba[a..z].copy_from_slice(&self.front.rgba[a..z]);
                            }
                        }
                    }
                }
                0
            }
            // `sgl_Blt(sx, sy, dx, dy, w, h, flags)` (`0x004BB350`): copia un rectángulo de la superficie de dibujo sobre sí misma
            // (`ddraw` Blt de pantalla a pantalla; los rectángulos se solapan). La usa el desplazamiento de la escena de ciudad
            // (`0x00592E60`): mueve lo ya dibujado y solo repinta la franja que queda al descubierto. Sin ella, con pasos de cámara
            // pequeños el suelo quedaba a trozos viejos. 0 = correcto.
            "sgl_Blt" => {
                let (sx, sy, dx, dy, w, h) = (i(m, 0), i(m, 1), i(m, 2), i(m, 3), i(m, 4), i(m, 5));
                self.surface.copy_rect(sx, sy, dx, dy, w, h);
                0
            }
            "sgl_GetTextureInfo" => {
                let (h, out) = (arg(m, 0), arg(m, 1));
                let (w, hh) = self.assets.image(h).map_or((0, 0), |img| (img.width as u32, img.height as u32));
                m.w32(out, w);
                m.w32(out + 4, hh);
                m.w32(out + 8, h);
                1
            }
            _ => {
                let slot = self.sgl.iter().find(|(_, n)| &***n == name).map_or(SGL_BASE, |(a, _)| *a);
                *self.missing.entry(slot).or_default() += 1;
                if self.sgl_missing.insert(name.to_string()) {
                    if let Some(d) = self.diag.as_mut() {
                        d(format!("SGL sin sustituto: {name}"));
                    }
                }
                0
            }
        }
    }
}

/// Montón de los portes de `p3_core` que piden [`p3_core::economy::task_lists::Heap`]: el mismo puntero que avanza `operator new`
/// del enlazador (memoria a cero, sin reciclar).
struct BumpHeap<'h>(&'h mut u32);
impl p3_core::economy::task_lists::Heap for BumpHeap<'_> {
    fn alloc(&mut self, m: &mut dyn p3_core::flatmem::Mem, size: u32) -> u32 {
        let at = *self.0;
        *self.0 = at.wrapping_add((size.max(1) + 15) & !15);
        for i in 0..size {
            m.w8(at + i, 0);
        }
        at
    }
    fn free(&mut self, _addr: u32) {}
}

thread_local! {
    static RAND: std::cell::Cell<u32> = const { std::cell::Cell::new(1) };
    static LAST: std::cell::Cell<u32> = const { std::cell::Cell::new(0) };
}

/// Semilla de `_rand` (`0x639C03`) del hilo actual (diagnóstico: volcados para cotejar con el x86).
pub fn rand_state() -> u32 {
    RAND.with(|r| r.get())
}

impl Host for Runtime<'_> {
    fn callee_pop(&self, target: u32) -> u32 {
        p3_render::ret_pop::ret_pop(target)
    }
    fn call_at(&mut self, esp: u32, m: &mut dyn p3_render::scene_math::Mem, target: u32, ecx: u32, args: &[u32]) -> u32 {
        // `esp` es la pila viva del llamante (locales por encima): el marco del llamado va por debajo.
        let saved = self.sp;
        if esp != 0 && esp < self.sp && esp > self.sp.wrapping_sub(0x10_0000) {
            self.sp = esp;
        }
        // Si los argumentos están tal cual en la pila del llamante (`esp`, `esp+4`...), el marco del llamado es el real: retorno en
        // `esp-4` y los argumentos encima, de modo que ve también los que el elevador no contó (p. ej. `sscanf` con más punteros o
        // las llamadas virtuales, que llegan sin argumentos deducidos: `anim.load(CString, id)` desde `button_base_load_layout`).
        // También cuando `esp` queda algo por encima de `self.sp` y faltan argumentos: el marco que `dispatch` dio al llamante deja 64 bytes de margen bajo
        // su entrada y un elevado con pocos locales llama sin bajar de ahí. Lo que hay bajo su `esp` está libre. Sin esto,
        // `window_draw_content` (`0x4B18F0`) llamaba al dibujo virtual de su animación (`+0x9C`, cuatro argumentos con la
        // posición) con un marco nuevo de un solo argumento: el fondo de las casillas de nombre de la partida nueva
        // (`edit_box_draw` `0x4C9DD0`) se pintaba en (0,0) y se oscurecía en cada repintado al pasar el ratón.
        // Solo si el llamado retira más argumentos de los que el elevador dedujo (`ret N`): con los contados basta el marco nuevo.
        let near = esp <= saved || (esp.wrapping_sub(saved) <= 0x2000 && p3_render::ret_pop::ret_pop(target) as usize > args.len());
        if esp != 0 && near && esp > saved.wrapping_sub(0x10_0000) && args.iter().enumerate().all(|(i, a)| m.r32(esp + 4 * i as u32) == *a) {
            self.force_frame = Some(esp.wrapping_sub(4));
        }
        let r = self.dispatch(m, target, ecx, args);
        self.sp = saved;
        r
    }
    fn call(&mut self, m: &mut dyn p3_render::scene_math::Mem, target: u32, ecx: u32, args: &[u32]) -> u32 {
        self.dispatch(m, target, ecx, args)
    }
}

/// Resultado de [`render_text`].
pub struct RenderedText {
    pub width: usize,
    pub height: usize,
    pub png_base64: String,
    /// Funciones del original que faltaron (deberían quedar vacías).
    pub missing: Vec<u32>,
}

/// Maqueta y dibuja `text` (con las marcas del juego) como `EventMessageWindow` (`0x004677B5`): `TextLayout` nuevo
/// (`0x004209E0`), fuente `font` en `+4` y `0x00420A10(CString, x+0x14, y, ancho-0x28, 0x14, color)` sobre pergamino.
/// Las líneas `\r` quedan a la izquierda de `x` como en el original (el desplazamiento de esa alineación es `-ancho`).
pub fn render_text(exe: &[u8], archive: &Archive<'_>, text: &[u8], width: u32, color: u32, font: u8) -> Result<RenderedText, String> {
    const PARCHMENT: [u8; 4] = [0xE8, 0xD8, 0xB0, 0xFF];
    let mut mem = PageMem::default();
    let mut rt = Runtime::new(&mut mem, exe, archive, width as usize, 8)?;
    let draw = |rt: &mut Runtime<'_>, mem: &mut PageMem, height: usize| {
        rt.surface = Surface::new(width as usize, height, PARCHMENT);
        let layout = rt.new_object(mem, 0x40);
        rt.call(mem, 0x0042_09E0, layout, &[]);
        mem.write(layout + 4, &[font]);
        let s = rt.cstring(mem, text);
        rt.call(mem, 0x0042_0A10, layout, &[s, 0x14, 0x0A, width.saturating_sub(0x28), 0x14, color])
    };
    // Primera pasada para conocer el alto (la función devuelve la Y final); segunda sobre la superficie justa.
    let bottom = draw(&mut rt, &mut mem, 4096) as i32;
    let height = (bottom.clamp(0, 4000) + 12).max(24) as usize;
    draw(&mut rt, &mut mem, height);
    if rt.aborted {
        return Err("el enlazador de interfaz abortó (límite de llamadas o memoria)".into());
    }
    let mut png = Vec::new();
    {
        let mut enc = png::Encoder::new(&mut png, width, height as u32);
        enc.set_color(png::ColorType::Rgba);
        enc.set_depth(png::BitDepth::Eight);
        let mut w = enc.write_header().map_err(|e| e.to_string())?;
        w.write_image_data(&rt.surface.rgba).map_err(|e| e.to_string())?;
    }
    Ok(RenderedText { width: width as usize, height, png_base64: base64(&png), missing: rt.missing.keys().copied().collect() })
}

fn base64(data: &[u8]) -> String {
    const T: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(data.len().div_ceil(3) * 4);
    for c in data.chunks(3) {
        let n = u32::from(c[0]) << 16 | u32::from(*c.get(1).unwrap_or(&0)) << 8 | u32::from(*c.get(2).unwrap_or(&0));
        for k in 0..4 {
            if k <= c.len() {
                out.push(char::from(T[(n >> (18 - 6 * k) & 63) as usize]));
            } else {
                out.push('=');
            }
        }
    }
    out
}

