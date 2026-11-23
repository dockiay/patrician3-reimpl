//! `.ini` del usuario (`P2.CFG`) junto al juego: lectura y escritura como `GetPrivateProfile*`/`WritePrivateProfileStringA`.
//!
//! Los `.ini` de `scripts/` salen del CPR y no cambian; `P2.CFG` lo escribe el propio juego (opciones `0x0045DF10`, nueva partida
//! `0x00499BE0`) y lo vuelve a leer al abrir la ventana de opciones (`0x0045CE70`). Sin esto los cambios de Opciones se perdían:
//! al reabrirla el autoguardado (y el resto de ajustes) volvía al valor por defecto y el intervalo elegido dejaba de valer.

use std::collections::HashMap;

/// Tabla (sección, clave) en minúsculas → primer valor, como lo resuelve Windows.
pub type IniTable = HashMap<(Vec<u8>, Vec<u8>), Vec<u8>>;

/// ¿Es `path` (relativo, sin `./`) un `.ini` del usuario que vive en la carpeta del juego y no en el CPR?
pub fn is_user_ini(path: &[u8]) -> bool {
    path.eq_ignore_ascii_case(b"P2.CFG")
}

pub fn parse(data: &[u8]) -> IniTable {
    let mut table = IniTable::new();
    let mut current: Vec<u8> = Vec::new();
    for line in data.split(|&b| b == b'\n') {
        let line = line.strip_suffix(b"\r").unwrap_or(line).trim_ascii();
        if line.starts_with(b"[") {
            current = line.iter().skip(1).take_while(|&&b| b != b']').map(u8::to_ascii_lowercase).collect();
        } else if let Some(eq) = line.iter().position(|&b| b == b'=') {
            let k = line[..eq].trim_ascii().to_ascii_lowercase();
            table.entry((current.clone(), k)).or_insert_with(|| line[eq + 1..].trim_ascii().to_vec());
        }
    }
    table
}

fn section_of(line: &[u8]) -> Option<&[u8]> {
    let line = line.trim_ascii();
    line.strip_prefix(b"[").map(|rest| {
        let end = rest.iter().position(|&b| b == b']').unwrap_or(rest.len());
        rest[..end].trim_ascii()
    })
}

fn key_of(line: &[u8]) -> Option<&[u8]> {
    let line = line.trim_ascii();
    line.iter().position(|&b| b == b'=').map(|eq| line[..eq].trim_ascii())
}

/// `WritePrivateProfileStringA(section, key, value)` sobre el texto `data`: `key = None` borra la sección, `value = None` borra la
/// clave; si no, la sustituye o la añade al final de su sección (creándola al final del archivo si falta). Conserva el resto.
pub fn write(data: &[u8], section: &[u8], key: Option<&[u8]>, value: Option<&[u8]>) -> Vec<u8> {
    let text: Vec<&[u8]> = if data.is_empty() { Vec::new() } else { data.split(|&b| b == b'\n').collect() };
    let mut lines: Vec<Vec<u8>> = text.iter().map(|l| l.strip_suffix(b"\r").unwrap_or(l).to_vec()).collect();
    if lines.last().is_some_and(Vec::is_empty) {
        lines.pop();
    }
    let start = lines.iter().position(|l| section_of(l).is_some_and(|s| s.eq_ignore_ascii_case(section)));
    let new_line = |k: &[u8], v: &[u8]| [k, b"=", v].concat();
    match start {
        None => {
            if let (Some(k), Some(v)) = (key, value) {
                lines.push([b"[", section, b"]"].concat());
                lines.push(new_line(k, v));
            }
        }
        Some(s) => {
            let end = lines[s + 1..].iter().position(|l| section_of(l).is_some()).map_or(lines.len(), |p| s + 1 + p);
            match key {
                None => {
                    lines.drain(s..end);
                }
                Some(k) => {
                    let found = (s + 1..end).find(|&i| key_of(&lines[i]).is_some_and(|x| x.eq_ignore_ascii_case(k)));
                    match (found, value) {
                        (Some(i), Some(v)) => lines[i] = new_line(k, v),
                        (Some(i), None) => {
                            lines.remove(i);
                        }
                        (None, Some(v)) => {
                            // Detrás de la última línea no vacía de la sección.
                            let mut at = end;
                            while at > s + 1 && lines[at - 1].trim_ascii().is_empty() {
                                at -= 1;
                            }
                            lines.insert(at, new_line(k, v));
                        }
                        (None, None) => {}
                    }
                }
            }
        }
    }
    let mut out = Vec::new();
    for l in lines {
        out.extend_from_slice(&l);
        out.extend_from_slice(b"\r\n");
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replaces_adds_and_deletes() {
        let base = b"[OPTIONS]\r\nLSSOUND=127\r\nAUTOSAVE=3\r\n\r\n[EINZEL]\r\nNAME=Sergio\r\n";
        let out = write(base, b"options", Some(b"autosave"), Some(b"1"));
        assert_eq!(out, b"[OPTIONS]\r\nLSSOUND=127\r\nautosave=1\r\n\r\n[EINZEL]\r\nNAME=Sergio\r\n");
        assert_eq!(parse(&out)[&(b"options".to_vec(), b"autosave".to_vec())], b"1");
        let out = write(base, b"OPTIONS", Some(b"DETAIL"), Some(b"2"));
        assert_eq!(out, b"[OPTIONS]\r\nLSSOUND=127\r\nAUTOSAVE=3\r\nDETAIL=2\r\n\r\n[EINZEL]\r\nNAME=Sergio\r\n");
        let out = write(base, b"NEU", Some(b"A"), Some(b"x"));
        assert!(out.ends_with(b"NAME=Sergio\r\n[NEU]\r\nA=x\r\n"));
        let out = write(base, b"OPTIONS", Some(b"LSSOUND"), None);
        assert_eq!(out, b"[OPTIONS]\r\nAUTOSAVE=3\r\n\r\n[EINZEL]\r\nNAME=Sergio\r\n");
        let out = write(base, b"OPTIONS", None, None);
        assert_eq!(out, b"[EINZEL]\r\nNAME=Sergio\r\n");
        assert_eq!(write(b"", b"OPTIONS", Some(b"AUTOSAVE"), Some(b"1")), b"[OPTIONS]\r\nAUTOSAVE=1\r\n");
    }
}
