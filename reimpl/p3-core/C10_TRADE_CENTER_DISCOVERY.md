# C10: descubrimiento de centro comercial

`trade_center_discovery::prepare` prepara las puertas y efectos de 0x52D0B0:
actualización del centro, prestigio, dos cartas 0x1C y avisos humanos. No
publica ninguna vista: el llamante debe confirmar todos los efectos a la vez.
Rechaza explícitamente imágenes corruptas y dueño o divisor imposibles.

El fixture `x86_c10_trade_discover.py` ejecuta nueve escenarios x86: cinco
rechazos y cuatro descubrimientos (IA, humano no local, local y hotseat).
Prestigio, entrega de cartas y presentación son callbacks observados, no están
ejecutados por el fixture. `Letter::init` deja sin inicializar los bytes 6..7:
el contrato entrega los otros 14 bytes acreditados y no inventa los dos de pila.
Los argumentos con mitad alta no acreditada se comparan por sus 16 bits bajos.

Falta conectar estos efectos al anfitrión de 0x12, y aún no está implementado
el comercio de 0x52BFE0/0x52C019. No se afirma paridad de toda la expedición ni
validación en partida.
