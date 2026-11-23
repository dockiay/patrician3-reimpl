# Oráculo de partida entera (10/11/2026)

Compara el bucle diario completo del ejecutable original con la simulación Rust,
día a día y sobre guardados reales. Busca la primera muestra distinta y traduce
los primeros bytes distintos a la estructura del juego (ciudad, oficina,
comerciante, barco, convoy, tarea o carta).

## Método

`oracle/whole_game_corpus.py` ejecuta los dos lados con el mismo guardado y el
mismo perfil: CRT 123, fiesta `0:255`, anfitrión local y jugador = propietario
que deduce la sesión Rust (36 en todo el corpus).

- **Original.** `oracle/x86_c10_real_save_clock.py` usa emulación Unicorn del EXE
  inglés (`fe436dc5…`). Carga la partida con los lectores originales (`0x549F40`)
  y avanza con el reloj `0x530E80`, la cola de operaciones `0x546870` y la
  resolución de batallas de IA. Cada día vuelca el guardado con los
  serializadores originales (`0x549CC0`/`0x549C60`). Una caché de rutas
  (`--route-cache`, pickle comprimido) evita repetir el cálculo de rutas de la carga.
- **Rust.** `p3-mvp/examples/c10_snapshots` abre la sesión, avanza con
  `headless::advance_ticks` y vuelca el guardado descomprimido de cada muestra.
- **Contraste.** `compare_c10_real_snapshots.py` compara los ocho bloques. Solo
  normaliza las direcciones de proceso (puntero de ruta de barco, guion y
  argumentos de la tarea 0x1B, y cartas con blob). Las muestras se comparan solo
  donde existen en los dos lados.

```bash
cd reimpl && cargo build --release -p p3-mvp --example c10_snapshots
cd oracle && python3 whole_game_corpus.py --binary Patrician3.exe \
  --cpr p2arch0_eng.cpr --cpr p2arch1_eng.cpr --save AUTO.pat --days 365 \
  --work /ruta/trabajo --native ../target/release/examples/c10_snapshots \
  --instructions 50000000000 --output informe.json
# --skip-original reutiliza las muestras x86 ya guardadas en --work y repite solo Rust.
```

El EXE, los CPR, los guardados y los volcados se quedan fuera de Git. Con siete
procesos en paralelo en la nube, el lado x86 tarda unos 10 minutos por guardado
para un año y el lado Rust de 2 a 5 minutos.

Para depurar una divergencia, `P3_X86_WATCH='507d60,5063c0;TICK'` hace que el
reloj x86 imprima en stderr cada entrada a esas direcciones a partir de `TICK`:
retorno, cuatro argumentos de pila y ECX. Esta traza solo observa y no cambia la
ejecución. Así se localizó la corrección 5.

## Resultado a 365 días (10/11, tarde)

Seis guardados distintos: `ConvoyMar` y `TEST25_ConvoyMar` tienen el mismo
contenido. Con las opciones del apartado siguiente, **los dos lados completan los
365 días en los siete recorridos y todas las muestras diarias coinciden byte a
byte** (367 muestras por recorrido: la del primer tick y una por día, del 0 al 365). Informe compacto:
`oracle/reports_whole_game_365.json`.

| Guardado | Muestras iguales | Batallas con jugador resueltas como IA (x86) |
|---|---|---|
| fps-cache AUTO | 367/367 | 0 |
| ConvoyAvista | 367/367 | 2 (días 175 y 222) |
| ConvoyHecho | 367/367 | 0 |
| ConvoyMar | 367/367 | 1 (día 175) |
| ConvoyPrueba | 367/367 | 2 (días 175 y 222) |
| TEST25_ConvoyMar | 367/367 | 1 (día 175) |
| TestAstiller | 367/367 | 0 |

```bash
python3 whole_game_corpus.py ... --days 365 --human-battles-auto \
  --siege-tails-from-original --output informe.json
```

### Paradas quitadas

**Original emulado.** Las paradas de los días 175–323 eran llamadas de interfaz
con el puntero de ventana nulo, porque la emulación no crea ventanas. El reloj
x86 sustituye ahora cinco funciones de presentación (más `0x44F940`, caché gráfico de ruta, que ya estaba) por un retorno con la
limpieza de pila correcta (`report.presentation_adapters`). Ninguna escribe en
el estado que se guarda:

| Función | Ventana | Llamada desde |
|---|---|---|
| `0x5C6BA0` redibujar ventana hanseática (`[0x6E5558]`) | subasta | `st_auction` `0x4E30ED` |
| `0x5C6740` añadir resultado de subasta (ret 0x14) | hanseática | subasta |
| `0x5C6BF0` subasta terminada | hanseática | subasta |
| `0x494990` menú lateral del barco (`[0x6CE6D0]`) | barco retirado | `0x509353`, `0x54070E` |
| `0x4778A0` lista de barcos (`[0x6CC950]`) | barco retirado | `0x509367`, `0x540726` |

La parada «EIP en la pila» tras `0x41D9B0` era el diálogo de abordaje que abre
la actualización de batalla (`0x606B8D`) cuando el jugador local participa. Con
`--human-battles-auto` el x86 aplica la misma política que Rust
(`C07_HEADLESS_HUMAN_BATTLES.md`): en `SeaBattle::start` (`0x6056A9`) marca los
bandos `0x5D/0x5E/0x5F` como IA, lo que lleva a `0xAC8=1` y evita los avisos
`0x548CA0` y la sincronización `0x612C90`; durante la actualización
(`c10_real_battle_driver.py`) el jugador local `0x6DFC14` vale `0xFFFFFFFF`.
El informe lista cada batalla así resuelta (`human_battles_auto`).

**Rust.** `c10_snapshots --human-battles-auto` activa
`NullHost::headless_human_battles`. Las rutas compartidas y la banda pirata
dejaron de pararse al corregir las diferencias 6 y 8. El asedio necesitaba los
tres bytes de pila sin inicializar que lee la demanda `0x635220` (`esp+0x15..0x17`).
El reloj x86 los registra en `0x63529F` (`siege_stack_tails`, 288 llamadas por
año) y `--siege-stack-tails INFORME_X86.json` los da a Rust por tick
(`SimHost::siege_stack_tails`). Casi siempre valen `[1,0,0]`; en ConvoyHecho
también `[0,0,0]`.

## Correcciones que salieron del contraste

1. **Ciudades conocidas por barcos atracados** (`0x00535400`). El tick naval
   original marca la ciudad en el historial de información por cada barco humano
   atracado al empezar el día. `TickHost`/`SeaHost` emiten ahora `conocer:ciudad:dueño`
   y `NullHost::apply_navigation_town_knowledge` lo aplica con `mark_town_known`.
   Prueba: `p3-mvp/tests/whole_game_town_knowledge.rs`, con fixture x86 de TestAstiller.
2. **Casamentero.** Las ofertas 0x21 y 0x22 (`0x004E8234`) dividen por el contador
   de apellidos ordinarios `0x006DDB74` (`ordinary_family_name_count`), no por el
   contador de nombres de préstamos.
3. **Temporizadores meteorológicos.** `0x00504230` pone a cero al cargar el
   temporizador visual de cada registro meteorológico (`+0xA`), y el guardado
   original escribe ese valor. Rust conservaba el valor del archivo. ConvoyAvista
   lo mostró desde la primera muestra (`persist.rs`; prueba en el mismo test).
4. **Marca de vecino en la oficina** (`0x004FF2D0`). Ante un vecino del mismo tipo,
   el original guarda el índice del sucesor inmediato (EBX no avanza en
   `0x004FF525`). Además escribe `0xFF` antes de mirar el recuento del tipo. Se
   vio en ConvoyPrueba, día 150. Nuevo oráculo `oracle/x86_office_adjacency.py`:
   600 casos con vecinos no consecutivos y la prueba
   `p3-core/tests/x86_c04_office_recount.rs`. La implementación anterior falla esos casos.
5. **Barco nuevo de banda pirata en una captura** (`0x00516170` desde `0x00515F70`).
   El original coloca el barco en el lugar de la banda de la tabla `0x006DDBB0`,
   indexada directamente por `banda+0xE`. `PirateCaptureState` usaba la lista de
   puertos por ciudad, que pasa por el mapa `0x006DE4B8`. Se vio en AUTO, día 315.
   `PirateCaptureState::locations` lleva ahora la tabla de lugares. El oráculo
   `x86_c10_pirate_capture.py` siembra un mapa ciudad→lugar distinto de la
   identidad, así el fixture distingue los dos caminos; el código anterior lo falla.

6. **Origen de una ruta compartida desviada** (ConvoyHecho, día 138).
   `navigation::divert` toma la ciudad de origen de la tabla de ciudades del
   planificador (entrada con `p % ciudades == destino`) y solo recurre a
   `imagen[0x37]` si no la hay. Antes el guardado Rust rechazaba la ruta.
7. **Orden de las cartas de una batalla** (ConvoyPrueba, día 223). El cierre de
   batalla intercala los informes 0x29 con las noticias en el orden del original
   (`FinishEffects::report_positions`, `BattleAdvance::report_positions`).
8. **Banda pirata al retirar un barco en el mar** (ConvoyPrueba, día 245).
   `remove_at_sea` desenlaza la cadena de la banda también cuando el puntero de
   gestor es cero pero la banda existe en `absorption.bands`.
9. **Carta 0x54 de capitán pirata capturado** (ConvoyMar, día 176). El original
   reutiliza una carta 0x2A y conserva su byte `+14`. Nueva
   `messages::add_pirate_captain_record`, que copia solo la carga `+8..+16`.
10. **Comparador: tarea libre** (TestAstiller, día 364). Una tarea libre (tipo
    `0xFFFF`, `+12 == 0`) conserva en `+8` un puntero viejo de guion. El
    comparador lo normaliza como dirección de proceso (`stale_script_pointer`).

## Límites

- Los adaptadores x86 solo quitan presentación; no se ha comprobado en el juego
  real que esas ventanas no influyan en algo que no se guarda.
- Las batallas con jugador se resuelven como IA en los dos lados: no reproduce
  lo que habría decidido una persona.
- Los bytes de pila del asedio vienen del original emulado. En el juego real
  dependen de lo que haya dejado en la pila la llamada anterior; Rust no los
  deduce por sí solo y, sin `--siege-stack-tails`, usa el valor fijo anterior.
- La comparación es sobre el guardado. Lo que el original no serializa (escena,
  interfaz, sonido) queda fuera.
- Mismo perfil en los dos lados: no cubre otras semillas CRT, fiestas ni un cliente de red.
- Sin prueba en partida.
