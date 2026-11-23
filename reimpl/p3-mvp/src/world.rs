//! Mapa del mundo: la imagen de `images/vollansichtskarte.bmp` (2400×1800), las ciudades de `scripts/StadtDaten.ini`
//! (posiciones en píxeles de esa imagen) y los barcos de la partida (`+0x1E`/`+0x22` del barco, en el mismo espacio).

use serde_json::{json, Value};

/// Una ciudad del mapa del mundo (`[StadtN]`); el nombre sale del comentario `// Nombre` que precede a la sección.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorldTown {
    pub id: usize,
    pub name: String,
    pub symbol: (i32, i32),
    pub label: (i32, i32),
    pub label_right: bool,
    pub approach: (i32, i32),
    /// `EreignisSymbolXPos/YPos` (iconos de estado de la carta, `+0x10`).
    pub event: (i32, i32),
    /// `SchiffAufLandXPos/YPos` (barcos en puerto de la carta, `+0x18`).
    pub harbor: (i32, i32),
    /// `+0x2F` = `AmFluss == 1` (`0x0051E63A`).
    pub river: bool,
    /// `+0x33`: primera letra de `NameLinksOderRechtsBuendig` (`0x0051E69C`).
    pub align: u8,
    /// `Region=`: grupo de estilo de la ciudad (0..4); el original lo guarda en `0x006DDBC0 + 0x34*ciudad` y elige con él las hojas de iglesia, torres y puertas.
    pub region: u32,
}

/// Interpreta `StadtDaten.ini`. Las secciones sin símbolo o con claves inválidas se ignoran.
pub fn parse_towns(ini: &str) -> Vec<WorldTown> {
    let mut out = Vec::new();
    let mut name = String::new();
    let mut current: Option<(usize, std::collections::HashMap<String, String>)> = None;
    let mut flush = |current: &mut Option<(usize, std::collections::HashMap<String, String>)>,
                     name: &str,
                     out: &mut Vec<WorldTown>| {
        if let Some((id, keys)) = current.take() {
            let num = |k: &str| keys.get(k).and_then(|v| v.trim().parse::<i32>().ok());
            if let (Some(sx), Some(sy)) = (num("StadtSymbolXPos"), num("StadtSymbolYPos")) {
                out.push(WorldTown {
                    id,
                    name: name.to_string(),
                    symbol: (sx, sy),
                    label: (num("NamenXPos").unwrap_or(sx), num("NamenYPos").unwrap_or(sy)),
                    label_right: keys
                        .get("NameLinksOderRechtsBuendig")
                        .is_some_and(|v| v.trim().eq_ignore_ascii_case("r")),
                    approach: (num("AnfahrtXPos").unwrap_or(sx), num("AnfahrtYPos").unwrap_or(sy)),
                    event: (num("EreignisSymbolXPos").unwrap_or(0), num("EreignisSymbolYPos").unwrap_or(0)),
                    harbor: (num("SchiffAufLandXPos").unwrap_or(0), num("SchiffAufLandYPos").unwrap_or(0)),
                    river: num("AmFluss") == Some(1),
                    align: keys.get("NameLinksOderRechtsBuendig").and_then(|v| v.trim().bytes().next()).unwrap_or(0),
                    region: num("Region").and_then(|v| u32::try_from(v).ok()).unwrap_or(2),
                });
            }
        }
    };
    for raw in ini.lines() {
        let line = raw.trim();
        if let Some(comment) = line.strip_prefix("//") {
            flush(&mut current, &name, &mut out);
            name = comment.trim().to_string();
        } else if let Some(section) = line.strip_prefix('[').and_then(|l| l.strip_suffix(']')) {
            flush(&mut current, &name, &mut out);
            current = section
                .strip_prefix("Stadt")
                .and_then(|n| n.parse::<usize>().ok())
                .map(|id| (id, Default::default()));
        } else if let (Some((_, keys)), Some((k, v))) = (current.as_mut(), line.split_once('=')) {
            keys.insert(k.trim().to_string(), v.trim().to_string());
        }
    }
    flush(&mut current, &name, &mut out);
    out
}

/// JSON de una ciudad para el cliente.
pub fn town_json(t: &WorldTown, current: bool) -> Value {
    json!({"id":t.id,"name":t.name,"x":t.symbol.0,"y":t.symbol.1,"label_x":t.label.0,"label_y":t.label.1,"label_right":t.label_right,
           "approach_x":t.approach.0,"approach_y":t.approach.1,"current":current})
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_sections_names_and_labels() {
        let ini = "// Edinburgh\r\n[Stadt0]\r\nStadtSymbolXPos=95\r\nStadtSymbolYPos=709\r\nNamenXPos=71\r\nNamenYPos=736\r\nNameLinksOderRechtsBuendig=l\r\nAnfahrtXPos=115\r\nAnfahrtYPos=715\r\n\r\n// Newcastle\r\n[Stadt1]\r\nStadtSymbolXPos=169\r\nStadtSymbolYPos=848\r\nNameLinksOderRechtsBuendig=r\r\n[Region3]\r\nStadtSymbolXPos=1\r\n";
        let towns = parse_towns(ini);
        assert_eq!(towns.len(), 2);
        assert_eq!(
            towns[0],
            WorldTown {
                id: 0,
                name: "Edinburgh".into(),
                symbol: (95, 709),
                label: (71, 736),
                label_right: false,
                approach: (115, 715),
                event: (0, 0),
                harbor: (0, 0),
                river: false,
                align: b'l',
                region: 2
            }
        );
        assert_eq!(
            (
                towns[1].id,
                towns[1].name.as_str(),
                towns[1].label_right,
                towns[1].label
            ),
            (1, "Newcastle", true, (169, 848))
        );
    }
}
