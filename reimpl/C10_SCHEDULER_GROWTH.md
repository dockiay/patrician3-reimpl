# C10: crecimiento del planificador durante 0x03 y 0x18

El oráculo `reimpl/oracle/x86_c10_scheduler_growth.py` ejecuta `0x004D85C0` y `0x004D8CF0` del ejecutable inglés SHA-256 `fe436dc5f8addc4d3aa437eb48a8bc6415d9ba5577c1027d06db9bb915eddece`. Comando:

```powershell
D:/AI/Patrician3/tools/python/Scripts/python.exe reimpl/oracle/x86_c10_scheduler_growth.py --binary D:/AI/Patrician3/games/iso-english/app/Patrician3.exe --check
```

Parte de 128 registros ocupados; la reserva añade 128 y cambia la dirección del array. Para distinguir la dependencia del asignador se modelan tres respuestas a la liberación del bloque anterior: conservar sus bytes, rellenarlo con `0xDD` o desmapear su página. Son modelos controlados del emulador, no mediciones del heap de Windows.

- **0x03:** el despachador conserva el puntero previo en EDI. Al volver del manejador, `0x004D8668` lee el registro anterior y `0x004D8671` escribe en esa memoria ya liberada. Con memoria retenida o rellenada, el primer cambio de fecha no alcanza el registro vivo y el despachador ejecuta 0x03 otra vez: se observan dos incrementos del contador de la tarea y finalmente retorna con `due += 0xA00`. Con la página desmapeada falla la lectura en `0x004D8668`, antes del segundo despacho. En esta prueba una dependencia interceptada crea una tarea mediante `0x004D8CF0` real solo en la primera llamada; las demás dependencias de 0x03 no se reproducen. No demuestra que una partida real siempre cuelgue ni que el heap instalado use alguno de esos patrones.
- **0x18:** en fecha trimestral, acciones 20, patrimonio 100.000 y saldo 0, el manejador reserva una deuda con `0x004D8CF0` real. `0x004E89AA..BD` vuelve a obtener el array y la cabeza tras la reserva. En los tres modelos, incluso desmapeando el bloque anterior, retorna con capacidad 256, tarea original `due += 0x100`, patrimonio previo 100.000 y deuda 0x01 de 5.000 con vencimiento `tick + 0x1400`. La consulta de comerciante, el constructor de cartas y su entrega están interceptados. No son serializadores ni prueba en partida.

El modelo Rust `freed_blocks` reproduce el caso de bytes retenidos, no acredita el comportamiento de otros asignadores. Una protección del producto puede rechazar atómicamente 0x03 cuando un manejador fuerce crecimiento, o reservar capacidad antes del despacho si se documenta que evita el acceso del original; no debe declarar una única política de memoria liberada como paridad universal. El crecimiento 0x18 sí puede usarse si la implementación vuelve a acceder al registro vivo y sus pruebas incluyen cola, deuda y persistencia.
