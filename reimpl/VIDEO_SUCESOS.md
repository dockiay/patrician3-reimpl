# Vídeos de las ventanas de suceso (`Vto.dll` sustituida)

15/11/2026, nube de Claude (hilo «Reproducir los vídeos del juego»). Código: `p3-mvp/src/ui_rt/vto.rs`.

## Qué usa el original

- Las ventanas de suceso (`0x469AF0`, ascensos «Maestro mercader», «Patrono», boda, «Beneke ataca de nuevo», asedio,
  peste...) abren un vídeo en `0x46AA40`: `SKDN_InitVideo(ruta, 0, 2000)`, `SKDN_GetVideoInfos(h, this+0x388)`,
  `SKDN_StartVideo(h, 0, 0)` y una textura de memoria `sgl_CreateMemoryTexture(ancho, alto, 0x1000050)` (`0x4BB2A0`).
  Cada fotograma (`0x46AD32`) llama a `SKDN_RenderVideoFrame(h, tex)`: 0 = fotograma nuevo (invalida el rectángulo),
  100 = terminado (cierra el vídeo y libera la textura; la ventana sigue con el último fotograma en pantalla).
- **No es Bink.** `Vto.dll` es un reproductor propio de Ascaron (exporta 7 `SKDN_*`, importa `ijl11.dll`, `AIM.dll`,
  `WINMM` y 140 funciones `sgl_*` de `ddraw_Dll.dll` por nombre en `SKDN_Init`). Bink solo se usa en la introducción
  (`Intro_1..3.bik`), que el juego reproduce lanzando `binkplay.exe` (ver abajo).
- Archivos: `Videos\*.sks` en la carpeta del juego (32 vídeos, 150 MB; la ruta sale de `scripts/sound.ini`, sección
  `Videos`). Formato «SK10»:

  | Desplazamiento | Contenido |
  |---|---|
  | `+0x00` | `"SK10"` |
  | `+0x04/+0x08` | ancho, alto (400×300 en todos) |
  | `+0x0C` | fotogramas por segundo (15) |
  | `+0x10` | número de fotogramas `n` |
  | `+0x14..+0x1F` | tres palabras sin uso conocido (0) |
  | `+0x20` | `n + 1` pares `(desplazamiento, tamaño)`; el último marca el final |
  | después | cada fotograma es un JPEG/JFIF completo; sin sonido |

- La DLL lee con un hilo y un anillo de `max(3, fps * max(ms_búfer, 250) / 1000)` fotogramas y descodifica con `ijl11.dll`
  (`ijlRead` de parámetros y de imagen entera, `DIBChannels = 4`, `DIBColor = 5` IJL_RGBA_FPX) directamente sobre los
  píxeles de `sgl_LockTexture(tex, info, 2)`, con `DIBPadBytes = paso − ancho * 4`.
- **Colores:** `ijl11.dll` escribe R, G, B, 255 (comprobado ejecutando la DLL original en Unicorn,
  `oracle/vto/ijl_emu.py`), y la textura es B, G, R, A. Los JPEG están guardados con rojo y azul cambiados, así que en
  pantalla salen bien. El sustituto escribe los canales en el orden del JPEG y alfa 255, igual.

## Contrato de `SKDN_RenderVideoFrame` (`0x10001580`)

`fotograma = (timeGetTime − inicio − pausa) / (1000 / fps)` (división entera con signo, negativo → 0). Si
`fotograma ≥ n` y el lector ya marcó el fin (sin bucle siempre ocurre antes), se queda en `n − 1` y marca «terminado».
Si el fotograma no está en el anillo devuelve 1. Si `fotograma − último − 1 > 0` suma saltados. Si no es posterior al
último y no está el bit 2 de los indicadores, devuelve 100 si terminó y 50 si no. Si no, bloquea la textura,
descodifica, desbloquea y devuelve 100 si terminó y 0 si no (1 si el JPEG falla). `SKDN_StartVideo` guarda
`indicadores | 2` (el juego pasa 0: sin bucle), último = −1 e inicio = ahora. `SKDN_GetVideoInfos(0, …)` devuelve 1; con
vídeo copia la cabecera, `+0x20` indicadores, `+0x24` último y `+0x28` saltados.

## Sustituto

- `ui_rt/vto.rs`: los seis `SKDN_*` en Rust; el `.sks` se lee entero (2–10 MB), así que todos los fotogramas están
  siempre «en el anillo» (no hay hilo). El reloj es el de `timeGetTime` del enlazador (el del cliente o `P3_TIME`).
  La textura se bloquea y desbloquea con los mismos `sgl_LockTexture`/`sgl_UnlockTexture` de `mod.rs`.
- Búsqueda: `rt.vto.dirs` (carpeta del juego, del CPR y del EXE en `p3-juego` y `p3-stream`; vacío = las de Miles,
  así en `p3-web` sirve `/juego`). Sin el archivo `SKDN_InitVideo` devuelve 0 y la ventana sigue el camino sin vídeo
  (imagen fija), como `a23eb38`. `P3_SIN_VIDEO=1` lo fuerza; `P3_VTO_TRACE=1` traza cada llamada.
- Descodificador: `jpeg-decoder` (el mismo de las texturas `.jpg` del CPR). Frente a `ijl11.dll` original, 4 fotogramas
  de 4 vídeos: diferencia media 1,0–1,3 niveles por canal, máxima 18–46 en bordes de color (suavizado de croma distinto),
  ~40 % de bytes idénticos. Paridad visual, no bit a bit.
- Web: `p3-web/www/main.js` y `serve.py` copian `Videos/*.sks` junto con `Musik/**`.
- Diagnóstico: `cargo run --release -p p3-mvp --example vto_frame -- Videos/X.sks N salida.bgra`.

## Comprobación (nube)

```
P3_EXE=…/Patrician3.exe P3_CPR=…/p2arch0_eng.cpr P3_GAME_DIR=…/app P3_VTO_TRACE=1 \
P3_BENCH_CALL="40:469380:23a190f0:0;40:469af0:23a190f0:16:0:0:6c7ce0:6c7ce0" \
target/release/examples/ui_benchmark city 300
```

Abre `Videos/historisches_Ereignis.sks` (suceso 0x16): 146 fotogramas nuevos a 15 fps, termina con 100 y
`SKDN_CloseVideo`; la ventana muestra el vídeo dentro del marco con el pergamino debajo y conserva el último fotograma.
Captura: `/mnt/project-files/videos/ventana-suceso-video.png`. Falta verlo en Windows (VID-01 en `PRUEBAS.md`).

## Introducción (`binkplay.exe`)

`game_main_window_initialize` (`0x00463821`) busca `binkplay.exe` e `Intro_1..3.bik` en la carpeta del juego y, salvo
`/nointro`, lanza `binkplay.exe Intro_1.bik Intro_2.bik Intro_3.bik /d408 /b2 /k …` con `CreateProcessA` y espera
(`WaitForSingleObject`). En `p3-juego`, `--intro` (o `P3_INTRO=1`) instala `Runtime::on_create_process`, que lanza ese
mismo `binkplay.exe` de la instalación y espera; sin la opción no hay intro, como hasta ahora. Opcional porque la ventana
del cliente ya está abierta cuando el juego la lanza y no está probado cómo se reparten el primer plano en Windows. No hay
descodificador Bink propio (en la web no hay intro).
