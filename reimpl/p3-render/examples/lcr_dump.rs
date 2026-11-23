use p3_assets::cpr::Archive;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let bytes = std::fs::read(&a[1]).unwrap();
    let ar = Archive::parse(&bytes).unwrap();
    let out = std::path::Path::new(&a[2]);
    std::fs::create_dir_all(out).unwrap();
    let mut seen = std::collections::HashSet::new();
    for e in ar.entries.iter().rev() {
        let n = String::from_utf8_lossy(&e.name).to_lowercase().replace('\\', "/");
        if n.starts_with("iso/lcr/") && seen.insert(n.clone()) {
            if let Some(d) = ar.read(e) { std::fs::write(out.join(n.trim_start_matches("iso/lcr/")), d).unwrap(); }
        }
    }
}
