# C10: publicación inmediata del reparto de viviendas

PC de Sergio, 4/11/2026. `0x00520BD0` repartía correctamente habitantes en
`NullHost.housing`, pero `OfficeTownHost` no publicaba el resultado en los registros
vivos de oficina. La siguiente ciudad con obras reponía datos antiguos, y el
balance del comerciante y S05 leían esas asignaciones antiguas.

El puente ahora publica número de casas, capacidades y habitantes inmediatamente
tras el recuento. Conserva la experiencia transitoria del administrador. No cambia
el algoritmo de vivienda ni el orden del reloj.

En `visibility/save/Ein/AUTO.pat`, el día original completo deja las oficinas
24/29/33 con 61/21/36 habitantes ricos; Rust retenía 59/19/33. La publicación
corrige esos tres bytes y dos bytes derivados de reputación del comerciante24.
La regresión compara todos los registros de oficinas y comerciantes con el
snapshot x86 day-01 y comprueba los valores vivos antes de guardar o recargar.

- Entrada SHA256: `0263531c207f437aab09c4d3c01eb8a4d5ccffc1198d94223eddb2ee364133b5`.
- Snapshot original SHA256: `9abce408f78ae84c3b565b6ef940faecfe9b3c30b0594d56e0401da6d871ccd2`.
- Perfil: jugador36, CRT123, fiesta(0,255), anfitrión, tramos de hasta8ticks;
  primer tick y luego día256, como `x86_c10_real_save_clock.py`.
- Prueba: `cargo test -p p3-mvp --test c10_housing_daily`, con P3_SAVES,
  P3_CPR, P3_EXE y P3_C10_ORIGINAL_DAY1 apuntando al snapshot original local.
- Regresiones: `x86_housing` y los tres `s05_*housing_original`.

El snapshot y la partida comercial permanecen ignorados. Este resultado es un
día de simulación original aislada y persistencia; no acredita30días ni juego.

## Rotación al segundo día

La segunda jornada reveló otra frontera: `0x0051BA10` pasa la última oficina al
principio, pero las vistas `housing`, `lists` y `tax_offices` conservaban los
`next` anteriores. En ciudades5/15/19, el recuento omitía las oficinas24/29/33:
faltaban80 plazas privadas y retenía la ocupación anterior. Se publica la
rotación inmediatamente en `TownHost::office_links_changed`, también en las
imágenes persistidas del puente. No se altera el algoritmo ni el orden original.

Con la misma entrada y perfil, el snapshot del segundo día tiene SHA256
`d26da91588b364048aa8e78f98ed0fa3e700b20350b09aa71fb7e1dccd8fcd7d`.
El mundo completo coincide byte a byte, incluidas viviendas y reputación; el
reparto rico de oficinas24/29/33 es60/23/39. La prueba comprueba todos los enlaces
vivos en ambos días, guardado repetido y recarga. Añadir `P3_C10_ORIGINAL_DAY2` al
comando anterior. Una diferencia naval independiente en convoy2+0x14 permanece
fuera de esta corrección: no se acredita aún paridad completa del segundo día.
