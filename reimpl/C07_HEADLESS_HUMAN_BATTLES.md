# C07: batallas piratas contra un jugador humano sin interfaz

Política explícita y opcional (`--human-battles-auto` en `p3-mvp/examples/c10.rs`,
campo `NullHost::headless_human_battles`). Desactivada por defecto: sin ella la
simulación sigue parándose en el primer encuentro pirata con un convoy humano.

## Qué sustituye

En el original, un encuentro pirata contra un convoy de un jugador local abre la
pantalla de combate (`object+0xAC8 == 0`, bandos `0x5D/0x5E` sin reloj automático)
y el jugador decide: luchar en tiempo real, abordar, huir o rendirse, y además lee
los avisos de la batalla. La política reemplaza **solo esas decisiones**:

| Decisión del jugador | Con la política |
|---|---|
| Aceptar/dirigir la batalla en pantalla | El combate se resuelve como batalla IA contra IA del original |
| Orden de abordaje y diálogo de captura | Sin diálogo; el abordaje sigue la regla IA (`can_capture`) |
| Lectura de avisos de la batalla | Las noticias de la batalla se vacían (`notifications`) |

## Precedente en el original

`classify_battle_sides` ya resuelve como IA los bandos humanos que no son el
jugador local (partida en red/hotseat): marca `0x5D=1`, `0x5E=1` y `0xAC8=1`.
`start_existing_ai_pirate_convoys(..., human_as_ai)` aplica exactamente esas tres
escrituras al bando humano; el resto del arranque es el código original. La prueba
`headless_policy_resolves_human_encounter_as_original_ai_battle`
(`p3-core/tests/x86_pirate_engage_start_chain.rs`) exige con la política los mismos
barcos, convoyes, gestor y batalla serializada que el fixture x86 de la rama IA.
En el runtime de batalla el jugador local pasa a `u32::MAX`, por lo que tampoco se
pide el diálogo de abordaje.

## Captura con dueño NPC (0x00540520)

El abordaje entre un barco humano y uno pirata ejecuta `op_capture_ship` con un
propietario NPC. Se portan las dos ramas (`capture_world_with_npc`):

- Captor pirata: desenlace del comerciante, botín del patrón del líder (65 % de
  casco 0x52A7D0 más carga, tope 60000, prestigio 65), carta 0x30, enlace `+4`,
  liberación de capitanes, registro de banda 0x51A470/0x515F70 y estado 0x12.
- Captor comerciante de un barco pirata: desenlace de bandas 0x5066F0,
  recompensa 0x4F8D90 (`0x672DDE`), carta 0x54 por capitán, tarea tipo 5 del
  patrocinador y estado 0x14.

- Captor y objetivo con dueños NPC distintos (p. ej. 255 contra 250): se combinan las dos
  ramas, el desenlace de bandas del objetivo y la rama del captor pirata; la carta 0x30 lleva
  `captor & 0x3F` y `objetivo & 0x3F`.

- Captura con un convoy de la caza hanseática (0x503B00 verdadera). Si el objetivo era de la caza
  (0x540836), se antepone a `tarea+8` el nodo `{dueño perdido, clase 0x672855 = 1, +0x15E}`. Si el
  captor es comerciante y su convoy es la caza (0x540B00), el nodo es `{captor, clase 0x672857 = 3,
  +0x15E}` y, cuando los bytes `+2/+3` del último capitán con carta 0x54 coinciden con
  `tarea+0xC/+0xD` (el capitán buscado), `tarea+0xE` = 0x67285A = 1. El anfitrión da la tarea
  (`hunt_task`) y guarda el nodo (`hunt_loss`).

Contraste: `oracle/x86_capture_world_npc.py` ejecuta el handler x86 aislado en 640
casos, 128 con dos dueños NPC y 128 con la caza en el objetivo, el captor o ambos (nodos 0x4F3980 y
0x4F3AD0 reales; 0x503B00 y get_task interceptados). `p3-core/tests/x86_capture_world_npc.rs`:
todos iguales, comparando también la cadena de nodos y la tarea. Mutaciones (invertir los bytes
del capitán buscado o la marca del nodo) se detectan.

Contraste: `oracle/x86_capture_world_npc.py` también registra el informe al
regidor 0x4EC864 (estado 0x12, uno de cada cinco casos).

## Efectos conectados después de la captura

- Tarea tipo 5 del patrocinador (0x4D8CF0 con `tick + 0x300`): el runtime de batalla la
  aplaza (`BattleAdvance::sponsor_tasks`) y p3-mvp la escribe en el planificador con
  `SponsorTask::write` junto a los informes al regidor. Contraste del registro (vencimiento,
  tipo y carga) con los casos x86 que la crean: `p3-sim/tests/c07_sponsor_task.rs`. Una captura
  produce siempre `capture_events`, así que las guardias de asalto y misión que no publican
  efectos ya rechazan esa combinación.
- Informe al regidor 0x4EC864: el runtime de batalla no tiene planificador; guarda
  `(barco, captor, bytes +2/+3 del capitán)` leídos en la llamada y p3-mvp los aplica
  con `Scheduler::report_captured_look` antes de cualquier otro cambio de tareas.
- Cambio de líder de un convoy pirata sin capitanes (0x5017F0): `allocate_captain`
  0x5097C0(1) con `init_captain(flag=false)` y los recursos de la sesión B4. Con la
  lista libre agotada se rechaza («efecto de líder pendiente»).
- `remove_from_x_order` 0x50D040 («order») se ejecuta sobre las listas vivas en la
  navegación y en el cambio de líder de la captura (`remove_order_with_world`).
- Resolución tras la batalla (0x50BFD0) de un convoy NPC que se ha quedado sin
  barcos: estado 0x12/0xA00 y `remove_convoy` 0x50A3D0, sin consulta 0x503B00.
- Convoy NPC que vuelve con barcos hundidos (`return_npc_with_losses`, rama
  0x50C0EE..0x50C3CB con la cola 0x50C8F6): baja con `remove_ship`, recompensa
  0x4F8D90 sin capitán (remove_ship deja `+0x42=0xFFFF`, sin cartas), estado 0x12 de
  los supervivientes y +0x1B del acreditado. Si no sobrevive ninguno, `ensure_leader`
  falla y se usa la disolución 0x50C5A6 (`return_dissolved_*` admite ahora dueño NPC
  sin supervivientes y añade la recompensa extra 0x50C99B).
- Líder NPC hundido con supervivientes: `ensure_leader` 0x5017F0 (`convoy::pick_main`) se ejecuta
  antes del recorrido sobre las listas vivas (`LeaderHooks`): saca al líder del orden X, elige el
  primer barco vivo con capitán y, si no hay, asigna uno con 0x5097C0(1) real (`init_captain`
  con los recursos B4 de la sesión, `DestroyWorld::captain`); después reinserta al nuevo líder
  (`insert_in_x_order(main, 0xFFFF)`). Si el líder no aparece vivo al final se repite.
  Con la lista libre agotada (`+0xE0 == +0xF2`), 0x5097C0 amplía el pool en 64 entradas
  (0xFF y cadena `i+1`, `+0xF2 += 64`) antes de asignar: el retorno repite `pick_main` sobre el
  pool ampliado (la asignación es lo único que lee el pool) y `DestroyWorld::captains` crece; p3-mvp
  toma la capacidad de su longitud.
  Contraste: `oracle/x86_battle_return_npc_losses.py`, 384/384 casos iguales en seis modos
  (líder vivo x2, todos hundidos, líder hundido con superviviente tripulado, sin capitanes y
  con el pool lleno, que crece de 4 a 68), la mitad con orden X enlazado.
- Bajas en el tick marítimo (`remove_ship_with_world`): la navegación ejecuta
  0x507FB0 con el desenlace de banda 0x515F00 antes de la baja, como el adaptador ya
  contrastado `pirate_group_remove`; ClearRoute, Order y ConvoyRefresh se aplican
  sobre las listas vivas. Otros efectos (PoolRelease, MerchantChanged) se rechazan.

## Expedición hanseática de caza sin bajas (0x50C06F)

Con 0x503B00 verdadera (tarea 0x34 en fase >= 7 cuyo `+0x16` es el convoy), el convoy comercial
no se disuelve: `return_hunt_expedition` aplica el retorno conservado con estado 0x16 en el
convoy y sus barcos y, si el acreditado no es comerciante, `tarea+0x15 = 0xA` (llegada de la
caza, fase 10, ya portada en `c08_hunt`) y `tarea+0xE = 2` (byte 0x67285B) cuando valía 0.
Si el convoy no tiene líder aceptable (`+0x14` bit 0x10) tampoco se disuelve: `tarea+0x15 = 0xA`
y `tarea+0xE = 3` (byte 0x67285C) siempre, y el bit se conserva. `battle_manager` lo usa antes
de la rama de disolución. Contraste: `oracle/x86_battle_return_hanse_hunt.py`, 224/224 casos
iguales (64 sin líder aceptable; 93 modifican la tarea).

Bajas de la caza (0x50C401..0x50C4DA): cada barco hundido sale del convoy en estado 0x11
(`+8`, `+0x50`, `+0x52` a 0xFFFF, ruta limpia, `+6` intacto) y produce un nodo
`{marca = dueño, clase = byte 0x672854 = 0, texto = ship+0x15E}` antepuesto a `tarea+8`. Si el
dueño es humano el barco queda así; si es un comerciante IA, además `remove_ship` 0x507FB0 lo da
de baja (orden X, capitán a la lista libre, flota del comerciante y poda de grupos 0x4F5900).

Líder de la caza hundido: `return_hunt_expedition` reproduce el bucle comercial completo
(0x50C3CB..0x50C51F) en lugar de reutilizar el retorno conservado, porque los supervivientes se
procesan antes de elegir líder y `pool_ok` lee la carga que `recompute_load` acaba de escribir.
Todos los supervivientes pierden la ruta (ninguno es el líder viejo), `convoy+0x10 = 0xFFFF` y
`ensure_leader` 0x5017F0 del comerciante elige otro: con `pool_ok`, o marca el bit 0x10 y toma el
primer barco con capitán. Si no queda nadie el convoy vuelve vacío y sin líder, sin
`remove_convoy`, como el original. El original vuelve a ejecutar esa elección aunque el líder
sobreviva; el resultado es el mismo.

Contraste del retorno de la caza: `oracle/x86_battle_return_hanse_hunt.py`, 448/448 casos iguales
(sin bajas, sin líder aceptable, bajas humanas, bajas IA con `remove_ship` real, líder hundido con
dueño humano o IA y líder vivo que no es el primero), comparando también capitanes, comerciantes,
poda de grupos y la cadena de nodos que deja el x86 en su montón. Mutaciones: tratar como humano
cualquier hundido y elegir líder solo si sobrevive se detectan; forzar la elección también con
el líder vivo es equivalente al original. Límite: el líder debe seguir vivo y
presente, porque el `ensure_leader` final 0x50C52F solo se reproduce cuando no cambia nada.

## Cierre de batalla sin vista (revisión independiente, caso 24)

`finish_ai_headless` aceptaba como silencioso un cierre en el que el convoy ampliado
convierte el participante 0xFE en 0xFF antes de la carta pirata 0x2A. Sin datos
mercantiles el recuento de capitanes es 0 y la carta (con su consulta 0x4F9430) se
omitía. Ahora se rechaza mirando el estado final. Nuevo fixture con la semilla
desplazada `0xF1415 ^ 0x2222` (`battle_finish_shifted.json`); `sea_finish` y el
cierre sin vista pasan con las dos semillas.

## Límites explícitos

- Nodos de bajas de la caza (C10, 10/11/2026, nube de Claude): **conectados**. El retorno
  (`return_hunt_expedition`, 0x50C41A), la captura (0x540836/0x540B00, `hunt_loss`) y la salida de un
  barco del convoy en `Convoy::advance` (0x502285, clase 2) anteponen el nodo a la lista de `tarea 0x34 + 8`,
  que p3-sim guarda por ranura (`Host::hunt_losses`, cabeza primero) y escribe en `scheduled_tasks` con el
  formato de 0x004F39E0 (`p3_save::tail::encode_hunt_losses`; al cargar, 0x004F3A40 rehace `+6` como byte de
  signo × 0x101). La llegada (fase 10) informa con la lista y la libera. Contraste: 1.200 listas del
  escritor/lector reales (`oracle/x86_c10_hunt_loss_list.py`) y 192 retornos con lista previa → guardado →
  carga (`oracle/x86_c10_hunt_loss_chain.py`, `p3-sim/tests/c10_hunt_loss_chain_x86.rs`). Límite que queda:
  `PirateCaptureState::hunt_convoys` sigue sin comprobar que `convoy+2` sea esa tarea (0x503B00 sí lo mira).
- Rutas en el retorno de la caza: `clear_route` 0x516840 quita la ruta de los hundidos y de los
  supervivientes que no son el líder; si es propia (`+0x36 != 0`) el original libera los puntos
  y el descriptor. `HuntEffects::removed_routes` los lista y `battle_manager` suelta su
  `SavedRoute`. Contraste: 64 casos más con rutas propias y compartidas (512/512, comparando
  también qué libera `free` y en qué orden).
- Hundido IA de la caza con ranura del pool de paradas (`+0x132`): `remove_ship` la libera con
  0x4D4DD0(ranura, 0). `HuntEffects::released_stops` las da en orden, `battle_manager` las aplica
  a una copia de `RouteStopRuntime` (`DestroyWorld::route_stops`) y p3-mvp la publica con el resto.
  Contraste: 64 casos más con un pool de 8 paradas y cadenas circulares de 1 y 2 (576/576,
  comparando el pool después de liberar). Mutaciones (no liberar, liberar en orden inverso) se
  detectan. `remove_ship` de un barco IA admite orden X, ruta, poda de grupos y el pool de
  paradas; otros efectos (interfaz, bandas, refresco de convoy) se rechazan.
- Asignación de capitán sin recursos de sesión: se rechaza («efecto de líder pirata
  pendiente»). En la captura (0x540520) la lista libre agotada también se rechaza; solo el
  retorno tras la batalla amplía el pool.
- Convoy NPC vaciado por las bajas (0x50C3A9 `remove_convoy`): inalcanzable, porque
  `ensure_leader` solo devuelve éxito con un líder vivo, y los vivos no se dan de baja.
  Queda como error defensivo.
- Si el nuevo líder es la cabeza del orden X (anterior 0xFFFF), `insert_in_x_order` del
  original lo trata como desenlazado y `resort_in_x_order` entra en un ciclo: el x86 no vuelve.
  El oráculo coloca un barco ajeno al convoy en la cabeza; ese caso no se contrasta.
- La tarea tipo 5 toma el tick del reloj de p3-mvp al publicar el avance de la batalla; el
  original lee 0x6DE4B4 en la captura. Coinciden mientras la batalla avance dentro del mismo tick.
- No reproduce lo que un humano habría hecho; es la resolución IA del original.

## Pase de 365 días (10/11, `--human-battles-auto`, 9 partidas)

| Ritmo | Año completo | Paradas |
|---|---|---|
| `paused` (por defecto) | 6/9: fps-cache AUTO y TestAstiller, visibility AUTO, ConvoyAvista, ConvoyHecho y TestAstiller | ConvoyMar (x2) y ConvoyPrueba: subasta 0x0A, adjudicación de lote existente |
| `clock` | 2/9: TestAstiller (x2) | subasta 0x0A (3), rayo sin corte «pila sin inicializar» (2), alta meteorológica 0x78 «ranura libre de fábrica desincronizada» (2) |

AUTO y ConvoyHecho pasan ya la fase 0 de la subasta. `paused` sigue siendo el valor
por defecto: `clock` alcanza otras ramas (tormentas, altas meteorológicas) que aún
no están conectadas. Esas paradas no son de C07.

## Divergencias ajenas anotadas (revisión 9/11, fuera de C07)

- C05 `x86_construction` caso 497: panic en `grid.rs:30`.
- J02 `snow_step` 0x576300 caso 472.

## Misma política en el original emulado (10/11, tarde)

`x86_c10_real_save_clock.py --human-battles-auto` aplica la política al x86 para
el oráculo de partida entera: en `SeaBattle::start` (`0x6056A9`) escribe
`0x5D/0x5E/0x5F = 1` en el bando humano (como `classify_battle_sides` con un
humano remoto) y, durante cada actualización de la batalla, el jugador local
`0x6DFC14` vale `0xFFFFFFFF`. Así no se abre el diálogo de abordaje (`0x606B8D`
→ `0x41D9B0`). Con la política en los dos lados, ConvoyMar, ConvoyPrueba y
ConvoyAvista coinciden byte a byte durante 365 días (`PARTIDA_ENTERA.md`).
