# Fase diaria 0x00514000 (C10)

Traducción del controlador sobre `flatmem::Mem`, con llamadas explícitas al
anfitrión para retirada, disolución y destrucción de cuadrilla, creación y mejora
de barcos, tripulación, capitán y cola B5. El llamante debe usar copias y
publicarlas conjuntamente sólo tras completar toda la llamada. El campo
`ship+0x158` utiliza directamente la traducción verificada de 0x0051A4C0.

`x86_c10_pirate_group_tick` ejecuta 128 escenarios contra el binario inglés
verificado, con llamadas de frontera trazadas y deterministas. Contrasta los
bytes completos de cuadrilla, flota y piezas, orden/argumentos de callbacks,
reparación, selección, guardas, convoy y destrucción. Cubre retirada, creación,
mejora, tripulación, capitán, cola B5, disolución y liberación. Las dependencias
se interceptan en esta prueba; esto NO prueba que los adaptadores vivos existan.

Falta integrar el callback del tick naval, reconstruir punteros del gestor para
las bandas S05 y conectar realmente las dependencias de este trait (sin mocks).
La transacción debe preservar el orden y el CRT/cola. Quedan por ampliar los
escenarios de listas cíclicas inválidas y la disolución del escondite vivo.
Nada se probó en partida.

`target_crew` reproduce 0x005185C0 para su consumidor de palabra baja: 288
casos x86, capacidades negativas/extremas, cuatro clases y seis niveles.

`allocate_pirate_captain` conecta pool e inicializador reales (flag=false),
crecimiento de64 y salario. Contraste x86 de30 reservas, incluyendo crecimiento;
la dirección del pool puede variar y se compara su contenido, cabeza y capacidad.

`town_weapon_level` reproduce 0x0052BB00: 267 contrastes x86, incluyendo
los umbrales de experiencia y el máximo u32.

`arm_ship` implementa 0x0051A4E0 (armas, potencia y carga): 640 escenarios
x86 con 24 ranuras, clases/niveles variados, huecos y límites de peso.

`allocate_reserve_ship` reproduce 0x00507D60(...,0,3), incluyendo crecimiento
de64: 128 escenarios x86 y contraste de todo el pool con malloc inicializado a
cero en el oráculo. El anfitrión debe publicar los vectores ampliados y las
referencias en una transacción; la memoria no inicializada del original no se
considera acreditada para todas las ejecuciones.

El alta completa 0x00516170 vive en `p3-sim/pirate_group_create`: incluye
reserva y crecimiento del pool, cadenas de banda, estadísticas, nombre, mejora,
armamento, carga, posición y capitán. Se ejecutan48 escenarios en x86 con
todas esas funciones originales; sólo banco de nombres y recursos se suministran
como datos controlados. Incluye crecimiento naval y de capitanes, pero no
acredita todavía su publicación en el tick de la sesión.

La baja 507FB0 tiene adaptador `p3-sim/pirate_group_remove`: ejecuta 515F00
antes de destruir el enlace +4, corrige orden x, limpia ruta y devuelve los
eventos externos pendientes.16 escenarios x86 de flota/cadena pasan. Este
contraste no cubre todos los eventos de interfaz, pool ni propietarios humanos.

Disolución 513EC0 con reservas y convoy activo (incluida la liberación 50A3D0)
contrastada en8 escenarios x86. El destructor del plano usa una frontera de
memoria, observada sin validar los bytes del objeto liberado. Baja con ruta y
PoolRelease conserva ambos efectos para publicación externa. El evento
`ConvoyRelease` de RemoveEvent es realmente 515F00: se verifica que el
desenlace ya se haya aplicado, y se rechaza cualquier discordancia.

`free_route_stop_pool` reproduce 4D4DD0(índice,0),64 cadenas circulares
x86; valida corrupción antes de mutar. Debe conectarse al bloque S05 vivo.

La variante B5 `free_route_stop_pool_with_followers` porta 4D4DD0(índice,1)
y 509030: desliga el índice de todos los barcos antes de liberar.64 casos x86.
