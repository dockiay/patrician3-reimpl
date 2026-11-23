# J04: control de audio

Entrega del 26/10/2026, PC de Sergio. Primer bloque de la ruta acordada
**audio → interfaz → gráficos**, separado de la simulación de Claude.

`p3-audio` reproduce el control de canales, volúmenes, panorámica y fundidos.
El decoder MP3 usa Symphonia0.6.1; [dependencias y licencias](THIRD_PARTY.md).
`Backend` recibe identificadores opacos y
servicios explícitos; el reloj de presentación y la presencia de la tabla auxiliar
de efectos entran como argumentos. El selector regional pide el azar por un
puerto al flujo CRT compartido con la simulación; no crea ni siembra otro RNG.
El backend nativo conecta ya recursos, decoder, mezcla y muestras/caché del gestor
sin dispositivo; alcance actual y reproducción en los bloques 12–15.

## Funciones implementadas y contrastadas

| EXE inglés | Método de `SoundManager` |
|---|---|
| `0x00442130` | `count_playing_samples` |
| `0x00442180` | `close_finished_streams` |
| `0x00442210` | `is_ambient_stream_active` |
| `0x00442230` | `is_music_stream_active` |
| `0x00442D10` | `stop_all_streams` |
| `0x00442D90` | `set_music_volume` |
| `0x00442E30` | `set_voice_volume` |
| `0x004430D0` | `clear_sample_slot_flags` |
| `0x00443110` | `is_sound_1bbc_playing` |
| `0x004438D0` | `set_sample_pan` |
| `0x00443940` | `stop_all_samples` |
| `0x00444030` | `update_fades` |
| `0x00444370` | `fade_out_music` |
| `0x004443D0` | `fade_out_ambient` |

El segundo bloque añade `set_effects_volume` (`0x00442E90`),
`update_sample_volume` (`0x00443000`) y `play_sound` (`0x00443150`). El tercero,
`get_sound_list_size` (`0x00443690`) y `load_sound_list` (`0x004439B0`).
El cuarto añade `play_music` (`0x004426C0`), `play_ambient_stream` (`0x00442A10`),
`get_music_remaining_ms` (`0x00442C20`) y `get_file_duration_ms` (`0x00442C60`).
El quinto añade `play_building_music` (`0x00442280`), `play_speech_stream`
(`0x00442460`), `end_speech_and_resume` (`0x004425D0`), `update_speech`
(`0x00442690`) y `stop_building_music` (`0x004438A0`). El sexto añade
`set_lowpass_filter` (`0x004436E0`). El séptimo añade tres selectores de
`ScreenAudio`: `update` (`0x0042C080`), `start_atmosphere` (`0x0042C770`) y
`start_town_music` (`0x0042CD90`). El octavo añade `initialize` (`0x004419A0`),
`destroy` (`0x00441AB0`), `shutdown` (`0x00441AC0`) y
`load_building_music_config` (`0x00441CD0`). El noveno añade `open_audio_driver`
(`0x00441E10`) y `start_service` (`0x004429D0`). El undécimo añade las diez
entradas `SoundEffectList`: **48 funciones de J04** registradas. El catálogo
regional del bloque 10 sigue siendo un tramo, no otra función completa.
Sus corpus y límites se detallan al final.

Se conservan los detalles observados en el ensamblador:

- Sin dispositivo, contar devuelve cero pero no borra el contador guardado.
  Las consultas de pista pueden devolver verdadero si conservan un handle;
  iniciar y actualizar fundidos no comprueba el dispositivo.
- Cerrar pistas terminadas examina música actual, ambiente actual y edificio.
  Parar las pistas conserva edificio, voz y la bandera de fundido de ambiente.
- Los setters comparan el volumen sin saturar con la respuesta del backend;
  saturan solo el valor que envían. La multiplicación conserva el desbordamiento
  de 32 bits y la división con signo trunca hacia cero.
- La panorámica se invierte antes de truncarla a u16; no se limita a 0..127.
  Las búsquedas de slot o ID se detienen en la primera coincidencia.
- El fundido usa tiempo de presentación con resta u32. La igualdad con el
  objetivo no termina todavía el fundido entrante. El saliente puede borrar la
  bandera compartida aunque el entrante siga pendiente. Música con modo cero
  conserva la pista al alcanzar el suelo, con una segunda consulta de volumen.
- Volúmenes negativos extremos pueden producir intervalo cero: el x86 lanza
  una excepción DIV. Rust conserva el estado y las llamadas previas y devuelve
  `FadeError::ZeroInterval`; no intenta reproducir un fallo del procesador nativo.

## Evidencia reproducible

Fuentes de interpretación: [revisión estática de audio](../../research/systems/revision-r01/priority-06-audio/README.md)
y [bancos de audio](../../research/systems/borde-motor/audio-banks/README.md).
El oráculo es [x86_audio.py](../oracle/x86_audio.py); verifica el SHA-256 del EXE
inglés antes de mapearlo con Unicorn. Nunca carga `mss32.dll` ni abre el juego.

```powershell
python reimpl/oracle/x86_audio.py --binary RUTA/Patrician3.exe --check
cargo test --locked --manifest-path reimpl/Cargo.toml -p p3-audio
cargo clippy --locked --manifest-path reimpl/Cargo.toml -p p3-audio --all-targets -- -D warnings
cargo fmt --manifest-path reimpl/Cargo.toml -p p3-audio --check
```

- 202 secuencias, **3.852 llamadas** originales; 35 excepciones DIV controladas.
- Las dos salidas de las 77 ramas condicionales del bloque aparecen en la traza:
  **154/154 aristas**. Esto no demuestra exhaustividad de entradas ni de Miles.
- Tras cada llamada se contrastan retorno definido, excepción, SHA-256 de los
  484 bytes del objeto, estado del backend guionizado y llamadas en orden.
  También se preservan bytes ajenos al método, inicialmente aleatorios.
- Fixture `tests/fixtures/audio.json`: SHA-256
  `1d4413961aa44a44c629a877b5fe3fe614c0d61b279a8df6245304a5a099db98`.
  Incluye semilla, hash del generador y del arnés, rangos y hashes de cada función.
  Contiene entradas y resultados propios, nunca bytes del ejecutable.
- Sensibilidad comprobada: quitar la inversión de panorámica hace fallar la
  comparación de llamadas (secuencia 3, paso 6); se restauró el código y pasó.
  Un archivo con hash incorrecto es rechazado antes del análisis PE.

## Límites y siguiente bloque

`from_image` recibe una imagen del estado y conserva los campos no tratados.
Por sí solo no aplica el constructor. `initialize` reproduce sus escrituras
en el bloque 8, conservando los bytes que el original deja sin inicializar.
Los campos que contienen direcciones se conservan como bytes y no se dereferencian;
los métodos de este bloque usan solo los handles del backend. La imagen no es un
formato de partida ni una ABI para enlazar DLL originales.

El doble de backend responde a consultas y registra setters, pausas, cierres y
paradas. Sus cambios de estado están definidos en ambos arneses. No demuestra
decodificación, sonido audible, latencia, vida real de handles ni concurrencia.
El backend debe responder sin reentrar ni modificar el objeto durante una llamada.
El reloj se mantiene estable durante cada llamada del corpus; no se contrasta
una actualización concurrente de ese reloj entre dos servicios del backend.

Pendientes, en orden:

1. `SoundEffectList`: sus diez entradas de catálogo (`0x004C4130..0x004C4780`).
   La apertura y el cierre ya lo necesitan; su constructor sigue siendo un puerto.
2. Backend de salida nativo e integración de disparadores con el cliente.

J04 continúa abierta: integrar y jugar una sesión autónoma depende de C09/J03.
**Nada validado en partida; este bloque todavía no emite sonido.**

## Bloque 2: volumen, selección, caché y reproducción de muestras

`samples.rs` porta las tres funciones completas de este bloque. La selección
ocurre dentro de `play_sound`, por lo que se reproduce junto a las búsquedas,
la caché, las reservas y la configuración del canal. `SampleSlot` representa el
índice del slot que devolvía el original por dirección; `sample_handle` obtiene
su handle actual. El resultado no se confunde con el propio handle de Miles.

`SampleCatalog` aporta las tablas de IDs, ganancias u16, nombres y tamaños.
`SampleBackend` y `PlaybackBackend` añaden servicios de volumen, búferes,
Archiver, preparación, arranque y filtro. El backend de prueba se comparte con
el bloque anterior; el corpus anterior sigue pasando y su digest no cambia.

Comportamientos conservados:

- Volumen de efectos: primera coincidencia de ID, dos consultas del volumen de
  cada muestra válida, escalado desde su ganancia cuando vale cero o desde el
  volumen anterior en otro caso. Se actualiza aunque la muestra no esté sonando.
  El ambiente usa el canal actual o, si falta, el saliente; edificio y voz quedan
  fuera. El callback auxiliar recibe `volume * 100 / 127`, con producto i32.
- Volumen individual: búsqueda del primer handle, escalado en dos divisiones
  con truncado y producto i32 con vuelta; comparación sin saturar antes del setter.
- Reproducción: bloqueo de voz/edificio y reserva prueban el **bit cero**; no
  equivalen a cualquier byte no nulo. El ID `0x1770` consulta solo su primera
  coincidencia. Se cuentan 20 canales y se elige el primero sin reserva y sin
  estado de reproducción, incluido un handle cero si el backend lo permite.
- Aciertos de caché se promueven al frente. Los fallos asignan y leen antes de
  comprobar la expulsión. Una expulsión rechazada conserva la fuga del búfer
  nuevo; un fallo de apertura o lectura sí lo libera. El contador se incrementa
  al insertar en caché, aunque después el volumen cero o la falta de canal
  impidan arrancar. Esto es comportamiento original, no un contador corregido.
- Preparar sin arrancar conserva la configuración, panorámica, filtro y reserva.
  Si se arranca, el orden es volumen → arranque → servicio de ambiente → servicio
  de música. Un fallo de `set_named_sample_file` no evita por sí solo el arranque;
  su resultado condiciona la reserva. El filtro usa la última clave escrita.

### Corpus del bloque 2

En el PC de Sergio, cargar primero el entorno del repositorio principal con
`. D:/AI/Patrician3/scripts/Enter-P3Environment.ps1`, también desde el worktree.

```powershell
python reimpl/oracle/x86_audio_samples.py --binary RUTA/Patrician3.exe --check
cargo test --locked --manifest-path reimpl/Cargo.toml -p p3-audio
```

- **538 secuencias / 4.634 llamadas** al x86 inglés; 427 devoluciones de slot.
- **229 divisiones por cero y 32 desbordamientos de IDIV**: coinciden el estado
  parcial y las llamadas previas; Rust devuelve un error explícito.
- 148 de 156 aristas condicionales recorridas. Las ocho restantes se enumeran
  en el fixture con su invariante: índice ya encontrado menor que 20, handle
  seleccionado que existe o dispositivo estable. La exclusión depende del
  backend sin reentrada ni mutación del gestor; no se declara cobertura absoluta.
- Tras cada paso: retorno normalizado a índice, excepción, objeto completo,
  llamadas en orden y estado de canales y búferes, incluidas sus liberaciones.
- Fixture `tests/fixtures/audio_samples.json`, SHA-256
  `6a4568e1649c84dc149096bd81c075f38a558724767376eed2fd860aa3744b49`.
  Procedencia: hash del EXE, las tres funciones y los tres scripts usados.
- Tres mutaciones detectadas: divisor fijo incorrecto en volumen de efectos,
  división entre 100 en lugar de 127 en el volumen individual y liberar el búfer
  que el original pierde al rechazar una expulsión. Fuentes restauradas.

### Límites propios del bloque 2

Los servicios de Archiver y Miles se guionizan: no se implementa aquí su lectura
real, decodificación, procesamiento digital ni salida sonora. El corpus incluye
fallos de asignación, apertura, lectura corta y configuración. Los tokens de
búfer y los nombres sustituyen direcciones del proceso original.

Si falta un ID, `update_sample_volume` lee en el original una ganancia después
del vector. El corpus proporciona ese valor contiguo; sin él, Rust devuelve
`MissingCatalogData` y no lee fuera de su slice. Las cachés con IDs obsoletos se
contrastan solo con tablas de nombres/tamaños/ganancias contiguas conocidas.
No se atribuye un valor determinista a memoria desconocida.

El corpus de filtro tiene al menos un atributo escrito. Si el backend no escribe
ninguna clave, el original puede usar un local de pila sin inicializar: Rust
devuelve `MissingFilterKey`. Ambos límites seguros tienen una prueba propia,
separada de la paridad x86. La enumeración del backend debe terminar.

**Continuado en el bloque 3:** consultas y carga de listas/bancos, ciclo de vida de la
caché y conexiones con los recursos; después streams de música/ambiente/voz y
lowpass de pantallas. La integración y el backend de salida siguen pendientes.

## Bloque 3: carga de listas y catálogo desde recursos

`banks.rs` añade las dos funciones completas en el dominio de asignaciones
correctas, estado coherente y `Count` no negativo. `SoundBank` conserva las tablas
de IDs, nombres, ganancias u16, tamaños y listas, incluidos su orden y duplicados.
`name_refs`/`catalog` las conectan con las funciones de muestras del bloque 2.
Un escenario encadenado carga → consulta → volumen → recarga comprueba esa unión
contra el x86, reutilizando `0x00443000` sin contarla como otra función nueva.

`BankResources` hace explícitas las consultas INI y el ciclo de archivo.
`IndexedResources` las resuelve desde documentos e índices suministrados, sin
E/S en la biblioteca. Su lector cubre el formato simple del banco: secciones,
claves, valores y comentarios `;`; no se acredita como toda la clase INI original.

Comportamientos conservados:

- Sin dispositivo no se consulta el INI. Faltar `Files` o `IDs` termina la carga,
  incluso si `Count` era negativo. Con ambos presentes, el original no valida
  ese contador; Rust devuelve `NegativeCount` si es negativo.
- Se anexa `{lista, Count}` antes de consultar las muestras, también con Count
  cero. No hay deduplicación de listas ni IDs; la consulta devuelve la primera
  lista coincidente. Las muestras se añaden al final de las tablas anteriores.
- El cargador busca un espacio **desde el byte 1**, convierte el prefijo con
  `atoi` y borra hasta el separador. Sin otro espacio no borra nada: con Count
  mayor que el número de tokens repite el último. Tabs no son separadores.
  Se preservan signo, prefijos decimales y desbordamiento de 32 bits en locale C.
- Se consultan primero todos los nombres y ganancias; después las longitudes.
  Loudness usa defecto 100 y se trunca a u16. Un archivo ausente produce tamaño
  cero, y se llama a cerrar aunque abrir haya fallado. El modo es `0x8000`.

### Comparación con x86

- Corpus sintético: **161 secuencias / 966 llamadas**, incluidas dos llamadas a
  volumen ya portadas. El oráculo ejecuta también `CString::Find` y `atoi`
  originales en locale C; INI, Archiver y soporte C++ de memoria/strings se
  interceptan. Se comparan servicios en orden y todas las tablas tras cada paso.
- En la imagen de 484 bytes se normalizan los cinco punteros a tablas; se
  compara el resto, incluidos los contadores. `SoundBank` es dueño de las tablas
  Rust: las direcciones y el reparto de asignaciones C++ no son parte de la paridad.
  La imagen no se usa como partida ni para llamar al proceso original.
- **56/60 aristas** recorridas. Tres salidas restantes dependen de un fallo de
  asignación de arrays CString; otra, de un recuento de listas inconsistente o
  desbordado. Se enumeran en el fixture; no se presenta cobertura absoluta.
- SHA-256 de `tests/fixtures/audio_banks.json`:
  `8dadf3aa92291bca04e07b7921392c59df0f2942e79ea7b0efe88eb9eaabc227`.
- Tres mutaciones detectadas y restauradas: consultar las listas al revés,
  vaciar el último token y omitir el cierre con longitud cero/apertura fallida.
  El límite seguro de Count malformado tiene una prueba nativa separada.

### Datos ingleses reales, solo en local

El `scripts/sound.ini` del `p2arch0_eng.cpr` está disponible en este PC: 52
secciones de listas y 202 de muestras. El fixture local ejecuta **104 llamadas**
originales y la misma secuencia Rust. Resultado: **51 listas cargadas, 201
entradas, 200 IDs distintos**, todas con longitud no nula en el índice. Las
cabeceras RIFF de los **163 archivos distintos** confirman esas longitudes.
No se decodificó ni reprodujo el audio; Archiver sigue siendo un servicio
controlado, alimentado por el directorio del contenedor.

Discrepancias de datos conservadas:

| Lista | Count | IDs escritos | Resultado del cargador |
|---|---:|---:|---|
| 10860 | 9 | 10 | no carga 10869 |
| 10850 | 7 | 8 | no carga 10857 |
| 61090 | 4 | 3 | repite 61092 |

El [informe compacto](evidence/banks-real.json) guarda solo estadísticas y hashes
del EXE, INI, archivos, generadores y resultado. El INI y el fixture completo con
datos del juego permanecen en `artifacts/`, ignorado por Git. El CLI rechaza
guardar el fixture real fuera de esa carpeta. No se atribuye aceptación del
resto de las entregas ni paridad en partida a esta prueba.

```powershell
# Desde el worktree, con el entorno del repositorio principal ya cargado:
python reimpl/oracle/x86_audio_banks.py --binary D:/AI/Patrician3/games/iso-english/app/Patrician3.exe --check
python reimpl/oracle/x86_audio_banks.py --binary D:/AI/Patrician3/games/iso-english/app/Patrician3.exe --archive D:/AI/Patrician3/games/iso-english/app/p2arch0_eng.cpr --archive D:/AI/Patrician3/games/iso-english/app/p2arch1_eng.cpr --real-fixture artifacts/audio-banks-real.json --report reimpl/p3-audio/evidence/banks-real.json --check
$env:P3_AUDIO_BANK_FIXTURE=(Resolve-Path artifacts/audio-banks-real.json).Path
cargo test --locked --manifest-path reimpl/Cargo.toml -p p3-audio --test x86_audio_banks real_resource_bank_matches_original_x86 -- --ignored
```

Para regenerar un fixture local ausente, ejecutar el segundo comando sin
`--check`. La prueba comercial es optativa y marcada `ignored` en la suite
normal; se ejecutó explícitamente en esta entrega. Los dos corpus anteriores
siguen vigentes. Streams de música/ambiente y consultas de duración se entregan
en el bloque 4. No hay salida sonora ni validación en partida.

## Bloque 4: streams de música y ambiente y consultas de duración

`streams.rs` porta cuatro entradas y usa los controles de limpieza y fundido
ya contrastados. `StreamBackend` explicita apertura, posición, arranque, bucle y
la relación entre stream y muestra. No se dereferencia ningún puntero Miles en
Rust. La enumeración de filtro pasa a `FilterBackend` compartido con muestras;
el corpus del bloque 2 sigue pasando sin cambiar sus datos.

Comportamientos conservados:

- Música: sin dispositivo, con hilo de fundido o con música de edificio no abre.
  `force` solo omite la comprobación de volumen escalado cero. Los argumentos
  de volumen, sustitución, bucle y fuerza son bytes; un flag no nulo se activa.
- La sustitución termina las 20 muestras sin borrar sus indicadores. Retira
  la música actual con fundido, cierra su saliente anterior y abre a volumen
  escalado y saturado. Consulta la duración pero **devuelve cero al tener éxito**.
  El escalado es `volume_byte * music_setting / 100`, con producto i32 con vuelta
  y truncado hacia cero; la llamada se limita a 0..127.
  Cierra ambiente actual y saliente. Si la primera apertura falla, cae al camino
  normal y puede **abrir el mismo archivo otra vez**.
- El camino normal no abre si aún hay música actual. Retira el ambiente antes
  de intentar abrir, así que un fallo conserva esa transición. Al abrir inicia
  con volumen cero y fundido, guarda el objetivo y devuelve la duración total.
- Ambiente: comprueba dispositivo/edificio y limpia las pistas terminadas
  **antes** de comprobar el volumen de efectos. Una apertura fallida deja la
  pista anterior como saliente. Una correcta ajusta posición, consulta total,
  retira la música, inicia con volumen cero y activa el fundido y el bucle.
- El retorno de ambiente es `total - inicio_pedido` con vuelta u32, aunque el
  backend haya ajustado la posición efectiva. No se corrige a una resta saturada.
- La duración restante de música consulta solo la pista actual; sin ella vale
  cero aunque haya saliente. Sin dispositivo devuelve el patrón `u32::MAX`
  (el -1 original). La resta `total - actual` también conserva la vuelta u32.
- La consulta de duración de archivo abre una pista temporal, consulta su total,
  pausa y cierra, sin guardarla en el gestor. En apertura se usa `0x3E800` y al
  solicitar bucle se envía contador cero; la semántica interna de Miles no se emula.
- El filtro del ambiente enumera primero los atributos; después obtiene la
  muestra de la relación original `HSTREAM+0xC` para configurar procesador y
  preferencia 400.0. La relación la aporta el backend, no una estructura DLL nativa.

### Evidencia y límites del bloque 4

- **398 secuencias / 4.627 llamadas** originales. Se contrastan retorno u32,
  los 484 bytes del gestor sin normalizar punteros y las llamadas en orden;
  además el estado de canales, duraciones, posiciones, muestras, bucles,
  arranques y número de aperturas del backend.
- **69/74 aristas** recorridas. Las cinco restantes pertenecen al intento de
  restaurar la música saliente: requieren que el handle recién abierto se vuelva
  nulo dentro de un servicio del backend. Quedan enumeradas con esa invariante.
  El contraste exige backend sin reentrada ni mutación del gestor, reloj estable
  durante cada llamada y una clave de filtro conocida. No cubre concurrencia.
- `tests/fixtures/audio_streams.json`, SHA-256
  `2a6ac7c5a71c8139163f35e61317c652d4cdd4d0d47c3c3a021d633e3aa48d53`.
  Lleva hashes del EXE, las cuatro funciones y los scripts de procedencia.
- Cuatro mutaciones detectadas y restauradas: suprimir el segundo intento de
  apertura, saturar la resta de ambiente, devolver cero sin dispositivo en la
  consulta restante y omitir el cierre de la pista temporal.
- Si no se escribió una clave de filtro, Rust devuelve `FilterError::MissingKey`
  después de iniciar la pista y conserva ese estado parcial. La prueba de este
  límite seguro es independiente del corpus x86, cuyo local original sería
  desconocido. El bloque anterior conserva su error `SampleError::MissingFilterKey`.

```powershell
python reimpl/oracle/x86_audio_streams.py --binary D:/AI/Patrician3/games/iso-english/app/Patrician3.exe --check
cargo test --locked --manifest-path reimpl/Cargo.toml -p p3-audio
```

Las duraciones y respuestas son guionizadas: **no se decodificó ni reprodujo
audio ni se cargó Miles**. No hay validación en partida. Voz y música de edificio
se entregan en el bloque 5; después siguen lowpass, bancos por región y ciclo de vida.

## Bloque 5: voz, música de edificio y reanudación

`voice.rs` porta las cinco entradas y reutiliza los controles anteriores de
limpieza, muestras, streams y filtro. Los títulos se suministran como una lista
prestada en lugar de dereferenciar la tabla CString de `SoundManager+0xCC`.

Comportamientos conservados:

- Edificio limpia primero las pistas terminadas. Una pista de edificio existente
  bloquea el inicio por índice si no ha terminado. El índice `0xFFFF` usa el
  archivo directo y **pausa/cierra la pista previa incluso si el handle es cero**,
  antes de comprobar volumen o flag. Los demás índices son u16.
- Con tutorial desactivado, el guard prueba el **bit cero** del byte flag. El
  volumen de edificio usa efectos, limitado a 0..127, y permite valores negativos
  no nulos como volumen cero tras saturar. Si abre, retira la música actual con
  modo de fundido cero, activa `+0x1E0` y arranca. Solo fuera de tutorial envía
  contador de bucle cero.
- Tutorial aplica el filtro a los **ambientes actual y saliente**, no a la nueva
  pista de edificio. Enumera atributos incluso sin ambientes. Si no hay ningún
  ambiente no consume la clave y admite que la enumeración no escriba ninguna.
- Voz interpreta volumen como **i8**, lo multiplica por el ajuste i32 con vuelta
  y divide entre 100 antes de saturar. Solo el ajuste cero desactiva esta ruta;
  un argumento de volumen cero puede todavía iniciar una pista muda.
- Antes de abrir voz, limpia la tabla auxiliar si existe, detiene/termina solo
  muestras con estado 4, borra indicadores y cierra salientes. Pausa la música
  actual y el ambiente actual. **Sobrescribe la voz anterior sin cerrarla**,
  también cuando falla la nueva apertura. No corrige esa pérdida del original.
  Solo una apertura correcta escribe `+0x1DF=1`.
- Terminar voz cierra la pista y reanuda música/ambiente solo en **estado 8**.
  Música se reanuda con volumen cero y fundido; ambiente se reanuda sin esos
  cambios. Borra `+0x1DF` con dispositivo, incluso sin voz; sin dispositivo lo
  conserva. `update_speech` lo invoca solo con voz no nula y estado 2.
- Parar edificio no limpia `+0x1E0` ni reanuda música. Las otras funciones de este
  bloque tampoco limpian esa bandera; su desactivación corresponde al lowpass.

### Evidencia y límites del bloque 5

- **340 secuencias / 3.231 llamadas originales**, más **641 estímulos del backend**
  que señalan fin de voz. Esos estímulos no son funciones ni llamadas x86 nuevas.
  Se comparan los 484 bytes del gestor, servicios en orden y estado de canales,
  pausas, cierres, volúmenes, muestras, bucles y aperturas tras cada paso.
- **79/84 aristas** recorridas. Las cinco restantes restaurarían música saliente
  si un servicio cambia el handle del gestor durante la reanudación; la invariante
  es backend sin reentrada ni mutación del gestor. No se prueba concurrencia.
- Las respuestas de pausa/estado y los archivos son guionizados; no se acredita
  el funcionamiento interno de Miles ni salida sonora. El modelo de pausa se
  declara en cada secuencia y no se impone como semántica real de la DLL.
- `tests/fixtures/audio_voice.json`, SHA-256
  `bcaa5fedcbe79850b71fe6116baf4a409c195638ee8db474de9a91c41175df68`.
  Incluye hashes del EXE, las cinco funciones y cuatro scripts de procedencia.
- Se detectaron y restauraron seis mutaciones: flag no nulo en vez de bit cero,
  volumen sin signo, cerrar la voz previa, reanudar fuera del estado 8, finalizar
  en estado 4 y borrar la bandera al parar edificio. Los corpus anteriores siguen
  pasando; el backend de pruebas de streams se comparte sin cambiar sus datos.
- Índices válidos o `0xFFFF`, reloj estable y clave conocida **cuando se consume**
  delimitan el contraste x86. Sin título disponible Rust devuelve `MissingTitle`;
  si falta una clave usada devuelve `MissingFilterKey` después de abrir/configurar
  edificio pero antes de arrancarlo. Son límites seguros probados aparte, no
  paridad con memoria desconocida. El caso sin ambientes y sin clave sí se compara
  con el x86 porque no lee esa memoria.

```powershell
python reimpl/oracle/x86_audio_voice.py --binary D:/AI/Patrician3/games/iso-english/app/Patrician3.exe --check
cargo test --locked --manifest-path reimpl/Cargo.toml -p p3-audio
```

El lowpass que depende de este bloque se entrega a continuación. J04 y la
integración del cliente continúan abiertas. Sin juego ni DLL cargada.

## Bloque 6: filtro lowpass de pantallas

`lowpass.rs` porta `set_lowpass_filter` (`0x004436E0`) y reutiliza muestras,
edificio y filtro. Conserva el byte de activación sin normalizarlo a bool y el
índice de edificio u8; este último nunca puede tomar el valor directo `0xFFFF`.

- Sin dispositivo no escribe ni llama al backend. Al activar enumera atributos,
  aplica el último a los ambientes actual y saliente con frecuencia 400, limpia
  muestras y llama al inicio de edificio. No borra las banderas de voz/edificio.
- Desactivar quita ambos procesadores. Si falta música actual, restaura la
  saliente y su volumen objetivo; si hay música, programa fundido entrante con el
  reloj recibido. No emite una pausa/reanudación ni un setter de volumen aquí.
- Fuera del tutorial para el edificio. En tutorial conserva su pista, pero en
  ambos casos limpia `+0x1DF` y `+0x1E0`. La pista de voz permanece abierta.

### Evidencia y límites del bloque 6

- **274 secuencias, 1.050 llamadas lowpass y 769 a controles originales ya
  portados**, más **256 estímulos** que marcan fin de voz. Se comparan tras cada
  paso la imagen completa de 484 bytes, servicios en orden y estado del backend.
- **31/34 aristas**. Dos salidas requieren que el backend cambie el dispositivo
  del gestor durante la llamada; otra exige que EAX deje de ser cero sin ninguna
  instrucción que lo cambie. Se enumeran las tres en el fixture. Backend sin
  reentrada, reloj estable y enumeración finita delimitan la prueba.
- `tests/fixtures/audio_lowpass.json`, SHA-256
  `6161281ab8ea4846d71259998bf7e8295137ba2e84f528e0463224bbcba43354`.
  Guarda hashes del EXE, el rango de la función y los cinco scripts de procedencia.
- Cinco mutaciones detectadas y restauradas: normalizar enable, perder el objetivo
  restaurado, cerrar edificio durante tutorial, conservar la bandera de voz y
  omitir la limpieza de muestras. Los corpus anteriores siguen pasando con sus
  datos originales; voz y lowpass comparten el soporte de pruebas.
- Una enumeración vacía sin ambientes es válida y se contrasta con x86. Con
  ambientes, Rust informa `MissingFilterKey` antes de instalar el procesador.
  Un índice sin título informa `MissingTitle` después de la limpieza de muestras.
  Ambos estados parciales tienen prueba nativa separada; no se acredita paridad
  con el acceso a memoria desconocida del original.
- Archivos, pausa, filtros y estado de canales son servicios guionizados.
  **No hay procesamiento digital, salida sonora ni validación en partida.**

```powershell
python reimpl/oracle/x86_audio_lowpass.py --binary D:/AI/Patrician3/games/iso-english/app/Patrician3.exe --check
cargo test --locked --manifest-path reimpl/Cargo.toml -p p3-audio
```

La selección por región se entrega en el bloque siguiente.

## Bloque 7: selección regional y modos de pantalla

`region.rs` reproduce tres funciones de `GameScreen`. `ScreenAudio` conserva el
tramo `+0x287C..+0x28CB` (80 bytes), incluidos contadores, punteros opacos y plazos.
No representa toda la pantalla ni reconstruye su constructor. `Catalog` presta
los nombres y volúmenes; `Scene` recibe los campos estables del mundo y opciones.
Los recuentos deben corresponder a esos catálogos. El bloque 10 añade su carga
desde `sound.ini`, como parte acotada del constructor de la pantalla.

`ScreenHost` explicita cada consumo de azar y consulta de interfaz, clima y mes.
El cliente debe conectar `random` al **mismo** `p3_core::rng::CrtRand` que utiliza
la simulación, sin resiembra propia. En las pruebas recibe una secuencia conocida;
el generador no se duplica en audio. `NativeScreenServices` conecta el selector a
`SoundManager` y los controles ya portados, con el mismo reloj fijo de `Scene`.

Comportamientos conservados:

- Voz activa actualiza voz y sale antes de los fundidos. Estos se actualizan si
  alguna bandera está activa. Los plazos se comparan como u32 con desigualdad
  estricta; la duración restante se compara como i32, incluso si devuelve -1.
- Selección: región 0..4 usa recuento y prefijos; ciudad fuera del mundo usa
  Region5 y sus prefijos. Una región mayor que 4 deja índice cero; Region6 no se
  lee. El modo mar usa su recuento sin prefijo. Se compara el índice completo con
  el anterior guardado como byte y se trunca solo al guardarlo.
- Se consulta la pantalla superior dos veces y la visibilidad aunque ya haya
  coincidencia. Antes de reproducir se repite la selección de banco por pantalla.
  Repetir índice difiere música 20.000 ms y deja ambiente vencido. Las sumas y
  restas de temporizadores conservan su vuelta u32.
- Modo 1: con música activa, sin fuerza y `viewed_town <= towns_count`, conserva
  música y consulta dos veces su duración restante. La prueba del ensamblador es
  `towns_count < viewed_town`, no la comparación inversa. Modo 2 usa Region5,
  sustituye inmediatamente y reutiliza el mismo aleatorio para su plazo.
- En modo 0, duración y reproducción intercambian las pistas de ambiente 0 y 1;
  la pista meteorológica 2 mantiene su posición. Clima y visibilidad vuelven a
  consultarse, por lo que pueden devolver valores sucesivos. La batalla reproduce
  con volumen 100, aunque su catálogo declare otro volumen.
- El mes decide ambiente marino: 0 o mayor que 10 elige la segunda pista.
  `start_town_music` traduce el argumento u8 por `town_ids`; la selección regional
  de `update` usa directamente el índice mundial, sin esa traducción.
- Se conservan las conversiones i32 → f32 almacenado → producto x87 → truncado,
  incluso con duración mayor que INT_MAX o no representable exactamente en f32.
  Las duraciones son respuestas del backend, no valores medidos del audio real.

### Evidencia y límites del bloque 7

- **324 secuencias / 2.565 llamadas** a las tres funciones con puertos controlados;
  se comparan el tramo completo de pantalla y todos los servicios, argumentos,
  retornos y consumos de azar en orden. Los bytes ajenos al método se conservan.
- **16 secuencias / 80 llamadas adicionales** ejecutan selectores y controles
  `SoundManager` originales juntos. Coinciden además sus 484 bytes y todo el estado
  de canales, posiciones y aperturas del backend. Las llamadas internas a controles
  ya portados no se cuentan como funciones nuevas.
- **98/100 aristas condicionales**. Las otras dos exigen que el plazo recién
  calculado coincida con el reloj estable, pero su diferencia es 15.000..19.999.
  Las dos invariantes están enumeradas en el fixture. No se prueba concurrencia.
- `tests/fixtures/audio_region.json`, SHA-256
  `5fb5468d8edde24abc637c5574d318cea96d3d3f421eb55cf5f7fe680ae4f343`.
  Guarda hashes del EXE, las tres funciones y cinco scripts de procedencia.
  Nombres, catálogos y respuestas son sintéticos, sin audio comercial.
- Ocho mutaciones detectadas y restauradas: quitar prefijos, cambiar el intervalo
  de meses, invertir ciudad vista, comparar duración sin signo, omitir el redondeo
  f32, saltar `town_ids`, quitar sustitución inmediata y corregir indebidamente la
  inversión de pistas. Los fixtures anteriores conservan sus datos.
- El bucle de búsqueda de posición puede no terminar si la pista es demasiado
  corta o el azar no da una posición válida. `Atmosphere::seek_limit` acota sus
  intentos; agotarlo devuelve `SeekLimit` conservando estado y consumos previos,
  sin iniciar ambiente. No se declara paridad de terminación con el original.
  Hay pruebas nativas separadas para ese límite, falta de azar, valores fuera de
  0..32767 y tablas incompletas. No se leen slices fuera de límites.
- **Sin interfaz real, decodificación, salida sonora ni validación en partida.**
  El adaptador une la lógica de selección y el gestor; Miles sigue guionizado.

```powershell
python reimpl/oracle/x86_audio_region.py --binary D:/AI/Patrician3/games/iso-english/app/Patrician3.exe --check
cargo test --locked --manifest-path reimpl/Cargo.toml -p p3-audio
```

El ciclo de vida y la configuración se entregan en el bloque siguiente.

## Bloque 8: constructor, configuración y cierre

`lifecycle.rs` añade cuatro funciones completas con servicios de memoria C++,
INI, Miles y espera explícitos. Los tokens son valores u32 del adaptador; nunca
se convierten en punteros Rust. `LifecycleMemory` conserva su propiedad y puede
rechazar tokens inválidos. `AudioGlobals` aporta el objeto de servicio y su bandera.

- `initialize` conserva muestras `+0x18..+0x67`, proveedor `+0x6C`, títulos `+0xCC`
  y todos los demás bytes que el original no escribe. Borra solo un word en
  `+0x1B4`, mantiene sus dos bytes altos y fija los tres volúmenes en 80. Devuelve
  la misma instancia; no reserva canales, archivos ni memoria C++.
- La configuración lee `[Gebaeude]` de `scripts/sound.ini`, trunca `Titel` a u16
  y reserva un array de CString incluso con cero títulos. Copia `Titel0..N-1`
  con capacidad 4096; ausente equivale a vacío. Trunca el modo a byte y convierte
  cero en 2, conservando los demás valores. Devuelve nombres Rust que se pueden
  prestar a `play_building_music`; la imagen guarda el token que posee el adaptador.
- Recargar títulos sobrescribe `+0xCC` sin liberar su asignación previa. No se
  oculta esa pérdida de propiedad liberando por anticipado desde la crate.
- Cierre libera primero IDs, nombres, tamaños y ganancias, aunque no haya driver.
  Con driver termina/libera las 20 muestras (también handles cero), borra sus
  slots y libera 20 buffers bloqueados **sin borrar sus tokens**. Cierra streams
  en orden música, música saliente, edificio, ambiente, ambiente saliente, voz.
- Solicita espera de 1000 ms, destruye el objeto de servicio si existe, borra la
  bandera global y cierra driver. No borra el token del objeto de servicio. La
  espera es un servicio, no un `sleep` en la biblioteca ni prueba de sincronización.
- Libera las listas según contador i32 positivo; después **incrementa `+0x138`**,
  libera la tabla incluso con token cero y conserva `+0x12C`. Borra fundidos y
  destruye títulos. Las banderas de voz/edificio y otros campos persisten.
  Por ello el cierre original no es idempotente: un cliente debe controlar el
  ciclo de vida y el adaptador debe rechazar un token ya liberado.
- `destroy` llama al cierre y luego solicita apagado global de la biblioteca,
  también sin driver. Ante un error nativo seguro deja de avanzar: no pretende
  continuar la ejecución original sobre memoria desconocida.

### Evidencia y límites del bloque 8

- **260 secuencias / 474 llamadas originales**, **42/42 aristas condicionales**.
  Después de cada paso coinciden los 484 bytes, globales, estado de canales,
  tablas de palabras y strings, tokens asignados/liberados y servicios en orden.
  Se prueba count mayor que 255, truncado u16, modos con bits altos, nombres
  ausentes/largos y fallo de asignación con count cero.
- `tests/fixtures/audio_lifecycle.json`, SHA-256
  `c3ecfd9b5ed1fcbd526c8b1d5dbf003f144e3c9a60112aafa8a5097ddf1f133a`.
  Incluye hashes del EXE, las cuatro funciones y los cinco scripts de procedencia.
- Ocho mutaciones detectadas y restauradas: borrar CC en constructor, borrar
  un dword de contador, volumen inicial 100, count u8, no sustituir modo cero,
  reiniciar contador de listas, alterar orden de cierre y omitir apagado global.
- Un array no vacío cuya asignación devuelve cero produce `AllocationFailed`
  después de escribir `CC=0`, antes de leer títulos. Cabecera o palabra ausente
  produce `MissingWord`; el adaptador puede devolver `InvalidToken`. Tienen prueba
  nativa del estado parcial; no se atribuye paridad a la memoria inválida del x86.
- El montón de prueba conserva bytes y registra liberaciones: no acredita el
  asignador real, destructores CString, reclamación de memoria ni hilos reales.
  Los helpers de construcción/destrucción de arrays C++ son una frontera de
  servicios, igual que en el bloque de bancos. No hay reentrada ni mutación del
  gestor desde el backend. Entradas de texto ASCII y asignaciones conocidas
  delimitan el contraste sintético; la salida sonora sigue pendiente.

### Configuración inglesa real, conservada solo en local

Se reutiliza el INI verificado del fixture local de bancos, sin volver a recorrer
el archivo comercial. Constructor → configuración → destructor: **3 llamadas**
originales y su secuencia Rust; **12 títulos, modo 3**, imágenes y servicios
coinciden. INI/textos se suministran al puerto de recursos; no es una nueva prueba
del lector INI ni de Archiver. El [informe compacto](evidence/lifecycle-real.json)
publica recuentos y hashes. El fixture con nombres reales queda en `artifacts/`.

```powershell
python reimpl/oracle/x86_audio_lifecycle.py --binary D:/AI/Patrician3/games/iso-english/app/Patrician3.exe --check
python reimpl/oracle/x86_audio_lifecycle.py --binary D:/AI/Patrician3/games/iso-english/app/Patrician3.exe --real-bank-fixture artifacts/audio-banks-real.json --real-fixture artifacts/audio-lifecycle-real.json --report reimpl/p3-audio/evidence/lifecycle-real.json --check
$env:P3_AUDIO_LIFECYCLE_FIXTURE=(Resolve-Path artifacts/audio-lifecycle-real.json).Path
cargo test --locked --manifest-path reimpl/Cargo.toml -p p3-audio --test x86_audio_lifecycle real_building_config_matches_original_x86 -- --ignored
```

La prueba comercial se ejecutó expresamente y queda ignorada por defecto. Para
regenerar el fixture local, quitar `--check` del segundo comando. No se generan
datos comerciales fuera de `artifacts/`. Los corpus anteriores siguen vigentes.
**Sin audio audible, DLL cargadas, hilos, esperas reales ni validación en partida.**

La apertura y el servicio se entregan en el bloque siguiente.

## Bloque 9: apertura y servicio cooperativo de música

`driver.rs` porta las dos entradas del catálogo. Además reproduce como
`MusicService::begin/poll` la rutina `0x00442090` (146 bytes), que ya constaba en
la revisión estática y es la dependencia directa del arranque del servicio.
**Esa rutina no está en el catálogo de 5.507 funciones y no se añade como fila
ni se suma a las 38.** No se modifica el catálogo compartido para contarla.

`DriverBackend` representa Miles, el registro de apagado, el aviso y la creación
de `SoundEffectList`. `ServiceBackend` agenda trabajo y entrega tiempos. Son
peticiones al cliente, sin carga de DLL, hilos ni dispositivo nativo en la crate.
El adaptador debe volver sin reentrar ni modificar el gestor o sus globales.

- La apertura solicita `./win32`, registra apagado y arranca la biblioteca. Si
  startup falla, solicita el aviso y **continúa**. Restablece preferencia 15 a
  cero y, con tasa menor que 11.025, borra solo el driver sin intentar abrir.
- Conserva divisiones i32, productos con vuelta y truncados u16 al construir
  los primeros 16 bytes de WAVEFORMATEX; los dos bytes de `cbSize` permanecen.
  Respeta el parámetro de salida del driver incluso cuando la apertura falla.
- Un fallo con preferencia cero la activa y reintenta con la misma tasa. Con
  preferencia activa reduce la tasa a la mitad; al quedar por debajo de 11.025,
  si usaba 16 bits cambia a 8 y vuelve a 44.100. Los demás formatos terminan con
  driver cero. El formato global conserva el último intento, aunque la tasa de
  configuración haya vuelto a reducirse.
- Una apertura exitosa cuya descripción contiene `Emulated`, distinguiendo
  mayúsculas, y preferencia cero cierra el driver y escribe la preferencia activa
  **dos veces** antes del reintento. Una preferencia ya activa permite aceptarla.
- Al aceptar reserva 20 muestras, también si alguna devuelve cero. Publica el
  driver global y crea `SoundEffectList` solo cuando falta. Fallar su reserva no
  evita el paso de filtros. Sus constructores C++ permanecen fuera de este bloque.
- Enumera el filtro **una sola vez** con cursor cero y no usa su código de
  retorno. Compara el nombre exacto `LowPass Filter`; si difiere borra proveedor,
  si coincide solicita abrirlo sin comprobar su retorno. No busca otro candidato.
- `start_service` requiere driver, solicita un trabajo aun si ya había otro,
  guarda su token y lo borra si falta música. **No cancela el trabajo agendado**
  por haber borrado la marca. La petición se conserva aun con música nula.
- El worker usa `5000 / music_volume` i32 y tiempo de presentación con resta u32.
  Cada `poll` ejecuta una vuelta, conserva el volumen leído en el estado del
  worker y omite setters mientras no cambie el paso. Compara el cociente como
  i32; conserva valores negativos tras vuelta del reloj y envíos sin saturar.
- Al terminar pausa/cierra música incluso con handle cero y borra música y
  `+0x120`, conservando las otras pistas y banderas. No necesita emitir un setter
  final de volumen cero. Con volumen inicial cero/negativo todavía lee reloj y
  cierra; con volumen positivo mayor que 5000, el x86 llega a DIV por cero después
  del setter y otra lectura de reloj. Rust devuelve `ZeroInterval` en ese punto.

### Evidencia y límites del bloque 9

- **323 secuencias / 1.043 llamadas originales**, contando las invocaciones
  directas del worker como llamadas a la dependencia. **40/40 aristas**.
  Coinciden los 484 bytes, configuración, formato, globales, estado del backend,
  errores DIV y servicios en orden. Cada lectura del reloj guarda además la traza
  y estado de canales; los puntos cooperativos reproducen esos mismos prefijos.
- `tests/fixtures/audio_driver.json`, SHA-256
  `d32a5ed84621a476d88fe17252f72826bddf8a8bab36b82179cf696dba55bb03`.
  Guarda hashes del EXE, dos entradas y dependencia, y tres scripts. Datos
  sintéticos y nombres ASCII válidos, sin recursos ni código comercial.
- Nueve mutaciones detectadas y restauradas: abortar por startup, alineación
  incorrecta, excluir tasa 11.025, cambiar fallback, omitir preferencia duplicada,
  comparar filtro sin mayúsculas, cancelar sin música, comparar el paso sin signo
  y alterar el intervalo. Los fixtures anteriores siguen vigentes.
- `max_attempts` limita un proveedor que no aplica la preferencia y no termina;
  devuelve `AttemptLimit` con el estado parcial, sin inventar un retorno x86.
  Nombre de filtro no escrito produce `MissingFilterName` después de reservar
  muestras y publicar proveedor. Ambos límites tienen prueba nativa separada.
- Reloj inmóvil devuelve `Pending`, sin girar ni repetir el setter. Un reloj no
  suministrado devuelve `ClockUnavailable`; no cierra la pista ni da el trabajo
  por terminado. Un trabajo terminado tolera otro poll sin nuevos servicios.
- El corpus invoca la rutina directamente incluso para estudiar el caso sin
  música: no acredita el despacho real de trabajos ni la concurrencia de MFC.
  No demuestra sincronización entre sliders, hilo y fotogramas, procesamiento
  digital, latencia, hardware, `SoundEffectList` ni el funcionamiento de Miles.

```powershell
python reimpl/oracle/x86_audio_driver.py --binary D:/AI/Patrician3/games/iso-english/app/Patrician3.exe --check
cargo test --locked --manifest-path reimpl/Cargo.toml -p p3-audio
```

El catálogo regional se entrega en el bloque siguiente. **Sin sonido ni validación en partida.**

## Bloque 10: catálogo regional desde recursos y conexión al selector

`region/catalog.rs` implementa `ScreenAudio::load_regional_catalog`. Devuelve
`RegionalBank`, dueño de los nombres y ganancias; `references().catalog(town)`
presta esos datos a los tres selectores ya portados. La música propia de cada
ciudad procede de la tabla del mundo y sigue siendo un argumento del cliente.
Los recuentos quedan en el estado de la misma instancia `ScreenAudio`.

El alcance original es **`0x004274FC..0x00427902` dentro de `0x00425EEE`**, desde
la primera llamada INI con sus argumentos preparados hasta la última escritura
de ganancia. El arnés prepara ESI, EBP y pila como en ese punto del constructor.
Excluye los menús, las dos escrituras UI anteriores y la limpieza posterior.
**No se añade una fila a `functions.tsv`: siguen siendo 38 funciones completas.**

- Consulta `./scripts/sound.ini` en el orden original. El total regional conserva
  el entero completo; los siete recuentos `Region0..6` se truncan a byte, incluido
  Region6 aunque el selector no lo lea. No exige que su suma coincida con el total.
- Lee nombre y ganancia por pista, intercalados; archivo ausente se vuelve vacío,
  capacidad de texto 4096 y ganancia por defecto 100, truncada a u8 sin saturar.
- Carga **tres** ambientes urbanos, **dos** marinos y **uno** de batalla, sin
  consultar sus claves `Titel` de recuento. El INI inglés declara dos ambientes
  urbanos pero contiene los tres nombres; el código carga el tercero igualmente.
- El recuento de pistas de mar se convierte a u8 **antes** de reservar e iterar.
  Son bancos distintos: `Region.Titel=256` carga 256, `SeaMap.Titel=256` carga cero.
- El tramo conserva temporizadores, índice de música anterior y rellenos. Diez
  punteros CString/tablas se sustituyen por almacenamiento Rust y se normalizan
  a cero al comparar el tramo de 80 bytes; nombres y ganancias se contrastan aparte.
  No se reproduce la disposición, refcounts o fugas de asignaciones C++ al recargar.

### Evidencia y límites del bloque 10

- **146 cargas del tramo y 48 llamadas posteriores a selectores**. Coinciden
  datos, recuentos, imagen normalizada y consultas INI en orden. Carga → selección
  comprueba nombres y volúmenes enviados por los catálogos prestados; el soporte
  de pruebas se comparte con el bloque 7, cuyo corpus permanece sin cambios.
- **18/18 aristas condicionales del tramo**. No significa cobertura del constructor
  entero ni de los helpers C++. El dominio usa asignaciones no vacías correctas;
  incluye arrays vacíos cuyo asignador devuelve cero, porque no se desreferencian.
- `tests/fixtures/audio_catalog.json`, SHA-256
  `4f31f1b52fa4d5fae9ca77b3497d7a3321c0d10950a90fb8770bb89804de1717`.
  Guarda límites y hash del tramo, máscara de punteros y siete scripts de procedencia.
- Nueve mutaciones detectadas y restauradas: total regional u8, saturar el recuento
  marino, ganancia por defecto cero, omitir Region6, saturar ganancias, omitir el
  tercer ambiente urbano, reducir el buffer, cruzar volúmenes marinos y conectar
  al selector los bancos intercambiados. La última falla en la conexión, no en
  la carga aislada.
- Total regional negativo o por encima de `region_limit` devuelve `NegativeCount`
  o `TitleLimit` después de leer/escribir los siete recuentos. No publica tablas
  ni sigue leyendo nombres. Ambos estados parciales tienen prueba nativa aparte;
  no se presenta paridad con reservas malformadas o acceso inválido del original.
  Fallar una reserva Rust produce `Capacity`. El texto del dominio es ASCII.
- Una configuración con recuentos incoherentes se conserva. Su consumo sigue
  usando `MissingTrack` del selector si pide una posición sin datos, sin leer
  memoria fuera del slice ni atribuir un valor al original fuera del array.
- INI, asignador y helpers CString del x86 son servicios controlados. Se usa el
  adaptador `IndexedResources` nativo; esto no acredita toda la clase INI original.
  Las consultas de pantalla, azar, mes, duración y reproducción están guionizadas.

### INI inglés real

Se reutiliza el INI del fixture verificado de bancos: **23 pistas regionales**,
recuentos **3/4/7/3/3/3/0**, **18 pistas de mar** y **seis ambientes**, en total
**47 entradas con 47 nombres distintos**. Una carga más cinco llamadas de
selección coinciden con el tramo/funciones originales. No se midieron duraciones
ni se comprobaron/reprodujeron los 47 archivos musicales; son nombres del INI.
El [informe compacto](evidence/catalog-real.json) conserva recuentos y hashes;
INI y fixture comercial permanecen bajo `artifacts/`. No se reextrajo el archivo.

```powershell
python reimpl/oracle/x86_audio_catalog.py --binary D:/AI/Patrician3/games/iso-english/app/Patrician3.exe --check
python reimpl/oracle/x86_audio_catalog.py --binary D:/AI/Patrician3/games/iso-english/app/Patrician3.exe --real-bank-fixture artifacts/audio-banks-real.json --real-fixture artifacts/audio-catalog-real.json --report reimpl/p3-audio/evidence/catalog-real.json --check
$env:P3_AUDIO_CATALOG_FIXTURE=(Resolve-Path artifacts/audio-catalog-real.json).Path
cargo test --locked --manifest-path reimpl/Cargo.toml -p p3-audio --test x86_audio_catalog real_regional_catalog_matches_original_x86_slice -- --ignored
```

La prueba comercial se ejecutó expresamente y queda ignorada por defecto; quitar
`--check` del segundo comando permite regenerarla en local. **Sin ventana, juego,
DLL del juego, salida sonora ni validación en partida.**

Las diez entradas previstas se entregan en el bloque siguiente.

## Bloque 11: SoundEffectList

26/10/2026, PC de Sergio. `effects::SoundEffectList` conserva la imagen de
132 bytes, cuatro canales, veinte nombres/búferes y una cola FIFO con umbral
diez. Los tokens de servicios son opacos; el objeto usa una `VecDeque<i32>`
para el orden lógico, sin reproducir nodos ni pools de CList.

| EXE inglés | Método de `SoundEffectList` |
|---|---|
| `0x004C4130` | `initialize_default` |
| `0x004C4240` | `initialize_named` |
| `0x004C4340` | `destroy` |
| `0x004C4460` | `release_all` |
| `0x004C4540` | `set_volume_percent` |
| `0x004C45A0` | `play` |
| `0x004C4620` | `load_effect` |
| `0x004C4710` | `effect_count` |
| `0x004C4750` | `class_name` |
| `0x004C4780` | `deleting_destroy` |

- El constructor por defecto borra los 24 slots antes de liberar; el constructor
  con nombre **no** los borra. Este último requiere tokens previos conocidos.
  Ambos cargan los IDs 0..9 incluso sin driver; con driver inicializan los
  cuatro resultados del asignador, incluso un handle cero.
- La carga no refresca aciertos. Expulsa el primero antes de comprobar si el
  nombre solicitado está vacío. Un fallo completo también encola el ID; puede
  haber duplicados. La expulsión pide liberar incluso un token cero, salvo si
  el ID saliente es el solicitado o está fuera de 0..19.
- Se libera el descriptor Archiver incluso si vale cero. Un `arc_Read` corto
  conserva el búfer; solo el búfer nulo activa la lectura alternativa. No se
  atribuye validez WAV a esos datos ni se decodifica audio en este bloque.
- `play` elige el primer canal no nulo con estado 2; si su carga/asignación falla,
  no prueba el siguiente. No exige driver. Solo arranca si `set_sample_file`
  devuelve un valor distinto de cero; `Ok(true)` expresa esa petición, no un
  retorno del original, que no está definido para esta entrada.
- Volumen: `wrapping_i32(percent * 127) / 100`, sin saturación. El segundo
  argumento se ignora. El destructor escalar solo borra el objeto si flags&1;
  el cliente es dueño del objeto Rust y debe descartarlo tras esa petición.
- **Posible defecto conservado:** `release_all` recorre solo los diez primeros
  búferes aunque la tabla y la carga admiten veinte. Los slots 10..19 quedan
  intactos al cerrar; el backend de prueba conserva esas asignaciones vivas.
  No se ha reparado silenciosamente. También se conservan dos consultas de
  estado: si pasan de 2 a 1, ese canal no se libera.

### Evidencia y límites del bloque 11

`tests/fixtures/audio_effects.json` contiene **185 secuencias y 6.966 llamadas**
al x86 original: las diez entradas, secuencias de carga/reproducción/cierre,
fallos de archivo/reserva/asignación de muestra, lecturas cortas, índices de carga
inválidos, nombres vacíos, cola con ID repetido o inválido y transición 2→1.
Coinciden orden FIFO, servicios en orden, estado del backend, retornos definidos,
texto de clase y los 132 bytes normalizados. Solo se enmascaran cuatro punteros
CList (`+0x68/+0x6C/+0x74/+0x78`); count, tablas virtuales y demás bytes se cotejan.

Se recorren **81/84 aristas condicionales**. Las tres restantes son guardas
redundantes: los dos constructores comprueban índice>=20 dentro de un bucle que
solo visita 0..9, y `play` comprueba canal>=4 tras encontrarlo entre 0..3.
No es cobertura de Miles, Archiver, CList ni de sus posibles excepciones.
El corpus fija el hash del ejecutable, rangos y hashes de las diez entradas y
tres fuentes del oráculo. SHA-256 del fixture:
`1b6ec3eb6ebbb0664ea177928da7ce921b38e60bd86de95bdab364eb38000237`.
La tabla nativa de veinte nombres se contrasta con un hash de la tabla real del
EXE; el resto de entradas/resultados usa nombres sintéticos, sin assets sonoros.

Nueve mutaciones detectadas por fallo de comparación y restauradas: liberar
veinte, alterar la condición de estado, expulsar a once, omitir fallos en FIFO,
saturar volumen, seleccionar canal ocupado, omitir init de handle cero, borrar
con cualquier flag y sustituir una lectura corta por fallback.

Una prueba nativa adicional conecta los puertos previos `DriverBackend`,
`SampleBackend` y `LifecycleBackend` con la clase: apertura de 20+4 canales,
precarga, propagación del volumen con doble truncado (64→50→63), reproducción
del ID 17, expulsión y cierre. Comprueba también el global que queda sin borrar
y el búfer alto que no se libera. Esa conexión no se presenta como un nuevo
corpus x86 ni como un driver de salida.

Límites seguros probados aparte: índice de reproducción fuera de 0..19 después
de encontrar canal libre devuelve `InvalidPlayIndex`; no se atribuye un valor a
la lectura fuera de tabla del original. Una cola cuyo count no corresponde con
su lista lógica devuelve `InconsistentQueue` antes de acceder a un nodo ausente.
Conserva los efectos previos al punto de detección. El nombre de clase es texto
Rust; no se reproduce el ABI/refcount de CString. El arnés CList es lógico,
y archivos, asignación, Miles y estado de canales siguen guionizados.

```powershell
python reimpl/oracle/x86_audio_effects.py --binary D:/AI/Patrician3/games/iso-english/app/Patrician3.exe --check
cargo test --offline --locked --manifest-path reimpl/Cargo.toml -p p3-audio --test x86_audio_effects
```

La primera parte del backend nativo se entrega en el bloque siguiente. J04 sigue
abierta por el codec, la salida sonora y la integración C09/J03.

## Bloque 12: archivos y búferes nativos

26/10/2026, PC de Sergio. `native::files` lee datos reales y conserva su propiedad
en Rust; `native::wave` explora el contenedor WAV; `native::NativeEffects<D>`
conecta ambos con el `SoundEffectList` portado. No añade dependencias de ejecución,
filas a `functions.tsv` ni nuevas funciones del juego al recuento: **48 de J04**.
Es un adaptador propio, no otra comparación x86 ni un reemplazo entero de Miles
o Archiver. No cambia los once corpus anteriores ni el control ya contrastado.

### Fuentes y búsqueda

- `ResourceFiles` admite bytes en memoria, registro explícito de archivos sueltos
  y montajes CPR V0.9 de solo lectura. No explora ni modifica la instalación.
  En CPR lee el índice y luego solo los rangos de los archivos solicitados.
- Soporta entradas CPR **tipo 1**: offsets absolutos y tamaños. Los dos tamaños
  de índice del bloque incluyen su cabecera de 16 bytes; confirmado en los
  **86 bloques** ingleses. Un montaje se publica solo después de validar el índice
  completo; el último nombre repetido sustituye al anterior. No implementa BAUM,
  compresión de archivo, escritura ni enumeración por cubos de la DLL.
- Plegado por nombres acreditado en [Archiver](../../research/systems/borde-motor/dll-boundaries/archiver-container/README.md):
  A–Z y Ä/Ö/Ü, `/` y `\`. El adaptador acepta rutas relativas Latin-1, normaliza
  segmentos `.`/vacíos y rechaza ascensos, nombres absolutos, NUL y `:`. Esta
  validación es una política nativa, no una afirmación sobre rutas malformadas
  que aceptaría el original. Los nombres públicos Rust son UTF-8.
- Búsqueda configurable: suelto, contenedor, suelto primero o contenedor primero.
  Si falla la lectura preferida se prueba la alternativa. El adaptador de efectos
  usa por defecto suelto primero, como el bit0 del contenedor original. Token
  de contenedor cero elige solo suelto; los demás deben corresponder al token
  configurado. `file_read` solo admite su segundo argumento cero.
- Los archivos montados deben mantenerse inmutables durante su uso. Una lectura
  abre su propio descriptor y lee el rango completo; un truncamiento se comunica
  como error, sin publicar bytes parciales. No reproduce mapeos ni carreras de
  la DLL. Límite por archivo: 16 MiB por defecto; índice CPR usado máximo 16 MiB
  y 100.000 entradas por montaje, con aritmética y límites comprobados.

### Tokens, propiedad y WAV

`ResourceArena` distingue descriptores con cursor y búferes. Sus tokens son
monótonos, no se reutilizan, y cero representa ausencia. Cerrar/liberar cero es
un no-op; un token desconocido, ya liberado o de otra clase devuelve error sin
modificar otro recurso. La lectura avanza el cursor del archivo y escribe al
principio del búfer, comprueba capacidad y conserva la cantidad de bytes válidos.
Un búfer compartido no puede sobrescribirse mientras existe un préstamo.

Los límites por defecto son 16 MiB por búfer, 64 MiB de búferes propios y 4.096
recursos vivos en el arena. Son límites de admisión/propiedad, **no un techo de
memoria total del proceso**: fuentes y préstamos de canales pueden conservar
bytes después de retirar un token. Las reservas usan `try_reserve_exact` cuando
corresponde; no se promete recuperación de cualquier agotamiento del asignador.

`WaveLease` retiene bytes inmutables por `Arc`, con metadatos y rango de datos
protegidos. El dispositivo recibe esta lease y puede conservarla mientras use
la muestra. La expulsión de una caché no invalida los datos ya entregados a un
canal. No se altera para ello la política original: el cierre de SoundEffectList
sigue dejando los búferes 10..19 vivos en el arena. Su propietario puede verlos
en `stats`; no se reparan silenciosamente esas omisiones del control.

El explorador WAV sigue la [estructura RIFF de Microsoft](https://learn.microsoft.com/en-us/windows/win32/xaudio2/resource-interchange-file-format--riff-):
recorre chunks con tamaños acotados y relleno a word, conserva `fmt `, datos
extra y `fact`, y omite chunks desconocidos. Admite orden variable de chunks.
Exige tamaño RIFF exacto, un único `fmt `/`data` y como máximo un `fact`; rechaza
extensiones truncadas, formatos sin canal/frecuencia/alineación y límites que
salgan del archivo. Son criterios nativos explícitos; no se acredita que Miles
rechace exactamente los mismos archivos. La validación usa solo los bytes
realmente leídos, sin completar una lectura corta con ceros de la reserva.

`EffectDevice` sigue siendo el puerto del codec/dispositivo. El adaptador entrega
errores de archivos/búferes mediante resultados cero esperados por el control y
los conserva en `take_errors`; no arranca una muestra con un WAV estructuralmente
inválido. No calcula duraciones, decodifica, mezcla, reproduce ni avanza canales.
El argumento `block` se transmite al dispositivo sin reinterpretarlo.

### Pruebas y recursos ingleses reales

Seis pruebas nativas cubren precedencia, plegado, CPR de varios bloques y archivos,
montaje atómico tras errores, prefijos truncados, rangos inválidos, cursores,
tokens de clase errónea/liberados, cuotas, propiedad compartida, WAV con chunks
impares/desconocidos, datos extra y archivos malformados. La prueba encadenada
usa CPR y WAV sintéticos reales: precarga diez → petición de reproducción →
expulsión mientras el primer canal conserva su lease → cierre. Quedan los cuatro
búferes altos esperados; no quedan descriptores. El dispositivo de prueba solo
registra peticiones y retiene leases, sin salida sonora ni paridad Miles.

Ocho mutaciones detectadas por fallos de pruebas y restauradas: prioridad de
fuentes, offset absoluto ignorado, primer archivo repetido gana, token reutilizado,
cursor inmóvil, guarda de capacidad omitida, ceros tratados como lectura válida
y relleno RIFF ignorado.

La prueba comercial optativa compara contra un inventario Python independiente
que reutiliza el lector de índice auditado: **6.485 entradas/nombres**, **178 WAV**
y **14.244.386 bytes de WAV**. Se cotejan hashes de archivo y de datos, formato,
extensión, `fact` y rangos. Los 178 usan formato **0x11 (IMA ADPCM)**, 44.100 Hz,
4 bits: 175 mono con bloque 1.024 y tres estéreo con bloque 2.048. El recuento de
163 del bloque 3 sigue siendo el subconjunto de nombres distintos cargados por
sus listas, no el censo completo de WAV. **Leer esos 178 no significa decodificarlos.**

El [informe compacto](evidence/native-files-real.json) solo versiona cifras y
hashes, con procedencia del CPR y scripts. Manifiesto detallado local
`artifacts/native-audio-real.json`, SHA-256
`39c8c82834420d6e816c008e045899f5824a9966e1defe75db0f2df9e19313cc`.
La prueba comercial se ejecutó expresamente; queda ignorada por defecto.

```powershell
python reimpl/oracle/native_audio_resources.py --archive D:/AI/Patrician3/games/iso-english/app/p2arch0_eng.cpr --fixture artifacts/native-audio-real.json --report reimpl/p3-audio/evidence/native-files-real.json --check
cargo test --offline --locked --manifest-path reimpl/Cargo.toml -p p3-audio --test native_resources
$env:P3_AUDIO_NATIVE_FIXTURE=(Resolve-Path artifacts/native-audio-real.json).Path
$env:P3_AUDIO_NATIVE_CPR='D:/AI/Patrician3/games/iso-english/app/p2arch0_eng.cpr'
cargo test --offline --locked --manifest-path reimpl/Cargo.toml -p p3-audio --test native_resources real_cpr_wave_payloads_match_independent_python_inventory -- --ignored
```

El decodificador previsto se entrega en el bloque siguiente; después siguen
mezcla/salida y las conexiones de cliente. J04 permanece abierta.

## Bloque 13: IMA ADPCM a PCM16 y contraste con Miles

26/10/2026, PC de Sergio. `native::ima` decodifica IMA WAV **0x11, cuatro bits,
mono o estéreo**, a PCM16 intercalado. Bloques completos, primer predictor,
reinicio de estado por bloque/canal, saturación i16, índice 0..88, nibble bajo
primero y grupos estéreo de cuatro bytes por canal. `max_frames` limita la salida
y la reserva es comprobada. No añade dependencias ni filas de funciones del
juego: **J04 conserva 48**. Estas funciones de biblioteca no forman parte del
universo de 5.507 funciones del EXE.

| Opciones de `decode` | Comportamiento |
|---|---|
| `ImaRounding::Product` | Producto antes de desplazar; coincide con el FFmpeg 7.1 ensayado. |
| `ImaRounding::ShiftSum` | Suma de términos desplazados; coincide con libsndfile 1.2.2 y el PCM definido de Miles. |
| `FactPolicy::Available` | Todos los fotogramas contenidos en los bloques. |
| `FactPolicy::Clamp` | Aplica fact hasta el máximo disponible, sin añadir muestras. |
| `FactPolicy::Strict` | Rechaza fact si excede el contenido. |

Los perfiles difieren en los **178 WAV reales**. El camino del backend es
`WaveLease::decode_miles_ima(max_frames)` → `decode_miles_defined`:
**ShiftSum + Clamp**, error `NoFrames` si la señal está vacía. Conserva recuentos
declarado/disponible. La conexión recurso → lease → decoder pasa incluso después
de liberar el token y destruir la fuente. El lector genérico permite señal vacía;
el camino Miles la rechaza, como la exportación ensayada.

Formato distinto, bloques parciales, SamplesPerBlock incoherente, índice de
cabecera fuera de rango, byte reservado no nulo y límite excedido devuelven
errores explícitos. Comprueba cabeceras también fuera del recorte fact.
Son límites nativos; no se acredita paridad para archivos malformados.

### Referencias y evidencia

- **FFmpeg 7.1**, PCM16 little endian por pipe, sin dispositivo.
  [Código de referencia](https://github.com/FFmpeg/FFmpeg/blob/n7.1/libavcodec/adpcm.c).
- **libsndfile 1.2.2**, [sf_readf_short](https://libsndfile.github.io/libsndfile/api.html),
  sin conversión flotante. Herramienta externa, no DLL del juego.
- **mss32.dll en Unicorn**: `_AIL_WAV_info@8` (`0x2110C8A0`) y
  `_AIL_decompress_ADPCM@12` (`0x2110C7F0`), incluidos lector y kernels
  mono/estéreo originales. Solo reserva/liberación interceptadas. Imagen PE
  leída como datos, sin cargar la DLL en Windows ni inicializar su runtime.
  Ruta con diagnóstico desactivado según la imagen inicial; no acredita
  inicialización, fallos del asignador, concurrencia ni otros exports.

DLL SHA-256 `f09e6024ad35eb6b7b46982efb998df26d162391b26fcf56519a4c23fcbc7015`.
El corpus fija además cuatro ventanas y las fuentes del arnés. Herramientas
externas con versión/hash, obtenidas de wheels oficiales
[imageio-ffmpeg 0.6.0](https://pypi.org/project/imageio-ffmpeg/0.6.0/) y
[soundfile 0.13.1](https://pypi.org/project/soundfile/0.13.1/), hash PyPI comprobado.
Binarios/licencias en `tools/audio-reference/`, ignorado; no se copia su código
al crate ni se redistribuyen. Los tests Rust normales solo consumen fixtures.

**130 WAV sintéticos**, 172.080 fotogramas disponibles y 113.952 tras fact.
Dos casos cubren los 89 índices × 16 códigos, mono/estéreo, signo y saturación;
otros 128 varían bloques, predictores y fact. Cinco pruebas Rust cubren perfiles,
guardas, señal vacía, recorte, límites y propiedad. Nueve mutaciones detectadas
y restauradas: orden de nibbles, distribución estéreo, predictor sin signo,
omitir saturación, ignorar índice inicial, ignorar fact, elegir Product como
Miles, rellenar hasta fact y omitir límite. La DLL descomprime 114 y rechaza
los 16 con fact cero; el port reproduce su PCM definido y ese rechazo.

**178 WAV de p2arch0_eng.cpr**, 175 mono y tres estéreo, 44.100 Hz/4 bits,
bloques completos: ambos perfiles coinciden con sus referencias externas y
ShiftSum con el PCM definido de Miles aislado. Total 26.455.442 fotogramas
disponibles, **26.427.830 definidos** tras fact. 137 no tienen fact, cinco
coinciden, 34 recortan y dos exceden el contenido. Dos pruebas comerciales
optativas ejecutadas, ignoradas por defecto. Solo cifras/hashes versionados en
[ima-real.json](evidence/ima-real.json) y [mss-ima-real.json](evidence/mss-ima-real.json).

**Finales sin definir:** `bell1.wav` declara 93.889 frente a 93.886 fotogramas;
`bell2.wav`, 112.258 frente a 112.255. La exportación reserva según fact y deja
**seis bytes sin escribir en cada archivo**. Reserva marcada A5 y vigilancia
de escrituras, incluidas las anchas que cruzan el límite, lo comprueban. El port
conserva el recuento declarado y devuelve solo el PCM definido, sin inventar
esos tres fotogramas. No se atribuye un defecto audible al juego: se ensaya
esta exportación, no la ruta de reproducción en partida.

Replay de Miles: **308 pares / 616 llamadas SDK**. No se suman a los corpus
anteriores del control. No se porta el ABI AILSOUNDINFO, la cabecera WAV de
salida, el asignador ni la semántica completa de AIL_set_sample_file, mezcla,
remuestreo o filtros. Sin sonido ni validación en partida.

### Reproducir

Fixtures sintéticos versionados: `ima.json`, SHA-256
`1871af4acc35c9401da6b72296aa4b1aaa294365d6bd5a7bb6cd2260692877c9`;
`mss_ima.json`, `89b682fadaad9e3c1b27f7188234d764ad6c7e482b7722bde18839342870b724`.
Las referencias externas eliminan fact solo para obtener toda la señal y
comparar después el prefijo Clamp; Miles recibe los WAV originales intactos.
Fixtures reales en `artifacts/ima-real.json` y `artifacts/mss-ima-real.json`;
sus hashes figuran en los informes compactos. No se versiona WAV/PCM comercial.

```powershell
$ff='D:/AI/Patrician3/tools/audio-reference/imageio_ffmpeg/binaries/ffmpeg-win-x86_64-v7.1.exe'
$sf='D:/AI/Patrician3/tools/audio-reference/soundfile/_soundfile_data/libsndfile_x64.dll'
python reimpl/oracle/ima_reference.py --ffmpeg $ff --sndfile $sf --check
python reimpl/oracle/x86_mss_ima.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --check
cargo test --offline --locked --manifest-path reimpl/Cargo.toml -p p3-audio --test ima

python reimpl/oracle/ima_reference.py --ffmpeg $ff --sndfile $sf --archive D:/AI/Patrician3/games/iso-english/app/p2arch0_eng.cpr --real-fixture artifacts/ima-real.json --report reimpl/p3-audio/evidence/ima-real.json --check
python reimpl/oracle/x86_mss_ima.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --archive D:/AI/Patrician3/games/iso-english/app/p2arch0_eng.cpr --reference artifacts/ima-real.json --real-fixture artifacts/mss-ima-real.json --report reimpl/p3-audio/evidence/mss-ima-real.json --check
$env:P3_IMA_FIXTURE=(Resolve-Path artifacts/ima-real.json).Path
$env:P3_MSS_IMA_FIXTURE=(Resolve-Path artifacts/mss-ima-real.json).Path
$env:P3_AUDIO_NATIVE_CPR='D:/AI/Patrician3/games/iso-english/app/p2arch0_eng.cpr'
cargo test --offline --locked --manifest-path reimpl/Cargo.toml -p p3-audio --test ima -- --ignored
```

Siguiente bloque fijo: **canales y mezcla PCM fuera de línea**, con EffectDevice
nativo, cursores, volumen/panorámica, fin y liberación comprobables sin dispositivo.
Después salida y conexión con otras familias de audio y cliente C09/J03.
J04 sigue abierta; no se elige motor gráfico ni se abre el juego.

## Bloque 14: voces nativas y mezcla PCM fuera de línea

26/10/2026, PC de Sergio. `native::mixer::OfflineDevice` implementa
`EffectDevice` y los controles de muestras de `Backend`. Conectado mediante
`NativeEffects<OfflineDevice>`: recurso → lease → IMA ShiftSum/Clamp → PCM16
→ mezcla. La prueba de SoundEffectList carga, reproduce, expulsa un recurso
mientras su PCM sigue vivo y cierra las cuatro voces. Conserva el defecto de
liberar solo los búferes 0..9: el búfer 10 permanece en la arena del llamante.
No modifica el núcleo, Cargo ni el registro: **J04 sigue en 48 funciones**.

### Contrato nativo y límites

- Salida estéreo PCM16 intercalada; entradas mono o estéreo de igual frecuencia,
  44.100 Hz por defecto. El mono se duplica antes de las ganancias por lado.
  Una frecuencia distinta se rechaza; no se remuestrea silenciosamente.
- Driver lógico 1, hasta 32 voces (límite para el acumulador i32), tokens
  monótonos sin reutilización. Inicializar y liberar descartan el PCM adjunto;
  terminar o llegar al final conserva sus datos hasta inicializar/liberar.
- `start` rebobina; `stop` conserva cursor; `resume` continúa si quedan datos;
  `end` pasa a Done. Estados 2/4/8. Solo `render` avanza los cursores; volumen
  cero también avanza. Consultas y pausas no consumen tiempo real.
- Render dividido o entero produce igual PCM y eventos. Las terminaciones
  naturales se notifican una sola vez, ordenadas por posición dentro del bloque
  y token. **Son eventos nativos**, no callbacks ni temporización del SDK.
- Límites comprobados antes de alterar las voces: fotogramas de render y clip,
  presupuesto lógico de muestras adjuntas, formato y handles. Un reemplazo
  rechazado conserva PCM, cursor y estado anteriores. Por defecto: 65.536
  fotogramas por render, 2.000.000 por clip y 32 Mi muestras i16 adjuntas.
- Los puertos heredados anotan errores recuperables; las APIs nativas devuelven
  `Result`. Streams no nulos dan diagnóstico explícito de operación no soportada.
  Borrar el objeto o las tablas externas solo encola la solicitud para su dueño.

### Contraste numérico con la biblioteca original

`x86_mss_mix.py` lee el PE de mss32.dll como datos y ejecuta en Unicorn:

| Rutina original | Dirección | Alcance ensayado |
|---|---|---|
| Cálculo interno de ganancias | `0x21113A90` | Volumen/pan acotados 0..127; master 0..127. |
| MIXER_flush | `0x2111EE1D` | Acumulador seco a cero. |
| MIXER_merge | `0x2111EE71` | PCM16 mono/estéreo, salida estéreo, ratio 65536, escala activada (bit 0x10), MMX desactivado. |
| MIXER_copy | `0x2111F00C` | Desplazamiento aritmético 11 bits y saturación PCM16 final. |

Ningún cálculo se sustituye. GDT plana de 32 bits para las instrucciones de
segmentos de los kernels; no DllMain, Windows ni dispositivo. DLL SHA-256
`f09e6024ad35eb6b7b46982efb998df26d162391b26fcf56519a4c23fcbc7015`;
ventanas de código y hashes de tres fuentes del arnés fijados en los fixtures.

El pan usa tabla `2*p` hasta 62 y 128 desde 63. Ganancia efectiva entera:
`master*volume/127`; por lado `tabla(p)*efectiva/8`, con valores ≥2032
sustituidos por 2047. Ambos lados alcanzan el máximo en el centro. Acumula
productos i32 de todas las voces, luego desplaza y satura una vez. No recorta
cada voz por separado. En la ruta con escala, 2047 no es 2048: por ejemplo
1000 da 999 y -1000 da -1000 a ese coeficiente.

**17.398 combinaciones de ganancias**: las 128×128 a master127 y 1.014
combinaciones de bordes, incluidos valores negativos y fuera de rango.
**256 mezclas sintéticas**, 1..32 voces, 2.674 llamadas merge y 9.589
fotogramas de salida: signos, silencio, recorte, mono/estéreo, bloques cortos,
igual longitud y más largos. Cada mezcla coteja hash de todo el PCM y cursores.

**16 mezclas reales**: fragmentos de los primeros dos WAV mono y dos estéreo
del archivo CPR, 64 llamadas merge y 4.336 fotogramas. La referencia obtiene
PCM con el decoder original; Rust vuelve a leer y decodificar los WAV, coteja
los fragmentos y compara las mezclas completas. Prueba comercial optativa
ejecutada. Solo hashes/cifras en [mix-real.json](evidence/mix-real.json);
los fragmentos PCM quedan en `artifacts/mix-real.json`, ignorado.

Cinco pruebas normales y una comercial. Nueve mutaciones provocaron fallos
de aserción y se restauraron: salto pan63, 2048 por 2047, división hacia cero
por desplazamiento aritmético, duplicar canal izquierdo, rebobinado incorrecto,
estado de pausa, avance pausado, presupuesto PCM y orden de terminaciones.

**Límite de paridad:** coincide con estos kernels escalares y su ruta con
escala. No acredita cómo el driver Miles escoge otras rutas (incluida la que
omite escala), MMX, filtros, bucles, remuestreo, streams, bombeo, callbacks ni
salida física. Los controles y la propiedad del mezclador se prueban como
contrato nativo; no se presentan como comparación del runtime Miles entero.
No se cargó la DLL original en Windows ni se abrió el juego.

### Reproducir

Fixture sintético `mixer.json`, SHA-256
`a08e56874466969464b51da7799cf07113d545bf54fb3519824cb4681315b657`.
Fixture comercial local, SHA-256
`29ba3c1ea4790d6304786fc38ac4775701176ea356ecc5613a91e26ead1fa63f`.

```powershell
python reimpl/oracle/x86_mss_mix.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --check
python reimpl/oracle/x86_mss_mix.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --archive D:/AI/Patrician3/games/iso-english/app/p2arch0_eng.cpr --real-fixture artifacts/mix-real.json --report reimpl/p3-audio/evidence/mix-real.json --check
cargo test --offline --locked --manifest-path reimpl/Cargo.toml -p p3-audio --test mixer
$env:P3_MIX_FIXTURE=(Resolve-Path artifacts/mix-real.json).Path
$env:P3_AUDIO_NATIVE_CPR='D:/AI/Patrician3/games/iso-english/app/p2arch0_eng.cpr'
cargo test --offline --locked --manifest-path reimpl/Cargo.toml -p p3-audio --test mixer real_ima_to_pcm_mixes_match_original_miles_kernels -- --ignored
```

Siguiente bloque fijo: conectar **SampleBackend/PlaybackBackend de SoundManager**
con recursos, decoder y mezclador nativos; contrastar la selección de ruta de
mezcla antes de atribuir paridad de reproducción. Esta conexión precede a la
salida física. Después streams, salida e integración C09/J03. J04 sigue abierta
y pendiente de revisión independiente; continúa la ruta audio → interfaz → gráficos.

## Bloque 15: muestras/caché de SoundManager y selección de mezcla

26/10/2026, PC de Sergio. `NativeEffects<OfflineDevice>` implementa ahora
`SampleBackend` y `PlaybackBackend`: el `play_sound` ya contrastado carga de
recursos montados, conserva los veinte búferes de caché, adjunta WAV IMA al
decoder y reproduce en voces nativas. Volumen global/individual, pan,
preparación, reserva y parada del gestor llegan al mismo dispositivo. Una
`effect_list` opcional comparte cuatro voces y recibe el volumen auxiliar real.
Sin nuevos crates ni filas de funciones del juego: **J04 sigue en 48**.

### Conexión y propiedad

`sample_manager(&catalog)` prepara un gestor nativo y reserva veinte voces.
El catálogo es completo, estable y suministrado por el cliente (por ejemplo,
`SoundBank::catalog`); no se inventan punteros a tablas CString del EXE. Las
tablas de IDs que el constructor original deja intactas se inicializan con
un ID ausente del catálogo, también si este contiene 0 o u32::MAX. Si falta
capacidad se liberan las voces recién reservadas y se conservan las anteriores.
Este bootstrap no acredita negociación de driver, INI ni el arranque Miles entero.

`release_sample_manager` libera las voces y los búferes aún referenciados por
ese gestor en **el mismo adaptador**. No sustituye el destructor original completo
ni destruye tablas del cliente. El backend conserva su propiedad hasta Drop;
los tokens no son punteros ni tienen significado en otro adaptador.

El adaptador mantiene una sesión Archiver y un archivo abiertos como máximo,
acepta el modo0x8000 del port y usa la búsqueda de recursos configurada.
Lectura corta o apertura fallida recorre la limpieza ya portada. Nombre distinto
de WAV, longitud suministrada distinta del prefijo leído o formato rechazado
dan diagnóstico. Consulta `take_errors()` y `device.take_errors()`; el algoritmo
original puede devolver slot aunque falle la configuración y llamar a start.
No se transforma ese comportamiento en éxito de decodificación.

Se conserva el orden del algoritmo, incluido volumen cero **después** de cargar
caché, promover aciertos y rechazar expulsar una muestra activa/reservada.
Ese último rechazo fuga el búfer recién leído en el original: la prueba observa
21 búferes para veinte entradas; no se corrige silenciosamente. La liberación
del gestor deja la asignación no referenciada hasta destruir el dueño nativo.
Un PCM ya adjunto sobrevive a expulsar/liberar el recurso que lo originó.

### Selector de ruta original

El bloque14 forzaba escala para contrastar sus kernels. Este bloque ejecuta
**`0x21111D90` entero** con el proveedor `MIXER_merge` original conectado en
sample+0x218. No se intercepta ni sustituye la decisión ni la mezcla. Solo se
observan los argumentos al entrar en merge. Configuración ensayada: salida
estéreo16, misma frecuencia, MMX0, flags auxiliares0, sin filtro ni reverberación.

En ese dominio, las operaciones son3/11 (mono/estéreo sin escala) cuando las dos
ganancias son2047; el resto usa19/27 (bit0x10). El kernel sin escala conserva
unidad exacta: 1000 devuelve1000, donde la escala por2047 daba999. El backend
elige **`MixMode::DryDispatch` por defecto**. `ScaledKernel` sigue disponible para
reproducir los fixtures anteriores intactos; sus tests lo seleccionan explícitamente.

**20.480 decisiones**: ocho volúmenes de borde ×128 pan ×cinco master ×formatos
1/3/5/7 (PCM16 e IMA, mono/estéreo). La matriz se detiene en0x21111E05, antes de
la tasa/decodificación; no acredita ese código posterior para entrada comprimida.
**512 mezclas completas** con entrada PCM16: 256 estímulos reutilizados del
bloque14 y256 con volumen máximo/pan62..65. 5.348 llamadas al dispatcher/merge:
1.376 sin escala y3.972 con escala; 19.178 fotogramas de salida. Coteja operaciones,
ganancias, PCM completo y consumo. El PCM IMA se contrasta por separado con el
decoder original, como en el bloque13; no se acredita el pipeline ADPCM interno
del driver entero al componer estas pruebas.

**16 mezclas reales**, cuatro WAV (dos mono/dos estéreo): 64 dispatch, mitad con
escala y mitad sin ella, 4.336 fotogramas. Referencia de fragmentos decodificados
por Miles; Rust vuelve a decodificar y cotejarlos antes de mezclar. Además pasa
la cadena CPR real → `play_sound` → caché → decoder → PCM del mezclador, con
comparación del PCM y cierre de sus recursos.

DLL SHA-256 `f09e6024ad35eb6b7b46982efb998df26d162391b26fcf56519a4c23fcbc7015`.
Cuatro fuentes del arnés y tres ventanas fijadas en los fixtures. Informes compactos:
[sintético](evidence/dispatch-synthetic.json) y [real](evidence/dispatch-real.json).
El fixture con PCM comercial solo vive en `artifacts/dispatch-real.json`, ignorado.
Los corpus anteriores no se regeneran ni se presentan como casos nuevos.

Ocho pruebas normales y una comercial optativa pasan. Diez mutaciones detectadas
por fallos de aserción y restauradas: forzar escala por defecto, umbral de unidad,
ganancia sin escala, lectura corta, cierre de archivo, rollback, ID vacío0,
getter de volumen, conexión decoder y volumen auxiliar.

**Capacidades pendientes:** `FilterBackend` diagnostica proveedor no implementado
y conserva `MissingFilterKey` después de iniciar, como ordena el port; sin filtro
habilitado, el flag de la muestra por sí solo no activa DSP. Servicios de streams
no nulos dan diagnóstico. No hay filtros nativos, remuestreo, loops, flags de
inversión, MMX, bombeo/callbacks SDK ni salida física. Los eventos y el reloj
siguen siendo contrato nativo. DLL leída/emulada; no se carga en Windows ni se
abre el juego. La integración no equivale a aceptación independiente de J04.

### Reproducir

Fixture sintético `dispatch.json`, SHA-256
`e45058a97fd504f0a45ffcf7ad96b76601b73e9e61b03ee9f54ccc0c464e2559`;
comercial local `2adda5b6579efc51ad29194131fa2ff484c5df0a5e3c0ae333355c0ac2ba8e0c`.

```powershell
python reimpl/oracle/x86_mss_dispatch.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --report reimpl/p3-audio/evidence/dispatch-synthetic.json --check
python reimpl/oracle/x86_mss_dispatch.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --archive D:/AI/Patrician3/games/iso-english/app/p2arch0_eng.cpr --real-fixture artifacts/dispatch-real.json --report reimpl/p3-audio/evidence/dispatch-real.json --check
cargo test --offline --locked --manifest-path reimpl/Cargo.toml -p p3-audio --test native_playback
$env:P3_DISPATCH_FIXTURE=(Resolve-Path artifacts/dispatch-real.json).Path
$env:P3_AUDIO_NATIVE_CPR='D:/AI/Patrician3/games/iso-english/app/p2arch0_eng.cpr'
cargo test --offline --locked --manifest-path reimpl/Cargo.toml -p p3-audio --test native_playback real_decode_and_dry_dispatch_match_original_miles -- --ignored
```

Siguiente bloque fijo: backend nativo de **streams de música/ambiente/voz**,
recursos/codec, duración/posición y reloj/pausa fuera de línea. Filtros lowpass
siguen pendientes antes de acreditar reproducción filtrada. Salida física e
integración C09/J03 después, conservando audio → interfaz → gráficos.

## Bloque16: streams nativos de música, ambiente y voz

26/10/2026, PC de Sergio. `StreamBackend` ya está conectado a
`NativeEffects<OfflineDevice>`. El mismo mezclador reúne muestras y streams
antes de saturar; `stream_sample` identifica la voz subyacente, sin punteros C++.
Música, ambiente, voz, edificio, volúmenes y fundidos del gestor usan recursos y
PCM reales. No añade funciones del EXE al registro: **J04 conserva48**.

### Contrato nativo

- MP3 con Symphonia0.6.1, WAV PCM8/16 y el IMA ShiftSum/Clamp ya contrastado.
  No se invoca FFmpeg en ejecución. [Versiones, autoría y licencias](THIRD_PARTY.md).
- **Predecodificación completa y acotada al abrir**, no lectura progresiva ni
  hilo. Fuente montada explícitamente, hasta16MiB codificados y26.460.000
  fotogramas por stream (600s a44.100Hz). Cambios de frecuencia/canales y formatos
  no soportados se rechazan; el mezclador exige la frecuencia configurada.
- Hasta ocho streams dentro de las32 voces compartidas; con20 del gestor y
  cuatro auxiliares quedan ocho. El presupuesto predeterminado de PCM adjunto
  pasa de32Mi a128Mi muestras i16 (256MiB), configurable. La decodificación
  temporal consume memoria adicional antes de comprobar ese presupuesto.
- Abierto sin iniciar: estado2. Solo render avanza; cero volumen también avanza.
  Pausa conserva cursor; reanudar conserva posición y pases restantes. El primer
  inicio respeta la búsqueda previa; iniciar de nuevo rebobina. Bucles0 infinitos,
  N>0 incluye el pase actual; el último genera una sola terminación natural.
- Posición y duración: `floor(frames*1000/rate)`. Buscar usa
  `floor(ms*rate/1000)`, limitado al final. Este reloj es contrato nativo,
  **no paridad de los redondeos, latencias ni búsqueda por paquetes de Miles**.
  El llamante proporciona el tiempo del gestor para actualizar los fundidos.
- `RenderedPcm.finished_streams` informa el instante dentro del bloque; `finished`
  también contiene sus voces subyacentes, dos vistas del mismo final. Dividir
  renders no cambia PCM ni eventos. Cerrar libera voz/PCM e invalida el token;
  liberar la muestra subyacente invalida también su stream. No se reutilizan IDs.
- Consultas, duración temporal y `service_stream` no avanzan el reloj. Al estar
  el PCM listo, este último valida el handle sin simular colas ni callbacks.
  Se aceptan los hints de búfer0/0x3E800; no son una reserva hardware.

Se conserva el comportamiento del gestor: la voz pausa el fondo y al terminar
lo reanuda; parar edificio deja la bandera de exclusión hasta la transición que
la limpia. Una segunda voz sobrescribe VOICE sin cerrar la primera: el stream
anterior continúa y queda sin referencia en el gestor, como el port original.
La prueba exige su cierre explícito al dueño nativo. No se oculta esa deuda.

### Evidencia separada

**MP3 sintéticos propios:** ocho casos, mono/estéreo,44.100/22.050Hz y longitudes
1/6.000 antes de codificar. FFmpeg7.1 con libmp3lame sin Xing/ID3 genera la
entrada y su referencia PCM. 39.168 fotogramas decodificados/58.752 muestras:
máximo1 LSB y21.709 errores cuadrados acumulados. Fixtures no comerciales.

**123 MP3 reales:**79.040.367 bytes, todos estéreo44.100Hz. Rust decodifica todos
y coteja el recuento total de214.660.224 fotogramas con la referencia. Se comparan
369 ventanas (inicio, centro, final),3.022.848 muestras: máximo1 LSB y1.269.841
errores cuadrados. **No hay identidad PCM completa ni comparación con el
decoder MP3 de Miles**. La prueba comercial también conecta recursos reales a
música → ambiente → voz → reanudación, mezcla y cierre sin recursos vivos.

Diez archivos terminan en una trama incompleta:3.808 bytes de final. FFmpeg
con el archivo intacto completa esa trama; Symphonia la descarta. El arnés
identifica cabeceras/tramas y da a la referencia solo el prefijo completo;
Rust recibe el original intacto. Se guardan hash, longitud y final separado.
Por ejemplo, `MenueMusic/hauptmenue.mp3` tiene7.720 tramas completas y388 bytes
de una trama que requiere417. No se inventa ese final, ni se acredita cómo
Miles lo reproduce en partida. Informes [referencia](evidence/mp3-real.json) y
[comparación nativa](evidence/mp3-native.json); PCM comercial solo en artifacts.

**Miles aislado, otro alcance:**120 casos/603 llamadas a rutinas internas:
status0x21121FE0, volumen0x211228E0/0x21122920 y bucles0x211229A0/0x21122A70.
HSTREAM preparado con muestra enlazada, error8C=0, started7C=1, fuente90=0,
controlAC=1; se cotejan estados2/4/8, saturación de volumen, contador y NULL.
No se intercepta el cálculo. Este corpus no cubre apertura, transporte, codec,
bombeo ni tiempo. Imagen DLL con hash y ventanas, fuentes del arnés fijadas;
no se carga en Windows. No sumar estas llamadas al corpus del gestor.

Diez pruebas normales nuevas y una comercial optativa. Diez mutaciones
detectadas y restauradas: bucles infinito/finito, pausa, búsqueda antes de
iniciar, reanudación, conversión ms, cierre/propiedad, offset entre pases,
alias HSTREAM/muestra y límite de frames del MP3. Los corpus previos siguen
intactos; solo cambian expectativas de errores de streams antes no soportados.

### Reproducir

```powershell
$ff='D:/AI/Patrician3/tools/audio-reference/imageio_ffmpeg/binaries/ffmpeg-win-x86_64-v7.1.exe'
python reimpl/oracle/native_audio_streams.py --ffmpeg $ff --report reimpl/p3-audio/evidence/mp3-synthetic.json --check
python reimpl/oracle/native_audio_streams.py --ffmpeg $ff --root D:/AI/Patrician3/games/iso-english/app/Musik --fixture artifacts/mp3-real.json --report reimpl/p3-audio/evidence/mp3-real.json --check
python reimpl/oracle/x86_mss_stream_queries.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --check
cargo test --offline --locked --manifest-path reimpl/Cargo.toml -p p3-audio --test native_streams
$env:P3_STREAM_FIXTURE=(Resolve-Path artifacts/mp3-real.json).Path
$env:P3_STREAM_ROOT='D:/AI/Patrician3/games/iso-english/app/Musik'
cargo test --release --offline --locked --manifest-path reimpl/Cargo.toml -p p3-audio --test native_streams real_mp3_inventory_pcm_windows_and_manager_roles -- --ignored --nocapture
```

Hashes en los informes compactos; fixture de consultas
`d1d4fdac4923369656b2a1b03c4f40af386092a0958f69435e29f97d2dac701c`.
Dependencia exacta y13 paquetes añadidos al lock, sin actualizar los existentes.

El DSP Lowpass nativo se entrega en el bloque17. Antes de salida física quedan
paridad MP3/tiempos con Miles y lectura progresiva, con pruebas propias; después
integración C09/J03. Sin remuestreo, MMX, driver/colas/callbacks originales ni QA
en partida. J04 sigue abierta y pendiente de revisión independiente.

## Bloque17 — DSP Lowpass nativo (26/10/2026, PC de Sergio)

`native/lowpass.rs` implementa el filtro de dos polos de `Lowpass.flt` y
`native/playback.rs` conecta la enumeración de atributos, el procesador de etapa1
y su preferencia con muestras y streams. El bootstrap asigna un proveedor lógico
nativo (`LPF1`), inicialmente desactivado. La única clave es0 (`Lowpass Cutoff`).
SoundManager conserva la transición original: ambiente a400Hz, efectos marcados
como filtrados, parada de muestras y música de edificio; desactivar retira el
procesador del ambiente. No requiere instalar ni cargar la DLL original.

El filtro recibe el acumulador Q11 después de volumen/panorámica y antes del
recorte final compartido. Cada canal conserva cuatro valores; coeficientes e
historia se almacenan como f32, con operaciones intermedias f64, orden de sumas
por canal y epsilon del original. La referencia usa x87 CW027F (precisión53,
redondeo al más cercano). El corte inicial100 usa la constante original;
la preferencia limita a20..(frecuencia/2−1), incluido NaN→20 e infinito positivo
al máximo. Cambiar el corte conserva el historial. Reasignar el mismo proveedor
destruye y recrea el filtro: reinicia historial y corte100. Parar/pausar conserva
estado; inicializar la muestra lo elimina. Los bucles continúan el historial y
el final del origen no genera una cola artificial.

### Referencia original y verificación

[Oráculo](../oracle/x86_lowpass.py), [informe](evidence/lowpass.json) y fixture
sintético propio fijan las fuentes y hashes completos de ambas imágenes:
`Lowpass.flt`9a7d7e66…0e830be1 y `mss32.dll`f09e6024…bc7015.
Unicorn ejecuta el registro RIB, constructores, preferencia y procesamiento
originales del filtro, junto con merge/copy originales de Miles. Se interceptan
asignación/liberación de memoria, configuración de frecuencia, registro y consulta
MMX. En la prueba de reasignación se ejecuta también `AIL_set_sample_processor`
interno0x21114EF0; solo se suministran sus consultas de proveedor/driver. Esto
comprueba su decisión de cerrar y abrir, sin acreditar el runtime RIB completo.

**168 casos,86.184 fotogramas:** siete frecuencias8.000..192.000Hz, origen mono
y estéreo, silencio, impulso, continua, alternancia extrema y ruido reproducible;
doce cortes iniciales, ganancias/panorama y cambios de corte y volumen. Son168
aperturas,322 preferencias,840 procesos y14 cambios de volumen. Coinciden el PCM
completo y los14 patrones de32bits de corte/historia/coeficientes tras cada paso,
sin tolerancia. Siete casos secundarios con precisión64 no difieren; eso no
prueba equivalencia universal entre perfiles. Fixture SHA256:
`a142f8c77946895ece5af22ad466e5beaa106ae3fb4cdea66978ad6c526c78ed`.

Cinco pruebas normales cubren el corpus, reinicio frente a cambio de preferencia,
conexión gestor→ambiente/efecto, errores de proveedor/etapa/clave y continuidad
al partir bloques, pausar y cruzar bucles. Diez mutaciones provocan fallos de
aserción y se restauran: epsilon, signo de realimentación, corte mínimo,
coeficiente inicial, historial entre canales, reasignación, borrado de historial
al cambiar corte, bypass, ganancias cruzadas y clave del adaptador.
Regresión del workspace:147 Rust OK y nueve comerciales ignoradas por defecto;
547 generales,14 omitidas sin fallos, panel3 OK. El test real A03 retorna sin
P3_NAVDATA; no se presenta como repetición de sus datos.

Alcance nativo: salida estéreo y ruta software escalar; se rechazan frecuencias
fuera de8.000..192.000Hz y proveedores, etapas o claves no soportados. Son límites
seguros del adaptador, no reconstrucción general del SDK. No demuestra salida
mono, MMX, remuestreo, dispositivo, colas, callback ni paridad en partida. Los
fixtures anteriores siguen intactos; este bloque de biblioteca no añade filas
al registro de funciones del juego (J04 sigue48).

```powershell
python reimpl/oracle/x86_lowpass.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --filter D:/AI/Patrician3/games/iso-english/app/Win32/Lowpass.flt --report reimpl/p3-audio/evidence/lowpass.json --check
cargo test --offline --locked --manifest-path reimpl/Cargo.toml -p p3-audio --test lowpass
cargo test --release --offline --locked --manifest-path reimpl/Cargo.toml -p p3-audio --test lowpass
```

La comparación directa MP3 y la aritmética temporal se entregan en el bloque18;
la paridad MP3 sigue abierta. Después de resolverla: lectura progresiva, salida
física e integración C09/J03. J04 sigue abierta y falta revisión ajena.

## Bloque18 — comparación MP3 con Miles y corrección del reloj PCM

**Rectificación en bloque19:** tres variantes sintéticas alcanzaban una lectura
fuera de la tabla pow43. El arnés no la detenía y el PCM posterior dependía del
vecino en el heap. La cifra44.174 de este apartado conserva valor histórico,
pero queda retirada como referencia de paridad actual. Usar los fixtures y
oráculo acotados del bloque19; los antiguos quedan congelados para trazabilidad.

26/10/2026, PC de Sergio. Se ejecuta ahora **Mp3dec.asi original en Unicorn**,
además de la referencia FFmpeg anterior. La comparación encuentra diferencias:
**Symphonia sigue siendo un decoder provisional; no tiene paridad exacta con
el perfil original ensayado**. Las pruebas nuevas conservan un inventario de
diferencias (`exact_parity:false`); que pasen solo demuestra reproducibilidad
de ese inventario. No aceptan el decoder ni convierten errores en tolerancias.

### Decoder original aislado

[Oráculo](../oracle/x86_mp3.py): hash del plugin
`9198b18bfd5b5b22cb392db32c73444c352ce1e9b9fe6604bce3c42dc78e7c43` y hash de
Miles ya fijado. Registro RIB, startup, apertura, proceso, seek y cierre ejecutan
los cuerpos originales. Solo se suministran memoria, registro, MMX=0 y callback
de lectura de bytes. Se comprueba propiedad de asignaciones y liberación final;
no se carga la DLL en Windows. Perfil escalar/CW027F, no dispositivo, HSTREAM
completo, ruta acelerada ni comportamiento audible del juego.

- **16 casos propios:** los ocho MP3 sintéticos previos, más ocho variantes
  quitando1/17bytes a los cuatro de señal. El original produce101.376 fotogramas
  entre todos los casos. La salida original coincide al partir las peticiones
  en tamaños1/7/513/4093 frente a4096bytes:368 procesos, más16 procesos tras seek0.
  En seis casos MPEG2 a22.050Hz, esos primeros512bytes tras seek0 difieren de
  abrir de nuevo; se conserva el hash y queda sin atribuir la causa.
- Frente a Rust se comparan141.696 muestras:42.609 distintas, error máximo44.174.
  Solo los cuatro casos de silencio son idénticos. Los ocho finales truncados
  producen una trama más en Miles:1.152 fotogramas a44.100Hz o576 a22.050Hz;
  Symphonia la descarta. Esto afecta a duración y fin, además del PCM.
- **123 MP3 reales:** solo los primeros4.096 fotogramas de cada archivo original,
  503.808 fotogramas/1.007.616 muestras. Hay394.364 muestras distintas y un error
  máximo2.363;122 de123 prefijos difieren. No se compara el PCM completo original,
  el centro/final, ni se resuelven los diez finales comerciales incompletos.
  Rust sí decodifica el archivo completo, pero solo se coteja ese prefijo común.

Informes [original sintético](evidence/miles-mp3.json),
[diferencias sintéticas](evidence/mp3-miles-native.json),
[original real](evidence/miles-mp3-real.json) y
[diferencias reales](evidence/mp3-miles-real-native.json). El PCM comercial queda
solo en artifacts. Las diferencias grandes del perfil emulado necesitan aislar
su primera divergencia; no se atribuyen a un defecto del juego ni se copian como
corrección del decoder Rust sin esa investigación.

### Aritmética temporal portada y aplicada donde está acreditada

`native/timing.rs` porta consulta0x21122BC0, conversión de seek0x21122CC0 y
alineación0x21122670. **759 casos/33 formatos**,7.590 conversiones de búsqueda y
3.098 alineaciones adicionales de frontera coinciden con los cuerpos originales.
El [oráculo temporal](../oracle/x86_stream_timing.py) prepara los buffers de la
muestra; ejecuta también stream_position/buffer_info originales. Para seek solo
intercepta el destino de posición en bytes, antes del transporte. No sustituye
las fórmulas ni acredita lectura, bombeo, reloj físico o apertura de un stream.

Miles usa bytes/tasa y redondeo al más cercano con empate al par, no truncado.
La tasa PCM depende de ancho/canales; IMA usa el tamaño de bloque y sus muestras;
ASI usa la tasa de entrada. La alineación de codec elige el bloque cercano, con
empate hacia abajo; PCM alinea hacia abajo a su unidad y limita al total.
Tasa cero devuelve1ms. El adaptador rechaza campos negativos/desbordamientos y
bloques inválidos fuera del subdominio acreditado; no simula esos defectos SDK.

El **reloj PCM16 nativo queda corregido**, incluidas las búsquedas: primero se
redondean bytes y después se alinean fotogramas. Ejemplo a44.100Hz,9ms:397frames,
antes396. También se corrigen consultas a media unidad:1,5ms→2 y2,5ms→2. El
backend continúa midiendo **PCM predecodificado**, también en MP3/IMA. El modelo
de tasa/posición de entrada del SDK está separado: no se inventa su cursor en
bytes a partir del cursor PCM. Conectarlo necesita reconstruir el transporte.

Cuatro pruebas normales nuevas (tres de tiempos y una de inventario MP3) y una
comercial optativa. Siete mutaciones temporales se detectan por aserción y se
restauran: redondeos consulta/seek, tasa ASI, muestras por bloque IMA, empate de
alineación y los dos caminos del reloj nativo. No cambian dependencias, núcleo,
registro de funciones del EXE ni fixtures históricos.
Regresión:151 Rust en la rama y156 al integrar los avances A03 de Claude,
10 comerciales omitidas por defecto; el nuevo inventario comercial sí se repite
en release desde rama y main.547 generales/14omitidas sin fallos, panel3 OK,
Clippy/formato/tablero válidos. A03 real_nav_data y real_routes retornan sin
P3_NAVDATA: sus datos no se validan aquí. Registro312/J0448, tareas y fuentes
ajenas conservadas.

```powershell
python reimpl/oracle/x86_mp3.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --plugin D:/AI/Patrician3/games/iso-english/app/Win32/Mp3dec.asi --report reimpl/p3-audio/evidence/miles-mp3.json --check
python reimpl/oracle/x86_stream_timing.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --report reimpl/p3-audio/evidence/stream-timing.json --check
cargo test --offline --locked --manifest-path reimpl/Cargo.toml -p p3-audio --test stream_timing --test mp3_miles
python reimpl/oracle/x86_mp3.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --plugin D:/AI/Patrician3/games/iso-english/app/Win32/Mp3dec.asi --root D:/AI/Patrician3/games/iso-english/app/Musik --fixture artifacts/miles-mp3-real.json --report reimpl/p3-audio/evidence/miles-mp3-real.json --check
$env:P3_MILES_MP3_FIXTURE=(Resolve-Path artifacts/miles-mp3-real.json).Path
$env:P3_STREAM_ROOT='D:/AI/Patrician3/games/iso-english/app/Musik'
cargo test --release --offline --locked --manifest-path reimpl/Cargo.toml -p p3-audio --test mp3_miles -- --include-ignored
```

**Orden fijo tras estos hallazgos:**1) aislar la primera divergencia del núcleo
LayerIII seleccionado en0x26F05D40 y corregir/portar su comportamiento con pruebas
por etapa;2) resolver finales parciales y estado de seek/reinicio;3) conectar
posición/tasa de entrada, fin y tiempos de HSTREAM;4) lectura progresiva;
5) salida física e integración C09/J03. La paridad MP3 y el runtime temporal
completo siguen pendientes; no avanzar por darlos por cerrados con FFmpeg.

## Bloque19 — primera divergencia aislada y requantización numérica nativa

26/10/2026, PC de Sergio. La saturación grande del caso estéreo sintético3 se
rastrea hasta una lectura fuera de la tabla de magnitudes. El original reserva
0x8000bytes en0x26F025FF y rellena0x2000entradas (comparación0x26F0266B).
En0x26F02B34 aparece el índice8195: lee12bytes después del final. En el heap del
arnés cae en un dword0xFFFFFFFF de otro objeto, se propaga NaN y la síntesis
termina saturando. [Revisión compacta](evidence/mp3-bounds-review.json).

Mover el objeto vecino4.096bytes cambia el PCM bruto sin cambiar el MP3.
El control anterior con reservas A5 no cambiaba las posiciones y no detectaba
esta dependencia. Ese PCM posterior **no es comportamiento definido para portar**.
No se ha abierto el juego ni se afirma cómo se manifiesta con el heap real.

### Referencia acotada que sustituye al inventario anterior

`x86_mp3_defined.py` conserva el inicializador/decoder original y detiene antes
de ejecutar los tres FLD de pow43 si su índice queda fuera de0..8191:
0x26F02AFC,0x26F02B11 y0x26F02B34. Quince sondas comprueban los tres puntos;
8191 pasa,8192/8195/8206/32767 se detienen. Solo entran al fixture peticiones de
PCM que retornaron antes del primer acceso inválido. La unidad canónica de
petición es una trama MPEG; al partirla se comprueba el prefijo común, sin
recuperar bytes de la petición abortada. El guard protege esta tabla, no acredita
todos los accesos del decoder ni la paridad del runtime.

- **16 variantes propias:** tres se cortan antes del índice8195, con1.152
  fotogramas de referencia cada una. Entre todas quedan80.640 fotogramas de
  referencia. Comparados con Rust:104.832 muestras,28.591 distintas, máximo636.
  Seis finales truncados llegan a EOF y conservan la diferencia de una trama;
  los otros dos se detienen antes y no acreditan su EOF. Los seis reinicios
  MPEG2 distintos siguen dentro del rango de esta tabla.
- **123 prefijos reales de4.096 fotogramas:** ninguno alcanza el guard.
  503.808 fotogramas/1.007.616 muestras,394.364 distintas, máximo2.363;
  122 prefijos difieren. No se ha cotejado su PCM completo ni finales originales.

Informes actuales: [origen propio](evidence/miles-mp3-defined.json),
[comparación propia](evidence/mp3-defined-native.json),
[origen real](evidence/miles-mp3-defined-real.json) y
[comparación real](evidence/mp3-defined-real-native.json).
`tests/mp3_miles.rs` utiliza ya estas referencias; sigue marcando
`exact_parity:false`. Pasar el inventario no acepta el decoder. Los archivos
del bloque18 no se sobrescriben ni se usan como referencia vigente.

### Módulo numérico de Rust entregado, aún sin conectar al decoder completo

`native/mp3_requantize.rs` porta los componentes numéricos previos a la síntesis:
tabla de8.192 magnitudes, ganancia global, escalas de banda/subbloque y productos
con signo. Coinciden bit a bit con el original aislado, perfil x87 CW027F:
8.192 magnitudes,144 productos,256 ganancias globales, dos tablas de256 factores
y2.048 productos de banda. Incluye cero con signo, subnormales y overflow del
resultado f32; una ganancia de entrada no finita se rechaza explícitamente.

Se conservan el exponente4/3 de f64 y el almacenamiento f32 del original; la
ganancia de banda se almacena antes de multiplicar la magnitud. La expresión
f32 de la dependencia provisional difiere en8.187/8.192 entradas de magnitud.
Eso localiza una diferencia numérica, pero no atribuye todo el error PCM a esta
tabla ni acredita el resto de las etapas. No se modifica el código instalado
de Symphonia ni se copia una dependencia. Sin versiones o paquetes nuevos.

El módulo devuelve `OutsideOriginalTable` para índices fuera de8192entradas.
Es el límite de la referencia original, no una validación general del formato
MPEG: no se reproduce una lectura de heap para admitirlos. El decoder activo
sigue siendo Symphonia; este módulo **todavía no está conectado a su bitstream**.
No es una sustitución completa de Mp3dec.asi ni añade funciones del EXE a J04.

Tres pruebas nuevas de componentes y sus límites; siete mutaciones nativas y
tres del guard detectadas por aserción y restauradas. Originales de cálculo de
ganancia y producto se ejecutan en ventanas acotadas, con registros/estado FPU
preparados: no se sustituyen sus fórmulas por respuestas del arnés. Informes
[magnitudes](evidence/mp3-requantize.json) y [ganancias](evidence/mp3-gains.json).
Regresión:159 Rust OK,10comerciales omitidas por defecto; el inventario comercial
acotado sí se repite en release.547generales/14omitidas sin fallos; Clippy y
formato OK. Las dos pruebas reales de A03 retornan sin P3_NAVDATA: no se validan
sus datos en este PC. Este bloque no modifica núcleo, Cargo/lock ni registro.
Repetidas desde main las tres pruebas de componentes y las dos de inventario
release (comercial incluida), sin fallos; panel3 y tablero válidos. Registro312,
J0448, fuentes y tareas de Claude conservados.

```powershell
python reimpl/oracle/x86_mp3_defined.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --plugin D:/AI/Patrician3/games/iso-english/app/Win32/Mp3dec.asi --report reimpl/p3-audio/evidence/miles-mp3-defined.json --check
python reimpl/oracle/x86_mp3_defined.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --plugin D:/AI/Patrician3/games/iso-english/app/Win32/Mp3dec.asi --requantize --report reimpl/p3-audio/evidence/mp3-requantize.json --check
python reimpl/oracle/x86_mp3_gains.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --plugin D:/AI/Patrician3/games/iso-english/app/Win32/Mp3dec.asi --report reimpl/p3-audio/evidence/mp3-gains.json --check
cargo test --release --offline --locked --manifest-path reimpl/Cargo.toml -p p3-audio --test mp3_requantize
python reimpl/oracle/x86_mp3_defined.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --plugin D:/AI/Patrician3/games/iso-english/app/Win32/Mp3dec.asi --root D:/AI/Patrician3/games/iso-english/app/Musik --fixture artifacts/miles-mp3-defined-real.json --report reimpl/p3-audio/evidence/miles-mp3-defined-real.json --check
$env:P3_MILES_MP3_DEFINED_FIXTURE=(Resolve-Path artifacts/miles-mp3-defined-real.json).Path
$env:P3_STREAM_ROOT='D:/AI/Patrician3/games/iso-english/app/Musik'
cargo test --release --offline --locked --manifest-path reimpl/Cargo.toml -p p3-audio --test mp3_miles -- --include-ignored
```

### Bloque 20 — escalas y coeficientes Huffman

`native/mp3_scales.rs` lee escalas MPEG-1 y MPEG-2/2.5, bloques largos,
cortos y mixtos, reutilización `scfsi` y estéreo de intensidad. La salida
conserva los campos que el original no escribe. La matriz x86 aislada
comprueba 3.370 combinaciones de compresión/modo/bloque y reserva seis
combinaciones de intensidad con `scalefac_compress` 510 o 511: el original
no inicializa los locales de esa rama. Rust las rechaza; el oráculo no ejecuta
esa rama ni registra resultados inventados. Además se contrastan íntegramente
las 62 posiciones y el cursor de 79 fragmentos sintéticos del decoder.

`native/mp3_huffman.rs` decodifica árboles, regiones, extensión linbits, signos
y cuádruples, conservando los signos antiguos que el original no escribe.
Las 5.266 hojas de los árboles extraídos se prueban; los 79 fragmentos pasan
encadenando escalas nativas y Huffman. El límite de magnitud8192 sigue activo.
Huffman se conecta al módulo numérico mediante **ganancias preparadas por el
original**: 78 espectros sintéticos y 1.968 espectros de 123 prefijos reales
coinciden bit a bit, antes de estéreo. Un sintético se detiene por índice8195,
sin afirmar espectro posterior. Los datos reales permanecen en `artifacts`.

En el bloque20 la asignación de ganancias y el reordenado usaban el mapa
observado como entrada de prueba; el bloque21 siguiente los calcula en Rust.
El reproductor sigue usando Symphonia. La paridad PCM sigue fallando y no se
ha validado sonido en juego ni salida física. No se han sumado funciones del
EXE: los lectores pertenecen a la biblioteca de audio.

Los oráculos `x86_mp3_spectral.py`, `x86_mp3_scales.py` y
`x86_mp3_scale_matrix.py` ejecutan código aislado y admiten `--check`. Las
referencias propias y resúmenes reproducibles están versionados; los fragmentos
comerciales no. Pruebas: `cargo test --manifest-path reimpl/Cargo.toml -p
p3-audio --test mp3_scales --test mp3_spectral`; la prueba real ignorada requiere
`P3_MP3_SPECTRAL_FIXTURE=artifacts/mp3-spectral-real.json`.

### Bloque 21 — ganancias por banda y reordenado nativos

`native/mp3_band.rs` forma ganancias largas y cortas a partir de escalas,
`preflag`, `scalefac_scale`, `global_gain`, ganancias de subbloque y límites
de banda; aplica la requantización Rust sin mapas de ganancias del original.
Los 78 espectros propios definidos y los 1.968 de 123 prefijos reales coinciden
bit a bit con x86 aislado. En los 78 se encadenan escalas → Huffman → ganancias
→ espectro y reordenado Rust desde el flujo propio; uno se detiene por el índice
8195 fuera de pow43. Otros 720 casos sintéticos cubren las seis tablas de
frecuencia, `preflag`, ambos factores de escala, cuatro ganancias globales,
subbloques y tipos largos/cortos/mixtos definidos.

La permutación de ventanas se calcula desde los límites de banda. Se contrasta
con la rutina x86 `0x26F04270`: 18 casos monocanal y 18 de dos canales con la
mezcla estéreo desactivada. También se aplican individualmente sus ramas
monocanal x86 y Rust a los 1.968 espectros reales: coinciden sus 576 posiciones.
Esta última comparación **no** acredita mid/side ni estéreo de intensidad.

Dos límites observados se conservan explícitamente:

- En bloques mixtos monocanal, el original deja 37 posiciones de salida sin
  escribir y escribe otras 36 fuera del bloque de 576; además omite el último
  coeficiente de entrada. `reorder_mono` reproduce la huella solo con un búfer
  de al menos 612 posiciones y rechaza uno más corto. Es evidencia aislada,
  no una afirmación sobre su efecto audible en partida. En la rama de dos
  canales observada el mismo bloque mixto no presenta ese desplazamiento.
- En MPEG-2 mixto, el ancho que usa la requantización procede de un local previo
  de pila; en las tres frecuencias se observan divisores 0 y 7 al cambiar solo
  ese estado. El guard detiene x86 antes de `IDIV` y Rust rechaza la ruta; no
  se fabrica un espectro dependiente de la pila. Hay guard también para todos
  los índices de escala que el original prepara aunque no haya coeficientes.

`x86_mp3_band.py`, `x86_mp3_band_matrix.py`, `x86_mp3_reorder.py`,
`x86_mp3_reorder_stereo.py` y `x86_mp3_mixed_guard.py` aceptan `--check`.
`x86_mp3_reorder_real.py` guarda únicamente bajo `artifacts` las referencias
comerciales; los informes compactos y los datos sintéticos están versionados.
La prueba real de `mp3_band` requiere `P3_MP3_BAND_FIXTURE` y
`P3_MP3_REORDER_FIXTURE`. No se añaden funciones del EXE al registro de J04.

```powershell
python reimpl/oracle/x86_mp3_band.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --plugin D:/AI/Patrician3/games/iso-english/app/Win32/Mp3dec.asi --root D:/AI/Patrician3/games/iso-english/app/Musik --fixture artifacts/mp3-band-real.json --report reimpl/p3-audio/evidence/mp3-band-real.json --check
python reimpl/oracle/x86_mp3_reorder_real.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --plugin D:/AI/Patrician3/games/iso-english/app/Win32/Mp3dec.asi --input artifacts/mp3-band-real.json --output artifacts/mp3-reorder-real.json
$env:P3_MP3_BAND_FIXTURE=(Resolve-Path artifacts/mp3-band-real.json).Path
$env:P3_MP3_REORDER_FIXTURE=(Resolve-Path artifacts/mp3-reorder-real.json).Path
cargo test --release --offline --locked --manifest-path reimpl/Cargo.toml -p p3-audio --test mp3_band -- --include-ignored
```

Si todavía no existe la captura real local, omitir `--check` en la primera
orden para crearla; los datos comerciales permanecen ignorados por Git.

### Bloque 22 — estéreo y reordenado de dos canales

`native/mp3_stereo.rs` reproduce la etapa escalar de `0x26F04270`: determina
las posiciones de intensidad a partir de las escalas y del último valor no
nulo del canal derecho, aplica mid/side o intensidad y reordena cada canal.
Admite bloques largos, cortos y mixtos, incluidos tipos distintos entre canales,
MPEG-1/2, las seis tablas de frecuencia y la combinación de ambos modos.
Conserva el valor binario exacto de la constante mid/side del original
`0.707106781`, así como la copia observada en la última banda corta. No toma
mapas ni factores preparados por el oráculo. La información lateral y el
estado anterior son entradas explícitas; no es un lector completo de trama.

La comparación x86 aislada verifica **3.528 casos sintéticos**: 720 de modos,
1.440 de posiciones de intensidad, 432 numéricos, 864 de tipos de bloque
entre canales y 72 de bordes finitos de `f32`. Incluyen intensidad MPEG-1 y
MPEG-2, ceros con signo, subnormales y desbordamiento de salida. Se contrastan
tanto las 576 posiciones de intensidad como los 1.152 valores de salida.
Seis mutaciones de constante, exponentes, copia de banda, mixtos, reordenado
y prioridad de intensidad fueron detectadas y restauradas.

Los **984 gránulos de dos canales de 123 prefijos reales** coinciden con x86;
734 usan mid/side y ninguno utiliza intensidad. También coinciden 22 gránulos
propios de dos canales. Una segunda prueba encadena las ganancias por banda
Rust del bloque21 con el estéreo y reordenado Rust y compara el resultado para
los mismos 984 gránulos reales y 22 propios. En esos casos la entrada del
módulo de bandas se alimenta de las escalas y símbolos capturados: no es una
prueba del decoder completo de principio a fin.

El puerto rechaza entradas no finitas y posiciones de intensidad fuera del
rango contrastado: MPEG-1 `0..7` y MPEG-2 `0..31`. El valor 7 conserva la
rama sin intensidad del original. Se mantienen los guards del bloque21 para
MPEG-2 mixto y el reordenado monocanal con búfer explícito; no se les atribuye
paridad en juego. Los recursos reales capturados no prueban intensidad: la
cobertura de esa rama es sintética. Informes compactos: [propio](evidence/mp3-stereo.json)
y [real](evidence/mp3-stereo-real.json); datos comerciales solo en `artifacts`.

Los oráculos `x86_mp3_stereo.py`, `x86_mp3_stereo_matrix.py`,
`x86_mp3_intensity_matrix.py`, `x86_mp3_stereo_numeric.py`,
`x86_mp3_stereo_shapes.py` y `x86_mp3_stereo_edges.py` admiten `--check`.

```powershell
python reimpl/oracle/x86_mp3_stereo.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --plugin D:/AI/Patrician3/games/iso-english/app/Win32/Mp3dec.asi --root D:/AI/Patrician3/games/iso-english/app/Musik --fixture artifacts/mp3-stereo-real.json --report reimpl/p3-audio/evidence/mp3-stereo-real.json --check
$env:P3_MP3_STEREO_FIXTURE=(Resolve-Path artifacts/mp3-stereo-real.json).Path
$env:P3_MP3_BAND_FIXTURE=(Resolve-Path artifacts/mp3-band-real.json).Path
cargo test --release --offline --locked --manifest-path reimpl/Cargo.toml -p p3-audio --test mp3_stereo -- --include-ignored
```

Si aún no existe la captura real, omitir `--check` para crearla. La captura de
bandas se prepara con las instrucciones del bloque21.

### Bloque 23 — antialias y síntesis híbrida

`native/mp3_hybrid.rs` porta antialias, selección de subbandas y las IMDCT
larga y corta con su solapamiento. La selección conserva dos detalles del
original: siempre procesa las dos primeras subbandas, y omite antialias en
todos los bloques cortos, incluidos los mixtos. Las operaciones largas y
cortas de `mp3_imdct_long.rs` y `mp3_imdct_short.rs` se generan como aritmética
Rust desde las operaciones escalares originales; cada almacenamiento intermedio
conserva su redondeo `f32`. Los generadores verifican los hashes de las DLL y
admiten `--check`; no se versionan bytes ejecutables comerciales.

La comparación con x86 aislado, perfil escalar x87 CW027F, cubre:

- 256 casos de antialias y 144 secuencias sintéticas del controlador, con
  ventanas de inicio y fin, bloques cortos y mixtos, colas y solapamiento.
- 48 llamadas IMDCT sintéticas dentro del rango admitido, más 16 estímulos
  extremos que el puerto rechaza. Se comparan entrada modificada, salida y
  solapamiento, no solo el resultado final.
- 78 canales de gránulos propios y **1.968 de 123 prefijos reales**, de los
  cuales 116 son cortos y ninguno mixto. Rust conserva el solapamiento de un
  gránulo al siguiente. También se encadenan estéreo Rust → antialias → IMDCT
  en 984 gránulos reales de dos canales. Las entradas espectrales y la
  información lateral siguen procediendo de capturas; no es el decoder completo.

La IMDCT rechaza entradas no finitas y magnitudes superiores a `2^100`, un
límite conservador del puerto. Con estímulos cercanos al máximo de `f32`, las
operaciones intermedias producen NaN y el signo de algunos NaN difiere entre
compilaciones normal y optimizada; esos resultados no se presentan como
paridad. Si se rechaza la operación, `hybrid_channel` conserva sus buffers.
El guard no afirma que todas las entradas dentro del límite estén probadas.
Se mantienen los límites de pow43 y los mixtos del bloque21. No se ha probado
otra ruta SIMD, salida física ni juego.

Informes compactos: [híbrida propia](evidence/mp3-hybrid.json),
[híbrida real](evidence/mp3-hybrid-real.json) y
[antialias real](evidence/mp3-antialias-real.json). `x86_mp3_antialias.py`,
`x86_mp3_imdct.py`, `x86_mp3_hybrid_matrix.py`, `x86_mp3_hybrid.py` y los
dos generadores de IMDCT admiten `--check`. Los datos reales solo se guardan
bajo `artifacts`.

```powershell
python reimpl/oracle/x86_mp3_hybrid.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --plugin D:/AI/Patrician3/games/iso-english/app/Win32/Mp3dec.asi --root D:/AI/Patrician3/games/iso-english/app/Musik --fixture artifacts/mp3-hybrid-real.json --report reimpl/p3-audio/evidence/mp3-hybrid-real.json --check
python reimpl/oracle/x86_mp3_antialias_real.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --plugin D:/AI/Patrician3/games/iso-english/app/Win32/Mp3dec.asi --input artifacts/mp3-stereo-real.json --output artifacts/mp3-antialias-real.json
$env:P3_MP3_HYBRID_FIXTURE=(Resolve-Path artifacts/mp3-hybrid-real.json).Path
$env:P3_MP3_STEREO_FIXTURE=(Resolve-Path artifacts/mp3-stereo-real.json).Path
$env:P3_MP3_ANTIALIAS_FIXTURE=(Resolve-Path artifacts/mp3-antialias-real.json).Path
cargo test --release --offline --locked --manifest-path reimpl/Cargo.toml -p p3-audio --test mp3_hybrid -- --include-ignored
```

Si aún no existe la captura híbrida real, omitir `--check` para crearla. La
captura estéreo se prepara con las instrucciones del bloque22.

### Bloque 24 — síntesis polifásica y estado

`native/mp3_polyphase.rs` implementa la DCT de 32 subbandas, la ventana y
la conversión a PCM16. Mantiene dos bancos de 17 × 16 valores por canal,
una posición circular y una fase; al procesar cada instante actualiza la
posición y alterna la fase. La DCT y las sumas de la ventana se generan desde
las operaciones x86 escalares y sus constantes numéricas, sin incluir código
binario comercial. La conversión usa redondeo al par, y conserva el entero
indefinido x87 para conversiones inválidas antes de limitar a PCM16.

En x86 aislado, perfil x87 CW027F, coinciden **1.152 transformadas**, **320
ventanas** y **3.456 pasos de secuencias sintéticas**, con las 16 posiciones,
ambas fases, vuelta circular y bancos inicialmente no nulos. Además se
encadenan la síntesis híbrida y la polifásica Rust en 78 canales propios y
**1.968 canales de 123 prefijos reales**: 35.424 pasos y 1.133.568 muestras
PCM, comprobando los dos bancos tras cada paso. La entrada espectral de la
etapa híbrida sigue capturada del original; no es una prueba integral del
lector y decoder. En esos prefijos el estado inicial observado es cero, con
posición y fase cero; después Rust conserva ambos estados entre gránulos.
La comparación con el controlador original detectó y permitió corregir una
inversión de los bancos que los tests de las etapas por separado no detectaban.

Se rechazan entradas o estados no finitos y magnitudes mayores que `2^100`;
los pasos rechazados no alteran el estado. El límite es conservador y no
acredita todas las entradas finitas. Se conservan los límites de pow43, bloques
mixtos y síntesis híbrida. No se ha contrastado la ruta SIMD, salida física ni
juego. Symphonia sigue en el reproductor; no se declara paridad PCM integral.

Informes compactos: [propio](evidence/mp3-polyphase.json) y
[real](evidence/mp3-polyphase-real.json); las capturas comerciales permanecen
en `artifacts`. Los oráculos `x86_mp3_poly_dct.py`, `x86_mp3_poly_window.py`,
`x86_mp3_poly_sequence.py`, `x86_mp3_polyphase.py` y ambos generadores admiten
`--check` con las mismas DLL originales verificadas por hash, ejecutadas solo
en el emulador x86 aislado.

```powershell
python reimpl/oracle/x86_mp3_polyphase.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --plugin D:/AI/Patrician3/games/iso-english/app/Win32/Mp3dec.asi --root D:/AI/Patrician3/games/iso-english/app/Musik --fixture artifacts/mp3-polyphase-real.json --report reimpl/p3-audio/evidence/mp3-polyphase-real.json --check
$env:P3_MP3_HYBRID_FIXTURE=(Resolve-Path artifacts/mp3-hybrid-real.json).Path
$env:P3_MP3_POLYPHASE_FIXTURE=(Resolve-Path artifacts/mp3-polyphase-real.json).Path
cargo test --release --offline --locked --manifest-path reimpl/Cargo.toml -p p3-audio --test mp3_polyphase -- --include-ignored
```

Si la captura real aún no existe, omitir `--check` para crearla; la captura
híbrida se prepara con las instrucciones del bloque23.

### Bloque 25 — archivo MP3 hasta PCM definido

`native/mp3_frame.rs` lee desde los bytes de archivo la cabecera y la
información lateral Layer III de MPEG-1 y MPEG-2, incluida una etiqueta ID3v2
inicial y CRC en la trama. `native/mp3_decoder.rs` reconstruye el depósito de
bits y encadena escalas, Huffman, ganancias, estéreo/reordenado, híbrida y
síntesis polifásica. Conserva las escalas, signos, solapamiento y bancos entre
tramas y canales. El límite entre regiones de bloques de inicio/fin MPEG-2 usa
la tabla de bandas de su frecuencia; aplicarle siempre 36 hacía divergir el PCM.

`decode_prefix` ofrece un prefijo PCM16 desde el archivo, sin datos de etapas
preparados por x86; `Decoder::decode_frame` conserva el estado entre tramas
completas y revierte los cambios si una falla. El prefijo puede terminar dentro
de una trama y descarta lo que sobra de ella: su `next_offset` no permite
reanudar esas muestras. Es un API de contraste, aún no un transporte de sonido.

Se comparó el PCM bit a bit con [la referencia x86 definida](evidence/miles-mp3-defined-real.json),
en perfil escalar x87 CW027F: **1.007.616 muestras, 503.808 fotogramas, de
123 prefijos reales de 4.096 fotogramas cada uno**, leyendo el MP3 directamente.
Se comprueban el hash de cada archivo y el de la captura x86. También se
contrastaron 16 variantes propias: diez alcanzan todo el PCM definido
(incluidos tres casos detenidos por pow43); las otras seis se resolvieron
después, en el bloque26. Cuatro mutaciones de región, depósito, estéreo y
estado detectadas y restauradas. El guard rechaza el índice8195 de los tres
casos propios antes del acceso indefinido original; los 123 prefijos reales no
alcanzaron ese guard. Se verifican los rechazos transaccionales del decoder.

Límites vigentes: se exige cabecera, información lateral y bits necesarios, y
formato estable; MPEG-2.5, bitrate libre, depósito inicial insuficiente, bits
comprimidos ausentes y mono mixto se rechazan. También siguen vigentes el guard pow43, MPEG-2 mixto, límites de
magnitud y ramas de intensidad no cubiertas por la música real. El lector no
busca sincronización arbitraria tras ID3. La referencia real son prefijos,
no archivos completos; final y lectura secuencial tienen la cobertura acotada
del bloque26. Los seek del proveedor tienen la cobertura acotada de los bloques27 y28; no se
ha contrastado entrada progresiva, SIMD ni dispositivo. **Symphonia sigue en el reproductor**:
la paridad de esta cadena de laboratorio no equivale a integración ni QA en
juego. Los datos musicales y las capturas permanecen bajo `artifacts`.

```powershell
$env:P3_MP3_ROOT='D:/AI/Patrician3/games/iso-english/app/Musik'
$env:P3_MP3_DEFINED_FIXTURE=(Resolve-Path artifacts/miles-mp3-defined-real.json).Path
cargo test --release --offline --locked --manifest-path reimpl/Cargo.toml -p p3-audio --test mp3_full real_files_to_pcm -- --ignored
```

La captura se reproduce siguiendo las instrucciones del bloque19.

### Bloque 26 — finales y lectura secuencial por bytes

`parse_final_frame` acepta una última trama físicamente incompleta cuando
contiene cabecera e información lateral. El decoder exige que todos los bits
que realmente consume estén presentes: no rellena con ceros datos comprimidos
faltantes. `parse_frame` conserva su contrato estricto. `decode_prefix` marca
la última trama incompleta y su cursor termina en el último byte disponible.
Si una trama falla, el estado del decoder se conserva.

De 66 recortes propios comparados con x86, **54 producen idénticas 513.216
muestras PCM**; 46 de ellos terminan en tramas físicamente incompletas. Los
otros 12 se detienen de forma segura por bits ausentes; el original emulado
produjo una salida para ellos, pero no se afirma equivalencia ni se inventan
esos bits. Las seis variantes recortadas del bloque25 ya producen todo el PCM
que entregó x86. El oráculo inicializa a cero la memoria del asignador; la
comparación no acredita el contenido de memoria de cualquier otro entorno.

`ByteStream` conserva el PCM restante entre lecturas de tamaños arbitrarios,
incluidos bytes sueltos. En EOF limpio pone a cero el resto del búfer, como
se observó en el proceso original. Si una trama falla tras entregar bytes,
devuelve primero los bytes ya escritos y mantiene el error para la siguiente
lectura; no confunde ese error con EOF. Se contrastaron **18 secuencias y
303 llamadas x86** sobre archivos propios, incluida la cola tras EOF. También
coinciden las **1.968 lecturas** de los 123 prefijos reales con el PCM x86 ya
capturado. Dos efectos reales, `kontor` y `schmied`, tienen contenido idéntico:
ese único MP3 distinto se comparó hasta EOF en ambas rutas, con **200 llamadas
x86 en total** y 57.600 muestras por ruta; coinciden los bytes y el relleno
tras EOF. Informe [compacto](evidence/mp3-short-eof.json); PCM solo en
`artifacts`. Cuatro mutaciones de aceptación
de trama, cursor, relleno EOF y guard de bits fueron detectadas y restauradas.

`ByteStream` recibe el archivo completo en memoria; los seek del proveedor se
contrastan en los bloques27 y28. No implementa entrada progresiva ni transporte
HSTREAM. La comparación cubre perfil
escalar; no acredita SIMD, dispositivo ni juego. El reproductor sigue usando
Symphonia. Siguen vigentes los guards pow43, mixtos y magnitud. Los 12 recortes
rechazados y la semántica original de bits ausentes permanecen abiertos.

```powershell
python reimpl/oracle/x86_mp3_endings.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --plugin D:/AI/Patrician3/games/iso-english/app/Win32/Mp3dec.asi --check
python reimpl/oracle/x86_mp3_stream_bytes.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --plugin D:/AI/Patrician3/games/iso-english/app/Win32/Mp3dec.asi --check
python reimpl/oracle/x86_mp3_short_eof.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --plugin D:/AI/Patrician3/games/iso-english/app/Win32/Mp3dec.asi --root D:/AI/Patrician3/games/iso-english/app/Musik --fixture artifacts/mp3-short-eof.json --report reimpl/p3-audio/evidence/mp3-short-eof.json --check
$env:P3_MP3_ROOT='D:/AI/Patrician3/games/iso-english/app/Musik'
$env:P3_MP3_DEFINED_FIXTURE=(Resolve-Path artifacts/miles-mp3-defined-real.json).Path
$env:P3_MP3_SHORT_EOF_FIXTURE=(Resolve-Path artifacts/mp3-short-eof.json).Path
cargo test --release --offline --locked --manifest-path reimpl/Cargo.toml -p p3-audio --test mp3_full real_sequential_reads_match_x86_prefixes -- --ignored
cargo test --release --offline --locked --manifest-path reimpl/Cargo.toml -p p3-audio --test mp3_full real_short_effects_match_x86_through_eof -- --ignored
```

Si la captura de efectos no existe aún, omitir `--check` para crearla.
### Bloque 27 — reinicio a byte cero

`Decoder::rewind` y `ByteStream::rewind` reproducen el caso `seek(0)` de
`Mp3dec.asi` escalar (`0x26F01AD0`). El original borra el solapamiento,
reinicia las posiciones y fases, descarta la salida pendiente y vuelve a
leer desde el inicio. **No borra los bancos de síntesis polifásica**; las
escalas y signos tampoco se limpian en esa rutina. Rust conserva esos datos,
borra el depósito lógico y descarta PCM pendiente y errores anteriores. El
método recibe un archivo ya cargado y solo implementa el destino cero.

La comparación con x86 aislado cubre **10 secuencias propias, 30 reinicios y
90 lecturas**, con tamaños impares, reinicios repetidos, al terminar un archivo
y tres archivos con etiqueta ID3. También se comparan **seis reinicios y 18
lecturas** de dos rutas que contienen el mismo efecto MP3 real; incluyen el
reinicio tras EOF. Se contrastan longitud, bytes PCM y cola cero en cada
lectura. El oráculo verifica que el banco original conserva su hash, que el
solapamiento queda a cero y que posición y fase se reinician. Sus capturas
reales quedan en `artifacts`; [informe compacto](evidence/mp3-rewind-real.json).

En los dos ejemplos MPEG-2 con señal, el PCM al volver al inicio tras EOF es
distinto del obtenido al abrir: el puerto reproduce esa diferencia al mantener
el estado del filtro. También aparece una diferencia tras reiniciar durante
la lectura de un ejemplo MPEG-1. Cuatro mutaciones —borrar los bancos,
conservar el solapamiento, alterar la posición o no vaciar PCM pendiente—
fueron detectadas y restauradas. Esto no prueba todos los estados posibles ni
el comportamiento tras un fallo que haya detenido el oráculo.

Un sondeo x86 adicional guarda 27 operaciones de posición en
[mp3_seek_probe.json](tests/fixtures/mp3_seek_probe.json): en los 24 casos
ordinarios, incluida la posición cero, el siguiente callback empieza en el
byte pedido y se vacían los cursores de salida. En tres casos con `-2` se
conservan esos cursores y los siguientes 512 bytes salen sin callback nuevo.
Es una observación acotada tras una lectura de 512 bytes, no una implementación
ni una prueba de todos los posibles estados del depósito.

Los destinos distintos de cero y el centinela `-2` se implementaron después en
el bloque28; todavía falta el transporte HSTREAM. Siguen abiertos los 12 recortes con bits faltantes y los
guards de pow43/mixtos; Symphonia sigue en el reproductor. No se ha probado
entrada progresiva, SIMD, dispositivo ni juego.

```powershell
python reimpl/oracle/x86_mp3_rewind.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --plugin D:/AI/Patrician3/games/iso-english/app/Win32/Mp3dec.asi --check
python reimpl/oracle/x86_mp3_seek_probe.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --plugin D:/AI/Patrician3/games/iso-english/app/Win32/Mp3dec.asi --check
python reimpl/oracle/x86_mp3_rewind_real.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --plugin D:/AI/Patrician3/games/iso-english/app/Win32/Mp3dec.asi --root D:/AI/Patrician3/games/iso-english/app/Musik --fixture artifacts/mp3-rewind-real.json --report reimpl/p3-audio/evidence/mp3-rewind-real.json --check
$env:P3_MP3_ROOT='D:/AI/Patrician3/games/iso-english/app/Musik'
$env:P3_MP3_DEFINED_FIXTURE=(Resolve-Path artifacts/miles-mp3-defined-real.json).Path
$env:P3_MP3_SHORT_EOF_FIXTURE=(Resolve-Path artifacts/mp3-short-eof.json).Path
$env:P3_MP3_REWIND_FIXTURE=(Resolve-Path artifacts/mp3-rewind-real.json).Path
cargo test --release --offline --locked --manifest-path reimpl/Cargo.toml -p p3-audio --test mp3_full seek_zero_preserves_original_synthesis_state -- --include-ignored
```

Si no existe la captura real, omitir `--check` para crearla.

### Bloque 28 — seek por byte comprimido y centinela -2

`ByteStream::seek_input` recibe una posición **en bytes del MP3 comprimido**,
no en muestras ni milisegundos. El destino cero usa el reinicio del bloque27;
un destino ordinario descarta el PCM pendiente y vuelve a sincronizar desde el
byte solicitado. `0xFFFFFFFE` (`-2`) conserva la salida pendiente, reinicia el
estado de síntesis y depósito, y continúa desde la posición física de lectura.
En este bloque `0xFFFFFFFF` (`-1`) se rechazaba; el bloque29 acredita e implementa su transición acotada.

El scanner reproduce un detalle observado en `0x26F010B0`: tras encontrar un
candidato de cuatro bytes inválido, continúa después de esos bytes, sin volver
a examinar las cabeceras solapadas. Después de seek se conservan los bancos
polifásicos; si una trama necesita historial de depósito todavía no reunido,
se acumulan sus datos y se avanza sin emitir PCM hasta que haya suficiente.
Se guardan por separado la posición solicitada y la posición física: por eso
`seek(0)` seguido inmediatamente de `-2` no se confunde con haber leído ya
desde cero. El estado rechazado por el guard no se borra con `-2`.

El perfil x86 escalar coincide bit a bit en:

- **132 secuencias y 528 lecturas** propias, con destinos de byte y `-2`,
  posiciones intermedias, fuera del archivo, y distintas lecturas previas.
- **63 parejas de seek y 252 lecturas** propias, incluidas búsquedas sucesivas
  sin lectura entre ellas; y 28 secuencias específicas de `-2`, con 56
  comandos y 165 lecturas, algunas con ID3 y desde antes de la primera lectura.
- **36 secuencias y 144 lecturas** de un efecto MP3 real, incluida la cola de
  EOF; [informe compacto](evidence/mp3-seek-sequences-real.json). El archivo
  real de esa captura queda bajo `artifacts`.

En 27 casos originales adicionales se conservan también los offsets del
callback y los cursores como evidencia. Cinco mutaciones —scanner solapado,
cursor físico, depósito, salida pendiente en `-2` y vaciado en seek ordinario—
fueron detectadas y restauradas. La cobertura no acredita todas las cabeceras,
todas las entradas dañadas ni la fuente progresiva; siguen vigentes los límites
de formato, pow43, mixtos y los 12 recortes con bits ausentes.

En el bloque28 el API trabajaba con archivo cargado sin conectar HSTREAM; el bloque29 añade un puente acotado de seek temporal. Symphonia sigue en el reproductor.
No se ha probado SIMD, dispositivo ni juego.

```powershell
python reimpl/oracle/x86_mp3_special_seek.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --plugin D:/AI/Patrician3/games/iso-english/app/Win32/Mp3dec.asi --check
python reimpl/oracle/x86_mp3_seek_sequences.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --plugin D:/AI/Patrician3/games/iso-english/app/Win32/Mp3dec.asi --check
python reimpl/oracle/x86_mp3_seek_chain.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --plugin D:/AI/Patrician3/games/iso-english/app/Win32/Mp3dec.asi --check
python reimpl/oracle/x86_mp3_seek_sequences_real.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --plugin D:/AI/Patrician3/games/iso-english/app/Win32/Mp3dec.asi --root D:/AI/Patrician3/games/iso-english/app/Musik --fixture artifacts/mp3-seek-sequences-real.json --report reimpl/p3-audio/evidence/mp3-seek-sequences-real.json --check
$env:P3_MP3_ROOT='D:/AI/Patrician3/games/iso-english/app/Musik'
$env:P3_MP3_DEFINED_FIXTURE=(Resolve-Path artifacts/miles-mp3-defined-real.json).Path
$env:P3_MP3_SHORT_EOF_FIXTURE=(Resolve-Path artifacts/mp3-short-eof.json).Path
$env:P3_MP3_REWIND_FIXTURE=(Resolve-Path artifacts/mp3-rewind-real.json).Path
$env:P3_MP3_SEEK_FIXTURE=(Resolve-Path artifacts/mp3-seek-sequences-real.json).Path
cargo test --release --offline --locked --manifest-path reimpl/Cargo.toml -p p3-audio --test mp3_full -- --include-ignored
```

Si la captura real no existe, omitir `--check` para crearla. El bloque29 continúa con seek temporal y pausa; los búferes, fin y bucles siguen pendientes.


### Bloque 29 — seek temporal y transición de pausa HSTREAM

`Mp3SeekBridge` conecta la conversión temporal y la alineación de HSTREAM con
`ByteStream` para un MP3 ya cargado y un proveedor abierto. El puente acepta solo PCM16 mono o
estéreo conforme al MP3 y una tasa de códec no nula, dominio comparado. Se corrigió la
alineación: con proveedor abierto se alinea al ancho PCM; sin él se conserva el
redondeo por bloque anteriormente contrastado. El puente ejecuta el reinicio
especial `-1` y aplica el desplazamiento comprimido; conserva los guards de error.
`-1` reinicia síntesis y depósito, descarta PCM pendiente y conserva la posición
física: **28 secuencias, 56 seek y 165 lecturas** comparadas con el proveedor
x86, con tamaños previos distintos y tres archivos con etiqueta ID3.

La secuencia temporal llama al control HSTREAM y al proveedor originales en
emulación x86; el sample y `SetFilePointer` son simulados. Los **63 casos y 252
lecturas** abarcan mono/estéreo, dos frecuencias, varias posiciones previas y
destinos hasta el final; coinciden posición calculada, alineación, bytes PCM y
cola cero. No se ejecuta la rutina de alimentación de búferes original en esta
comparación. La prueba guarda estado de HSTREAM como evidencia adicional.

`PauseControl` reproduce **120 transiciones** de pausa y reanudación en un
HSTREAM preparado sin error. Los eventos de consulta, fin/reanudación de sample,
arranque y servicio se contrastan frente a x86 y son fronteras simuladas, no
operaciones de un dispositivo. El modelo de pausa aún no está unido al puente.

Siguiente: portar y contrastar el servicio de búferes, fin y bucles de HSTREAM
y conectar sus transiciones al proveedor; después lectura progresiva y conexión
al reproductor. Los 12 recortes rechazados, guards pow43/mixtos y demás límites
previos siguen abiertos. Symphonia sigue en uso; ninguna prueba es en juego.

```powershell
python reimpl/oracle/x86_mp3_minus_one.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --plugin D:/AI/Patrician3/games/iso-english/app/Win32/Mp3dec.asi --check
python reimpl/oracle/x86_hstream_seek.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --plugin D:/AI/Patrician3/games/iso-english/app/Win32/Mp3dec.asi --check
python reimpl/oracle/x86_hstream_pause.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --check
cargo test --release --offline --locked --manifest-path reimpl/Cargo.toml -p p3-audio --test mp3_full
cargo test --release --offline --locked --manifest-path reimpl/Cargo.toml -p p3-audio --test hstream_pause
```


### Bloque 30 — servicio, fin y bucles HSTREAM

`native/hstream_buffers.rs` porta el servicio de archivo normal
`0x21122020`, el llenado `0x21122260` y la entrega `0x21121380` en un dominio
seguro y de un solo hilo. Conserva tres búferes, presupuesto y orden rotatorio;
contabiliza bytes, relleno de códec, posiciones, bucles finitos o infinitos,
lectura corta, EOF de la fuente y entrega del búfer vacío que marca el fin.
Si un bucle recoloca el archivo, avisa al proveedor con `-2`. La conexión de
`StreamController` aplica pausa y reanudación y bombea hasta dos búferes al
sample, siguiendo el servicio global original para un stream.

Se ejecutaron las rutinas originales en emulación x86 aislada con E/S y sample
simulados, y se compararon estado, bytes y orden de llamadas:

- Servicio y llenado: **384 escenarios y 1.072 llamadas**, con 2.440 lecturas,
  288 reposicionamientos, 140 avisos al proveedor, 104 finales y 192 lecturas
  cortas. Incluye desplazamiento físico inicial del archivo y relleno en límites de bucle y de archivo.
- Entrega y fin: **72 escenarios y 216 llamadas**; pausa/reanudación con
  entrega: **32 secuencias y 64 llamadas**.
- HSTREAM y `Mp3dec.asi` originales juntos: **seis secuencias** con llenado,
  `-2`, pausa, reanudación y entrega al sample simulado; coincide el búfer y
  la siguiente lectura de 512 bytes de PCM pendiente. `MemoryMp3Io` conecta
  el aviso `-2` al `ByteStream` Rust y conserva su guard de error.

La última prueba no acredita la decodificación de los bytes entregados al
sample: el proveedor de la comparación lee directamente desde la fuente
simulada y esos 512 bytes ya estaban pendientes. El bloque31 conecta la entrada progresiva a esos búferes y contrasta
lecturas posteriores al PCM pendiente, con bucles, EOF y seek en el dominio
descrito a continuación. El arranque inicial,
el callback de fin, el dispositivo, concurrencia y lista de múltiples streams
son fronteras no portadas de este bloque. Los 12 recortes rechazados y guards
previos siguen vigentes. Symphonia sigue en el reproductor; sin prueba en juego.

```powershell
python reimpl/oracle/x86_hstream_service.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --check
python reimpl/oracle/x86_hstream_dispatch.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --check
python reimpl/oracle/x86_hstream_pause_dispatch.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --check
python reimpl/oracle/x86_hstream_provider.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --plugin D:/AI/Patrician3/games/iso-english/app/Win32/Mp3dec.asi --check
cargo test --release --offline --locked --manifest-path reimpl/Cargo.toml -p p3-audio --test hstream_service
```


### Bloque 31 — decoder MP3 alimentado por búferes HSTREAM

`ProgressiveMp3` recibe fragmentos comprimidos con memoria acotada. Devuelve
`NeedInput` sin fabricar EOF ni borrar la cola de salida; el fin explícito
permite distinguir una última trama incompleta. Descarta tramas ya consumidas,
conserva PCM pendiente y reinicia depósito/síntesis en `-2`. Tras seek temporal
se vacían el sample y los tres búferes, se recoloca el archivo y el decoder
reanuda la búsqueda desde el nuevo origen. Los errores existentes se conservan.
Si la capacidad acotada no permite entregar otro búfer necesario, se devuelve
`InvalidLimit` en vez de quedar esperando indefinidamente.

`ProgressiveSample` recibe los búferes de `BufferService::dispatch` y los
alimenta al decoder; `ProgressiveFileIo` conecta los avisos del servicio. Se
comparó la cadena con las rutinas HSTREAM y el proveedor `Mp3dec.asi` originales
en x86 aislado. El sample es simulado: acepta los búferes y atiende el callback
del proveedor desde una cola. Los ensayos no agotan esa cola antes de EOF.
Se compararon bytes entregados, posiciones, fin, avisos y PCM:

- Tres MP3 sintéticos, sin bucle y con un bucle que interrumpe una trama:
  **6 secuencias y 254 lecturas** hasta EOF. El scanner vuelve a sincronizar
  entre tramas, como el original al encontrar bytes ajenos.
- Los mismos tres archivos con cuatro seek temporales: **12 secuencias y
  325 lecturas**, incluidos seek al inicio, intermedios y más allá de EOF.
- Un efecto real local, `Gebaeude/kontor.mp3`: **2 secuencias/452 lecturas**
  con y sin bucle y **4 secuencias/461 lecturas** con seek. Para el seek real
  se entregan búferes de 4.096 bytes y se precarga el origen restante: el
  proveedor puede recorrer más de 2.304 bytes buscando la siguiente cabecera.
  Los datos comerciales y el PCM quedan bajo `artifacts`; informes de hash en
  [bucle/EOF](evidence/hstream-progressive-real.json) y
  [seek](evidence/hstream-progressive-seek-real.json).
- Fragmentos propios de 1, 7, 23, 127 y 512 bytes reproducen el PCM de las
  capturas x86 hasta EOF sin tratar la falta temporal como fin. Esta prueba
  verifica el puerto Rust, no un callback de sample original con underrun.

**Límites del bloque31:** el sample y el archivo del oráculo se simulaban.
El bloque32 contrasta el callback original, sus underruns aislados y el arranque;
concurrencia, llegada asíncrona y salida física siguen pendientes. El seek requiere un sample ya
iniciado, proveedor abierto y formato PCM16 mono/estéreo comprobado. Los guards
de datos ausentes, pow43 y mixtos siguen vigentes. Symphonia continúa en el
reproductor y no se ha probado en juego. Siguiente: conectar el puerto
progresivo al reproductor, contrastar el callback original del sample y sus
underruns, arranque y pausa; después la salida física.

```powershell
python reimpl/oracle/x86_hstream_progressive.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --plugin D:/AI/Patrician3/games/iso-english/app/Win32/Mp3dec.asi --check
python reimpl/oracle/x86_hstream_progressive_seek.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --plugin D:/AI/Patrician3/games/iso-english/app/Win32/Mp3dec.asi --check
python reimpl/oracle/x86_hstream_progressive_real.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --plugin D:/AI/Patrician3/games/iso-english/app/Win32/Mp3dec.asi --input D:/AI/Patrician3/games/iso-english/app/Musik/Gebaeude/kontor.mp3 --fixture artifacts/hstream-progressive-real.json --report reimpl/p3-audio/evidence/hstream-progressive-real.json --check
python reimpl/oracle/x86_hstream_progressive_seek_real.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --plugin D:/AI/Patrician3/games/iso-english/app/Win32/Mp3dec.asi --input D:/AI/Patrician3/games/iso-english/app/Musik/Gebaeude/kontor.mp3 --fixture artifacts/hstream-progressive-seek-real.json --report reimpl/p3-audio/evidence/hstream-progressive-seek-real.json --check
$env:P3_MP3_ROOT='D:/AI/Patrician3/games/iso-english/app/Musik'
$env:P3_HSTREAM_PROGRESSIVE_FIXTURE=(Resolve-Path artifacts/hstream-progressive-real.json).Path
$env:P3_HSTREAM_PROGRESSIVE_SEEK_FIXTURE=(Resolve-Path artifacts/hstream-progressive-seek-real.json).Path
cargo test --release --offline --locked --manifest-path reimpl/Cargo.toml -p p3-audio --test hstream_progressive -- --include-ignored
```


### Bloque 32 — reproductor progresivo y callback de sample

El reproductor lógico `NativeEffects<OfflineDevice>` puede abrir MP3 con
`backend.mp3_backend = Mp3Backend::NativeProgressive`. La fuente comprimida
sigue en memoria; `ProgressivePlayback` conecta HSTREAM, dos búferes de
sample, el decoder progresivo y la mezcla PCM fuera de línea. Apertura, servicio,
arranque, pausa, reanudación, seek, bucle, volumen y cierre usan la API existente.
No se predecodifica todo el MP3. La ruta predecodificada con Symphonia sigue
siendo la opción por defecto mientras se contrastan las rutas pendientes.

El callback de entrada original es `0x211119C0`: al agotar el primer búfer
puede copiar del segundo; si este no está disponible, devuelve lo copiado y
marca underrun, mientras que un búfer marcado como terminado no es una ausencia
temporal. El puerto `SampleInput` y el planificador de búferes corresponden a
ese comportamiento. Los controles de arranque y pausa conservan sus acciones
y se conectan al reproductor. El seek antes del arranque, usado por los
ambientes de `SoundManager`, también está contrastado. Evidencia x86 aislada:

- **2.400 casos** del callback y **36 secuencias/216 llamadas** de underrun,
  recarga y EOF; **32 secuencias/256 operaciones** de disponibilidad y carga.
- **48 casos** de arranque; las transiciones de pausa del bloque30 siguen
  vigentes. Tres secuencias adicionales usan HSTREAM, callback de sample y
  `Mp3dec.asi` originales juntos: **121 lecturas**, repetidas con pausa y
  reanudación. Coinciden los estados de control y PCM del reproductor Rust.
- El seek antes de arrancar con HSTREAM, sample y proveedor originales cubre
  **12 secuencias y 277 lecturas**, incluidos destinos intermedios y EOF.
- El mismo recorrido completo con `Gebaeude/kontor.mp3` alcanza **226 lecturas**
  hasta EOF. Coincide tanto el reproductor como la salida del mezclador
  `NativeEffects`; [informe de procedencia](evidence/hstream-sample-commercial.json).
  La captura con PCM comercial queda en `artifacts`.
- Las pruebas de la API cubren mezcla mono, pausa, reanudación, seek estando
  pausado, reinicio tras EOF y cierre. Los errores de capacidad permanecen
  explícitos.

**Límites:** el archivo está ya en memoria y el driver físico se simula. Los
underruns se contrastan en el callback y su recarga, no con un hilo asíncrono
ni con el comportamiento integral del dispositivo en tiempo real. El lector
nativo solicita bytes según su propia necesidad; no afirma idéntica secuencia
temporal de callbacks del proveedor original. El mezclador admite la misma tasa
que su salida, sin remuestreo. La duración usa la tasa de la primera trama y
es una estimación para archivos VBR; los seek previos y posteriores al
arranque se cotejaron en el dominio de las capturas. Los guards MP3 previos y la falta de validación en juego siguen
vigentes. Siguiente: entrada asíncrona y política integral de underrun, reloj
HSTREAM y salida física; después una sesión sin EXE ni DLL del motor.

```powershell
python reimpl/oracle/x86_sample_input_callback.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --check
python reimpl/oracle/x86_sample_buffer_ready.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --check
python reimpl/oracle/x86_sample_refill.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --check
python reimpl/oracle/x86_hstream_start.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --check
python reimpl/oracle/x86_hstream_sample_real.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --plugin D:/AI/Patrician3/games/iso-english/app/Win32/Mp3dec.asi --check
python reimpl/oracle/x86_hstream_sample_pause.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --plugin D:/AI/Patrician3/games/iso-english/app/Win32/Mp3dec.asi --check
python reimpl/oracle/x86_hstream_prestart_seek.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --plugin D:/AI/Patrician3/games/iso-english/app/Win32/Mp3dec.asi --check
python reimpl/oracle/x86_hstream_sample_commercial.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --plugin D:/AI/Patrician3/games/iso-english/app/Win32/Mp3dec.asi --input D:/AI/Patrician3/games/iso-english/app/Musik/Gebaeude/kontor.mp3 --fixture artifacts/hstream-sample-commercial.json --report reimpl/p3-audio/evidence/hstream-sample-commercial.json --check
$env:P3_MP3_ROOT='D:/AI/Patrician3/games/iso-english/app/Musik'
$env:P3_HSTREAM_SAMPLE_FIXTURE=(Resolve-Path artifacts/hstream-sample-commercial.json).Path
cargo test --release --offline --locked --manifest-path reimpl/Cargo.toml -p p3-audio --test progressive_player -- --include-ignored
cargo test --release --offline --locked --manifest-path reimpl/Cargo.toml -p p3-audio --test sample_input_callback
```


### Bloque 33 — entrada asíncrona y falta temporal

`Mp3Backend::AsyncNative` es una opción explícita en `NativeEffects` y
`OfflineDevice`; Symphonia predecodificada sigue predeterminada. `ResourceStream`
abre un archivo suelto, memoria o una región limitada de CPR. Se sondean hasta
4096 bytes **síncronamente al abrir**, antes de arrancar; ID3 o cabecera más
allá del prefijo no están soportadas. Un hilo lector alimenta una cola acotada
(tres bloques de 4096 bytes en el adaptador); no precarga el archivo completo.
Los errores y el EOF solo se notifican después de entregar los bytes pendientes.

La ruta de lectura usa `try_lock` para obtener lo disponible, sin esperar una
lectura de archivo. Si no hay suficientes bytes, entrega el PCM disponible y
rellena el resto con silencio; conserva el stream, el estado del decoder y el
fragmento comprimido. El cursor avanza solo por PCM producido, no por el silencio;
`async_stream_underruns` registra llamadas, fotogramas silenciosos, faltas del
callback y contenciones. En pausa el productor sigue precargando hasta el límite.
Un seek de control invalida la generación anterior, incluso si la lectura tarda;
el cierre no espera a una E/S bloqueada, cuyo hilo termina cuando esa E/S vuelve.
El bucle al consumir EOF utiliza un seek no bloqueante. Los métodos de control
pueden tomar el cerrojo, la mezcla y el decoder reservan memoria y **no se
promete tiempo real duro**. El cierre no puede cancelar la E/S del sistema.

Evidencia y límites:

- Un lector deliberadamente bloqueado y cuatro llamadas separadas 10 ms dan
  silencio sin esperar al lector, conservan pausa y recuperan el PCM de la
  captura x86. Se comprueban además EOF, errores tras datos ya almacenados,
  seek que invalida lectura en curso y cierre con el lector bloqueado.
- El adaptador lee un archivo en disco y el PCM recuperado coincide con la
  captura x86. El MP3 comercial `Gebaeude/kontor.mp3` desde disco coincide con
  las **226 lecturas** originales acumuladas tras recargas. También se contrastó
  un seek previo al arranque con x86 y un bucle finito con el puerto progresivo
  anterior; no acreditan todas las intercalaciones posibles.
- Nuevo oráculo aislado ejecuta juntos el callback sample y `Mp3dec.asi` x86:
  **9 secuencias/18 llamadas** con 64, 128 o 256 bytes iniciales. En las nueve,
  con el callback marcado como sin datos, el proveedor devuelve 1024 bytes PCM;
  en ocho difieren del PCM continuo. La política Rust de silencio y espera es
  deliberadamente distinta: **no hay paridad de underrun con el original**.
  La captura sintética es `tests/fixtures/sample_asi_underrun.json` SHA-256
  `dcc1badbfa6069935f94c7100b62962165b290cefe13191d83fc029c6801aa52`.
- Es una prueba de hilos y tiempos de pared con salida simulada, no un callback
  de hardware ni una prueba del juego. No acredita un calendario idéntico al
  servicio HSTREAM real, todos los seek/bucles ni archivos VBR. Siguen los guards
  del decoder y el límite de frecuencia sin remuestreo de los bloques previos.

Siguiente: investigar y decidir si reproducir los resultados del original ante
entrada incompleta; ampliar los contrastes asíncronos de HSTREAM, seek y bucle;
después conectar reloj, dispositivo físico y sesión sin EXE. No declarar paridad
completa hasta contrastar esos caminos y la futura QA humana en juego.

```powershell
. D:/AI/Patrician3/scripts/Enter-P3Environment.ps1
python reimpl/oracle/x86_sample_asi_underrun.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --plugin D:/AI/Patrician3/games/iso-english/app/Win32/Mp3dec.asi --check
cargo test --release --offline --locked --manifest-path reimpl/Cargo.toml -p p3-audio --test async_input --test async_playback
$env:P3_MP3_ROOT='D:/AI/Patrician3/games/iso-english/app/Musik'
$env:P3_HSTREAM_SAMPLE_FIXTURE=(Resolve-Path artifacts/hstream-sample-commercial.json).Path
cargo test --release --offline --locked --manifest-path reimpl/Cargo.toml -p p3-audio --test async_playback asynchronous_disk_mp3_matches_commercial_x86_pcm_after_refills -- --ignored
```


### Bloque 34 — primera trama corta, seek y bucle asíncronos

El callback original puede leer menos datos de los solicitados y aun así entregar
PCM: en los casos estudiados procesa una trama a partir de los bytes disponibles,
en lugar de esperar su resto. Añadida política **experimental y optativa**:

```rust
use p3_audio::native::async_playback::UnderrunPolicy;
backend.device.set_async_underrun_policy(stream, UnderrunPolicy::OriginalFirstFrameGuarded)?;
```

Se aplica solo a la primera trama, cuando hay falta real de entrada y ya se
pueden leer cabecera e información lateral. El decoder trabaja en una copia con
el resto de la trama y hasta 4096 bytes adicionales modelados como ceros; si sus
guardas rechazan los datos, conserva el estado y vuelve a esperar. Una contención
del cerrojo no se interpreta como falta de entrada. Si acepta, consume el
fragmento disponible, mantiene el PCM pendiente y se resincroniza al recibir
más bytes. El contador `original_short_frames` registra los casos aplicados.
`WaitForInput` sigue siendo la política predeterminada. El hash del script normaliza CRLF a LF para permitir el contraste en Windows.
El relleno cero es un
modelo contrastado en el dominio indicado, no una prueba de toda memoria o
planificación posible del proveedor original.

La nueva captura ejecuta callback sample y proveedor originales juntos con tres
MP3 sintéticos, tres prefijos (64/128/256 bytes), recarga y hasta 24 llamadas:
**9 secuencias/165 llamadas**, con PCM y final observado coincidentes en el
puerto experimental. SHA-256 de `sample_asi_underrun_extended.json`:
`8ced5ff5f20b28052d91c6238979f732add9bab2a1745dffe17e1a9019a1ddbe`.
Cabecera incompleta y errores siguen la política de espera; las lecturas de tabla
fuera de rango se rechazan. No se acreditan underruns de tramas posteriores,
recargas múltiples, secuencias comerciales ni todos los estados de memoria.

Los seeks asíncronos se comparan con las capturas anteriores: **12 secuencias/
277 lecturas** antes del arranque y **12/325** en marcha, para tres MP3 y cuatro
posiciones. Una prueba adicional desplaza el stream mientras el productor tiene
una lectura bloqueada y compara después el PCM con x86. Los retardos son
controlados y el driver se simula.

El nuevo oráculo de HSTREAM y proveedor x86 recorre el archivo completo una o
dos veces: **6 secuencias/352 lecturas**. Las tres secuencias sin bucle
coinciden. En las tres con bucle, el primer PCM diferente aparece en las
llamadas **9, 11 y 13**, contando desde cero, para los casos 1, 5 y 7. El
HSTREAM original envía `-2` durante la precarga antes de que se agote el sample;
`AsyncPlayback` espera a consumir el final. La prueba registra esa diferencia;
**no se ha resuelto ni se afirma paridad de bucle**. La captura
`hstream_progressive_full_loop.json` tiene SHA-256
`08c6a50fd924453c3efad8e7bfeaab8ceb60cf27e725b8e2d1b8ec53a2cf50a9`.

Siguiente: conectar la entrada asíncrona al servicio HSTREAM ya portado para
controlar el instante de `-2` sin perder las generaciones de seek; contrastar
orden de entrega, PCM pendiente, pausa y múltiples bucles, luego ampliar la
política de underrun con datos reales y tramas posteriores. No se ha probado el
dispositivo ni el juego y no se promete tiempo real duro.

```powershell
. D:/AI/Patrician3/scripts/Enter-P3Environment.ps1
python reimpl/oracle/x86_sample_asi_underrun_extended.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --plugin D:/AI/Patrician3/games/iso-english/app/Win32/Mp3dec.asi --check
python reimpl/oracle/x86_hstream_progressive_full_loop.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --plugin D:/AI/Patrician3/games/iso-english/app/Win32/Mp3dec.asi --check
cargo test --release --offline --locked --manifest-path reimpl/Cargo.toml -p p3-audio --test async_input --test async_playback
```


### Bloque 35 — productor asíncrono unido a HSTREAM

`Mp3Backend::AsyncNativeHstream` selecciona la nueva ruta en `NativeEffects`;
la opción predecodificada sigue predeterminada y `AsyncNative` conserva el
comportamiento anterior. `AsyncHstream` conserva el avance de una llamada de
servicio cuando el productor aún no tiene datos o su cerrojo está ocupado. Un
fragmento temporalmente insuficiente no se convierte en EOF ni en fallo. Al
cruzar el límite de bucle, ordena un seek de nueva generación y aplica `-2` al
decoder durante la precarga, antes de que se agote el PCM ya pendiente.

El servicio termina antes de entregar sus búferes al sample. La llamada de
mezcla puede devolver silencio temporal si todavía está esperando; `service` y
`async_stream_service_ready` permiten coordinar explícitamente el servicio con
el siguiente callback. Esta coordinación se usa en los contrastes siguientes;
no prueba todos los posibles calendarios de hilos. El adaptador de `NativeEffects`
usa tres búferes de 4096 bytes y consume la entrada del sample byte a byte para
conservar su posición frente al callback original. Falta perfilar y optimizar
esa lectura sin alterar el momento del reinicio. Sigue sin prometer tiempo real
duro ni salida física.

Evidencia x86 aislada, sin cargar DLL en Windows ni abrir el juego:

- HSTREAM y proveedor originales con sample simulado: **18 secuencias y 1.064
  lecturas**, con una, dos y tres pasadas. Coinciden los bytes entregados, estado,
  señales `-2` y PCM, incluido el pendiente tras el reinicio.
- HSTREAM, callback sample original y proveedor juntos: **21 secuencias y 1.784
  lecturas**, capacidades 1024 y 4096, una a tres pasadas y tres secuencias de bucle infinito (120 lecturas cada una).
  Coinciden estados
  del sample al inicio de cada ciclo, momento de `-2` y PCM. El puerto lee un
  byte por vez y no reproduce las longitudes de cada llamada al callback original.
  La configuración de 4096 también se prueba
  a través de `NativeEffects` y el mezclador, con tres pasadas. Los oráculos
  sintéticos `hstream_sample_real_loops.json` y `..._4096.json` tienen SHA-256
  `cbbfbf0d65823dc5c41c6ce082d3e765dc3b48b08b02b0c5ae0151137891d5fd` y
  `7065da3cb57605dad8e3edea63999a8303bc96fa9fa7512368c70cec0653fc07`;
  el de bucle infinito es `37ad4b605bc56bf7a2adf0cda8e5b5b18cab5226b846df5723f035661eca1a68`.
- El efecto comercial `Gebaeude/kontor.mp3` desde disco coincide en **226
  lecturas** sin bucle y **2 secuencias/1.073 lecturas** con dos y tres pasadas;
  la captura comercial está solo en `artifacts`, el informe de procedencia en
  [evidence](evidence/hstream-sample-commercial-loops.json).
- Los seeks previos y activos vuelven a cotejar **24 secuencias/602 lecturas**.
  Hay pruebas adicionales de pausa, seek mientras una lectura está bloqueada,
  reanudación del servicio y conservación del PCM pendiente con el productor
  bloqueado justo después de `-2`.

**Límites:** se cubre archivo normal, origen cero y padding de bucle/fin cero;
se rechazan otras configuraciones del adaptador. La opción experimental de
underrun de primera trama del bloque34 y sus guards siguen limitados a su dominio.
El servicio puede tener que esperar sin bloquear al productor antes de entregar
una carga; falta contrastar retardos variables, underrun durante más tramas,
relojes y salida física, y medir el coste del consumo byte a byte. No se ha
validado el audio en partida ni toda la integración del juego.

```powershell
. D:/AI/Patrician3/scripts/Enter-P3Environment.ps1
python reimpl/oracle/x86_hstream_progressive_three_loops.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --plugin D:/AI/Patrician3/games/iso-english/app/Win32/Mp3dec.asi --check
python reimpl/oracle/x86_hstream_sample_real_loops.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --plugin D:/AI/Patrician3/games/iso-english/app/Win32/Mp3dec.asi --check
python reimpl/oracle/x86_hstream_sample_real_loops_4096.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --plugin D:/AI/Patrician3/games/iso-english/app/Win32/Mp3dec.asi --check
python reimpl/oracle/x86_hstream_sample_real_infinite_4096.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --plugin D:/AI/Patrician3/games/iso-english/app/Win32/Mp3dec.asi --check
python reimpl/oracle/x86_hstream_sample_commercial_loops.py --dll D:/AI/Patrician3/games/iso-english/app/mss32.dll --plugin D:/AI/Patrician3/games/iso-english/app/Win32/Mp3dec.asi --input D:/AI/Patrician3/games/iso-english/app/Musik/Gebaeude/kontor.mp3 --fixture artifacts/hstream-sample-commercial-loops.json --report reimpl/p3-audio/evidence/hstream-sample-commercial-loops.json --check
$env:P3_MP3_ROOT='D:/AI/Patrician3/games/iso-english/app/Musik'
$env:P3_HSTREAM_SAMPLE_FIXTURE=(Resolve-Path artifacts/hstream-sample-commercial.json).Path
$env:P3_HSTREAM_SAMPLE_LOOPS_FIXTURE=(Resolve-Path artifacts/hstream-sample-commercial-loops.json).Path
cargo test --release --offline --locked --manifest-path reimpl/Cargo.toml -p p3-audio --test async_hstream -- --include-ignored
```

## Bloque 36 — mezclador en tiempo real para el cliente (14/11/2026, nube de Claude)

`live.rs` es la salida que faltaba: `Mixer` recibe las órdenes `AIL_*` que el enlazador traduce en `p3-mvp/src/ui_rt/miles.rs`
y `p3-client/src/audio.rs` lo saca por la tarjeta. Usa `native::stream::decode_stream` para WAV/IMA y Symphonia para MP3
progresivo en un hilo. Es un contrato propio, no la mezcla Q11 de los bloques 14–35 (que siguen sin conectarse al cliente).
Tres pruebas unitarias (`cargo test -p p3-audio --lib live`). Detalle, evidencia y límites en
[SONIDO_CLIENTE.md](../SONIDO_CLIENTE.md).
