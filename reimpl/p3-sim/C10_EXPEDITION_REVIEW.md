# Revisión de integración de expediciones

PC de Sergio. Revisión de `c10_convoy_runtime`, `navigation::ExpeditionPlanning`,
recursos del EXE/CPR y copia del contador desde `DailyClock`.

La planificación de la salida publica flota, cola y CRT juntos. Una carencia
de recursos devuelve error y mantiene sus valores anteriores. El despacho
mantiene la operación fallida y las siguientes en orden; cada operación
anterior ya aplicada conserva sus efectos, como el despacho secuencial.
El contador se copia del reloj vivo después del decremento diario.

Corregidos durante la revisión por el integrador:

- Un convoy humano que termina de salir no necesita los recursos de
  expedición. Se comprueba esa salida antes de exigir contexto.
- Reconfigurar CPR sin `Expeditionen.dat` borra la tabla anterior para no
  mezclar bancos.
- El movimiento a `(118,1600)` exige una ruta `Built`; una respuesta `Shared`
  ya no se convierte silenciosamente en propia. El buscador devuelve estado
  1 en todas sus respuestas `Built`.

`c10_convoy_planning_live`: tres pruebas pasan contra la integración C10.
La primera ejecuta la salida del convoy a través del anfitrión de navegación
completo y compara imágenes, cinco operaciones y estado CRT con el original
`506720` hasta `5079DA`, incluyendo `503B50`, `534F10` y el constructor de
operaciones. También compara las salidas humana y con contador activo.
Las otras pruebas comprueban rollback de flota/cola/CRT sin mapa y la salida
humana sin contexto de expedición. No hay ejecución del juego.

Siguiente hueco confirmado: `514000` no está conectado. El snapshot deja a
cero los cinco punteros de `manager+0xC`, mientras las bandas existen en el
estado S05 separado; el callback `TickCall::Pointer` no tiene implementación.
Las pruebas previas de tick interceptaban esa función. Reparación de cascos,
capitanes, bajas y operación B5 requieren un módulo y contraste propios.
