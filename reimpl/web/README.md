# Juego web: carga, caché, PWA y sin conexión (WEB-6)

Uso privado: el `.wasm` contiene código traducido del EXE. **No publicar** en ninguna web
(ver `coordination/ARQUITECTURA_WEB.md`, riesgo legal). El servidor solo escucha en `127.0.0.1`.

## Uso

```
python web/build_web.py [--wasm RUTA] [--site reimpl/p3-web/www] [--opt -O3|-Os|-Oz]
python web/serve.py            # http://localhost:8765/
```
En Windows: `web\Servir-Web.cmd`. Después, en Chrome/Edge, «Instalar Patrician III» (icono de la
barra de direcciones). Tras la primera visita el `.wasm` y la página quedan en la caché del
service worker y todo funciona sin red ni servidor (los archivos del juego, en OPFS, aparte).

## Qué hace

| Pieza | Fichero | Detalle |
|---|---|---|
| Compilación | `build_web.py` | `cargo build --profile wasm-release -p p3-wasm`, luego `wasm-opt -Os --all-features --strip-*`. Busca `wasm-opt` en PATH, `web/.tools` (npm `binaryen@123`) o lo descarga de GitHub |
| Tamaño | `Cargo.toml` (`[profile.wasm-release]`) | `opt-level="s"`, LTO fat, `panic="abort"`, 1 unidad de codegen; `p3-render` y `p3-audio` (calientes) a `opt-level=3`. Cargo solo permite `opt-level` por crate, no por módulo: el código elevado vive dentro de `p3-mvp`, así que va todo en `s`. Si hiciera falta más fino, separar los módulos fríos en un crate propio |
| Compresión | `build_web.py` | `p3.<hash>.wasm` + `.gz` + `.br` (instalar `pip install brotli` para `.br`) |
| Servidor | `serve.py` | COOP/COEP/CORP (SharedArrayBuffer), `application/wasm`, `.br/.gz` precomprimidos, inmutable para el `.wasm` con hash |
| Caché y sin conexión | `shell/sw.js` | Precarga todo en `install` (con progreso `p3-progress`), caché nueva por versión y borra la vieja, repone COOP/COEP al servir desde caché |
| Carga en streaming | `shell/p3-load.js` | `loadP3Module(onProgress)` → `WebAssembly.compileStreaming` desde la caché; Chrome guarda el código compilado y no recompila al reabrir. Sirve en página y en Worker |
| PWA | `shell/manifest.webmanifest`, `pwa.js`, iconos | Pantalla completa, `storage.persist()`, eventos `p3-pwa-progress/ready/update/installable` |

## Para `p3-web`

`--site` copia su carpeta estática a `dist/`; si su `index.html` no los trae, el build inyecta
`<link rel=manifest>` y `<script type=module src=pwa.js>`. En el Worker:
`import { loadP3Module } from "./p3-load.js"; const m = await loadP3Module(); WebAssembly.instantiate(m, imports)`.
Los ficheros de `p3-web` deben ser rutas relativas y estar bajo la raíz de la carpeta (el SW los precarga todos).
Para actualizar, volver a ejecutar `build_web.py`: otra `version` en `build.json` → el SW nuevo queda
en espera y `p3-pwa-update` ofrece `apply()` (no recarga en mitad de una partida).

## Medido con el `.wasm` real de `p3-wasm` (14/11, nube, Chromium headless)

| Paso | Resultado |
|---|---|
| `cargo build --profile wasm-release -p p3-wasm` | 32,9 MB en bruto; 5 min en frío (LTO fat), 3 min con dependencias ya compiladas |
| `wasm-opt -Os` | 31,4 → **24,4 MB** (−22 %). Tarda ~25 min con el `binaryen` de npm (JS); el binario nativo (`wasm-opt` en PATH o descarga de GitHub) debería ir mucho más rápido |
| Compresión | **5,8 MB** gzip -9, **4,1 MB** brotli q11 (el brotli q11 tarda un par de minutos) |
| Compilación en Chromium (`compileStreaming`) | 240 ms en frío por red local; 65–85 ms desde la caché del SW. V8 valida pronto y compila perezoso, así que el coste real aparece al ejecutar (WEB-3/4 lo medirán) |
| Instanciación | 1 ms (sin imports; memoria inicial 16,5 MB) |
| Sin conexión | Recarga con el servidor parado: página y módulo salen de la caché |

Código compilado en caché: en Chromium la segunda carga no es más lenta que la primera, pero con estos tiempos
(V8 lazy) no se puede separar el efecto de la caché de código; no hace falta nada más por ahora.
Tamaño de la caché del navegador: ~25 MB del `.wasm` + la página (guardado sin comprimir).

## WEB-6, rendimiento (14/11, tarde)

`--opt` pasa a `-O3` por defecto: con el `wasm-opt` nativo (binaryen 123 en `web/.tools`, 6,5 min con 2 núcleos) `-O3` y `-Os`
dejan el mismo tamaño (35,2 → 26,9 MB, 6,6 MB gzip), pero el de `-Os` va un 30 % más lento en V8. Medidas y el resto de mejoras
de velocidad y memoria: [`../WEB_RENDIMIENTO.md`](../WEB_RENDIMIENTO.md). Aviso: el perfil `wasm-release` de `Cargo.toml` usa
`panic = "abort"`, que no sirve para `ui_rt` (hallazgo 1 de `p3-wasm/README.md`); el `.wasm` jugable es el de
`p3-web/build.sh` (`--wasm` para empaquetarlo aquí).
