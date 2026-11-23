# C10: dirección marítima de la ciudad

`settlement_sea_directions` conserva las 40 filas de `AusrichtungDesMeeres` en StadtDaten.ini, leídas con la misma prioridad de archivos que las posiciones urbanas. El cargador original 0x0051E440 guarda el byte bajo en TownData+0x31 y usa cero si no hay clave; 0x0050E280 consulta este campo al orientar un convoy al atacar una guarida. La dirección deja de ser una entrada inventada para la futura conexión del scheduler.

`cargo test -p p3-sim --lib sea_direction`, con `P3_EXE`, `P3_ORACLE_PY` y `P3_CPR`, ejecuta el cargador original aislado para las 40 filas reales y compara el byte resultante con Rust. El límite INI y CString está adaptado, sin abrir el juego. Otros tests comprueban el valor por defecto y el truncamiento de byte. El parser rechaza valores enteros inválidos en lugar de aceptar entradas mal formadas; no acredita el ataque, el scheduler ni S05.
