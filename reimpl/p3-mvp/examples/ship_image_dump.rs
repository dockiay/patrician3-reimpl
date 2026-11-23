//! Diagnóstico: `ship_image_dump PAYLOAD.bin BARCO [DESDE LARGO]` imprime la imagen 0x180 de un barco
//! (descomprimida del guardado) y su ruta lógica.
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let data = std::fs::read(&a[1]).unwrap();
    let k: usize = a[2].parse().unwrap();
    let from = a.get(3).map(|v| usize::from_str_radix(v.trim_start_matches("0x"), 16).unwrap()).unwrap_or(0);
    let len = a.get(4).map(|v| usize::from_str_radix(v.trim_start_matches("0x"), 16).unwrap()).unwrap_or(0x180);
    let save = p3_save::Save::parse_unpacked(&data).unwrap();
    let ships = p3_save::ships::Ships::parse(save.block_named("ships").unwrap()).unwrap();
    let image = ships.ship_image(k).unwrap();
    for (i, c) in image[from..from + len].chunks(16).enumerate() {
        println!("{:04x}: {}", from + 16 * i, c.iter().map(|b| format!("{b:02x}")).collect::<Vec<_>>().join(" "));
    }
    println!("ruta: {:?}", ships.saved_route(k));
}
