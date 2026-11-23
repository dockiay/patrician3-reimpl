// Windows (MinGW): OpenH264 es C++ y Rust enlazaría libstdc++-6.dll (que el PC no tiene). Se pone en una carpeta propia, antes que la
// del compilador, solo la libstdc++.a estática: el enlazador la elige en vez del import de la DLL. libgcc y winpthread también estáticos.
fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    if std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("gnu") && std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        let cxx = std::env::var("CXX_x86_64_pc_windows_gnu").unwrap_or_else(|_| "x86_64-w64-mingw32-g++".into());
        if let Ok(out) = std::process::Command::new(&cxx).arg("-print-file-name=libstdc++.a").output() {
            let path = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if std::path::Path::new(&path).is_file() {
                let dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("static-cxx");
                std::fs::create_dir_all(&dir).unwrap();
                std::fs::copy(&path, dir.join("libstdc++.a")).unwrap();
                println!("cargo:rustc-link-search=native={}", dir.display());
            }
        }
        for a in ["-static-libgcc", "-static"] {
            println!("cargo:rustc-link-arg-bins={a}");
        }
    }
}
