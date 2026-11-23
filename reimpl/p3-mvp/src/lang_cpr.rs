//! CPR de otro idioma para el EXE inglés (común al cliente nativo, `p3-stream` y `p3-web`).
//!
//! El EXE inglés pide `scripts/*_eng.*` y dibuja con tiras de glifos sin ñ ¿ ¡. Para jugar con los textos de otro idioma se combina
//! su CPR con el inglés: las entradas del idioma (con `_<código>.` → `_eng.`) más las que solo tiene el inglés (fuentes), y, en
//! español, las tiras de glifos alargadas con ñ Ñ ¿ ¡. [`plan`] dice de dónde sale cada entrada; el cliente nativo escribe el
//! resultado una vez en disco (`p3_client::idioma::effective_cpr`) y la web lo monta sin copiarlo
//! (`p3_assets::cpr::build_virtual`).
use p3_assets::cpr::{self, Archive, Entry};

/// Código del CPR con el que corre el EXE.
pub const BASE: &str = "eng";

pub fn display_name(code: &str) -> String {
    match code {
        "eng" => "English",
        "spa" => "Spanish",
        "ger" | "deu" => "German",
        "fre" | "fra" => "French",
        "ita" => "Italian",
        "pol" => "Polish",
        "rus" => "Russian",
        "dut" | "nld" => "Dutch",
        other => return other.to_owned(),
    }
    .to_owned()
}

/// Origen de una entrada del CPR combinado.
pub enum Source {
    /// Entrada del CPR inglés.
    Base(Entry),
    /// Entrada del CPR del idioma.
    Lang(Entry),
    /// Datos nuevos (fuentes alargadas).
    Owned(Vec<u8>),
}

/// Entradas del CPR combinado para el idioma `code`, en orden. `base` es el CPR inglés y `lang` el del idioma.
pub fn plan(base: &Archive<'_>, lang: &Archive<'_>, code: &str) -> Vec<(Vec<u8>, Source)> {
    let mut files: Vec<(Vec<u8>, Source)> = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for e in lang.entries.iter().filter(|e| e.kept && e.kind == cpr::TYPE_STORED) {
        let name = rename_entry(&e.name, code);
        seen.insert(cpr::fold_name(&name));
        files.push((name, Source::Lang(e.clone())));
    }
    for e in base.entries.iter().filter(|e| e.kept && e.kind == cpr::TYPE_STORED) {
        if !seen.contains(&cpr::fold_name(&e.name)) {
            files.push((e.name.clone(), Source::Base(e.clone())));
        }
    }
    if code == "spa" {
        let read = |s: &Source| -> Option<Vec<u8>> {
            match s {
                Source::Base(e) => base.read_transient(e).map(|c| c.into_owned()),
                Source::Lang(e) => lang.read_transient(e).map(|c| c.into_owned()),
                Source::Owned(v) => Some(v.clone()),
            }
        };
        let key_of = |n: &[u8]| cpr::fold_name(n);
        let ini = files.iter().find(|(n, _)| key_of(n) == key_of(b"scripts/fonts.ini")).and_then(|(_, s)| read(s)).unwrap_or_default();
        for (font, cw, ch) in font_cells(&ini) {
            let key = key_of(&font);
            if let Some(entry) = files.iter_mut().find(|(n, _)| key_of(n) == key) {
                match read(&entry.1).ok_or_else(|| "no se puede leer".to_string()).and_then(|d| extend_font(&d, cw, ch)) {
                    Ok(wide) => entry.1 = Source::Owned(wide),
                    Err(e) => eprintln!("idioma {code}: fuente {}: {e}", String::from_utf8_lossy(&font)),
                }
            }
        }
    }
    files
}

pub fn rename_entry(name: &[u8], code: &str) -> Vec<u8> {
    // `_spa.` → `_eng.` sin distinguir mayúsculas; el resto del nombre queda tal cual.
    let from = format!("_{code}.").into_bytes();
    let to = format!("_{BASE}.").into_bytes();
    let mut out = Vec::with_capacity(name.len());
    let mut i = 0;
    while i < name.len() {
        if name[i..].len() >= from.len() && name[i..i + from.len()].eq_ignore_ascii_case(&from) {
            out.extend_from_slice(&to);
            i += from.len();
        } else {
            out.push(name[i]);
            i += 1;
        }
    }
    out
}

/// Caracteres que el español necesita y la tira de glifos de las fuentes del EXE inglés no tiene: se añaden al final del juego de
/// caracteres (`p3_mvp::ui_rt::language`, línea `X` del paquete) y aquí se dibujan sus celdas.
pub const EXTRA_CHARSET: &[u8] = b"\xf1\xd1\xbf\xa1";

fn glyph_index(c: u8) -> usize {
    p3_render::sgl_font::GAME_CHARSET.iter().position(|&x| x == c).expect("carácter del juego")
}

/// Alarga una tira `.aim` de glifos (celdas `cw × ch`, juego de caracteres del EXE) con ñ, Ñ, ¿ y ¡ hechos con los glifos que ya hay:
/// la virgulilla `~` colocada a la altura de los puntos de ä/Ä, y `?`/`!` invertidos.
pub fn extend_font(file: &[u8], cw: u32, ch: u32) -> Result<Vec<u8>, String> {
    let img = p3_assets::aim::decode(file).map_err(|e| format!("{e:?}"))?;
    let bpp = img.bytes_per_pixel as usize;
    if img.palette.is_some() || bpp == 0 || cw == 0 || ch == 0 || img.height != ch {
        return Err("formato de fuente no previsto".into());
    }
    let n = (img.width / cw) as usize;
    if n < p3_render::sgl_font::GAME_CHARSET.len() {
        return Err("la tira no cubre el juego de caracteres".into());
    }
    let (cw, ch) = (cw as usize, ch as usize);
    let new_w = (n + EXTRA_CHARSET.len()) * cw;
    let src_pitch = img.pitch as usize;
    let mut out = vec![0u8; new_w * bpp * ch];
    for y in 0..ch {
        out[y * new_w * bpp..y * new_w * bpp + n * cw * bpp].copy_from_slice(&img.pixels[y * src_pitch..y * src_pitch + n * cw * bpp]);
    }
    let get = |out: &Vec<u8>, g: usize, x: usize, y: usize| -> Vec<u8> {
        let at = (y * new_w + g * cw + x) * bpp;
        out[at..at + bpp].to_vec()
    };
    let ink = |px: &[u8]| px.iter().any(|&b| b != 0);
    // Filas y columnas con tinta de un glifo.
    let bbox = |out: &Vec<u8>, g: usize| -> Option<(usize, usize, usize, usize)> {
        let (mut x0, mut y0, mut x1, mut y1) = (usize::MAX, usize::MAX, 0, 0);
        for y in 0..ch {
            for x in 0..cw {
                if ink(&get(out, g, x, y)) {
                    (x0, y0, x1, y1) = (x0.min(x), y0.min(y), x1.max(x), y1.max(y));
                }
            }
        }
        (x0 != usize::MAX).then_some((x0, y0, x1, y1))
    };
    let copy_cell = |out: &mut Vec<u8>, from: usize, to: usize| {
        for y in 0..ch {
            for x in 0..cw {
                let px = get(out, from, x, y);
                let at = (y * new_w + to * cw + x) * bpp;
                out[at..at + bpp].copy_from_slice(&px);
            }
        }
    };
    let (gn, g_n, gt, gq, ge, ga, g_a) = (glyph_index(b'n'), glyph_index(b'N'), glyph_index(b'~'), glyph_index(b'?'), glyph_index(b'!'), glyph_index(0xE4), glyph_index(0xC4));
    let tilde = bbox(&out, gt).ok_or("falta ~")?;
    // ñ y Ñ: la base, más la virgulilla centrada sobre ella y donde empiezan los puntos de la diéresis.
    for (new, base, dots) in [(n, gn, ga), (n + 1, g_n, g_a)] {
        copy_cell(&mut out, base, new);
        let (b0, _, b1, _) = bbox(&out, base).ok_or("falta la base")?;
        let (_, top, _, _) = bbox(&out, dots).ok_or("falta la diéresis")?;
        let (tx0, ty0, tx1, ty1) = tilde;
        let dx = (b0 + b1) as isize / 2 - (tx0 + tx1) as isize / 2;
        for ty in ty0..=ty1 {
            for tx in tx0..=tx1 {
                let px = get(&out, gt, tx, ty);
                let (x, y) = (tx as isize + dx, (ty - ty0 + top) as isize);
                if ink(&px) && (0..cw as isize).contains(&x) && (0..ch as isize).contains(&y) {
                    let at = (y as usize * new_w + new * cw + x as usize) * bpp;
                    out[at..at + bpp].copy_from_slice(&px);
                }
            }
        }
    }
    // ¿ y ¡: el glifo girado verticalmente dentro de su caja de tinta.
    for (new, base) in [(n + 2, gq), (n + 3, ge)] {
        let (x0, y0, x1, y1) = bbox(&out, base).ok_or("falta ? o !")?;
        for y in y0..=y1 {
            for x in x0..=x1 {
                let px = get(&out, base, x, y1 - (y - y0));
                let at = (y * new_w + new * cw + x) * bpp;
                out[at..at + bpp].copy_from_slice(&px);
            }
        }
    }
    let wide = p3_assets::aim::Image { pixels: out, width: new_w as u32, pitch: (new_w * bpp) as u32, ..img };
    p3_assets::aim::encode(&wide).map_err(|e| format!("{e:?}"))
}

/// `(ruta, ancho de celda, alto)` de cada `[FontN]` de `scripts/fonts.ini` (`Params=ancho alto espaciado`).
pub fn font_cells(ini: &[u8]) -> Vec<(Vec<u8>, u32, u32)> {
    let text = String::from_utf8_lossy(ini);
    let (mut cells, mut params): (Vec<(Vec<u8>, u32, u32)>, Option<(u32, u32)>) = (Vec::new(), None);
    for line in text.lines().map(str::trim) {
        if line.starts_with('[') {
            params = None;
        } else if let Some(v) = line.strip_prefix("Params=") {
            let mut it = v.split_whitespace().filter_map(|t| t.parse::<u32>().ok());
            params = it.next().zip(it.next());
        } else if let (Some(v), Some((w, h))) = (line.strip_prefix("Font="), params) {
            cells.push((v.trim().as_bytes().to_vec(), w, h));
        }
    }
    cells
}


/// Código de idioma de un nombre `p2arch0_<código>.cpr` (sin distinguir mayúsculas).
pub fn cpr_code(file_name: &str) -> Option<String> {
    let name = file_name.to_lowercase();
    let code = name.strip_prefix("p2arch0_")?.strip_suffix(".cpr")?;
    (!code.is_empty() && code.len() <= 8 && code.chars().all(|c| c.is_ascii_alphanumeric())).then(|| code.to_owned())
}
