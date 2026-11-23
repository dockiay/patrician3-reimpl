# Continuación sin interfaz y estadísticas

PC de Sergio, 4/11/2026. El ejecutable `c10` usa la misma sesión y el mismo
avance sin interfaz que el MVP. Lee los archivos originales y crea salidas nuevas;
rechaza los destinos que ya existen.

```powershell
cargo run -p p3-mvp --example c10 -- entrada.pat p2arch0_eng.cpr 1 123 salida.pat --out-pst salida.pst --celebration-state 0:255 --investigation-local --exe Patrician3.exe --battle-news-stamp 0
```

`--pst entrada.pst` es opcional. Si falta, las capturas diarias comienzan un
histórico nuevo; no reconstruyen los días anteriores. Con cero días y una salida
PST solicitada se escribe el estado vacío de `game_statistics_clear` (`0x54DDF0`):
seis words de cabecera a cero, cadenas ausentes, ningún comerciante y slack cero.
No se ejecuta una captura ficticia para generar ese archivo.

El JSON declara `statistics_origin`: `loaded` (había entrada), `new_capture`
(histórico iniciado durante este avance), `new_empty` (cero capturas) o `absent`
(no había histórico ni se solicitó escribir uno). Incluye los hashes de PAT/PST
y las rutas utilizadas. Para continuar el histórico se deben conservar **ambos**
archivos; `--out-pst` exige una salida PAT. Si se carga `--pst` y se guarda PAT,
también se exige la salida PST.

Además del PAT/PST, la reanudación necesita el estado del anfitrión: `final_crt`,
`final_celebration`, `investigation_local` y `battle_news_stamp` del JSON.
El último valor se repone con `--battle-news-stamp VALOR` (u32 decimal, cero por
defecto), después de preparar la flota. El contador de noticias de combate no es
un campo de la partida ni del PST.

La publicación abre todos los destinos con `create_new` antes de escribirlos,
sin reemplazar archivos previos. Si una creación o escritura falla, cierra y
retira solo los archivos creados por esa llamada. Las escrituras se sincronizan;
esto garantiza reversión ante errores detectados por el proceso, no una
transacción conjunta de dos archivos ante una pérdida de alimentación.

Prueba local de la CLI, con `P3_SAVES` y `P3_CPR` configurados:

```powershell
cargo test -p p3-mvp --example c10
```

La prueba cubre capturas nuevas de cero y un día, lectura de las salidas y hashes,
conservación de la entrada, contador externo y conflictos de destino sin PAT
parcial. No abre el juego ni sustituye el contraste completo de 30 días con x86.

La salida incluye `start_market_quotes` y `end_market_quotes`: por ciudad y
mercancía, precio teórico de compra y venta para un lote de referencia (2000,
200 o 10 unidades internas según mercancía). Son cotizaciones calculadas a
partir de las existencias, umbrales y dificultad actuales; un cero de compra
puede indicar que el mercado no comercia o no alcanza el mínimo del original.
No representa una compra ejecutada ni acredita disponibilidad de almacén.

La fase de cuadrillas necesita la tabla del ejecutable inglés verificado.
Se puede aportar con `--exe`; si no se indica, la CLI intenta usar
`Patrician3.exe` junto al CPR. Si la fase necesita la tabla y no está
disponible, detiene el avance sin inventar valores.
