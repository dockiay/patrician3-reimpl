# C10: formato inicial de ataque a ciudad o escondite

`allocate_town_attack_battle` y `construct_town_attack_battle` reproducen la
reserva `0x00611D80`: el objeto ocupa 0x2F24 bytes, usa el constructor base
`SeaBattle`, instala la tabla virtual 0x0067AAA0 e inicializa sus seis bytes
propios en +0x2F1C. Se conservan los bytes que el original deja sin inicializar.
La prueba compara 32 escenarios con el ejecutable inglés, incluida falta de
memoria y tabla llena; se intercepta exclusivamente el asignador.

Los prefijos propios de escritura y lectura de `0x005FBEA0`/`0x005FBF30`
comprenden 29 bytes: destino y cuatro registros de valor, coordenadas y
recarga. La escritura puede marcar el segundo bando; la lectura reinicia
fuerza y acumuladores y marca la máscara de tierra para reconstrucción. Las
pruebas comparan 40 escenarios por sentido y cuatro versiones de lectura;
el serializador o lector base se intercepta en su entrada, por lo que estas
pruebas cubren el prefijo y los efectos del objeto, no el guardado completo.
Se rechazan entradas truncadas sin modificar el objeto.

Estos componentes no inicializan la escena de ataque `0x005FBFE0`, ni la
simulan, ni la conectan al reloj o S05. Sin validación en partida.

La máscara de tierra para un escondite reproduce `0x005FFC10` sobre el
plano aportado y conserva los cálculos y truncamientos de 32 y 16 bits.
Se contrastan 40 planos y proyecciones x86, incluidos agua, tierra, límites,
offset negativo y saturación de la cuadrícula. El oráculo intercepta la
reserva espacial `0x00619450` y compara sus seis argumentos; el llamante debe
crear la entrada y pasar su ID. El puerto rechaza dimensiones superiores a
4096 o más de un millón de celdas para evitar recorridos fuera de su ámbito.
Todavía no incorpora la máscara de ciudad ni conecta este resultado a una
escena o a S05.

El prefijo de `0x005FBFE0` para guaridas sin defensores ni torres prepara el
objeto de ataque, máscara, notificación, sincronización y petición de fin en
copias. Se compara el objeto completo en dieciocho escenarios x86 con barcos
sueltos, convoyes, miembros que no son cabeza, dueños humanos o IA, cinco
guaridas y varias fechas. El llamante aporta la cabeza del convoy por separado
del barco seleccionado; se usa el byte del dueño. Se
interceptan creación de colisión, notificación, control de tiempo, consulta
de comerciante y `town_attack_battle_finish`; los retratos y el recuento de
barcos llegan como entradas. La condición de ausencia de torres y defensores
la debe comprobar el llamante. No se ejecutan ni persisten los efectos de
finish: esta prueba no acredita un recorrido completo ni una batalla viva.

El final directo de victoria de la guarida vacía, con ambos bandos controlados
por IA y sin barcos locales, calcula el RNG, la recompensa y las peticiones de
cartas global e informe; actualiza las dos cifras del comerciante que posea
el convoy. El llamante aporta los textos y el estado del convoy. Se rechazan
convoy no válido, divisor cero y otros estados fuera del caso contrastado.
Dieciocho variantes x86 comparan objeto, comerciantes, cartas y los 1.110 bytes
serializados de la escena, incluidos dueños repetidos o adicionales y cadenas C
con terminador. Otra prueba usa el depósito y serializador de
cartas originales, compara las cabezas y normaliza únicamente el puntero
transitorio del texto. Los nombres y textos del oráculo son sintéticos. Aún
faltan la conexión al reloj y S05 completo, el caso humano, la escena con
defensores o torres y las demás ramas de finalización.

Advertencia de procedencia: la prueba previa de escritura y lectura originales
usaba un objeto sintético con contadores de bando cero. La comparación del
escritor también cubre contadores de uno y dos atacantes. En el inicio real sin defensores
`+0x658` puede quedar en 1 o 2 aunque `+0x14` y las listas locales estén
vacías. El escritor usa `+0x14`, pero el lector deriva su longitud de la suma
de los contadores; con dos atacantes el lector consume ocho bytes adicionales.
Una segunda escritura original difiere. No se acredita la recarga del asalto
real y no debe normalizarse ese desajuste sin evidencia adicional.

La cadena x86 aislada `0x00603A70 → 0x005FBFE0 → 0x005FE7E0 →
0x00611950 → 0x00611AE0`, con versión 0x85, confirma el desfase para uno y
dos atacantes. El gestor escribe 1.116 bytes. El lector desplaza la frontera
4 u 8 bytes y lee respectivamente el separador `0x0000AFFE` o el principio
del banco de nombres `0x00410041` como número de parejas. Se usaron los bytes
siguientes de AUTO y se detuvo el oráculo antes de `0x00612A20`, que habría
intentado procesar esos índices; no se afirma qué ocurriría en Windows.
Recursos de texto, CRT, heap, UI y depósito de cartas fueron adaptados.

La variante humana directa del mismo final prepara la carta global y el texto
para una ventana inmediata o diferida, sin abrirla; devuelve una referencia
lógica al diálogo. Dieciocho escenarios contrastan el objeto completo, dinero,
mensajes y solicitud de ventana x86 con temporización 0/1 y una ventana ya
ocupada o libre; el depósito original confirma la carta global. Los textos y
las funciones de ventana del oráculo se adaptan. Sigue pendiente conectar el
diálogo, esta rama y el objeto derivado a la sesión y al guardado completo.

Hasta reconstruir estas escenas, `Sim::from_save_bytes` rechaza explícitamente
un ataque guardado: antes podía ignorar el error del parser y permitir avanzar
el reloj sin reconstruir la batalla. La regresión usa los bloques del escritor
x86 con uno y dos atacantes, también detrás de una batalla naval básica, y
comprueba que la ruta anterior para una batalla básica sigue aceptándose.
La guarda no corrige el formato original ni reemplaza la implementación de la
escena y deberá mantenerse al ampliar el parser hasta conectar su estado vivo.

El mismo inicio y final cubren ahora también una banda cuyo convoy defensor
existe pero tiene lista vacía: 24 escenarios x86 por ruta, con cartas y depósito.
El puente debe verificar que ese convoy es válido y que su cadena está vacía;
el indicador suministrado al helper no sustituye dicha comprobación.

El prefijo de `0x005FD510` para escena ya terminada gestiona el diálogo y el
temporizador y devuelve la solicitud de destructor al anfitrión. Se contrastan
48 escenarios x86 de ticks, diálogo libre/ocupado y temporizador. Se interceptan
la consulta de ventana, la apertura y el destructor; cuando el contador no
vence se intercepta también el resto del update. La prueba no acredita la
liberación de la escena, la resolución del convoy ni el fin de la banda.
La operación de autoguardado puede despacharse antes del siguiente frame de
batalla; no se presupone que el destructor ya haya corrido al guardar.

`0x0064F7B9` llama a `malloc` y el constructor no inicializa `+0xAC8`.
Con la misma escena, el oráculo prueba bytes previos 0, 1, 40, 126 y 255:
para IA, el cero provoca Sync y se convierte en 1, mientras que los demás se
conservan y no provocan Sync; para humano todos dan Sync y valor 1. Por ello
el puente debe recibir una imagen de memoria inicial acreditada o rechazar;
una asignación cero del emulador no demuestra el contenido del heap de Windows.
