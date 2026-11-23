//! Reescribe con `aim::encode` cada `.aim` descodificable de `P3_CPR` en el directorio `argv[1]` (nombre = índice
//! del archivo en el orden del CPR) para que el oráculo `reimpl/oracle/aim/aim_reencoded.py` los pase por `AIM.dll`.
use p3_assets::{aim, cpr::Archive};
fn main() {
    let dir = std::env::args().nth(1).expect("directorio de salida");
    std::fs::create_dir_all(&dir).unwrap();
    let bytes = std::fs::read(std::env::var("P3_CPR").expect("P3_CPR")).unwrap();
    let archive = Archive::parse(&bytes).unwrap();
    let mut index = String::new();
    let (mut ok, mut skipped, mut orig, mut new) = (0, 0, 0usize, 0usize);
    for e in &archive.entries {
        let name: String = e.name.iter().map(|&b| b as char).collect();
        if !name.to_lowercase().ends_with(".aim") {
            continue;
        }
        let data = archive.read(e).unwrap();
        let Ok(img) = aim::decode(data) else {
            skipped += 1;
            continue;
        };
        let out = aim::encode(&img).unwrap();
        let back = aim::decode(&out).unwrap();
        assert_eq!(back, img, "{name}: ida y vuelta en Rust");
        orig += data.len();
        new += out.len();
        std::fs::write(format!("{dir}/{ok}.aim"), &out).unwrap();
        index.push_str(&format!("{ok}\t{name}\n"));
        ok += 1;
    }
    std::fs::write(format!("{dir}/index.tsv"), index).unwrap();
    eprintln!("{ok} reescritos, {skipped} sin soporte; {orig} -> {new} bytes");
}
