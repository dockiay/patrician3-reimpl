# C10: visita de destinos del regreso de expedición

`expedition_visit::prepare` reproduce el bucle x86 `004E6BDB..004E6C78`: procesa hasta cuatro destinos, intercala sus marcas del mapa con el primer centro coincidente, y conserva el orden de descubrimientos y la restricción de un contrato nuevo. Trabaja sobre copias; los fallos del anfitrión descartan todos los cambios preparados.

`c10_expedition_visit_x86` contrasta 32 estados contra el ejecutable inglés en x86 aislado, con marcas de mapa, descubrimiento inicial, carta global y serializador originales. Comprueba comerciantes, mapa, pool, orden y argumentos de callbacks, centros, destinos no encontrados, descubrimientos y bandera de contrato. El descubrimiento y el comercio de los centros se interceptan con efectos de prueba; esa frontera requiere un anfitrión nativo real y no acredita todavía la fase completa. Una prueba adicional comprueba rollback del anfitrión.

Pendiente: conectar el anfitrión real de centros, desgaste, texto 0x63, cierre 502B20, tarea y persistencia S05; contrastar toda la cadena. Esto no es validación en partida.
