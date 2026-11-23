//! Vuelca un archivo del `.cpr`: `cpr_cat ARCHIVO.cpr ruta/interna SALIDA`.
use p3_assets::cpr::Archive;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let cpr = std::fs::read(&a[1]).unwrap();
    let archive = Archive::parse(&cpr).unwrap();
    let e = archive.find(a[2].as_bytes()).expect("no está en el archivo");
    std::fs::write(&a[3], archive.read(e).expect("lectura")).unwrap();
}
