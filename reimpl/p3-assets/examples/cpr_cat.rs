//! Escribe en la salida estándar un archivo del CPR: `cpr_cat <archivo.cpr> <ruta>`.
use std::io::Write;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let data = std::fs::read(&a[1]).expect("cpr");
    let arc = p3_assets::cpr::Archive::parse(&data).expect("cpr válido");
    let e = arc.find(a[2].as_bytes()).expect("no está");
    std::io::stdout().write_all(arc.read(e).expect("lectura")).unwrap();
}
