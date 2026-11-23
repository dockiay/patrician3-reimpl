# Resoluciones modernas (Opciones y CLI de `p3-juego`)

## Elegir desde el juego (13/11/2026, PC de Sergio)

Abrir el juego normalmente, entrar en **Options → Game resolution**, cambiar con
las flechas y pulsar **OK**. El selector ofrece los tres modos originales y
1366×768, 1600×900, 1920×1080, 2560×1440, 3440×1440, 3840×2160 y **Auto**.
Auto conserva la elección del monitor y la escala entera existente (4K →
1920×1080 ampliado al doble). La resolución elegida se usa en la partida; los
menús conservan su lienzo de 800×600.

El cliente guarda el modo en `video-mode.txt` dentro de la carpeta indicada por
`--juego` (en el lanzador instalado: `D:\AI\P3-Prueba`). Cancel conserva la
elección anterior y no sobrescribe el archivo. Sin archivo se parte del modo
original guardado. Una opción `--moderna` explícita prevalece al arrancar y se
puede cambiar después en Options. Los modos originales siguen disponibles.

Implementación: `p3-mvp/src/ui_rt/video_options.rs`. El selector comercial solo
reserva tres índices y seis combinaciones de modo/profundidad: las opciones
modernas usan un índice físico válido y una elección lógica separada. La
etiqueta aprovecha el dibujo y centrado originales, sin modificar las fuentes.
La configuración comercial conserva índices 0..2. El motor moderno de
`display.rs` y el código elevado de colocación no se cambian.

Pruebas con EXE/CPR locales: `ui_video_options` recorre flechas, OK, Cancel,
reinicio, Auto, prioridad CLI y **Options → Single Game → Start → ciudad a
1920×1080**; `ui_resolution` conserva las tres pruebas de colocación originales
y modernas. Esto no acredita todas las ventanas ni elimina los límites gráficos
ya documentados más abajo.

Hilo «Añadir resoluciones modernas al juego», 12/11/2026, nube de Claude. Código: `p3-mvp/src/ui_rt/display.rs`,
`p3-mvp/src/ui_rt/display_lifted.rs` (generado por `tools/ui_rt_display.py`), `p3-client` (`--moderna`). Pruebas:
`p3-mvp/tests/ui_resolution.rs` y las unitarias de `display.rs`.

## Qué hace

`p3-juego ... --moderna [ANCHOxALTO]` pone la partida a la resolución pedida (1920×1080, 2560×1440, 3440×1440, 3840×2160...) de
modo que **se ve más ciudad y más mapa**, no la misma imagen ampliada. Sin tamaño usa el del monitor; si el monitor pasa de 1600
de alto se divide por un entero (4K → 1920×1080 mostrado al doble, con píxeles enteros) para que la interfaz no quede diminuta.
Sin la opción CLI ni un modo moderno guardado se usa el modo original.

| Resolución | Escena de ciudad | Frente a 1024×768 (740×726) |
|---|---|---|
| 1024×768 (original) | 740×726 | 1,0 |
| 1280×1024 (original) | 996×982 | 1,8 |
| 1920×1080 | 1636×1038 | 3,2 |
| 2560×1440 | 2276×1398 | 5,9 |
| 3440×1440 | 3156×1398 | 8,2 |
| 3840×2160 | 3556×2118 | 14,0 |

## Cómo lo hace el original y dónde se engancha

El juego ya es casi independiente de la resolución. Todo cambio de modo pasa por `set_ddraw_resolution(hwnd, config*)`
(`0x004BCCC0`): los menús piden 800×600 y la partida 800×600, 1024×768 o 1280×1024 según las opciones (`game_main_window_run`
`0x00464027`, `game_screen_update_hud_and_hotkeys` `0x00423C25`, `options_window_update` `0x0045FE9E`). Después
`game_screen_apply_resolution_layout` (`0x00429A40`) coloca la pantalla de juego: la barra lateral de 284 px a la derecha, la barra
superior de 42 px, y la escena de ciudad, la carta, el minimapa y los botones a partir de esas dos. Casi todo sale de
`0x006DCCE0/E4` y de la posición de la barra vertical; lo único resuelto con tres casos fijos (ancho 800, 1024 o 1280) es:

- qué pieza superior izquierda se inserta (`ObenLinks800/1024/1280Anim`: 516, 740 o 996 px) y qué relleno inferior de la barra
  lateral (`RechtsUnten1024/1280Anim`: 284×168 o 284×424, desde y = 600);
- el mapa a pantalla completa (`accelMap.ini`, `[SCREEN0..2]` de 800×600, 1024×768 y 1280×1024);
- el reparto de los cuatro elementos de la barra superior (`0x00429FE0`), que con otro ancho lee una variable sin iniciar
  (`0x0042A048`).

Con la opción activada (`Runtime::display.modern = Some((ancho, alto))`):

1. Los menús no cambian: siguen a 800×600 y la ventana los amplía sin deformar, como hacía el monitor.
2. Al entrar en partida, `0x00429A40` se envuelve. La resolución del juego (`0x006DCCE0/E4`, `0x006DAB4C/44`) y la superficie pasan
   a ser las pedidas, y la pieza superior izquierda del ancho estándar de base (el mayor que cabe: 1280, 1024 u 800) pasa a medir
   `ancho − 284` (se responde eso a su `get_width`). El original se ejecuta dos veces: una con el ancho de base (inserta sus
   piezas, prepara el mapa a pantalla completa y coloca las tres barras a partir de la pieza) y otra con el ancho real (rama por
   defecto, que recoloca todo lo demás y da a la escena de ciudad `(ancho − 284) × (alto − 42)`). Las dos piezas de base se
   reinsertan en el mismo orden en que las deja el original.
3. `GameScreen::set_size` recibe la resolución pedida en lugar de la de las opciones.
4. Al dibujarse, la pieza superior izquierda repite un tramo de tablones (de junta a junta, múltiplo del adorno de 39 px) entre su
   cuerpo y su remate derecho, y el relleno inferior se apila hasta el borde de la pantalla.
5. Al volver a los menús, `set_ddraw_resolution` deja otra vez 800×600.

Con una resolución del original (800×600, 1024×768, 1280×1024) la opción no hace nada distinto: la prueba compara la memoria de
`GameScreen` y de la escena byte a byte con la del modo original.

## Qué cambia respecto al original

- **Con la opción desactivada, nada del juego.** Dos cambios del enlazador valen para los dos modos:
  - `set_ddraw_resolution` cambia además el tamaño de la superficie al modo pedido (antes el menú de 800×600 se dibujaba en la
    esquina de una superficie de 1024×768). Es lo que hacía DirectDraw; la ventana amplía la imagen.
  - `0x00429A40` y `0x00429FE0` se ejecutan desde `display_lifted.rs`: el mismo x86 elevado de nuevo con la retirada real de cada
    llamada virtual. La versión de `p3_core::lifted` contaba mal los argumentos en la rama de 1280×1024 (sus trazas no la recorrían)
    y desplazaba la pila: la barra vertical quedaba en y = 0. Las filas de `functions.tsv` y sus pruebas de paridad no cambian.
- **Con la opción activada** (decisiones nuevas, no existen en el original):
  - resolución de partida libre; las opciones de vídeo del juego (800/1024/1280) dejan de decidirla;
  - pieza superior izquierda alargada con tablones repetidos y relleno lateral apilado: son las texturas originales, repetidas;
  - reparto de la barra superior sobre el ancho nuevo con la misma fórmula del original;
  - el mapa a pantalla completa usa el fondo del ancho de base (1280×1024 como mucho): por encima de eso no cubre la pantalla;
  - escala entera automática en monitores de más de 1600 de alto (solo en el cliente, no toca el juego).

## Estado y límites

- Probado en el enlazador con el arranque original y las piezas cargadas de `ScreenGame.ini`: colocación en 800×600, 1024×768,
  1280×1024 (original) y 1280×720, 1366×768, 1600×900, 1152×864, 1920×1080, 2560×1440, 3440×1440 y 3840×2160 (moderna).
- **Visto en partida (13/11/2026)** con el primer fotograma de ciudad del enlazador (carga de `AUTO.pat` con el código original):
  a 1920×1080 la barra lateral queda pegada al borde derecho, la barra superior cubre todo el ancho con sus tablones repetidos,
  el relleno lateral llega al borde inferior y la escena de ciudad ocupa 1636×1038. Reproducir desde `reimpl/`:
  `P3_MODERN=1920x1080 P3_CALLS=60000000 P3_GAME_DIR=<carpeta con save/Ein/AUTO.pat> P3_SCRIPT="3:6cc6b8=4;8:*6cbb7c+11c4=0;20:stop"
  target/release/examples/ui_game Patrician3.exe p2arch0_eng.cpr salida.png boot run`. La pieza alargada toma su textura y su
  posición del último volcado del dibujo original (`Surface::last_blit`), porque el original restaura la textura activa al salir.
  El suelo, la clave de color de los sprites y el minimapa que faltan en esa captura son del enlazador (hilo del menú), iguales a
  1024×768. Pendiente: ventanas de edificio (se centran en la escena), mapa a pantalla completa y rendimiento a 4K.
- **Ratón sobre la interfaz (13/11/2026, hilo «Arreglar la interfaz en resoluciones altas»)**: al pasar el ratón por la barra
  lateral o la superior el juego repinta solo zonas. La pieza alargada tomaba su posición del volcado recortado de esa zona
  (no del origen de la pieza) y repetía la textura sin recortar: tablones sobre los textos y el dinero, y trozos de la barra con
  la pluma en el relleno lateral (captura de Sergio a 2560×1440). Ahora el origen es destino − origen en la textura, se guarda
  por pieza para las zonas que solo tocan la parte alargada, y los tramos se dibujan con el recortador activo. Prueba:
  `tests/ui_chrome_hover.rs` (falla sin el arreglo).
- La vista de prueba (`p3-juego vista`) ya usa la resolución moderna: ciudad a pantalla entera sin ampliar y carta náutica centrada
  (la carta entera con su marco mide unos 2400×1430; a partir de ahí sobra pantalla).

## Uso

```text
p3-juego vista AUTO.pat p2arch0_eng.cpr --moderna               ciudad y carta a la resolución del monitor
p3-juego vista AUTO.pat p2arch0_eng.cpr --moderna 2560x1440     a una resolución concreta
p3-juego menu Patrician3.exe p2arch0_eng.cpr --moderna          arranque original; la partida usará la resolución moderna
```

En el PC de Sergio: `D:\AI\P3-Prueba\Pantalla-P3-Moderna.cmd` (sin argumento, vista; `menu`, arranque original; un tamaño como
`2560x1440` lo fija; `ventana` abre en ventana).

Regenerar `display_lifted.rs`: `python tools/ui_rt_display.py --binary Patrician3.exe` (desde `reimpl/`).

## Mosaico al usar la rueda por encima del 100 % (14/11/2026)

Vídeo de Sergio a 2560x1440: al acercar o alejar con la rueda, cada paso dejaba un fotograma con trozos viejos sin ampliar
(con zoom 100 % no pasaba). Causa: con los dos búferes de pantalla (d436bfb) la imagen ampliada de `zoom.rs` quedaba solo en
el búfer vigente; el desplazamiento de cámara de cada paso de rueda dibuja en el delantero y `present_frame` copia las zonas
sucias del trasero, que aún tenía la escena sin ampliar. Ahora `zoom_mirror_scene` copia el rectángulo de la escena al otro
búfer. Prueba: `cargo test --release -p p3-mvp --test ui_zoom_buffers` (P3_EXE, P3_CPR): todos los fotogramas iguales con uno y
dos búferes; sin el arreglo fallaban 20 de 42. Banco: `ui_benchmark zoom N 2560x1440` con `P3_BENCH_WHEELSEQ="F:delta:x:y;..."`.

## Ventana de comercio abajo con el zoom alejado (15/11/2026)

Captura de Sergio: alejada la vista, la grúa abría «Mercancías comerciales» abajo a la derecha, cortada. Durante el clic de la
escena (`0x587AA0`) el tamaño de la escena es el del lienzo virtual (hace falta para elegir objetos lejos), y la acción
(`class22_on_click_map_object` 0x594DD0 → 0x59AE40 → `trade_parchment_open` 0x4A9690) centra la ventana con la ranura +0x40 de
la escena (`0x402470`). Ahora, mientras dura el clic, esa ranura devuelve el tamaño físico a quien llama desde fuera del código
de la escena (0x585000..0x5A0000). Con el zoom al 100 % la ventana también quedaba media baldosa desplazada (940,472 → 924,461);
`ui_campaign_ship_trade` ajusta sus clics. Prueba: `cargo test --release -p p3-mvp --test ui_zoom_trade_window`.
