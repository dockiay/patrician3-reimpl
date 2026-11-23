//! Comprueba sobre el CPR entero la condición de la que depende la carga aplazada de texturas del enlazador
//! (`p3_mvp::ui_rt::lazy_decodable`): todo archivo que se acepta aplazar se descodifica bien cuando llega su primer uso.
//! Uso: `P3_CPR=p2arch0_eng.cpr cargo run --release -p p3-mvp --example ui_texture_probe`. Sale con error si algún archivo falla.
use p3_mvp::ui_rt::{decode_image, lazy_decodable};
use std::collections::BTreeMap;

fn main() {
    let cpr = std::fs::read(std::env::var_os("P3_CPR").expect("P3_CPR required")).unwrap();
    let archive = p3_assets::cpr::Archive::parse(&cpr).unwrap();
    let files: Vec<(&[u8], &[u8])> = archive.entries.iter().filter_map(|e| Some((e.name.as_slice(), archive.read(e)?))).collect();
    let threads = std::thread::available_parallelism().map_or(4, |n| n.get());
    let chunk = files.len().div_ceil(threads).max(1);
    // (clase, aplazable, descodifica) → cuántos
    let mut totals: BTreeMap<(String, bool, bool), usize> = BTreeMap::new();
    let mut bad = Vec::new();
    std::thread::scope(|s| {
        let handles: Vec<_> = files
            .chunks(chunk)
            .map(|part| {
                s.spawn(move || {
                    let mut local: BTreeMap<(String, bool, bool), usize> = BTreeMap::new();
                    let mut bad = Vec::new();
                    for (name, data) in part {
                        let name = String::from_utf8_lossy(name).into_owned();
                        let lazy = lazy_decodable(data, &name);
                        let lower = name.to_ascii_lowercase();
                        let class = if data.get(..3) == Some(b"AIM") {
                            format!("AIM {:#04x}", data.get(p3_assets::aim::SUBLAYER_OFFSET).copied().unwrap_or(0))
                        } else if let Some(ext) = [".tga", ".bmp", ".jpg"].iter().find(|e| lower.ends_with(**e)) {
                            ext.to_string()
                        } else {
                            continue;
                        };
                        let ok = std::panic::catch_unwind(|| decode_image(data, &name).is_some()).unwrap_or(false);
                        *local.entry((class, lazy, ok)).or_default() += 1;
                        if lazy && !ok {
                            bad.push(name);
                        }
                    }
                    (local, bad)
                })
            })
            .collect();
        for h in handles {
            let (local, b) = h.join().unwrap();
            for (k, n) in local {
                *totals.entry(k).or_default() += n;
            }
            bad.extend(b);
        }
    });
    println!("clase\taplazable\tdescodifica\tarchivos");
    for ((class, lazy, ok), n) in &totals {
        println!("{class}\t{lazy}\t{ok}\t{n}");
    }
    println!("aplazables que no se descodifican: {}", bad.len());
    for b in &bad {
        println!("  {b}");
    }
    if !bad.is_empty() {
        std::process::exit(1);
    }
}
