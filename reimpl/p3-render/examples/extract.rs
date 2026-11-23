use p3_assets::cpr::Archive;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let bytes = std::fs::read(&a[1]).unwrap();
    let ar = Archive::parse(&bytes).unwrap();
    let want = a[2].to_lowercase();
    let e = ar.entries.iter().rev().find(|e| String::from_utf8_lossy(&e.name).to_lowercase().replace('\\', "/") == want).expect("no existe");
    std::fs::write(&a[3], ar.read(e).unwrap()).unwrap();
}
