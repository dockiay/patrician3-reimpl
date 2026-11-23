//! Menú principal y barra lateral del original sobre el enlazador (hilo «Montar el menú principal y la barra lateral», 12/11/2026).
//!
//! El flujo es el del juego: `p3_win_app_init_instance` (`0x00464980`) construye la ventana principal global `0x006CC3F0`
//! (`game_main_window_ctor` `0x00463180`, que lleva dentro el menú principal en `+0x44`), la inicializa (`0x00463240`: gráficos,
//! fuentes, cursores, `GameScreen`...) y entra en `game_main_window_run` (`0x00463D20`). Este abre el menú (`[vt+8]` = `main_menu_init`
//! `0x0042ECC0` con `./scripts/options.ini`) y su bucle modal (`[vt+0x120]` = `screen_window_run_modal_loop` `0x004B70C0`, que repite
//! `screen_list_run_frame` `0x004B8A40` hasta que una pantalla termina). Las elecciones del menú (`main_menu_process_selection`
//! `0x0042F4F0`) apilan la pantalla elegida en la lista `0x006DA5F0` y abren su propio bucle modal; al cargar o empezar partida,
//! `game_main_window_run` pasa a la pantalla de juego (`GameScreen`, barra lateral y menús laterales).
//!
//! Todo ese código está portado con paridad x86 (`p3_core::lifted`); aquí solo van las nativas que el enlazador necesita para que
//! corra sin el sistema: las de biblioteca (MFC) y las de tipos ya portados en `p3-core` sobre la memoria del original.

use p3_render::scene_math::Mem;

/// Ventana principal global (`CGameMainWindow`), construida por el CRT en el original.
pub const MAIN_WINDOW: u32 = 0x006C_C3F0;
/// Menú principal dentro de la ventana principal (`+0x44`).
pub const MAIN_MENU: u32 = MAIN_WINDOW + 0x44;
/// Lista de pantallas global.
pub const SCREEN_LIST: u32 = 0x006D_A5F0;

/// Construye la ventana principal y la inicializa como `p3_win_app_init_instance`; devuelve si la inicialización tuvo éxito.
pub fn boot(rt: &mut super::Runtime, mem: &mut super::PageMem) -> bool {
    construct_globals(rt, mem);
    rt.call(mem, 0x0046_3180, MAIN_WINDOW, &[]);
    rt.call(mem, 0x0046_3240, MAIN_WINDOW, &[]) & 0xFF != 0
}

/// Objetos globales del CRT (`__xc_a..__xc_z`, `0x00692004..0x0069288C`), en el orden de la tabla (como `_initterm`). Las entradas
/// `mov ecx, objeto; call ctor; push dtor; call atexit; pop ecx; ret` construyen su objeto con el constructor original; las demás
/// (vectores de objetos con `eh vector constructor iterator`, tablas virtuales escritas en línea, cadenas, ceros) se ejecutan enteras.
/// Sin ellas las pantallas estáticas tienen la tabla virtual a cero y las escenas recorren listas sin construir (la escena del puerto
/// `0x006E6220`, las hojas y animaciones de sprites `0x006E11E8`... de la vista de ciudad). `atexit` se ejecuta elevado (no hay salida).
/// Se saltan [`SKIP_CTORS`] y la biblioteca MFC/CRT (`>= 0x00639000`). Devuelve las entradas ejecutadas.
const SKIP_CTORS: [u32; 11] = [
    0x0046_3180, // game_main_window_ctor: lo llama `boot` después
    0x0046_46D0, // p3_win_app_ctor (CWinApp: el enlazador siembra el estado de módulo)
    0x004B_39A0, // class75_ctor (gancho del enlazador)
    0x004B_8740, // screen_list_ctor: ya lo construye `Runtime::new`
    0x004B_D290, // display_audio_config_ctor: configuración que fija el enlazador (800x600, ventana)
    0x004B_DE30, // ini_file_cache_ctor: caché de INI nativa
    0x004C_5750, // texture_collection_ctor: la siembra `Runtime::seed_textures`
    0x0050_3E30, // ships_construct
    0x0051_19B0, // name_bank_init
    0x0052_F550, // game_world_construct
    0x0054_6450, // operations_constructor (mundo, barcos y operaciones los siembra el enlazador; `initialize` espera ese estado)
];

pub fn construct_globals(rt: &mut super::Runtime, mem: &mut super::PageMem) -> usize {
    let mut done = 0;
    // `_cfltcvt_init` (`0x00639BBE`, desde `_fpmath` en `_cinit`): punteros del CRT para `printf` con coma flotante. Sin ellos siguen
    // en `_fptrap` (`0x00645AC2`, «R6002 floating point not loaded») y todo `%f`/`%.0f` salía vacío: la ventana de balance de las
    // estadísticas dejaba en blanco el valor de la empresa, mercancías vendidas y compradas (15/11/2026).
    for (slot, f) in [
        (0x006C_8748, 0x0063_F39A), // _cfltcvt
        (0x006C_874C, 0x0063_F044), // _cropzeros
        (0x006C_8750, 0x0063_F0AA), // _fassign
        (0x006C_8754, 0x0063_EFEA), // _forcdecpt
        (0x006C_8758, 0x0063_F092), // _positive
        (0x006C_875C, 0x0063_F39A), // _cfltcvt (otra entrada de la tabla)
    ] {
        mem.w32(slot, f);
    }
    for slot in (0x0069_2004..0x0069_288C).step_by(4) {
        let f = mem.u32_at(slot);
        if f == 0 || f >= 0x0063_9000 {
            continue;
        }
        if mem.u8_at(f) == 0xB9 && mem.u8_at(f + 5) == 0xE8 {
            let obj = mem.u32_at(f + 1);
            let ctor = (f + 10).wrapping_add(mem.u32_at(f + 6));
            // Los de biblioteca (`std::_Lockit`, `ios_base::Init`... `>= 0x00639000`) no hacen falta.
            if !SKIP_CTORS.contains(&ctor) && ctor < 0x0063_9000 {
                rt.call(mem, ctor, obj, &[]);
                done += 1;
            }
            continue;
        }
        if run_initializer(rt, mem, f) {
            done += 1;
        }
    }
    // De `game_world_construct` (`0x0052F550`, saltado: el mundo lo siembra el enlazador) sí hacen falta los 16 centros comerciales
    // de `mundo+0xB10` (`0x006DEFB0`, 0x2C bytes, ctor `0x00533810`): crean el nodo centinela de su lista (`+0x24`, cuenta `+0x28`).
    // Sin él la primera carga vacía una lista sin cabeza (cuenta −5) y una segunda carga desde las Opciones no termina nunca.
    // El ctor no está en el catálogo (no se eleva); se reproduce aquí: `+0x20` toma un byte sin iniciar de la pila (0 aquí) y el nodo
    // de 0x18 bytes se enlaza consigo mismo.
    for i in 0..16u32 {
        let tc = 0x006D_EFB0 + i * 0x2C;
        let node = rt.call(mem, 0x0064_F7B9, 0, &[0x18]);
        mem.w8(tc + 0x20, 0);
        mem.w32(node, node);
        mem.w32(node + 4, node);
        mem.w32(tc + 0x24, node);
        mem.w32(tc + 0x28, 0);
    }
    seed_operations_fields(mem);
    done
}

/// Campos propios de `operations_constructor` (`0x00546450`, saltado arriba; `p3_core::pump::TimeMode::default`) sobre el objeto
/// de operaciones `0x006DF2F0`, sin la base `operations_construct` (`0x0054A600`: la cola funciona sobre ceros en el enlazador).
/// Sin ellos `+0x91C` quedaba a 0 y `operations_save_game` (`0x005472F0`) rechazaba cada autoguardado `0xC2` («AUTO», encolado
/// por `operations_pump_time_advance_and_autosave` `0x00546620` al vencer `+0x940`): se reintentaba cada minuto sin escribir nada.
/// Cargar partida o empezar una nueva sobrescribe después reloj, modo e intervalo; `+0x91C`/`+0x920` solo los cambia la
/// operación `0xC8` (cambio de modo de tiempo con quinto campo distinto de `0xFFFFFFFF`).
pub fn seed_operations_fields(mem: &mut super::PageMem) {
    const OPS: u32 = 0x006D_F2F0;
    // Vector de 4 `CString` vacías en `+0x8E0` (`eh vector constructor iterator` `0x00639E9C` con `0x004103C0`, paso 8).
    let empty = mem.u32_at(0x006C_7CCC);
    for i in 0..4 {
        mem.w32(OPS + 0x8E0 + i * 8, empty);
    }
    // `operation_record_init` (`0x00535740`) sobre `+0x900`.
    mem.w32(OPS + 0x900, u32::MAX);
    for off in [0x904, 0x908, 0x90C, 0x910] {
        mem.w32(OPS + off, 0);
    }
    let ms = p3_core::pump::ms_per_tick(2);
    for (off, v) in [
        (0x914, 0), (0x918, 1), (0x92C, 0), (0x928, 0), (0x924, 0xFFFF), (0x930, 0), (0x934, 0xFFFF), (0x944, 0),
        (0x8D8, 0x19), (0x940, u32::MAX), (0x91C, 1), (0x920, 1), (0x8DC, ms), (0x8D4, ms),
    ] {
        mem.w32(OPS + off, v);
    }
}

/// Ejecuta un inicializador estático de código lineal (los del juego no están en el catálogo y no se elevan): `push`, `mov` entre
/// registros, inmediatos y globales (byte, palabra, doble palabra), `xor r,r`, `call` (los argumentos son lo apilado desde la
/// llamada anterior; `atexit` `0x0063978E` no se registra), `pop`/`push ecx` y `ret`. Ante cualquier otra instrucción se detiene
/// sin deshacer lo hecho y devuelve `false` (p. ej. los registros `atexit` con guarda `test`/`jne`, que no construyen nada).
fn run_initializer(rt: &mut super::Runtime, mem: &mut super::PageMem, f: u32) -> bool {
    let mut r = [0u32; 8];
    let mut pushed: Vec<u32> = Vec::new();
    let mut pc = f;
    for _ in 0..64 {
        let b = |o: u32| mem.u8_at(pc + o);
        let d = |o: u32| mem.u32_at(pc + o);
        // modrm `00 reg 101` + disp32: operando de memoria absoluto
        let abs = |m: u8| m & 0xC7 == 0x05;
        let reg = |m: u8| usize::from((m >> 3) & 7);
        match b(0) {
            0x68 => {
                pushed.push(d(1));
                pc += 5;
            }
            0x6A => {
                pushed.push(b(1) as i8 as i32 as u32);
                pc += 2;
            }
            0x50..=0x57 => {
                pushed.push(r[usize::from(b(0) - 0x50)]);
                pc += 1;
            }
            0x58..=0x5F => {
                pc += 1;
            }
            0xB8..=0xBF => {
                r[usize::from(b(0) - 0xB8)] = d(1);
                pc += 5;
            }
            0x31 | 0x33 if b(1) >> 6 == 3 && reg(b(1)) == usize::from(b(1) & 7) => {
                r[reg(b(1))] = 0;
                pc += 2;
            }
            0xA1 => {
                r[0] = mem.u32_at(d(1));
                pc += 5;
            }
            // The wall-shape initializer (0x581F40) copies direction bytes,
            // then increments four of them to create its end-piece codes.
            // Preserve the rest of the register, as MOV r8 does on x86.
            0xA0 => {
                r[0] = (r[0] & !0xFF) | u32::from(mem.u8_at(d(1)));
                pc += 5;
            }
            0x8A if abs(b(1)) && reg(b(1)) < 4 => {
                let i = reg(b(1));
                r[i] = (r[i] & !0xFF) | u32::from(mem.u8_at(d(2)));
                pc += 6;
            }
            0xFE if b(1) & 0xFC == 0xC0 => {
                let i = usize::from(b(1) & 3);
                r[i] = (r[i] & !0xFF) | u32::from((r[i] as u8).wrapping_add(1));
                pc += 2;
            }
            0x8B if abs(b(1)) => {
                r[reg(b(1))] = mem.u32_at(d(2));
                pc += 6;
            }
            0xA3 => {
                mem.set_u32(d(1), r[0]);
                pc += 5;
            }
            0x89 if abs(b(1)) => {
                mem.set_u32(d(2), r[reg(b(1))]);
                pc += 6;
            }
            0xC7 if b(1) == 0x05 => {
                mem.set_u32(d(2), d(6));
                pc += 10;
            }
            0xA2 => {
                mem.write(d(1), &[r[0] as u8]);
                pc += 5;
            }
            0x88 if abs(b(1)) && reg(b(1)) < 4 => {
                mem.write(d(2), &[r[reg(b(1))] as u8]);
                pc += 6;
            }
            0xC6 if b(1) == 0x05 => {
                mem.write(d(2), &[b(6)]);
                pc += 7;
            }
            0x66 => {
                let (b1, b2) = (b(1), b(2));
                let (at, v, n) = match b1 {
                    0xA3 => (d(2), r[0], 6),
                    0x89 if abs(b2) => (d(3), r[reg(b2)], 7),
                    0xC7 if b2 == 0x05 => (d(3), u32::from(b(7)) | u32::from(b(8)) << 8, 9),
                    _ => return false,
                };
                mem.write(at, &(v as u16).to_le_bytes());
                pc += n;
            }
            0xE8 => {
                let target = (pc + 5).wrapping_add(d(1));
                if target != 0x0063_978E {
                    let args: Vec<u32> = pushed.iter().rev().copied().collect();
                    r[0] = rt.call(mem, target, r[1], &args);
                }
                pushed.clear();
                pc += 5;
            }
            0xC3 => return true,
            _ => return false,
        }
    }
    false
}

/// `game_main_window_run`: el bucle del juego entero (menú principal, partidas, pantalla de juego). Vuelve al salir.
pub fn run(rt: &mut super::Runtime, mem: &mut super::PageMem) -> u32 {
    rt.call(mem, 0x0046_3D20, MAIN_WINDOW, &[])
}

/// Estado de `menus.rs` dentro del enlazador.
#[derive(Default)]
pub struct MenuState {
    /// `AfxGetModuleState`: bloque reservado la primera vez.
    pub module_state: u32,
    /// Ranuras TLS de Win32 (`TlsAlloc`/`TlsSetValue`): el enlazador es monohilo, así que son una tabla simple (las usa `CThreadSlotData` de MFC).
    tls: Vec<u32>,
    /// Guion por fotograma: se llama al entrar en `present_frame` (`0x004B8FF0`) con la memoria y el número de fotograma, para pruebas
    /// que eligen opciones del menú o de la barra lateral sin ratón. `false` corta la ejecución (como agotar `call_limit`).
    pub frame_script: Option<Box<dyn FnMut(&mut dyn Mem, u32) -> bool>>,
    /// Llamadas de prueba al original al entrar en `present_frame`: (fotograma, función, ecx, argumentos). Diagnóstico de bancos.
    /// Un `ecx` con el bit alto se lee del global `ecx & 0x7FFF_FFFF` (objetos creados en la partida).
    pub frame_calls: Vec<(u32, u32, u32, Vec<u32>)>,
    /// Fotogramas presentados desde el arranque.
    pub frames: u32,
    /// Carpeta del juego donde están `save/ein/*.pat` etc. (la de trabajo del original). Sin ella no hay partidas guardadas.
    pub game_dir: Option<std::path::PathBuf>,
    /// Búsquedas `CFileFind` abiertas: objeto → (rutas encontradas, índice de la actual).
    finds: std::collections::HashMap<u32, (Vec<std::path::PathBuf>, usize)>,
    /// Archivos abiertos con `CFile::Open` de MFC (guardar partida): manejador (`CFile+4`) → archivo de la carpeta del juego.
    cfiles: std::collections::HashMap<u32, Box<dyn crate::hostfs::HostFile>>,
    next_cfile: u32,
    /// Recorridos del contenedor (`arc_GetFirstFileStart`): iterador → (siguiente entrada, búfer del nombre).
    arc_iters: std::collections::HashMap<u32, (usize, u32)>,
    /// Archivos añadidos al contenedor de texturas (`arc_AppendArchive`): solo el CPR del enlazador.
    arc_appended: u32,
}

/// Contenedor de archivos que abre `sgl2d_open_texture_archives` (`0x004BC880`): un valor ficticio; el enlazador sirve su CPR.
pub const ARCHIVE_CONTAINER: u32 = 0x00A5_0001;

impl MenuState {
    /// Archivos de la carpeta del juego que casan con `pattern` (ruta relativa con `*` y `?` solo en el último tramo, sin distinguir
    /// mayúsculas, como NTFS), en orden alfabético.
    pub(super) fn find_files(&self, pattern: &str) -> Vec<std::path::PathBuf> {
        let Some(root) = &self.game_dir else { return Vec::new() };
        let pattern = pattern.replace('\\', "/");
        let pattern = pattern.trim_start_matches("./");
        let (dir, mask) = pattern.rsplit_once('/').unwrap_or(("", pattern));
        // Cada tramo de la carpeta se busca sin distinguir mayúsculas (`save/ein` frente a `Save/Ein`).
        let fs = crate::hostfs::current();
        let mut at = root.clone();
        for part in dir.split('/').filter(|p| !p.is_empty()) {
            let Some(e) = fs.find_ci(&at, part) else { return Vec::new() };
            at = e.path;
        }
        let Ok(entries) = fs.list(&at) else { return Vec::new() };
        let mask = mask.to_ascii_lowercase();
        let mut out: Vec<_> = entries
            .into_iter()
            .filter(|e| e.is_file && wildcard(mask.as_bytes(), e.name.to_ascii_lowercase().as_bytes()))
            .map(|e| e.path)
            .collect();
        out.sort_by_key(|p| p.file_name().map(|n| n.to_string_lossy().to_ascii_lowercase()));
        out
    }

    /// Ruta en la carpeta del juego de un archivo del original (`./save/ein/AUTO.pat`): cada carpeta y el archivo se buscan sin
    /// distinguir mayúsculas; si el archivo no existe se devuelve con el nombre pedido (para crearlo).
    pub(super) fn host_path(&self, name: &str) -> Option<std::path::PathBuf> {
        let root = self.game_dir.as_ref()?;
        let name = name.replace('\\', "/");
        let name = name.trim_start_matches("./");
        let (dir, file) = name.rsplit_once('/').unwrap_or(("", name));
        let fs = crate::hostfs::current();
        let mut at = root.clone();
        for part in dir.split('/').filter(|p| !p.is_empty() && *p != ".") {
            // Como antes, una carpeta que no se puede listar corta la búsqueda (`?`); una que no existe se arrastra con su nombre.
            let found = fs.list(&at).ok()?.into_iter().find(|e| e.name.eq_ignore_ascii_case(part));
            at = match found {
                Some(e) => e.path,
                None => at.join(part),
            };
        }
        let existing = fs.find_ci(&at, file);
        Some(existing.map_or_else(|| at.join(file), |e| e.path))
    }
}

/// `strftime` del CRT con las conversiones que usa el juego (`%d %m %Y %y %H %M %S %%`), en UTC.
fn strftime(fmt: &[u8], t: i64) -> Vec<u8> {
    let days = t.div_euclid(86_400);
    let secs = t.rem_euclid(86_400);
    // Días desde 1970-01-01 a fecha civil (algoritmo de H. Hinnant).
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + i64::from(month <= 2);
    let mut out = Vec::new();
    let mut i = 0;
    while i < fmt.len() {
        if fmt[i] == b'%' && i + 1 < fmt.len() {
            let v = match fmt[i + 1] {
                b'd' => format!("{day:02}"),
                b'm' => format!("{month:02}"),
                b'Y' => format!("{year}"),
                b'y' => format!("{:02}", year % 100),
                b'H' => format!("{:02}", secs / 3600),
                b'M' => format!("{:02}", secs / 60 % 60),
                b'S' => format!("{:02}", secs % 60),
                b'%' => "%".into(),
                c => format!("%{}", c as char),
            };
            out.extend_from_slice(v.as_bytes());
            i += 2;
        } else {
            out.push(fmt[i]);
            i += 1;
        }
    }
    out
}

/// `*` y `?` de Windows.
fn wildcard(mask: &[u8], name: &[u8]) -> bool {
    match (mask.first(), name.first()) {
        (None, None) => true,
        (Some(b'*'), _) => wildcard(&mask[1..], name) || (!name.is_empty() && wildcard(mask, &name[1..])),
        (Some(b'?'), Some(_)) => wildcard(&mask[1..], &name[1..]),
        (Some(a), Some(b)) => a == b && wildcard(&mask[1..], &name[1..]),
        _ => false,
    }
}

impl super::Runtime<'_> {
    /// Nativas del arranque y del menú. `None` = no es de aquí.
    pub(super) fn menu_native(&mut self, m: &mut dyn Mem, f: u32, ecx: u32, esp: u32) -> Option<u32> {
        let arg = |m: &mut dyn Mem, i: u32| m.r32(esp + 4 + 4 * i);
        if let Some(r) = self.graphics_stub(m, f, ecx, esp) {
            return Some(r);
        }
        // `present_frame` (`0x004B8FF0`): cuenta el fotograma y deja actuar al guion de prueba; después sigue el elevado.
        if f == 0x004B_8FF0 {
            self.menus.frames += 1;
            if let Some(mut script) = self.menus.frame_script.take() {
                let keep = script(m, self.menus.frames);
                self.menus.frame_script = Some(script);
                if !keep {
                    self.call_limit = 0;
                }
            }
            let frame = self.menus.frames;
            let calls: Vec<_> = self.menus.frame_calls.iter().filter(|c| c.0 == frame).cloned().collect();
            for (_, target, this, args) in calls {
                // `this` con el bit alto: la dirección de un global que guarda el objeto (p. ej. `0x8000_0000 | 0x6E54CC`).
                let this = if this & 0x8000_0000 != 0 {m.r32(this & 0x7fff_ffff)} else {this};
                let r = self.dispatch(m, target, this, &args);
                eprintln!("llamada de prueba f{frame} {target:#x}(ecx={this:#x}, {args:x?}) = {r:#x}");
            }
            return None;
        }
        Some(match f {
            // `lcg_random_ctor` / `_dtor` / `_set_seed` / `_next` (`0x0046C320..0x0046C350`): `p3_core::rng::LcgRandom` sobre `this`.
            0x0046_C320 => {
                let r = p3_core::rng::LcgRandom::new();
                m.w32(ecx, r.state);
                m.w32(ecx + 4, r.second);
                ecx
            }
            0x0046_C330 => 0,
            0x0046_C340 => {
                let mut r = p3_core::rng::LcgRandom { state: m.r32(ecx), second: m.r32(ecx + 4) };
                r.set_seed(arg(m, 0));
                m.w32(ecx, r.state);
                0
            }
            0x0046_C350 => {
                let mut r = p3_core::rng::LcgRandom { state: m.r32(ecx), second: m.r32(ecx + 4) };
                let v = r.next_u32();
                m.w32(ecx, r.state);
                v
            }
            // `set_autosave_interval_by_index` (`0x00549BA0`, `ret 4`): opción de autoguardado de la ventana de opciones.
            0x0054_9BA0 => {
                let a = [arg(m, 0)];
                p3_core::economy::small_fns::set_autosave_interval_by_index(&mut super::RenderMem(m), ecx, &a);
                0
            }
            // Gestor de sonido (`[0x006CBDB8]`): el audio del cliente va por `p3-audio` (J04); en el enlazador las órdenes de música y
            // volumen no suenan. Constructor `0x004419A0` deja el objeto a cero (lo reserva el llamante).
            // Con mezclador (cliente nativo con sonido) corre el original y Miles va por `miles.rs`.
            0x0044_19A0 if self.miles.mixer.is_none() => ecx,
            0x0044_1CD0 | 0x0044_1E10 | 0x0044_26C0 | 0x0044_2D90 | 0x0044_2E30 | 0x0044_2E90 | 0x0044_29D0 if self.miles.mixer.is_none() => 0,
            // Constructor de una pareja de `CString` vacías (`0x00430F60`, elementos del `eh vector constructor iterator` de la
            // pantalla de carga): `[this] = [this+4] = afxEmptyString` (`[0x006C7CCC]`).
            0x0043_0F60 => {
                let empty = m.r32(0x006C_7CCC);
                m.w32(ecx, empty);
                m.w32(ecx + 4, empty);
                ecx
            }
            // `CFixedAlloc::Alloc` (`0x006394FD`) de MFC: bloque de `[this]` bytes (los `CString` cortos); el montón no recicla.
            0x0063_94FD => {
                let size = m.r32(ecx).max(4);
                self.heap_alloc(m, size)
            }
            // CRT que el elevador no admite: `_memcmp` (`0x0063B3E0`), `_memchr` (`0x0063C070`) y `tolower` (`0x0063BF27`, configuración
            // regional "C": `[0x006F0224]` = 0), todas `cdecl`.
            0x0063_B3E0 => {
                let (a, b, n) = (arg(m, 0), arg(m, 1), arg(m, 2));
                (0..n)
                    .map(|i| i32::from(m.r8(a + i)) - i32::from(m.r8(b + i)))
                    .find(|d| *d != 0)
                    .unwrap_or(0) as u32
            }
            0x0063_C070 => {
                let (buf, val, n) = (arg(m, 0), arg(m, 1) as u8, arg(m, 2));
                (0..n).map(|i| buf + i).find(|a| m.r8(*a) == val).unwrap_or(0)
            }
            0x0063_BF27 => {
                let c = arg(m, 0);
                if (0x41..=0x5A).contains(&c) { c + 0x20 } else { c }
            }
            // `srand` (`0x00639BF6`): semilla del `rand` del CRT (`0x00639C03`, nativa de `mod.rs`).
            0x0063_9BF6 => {
                let seed = arg(m, 0);
                super::RAND.with(|r| r.set(seed));
                0
            }
            // `CFileFind` de MFC sobre la carpeta del juego ([`MenuState::game_dir`]): constructor `0x00638F07`, destructor `0x00638F44`,
            // `Close` `0x00638F80`, `FindFile(nombre, 0)` `0x00638FCA`, `FindNextFile` `0x006391EA` (cierto si quedan más tras la actual)
            // y `GetFileTitle(CString*)` `0x00639351`. El juego busca `binkplay.exe` y los vídeos de introducción (sin ellos no la
            // reproduce) y las partidas de la pantalla de carga (`./save/ein/*.pat`...). `+0xC` es el manejador de búsqueda.
            0x0063_8F07 => {
                let empty = m.r32(0x006C_7CCC);
                m.w32(ecx + 0x14, empty);
                m.w32(ecx, 0x0067_CE50);
                m.w32(ecx + 4, 0);
                m.w32(ecx + 8, 0);
                m.w32(ecx + 0xC, 0);
                m.w8(ecx + 0x18, 0x5C);
                ecx
            }
            // `CFile` de MFC (`+4` manejador, `+8` cerrar al destruir, `+0xC` nombre) sobre la carpeta del juego: lo usa guardar partida
            // (`load_save_screen_save_game_to_slot` `0x00433A80`, `operations_save` `0x005472F0`). `Open(nombre, modo, excepción)`
            // `0x00653C1D` (`ret 0xC`; modos `modeWrite` 1, `modeReadWrite` 2, `modeCreate` 0x1000, `modeNoTruncate` 0x2000),
            // `Write(búfer, n)` `0x00653D74` (`ret 8`), `Close` `0x00653E38` y el destructor `0x00653B60` (cierra si sigue abierto).
            0x0065_3C1D => {
                let (name, mode) = (arg(m, 0), arg(m, 1));
                let name = String::from_utf8_lossy(&super::cstr(m, name)).into_owned();
                m.w32(ecx + 8, 0);
                m.w32(ecx + 4, u32::MAX);
                let Some(path) = self.menus.host_path(&name) else { return Some(0) };
                let mut o = crate::hostfs::OpenMode::default();
                match mode & 3 {
                    0 => o.read = true,
                    1 => o.write = true,
                    _ => {
                        o.read = true;
                        o.write = true;
                    }
                }
                if mode & 0x1000 != 0 {
                    o.create = true;
                    if mode & 0x2000 == 0 {
                        o.truncate = true;
                    }
                }
                match crate::hostfs::current().open(&path, o) {
                    Ok(f) => {
                        self.menus.next_cfile += 1;
                        let h = 0x4000 + self.menus.next_cfile;
                        self.menus.cfiles.insert(h, f);
                        m.w32(ecx + 4, h);
                        m.w32(ecx + 8, 1);
                        1
                    }
                    Err(_) => 0,
                }
            }
            0x0065_3D74 => {
                use std::io::Write;
                let (buf, n) = (arg(m, 0), arg(m, 1));
                let data: Vec<u8> = (0..n).map(|i| m.r8(buf.wrapping_add(i))).collect();
                if let Some(f) = self.menus.cfiles.get_mut(&m.r32(ecx + 4)) {
                    let _ = f.write_all(&data);
                }
                0
            }
            0x0065_3E38 | 0x0065_3B60 => {
                let h = m.r32(ecx + 4);
                if h != u32::MAX && (f == 0x0065_3E38 || m.r32(ecx + 8) != 0) {
                    self.menus.cfiles.remove(&h);
                    m.w32(ecx + 4, u32::MAX);
                    m.w32(ecx + 8, 0);
                }
                0
            }
            0x0063_8F44 | 0x0063_8F80 => {
                self.menus.finds.remove(&ecx);
                m.w32(ecx + 0xC, 0);
                0
            }
            0x0063_8FCA => {
                let name = arg(m, 0);
                let pattern = if name == 0 { b"*.*".to_vec() } else { super::cstr(m, name) };
                let found = self.menus.find_files(&String::from_utf8_lossy(&pattern));
                let any = !found.is_empty();
                self.menus.finds.insert(ecx, (found, 0));
                m.w32(ecx + 0xC, u32::from(any));
                u32::from(any)
            }
            0x0063_91EA => match self.menus.finds.get_mut(&ecx) {
                Some((paths, next)) if *next < paths.len() => {
                    *next += 1;
                    u32::from(*next < paths.len())
                }
                _ => 0,
            },
            0x0063_9351 => {
                let out = arg(m, 0);
                let title = self
                    .menus
                    .finds
                    .get(&ecx)
                    .and_then(|(paths, next)| paths.get(next.wrapping_sub(1)))
                    .and_then(|p| p.file_stem())
                    .map(|t| t.to_string_lossy().into_owned().into_bytes())
                    .unwrap_or_default();
                self.cstring_set(m, out, &title);
                out
            }
            // `CFileFind::GetLastWriteTime(CTime&)` (`0x0063915B`, `ret 4`): fecha de modificación del archivo actual en segundos.
            0x0063_915B => {
                let out = arg(m, 0);
                let time = self
                    .menus
                    .finds
                    .get(&ecx)
                    .and_then(|(paths, next)| paths.get(next.wrapping_sub(1)))
                    .and_then(|p| crate::hostfs::current().metadata(p).ok())
                    .and_then(|md| md.modified_secs)
                    .map_or(0, |s| s as u32);
                m.w32(out, time);
                1
            }
            // `CFile::Remove(ruta)` (`0x00653F49`, `ret 4`) sobre la carpeta del juego ([`MenuState::host_path`], como `CFile::Open`): borrar partida
            // de la pantalla de carga (`load_save_screen_delete_savegame` `0x00433CC0`) pasa `./save/ein/NOMBRE.pat`. Sin carpeta del juego
            // se borra la ruta tal cual. Devuelve 1 si la borró; el original, al fallar, lanza `CFileException` (aquí 0).
            0x0065_3F49 => {
                let p = arg(m, 0);
                let name = String::from_utf8_lossy(&super::cstr(m, p)).into_owned();
                let path = self.menus.host_path(&name).unwrap_or_else(|| name.replace('\\', "/").into());
                u32::from(crate::hostfs::current().remove_file(&path).is_ok())
            }
            // `CTime::Format(CString*, formato)` (`0x0064E5B2`, `ret 8`): `strftime` sobre `localtime` del `time_t` de `this`. Aquí la
            // hora local es UTC (el enlazador no conoce la zona del sistema).
            0x0064_E5B2 => {
                let (out, fmt) = (arg(m, 0), arg(m, 1));
                let fmt = super::cstr(m, fmt);
                let text = strftime(&fmt, i64::from(m.r32(ecx)));
                self.cstring_set(m, out, &text);
                out
            }
            // `ini_file_cache_set_enabled` (`0x004BF110`): la caché INI `0x006DD524` es solo velocidad (guarda lo que leería
            // `GetPrivateProfileString`). El enlazador no construye su árbol (inicializador estático `0x004BDDF0`) y la deja
            // apagada (`+0x30` = 0): las lecturas van por `GetPrivateProfileStringA`/`GetPrivateProfileIntA` sobre el CPR.
            0x004B_F110 => 0,
            // `_strncmp(a, b, n)` (`0x00642B30`, cdecl).
            0x0064_2B30 => {
                let (a, b, n) = (arg(m, 0), arg(m, 1), arg(m, 2));
                let mut r = 0i32;
                for k in 0..n {
                    let (x, y) = (m.r8(a + k), m.r8(b + k));
                    if x != y {
                        r = if x < y { -1 } else { 1 };
                        break;
                    }
                    if x == 0 {
                        break;
                    }
                }
                r as u32
            }
            // `__aulldiv` / `__aullrem` (`0x006461A0` / `0x00646210`, `ret 0x10`): división de 64 bits sin signo. El enlazador solo
            // devuelve EAX; los usos del arranque (cifras de `sprintf`) caben en 32 bits.
            0x0064_61A0 | 0x0064_6210 => {
                let a = u64::from(arg(m, 0)) | (u64::from(arg(m, 1)) << 32);
                let b = u64::from(arg(m, 2)) | (u64::from(arg(m, 3)) << 32);
                if b == 0 {
                    return None;
                }
                (if f == 0x0064_61A0 { a / b } else { a % b }) as u32
            }
            // `ini_file_cache_read_value(sección, clave, buf, tamaño, fichero)` (`0x004BE1E0`, `ret 0x14`): con la caché encendida (como
            // en el original tras `initialize`) busca la clave exacta en el árbol del fichero ya analizado. Apagada, el original pasa a
            // `GetPrivateProfileStringA` y, si falla, a un recorrido del fichero del archivo que compara solo el prefijo de la clave
            // (`0x004BE579..0x004BE590`, `_strnicmp(clave, línea, strlen(clave))`): `Text` tomaba el valor de `TextColor`. Se responde
            // como la caché: valor de la clave exacta, recortado a `tamaño - 1`, y 1 si existe; 0 sin tocar `buf` si no.
            0x004B_E1E0 => {
                let (section, key, buf, size, file) = (arg(m, 0), arg(m, 1), arg(m, 2), arg(m, 3), arg(m, 4));
                let found = self.ini_value(m, file, section, key);
                if self.trace {
                    let name = |m: &mut dyn Mem, p: u32| String::from_utf8_lossy(&(0..80).map(|i| m.r8(p + i)).take_while(|&b| b != 0).collect::<Vec<u8>>()).into_owned();
                    eprintln!("    ini {} [{}] {} = {:?}", name(m, file), name(m, section), name(m, key), found.as_ref().map(|v| String::from_utf8_lossy(v).into_owned()));
                }
                match found {
                    Some(mut v) => {
                        v.truncate(size.saturating_sub(1) as usize);
                        for (i, b) in v.iter().enumerate() {
                            m.w8(buf + i as u32, *b);
                        }
                        m.w8(buf + v.len() as u32, 0);
                        1
                    }
                    None => 0,
                }
            }
            _ => return None,
        })
    }

    /// Arranque gráfico y ventana del sistema, provisional hasta que el hilo de la ventana nativa (`win32.rs`, `p3-client`) ponga los
    /// suyos: aquí devuelven éxito sin abrir nada, para que `game_main_window_initialize` siga con fuentes, cursores y pantallas.
    fn graphics_stub(&mut self, m: &mut dyn Mem, f: u32, _ecx: u32, esp: u32) -> Option<u32> {
        Some(match f {
            // `load_gl_cfg_and_init_graphics`, `sgl2d_init_graphics`: sin dispositivo. La configuración `0x006DCB70` la fija después
            // `game_main_window_initialize` (`0x004635CB..0x004635EE`: 800×600, 16 bits, ventana) sea cual sea `gl.cfg`.
            0x004B_C010 => 1,
            // `sgl2d_init_graphics`: sin controlador, pero deja como el original (`0x004BC696`, `0x004BC75C`) `0x006DCB8C` distinto de cero
            // (controlador cargado): `sgl2d_open_texture_archives` y `sgl2d_get_texture_archive` lo exigen para abrir el contenedor.
            0x004B_C680 => {
                m.w32(0x006D_CB8C, 1);
                1
            }
            // `game_window_create_main_window(config*, ancho, alto, título)`: sin ventana, pero guarda como el original
            // (`0x004C0A42..48`) la resolución pedida en `0x006DCCE0/E4`; en ventana llega a cero y la fija `set_ddraw_resolution`.
            0x004C_0A20 => {
                let (w, h) = (m.r32(esp + 8), m.r32(esp + 12));
                m.w32(0x006D_CCE0, w);
                m.w32(0x006D_CCE4, h);
                1
            }
            // `set_ddraw_resolution(hwnd, config*)`: sin DirectDraw, pero deja como el original (`0x004BCE5A..0x004BCE79`) el tamaño
            // de la configuración (`+8` ancho, `+0xC` alto) en `0x006DAB4C/44` y `0x006DCCE0/E4`. Las pantallas se centran con
            // `0x006DCCE0/E4` (`(ancho - 800) / 2`): a cero quedaban en (-400, -300).
            0x004B_CCC0 => {
                let cfg = m.r32(esp + 8);
                let (w, h) = (m.r32(cfg + 8), m.r32(cfg + 0xC));
                m.w32(0x006D_AB4C, w);
                m.w32(0x006D_AB44, h);
                m.w32(0x006D_CCE0, w);
                m.w32(0x006D_CCE4, h);
                1
            }
            // `AfxGetModuleState` (`0x00659981`): estado de módulo MFC; `+0xC` es la instancia (`0x00400000`).
            0x0065_9981 => {
                if self.menus.module_state == 0 {
                    self.menus.module_state = self.heap_alloc(m, 0x1074);
                    m.w32(self.menus.module_state + 0xC, 0x0040_0000);
                }
                self.menus.module_state
            }
            _ => return None,
        })
    }

    /// Importaciones de `Archiver.dll` y `SHLWAPI` que usan el contenedor de texturas y el buscador de archivos del CPR
    /// (`archive_file_finder_find_in_archive` `0x004AECD0`: recorre los nombres y filtra con `PathMatchSpecA`).
    pub(super) fn menu_import(&mut self, m: &mut dyn Mem, func: &str, arg: &dyn Fn(&mut dyn Mem, u32) -> u32) -> Option<u32> {
        Some(match func {
            "arc_CreateContainerEx" => ARCHIVE_CONTAINER,
            // `arc_AppendArchive(contenedor, ruta, ...)` se llama con `p2arch0`, `p2arch1`... hasta que falla: hay un solo CPR.
            "arc_AppendArchive" => {
                self.menus.arc_appended += 1;
                u32::from(self.menus.arc_appended == 1)
            }
            "arc_ReleaseContainer" => 1,
            // `CreateDirectoryA(ruta, seguridad)` (stdcall): guardar partida crea la carpeta según el modo (`Save\Kam` en campaña,
            // `Save\Ein` libre, `Save\Mehr` multijugador; ver save-container/README.md) antes de abrirla con `CFile::Open`. Sin ella
            // la partida de campaña no se escribía porque `save/kam` no existe. Sobre la carpeta del juego ([`MenuState::host_path`]),
            // sin recursión como el original; devuelve 0 si ya existía o falla (el juego no mira el resultado).
            "CreateDirectoryA" => {
                let p = arg(m, 0);
                let name = String::from_utf8_lossy(&super::cstr(m, p)).into_owned();
                let name = name.trim_end_matches(['\\', '/']);
                let Some(path) = self.menus.host_path(name) else { return Some(0) };
                let fs = crate::hostfs::current();
                if fs.exists(&path) {
                    0
                } else {
                    u32::from(fs.create_dir_all(&path).is_ok())
                }
            }
            "arc_GetFirstFileStart" => {
                let it = 0x00A6_0000 + self.menus.arc_iters.len() as u32;
                let buf = self.heap_alloc(m, 0x200);
                self.menus.arc_iters.insert(it, (0, buf));
                it
            }
            // Nombre siguiente (Latin-1, como en el CPR) o cadena vacía al acabar; el búfer se reutiliza (el llamante lo copia).
            "arc_GetNextFileName" => {
                let it = arg(m, 1);
                let Some(&(i, buf)) = self.menus.arc_iters.get(&it) else { return Some(0) };
                let entries = &self.assets.archive.entries;
                let (name, next) = match entries[i.min(entries.len())..].iter().position(|e| e.kept) {
                    Some(k) => (entries[i + k].name.clone(), i + k + 1),
                    None => (Vec::new(), entries.len()),
                };
                let n = name.len().min(0x1FF);
                for (k, b) in name[..n].iter().enumerate() {
                    m.w8(buf + k as u32, *b);
                }
                m.w8(buf + n as u32, 0);
                self.menus.arc_iters.insert(it, (next, buf));
                buf
            }
            "arc_GetFileNameEnd" => {
                self.menus.arc_iters.remove(&arg(m, 1));
                1
            }
            // `PathMatchSpecA(ruta, patrón)`: comodines `*` y `?` sin distinguir mayúsculas.
            "PathMatchSpecA" => {
                let read = |m: &mut dyn Mem, p: u32| (0..0x200).map(|k| m.r8(p + k)).take_while(|&b| b != 0).map(|b| b.to_ascii_lowercase()).collect::<Vec<u8>>();
                let (pn, ps) = (arg(m, 0), arg(m, 1));
                let (name, spec) = (read(m, pn), read(m, ps));
                u32::from(wildcard(&spec, &name))
            }
            // TLS y memoria Local/Global de Win32 (monohilo, bloques fijos: el «manejador» es el puntero). Los usa `CThreadSlotData` de MFC
            // (`AfxGetThreadState`, mapas de manejadores `CWnd::FromHandle`/`Detach`, `0x0065A268`...); sin ellos fallaban con `AfxThrowMemoryException`.
            "TlsAlloc" => {
                self.menus.tls.push(0);
                self.menus.tls.len() as u32 - 1
            }
            "TlsFree" => 1,
            "TlsGetValue" => self.menus.tls.get(arg(m, 0) as usize).copied().unwrap_or(0),
            "TlsSetValue" => {
                let (i, v) = (arg(m, 0) as usize, arg(m, 1));
                if i >= self.menus.tls.len() {
                    self.menus.tls.resize(i + 1, 0);
                }
                self.menus.tls[i] = v;
                1
            }
            "LocalAlloc" | "GlobalAlloc" => {
                let n = arg(m, 1);
                let at = self.heap_take(n);
                super::zero_fill(m, at, n);
                at
            }
            "LocalReAlloc" | "GlobalReAlloc" => {
                let (p, n) = (arg(m, 0), arg(m, 1));
                let old = self.heap_blocks.get(&p).copied().unwrap_or(0).min(n);
                let at = self.heap_take(n);
                for i in 0..n {
                    let b = if p != 0 && i < old { m.r8(p.wrapping_add(i)) } else { 0 };
                    m.w8(at.wrapping_add(i), b);
                }
                self.heap_free(m, p);
                at
            }
            "LocalFree" | "GlobalFree" => {
                { let p = arg(m, 0); self.heap_free(m, p); }
                0
            }
            "GlobalHandle" | "GlobalLock" => arg(m, 0),
            "GlobalUnlock" => 0,
            _ => return None,
        })
    }

    fn heap_alloc(&mut self, m: &mut dyn Mem, size: u32) -> u32 {
        let at = self.heap;
        self.heap = self.heap.wrapping_add((size.max(1) + 15) & !15);
        for i in 0..size {
            m.w8(at + i, 0);
        }
        at
    }
}

#[cfg(test)]
mod tests {
    /// Los campos sembrados coinciden con `operations_constructor` (`0x00546450`) portado y probado contra el x86
    /// (`p3_core::pump::TimeMode::default`, `tests/c08_time_mode_x86.rs`); `+0x91C` = 1 deja pasar el autoguardado.
    #[test]
    fn operations_fields_match_time_mode_default() {
        let mut mem = super::super::PageMem::default();
        super::seed_operations_fields(&mut mem);
        let d = p3_core::pump::TimeMode::default();
        let ops = 0x006D_F2F0;
        assert_eq!(mem.u32_at(ops + 0x91C), d.field_91c);
        assert_eq!(mem.u32_at(ops + 0x8D4), d.pump.ms_per_tick);
        assert_eq!(mem.u32_at(ops + 0x8DC), d.normal_ms);
        assert_eq!(mem.u32_at(ops + 0x8D8), d.pump.mode1_divisor);
        assert_eq!(mem.u32_at(ops + 0x940), d.pump.autosave_deadline);
        assert_eq!(mem.u32_at(ops + 0x944), d.pump.autosave_interval);
        assert_eq!(mem.u32_at(ops + 0x914), d.pump.running);
        assert_eq!(mem.u32_at(ops + 0x928), d.pump.hotseat);
        assert_eq!(mem.u32_at(ops + 0x92C), d.pump.mode);
        assert_eq!(mem.u32_at(ops + 0x920), 1);
        assert_eq!(mem.u32_at(ops + 0x900), u32::MAX);
    }
}
