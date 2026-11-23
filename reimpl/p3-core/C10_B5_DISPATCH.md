# C10: despachador de la orden naval B5

PC de Sergio. `execute_operation` 0x535760 tabla 0x537030[B5] salta a
0x536E8C: lee la ranura u32 de operación+4 y retorna si >=5 o si el puntero
manager+0C+4*ranura es nulo. En otro caso llama a0x514D40 con el grupo en ECX,
límite en el low16 de operación+0C y segundo parámetro low16 de operación+08, usado como valor inicial
compartido por los callbacks de equipamiento 515750. Los bits altos de las palabras cargadas en registros
proceden de residuos de pila/registros y no forman parte del contrato.

`x86_c10_b5_gate.py` ejecuta 535760 x86 real con el handler514D40 observado;
512 escenarios de slots, presencia y extremos de argumentos coinciden con
`economy::pirate_b5::decode`. El porte preexistente de 514D40 tiene1200 casos
x86 aislados con16 callbacks simulados (`x86_c08_pirate_launch`).

Esto NO conecta ni prueba los16 callbacks reales ni su transacción de barcos,
capitanes, convocación, cartas, tareas, colas y guardado. La integración del
host y comparación de la cadena completa siguen pendientes; no consumir B5
como si fuese un no-op cuando el grupo exista.

`pirate_b5_memory::prepare` ejecuta la operación completa sobre memoria y host
clonados; devuelve ambos sólo al completar todos sus callbacks. `NativeBasics`
conecta LCG, consulta de reputación, recálculo de carga y liberación de capitán
a los portes existentes; cualquier otra dependencia retorna error y revierte
la operación. No se ha conectado todavía a la cola real ni se han contrastado
las cadenas completas: el anfitrión debe implementar los callbacks restantes.

`x86_c10_b5_live.py` ejecuta 535760→514D40→5098B0 realmente sin
interceptar callbacks en tres escenarios (capitán libre, no libre y ya libre).
Coinciden bytes completos del gestor, barco, capitán y banda. La limpieza
de rutas se publica al llamante mediante `cleared_routes` y los avisos de
interfaz mediante `ui_invalidations`. Esto aún no cubre lanzamiento exitoso
con convoy, cartas y tareas.

El callback `ConvoyDone` reproduce 50AAE0 e inserta en orden x mediante
50CE50/50D0C0 cuando ya hay líder válido: cinco escenarios contra x86 real
con bytes completos de gestor, convoy y flota. La elección de líder ausente
retorna error explícito. El caso líder igual a cabeza del mar también se
rechaza para no repetir el bucle no terminante observado en el original.

`CreateConvoy` reproduce 5062F0→501680→501BE0: cuatro casos x86 reales
con el temporal FFFF usado por B5, bytes completos de gestor, convoy y barco.
El original inicializa el temporal a FFFF antes de desplazar ESP con `push edi`;
50AAE0 reemplaza después el enlace con la cabeza anterior. Si el pool está lleno,
509AE0 amplía 32 ranuras; 128 escenarios contrastan el crecimiento. El EXE deja
sin inicializar otros bytes de las ranuras nuevas: los ensayos usan un asignador
x86 a cero, igual que `w8_seed`, sin afirmar paridad de bytes indeterminados.

El segundo parámetro sí afecta el equipamiento: el original pasa un puntero al
valor inicializado con low16 de operación+08; `515750` lo actualiza y el mismo
valor se pasa al siguiente barco. `x86_c10_b5_scratch.py` prueba con dos
rellenos de pila y un callback observador que suma uno: en el caso de dos
barcos el valor pasa de18880 a18881. El puente conserva esa secuencia mediante
`Effect::ShipSetup(id, valor)` y usa su retorno como nuevo valor compartido.
La rutina real de equipamiento y demás dependencias siguen pendientes.

`ShipSetup` ejecuta ahora el porte existente de 515750 con los seis callbacks
nativos de marcar, ranuras, recuento, nivel de ciudad, armamento y marineros.
El temporal se mantiene para los siguientes barcos. Doce escenarios ejecutan
535760→514D40→5062F0/515750/501BE0/50AAE0 y sus dependencias en x86, sin
interceptar ningún callback, para uno o dos barcos, tres dificultades, dos niveles y dos valores iniciales;
el caso de nivel2 instala armas reales, el caso de dos barcos comprueba el
valor mutable compartido; los bytes completos de gestor, banda, barco, convoy, capitán, comerciantes y
ciudad coinciden con Rust. No cubren crecimiento de pool ni la rama de carta,
tarea y baja que corresponde a otra condición de B5.

`pirate_b5_stage::prepare` traduce TickWorld y cinco bandas a memoria
temporal, llama el porte y devuelve gestor, flota, capitanes, convoyes,
comerciantes, ciudades, bandas y host sólo tras éxito. Las pruebas cruzan los
12 escenarios completos x86 también a través de esta frontera y comprueban que
una parada de ruta no conectada revierte todas las vistas. El llamante debe
confirmar conjuntamente esas vistas, ruta y efectos externos; aún no es S05.
