# p3-web: el juego en el navegador (WEB-3)

Estado (14/11/2026, WEB-4): **el juego entero corre en Chromium** con el EXE y el CPR originales: menú → nueva partida → ciudad,
carta náutica, Opciones, guardar y cargar (`.pat` + `.pst` en OPFS), resolución moderna (probado 1920×1080) y Alt+Intro. Sesión de
33 min (115.000 fotogramas, ciudad y carta alternadas) sin funciones ausentes, abortos ni pánicos; memoria de wasm estable en 1,2 GB.
Diseño: `coordination/ARQUITECTURA_WEB.md`.

## Probar
```
reimpl/p3-web/build.sh            # genera www/p3_web.wasm (30 MB, no va a Git) con excepciones WASM (-Zbuild-std, ~6 min en frío)
python reimpl/p3-web/serve.py     # http://127.0.0.1:8080 con COOP/COEP (SharedArrayBuffer)
```
En la página: «Elegir carpeta del juego» (copia solo `Patrician3.exe` y el `.cpr` a OPFS; nada sale del navegador) y «Jugar».
Prueba automática (los archivos del juego **no** están en Git): `node test/e2e.cjs CARPETA_CON_EXE_Y_CPR captura.png`.

## Cómo está hecho
| Pieza | Dónde | Notas |
|---|---|---|
| Lógica independiente de plataforma | `src/lib.rs` (`run_menu`, `attach`, rasgo `WebHost`), `src/input.rs`, `src/screen.rs` | Prueba nativa: `cargo test -p p3-web --lib`. `screen.rs` es copia de `p3-client/src/device.rs` sin `winit` (subirlo a `p3-mvp` queda pendiente) |
| Pegamento wasm | `src/web.rs` | Importaciones C del módulo `p3` y exportaciones `p3_alloc`/`p3_start`; **sin `wasm-bindgen`**, sin nightly ni memoria compartida |
| Worker | `www/worker.js` | Importa a OPFS (acceso síncrono, tramos de 8 MB), lee EXE y CPR a la memoria de WASM y no vuelve del bucle del juego |
| Entrada | `www/ring.js`, `www/main.js`, `www/vk.js` | Anillo `SharedArrayBuffer` de 1024 eventos de 4 palabras; Rust lo vacía en cada `present` y en `on_pump` (carga). Posición normalizada 0..65535 (el modo del juego puede cambiar) |
| Reloj | `p3_now_ms` → `performance.now()` en `InputState.time_ms` | |
| Fotogramas | `p3_frame_wait` = `Atomics.wait` sobre un contador que la página incrementa en cada `requestAnimationFrame` (máx. 40 ms) | |

### Desviación del diseño: ImageBitmap en vez de OffscreenCanvas transferido
Un `OffscreenCanvas` de Worker solo publica al terminar la tarea, y este Worker no vuelve jamás al bucle de eventos: se veía negro
con 479.825 píxeles de imagen correctos. El Worker pinta en su propio `OffscreenCanvas`, hace `transferToImageBitmap()` y lo envía
por `postMessage` (que sale al instante); la página lo muestra con `bitmaprenderer`. Si el hilo principal va atrasado se descartan
fotogramas viejos. WebGL2 queda para WEB-6 (escalado/filtro).

## Sonido (WEB-5 integrado)
`p3-audio::web::WebAudio` (mezclador cooperativo, sin hilos) vive en el Worker; el `AudioWorklet` de `p3-audio/web` lo vacía.
- **Desviación del diseño**: la memoria de WASM de `p3-web` **no** es compartida (sin atomics), así que el `AudioRing` vive en la memoria
  lineal y el Worker lo copia a un `SharedArrayBuffer` gemelo (misma disposición, `www/worker.js::audioSync`) que sí ve el worklet. Se
  sincroniza antes y después de cada `pump` (trae la posición de lectura y los vaciados del worklet; envía lo mezclado).
- Clic en «Jugar» → `createAudioContext()` (`www/p3-audio-web.js`) → `start` lleva `audioRate` → `p3_start(..., audio_rate)` → `attach_audio`
  (`WebAudio::new(rate, 150)`, `rt.miles.mixer`, gancho `Mixer::service` para cargas largas) → `p3_audio_attach` crea el SAB y la página conecta el
  worklet. `wrap_audio_pump` llama a `pump()` en cada `on_present` y `on_pump`. `run_game` pone `rt.miles.dirs = [game_dir]`: Miles abre
  `Musik\...` por el `HostFs` web.
- Medida: `node test/audio.cjs CARPETA [segundos]` (con `P3_CLICKS="x,y,espera_ms;..."` y `P3_H` para llegar a la ciudad) imprime vaciados del
  anillo, fotogramas por delante y RMS de la salida. Ver `reimpl/SONIDO_CLIENTE.md`. La carpeta debe contener `Musik/` además del EXE y el CPR.
- `www/p3-audio-worklet.js` y `www/p3-audio-web.js` son copias de `p3-audio/web/` que refresca `build.sh` (no se versionan).

## Interfaces para los otros hilos
- **WEB-2 (`HostFs`)**: hoy el CPR (237 MB) entra entero en la memoria de WASM y `rt.menus.game_dir` es `None` (sin partidas ni
  archivos sueltos). Con `HostFs` cambia solo cómo `run_menu` obtiene `cpr` y `game_dir`.
- **WEB-5 (sonido)**: engancharse en `run_menu` antes de `menus::boot`, como `p3_client::audio::attach`; el anillo de audio sería otro
  `SharedArrayBuffer` pasado en `init`.
- **Partidas / PWA / multijugador**: la página (`main.js`) es el sitio para importar/exportar `.pat`, registrar el service worker y
  abrir el WebSocket; el Worker recibe los mensajes con `onmessage` solo **antes** de `start`.

## Límites conocidos
- Solo menús a 800×600 (la resolución moderna y la partida entera son WEB-4). Sin sonido. Sin guardar/cargar.
- La pantalla completa es la de la página (`requestFullscreen`); sin Pointer Lock.
- No se comprueba el SHA-256 del EXE; solo se prueba la edición inglesa. Probado solo en Chromium sin cabeza (nube, Linux).
- Arranque en nube: primer fotograma a ~2,5 s del «Jugar».

## Con la carcasa PWA de WEB-6 (`reimpl/web/`)
`www/` es la carpeta estática que espera `web/build_web.py --site reimpl/p3-web/www` (rutas relativas). `build.sh`/`serve.py` de aquí
son para desarrollar rápido: compilan **este** `.wasm` (`p3_web.wasm`) y sirven `www/` con COOP/COEP. Para empaquetar con caché y
sin conexión: `python web/build_web.py --wasm reimpl/target/wasm32-unknown-unknown/release/p3_web.wasm --site reimpl/p3-web/www`
(el Worker puede pasar a `loadP3Module()` de `p3-load.js` cuando se integre; hoy usa `WebAssembly.instantiateStreaming`).
`p3-web` es un cdylib distinto de `p3-wasm` (el de WEB-0/WEB-1, pruebas de paridad sin pantalla).

## WEB-4: partida jugable (14/11/2026)
- **CPR por tramos**: `p3_assets::cpr::Archive::from_streamed(&StreamedArchive)`: el enlazador sigue usando `Archive`, pero los bytes salen de
  `FileSystemSyncAccessHandle.read` (`p3_fs_read`) la primera vez que el juego pide cada archivo; el CPR (237 MB) ya no entra en memoria.
- **`HostFs` web** (`src/webfs.rs`): solo lectura sobre los archivos abiertos del Worker (EXE, CPR, `Musik/`…); lo que se escribe fuera de
  `save/` queda en memoria. Encima va `p3-web-save::SaveFs` (`save/` en memoria + confirmación a OPFS por `postMessage`). `p3_save_*` se
  reexportan desde `web.rs` (feature `embedded` de p3-web-save).
- **Arranque**: `run_game` (`lib.rs`) = `menu()` de `p3-juego` (`ui_rt::menus::{boot, run}`, `restore_video_choice`, `enable_video_options`
  con `save/video-mode.txt`). `p3_start(modern, w, h, monitor_w, monitor_h)`: 0 = lo guardado u 800×600, 1 = automática, 2 = tamaño.
- **Worker**: instancia el wasm al cargar la página (validar importaciones en el menú previo), abre los archivos de `game/` al pulsar «Jugar».
  Importar/borrar partidas solo con el juego parado (el Worker bloqueado no atiende mensajes); exportar vale siempre.
- **Fotogramas**: el Worker copia el RGBA y lo transfiere; la página lo pinta con `putImageData`. `OffscreenCanvas.transferToImageBitmap`
  acumulaba ~80 MB/s en el renderer (gigas en minutos) con un Worker que no cede.
- **Diagnóstico**: `Runtime.diag` envía a la consola de la página las funciones ausentes, importaciones/SGL sin sustituto y abortos.
- **Excepciones WASM obligatorias**: `ui_rt` desenrolla (`AbortRun`); `build.sh` compila con `panic=unwind` + `-Zbuild-std`.

Pruebas: `node test/smoke.cjs` (sin juego), `node test/drive.cjs CARPETA [puerto]` (pilota el juego por HTTP: `/click?x=&y=` en píxeles del
juego, `/key`, `/shot`, `/status`, `/eval`; ojo: el tamaño del juego cambia entre menús 800×600 y ciudad 1024×768 u otro).
Límites: sin sonido (WEB-5), probado solo en Chromium sin cabeza, la caché de archivos del CPR no se desaloja (1,2 GB tras 30 min),
Safari/Firefox sin probar, la importación de `.pat` con el juego en marcha exige recargar.

## Empaquetado verificado (WEB-4, 14/11/2026)
`python web/build_web.py --wasm p3-web/www/p3_web.wasm --site p3-web/www` → `web/dist/` con `p3.<hash>.wasm` de 22,3 MB (4,9 MB gzip)
tras `wasm-opt -Os` y la página de p3-web (Worker, partidas, audio). Probado con `P3_DIST=1 node test/drive.cjs CARPETA`: la ciudad de Luebeck
corre desde `dist/` (aislamiento de origen cruzado, `p3-load.js`, wasm optimizado con excepciones). El service worker se bloquea en esa
prueba (Playwright no intercepta lo que pasa por él); su modo sin conexión es el de WEB-6.
Nota: el `wasm-opt` de npm (binaryen JS) termina el trabajo a los ~6 min pero el proceso no sale (se queda al 100 % de CPU): si se cuelga,
`kill` y usar `web/.work/p3.opt.wasm` con `--opt none`. Un binaryen nativo (`apt`/descarga) no tiene el problema.

## Rendimiento y memoria (WEB-6)
Medidas y cambios en [`../WEB_RENDIMIENTO.md`](../WEB_RENDIMIENTO.md). Lo que toca a este crate: la página recibe directamente la
superficie del enlazador (sin los dos búferes del dispositivo), `build.sh` compila con `+simd128`, las texturas descodificadas
tienen un tope de 48 MiB en `wasm32` y el Worker enciende el modo de poca memoria en móviles
(`p3_set_low_memory`; `opts.lowMemory: true/false` lo fuerza desde la página).
Sobre el límite anotado arriba por WEB-4 (la caché del CPR no se desaloja): las texturas ya no pasan por esa caché
(`Archive::read_transient`); en el banco se queda en 0,5 MiB al entrar en una ciudad, frente a 109 MiB. Falta repetir en el
navegador la medida larga (30 min).

## Idioma (14/11/2026)

Si la carpeta del juego trae `p2arch0_spa.cpr` (junto al inglés) e `idioma/spa.pack`, la página los copia y en Opciones aparece
`Language: < >` como en p3-juego. Textos del CPR y menús del EXE salen en español; el CPR combinado se monta sin copiarlo (ver
`reimpl/IDIOMA.md`). Prueba: `P3_FROM_SERVER=1 node test/drive.cjs CARPETA` y `/restart` tras cambiar el idioma.
