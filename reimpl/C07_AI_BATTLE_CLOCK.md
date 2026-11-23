# C07: batallas de la IA que avanzan junto al reloj mundial

Opción `--battle-pacing clock` de `p3-mvp/examples/c10.rs`
(`headless::BattlePacing::WithClock`). El modo por defecto sigue siendo `paused`
(el anterior: el mundo se detiene y la batalla avanza de 16 en 16 subticks hasta
terminar).

## Qué hace el original

- `0x00612D20` (final de cada paso del gestor de batallas) encola C8, que reanuda
  el reloj, solo si **todas** las batallas activas son automáticas (`+0xAC8 != 0`).
  Una batalla entre IA, o con un humano resuelto como IA (`--human-battles-auto`),
  no detiene el mundo.
- La batalla recibe sus pasos por la operación 0x94 (en línea en `0x00536BBF`),
  que llama al `update(ticks)` de cada batalla (vtable+0x14). `update` simula
  `(ticks + resto) >> 4` pasos y guarda el resto en `+0x130F`.
- La cadencia de esas 0x94 la decide `0x00612DB0`, ya portado como
  `p3_core::pump::BattleTimePump::schedule(now, modo, hotseat, ms, div)`:
  `ticks = (transcurrido / 40) * 3375 / ms`, con el transcurrido limitado a 100 ms
  fuera del modo 1 y un tope de 0x5460; el bombeo se desactiva tras emitir.
- `ms` (milisegundos por tick mundial) y el divisor del modo 1 vienen de la
  cabecera del guardado (`header[4 + SETTINGS_LEN]` y el dword siguiente; un
  divisor mayor que 4 se normaliza a 2, como el original).

## Cómo se reproduce sin pantalla

`headless::run_with_clock` usa un reloj de pared simulado con fotogramas de
`frame_ms` (por defecto 40 ms, la unidad de la fórmula de `0x00612DB0`):

1. Cada fotograma despacha la 0x94 que quedó encolada en el anterior
   (`Session::advance_paused_battle(ticks)`) y vuelve a programar con el bombeo.
2. Cada `ms` milisegundos de pared avanza un tick mundial
   (`Session::advance_tick_beside_auto_battle`). Mientras dura la batalla, su
   inicio pasa de `encounter_battle` a `Session::concurrent_start`, de modo que la
   simulación no ve un encuentro pendiente y sigue su tick normal.
3. Al destruirse la batalla, `install_resolved_battle_manager` publica el gestor
   y se limpian `encounter_battle`, `concurrent_start` y `paused_battle`.

El reloj de pared no forma parte de la partida: el ritmo de fotogramas es una
elección del anfitrión, igual que en el original lo es la velocidad del equipo.
Por eso este modo no promete paridad tick a tick con una ejecución concreta del
juego, sino el mismo **orden** de sucesos (mundo y batalla intercalados).

## Casos sin reloj (se informan)

Cada encuentro del informe añade `world_ticks_during_battle`, `battle_subticks` y
`ended`:

- `destruida`: la batalla terminó por sí misma con el reloj en marcha.
- `fin_de_intervalo_sin_reloj`: se alcanzó el final del intervalo pedido con la
  batalla viva; se completa con el mundo detenido (el guardado no admite aún una
  batalla automática a medias: `Session::save` lo rechaza).
- `otro_encuentro_sin_reloj`: el tick mundial creó otro encuentro. Dos batallas a
  la vez todavía no se admiten: el nuevo encuentro se aparta
  (`take_pending_encounter`), la batalla viva se completa sin reloj y el nuevo se
  procesa después (`restore_pending_encounter`).

## Resultados (9 guardados, 365 días)

Ver la tabla de `C07_AI_BATTLE_CLOCK_RUNS.md`.

## Límites

- Solo una batalla viva; las superpuestas se serializan (caso anterior).
- Batallas con un jugador local real (`+0xAC8 == 0`) siguen en el modo pausado:
  necesitan la interfaz.
- Hallazgo ajeno a la batalla: con navegación viva, la fase 1 de la expedición
  hanseática (tareas 0x23/0x24/0x34, `factory_preview` en `p3-sim/src/lib.rs`)
  crea los barcos en `expedition_factory` y en las cargas, pero no en la imagen
  naval viva. En `fps-cache/AUTO` con este modo la tarea 0x23 llega en el tick
  125790208 y la obra de muralla se detiene después con «barco de muralla
  desincronizado» (barco 122). En modo pausado esa tarea no llega dentro del año.
  Es zona de C10 (cacerías, Codex): no se toca aquí.
- Prueba: `p3-mvp/tests/c07_ai_battle_clock.rs` (necesita `P3_SAVES`, `P3_CPR` y,
  opcionalmente, `P3_EXE`).
