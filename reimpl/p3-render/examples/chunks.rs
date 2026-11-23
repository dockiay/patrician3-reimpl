use p3_assets::cpr::Archive;
use p3_assets::let_sheet::LetSheet;
fn fold(p: &[u8]) -> Vec<u8> { p.iter().take_while(|&&b| b != 0).map(|&b| if b == b'\\' { b'/' } else { b.to_ascii_lowercase() }).collect() }
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let bytes = std::fs::read(&a[1]).unwrap(); let ar = Archive::parse(&bytes).unwrap();
    let mut shown = std::collections::BTreeMap::<u32, usize>::new();
    let mut total = 0;
    for e in &ar.entries { let n = fold(&e.name); if n.starts_with(b"iso/let/") && n.ends_with(b".let") { if let Ok(s) = LetSheet::parse(ar.read(e).unwrap()) { if s.chunks.is_empty() { continue; } total += 1;
        for c in &s.chunks { let k = shown.entry(c.tag).or_default(); *k += 1; if *k <= 3 { println!("{} id={} tag={} array={:?} f0={} f1={} frames={:?} tex={:?}", String::from_utf8_lossy(&n), s.tile_id, c.tag, c.array, c.f0, c.f1, &c.frames[..c.frames.len().min(4)], c.texture.as_ref().map(|t| (String::from_utf8_lossy(&t.path).to_string(), t.index))); } } } } }
    println!("hojas con trozos: {total}; por etiqueta: {shown:?}");
}
