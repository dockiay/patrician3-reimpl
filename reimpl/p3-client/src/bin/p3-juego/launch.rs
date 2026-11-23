//! Arranque sin argumentos, independiente de la carpeta de trabajo del acceso.
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

struct Game {
    directory: PathBuf,
    original: PathBuf,
    resources: PathBuf,
}

fn locate(directory: &Path) -> Result<Game, String> {
    let config_path = directory.join("p3-juego.ini");
    let config = match fs::read_to_string(&config_path) {
        Ok(text) => text,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => String::new(),
        Err(e) => return Err(format!("{}: {e}", config_path.display())),
    };
    let mut original = None;
    let mut resources = None;
    for line in config.lines() {
        let line = line.trim().trim_start_matches('\u{feff}');
        if line.is_empty() || line.starts_with(['#', ';', '[']) { continue; }
        let Some((key, value)) = line.split_once('=') else {
            return Err(format!("{}: línea inválida: {line}", config_path.display()));
        };
        let value = value.trim().trim_matches('"');
        if value.is_empty() { return Err(format!("{}: falta el valor de {key}", config_path.display())); }
        let path = directory.join(value);
        match key.trim() {
            "original_exe" => original = Some(path),
            "resources" => resources = Some(path),
            key => return Err(format!("{}: opción desconocida {key}", config_path.display())),
        }
    }
    let original = original.unwrap_or_else(|| directory.join("Patrician3.exe"));
    let resources = resources.unwrap_or_else(|| directory.join("p2arch0_eng.cpr"));
    for path in [&original, &resources] {
        if !path.is_file() {
            return Err(format!("Falta {}. Indica original_exe y resources en {}.", path.display(), config_path.display()));
        }
    }
    Ok(Game { directory: directory.into(), original, resources })
}

fn prepare_saves(directory: &Path) -> Result<(), String> {
    let saves = directory.join("save").join("ein");
    fs::create_dir_all(&saves).map_err(|e| format!("{}: {e}", saves.display()))?;
    let source = directory.join("partidas").join("AUTO.pat");
    if !source.is_file() { return Ok(()); }
    let destination = saves.join("AUTO.pat");
    // create_new conserva cualquier partida previa, también ante dos aperturas simultáneas.
    match OpenOptions::new().write(true).create_new(true).open(&destination) {
        Ok(mut output) => {
            let result = File::open(&source).and_then(|mut input| std::io::copy(&mut input, &mut output));
            if let Err(e) = result {
                drop(output);
                let _ = fs::remove_file(&destination);
                return Err(format!("No se pudo copiar {}: {e}", source.display()));
            }
        }
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {}
        Err(e) => return Err(format!("{}: {e}", destination.display())),
    }
    Ok(())
}

pub fn start() -> Result<(), String> {
    let executable = std::env::current_exe().map_err(|e| format!("Ruta del cliente: {e}"))?;
    let directory = executable.parent().ok_or("El cliente no tiene carpeta")?;
    let game = locate(directory)?;
    prepare_saves(directory)?;
    let log_path = directory.join("p3-juego.log");
    let log = File::create(&log_path).map_err(|e| format!("{}: {e}", log_path.display()))?;
    // El mismo modo nativo que usa el CMD. Un hijo con argumentos permite conservar
    // el registro de stderr sin cambiar handles globales ni ejecutar un intérprete.
    let mut command = Command::new(&executable);
    command.arg("menu").arg(&game.original).arg(&game.resources)
        .arg("--juego").arg(&game.directory).current_dir(&game.directory)
        .stdin(Stdio::null()).stdout(Stdio::null()).stderr(log);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x0800_0000); // CREATE_NO_WINDOW
    }
    let status = command.status().map_err(|e| format!("No se pudo abrir el cliente: {e}"))?;
    if !status.success() {
        return Err(format!("El juego terminó con {status}. Consulta {}.", log_path.display()));
    }
    Ok(())
}

pub fn show_error(error: &str) {
    // En una apertura desde el Explorador el error debe permanecer visible.
    if let Ok(executable) = std::env::current_exe() {
        if let Some(directory) = executable.parent() {
            if let Ok(mut log) = OpenOptions::new().create(true).append(true).open(directory.join("p3-juego.log")) {
                let _ = writeln!(log, "p3-juego: {error}");
            }
        }
    }
    #[cfg(windows)]
    {
        #[link(name = "user32")]
        extern "system" {
            fn MessageBoxW(window: *mut std::ffi::c_void, text: *const u16, caption: *const u16, flags: u32) -> i32;
        }
        let text: Vec<u16> = error.encode_utf16().chain(Some(0)).collect();
        let title: Vec<u16> = "Patrician III".encode_utf16().chain(Some(0)).collect();
        unsafe { MessageBoxW(std::ptr::null_mut(), text.as_ptr(), title.as_ptr(), 0x10); }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    static NEXT: AtomicU64 = AtomicU64::new(0);
    struct Temp(PathBuf);
    impl Temp {
        fn new() -> Self {
            let path = std::env::temp_dir().join(format!("p3 direct launch {} {}", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed)));
            fs::create_dir(&path).unwrap();
            Self(path)
        }
    }
    impl Drop for Temp { fn drop(&mut self) { let _ = fs::remove_dir_all(&self.0); } }

    #[test]
    fn config_resolves_resources_beside_executable_with_spaces() {
        let temp = Temp::new();
        fs::create_dir(temp.0.join("original data")).unwrap();
        fs::write(temp.0.join("original data/game.exe"), b"exe").unwrap();
        fs::write(temp.0.join("original data/data.cpr"), b"cpr").unwrap();
        fs::write(temp.0.join("p3-juego.ini"), "original_exe=original data/game.exe\nresources=original data/data.cpr\n").unwrap();
        let game = locate(&temp.0).unwrap();
        assert_eq!(game.original, temp.0.join("original data/game.exe"));
        assert_eq!(game.resources, temp.0.join("original data/data.cpr"));
        assert_eq!(game.directory, temp.0);
    }

    #[test]
    fn first_launch_seeds_save_but_never_overwrites_progress() {
        let temp = Temp::new();
        fs::create_dir(temp.0.join("partidas")).unwrap();
        fs::write(temp.0.join("partidas/AUTO.pat"), b"initial save").unwrap();
        prepare_saves(&temp.0).unwrap();
        let destination = temp.0.join("save/ein/AUTO.pat");
        assert_eq!(fs::read(&destination).unwrap(), b"initial save");
        fs::write(&destination, b"player progress").unwrap();
        prepare_saves(&temp.0).unwrap();
        assert_eq!(fs::read(destination).unwrap(), b"player progress");
    }

    #[test]
    fn missing_resource_points_to_configuration() {
        let temp = Temp::new();
        fs::write(temp.0.join("Patrician3.exe"), b"exe").unwrap();
        let error = locate(&temp.0).err().unwrap();
        assert!(error.contains("p2arch0_eng.cpr"));
        assert!(error.contains("p3-juego.ini"));
    }
}
