//! Diagnóstico J05: cabeceras de los conjuntos de sprites de barco del puerto (`iso/lcr/<tipo>_*.lcr`).
use p3_assets::cpr::Archive;
use p3_assets::lcr::Lcr;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let bytes = std::fs::read(&a[1]).unwrap();
    let arc = Archive::parse(&bytes).unwrap();
    for e in &arc.entries {
        let n = String::from_utf8_lossy(&e.name).to_lowercase().replace('/', "\\");
        let ship = ["holk", "kogge", "kraier", "schnigge"]
            .iter()
            .any(|t| n.starts_with(&format!("iso\\lcr\\{t}_")));
        if !ship || n.contains("sink") {
            continue;
        }
        let data = arc.read(e).unwrap();
        for k in [if n.contains("schaden") { 5usize } else { 4 }] {
            match Lcr::parse(data, k) {
                Ok(l) => println!(
                    "{n} kind {} dirs {} fps_ms {} cells {} extra {:?} states {} path {} frames/dir {}",
                    l.kind,
                    l.directions,
                    l.interval_ms,
                    l.has_cells,
                    l.extra_header,
                    l.states.len(),
                    String::from_utf8_lossy(&l.path),
                    l.dirs[0].len()
                ),
                Err(err) => println!("{n} k={k}: {err}"),
            }
        }
    }
}
