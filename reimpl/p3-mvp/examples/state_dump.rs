//! Vuelca `Session::state()` de una partida: `state_dump PARTIDA.pat ARCHIVO.cpr`.
use p3_mvp::Session;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let s = Session::open(&std::fs::read(&a[1]).unwrap(), std::fs::read(&a[2]).unwrap()).unwrap();
    println!("{}", serde_json::to_string_pretty(&s.state()).unwrap());
}
