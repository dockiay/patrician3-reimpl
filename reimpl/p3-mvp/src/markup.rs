//! Códigos de formato del texto del juego, tal como los interpreta `text_layout_base_layout_text` (`0x00462520`).
//!
//! El texto redactado de cartas, avisos y diálogos lleva escapes con barra invertida que el original no pinta:
//!
//! | Código | Efecto en `0x00462520` | Caracteres que consume |
//! |---|---|---|
//! | `\l` `\r` `\c` | alineación 1/2/3 (izquierda, derecha, centro) de la línea en curso y las siguientes (`[esp+0x20]`) | 2 |
//! | `\fN` | fuente `atoi("N…")` desde ese punto (`[esp+0x14]`, tabla de fuentes `0x006DCD28 + 0xA8·N`) | 3 |
//! | `\dX` (X en `A`–`Z`) | letra capital como imagen (`0x004C5870` → `0x00462F00`); otro X no es código | 3 |
//! | `\C` `\B` `\L` | imagen en línea de las texturas `0x3E8E` (moneda), `0x3E9E` y `0x3E9F` (`0x0042EDB7`…) | 2 |
//! | `\tNOMBRE_` `\hNOMBRE_` | textura por nombre (`0x004B3DD0`) o por número (`atoi`) hasta el `_` | hasta el `_` incluido |
//!
//! Alineación y fuente persisten hasta otro código. Un `_` al principio de una palabra (tras espacio, salto o código) no cuenta:
//! `0x004629CD` mide la palabra sin él (lo quita con `memmove` y lo repone), y se usa tras `\fN` para separar el número de la
//! palabra siguiente. El resto de caracteres es texto. El texto es
//! latín-1 (juego de caracteres de `p3_render::sgl_font::GAME_CHARSET`). No se reproduce aquí el corte de líneas por ancho,
//! que depende de las métricas de la fuente: lo hace quien dibuja.
use serde_json::{json, Value};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Piece {
    Text(String),
    /// `\C`, `\B`, `\L`: letra del código.
    Icon(char),
    /// `\dX`: capital `X` dibujada como imagen.
    Initial(char),
    /// `\t` (`by_name` = true) o `\h`: textura en línea.
    Texture { by_name: bool, name: String },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Run {
    pub font: u8,
    pub piece: Piece,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Line {
    /// 1 izquierda, 2 derecha, 3 centro (`+0x10` de la línea maquetada).
    pub align: u8,
    pub runs: Vec<Run>,
}

fn atoi(bytes: &[u8]) -> i32 {
    let mut i = 0;
    while i < bytes.len() && matches!(bytes[i], b' ' | b'\t') {
        i += 1;
    }
    let neg = bytes.get(i) == Some(&b'-');
    if matches!(bytes.get(i), Some(b'-' | b'+')) {
        i += 1;
    }
    let mut v: i32 = 0;
    while let Some(d) = bytes.get(i).filter(|d| d.is_ascii_digit()) {
        v = v.wrapping_mul(10).wrapping_add(i32::from(d - b'0'));
        i += 1;
    }
    if neg {
        v.wrapping_neg()
    } else {
        v
    }
}

/// Divide `text` (bytes latín-1) en líneas con su alineación y tramos con su fuente. `align` y `font` son los valores con
/// que empieza la maqueta (los del objeto de texto).
pub fn layout(text: &[u8], mut align: u8, mut font: u8) -> Vec<Line> {
    let mut lines = vec![Line { align, runs: Vec::new() }];
    let mut word = String::new();
    let flush = |lines: &mut Vec<Line>, word: &mut String, font: u8| {
        if !word.is_empty() {
            let line = lines.last_mut().unwrap();
            match line.runs.last_mut() {
                Some(Run { font: f, piece: Piece::Text(t) }) if *f == font => t.push_str(word),
                _ => line.runs.push(Run { font, piece: Piece::Text(word.clone()) }),
            }
            word.clear();
        }
    };
    let push = |lines: &mut Vec<Line>, font: u8, piece: Piece| lines.last_mut().unwrap().runs.push(Run { font, piece });
    let mut i = 0;
    let mut word_start = true;
    while i < text.len() {
        let c = text[i];
        if c == b'_' && word_start {
            word_start = false;
            i += 1;
            continue;
        }
        word_start = c == b' ' || c == b'\n' || c == b'\\';
        if c == b'\n' {
            flush(&mut lines, &mut word, font);
            lines.push(Line { align, runs: Vec::new() });
            i += 1;
            continue;
        }
        if c == b'\\' {
            let code = text.get(i + 1).copied().unwrap_or(0);
            match code {
                b'l' | b'r' | b'c' => {
                    flush(&mut lines, &mut word, font);
                    align = match code {
                        b'l' => 1,
                        b'r' => 2,
                        _ => 3,
                    };
                    let line = lines.last_mut().unwrap();
                    line.align = align;
                    i += 2;
                    continue;
                }
                b'f' => {
                    flush(&mut lines, &mut word, font);
                    font = atoi(&text[(i + 2).min(text.len())..]) as u8;
                    i += 3;
                    continue;
                }
                b'd' if text.get(i + 2).is_some_and(|x| x.is_ascii_uppercase()) => {
                    flush(&mut lines, &mut word, font);
                    push(&mut lines, font, Piece::Initial(char::from(text[i + 2])));
                    i += 3;
                    continue;
                }
                b'C' | b'B' | b'L' => {
                    flush(&mut lines, &mut word, font);
                    push(&mut lines, font, Piece::Icon(char::from(code)));
                    i += 2;
                    continue;
                }
                b't' | b'h' => {
                    flush(&mut lines, &mut word, font);
                    let start = i + 2;
                    let end = text[start.min(text.len())..].iter().position(|&b| b == b'_').map_or(text.len(), |p| start + p);
                    let name = text[start.min(end)..end].iter().map(|&b| char::from(b)).collect();
                    push(&mut lines, font, Piece::Texture { by_name: code == b't', name });
                    i = end + 1;
                    continue;
                }
                _ => {}
            }
        }
        word.push(char::from(c));
        i += 1;
    }
    flush(&mut lines, &mut word, font);
    lines
}

/// Texto sin códigos, con los iconos en su forma legible (moneda como `¤`).
pub fn plain(lines: &[Line]) -> String {
    lines
        .iter()
        .map(|l| {
            l.runs
                .iter()
                .map(|r| match &r.piece {
                    Piece::Text(t) => t.clone(),
                    Piece::Icon('C') => "\u{a4}".into(),
                    Piece::Icon(c) => format!("[{c}]"),
                    Piece::Initial(c) => c.to_string(),
                    Piece::Texture { .. } => String::new(),
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Maqueta en JSON para el cliente: `[{"align": "left"|"right"|"center", "runs": [{"font": n, "text"|"icon"|"initial"|"texture": …}]}]`.
pub fn to_json(lines: &[Line]) -> Value {
    Value::Array(
        lines
            .iter()
            .map(|l| {
                let runs: Vec<Value> = l
                    .runs
                    .iter()
                    .map(|r| match &r.piece {
                        Piece::Text(t) => json!({"font": r.font, "text": t}),
                        Piece::Icon(c) => json!({"font": r.font, "icon": c.to_string()}),
                        Piece::Initial(c) => json!({"font": r.font, "initial": c.to_string()}),
                        Piece::Texture { by_name, name } => json!({"font": r.font, "texture": name, "by_name": by_name}),
                    })
                    .collect();
                let align = match l.align {
                    2 => "right",
                    3 => "center",
                    _ => "left",
                };
                json!({"align": align, "runs": runs})
            })
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weekly_accounts_letter() {
        let raw = b"\n\\r\\f1_Luebeck,  2. 9. 1345\n\n\\lTaxes for your trading offices and workshops last week amounted to 280\\C.\n";
        let lines = layout(raw, 1, 0);
        assert_eq!(lines[1].align, 2);
        assert_eq!(lines[1].runs, vec![Run { font: 1, piece: Piece::Text("Luebeck,  2. 9. 1345".into()) }]);
        assert_eq!(lines[2].align, 2, "la alineación persiste en las líneas siguientes");
        assert_eq!(lines[3].align, 1);
        assert_eq!(lines[3].runs.last().unwrap().piece, Piece::Text(".".into()));
        assert!(lines[3].runs.contains(&Run { font: 1, piece: Piece::Icon('C') }));
        assert_eq!(plain(&lines), "\nLuebeck,  2. 9. 1345\n\nTaxes for your trading offices and workshops last week amounted to 280\u{a4}.\n");
    }

    #[test]
    fn initial_texture_and_unknown_codes() {
        let lines = layout(b"\\dHello \\tcoin_x \\hx\\q", 1, 0);
        let pieces: Vec<_> = lines[0].runs.iter().map(|r| r.piece.clone()).collect();
        assert_eq!(
            pieces,
            vec![
                Piece::Initial('H'),
                Piece::Text("ello ".into()),
                Piece::Texture { by_name: true, name: "coin".into() },
                Piece::Text("x ".into()),
                Piece::Texture { by_name: false, name: "x\\q".into() },
            ]
        );
        assert_eq!(layout(b"\\dx", 1, 0)[0].runs[0].piece, Piece::Text("\\dx".into()));
    }
}
