# Rendimiento de la reimplementación: medidas, mejoras aplicadas y pendientes (12–13/11/2026, nube de Claude)

> 13/11/2026 (tarde), nube de Claude: **carga de partida nueva en `p3-juego menu`, de unos 11 s a unos 3 s** (3,3 veces menos
> instrucciones) y 630 MB menos de memoria, con los mismos fotogramas y el mismo estado del juego.
> Ver [«Carga de partida nueva»](#carga-de-partida-nueva-de-single-game-y-start-a-la-ciudad-13112026-tarde-nube-de-claude) al final.

> Actualización de resoluciones altas en Windows: cliente2560×1440 de14,15a34,26FPS;
> [bancos, correcciones y límites](../coordination/revision/UI_FPS_HIGHRES_2026-11-13.md).

> 13/11/2026, PC de Sergio: el cliente `menu` tiene una ruta UI distinta de `p3-sim`.
> Investigación y corrección Windows: ciudad a pantalla completa 17,3 → 46,2 FPS,
> con [medidas, pruebas y límites](../coordination/revision/UI_FPS_PROFILE_2026-11-13.md).

Pregunta de Sergio: ¿se puede mejorar el código antiguo del juego y cuánto se gana? Respuesta corta:

- **El código original portado casi no pesa.** En un tick de simulación, la lógica de 2003 elevada a Rust (`p3-core`) es menos del
  5 % de las instrucciones. El 84 % del tick se iba en el tick naval, y dentro de él casi todo eran copias de respaldo y espejos de
  estado de nuestro andamiaje (`p3-sim`, `p3-state`), no cálculo del juego.
- **Aplicado sin cambiar el comportamiento: la simulación va 2,6 veces más rápida** (un año simulado pasa de unos 3 min a algo más
  de 1 min por guardado). Los guardados diarios son idénticos byte a byte a los de antes y al x86 original emulado (apartado «Paridad»).
- Queda margen grande (estimado, otras 3–5 veces en simulación y unas 5 veces en la carta náutica), pero ya exige reorganizar
  cómo se comparte el estado entre capas; está ordenado por ganancia y riesgo al final.

## Cómo se midió

- Máquina: contenedor de la nube de Claude, 4 núcleos, Rust 1.97.0, `--release`. Un solo hilo. Los tiempos son de CPU de usuario.
- Simulación: `p3-mvp/examples/c10_snapshots GUARDADO p2arch0_eng.cpr Patrician3.exe SALIDA DÍAS --human-battles-auto
  --battle-pacing clock`. Cada día guarda la partida y calcula su SHA-256, así que el mismo comando sirve de cronómetro y de prueba.
- Perfil: `valgrind --tool=callgrind --toggle-collect='*advance_ticks_with*'` (cuenta instrucciones solo dentro del avance de
  ticks, sin la carga) sobre 3 días (768 ticks) de `fps-cache/AUTO`. No hay `perf` en el contenedor.
- Cliente: `xvfb-run p3-juego vista GUARDADO CPR --ventana --fps 100000 --fotogramas N`, diferencia entre 30 y 330 fotogramas.

## Simulación: antes y después

| Medida | Antes (main 079b182) | Después | Ganancia |
|---|---|---|---|
| Instrucciones por tick (perfil, 768 ticks de fps-cache/AUTO) | 12,2 M | 4,7 M | 2,6× |
| 30 días, fps-cache/AUTO (CPU) | 14,76 s | 5,6 s | 2,6× |
| 120 días, ConvoyMar (CPU) | 63,3 s | 24,3 s | 2,6× |
| Por día simulado | 0,47–0,53 s | 0,17–0,20 s | |
| Un año simulado (estimado de lo anterior) | unos 3 min | algo más de 1 min | |
| Carga de la partida (CPU) | 0,55 s | 0,45 s | 1,2× |
| Memoria máxima | 336 MB | 336 MB | igual (248 MB son el CPR en memoria) |

Reparto del tick antes de tocar nada (9.390 M de instrucciones en 768 ticks):

| Parte | Instrucciones | % |
|---|---|---|
| Tick naval (`ships_tick`) | 7.870 M | 84 % |
| de ello, copiar y liberar el planificador de rutas (3 copias por tick) | 2.900 M | 31 % |
| de ello, copiar todos los comerciantes una vez por barco en el mar | 1.280 M | 14 % |
| de ello, aplanar ciudades y comerciantes byte a byte | 1.130 M | 12 % |
| Tareas programadas (`scheduled_tasks_tick`) | 800 M | 9 % |
| Escenas de asedio, fase diaria y resto | 720 M | 8 % |

## Mejoras aplicadas (todas sin cambio de comportamiento)

1. **Planificador de rutas compartido** (`p3-core/src/route/finder.rs`). `RoutePlanner` se copiaba entero tres veces por tick
   (rejilla de 640×472, matriz de costes y tabla de rutas ciudad→ciudad) para poder deshacer el tick si fallaba. Ahora `nav`, `rows`,
   `routes` y `sea_points` van en `Arc` con copia al escribir (`Arc::make_mut`): copiar el planificador es barato y solo se duplica
   la tabla que de verdad se modifica. El primer punto de una ruta compartida solo se reescribe si cambia.
2. **Comerciantes por referencia en el avance de cada barco** (`p3-sim/src/navigation.rs`, `p3-core/src/ships/advance.rs`).
   `Host::advance` copiaba todos los registros de comerciante (0x650 bytes cada uno) por cada barco en el mar y tick.
   `AdvanceWorld::merchants` es ahora `&[&[u8]]`: vistas sobre los mismos bytes. El paso solo los lee.
3. **Copias por bloques** (`navigation.rs`, `p3-state/src/office.rs`). Las imágenes de ciudades, comerciantes y oficinas se
   construían con `flat_map(...).collect()` byte a byte; ahora con `extend_from_slice`. Mismo resultado.
4. **Una capa de respaldo menos** (`navigation.rs`, `p3-sim/src/lib.rs`). `ships_tick_with_images` ya trabaja sobre copias
   propias y las descarta enteras si hay error, pero llamaba a `step_with_ai_bands_and_expeditions`, que volvía a copiarlo todo.
   Nueva variante `step_with_ai_bands_and_expeditions_staged` sin esa segunda copia; la función original sigue igual para los demás usos.
5. **Publicación de cambios** (`publish_navigation_delta`). Antes de comparar byte a byte un barco o convoy con su versión
   anterior se comprueba si el registro entero es igual.
6. **Memoria dispersa de los portes planos** (`p3-core/src/flatmem.rs`). `SparseMem::seed` construye el árbol de un bloque grande
   de una vez y lo funde, y `read` recorre un tramo contiguo en una sola pasada. Mismos contenidos y mismo error si falta un byte.
7. **Búsqueda en el CPR** (`p3-assets/src/cpr.rs`). `Archive::find` plegaba a minúsculas una copia de cada nombre del índice en
   cada búsqueda; ahora compara plegando byte a byte, sin reservar memoria.

Pruebas adaptadas por el cambio de tipos (solo cómo construyen sus datos): `p3-core/tests/x86_ship_advance.rs`,
`x86_mvp_move_start.rs`, `x86_route_planner.rs` y `p3-sim/tests/town_plans.rs`.

## Paridad

- **x86 original emulado, 730 días, modo `clock`** (`oracle/whole_game_corpus.py --days 730 --human-battles-auto --battle-pacing
  clock --siege-tails-from-original --residue-from-original`, binario optimizado):

| Guardado | Muestras | Iguales al x86 |
|---|---|---|
| AUTO | 732 | 732 |
| ConvoyAvista | 732 | 732 |
| ConvoyHecho | 732 | 732 |
| ConvoyMar | 732 | 732 |
| ConvoyPrueba | 732 | 732 |
| TestAstiller | 732 | 732 |

  Los seis guardados, sin ninguna diferencia. AUTO y TestAstiller se contrastaron con el binario anterior al cambio de
  `Archive::find`; los otros cuatro, con el binario final.

- **Antes frente a después, mismos comandos:** 32/32 muestras iguales en 30 días de fps-cache/AUTO y 122/122 en 120 días de
  ConvoyMar (SHA-256 del guardado descomprimido de cada día).
- **Batería Rust** (`python scripts/reimpl_check.py`, con EXE y CPR): 2.616 pruebas correctas, 25 fallidas, 30 excluidas en 1.088
  binarios. Las 25 fallan igual en main 079b182 sin mis cambios (comprobado ejecutando esos grupos en ese commit): 18 de
  `p3-audio` (comparan con capturas del SDK Miles que solo están en el PC de Sergio) y 7 de `p3-render/tests/x86_j07.rs`
  (`j08_lift_pan4`, `rest3`, `sim2`, `sim3`, `sim5`, `sim7`, `sim11`). No las he tocado: no son de este trabajo, pero las 7 de j08
  conviene que las mire el hilo del elevador.

## Render e interfaz (medido, sin cambios aplicados)

| Vista | Coste | Observación |
|---|---|---|
| Ciudad en `p3-juego` (800×600) | 2,4 ms por fotograma (unos 415 fps) | Sobra margen. Al abrir la ciudad hay un tirón de 0,7–0,8 s al componer el mapa de 4386×2551. |
| Carta náutica en `p3-juego` | 15 ms por fotograma (62–65 fps), 26 de 330 fotogramas pasan de 16,7 ms | Redibuja el mapa entero cada fotograma. |
| `render_city` a PNG | 1,1 s | La mitad es abrir la partida. |

Perfil de la carta náutica (158 M de instrucciones por fotograma): 57 % en `draw_nav_map` → `Canvas::blit` → `sgl::draw_bitmap_rect`
→ `blend_pixel` píxel a píxel, 14 % en convertir cada mapa de bits de bytes RGBA a `Vec<u32>` en cada dibujo, 11 % en copiar el
fondo y 7 % en las etiquetas de ciudad. No lo he tocado porque `p3-mvp/src/navmap.rs` y la interfaz tienen otros hilos abiertos.

## Lo que queda, por ganancia y riesgo

Perfil actual del tick (3.600 M en 768 ticks; `malloc`/`free` y `memcpy` siguen siendo casi la mitad):

| # | Mejora | Ganancia estimada | Riesgo | Notas |
|---|---|---|---|---|
| 1 | Carta náutica: guardar la capa fija (fondo, costas, etiquetas) y redibujar solo barcos y lo que cambie; convertir los mapas de bits a `u32` una vez al cargar | carta de 15 ms a unos 3 ms por fotograma | Bajo (la salida se compara píxel a píxel con la actual) | Toca `p3-mvp/src/navmap.rs`: coordinar con el hilo de J06. |
| 2 | `publish_navigation_delta` y `dispatch_scheduled_tasks_fleet`: dejar de clonar la instantánea naval y las imágenes de expedición en cada tick; marcar qué barcos cambian | 15–20 % del tick | Medio | 540 M + 660 M. Hay tres espejos del mismo barco (navegación, fábrica, expedición). |
| 3 | `ships_tick_with_offices`: no convertir las 64 oficinas a imagen y de vuelta en cada tick (`to_save_v85` → `decode_office` → `encode_office`) | 10–15 % del tick | Medio | 670 M. Mantener la imagen viva y sincronizar solo la oficina tocada. |
| 4 | `SparseMem` como páginas de 4 KiB con mapa de bytes sembrados en vez de `BTreeMap<u32,u8>` | 5–8 % del tick | Medio | El campo `bytes` es público y lo leen 5 sitios; hay que darles una interfaz equivalente. |
| 5 | Escenas de asedio: `siege_scene` se llama 24 veces por tick y calcula un SHA-256 del estado cada vez | 5–10 % del tick | Bajo-medio | 370 M. Sirve un contador de versión o comparar bytes. En CPU real el SHA usa instrucciones nativas y pesa menos que en el perfil. |
| 6 | `mission_script_preview`: reconstruye contexto y nombres en cada tick aunque el guion no avance | 5–8 % del tick | Medio | 290 M. |
| 7 | Quitar la transacción por copia del tick naval: trabajar sobre el estado vivo y guardar un registro de deshacer solo si se necesita | 20–30 % del tick | Alto | Hoy un error deja el estado como antes del tick; hay pruebas que lo exigen (`pursuit_replan_rolls_back_...`). |
| 8 | Simular varios guardados en paralelo (un proceso por guardado) | lineal con los núcleos | Nulo | Ya se hace a mano en los contrastes; solo es comodidad. |

Con 2 a 6 hechas, la estimación es llegar a 1,5–2 M de instrucciones por tick (otras 2–3 veces). El suelo es la lógica original:
unos 0,2–0,5 M por tick.

## Cambios que alterarían el original (no aplicados, solo anotados)

- **Rutas compartidas ciudad→ciudad.** El original reescribe el primer punto de la ruta compartida en cada consulta
  (`find_route`, `0x00445010`). Es un efecto lateral visible para otros barcos que usan esa ruta; no se puede quitar. Se conserva.
- **Las 24 escenas de asedio de cada tick.** Es un bucle del original sobre una tabla fija. Saltarse ranuras cambiaría el
  orden de consumo del generador aleatorio si alguna rama lo usa. No tocar sin oráculo.
- **Recalcular rutas solo cuando cambia el destino.** El original replanifica cada 8 ticks y cada cálculo avanza el contador
  global `0x006CBDC4` (`line_counter`). Cachear rutas cambiaría ese contador; no está comprobado que sea inocuo.
- **Coma flotante.** El original usa x87 con su control de precisión; cualquier reordenación o vectorización de las fórmulas
  de precios o de movimiento puede cambiar el último bit. No tocar.

## Reproducir

```bash
cd reimpl && cargo build --release --locked -p p3-mvp --example c10_snapshots
time target/release/examples/c10_snapshots AUTO.pat p2arch0_eng.cpr Patrician3.exe /tmp/salida 30 \
  --human-battles-auto --battle-pacing clock            # cada línea de stdout lleva el sha256 del día
CARGO_PROFILE_RELEASE_DEBUG=1 cargo build --release --locked -p p3-mvp --example c10_snapshots --target-dir /tmp/perfil
valgrind --tool=callgrind --toggle-collect='*advance_ticks_with*' /tmp/perfil/release/examples/c10_snapshots ... 3 ...
callgrind_annotate --inclusive=yes callgrind.out.*
```

## Rama `optimizaciones` (13/11/2026, nube de Claude): segunda tanda, con ideas que se apartan del diseño original permitidas

Cada idea se hizo en su subrama (`opt/*`), se midió contra la base de `b0b13fb` con callgrind (3 días de AUTO, 768 ticks) y se
comparó el guardado de cada día de 30 días contra la base (AUTO y TestAstiller, `--human-battles-auto --battle-pacing clock`).
Notas completas, trampas y comandos de cada una en `reimpl/opt-notas/*.md`. No se aplicó ninguna desviación del comportamiento del
original: todas conservan los guardados byte a byte.

| Idea | Qué cambia | Ganancia medida | Guardados 30 días |
|---|---|---|---|
| Espejos navales, oficinas y tick vivo (`naval`) | No clonar instantánea ni imágenes en cada tick; oficinas sin pasar por el formato de guardado; tick naval sobre el estado vivo con registro de deshacer; imágenes por intercambio | 3.364 M → 1.729 M instrucciones (−48,6 %, 4,38 → 2,25 M por tick) | 32/32 AUTO, 32/32 TestAstiller |
| Asedios y guion de misión (`asedio`) | Versión de asedio por comparación de bytes sin SHA; saltar ranuras sin escena; `FormatExtras` diferidas y caché del guion | 3.363 M → 2.737 M (−18,6 %); `mission_script_preview` 287 M → 28 M | 32/32 y 122/122 a 120 días, AUTO y TestAstiller |
| `SparseMem` por páginas (`sparsemem`) | Páginas de 4 KiB con mapa de bits en vez de un nodo por byte | −6,0 % (coste diario de `pirate_group_tick_stage`) | 32/32, 32/32 |
| Pasos de batalla (`rutas`) | El bucle sin pantalla ya no calcula `state()` (resumen con SHA) por paso de batalla; variantes `_lean` | TestAstiller 9.579 M → 4.382 M (−54,3 %); AUTO −1,2 % | 32/32, 32/32 |
| Carta náutica (`navmap`) | Capa fija en caché, texturas a `u32` al cargar, copia rápida sin mezcla | 13,9 → 1,55 ms por fotograma quieta; 11,5 → 2,73 ms desplazando | imagen idéntica píxel a píxel en 141 fotogramas |
| Apertura de ciudad y carga (`ciudad`) | Composición por filas recortadas y en paralelo, índice de nombres una vez por escena, CPR proyectado en memoria (`cpr::Bytes`) | Abrir ciudad 0,72 → 0,11 s; abrir partida 0,71 → 0,03 s; arranque 2,42 → 0,28 s; memoria 386 → 212 MB | 192/192 imágenes iguales; AUTO 32/32 |
| Interfaz real (`uirt`) | `is_line_clear` y `snap_to_sea` nativas sobre la memoria emulada | ~5 % de la carga, dentro del ruido; lo grande ya lo hizo Codex en `main` | 100/100 hashes del banco |
| Rutas (`rutas`) | Tabla ciudad→ciudad compartida (`Arc`) | −0,28 %; el resto de ideas de rutas se descartó: la lógica portada pesa ~1 % del tick | 32/32 |

Combinadas, las de simulación llevan AUTO de 4,38 M a algo menos de 2 M instrucciones por tick (estimación por suma; el conjunto compila
pero no se ha vuelto a medir junto).

**Sin verificar:** la combinación de todas las ideas no se ha comparado guardado a guardado (cada una sí, por separado); las pruebas
que exigen el oráculo x86 (`P3_ORACLE_PY`) se saltan solas en la nube; un error a mitad del tick naval con rutas ya cambiadas
(`VoyageUndo`) sigue solo razonado; el mapeo en memoria del CPR puede dar error si alguien modifica el `.cpr` con el juego abierto; la
primera compilación necesita `--offline` por `memmap2`. **Pendiente y medido:** `NavBefore::capture` (76 M) y `patch_changed` (78 M) del tick
naval; fase diaria (`town_daily` ~190 M); `scheduled_tasks_tick` ~368 M; `Host::route_points` (0,5 %); `load_figures` por escena y resolución
alta en la ciudad; perfil de compilación (LTO, `codegen-units=1`) y asignador de memoria, no probados.

## Carga de partida nueva: de «Single Game» y «Start» a la ciudad (13/11/2026, tarde, nube de Claude)

Pregunta de Sergio: bajar aún más el tiempo desde que pulsa «Start» hasta que ve la ciudad en el cliente real (`p3-juego menu`:
código original elevado sobre el enlazador `p3-mvp/src/ui_rt`). Base: `main` 1dfe60c, que ya trae lo de Codex (páginas por
índice directo, mezcla SSE2) y la tanda `optimizaciones`.

### Resultado

| Medida (banco `ui_benchmark city`, 800×600, nube, 4 núcleos compartidos) | Antes (1dfe60c) | Después | Ganancia |
|---|---|---|---|
| Fotograma de carga (el 14: de «Start» a la ciudad), reloj, mejor de 3 | 10,9–12,8 s | 3,0–3,4 s | unas 3,5× |
| Instrucciones de toda la ejecución de 16 fotogramas (callgrind) | 142,3 mil millones | 43,5 mil millones | 3,3× |
| Memoria máxima del proceso (incluye 248 MB del CPR, que el banco lee entero) | 1.132 MB | 500 MB | −632 MB |
| Arranque hasta el menú (fotograma 1) | 0,51 s | 0,12–0,15 s | 3–4× |
| Primeros fotogramas de ciudad (15 y 16) | 4 y 40 ms | 4 y 36–43 ms | igual |

El reloj de la nube es ruidoso (máquina compartida): la cifra firme es la de instrucciones. En el PC de Sergio falta medirlo con el
ejecutable de Windows (ver «Sin verificar»).

### Dónde se iba el tiempo (perfil de instrucciones de la base, 142,3 mil millones)

| Parte | % | Qué era |
|---|---|---|
| Búsqueda en la caché de texturas de la ciudad (`texture_cache_get_or_load` `0x54F290`) | 39 % | Unas 6.000 búsquedas lineales sobre más de mil nombres: 8,1 millones de `_stricmp`, y cada uno copiaba las dos cadenas byte a byte a memoria nueva |
| Simulación inicial del mundo (`advance_time` `0x530E80`, 34.049 ticks de barco) | 26 % | Código original elevado: es trabajo del juego |
| Lectura de `.ini` (`GetPrivateProfileString/Int`, 64.624 consultas) | 14 % | Cada consulta copiaba el archivo entero del CPR y lo recorría línea a línea |
| Descodificar imágenes (`sgl_LoadMemoryTexture`, 4.443 cargas) | 8 % | Se descodificaban al cargar las 3.477 texturas de archivo (806 MB en RGBA: todos los edificios y figuras), y en los primeros fotogramas se dibuja una pequeña parte |
| Coste fijo del enlazador por llamada (21,4 millones de llamadas) | ~10 % | Tablas con SipHash, cadena de sustitutos, contabilidad |

### Cambios (todos en `p3-mvp/src/ui_rt/mod.rs` salvo donde se indica)

| # | Cambio | Instrucciones tras aplicarlo |
|---|---|---|
| 1 | `_stricmp`/`_mbsicmp`/`strcmp` sin copiar las cadenas (`cmp_cstr`); `.ini` del CPR analizados una vez (`ini_cache`: sección y clave en minúsculas → primer valor); `w16`/`w32` con una sola búsqueda de página; tablas por dirección con dispersión barata (`U32Map`) en vez de SipHash | 142,3 → 69,7 |
| 2 | Texturas de archivo descodificadas al primer uso (`Assets::pending`, `materialize`): el manejador se entrega al cargar y la imagen sale del CPR cuando alguien la dibuja, la mide o la usa de fuente o destino | 69,7 → 61,5 |
| 3 | Comparación por palabras de 8 bytes dentro de `PageMem` (`Mem::cmp_cstr`, método nuevo con valor por defecto en los rasgos `Mem` de `p3-core` y `p3-render`); camino corto de `_stricmp` en `dispatch_inner`; accesos de `PageMem` y `x86rt::rm`/`wm`/`push`/`pop` en línea | 61,5 → 49,5 |
| 4 | Búsqueda de `texture_cache_get_or_load` hecha de una vez en el enlazador; búsquedas en el índice del CPR recordadas (`Assets::file_bytes`; `Archive::find` recorría el índice entero en cada carga); directorio de páginas sin comprobación de rango | 49,5 → 43,5 |

Reparto actual (43,5 mil millones): simulación inicial del mundo 55 %, recursos de la pantalla de juego 23 % (escena de ciudad
14 %), creación de la partida 9 %, menú 2 %. Lo que queda es casi todo código original elevado que se ejecuta instrucción a
instrucción sobre la memoria emulada.

### Por qué no cambia el comportamiento

- **Mismo estado y mismas imágenes.** `ui_benchmark` da ahora, además del hash FNV-1a de cada fotograma y los contadores de
  llamadas, el hash de toda la memoria del original (`PageMem::content_hash`) y el mismo sin la pila del enlazador
  (`content_hash_outside`). Antes frente a después:

| Escenario | Fotogramas iguales | Memoria del juego (sin pila) | Contadores y ausentes |
|---|---|---|---|
| `city 60` (partida nueva, Luebeck) | 60/60 | igual | iguales |
| `pan 110` (ciudad desplazándose) | 110/110 | igual | iguales |
| `city 50 1920x1080` (resolución moderna) | 50/50 | igual | iguales |
| `menu 30` | 29/29 | igual (también con pila) | iguales |

  La memoria del original contiene el mundo entero (ciudades, barcos, comerciantes), así que un guardado hecho desde ese estado es
  el mismo. No se ha vuelto a comparar un `.pat` escrito a disco, porque sale de esos mismos bytes.
- Los cambios 1 a 3 dejan idéntica **toda** la memoria, pila incluida. El cambio 4 (búsqueda de una vez) solo cambia lo que el
  enlazador dejaba escrito por debajo de la pila viva: antes cada `_stricmp` dejaba un marco muerto (retorno y dos argumentos) 68
  bytes más abajo que el anterior. Eso no es estado del juego (el original real deja ahí otros restos). Con `P3_PROF` a un número
  enorme (cualquier diagnóstico activo) los atajos se desactivan y la memoria vuelve a ser idéntica byte a byte a la de la base:
  es la referencia con la que se comparó.
- **Texturas aplazadas.** Aplazar solo es válido si ya se sabe que la descodificación no fallará (la carga devuelve 0 si falla).
  `lazy_decodable` lo acepta para TGA (mismas comprobaciones de cabecera que el descodificador, que después no puede fallar) y
  para AIM de tipos 0x21 y 0x22; BMP y JPG se descodifican al cargar como antes. `examples/ui_texture_probe.rs` descodifica
  **todos** los archivos del CPR: 318 TGA y 3.067 AIM aplazables, los 3.385 se descodifican; 0 fallos (los AIM 0x16 y 0x1A, que no
  se descodifican, no se aplazan). Con un CPR dañado, una textura aplazada que no se pueda descodificar deja un aviso y no se
  dibuja (antes la carga habría devuelto 0). `P3_EAGER_TEXTURES=1` vuelve a descodificar al cargar.
- La clave de color que `sgl_LoadMemoryTexture` aplicaba al cargar se guarda y se aplica al descodificar; `sgl_ReLoadMemoryTexture`,
  las fuentes, el destino de dibujo y `sgl_GetTextureInfo` descodifican antes de usar la imagen.
- **Pruebas:** ver «Verificación» abajo.

### Verificación

- Sobre el `main` más reciente (e659f3c, con el zoom limitado al mapa y la alineación de texto de Codex, que cambian 12 de los
  60 fotogramas respecto a 1dfe60c): los tres escenarios de ciudad se repitieron comparando el mismo binario con atajos y sin
  ellos (`P3_PROF` enorme y `P3_EAGER_TEXTURES=1`): 60/60, 110/110 y 50/50 fotogramas, memoria del juego y contadores iguales.
- `cargo test -p p3-mvp --release` de las 31 pruebas `ui_*` y `real_ui_runtime_snapshots`, más `cargo test -p p3-client --release`,
  con `P3_EXE` y `P3_CPR`: 69 correctas, 0 fallidas en 32 binarios. Nueve binarios de `ui_*` terminan en 0,00 s en la nube porque
  necesitan partidas o capturas que solo están en el PC de Sergio: no cuentan como comprobación.
- `cargo build --release --workspace`: compila entero (el método nuevo de los rasgos `Mem` tiene valor por defecto).
- **Sin verificar:** no se ha compilado ni medido el ejecutable de Windows ni se ha probado en partida real con ventana; las
  pruebas x86 de `p3-core` y `p3-render` (oráculo Unicorn) no se han repetido, y los cambios en esos dos crates son un método de
  rasgo sin uso por defecto y atributos `#[inline]`; los 10 s que tarda en abrirse la ventana en el PC no se han investigado
  aquí (en la nube el arranque hasta el menú baja de 0,51 a 0,13 s, así que ese tiempo no parece ser del enlazador).

### Reproducir

```bash
cd reimpl && cargo build --release --locked -p p3-mvp --example ui_benchmark --example ui_texture_probe
export P3_EXE=Patrician3.exe P3_CPR=p2arch0_eng.cpr
target/release/examples/ui_benchmark city 60 > nuevo.json        # "ms" del fotograma 14 = carga; "pixels", "memory", "memory_no_stack"
P3_PROF=1000000000000 target/release/examples/ui_benchmark city 60 > referencia.json   # sin atajos: memoria idéntica a la base
target/release/examples/ui_texture_probe                           # condición de las texturas aplazadas sobre el CPR entero
valgrind --tool=callgrind target/release/examples/ui_benchmark city 16   # instrucciones (compilar con CARGO_PROFILE_RELEASE_DEBUG=1)
```

### Lo que queda, por ganancia

| # | Idea | Ganancia estimada | Riesgo |
|---|---|---|---|
| 1 | Simulación inicial (55 %): sustituir en el enlazador los elevados más llamados (`advance_along_route` `0x516CC0`, `XOrder::resort` `0x50D0C0`, `trade_amount` `0x52F030`, `tick` `0x506720`) por sus portes de `p3-core` trabajando sobre la memoria emulada en vez de sobre copias | hasta la mitad de lo que queda | Medio-alto: hay que demostrar paridad sobre la memoria viva |
| 2 | Coste fijo por llamada (unas 450 instrucciones × 13 millones): tabla única de ruta por dirección en vez de la cadena de sustitutos | 5–8 % | Medio: `display_native` intercepta direcciones que salen de tablas virtuales, no vale una lista fija |
| 3 | Perfil de compilación (`lto`, `codegen-units=1`) para el ejecutable de Windows | sin medir | Bajo |
| 4 | Descodificar en segundo plano las texturas aplazadas para que el primer desplazamiento por la ciudad no descodifique nada | suaviza, no acelera la carga | Bajo |

## «La carga de la partida tarda mucho más» (22/11/2026, nube de Claude y PC de Sergio)

Queja de Sergio con la build de sucesos (`feature/sucesos` 12682f3): su registro marcaba 17 s en un fotograma al empezar partida.

### Qué se midió

| Medida | Versión anterior (`p3-juego.antes-sucesos.exe`) | Build de sucesos |
|---|---|---|
| Nube, cliente real con el `P2.CFG` de Sergio, partida nueva (un fotograma) | 3,8–4,0 s | 4,1–4,4 s |
| Nube, cargar `AUTO.pat` | 1,4–1,5 s | 1,2–1,3 s |
| PC de Sergio, partida nueva con guion, en ventana y a pantalla completa (8 ejecuciones) | 2,2–2,3 s | 2,3–2,4 s |
| Asignaciones de memoria en la partida nueva (por tamaños) | igual | igual |

La carga en sí no ha empeorado. Lo que se alargaba en el PC eran los fotogramas siguientes: de 5 a 20 s parados en
`sgl_SwapBuffer`, varios seguidos, y de forma irregular (unas ejecuciones sí y otras no; también alguna con el exe anterior).

### Causa

`D:` (disco duro mecánico, donde están `P3-Prueba`, el juego y los árboles de compilación) estaba saturado por otro proceso:
200–300 % de tiempo de disco con la CPU al 3 %, y listar una carpeta tardaba minutos. Con `P3_SUCESOS=0 P3_ABASTO=0` los
parones desaparecían: no es que ese código sea lento, es que su primera ejecución leía del disco las páginas del ejecutable
que aún no estaban en memoria (y la música nueva, y el catálogo `sucesos\*.txt`). La precarga del 19/11 leía el `.exe` como
archivo, pero Windows pagina la imagen cargada aparte y eso no la calentaba.

### Cambios

- `p3_client::precarga`: el hilo toca primero todas las páginas de la imagen del ejecutable (`touch_image`, `VirtualQuery` +
  una lectura por página) y, al final, lee la música suelta (`Musik\**\*.mp3|wav`) de las carpetas donde la busca el sonido. En
  el remaster también `sucesos\*.txt`. Sale en el registro: «precarga: imagen del ejecutable…» y «precarga: N archivos sueltos…».
- `ui_rt/stall.rs`: el muestreo cubre el fotograma entero (antes solo los 2 primeros segundos: se queda una muestra de cada dos
  y dobla el paso al llenarse), los fotogramas de más de 2 s añaden «tramos de N ms (común/hoja)» y el código propio sale con
  nombre (`superposiciones`, `cambio-de-modo`, `mostrar`, `arranque-sonido`…; en el remaster también `sucesos`, `abasto`,
  `viajeros`, `lonja`…).

### Sin verificar

- `touch_image` solo se ha compilado para Windows (`x86_64-pc-windows-gnu`); falta verlo correr en el PC (wine no arranca en
  la nube). Si el disco sigue saturado, lo que quede se verá en las líneas «tirón» con nombre de tramo.
- Con sonido, el arranque lanzado desde otra sesión gasta 7–8 s más antes del primer fotograma (tramo `arranque-sonido` en el
  registro nuevo); en la sesión normal de Sergio no se vio (primer fotograma a los 0,2 s).
- Lo que de verdad lo arregla es sacar `P3-Prueba` y el juego del disco mecánico o averiguar qué proceso ocupa `D:`.
