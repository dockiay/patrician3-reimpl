//! Misma entrada que el .wasm, en nativo: `native_bench ui EXE CPR menu|city FRAMES` o `native_bench sim SAVE EXE CPR DAYS`.
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let rd = |i: usize| std::fs::read(&a[i]).unwrap();
    let out = match a[1].as_str() {
        // `P3_STREAMED`: el CPR por tramos desde el disco, como en la web (no entra entero en memoria); al final dice en stderr
        // cuántos bytes de archivos retiene su caché.
        "ui" if std::env::var_os("P3_STREAMED").is_some() => {
            use p3_assets::cpr::{Archive, FileRange, StreamedArchive};
            let streamed = StreamedArchive::open(std::sync::Arc::new(FileRange::open(&a[3]).unwrap())).unwrap();
            let out = p3_wasm::ui_bench_archive(&rd(2), &Archive::from_streamed(&streamed), &a[4], a[5].parse().unwrap());
            eprintln!("mem cpr_cache {:.1} MiB", streamed.cached_bytes() as f64 / 1048576.0);
            out
        }
        "ui" => p3_wasm::ui_bench(&rd(2), &rd(3), &a[4], a[5].parse().unwrap()),
        "sim" => p3_wasm::sim_run(&rd(2), &rd(4), &rd(3), a[5].parse().unwrap()),
        _ => panic!("ui|sim"),
    };
    println!("{}", out.unwrap());
}
