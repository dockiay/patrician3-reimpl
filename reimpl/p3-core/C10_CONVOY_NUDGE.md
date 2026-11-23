# C10: desplazamiento de convoy antes del asalto

`ships::convoy_nudge::nudge` reproduce `0x0050E280`: recorre los miembros del convoy y desplaza sus coordenadas de coma fija 16.16 una unidad según la dirección del punto del mapa. Conserva el desbordamiento de 32 bits, la conversión a `u16` del identificador y el comportamiento de direcciones fuera de 0–7. Opera sobre copia y rechaza una lista cíclica sin cambios.

`cargo test -p p3-core --test c10_convoy_nudge_x86` contrasta 768 casos frente al ejecutable inglés: los 256 valores de dirección con convoy de cuatro barcos, convoy vacío e identificador inválido; incluye coordenadas cercanas al desbordamiento. La prueba requiere `P3_EXE` y `P3_ORACLE_PY` y se omite si faltan.

Es una dependencia aislada. Aún no conecta el ataque ni acredita la escena, el guardado S05 o una partida.
