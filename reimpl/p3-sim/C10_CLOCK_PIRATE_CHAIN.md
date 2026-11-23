# C10: reloj, guarida y guardado en una ejecución original

Una partida sintética derivada de AUTO conserva la flota y las coordenadas
reales de los recursos ingleses. La tarea 0x32/3 provoca una guarida sin
defensores. El oráculo `x86_c10_pirate_clock_trace.py` carga esa partida y,
en la misma instancia x86, ejecuta reloj, planificador, creación y apertura
de escena, fotogramas 0x94, despacho B4 y serializador original. El conductor
inyecta los fotogramas por la cola original; adapta memoria, recursos, archivos
y presentación según el oráculo general. Rechaza la ejecución si faltan las
etapas ordenadas, queda alguna escena u operación o no hay guardado.

La prueba `c10_pirate_original_chain` recorre la misma secuencia por Session y
Sim y compara los ocho bloques S05 con `compare_c10_real_snapshots.py`, que
normaliza los punteros documentados. Coinciden la base inicial y los guardados
finales para un fotograma de delta 1 y para dos de delta 0 y 1. La escena se
ejecuta con jugador local 36, semilla CRT 123 y adaptadores explícitos. La
partida, capturas y reportes permanecen locales en coordination-runtime.

El contraste descubrió que B4 leía la región para elegir la configuración de
la banda, mientras que `0x513DB2` consulta la orientación del mar de
`StadtDaten.ini`. También reveló datos ausentes del destinatario en la carta
0x3A, la pérdida de textos al activar la flota completa y una comprobación que
confundía el token de ruta nativo con el puntero original. Se corrigieron y se
conserva el rechazo ante discrepancias reales de las demás posiciones.

Límites: es una guarida vacía con terreno, una escena activa y una cola de
presentación adaptada. No acredita defensores, torres, proyectiles, varias
escenas simultáneas, UI real ni paridad en partida. Las rutas aún rechazadas
deben implementarse y contrastarse antes de aceptar C10 como completo.

## Primera preparación con defensores

`P3_C10_PIRATE_KEEP_DEFENDERS=1`, junto con las opciones de coordenadas reales
y de salida del generador de `c10_pirate_session`, conserva las piezas de la
banda de AUTO y prepara un plano sintético y una ranura de convoy reservada.
Solo produce una entrada local: no afirma una escena o una batalla completas.
Con esos datos, la sesión alcanza la guardia explícita `guarida con defensores
no soportada`. Sin preparar el plano y el convoy, antes se rechazan sus altas.

El original con plano sintético y sin convoy reservado llegó a `0x5FBFE0`,
pero el entorno aislado carecía del objeto gráfico global `0x6E51AC` y falló
en la consulta de píxel a casilla `0x58D048`. El observador registra dirección,
instrucción y registros de esa frontera sin sustituir la función. Debe
reconstruirse el contexto geométrico y contrastarse antes de presentar el
ataque poblado como ejecutado o de eliminar la guardia nativa.

La creación de plano que faltaba se prepara ahora en una copia de la banda:
`prepare_missing_hideout_plan` carga el recurso original, coloca las piezas y
se conecta al comienzo del ataque. En el escenario real sin plano, el contraste
con `0x514530` antes de abrir la escena compara cabecera, las 28.928 celdas,
piezas y cabecera de banda; son iguales. La instalación de geometría sintética
ocurre después de esa observación. Con convoy reservado, la sesión alcanza la
guardia de defensores y conserva intactas banda y flota al rechazar la escena.
Si existe un barco de reserva y una ranura de convoy libre, el ataque también
prepara el convoy en copias. Un escenario que parte del reloj compara con
`0x514530` original la banda, gestor, convoy y barco; coinciden. Si la reserva
está vacía, se crea el barco en la transacción de misión. La
escena con defensores todavía no se publica y el comportamiento de otras entradas de
plano y convoy sigue pendiente de contrastar.

Si el pool de convoyes está lleno, ahora se prepara en copia su ampliación de
32 ranuras antes del alta. `x86_c10_pirate_full_pool.py` fuerza la cabeza libre
al límite después de preparar el plano; desde ese punto el asignador y la
creación de convoy ejecutan código original. El gestor, cabecera de banda,
convoy y barco coinciden con Rust. El asignador aislado 0x509AE0 ya se contrastó
en 128 casos, con el asignador de memoria del oráculo inicializado a cero; los
bytes que el original no inicializa no se deben atribuir a una garantía del
juego. La geometría posterior sigue siendo hipotética. Este contraste termina
antes de la escena y no demuestra su simulación ni su persistencia.

La preparación `prepare_hideout_new_reserve` reproduce la
rama sin barco: crea un barco y capitán, crea un convoy vacío y devuelve el
barco a la reserva con un cuarto de casco. En dos escenarios x86 iniciados por
el reloj se fuerza la reserva vacía tras crear el plano; en uno también se
agotan los pools de barco y capitán. Coinciden banda, gestor, barco, capitán y
convoy, con rollback si faltan recursos. El conductor x86 de esos escenarios
alcanza B4 y el guardado con geometría hipotética. La misión ya usa esta
preparación y publica capitanes, rutas y vistas ampliadas. Tres ejecuciones
comparan los ocho bloques del guardado tras reloj → misión → 0x94 → B4 con
Rust: coinciden, con ranuras libres, con pools de barcos y capitanes forzados a
crecer y con esos pools y el de convoyes forzados a crecer a la vez. Estas
escenas no tienen defensores ni torres operativas.

El inicio y cierre recorren la cadena real de piezas desde la entrada cero y
distinguen las torres operativas 0x30/0x31 de las demás piezas. La comprobación
anterior usaba por error la cabeza de barcos como índice de pieza; ahora se
detectan ciclos y se conservan las piezas al cerrar la guarida. La presencia de
defensores o torres operativas sigue rechazándose, sin publicar la transacción.

El explorador `x86_c10_pirate_synthetic_scene.py` instala a propósito una
rejilla hipotética 128×256 y la máscara documentada en `geom.rs`. Con la
variante de convoy reservado, el original abre la guarida y ejecuta dos
operaciones 0x94 con un barco atacante, uno defensor, dos barcos locales y
ningún proyectil; la escena continúa activa, por lo que el conductor devuelve
`driver_incomplete`. El informe incluye el hash y la declaración de geometría
sintética. No reproduce el contexto gráfico real ni compara el resultado con
Rust; se conserva como punto de partida para reconstruir esa dependencia.

La traza opcional de escena conserva localmente el objeto y los barcos de cada
frame. En la escena poblada de IA, `0x5FD510` acumula los deltas en
`+0x130F`: los primeros 15 frames de delta 1 solo cambian ese acumulador y el
byte `+0xAC8`; al alcanzar 16 se ejecutan movimiento y colisiones. La función
`pirate_active_frame::advance_ai_prefix` reproduce y contrasta esos 15 frames
y el delta cero; rechaza sin mutar cuando se alcanza un tick completo, hay
UI humana, diálogo o cambios de terreno. Aún no está conectada a la apertura
de escena ni acredita su simulación, destrucción o guardado.

La captura local `x86_c10_pirate_active_snapshot.py` conserva también las
tablas y matrices espaciales, flota y capitanes antes y después de cada tick
seleccionado. Para un atacante y un defensor, la función de exploración
`advance_town_attack_before_contact` encadena los dos primeros ticks completos
y los 15 frames intermedios: escena, barcos locales, flota e índice espacial
coinciden con x86 byte a byte. Se usó la geometría hipotética descrita arriba.

Esta vía todavía no inicia ni publica la escena en la sesión. Solo admite un
tick por llamada y comprueba que el entorno de cada barco sea agua despejada;
rechaza transaccionalmente costa, armas, abordaje, impactos de torre,
finalización y la decisión de huida dependiente de velocidad. Rechaza también
proyectiles, cambios de terreno y otras condiciones no contrastadas. El
escritor independiente `serialize_single_active_attack` reproduce el bloque
de batallas con una escena activa y sin listas auxiliares; coincide byte a
byte con el guardado original después de cada uno de los dos ticks. Esta
escritura todavía no está conectada al guardado de sesión S05. `parse_single_active_attack` lee los
campos serializados de esa variante y vuelve a escribir el bloque original,
pero deja marcada la reconstrucción de la máscara. El cargador x86 aislado
recarga la partida activa y consume todo el fichero. En ese mismo caso,
`rebuild_loaded_active_attack` reconstruye el objeto, los primeros 0x164
bytes de cada barco y el índice espacial, iguales al original tras normalizar
las direcciones de proceso. `expand_loaded_town_ships` añade los 0x1004 bytes
transitorios de cada TownAttackShip y coincide con las dos colas de la captura
de recarga: mapa a cero y casillas de destino a 0xFFFF. La caché aún debe
recalcularse en el giro. No se habilita la recarga de
sesión ni el siguiente frame; estos contrastes no forman aún una cadena nativa
desde el reloj al guardado.

Para la apertura con un atacante y un defensor de IA se han separado dos
etapas anteriores a la colocación: `build_active_roster` crea los dos barcos
locales y las entradas de colisión; `prepare_active_wind` ajusta las listas,
los dos sorteos del viento y los contadores. En el escenario capturado desde
el reloj, ambas etapas coinciden con el original: objetos locales, enlaces,
índice espacial y objeto de escena. El original usa geometría hipotética.
`place_two_active_ships` reproduce la colocación 0x600060 de un barco por
bando, incluidos sus reintentos, semilla y rumbo inicial. Coincide con esa
captura completa y con 256 casos aislados de direcciones, semillas y consultas
de agua interceptadas. La segunda consulta del original usa la posición del
primer barco; se conserva ese comportamiento. La búsqueda se limita a 256
intentos para no reproducir un posible bucle infinito. La captura usa
geometría hipotética; los casos aislados no prueban la máscara real.
`town_attack_cell_is_water` reproduce 0x5FFE20 sobre el plano serializado;
528 consultas x86 cubren los 256 valores en ciudad y guarida y los bordes.
`place_two_ships_in_plan` enlaza ambas rutinas y coincide con la apertura
capturada usando el plano de la guarida y esa geometría hipotética. Todavía
falta publicar la escena y conectar los ticks y la persistencia a sesión.

`prepare_populated_roster` encadena esas etapas desde la imagen capturada al
final de construcción de listas: recrea barcos e índice, ajusta el viento y
coloca las naves. En la prueba, su salida alimenta los frames intermedios y
dos ticks mediante `advance_populated_ai_frame`, sin sustituir la escena por
la captura posterior. El frame admite el prefijo fraccional o un solo tick y
rechaza los casos de combate no contrastados. No genera aún el
objeto previo a ese límite ni la instala en sesión. La geometría sigue siendo
hipotética y solo se admite el caso acotado de un barco por bando.

La frontera se ha adelantado hasta la entrada de `0x5FBFE0`: Rust prepara la
escena para una guarida sin torres operativas, reconstruye su máscara y
listas, ajusta el viento y coloca ambos barcos. En la prueba de sesión, el
reloj llega a la tarea, `prepare_live_populated_pirate_scene` deriva la misión,
la banda y la flota vivas y prepara esta apertura sin publicarla. La apertura
y dos ticks coinciden con la ejecución x86 de la misma partida. Para el S05
del segundo tick se usa todavía una base original intermedia; sus siete
bloques coinciden y la recarga x86 del mismo contenido se contrastó por separado.
La geometría y la imagen inicial del asignador son entradas explícitas del
host. La prueba cubre un atacante y un defensor de IA, no su combate completo.

`stage_active_attack_save` prepara en una copia el bloque de batalla y los
cambios de campos persistidos de barcos, comprobando el estado anterior por
campo y rechazando conflictos o cambios no serializados. Para los dos ticks
anteriores, un S05 basado en el primer guardado original reproduce los siete
bloques del segundo y puede volver a abrirse con el lector Rust. El original
modificó `ship+0x30` de ambos participantes en el segundo tick; sustituir solo
el bloque de batallas no reproduce el guardado. Esta prueba parte de una base
S05 original intermedia. El cargador x86 aislado consumió el fichero producido
en Rust y reconstruyó la escena; objeto, barcos, colas y tabla espacial
coinciden tras normalizar punteros. Esto no demuestra aún la conciliación desde
el reloj ni habilita el guardado o la recarga de la sesión activa.

El mapa de navegación del barco dispone ahora de la búsqueda de casilla
abierta 0x625B00 (256 casos sintéticos contra x86) y del campo de distancias
0x6258A0 (24 máscaras completas contra x86, asignador adaptado). Se rechazan
mapas sin salida y el desbordamiento de distancia todavía no contrastado.
`refresh_town_navigation_map` y `update_town_navigation` reproducen la
actualización del mapa y las penalizaciones de costa de 0x625BF0, comparando
el barco completo en 32 escenarios sintéticos con x86 y asignador adaptado.
Se rechazan posiciones o destinos negativos y mapas no admitidos. Aún falta
conectar esta rutina al giro y a los objetos vivos del asalto; el contraste
aislado no demuestra el recorrido completo ni el efecto de las torres.

### Apertura poblada en sesión y recarga protegida (PC de Sergio)

La frontera del reloj publica atómicamente la misión y la escena de dos barcos de IA sin torres operativas. El gestor admite dos ticks contrastados; rechaza el tercero y conserva la operación pendiente sin mutar el estado. La preparación de S05 parte de la base anterior al asalto y se ha comparado por bloques con los serializadores x86, normalizando direcciones transitorias. El archivo producido se ha cargado en x86 y se han comparado escena y barcos reconstruidos. La lectura nativa reconoce el asalto y reconstruye la flota y el índice, pero mantiene el reloj bloqueado y no habilita el guardado general: faltan recursos y efectos completos.

Una captura x86 parcial del primer frame de recarga, con geometría sintética explícita, observa cuatro colisiones de torre (tipos 8, índices 4–7) aunque las cuatro vidas son cero y después la colisión de tierra (tipo 7, índice 8). El adaptador termina más adelante con `UC_ERR_READ_UNMAPPED` en `0x63A790`: esta captura no demuestra que el frame termine ni paridad en juego. Falta contrastar e integrar la reconstrucción de máscara y colisiones, giro, costa, torres, armas y resolución, conectar la recarga plenamente a Session y extender el recorrido completo de reloj a guardado y recarga. No se ha abierto el juego.

La reconstrucción provisional de la recarga reproduce la máscara, coordenadas y altas de cuatro torres y tierra, y está conectada a la escena viva de `Sim` en una operación transaccional. En el caso de geometría sintética de la captura parcial se compararon campos de la escena y las nueve tablas y tres matrices completas del índice espacial contra x86. La captura se tomó antes del fallo posterior del adaptador; no acredita el resto del frame, el giro o la resolución. El reloj y el guardado general siguen bloqueados.

La limitación anterior del adaptador se corrigió preservando los registros del cargador en el trampolín. En la misma partida y geometría sintética, el frame de delta cero regresa, el cargador consume el archivo entero y los objetos, barcos, tablas y matrices al retorno son idénticos a la captura tras reconstruir la máscara. La prueba acredita exclusivamente este frame sin avance; los ticks posteriores y la resolución siguen pendientes.
La captura x86 de delta 16 tras recargar también termina y devuelve el cargador, con cambios de barcos, escena y colisiones. Se dispone de ella para el siguiente contraste, pero el paso nativo aún la rechaza: los campos de torres cargados y la cola transitoria de los barcos no están conectados al ejecutor. No se acredita paridad del tick ni se desbloquea el reloj.

### Navegación recargada y prueba temprana de costa

El adaptador x86 captura entrada y salida reales de `0x625BF0` durante los frames de delta 16 y 32 tras cargar el S05 de la sesión, con geometría sintética explícita. En dos barcos y dos ticks se contrastaron las cuatro llamadas completas, incluidos mapas de distancia, penalizaciones y reutilización de las colas transitorias. `TownNavigationCaches` conserva esas colas por barco y las instala al reconstruir la escena en `Sim`; sus cambios son atómicos. **Todavía no está conectado el despacho de la llamada en el paso naval**, que reside en archivos de C07 reservados a Claude; el reloj y `Session::open` siguen protegidos.

La primera decisión de costa de `0x600770` (vecindad 3×3 antes del casco fino) se ha extraído y conectado al despachador de C10; coincide con el original en 384 escenarios sintéticos, incluidos bordes. La prueba no ejecuta ni acredita el contacto fino, daños, torres, armas ni resolución. Ninguna prueba es en partida.

### Proyectil frente a una pieza defensiva

Se ha portado `0x60117B`, la intersección del segmento del proyectil con el rombo de una de las cuatro piezas, incluyendo el redondeo de velocidad y el modo instantáneo. Coincide con x86 en 2.048 escenarios sintéticos con impactos y fallos, las cuatro piezas, velocidades extremas y ambos modos. El despachador de C10 dispone de la comprobación para rechazar los impactos reales hasta implementar sus efectos; la validación general de escena todavía rechaza proyectiles, por lo que **la ruta aún no está habilitada en una batalla viva**. Faltan aplicar daño, retirar el proyectil, actualizar la pieza y S05, y los demás efectos de torre, costa fina y resolución. No hay prueba en partida.
La rama no letal de `0x60130B` para guaridas piratas se ha portado aparte: si la vida es mayor que el daño, resta con aritmética x86 y conserva el resto de la escena. El callback pirata `0x515D90` es un retorno sin efectos. En 256 escenarios x86, con y sin pieza en la lista y con operandos extremos, coincide la imagen completa; el caso de destrucción se rechaza sin cambios. Esta rutina aún no está conectada al impacto ni a S05.

### Destrucción de pieza pirata preparada

La rama letal de `0x60130B` prepara ahora una copia de la escena, el índice espacial que debe retirarse y, cuando encuentra la pieza, los campos de la tarea de derribo `0x25`. El original también programa esa tarea para una pieza pirata coincidente. En 160 escenarios x86 se compararon escena, identificador espacial y campos de tarea; el asignador de tareas y la retirada espacial se interceptaron para observar esos efectos. La copia aún no se confirma en el planificador ni S05, y los casos con vista o hotseat se rechazan. No demuestra el impacto completo ni el resultado en partida.

La preparación agrupa ahora copias de escena, índice espacial y planificador, sin mutar las entradas: valida el hueco de torre y las listas del planificador, retira la colisión y construye la tarea si corresponde. Se han probado confirmación en copia y rollback ante ranura ajena, lista cíclica o falta de capacidad libre. La comparación x86 previa intercepta asignador y retirada; **la transacción encadenada todavía no se ha contrastado íntegramente con x86 ni se publica en Sim o S05**.

La retirada espacial y la inserción en el planificador se han ejecutado ahora sin interceptarlas: en 16 escenarios x86 encadenados coinciden la escena, todas las tablas y matrices espaciales y los 128 registros del planificador. Solo se adapta `realloc`. Esta prueba aún no incluye el proyectil, Session ni S05.

### Impacto pirata encadenado en copia

`stage_pirate_tower_impact` prepara el contacto de un proyectil sin tirador ni recurso gráfico con una pieza pirata sin vista ni hotseat. Si falla conserva las copias; si acierta prepara el daño, el posible derribo y tarea, y la retirada del proyectil y de su colisión. En 18 escenarios sintéticos una sola ejecución x86 de `0x5FF8E0` coincide con Rust en escena, bitmap, ranuras, tablas y matrices espaciales y los 128 registros del planificador, con y sin impacto y con daño letal o no letal. Se adaptan `realloc` y la liberación CRT; no se prueban presentación, tiradores identificados, guardado, recarga ni partida. La transacción aún no se publica en el gestor de `Sim` ni elimina el guard general de proyectiles.

La transacción sin vista, hotseat, tirador ni recurso gráfico puede publicarse ahora explícitamente en el gestor de `Sim` con `apply_pirate_tower_impact`: confirma juntos escena, índice, proyectil y planificador. En tres estados sintéticos instalados tras cargar S05 coinciden con x86 fallo, daño no letal y derribo, y se verificó rollback si falta espacio de tareas. El método todavía no se invoca automáticamente desde el frame; el guard de proyectiles sigue activo y la prueba no acredita el serializador S05 posterior ni `Session::open`.

### Base S05 de asalto recargado

Una batalla urbana ya cargada puede capturarse como base privada antes de sus efectos. La base se acepta solo si el bloque de batalla se reserializa exactamente, el índice coincide y la conciliación sin cambios reproduce el S05; se rechazan archivo o escena ajenos. Tras un impacto sintético de torre se prepara S05 con la tarea de derribo o la vida no letal, y ambos se recargan en Rust. Para el derribo, el cargador x86 consumió el archivo completo y coinciden la escena, barcos y tarea concreta; se utilizó geometría sintética y un proyectil inyectado, y no se comparó el escritor x86 completo. El guardado general y `Session::open` siguen protegidos; faltan despacho automático, otras clases de impacto y paridad completa.
El cargador x86 también ha consumido íntegramente el S05 no letal, con vida persistida 95; escena y barcos reconstruidos coinciden con Rust. El letal conserva además la tarea de derribo cotejada byte por byte. Ambos escenarios parten de un asalto cargado y un impacto sintético; no comparan el escritor x86 completo ni prueban la partida.

### Reescritura después de cargar el asalto

El lector x86 invierte las entradas enlazadas de suministro urbano en cada carga. La preparación S05 desde una base activa reproduce esa inversión con los registros completos y rechaza cambios de suministro aún no conciliados. En los casos letal y no letal sintéticos se han comparado seis de los siete bloques con el escritor x86, desde la misma base para el mundo y tras recarga para batalla y tareas; se normalizaron solo punteros documentados. El bloque de barcos todavía difiere en siete bytes: durante la carga `0x5048C0` llama a `0x517C10` y replantea una ruta inválida. La rama acotada de ruta propia horizontal, con carga normalizada, produce el mismo registro de barco que x86 usando recursos reales; **aún no se aplica al guardado general ni cubre rutas compartidas y otros casos**. No existe paridad del S05 completo.

### Normalización naval acotada en S05

La decisión de `0x517C10` de reparar una ruta se ha contrastado en 1.024 casos sintéticos x86. La preparación de un asalto recargado construye la tabla de rutas con los recursos originales, revisa rutas propias y compartidas y aplica la rama acotada de ruta propia horizontal ya contrastada; una reparación de otra clase, una flota modificada, recursos ausentes o valores fuera del alcance se rechazan sin publicar. Para el guardado de referencia corrige la ruta del barco 0 y el bloque naval completo coincide con el escritor x86 tras normalizar punteros de ruta.

En los dos impactos sintéticos letal/no letal, los siete bloques preparados coinciden bajo este protocolo: el mundo se compara con el escritor x86 de la **misma base antes del impacto**, y los otros seis con una reescritura x86 tras recargar el resultado; se normalizan punteros documentados. El cargador original consume ambos archivos y se comprueban escena, barcos y tarea letal. **No es una única ejecución x86 desde el reloj hasta el impacto y el guardado**, ni cubre rutas que requieran otras ramas, Session, costa fina o combate completo.

### Apertura diagnóstica del asalto cargado en Session

`Session::open_with_paused_pirate_attack` reconstruye el caso admitido de
asalto pirata poblado **sin torres operativas**, con estado CRT explícito;
captura la base de S05 y permite preparar una copia experimental sin confirmar
el guardado ordinario. El reloj, navegación, comercio, construcción y cambio de
oficina permanecen protegidos. El indicador de batalla pausada está activo,
pero el combate no se presenta como listo para avanzar.

La prueba `c10_pirate_session_resume` compara la escena y los barcos de la
recarga con el cargador x86 del ejecutable inglés, verificando hashes de
partida, ejecutable y adaptadores; normaliza direcciones de proceso y comprueba
que cada barco apunta a su gestor. Comprueba una segunda carga S05 y los
rechazos de operaciones incompatibles. Otros dos guardados sintéticos con
torres operativas se rechazan: la geometría y la ejecución aún no están
reconciliadas. El contacto fino con costa, disparo y resolución completos, el
frame automático y el guardado general siguen pendientes. No es una prueba en
partida ni un contraste encadenado completo desde el reloj.


### Ventana fina de costa: acumulación aislada

La parte de `0x600770` que recorre 5 × 9 casillas terrestres y acumula los
pesos direccionales de `0x67AA70` y el escalado posterior coinciden con x86 en 175 casos sintéticos,
incluidas las 45 posiciones individuales y muestras combinadas. El adaptador
inyecta proyección, consulta de agua y contacto del casco, y detiene el original
antes de aplicar la respuesta al barco. Los pesos se leen del EXE inglés
verificado. La función Rust no está conectada al frame y no acredita todavía la
consulta de casco `0x601C7B`, proyección, efecto físico, daños o persistencia.


### Casco y ventana fina encadenados

La consulta `0x601C7B` se ha implementado con contorno de cinco vértices y
comparado con x86 en 4.000 casos sintéticos, con las cuatro salidas cubiertas:
fuera de mapa, centro interior, cruce de bordes y ausencia de contacto. Los
contornos y senos proceden del EXE inglés; proyección y tipo se inyectan.
Una segunda prueba encadena el agua real `0x5FFE20`, el casco y la acumulación
de `0x600770` en 256 escenarios de guarida; solo proyección y acceso al tipo
están adaptados. `town_coast_window_from_plan` enlaza los puertos Rust en el
mismo caso. Aún falta conectar proyección de recursos y respuesta física al
frame, y contrastar la ruta completa con Session y S05; sin prueba en partida.


### Respuesta física al contacto de costa

`0x6029B0` y la respuesta de `0x600770` están portadas con aritmética x86.
El ángulo coincide en 8.336 vectores, incluidos extremos; 1.024 escenarios
encadenan en una llamada x86 agua, casco, acumulación, ángulo y respuesta,
y comparan el barco completo con Rust. Se cubren ambos modos y más de 100
contactos efectivos. El adaptador aporta proyección y acceso al tipo de casco,
usa los datos originales de tablas y omite la rama de sonido. El frame vivo
todavía rechaza el contacto y no se ha probado Session/S05 de esta ruta.


### Contacto explícito en Sim y Session

`apply_town_coast_contact` prepara el efecto sin mutación parcial y rechaza la
rama de sonido aún no conectada. `Sim::apply_pirate_coast_contact` lo publica
en una escena cargada con geometría reconstruida, y Session lo expone como
operación explícita. En un escenario sintético, el barco completo coincide con
x86 antes de guardar y sus 47 bytes serializados se conservan en S05 y en la
recarga Rust. La proyección y las tablas las suministra el llamante; no se ha
contrastado el escritor x86 para este recorrido. El frame automático sigue
rechazando contacto, y las torres, armas y resolución no están terminadas.


### Cargador y escritor x86 del S05 de costa

El cargador original consumió el S05 experimental de Session tras el contacto
sintético. La escena y los barcos recargados coinciden. El bloque de batalla y
los otros seis bloques coinciden con el escritor x86 usando el mismo protocolo
mixto de la prueba de torre: el mundo se compara con la escritura de la base
antes del contacto, y los demás con la reescritura tras recargar el resultado;
se normalizan punteros documentados. Se verifican hashes de guardados, EXE,
payload y adaptadores. La proyección y el tipo se adaptan en la ejecución de
contacto; no es una sola ejecución x86 desde el reloj ni habilita el frame.


### Proyección y tablas originales desde Session

`Session::apply_paused_pirate_coast_from_exe` verifica el hash del EXE inglés,
lee sus tablas y usa la geometría del CPR (casilla 34×21, desplazamiento
17,11) y la máscara de detección contrastada con `detect.aim`. La ruta
acotada admite mapa 128×256; otras geometrías se rechazan. En 256 escenarios
sintéticos se ejecutan en x86 la proyección, consulta de agua, casco y
respuesta y coinciden los barcos; el acceso al tipo se adapta y no hay sonido.
Un asalto pausado en Session también coincide con x86 y conserva los campos
serializados al recargar en Rust. Sigue siendo una llamada explícita: el frame
automático, sonido, torres, armas, resolución y cadena desde reloj no están
completados.


El S05 de ese caso con proyección original también fue consumido y reescrito
por x86. Coinciden escena y barcos al cargar, y los siete bloques bajo el
protocolo mixto: mundo desde la base anterior y los demás desde la reescritura
posterior, con los punteros documentados normalizados. El tipo de barco sigue
adaptado en el contraste del contacto, se omite sonido y no se ejecuta el
frame automático ni el reloj completo.


### Apertura diagnóstica con torres operativas

Session puede reconstruir en pausa los dos S05 sintéticos tras impacto letal
y no letal aunque la banda conserve torres operativas. Las escenas y barcos
coinciden con las capturas del cargador x86, normalizando direcciones del
gestor, y se puede preparar S05 experimental. El frame con torres operativas
se rechaza explícitamente y la prueba comprueba que no muta la escena; los
disparos, efectos, resolución y guardado general siguen sin completarse.


### Recarga de piezas defensivas

La recarga y el retorno sin atacante de `0x5FE080` coinciden con x86 en 2.048
casos (cuatro piezas, 256 contadores y dos modos). Sim y Session pueden aplicar
la recarga explícitamente; dos casos confirman el contador en S05 y recarga
Rust y comprueban rollback al pedir selección de blanco aún no implementada.
El frame sigue rechazando torres operativas y falta selección, disparo, efectos
y contraste x86 del escritor S05 para esta ruta.


Con recarga cero, el recorrido de atacantes hundidos o huidos coincide en
512 listas x86. Session admite explícitamente ese retorno sin disparo; listas
cíclicas, truncadas o un atacante elegible se rechazan sin mutación. No se ha
implementado aún la selección de un blanco elegible ni se habilita el frame.


La lista de atacantes admite además barcos inmóviles fuera del alcance
cuadrático `0x7E900`, con distancias y retorno comparados en 256 escenarios
x86. El cálculo original se ejecutó sin interceptar la distancia. El puerto
rechaza overflow, movimiento y blancos dentro de alcance; la predicción, el
disparo y los efectos siguen pendientes y el frame permanece protegido.


El alcance predicho de atacantes móviles se contrasta con x86 en 4.096
escenarios. Cada una de las cuatro iteraciones recalcula desde el vector
inicial; se rechaza un objetivo dentro de alcance o una distancia desbordada,
por lo que aún no se selecciona ni dispara a ningún blanco. Un S05 con la
recarga actualizada fue cargado y reescrito por x86: escena/barcos y siete
bloques coinciden con el protocolo mixto documentado y punteros normalizados.
No se ha habilitado el frame de torres ni ejecutado un disparo completo.


`town_defense_select_unopposed` selecciona el blanco más cercano y su punto
predicho cuando no hay defensores; coincide con `0x5FE080` en 1.024 listas
sintéticas. La captura x86 se detiene antes de daño y disparo. La consulta
rechaza defensores, recarga y listas inválidas; aún no está conectada al frame.


La preparación de disparo instantáneo sin defensores ni participantes humanos
coincide con x86 en 2.048 escenarios: blanco, daño calculado y recarga. El
oráculo intercepta `0x61F96F`; no aplica ni persiste daño. Con defensores,
participantes humanos o proyectiles, el puerto rechaza la ruta. El frame sigue
protegido y la resolución de batalla continúa pendiente.


La preparación transaccional del disparo instantáneo y el daño real de
`0x61F96F` coinciden en 128 escenarios x86 encadenados sin efectos externos:
escena, barco local y barco mundial completos. El sandbox adapta servicios
externos; la ruta rechaza cualquier evento que requiera conciliación. Aún no
se publica en Sim ni S05: los cambios navales posteriores a la carga necesitan
conciliarse con la normalización de rutas. Persisten límites de participantes,
defensores, proyectiles, hundimientos y frame.


### Disparo instantáneo explícito y orden de rutas

Sim y Session publican explícitamente el disparo instantáneo sin defensores,
humanos ni efectos externos. La escena, barco local y barco mundial coinciden
con una ejecución x86 encadenada sobre las mismas imágenes sintéticas. El S05
resultante lo cargan y escriben Rust y x86: escena/barcos y siete bloques
coinciden bajo protocolo mixto (mundo de la base anterior, seis bloques tras
recarga y punteros documentados normalizados). No es una ejecución única desde
el reloj. La reparación de rutas se hace con la vida anterior al daño y luego
se aplican vida, suciedad y tripulación; el test demuestra que invertir el
orden cambia la duración. Otros cambios navales se rechazan. Los hundimientos,
eventos externos, dificultad humana, defensores, proyectiles y frame siguen
protegidos; no hay prueba en partida.


### Torre instantánea: dificultad humana (PC de Sergio)

La escala de `0x5FE080` para participantes humanos se aplica en orden de bando, con multiplicación y truncado x86. Coincide en 3.072 escenarios de selección y preparación, con cobertura de ambos bandos, los extremos de dificultad y dos participantes humanos. La consulta de comerciante está adaptada en ese censo. En una sesión sintética con escala humana efectiva se ejecutan selección, consulta real del comerciante, disparo y daño x86; los cambios de escena y barcos coinciden con Rust y el daño se conserva en S05 preparado. La escritura y recarga x86 de ese caso humano no se han contrastado. Sin defensores ni eventos externos; no habilita el frame ni equivale a un recorrido continuo desde el reloj. No se ha probado en partida.


El S05 del caso humano y su base anterior pasaron carga y escritura x86: escena, barcos y siete bloques coinciden con el protocolo mixto tras normalizar punteros y una inversión de dos nodos de 16 bytes en la cola de la ciudad 9. Se comprobó por separado que el disparo no modifica el mundo ni en Rust ni en x86. Esa inversión se observa al añadir una carga original; no se ha corregido el orden del serializador ni se acredita igualdad byte por byte sin normalizar.

La selección con defensores de `0x5FE080` coincide en 2.048 escenarios x86, con bloqueos y tiros permitidos. El original usa la componente X dos veces al calcular el primer ángulo; Rust conserva ese comportamiento observado sin atribuirle intención. Sim y Session aceptan ahora el disparo instantáneo con defensores y tabla angular del EXE verificado: dos escenarios de sesión, uno bloqueado y otro no bloqueado, coinciden con selección y daño x86; el segundo conserva el daño en S05 Rust. No se han contrastado cargador ni escritor x86 de esos casos con defensores, y faltan proyectiles, efectos, frame, resolución y ejecución encadenada desde el reloj. Sin prueba en partida.


La pérdida de equipo por el disparo instantáneo y el recálculo de `0x5182B0` coinciden con x86 en 256 escenarios; una Session sintética ejecuta el disparo y el recálculo originales, coincide con Rust y prepara S05 con el equipo dañado. La reparación de rutas usa el equipo anterior y valida el recálculo y los cambios guardados, rechazando cambios ajenos. Los derivados `ship+0x118..0x123` no se serializan. La carga y escritura x86 de este S05 están en curso; siguen faltando hundimientos, bajas espaciales, proyectiles, frame, resolución y cadena reloj–S05. No se ha probado en partida.

El cargador y escritor x86 completaron después la base y el resultado con pérdida de equipo. Coinciden escena, barcos y siete bloques bajo el protocolo mixto con normalización de punteros y de los dos nodos urbanos. Las pruebas verifican que el mundo no cambia entre base y resultado en cada implementación. La inversión de nodos reproduce la inserción en cabeza del cargador: la comparación usa una carga adicional y no demuestra una divergencia de serialización.


El hundimiento sin aviso externo aplica de forma transaccional las dos bajas espaciales y sincroniza la cabecera de escena. Coincide con x86 en 128 escenarios, incluidos ejes, listas libres y matrices; se rechazan índices o cabeceras inconsistentes. Un caso de Session coincide con selección, daño y bajas espaciales originales; el S05 resultante y su base pasaron cargador y escritor x86. Escena, barcos y siete bloques coinciden bajo el protocolo mixto, con punteros y dos nodos de suministro normalizados. No incluye noticia de hundimiento, proyectiles, frame, resolución ni recorrido continuo desde el reloj; sin prueba en partida.


El hundimiento de un barco pirata por torre publica también la noticia global. En una Session sintética se ejecutan selección, hundimiento y bajas originales; la entrega `0x4D6530` se intercepta y coinciden destinatario y campos semánticos de la noticia con Rust. La inserción en el depósito y el S05 se prueban por separado: base y resultado pasan cargador/escritor x86 y los siete bloques coinciden bajo el protocolo mixto con las normalizaciones documentadas. La sesión rechaza depósitos inconsistentes sin publicar el disparo. Faltan proyectiles, ejecución automática del frame, resolución y una cadena única desde el reloj. Sin prueba en partida.


El guardado con defensores y tiro permitido y su base también pasan cargador y escritor x86; coinciden escena, barcos y siete bloques bajo el protocolo mixto con las normalizaciones documentadas.

Para el tiro no instantáneo, 3.072 preparaciones coinciden con los argumentos de `0x602A90` interceptado; 128 cadenas ejecutan el constructor original y coinciden proyectil, escena, RNG CRT, bitmap e índice espacial. El asignador se adapta y se usa una ranura libre; el índice debe tener capacidad. Rust prepara los cambios transaccionalmente en copias, pero aún no los publica en Sim/Session ni los guarda. Faltan avance, impactos, frame y resolución; sin prueba en partida.


Sim y Session crean y avanzan explícitamente el proyectil de torre sin despachar colisiones; 16 trayectos x86 hasta retirada coinciden paso a paso, y una Session compara escena, proyectil, RNG e índice con la misma cadena original. El S05 con proyectil activo y su base pasan cargador y escritor x86 y coinciden en siete bloques bajo protocolo mixto; el resultado tras retirada se vuelve a leer en Rust. El cargador Rust aún no reconstruye la escena viva con proyectiles, no se ejecutan impactos ni frame automático y falta resolución y recorrido continuo desde el reloj. Sin vista, hotseat ni prueba en partida.


La apertura diagnóstica reconstruye proyectiles de torre desde S05 con semilla CRT explícita; consume las dos llamadas originales por proyectil y confirma el RNG solo si la reconstrucción tiene éxito. Una ejecución x86 carga el S05 activo, avanza el proyectil hasta retirarlo y escribe el resultado: Rust coincide en escena, barcos, proyectil, RNG, índice, cada paso y los siete bloques finales, con punteros documentados normalizados. Al cargar, la etiqueta transitoria pasa de `0x1C` a `8`, como en el original. La entrega no despacha colisiones, no integra el frame ni la resolución y no parte del reloj. Sin vista, hotseat ni prueba en partida.


El contacto explícito entre proyectil de torre y barco consulta el casco real y aplica daño, retirada espacial y noticias en copias antes de publicar en Sim/Session. En 32 trayectorias x86 coinciden los pasos sin contacto y los impactos; ocho hunden el blanco. Una sesión sintética coincide con la ejecución original hasta el impacto y prepara S05. El escritor y cargador x86 de esta partida están en curso. La pareja la suministra el llamante: aún no la despacha el frame, y faltan resolución y recorrido completo desde reloj. Sin vista, hotseat ni prueba en partida.

El S05 después del impacto de proyectil contra barco y su base pasan cargador y escritor x86; los siete bloques coinciden bajo el protocolo mixto y sus normalizaciones. El despacho sigue siendo explícito y falta contrastar impacto tras recarga, frame y resolución.


Una partida sintética con modo no instantáneo se carga en una sola ejecución x86, avanza el proyectil, despacha cada pareja barco–proyectil, aplica el impacto y escribe S05. Session coincide en cambios de escena y barco, proyectil e índice en cada paso, y en los siete bloques finales con punteros documentados normalizados. El escenario usa semilla CRT explícita y despacho manual de parejas; no ejecuta el frame completo ni parte del reloj. Sin vista, hotseat ni prueba en partida.
# Fotograma fraccional con torres (PC de Sergio)

El prefijo de `0x005FD510` admite torres operativas si la fracción acumulada
no llega a 16. El oráculo `x86_c10_fractional_towers.py` contrasta las 136
combinaciones posibles de fracción y avance hasta ese límite, con escena y
torres sintéticas; compara todos los bytes de la escena con Rust. La prueba
de Session prepara geometría, avanza una fracción, comprueba que el tick
completo sigue rechazado y ensaya la escritura y reconstrucción S05. El
acumulador `+0x130f` es transitorio y la reconstrucción lo inicia a cero.
Esto no conecta las torres ni las colisiones al tick completo ni demuestra
paridad de una batalla o de una partida.

Un tick adicional de geometría sintética contrasta cuatro descriptores de
pieza distintos de cero pero con vida `+0x1ec4..+0x1ed3` a cero. El original
no actualiza esas piezas; Rust coincide en escena, barcos, flota e índice y
ya no bloquea ese caso. Los contornos de esas piezas no se instalaron en el
índice y el conductor x86 queda deliberadamente incompleto: no acredita una
batalla ni una transacción S05 completa. Se rechaza con rollback cualquier
pieza con vida. En otra captura exploratoria con vida positiva el original
actualiza las estadísticas defensivas y descuenta las recargas dentro del
tick, antes de avanzar proyectiles y despachar parejas; esas estadísticas y
el disparo todavía no están conectados al ejecutor del asalto.

`town_defense_statistics` reproduce aisladamente el bloque de estadísticas de
torres del original: 1.024 estados sintéticos con límites y desbordamientos,
más la captura anterior y posterior de ese bloque durante un tick con cuatro
torres vivas, coinciden con x86. El divisor y factor se toman como parámetros;
en la captura valen 96 y 30. El conductor del tick permanece incompleto y la
operación aún no se inserta en el frame ni acredita Session o S05 completos.

Las operaciones diagnósticas de recarga, disparo y creación de proyectil
verifican ahora la vida de la pieza, como el bucle que las invoca en x86.
Una pieza sin vida se rechaza antes de publicar escena, índice o RNG. Pasan
las pruebas de recarga y disparos con la partida local; las rutas de
proyectiles también se repitieron ejecutando el oráculo x86 aislado. Esto no
conecta automáticamente esos callbacks al tick.

La captura `x86_c10_reload_tower_wait.py` carga la partida local de torre,
inyecta recarga 32 y ejecuta un frame original de 16 con el índice espacial
reconstruido y geometría hipotética. La pieza pasa de vida 95 a cero dentro de
`0x0060130B` (escritura observada en `0x006015BD`), aunque al empezar el tick
estaba en recarga. La llamada procede del paso de barco de asedio en
`0x006252A9`; la recarga impide el fuego de la torre, no los ataques que
recibe. El original ejecuta navegación y daño a pieza en ese mismo
frame. La captura no compara todavía Rust, no serializa el resultado y no
acredita la batalla completa. El prototipo temporal que reproduce un caso
sintético sin índices de torre se conservó en el checkpoint local
`20261108T190333957902Z`; no está habilitado ni publicado como implementación.

La operación explícita de daño de barco a torre en Sim/Session publica
atómicamente escena, índice y planificador, para daño no letal o derribo.
El daño y derribo ya se contrastaron aisladamente contra x86, y el archivo
preparado tras daño y derribo pasa cargador y escritor x86: coinciden los
bloques de batalla y tareas, con punteros temporales de misión normalizados.
La entrega la inicia el llamante en pausa diagnóstica; no ejecuta el barco,
el frame o el reloj. La base equivalente y el resultado pasan también el
escritor x86: coinciden los siete bloques bajo protocolo mixto, normalizando
punteros temporales y compensando la inversión de dos nodos de ciudad tras
comprobar por separado los mundos base y resultado de ambos lados.

El plan y ejecución de las armas instantáneas de un barco de asedio se
conectan en Sim/Session en pausa para disparos dirigidos solo a torres
piratas, sin vista ni hotseat. En una llamada real dentro de un frame x86
recargado coinciden selección, barco, escena, RNG de batalla, índice y
planificador; la posición de la torre se fijó sintéticamente tras reconstruir
el índice, de modo que su contorno espacial no se recolocó. El S05 de Session
pasa cargador y escritor x86 para los bloques de batalla y tareas, con los
punteros de misión normalizados. Las andanadas, objetivos barco, avisos,
geometría coherente y despacho automático del frame quedan pendientes.

En la captura sin recolocar la torre el cuadrado de distancia desborda y la
CRT original entrega longitud cero. Se reproduce esa rama acotada para dos
barcos, una torre viva, sin otro blanco elegido y con política y ángulo
verificados. Coinciden disparo, escena, barco, índice espacial reconstruido y
planificador; las otras formas de desbordamiento se siguen rechazando. La
geometría del escenario continúa siendo hipotética y la operación se invoca
explícitamente desde la sesión, sin ejecutar el frame entero. El S05
resultante pasa el cargador y escritor x86 en los bloques de batalla y tareas,
normalizando punteros temporales de misión; los otros cinco no se compararon.

El paso explícito de un barco de asedio ahora conserva su cola de navegación,
ejecuta movimiento y armas y publica juntos escena, flota, índice y tareas en
Sim/Session. Dos capturas x86 tras cargar coinciden en barco completo y caché,
escena, flota, índice y planificador, una con torre reubicada sintéticamente
y otra sin recolocarla; ambas usan geometría hipotética. La velocidad mundial
del barco se concilia con la ruta solo para barcos presentes en la escena.
El S05 de la segunda muestra pasa cargador/escritor x86 para batalla, tareas
y velocidad, con punteros temporales normalizados. El llamante debe situarse
en el punto anterior al barco: no se ejecuta el frame completo ni se cubren
automáticamente los demás barcos, abordajes, efectos visuales o resolución.

En la muestra sin recolocación se encadenan explícitamente atacante y defensor
en Session: tras cada paso coinciden escena, barcos con sus cachés e índice
con el mismo frame x86. El segundo no dispara en esta muestra. El S05 después
de ambos pasa cargador y escritor x86 en batalla, tareas y velocidad de los
dos barcos, con punteros temporales de misión normalizados. Las parejas
espaciales que siguen a esos pasos, el frame completo y la resolución todavía
no se publican automáticamente; la geometría continúa siendo hipotética.

El despacho explícito posterior ordena el índice y procesa parejas urbanas
en copias. En la misma muestra de dos barcos, las 30 parejas candidatas
dejan escena, barcos, mundo e índice iguales a x86; ninguna produce contacto
de costa y el S05 es idéntico al ya contrastado tras ambos pasos. Otra prueba
provoca una pareja barco–costa con plano sintético: la consulta de casco y
respuesta coinciden con x86 y el barco se conserva al preparar y recargar
S05 en Rust. Para altura de plano 226 solo se admite el descarte rápido sin
contacto; la proyección fina contrastada se limita a 128×256. Proyectiles,
daños o efectos externos no conectados se rechazan transaccionalmente. No se
ha conectado este despacho al frame automático ni probado en partida.

Un frame acotado en pausa se compone ahora en una sola transacción:
estadísticas, viento, dos barcos con caché, armas, recargas y parejas. En la
captura x86 sin recolocar la torre coinciden escena, barcos completos, flota,
planificador e índice desde el estado posterior a reconstruir la geometría
hasta volver del frame; no hay contacto costero en esa muestra. Session
produce exactamente el mismo S05 ya cargado y escrito por x86 tras los dos
barcos, con sus límites de bloques documentados arriba. La geometría es
hipotética, la operación se llama explícitamente en pausa y no enlaza todavía
el reloj. Se rechazan proyectiles, disparos automáticos de torre, fin de
batalla y otras ramas no conectadas; no acredita paridad en partida.

Dos variantes adicionales reubican la torre sintéticamente fuera de alcance:
con recarga 32 el original y Rust la reducen a 16; con recarga cero y ningún
blanco en alcance permanece lista sin disparar. Coinciden escenas, barcos,
cachés, flota, índice y planificador del frame x86. El S05 de la primera pasa
cargador/escritor x86 en batalla y tareas. Si hay blanco al alcance con la
torre lista, el frame rechaza la transición y conserva escena, caché, flota
y planificador; ese disparo automático aún no está integrado. En estas dos
muestras se cambia la posición de la torre sin recolocar su contorno en el
índice, por lo que no acreditan su colisión ni una geometría real.
