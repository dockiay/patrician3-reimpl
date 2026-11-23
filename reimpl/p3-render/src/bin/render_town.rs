//! `render_town CPR CIUDAD SALIDA.png [capas]`: dibuja la ciudad (capas `0`, `1` o `01`, por defecto `01`) con el orden fila a fila.

use p3_assets::cpr::Archive;
use p3_assets::town::TownMap;
use p3_render::{write_png, Scene};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        eprintln!("uso: render_town CPR CIUDAD SALIDA.png [capas]");
        std::process::exit(2);
    }
    let bytes = std::fs::read(&args[1]).expect("CPR");
    let archive = Archive::parse(&bytes).expect("CPR válido");
    let name = format!("iso/towns/{}.town", args[2]);
    let entry = archive
        .find(name.as_bytes())
        .unwrap_or_else(|| panic!("{name} no existe"));
    let map = TownMap::parse(archive.read(entry).expect("lectura")).expect(".town válido");
    let layers: Vec<usize> = args
        .get(4)
        .map_or("01", |s| s.as_str())
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as usize)
        .collect();
    let mut scene = Scene::new(&archive);
    let (w, h, rgba) = scene.render(&map, &layers);
    write_png(std::path::Path::new(&args[3]), w, h, &rgba).expect("PNG");
    eprintln!(
        "{w}x{h}, {} hojas .let, {} casillas sin hoja, {} fotogramas inexistentes, {} texturas sin soporte",
        scene.sheet_count(),
        scene.missing_sheets,
        scene.bad_frames,
        scene.unsupported_textures
    );
}
