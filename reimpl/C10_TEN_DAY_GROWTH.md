# Crecimiento del scheduler durante el ciclo decenal

El despacho `004D85C0` y el ciclo `004DDA40` se han ejecutado con el evento de
capitanes `004DCEA0` y la inserción `004D8CF0` originales, sin interceptar la
jubilación. Se modelaron tres políticas de liberación del array anterior:
conservar sus bytes, rellenarlos con `0xDD` o desmapear su página. Son modelos
controlados de asignador; no describen el heap de una instalación concreta.
Se adaptaron la consulta de comerciantes, la purga de cartas y el recuento de
mercancías entrantes, que no son el objeto de este contraste.

- Cola llena y capitán joven: no se reserva, el ciclo se ejecuta una vez y
  coincide con Rust en las tres políticas.
- Cola llena y un capitán IA antiguo: se inserta una jubilación y el array crece
  de 128 a 256. Retener o envenenar produce dos ejecuciones del ciclo; al
  desmapear, el original falla al leer el puntero anterior en `004D8668`.
- Una plaza libre y dos capitanes IA antiguos: la primera jubilación ocupa la
  plaza y la segunda provoca crecimiento. Retener o envenenar ejecuta el ciclo
  dos veces y deja ambas jubilaciones; desmapear falla en el mismo acceso.
- Dos plazas libres y los mismos capitanes: no se libera el array, el ciclo se
  ejecuta una vez y Rust coincide con las dos jubilaciones y sus fechas.

Los resultados dependen de la memoria liberada. No existe un comportamiento
único que pueda atribuirse al ejecutable sin elegir una política de heap; el
puerto mantiene el rechazo explícito al necesitar crecimiento. La preparación
trabaja sobre copias: si la segunda jubilación agota la cola, también revierte
la primera. Una prueba con partida real comprueba que el rechazo no publica la
purga previa de cartas, el recuento urbano, los capitanes ni el estado CRT.
Este rechazo es una protección transaccional: en el caso desmapeado, el
original ya había aplicado efectos antes de fallar. Un modo explícito de bytes
retenidos podría reproducir sus dos despachos sobre una copia virtual, pero no
sería equivalente al caso desmapeado ni prueba del heap de Windows.

Pruebas: `cargo test -p p3-sim --test c10_ten_day_real_growth_x86` y el caso
`client_ten_day_dispatch_matches_original_and_reloads` de
`c10_ten_day_live_x86`, con `P3_EXE`, `P3_ORACLE_PY`, `P3_SAVES` y `P3_CPR`
configurados. No se abrió el juego ni se ha validado en partida.
