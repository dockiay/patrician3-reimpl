# Pantalla completa con la interfaz original: acuerdo entre hilos (12/11/2026)

Desde el 13/11, el cliente normal permite **zoom en la ciudad con la rueda**:
arriba acerca y abajo aleja (50–250 %, pasos de 25).
Al 75 % y al 50 % se dibuja más ciudad dentro de la misma ventana.
El zoom se centra en el cursor; barras y ventanas conservan su tamaño.
[Implementación, pruebas y límites](../coordination/revision/UI_WHEEL_ZOOM_2026-11-13.md).

Tres hilos trabajan a la vez sobre `p3-mvp/src/ui_rt` para tener la pantalla original del juego en Rust:

| Hilo | Ficheros propios | Qué hace |
|---|---|---|
| Ventana nativa a pantalla completa | `p3-mvp/src/ui_rt/win32.rs`, crate `reimpl/p3-client` (binario `p3-juego`) | Cola de mensajes Win32, estado de entrada, gancho de presentación, WindowProc de la ventana principal |
| Escena global y gestor de ventanas | (lo anota ese hilo) | Escena `0x6E51AC`, gestor de ventanas, elección de página y datos dentro de cada ventana |
| Menú principal y barra lateral | `p3-mvp/src/ui_rt/menus.rs`, pruebas `p3-mvp/tests/ui_menus.rs` | Arranque del flujo original (`game_main_window_run` `0x463D20`), menú principal, carga/guardado, GameScreen y sus menús laterales |

## Abrir el EXE directamente

Sin argumentos, `p3-juego.exe` abre el menú a pantalla completa, usando las mismas
partidas (`save/ein`) y ajustes (`video-mode.txt`) de la carpeta del cliente.
Busca `Patrician3.exe` y `p2arch0_eng.cpr` junto al EXE, o las rutas indicadas en
`p3-juego.ini` (ver `p3-client/p3-juego.ini.example`). Las rutas relativas parten
de la carpeta del EXE, independientemente del directorio desde donde se abra.

El arranque directo crea `save/ein` y copia `partidas/AUTO.pat` solo si todavía
no existe esa partida. Guarda la salida en `p3-juego.log`; los errores de inicio
también aparecen en un cuadro de Windows. No hace falta un intérprete CMD.
Los modos con argumentos (`menu`, `vista`, `original`, `prueba`) siguen disponibles.

## Principio

El bucle se queda dentro del código emulado, como en el juego: `game_main_window_run` → `[vt+0x120]` `screen_window_run_modal_loop`
(`0x4B70C0`) → `screen_list_run_frame` (`0x4B8A40`, PeekMessage/DispatchMessage) → `present_frame` (`0x4B8FF0`) → `sgl_SwapBuffer`.
La ventana nativa entra por el present: presenta la superficie y recoge la entrada. No se invierte el bucle.

## API (la implementa el hilo de la ventana en `win32.rs`)

- `pub struct Msg { hwnd, message, wparam, lparam, time, pt: (i32, i32) }`, constantes `WM_*`/`VK_*` en `win32.rs`.
- `Runtime::post_message(message, wparam, lparam)`.
- `Runtime::input: win32::InputState` (cursor, teclas, botones), que alimenta `GetCursorPos`/`GetAsyncKeyState`/`GetKeyState`.
- `Runtime::on_present: Option<Box<dyn FnMut(&Surface, &mut VecDeque<Msg>, &mut InputState) -> bool>>`, llamado desde
  `sgl_SwapBuffer`; `false` = salir (encola `WM_QUIT`, que `run_frame` convierte en la bandera `0x6DA846`).
- Sin gancho todo se comporta como antes (cola vacía, reloj `P3_TIME`).
- En `mod.rs` ese hilo solo toca los brazos de `import_call` (PeekMessageA, GetMessageA, TranslateMessage, DispatchMessageA,
  GetCursorPos, GetAsyncKeyState, GetKeyState, timeGetTime) y `sgl_SwapBuffer`, más tres campos de `Runtime`.

## Reglas

`git pull --rebase` antes de cada push; cada hilo añade aquí su parte al empujar.

## Ventana nativa (hilo «Abrir el juego a pantalla completa», 12/11)

Hecho y en `main`:

- `p3-mvp/src/ui_rt/win32.rs`: `Msg` (MSG de 28 bytes), constantes `WM_*`/`MK_*`/`VK_*`, `InputState` (`GetKeyState` con bit de
  alternancia, `GetAsyncKeyState` con «pulsada desde la última consulta», variantes izquierda/derecha → genérica), `post`/`peek`,
  `mouse_lparam`, `key_lparam`, códigos de exploración. Pruebas `cargo test -p p3-mvp --lib win32`.
- `Runtime`: campos `messages`, `input`, `on_present` y `post_message`. Brazos de `import_call`: `PeekMessageA` (rango de mensajes
  y `PM_REMOVE`), `GetMessageA` (presenta hasta que hay mensaje; 0 con `WM_QUIT`), `TranslateMessage` (no añade nada; el original no la
  llama y traduce las teclas con `ToAscii`, portado con distribución EE. UU.), `GetKeyboardState`, `ToAscii`, `GetClientRect`,
  `PtInRect`, `SetCapture`/`ReleaseCapture`/`SetCursor`/`ShowCursor`, `GetCursorPos`, `SetCursorPos` (mueve el cursor del juego),
  `GetKeyState`, `GetAsyncKeyState`, `timeGetTime` (reloj de la ventana; sin ventana, `P3_TIME` como antes). `sgl_SwapBuffer` llama
  al gancho; si devuelve `false` encola `WM_QUIT` una vez. Con gancho, `Runtime::calls` vuelve a 0 en cada presentación (el tope de
  llamadas pasa a ser por fotograma).
- `DispatchMessageA` → `Runtime::dispatch_window_message` (`p3-mvp/src/ui_rt/game_window.rs`): busca el mensaje en el mapa MFC
  de GameWindow (`AFX_MSGMAP` `0x671148`, entradas `0x670F98`, misma tabla para GameMainWindow `0x6CC3F0`), desempaqueta los
  argumentos según la firma (`AfxSig`) como `CWnd::OnWndMsg` y llama al manejador. Portados 24 manejadores y reenvíos con paridad
  x86 (`oracle/x86_game_window.py`, 960 casos, `cargo test --release -p p3-mvp --test ui_game_window`): `WM_CLOSE`, teclas y teclas
  de sistema, botones izquierdo/derecho, `WM_MOUSEMOVE` (con el recorte `0x4BD5E0`), `WM_SIZE`, `WM_NCHITTEST`,
  `WM_CAPTURECHANGED`, `WM_ACTIVATE`, `WM_MOUSEWHEEL` y los reenvíos a la pantalla superior `0x4B92E0..0x4B9410`. `WM_SETCURSOR`
  (`0x4C0E80`) y `WM_PAINT` (`0x4C0F20`) usan los puertos que ya había (`p3_core::ui_input`, J08).
- Crate `reimpl/p3-client` (binario `p3-juego`): winit 0.30 + softbuffer 0.4, llevado desde dentro con `pump_app_events`
  (`window::GameWindow::present` = mostrar + esperar el fotograma + recoger eventos). 1024x768 por defecto, pantalla completa sin
  bordes con la imagen escalada sin deformar (Alt+Intro alterna; `--exclusivo` pide el modo de vídeo 1024x768), 60 fps con
  espera fija, cursor del sistema oculto salvo en la vista de prueba. Ratón confinado (13/11/2026, hilo «Arreglar la interfaz en
  resoluciones altas»): a pantalla completa y con el foco, `set_cursor_grab(Confined)` (ClipCursor en Windows) y, si hay bandas
  negras, el cursor se devuelve al borde de la imagen; con dos monitores ya no pasa a la otra pantalla y la cámara se desplaza
  contra el borde. Se suelta al perder el foco (Alt+Tab) y en ventana (Alt+Intro); `--raton-libre` lo desactiva. Ojo: winit en Windows fija un cursor *oculto* al píxel central al
  confinarlo; por eso el cursor del sistema no se oculta, se sustituye por uno transparente de 16×16 (con el oculto el ratón
  quedaba inmóvil a pantalla completa, arreglado el 13/11/2026). `device::NativeScreen` implementa
  `p3_render::sgl_screen::ScreenDevice` (búfer trasero/frontal RGBA) y todo fotograma pasa por `SglScreen::swap_buffer`.
  - `original::attach(rt, ventana)`: instala `on_present` (superficie → `SwapBuffer` → ventana → mensajes nuevos).
  - `original::run_minimal`: pantalla contenedora `0x4B4920` en la lista `0x6DA5F0`, ventanas opcionales y el bucle
    `screen_list_run_frame` `0x4B8A40` hasta que devuelva otra cosa que -1 o aparezca la bandera `0x6DA846`.
  - `view::View`: vista de prueba de una partida (ciudad de `Session::render_fx_rgba`, carta de `Session::navmap_frame_rgba`).
- Uso:
  ```text
  p3-juego vista AUTO.pat p2arch0_eng.cpr [--ventana] [--carta]      ciudad y carta (Tab), ratón en el borde/arrastre/flechas
  p3-juego original Patrician3.exe p2arch0_eng.cpr [AUTO.pat] [--escena] [--abrir TAMAÑO:CTOR[:MÉTODO...]]
  p3-juego menu Patrician3.exe p2arch0_eng.cpr                       arranque original y menú principal en la ventana
  p3-juego prueba                                                     patrón y eco de mensajes, sin datos del juego
  opciones comunes: --fps N --fotogramas N --captura salida.png --guion "10:mover 1020 400;60:tab;90:tecla 27"
  ```
- Linux necesita `libxkbcommon-x11` (en la nube: `apt-get install libxkbcommon-x11-0`); probado con Xvfb y xdotool: ratón, botones,
  rueda, teclas y Esc llegan con el formato de Win32. En el menú principal el cursor propio del juego se dibuja donde está el ratón y
  un clic guionizado abre la pantalla del tutorial.
- Visto en el menú (para el hilo del menú): el resaltado de paso del ratón marca siempre «Tutorial» y un clic dirigido a «Créditos»
  abrió el tutorial; el resaltado sale con el color clave magenta (sin sustituto de `sgl_Blt_RC`/`sgl_LockTexture`).
- Resuelto (13/11, hilo «Arreglar el ratón en el menú principal»): `main_menu_on_mouse_event` (`0x42F9B0`) lee el mapa de zonas
  `images/options/hmenu_colormap.bmp` (BMP de 8 bits, índice = botón 0..7) con `sgl_LockTexture`, que solo conocía las texturas de
  `sgl_CreateMemoryTexture`; con el bloqueo fallido todo caía en «Tutorial». Ahora una textura cargada de archivo se registra al
  bloquearla (`Runtime::register_loaded_texture`, `navmap::bmp_indexed`). `sgl_SetColorKey` guarda la clave y `sgl_LoadMemoryTexture`
  con indicadores `&7 == 7` la aplica (el resaltado `p_hmenu01.bmp` ya no sale magenta). Coordenadas del ratón: sin cambios (el
  menú recibe 800×600 sin centrar). Prueba `tests/ui_menus.rs#main_menu_hover_follows_mouse`.
- Falta probarlo en Windows (pantalla completa real y fps).
- Windows: `cargo build --release -p p3-client --target x86_64-pc-windows-gnu` (en la nube, `apt-get install gcc-mingw-w64-x86-64` y
  `rustup target add x86_64-pc-windows-gnu`); solo importa DLL del sistema. Lanzador `reimpl/p3-client/Pantalla-P3.cmd` (copiado en
  `D:\AI\P3-Prueba` con `p3-juego.zip`, que descomprime la primera vez): sin argumento vista de AUTO.pat, `menu` arranque original,
  `ventana` en ventana.

## Modo de vídeo y resoluciones modernas (hilo «Añadir resoluciones modernas al juego», 12/11)

Fichero propio `p3-mvp/src/ui_rt/display.rs` (+ `display_lifted.rs`, generado), campo `Runtime::display` y gancho `display_native`,
que `builtin` consulta antes que `menu_native`. Detalle en [RESOLUCIONES_MODERNAS.md](RESOLUCIONES_MODERNAS.md).

- `set_ddraw_resolution` (`0x004BCCC0`) se atiende ahí: mismas escrituras que el cabo de `menus.rs` (que ya no se alcanza) y además
  `rt.surface` toma el tamaño del modo pedido. Quien cree el `Runtime` con otro tamaño lo verá cambiar a 800×600 en el arranque.
  El gancho de presentación de `p3-client` cambia el dispositivo con `SwitchScreenMode` cuando la superficie cambia de tamaño, y
  la ventana traduce el ratón al tamaño de la última imagen mostrada.
- `0x00429A40` y `0x00429FE0` se ejecutan desde `display_lifted.rs` (misma función, con la retirada real de sus llamadas
  virtuales; la rama de 1280×1024 de la versión de `p3_core::lifted` desplazaba la pila).
- `rt.display.modern = Some((ancho, alto))` (`p3-juego ... --moderna`): la partida usa esa resolución; los menús siguen a 800×600.

## Menú principal, carga y GameScreen (hilo «Montar el menú principal y la barra lateral», 12–13/11)

Estado en `main`:

- `menus::boot` + `menus::run`: el arranque original llega al menú principal (800x600, pantallas centradas) y a la pantalla de
  cargar partida completa (lista de `save/ein/*.pat`, botones con texto). Guion de prueba en `ui_game`:
  `P3_SCRIPT="3:6cc6b8=4;8:*6cbb7c+11c4=0;30:stop"` (opción 4 del menú y entrada 0 de la lista).
- Carga de `AUTO.pat` con el código original: cabecera `0x547BE0`, `operations_load_game` `0x5476E0`, lectores por subsistema
  de `0x549F40`, descompresor `0x6387E0` (nativo con el porte A03 `p3_core::route::rou::decompress`), `game_screen_init_resources`
  `0x424D50` con el contenedor de texturas (`arc_*`, `PathMatchSpecA`) y la escena de ciudad `0x589360`. Ahora llega a
  `0x58A4F0`/`0x595CA0` (objetos de la ciudad); el primer fotograma de ciudad aún no sale.
- `menus::construct_globals`: inicializadores estáticos del CRT en el orden de `_initterm` (constructores, vectores de objetos,
  tablas virtuales en línea), salvo `SKIP_CTORS` (mundo, barcos, operaciones, app MFC, lista de pantallas, caché INI, texturas).
- `tools/ui_rt_fix.sites` también sirve para elevar funciones que faltan (línea `0xFUNC 0x00000000 0`); `ui_rt_fix.py` prueba
  primero `--fpu-exit` (rutinas x87 del CRT en línea). Unas 230 funciones en `fix_lifted.rs`.
- Diagnóstico: `P3_MISSING=1` anuncia cada función ausente; `P3_PROF=n` imprime la pila cada n llamadas. Agotar `call_limit`,
  la profundidad o el montón desenrolla hasta `Runtime::call` (`aborted`), en vez de seguir girando con ceros.
- `arc_CreateMemMapped` con contenedor 0 lee de la carpeta del juego y, si no está, del CPR.
- El ratón del menú principal lo lleva el hilo «Arreglar el ratón en el menú principal».

### Ciudad y barra lateral (13/11)

- El primer fotograma de ciudad sale completo con `AUTO.pat` (barra superior, ciudad, minimapa, iconos y paneles de la barra
  lateral). Cambios del enlazador que lo permitieron: destinos de dibujo en textura (`sgl_SetRenderDest`, `Lock/UnlockTexture(-4)`),
  `sgl_GetColorKey`, clave de color de origen en las copias a pantalla para todos los modos, retirada de 814 desapilados falsos
  tras llamadas `sgl_*` en J08 (`oracle/lift_x86.py` `callee_sig` y prueba `j08_cdecl_cleanup`), listas libres en el montón y
  recortes 2D (`SglClip::passes`) en `sgl_DrawBitmapRect`/`sgl_FillSolidRect`.
- Reloj: `ui_game` avanza `timeGetTime` `P3_FRAME_MS` ms por fotograma (40). Sin reloj las animaciones por tiempo no avanzan (el
  pergamino del menú lateral, `window_base44_update` `0x4415D0`); con él corre la partida (habitantes, barcos, tareas
  programadas), por eso el cierre de funciones elevadas pasa de ~270 a ~550 en `fix_lifted.rs`. El contador de dinero de la
  barra superior sube por saltos hasta la cifra real (`0x423840`, paso potencia de 10 + 7): es del original.
- Entrada por guion: `P3_INPUT="F:click=X,Y;F:rclick=X,Y;F:move=X,Y;F:key=VK;F:shot=ruta.png"` (fotograma F contado por
  `sgl_SwapBuffer`; un clic mueve en F, pulsa en F+1 y suelta en F+2). Empezar con `1:move=512,384` para que el cursor en (0,0) no
  desplace la vista por el borde.
- Comprobado con clics (pantalla 1024x768; Opciones, Cargar y Guardar son de 800x600):
  - edificio de la ciudad (530,220) → ventana del ayuntamiento y su menú lateral con 7 botones;
  - iconos de la barra lateral en y=285: x=790 lista de barcos, 826 mapa de Europa, 868 vuelta a la ciudad, 910 menú de
    construcción, 950 carta de navegación con las ciudades, 990 Opciones;
  - Opciones: Cancelar (527,562) vuelve a la ciudad, Cargar (209,562) y Guardar (368,562) abren sus pantallas;
  - Guardar: AUTO (452,146), Guardar (668,542), «¿Sobrescribir?» con dos botones, sí (302,352) escribe `save/ein/AUTO.pat`
    (`CFile` de MFC nativo sobre la carpeta del juego en `menus.rs` y compresor `0x6381E0` nativo con
    `p3_core::route::compress`); esa partida vuelve a cargar desde el menú principal.
- Correcciones de argumentos en llamadas virtuales (`tools/ui_rt_fix.sites`): `0x5A9780` retorno `0x5A99B6` 1 (refresco de los 8
  botones del menú lateral, antes salía del vector y llamaba a 0x0), `0x4AC700` (posición de los botones del cuadro de mensaje,
  `0x4AC7F5` 2) y `0x431CC0`. `viper_slider_draw` `0x4D02A0` se eleva de nuevo (la versión J08 descuadraba la pila y dejaba las
  Opciones sin responder).
- Depuración: `P3_SGL_LOG` (cada llamada SGL con su pila), `P3_WATCH=0xA,0xB` (llamadas a esas funciones con `this`, pila y
  quién llama) y `P3_WATCHMEM=DIR` o `*PUNTERO+DESP` (función que deja cambiada esa palabra).
- Cargar desde las Opciones con una partida abierta: Cargar (209,562), AUTO, Cargar (668,542) vuelve a la ciudad. Hacía falta
  el nodo cabeza de las listas de los 16 centros comerciales (`0x6DEFB0`, ctor `0x533810` dentro de `game_world_construct`, que se
  salta): `construct_globals` los crea; sin ellos la segunda carga vaciaba una lista sin cabeza y no terminaba.
- Lista de barcos: `ship_list_window_draw` `0x473220` re-elevada (la de J08 pasaba mal la x de las filas) y argumentos reales en
  `0x473650`, `0x473B20`, `0x4785B0` y `0x476DC0` (`P3_ARGLOG` filtrado).
- Pendiente: textos de los botones del cuadro de mensaje (tabla `0x699948`/`0x699950`); los destructores sin nombre `0x4F3FA0` (comerciante) y `0x633520` (registro de asedio, salto a `0x6339C0`) ya están portados (`p3_core::ui_dtors`, paridad x86 en `x86_ui_dtors.rs`, enlazador en `p3-mvp/tests/ui_dtors.rs`).
- Líneas magenta del agua (13/11): no eran el efecto de ondas (`water_ripple_render_refraction` `0x59E8E0` ni se llama: `Lock(-4)` sin
  destino en textura devuelve 0; pendiente aparte). Las pintaba `Klippen_Flut.bmp` (marea de los acantilados, indicadores
  `0x1000003`, rutina alfa 4) en el búfer de la escena: su magenta debe ser transparente y `decode_bmp` lo dejaba opaco. AIM.dll
  (`AIM_CONVERT_MEMFILE` sobre un `.bmp`, `aim_stage` de `x86_sgl_load.py` llamado con un nombre `*.bmp`, no `x.aim`) toma el alfa de cada entrada de la paleta
  como `255 - byte reservado` (0 opaco, 0xFF transparente, 0x80 -> 0x7F); `decode_bmp` ya lo hace. Prueba
  `p3-render/tests/bmp_flut_alpha.rs` (P3_CPR; 682.423 píxeles magenta, todos y solo ellos con alfa 0) y unitaria
  `palette_reserved_byte_is_inverted_alpha`. En `ui_game` de AUTO.pat los píxeles magenta pasan de ~3.000 (fotogramas 50-85) a <20 (sprites).

### Cuadro de mensaje y guardado (13/11, tarde)

- `0x64F61F`/`0x64F5E3` son `CString::operator+=` (ConcatInPlace `0x64F584`), no asignaciones: guardar compone ya
  `save\ein\AUTO.pat` y vuelve a preguntar «¿Sobrescribir?».
- Los botones «Yes/No» del cuadro de mensaje salían vacíos: la versión J08 del dibujo de botón `0x464C80` pasaba destino de
  dibujo 0 a `sgl_font_draw_text_at` (`0x4C1780`), que entonces no pinta. Se re-eleva con `0x00464c80 0x00000000 0` en
  `tools/ui_rt_fix.sites`. Captura: `/mnt/project-files/interfaz-original/barra-lateral/sobrescribir-si-no.png`.
- `p3-juego menu` sube el tope de llamadas por fotograma (empezar partida hace ~2 M) y toma `--juego DIR` para `save\ein`;
  `Pantalla-P3.cmd` arranca por defecto en el menú y deja la salida en `p3-juego.log`.


### Limite del zoom segun el terreno (13/11)

Al alejar, la vista no debe superar el mapa de la ciudad. El minimo se calcula
con su tamano y los margenes del renderer: Luebeck llega a58% en1440p y88% en4K;
a1080p conserva50%. El desplazamiento usa el area ampliada. Fuentef776b83b5,
main4b258f4e9; [pruebas y limites](../coordination/revision/UI_ZOOM_BOUNDS_2026-11-13.md).

### Autoguardado (14/11)

`operations_pump_time_advance_and_autosave` (`0x00546620`, desde `execute_operations` `0x00546870`) encola la operación `0xC2`
«AUTO» (ranura `-1`) cuando el reloj `0x006DCCF8` pasa el plazo `ops+0x940` (objeto `0x006DF2F0`, bit `0x10` de `ops+0` y
modo distinto de 2). El intervalo `ops+0x944` sale de las Opciones (`set_autosave_interval_by_index` `0x00549BA0`, de 2 a 10
min) o de `scripts.ini` `[TIME_BETWEEN_SAVES] minutes` (15, `0x005465B0`). `0xC2` llama a `operations_save_game` (`0x005472F0`):
si devuelve 1 reintenta al minuto; si guarda, plazo = reloj + intervalo. El enlazador salta `operations_constructor`
(`0x00546450`) en `menus::construct_globals` y `ops+0x91C` quedaba a 0, así que el guardado se rechazaba siempre y `AUTO.pat`
no se escribía. `menus::seed_operations_fields` siembra los campos propios de ese constructor (los de `TimeMode::default`,
prueba unitaria) sin la base `0x0054A600`. Prueba de punta a punta: `tests/ui_autosave.rs` (carga, 200 fotogramas de 5 s,
escribe `save/ein/AUTO.pat` y lo vuelve a cargar).

### Barcos parados en la carta y cierre en la ruta automática (15/11, hilo «Arreglar barcos parados en la carta»)

- `town_effect_sprite_draw_mask_layer` `0x579EC0` llama a la escena por `[vt+0x70]` (`0x57A078`) y `[vt+0x6C]` (`0x57A096`) sin
  argumentos de pila: lo apilado antes son argumentos de `0x4BB330` (`add esp, 0x18`). El elevador contaba 2 y 1 y la pila quedaba
  12 bytes corrida al dibujar la sombra de un efecto de tipo `0x20`-`0x25`; la ranura pasada a `0x559A10` era basura y se leía y
  escribía fuera del bloque de enlaces. Síntomas en la partida de Sergio: cabeceras de las listas de barcos `0x6DD888` pisadas
  (barcos en estado `0xF` fuera de la lista del mar, parados en la carta), reserva de paradas de ruta rota (bucle en `0x4934B0` hasta
  el tope del montón) y `faltan (0, 559a10)` / `(4285b3, 559a10)` en `p3-juego.log`. Sitios en `tools/ui_rt_fix.sites`, función
  re-elevada con `ui_rt_fix.py --only 0x579ec0`; prueba `p3-mvp/tests/ui_effect_shadow_sites.rs`. Comprobado con `p3-juego menu
  --guion` cargando la partida: antes fallaba entre 1.500 y 2.900 fotogramas, después tres pasadas de 5.000 sin llamadas ausentes.
  Una partida ya guardada con las listas rotas no se recupera; hay que volver a una anterior.
- Segundo caso (mismo día, exe ya con lo anterior: contadores de barcos de la barra a 0 y barcos sin moverse, sin `faltan`):
  `dialog_parchment_close_with_result` `0x41E680` avisa por un puntero a función cdecl (`push ecx; call eax; add esp, 4`) y el
  elevado retiraba el argumento y además aplicaba la limpieza: 4 bytes de más, `esi`/`ebx` cambiados y la pila de los llamantes
  corrida al cerrar un pergamino con aviso. Sitio `0x41E7B1` con 0 en `ui_rt_fix.sites`. No se reprodujo el fallo de Sergio en
  ejecución (su AUTO anterior corre bien 7 días): es el único candidato firme que da `tools/ui_rt_scan_pops.py`, un barrido
  estático del EXE contra los argumentos retirados en el código elevado (dos reglas; ver su cabecera).
- Tercer caso, el que de verdad paraba los barcos con el idioma español (registro de Sergio con `P3_WATCHMEM=6DD888`:
  `memoria 0x6dd888: 0x6b0020 -> 0x600150ec al volver de 0xe06daa08`, dentro de `sgl_DrawText`): `language.rs::redirect_slots`
  barría todo `.rdata`/`.data` cada 30 fotogramas y cambiaba por la dirección de la traducción (`0x6000_0000`…) cualquier palabra
  igual a la dirección de un literal traducido. Las cabeceras de las listas de barcos son dos `u16` seguidos y con ciertos
  índices valen una de esas direcciones. Ahora solo se reescriben las ranuras fijadas al cargar el paquete: las que ya apuntan al
  literal en la imagen del EXE y los destinos de `mov dword ptr [dir], literal` (`C7 05`). Con la partida «Atasco» y el paquete
  español, 4.000 fotogramas: la única palabra fuera de esa lista que el barrido antiguo habría tocado era `0x6DD888`. Prueba
  `p3-mvp/tests/ui_language_slots.rs`. Solo pasa con un idioma distinto del inglés: las reproducciones sin paquete no lo ven.
- Diagnóstico: `P3_WATCHMEM=6DD888` anota quién cambia las cabeceras; `P3_MISSING=1` anuncia la primera llamada a una dirección basura.

### Rutas de los barcos (14/11, hilo «Arreglar las rutas de los barcos en el mapa»)

- `__allmul` `0x63A6A0` devuelve 64 bits en EDX:EAX y `host.call` solo trae EAX: en `select_nodes` `0x444800` las distancias
  al cuadrado perdían la parte alta y las rutas cruzaban tierra. `ui_rt_fix.py` lo sustituye por `seh_pending::native` igual que
  `__alldiv`/`__allshr`. Comprobación: `ui_game EXE CPR x.png boot run routes_native route_diff:2000` con `P3_GAME_DIR`
  (tabla ciudad→ciudad y rutas al azar contra `p3_core::route`) y `tests/ui_route_planner.rs`.
- `0x56E000` llama a `[vt+0x4C]` (`0x56E3E0`, `ret` sin argumentos) y reutiliza los dos argumentos apilados para `0x5595F0`:
  sitios `0x56E0EE`/`0x56E10B` con 0 argumentos en `tools/ui_rt_fix.sites`. Sin ello un barco que cambia de pose no se borraba
  de las casillas y la lista de profundidad (`0x559A50`) acababa cíclica (cuelgue al salir del puerto).
