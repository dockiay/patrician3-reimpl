# C10: operación B4 de banda pirata

`pirate_b4_stage::prepare` prepara transaccionalmente el despachador original
`0x00542E40`: alta o sustitución de una de las cinco bandas, elección de puerto,
creación de sus barcos y capitanes y actualización de los pools. Una ranura de
índice cinco o mayor consume la operación sin cambios, como el original.

El módulo opera sobre imágenes y recursos explícitos; devuelve imágenes nuevas,
la identidad lógica elegida por el llamante para una banda nueva y los efectos
ordenados de la disolución (`removed_routes`, `remove_events`, `merchant_owners`
y `removed_plan`). El llamante debe conciliar y publicar esos efectos, los pools,
las vistas de bandas y el índice espacial como una transacción antes de quitar
B4 de la cola. El token no es un puntero x86 ni se serializa como tal. El módulo
no conecta por sí solo el despacho al bucle de sesión ni agota esa cola.

Las ubicaciones, orden y ocupación de puertos, bytes de piezas, banco de nombres,
fecha y población deben proceder del estado y recursos reales. Se comprueba que
ciudad 0, puerto 0, población y tick coincidan con las vistas vivas. Se rechazan
identidades de banda inconsistentes, datos insuficientes y tamaños incompatibles
con las regiones aisladas; el rechazo no modifica las imágenes de entrada.

`c10_pirate_b4_x86` compara 200 escenas ejecutando en una misma memoria x86
`542E40`, `513EC0`, `513D80`, `5159C0` y `516170`, además de sus dependencias
reales. Cubre alta, banda previa vacía, baja de dos barcos, liberación de plano
y crecimiento real de los pools; incluye puertos ocupados y banda vecina. El
heap y el banco de nombres se adaptan en el oráculo. Se cotejan las imágenes de
gestor, barcos, capitanes, convoyes, comerciantes, ciudades, banda y piezas.
El perfil x87 es 0x027f. Una prueba separada guarda y recarga en S02/S05 una
banda y flota creadas sobre una partida real con recursos sintéticos.

La comparación no acredita todavía la publicación de todos los efectos de baja
sobre la sesión y su cola, ni el recorrido completo desde `514920` hasta B4;
esos puntos pertenecen a la integración. Tampoco sustituye a pruebas en partida.

## Puente de vistas y cola

`pirate_b4_live::prepare` recibe una imagen viva de navegación, factoría,
ciudades, comerciantes, bandas, rutas, depósito, grupos y cola. Comprueba que
B4 siga al frente, prepara su resultado y usa `pirate_group_effects::prepare`
para aplicar ordenadamente `PoolRelease`, rutas liberadas y `MerchantChanged`
con las capturas de propietarios tomadas durante cada baja. Concilia los cambios
en factoría, amplía las vistas para nuevos barcos/capitanes y solo retira B4 en
la copia devuelta. Un error conserva las entradas y la cola.

El puente deja a su llamador la publicación conjunta en `Sim`, las proyecciones
de barcos/convoyes y el bombeo de la siguiente operación. No debe quitarse B4
fuera de esa publicación atómica. Su prueba de persistencia usa la representación
serializable real y una partida AUTO como base; la información de puertos y de
nombres es sintética. No equivale a ejecutar el juego ni a contrastar el
recorrido completo de `514920` a B4 en una sola memoria x86.

El armamento de `516170` procede de `Town[0x006DE52E]+0x810` (ciudad del
Ältermann). Se contrastaron 200 recorridos con 16 ciudades y seis niveles de
experiencia, además de la inicialización de estadísticas con Town 0.
