# C10: comercio del convoy con un centro

`trade_center_convoy::prepare` reproduce el flujo de 0x52BFE0/0x52C019 sobre
copias: contrato previo, ventas, contrato posterior, capacidad, hasta tres
compras, reparto por barcos y resumen. Usa los precios portados 0x52D4F0 y
0x52D6B0. Devuelve el estado y anfitrión preparados solo si todos los
callbacks tienen éxito; rechaza cadenas corruptas y presupuestos que no
convergen. El llamante debe publicar centro, convoy, barcos, comerciante y
estado del anfitrión como una sola transacción.

La interfaz `Host` deja explícitos el contrato 0x52C6C0, recomputar carga,
añadir mercancía y componer el informe. **No están conectados aquí**. El
`Report` expone cantidades e importes y la última mercancía usada, necesaria
para reproducir el formateo del original. Tampoco se cargan las tablas
comerciales del EXE en este módulo.

El oráculo `x86_c10_trade_convoy.py` ejecuta las funciones originales de
comercio y precio en 48 casos con dos barcos, cuatro combinaciones de compra,
tres presupuestos, dos modos y dos resultados del contrato. Contrato, carga,
capacidad y texto se interceptan con callbacks sintéticos; coinciden centro,
convoy, barcos, comerciante y orden de callbacks. Se prueba además la
reversión ante error del informe. El fixture no acredita el comportamiento
real de contratos, carga o texto ni la transacción integrada de la expedición.
