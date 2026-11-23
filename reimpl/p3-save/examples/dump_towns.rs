//! `dump_towns CARPETA_GAMES`: una línea `huella ciudad sha256(Town)` por ciudad de cada guardado distinto, para
//! `scripts/build_town_save_fields.py --rust-dump` (comparación independiente con la lectura en Python).

use p3_save::{world::World, Save};
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
        for i in 0..w.town_count() {
            println!("{} {} {:x}", &h[..12], i, Sha256::digest(&w.town(i).0[..]));
        }
    }
}
