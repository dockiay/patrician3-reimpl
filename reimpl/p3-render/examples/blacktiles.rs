use p3_assets::cpr::Archive;
use p3_assets::let_sheet::LetSheet;
use p3_assets::town::TownMap;
use std::collections::{BTreeMap, HashMap};
fn fold(p: &[u8]) -> Vec<u8> { p.iter().take_while(|&&b| b != 0).map(|&b| if b == b'\\' { b'/' } else { b.to_ascii_lowercase() }).collect() }
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let bytes = std::fs::read(&a[1]).unwrap();
    let ar = Archive::parse(&bytes).unwrap();
    let mut sheets: HashMap<u16, LetSheet> = HashMap::new();
    for e in &ar.entries { let n = fold(&e.name); if n.starts_with(b"iso/let/") && n.ends_with(b".let") { if let Ok(s) = LetSheet::parse(ar.read(e).unwrap()) { sheets.entry(s.tile_id).or_insert(s); } } }
    let mut used: BTreeMap<u16, usize> = BTreeMap::new();
    for e in &ar.entries {
        let n = fold(&e.name);
        let Some(_) = n.strip_prefix(b"iso/towns/") else { continue };
        let Ok(m) = TownMap::parse(ar.read(e).unwrap()) else { continue };
        for layer in 0..2 { for c in &m.layers[layer] { if c.tile != 0 && c.b2 != 0xFF { *used.entry(c.tile).or_default() += 1; } } }
    }
    for (tile, uses) in used {
        let Some(s) = sheets.get(&tile) else { continue };
        let src = fold(&s.source);
        let Some(te) = ar.entries.iter().rev().find(|e| e.kept && fold(&e.name) == src) else { continue };
        let Ok(img) = p3_assets::aim::decode(ar.read(te).unwrap()) else { continue };
        let Some(t) = p3_render::to_rgba(&img) else { continue };
        let mut black_frames = 0;
        for fr in &s.strips {
            let (mut opaque, mut dark) = (0, 0);
            for j in 0..fr[3] as i32 { for i in 0..fr[2] as i32 {
                let (sx, sy) = ((fr[0] as i32 + i) as usize, (fr[1] as i32 + j) as usize);
                if sx >= t.width || sy >= t.height { continue; }
                let p = &t.rgba[(sy * t.width + sx) * 4..][..4];
                if p[3] != 0 { opaque += 1; if p[0] < 8 && p[1] < 8 && p[2] < 8 { dark += 1; } }
            } }
            if opaque > 0 && dark * 10 >= opaque * 9 { black_frames += 1; }
        }
        if black_frames > 0 { println!("tile {tile}: {black_frames}/{} fotogramas negros, {uses} usos, fuente {}", s.strips.len(), String::from_utf8_lossy(&src)); }
    }
}
