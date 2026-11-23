# Oráculo x86 en modo `clock` (11/11/2026)

El oráculo de partida entera (`oracle/whole_game_corpus.py`, ver `PARTIDA_ENTERA.md`) resolvía las batallas de IA
con el mundo detenido (`paused`). Ahora el original emulado también sigue el ritmo `clock` de Rust
(`C07_AI_BATTLE_CLOCK.md`): mundo y batalla intercalados con fotogramas de 40 ms.

## Qué se añadió

- `oracle/c10_real_battle_driver.py`: `Driver.enable_clock(ms, divisor, hotseat)` y `_run_clock`. Replica
  `headless::run_with_clock`: bombeo `0x00612DB0` (`BattleTimePump::schedule`, con el truncado a `u16` y el tope
  `0x5460`), 0x94 con `update(ticks)` real (`0x00606CC0`), un tick mundial cada `ms` de reloj de pared (el
  mismo `0x530E80` y la misma cola `0x546870` que el resto del oráculo), y los tres finales de Rust:
  `destruida`, `fin_de_intervalo_sin_reloj` y `otro_encuentro_sin_reloj` (la batalla viva se termina sin reloj y la
  nueva se procesa después). `ms` y el divisor salen de la cabecera del guardado (`payload[0x50..0x58]`).
- `oracle/x86_c10_real_save_clock.py`: `--battle-pacing paused|clock`, `--crt N` (semilla tras la carga, 123 por
  defecto), `--probe-ticks DESDE:HASTA:PASO` (muestras extra para acotar un día distinto) e informe
  `ai_work_slots`: en cada entrada a `0x004F5A20` anota la ranura de pila `E-0x28/-0x24/-0x2c` (solo observa).
  Adaptador nuevo: `0x5256F0` (traspaso de oficina entre comerciantes, desde la subasta `0x4E3664`) lee
  `[0x6E51AC]+0xC324` (ciudad mostrada en la escena, ver `SceneRoutes::shown_town`) solo para repintar; sin escena se
  le da una ciudad mostrada imposible (-1) y toma la rama sin repintado (hook en `0x525912`).
- `p3-mvp/examples/c10_snapshots.rs`: `--battle-pacing clock`, `--crt`, `--ai-work-residue N`,
  `--ai-work-residues INFORME_X86.json` (ranura de `0x4F5A20` por tick, medida en el original) y `--probe-ticks`.
- `p3-sim`: `ai_work_residue_by_tick` (prioridad sobre `ai_work_residue`); los demás recorridos no cambian.
- `oracle/whole_game_corpus.py`: `--battle-pacing clock` y `--residue-from-original`.

```bash
python3 whole_game_corpus.py ... --days 365 --human-battles-auto --battle-pacing clock \
  --siege-tails-from-original --residue-from-original --output informe.json
```

## Resultado: la tarea 0x04 de ConvoyPrueba

El motivo de la parada en `clock` era que el oráculo no tenía línea temporal de reloj. Con ella, el original emulado
entra **una sola vez** en `0x004F5A20` en el año (tick 125769805, el mismo en que Rust se detenía) y la ranura de
pila vale **0** (`E-0x28`; `E-0x24` = 5338523 no interviene). Con `--ai-work-residue 0` Rust completa los 365 días y
las **367 muestras coinciden** (ConvoyPrueba, CRT 123, `--human-battles-auto`). El valor queda medido, no supuesto.
En ConvoyHecho el original entra dos veces con residuos distintos (550581584 y 0): por eso el valor es por tick.

## Contraste en `clock`, 365 días (CRT 123)

| Guardado | x86 | Rust | Muestras iguales |
|---|---|---|---|
| ConvoyPrueba | 365 días | 365 días con residuo medido | 367/367 |
| ConvoyAvista | 365 días | 365 días | 367/367 |
| TestAstiller | 365 días | 365 días | 367/367 |
| ConvoyMar | 365 días | 365 días | 367/367 (tras corregir los barcos humanos sin ruta y la pérdida de barco humano) |
| ConvoyHecho | 365 días | 365 días | 367/367 (tras corregir la recarga) |
| AUTO | 365 días | 365 días | 367/367 |

Informe compacto: `oracle/reports_clock_365.json`. AUTO sale de la copia de la nube
(`games/cloud-stage/visibility_AUTO.pat`, el original tiene enlace duro y no se podía copiar).

## Diferencias reales de Rust que salieron de `clock` (corregidas)

1. **ConvoyMar, día 258: barcos humanos sin ruta.** Con `--probe-ticks 65792:65808:1` la primera muestra distinta era el tick
   65793: los barcos 118 y 119 (estado 0xF, jugador, sin ruta) no cambiaban de sitio en la lista por x. La causa no era el
   reordenamiento (`XOrder::resort` coincide con `0x50D0C0`), sino que `Host::process_ship` de `p3-sim::navigation` solo
   recorre barcos con viaje vivo y congelaba estos dos. El original los recorre siempre: `advance_along_route` sin ruta resta
   vida cada 4 ticks (dueño < comerciantes, humano) y el barco se reordena por x. `SeaHost::walk_idle_human` (falso por
   defecto; verdadero en el anfitrión de navegación) hace que `tick_sea` también recorra los barcos humanos en 0xF sin ruta.
2. **ConvoyMar, día 273: barco humano que se hunde en el mar.** Al llegar la vida a 0 el original llama a `0x453DF0`
   (interfaz), `0x519D90` y `0x548CA0`. `0x519D90` compone la carta `0x1B` (16 bytes: x, y, palabra `+0x15E`, capitán, tripulación)
   solo si `merchant+8 == 0` y la entrega con `0x4D6530`; las otras dos son presentación. Se conecta como
   `SeaHost::prepare_dead_with_world` → evento `barco-perdido:` → `Host::apply_navigation_ship_lost_letters` en `p3-sim`.
3. **ConvoyHecho, recarga del guardado propio.** `Save::parse_unpacked` tomaba los últimos siete `0xAFFE0000` como separadores
   de bloque, pero esos cuatro bytes aparecen por casualidad dentro de un bloque (aquí, el de barcos). Ahora prueba las
   combinaciones de separadores y acepta la primera cuyo bloque de barcos se lee (`Ships::parse`, versión `0x85`);
   `scripts/save_split.py` aplica la misma regla.

## Contraste en `paused` (11/11, mismos guardados)

Los seis guardados dan **367/367** muestras iguales en `paused` contra x86 con el código corregido (ConvoyPrueba, ConvoyAvista,
TestAstiller, ConvoyMar, ConvoyHecho y AUTO; `--residue-from-original --siege-tails-from-original`).

## Carta 0x1B aislada

`oracle/x86_ship_lost_letter.py` ejecuta `0x00519D90` real (gancho en `0x4D6530`) con 1.500 casos aleatorios; fixture
`ship_lost_letter.json`. `p3_core::ships::lost_letter::ship_lost_letter` coincide en los 1.500 (1.002 con carta, 498 sin
ella por `merchant+8 != 0`; la mutación de un desplazamiento se detecta). Los bytes 6 y 7 del registro valen siempre 0.

## Segundo año (`--days 730`, `clock`)

El ejemplo `c10_snapshots` limitaba `DAYS` a 365; ahora admite 1100. Al pasar el día 365 apareció otro fallo real de Rust:
`transfer_host_context` (reconstrucción de la sesión, p. ej. tras una orden naval) no copiaba `mission_flat_rdata`, y los guiones
con comandos planos (0x45) fallaban en ConvoyMar y AUTO. Corregido.

## Segundo año (730 días, 11–12/11/2026)

Resultado con `--days 730 --battle-pacing clock` (informe `oracle/reports_clock_730.json`), x86 frente a Rust, muestras comunes:

| Guardado | Muestras | Iguales | Primera diferencia |
|---|---|---|---|
| ConvoyMar | 732 | 732 | ninguna |
| ConvoyPrueba | 732 | 732 | ninguna |
| ConvoyAvista | 732 | 732 | ninguna |
| ConvoyHecho | 732 | 732 | ninguna |
| TestAstiller | 732 | 732 | ninguna |
| AUTO | 732 | 732 | ninguna |

Correcciones que lo permitieron:

- **Asedio 0x8E contra el jugador.** `town_siege_begin` (0x629C60) solo abre una ventana (aviso 0x15) y marca `Town+0x2C8 |= 0x10`. Política sin pantalla: x86 sustituye la escena ausente en `0x62B247` por un bloque de 0x10000 bytes a 0xFF (adaptador `siege_scene_without_window`); Rust ya no rechaza la ciudad del jugador (`start_siege`). El reloj de pared del asedio amurallado se toma del volcado x86 (`siege_starts`, leído por `--siege-stack-tails`) y la memoria de torres se sintetiza a cero como el asignador x86.
- **Escenas de asedio en el tick de un encuentro naval.** `resume_encounter_after_manager` (p3-sim) reanudaba el tick con el puente de economía y omitía `siege_scene`; el temporizador de aproximación (+0x14) perdía una cuenta por encuentro. Ahora las ejecuta `DailyClock`.
- **Torre de reserva.** `assault_walled_inner` validaba el tipo de la ranura de reserva aunque el recuento de torres fuera cero; el original no lo consulta.
- **Incendio 0x26.** `prepare_burn` contaba como ambiguas las casillas libres de la tabla de oficinas (dueño 0, ciudad 0, todo a cero) junto a la oficina real del comerciante 0.
- **Comparador.** Una subasta naval 0x0A recién creada (fase 0) sobre una casilla liberada conserva en +8 la dirección obsoleta del guion 0x1B anterior; solo se compara su presencia (`stale_script_pointer_reused`).
- **Cabeza de atracados.** Ver «AUTO, día 638».
- **Heap x86.** El bump allocator del oráculo se amplía a 0x2F000000 (`heap_limit`): con 64 MB se agotaba hacia el día 458–499.

**AUTO, día 638.** Tras una batalla idéntica en los dos lados, una expedición hanseática terminaba (fase 5, `finish_convoy`, tick 125903430) y apilaba sus barcos en la lista de atracados (`gestor+0xE8`, global 0x6DD888). Rust llevaba a la flota viva las imágenes de barcos y convoyes (`factory_nav::publish_images`) pero no la cabeza de esa lista, y `publish_navigation_delta` la sobrescribía después con la antigua; el siguiente barco liberado (barco 64, tick 432) se enlazaba a la cabeza vieja (62) en lugar de a 120. Ahora `publish_images` recibe la cabeza antes y después de la tarea y la escribe en `gestor+0xE8`. Con ello AUTO completa 732/732.

La localización se hizo con sondas por tick (`--probe-ticks`, iguales hasta el inicio de la batalla) y con una traza solo-observación de escrituras a ranuras de barco en el x86 (`P3_X86_MEMWATCH='62,64,120;tick'`, nueva en `x86_c10_real_save_clock.py`).

Procedimiento: `whole_game_corpus.py --days 730 --human-battles-auto --siege-tails-from-original --battle-pacing clock --residue-from-original` (un guardado por proceso; unos 25 min de x86 y 10 de Rust por guardado). Las ejecuciones largas se pierden si el contenedor se reinicia: lanzarlas con `setsid nohup` y vigilarlas dentro del turno.

## Límites

- La cadencia de fotogramas (40 ms) es una elección del anfitrión: el oráculo prueba paridad con ese ritmo, no con una ejecución
  concreta del juego real.
- El adaptador `0x5256F0` es presentación (ventana ausente); no se ha comprobado en el juego.
- Un corte de muestra con una batalla viva la termina sin reloj (en los dos lados); usar `--probe-ticks` fuera de batallas.
