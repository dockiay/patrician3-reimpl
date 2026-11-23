//! Idioma del juego en el cliente nativo.
//!
//! Cada EXE lleva su idioma fijo: el inglés pide `p2arch%d_eng.cpr` y `scripts/*_eng.*`; el español, `_spa`. Las cadenas de
//! interfaz están en el EXE, y el código elevado de `ui_rt` está atado al EXE inglés. Por eso el idioma del cliente es el del
//! **CPR**: se parte del CPR inglés (con el que corre el EXE) y, para otro idioma, se genera una vez
//! `idioma/p2arch0_<código>.cpr` con las entradas del CPR de ese idioma (los `_<código>.` pasan a `_eng.`, que es lo que pide el
//! EXE inglés) más las entradas que solo tiene el inglés (fuentes, etc.). Misiones, nombres de ciudades y barcos, créditos y
//! tutorial salen en ese idioma; los menús siguen en inglés. La elección se guarda en `idioma.txt` y se aplica al reiniciar.
use p3_assets::cpr::{self, Archive, Bytes};
pub use p3_mvp::lang_cpr::{display_name, extend_font, font_cells, rename_entry, EXTRA_CHARSET};
use p3_mvp::lang_cpr::Source;
use std::path::{Path, PathBuf};

/// Código del CPR con el que corre el EXE.
pub use p3_mvp::lang_cpr::BASE;
pub const CHOICE_FILE: &str = "idioma.txt";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Idioma {
    pub code: String,
    pub name: String,
    pub cpr: PathBuf,
}

/// `p2arch0_<código>.cpr` en las carpetas dadas (la primera que lo tenga gana), con el base primero y el resto por código.
pub fn discover(dirs: &[&Path]) -> Vec<Idioma> {
    let mut found: Vec<Idioma> = Vec::new();
    for dir in dirs {
        let Ok(read) = std::fs::read_dir(dir) else { continue };
        for entry in read.flatten() {
            let name = entry.file_name().to_string_lossy().to_lowercase();
            let Some(code) = name.strip_prefix("p2arch0_").and_then(|r| r.strip_suffix(".cpr")) else { continue };
            if code.is_empty() || code.len() > 8 || !code.chars().all(|c| c.is_ascii_alphanumeric()) || found.iter().any(|i| i.code == code) {
                continue;
            }
            found.push(Idioma { code: code.to_owned(), name: display_name(code), cpr: entry.path() });
        }
    }
    found.sort_by(|a, b| (a.code != BASE, &a.code).cmp(&(b.code != BASE, &b.code)));
    found
}

pub fn read_choice(dir: &Path) -> Option<String> {
    let text = std::fs::read_to_string(dir.join(CHOICE_FILE)).ok()?;
    let code = text.trim().to_lowercase();
    (!code.is_empty()).then_some(code)
}

fn stamp(path: &Path) -> String {
    let meta = std::fs::metadata(path).ok();
    let mtime = meta.as_ref().and_then(|m| m.modified().ok()).and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok()).map_or(0, |d| d.as_secs());
    format!("{}:{}", meta.map_or(0, |m| m.len()), mtime)
}

/// CPR que debe cargar el juego para `choice`. `base` es el CPR inglés indicado; `dirs` donde buscar el del otro idioma. Con el
/// idioma base (o si no hay CPR de ese idioma) devuelve `base`. El CPR derivado se guarda en `<game_dir>/idioma/` y solo se
/// regenera si cambia alguno de sus dos orígenes.
pub fn effective_cpr(game_dir: &Path, base: &Path, choice: &str, dirs: &[&Path]) -> Result<PathBuf, String> {
    if choice == BASE {
        return Ok(base.to_path_buf());
    }
    let Some(lang) = discover(dirs).into_iter().find(|i| i.code == choice) else {
        eprintln!("idioma {choice}: no hay p2arch0_{choice}.cpr; se usa el base");
        return Ok(base.to_path_buf());
    };
    let out_dir = game_dir.join("idioma");
    let out = out_dir.join(format!("p2arch0_{choice}.cpr"));
    let marker = out_dir.join(format!("p2arch0_{choice}.stamp"));
    let want = format!("v2 {} {}", stamp(base), stamp(&lang.cpr));
    if out.is_file() && std::fs::read_to_string(&marker).is_ok_and(|s| s.trim() == want) {
        return Ok(out);
    }
    eprintln!("idioma {choice}: preparando {} (una vez)", out.display());
    let base_bytes = Bytes::map_file(base).map_err(|e| format!("{}: {e}", base.display()))?;
    let lang_bytes = Bytes::map_file(&lang.cpr).map_err(|e| format!("{}: {e}", lang.cpr.display()))?;
    let base_archive = Archive::parse(&base_bytes).map_err(|e| format!("{}: {e:?}", base.display()))?;
    let lang_archive = Archive::parse(&lang_bytes).map_err(|e| format!("{}: {e:?}", lang.cpr.display()))?;
    let files: Vec<(Vec<u8>, std::borrow::Cow<[u8]>)> = p3_mvp::lang_cpr::plan(&base_archive, &lang_archive, choice)
        .into_iter()
        .filter_map(|(name, src)| {
            let data: std::borrow::Cow<[u8]> = match src {
                Source::Base(e) => base_archive.read(&e)?.into(),
                Source::Lang(e) => lang_archive.read(&e)?.into(),
                Source::Owned(v) => v.into(),
            };
            Some((name, data))
        })
        .collect();
    let refs: Vec<(&[u8], &[u8])> = files.iter().map(|(n, d)| (n.as_slice(), d.as_ref())).collect();
    let built = cpr::build(&refs).map_err(|e| format!("CPR derivado: {e}"))?;
    std::fs::create_dir_all(&out_dir).map_err(|e| format!("{}: {e}", out_dir.display()))?;
    let tmp = out.with_extension("tmp");
    std::fs::write(&tmp, &built).map_err(|e| format!("{}: {e}", tmp.display()))?;
    std::fs::rename(&tmp, &out).map_err(|e| format!("{}: {e}", out.display()))?;
    std::fs::write(&marker, format!("{want}\n")).map_err(|e| format!("{}: {e}", marker.display()))?;
    Ok(out)
}

/// Idioma ya resuelto al arrancar: el CPR que hay que abrir y lo que necesita el selector de Opciones.
pub struct Setup {
    /// CPR con el que arrancar (el base o el derivado).
    pub cpr: PathBuf,
    pub code: String,
    /// (código, nombre) de los idiomas disponibles; vacío o uno solo = sin selector.
    pub options: Vec<(String, String)>,
    pub active: usize,
    /// Dónde guarda el selector la elección (`idioma.txt`).
    pub choice_file: PathBuf,
    /// Carpetas donde buscar `idioma/<código>.pack`.
    pub pack_dirs: Vec<PathBuf>,
}

/// Elige el idioma (`wanted`, si no `idioma.txt` de `game_dir`, si no el base) y prepara su CPR. `exe` y `cpr` son los indicados al
/// arrancar (el CPR inglés); los CPR de otros idiomas se buscan junto al CPR, en `game_dir`, junto al EXE y en `extra` (y ahí también
/// `idioma/<código>.pack`). Como `p3-juego menu`.
pub fn prepare(exe: &Path, cpr: &Path, game_dir: &Path, wanted: Option<&str>, extra: &[PathBuf]) -> Setup {
    let cpr_dir = cpr.parent().map(PathBuf::from).unwrap_or_default();
    let mut dirs: Vec<&Path> = vec![&cpr_dir, game_dir];
    if let Some(exe_dir) = exe.parent() {
        dirs.push(exe_dir);
    }
    dirs.extend(extra.iter().map(PathBuf::as_path));
    let available = discover(&dirs);
    let wanted = wanted.map(str::to_lowercase).or_else(|| read_choice(game_dir)).unwrap_or_else(|| BASE.into());
    let code = if available.iter().any(|l| l.code == wanted) { wanted } else { BASE.into() };
    let effective = match effective_cpr(game_dir, cpr, &code, &dirs) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("idioma: {e}");
            cpr.to_path_buf()
        }
    };
    let code = if effective == cpr && code != BASE { BASE.to_string() } else { code };
    let active = available.iter().position(|l| l.code == code).unwrap_or(0);
    let mut pack_dirs = vec![game_dir.to_path_buf()];
    pack_dirs.extend(exe.parent().map(PathBuf::from));
    pack_dirs.push(cpr_dir.clone());
    pack_dirs.extend(extra.iter().cloned());
    Setup {
        cpr: effective,
        code,
        options: available.iter().map(|l| (l.code.clone(), l.name.clone())).collect(),
        active,
        choice_file: game_dir.join(CHOICE_FILE),
        pack_dirs,
    }
}

/// Después de crear el `Runtime`: carga el paquete de cadenas del EXE (`idioma/<código>.pack`) y activa el selector de Opciones.
pub fn apply(rt: &mut p3_mvp::ui_rt::Runtime<'_>, mem: &mut p3_mvp::ui_rt::PageMem, setup: &Setup) {
    if setup.code != BASE {
        let code = &setup.code;
        let pack = setup.pack_dirs.iter().flat_map(|d| [d.join("idioma").join(format!("{code}.pack")), d.join(format!("{code}.pack"))]).find_map(|f| std::fs::read_to_string(f).ok());
        match pack {
            Some(text) => {
                let (n, skipped, tables) = rt.load_language_pack(mem, &text);
                eprintln!("idioma {code}: {n} cadenas traducidas ({skipped} descartadas), {tables} tablas");
            }
            None => eprintln!("idioma {code}: no hay idioma/{code}.pack; menús y mensajes del EXE siguen en inglés"),
        }
    }
    if setup.options.len() > 1 {
        rt.language.enable(setup.options.clone(), setup.active, Some(setup.choice_file.clone()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn script_names_follow_the_exe_language() {
        assert_eq!(rename_entry(b"scripts\\Missions_SPA.ini", "spa"), b"scripts\\Missions_eng.ini");
        assert_eq!(rename_entry(b"scripts\\tooltips.txt", "spa"), b"scripts\\tooltips.txt");
    }

    #[test]
    fn derived_cpr_prefers_the_selected_language_and_keeps_missing_entries() {
        let dir = std::env::temp_dir().join(format!("p3-idioma-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let eng = cpr::build(&[(b"scripts\\credits_eng.ini", b"english"), (b"fonts\\only.aim", b"font"), (b"images\\same.bmp", b"E")]).unwrap();
        let spa = cpr::build(&[(b"scripts\\credits_spa.ini", b"espanol"), (b"images\\same.bmp", b"S")]).unwrap();
        std::fs::write(dir.join("p2arch0_eng.cpr"), eng).unwrap();
        std::fs::write(dir.join("p2arch0_spa.cpr"), spa).unwrap();
        let langs = discover(&[&dir]);
        assert_eq!(langs.iter().map(|l| l.code.as_str()).collect::<Vec<_>>(), ["eng", "spa"]);
        let out = effective_cpr(&dir, &dir.join("p2arch0_eng.cpr"), "spa", &[&dir]).unwrap();
        let bytes = Bytes::map_file(&out).unwrap();
        let a = Archive::parse(&bytes).unwrap();
        let get = |n: &[u8]| a.find(n).and_then(|e| a.read(e)).map(<[u8]>::to_vec);
        assert_eq!(get(b"scripts/credits_eng.ini").as_deref(), Some(&b"espanol"[..]));
        assert_eq!(get(b"images/same.bmp").as_deref(), Some(&b"S"[..]));
        assert_eq!(get(b"fonts/only.aim").as_deref(), Some(&b"font"[..]));
        // Segunda llamada: reutiliza el archivo.
        assert_eq!(effective_cpr(&dir, &dir.join("p2arch0_eng.cpr"), "spa", &[&dir]).unwrap(), out);
        assert_eq!(effective_cpr(&dir, &dir.join("p2arch0_eng.cpr"), "eng", &[&dir]).unwrap(), dir.join("p2arch0_eng.cpr"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// Con los CPR reales (`P3_IDIOMA_DIR` = carpeta con `p2arch0_eng.cpr` y `p2arch0_spa.cpr`).
    #[test]
    fn real_spanish_cpr_serves_spanish_scripts_under_english_names() {
        let Some(dir) = std::env::var_os("P3_IDIOMA_DIR").map(PathBuf::from) else { return };
        let work = std::env::var_os("P3_IDIOMA_KEEP").map_or_else(|| std::env::temp_dir().join(format!("p3-idioma-real-{}", std::process::id())), PathBuf::from);
        let out = effective_cpr(&work, &dir.join("p2arch0_eng.cpr"), "spa", &[&dir]).unwrap();
        let bytes = Bytes::map_file(&out).unwrap();
        let a = Archive::parse(&bytes).unwrap();
        let spa = Bytes::map_file(dir.join("p2arch0_spa.cpr")).unwrap();
        let s = Archive::parse(&spa).unwrap();
        let get = |a: &Archive<'_>, n: &[u8]| a.find(n).and_then(|e| a.read(e)).map(<[u8]>::to_vec);
        for (eng, real) in [("scripts/credits_eng.ini", "scripts/credits_spa.ini"), ("scripts/missions_eng.ini", "scripts/missions_spa.ini"), ("scripts/NamenStaedte_eng.txt", "scripts/NamenStaedte_spa.txt")] {
            assert!(get(&s, real.as_bytes()).is_some(), "{real}");
            assert_eq!(get(&a, eng.as_bytes()), get(&s, real.as_bytes()), "{eng}");
        }
        assert!(get(&a, b"fonts/elgreco24.aim").is_some(), "falta la fuente que solo trae el CPR inglés");
        eprintln!("{} entradas, {} bytes", a.entries.len(), bytes.len());
        if std::env::var_os("P3_IDIOMA_KEEP").is_none() {
            let _ = std::fs::remove_dir_all(&work);
        }
    }
}
