# C10: aceptación del ejecutable de simulación

PC de Sergio, 4 de noviembre de 2026. **Reanudado: no aceptado todavía.**

El contrato exige cargar una partida original, avanzar 30 días, persistir y
poder comparar ese recorrido con el original. Los contrastes de rutinas por
separado no sustituyen la comparación del estado integrado.

## Perfiles reproducibles

- Ejecutable inglés SHA256 `fe436dc5f8addc4d3aa437eb48a8bc6415d9ba5577c1027d06db9bb915eddece`.
- Partida de contraste `visibility/save/Ein/AUTO.pat`, SHA256
  `0263531c207f437aab09c4d3c01eb8a4d5ccffc1198d94223eddb2ee364133b5`.
- Jugador 36; anfitrión local; CRT inicial 123; fiesta `(0,255)`; reloj inicial
  125740230. Se leen de la partida los ajustes de comercio, población, combate
  y piratería, y los recursos reales del CPR inglés.
- El avance sin pantalla pausa el mundo al encontrar una batalla de IA, la
  resuelve en pasos de 16 y reanuda el reloj. El oráculo debe aplicar el mismo
  protocolo externo: avanzar solo `WorldClock` mientras Rust resuelve batallas
  no constituye una comparación equivalente.
- El `.pat` no contiene CRT, marca de noticias ni todo el estado del proceso.
  Las continuaciones deben conservar esos valores explícitos; el `.pst` es
  un archivo distinto. La CLI documenta las opciones en `p3-mvp/C10_CLI.md`.

## Evidencia alcanzada

`oracle/x86_c10_real_save_clock.py` carga el guardado mediante los lectores
originales, conserva su heap y ejecuta reloj, ciudades, oficinas, tareas,
flota y serializadores x86. Se declaran las adaptaciones de archivos, INI,
memoria, servicios de un hilo y presentación. No se ha abierto el juego.

`p3-mvp/examples/c10_snapshots.rs` exporta la carga, el primer tick y cada día
nativo. `oracle/compare_c10_real_snapshots.py` compara ocho bloques enteros.
Solo excluye direcciones de proceso acreditadas: puntero de ruta del barco,
dos punteros de la tarea 0x1B y punteros de cargas inline de cartas. Conserva
listas, payloads, contadores, orden y todos los valores numéricos. En modo
directorio exige el mismo conjunto de muestras en ambos lados.

Carga, primer tick y días 1–6: **8/8 bloques coinciden**, con cero diferencias
numéricas. Referencia original de 30 días en `oracle/evidence/c10_real_save_30days.json`;
exportación local `coordination-runtime/c10-native-expedition30`.
Corregidos mediante ese contraste: inversión de entradas urbanas al cargar,
lectura de dificultades reales, captura estadística aun sin PST de entrada,
apellidos de préstamos y publicación inmediata del reparto de viviendas.

El corpus `p3-mvp/tests/c10_corpus.rs` recorre nueve partidas locales, dos
veces cada una, 30 días más guardar, recargar y continuar otro día. Comprueba
repetición del guardado, determinismo y hashes de entrada, ejecutable de prueba
y fuentes compiladas. La ejecución histórica de nueve de nueve
(`coordination-runtime/c10-corpus-letters-integrated.json`) precede a la
integración de bandas. Con esa fase conectada, la ejecución posterior completa
seis de nueve: dos partidas encontraron el callback `cv_free` sin conectar y
una no reconstruía la ruta de un barco al reanudar
(`coordination-runtime/c10-corpus-b5-names-integrated.json`). Corregidos ambos
casos y conectada la fase inicial del retorno, el corpus posterior pasa las
nueve partidas, con dos ejecuciones, doble guardado, recarga, un día adicional
y comprobación de cursores (`coordination-runtime/c10-corpus-return-start.json`).
Esta ejecución aún no cubre la fase final del retorno. Las clases de carta
de WareVerstecken y los comandos 0x44/0x11 ya están contrastados e integrados.
También están conectados 503B50/534F10 (640 casos x86) y las operaciones
1E/1F (192 casos x86), con prueba de cola real, recursos, dinero, rutas,
contador, puntos, rollback por recurso ausente y S05.

La referencia previa de batalla se invalida: dejaba el ancho de baldosa en
cero. Con `scripts/iso.ini` y `Class22+0x510=34` durante la generación original,
el sello preliminar `coordination-runtime/c10-b5-names-preliminary-seal.json`
acepta las 32 muestras de AUTO con la fase de bandas y la corrección de nombres:
ocho bloques, CRT y resúmenes coinciden. Incluye tres batallas x86 y nativas.
La referencia original se repitió desde carga completa y sin caché, con las 576
rutas y los 32 hashes de muestra idénticos a la referencia anterior. Debe
repetirse el sello nativo después de corregir los fallos del corpus. El sello
anterior acreditaba el binario y cinco fuentes explícitas. Una compilación
limpia y el grafo de entradas locales de rustc también pasaron en
`coordination-runtime/c10-seal-stage0.json`; ambos sellos preceden a la fase
final del retorno y deberán repetirse sobre las fuentes definitivas.

La fase `0x00514000` y la operación B5 están conectadas al reloj, la cola y S05.
Una B5 capturada en el reloj original reproduce exactamente 77 cambios en el
bloque de barcos; los otros seis bloques comparados no cambian. La comparación
es de las diferencias producidas por esa operación, no de fotografías de carga
íntegras. El crecimiento de 32 convoyes se contrasta en 128 casos x86 y tiene
pruebas nativas dirigidas de conflicto, doble guardado y recarga. El contraste
aislado de recarga urbana coincide en 24 de 24 ciudades. Los perfiles de prueba
no acreditan todos los estados posibles de heap ni sustituyen el corpus completo.

## Progreso posterior y criterios pendientes

La fase final de la expedición 0x12 está conectada al reloj, el comercio, las
cartas y S05. Se contrastaron seis recorridos dirigidos con el manejador y los
serializadores x86: descubrimiento, venta, compra, contrato existente,
contrato nuevo y aldea. Una prueba de reloj usa un contrato sobre una partida
real modificada, guarda y recarga centro, comerciante, cartas, barco, convoy y
ruta; otra verifica dos expediciones con un guardado intermedio. Estas pruebas
son complementarias: no constituyen una única ejecución x86 y Rust en partida.
Para la aldea, el sufijo de la carta se suministra explícitamente; el EXE inglés
headless lo mantuvo vacío durante una carga sin caché y 30 días, sin demostrar
que todas las sesiones gráficas compartan ese valor.

El sello `coordination-runtime/c10-seal-v2.json` acepta 32 muestras con ocho
bloques, CRT y resúmenes, con build limpio y 437 entradas locales del compilador.
Incluye tres batallas x86 y nativas. El corpus
`coordination-runtime/c10-corpus-v2.json` pasa nueve partidas, dos ejecuciones
por partida, 30 días, doble guardado, recarga y un día más. Ambos resultados
corresponden a fuentes hasta `e80a95a8`; deben renovarse si cambian. El corpus
es una prueba de determinismo y persistencia, no nueve comparaciones con el
original. La tarea 0x12 de AUTO no vence durante esos 30 días; sus rutas se
contrastan por separado.

Las tareas 0x0B, 0x0C, 0x0E, 0x0F, 0x10, 0x13, 0x18, 0x28 y 0x2D
están conectadas y contrastadas con x86 y S05. La clasificación final prueba
ambas variantes de un byte de pantalla no inicializado y exige contexto si
cambia el ganador. En 0x18 se contrastan el crecimiento de cola de 128 a 256
y el desbordamiento de fecha con el dispatcher y los serializadores originales;
este último se limita al EXE inglés y a la ventana descrita en
`p3-sim/C10_PROFIT_SHARE.md`. En 0x1E están conectadas las ramas de sucesos y primera
banda; se rechaza sin mutar el caso en que el original conserva un puntero de
barco obsoleto al crecer el pool. El plan anual 0x1F incluye selección de
acontecimientos y programación del ciclo siguiente. Las misiones 0x32 de
población, carretera y pasajeros ya no exigen flota ni destinos que no usan.
La fundación asignada desde el planificador tiene 64 escenarios x86 y una
prueba nativa de despacho y S05; el oráculo intercepta nombres, planos y rutas,
y no constituye una comparación de todo el tick. Si la cola debe crecer durante
la fundación se rechaza, porque el original conserva un puntero antiguo.

La modalidad 2 de creación pirata urbana selecciona el barco mediante las tres
cajas del ejecutable inglés y tiene contraste x86 con sus extremos. La fase 0
de misión pirata 0x32/3 ya se despacha desde la sesión, repone reservas y
persiste flota y bandas: cuatro cadenas x86 ejecutan las altas reales y la
sesión guarda y recarga con y sin crecimiento. El heap y el banco de nombres
del oráculo están adaptados; la prueba de sesión ejecuta el planificador, no
una cadena única del reloj completo. La difusión F7 con respuestas y la entrega
urbana -3 están conectadas y contrastadas con intérprete, mensajes y S05.

Para el asalto de guarida sin torres ni defensores se preparan en copias el
ataque y la escena humana o de IA, las cartas y el destructor. Hay 96 escenas
encadenadas con la escritura de cartas y los serializadores originales; 32
frames encadenan temporizador, destructor, retorno del convoy y cierre de la
banda. El depósito de cartas, banda y pool naval se han recargado en pruebas
separadas. Los efectos, el despacho de fase 1 y la persistencia completa siguen
sin estar conectados; la escena con defensores tampoco está cubierta. El
original presenta una discrepancia al recargar la escena vacía acabada: lee
bytes del bloque siguiente como una cantidad de entradas. La reproducción
x86 y sus límites están en `p3-core/C10_TOWN_ATTACK_FORMAT.md`. Los contrastes
previos de escena usaban geometría sintética; el recurso inglés tiene tamaño
34×21 y desplazamiento 17/11, aún pendiente de integrar y contrastar.

El reloj de `Sim` bloquea las entradas normales mientras una batalla naval
cargada esté pendiente de resolución. La sesión libera esa guardia al publicar
el gestor resuelto; 34 desenlaces y su recarga pasan en Rust. El asalto urbano
activo sigue rechazado al cargar y su gestor aislado rechaza el guardado. Esta
protección aislada no implica que el despacho ni el guardado de sesión estén ya
conectados a la escena urbana. El acceso directo a campos públicos de bajo nivel
no equivale a avanzar una sesión completa.

El sello posterior `coordination-runtime/c10-seal-v3-ac579.json` acepta 32
muestras de AUTO durante 30 días frente al x86 aislado, con ocho bloques, RNG,
resúmenes y tres batallas. El corpus nativo correspondiente pasó nueve partidas
con doble ejecución, guardado y recarga. Ambos son del commit `ac57957d` y no
cubren las fuentes posteriores: deben renovarse al completar la integración.
El corpus no equivale a nueve comparaciones contra el original.

Quedan rutas parciales de misión pirata 0x32/3 y del ciclo decenal 0x03.
Este último rechaza de forma transaccional el crecimiento durante jubilaciones:
el ejecutable usa un puntero liberado y el resultado depende del asignador.
Las escenas gráficas activas no se han contrastado. Para cerrar C10 quedan
conectar y contrastar las rutas pendientes y sus dependencias, resolver o
justificar los límites de memoria del original, renovar corpus y sello con las
fuentes finales y registrar una revisión independiente. Estas evidencias
distinguen código nativo, ejecución x86 aislada y estado persistido: no
acreditan paridad visual, multijugador ni una partida humana.
