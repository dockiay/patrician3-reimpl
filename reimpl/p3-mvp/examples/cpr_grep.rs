//! Busca un texto en los archivos almacenados de un `.cpr`: `cpr_grep ARCHIVO.cpr TEXTO`.
use p3_assets::cpr::Archive;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let cpr = std::fs::read(&a[1]).unwrap();
    let archive = Archive::parse(&cpr).unwrap();
    let needle = a[2].as_bytes();
    for e in archive.entries.iter() {
        if let Some(d) = archive.read(e) {
            if d.windows(needle.len()).any(|w| w.eq_ignore_ascii_case(needle)) {
                println!("{}", String::from_utf8_lossy(&e.name));
            }
        }
    }
}
