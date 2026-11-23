#!/usr/bin/env bash
# Compila el motor a WebAssembly y lo deja en www/p3_web.wasm (no se versiona).
#
# Variante con excepciones WASM (panic=unwind, std recompilada): `ui_rt` usa el desenrollado de pila como control de flujo
# (`AbortRun`: límite de llamadas, de profundidad o de montón, o cerrar el juego durante una carga) y con panic=abort eso es un
# trap. Necesita `rustup component add rust-src` y RUSTC_BOOTSTRAP=1 (-Zbuild-std); ver `p3-wasm/README.md`, hallazgo 1.
# La pila de 16 MB hay que repetirla en RUSTFLAGS porque sustituye a `.cargo/config.toml`.
set -euo pipefail
export RUST_MIN_STACK="${RUST_MIN_STACK:-268435456}" # rustc se desborda de pila con p3-mvp en Windows
cd "$(dirname "$0")"
rustup target add wasm32-unknown-unknown >/dev/null 2>&1 || true
rustup component add rust-src >/dev/null 2>&1 || true
TARGET_DIR="${P3_TARGET_DIR:-../target}"
RUSTC_BOOTSTRAP=1 RUSTFLAGS="-C panic=unwind -C target-feature=+exception-handling,+simd128 -C link-arg=-zstack-size=16777216" \
  cargo build -p p3-web --release -Zbuild-std=std,panic_unwind --target wasm32-unknown-unknown --target-dir "$TARGET_DIR" ${P3_CARGO_ARGS:-}
cp "$TARGET_DIR/wasm32-unknown-unknown/release/p3_web.wasm" www/p3_web.wasm
# JS de las partidas (WEB-4, p3-web-save): copia, no se versiona (ver .gitignore de www/)
cp ../p3-web-save/js/p3-saves-main.js ../p3-web-save/js/p3-saves-worker.js www/
# Worklet y pegamento del audio (WEB-5, p3-audio/web): copia
cp ../p3-audio/web/p3-audio-worklet.js ../p3-audio/web/p3-audio-web.js www/
ls -l www/p3_web.wasm
