use p3_assets::cpr::Archive;
use p3_assets::let_sheet::LetSheet;
fn fold(p: &[u8]) -> Vec<u8> { p.iter().take_while(|&&b| b != 0).map(|&b| if b == b'\\' { b'/' } else { b.to_ascii_lowercase() }).collect() }
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let bytes = std::fs::read(&a[1]).unwrap();
    let ar = Archive::parse(&bytes).unwrap();
    let id: u16 = a[2].parse().unwrap();
    let mut sheet = None;
    for e in &ar.entries { let n = fold(&e.name); if n.starts_with(b"iso/let/") && n.ends_with(b".let") { if let Ok(s) = LetSheet::parse(ar.read(e).unwrap()) { if s.tile_id == id && sheet.is_none() { sheet = Some((n.clone(), s)); } } } }
    let (name, s) = sheet.unwrap();
    println!("{} source {} strips {:?} a={} b={} {}x{} f22={} f24={} field_2d={}", String::from_utf8_lossy(&name), String::from_utf8_lossy(&fold(&s.source)), s.strips, s.strips_a, s.strips_b, s.tile_width, s.tile_height, s.field_22, s.field_24, s.field_2d);
    let src = fold(&s.source);
    let te = ar.entries.iter().rev().find(|e| e.kept && fold(&e.name) == src).unwrap();
    let img = p3_assets::aim::decode(ar.read(te).unwrap()).unwrap();
    let t = p3_render::to_rgba(&img).unwrap();
    p3_render::write_png(std::path::Path::new(&a[3]), t.width, t.height, &t.rgba).unwrap();
}
