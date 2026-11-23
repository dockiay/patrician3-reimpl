use p3_assets::cpr::Archive;
use p3_assets::let_sheet::LetSheet;
use p3_assets::town::TownMap;
use std::collections::HashMap;
fn fold(p: &[u8]) -> Vec<u8> { p.iter().take_while(|&&b| b != 0).map(|&b| if b == b'\\' { b'/' } else { b.to_ascii_lowercase() }).collect() }
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let bytes = std::fs::read(&a[1]).unwrap();
    let ar = Archive::parse(&bytes).unwrap();
    let want = format!("iso/towns/{}.town", a[2]);
    let e = ar.entries.iter().find(|e| fold(&e.name) == want.as_bytes()).unwrap();
    let m = TownMap::parse(ar.read(e).unwrap()).unwrap();
    let mut sheets: HashMap<u16, LetSheet> = HashMap::new();
    for e in &ar.entries { let n = fold(&e.name); if n.starts_with(b"iso/let/") && n.ends_with(b".let") { if let Ok(s) = LetSheet::parse(ar.read(e).unwrap()) { sheets.entry(s.tile_id).or_insert(s); } } }
    let (x0, x1, y0, y1): (usize, usize, usize, usize) = (a[3].parse().unwrap(), a[4].parse().unwrap(), a[5].parse().unwrap(), a[6].parse().unwrap());
    for y in y0..=y1 { for x in x0..=x1 {
        let i = y * m.width as usize + x;
        let d = |l: usize| { let c = m.layers[l][i]; match sheets.get(&c.tile) { Some(s) if c.tile != 0 => format!("t{} b2={} fr={:?} a={} b={} {}x{}", c.tile, c.b2, s.strips.get(usize::from(c.b2)), s.strips_a, s.strips_b, s.tile_width, s.tile_height), _ => format!("t{} b2={}", c.tile, c.b2) } };
        println!("({x},{y}) L0 {} | L1 {}", d(0), d(1));
    } }
}
