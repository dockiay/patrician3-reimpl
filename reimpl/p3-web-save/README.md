# p3-web-save (WEB-4): partidas en el navegador

Guardar y cargar `.pat`/`.pst` (y `AUTO.pat`) en OPFS, sobre el rasgo `HostFs` de WEB-2.

- `src/savefs.rs`: `SaveFs`, un `HostFs` que guarda toda la carpeta `save/` **en memoria** (síncrono, sin distinguir mayúsculas) y
  entrega cada cambio terminado (cerrar archivo, escribir, renombrar, borrar) a un `CommitSink`. Lo que no cuelga de `save/` pasa al
  `HostFs` instalado antes (EXE, CPR, música...).
- `src/wasm.rs`: puente `extern "C"` (`p3_save_install`, `p3_save_preload`, `p3_save_forget`, `p3_save_inspect`; importa
  `p3_save_commit` y `p3_save_now_secs` del módulo `env`).
- `js/p3-saves-main.js` (`SaveStore`, hilo principal): escribe en OPFS (`createWritable`, atómico), lista, importa (valida con
  `p3-save` a través del Worker), exporta/descarga, borra y pide `navigator.storage.persist()`.
- `js/p3-saves-worker.js` (`WorkerSaves`, Worker del juego): imports del `.wasm`, precarga y atiende importaciones.

Por qué en memoria y mensajes: el Worker del juego pasa el tiempo bloqueado dentro de wasm, y abrir archivos nuevos en OPFS es asíncrono.
Los mensajes (`p3-save-commit`, `p3-save-init`, `p3-save-import`, `p3-save-import-result`, `p3-save-forget`) se envían con
`postMessage`, que no necesita ceder el control. Los entrantes (importar, olvidar) se atienden cuando el Worker vuelve al bucle de
eventos (cada fotograma).

Integración (WEB-3): en el Worker, `new WorkerSaves(self)` antes de instanciar; `bind(exports)`; `await install('/juego')` justo antes de
arrancar el enlazador y **después** de instalar el sistema de lectura de EXE/CPR; en la página, `store.attach(worker)`,
`await store.persist()` y `await store.sendInitialSaves()`. Borrar los datos del sitio borra las partidas: la página debe ofrecer **Exportar**.

Pruebas:
```
cargo test -p p3-web-save                                  # 5 pruebas nativas (incluye AUTO.pat real)
cargo build -p p3-web-save --target wasm32-unknown-unknown --release
node reimpl/p3-web-save/js/test/run.cjs RUTA/AUTO.pat reimpl/target/wasm32-unknown-unknown/release/p3_web_save.wasm   # Chromium + OPFS real, 19 comprobaciones
```
Límites: validado con Chromium en la nube, no con el juego en el navegador (aún no hay menú web) ni en Safari/Firefox
(`createWritable` en hilo principal necesita Chrome/Edge/Firefox recientes; Safari antiguo no lo tiene).
