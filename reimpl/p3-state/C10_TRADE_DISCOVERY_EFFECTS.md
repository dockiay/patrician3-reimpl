# C10: prestigio y cartas del descubrimiento

`trade_center_discovery::prepare` toma el centro en su formato serializado,
los convoyes/barcos/capitanes y las imágenes de comerciantes y prepara sobre
copias la actualización del centro, la reputación 0x4F8BD0 y las cartas 0x1C
de `add_message` (-1 a humanos, -2 a la cadena global). Devuelve por separado
los avisos de presentación; el llamante debe confirmar centro, comerciantes,
depósito de cartas y presentación en su propia transacción. La fecha y el
incremento de renombre se aportan desde el mundo y el EXE.

El oráculo ejecuta 52D0B0→4F8BD0→4D6530 y el serializador de cartas originales
en nueve escenarios con propietarios humanos/IA y tres perfiles de receptores;
incluye crecimiento del pool. Coinciden centro, comerciantes y pool completos;
se prueba rollback ante pool inválido y doble guardado/recarga S05 del pool.
UI y asignación/liberación genéricas están adaptadas. El host no está todavía
conectado al despachador de expediciones y no prueba paridad en partida.
