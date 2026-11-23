# Tarea 0x0E: fin de partida

`004E4A44` comprueba seis objetivos: rango, alcaldía, elección de la Liga,
clasificación al plazo, mayor fortuna al plazo y población agregada. El modelo
conserva las reprogramaciones y devuelve un evento con objetivo, ganador, tipo
de ventana y ciudad natal para el aviso. Al terminar marca `game_end_stopped`.
La fecha de inicio procede de la cabecera S05 +0x14; la tarea se guarda en el
scheduler S05. El evento de presentación y la bandera de ejecución son estado
de sesión; su reproducción visual queda pendiente. El bombeo manual del MVP
consulta la bandera y deja de avanzar tras el final.

Los objetivos de clasificación y población calculan el ganador con el puerto
de `004220B0`/`004220D0`, usando comerciantes, ciudades y oficinas vivas. La
puntuación depende de `GameEndScreen+0x34C`, byte que el constructor original
no inicializa. Si el anfitrión no aporta ese valor, se calculan ambas variantes:
cuando dan un ganador distinto se conserva la tarea y se informa del límite.
La interfaz se interceptó en el contraste x86; no se abrió el juego ni se
validó la presentación en partida.

El núcleo coincide en 240 escenarios x86; la tabla de clasificación completa
coincide en 96 casos; y seis escenarios con una partida local contrastan el
despacho a través del reloj y la recarga de S05, incluyendo la clasificación
original para los objetivos que la requieren. En la comparación de una
tarea reprogramada se excluye su enlace, que el despachador mueve después de
la llamada aislada al manejador y se comprueba mediante la recarga.

En la rama de rango sin ganador, el original usa como origen del nuevo plazo el
registro de índice igual al número de comerciantes (`004E4BD6..004E4BF7`). Si
ese índice cae fuera del pool el puerto rechaza la lectura, que en el original
accedería fuera de los registros. Las duraciones no reconocidas solo están
definidas como ilimitadas para alcalde y Liga; otras ramas usan una variable de
pila sin inicializar y se rechazan.
