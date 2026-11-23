# C10: crecimiento tras captura pirata

PC de Sergio, 4/11/2026. `p3_sim::pirate_capture::PirateCaptureState::register_capture`
porta `0x00515F70` y prepara sus cambios de manera atómica. El anfitrión debe
sincronizar la fábrica con los barcos/convoyes/capitanes vivos antes de cada
notificación, y publicar fábrica y bandas conjuntamente con la batalla.

- Sin cambios si `band+0x10 < convoys_size`, exactamente como el original.
- Suma de botín con wrap de u16, choza original ya portada, RNG local con el
  reloj, límite de barcos de reserva y reparación de piezas. La última pieza
  de la cadena activa no se examina: el ensamblador comprueba su enlace siguiente.
- Creación `0x516170` con argumento de capitán cero (única ruta de esta captura):
  asignador con crecimiento, estadísticas de ciudad cero, nombre, mejora,
  artillería, marineros, carga y puerto de la banda. Casco actual queda en uno.
- `day` representa el byte de ciudad `0x6DE52E`; `day_of_year` es `0x6DE4A4`.
  No consume CRT ni emite noticias. La captura que llama a esta función sí puede
  producir noticias, gestionadas por su propio publicador.

Contraste: `x86_c10_pirate_capture.py --binary ... --check` ejecuta 512 escenarios
(626 llamadas), con 61 escenarios de alta de barcos y 32 crecimientos del pool.
Compara toda la imagen de barcos, cursores y lista libre, y la banda/plano/piezas
pasados por el serializador original. Incluye convoyes activos, límite de
reserva, score desbordado, reparaciones y varias capturas con el mismo reloj.
No se interceptan hut, creación, mejora, armamento ni carga; solo heap/delete,
memcpy, consulta de texto del nombre y `0x535740` (fase de copia del pool).

Pruebas: `cargo test -p p3-sim --test c10_pirate_capture --test c10_absorb_hut`
(4 pruebas); 512 escenarios x86 reproducibles. Banco de nombres sintético,
heap nuevo inicializado a cero como en los otros oráculos: los bytes no
inicializados de ranuras libres no acreditan su valor en una partida real.
El puerto de armamento es específico de un barco nuevo sin armas; una entrada
prearmada se rechaza. Los niveles de banda válidos son 0..3 y las tablas y
recursos deben existir. No valida por sí solo captura dentro de batalla ni
paridad del recorrido de 30 días. No se abrió el juego.
