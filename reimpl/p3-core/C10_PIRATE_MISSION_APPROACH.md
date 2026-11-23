# C10: aproximación a una guarida pirata en misión 0x32

`prepare_pirate_mission_approach` calcula la fase 0 cuando barcos del comerciante se acercan a la posición de la banda. Prepara en copias la misión, el campo `merchant+0xE1` y las llamadas de reposición en su orden original; recorre todos los barcos, conserva el retraso mínimo calculado y rechaza cadenas cíclicas o truncadas. El llamante debe aplicar en una transacción las reposiciones y las vistas preparadas.

`cargo test -p p3-core --test c10_pirate_approach_x86` contrasta 256 escenarios frente a `0x004EAFE4` del ejecutable inglés, con varios barcos próximos y hasta cinco bandas. Compara tarea, comerciante, orden y destino de cada llamada, y el retorno. La función original `0x005144A0` se intercepta: solo se comparan los 16 bits de argumento que consume, ya que la parte alta del original contiene basura de registro. `P3_EXE` y `P3_ORACLE_PY` deben apuntar al ejecutable y al Python con Unicorn; si faltan la prueba se omite.

La conexión posterior al planificador, la reposición real y sus límites se documentan en `p3-sim/C10_PIRATE_MISSION_LIVE.md`. El encuentro de fase 1, que puede crear o reforzar una escena de asalto a la guarida, sigue pendiente. No se ha probado en partida.
