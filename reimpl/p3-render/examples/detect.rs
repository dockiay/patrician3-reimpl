use p3_assets::aim;
fn main() {
    let d = std::fs::read(std::env::args().nth(1).unwrap()).unwrap();
    let im = aim::decode(&d).unwrap();
    println!("{}x{} bpp={} format={:#x} pitch={}", im.width, im.height, im.bytes_per_pixel, im.format, im.pitch);
    for y in 0..im.height as usize {
        let row: Vec<String> = (0..im.width as usize).map(|x| format!("{:02x}", im.pixels[y * im.pitch as usize + x])).collect();
        println!("{}", row.join(""));
    }
}
