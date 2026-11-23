use p3_assets::cpr::Archive;
use p3_assets::let_sheet::LetSheet;
use std::collections::BTreeMap;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let bytes = std::fs::read(&a[1]).unwrap();
    let ar = Archive::parse(&bytes).unwrap();
    let mut by: BTreeMap<u32, Vec<u16>> = BTreeMap::new();
    for e in &ar.entries {
        let n = String::from_utf8_lossy(&e.name).to_lowercase().replace('\\', "/");
        if n.starts_with("iso/let/") && n.ends_with(".let") {
            if let Ok(s) = LetSheet::parse(ar.read(e).unwrap()) { by.entry(s.color_key).or_default().push(s.tile_id); }
        }
    }
    for (k, v) in by { println!("{k:#08x}: {} hojas {:?}", v.len(), &v[..v.len().min(12)]); }
}
