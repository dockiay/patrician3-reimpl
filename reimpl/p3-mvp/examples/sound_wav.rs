//! Renderiza una muestra del juego a WAV: `sound_wav ARCHIVO.cpr ID SALIDA.wav` (ID de `sound.ini`, p. ej. 13001).
use p3_assets::cpr::Archive;
use p3_mvp::audio::SoundCatalog;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let cpr = std::fs::read(&a[1]).unwrap();
    let archive = Archive::parse(&cpr).unwrap();
    let cat = SoundCatalog::from_archive(&archive).unwrap();
    let id: u32 = a[2].parse().unwrap();
    let pcm = cat.render(&archive, id).unwrap();
    std::fs::write(&a[3], pcm.to_wav()).unwrap();
}
