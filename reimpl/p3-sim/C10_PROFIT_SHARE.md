# Reparto de participaciones: contraste encadenado

La tarea 0x18 se contrastó en una ejecución x86 desde el despachador original
`004D85C0`, con alta real de la deuda, entrega de cartas `004D6530` y los
serializadores originales `004D80F0`, `004D62B0` y `004F4010`. Se compararon
las tareas, el depósito, las cuentas de todos los comerciantes y los dos
registros de comerciante serializados con la transacción y S05 de Rust.

Los seis escenarios cubren pago íntegro, deuda parcial, deuda con crecimiento
de 128 a 256 ranuras, caída del valor, participaciones del público y baja por
participaciones nulas. Cada uno se guarda y recarga y admite un segundo
guardado idéntico. El caso de crecimiento comprueba deuda, capacidad y longitud
de variables de guion. El montón y las funciones de presentación se adaptan en
el emulador; no se abrió el juego ni se probó la presentación.

La cola crece sobre una copia antes de publicar efectos. Si la ampliación
desbordaría su capacidad u16, o si una deuda con fecha desbordada se ordenaría
por delante de la cabeza activa, el puerto rechaza la operación sin mutar el
estado. El segundo caso sigue pendiente de un contrato del comportamiento
original con esa ordenación. Una prueba aislada comprueba el primer límite.


El contraste extremo de fecha `0xFFFFF000` incluye el dispatcher completo,
ambos comerciantes, el depósito de cartas y S05. La deuda adelantada se
convierte en la cabeza y el original reutiliza sus campos; el deudor 255 se
elimina en el perfil contrastado. El oráculo mantiene mapeada la memoria fuera
de la tabla de comerciantes; con más de 255 comerciantes, el puerto rechaza
esa rama. La continuación más allá de la tabla trimestral está acotada a la
ventana observada del EXE inglés; fuera de ella se conserva el rechazo.
