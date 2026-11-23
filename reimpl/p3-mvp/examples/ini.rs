use p3_assets::cpr::Archive;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let b = std::fs::read(&a[1]).unwrap();
    let ar = Archive::parse(&b).unwrap();
    let e = ar.find(a[2].as_bytes()).unwrap();
    print!("{}", String::from_utf8_lossy(ar.read(e).unwrap()));
}
