//! Dibuja el terreno de una ciudad del CPR a PNG: `render_town RUTA.cpr ID SALIDA.png`.
use p3_assets::cpr::Archive;
use p3_assets::town::TownMap;
use p3_render::{write_png, Scene};

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let bytes = std::fs::read(&a[1]).unwrap();
    let archive = Archive::parse(&bytes).unwrap();
    let want = format!("iso/towns/{}.town", a[2].to_lowercase());
    let entry = archive
        .entries
        .iter()
        .find(|e| String::from_utf8_lossy(&e.name).to_lowercase().replace('\\', "/") == want)
        .expect("ciudad no encontrada");
    let map = TownMap::parse(archive.read(entry).unwrap()).unwrap();
    let mut scene = Scene::new(&archive);
    let (w, h, rgba) = scene.render(&map, &a.get(4).map(|s| s.split(',').map(|v| v.parse().unwrap()).collect::<Vec<usize>>()).unwrap_or(vec![0, 1]));
    write_png(std::path::Path::new(&a[3]), w, h, &rgba).unwrap();
    println!("{w}x{h}");
}
