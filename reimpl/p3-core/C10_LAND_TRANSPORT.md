# C10: transporte terrestre 0x0C

`economy::land_transport::prepare` prepara las ocho fases de `0x4E38B4`
sobre copias de tarea, oficinas y comerciantes. Gestiona la extracción, los
siete enlaces terrestres, esperas, bloqueo y vuelta, pérdidas, entrega y sus
avisos. Exige los siete registros de ruta y unidades originales, y que las
oﬁcinas se entreguen en el orden de sus listas urbanas. Con escena de ciudad
activa rechaza la ejecución hasta conectar su presentación.

El x86 divide de nuevo por 2000 la cantidad transportada si la oficina carece
de existencias suficientes, incluso para lotes de 200 y 10; se conserva ese
comportamiento observado. Las cartas `0x66..0x69` y avisos son efectos tipados.
`p3_state::land_transport::prepare` aplica las cartas mediante el depósito
existente y las preferencias, también sobre copias.

`c10_land_transport_x86` compara 405 escenarios x86 de las ocho fases,
mercancías, bloqueos, esperas, oficinas, desbordamientos de contador y avisos.
La consulta de oficina y presentación se interceptan; la selección exacta de
la lista viva se debe contrastar en la integración. Otra prueba ejecuta
`4D6530` y el serializador `4D62B0` originales en seis escenarios y comprueba
retroceso ante depósito inválido. Ambos usan FPCW `0x027F` y EXE inglés.

El adaptador `land_transport_stage` selecciona oficinas recorriendo los enlaces
reales de ciudad y el scheduler ejecuta 0x0C con preview transaccional. Los
avisos se conservan en `land_transport_notifications`; el adaptador de sesión
los conserva al reconstruir la flota. Sobre AUTO modificado, dos escenarios de
salida y entrega con una oficina interior de la lista comparan el scheduler
`4D85C0`, lookup `5308A0` y serializadores x86 de todas las oficinas, tareas,
comerciante y cartas. Pasan doble guardado, recarga de campos afectados y
rollback ante pool de cartas inválido. Las tablas de siete rutas y unidades son
las verificadas contra el EXE inglés.

La ventana de escena urbana activa no está implementada: el perfil de sesión
conectado corresponde a la escena original ausente, y los avisos son efectos
pendientes de presentación. Faltan contraste integrado de las demás fases y
validación en partida; las 405 escenas aisladas cubren dichas fases con lookup
de oficina y presentación interceptados.
