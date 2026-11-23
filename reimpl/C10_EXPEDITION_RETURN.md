# C10: fase inicial de regreso de expedición

PC de Sergio. `st_expedition_return` (0x004E6A54) recibe una tarea 0x12
generada cuando un convoy sale por el borde del mapa. En la fase inicial, el
original calcula las distancias de ida y vuelta con 0x0052D950 y 0x0052DB50,
las pondera por la velocidad, suma la demora a la fecha, aplica el 50 % extra
cuando `convoy+0x38 == 0` y cambia `task+0xA` de `0xFFFF` a 4. Un índice
de convoy fuera del pool elimina la tarea. Se prepara sobre una copia: velocidad
nula, entrada truncada y coordenadas no contrastadas no publican cambios.

`economy::expedition_return_start` implementa esa fase y ambas métricas; el
despachador C10 usa el convoy vivo de navegación o la fábrica si no hay
navegación. Un escenario dirigido comprueba despacho, guardado, recarga y
segundo guardado. El oráculo `x86_c10_expedition_start.py` ejecuta 600 tareas
con el EXE inglés y ambos auxiliares originales sin sustituirlos; sus bytes de
tarea y retorno coinciden. El contraste requiere `P3_EXE` y `P3_ORACLE_PY`;
el de sesión requiere `P3_SAVES`. Sin esas variables los tests se omiten.

**Límite:** solo se admite la fase inicial y se conserva un rechazo explícito
para el regreso (fase no `0xFFFF`) cuando el convoy existe. Conectar daños,
contratos, cartas y vuelta física sigue pendiente. No equivale a la tarea 0x12 completa,
a una expedición jugada ni a paridad para estados corruptos fuera del dominio
probado. No se abrió el juego ni modificaron DLL.

El tramo de desgaste y sucesos del retorno (`0x004E6C78..0x004E7099`) está
portado por separado en `economy::expedition_damage`. Sus entradas y efectos
se preparan sobre copias; la cadena cíclica, una división nula o una raíz fuera
del dominio contrastado se rechazan. En 1.200 escenarios x86, 112 tormentas,
119 ataques y 20 saqueos coinciden en barcos, dinero, banderas y estado LCG.
Este tramo aún no se ejecuta desde la tarea completa y no comprende centros,
cartas ni fin de convoy; el rechazo de fase de retorno sigue vigente.

`economy::expedition_discovery` prepara `0x005319D0` (primer descubrimiento,
prestigio, bandera, carta global 0x56 y avisos) sin publicar entradas. El
anfitrión `expedition_discovery_event` agrega la carta de forma transaccional.
En 128 casos coincide la mutación del comerciante, la bandera y los valores
de carta/avisos con el x86; las mitades altas no definidas de argumentos de
interfaz y bytes 6–7 de la carta no se atribuyen. Otros 16 escenarios comparan
0x5319D0 → 0x4D6530 → 0x4D62B0 originales con el pool nativo, incluido su
crecimiento; también se comprueba rechazo atómico. Se trata aún de un efecto
aislado: la bandera de mundo y el resultado deben conectarse a la fase 4.

La bandera `GameWorld+0x89` se conserva en `world.header[0x10]` de S05 y
se normaliza a 0/1 al cargar, como hace el lector original. Las imágenes
inicial y viva permiten detectar conflictos sin tocar el modelo, y la bandera
sobrevive a la reconstrucción de la flota en Session. El test del lector y
escritor x86 cubre los bytes 0, 1, 2, 0x7F y 0xFF; se comprueban también
doble guardado, recarga, reconstrucción de Session y rechazo de conflicto.

`expedition_discovery_event::prepare_mark` prepara un waypoint con las marcas
en mapa propio y global, primer descubrimiento y carta. En 64 casos, la cadena
original 4F93C0 → 5319D0 → 4D6530 y serialización S05 coincide con la copia
nativa: 48 puntos especiales, 16 no especiales y 32 recompensas, también con
crecimiento del pool. Un error de carta no publica comerciante ni mapa. El
caller debe intercalar cada waypoint con su centro, como hace el original.
