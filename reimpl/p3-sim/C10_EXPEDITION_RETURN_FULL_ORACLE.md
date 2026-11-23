# Oráculo x86 de retorno de expedición

`x86_c10_expedition_return_full.py` acepta por stdin el mismo JSON de casos que
`x86_c10_expedition_damage_finish.py`. En una memoria Unicorn ejecuta la
función original de tarea `0x4E6A54`, con centros no coincidentes, incluyendo
el desgaste, `0x502B20`, la construcción de carta humana `0x63`, la inserción
en el pool y los serializadores de barco, capitán, comerciante y cartas.
Los efectos de presentación, el banco de nombres y `0x445010` se adaptan; la
ruta entregada es la del caso. Los nombres adaptados son `GIVEN56` y
`FAMILY78` y los argumentos reales se registran en `names`. El puntero de
ruta se normaliza en el resultado de barco solo después de ejecutar el
serializador original. CString/CRT utilizan `install_external_services` del
oráculo de reloj; no se instala DLL ni se abre el juego.

Observado con el EXE inglés y la escena local de nueve pruebas existente:
cuando el barco líder muere y el suplente con capitán propio sobrevive, la
función de nombres recibe `0x56/0x78` y la carta real contiene
`GIVEN56 FAMILY78`. Dos escenas de pérdida total también se ejecutaron. Los
casos son sintéticos y el origen de la ruta se intercepta; este oráculo no
cubre comercio en centros ni despacho del scheduler, y su salida todavía debe
compararse con `expedition_return_stage`, que implementa otro agente.

Admite `waypoints` y `discovery_points` en el JSON; para un punto válido exige
puntos de descubrimiento no vacíos, y devuelve
`global_map` y `world_flag`. Un caso con (999,999) recorre el mapa y vuelve con
el texto de centro no hallado en la carta. Por defecto todos los centros del escenario son
inválidos. Con `world_flag=0` también se observan en la misma ejecución el
primer hallazgo, su carta global `0x56`, el aviso y la ventana adaptada; se
devuelven `notifications` (mitades bajas de los seis argumentos) y `windows`.
El comercio con cantidades sigue pendiente de contrastar en este oráculo.

Con `centers` (15 registros de 30 bytes más contratos de 16 bytes) y `locations`, el
oráculo también ejecuta los centros originales y los serializa con `0x52BB30`
en `serial_centers`. La escena dirigida de centro tipo 1 (999,999), sin dueño
ni mercancías, ejecuta descubrimiento `0x52D0B0`, sus cartas 0x1C, comercio
`0x52BFE0` sin intercambios, desgaste, cierre, carta 0x63 y guardado, en esa
única ejecución x86. Los índices del capitán de las cartas de descubrimiento
preceden al cierre; los nombres de la carta de regreso se consultan después.
Se han ejecutado por separado
una venta de 10.000 unidades por 822 monedas y una compra de 20.000 por 1.413;
en ambos casos el cierre posterior y la carta 0x63 incluyen los efectos del
comercio. Está pendiente contrastarlos con el stage y scheduler nativos.

La palabra de control x87 se fija a `0x027F` (53 bits) tras inicializar los
servicios CRT y se comprueba al terminar. Una versión anterior de esta sonda
dejaba el valor inicial de Unicorn `0x0000` (24 bits): en una compra el valor
de carga de 18.000 unidades se redondeaba erróneamente a 1272; con `0x027F`
el x86 devuelve 1271, igual que Rust. Todos los contrastes anteriores con esta
sonda deben repetirse con el control corregido.

El oráculo reconstruye listas x86 de hasta 32 contratos por centro. La escena
de venta con contrato existente (10 unidades al precio 120) completó y retiró
el contrato, registró ingreso total 1857 y caja final 4857 e incluyó contrato
y venta ordinaria en la carta. Se repitió con FPCW=0x027F; falta compararla con
el stage nativo. La creación de contratos no se acredita en este escenario.
