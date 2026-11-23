//! `dump_records CARPETA_GAMES`: una línea `huella tipo índice sha256(imagen)` por comerciante y oficina en uso de cada guardado distinto, para
//! `scripts/build_save_struct_fields.py --rust-dump` (comparación independiente con la lectura en Python).

use p3_save::{ships::Ships, world::World, Save};
use sha2::{Digest, Sha256};

fn main() {
    let root = std::env::args().nth(1).expect("carpeta games");
    let mut seen = std::collections::BTreeSet::new();
    let mut files = Vec::new();
    for lab in std::fs::read_dir(&root).unwrap().flatten() {
        for mode in std::fs::read_dir(lab.path().join("save")).into_iter().flatten().flatten() {
            for f in std::fs::read_dir(mode.path()).unwrap().flatten() {
                if f.path().extension().is_some_and(|e| e == "pat") {
                    files.push(f.path());
                }
            }
        }
    }
    files.sort();
    for f in files {
        let raw = std::fs::read(&f).unwrap();
        let h = format!("{:x}", Sha256::digest(&raw));
        if !seen.insert(h.clone()) {
            continue;
        }
        let save = Save::parse(&raw).unwrap();
        let w = World::parse(save.block_named("game_world").unwrap()).unwrap();
        for k in 0..w.merchant_slots() {
            println!("{} merchant {} {:x}", &h[..12], k, Sha256::digest(w.merchant_image(k)));
        }
        let sh = Ships::parse(save.block_named("ships").unwrap()).unwrap();
        for k in 0..sh.ship_slots() {
            if let Some(img) = sh.ship_image(k) {
                println!("{} ship {} {:x}", &h[..12], k, Sha256::digest(img));
            }
        }
        for k in 0..sh.convoys.len() {
            println!("{} convoy {} {:x}", &h[..12], k, Sha256::digest(sh.convoy_image(k)));
        }
        for k in 0..w.office_slots() {
            if let Some(img) = w.office_image(k) {
                println!("{} office {} {:x}", &h[..12], k, Sha256::digest(img));
            }
        }
    }
}
