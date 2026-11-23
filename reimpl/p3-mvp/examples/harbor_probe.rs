//! Diagnóstico J05: barcos del puerto de la ciudad de una partida: `harbor_probe PARTIDA.pat ARCHIVO.cpr`.
use p3_assets::cpr::Archive;
use p3_mvp::Session;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let save = std::fs::read(&a[1]).unwrap();
    let cpr = std::fs::read(&a[2]).unwrap();
    let s = Session::open(&save, cpr.clone()).unwrap();
    let archive = Archive::parse(&cpr).unwrap();
    let id = s.state()["city"].clone();
    println!("ciudad {id}");
    let file_id = s.town_file_id();
    println!(
        "rutas de ranura: {}",
        p3_mvp::harbor::slot_routes(&archive, file_id).len()
    );
    for p in s.harbor_ships(&archive, file_id, 0) {
        println!("{p:?}");
    }
}
