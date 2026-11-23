# C10: misión del alcalde y reloj

## C10: actualización decenal, trabajo en curso

El orden y las condiciones de `0x03` coinciden en 512 escenarios con la rutina
original, interceptando los servicios a los que llama. También se reconstruyó
el emisor de investigaciones `0x004DCD90`: 1.024 casos x86 comparan el cursor,
el generador CRT y la operación `0x81`. El orden y el emisor todavía no están
conectados al reloj: faltan los efectos reales de sus demás servicios, entre
ellos capitanes, eventos urbanos y recuento naval. La tarea sigue pendiente.

La purga de buzones `0x004D6780` compara 216 combinaciones de conservación,
tipos y adjuntos con el depósito y serializador x86. La sesión puede purgar un
comerciante y guardar y recargar el resultado; el disparo decenal aún no está
conectado. Las cadenas o fechas inválidas se rechazan sin modificar el buzón.

El recuento de carga entrante reutiliza la rutina `0x00504970` de C07 con la
flota viva de la sesión. Un guardado local reconstruido coincide con el x86 y
las cantidades se conservan al recargar; rechaza cargas desincronizadas. Este
servicio tampoco se ejecuta todavía desde `0x03`.

Los eventos decenales de capitanes `0x004DCEA0` coinciden en 384 escenarios
x86 para el orden, el CRT, los indicadores y las operaciones `0x12`, `0x13`,
`0x67` y la programación `0x27`. Sus efectos están interceptados: faltan sus
manejadores vivos y persistencia antes de conectar este servicio a `0x03`.

La operación `0x12` ya actualiza las habilidades y sueldo en la flota viva,
sincroniza las vistas y entrega la carta `0x4D`. Se contrastaron 1.024 casos
x86 y una variante sintética de guardado con aviso y recarga S05; rechaza vistas
divergentes sin consumir la operación. La ejecución directa desde los eventos
decenales y los otros efectos siguen pendientes.

La operación `0x13` marca la jubilación, programa `0x27` y entrega la carta
`0x52`; 512 casos contrastan el estado y la cola x86 y una variante sintética
con flota viva comprueba el guardado y la recarga. La resolución `0x27` compara
504 escenarios x86 con la disolución de convoy interceptada. El despachador
conecta las esperas y la liberación sin disolver convoy, contrastadas con su
serializador y recarga S05; si necesita disolverlo se detiene con ese límite.
La búsqueda que dejaría el original en un bucle se rechaza sin cambios.

La operación `0x67` actualiza la habilidad y el sueldo del administrador de
oficina y entrega la carta `0x7C`: 512 escenarios x86 y una variante sintética
de guardado verifican el efecto y la recarga. La carga S05 reconstruye también
el sueldo y la experiencia del administrador para el comercio de oficina.

## C10: llegada de especias

La tarea `0x00` actualiza las existencias y el contador de llegadas, calcula
el siguiente destino y vencimiento y, en modo de red, publica la operación
`0x83` que programa la siguiente llegada. Usa coordenadas de StadtDaten y el
estado CRT suministrado a la sesión. El generador y el estado coinciden en
512 escenarios x86; tres cadenas contrastan reloj, operación, cola y recarga
S05 para modo local, anfitrión y cliente, con otras fases interceptadas. Los
modos son entradas explícitas; no se ha probado el transporte de red ni una
partida. Faltando recursos el despacho se rechaza sin aplicar la llegada.

## C10: tareas pendientes del planificador

Una tarea conocida que todavía no tiene implementación conserva su posición
en la cola y detiene el avance bombeado con su código en el error; el tick que
la alcanza ya ha ejecutado fases previas y se comunica como parcial. Los tipos
mayores que `0x39` se eliminan como en el original: contrastados tres valores
con el reloj, el despachador y el serializador x86. Los demás tipos todavía
pendientes deben implementarse; este aviso no completa sus comportamientos.

## C10: renovación de préstamos

La tarea `0x19` genera operaciones `0x7B` con el estado CRT compartido; el
bombeo las aplica a las seis ranuras de las ciudades y S05 las conserva. El
estado CRT y los bancos de nombres deben suministrarse desde la sesión: el
guardado original no incluye la semilla CRT. Faltando datos, o si una imagen
incoherente dejaría el bucle original sin salida, se rechaza la tarea sin
cambiarla. El generador coincide en 1.024 casos x86; otros 320 contrastan
renovación, cola, RNG y aplicación original de `0x7B`, incluidos cliente de red y
ocupación variable. Cuatro cadenas pasan por reloj, operaciones y recarga S05;
otras fases se interceptan. No es validación en partida.

## C10: reservas periódicas de marineros

La tarea `0x1A` recorre los comerciantes activos y actualiza sus reservas por
ciudad cada 64 ticks; el reloj, ambas vistas del comerciante y S05 conservan el
resultado. El incremento usa la reputación anterior al límite de 20 y la suma
del byte desborda como en x86. Once escenarios con dos comerciantes y hasta
40 ciudades encadenan tres ejecuciones y la tarea `0x37`, comparan el reloj,
el planificador y los serializadores originales, y recargan S05. Las demás
fases, incluido el día de comerciantes, están interceptadas en el contraste.
Las cifras de comerciantes o ciudades fuera de la imagen admitida detienen la
tarea con error. Sin validación en partida.

## C10: limpieza programada de indicador del comerciante

El reloj despacha la tarea `0x37`: limpia en `merchant+0x608` el bit indicado,
con el desplazamiento enmascarado a 31 como en x86, y retira la tarea. Las dos
vistas del comerciante y S05 conservan el resultado. Nueve valores de bit,
incluidos 31, 32, 63 y `0xFFFFFFFF`, coinciden con el reloj, el despachador y
los serializadores originales en x86 aislado; otras fases están interceptadas.
Un índice de comerciante inválido deja la tarea en cola y registra el error,
en lugar de reproducir un acceso fuera de memoria. No se ha probado en partida.

El avance normal de `p3-sim` ejecuta el día del comerciante y puede preparar las misiones del alcalde con los 40 puntos y regiones de `scripts/StadtDaten.ini`. La configuración se carga desde el CPR suministrado. El ajuste de bandas procede de S0+0x17 y los datos de banda de S02. Sin el recurso requerido el avance informa del error, no inventa posiciones.

La preparación 0x005326E0, su selector 0x00532E30 y las operaciones 0x85 y 0x86 tienen contrastes x86. La preparación con bandas incluye los efectos sobre su estado; S05 conserva esos cambios. Los destinos y estados proceden de la imagen cargada y deben sincronizarse con la IA pirata viva cuando C07 la conecte.

La tarea 0x32 incluye espera y asignación, inicios 1, 2, 3 y 4, esperas 0/2/4 y resolución exitosa 0/2/4. La resolución crea carta global 0x6F, carta directa y aviso para humanos; las peticiones de ventana quedan como efectos del anfitrión. Hay fases de inicio, espera, fracaso y resolución que aún rechazan el despacho: no son paridad completa. Las tareas pendientes de otros sistemas tampoco equivalen al juego completo.

Las ejecuciones x86 son aisladas y no abren el juego. Los casos abarcan 20 cadenas de ascenso hasta el planificador, 160 preparaciones con bandas, 160 sin bandas, 512 consultas de validez, 512 asignaciones, 512 esperas iniciales, 512 inicios, 512 inicios terrestres, 512 inicios de banda, 512 esperas asignadas, 512 cierres y 512 recorridos de cierre. Las cartas globales se contrastan en 400 casos y las directas en 1.408. Algunos contrastes interceptan la presentación o la cola: sus pruebas lo indican. La prueba de 30 días sobre el corpus confirma determinismo, no comparación completa con el original.

## Pausa: fracaso y cierre de misiones

La tarea 0x32 contempla el vencimiento asignado de IA y de los cinco tipos humanos: penalización de reputación, carta 0x70 y aviso; el tipo terrestre añade una tarea secundaria. La limpieza del tipo 5 respeta el estado de la ventana original. El reloj y S05 conservan tarea, carta y reputación. El vencimiento terrestre desbordado reproduce la selección de otra cabeza y destinatario observada en el original, siempre que el destinatario exista; la variante completa en reloj y S05 aún no se ha contrastado. Los eventos de ventana aún requieren conexión visual.

Se contrastaron 512 casos aislados x86 por cada frontera: fracaso de IA, fracaso humano tipos 0/2/3/4, fracaso terrestre con tarea adicional y limpieza de tipo 5; se interceptan las funciones de presentación y entrega de carta. Pasan 22 pruebas de comerciante con el corpus local, incluidas persistencia y recarga de la tarea adicional. No es un contraste integral de 30 días ni una validación en juego.

El crecimiento y el desbordamiento de la tabla se contrastaron en 128 casos x86 adicionales cada uno; en la variante desbordada se variaron los bytes reutilizados de la ranura. El desbordamiento cambia la tarea y el destinatario; la presentación y entrega se interceptan en el oráculo.

Siguiente paso: contrastar la variante desbordada encadenada con reloj, cola de cartas y S05; portar el inicio de asentamiento tipo 0, seguimiento y resolución de tipos 1/3, y conectar eventos visuales y estado pirata vivo con C07 sin interferir con Claude.

## Progreso de ruta terrestre

La fase 1 consume los cuatro recursos en unidades 3/4/1/2, avanza su fecha y prorroga el plazo solo si quedan suficientes recursos para las etapas pendientes. Coincide en 1.024 escenarios x86 con 2–19 etapas, con y sin recursos y con plazos próximos; reloj y S05 se comprobaron con un guardado real. La última etapa crea dos instalaciones cuando las casillas son independientes de los bytes iniciales de pila; las variantes ambiguas detienen el despacho. La espera de las instalaciones y el cierre cuando ambas existen están conectados al reloj: 512 casos x86 con la rutina de cierre interceptada y prueba de carta y recarga S05.

## Última entrega terrestre y obras

La última entrega y las dos obras se compararon en 128 escenarios x86 con coordenadas de pila controladas y con las funciones originales de búsqueda y construcción. El original lee coordenadas locales sin inicializar; el simulador solo despacha si todas las coordenadas iniciales válidas eligen la misma casilla en cada ciudad. La comprobación evita suposiciones, pero puede ser costosa y rechaza variantes que el original ejecuta. El puerto con coordenadas explícitas cubre las 128 variantes; el criterio estable acepta 18. Los objetos de mapa visual se omiten (escena inactiva). Una prueba con dos planos sintéticos comprueba el reloj y la persistencia de ambos en S05; no es una cadena completa x86 hasta los serializadores.

## Entrega de materiales 0x3B

La operación 0x0053A3E0 toma materiales de la oficina y después de la cadena de barcos del comerciante, con lotes y condiciones del original; actualiza el contador empaquetado de la misión y recalcula la carga del barco. El despacho procesa la operación y S05 conserva existencias y tarea; se rechaza una flota con imágenes y cargas desincronizadas. El cálculo se comparó con 512 ejecuciones x86 con búsqueda de oficina, comerciante y recálculo naval interceptados; el recálculo naval tiene pruebas propias. Dos escenarios S05 cubren oficina y barco. El botón de interfaz y la cadena completa con obras aún no se han contrastado.

## Aceptación de misiones 0x4B

La operación 0x0053B440 comprueba rango, disponibilidad, categoría y que el comerciante no tenga otra misión, y arranca inmediatamente los tipos 1–4. La fundación tipo 0 aún se rechaza de forma transaccional. Se compararon 1.024 decisiones x86 con el arranque interceptado y 512 cadenas originales de aceptación e inicio terrestre, incluidas cartas y banderas. Reloj y S05 se probaron con un guardado real para la ruta terrestre. La interfaz todavía debe emitir esta operación.

## Seguimiento y finales de misión pirata

La tarea 0x32/3 comprueba el destino de la banda, programa la siguiente consulta por distancia cuando no hay encuentro, y gestiona la desaparición de banda, éxito y abandono, con indicador de comerciante, cartas y avisos según corresponda. Se contrastaron 512 casos x86 de seguimiento sin encuentros y 512 de desaparición y finales; en estos últimos se interceptan el cierre y los avisos (se comparan solo sus argumentos bajos acreditados). Cuatro estados de final y dos de seguimiento se probaron con reloj y recarga S05. La aparición de piratas, contacto, combate y estado vivo de bandas siguen pendientes; los destinos todavía proceden de la imagen cargada y no de la IA C07.

## Cadena terrestre x86

Una misma ejecución x86 acepta la misión, entrega los cuatro materiales desde una oficina, inicia ambas obras, las completa en 20 pasadas y cierra la misión. Coinciden tarea, existencias, registros, ciudades, comerciante y cargas de cartas con Rust en 16 ticks de un mismo escenario sintético con planos de una casilla. Se interceptan la búsqueda de oficina, las cartas, el aviso y el recuento urbano; no intervienen los serializadores x86 ni el reloj continuo. S05 tiene pruebas separadas de sus etapas, no una comparación integral de la cadena.

## Fundación: inicialización del concejo

Portada la rutina 0x005289B0: genera nombres y candidatos, evita colisiones entre ciudades según los índices y límites reales, fija cuentas e impuestos y conserva dos particularidades del original en la segunda lista. Coincide byte a byte en 512 ciudades x86 con semillas, pasos, bancos y ciudades vecinas variados. Se admiten bancos de 8 a 256 nombres; casos fuera de ese rango están pendientes. La rutina todavía no está conectada a la creación de la ciudad ni al guardado.

La fundación dispone además de valores predeterminados 0x545B20, niveles del astillero 0x545BE0 y estado de ciudad fundada 0x5458D0 en modo 0, con entradas de suministro enlazadas por orden lógico y recuento de calles. Cada rutina coincide byte a byte en 512 ejecuciones x86; el asignador de las entradas y memset están interceptados en el oráculo. El modo de partida nueva, capitanes, carga del plano, crecimiento del mundo y S05 quedan pendientes de integración.

El alta de capitán de taberna 0x526A50 incluye reserva, ampliación y salario; coincide con 512 escenarios x86, con asignador interceptado. El reinicio de edificios 0x51E8D0 coincide en otros 512, con la carga del plano interceptada; la lectura nativa del plano existe en `planlife.rs` y usa archivos `iso/towns/stadtplatzNN.tow` con alternativa `stadtplatz00.tow`. Aún falta unir estas piezas a la fundación y serializar la ciudad y capitanes nuevos.

El simulador carga ahora el banco de planos TOW al configurar los recursos CPR; usa el archivo pedido o el plano 00 alternativo y rechaza dimensiones truncadas antes de modificar la ciudad. Se comprobaron precedencia y alternativa con CPR sintéticos y 40 índices con el CPR inglés local (algunos pueden resolver al 00), además de las pruebas previas x86 del deserializador. La fundación aún no invoca el banco ni amplía el guardado.

S05 puede construir un `TownModel` desde ciudad, plano y entradas vivas. Coincide byte a byte con 128 serializaciones x86 sintéticas; el serializador de edificios se ejecuta por separado y se inyecta en el escritor de ciudad, que recorre nodos reales reconstruidos. La ampliación de la lista de ciudades, matriz, asedios, nombres y capitanes todavía no está conectada.

La longitud de ruta 0x516930 coincide en 1.024 casos x86, incluidos desbordamientos y rutas nulas. La ampliación 0x531BD0 preserva la matriz anterior y coincide en 128 escenarios con el planificador interceptado; la nueva ruta real aún debe conectarse. El constructor y serializador del asedio coinciden para 40 índices x86. S05 puede añadir un TownModel, su asedio y una matriz suministrada; una partida del corpus se reescribe y recarga conservando las ciudades previas. La fundación completa, las rutas reales, los nombres y capitanes aún no se publican en una transacción conjunta.

El constructor `0x0051B100` se ha portado conservando los bytes que el original deja sin escribir. Sus subconstructores x86 se ejecutaron en 64 escenarios sobre memoria previa variable y con distintos valores de jugador, ciudad inicial y modo; el concejo inicial puede tener 5, 6 o 7 miembros. El puente al planificador también se probó con una ruta calculada en mar libre y reversión ante fallo. Si el planificador deja la salida sin escribir, el llamante original lee una posición de pila sin inicializar; el puente rechaza ese caso, no le asigna una distancia supuesta. Estas piezas aún no están conectadas a la operación de fundación ni se ha contrastado la cadena completa.

La persistencia general de C10 incluye ahora los cambios navales de la instantánea viva (barcos, rutas, convoyes, listas y clima), con detección de conflictos y rechazo de encuentros o navegación interrumpida. Una prueba modifica un barco de una partida real, guarda y recarga, y comprueba reversión ante conflicto; no es un contraste de viaje completo con x86 ni cubre batallas vivas.

Los capitanes de la instantánea naval también se fusionan y serializan en S05; se conservan ampliaciones hechas por la fábrica y se rechazan conflictos. Las pruebas usan el serializador 0x85 previamente contrastado con 1.000 casos x86 y comprueban guardado, recarga y conflictos sobre una partida real. No se ha contrastado una batalla viva completa.

La preparación parcial de una ciudad nueva reúne constructor, concejo, dos capitanes, astillero, plano, suministros, `TownModel` y asedio sin modificar las estructuras de entrada. Una cadena x86 ejecuta consecutivamente las rutinas originales de constructor, concejo, capitanes, astillero y estado urbano, con el cargador de plano interceptado como archivo corto: coinciden ciudad, pool y suministros para ese escenario. El recurso CPR y su alternativa también se prueban en Rust. Aún no se publica la ciudad en el mundo ni se ejecutan nombres, rutas, tareas, operación 0x4B o guardado completo de fundación.

Las tareas de fundación `0x02` y `0x2F` se preparan junto a la ciudad, incluyendo separación de elecciones próximas, fecha y carga. Coinciden en 96 escenarios con el tramo x86 y el planificador original; el mes de la elección se guarda desde 1. La ciudad preparada puede ampliar una copia de S05 con la matriz del planificador y revierte si falta una ruta; la prueba de esta composición usa rutas sintéticas. Aún falta conectar el alta a la operación real, actualizar el planificador de puertos, nombres y publicar la transacción completa.

El alta preparada conecta ahora la expansión real del planificador de puertos con la matriz y prepara una copia conjunta del mundo S05, el planificador de tareas y el pool de capitanes, verificando el estado anterior. Una partida real con CPR inglés se amplía, escribe y recarga por S05 y `Sim`; el mapa de navegación de esta prueba es sintético. Los componentes de ampliación de rutas y serialización tienen contrastes x86 propios, pero la transacción conjunta no se ha comparado todavía con el original. La operación 0x4B aún no publica la ciudad en la simulación viva.

La preparación de misión acepta ahora `0x4B`, deriva de la tarea la operación urbana, asigna el alcalde y programa `0x39` antes de preparar la escritura S05. El inicio coincide en 128 escenarios x86 con la creación urbana interceptada. Se preparó una oferta y un rango de comerciante sintéticos sobre una partida real; la ciudad, el alcalde y las tareas sobreviven a la recarga, con navegación sintética. El despachador vivo aún rechaza fundar; falta conectarlo y contrastar la cadena completa con el original.

El despachador vivo ya acepta fundación 0x4B cuando están disponibles el banco de planos, nombres, posiciones y un planificador completo de puertos. Publica ciudad, tareas, capitanes, puerto y matriz; S05 los guarda y recarga y se comprobó un día posterior con una partida real, oferta/rango y navegación sintéticos. Si faltan recursos deja la operación en cola sin cambios. `AmFluss != 1` alimenta el bit urbano, separado de `Region`: la asignación se comprobó en el ensamblador del cargador 0x51E440 y el alta 0x531D50. No hay contraste x86 integral del recorrido ni garantía de que las tareas posteriores se hayan ejecutado: el informe aún enumera fases inactivas, y la configuración habitual todavía debe construir el planificador completo.

La configuración de mapa carga ahora los tres recursos `navdata`, los rectángulos `StadtSymbolXPos/YPos`, los puertos `AnfahrtXPos/YPos` y los contadores de nombres. Al aceptar una fundación, si no existe tabla completa, se construye para las ciudades actuales y se amplía para el nuevo puerto. Una prueba del despachador usa guardado y CPR ingleses reales, con oferta y rango sintéticos: funda, escribe y recarga; también se prueban archivos parciales y grafos inválidos. El núcleo de construcción de rutas dispone de contraste x86 previo, pero esta ejecución de misión/guardado no está contrastada de extremo a extremo y las fases inactivas siguen pendientes.

Se ha encadenado en x86 `0x4B -> 0x531D50` y los serializadores originales de ciudad, tareas, capitanes y asedio: 32 escenarios sintéticos, con cero o seis ciudades previas, recursos de suministro y crecimiento del pool de capitanes. Los bytes coinciden con Rust en los estados comparados; el contraste detectó que el indicador de despacho debe terminar en 1 y se corrigió. La carga de plano, asignación de nombres, ampliación de rutas y matriz están interceptadas, y no se ejecuta el reloj completo: no es paridad integral ni prueba en partida.

El contraste encadenado se amplía a ocho escenarios con `0x4B -> 0x531D50 -> 0x448040 -> 0x531BD0` originales y sus serializadores de ciudad, asedio, tareas y capitanes. Coinciden con Rust la matriz, las rutas (incluida identidad compartida), puntos de mar, contador, ciudad y registros serializados. El mundo, la rejilla y el grafo son sintéticos; se interceptan la lectura del plano, los nombres y el asignador/liberador, y no se ejecuta el reloj completo. No demuestra paridad en partida.

El contraste de ocho fundaciones ejecuta ahora también el cargador y deserializador x86 del plano y la asignación de nombres `0x5127E0`, además de rutas, matriz y serializadores. El archivo y las cadenas se suministran en fronteras simuladas; el mundo, plano y navegación son sintéticos, y la reserva/liberación sigue interceptada. Se carga `NamenStaedte_eng.txt` del CPR y se resuelven los dos textos por posición del mapa; una ciudad fundada conserva esa correspondencia tras recargar y volver a configurar los recursos. El reloj y la paridad en partida siguen sin contrastar.

La tarea de muralla `0x39` ya se despacha desde el reloj y usa el trazado y colocación nativos. Se compararon 192 decisiones x86 de la rutina original, con la colocación interceptada; el trazado y colocación tienen contrastes x86 anteriores independientes. Una prueba funda una ciudad con guardado y CPR ingleses, fuerza la fecha y los puntos de plano, avanza un tick y comprueba el resultado y la recarga S05. No es un contraste x86 encadenado de reloj, colocación y serializadores; se rechazan planos o listas inválidos. La tarea de hitos `0x2F`, el reloj completo y la validación en partida siguen pendientes.

La lógica de los hitos `0x2F` y la selección de alcalde coinciden en 256 casos x86, con creación de oficina, construcción y cola de operaciones interceptadas. El reloj ejecuta sus ramas sin dependencia pendiente, encola y despacha el nombramiento `0x46` y guarda el alcalde y la tarea en S05; una prueba fuerza mercado y fecha en una ciudad fundada y verifica la recarga. Si falta la oficina del alcalde IA o se debe construir el prestamista, el simulador conserva la tarea y registra el límite: esas dos dependencias aún no están conectadas. Tampoco se ha contrastado el reloj y la persistencia conjuntos contra x86 ni probado en partida.

La dependencia del prestamista ya se conecta: `0x2F` planifica la obra y el despachador ejecuta la operación municipal `0x22` de tipo `0x25`. Coinciden tarea, operación, ciudad, registros y celdas en 32 cadenas x86 sintéticas; solo se intercepta la cola de operaciones y se despacha la operación original en la misma máquina. Una prueba adicional fuerza el hito desde el reloj sobre una ciudad fundada y verifica S05 y recarga. Sigue pendiente la creación de oficina del alcalde IA; el contraste x86 no incluye el reloj ni los serializadores originales y no hay validación en partida.

La fábrica `find_or_create_office` dispone de un puerto para buscar, reservar y ampliar el pool, enlazar ciudad y comerciante, inicializar la oficina y preparar la carta global `0x0D`. Coinciden 64 ejecuciones x86 y el serializador original, con ranura libre, primer pool y crecimiento; se interceptan reserva de memoria y entrega de carta. La fábrica todavía no está conectada al reloj, al pool vivo S05 ni a la entrega de esa carta. Sus entradas son sintéticas y no se ha probado en partida.

S05 admite ahora aplicar la fábrica en una transacción con enlaces, pool y carta global `0x0D`; rechaza cambios concurrentes y revierte si falla la carta. Se fuerza el crecimiento sobre un guardado del corpus y se comprueba su recarga; la entrega de carta coincide en 400 escenarios x86 con reserva y búsqueda de comerciante interceptadas. La transacción se invoca explícitamente y aún no está conectada al reloj ni a los datos vivos de las oficinas; no es una comparación x86 del recorrido integrado ni una prueba en partida.

El reloj conecta la creación de oficina del alcalde IA con el pool vivo, las copias de comercio, cuentas y vivienda, la carta y S05; tras ampliar el pool reintenta la tarea en el mismo tick. En un guardado del corpus se fuerzan fundación, mercado, nombramiento, ampliación, doble guardado y recarga. Una cadena independiente compara 64 ejecuciones x86 de `0x2F -> find_or_create_office -> office_serialize`, con reserva y entrega de carta interceptadas, frente a Rust. No se ha encadenado el reloj original ni el serializador completo de mundo y cola; tampoco se ha validado en partida. Las otras fases inactivas de C07/C10 siguen pendientes.

La cadena se amplía a `advance_time -> handle_scheduled_tasks_tick -> 0x2F -> find_or_create_office`, seguida de los serializadores x86 de oficina y cola: 64 escenarios sintéticos coinciden con el reloj, planificador y fábrica Rust. En 16 de ellos se compara además el puente vivo de C10 (ciudad, comerciante, tarea, oficina serializada, cabeza libre y carta), con y sin crecimiento. Las demás fases del reloj, la reserva y la entrega x86 de la carta están interceptadas; no interviene el serializador completo de mundo ni es validación en partida.

Dos cadenas x86 recorren la fundación y 180 días: la tarea `0x39` traza piezas de muralla y `0x2F` crea una oficina. Coinciden con Rust la ciudad, registros de muralla, comerciante, oficina y sus serializaciones y la cola. El mercado se activa y el alcalde se marca IA de forma controlada tras el primer día; las demás fases diarias y servicios externos se interceptan. El contraste detectó y corrigió una particularidad del original: al crecer el pool, un enlace que era centinela puede pasar a ser un índice válido, afectando la capacidad inicial y los recuentos acotados de la carta. No es una simulación diaria completa ni una validación en partida.

El tick urbano ejecuta ahora el recuento de calles y la construcción municipal de tipo `0x2F` según empleo; ambos actualizan el plano vivo y se guardan en S05. El puente de construcción coincide en los 3.000 escenarios x86 del corpus C05, sin presentación. Una cadena adicional fuerza empleados y un marcador en un guardado real y ejecuta en una misma máquina x86 el reloj, tick de ciudad, oficinas, obra municipal y serializadores; coincide el estado serializado con Rust y S05. El planificador urbano y otros servicios siguen interceptados. Se completaron las 38 pruebas de guardados, incluida la ejecución determinista de 30 días, sin afirmar paridad integral ni prueba en partida.

La rama cívica del planificador `0x51D870` (día del año & 7 igual a 4) se conecta al tick urbano: selección de edificios, búsqueda de solar y operaciones municipales `0x22`, `0x23` y `0x27`. Su decisión coincide en 2.340 casos x86 con solar, CRT y cola interceptados; 64 cadenas adicionales ejecutan la colocación y el despacho originales y comparan ciudad, registros y celdas. Los trazados de muralla no vacíos se contrastaron por separado en el corpus de C05, no en estas cadenas. Se exige un estado CRT aportado por el anfitrión cuando se necesita aleatorio: el guardado no lo contiene y otros consumidores todavía no están todos conectados al mismo flujo. Si falta, se informa del límite. La cola se vacía al final del avance actual: falta contrastar y conectar su cadencia original durante avances de varios días. Las demás fases del planificador siguen inactivas; no se ha contrastado el recorrido completo con reloj y S05 ni probado en partida.

También se portó la decisión de la fase de muralla (día & 7 igual a 0): elige alcalde o comerciante de una oficina y encola una torre o doce tramos. Coincide en 144 decisiones x86, con búsquedas de oficina/comerciante y cola interceptadas. Todavía no está conectada al tick: antes hay que conectar y contrastar el despacho `0x24`, incluida la valoración, cobro, cartas y tareas de la pieza. No implica paridad del planificador ni prueba en partida.

Portada la extracción `0x500A70` de una mercancía entre oficina y barcos: si la oficina no cubre el pedido, primero recorre barcos en esa ciudad con estado menor de 4, incluso estado 2, y redondea los restos al lote; después usa la oficina. Se compararon 600 escenarios x86 con el recálculo de carga interceptado y se protege la transacción ante ciclos de lista. Falta conectar esta rutina al despacho de obra, actualizar y persistir las cargas y contrastar la transacción conjunta; no es paridad en partida.

La extracción dispone ahora de un adaptador vivo que recalcula el barco, marca el convoy y sincroniza oficina y copias de flota para S05. En 128 escenarios sintéticos se ejecutan también el marcado y recálculo x86 originales y coinciden mercancías, pesos, valores y bandera de convoy. Se forzó una retirada en un guardado local y se comprobó su recarga S05. El adaptador aún no está llamado por la operación de obra `0x24`; quedan valoración, contabilidad, cartas y tareas, y no se ha contrastado el recorrido conjunto de reloj y muralla ni probado en partida.

La fase de muralla (día & 7 igual a 0) se conecta al tick y la operación `0x24` ejecuta valoración, materiales, recálculo de barcos y convoyes, contabilidad, final de trazado, carta `0x2F` y tarea de deuda. Coinciden 128 escenarios x86 sintéticos, incluidos cierre con factura y secuencias de doce operaciones; búsquedas de oficina/comerciante, reserva y entrega de carta/tarea se interceptan. Otros 128 casos contrastan `add_message` por separado. Tres escenarios forzados sobre un guardado conectan el despachador vivo y comprueban ciudad, plano, almacén, barco, contabilidad y recarga S05; la carta y tarea se conservan. Pasan 40 pruebas de guardados, incluidas las de 30 días. Aún falta ejecutar en la misma máquina x86 el reloj, planificador, cola y serializadores originales, y aclarar la cadencia del bombeo entre avances. Hay otras fases del planificador y sistemas de C07/C10 pendientes; no se ha probado en partida.

Un escenario adicional forzado sobre un guardado encadena ahora en la misma máquina x86 `advance_time`, tick urbano, `0x51D870`, las doce operaciones `0x24` y serializadores originales; las dos piezas pasan a la cola de obras y se cierra el trazado. Los datos de ciudad, plano, oficinas y S05 comparados coinciden con Rust. La cola de operaciones se captura y despacha en la frontera, y se interceptan otros servicios, reservas e interfaz: no es un contraste del bombeo de fotogramas ni de la simulación completa. La prueba detectó que la búsqueda de oficina debe recorrer la cadena de la ciudad y tomar la primera coincidencia, en lugar de buscar por todas las ranuras; se corrigió. Sin prueba en partida.

El escenario anterior encadena un segundo avance de hasta ocho ticks después del despacho de muralla y vuelve a comparar los serializadores x86 con Rust. El orden de esa frontera coincide para este caso; el bombeo real y su temporización siguen interceptados, y aún no se comparan varios días ni los modos de tiempo con el bucle principal.

El `execute_operations` original confirma, en 18 colas y cinco pasadas por caso, el límite de 52 registros, el orden de un `0xC4` detrás de un paquete lleno y el aplazamiento de operaciones emitidas por manejadores hasta la pasada siguiente. El ejecutor vivo respeta ese límite y conserva el orden pendiente si una operación falla; `persist_all` rechaza guardar una cola pendiente que S05 no serializa. `advance_pumped_until` ofrece pasos explícitos de hasta 256 ticks y no permite adelantar un `0xC4` cuando hay 51 o más entradas; una prueba con un guardado verifica un paquete lleno y un error. `advance_days` y la vía de encuentro utilizan pasos de hasta ocho ticks; al concluir un avance por días se detiene el reloj y se vacían las pasadas pendientes. El oráculo intercepta los manejadores y el avance; esta vía no reproduce tiempo de pared, red ni hotseat. No es paridad completa ni prueba en partida.

El escenario de reloj y muralla compara ahora la vía `advance_pumped_until` con dos avances originales y el despacho intermedio, además de los serializadores. El corpus de guardados avanza 30 días por la nueva vía y terminan 28 pruebas de sesión con los recursos locales; se comprueba también la recarga tras dos días. Los demás servicios se mantienen interceptados en la cadena x86 y no se ha demostrado paridad de esos 30 días ni simulado el combate completo.

Las fases de vivienda del planificador (días & 7 iguales a 1, 3 y 5) ejecutan la elección de solar y la operación municipal; cuando no cabe una casa pueden proponer ampliar la muralla. Coinciden 1.728 decisiones x86 con solar interceptado, 144 propuestas de muralla y 48 cadenas con búsqueda de solar y despacho originales, incluido el estado CRT y la tarea del concejo. Tres escenarios forzados conectan el anfitrión vivo y recargan plano y tarea desde S05. Si la búsqueda necesita `rand`, el anfitrión debe aportar su estado mediante `configure_crt_state`: el guardado no lo conserva y faltan consumidores por integrar en ese flujo. El día 2 del planificador no hace nada en el original. Faltan la fase de pozos (6), la de defensas (7), el contraste de varios días con el reloj original y paridad en partida.

La decisión y elección del solar de pozo (fase 6) coinciden en 192 casos x86 con la cola interceptada. Aún no se llama desde el tick: la operación resultante puede pertenecer a un comerciante y necesita conectar la construcción privada, sus materiales y reputación. La fase 7 sigue pendiente.

La fase de pozos (día & 7 igual a 6) está ahora conectada al planificador y al despacho de obras: contempla propiedad municipal o privada, materiales de oficina y barcos, precio y reputación. La extracción directa del comerciante coincide en 192 casos x86 con recálculo real; 64 inicios privados y 16 municipales contrastan ciudad, plano, oficina, barco, pago y reputación, con búsqueda de objetos y reserva interceptadas. Cuatro casos forzados atraviesan el despacho vivo y recargan S05; se preparan expresamente los objetos sintéticos en el guardado. La decisión y conexión del planificador se contrastan en 192 casos separados y el corpus avanza 30 días. No hay aún una cadena x86 única de varios días ni validación en partida; la fase 7 continúa pendiente.

La decisión de defensa de la fase 7 está portada por separado y coincide en 896 casos x86, con consumidores interceptados: incluye calendario por ciudad, asedio, listas y la prioridad de ciudades hanseáticas con humano local. Aún no se conecta al tick ni se implementan sus cuatro consumidores como recorrido completo.

La fase 7 de defensas se conecta al tick y al despacho `0x22` de tipos `0x30..0x35`, con contabilidad, suministros, barcos y S05. Hay 224 cadenas sintéticas de planificador y despacho en una ejecución x86, con cola, CRT y búsquedas interceptados, y 160 obras aisladas con materiales, pagos y precios activos; el precio usa el almacén después del descuento, como el original; diez escenarios forzados atraviesan el despacho vivo y recargan S05. Los emisores de defensas y la búsqueda de solar reutilizan C05. La búsqueda aleatoria de puertas exige un estado CRT explícito; el guardado no lo contiene, por lo que el avance puede detenerse si falta y la semilla de las pruebas no demuestra equivalencia con la del original. La operación de `0x00524E20` conserva la discrepancia documentada entre el tipo emitido y la pieza buscada. Aún faltan contraste de varios días con el reloj x86 y paridad en partida.

Un escenario forzado enlaza durante ocho días el reloj, el tick de ciudad, la oficina, la obra, el cierre diario y el planificador originales con su despacho y los serializadores x86. En cada jornada coinciden en Rust las operaciones, la ciudad, las oficinas, comerciantes y cartas serializados, la cola de tareas y el estado CRT; se recargan los edificios desde S05. Se ejercitan las obras de muralla, tres tipos de vivienda, un pozo y una mejora de defensa. El escenario aísla una ciudad, intercepta servicios externos y la cola se despacha en la frontera del oráculo; la fase diaria de comerciantes se intercepta en ambos lados. La semilla CRT es explícita y no se conoce por el guardado. No prueba paridad completa ni partida.

Una segunda variante de la cadena de ocho días ejecuta también siete llamadas a `merchant_daily` originales y en Rust: la cantidad de comerciantes activos se fija a uno, las oficinas sintéticas apuntan a la ciudad de prueba y se comparan sus comerciantes, oficinas y cartas serializadas en cada jornada. El resto de límites de la cadena anterior se mantiene; no cubre comerciantes simultáneos, todas las ciudades ni paridad en partida.

Otra variante de la misma cadena activa dos comerciantes y sus oficinas durante ocho días; el ejecutable original ejecuta quince actualizaciones de comerciante y coinciden sus cuentas y serializadores con Rust. Los dos comerciantes no tienen barcos en este escenario; las demás ciudades y servicios externos permanecen fuera del contraste.

La variante con dos comerciantes se repite asignando a cada uno un barco comercial y tripulación: se comprueba que se acumula su coste y que las cuentas y serializadores coinciden durante los ocho días. El tick de movimiento de barcos se intercepta en esta cadena; no se contrasta navegación ni combate.

El CLI acepta `--crt-state` y muestra el valor final para reanudar; rechaza guardar un resultado si el avance terminó con errores. El hash de ciudad y comerciantes omite dos campos temporales que no serializa el original (`Town+0x7A4` y `merchant+0x47C`). En los nueve guardados del corpus local, 8 días y recarga con CRT explícito más otros 8 coinciden en resumen y estado CRT con 16 días continuos. La comprobación no cubre los sistemas no incluidos en el hash; no implica paridad en juego ni persistencia automática del CRT.

El CLI dispone de `--fleet-player ID` con CPR inglés para reconstruir la flota desde S02; reúne barcos o líderes de distintos propietarios y resuelve rutas propias y compartidas con los recursos de navegación. El cambio se publica de forma transaccional y rechaza partidas distintas de la sesión o batallas activas. En un escenario sintético basado en guardado se reconstruyen varios propietarios y el reloj empieza a avanzar; el tick se detiene ante IA atracada aún no conectada. El guardado real `TEST25_ConvoyMar` se rechaza antes de avanzar porque contiene salidas de IA sin ruta serializada. Pasan las regresiones navales x86 anteriores, pero esta reconstrucción multioperador no se ha contrastado como recorrido completo contra x86 ni en partida.


## C10 — rutas de salida y recarga de flota (PC de Sergio, 01/11/2026)

La reconstrucción usa la tabla de rutas ciudad→ciudad, conecta la IA atracada al reloj y a las oficinas, y crea rutas para barcos y convoyes que no tenían descriptor. Se conserva el borrado de rutas en S05 y se reconstruyen líderes de convoyes en espera; se sincronizan existencias de oficina tras el tick urbano. La cola de reparación coincide en 1.000 entradas con el x86 aislado y la salida de convoy con ruta recién creada en 48 casos x86 con el planificador interceptado. Los oráculos se reproducen.

En `TEST25_ConvoyMar`, dos días continuos y uno más recarga más otro coinciden en ciudades, oficinas, comerciantes, imágenes de barcos y convoyes, normalizando punteros de rutas. Una versión anterior del CLI completó 30 días con estado CRT explícito omitiendo parte de los convoyes; esa prueba no demuestra la flota completa. Al incluir los convoyes activos, TEST25 llega a un encuentro pirata tras 714 ticks y se detiene porque el combate aún no está conectado. El circuito completo no se ha contrastado con el original; faltan barcos y convoyes no registrados, efectos, encuentros, combate, IA y persistencia de todas sus variantes. El paso activo se rechaza si crece el pool de convoyes. No se abrió el juego.

La suite antigua `persist_all` presenta dos fallos también reproducidos en `main`: compara sin normalizar `Town+0x7A4`, que el guardado original no contiene, y avanza sin aportar el estado CRT que ahora requieren decisiones de defensa. No se han tratado como regresiones nuevas ni como prueba de paridad.


La búsqueda de presas piratas 0x50D9E0 coincide en 2.000 casos x86; la decisión de ataque 0x515360 en 2.500 casos, con limpieza de ruta interceptada, y se conectan al tick de convoyes. Se integró la velocidad 0x612930 publicada por Claude. La preparación 0x50BC40 del encuentro coincide en 160 casos x86 con orden, copia de ruta, reserva, interfaz y `SeaBattle::start` interceptados; es un puerto aislado, no crea todavía la batalla en la sesión. Se procesan convoyes activos antes omitidos y se detiene ante el primer efecto pendiente, en vez de omitirlo. Sin paridad integrada de flota ni prueba en partida.

## C10 — selección de bandos piratas (PC de Sergio, 01/11/2026)

La construcción aislada de bandos admite participantes especiales 0xFE, convoyes con propietarios mezclados y marcas de los barcos cuyo propietario coincide con el bando contrario. La reserva de dos objetos espaciales por barco se conecta de forma transaccional al pool nativo. Coinciden los objetos locales ordenados, sus enlaces y el número de reservas en 192 ejecuciones x86 del inicio, con asignador y altas de colisión interceptados; los otros efectos del inicio no se comparan. Se cubren propietarios 0xFF y se rechazan listas cíclicas sin alterar el pool.

Solo cubre dos convoyes existentes y la variante de inicio sin omitir barcos del segundo bando. No calcula ni publica por sí sola el estado completo de batalla, no conecta 0x50BC40 al reloj ni S05 y no implica paridad en partida. La siguiente acción es integrar la construcción en SeaBattle::start, probar sus demás campos y efectos con x86, y enlazarla a la transacción del encuentro sin interferir con C07 de Claude.

El prefijo y la clasificación existentes se encadenan ahora con la reserva espacial, la selección y el registro de listas de ambos bandos. El ajuste de `SeaBattle+0x1E94` por barcos descartados y 13 campos de listas, participantes y banderas coinciden en 192 inicios x86 adicionales con valores iniciales variados; se interceptan el asignador, las altas espaciales y la apertura de vista. Este contraste no valida temporizadores, viento, serialización ni la conexión del encuentro al reloj.

La rama con dos convoyes existentes y ambos bandos de IA inicia ahora el objeto nativo completo y entrega la solicitud de apertura al anfitrión. En 192 inicios x86 con propietarios mezclados y distintos fenómenos meteorológicos coinciden los barcos locales, el objeto salvo punteros normalizados, nueve tablas espaciales y los bytes del serializador original; se interceptan la reserva de memoria y la apertura de vista. El capitán de un bando de propietario 0xFF aporta sus dos bytes al objeto, una variante que no cubría el inicio comercial previo. No reserva ni retira convoyes, no publica la batalla en S05 y rechaza participantes humanos; tampoco simula ni resuelve el combate.

La limpieza de referencias de objetivo del jugador `0x50CFB0` se portó por separado: recorre la lista del comerciante y solo borra el primer campo coincidente entre `+0x50` y `+0x52`, incluso si ambos apuntan al objetivo. Coincide en 384 estados x86 y revierte ante listas cíclicas. Aún falta conectar este servicio y la copia de ruta al encuentro vivo.

La copia de ruta `0x5176C0` reproduce las imágenes de barco para rutas ausentes, compartidas y propias en 384 casos x86. Para las propias devuelve explícitamente el descriptor origen y el nuevo: el integrador debe todavía copiar los puntos y hacer transaccional su reserva. `EngageHost` usa por defecto la baja de orden y la limpieza de objetivos nativas; el reloj aún no llama a esa preparación.

La preparación `0x50BC40` se encadena con bajas de orden y objetivos, copia de ruta, inicio de dos convoyes IA y serializador de batalla. En 192 cadenas x86 coinciden imágenes de gestor, barcos, convoyes, llamadas UI y bytes serializados; las rutas están ausentes y se interceptan reserva de ranura, asignador y vista. La copia de rutas propias se expone como efecto pendiente y tiene una prueba de rechazo de alias, pero todavía no se ha contrastado en esta cadena ni publicado en S05. El resultado no está conectado al reloj; se rechazan participantes humanos y barcos objetivo sueltos. Sin partida.

La vía explícita de flota e IA procesa el encuentro de dos convoyes y entrega la batalla al anfitrión; se acepta ranura y clasificación de banda suministradas por este. En 192 cadenas x86 coinciden tick de convoy, preparación, imágenes de gestor, barcos y convoyes, y serialización inicial. S05 valida pertenencia, selección, listas activa/libre y conflictos, publica flota y batalla atómicamente y recarga los registros persistidos en esos escenarios. Se prueban rollback ante conflictos, duplicados y convoy activo. La reserva de batalla y UI x86 se interceptan, y las rutas están ausentes; los descriptores propios y compartidos necesitan contraste integrado. Esta vía no está conectada al reloj normal ni reconstruye la simulación tras cargar. No admite jugador humano ni objetivo suelto; sin partida.

El reloj puede avanzar con la IA naval activa hasta un encuentro de dos convoyes y detenerse antes del gestor de batalla, mediante la configuración explícita de ranura. La clasificación `0x503AC0` consulta la tarea 0x34 del planificador: 512 casos x86 y comprobación de su uso desde flota. En 192 cadenas x86 reloj→convoy→encuentro coinciden gestor, flota y serialización al detenerse; las tareas, clasificación x86, reserva y UI se interceptan y el clima no se activa en ese tick. La fase de clima usa la misma rutina transaccional de la vía comercial cuando corresponde. Todavía no hay reserva automática de ranura, guardado de la sesión durante la batalla, reconstrucción viva, humano ni objetivo suelto; sin partida.

La recarga de la batalla inicial de convoyes reconstruye los barcos locales y las tablas espaciales; coincide con el cargador x86 en 384 casos, con asignador interceptado y punteros normalizados. En el cargador, `+0xAC8` se deriva de los dos bandos y `+0xAB1` vale 2 para 0xFE frente a 0xFF; el inicio escribía 0xFF para esa misma combinación. Se corrigió también el primer campo en la recarga comercial de IA. La reconstrucción todavía no se incorpora a la sesión ni ejecuta un tick de combate; sin partida.

La sesión visual admite guardar un encuentro inicial de convoyes de IA tras completar el tick, siempre que no haya diálogo ni operaciones pendientes; S05 valida y publica flota y batalla juntas. La apertura explícita de batalla en pausa reconstruye también convoyes y comprueba que los miembros y las listas guardadas sean coherentes. Un escenario sintético basado en un guardado local recorre reloj→encuentro→fin de tick→guardado→reapertura en pausa; el bloque inicial coincide con el serializador x86. No se ejecuta el combate, se mantienen bloqueadas las acciones en pausa y no se ha probado en partida.

En `TEST25_ConvoyMar`, el primer encuentro es un convoy pirata contra un barco suelto de IA: la retirada del mar y la creación del convoy coinciden en 192 cadenas x86. La flota del guardado alcanza el encuentro tras 714 ticks y S05 conserva sus rutas al guardar y reconstruir la batalla en pausa; el recorrido concreto de esa partida no se ha comparado completo con x86. El cliente incorpora una opción explícita para activar la flota IA, avanzar hasta el encuentro, completar el tick, guardar y reabrirlo en pausa. Cuatro pruebas Tk ocultas pasan; no se abre el juego original y la simulación de combate sigue pendiente.

## C10 — regresión de treinta días tras reconstruir administradores (PC de Sergio, 02/11/2026)

`two_cities_voyage_with_fleet_api_thirty_days` vuelve a pasar después de reconstruir la experiencia y el salario de los administradores al cargar. La cadena sintética incluye dos ciudades, asedios, oficina, obra, astillero, cartas, flota y viaje humano, con guardado y recarga diarios, y compara fases seleccionadas del reloj, gestor naval y serializadores x86. Otras fases se interceptan: no es una comparación completa de C10 ni una prueba en partida. La ejecución terminó en 227,73 segundos con el ejecutable y los recursos ingleses locales.

La jubilación `0x27` reutiliza la disolución de convoy C07 para un líder en estado 0 o 1 cuando el convoy lleva el bit `+0x14 & 2` y está en estado 0, 1, 4 o 5. La actualización de barcos, convoyes, listas y capitanes se prepara antes de publicarse. Coincide en 48 casos sintéticos con la rutina x86 de jubilación y la disolución originales; otra prueba usa el despachador original sobre una flota derivada de un guardado, fuerza ese bit y verifica la persistencia y recarga S05. Esta última no encadena el reloj ni el serializador naval x86 completo. Se rechazan convoyes cíclicos y las variantes que necesitan reembolsos, reputación u otros efectos, que siguen pendientes; no se ha probado en partida.

También se admite la variante ordinaria sin reembolso: se aplica la reputación de entrega `0x004F9050` al comerciante y se sincroniza con el resto de la transacción. Coinciden 192 combinaciones sintéticas x86, con y sin ese bit y con barcos ajenos. En dos variantes adicionales de un guardado modificado se comparan el despachador, la disolución y la reputación originales y se comprueba la recarga S05; se ha forzado un propietario ajeno para ejercitar el cambio. El recorrido con reembolsos, distancias y cartas aún se rechaza. No se compara aquí el reloj ni el serializador naval completo y no hay validación en partida.

La variante con reembolso entre comerciantes de IA usa ahora la matriz de distancias cargada, actualiza dinero, gastos e ingresos y conserva sus vistas económicas. Coincide con jubilación, disolución y consulta de distancia x86 en 80 escenarios sintéticos y en una variante de un guardado modificado, que además se guarda y recarga en S05. Si se requiere una carta para un humano, la tarea se rechaza sin cambiar la cola, la flota ni los comerciantes; estas cartas siguen pendientes. No se ha encadenado el reloj ni el serializador naval original completo y no hay prueba en partida.

Las cartas humanas `0x31` y `0x32` de esa disolución ya se entregan en S05. En 320 combinaciones se comparan sus datos emitidos, importes y listas con la jubilación y la disolución x86; la entrega `0x004D6530` se intercepta en ese corpus. Otras 192 ejecuciones comparan la entrega y ampliación del depósito originales, con preferencias variadas. Tres variantes de un guardado modificado encadenan despachador, disolución, entrega y serializador de cartas x86, además de guardar y recargar S05. El original reutiliza dos palabras de la carta anterior al emitir ambas; se conserva ese comportamiento. El simulador usa la tabla de retención inicial del ejecutable inglés; falta conectar posibles cambios de esa tabla en tiempo de ejecución. No se compara aquí el reloj ni el serializador naval completo, ni hay prueba en partida.

La fase decenal de capitanes dispone de una preparación transaccional que aplica las mejoras directas de IA y prepara las operaciones y jubilaciones, con estado CRT explícito. Coinciden 192 escenarios con los eventos y las mejoras directas x86; la inserción de tareas y la cola de operaciones x86 se interceptan para comparar sus datos. Rechaza el crecimiento del planificador, una reordenación de la tarea activa y la ausencia de CRT cuando es necesario. Aún no publica los resultados ni está conectada a `0x03`, al reloj o a S05; quedan los demás servicios decenales. No se ha probado en partida.

La vía de cliente de `0x03` ya une purga de cartas, recuento de carga entrante y eventos de capitanes en una transacción. En un guardado modificado se compara tanto el despacho directo como un tick del reloj x86, seguidos de los serializadores originales de cola y cartas y la recarga S05; para el tick se vacían las listas de puerto y mar y se interceptan las demás fases del reloj. Se fuerza la purga de una carta vencida y la jubilación de un capitán de IA; la nueva tarea queda enlazada y coincide la cola serializada. Se corrigió la pérdida de ese enlace al preparar la transacción. Si se alcanza un servicio aún pendiente, la tarea permanece en cola y cada intento de avance informa de que el tick quedó parcialmente ejecutado. La vía ordinaria de partida local sigue necesitando los demás servicios de `0x03`, además del crecimiento del planificador decenal y los casos de capitán compartido con administrador. Al cargar se corrigió la vista pirata: ahora usa los capitanes deserializados, como la fábrica y la flota. Sin partida.

El mismo contraste también pasa para la rama local concreta con `world+0x8A == 1`, fase decenal resultante 2 y día del año al menos 10, usando comerciantes de IA: el original omite en ella los servicios aún pendientes. Tanto el despacho directo como un tick controlado del reloj conservan la cola y recargan S05. No cubre la rama local habitual ni el resto del reloj, y no se ha probado en partida.

El emisor de pasajeros `0x004DC6B0` y la operación `0x7A` están portados: 512 ofertas y 512 despachos coinciden con x86, y el recuento de las cinco bandas coincide en 256 casos. La sesión puede encolar una oferta con la matriz, los barcos de bandas y el CRT vivos, despacharla y guardar la taberna; tres semillas de un guardado se contrastan con emisor, recuento y despacho x86 y se recargan en S05. También se prepara el emisor dentro de la transacción decenal, pero la rama habitual de `0x03` aún se detiene antes en la asignación de plazas sin conectar. Se rechaza una lista de destinos vacía porque el original lee pila sin inicializar. El CRT de una partida guardada debe suministrarlo el anfitrión; no hay prueba en partida.

La asignación decenal de plazas `0x004DC800` y el despacho `0x79` ya están conectados a la preparación transaccional y a la cola de la sesión. Coinciden 512 emisiones y estados CRT y 512 despachos con el x86 aislado, con la cola y el CRT interceptados en la emisión. Tres semillas de un guardado, con regiones del archivo de recursos, encadenan emisor y despacho originales y comprueban la persistencia y recarga S05. Se rechazan más de ocho plazas duplicadas, una tabla de regiones incompleta y los datos que llevarían al original a un bucle; no se altera el CRT al rechazar. La rama local habitual de `0x03` aún no se publica: necesita conectar celebración, peste o incendio y los restantes servicios decenales. Este contraste no encadena el reloj completo ni demuestra paridad en partida.

La preparación de fiestas `0x005316E0`, la operación `0x61` y la rama de IA de la tarea `0x07` están conectadas: 512 emisiones, 512 despachos y 256 ejecuciones de tarea coinciden con x86 aislado. La emisión intercepta asignador, CRT y cola; la ejecución intercepta la construcción visual. Un guardado modificado prueba el pago, la inserción de la tarea, su efecto en la ciudad y la recarga S05, además del rechazo de la fiesta humana. Los campos de planificación `GameWorld+0x84/0x88` no se serializan en el original y deben suministrarse al anfitrión al reconstruir la sesión. La fiesta humana sigue pendiente y la rama local habitual de `0x03` aún revierte ante los restantes servicios sin conectar. No se ha contrastado el reloj completo ni probado en partida.

La rama humana de `0x07` también aplica ahora el consumo de la oficina, reputación, marca de ciudad, carta `0x24` y peticiones de presentación. En 384 tareas x86 aisladas coinciden los efectos económicos y el contenido de carta; se interceptan el precio, la entrega y la presentación. Dos variantes de un guardado modificado encadenan la tarea y la entrega original de la carta con preferencias activada y desactivada, y comprueban oficina, reputación, ciudad, depósito y recarga S05; se interceptan precio y presentación. Si faltan existencias o modo de presentación, se conserva la tarea. Las ventanas se entregan como solicitudes al anfitrión y no se han probado en partida. `0x03` continúa bloqueada cuando alcanza servicios pendientes.

El emisor decenal de peste `0x004DCA90` y la programación `0x7C` están conectados a la preparación y al despacho. Coinciden 2.048 emisiones y estados CRT y 512 inserciones de tarea frente al x86; en la emisión se interceptan CRT y cola. Un guardado modificado encadena el emisor y el despacho originales, comprueba la cola de tareas y la recarga S05. La tarea `0x1C` que ejecuta la peste todavía no está implementada: el reloj la conserva y avisa cuando vence. La rama habitual de `0x03` necesita también incendio y otros servicios. Sin contraste del reloj completo ni prueba en partida.

La tarea de peste `0x1C` procesa ya población, negocios, empleados municipales, marca de ciudad y carta global, y entrega las solicitudes de aviso y ventana. En 512 estados coinciden los efectos con x86 aislado, con la entrega interceptada. Dos variantes de un guardado modificado, con y sin jugador humano, encadenan la tarea y la entrega original de la carta y contrastan avisos, ventana solicitada y recarga S05; se intercepta la presentación. Si una peste debe retirar un administrador, la preparación revierte y conserva la tarea: esa baja de capitán y sus vistas de oficina aún están pendientes. No se ha probado en partida.

La baja del administrador por peste ya actualiza la lista libre de capitanes, el identificador y las banderas de la oficina, la experiencia y el salario. Coincide en 256 estados x86 aislados, incluidos capitanes ya libres; se rechaza una lista libre cíclica sin modificar el estado. Dos variantes de un guardado modificado, con y sin flota reconstruida, comparan la tarea original y verifican las vistas de oficina y capitanes y su recarga S05. Los cambios pendientes de oficina impiden guardar hasta que el puente los sincroniza. Esto no es una prueba en partida ni una comparación del reloj entero.

El emisor de incendios `0x004DCBF0` y la operación `0x7D` se preparan y despachan con los planos vivos: 2.048 emisiones y 512 inserciones coinciden con x86 aislado, con CRT y cola interceptados en la emisión. Un guardado modificado encadena emisor y despachador originales y verifica la tarea `0x1D` tras recargar S05. La ejecución y propagación de esa tarea aún no está conectada; se conserva al vencer. La rama habitual de `0x03` continúa dependiendo de subastas y otros servicios. No se ha probado en partida.

La tarea `0x1D` procesa ahora la propagación a edificios públicos, el avance del fuego, la liberación de registros y la retirada de subastas, sin vista. Coincide en 512 estados x86 con la subasta interceptada; dos casos adicionales ejecutan la retirada de subasta y la liberación del edificio originales. Un guardado modificado contrasta el tick y la entrega original de la carta global `0x59`, y recarga el edificio y la tarea en S05. Si el fuego selecciona un edificio privado o un tipo no acreditado, se conserva la tarea sin cambios: faltan los efectos de almacén y recuento de oficina. Las ventanas son solicitudes al anfitrión; no hay contraste del reloj completo ni prueba en partida.

La propagación privada de `0x1D` también recuenta viviendas, negocios y empleados, actualiza las cadenas y el pool y sincroniza la oficina con S05. Coincide con x86 aislado en 256 estados sintéticos con retirada de subasta y recuento originales; un guardado modificado contrasta además ciudad, plano, oficina, pool y tarea, y recarga los efectos. La prueba integrada descubrió y corrigió una lista de negocios desfasada al guardar. El crecimiento y la relocalización del pool no se han contrastado para esta ruta; se rechazan sin cambios el incendio de almacén y los tipos no acreditados. Se intercepta la presentación y no se ha probado en partida.

El incendio de almacén también aplica `0x00500C30`: descuenta las mercancías y prepara la carta `0x75` antes del recuento de la oficina. Coinciden 2.048 cálculos aislados y 128 propagaciones con el original x86; la primera comparación intercepta la entrega de la carta y la segunda ejecuta la retirada de subasta y el recuento originales. Dos variantes de un guardado modificado contrastan la entrega original, con el aviso activado o desactivado, y recargan mercancías, carta y plano en S05. La rutina rechaza existencias negativas o cuya suma desborda i32. El crecimiento del pool de negocios durante esta ruta sigue sin contraste; no hay prueba en partida.

El generador anual `0x004DD400` prepara las operaciones `0x38` en la transacción decenal; el despachador crea las tareas de subasta de barcos `0x0A` y S05 conserva fecha y ciudad. Coinciden 512 emisiones y 512 despachos frente a x86 aislado; un guardado modificado enlaza emisor y despachador originales y recarga las tareas. Se interceptan CRT y cola en el emisor. El ciclo de subasta `0x0A` todavía no está portado: al vencer, la sesión conserva la tarea y señala el límite. Tampoco está completa la rama decenal habitual, ni hay prueba en partida.

La demanda de notable `0x004DD530` se prepara en `0x03` y la operación `0x84` crea la petición `0x30`. Coinciden 2.048 emisiones y estados CRT y 512 despachos en x86 aislado; un guardado modificado enlaza emisor y operación originales y recarga la petición en S05. El emisor intercepta CRT, consulta del comerciante y cola; rechaza referencias de notable que quedan fuera de la tabla. La ruta habitual de `0x03` sigue pendiente del robo y la selección de capitán. Sin prueba en partida.

El robo decenal `0x004DD790`, la operación `0x64` y la tarea `0x33` están conectados: el robo traslada mercancías al mercado urbano y entrega la carta `0x73`. Coinciden 1.024 emisiones, 512 despachos y 1.024 resoluciones con x86 aislado; se interceptan CRT y cola en la emisión y búsqueda de oficina y entrega en la resolución aislada. Un guardado modificado enlaza en la misma máquina x86 emisor, operación, despachador, búsqueda original, resolución y entrega, y recarga oficina, ciudad y cartas en S05; se interceptan CRT, cola y presentación. Dos variantes adicionales contrastan la resolución y preferencias de la carta. La resolución rechaza existencias negativas sin aplicar cambios. No se ha probado en partida; `0x03` aún necesita selección de capitán y otras dependencias.

La selección decenal de candidatos reutiliza `0x004F69B0` y carga las longitudes de nombres estáticos del CPR y las 20 ranuras personalizadas de S05; la operación `0x87` crea el capitán, sincroniza sus vistas y entrega la carta `0x7A`. Coinciden 512 bucles de selección y 512 despachos de la operación con x86; cuatro variantes de guardado modificado cubren presencia de flota y crecimiento del pool y recargan S05. La serialización original normaliza el booleano de los registros libres. Ante bancos ausentes, listas cíclicas o selección de aspecto sin salida, se conserva la transacción.

La rama local habitual de `0x03` se ha contrastado ahora en nueve variantes de un guardado modificado, tres fases y tres semillas, con una creación de candidato forzada. El original ejecuta la tarea y las operaciones emitidas en una misma máquina x86; coinciden cola, aleatorios, comerciantes, capitanes, tabernas y planificador, y Rust recarga S05. Se interceptan CRT, cola y búsqueda de nombres y comerciantes; la cola se despacha en la frontera. Esto no cubre todos los resultados de peste, fuego o subastas, ni el reloj completo. El estado CRT y la planificación de fiestas siguen siendo datos transitorios; el modo de sincronización que emite `0xB8` y las tareas aún no portadas conservan sus límites. No hay prueba en partida.

En un escenario de 30 días se encadenan el reloj `0x00530E80`, la tarea decenal `0x03`, las operaciones emitidas y los serializadores originales de tareas y cartas. Coinciden a diario operaciones, reloj, estado CRT, comerciantes, capitanes, tabernas, mercancías entrantes y cola de tareas; se guarda S05 cada día y se comprueban recargas los días 10, 20 y 30. La semilla CRT es explícita y el escenario fuerza una candidatura; las demás fases del reloj se interceptan y las operaciones se despachan al final de cada jornada. No representa el reloj completo ni una prueba en partida.

El contraste descubrió que el guardado repetido rechazaba la imagen intermedia de capitanes creada por la propia sesión. La sesión ahora recuerda la imagen S02 completa y la fábrica asociada solamente tras un guardado correcto; la reutiliza si la imagen recibida coincide exactamente. El contraste también altera externamente un capitán y verifica el rechazo transaccional. Las tareas aún no portadas y la sincronización `0xB8` siguen pendientes.

Las regresiones `fleet_resume` presentan seis fallos: cinco señalan explícitamente la tarea `0x04` del turno de comerciante sin conectar y el sexto falla al avanzar. `persist_all` presenta dos fallos por la tarea `0x1B` del intérprete de misiones. Se reprodujo un fallo de cada grupo también en `main` antes de incorporar este cambio; no son prueba de una regresión de persistencia, pero impiden considerar C10 completo. Las diez pruebas de `c10_ten_day_live_x86` y las siete de `employment_x86` pasan.

La regresión `two_cities_voyage_with_fleet_api_thirty_days` vuelve a pasar en 203,66 segundos, con las mismas fases interceptadas y límites ya indicados; `office` pasa sus 22 casos.

La tarea de turno de comerciante `0x04` procesa en C10 los comerciantes humanos y elimina las referencias a comerciantes inexistentes. El manejador `0x004DFC94` coincide en 1.024 casos x86 con el turno de IA y su limpieza interceptados, incluidos desbordamientos y reprogramación; dos variantes de un guardado modificado enlazan el despachador y serializador de tareas originales y recargan S05. Se prueba además que la rama de IA queda en cola sin alterar el planificador: sus acciones aún no están conectadas, por lo que los fallos de `fleet_resume` siguen pendientes. No hay prueba en partida.

El turno `0x04` también ejecuta el puerto C06 de IA cuando solo necesita recalcular objetivos de existencias, o cuando no pide acciones externas. Se enlazan las oficinas vivas, sus 24 objetivos y marcas con S05, y se rechazan cadenas cíclicas antes de consumir la tarea. En variantes de un guardado modificado, el despachador, turno, recálculo y serializador x86 originales coinciden con Rust y sus resultados se recargan; se interceptan búsquedas de comerciante, tarea y oficina, y el resto de acciones. Las pruebas aisladas de turno y existencias también pasan. La limpieza especial y las acciones de vivienda, gremio, financiación y barcos siguen pendientes; no hay prueba en partida.

El turno `0x04` cubre además el comerciante sin oficinas, la falta de financiación cuando está activo el modo de red y la subvención de 15.000 del modo `GameWorld+0x8A`; esa subvención actualiza las dos vistas de dinero. Variantes de guardado modificado encadenan las funciones x86 originales de turno, financiación, vivienda y gremio en sus ramas sin efecto, y comparan y recargan S05; otra combina la subvención con el recálculo de objetivos. Se interceptan las búsquedas de comerciante, tarea y oficina. La prueba `real_fleet_creates_shared_departure_routes_and_persists_them` sigue detenida en una acción externa de IA no conectada; no hay prueba en partida.

La decisión de vivienda de la IA `0x004F4B40` y la evaluación del gremio `0x004F4A20` leen las oficinas y planos vivos. Si la decisión de vivienda emite `0x22`, se encola y la obra privada utiliza el puente transaccional de materiales y pagos existente. En un guardado modificado se encadenan el despachador y turno x86, la decisión original y el inicio original de vivienda, y coinciden operación, ciudad, plano, comerciante y materiales; Rust guarda y recarga S05. Se interceptan las búsquedas de comerciante, tarea y oficina y la cola; el inicio original se llama directamente, sin pasar por el despachador original de operaciones. Otros 80 inicios sintéticos comparan el puerto de obra con x86. Las cadenas cíclicas se rechazan. Si la IA emite un ingreso en gremio `0x37`, se contrasta la operación pero se conserva la tarea hasta conectar sus efectos. La partida de referencia supera el bloqueo de vivienda y se detiene en el intérprete de misiones `0x1B`; la financiación ordinaria, decisiones de flota, ingreso en gremio y otras tareas siguen pendientes. Sin prueba en partida.

El núcleo aislado de `0x004ECF64` ejecuta asignaciones, reloj, aritmética, comparaciones, saltos y esperas `0x00`, `0xFD`, `0xFE` y fin. Coincide en 1.400 guiones sintéticos con el x86, incluidas operaciones con desbordamiento y controles temporales; libera el puntero de variables como el original al finalizar. Un opcode no portado o un bucle excesivo rechaza la ejecución sin modificar tarea ni variables. Este núcleo aún no está conectado a la tarea `0x1B`, sus recursos CPR ni su estado en S05, y la gran mayoría de comandos siguen pendientes. Sin prueba en partida.

La tarea `0x1B` carga su blob desde el CPR y las variables desde S05, ejecuta el subconjunto portado y guarda cursor y variables con control de cambios externos. El primer tramo activo del guion real 12 del guardado `TEST25_ConvoyMar` coincide dos veces con despachador, intérprete y serializador x86, normalizando punteros; Rust recarga ambas versiones y rechaza un cambio externo. Los comandos desconocidos y la finalización aún no conectada conservan la tarea. Cinco de ocho pruebas `fleet_resume` pasan ahora; las otras se detienen ante `0x0D` o el comando `0x21` de guion. Dos pruebas de `persist_all` siguen detenidas por el armado de barcos de IA. Las seis pruebas de cola S05 y las diez decenales pasan. Falta la mayoría del intérprete, su finalización y el resto de tareas; no hay paridad completa ni prueba en partida.

El comando de guion `0x21` selecciona una ciudad con el reloj, la cabeza viva del planificador y el dword adyacente del array urbano, mediante aritmética x86 sin signo. Se amplía el corpus básico a 1.500 casos y un guion sintético sobre un guardado enlaza despachador y serializador x86, persistencia y recarga. El avance largo del guardado de referencia supera `0x21` y se detiene en el comando `0x04`; el clima diario `0x0D` sigue pendiente en otras variantes. Sin prueba en partida.

El comando `0x04` del guion selecciona el comerciante humano con el mismo recuento de 8 bits con signo y la población de la ciudad que el original. El corpus llega a 1.600 casos x86, con poblaciones negativas y hasta 255 comerciantes; sin humanos se rechaza antes de modificar la tarea. Un guion sintético sobre un guardado enlaza `0x21` y `0x04` con el despachador, serializador y recarga S05. El avance largo de referencia supera ambos y se detiene en `0x38`; no se ha probado en partida.

Los comandos de guion `0x38` y `0x3F` consultan respectivamente la cadena urbana de oficinas y el siguiente comerciante humano. El corpus pasa a 1.800 casos x86; un guion sintético sobre S05 enlaza ambos con selección de ciudad, despacho y recarga. Una cadena de oficinas cíclica y la búsqueda humana sin salida se rechazan sin alterar el estado. El avance largo del guardado de referencia supera esos comandos y llega al recálculo de reputación de la IA del turno `0x04`; siguen pendientes sus efectos y el clima diario. Sin prueba en partida.

El turno de IA `0x04` recalcula el factor base de reputación `Merchant+0x464` mediante el puerto C06 y la puntuación de las ciudades; se sincroniza entre las vistas vivas y se guarda en S05. En un guardado modificado el despachador y cálculo x86 reales coinciden con Rust y la recarga; se interceptan las búsquedas de comerciante y oficina. Otra variante comprueba que, si falla una acción posterior no conectada, el factor y la tarea permanecen intactos. Los contrastes aislados de C06 y del despachador también pasan. La prueba larga supera este punto y se detiene en el comando de guion `0x09`; el clima `0x0D` y otras acciones de IA siguen pendientes. Sin prueba en partida.

El comando de guion `0x09` usa el reloj, la cabeza del planificador y la posición del primer barco en mar de la imagen naval viva. Se reproduce el orden de escritura incluso si coinciden las variables de entrada y salida, con rechazo transaccional de división inválida. El corpus sintético asciende a 1.900 casos x86, con y sin barco, y un guion encadenado compara despacho y serialización del original con S05 y recarga; se intercepta el acceso al barco. El avance largo alcanza el comando `0x20`; el intérprete y el clima diario siguen incompletos. Sin prueba en partida.

El comando de guion `0x20` utiliza el generador de nombres ya existente de la fábrica naval, extraído como rutina compartida; el cursor se sincroniza con la navegación y se conserva en S02. El avance es transaccional ante comandos posteriores no conectados o ciclos sin salida. El corpus llega a 2.000 casos x86; el contraste encadenado comprueba el cursor modificado, tareas, guardado y recarga. Las 28 regresiones x86 de expediciones y fábrica también pasan. El avance largo llega a `0xF7`, creación de cartas de misión, aún pendiente. Sin prueba en partida.

La rama de `0xF7` para cartas de clase 5 sin aviso crea el descriptor y expande los formatos `%Q` (nombre estático de barco) y `%c` (moneda), y entrega a destinatario directo, difusión humana o cadena global. El intérprete publica cartas, variables y cursor de nombres en una transacción; las clases, formatos, destinatario -3, nombres dinámicos y avisos no cubiertos se rechazan. Hay 100 contrastes x86 sintéticos de construcción, 100 de entrega y serialización, y una prueba enlazada del despachador, carta, depósito y S05 con recarga; el rechazo posterior no deja cambios. Se interceptan la búsqueda del nombre y la presentación en los oráculos. El avance largo supera la carta real y se detiene en clima diario `0x0D`; sigue siendo parcial y sin prueba en partida.

La tarea climática `0x0D` procesa los días cuya secuencia requiere ajustes estacionales, marcador periódico urbano y marcas de tormenta próximas. El marcador `GameWorld+0x8F` se conserva en la cola S04/S05 con detección de cambios externos y guardados consecutivos. Coinciden 2.048 secuencias x86 del despachador, 256 estados de clima urbano y 512 selecciones de marcador; dos despachos enlazan el código original con serializadores de tareas y ciudades y la recarga. El generador de nombres y el estado naval se reutilizan, sin reimplementar C07. Las heladas, tormentas, calmas e icebergs no conectados se rechazan sin consumir la tarea. El avance largo pasa la ruta cubierta y alcanza el comando de guion `0x29`. Sin prueba en partida.

El clima invernal calcula frío, temporizadores y congelaciones con el LCG local y programa el deshielo `0x35`; este libera la bandera del puerto. El contraste cubre 512 escenarios x86 y enlaza despacho de clima, creación de tarea, serializadores, guardado, recarga y despacho x86 de deshielo. Los avisos se conservan en una cola de presentación, todavía sin mostrar en interfaz. La velocidad parte de los ajustes cargados y debe actualizarla el anfitrión si cambia; se rechazan fechas desbordadas o ampliación del planificador durante el alta. Tormentas, calmas e icebergs siguen pendientes. Sin prueba en partida.

El productor invernal de icebergs reproduce la tabla solapada, los intentos de navegación y el avance del CRT: 1.024 escenarios sintéticos comparan operaciones y semilla con x86. Se conecta a `0x0D` la ruta que no crea entidad y se contrasta con despachador, clima, serialización y recarga; cuando emite `0x78` la tarea se rechaza de forma transaccional hasta conectar el alta naval existente. El CRT no se conserva en el guardado original y debe aportarse al reanudar. Los generadores estivales y el alta de entidades siguen pendientes; sin prueba en partida.

El productor estival de tormentas y calmas reproduce las dos tablas de posiciones, el límite de operaciones y el CRT: 2.048 escenarios sintéticos coinciden con x86. La ruta sin creación está conectada a `0x0D` y pasa el contraste encadenado con guardado; si genera una entidad `0x78`, la tarea y el CRT se conservan hasta conectar el alta naval. El tablero mantiene C07 asignada a Claude, por lo que esa dependencia no se reimplementa aquí. Sin prueba en partida.

El comando `0x29` selecciona la próxima ciudad de misión con la matriz viva, excluye el historial y reproduce la variación según el reloj y el orden de escrituras con operandos solapados. El corpus alcanza 2.100 casos x86, con distancias empatadas y alias; un guion sintético se contrasta con despacho y S05 usando la matriz del guardado. Una ciudad o matriz inválidas se rechazan sin cambios. El avance largo alcanza `0x53`; falta el resto del intérprete y no hay prueba en partida.

El comando `0x53` consulta la distancia entre ciudades, devuelve 0xFF para índices inválidos y conserva el orden original incluso con variables compartidas. El corpus del intérprete llega a 2.200 casos x86 y el recorrido sintético con la matriz viva guarda y recarga S05. El avance largo pasa esta consulta y alcanza el formato de carta `%t`, aún pendiente. Sin prueba en partida.

El formato `%t` de las cartas de misión resuelve el nombre urbano mediante el identificador de mapa y el banco de nombres cargado; se contrasta con x86 tanto en 100 cartas sintéticas como en el despacho, depósito S05 y recarga. El recorrido largo supera el formato y alcanza el comando `0x24`. Si faltan nombres se rechaza sin modificar la tarea. Sin prueba en partida.

El comando `0x24` calcula un índice de apellido con los dos campos urbanos, la cabeza del planificador y el reloj; usa el contador cargado del banco de nombres. El corpus alcanza 2.300 casos x86 y un recorrido sintético compara despacho, serialización y recarga S05. El avance largo alcanza el formato `%N` de las cartas, para el que falta conectar los apellidos del recurso. Sin prueba en partida.

El formato `%N` resuelve apellidos estáticos y las veinte ranuras personalizadas leídas del guardado; los índices fuera de rango usan la primera entrada como el original. Cien cartas sintéticas contrastan el intérprete y la consulta x86 real, y un recorrido enlazado comprueba despacho, depósito y recarga S05. Si el banco está incompleto se rechaza la carta. La ejecución prolongada supera este punto y se detiene en `0x23`; no demuestra paridad de 30 días ni validación en partida.

El comando `0x23` selecciona un nombre de pila con el reloj, la cabeza del planificador y la ciudad, respetando el selector de tabla y la sobrescritura cuando las variables coinciden. El corpus del intérprete alcanza 2.400 casos x86 y un guion enlazado contrasta despacho y recarga S05. Si faltan contadores o ciudades se rechaza sin consumir la tarea. El avance largo llega al formato `%V`; no es una prueba de paridad integral ni en partida.

El formato `%V` resuelve nombres masculinos y femeninos del CPR y sus ranuras personalizadas del guardado, con la selección y los índices fuera de rango del original. Cien cartas sintéticas comparan la consulta x86 y una prueba enlaza despacho, depósito y recarga S05. Si falta el banco se rechaza sin modificar la tarea. El avance prolongado llega a `%B`; no acredita paridad completa ni prueba en partida.

Los formatos de carta `%B` y `%E` reproducen respectivamente el entero decimal con signo y la marca elegida por el byte inmediato, incluido cero. Cien cartas sintéticas comparan x86 y Rust con límites numéricos, y el despacho enlazado verifica el depósito y la recarga S05. El avance largo supera estos formatos y llega a un estado transitorio de fiestas ausente en `0x03`; la prueba sigue incompleta y no se ha ejecutado el juego.

La ejecución exploratoria de treinta días suministra ahora un estado de fiestas explícito (`next=0`, ciudad `0xFF`), porque el guardado no contiene esos campos; no se atribuye ese valor a la sesión original. Con esa hipótesis alcanza el alta meteorológica `0x78`, que sigue sin conectar. C07 continúa reservada a Claude en el tablero, por lo que no se reimplementa su alta naval aquí. La prueba sigue fallando en esa dependencia y no es una comparación integral.

La tarea periódica `0x16` selecciona destinatarios del casamentero, genera la carta `0x21` o pospone la tarea cuando existe una oferta o candidato vigente. En 512 escenarios sintéticos coinciden estado de tarea y ofertas con x86; dos variantes enlazan despachador, inserción real de carta, serializadores y recarga S05, con entrega permitida o desactivada. Se interceptan constructores y accesos a comerciantes y presentación; el rechazo por dependencias ausentes o cadenas cíclicas conserva la tarea. El ciclo de aceptación, dossier y boda no queda acreditado por esta prueba; no se ha probado en partida.

La tarea `0x17` construye el dossier `0x22`, con tablas de retratos y nombres del sexo correspondiente, y lo entrega o descarta según las preferencias antes de retirar la tarea. Coincide en 512 casos x86 aislados y tres variantes enlazadas comparan despacho, depósito, serialización y recarga; se interceptan constructores, consulta de comerciante y presentación. La operación que crea la tarea tras aceptar la oferta y la boda siguen pendientes. Sin prueba en partida.

La operación `0x72` cobra la tasa, invalida la oferta y programa el dossier `0x17`, o envía el aviso tardío `0x2B`; mantiene sincronizadas las vistas económicas y S05. Coincide en 512 casos x86 y cuatro escenarios enlazados comparan pago, tarea, carta, despachador, serialización y recarga, con respuesta en plazo o tardía y entrega activada o desactivada. Se interceptan presentación y acceso al comerciante. La conversión de la respuesta de interfaz a `0x72` y la boda siguen pendientes; sin prueba en partida.

La respuesta a la carta `0x21` genera y encola `0x72` con cuatro muestras CRT, el destinatario local y la comprobación del plazo del original. Coincide en 512 casos x86 y cuatro recorridos encadenan respuesta, operación, pago, tarea `0x17`, depósito y recarga; los escenarios en plazo y tardíos incluyen preferencias de entrega. Se interceptan el CRT, la cola y la presentación, pero la construcción de la carta de aviso usa el constructor original. La interfaz visual aún debe llamar al método expuesto y la boda sigue pendiente; sin prueba en partida.

La respuesta al dossier `0x22` genera `0x60`; esta operación programa la tarea de boda `0x15`, limpia las respuestas pendientes de las cartas `0x21/0x22` y envía el anuncio `0x33`. La respuesta y el alta coinciden cada una en 512 casos x86, incluidos ticks con bit alto; dos recorridos enlazados contrastan respuesta, CRT, operación, depósito y recarga S05 con ambas preferencias. Se interceptan CRT, cola, presentación y acceso al comerciante. La tarea `0x15` al vencer y la interfaz visual siguen pendientes; sin prueba en partida.

La tarea de boda `0x15` se retira cuando el comerciante no existe o su condición temporal ya no permite la boda: 512 casos x86 aislados y una prueba de despacho y recarga. Si la boda está activa, el reloj conserva la tarea sin efectos, pues la ejecución original crea barcos mediante `0x0050E000` y esa fábrica C07 aún no está conectada. No se ha probado en partida.

El ciclo de cosecha `0x09` ejecuta sus seis fases, cambia las banderas urbanas, consulta las oficinas vivas y publica avisos personales `0x08` y globales `0x09`; respeta la reprogramación y el cierre del ciclo. Coincide en 1.024 casos x86 y ocho variantes enlazan despachador, serializadores originales de economía urbana, cartas y tareas, con recarga S05. Se interceptan búsquedas de oficina/comerciante y presentación; una cadena cíclica revierte. El alta desde el comando de guion `0xFB` aún está pendiente. Sin prueba en partida.

El comando de guion `0xFB` crea la tarea de cosecha con sus variables y la inserta en el planificador sin perder el enlace de la misión. Una ejecución x86 encadena el guion, alta, primera fase de cosecha y serializadores de tareas y economía urbana; Rust compara dos guardados y recargas S05. Sin ranura libre, ante fecha desbordada o un comando posterior no conectado se conserva la tarea sin publicar el alta. El crecimiento del planificador en esta ruta sigue pendiente; sin prueba en partida.

El alta de cosecha desde `0xFB` también amplía el planificador cuando está lleno, conserva las variables de la misión y guarda la tabla ampliada. Una segunda ejecución encadenada contra x86 compara la realocación, la primera fase y dos recargas S05. Se rechazan capacidades incoherentes o desbordadas; sin prueba en partida.

La tarea `0x14` reproduce el recordatorio, el CRT y la conservación o retirada según el modo por turnos. Coincide en 1.024 casos x86 y cuatro variantes enlazan despachador y serializador de tareas con recarga S05. El aviso queda en una cola de presentación aún no conectada; el modo y el CRT son estado de sesión. El índice exactamente igual al número de comerciantes se rechaza porque el original accede fuera del vector. Sin prueba en partida.

El vencimiento `0x11` de encargos urbanos liquida pagos o penalizaciones, actualiza las ranuras de la ciudad, entrega mercancías durante varios días y publica los avisos `0x5A/0x5B`. Coincide en 1.024 casos x86; seis escenarios de dos días enlazan despachador y serializadores originales de comerciante, ciudad, taberna, cartas y tareas, con recarga S05. El contraste descubrió y corrigió la pérdida del ingreso acumulado al guardar repetidamente. El alta y la entrega previa del encargo deben contrastarse por separado; sin prueba en partida.

La operación `0x3D` acepta un encargo urbano, reserva una ranura, programa `0x11`, retira los anuncios de la ciudad y entrega la carta `0x1A`. Coincide en 512 casos x86 aislados; cuatro variantes enlazan alta, dos despachos de vencimiento y serializadores originales con tres guardados y recargas S05. El tablón se conserva en la cabecera del mundo y se rechazan cambios concurrentes sin alterar el guardado. Se interceptan consultas y presentación; la generación de anuncios y la entrega previa de mercancías siguen pendientes. Sin prueba en partida.

La tarea `0x22` renueva los siete anuncios del ayuntamiento y las máscaras de escasez, y reprograma el siguiente ciclo. Coincide en 512 casos sintéticos x86; dos variantes enlazan dos despachos, serializadores originales de las ciudades y tareas y recargas S05. El ejecutable consulta `Town+0x4F0+mercancía*4` para decidir si anuncia, pero usa `Town+0x4F0+mercancía*16` para el precio, distinción que no recogía el resumen previo. Se rechaza un paso múltiplo de 53 que podría dejar al original girando; la entrega previa de mercancías sigue pendiente. Sin prueba en partida.

La operación `0x51` entrega mercancías primero desde los barcos locales del comerciante y después desde su oficina, cobra el precio cuando corresponde y actualiza la tarea y las ranuras urbanas. En 256 escenarios sintéticos coincide con x86; seis variantes enlazan entrega, vencimiento y tres recargas S05, con serializadores originales de oficina, comerciante, ciudad, tareas y cartas. Dos incluyen un barco y una de ellas comprueba también la marca del convoy con el serializador original. Se reutilizan los datos navales existentes y se rechazan cadenas cíclicas o existencias negativas de oficina. Los escenarios no acreditan toda la flota, ni prueban en partida.

La tarea semanal `0x06` acumula experiencia y recargo de los astilleros, actualiza los niveles de casco y distribuye el aviso `0x81`. Coincide en 512 escenarios sintéticos x86; dos variantes con y sin pedido naval ejecutan dos semanas, serializadores originales de astilleros, mensajes, comerciantes y tareas, y recargas S05. Se comprobó la entrega a un humano y la exclusión de otro por sus preferencias. Se rechazan cadenas de pedidos cíclicas o incompletas y niveles incompatibles; no hay prueba en partida.

La tarea `0x2C` recalcula el intervalo de construcción de la IA usando población y cadenas de oficinas y negocios: 512 casos sintéticos coinciden con x86. Está conectada solo en modo cliente de red, donde el original omite la decisión de construcción; dos variantes con y sin recálculo contrastan despachos y recargas S05. En modo anfitrión la tarea conserva su estado y señala que falta `ai_decide_construction` (`0x004DEBA0`); por tanto la IA constructora y la simulación normal de 30 días siguen incompletas. El modo de red se suministra como estado de sesión. Sin prueba en partida.

La ruta de anfitrión de `0x2C` también avanza cuando `ai_decide_construction` vuelve sin efectos: con al menos 20 comerciantes y ninguno desde el índice 20 con más de 40000 monedas. La condición coincide en 256 casos x86, incluidos los umbrales y menos de 20 comerciantes; un escenario enlaza despacho y recargas. Cuando hay una decisión posible, la tarea se conserva y avisa de que esa parte sigue pendiente. Sin prueba en partida.

El cálculo inicial de demanda de `ai_decide_construction` suma consumos ciudadanos y de oficinas, producción y suministros virtuales, aplica las restricciones estacionales y puntúa las instalaciones. Coincide en 256 escenarios x86 con el estado intermedio del original. Es un puerto aislado: falta conectarlo a la selección de ciudad, comerciante y emplazamiento, por lo que no habilita todavía decisiones activas de `0x2C`. Sin prueba en partida.

La decisión activa de `0x2C` ya ordena ciudades e instalaciones, selecciona comerciante y parcela, y encola la obra privada `0x22`. Se corrigió el despacho para iniciar negocios privados con la transacción de construcción existente. El orden coincide en 256 escenarios x86; la selección coincide en 96 con búsquedas controladas y otros 96 con búsqueda real del plano; la posición inicial coincide en 256. Un escenario de un guardado del corpus encadena el planificador, la decisión y el inicio original de una obra real, y compara estado aleatorio, tarea, oficina, comerciante, economía, registros y plano, con recarga S05. Se interceptan asignación/liberación, cola y presentación; no acredita todas las variantes de construcción ni prueba en partida. La ejecución exploratoria de 30 días sigue deteniéndose en el alta meteorológica `0x78` de C07 y no es una comparación integral.

La tarea `0x2A` selecciona la expansión de una oficina IA y usa la fábrica viva de oficinas. La elección y sus tres plazos coinciden en 512 escenarios x86; dos recorridos con una partida del corpus encadenan planificador, fábrica y serializadores originales, uno con ranura libre y otro con crecimiento del pool, con recarga S05. La elección se conserva durante la reserva y no se permite guardar mientras quede pendiente. Se interceptan asignación/liberación y presentación; sin prueba en partida.

La planificación de pedidos navales IA `0x2B` cuenta la flota, decide si hacen falta barcos, selecciona comerciante y tipo, y busca ciudad. Coincide en 512 escenarios x86 con cálculo de coste y cola interceptados. Se conectan al despachador los casos sin pedido y de cliente de red: dos recorridos con dos ciclos y recargas S05 coinciden con el serializador original. Si el anfitrión necesita calcular un pedido, el reloj conserva la tarea y señala que faltan conectar el cálculo de coste y la operación naval `0x2C`, dependencia de C07; no se ha implementado su creación. Sin prueba en partida.

La tarea `0x29` libera una reserva de capitán de taberna, lo devuelve a la cabecera o a la lista libre y conserva o retira la tarea según el indicador. Coincide en 512 casos x86 aislados; cuatro variantes, con y sin flota cargada y con ambos indicadores, enlazan dos despachos, serializadores originales de tareas y taberna y recargas S05. Se comprueba la coherencia entre las vistas existentes y se rechazan listas cíclicas; no se ha probado en partida. El alta previa mediante `0x28` sigue pendiente.

Las subastas `0x0A/0x36` tienen la espera y la apertura de pujas contrastadas en 1.024 casos x86, con la tasación interceptada. La fase de espera está conectada al despachador y pasa dos recorridos con el serializador original y recarga S05. El comienzo de subasta de barcos necesita la fábrica C07; el anuncio, la tasación conectada, las pujas y la adjudicación siguen pendientes y el reloj conserva la tarea cuando llega a una fase no conectada. Sin prueba en partida.

Las rondas de puja de subasta (fases 4–6) coinciden en 768 escenarios x86 con límites por comerciante controlados y presentación interceptada. El puerto conserva el registro si falta una tasación. Aún no está conectado a las tasaciones vivas ni al despacho; el anuncio, la adjudicación y la fábrica naval siguen pendientes. Sin prueba en partida.

Las rondas 4–6 de subasta naval usan ahora la valoración de casco y el límite de puja C06 existentes, con las oficinas y comerciantes vivos. Una variante sintética de un guardado enlaza planificador, puja y valoración originales y compara la tarea guardada y recargada en S05. Se comprueban los datos y se rechazan cadenas cíclicas. La interfaz de subasta, el anuncio, la adjudicación, las pujas de edificios y la fábrica naval siguen pendientes; no hay prueba en partida.

El anuncio de subasta (fase 2) selecciona lotes alternativos, espera si la ciudad está ocupada y reserva su índice de tarea. Coincide en 768 casos x86 con presentación interceptada; cuatro recorridos con y sin reserva previa contrastan el planificador original y la recarga S05. Cuando deben mostrarse avisos, el reloj conserva tarea y ciudad sin cambios hasta conectar la presentación. Siguen pendientes la tasación inicial viva, la adjudicación, pujas de edificios y fábrica naval; sin prueba en partida.

Los destinatarios humanos del anuncio de subasta se entregan ahora a una cola lógica de presentación con ciudad y comerciante; ya no frenan el planificador. La selección coincide con los 768 casos x86 de la fase 2 y una prueba viva verifica la cola. La cola es estado de sesión, no se serializa en S05, y aún falta convertirla en la ventana o mensaje visual que corresponda al modo de velocidad original; sin prueba en partida.

La tasación inicial de subasta naval `0x004DA860` coincide en 2.048 casos x86, incluidas capacidades, mejoras y daños diversos, y se conecta a la fase 3 con despacho y recarga S05. Corrige la descripción previa de `research/systems/interfaz/names.tsv`: el daño es solo un descuento; la función también calcula una base según clase, capacidad y mejora, aplica 0,7 y redondea. El contraste usa el ejecutable inglés y no prueba en partida. La adjudicación y la creación naval siguen pendientes.

El cierre de subasta (fase 8) libera la reserva urbana solo si sigue perteneciendo a la misma tarea, elimina esta y registra una señal de presentación. Coincide en 256 casos x86 y en cuatro recorridos con despachador y recarga S05 para barcos y edificios. La señal es estado de sesión y su pantalla final sigue pendiente; la adjudicación, las pujas de edificios y la creación naval aún no están terminadas. Sin prueba en partida.

La tasación de venta de edificios coincide en 1.024 casos x86 con valor base y estado de asedio controlados. La fase 3 usa el plano y coste base vivos: dos recorridos contrastan edificio interior y exterior sin escena de asedio con el despachador y recarga S05. Si el inmueble exterior tiene una escena de asedio, el reloj conserva la tarea hasta conectar su estado activo. Las pujas y adjudicación de inmuebles siguen pendientes; sin prueba en partida.

La tasación exterior de edificios ya consulta el estado del asedio: usa la escena viva si existe y reconstruye los campos guardados en otro caso. La detección coincide en 512 casos x86 y cinco variantes integradas comparan precio y recarga S05, con asedio ausente, activo o inactivo e imagen viva diferente de la guardada. La adjudicación y las pujas de edificios siguen pendientes; sin prueba en partida.

Las rondas de puja de edificios usan ahora el plano, valor, oficinas y cadenas de negocios vivos junto al cálculo de IA C06. Dos variantes de un guardado, vivienda y negocio con una oficina y empleados, contrastan planificador, tasación y puja originales y recargan S05. Se rechazan cadenas cíclicas, tipos fuera de la tabla y casos de dinero negativo con comportamiento matemático no acreditado; la invalidación visual y la adjudicación siguen pendientes. Sin prueba en partida.

La adjudicación de una subasta sin lote válido pasa al siguiente o prepara el cierre, con normalización de ciudad para inmuebles. Coincide en 512 casos x86 y cuatro variantes con despachador y recarga S05; el reloj conserva la tarea si existe un lote que debe adjudicarse. La entrega de los lotes reales sigue pendiente; sin prueba en partida.

La fase 7 de subasta gestiona los edificios públicos sin comprador: publica el resultado en la cola de presentación de sesión y avanza al siguiente lote o al cierre. Coincide en 512 casos x86 y un recorrido con planificador y recarga S05. Las ventas, traspasos y cartas de lotes con propietario o comprador real aún detienen el reloj; la cola visual no se serializa. Sin prueba en partida.

La subasta de un edificio sin comprador también entrega al propietario la carta `0x7E` según su preferencia y conserva el resultado y el depósito en S05. Cuatro variantes enlazan adjudicación sin venta, entrega de carta y serializadores originales; incluyen propietario público o privado y avisos activados o desactivados. La entrega de los cuatro tipos de carta de subasta coincide en 192 casos x86. Las ventas y traspasos reales siguen pendientes; sin prueba en partida.

La adjudicación de un edificio al comerciante que ya lo poseía cobra el precio, acredita el 90 %, actualiza ambos contadores y entrega la carta `0x7D`, sin mover las listas. Dos variantes con preferencias distintas enlazan la tarea original, el traspaso real, cartas y serializadores x86 con recarga S05. Las ventas a otro propietario todavía esperan su transacción de oficinas y recuentos; sin prueba en partida.

El reloj detiene el avance al producirse cualquier error del despacho programado, incluidos los de subasta y las dependencias ausentes. Dos entradas de avance se probaron con una tarea vencida sobre un guardado real; se conserva la tarea y no se avanza más allá del tick fallido. Como fases anteriores y la sincronización del puerto pueden haberse ejecutado, el estado queda marcado como tick incompleto: no se puede guardar ni continuar y se requiere recargar. Las regresiones de creación de oficina siguen pasando, incluidas las contrastadas con x86. Es una protección de la reimplementación, no una comparación de esta condición de error con el original; sin prueba en partida.

La fase 7 de subasta traspasa inmuebles entre dos comerciantes que ya tienen oficina en la ciudad. La transacción actualiza los dos saldos y acumuladores, enlaza el edificio, recuenta oficinas y negocios, entrega cartas 0x76/0x7F y sincroniza S05. Cuatro variantes de un guardado del corpus comparan vivienda y negocio, avisos activos o desactivados, registros, dinero, pool, serializadores originales y recarga; además cuatro casos sintéticos comparan transferencia y recuentos x86. La búsqueda de oficina y presentación se interceptan; no se ha probado en partida. No están conectadas ventas de inmuebles públicos ni la creación de oficina cuando falta, y no se acredita la paridad de todas las combinaciones.

La subasta de vivienda o negocio ya puede crear la oficina del comprador en el mismo tick, respetando el orden de las cartas 0x76/0x7F y del aviso de oficina 0x0D. La bandera GameWorld+0x8A estaba invertida para el aviso humano; se corrigió con el ensamblador de 0x00530C35. Diez variantes de venta incluyen oficinas existentes o nuevas, avisos, las dos posiciones de la bandera y crecimiento del array, con serializadores originales y recarga S05. La creación y transferencia originales se ejecutan en x86; asignación/liberación y presentación están interceptadas. Al crecer el array el original recuenta la oficina antigua por una referencia al bloque anterior; el contraste conserva accesible el bloque liberado y la reimplementación reproduce ese resultado. El comportamiento real del asignador tras liberar queda sin acreditar. Las ventas públicas y la paridad completa del reloj siguen pendientes; sin prueba en partida.

La venta de vivienda o negocio públicos se conecta cuando el comprador tiene desactivada la carta 0x7F, con oficina existente o recién creada; cuatro recorridos con un guardado comparan despacho, recuento, serializadores y recarga con x86. El ejecutable lee el nombre del propietario público fuera del array de comerciantes (0x004E3617): el oráculo mapea memoria legible en esa dirección, pero no acredita esos bytes ni descarta un fallo real. Si la carta está activa, se rechaza la operación antes de modificar la tarea o el estado, en vez de inventar el nombre. Sin prueba en partida.

La carga y S05 conservan ahora `GameWorld+0x8A` y la cuenta atrás `+0x8B`: estaban inicializadas a cero al cargar. El lector original solo activa +0x8A si el byte guardado es 0x7F y el escritor normaliza la bandera a 0 o 0x7F; +0x8B se copia sin conversión (ensamblador 0x0052FC1F y 0x0052F901). Tres variantes comprueban carga, modificación y recarga; pasan las regresiones de subasta, parada y creación de oficinas. Es contraste con el ensamblador de serialización y ejecución nativa, no una partida ni una comparación de 30 días.

La operación 0x37 de ingreso al gremio se ejecuta desde la cola: aplica cuota, máscara, reputación y carta global 0x0C, y mantiene el mapa global de descubrimiento. Al cargar, este se reconstruye como unión de los mapas persistidos de los comerciantes, como el lector original. Siete casos de operación contrastan el ejecutable y serializadores, incluidos guardas y coste negativo; un escenario enlaza dos turnos de IA, operación original y recargas S05. Se interceptan asignación/liberación y presentación. Un fallo del pool conserva la operación y evita publicar sus efectos. No se ha probado en partida ni se acredita el resto de acciones de IA.

Los guiones 0x1B ya terminan, liberan su ranura del planificador y dejan de escribir su bloque de variables en S05. Siete variantes x86 cubren fin, fin por plazo, espera seguida de fin, puntero de guion nulo, otros dos terminadores y alta de cosecha antes del fin; el guardado recarga. El escritor conserva los extras de otros guiones y de tareas con listas de pérdidas, y rechaza cambios ajenos. La liberación de recursos y memoria del original está interceptada, por lo que no se acredita su ciclo de vida físico ni se ha probado en partida.

La ranura de un guion terminado puede reutilizarse antes del guardado para otra tarea sin datos externos: un recorrido x86 encadena el fin de un guion, la ejecución de otro y el alta de una cosecha en esa misma ranura, conserva las variables restantes y coincide tras dos guardados y recarga. El escritor sigue rechazando la creación de listas externas no serializadas y los cambios ajenos; memoria liberada interceptada, sin partida.

El planificador avisa ahora al reloj si rechaza una tarea vencida sin haber producido un error explícito. La ruta de investigación 0x05 antes podía quedar en cabeza mientras el tiempo seguía avanzando; una prueba con guardado real verifica que ambos modos de avance se detienen y no permiten guardar el tick parcial. Los reintentos para crear oficinas siguen funcionando. Es una protección de la reimplementación ante rutas no conectadas; no es una comparación de ese fallo con el juego.

El despacho de investigación 0x05 retira las tareas con comerciante o ciudades fuera de rango, y los delitos mayores que 15 en fase inicial, igual que el original; ya no detienen el reloj como dependencias sin implementar. Seis variantes comparan planificador y serializador x86 y recargan S05. Las otras clases de delito y condiciones de red siguen acotadas; sin prueba en partida.

La planificación de reposición de capitanes de taberna de la tarea 0x08 coincide en 512 escenarios x86: se comparan tarea y orden de las altas, con búsquedas, recuentos y altas interceptados. Aún no está conectada al estado vivo ni a S05; no se ha probado en partida. Al aplazar artificialmente el clima solo para un diagnóstico, esta tarea es el siguiente bloqueo observado a los 582 ticks; esa ejecución no constituye una comparación de 30 días.

La tarea 0x08 consulta las cadenas vivas de taberna y la flota, y repone capitanes normales y piratas con el pool existente. Los censos coinciden en 128 escenarios x86; seis variantes integradas con y sin navegación, con pool ampliado o sin altas, comparan dos despachos, tabernas, capitanes y serializadores originales, con guardado y recarga S05 entre ambos. El asignador y la consulta de comerciante están interceptados; se rechazan cadenas cíclicas. El diagnóstico con clima aplazado artificialmente alcanza la tarea 0x02 a los 826 ticks: no es un contraste integral ni una prueba en partida.

La espera anual de la elección de alcalde 0x02 inicializa la fecha y reprograma el siguiente año; 512 casos x86 contrastan fecha y decisión de convocar con la elección interceptada, y tres variantes enlazan despachador original, serializador y recarga S05. Al llegar al día electoral, el reloj conserva la tarea y señala que aún falta conectar elección, rangos, mensajes y posible estatua. Se rechaza la fecha inicial con ticks mayores que 0x7FFFFFFF; sin prueba en partida.

La elección municipal 0x02 tiene selección de candidatos contrastada en 128 casos x86, incluida la actualización de la bandera de candidatura de la IA. El cálculo de votos, sobornos, ganador y mandatos coincide en 256 casos, con candidatos y actualización de rango interceptados y cartas desactivadas. Estas piezas aún no están conectadas al reloj: faltan rangos, cartas, estatua y contraste de la transacción completa; sin partida.

La elección 0x02 ejecuta ahora la ruta con alcalde anterior y ganador ficticios cuando no hay humanos de la ciudad que deban recibir la carta textual. Se actualizan candidaturas, votos, sobornos, mandatos, rangos de concejales y carta global 0x55; cuatro variantes contrastan el despachador, la elección, el pool y la recarga S05 con x86, incluidas elecciones con un comerciante candidato y la bandera mundial en ambas posiciones. Consulta de comerciantes, nombres, formato y asignación están interceptados; la carta textual no se contrasta. Las rutas que requieren rangos de comerciantes, carta textual o estatua siguen detenidas. En un diagnóstico con el clima aplazado artificialmente el siguiente límite aparece en el armado de IA, a los 1.339 ticks; no es paridad de 30 días ni prueba en partida.

El turno de comerciante 0x04 conecta el marcado de armado a la rutina C06 existente y sincroniza las vistas de barcos y S05. Cuatro variantes de un guardado comparan planificador, marcado y campos fijos del serializador x86, con y sin navegación y con y sin barco ya marcado; guardado, recarga, guardados sucesivos y rechazo de cambios externos también pasan. La búsqueda de comerciante está interceptada y para serializar los campos fijos se emplea una copia x86 sin ruta: no se contrastan aquí las rutas. Los puertos base y otras decisiones de IA siguen pendientes; sin partida.

El turno 0x04 conecta la asignación de puertos base a la rutina C06 existente y sincroniza las vistas y S05. Cuatro variantes con una partida de red contrastan despacho, listas de oficinas y barcos, asignación y campos fijos del serializador x86, con y sin navegación y con puertos iniciales válidos o sin asignar; pasan recarga, guardados sucesivos y rechazo de conflictos. Se interceptan las consultas de comerciante/oficina y se serializa una copia x86 sin ruta, que no se contrasta aquí. El diagnóstico con clima aplazado alcanza el cálculo de pedidos 0x2B a los 2.362 ticks; no es paridad de 30 días ni prueba en partida.

La tarea 0x2B usa ahora la disponibilidad de materiales del astillero para comerciantes IA: cuenta existencias de la oficina y barcos atracados, la disponibilidad urbana y la clase del casco. El predicado coincide en 1.024 escenarios x86, con consulta de oficina y precio interceptados; no calcula el presupuesto completo. Cuatro variantes enlazan dos despachos y recargas S05 cuando no se emite pedido, incluyendo astilleros sin materiales. Si se puede pedir un barco, se conserva la tarea hasta conectar la operación naval 0x2C; sin prueba en partida.

La elección 0x02 también actualiza los rangos del alcalde anterior y el elegido cuando son comerciantes de IA, reutilizando C06 y manteniendo la operación transaccional. Cinco variantes de un guardado contrastan cambio y reelección, rangos, concejales, carta global y recarga con el despachador y ejecutable x86; se interceptan consultas, asignación, nombres y formato, y se normaliza el puntero temporal Merchant+0xE8. Se detienen los casos que requieren rango humano, carta textual, estatua o programación adicional de misiones. Sin prueba en partida.

La reelección de un alcalde comerciante programa la tarea de estatua 0x38 con la fecha calculada a partir de los votos. Cuatro variantes contrastan el alta, orden del planificador y recarga con x86, con ranura libre o ampliación del array y ambas posiciones de la bandera mundial; la asignación/liberación está interceptada. La ejecución de la estatua al vencer sigue pendiente, al igual que las elecciones humanas y su carta textual. Sin prueba en partida.

La tarea 0x38 ejecuta ahora la espera, comprobación de duplicados y dedicación de la estatua, y guarda el edificio y las cartas global y personales. La rutina aislada coincide en 256 casos con x86; siete variantes integran el despacho, los serializadores originales de tareas, edificios y mensajes y la recarga de S05, incluidos dos destinatarios humanos y preferencias desactivadas. Se intercepta la consulta de comerciante, el asignador y la vista urbana; el evento visual se conserva para presentación, que no se ha conectado ni contrastado aquí. El contraste integrado usa el estado naval de la partida sin activar navegación; la coordenada de un barco en el mar se contrasta por separado en la rutina aislada. Las elecciones humanas y su carta textual siguen pendientes. Sin prueba en partida.

El clima diario encola y despacha ahora la operación 0x78 reutilizando el reinicio de barcos y el orden espacial de C07. La operación coincide en 96 escenarios x86 aislados, con ranuras ocupadas, libres y ampliación de la flota. Dos escenarios encadenan en x86 el productor invernal o estival con las altas originales y comparan cola, CRT, planificador, ciudades y flota; la asignación y la consulta de navegación están interceptadas. Otros recorridos guardan y recargan varias altas, una caducidad y la reutilización de la ranura, con y sin ampliación, y rechazan un conflicto ajeno. No se ha contrastado aquí la presentación visual ni todos los efectos marítimos. En la partida de diagnóstico, con CRT y estado de fiestas suministrados por el anfitrión, el avance supera 0x78 y se detiene en el comando de guion 0xF0 de la tarea 0x1B; no es una comparación integral de 30 días ni una prueba en partida.

Los guiones ejecutan `0xF0` (retirar mapas de tesoro o completar sus máscaras) y `0x39` (contar humanos). El depósito y sus enlaces coinciden en 200 casos con la rutina y el serializador x86; el intérprete compara 2.500 casos, 100 de ellos del recuento. Un escenario encadena `0xF0`, envío `0xF7` y otro `0xF0`, con asignación/liberación y consulta de comerciante interceptadas, y compara tarea, cartas y recarga. Si un comando posterior falla, no se publican los efectos anteriores. La ejecución de diagnóstico llega ahora a un encuentro marítimo, pero la prueba antigua que serializa solo la batalla falla al encontrar una entidad meteorológica creada durante el avance; sigue pendiente conciliar la flota y el estado completo de C10 antes de guardar la transición. El CRT y las fiestas se inicializaron con valores explícitos de prueba; no hay comparación integral de 30 días ni prueba en partida.

El encuentro inicial puede guardarse ahora junto con el estado completo de C10, si el tick está terminado, no quedan operaciones ni diálogo y el gestor no contiene otra batalla. La conciliación contempla una entidad meteorológica inicial que caduca y cuya ranura se reutiliza; el resultado se comprueba contra la flota recargada, admite guardado repetido y rechaza una batalla ajena. El recorrido de diagnóstico llega al encuentro tras 3.998 ticks, guarda y recarga; no contrasta ese recorrido completo con x86 ni continúa el combate tras cargar. Dos campos internos de C10 impedían compilar los fixtures externos de `NullHost`; se restableció su visibilidad compatible y pasan `pirate_fleet_tick` (6 pruebas) y `famine_x86` (2). Sin prueba en partida.

La continuación experimental reabre el encuentro con el anfitrión existente de C07 y el CRT explícito, resuelve este caso en 64 ticks de batalla, guarda y reconstruye 74 barcos. Una prueba avanza 128 ticks adicionales y vuelve a guardar y recargar el reloj. En una exploración más larga, también con modo local explícito, el reloj se detiene en la tarea 0x05 por un delito 6 aún no admitido tras 4.282 ticks desde el comienzo. No es un contraste integrado contra x86, ni demuestra paridad de 30 días o en partida.

La acusación inicial por delito 6 de la tarea 0x05 coincide en 120 casos x86, con dos muestras CRT y carta contrastada para humanos; cuatro variantes de IA enlazan el planificador y el serializador original con recarga S05. El juicio del delito 6 aún no está implementado. La exploración del guardado anterior, con CRT, fiestas y modo local explícitos, supera esa acusación y se detiene en el comando de guion 0x40 a los 4.652 ticks. No es contraste integral con x86 ni prueba en partida.

El comando de guion 0x40 recorre la cadena naval del comerciante y selecciona por porcentaje de casco y estado. Se contrastan 400 escenarios x86 con órdenes de lista y entradas solapadas; cuatro variantes de una partida enlazan el despachador, la flota y el serializador originales, y la recarga S05. El diagnóstico continúa hasta el juicio del delito 6 de la tarea 0x05 en el tick 6.330. El CRT, fiestas y modo local son hipótesis explícitas de prueba; no hay contraste integral de 30 días ni prueba en partida.

El juicio del delito 6 coincide en 120 escenarios x86, con absolución, multa y cartas; incluye ciudades con bandera 0x20000 para mercancías 0..23. Como la búsqueda original usa ciudad 0xFF, en mundos de hasta 255 ciudades no encuentra oficina y conserva la multa monetaria. Cuatro variantes IA enlazan planificador, ciudad, comerciante, serializador original y recarga S05. El diagnóstico alcanza después el comando de guion 0x43 a los 6.462 ticks; el programa auxiliar de diagnóstico falla al imprimir esa tarea no criminal. No es contraste integral de 30 días ni prueba en partida.

El comando de guion 0x43 devuelve el número de ciudades de la sesión. Se compara como parte de 2.500 casos x86 y de un guion encadenado con el despachador y recarga S05. La exploración prosigue hasta el comando 0x4F en el mismo tick 6.462. Sin contraste integral de 30 días ni prueba en partida.

El comando de guion 0x4F cuenta oficinas urbanas según el control de sus propietarios y respeta la lectura dinámica cuando la variable de salida coincide con las entradas. Se contrasta dentro de 2.500 casos x86 y en un guion encadenado con planificador y S05. Una prueba con AUTO.pat, CRT 123, modo local, estado de fiestas y escena de batalla marcada ocupada para el ensayo alcanza 30 días (7.680 ticks), avanza dos encuentros con C07 a los ticks 3.998 y 7.312 y guarda y recarga al final, omitiendo la llamada pendiente `open_for_player` en al menos uno de ellos. Es una prueba de continuidad con supuestos explícitos: aún no compara el estado integral de esos 30 días con el ejecutable original ni acredita paridad en partida.

La recarga final del diagnóstico de 30 días conserva población, mercancías, dinero, cola de tareas y depósito de cartas, además del reloj. Al activar la flota completa, la sesión conserva el CRT, el estado de fiestas y los modos externos de investigación, recordatorios, operaciones, asedio y velocidad meteorológica; esos datos no deben inferirse de S05. La cadena sintética con 30 muestras diarias, dos ciudades, asedios y viaje naval vuelve a coincidir en las fases contrastadas con x86; la última muestra está a 29 días y 9 ticks y otras fases siguen interceptadas. No se ha probado en partida.

El recorrido de 30 días no es aún un avance válido del anfitrión sin pantalla: el inicio de batalla solicita open_for_player y su ejecución C07 sigue pendiente en esta integración. Los diagnósticos anteriores omitieron expresamente ese efecto; marcar la escena como ocupada no lo elimina. La escritura inicial de C10 y el guardado de sesión rechazan ahora esa petición sin despachar para evitar perderla en S05. El avance sin pantalla normal también se detiene y comunica el efecto. Sin comparación integral ni partida.

La fábrica puede modificar el enlace de una ranura libre antes de su alta meteorológica: la conciliación acepta esa versión únicamente cuando procede de la base o de un guardado de la sesión, y sigue rechazando una tercera versión. Una prueba con flota cargada verifica guardados repetidos, recarga y conflicto atómico; pasan también los dos contrastes meteorológicos x86 existentes. El diagnóstico continuo llega a 7.680 ticks, guarda desde la sesión y recarga sin perder los totales comprobados, pero omite el efecto naval pendiente indicado arriba. La sesión sin batalla activa ni comercio naval pendiente usa la transacción completa de C10 para guardar; otros recorridos conservan su ruta anterior.

La batería Rust completa aún no está aprobada. Una ejecución aislada llegó hasta la prueba PST, cuyo fixture no restauraba el estado de fiestas; corregida y repetida pasa. Las pruebas de sesión que reabren partidas sin restablecer el CRT y otros estados transitorios requieren revisión; no se atribuye ningún valor de prueba a la partida original.

Una orden desde la flota inactiva reconstruía internamente la sesión y perdía
el CRT, las fiestas y otros valores que no están en S05. La reconstrucción
conserva ahora esos valores, las estadísticas externas, los textos de batalla,
las entradas visuales y las colas de avisos. Una regresión comprueba CRT,
fiestas, modo local, velocidad meteorológica y un aviso. En una batería de 48
pruebas de sesión pasan 33 y fallan 15: las recargas necesitan estado externo
y recursos navales, y quedan rutas de guardado y encuentros por investigar.
No se ha demostrado paridad con el original ni probado en partida.

La transacción completa de C10 distingue ahora la baja de un barco comercial
de la caducidad de una entidad meteorológica. Para una baja naval admite la
retirada de la ruta anterior, comprueba que no haya sido alterada por otro
sistema y aplica la serialización de bajas existente. La regresión de sesión
`session_removes_ai_sunk_ship_and_reloads` pasa con sus fixtures de combate
x86, conflictos y recarga; no se ha contrastado con el original la transacción
completa de 30 días ni probado en partida.

Al dar una orden desde la flota inactiva se conservan también la matriz de IA
y el planificador naval previamente configurados. La ruta solicitada reutiliza
el planificador con ciudades en lugar de reemplazarlo por otro sin ciudades:
esto evita detener el reloj al identificar llegadas de otros barcos. Una prueba
de dos viajes simultáneos avanza la flota completa, guarda y reabre dos veces
trasladando explícitamente CRT y demás estado de sesión, y compara los barcos
entre el recorrido continuo y el recargado. No es un contraste integral con
el original; siguen pendientes otras pruebas de sesión y `open_for_player`.

La batería de sesión pasa ahora 42 de 48 pruebas. Además de los dos barcos
simultáneos pasan el viaje y la llegada de un convoy, maniobras y cambios de
destino con flota completa y recarga. De los seis fallos restantes, uno
rechaza una orden por viaje duplicado con flota completa; otros requieren
revisar recursos y estados temporales al reabrir, y uno comprueba un guardado
que ahora rechaza correctamente `open_for_player` pendiente. Es evidencia de
ejecución nativa, no contraste integral con el original ni prueba en partida.

Tras restaurar explícitamente el CRT y el estado de sesión al reabrir, la
batería de 48 pruebas pasa 45 y mantiene tres fallos: una orden a barco ya
administrado devuelve `viaje duplicado`, y dos guardados rechazan el encuentro
con `open_for_player` sin despachar. No se ha omitido ese efecto en esas
pruebas. También pasan la ruta compartida y los barcos del jugador sin viaje
mientras el resto de la flota continúa; no equivalen a un contraste integral
con el original.

La activación de una batalla también rechaza `open_for_player` pendiente,
para que no se pierda al trasladarla al anfitrión. El diagnóstico que continúa
30 días prueba primero el rechazo y omite expresamente ese efecto solo para
examinar el resto. Se repitió el escenario sintético de 30 muestras diarias, dos ciudades y
flota con fases seleccionadas del original en x86: pasa; la última muestra
queda a 29 días y 9 ticks y otras fases se interceptan, por lo que no sustituye
el contraste integral.

Una orden nueva para un barco que la flota ya administra reutiliza el inicio
de viaje C07 y actualiza esa entrada sin descartar los demás recorridos. La
prueba de dos órdenes separadas por un día, guardado y recarga pasa; este
resultado es nativo y no compara la transacción completa con x86. La prueba
del centro comercial usa ahora un CRT explícito y aplaza las tareas de la
partida porque el tick del fixture x86 no corresponde a su calendario; el
centro y su recarga coinciden, con ese aislamiento declarado.

La batería de sesión posterior pasa 46/48: solo fallan los dos encuentros
que requieren `open_for_player`. Pasan además 31 regresiones de salida y
convoyes contra fixtures obtenidos del original, y el grupo sin pantalla
(8 pruebas). Los fixtures navales no prueban por sí solos la orden integrada
en el reloj original. El centro comercial aislado pasa con la hipótesis de
calendario y el aplazamiento de tareas indicados arriba.

El guardado de sesión con comercio naval conserva ahora el marcador
meteorológico y el tablón urbano. Una prueba de comercio, guardado y recarga
reprodujo la pérdida anterior y verifica los dos campos y la carga; también
comprueba rechazo atómico de un tablón ajeno. Pasan las regresiones de ambos
campos contra x86 y las nueve pruebas sin pantalla. La ruta completa de
comercio y 30 días aún no está contrastada con el original.

El comercio desde un barco publica ahora la carga y las banderas de convoy en
las vistas económica y naval; antes una tarea decenal detenía el reloj por
desincronización. La prueba compra, avanza dos días con toda la flota, guarda
y recarga población, mercancías, dinero, tareas y cartas, vende y vuelve a
guardar. Verifica precios de carga, guardados repetidos y rechazo de vistas
de carga y de banderas de convoy divergentes antes de modificar la economía. Pasan diez pruebas sin pantalla y el fixture x86 de comercio
naval. El recorrido de dos días es nativo; no se ha contrastado íntegramente
con el original ni acredita paridad de 30 días o en partida.

Los ejecutables de C10 aceptan el estado externo de fiestas y el modo local
de investigación; comunican el estado final de fiestas y CRT para reanudar.
El ejemplo también admite el ejecutable inglés para textos de batalla y
elecciones. Al reconstruir flota se conservan los textos y avisos C08. Pasan
cinco pruebas del CLI, una comparación de continuaciones desde la misma
recarga y el ejemplo de un día con recursos locales. El cargador original
recalcula precios, por lo que una continuación recargada puede diferir de
una continua; no se ha demostrado la causa de cada diferencia ni contrastado
el recorrido integral de 30 días.

Con el CLI, `AUTO.pat`, flota, CPR y ejecutable ingleses, CRT 123, fiestas
`0:255` y modo local explícitos, el intento de 30 días se detiene en el primer
encuentro pirata a los 3.998 ticks (15 días completos). El CLI aún no
configura ese encuentro; no produce un guardado parcial. Es una exploración
nativa, no una comparación de los 30 días con el original.

Corrección y ampliación del contraste encadenado: 30 muestras diarias cubrían
29 días y 9 ticks, no 30 días completos. Una variante con 31 muestras alcanza
30 días y 9 ticks (7.689), con dos ciudades, viaje, flota y recargas; coinciden
las fases y serializadores que compara. El oráculo intercepta puerto y batalla
7.689 veces cada uno y mantiene otros ganchos declarados: no compara sus
efectos ni el estado integral de la partida. Ejecución x86 aislada, sin partida.

La variante de 30 días y 9 ticks ejecuta ahora las 7.689 llamadas originales
al gestor de batalla y al puerto, con ambos vacíos y sin vista de puerto. Las
imágenes comprobadas permanecen estables y el resto de las fases comparadas
coincide; pasan también cuatro regresiones del gestor, incluida su conexión al
reloj y S05 con recursos explícitos. Es un escenario sintético sin encuentros
ni puerto poblado; siguen otros ganchos y falta el contraste integral.

El guardado de las listas temporales del gestor naval acepta ahora el último
S05 confirmado por la misma sesión y rechaza un tercero; no confirma el punto
de guardado si falla otra parte de la transacción. Tres ticks y escrituras
sucesivos coinciden con el gestor y el serializador x86. Una cadena sintética
de 31 días y 9 ticks ejecuta el puerto vacío y el gestor originales, comienza
con listas temporales que caducan y compara 32 escrituras con Rust, que recarga
S05 cada día; el gestor x86 permanece en memoria. El asignador está interceptado; no hay batalla
activa y persisten otros ganchos, por lo que sigue sin paridad integral. Una
prueba de encuentro inicial admite un guardado repetido con gestor vacío y
rechaza conservar la batalla si aún hay listas auxiliares vivas; en ese
ensayo se omite explícitamente el efecto `open_for_player` pendiente.

La transacción S05 conserva ahora la banda completa de S02, incluidos la lista
de reserva `+0x12`, el plano y las piezas, y concilia sus campos compartidos
con la vista de misiones. Una entrada explícita aplica la parte de banda de
una absorción ya calculada; el llamante todavía debe ejecutar y publicar los
barcos, el convoy y los demás efectos. En 64 escenarios x86 se ejecutan
`0x00514B80` y `0x00515DA0`, con y sin plano; el serializador original y el
guardado Rust coinciden para la banda. La liberación del convoy, `0x535740` y
`memcpy` están interceptados, y se suprimen mejoras y eventos. No es un tick
naval completo ni una prueba en partida. La prueba real de 30 días, con flota
reconstruida, sigue deteniéndose en `cv_514b80` a los 994 ticks.

## 4/11/2026 — integración C10 en el PC de Sergio

El límite de absorción anterior está resuelto: el tick ejecuta absorción,
mejora de barco, liberación de convoy, guarida y emisiones originales. Pasan
128 cadenas x86 y un recorrido real de 1.000 ticks con guardados consecutivos
y recarga de bandas, barcos, carga y cabezas. La fábrica y la navegación
comparten ahora también los cambios de barcos activos; S02 concilia las rutas
por su contenido y rechaza una tercera versión sin publicar la transacción.

La operación IA `0x2C` y la entrega de bodas `0x15` están conectadas al
despachador y a S05. Los pedidos contrastan 16 operaciones originales en ocho
escenarios y las bodas ocho escenarios completos con cartas, preferencias,
altas y crecimiento de 64 ranuras. Todos pasan el guardado repetido y recarga.
Se corrigió además la publicación de ranuras meteorológicas: al activarse
deben transferir la imagen completa, pues S02 solo guardaba cinco bytes libres.

La apertura automática pirata carga `TileSize` desde `scripts/iso.ini` y
ejecuta colocación y obstáculos inmediatamente después del inicio. La cadena
original inicio → apertura → serializador coincide en 64 escenarios. Detectó
y corrigió `SeaBattle+0x12E8` al descartar miembros interiores del segundo
bando; el punto de llamada y el orden RNG coinciden. Esto no acredita todavía
la resolución de todos los encuentros del ensayo real de 30 días.

TestAstiller completa ahora 7.680 ticks nativos con recursos y estados externos
explícitos. El bloqueo en el tick relativo 4.942 era una investigación del
delito 5. Se conectaron los delitos 5 y 7 y se corrigió la carta inicial humana
compartida con el 6 (`0x0A`); 96 escenarios de dos fases y la cadena real de
despacho y S05 coinciden con x86. La revisión de los demás delitos sigue activa.
El ensayo nativo no se presenta como comparación integral con el original:
se está construyendo el entorno de carga original completo para ese contraste.

La fase 0 de la subasta de barcos (`0x0A`, caso 0 de `st_auction` en `0x004E2D1A`) ya crea el barco subastado: reserva en modo 2 de `0x00507D60` (estado `0x0C`, cabeza `gestor+0xE8`, sello `+0x4C`, crecimiento de 0x40 registros), tipo según la bandera `Town+0x2C8` y el tick, estadísticas de astillero, ranuras, carga, daño `((tick>>8)+tick)&0x7F+0x81` sobre el casco y nombre. Puerto en `p3_core::economy::auction_ship` (módulo aparte) y publicación en las vistas navales de `p3-sim` como el pedido de astillero `0x0B`; el guardado escribe además la cabeza `+0xE8` cuando cambia. Coinciden 400 casos sintéticos con el despachador x86 (179 con crecimiento) y seis variantes sobre `TEST25_ConvoyMar` con y sin flota viva, pool agotado y ciudad fuera de rango, con S05 y recarga. Se interceptan solo el objeto de pila y el banco de nombres; la memoria nueva del asignador se toma a cero. La fase 0 de una tarea `0x36` sigue sin conectar. Sin prueba en partida.
