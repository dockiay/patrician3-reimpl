# Juego web: rendimiento y memoria (WEB-6, 14/11/2026, nube de Claude)

Objetivo: 60 fps en ciudad en un portátil corriente, menos memoria (Sergio quiere jugar en iPhone) y menos tiempo de carga,
**sin cambiar lo que se dibuja**: los hashes de los bancos siguen iguales (apartado «Paridad»).

## Resultado

Banco `ui city` de `p3-wasm` (menú → partida nueva → ciudad, 800×600, 200 fotogramas), contenedor de 4 núcleos, Node 22 (V8).
El banco calcula un FNV-1a de la pantalla en cada fotograma (2,6–4 ms en WASM, 1,5 ms en nativo) que el juego real no hace.

| Medida | Antes (main bf55d93) | Después | |
|---|---|---|---|
| WASM, fotograma de ciudad (mediana, con el hash del banco) | 16,2–19,9 ms | 12,3–14,9 ms | −24 % |
| WASM, fotograma sin el hash del banco (perfil de Node) | ≈ 14 ms | ≈ 7,5–9 ms | 110–130 fps |
| WASM, abrir la ciudad (partida nueva) | 7,5–8,0 s | 6,1–7,0 s | −13 % |
| WASM, memoria lineal del banco (CPR entero dentro, 240 MiB) | 508 MiB | 432 MiB | −76 MiB |
| Nativo, fotograma de ciudad | 10,3–11,7 ms | 8,6–9,8 ms | −17 % |
| Nativo con CPR por tramos (como la web), memoria máxima del proceso | ≈ 387 MiB (278 + 109 de caché del CPR) | 278 MiB; **170 MiB** con tope de texturas y modo de poca memoria | −56 % |
| `.wasm` (variante con excepciones) | 35,3 MB | 35,2 MB; 26,9 MB con `wasm-opt -O3` | −24 % |

Las horquillas son de varias pasadas: el contenedor varía un ±15 % entre ejecuciones.

## De dónde salía el tiempo (perfil antes de tocar nada)

`valgrind --tool=callgrind` por líneas sobre `native_bench` (diferencia entre 30 y 90 fotogramas) y `node --cpu-prof` sobre el
`.wasm`. Un fotograma de ciudad eran 82,6 M de instrucciones:

| Parte | % del fotograma |
|---|---|
| `sgl_LockTexture(-4)` / `sgl_UnlockTexture`: copiar la pantalla entera a un espejo BGRA y volver, byte a byte (ondas del agua) | 25 % |
| Hash del banco (no es del juego) | 14 % |
| Reparto de llamadas del enlazador (`dispatch_inner`, `builtin`, `ret_pop`, contador `ported`) | 22 % |
| Lecturas y escrituras de memoria (`PageMem` tras `dyn Mem`, y otra vez tras el adaptador `CoreMem`) | 15 % |
| Dibujo (`blit::row`, mezcla): 2 % en nativo con SSE2, 9 % en WASM, que iba por la ruta escalar | 2–9 % |

## Qué se cambió

Velocidad (ninguno cambia un byte de lo que se dibuja ni del estado del juego):

1. **Espejo de pantalla de las ondas** (`ui_rt/mod.rs`, `sgl_LockTexture`/`sgl_UnlockTexture` con `-4`): el intercambio R↔B se hace
   por palabras de 32 bits y se vectoriza.
2. **Filas de `blit` con `simd128`** (`ui_rt/blit.rs`, `wasm_row`): la misma secuencia que la ruta SSE2 (carriles de 16 bits).
   `blit::self_check` compara 7.920 casos contra la ruta escalar; se ejecuta dentro del `.wasm` con
   `node node/run.mjs P3.wasm selftest`. `.cargo/config.toml` y `p3-web/build.sh` compilan ya con `+simd128`
   (Chrome 91+, Firefox 89+, Safari 16.4+).
3. **`ret_pop`** (`p3-render/src/ret_pop.rs`): índice por tramos de 256 direcciones en lugar de bisecar 3.214 filas en cada llamada.
4. **Contador `ported`**: se lleva en una tabla de dispersión y se vuelca al `BTreeMap` público al volver de `Runtime::call`
   (era un 4 % de la carga de una ciudad).
5. **Nombres de `sgl_*` e importaciones** como `Rc<str>`: ya no se copia una `String` en cada llamada.
6. **Sin adaptador doble de memoria**: `CoreSide::call` recupera el `PageMem` (`Mem::as_any_mut`) en vez de envolverlo en
   `CoreMem`, que añadía un salto indirecto a cada acceso de todo lo que colgaba de esa llamada.
7. **`operator new`/`LocalAlloc`/bloques de Miles** ponen a cero por tramos (`zero_fill`), no con una llamada por byte.
8. Los 16 argumentos que se pasan a los portes de `p3-render::j07` van en la pila de Rust, sin reservar un `Vec` por llamada.
9. **Web** (`p3-web/src/lib.rs`): la página recibe directamente la superficie del enlazador. Antes cada fotograma se copiaba al
   búfer trasero del dispositivo, se intercambiaba y se volvía a copiar (dos copias de pantalla por fotograma y dos búferes fijos:
   16 MiB a 1920×1080).

Memoria:

10. **Tope para las texturas descodificadas** (`Assets::set_image_budget`). Entrar en una ciudad dejaba 149 MiB de imágenes RGBA
    (138 rehacibles desde el CPR). Con tope, las que llevan más tiempo sin usarse se sueltan y vuelven a `pending`; se
    descodifican otra vez, con la misma clave de color, cuando alguien las pide. Las que el juego modifica, usa como destino o
    recarga dejan de ser rehacibles (`Assets::pin`). Si lo que se dibuja no cabe (se rehacen texturas recién soltadas), el tope
    crece un octavo cada 64 veces hasta cuatro veces el inicial. Por defecto: **48 MiB en `wasm32`**, sin tope en nativo
    (`P3_IMAGE_BUDGET_MB=N` para probar; 0 = sin tope). Con 32 MiB el banco hace 733 descodificaciones de más y va igual de
    fluido; con 16 MiB sin crecimiento se rehacían 116 por fotograma (45 ms).
11. **El CPR por tramos ya no guarda cada textura que lee** (`Archive::read_transient`, `Assets::file_transient`). La caché de
    `StreamedArchive` retenía para siempre cada archivo pedido: 109 MiB al entrar en una ciudad. Las imágenes se leen, se
    descodifican y se sueltan; la caché se queda en 0,5 MiB.
12. **Modo de poca memoria** (`Runtime::set_low_memory`, apagado por defecto). El montón emulado guarda los bloques liberados en
    listas por tamaño exacto; al entrar en una ciudad había 50 MiB libres sin devolver (41 en bloques de 16 KiB o más). En este
    modo las páginas enteras de esos bloques se sueltan al liberar (`PageMem::release_pages`): la memoria del juego pasa de 79 a
    44 MiB. El Worker lo enciende en móviles (`worker.js`, `opts.lowMemory` lo fuerza). Ver «Límites».

## Cómo medir

```
cargo build --release -p p3-wasm --example native_bench
P3_MARK=1 native_bench ui EXE CPR city 200 > nat.json         # stderr: marca por fotograma y reparto de memoria («mem …»)
P3_STREAMED=1 P3_LOW_MEMORY=1 P3_IMAGE_BUDGET_MB=48 P3_MARK=1 native_bench ui EXE CPR city 200   # como la web en un móvil
P3_MARK=1 node node/run.mjs P3.wasm ui EXE CPR city 200 > wasm.json   # stderr: carga, mediana por fotograma, memoria lineal
node node/run.mjs P3.wasm selftest
python3 compare.py nat.json wasm.json
node --cpu-prof --cpu-prof-interval 200 node/run.mjs P3.wasm ui EXE CPR city 400   # perfil por función del .wasm
```
`P3_MARK` no entra en el JSON ni en los hashes. El `.wasm` se compila con la variante B de `p3-wasm/README.md` añadiendo
`+simd128` a `target-feature`.

## Paridad

- WASM antes y después: JSON **idéntico** (200 fotogramas, memoria completa y sin pila, llamadas).
- Nativo antes y después: idéntico, también con `P3_IMAGE_BUDGET_MB` = 64, 48, 32 y 16 y con el CPR por tramos.
- Nativo frente a WASM: memoria completa y sin pila iguales; fotogramas 16–200 iguales. Los fotogramas 1–15 (fondo JPEG del
  menú) ya diferían en `bf55d93` con el `native_bench` de x86-64, porque `jpeg-decoder` usa su IDCT con SIMD en nativo (hallazgo 2
  de `p3-wasm/README.md`); no lo cambia este trabajo.
- Modo de poca memoria: los 200 fotogramas iguales. El hash de memoria no es comparable por construcción (las páginas de
  bloques ya liberados quedan a cero).
- Pruebas: `p3-render` `ret_pop` (índice contra bisección en todas las filas y vecinas), `p3-assets` (27), `p3-mvp --lib`
  `vector_rows`/`memory_bridge`, `p3-web --lib` (8), y `p3-mvp --test ui_menus` con el EXE y el CPR reales.

## Tamaño y tiempo de carga del `.wasm`

- 33,4 de los 35,2 MB son código (casi todo, las funciones elevadas de `ui_rt`), 0,5 datos y 1,4 la sección de nombres.
- `wasm-opt` nativo (binaryen 123 descargado de GitHub en `web/.tools`, que `build_web.py` ya busca) tarda **6,5 min** con
  2 núcleos (25 min con el de npm). `-Os` y `-O3` dejan el mismo tamaño, 26,9 MB (6,6 MB gzip frente a 7,1), con los mismos
  hashes. La diferencia es la velocidad en V8, mismo banco:

  | `.wasm` | Fotograma | Abrir la ciudad |
  |---|---|---|
  | sin `wasm-opt` (35,2 MB) | 13,8–14,9 ms | 6,1–7,6 s |
  | `-O3` (26,9 MB) | 14,3–15,0 ms | 7,4–8,5 s |
  | `-Os` (26,9 MB) | 17,7–19,8 ms | 8,0–10,2 s |

  `build_web.py` usa ahora `-O3` por defecto: un 24 % menos de descarga y de código que compilar a cambio de poco o nada
  (dentro del ruido en el fotograma; la carga sale algo más lenta en dos pasadas).
- V8 compila perezosamente: instanciar tarda 80–160 ms; el coste aparece repartido durante la primera carga.

## Límites y pendientes

- No se ha medido en un navegador con el `p3-web` completo ni en Safari/iPhone: los números son del banco en Node y del banco
  nativo con el CPR por tramos. El hilo del montaje web debería repetir su medida de memoria en ciudad (antes ≈ 1 GB).
- Modo de poca memoria: quien pide un bloque lo pone a cero, así que solo cambia lo que leería un uso tras liberar (ceros en vez
  del contenido viejo). En los bancos no cambia ningún fotograma; no se ha probado en batallas ni en la carta náutica.
- El tope de texturas solo cubre las cargadas con `sgl_LoadMemoryTexture` (el 93 %); las de `textures.ini` y las fuentes quedan.
- Lo que queda del fotograma es estructural: cada acceso a memoria del código elevado es una llamada indirecta a `dyn Mem`.
  Quitarla exige generar el código elevado contra `PageMem` concreto (cambio del elevador `tools/lift.py`, todos los `*_lifted.rs`).
- La carga de una ciudad en WASM sigue siendo 2 veces la nativa por lo mismo.
- En la web, cada fotograma aún pasa por `putImageData` + `transferToImageBitmap` + `postMessage` (`worker.js`).
