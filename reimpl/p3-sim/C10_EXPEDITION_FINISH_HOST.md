# Finalización nativa de expedición 0x00502B20

`p3_core::ships::expedition_finish::prepare` recibe un `SparseMem` y un
`NativeFinishHost`, clona ambos y devuelve las dos imágenes nuevas solo si
cada efecto se completa. El llamante debe preparar todas las regiones de
memoria requeridas por el núcleo (véase `p3-core/C10_EXPEDITION_FINISH.md`),
incluidos los pools completos. `SparseMem` falla con panic ante direcciones no
sembradas: validar el plano antes de invocar el núcleo es responsabilidad del
adaptador de la sesión.

El host aplica la carta 0x34 a `message_pool`, los cambios de capitán y
convoy a memoria, planifica el retorno con `RoutePlanner`, actualiza el convoy
e inserta la orden. Devuelve rutas (`routes` y `route_points`), pool de cartas
y notificaciones como estado transaccional; la sesión debe publicar estos
datos junto con la memoria, las listas de barcos, el reloj y S05. Las ventanas
son notificaciones tipadas pendientes de consumo por la interfaz. Se rechazan
las rutas con puntero anterior vivo, destinos sin puerto, rutas compartidas
cuyo origen no se acredita y callbacks de convoy no conectados. No hay host
alternativo para cubrir silenciosamente estos casos.

`c10_expedition_finish_native` compara seis escenas de pérdida completa y
ocho escenas con supervivientes contra 502B20 y sus dependencias x86 reales,
incluidos la carta y los pools; el planificador x86 0x445010 se intercepta con
los puntos calculados por Rust sobre un mapa sintético, por lo que no se
acredita aquí paridad integral del buscador. También prueba la reversión tras
una carta ya preparada cuando falta el planificador y dos escrituras idénticas
con recarga S05 de barco, ruta, convoy, capitán y cartas. Es un ensamblado de
bloques de prueba, no una transacción de sesión completa ni prueba en juego.

Prueba con `P3_ORACLE_PY`, `P3_EXE` y `P3_SAVES` configurados:

```
cargo test -p p3-sim --test c10_expedition_finish_native -- --nocapture
```

Pendiente integrar la fase 4 de la tarea 0x12 y publicar conjuntamente sus
mutaciones con estas vistas en el reloj y S05, verificando la cadena completa
contra el original. Las ramas rechazadas siguen pendientes de implementación.

`expedition_finish_stage::prepare` adapta `LiveVoyage`, ciudades,
comerciantes, cartas y planificador a la memoria x86 preparada, y devuelve las
vistas mutadas sin cambiar la entrada. La tabla de prestigio (delta), el reloj,
el calendario, los puertos y el límite del pool son datos explícitos. El
adaptador reconstruye rutas y listas de navegación; la sesión todavía debe
confirmar conjuntamente el resultado con la tarea 0x12 y persistirlo en S05.
Los mismos 14 escenarios comparados contra x86 pasan también por este puente,
incluida reversión por planificador ausente. No es aún contraste de la tarea
programada completa ni de sus serializadores originales encadenados.
