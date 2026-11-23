# C10: precios de centros comerciales

`trade_center_prices` prepara los cálculos originales 0x52D4F0 (venta al
centro, incluida la cantidad que no se vende) y 0x52D6B0 (compra). Los precios,
capacidades y unidades se aportan como tablas: no se incluyen recursos del
juego. El cálculo conserva el paso de los coeficientes a `f32`, la división
entera con signo y la mitad alta del producto de 64 bits.

`x86_c10_trade_prices.py` contrasta 720 pares sobre el EXE inglés, con tablas
sintéticas, dos modos, ambas ranuras, clases 0–2 y los dos lotes de mercancía;
coinciden coste, ingreso y saldo no vendido. No se han probado en esta muestra
precios no finitos, clases o índices fuera de rango, ni todo el comercio de
0x52BFE0. El anfitrión debe cargar y verificar las tablas reales del juego.
