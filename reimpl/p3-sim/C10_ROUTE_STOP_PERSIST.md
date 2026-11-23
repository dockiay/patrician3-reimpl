# Depósito de paradas y persistencia C10

`RouteStopRuntime` conserva el bloque S05 íntegro, permite preparar una baja
4D4DD0(índice,0) o adoptar un bloque producido por el anfitrión (`replace`),
y lo publica en `SaveModel` solamente si su versión coincide con la actual o
una versión histórica del mismo runtime. La sustitución con base incorrecta,
formato inválido o conflicto de escritura se rechaza antes de modificar estado.
El historial conserva hashes SHA256 de versiones, no copias completas.

`c10_route_stop_persist_x86` compara dos bajas sucesivas en doce casos con
los cargadores y serializadores originales 4D4BC0/4D4B60, incluyendo recarga.
También escribe y vuelve a leer un SaveModel construido sobre AUTO.pat, y
comprueba conflictos, errores atómicos y persistencia de versiones anteriores.
No se ejecutó el juego. Falta conectar el estado al anfitrión y `persist_all`.
