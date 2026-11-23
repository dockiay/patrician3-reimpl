# C10: fundación asignada desde el planificador

Una misión del alcalde `0x32/0` asignada por el planificador puede comenzar sin pasar por la operación humana `0x4B`. Se reutiliza la preparación transaccional de la ciudad, capitanes, tareas, rutas y asedio que ya usa `0x4B`. El despacho se suspende sin consumir la tarea; el reloj completa inmediatamente la fundación y reanuda el planificador en el mismo tick. Se incrementa el vencimiento de la misión en `0x100`, se reordena la cola y se actualiza el número de ciudades antes de las fases posteriores del reloj.

El preparador usa copias y no publica la ciudad cuando faltan recursos o las vistas son incompatibles. La escritura S05 rechaza una fundación pendiente. Se detectan ciclos o solapamientos entre las listas activa y libre; no se interpreta el campo `kind` como indicador de libertad, pues las ranuras recién creadas pueden contener cero. Si no quedan dos ranuras libres para las dos tareas nuevas se rechaza el despacho: no se ha caracterizado aquí el efecto de crecer el planificador mientras el original conserva un puntero al registro despachado.

## Pruebas

- `c10_found_scheduled_x86`: 32 escenarios comparan el planificador original `0x004D85C0`, la creación original `0x00531D50` y los serializadores x86 de ciudad, asedio, tareas y capitanes con la preparación nativa y el reordenado. Se cubren cero y seis ciudades previas.
- `c10_found_scheduled_late_x86`: otros 32 escenarios con la tarea atrasada `0x100` y plazo válido contrastan la segunda ejecución de la misión en la misma llamada y el reordenado de la cola.
- `c10_found_full_x86`: sigue pasando la regresión de 32 fundaciones por operación `0x4B`.
- `merchant_daily::founding_scheduler_tests::assigned_founding_resumes_scheduler_and_survives_save`: con un guardado y CPR locales comprueba la reentrada del despacho, otra tarea vencida en el mismo tick, la ciudad y tarea tras guardar y recargar S05, y rechazo sin crear la ciudad para crecimiento, lista libre cíclica o solapada.

Los oráculos x86 usan el ejecutable inglés y `FPCW=0x027F`; interceptan las entradas de nombres, lectura de plano y cálculo de rutas. La prueba de sesión utiliza un planificador sintético y llama a la fase del planificador del reloj directamente: no contrasta en una sola ejecución el tick completo, las escenas de asedio y S05 con el original. No es una prueba en partida ni demuestra todas las rutas de fundación.

Los oráculos se regeneran con `reimpl/oracle/x86_c10_found_scheduled.py` y `reimpl/oracle/x86_c10_found_scheduled_late.py`, usando `--binary` y `--check`. La prueba de sesión requiere `P3_SAVES` y `P3_CPR`; si faltan, se omite.
