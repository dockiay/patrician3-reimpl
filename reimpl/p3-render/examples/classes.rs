use p3_assets::cpr::Archive;
use p3_assets::let_sheet::LetSheet;
fn fold(p: &[u8]) -> Vec<u8> { p.iter().take_while(|&&b| b != 0).map(|&b| if b == b'\\' { b'/' } else { b.to_ascii_lowercase() }).collect() }
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let bytes = std::fs::read(&a[1]).unwrap();
    let ar = Archive::parse(&bytes).unwrap();
    let want: Vec<u8> = a[2..].iter().map(|s| s.parse().unwrap()).collect();
    for e in &ar.entries { let n = fold(&e.name); if n.starts_with(b"iso/let/") && n.ends_with(b".let") {
        if let Ok(s) = LetSheet::parse(ar.read(e).unwrap()) { if want.contains(&s.field_2d) {
            println!("class {} id {} {} fr={}x{} a={} b={} matrix={:?}", s.field_2d, s.tile_id, String::from_utf8_lossy(&n), s.tile_width, s.tile_height, s.strips_a, s.strips_b, s.matrix);
        } }
    } }
}
