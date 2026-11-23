// Las cadenas de llamadas elevadas son profundas; la pila de WASM es de 1 MB por defecto. Solo afecta a este crate.
fn main() {
    if std::env::var("CARGO_CFG_TARGET_ARCH").as_deref() == Ok("wasm32") {
        println!("cargo:rustc-link-arg-cdylib=-zstack-size=33554432");
        println!("cargo:rustc-link-arg-cdylib=--initial-memory=268435456");
        println!("cargo:rustc-link-arg-cdylib=--max-memory=4294967296");
    }
}
