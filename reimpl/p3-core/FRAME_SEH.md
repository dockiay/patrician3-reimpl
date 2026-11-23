# Funciones con marco SEH o locales de pila pasados por dirección

Forma común (11/11/2026) para portar con paridad x86 las funciones que montan un marco SEH de MSVC (`push -1; push manejador;
push fs:[0]; mov fs:[0], esp`) o que pasan a otras funciones la dirección de una variable local (mensajes `0x4D6530`, órdenes de la cola
`0x54AA70`, búferes de nombres, salidas de `0x5311D0`…).

## Cómo funciona

- **ESP fijo.** El oráculo (`oracle/hookcase.py`) llama siempre con el mismo ESP de entrada y lo guarda en el caso (`esp`). El porte
  recibe ese `esp` y pone sus locales en `esp - desplazamiento` con `p3_core::frame::Frame::at`, igual que el original. Las direcciones
  de los locales coinciden, así que se comparan como cualquier argumento.
- **Zona de locales comparada.** El caso siembra con bytes aleatorios solo la zona de locales del marco (sin los registros guardados ni
  la dirección de retorno) y la compara después de la llamada. Los argumentos de la pila están en `esp+4..` (`Frame::arg`), y la
  prueba los siembra; algunas funciones usan su propio argumento como local (`0x517260`, `0x50CC90`).
- **SEH.** `fs:[0]` vive en la página cero (base de FS 0). `Frame::seh_enter/seh_state/seh_leave` escriben el registro (enlace previo,
  manejador, estado de destrucción en `esp-4`) en los mismos puntos que el original. Los manejadores no se ejecutan nunca.
- **Contenido de los locales.** `Harness.hook(..., peek=[(argumento o 'ecx', palabras)])` añade al registro de la llamada las palabras
  que hay en la dirección pasada; el porte hace lo mismo con `p3_core::frame::peek`. Así se comprueba que el local estaba relleno igual
  en el momento de cada llamada.
- **Salidas por puntero.** Un gancho puede devolver `(valor, [(dirección, valor, tamaño)])` con tamaño 1, 2 o 4; el oráculo escribe esos
  bytes y la prueba los aplica en el mismo orden.
- **CRT real.** `memcpy` 0x63A790, `memset` 0x639F20 y el constructor 0x535740 corren del original y no se enganchan; `malloc` 0x64F7B9,
  `free` 0x64F7E2 y `rand` 0x639C03 sí.

Trampas: registros parciales (`movzx cx, ...; push ecx` o `mov bp, ...`) dejan basura en los 16 bits altos; se enmascaran en el gancho
(`mask`). Listas enlazadas de los casos sin ciclos (el siguiente siempre mayor), porque el original no se protege de ellos.

## Funciones portadas

Oráculo `oracle/x86_seh_frame.py`, fixture `tests/fixtures/seh_frame.json`, prueba `tests/x86_seh_frame.rs`, código
`src/economy/seh_frame_fns.rs` (filas en `reimpl/functions.tsv`).

0x5338A0 trade_offer_init_and_escrow, 0x533510 setup_hamburg_luebeck_overland_route, 0x538DE0 op_edit_ship_route_waypoints,
0x53DA30 op_create_trade_offer, 0x5424D0 op_add_town_supply_entry, 0x4DBB00 st_town_mayor_election, 0x4DDE70 st_captain_return_to_pool,
0x543810 op_reset_world_state_on_start (con FPU byte→f32), 0x53DA90 op_schedule_wedding, 0x53EAC0 op_pay_matchmaker_fee,
0x635B80 op_create_siege_supply_request, 0x51E780 reset_buildings, 0x517260 get_town_index_at_point, 0x543570
op_append_indexed_text_chunk, 0x4E2634 st_captain_pool_refill, 0x4DE790 st_ai_build_ship, 0x50CC90 detach_town_outrigger_for_battle,
0x53CCE0 op_rename_ship, 0x542380 op_request_create_merchant_from_lobby, 0x53A810 op_change_hometown, 0x505B60
unlink_ship_from_at_sea_list, 0x5062F0 create_convoy_for_ship.

Después (11/11, segunda parte): 0x534750 map_discovery_construct, 0x531BD0 build_town_distance_matrix, 0x52BBE0
trade_center_deserialize, 0x530470 create_merchant, game_world_construct/destruct, operations_construct/destroy/destructor, 0x544830
custom_map_construct, 0x4E94A4 st_unfreeze_port, 0x4E8684 st_matchmaker_candidate_dossier, 0x4E2144 st_update_shipyard_experience
(x87), 0x4DC1F0 st_hanse_diet_vote, 0x542860 op_create_initial_ai_merchants_all_towns, 0x542A30 op_create_ai_merchants_batch (x87).

Segunda tanda (oráculo `oracle/x86_seh_frame2.py`, fixture `seh_frame2.json`, `run2` en la prueba): 0x541D70 op_create_player_merchant,
0x516520 ship_load_core, 0x5462F0 town_add_facility_supply_entry, 0x5419C0 op_append_lobby_player_name_chunk, 0x4D6B40
message_pool_grow, 0x519D90 ship_send_lost_letter_to_owner, 0x533F20 trade_offer_format_description, 0x544730
load_received_game_statistics_block, 0x549940 operations_load_custom_map.

Más formas útiles:
- `Frame::push(m, off, valor)`: hueco por debajo de los locales que el original rellena con un `push` (CString por valor que construye
  la función llamada). Declara los bytes con `w8_seed` y no entra en la zona comparada.
- x87: el CRT deja la palabra de control en 0x27F (53 bits), así que `fild/fadd/fdiv/fstp` equivalen a `f64` redondeado a `f32` al
  guardar.
- Llamadas variádicas (`sprintf` 0x64DF80): `hook(addr, n, ...)` acepta `n` como función de la dirección de retorno, así cada sitio registra todos sus argumentos.
- Funciones de Windows por la IAT (Sleep 0x66A2BC): la entrada de la IAT es parte del caso y apunta a una dirección falsa enganchada.

Después en la segunda tanda: 0x53FEA0 op_add_captain_candidate, 0x541840 op_set_lobby_player_profile, 0x5110E0
script_blob_pool_register_group_scripts, 0x5116F0 script_blob_pool_load_script_blob, 0x544590 load_received_savegame_blob, 0x549190
handle_player_left_game, 0x4D7B10 message_notice_for_new_message, 0x52ACD0 op_repair_ship, 0x51E440 load_town_ini_entry, 0x5448A0
custom_map_load_from_archive, 0x544B80 custom_map_save_to_file (marco EBP; los `catch` de CFile no se modelan), 0x547BE0
savegame_read_header, 0x5483D0 operations_show_ship_event_notification, 0x548860 operations_switch_to_hotseat_player. La segunda fixture
ocupa ~21 MB: no añadir más.

Tercera tanda (oráculo `oracle/x86_seh_frame3.py`, fixture `seh_frame3.json` ~14 MB, `run3`): 0x51B520 town_deserialize, 0x504230
ships_deserialize, 0x52FB60 game_world_deserialize (iteradores de vector de MSVC 0x639E9C/0x639DA8 enganchados con `cd=1`: stdcall
sin registrar ECX), 0x528E60 run_mayor_election.

Portadas el 11/11 en el PC de Sergio: script_blob_pool_load_for_game_mode 0x511270 y trade_center_handle_convoy_contract 0x52C6C0.
**Aclaración posterior:** 0x52C6C0 ya está integrada y tiene paridad x86 para el corpus descrito; la mención histórica a «pendiente»
en otro checkpoint no exige otro porte. Revisión independiente: 240 contratos base y 240 nuevos correctos, 706/745 instrucciones
visitadas, sin excepciones ni juego. [Evidencia y límites](../../research/systems/revision-reimpl/CONTRATO_CONVOY_52C6C0_2026-11-11.md).
`oracle/x86_seh_pending.py`, fixture `seh_pending.json`, prueba `x86_seh_pending.rs`: 144 casos de carga de guiones y 240 de contratos.
El registro de cobertura exige las ramas de asignación de variables, alta de tarea, retirada de guion, creación de contrato, x87,
venta, compra y eliminación del contrato. Se comparan las regiones, los locales y argumentos modificados, las llamadas externas
(incluido el contenido de CString y del contrato pasado por dirección), FS y el retorno observable (AL para el contrato).

El generador reproducible `tools/lift_seh_pending.py` usa el modelo de registros de `p3_core::x86rt` y firmas explícitas verificadas
por profundidad de pila. Código: `lifted/seh_pending.rs`, exportado por `economy::seh_pending`. Mantiene el ESP fijo como los portes
con `Frame`; los bytes transitorios por debajo de los locales no forman parte de la imagen comparada. El CRT memset,
0x63A6A0/0x63A6E0, 0x63C610 y __ftol se ejecutan originales en el oráculo y se modelan nativamente en Rust (retornos EDX:EAX y x87).
Las llamadas de recursos, asignadores y otros sistemas siguen interceptadas. No se prueban desenrollado SEH, excepciones, programas
malformados ni la integración de estas dos funciones en una sesión completa; no se ha abierto el juego.
Las st_* de C10 (0x4DDF40, 0x4E4A44, 0x4E5664, 0x4E63E4, 0x4EAFE4) tienen módulo tipado propio; 0x52BFE0 es un fragmento.

## Funciones sueltas con el elevador x86 (11/11/2026, tarde)

Las seis funciones que el generador genérico de `oracle/j08_auto.py` no lograba ejecutar se portaron con el elevador (`oracle/lift_x86.py`,
que ya monta el marco SEH y los locales de pila) y casos a medida en `oracle/j08_loose.py` (`patch(case, g, fn, info)` al final de `build_case`):

| Función | Módulo Rust (fixture) | Casos | Cobertura de instrucciones |
|---|---|---|---|
| 0x605420 `start` (arranque de batalla naval, SEH, 6 KB) | `lift_sim13` (`j08_lift_sim13.json`) | 118 | 88 % |
| 0x52BFE0 / 0x52C019 `trade_center_trade_with_convoy` | `lift_sim12` (`j08_lift_sim12.json`) | 150 | 89 % |
| 0x5472F0 `operations_save_game` (con la cola 0x547566) | `lift_sim12` | 110 | 100 % salvo los 4 manejadores SEH |
| 0x534A40 `map_discovery_render_fog_surface` (DirectDraw) | `lift_sim13` | 47 | 94 % |
| 0x51AC60 `letter_render_map_fragment_bitmap` (DirectDraw) | `lift_sim13` | 36 | 91 % |
| 0x54AAE0 `start_lan_game_search` (CreateThread, Sleep, sockets) | `lift_sim13` | 150 | 100 % salvo el fallo de malloc |

- Se regeneran con `J08_REPS=150 python3 oracle/x86_j08.py --binary EXE --group lift_sim12 --group lift_sim13 [--check]`; los lifts con
  `oracle/lift_x86.py --name sim13 --addr 0x605420 --addr 0x534A40 --addr 0x51AC60 --addr 0x54AAE0 --overrides oracle/j08_lift_sim13_overrides.json`
  y `--name sim12 --addr 0x52BFE0 --addr 0x5472F0 --size 0x52BFE0=0x6E0 --size 0x5472F0=0x3F0`.
- `lift_sim13` va primero en el despacho de `p3-render/src/j08/mod.rs`: oculta los portes antiguos de sim3/sim4/sim5 de las mismas funciones,
  que tenían mal la firma de las llamadas.
- **DirectDraw simulado**: `0x4BB2A0` (crear superficie) devuelve un objeto de la arena y `0x4BBB00` (bloqueo) escribe en la descripción
  que recibe como 2.º argumento: píxeles (+0), paleta (+4), ancho (+8), alto (+0xC) y paso en bytes (+0x10). Las escrituras en los píxeles
  entran en el fixture como cualquier otra. Estas funciones son **cdecl** con limpieza acumulada (`add esp, N` tras varias llamadas):
  el elevador deduce `ret N` del destino y se equivoca; hay que forzar `[nlog, 0]` por dirección de llamada
  (`j08_lift_sim13_overrides.json`, clave = función → dirección de la llamada).
- **CreateThread**: `0x654296` (`_beginthreadex`) es una llamada directa más: se intercepta y se registra, el hilo nunca arranca. `Sleep`
  (IAT 0x66A2BC) es un gancho que pone a cero `this+0x38`, que es lo que espera el bucle.
- **Trampa**: `ptr_fields`/`ptr_args` inferidos por el elevador pueden pisar la vtable de `this` (en 0x605420 inferió campos en +2, +4…);
  el parche vuelve a escribirla. `case.put(dir, …)` con la clave de una región ya sembrada la sustituye entera: para tocar unas
  palabras usar `put32`/`put16`.
- 0x547566 y 0x52C019 no son funciones: son interiores de 0x5472F0 y 0x52BFE0 (misma función con dos entradas), y quedan registradas con el
  mismo porte.
