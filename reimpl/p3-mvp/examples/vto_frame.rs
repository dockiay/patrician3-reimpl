//! Descodifica un fotograma de un vídeo `.sks` (ventanas de suceso, `ui_rt/vto.rs`) como lo deja en la textura el
//! sustituto de `Vto.dll`: bytes B, G, R, 255. `vto_frame ARCHIVO.sks N salida.bgra` (sin salida: solo la cabecera).
use p3_mvp::ui_rt::vto::{decode_frame, Sks};

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let sks = Sks::parse(std::fs::read(&a[1]).expect("archivo .sks")).expect("no es SK10");
    println!("{}x{} {} fps, {} fotogramas, cabecera {:x?}", sks.width(), sks.height(), sks.fps(), sks.frames.len(), sks.header);
    let n: usize = a.get(2).and_then(|v| v.parse().ok()).unwrap_or(0);
    let (w, h, px) = decode_frame(sks.jpeg(n)).expect("JPEG");
    println!("fotograma {n}: {w}x{h}");
    if let Some(out) = a.get(3) {
        let bgra: Vec<u8> = px.chunks_exact(3).flat_map(|p| [p[0], p[1], p[2], 255]).collect();
        std::fs::write(out, bgra).unwrap();
    }
}
