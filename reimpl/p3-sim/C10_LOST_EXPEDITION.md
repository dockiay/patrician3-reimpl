# C10: convoy perdido por el borde del mapa

PC de Sergio. `Convoy::lost_at_edge` (0x5029F0) crea en el planificador
original una tarea con vencimiento el tick actual y tipo 0x12. El registro
recibe el índice de convoy del líder en +8, FFFF en +0xA y el tick en +0xC.
El anfitrión prepara el planificador sobre una copia y solo lo publica junto
con la flota, la multa y las demás vistas cuando el tick completo termina.
También retira al líder del orden naval usando el método real con mundo.

El oráculo `x86_c10_lost_edge.py` ejecuta 502110→5029F0→4D8CF0 y la retirada
del orden originales en seis casos sintéticos, con saldos -1, 0, 40000,
40001, 80000 y 600000. La búsqueda de ciudad y los servicios genéricos de
memoria son adaptadores; el planificador x86 sí se ejecuta. Coinciden barco,
convoy, comerciante y los 128 registros del planificador; la multa trata el
saldo con signo. Un segundo convoy con ruta inválida demuestra la reversión
del primero y de la tarea, y se contrastan doble guardado y recarga S05 de la
tabla de tareas.

Con esta preparación ambas partidas TestAstiller superan `cv_obj_new` y se
detienen en el despacho posterior de la tarea 0x12, tick 125727054. Este bloque
no implementa ni acepta su ejecución posterior, ni demuestra paridad del ciclo
completo o validación en partida.
