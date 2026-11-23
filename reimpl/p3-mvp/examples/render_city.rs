//! Dibuja la ciudad seleccionada de una partida: `render_city PARTIDA.pat ARCHIVO.cpr SALIDA.png [RELOJ_MS]`.
use p3_mvp::Session;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let save = std::fs::read(&a[1]).unwrap();
    let cpr = p3_assets::cpr::Bytes::map_file(&a[2]).unwrap();
    let s = Session::open(&save, cpr).unwrap();
    let clock = a.get(4).and_then(|c| c.parse().ok()).unwrap_or(0);
    let result = s.render_at(std::path::Path::new(&a[3]), clock).unwrap();
    println!("{result}");
}
