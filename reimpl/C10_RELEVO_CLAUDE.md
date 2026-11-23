# C10 continuada por Claude (10/11/2026, nube de Claude, `claude-c10`)

Codex delegó C10 en Claude. Este relevo cierra los tres pendientes que dejó el coordinador:
la expedición 0x23 en `clock`, la lista de bajas de la caza hanseática y los avistamientos de la carta.

## Expedición 0x23 y flota viva (fps-cache/AUTO en `clock`)

La fase 1 de la expedición (`factory_preview`) construía los barcos y el convoy de la expedición en las
vistas `expedition_factory` / `expedition_ship_images` / `expedition_convoy_images`, pero la navegación
viva (`LiveVoyage`) no los veía: el barco 122 quedaba libre en la flota viva con su ciudad en
`cargo_ships` y el convoy de la expedición paraba con «barco de muralla desincronizado».

- `p3-sim/src/factory_nav.rs::publish_factory`: tras la fase 1, copia a la flota viva (imagen y base) los
  barcos, convoyes y capitanes que cambió la fábrica, las palabras del gestor `+0xE0..+0xF0` y los
  recuentos `+0xF2/+0xF4/+0xF6`, y rehace las listas.
- `publish_images`: tras `dispatch_scheduled_tasks_fleet`, publica en la imagen viva los bytes de barco y
  convoy que cambiaron las fases 5, 7 y 8–10 (antes de `publish_navigation_delta`).

Resultado: fps-cache/AUTO completa 365 días en `clock` y en `paused` sin `task_errors`.

## Lista de bajas de la caza (`tarea 0x34 + 8`)

Nodos de 8 bytes `{+0 hijo, +4 clase, +5 marca, +6 texto}`. Clases: 0 baja al volver, 1 capturado
(`0x672855`), 2 baja del convoy en el mar (`0x672856`), 3 captor (`0x672857`).

| Pieza | Dirección | Rust | Evidencia |
|---|---|---|---|
| Escritor del guardado | `0x004F39E0` | `p3_save::tail::encode_hunt_losses` | `p3-save/tests/c10_hunt_loss_list_x86.rs` (600 listas) |
| Lector de la carga | `0x004F3A40` | `p3_save::tail::decode_hunt_losses` | mismo (600 listas + 600 flujos arbitrarios) |
| Retorno con lista previa | `0x0050BFD0` | `battle_manager::prepend_hunt_losses` | `p3-sim/tests/c10_hunt_loss_chain_x86.rs` (192 casos, 148 con nodos previos; guardado y recarga x86) |
| Baja de convoy en el mar | `0x00502285..0x00502331` | `ConvoyHost::convoy_ship_finished_with_world` → `Sim::apply_navigation_convoy_losses` | encadenado en p3-sim |

El lector es con pérdida (`+6 = byte_de_signo * 0x101`); Rust lo reproduce. El modelo vivo es
`Host::hunt_losses` (ranura → nodos, cabeza primero); `update_scheduled_tasks_with_lists` lo escribe y
exige una lista no vacía exactamente donde la tarea tiene `+8 != 0` y fase `>= 7`. La batalla
(`DestroyWorld`), la captura (`PirateCaptureServices::hunt_loss`) y la llegada (`c08_hunt::arrive_hunt`)
leen y escriben el mismo modelo. Guardado real: `AUTO.pat` ida y vuelta con rechazo de lista ausente.

`hunt_convoys`/`hunt_tasks` (`c10_battle_capture::hunt_tasks_of`) aplican ya 0x00503B00 completo: tarea con ranura dentro de la capacidad, clase 0x34, fase >= 7, `+0x16` = convoy **y** `convoy+2` = esa ranura (prueba `hunt_tests`; desensamblado de 0x503AC0/0x503B00 comprobado).

## Avistamientos (`0x0050D2B0`, `0x0050DC50`)

`p3_core::ships::spotting` porta las dos funciones con paridad x86 (`oracle/x86_c10_spotting.py`,
`p3-core/tests/c10_spotting.rs`: 800 barcos y 400 pasadas de humanos; bytes cambiados y avisos de interfaz
0x452F60/0x46F340/0x459790 en orden).

En el original la pasada solo corre con carta y ventana (`[0x6CBDD8]`, `[0x6CC940]`), dos veces por paquete
de reloj `0xC4` de `execute_operations` (`0x00546A43`, `0x00546A87`), y escribe `ship+0x50/+0x52` en la
tabla guardada. La simulación sin pantalla (y el oráculo x86 de partida entera) no la ejecuta, igual que el
original sin interfaz. La carta (`p3_mvp::navmap::spot_frame`) la ejecuta una vez por fotograma sobre la
copia que dibuja, con las ranuras guardadas en el estado de la carta; sustituye al enganche provisional
`lost_sightings`. Añade los sonidos de clase 2 a la escena del mapa y devuelve `sightings.lost/pulses`.
**Supuestos**: una pasada por fotograma (no dos por tick), partida de un jugador.

El pulso `0x0046F340` es del **minimapa** (`this+0xC4` lista de 12 bytes, escalas 9,23077/7,92952). Portado en
`p3_core::ships::spotting::MinimapPulses` (alta con deduplicación; dibujo 0x46E004: rombo de radio 2 en gris = nivel,
nivel -10 por repintado; limpieza 0x46E644 con nivel < 0, 26 repintados) y dibujado por la carta como minimapa
en la esquina inferior derecha (`Canvas::minimap_inset`, `sightings.minimap_pulses` en el JSON). **No** contrastado
con x86 (la rutina de dibujo usa el contexto gráfico): solo aritmética de alta/envejecimiento por pruebas unitarias;
el orden relativo dibujo/limpieza y la señal `[0x6DFC1C] != 1` son supuestos.

## Pruebas

- Nuevas: `p3-core/tests/c10_spotting.rs`, `p3-save/tests/c10_hunt_loss_list_x86.rs`, `p3-sim/tests/c10_hunt_loss_chain_x86.rs`
  y `navmap::tests::spot_frame_*`, todas correctas.
- `cargo test --release -p p3-save -p p3-sim -p p3-mvp`: 26 pruebas fallan, exactamente las mismas que en `375153a`
  antes del relevo (`c09_reference` por huellas de ConvoyAvista con corpus de `a949c9b`; `live_chain` historial día 1;
  `c10_astiller_30_days`, `c10_founded_transaction`, `persist_all`, `pumped_real_save`, `real_saves`, `resume_crt` por
  encuentro pirata, guiones o flota sin configurar en esas pruebas). No son de este relevo.
- 365 días, 9 guardados × `paused`/`clock` (`c10 ... --human-battles-auto --investigation-local`): 15 de 18 completan sin
  `task_errors`; paran TestAstiller `paused` ×2 (asedio 6, bytes de pila) y ConvoyPrueba `clock` (tarea 0x04, residuo de pila).

## Pruebas de p3-sim con datos (10/11, 15 fallos arreglados)

Los 15 fallos no eran divergencias sino pruebas de `Sim` desnudo sin cablear (flota viva, banco de guiones, estado de
fiestas, textos del EXE, regiones de asedio). Ahora usan la misma configuración que `c10 ... --celebration-state 0:255
--investigation-local --exe` (`resume_saved_fleet`, `configure_hanse_fleet_archives`, textos, `siege_headless_march`):
`real_saves` 46/46, `persist_all` 5/5, `pumped_real_save` 3/3, `resume_crt` 2/2, `c10_founded_transaction` 1/1. Asedio 10
(«regiones de asedio ausentes») era solo esa falta de configuración. `cargo test --release -p p3-sim`: 288 binarios, 0 fallos.

**Frontera que queda**: el encuentro pirata sin batalla headless (`navegación: encuentro pirata pendiente de conectar`) sigue
sin resolverse en `Sim` desnudo (lo resuelve la sesión de p3-mvp: `c10_corpus`, `c10_headless`). Las pruebas que alcanzan
esa frontera la reconocen y se detienen o comparan las dos ejecuciones hasta ese punto (`real_saves` determinismo); **por eso
`c10_astiller_30_days` ya no cubre 30 días completos** en `Sim` desnudo. `resume_crt` compara el resumen sin el hash y las
ciudades fuera de `+0x4F0..+0x670` (la primera recarga recalcula umbrales de precio, como en `persist_all.rs::same`).
`p3-mvp` tests: `screens_lender` no compila en `main` (métodos de otro hilo: `church_feed_poor`, `town_hall_treasury`,
`tavern_screen`); no es de este relevo.
