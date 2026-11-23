# C10: finalización física de una expedición (`0x00502B20`)

PC de Sergio. `ships::expedition_finish::prepare` prepara sobre copia la
llegada del convoy, bajas, traspaso de capitán, dinero, puntos y efectos. El
módulo se entrega sin conectarlo aún al despacho de la tarea `0x12`: no debe
publicarse ninguna ruta si faltan sus dependencias.

La memoria de entrada requiere los punteros globales `0x6DD7A0` (capitanes),
`0x6DD7A4` (barcos), `0x6DE518` (comerciantes), capacidades `0x6DD892`,
`0x6DD894`, `0x6DE4AA`, número de ciudades `0x6DE4B0`, tick `0x6DE4B4` y
bonus firmado del ejecutable `0x672DD0`, además de la imagen del convoy y los
arrays referenciados. El host debe clonar profundamente su estado, sin
publicar efectos externos antes de confirmar el resultado. `prepare` entrega
las dos copias solo si termina sin error.

Los efectos obligatorios se enumeran en `Effect`: carta, ventanas de pérdidas
y llegada, baja de convoy, elección de líder, liberación de capitán,
planificación de ruta, recuento e inserción en el orden naval. El host debe
implementar cada efecto con las rutinas y recursos reales o devolver error;
ningún efecto se ignora. La carta se entrega como plantilla de 16 bytes:
`4D6530` asigna la fecha y la lista al insertarla; los bytes +6/+7 no deben
tratarse como valor persistente de la plantilla local.

`c10_expedition_finish_x86` contrasta 96 escenarios de 502B20 x86 aislada,
con supervivientes y bajas, humanos/IA, transferencia/liberación de capitán,
reputación con signo y límites, y efectos externos interceptados. Compara
imágenes completas y secuencia de callbacks, y rechaza una cadena cíclica o
un error de callback sin publicar la copia. El oráculo intercepta el envío de
cartas, ventanas, baja/elección de convoy, liberación de capitán, trazado,
recuento e inserción; no acredita aún esas dependencias ni la fase 4 completa.
El convoy sin miembros al entrar se rechaza explícitamente: el índice que el
original usaría para la baja procede de un temporal no inicializado.

Definir `P3_EXE` y `P3_ORACLE_PY`, y ejecutar
`cargo test -p p3-core --test c10_expedition_finish_x86 -- --nocapture`.
Sin esas variables la comparación x86 se omite. No es validación en partida.
