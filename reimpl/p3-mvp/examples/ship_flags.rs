fn main() {
    let a: Vec<String> = std::env::args().collect();
    let s = p3_mvp::Session::open(&std::fs::read(&a[1]).unwrap(), std::fs::read(&a[2]).unwrap()).unwrap();
    let mut n = 0;
    for (i, im) in s.sim.host.expedition_ship_images.iter().enumerate() {
        let Some(im) = im else { continue };
        let f = im[0x12a];
        let st = u16::from_le_bytes([im[0x134], im[0x135]]);
        if f & 4 != 0 {
            n += 1;
            println!(
                "ship {i} owner {} flags {f:#x} +0x11 {:#x} state {st:#x} x {} y {} +4 {}",
                im[0],
                im[0x11],
                i32::from_le_bytes([im[8], im[9], im[10], im[11]]),
                i32::from_le_bytes([im[12], im[13], im[14], im[15]]),
                u16::from_le_bytes([im[4], im[5]])
            );
        }
    }
    println!("{n} with bit4; city {}", s.state()["city"]);
}
