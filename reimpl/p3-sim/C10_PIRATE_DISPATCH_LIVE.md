# Despacho de asalto pirata 0x32/3 (C10, PC de Sergio)

El reloj interrumpe el tick tras el planificador y antes del puerto al encontrar
la tarea de asalto. `Session::resume_pirate_mission` prepara una base privada
S05 del estado vivo y reanuda el mismo tick; la base no es un guardado válido
para entregar mientras el tick sigue incompleto. El gestor y sus efectos se
atienden antes de los asedios; el frame se procesa después mediante la primera
operación 0x94 de la cola. El llamador aporta explícitamente las consultas de
interfaz. Se admiten varios frames en el mismo tick si no hay barcos locales,
proyectiles ni torres, y el índice espacial contiene únicamente el terreno;
las demás continuaciones se rechazan transaccionalmente. Los frames que
terminan admiten el delta del registro 0x94.
El despachador genérico conserva 0x94 y exige contexto en lugar de tratarla
como oferta del concejo.

La operación B4 posterior se prepara en copias y se consume en el despachador
solo tras validar y publicar conjuntamente sus vistas. Una preparación fallida
conserva B4 y las operaciones siguientes. La base anterior, la escena y el
resultado se concilian con S05 después de atender la cola y los efectos;
`Session::finish_pirate_mission` verifica la recarga antes de publicar el
modelo. Se mantienen barreras de avance y guardado mientras exista una fase,
escena, efecto u operación pendientes.

La prueba `live_wait_uses_save_views_and_rejects_stale_positions` usa AUTO
modificado: ejecuta reloj→corte→base→reanudación→gestor→asedios→0x94→B4→S05,
recarga banda, mensajes y planificador y vuelve a guardar. La prueba adicional
`c10_pirate_session` recorre las entradas públicas de `Session` y verifica la
recarga. Comprueba que una
base sin capturar, la cola ocupada y un frame no terminado rechazan sin perder
operaciones. Los recursos de escena, nombres y presentación de esta prueba son
sintéticos. Las etapas tienen contrastes x86 separados; todavía no se ha
comparado toda esta cadena en una única ejecución del original.

Doce escenas sin objetos se contrastan durante tres frames sucesivos por caso
con una misma instancia x86; otra usa el rectángulo espacial de terreno y
despacha cuatro operaciones 0x94 mediante `0x546870`. Se
comparan los bytes completos de escena y gestor, incluidos estadísticas,
viento y semilla. El despacho marca `gestor+0x3FE` y puede generar 0xC8: el
llamador aporta el modo actual y recibe la operación para aplicarla, sin que
el despachador del concejo la descarte. La prueba nativa de sesión enlaza dos frames, B4 y recarga,
y rechaza sin consumir la cola si aparece otro objeto espacial. Son escenarios
preparados; aún no se llega a los frames x86 desde el reloj.
Otros 32 casos ejecutan el frame final mediante la cola x86, incluyendo la
destrucción y la emisión de B4; esta prueba todavía no despacha B4.

Límites: se admite la guarida vacía ya cubierta; defensores, varias escenas,
ventanas activas y avanzar el reloj mundial entre frames permanecen pendientes.
El índice espacial se crea para esa escena aislada. La conciliación rechaza
ciudades o cambios opacos no representados y conflictos concurrentes. No se
abrió el juego ni se acredita paridad visual o en partida.
