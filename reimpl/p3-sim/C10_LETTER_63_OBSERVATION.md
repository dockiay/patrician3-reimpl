# C10: observación de la cadena final de la carta 0x63

La función x86 `0x4E7726` copia una cadena terminada en cero desde el
puntero `[0x006B9634]`, que en el EXE inglés apunta a `0x006CC936`. En este
binario esa dirección queda en la parte virtual de `.data`, sin bytes en el
archivo: el cargador PE la inicializa a cero. El único literal absoluto de
`0x006CC936` encontrado en el archivo es ese puntero; el único de `0x006B9634`
es la lectura citada. No se encontró un recurso o escritura directa que
inicialice el byte. Esto no descarta escrituras indirectas ni desbordamientos.

Observación en PC de Sergio con `x86_c10_letter63_watch.py`: se restauró el
checkpoint local de rutas, se terminó de cargar `AUTO.pat` (cursor final
completo) y se ejecutaron 30 días con resolución de batallas headless. Se
observó `00 00` en `0x006CC936` justo antes del primer tick y al terminar, sin
escrituras Unicorn al byte durante la reanudación y el reloj. El checkpoint ya
contenía `00 00`. Los servicios y adaptadores son los de
`x86_c10_real_save_clock.py`; no es una ejecución con interfaz ni en partida.
El observador se instala antes de restaurar la memoria y no ve las escrituras
ocurridas durante la ejecución que produjo el checkpoint, ni acredita otras
partidas, ediciones, configuraciones o rutas de ejecución. Por ello el
anfitrión mantiene explícito el texto dinámico y no presupone que siempre sea
vacío.

Procedencia SHA-256:

- EXE: `fe436dc5f8addc4d3aa437eb48a8bc6415d9ba5577c1027d06db9bb915eddece`.
- Partida: `0263531c207f437aab09c4d3c01eb8a4d5ccffc1198d94223eddb2ee364133b5`.
- CPR 0: `6ebd9aa5a42c496316dd2d137ea99727d200ca557808c15d7ec1b473d4079009`.
- CPR 1: `45acc1f0df18f14c9d78d30b74a37c254bf6f19c3b28e3bc255178e2e6a1912f`.
- Checkpoint local: `ea2e20c692620f410aa9032f14d0e217fde0c3901f5f4a170ad635df257e9970`.
- Fuente que creó el checkpoint: `1a8afdd2f6b90f234dd2fdb29ad2a8905e80d88affb0b1e4e75799f7700b0a4b`.
- Fuente del oráculo de reloj en esta ejecución: `892a5e9b9c019ca8732f0718f3bfae5f1837012cfaf2f5c6a2800b845466cb01`.
- Observador: `4096793fb93f32f7ea29301b817834a13c9d27cd587a468e747b86b989493dab` (normalización LF).

El JSON local de salida es `coordination-runtime/c10-letter-63-watch-versioned.json`;
ni el checkpoint ni la partida están versionados. La sonda recibe las rutas de
EXE, partida, CPR y checkpoint mediante sus opciones `--help`.

Otra ejecución en PC de Sergio omitió por completo el checkpoint de rutas:
el observador se instaló antes de los constructores del perfil headless, se
cargó íntegramente AUTO y se ejecutaron 30 días. `0x006CC936..937` eran
`00 00` antes de los constructores, antes del primer tick y al finalizar, sin
escrituras x86 observadas sobre el primer byte. El resultado local está en
`coordination-runtime/c10-letter-63-uncached30-2b.json`; el EXE, partida y CPR
son los identificados arriba, el oráculo de reloj tiene hash
`892a5e9b9c019ca8732f0718f3bfae5f1837012cfaf2f5c6a2800b845466cb01` y el
observador normalizado LF `422eede3ecc0871242cd0e250e80bb130ae952fa4f8c84fd17340fefaf1cdd1f`.
No se ejecutó el arranque gráfico completo, ni otras partidas o ediciones;
se mantiene el sufijo como entrada explícita en la sesión general. Un perfil
headless equivalente puede suministrar vacío declarando esa hipótesis.
