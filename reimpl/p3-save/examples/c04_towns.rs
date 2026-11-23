//! Extrae ciudades parciales de guardados locales para contrastes C04. La salida contiene datos privados.
use p3_save::{world::World, Save};
use sha2::{Digest, Sha256};

fn main() {
    let paths: Vec<_> = std::env::args_os().skip(1).collect();
    assert!(!paths.is_empty(), "uso: c04_towns PARTIDA.pat [...]");
    for path in paths {
        let file = std::fs::read(path).expect("no se pudo leer la partida");
        let hash = format!("{:x}", Sha256::digest(&file));
        let save = Save::parse(&file).expect("partida inválida");
        let world = World::parse(save.block(0)).expect("bloque game_world no reconocido");
        let towns: Vec<String> = (0..world.town_count()).map(|i| {
            world.town(i).0.iter().map(|b| format!("{b:02x}")).collect()
        }).collect();
        println!("{}", serde_json::json!({"save_sha256":hash,"towns":towns}));
    }
}
