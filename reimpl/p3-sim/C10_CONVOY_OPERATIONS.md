# Expediciones: operaciones 1E y 1F

PC de Sergio. Contraste aislado del EXE inglés de referencia, sin juego ni DLL.

`c10_convoy_operations` reproduce `0x00539B50` y `0x00539C60`.
`prepare_start` valida convoy, líder y, si el convoy todavía no está en
estado 15, la aptitud original `0x00519BA0`. Conserva cantidad y selector antes
de transformar la operación en una orden al líder hacia `(118,1600)`.
El anfitrión ejecuta el movimiento sobre sus copias y después `finish_start`:
reinicio de cuatro puntos y flags, selector, saldo del convoy, dinero del
comerciante y contador de expedición. El límite del dinero es **con signo**,
después del posible reembolso, y las sumas conservan el wrapping de 32 bits.
El contador usa el byte de la operación **ya transformada**: `0x76`, no el
argumento previo. Un propietario fuera del vector no limita ni cobra el dinero
ni cambia el contador, pero sí modifica el saldo de expedición.

`set_waypoint` conserva la misma guarda y escribe un par de words y un byte.
Los cuatro índices válidos están contrastados. Un índice mayor se rechaza
antes de modificar el estado; el original escribe fuera del array.

`departure::reroute_convoy_to_point` amplía la orden del líder navegando a un
destino externo: estados 15/16, ruta propia suministrada por el anfitrión,
reembolso, último puerto conservado, destino `0xFF`, recuento y miembros del
convoy. No calcula el A*; el anfitrión debe conservar los puntos reales y el
descriptor de ruta en su caché. Las maniobras de puerto siguen usando sus
otras rutas de despacho y no quedan acreditadas por esta prueba.

Verificación: `cargo test -p p3-sim --test c10_convoy_operations_x86` con
`P3_EXE` y `P3_ORACLE_PY` configurados: **192 escenarios**, 128 de inicio y
64 de puntos, además de fallos atómicos de slot y ruta. Se ejecuta la cadena
original `539B50 → 537670 → 5172A0 → 501BE0`; sólo el buscador de ruta recibe
puntos controlados y devuelve estado 1. Se comparan todas las imágenes de
barcos, convoyes, gestor y comerciantes, la operación transformada, el
contador y el registro serializado por `5013D0`. El oráculo también ejecuta
el serializador de barcos. No equivale al contraste del reloj completo.

El integrador C10 conecta despacho, caché de rutas, copias de comerciantes,
contador `idle.clock.expedition_countdown` y transacción S05. El contador ya
se guarda en `world.header[13]`; no se añade otro campo de mundo.

## Integración en C10

`c10_convoy_runtime` despacha 1E/1F desde la cola del reloj y calcula la ruta
con los recursos CPR reales. Publica de forma conjunta las imágenes navales,
las vistas económicas, el contador WorldClock+8B y las rutas persistibles.
El fallo de un recurso conserva la operación para reintento sin cambios.
`p3-mvp/tests/c10_expedition_runtime.rs` comprueba esta transacción sobre
AUTO.pat, incluyendo guardado doble, recarga y reconstrucción de flota.
Esto complementa los 192 casos x86 con rutas controladas; no es juego humano.

La entrada automática 503B50 se ejecuta después de poner el convoy en estado15.
Las órdenes manuales 1E durante maniobras o en puerto se rechazan explícitamente
en este adaptador de C10. La ruta exterior debe ser propia; una salida de
recursos que coincidiese con un puerto no se convierte en ruta propia a ciegas.
