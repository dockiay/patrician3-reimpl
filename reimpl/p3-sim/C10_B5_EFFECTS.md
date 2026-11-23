# B5: carta, tarea y baja en la misma copia

`PirateB5Effects` envuelve las dependencias nativas del lanzamiento B5. Conserva
copias de cartas, planificador, depósito de rutas y grupos de comerciantes;
materializa la tarea temporal en `Effects::finalize` antes de publicar. La
liberación de paradas con argumento1 desvincula todos los barcos seguidores;
la baja conserva cada evento de comerciante con los propietarios del momento.
La notificación visual queda registrada para el anfitrión, no se abre ventana.

`c10_b5_letter_task_x86` contrasta seis escenarios de B5 con 514D40, generador,
4D6530, 4D8CF0, 507FB0, 515F00 y serializadores de carta/tarea originales.
Incluye creación y ausencia de tarea0x05, recarga de los bloques vía SaveModel y
comprobación de liberación de ruta y rollback por capacidad incompatible.
Los recursos de memoria y el heap son controlados; no es validación en partida.
El llamante debe publicar el anfitrión y todas las vistas de la flota juntos.
