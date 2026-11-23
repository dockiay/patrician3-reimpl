//! Para cada oficina humana y cada tipo de edificio, busca una casilla donde `build_check` dé válido e intenta construir. Imprime los tipos que funcionan.
use std::collections::BTreeMap;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let save = std::fs::read(&a[1]).unwrap();
    let cpr = std::fs::read(&a[2]).unwrap();
    let ids: Vec<usize> = p3_mvp::Session::open(&save, cpr.clone()).unwrap().state()["offices"]
        .as_array()
        .unwrap()
        .iter()
        .map(|o| o["office"].as_u64().unwrap() as usize)
        .collect();
    eprintln!("oficinas: {ids:?}");
    let mut result: BTreeMap<u8, String> = BTreeMap::new();
    for &office in &ids {
        let probe = p3_mvp::Session::open(&save, cpr.clone()).unwrap();
        let _ = probe;
        for ty in 1u8..0x39 {
            let mut s = p3_mvp::Session::open(&save, cpr.clone()).unwrap();
            if s.select_office(office).is_err() {
                continue;
            }
            let (rows, stride) = {
                let (t, _) = s.debug_town();
                (
                    t.i32(p3_core::town::off::PLAN_ROWS),
                    t.i32(p3_core::town::off::PLAN_STRIDE),
                )
            };
            let city = s.state()["town_name"].to_string();
            let mut tried = 0;
            let mut last = String::new();
            'f: for r in 0..rows as u32 {
                for c in 0..stride as u32 {
                    if s.build_check(ty, r as u8, c as u8)
                        .map(|v| v["matches"] == true)
                        .unwrap_or(false)
                    {
                        tried += 1;
                        match s.build(ty, r as u8, c as u8) {
                            Ok(_) => {
                                result.insert(ty, format!("OK en {city} ({r},{c})"));
                                break 'f;
                            }
                            Err(e) => {
                                last = e;
                                if tried >= 3 {
                                    break 'f;
                                }
                            }
                        }
                    }
                }
            }
            if !result.get(&ty).is_some_and(|v| v.starts_with("OK")) {
                let e = result.entry(ty).or_default();
                if !e.starts_with("OK") {
                    *e = if tried == 0 {
                        format!("sin casilla en {city}")
                    } else {
                        format!("{city}: {last}")
                    }
                }
            }
        }
    }
    for (t, v) in result {
        println!("tipo {t:#04x}: {v}")
    }
}
