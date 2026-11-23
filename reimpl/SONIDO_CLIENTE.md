# Sonido en el cliente nativo (Miles sustituida)

14/11/2026, nube de Claude, hilo «Hacer que suene el juego». Antes, en `p3-juego` no sonaba nada: el enlazador `ui_rt`
dejaba el gestor de sonido del original (`[0x006CBDB8]`, J04) sin dispositivo con sustitutos que devolvían 0, once de sus
funciones no estaban enlazadas y las 44 importaciones `AIL_*` de `mss32.dll` caían en «importación sin sustituto».

## Qué se ha hecho

| Pieza | Archivo | Qué hace |
|---|---|---|
| Mezclador en tiempo real | `p3-audio/src/live.rs` | Voces Miles (muestras y streams): volumen y panorámica 0..127, estados 2/4/8, bucles (0 = infinito), posiciones en ms, paso bajo de un polo, remuestreo lineal, mezcla en coma flotante. WAV PCM8/16 e IMA ADPCM de Miles con `native::stream::decode_stream`; MP3 (Symphonia) en un hilo que publica por tramos (la música suena desde el primer tramo). Caché de streams por nombre con tope de 128 MiB. |
| `mss32.dll` sustituida | `p3-mvp/src/ui_rt/miles.rs` | Las 44 `AIL_*` del EXE sobre el mezclador. Los manejadores son bloques del montón emulado; `HSTREAM+0xC` apunta a su muestra (lo lee `set_lowpass_filter`, `0x00443742`). `AIL_enumerate_filters` ofrece «LowPass Filter» y su atributo «Lowpass Cutoff» (400 Hz del juego). La música (`Musik\...\*.mp3`) se busca sin distinguir mayúsculas en la carpeta del juego, la del CPR y la del EXE; si no está suelta, en el CPR. |
| Gestor de sonido enlazado | `p3-mvp/src/ui_rt/grp_snd_lifted.rs` | 11 funciones elevadas con `tools/ui_rt_grp.py snd` (semillas `tools/ui_rt_grp_snd.seeds`; argumentos de las llamadas a Miles por registro en `tools/ui_rt_grp_snd.nover`, sacados de la decoración `@N`): ctor `0x4419A0`, configuración `0x441CD0`, apertura del driver `0x441E10`, `play_music` `0x4426C0`, volúmenes `0x442D90/0x442E30/0x442E90`, `start_service_thread` `0x4429D0`, paso bajo `0x4436E0`, listas `0x4439B0` y `play_sound` `0x443150`. |
| Salida | `p3-client/src/audio.rs` | `cpal` 0.15: WASAPI en Windows; en Linux solo con `--features sonido` (ALSA, `libasound2-dev`). `p3-juego menu/original` abre la salida predeterminada a su frecuencia y conecta el mezclador; `--sin-sonido` o `P3_SIN_SONIDO` lo desactivan. |

Sin mezclador (`rt.miles.mixer == None`: pruebas, bancos, `ui_window`...) los sustitutos de antes siguen activos y no se
llama a ninguna `AIL_*`: las 115 pruebas de `p3-mvp` dan lo mismo que antes.

## Comprobado (nube)

- `cargo test -p p3-audio --lib live` (3), `cargo test -p p3-mvp --lib miles` (1), `cargo test -p p3-mvp --test ui_sound` (2):
  con mezclador el original llama a `AIL_startup`, `AIL_waveOutOpen`, reserva 24 canales, abre el paso bajo, y el menú abre
  `hauptmenue.mp3` en bucle con señal en más de la mitad de los fotogramas; sin mezclador no hay llamadas.
- `cargo run --example ui_sound city 1000`: menú → partida nueva → ciudad. Suenan la música del menú (desde el 2.º fotograma),
  los clics de los botones (`set_sample_file`) y efectos de la ciudad (`set_named_sample_file` con panorámica); ninguna función
  del gestor queda sin enlazar y ninguna `AIL_*` sin sustituto.
- `p3-juego menu` con `--features sonido` en Xvfb y un ALSA que escribe a archivo: «sonido: default a 44100 Hz, 2 canales (F32)»
  y la música del menú sale por el dispositivo (RMS 0,03–0,06).
- 115 binarios de pruebas de `p3-mvp` correctos (sin sonido) y clippy limpio en lo nuevo.

## Límites conocidos

- Contrato propio, no paridad con Miles: no se reproducen los redondeos, la latencia ni la mezcla Q11 de la DLL (eso está
  contrastado aparte en los bloques 13–35 de `p3-audio/README.md`, que siguen sin conectarse al cliente).
- `start_service_thread` (`0x4429D0`) llama a `AfxBeginThread` con el trabajador `0x00442090`, que no está en el catálogo ni
  elevado: el enlazador es monohilo y ese hilo (el fundido de salida de la música al cerrar) no arranca.
- La música del menú sigue sonando al entrar en la ciudad mientras esté activa: es lo que decide el código original
  (`game_screen_update_music_and_ambience` `0x42C080` conserva la música activa en el modo 1); la música de la ciudad entra
  cuando el selector la cambia. Falta confirmarlo contra el juego original con sonido.
- `AIL_file_read` de `sound/sound02..14.wav` (lista de efectos `0x4C4130`) falla igual que en la instalación inglesa: esos
  archivos no existen.
- Sin probar en Windows con altavoces: pendiente en `PRUEBAS.md`.

```bash
cd reimpl
export P3_EXE=.../Patrician3.exe P3_CPR=.../p2arch0_eng.cpr        # Musik junto al EXE o P3_TEST_GAME_DIR
cargo test --release -p p3-mvp --test ui_sound
cargo run --release -p p3-mvp --example ui_sound -- city 1000 salida.wav   # WAV de lo que sonaría; P3_AIL_TRACE=1 traza cada AIL_*
cargo build --release -p p3-client                                          # Windows: con sonido; Linux: --features sonido
```

## Sonido en el navegador (WEB-5, 14/11)

Backend web de `p3-audio` sin tocar el nativo (`p3-client/audio.rs` sigue con `cpal`). Diseño en `coordination/ARQUITECTURA_WEB.md`, «Sonido», paso 1.

- `p3-audio/src/ring.rs`: `AudioRing`, anillo estéreo f32 de un productor y un consumidor en un único bloque de palabras atómicas (cabecera de 8 palabras + datos; disposición documentada en el módulo y repetida en `web/p3-audio-ring.js`). Vive en la memoria lineal compartida del Worker; JS solo necesita su dirección (`ring_ptr`).
- `p3-audio/src/web.rs`: `WebAudio` (mezclador + anillo + bombeo). `pump()` descodifica unos paquetes MP3 y rellena el anillo hasta `latency_ms` (150 por defecto); con el consumidor parado no mezcla de más. `install_service()` engancha `Mixer::service`, que `ui_rt/miles.rs` llama en cada orden `AIL_*`: así el anillo sigue alimentado en cargas largas aunque no se presente fotograma.
- `p3-audio/src/live.rs`: `Mixer::set_cooperative` (por defecto activo en `wasm32`, donde no hay `thread::spawn`): el MP3 se descodifica a pasos (`Mp3Job`, `Mixer::pump_decode`) en vez de en un hilo. El hilo nativo usa el mismo `Mp3Job` con presupuesto ilimitado; prueba `threaded_and_cooperative_decoders_give_the_same_samples`.
- `p3-audio/web/`: `p3-audio-worklet.js` (AudioWorkletProcessor «p3-audio»), `p3-audio-web.js` (`createAudioContext`, `attachRing`, `keepAudioAlive`), `p3-audio-ring.js` + `test-ring.mjs` (`node web/test-ring.mjs`).

Integración que debe hacer `p3-web` (WEB-3): gesto del usuario → `createAudioContext()` → enviar `ctx.sampleRate` al Worker → `WebAudio::new(rate, 150)`, `rt.miles.mixer = Some(web.mixer())`, `web.install_service()`, devolver `web.ring_ptr()` → `attachRing(ctx, memory, ptr, url)`; en cada `on_present`, `web.pump()`. Requiere memoria WASM compartida (COOP/COEP).

Comprobado: `cargo test -p p3-audio` (tests nuevos en `web.rs`, `ring.rs`, `tests/web_cooperative_mp3.rs`, fixture sintético `sine_2s_synth.mp3`), `cargo check -p p3-audio --lib --target wasm32-unknown-unknown`, `cargo check -p p3-mvp`, `node web/test-ring.mjs`. No comprobado: oírlo en un navegador (necesita `p3-web`); si hay cortes en cargas largas sin órdenes AIL, paso 2 del diseño (mezclador dentro del worklet). Los tests `*_match_original_*` de p3-audio fallan en la nube por falta de EXE/DLL, también sin estos cambios.

### Integrado en p3-web (14/11)

`reimpl/p3-web` ya usa `WebAudio` (ver su README, «Sonido»). Cambio respecto al diseño: la memoria de WASM de `p3-web` no es compartida, así que el `AudioRing` de WASM se copia a un `SharedArrayBuffer` gemelo que vacía el worklet (`www/worker.js::audioSync`, antes y después de cada `pump`).

Medido en Chromium sin cabeza (nube, 44,1 kHz, colchón 150 ms) con el EXE y el CPR ingleses reales y seis MP3 de `Musik/` (menú, ambiente de ciudad, regiones): la música llega al worklet y el anillo se mantiene en 5.200–6.600 fotogramas (≈150 ms).
- Menú, 100 s: **0 vaciados** desde que suena (RMS 0,02–0,08).
- Ciudad de Lübeck tras «Single Game → Start», 60 s: **0 vaciados** (RMS 0,04–0,14, música más ambiente).
- Los vaciados del arranque y de la carga de la partida (~125 al llegar al menú, ~80 más al cargar la ciudad, unos 0,25 s de silencio en total cada vez) son silencio mientras el juego no presenta fotogramas; no hay cortes durante el juego.
`node reimpl/p3-web/test/audio.cjs CARPETA 60` (con `P3_H=1500 P3_CLICKS="400,492,8000;850,845,30000"` para entrar en la ciudad).
No comprobado: oído humano (la nube no tiene salida), efectos de edificios, carta náutica, ni la carga de CPU en un PC o móvil real.
