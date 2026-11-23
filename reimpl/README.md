# Reimplementación de Patrician III

Decisión: [coordination/DECISION_REIMPLEMENTACION.md](../coordination/DECISION_REIMPLEMENTACION.md) (Rust, paridad exacta, C01+C02 primero, comparación con el x86 original).

**Cliente, 26/10/2026 (PC de Sergio):** [p3-audio](p3-audio/README.md) inicia la
ruta audio → interfaz → gráficos en un crate independiente: 48 funciones de
control, muestras, bancos, música/ambiente, duración, voz, edificio, lowpass y
selección regional/modos de pantalla, constructor, configuración, apertura,
servicio cooperativo y cierre.
Corpus contrastados con el x86: 3.852, 4.634, 966, 4.627 y 3.231 llamadas, más 104 con
el INI inglés real (51 listas/201 entradas). Lowpass: otras 1.050 llamadas, 769 a
controles ya portados y 256 estímulos de fin de voz contados aparte. Selección
regional: 2.565 llamadas y otras 80 junto con el gestor de sonido. Consumo de azar
por un puerto al CRT compartido, sin nuevo generador; límite explícito para el
bucle de posición que puede no terminar. Ciclo de vida: 474 llamadas; configuración
inglesa real con 12 títulos/modo 3 (tres llamadas, fixture comercial local).
Apertura y servicio: 1.043 llamadas, incluidas las de una rutina dependiente no
catalogada que no se suma al registro; 40/40 aristas y 58 DIV controlados.
Catálogo regional: tramo de constructor contrastado en 146 cargas/48 selecciones;
INI real con 47 entradas (23 regionales, 18 mar y 6 ambientes). Ese tramo no
acredita el constructor GameScreen entero. SoundEffectList añade diez entradas:
6.966 llamadas, 81/84 aristas (tres redundantes), prueba nativa de conexión con
el gestor y tabla de veinte nombres cotejada con el EXE. Solo libera los diez
primeros búferes, posible defecto conservado. Backend nativo de recursos
entregado: CPR tipo 1, sueltos, tokens y leases. Inventario independiente cotejado:
6.485 entradas y 178 WAV/14.244.386 bytes, todos IMA ADPCM (175 mono, tres estéreo).
Decodificador IMA ADPCM a PCM16 entregado: 130 sintéticos y 178 reales;
dos perfiles frente a FFmpeg/libsndfile. ShiftSum coincide con el PCM definido
de Miles en x86 aislado (616 llamadas SDK), con fact acotado; en dos campanas
se omiten seis bytes finales que la DLL reserva sin escribir. La biblioteca
no añade funciones del juego al registro. Canales y mezcla PCM fuera de línea
entregados: EffectDevice nativo, cursores, pausa, fin y liberación. Ganancias
y kernels escalares con escala contrastados: 17.398 combinaciones, 256 mezclas
sintéticas y 16 de fragmentos reales. Sin remuestreo, filtros ni selección
completa de rutas del SDK; ver bloque 14. Muestras/caché de SoundManager ya
conectadas con recursos/decoder/mezclador reales (bloque 15). Selector seco
original contrastado: 20.480 decisiones, 512 mezclas sintéticas y 16 reales;
omitir escala a ganancias 2047/2047 conserva unidad exacta. Bootstrap nativo de
veinte voces, caché y lista auxiliar comparten dispositivo. Streams nativos ya
conectados (bloque16): MP3 Rust y WAV PCM/IMA, música/ambiente/voz/edificio,
predecodificación acotada y reloj PCM con pausa/búsqueda/bucles. 123 MP3 reales,
369 ventanas frente FFmpeg, error máximo1 LSB; no acredita codec/tiempos Miles.
Consultas/volumen/bucles preparados sí contrastados:120 casos/603 llamadas.
DSP Lowpass nativo conectado (bloque17):168 casos/86.184 fotogramas contra
filtro y mezclador originales aislados, PCM y estado interno exactos; ruta
estéreo software, reinicio al reasignar proveedor comprobado. Siguiente:
aislar y resolver divergencias MP3. Bloque18 compara el decoder original en
16 casos propios y123 prefijos reales:122 de123 difieren; paridad MP3 fallida,
sin aceptación por tolerancia. Reloj PCM corregido y aritmética temporal SDK
contrastada:759 casos/7.590 seeks y3.098 alineaciones. El cursor de entrada SDK
sigue sin conectarse al PCM. Antes de lectura progresiva/salida: divergencias,
finales/seek y transporte temporal; ver bloque18. Bloque19 rectifica la referencia
propia:3variantes leen índice8195 fuera de tabla8192, ahora detenidas; el PCM
posterior no acredita paridad. Comparación actual propia104.832muestras/max636;
los123prefijos reales no alcanzan ese acceso y conservan sus diferencias.
Magnitudes/ganancias numéricas ya portadas bitexact aRust:8192entradas,
144productos,256bases,dos tablas256 y2048bandas. Falta conectarlas aHuffman y
escalas antes de estéreo/síntesis; el decoder activo sigue provisional. Ver bloque19.
[Dependencias y licencias](p3-audio/THIRD_PARTY.md).
Todavía no produce sonido; J04 y la integración del cliente siguen
pendientes. El núcleo de Claude conserva su propia ruta.

## Registro de funciones (`functions.tsv`)

Una fila por función del juego (vista de 5.507 de `research/catalog/functions.tsv`) que ya tiene destino en la reimplementación.

| Columna | Contenido |
|---|---|
| `address` | Dirección en el EXE inglés (`0x00XXXXXX`). |
| `state` | `implementada`, `probada_emulador`, `validada_juego`, `sustituida` o `no_necesaria`. |
| `item` | Ruta del elemento en Rust (`crate::modulo::funcion`) o motivo de la sustitución. |
| `task` | Tarea del tablero (C01, S04…). |
| `evidence` | Prueba o informe que lo respalda (obligatorio desde `probada_emulador`). |
| `note` | Límites. |

El panel se regenera con `python scripts/agentctl.py dashboard` (o `python scripts/build_reimpl_progress.py`) y se abre en `dashboard/reimpl.html`.

## Núcleo `p3-core` (Rust)

```
cd reimpl
cargo test            # paridad con los fixtures del x86 original
cargo clippy --all-targets && cargo fmt --check
```

| Módulo | Reproduce | Estado |
|---|---|---|
| `calendar` | `update_calendar` 0x005310D0 | probado contra el original |
| `scheduler` | planificador 0x006DD73C: alta, reprogramación, borrado por tipo, liberación, tareas iniciales y despachador de 58 tipos | probado contra el original |
| `tick` | `advance_time` 0x00530E80: fases del tick, paso diario, orden de subsistemas y tope de oro | probado contra el original |
| `pump` | `operations_pump_time_advance_and_autosave` 0x00546620: tiempo real a ticks, hotseat y autoguardado | probado contra el original |
| `rng` | `rand`/`srand` del CRT, `LcgRandom` (0x0046C320..0x0046C380), `sea_battle_next_random` 0x0060D259 y sus 24 copias en línea | probado contra el original |
| `economy::prices` | `get_buy_price` 0x0052E430 y `get_sell_price` 0x0052E1D0 | probado contra el original |
| `town` | `Town` (0x9F8 bytes) con los desplazamientos de `research/types/Town.h` | base de C04 |
| `economy::production` | 17 `produce_*` (0x0050E690–0x0050FBC0), armero y `get_shipyard_level` | probado contra el original |
| `economy::employment` | plantilla objetivo, contratar, despedir, liberar trabajadores, huecos de mano de obra, consumo de milicia/astillero, constructores | probado contra el original |
| `economy::militia` | milicia diaria, reclutar/despedir, `book_transaction` | probado contra el original |
| `economy::facility` | `Facility::daily_tick` 0x005101D0 (tipos 0–20) | probado contra el original |
| `economy::population` | mínimo de pobres, consumo y hambruna, satisfacción, niveles, emigración, mendigos | probado contra el original |
| `economy::market` | umbrales de precio, mercancía más escasa, consumo de excedentes, tesoro, suministro diario | probado contra el original |
| `economy::town_tick` | `handle_town_tick` 0x0051BA10 | probado contra el original (30 días) |
| `numeric` | `__ftol` 0x00639C28 (truncado a 64 bits con entero indefinido), divisiones con y sin signo, saturación | `__ftol` probado contra el original |

Los demás sistemas (manejadores de los 58 tipos de tarea, ciudades, comerciantes, barcos…) entran por los rasgos
`TaskHandlers`, `TickSystems` y `PumpSink`; cada uno se reimplementa en su tarea.

## Oráculo (`oracle/x86_clock.py`)

Ejecuta las funciones originales del EXE inglés con Unicorn, sin DLL ni juego, intercepta las llamadas a otros
sistemas y guarda entradas y resultados en `p3-core/tests/fixtures/` (sin bytes del EXE). Las pruebas de Rust
repiten los mismos escenarios y guiones y exigen el mismo estado (FNV-1a de los registros tras cada paso), los mismos
eventos en el mismo orden y los mismos campos finales.

```
python reimpl/oracle/x86_clock.py --binary RUTA/Patrician3.exe --check   # los fixtures siguen saliendo iguales
python reimpl/oracle/x86_clock.py --binary RUTA/Patrician3.exe           # regenerarlos (semilla 0xC01)
```

Corpus actual: 6.300 fechas, 120 escenarios del planificador (97.740 eventos: los 58 tipos, empates, tareas creadas
dentro de un manejador con vencimiento pasado, crecimiento del array dentro de un manejador, borrado por tipo,
`free_all` e `init_new_game`), 60 avances de tiempo (hasta 1.300 ticks, cambio de ficha a mitad) y 150 secuencias
del bombeo.

### Límites de C01

- Solo se comparan los efectos que se ven desde fuera de cada función (estado del objeto, llamadas y sus
  argumentos). Los manejadores de tipo, las ciudades, los barcos, etc. son guiones, no código del juego.
- Los registros nuevos del planificador valen cero; en el original son memoria sin inicializar del montón.
- Si el despachador se queda sin cabeza válida, el original lee fuera del array; el modelo se detiene y lo cuenta en
  `Diagnostics::head_out_of_bounds`. Con la tarea centinela de las partidas reales no ocurre.
- Si el array crece dentro de un manejador de 0x03/0x06/0x1A/0x2C, el original escribe en el bloque liberado; el
  modelo lo reproduce suponiendo que el montón no reutiliza ese bloque durante la llamada.
- Fuera de alcance aquí: casos 0xC4/0xC8/0xC9 de `execute_operations` (velocidad y pausa; van con las operaciones),
  carga y guardado del planificador (S04/S05) y las preguntas que necesitan el juego (TIME-Q1..Q5, CLOCK-Q1..Q8).
- No incluye el cobro de impuestos, la peste/asedio más allá de lo que lee el tick, ni el comercio de los comerciantes con la ciudad (van con C05 y los sistemas de operaciones); sí lee la tasa de impuestos en la satisfacción.
- Nada validado en partida.

## C02: azar y reglas numéricas

`oracle/x86_rng.py` (fixture `p3-core/tests/fixtures/rng.json`) ejecuta en el x86 original `srand`/`rand` (con
`_getptd` interceptado), constructor, siembra, `next` y `next_below` de `LcgRandom`, `sea_battle_next_random`, cada
una de las **24 copias en línea** del paso (desde su `imul` hasta el almacenamiento en `+0xADC` tras unir las dos
ramas, con 34 semillas) y `__ftol` con 1.735 valores (NaN, infinitos, ±2^63, límites de 32 bits, subnormales y
bits aleatorios). `cargo test` lo compara con Rust y comprueba que las secuencias se repiten tras recargar.

Reglas para el resto de sistemas:

- Las llamadas a `rand` del mismo hilo original comparten un `SimRandom::crt` y su orden, incluidas las de
  interfaz o audio que ocurran en ese hilo (por ejemplo `scroll_map_play_ambient_sound`). Cada hilo
  original requiere estado separado; la distribución completa de consumidores por hilo aún no está acreditada.
- Cargar un `.pat` no toca el flujo CRT (`SimRandom::load_original`). Guardarlo en el formato propio
  (`snapshot`/`restore`) es una mejora, no paridad.
- Los `LcgRandom` de simulación se siembran con el tick o con campos del mundo y el de la batalla naval se guarda,
  así que repiten tras recargar.
- `(int)x` es `numeric::ftol_i32`, nunca `as i32` (que satura). Las divisiones respetan la signatura del original.

Límites de C02: la precisión x87 en marcha (palabra de control 0x27F según el CRT) no se ha medido con el juego; el
rango de exponente extendido de los intermedios x87 no se modela; las 66 divisiones mágicas y los 310 `rand` no
alcanzados por llamadas directas se tratan al reimplementar cada consumidor.

## C03: núcleo portable y primera función sustituida (precios)

`p3-core` no tiene dependencias, ni E/S, ni `unsafe`, ni direcciones del proceso original
(`tests/test_reimpl_c03.py` lo comprueba). La primera fórmula sustituida son los precios de compra y venta
(`economy::prices`): reciben un `Market` (bandera de comercio, existencias, cuatro umbrales y mercancía marcada) en
lugar de leer la tabla global de ciudades, y la venta recibe el factor de dificultad que el original lee de
`this+0x64`.

`oracle/x86_prices.py` (fixture `p3-core/tests/fixtures/prices.json`) monta ciudades sintéticas y ejecuta las dos
funciones originales con 4.734 casos: cada mercancía con umbrales verosímiles y existencias en cada umbral ±1, el
ejemplo del libro (hierro, 20000/60000/70000/80000) y límites (ciudad sin comercio con otros bits activos, 199/200 y
1999/2000 unidades, umbrales nulos, iguales, invertidos o cerca de `i32::MAX`, cantidades negativas, nulas o mayores
que las existencias, existencias negativas y dificultad 0, infinita, NaN o subnormal). Rust coincide en todos.

Hallazgos del ensamblador que el libro no recoge:

- Comprar más de lo que hay cobra cada unidad que falta a 4 veces el precio base.
- Si lo que queda en la ciudad supera `t3`, la compra usa solo el factor 0,6: sin exceso ni recargo de 1,15.
- La venta toma 0 como existencias si son negativas, pero suma la cantidad a las existencias sin corregir.
- La compra no rechaza cantidades nulas o negativas; la venta sí (`amount < 1`).

Precisión: el oráculo fija la palabra de control 0x27F (53 bits, la de Windows; el juego solo usa `fldcw` dentro
del CRT para guardar y restaurar). Con 0x37F (64 bits) no cambia ningún caso; con 0x7F (24 bits, la que pondría
Direct3D sin `FPU_PRESERVE`) cambian 131 compras y 165 ventas, así que la paridad depende de que el juego no corra
en precisión simple. Medirlo en marcha queda con las pruebas en partida aplazadas.

Límites de C03: los umbrales vienen dados (su cálculo, `update_town_price_thresholds` 0x00528070, va con C04);
los descuentos de capitanes y administradores se aplican fuera de estas funciones; nada validado en partida.

## C04: economía y población de la ciudad

`town::Town` guarda los 0x9F8 bytes de la ciudad y se accede por desplazamiento; así el estado completo se compara
con el del x86 tras cada paso. `economy::world::EconomyWorld` reúne lo que el tick lee fuera de la ciudad
(comerciantes, oficinas, negocios de comerciantes, barcos, huecos de mano de obra, entradas de suministro, opciones)
y `TownHost` es la frontera con lo que aún no está reimplementado: tick de oficinas, construcción, iglesia, taberna,
consejo, astillero, muralla, asedio y mensajes. Cada método se llama en el mismo punto y con los mismos argumentos que
en el original.

Oráculos (todos con FPU 0x27F y ciudades sintéticas de `oracle/synthetic.py`, que guardan la semilla y los bytes que
cambian): `x86_production.py` (17 `produce_*`), `x86_employment.py`, `x86_weaponsmith.py`, `x86_facility.py`
(`Facility::daily_tick` de los 21 tipos) y `x86_town.py` (14 funciones de población y mercado, `handle_town_tick` y
6 mundos de 30 días con el SHA-256 de cada ciudad por día). Rust coincide en todos los casos (unos 40.000).

Hallazgos del ensamblador que el libro no recoge o que se confunden fácilmente:

- `town_excess_consumption` divide entre el umbral **3** (no el 2) y solo actúa sobre mercancías con producción
  diaria nula.
- `remove_workers_for_emigration` avanza al sumar los negocios de comerciantes con el índice con que acabó la lista
  de negocios, no con `office.next`.
- `town_daily_construction_planner` y el consejo se llaman por `TownHost` y su efecto no está cubierto aquí. La iglesia y la taberna se conectaron después: ver `research/systems/economia/C04_IGLESIA.md` y `research/systems/economia/C04_TABERNA.md`.

Límites de C04:

- En estos oráculos anteriores, los efectos de `TownHost` (oficinas, construcción, iglesia, taberna, consejo, astillero) se
  sustituyen por ganchos que devuelven un valor del caso: no hay paridad de lo que hacen por dentro.
- Los corpus originales no tenían comerciante local en la ciudad. La revisión C04 añade cuatro casos con aviso local
  y no local, y contrasta la secuencia de ventanas mediante ganchos; `TownHost::famine_notify` recibe el indicador
  local. Las ventanas y los constructores de cadenas siguen simulados.
- `recruit_militia_unit` con alcalde humano usa `find_or_create_office` de `TownHost`; los 30 días no la ejercitan.
- Los mundos sintéticos no son estados de partida: comprueban cada rama, no su frecuencia real.
- Implementados impuestos diarios y cierres semanal/mensual, compra/venta directa de oficina y de barco, y compra
  automática de insumos, conectados a precios, presupuesto, carga y reputación. `EconomyTickBridge` conecta las
  fases de reloj y ciudad; el anfitrión coordina los demás sistemas. El comercio agregado de convoyes y las ramas de insumos, administrador e IA están implementados y contrastados.
  Quedan el resto de `Office::daily_tick`, la conexión real de otros puertos y la persistencia integrada; peste y
  asedio solo cubiertos en lo que lee el tick.
- Nada validado en partida.

## C05: construcción y urbanismo (en curso)

Módulo `p3_core::buildings`. Oráculo `oracle/x86_buildings.py` (fixture `p3-core/tests/fixtures/buildings.json`,
prueba `tests/x86_buildings.rs`, una ejecución con ~10.800 casos). Estado de una ciudad: la `Town` más
`Site { records, cells }` (registros de 8 bytes y rejilla del plano; los punteros `Town+0x75C/0x80C` quedan a 0).

Hecho con paridad en emulación (24 funciones del original; ver `functions.tsv`, tarea C05):
registros de edificios y listas, huellas y celdas del plano, recuentos de defensa, colocar edificios y
caminos, cola de obras (`enqueue_construction_record`), edificio vecino del mismo tipo, valor base con FPU
y búsqueda de la celda más cercana para un tipo.

Límites: `find_or_create_office` entra como cabeza de lista; los tipos de huella 2 y 5 de
`pave_roads_beside_building` y los tipos 52–53 de `find_nearest_cell_for_type` leen pila sin inicializar en el
original y no se prueban; los accesos fuera de la rejilla panican en Rust y el generador los evita con márgenes.

Pendiente de C05: `start_building_construction` (0x521900), `evaluate_build_request`, `place_planned_building`,
`advance_construction`, `finish_all_pending`; guardado/recarga; oficinas, astilleros, demolición y muros.

Actualización C05 (parte C5): `start_building_construction` (0x521900) tiene paridad en 3.000 casos con todos
los tipos 0–47 (`oracle/x86_construction.py`, `tests/x86_construction.rs`). Las llamadas a oficinas, mapa
visual (class22), reputación, precios y mercancías de barcos son ganchos: el fixture guarda cada llamada y su
respuesta y la prueba comprueba que Rust hace las mismas, en el mismo orden. Siguen pendientes
`evaluate_build_request`, `place_planned_building`, `advance_construction` y `finish_all_pending`.

### C05, partes C7 a C15 y astillero (25/10/2026)

62 funciones de C05 con paridad frente al x86 en emulación (`functions.tsv`, 3.000 casos cada prueba salvo
`x86_buildings.rs`). Cada una tiene oráculo en `oracle/` y fixture en `p3-core/tests/fixtures/`:

| Prueba | Funciones |
|---|---|
| `x86_evaluate.rs` | valoración de una petición de obra (0x51F3A0) |
| `x86_advance.rs`, `x86_finish.rs` | avance de obras y cierre de las pendientes (0x51FF30, 0x5205D0) |
| `x86_save.rs` | tamaño, guardado y carga de los datos de edificios por versión de partida |
| `x86_housing.rs` | recuento de edificios y reparto de niveles de casa (0x520BD0, 0x520840) |
| `x86_remove.rs` | derribo de edificios (0x53CF50) |
| `x86_planq.rs` | puntos válidos del plano, límites de capilla y hospital, espacio libre dentro de murallas |
| `x86_pick.rs` | posición de edificio al azar (0x525B90) |
| `x86_streets.rs` | celdas de calle junto al centro (0x4FB580) |
| `x86_road.rs` | construir una calle (0x5244F0) |
| `x86_shipyard.rs` | avance de reparaciones y obras del astillero (0x508AA0), desenlace de atracados (0x508FC0) |
| `x86_repair.rs` | inicio de una reparación (0x508660) |
| `x86_quote.rs` | presupuesto de mejora, coste y días de reparación (0x52B670, 0x52A9E0, 0x52AAD0) |
| `x86_launch.rs` | botadura de un barco nuevo (0x5083B0) |

El astillero vive en `p3_core::shipyard` (`ShipManager`: barcos de 0x180 bytes y entradas de obra de 60 con los
campos que lee cada función, accesibles por desplazamiento). Los barcos y comerciantes completos pertenecen a
S04/A02, que aún no están hechos: la ciudad, los barcos y las entradas llegan por rasgos y los demás sistemas
(oficinas, comerciante, mapa visual, mensajes, precios, reputación, notificaciones, ventanas de evento) son
**ganchos**. El fixture guarda cada llamada y su respuesta y la prueba comprueba que Rust hace las mismas, en
el mismo orden. En los oráculos de astillero se ejecutan del original `__allmul`/`__alldiv`/`__allshr` y el
`memset` del CRT.

Límites que se conservan:
- `.locked` y las reservas dinámicas: el EXE **no lee** `.locked` (`research/systems/mapa-dibujo/asset-formats`), así que
  la reimplementación no tiene máscara estática de entrada; toda reserva es un valor de celda que escriben
  `place_building`, `start_building_construction` y compañía, y la prueba encadenada las comprueba (celdas
  idénticas al x86 tras construir, obrar y derribar).
- Los casos en que el original lee fuera de la rejilla, divide por cero o lee pila sin inicializar se excluyen
  en el oráculo; Rust entra en pánico en lugar de imitar el valor basura.
- `town_pick_random_building_position` y `town_plan_get_free_space_percent_inside_walls` leen una celda más allá
  del final en casos que el perfil de pruebas no genera.
- `ship_upgrade_levels` y el resto de ganchos no tienen efecto en el oráculo: no se prueba su cuerpo.
- Nada validado en partida.

### C05, cierre de la parte de planos y escenario (25/10/2026)

99 funciones de C05 con paridad (`functions.tsv`). Nuevas desde la tabla anterior, todas con oráculo y prueba:

| Prueba | Funciones |
|---|---|
| `x86_walls.rs` | trazado de muralla (0x4FA1D0), alta de sus piezas (0x522B20) |
| `x86_fills.rs` | rellenos del plano (0x4FD680, 0x4FDA10, 0x4FD830) |
| `x86_defpiece.rs` | arranque de pieza de defensa (0x522C70) |
| `x86_wallrequest.rs` | petición de obra de muralla (0x5226C0): cantidades, coste, total, plazo y búfer de piezas |
| `x86_planmask.rs` | máscaras de solares (0x4FD0D0, 0x4FD4A0, 0x4FD5E0) |
| `x86_planlife.rs` | init, free, reset, resize y carga de archivo del plano (0x4F95C0, 0x4F9650, 0x4F9930, 0x4F99B0, 0x4F9B40) |
| `x86_chain.rs` | **escenario encadenado**: construir (0x521900) → obrar (0x51FF30, tres etapas de 1 a 70 pasadas) → derribar (0x53CF50) → guardar (0x51EA30) → recargar (0x51EC10) en una sola emulación; 1.500 casos, hash del estado tras cada paso |

El escenario cuenta 547 casos con obra terminada, 89 con derribo visual y 309 en que el derribo cambia los registros;
las cabezas de oficina pasan de un paso al siguiente. Guardar-recargar-guardar es estable.

Límites añadidos: los ganchos (oficina, comerciante, mapa visual, precios, mensajes, tareas, archivo, reserva de
memoria) siguen siendo modelos registrados; las huellas y máscaras que escriben fuera de su búfer, las
tablas leídas con un tipo >= 64, y las listas libres con centinela `count+5` que la carga recorre más allá de lo
guardado se excluyen del oráculo (en Rust, pánico). Cobertura sin ejercitar: la cola de `place_building` con
huella de clase >= 6. Nada validado en partida. Añadidas después: mejora de iglesia (0x525390), muros y edificios iniciales
(0x545F50, 119 casos, 74 con actividad: el x86 sobre ciudades sintéticas sale de la rejilla en ~99,9 % de los intentos y el generador corta a los 25 min; los mensajes de finalización son inalcanzables para edificios iniciales de ciudad), fundar oficina (0x51B890) y
donación a la caja de obras (0x4FE420). Pendiente de C05 (no bloquea el cierre): el resto de funciones de oficina.

### C10, primer ejecutable sin pantalla (30/10/2026)

`p3-sim` carga un guardado `.pat` original y avanza días con los sistemas ya portados:

```
cargo run -p p3-sim --release -- GUARDADO.pat 30 --crt-state 123
```

`--crt-state` acepta el estado CRT como entero decimal o hexadecimal de 32 bits.
La partida original no lo guarda: el número es una entrada explícita de la
simulación, no una semilla recuperada del juego. El programa muestra el estado
final para poder aportarlo al reanudar una copia guardada con `--out NUEVO.pat`.
Sin ese dato la simulación se detiene si alcanza una fase que necesita CRT.
`--celebration-state NEXT:TOWN` aporta el próximo tick de fiestas y la ciudad
(enteros decimales; 255 representa la ciudad sin asignar). El estado final se
muestra para poder reanudarlo: tampoco está en S05. `--investigation-local`
activa explícitamente el modo local usado en los ensayos; no se deduce del
archivo. Estos valores son entradas del anfitrión, no datos recuperados de la
partida. El cargador original recalcula umbrales de precio, por lo que un
recorrido con recarga puede diferir de uno continuo.
Si el avance termina con errores, `--out` no escribe una partida parcial.
El hash del resumen cubre las imágenes de ciudades y comerciantes, omitiendo la
marca gráfica `Town+0x7A4` y el contador temporal `merchant+0x47C`, que el
serializador original no guarda. No cubre todos los sistemas del mundo ni
garantiza paridad o equivalencia de todas las partidas al reanudar.

El ejecutable de ejemplo `p3-mvp --example c10` acepta también
`--celebration-state NEXT:TOWN`, `--investigation-local` y `--exe ORIGINAL.exe`.
Con el ejecutable inglés carga los textos navales y electorales; el JSON devuelve
el estado final de fiestas además del CRT, para aportarlos al reanudar.

Para intentar reconstruir la flota desde S02 se puede aportar el CPR inglés y
`--fleet-player ID`, donde ID es el comerciante local. La reconstrucción de
rutas propias y compartidas es experimental: rechaza batallas activas y
variantes navales aún no conectadas. `TEST25_ConvoyMar` reconstruye la flota,
incluidas salidas de IA sin ruta serializada. En este escenario, dos días
continuos y dos días con recarga intermedia coinciden en ciudades, oficinas,
comerciantes, barcos y convoyes, normalizando punteros de ruta; la cadena
completa no se ha contrastado contra x86 ni probado en partida. Al incluir convoyes
antes omitidos, TEST25 alcanza un encuentro pirata tras 714 ticks y se
detiene: aún faltan conectar el inicio y simulación de la batalla. Tampoco
están completos todos los barcos, efectos externos ni el guardado del combate.

En la vía explícita de encuentro de convoyes IA, la copia de rutas propias y
compartidas se ha contrastado con el original en 192 preparaciones y 192 ticks
sintéticos; el inicio de batalla se serializa en la misma ejecución x86.
Otros 192 casos comprueban que S05 conserva y reconstruye ambas rutas tras el
encuentro. Reserva, asignador y UI están interceptados; la simulación y
resolución de la batalla siguen pendientes. No es validación en partida.

La apertura en pausa también admite el formato actual de proyectiles activos
en la variante de convoyes ya soportada, con
`Session::open_with_paused_battle_and_crt`: exige un estado CRT externo, que
la partida original no conserva, y avanza ese estado durante la carga. Se han
contrastado 96 cargas x86 aisladas, registros de proyectiles y el índice espacial
con y sin obstáculos;
el asignador y el RNG están interceptados. El guardado conserva el bloque; los
disparos y la resolución de la batalla siguen pendientes.
El proceso `p3-mvp` acepta ese estado mediante `P3_CRT_STATE` en decimal o
hexadecimal (`0x...`); el cliente gráfico hereda esa variable del entorno.
Una política manual o automática sobre la batalla reabierta puede guardarse y
recargarse; los efectos sobre el reloj y la vista se devuelven al cliente para
su aplicación y no se aplican todavía. El cambio y su serialización coinciden
en 96 cadenas x86 de carga, operación 0x96 y escritura, con UI y RNG
interceptados. La actualización completa de la batalla sigue pendiente.

El reloj también procesa las listas temporales del gestor naval y las
persiste en S05, incluso junto a un encuentro inicial. El gestor coincide en
256 ticks x86 y en 128 cadenas de carga, tick y escritura; la prueba de
sesión con encuentro usa esos resultados por tramos. La liberación y la vista
están interceptadas. Al cargar una lista, el original usa solo el byte bajo
del contador; los tres bytes opacos de una reserva nueva no quedan definidos
por el código original y el anfitrión Rust los inicializa a cero.

El índice espacial reconstruido al cargar puede convertirse al índice vivo y
volver sin pérdida. `advance_battle` ejecuta en una sesión reabierta el paso
de combate sin vista cuando las ramas necesarias están conectadas y permite
guardar el resultado. En 192 batallas sintéticas sin armas y otras 192 con
proyectiles en vuelo, carga, un paso y escritura coinciden con x86, incluidos
movimiento y retirada de proyectiles; asignador y RNG están interceptados.
El paso también conecta armas, andanadas, impactos y daño al estado vivo.
Coinciden con x86 192 pasos con armas en modo directo, otros 192 con andanadas,
32 batallas de 50 pasos con 58 impactos y 32 escenarios con armamento abundante,
seis de ellos con ampliación de las tablas espaciales. Se comparan barcos,
proyectiles, tablas, CRT y escritura; reserva y servicios externos interceptados.
Otros 16 escenarios llegan hasta el primer hundimiento y coinciden con x86;
uno se guarda y reabre también mediante la sesión y S05.
`activate_encounter_battle` permite continuar en la misma sesión un encuentro
sin efectos pendientes y luego guardar el estado avanzado. Una cadena x86
encadena reloj, encuentro, primer paso y escritura; compara el bloque de batalla
y los campos persistidos de barcos, con servicios externos interceptados.
El cliente puede bombear el combate sin vista con tiempo de pared: la operación
0x94 se ejecuta en la pasada siguiente y al pausar se despacha la pendiente.
La cadencia coincide en 2.048 casos x86 y sus peticiones de modo en 512;
una secuencia de sesión con pasos temporizados coincide con la escritura x86.
El calendario mundial permanece detenido durante este recorrido; la gestión de
varias batallas simultáneas, efectos de interfaz y resolución siguen pendientes.
El recuento de pérdidas, los campos de las cartas 0x72 y el acumulador de
investigación del cierre se han contrastado por separado en 96 casos x86
sintéticos; la composición del texto y la entrega de cartas se interceptan.
En otras 32 cadenas el original ejecuta además la entrega 0x72: coinciden el
pool y las cabezas de comerciantes, con 17 cartas insertadas. S05 guarda y
recarga estas cartas. El sorteo de barco para una batalla encadenada y la
ampliación temporal de participantes coinciden en 96 casos x86 separados.
`finish` está conectado para la retirada sin bajas de participantes automáticos
sin combate encadenado: 96 pasos y cierres originales coinciden. El texto 0x29,
las demás variantes y la destrucción del combate siguen pendientes.
Para la destrucción se han contrastado por separado 128 registros cruzados de
encuentros entre convoyes, 1.024 sorteos de investigación y 512 altas de la
tarea 0x05; las listas y una tarea se guardan y recargan en S05. La devolución
de convoyes y la conexión de esos efectos al destructor siguen pendientes.
La captura, ciertos avisos de hundimiento, resolución y efectos de interfaz
siguen pendientes y causan rollback. El daño por obstáculos está conectado,
pero no forma parte de estos contrastes. No es una batalla completa ni se ha
validado en partida.

Imprime el recuento de ciudades, comerciantes y oficinas, el calendario guardado, un resumen (tick, fecha, oro de los comerciantes, dinero y población de las ciudades) al inicio y al final, y un hash del estado. Reutiliza el cargador de `p3-save`, el reloj (`advance_time`) y el puente de economía de C04 (impuestos y tick de ciudad), más los recuentos de edificios, defensa y muralla de C05 sobre los registros y celdas del guardado. La prueba `p3-sim/tests/real_saves.rs` (con `P3_SAVES`) carga todos los guardados del corpus, comprueba que el calendario guardado coincide con el que da el tick (el tick está en el `u32` de `+0x18` de la cabecera del mundo) y que 30 días corren sin fallar y con el mismo hash en dos ejecuciones.

Desde la segunda entrega también ejecuta el tick diario de las oficinas (`p3_state::office::OfficeTownHost`, `PersistedOffice::daily_tick_with_specials`) con los negocios de cada oficina resueltos desde la cadena del guardado; el oro de los comerciantes ya se mueve (+84.000 en 30 días en un guardado de prueba). Límites: sin salario ni experiencia de administrador (no se leen los capitanes del guardado), el comerciante que usa la oficina es una copia que solo se sincroniza en oro e ingresos, y los fallos del tick de oficinas se listan en el informe.

**Qué NO hace todavía** (el informe lista cada gancho inactivo con su número de llamadas): `office_update_business_employees` (0x4FFD40, sin portar), IA de comerciantes (C06), barcos en el mar (`ships_tick`, C07), construcción, taberna, concejo, iglesia, astillero, asedios, tareas programadas y calles. La **población** baja de forma constante (p. ej. de 6.925 a 4.517 en el primer grupo en 30 días, con el tercer grupo subiendo): no se ha establecido si es la dinámica real sin comercio de mercancías o un artefacto de los ganchos inactivos, así que no es un resultado fiable. Tampoco se ha comparado con el original: no hay forma de avanzar el juego completo en el emulador; el cierre de C10 exige enchufar lo anterior y comparar un tramo corto.

### Guardar la simulación completa (1/11/2026)

`Sim::persist_all(&mut SaveModel)` (`p3-sim/src/persist.rs`) escribe sobre el modelo S05 los estados conectados: las ciudades (campos guardados y suministros), lo que ya persistía `persist_construction_city` (oficinas, negocios, comerciantes, buzón, tareas, barcos tocados), el reloj (`GameWorld+0..5`, tick en `+0x18`) la navegación conectada y la fecha de los ajustes. Rechaza encuentros pendientes y conflictos navales; no cubre la evolución de batallas vivas. Es transaccional. CLI: `cargo run -p p3-sim --release -- GUARDADO.pat DÍAS --out NUEVO.pat` (no sobrescribe, recarga el resultado y comprueba el reloj).

La prueba `p3-sim/tests/persist_all.rs` (con `P3_SAVES`, 9 guardados reales) comprueba que: guardar sin avanzar devuelve el fichero **idéntico**; tras 30 días, guardar, recargar y 30 días más da las mismas ciudades byte a byte y el mismo reloj que seguir sin guardar.

Límites: (1) la simulación mantenía **dos copias del comerciante** (`economy.merchants` y `rep_merchants`) que sus sistemas modificaban por separado (salarios y ventas 0x98, 0x480–0x48C, 0x4B0–0x4B4); ahora `Sim::reconcile_merchants` las funde al terminar cada paso que toca comerciantes (cierre de día, ciudad, centro comercial, asedio, barcos, tareas): lo que cambió una sola copia se toma de ella y los acumuladores cambiados por las dos suman sus variaciones. En los 9 guardados reales ninguna fase cambia el mismo byte en las dos copias (`merchant_conflicts` y `merchant_double_changes` quedan en 0) y tras cada paso las copias son idénticas, de modo que el guardado ya no elige entre dos valores. Antes de esto el cierre de día sobrescribía en la copia de economía lo que las oficinas habían sumado; ese efecto ya no existe; (2) los bytes del comerciante que el formato no guarda (contadores recalculados cada día, p. ej. 0x47C) no se comparan; (3) barcos, estadísticas (`.pst` no se escribe), asedios y tareas programadas no los avanza la simulación y se conservan tal cual; (4) no se ha probado que el juego original cargue el resultado. La fusión es la reconstrucción de «un solo registro»; no se ha contrastado con el original qué sistema debería ver el valor de otro dentro de un mismo paso.

### C07, barcos en el mar (30/10/2026, en curso)

Módulo `p3_core::ships`, primer tramo (7 funciones, `x86_ship_motion.rs`; oráculo `oracle/x86_ship_motion.py`, fixture `ship_motion.json`):

| Módulo | Funciones |
|---|---|
| `motion` | `get_speed` (0x5170F0, 3.000 casos) y `set_velocity_towards` (0x517410, 3.000 casos, los 16 rumbos) |
| `order` | `remove` / `insert` / `resort` de la lista por x (0x50D040 / 0x50CE50 / 0x50D0C0, 4.000 casos con enlaces estropeados) |
| `captains` | `allocate` / `free` de la lista libre de capitanes (0x5097C0 / 0x5098B0, 2.000 casos con crecimiento del array) |

Hallazgos: el pseudocódigo de `set_velocity_towards` oculta que el primer argumento de pila es la velocidad (divisor de la distancia, constante `-128.0` en 0x673678), y a `1 - ftol(-128·d/velocidad) == 0` el original falla con #DE (Rust devuelve `Err(DivideError)`). La suma de cuadrados se limita a 31 bits en el oráculo: la raíz CRT con dominio negativo no se emula. El oráculo recrea la máquina tras cada #DE (deja la pila x87 sin retirar). Se excluyen las listas cíclicas, donde el original no termina. 13 mutaciones del código detectadas.

Segundo tramo (`x86_ship_dock.rs`, oráculo `x86_ship_dock.py`, 700 casos por función): `ship::recompute_load` sobre los bytes del barco (`ships::cargo`, puente con el `ShipCargo` de C04), `put_at_sea` (0x505C30), `set_sail` (0x5090E0) y `finish_docking` (0x519C90). Las ciudades, los comerciantes y los barcos se comparan por SHA-256 de sus bytes. Ganchos: `plan_route` (0x516BC0), el aviso de interfaz del jugador (0x44F940) y `do_notification` (0x548CA0, con sus seis argumentos). Detalle del original: en el aviso de `finish_docking` las partes altas de los argumentos de dueño y ciudad son restos de registros (la del índice del barco y la del desplazamiento del barco en el array).

Tercer tramo (`x86_ship_route.rs`, oráculo `x86_ship_route.py`, 1.500 casos por función, 8 mutaciones detectadas): `ship::route_duration` (0x5169C0), `ship::clear_route` (0x516840; `operator delete` y aviso de interfaz como ganchos) y `ship::reset_position_to_town` (0x517170; los registros de puerto de 0x6DDBB0 son de 52 bytes, no de 13).

Cuarto tramo (`x86_ship_advance.rs`, oráculo `x86_ship_advance.py`, 3.000 casos, 16 mutaciones detectadas): `ship::advance_along_route` (0x516CC0), el paso de un barco por su ruta: desgaste de vida, salida de puerto de las IA, avance entre puntos con cambio de rumbo y ritmo, llegada (con ciudad, sin ciudad o ruta de comercio) y liberación de la ruta propia. Reutiliza `get_speed` y `set_velocity_towards`. Ganchos: `find_town_at_point`, la salida de puerto (0x54AA70), la interfaz, `do_notification` y `delete`. Límite: del aviso solo se comparan los bits propios de dueño, x e y (el resto son restos de registros del original); las posiciones de puerto se leen como u32 con la parte alta a cero, y esa parte alta en los datos reales no está verificada.

Quinto tramo (`x86_ship_remove.rs`, oráculo `x86_ship_remove.py`, 1.200 casos): `ships::remove_ship` (0x507FB0), la baja de un barco: capitán a la lista libre, lista por x, lista del comerciante, listas de puerto, mar y ciudad, convoy y lista de libres. Las llamadas externas (interfaz, `clear_route`, quitar de la lista por x, ranura del pool, comerciante, convoy) salen como eventos en orden; ninguna cambia datos que lea el resto de la función. Detalle del original: sin comerciante válido el recorrido de ciudades empieza en la 5, no en la 0. Límite: no se probó un barco presente a la vez en las dos listas de una ciudad.

Sexto tramo (`x86_ship_tick.rs`, oráculo `x86_ship_tick.py`, 1.000 casos, 8 de 9 mutaciones detectadas): la **lista A de `ships::tick`** (0x506720), los barcos en puerto, con `ships::tick::tick_docked`: estados 0/10 (trámites del jugador o de la IA y cuenta de espera), 1 (cuenta atrás), 2 (salida: escena del puerto o `set_sail` y paso a la lista del mar), 3 (llegada: `finish_docking`), 4 (baja de la lista), 5, 8/9, 0xC, 0x11 (reinicio en la ciudad o baja) y el resto (baja con `remove_ship`). El oráculo ejecuta la función real con `remove_ship`, `get_speed` y la búsqueda de comerciante reales y se detiene al terminar la lista A (0x506C03); el resto son ganchos que registran sus argumentos. Límites: la lista del mar (estados 0xF, 0x11, 0x12…) y los convoyes siguen sin portar; el gancho 0x519EB0 devuelve siempre 0 (la rama que lo ve devolver 1 no se prueba); el límite exacto `hora == +0x44 + 0x100` no lo cubren los casos aleatorios.

Séptimo tramo (`x86_ship_tick_sea.rs`, fixture `ship_tick_sea.json`, 13 de 14 mutaciones detectadas; la restante es equivalente): la **lista del mar de `ships::tick`** con `tick_sea`: estados 0xA/0xB/0x16, 0xF (muerto, rutas X1/X2/Y1/Y2), 0x11, 0x12 y baja por defecto con `remove_ship`. El oráculo se detiene en 0x5070FD. Límites: el avance por ruta, la persecución, la comprobación de flota y las ranuras son ganchos guionizados con respuestas del oráculo, no código portado; el bucle de convoyes y `plan_route` siguen sin portar.

Octavo tramo (`x86_ship_tick_conv.rs`, fixture `ship_tick_conv.json`, 1.000 casos, 36 de 38 mutaciones detectadas; las dos restantes son equivalentes): el **bucle de convoyes de `ships::tick`** (0x5070FD..0x5079DA) con `tick_convoys`, completo: recorrido de la lista, convoy sin barcos, barco principal inválido y todos los estados (0 trámites del puerto, 1 cuenta atrás, 2 salida, 3 llegada, 4 paso a la lista de la ciudad, 9, 0xA, 0xB, 0xF/0x16 avance, 0x12 persecución, 0x13 y el resto). Con esto `ships::tick` queda portado en sus tres listas (puerto, mar y convoyes). Límites: unas treinta rutinas externas (`Convoy::advance` 0x502110, 0x5017F0, 0x50A3D0, 0x50A580, 0x509F80, 0x50D9E0, 0x515360...) son ganchos del oráculo con respuestas guionizadas, no código portado; el estado 0x14 tras `Convoy::advance` devuelve `Unported` porque el original lee ahí un registro sin inicializar; la llamada final a 0x5053C0 (`arg & 7 == 0`) queda fuera; las tres listas se prueban por separado, no encadenadas.

Noveno tramo (mismo fixture `ship_tick_conv.json`, con `Convoy::advance` real en el oráculo y memoria de rutas en 0x5B000000): `convoy_advance` portado completo salvo la tabla de objetos: integración de posición con la comparación `movsx` del original, descuento del contador `+0x18`, retirada de los barcos terminados, arribo de la IA, marcas finales y el tramo con ruta (0x50248A..0x502B18: siguiente punto, llegada a ciudad, ruta agotada y hundimiento). Límites: el tramo de la tabla de objetos (0x502285..0x502331) se omite porque el oráculo deja vacía la tabla de 0x6DD748; el estado 0x14 tras `advance` devuelve `Unported` (el original lee un registro sin inicializar); no se ejercitan `cv_free`, `cv_ui`, la rama de 0x5172A0 (inalcanzable con un propietario válido) ni los convoyes con propietario fuera de rango; 21 mutaciones del tramo con ruta, 5 escapan (bordes y ramas poco frecuentes).

Décimo tramo (mismo fixture): `release_convoy` (0x50A3D0) portada y ejecutada con el x86 real en el oráculo: da de baja los barcos con `remove_ship`, saca el convoy de la lista de convoyes y lo pone al frente de la lista libre. Quedan como ganchos 0x50A580 (374 instrucciones con x87), 0x5017F0, 0x509F80, 0x505C30 y las demás. 6 de 8 mutaciones detectadas; el desenganche desde el medio de la lista no se ejercita.

Undécimo tramo (`x86_ship_convoy_remove.rs`, fixture `ship_convoy_remove.json`, 1.000 casos): `remove_from_convoy` (0x509F80) portada y probada directamente contra el x86 real (el oráculo llama a la función con convoyes en todos los estados), y usada ya sin gancho en el bucle de convoyes. Ramas: estados 0/1/4 y 5 (barco a la lista del puerto), 2/3 (con o sin barco principal), 0xF (lista del mar e inserción), 0x12 (gancho 0x5017F0) y recursión al vaciar el convoy. Quedan como ganchos 0x5017F0 (320 instrucciones), 0x50A580 (374, con x87), 0x505C30 y las rutinas de interfaz.

Duodécimo tramo (mismos fixtures de convoyes): `pick_main` (0x5017F0, elige el barco principal del convoy) portada y ejecutada real en el oráculo, sin gancho, dentro de `advance`, `remove_from_convoy` y el bucle de convoyes. Hallazgos de la paridad: las comparaciones de `+0x18` son con signo (`jle`/`jg`); `advance` conserva el puntero del barco principal antiguo (`edi`) aunque `pick_main` cambie el principal, de modo que la parte de ruta usa los campos del barco antiguo pero los argumentos de los ganchos salen de `convoy+0x10` (nuevo); tras `advance` en el estado 0x12 el orden se reinserta con el principal nuevo; el oráculo no cargaba la capacidad de capitanes en `0x6DD892`. Límites: el convoy con dueño >= número de comerciantes no se ejercita (el generador mantiene dueños válidos), 0x5097C0 es un gancho, y 6 de 15 mutaciones escapan por ramas no generadas. Siguen como ganchos 0x5097C0 (ya portada como `CaptainPool::allocate`, falta enlazarla al anfitrión del tick) y las rutinas de interfaz.

`pool_ok` (0x519C40) y `dock_ok` (0x519BA0) también son código real: prueba directa `x86_ship_predicates.rs` (4.000 barcos, `ship_predicates.json`, 15 de 15 mutaciones detectadas), y el generador de convoyes ahora produce barcos que los cumplen. `lost_at_edge` (0x5029F0, convoy perdido por el borde del mapa) también es real (objeto de evento y aviso de interfaz registrados como ganchos). `state9` (0x503930) se porta y 0x501E10 resulta no tener efectos (solo lee), con el oráculo ejecutándolos reales. `put_at_sea` (0x505C30) deja de ser gancho en el cierre de barcos de `advance`: el oráculo ejecuta el código real (redirigiendo ECX al gestor del oráculo, porque el original usa el global 0x6DD7A0) y `insert`/`recompute` siguen registrándose; 4 de 4 mutaciones detectadas.

Decimotercer tramo (`convoy_drop.rs`, mismos fixtures): `dissolve_convoy` (0x50A580) portada y ejecutada real en el oráculo, sustituye al gancho `cv_drop` en los tres sitios del tick (ya no hace falta emular la escritura del enlace: `release_convoy` hace el desenganche). Reembolso por barcos de otro comerciante = `(⌊√valor⌋·distancia + 99)/100·10`, con distancia `⌊√(celda·20)⌋`; mensajes 0x32 (reembolso) y 0x31 (total). El dueño del convoy se relee tras cada baja porque `pick_main` puede reasignarlo. Los campos de los mensajes se escriben tras apilar los argumentos, de ahí los desplazamientos de 8 bytes. Límites: ver `functions.tsv`.

Decimocuarto tramo (`convoy_plan.rs`, fixture propio `convoy_plan.json`, oráculo `oracle/x86_convoy_plan.py`): `plan_destination` (0x505C90) con `first_object` (0x51A470), `free_port` (0x5159C0) y el generador local 0x46C380, sobre 3.000 casos con la tabla de puertos, el objeto de interfaz, los cinco objetos de facción y las ciudades como `PlanEnv`. El destino por defecto es (0x3D2, 0x317), o (0x220, 0x368) si el barco ya está en x = 0x3D2. Si el convoy tiene más barcos que el límite del gestor (`+0xF8`) y más de tres, se elige el puerto de una ciudad marcada (bit 0x40) según `(+0x42 & 7) + (reloj >> 2) + (principal & 1)`; si no, con pérdidas o tripulación corta va al puerto del objeto (y entra en él si ya está allí: saca el barco del orden y llama a 0x514B80); y si no, sigue al barco objetivo salvo que esté en una ciudad o en uno de tres rectángulos, caso en el que elige un punto de ruta aleatorio de una ciudad (semilla = tiempo). La suma de reservas se divide a la mitad con tope 0x7530. El argumento que el original pasa a 0x514B80 es el índice del convoy (restos de pila). Quien aporte `plan_env` al anfitrión activa el código; sin él sigue el gancho. Límites: el original no termina cuando todos los puntos de ruta coinciden con la posición del barco (Rust corta a 0x10000 vueltas); los índices de tipo de objeto y de ciudad deben ser < 40 y 16 respectivamente.

Decimoquinto tramo (tarea «IA sobre barcos reales», `ai_dock.rs`, fixture `ai_dock.json`, oráculo `oracle/x86_ai_dock.py`): `ai_docked_update` (0x518B80) con `reachable_ports` (0x518D60) y `rest_or_crew` (0x518E00). Segundo paso: `fill_crew` (0x519FC0), `needed_crew` (0x5184F0), `mark_changed` (0x517770) y `recompute_load` (0x5182B0, ya portada) sustituyen sus ganchos. Tercer paso: `choose_destination` (0x519170), `trip_time` (0x516AB0) y la distancia 0x531B70 (matriz del mundo; su tamaño es el número de ciudades). Cuarto paso: la estimación de beneficio `trade_amount` (0x52F030, `ai_trade.rs`), la mejor ciudad `best_trade_town` (0x518FF0), la compra `buy_cargo_for` (0x519920), la búsqueda de oficina `find_office` (rama de consulta de 0x5308A0) con `unload_and_sell` (0x519280), `sell_for_goal` (0x5193F0) y `sell_surplus` (0x5196B0) en `ai_cargo.rs`, y `arm_and_crew` (0x51A070) con 0x5269A0 y 0x526AA0 en `ai_arm.rs`. Ya no queda ningún gancho de comercio: `ai_docked_update` reproduce el original con 4.000 casos (barco, ciudades, comerciantes con reputación, convoyes, oficinas y capitanes). El anfitrión aporta la matriz de distancias, el factor de dificultad (`this+0x64` del objeto 0x6DE3D8) y la reserva de oficinas (`AiDockHost`). Fixtures `ai_trade.json`, `ai_helpers.json` (oráculo `x86_ai_helpers.py`) y `ai_dock.json`; mundo de prueba común en `tests/common/ai_world.rs`. Sexto paso: `nearest_port_plan` (0x51A2C0, redirección al puerto más cercano que sustituye el gancho «arrive»), 800 casos en `ai_helpers.json`. Quinto paso (`ai_depart.rs`, fixture `ai_depart.json`, oráculo `oracle/x86_ai_depart.py`): la salida `ai_depart` (0x519EB0) con `create_convoy` (0x509B90) e `init_convoy` (0x501680); 1.200 casos con 12 de 14 mutaciones detectadas (las otras dos son equivalentes). La ampliación de la tabla de convoyes (0x509AE0) es `grow_in_place` (sobre memoria reservada). La rama que crea oficinas de 0x5308A0 no es alcanzable desde los llamadores portados (siempre pasan crear=0). En p3-sim, `LiveVoyage::step_idle_with_ai_dock` cablea `ai_docked_update` y `ai_depart` al tick naval: la IA atracada comercia y se une a un convoy en formación (`tests/ai_dock_live.rs`); el viaje de esos convoyes en el mar sigue sin conectar (hace falta planificar rutas de IA, llegadas y atraque de barcos no gestionados, con su oráculo de tick). `TickHost::ai_state2_with_world` pasa el enlace de la lista de atracados.

Combate naval (1/11/2026, nube Claude): `ships::step` (0x61A9E0, 0x61DBBF), `ships::collide` (cascos, abordaje, obstáculos, despachador de pares 0x608850), `ships::sea_update` (`SeaBattle::update`, 1.000 casos) y `ships::finish` (`SeaBattle::finish` 0x60D299, 1.500 casos, oráculo `x86_battle_finish.py`). `finish` no contiene textos del juego: pide cada uno al anfitrión por la dirección de su puntero (`FinishHost::text`) y reproduce el formato `%s/%d`; el oráculo usa cadenas sintéticas y compara cada strcpy/strcat/sprintf. Hallazgos: las tablas de tratamiento y de tipo de barco son contiguas (4 y 8 entradas); la comparación del reloj de noticias es sin signo; el mensaje guarda un puntero absoluto a su texto (`base+0x1310`). Barco de asedio (vtable 0x67AAD0) portado con paridad x86: `ships::siege` (elegir objetivo 0x6247B0, inundación 0x6258A0/0x5FFF40/0x625B00, mapa de rumbos 0x625BF0, paso 0x621770; 1.500 casos) y `ships::siege_weapons` (armas 0x624950; 2.500 casos). Límites: el daño a torres (0x60130B) y el aviso (0x611649) son ganchos del anfitrión; en el paso sobreviven 23 de 50 mutaciones y en las armas ninguna mutación no equivalente (32 de 34 muertas; las dos que viven no cambian nada observable: el índice de torre de la pasada 3 solo se compara con «sin candidato» y `own[0x130]==0` ya está implícito), con 1.200 casos aleatorios y 3.000 de borde (umbrales de distancia, ángulo y empates); la conexión de `finish`/destrucción con `p3-sim` queda para Codex. Del paso se buscaron además casos que maten los supervivientes con un fixture de 9.000 casos (no versionado): cinco mutaciones más cayeron (se añadieron esos cinco casos) y 18 siguen vivas incluso con ellos, así que son equivalentes o exigen escenarios no generados; no se afirma paridad en esos bordes.

Ganchos de C07 que quedaban abiertos, ya portados con paridad x86: `ships::convoy_motion::set_convoy_motion` (0x50A240, 2.500 casos), `ships::convoy_absorb::absorb_convoy` (0x514B80, 3.000 casos, tabla de floats 0x673A18 incluida) y `ships::convoy_report::report_to_task` (tramo de tareas 0x502285..0x502331 de `Convoy::advance`, 2.500 casos ejecutando el rango real). Los tres son funciones sueltas con ganchos del anfitrión: no se han enchufado a `ConvoyHost` ni a `convoy_advance` para no alterar los fixtures de `ship_tick_conv`; `ship::plan_route` (0x5172A0) ya estaba cubierta por `route.rs`. Reutiliza `steer`, `turn`, `heading`, `board_filter`, `boarding_step` y `artillery_strength`, ya portadas. No es el combate mar contra mar (su vtable es 0x67B0D4). El crecimiento del índice espacial (0x61A1D0) ya lo cubre `oracle/x86_mvp_collision_grow.py` (Codex). Límites: la vista (0x608550), los textos reales y la conexión completa a `p3-sim` siguen fuera; abajo se describe el subconjunto conectado en C10.

C10 (PC Sergio): el avance sin vista utiliza tambien el cierre de Claude cuando solo hay cartas 0x29 destinadas a IA o avisos 0x72; las primeras se descartan y los segundos se entregan si el destinatario es humano. Tambien se entregan informes 0x29 humanos con textos verificados. Se rechazan y revierten noticias, dialogos y piratas pendientes; los textos vacios no se publican. 71 casos admitidos sin entrega humana del corpus de cierre (51 con convoy ampliado), 1.353 rechazados sin mutar el estado y 96 recorridos x86 de actualizacion, cierre y serializacion de convoyes ampliados (`x86_battle_finish_ai_expanded_update.py`; formato y entrega de cartas interceptados). El retorno de convoy vivo sin disolver tiene 256 contrastes x86 (`x86_battle_return_convoy.py`); el recuento urbano y el destructor estan conectados para el subconjunto descrito abajo; bajas y otras variantes siguen pendientes. Sin validacion en juego.

Las cartas 0x72 de bajas para comerciantes humanos ya se insertan en la misma transaccion que el paso de combate, mientras que las cartas a IA se descartan como en el original. 96 secuencias x86 de update y finish producen los mismos campos; la sesion guarda y recarga una carta en S05 y revierte el paso si falla el pool. El generador intercepta la entrega; las 200 pruebas x86 del gestor y 32 cadenas de finish y entrega real cubren esa dependencia por separado. El informe textual 0x29 se construye con datos del EXE ingles verificado, respeta la preferencia personal y se guarda en S05; 192 secuencias x86 con informe solo o con bajas comparan hashes, y 200 casos contrastan entrega y serializador, incluyendo crecimiento y plazo cero. Se busca Patrician3.exe junto al CPR o en P3_ORIGINAL_EXE. La noticia global 0x2A por hundimiento pirata se entrega en la misma transaccion y se conserva en S05. Catorce recorridos x86 llegan al hundimiento y comparan estado, proyectiles y contenido del aviso con la entrega interceptada; el gestor de cartas se contrasta por separado en 400 casos. La sesion verifica guardado, recarga y rollback del pool. El botin de abordaje sin cambio de propietario ni efectos externos se aplica durante el tick: 192 casos x86 y una sesion con guardado y recarga; el oraculo usa precision x87 de 53 bits. La recompensa de botin, el oro, la reputacion y la carta 0x30 se confirman con el combate y S05: 192 cadenas x86, 200 entregas y una sesion con recarga y rollback. La captura comercial ejecuta 0x91 durante el abordaje, mueve listas locales y mundiales y persiste barcos, convoyes y comerciantes; 192 escenarios de handler y 192 de actualizacion encadenada coinciden con x86, con UI y dispatcher interceptados. S05 admite las cadenas locales desordenadas y un bando vacio: 192 recargas coinciden con el constructor original; dos escenarios de sesion cubren guardado y recarga, uno de ellos tambien un paso original tras recargar. Las rutas propias y compartidas del barco capturado se liberan y se retiran de la cache de viajes, con persistencia S05; 192 handlers y 192 pasos de batalla x86 contrastan las variantes, y la sesion prueba guardado y recarga. Los punteros del original se representan como identificadores opacos, no se liberan como memoria nativa. El recuento no serializado merchant+0x47C se reconstruye desde las listas como en el cargador, contrastado en 256 casos. La devolucion del capitan al pool y su cabeza se conectan a la batalla, la fabrica naval y S05: 192 handlers y 192 pasos de batalla x86, con guardado, recarga y rollback ante pools incoherentes. La captura del lider recalcula el convoy: 192 casos del handler y dos batallas x86 con convoy vacio o sustituto, tambien tras cargar y guardar en S05. Rechaza efectos de seleccion y ranuras aun no conectados. Al disolver el convoy, los hundidos humanos se conservan en estado 0x11 y se ajustan los contadores: 256 retornos y 96 destructores x86. Un convoy comercial vacio se devuelve a la lista libre: 256 retornos y 96 destructores x86. La baja automatica sin ruta ni listas auxiliares se conecta al destructor, la fabrica naval y S05; tambien devuelve su capitan, si existe: 256 retornos y 96 destructores x86 por cada variante con o sin capitan, con dos listas libres y comparacion del bloque de barcos con el serializador original. La sesion guarda y recarga ambos tipos y revierte variantes con ruta. Las demas bajas no humanas siguen pendientes. La notificacion de objetos, otras variantes de captura y la eliminacion efectiva de barcos siguen pendientes y revierten el paso. Sin partida.

Retorno posterior al combate: disolucion de convoyes comerciales vivos con o sin ruta conservada, 256 escenarios x86; recuento urbano 0x504970, 256 escenarios y 256 cadenas disolucion-recuento con saldo, listas y ciudades. Se rechazan rutas que requieren planificacion, bajas y tareas hanseaticas. El destructor sin vista ya enlaza registro de parejas, investigacion, retorno y recuento en una transaccion; 96 ejecuciones x86 conjuntas. La sesion puede cerrar este caso, guardar S05 y recargar: TEST25 coincide en rutas logicas, barcos, convoyes, listas y carga entrante tanto al cerrar directamente como al recargar primero. Siguen rechazados bajas, interfaz humana, noticias, replanificacion, algunas rutas y batallas encadenadas; sin prueba en juego.


Quedan, dentro de C07: la lista del mar y los convoyes de `ships::tick`, `plan_route` (0x516BC0 con `find_route` de A03; exige decidir cómo guarda Rust las rutas por barco, hoy puntero opaco), `ships::tick` y sus estados, `Convoy::advance`, persecución y combate (SeaBattle), visibilidad.

### C06, IA comercial (26/10/2026)

Módulo `p3_core::ai` con 81 funciones del comerciante y su IA (10 del turno, 28 de reputación, 2 de pujas, 1 de carga, 2 de guardado y carga con sus subfunciones, 3 de grupos de barcos, 28 métodos pequeños y 3 del mapa), todas con oráculo x86 y prueba (`functions.tsv`):

| Prueba | Funciones |
|---|---|
| `x86_ai_ships.rs` | `mark_ship_for_arming` (0x4F5080), `assign_ship_home_towns` (0x4F5130) |
| `x86_ai_cash.rs` | `raise_cash_or_go_bankrupt` (0x4F4E50): operación 0x29, quiebra 0x65 y consumo de `rand` |
| `x86_ai_stock.rs` | `update_office_trade_stock_targets` (0x4F4F80) |
| `x86_ai_guild.rs` | reputación entera (0x4F9580), cuota de gremio (0x51B7E0), `try_join_guild` (0x4F4A20) |
| `x86_ai_housing.rs` | `choose_housing` (0x4F4B40): puntuación de grupos de vivienda y operación 0x22 |
| `x86_ai_turn.rs` | `run_ai_turn` (0x4F4940): contadores de tarea y orden de las seis acciones |
| `x86_ai_work.rs` | `queue_building_work` (0x4F5A20): razón cruzada entre listas de edificios y operación 0x28 |
| `x86_ai_rep.rs` | 28 métodos de reputación y prestigio de `Merchant` (0x4F8690..0x4F9550): ventas/compras a ciudades, eventos sociales, préstamo, entrega de barcos, candidatura, epíteto, factor base |
| `x86_ai_bids.rs` | Factor base de reputación (0x4F42B0) y límite de puja en subastas de barcos (0x4F4400); puntuación y valor de barco como ganchos |
| `x86_ai_cargo.rs` | Retirada de mercancías de los barcos del comerciante en una ciudad (0x4F5450); recuento de bodega como gancho |
| `x86_ai_seize.rs` | Embargo de bienes por deuda: oro, oficina y barcos (0x4F6570); oficina, valoración, mensajes y avisos como ganchos |
| `x86_ai_bankrupt.rs` | Reinicio de un comerciante de IA por bancarrota: misiones, oro, identidad, nombre y apellido (0x4F5D80); avisos, misiones, mensajes y reserva como ganchos |
| `x86_ai_candidate.rs` | Candidato a capitán de la taberna para un comerciante de IA (0x4F69B0); nombre, rand y cola de operaciones como ganchos |
| `x86_ai_daily.rs` | Actualización diaria de reputación y valor del comerciante: libro semanal, barcos, oficinas, decaimiento y rangos (0x4F7BB0); deudas, valor de barco, coste, oficina y cola como ganchos; sin interfaz |
| `x86_ai_merchant_io.rs` | Guardado y carga del comerciante (0x4F4010 / 0x4F4100): tramos fijos deducidos del x86 por el oráculo (`merchant_layout.rs`, generado), umbrales de versión 0x5D/0x5E/0x6B/0x6E/0x74/0x7E y diez listas de palabras con recuento; asignador como gancho |
| `x86_ai_groups.rs` | Grupos de barcos del comerciante: fijar un grupo (0x4F5500), añadir barcos con órdenes 0xBF (0x4F56C0) e índice de comerciante (0x5303E0); asignador y cola de órdenes como ganchos |
| `x86_ai_misc.rs` | 20 métodos pequeños de Merchant (0x4F36C0..0x4F9400) y 2 del objeto de mapa (0x534960, 0x534990): identidad inicial, compra de conocimiento del mapa (0x4F8560), marca de puntos del mapa vistos, puntuaciones acotadas, reputación fija y local, grupo y poda de barcos, máscara de presencia, reserva de marineros, nombres, puntuación total de reputación y borrado de libro; nombres y listas como ganchos |

Límites: las llamadas a otras partes (buscar/crear oficina, buscar celda del plano, cola de operaciones, tarea
programada, consultas 0x4DAFE0/0x4DAF10) son ganchos que registran sus argumentos. `queue_building_work` lee una
ranura de pila sin inicializar en su primera comparación; Rust la recibe como `residue`. No están reimplementadas las
partes de capitanes, rutas y piratas (dependen de A03). Nada validado en partida.

Reputación (`ai::reputation`): el comerciante son sus bytes originales (`Merchant`, 0x650); las ciudades, el recuento y los tres prestigios ajustables (`0x00672DD2/DD4/DDE`) son entrada. Las operaciones inválidas del x87 dan el NaN indefinido `0xFFC00000`. Sin cubrir completamente: ciudades más allá de 14 (campos solapados); para mercancías 20–23 en `bought` se reproducen lecturas adyacentes del original, no pesos semánticos; `0x4F8560`, `0x4F93C0` y `0x4F9400` (llaman a objetos de mapa y gremios) siguen sin portar.

**J04, bloque28 (PC de Sergio):** seek por byte comprimido y centinela -2
cotejados con x86 en 132 secuencias/528 lecturas propias, 63 parejas y 36
secuencias de un MP3 real distinto. Se conserva posición física, PCM pendiente
en -2 y los bancos; -1 se acreditó en bloque29. Faltan transporte HSTREAM, entrada progresiva
y conexión al reproductor; Symphonia sigue activa y no hay paridad de juego.
Detalle en [`p3-audio/README.md`](p3-audio/README.md).

**J04, bloque29 (PC de Sergio):** seek temporal HSTREAM con proveedor abierto
conectado a MP3 en memoria, 63 secuencias y 252 lecturas comparadas con x86;
-1 y transiciones de pausa contrastados. Sample, archivo y servicio se simulan;
no hay servicio real de búferes, salida física ni juego. Symphonia sigue activa.
Detalle en [`p3-audio/README.md`](p3-audio/README.md).

**J04, bloque30 (PC de Sergio):** servicio de tres búferes, entrega, fin,
bucles y pausa en el dominio acotado contrastado con x86; `-2` enlazado con
el proveedor MP3 nativo. El bloque31 conecta esos búferes al decoder
progresivo; el sample físico sigue pendiente; Symphonia sigue activa.
Detalle en [`p3-audio/README.md`](p3-audio/README.md).

**J04, bloque31 (PC de Sergio):** decoder progresivo conectado a los búferes
HSTREAM y contrastado con x86 en bucles, seek y EOF, incluidos dos recorridos y
cuatro seek de un mismo efecto real. El bloque32 contrasta el callback
original y conecta un reproductor progresivo opcional. Symphonia sigue predeterminada.
Detalle en [`p3-audio/README.md`](p3-audio/README.md).

**J04, bloque32 (PC de Sergio):** ruta MP3 progresiva seleccionable en
`NativeEffects` y mezcla fuera de línea; callback, arranque, pausa y underrun
aisladamente cotejados con x86. Fuente en memoria, driver simulado; sin salida
física ni juego. La ruta predecodificada sigue como opción predeterminada.
Detalle en [`p3-audio/README.md`](p3-audio/README.md).


**J04, bloque33 (PC de Sergio):** MP3 asíncrono opcional desde disco o CPR,
cola acotada y underrun con silencio en mezclador offline. Productor bloqueado y
cuatro periodos de 10 ms contrastados; la recuperación reproduce PCM x86,
incluido un MP3 comercial desde disco (226 lecturas). El proveedor original
produce PCM con trama incompleta en 9 escenarios: 8 difieren del PCM continuo.
Rust espera los bytes y por tanto esa situación no es paridad. Seek previo y
bucle finito se comparan en casos concretos; quedan reloj, dispositivo y más
secuencias asíncronas. Sin validación de juego ni garantía de tiempo real duro.
Ver `p3-audio/README.md`, bloque33.


**J04, bloque34 (PC de Sergio):** política experimental para primera trama corta
con relleno cero y guardas: 9 secuencias/165 llamadas x86 coinciden, recarga
incluida. Seek asíncrono 24 secuencias/602 lecturas; bucle de archivo completo
6/352, con diferencias en los tres casos con bucle por `-2` anticipado del
HSTREAM original. Falta reproducir ese orden conectando el servicio al productor,
ampliar otros underruns y conectar reloj/dispositivo. Ver `p3-audio/README.md`,
bloque34. No hay paridad general ni juego.


**J04, bloque35 (PC de Sergio):** `AsyncNativeHstream` conecta productor,
servicio y mezclador; `-2` se ejecuta durante la precarga y conserva el PCM
pendiente. Callback sample x86: 21 secuencias/1.784 lecturas con dos tamaños de
búfer y hasta tres pasadas; un efecto comercial desde disco 2/1.073, más 226 sin
bucle. Seek 24/602, pausa y lectura bloqueada probados en el dominio documentado.
Servicio coordinado y driver simulado; sin paridad de planificación general ni
juego. Ver `p3-audio/README.md`, bloque35.

**C04/C10 — empleados de negocios:** recuento conectado a contratación y
producción; 400 escenarios de una oficina durante un día coinciden con x86,
93 contratan. S05 guarda y recarga selectivamente una ciudad; no es un
guardado completo. Otros puertos de C10 siguen sin conectar. Los resultados
anteriores de C10 se obtuvieron con el recuento inactivo y no prueban el
estado actual. Ver `research/systems/economia/C04_EMPLEO.md`.


**C07, estado del tick naval (31/10, nube Claude).** Código real con paridad x86 (suites `x86_ship_tick*`, `x86_ship_convoy_remove`, `x86_ship_predicates`): las tres listas de `ships::tick`, el bucle de convoyes, `advance`, `release`, `remove_ship`, `pick_main`, `dissolve`, `put_at_sea`, `pool_ok`, `dock_ok`, estado 9 y convoy perdido por el borde. Desde el decimocuarto tramo `0x505C90` (destino del convoy, `ships::convoy_plan`) también es código real con paridad, con sus globales en `PlanEnv` (el anfitrión debe aportarlos; sin ellos sigue el gancho `cv_505c90`) y los ganchos `0x5172A0`, `0x514B80` y `0x50D040`. Siguen como ganchos, por este orden de importancia para la simulación: (1) los efectos de `0x514B80` (absorber el convoy en el objeto del puerto) y `0x5172A0` (ruta hasta el destino); (2) `0x50BC40` y `0x503CF0` (maniobras de convoy; Codex ya tiene `begin_departure`/`set_half_speed` en `p3-sim`); (3) escena, mensajes e interfaz (`0x503230`, `0x503B50` con azar de la CRT, `0x4D6530`, `0x4885A0`); (4) IA, persecución y flota (Codex, `p3-sim`). El segmento de la tabla de objetos de `advance` (0x502285..0x502331) sigue omitido. No tocar `p3-sim/navigation.rs` desde esta línea: es de Codex.

C10 (PC Sergio): al disolver un convoy se liberan tambien las rutas propias y compartidas de barcos comerciales automaticos hundidos; la sesion actualiza la cache y S05 comprueba la ruta anterior antes de compactar la ranura. Son 256 retornos y 96 destructores x86 con rutas, capitan y los dos pools, con asignaciones y servicios externos interceptados. Siete recorridos de sesion incluyen rutas propias y compartidas, guardado, recarga y comprobacion de conflicto; el bloque naval guardado se compara con el serializador x86. Siguen pendientes listas urbanas y de comerciante, NPC y variantes restantes de combate, flota y reloj. No es paridad de juego ni prueba en partida.

C10 (PC Sergio): se admite la baja de un hundido presente en las listas urbanas +0x81C/+0x81E. El enlace del convoy se invalida antes de remove_ship, como en 0x0050C886: sin ese orden la cabeza de ciudad era distinta. Se comparan 128 retornos y 96 destructores con x86, y cuatro escenarios de sesion guardan y recargan ambas listas en dos ciudades. El recuento externo del primer corpus esta interceptado; no se ha validado en partida. Quedan grupos de comerciante, NPC y variantes restantes.

C10 (PC Sergio): la baja de un comerciante automatico filtra tambien sus once grupos de barcos y conserva los diez grupos serializados en S05. Se reconstruyen sin punteros x86, se preservan las marcas de recuento y se detecta si otra transaccion modifica la lista guardada. Coinciden 128 retornos y 96 destructores x86; los dos comerciantes de cada destructor se comparan con su serializador original. Cuatro variantes de sesion combinan grupos, rutas, listas urbanas y recarga. El undécimo grupo es transitorio en el formato original. Persisten limites de NPC, interfaz y demas variantes de combate; sin prueba en partida.

C10 (PC Sergio): al disolver un convoy, la mision de captura 0x32/2 recibe los hundidos usando el capitan anterior a la liberacion. Coincide con 256 llamadas x86 y 96 destructores; cuatro variantes de sesion comprueban exito, fracaso y mision sin destinatario al recargar. Se aceptan bajas no comerciales sin banda, sin recompensa o con recompensa al comerciante cuando no tienen capitan; 256 retornos y 192 destructores x86 y cuatro escenarios S05 cubren esas variantes. La recompensa y la carta cuando hay capitán se conectan como se describe más abajo. Los corpus usan ejecucion x86 aislada y fronteras interceptadas; sin prueba en partida ni paridad total.

C10 (PC Sergio): el callback 0x00515F00 desenlaza la baja NPC de las cinco bandas, incluyendo cabeza, predecesor y sucesor. Se sincronizan las copias de flota y el estado de misiones; las cabezas y enlaces se guardan en S05 con deteccion de conflictos. El destructor coincide en 96 casos x86, con serializador original; siete escenarios de sesion comprueban las cinco bandas y sus posiciones. Los punteros x86 se representan mediante tokens temporales; se rechazan cadenas ciclicas y punteros compartidos. Sin prueba en partida.

C10 (PC Sergio): la baja NPC con capitán aplica notoriedad y reputación social al comerciante acreditado y entrega la carta 0x54 si es humano; la IA no la recibe. La carta, el pool, la misión, la banda, el capitán y la flota se confirman en la misma transacción S05; si falla el pool se revierte el paso. Se contrastaron 96 destructores x86 y 200 entregas y serializadores originales, además de cuatro sesiones humanas y de IA con recarga. La presentación de la interfaz está interceptada; no hay prueba en partida ni paridad total.

C10 (PC Sergio): el contraste encadenado `urban_two_merchants_and_ships_daily_chain_thirty_days` amplía de ocho a treinta despachos diarios el reloj original, el tick de una ciudad, sus oficinas y obras, el planificador urbano y el día de dos comerciantes con costes de tripulación. Se comparan cada día el calendario, las operaciones, la ciudad, los comerciantes y los serializadores de ciudad, oficinas, negocios y cartas; la sesión Rust persiste en S05. El escenario parte de una partida local modificada para el ensayo y ejecuta x86 aislado: los demás callbacks del reloj y servicios indicados en `oracle/x86_c04_clock_transaction.py` están interceptados. No contrasta el movimiento de flota ni todo el estado de una partida, ni supone prueba en el juego. La variante anterior de ocho días sigue pasando.

C10 (PC Sergio): al activar la flota completa, la reconstruccion de la sesion conserva el estado CRT explicito, que S05 no serializa. Una prueba nativa avanzo treinta dias de `visibility/save/Ein/AUTO.pat` con flota y resolvio batallas, pero ignoro la solicitud original `open_for_player(1)`, que tambien coloca barcos y genera obstaculos. Por tanto ese recorrido no acredita una simulacion fiel; la via segura se detiene al encontrar esa solicitud. Se cubren ademas tres estados CRT al activar la flota. No se abrio el juego ni se comparo el recorrido global con x86. El ejecutable `p3-sim --fleet-player` aun se detiene ante el primer encuentro porque no usa esa via de sesion; falta conectarla y contrastar el estado global original.

C10: `cargo run -p p3-mvp --example c10 -- GUARDADO.pat ARCHIVO.cpr DIAS ESTADO_CRT [SALIDA.pat]` proporciona una entrada sin interfaz con flota y resumen JSON; distingue el hash parcial de ciudades y comerciantes del SHA-256 de la partida serializada. El estado CRT va por separado porque S05 no lo contiene. La salida opcional se crea sin sobrescribir. Rechaza encuentros con dialogo, operaciones o `open_for_player` sin aplicar, en lugar de continuar con un estado incompleto. En el guardado local AUTO avanza un dia y guarda/recarga; un intento de treinta dias se detiene ante el primer `open_for_player`. Esta dependencia tambien coloca barcos y genera obstaculos (0x0060F479), por lo que no equivale solo a una vista. El resumen separa los callbacks urbanos sin implementar y los callbacks auxiliares invocados; estos ultimos incluyen marcas como ships_tick incluso cuando la flota esta conectada. Faltan la integracion naval y el contraste global de los treinta dias con el original.

C10 (PC Sergio): los 15 centros comerciales de expedición ejecutan ahora su tick en la fase original del reloj, usando la población viva de las ciudades. Se actualizan existencias y demanda y se retiran contratos vencidos o sin cantidad; los registros se guardan en S04/S05 con detección de conflictos y se reconstruyen al cargar. Son 384 escenarios x86 de `0x0052D7D0` y su serializador `0x0052BB30`, doce de ellos encadenados durante treinta días; se comparan los registros en cada paso y la baja de contratos. Una prueba adicional encadena el reloj Rust, el resultado x86, guardado y recarga de una partida local, e incluye rollback por conflicto. La interfaz de un centro abierto y su aviso no se presentan en la vía sin pantalla; los tipos distintos de 0, 1 y 2 y el formato no admitido de ofertas se rechazan. Sin partida ni contraste global de treinta días.

C10 (PC Sergio): el reloj actualiza el historial urbano S04: banderas por día, ocho ranuras de comerciantes humanos, oficinas conocidas y propagación de edades mediante la matriz de distancias. El guardado conserva la cola de centros comerciales y detecta conflictos en el historial. Hay 160 escenarios x86 de `0x00535450` y su serializador `0x00535160`, ocho de ellos durante 30 días, incluidas cadenas de varias oficinas. Una prueba adicional conecta el estado de una partida local, cadenas de oficinas y matriz con el resultado x86, escritura, recarga y rechazo de conflictos. El asignador y los accesos a oficinas y distancias del oráculo están interceptados; no es contraste global ni prueba en partida. Se rechazan cadenas cíclicas y formatos incompatibles.

C10 (PC Sergio): si se aporta el fichero de estadísticas asociado con `--pst ENTRADA.pst`, el reloj actualiza las estadísticas diarias de ciudades y comerciantes, incluidos los cierres semanales, mensuales y anuales. Para escribir una partida con estadísticas se exige `SALIDA.pat --out-pst SALIDA.pst`; ambas salidas se crean sin sobrescribir archivos existentes y el estado CRT debe conservarse por separado. Sin `.pst`, el informe registra esta fase como inactiva. El gestor `0x0054DE90` y el serializador coinciden con x86 en 48 escenarios, cuatro de 30 días, con asignación y reputaciones interceptadas; también coinciden PST, ciudades y comerciantes en tres fechas con datos completos de una partida local, usando las funciones de reputación originales. El ensayo no ejecuta los demás sistemas en el mismo emulador ni demuestra paridad global de 30 días. No se ha probado en partida.

C10 (PC Sergio): con la flota viva, el astillero público y los negocios privados de cada ciudad avanzan las colas de construcción y reparación durante el tick urbano. Los barcos, entradas, listas, contadores y cartas se incorporan a la sesión y a S05; las peticiones de escena y notificación quedan registradas, sin presentar. La transacción reutiliza los corpus x86 de botaduras y reparaciones, y cuatro escenarios sobre una partida local fuerzan obra o reparación pública, terminada o incompleta; otros dos fuerzan obra privada. Todos comprueban reloj, guardado y recarga. El tick privado y los serializadores se compararon además con el original en 16 escenarios separados. Las ejecuciones x86 son aisladas y sus callbacks de presentación están interceptados; los recorridos Rust sobre el reloj no son una cadena completa ejecutada en el original. Sin flota viva, una cola activa sigue registrada como no implementada; falta presentar los eventos. No se ha probado en partida.

C10 (PC Sergio): de `siege_record_daily_update` están contrastados por separado el desgaste de buena voluntad y la decisión de emitir 0x8E (3.000 escenarios x86, con las cuatro ayudas interceptadas), el envío de excedentes (2.400 escenarios), la selección mensual de demandas (1.800 escenarios con agregado mundial interceptado) y dicho agregado (600 escenarios, solo `memset` interceptado). También coinciden en 192 casos el agregado y la selección encadenados en una ejecución x86. El envío usa el umbral por ranura de demanda y las existencias por mercancía. Cuando una demanda queda vacía, el original escribe niveles a partir de bytes de pila no inicializados: el puerto los recibe explícitamente y no les atribuye un valor. Todavía no están conectados al reloj y falta despachar la operación que inicia el asedio. Sin contraste global ni prueba en partida.

C10/S05 (PC Sergio): el lector del mundo admite registros de asedio de longitud variable con listas de acuerdos y solicitudes, siempre que no haya escena activa; antes suponía 20 bytes por ciudad. Se comprobaron 600 registros con el lector y escritor x86 (`malloc` y `free` interceptados), incluidos ambos tipos de lista, y se insertaron registros en una partida local para verificar que las secciones posteriores se conservan al guardar y recargar. La escena activa se rechaza explícitamente y la conexión diaria de las listas sigue pendiente. No se ha abierto el juego.

C10 (PC Sergio): el procesamiento de acuerdos de entrega está contrastado en 1.600 escenarios x86, con gestores y oficinas originales y `free` interceptado. Se comparan los nodos restantes, bienes, pagos, ingresos y buena voluntad; se rechazan cadenas cíclicas sin modificar el estado. Las solicitudes vencidas coinciden en 1.800 escenarios con el buzón interceptado; la entrega de su carta 0x85 coincide en 600 escenarios con `add_message` original. Además, 96 transacciones ejecutan encadenados el lector, tick y serializador x86 con las ayudas originales, comparando registros, ciudades, oficinas, comerciantes, buzón, CRT y decisión de 0x8E. El heap, CRT, `memset` y el despacho de operaciones se interceptan; los bytes de pila indeterminados se capturan como entrada. En ese momento quedaban la conexión al reloj, el despacho de 0x8E y las escenas activas; la continuación se describe debajo. Sin prueba en partida.

C10 (PC Sergio, continuación): los registros de asedio sin escena se ejecutan desde el reloj y se guardan en S05 con detección de cambios simultáneos. Se sincronizan existencias de oficinas, ciudades, cuentas, cartas y CRT; una operación 0x8E que requiera escena se conserva en cola y detiene el despacho porque la creación de la escena aún no está implementada. Una transacción preparada sobre datos de una partida local se comparó con el lector, tick y serializador x86 encadenados (heap, CRT y despacho interceptados), y el resultado se guardó y recargó; también se avanzó un día completo en Rust con esa partida. No son un contraste del reloj completo ni de 30 días. En la selección mensual, si hay niveles dependientes de pila indeterminada, se exige que el anfitrión aporte esos bytes; ante un error durante el asedio se bloquean el avance y el guardado porque el tick urbano ya se ha ejecutado parcialmente. Escenas activas y 0x8E siguen pendientes. Sin prueba en partida.

C10 (PC Sergio): de 0x8E se implementó la puerta inicial: descarta la operación si la ciudad no comercia o no reúne tres piezas de defensa terminadas o en obra. Se contrastaron 2.000 escenarios x86, interceptando la asignación y la creación de escena. Cuando la puerta admite el asedio, la operación se conserva en cola y el despacho se detiene: la escena, su simulación y sus efectos siguen pendientes.

C10/S05 (PC Sergio): una escena de asedio guardada se puede conservar como datos opacos. La longitud variable coincide con 600 serializaciones x86 del registro y la escena (`memcpy` interceptado); una partida local preparada se vuelve a leer sin perder los registros y secciones posteriores. En ese punto aún no se reconstruía ni simulaba la escena; el comportamiento posterior se describe abajo. No se ha probado en partida.

C10/S05 (PC Sergio): los campos persistidos de la imagen de `TownSiege` tienen codificador y lectura parcial en Rust. Se compararon 600 escrituras x86 y 300 lecturas de escenas inactivas; en las lecturas se interceptan el reset y la copia, y se verifican por separado los tres bytes que el original ajusta al final. El lector parcial no ejecuta el constructor ni restaura estructuras auxiliares y no permite simular escenas activas.

C10 (PC Sergio): una escena terminada sin cuenta atrás ni atacantes se limpia desde el reloj, retirando su registro y la bandera urbana de asedio. Se comparó el tick y reset original en 1.200 casos x86 sin vista ni punteros auxiliares (`free` interceptado). Las 96 transacciones encadenadas incluyen también escenas terminadas y su posterior limpieza; el guardado y recarga se comprobaron en una partida local preparada. Las escenas activas o con espera siguen sin simularse y detienen el avance antes de cambiar el reloj.

C10 (PC Sergio): el cálculo inicial de atacantes y milicia de `town_siege_begin` coincide en 1.800 escenarios x86, incluidos desbordamientos, dificultad y límites de unidades. El oráculo detiene la función antes de la primera carta e intercepta la búsqueda del comerciante y el nivel de muralla. Ese contraste no incluye la escena completa, los avisos ni la elección de puertas; la continuación se describe abajo.

C10 (PC Sergio): la ruta 0x8E para una ciudad sin murallas quedó conectada inicialmente cuando no requería ventana para el jugador ni la petición adicional 0x8B. Se compararon 1.200 inicios aislados y 300 operaciones encadenadas con constructor, inicio, buzón y serializador x86; la ruta del buzón de aviso 0x53 se probó además en 400 casos. Las pruebas interceptan búsqueda de comerciante, nivel de muralla y asignación; la comparación aislada también intercepta el buzón y no incluye vista. Una transacción preparada se guardó y recargó. Las demás rutas de inicio se conservan en cola; el avance de la escena activa sigue pendiente y se rechaza antes del tick.

C10 (PC Sergio): la emisión y el despacho de la petición 0x8B también están conectados. Coinciden 2.000 decisiones y estados CRT, 1.600 inserciones y 600 cartas aisladas, 400 peticiones con buzón y serializador y 300 recorridos encadenados desde 0x8E hasta 0x8B en una máquina x86. El asignador deja dos bytes del nodo sin inicializar: el anfitrión los suministra por cada petición y, si faltan, la operación permanece en cola sin modificar el estado. El cálculo de la fecha textual conserva la aritmética firmada y el mes de 1 a 12 del original. La ventana para el jugador y la simulación de la escena activa siguen pendientes; las pruebas no abren el juego.

C10 (PC Sergio): en modo explícito sin vista, el reloj ya avanza la marcha de asedios activos sin murallas y sin torres mientras no se alcance un umbral de saqueo o salida. Se contrastaron 1.800 ticks x86 y se ampliaron las 96 transacciones encadenadas con lectura, tick diario, marcha y escritura originales (nivel de muralla, heap, CRT y despacho interceptados). El reloj comprueba cada tick antes de ejecutarlo y se detiene antes del primer caso no cubierto; una partida preparada se guardó y recargó en ese límite. El resto del asedio activo y su presentación siguen pendientes.

C10 (PC Sergio): la reposición de paciencia 0x634C50 y el marcado de solicitudes de todas las ciudades coinciden en 2.000 escenarios x86 sin dependencias interceptadas. En una rama, el valor de retorno conserva los 16 bits altos de EAX de entrada; se pasan explícitamente al puerto. La rutina aún no está conectada a la transición de saqueo.

C10 (PC Sergio): el vencimiento de solicitudes y la entrega de la carta oficial 0x85 se contrastaron por separado en 1.800 y 600 escenarios x86, respectivamente. Se comprueban la lista restante, el pago, el acumulador del comerciante y los campos y preferencias de la carta; el buzón se intercepta en la primera prueba y la búsqueda de comerciante y el heap en la segunda. La división por cero del original se rechaza sin modificar la transacción. Aún falta ejecutar el recorrido completo de todas las ayudas en el mismo emulador y conectarlo al reloj y S05.

C10 (PC Sergio, estadísticas parciales): portadas la captura urbana `0x0054D1D0`, su promedio `0x0054D390`, la captura de comerciante `0x0054C520` y su suma y promedio `0x0054C810/0x0054C940`. Coinciden respectivamente en 160, 160, 200 y 240 escenarios x86; los accesos a reputación del comerciante se suministran al oráculo como entradas explícitas. La captura reinicia los acumuladores que borra el original. Corrección respecto de la descripción anterior: en el promedio urbano la milicia se acumula sin dividir; poblaciones y satisfacción sí se dividen. Estas rutinas todavía no están conectadas a los históricos ni al reloj C10, y no hay prueba en partida.

C10 (PC Sergio, estadísticas parciales): el constructor y avance de los anillos de comerciantes `0x0054CA20/0x0054CE20` coinciden con 64 escenarios x86 de cursores, límites semanal/mensual/anual y años existentes, usando muestras suministradas al oráculo. Aún faltan el gestor diario completo, la entrada y salida del `.pst`, y el contraste encadenado con el reloj. No se ha probado en partida.

C10 (PC Sergio): `0x633DB0` sin convoy coincide en 1.200 escenarios x86; se contrastó además el umbral de saqueo de la escena sin vista ni murallas en 300 ejecuciones encadenadas que incluyen el tick original, saqueo, carta 0x38, reposición de solicitudes y serialización de dos ciudades (`memcpy` y liberación de carta interceptados). Las 400 cartas 0x38 también coinciden por separado. La transición aún no está conectada al reloj; las vistas, murallas y convoyes quedan fuera. Sin validación en partida.

C10 (PC Sergio): el reloj sin vista conecta el umbral de saqueo territorial sin convoy con ciudades, oficinas, solicitudes y buzón S05. Además de los 300 contrastes de escena, 96 transacciones encadenan lectura, actualización diaria, escena y escritura x86 (32 con saqueo; heap, CRT, despacho y nivel de muralla interceptados). Una partida preparada se guarda y recarga; se detectó y corrigió una copia de oficinas libres que las hacía parecer ocupadas durante el planificador. La siguiente fase de retirada y los asedios con vista o murallas siguen sin simularse y el avance se detiene antes del límite. Sin contraste de 30 días ni prueba en partida.

C10 (PC Sergio): la retirada sin vista ni murallas, desde el último tick de marcha hasta eliminar la escena, coincide en 600 secuencias x86 de tres ticks y en 96 transacciones con actualización diaria (32 con retirada). Se guarda y recarga una escena en retirada. El estado vivo `+0x79` no figura en S05: al cargar se reconstruye a partir del último saqueo y el reloj; en el paso que lo necesita se admite solo el caso acreditado como activo. El caso reconstruido como inactivo, las murallas y la vista siguen pendientes y se detienen antes de avanzar. No hay validación en partida.

C10 (PC Sergio): el indicador `+0x79` se conserva ahora como estado vivo y se reconstruye al cargar, sin modificar los bytes opacos de escenas no simuladas. La lectura coincide en 1.200 escenarios x86; la retirada con indicador inactivo y daño de milicia coincide en 1.500 ticks, y se contrastaron 96 transacciones con ambos valores. La ruta sigue limitada a ausencia de vista y murallas. Las pruebas antiguas `pumped_real_save` de dos y 30 días fallan porque no configuran las regiones de `StadtDaten.ini`; no acreditan paridad de 30 días. Sin prueba en partida.

C10 (PC Sergio): las pruebas de guardados y reloj configuran ahora el CPR inglés y el estado CRT explícito. Pasan 43 pruebas de `real_saves`, su prueba separada de determinismo de 30 días sobre nueve guardados y las pruebas de dos y 30 días de `pumped_real_save`. Son avances Rust parciales sin activar la flota completa y no contrastan 30 días con x86. El ejemplo C10 con flota completa sigue interrumpiéndose en `open_for_player`, frontera de C07 pendiente de integración; no se ha abierto el juego.

C10 (PC Sergio): el reloj original `0x530E80` y el Rust coinciden durante tres recorridos de 30 días (529.410 llamadas ordenadas en total, con cambio de año, tamaño habitual de ciudades y mundo vacío). Se comparan calendario, tick, cuenta atrás, límite de oro, recuento y digest de todas las llamadas; los sistemas llamados están interceptados, por lo que no es una comparación de la simulación completa ni de una partida. La flota completa aún detiene el recorrido en `open_for_player`.

C10 (PC Sergio): recuperada la comparación encadenada de 30 días de `live_chain`, con cuatro variantes y cotejo diario contra el reloj, ciudad, dos comerciantes, oficinas, obras y serializadores originales. Los asedios se preparan inertes porque el oráculo los intercepta; el tick de movimiento de la flota y otros callbacks también están interceptados. Los barcos presentes intervienen en los cálculos económicos de los comerciantes. Pasan también las regresiones de ocho días y del planificador de murallas. No es aún el contraste global de una partida con flota completa ni una prueba en juego.

C10 (PC Sergio): portada la selección de puerta de un asedio con murallas, con defensas en dos listas, proximidad de torres y desempate por población. Coincide en 1.800 inicios x86 hasta la llamada de construcción de la lista de torres; se interceptan el nivel de muralla y la carta. Las listas cíclicas o sin puerta se rechazan explícitamente. La selección aún no inicia ni simula la escena amurallada; no se ha probado en partida. En otro guardado local, el intento C10 con flota se detiene en la frontera naval `cv_514b80`.

C10 (PC Sergio): la reconstrucción de torres de la puerta elegida coincide con `0x62B2B0` en 1.800 casos x86, 816 con torres terminadas cercanas. El reloj de disparo y los bytes no escritos del montón son entradas explícitas; se intercepta `realloc` y se comprueban tamaño, contenido y recuento. La escena amurallada y su conexión al reloj siguen pendientes. Sin validación en partida.

C10 (PC Sergio): la geometría de puerta `0x632780` coincide en 1.500 casos x86, incluidas las cuatro clases de puerta, las dos ramas de la rejilla y los límites de coordenadas; no se interceptaron dependencias. Una rejilla truncada se rechaza sin cambiar la imagen. Aún falta encadenar la escena amurallada completa y conectarla al reloj; sin prueba en partida.

C10 (PC Sergio): la lista de negocios fuera de murallas `0x525220` coincide en 1.500 casos x86 con oficinas y planos sintéticos; `malloc` está interceptado. La reserva original solicita dos bytes por registro pero también escribe el contador: en 480 casos del corpus se escribió fuera de la reserva, incluidos tamaños cero. El puerto requiere explícitamente los bytes posteriores necesarios, informa del desbordamiento y rechaza ciclos o memoria ausente; no se fuerza una escritura fuera de un búfer Rust. Falta integrar esta lista en el inicio amurallado; sin prueba en partida.

C10 (PC Sergio): la selección y el orden de objetivos exteriores de `town_siege_begin` coinciden en 1.800 escenarios x86 con empates, población con signo y coordenadas extremas. Se interceptan nivel de muralla, carta, torres, geometría, lista exterior y asignación; el contraste se detiene antes de las duraciones. Las entradas truncadas o casos que dependerían de un registro previo se rechazan. Falta encadenar el inicio y simular la escena amurallada; sin prueba en partida.

C10 (PC Sergio): las duraciones amuralladas y los hitos por negocio coinciden en 2.000 escenarios sin objetivos y 1.800 con objetivos; se ejecuta el cálculo x87 original y se interceptan sus ayudas previas. El puerto admite puertas dentro del mapa 0..127 × 0..225 y conserva los recortes de 16 bits y límites del original. Falta encadenar el inicio, el cálculo final de fuerzas y la escena viva; sin prueba en partida.

C10 (PC Sergio): la puntuación de fuerzas amuralladas y la selección de torre activa coinciden respectivamente en 1.800 escenarios x86 cada una. En 70 casos el original recorrió más entradas de las que rellenó al filtrar torres en obra; esas entradas provienen del montón y pueden afectar a la puntuación. El puerto recibe la cola explícitamente y rechaza índices inválidos. La selección de torre se contrastó con listas válidas y población no negativa. Realloc y otras ayudas del inicio se interceptaron en el contraste de fuerzas; la selección de torre se ejecutó sin interceptar dependencias. Falta el inicio completo y su ejecución; sin prueba en partida.

C10 (PC Sergio): el inicio amurallado sin vista se ha encadenado en Rust y cotejado con 400 ejecuciones completas de `town_siege_begin` y del serializador x86: coinciden la imagen de escena, banderas, dos bloques de montón y la carta en sus campos de tipo y tropas. Son 210 casos con reserva de torres y 111 con objetivos; se interceptan asignadores, liberación, nivel de muralla, búsqueda de comerciante, `memcpy` y entrega de carta. La ejecución conserva también las ranuras de objetivos descartados que el original deja escritas. Aún no se despacha desde el reloj ni se simulan las escenas amuralladas; sin prueba en partida.

C10 (PC Sergio): la operación 0x8E ya puede preparar un asedio amurallado sin vista y guardar su escena; exige al anfitrión el reloj externo y la memoria temporal original y conserva la operación si faltan. Se rechaza la escritura fuera de la reserva cuando sus efectos adyacentes no están disponibles. La operación, constructor, inicio, buzón y serializador originales coinciden en 383 ejecuciones x86 (asignadores, nivel de muralla y consulta de comerciante interceptados); una partida preparada se guarda y recarga. El avance posterior de la escena amurallada sigue detenido antes del primer tick no simulado. No se ha probado en partida.

C10 (PC Sergio): conectada al reloj la aproximación amurallada sin vista, salvo el último tick y los hitos que programan ataques a edificios. Se compararon 1.532 ticks y sus imágenes y serializadores x86, y 96 cierres diarios encadenados con lectura, tick y escritura originales; el segundo contraste intercepta asignadores, CRT, despacho y nivel de muralla. La escena y la milicia avanzan en Rust y se persisten en S05. Al cargar se detiene hasta reconstruir los datos vivos no serializados; los ataques a edificios, el resto de la batalla y la vista siguen pendientes. Sin prueba en partida.

C10 (PC Sergio): los hitos de aproximación ya añaden al planificador una tarea 0x26 para el edificio y el tick siguiente. Coinciden 218 hitos con escena, registro de tarea y serializador x86; la inserción en el planificador está interceptada en ese oráculo y se ejecuta con el planificador Rust probado por separado. Una transacción desde el reloj guarda y recarga la tarea. El manejador de 0x26 todavía no está conectado: el reloj se detiene antes de su vencimiento; al cargar una escena amurallada aún faltan sus datos vivos. Sin prueba en partida.

C10 (PC Sergio): el manejador 0x26 ya admite edificios públicos sin vista ni subasta afectada: inicio, avance, anulación por incendio general y liberación de registro y huella. Se cotejaron 1.400 escenarios con el x86; el oráculo intercepta la retirada de subasta. Una prueba ejecuta el primer paso desde el reloj y guarda y recarga la tarea y los registros. Los edificios privados, las subastas afectadas y los planos fuera del margen acreditado se detienen antes del tick; al cargar una escena amurallada aún faltan sus datos vivos. Sin prueba en partida.

C10 (PC Sergio): el inicio del incendio privado ya enlaza las dos listas de edificios de la oficina y se ejecuta desde el planificador. Otros 1.400 escenarios x86 comparan la tarea, ciudad, registros y oficina; las llamadas a búsqueda de oficina y retirada de subasta están interceptadas. Una prueba desde el reloj guarda y recarga los cambios de S02 y del plano. Se exige una oficina única y conocida; si el edificio figura en una subasta, o el plano no entra en la región acreditada, se detiene. La reconstrucción de escena amurallada tras cargar sigue pendiente. Sin prueba en partida.

C10 (PC Sergio): el incendio ya retira su edificio de la subasta 0x36, conserva las demás entradas y libera la tarea si queda vacía. La retirada coincide en 1.800 casos x86 sin dependencias interceptadas; otros 500 comparan en una sola ejecución original incendio y retirada de subasta, interceptando la búsqueda de oficina. Se corrigió una pérdida del enlace de la tarea de incendio al liberar la subasta siguiente. Una prueba desde el reloj guarda y recarga la oficina, el plano y la subasta liberada. Siguen limitados los planos fuera del margen acreditado y la reconstrucción de escenas amuralladas al cargar; sin prueba en partida.

C10 (PC Sergio): al cargar se reconstruyen los campos necesarios para reanudar la aproximación amurallada sin vista, incluidas puertas, cuenta de torres y los indicadores derivados; no se reconstruye la vista ni el estado de disparo. Lectura, tick y escritura coinciden en 1.149 escenarios x86, con asignadores, liberación, `memcpy` y nivel de muralla interceptados. El reloj puede continuar tras guardar y recargar, también hasta el último tick de aproximación; el combate posterior sigue detenido. Sin prueba en partida.

C10 (PC Sergio): la primera ruta de asalto amurallado sin vista ya avanza desde el reloj y tras recargar: sin torres, sin tropas de tipos 1–4 y con atacantes al menos iguales a los defensores, en una puerta sin consumo de brea. El tick coincide en 1.500 casos x86, y la carga seguida del tick en otros 1.149; 96 cierres diarios también coinciden. Se interceptan asignadores, `memcpy`, nivel de muralla y otras dependencias señaladas en los oráculos. El resto del combate, la vista y otros estados siguen pendientes; sin prueba en partida.

C10 (PC Sergio): ampliada esa ruta a las cuatro clases de puerta, con consumo de brea, agotamiento de reservas, fracción acumulada y daño de puerta. Coinciden 1.500 ticks x86, con reservas suficientes e insuficientes y `memcpy` interceptado. Una prueba desde el reloj compara los cambios de ciudad y escena, guarda, recarga y continúa. Persisten las restricciones de tropas, torres y vista; sin prueba en partida.

C10 (PC Sergio): el asalto sin vista ni torres admite ahora atacantes de los cinco tipos y milicia defensora de tipo 0, siempre que no haya baja de milicia en ese tick. Otros 1.500 escenarios x86 comparan escena, ciudad y serializador, con `memcpy` interceptado; el reloj y la recarga se probaron con tropas combinadas. Las bajas, las otras clases defensoras y las torres siguen pendientes. Sin prueba en partida.

C10 (PC Sergio): las bajas de milicia tipo 0 se aplican con sus efectos en empleados, cuatro grupos de población y total. La función original coincide en 2.400 casos x86 sin interceptaciones; 1.500 asaltos completos incluyen 1.225 con baja y comparan ciudad, escena y serializador (`memcpy` interceptado). Una baja ejecutada desde el reloj se guarda y recarga. Las demás clases defensoras, las torres y las resoluciones aún no cubiertas siguen pendientes; sin prueba en partida.

C10 (PC Sergio): el asalto sin vista ni torres admite las cuatro clases defensoras, el reparto de brea y daño entre unidades y sus bajas. Coincide en 1.500 escenarios x86, 103 con alguna baja; `memcpy` está interceptado. Una baja de una clase adicional se ejecuta desde el reloj y conserva población, milicia y escena al guardar y recargar. Siguen pendientes las torres, algunas resoluciones y los casos fuera de los límites documentados; sin prueba en partida.

C10 (PC Sergio): el asalto sin vista ya contempla el daño y la fuerza de torres mientras ninguna se destruya. Coincide en 1.500 escenarios x86 y se ha conectado al reloj y al guardado; otros 96 cierres diarios coinciden con la reconstrucción de torres interceptada. La destrucción, la lista viva tras recargar y otras resoluciones siguen pendientes; sin prueba en partida.

C10 (PC Sergio): al recargar se reconstruye la lista mínima de torres para continuar el asalto sin vista. En 315 escenarios coinciden carga, índices de torre, selección, siguiente tick y ciudad con x86; se interceptan asignadores, liberación, `memcpy` y nivel de muralla. El reloj también continúa tras guardar y recargar. Los bytes indeterminados y tiempos de disparo visual no se acreditan; la destrucción de torres y otras resoluciones siguen pendientes. Sin prueba en partida.

C10 (PC Sergio): la destrucción de una torre ya reordena la lista, selecciona la siguiente y programa la tarea 0x25. Coincide en 1.500 asaltos x86, 1.337 con destrucción; la inserción de tarea y `memcpy` están interceptados. Una prueba ejecuta el paso desde el reloj y guarda y recarga la tarea. El reloj se detiene antes de ejecutar 0x25, cuyo manejador sigue pendiente; sin prueba en partida.

C10 (PC Sergio): la tarea 0x25 de derrumbe urbano ya se ejecuta sin vista: busca la defensa por coordenadas, la pasa a la lista de derrumbes y la convierte en ruina o libera su huella. Coincide en 1.400 casos x86 sin dependencias interceptadas; una prueba desde el reloj guarda y recarga la tarea y los registros. Las guaridas piratas no están incluidas y otras fases del asedio siguen pendientes. Sin prueba en partida.

C10 (PC Sergio): tras caer la última torre se conserva la lista residual que el original todavía consulta. Coincide en 1.500 asaltos x86; otras 284 ejecuciones encadenan en una misma máquina caída, tarea 0x25 y siguiente tick, con inserción de tarea y `memcpy` interceptados. El mismo recorrido pasa por el reloj y guarda y recarga la escena y los registros. Faltan estados de brecha, otras resoluciones y la validación en partida.

C10 (PC Sergio): el combate con brecha abierta, torres opcionales y fuerzas combinadas coincide en 1.500 ticks x86 sin vista. Además, 96 cierres diarios x86 sobre una escena viva confirmaron que debía conservarse su indicador de brecha; se corrigió el cierre para no reconstruirlo como si acabara de cargar. El manejo de derrota y otras transiciones sigue pendiente. Sin prueba en partida.

C10 (PC Sergio): la victoria defensiva sin vista ya actualiza escena, satisfacción, paciencia y carta global en la transacción del reloj. Coincide en 600 escenarios x86, con y sin modo de suministros y `memcpy` interceptado; uno se ejecuta desde el reloj y guarda y recarga ciudad, escena y buzón. El corpus usa tropas de tipo 0 y ninguna torre; la victoria simultánea a una caída de torre, el alta de milicia cuando ambas fuerzas llegan a cero y las fases posteriores permanecen por contrastar. No acredita paridad en partida.

C10 (PC Sergio): ampliada la victoria defensiva a tropas mixtas, torres y bajas, incluido el alta de milicia si ambos bandos pierden su última unidad de primera línea. Otros 1.600 escenarios x86 coinciden en escena, ciudad, carta, paciencia y tarea de torre; 823 incluyen victoria, 529 caída de torre y 209 ambos sucesos. Las últimas 200 fuerzan el alta de milicia. La inserción de tarea y `memcpy` se interceptan; las fases posteriores y la vista siguen pendientes. Sin prueba en partida.

C10 (PC Sergio): conectadas al reloj la espera y limpieza tras victoria defensiva en escena viva, sin vista ni lista de torres. Coinciden 1.200 pasos x86 y una cadena de seis ticks hasta eliminar la escena; el escenario del reloj desactiva el procesado diario de asedios y guarda y recarga tras el cierre. `free` y `memcpy` se interceptan. Falta reconstruir la espera al cargar, la lista residual de torres y la espera con primera línea ya agotada; sin prueba en partida.

C10 (PC Sergio): la espera defensiva se reconstruye al cargar, incluidas las torres y la bandera viva dependiente del reloj. Coincide con 766 cargas y ticks x86; asignadores, liberaciones, `memcpy` y consulta del nivel de muralla están interceptados. Un escenario recarga, avanza por el reloj y vuelve a guardar; desactiva el procesado diario de asedios. Los tiempos visuales de disparo y la espera con primera línea agotada siguen pendientes. Sin prueba en partida.

C10 (PC Sergio): la espera admite también primera línea agotada: reparte el daño entre los restantes atacantes, ajusta la puerta y, cuando corresponde, crea una unidad de milicia. El corpus de espera suma 1.800 pasos x86, 600 con primera línea agotada; las 766 cargas incluyen ahora esos estados. Se contrastan escena, población, puerta y serialización, con `free` y `memcpy` interceptados; la recarga también intercepta asignadores y nivel de muralla. No acredita la vista ni la partida.

C10 (PC Sergio): conectada la marcha de asedios amurallados al reloj cuando ya no quedan torres activas. Coincide en 1.400 ticks x86 fuera de los umbrales de saqueo y salida, con y sin puntero residual de torre; `memcpy` interceptado. Una prueba avanza el reloj y conserva escena al guardar y recargar. La continuación de la marcha tras recargar y el saqueo amurallado siguen pendientes; sin vista ni prueba en partida.

C10 (PC Sergio): la marcha continúa tras cargar, incluso cuando el original reconstruye torres durante ella. Coinciden 766 cargas y ticks x86; asignadores, liberaciones, `memcpy` y consulta de muralla están interceptados. La sesión recarga, avanza desde el reloj y vuelve a guardar con el procesado diario de asedios desactivado. Los umbrales de saqueo y salida amurallada siguen pendientes; sin vista ni prueba en partida.

C10 (PC Sergio): el saqueo al llegar al punto medio de la marcha amurallada ya se ejecuta desde el reloj y persiste la escena, los almacenes, la ciudad, las solicitudes y la carta. Coincide en 450 escenarios x86 con torres activas o residuales, sin vista ni convoy atacante; `memcpy` interceptado. Una prueba de sesión guarda y recarga el resultado. Falta completar la salida de la marcha; sin prueba en partida.

C10 (PC Sergio): la salida de la marcha amurallada inicia la espera y llega a eliminar la escena. Coinciden 240 cadenas x86 completas con tiempos de retirada de uno a seis ticks y torres opcionales; `free` y `memcpy` interceptados. Una cadena se ejecuta desde el reloj y se guarda tras el cierre, con el procesado diario de asedios desactivado. No cubre duración cero, desbordamiento del temporizador ni saqueo simultáneo con salida; sin vista ni prueba en partida.

C10 (PC Sergio): cubierto el saqueo simultáneo con la salida y el cálculo del temporizador de retirada con aritmética de 16 bits. El corpus de saqueo suma 550 escenarios x86, incluidos 100 de salida simultánea; el de salida incluye además 80 ticks con duración cero y valores próximos al desbordamiento. El escenario de sesión guarda y recarga el saqueo simultáneo. Los pasos posteriores cuando la espera queda a cero por desbordamiento aún no están acreditados; sin vista ni prueba en partida.

C10 (PC Sergio): si la espera se desborda a cero y la bandera viva de retirada está activa, el original reinicia la marcha; esa transición se ha conectado al reloj y contrastado en 84 secuencias x86 adicionales. El corpus de salida incluye 640 escenarios, con 240 cadenas completas y 400 de valores extremos. Sin vista ni prueba en partida.

C10 (PC Sergio): tres escenarios encadenan el reloj `0x530E80`, la escena amurallada y su serializador x86 durante treinta días. Se comparan cada día calendario, registro, imagen viva y ciudad; uno conserva la escena activa durante los treinta días y dos la cierran antes. Los demás callbacks del reloj, `free` y `memcpy` están interceptados. Es una comparación del reloj y este estado de asedio, no de todos los sistemas ni de una partida completa; sin prueba en juego.

C10 (PC Sergio): corregida una restricción que detenía el cierre diario durante esperas largas o escenas que dejaban de estar activas. Coincide en 288 cierres x86 con diferentes duraciones, marcha y banderas vivas. Tres cadenas de 30 días ejecutan en la misma máquina x86 el reloj, el cierre diario, la escena y el serializador de asedio y comparan registro, ciudad, calendario, buzón, CRT y operación; los demás callbacks, CRT, `memset`, `free` y `memcpy` se interceptan. El cierre mensual recibe los bytes de pila capturados del original. No es paridad global ni prueba en partida.

C10 (PC Sergio): cuatro variantes encadenan durante 30 días el reloj, una ciudad, sus oficinas, obras y negocios, dos comerciantes, el asedio vivo y los serializadores originales; se comparan los estados compartidos y cada día se guarda y recarga S05. Los barcos intervienen en la economía de los comerciantes, pero el movimiento de flota y otros callbacks siguen interceptados. Un nuevo intento de 30 días con flota completa se detuvo en `open_for_player`, cuya colocación de barcos y obstáculos todavía no está conectada en la entrada C10. No es paridad global ni prueba en partida.

C10 (PC Sergio): las mismas cuatro cadenas de 30 días incluyen ahora los centros comerciales originales y comparan sus 16 registros serializados cada día, también tras guardar y recargar S05. El reloj actualiza los primeros 15 como en el original y usa la población de la ciudad viva. La flota y otros callbacks continúan interceptados; sin contraste global ni prueba en partida.

C10 (PC Sergio): la cadena de 30 días incorpora además el historial urbano original: compara sus banderas y ranuras de conocimiento, comprueba los 29 despachos diarios y verifica la recarga S05 en las cuatro variantes. Los accesos a distancias y oficinas se interceptan con las mismas entradas que usa Rust. La flota y otros callbacks siguen interceptados; sin paridad global ni prueba en partida.

C10 (PC Sergio): las cuatro cadenas encadenadas incorporan también el gestor y serializador de estadísticas originales, partiendo de un PST sintético vacío. Se comparan sus 29 cierres y el PST cada día, con recarga del archivo comprimido junto a S05; las estadísticas comparten ciudades, oficinas, negocios y comerciantes con el resto de la simulación. El movimiento de flota y otros callbacks siguen interceptados; no es paridad global ni prueba en partida.

C10 (PC Sergio): tres escenarios de 30 días comparan dos asedios simultáneos mediante el reloj y los cierres y escenas originales x86. Se comprueban cada día el orden exacto de los despachos, registros, escenas, ciudades, calendario, buzón y estado aleatorio; los cierres mensuales usan bytes de pila capturados. Los demás callbacks del reloj, CRT, `memset`, `free` y `memcpy` se interceptan, por lo que no comprueba el tick urbano real ni la paridad global. Sin prueba en partida.

C10 (PC Sergio): el tick urbano ya conecta el astillero al gestor existente cuando aún no hay navegación viva, en lugar de omitir silenciosamente las reparaciones. Persiste la transacción y sus cartas con comprobación de conflictos; las demás vistas reciben los cambios y se exige guardar y recargar antes de reanudar esa flota. Una cadena de 30 días compara reloj, dos ticks urbanos, reparación de un barco, un asedio, comerciantes, obras, centros, historial, estadísticas y serializadores x86; cada día verifica S05 y recarga. El movimiento de la flota y otros callbacks se interceptan, y el escenario no acredita botaduras de este camino ni paridad global. Las imágenes guardadas se crean desde una base común; no se ha probado continuar el avance desde cada recarga ni en partida.

C10 (PC Sergio): el historial de astillero guarda únicamente las imágenes de barco o entrada que cambia cada paso, además de las cabezas de lista. La cadena de dos ciudades verifica el guardado del astillero desde los estados intermedios de los 30 días y rechaza cambios ajenos sin modificar S05. Persiste el límite de reanudar la navegación tras guardar y recargar; sin flota completa ni prueba en partida.

C10 (PC Sergio): los centros comerciales, el historial urbano y los asedios conservan huellas SHA-256 de las versiones realmente producidas por el reloj de la sesión. Así pueden guardarse sobre una versión intermedia propia sin aceptar cambios ajenos. La cadena x86 de dos ciudades guarda sobre el mismo S05 durante 30 días y comprueba su recarga, además de rechazar alteraciones en cada una de esas tres secciones y en el barco en reparación. No acredita que se continúe desde cada recarga ni incluye el movimiento completo de flota; sin prueba en partida.

C10 (PC Sergio): corregida la lectura de `Town+0x7A4`: el deserializador original del plano deja el indicador a 1 aunque no esté guardado. Un escenario de 30 días recarga S05 y PST en Rust y continúa después de cada día, con el CRT transmitido explícitamente. El oráculo x86 también serializa y reconstruye el asedio cada día, pero conserva los demás objetos en memoria y mantiene interceptados el movimiento de flota y otros callbacks; por tanto aún no acredita una recarga x86 completa ni paridad global. Sin prueba en partida.

C10 (PC Sergio): la carga de sesión aplica ahora el deserializador de edificios existente, incluida la reparación del edificio público 0x2D, y rechaza cadenas cíclicas. También recalcula los umbrales de precio en el orden observado: `game_world_deserialize` deja nulo el gestor de asedios (`0x52FD26`) antes de cargar ciudades (`0x52FD99`), y la economía recalcula antes de leer las instalaciones. En el contraste de 30 días Rust recarga S05/PST a diario; x86 ejecuta el constructor y el deserializador de ambas ciudades y reconstruye el asedio, conservando el resto de objetos en memoria. El movimiento de flota y otros callbacks permanecen interceptados; no acredita carga x86 completa ni partida.

### Formatos de carta de misión R r S s n v w D W F (9/11/2026)
`p3-core/src/mission_format.rs` porta los diez formatos que faltaban del bloque `0x004ED5D2..0x004ED976` de `st_mission_script` (junto a `%+ %- %% %Q %t %N %V %B %E %c` y los de género `%0..%7`, que ya estaban). Oráculo `oracle/x86_c10_script_format.py` (intérprete x86 real con Unicorn, CRT real salvo `lstrlenA`), 900 casos (90 por formato) con paridad exacta en `p3-sim/tests/c10_script_format_x86.rs`. Las tablas estáticas (títulos `0x6A0608/0x6A0718`, tipos de barco, mercancías, meses, calendario `0x672D76`, marcas `0x672C14`) las lee `ScriptTextBank::format_static` del EXE y el test las compara con las del oráculo. `%D` reproduce `0x5311D0` incluido el borde de fechas negativas; `%F` reproduce `0x5333C0` (jugador local, con la rama de visibilidad) y el búfer de 32 bytes del `snprintf` (más largo se rechaza). Límites: el estado vivo (nombres de barcos `+0x160`, cadenas `+0xE4/+0xE8` del comerciante, jugador local) lo aporta el anfitrión por `FormatExtras`; `p3-sim` aún pasa `format_extras: None`, así que una carta con estos formatos en la simulación devuelve error («tablas de formato ausentes») en vez de texto inventado. Índices fuera de tabla (`%w` ≥ 24, `%W` negativo, género > 1 en `%R`) se rechazan; el original lee memoria cercana.

### Comandos de guion 0x3A y 0x65 y formatos en la simulación (9/11/2026)
`0x37` y `0xF8` ya estaban portados por el hilo Rust; este lote añade `0x3A` (reserva de barco: `apply_ship_reserve` sube/baja `ship+0x137` y, al bajar, mueve a la lista libre los barcos de la lista `+0xE6` con reserva 0) y `0x65` (índice de ciudad por nombre del banco de 40 nombres; devuelve -1 si no existe). Oráculo `oracle/x86_c10_script_reserve.py`, 400 casos por comando, test `p3-core/tests/c10_script_reserve_town_x86.rs`. En `p3-sim`, `0x3A` se aplica a `navigation.snapshot` al confirmar la tarea (`MissionPrepared::ship_reserve`) y `0x65` recibe `TownNameBank::map_names()` y los nombres de las ciudades. Las cartas de la simulación ya reciben `FormatExtras`: tablas estáticas del EXE y nombres de barcos (`+0x160`); `%n`/`%v` y `%F` llegaron después (véase «Textos de carta `%n`, `%v` y `%F`», 10/11).

### Comandos de guion 0xE7, 0xE9 y 0xF9: barco pirata de misión (10/11/2026)
Los tres comandos comparten el manejador `0x004ED125` de `st_mission_script`. Si `OPS+0x2C` (0x006DF31C, cliente de red) es 0, arman y publican con `0x0054AA70` la operación 0x82 (`op_spawn_mission_pirate_ship`, 0x0053FB00, ya portada); en todos los casos escriben -1 en la variable `d` del comando, que la operación rellenará más tarde con el barco creado (o -2 si no hay alta). Registro (`mission_script::mission_pirate_record`): `+4` barco de referencia `vars[b]` (0xFFFF en 0xE9), `+6/+7` bytes 1 y 0 de `vars[a]`, `+8` cabeza del planificador (tarea de guion), `+0xA` índice `d`, `+0xB` modo `vars[c]`, `+0xC` ciudad de patrulla (`vars[b]` en 0xE9, 0xFF en los otros) y `+0xD` = 1 solo en 0xE7 (0x0053FB00 no lo lee). Oráculo `oracle/x86_c10_script_mission_pirate.py` (intérprete x86 real, encolado interceptado): 1.500 casos con paridad en `p3-core/tests/c10_script_mission_pirate_x86.rs`, 4/4 mutaciones detectadas. En `p3-sim` el comando deja `LetterEffect::MissionPirate`; `mission_script_preview` lo separa de las cartas y, fuera de cliente de red, la tarea 0x1B lo añade a la cola de operaciones. `p3-sim/tests/c10_script_mission_pirate_live_x86.rs` encadena guion → cola → despacho 0x82 en TEST25_ConvoyMar (9 casos: 8 altas comparadas con 0x53FB00 + 0x5063C0 reales en barcos, capitanes, gestor y variable de resultado, más guardado y recarga; 1 en cliente de red sin publicación). Usan estos comandos 7 guiones reales (Eskorte, Pirat, PiratVernichten, patrouille y tres de la campaña 3). Límites: la espera de `0x54AA70` con la cola llena (0x34 operaciones) no se reproduce; la creación de una banda nueva sigue sin contrastar (la partida ya tiene banda).

### Comandos de guion frecuentes 0x07, 0x1E, 0x28, 0x34, 0x37 y 0xF8 (10/11/2026)
Los seis ya estaban implementados en `p3-core/src/mission_script.rs`; `0x07/0x34/0xF8` y `0x1E/0x28` tenían oráculo (`x86_c10_mission_merchant_fields.py`, `x86_c10_mission_ship_fields.py`), pero sin destino igual al origen en `0x1E/0x28` ni contraste de `0x37`. Nuevo oráculo `oracle/x86_c10_script_frequent.py` (intérprete `0x4ECF64` real, `0x46CC00` sin interceptar) y test `p3-core/tests/c10_script_frequent_x86.rs`: 1.200 casos `0x1E/0x28` (la mitad con `a == b`) y 600 de `0x37`. Hallazgo corregido: `0x1E` escribe -1 en `vars[b]` antes de leer `vars[a]`, así que con `a == b` el original consulta el barco -1 (memoria anterior a la tabla); Rust leía antes y daba la ciudad del barco. Ahora reproduce el orden y devuelve error sin publicar en ese caso. `0x28` lee antes de escribir (el orden anterior era correcto). `0x37` solo llama a la interfaz (`0x469380(0)` y `0x469AF0(0x16, task+8, task+0xA, texto +1, texto +5)`); `event_window_texts` devuelve esos dos textos para un cliente. Cuatro mutaciones (orden de `0x1E`, orden de textos, umbral de estado, máscara de 16 bits de `0x28`) detectadas. Límites: sin ventana en `p3-sim`; índices de ciudad o comerciante fuera de tabla se rechazan donde el original lee memoria cercana.
### Resto de comandos de guion sobre imagen plana (10/11/2026)
`p3-core/src/mission_script_flat.rs` porta los 84 comandos de `st_mission_script` (`0x004ECF64`) que seguían sin implementar, excluidos los de otros hilos (`0x37, 0xF8, 0x34, 0x07, 0x28, 0x1E, 0xE7, 0xE9, 0xF9`): 0x02, 0x03, 0x08, 0x0F, 0x10, 0x1F, 0x25, 0x2A–0x2F, 0x31–0x33, 0x35, 0x36, 0x3B, 0x3C, 0x3E, 0x41, 0x42, 0x45–0x4E, 0x50–0x52, 0x54–0x64, 0x66–0x6A, 0x6C–0x74, 0xE4–0xE6, 0xE8, 0xEA–0xEF, 0xF1–0xF6 y 0xFA. Cada manejador (tabla `0x004F2E34`/`0x004F3054`) se traduce instrucción a instrucción sobre `flatmem::Mem` con las direcciones del original (mundo `0x006DE4A0`, gestor naval `0x006DD7A0`, buzón `0x006DD730`, planificador `0x006DD73C`, tablas `.rdata`). Las llamadas con lógica propia pasan por `ScriptCalls` (dirección, `this`, argumentos; contratos en la cabecera del módulo). `mission_script::execute_in_world_flat` los despacha desde el intérprete y deshace las escrituras si el guion falla (`Journal`).

Oráculo `oracle/x86_c10_mission_flat.py`: intérprete x86 real sobre un mundo aleatorio estructurado (ciudades, comerciantes, oficinas, barcos, convoyes, buzón, bandas), 60 casos por comando, llamadas interceptadas y grabadas. `p3-core/tests/c10_mission_flat_x86.rs` reproduce cada caso y compara variables, tarea, cada byte cambiado del mundo y la secuencia de llamadas (dirección, `this`, argumentos): los 84 coinciden (≈5.000 casos; los casos en que el original lee memoria sin proyectar, no avanza o usa un byte de pila sin inicializar también fallan en Rust). Hallazgos: varios comandos releen variables ya escritas (`0x6F` copia en cadena, `0x6D` compara el índice tras escribir -1, `0x3E/0x47` guardan el valor bruto y releen la mercancía); `0x10624DD3 >> 7` es /2000, no /1000.

En `p3-sim` (`src/c10_script_flat.rs`), si el guion de la tarea 0x1B se detiene en uno de estos comandos, se repite entero con la imagen plana sembrada desde las vistas vivas (mismo esquema que el puente C08) y el `.rdata` del ejecutable (`configure_discovery_locations`). Se publican ciudades, comerciantes, buzón (cartas de `0x004D6530`), planificador (`0x004D8CF0`), operaciones (`0x0054AA70`) y, desde la tarde del 10/11, barcos, convoyes y oficinas (apartado siguiente). Los comandos estructurados del mismo guion leen el estado anterior al guion. Desde el 10/11 `Session::open` (p3-mvp) carga el banco real (`scripts/missions_eng.ini`, 91 guiones de `missions_addon/`), así que el CLI `c10` ejecuta los 59 guiones de los guardados; su informe incluye `task_errors` (errores de tareas que no detienen la partida, que bloquean el planificador en ese tick). `0xEF`, `0xF5` y la ventana de `0xE8` son solo presentación.

### Guiones que escriben barcos, convoyes y oficinas (10/11/2026, tarde)
`c10_script_flat` admite ahora las escrituras de los comandos de guion en los campos que tocan: barco `+0x18` (casco), `+0x3C/+0x3D` (marcas), `+0x3F`, `+0x40` (tripulación), `+0x54..+0xB4` (carga), `+0x114..+0x124` (valor, peso, equipo, armas), `+0x136`, `+0x156` y `+0x157`; convoy `+0x18`; oficina `+4..+0x64` (existencias) y `+0x2E8`. Cualquier otro byte cambiado, el gestor naval y la cabecera del mundo siguen rechazándose. Los bytes de barcos y convoyes se aplican uno a uno sobre la flota de navegación (la tabla común; `publish_navigation_delta` los lleva a la fábrica, la carga, las imágenes de expedición y los metadatos tras el despacho, así que el guardado los recoge por diferencia) y las oficinas cambiadas se publican en `stocks` y como actualización de imagen completa. Se siembran desde el guardado la fecha de inicio (`GameWorld+0xC`, cabecera del mundo `+0x14`), las opciones `[0x006CC3E8]` tal como las deja el cargador `0x00549F40` (borra `+0x3C..+0x4C`; acota `+8` a 14 salvo con `+0xF == 6`; `c10_script_flat::loaded_settings`), la matriz de distancias (`GameWorld+0x6C`) y el factor de venta `0x006DE3D8+0x64`.

Llamadas conectadas: `0x005182B0` (recuento de bodega), `0x004D7770` (estado del mensaje del capitán), `0x0052E1D0` (precio de venta), `0x005252F0` (nivel de muralla, precalculado por ciudad) y cinco puertos nuevos en `p3-core/src/mission_script_calls.rs`: `0x00518640`/`0x005186D0` (subir y bajar pasajeros), `0x00533210` (ciudad más cercana sin las excluidas), `0x00531260` (fecha a ticks) y `0x0051BEB0` (mercancía producida más escasa). Oráculo `oracle/x86_c10_script_calls.py` (funciones reales sobre memoria aleatoria, con las subllamadas `0x517770` y `0x5182B0` reales) y `p3-core/tests/c10_script_calls_x86.rs`: 3.400 casos con el mismo valor devuelto y los mismos bytes, y el original nunca escribe fuera de lo sembrado. Hallazgos: el original suma a `+0x118` el peso de **todos** los pasajeros al subir y resta el de los que **quedan** al bajar. Prueba encadenada `p3-sim/tests/c10_script_flat_writes_live.rs` sobre TEST25: un guion con `0x41`, `0xF2`, `0xEE`, `0x02` y `0x35` cambia casco, contador y existencias de oficina en las copias vivas, y el guardado se recarga con los mismos bytes.

Pasadas de un año (7 guardados, nube): ningún guion de los guardados escribe barcos u oficinas sin que antes se acepte una misión en la taberna, así que los recorridos no cambian; la cobertura de estas escrituras es la prueba encadenada. Límites: bandas piratas (`0x0F`), rutas `0x006CBDC8` (`0x5B`, `0x5D`, `0x5F`, `0x67`) y lo que solo existe en red no se siembran (C10 y Codex); sin validación en partida.

### Las cuatro llamadas de guion que faltaban (10/11/2026, tarde)
Con ellas, todas las llamadas al original que hacen los comandos de guion sobre imagen plana están conectadas en `c10_script_flat`:
- `0x00639C03` (`rand` del CRT, peste `0xEC`): usa y devuelve el mismo flujo que el resto de la simulación (`investigation_rng`); sin estado CRT el guion se rechaza.
- `0x00515750` con barco 0xFFFF (comando `0x10`): media del armamento `+0x120` de todas las flotas con el puerto contrastado de C08, sobre una variable local que empieza en -1.
- `0x00525B90` (incendios `0xE5`/`0xED`): registros de edificios de la ciudad (`Site`) y enlaces de oficinas de la imagen; se siembra también el modo de operaciones `0x006DF2F0`. `0xEC` lee ahora solo la palabra baja de `GameWorld+4` (el original guarda solo esa).
- `0x0050E000` (comando `0x67`, único uso: `Kampagne3_SabotageRostock.p2m`, tipo 2 con armamento mínimo 140): `p3_core::mission_script_calls::create_docked_ship_for_merchant`, con dos funciones nuevas de la fábrica, `0x0051A4E0` (`arm_to_minimum`) y `0x0051A750` (`upgrade_level`). Oráculo `oracle/x86_c10_script_create_ship.py` con la cadena real de la fábrica y `p3-core/tests/c10_script_create_ship_x86.rs`: 1.500 casos iguales (676 con crecimiento de la tabla, 1.244 armados, 256 con el tipo rebajado por `Town+0x2C8`). Hallazgo: con barcos recién reiniciados todas las ranuras están en 7, así que el armamento solo entra después de subir de nivel (`0x0051A750` recalcula las ranuras). En p3-sim la imagen siembra 64 barcos libres tras la tabla por si crece, `finish` admite los cuatro campos del gestor que mueve la fábrica (`+0xE2`, `+0xE8`, `+0xEE`, `+0xF4`) y la tabla completa se publica en todas las vistas navales a la vez (`mission_ship_create.rs`, como la boda). Prueba encadenada `p3-sim/tests/c10_script_create_ship_live.rs` sobre TEST25 (tres altas, la de SabotageRostock incluida): flota de navegación, fábrica, carga, lista de atracados y comerciante, y el guardado se recarga con el mismo barco (el formato no guarda `+0x118..+0x124`, que el lector recalcula).
Límites: el crecimiento escribe a continuación de la tabla (el original reserva un bloque nuevo) y deja a cero lo que el original no inicializa; sin validación en partida.

## M02/M03: red (10/11/2026)

`p3-net` porta el protocolo del original con paridad x86 (M03) y define el lockstep entre clientes nuevos (M02); `p3-mvp::net::NetGame` lo conecta a `Session`. Resultados, pruebas y límites en [`p3-net/README.md`](p3-net/README.md). C09: `p3-mvp/tests/c09_reference.rs` y `research/corpus/scenarios/c09_reference.json` (ver `research/corpus/README.md`).

### Textos de carta `%n`, `%v` y `%F` en la simulación (10/11/2026)
`%n` y `%v` no leen los índices `+0x1D/+0x1C` del comerciante, sino los punteros cacheados `+0xE4` (apellido) y `+0xE8` (nombre de pila), que no viajan en el guardado. Los escriben `merchant_refresh_first_name_ptr` (`0x4F6420`, que además normaliza el bit 7 de `+0x1C` por el sexo `+0x1A`) y `merchant_refresh_family_name_ptr` (`0x4F6450`). En la partida sin pantalla los refrescan la carga del banco de nombres (`0x511E30`, bucle `0x51203A`, para cada `i < GameWorld+0x0A`) y la elección de alcalde (`0x52941A`, solo el nombre de pila); la quiebra (`0x4F5D80`) cambia `+0x1C/+0x1D` **sin** refrescar, así que la carta sigue mostrando el nombre anterior hasta la siguiente carga. `p3_core::mission_format::name_ptr` representa cada puntero por el índice con que se buscó (las ranuras del banco tienen posición fija) y `merchant_name_strings` lo resuelve al formatear. `p3-sim` aplica el refresco al cargar (`refresh_loaded_merchant_names`), conserva los índices en los bytes `+0xE4..+0xEB` de las dos copias del comerciante y, al preparar una carta, rellena `FormatExtras::merchant_strings`. Las ranuras personalizadas del guardado se truncan a 22 bytes como el bucle de copia original (`mission_text::custom_name_slots`).

`%F` (`0x5333C0`) es el nombre del **Ältermann** (`GameWorld+0x8D`, ciudad `+0x8E`), no del jugador local: si no es un comerciante usa el consejero de su ciudad (`Town+0x6F1`, `+0x6D4`, `+0x6D8`). `p3-sim` pasa ahora `alderman_merchant`, `alderman_town` y esos bytes de cada ciudad.

Paridad: `oracle/x86_name_bank_refs.py` ejecuta `0x511E30..0x51206B` real; `p3-sim/tests/c10_merchant_names_x86.rs` compara 205 casos (5 guardados reales y 200 sintéticos con sexo e índices al azar y nombres de hasta 30 bytes): 6.205 comerciantes con el mismo `+0x1C`, la misma ranura en `+0xE4/+0xE8` y el mismo texto. `p3-sim/tests/c10_letter_names_live_x86.rs` despacha 16 cartas F7 `Dear %v %n, the alderman %F writes.` en la simulación y en el intérprete/cola original (`x86_c10_mission_live.py`, ampliado con `name_merchants`, `stale`, `alderman` y `councils`): mismas tareas y mismo depósito de cartas en S05, con Ältermann comerciante o ciudad, y con quiebra sin refresco. Mutaciones detectadas: truncado a 23, sin normalizar `+0x1C`, `%n` desde `+0x1D`.

Límites: las ventanas (ayuntamiento, ficha de ciudad, cartas personales, crónica) también refrescan estos punteros al dibujarse; la simulación sin pantalla no lo hace, igual que el original sin interfaz. Un comerciante creado en la simulación sin pasar por un refresco modelado deja los punteros desconocidos y `%n/%v` se rechazan (no se inventa texto). Los nombres de barco (`%s`) ya llegaban desde la navegación. Sin validación en partida.

## C08: batalla dentro de la ciudad durante un asedio (10/11, nube de Claude)

`p3-core/src/economy/siege_battle.rs` porta con paridad x86 lo que ocurre en la vista de la ciudad sitiada: campo de
rutas por fotograma (`0x62B9C0`, `0x632E90`, `0x633060`), colocación (`0x62F290`), defensores, marcadores de puerta,
milicia, asaltantes y reencolado (`0x6307B0`, `0x630A80`, `0x6311E0`, `0x62D170`, `0x631120`, `0x630DC0`), apertura
completa (`0x62C730`), fotograma (`0x631A50`), retirada de figuras (`0x634D80`, `0x62DFB0`) y cierre (`0x62F5A0`).
Contrastes: `p3-render/tests/x86_siege_battle.rs` (9 pruebas, 2.120 casos, `P3_EXE` y `P3_ORACLE_PY`). `p3-sim`
conecta apertura, fotograma y cierre a la vista del cliente (`Sim::open_siege_view`, `Sim::step_siege_view`,
`Sim::close_siege_view`), con la parte viva del registro de asedio guardada en la vista; `p3-mvp` avanza fotogramas
(`Session::step_siege_view`) y dibuja las figuras. Las figuras aún no se mueven ni combaten (sus métodos por fotograma
no están portados). Detalle y límites en `research/systems/eventos/CENSO_C08.md` (cuarta tanda del asedio en curso).
