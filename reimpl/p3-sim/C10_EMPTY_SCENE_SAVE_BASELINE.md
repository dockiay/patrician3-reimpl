# S05 después de abrir y cerrar una guarida

`pirate_empty_scene_save` prepara la conciliación desde una instantánea y un
S05 capturados **antes** de la apertura. Usa la escena activa solo para
identificar su origen, y entrega una copia S05 con el resultado del frame.
No guarda la escena a mitad de ejecución. El host debe conservar los efectos
de apertura hasta atenderlos y no pasar un frame con B4 o efectos pendientes.

El puente compara como unidades los campos serializados conocidos de
comerciante y los campos de cabecera, fecha, enlace, tipo y payload de tareas.
Preserva cambios ajenos en campos distintos. Los registros de flota y los
bloques opacos, incluido el pool de cartas, se reconcilian de forma conservadora:
si han cambiado tanto la sesión como otro sistema, deben coincidir íntegramente
con el estado previo o final; no se mezclan bytes de estructuras desconocidas.
Comprueba el origen del gestor de batalla y analiza el pool, planificador y S05
resultantes antes de devolverlos. Ninguna entrada se modifica en caso de error.

`c10_hideout_full_save` parte de AUTO con una banda, convoy y tarea preparados;
encadena la preparación nativa de apertura, sus cartas, el frame destructor,
S05 y recarga, y fuerza un conflicto de tarea. Las entradas de la escena y la
presentación del test son sintéticas; el test no inicia el reloj ni la UI, ni
compara él mismo la cadena entera con x86. Las partes de escena, cartas y
frame tienen sus contrastes aislados documentados en los módulos anteriores.

## Grupos del gestor (PC de Sergio)

La base incluye ahora los grupos auxiliares reconstruidos del bloque S05. El estado final aporta explícitamente sus grupos y el gestor confirma su puntero y recuento; toda escena debe haber terminado. Si el bloque previo no coincide, falta la vista o el gestor la contradice, se rechaza sin mutación. Si los grupos no cambian se conservan los bytes originales; si cambian se escriben con el serializador de gestor ya utilizado por el proyecto.

El caso dirigido con AUTO sintético pasa de contador 2 a 1 y de 1 a lista vacía; comprueba recarga y rechazo de conflicto o vistas inconsistentes. La preparación previa de la base y su conexión a Sim/Session siguen pendientes, así como B4 y las rutas que cambian ciudades o rutas navales. La prueba no ejecuta el juego. La reconstrucción de filas hereda el límite documentado por `battle_manager::groups_from_save` para los tres bytes opacos.

Dos entradas adicionales usan los casos 1 y 21 del corpus `battle_manager_load_tick_save.json`: la prueba compara los bytes completos guardados con los ya obtenidos de `0x611AE0 → 0x6127C0 → 0x611950`. Los estados de escena se montan en Rust, y el asignador y la UI están adaptados en ese oráculo; no es una ejecución x86 encadenada de toda la misión.

## Conexión de sesión

La base privada se captura al interrumpir el reloj antes de abrir el asalto.
La sesión puede reconciliarla después de 0x94 y B4 y verificar su recarga; se
rechazan operaciones, efectos o errores pendientes. Los cambios de rutas se
tramitan mediante el serializador naval existente y un crecimiento de registros
solo se acepta si la vista guardada coincide con la anterior o con la final.
La prueba de recorrido usa AUTO preparado y recursos sintéticos; el original
se ha contrastado por etapas, no como una sola ejecución de extremo a extremo.

Tras conciliar la escena se preparan también los serializadores generales del
tick sobre la misma copia, sin confirmar checkpoints hasta verificarla. Una
prueba modifica un registro de asedio después de la base y comprueba que
sobrevive al guardado y la recarga; esa modificación es sintética, no un
contraste de un asedio completo con x86. Si algún serializador detecta un
conflicto, la publicación falla sin actualizar la sesión.
