# Enlazador de interfaz (`p3-mvp/src/ui_rt`)

Ejecuta las funciones de interfaz ya portadas sobre una memoria con la imagen del EXE original, llamándose entre sí por
dirección como el x86. Sustituye lo que no es código del juego: CRT/MFC (`new`, `memcpy`, `CString`, `sprintf`, `sscanf`,
`strstr`...), las ranuras SGL (sobre `p3_render::sgl` y `sgl_font`) y las importaciones (`Archiver.dll` sobre el CPR,
`GetPrivateProfileStringA`, `lstrlenA`, `SetRectEmpty`).

## Estado (11/11/2026, nube de Claude)

- Arranque: inicializadores estáticos simples del CRT (`0x00692004..0x0069288C`, solo copias entre globales), gestor de
  texturas `0x006DA820`, iconos del texto y las seis fuentes `0x006DCD28 + 0xA8·N` con el propio `0x004C1060` + `0x004C1230`
  leyendo `scripts\fonts.ini` (tipo, `Params`, `SpaceWidth`, textura `.aim`).
- `TextLayout` (`0x004209E0` / `0x00420A10` / `0x00462520`) maqueta y dibuja el texto de las cartas sin ninguna función
  ausente (`tests/ui_text.rs`, `examples/ui_text.rs`). Comando `ui_text` de `p3-mvp`; la ventana Cartas del cliente muestra
  esa página y deja el texto propio como reserva si falta `Patrician3.exe` junto al CPR (o `P3_ORIGINAL_EXE`).
- Orden de búsqueda: sustitutos → `p3_core::lifted` (tabla `ui_rt/core_table.rs`, generada con `tools/ui_rt_table.py`) →
  `p3_render::j07`/`j08`/`panels`. El núcleo va primero porque `j08::lift_misc2` (`0x00462520`) suma 4 a ESP tras `strlen`
  (el elevador `oracle/lift_x86.py` toma `callee_sig` = desconocido como «desapila el llamado» cuando el cuerpo de `strlen`
  salta hacia atrás antes del `ret`); el original limpia después con `add esp, 0x14`. Afecta a otras llamadas cdecl
  con limpieza diferida generadas por ese elevador.
  **Corregido (11/11, hilo ELV):** `callee_sig` recorre ahora el flujo de control hasta el `ret` real (con respaldo lineal), de modo que
  `strlen`, `0x63add0` y `0x63bd1a` ya no suman a ESP; 12 módulos `j08::lift_*` parcheados y `p3-render/tests/j08_cdecl_cleanup.rs`
  detecta el patrón (falla con el código antiguo). `reimpl/tools/lift.py` no estaba afectado.

## Observado

- `0x00420A10(CString, x, y, ancho, 0x14, color)`, `ret 0x18`; las líneas `\r` se desplazan `-ancho_línea` desde `x` (las
  `\c`, `(ancho - línea)/2`), así que con `x` = margen izquierdo la línea derecha queda a la izquierda del margen. Falta ver
  con qué `x` dibuja el original la ventana de cartas (no se ha corregido nada: es lo que hace el código).
- `EventMessageWindow` (`0x004677B5`…) llama con `x = izquierda + 0x14`, `ancho = ancho_ventana - 0x28`, `0x14`.

## Herramientas

- `examples/ui_window`: `ui_window EXE CPR salida.png TAMAÑO CTOR [PASO...]`, cada paso `DIR[:arg,...]` (llamada con ecx = objeto)
  o `=DESPL:valor` (byte del objeto). Ej.: hospital `4000 5ee000 4024b0:400,300 =48:1 5ee740:0,0,0,0`.
- `tools/ui_rt_lib.py`: eleva con `tools/lift.py` el CRT/MFC del EXE (desde `0x00639000`) que piden las ventanas y su cierre
  por llamadas, a `ui_rt/lib_lifted.rs` (semillas en `tools/ui_rt_lib.seeds`). Lo que ya va en nativo se excluye.
- `tools/ui_rt_sweep.py`: el barrido de abajo.

## Siguiente

1. Fondos: seguir `class75_load_texture_list` (`0x4B3EA0`) y los prerender a textura (`sgl_SetRenderDest` ≠ 0 aún dibuja en pantalla).
2. Datos de partida: las funciones portadas leen el estado con la disposición del original; hace falta volcar el estado de
   `p3-sim` a esas direcciones (ciudades, comerciantes, barcos) antes de dibujar una ventana de edificio.
3. Portes que faltan (lista de abajo): son del hilo de ventanas de edificios y menús.

### Estado de partida (hecho) y lo que falta para una ventana de edificio entera

- `p3_sim::Sim::original_memory_image()` da el estado vivo en las direcciones del puente C08 (mundo `0x006DE4A0`, ciudades,
  comerciantes, oficinas, planificador, buzón; ~955.000 bytes con un guardado real) y `Runtime::seed_bytes` lo vuelca;
  `ui_window` lo hace con `P3_SAVE=partida.pat`.
- Mercado (`0x5DD9E0`): con partida, `set_page(0)` + `update` (`0x5DDD20` → `0x5DE810`) ya recorre las 20 mercancías, pero
  (a) los valores pasan por `0x52E1D0`/`0x52E430` (sin porte) y por `_ftol` `0x639C28`, que lee ST0 del llamante: el modelo
  `p3_core::x86rt` no pasa la pila x87 al anfitrión, así que hace falta que el elevador resuelva `_ftol` en línea;
  **Hecho (12/11, hilo ELV):** `lift_x86.py` resuelve `_ftol` `0x639C28` en línea (trunca ST0 a i64, EAX/EDX, pop) y las 106 llamadas de 16 módulos `j08::lift_*` están parcheadas;
  (b) las cifras van a controles hijos que dibuja el gestor de ventanas (`0x006DA5F0`), y `open` (`0x5DDB20`) usa la escena
  global `[0x006E51AC]` (Class22): hay que crear ambos objetos con sus constructores antes de abrir una ventana como el juego.

## Barrido de ventanas (11/11/2026 noche, `tools/ui_rt_sweep.py`)

Cada ventana del catálogo con constructor y dibujo (`p3-mvp/examples/ui_windows.tsv`, 97) se construye en el enlazador, se le da
tamaño 400×300 (`0x004024B0`), se marca visible (`+0x48`), se carga su maqueta y se dibuja en (0,0), sin estado de partida.
Resultado: **86 terminan, 10 agotan 40 s (bucles sin estado), 1 pánico** (`well_dialog`, pila del modelo x86). **21 dibujan con el
código original** (títulos, cabeceras y textos del juego con sus fuentes); faltan fondos (texturas que se cargan o prerenderizan
fuera del dibujo) y los datos de partida (nombres «(null)», cifras 0).

Funciones ausentes más frecuentes (sin porte en ningún módulo): `0x4B2360` window_on_mouse_release (35 ventanas), llamadas a
dirección 0 (vtable u objeto global sin crear, 26), `0x4B3EA0` class75_load_texture_list (13), `0x4B7240`
screen_window_hit_test_children (13), `0x4CAAC0` text_box_update_borders (11), `0x4B58C0` (9), `0x4BD340` (6), `0x420C70`
render_window_title (5), `0x4F9430`/`0x5303C0` (epíteto y comerciante, 5). Constructores sin porte: `0x466D70` EventMessageWindow,
`0x49AA50`, `0x401060` GameScreenHost, `0x4AB1A0`, `0x496C90`, `0x45CAC0`, `0x6369B0`.

| Ventana | Estado | Píxeles | Ausentes |
|---|---|---|---|
| anim_button | ok | 0 |  |
| animation2d | ok | 0 |  |
| arrow_outline | ok | 0 | `0` |
| barn_content_window | ok | 0 | `4b2360` window_on_mouse_release |
| barn_side_menu | tiempo agotado | - |  |
| bath_house_bribe_dialog | ok | 1083 | `420c70` render_window_title, `4439b0` sound_manager_load_sound_list, `4b2360` window_on_mouse_release, `4caac0` text_box_update_borders, `535740` operation_record_init, `543f10` schedule_operation |
| bath_house_dialog | ok | 0 |  |
| build_side_menu | ok | 0 | `4b2360` window_on_mouse_release |
| building_highlight | ok | 0 | `0`, `57fba0` building_highlight_ctor |
| building_info_popup | ok | 0 |  |
| building_interior_anim | ok | 0 |  |
| building_parchment_dialog | ok | 0 | `4b2360` window_on_mouse_release |
| building_scene_window | ok | 0 | `0`, `4b3ea0` class75_load_texture_list |
| campaign_select_screen | ok | 1070 | `4b58c0` orig, `4b7240` screen_window_hit_test_children, `64f510` |
| celebration_announce_dialog | ok | 3350 | `4b2360` window_on_mouse_release |
| chapel_info_dialog | ok | 438 | `4b2360` window_on_mouse_release, `4f9430` get_merchant_epithet_index, `5303c0` get_merchant |
| choice_list_dialog | ok | 227 | `4b2360` window_on_mouse_release, `4ca6f0` edit_box_on_focus_lost, `4caac0` text_box_update_borders |
| choose_map_screen | ok | 0 | `4b2360` window_on_mouse_release, `4b3ea0` class75_load_texture_list, `4b7240` screen_window_hit_test_children, `4bd340` clamp_cursor_to_screen |
| church_content_window | ok | 0 | `4b2360` window_on_mouse_release, `4b3ea0` class75_load_texture_list, `4caac0` text_box_update_borders |
| church_side_menu | ok | 0 | `4b2360` window_on_mouse_release |
| class22 | tiempo agotado | - |  |
| combo_box | ok | 0 | `4b2360` window_on_mouse_release, `4c9120` combo_box_close_dropdown_on_focus_lost |
| convoy_choice_window | ok | 862 |  |
| credits_window | tiempo agotado | - |  |
| crew_transfer_dialog | ok | 1242 | `5185c0` ship_get_sailor_complement |
| dyn_gfx_text_box | ok | 0 |  |
| event_message_window | ok | 0 | `0`, `466d70` event_message_window_ctor |
| expedition_dialog | ok | 0 | `4b3ea0` class75_load_texture_list |
| game_details_screen | ok | 0 | `0`, `49aa50` game_details_screen_ctor, `4b7240` screen_window_hit_test_children, `4bd340` clamp_cursor_to_screen |
| game_end_screen | ok | 28975 | `4b58c0` orig, `4b7240` screen_window_hit_test_children, `4f9430` get_merchant_epithet_index |
| game_screen | tiempo agotado | - |  |
| game_screen_host | ok | 0 | `401060` game_screen_host_ctor, `4011f0` game_screen_host_load_layout, `401350` game_screen_host_draw |
| hanse_content_window | ok | 0 | `4b2360` window_on_mouse_release, `4caac0` text_box_update_borders |
| hanse_side_menu | ok | 0 | `4b2360` window_on_mouse_release |
| hometown_picker_dialog | ok | 0 | `0`, `4b2360` window_on_mouse_release, `4b3ea0` class75_load_texture_list, `4b58c0` orig, `4b7240` screen_window_hit_test_children, `4bd340` clamp_cursor_to_screen (+2) |
| hospital_info_dialog | ok | 578 | `4b2360` window_on_mouse_release, `4f9430` get_merchant_epithet_index, `5303c0` get_merchant |
| ip_address_prompt | ok | 0 | `4b58c0` orig, `4b7240` screen_window_hit_test_children |
| leaping_critter | ok | 0 | `0` |
| load_save_screen | ok | 7534 | `0`, `4b2360` window_on_mouse_release, `4b3ea0` class75_load_texture_list, `4b58c0` orig, `4b7240` screen_window_hit_test_children, `4bd340` clamp_cursor_to_screen (+4) |
| main_menu | ok | 151 | `4b58c0` orig, `4b7240` screen_window_hit_test_children |
| market_hall_side_menu | ok | 0 | `4b2360` window_on_mouse_release |
| market_hall_window | ok | 4747 | `420c70` render_window_title, `4b3ea0` class75_load_texture_list |
| merchant_picker_dialog | ok | 0 | `4b2360` window_on_mouse_release, `4b58c0` orig, `4b7240` screen_window_hit_test_children, `4bd340` clamp_cursor_to_screen |
| message_box_screen | ok | 0 | `4b58c0` orig, `4b7240` screen_window_hit_test_children |
| militia_training_dialog | tiempo agotado | - |  |
| mint_dialog | ok | 381 | `4b2360` window_on_mouse_release, `4f9430` get_merchant_epithet_index, `5303c0` get_merchant |
| mission_select_screen | ok | 0 | `0`, `4ab1a0` mission_select_screen_ctor |
| money_lender_bank_dialog | tiempo agotado | - |  |
| money_lender_side_menu | ok | 0 | `4b2360` window_on_mouse_release |
| multiplayer_screen | ok | 0 | `0`, `25`, `3b1c7cc0`, `43f870` multiplayer_screen_release_resources, `4b2360` window_on_mouse_release, `4b3ea0` class75_load_texture_list (+10) |
| nav_map | ok | 0 | `44ab70` nav_map_draw, `4b3ea0` class75_load_texture_list |
| new_game_screen | ok | 0 | `0`, `496c90` new_game_screen_ctor, `4b7240` screen_window_hit_test_children |
| options_window | ok | 0 | `0`, `45cac0` options_window_ctor |
| p2_scrollbar | ok | 0 |  |
| personal_letters_side_menu | tiempo agotado | - |  |
| production_building_info_dialog | ok | 0 | `4b2360` window_on_mouse_release, `4ca6f0` edit_box_on_focus_lost, `4caac0` text_box_update_borders |
| route_stop_goods_dialog | ok | 1980 | `64f5e3` |
| school_info_dialog | ok | 402 | `4b2360` window_on_mouse_release, `4f9430` get_merchant_epithet_index, `5303c0` get_merchant |
| sgl_scrollbar | ok | 380 | `4bd680` draw_geometry |
| ship_list_icon_button | ok | 0 | `478590` ship_list_icon_button_ctor, `4785b0` ship_list_icon_button_draw |
| ship_side_menu | ok | 0 | `0`, `45c1f0` number_input_field_reset, `4b2360` window_on_mouse_release, `4caac0` text_box_update_borders, `638f07`, `64e7c9` (+1) |
| shipyard_parchment_dialog | ok | 282 | `420c70` render_window_title, `4b3ea0` class75_load_texture_list, `5303c0` get_merchant, `639c28` __ftol |
| shipyard_side_menu | ok | 0 | `4b2360` window_on_mouse_release |
| siege_cart | ok | 0 | `573f30` siege_cart_ctor |
| siege_dialog | ok | 0 | `4b2360` window_on_mouse_release, `4caac0` text_box_update_borders |
| siege_projectile | ok | 0 | `0`, `576690` siege_projectile_ctor |
| siege_unit | ok | 0 | `0`, `564540` siege_unit_ctor |
| sprite_entity | ok | 0 | `0`, `63aac5` |
| statistics_side_menu | ok | 0 | `0`, `402685`, `402686`, `402687`, `402688`, `402689` (+38) |
| statistics_window | tiempo agotado | - |  |
| tavern_content_window | ok | 0 | `4b2360` window_on_mouse_release, `4b3ea0` class75_load_texture_list, `4caac0` text_box_update_borders, `5cdc60` tavern_content_window_draw |
| tavern_side_menu | ok | 0 | `4b2360` window_on_mouse_release |
| text_layout | tiempo agotado | - |  |
| town_drift_fade_sprite | ok | 0 | `0` |
| town_drift_sprite_type1c | ok | 0 | `0` |
| town_effect_sprite | ok | 0 | `0`, `579a00` town_effect_sprite_ctor |
| town_flying_bird | ok | 0 | `0` |
| town_gate_side_menu | ok | 0 | `4b2360` window_on_mouse_release |
| town_hall_side_menu | ok | 0 |  |
| town_hall_window | ok | 0 | `4b2360` window_on_mouse_release, `4caac0` text_box_update_borders |
| town_info_popup | tiempo agotado | - |  |
| town_mini_map | ok | 0 | `59c200` town_mini_map_ctor, `59c590` town_mini_map_draw |
| town_static_sprite_type0e | ok | 0 | `0` |
| town_status_icon | ok | 0 |  |
| town_streak_line | ok | 0 | `0` |
| town_wind_sprite_type1e | ok | 0 | `0` |
| trade_offer_dialog | ok | 1610 | `4b2360` window_on_mouse_release, `4b3ea0` class75_load_texture_list, `4ca6f0` edit_box_on_focus_lost, `4caac0` text_box_update_borders |
| trade_parchment | ok | 434 |  |
| trading_office_content_window | ok | 0 | `0`, `25000069`, `30e85000`, `420c70` render_window_title, `4b044068`, `4b2360` window_on_mouse_release (+6) |
| trading_office_side_menu | ok | 0 | `4b2360` window_on_mouse_release |
| tutorial_window | ok | 0 | `0`, `6369b0` tutorial_window_ctor, `6374c0` tutorial_window_draw |
| viper_button | ok | 0 | `6592c2` |
| viper_message_box | ok | 120000 | `4b58c0` orig, `4b7240` screen_window_hit_test_children, `4bd680` draw_geometry |
| viper_slider | ok | 0 |  |
| weapons_smith_content_window | ok | 817 | `420c70` render_window_title, `4b2360` window_on_mouse_release, `639cf0` _strcmp |
| weapons_smith_side_menu | ok | 0 |  |
| well_dialog | pánico: p3-core/src/x86rt.rs:267:9: | - |  |

## 12/11/2026: escena, portes a mano y mercado

- `Runtime::create_scene` crea la escena global `[0x006E51AC]` como el arranque (`0x00424DD4..0x00424E00`): Class22 de `0xCBA8` bytes
  con el constructor `0x586FF0`; `ui_window` la crea con `P3_SCENE=1`. Las llamadas a dirección 0 bajan de 20 a 7 ventanas.
- `tools/ui_rt_hand.py` genera `hand_table.rs`: 68 funciones escritas a mano de `p3-core` (`ui_input`, `ui_hud`, `dialogs::confirm`,
  firma `(m, host, this, esp)`) enlazadas por su dirección. Resuelven `0x4B2360`, `0x4B7240`, `0x4BD340`. Además `get_merchant`
  (`0x5303C0`) y `_chkstk` (`0x63B920`) como nativas. `core_table.rs` pasa a 2.726 funciones (constructores de SCR-2 incluidos).
- Barrido de 97 ventanas con escena: 83 terminan, 13 agotan el tiempo, 1 pánico; 26 dibujan texto original (antes 21). Quedan
  ausentes `0x4CAAC0` (11 ventanas), `0x4F9430` (5) y los CRT `0x6592C2`, `0x639F78`, `0x64F5E3`. Los fondos siguen sin aparecer:
  son texturas que cargan los hijos del gestor de ventanas.
- Mercado (`market_hall_window`, `0x5DD9E0`): construcción, `0x5DDEC0` y `0x5DDD80` sin ninguna función ausente; dibuja con el
  código original las cabeceras «Goods / Stock / buy from / sell to» y las 20 mercancías. Sin partida las cifras no aparecen
  (`P3_SAVE` solo existe en el PC). `_ftol` ya lo resuelve el elevador (f20d0f85).

### Segundo barrido del 12/11 (tarde)

`0x4CAAC0` (`text_box_update_borders`, salto a `0x4D30F0`) entra ya por la tabla core (2.800 funciones tras el pull del elevador) y
`0x4F9430` (`Merchant::get_merchant_epithet_index`) es nativa del enlazador sobre una copia de los `0x650` bytes. Con escena:
83 terminan, 13 agotan el tiempo (`barn_side_menu`, `class22`, `credits_window`, `dyn_gfx_text_box`, `game_screen`,
`hometown_picker_dialog`, `militia_training_dialog`, `mission_select_screen`, `money_lender_bank_dialog`,
`personal_letters_side_menu`, `statistics_window`, `text_layout`, `town_info_popup`), 1 pánico, **27 dibujan** texto original.
Ausentes que quedan (una o dos ventanas cada una): CRT `0x6592C2`, `0x639F78`, `0x64F5E3`, `0x64F510`; portes `0x4BD680`
(draw_line), `0x4439B0`, `0x535740`, `0x543F10`, `0x57FBA0`; y 7 llamadas a dirección 0. El humo del puerto (J05) ya está
conectado en el cliente (`p3-mvp/src/harbor_fx.rs`) y J05 sigue reservada por otro hilo. Los fondos siguen pendientes: hace falta
que el gestor de ventanas cree los hijos de cada pantalla (`0x401060` GameScreenHost y sus textures), lote siguiente.

### Las 13 ventanas que agotan el tiempo (12/11, tarde)

Diagnóstico con `P3_UI_TRACE` sobre `credits_window`: el cargador (`0x40D8E0`) lee `./scripts/credits_eng.ini` con
`GetPrivateProfileString` (`0x4BE0B0` → `0x4BE1E0`) y trocea cada línea con `CString::Find` (`0x64D3D0`, ~1,3 millones de
llamadas, cuadrático como en el original). Se añade `Find` nativo y `_strnicmp` acotado a `n` bytes (antes copiaba las dos
cadenas enteras): ~7 veces más llamadas por segundo, pero la ventana sigue sin terminar en 100 s. Se probó servir los `.ini`
desde el CPR (`scripts/credits_eng.ini` está en él, con secciones `[ENSEMBLEn]`); el primer argumento de esas llamadas elevadas
no es un nombre de sección (apunta a código, `0x4BE07F`: el argumento se desplaza una palabra en la llamada de `0x4BE0B0`) y,
con la clave buscada en cualquier sección, `expedition_dialog` dejó de terminar, así que se **revirtió** y el valor por defecto
sigue siendo el de Windows sin archivo. Pendiente para el hilo del elevador: la firma de `0x4BE0B0`/`0x4BE060` (hay un argumento
de más o de menos) y el bucle que no acaba con el `.ini` real. Barrido: 82 terminan, 14 agotan (con `expedition_dialog`
contando solo en la prueba con `.ini`), 27 dibujan.

### Causa real de «el primer argumento apunta a código» (12/11, hilo ELV)

No era la firma elevada de `0x4BE0B0`/`0x4BE060` (coinciden con el original: `ret 0x10`/`ret 0x18`, 4 y 6 argumentos). `Runtime::dispatch` reservaba un marco nuevo en `self.sp` para las llamadas con argumentos, y el llamante elevado (`credits_window_load_layout`) tiene su pila viva por debajo de `self.sp`: el marco del llamado pisaba un local del llamante con `0x4BE07F`, el retorno del propio llamado. Corregido en `p3-mvp/src/ui_rt/mod.rs`: si el llamante ya dejó retorno y argumentos idénticos en su pila (última palabra escrita), el llamado usa esa pila; si no, el marco nuevo va por debajo de ella. `credits_window` pasa de agotar 100 s a terminar en 0,3 s (8.280 llamadas). Queda por repetir el barrido de las 13 ventanas.

### Apertura de ventanas y fondos (12/11, tarde; en curso)

- El fondo y los controles hijos no salen de dibujar la ventana sino de **abrirla** (mercado: `0x5DDB20`, que ejecuta la salida
  de `is_open` `+0x48`, crea los botones copiándolos de las plantillas de estilo y coloca la ventana). Para abrirla hace falta
  `+0x48 = 0` al llamar (si no, retorna sin hacer nada) y volver a ponerlo a 1 para dibujar.
- Se añade `Runtime::load_button_styles` (en `create_scene`): inicializador estático `0x00460730` (`eh vector constructor
  iterator(0x6CBE08, 0xE8, 6)`) y `button_base_load_layout` (`0x4C6D20`) por estilo con `./scripts/buttons.ini`, como el arranque
  (`0x424DA4`). También `AddTail` `0x64E6FC` (lib elevada), `CopyRect`, secciones críticas e `InterlockedExchange`, y los `.ini` del
  CPR por sección (`ini_value`; `P3_NO_INI=1` los desactiva) y `P3_DUMP=dir,palabras` en `ui_window` para volcar memoria.
- Estado: la apertura del mercado ya no aborta por funciones ausentes, pero las plantillas de botón quedan sin sus cuatro hijos
  (`+0x94..+0xA0` a 0): `button_base_load_layout` lee `Button0`/`TEXCOL1` de `buttons.ini` y carga imágenes por máscara
  (`images\frames_konto\konto00*.tga`), y esa enumeración por comodín de archivos no está enlazada. Es lo siguiente.
- Tras `0352242` (pila del llamante) el barrido da 91 terminan / 5 agotan (`class22`, `dyn_gfx_text_box`, `mission_select_screen`,
  `options_window`, `text_layout`) / 1 pánico, pero **solo 18 dibujan** (antes 27): diez ventanas (p. ej. `crew_transfer_dialog`,
  `game_details_screen`, `load_save_screen`) pasan a 0 píxeles porque su constructor llama a `eh vector constructor iterator`
  (`0x639E9C`, `0x40EC50` con `count = 6`) y el bucle itera miles de veces hasta el tope de 2 millones de llamadas. Ocurre también
  sin escena ni `.ini`; apunta al nuevo cálculo de marco en `dispatch` (el recuento se lee de otra palabra de la pila).

### Retroceso de 03522423 corregido (12/11, hilo ELV)

Reutilizar la pila del llamante también para las llamadas con argumentos rompía los constructores con `eh vector constructor iterator` (`0x639E9C`; el contador del bucle vive en el marco del propio iterador). Se deja solo la parte segura: las llamadas con argumentos siguen usando un marco nuevo, pero ahora por debajo de la última palabra escrita (la pila viva del llamante). `credits_window` sigue terminando (162 llamadas al `.ini`, ninguna con `0x4BE07F`) y `crew_transfer_dialog` (1.242 px), `game_details_screen` (17.414) y `load_save_screen` (18.154) vuelven a dibujar. Barrido completo: 93 terminan, 3 agotan el tiempo (`class22`, `mission_select_screen`, `text_layout`), 1 pánico (`well_dialog`), 28 dibujan.

### Tercer paso (12/11, tarde): tras b7aa9af

Barrido con escena: **92 terminan, 4 agotan (`class22`, `dyn_gfx_text_box`, `mission_select_screen`, `text_layout`), 1 pánico, 28 dibujan.**
Nuevas nativas: `CString::Mid` `0x64D0C0`, `CString::operator=(LPCSTR)` `0x64F5E3`, `CString::Delete` `0x64CC80`, el constructor `0x40E2F0`
de un elemento de créditos y los imports `SetRect`, `IsRectEmpty`, `OffsetRect`, `UnionRect`, `ClientToScreen`, `SetCursorPos`,
`timeGetTime` (`P3_TIME=ms`). `credits_window` termina sin funciones ausentes pero dibuja 0 píxeles con cualquier `P3_TIME`: el
dibujo (`0x40E020`) solo pinta tras el arranque del conjunto de créditos, que no se ha llamado. `well_dialog` da pánico en un
`idiv` entre cero (`x86rt.rs:267`) justo después de destruir una `CString`: la ventana divide por un dato de la ciudad que sin
partida vale 0 (el original también lanzaría la excepción), así que necesita estado de partida y no un porte. Sigue pendiente lo
de los fondos: `button_base_load_layout` deja sin hijos las plantillas de botón porque una de sus lecturas recibe la sección `0`
(un `CString` formateado no llega a la llamada), a localizar en `0x4C6D20`.

### Plantillas de botón: estado y pista (12/11, noche)

Con las plantillas de estilo cargadas, `button_base_load_layout` (`0x4C6D20`) lee bien la sección `Button0` de `buttons.ini` en sus
cuatro primeras lecturas (`MFile`, `Disable`, `DFile`...), pero las siguientes (`0x4BE060` con claves `Neutral`/`Pressed`/`NFile`...)
llegan con **sección = 0 y archivo = 1** (los dos primeros argumentos que el elevador lee de los locales del llamante): los locales
`CString` de `0x4C6D20` están pisados tras la llamada a `0x4B92C0`/`0x4B1AD0`. Con ello solo se crean 2 de los 4 hijos
(`+0x94`, `+0x98`) y la copia a un botón (`0x4C7A90` → `0x4ADA30`) lee un hijo nulo. Se probó dejar 0x1000 bytes de margen bajo cada
marco anidado (no arregla el caso y hace que `game_screen` y `hometown_picker_dialog` agoten el tiempo), por lo que se revirtió. Hace
falta que el elevador conozca el `esp` real del llamante al hacer `host.call` (hoy solo recibe la dirección, `ecx` y los argumentos).

### `ret N` de llamadas indirectas (12/11, hilo ELV)

Las llamadas indirectas por vtable de los módulos J08 no retiraban el `ret N` del destino (el generador solo ve el sitio). `lift_x86.py` emite ahora, tras cada llamada indirecta sin pop estático, `esp += 4 * h.callee_pop(t)`; `Host::callee_pop` (por defecto 0) la implementa `Runtime` con `p3_render::ret_pop::ret_pop`, tabla generada por `tools/ret_pop_table.py` (3.214 funciones del catálogo cuyos `ret` coinciden y retiran argumentos; p. ej. `0x4ACF90` → 2 palabras). 1.291 sitios de los módulos `j08::lift_*` actualizados. `market_hall_window` (`P3_SCENE=1 P3_DUMP=0x6cbe98,8`): 4 hijos distintos de cero (antes 2), 4.747 px. Barrido: 91 terminan, 5 agotan (`class22`, `dyn_gfx_text_box`, `mission_select_screen`, `text_layout`, y ahora `ship_side_menu`, que crea objetos de 0xD8 bytes en bucle sin estado de partida: antes «terminaba» con 0 px porque la pila estaba desplazada), 1 pánico, 28 dibujan.

### Mercado abierto con sus 4 hijos (12/11, noche, tras 8fb2cd1)

Con las llamadas indirectas arregladas, las plantillas de botón quedan con sus 4 hijos (`+0x94..+0xA0`) y `0x5DDB20` abre sin abortar.
Recetas para abrir y dibujar una ventana con `ui_window`:
`P3_SCENE=1 ui_window EXE CPR out.png 4000 CTOR 4024b0:400,300 =48:0 LAYOUT OPEN:0 =48:1 SET_PAGE:0 DIBUJO:0,0,0,0`
(mercado: `0x5DD9E0 .. 0x5DDEC0 0x5DDB20:0 =48:1 0x5DDCD0:0 0x5DDD80:0,0,0,0`; `0x5DDCD0` es `set_page`, `0x5DDD20` es el
manejador de botones, no el cambio de página). `+0x48` debe valer 0 al abrir y 1 al dibujar.
Se añaden: tamaño de la escena (`+0x2C/+0x30` = superficie) y región de recorte inicial a pantalla completa en `create_scene`
(sin ella `IsRectInClippingRegion` descartaba la ventana, ya centrada en (200,150)); `~DIRECCIÓN` en `ui_window` para llamar
métodos de la lista de pantallas (`ecx = 0x6DA5F0`); `sgl_Begin3D/Finish3D/SwapBuffer/SetZoomFactor/ClearBuffer`, `Sleep` y
`PeekMessageA` como no-ops. El mercado dibuja 4.544 px en su posición centrada: cabeceras y 20 mercancías.
Lo que aún no sale es el fondo y los botones hijos: los dibuja el recorrido de la lista de pantallas (`screen_list_run_frame`,
`0x4B8A40`), que termina en `-2` con llamadas a dirección 0 y 2 (objetos de la lista sin crear) y pide `sgl_Blt_RC`. Siguiente paso.
Barrido final: 91 terminan, 5 agotan (`class22`, `dyn_gfx_text_box`, `hometown_picker_dialog`, `mission_select_screen`,
`text_layout`), 28 dibujan. `ship_side_menu` termina en el barrido (sin partida el elevador lo ve en bucle de objetos `0xD8`).

### Fondo y botón hijo del mercado (12/11, tarde)

El mercado sale ya como en el juego: tablero colgado con cadenas, pintura interior del edificio, pergamino translúcido, cabecera
con cinta, las 20 mercancías y el botón de cerrar con su textura. Imágenes en `/mnt/project-files/interfaz-original/`
(`mercado-interior.png`, `mercado-existencias.png`); prueba `p3-mvp/tests/ui_market.rs` (`P3_EXE`, `P3_CPR`).

Qué era cada cosa:

- **Quién dibuja los hijos.** Las ventanas se cuelgan de la pantalla actual (`screen_window`, ctor `0x4B4920`, lista de hijos en
  `+0x94`/`+0xC0`) y su dibujo `0x4B58C0` los recorre. `ui_window` crea una contenedora con `P3_HOST=1` (apilada con
  `0x4B90E0`) y `^DIRECCIÓN` llama a sus métodos. `0x4B58C0` va en nativo: el elevado cuenta un argumento para el dibujo virtual
  de cada hijo (retira cuatro) y salía del bucle tras el primero.
- **El fondo** no es de la ventana de existencias: es `BuildingSceneWindow` (`[0x6CC7E0]`, ctor `0x465010`, maqueta
  `0x4653A0("./scripts/animation.ini", 0)`, 451x537, `0x10000` bytes). `market_hall_side_menu_show` (`0x5AC9C0`) la centra
  (`0x466320`), elige la escena 5 (`0x466430`) y abre la ventana; al elegir página, `0x4665D0` pone `+0xFC = 0` y la pintura queda
  bajo el pergamino (`BackTexID`). La ventana de existencias se crea como en `0x426CF3`: maqueta
  `("./scripts/BuildingParchment.ini", 0)`, tamaño 425x510, oculta.
- **Plantillas de botón**: `button_base_load_layout` es `(CString, índice)`, no `(CString, anfitrión, índice)`: las seis cargaban
  `[Button0]`. Con el índice bien, la 5 es el botón de cerrar (32x18, `ANIM13..15`, textura 16023).
- **Texturas**: `parchment_base_load_layout` (`0x462190`) registra `TEXLIST16000` al crear la escena. `class75_load_texture_list`
  (`0x4B3EA0`) reserva 0x1014 bytes con `_chkstk`, que el elevado trata como llamada corriente: se le da un marco propio con los
  argumentos donde los lee. Los identificadores solo sembrados por `seed_textures` se anulan al pedirlos el original, que los carga
  de `textures.ini` con sus rectángulos. `sgl_LoadMemoryTexture` respeta los indicadores (1 opaca, 3 alfa). Las 28 texturas `.jpg`
  (interiores) se descodifican con `jpeg-decoder` (el original usa `ijl11.dll`; puede variar algún nivel por píxel).
- **Marcos de llamada**: los elevados J08 no apilan retorno; `call_at` da ahora al llamado el marco real (`esp-4`), con lo que ve
  los argumentos que el elevador no contó (`sscanf` con dos punteros, `anim.load(CString, id)`). Los de `p3_core::lifted` usan
  la pila del llamante también cuando hay argumentos deducidos.
- **Argumentos mal deducidos en llamadas virtuales** (`get_height()` con un argumento pendiente, `set_position(x, y, z)` contado
  como uno): el enlazador los anota (`P3_ARGLOG=fichero`) comparando con lo que retira el destino real (`ret_pop`), y
  `tools/ui_rt_fix.py` vuelve a elevar esas funciones con `lift.py --nover` a `ui_rt/fix_lifted.rs`, que se consulta antes que
  p3-core. Sitios en `tools/ui_rt_fix.sites` (por ahora `market_hall_window_open` y `text_layout_draw` `0x420C70`). No se toca
  p3-core: sus pruebas de paridad usan ganchos que retiran lo deducido.
  Aviso (13/11): no aplicar un registro `P3_ARGLOG` entero sin mirar. Los sitios de `0x42ECC0` (3), `0x463D20`/`0x463DA3`,
  `0x424D50`/`0x4261B1` y `0x5513F0`/`0x551465` rompieron la carga de partida y se retiraron; filtrar el registro por las
  funciones que interesan (`grep -E '^0x(4ac700|431cc0) '`) y pasar solo ese trozo a `ui_rt_fix.py`. Una llamada que va a 0x0
  no aparece en el registro: su número de argumentos se añade a mano tras leer el desensamblado (`0x4AC700 0x4AC7F5 2`).
- Nativas nuevas: `strtol`, `abs`, `operator+` de CString (tres variantes). `ui_window`: `new:TAMAÑO:CTOR`, `use:N`,
  `glob:DIRECCIÓN`, `child`, argumentos `s=TEXTO`, `P3_CALLS`.

Receta: `P3_CALLS=100000000 P3_HOST=1 P3_SCENE=1 ui_window EXE CPR out.png 0x10000 0x465010 0x4653a0:s=./scripts/animation.ini,0
4024b0:451,537,0 glob:6cc7e0 child 4b21a0:0 0x466320 0x466430:5 0x4665d0 new:4000:0x5dd9e0
0x5ddec0:s=./scripts/BuildingParchment.ini,0 4024b0:425,510,0 4b21a0:0 0x5ddb20 0x5ddcd0:0 ^0x4b58c0:0,0,0,0` (unos 10 s: cada
consulta de INI relee el fichero del CPR). Sin `0x4665d0` se ve la pintura interior sin pergamino.

Pendiente: columnas de existencias y precios (necesitan partida sembrada, `P3_SAVE`); repetir la receta con el resto de ventanas
de edificio (mismos pasos, otra escena y otro ctor) y pasar el barrido con `P3_ARGLOG` para ampliar `ui_rt_fix.sites`.

### Barrido de las 12 ventanas de edificio (12/11, tarde)

`tools/ui_rt_buildings.py` (tabla `p3-mvp/examples/ui_building_windows.tsv`: ctor, bytes y escena interior de cada una, sacados de
`0x425EEE` y de los `select_scene` de los menús laterales) repite la receta del mercado con cada ventana abierta por `+0x120`.
Sin partida sembrada: 11 de 12 dibujan su tablero con la pintura interior y el botón de cerrar en su sitio (unos 242.800 px);
`trading_office_content_window` agota el límite de llamadas al cargar su maqueta. Imágenes en
`/mnt/project-files/interfaz-original/edificios/`. Funciones que faltan, todas de estado de partida o sonido:
`5cc120` (taberna), `52b2c0` (astillero), `5308a0`, `64d230` (iglesia, hansa, ayuntamiento, asedio), `4d9100/4d9320/4d9620/51ce80`
(prestamista), `5326e0` (ayuntamiento), `4439b0/535740/543f10` (baños), `4436e0` (armería, filtro de sonido), `639c03` (asedio,
granero). El barrido aportó a `ui_rt_fix.sites` las aperturas del prestamista, baños y astillero (botón de cerrar descolocado
antes de re-elevarlas). Siguiente: sembrar partida (`P3_SAVE`) y elegir página en cada ventana.

## 12/11 tarde: estilos de botón en el barrido y `ship_side_menu`

`ship_side_menu` no entraba en bucle por el porte: `ship_side_menu_load_layout` copia sus botones de las seis plantillas globales `0x006CBE08`
(`button_base_copy_to`), que sin `load_button_styles` están vacías; copiaba desde `this = 0` hasta pedir una reserva enorme. `examples/ui_window`
carga ahora los estilos siempre (con `P3_SCENE=1` ya lo hacía `create_scene`). La ventana termina; le faltan `0x638F07`, `0x6394FD`, `0x64E7C9`
y `0x64E85C` (biblioteca) y datos de barco. Barrido: 92 terminan, 4 agotan el tiempo (`class22`, `dyn_gfx_text_box`, `mission_select_screen`,
`text_layout`: se dibujan sin maqueta y recorren contadores sin inicializar), 1 pánico (`well_dialog`), 29 dibujan.
### Mercado con partida sembrada (12/11, noche)

Con `P3_SAVE=partida.pat` la ventana de existencias del original dibuja las 20 filas con existencias en lotes, icono de barril o
carga, precio de compra y de venta: `/mnt/project-files/interfaz-original/mercado-con-partida.png`. Hizo falta:

- Ciudad a la vista: las ventanas de edificio la leen de `escena+0xC324` al abrirse (`0x5DDB3E`); `ui_window` escribe la de la oficina
  del jugador (o `P3_TOWN=N`).
- `get_sell_price 0x52E1D0` y `get_buy_price 0x52E430` como nativas del enlazador sobre `p3_core::economy::prices` (thiscall de
  `0x6DE3D8`; mercancía, ciudad, cantidad), y el factor de venta de la dificultad en `0x6DE3D8+0x64` (`Runtime::seed_session`).
- Paso de actualización `0x5DDD20` (→ `0x5DE810`, que llena `+0x1A0` lotes, `+0x1F0` compra y `+0x240` venta en el orden de la tabla
  `0x698538`) antes de dibujar.
- Alfa de los TGA de 32 bits sin `UseAlpha` en `textures.ini` (iconos `16030`/`16031` con fondo negro): el cargador del original
  pide indicadores 3; `Assets::image` lo aplica al decodificar.

Prueba: `tests/ui_market.rs#market_hall_with_save_shows_stock_and_prices` (necesita `P3_EXE`, `P3_CPR`, `P3_SAVES`).

### Barrido con partida sembrada (12/11, noche)

`P3_SAVE=… python tools/ui_rt_buildings.py …`: con la partida, 11 de 12 abren y dibujan su interior y el botón de cerrar; iglesia,
hansa, armero, asedio, granero y mercado ya no piden ninguna función. Nativas nuevas del enlazador: `_rand 0x639C03`,
`CString::Left 0x64D230`, `get_merchant_reputation_in_town_int 0x4F9580`, `find_or_create_office 0x5308A0` (solo busca, no crea) y
el sonido `0x4436E0`/`0x4439B0` sin efecto. Siguen pidiendo función: taberna `5cc120` (su `show`, sin portar), astillero `52b2c0`
(presupuesto de barco nuevo), prestamista `4d9100/4d9320/4d9620/51ce80` (listas de deudas y ofertas), ayuntamiento `5326e0`,
baños `535740/543f10` (encolan la orden de entrar). Todas tienen porte tipado en p3-core/p3-sim; falta el adaptador sobre memoria.
`trading_office_content_window` sigue agotando el límite al cargar su maqueta. Como en el original, estas ventanas muestran solo
la pintura hasta que el menú lateral elige página: el siguiente paso es llamar a `select_page` de cada una (como `0x5DDCD0` +
`0x5DDD20` en el mercado) y los menús laterales.

### Prestamista con partida sembrada (12/11, noche)

`money_lender_bank_dialog` (ctor `0x5B2800`) abre y dibuja sin funciones ausentes. Al abrirse (`0x5B2BC0`…`0x5B2CCC`) pide las deudas del
jugador `0x4D9100` (lista de 0x18 bytes en `+0xFE0`, cuenta `+0xFE4`), las ofertas de préstamo de la ciudad `0x51CE80` (registros en `+0xF5C`,
cuenta `+0xFD0`, ciudad `+0xFCC`), el reparto del jugador `0x4D9620` (`+0xF9C`/`+0xFC0`) y las ofertas de reparto `0x4D9320`
(`+0xFA0`/`+0xFC4`), todas thiscall sobre las tareas programadas `0x6DD73C` salvo `0x51CE80` (ciudad). Son nativas del enlazador sobre los
portes con paridad (`p3_core::economy::task_lists` con el montón del enlazador como `Heap`, y `loan_offers::build` sobre copias de la ciudad y
los comerciantes). Prueba `tests/ui_lender.rs`: con `AUTO.pat` las tres ofertas (total, tipo, importe, semanas) coinciden con
`Session::lender_screen` del cliente y la cuenta de deudas también (0 en esa partida).

### Astillero con partida sembrada (12/11, tarde)

`shipyard_parchment_dialog` abre con la partida sin funciones ausentes. Su apertura (`+0x120` → `0x5F3AD0`) pide el presupuesto de
una snaikka en la ciudad a la vista (`quote_new_ship 0x52B2C0`, thiscall de `[0x6DE508]+ciudad·0x9F8`; salida `ventana+0x640`, clase 0,
dueño `[0x6DFC14]`; `+0xCB8` = ciudad). Es nativa del enlazador sobre `p3_core::economy::ops_misc::quote_new_ship` (800 casos x86),
con oficina, comerciante y precio de compra devueltos al enlazador. Prueba `tests/ui_shipyard.rs` (necesita `P3_EXE`, `P3_CPR`,
`P3_SAVES`): coste `+0x50` igual al del encargo tipado del cliente (`Sim::shipyard_order_quote`, ciudad 11 de AUTO.pat: 16.030).
Imagen: `/mnt/project-files/interfaz-original/edificios/shipyard_con_partida.png` (pintura y botón de cerrar; la página la elige el
menú lateral, igual que en las demás).

### Oficina comercial: la maqueta ya carga (12/11, noche)

`trading_office_content_window` no entraba en bucle: agotaba el **montón** del enlazador (tope de 128 MB) al cargar su maqueta
(`0x005D9710`), que además carga las caras de los retratos (`class75_load_texture_list` con las listas `0x7D00` y `0x88B8`, unas 46
texturas). Cada textura consulta `textures.ini` clave a clave con la caché INI del original (`0x004BE1E0`, objeto `0x006DD524`);
sin caché, cada clave proyecta el archivo entero con `arc_CreateMemMapped` (`0x006389EE`) y lo recorre, sin liberar la vista. En el
juego la vista apunta dentro del contenedor ya proyectado (solo lectura), así que no cuesta memoria; el enlazador copiaba los
108 KB en el montón en cada una de las ~3.800 consultas. Ahora `arc_CreateMemMapped` copia cada archivo una sola vez y devuelve la
misma dirección en las siguientes proyecciones (`Runtime::mapped`). Ninguna función nueva hacía falta.

Con la partida sembrada la oficina abre y dibuja su interior con el botón de cerrar sin ninguna función ausente (242.840 px, como las
demás): `/mnt/project-files/interfaz-original/edificios/trading_office_content_window.png`. Prueba:
`tests/ui_trading_office.rs#trading_office_loads_layout_and_opens_with_save` (necesita `P3_EXE`, `P3_CPR`, `P3_SAVES`).
Siguiente, al elegir página (`select_page` `0x005D9A20` + `update` `0x005D9500`): la página 0 pide `0x5311D0` (desde `0x5DC2A0`,
balance) y `0x64F61F` (desde `0x5DA5B0`, total); la 1 pide `0x63B3E0` (desde `0x5DBF70`, existencias); la 2 no pide nada.

### Baños con partida sembrada (12/11, noche)

`bath_house_bribe_dialog` (constructor `0x5B1410`, escena 8) ya abre y dibuja con `P3_SAVE` sin pedir ninguna función. Su constructor
encola la orden `0x39` (comerciante del jugador `0x6DFC14`, registro `0x6DE4B4+0xA00`) con `operation_record_init 0x535740` y
`enqueue_operation 0x54AA70` → `schedule_operation 0x543F10` sobre el paquete `0x6DF2F0+0x48`. Las dos que faltaban están elevadas con
`tools/lift.py` en `p3_core::lifted::bath` (trazas `oracle/lifttrace.py`, 60 casos por función, prueba
`p3-core/tests/x86_lifted.rs#lifted_bath_match_x86`) y entran en `core_table.rs`. Prueba de la ventana:
`p3-mvp/tests/ui_bath.rs` (comprueba la orden en el paquete y que no falte nada; necesita `P3_EXE`, `P3_CPR`, `P3_SAVES`). Captura:
`/mnt/project-files/interfaz-original/edificios/bath_house_con_partida.png`. Como las demás, muestra la pintura hasta que el menú
lateral elige página.

### Ayuntamiento con partida sembrada (12/11, noche)

`town_hall_window` (ctor `0x5DF530`) abre y dibuja sin funciones ausentes. Su apertura (`0x5DFA60`, final en `0x5E05E9`) llama a
`schedule_prep_alderman_missions 0x5326E0` (thiscall del mundo `0x6DE4A0`): prepara las misiones del alcalde y las encola con
`enqueue_operation 0x54AA70`, que las guarda en la cola de órdenes `0x6DF2F0+0x48` (`schedule_operation 0x543F10`: bloques de 0x34
registros de 0x14 bytes, cuenta en `+0xE`, bloque siguiente en `+0x420`). Con `ConvoyHecho.pat` deja tres órdenes: 0x86, 0x85, 0x85.

Las funciones que faltaban son elevadas con `tools/lift.py`: `0x5326E0`, `0x4D8DB0`, `0x4EC5F4`, `0x51A470`, `0x532E30` y `0x543DA0` en
`p3_core::lifted::ayto`; `0x535740` y `0x543F10` son las de `p3_core::lifted::bath` (hilo de los baños): el código elevado es idéntico al que salió aquí y sus
32 trazas sobre los volcados del ayuntamiento también coincidían (no se guardan para no duplicar). Paridad de `ayto`: 32 trazas x86 por función sobre los
volcados nativos `09-town-hall.zip` y `town-hall-cotejo13-user-confirmed.zip` (`lifted_r_ayto.json`, `lifted_real_match_x86`); aristas
415/431 en `0x5326E0`. Mejoras de `oracle/snaptrace.py`: `STHIS=func:dir,dir;…` fija THIS a objetos del volcado sin vtable (globales
como `0x6DE4A0`, `0x6DD73C`, la cola o barcos de `[0x6DD7A4]`), y una llamada recursiva a la propia función se intercepta como cualquier
otro llamado (antes `0x543F10` se ejecutaba dentro y no casaba). Valores usados:

```
STHIS="schedule_prep_alderman_missions:6de4a0;determine_new_settlement:6de4a0;scheduled_tasks_get_task:6dd73c;scheduled_tasks_collect_open_alderman_missions:6dd73c;ship_get_pirate_band_index:1f326070,1f3261f0,1f3267f0,1f3279f0,1f329c70,1f32d3f0;schedule_operation:6df338,6df2f0;operation_packet_clear:6df338;operation_record_init:6df348,6df35c"
```

Prueba: `tests/ui_town_hall.rs` (necesita `P3_EXE`, `P3_CPR`, `P3_SAVES`). Como el resto, la ventana muestra la pintura hasta que el menú
lateral elige página; las páginas (`0x5E0BA0` y los `draw_*_page`) ya están elevadas en `elv1..3`.
### Taberna con partida sembrada (12/11, noche)

`tavern_content_window` abre y dibuja con `P3_SAVE` sin pedir ninguna función. El `show` de la ventana (`0x5CC120`, 4.466 bytes) y lo que
llama de la partida están elevados con `tools/lift.py --fpu-exit` en `p3_core::lifted::bld_tavern` (12 funciones: el `show`, capitán de la
taberna `0x5269A0` con `0x4FE150`, armero `0x526470`/`0x5263D0`, oferta de mercancía `0x526650`, noticias `0x526B40` con `0x4D75D0`,
`0x4EC694`, `0x5353C0`, `0x535300` (y `0x4D8DB0`, que es la de `ayto`), y rivales con oficina `0x531470`). Paridad: 373 trazas de `oracle/snaptrace.py` sobre los
volcados nativos de la iglesia (`12-church.zip`, `church-cotejo13-user-confirmed.zip`; `SYNTH=1 SPRUNE=1`), prueba `lifted_real_match_x86`.
Los `nover` del `show` se acumulan a mano en `oracle/nover_bld_tavern.json`: `real_retry.py` solo lee el último informe y perdía los de
pasadas anteriores (patrón `push 0x64; push x; call [vt+0x44]` que deja los dos argumentos para el `[vt+0x64]` siguiente).
Además: `Sim::original_memory_image` siembra la tabla de capitanes `[0x6DD7A0]` (sin ella `0x5269A0` no termina) y nativas MFC
`operator+(CString, char)` `0x64CB60`, `operator+(char, CString)` `0x64CBF0` y `CString::operator=(const CString&)` `0x64F61F`.
Prueba: `tests/ui_tavern.rs` (precios del armero de la ventana iguales a `Sim::weapon_quote`). Como en el original, sin página elegida la
ventana muestra la pintura del interior; falta `open_page` `0x5CED00` y el menú lateral.

### Páginas de las ventanas de edificio (12/11, noche, hilo «Montar la escena y el gestor de ventanas»)

La escena global (`Runtime::create_scene`, Class22 en `[0x006E51AC]`) y la lista de pantallas `0x006DA5F0` con su contenedora
(`screen_window` `0x4B4920`, hijos dibujados por `0x4B58C0`) ya estaban; lo que faltaba era **elegir página** y ejecutar el `update`
de cada ventana, y las funciones de partida que piden las páginas. Ahora las doce ventanas pintan sus páginas con datos de la
partida sembrada: hoja de capturas en `/mnt/project-files/interfaz-original/paginas/paginas-edificios.png` (65 páginas).

- Receta (`tools/ui_rt_pages.py`, tabla `p3-mvp/examples/ui_building_pages.tsv`): como `ui_rt_buildings.py`, más la pintura bajo el
  pergamino (`0x4665D0`), el **menú lateral** del edificio creado como en `0x00425EEE` (bytes, ctor y global de cada uno; maqueta
  `./scripts/sidemenu.ini`) y los **globales** del juego apuntando a la ventana (`0x6E5534`..`0x6E55C0`) y al menú (`0x6E54C8`..
  `0x6E5524`): las páginas los consultan (p. ej. la taberna cierra la página con `0x005A6E20` sobre `[0x6E54F8]`). Después
  `select_page(u8)` (el que llama el menú lateral) y el `update` antes de dibujar. Números de página: los observados en el juego por
  Codex (`coordination/revision/PAGINAS_EDIFICIOS_NATIVAS_2026-11-12.md`). `ARGLOG=fichero` pasa `P3_ARGLOG`.
- Elevadas con `tools/lift.py --fpu-exit` y paridad por `oracle/snaptrace.py` (`SYNTH=1 SPRUNE=1`) sobre `09-town-hall`, `12-church`,
  `11-weaponsmith` y `town-hall-cotejo13`: `p3_core::lifted::bld_pages` (29: `tavern_content_window_open_page` `0x5CED00`, también
  sobre las entradas nativas de páginas de la taberna con el objeto real, y funciones de partida de iglesia, prestamista, Hansa,
  ayuntamiento, armero, asedio y granero), `bld_pages2` (`barn_content_window_update_wares_page` re-elevada: la de `real17` llamaba a
  `_ftol` como función externa) y `bld_pages3` (15 de la taberna, iglesia, prestamista, Hansa y ayuntamiento, incluida
  `tavern_content_window_update_news_page` `0x5D5970`). 2.700 trazas en total, prueba `lifted_real_match_x86`.
- Nativas nuevas: `ticks_to_date` `0x5311D0`, `_memcmp` `0x63B3E0` (−1/0/1 como el CRT), `get_office` `0x530400` (sin ella la
  producción del mercado `0x5DE960` recorría las oficinas con un puntero nulo para siempre), `0x4D9210` (préstamos concedidos, porte
  `task_lists`), `play_sound` `0x443150` sin audio. Biblioteca elevada: sprintf posicional `0x64D410`, `CObList::RemoveAt` `0x64E749`.
  `Runtime::texts` guarda los textos dibujados.
- Prueba: `p3-mvp/tests/ui_pages.rs` (9 pruebas, `P3_EXE`/`P3_CPR`/`P3_SAVES`): cada página abre sin funciones ausentes; oficina y
  taberna comprueban la página guardada (`+0xECC4`, `+0x1BF4`; la taberna cierra a −1 las páginas sin visitante). Cotejo con el
  cliente: «There are N sailors» de la taberna sale de la misma cifra que `Session::tavern_screen` (`get_hireable_sailors`), pero el
  original **la disfraza por encima de 50** (`0x5D03D1`: `trunc(factor·40 + 10,5)` con el factor aleatorio `+0x1CA0`); el cliente Tk
  muestra la real.

Pendiente:
- **Hecho (12/11, hilo nombres):** textos «(null)» resueltos. El banco de nombres `0x006DDAA0` lo llena `name_bank_load_name_files` `0x005121C0`
  (`NamenStaedte_eng.txt` y ficheros de pila/apellido por el CPR) y los punteros de nombre de cada comerciante (`+0xE8/+0xE4`)
  los refrescan `0x004F6420`/`0x004F6450` (la imagen de `p3-sim` los deja como índices etiquetados). `Runtime::seed_session` ejecuta ambos
  con el original elevado (`tools/ui_rt_names.py` → `ui_rt/names_lifted.rs`). Prueba `ui_pages::names_are_loaded` (Hansa 4: «Lars Müller
  from Cologne»); `P3_SAVE_FILE` permite otra partida que AUTO.pat.
- ~~Asedio página 1 y taberna página 9 en bucle~~ (resuelto 12/11 noche). Asedio 1 (`0x5F2200`): la llamada virtual `0x5F2384` (`0x4C7CD0`, `ret 4`) la dedujo el elevador con 0 argumentos y desplazaba el contador del bucle `[esp+0x10]`; se añadió el sitio a `tools/ui_rt_fix.sites` y se regeneró `fix_lifted.rs`. Taberna 9 (`0x5A74B0`): faltaban dos nativas, `find_letter` `0x4D7900` (porte `tavern_interaction::find_letter`) y `_strrchr` `0x63C920`; sin la primera la función ausente devolvía 0 y la cadena de cartas no terminaba. Pruebas `siege_pages` y `tavern_offer_letter_page` (11/11 en `ui_pages`). Lección: un bucle en Rust puro no agota `call_limit`; para localizarlo, sondear la memoria antes de la actualización y buscar funciones ausentes.
- ~~Hansa 3 (convoyes) y astillero 5 (nombre del barco) dividen entre cero~~ (13/11, hilo «Abrir páginas de convoyes y nombre del barco»):
  no piden ninguna selección previa. Hansa 3 recorre la lista que arma `0x0050A340` sobre el gestor naval `0x6DD7A0` y el astillero 5
  (`0x005F4C6E`) pide a `0x00504AC0` el primer barco del jugador en el puerto (`+0xCB8`) y lo guarda en `+0xC84`; ambos dividían entre cero
  (`0x005C5770` media de los barcos del convoy; `+0x14` del barco) porque la imagen de partida no sembraba el gestor. Ahora
  `Sim::original_memory_image` lo siembra desde la navegación viva (`[+4]` barcos 0x180, `[+8]` convoyes 0x3C, recuentos `+0xF4/+0xF6`,
  bandas piratas a direcciones sin sembrar). Nativas nuevas: `0x50A340`, `0x52A9E0`, `0x52A950`, `0x52AAD0`, `0x5182B0`, `0x5184F0`,
  `0x5185C0`; `0x4901C0` (iconos de artillería) se re-eleva con `--fpu-exit` (`tools/ui_rt_fix.py`, `FPU_FIRST`) para llevar `sqrt`/`_ftol` en
  línea. Prueba `ui_pages` (`hanse_pages`, `shipyard_pages`): filas «Convoy …» con «Sail: …», barco elegido con nombre y «New name». 
- Comprobar en el juego la edad del capitán de la taberna (sale 1345) y los precios del armero en su página (0).

### Niebla del mapa de la Hansa (12/11, noche, hilo «Portar la niebla del mapa de la Hansa»)

La página 5 de la ventana de la Hansa (`merchant_render_world_fog_map` `0x4F7490` → `0x534750`, `0x534920`, `0x534A40`) ya pinta la
niebla. Las «tres funciones gráficas» eran las del envoltorio SGL de texturas de memoria, que el enlazador no sustituía:
- `Runtime::sgl_call`: `sgl_CreateMemoryTexture` crea además el estado de `p3_render::sgl_tex::TexStore` (el mismo que se probó contra
  `ddraw_Dll.dll`) y un **espejo de los píxeles en la memoria emulada** (más 0x400 bytes para la paleta del primer bloqueo);
  `sgl_LockTexture(tex, info*, flags)` rellena `[píxeles, paleta, ancho, alto, paso]`; `sgl_UnlockTexture` relee el espejo;
  `sgl_ChangePalette(tex, tabla*)` aplica la clave de color a la tabla del llamante. Después de cada cambio `refresh_mem_texture`
  recalcula la imagen RGBA que usa el dibujo (8 bits con paleta o 32 bits BGRA). La niebla es una textura de 400×150 de 8 bits cuya
  paleta tiene alfa = índice (0xC8 = sin descubrir).
- Con centros descubiertos el original llama a `_strncmp` `0x642B30` (nativa nueva) y, dentro de `_output` `0x642234`, a `__aulldiv`
  `0x6461A0`/`__aullrem` `0x646210` (stdcall, 64 bits sin signo, devuelven EDX:EAX): el anfitrión solo devuelve EAX, así que
  `tools/ui_rt_lib.py` sustituye esas dos llamadas en `lib_lifted.rs` por la división en línea con EDX.
- Pruebas (`tests/ui_pages.rs`): `hanse_pages` incluye la página 5; `hanse_fog_discovered_centres` abre la página con la partida
  tal cual (el comerciante del jugador de `AUTO.pat` no tiene ningún centro descubierto: niebla uniforme 200 en 60.000 píxeles) y con todos los
  bits de `+0x4D8` puestos (histograma de 0 a 200: claros alrededor de los centros) y comprueba que el dibujo cambia.
  Capturas en `/mnt/project-files/interfaz-original/paginas/hanse5-niebla-*.png`.

### Astillero y armero: órdenes que se descartaban (14/11)

Con clics reales sobre la partida nueva (`p3-mvp/tests/ui_yard_actions.rs`, `ui_weapons_actions.rs`; barridos `ui_yard_sweep.rs`, `ui_weapons_sweep.rs`; utilidades en `tests/common`)
se comprobó que cada acción cambia el estado. Faltaban en el enlazador (la orden encolada se perdía): astillero `0x52A360` (encargar),
`0x52AFC0`, `0x52B0D0`, `0x52B110` (cancelar contrato), `0x52B670` (cotizar mejora), `0x508660` (start_repair, mejora), `0x4F9120`,
`0x53CCE0` (renombrar), `0x52ACD0` (reparar), `0x4D7B10` (aviso de carta), `0x64CB40`; armero `0x53CAF0` (comprar/vender armas) y
`0x538210` (cargar/descargar armas del barco ↔ oficina). Las elevadas están en `ship_orders_lifted.rs` (`tools/ui_rt_ship_orders.py`);
`0x53CCE0`, `0x52ACD0` y `0x4D7B10` usan los portes de `seh_frame_fns`.
Resuelto (14/11, e88af97): la tabla de «Improve ship» salía arriba a la izquierda porque 0x5F7380 cuenta un argumento de más en `get_y()` virtual (retorno 0x5F7A91). Antes:
«Hand weapons» sale desactivado en la partida nueva y no se probó; el oro de la barra superior no es `merchant+0` (incluye bienes).

### Panel lateral del barco: botón «Weapons» sobre el casco (14/11, nube de Claude)

El panel del barco propio enseñaba un botón «Weapons» (abre el comercio de armas) encima del dibujo del barco y, en la
vista de cubierta, encima del casco con sus ranuras de armas. Es el botón de comercio `menú+0x3724` (ctor `0x4C6910`,
uno de los cuatro de `0x48B3E0` con `+0x744/+0x82C/+0x914`): `ship_side_menu_update_mode_controls` (`0x492580`) lo
oculta al final con un salto de cola `pop ×4; mov dword [esp+4], 0; jmp [eax+0xCC]` (`set_visible(0)`, `0x4B21A0`,
bandera `+0x48`). El elevador emite ese salto como llamada sin argumentos; el enlazador no reconocía el marco (la última
escritura es el argumento, no un retorno) y daba al llamado un marco nuevo: `set_visible` leía basura (0x50, 0x78, 0xD4…)
y el botón quedaba visible. `dispatch_inner` reconoce ahora el salto de cola: llamada sin argumentos cuya última
escritura está justo encima de la pila de entrada del elevado en curso, con un retorno (de código o `RET_MAGIC`) debajo;
el llamado usa esa pila. Prueba `p3-mvp/tests/ui_ship_selection.rs`
(`own_ship_panel_hides_weapons_trade_button_and_draws_deck_slots`): partida nueva, Tab, botón de cubierta; el botón queda
oculto en las dos vistas y se dibujan los cañones y las ranuras vacías (`0x495210`/`0x48ED80`). Sin el cambio falla
(bandera 0x50/0x78). En el original el botón solo aparece en el modo comercio del panel (`0x492430`, al pulsar el
mercado u oficina con el barco elegido: «Town <-> Ship», «Ship <-> Office», «Town <-> Office», «Weapons»); `ui_weapons_actions`
llegaba a él desde la vista de cubierta gracias al fallo y ahora entra por ese modo. Batería UI de p3-mvp: 187 correctas. No acredita las ranuras de todos los tipos de barco ni el resto de saltos de cola del programa.

## Grupo lib2 (segunda pasada de biblioteca, 14/11)

`tools/ui_rt_grp.py lib2` (semillas `tools/ui_rt_grp_lib2.seeds`, llamadas indirectas en `tools/ui_rt_grp_lib2.nover`) eleva a `ui_rt/grp_lib2_lifted.rs` las
ausentes CRT/MFC que llaman las funciones de `lib` y su cierre transitivo (125 funciones, entre ellas `operations_switch_to_hotseat_player` `0x548860` y
`host_receive_from_clients` `0x54B670`, que fallaban con «pila incoherente» por sus llamadas indirectas). `ui_rt/grp_lib2_native.rs` trae las nativas `_getptd`
(datos de hilo persistentes: `errno`, `gmtime`), `_calloc_base`, `_tzset` (UTC) y los creadores MFC de estado por hilo. `menus.rs` implementa las
importaciones `Tls*`/`Local*`/`Global*` (monohilo) y `CFile::Remove` sobre la carpeta del juego (borrar partida de la pantalla de carga). El resultado por
dirección, con el motivo de las 29 innecesarias, está en `coordination/ENLAZADO_AUSENTES.md`; prueba: `p3-mvp/tests/ui_grp_lib2.rs`.


## Llamadas virtuales mal contadas: barrido de páginas (14/11)

`tests/ui_pages.rs::every_page_text_is_readable` abre todas las páginas de `ui_building_pages.tsv`, falla con textos
«(null)», vacíos o con bytes de control e imprime `INDIRECTAS`: llamadas indirectas cuyo número de argumentos deducido no
coincide con el `ret N` real (`Runtime::arg_mismatch`). Para corregir una: añadir `llamante retorno reales` a
`tools/ui_rt_fix.sites` y `python3 tools/ui_rt_fix.py --binary Patrician3.exe --only 0xLLAMANTE` (re-eleva solo esa función
y conserva el resto de `fix_lifted.rs`). Patrón típico: `push texto; call [vt+0x44/0x6c/0x70]` (get_width/get_y sin
argumentos) antes de `draw_text`. Corregidas: 0x5F7380, 0x5D14A0, 0x5CA210, 0x5B2050, 0x5F0680.

## Comercio desde el barco en la campaña: cuelgue al cerrar (14/11, nube de Claude)

Sergio (2560x1440): clic en el barco, grúa «Trade», comprar «Iron goods» y cerrar el comercio dejaba p3-juego sin responder.
Dos fallos: `0x489B80` llamaba a 0x0 por llamadas virtuales mal contadas (101 sitios en `tools/ui_rt_fix.sites`;
`tools/ui_rt_ship_orders.py` pasa ahora `--nover`, ea05c2f) y, con carga en el barco, `ship_side_menu_draw_cargo_list`
`0x48F510` (J08 `lift_pan3`) giraba ~560 millones de vueltas: tras cada `ui_render_text_at` (`0x4BB3E0`, envoltorio `sgl_*`
cdecl) el módulo J08 sumaba 12 a ESP además del `add esp` del llamante, y el puntero de fila `[esp+0x20]` era basura.
Quedaban 143 desapilados así en 17 módulos J08 (generados antes de la corrección de `callee_sig` del 13/11; `0x4BB3E0`,
`0x4BB760`, `0x4BBB10`, `0x4BB820`). `tools/j08_cdecl_pops.py EXE` los cuenta (debe dar 0) y `--fix` los retira. Prueba
`tests/ui_campaign_ship_trade.rs` (mismos clics; sin el arreglo se cuelga).

### Cierre de la partida de Sergio a los 55 minutos (14/11, 13:33)

`p3-juego.log`: `fotogramas 83574 resultado Some(0) abortado true`, sin pánico ni evento de Windows: el enlazador cortó por uno de
sus topes (profundidad > 400, llamadas por fotograma > 1e9 o montón > 256 MiB a 2560x1440). Faltaban (devolvían 0) `delete_this`
`0x611740` (9.609 veces desde `sea_update` 0x606CC0), `finish` `0x60D299` (96), `grouped_sprite_animator_update` `0x54EE30`, los
clics de los minimapas `0x46D2F0`/`0x59C4A0` y `0x65452D`: ahora en `grp_bat2`. En reposo el montón crece ~100 bytes por fotograma
(97→98 MiB en 4.000; `P3_HEAPLOG=N` lo imprime). El cliente anota ya el motivo del corte (`aborta: profundidad … llamadas …
montón …`), la hora de arranque y la de fin (segundos Unix) y la cima del montón.

## Batalla naval: nubes, pantalla y minimapa (15/11, nube de Claude)

Sergio: en batalla el minimapa salía negro, no había nubes, la entrada tardaba con la pantalla en negro y al salir se cerraba
el juego (`excepción de división del original`, código 101). Reproducción en la nube con `examples/ui_benchmark.rs city`:
partida nueva, convoy 31 con el barco 109 del jugador (sacado de la lista `0xE8` del gestor y de la lista libre de convoyes),
`engage_target` `0x50BC40(0x6DD7A0, 31)` con `P3_BENCH_CALL` y clic en «Steer manually». Nuevos diagnósticos del banco:
`P3_BENCH_CALL="F:función:ecx[:args];..."`, `P3_BENCH_MEMDUMP="F:dir|*puntero:largo:archivo"`, `P3_BENCH_PACK=idioma.pack`.

- **Nubes**: faltaban `town_drift_sprite_type14_init`/`_update` (`0x5803B0`, `0x580520`): grupo `nav` (`tools/ui_rt_grp_nav.seeds`).
- **Pantalla de 260×227**: `animation2d_draw` (`0x4AD540`) deja elegida la textura del minimapa como destino al acabar el
  fotograma y `SwapBuffer` presentaba esa textura (era la «pantalla negra» de la entrada; el registro de Sergio muestra un
  fotograma de 260x227). `SwapBuffer` y `Blt_RC` actúan ahora siempre sobre la pantalla (`Runtime::on_screen`).
- **Minimapa negro**: `sgl_StretchBitmapRect` no tenía sustituto (`Surface::stretch` sobre `p3_render::sgl::draw_scaled_rect`).
  Además el modo de mezcla es el de la textura **origen** (byte `+0x37`, `0x050147BA`), no el de `SetBlendMode` (que solo toca el
  objeto `0x05080B34`). `CreateMemoryTexture` (`0x05013800`) lo toma de `indicadores >> 24` o pone 1 si llevan el bit 0; si no,
  queda 0 (copia). Con el modo global 1 y el color constante `0xFF000000`, la rutina 3 multiplicaba el mar por negro.
  `Assets::own_mode` guarda ese modo para las texturas de memoria.
- **Salida**: huir (orden de retirada `0x6DCBE4`), dejar la vista (`leave_current_battle_view` `0x611FA0`) y acabar el combate
  vuelven a la carta sin fallo a 1024×768 y 1920×1080; no se reprodujo la división. Un fallo del original deja ahora en
  `p3-juego.log` la pila de funciones del original (`fallo del original «…» en [...]`, la última es la que falló).
- **Idioma**: con el paquete del PC, el aviso de batalla sale en español gracias a las tablas reescritas del hilo de idioma
  (`fbeae50`); «the convoy» → «Suma» es un emparejado erróneo de ese paquete.

### Sombra del humo de barcos dañados (17/11)

Bajo cada nube de humo de un barco dañado salía un círculo negro sólido. Es la sombra del humo: el pase de sombras `0x591C50`
(`sgl_SetBlendMode(2)`) llama a `0x579EC0`, que para los efectos de código 0x20-0x25 (0x24 = `_X_rauchanimpage`) pinta el propio
fotograma del humo desplazado con color `(opacidad·100/255) << 24` (negro, alfa ≤ 0x64). La rutina 7 (producto por el color) no
mira el alfa y dejaba negro todo texel con alfa. En pantalla, `Surface::blit` mezcla ahora ese producto con el destino por
alfa del texel × alfa del color (solo rutina 7, indicadores con alfa y color de alfa < 0xFF). Reproducción: `ui_benchmark city 250`
con `P3_BENCH_KEYS="60:38:25"` y `P3_BENCH_POKE` bajando `+0x18` (vida) de los barcos (`[0x6DD7A4]`, 0x180 bytes cada uno) en el
fotograma 30; prueba `smoke_shadow_darkens_by_color_alpha_on_screen`.


## Atacar a un pirata desde la carta (18/11, nube de Claude)

Sergio: al ir con su barco hacia un pirata en la carta náutica no empezaba la batalla naval. En el original se ataca con **un
solo barco propio seleccionado y clic derecho sobre el pirata** (uno de sus dos contactos `ship+0x50/+0x52`):
`nav_map_on_mouse_event` (`0x4499D0`, código -3) → `nav_map_handle_point_click` (`0x44FB40`) encola la operación 0x0E
(`op_set_ship_target` `0x538800`: fija `ship+0x3A` si el objetivo está a menos de `gestor+0xFC`); cada tick `update_pursuit`
(`0x50B030`) recalcula la ruta y, a menos de 20 unidades (400 al cuadrado), `start_sea_battle` (`0x50B250`) abre la batalla.
Navegar cerca no basta: sin la orden solo hay batalla si el pirata decide atacar (`pirate_check_attack` `0x515360`).

- **Causa**: `0x44FB40` (y el clic izquierdo que selecciona barcos en el mar) solo acepta el barco si el píxel bajo el cursor de
  su hoja `images/Schiffe/page_m*.tga` no es cero: bloquea la textura con `sgl_LockTexture` (`0x44FE26`) y lee el dword sin mirar
  si el bloqueo falló. `register_loaded_texture` solo sabía cargar AIM y BMP: con TGA el bloqueo devolvía 0 («SGL sin sustituto:
  sgl_LockTexture (textura no creada en memoria)» en `p3-juego.log`), el original leía un puntero viejo y nunca acertaba.
- **Arreglo** (`ui_rt::tga_as_aim`): los TGA se bloquean como los deja `AIM_CONVERT_MEMFILE` + `AIM_CONVERT_RAW`: formato 4,
  B, G, R, A de arriba abajo. Comprobado ejecutando `AIM.dll` en Unicorn sobre `page_mschnigge01.tga` (256×256×32, origen
  abajo): los 65.536 píxeles iguales, también los transparentes con color (38.890 dwords a cero en los dos).
- **Pruebas**: `tests/ui_chart_ship_click.rs` (partida nueva: clic izquierdo selecciona el barco, la esquina transparente no
  acierta, clic derecho encola 0x0E y la cola fija `+0x3A`; falla sin el arreglo). Con la partida `Kam/AUTO` de Sergio,
  `examples/pirate_probe.rs` (`P3_F8`, `P3_PLACE`, `P3_CLICK`, `P3_RCLICK` con `P3_NOSEL=1`) hace los clics reales y el barco
  pasa a 0x14 con el diálogo «Sea battle: Your cog … is attacking the snaikka …». Un pirata más rápido que el barco propio se
  escapa (la distancia no baja de 20): la persecución es el código x86 elevado de `update_pursuit`.
