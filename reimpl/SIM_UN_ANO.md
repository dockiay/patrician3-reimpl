# Simulación de un año: 8 guardados × 2 ritmos (10/11/2026, nube de Claude)

`c10 GUARDADO p2arch0_eng.cpr 365 4660 --celebration-state 0:0 --exe Patrician3.exe
--human-battles-auto --investigation-local --battle-pacing paused|clock`, main ad12e97.
Guardados en `games/*/save/Ein` (fuera de Git). `visibility/AUTO.pat` se repitió después desde la
copia sin enlace duro `coordination-runtime/revision-autopat/AUTO.pat` del PC (mismo tamaño y fecha).

| Guardado | `paused` | `clock` |
|---|---|---|
| visibility/AUTO | año completo | año completo |
| fps-cache/AUTO | año completo | **año completo** (antes: expedición 0x23, C10; corregido en 01e0ea1) |
| fps-cache/TestAstiller | para: asedio 6, bytes de pila (año completo con `--siege-stack-tail 0,0,0`) | año completo |
| visibility/TestAstiller | igual que el anterior | año completo |
| lab-english/TEST25_ConvoyMar | **año completo** (antes: subasta 0x0A) | año completo |
| visibility/ConvoyMar | **año completo** (antes: subasta 0x0A) | año completo |
| visibility/ConvoyHecho | **año completo** (antes: subasta 0x0A) | **año completo** (antes: subasta 0x0A) |
| visibility/ConvoyAvista | año completo | año completo |
| visibility/ConvoyPrueba | año completo | para: tarea 0x04, ranura de pila de 0x004F5A20 (año completo con `--ai-work-residue 1`) |

Sin hipótesis: 15 de 18 recorridos completan el año (repetido el 10/11 con 01e0ea1, nube de Claude:
`task_errors` vacío en los 15). Con las dos hipótesis de pila explícitas, 18 de 18. Ya no queda ninguno de C10;
la expedición 0x23 publica sus barcos en la flota viva (`reimpl/C10_RELEVO_CLAUDE.md`).

Repetido el 10/11 por la tarde con las cuatro llamadas de guion conectadas (717d958, sin el S07 de fac130e):
14 de 18 recorridos completan el año, los mismos que antes, y los cuatro que paran lo hacen en el mismo día y por
la misma causa (obra privada en fps-cache/AUTO `clock`, asedio 6 en los dos TestAstiller `paused`, tarea 0x04 en
ConvoyPrueba `clock`). Ningún guion de los guardados llega a `0x67`, `0x10`, `0xE5`, `0xEC` ni `0xED`.

## Lo que se ha conectado

- Adjudicación del lote naval de la subasta 0x0A (fase 7, `0x004E31D6`) y
  `ShipManager::transfer_ship` (`0x00509250`) con comprador: `p3_core::economy::auction_ship::award_lot_ship`,
  600 casos iguales a `st_auction` x86 y 5 variantes encadenadas en p3-sim contra el despachador
  original con guardado y recarga (`p3-sim/tests/c10_auction_award_ship_live_x86.rs`).

- Barco propio a subasta (operación 0x36, `0x004DA510`): `p3_core::economy::auction_register`, 1.500 casos
  iguales al x86 (`p3-core/tests/c10_auction_register_x86.rs`), y en p3-sim (`Sim::register_ship_for_auction`).
  Es el único origen de un lote con dueño (reventa): la fase 0 crea siempre barcos sin dueño.
- Reventa en la adjudicación: poda `0x004F5900` de los grupos del vendedor y baja `0x004D4DD0` de la
  parada `+0x132` (depósito S05). `resale_registration_and_award_match_original` encadena 0x36 y fase 7 en
  p3-sim contra el despachador original con poda y baja nativas (4 variantes: con y sin parada,
  vendedor humano o IA, el vendedor se recompra) y recarga el guardado. Ningún guardado real tiene
  paradas; la variante con parada usa un depósito sintético de 3 registros.
- Lote en convoy y orden de viaje `0x00535760` (operación 0, solo para un lote navegando) siguen
  rechazados: `0x004DA510` exige barco en puerto y sin convoy y la fase 0 lo crea así. No está probado que
  ningún camino saque de puerto o meta en un convoy un barco en estado 0x0C; si ocurriera, la
  adjudicación se detiene con error en vez de divergir. La operación 0 tampoco tiene ejecutor en p3-sim.

## Paradas que quedan y su causa

- **AUTO `clock`, obra privada** (`p3-sim/src/wall_work.rs`): el barco 122 está libre (estado 0xFF) en la
  flota viva y `cargo_ships` conserva ciudad 12. Tick 125790375, justo tras la expedición hanseática
  0x23/0x24/0x34 (tick 125790208), que con navegación viva no publica sus barcos en `nav.snapshot`
  (`factory_preview`). Es de C10 (Codex); no se ha tocado.
- **ConvoyPrueba `clock`, tarea 0x04**: `0x004F5A20` compara primero con `best_a`, la doble palabra en
  `E-0x28` (E = ESP a la entrada) que nunca escribe. Llega por despachador `0x004D85C0` → `0x004DFC94` →
  `0x004F4940` → `0x004F4E50` → `0x004F5A20`; con oro ≥ −50.000 (aquí −7.546) pasa antes por `rand`
  (`0x00639C03` → `_getptd`, kernel32), cuyos marcos no llegan a esa profundidad. El valor es, por tanto,
  el residuo de lo que el despachador ejecutó antes en ese tick a esa profundidad (otras tareas, y
  `0x004F4F80`/`0x004F5080`/`0x004F5130` cada 4 y 31 turnos): no se deduce del guardado. Con residuo 0
  no se encola la obra; con cualquier otro de los probados (1, 2, 7, 0x80000000, 0xFFFFFFFF) sí.
  `--ai-work-residue` fija el valor para recorridos exploratorios.
  **Medición en el original (10/11, hilo «Llevar el oráculo al año completo»):** con `P3_X86_WATCH='4f5a20;0'` y
  `P3_X86_WATCH_SLOTS='-0x28,-0x2c'` (solo observación, `x86_c10_real_save_clock.py`) el original emulado recorre
  ConvoyPrueba 365 días con el ritmo del oráculo (`--resolve-ai-battles --human-battles-auto`, CRT 123) **sin entrar
  nunca** en `0x004F5A20` (0 llamadas en 365 días), y Rust en `paused` tampoco la necesita: el año coincide sin
  residuo. La parada solo aparece en `clock` (día 168 con CRT 4660, 8/1/1346), cuya línea temporal el oráculo no
  reproduce (la cadencia mundo/batalla depende del reloj de pared). La ranura es un local de `0x004F5A20` (`E-0x28`,
  `E-0x2c`; `E` = ESP a la entrada) por debajo del marco de `0x004F4E50`: el residuo viene de las llamadas hermanas
  previas de `0x004F4940` y sus profundidades, no del guardado. Con el residuo en 0 la primera comparación
  (`esi*[esp+0x20] < [esp+0x2c]*ebp`) nunca mejora el candidato y no se encola obra; con cualquier residuo grande el
  primer candidato gana. No hay valor que acreditar con el x86 mientras no exista un oráculo con cadencia de reloj.
  Se mantiene la parada explícita en `clock` (no se inventa un valor); `--ai-work-residue` sigue siendo la
  salida declarada para recorridos exploratorios.
- **TestAstiller `paused`, asedio 6**: bytes de pila de la demanda del asedio (C08); `--siege-stack-tail`.
- **No vistos en la nube** (sí en el PC, `C07_HEADLESS_HUMAN_BATTLES.md`). En Linux no aparecen en ninguno
  de los 9 guardados en `clock`, ni con main actual (ac29a21) ni con el código anterior a la pasada del PC
  (60e88a4: fps-cache/AUTO para en la obra privada, visibility/AUTO en la subasta, ConvoyPrueba en la
  0x04 y ConvoyAvista completa). Los guardados tienen el mismo sha256 que el corpus del PC
  (`coordination-runtime/c10-corpus-clock.json`). Si solo ocurren en Windows, la causa probable es una
  función de libm que difiere entre plataformas (`acos` en `route/planner.rs`, `ln` en `ai/bids.rs`).
  En Windows (PC de Sergio, main ac29a21, `c10.exe` sha256 B205D78C…) tampoco: 7 de 9 completan el año
  en `clock` y los otros dos paran igual que en Linux (fps-cache/AUTO obra privada, ConvoyPrueba 0x04).
  Las paradas del PC eran de código intermedio ya superado; quedan como errores defensivos.
  - «rayo sin corte: pila sin inicializar» (`p3-core/src/ships/collide.rs`, choque de cascos
    `0x00608F70`): las aristas de los dos rayos (`0x0060ADBF`) se guardan en `B+0x2C`/`B+0x34` del marco
    propio y solo se escriben si hay corte; si no, se usa el residuo de la invocación anterior a esa
    profundidad (directa desde `0x006088F0`/`0x0060891B` o vía `0x00609BA0`). Solo importa si vale 0, 1
    u otro.
  - Alta meteorológica 0x78 «ranura libre de fábrica desincronizada» (`p3-sim/src/weather_spawn.rs`):
    la ranura que reutiliza el alta difiere entre `factory.ships` y la flota viva; otra desalineación de
    las copias navales (se resuelve con una sola copia de la flota).
