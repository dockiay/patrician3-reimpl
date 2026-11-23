# Interfaz que necesita objetos reales (UI-REAL-PORT, 12/11/2026)

Alcance: las funciones de interfaz sin portar que exigen objetos vivos (GameScreen host y menús laterales, barra de estado SGL2D,
vista de ciudad, lista de barcos del mapa local, páginas de depuración, pantallas multijugador/cargar/nueva partida). No incluye las 45 de
BLD/SCR del hilo ELV. De 196 funciones sin fila quedan **42** (lista abajo); el resto tiene fila `UI-REAL-PORT` con paridad x86.

## Método

1. Elevar con `reimpl/tools/lift.py` a `p3-core/src/lifted/real*.rs`. Mejoras de esta tarea: `--size DIR=BYTES` y tamaño real de funciones con
   funclets `Catch@` dentro (el catálogo no cuenta sus bytes; antes el elevador infería mal los `ret N` de los llamados y dejaba la pila desequilibrada),
   llamadas de cola con el número de argumentos observado (`nover` con clave STOP). `tools/real_retry.py` reinyecta los `nover` observados por el trazador.
2. Trazar con `oracle/snaptrace.py` sobre volcados reales (menú, ciudad ×2, edificio; `ui-real-states` de Codex): el objeto, las vtables y los globales
   salen del volcado. `SYNTH=1` construye el objeto (vtable original, campos a cero) **solo si el volcado no tiene instancia viva**: eso no es un estado
   capturado y consta en la nota de cada fila. `oracle/snapbatch.py` lanza una traza por función con tiempo límite.
   Mejoras del trazador: llamadas de cola, `CString::CString` (deja el puntero al texto en [ecx]).
3. Prueba `lifted_real_match_x86` (`p3-core/tests/x86_lifted.rs`), lista `tests/fixtures/lifted_real_passed.txt`, fixtures `lifted_r_*.json` (≈25 MB), filas con `tools/real_rows.py`.

## Límites

- Registros, pila y argumentos iniciales se construyen; no son entradas capturadas de la función. Cobertura de aristas parcial en las grandes (≈50 %).
- Los fragmentos `_tail` solo se cubren a través de su función madre cuando ésta pasa (start_game, update_hotseat_setup, std_string_copy_alloc); no se midió su cobertura aparte y no tienen fila propia.
- 46 thunks y 21 funclets `Catch@` (real2) pasan con 32 trazas cada uno, salvo los tres `window_draw_at_origin` con número de argumentos corregido en real6.

## Pendientes (42) y motivo

- Pila no deducible en el elevador (SEH con `and esp`, CString temporales): `game_screen_host_update`, `render_window_title`, `main_menu_init` (+ `main_menu_create_screens_tail`), `load_save_screen_delete_savegame`/`remove_list_entry`, `multiplayer_screen_draw_wait_overlay`/`reset_to_connection_mode`, `game_main_window_run`, `draw_interior_window_ornaments`, `statistics_window_*`, `class75_load_texture_list`, `tutorial_window_place_and_layout`, `merchant_debug_dialog_show_selected_ship`.
- Elevada pero la traza no coincide (cuenta de argumentos o lecturas de pila del lote laxo): `multiplayer_screen_load_layout`/`enter_client_lobby`, `convoy_choice_window_layout`, varias páginas de depuración, `hanse_content_window_show`, `class22_*`.
- Función enorme repartida en el catálogo: `personal_letters_side_menu_open_selected_letter` (~29 KB con tabla de saltos).
- Sin traza válida o colgadas: `std_map_erase_subtree` (recursiva), `load_gl_cfg_and_init_graphics`, `battle_publish_player_side_ships`, `scrolling_scene_buffer_*`, `hometown_picker_dialog_open`.
- Fragmentos de SEH sin madre elevada: `route_save_to_file_failure_tail`, `custom_map_save_release_buffers_tail`, `operations_save_game_cleanup`, `save_last_won_free_buffers_and_return`, `choice_list_dialog_on_delete_answer_tail`.

Lista exacta: `reimpl/UI_OBJETOS_REALES_PENDIENTES.txt`.

## Segunda pasada (12/11, hilo ELV)

`lift.py` ahora restaura el marco en `pop ebp` y tiene `--guess` (con `--ext --lax`): las llamadas indirectas cuya cuenta de argumentos no se deduce valen 0 y el balance de pila no aborta; `tools/real_retry.py` reinyecta los `nover` observados por snaptrace. Con eso se traducen 38 de las 42 (real9; quedan `game_screen_init_resources`, `personal_letters_side_menu_open_selected_letter`, `operations_save_game_cleanup`, `save_last_won_free_buffers_and_return`). Con paridad sobre volcados: host_update, merchant_debug show_selected_ship, statistics account summary, convoy_choice layout, debug_page trade_balance_matrix, hanse_content_window_show, multiplayer draw_wait_overlay. Siguen con discrepancia en pila (lecturas fuera de lo leído por el original): barn update_wares_page, class22_update_scene_buffer_cells_and_animations, statistics prepare_town_detail_page, multiplayer enter_client_lobby/load_layout/reset_to_connection_mode.

## Tercera pasada (12/11, volcados nativos de Codex `bld-scr-native-2026-11-12`)

Con `SPRUNE=1` en `snaptrace.py` (descarta casos cuyo `ret N` observado en un sitio difiere de la mayoría) y volcados nativos, pasan además: `church_content_window_show`, `town_hall_window_open`, `weapons_smith_content_window_open` (real14/real15) y `hometown_picker_dialog_open` (real9 sobre `05-new-game.zip`). `choice_list_dialog_on_delete_answer` (0x472470) ya tenía fila J08; su tail 0x472536 es fragmento de ella.

Siguen sin paridad (módulo `real16`, trazas sobre 03/05/07/08/10): `barn_content_window_update_wares_page` y `class22_update_scene_buffer_cells_and_animations` (lectura de un local de pila que el original no lee, ya en el caso 0: la pila de la traducción se desplaza respecto a la del original tras algún sitio sin `nover`; sin localizar), `statistics_window_prepare_town_detail_page` (idem), `multiplayer_screen_enter_client_lobby` (llamadas), `multiplayer_screen_load_layout` (un byte de pila), `multiplayer_screen_reset_to_connection_mode` (ESP final difiere en 4 y EDI). `game_screen_create_side_menus_and_dialogs` (0x425EEE) da 0 casos válidos también sobre 07-game-side-menus (lecturas inválidas: fragmento con el marco EBP de su madre 0x427600). `load_gl_cfg_and_init_graphics`, `scrolling_scene_buffer_add_dirty_rect_wrapped`, `main_menu_init`, `render_window_title`, `class75_load_texture_list`, `draw_interior_window_ornaments`, `tutorial_window_place_and_layout` y las `_tail` de menú/multijugador siguen sin casos válidos.

## Cuarta pasada (12/11, tarde): las seis con discrepancia de pila

Causas encontradas en el trazador, no en las funciones: (1) `snaptrace.py` leía de memoria al final los bytes que un llamado real había escrito, y recogía basura de pila muerta de llamados posteriores (ahora guarda el valor al volver el llamado); (2) no anotaba `nover` cuando un destino directo sin `ret N` deducible retiraba argumentos (p. ej. `0x439DE0`). Con eso y `--localargs` pasan las seis: `barn_content_window_update_wares_page`, `class22_update_scene_buffer_cells_and_animations`, `statistics_window_prepare_town_detail_page` (real17) y `multiplayer_screen_enter_client_lobby`/`load_layout`/`reset_to_connection_mode` (real18).

## 12/11 tarde: `game_screen_init_resources` entera (0x424D50 + tramo 0x425EEE)

Con paridad en `real19`: 8 trazas sobre `07-game-side-menus.zip` (`SCTOR=1 SREAL=0 SPOOLRET=0.9 SPRUNE=1`), 1.763 llamadas por caso y 3.253 de 3.413 aristas.
La diferencia era de cuenta de argumentos en ocho llamadas virtuales (`push 0x64` que queda en pila para la llamada `[vt+0x64]` siguiente mientras
`[vt+0x48]` no toma ninguno); dos pasadas de `SIZES=424D50=11280 tools/real_retry.py real19 game_screen_init_resources` con los `nover` observados la cierran.
`0x425EEE` (`game_screen_create_side_menus_and_dialogs`) es un tramo de la misma función: su fila cita la madre. La versión de `real8` queda sin uso.

## 12/11 tarde: volcados nativos de Codex (`research/native-ui/2026-11-12/entries`)

- `personal_letters_side_menu_open_selected_letter` (0x47F3B0): la función real ocupa 29.339 bytes (switch de 134 tipos de carta hasta 0x48664B; `--size 47F3B0=29339`).
  Paridad en 300 casos (`real20`) sobre `entry-0047f3b0.zip` y `resources-letters-armoury-v1-entry-0047f3b0.zip`. Los volcados solo traen Tax bill (tipo 17):
  `SPOKE=6dd730:10:4:85` (nuevo en `snaptrace.py`) sortea el byte de tipo de cada registro de la tabla `[0x6DD730]`; 6.289 de 11.276 aristas.
- `choice_list_dialog_on_delete_answer` + tramo 0x472536: `real21`, 20 casos, tras tres pasadas de `real_retry.py` (cuentas de argumentos de llamadas virtuales).
- 0x4BC010 y 0x5869A0 ya tenían fila; 0x424D50/0x425EEE se cerraron antes con `07-game-side-menus.zip`.

## 12/11 noche: las 18 que quedaban sin fila

Con fila nueva (todas `probada_emulador`):

- **Tramos cubiertos por su madre ya con paridad** (solo faltaba la fila): `multiplayer_screen_start_game_tail` (0x43C596), `..._update_hotseat_setup_tail_enter_town` (0x43E148), `std_string_copy_alloc_tail` (0x4645AA).
- **Guardado** (`oracle/lifttrace.py`, módulos `elv6`–`elv8`): los tramos sueltos no se pueden trazar (entran con el marco EBP de la madre), así que se eleva la madre entera con `--size`:
  `operations_save_game` + `operations_save_game_cleanup` (0x547650; 40 casos), `operations_save_alderman_character_file` + `save_last_won_free_buffers_and_return` (0x549B3E; 28),
  `route_save_to_file` + `route_save_to_file_failure_tail` (0x4D5E9C; 12, 5 por la salida de fallo), `WritePrivateProfileStringA` (0x4BE110; 40). `custom_map_save_release_buffers_tail` (0x544D79) es el único epílogo de `custom_map_save_to_file`: lo recorren los 40 casos del porte a mano.
- **Volcados reales** (`snaptrace.py`): `load_save_screen_delete_savegame` + `load_save_screen_remove_list_entry` (`real22`, 24 casos), `class75_load_texture_list` (`real23`, 22), `game_main_window_run` (`real24`, 18),
  `main_menu_init` + `main_menu_create_screens_tail` (`real25`, 24), `render_window_title` y `draw_interior_window_ornaments` (`real26`; 8 y 14 pasadas de `real_retry.py`: todas sus llamadas virtuales toman 0 argumentos con empujes pendientes).

Cambios de herramienta:

- `_chkstk` (0x63B920): `lift.py` lo eleva como gancho que además baja `esp` en EAX bytes, y `snaptrace.py` hace lo mismo en el x86. Sin eso `class75_load_texture_list` leía fuera de la pila.
- `snaptrace.py`: una llamada recursiva (virtual que resuelve a la propia función, p. ej. `[vt+8]` del menú principal = `main_menu_init`) se registra como llamado y no como cuerpo;
  `SARGPOOL=p` hace que los argumentos de pila sean objetos del volcado; `SCLASS=Clase,Clase` da candidatos a las funciones sin `this` (`render_window_title` recibe una ventana por la pila).
- `tools/real_retry.py` acumula los `nover` de todas las pasadas en `/tmp/nover_acc.json`: cada pasada solo ve la primera llamada virtual dispar, y sin acumular oscilaba entre dos conjuntos.

Sin portar, con motivo:

- `tutorial_window_place_and_layout` (0x637E20): ningún volcado tiene la ventana del tutorial abierta; con objeto sintético los hijos (`+0xB0`) no tienen vtable y la tabla `+0x290` no existe. Hace falta un volcado con el tutorial en pantalla.

## 12/11 noche: los 19 volcados que faltaban (`research/native-ui/2026-11-12/missing19`)

- Las 18 páginas de depuración y `town_attack_battle_place_ships` (0x600060) ya tenían fila con casos generados. Ahora se repiten sobre su volcado nativo:
  `oracle/lifted_nat19.json` reúne sus entradas de especificación y `lifted_r_nat19.json` guarda 8 trazas por función (objeto y globales reales; registros y argumentos generados,
  no el contexto capturado). 18 de 19 coinciden con el porte que cita su fila, sin tocar código. `debug_page_town_ware_stock_balance` (0x417940) no da casos válidos
  (lecturas inválidas con argumentos generados: recibe ventana e índice de ciudad por la pila); conserva su paridad de `lifted_elv_match_x86`.
- `battle_publish_player_side_ships` (0x58A240): `real27`, 24 casos sobre `battle-visible-native-v1.zip` y `entry-00600060.zip` (60 de 67 aristas).
- Las filas citaban `p3_core::lifted::dbgleft::`, módulo que no existe (las funciones están en `lifted::dbg`): corregido en cinco filas.
- `functions.tsv` tenía 0x5353C0 dos veces (forma corta y larga): queda la fila del porte a mano, con nota de la versión elevada.

Sigue sin portar `tutorial_window_place_and_layout` (0x637E20): ningún volcado tiene la ventana del tutorial abierta.

## 13/11: `tutorial_window_place_and_layout` (0x637E20), la última

Los 30 volcados de `building-pages-extra` son páginas de edificios; ninguno tiene la ventana del tutorial (solo existe en el tutorial). Se traza con objeto construido:
`SYNTH=1 SSEED=b0:v6714c8,198:v6714c8,290:a` (nuevo en `snaptrace.py`: siembra miembros del objeto sintético, `v` = dword fijo como la vtable de un hijo, `a` = tabla de 16 punteros)
sobre `council-visible-v1.zip` (escena `[0x6E51AC]` real). 16 casos, 168 de 168 aristas, paridad en `real28` tras cuatro pasadas de `real_retry.py`.
Con esto no queda ninguna función de la lista sin fila.
