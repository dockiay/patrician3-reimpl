use p3_assets::cpr::Archive;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let bytes = std::fs::read(&a[1]).unwrap();
    let ar = Archive::parse(&bytes).unwrap();
    let e = ar
        .entries
        .iter()
        .find(|e| {
            e.name.ends_with(b"Rasen_Grass_8uhr_1.let")
                || e.name.to_ascii_lowercase().ends_with(b"rasen_grass_8uhr_1.let")
        })
        .unwrap();
    let s = p3_assets::let_sheet::LetSheet::parse(ar.read(e).unwrap()).unwrap();
    println!(
        "{:?} src {:?} strips {:?} tile {} {}",
        String::from_utf8_lossy(&e.name),
        String::from_utf8_lossy(&s.source),
        s.strips,
        s.tile_width,
        s.tile_height
    );
    let key: Vec<u8> = s
        .source
        .iter()
        .map(|&b| if b == b'\\' { b'/' } else { b.to_ascii_lowercase() })
        .collect();
    println!(
        "{:?}",
        ar.entries
            .iter()
            .filter(|e| e
                .name
                .iter()
                .map(|&b| if b == b'\\' { b'/' } else { b.to_ascii_lowercase() })
                .collect::<Vec<u8>>()
                == key)
            .count()
    );
}
