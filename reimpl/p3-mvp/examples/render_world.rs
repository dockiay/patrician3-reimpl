//! Mapa del mundo de una partida: `render_world PARTIDA.pat ARCHIVO.cpr SALIDA.png`.
use p3_mvp::Session;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let s = Session::open(&std::fs::read(&a[1]).unwrap(), std::fs::read(&a[2]).unwrap()).unwrap();
    let r = s.render_world(std::path::Path::new(&a[3])).unwrap();
    println!("{}", serde_json::to_string_pretty(&r).unwrap());
}
