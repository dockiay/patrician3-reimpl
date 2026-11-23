use p3_assets::cpr::Archive;
use p3_assets::let_sheet::LetSheet;
fn fold(p: &[u8]) -> Vec<u8> { p.iter().take_while(|&&b| b != 0).map(|&b| if b == b'\\' { b'/' } else { b.to_ascii_lowercase() }).collect() }
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let bytes = std::fs::read(&a[1]).unwrap(); let ar = Archive::parse(&bytes).unwrap();
    let id: u16 = a[2].parse().unwrap();
    for e in &ar.entries { let n = fold(&e.name); if n.starts_with(b"iso/let/") && n.ends_with(b".let") { if let Ok(s) = LetSheet::parse(ar.read(e).unwrap()) { if s.tile_id == id {
        println!("{} strips_a={} strips={:?}", String::from_utf8_lossy(&n), s.strips_a, s.strips);
        for c in &s.chunks { println!("tag={} array={:?} f0={} f1={} nframes={} frames={:?} tex={:?}", c.tag, c.array, c.f0, c.f1, c.frames.len(), &c.frames[..c.frames.len().min(3)], c.texture.as_ref().map(|t| String::from_utf8_lossy(&t.path).to_string())); }
    } } } }
}
