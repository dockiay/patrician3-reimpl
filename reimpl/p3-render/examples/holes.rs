use p3_assets::cpr::Archive;
use p3_assets::town::TownMap;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let bytes = std::fs::read(&a[1]).unwrap();
    let archive = Archive::parse(&bytes).unwrap();
    let want = format!("iso/towns/{}.town", a[2]);
    let e = archive.entries.iter().find(|e| String::from_utf8_lossy(&e.name).to_lowercase().replace('\\', "/") == want).unwrap();
    let m = TownMap::parse(archive.read(e).unwrap()).unwrap();
    println!("{}x{}", m.width, m.height);
    let mut n = 0;
    for y in 2..m.height as usize - 2 {
        for x in 2..m.width as usize - 2 {
            let c = m.layers[0][y * m.width as usize + x];
            if c.tile == 0 {
                n += 1;
                if n < 12 { println!("hueco capa0 ({x},{y}) {:?} capa1 {:?}", c, m.layers[1][y * m.width as usize + x]); }
            }
        }
    }
    println!("{n} huecos interiores");
}
