# p3-wasm: WEB-0 y WEB-1 (14/11/2026, nube de Claude)

Punto de entrada mínimo de `p3-mvp` para `wasm32-unknown-unknown`, sin `wasm-bindgen`. Exporta `p3_alloc`, `p3_free`,
`p3_ui_bench`, `p3_sim_run`, `p3_result_ptr` y `p3_last_panic`. Los bytes del EXE, CPR y partida los pone el anfitrión; no se lee
ningún archivo. Nunca se suben EXE ni assets a Git.

> **WEB-6 (14/11/2026):** rendimiento y memoria, con medidas antes/después y cómo perfilar: [`../WEB_RENDIMIENTO.md`](../WEB_RENDIMIENTO.md).
> El banco acepta ahora `P3_MARK=1` (tiempo por fotograma y reparto de memoria en stderr), `P3_STREAMED=1` (CPR por tramos en
> `native_bench`) y `node node/run.mjs P3.wasm selftest`. La variante B se compila con `+exception-handling,+simd128`.
> Los resultados de abajo son los de WEB-1.

## Compilar

```
# A) estándar (panic=abort): sirve para p3_sim_run y para cualquier ruta que no desenrolle
cargo build --release --target wasm32-unknown-unknown -p p3-wasm          # 33,4 MB, 6,9 MB en gzip, ~2,5 min tras la primera vez (5 min en frío)

# B) con excepciones WASM (panic=unwind): imprescindible para ui_rt (ver «Hallazgo»)
RUSTC_BOOTSTRAP=1 RUSTFLAGS="-C panic=unwind -C target-feature=+exception-handling,+simd128 -C link-arg=-zstack-size=16777216" \
  cargo build --release -Zbuild-std=std,panic_unwind --target wasm32-unknown-unknown -p p3-wasm --target-dir /tmp/p3-eh
# (necesita `rustup component add rust-src`; 35,3 MB; ~7 min en frío)
```
`.cargo/config.toml` ya fija la pila a 16 MB en la variante A; en B hay que repetirla en `RUSTFLAGS` (lo sustituye).

## Ejecutar y comparar con nativo

```
cargo build --release -p p3-wasm --example native_bench
native_bench ui EXE CPR city 60 > nat.json        # ui_rt 800x600, FNV-1a de cada fotograma y de la memoria
native_bench sim AUTO.pat EXE CPR 10 > nat_sim.json   # p3-sim como c10_snapshots --human-battles-auto --battle-pacing clock
node node/run.mjs p3_wasm.wasm ui EXE CPR city 60 > wasm.json
node node/run.mjs p3_wasm.wasm sim AUTO.pat EXE CPR 10 > wasm_sim.json
python3 compare.py nat.json wasm.json
```

## Resultados (Node 22.23 / V8, contenedor de 4 núcleos; EXE inglés, `research/native-ui/2026-11-12/source-save/AUTO.pat`)

| Banco | Nativo | WASM (Node) | Hashes |
|---|---|---|---|
| `ui menu`, 60 fotogramas (`calls`=81, memoria) | 1,0 s | 1,3 s | idénticos |
| `ui city`, 60 fotogramas | 6,5 s | 13,5 s (2,1×) | idénticos (59/60 de cada fotograma + memoria completa y sin pila) |
| `sim`, 10 días (11 guardados, SHA-256 y FNV-1a) | 1,4 s | 3,4 s (2,4×) | 11/11 idénticos |

Carga del módulo (33 MB): Node y Chromium 153 headless compilan de forma perezosa en ~80 ms; con compilación completa TurboFan
(`--no-liftoff --no-wasm-lazy-compilation`) 9,4 s. `WebAssembly.compileStreaming` en Chromium: 153 ms. Memoria lineal: 319–512 MiB
(el CPR entero va en memoria; WEB-2 lo evita con el lector por tramos). **No se probó en Firefox ni Safari** (no hay en el contenedor).

## Hallazgos

1. **`ui_rt` usa el desenrollado de pila como control de flujo** (`AbortRun` vía `resume_unwind`: fin del banco con `call_limit`,
   `on_pump` falso, límite de profundidad o de montón). Con `panic=abort` (variante A) eso es un trap `unreachable`. La variante B
   (std recompilada con excepciones WASM; V8 de Node 22 y Chromium 153 las ejecutan) lo resuelve sin tocar código y da los hashes
   de arriba. Para el Worker de web, que no termina, solo importa si el juego llega a esos límites; conviene usar B o sustituir
   `AbortRun` por un indicador de salida antes de WEB-4. `p3_sim_run` no desenrolla y funciona con A y con B.
2. **`jpeg-decoder` difiere entre x86 y WASM**: en x86 usa un IDCT con SIMD y algunos píxeles del fondo del menú cambian de valor frente
   al IDCT portable. Con `platform_independent` (ahora activo solo en wasm32, `p3-mvp/Cargo.toml`) nativo y WASM coinciden bit a bit.
   La simulación no usa JPEG. Nativo no cambia.
3. `Instant` (perfiles) y los hilos del compositor de ciudad y de la descodificación de audio ya no abortan en wasm32 (reloj nulo,
   un hilo; el audio cooperativo lo puso otro hilo). `FileRange::read_at` no compilaba en wasm32: añadido un brazo `Unsupported`.
