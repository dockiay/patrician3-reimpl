//! Precarga de archivos en la caché del sistema (19/11/2026, «congelados» de Sergio).
//!
//! El cliente proyecta en memoria el `.cpr` (`Bytes::map_file`) y su propio ejecutable (unos 30 MB de código elevado): cada página
//! se lee del disco la primera vez que se toca. En el PC de Sergio `D:\AI` está en un disco duro mecánico; con otra compilación
//! usándolo a la vez, cada fallo de página costaba decenas o cientos de ms y el registro de tirones mostraba el juego parado en
//! funciones triviales (`dialog_parchment_reset_state` 20 s al arrancar, `arc_CreateFile` 25 s al cargar, ventanas de 0,3–2,7 s).
//! Un hilo lee esos archivos de principio a fin al arrancar (lectura secuencial, la que mejor lleva un disco duro): después los
//! fallos de página salen de la caché en memoria. No cambia nada del juego. `P3_SIN_PRECARGA=1` lo apaga.
//!
//! 22/11/2026 («la carga tarda mucho más»): con `D:` saturado por otro proceso (200–300 % de tiempo de disco, listar una carpeta
//! tardaba minutos) el juego se quedaba de 5 a 20 s en `sgl_SwapBuffer` varios fotogramas seguidos tras empezar la partida, y
//! solo con el código nuevo encendido. Leer el ejecutable como archivo no basta: Windows pagina la imagen cargada (`SEC_IMAGE`)
//! por su cuenta, sin pasar por la caché de archivos, así que el primer uso de cada página de código seguía yendo al disco.
//! Ahora el hilo toca primero todas las páginas de la imagen ya cargada ([`touch_image`]) y, al final, lee la música suelta
//! (`Musik\...`, que `AIL_open_stream` leía del disco en el hilo del juego la primera vez) y las carpetas pequeñas que se le den.

use std::io::Read;
use std::path::PathBuf;

/// Toca una vez cada página de la imagen del ejecutable ya cargada en memoria, para que el código se lea del disco ahora (en
/// este hilo) y no la primera vez que el juego lo ejecute. Solo lee; devuelve los bytes recorridos.
#[cfg(windows)]
pub fn touch_image() -> u64 {
    #[repr(C)]
    struct Mbi {
        base: *const u8,
        alloc_base: *const u8,
        alloc_protect: u32,
        partition: u16,
        size: usize,
        state: u32,
        protect: u32,
        kind: u32,
    }
    extern "system" {
        fn GetModuleHandleW(name: *const u16) -> *const u8;
        fn VirtualQuery(addr: *const u8, info: *mut Mbi, len: usize) -> usize;
    }
    const MEM_COMMIT: u32 = 0x1000;
    const PAGE_NOACCESS: u32 = 0x01;
    const PAGE_GUARD: u32 = 0x100;
    let mut total = 0u64;
    // SAFETY: se recorre solo la reserva de la propia imagen, y solo se leen páginas que `VirtualQuery` da por confirmadas y
    // legibles; la imagen del proceso no se descarga mientras corre.
    unsafe {
        let base = GetModuleHandleW(std::ptr::null());
        if base.is_null() {
            return 0;
        }
        let mut at = base;
        loop {
            let mut info: Mbi = std::mem::zeroed();
            if VirtualQuery(at, &mut info, std::mem::size_of::<Mbi>()) == 0 || info.alloc_base != base || info.size == 0 {
                break;
            }
            if info.state == MEM_COMMIT && info.protect & (PAGE_NOACCESS | PAGE_GUARD) == 0 && info.protect != 0 {
                let mut off = 0usize;
                while off < info.size {
                    std::hint::black_box(std::ptr::read_volatile(info.base.add(off)));
                    off += 4096;
                }
                total += info.size as u64;
            }
            at = info.base.add(info.size);
        }
    }
    total
}

#[cfg(not(windows))]
pub fn touch_image() -> u64 {
    0
}

/// Archivos de `dirs` (y de sus subcarpetas, hasta tres niveles) con alguna de las extensiones dadas, en orden de nombre.
pub fn folder_files(dirs: &[PathBuf], exts: &[&str]) -> Vec<PathBuf> {
    fn walk(dir: &std::path::Path, depth: u32, exts: &[&str], out: &mut Vec<PathBuf>) {
        let Ok(rd) = std::fs::read_dir(dir) else { return };
        let mut entries: Vec<PathBuf> = rd.flatten().map(|e| e.path()).collect();
        entries.sort();
        for p in entries {
            if p.is_dir() {
                if depth < 3 {
                    walk(&p, depth + 1, exts, out);
                }
            } else if p.extension().is_some_and(|x| exts.iter().any(|e| x.eq_ignore_ascii_case(e))) {
                out.push(p);
            }
        }
    }
    let mut out = Vec::new();
    for d in dirs {
        walk(d, 0, exts, &mut out);
    }
    out
}

/// Lee en otro hilo los archivos dados (los que existan, sin repetir) y escribe en stderr cuánto tardó. Antes toca la imagen del
/// propio ejecutable ([`touch_image`]). `later` son carpetas cuyos archivos se buscan y leen al final, ya en el hilo (música).
pub fn warm(paths: Vec<PathBuf>) {
    warm_with(paths, Vec::new());
}

/// [`warm`] y, después, los archivos de `later` (carpeta, extensiones): se listan ya dentro del hilo, sin frenar el arranque.
pub fn warm_with(paths: Vec<PathBuf>, later: Vec<(PathBuf, &'static [&'static str])>) {
    if std::env::var_os("P3_SIN_PRECARGA").is_some() {
        return;
    }
    let mut list: Vec<PathBuf> = Vec::new();
    for p in paths {
        let p = std::fs::canonicalize(&p).unwrap_or(p);
        if p.is_file() && !list.contains(&p) {
            list.push(p);
        }
    }
    let _ = std::thread::Builder::new().name("p3-precarga".into()).spawn(move || {
        let t = std::time::Instant::now();
        let image = touch_image();
        if image != 0 {
            eprintln!("precarga: imagen del ejecutable, {} MiB en {:.1} s", image >> 20, t.elapsed().as_secs_f64());
        }
        let mut buf = vec![0u8; 4 << 20];
        let mut read_all = |list: &[PathBuf]| {
            let mut total = 0u64;
            for p in list {
                if let Ok(mut f) = std::fs::File::open(p) {
                    while let Ok(n) = f.read(&mut buf) {
                        if n == 0 {
                            break;
                        }
                        total += n as u64;
                    }
                }
            }
            total
        };
        let total = read_all(&list);
        eprintln!("precarga: {} archivos, {} MiB en {:.1} s", list.len(), total >> 20, t.elapsed().as_secs_f64());
        if !later.is_empty() {
            let mut extra = Vec::new();
            for (dir, exts) in &later {
                extra.extend(folder_files(std::slice::from_ref(dir), exts));
            }
            extra.dedup();
            let total = read_all(&extra);
            eprintln!("precarga: {} archivos sueltos (música y demás), {} MiB, a los {:.1} s", extra.len(), total >> 20, t.elapsed().as_secs_f64());
        }
    });
}

/// Archivos que conviene precargar: el propio ejecutable, el EXE original, el `.cpr` y los `.cpr`/`sound.cp0` de su carpeta y de
/// la carpeta del juego (paquete de idioma, sonidos).
pub fn files(exe: Option<&str>, cpr: Option<&str>, game_dir: Option<&std::path::Path>) -> Vec<PathBuf> {
    let mut out: Vec<PathBuf> = std::env::current_exe().ok().into_iter().collect();
    out.extend(exe.map(PathBuf::from));
    out.extend(cpr.map(PathBuf::from));
    let dirs = cpr.and_then(|c| std::path::Path::new(c).parent().map(PathBuf::from)).into_iter().chain(game_dir.map(PathBuf::from));
    for d in dirs {
        if let Ok(rd) = std::fs::read_dir(&d) {
            let mut extra: Vec<PathBuf> = rd
                .flatten()
                .map(|e| e.path())
                .filter(|p| p.extension().is_some_and(|x| x.eq_ignore_ascii_case("cpr") || x.eq_ignore_ascii_case("cp0")))
                .collect();
            extra.sort();
            out.extend(extra);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    #[test]
    fn touch_image_reads_the_loaded_image() {
        let n = super::touch_image();
        assert_eq!(n != 0, cfg!(windows), "{n}");
        assert_eq!(n % 4096, 0);
    }

    #[test]
    fn lists_cpr_folder_archives_once() {
        let dir = std::env::temp_dir().join(format!("p3-precarga-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        for n in ["p2arch0_eng.cpr", "p2arch0_spa.CPR", "sound.cp0", "otro.txt"] {
            std::fs::write(dir.join(n), b"x").unwrap();
        }
        let cpr = dir.join("p2arch0_eng.cpr");
        let files = super::files(None, cpr.to_str(), Some(&dir));
        let names: Vec<String> = files.iter().filter_map(|p| p.file_name()).map(|n| n.to_string_lossy().into_owned()).collect();
        assert!(names.contains(&"sound.cp0".to_string()) && names.contains(&"p2arch0_spa.CPR".to_string()), "{names:?}");
        assert!(!names.contains(&"otro.txt".to_string()));
        std::fs::create_dir_all(dir.join("Musik/Atmos")).unwrap();
        std::fs::write(dir.join("Musik/Atmos/a.MP3"), b"x").unwrap();
        std::fs::write(dir.join("Musik/leeme.txt"), b"x").unwrap();
        let music = super::folder_files(&[dir.join("Musik"), dir.join("no-existe")], &["mp3", "wav"]);
        assert_eq!(music.len(), 1, "{music:?}");
        super::warm_with(files, vec![(dir.join("Musik"), &["mp3", "wav"])]);
        std::thread::sleep(std::time::Duration::from_millis(200));
        let _ = std::fs::remove_dir_all(&dir);
    }
}
