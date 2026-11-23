use p3_assets::cpr::Archive;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let bytes = std::fs::read(&a[1]).unwrap();
    let ar = Archive::parse(&bytes).unwrap();
    let pat = a[2].to_lowercase();
    for e in &ar.entries { let n = String::from_utf8_lossy(&e.name).to_lowercase().replace('\\', "/"); if n.contains(&pat) { println!("{n} {} {}", e.kept, ar.read(e).map_or(0, |d| d.len())); } }
}
