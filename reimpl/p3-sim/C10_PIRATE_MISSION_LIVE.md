# C10: aproximación de misión pirata y reposición

La fase 0 de la misión `0x32/3` detecta los barcos próximos a la guarida, marca la tarea y el comerciante, y repone las bandas en el orden del original. El planificador se suspende antes de consumir la tarea; la sesión prepara flota, capitanes, bandas y efectos en copias, los publica juntos y reanuda el planificador en el mismo tick. Si falta un recurso o las vistas de bandas, asignador, reservas, posiciones o tamaños no coinciden, no publica la aproximación. S05 rechaza el estado intermedio pendiente.

Se corrigen dos recursos usados al crear los barcos tanto en esta aproximación como en el tick diario `0x00514000`: los capitanes usan los recuentos ordinarios D0/D4, y el nivel de armamento se lee de la ciudad del Ältermann, no necesariamente la ciudad 0. Si todas las bandas aplicables ya tienen plano, el original no consulta la tabla de dificultad y tampoco la exige la etapa nativa.

## Contraste

- `c10_pirate_mission_approach_full_x86`: cuatro combinaciones de uno o dos barcos próximos, dos bandas y límites de reserva 1 o 3 ejecutan el original `0x004EAFE4 → 0x005144A0 → 0x00516170` y comparan tarea, comerciante, gestor, barcos, capitanes y cabeceras de bandas con la etapa nativa. Incluyen crecimiento de ambos pools de 8 a 72. El heap y el lookup del banco de nombres están adaptados; `FPCW=0x027F`. La variante de tres reservas comprueba que cambiar la ciudad del Ältermann modifica el armamento original.
- La misma prueba compara el tick original `0x00514000` con su etapa nativa para una banda, incluyendo barco, capitán y armas de una ciudad del Ältermann distinta de la ciudad 0.
- `pirate_mission_approach_tests`: una copia de AUTO con banda preparada ejecuta el despacho de sesión, crea dos barcos y capitanes y guarda y recarga tarea, comerciante y S02, tanto con slots libres como forzando crecimiento de 64 barcos y capitanes. Comprueba rollback por recursos o vistas incompatibles, que una reserva de otro destino no bloquee el recorrido, y un caso observable con D4=80 y D6=1.
- Las pruebas de `C10_PIRATE_REPLENISH.md` cubren la reposición por separado, incluido su S02; las de `C10_PIRATE_MISSION_APPROACH.md` cubren el barrido con callbacks interceptados.

Las pruebas x86 requieren `P3_EXE` y `P3_ORACLE_PY`; la prueba de sesión requiere `P3_SAVES`. Si faltan, se omiten. La sesión se comprueba llamando a la fase del planificador; no se ha contrastado en una sola ejecución el reloj completo y S05 con el original.

La fase 1, que prepara o refuerza el asalto a la guarida y su `TownAttackBattle`, sigue rechazada cuando alcanza ramas aún no conectadas. No se ha validado en partida ni se acredita paridad del combate.
