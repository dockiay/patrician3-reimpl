use p3_assets::cpr::Archive;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let bytes = std::fs::read(&a[1]).unwrap();
    let ar = Archive::parse(&bytes).unwrap();
    let e = ar.find(a[2].as_bytes()).unwrap();
    let img = p3_assets::aim::decode(ar.read(e).unwrap()).unwrap();
    println!(
        "{}x{} bpp {} fmt {} pitch {} px {} pal {:?}",
        img.width,
        img.height,
        img.bytes_per_pixel,
        img.format,
        img.pitch,
        img.pixels.len(),
        img.palette.as_ref().map(|p| p[..16].to_vec())
    );
    let hist = img.pixels.iter().fold([0usize; 256], |mut h, &b| {
        h[b as usize] += 1;
        h
    });
    println!(
        "{:?}",
        hist.iter()
            .enumerate()
            .filter(|(_, c)| **c > 0)
            .take(8)
            .collect::<Vec<_>>()
    );
}
