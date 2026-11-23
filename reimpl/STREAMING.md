# Jugar en streaming desde el servidor (p3-stream)

Decisión de Sergio (14/11/2026): **el juego corre en su PC y el navegador solo muestra**. Nadie importa el juego ni descarga EXE, CPR
o `.wasm`; quien abre la URL (iPhone Safari incluido) recibe vídeo y sonido y envía ratón, teclado y toques. Es la alternativa al
camino WASM de `coordination/ARQUITECTURA_WEB.md` (que sigue ahí, pero ya no es lo que se sirve).

## Qué es
`reimpl/p3-stream` = el cliente nativo (`p3-client`/`p3-juego menu`) **sin ventana**:
- El juego arranca igual que `p3-juego menu` (`ui_rt::menus::{boot, run}`); en cada `sgl_SwapBuffer` la imagen pasa por la capa SGL
  portada, se compara con la última enviada y, si cambió (máx. 30 por segundo), se codifica **H.264** (OpenH264, modo contenido de
  pantalla, 3 Mbps de techo) en un hilo aparte. Imagen quieta = cero bytes.
- El sonido (`p3_audio::live::Mixer`, el mismo del cliente nativo) se mezcla a ritmo real en un hilo y sale como **PCM i16 estéreo a
  22.050 Hz** en paquetes de 20 ms (≈0,7 Mbps; `--audio-hz` lo cambia).
- Servidor HTTP + WebSocket sin runtime asíncrono (`tungstenite`). Un solo juego; todos los espectadores ven lo mismo y cualquiera
  manda entrada. Si el navegador se cierra, **la partida sigue** y se vuelve a entrar donde se dejó.
- La página (`www/index.html`, una sola, embebida en el EXE) decodifica con **WebCodecs** (`VideoDecoder`, Safari 16.4+ / Chrome),
  dibuja en un `<canvas>`, reproduce el PCM con Web Audio y traduce ratón/toques/teclado a mensajes de texto (`m fx fy`, `b botón 0|1`,
  `w pasos`, `k vk 0|1`, `f 0|1`). Botones: clic derecho (para el próximo toque), Esc, teclado en pantalla (iPhone), pantalla
  completa y datos (resolución, fps, kbps, ping; también `#stats` en la URL).

## Seguridad (la contraseña es obligatoria)
- Sin `P3_PASSWORD` (≥6 caracteres) **el servidor no arranca**. Escucha solo en `127.0.0.1`; delante va el túnel Cloudflare.
- La página exige Basic auth (usuario cualquiera); el WebSocket exige un token aleatorio por ejecución que solo entrega la página ya
  autenticada. Comparación en tiempo constante y 0,5 s de espera en cada fallo. No se sirve ningún archivo del juego.
- Pensado para uso propio, no para repartir el juego.

## Uso
```
p3-stream --exe Patrician3.exe --cpr p2arch0_eng.cpr [--juego CARPETA_PARTIDAS] [--puerto 8090]
          [--moderna 1920x1080] [--fps 60] [--fps-video 30] [--kbps 3000] [--audio-hz 22050] [--sin-sonido]
```
`P3_PASSWORD=... ` en el entorno. `--juego` es donde se escribe `save\ein\*.pat` (por defecto, la carpeta del CPR). La música
(`Musik\`) se busca junto al CPR. En Windows: `reimpl/p3-stream/Servir-Stream.cmd` (clave fija en `clave.txt`, túnel `quick` de
Cloudflare, URL en `url.txt`). Windows: `cargo build --release -p p3-stream --target x86_64-pc-windows-gnu` en la nube
(`gcc-mingw-w64-x86-64 g++-mingw-w64-x86-64`, con la variante `posix`: `update-alternatives --set x86_64-w64-mingw32-g++ …-posix`).

Idioma: como `p3-juego` (Opciones → Language, `partidas\idioma.txt`, aplica al reiniciar el servidor); `--idioma spa` lo fuerza y
`--idiomas CARPETA` dice dónde están `p2arch0_spa.cpr` e `idioma\spa.pack` (Servir-Stream usa `D:\AI\P3-Prueba`). Ver `IDIOMA.md`.

## Medidas (nube, 4 vCPU, 14/11/2026; el PC de Sergio debería ir mejor)
| | 1024×768 | 1920×1080 (`--moderna`) |
|---|---|---|
| Juego | 60 fps (42 al cargar) | 30 fps (el pintado a 1080p en la nube) |
| Vídeo enviado | 17–22 fps, **1,5–2,0 Mbps** con la ciudad animada; menú quieto 0 | 17 fps, **1,8 Mbps** |
| Codificar | 10–15 ms/fotograma | 17–19 ms/fotograma |
| Sonido | 0,71 Mbps (constante) | igual |
| Latencia ratón→imagen (local, imagen quieta) | **mediana 33 ms** (26–44) | — |
Por Cloudflare se suma el RTT del móvil al servidor (≈2×). Total con sonido: ≈2,5 Mbps; en 4G sobra.

## Pruebas (los archivos del juego no están en Git)
- `cargo test -p p3-stream` (entrada).
- `p3-stream/test/client.py` (cliente sin navegador: contraseña mala/sin token rechazados, guion de entrada, vuelca H.264/PCM), `shot.sh`
  (captura del último fotograma con `ffmpeg`), `browser.cjs`/`browser2.cjs`/`browser3.cjs` (Chromium: carga, clic, toque) y
  `latencia.cjs` (latencia con la imagen quieta). Todo comprobado: menú → Single Game → Start → ciudad → Esc → Opciones → Cancel,
  con ratón y con toques, en 800×600, 1024×768 y 1920×1080.

## Límites conocidos
- Solo H.264 (WebCodecs): iOS ≥ 16.4. No hay JPEG de reserva.
- iPhone: el sonido de Web Audio calla con el interruptor de silencio; hay que tocar «Jugar» (gesto del usuario).
- Un único juego compartido; no hay cuentas ni partidas por espectador. Si hay varios, mandan todos.
- Sin pruebas todavía con el EXE de Windows de este crate (compilado cruzado en la nube; ver el hilo).
- El sonido es PCM sin comprimir (Opus bajaría a ~0,1 Mbps).
