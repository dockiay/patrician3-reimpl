# Recursos de apertura de guarida

El adaptador aislado `p3-sim/src/pirate_hideout_resources.rs` prepara valores
para el anfitrión de la escena; todavía debe integrarse en `Session`. No abre
el juego ni acredita el funcionamiento visual o el combate completo.

* `projection_from_archives` extrae `LIGHTNINGVIEW0/TileSize` y `DownVector`
  desde `scripts/iso.ini` con precedencia del último CPR. `0x587ED0` pasa los
  destinos `Class22+0x510` y `+0x4D0` al lector original `0x4BD0E0`. El CPR
  inglés probado contiene `(34,21,0)` y `(17,11,0)` (SHA-256 del recurso
  `4cf94cd1c0c14d1cb771ecaedf95f6eef1edfc9fe60028a1032b6dc55b71bc4f`).
  La proyección nativa es 34, 21, 17, 11; los valores 16 y 8 de pruebas
  históricas eran parámetros sintéticos, no el recurso inglés.
* `position` consulta las 40 posiciones de la tabla cargada por `0x51E440`;
  usa el índice de `band+0xE`. Rechaza coordenadas no representables y
  destinos fuera de tabla, sin presuponer que la entrada es válida.
* `portrait` recorre la cadena de barcos desde `band+0xC`; selecciona el
  primer capitán con `captain+8 > 0x20`. Si no existe, reproduce nombre 45 y
  el byte bajo del recuento real de apellidos comunes (`0x6DDB74`). El
  anfitrión debe aportar ese recuento desde los recursos de nombres cargados;
  ciclos y pools truncados se rechazan de forma explícita.
  Tanto la cabeza como el siguiente enlace terminan al alcanzar cualquier
  índice >= capacidad de barcos, según `cmp`/`jae` de 0x515AF4 y `cmp`/`jb`
  de 0x515B42; no es una comparación exclusiva con el centinela `0xFFFF`.
* `names` toma los bancos vivos de la sesión y aplica el selector de sexo,
  el índice y el primer nombre alternativo de `0x512A60/0x512AE0`. `texts`
  exige las cinco cadenas del `BattleTextBank` validado contra el EXE inglés.

`cargo test -p p3-sim --test c10_hideout_resources_x86` con `P3_EXE`, `P3_CPR`,
`P3_SAVES` y `P3_ORACLE_PY` ejecuta el lector x86 de vectores con INI y
`sscanf` adaptados, el cargador x86 de 40 ciudades, 200 cadenas de banda contra
`0x515AE0`, 176 pares de nombres contra el banco original, las cinco cadenas
contra la imagen cargada y 40 máscaras contra `0x5FFC10` con la proyección real.
Las pruebas rechazan recursos ausentes, duplicados, vectores incompletos y
cadenas cíclicas. Los puentes de archivos, CRT y reserva espacial están
adaptados; la prueba no ejecuta el cargador completo de escena ni la UI.
