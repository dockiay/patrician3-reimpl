# Cierre de banda pirata vacía

`pirate_empty_band_end::prepare` ejecuta el porte `pirate_end::band_on_battle_end` (`0x514920`) sobre imágenes copiadas. Conecta la liberación nativa del convoy vacío, la limpieza de referencias de comerciantes, la actualización de la primera misión asociada, el estado CRT y la operación B4; devuelve todo junto con los avisos de interfaz. El llamante debe publicar también esas operaciones y avisos como una sola transacción.

El contraste `c10_empty_band_end_x86` ejecuta directamente `0x514920`, la liberación de convoy, el lookup de tareas, el `rand` y la cola de operaciones originales en 40 escenarios. El getter TLS y la liberación del heap son adaptaciones del entorno; las llamadas de dominio permanecen reales. Se comparan imágenes navales y de comerciantes, cabecera y desaparición del plano, tareas, estado CRT y registros de la cola. La prueba de rechazo conserva las entradas.

Alcance: exige plano presente, ningún registro de piezas y convoy defensor válido y vacío. Las bajas, piezas, crecimiento y la retirada de la propia escena todavía no se tramitan. S02/S05 y la entrega de operaciones/avisos corresponden al llamante; no es una prueba en partida.
