# C10: acontecimientos urbanos

La tarea programada `0x1E` prepara en copias el acontecimiento, las cartas y,
si corresponde, la creación pirata `0x5063C0`. La sesión publica juntas las
vistas de banda, navegación, fábrica y retratos. Se rechazan vistas
incompatibles antes de publicar. Si no hay banda, crea la primera junto con sus
barcos de reserva, usando los recursos verificados del ejecutable y del mapa.

La cadena aislada `0x4DFD44 → 0x5063C0` se contrasta con el ejecutable inglés
en 32 escenarios, con banda existente y crecimiento independiente de los
depósitos de barcos, capitanes y convoyes. Se usan adaptadores para interfaz y
nombres. `c10_town_event_persist` fuerza un acontecimiento junto a un barco en
una copia de `ConvoyMar`, lo despacha con el reloj, guarda, recarga y verifica
una segunda escritura idéntica. La ubicación de prueba se prepara localmente;
la prueba no acredita selección natural del acontecimiento ni paridad en juego.

El planificador `0x1F` selecciona los cuatro acontecimientos, programa `0x1E`
y emite `0x84` cuando corresponde. El cierre anual emite `0x7E`, que crea el
plan del año siguiente preservando los bytes residuales de la ranura libre.
Las imágenes de oficinas y negocios se concilian con las vistas vivas antes de
decidir. Si el original entraría en un ciclo, leería fuera de las tablas o
escribiría más de 16 ciudades fuera del registro, la transacción se rechaza.

La cadena `0x4E0334 → 0x4D8CF0 → 0x53FCA0 → 0x4D80F0` se contrasta en 192
casos x86 con crecimiento del scheduler, operaciones locales y remotas, y
serialización. Otros 96 casos contrastan `0x53F8B0 → 0x4D8CF0 → 0x4D80F0`
para `0x7E`, incluyendo ciudades no hanseáticas, mezcla y bytes residuales;
la ampliación usa el asignador x86 adaptado a memoria cero. El reloj de sesión
también ejecuta selección, petición y guardado, y el cierre anual crea y
recarga el plan siguiente. El original suma repetidamente la ciudad cuyo índice
es la posición de mercancía en una de sus tablas; se conserva ese comportamiento.

La creación de una banda inicialmente ausente se ha
contrastado en 12 casos de `0x513720 → 0x513D80 → 0x516170`, con puerto real
`0x5159C0`, piezas y reservas. Otros 12 casos recorren `0x5063C0`: ocho
coinciden íntegramente y cuatro se rechazan porque agotan el pool de barcos
durante la creación de reservas. En esa situación el original conserva un
puntero al bloque anterior y lo utiliza tras el `delete`; el oráculo demuestra
el acceso desmapeando ese bloque, sin afirmar qué haría el heap de Windows.
También se rechazan listas de orden naval circulares: el original no terminó
con la entrada equivalente dentro de 50 millones de instrucciones.

La prueba de sesión ejecuta dos altas seguidas tanto con banda existente vacía
como sin bandas iniciales; comprueba sus cadenas, S05, recarga y segunda
escritura. Son capturas forzadas sobre una partida y no prueban paridad en juego.

La modalidad 2 de `0x5063C0` selecciona el tipo de barco según tres
rectángulos de coordenadas con signo del EXE inglés, y aplica dos niveles de
mejora; fuera de ellos o con destino inexistente escoge el tipo 3. El puerto
usa los 24 bytes de `0x00673564`, comprobados contra ese ejecutable por la
prueba. Además de los doce casos anteriores, se contrastan doce de modalidad 2
en los extremos de las tres cajas, fuera de ellas y con destino inexistente;
cuatro ejercen el rechazo por el puntero obsoleto ya descrito. El oráculo usa
el constructor original completo con adaptadores de nombres e interfaz.
